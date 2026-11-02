//! People and creatures noticing and fighting, as the game's combat AI
//! does (`world::combat_ai`, read from its code; this file only measures
//! and moves):
//!
//! - Each actor's detection run (every 0.3 s in combat, staggered up to
//!   5 s more out of it; none farther than 8192 units from the player)
//!   works out its detection value for the player and everyone else
//!   loaded. A value rising above −20 starts a fight where aggression and
//!   factions say so; allies join by their Assistance; the unaggressive run
//!   from those much stronger who'd attack them.
//! - Gunmen keep within their weapon's band, strafing every 2–5 s, and fire
//!   at the game's pace (semi-automatic: after the attack and a random
//!   delay; automatic: 1 s bursts, 1 s pauses), only at a target in sight
//!   within the aim arc. Melee fighters run in, fast-walk the last 64 units,
//!   and after each attack roll attack or hold by their combat style.
//! - A target unseen for 15 s is searched for; unseen for 30 s (never seen)
//!   or 60 s (and more than 4096 units away) it's given up, and they go
//!   back to their packages.
//!
//! Measured here (guesses where the game's way isn't traced): lines of
//! sight are rays through the cell's collision 60 units above the feet;
//! the spot in the band a gunman moves to is on the line to the target,
//! halfway into the band (the game searches the navmesh, `009d5000`); a
//! search walks to where the target was last seen, then to random spots
//! within the smallest search radius every `fCombatSearchAreaUpdateTime`;
//! someone fleeing runs `fCombatFleeNormalDistance` (2048) straight away
//! from the threat until they no longer notice it; paths toward a moving
//! target are made again every half second. Not done: crouching, dodging,
//! cover, blocking (no block animations are played, so the block score is 0
//! as for those without one), suppressive fire, the hit landing at the
//! attack animation's hit key, the reload and equip animations (the reload
//! only waits its time).
//!
//! Shots (`world::npc_aim`, [`resolve_shots`]): each is aimed at the
//! middle of the target's height from 0.75 of the shooter's (the fire
//! node's own place, posed by the animation, isn't used), turned at random
//! within the weapon's cone plus the shooter's gun wobble × 15°
//! (`fNPCMaxGunWobbleAngle`; walking, running and aiming down the sights
//! past 512 × the weapon's sight usage count), and flies as a ray to the
//! first body (its skeleton's capsules; the player's bounds), scripted
//! object or wall within the projectile's range: shots miss and hit
//! bystanders. Someone other than their targets first on the straight
//! line holds the shot (`009a6e90`). Missiles that fly in the game are
//! rays here too.
//!
//! Their targets (`Targets`): everyone they start a fight with or notice
//! rising while fighting and would attack; when the target dies or is
//! given up, the best of the rest (`world::npc_aim::best_target`) is
//! fought next, as the combat group's target choice does. The group's
//! shared targets and detection, and the re-choice whenever the combat
//! planner plans anew (its timer isn't traced), aren't modelled: each
//! fighter keeps its own.
//!
//! People choose what they fight with (`world::npc_combat`): when the fight
//! starts and every 5 s, the best weapon of each kind they carry with
//! ammunition, guns rated out of play when they don't reach, the cheapest
//! attack kind, fists included; clips empty and are reloaded, and only
//! weapons flagged "NPCs use ammo" (dynamite) or a teammate's use rounds
//! up. Their script's `OnStartCombat` runs once their target is detected.
//!
//! Someone holding a grenade or dynamite keeps out of its blast (the band's
//! minimum is at least its radius) and throws it at the target's pace of
//! fire (`explosives` works out the arc); the grenade procedure's own
//! timing (`009cafe0`) isn't traced further.

use std::collections::{HashMap, HashSet};

use bevy::prelude::{Local, Query, Res, ResMut, Resource, Time};
use esm::{FormId, LoadOrder};
use world::ai::NavMesh;
use world::combat::Weapon;
use world::combat_ai::{
    self, Approach, CombatStyle, Engage, EngageMove, Gait, MeleeChoice, MeleeSituation,
    RangedAttack, SettingCache, TargetMemory,
};
use world::dialogue::PLAYER_REF;
use world::scripting::{Facts, GameState, Runner, ScriptCache};

use crate::actors::ActorRig;
use crate::ai::{distance, step, Walker};

/// How often a path toward a moving target is made again, seconds (not
/// read from the game).
const REPATH_SECONDS: f32 = 0.5;

/// Someone others can notice, as the frame began.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Seen {
    pub reference: FormId,
    pub position: [f32; 3],
    pub moving: bool,
    pub running: bool,
    /// Attacking now (an attack under way).
    pub attacking: bool,
    /// Their collision radius.
    pub radius: f32,
}

/// What an actor fights with, read once: its combat style, a creature's
/// reach and type, its collision radius, how fast it walks and runs, and
/// how long its attack animation lasts (the weapon kind's for people, its
/// own for creatures).
#[derive(Debug, Clone)]
pub(crate) struct Kit {
    pub style: CombatStyle,
    pub creature: Option<(f32, u8)>,
    pub radius: f32,
    pub walk: f32,
    pub run: f32,
    pub attack_animation: f32,
}

impl Kit {
    /// Read from the records and the skeleton: people's radius
    /// [`combat_ai::PERSON_RADIUS`]; a creature's from its skeleton's bound
    /// (`combat_ai::creature_radius`), else the game's 25 × scale for a
    /// creature without one; walking at the game's speed
    /// (`world::animation::base_speed`: `fMoveBaseSpeed` × SpeedMult ÷
    /// 100, × the scale; the legs' condition is applied where they move)
    /// and running `fMoveRunMult` times that; the attack animation's
    /// length (1 s without one: a guess).
    pub fn read(
        order: &LoadOrder,
        state: &GameState,
        walker: &Walker,
        skeleton: &preview::cell::ActorSkeleton,
    ) -> Kit {
        let creature = world::combat::creature_reach(order, walker.reference);
        let radius = match (creature, skeleton.bound) {
            (Some(_), Some(b)) => combat_ai::creature_radius(b.half_extents, walker.scale),
            (Some(_), None) => 25.0 * walker.scale,
            (None, _) => combat_ai::PERSON_RADIUS,
        };
        let walk = world::animation::base_speed(order, state, walker.reference) * walker.scale;
        let run = walk * world::animation::run_mult(order);
        let attack_animation = skeleton
            .attack
            .as_ref()
            .map_or(1.0, |a| (a.stop - a.start).max(0.1));
        Kit {
            style: CombatStyle::of(order, walker.reference),
            creature,
            radius,
            walk,
            run,
            attack_animation,
        }
    }

    /// How long an attack lasts (`Weapon::attack_seconds`: a gun's 1 ÷
    /// its attack shots a second, a melee weapon's attack animation at its
    /// attack multiplier), else the attack animation.
    pub fn attack_seconds(&self, weapon: Option<&Weapon>) -> f32 {
        weapon.map_or(self.attack_animation, |w| {
            w.attack_seconds(self.attack_animation)
        })
    }
}

/// A move under way in a fight: its gait, whether they keep facing the
/// target meanwhile (strafing), for a chase the distance from the target
/// at which it ends, and whether it ends with the target in sight again.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Move {
    gait: Gait,
    facing: bool,
    chase: Option<f32>,
    until_seen: bool,
}

impl Move {
    /// A move to a spot.
    fn to_spot(gait: Gait, facing: bool) -> Move {
        Move {
            gait,
            facing,
            chase: None,
            until_seen: false,
        }
    }
}

/// A fight under way: what's known of the target, the gunman's and the
/// swordsman's timers, the move under way.
#[derive(Debug, Clone)]
pub(crate) struct Fight {
    pub memory: TargetMemory,
    engage: Engage,
    ranged: RangedAttack,
    /// When the attack under way ends; holding until when.
    attack_until: f32,
    holding: bool,
    hold_until: f32,
    /// The target's last detection value and whether it was in sight.
    in_sight: bool,
    moving: Option<Move>,
    repath_at: f32,
    /// Searching: whether the last known spot has been reached, and when the
    /// next spot is due.
    searched_spot: bool,
    search_at: f32,
}

impl Fight {
    fn new(target: FormId, now: f32, at: [f32; 3]) -> Fight {
        Fight {
            memory: TargetMemory::new(target, now, at),
            engage: Engage::default(),
            ranged: RangedAttack::default(),
            attack_until: f32::NEG_INFINITY,
            holding: false,
            hold_until: f32::NEG_INFINITY,
            in_sight: true,
            moving: None,
            repath_at: f32::NEG_INFINITY,
            searched_spot: false,
            search_at: f32::NEG_INFINITY,
        }
    }
}

/// Random numbers for a frame's choices, from the state's dice.
pub(crate) struct Dice(u64);

impl Dice {
    pub fn new(state: &mut GameState) -> Dice {
        Dice(state.roll().max(1))
    }

    pub fn roll(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x >> 32) as u32
    }

    /// 0 up to (not including) 1.
    pub fn unit(&mut self) -> f32 {
        (self.roll() >> 8) as f32 / (1u32 << 24) as f32
    }
}

/// Whether nothing solid is between two people (from 60 units above their
/// feet: the game's eye points aren't traced).
pub(crate) fn clear_between(collision: &physics::Collider, from: [f32; 3], to: [f32; 3]) -> bool {
    let a = [from[0], from[1], from[2] + 60.0];
    let b = [to[0], to[1], to[2] + 60.0];
    let d = distance(a, b);
    if d < 1.0 {
        return true;
    }
    let dir = [(b[0] - a[0]) / d, (b[1] - a[1]) / d, (b[2] - a[2]) / d];
    collision
        .raycast(a, dir, d)
        .is_none_or(|(hit, _)| hit >= d - 10.0)
}

/// What noticing needs besides the state.
pub(crate) struct Noticing<'a> {
    pub order: &'a LoadOrder,
    pub settings: &'a SettingCache,
    pub collision: &'a physics::Collider,
    pub now: f32,
}

/// One actor's detection run (`008e40d0`, `008ff350`): its value for
/// everyone in `others` (the player first), what it knows of its target,
/// and, out of combat, a fight started (on noticing someone it would
/// attack, or a friend's enemy), or a flight.
pub(crate) fn detect(n: &Noticing, state: &mut GameState, walker: &mut Walker, others: &[Seen]) {
    let order = n.order;
    let me = walker.reference;
    let s = |name: &str, d: f32| n.settings.get(order, name, d);
    let max = s("fSneakMaxDistance", 1500.0)
        * if state.player_world.is_some() {
            s("fSneakExteriorDistanceMult", 2.0)
        } else {
            1.0
        };
    let mut values: Vec<(FormId, i32, bool, [f32; 3])> = Vec::new();
    {
        let facts = Facts {
            order,
            state,
            speaker: None,
        };
        for o in others {
            if o.reference == me || state.dead.contains(&o.reference) {
                continue;
            }
            let d = distance(walker.position, o.position);
            let sight = d < max && clear_between(n.collision, walker.position, o.position);
            let motion = (o.reference != PLAYER_REF).then_some((o.moving, o.running));
            if let Some(v) = combat_ai::detection_value(&facts, me, o.reference, sight, motion, &s)
            {
                values.push((o.reference, v, sight, o.position));
            }
        }
    }
    // Kept with the detection data, for `GetLineOfSight` (`008f6930`).
    for &(r, _, sight, _) in &values {
        world::more_functions::report_detection_sight(state, me, r, sight);
    }
    let noticed_min = s("fSneakNoticedMin", -20.0);
    let in_combat = state.combat.contains_key(&me);
    let mut start = None;
    // Whose side they took (their combat group, `world::combat_groups`).
    let mut helped = None;
    // The view cone: `fDetectionViewCone` (190°) across (`0088c570`).
    let half_cone = s("fDetectionViewCone", 190.0).to_radians() * 0.5;
    for &(r, v, sight, at) in &values {
        if r == PLAYER_REF {
            walker.detected_player = v;
        }
        let (yaw, _) = offsets(walker.position, walker.heading, at);
        walker
            .targets
            .saw(r, v, sight, sight && yaw.abs() <= half_cone, n.now);
        let noticed = v as f32 > noticed_min;
        let rising = noticed && walker.noticed.insert(r);
        if !noticed {
            walker.noticed.remove(&r);
            if walker.fleeing == Some(r) {
                walker.fleeing = None;
                walker.path.clear();
                walker.forget_package(n.now);
            }
        }
        if let Some(f) = walker.fight.as_mut().filter(|f| f.memory.target == r) {
            f.in_sight = sight;
            if v > 0 {
                f.memory.saw(n.now, at);
            }
        }
        // Everyone rising whom they'd attack becomes one of their targets,
        // in a fight too (`008ff350` queues a start of combat for each,
        // `009031b0`); the first starts the fight.
        let fighting = in_combat || start.is_some();
        if rising && combat_ai::starts_combat(order, state, me, r, v, &s) {
            if walker.targets.add(r) && fighting {
                println!("{:.1} s: {me} also takes on {r} (detection {v}).", n.now);
            }
            if !fighting {
                start = Some(r);
            }
            continue;
        }
        if !fighting
            && rising
            && walker.fleeing.is_none()
            && combat_ai::flees_on_sight(order, state, me, r, &s)
        {
            println!("{:.1} s: {me} runs from {r}.", n.now);
            walker.fleeing = Some(r);
            walker.path.clear();
        }
        if v > 0 {
            let value_of = |x: FormId| values.iter().find(|e| e.0 == x).map(|e| e.1);
            // Taking a new target to help only out of a fight: what
            // `008ff350`'s help does to someone already fighting beyond
            // moving them into the friend's combat group isn't traced (and
            // the player's "fight" here is only whoever hurt them last).
            let helping = (!fighting)
                .then(|| combat_ai::assists_against(order, state, me, r, v, value_of))
                .flatten();
            if let Some(enemy) = helping {
                println!("{:.1} s: {me} helps {r} against {enemy}.", n.now);
                walker.targets.add(enemy);
                start = Some(enemy);
                helped = Some(r);
            } else if fighting
                && world::combat_groups::helps_group(order, state, me, r, v, value_of)
            {
                // Already fighting: the help check still puts them in the
                // friend's combat group (`008ff350` → `CombatManager::
                // AddGroupMember` → `CombatGroup::MergeGroup`).
                println!("{:.1} s: {me} joins {r}'s combat group.", n.now);
                world::combat_groups::join(state, me, r);
            }
        }
    }
    if let Some(t) = start {
        let value = values.iter().find(|e| e.0 == t).map_or(0, |e| e.1);
        println!(
            "{:.1} s: {me} notices {t} (detection {value}) and attacks.",
            n.now
        );
        state.combat.insert(me, t);
        walker.fleeing = None;
        if let Some(friend) = helped {
            world::combat_groups::join(state, me, friend);
        }
    }
}

/// What a fight needs besides the state and the fighter.
pub(crate) struct FightCtx<'a> {
    pub order: &'a LoadOrder,
    pub scripts: &'a ScriptCache,
    pub settings: &'a SettingCache,
    pub mesh: &'a NavMesh,
    pub sounds: &'a mut crate::sounds::SoundRequests,
    /// Hits made, for their sounds and effects (`hiteffects`).
    pub hits: &'a mut crate::hiteffects::HitReports,
    /// Shots fired, for [`resolve_shots`].
    pub shots: &'a mut NpcShots,
    pub others: &'a [Seen],
    /// Turning settings (`world::movement`).
    pub moves: &'a world::movement::MoveSettings,
    pub now: f32,
    pub dt: f32,
}

/// What a fight frame did: moving (at what gait), and whether an attack
/// began.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FightFrame {
    pub gait: Option<Gait>,
    pub attacked: bool,
}

/// Reports a blow or shot that hurt `target` (`hiteffects`: its sounds,
/// heard within their distances of where it struck, and the hurt or death
/// cry). Where people's attacks meet a body isn't worked out here, so the
/// point is where the target stands.
fn report_hit(
    c: &mut FightCtx,
    state: &GameState,
    attacker: FormId,
    (target, at): (FormId, [f32; 3]),
    weapon: Option<FormId>,
    damage: f32,
) {
    c.hits.0.push(crate::hiteffects::HitReport {
        attacker,
        target: Some(target),
        weapon,
        point: at,
        havok: None,
        normal: None,
        triangle: None,
        direction: [0.0; 3],
        on_body: false,
        part: None,
        damage,
        killed: state.dead.contains(&target),
    });
}

/// Where someone is now.
fn position_of(order: &LoadOrder, state: &GameState, who: FormId) -> Option<[f32; 3]> {
    if who == PLAYER_REF {
        state.player_position
    } else {
        state.place(order, who).map(|p| p.2)
    }
}

/// One frame of a fight against `target`.
pub(crate) fn fight(
    c: &mut FightCtx,
    state: &mut GameState,
    walker: &mut Walker,
    kit: &Kit,
    target: FormId,
) -> FightFrame {
    let order = c.order;
    let me = walker.reference;
    let settings = c.settings;
    let s = |name: &str, d: f32| settings.get(order, name, d);
    let Some(goal) = position_of(order, state, target) else {
        return FightFrame::default();
    };
    let mut fight = match walker.fight.take() {
        Some(f) if f.memory.target == target => f,
        _ => {
            walker.path.clear();
            Fight::new(target, c.now, goal)
        }
    };
    walker.targets.add(target);
    // Whoever hurt them meanwhile and is now another target
    // (`world::combat_ai::attacked_by`: friends and allies aren't).
    for attacker in state.hit_targets.remove(&me).unwrap_or_default() {
        if !state.dead.contains(&attacker) && walker.targets.add(attacker) {
            println!(
                "{:.1} s: {me} also takes on {attacker} (hurt by them).",
                c.now
            );
        }
    }
    let d = distance(walker.position, goal);
    if fight
        .memory
        .gives_up(c.now, d, state.dead.contains(&target), &s)
    {
        println!("{:.1} s: {me} gives up on {target}.", c.now);
        // The target dropped (`00987220`), the next is chosen
        // (`0097f4d0`).
        walker.fight = Some(fight);
        walker.targets.remove(target);
        match next_target(order, state, walker, c.others, c.now) {
            Some(next) => turn_to(state, walker, next, c.now),
            None => {
                state.combat.remove(&me);
                state.hit_targets.remove(&me);
                end_fight(walker, c.now);
            }
        }
        return FightFrame::default();
    }
    // What they fight with (`world::npc_combat`: at the fight's start and
    // every `fCombatInventoryUpdateTimer`).
    if world::npc_combat::choose_weapon(order, state, me, &kit.style, Some(d), &s) {
        let held = world::combat::weapon_in_hand(order, state, me);
        println!(
            "{:.1} s: {me} takes up {}.",
            c.now,
            held.map_or("their fists".to_string(), |w| w.name)
        );
    }
    let mut dice = Dice::new(state);
    let frame = if fight.memory.searching(c.now, &s) {
        search(c, state, walker, kit, &mut fight, &mut dice)
    } else {
        let weapon = world::combat::weapon_in_hand(order, state, me);
        match weapon.as_ref().filter(|w| !w.is_melee()) {
            Some(w) => ranged(
                c,
                state,
                walker,
                kit,
                &mut fight,
                (w, goal, target),
                &mut dice,
            ),
            None => melee(
                c,
                state,
                walker,
                kit,
                &mut fight,
                (weapon.as_ref(), goal, target),
                &mut dice,
            ),
        }
    };
    walker.fight = Some(fight);
    frame
}

/// A fight over: back to their packages at once.
pub(crate) fn end_fight(walker: &mut Walker, now: f32) {
    walker.fight = None;
    walker.targets.list.clear();
    walker.path.clear();
    walker.next = 0;
    walker.forget_package(now);
}

/// What a fighter's detection runs last found of someone.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Known {
    value: i32,
    sight: bool,
    in_view: bool,
    /// When their value was last above 0.
    detected_at: f32,
}

/// A fighter's combat targets (standing in for its combat group's:
/// `CombatGroup::TargetArray`, Xbox PDB), and what its detection runs
/// found of everyone.
#[derive(Debug, Clone, Default)]
pub(crate) struct Targets {
    list: Vec<FormId>,
    known: HashMap<FormId, Known>,
}

impl Targets {
    /// One more target; whether it's new.
    fn add(&mut self, who: FormId) -> bool {
        let new = !self.list.contains(&who);
        if new {
            self.list.push(who);
        }
        new
    }

    fn remove(&mut self, who: FormId) {
        self.list.retain(|t| *t != who);
    }

    /// A detection run's value for `who`.
    fn saw(&mut self, who: FormId, value: i32, sight: bool, in_view: bool, now: f32) {
        let before = self
            .known
            .get(&who)
            .map_or(f32::NEG_INFINITY, |k| k.detected_at);
        self.known.insert(
            who,
            Known {
                value,
                sight,
                in_view,
                detected_at: if value > 0 { now } else { before },
            },
        );
    }
}

/// Whether someone fights in melee, as the target choice compares them
/// (`009a9630`: no weapon, or a melee one; the combat plan's melee actions
/// aren't looked at).
fn fights_in_melee(order: &LoadOrder, state: &GameState, who: FormId) -> bool {
    world::combat::weapon_in_hand(order, state, who).is_none_or(|w| w.is_melee())
}

/// The best of a fighter's live targets to fight next
/// (`world::npc_aim::best_target`), within its combat style's targeting
/// field of view.
pub(crate) fn next_target(
    order: &LoadOrder,
    state: &GameState,
    walker: &Walker,
    others: &[Seen],
    now: f32,
) -> Option<FormId> {
    let me = walker.reference;
    let current = state.combat.get(&me).copied();
    let melee = fights_in_melee(order, state, me);
    let style = walker.kit.as_ref().map(|k| &k.style);
    let candidates: Vec<world::npc_aim::TargetCandidate> = walker
        .targets
        .list
        .iter()
        .copied()
        .filter(|t| *t != me && !state.dead.contains(t))
        .filter_map(|t| {
            let at = others
                .iter()
                .find(|o| o.reference == t)
                .map(|o| o.position)
                .or_else(|| position_of(order, state, t))?;
            let (yaw, _) = offsets(walker.position, walker.heading, at);
            if style.is_some_and(|s| !combat_ai::within_targeting_fov(s, yaw)) {
                return None;
            }
            let known = walker.targets.known.get(&t);
            let d = distance(walker.position, at);
            let is_current = current == Some(t);
            Some(world::npc_aim::TargetCandidate {
                reference: t,
                detection: known.map_or(i32::MIN, |k| k.value),
                in_view: known.is_some_and(|k| k.in_view),
                in_sight: known.is_some_and(|k| k.sight),
                current: is_current,
                seen_recently: is_current
                    && walker
                        .fight
                        .as_ref()
                        .is_some_and(|f| f.memory.target == t && f.memory.unseen_for(now) < 2.0),
                same_kind: melee == fights_in_melee(order, state, t),
                distance_sq: d * d,
                down: state.unconscious.contains(&t),
                attacked_by_others: false,
                last_detected: known.map_or(f32::NEG_INFINITY, |k| k.detected_at),
            })
        })
        .collect();
    world::npc_aim::best_target(&candidates)
}

/// The fighter takes on `next`: a new fight against it, the fight going on.
fn turn_to(state: &mut GameState, walker: &mut Walker, next: FormId, now: f32) {
    println!("{now:.1} s: {} turns to {next}.", walker.reference);
    state.combat.insert(walker.reference, next);
    walker.fight = None;
    walker.path.clear();
    walker.next = 0;
}

/// After their target was killed (which ends their fight, `world::combat::
/// hurt`), the next of their targets, if one is left (`0097f4d0`:
/// `CombatController::SetTarget(CombatGroup::GetBestTarget)`, Xbox PDB);
/// a fight a script stopped isn't taken up again.
pub(crate) fn target_killed(
    order: &LoadOrder,
    state: &mut GameState,
    walker: &mut Walker,
    others: &[Seen],
    now: f32,
) {
    let Some(dead) = walker
        .fight
        .as_ref()
        .map(|f| f.memory.target)
        .filter(|t| state.dead.contains(t))
    else {
        return;
    };
    walker.targets.remove(dead);
    if let Some(next) = next_target(order, state, walker, others, now) {
        turn_to(state, walker, next, now);
    }
}

/// Sets a path to `to`, asked as any of the walker's path requests
/// (`ai::path_for`: their radius, the ends joined to the navmesh by ray
/// casts); whether there is one. None: the move fails, as the combat
/// procedures' do ("Pathing failed while approaching target", `009d3b80`
/// → `009caac0`); the game's combat requests don't allow a direct path
/// when pathing fails (request +0xa1 is set only by `008df1c0`'s callers,
/// none of them combat's). (`_straight`: what the old straight-line
/// fallback was for, kept for the callers.)
fn go(mesh: &NavMesh, walker: &mut Walker, to: [f32; 3], _straight: bool) -> bool {
    let Some(path) = crate::ai::path_for(mesh, walker, to) else {
        walker.clear_path();
        return false;
    };
    // In a fight the walk starts at once (no turn in place first).
    walker.set_path(path, 0.0, false, &world::movement::MoveSettings::defaults());
    true
}

/// How far off to the side (radians, either way) and up or down `to` is
/// for someone at `from` facing `heading` (clockwise from north).
fn offsets(from: [f32; 3], heading: f32, to: [f32; 3]) -> (f32, f32) {
    let (dx, dy, dz) = (to[0] - from[0], to[1] - from[1], to[2] - from[2]);
    let mut yaw = (dx.atan2(dy) - heading).rem_euclid(std::f32::consts::TAU);
    if yaw > std::f32::consts::PI {
        yaw -= std::f32::consts::TAU;
    }
    (yaw, dz.atan2(dx.hypot(dy)))
}

/// Turns to face `to`: in place, at the combat rate (225°/s for people,
/// `fAICombatTurnSpeedScale`; `ai::face`).
fn face(walker: &mut Walker, to: [f32; 3], c: &FightCtx) {
    let (dx, dy) = (to[0] - walker.position[0], to[1] - walker.position[1]);
    if dx.hypot(dy) > 1.0 {
        crate::ai::face(walker, dx.atan2(dy), c.dt, true, c.moves);
    }
}

/// Walks the move under way, at `legs` × the gait's speed (crippled legs:
/// `world::body_parts::leg_speed_mult`); whether they moved.
fn walk_move(
    walker: &mut Walker,
    fight: &Fight,
    kit: &Kit,
    (goal, legs): ([f32; 3], f32),
    dt: f32,
) -> bool {
    let Some(m) = fight.moving else {
        return false;
    };
    // Strafing, they keep facing the target (stepping sideways): the
    // walking turn aims at it, not along the path (`009e4450`).
    if m.facing {
        walker.face_point = Some(goal);
    }
    let walking = step(walker, m.gait.speed(kit.walk, kit.run) * legs, dt);
    walker.face_point = None;
    walking
}

/// A gunman's frame: keeping to the band (`Engage`), and shooting
/// (`RangedAttack`).
fn ranged(
    c: &mut FightCtx,
    state: &mut GameState,
    walker: &mut Walker,
    kit: &Kit,
    fight: &mut Fight,
    (w, goal, target): (&Weapon, [f32; 3], FormId),
    dice: &mut Dice,
) -> FightFrame {
    let order = c.order;
    let settings = c.settings;
    let s = |name: &str, d: f32| settings.get(order, name, d);
    let now = c.now;
    // What the shot is: the ammunition's projectile when it names one.
    let fired = world::combat::fired_projectile(order, state, walker.reference, w);
    let reach = fired.and_then(|p| world::combat::projectile_reach(order, p));
    // A projectile that explodes keeps the thrower out of its blast.
    let explosion = fired
        .and_then(|p| world::explosions::ProjectileRecord::load(order, p))
        .filter(|p| p.explodes())
        .and_then(|p| world::explosions::ExplosionRecord::load(order, p.explosion?));
    let blast = explosion.as_ref().map(|e| {
        world::explosions::blast_radius(order, state, Some(walker.reference), Some(w.form_id), e)
    });
    let band = combat_ai::ranged_band_blast(Some((w, reach)), blast, &kit.style, &s);
    let d = distance(walker.position, goal);
    // A move ends when its path does, a chase within its distance, an
    // approach once the target is in sight again.
    if let Some(m) = fight.moving {
        let done = walker.next >= walker.path.len()
            || m.chase.is_some_and(|stop| d <= stop)
            || (m.until_seen && fight.memory.unseen_for(now) <= 0.5);
        if done {
            walker.path.clear();
            fight.moving = None;
            fight.engage.arrived(now, dice.unit());
        } else if m.chase.is_some() && now >= fight.repath_at {
            fight.repath_at = now + REPATH_SECONDS;
            go(c.mesh, walker, goal, true);
        }
    }
    let unseen = fight.memory.unseen_for(now);
    let decision = fight
        .engage
        .update(now, d, &band, fight.in_sight, unseen, &mut || dice.unit());
    let toward = {
        let (dx, dy) = (goal[0] - walker.position[0], goal[1] - walker.position[1]);
        let l = dx.hypot(dy).max(1e-3);
        [dx / l, dy / l]
    };
    let at = |p: [f32; 3], along: f32, side: f32| {
        [
            p[0] + toward[0] * along + toward[1] * side,
            p[1] + toward[1] * along - toward[0] * side,
            p[2],
        ]
    };
    let started = match decision {
        EngageMove::Stay => None,
        EngageMove::Strafe { distance, left } => {
            let side = if left { -distance } else { distance };
            let spot = at(walker.position, 0.0, side);
            go(c.mesh, walker, spot, false).then_some(Move::to_spot(Gait::FastWalk, true))
        }
        EngageMove::ToBand { run } => {
            // On the line to the target, halfway into the band, else just
            // inside its nearer edge (a guess for the game's navmesh
            // search for a spot in range).
            let edge = if d < band.min {
                band.min + 16.0
            } else {
                band.optimal - 16.0
            };
            let gait = if run { Gait::Run } else { Gait::FastWalk };
            [(band.min + band.optimal) * 0.5, edge]
                .into_iter()
                .any(|want| go(c.mesh, walker, at(goal, -want, 0.0), false))
                .then_some(Move::to_spot(gait, false))
        }
        EngageMove::Step { distance, closer } => {
            let spot = at(
                walker.position,
                if closer { distance } else { -distance },
                0.0,
            );
            go(c.mesh, walker, spot, false).then_some(Move::to_spot(Gait::FastWalk, false))
        }
        EngageMove::RunAt => {
            fight.repath_at = now + REPATH_SECONDS;
            go(c.mesh, walker, goal, true).then_some(Move {
                gait: Gait::Run,
                facing: false,
                chase: Some(128.0),
                until_seen: false,
            })
        }
        EngageMove::Approach => {
            fight.repath_at = now + REPATH_SECONDS;
            go(c.mesh, walker, goal, true).then_some(Move {
                gait: Gait::FastWalk,
                facing: false,
                chase: Some(kit.radius + combat_ai::PERSON_RADIUS),
                until_seen: true,
            })
        }
    };
    if decision != EngageMove::Stay {
        println!(
            "{now:.1} s: {} at {d:.0} units (band {:.0}–{:.0}): {decision:?}{}",
            walker.reference,
            band.min,
            band.optimal,
            if started.is_some() { "" } else { " (no path)" }
        );
        match started {
            Some(m) => fight.moving = Some(m),
            // Nowhere to go: the timer starts again.
            None => fight.engage.arrived(now, dice.unit()),
        }
    }
    let legs = world::body_parts::leg_speed_mult(order, state, walker.reference);
    let walking = walk_move(walker, fight, kit, (goal, legs), c.dt);
    if !walking {
        face(walker, goal, c);
    }
    // Shooting: in sight, within the style's targeting field of view, and
    // within the aim arc (or nearer than the band's minimum).
    let (yaw, pitch) = offsets(walker.position, walker.heading, goal);
    let aimed = fight.in_sight
        && combat_ai::within_targeting_fov(&kit.style, yaw)
        && (combat_ai::within_aim_arc(w.aim_arc, yaw, pitch) || d < band.min)
        && blast_is_safe(c, state, walker, (&kit.style, explosion.as_ref()), goal);
    let attack = kit.attack_seconds(Some(w));
    // Not while reloading (`world::npc_combat`).
    let reloading = world::npc_combat::reloading(state, walker.reference);
    let shoots = !reloading
        && fight
            .ranged
            .update(now, w, &kit.style, aimed, attack, dice.unit(), &s);
    if shoots {
        // The attack's noise (`world::noise`).
        world::noise::attacked(order, state, walker.reference, w);
        match world::npc_combat::fired(order, state, walker.reference, w) {
            world::npc_combat::AfterShot::Reloading(t) => {
                println!("{now:.1} s: {} reloads ({t:.1} s).", walker.reference)
            }
            world::npc_combat::AfterShot::Dry => {
                println!(
                    "{now:.1} s: {} has no more for {}.",
                    walker.reference, w.name
                )
            }
            world::npc_combat::AfterShot::Ready => {}
        }
    }
    if shoots && world::explosions::is_thrown(w) {
        // Grenades and dynamite are thrown at where the target stands
        // (`explosives`: the arc, `CombatProcedureAttackGrenade`).
        if let Some(sound) = w.sound {
            c.sounds.0.push(sound);
        }
        let p = walker.position;
        crate::explosives::throw(crate::explosives::Launch {
            thrower: walker.reference,
            weapon: w.clone(),
            origin: [p[0], p[1], p[2] + crate::explosives::THROW_HEIGHT],
            aim: crate::explosives::Aim::At(goal),
        });
    } else if shoots {
        // Its firing sound and muzzle flash (`weapon_fx`, `0083ac30`).
        crate::weapon_fx::fired(crate::weapon_fx::Fired {
            shooter: walker.reference,
            weapon: w.form_id,
            from: None,
        });
        // The shot flies once everyone has moved ([`resolve_shots`]).
        c.shots.0.push(NpcShot {
            shooter: walker.reference,
            target,
            weapon: w.clone(),
            gait: if walking {
                fight.moving.map(|m| m.gait)
            } else {
                None
            },
        });
    }
    FightFrame {
        gait: walking.then(|| fight.moving.map_or(Gait::FastWalk, |m| m.gait)),
        attacked: shoots,
    }
}

/// Whether an exploding shot or throw at `goal` may go (the attack
/// procedures ask `CombatManager::CheckExplosionAttack`, `00992720`,
/// `world::explosions::explosion_attack_allowed`): counted within the
/// reach where its blast still does `fDangerousProjectileExplosionDamage`
/// are the shooter, their combat group and spectators, of those the combat
/// manager holds (`00992ba0`, `world::combat_groups::explosion_nearby`).
/// Each is counted where they stand now; the game asks each one's place
/// after the shot's flight time (vtable +0x4d8 with `009a7e00`'s time),
/// not translated.
fn blast_is_safe(
    c: &FightCtx,
    state: &GameState,
    walker: &Walker,
    (style, explosion): (&CombatStyle, Option<&world::explosions::ExplosionRecord>),
    goal: [f32; 3],
) -> bool {
    let order = c.order;
    let Some(e) = explosion else {
        return true;
    };
    let units = c.settings.get(order, "fBSUnitsPerFoot", 22.0);
    let me = walker.reference;
    let responsibility = world::scripting::Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(me, 3)
    .unwrap_or(0.0) as f32;
    world::explosions::explosion_attack_allowed(
        Some((e.radius_units(units), e.damage)),
        &world::explosions::DangerSettings::read(order),
        style.flags,
        responsibility,
        |reach| {
            world::combat_groups::explosion_nearby(
                state,
                me,
                std::iter::once((me, walker.position)).chain(
                    c.others
                        .iter()
                        .filter(|o| o.reference != me)
                        .map(|o| (o.reference, o.position)),
                ),
                goal,
                reach,
            )
        },
    )
}

/// A melee fighter's frame: closing in (`melee_approach`), then rolling
/// attack or hold after each attack (`melee_choice`).
fn melee(
    c: &mut FightCtx,
    state: &mut GameState,
    walker: &mut Walker,
    kit: &Kit,
    fight: &mut Fight,
    (weapon, goal, target): (Option<&Weapon>, [f32; 3], FormId),
    dice: &mut Dice,
) -> FightFrame {
    let order = c.order;
    let settings = c.settings;
    let s = |name: &str, d: f32| settings.get(order, name, d);
    let now = c.now;
    fight.moving = None;
    let creature = if weapon.is_none() { kit.creature } else { None };
    let reach = combat_ai::melee_reach(weapon, creature, walker.scale, &s);
    let them = c.others.iter().find(|o| o.reference == target);
    let their_radius = them.map_or(combat_ai::PERSON_RADIUS, |o| o.radius);
    // Out of sight for a moment: toward where they were last seen.
    let toward = if fight.memory.unseen_for(now) > 0.5 {
        fight.memory.last_known
    } else {
        goal
    };
    let gap = combat_ai::gap(distance(walker.position, toward), kit.radius, their_radius);
    let gait = match combat_ai::melee_approach(gap, reach, &s) {
        Approach::Run => Some(Gait::Run),
        Approach::FastWalk => Some(Gait::FastWalk),
        Approach::InReach => None,
    };
    if let Some(g) = gait {
        if now >= fight.repath_at || walker.next >= walker.path.len() {
            fight.repath_at = now + REPATH_SECONDS;
            go(c.mesh, walker, toward, true);
        }
        // Crippled legs slow them (`world::body_parts::leg_speed_mult`).
        let legs = world::body_parts::leg_speed_mult(order, state, walker.reference);
        let walking = step(walker, g.speed(kit.walk, kit.run) * legs, c.dt);
        if !walking {
            face(walker, toward, c);
        }
        return FightFrame {
            gait: walking.then_some(g),
            attacked: false,
        };
    }
    walker.path.clear();
    face(walker, goal, c);
    let due = now >= fight.attack_until && (!fight.holding || now >= fight.hold_until);
    if !due {
        return FightFrame::default();
    }
    let situation = MeleeSituation {
        skill: combat_ai::melee_skill(order, state, walker.reference, weapon),
        target_attacking: them.is_some_and(|o| o.attacking),
        blocking: false,
        target_recoiling: false,
        target_unconscious: state.unconscious.contains(&target),
        unarmed: weapon.is_none(),
        can_block: false,
        target_in_combat: state.combat.contains_key(&target),
        target_is_player: target == PLAYER_REF,
        holding: fight.holding,
    };
    let scores = combat_ai::melee_scores(&kit.style, &situation, &s);
    match combat_ai::melee_choice(&scores, dice.roll(), fight.holding) {
        MeleeChoice::Attack => {
            fight.holding = false;
            // Fatigue isn't kept: always full.
            let power = combat_ai::power_attack(
                &kit.style,
                situation.target_recoiling,
                situation.target_unconscious,
                1.0,
                dice.roll(),
            );
            fight.attack_until = now + kit.attack_seconds(weapon);
            // Melee attacks play no firing sound (`0083ac30` is the guns'):
            // a swing that meets no one plays its `TNAM` (below).
            if let Some(w) = weapon {
                world::noise::attacked(order, state, walker.reference, w);
            }
            let dealt = Runner::new(order, c.scripts, state).strike(
                walker.reference,
                target,
                weapon,
                power,
            );
            if let Some(dmg) = dealt {
                report_hit(
                    c,
                    state,
                    walker.reference,
                    (target, goal),
                    weapon.map(|w| w.form_id),
                    dmg,
                );
                let left = world::combat::health(order, state, target).unwrap_or(0.0);
                println!(
                    "{now:.1} s: {} {} {target} for {dmg:.1} ({left:.1} left).",
                    walker.reference,
                    if power { "power-attacks" } else { "hits" }
                );
            } else if let Some(w) = weapon {
                // Met no one (`00899200`).
                crate::weapon_fx::swung(walker.reference, w.form_id);
            }
            FightFrame {
                gait: None,
                attacked: true,
            }
        }
        MeleeChoice::Hold => {
            fight.holding = true;
            fight.hold_until = now + combat_ai::hold_seconds(&kit.style, dice.unit());
            FightFrame::default()
        }
        MeleeChoice::Block | MeleeChoice::Nothing => {
            fight.holding = false;
            FightFrame::default()
        }
    }
}

/// Searching for a target unseen for 15 s: to where it was last seen, then
/// spots within the search radius around it (see the module notes).
fn search(
    c: &mut FightCtx,
    state: &mut GameState,
    walker: &mut Walker,
    kit: &Kit,
    fight: &mut Fight,
    dice: &mut Dice,
) -> FightFrame {
    let order = c.order;
    let settings = c.settings;
    let s = |name: &str, d: f32| settings.get(order, name, d);
    let now = c.now;
    let spot = fight.memory.last_known;
    let idle = walker.next >= walker.path.len();
    if idle {
        if !fight.searched_spot {
            fight.searched_spot = true;
            println!(
                "{now:.1} s: {} searches for {}.",
                walker.reference, fight.memory.target
            );
            go(c.mesh, walker, spot, false);
        } else if now >= fight.search_at {
            fight.search_at = now + s("fCombatSearchAreaUpdateTime", 5.0);
            let radius = combat_ai::search_radius(state.player_world.is_some(), &s);
            let angle = dice.unit() * std::f32::consts::TAU;
            let r = radius * dice.unit().sqrt();
            let to = [
                spot[0] + r * angle.sin(),
                spot[1] + r * angle.cos(),
                spot[2],
            ];
            go(c.mesh, walker, to, false);
        }
    }
    let walking = step(walker, kit.walk, c.dt);
    FightFrame {
        gait: walking.then_some(Gait::Walk),
        attacked: false,
    }
}

/// Someone running from `threat` (an unaggressive actor, see `detect`):
/// `fCombatFleeNormalDistance` (2048) straight away from it, else half
/// that, else a quarter (where the navmesh allows); whether they're
/// moving.
pub(crate) fn flee(
    order: &LoadOrder,
    settings: &SettingCache,
    mesh: &NavMesh,
    state: &GameState,
    walker: &mut Walker,
    kit: &Kit,
    dt: f32,
) -> bool {
    let Some(threat) = walker.fleeing else {
        return false;
    };
    let Some(from) = position_of(order, state, threat).filter(|_| !state.dead.contains(&threat))
    else {
        walker.fleeing = None;
        return false;
    };
    if walker.next >= walker.path.len() {
        let far = settings.get(order, "fCombatFleeNormalDistance", 2048.0);
        let (dx, dy) = (walker.position[0] - from[0], walker.position[1] - from[1]);
        let l = dx.hypot(dy).max(1e-3);
        for share in [1.0, 0.5, 0.25] {
            let to = [
                walker.position[0] + dx / l * far * share,
                walker.position[1] + dy / l * far * share,
                walker.position[2],
            ];
            if go(mesh, walker, to, false) {
                break;
            }
        }
    }
    step(walker, kit.run, dt)
}

/// A shot someone fired this frame: at whom, with what, and how they were
/// moving (for the wobble).
#[derive(Debug, Clone)]
pub(crate) struct NpcShot {
    shooter: FormId,
    target: FormId,
    weapon: Weapon,
    gait: Option<Gait>,
}

/// The shots fired this frame, flown by [`resolve_shots`].
#[derive(Resource, Default)]
pub struct NpcShots(Vec<NpcShot>);

/// The player's base record (`NPC_` 00000007), for their bounds.
const PLAYER_BASE: FormId = FormId(7);

/// Feet and height (the base's bounds × scale, `world::npc_aim::
/// actor_height`; else the viewer's body default) of someone on screen or
/// the player.
fn body_of(
    order: &LoadOrder,
    state: &GameState,
    rigs: &Query<(&Walker, &ActorRig)>,
    talkers: &crate::dialogue::Talkers,
    who: FormId,
) -> Option<([f32; 3], f32)> {
    let (feet, scale, base) = if who == PLAYER_REF {
        (state.player_position?, 1.0, PLAYER_BASE)
    } else {
        let (w, _) = rigs.iter().find(|(w, _)| w.reference == who)?;
        let base = talkers
            .0
            .iter()
            .find(|t| t.reference == who)
            .map(|t| t.base)
            .or_else(|| world::scripting::base_of(order, who))?;
        (w.position, w.scale, base)
    };
    let height = world::npc_aim::actor_height(order, base, scale)
        .unwrap_or_else(|| crate::combat::body(order, base).1 * scale);
    Some((feet, height))
}

/// What a person's shot along `dir` from `origin` meets first within
/// `reach` (`combat::first_met_past`, passing by the shooter), the player
/// by their bounds included: who or what, how far, the body part; and the
/// world's collision met, if any.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn met_first(
    order: &LoadOrder,
    state: &GameState,
    caches: &mut crate::combat::PlayerAttack,
    around: (
        &crate::dialogue::Talkers,
        &crate::scripts::CellScripts,
        &crate::walk::CellCollision,
        &Query<(&Walker, &ActorRig)>,
    ),
    (origin, dir, reach): ([f32; 3], [f32; 3], f32),
    shooter: FormId,
    now: f32,
) -> (Option<(f32, FormId, Option<u8>)>, Option<(f32, u32)>) {
    let collision = around.2;
    let met = crate::combat::first_met_past(
        order,
        state,
        caches,
        around,
        (origin, dir),
        reach,
        (false, Some(shooter)),
        now,
    );
    let wall = crate::combat::cast(collision, (origin, dir), reach, false);
    let player = (shooter != PLAYER_REF && !state.dead.contains(&PLAYER_REF))
        .then_some(state.player_position)
        .flatten()
        .and_then(|p| {
            let (radius, tall) = crate::combat::body(order, PLAYER_BASE);
            crate::combat::ray_body(origin, dir, p, radius, tall)
        })
        .filter(|d| *d <= reach && wall.is_none_or(|(w, _)| w >= d - 5.0));
    let first = match (&met, player) {
        (
            crate::combat::Met::Thing {
                distance: d,
                reference,
                part,
            },
            p,
        ) if p.is_none_or(|p| *d <= p) => Some((*d, *reference, *part)),
        (_, Some(p)) => Some((p, PLAYER_REF, None)),
        _ => None,
    };
    (first, wall)
}

/// Flies the shots people fired this frame (`world::npc_aim`; see the
/// module notes): each pellet from 0.75 of the shooter's height toward the
/// middle of the target's, turned within the cone, to the first body,
/// scripted object or wall within the projectile's range; the player is
/// met by their bounds. Whoever is struck takes the pellet's share of the
/// damage on the part struck.
#[allow(clippy::too_many_arguments)]
pub(crate) fn resolve_shots(
    time: Res<Time>,
    game: Res<crate::GameFiles>,
    mut dialogue: ResMut<crate::dialogue::DialogueState>,
    mut caches: ResMut<crate::combat::PlayerAttack>,
    world: (
        Res<crate::dialogue::Talkers>,
        Res<crate::scripts::CellScripts>,
        Res<crate::walk::CellCollision>,
        Res<crate::scripts::Scripts>,
    ),
    settings: Res<crate::ai::CombatSettings>,
    mut hits: ResMut<crate::hiteffects::HitReports>,
    mut shots: ResMut<NpcShots>,
    mut vats_settings: Local<Option<world::vats::Settings>>,
    rigs: Query<(&Walker, &ActorRig)>,
) {
    if shots.0.is_empty() {
        return;
    }
    let (talkers, cell_scripts, collision, scripts) = world;
    let order = &game.0.order;
    let state = &mut dialogue.0;
    let now = time.elapsed_secs();
    let s = |n: &str, d: f32| settings.0.get(order, n, d);
    let vs = vats_settings.get_or_insert_with(|| world::vats::Settings::load(order));
    for shot in std::mem::take(&mut shots.0) {
        let me = shot.shooter;
        let w = &shot.weapon;
        let Some((from, my_height)) = body_of(order, state, &rigs, &talkers, me) else {
            continue;
        };
        let Some((feet, height)) = body_of(order, state, &rigs, &talkers, shot.target) else {
            continue;
        };
        let origin = [
            from[0],
            from[1],
            from[2] + world::npc_aim::fire_height(my_height),
        ];
        let down = state.unconscious.contains(&shot.target);
        let aim = [
            feet[0],
            feet[1],
            feet[2]
                + world::npc_aim::aim_height(height, world::npc_aim::SEGMENT_MIDDLE, down, false),
        ];
        // Aiming down the sights past the weapon's distance; walking or
        // running as they moved.
        let aiming = world::npc_aim::uses_iron_sights(
            distance(from, aim),
            world::npc_aim::sight_usage(order, w.form_id),
            w.is_melee(),
            &s,
        );
        let stance = world::vats::Stance {
            sneaking: false,
            swimming: false,
            walking: matches!(shot.gait, Some(Gait::Walk | Gait::FastWalk)),
            running: shot.gait == Some(Gait::Run),
            aiming,
        };
        let wobble = world::vats::wobble(order, state, vs, me, Some(w), stance);
        let ammo = w.ammo_in_use(order, state, me);
        let (count, weapon_cone) = w.shot(order, ammo);
        let cone = world::npc_aim::npc_cone(weapon_cone, wobble, vs.npc_max_gun_wobble);
        let reach = w.range(order).unwrap_or(crate::combat::SHOT_RANGE);
        let mut pellet = w.clone();
        pellet.damage /= count.max(1) as f32;
        // The ammunition's own projectile when it names one
        // (`world::combat::fired_projectile`).
        pellet.projectile = world::combat::fired_projectile(order, state, me, w);
        let (heading, pitch) = world::npc_aim::heading_pitch(origin, aim);
        // The line of fire (`009d0a30` → `009a6e90`): someone other than
        // the target or another of the shooter's targets first on the line
        // to the aim point holds the shot.
        let mine = |who: FormId| {
            who == shot.target
                || rigs
                    .iter()
                    .find(|(w, _)| w.reference == me)
                    .is_some_and(|(w, _)| w.targets.list.contains(&who))
        };
        let straight = world::npc_aim::direction(heading, pitch);
        let (first, _) = met_first(
            order,
            state,
            &mut caches,
            (&talkers, &cell_scripts, &collision, &rigs),
            (origin, straight, reach),
            me,
            now,
        );
        if let Some((_, friend, _)) =
            first
                .filter(|(_, who, _)| !mine(*who))
                .filter(|(_, who, _)| {
                    *who == PLAYER_REF || talkers.0.iter().any(|t| t.reference == *who)
                })
        {
            println!(
                "{now:.1} s: {me} holds fire at {}: {friend} is in the way.",
                shot.target
            );
            continue;
        }
        let mut struck_any = false;
        let mut walled = None;
        for _ in 0..count.max(1) {
            let unit = |v: u64| (v % 1_000_000) as f32 / 1_000_000.0;
            let (u_r, u_turn) = (unit(state.roll()), unit(state.roll()));
            let (h, p) = world::npc_aim::deviate(heading, pitch, cone, u_r, u_turn);
            let dir = world::npc_aim::direction(h, p);
            // A plasma bolt flies and strikes on its way (`bolts`, which
            // says what it struck; no miss is told here).
            if crate::bolts::flies(order, &pellet) {
                crate::bolts::fire(order, state, me, &pellet, (origin, dir));
                struck_any = true;
                continue;
            }
            let (victim, wall) = met_first(
                order,
                state,
                &mut caches,
                (&talkers, &cell_scripts, &collision, &rigs),
                (origin, dir, reach),
                me,
                now,
            );
            let Some((d, victim, part)) = victim else {
                if let Some(at) = wall {
                    walled = Some(at.0);
                    hits.shot_on_world(&collision.0, (origin, dir), at, me, w.form_id);
                }
                continue;
            };
            let Some(hit) =
                Runner::new(order, &scripts.0, state).hit_at(me, victim, Some(&pellet), part)
            else {
                continue;
            };
            struck_any = true;
            hits.0.push(crate::hiteffects::HitReport {
                attacker: me,
                target: Some(victim),
                weapon: Some(w.form_id),
                point: [0, 1, 2].map(|k| origin[k] + dir[k] * d),
                havok: None,
                normal: None,
                triangle: None,
                direction: dir,
                on_body: true,
                part,
                damage: hit.dealt,
                killed: state.dead.contains(&victim),
            });
            let left = world::combat::health(order, state, victim).unwrap_or(0.0);
            let aside = if victim == shot.target {
                String::new()
            } else {
                format!(" (aiming at {})", shot.target)
            };
            println!(
                "{now:.1} s: {me} shoots {victim} from {d:.0} units for {:.1} ({left:.1} left){aside}.",
                hit.dealt
            );
        }
        if !struck_any {
            println!(
                "{now:.1} s: {me} misses {} from {:.0} units (cone {:.2}°{}).",
                shot.target,
                distance(from, feet),
                cone.to_degrees(),
                walled.map_or(String::new(), |d| format!(", into the world at {d:.0}"))
            );
        }
    }
}

/// Who's been noticed, kept per actor (see `detect`).
pub(crate) type Noticed = HashSet<FormId>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targets_are_measured_off_the_heading_both_ways() {
        use std::f32::consts::FRAC_PI_2;
        // Facing north: a target due east is a quarter turn to the right,
        // one due west a quarter to the left; one 100 up at 100 is 45° up.
        let (yaw, _) = offsets([0.0; 3], 0.0, [10.0, 0.0, 0.0]);
        assert!((yaw - FRAC_PI_2).abs() < 1e-5);
        let (yaw, _) = offsets([0.0; 3], 0.0, [-10.0, 0.0, 0.0]);
        assert!((yaw + FRAC_PI_2).abs() < 1e-5);
        // Facing east, a target just north of east is a little left.
        let (yaw, pitch) = offsets([0.0; 3], FRAC_PI_2, [100.0, 1.0, 100.0]);
        assert!(yaw < 0.0 && yaw > -0.02, "{yaw}");
        assert!((pitch - std::f32::consts::FRAC_PI_4).abs() < 1e-3);
    }

    /// Someone at the origin facing north, and two people they fight: one
    /// 500 units east, one 3000 north.
    fn fighter() -> (Walker, Vec<Seen>) {
        let mut w = Walker::at(FormId(0x100), [0.0; 3], 0.0, 1.0, false);
        let seen = |r: u32, p: [f32; 3]| Seen {
            reference: FormId(r),
            position: p,
            moving: false,
            running: false,
            attacking: false,
            radius: combat_ai::PERSON_RADIUS,
        };
        let others = vec![
            seen(0x200, [500.0, 0.0, 0.0]),
            seen(0x300, [0.0, 3000.0, 0.0]),
        ];
        w.targets.add(FormId(0x200));
        w.targets.add(FormId(0x300));
        (w, others)
    }

    #[test]
    fn targets_remember_when_they_were_last_detected() {
        let mut t = Targets::default();
        assert!(t.add(FormId(1)));
        assert!(!t.add(FormId(1)));
        t.saw(FormId(1), 30, true, true, 2.0);
        t.saw(FormId(1), -10, false, false, 5.0);
        let k = t.known[&FormId(1)];
        assert_eq!((k.value, k.sight, k.detected_at), (-10, false, 2.0));
        t.remove(FormId(1));
        assert!(t.list.is_empty());
    }

    #[test]
    fn the_next_target_is_the_best_detected_one_left() {
        let data = testdata::ai::world("next-target");
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..Default::default()
            },
        )
        .unwrap();
        let order = &game.order;
        let mut state = GameState::default();
        let (mut w, others) = fighter();
        // Both detected and in sight; the near one also in view: it's
        // chosen (1000 + 100 + 940 against 100 + 0).
        w.targets.saw(FormId(0x200), 40, true, true, 1.0);
        w.targets.saw(FormId(0x300), 40, true, false, 1.0);
        assert_eq!(
            next_target(order, &state, &w, &others, 2.0),
            Some(FormId(0x200))
        );
        // Undetected, the near one gives way to the detected far one.
        w.targets.saw(FormId(0x200), -30, false, false, 3.0);
        assert_eq!(
            next_target(order, &state, &w, &others, 3.0),
            Some(FormId(0x300))
        );
        // The dead aren't chosen; none left, none chosen.
        state.dead.insert(FormId(0x300));
        assert_eq!(
            next_target(order, &state, &w, &others, 3.0),
            Some(FormId(0x200))
        );
        state.dead.insert(FormId(0x200));
        assert_eq!(next_target(order, &state, &w, &others, 3.0), None);
    }

    #[test]
    fn a_killed_targets_killer_takes_on_the_next() {
        let data = testdata::ai::world("target-killed");
        let game = cellview::Game::open(
            data.path(),
            &cellview::Options {
                official: true,
                ..Default::default()
            },
        )
        .unwrap();
        let order = &game.order;
        let mut state = GameState::default();
        let (mut w, others) = fighter();
        w.targets.saw(FormId(0x200), 40, true, true, 1.0);
        w.targets.saw(FormId(0x300), 40, true, true, 1.0);
        // Fighting the near one, which dies (`world::combat::hurt` ends
        // the fight).
        w.fight = Some(Fight::new(FormId(0x200), 0.0, [500.0, 0.0, 0.0]));
        state.dead.insert(FormId(0x200));
        target_killed(order, &mut state, &mut w, &others, 4.0);
        assert_eq!(state.combat.get(&FormId(0x100)), Some(&FormId(0x300)));
        assert!(w.fight.is_none());
        assert_eq!(w.targets.list, vec![FormId(0x300)]);
        // A fight a script stopped (the target alive) isn't taken up.
        state.combat.clear();
        w.fight = Some(Fight::new(FormId(0x300), 4.0, [0.0, 3000.0, 0.0]));
        target_killed(order, &mut state, &mut w, &others, 5.0);
        assert!(state.combat.is_empty());
    }

    #[test]
    fn dice_give_numbers_from_zero_to_one() {
        let mut dice = Dice(0x1234_5678_9ABC_DEF0);
        let draws: Vec<f32> = (0..1000).map(|_| dice.unit()).collect();
        assert!(draws.iter().all(|u| (0.0..1.0).contains(u)));
        // Spread over the range.
        assert!(draws.iter().any(|u| *u < 0.1) && draws.iter().any(|u| *u > 0.9));
    }
}

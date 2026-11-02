//! The player fighting (`world::combat`): with the weapon in hand (the
//! one equipped, else the first carried) or fists, the left mouse button
//! attacks along the view: guns as far as their projectile's range, each
//! pellet within the weapon's cone (its min spread, as the game fires
//! them), melee weapons and fists as far as their reach (× 128; 64
//! unarmed) between the bodies' edges. What's hit first takes it: a person
//! or creature, a scripted object (its `OnHit` blocks run), or a wall,
//! which stops it. Guns fire at their attack rate, use their ammunition
//! from the inventory, and need reloading (R) after a clip. Whoever is
//! hurt fights back (`ai`). A line at the bottom shows health, ammunition
//! and the player's crippled limbs.
//!
//! Where a hit lands (`world::body_parts`): shots must meet one of the
//! capsules the target's skeleton carries on its bones (the game's own hit
//! shapes: its ragdoll's bodies, posed as the actor is now), and land on
//! that bone's part; a melee hit reaches the target by its body's bounds
//! and lands where the view meets a capsule, else on the bone nearest the
//! view (a guess). Skeletons without bodies use their record's bounds and
//! the nearest bone. A person who drops their weapon has its model hidden.
//!
//! Grenades and thrown weapons (dynamite) are thrown instead: their
//! projectiles fly and explode in `explosives`.
//!
//! Not yet: other projectiles in flight (the game's bullets are hitscan;
//! missiles, flames and beams fly), auto-aim (3° toward a target), the
//! gun sway's turn on shots (`world::gun_wobble`), hits on a held weapon
//! outside V.A.T.S. (part 14: its collision isn't tested). V.A.T.S. is
//! `vats`, which shoots through [`first_met`] too.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::prelude::*;
use esm::FormId;
use world::body_parts::BodyPartData;
use world::combat::{self, Weapon};
use world::dialogue::PLAYER_REF;
use world::scripting::Runner;

use crate::actors::ActorRig;
use crate::ai::Walker;
use crate::dialogue::{Conversation, DialogueState, Talkers};
use crate::menus::Menus;
use crate::scripts::{CellScripts, Scripts};
use crate::sounds::SoundRequests;
use crate::walk::{game_point, CellCollision, Player};
use crate::{FlyCamera, GameFiles};

/// The critical hit message's icon (`0089a760`).
const CRITICAL_ICON: &str = "Interface\\Icons\\Message Icons\\glow_message_vaultboy_very_happy.dds";

/// How far a shot can reach when nothing else says (units).
pub(crate) const SHOT_RANGE: f32 = 10_000.0;

/// The player's attacking: when the next one can come, rounds left in the
/// clip (`None`: a full one), and a reload under way.
#[derive(Resource, Default)]
pub struct PlayerAttack {
    next: f32,
    in_clip: Option<u32>,
    weapon: Option<FormId>,
    reloaded_at: f32,
    /// When the last attack and the reload under way started (for the
    /// first-person animations).
    pub fired_at: Option<f32>,
    pub reload_started: Option<f32>,
    /// How fast those animations play relative to the world (V.A.T.S.'s
    /// playback runs the player at its own multiplier, `vats`); `None`: as
    /// recorded.
    pub sped_up: Option<f32>,
    /// The rates the attack and the reload under way play at
    /// (`world::combat::attack_rate`, `reload_rate`: the weapon's speed
    /// and attack multiplier, Agility, and the perks), on top of
    /// `sped_up`; 0 before any.
    pub attack_rate: f32,
    pub reload_rate: f32,
    /// Whether the weapon (or fists) is out, as the game keeps it for the
    /// player (`process+0x135`, `IsWeaponOut`): a new game and every
    /// change of weapon start holstered (equipping into an empty hand and
    /// unequipping both put it away: `0088db20`, `0088d7d0`); the Ready
    /// Item key and attacking draw it (see [`player_attack`]).
    pub out: bool,
    /// When the weapon was last drawn (`true`) or put away (`false`), for
    /// the first-person animations (`viewmodel`).
    pub readied_at: Option<(f32, bool)>,
    /// Until when the drawing or putting away plays (the first-person view
    /// says, knowing the animation: [`Self::readying_until`]); the key and
    /// attacks wait for it, as the game waits for its queued weapon action
    /// (`008a7570() == -1`).
    busy_until: f32,
    /// The Ready Item key's press (`world::combat::ReadyKey`).
    ready: combat::ReadyKey,
    /// Looking down the sights (the process's iron sights flag, vfunc
    /// +0x404 `GetIronSights` (Xbox PDB); `world::iron_sights`): the Aim
    /// control (right mouse button) held with a gun out.
    pub iron_sights: bool,
    /// Blocking (anim action 7, `world::melee`): the Aim control held with
    /// a melee weapon or fists out.
    pub blocking: bool,
    /// When the last blocked hit's `BlockHit` started (first person).
    pub block_hit_at: Option<f32>,
    /// The counter-attack timer (player +0xe28): `fCounterAttackTimer`
    /// after a blocked hit, counting down.
    pub counter_timer: f32,
    /// The Attack control's hold (`011e07b0`), for power attacks, and a
    /// power attack waiting for the attack playing to end (`011e07ac` 2).
    power_timer: f32,
    power_queued: bool,
    /// The animation group of the attack playing (`world::melee::group`;
    /// 0x20 the default `AttackRight`, the weapon's own otherwise), and
    /// whether it's a power attack.
    pub attack_group: u8,
    pub power: bool,
    /// The attack's first-person length (the view says, knowing the
    /// animation), for the power attack waiting on it.
    pub attack_length: f32,
    /// The Ammo Swap timer (player +0xd50).
    ammo_swap_timer: f32,
    /// After the player's death (`world::player_death`).
    death: world::player_death::DeathReload,
    /// Bodies by base record: half width and height (from `OBND`).
    bodies: HashMap<FormId, (f32, f32)>,
    /// Body part data by person or creature (`world::body_parts`).
    parts: HashMap<FormId, Option<Arc<BodyPartData>>>,
}

impl PlayerAttack {
    /// Rounds in the clip of `weapon` (a full one to begin with), at most
    /// what's carried.
    pub(crate) fn clip(&self, weapon: &Weapon, carried: u32) -> u32 {
        let full = if self.weapon == Some(weapon.form_id) {
            self.in_clip.unwrap_or(weapon.clip)
        } else {
            weapon.clip
        };
        full.min(carried)
    }

    /// The clip of the weapon in hand now holds `rounds`.
    pub(crate) fn set_clip(&mut self, weapon: &Weapon, rounds: u32) {
        self.weapon = Some(weapon.form_id);
        self.in_clip = Some(rounds);
    }

    /// Draws (`true`) or puts away the weapon now, playing its animation.
    fn set_out(&mut self, out: bool, now: f32) {
        self.out = out;
        self.readied_at = Some((now, out));
        self.busy_until = now;
    }

    /// The player's animation action now, by the game's numbers
    /// (`GetAnimAction`, process vfunc +0x3e4: 0 drawing the weapon, 1
    /// putting it away, 8 reloading; the attacks and the rest aren't kept
    /// here), for what waits on it (the Sneak control, `walk`).
    pub fn anim_action(&self, now: f32) -> Option<u8> {
        if self.blocking {
            return Some(7);
        }
        if now < self.busy_until {
            return self
                .readied_at
                .map(|(_, drawing)| if drawing { 0 } else { 1 });
        }
        (now < self.reloaded_at).then_some(8)
    }

    /// The weapon `weapon` (`None`: fists) in hand and out, without the
    /// animation: for `--weapon` pictures (the game starts holstered).
    pub fn draw_at_once(&mut self, weapon: Option<FormId>) {
        self.weapon = weapon;
        self.in_clip = None;
        self.out = true;
        self.readied_at = None;
        self.busy_until = 0.0;
    }

    /// The drawing or putting away that started at `since` plays for
    /// `seconds`.
    pub fn readying_until(&mut self, since: f32, seconds: f32) {
        if self.busy_until <= since {
            self.busy_until = since + seconds;
        }
    }

    /// Body part data for someone, read once.
    pub(crate) fn body_parts(
        &mut self,
        order: &esm::LoadOrder,
        who: FormId,
    ) -> Option<Arc<BodyPartData>> {
        self.parts
            .entry(who)
            .or_insert_with(|| BodyPartData::of(order, who).map(Arc::new))
            .clone()
    }
}

/// What a line from the eye meets first within its reach.
pub(crate) enum Met {
    /// Someone (and the body part it lands on) or a scripted object, this
    /// far along.
    Thing {
        distance: f32,
        reference: FormId,
        part: Option<u8>,
    },
    /// Nothing; `true` when a wall stopped it.
    Nothing(bool),
}

/// What a shot or blow along `(eye, dir)` meets first within `reach`
/// (see the module notes): the living people and creatures in `talkers`
/// (shots: their capsules; blows: their bounds), scripted objects, and
/// the cell's walls, which stop it.
#[allow(clippy::too_many_arguments)]
pub(crate) fn first_met(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    caches: &mut PlayerAttack,
    talkers: &Talkers,
    cell_scripts: &CellScripts,
    collision: &CellCollision,
    rigs: &Query<(&Walker, &ActorRig)>,
    (eye, dir): ([f32; 3], [f32; 3]),
    reach: f32,
    melee: bool,
    now: f32,
) -> Met {
    first_met_past(
        order,
        state,
        caches,
        (talkers, cell_scripts, collision, rigs),
        (eye, dir),
        reach,
        (melee, None),
        now,
    )
}

/// [`first_met`], passing by `skip` (someone's own shot leaving their
/// body).
#[allow(clippy::too_many_arguments)]
pub(crate) fn first_met_past(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    caches: &mut PlayerAttack,
    (talkers, cell_scripts, collision, rigs): (
        &Talkers,
        &CellScripts,
        &CellCollision,
        &Query<(&Walker, &ActorRig)>,
    ),
    (eye, dir): ([f32; 3], [f32; 3]),
    reach: f32,
    (melee, skip): (bool, Option<FormId>),
    now: f32,
) -> Met {
    let mut best: Option<(f32, FormId, Option<u8>)> = None;
    for t in &talkers.0 {
        if state.dead.contains(&t.reference) || Some(t.reference) == skip {
            continue;
        }
        // Shots pass a ghost by (`SetGhost`: the projectiles' target
        // searches skip ghosts, `00816f10`, `00817b90`).
        if !melee && world::more_functions::is_ghost(state, t.reference) {
            continue;
        }
        let (radius, height) = *caches
            .bodies
            .entry(t.base)
            .or_insert_with(|| body(order, t.base));
        let data = caches.body_parts(order, t.reference);
        let rig = rigs.iter().find(|(w, _)| w.reference == t.reference);
        let cylinder = ray_body(eye, dir, t.position, radius, height);
        let met = ray_actor(rig, data.as_deref(), (eye, dir), cylinder, melee, now);
        if let Some((d, bone)) = met {
            if d <= reach && best.is_none_or(|(bd, _, _)| d < bd) {
                let part =
                    bone.zip(rig)
                        .zip(data.as_deref())
                        .and_then(|((bone, (_, rig)), data)| {
                            data.part_of_bone(&rig.skeleton.bones, bone)
                        });
                best = Some((d, t.reference, part));
            }
        }
    }
    let struck = cast(collision, (eye, dir), reach, melee);
    for r in cell_scripts
        .refs
        .iter()
        .filter(|r| meetable(order, state, r))
    {
        // An object Havok moves (`clutter`) is met where its body is now:
        // by its triangles in the collider, which move with it (its placed
        // bounds stay where it stood).
        let met = if collision.0.owns(r.reference.0) {
            struck
                .filter(|&(_, t)| collision.0.owner(t) == r.reference.0)
                .map(|(d, _)| d)
        } else {
            r.ray_hit(eye, dir)
        };
        if let Some(d) = met {
            // A person's shot leaving from inside an object's bounds (Easy
            // Pete on his porch) isn't stopped by them: the bounds stand in
            // for the object's collision here, and the shooter stands in
            // that space.
            let inside = skip.is_some() && d < 1.0;
            if !inside && d <= reach && best.is_none_or(|(bd, _, _)| d < bd) {
                best = Some((d, r.reference, None));
            }
        }
    }
    let wall = struck.map(|(d, _)| d);
    match best.filter(|(d, _, _)| wall.is_none_or(|w| w >= d - 5.0)) {
        Some((distance, reference, part)) => Met::Thing {
            distance,
            reference,
            part,
        },
        None => Met::Nothing(wall.is_some()),
    }
}

/// Someone's collision radius for a swing's gap (`008be420`, as
/// `fighting::Kit` reads it): people [`world::combat_ai::PERSON_RADIUS`],
/// a creature its skeleton's bound (25 × its scale without one).
fn swing_radius(order: &esm::LoadOrder, who: FormId, rig: Option<(&Walker, &ActorRig)>) -> f32 {
    if world::combat::creature_reach(order, who).is_none() {
        return world::combat_ai::PERSON_RADIUS;
    }
    match rig {
        Some((w, r)) => match r.skeleton.bound {
            Some(b) => world::combat_ai::creature_radius(b.half_extents, w.scale),
            None => 25.0 * w.scale,
        },
        None => 25.0,
    }
}

/// Someone's bounds' bottom and top for a swing's gap (`009a64d0`, vtable
/// `+0x1d8`/`+0x1dc`): their base's `OBND` z (`world::npc_aim::bound_z`).
/// A base without `OBND` is given zero bounds: unresolved guess, the empty
/// bound object's value isn't traced.
fn melee_bound_z(order: &esm::LoadOrder, base: FormId) -> [f32; 2] {
    world::npc_aim::bound_z(order, base).unwrap_or([0.0; 2])
}

/// Whether someone swims (actor `+0x14d`, as `IsSwimming` reads it).
fn melee_swimming(state: &world::scripting::GameState, who: FormId) -> bool {
    state.more.seen.get(&who).is_some_and(|s| s.swimming)
}

/// Whom the player's swing meets (`Actor::MeleeAttack`, `00899200`):
/// the person or creature [`world::melee::find_target`] picks
/// (`009a60e0`: within reach of the gap between the bodies and the hit
/// cone, nearest its middle), when the collider doesn't stand between
/// the eye and their middle (`0088b880`); the body part the view's ray
/// meets on them within 200 units (`009b6620`), if any. Without one, a
/// scripted object along the view within reach (the game's object hit,
/// `008ae660`, isn't traced further).
#[allow(clippy::too_many_arguments)]
pub(crate) fn melee_met(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    caches: &mut PlayerAttack,
    (talkers, cell_scripts, collision, rigs): (
        &Talkers,
        &CellScripts,
        &CellCollision,
        &Query<(&Walker, &ActorRig)>,
    ),
    (eye, dir): ([f32; 3], [f32; 3]),
    swing: &world::melee::Swing,
    reach: f32,
    now: f32,
) -> Met {
    // A body its critical stage ended has no collision left (`008a1a70` →
    // `0057b520(0)`, `world::more_functions::body_gone`): no swing meets it.
    let near: Vec<world::melee::Body> = talkers
        .0
        .iter()
        .filter(|t| !world::more_functions::body_gone(state, t.reference))
        .map(|t| {
            let rig = rigs.iter().find(|(w, _)| w.reference == t.reference);
            world::melee::Body {
                reference: t.reference,
                position: rig.map_or(t.position, |(w, _)| w.position),
                radius: swing_radius(order, t.reference, rig),
                bound_z: melee_bound_z(order, t.base),
                swimming: melee_swimming(state, t.reference),
                dead: state.dead.contains(&t.reference),
            }
        })
        .collect();
    if let Some(b) = world::melee::find_target(swing, None, &near)
        .and_then(|r| near.iter().find(|b| b.reference == r))
    {
        let base = talkers
            .0
            .iter()
            .find(|t| t.reference == b.reference)
            .map(|t| t.base)
            .unwrap_or(b.reference);
        let (radius, height) = *caches
            .bodies
            .entry(base)
            .or_insert_with(|| body(order, base));
        let middle = [b.position[0], b.position[1], b.position[2] + height * 0.5];
        let to = [middle[0] - eye[0], middle[1] - eye[1], middle[2] - eye[2]];
        let distance = (to[0] * to[0] + to[1] * to[1] + to[2] * to[2])
            .sqrt()
            .max(1e-3);
        let toward = to.map(|v| v / distance);
        let blocked = cast(collision, (eye, toward), distance, true)
            .is_some_and(|(d, _)| d < distance - radius);
        if !blocked {
            let data = caches.body_parts(order, b.reference);
            let rig = rigs.iter().find(|(w, _)| w.reference == b.reference);
            let cylinder = ray_body(eye, dir, b.position, radius, height);
            let part = ray_actor(rig, data.as_deref(), (eye, dir), cylinder, true, now)
                .filter(|(d, _)| *d <= 200.0)
                .and_then(|(_, bone)| bone.zip(rig).zip(data.as_deref()))
                .and_then(|((bone, (_, rig)), data)| data.part_of_bone(&rig.skeleton.bones, bone));
            return Met::Thing {
                distance,
                reference: b.reference,
                part,
            };
        }
    }
    match first_met(
        order,
        state,
        caches,
        talkers,
        cell_scripts,
        collision,
        rigs,
        (eye, dir),
        reach,
        true,
        now,
    ) {
        Met::Thing { reference, .. } if talkers.0.iter().any(|t| t.reference == reference) => {
            Met::Nothing(false)
        }
        met => met,
    }
}

/// Where a shot or blow along `(eye, dir)` meets the collider within
/// `reach`. Shots are cast on Havok's projectile layer (6): the game's
/// collision filter (`physics::layers`, `00c84930` for casts) lets them
/// through bodies on layers it doesn't touch, such as a `TRANSPARENT`
/// chain-link fence a walker can't pass. That the shot's own cast uses
/// layer 6 is taken from the layer's name (the projectile's cast filter
/// word isn't traced). Blows meet every surface (the melee pick's layer
/// isn't traced).
pub(crate) fn cast(
    collision: &CellCollision,
    (eye, dir): ([f32; 3], [f32; 3]),
    reach: f32,
    melee: bool,
) -> Option<(f32, u32)> {
    if melee {
        collision.0.raycast(eye, dir, reach)
    } else {
        collision
            .0
            .raycast_layer(eye, dir, reach, physics::layers::layer::PROJECTILE)
    }
}

/// A scripted object a shot or blow can meet. Trigger volumes aren't solid:
/// shots go through them (the player standing in one caught every shot at
/// 0 units before). A disabled reference has no 3D to meet (inferred from
/// the reference-script pass `0054c740`, which treats "has 3D"
/// (`Get3D` `0043fcd0`) and "disabled" as the two separate cases): before
/// `VCG02BottleMarkerREF.Enable` the tutorial's unseen bottles counted hits.
pub(crate) fn meetable(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    r: &world::scripting::Interactive,
) -> bool {
    r.script.is_some()
        && r.trigger.is_none()
        && world::enabled_now(order, r.reference, &state.disabled)
        // A body its critical stage ended has its collision taken out
        // (`008a1a70` → `0057b520(0)`) and its 3D culled (`00450f90(1)`,
        // `world::more_functions::body_gone`): nothing left to meet.
        && !world::more_functions::body_gone(state, r.reference)
}

/// The player's critical on someone alive (`0089a760`): "Sneak Attack
/// Critical on <name>" (hit flag 0x400) or "Critical Strike on <name>",
/// with the very happy Vault Boy.
pub(crate) fn critical_message(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    messages: &mut crate::hud::HudMessages,
    target: FormId,
    sneak_attack: bool,
) {
    let (setting, exe) = if sneak_attack {
        ("sSneakAttackCriticalStrike", "Sneak Attack Critical on")
    } else {
        ("sCriticalStrike", "Critical Strike on")
    };
    let words =
        world::scripting::game_setting_text(order, setting).unwrap_or_else(|| exe.to_string());
    let name = world::script_functions::full_name(order, state, target).unwrap_or_default();
    messages.queue.push(crate::hud::HudMessage::with_icon(
        format!("{words} {name}"),
        Some(CRITICAL_ICON),
    ));
}

/// Says what a hit did, as the attacks print it.
pub(crate) fn tell_hit(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    caches: &mut PlayerAttack,
    target: FormId,
    distance: f32,
    hit: &world::combat::Hit,
) -> String {
    let left = combat::health(order, state, target).unwrap_or(0.0).max(0.0);
    let place = caches
        .body_parts(order, target)
        .zip(hit.part)
        .and_then(|(d, p)| d.part(p).map(|p| p.name.to_lowercase()))
        .map_or(String::new(), |name| format!(" in the {name}"));
    let mut said = format!(
        "Hit {target}{place} at {distance:.0} units for {:.1} ({left:.1} left)",
        hit.dealt
    );
    if hit.critical {
        said.push_str(", a critical");
    }
    if let Some(hurt) = &hit.hurt {
        if hurt.crippled {
            said.push_str(&format!("; {} crippled", hurt.name));
        }
        if hurt.dropped.is_some() {
            said.push_str("; dropped their weapon");
        }
        if hurt.staggered {
            said.push_str("; staggered");
        }
    }
    if hit.knocked_down {
        said.push_str("; knocked down");
    }
    // Fatigue damage (`world::fatigue`): fists' half, a bean bag's.
    if hit.fatigue > 0.0 {
        let fatigue = world::fatigue::fatigue(order, state, target).unwrap_or(0.0);
        said.push_str(&format!("; fatigue {:.1} ({fatigue:.1} left)", hit.fatigue));
    }
    said
}

/// Where a ray meets someone on screen: how far along it, and the bone it
/// lands on (`None`: they have no rig, or no bone was found).
fn ray_actor(
    rig: Option<(&Walker, &ActorRig)>,
    data: Option<&BodyPartData>,
    (eye, dir): ([f32; 3], [f32; 3]),
    cylinder: Option<f32>,
    melee: bool,
    now: f32,
) -> Option<(f32, Option<usize>)> {
    let Some((walker, rig)) = rig else {
        return cylinder.map(|d| (d, None));
    };
    let pose = rig.pose_now(now);
    let placement = walker.placement();
    let capsule = rig
        .skeleton
        .ragdoll
        .as_ref()
        .and_then(|r| r.ray_hit(&pose, &placement, eye, dir));
    let nearest = || {
        preview::ragdoll::nearest_bone(&pose, &placement, eye, dir, |i| {
            data.is_some_and(|d| d.part_of_bone(&rig.skeleton.bones, i).is_some())
        })
    };
    match (rig.skeleton.ragdoll.is_some(), melee) {
        // Shots meet the bodies or miss.
        (true, false) => capsule.map(|(d, bone)| (d, Some(bone))),
        // Melee reaches by the bounds and lands where the view meets a
        // body, else on the bone nearest the view.
        (true, true) => {
            let d = cylinder?;
            Some((d, capsule.map(|(_, b)| b).or_else(nearest)))
        }
        (false, _) => cylinder.map(|d| (d, nearest())),
    }
}

/// People who've dropped their weapon (`world::body_parts::hurt_part`)
/// have its model hidden.
pub fn show_dropped_weapons(state: Res<DialogueState>, mut rigs: Query<(&Walker, &mut ActorRig)>) {
    if state.0.dropped.is_empty() {
        return;
    }
    for (walker, mut rig) in &mut rigs {
        let disarmed = state.0.dropped.iter().any(|(w, _)| *w == walker.reference);
        if rig.disarmed != disarmed {
            rig.disarmed = disarmed;
        }
    }
}

impl PlayerAttack {
    /// Rounds left in the clip of the weapon in hand (`None`: a full one).
    pub fn in_clip(&self) -> Option<u32> {
        self.in_clip
    }
}

/// The health and ammunition line.
#[derive(Component)]
pub struct HudText;

pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        Text::new(String::new()),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.75, 0.3)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(16.0),
            ..default()
        },
        HudText,
    ));
}

/// A body's half width and height from its base record's bounds (people
/// without bounds: 25 and 130).
pub(crate) fn body(order: &esm::LoadOrder, base: FormId) -> (f32, f32) {
    order
        .get(base)
        .and_then(|r| r.record().ok())
        .and_then(|r| {
            let s = r
                .get(esm::FourCC::new(b"OBND"))
                .filter(|s| s.data.len() >= 12)?;
            let v = |i: usize| f32::from(i16::from_le_bytes([s.data[i * 2], s.data[i * 2 + 1]]));
            let half = ((v(3) - v(0)).max(v(4) - v(1)) * 0.5).max(10.0);
            let height = (v(5) - v(2)).max(20.0);
            Some((half, height))
        })
        .filter(|(h, t)| *h > 0.0 && *t > 0.0)
        .unwrap_or((25.0, 130.0))
}

/// Where a ray from the eye meets an upright cylinder around someone's
/// feet, if it does.
pub(crate) fn ray_body(
    eye: [f32; 3],
    dir: [f32; 3],
    feet: [f32; 3],
    radius: f32,
    height: f32,
) -> Option<f32> {
    let (ox, oy) = (eye[0] - feet[0], eye[1] - feet[1]);
    let a = dir[0] * dir[0] + dir[1] * dir[1];
    let b = 2.0 * (ox * dir[0] + oy * dir[1]);
    let c = ox * ox + oy * oy - radius * radius;
    let t = if a < 1e-9 {
        (c <= 0.0).then_some(0.0)?
    } else {
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        }
        let near = (-b - disc.sqrt()) / (2.0 * a);
        let far = (-b + disc.sqrt()) / (2.0 * a);
        if far < 0.0 {
            return None;
        }
        near.max(0.0)
    };
    let z = eye[2] + dir[2] * t - feet[2];
    (0.0..=height).contains(&z).then_some(t)
}

/// What the Aim control's handling needs: the view (switching or not),
/// V.A.T.S., the bindings.
type AimGates<'w> = (
    Res<'w, crate::player_camera::PlayerView>,
    Res<'w, crate::vats::Vats>,
    Option<Res<'w, crate::controls::Controls>>,
);

/// Left click: an attack, if one is ready; R: reloading.
#[allow(clippy::too_many_arguments)]
pub fn player_attack(
    time: Res<Time>,
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    (mut state, player, conversation, menus): (
        ResMut<DialogueState>,
        Res<Player>,
        Res<Conversation>,
        Res<Menus>,
    ),
    (talkers, cell_scripts, collision): (Res<Talkers>, Res<CellScripts>, Res<CellCollision>),
    mut attack: ResMut<PlayerAttack>,
    mut sounds: ResMut<SoundRequests>,
    mut hits: ResMut<crate::hiteffects::HitReports>,
    cameras: Query<&Transform, With<FlyCamera>>,
    mut hud: Query<&mut Text, With<HudText>>,
    rigs: Query<(&Walker, &ActorRig)>,
    (aim_gates, mut messages, mut load): (
        AimGates,
        ResMut<crate::hud::HudMessages>,
        ResMut<crate::scripts::LoadRequest>,
    ),
) {
    let order = &game.0.order;
    let now = time.elapsed_secs();
    let state = &mut state.0;
    let weapon = combat::weapon_in_hand(order, state, PLAYER_REF);
    let id = weapon.as_ref().map(|w| w.form_id);
    if attack.weapon != id {
        attack.weapon = id;
        attack.in_clip = None;
        // A change of weapon starts holstered (`0088db20`: equipping with
        // nothing in hand puts the player's weapon away; `0088d7d0`:
        // unequipping a weapon does too).
        attack.out = false;
        attack.readied_at = None;
        attack.iron_sights = false;
    }
    if attack.out {
        state.weapon_out.insert(PLAYER_REF);
    } else {
        state.weapon_out.remove(&PLAYER_REF);
    }
    // The kind of ammunition in use, and how much of it is carried.
    let ammo_held = |state: &world::scripting::GameState, w: &Weapon| {
        w.ammo_in_use(order, state, PLAYER_REF)
            .map(|a| (a, state.item_count(order, PLAYER_REF, a).max(0) as u32))
    };
    // The HUD line.
    let health = combat::health(order, state, PLAYER_REF)
        .unwrap_or(0.0)
        .max(0.0);
    let full = combat::max_health(order, state, PLAYER_REF).unwrap_or(0.0);
    let mut line = format!(
        "HP {health:.0}/{full:.0}  AP {:.0}/{:.0}",
        world::vats::action_points(order, state),
        world::vats::max_action_points(order, state)
    );
    match &weapon {
        Some(w) => {
            line.push_str(&format!("    {}", w.name));
            if let Some((_, held)) = ammo_held(state, w) {
                let clip = attack.in_clip.unwrap_or(w.clip).min(held);
                line.push_str(&format!("  {clip}/{held}"));
            }
            if now < attack.reloaded_at {
                line.push_str("  (reloading)");
            }
        }
        None => line.push_str("    Fists"),
    }
    if !attack.out {
        line.push_str("  (holstered: R or attack draws)");
    }
    let crippled = world::body_parts::crippled_parts(order, state, PLAYER_REF);
    if !crippled.is_empty() {
        line.push_str(&format!("    Crippled: {}", crippled.join(", ")));
    }
    let dead = state.dead.contains(&PLAYER_REF);
    if dead {
        line = "You are dead.".into();
    }
    for mut text in &mut hud {
        if text.0 != line {
            text.0 = line.clone();
        }
    }
    // After death (`world::player_death`, `0093e860`): the scope goes,
    // and after `fPlayerDeathReloadTime` the most recent save loads (the
    // viewer's one save, its quick save), or with none the game's main
    // menu would open (this viewer has none).
    let dt = time.delta_secs();
    let reload_time =
        world::scripting::game_setting(order, "fPlayerDeathReloadTime").unwrap_or(5.0);
    let save_exists = std::path::Path::new(crate::scripts::QUICKSAVE).exists();
    match attack
        .death
        .update(dead, dead, dt, reload_time, save_exists)
    {
        world::player_death::DeathStep::Started => {
            attack.iron_sights = false;
            attack.blocking = false;
        }
        world::player_death::DeathStep::Reload => {
            if save_exists {
                println!("The player died: loading the most recent save.");
                load.0 = true;
            } else {
                println!(
                    "The player died with no save: the game would go back to its main menu \
                     (`007d0a70`), which this viewer doesn't have."
                );
            }
        }
        world::player_death::DeathStep::Ask => {
            // Only with `fPlayerDeathReloadTime` 0 or below (the data
            // keeps 5): the game's message box isn't shown here.
            println!("The player died: the game would ask to reload or go to the main menu.");
        }
        world::player_death::DeathStep::Wait => {}
    }
    // The counter-attack timer and the swap timer run on.
    attack.counter_timer = (attack.counter_timer - dt).max(0.0);
    attack.ammo_swap_timer += dt;
    // Hits the player blocked (`world::melee`): the block hit plays and
    // the counter-attack window opens.
    if state.blocked_hits.contains(&PLAYER_REF) {
        attack.block_hit_at = Some(now);
        attack.counter_timer = world::melee::Settings::read(order).counter_attack_time;
    }
    state.blocked_hits.clear();
    let busy = !player.walking
        || !player.ready
        || conversation.0.is_some()
        || menus.is_open()
        || state.dead.contains(&PLAYER_REF)
        || state.controls_off[world::scripting::controls::FIGHTING];
    if busy {
        attack.iron_sights = false;
        attack.blocking = false;
        state.blocking.remove(&PLAYER_REF);
        return;
    }
    // The Aim control (6, the right mouse button), as `0093e860` reads it
    // (at `00941f4f`): held, a drawn gun's sights come up (`008bb650(1, 0,
    // 0)`), unless the view is switching or V.A.T.S. is on; let go, they
    // go down. A drawn melee weapon or fists would block (`00894cc0(1)`),
    // which isn't here.
    let (view, vats, controls) = aim_gates;
    let controls = controls.map_or(crate::controls::Controls::default(), |c| *c);
    let aim = controls.aim;
    let switching = view.camera.want_third != view.camera.actually_third;
    let readying_now = now < attack.busy_until;
    let control = world::iron_sights::aim_control(weapon.as_ref().map(|w| w.animation), attack.out);
    if aim.pressed(&keys, &mouse) {
        if !attack.iron_sights
            && !switching
            && !vats.is_on()
            && control == world::iron_sights::AimControl::IronSights
        {
            attack.iron_sights = true;
        }
        // A melee weapon or fists block (`00894cc0(1)` → `00894940`:
        // `BlockIdle` as anim action 7), when no other action plays (anim
        // action none, or an attack that has ended).
        let attacking = now < attack.next;
        if control == world::iron_sights::AimControl::Block
            && !attack.blocking
            && !readying_now
            && !attacking
            && now >= attack.reloaded_at
            && !vats.is_on()
        {
            attack.blocking = true;
        }
    } else {
        attack.iron_sights = false;
        attack.blocking = false;
    }
    if control != world::iron_sights::AimControl::Block {
        attack.blocking = false;
    }
    if attack.blocking {
        let heading = cameras.single().map_or(0.0, |c| {
            let f = c.forward().as_vec3();
            f.x.atan2(-f.z)
        });
        state.blocking.insert(PLAYER_REF, heading);
    } else {
        state.blocking.remove(&PLAYER_REF);
    }
    // The Ammo Swap control (18; `world::ammo_swap`, `0093e860` →
    // `009462c0`): the next carried kind loads, with the reload animation
    // when the swap timer has passed and the weapon is out.
    let swap_key = controls.ammo_swap;
    if swap_key.just_pressed(&keys, &mouse) && attack.out && !readying_now {
        if let Some(w) = weapon.as_ref() {
            let (swap, reset) = world::ammo_swap::press(
                order,
                state,
                PLAYER_REF,
                w,
                attack.out,
                attack.ammo_swap_timer,
            );
            if reset {
                attack.ammo_swap_timer = 0.0;
            }
            if let Some(swap) = swap {
                state.ammo_loaded.insert(PLAYER_REF, swap.ammo);
                let carried = state.item_count(order, PLAYER_REF, swap.ammo).max(0) as u32;
                if swap.animated {
                    let rate = combat::reload_rate(order, state, PLAYER_REF, Some(w)).max(1e-3);
                    attack.in_clip = Some(w.clip.min(carried));
                    attack.reloaded_at = now + w.reload_time / rate;
                    attack.reload_started = Some(now);
                    attack.reload_rate = rate;
                } else {
                    // `ReloadWeaponNV(…, 0, …)`: no reload animation (the
                    // clip taken as filled at once: inferred).
                    attack.in_clip = Some(w.clip.min(carried));
                }
                let name = order
                    .get(swap.ammo)
                    .and_then(|r| r.record().ok())
                    .and_then(|r| r.get(esm::FourCC::new(b"FULL")).map(|s| s.zstring()))
                    .unwrap_or_default();
                println!("Ammo swap: {name}.");
            }
        }
    }
    // A reload takes the weapon's reload time at the reload rate
    // (`world::combat::reload_rate`: Agility and Rapid Reload).
    let start_reload =
        |attack: &mut PlayerAttack, state: &world::scripting::GameState, w: &Weapon| {
            let rate = combat::reload_rate(order, state, PLAYER_REF, Some(w)).max(1e-3);
            attack.in_clip = Some(w.clip);
            attack.reloaded_at = now + w.reload_time / rate;
            attack.reload_started = Some(now);
            attack.reload_rate = rate;
        };
    // The Ready Item key (R), as the game reads it: `world::combat::
    // ReadyKey`. "Reloadable": a gun with its ammunition.
    let reloadable = weapon
        .as_ref()
        .is_some_and(|w| ammo_held(state, w).is_some());
    let readying = now < attack.busy_until;
    let key = if keys.pressed(KeyCode::KeyR) {
        combat::KeyState::Held
    } else if keys.just_released(KeyCode::KeyR) {
        combat::KeyState::Released
    } else {
        combat::KeyState::Up
    };
    let (out, mut ready) = (attack.out, std::mem::take(&mut attack.ready));
    let action = ready.update(key, time.delta_secs(), out, reloadable, readying);
    attack.ready = ready;
    match action {
        combat::ReadyAction::Draw => attack.set_out(true, now),
        combat::ReadyAction::PutAway => attack.set_out(false, now),
        combat::ReadyAction::Reload => {
            if let Some(w) = weapon.as_ref() {
                start_reload(&mut attack, state, w);
            }
            return;
        }
        combat::ReadyAction::Nothing => {}
    }
    // Power attacks (`world::melee`, `00948310`): with a melee weapon or
    // fists out, holding the Attack control past `fPowerAttackDelay` brings
    // a power attack, after the attack playing ends.
    let use_key = controls.attack;
    let melee_out = attack.out && weapon.as_ref().is_none_or(|w| w.is_melee());
    let melee_settings = world::melee::Settings::read(order);
    let mut power_now = false;
    if melee_out && use_key.pressed(&keys, &mouse) && !use_key.just_pressed(&keys, &mouse) {
        if !attack.power && !attack.power_queued {
            attack.power_timer += dt;
        }
        // Over-encumbered players don't (`0093e860`'s vfunc +0x358).
        let heavy = state.over_encumbered(order, PLAYER_REF);
        if !heavy && attack.power_timer > melee_settings.power_attack_delay {
            attack.power_timer = 0.0;
            let playing = attack
                .fired_at
                .is_some_and(|t| now < t + attack.attack_length.max(0.0));
            if playing {
                attack.power_queued = true;
            } else {
                power_now = true;
            }
        }
    } else if !use_key.pressed(&keys, &mouse) {
        attack.power_timer = 0.0;
    }
    if attack.power_queued {
        let ended = attack
            .fired_at
            .is_none_or(|t| now >= t + attack.attack_length.max(0.0));
        if ended {
            attack.power_queued = false;
            power_now = true;
        }
    }
    let pressed = use_key.just_pressed(&keys, &mouse);
    if !power_now && (!pressed || now < attack.next || now < attack.reloaded_at) {
        return;
    }
    if pressed {
        attack.power_timer = 0.0;
    }
    // Attacking with the weapon holstered draws it instead (`00948310`:
    // the attack control just pressed and the weapon not out).
    if !attack.out {
        if !readying {
            attack.set_out(true, now);
        }
        return;
    }
    if readying {
        return;
    }
    // Ammunition: each shot uses the weapon's ammo use from what's carried
    // (the clip counts down; an empty clip reloads first).
    if let Some(w) = weapon.as_ref() {
        if let Some((ammo, held)) = ammo_held(state, w) {
            let use_ = u32::from(w.ammo_use.max(1));
            if held < use_ {
                println!("Out of ammunition for the {}.", w.name);
                return;
            }
            let clip = attack.in_clip.unwrap_or(w.clip);
            if clip < use_ {
                start_reload(&mut attack, state, w);
                return;
            }
            attack.in_clip = Some(clip - use_);
            if let Some(n) = state.items.get_mut(&(PLAYER_REF, ammo)) {
                *n -= use_ as i32;
            }
            // The round's case or cell, some of the time (`world::combat::
            // ammo_item_recovered`: the ammunition's own chance, Hand
            // Loader's doubling).
            let roll = state.roll();
            if let Some(item) = combat::ammo_item_recovered(order, state, PLAYER_REF, w, ammo, roll)
            {
                *state.items.entry((PLAYER_REF, item)).or_insert(0) += 1;
            }
        }
    }
    // Every attack wears the weapon a little (`world::combat::attack_wear`,
    // through the perks' "Modify Item Damage").
    if let Some(w) = weapon.as_ref() {
        let ammo = w.ammo_in_use(order, state, PLAYER_REF);
        let wear = combat::attack_wear(order, ammo);
        combat::damage_weapon(order, state, PLAYER_REF, w, wear);
        // Heard for a while (`world::noise::attacked`).
        world::noise::attacked(order, state, PLAYER_REF, w);
    }
    attack.next = now + weapon.as_ref().map_or(0.5, |w| w.shot_interval());
    attack.fired_at = Some(now);
    attack.attack_rate = combat::attack_rate(order, state, PLAYER_REF, weapon.as_ref());
    // Attacking ends a block (`00948310`: `00894cc0(0)` in anim action 7).
    attack.blocking = false;
    state.blocking.remove(&PLAYER_REF);
    // Which attack (`00948310`, at `009498cf`): a melee attack while
    // sneaking is the power attack (fists, or a melee weapon that isn't
    // automatic); an unarmed one within the counter-attack timer with the
    // perk the `Counter`; a power attack goes the way the player moves.
    let sneaking = state.player_sneaking;
    let unarmed = weapon
        .as_ref()
        .is_none_or(|w| w.skill == world::combat::av::UNARMED);
    let mut group = weapon
        .as_ref()
        .map(|w| w.attack_animation)
        .filter(|&a| a != 0xff)
        .unwrap_or(world::melee::group::ATTACK_RIGHT);
    if power_now {
        group = world::melee::group::ATTACK_POWER;
    }
    if melee_out
        && sneaking
        && weapon
            .as_ref()
            .is_none_or(|w| w.flags1 & world::vats::flags::AUTOMATIC == 0)
    {
        group = world::melee::group::ATTACK_POWER;
    }
    if melee_out && world::melee::counter_attack(order, state, unarmed, attack.counter_timer) {
        group = world::melee::group::COUNTER;
    }
    if group == world::melee::group::ATTACK_POWER {
        let legs = |a: u16| {
            world::scripting::Facts {
                order,
                state,
                speaker: None,
            }
            .current_actor_value(PLAYER_REF, a)
            .unwrap_or(100.0)
                <= 0.0
        };
        let m = player.moving;
        group = world::melee::power_attack_group(
            order,
            state,
            world::melee::Moving {
                forward: m.forward,
                back: m.backward,
                left: m.left,
                right: m.right,
            },
            sneaking,
            legs(29) && legs(30),
            unarmed,
        );
    }
    // An unarmed attack may become an uppercut or a cross by the Unarmed
    // skill (`world::melee::unarmed_special_group`, `00893a40`).
    if melee_out && world::melee::may_turn_special(weapon.as_ref(), true, sneaking, false) {
        let skill = world::scripting::Facts {
            order,
            state,
            speaker: None,
        }
        .current_actor_value(PLAYER_REF, world::combat::av::UNARMED)
        .unwrap_or(0.0) as f32;
        let roll = (state.roll() % 1_000_000) as f32 / 10_000.0;
        let specials = world::melee::UnarmedSpecials::read(order);
        if let Some(g) = world::melee::unarmed_special_group(&specials, skill, roll) {
            group = g;
            println!(
                "Unarmed special: {}.",
                world::melee::group_file_stem(g).unwrap_or_default()
            );
        }
    }
    attack.attack_group = group;
    attack.power = melee_out && world::animation::kind_of(group) == 6;
    if attack.power {
        state.power_attacking.insert(PLAYER_REF);
        println!(
            "Power attack: {}.",
            world::melee::group_file_stem(group).unwrap_or_default()
        );
    } else {
        state.power_attacking.remove(&PLAYER_REF);
    }
    // A gun's firing sound and muzzle flash (`weapon_fx`, `00523150` →
    // `0083ac30`; melee attacks have none: a swing that meets no one plays
    // its `TNAM`, below). Thrown weapons keep their `SNAM` as before (not
    // traced: `00523150` doesn't play it for them).
    match weapon.as_ref() {
        Some(w) if world::explosions::is_thrown(w) => sounds.0.extend(w.sound),
        Some(w) if !w.is_melee() => crate::weapon_fx::fired(crate::weapon_fx::Fired {
            shooter: PLAYER_REF,
            weapon: w.form_id,
            from: None,
        }),
        _ => {}
    }
    // What the attack meets first along the view.
    let Ok(camera) = cameras.single() else {
        return;
    };
    let eye = game_point(camera.translation);
    let f = camera.forward().as_vec3();
    let view = [f.x, -f.z, f.y];
    // Grenades and thrown weapons leave the hand instead (`explosives`;
    // `00523150`: animation types 10–13).
    if let Some(w) = weapon.as_ref().filter(|w| world::explosions::is_thrown(w)) {
        crate::explosives::throw(crate::explosives::Launch {
            thrower: PLAYER_REF,
            weapon: w.clone(),
            origin: eye,
            aim: crate::explosives::Aim::Along(view),
        });
        return;
    }
    let melee = weapon.as_ref().is_none_or(|w| w.is_melee());
    // Guns: the projectile's range, a shot's pellets each carrying an even
    // share of the damage, flying within the weapon's cone (`world::combat::
    // Weapon::shot`); melee: the weapon's reach between the bodies' edges
    // (from the eye's axis, so with the player's own radius added).
    let ammo = weapon
        .as_ref()
        .and_then(|w| w.ammo_in_use(order, state, PLAYER_REF));
    let (count, cone, reach) = match &weapon {
        Some(w) if !melee => {
            let (count, cone) = w.shot(order, ammo);
            (count, cone, w.range(order).unwrap_or(SHOT_RANGE))
        }
        _ => (
            1,
            0.0,
            Weapon::melee_reach(weapon.as_ref()) + physics::CharacterShape::PLAYER.radius,
        ),
    };
    let pellet = weapon.clone().map(|mut w| {
        w.damage /= count as f32;
        // The ammunition's own projectile when it names one (the rockets'
        // HE and HV loads, `world::combat::fired_projectile`).
        if !melee {
            w.projectile = world::combat::fired_projectile(order, state, PLAYER_REF, &w);
        }
        w
    });
    let heading = view[0].atan2(view[1]);
    let pitch = view[2].clamp(-1.0, 1.0).asin();
    // A swing (`world::melee`, `008990f0`): from the player's feet, facing
    // the view, the weapon's reach, the cone of the attack playing.
    let cones = world::melee::Cones::read(order);
    let swing = melee.then(|| world::melee::Swing {
        at: player.character.feet,
        heading,
        radius: physics::CharacterShape::PLAYER.radius,
        bound_z: melee_bound_z(order, world::dialogue::PLAYER_BASE),
        swimming: melee_swimming(state, PLAYER_REF),
        slope_difference: world::scripting::game_setting(order, "fAICombatSlopeDifference")
            .unwrap_or(48.0),
        reach: world::melee::swing_reach(order, weapon.as_ref(), 1.0, false),
        cone: cones.for_attack(attack.attack_group),
        dead_mult: cones.dead_mult,
        player: true,
        vats_playback: false,
    });
    for _ in 0..count {
        // Within the cone, uniform in the angle off the view (so shots
        // bunch toward the middle), as the game does: r = U(0, cone), θ =
        // U(0, 2π), added to heading and pitch.
        let unit = |v: u64| (v % 1_000_000) as f32 / 1_000_000.0;
        let r = cone * unit(state.roll());
        let theta = std::f32::consts::TAU * unit(state.roll());
        let (h, p) = (heading + r * theta.cos(), pitch + r * theta.sin());
        let dir = [h.sin() * p.cos(), h.cos() * p.cos(), p.sin()];
        // A plasma bolt (a missile that isn't hitscan) flies from the eye
        // and strikes on its way (`bolts`); beams and bullets strike now.
        if let Some(bolt) = pellet
            .as_ref()
            .filter(|w| !melee && crate::bolts::flies(order, w))
        {
            crate::bolts::fire(order, state, PLAYER_REF, bolt, (eye, dir));
            continue;
        }
        // The nearest thing met: how far, who, and the body part; a swing
        // finds its target by the game's rule (`melee_met`).
        let met = if let Some(swing) = swing.as_ref() {
            melee_met(
                order,
                state,
                &mut attack,
                (&talkers, &cell_scripts, &collision, &rigs),
                (eye, dir),
                swing,
                reach,
                now,
            )
        } else {
            first_met(
                order,
                state,
                &mut attack,
                &talkers,
                &cell_scripts,
                &collision,
                &rigs,
                (eye, dir),
                reach,
                melee,
                now,
            )
        };
        // A shot striking the world: its impact (`hiteffects`).
        let struck = cast(&collision, (eye, dir), reach, melee);
        let gun = weapon.as_ref().filter(|w| !w.is_melee()).map(|w| w.form_id);
        let Met::Thing {
            distance: d,
            reference: target,
            part,
        } = met
        else {
            let wall = matches!(met, Met::Nothing(true));
            // A melee swing that meets no one (`00899200`).
            if let Some(w) = weapon.as_ref().filter(|w| w.is_melee()) {
                crate::weapon_fx::swung(PLAYER_REF, w.form_id);
            }
            if let (Some(s), Some(g)) = (struck, gun) {
                hits.shot_on_world(&collision.0, (eye, dir), s, PLAYER_REF, g);
            }
            let layer = struck.map(|(_, t)| collision.0.layer(t));
            // A surface the shot's layer passes (a chain-link fence).
            let passed = collision
                .0
                .raycast(eye, dir, reach)
                .filter(|&(d, _)| struck.is_none_or(|(s, _)| d < s - 1.0))
                .map_or(String::new(), |(d, t)| {
                    format!(
                        " (through a layer {} surface at {d:.0} units)",
                        collision.0.layer(t)
                    )
                });
            println!(
                "The attack hit nothing{}{passed}.",
                match layer.filter(|_| wall) {
                    Some(physics::ANY_LAYER) => " but a wall".to_string(),
                    Some(l) => format!(" but a wall (layer {l})"),
                    None => String::new(),
                }
            );
            continue;
        };
        // A sneak attack, as the hit works it out (`world::scripting`):
        // sneaking, and the target not detecting the player.
        let sneak_attack = state.player_sneaking
            && world::scripting::Facts {
                order,
                state,
                speaker: None,
            }
            .detection(target, PLAYER_REF)
            .is_none_or(|v| v < 1);
        let alive = !state.dead.contains(&target);
        // A power attack's damage × `fDamagePowerAttackBonus`, not while
        // sneaking (`009b5170`, `world::melee::power_attack_mult`).
        let power = attack.power && !state.player_sneaking;
        // An unarmed uppercut or cross (`world::melee::special_of`).
        let special = if melee {
            world::melee::special_of(weapon.as_ref(), attack.attack_group)
        } else {
            world::melee::Special::None
        };
        let hit = Runner::new(order, &scripts.0, state).blow_at(
            PLAYER_REF,
            target,
            pellet.as_ref(),
            part,
            world::melee::Blow { power, special },
        );
        // The player's critical on someone alive (`0089a760`): "Sneak Attack
        // Critical on <name>" (hit flag 0x400) or "Critical Strike on
        // <name>", with the very happy Vault Boy.
        if hit.as_ref().is_some_and(|h| h.critical) && alive {
            critical_message(order, state, &mut messages, target, sneak_attack);
        }
        let Some(hit) = hit else {
            // An object (a scripted bottle): its impact where the shot
            // meets the cell's collision there.
            if let (Some(s), Some(g)) = (struck.filter(|(w, _)| (w - d).abs() < 5.0), gun) {
                hits.shot_on_world(&collision.0, (eye, dir), s, PLAYER_REF, g);
            }
            println!("Hit {target} at {d:.0} units.");
            continue;
        };
        hits.0.push(crate::hiteffects::HitReport {
            attacker: PLAYER_REF,
            target: Some(target),
            weapon: id,
            point: [0, 1, 2].map(|k| eye[k] + dir[k] * d),
            havok: None,
            normal: None,
            triangle: None,
            direction: dir,
            on_body: true,
            part,
            damage: hit.dealt,
            killed: state.dead.contains(&target),
        });
        println!("{}.", tell_hit(order, state, &mut attack, target, d, &hit));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shots_meet_only_shown_scripted_objects() {
        let order = esm::LoadOrder::from_plugins(Vec::new()).unwrap();
        let mut state = world::scripting::GameState::default();
        let bottle = world::scripting::Interactive {
            reference: FormId(0x0010A209),
            base: FormId(0x0010A1F6),
            script: Some(FormId(0x0010A1EF)),
            count: 1,
            position: [0.0; 3],
            rotation: [0.0; 3],
            scale: 1.0,
            trigger: None,
            bounds: None,
            name: None,
            kind: esm::FourCC::new(b"MISC"),
        };
        assert!(meetable(&order, &state, &bottle));
        state.disabled.insert(bottle.reference, true);
        assert!(!meetable(&order, &state, &bottle));
        state.disabled.insert(bottle.reference, false);
        assert!(meetable(&order, &state, &bottle));
        let unscripted = world::scripting::Interactive {
            script: None,
            ..bottle.clone()
        };
        assert!(!meetable(&order, &state, &unscripted));
    }

    #[test]
    fn rays_meet_a_body_of_its_own_size() {
        // A gecko-sized body (half width 35, height 100) 100 units ahead.
        let feet = [100.0, 0.0, 0.0];
        let d = ray_body([0.0, 0.0, 50.0], [1.0, 0.0, 0.0], feet, 35.0, 100.0).unwrap();
        assert!((d - 65.0).abs() < 1e-3);
        // Over its back.
        assert!(ray_body([0.0, 0.0, 120.0], [1.0, 0.0, 0.0], feet, 35.0, 100.0).is_none());
    }

    /// Someone standing at the origin facing north: `Bip01` (60 up) under
    /// the root, its spine (90), neck (100) and head (110) above, and a
    /// camera node (108) straight under the root; one body, a ball of
    /// radius 8 on the head; body part data with a head from the neck and
    /// a torso from `Bip01`.
    fn person(with_body: bool) -> (Walker, ActorRig, BodyPartData) {
        let at = |z: f32| nif::Transform {
            translation: [0.0, 0.0, z],
            ..nif::Transform::IDENTITY
        };
        let bone = |name: &str, parent, z| nif::Bone {
            name: name.into(),
            parent,
            local: at(z),
        };
        let bones = vec![
            bone("Scene Root", None, 0.0),
            bone("Bip01", Some(0), 60.0),
            bone("Bip01 Spine2", Some(1), 30.0),
            bone("Bip01 Neck1", Some(2), 10.0),
            bone("Bip01 Head", Some(3), 10.0),
            bone("Camera3rd", Some(0), 108.0),
        ];
        let ragdoll = with_body.then(|| {
            let head = nif::RagdollBody {
                bone: 4,
                bone_name: "Bip01 Head".into(),
                frame: at(110.0),
                mass: 3.0,
                center: [0.0; 3],
                inertia: [1.0; 3],
                linear_damping: 0.1,
                angular_damping: 0.05,
                friction: 0.3,
                restitution: 0.8,
                max_linear_speed: 7000.0,
                max_angular_speed: 30.0,
                capsule: Some(([0.0; 3], [0.0; 3], 8.0)),
                layer: 8,
                part: 1,
            };
            preview::ragdoll::RagdollRig::new(
                &bones,
                nif::Ragdoll {
                    bodies: vec![head],
                    joints: Vec::new(),
                },
            )
        });
        let skeleton = Arc::new(preview::cell::ActorSkeleton {
            bones,
            ragdoll,
            ..Default::default()
        });
        let actor = cellview::ActorData {
            skeleton: skeleton.clone(),
            transform: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
            reference: 0x1234,
            base: 0x1235,
            position: [0.0; 3],
            female: false,
            look: None,
        };
        let rig = ActorRig::new(skeleton, 1.0, 0.0);
        let part = |name: &str, node: &str, kind: u8| {
            Some(world::body_parts::BodyPart {
                name: name.into(),
                node: node.into(),
                target: node.into(),
                ik_start: None,
                tracking_max_angle: 0.0,
                damage_mult: 1.0,
                flags: 0,
                part_type: kind,
                health_percent: 50,
                actor_value: 25,
                to_hit_chance: 30,
                explode_chance: 0,
                limb_model: None,
            })
        };
        let mut parts = vec![None; 15];
        parts[0] = part("Torso", "Bip01", 0);
        parts[1] = part("Head", "Bip01 Neck1", 1);
        let data = BodyPartData {
            form_id: FormId(0x1D),
            editor_id: None,
            model: None,
            parts,
            ragdoll: None,
        };
        (Walker::new(&actor), rig, data)
    }

    #[test]
    fn a_body_its_critical_stage_culled_offers_the_crosshair_nothing() {
        use bevy::ecs::system::RunSystemOnce;
        // 008a1a70: stages 2 and 4 take the collision out (0057b520(0)).
        let (walker, mut rig, _) = person(true);
        let who = walker.reference;
        assert!(rig.go_limp(0.0, walker.placement(), None, None));
        let mut app = World::new();
        app.spawn((walker, rig));
        let shapes = |stage: Option<i32>| {
            let mut state = world::scripting::GameState::default();
            state.dead.insert(who);
            if let Some(s) = stage {
                state.more.critical_stage.insert(who, s);
            }
            move |rigs: Query<(&Walker, &ActorRig)>| {
                let talkers = Talkers(vec![crate::dialogue::Talker {
                    reference: who,
                    base: FormId(0x1235),
                    position: [0.0; 3],
                }]);
                crate::crosshair::people_shapes(&talkers, &state, &rigs).len()
            }
        };
        // Dead, whole: the ragdoll's one body.
        assert_eq!(app.run_system_once(shapes(None)).unwrap(), 1);
        // Disintegrated or gooed: nothing.
        let end = world::more_functions::DISINTEGRATE_END;
        assert_eq!(app.run_system_once(shapes(Some(end))).unwrap(), 0);
        let goo = world::more_functions::GOO_END;
        assert_eq!(app.run_system_once(shapes(Some(goo))).unwrap(), 0);
    }

    #[test]
    fn shots_must_meet_a_body_and_land_on_its_part() {
        let (walker, actor_rig, data) = person(true);
        let bones = &actor_rig.skeleton.bones;
        let rig = Some((&walker, &actor_rig));
        let east = [1.0, 0.0, 0.0];
        // Through the head's ball: 8 short of its centre, on the head's bone.
        let cylinder = ray_body([-100.0, 0.0, 110.0], east, [0.0; 3], 25.0, 130.0);
        let met = ray_actor(
            rig,
            Some(&data),
            ([-100.0, 0.0, 110.0], east),
            cylinder,
            false,
            0.0,
        );
        let (d, bone) = met.unwrap();
        assert!((d - 92.0).abs() < 1e-3, "{d}");
        assert_eq!(data.part_of_bone(bones, bone.unwrap()), Some(1));
        // Through the bounds but no body: a shot misses ...
        let chest = ([-100.0, 0.0, 80.0], east);
        let cylinder = ray_body(chest.0, east, [0.0; 3], 25.0, 130.0);
        assert!(cylinder.is_some());
        assert!(ray_actor(rig, Some(&data), chest, cylinder, false, 0.0).is_none());
        // ... a blow lands on the nearest bone with a part: the spine (10
        // off), the torso's.
        let (_, bone) = ray_actor(rig, Some(&data), chest, cylinder, true, 0.0).unwrap();
        assert_eq!(bone, Some(2));
        assert_eq!(data.part_of_bone(bones, 2), Some(0));
    }

    #[test]
    fn without_bodies_the_bounds_and_the_nearest_bone_decide() {
        let (walker, rig, data) = person(false);
        let rig = Some((&walker, &rig));
        let east = [1.0, 0.0, 0.0];
        let eye = [-100.0, 0.0, 108.0];
        let cylinder = ray_body(eye, east, [0.0; 3], 25.0, 130.0);
        let (d, bone) = ray_actor(rig, Some(&data), (eye, east), cylinder, false, 0.0).unwrap();
        assert!((d - 75.0).abs() < 1e-3);
        // The camera node (108) is nearest but gives no part: the head's
        // bone (110).
        assert_eq!(bone, Some(4));
    }
}

/// Weapons placed objects fired (`FireWeapon`: shooter, weapon), for
/// [`object_shots`].
#[derive(Resource, Default)]
pub struct ObjectShots(pub Vec<(FormId, FormId)>);

/// Carries out `FireWeapon` (`00523150` for something that isn't an
/// actor): the shot leaves the object's projectile node (else its
/// position) along its facing (`world::more_functions::traps::shot_from`),
/// each pellet within the weapon's cone as the player's do, to the
/// projectile's range; the first person met (the player or someone about,
/// by their bounds) before a wall takes the hit. Not yet: projectiles in
/// flight, hit shapes (bounds only), the weapon's own node name, objects
/// made by `PlaceAtMe`.
#[allow(clippy::too_many_arguments)]
pub fn object_shots(
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
    mut state: ResMut<DialogueState>,
    mut shots: ResMut<ObjectShots>,
    collision: Res<CellCollision>,
    rigs: Query<(&Walker, &ActorRig)>,
    mut nodes: Local<HashMap<String, Option<nif::math::Transform>>>,
) {
    use world::more_functions::traps;
    if shots.0.is_empty() {
        return;
    }
    let order = &game.0.order;
    let state = &mut state.0;
    for (from, weapon) in std::mem::take(&mut shots.0) {
        let (Some(w), Some(p)) = (
            Weapon::load(order, weapon),
            world::placement_of(order, from),
        ) else {
            continue;
        };
        let node = p.model.as_ref().and_then(|m| {
            *nodes.entry(m.to_ascii_lowercase()).or_insert_with(|| {
                let bytes = game.0.assets.read(&format!("meshes\\{m}")).ok().flatten()?;
                let nif = nif::Nif::parse(bytes).ok()?;
                nif.placed_node(traps::PROJECTILE_NODE)
                    .or_else(|| nif.placed_node(traps::PROJECTILE_NODE_ALT))
            })
        });
        let (origin, aim) = traps::shot_from(p.position, p.rotation, p.scale, node);
        let (count, cone) = w.shot(order, None);
        let reach = w.range(order).unwrap_or(SHOT_RANGE);
        let pellet = {
            let mut w = w.clone();
            w.damage /= count.max(1) as f32;
            w
        };
        // Its firing sound, from the object (`weapon_fx`; `0083ac30` with
        // no fire node: the object's place).
        crate::weapon_fx::fired(crate::weapon_fx::Fired {
            shooter: from,
            weapon: w.form_id,
            from: Some(origin),
        });
        let heading = aim[0].atan2(aim[1]);
        let pitch = aim[2].clamp(-1.0, 1.0).asin();
        // Who can be met: the player and the people about, by their bounds.
        let mut bodies: Vec<(FormId, [f32; 3], f32, f32)> = Vec::new();
        if let Some(feet) = state.player_position {
            let shape = physics::CharacterShape::PLAYER;
            bodies.push((PLAYER_REF, feet, shape.radius, shape.height));
        }
        for (walker, _) in &rigs {
            if state.dead.contains(&walker.reference) {
                continue;
            }
            let base =
                world::scripting::base_of(order, walker.reference).unwrap_or(walker.reference);
            let (half, height) = body(order, base);
            bodies.push((
                walker.reference,
                walker.position,
                half * walker.scale,
                height * walker.scale,
            ));
        }
        for _ in 0..count {
            let unit = |v: u64| (v % 1_000_000) as f32 / 1_000_000.0;
            let r = cone * unit(state.roll());
            let theta = std::f32::consts::TAU * unit(state.roll());
            let (h, p) = (heading + r * theta.cos(), pitch + r * theta.sin());
            let dir = [h.sin() * p.cos(), h.cos() * p.cos(), p.sin()];
            let wall = collision
                .0
                .raycast(origin, dir, reach)
                .map_or(reach, |(d, _)| d);
            let met = bodies
                .iter()
                .filter_map(|(who, feet, radius, height)| {
                    ray_body(origin, dir, *feet, *radius, *height).map(|d| (d, *who))
                })
                .filter(|(d, _)| *d <= wall)
                .min_by(|a, b| a.0.total_cmp(&b.0));
            let Some((d, target)) = met else {
                continue;
            };
            let hit =
                Runner::new(order, &scripts.0, state).hit_at(from, target, Some(&pellet), None);
            // An object isn't someone to fight back against.
            if state.combat.get(&target) == Some(&from) {
                state.combat.remove(&target);
            }
            if let Some(hit) = hit {
                println!("{from} shot {target} at {d:.0} units for {:.1}.", hit.dealt);
            }
        }
    }
}

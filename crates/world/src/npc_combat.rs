//! People fighting with what they carry, as the game's combat controller
//! does it (`FalloutNV.exe` 1.4.0.525; evidence, addresses and gaps in
//! `docs/NPC_COMBAT.md`):
//!
//! - **Which weapon** (`CombatState::UpdateWeapons` from the Xbox
//!   prototype's source path, `009993c0`): every weapon they carry that
//!   isn't "player only", hasn't been dropped and has ammunition with it (or
//!   needs none) is rated by its damage per second
//!   ([`damage_per_second`], `00645380` through `00646060`); a gun whose
//!   absolute maximum range doesn't reach the target is set aside (its
//!   score ÷ 10000) unless no gun reaches; the best of each kind
//!   ([`combat_weapon_type`], `00522c80`) and fists make the arsenal. The
//!   planner's attack actions cost their base (`011a4280`: ranged, explosive
//!   and grenade 1.5, melee and hand to hand 2.0) plus how far the kind's
//!   score falls short of the best (`0097ac30`); the cheapest is used. The
//!   arsenal is made again when a fight starts (`00997c30`) and every
//!   `fCombatInventoryUpdateTimer` (5 s).
//! - **Ammunition** (`Actor::ShouldUseAmmo` (Xbox PDB), `008a8dd0`): only
//!   the player, weapons flagged "NPCs use ammo" (`DNAM` flags2 0x2, the
//!   dynamite's) and a teammate's playable weapons use ammunition up; others
//!   need one round to fire but never run out. Clips empty and are reloaded
//!   (`008a89a0`, `008a8420`): full when they don't use ammunition, else
//!   what they carry up to a clip.
//! - **The fight's script event** (`00980830`, `009887b0`): an actor whose
//!   target goes from none to someone runs its `OnStartCombat` blocks
//!   (event 0x8000, `005ca860`) naming the target once the target is
//!   detected (value above 0).
//!
//! Inferred (labelled where used): that the cheapest single action decides
//! the weapon (the planner's search over its world state isn't traced),
//! the order ties go in, a clip's first load, and switching at once (the
//! switch procedure `009da7c0` waits for the equip animation).

use std::collections::{HashMap, HashSet};

use esm::{FormId, LoadOrder};

use crate::combat::{self, Weapon};
use crate::combat_ai::{CombatStyle, Setting};
use crate::dialogue::PLAYER_REF;
use crate::explosions::{ExplosionRecord, ProjectileRecord};
use crate::scripting::{GameState, Runner};

/// The combat weapon kinds (`COMBAT_WEAPON_TYPE` (Xbox PDB)), the index of
/// each kind's slot in the arsenal (`009993c0`: `+0x3c + kind × 4`).
pub mod kind {
    pub const RANGED_EXPLOSIVE: usize = 0;
    pub const RANGED: usize = 1;
    pub const MELEE: usize = 2;
    pub const GRENADE: usize = 3;
    pub const MINE: usize = 4;
    /// Fists: the seventh slot (`+0x54`), filled with no weapon.
    pub const HAND_TO_HAND: usize = 6;
    pub const COUNT: usize = 7;
}

/// The attack actions that use a weapon kind, in the planner's action
/// order (`00979f90`: 0 ranged explosive, 2 ranged, 4 grenade, 7 melee, 8
/// hand to hand), with their base costs (the table at `011a4280`).
pub const ATTACK_COSTS: [(usize, f32); 5] = [
    (kind::RANGED_EXPLOSIVE, 1.5),
    (kind::RANGED, 1.5),
    (kind::GRENADE, 1.5),
    (kind::MELEE, 2.0),
    (kind::HAND_TO_HAND, 2.0),
];

/// A gun in the code's sense (`004c0c30`): animation types 3 to 13, so
/// grenades, mines and throws too.
pub fn is_gun(w: &Weapon) -> bool {
    (3..=13).contains(&w.animation)
}

/// The explosion a weapon's projectile makes, if any (the projectile
/// `00525a90` gives: the weapon's own, `DNAM` 36; an ammunition's own
/// projectile isn't looked at here).
pub(crate) fn explosion_of(order: &LoadOrder, w: &Weapon) -> Option<ExplosionRecord> {
    let p = ProjectileRecord::load(order, w.projectile?)?;
    ExplosionRecord::load(order, p.explosion?)
}

/// A weapon's combat kind (`TESObjectWEAP::GetCombatWeaponType` (Xbox
/// PDB)): melee for hand-to-hand and melee animations; ranged for guns and
/// throws, "ranged explosive" when the projectile's explosion is
/// dangerous (damage above `fDangerousProjectileExplosionDamage` 5 and
/// radius above `fDangerousProjectileExplosionRadius` 30 units); grenade;
/// mine (land and dropped mines). Anything else is logged and treated as
/// melee.
// Translated from 00522c80 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn combat_weapon_type(order: &LoadOrder, w: &Weapon, s: Setting) -> usize {
    match w.animation {
        0..=2 => kind::MELEE,
        3..=9 | 13 => {
            let dangerous = explosion_of(order, w).is_some_and(|e| {
                e.damage > s("fDangerousProjectileExplosionDamage", 5.0)
                    && e.radius_units(s("fBSUnitsPerFoot", 22.0))
                        > s("fDangerousProjectileExplosionRadius", 30.0)
            });
            if dangerous {
                kind::RANGED_EXPLOSIVE
            } else {
                kind::RANGED
            }
        }
        10 => kind::GRENADE,
        11 | 12 => kind::MINE,
        _ => kind::MELEE,
    }
}

/// Whether `who` uses ammunition up firing `w` (`Actor::ShouldUseAmmo`
/// (Xbox PDB)): the player always; anyone with a weapon flagged "NPCs use
/// ammo" (`DNAM` flags2 0x2, `008a8e30`); a teammate with a playable weapon
/// (flags1 0x80 clear, `0047bcf0`).
// Translated from 008a8dd0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn should_use_ammo(state: &GameState, who: FormId, w: &Weapon) -> bool {
    if who == PLAYER_REF {
        return true;
    }
    if w.flags2 & 0x2 != 0 {
        return true;
    }
    w.flags1 & 0x80 == 0 && state.teammates.contains(&who)
}

/// A worn weapon's damage multiplier (`00646d00`): 1 above 75% condition,
/// else 1 − (0.75 − condition) × 0.67.
// Translated from 00646d00 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn condition_mult(condition: f32) -> f32 {
    if condition <= 0.75 {
        1.0 - (0.75 - condition) * 0.67
    } else {
        1.0
    }
}

/// The jam chance index for a condition (`004bd510(condition × 10, 1)`:
/// rounded half up).
// Translated from 004bd510 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn jam_index(condition: f32) -> i32 {
    let x = condition * 10.0;
    let whole = x.trunc();
    whole as i32 + i32::from(x - whole >= 0.5)
}

/// What [`dps`] needs, looked up: everything is the weapon's own record
/// except the actor values.
#[derive(Debug, Clone, PartialEq)]
pub struct DpsInputs {
    /// `None`: fists.
    pub weapon: Option<DpsWeapon>,
    /// 0 to 1.
    pub condition: f32,
    /// The player (for whom semi-automatic delays and bursts don't count).
    pub player: bool,
    /// A creature's own attack damage, when fighting without a weapon.
    pub creature_damage: Option<f32>,
    /// Unarmed Damage (actor value 56) and Unarmed (45).
    pub unarmed_damage: f32,
    pub unarmed_skill: f32,
}

/// A weapon's figures for [`dps`].
#[derive(Debug, Clone, PartialEq)]
pub struct DpsWeapon {
    pub animation: u32,
    /// Base damage after `fDamageWeaponMult` and the ammunition's damage
    /// effects, plus any explosion as the code adds it (see
    /// [`damage_per_second`]).
    pub damage: f32,
    /// For grenades and mines: their explosion's damage (×
    /// `fDamageWeaponMult`), which replaces the rest.
    pub explosive_damage: Option<f32>,
    /// The holder's skill with it, 0–100.
    pub skill: f32,
    pub clip: u32,
    /// `DNAM` 88, 92, 96: attack shots a second, reload and jam seconds.
    pub shots_per_second: f32,
    pub reload_time: f32,
    pub jam_time: f32,
    pub flags1: u8,
    pub flags2: u32,
    pub semi_auto_delay: (f32, f32),
    /// The holder's Critical Chance (actor value 14) × the weapon's
    /// critical multiplier (`CRDT`), and its critical damage.
    pub crit_chance: f32,
    pub crit_damage: f32,
}

/// Damage a second (`CombatFormulas::CalcWeaponDamagePerSecond` (Xbox
/// PDB)), as the combat controller asks for it (no mods, no perks, the
/// holder's own skill and critical chance): a weapon's damage × (skill
/// base + skill mult × skill ÷ 100) × the condition's multiplier, plus the
/// critical share, × a clip's rounds, over the time a clip takes (shots ÷
/// (attack shots a second + rounds × jam chance × jam seconds), with the
/// reload for clips of `iMinClipSizeToAddReloadDelay` (2) or fewer, the
/// semi-automatic delay beyond the attack for others than the player, and
/// automatic weapons' bursts and cooldowns); fists `fUnarmedDamageMult` ×
/// Unarmed Damage + `fDamageSkillMult` × Unarmed ÷ 100 over
/// `fUnarmedNPCDPSMult` (`fUnarmedCreatureDPSMult` for creatures, with
/// their own damage). 0 for a broken weapon (condition 0).
// Translated from 00645380 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn dps(i: &DpsInputs, s: Setting) -> f32 {
    dps_with(i, &|d| d, s)
}

/// [`dps`] with the holder's perks on a weapon's damage once the skill
/// has scaled it (`00645380`'s fourth argument: "Calculate Weapon Damage",
/// entry point 0, `005e58f0`; see [`crate::dps`]).
pub fn dps_with(i: &DpsInputs, perks: &dyn Fn(f32) -> f32, s: Setting) -> f32 {
    if i.condition <= 0.0 {
        return 0.0;
    }
    let mut shots = 1i32;
    let (mut damage, mut time, mut crit, crit_damage);
    match &i.weapon {
        Some(w) => {
            damage = w.damage;
            let mut jam_at = jam_index(i.condition);
            match w.animation {
                0..=2 | 13 => {}
                10 | 11 => {
                    if let Some(e) = w.explosive_damage {
                        damage = e;
                    }
                    jam_at = 10;
                }
                _ => shots = w.clip as i32,
            }
            damage *= s("fDamageSkillBase", 0.5) + s("fDamageSkillMult", 0.5) * w.skill / 100.0;
            damage = perks(damage);
            time = w.shots_per_second;
            if !matches!(w.animation, 10 | 11 | 13) {
                time += shots as f32 * jam_chance(jam_at, s) * w.jam_time;
            }
            time = shots as f32 / time;
            let small_clip = shots <= s("iMinClipSizeToAddReloadDelay", 2.0) as i32;
            if small_clip && !matches!(w.animation, 0..=2 | 10 | 11 | 13) {
                time += w.reload_time;
            }
            if !i.player && (3..=13).contains(&w.animation) {
                let automatic = w.flags1 & 0x02 != 0;
                if !automatic {
                    let average = (w.semi_auto_delay.1 + w.semi_auto_delay.0) / 2.0;
                    let attack = 1.0 / w.shots_per_second;
                    if attack < average {
                        time += shots as f32 * (average - attack);
                    }
                }
                if automatic || w.flags2 & 0x200 != 0 {
                    let burst = s("fAutomaticWeaponBurstFireTime", 1.0);
                    let cooldown = s("fAutomaticWeaponBurstCooldownTime", 1.0);
                    time *= (burst + cooldown) / cooldown;
                }
            }
            crit = w.crit_chance.clamp(0.0, 100.0) * 0.01;
            if w.flags1 & 0x02 != 0 && w.shots_per_second != 0.0 {
                crit /= w.shots_per_second;
            }
            crit_damage = w.crit_damage;
        }
        None => {
            damage = match i.creature_damage {
                Some(d) => d,
                None => {
                    s("fUnarmedDamageMult", 0.5) * i.unarmed_damage
                        + s("fDamageSkillMult", 0.5) * i.unarmed_skill / 100.0
                }
            };
            time = if i.creature_damage.is_some() {
                s("fUnarmedCreatureDPSMult", 1.0)
            } else {
                s("fUnarmedNPCDPSMult", 1.57)
            };
            crit = 0.0;
            crit_damage = 0.0;
        }
    }
    damage *= condition_mult(i.condition);
    let total = (crit * crit_damage + damage) * shots as f32;
    if time != 0.0 {
        total / time
    } else {
        0.0
    }
}

/// `fWeaponConditionJam1`…`10` by index (`006477b0`, the setting table at
/// `0119b26c`); 0 outside 0–9.
fn jam_chance(index: i32, s: Setting) -> f32 {
    const DEFAULTS: [f32; 10] = [0.4, 0.2, 0.1, 0.05, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    match usize::try_from(index) {
        Ok(i) if i < 10 => s(&format!("fWeaponConditionJam{}", i + 1), DEFAULTS[i]),
        _ => 0.0,
    }
}

/// An ammunition's damage effects (`0059a030`, kind 0): the multiplying
/// ones first, then the adding and subtracting ones in turn.
// Translated from 0059a030 (decompiled, FalloutNV.exe 1.4.0.525)
pub(crate) fn ammo_damage(order: &LoadOrder, ammo: FormId, damage: f32) -> f32 {
    let effects = combat::ammo_effects(order, ammo);
    let mut d = damage;
    for &(k, op, x) in &effects {
        if k == 0 && op == 1 {
            d *= x;
        }
    }
    for &(k, op, x) in &effects {
        match (k, op) {
            (0, 0) => d += x,
            (0, 2) => d -= x,
            _ => {}
        }
    }
    d
}

/// [`dps`] for `who` with `weapon` (`None`: fists) at `condition`, as the
/// combat controller asks (`00646060` from `009993c0`: no perks, no
/// inventory entry, the weapon's own ammunition): [`crate::dps::weapon_dps`].
pub fn damage_per_second(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    weapon: Option<&Weapon>,
    condition: f32,
    s: Setting,
) -> f32 {
    crate::dps::weapon_dps(
        order,
        state,
        &crate::dps::DpsCall::controller(who, weapon, condition),
        None,
        s,
    )
}

/// A float in a weapon's `DNAM`.
pub(crate) fn dnam_f32(order: &LoadOrder, weapon: FormId, at: usize) -> Option<f32> {
    let record = order.get(weapon)?.record().ok()?;
    let d = record.get(esm::FourCC::new(b"DNAM"))?;
    (d.data.len() >= at + 4).then(|| crate::cell::le_f32(&d.data, at))
}

/// The damage a second the combat controller rates a weapon at
/// (`CombatFormulas::CalcCombatWeaponDPS` (Xbox PDB)): 0 for one "not used
/// in normal combat" (flags2 0x40, `00646230`); [`damage_per_second`] ÷ 10
/// for grenades, 0 for mines; ÷ 10000 for a gun when the style allows melee
/// only (weapon restrictions 1) or for melee and fists when it allows
/// ranged only (2).
// Translated from 00646060 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn combat_dps(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    weapon: Option<&Weapon>,
    condition: f32,
    style: &CombatStyle,
    s: Setting,
) -> f32 {
    crate::dps::combat_weapon_dps(
        order,
        state,
        &crate::dps::DpsCall::controller(who, weapon, condition),
        style,
        None,
        s,
    )
}

/// The style's weapon restriction on a rating (`00646060`): `gun` `None`
/// for fists.
pub(crate) fn restricted(d: f32, gun: Option<bool>, restriction: u32) -> f32 {
    match restriction {
        1 if gun == Some(true) => d / 10000.0,
        2 if gun != Some(true) => d / 10000.0,
        _ => d,
    }
}

/// One weapon the arsenal looked at.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub weapon: FormId,
    pub kind: usize,
    pub dps: f32,
    /// For guns with a target: its absolute maximum range (before the
    /// current weapon's allowance), and whether it reaches the target.
    pub max_range: f32,
    pub in_range: bool,
}

/// What someone can fight with (`009993c0`): the best weapon and its score
/// for each kind (fists in [`kind::HAND_TO_HAND`]), the best damage a
/// second, the best gun's, the best score, and the range at which a better
/// gun out of range would come into reach.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Arsenal {
    pub weapons: [Option<FormId>; kind::COUNT],
    pub scores: [f32; kind::COUNT],
    pub best_dps: f32,
    pub best_ranged_dps: f32,
    pub best_score: f32,
    pub recheck_range: f32,
}

/// The arsenal from rated candidates (in inventory order) and the fists'
/// rating. Guns are already marked in or out of range; out of range ones
/// are set aside (score ÷ 10000) unless none reaches, when the best of them
/// stays.
// Translated from 009993c0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn assemble(candidates: &[Candidate], fists: f32) -> Arsenal {
    let ranged = |k: usize| k == kind::RANGED || k == kind::RANGED_EXPLOSIVE;
    let mut best_in = 0.0f32;
    let mut best_out: Option<(f32, FormId)> = None;
    for c in candidates.iter().filter(|c| ranged(c.kind)) {
        if c.in_range {
            best_in = best_in.max(c.dps);
        } else if best_out.map_or(true, |(d, _)| d < c.dps) {
            best_out = Some((c.dps, c.weapon));
        }
    }
    if best_in > 0.0 {
        best_out = None;
    }
    let mut a = Arsenal::default();
    for c in candidates {
        let mut score = c.dps;
        let usable = !ranged(c.kind) || c.in_range || best_out.is_some_and(|(_, w)| w == c.weapon);
        if !usable {
            score /= 10000.0;
            if a.recheck_range < c.max_range && best_in < c.dps {
                a.recheck_range = c.max_range;
            }
        }
        a.best_dps = a.best_dps.max(c.dps);
        if ranged(c.kind) {
            a.best_ranged_dps = a.best_ranged_dps.max(c.dps);
        }
        a.best_score = a.best_score.max(score);
        if a.scores[c.kind] < score {
            a.weapons[c.kind] = Some(c.weapon);
            a.scores[c.kind] = score;
        }
    }
    a.scores[kind::HAND_TO_HAND] = fists;
    a.best_dps = a.best_dps.max(fists);
    a.best_score = a.best_score.max(fists);
    a
}

/// The weapon kind the planner attacks with: the cheapest attack action,
/// its base cost ([`ATTACK_COSTS`]) plus the best score less the kind's
/// (`0097ac30`; a kind rated 0 can't be used). That the cheapest single
/// action decides, and that ties go to the first in the planner's order,
/// is inferred: the planner's search over its world state isn't traced.
pub fn cheapest_kind(a: &Arsenal) -> Option<usize> {
    let mut best: Option<(f32, usize)> = None;
    for &(k, base) in &ATTACK_COSTS {
        let score = a.scores[k];
        if score == 0.0 {
            continue;
        }
        let cost = base + (a.best_score - score);
        if best.map_or(true, |(c, _)| cost < c) {
            best = Some((cost, k));
        }
    }
    best.map(|(_, k)| k)
}

/// The weapon someone has equipped now (`007031c0`).
fn equipped_weapon(order: &LoadOrder, state: &GameState, who: FormId) -> Option<FormId> {
    state.equipped.get(&who).and_then(|worn| {
        worn.iter().copied().find(|&i| {
            order
                .get(i)
                .is_some_and(|r| r.entry.header.kind.as_bytes() == b"WEAP")
        })
    })
}

/// Whether they carry ammunition for `w` (or it needs none): the kind
/// loaded, else any kind it takes (`00525980`, `009962f0`, `004c7300`).
fn has_ammo(order: &LoadOrder, state: &GameState, who: FormId, w: &Weapon) -> bool {
    w.ammo.is_empty() || w.ammo.iter().any(|&a| state.item_count(order, who, a) > 0)
}

/// Rates what `who` carries and assembles the arsenal (`009993c0`):
/// weapons not "player only" (flags2 0x1, `008bc9d0`), not dropped, with
/// ammunition; for guns the absolute maximum range of their band
/// (`009a9180`; × `fCombatCurrentWeaponAbsoluteMaxRangeMult` 1.5 for the
/// weapon in hand) against `target_distance`.
pub fn arsenal(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    style: &CombatStyle,
    target_distance: Option<f32>,
    s: Setting,
) -> Arsenal {
    let current = equipped_weapon(order, state, who);
    let mut candidates = Vec::new();
    for (id, n) in state.inventory(order, who) {
        if n <= 0 || state.dropped.contains(&(who, id)) {
            continue;
        }
        let Some(w) = Weapon::load(order, id) else {
            continue;
        };
        if w.flags2 & 0x1 != 0 || !has_ammo(order, state, who, &w) {
            continue;
        }
        let condition = combat::weapon_condition(state, who, id);
        let d = combat_dps(order, state, who, Some(&w), condition, style, s);
        if d <= 0.0 {
            continue;
        }
        let k = combat_weapon_type(order, &w, s);
        let (mut max_range, mut in_range) = (0.0, true);
        if let Some(distance) = target_distance.filter(|_| k <= kind::RANGED) {
            let reach = w
                .projectile
                .and_then(|p| combat::projectile_reach(order, p));
            let band = crate::combat_ai::ranged_band(Some((&w, reach)), style, s);
            max_range = band.absolute_max;
            let mut reaches = band.absolute_max;
            if current == Some(id) {
                reaches *= s("fCombatCurrentWeaponAbsoluteMaxRangeMult", 1.5);
            }
            in_range = distance * distance <= reaches * reaches;
        }
        candidates.push(Candidate {
            weapon: id,
            kind: k,
            dps: d,
            max_range,
            in_range,
        });
    }
    let fists = combat_dps(order, state, who, None, 1.0, style, s);
    assemble(&candidates, fists)
}

/// A clip being fired, or reloaded: the weapon, rounds left in it, and
/// when a reload under way ends (state seconds).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clip {
    pub weapon: FormId,
    pub rounds: u32,
    pub reloaded_at: f64,
}

/// What a fight's weapon handling keeps, by fighter. Not saved (the game
/// saves its combat controllers; their state isn't traced).
#[derive(Debug, Clone, Default)]
pub struct State {
    /// Who has had their fight's `OnStartCombat` (until the fight ends).
    pub announced: HashSet<FormId>,
    /// When each fighter's arsenal is next made (state seconds), and the
    /// last one.
    pub next_update: HashMap<FormId, f64>,
    pub arsenals: HashMap<FormId, Arsenal>,
    /// Fighters who chose their fists (weapon put away).
    pub unarmed: HashSet<FormId>,
    pub clips: HashMap<FormId, Clip>,
}

/// One fight frame for `who`'s weapon (people only: creatures' weapons are
/// their own and aren't chosen here): when due (the fight's start,
/// `fCombatInventoryUpdateTimer` (5 s) later, or at once after a weapon ran
/// dry), the arsenal is made and the cheapest kind's weapon equipped (fists:
/// the weapon put away). Switching is at once (the game's switch waits for
/// the equip animation). Whether the weapon changed.
pub fn choose_weapon(
    order: &LoadOrder,
    state: &mut GameState,
    who: FormId,
    style: &CombatStyle,
    target_distance: Option<f32>,
    s: Setting,
) -> bool {
    if who == PLAYER_REF || combat::is_creature(order, who) {
        return false;
    }
    let now = state.seconds;
    if state
        .npc_combat
        .next_update
        .get(&who)
        .is_some_and(|&t| now < t)
    {
        return false;
    }
    // What they carry includes what their leveled lists give
    // (`WithAmmoNV…Loot`: a gun and its rounds). The game has those in
    // the actor's inventory before any fight; the viewer picks them when
    // the holder's contents are first copied into the state
    // (`GameState::stock`), so that happens here at the latest. Without it
    // the arsenal saw only the record's direct items and the Powder
    // Gangers fought with their fists.
    state.stock(order, who);
    let a = arsenal(order, state, who, style, target_distance, s);
    state
        .npc_combat
        .next_update
        .insert(who, now + f64::from(s("fCombatInventoryUpdateTimer", 5.0)));
    let before = (
        equipped_weapon(order, state, who),
        state.npc_combat.unarmed.contains(&who),
    );
    match cheapest_kind(&a) {
        Some(kind::HAND_TO_HAND) => {
            if let Some(w) = before.0 {
                state.unequip(who, w);
            }
            state.npc_combat.unarmed.insert(who);
        }
        Some(k) => {
            if let Some(w) = a.weapons[k] {
                if before.0 != Some(w) {
                    state.equip(order, who, w);
                }
                state.npc_combat.unarmed.remove(&who);
            }
        }
        None => {}
    }
    state.npc_combat.arsenals.insert(who, a);
    let after = (
        equipped_weapon(order, state, who),
        state.npc_combat.unarmed.contains(&who),
    );
    after != before
}

/// Whether `who` is reloading now.
pub fn reloading(state: &GameState, who: FormId) -> bool {
    state
        .npc_combat
        .clips
        .get(&who)
        .is_some_and(|c| state.seconds < c.reloaded_at)
}

/// What a reload loads (`008a8420`): a full clip unless they use
/// ammunition up and carry less than one, when what they carry.
// Translated from 008a8420 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn reload_rounds(uses_ammo: bool, clip: u32, carried: u32) -> u32 {
    if !uses_ammo || clip < carried {
        clip
    } else {
        carried
    }
}

/// What a shot did to the shooter's ammunition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AfterShot {
    /// Rounds left in the clip (or nothing to count).
    Ready,
    /// The clip ran out and a reload began, lasting this many seconds.
    Reloading(f32),
    /// Nothing left to fire it with: a weapon is chosen again at once.
    Dry,
}

/// `who` fired `w` once (`Actor::UseAmmo` (Xbox PDB), `008a89a0`): the
/// clip loses the weapon's ammo use (`DNAM` 14) and, when they use
/// ammunition up ([`should_use_ammo`]), so does their inventory; a thrown
/// weapon is its own ammunition. An emptied clip is reloaded
/// ([`reload_rounds`]) over the weapon's reload time (`DNAM` 92) at their
/// reload rate (`world::combat::reload_rate`; the animation's own length
/// isn't read), or, with nothing left, the weapon is put away and another
/// chosen. A clip's first load, before any shot, is taken to be full (what
/// the game loads isn't traced).
pub fn fired(order: &LoadOrder, state: &mut GameState, who: FormId, w: &Weapon) -> AfterShot {
    let uses = should_use_ammo(state, who, w);
    if crate::explosions::is_thrown(w) {
        if uses {
            take(order, state, who, w.form_id, 1);
            if state.item_count(order, who, w.form_id) <= 0 {
                state.unequip(who, w.form_id);
                state.npc_combat.next_update.remove(&who);
                return AfterShot::Dry;
            }
        }
        return AfterShot::Ready;
    }
    let Some(ammo) = w
        .ammo_in_use(order, state, who)
        .filter(|_| !w.ammo.is_empty())
    else {
        return AfterShot::Ready;
    };
    let size = w.clip.max(1);
    let carried = |state: &GameState| state.item_count(order, who, ammo).max(0) as u32;
    let rounds = match state.npc_combat.clips.get(&who) {
        Some(c) if c.weapon == w.form_id => c.rounds,
        _ => reload_rounds(uses, size, carried(state)),
    };
    let used = u32::from(w.ammo_use.max(1)).min(rounds);
    if uses {
        take(order, state, who, ammo, used as i32);
    }
    let left = rounds - used;
    let now = state.seconds;
    if left > 0 {
        state.npc_combat.clips.insert(
            who,
            Clip {
                weapon: w.form_id,
                rounds: left,
                reloaded_at: now,
            },
        );
        return AfterShot::Ready;
    }
    let have = carried(state);
    if have == 0 {
        state.npc_combat.clips.remove(&who);
        state.npc_combat.next_update.remove(&who);
        return AfterShot::Dry;
    }
    let rate = combat::reload_rate(order, state, who, Some(w)).max(1e-3);
    let seconds = w.reload_time / rate;
    state.npc_combat.clips.insert(
        who,
        Clip {
            weapon: w.form_id,
            rounds: reload_rounds(uses, size, have),
            reloaded_at: now + f64::from(seconds),
        },
    );
    AfterShot::Reloading(seconds)
}

/// Takes `n` of an item from a holder's inventory.
fn take(order: &LoadOrder, state: &mut GameState, who: FormId, item: FormId, n: i32) {
    state.stock(order, who);
    let have = state.items.get(&(who, item)).copied().unwrap_or(0);
    let left = (have - n).max(0);
    if left == 0 {
        state.items.remove(&(who, item));
    } else {
        state.items.insert((who, item), left);
    }
}

/// `who`'s fight has a target that is now detected (`detected`: their
/// detection value of it above 0): the first time in this fight, their
/// script's `OnStartCombat` blocks for anyone or naming the target run
/// (`00980830` as the target is set, `009887b0` when it's first detected;
/// event 0x8000, `005ca860`). Not for the player.
pub fn start_combat_event(runner: &mut Runner, who: FormId, detected: bool) {
    if who == PLAYER_REF || !detected || runner.state.npc_combat.announced.contains(&who) {
        return;
    }
    let Some(&target) = runner.state.combat.get(&who) else {
        return;
    };
    runner.state.npc_combat.announced.insert(who);
    runner.run_event(who, "onstartcombat", target);
}

/// `who` is out of combat: the next fight starts afresh (its event, its
/// arsenal).
pub fn combat_over(state: &mut GameState, who: FormId) {
    let n = &mut state.npc_combat;
    n.announced.remove(&who);
    n.next_update.remove(&who);
    n.arsenals.remove(&who);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings(name: &str, default: f32) -> f32 {
        let _ = name;
        default
    }

    fn rifle() -> DpsWeapon {
        // The varmint rifle's record: 18 damage, 5 rounds, 1.2158 attacks
        // a second, reload 2.16, jam 1.467, semi-auto delay 0–0.3, critical
        // damage 18.
        DpsWeapon {
            animation: 5,
            damage: 18.0,
            explosive_damage: None,
            skill: 30.0,
            clip: 5,
            shots_per_second: 1.2158,
            reload_time: 2.16,
            jam_time: 1.467,
            flags1: 0x04,
            flags2: 0x3008,
            semi_auto_delay: (0.0, 0.3),
            crit_chance: 5.0,
            crit_damage: 18.0,
        }
    }

    fn inputs(weapon: Option<DpsWeapon>) -> DpsInputs {
        DpsInputs {
            weapon,
            condition: 1.0,
            player: false,
            creature_damage: None,
            unarmed_damage: 0.75,
            unarmed_skill: 5.0,
        }
    }

    #[test]
    fn a_rifle_is_rated_over_a_clip() {
        // (0.05 × 18 + 18 × (0.5 + 0.5 × 0.3)) × 5 ÷ (5 ÷ 1.2158).
        let d = dps(&inputs(Some(rifle())), &settings);
        let expect = (0.05 * 18.0 + 18.0 * 0.65) * 5.0 / (5.0 / 1.2158);
        assert!((d - expect).abs() < 1e-3, "{d} {expect}");
        // Worn to half: the condition multiplier and a jam chance (index
        // 5: fWeaponConditionJam6, 0) — 1 − 0.25 × 0.67.
        let worn = dps(
            &DpsInputs {
                condition: 0.5,
                ..inputs(Some(rifle()))
            },
            &settings,
        );
        let expect = (0.05 * 18.0 + 18.0 * 0.65 * (1.0 - 0.25 * 0.67)) * 5.0 / (5.0 / 1.2158);
        assert!((worn - expect).abs() < 1e-3, "{worn} {expect}");
        // A broken weapon isn't rated.
        let broken = DpsInputs {
            condition: 0.0,
            ..inputs(Some(rifle()))
        };
        assert_eq!(dps(&broken, &settings), 0.0);
    }

    #[test]
    fn small_clips_add_the_reload_and_slow_triggers_their_delay() {
        // A one-round gun: the reload is added to the clip's time.
        let single = DpsWeapon { clip: 1, ..rifle() };
        let d = dps(&inputs(Some(single)), &settings);
        let expect = (0.05 * 18.0 + 18.0 * 0.65) / (1.0 / 1.2158 + 2.16);
        assert!((d - expect).abs() < 1e-3, "{d} {expect}");
        // A semi-automatic delay longer than the attack adds the
        // difference per round (not for the player).
        let slow = DpsWeapon {
            semi_auto_delay: (2.0, 2.0),
            ..rifle()
        };
        let npc = dps(&inputs(Some(slow.clone())), &settings);
        let time = 5.0 / 1.2158 + 5.0 * (2.0 - 1.0 / 1.2158);
        let expect = (0.05 * 18.0 + 18.0 * 0.65) * 5.0 / time;
        assert!((npc - expect).abs() < 1e-3, "{npc} {expect}");
        let player = dps(
            &DpsInputs {
                player: true,
                ..inputs(Some(slow))
            },
            &settings,
        );
        assert!(player > npc);
    }

    #[test]
    fn automatics_burst_and_grenades_explode() {
        let smg = DpsWeapon {
            animation: 6,
            flags1: 0x02,
            shots_per_second: 10.0,
            clip: 30,
            ..rifle()
        };
        let d = dps(&inputs(Some(smg)), &settings);
        // Bursts of 1 s and cooldowns of 1 s halve the rate; the critical
        // chance is spread over the shots a second.
        let crit = 0.05 / 10.0;
        let expect = (crit * 18.0 + 18.0 * 0.65) * 30.0 / (30.0 / 10.0 * 2.0);
        assert!((d - expect).abs() < 1e-3, "{d} {expect}");
        // Dynamite: its explosion's damage replaces the weapon's, one throw
        // at its attack rate, no jams or reloads.
        let dynamite = DpsWeapon {
            animation: 10,
            damage: 1.0,
            explosive_damage: Some(50.0),
            clip: 12,
            shots_per_second: 0.405,
            flags1: 0x40,
            flags2: 0x10A,
            semi_auto_delay: (0.0, 0.0),
            crit_chance: 5.0,
            crit_damage: 0.0,
            ..rifle()
        };
        let d = dps(&inputs(Some(dynamite)), &settings);
        let expect = 50.0 * 0.65 / (1.0 / 0.405);
        assert!((d - expect).abs() < 1e-3, "{d} {expect}");
    }

    #[test]
    fn fists_are_rated_by_unarmed_damage() {
        let d = dps(&inputs(None), &settings);
        assert!((d - (0.5 * 0.75 + 0.5 * 0.05) / 1.57).abs() < 1e-5, "{d}");
        let creature = DpsInputs {
            creature_damage: Some(5.0),
            ..inputs(None)
        };
        assert!((dps(&creature, &settings) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn conditions_round_to_their_jam_index() {
        assert_eq!(jam_index(1.0), 10);
        assert_eq!(jam_index(0.34), 3);
        assert_eq!(jam_index(0.35), 4);
        assert_eq!(condition_mult(0.9), 1.0);
        assert!((condition_mult(0.25) - (1.0 - 0.5 * 0.67)).abs() < 1e-6);
    }

    #[test]
    fn styles_restrict_guns_or_hands() {
        assert_eq!(restricted(10.0, Some(true), 0), 10.0);
        assert_eq!(restricted(10.0, Some(true), 1), 0.001);
        assert_eq!(restricted(10.0, Some(false), 1), 10.0);
        assert_eq!(restricted(10.0, None, 2), 0.001);
        assert_eq!(restricted(10.0, Some(true), 2), 10.0);
    }

    fn cand(id: u32, k: usize, dps: f32, in_range: bool) -> Candidate {
        Candidate {
            weapon: FormId(id),
            kind: k,
            dps,
            max_range: 1000.0,
            in_range,
        }
    }

    #[test]
    fn the_best_of_each_kind_and_the_cheapest_attack() {
        // A rifle (15), a cleaver (8), dynamite (1.3 after ÷ 10), fists.
        let a = assemble(
            &[
                cand(1, kind::RANGED, 15.0, true),
                cand(2, kind::MELEE, 8.0, true),
                cand(3, kind::GRENADE, 1.3, true),
            ],
            0.3,
        );
        assert_eq!(a.weapons[kind::RANGED], Some(FormId(1)));
        assert_eq!(a.best_score, 15.0);
        assert_eq!(cheapest_kind(&a), Some(kind::RANGED));
        // Melee wins only when it does more than the cost difference (0.5)
        // better.
        let close = assemble(
            &[
                cand(1, kind::RANGED, 15.0, true),
                cand(2, kind::MELEE, 15.4, true),
            ],
            0.3,
        );
        assert_eq!(cheapest_kind(&close), Some(kind::RANGED));
        let better = assemble(
            &[
                cand(1, kind::RANGED, 15.0, true),
                cand(2, kind::MELEE, 15.6, true),
            ],
            0.3,
        );
        assert_eq!(cheapest_kind(&better), Some(kind::MELEE));
        // Nothing at all: fists.
        assert_eq!(cheapest_kind(&assemble(&[], 0.3)), Some(kind::HAND_TO_HAND));
    }

    #[test]
    fn guns_out_of_range_are_set_aside_unless_none_reaches() {
        // The rifle reaches; the better pistol doesn't: it's set aside and
        // the range it would need is kept.
        let a = assemble(
            &[
                cand(1, kind::RANGED, 10.0, true),
                Candidate {
                    max_range: 700.0,
                    ..cand(2, kind::RANGED, 20.0, false)
                },
            ],
            0.3,
        );
        assert_eq!(a.weapons[kind::RANGED], Some(FormId(1)));
        assert_eq!(a.recheck_range, 700.0);
        // None reaches: the best of them stays usable.
        let a = assemble(
            &[
                cand(1, kind::RANGED, 10.0, false),
                cand(2, kind::RANGED, 20.0, false),
                cand(3, kind::MELEE, 5.0, true),
            ],
            0.3,
        );
        assert_eq!(a.weapons[kind::RANGED], Some(FormId(2)));
        assert_eq!(a.scores[kind::RANGED], 20.0);
        assert_eq!(cheapest_kind(&a), Some(kind::RANGED));
    }

    #[test]
    fn reloads_fill_the_clip_unless_ammo_runs_short() {
        assert_eq!(reload_rounds(false, 5, 1), 5);
        assert_eq!(reload_rounds(true, 5, 12), 5);
        assert_eq!(reload_rounds(true, 5, 3), 3);
    }
}

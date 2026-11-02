//! Melee and unarmed fighting: blocking, power attacks, what a swing
//! hits, the unarmed specials and knockdowns, as `FalloutNV.exe` 1.4.0.525
//! does them (`docs/MELEE_UNARMED.md`).
//!
//! **Blocking.** The Aim control (6, "Block" in the INI) with a melee
//! weapon or fists out starts the block (`0093e860` → `00894cc0(1)`): the
//! process's block flag (+0xf0) is set and, for the player, `00894940`
//! plays the weapon's `BlockIdle` group (0xaa) as anim action 7; letting go
//! clears it. Someone is blocking while their anim action is 7
//! (`00894d60`). A hit on someone blocking (`009b5a30`, the hit's damage
//! through armour) adds to their damage threshold when the attacker (or the
//! projectile's shooter) is within their hit cone ([`in_hit_cone`],
//! `009a6ae0`): `fBlockSkillBase` + skill × `fBlockSkillMult`
//! ([`block_threshold`], `006463a0`), the skill Melee Weapons when both
//! have a weapon and the blocker's is a one- or two-handed melee weapon,
//! else Unarmed when the attacker has none or the blocker's is a
//! hand-to-hand weapon, else 0; nothing when blocking with fists against a
//! gun or a projectile. The hit is flagged blocked (`00407e00(1, 1)`) and
//! the player's counter-attack timer (+0xe28) is set to
//! `fCounterAttackTimer`.
//!
//! **Power attacks** (`00948310`, the Attack control): an attack starts on
//! the press; while the control stays held a timer (`011e07b0`) counts the
//! frame's seconds, and past `fPowerAttackDelay` (0.3) a power attack
//! follows ([`power_attack_group`]): `AttackPower` (0x5c) sneaking or with
//! both legs crippled, else by the movement keys `AttackForwardPower`
//! (0x5d), `AttackBackPower` (0x5e), `AttackLeftPower` (0x5f),
//! `AttackRightPower` (0x60), forward first; unarmed perks replace them by
//! the custom power attacks (entry points 61–66), and an unarmed attack
//! within the counter-attack timer with entry point 64 is the `Counter`
//! (0xa8). The power attack's damage is × `fDamagePowerAttackBonus`
//! unless the attacker sneaks (`009b5170`, `00644520`): inside a weapon's
//! damage, after the armour for fists (`world::combat::fists_power_bonus`).
//! Power attacks cost no action points or fatigue (the settings
//! `fActionPointsPowerAttackMult`, `fPowerAttackFatiguePenalty` and the
//! direction bonuses `fDamagePowerAttack…Bonus` are read by nothing), and
//! there's no cooldown besides the hold (`fPowerAttackDelay`).
//!
//! **What a swing hits** ([`find_target`], `009a60e0`, after
//! [`swing_reach`], `008990f0`): someone with a combat target only that
//! one; the player whoever is within reach (the gap between the bodies)
//! and the hit cone ([`Cones`], [`cone_check`]: the player's cone by the
//! attack, `009a6a40`; × 3 up close; dead ones × 2) nearest its middle.
//!
//! **Unarmed specials** ([`unarmed_special_group`], `00893a40`): outside
//! V.A.T.S. an unarmed attack becomes `Attack6` (uppercut) or `Attack7`
//! (cross) by chances from the Unarmed skill; the hit ([`special_of`],
//! `00899200` → `0089a760`) then staggers (uppercut, and the unarmed
//! left and right custom power attacks: a staggered person drops the
//! weapon when the arm holding it is hit) or does × 2.5 limb damage
//! (cross). V.A.T.S.'s Uppercut and Cross play the same groups
//! (`world::vats::attack_group`).
//!
//! **Knockdowns** ([`knockdown_chance`], [`knocks_down`]): the attacker's
//! "Knockdown Chance" perks (Super Slam) on each hit (`0089a760`).

use esm::{FormId, LoadOrder};

use crate::combat::{av, Weapon};
use crate::perks;
use crate::scripting::{game_setting, Facts, GameState};

/// The animation groups the player's melee attacks use (the table at
/// `011977d8`).
pub mod group {
    pub const ATTACK_RIGHT: u8 = 0x20;
    /// The unarmed uppercut and cross (`00893a40`, V.A.T.S.'s Uppercut and
    /// Cross, `00948310`).
    pub const ATTACK6: u8 = 0x38;
    pub const ATTACK7: u8 = 0x3e;
    pub const ATTACK_POWER: u8 = 0x5c;
    pub const ATTACK_FORWARD_POWER: u8 = 0x5d;
    pub const ATTACK_BACK_POWER: u8 = 0x5e;
    pub const ATTACK_LEFT_POWER: u8 = 0x5f;
    pub const ATTACK_RIGHT_POWER: u8 = 0x60;
    pub const ATTACK_CUSTOM1_POWER: u8 = 0x61;
    pub const ATTACK_CUSTOM2_POWER: u8 = 0x62;
    pub const ATTACK_CUSTOM3_POWER: u8 = 0x63;
    pub const ATTACK_CUSTOM4_POWER: u8 = 0x64;
    pub const ATTACK_CUSTOM5_POWER: u8 = 0x65;
    pub const COUNTER: u8 = 0xa8;
    /// V.A.T.S.'s Stomp (`00948310`).
    pub const STOMP: u8 = 0xa9;
    pub const BLOCK_IDLE: u8 = 0xaa;
    pub const BLOCK_HIT: u8 = 0xab;
}

/// The unarmed perks' entry points `00948310` asks (`005e58f0(0x3d…0x42)`;
/// the exe's names "Has Unarmed Forward/Back/Crouched/Left/Right Power
/// Attack").
pub mod entry {
    pub const UNARMED_FORWARD_POWER: u8 = 61;
    pub const UNARMED_BACK_POWER: u8 = 62;
    pub const UNARMED_CROUCHED_POWER: u8 = 63;
    pub const UNARMED_COUNTER: u8 = 64;
    pub const UNARMED_LEFT_POWER: u8 = 65;
    pub const UNARMED_RIGHT_POWER: u8 = 66;
}

/// The settings blocking and power attacks read (game settings; the exe's
/// defaults when the data doesn't set them).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// `fBlockSkillBase` (exe 0, `FalloutNV.esm` 5) and `fBlockSkillMult`
    /// (exe 1, data 0.3).
    pub block_skill_base: f32,
    pub block_skill_mult: f32,
    /// `fCombatHitConeAngle` (35 degrees).
    pub hit_cone: f32,
    /// `fCounterAttackTimer` (1 s).
    pub counter_attack_time: f32,
    /// `fPowerAttackDelay` (0.3 s).
    pub power_attack_delay: f32,
    /// `fDamagePowerAttackBonus` (exe 3, data 2).
    pub power_attack_bonus: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            block_skill_base: 0.0,
            block_skill_mult: 1.0,
            hit_cone: 35.0,
            counter_attack_time: 1.0,
            power_attack_delay: 0.3,
            power_attack_bonus: 3.0,
        }
    }
}

impl Settings {
    pub fn read(order: &LoadOrder) -> Settings {
        let d = Settings::default();
        let g = |n: &str, v: f32| game_setting(order, n).unwrap_or(v);
        Settings {
            block_skill_base: g("fBlockSkillBase", d.block_skill_base),
            block_skill_mult: g("fBlockSkillMult", d.block_skill_mult),
            hit_cone: g("fCombatHitConeAngle", d.hit_cone),
            counter_attack_time: g("fCounterAttackTimer", d.counter_attack_time),
            power_attack_delay: g("fPowerAttackDelay", d.power_attack_delay),
            power_attack_bonus: g("fDamagePowerAttackBonus", d.power_attack_bonus),
        }
    }
}

/// Whether `attacker_at` is within the hit cone of someone at `at` facing
/// `heading` (radians clockwise from north), as a block asks it: the
/// plain cone ([`cone_check`] with a living attacker).
pub fn in_hit_cone(at: [f32; 3], heading: f32, attacker_at: [f32; 3], cone: f32) -> bool {
    cone_check(at, heading, attacker_at, cone, false, 1.0).0
}

/// What a block adds to the blocker's damage threshold against one hit
/// (`009b5a30`, `006463a0`): `defender_weapon` the blocker's weapon
/// (`None`: fists), `attacker_weapon` the attacker's (`None`: unarmed);
/// `projectile` when the hit came from a projectile or an explosion.
// Translated from 009b5a30 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn block_threshold(
    order: &LoadOrder,
    state: &GameState,
    s: &Settings,
    defender: FormId,
    defender_weapon: Option<&Weapon>,
    attacker_weapon: Option<&Weapon>,
    projectile: bool,
) -> f32 {
    // Fists against a gun or a projectile block nothing.
    let fists = defender_weapon.is_none();
    if fists && (attacker_weapon.is_some_and(|w| !w.is_melee()) || projectile) {
        return 0.0;
    }
    let skill = |a: u16| {
        Facts {
            order,
            state,
            speaker: None,
        }
        .current_actor_value(defender, a)
        .unwrap_or(0.0) as f32
    };
    let skill = match (attacker_weapon, defender_weapon) {
        (Some(_), Some(d)) if d.animation == 1 || d.animation == 2 => skill(av::MELEE_WEAPONS),
        (None, _) => skill(av::UNARMED),
        (Some(_), Some(d)) if d.animation == 0 => skill(av::UNARMED),
        _ => 0.0,
    };
    s.block_skill_base + skill * s.block_skill_mult
}

/// What `target`'s block adds to their threshold against `attacker`'s hit
/// with `weapon` (`None`: unarmed), if they're blocking
/// (`GameState::blocking`) and the attacker is in their hit cone: see
/// [`block_threshold`]. A gun's shot counts as a projectile. Hits with no
/// attacker (no position to test the cone against) aren't blocked here.
pub fn block_bonus(
    order: &LoadOrder,
    state: &GameState,
    attacker: FormId,
    weapon: Option<FormId>,
    target: FormId,
) -> Option<f32> {
    let &heading = state.blocking.get(&target)?;
    let (_, _, at, _) = state.place(order, target)?;
    let (_, _, from, _) = state.place(order, attacker)?;
    let s = Settings::read(order);
    if !in_hit_cone(at, heading, from, s.hit_cone) {
        return None;
    }
    let attacker_weapon = weapon.and_then(|w| Weapon::load(order, w));
    let defender_weapon = crate::combat::weapon_in_hand(order, state, target);
    let projectile = attacker_weapon.as_ref().is_some_and(|w| !w.is_melee());
    Some(block_threshold(
        order,
        state,
        &s,
        target,
        defender_weapon.as_ref(),
        attacker_weapon.as_ref(),
        projectile,
    ))
}

/// Which movement keys are held (the movement flags' low bits).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Moving {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}

/// The power attack's group (`00948310`, after the hold passes
/// `fPowerAttackDelay`): see the module notes. `unarmed`: no weapon or an
/// Unarmed-skill weapon (the perks' moves are asked only then);
/// `legs_crippled`: both legs' condition (actor values 29, 30) at most 0.
// Translated from 00948310 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn power_attack_group(
    order: &LoadOrder,
    state: &GameState,
    moving: Moving,
    sneaking: bool,
    legs_crippled: bool,
    unarmed: bool,
) -> u8 {
    let has = |e: u8| unarmed && perks::apply(order, state, e, 0.0) > 0.0;
    if sneaking || legs_crippled {
        return if has(entry::UNARMED_CROUCHED_POWER) {
            group::ATTACK_CUSTOM3_POWER
        } else {
            group::ATTACK_POWER
        };
    }
    let (g, custom, e) = if moving.forward {
        (
            group::ATTACK_FORWARD_POWER,
            group::ATTACK_CUSTOM1_POWER,
            entry::UNARMED_FORWARD_POWER,
        )
    } else if moving.back {
        (
            group::ATTACK_BACK_POWER,
            group::ATTACK_CUSTOM2_POWER,
            entry::UNARMED_BACK_POWER,
        )
    } else if moving.left {
        (
            group::ATTACK_LEFT_POWER,
            group::ATTACK_CUSTOM4_POWER,
            entry::UNARMED_LEFT_POWER,
        )
    } else if moving.right {
        (
            group::ATTACK_RIGHT_POWER,
            group::ATTACK_CUSTOM5_POWER,
            entry::UNARMED_RIGHT_POWER,
        )
    } else {
        return group::ATTACK_POWER;
    };
    if has(e) {
        custom
    } else {
        g
    }
}

/// An unarmed attack within the counter-attack timer becomes the `Counter`
/// (0xa8) with entry point 64 (`00948310`).
pub fn counter_attack(
    order: &LoadOrder,
    state: &GameState,
    unarmed: bool,
    counter_timer: f32,
) -> bool {
    unarmed && counter_timer > 0.0 && perks::apply(order, state, entry::UNARMED_COUNTER, 0.0) > 0.0
}

/// A power attack's damage multiplier (`009b5170`: `00644520` returns
/// `fDamagePowerAttackBonus`): only when not sneaking.
pub fn power_attack_mult(s: &Settings, power: bool, sneaking: bool) -> f32 {
    if power && !sneaking {
        s.power_attack_bonus
    } else {
        1.0
    }
}

/// The hit cones' settings (`009a6a40`, `009a6ae0`; the exe's defaults,
/// which `FalloutNV.esm` doesn't change).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cones {
    /// `fCombatHitConeAngle` (35 degrees): everyone's, and the player's
    /// for attacks other than the ones below.
    pub hit: f32,
    /// `fCombatOverheadHitConeAngle` (40): the player's `AttackPower` and
    /// `AttackForwardPower`.
    pub overhead: f32,
    /// `fCombatSweepHitConeAngle` (100): the player's back, left and right
    /// power attacks.
    pub sweep: f32,
    /// `fCombatUppercutHitConeAngle` (50): the player's
    /// `AttackCustom1Power` (the unarmed forward power attack).
    pub uppercut: f32,
    /// `fCombatDeadActorHitConeMult` (2): the cone × it for a dead target
    /// outside it.
    pub dead_mult: f32,
}

impl Default for Cones {
    fn default() -> Self {
        Cones {
            hit: 35.0,
            overhead: 40.0,
            sweep: 100.0,
            uppercut: 50.0,
            dead_mult: 2.0,
        }
    }
}

impl Cones {
    pub fn read(order: &LoadOrder) -> Cones {
        let d = Cones::default();
        let g = |n: &str, v: f32| game_setting(order, n).unwrap_or(v);
        Cones {
            hit: g("fCombatHitConeAngle", d.hit),
            overhead: g("fCombatOverheadHitConeAngle", d.overhead),
            sweep: g("fCombatSweepHitConeAngle", d.sweep),
            uppercut: g("fCombatUppercutHitConeAngle", d.uppercut),
            dead_mult: g("fCombatDeadActorHitConeMult", d.dead_mult),
        }
    }

    /// The player's cone for the attack group playing (`009a6a40`): 0x5c
    /// and 0x5d overhead, 0x5e–0x60 sweep, 0x61 uppercut, anything else
    /// the plain one. Everyone else's is always the plain one
    /// (`009a6ae0`).
    // Translated from 009a6a40 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn for_attack(&self, group: u8) -> f32 {
        match group {
            group::ATTACK_POWER | group::ATTACK_FORWARD_POWER => self.overhead,
            group::ATTACK_BACK_POWER..=group::ATTACK_RIGHT_POWER => self.sweep,
            group::ATTACK_CUSTOM1_POWER => self.uppercut,
            _ => self.hit,
        }
    }
}

/// The angle (degrees, 0–180) between `heading` (radians clockwise from
/// north) at `at` and the direction to `other` (`009a6ae0`).
pub fn angle_off(at: [f32; 3], heading: f32, other: [f32; 3]) -> f32 {
    let d = [other[0] - at[0], other[1] - at[1]];
    let to_other = d[0].atan2(d[1]);
    // `00408860` (fabs) of the difference, in degrees, folded past 180.
    let mut angle = ((heading - to_other) * 57.29578).abs();
    if angle > 180.0 {
        angle = (angle - 360.0).abs();
    }
    angle
}

/// Whether `other` is within the hit cone of someone at `at` facing
/// `heading` (`009a6ae0`), and the angle off: within `cone` degrees,
/// × 3 when they're less than 128 units apart (the whole vector's
/// length, `00457990`); a dead `other` outside that, within the cone ×
/// `dead_mult`. (The player's auto-aim turn, `00965620` on the
/// `ProjectileNode`, isn't added to the heading here.)
// Translated from 009a6ae0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn cone_check(
    at: [f32; 3],
    heading: f32,
    other: [f32; 3],
    cone: f32,
    other_dead: bool,
    dead_mult: f32,
) -> (bool, f32) {
    let angle = angle_off(at, heading, other);
    let d = [other[0] - at[0], other[1] - at[1], other[2] - at[2]];
    let distance = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    let cone = if distance < 128.0 { cone * 3.0 } else { cone };
    let inside = angle <= cone || (other_dead && angle <= cone * dead_mult);
    (inside, angle)
}

/// Someone a swing might meet (`009a60e0`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Body {
    pub reference: FormId,
    /// Where they stand.
    pub position: [f32; 3],
    /// Their collision radius (`008be420`).
    pub radius: f32,
    /// The bottom and top of their bounds above where they stand
    /// (vtable `+0x1d8` min and `+0x1dc` max, z; not scaled), read by the
    /// gap ([`Swing::gap`]).
    pub bound_z: [f32; 2],
    /// Swimming (actor `+0x14d`, `005a2030`).
    pub swimming: bool,
    pub dead: bool,
}

/// A melee attack looking for what it hits (`009a60e0`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Swing {
    pub at: [f32; 3],
    /// Radians clockwise from north.
    pub heading: f32,
    /// The attacker's collision radius.
    pub radius: f32,
    /// The attacker's bounds' bottom and top z ([`Body::bound_z`]).
    pub bound_z: [f32; 2],
    /// The attacker is swimming ([`Body::swimming`]).
    pub swimming: bool,
    /// `fAICombatSlopeDifference` (48, `00f63ab0`): the height difference
    /// from which a combat target's gap is measured flat.
    pub slope_difference: f32,
    /// [`swing_reach`].
    pub reach: f32,
    /// The hit cone ([`Cones::for_attack`] for the player).
    pub cone: f32,
    pub dead_mult: f32,
    pub player: bool,
    /// V.A.T.S. is playing (mode 4).
    pub vats_playback: bool,
}

impl Swing {
    /// The gap to `b` (`009a64d0`): the distance less both radii
    /// (`009a6770`). The distance is the 3D one (`005723b0`), except that
    /// it's measured flat (z dropped, `00439ef0`/`00457990`) when the
    /// swinger's bounds' top or bottom lies within `b`'s bounds' height
    /// span, which is asked when both swim (`005a2030`, actor `+0x14d`) or,
    /// for the attacker's combat target (`009a60e0` passes 1 only there),
    /// when their heights differ by at least `fAICombatSlopeDifference`.
    // Translated from 009a64d0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn gap(&self, b: &Body, combat_target: bool) -> f32 {
        let d = [
            b.position[0] - self.at[0],
            b.position[1] - self.at[1],
            b.position[2] - self.at[2],
        ];
        let mut distance = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        let flat_asked = (self.swimming && b.swimming)
            || (combat_target && (self.at[2] - b.position[2]).abs() >= self.slope_difference);
        if flat_asked {
            let (a_top, a_bottom) = (self.bound_z[1] + self.at[2], self.bound_z[0] + self.at[2]);
            let (b_top, b_bottom) = (b.bound_z[1] + b.position[2], b.bound_z[0] + b.position[2]);
            let within = |z: f32| b_bottom <= z && z <= b_top;
            if within(a_top) || within(a_bottom) {
                distance = (d[0] * d[0] + d[1] * d[1]).sqrt();
            }
        }
        distance - (self.radius + b.radius)
    }

    fn cone(&self, b: &Body) -> (bool, f32) {
        cone_check(
            self.at,
            self.heading,
            b.position,
            self.cone,
            b.dead,
            self.dead_mult,
        )
    }
}

/// How far a swing reaches (`008990f0`): a weapon's reach ×
/// `fCombatDistance` (128), else `fHandReachMult` (0.5) × 128 for people
/// (the actor's `GetReach`, vtable `+0x380`, `0088b850`), × the
/// attacker's scale, × `fVATSMeleeReachMult` (2) for the player while
/// V.A.T.S. plays.
// Translated from 008990f0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn swing_reach(
    order: &LoadOrder,
    weapon: Option<&Weapon>,
    scale: f32,
    player_in_vats: bool,
) -> f32 {
    let g = |n: &str, v: f32| game_setting(order, n).unwrap_or(v);
    let distance = g("fCombatDistance", 128.0);
    let reach = match weapon {
        Some(w) => w.reach * distance,
        None => g("fHandReachMult", 0.5) * distance,
    };
    let reach = reach * scale;
    if player_in_vats {
        reach * g("fVATSMeleeReachMult", 2.0)
    } else {
        reach
    }
}

/// Whom a melee attack hits (`Actor::FindMeleeTarget`, Xbox PDB, `009a60e0`).
/// An attacker with a combat target hits only that one, when it's in the
/// cone and the gap is at most the reach. Otherwise (the player) everyone
/// near is looked at: not the attacker, not the dead while V.A.T.S. plays,
/// the dead only for the player; within reach and the cone; the one
/// nearest the middle of the cone (the smallest angle; the later of
/// equals) is hit. The player's swing then needs a line of sight to them
/// (`0088b880`), which the caller checks.
// Translated from 009a60e0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn find_target(swing: &Swing, combat_target: Option<&Body>, near: &[Body]) -> Option<FormId> {
    if let Some(t) = combat_target {
        return (swing.cone(t).0 && swing.gap(t, true) <= swing.reach).then_some(t.reference);
    }
    let mut best: Option<(FormId, f32)> = None;
    for b in near {
        if swing.vats_playback && b.dead {
            continue;
        }
        if b.dead && !swing.player {
            continue;
        }
        if swing.gap(b, false) > swing.reach {
            continue;
        }
        let (inside, angle) = swing.cone(b);
        if inside && best.map_or(true, |(_, a)| angle <= a) {
            best = Some((b.reference, angle));
        }
    }
    best.map(|(r, _)| r)
}

/// The settings of the unarmed specials outside V.A.T.S. (`00893a40`; the
/// exe's defaults, which `FalloutNV.esm` doesn't change).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnarmedSpecials {
    /// `fUpperCutThreshold` (50) and `fUpperCutSkillChance` (0.15).
    pub uppercut_threshold: f32,
    pub uppercut_chance: f32,
    /// `fCrossThreshold` (75) and `fCrossSkillChance` (0.15).
    pub cross_threshold: f32,
    pub cross_chance: f32,
    /// `fCrossSkillDamageMultiplier` (2.5): a cross's limb damage.
    pub cross_limb_mult: f32,
    /// `iCombatCrippledTorsoHitStaggerChance` (50).
    pub torso_stagger_chance: f32,
}

impl Default for UnarmedSpecials {
    fn default() -> Self {
        UnarmedSpecials {
            uppercut_threshold: 50.0,
            uppercut_chance: 0.15,
            cross_threshold: 75.0,
            cross_chance: 0.15,
            cross_limb_mult: 2.5,
            torso_stagger_chance: 50.0,
        }
    }
}

impl UnarmedSpecials {
    pub fn read(order: &LoadOrder) -> UnarmedSpecials {
        let d = UnarmedSpecials::default();
        let g = |n: &str, v: f32| game_setting(order, n).unwrap_or(v);
        UnarmedSpecials {
            uppercut_threshold: g("fUpperCutThreshold", d.uppercut_threshold),
            uppercut_chance: g("fUpperCutSkillChance", d.uppercut_chance),
            cross_threshold: g("fCrossThreshold", d.cross_threshold),
            cross_chance: g("fCrossSkillChance", d.cross_chance),
            cross_limb_mult: g("fCrossSkillDamageMultiplier", d.cross_limb_mult),
            torso_stagger_chance: g(
                "iCombatCrippledTorsoHitStaggerChance",
                d.torso_stagger_chance,
            ),
        }
    }
}

/// Whether an attack may turn into an unarmed special (`00893a40`): with
/// no weapon or a hand-to-hand one (animation type 0) whose attack
/// animation isn't a loop or spin (0x4a–0x5b), by a person or the player,
/// not sneaking (or while V.A.T.S. plays), and outside V.A.T.S. (where
/// the queue picks the moves).
pub fn may_turn_special(
    weapon: Option<&Weapon>,
    person: bool,
    sneaking: bool,
    in_vats: bool,
) -> bool {
    let hand = weapon.map_or(true, |w| {
        w.animation == 0 && !(0x4a..0x5c).contains(&w.attack_animation)
    });
    hand && person && !sneaking && !in_vats
}

/// The group an unarmed attack turns into (`Actor::StartAttack`, Xbox
/// PDB, `00893a40`), outside V.A.T.S.: with Unarmed (whole points) above
/// `fUpperCutThreshold`, a chance of Unarmed × `fUpperCutSkillChance` %
/// for `Attack6` (0x38, the uppercut), and above `fCrossThreshold` a
/// further Unarmed × `fCrossSkillChance` % for `Attack7` (0x3e, the
/// cross); `roll` U(0, 100). `None`: the attack stays as it was.
// Translated from 00893a40 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn unarmed_special_group(s: &UnarmedSpecials, unarmed_skill: f32, roll: f32) -> Option<u8> {
    let skill = unarmed_skill.trunc();
    let uppercut = if s.uppercut_threshold < skill {
        skill * s.uppercut_chance
    } else {
        0.0
    };
    let cross = if s.cross_threshold < skill {
        skill * s.cross_chance
    } else {
        0.0
    };
    if uppercut == 0.0 && cross == 0.0 {
        return None;
    }
    if roll <= uppercut {
        Some(group::ATTACK6)
    } else if roll < uppercut + cross {
        Some(group::ATTACK7)
    } else {
        None
    }
}

/// What an unarmed hit does besides its damage, by the attack group that
/// struck (`Actor::MeleeAttack`, Xbox PDB, `00899200`, passed to the hit,
/// `0089a760`): with no weapon or a hand-to-hand one, `Attack6` and the
/// left and right custom power attacks (0x64, 0x65) stagger, `Attack7`
/// crosses.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Special {
    #[default]
    None,
    /// 1: the one hit staggers (and drops the weapon when an arm is hit).
    Stagger,
    /// 2: the limb damage × `fCrossSkillDamageMultiplier`.
    Cross,
}

/// A melee blow's particulars for the hit (`Runner::blow_at`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Blow {
    /// A power attack (`fDamagePowerAttackBonus`).
    pub power: bool,
    /// [`special_of`].
    pub special: Special,
}

/// The [`Special`] of a blow struck with `weapon` in `attack_group`.
// Translated from 00899200 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn special_of(weapon: Option<&Weapon>, attack_group: u8) -> Special {
    if weapon.is_some_and(|w| w.animation != 0) {
        return Special::None;
    }
    match attack_group {
        group::ATTACK6 | group::ATTACK_CUSTOM4_POWER | group::ATTACK_CUSTOM5_POWER => {
            Special::Stagger
        }
        group::ATTACK7 => Special::Cross,
        _ => Special::None,
    }
}

/// The chance (0..1) that `attacker`'s hit with `weapon` knocks the one
/// hit down (`0089a760`): their perks' "Knockdown Chance" (entry point
/// 52, asked about the weapon, the fists when none) from 0 — Super Slam
/// 0.15 with hand-to-hand and one-handed melee weapons, 0.30 with
/// two-handed ones.
pub fn knockdown_chance(
    order: &LoadOrder,
    state: &GameState,
    attacker: FormId,
    weapon: Option<FormId>,
) -> f32 {
    perks::apply_for(
        order,
        state,
        attacker,
        perks::entry::KNOCKDOWN_CHANCE,
        0.0,
        &[perks::weapon_tab(weapon)],
    )
}

/// Whether the hit knocks them down (`0089a760`): a chance above 0 and a
/// roll U(0, 1) (`00476b70(0, 1)`) at most the chance. The knockdown
/// itself (the process's knock, vtable `+0x418`, from the attacker's
/// position, with `00646580`'s force from the target's Agility) is the
/// animation's and the physics'.
// Translated from 0089a760 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn knocks_down(chance: f32, roll: f32) -> bool {
    chance > 0.0 && roll <= chance
}

/// The animation group's file stem in `_1stperson` / `_male` (`AttackPower`
/// → `attackpower`), for the weapon kinds' files (`1hmattackpower.kf`).
pub fn group_file_stem(g: u8) -> Option<String> {
    crate::animation::GROUPS
        .get(usize::from(g))
        .map(|(name, _, _)| name.to_ascii_lowercase())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn the_hit_cone_faces_the_heading_and_widens_up_close() {
        // Facing north; an attacker 500 north is dead ahead, 500 east is
        // 90 degrees off.
        assert!(in_hit_cone([0.0; 3], 0.0, [0.0, 500.0, 0.0], 35.0));
        assert!(!in_hit_cone([0.0; 3], 0.0, [500.0, 0.0, 0.0], 35.0));
        // 40 degrees off at 500: outside; at 100 units: inside (35 × 3).
        let off = 40f32.to_radians();
        assert!(!in_hit_cone(
            [0.0; 3],
            0.0,
            [500.0 * off.sin(), 500.0 * off.cos(), 0.0],
            35.0
        ));
        assert!(in_hit_cone(
            [0.0; 3],
            0.0,
            [100.0 * off.sin(), 100.0 * off.cos(), 0.0],
            35.0
        ));
        // Headings either side of north fold through 360.
        assert!(in_hit_cone([0.0; 3], 6.2, [0.0, 500.0, 0.0], 35.0));
    }

    pub(crate) fn weapon(animation: u32) -> Weapon {
        Weapon {
            form_id: FormId(0x10),
            name: String::new(),
            damage: 10.0,
            clip: 1,
            health: 100,
            animation,
            ammo: Vec::new(),
            ammo_use: 1,
            min_spread: 0.0,
            spread: 0.0,
            projectile: None,
            projectiles: 1,
            min_range: 0.0,
            max_range: 0.0,
            shots_per_second: 1.0,
            reload_time: 0.0,
            skill: 38,
            crit_damage: 0.0,
            crit_mult: 1.0,
            sound: None,
            attack_animation: 255,
            reload_animation: 255,
            kill_impulse: 0.0,
            impulse_distance: 0.0,
            reach: 1.0,
            limb_damage_mult: 1.0,
            flags1: 0,
            flags2: 0,
            fire_rate: 0.0,
            attack_mult: 1.0,
            aim_arc: 0.0,
            semi_auto_delay: (0.0, 0.0),
            speed: 1.0,
            cone_mult: 1.0,
            crit_effect: None,
            crit_on_death: false,
            resist: None,
        }
    }

    #[test]
    fn a_block_adds_the_blocking_skill_to_the_threshold() {
        let order = LoadOrder::from_plugins(Vec::new()).unwrap();
        let state = GameState::default();
        let s = Settings {
            block_skill_base: 5.0,
            block_skill_mult: 0.3,
            ..Settings::default()
        };
        let who = FormId(0x14);
        let skill = |a: u16| {
            Facts {
                order: &order,
                state: &state,
                speaker: None,
            }
            .current_actor_value(who, a)
            .unwrap_or(0.0) as f32
        };
        let melee = skill(av::MELEE_WEAPONS);
        let unarmed = skill(av::UNARMED);
        let machete = weapon(1);
        let pistol = weapon(3);
        let knuckles = weapon(0);
        // A machete blocking a machete or a pistol: Melee Weapons.
        let t = block_threshold(
            &order,
            &state,
            &s,
            who,
            Some(&machete),
            Some(&machete),
            false,
        );
        assert_eq!(t, 5.0 + melee * 0.3);
        let t = block_threshold(&order, &state, &s, who, Some(&machete), Some(&pistol), true);
        assert_eq!(t, 5.0 + melee * 0.3);
        // Anything blocking a punch: Unarmed.
        let t = block_threshold(&order, &state, &s, who, Some(&machete), None, false);
        assert_eq!(t, 5.0 + unarmed * 0.3);
        // Brass knuckles blocking a machete: Unarmed.
        let t = block_threshold(
            &order,
            &state,
            &s,
            who,
            Some(&knuckles),
            Some(&machete),
            false,
        );
        assert_eq!(t, 5.0 + unarmed * 0.3);
        // Fists blocking a machete: no skill, the base.
        assert_eq!(
            block_threshold(&order, &state, &s, who, None, Some(&machete), false),
            5.0
        );
        // Fists against a gun or a projectile: nothing.
        assert_eq!(
            block_threshold(&order, &state, &s, who, None, Some(&pistol), true),
            0.0
        );
        assert_eq!(
            block_threshold(&order, &state, &s, who, None, None, true),
            0.0
        );
    }

    #[test]
    fn power_attacks_go_the_way_the_player_moves() {
        let order = LoadOrder::from_plugins(Vec::new()).unwrap();
        let state = GameState::default();
        let still = Moving::default();
        let g = |m: Moving, sneak: bool, legs: bool| {
            power_attack_group(&order, &state, m, sneak, legs, true)
        };
        assert_eq!(g(still, false, false), group::ATTACK_POWER);
        let forward = Moving {
            forward: true,
            left: true,
            ..still
        };
        assert_eq!(g(forward, false, false), group::ATTACK_FORWARD_POWER);
        let back = Moving {
            back: true,
            ..still
        };
        assert_eq!(g(back, false, false), group::ATTACK_BACK_POWER);
        let left = Moving {
            left: true,
            ..still
        };
        assert_eq!(g(left, false, false), group::ATTACK_LEFT_POWER);
        let right = Moving {
            right: true,
            ..still
        };
        assert_eq!(g(right, false, false), group::ATTACK_RIGHT_POWER);
        // Sneaking or both legs crippled: the plain one.
        assert_eq!(g(forward, true, false), group::ATTACK_POWER);
        assert_eq!(g(forward, false, true), group::ATTACK_POWER);
        assert_eq!(
            group_file_stem(group::ATTACK_LEFT_POWER).as_deref(),
            Some("attackleftpower")
        );
        assert_eq!(
            group_file_stem(group::BLOCK_IDLE).as_deref(),
            Some("blockidle")
        );
        // The damage bonus, not while sneaking.
        let s = Settings {
            power_attack_bonus: 2.0,
            ..Settings::default()
        };
        assert_eq!(power_attack_mult(&s, true, false), 2.0);
        assert_eq!(power_attack_mult(&s, true, true), 1.0);
        assert_eq!(power_attack_mult(&s, false, false), 1.0);
        assert!(!counter_attack(&order, &state, true, 0.5));
    }

    #[test]
    fn the_players_cone_follows_the_attack_and_widens_for_the_dead() {
        let c = Cones::default();
        assert_eq!(c.for_attack(group::ATTACK_RIGHT), 35.0);
        assert_eq!(c.for_attack(group::ATTACK_POWER), 40.0);
        assert_eq!(c.for_attack(group::ATTACK_FORWARD_POWER), 40.0);
        assert_eq!(c.for_attack(group::ATTACK_BACK_POWER), 100.0);
        assert_eq!(c.for_attack(group::ATTACK_LEFT_POWER), 100.0);
        assert_eq!(c.for_attack(group::ATTACK_RIGHT_POWER), 100.0);
        assert_eq!(c.for_attack(group::ATTACK_CUSTOM1_POWER), 50.0);
        assert_eq!(c.for_attack(group::ATTACK_CUSTOM2_POWER), 35.0);
        // 50 degrees off at 300 units: outside 35, inside for the dead
        // (× 2) and for a sweep.
        let off = 50f32.to_radians();
        let at = [300.0 * off.sin(), 300.0 * off.cos(), 0.0];
        assert!(!cone_check([0.0; 3], 0.0, at, 35.0, false, 2.0).0);
        assert!(cone_check([0.0; 3], 0.0, at, 35.0, true, 2.0).0);
        assert!(cone_check([0.0; 3], 0.0, at, 100.0, false, 2.0).0);
        assert!((cone_check([0.0; 3], 0.0, at, 35.0, false, 2.0).1 - 50.0).abs() < 1e-3);
    }

    fn body(r: u32, at: [f32; 3], dead: bool) -> Body {
        Body {
            reference: FormId(r),
            position: at,
            radius: 20.25,
            bound_z: [0.0, 128.0],
            swimming: false,
            dead,
        }
    }

    fn test_swing() -> Swing {
        Swing {
            at: [0.0; 3],
            heading: 0.0,
            radius: 20.25,
            bound_z: [0.0, 128.0],
            swimming: false,
            slope_difference: 48.0,
            reach: 64.0,
            cone: 35.0,
            dead_mult: 2.0,
            player: true,
            vats_playback: false,
        }
    }

    #[test]
    fn the_gap_is_flat_for_a_combat_target_on_a_slope_or_both_swimming() {
        // 009a64d0: 60 north and 60 up. Standing heights 0..128 overlap.
        let swing = test_swing();
        let up = body(1, [0.0, 60.0, 60.0], false);
        let three_d = (60f32 * 60.0 * 2.0).sqrt() - 40.5;
        assert!((swing.gap(&up, false) - three_d).abs() < 1e-3);
        // A combat target 60 higher (>= fAICombatSlopeDifference 48): flat.
        assert!((swing.gap(&up, true) - (60.0 - 40.5)).abs() < 1e-3);
        // 40 higher: under the slope difference, still 3D.
        let low = body(2, [0.0, 60.0, 40.0], false);
        let three_d = (60f32 * 60.0 + 40.0 * 40.0).sqrt() - 40.5;
        assert!((swing.gap(&low, true) - three_d).abs() < 1e-3);
        // Both swimming: flat whoever it is.
        let swimmer = Swing {
            swimming: true,
            ..swing
        };
        let low_swimmer = Body {
            swimming: true,
            ..low
        };
        assert!((swimmer.gap(&low_swimmer, false) - (60.0 - 40.5)).abs() < 1e-3);
        // Heights apart (200 up: neither end of the swinger's 0..128 is
        // within 200..328): 3D even for the combat target.
        let above = body(3, [0.0, 60.0, 200.0], false);
        let three_d = (60f32 * 60.0 + 200.0 * 200.0).sqrt() - 40.5;
        assert!((swing.gap(&above, true) - three_d).abs() < 1e-3);
    }

    #[test]
    fn a_swing_hits_the_one_nearest_the_middle_within_reach() {
        let swing = test_swing();
        // 100 north (gap 59.5) and 100 at 20 degrees: the straight one.
        let off = 20f32.to_radians();
        let ahead = body(1, [0.0, 100.0, 0.0], false);
        let aside = body(2, [100.0 * off.sin(), 100.0 * off.cos(), 0.0], false);
        assert_eq!(find_target(&swing, None, &[aside, ahead]), Some(FormId(1)));
        // Out of reach (gap 79.5 > 64): nobody.
        let far = body(3, [0.0, 120.0, 0.0], false);
        assert_eq!(find_target(&swing, None, &[far]), None);
        // The player may hit the dead, but not while V.A.T.S. plays;
        // others never.
        let corpse = body(4, [0.0, 100.0, 0.0], true);
        assert_eq!(find_target(&swing, None, &[corpse]), Some(FormId(4)));
        let vats = Swing {
            vats_playback: true,
            ..swing
        };
        assert_eq!(find_target(&vats, None, &[corpse]), None);
        let npc = Swing {
            player: false,
            ..swing
        };
        assert_eq!(find_target(&npc, None, &[corpse]), None);
        // With a combat target, only that one, and only in reach and cone.
        assert_eq!(
            find_target(&npc, Some(&aside), &[ahead, aside]),
            Some(FormId(2))
        );
        assert_eq!(find_target(&npc, Some(&far), &[ahead]), None);
    }

    #[test]
    fn unarmed_skill_turns_attacks_into_uppercuts_and_crosses() {
        let s = UnarmedSpecials::default();
        // Unarmed 50: nothing (above 50 is asked).
        assert_eq!(unarmed_special_group(&s, 50.0, 0.0), None);
        // Unarmed 60: 9% uppercut.
        assert_eq!(unarmed_special_group(&s, 60.9, 9.0), Some(group::ATTACK6));
        assert_eq!(unarmed_special_group(&s, 60.0, 9.5), None);
        // Unarmed 100: 15% uppercut, then 15% cross.
        assert_eq!(unarmed_special_group(&s, 100.0, 15.0), Some(group::ATTACK6));
        assert_eq!(unarmed_special_group(&s, 100.0, 29.0), Some(group::ATTACK7));
        assert_eq!(unarmed_special_group(&s, 100.0, 30.5), None);
        // Who may: fists or hand-to-hand weapons, a person, not sneaking,
        // not in V.A.T.S.
        let knuckles = weapon(0);
        let machete = weapon(1);
        assert!(may_turn_special(None, true, false, false));
        assert!(may_turn_special(Some(&knuckles), true, false, false));
        assert!(!may_turn_special(Some(&machete), true, false, false));
        assert!(!may_turn_special(None, false, false, false));
        assert!(!may_turn_special(None, true, true, false));
        assert!(!may_turn_special(None, true, false, true));
        // What the groups do when they hit.
        assert_eq!(special_of(None, group::ATTACK6), Special::Stagger);
        assert_eq!(
            special_of(None, group::ATTACK_CUSTOM4_POWER),
            Special::Stagger
        );
        assert_eq!(
            special_of(None, group::ATTACK_CUSTOM5_POWER),
            Special::Stagger
        );
        assert_eq!(special_of(Some(&knuckles), group::ATTACK7), Special::Cross);
        assert_eq!(special_of(Some(&machete), group::ATTACK6), Special::None);
        assert_eq!(special_of(None, group::ATTACK_RIGHT), Special::None);
        assert!(knocks_down(0.15, 0.15) && !knocks_down(0.15, 0.2) && !knocks_down(0.0, 0.0));
    }
}

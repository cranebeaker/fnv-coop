//! How fast someone walks and runs, when the player may run and jump, and
//! how a jump starts and is steered in the air: read from `FalloutNV.exe`
//! 1.4.0.525 with Ghidra (evidence and gaps: `docs/MOVEMENT.md`). The
//! physics crate moves the character controller; the viewer reads the keys.
//!
//! - Speed ([`walk_speed`], [`run_speed`], [`speed`]): `00647d10` and
//!   `00647f00`, called by `Actor::GetWalkSpeed` (Xbox PDB) at `00885a50`
//!   and `Actor::GetRunSpeed` (Xbox PDB) at `00885bf0`.
//! - Running and jumping allowed ([`may_run`], [`may_jump`]): the player's
//!   controls handler at `0093e860`.
//! - The jump ([`jump_height`], [`jump_speed`]): `00930640` and the jumping
//!   state `00cd4280`; steering in the air ([`air_gain`]): the in-air state
//!   `00cd3fb0`.

use esm::{FormId, FourCC, LoadOrder};

use crate::body_parts::{av as part_av, ignores_crippled_limbs, is_crippled};
use crate::scripting::{base_of, game_setting, Facts, GameState};

const ARMO: FourCC = FourCC::new(b"ARMO");
const BMDT: FourCC = FourCC::new(b"BMDT");
const CREA: FourCC = FourCC::new(b"CREA");

/// SpeedMult, the actor value the speed scales with (21).
pub const SPEED_MULT: u16 = 21;

/// The upper-body biped slot (index 2, mask 0x4): the armour whose weight
/// class slows its wearer (`00885a50` asks `00891b90` for slot 2).
pub const UPPER_BODY: u32 = 1 << 2;

/// The game settings the speed rules read (`GMST` where the data sets
/// them, else the exe's defaults; `FalloutNV.esm` sets the base speed, the
/// sneak multiplier, both armour penalties and the big two-hander penalty).
#[derive(Debug, Clone, PartialEq)]
pub struct SpeedSettings {
    /// `fMoveBaseSpeed` (exe 85, data 77).
    pub base: f32,
    /// `fMoveRunMult` (4).
    pub run_mult: f32,
    /// `fMoveSneakMult` (exe 0.6, data 0.57).
    pub sneak_mult: f32,
    /// `fMoveNoWeaponMult` (1.1): people with their weapon put away.
    pub no_weapon_mult: f32,
    /// `fMoveHeavyArmorPenalty` (exe 0.2, data 0.15) and
    /// `fMoveMediumArmorPenalty` (exe 0.1, data 0.075).
    pub heavy_armor: f32,
    pub medium_armor: f32,
    /// `fMove2HRPenalty` (0.1): rifles and automatics drawn.
    pub two_hand_rifle: f32,
    /// `fMove2HBigPenalty` (exe 0.15, data 0.1): two-handed "handle"
    /// weapons and launchers drawn.
    pub two_hand_big: f32,
    /// `fMoveOneCrippledLegSpeedMult` (0.85), `fMoveTwoCrippledLegsSpeedMult`
    /// (0.75).
    pub one_leg: f32,
    pub two_legs: f32,
    /// `fJumpHeightMin` (64, not in the data) and `fJumpSwimmingMult` (2).
    pub jump_height: f32,
    pub jump_swimming_mult: f32,
    /// `fGrabMaxWeightRunning` (50): carrying (grabbing) something heavier
    /// stops the player running.
    pub grab_max_weight_running: f32,
}

impl SpeedSettings {
    pub fn read(order: &LoadOrder) -> SpeedSettings {
        let d = SpeedSettings::defaults();
        let g = |name: &str, default: f32| game_setting(order, name).unwrap_or(default);
        SpeedSettings {
            base: g("fMoveBaseSpeed", d.base),
            run_mult: g("fMoveRunMult", d.run_mult),
            sneak_mult: g("fMoveSneakMult", d.sneak_mult),
            no_weapon_mult: g("fMoveNoWeaponMult", d.no_weapon_mult),
            heavy_armor: g("fMoveHeavyArmorPenalty", d.heavy_armor),
            medium_armor: g("fMoveMediumArmorPenalty", d.medium_armor),
            two_hand_rifle: g("fMove2HRPenalty", d.two_hand_rifle),
            two_hand_big: g("fMove2HBigPenalty", d.two_hand_big),
            one_leg: g("fMoveOneCrippledLegSpeedMult", d.one_leg),
            two_legs: g("fMoveTwoCrippledLegsSpeedMult", d.two_legs),
            jump_height: g("fJumpHeightMin", d.jump_height),
            jump_swimming_mult: g("fJumpSwimmingMult", d.jump_swimming_mult),
            grab_max_weight_running: g("fGrabMaxWeightRunning", d.grab_max_weight_running),
        }
    }

    /// The exe's own defaults (the settings' static initialisers).
    pub fn defaults() -> SpeedSettings {
        SpeedSettings {
            base: 85.0,
            run_mult: 4.0,
            sneak_mult: 0.6,
            no_weapon_mult: 1.1,
            heavy_armor: 0.2,
            medium_armor: 0.1,
            two_hand_rifle: 0.1,
            two_hand_big: 0.15,
            one_leg: 0.85,
            two_legs: 0.75,
            jump_height: 64.0,
            jump_swimming_mult: 2.0,
            grab_max_weight_running: 50.0,
        }
    }
}

/// An armour's weight class (`00514410` on its biped model's general flags,
/// `BMDT` byte 4: 0x80 heavy (`004c0bd0`), else 0x08 medium (`00514450`)).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmorClass {
    Light,
    Medium,
    Heavy,
}

impl ArmorClass {
    pub fn from_general_flags(flags: u8) -> ArmorClass {
        if flags & 0x80 != 0 {
            ArmorClass::Heavy
        } else if flags & 0x08 != 0 {
            ArmorClass::Medium
        } else {
            ArmorClass::Light
        }
    }
}

/// What the speed depends on, as `Actor::GetWalkSpeed` (Xbox PDB,
/// `00885a50`) gathers it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpeedFacts {
    /// SpeedMult (actor value 21).
    pub speed_mult: f32,
    /// Legs crippled (left and right mobility conditions at most 0,
    /// `00646800`): 0, 1 or 2.
    pub crippled_legs: u8,
    /// IgnoreCrippledLimbs (actor value 72) above 0.
    pub ignores_crippled_limbs: bool,
    /// Actor vfunc +0x358 (the player's `00954cc0`, over-encumbered; 0 for
    /// other people and creatures, `0047c850`). While it holds the legs'
    /// multiplier applies even to those ignoring crippled limbs.
    pub over_encumbered: bool,
    /// Actor vfunc +0x218: 1 for people and the player (`008d0360`), 0 for
    /// creatures (`0047c850`): only people move faster with their weapon
    /// put away.
    pub person: bool,
    /// The weight class of the armour worn on the upper body, if any.
    pub armor: Option<ArmorClass>,
    /// The current weapon's animation type (`WEAP` `DNAM` byte 0, form
    /// +0xf4), if any, and whether it's drawn (process vfunc +0x454).
    pub weapon_animation: Option<u8>,
    pub weapon_drawn: bool,
    /// Sneaking: movement flags 0x400 without 0x800 (`004997b0`).
    pub sneaking: bool,
}

impl Default for SpeedFacts {
    fn default() -> Self {
        SpeedFacts {
            speed_mult: 100.0,
            crippled_legs: 0,
            ignores_crippled_limbs: false,
            over_encumbered: false,
            person: true,
            armor: None,
            weapon_animation: None,
            weapon_drawn: false,
            sneaking: false,
        }
    }
}

/// The penalty a drawn weapon of this animation type gives (`00647d10`
/// through `00646cb0`): two-handed rifles (5) and automatics (6)
/// `fMove2HRPenalty`; "handle" weapons (8) and launchers (9)
/// `fMove2HBigPenalty`; two-handed melee (2) none, though it passes the
/// two-handed test; everything else (energy rifles, 7, included) none.
pub fn weapon_penalty(settings: &SpeedSettings, animation: u8) -> f32 {
    match animation {
        8 | 9 => settings.two_hand_big,
        5 | 6 => settings.two_hand_rifle,
        _ => 0.0,
    }
}

/// Walking speed, game units a second (`00647d10`).
///
/// SpeedMult × 0.01 × `fMoveBaseSpeed` × the legs' multiplier (one
/// crippled × 0.85, both × 0.75; × 1 when ignoring crippled limbs, unless
/// over-encumbered), × `fMoveNoWeaponMult` for a person with the weapon
/// away, × (1 − the armour's and the drawn weapon's penalties), ×
/// `fMoveSneakMult` sneaking; below 0 → 0.
///
/// Arithmetic as single-precision steps (CPU tier B, x87 at PC=24: the D3D
/// thread's control word, assumed for this caller; not run on the oracle).
// Translated from 00647d10 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn walk_speed(settings: &SpeedSettings, facts: &SpeedFacts) -> f32 {
    let mut penalty = 0.0f32;
    match facts.armor {
        Some(ArmorClass::Heavy) => penalty += settings.heavy_armor,
        Some(ArmorClass::Medium) => penalty += settings.medium_armor,
        _ => {}
    }
    if let (Some(animation), true) = (facts.weapon_animation, facts.weapon_drawn) {
        penalty += weapon_penalty(settings, animation);
    }
    let mut legs = match facts.crippled_legs {
        1 => settings.one_leg,
        2 => settings.two_legs,
        _ => 1.0,
    };
    if !facts.over_encumbered && facts.ignores_crippled_limbs {
        legs = 1.0;
    }
    // The constant is a double (`01031148`); the product is stored as a
    // float.
    let scaled = (f64::from(facts.speed_mult) * 0.01) as f32;
    let mut speed = scaled * settings.base * legs;
    if facts.person && !facts.weapon_drawn {
        speed *= settings.no_weapon_mult;
    }
    speed *= 1.0 - penalty;
    if facts.sneaking {
        speed *= settings.sneak_mult;
    }
    if speed >= 0.0 {
        speed
    } else {
        0.0
    }
}

/// Running speed before the run perks (`00647f00`): walking × `fMoveRunMult`.
// Translated from 00647f00 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn run_speed(settings: &SpeedSettings, facts: &SpeedFacts) -> f32 {
    (walk_speed(settings, facts) + 0.0) * settings.run_mult
}

/// Gathers [`SpeedFacts`] for someone from the game's state.
pub fn speed_facts(
    order: &LoadOrder,
    state: &GameState,
    who: FormId,
    sneaking: bool,
) -> SpeedFacts {
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let speed_mult = facts
        .current_actor_value(who, SPEED_MULT)
        .map_or(100.0, |v| v as f32);
    let crippled_legs = [part_av::LEFT_MOBILITY, part_av::RIGHT_MOBILITY]
        .into_iter()
        .filter(|&v| is_crippled(order, state, who, v))
        .count() as u8;
    let player = who == crate::dialogue::PLAYER_REF;
    let person = player
        || base_of(order, who)
            .and_then(|b| order.get(b))
            .map_or(true, |r| r.entry.header.kind != CREA);
    let armor = crate::impacts::worn_armour(order, state, who)
        .into_iter()
        .find_map(|item| upper_body_class(order, item));
    let weapon = crate::combat::weapon_in_hand(order, state, who);
    SpeedFacts {
        speed_mult,
        crippled_legs,
        ignores_crippled_limbs: ignores_crippled_limbs(order, state, who),
        over_encumbered: player && state.over_encumbered(order, who),
        person,
        armor,
        weapon_animation: weapon.map(|w| w.animation as u8),
        weapon_drawn: state.weapon_out.contains(&who),
        sneaking,
    }
}

/// The weight class of an armour covering the upper body, if it does.
fn upper_body_class(order: &LoadOrder, item: FormId) -> Option<ArmorClass> {
    let rr = order.get(item).filter(|r| r.entry.header.kind == ARMO)?;
    let record = rr.record().ok()?;
    let bmdt = record.get(BMDT).filter(|s| s.data.len() >= 5)?;
    let slots = u32::from_le_bytes([bmdt.data[0], bmdt.data[1], bmdt.data[2], bmdt.data[3]]);
    (slots & UPPER_BODY != 0).then(|| ArmorClass::from_general_flags(bmdt.data[4]))
}

/// Someone's speed, game units a second: [`run_speed`] then the perks'
/// "Modify Run Speed" (entry point 42, `00885bf0`) when running, else
/// [`walk_speed`] (no perk: `00885a50` applies none). (Both multiply by a
/// factor from actor vfunc +0x428 when it returns something; the player's
/// returns nothing, `00acbb70`.)
pub fn speed(
    order: &LoadOrder,
    state: &GameState,
    settings: &SpeedSettings,
    who: FormId,
    running: bool,
    sneaking: bool,
) -> f32 {
    let facts = speed_facts(order, state, who, sneaking);
    if running {
        crate::perks::apply_for(
            order,
            state,
            who,
            crate::perks::entry::MODIFY_RUN_SPEED,
            run_speed(settings, &facts),
            &[],
        )
    } else {
        walk_speed(settings, &facts)
    }
}

/// Whether the player runs (movement flag 0x200; `0093e860`, keyboard):
/// running is wanted (the Run key held, or with Always Run on not held),
/// and the player isn't over-encumbered, isn't looking down the sights
/// (process vfunc +0x404, `GetIronSights` (Xbox PDB)) and isn't carrying
/// something heavier than `fGrabMaxWeightRunning`. (Nor in process anim
/// action 7, vfunc +0x3e4 `GetAnimAction` (Xbox PDB); which action that is
/// isn't traced, and it isn't modelled.)
pub fn may_run(
    settings: &SpeedSettings,
    wanted: bool,
    over_encumbered: bool,
    iron_sights: bool,
    grabbed_weight: f32,
) -> bool {
    wanted && !over_encumbered && !iron_sights && grabbed_weight <= settings.grab_max_weight_running
}

/// Whether the Jump control may start a jump (`0093e860`): never while
/// over-encumbered (actor vfunc +0x358, the player's `00954cc0`). The
/// other tests on the way (menus, the sit state, a special idle) belong to
/// their own systems.
pub fn may_jump(over_encumbered: bool) -> bool {
    !over_encumbered
}

/// How high a jump goes (`00930640`): `fJumpHeightMin` × the actor's scale
/// (`00567400`), × `fJumpSwimmingMult` swimming (controller state 5).
// Translated from 00930640 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn jump_height(settings: &SpeedSettings, scale: f32, swimming: bool) -> f32 {
    let h = scale * settings.jump_height;
    if swimming {
        h * settings.jump_swimming_mult
    } else {
        h
    }
}

/// The jump's upward speed (the jumping state `00cd4280`): √(2 × |gravity ×
/// the controller's gravity multiplier| × height), units a second, for
/// `gravity` in units a second squared (the multiplier is 1).
pub fn jump_speed(gravity: f32, height: f32) -> f32 {
    (2.0 * gravity.abs() * height.abs()).sqrt()
}

/// The controller's air control (info +0x8c, copied to controller +0x534):
/// actor vfunc +0x2e4, 1 for everyone (`0087d6c0`).
pub const AIR_CONTROL: f32 = 1.0;
/// `bhkCharacterController::fJumpMoveMult` and `fJumpMoveBase` (Xbox PDB):
/// statics 0.3 (`011b0140`) and 0 (`01267bbc`), not the game settings of
/// the same names (nothing copies those).
pub const JUMP_MOVE_MULT: f32 = 0.3;
pub const JUMP_MOVE_BASE: f32 = 0.0;

/// The share of the gap between the wanted and the current horizontal
/// velocity the in-air state closes each controller update (`00cd3fb0`):
/// air control × 0.3 + 0, or all of it when the controller flags 0x1800
/// are set (not traced; left out). No air control (0) leaves the velocity
/// alone.
pub fn air_gain(air_control: f32) -> f32 {
    if air_control > 0.0 {
        air_control * JUMP_MOVE_MULT + JUMP_MOVE_BASE
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> SpeedSettings {
        // FalloutNV.esm's values where it sets them.
        SpeedSettings {
            base: 77.0,
            sneak_mult: 0.57,
            heavy_armor: 0.15,
            medium_armor: 0.075,
            two_hand_big: 0.1,
            ..SpeedSettings::defaults()
        }
    }

    // CPU tier B (x87 at PC=24, assumed): single-precision steps.
    #[test]
    fn walking_and_running_follow_the_games_formula() {
        let s = data();
        let f = SpeedFacts::default();
        // Weapon away: 77 × 1.1.
        assert_eq!(walk_speed(&s, &f), 77.0f32 * 1.1);
        let drawn = SpeedFacts {
            weapon_drawn: true,
            ..f
        };
        assert_eq!(walk_speed(&s, &drawn), 77.0);
        assert_eq!(run_speed(&s, &drawn), 308.0);
        // A drawn rifle: × 0.9; a launcher × 0.9 (data 0.1); melee none.
        let rifle = SpeedFacts {
            weapon_animation: Some(5),
            ..drawn
        };
        assert_eq!(walk_speed(&s, &rifle), 77.0 * (1.0 - 0.1));
        let melee = SpeedFacts {
            weapon_animation: Some(2),
            ..drawn
        };
        assert_eq!(walk_speed(&s, &melee), 77.0);
        let energy = SpeedFacts {
            weapon_animation: Some(7),
            ..drawn
        };
        assert_eq!(walk_speed(&s, &energy), 77.0);
        // Holstered, the weapon's penalty doesn't count.
        let away = SpeedFacts {
            weapon_drawn: false,
            ..rifle
        };
        assert_eq!(walk_speed(&s, &away), walk_speed(&s, &f));
        // Heavy armour and a drawn rifle add up: × (1 − 0.25).
        let heavy = SpeedFacts {
            armor: Some(ArmorClass::Heavy),
            ..rifle
        };
        assert_eq!(walk_speed(&s, &heavy), 77.0 * (1.0 - (0.15 + 0.1)));
        // Sneaking × 0.57, after the rest.
        let sneak = SpeedFacts {
            sneaking: true,
            ..drawn
        };
        assert_eq!(walk_speed(&s, &sneak), 77.0 * 0.57);
        // Creatures don't get the weapon-away bonus.
        let creature = SpeedFacts { person: false, ..f };
        assert_eq!(walk_speed(&s, &creature), 77.0);
        // SpeedMult 0: nothing; never below 0.
        let stuck = SpeedFacts {
            speed_mult: -50.0,
            ..f
        };
        assert_eq!(walk_speed(&s, &stuck), 0.0);
    }

    #[test]
    fn crippled_legs_slow_unless_ignored_and_not_over_encumbered() {
        let s = data();
        let one = SpeedFacts {
            crippled_legs: 1,
            weapon_drawn: true,
            ..SpeedFacts::default()
        };
        assert_eq!(walk_speed(&s, &one), 77.0 * 0.85);
        let two = SpeedFacts {
            crippled_legs: 2,
            ..one
        };
        assert_eq!(walk_speed(&s, &two), 77.0 * 0.75);
        let ignored = SpeedFacts {
            ignores_crippled_limbs: true,
            ..two
        };
        assert_eq!(walk_speed(&s, &ignored), 77.0);
        // Over-encumbered the game doesn't ask about ignoring them.
        let heavy = SpeedFacts {
            over_encumbered: true,
            ..ignored
        };
        assert_eq!(walk_speed(&s, &heavy), 77.0 * 0.75);
    }

    #[test]
    fn armour_class_reads_the_biped_general_flags() {
        assert_eq!(ArmorClass::from_general_flags(0x80), ArmorClass::Heavy);
        assert_eq!(ArmorClass::from_general_flags(0x88), ArmorClass::Heavy);
        assert_eq!(ArmorClass::from_general_flags(0x08), ArmorClass::Medium);
        assert_eq!(ArmorClass::from_general_flags(0x20), ArmorClass::Light);
    }

    #[test]
    fn running_and_jumping_are_refused_as_the_controls_do() {
        let s = data();
        assert!(may_run(&s, true, false, false, 0.0));
        assert!(!may_run(&s, false, false, false, 0.0));
        assert!(!may_run(&s, true, true, false, 0.0));
        assert!(!may_run(&s, true, false, true, 0.0));
        assert!(may_run(&s, true, false, false, 50.0));
        assert!(!may_run(&s, true, false, false, 50.5));
        assert!(may_jump(false));
        assert!(!may_jump(true));
    }

    #[test]
    fn jumps_rise_their_height_and_steer_at_three_tenths() {
        let s = data();
        assert_eq!(jump_height(&s, 1.0, false), 64.0);
        assert_eq!(jump_height(&s, 1.0, true), 128.0);
        let g = 686.614;
        let v = jump_speed(g, 64.0);
        // v² = 2gh: the apex 64 up.
        assert!((v * v / (2.0 * g) - 64.0).abs() < 1e-3);
        assert!((v - 296.45).abs() < 0.01, "{v}");
        assert_eq!(air_gain(AIR_CONTROL), 0.3);
        assert_eq!(air_gain(0.0), 0.0);
    }
}

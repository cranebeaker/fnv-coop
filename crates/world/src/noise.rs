//! The noise an attack makes, which detection hears (FalloutNV.exe
//! 1.4.0.525; names from the Xbox 360 prototype's PDB):
//!
//! - A sound level is 0 loud, 1 normal or 2 silent, and its value
//!   `iSoundLevelLoud` (100), `iSoundLevelNormal` (50) or
//!   `iSoundLevelSilent` (0), anything else 0
//!   (`TESObjectWEAP::GetSoundLevelValue` `00525620`,
//!   `TESCreature::GetSoundLevelValue` `005fbeb0`). A weapon's is its
//!   `VNAM`, a creature's its `NAM5`, each loaded as 1 when it's 4 or more
//!   (`0051f630`, Xbox `TESCreature::Load`) and 1 without one (Xbox
//!   `TESObjectWEAP::InitializeData`, `TESCreature::InitializeData`).
//! - An attack with a weapon (`008ba600`): a ranged one (animation types
//!   3–13, `004c0c30`) makes the weapon's level, silent when a fitted mod
//!   silences it (effect 11) and normal with effect 16
//!   (`WEAPON_MOD_EFFECT_COUNT` in the prototype, so no mod has it); any
//!   other (`00899200`) a creature's own level and anyone else's silent.
//! - The attacker keeps the value (`008bc240`, actor +0x150) with
//!   `fActorAlertSoundTimer` (5) seconds on its timer (+0x154). Each update
//!   (`00886360`) the timer loses the frame's seconds while it's above 0;
//!   once it isn't, the value is cleared.
//! - Detection reads the value (`00472380` in `008a0d10`) as the target's
//!   action noise (`world::detection::Inputs::shot_noise`).
//!
//! Not here: `00886360` scales the frame's seconds while V.A.T.S. plays
//! for one actor (`011f2250` mode 4, `011f21cc`), not traced further;
//! `00899200`'s other callers (`0087b990`, `00895110`), which may make
//! noise without a weapon.

use esm::{FormId, FourCC, LoadOrder};

use crate::combat::Weapon;
use crate::scripting::{game_setting, GameState};

const VNAM: FourCC = FourCC::new(b"VNAM");
const NAM5: FourCC = FourCC::new(b"NAM5");
const WEAP: FourCC = FourCC::new(b"WEAP");
const CREA: FourCC = FourCC::new(b"CREA");

/// The sound levels.
pub const LOUD: u32 = 0;
pub const NORMAL: u32 = 1;
pub const SILENT: u32 = 2;

/// The mod effect that makes a weapon normal (`008ba600` asks for 0x10).
const NORMAL_EFFECT: i32 = 0x10;

/// An attacker's noise: its value and the seconds left on its timer.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Noise {
    pub value: i32,
    pub timer: f32,
}

/// A sound level's value (`00525620`'s and `005fbeb0`'s switch).
pub fn level_value(order: &LoadOrder, level: u32) -> i32 {
    let s = |name: &str, default: f32| game_setting(order, name).unwrap_or(default) as i32;
    match level {
        LOUD => s("iSoundLevelLoud", 100.0),
        NORMAL => s("iSoundLevelNormal", 50.0),
        SILENT => s("iSoundLevelSilent", 0.0),
        _ => 0,
    }
}

/// A record's sound level subrecord as the loaders keep it (4 or more, or
/// none, is normal).
fn stated_level(order: &LoadOrder, form: FormId, kind: FourCC, sig: FourCC) -> Option<u32> {
    let rr = order.get(form).filter(|r| r.entry.header.kind == kind)?;
    let record = rr.record().ok()?;
    let v = record
        .get(sig)
        .filter(|s| s.data.len() >= 4)
        .map_or(NORMAL, |s| {
            u32::from_le_bytes(s.data[0..4].try_into().unwrap())
        });
    Some(if v < 4 { v } else { NORMAL })
}

/// A weapon's sound level (`VNAM`).
pub fn weapon_level(order: &LoadOrder, weapon: FormId) -> Option<u32> {
    stated_level(order, weapon, WEAP, VNAM)
}

/// A creature's sound level (`NAM5`).
pub fn creature_level(order: &LoadOrder, creature: FormId) -> Option<u32> {
    stated_level(order, creature, CREA, NAM5)
}

/// Whether a weapon is ranged (`004c0c30`: animation types 3 to 13).
pub fn is_ranged(weapon: &Weapon) -> bool {
    (3..=13).contains(&weapon.animation)
}

/// The noise of an attack with a weapon (`008ba600`, `00899200`).
pub fn attack_value(
    order: &LoadOrder,
    state: &GameState,
    attacker: FormId,
    weapon: &Weapon,
) -> i32 {
    match Some(weapon).filter(|w| is_ranged(w)) {
        Some(w) => {
            let flags = crate::weapon_mods::flags(state, attacker, w.form_id);
            let has = |e| crate::weapon_mods::has_effect(order, flags, w.form_id, e);
            let level = if has(crate::weapon_mods::effect::SILENCE) {
                SILENT
            } else if has(NORMAL_EFFECT) {
                NORMAL
            } else {
                weapon_level(order, w.form_id).unwrap_or(NORMAL)
            };
            level_value(order, level)
        }
        None => {
            let base = crate::more_functions::placed::base_now(order, state, attacker);
            match base.and_then(|b| creature_level(order, b)) {
                Some(level) => level_value(order, level),
                None => level_value(order, SILENT),
            }
        }
    }
}

/// Someone attacked with a weapon: they're heard for
/// `fActorAlertSoundTimer` seconds (`008bc240`).
pub fn attacked(order: &LoadOrder, state: &mut GameState, attacker: FormId, weapon: &Weapon) {
    let value = attack_value(order, state, attacker, weapon);
    let timer = game_setting(order, "fActorAlertSoundTimer").unwrap_or(5.0);
    state.noise.insert(attacker, Noise { value, timer });
}

/// The update (`00886360`): every timer above 0 counts down; a value whose
/// timer isn't is cleared.
pub fn update(state: &mut GameState, seconds: f32) {
    state.noise.retain(|_, n| {
        if n.timer > 0.0 {
            n.timer -= seconds;
            true
        } else {
            false
        }
    });
}

/// What detection hears from someone now (`00472380`).
pub fn value(state: &GameState, who: FormId) -> i32 {
    state.noise.get(&who).map_or(0, |n| n.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `00886360`: the timer counts down while above 0, then the value
    /// goes.
    #[test]
    fn noise_wears_off() {
        let mut state = GameState::default();
        let who = FormId(0x14);
        state.noise.insert(
            who,
            Noise {
                value: 100,
                timer: 5.0,
            },
        );
        update(&mut state, 3.0);
        assert_eq!(value(&state, who), 100);
        update(&mut state, 3.0);
        // −1 left: still heard this frame, gone the next.
        assert_eq!(value(&state, who), 100);
        update(&mut state, 0.1);
        assert_eq!(value(&state, who), 0);
    }
}

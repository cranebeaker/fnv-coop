//! `(unplaced)/run_009a0260` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BGSWorldLocation::BGSWorldLocation` (`0043a3c0`, engine map; `thiscall`, two words).
const BGS_WORLD_LOCATION_CONSTRUCT: u32 = 0x0043_a3c0;
/// `CombatState::CheckMovement` (`009a02a0`, engine map; `thiscall`, three words).
const COMBAT_STATE_CHECK_MOVEMENT: u32 = 0x009a_02a0;

// Translated from 009a0260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words): builds a `BGSWorldLocation` local from
/// `point` and `space` (`0043a3c0`, which takes only those two of the four words pushed) and
/// runs `CombatState::CheckMovement` on it with the two bytes `first` and `second`
/// (zero-extended; the words left on the stack); that call's result is passed back
/// unchanged.
pub fn fn_009a0260(
    e: &mut Engine,
    this: Ptr,
    point: u32,
    space: u32,
    first: u8,
    second: u8,
) -> Ret {
    e.with_stack(0x10, |e, location| {
        let made = e
            .call(BGS_WORLD_LOCATION_CONSTRUCT, &args![location, point, space])
            .u32();
        e.call(
            COMBAT_STATE_CHECK_MOVEMENT,
            &args![this, made, u32::from(first), u32::from(second)],
        )
    })
}

/// The registered form of `009a0260`: `this` and the four words, the result passed back
/// whole.
fn fn_009a0260_entry(e: &mut Engine, words: &[u32]) -> Ret {
    fn_009a0260(
        e,
        Ptr::new(words[0]),
        words[1],
        words[2],
        words[3] as u8,
        words[4] as u8,
    )
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![(0x009a_0260, fn_009a0260_entry as AbiFn)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_the_movement_to_a_built_location() {
        let mut e = Engine::new();
        e.register(BGS_WORLD_LOCATION_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(COMBAT_STATE_CHECK_MOVEMENT, |_, a| {
            (a[2] + a[3] + 0x40).into_ret()
        });
        e.call_log = Some(vec![]);
        let r = e.call(
            0x009a_0260,
            &args![0x7000u32, 0x10u32, 0x20u32, 0x101u32, 0x200u32],
        );
        assert_eq!(r.u32(), 0x41);
        let log = e.call_log.take().unwrap();
        let location = log[1].1[0];
        assert_eq!(
            log[1..],
            [
                (BGS_WORLD_LOCATION_CONSTRUCT, vec![location, 0x10, 0x20]),
                (COMBAT_STATE_CHECK_MOVEMENT, vec![0x7000, location, 1, 0]),
            ]
        );
    }
}

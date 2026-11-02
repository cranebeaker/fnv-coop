//! `fallout/magic/run_0080f2b0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ValueModifierEffect::GetSaveSize` (engine map, `00802d90`; `thiscall`, one word).
const VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE: u32 = 0x0080_2d90;

// Translated from 0080f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetSaveSize` slot (+0x10) of the `LimbConditionEffect` and
/// `ValueAndConditionsEffect` vtables (one shared body; `thiscall`, `RET 4`): the
/// `ValueModifierEffect` save size (`00802d90` with the same word) plus 4, as a 16-bit sum.
pub fn fn_0080f2b0(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    let base = e
        .call(VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE, &args![this, argument])
        .u16();
    base.wrapping_add(4)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_f2b0, fn_0080f2b0(Ptr, u32) -> u16)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_four_in_16_bits() {
        let mut e = Engine::new();
        e.register(VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE, |_, _| {
            0x1234_0010u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_f2b0, &[0x100, 0x200]).u32(), 0x14);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE)
            .collect();
        assert_eq!(base[0].1, vec![0x100, 0x200]);
        e.register(VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE, |_, _| {
            0xfffeu32.into_ret()
        });
        assert_eq!(e.call(0x0080_f2b0, &[0x100, 0x200]).u32(), 2);
    }
}

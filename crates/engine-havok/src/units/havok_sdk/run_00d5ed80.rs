//! `Havok SDK/run_00d5ed80` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d5ed80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 0x10`: four stack words `first`, `second`, `third`, `fourth`):
/// calls `00d5ecc0` on `this` with `(first, second, 0, third, fourth)` (the exe keeps `ECX` and
/// repushes the words, putting a 0 third).
pub fn fn_00d5ed80(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32, fourth: u32) {
    e.call(
        0x00d5_ecc0,
        &args![this, first, second, 0u32, third, fourth],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d5_ed80, fn_00d5ed80(Ptr, u32, u32, u32, u32))]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d5ed80_inserts_a_zero_third_word() {
        let mut e = Engine::new();
        e.register(0x00d5_ecc0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00d5_ed80, &args![this, 1u32, 2u32, 3u32, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x00d5_ecc0, vec![this, 1, 2, 0, 3, 4])]);
    }
}

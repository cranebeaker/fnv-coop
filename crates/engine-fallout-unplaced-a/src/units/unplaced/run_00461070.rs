//! `(unplaced)/run_00461070` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00461070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at `this + 0x128` (`ECX` only, no stack
/// words).
pub fn fn_00461070(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x128)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0046_1070, fn_00461070(Ptr) -> Ptr)]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_00461070_returns_this_plus_0x128() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0046_1070, &args![0x4000u32]).u32(), 0x4128);
    }
}

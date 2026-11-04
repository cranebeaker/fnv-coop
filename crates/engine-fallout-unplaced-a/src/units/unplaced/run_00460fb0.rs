//! `(unplaced)/run_00460fb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00460fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at `this + 0x178` (`ECX` only, no stack
/// words).
pub fn fn_00460fb0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x178)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0046_0fb0, fn_00460fb0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_00460fb0_returns_this_plus_0x178() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0046_0fb0, &args![0x4000u32]).u32(), 0x4178);
    }
}

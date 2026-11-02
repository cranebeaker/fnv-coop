//! `(unplaced)/run_00461190` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00461190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address `this + 0xE8`.
pub fn fn_00461190(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xe8)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0046_1190, fn_00461190(Ptr) -> Ptr)]
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    #[test]
    fn b020_00461190_is_this_plus_0xe8() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0046_1190, &args![0x1000u32]).u32(), 0x10e8);
    }
}

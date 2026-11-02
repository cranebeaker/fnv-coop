//! `BSHavok/run_00c9a420` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c9a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00c9a420` (unnamed in the Xbox PDB): takes one argument it does not use and answers
/// false (`XOR AL,AL`, `RET 4`).
pub fn fn_00c9a420(_e: &mut Engine, _this: u32, _unused_1: u32) -> bool {
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c9a420, fn_00c9a420(u32, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_answers_false() {
        let mut e = Engine::new();
        assert!(!e.call(0x00c9_a420, &args![0x1234u32, 1u32]).bool());
        assert_eq!(e.call(0x00c9_a420, &args![0u32, 0u32]).eax & 0xff, 0);
    }
}

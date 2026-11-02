//! `NiAnimation/run_00a38de0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a38de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method shared by many classes' vtables (`thiscall`, one stack
/// word, `RET 4`) that answers 0 without reading anything.
pub fn fn_00a38de0(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a38de0, fn_00a38de0(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a3_8de0, &args![0x1000u32, 7u32]).u32(), 0);
    }
}

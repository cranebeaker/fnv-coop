//! `(unplaced)/run_00a950e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a950e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 9 (slot +0x8C of `NiSpecularProperty`'s vtable `010a02e4`, by
/// its RTTI, and also the word at `010d7400`; `this` is not read).
pub fn fn_00a950e0(_e: &mut Engine, _this: u32) -> u32 {
    9
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_50e0, fn_00a950e0(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_nine() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a9_50e0, &[0x1000]).u32(), 9);
    }
}

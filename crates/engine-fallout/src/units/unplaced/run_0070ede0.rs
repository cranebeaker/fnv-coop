//! `(unplaced)/run_0070ede0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0070ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments read; plain `RET`): answers
/// `0x3F5`.
pub fn fn_0070ede0(_e: &mut Engine) -> u32 {
    0x3f5
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0070ede0, fn_0070ede0() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_0x3f5() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0070_ede0, &[]).u32(), 0x3f5);
    }
}

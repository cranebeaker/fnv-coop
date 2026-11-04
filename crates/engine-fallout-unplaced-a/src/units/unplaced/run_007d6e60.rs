//! `(unplaced)/run_007d6e60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte global [`fn_007d6e60`] answers.
const GLOBAL_FLAG_011A0294: u32 = 0x011a_0294;

// Translated from 007d6e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. No arguments: answers the byte global at `011a0294` in `AL` (no
/// other effect).
pub fn fn_007d6e60(e: &mut Engine) -> u8 {
    e.global(GLOBAL_FLAG_011A0294)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007d_6e60, fn_007d6e60() -> u8)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_007d6e60_answers_the_byte_global() {
        let mut e = Engine::new();
        e.map(GLOBAL_FLAG_011A0294, 4);
        assert_eq!(e.call(0x007d_6e60, &args![]).u8(), 0);
        e.set_global::<u8>(GLOBAL_FLAG_011A0294, 0x5a);
        // The neighbouring byte is not part of the result.
        e.set_global::<u8>(GLOBAL_FLAG_011A0294 + 1, 0xff);
        assert_eq!(e.call(0x007d_6e60, &args![]).u8(), 0x5a);
    }
}

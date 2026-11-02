//! `(unplaced)/run_0070ec00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The word global `0070ec00` tests.
const STATE_011D8950: u32 = 0x011d_8950;

// Translated from 0070ec00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): 1 when the word at `011d8950`
/// is non-zero, else 0 (the whole of `EAX`).
pub fn fn_0070ec00(e: &mut Engine) -> u32 {
    u32::from(e.mem.u32(STATE_011D8950) != 0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0070ec00, fn_0070ec00() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_the_word() {
        let mut e = Engine::new();
        e.map(0x011d_8000, 0x1000);
        assert_eq!(e.call(0x0070_ec00, &[]).u32(), 0);
        e.mem.set_u32(STATE_011D8950, 0x100);
        assert_eq!(e.call(0x0070_ec00, &[]).u32(), 1);
    }
}

//! `(unplaced)/run_0086d580` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The global object whose address `0086d580` returns (`011c6fc8`).
const OBJECT_011C6FC8: u32 = 0x011c_6fc8;

// Translated from 0086d580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): returns the address `011c6fc8`.
pub fn fn_0086d580(_e: &mut Engine) -> u32 {
    OBJECT_011C6FC8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0086_d580, fn_0086d580() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_global_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0086_d580, &[]).u32(), 0x011c_6fc8);
    }
}

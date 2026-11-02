//! `(unplaced)/run_00aa1d40` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte `00aa1d40` answers (`011f607c`); `NiStream` code calls it for the
/// endian decision.
const ENDIAN_FLAG: u32 = 0x011f_607c;

// Translated from 00aa1d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no stack word): answers the byte at
/// `011f607c` in `AL`.
pub fn fn_00aa1d40(e: &mut Engine) -> u8 {
    e.global::<u8>(ENDIAN_FLAG)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa1d40, fn_00aa1d40() -> u8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_global_byte() {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        assert_eq!(e.call(0x00aa1d40, &args![]).u8(), 0);
        e.set_global(ENDIAN_FLAG, 1u8);
        assert_eq!(e.call(0x00aa1d40, &args![]).u8(), 1);
    }
}

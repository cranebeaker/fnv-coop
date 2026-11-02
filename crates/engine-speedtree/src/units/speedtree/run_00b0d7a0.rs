//! `SpeedTree/run_00b0d7a0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The getter and setter of the byte at `011f8b79` (identified from the code; the engine map
//! gives them no names): the flag `CSpeedTreeRT::Compute` hands to the frond texture set-up and
//! `CIndexedGeometry` reads to decide whether the second texture coordinate is negated.

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte both functions work on.
const TEXTURE_FLAG: u32 = 0x011f_8b79;

// Translated from 00b0d7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `011f8b79` (identified from the code; `cdecl`, no arguments).
pub fn fn_00b0d7a0(e: &mut Engine) -> u8 {
    e.global::<u8>(TEXTURE_FLAG)
}

// Translated from 00b0d7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the low byte of the argument in the byte at `011f8b79` (identified from the code;
/// `cdecl`, one word).
pub fn fn_00b0d7b0(e: &mut Engine, value: u8) {
    e.set_global::<u8>(TEXTURE_FLAG, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0d7a0, fn_00b0d7a0() -> u8),
        entry!(0x00b0d7b0, fn_00b0d7b0(u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_flag_is_set_and_read() {
        let mut e = Engine::new();
        e.map(0x011f_8000, 0x1000);
        assert_eq!(e.call(0x00b0_d7a0, &args![]).u8(), 0);
        e.call(0x00b0_d7b0, &args![1u32]);
        assert_eq!(e.mem.u8(TEXTURE_FLAG), 1);
        assert_eq!(e.call(0x00b0_d7a0, &args![]).u8(), 1);
        // Only the low byte of the pushed word is stored.
        e.call(0x00b0_d7b0, &args![0x1234_5600u32]);
        assert_eq!(e.call(0x00b0_d7a0, &args![]).u8(), 0);
        assert_eq!(e.mem.u8(TEXTURE_FLAG + 1), 0);
    }
}

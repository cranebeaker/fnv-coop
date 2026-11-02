//! `fallout/misc/run_00846de0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00846de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a byte): the byte at
/// `this + 0x44 + index`.
pub fn fn_00846de0(e: &mut Engine, this: Ptr, index: u8) -> u8 {
    e.mem.u8(this
        .addr()
        .wrapping_add(0x44)
        .wrapping_add(u32::from(index)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084_6de0, fn_00846de0(Ptr, u8) -> u8)]
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    #[test]
    fn fn_00846de0_reads_the_indexed_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x150);
        e.mem.set_u8(this + 0x44 + 0x10, 0x9a);
        // Only the low byte of the index word counts.
        assert_eq!(e.call(0x0084_6de0, &args![this, 0x0110u32]).u8(), 0x9a);
    }
}

//! `fallout/audio/run_00831e00` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00831e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the word at `this + 8` is set: 1 or 0 in the whole of
/// `EAX` (`AND; NEG; SBB; NEG`). Ghidra matches the body to a library
/// function of the same code; the class is not confirmed (its caller is
/// `00831de0`).
pub fn fn_00831e00(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u32(this.addr().wrapping_add(8)) & 1 != 0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0083_1e00, fn_00831e00(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_bit_zero_of_the_third_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(12);
        e.mem.set_u32(this + 8, 0xffff_fffe);
        assert_eq!(e.call(0x0083_1e00, &args![this]).u32(), 0);
        e.mem.set_u32(this + 8, 0x8000_0001);
        assert_eq!(e.call(0x0083_1e00, &args![this]).u32(), 1);
    }
}

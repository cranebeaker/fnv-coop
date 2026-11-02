//! `fallout shared/run_00438f30` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00438f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit value at +0xe of `this`, zero-extended into `EAX` (no stack
/// word; the engine map gives it no name).
pub fn fn_00438f30(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u16(this.addr() + 0xe))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0043_8f30, fn_00438f30(Ptr) -> u32)]
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_00438f30_reads_the_word_at_e() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0xbeef_ffff);
        assert_eq!(e.call(0x0043_8f30, &args![this]).u32(), 0xbeef);
    }
}

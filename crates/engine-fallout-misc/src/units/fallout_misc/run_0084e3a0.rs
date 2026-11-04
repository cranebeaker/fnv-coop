//! `fallout/misc/run_0084e3a0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0084e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0xc` of `this` (a plain field getter; the Xbox PDB map names no class for it).
pub fn fn_0084e3a0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xc)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084e3a0, fn_0084e3a0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_field_at_offset_0xc() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_u32(object.addr() + 0xc, 0x1234_5678);
        assert_eq!(e.call(0x0084_e3a0, &args![object]).u32(), 0x1234_5678);
    }
}

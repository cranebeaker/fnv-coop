//! `(unplaced)/run_00824060` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00824060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x11 (`thiscall`, returned in AL; no Xbox PDB name).
pub fn fn_00824060(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x11)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0082_4060, fn_00824060(Ptr) -> u8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_byte_at_0x11() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u8(this + 0x11, 0x5a);
        assert_eq!(e.call(0x0082_4060, &args![this]).u8(), 0x5a);
    }
}

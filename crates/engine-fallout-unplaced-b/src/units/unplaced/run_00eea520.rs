//! `(unplaced)/run_00eea520` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00eea530 (decompiled, FalloutNV.exe 1.4.0.525)
/// A getter (unnamed in the map; `thiscall`, no arguments; referenced from two vtables at
/// `010af3b4` and `010af81c`): the word at `this + 0x28`.
pub fn fn_00eea530(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x28))
}

// Translated from 00eea520 (decompiled, FalloutNV.exe 1.4.0.525)
/// An address getter (unnamed in the map; `thiscall`, no stack words): `this + 0x18`. The
/// renderer's GPU program cache calls it on an `NiMaterialResource`, where +0x18 is
/// `m_kDefaultValue` (Xbox PDB); the body itself fixes no class.
pub fn fn_00eea520(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x18))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00eea530, fn_00eea530(Ptr) -> u32),
        entry!(0x00ee_a520, fn_00eea520(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00eea530_reads_the_word_at_0x28() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 0x28, 0xabcd);
        assert_eq!(e.call(0x00eea530, &args![this]).u32(), 0xabcd);
    }

    #[test]
    fn fn_00eea520_answers_this_plus_0x18() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ee_a520, &args![0x5000u32]).u32(), 0x5018);
        assert_eq!(e.call(0x00ee_a520, &args![0xffff_fff0u32]).u32(), 0x8);
    }
}

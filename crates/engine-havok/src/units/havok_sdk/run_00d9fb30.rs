//! `Havok SDK/run_00d9fb30` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d9fb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, no stack words: stores the vtable word `0x0102e388` at `this` and nothing else (the
/// vtable the base-class destructors of this part of the exe leave behind). The class is
/// unconfirmed.
pub fn fn_00d9fb30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d9_fb30, fn_00d9fb30(Ptr))]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    #[test]
    fn the_vtable_word_is_stored() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x55);
        e.call(0x00d9_fb30, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 4), 0x55);
    }
}

//! `Havok SDK/run_00dd51d0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `hkBaseObject` (`0102e388`, RTTI `.?AVhkBaseObject@@`).
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;

// Translated from 00dd51d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall` on `this`, no stack words). Clears the word at `+8` and
/// stores the `hkBaseObject` vtable at `+0`.
pub fn fn_00dd51d0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this, BASE_OBJECT_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00dd51d0, fn_00dd51d0(u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_word_is_cleared_and_the_base_vtable_stored() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x1234);
        e.call(0x00dd_51d0, &args![this]);
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
    }
}

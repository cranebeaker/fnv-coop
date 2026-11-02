//! `fallout shared/run_004b9e00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `GridArray`'s vtable (`01020634`; the RTTI locator in the word before it
/// names `.?AVGridArray@@`).
const GRID_ARRAY_VTABLE: u32 = 0x0102_0634;

// Translated from 004b9e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridArray`'s constructor (`__thiscall`, plain `RET`): stores the
/// `GridArray` vtable at `+0` of `this` and returns `this`. It leaves the
/// rest of the object (`iCenterX`, `iCenterY`, `iDimension`, Xbox PDB
/// +4..+0xC) untouched. The map gives it no name; the class is the vtable's
/// RTTI name.
pub fn grid_array_ctor(e: &mut Engine, this: Ptr) -> Ptr {
    // GridArray vfptr (Xbox PDB) +0x00
    e.mem.set_u32(this.addr(), GRID_ARRAY_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004b9e00, grid_array_ctor(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_vtable_and_returns_this() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(this.addr() + 4, 0x11);
        e.mem.set_u32(this.addr() + 0xc, 0x22);
        let back = e.call(0x004b_9e00, &args![this]).ptr::<()>();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this.addr()), GRID_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 4), 0x11);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0x22);
    }
}

//! `SpeedTree/run_00b3f0a0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CIdvCamera`'s vtable (its class confirmed by the RTTI before it).
const IDV_CAMERA_VTABLE: u32 = 0x010a_5890;

// Translated from 00b3f0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIdvCamera`'s destructor body (unnamed in the map; called from the class's scalar
/// deleting destructor): writes the class's vtable into `this`.
pub fn fn_00b3f0a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), IDV_CAMERA_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b3f0a0, fn_00b3f0a0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00b3f0a0_writes_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.call(0x00b3f0a0, &args![this]);
        assert_eq!(e.mem.u32(this), IDV_CAMERA_VTABLE);
    }
}

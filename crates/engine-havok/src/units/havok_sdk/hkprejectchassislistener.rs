//! `Havok SDK/hkprejectchassislistener.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! None of the functions has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the second base (`+8`) is left with (a table of the pure-virtual stub), and the
/// `hkBaseObject` vtable the object ends as.
const SECOND_BASE_VTABLE: u32 = 0x010c_925c;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;

// Translated from 00e360f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRejectChassisListener::~hkpRejectChassisListener` (Xbox PDB): sets the vtable at `+8` to
/// the pure-virtual one and the one at `+0` to `hkBaseObject`'s.
pub fn hkp_reject_chassis_listener_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 8, SECOND_BASE_VTABLE);
    e.mem.set_u32(this.addr(), BASE_OBJECT_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e3_60f0,
        hkp_reject_chassis_listener_destruct(Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_destructor_leaves_the_base_vtables_00e360f0() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0xcd; 0x20]);
        e.call(0x00e3_60f0, &args![this]);
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 8), SECOND_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(this + 12), 0xcdcd_cdcd);
    }
}

//! `Havok SDK/run_00d652a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d652a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the method at vtable offset 0x10 of `this` (the shape display viewer's
/// `synchronizeTransforms`, Xbox PDB name of `00d653f0`, the one the vtable `010d1b04` holds
/// there) with the word at `this + 0x18` and `world`, the one stack word. Slot 3 of the vtable
/// `010d1b04` (RTTI: `hkpShapeDisplayViewer`).
pub fn fn_00d652a0(e: &mut Engine, this: Ptr, world: u32) {
    let handler = e.mem.u32(this.addr().wrapping_add(0x18));
    e.vcall(this.addr(), 0x10, &args![handler, world]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d6_52a0, fn_00d652a0(Ptr, u32))]
}

#[cfg(test)]
mod shape_viewer_forward_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    #[test]
    fn fn_00d652a0_forwards_the_handler_and_the_world_through_slot_0x10() {
        let mut e = Engine::new();
        e.map(0x0900_0000, 0x100);
        e.mem.set_u32(0x0900_0010, 0x0900_0021);
        let seen: Seen = Rc::default();
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0021, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 0x0900_0000);
        e.mem.set_u32(this + 0x18, 0x7777);
        e.call(0x00d6_52a0, &args![this, 0x5000u32]);
        assert_eq!(*seen.borrow(), vec![vec![this, 0x7777, 0x5000]]);
    }
}

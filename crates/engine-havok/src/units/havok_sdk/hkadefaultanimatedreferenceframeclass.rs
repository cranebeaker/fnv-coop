//! `Havok SDK/hkadefaultanimatedreferenceframeclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::{allocator, release_array};

/// The vtable the destructors of this family leave in the object (`0102e388`).
const BASE_VTABLE: u32 = 0x0102_e388;

// Translated from 00dfe520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the default animated reference frame class (thiscall, one
/// stack word `flags`; `hkaDefaultAnimatedReferenceFrame::_scalar_deleting_destructor_`, Xbox PDB):
/// empties the array of 16-byte elements at +0x34 (data, size +0x38, capacity +0x3c) through
/// the allocator, leaves the base vtable and, when bit 0 of `flags` is set, returns the block to
/// the memory router's allocator (function at +8) with the 16-bit size kept at +4. Returns
/// `this`.
pub fn hka_default_animated_reference_frame_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let me = this.addr();
    release_array(e, me + 0x34, 16);
    e.mem.set_u32(me, BASE_VTABLE);
    if flags & 1 != 0 {
        let alloc = allocator(e);
        let size = u32::from(e.mem.u16(me + 4));
        e.vcall(alloc, 8, &args![me, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_e520, hka_default_animated_reference_frame_scalar_deleting_destructor(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod destructor_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const MEMORY_ROUTER: u32 = 0x00c8_5750;
    /// The allocator object's two virtual functions (free a block, free an array).
    const FREE_BLOCK: u32 = 0x00cc_0008;
    const FREE_ARRAY: u32 = 0x00cc_0010;

    /// An engine with the memory router (and its allocator) and logging doubles at the
    /// allocator's two functions and at `others`. Returns the log and the allocator object.
    fn setup(others: &[u32]) -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        let vtable = e.mem.alloc(0x20);
        let alloc = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(alloc, vtable);
        e.mem.set_u32(vtable + 8, FREE_BLOCK);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY);
        e.mem.set_u32(router + 0x10, alloc);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [FREE_BLOCK, FREE_ARRAY]
            .iter()
            .chain(others.iter())
            .copied()
        {
            let log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        (e, log, alloc)
    }

    fn calls(log: &Log) -> Vec<(u32, Vec<u32>)> {
        log.borrow().clone()
    }

    /// Gives the array header at `header` storage for `capacity` elements of `element` bytes
    /// with 2 used; `owned` false marks the storage as not owned (top capacity bit).
    fn array(e: &mut Engine, header: u32, capacity: u32, element: u32, owned: bool) -> u32 {
        let data = e.mem.alloc(capacity * element);
        e.mem.set_u32(header, data);
        e.mem.set_u32(header + 4, 2);
        e.mem
            .set_u32(header + 8, capacity | if owned { 0 } else { 0x8000_0000 });
        data
    }

    #[test]
    fn reference_frame_destructor_frees_an_owned_array_in_sixteen_byte_elements() {
        let (mut e, log, alloc) = setup(&[]);
        let this = e.mem.alloc(0x50);
        let data = array(&mut e, this + 0x34, 5, 16, true);
        let result = hka_default_animated_reference_frame_scalar_deleting_destructor(
            &mut e,
            Ptr::new(this),
            0,
        );
        assert_eq!(result, Ptr::new(this));
        assert_eq!(calls(&log), vec![(FREE_ARRAY, vec![alloc, data, 80])]);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn reference_frame_destructor_leaves_unowned_storage_alone() {
        let (mut e, log, _) = setup(&[]);
        let this = e.mem.alloc(0x50);
        array(&mut e, this + 0x34, 5, 16, false);
        hka_default_animated_reference_frame_scalar_deleting_destructor(&mut e, Ptr::new(this), 0);
        assert!(calls(&log).is_empty());
        assert_eq!(e.mem.u32(this + 0x3c), 0x8000_0000);
    }

    #[test]
    fn reference_frame_destructor_frees_the_block_when_bit_zero_is_set() {
        let (mut e, log, alloc) = setup(&[]);
        let this = e.mem.alloc(0x50);
        e.mem.set_u16(this + 4, 0x40);
        array(&mut e, this + 0x34, 2, 16, false);
        hka_default_animated_reference_frame_scalar_deleting_destructor(&mut e, Ptr::new(this), 1);
        assert_eq!(calls(&log), vec![(FREE_BLOCK, vec![alloc, this, 0x40])]);
    }
}

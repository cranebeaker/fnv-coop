//! `Havok SDK/hkadeltacompressedanimationclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::{allocator, release_array};

/// `hkaAnimation::~hkaAnimation` (Xbox PDB; `00dfb720`, thiscall, no stack words), outside this
/// unit.
const BASE_DESTRUCTOR: u32 = 0x00df_b720;

// Translated from 00dfbb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the delta compressed animation class (thiscall, one stack word
/// `flags`; `hkaDeltaCompressedAnimation::_scalar_deleting_destructor_`, Xbox PDB): empties
/// the byte array at +0x6c (data, size +0x70, capacity +0x74) through the allocator, runs the
/// base destructor and, when bit 0 of `flags` is set, returns the block to the memory router's
/// allocator (function at +8) with the 16-bit size kept at +4. Returns `this`.
pub fn hka_delta_compressed_animation_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let me = this.addr();
    release_array(e, me + 0x6c, 1);
    e.call(BASE_DESTRUCTOR, &args![this]);
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
        entry!(0x00df_bb80, hka_delta_compressed_animation_scalar_deleting_destructor(Ptr, u8) -> Ptr),
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
    fn delta_compressed_destructor_frees_the_bytes_then_runs_the_base_destructor() {
        let (mut e, log, alloc) = setup(&[BASE_DESTRUCTOR]);
        let this = e.mem.alloc(0x80);
        let data = array(&mut e, this + 0x6c, 12, 1, true);
        let result =
            hka_delta_compressed_animation_scalar_deleting_destructor(&mut e, Ptr::new(this), 0);
        assert_eq!(result, Ptr::new(this));
        assert_eq!(
            calls(&log),
            vec![
                (FREE_ARRAY, vec![alloc, data, 12]),
                (BASE_DESTRUCTOR, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        assert_eq!(e.mem.u32(this + 0x70), 0);
        assert_eq!(e.mem.u32(this + 0x74), 0x8000_0000);
    }

    #[test]
    fn delta_compressed_destructor_does_not_free_unowned_storage() {
        let (mut e, log, _) = setup(&[BASE_DESTRUCTOR]);
        let this = e.mem.alloc(0x80);
        array(&mut e, this + 0x6c, 12, 1, false);
        hka_delta_compressed_animation_scalar_deleting_destructor(&mut e, Ptr::new(this), 0);
        assert_eq!(calls(&log), vec![(BASE_DESTRUCTOR, vec![this])]);
    }

    #[test]
    fn delta_compressed_destructor_frees_the_block_after_the_base_destructor() {
        let (mut e, log, alloc) = setup(&[BASE_DESTRUCTOR]);
        let this = e.mem.alloc(0x80);
        e.mem.set_u16(this + 4, 0x78);
        array(&mut e, this + 0x6c, 4, 1, false);
        hka_delta_compressed_animation_scalar_deleting_destructor(&mut e, Ptr::new(this), 1);
        assert_eq!(
            calls(&log),
            vec![
                (BASE_DESTRUCTOR, vec![this]),
                (FREE_BLOCK, vec![alloc, this, 0x78]),
            ]
        );
    }
}

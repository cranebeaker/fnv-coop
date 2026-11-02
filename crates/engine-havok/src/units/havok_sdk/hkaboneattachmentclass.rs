//! `Havok SDK/hkaboneattachmentclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::allocator;

/// `hkReferencedObject::removeReference` (`00c905b0`, thiscall, no stack words) and
/// `hkStringPtr::~hkStringPtr` (`00cb0c00`, thiscall, no stack words).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
const STRING_DESTRUCT: u32 = 0x00cb_0c00;
/// The vtable the destructors of this family leave in the object (`0102e388`).
const BASE_VTABLE: u32 = 0x0102_e388;

// Translated from 00dfe620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the bone attachment class (thiscall, one stack word `flags`;
/// `hkaBoneAttachment::_scalar_deleting_destructor_`, Xbox PDB): destroys the string pointer at +0x54,
/// releases and clears the reference at +0x50, destroys the string pointer at +8, leaves the
/// base vtable and, when bit 0 of `flags` is set, returns the block to the memory router's
/// allocator (function at +8) with the 16-bit size kept at +4. Returns `this`.
pub fn hka_bone_attachment_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let me = this.addr();
    e.call(STRING_DESTRUCT, &args![me + 0x54]);
    let attached = e.mem.u32(me + 0x50);
    if attached != 0 {
        e.call(REMOVE_REFERENCE, &args![attached]);
    }
    e.mem.set_u32(me + 0x50, 0);
    e.call(STRING_DESTRUCT, &args![me + 8]);
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
    vec![entry!(0x00df_e620, hka_bone_attachment_scalar_deleting_destructor(Ptr, u8) -> Ptr)]
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

    #[test]
    fn bone_attachment_destructor_releases_everything_in_order() {
        let (mut e, log, _) = setup(&[REMOVE_REFERENCE, STRING_DESTRUCT]);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x50, 0x7777);
        let result = hka_bone_attachment_scalar_deleting_destructor(&mut e, Ptr::new(this), 0);
        assert_eq!(result, Ptr::new(this));
        assert_eq!(
            calls(&log),
            vec![
                (STRING_DESTRUCT, vec![this + 0x54]),
                (REMOVE_REFERENCE, vec![0x7777]),
                (STRING_DESTRUCT, vec![this + 8]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x50), 0);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn bone_attachment_destructor_skips_a_missing_reference() {
        let (mut e, log, _) = setup(&[REMOVE_REFERENCE, STRING_DESTRUCT]);
        let this = e.mem.alloc(0x60);
        hka_bone_attachment_scalar_deleting_destructor(&mut e, Ptr::new(this), 0xfe);
        assert_eq!(
            calls(&log),
            vec![
                (STRING_DESTRUCT, vec![this + 0x54]),
                (STRING_DESTRUCT, vec![this + 8]),
            ]
        );
    }

    #[test]
    fn bone_attachment_destructor_frees_the_block_when_bit_zero_is_set() {
        let (mut e, log, alloc) = setup(&[REMOVE_REFERENCE, STRING_DESTRUCT]);
        let this = e.mem.alloc(0x60);
        e.mem.set_u16(this + 4, 0x58);
        let result = hka_bone_attachment_scalar_deleting_destructor(&mut e, Ptr::new(this), 0x03);
        assert_eq!(result, Ptr::new(this));
        assert_eq!(
            calls(&log).last(),
            Some(&(FREE_BLOCK, vec![alloc, this, 0x58]))
        );
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }
}

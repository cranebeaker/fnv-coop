//! `Havok SDK/hkameshbinding.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaMeshBinding` (Xbox PDB name) is not in the Xbox PDB's type list, so its fields are read
//! off the code: `+8` and `+0x10` two referenced-object pointers, `+0xc` an `hkStringPtr`,
//! `+0x14` an `hkArray` (data, size `+0x18`, capacity word `+0x1c`) of 12-byte `hkArray`s of
//! 16-bit values, and `+0x20` an `hkArray` (size `+0x24`, capacity word `+0x28`) of 64-byte
//! entries. Nothing here has an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator, whose vtable offset
/// `0x10` frees an array buffer `(data, bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const SLOT_FREE_ARRAY: u32 = 0x10;
/// `hkReferencedObject` remove reference (`00c905b0`, `this` only) and `hkStringPtr::~hkStringPtr`
/// (Xbox PDB name, `00cb0c00`, `this` only).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
const STRING_PTR_DESTRUCTOR: u32 = 0x00cb_0c00;
/// The vtable the destructor writes first (`010eb770`) and the base one it ends with
/// (`0102e388`).
const MESH_BINDING_VTABLE: u32 = 0x010e_b770;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The top bit of a capacity word marks storage that is not owned; the rest is the capacity.
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// The allocator of the thread's memory router.
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

// Translated from 00e34820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaMeshBinding::~hkaMeshBinding` (Xbox PDB), `this` = the object. Writes the vtable, gives
/// the array at `+0x20` back to the allocator when it is owned (`capacity * 64` bytes; its size
/// is cleared first), then does the same for every 12-byte array of the array at `+0x14`
/// (`capacity * 2` bytes each, each asking the router for its allocator again; the outer
/// array's allocator was asked for once before) and for that array itself (`capacity * 12`
/// bytes). Every array ends empty and not owned. The referenced objects at `+0x10` and `+8` are
/// released if set (and cleared), the `hkStringPtr` at `+0xc` is destroyed in between, and the
/// base vtable ends it.
pub fn hka_mesh_binding_destructor(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, MESH_BINDING_VTABLE);
    let allocator = router_allocator(e);
    let flags = e.mem.u32(t + 0x28);
    e.mem.set_u32(t + 0x24, 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(t + 0x20);
        e.vcall(
            allocator,
            SLOT_FREE_ARRAY,
            &args![data, (flags & CAPACITY_MASK) << 6],
        );
    }
    e.mem.set_u32(t + 0x20, 0);
    e.mem.set_u32(t + 0x28, NOT_OWNED);
    let outer_allocator = router_allocator(e);
    let count = e.mem.u32(t + 0x18) as i32;
    let items = e.mem.u32(t + 0x14);
    if count > 0 {
        let mut item = items.wrapping_add(8);
        for _ in 0..count {
            let allocator = router_allocator(e);
            let flags = e.mem.u32(item);
            e.mem.set_u32(item.wrapping_sub(4), 0);
            if (flags as i32) >= 0 {
                let data = e.mem.u32(item.wrapping_sub(8));
                e.vcall(
                    allocator,
                    SLOT_FREE_ARRAY,
                    &args![data, (flags & CAPACITY_MASK).wrapping_mul(2)],
                );
            }
            e.mem.set_u32(item.wrapping_sub(8), 0);
            e.mem.set_u32(item, NOT_OWNED);
            item = item.wrapping_add(12);
        }
    }
    let flags = e.mem.u32(t + 0x1c);
    e.mem.set_u32(t + 0x18, 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(t + 0x14);
        e.vcall(
            outer_allocator,
            SLOT_FREE_ARRAY,
            &args![data, (flags & CAPACITY_MASK).wrapping_mul(12)],
        );
    }
    e.mem.set_u32(t + 0x14, 0);
    e.mem.set_u32(t + 0x1c, NOT_OWNED);
    let object = e.mem.u32(t + 0x10);
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
    e.mem.set_u32(t + 0x10, 0);
    e.call(STRING_PTR_DESTRUCTOR, &args![t + 0xc]);
    let object = e.mem.u32(t + 8);
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e3_4820, hka_mesh_binding_destructor(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const FREE_ARRAY_DOUBLE: u32 = 0x0900_0023;

    /// An engine with a router whose allocator logs `(0x10, words)` for frees, and logging
    /// `removeReference` and string destructor doubles.
    fn fixture() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(FREE_ARRAY_DOUBLE, move |_, a| {
            l.borrow_mut().push((0x10, a[1..].to_vec()));
            Ret::default()
        });
        for address in [REMOVE_REFERENCE, STRING_PTR_DESTRUCTOR] {
            let l = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                l.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        (e, log)
    }

    #[test]
    fn the_destructor_frees_the_nested_arrays_and_releases_the_objects() {
        let (mut e, log) = fixture();
        let this = e.mem.alloc(0x30);
        // The 64-byte entries: capacity 2.
        let entries = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x20, entries);
        e.mem.set_u32(this + 0x24, 1);
        e.mem.set_u32(this + 0x28, 2);
        // Three 12-byte arrays: the first owned (capacity 5), the second not, the third owned
        // (capacity 3); the outer array has capacity 4.
        let inner = e.mem.alloc(0x30);
        let buffers = [e.mem.alloc(16), e.mem.alloc(16), e.mem.alloc(16)];
        for (index, (flags, size)) in [(5u32, 2u32), (NOT_OWNED | 7, 1), (3, 3)]
            .into_iter()
            .enumerate()
        {
            let at = inner + 12 * index as u32;
            e.mem.set_u32(at, buffers[index]);
            e.mem.set_u32(at + 4, size);
            e.mem.set_u32(at + 8, flags);
        }
        e.mem.set_u32(this + 0x14, inner);
        e.mem.set_u32(this + 0x18, 3);
        e.mem.set_u32(this + 0x1c, 4);
        e.mem.set_u32(this + 0x10, 0xaaaa);
        e.mem.set_u32(this + 8, 0xbbbb);
        e.call(0x00e3_4820, &args![this]);
        assert_eq!(
            log.borrow()[..],
            [
                (0x10, vec![entries, 2 << 6]),
                (0x10, vec![buffers[0], 10]),
                (0x10, vec![buffers[2], 6]),
                (0x10, vec![inner, 48]),
                (REMOVE_REFERENCE, vec![0xaaaa]),
                (STRING_PTR_DESTRUCTOR, vec![this + 0xc]),
                (REMOVE_REFERENCE, vec![0xbbbb]),
            ]
        );
        for (array, size) in [(0x14, 0x18), (0x20, 0x24)] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + size), 0);
            assert_eq!(e.mem.u32(this + size + 4), NOT_OWNED);
        }
        for index in 0..3 {
            let at = inner + 12 * index;
            assert_eq!(e.mem.u32(at), 0);
            assert_eq!(e.mem.u32(at + 4), 0);
            assert_eq!(e.mem.u32(at + 8), NOT_OWNED);
        }
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn empty_unowned_arrays_and_missing_objects_do_nothing_more_than_reset() {
        let (mut e, log) = fixture();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x1c, NOT_OWNED);
        e.mem.set_u32(this + 0x28, NOT_OWNED);
        e.call(0x00e3_4820, &args![this]);
        assert_eq!(
            log.borrow()[..],
            [(STRING_PTR_DESTRUCTOR, vec![this + 0xc])]
        );
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }
}

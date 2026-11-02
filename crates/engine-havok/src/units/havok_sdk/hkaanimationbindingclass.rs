//! `Havok SDK/hkaanimationbindingclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dfea70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_e9e0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dfea70(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_e9e0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

use super::hkaabbtree::release_array;

/// `hkReferencedObject::removeReference` (`00c905b0`, thiscall, no stack words) and
/// `hkStringPtr::~hkStringPtr` (`00cb0c00`, thiscall, no stack words).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
const STRING_DESTRUCT: u32 = 0x00cb_0c00;
/// The vtable the destructors of this family leave in the object (`0102e388`).
const BASE_VTABLE: u32 = 0x0102_e388;

// Translated from 00dfe9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaAnimationBinding::~hkaAnimationBinding` (Xbox PDB), thiscall: empties the two arrays of
/// 16-bit values at +0x1c (data, size +0x20, capacity +0x24) and at +0x10 (+0x14, +0x18) through
/// the allocator (storage freed only when the array owns it), releases the animation reference
/// at +0xc, destroys the string pointer at +8 and leaves the base vtable.
pub fn hka_animation_binding_destructor(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    release_array(e, me + 0x1c, 2);
    release_array(e, me + 0x10, 2);
    let animation = e.mem.u32(me + 0xc);
    if animation != 0 {
        e.call(REMOVE_REFERENCE, &args![animation]);
    }
    e.mem.set_u32(me + 0xc, 0);
    e.call(STRING_DESTRUCT, &args![me + 8]);
    e.mem.set_u32(me, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_ea70, deleting_destructor_00dfea70(Ptr, u8) -> Ptr),
        entry!(0x00df_e9e0, hka_animation_binding_destructor(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00dfea70_frees_only_when_bit_zero_is_set() {
        check(0x00df_ea70, 0x00df_e9e0);
    }
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
    fn binding_destructor_frees_owned_arrays_and_releases_the_animation() {
        let (mut e, log, alloc) = setup(&[REMOVE_REFERENCE, STRING_DESTRUCT]);
        let this = e.mem.alloc(0x40);
        let tracks = array(&mut e, this + 0x1c, 4, 2, true);
        array(&mut e, this + 0x10, 4, 2, false);
        e.mem.set_u32(this + 0xc, 0x5000);
        hka_animation_binding_destructor(&mut e, Ptr::new(this));
        // The owned array's storage goes back with its capacity in bytes; the other one's does
        // not; then the reference, then the string.
        assert_eq!(
            calls(&log),
            vec![
                (FREE_ARRAY, vec![alloc, tracks, 8]),
                (REMOVE_REFERENCE, vec![0x5000]),
                (STRING_DESTRUCT, vec![this + 8]),
            ]
        );
        for header in [this + 0x1c, this + 0x10] {
            assert_eq!(e.mem.u32(header), 0);
            assert_eq!(e.mem.u32(header + 4), 0);
            assert_eq!(e.mem.u32(header + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn binding_destructor_skips_a_missing_animation_and_frees_the_other_array() {
        let (mut e, log, alloc) = setup(&[REMOVE_REFERENCE, STRING_DESTRUCT]);
        let this = e.mem.alloc(0x40);
        array(&mut e, this + 0x1c, 4, 2, false);
        let floats = array(&mut e, this + 0x10, 3, 2, true);
        hka_animation_binding_destructor(&mut e, Ptr::new(this));
        assert_eq!(
            calls(&log),
            vec![
                (FREE_ARRAY, vec![alloc, floats, 6]),
                (STRING_DESTRUCT, vec![this + 8]),
            ]
        );
    }
}

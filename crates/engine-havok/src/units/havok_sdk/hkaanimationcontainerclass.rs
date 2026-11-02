//! `Havok SDK/hkaanimationcontainerclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dfe940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_e720`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dfe940(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_e720, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

use super::hkaabbtree::{allocator, CAPACITY_MASK};

/// `hkReferencedObject::removeReference` (`00c905b0`, thiscall, no stack words).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The vtable the destructors of this family leave in the object (`0102e388`).
const BASE_VTABLE: u32 = 0x0102_e388;

/// Empties an array of object pointers (data, size at +4, capacity at +8): releases the
/// reference of every non-null element and clears the slot, sets the size to 0, frees the
/// storage through the allocator (the function at +0x10, with the capacity in bytes) when the
/// array owns it, and resets the array to empty. The allocator is looked up once, before the
/// elements are released.
fn release_reference_array(e: &mut Engine, array: u32) {
    let alloc = allocator(e);
    let count = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    for index in 0..count.max(0) as u32 {
        let slot = data.wrapping_add(4 * index);
        let object = e.mem.u32(slot);
        if object != 0 {
            e.call(REMOVE_REFERENCE, &args![object]);
        }
        e.mem.set_u32(slot, 0);
    }
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            alloc,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

// Translated from 00dfe720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the container class of this unit (thiscall, no stack words; the class name is
/// not confirmed by a PDB name): releases and empties the five arrays of object pointers at
/// +0x38, +0x2c, +0x20, +0x14 and +0x8 (each: data, size, capacity), in that order, and leaves
/// the base vtable.
pub fn fn_00dfe720(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    for offset in [0x38, 0x2c, 0x20, 0x14, 0x8] {
        release_reference_array(e, me + offset);
    }
    e.mem.set_u32(me, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_e940, deleting_destructor_00dfe940(Ptr, u8) -> Ptr),
        entry!(0x00df_e720, fn_00dfe720(Ptr)),
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
    fn deleting_destructor_00dfe940_frees_only_when_bit_zero_is_set() {
        check(0x00df_e940, 0x00df_e720);
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

    /// An array at `header` holding the given object pointers in storage for `capacity`
    /// elements; `owned` false marks the storage as not owned.
    fn objects(e: &mut Engine, header: u32, held: &[u32], capacity: u32, owned: bool) -> u32 {
        let data = e.mem.alloc(capacity * 4);
        for (index, object) in held.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *object);
        }
        e.mem.set_u32(header, data);
        e.mem.set_u32(header + 4, held.len() as u32);
        e.mem
            .set_u32(header + 8, capacity | if owned { 0 } else { 0x8000_0000 });
        data
    }

    #[test]
    fn container_destructor_releases_the_five_arrays_in_order() {
        let (mut e, log, alloc) = setup(&[REMOVE_REFERENCE]);
        let this = e.mem.alloc(0x50);
        let skins = objects(&mut e, this + 0x38, &[0x9001], 2, true);
        let attachments = objects(&mut e, this + 0x2c, &[], 2, true);
        let bindings = objects(&mut e, this + 0x20, &[0x7001, 0, 0x7002], 4, true);
        objects(&mut e, this + 0x14, &[0x6001], 3, false);
        let skeletons = objects(&mut e, this + 0x8, &[0, 0x5001], 5, true);
        fn_00dfe720(&mut e, Ptr::new(this));
        // Owned storage is freed with its capacity in bytes (capacity * 4), also when the
        // array is empty; the array that does not own its storage is not freed.
        assert_eq!(
            calls(&log),
            vec![
                (REMOVE_REFERENCE, vec![0x9001]),
                (FREE_ARRAY, vec![alloc, skins, 8]),
                (FREE_ARRAY, vec![alloc, attachments, 8]),
                (REMOVE_REFERENCE, vec![0x7001]),
                (REMOVE_REFERENCE, vec![0x7002]),
                (FREE_ARRAY, vec![alloc, bindings, 16]),
                (REMOVE_REFERENCE, vec![0x6001]),
                (REMOVE_REFERENCE, vec![0x5001]),
                (FREE_ARRAY, vec![alloc, skeletons, 20]),
            ]
        );
        for offset in [0x38, 0x2c, 0x20, 0x14, 0x8] {
            assert_eq!(e.mem.u32(this + offset), 0);
            assert_eq!(e.mem.u32(this + offset + 4), 0);
            assert_eq!(e.mem.u32(this + offset + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn container_destructor_clears_the_slots_it_releases() {
        let (mut e, _log, _) = setup(&[REMOVE_REFERENCE]);
        let this = e.mem.alloc(0x50);
        let data = objects(&mut e, this + 0x38, &[0x9001, 0x9002], 2, false);
        fn_00dfe720(&mut e, Ptr::new(this));
        assert_eq!((e.mem.u32(data), e.mem.u32(data + 4)), (0, 0));
    }

    #[test]
    fn container_destructor_ignores_negative_sizes() {
        let (mut e, log, _) = setup(&[REMOVE_REFERENCE]);
        let this = e.mem.alloc(0x50);
        for offset in [0x38, 0x2c, 0x20, 0x14, 0x8] {
            objects(&mut e, this + offset, &[], 1, false);
            e.mem.set_u32(this + offset + 4, 0xffff_fffe);
        }
        fn_00dfe720(&mut e, Ptr::new(this));
        assert!(calls(&log).is_empty());
    }
}

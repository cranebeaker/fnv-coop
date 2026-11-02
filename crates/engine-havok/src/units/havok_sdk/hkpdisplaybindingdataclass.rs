//! `Havok SDK/hkpdisplaybindingdataclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dff810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_f6a0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dff810(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_f6a0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dff840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_f730`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dff840(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_f730, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkReferencedObject` reference release (`00c905b0`, `this` in `ECX`, nothing on the stack).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The vtable the destructors leave in the object.
const BASE_VTABLE: u32 = 0x0102_e388;
/// The `hkArray` empty capacity word (no storage, not owned).
const EMPTY_CAPACITY: u32 = 0x8000_0000;

/// Releases a reference held in the word at `address` (when it is not 0) and clears the word.
fn release_slot(e: &mut Engine, address: u32) {
    let object = e.mem.u32(address);
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
    e.mem.set_u32(address, 0);
}

/// The allocator behind the memory router (`00c85750`, its `+0x10`).
fn allocator_of_router(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Releases every reference an `hkArray` of object pointers holds (data at `array`, size at
/// `+4`, capacity word at `+8`), empties it and frees its storage (allocator slot `0x10`, data
/// and capacity times 4 bytes) unless bit 31 of the capacity word says it is not owned. The
/// size is read once, before the loop.
fn release_pointer_array(e: &mut Engine, array: u32) {
    let allocator = allocator_of_router(e);
    let count = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    for index in 0..count.max(0) as u32 {
        release_slot(e, data + 4 * index);
    }
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity as i32 >= 0 {
        let storage = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![storage, (capacity & 0x3fff_ffff) * 4],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, EMPTY_CAPACITY);
}

// Translated from 00dff640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDisplayBindingData::RigidBody::_scalar_deleting_destructor_` (Ghidra name): `ECX` =
/// object, one stack word of flags. Releases the references held at `+0xC` and `+8`
/// (clearing both words), sets the vtable `0102e388` and, when bit 0 of the flags is set,
/// returns the block to the memory router's allocator (slot `8`) with the 16-bit size kept at
/// `+4`. Returns `this`.
pub fn hkp_display_binding_data_rigid_body_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let me = this.addr();
    release_slot(e, me + 0xc);
    release_slot(e, me + 8);
    e.mem.set_u32(me, BASE_VTABLE);
    if flags & 1 != 0 {
        let allocator = allocator_of_router(e);
        let size = u32::from(e.mem.u16(me + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dff6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor `deleting_destructor_00dff810` runs (no name in the engine map; `ECX` =
/// object, nothing on the stack; the class is unconfirmed). Releases the reference at `+0x14`
/// (clearing the word), then the references of the array at `+8` (size `+0xC`, capacity word
/// `+0x10`) and frees its storage, and sets the vtable `0102e388`.
pub fn fn_00dff6a0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    release_slot(e, me + 0x14);
    release_pointer_array(e, me + 8);
    e.mem.set_u32(me, BASE_VTABLE);
}

// Translated from 00dff730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor `deleting_destructor_00dff840` runs (no name in the engine map; `ECX` =
/// object, nothing on the stack; the class is unconfirmed). Releases the references of the array
/// at `+0x14` (size `+0x18`, capacity word `+0x1c`) and frees its storage, does the same for the
/// array at `+8` (size `+0xC`, capacity word `+0x10`), and sets the vtable `0102e388`.
pub fn fn_00dff730(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    release_pointer_array(e, me + 0x14);
    release_pointer_array(e, me + 8);
    e.mem.set_u32(me, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_f810, deleting_destructor_00dff810(Ptr, u8) -> Ptr),
        entry!(0x00df_f840, deleting_destructor_00dff840(Ptr, u8) -> Ptr),
        entry!(
            0x00df_f640,
            hkp_display_binding_data_rigid_body_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(0x00df_f6a0, fn_00dff6a0(Ptr)),
        entry!(0x00df_f730, fn_00dff730(Ptr)),
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
    fn deleting_destructor_00dff810_frees_only_when_bit_zero_is_set() {
        check(0x00df_f810, 0x00df_f6a0);
    }

    #[test]
    fn deleting_destructor_00dff840_frees_only_when_bit_zero_is_set() {
        check(0x00df_f840, 0x00df_f730);
    }

    /// What the doubles saw, in order: a tag and the argument words.
    type Events = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// Doubles over the reference release and a memory router whose allocator logs its slots
    /// `8` (free an object: block, size) and `0x10` (free storage: data, bytes).
    fn watch(e: &mut Engine) -> Events {
        let events = Events::default();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, 0x0900_0008);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for (address, tag) in [
            (0x0900_0008, "free object"),
            (0x0900_0010, "free storage"),
            (REMOVE_REFERENCE, "release"),
        ] {
            let sink = Rc::clone(&events);
            e.register_double(address, move |_, a| {
                let words = if address == REMOVE_REFERENCE {
                    a.to_vec()
                } else {
                    a[1..].to_vec()
                };
                sink.borrow_mut().push((tag, words));
                Ret::default()
            });
        }
        events
    }

    #[test]
    fn rigid_body_destructor_releases_both_references_and_frees_on_request() {
        let mut e = engine();
        let events = watch(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u16(this + 4, 0x20);
        e.mem.set_u32(this + 8, 0xb0);
        e.mem.set_u32(this + 0xc, 0xa0);
        let returned = e.call(0x00df_f640, &args![this, 1u32]).u32();
        assert_eq!(returned, this);
        assert_eq!(
            *events.borrow(),
            vec![
                ("release", vec![0xa0]),
                ("release", vec![0xb0]),
                ("free object", vec![this, 0x20]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // Null references are skipped and without bit 0 nothing is freed.
        events.borrow_mut().clear();
        e.call(0x00df_f640, &args![this, 0xffff_fffeu32]);
        assert!(events.borrow().is_empty());
    }

    /// An `hkArray` of object pointers at `array`: `entries` in a block with capacity word
    /// `capacity`.
    fn pointer_array(e: &mut Engine, array: u32, entries: &[u32], capacity: u32) -> u32 {
        let data = e.mem.alloc(4 * entries.len() as u32 + 4);
        for (slot, entry) in entries.iter().enumerate() {
            e.mem.set_u32(data + 4 * slot as u32, *entry);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, entries.len() as u32);
        e.mem.set_u32(array + 8, capacity);
        data
    }

    #[test]
    fn destructor_00dff6a0_releases_the_reference_and_the_array() {
        let mut e = engine();
        let events = watch(&mut e);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, 0xc0);
        let data = pointer_array(&mut e, this + 8, &[0xd0, 0, 0xd1], 4);
        e.call(0x00df_f6a0, &args![this]);
        assert_eq!(
            *events.borrow(),
            vec![
                ("release", vec![0xc0]),
                ("release", vec![0xd0]),
                ("release", vec![0xd1]),
                ("free storage", vec![data, 16]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(data), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }

    #[test]
    fn destructor_00dff730_empties_both_arrays_and_keeps_borrowed_storage() {
        let mut e = engine();
        let events = watch(&mut e);
        let this = e.mem.alloc(0x40);
        let first = pointer_array(&mut e, this + 0x14, &[0xe0], 2);
        pointer_array(&mut e, this + 8, &[0xf0], 0x8000_0002);
        e.call(0x00df_f730, &args![this]);
        assert_eq!(
            *events.borrow(),
            vec![
                ("release", vec![0xe0]),
                ("free storage", vec![first, 8]),
                ("release", vec![0xf0]),
            ]
        );
        for array in [this + 8, this + 0x14] {
            assert_eq!(e.mem.u32(array), 0);
            assert_eq!(e.mem.u32(array + 4), 0);
            assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }
}

//! `Havok SDK/hkpshapeinfoclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// `hkStringPtr::~hkStringPtr` (Xbox PDB, `00cb0c00`) and `hkReferencedObject::removeReference`
/// (`00c905b0`).
const STRING_PTR_DESTRUCTOR: u32 = 0x00cb_0c00;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

// Translated from 00e00830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeInfo::~hkpShapeInfo` (Xbox PDB): sets the vtable, frees the 64-byte-element array
/// (data +0x1c, size +0x20, capacity and flags +0x24) through the allocator when its
/// capacity word is not negative, then destroys each `hkStringPtr` of the 4-byte-element
/// array (data +0x10, size +0x14, capacity and flags +0x18) and frees it the same way,
/// drops the reference at +8 and ends with the base vtable (`0x010e_8824`, then `0x0102_e388`).
pub fn hkp_shape_info_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010e_8824);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 0x24);
    e.mem.set_u32(base + 0x20, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(base + 0x1c);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 6]);
    }
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x24, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let count = e.mem.i32(base + 0x14);
    let strings = e.mem.u32(base + 0x10);
    if count > 0 {
        for index in 0..count as u32 {
            e.call(
                STRING_PTR_DESTRUCTOR,
                &args![strings.wrapping_add(4 * index)],
            );
        }
    }
    let capacity = e.mem.u32(base + 0x18);
    e.mem.set_u32(base + 0x14, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(base + 0x10);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) * 4]);
    }
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x18, 0x8000_0000);
    let reference = e.mem.u32(base + 8);
    if reference != 0 {
        e.call(REMOVE_REFERENCE, &args![reference]);
    }
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base, 0x0102_e388);
}

// Translated from 00e008e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_0830`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e008e0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_0830, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_0830, hkp_shape_info_destructor(Ptr)),
        entry!(0x00e0_08e0, deleting_destructor_00e008e0(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00e008e0_frees_only_when_bit_zero_is_set() {
        check(0x00e0_08e0, 0x00e0_0830);
    }

    const BLOCK_FREE_DOUBLE: u32 = 0x0900_0021;

    type EventLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Router and allocator whose slot `0x10` logs `[2, data, size]`; the string destructor
    /// logs `[1, string]` and the reference release logs `[3, object]`.
    fn install_info(e: &mut Engine) -> EventLog {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut slots = vec![0u32; 8];
        slots[4] = BLOCK_FREE_DOUBLE;
        let vtable = 0x0900_0100;
        e.put_vtable(vtable, &slots);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(BLOCK_FREE_DOUBLE, move |_, a| {
            free_log.borrow_mut().push(vec![2, a[1], a[2]]);
            Ret::default()
        });
        let string_log = Rc::clone(&log);
        e.register_double(STRING_PTR_DESTRUCTOR, move |_, a| {
            string_log.borrow_mut().push(vec![1, a[0]]);
            Ret::default()
        });
        let release_log = Rc::clone(&log);
        e.register_double(REMOVE_REFERENCE, move |_, a| {
            release_log.borrow_mut().push(vec![3, a[0]]);
            Ret::default()
        });
        log
    }

    #[test]
    fn shape_info_destructor_frees_both_arrays_and_destroys_the_strings() {
        let mut e = engine();
        let log = install_info(&mut e);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, 0x7000);
        e.mem.set_u32(this + 0x10, 0x5000);
        e.mem.set_u32(this + 0x14, 2);
        e.mem.set_u32(this + 0x18, 0x4000_0005);
        e.mem.set_u32(this + 0x1c, 0x6000);
        e.mem.set_u32(this + 0x20, 9);
        e.mem.set_u32(this + 0x24, 0x4000_0003);
        e.call(0x00e0_0830, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![2, 0x6000, 3 << 6],
                vec![1, 0x5000],
                vec![1, 0x5004],
                vec![2, 0x5000, 20],
                vec![3, 0x7000],
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        for (offset, value) in [
            (8, 0),
            (0x10, 0),
            (0x14, 0),
            (0x18, 0x8000_0000),
            (0x1c, 0),
            (0x20, 0),
            (0x24, 0x8000_0000),
        ] {
            assert_eq!(e.mem.u32(this + offset), value, "offset {offset:#x}");
        }
    }

    #[test]
    fn shape_info_destructor_keeps_blocks_with_a_negative_capacity() {
        let mut e = engine();
        let log = install_info(&mut e);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10, 0x5000);
        e.mem.set_u32(this + 0x18, 0x8000_0004);
        e.mem.set_u32(this + 0x1c, 0x6000);
        e.mem.set_u32(this + 0x24, 0xc000_0001);
        e.call(0x00e0_0830, &args![this]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
    }
}

//! `Havok SDK/hkpconvexverticesconnectivityclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// The vtable the destructor leaves in the object.
const BASE_VTABLE: u32 = 0x0102_e388;
/// The `hkArray` empty capacity word (no storage, not owned).
const EMPTY_CAPACITY: u32 = 0x8000_0000;

/// Releases one `hkArray` (data at `array`, size at `+4`, capacity word at `+8`): the size
/// becomes 0, the data goes back to the allocator (slot `0x10`, data and byte count) unless
/// the capacity word's bit 31 says it is not owned, and the array is left empty.
fn release_array(e: &mut Engine, array: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff) * element_size;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, EMPTY_CAPACITY);
}

// Translated from 00e00dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor called by `deleting_destructor_00e00e40` (no name in the engine map;
/// `ECX` = object, nothing on the stack): releases the array of bytes at `+0x14` and the
/// array of 16-bit values at `+8` (each through the memory router's allocator) and sets the
/// vtable to `0102e388`.
pub fn fn_00e00dd0(e: &mut Engine, this: Ptr) {
    release_array(e, this.addr() + 0x14, 1);
    release_array(e, this.addr() + 8, 2);
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

// Translated from 00e00e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_0dd0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e00e40(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_0dd0, &args![this]);
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
        entry!(0x00e0_0dd0, fn_00e00dd0(Ptr)),
        entry!(0x00e0_0e40, deleting_destructor_00e00e40(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00e00e40_frees_only_when_bit_zero_is_set() {
        check(0x00e0_0e40, 0x00e0_0dd0);
    }

    #[test]
    fn destructor_releases_owned_arrays_and_keeps_borrowed_ones() {
        const FREE_16: u32 = 0x0900_0016;
        let mut e = engine();
        let frees = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_16);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&frees);
        e.register_double(FREE_16, move |_, a| {
            sink.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        // The 16-bit values at +8 are owned (capacity 5); the bytes at +0x14 are not.
        e.mem.set_u32(this + 8, 0x7000);
        e.mem.set_u32(this + 0xc, 3);
        e.mem.set_u32(this + 0x10, 5);
        e.mem.set_u32(this + 0x14, 0x7100);
        e.mem.set_u32(this + 0x18, 4);
        e.mem.set_u32(this + 0x1c, 0x8000_0006);
        e.call(0x00e0_0dd0, &args![this]);
        assert_eq!(*frees.borrow(), vec![(0x7000, 10)]);
        for (data, size, capacity) in [(8, 0xc, 0x10), (0x14, 0x18, 0x1c)] {
            assert_eq!(e.mem.u32(this + data), 0);
            assert_eq!(e.mem.u32(this + size), 0);
            assert_eq!(e.mem.u32(this + capacity), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        // The byte array is freed with its capacity as the byte count when owned.
        e.mem.set_u32(this + 0x14, 0x7200);
        e.mem.set_u32(this + 0x1c, 6);
        frees.borrow_mut().clear();
        e.call(0x00e0_0dd0, &args![this]);
        assert_eq!(*frees.borrow(), vec![(0x7200, 6)]);
    }
}

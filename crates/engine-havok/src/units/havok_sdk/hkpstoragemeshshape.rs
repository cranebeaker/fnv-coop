//! `Havok SDK/hkpstoragemeshshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d4bd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d4_bc00`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d4bd30(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d4_bc00, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d4bc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base destructor of the storage mesh shape (the class is unconfirmed; the Xbox PDB source
/// unit is `hkpstoragemeshshape.cpp`, `__fastcall`): sets the vtable `010d052c`, then empties
/// six inplace arrays from the last to the first, each one as follows: size set to 0; when the
/// capacity word's top bit is clear, the data block goes back to the memory router's
/// allocator (slot `0x10`, size = capacity masked with `0x3fffffff` times the element size);
/// data cleared and capacity word set to `0x80000000`. The arrays are (size, data, capacity
/// word at; element size): `+0x48` / `+0x44` / `+0x4c`, 2; `+0x3c` / `+0x38` / `+0x40`, 4;
/// `+0x30` / `+0x2c` / `+0x34`, 1; `+0x24` / `+0x20` / `+0x28`, 4; `+0x18` / `+0x14` /
/// `+0x1c`, 2; `+0xc` / `+8` / `+0x10`, 4. Ends with the vtable `0102e388`.
pub fn fn_00d4bc00(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010d_052c);
    for (size, element) in [
        (0x48u32, 2u32),
        (0x3c, 4),
        (0x30, 1),
        (0x24, 4),
        (0x18, 2),
        (0xc, 4),
    ] {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(base + size + 4);
        e.mem.set_u32(base + size, 0);
        if capacity & 0x8000_0000 == 0 {
            let data = e.mem.u32(base + size - 4);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(element)],
            );
        }
        e.mem.set_u32(base + size - 4, 0);
        e.mem.set_u32(base + size + 4, 0x8000_0000);
    }
    e.mem.set_u32(base, 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_bc00, fn_00d4bc00(Ptr)),
        entry!(0x00d4_bd30, deleting_destructor_00d4bd30(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00d4bd30_frees_only_when_bit_zero_is_set() {
        check(0x00d4_bd30, 0x00d4_bc00);
    }

    #[test]
    fn the_base_destructor_frees_the_arrays_that_own_their_storage() {
        let mut e = engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT_DOUBLE);
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
            free_log.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        let this = e.mem.alloc(0x60);
        // (size at, data, capacity word): the 3rd (+0x30) and 5th (+0x18) keep a negative
        // capacity word and are not freed.
        let arrays = [
            (0x48u32, 0x1000u32, 0x4000_0003u32),
            (0x3c, 0x2000, 5),
            (0x30, 0x3000, 0x8000_0007),
            (0x24, 0x4000, 0x4000_0002),
            (0x18, 0x5000, 0xc000_0004),
            (0xc, 0x6000, 1),
        ];
        for (size, data, capacity) in arrays {
            e.mem.set_u32(this + size, 9);
            e.mem.set_u32(this + size - 4, data);
            e.mem.set_u32(this + size + 4, capacity);
        }
        e.call(0x00d4_bc00, &args![this]);
        // Freed in order: 6 elements of 2, 20 bytes, 8 bytes and 4 bytes.
        assert_eq!(
            *log.borrow(),
            vec![(0x1000, 6), (0x2000, 20), (0x4000, 8), (0x6000, 4)]
        );
        for (size, _, _) in arrays {
            assert_eq!(e.mem.u32(this + size), 0);
            assert_eq!(e.mem.u32(this + size - 4), 0);
            assert_eq!(e.mem.u32(this + size + 4), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }
}

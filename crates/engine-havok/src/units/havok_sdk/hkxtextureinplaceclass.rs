//! `Havok SDK/hkxtextureinplaceclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e017a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e0_1750`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e017a0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e0_1750, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkStringPtr::~hkStringPtr` (named by Ghidra from the Xbox PDB; `this` only).
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
/// The vtable word every destructor ends with (`hkBaseObject`, `0102e388`).
const BASE_VTABLE: u32 = 0x0102_e388;

// Translated from 00e01750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxTextureInplace::~hkxTextureInplace` (Xbox PDB): destroys the `hkStringPtr` members at
/// `+0x1c` and `+0x18` (`00cb0c00`), then empties the `hkArray` of bytes at `+0xc` (data, size
/// `+0x10`, capacity word `+0x14`): the size is set to 0 and, unless the capacity word's top
/// bit says the memory is not owned, the data is returned to the memory router's allocator
/// (slot `0x10`, with the capacity in elements, `capacity & 0x3fffffff`). The data pointer is
/// cleared, the capacity word becomes `0x80000000`, and the vtable word `0102e388` is written.
pub fn hkx_texture_inplace_destruct(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(STRING_PTR_DESTRUCT, &args![base + 0x1c]);
    e.call(STRING_PTR_DESTRUCT, &args![base + 0x18]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 0x14);
    e.mem.set_u32(base + 0x10, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(base + 0xc);
        e.vcall(allocator, 0x10, &args![data, capacity & 0x3fff_ffff]);
    }
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x14, 0x8000_0000);
    e.mem.set_u32(base, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_1750, hkx_texture_inplace_destruct(Ptr)),
        entry!(0x00e0_17a0, deleting_destructor_00e017a0(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00e017a0_frees_only_when_bit_zero_is_set() {
        check(0x00e0_17a0, 0x00e0_1750);
    }

    /// The strings are destroyed, an owned data block is freed with its capacity, and the
    /// array and vtable are reset; a block marked as not owned is not freed.
    #[test]
    fn destruct_frees_owned_data_only() {
        let mut e = engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let string_log = Rc::clone(&log);
        e.register_double(STRING_PTR_DESTRUCT, move |_, a| {
            string_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
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
            free_log.borrow_mut().extend([2, a[0], a[1], a[2]]);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x0123_4560);
        e.mem.set_u32(this + 0x10, 7);
        e.mem.set_u32(this + 0x14, 0x4000_0010);
        e.call(0x00e0_1750, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                1,
                this + 0x1c,
                1,
                this + 0x18,
                2,
                allocator,
                0x0123_4560,
                0x10
            ]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0xc, 0x0123_4560);
        e.mem.set_u32(this + 0x14, 0x8000_0010);
        e.call(0x00e0_1750, &args![this]);
        assert_eq!(*log.borrow(), vec![1, this + 0x1c, 1, this + 0x18]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
    }
}

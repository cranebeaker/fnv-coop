//! `Havok SDK/hkxsparselyanimatedenumclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dfdc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_db90`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dfdc10(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_db90, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dfdcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_dc40`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dfdcf0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_dc40, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkReferencedObject::removeReference` (`00c905b0`, `thiscall`, no stack words).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

/// `hkStringPtr::~hkStringPtr` (`00cb0c00`, `thiscall`).
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;

/// The base class's vtable (`0102e388`) the destructors end with.
const BASE_VTABLE: u32 = 0x0102_e388;

/// The allocator behind the memory router (`00c85750`, its field `+0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Releases one reference field: `removeReference` when it is not null, then null.
fn release_field(e: &mut Engine, field: u32) {
    let object = e.mem.u32(field);
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
    e.mem.set_u32(field, 0);
}

/// The end of every array clear: the size at `size_field` is zeroed and, unless the capacity
/// word at `capacity_field` is negative (the don't-deallocate flag), the data at `data_field` is
/// given back (`bufFree(data, (capacity & 0x3fffffff) * entry_size)`); then the data pointer is
/// null and the capacity word `0x80000000`.
fn clear_array(
    e: &mut Engine,
    allocator: u32,
    data_field: u32,
    size_field: u32,
    capacity_field: u32,
    entry_size: u32,
) {
    let capacity = e.mem.u32(capacity_field);
    e.mem.set_u32(size_field, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(data_field);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(entry_size)],
        );
    }
    e.mem.set_u32(data_field, 0);
    e.mem.set_u32(capacity_field, 0x8000_0000);
}

// Translated from 00dfdb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxSparselyAnimatedInt::~hkxSparselyAnimatedInt` (Xbox PDB), `thiscall`, no stack words:
/// clears the array at `+0x14` (size `+0x18`, capacity word `+0x1c`) and then the array at `+8`
/// (size `+0xc`, capacity word `+0x10`), both 4 bytes an entry, each after its own fetch of the
/// memory router's allocator, and stores the base class's vtable.
pub fn hkx_sparsely_animated_int_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let allocator = router_allocator(e);
    clear_array(e, allocator, base + 0x14, base + 0x18, base + 0x1c, 4);
    let allocator = router_allocator(e);
    clear_array(e, allocator, base + 8, base + 0xc, base + 0x10, 4);
    e.mem.set_u32(base, BASE_VTABLE);
}

// Translated from 00dfdc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxEnum::~hkxEnum` (Xbox PDB), `thiscall`, no stack words: destroys the `hkStringPtr` at
/// `+4` of each entry of the array at `+8` (size `+0xc`, capacity word `+0x10`, 8 bytes an
/// entry; the memory router's allocator fetched first), clears the array and stores the base
/// class's vtable.
pub fn hkx_enum_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let allocator = router_allocator(e);
    let count = e.mem.i32(base + 0xc);
    let mut entry = e.mem.u32(base + 8).wrapping_add(4);
    let mut left = count;
    while left > 0 {
        e.call(STRING_PTR_DESTRUCT, &args![entry]);
        entry = entry.wrapping_add(8);
        left -= 1;
    }
    clear_array(e, allocator, base + 8, base + 0xc, base + 0x10, 8);
    e.mem.set_u32(base, BASE_VTABLE);
}

// Translated from 00dfdcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxSparselyAnimatedEnum::_scalar_deleting_destructor_` (Xbox PDB), `thiscall`, `RET 4`
/// (`flags`): releases and clears the reference at `+0x20`, runs `hkxSparselyAnimatedInt`'s
/// destructor (`00dfdb90`) and, when bit 0 of `flags` is set, gives the object back
/// (allocator slot `+8`, with the 16-bit size at `+4`). Returns `this`.
pub fn hkx_sparsely_animated_enum_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let base = this.addr();
    release_field(e, base + 0x20);
    hkx_sparsely_animated_int_destructor(e, this);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        let size = u32::from(e.mem.u16(base + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_dc10, deleting_destructor_00dfdc10(Ptr, u8) -> Ptr),
        entry!(0x00df_dcf0, deleting_destructor_00dfdcf0(Ptr, u8) -> Ptr),
        entry!(0x00dfdb90, hkx_sparsely_animated_int_destructor(Ptr)),
        entry!(0x00dfdc40, hkx_enum_destructor(Ptr)),
        entry!(0x00dfdcb0, hkx_sparsely_animated_enum_scalar_deleting_destructor(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00dfdc10_frees_only_when_bit_zero_is_set() {
        check(0x00df_dc10, 0x00df_db90);
    }

    #[test]
    fn deleting_destructor_00dfdcf0_frees_only_when_bit_zero_is_set() {
        check(0x00df_dcf0, 0x00df_dc40);
    }

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Registers a double at `addr` that logs its argument words and answers `answer`.
    fn log_calls(e: &mut Engine, addr: u32, answer: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let kept = log.clone();
        e.register_double(addr, move |_, a| {
            kept.borrow_mut().push(a.to_vec());
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        log
    }

    const ALLOCATOR_OBJECT: u32 = 0x0060_0000;
    const ALLOCATOR_TABLE: u32 = 0x0060_1000;
    const DELETE_DOUBLE: u32 = 0x0060_2008;
    const FREE_DOUBLE: u32 = 0x0060_2010;

    /// A memory router (`00c85750`) whose allocator logs `[block, size]` for slot `+0x10`
    /// (`bufFree`, the first log) and for slot `+8` (the second log).
    fn install_memory(e: &mut Engine) -> (Log, Log) {
        e.map(ALLOCATOR_OBJECT, 0x100);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, ALLOCATOR_OBJECT);
        e.mem.set_u32(ALLOCATOR_OBJECT, ALLOCATOR_TABLE);
        let mut table = vec![0u32; 5];
        table[2] = DELETE_DOUBLE;
        table[4] = FREE_DOUBLE;
        e.put_vtable(ALLOCATOR_TABLE, &table);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let frees: Log = Rc::new(RefCell::new(Vec::new()));
        let deletes: Log = Rc::new(RefCell::new(Vec::new()));
        let log = frees.clone();
        e.register_double(FREE_DOUBLE, move |_, a| {
            log.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        let log = deletes.clone();
        e.register_double(DELETE_DOUBLE, move |_, a| {
            log.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        (frees, deletes)
    }

    #[test]
    fn the_int_destructor_gives_both_arrays_back_four_bytes_an_entry() {
        let mut e = Engine::new();
        let (frees, _) = install_memory(&mut e);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x14, 0x3000);
        e.mem.set_u32(this + 0x18, 4);
        e.mem.set_u32(this + 0x1c, 5);
        e.mem.set_u32(this + 8, 0x4000);
        e.mem.set_u32(this + 0xc, 3);
        e.mem.set_u32(this + 0x10, 0x4000_0007);
        e.call(0x00df_db90, &args![this]);
        assert_eq!(*frees.borrow(), vec![vec![0x3000, 20], vec![0x4000, 28]]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn the_enum_destructor_destroys_the_names_of_the_entries() {
        let mut e = Engine::new();
        let (frees, _) = install_memory(&mut e);
        let strings = log_calls(&mut e, STRING_PTR_DESTRUCT, 0);
        let this = e.mem.alloc(0x30);
        let entries = e.mem.alloc(16);
        e.mem.set_u32(this + 8, entries);
        e.mem.set_u32(this + 0xc, 2);
        e.mem.set_u32(this + 0x10, 3);
        e.call(0x00df_dc40, &args![this]);
        assert_eq!(
            *strings.borrow(),
            vec![vec![entries + 4], vec![entries + 12]]
        );
        assert_eq!(*frees.borrow(), vec![vec![entries, 24]]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn the_enum_deleting_destructor_releases_then_runs_the_int_destructor() {
        let mut e = Engine::new();
        let (frees, deletes) = install_memory(&mut e);
        let removed = log_calls(&mut e, REMOVE_REFERENCE, 0);
        let this = e.mem.alloc(0x30);
        e.mem.set_u16(this + 4, 0x28);
        e.mem.set_u32(this + 0x20, 0x66);
        e.mem.set_u32(this + 0x14, 0x3000);
        e.mem.set_u32(this + 0x1c, 2);
        assert_eq!(e.call(0x00df_dcb0, &args![this, 0u8]).u32(), this);
        assert_eq!(*removed.borrow(), vec![vec![0x66]]);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(*frees.borrow(), vec![vec![0x3000, 8], vec![0, 0]]);
        assert!(deletes.borrow().is_empty());
        assert_eq!(e.call(0x00df_dcb0, &args![this, 1u8]).u32(), this);
        assert_eq!(*deletes.borrow(), vec![vec![this, 0x28]]);
    }
}

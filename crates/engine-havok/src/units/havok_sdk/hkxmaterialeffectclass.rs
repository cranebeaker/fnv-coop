//! `Havok SDK/hkxmaterialeffectclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator whose slot `+0x10`
/// (`bufFree(block, size)`) gives blocks back.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// `hkStringPtr::~hkStringPtr` (`00cb0c00`, `thiscall`).
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;

/// The base class's vtable (`0102e388`) the destructors end with.
const BASE_VTABLE: u32 = 0x0102_e388;

/// The allocator behind the memory router (`00c85750`, its field `+0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
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

// Translated from 00e022d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxMaterialEffect::_scalar_deleting_destructor_` (Xbox PDB), `thiscall`, `RET 4`
/// (`flags`): clears the array at `+0x10` (size `+0x14`, capacity word `+0x18`, one byte an
/// entry) through the memory router's allocator, destroys the `hkStringPtr` at `+8`, stores the
/// base class's vtable and, when bit 0 of `flags` is set, gives the object itself back
/// (allocator slot `+8`, with the 16-bit size at `+4`). Returns `this`.
pub fn hkx_material_effect_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let base = this.addr();
    let allocator = router_allocator(e);
    clear_array(e, allocator, base + 0x10, base + 0x14, base + 0x18, 1);
    e.call(STRING_PTR_DESTRUCT, &args![base + 8]);
    e.mem.set_u32(base, BASE_VTABLE);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        let size = u32::from(e.mem.u16(base + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e022d0, hkx_material_effect_scalar_deleting_destructor(Ptr, u8) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

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
    fn the_destructor_clears_the_bytes_and_the_name() {
        let mut e = Engine::new();
        let (frees, deletes) = install_memory(&mut e);
        let strings = log_calls(&mut e, STRING_PTR_DESTRUCT, 0);
        let this = e.mem.alloc(0x30);
        e.mem.set_u16(this + 4, 0x30);
        e.mem.set_u32(this + 0x10, 0x3000);
        e.mem.set_u32(this + 0x14, 9);
        e.mem.set_u32(this + 0x18, 0x4000_0011);
        assert_eq!(e.call(0x00e0_22d0, &args![this, 0u8]).u32(), this);
        assert_eq!(*frees.borrow(), vec![vec![0x3000, 0x11]]);
        assert_eq!(*strings.borrow(), vec![vec![this + 8]]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0x8000_0000);
        assert!(deletes.borrow().is_empty());
        // Bit 0 of the flags also gives the object back with its 16-bit size.
        assert_eq!(e.call(0x00e0_22d0, &args![this, 3u8]).u32(), this);
        assert_eq!(*deletes.borrow(), vec![vec![this, 0x30]]);
    }
}

//! `Havok SDK/hkxindexbufferclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator whose slot `+0x10`
/// (`bufFree(block, size)`) gives blocks back.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

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

// Translated from 00e023c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the destructor of an index buffer
/// class record. Clears the array of 32-bit indices (data `+0x18`, size `+0x1c`, capacity word
/// `+0x20`, 4 bytes an entry) and then the array of 16-bit indices (data `+0xc`, size `+0x10`,
/// capacity word `+0x14`, 2 bytes an entry) through the memory router's allocator, and stores
/// the base class's vtable (`0102e388`).
pub fn fn_00e023c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let allocator = router_allocator(e);
    clear_array(e, allocator, base + 0x18, base + 0x1c, base + 0x20, 4);
    let allocator = router_allocator(e);
    clear_array(e, allocator, base + 0xc, base + 0x10, base + 0x14, 2);
    e.mem.set_u32(base, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e023c0, fn_00e023c0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

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
    fn both_index_arrays_are_given_back_and_cleared() {
        let mut e = Engine::new();
        let (frees, _) = install_memory(&mut e);
        let this = e.mem.alloc(0x30);
        // 32-bit indices: data 0x1000, size 5, capacity word 3 (masked 3, flag bit 30 kept off).
        e.mem.set_u32(this + 0x18, 0x1000);
        e.mem.set_u32(this + 0x1c, 5);
        e.mem.set_u32(this + 0x20, 0x4000_0003);
        // 16-bit indices: data 0x2000, size 7, capacity word 9.
        e.mem.set_u32(this + 0xc, 0x2000);
        e.mem.set_u32(this + 0x10, 7);
        e.mem.set_u32(this + 0x14, 9);
        e.call(0x00e0_23c0, &args![this]);
        assert_eq!(*frees.borrow(), vec![vec![0x1000, 12], vec![0x2000, 18]]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn an_array_with_the_flag_is_not_given_back() {
        let mut e = Engine::new();
        let (frees, _) = install_memory(&mut e);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x18, 0x1000);
        e.mem.set_u32(this + 0x20, 0x8000_0003);
        e.mem.set_u32(this + 0xc, 0x2000);
        e.mem.set_u32(this + 0x14, 2);
        e.call(0x00e0_23c0, &args![this]);
        assert_eq!(*frees.borrow(), vec![vec![0x2000, 4]]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
    }
}

//! `Havok SDK/hkxmeshsectionclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator whose slot `+0x10`
/// (`bufFree(block, size)`) gives blocks back.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// `hkReferencedObject::removeReference` (`00c905b0`, `thiscall`, no stack words).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

/// The base class's vtable (`0102e388`) the destructors end with.
const BASE_VTABLE: u32 = 0x0102_e388;

/// The allocator behind the memory router (`00c85750`, its field `+0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Releases (`removeReference` when not null) and clears the `count` references of the array at
/// `data`, 4 bytes each.
fn release_references(e: &mut Engine, data: u32, count: i32) {
    let mut index = 0i32;
    while index < count {
        let slot = data.wrapping_add((index as u32).wrapping_mul(4));
        let object = e.mem.u32(slot);
        if object != 0 {
            e.call(REMOVE_REFERENCE, &args![object]);
        }
        e.mem.set_u32(slot, 0);
        index += 1;
    }
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

// Translated from 00e01f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): releases and clears the references
/// of the array at `+0x1c` (size `+0x20`, capacity word `+0x24`, 4 bytes an entry) and clears
/// it, releases the reference at `+0x18`, does the same for the array at `+0xc` (size `+0x10`,
/// capacity word `+0x14`), releases the reference at `+8` and stores the base class's vtable.
/// Each array has its own fetch of the memory router's allocator.
pub fn fn_00e01f70(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let allocator = router_allocator(e);
    let count = e.mem.i32(base + 0x20);
    let data = e.mem.u32(base + 0x1c);
    release_references(e, data, count);
    clear_array(e, allocator, base + 0x1c, base + 0x20, base + 0x24, 4);
    release_field(e, base + 0x18);
    let allocator = router_allocator(e);
    let count = e.mem.i32(base + 0x10);
    let data = e.mem.u32(base + 0xc);
    release_references(e, data, count);
    clear_array(e, allocator, base + 0xc, base + 0x10, base + 0x14, 4);
    release_field(e, base + 8);
    e.mem.set_u32(base, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e01f70, fn_00e01f70(Ptr))]
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
    fn both_arrays_and_both_references_are_released_in_order() {
        let mut e = Engine::new();
        let (frees, _) = install_memory(&mut e);
        let removed = log_calls(&mut e, REMOVE_REFERENCE, 0);
        let this = e.mem.alloc(0x30);
        let array_a = e.mem.alloc(8);
        e.mem.set_u32(array_a, 0xa1);
        e.mem.set_u32(array_a + 4, 0xa2);
        e.mem.set_u32(this + 0x1c, array_a);
        e.mem.set_u32(this + 0x20, 2);
        e.mem.set_u32(this + 0x24, 4);
        e.mem.set_u32(this + 0x18, 0xc1);
        let array_b = e.mem.alloc(4);
        e.mem.set_u32(array_b, 0xb1);
        e.mem.set_u32(this + 0xc, array_b);
        e.mem.set_u32(this + 0x10, 1);
        e.mem.set_u32(this + 0x14, 3);
        e.mem.set_u32(this + 8, 0xd1);
        e.call(0x00e0_1f70, &args![this]);
        assert_eq!(
            *removed.borrow(),
            vec![vec![0xa1], vec![0xa2], vec![0xc1], vec![0xb1], vec![0xd1]]
        );
        assert_eq!(*frees.borrow(), vec![vec![array_a, 16], vec![array_b, 12]]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn null_references_are_not_released() {
        let mut e = Engine::new();
        install_memory(&mut e);
        let removed = log_calls(&mut e, REMOVE_REFERENCE, 0);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x24, 0x8000_0000);
        e.mem.set_u32(this + 0x14, 0x8000_0000);
        e.call(0x00e0_1f70, &args![this]);
        assert!(removed.borrow().is_empty());
    }
}

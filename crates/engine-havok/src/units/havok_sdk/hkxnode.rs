//! `Havok SDK/hkxnode.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator whose slot `+0x10`
/// (`bufFree(block, size)`) gives blocks back.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// `hkReferencedObject::removeReference` (`00c905b0`, `thiscall`, no stack words).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

/// `hkStringPtr::~hkStringPtr` (`00cb0c00`, `thiscall`).
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;

/// `hkxAttributeHolder::~hkxAttributeHolder` (`00e3eaa0`).
const HOLDER_DESTRUCT: u32 = 0x00e3_eaa0;

/// The vtable `hkxNode` stores first (`010e311c`).
const NODE_VTABLE: u32 = 0x010e_311c;

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

/// Releases the reference at `reference_field` (`removeReference` when not null), clears it
/// and then destroys the `hkStringPtr` at `string_field`.
fn release_field_then_destroy_string(e: &mut Engine, reference_field: u32, string_field: u32) {
    release_field(e, reference_field);
    e.call(STRING_PTR_DESTRUCT, &args![string_field]);
}

// Translated from 00e3ed80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxNode::~hkxNode` (Xbox PDB), `thiscall`, no stack words: stores the vtable `010e311c`,
/// destroys the `hkStringPtr` at `+0x40`, destroys the `hkStringPtr` at `+4` of each entry of
/// the array at `+0x34` (size `+0x38`, capacity word `+0x3c`, 8 bytes an entry) and clears it,
/// releases and clears the references of the array at `+0x28` (size `+0x2c`, capacity word
/// `+0x30`, 4 bytes an entry) and clears it, clears the array at `+0x1c` (size `+0x20`,
/// capacity word `+0x24`, 64 bytes an entry), releases the reference at `+0x18`, destroys the
/// `hkStringPtr` at `+0x14` and ends in the attribute holder's destructor (`00e3eaa0`). Each
/// array has its own fetch of the memory router's allocator.
pub fn hkx_node_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NODE_VTABLE);
    e.call(STRING_PTR_DESTRUCT, &args![base + 0x40]);
    let allocator = router_allocator(e);
    let count = e.mem.i32(base + 0x38);
    let mut entry = e.mem.u32(base + 0x34).wrapping_add(4);
    let mut left = count;
    while left > 0 {
        e.call(STRING_PTR_DESTRUCT, &args![entry]);
        entry = entry.wrapping_add(8);
        left -= 1;
    }
    clear_array(e, allocator, base + 0x34, base + 0x38, base + 0x3c, 8);
    let allocator = router_allocator(e);
    let count = e.mem.i32(base + 0x2c);
    let data = e.mem.u32(base + 0x28);
    release_references(e, data, count);
    clear_array(e, allocator, base + 0x28, base + 0x2c, base + 0x30, 4);
    let allocator = router_allocator(e);
    clear_array(e, allocator, base + 0x1c, base + 0x20, base + 0x24, 64);
    release_field_then_destroy_string(e, base + 0x18, base + 0x14);
    e.call(HOLDER_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e3ed80, hkx_node_destructor(Ptr))]
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
    fn the_destructor_clears_every_member_and_ends_in_the_holder() {
        let mut e = Engine::new();
        let (frees, _) = install_memory(&mut e);
        let removed = log_calls(&mut e, REMOVE_REFERENCE, 0);
        let strings = log_calls(&mut e, STRING_PTR_DESTRUCT, 0);
        let holder = log_calls(&mut e, HOLDER_DESTRUCT, 0);
        let this = e.mem.alloc(0x50);
        // +0x34: two entries of 8 bytes (names at +4); +0x28: two references; +0x1c: 64-byte
        // entries; +0x18: a reference.
        let pairs = e.mem.alloc(16);
        e.mem.set_u32(this + 0x34, pairs);
        e.mem.set_u32(this + 0x38, 2);
        e.mem.set_u32(this + 0x3c, 0x4000_0004);
        let refs = e.mem.alloc(8);
        e.mem.set_u32(refs, 0x21);
        e.mem.set_u32(refs + 4, 0x22);
        e.mem.set_u32(this + 0x28, refs);
        e.mem.set_u32(this + 0x2c, 2);
        e.mem.set_u32(this + 0x30, 3);
        let big = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x1c, big);
        e.mem.set_u32(this + 0x20, 2);
        e.mem.set_u32(this + 0x24, 2);
        e.mem.set_u32(this + 0x18, 0x99);
        e.call(0x00e3_ed80, &args![this]);
        assert_eq!(e.mem.u32(this), NODE_VTABLE);
        assert_eq!(
            *strings.borrow(),
            vec![
                vec![this + 0x40],
                vec![pairs + 4],
                vec![pairs + 12],
                vec![this + 0x14]
            ]
        );
        assert_eq!(*removed.borrow(), vec![vec![0x21], vec![0x22], vec![0x99]]);
        assert_eq!(
            *frees.borrow(),
            vec![vec![pairs, 32], vec![refs, 12], vec![big, 128]]
        );
        for (data, size, capacity) in [(0x34, 0x38, 0x3c), (0x28, 0x2c, 0x30), (0x1c, 0x20, 0x24)] {
            assert_eq!(e.mem.u32(this + data), 0);
            assert_eq!(e.mem.u32(this + size), 0);
            assert_eq!(e.mem.u32(this + capacity), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(*holder.borrow(), vec![vec![this]]);
    }
}

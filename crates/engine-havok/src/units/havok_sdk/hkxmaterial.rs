//! `Havok SDK/hkxmaterial.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
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

/// The vtable `hkxMaterial` stores first (`010e33dc`).
const MATERIAL_VTABLE: u32 = 0x010e_33dc;

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

// Translated from 00e3f110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxMaterial::~hkxMaterial` (Xbox PDB), `thiscall`, no stack words: stores the vtable
/// `010e33dc`, releases the reference at `+0x7c`, then the references of the array at `+0x70`
/// (size `+0x74`, capacity word `+0x78`, 4 bytes an entry; the memory router's allocator
/// fetched first) and clears it, then the array at `+0x18` (size `+0x1c`, capacity word `+0x20`,
/// 12 bytes an entry whose reference is its first word, released and cleared) and clears it,
/// destroys the `hkStringPtr` at `+0x14` (`00cb0c00`) and ends in the attribute holder's
/// destructor (`00e3eaa0`).
pub fn hkx_material_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, MATERIAL_VTABLE);
    release_field(e, base + 0x7c);
    let allocator = router_allocator(e);
    let count = e.mem.i32(base + 0x74);
    let data = e.mem.u32(base + 0x70);
    release_references(e, data, count);
    clear_array(e, allocator, base + 0x70, base + 0x74, base + 0x78, 4);
    let allocator = router_allocator(e);
    let count = e.mem.i32(base + 0x1c);
    let mut entry = e.mem.u32(base + 0x18);
    let mut left = count;
    while left > 0 {
        release_field(e, entry);
        entry = entry.wrapping_add(0xc);
        left -= 1;
    }
    clear_array(e, allocator, base + 0x18, base + 0x1c, base + 0x20, 12);
    e.call(STRING_PTR_DESTRUCT, &args![base + 0x14]);
    e.call(HOLDER_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e3f110, hkx_material_destructor(Ptr))]
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
    fn the_destructor_releases_clears_and_ends_in_the_holder_destructor() {
        let mut e = Engine::new();
        let (frees, _) = install_memory(&mut e);
        let removed = log_calls(&mut e, REMOVE_REFERENCE, 0);
        let strings = log_calls(&mut e, STRING_PTR_DESTRUCT, 0);
        let holder = log_calls(&mut e, HOLDER_DESTRUCT, 0);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x7c, 0x7777);
        // The array at +0x70: two references (the second null).
        let refs = e.mem.alloc(8);
        e.mem.set_u32(refs, 0x4001);
        e.mem.set_u32(this + 0x70, refs);
        e.mem.set_u32(this + 0x74, 2);
        e.mem.set_u32(this + 0x78, 4);
        // The array at +0x18: two entries of 12 bytes whose first word is a reference.
        let entries = e.mem.alloc(0x18);
        e.mem.set_u32(entries, 0x5001);
        e.mem.set_u32(entries + 0xc, 0x5002);
        e.mem.set_u32(this + 0x18, entries);
        e.mem.set_u32(this + 0x1c, 2);
        e.mem.set_u32(this + 0x20, 6);
        e.call(0x00e3_f110, &args![this]);
        assert_eq!(e.mem.u32(this), MATERIAL_VTABLE);
        assert_eq!(
            *removed.borrow(),
            vec![vec![0x7777], vec![0x4001], vec![0x5001], vec![0x5002]]
        );
        assert_eq!(e.mem.u32(this + 0x7c), 0);
        assert_eq!(e.mem.u32(refs), 0);
        assert_eq!(e.mem.u32(entries), 0);
        assert_eq!(e.mem.u32(entries + 0xc), 0);
        assert_eq!(*frees.borrow(), vec![vec![refs, 16], vec![entries, 72]]);
        assert_eq!(e.mem.u32(this + 0x70), 0);
        assert_eq!(e.mem.u32(this + 0x78), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
        assert_eq!(*strings.borrow(), vec![vec![this + 0x14]]);
        assert_eq!(*holder.borrow(), vec![vec![this]]);
    }
}

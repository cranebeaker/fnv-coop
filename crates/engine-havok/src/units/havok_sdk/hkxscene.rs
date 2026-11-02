//! `Havok SDK/hkxscene.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
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

/// `hkStringPtr::hkStringPtr()` (`00cb0a60`, `thiscall`).
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;

/// The base class's vtable (`0102e388`) the destructors end with.
const BASE_VTABLE: u32 = 0x0102_e388;

/// The vtable `hkxScene` stores (`010e99d8`).
const SCENE_VTABLE: u32 = 0x010e_99d8;

/// The routine that answers the address of the identity matrix's three rows (`00cd4b50`,
/// cdecl; the decompiler calls it `hkMatrix3::getIdentity`, unconfirmed).
const MATRIX3_IDENTITY: u32 = 0x00cd_4b50;

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

// Translated from 00e4a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxScene::hkxScene` (Xbox PDB), `thiscall`, no stack words: sets the 16-bit word at `+6` to
/// 1 and the vtable `010e99d8`, constructs the `hkStringPtr`s at `+8` and `+0xc`, stores the
/// float `0.0` at `+0x10`, clears the reference at `+0x14`, makes the eight empty arrays at
/// `+0x18`, `+0x24`, ... `+0x6c` (data 0, size 0, capacity word `0x80000000`, 12 bytes each)
/// and copies the three 16-byte rows of the identity matrix (`00cd4b50`) to `+0x80`, `+0x90`
/// and `+0xa0`. Returns `this`.
pub fn hkx_scene_hkx_scene(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base, SCENE_VTABLE);
    e.call(STRING_PTR_CONSTRUCT, &args![base + 8]);
    e.call(STRING_PTR_CONSTRUCT, &args![base + 0xc]);
    e.mem.set_f32(base + 0x10, 0.0);
    e.mem.set_u32(base + 0x14, 0);
    for array in 0..8u32 {
        let at = base + 0x18 + 0xc * array;
        e.mem.set_u32(at, 0);
        e.mem.set_u32(at + 4, 0);
        e.mem.set_u32(at + 8, 0x8000_0000);
    }
    let identity = e.call(MATRIX3_IDENTITY, &args![]).u32();
    for row in 0..3u32 {
        for lane in 0..4u32 {
            let word = e.mem.u32(identity + 0x10 * row + 4 * lane);
            e.mem.set_u32(base + 0x80 + 0x10 * row + 4 * lane, word);
        }
    }
    this
}

// Translated from 00e4a750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxScene::~hkxScene` (Xbox PDB), `thiscall`, no stack words: stores the vtable
/// `010e99d8`; then for the eight arrays at `+0x6c`, `+0x60`, `+0x54`, `+0x48`, `+0x3c`,
/// `+0x30`, `+0x24` and `+0x18` (size 4 bytes on, capacity word 8 on; 4 bytes an entry), in
/// that order and each after its own fetch of the memory router's allocator, releases and
/// clears the references and clears the array. Then the reference at `+0x14` is released,
/// the `hkStringPtr`s at `+0xc` and `+8` destroyed and the base class's vtable stored.
pub fn hkx_scene_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SCENE_VTABLE);
    for array in (0..8u32).rev() {
        let at = base + 0x18 + 0xc * array;
        let allocator = router_allocator(e);
        let count = e.mem.i32(at + 4);
        let data = e.mem.u32(at);
        release_references(e, data, count);
        clear_array(e, allocator, at, at + 4, at + 8, 4);
    }
    release_field(e, base + 0x14);
    e.call(STRING_PTR_DESTRUCT, &args![base + 0xc]);
    e.call(STRING_PTR_DESTRUCT, &args![base + 8]);
    e.mem.set_u32(base, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4a6a0, hkx_scene_hkx_scene(Ptr) -> Ptr),
        entry!(0x00e4a750, hkx_scene_destructor(Ptr)),
    ]
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
    fn the_constructor_sets_the_defaults_and_copies_the_identity_rows() {
        let mut e = Engine::new();
        let strings = log_calls(&mut e, STRING_PTR_CONSTRUCT, 0);
        let identity = e.mem.alloc(0x30);
        for word in 0..12u32 {
            e.mem.set_u32(identity + 4 * word, 0x3000 + word);
        }
        e.register_double(MATRIX3_IDENTITY, move |_, _| Ret {
            eax: identity,
            ..Ret::default()
        });
        let this = e.mem.alloc(0xb0);
        e.mem.write(this, &[0xee; 0xb0]);
        assert_eq!(e.call(0x00e4_a6a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), SCENE_VTABLE);
        assert_eq!(*strings.borrow(), vec![vec![this + 8], vec![this + 0xc]]);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        for array in 0..8u32 {
            let at = this + 0x18 + 0xc * array;
            assert_eq!(e.mem.u32(at), 0, "data of array {array}");
            assert_eq!(e.mem.u32(at + 4), 0);
            assert_eq!(e.mem.u32(at + 8), 0x8000_0000);
        }
        // Between the last array and the rows nothing is written.
        assert_eq!(e.mem.u32(this + 0x78), 0xeeee_eeee);
        for word in 0..12u32 {
            let row = word / 4;
            assert_eq!(
                e.mem.u32(this + 0x80 + 0x10 * row + 4 * (word % 4)),
                0x3000 + word
            );
        }
    }

    #[test]
    fn the_destructor_clears_the_eight_arrays_from_the_last() {
        let mut e = Engine::new();
        let (frees, _) = install_memory(&mut e);
        let removed = log_calls(&mut e, REMOVE_REFERENCE, 0);
        let strings = log_calls(&mut e, STRING_PTR_DESTRUCT, 0);
        let this = e.mem.alloc(0xb0);
        // One reference in the first and in the last array; capacity words 2 and 3.
        let first = e.mem.alloc(4);
        e.mem.set_u32(first, 0x1a);
        e.mem.set_u32(this + 0x18, first);
        e.mem.set_u32(this + 0x1c, 1);
        e.mem.set_u32(this + 0x20, 2);
        let last = e.mem.alloc(4);
        e.mem.set_u32(last, 0x2b);
        e.mem.set_u32(this + 0x6c, last);
        e.mem.set_u32(this + 0x70, 1);
        e.mem.set_u32(this + 0x74, 3);
        e.mem.set_u32(this + 0x14, 0x77);
        e.call(0x00e4_a750, &args![this]);
        // Eight arrays are cleared (the other six have a capacity word of 0, so are "freed"
        // with size 0 from a null data pointer).
        assert_eq!(*removed.borrow(), vec![vec![0x2b], vec![0x1a], vec![0x77]]);
        let freed = frees.borrow();
        assert_eq!(freed.len(), 8);
        assert_eq!(freed[0], vec![last, 12]);
        assert_eq!(freed[7], vec![first, 8]);
        assert_eq!(*strings.borrow(), vec![vec![this + 0xc], vec![this + 8]]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x74), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }
}

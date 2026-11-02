//! `NiXenonRenderer/run_00e7a640` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::ALLOCATE;

/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00a90180` (`thiscall`, `RET 4`, engine map unit `nioldscreenspace.cpp`): sets the
/// capacity of an array.
const ARRAY_SET_SIZE: u32 = 0x00a9_0180;
/// `00ba3aa0` (`thiscall`, `RET 8`): stores the word `value` points to at an index of an array.
const ARRAY_SET_AT: u32 = 0x00ba_3aa0;
/// `00e78560` (`cdecl`, two words): sets one handler of the table being built.
const SET_HANDLER: u32 = 0x00e7_8560;
/// `0126f550`: the outer table (0x12 entries) `fn_00e7a860` builds.
const OUTER_TABLE_SLOT: u32 = 0x0126_f550;
/// The 17 words at `0126f508` that `fn_00e7a760` fills (4, 8, 0xc or 0x10).
const SIZE_TABLE: [(u32, u32); 17] = [
    (0x0126_f508, 4),
    (0x0126_f50c, 8),
    (0x0126_f510, 0xc),
    (0x0126_f514, 0x10),
    (0x0126_f518, 4),
    (0x0126_f51c, 4),
    (0x0126_f520, 4),
    (0x0126_f524, 8),
    (0x0126_f528, 4),
    (0x0126_f52c, 4),
    (0x0126_f530, 8),
    (0x0126_f534, 4),
    (0x0126_f538, 8),
    (0x0126_f53c, 4),
    (0x0126_f540, 4),
    (0x0126_f544, 4),
    (0x0126_f548, 8),
];

// Translated from 00e7a640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the body of a constructor, `thiscall`, no stack word): sets
/// the words `+0x12` to 1, `+0xc`, `+0xe`, `+0x10` and `+8` to 0, the vtable-like word at
/// `+4` to `010ee8a0` and the word at `+0` to 0x11, clears the entries of the array at
/// `+8` (its count, the word at `+0xe`, was just set to 0, so none) and sets `+0xe` and
/// `+0x10` to 0. Returns `this`.
pub fn fn_00e7a640(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 0x12, 1);
    e.mem.set_u16(base + 0xc, 0);
    e.mem.set_u16(base + 0xe, 0);
    e.mem.set_u16(base + 0x10, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 4, 0x010e_e8a0);
    e.mem.set_u32(base, 0x11);
    // The loop over the entries runs while the index (starting at 0) is below the count at
    // `+0xe`, which was set to 0 above, so it never runs.
    let count = u32::from(e.mem.u16(base + 0xe));
    if count > 0 {
        let entries = e.mem.u32(base + 8);
        for index in 0..count {
            e.mem.set_u32(entries.wrapping_add(index * 4), 0);
        }
    }
    e.mem.set_u16(base + 0xe, 0);
    e.mem.set_u16(base + 0x10, 0);
    this
}

// Translated from 00e7a760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments; answers `true`): when the outer table
/// at `0126f550` does not hold 0x12 entries yet, fills the 17 size words at `0126f508`
/// (`SIZE_TABLE`), then for each of the 0x12 numbers makes a 0x14-byte object
/// (`00aa13e0`, `fn_00e7a640`, the number stored at `+0`, `00a90180` giving its part at `+4`
/// the capacity 0x26) and stores it at that index of the outer table (growing it first
/// through `00a90180` when the index is past its count), and finally calls `00e78560` for
/// every pair of a number below 0x12 and a second number below 0x21.
pub fn fn_00e7a760(e: &mut Engine) -> bool {
    let table = e.mem.u32(OUTER_TABLE_SLOT);
    if e.mem.u16(table + 0xa) == 0x12 {
        return true;
    }
    for (address, value) in SIZE_TABLE {
        e.mem.set_u32(address, value);
    }
    for index in 0..0x12u32 {
        let block = e.call(ALLOCATE, &args![0x14u32]).u32();
        let object = if block != 0 {
            fn_00e7a640(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), object);
            e.mem.set_u32(object, index);
            e.call(ARRAY_SET_SIZE, &args![object + 4, 0x26u32]);
            let table = e.mem.u32(OUTER_TABLE_SLOT);
            let count = u32::from(e.mem.u16(table + 8));
            if index >= count {
                let wanted = u32::from(e.mem.u16(table + 0xe)) + index;
                e.call(ARRAY_SET_SIZE, &args![table, wanted]);
            }
            e.call(ARRAY_SET_AT, &args![table, index, local]);
        });
    }
    for first in 0..0x12u32 {
        for second in 0..0x21u32 {
            e.call(SET_HANDLER, &args![first, second]);
        }
    }
    true
}

// Translated from 00e7a860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments; answers what `fn_00e7a760` answers,
/// which it ends with by a jump): allocates 0x10 bytes from the memory manager and, when
/// that succeeds, sets the words `+8` and `+0xa` and `+0xc` to 0 (16 bits), `+0xe` to 1,
/// `+4` to 0 and the vtable-like word at `+0` to `010ee898`, keeping the block in
/// `0126f550` (null when the allocation fails); then runs `fn_00e7a760`.
pub fn fn_00e7a860(e: &mut Engine) -> bool {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x10u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(OUTER_TABLE_SLOT, 0);
        return fn_00e7a760(e);
    }
    e.mem.set_u16(block + 8, 0);
    e.mem.set_u16(block + 0xe, 1);
    e.mem.set_u16(block + 0xa, 0);
    e.mem.set_u16(block + 0xc, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block, 0x010e_e898);
    e.mem.set_u32(OUTER_TABLE_SLOT, block);
    fn_00e7a760(e)
}

// Translated from 00e7a8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7a640`), `cdecl` with three words: a Direct3D
/// depth/stencil format and the addresses of its depth and stencil bit counts, which it stores
/// (the jump table at `00e7a970`, read in Ghidra): 0x46 and 0x50 give 16 and 0, 0x47 32 and 0,
/// 0x49 15 and 1, 0x4b 24 and 8, 0x4d 24 and 0, 0x4f 24 and 4, every other format 0 and 0.
pub fn fn_00e7a8b0(e: &mut Engine, format: u32, depth: Ptr, stencil: Ptr) {
    let (depth_bits, stencil_bits) = match format {
        0x46 | 0x50 => (0x10, 0),
        0x47 => (0x20, 0),
        0x49 => (0xf, 1),
        0x4b => (0x18, 8),
        0x4d => (0x18, 0),
        0x4f => (0x18, 4),
        _ => (0, 0),
    };
    e.mem.set_u32(depth.addr(), depth_bits);
    e.mem.set_u32(stencil.addr(), stencil_bits);
}

/// `00aa10f0` (`cdecl`, one word): the Ni free.
const NI_FREE: u32 = 0x00aa_10f0;
/// `00aa1460` (`cdecl`, two words: block, size): the Ni sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// The vtable-like word `fn_00e7a6a0` stores at `+4` of each object before freeing it.
const PART_VTABLE_010EE890: u32 = 0x010e_e890;

// Translated from 00e7a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments): tears down what `fn_00e7a860` built.
/// For each index below the outer table's 16-bit count at `+0xa` (re-read after each object
/// freed) the word at that index of the table's array (`+4`) is an object; a non-null one has
/// the entries of its array (`+8`, count the 16-bit word at `+0xe`) cleared, its words `+0xe`
/// and `+0x10` set to 0, `010ee890` stored at `+4`, its array freed (`00aa10f0`) and itself
/// freed with size 0x14 (`00aa1460`). Then the table's own entries (count `+0xa`) are
/// cleared, its words `+0xc` and `+0xa` set to 0, and, when the table pointer `0126f550` is
/// not null, its vtable slot 0 is called with 1.
pub fn fn_00e7a6a0(e: &mut Engine) {
    let mut table = e.mem.u32(OUTER_TABLE_SLOT);
    let mut index = 0u32;
    if e.mem.u16(table + 0xa) != 0 {
        loop {
            let entries = e.mem.u32(table + 4);
            let object = e.mem.u32(entries.wrapping_add(index.wrapping_mul(4)));
            if object != 0 {
                let mut inner = 0u16;
                if e.mem.u16(object + 0xe) != 0 {
                    loop {
                        let array = e.mem.u32(object + 8);
                        e.mem.set_u32(array.wrapping_add(u32::from(inner) * 4), 0);
                        inner = inner.wrapping_add(1);
                        if inner >= e.mem.u16(object + 0xe) {
                            break;
                        }
                    }
                }
                e.mem.set_u16(object + 0xe, 0);
                e.mem.set_u16(object + 0x10, 0);
                let array = e.mem.u32(object + 8);
                e.mem.set_u32(object + 4, PART_VTABLE_010EE890);
                e.call(NI_FREE, &args![array]);
                e.call(NI_SIZED_DELETE, &args![object, 0x14u32]);
                table = e.mem.u32(OUTER_TABLE_SLOT);
            }
            index = index.wrapping_add(1);
            if index >= u32::from(e.mem.u16(table + 0xa)) {
                break;
            }
        }
    }
    let mut cleared = 0u16;
    if e.mem.u16(table + 0xa) != 0 {
        loop {
            let entries = e.mem.u32(table + 4);
            e.mem
                .set_u32(entries.wrapping_add(u32::from(cleared) * 4), 0);
            cleared = cleared.wrapping_add(1);
            if cleared >= e.mem.u16(table + 0xa) {
                break;
            }
        }
    }
    e.mem.set_u16(table + 0xc, 0);
    e.mem.set_u16(table + 0xa, 0);
    let holder = e.mem.u32(OUTER_TABLE_SLOT);
    if holder != 0 {
        e.vcall(holder, 0, &args![1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_a6a0, fn_00e7a6a0()),
        entry!(0x00e7_a640, fn_00e7a640(Ptr) -> Ptr),
        entry!(0x00e7_a760, fn_00e7a760() -> bool),
        entry!(0x00e7_a860, fn_00e7a860() -> bool),
        entry!(0x00e7_a8b0, fn_00e7a8b0(u32, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn constructor_sets_the_header_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        e.mem.write(this, &[0xcc; 0x14]);
        assert_eq!(e.call(0x00e7_a640, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x11);
        assert_eq!(e.mem.u32(this + 4), 0x010e_e8a0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u16(this + 0xe), 0);
        assert_eq!(e.mem.u16(this + 0x10), 0);
        assert_eq!(e.mem.u16(this + 0x12), 1);
    }

    /// An engine for the builders, with every callee outside this file doubled; the
    /// `00a90180` double records the capacity and `00ba3aa0` stores into an array of
    /// 0x12 words that `00a90180` set up for the outer table.
    fn engine() -> (
        Engine,
        Rc<RefCell<Vec<(u32, u32)>>>,
        Rc<RefCell<Vec<(u32, u32)>>>,
    ) {
        let mut e = engine_with_counting();
        e.map(0x0126_f000, 0x2000);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let sizes: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let log = sizes.clone();
        e.register_double(ARRAY_SET_SIZE, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let stores: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let log = stores.clone();
        e.register_double(ARRAY_SET_AT, move |e, a| {
            log.borrow_mut().push((a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        e.register(SET_HANDLER, |_, _| Ret::default());
        (e, sizes, stores)
    }

    #[test]
    fn builds_eighteen_objects_and_sets_every_handler() {
        let (mut e, sizes, stores) = engine();
        let table = e.mem.alloc(0x10);
        e.mem.set_u16(table + 8, 0x12);
        e.mem.set_u16(table + 0xe, 0);
        e.mem.set_u16(table + 0xa, 0);
        e.mem.set_u32(OUTER_TABLE_SLOT, table);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e7_a760, &args![]).bool());
        assert_eq!(e.mem.u32(0x0126_f508), 4);
        assert_eq!(e.mem.u32(0x0126_f514), 0x10);
        assert_eq!(e.mem.u32(0x0126_f548), 8);
        let stores = stores.borrow();
        assert_eq!(stores.len(), 0x12);
        for (index, (stored_index, object)) in stores.iter().enumerate() {
            assert_eq!(*stored_index, index as u32);
            assert_eq!(e.mem.u32(*object), index as u32);
            assert_eq!(e.mem.u32(*object + 4), 0x010e_e8a0);
        }
        // Each object part gets capacity 0x26; the table is at the count: no growth.
        assert_eq!(sizes.borrow().len(), 0x12);
        assert!(sizes.borrow().iter().all(|(_, size)| *size == 0x26));
        let handlers = calls_to(&e, SET_HANDLER);
        assert_eq!(handlers.len(), 0x12 * 0x21);
        assert_eq!(handlers[0], vec![0, 0]);
        assert_eq!(handlers[0x21], vec![1, 0]);
        assert_eq!(handlers[0x12 * 0x21 - 1], vec![0x11, 0x20]);
    }

    #[test]
    fn a_full_outer_table_is_left_alone() {
        let (mut e, sizes, _) = engine();
        let table = e.mem.alloc(0x10);
        e.mem.set_u16(table + 0xa, 0x12);
        e.mem.set_u32(OUTER_TABLE_SLOT, table);
        e.mem.set_u32(0x0126_f508, 99);
        assert!(e.call(0x00e7_a760, &args![]).bool());
        assert_eq!(e.mem.u32(0x0126_f508), 99);
        assert!(sizes.borrow().is_empty());
    }

    #[test]
    fn first_step_makes_the_outer_table_then_builds() {
        let (mut e, sizes, stores) = engine();
        assert!(e.call(0x00e7_a860, &args![]).bool());
        let table = e.mem.u32(OUTER_TABLE_SLOT);
        assert_eq!(e.mem.u32(table), 0x010e_e898);
        assert_eq!(e.mem.u16(table + 0xe), 1);
        assert_eq!(e.mem.u32(table + 4), 0);
        // The array part is at count 0: each index is past it, so the table grows to
        // `word(+0xe) + index` first.
        let sizes = sizes.borrow();
        let growth: Vec<_> = sizes.iter().filter(|(this, _)| *this == table).collect();
        assert_eq!(growth.len(), 0x12);
        assert_eq!(growth[0].1, 1);
        assert_eq!(growth[0x11].1, 0x12);
        assert_eq!(stores.borrow().len(), 0x12);
    }
}

#[cfg(test)]
mod b000y_tests {
    use super::*;

    #[test]
    fn fn_00e7a8b0_stores_the_depth_and_stencil_bits() {
        let mut e = Engine::new();
        let depth = e.mem.alloc(4);
        let stencil = e.mem.alloc(4);
        let cases: [(u32, u32, u32); 12] = [
            (0x46, 16, 0),
            (0x47, 32, 0),
            (0x48, 0, 0),
            (0x49, 15, 1),
            (0x4a, 0, 0),
            (0x4b, 24, 8),
            (0x4c, 0, 0),
            (0x4d, 24, 0),
            (0x4e, 0, 0),
            (0x4f, 24, 4),
            (0x50, 16, 0),
            (0x45, 0, 0),
        ];
        for (format, want_depth, want_stencil) in cases {
            e.mem.set_u32(depth, 0xaaaa_aaaa);
            e.mem.set_u32(stencil, 0xaaaa_aaaa);
            e.call(0x00e7_a8b0, &args![format, depth, stencil]);
            assert_eq!(e.mem.u32(depth), want_depth, "{format:#x}");
            assert_eq!(e.mem.u32(stencil), want_stencil, "{format:#x}");
        }
        e.call(0x00e7_a8b0, &args![0x51u32, depth, stencil]);
        assert_eq!((e.mem.u32(depth), e.mem.u32(stencil)), (0, 0));
    }
}

#[cfg(test)]
mod b053_tests {
    use super::*;

    #[test]
    fn fn_00e7a6a0_frees_every_object_then_the_table() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        e.register(0x0e00_1000, |_, _| Ret::default());
        e.put_vtable(0x0e00_0000, &[0x0e00_1000]);
        let table = e.mem.alloc(0x10);
        let entries = e.mem.alloc(0xc);
        e.mem.set_u32(table, 0x0e00_0000);
        e.mem.set_u32(table + 4, entries);
        e.mem.set_u16(table + 0xa, 3);
        e.mem.set_u16(table + 0xc, 3);
        let object = e.mem.alloc(0x14);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, 7);
        e.mem.set_u32(array + 4, 8);
        e.mem.set_u32(object + 8, array);
        e.mem.set_u16(object + 0xe, 2);
        e.mem.set_u16(object + 0x10, 2);
        e.mem.set_u32(entries, object);
        e.mem.set_u32(entries + 4, 0);
        e.mem.set_u32(entries + 8, object);
        e.mem.set_u32(OUTER_TABLE_SLOT, table);
        e.call_log = Some(vec![]);
        e.call(0x00e7_a6a0, &args![]);
        assert_eq!((e.mem.u32(array), e.mem.u32(array + 4)), (0, 0));
        assert_eq!(e.mem.u16(object + 0xe), 0);
        assert_eq!(e.mem.u16(object + 0x10), 0);
        assert_eq!(e.mem.u32(object + 4), PART_VTABLE_010EE890);
        assert_eq!(e.mem.u32(entries), 0);
        assert_eq!(e.mem.u32(entries + 8), 0);
        assert_eq!(e.mem.u16(table + 0xa), 0);
        assert_eq!(e.mem.u16(table + 0xc), 0);
        let log = e.call_log.take().unwrap();
        let of = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|(a, _)| *a == addr)
                .map(|(_, w)| w.clone())
                .collect()
        };
        // Both non-null entries (the same object twice here) are freed.
        assert_eq!(of(NI_FREE), vec![vec![array], vec![array]]);
        assert_eq!(of(NI_SIZED_DELETE), vec![vec![object, 0x14]; 2]);
        assert_eq!(of(0x0e00_1000), vec![vec![table, 1]]);
    }

    #[test]
    fn fn_00e7a6a0_with_an_empty_table_only_releases_it() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(0x0e00_1000, |_, _| Ret::default());
        e.put_vtable(0x0e00_0000, &[0x0e00_1000]);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, 0x0e00_0000);
        e.mem.set_u16(table + 0xc, 5);
        e.mem.set_u32(OUTER_TABLE_SLOT, table);
        e.call_log = Some(vec![]);
        e.call(0x00e7_a6a0, &args![]);
        assert_eq!(e.mem.u16(table + 0xc), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (0x0e00_1000, vec![table, 1]));
    }
}

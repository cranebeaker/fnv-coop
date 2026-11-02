//! `NiAnimation/run_00a413d0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// The memory manager singleton and its `Allocate` (`thiscall`, the size) and `Deallocate`
/// (`thiscall`, the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The array storage free (`cdecl`, the block).
const ARRAY_FREE: u32 = 0x00aa_10f0;
/// `00a90180` (unit `nioldscreenspace.cpp`, `thiscall` on the array, the new capacity): the
/// array's resize.
const ARRAY_SET_SIZE: u32 = 0x00a9_0180;
/// `00ba3aa0` (`thiscall` on the array, (index, &element)): stores an element at an index.
const ARRAY_SET_AT: u32 = 0x00ba_3aa0;
/// `NiObject`'s constructor (`thiscall`, no stack words).
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// The vtables of `NiTArray<BSAnimNoteListener::BSAnimReceiverType *>` (`010976ec`),
/// `NiTPrimitiveArray<BSAnimNoteListener::BSAnimReceiverType *>` (`010976f4`) and
/// `NiMorphData` (`010976fc`), by their RTTI type descriptors.
const RECEIVER_ARRAY_VTABLE: u32 = 0x0109_76ec;
const RECEIVER_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0109_76f4;
const NI_MORPH_DATA_VTABLE: u32 = 0x0109_76fc;

// Translated from 00a413d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: in an array of pointers to two-word records (an `NiTArray`, base
/// +4, used size +0xa), the index of the last record equal to `(first, second)`, or -1.
pub fn fn_00a413d0(e: &mut Engine, this: Ptr<NiTArray>, first: u32, second: u32) -> i32 {
    let size = u32::from(e.get(this, NiTArray::m_usSize));
    let mut found = -1i32;
    let base = e.get(this, NiTArray::m_pBase);
    for index in 0..size {
        let record = e.mem.u32(base + index * 4);
        if e.mem.u32(record) == first && e.mem.u32(record + 4) == second {
            found = index as i32;
        }
    }
    found
}

// Translated from 00a41450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (destructor-shaped, on an
/// `NiTPrimitiveArray<BSAnimNoteListener::BSAnimReceiverType *>`): as many times as the
/// array's used size was, takes the last element out (used size down by one, the slot
/// cleared, the non-null count down for a non-null one; null when the size is already 0) and
/// frees it with `MemoryManager::Deallocate`; then stores the `NiTArray` vtable `010976ec` and
/// frees the storage (`00aa10f0`).
pub fn fn_00a41450(e: &mut Engine, this: Ptr<NiTArray>) {
    let count = e.get(this, NiTArray::m_usSize);
    for _ in 0..count {
        let size = e.get(this, NiTArray::m_usSize);
        let element = if size == 0 {
            0
        } else {
            let base = e.get(this, NiTArray::m_pBase);
            let size = size - 1;
            e.set(this, NiTArray::m_usSize, size);
            let slot = base + u32::from(size) * 4;
            let element = e.mem.u32(slot);
            e.mem.set_u32(slot, 0);
            if element != 0 {
                let used = e.get(this, NiTArray::m_usESize).wrapping_add(0xffff);
                e.set(this, NiTArray::m_usESize, used);
            }
            element
        };
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, element]);
    }
    let base = e.get(this, NiTArray::m_pBase);
    e.mem.set_u32(this.addr(), RECEIVER_ARRAY_VTABLE);
    e.call(ARRAY_FREE, &args![base]);
}

// Translated from 00a414f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (constructor-shaped): an empty
/// `NiTPrimitiveArray<BSAnimNoteListener::BSAnimReceiverType *>` (vtable `010976f4`): no
/// storage, all counts 0, growing by 1. Answers `this`.
pub fn fn_00a414f0(e: &mut Engine, this: Ptr<NiTArray>) -> Ptr<NiTArray> {
    e.set(this, NiTArray::m_usGrowBy, 1);
    e.set(this, NiTArray::m_usMaxSize, 0);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    e.set(this, NiTArray::m_pBase, 0);
    e.mem.set_u32(this.addr(), RECEIVER_PRIMITIVE_ARRAY_VTABLE);
    this
}

// Translated from 00a41520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds the two-word record `(first, second)` to the array unless
/// an equal one is there (`00a413d0`). The record is allocated with
/// `MemoryManager::Allocate` (8 bytes; null when that fails), the array is grown to its used
/// size plus its grow step (`00a90180`) when full, and the record is stored at the used size
/// (`00ba3aa0`). Answers 0 (`AL`) either way.
pub fn fn_00a41520(e: &mut Engine, this: Ptr<NiTArray>, first: u32, second: u32) -> bool {
    if fn_00a413d0(e, this, first, second) != -1 {
        return false;
    }
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 8u32]).u32();
    let record = if block != 0 {
        e.mem.set_u32(block, first);
        e.mem.set_u32(block + 4, second);
        block
    } else {
        0
    };
    let size = e.get(this, NiTArray::m_usSize);
    if size >= e.get(this, NiTArray::m_usMaxSize) {
        let capacity = u32::from(e.get(this, NiTArray::m_usGrowBy)) + u32::from(size);
        e.call(ARRAY_SET_SIZE, &args![this, capacity]);
    }
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), record);
        e.call(ARRAY_SET_AT, &args![this, u32::from(size), element]);
    });
    false
}

// Translated from 00a41590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers `this + 4`.
pub fn fn_00a41590(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(4))
}

// Translated from 00a415a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiMorphData`): allocates 0x18 bytes (`operator new`); when that succeeds, the
/// `NiObject` constructor, no targets, no vertices per target, no target array, not relative,
/// the vtable `010976fc`. Answers the object, or null.
pub fn fn_00a415a0(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(OBJECT_CONSTRUCT, &args![block]);
    e.mem.set_u32(block + 0x8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 0);
    e.mem.set_u8(block + 0x14, 0);
    e.mem.set_u32(block, NI_MORPH_DATA_VTABLE);
    Ptr::new(block)
}

// Translated from 00a41410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an array of pointers to two-word records, one
/// word, `RET 4`): for each record (the used size +0xa re-read each step) whose first word
/// equals the argument's word at +8, the second word's object is called through its vtable
/// +4 with the argument.
pub fn fn_00a41410(e: &mut Engine, this: Ptr<NiTArray>, argument: u32) {
    let mut index = 0u32;
    while index < u32::from(e.get(this, NiTArray::m_usSize)) {
        let base = e.get(this, NiTArray::m_pBase);
        let record = e.mem.u32(base.wrapping_add(index * 4));
        if e.mem.u32(record) == e.mem.u32(argument + 8) {
            let object = e.mem.u32(record + 4);
            e.vcall(object, 4, &args![argument]);
        }
        index += 1;
    }
}

// Translated from 00a414c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the flags byte, `RET 4`; the deleting destructor of
/// the `NiTArray<BSAnimNoteListener::BSAnimReceiverType *>` its vtable `010976ec` names): the
/// vtable, the storage freed (`00aa10f0`) and, with flag 1, the array given back to the memory
/// manager. Answers `this`.
pub fn fn_00a414c0(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr<NiTArray> {
    let base = e.get(this, NiTArray::m_pBase);
    e.mem.set_u32(this.addr(), RECEIVER_ARRAY_VTABLE);
    e.call(ARRAY_FREE, &args![base]);
    if flags as u8 & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4_1410, fn_00a41410(Ptr<NiTArray>, u32)),
        entry!(
            0x00a4_14c0,
            fn_00a414c0(Ptr<NiTArray>, u32) -> Ptr<NiTArray>
        ),
        entry!(0x00a4_13d0, fn_00a413d0(Ptr<NiTArray>, u32, u32) -> i32),
        entry!(0x00a4_1450, fn_00a41450(Ptr<NiTArray>)),
        entry!(0x00a4_14f0, fn_00a414f0(Ptr<NiTArray>) -> Ptr<NiTArray>),
        entry!(0x00a4_1520, fn_00a41520(Ptr<NiTArray>, u32, u32) -> bool),
        entry!(0x00a4_1590, fn_00a41590(Ptr) -> Ptr),
        entry!(0x00a4_15a0, fn_00a415a0() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// An array holding `records` (pointers to two-word records), capacity `capacity`.
    fn array(e: &mut Engine, records: &[(u32, u32)], capacity: u16) -> Ptr<NiTArray> {
        let a = e.new_object::<NiTArray>();
        let base = e.mem.alloc(4 * u32::from(capacity).max(1));
        for (i, (first, second)) in records.iter().enumerate() {
            let r = e.mem.alloc(8);
            e.mem.set_u32(r, *first);
            e.mem.set_u32(r + 4, *second);
            e.mem.set_u32(base + 4 * i as u32, r);
        }
        e.set(a, NiTArray::m_pBase, base);
        e.set(a, NiTArray::m_usMaxSize, capacity);
        e.set(a, NiTArray::m_usSize, records.len() as u16);
        e.set(a, NiTArray::m_usESize, records.len() as u16);
        e.set(a, NiTArray::m_usGrowBy, 2);
        a
    }

    #[test]
    fn fn_00a413d0_finds_the_last_equal_record() {
        let mut e = Engine::new();
        let a = array(&mut e, &[(1, 2), (3, 4), (1, 2), (1, 5)], 4);
        assert_eq!(e.call(0x00a4_13d0, &args![a, 1u32, 2u32]).i32(), 2);
        assert_eq!(e.call(0x00a4_13d0, &args![a, 3u32, 4u32]).i32(), 1);
        assert_eq!(e.call(0x00a4_13d0, &args![a, 4u32, 3u32]).i32(), -1);
        let empty = array(&mut e, &[], 0);
        assert_eq!(e.call(0x00a4_13d0, &args![empty, 1u32, 2u32]).i32(), -1);
    }

    #[test]
    fn fn_00a41450_frees_every_element_then_the_storage() {
        let mut e = Engine::new();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(ARRAY_FREE, |_, _| Ret::default());
        let a = array(&mut e, &[(1, 2), (3, 4), (5, 6)], 4);
        let base = e.get(a, NiTArray::m_pBase);
        e.mem.set_u32(base + 4, 0);
        let first = e.mem.u32(base);
        let last = e.mem.u32(base + 8);
        e.call_log = Some(vec![]);
        e.call(0x00a4_1450, &args![a]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![
                vec![MEMORY_MANAGER, last],
                vec![MEMORY_MANAGER, 0],
                vec![MEMORY_MANAGER, first]
            ]
        );
        assert_eq!(e.get(a, NiTArray::m_usSize), 0);
        assert_eq!(e.get(a, NiTArray::m_usESize), 1);
        assert_eq!(e.mem.u32(base), 0);
        assert_eq!(e.mem.u32(a.addr()), RECEIVER_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, ARRAY_FREE), vec![vec![base]]);
    }

    #[test]
    fn fn_00a414f0_builds_an_empty_array() {
        let mut e = Engine::new();
        let a = e.new_object::<NiTArray>();
        e.mem.write(a.addr(), &[0xff; 16]);
        assert_eq!(e.call(0x00a4_14f0, &args![a]).u32(), a.addr());
        assert_eq!(e.mem.u32(a.addr()), RECEIVER_PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(e.get(a, NiTArray::m_pBase), 0);
        assert_eq!(e.get(a, NiTArray::m_usMaxSize), 0);
        assert_eq!(e.get(a, NiTArray::m_usSize), 0);
        assert_eq!(e.get(a, NiTArray::m_usESize), 0);
        assert_eq!(e.get(a, NiTArray::m_usGrowBy), 1);
    }

    #[test]
    fn fn_00a41520_adds_a_new_record_growing_a_full_array() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register_double(ARRAY_SET_AT, |e, a| {
            let record = e.mem.u32(a[2]);
            assert_eq!((e.mem.u32(record), e.mem.u32(record + 4)), (7, 8));
            Ret::default()
        });
        let a = array(&mut e, &[(1, 2)], 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_1520, &args![a, 7u32, 8u32]).u32() & 0xff, 0);
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![a.addr(), 3]]);
        let set = calls_to(&e, ARRAY_SET_AT);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][..2], [a.addr(), 1]);
        // An equal record: nothing is added.
        e.call_log = Some(vec![]);
        e.call(0x00a4_1520, &args![a, 1u32, 2u32]);
        assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
        // A failed allocation stores null, room left: no resize.
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        e.register_double(ARRAY_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            Ret::default()
        });
        e.set(a, NiTArray::m_usMaxSize, 4);
        e.call_log = Some(vec![]);
        e.call(0x00a4_1520, &args![a, 9u32, 9u32]);
        assert!(calls_to(&e, ARRAY_SET_SIZE).is_empty());
        assert_eq!(calls_to(&e, ARRAY_SET_AT).len(), 1);
    }

    #[test]
    fn fn_00a41590_answers_the_second_word() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a4_1590, &args![0x1000u32]).u32(), 0x1004);
    }

    #[test]
    fn fn_00a415a0_builds_a_morph_data_object() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xff; 0x18]);
            block.into_ret()
        });
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        let object = e.call(0x00a4_15a0, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_MORPH_DATA_VTABLE);
        for offset in [0x8u32, 0xc, 0x10] {
            assert_eq!(e.mem.u32(object + offset), 0);
        }
        assert_eq!(e.mem.u8(object + 0x14), 0);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_15a0, &args![]).u32(), 0);
    }
}

#[cfg(test)]
mod n007_tests {
    use super::*;

    #[test]
    fn matching_records_call_their_object() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0343_0000);
        e.register(0x0343_0000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        let array = e.new_object::<NiTArray>();
        let base = e.mem.alloc(8);
        for (i, kind) in [5u32, 6].iter().enumerate() {
            let record = e.mem.alloc(8);
            e.mem.set_u32(record, *kind);
            e.mem.set_u32(record + 4, object);
            e.mem.set_u32(base + 4 * i as u32, record);
        }
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usSize, 2);
        let argument = e.mem.alloc(0xc);
        e.mem.set_u32(argument + 8, 6);
        e.call(0x00a4_1410, &args![array, argument]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == 0x0343_0000).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![object, argument]);
    }

    #[test]
    fn deleting_destructor_frees_storage_and_array() {
        let mut e = Engine::new();
        e.register(ARRAY_FREE, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let array = e.new_object::<NiTArray>();
        e.set(array, NiTArray::m_pBase, 0x1230);
        assert_eq!(
            e.call(0x00a4_14c0, &args![array, 0x100u32]).u32(),
            array.addr()
        );
        assert_eq!(e.mem.u32(array.addr()), RECEIVER_ARRAY_VTABLE);
        e.call(0x00a4_14c0, &args![array, 1u32]);
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|c| c.0 == ARRAY_FREE).collect();
        assert_eq!(frees.len(), 2);
        assert_eq!(frees[0].1, vec![0x1230]);
        let deallocations: Vec<_> = log.iter().filter(|c| c.0 == MEMORY_DEALLOCATE).collect();
        assert_eq!(deallocations.len(), 1);
        assert_eq!(deallocations[0].1, vec![MEMORY_MANAGER, array.addr()]);
    }
}

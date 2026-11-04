//! `(unplaced)/run_006f28a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Four methods of an array of 0x28-byte elements: insertion, removal, and
//! the constructor and destructor of `BSScrapArray<PathingLocation,1024>`
//! (vtable `0106d864`, RTTI name `BSScrapArray<PathingLocation,1024>`; Xbox
//! PDB: a `BSSimpleArray<PathingLocation,1024>` plus `pScrapHeap` at +0x10,
//! 0x14 bytes). `PathingLocation` is 0x28 bytes. The engine map has no
//! names for these four functions.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// Size of one element (`PathingLocation`, Xbox PDB size 0x28).
const ELEMENT_SIZE: u32 = 0x28;
/// `BSScrapArray<PathingLocation,1024>`'s vtable (RTTI).
const SCRAP_ARRAY_VTABLE: u32 = 0x0106_d864;
/// `pScrapHeap` (Xbox PDB `BSScrapArray`): +0x10.
const SCRAP_HEAP_OFFSET: u32 = 0x10;
/// Vtable slot +0x04 of `BSSimpleArray` (Xbox PDB `_Allocate`): a new
/// buffer for the given capacity.
const SLOT_ALLOCATE: u32 = 0x04;

/// `006cf1b0` (engine map: `pathbuilder.cpp`): appends the item (thiscall,
/// one word).
const APPEND: u32 = 0x006c_f1b0;
/// `00438b90`: whether the array must grow before one more element fits.
const NEEDS_GROWTH: u32 = 0x0043_8b90;
/// `009a3910`: the capacity to grow to.
const GROWN_CAPACITY: u32 = 0x009a_3910;
/// `006f3170`: whether the array should shrink after a removal.
const SHOULD_SHRINK: u32 = 0x006f_3170;
/// `00869600`: the capacity to shrink to.
const SHRUNK_CAPACITY: u32 = 0x0086_9600;
/// `006928a0`: moves elements (destination, source, count).
const MOVE_ELEMENTS: u32 = 0x0069_28a0;
/// `006d00a0`: constructs `count` elements at an address.
const CONSTRUCT_ELEMENTS: u32 = 0x006d_00a0;
/// `006ac070`: destroys `count` elements at an address.
const DESTROY_ELEMENTS: u32 = 0x006a_c070;
/// `006a8500`: releases the current buffer.
const RELEASE_BUFFER: u32 = 0x006a_8500;
/// `006ac010`: empties the array (one byte argument).
const CLEAR: u32 = 0x006a_c010;
/// `006dcce0`: `PathingLocation::operator=` (engine map).
const PATHING_LOCATION_ASSIGN: u32 = 0x006d_cce0;
/// `006cf180`: the base array's constructor.
const BASE_CONSTRUCT: u32 = 0x006c_f180;
/// `006abfc0`: the base array's destructor.
const BASE_DESTROY: u32 = 0x006a_bfc0;
/// `00401020`: returns the memory manager the scrap heap is asked from.
const MEMORY_MANAGER: u32 = 0x0040_1020;
/// `00aa42e0`: `MemoryManager::GetThreadScrapHeap` (decompiler name).
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// `006d0140`: reserves the array's initial buffer (two words).
const RESERVE: u32 = 0x006d_0140;

fn element(base: u32, index: u32) -> u32 {
    base.wrapping_add(index.wrapping_mul(ELEMENT_SIZE))
}

// Translated from 006f28a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts a copy of `item` at `index`: appends it (`006cf1b0`) when
/// `index` is the size; otherwise moves the tail up one element (into a new
/// buffer from vtable slot +0x04 when `00438b90` says the array is full),
/// constructs the gap, bumps the size and assigns the item into the gap
/// (`PathingLocation::operator=`).
pub fn fn_006f28a0(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, item: Ptr) {
    let size = e.get(this, BSSimpleArray::iSize);
    if index == size {
        e.call(APPEND, &args![this, item]);
        return;
    }
    if e.call(NEEDS_GROWTH, &args![this]).bool() {
        let capacity = e.call(GROWN_CAPACITY, &args![this]).u32();
        let buffer = e.vcall(this.addr(), SLOT_ALLOCATE, &args![capacity]).u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(MOVE_ELEMENTS, &args![this, buffer, old, index]);
        e.call(
            CONSTRUCT_ELEMENTS,
            &args![this, element(buffer, index), 1u32],
        );
        let size = e.get(this, BSSimpleArray::iSize);
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            MOVE_ELEMENTS,
            &args![
                this,
                element(buffer, index).wrapping_add(ELEMENT_SIZE),
                element(old, index),
                size.wrapping_sub(index)
            ],
        );
        e.call(RELEASE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    } else {
        let size = e.get(this, BSSimpleArray::iSize);
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            MOVE_ELEMENTS,
            &args![
                this,
                element(old, index).wrapping_add(ELEMENT_SIZE),
                element(old, index),
                size.wrapping_sub(index)
            ],
        );
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(CONSTRUCT_ELEMENTS, &args![this, element(old, index), 1u32]);
    }
    let size = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, size.wrapping_add(1));
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    e.call(
        PATHING_LOCATION_ASSIGN,
        &args![element(buffer, index), item],
    );
}

// Translated from 006f29e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `count` elements at `index`. When `count` is not below the size
/// the whole array is emptied (`006ac010` with `shrink`). Otherwise the
/// elements are destroyed and the tail moved down; with `shrink` set and
/// `006f3170` agreeing, the survivors go to a smaller buffer from vtable
/// slot +0x04 (the tail's count there is `size - count`, as the exe passes
/// it). The size drops by `count`.
pub fn fn_006f29e0(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, count: u32, shrink: u8) {
    let size = e.get(this, BSSimpleArray::iSize);
    if count >= size {
        e.call(CLEAR, &args![this, shrink]);
        return;
    }
    if shrink != 0 && e.call(SHOULD_SHRINK, &args![this]).bool() {
        let capacity = e.call(SHRUNK_CAPACITY, &args![this]).u32();
        let buffer = e.vcall(this.addr(), SLOT_ALLOCATE, &args![capacity]).u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(MOVE_ELEMENTS, &args![this, buffer, old, index]);
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(DESTROY_ELEMENTS, &args![this, element(old, index), count]);
        let size = e.get(this, BSSimpleArray::iSize);
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            MOVE_ELEMENTS,
            &args![
                this,
                element(buffer, index),
                element(element(old, index), count),
                size.wrapping_sub(count)
            ],
        );
        e.call(RELEASE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    } else {
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(DESTROY_ELEMENTS, &args![this, element(old, index), count]);
        let size = e.get(this, BSSimpleArray::iSize);
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            MOVE_ELEMENTS,
            &args![
                this,
                element(old, index),
                element(element(old, index), count),
                size.wrapping_sub(index).wrapping_sub(count)
            ],
        );
    }
    let size = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, size.wrapping_sub(count));
}

// Translated from 006f2b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSScrapArray<PathingLocation,1024>`: the base array's
/// constructor, the vtable, the thread's scrap heap at +0x10, then an empty
/// reservation (`006d0140(0, 0)`). Returns `this`.
// Translation incomplete: 006f2b30 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_006f2b30(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE);
    let manager = e.call(MEMORY_MANAGER, &args![]).u32();
    let heap = e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32();
    e.mem.set_u32(this.addr() + SCRAP_HEAP_OFFSET, heap);
    e.call(RESERVE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006f2bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `BSScrapArray<PathingLocation,1024>`: the vtable,
/// empties the array (`006ac010(1)`), then the base array's destructor.
// Translation incomplete: 006f2bb0 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_006f2bb0(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE);
    e.call(CLEAR, &args![this, 1u32]);
    e.call(BASE_DESTROY, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f28a0, fn_006f28a0(Ptr<BSSimpleArray>, u32, Ptr)),
        entry!(0x006f29e0, fn_006f29e0(Ptr<BSSimpleArray>, u32, u32, u8)),
        entry!(
            0x006f2b30,
            fn_006f2b30(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006f2bb0, fn_006f2bb0(Ptr<BSSimpleArray>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    const ALLOCATE_DOUBLE: u32 = 0x0001_0000;
    const VTABLE: u32 = 0x0002_0000;
    const NEW_BUFFER: u32 = 0x0050_0000;

    /// Doubles for every callee; `grow` is what `00438b90` and `006f3170`
    /// answer.
    fn engine(grow: bool) -> Engine {
        let mut e = Engine::new();
        for addr in [
            APPEND,
            MOVE_ELEMENTS,
            CONSTRUCT_ELEMENTS,
            DESTROY_ELEMENTS,
            RELEASE_BUFFER,
            CLEAR,
            PATHING_LOCATION_ASSIGN,
            BASE_CONSTRUCT,
            BASE_DESTROY,
            RESERVE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let answer = Ret {
            eax: grow as u32,
            ..Ret::default()
        };
        e.register_double(NEEDS_GROWTH, move |_, _| answer);
        e.register_double(SHOULD_SHRINK, move |_, _| answer);
        e.register(GROWN_CAPACITY, |_, _| 16u32.into_ret());
        e.register(SHRUNK_CAPACITY, |_, _| 4u32.into_ret());
        e.register(ALLOCATE_DOUBLE, |_, _| NEW_BUFFER.into_ret());
        e.put_vtable(VTABLE, &[0, ALLOCATE_DOUBLE]);
        e.register(MEMORY_MANAGER, |_, _| 0x0060_0000u32.into_ret());
        e.register(GET_THREAD_SCRAP_HEAP, |_, a| (a[0] + 1).into_ret());
        e
    }

    fn array(e: &mut Engine, buffer: u32, size: u32, capacity: u32) -> Ptr<BSSimpleArray> {
        let array: Ptr<BSSimpleArray> = Ptr::new(e.mem.alloc(0x14));
        e.mem.set_u32(array.addr(), VTABLE);
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iSize, size);
        e.set(array, BSSimpleArray::iReservedSize, capacity);
        array
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn insert_at_the_end_appends() {
        let mut e = engine(false);
        let a = array(&mut e, 0x1000, 3, 8);
        e.call_log = Some(vec![]);
        e.call(0x006f_28a0, &args![a, 3u32, 0x7777u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, APPEND), vec![vec![a.addr(), 0x7777]]);
        assert!(calls(&log, PATHING_LOCATION_ASSIGN).is_empty());
        assert_eq!(e.get(a, BSSimpleArray::iSize), 3);
    }

    #[test]
    fn insert_in_place_moves_the_tail_up() {
        let mut e = engine(false);
        let a = array(&mut e, 0x1000, 5, 8);
        e.call_log = Some(vec![]);
        e.call(0x006f_28a0, &args![a, 2u32, 0x7777u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MOVE_ELEMENTS),
            vec![vec![a.addr(), 0x1000 + 3 * 0x28, 0x1000 + 2 * 0x28, 3]]
        );
        assert_eq!(
            calls(&log, CONSTRUCT_ELEMENTS),
            vec![vec![a.addr(), 0x1000 + 2 * 0x28, 1]]
        );
        assert_eq!(
            calls(&log, PATHING_LOCATION_ASSIGN),
            vec![vec![0x1000 + 2 * 0x28, 0x7777]]
        );
        assert_eq!(e.get(a, BSSimpleArray::iSize), 6);
    }

    #[test]
    fn insert_into_a_full_array_moves_to_a_new_buffer() {
        let mut e = engine(true);
        let a = array(&mut e, 0x1000, 4, 4);
        e.call_log = Some(vec![]);
        e.call(0x006f_28a0, &args![a, 1u32, 0x7777u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, ALLOCATE_DOUBLE), vec![vec![a.addr(), 16]]);
        assert_eq!(
            calls(&log, MOVE_ELEMENTS),
            vec![
                vec![a.addr(), NEW_BUFFER, 0x1000, 1],
                vec![a.addr(), NEW_BUFFER + 2 * 0x28, 0x1000 + 0x28, 3],
            ]
        );
        assert_eq!(
            calls(&log, CONSTRUCT_ELEMENTS),
            vec![vec![a.addr(), NEW_BUFFER + 0x28, 1]]
        );
        assert_eq!(calls(&log, RELEASE_BUFFER), vec![vec![a.addr()]]);
        assert_eq!(e.get(a, BSSimpleArray::pBuffer), NEW_BUFFER);
        assert_eq!(e.get(a, BSSimpleArray::iReservedSize), 16);
        assert_eq!(e.get(a, BSSimpleArray::iSize), 5);
        assert_eq!(
            calls(&log, PATHING_LOCATION_ASSIGN),
            vec![vec![NEW_BUFFER + 0x28, 0x7777]]
        );
    }

    #[test]
    fn remove_everything_clears() {
        let mut e = engine(false);
        let a = array(&mut e, 0x1000, 3, 8);
        e.call_log = Some(vec![]);
        e.call(0x006f_29e0, &args![a, 0u32, 3u32, 1u8]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, CLEAR), vec![vec![a.addr(), 1]]);
        assert_eq!(e.get(a, BSSimpleArray::iSize), 3);
    }

    #[test]
    fn remove_in_place_moves_the_tail_down() {
        let mut e = engine(true);
        let a = array(&mut e, 0x1000, 6, 8);
        e.call_log = Some(vec![]);
        // `shrink` 0: `006f3170` is not asked.
        e.call(0x006f_29e0, &args![a, 1u32, 2u32, 0u8]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, SHOULD_SHRINK).is_empty());
        assert_eq!(
            calls(&log, DESTROY_ELEMENTS),
            vec![vec![a.addr(), 0x1000 + 0x28, 2]]
        );
        assert_eq!(
            calls(&log, MOVE_ELEMENTS),
            vec![vec![a.addr(), 0x1000 + 0x28, 0x1000 + 3 * 0x28, 3]]
        );
        assert_eq!(e.get(a, BSSimpleArray::iSize), 4);
    }

    #[test]
    fn remove_with_shrink_moves_to_a_smaller_buffer() {
        let mut e = engine(true);
        let a = array(&mut e, 0x1000, 6, 16);
        e.call_log = Some(vec![]);
        e.call(0x006f_29e0, &args![a, 1u32, 2u32, 1u8]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, ALLOCATE_DOUBLE), vec![vec![a.addr(), 4]]);
        assert_eq!(
            calls(&log, MOVE_ELEMENTS),
            vec![
                vec![a.addr(), NEW_BUFFER, 0x1000, 1],
                vec![a.addr(), NEW_BUFFER + 0x28, 0x1000 + 3 * 0x28, 4],
            ]
        );
        assert_eq!(e.get(a, BSSimpleArray::pBuffer), NEW_BUFFER);
        assert_eq!(e.get(a, BSSimpleArray::iReservedSize), 4);
        assert_eq!(e.get(a, BSSimpleArray::iSize), 4);
    }

    #[test]
    fn constructor_sets_vtable_scrap_heap_and_reserves() {
        let mut e = engine(false);
        let a: Ptr<BSSimpleArray> = Ptr::new(e.mem.alloc(0x14));
        e.call_log = Some(vec![]);
        let back = e.call(0x006f_2b30, &args![a]).ptr::<BSSimpleArray>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, a);
        assert_eq!(e.mem.u32(a.addr()), SCRAP_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(a.addr() + 0x10), 0x0060_0001);
        assert_eq!(calls(&log, BASE_CONSTRUCT), vec![vec![a.addr()]]);
        assert_eq!(calls(&log, RESERVE), vec![vec![a.addr(), 0, 0]]);
    }

    #[test]
    fn destructor_clears_then_runs_the_base_destructor() {
        let mut e = engine(false);
        let a = array(&mut e, 0x1000, 2, 4);
        e.call_log = Some(vec![]);
        e.call(0x006f_2bb0, &args![a]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(a.addr()), SCRAP_ARRAY_VTABLE);
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).skip(1).collect();
        assert_eq!(order, vec![CLEAR, BASE_DESTROY]);
        assert_eq!(calls(&log, CLEAR), vec![vec![a.addr(), 1]]);
    }
}

//! `fallout/ai/run_006f23d0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAUPathEdge@PathSmoother@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106D800: u32 = 0x0106_d800;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAUUniqueVert@PathSmoother@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106D828: u32 = 0x0106_d828;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAUBoundsEdge@PathSmoother@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106D850: u32 = 0x0106_d850;

// Translated from 006f25b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAUPathEdge@PathSmoother@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006f25b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106D800);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006f26d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAUUniqueVert@PathSmoother@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006f26d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106D828);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006f27f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAUBoundsEdge@PathSmoother@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006f27f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106D850);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `BSSimpleArray<BGSBodyPart *,1024>::AddUninitialized` in the engine map (`007cb2e0`,
/// `thiscall`, one stack word); called here with the item's address, so the name may be a
/// folded one: described by address.
const APPEND_007CB2E0: u32 = 0x007c_b2e0;
/// `00438b90` (unnamed, `thiscall`, no stack words; a `bool` in `AL`): whether the insert
/// needs a new buffer.
const NEEDS_GROWTH_00438B90: u32 = 0x0043_8b90;
/// `009a3910` (unnamed, `thiscall`, no stack words): the reserved size of the new buffer.
const GROWN_SIZE_009A3910: u32 = 0x009a_3910;
/// The array's vtable slot `+4` (one stack word, the new reserved size): allocates the new
/// buffer.
const ALLOCATE_SLOT: u32 = 0x4;
/// `0042fb60` (unnamed, engine map unit `extradatalist.cpp`, `thiscall`, three stack words:
/// destination, source, count of 4-byte elements).
const MOVE_ELEMENTS_0042FB60: u32 = 0x0042_fb60;
/// `00726bf0` (unnamed, `thiscall`, two stack words: the first element, count).
const PREPARE_ELEMENTS_00726BF0: u32 = 0x0072_6bf0;
/// `006a8500` (unnamed, engine map unit `navmeshsearch.cpp`, `thiscall`, no stack words): run
/// on the array before the new buffer replaces the old one.
const RELEASE_BUFFER_006A8500: u32 = 0x006a_8500;

// Translated from 006f23d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An insert of a `BSSimpleArray` instance with 4-byte elements (unnamed in the engine map;
/// `thiscall`, two stack words: the index and the address of the item, `RET 8`). At the end
/// (`index == iSize`) it only calls `007cb2e0(this, item)`. Otherwise, when `00438b90` says
/// the array must grow: the new reserved size (`009a3910`), a new buffer from vtable slot
/// `+4`, the elements before `index` moved into it (`0042fb60`), `00726bf0` on its slot
/// `index`, the elements from `index` on moved one slot up into it, `006a8500` on the array,
/// then `pBuffer` and `iReservedSize` take the new buffer and size. When it need not grow, the
/// elements from `index` on move one slot up in place and `00726bf0` runs on slot `index`.
/// Both paths then add one to `iSize` and store the item's word at slot `index`.
pub fn fn_006f23d0(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, item: u32) {
    let array = this.addr();
    if index == e.get(this, BSSimpleArray::iSize) {
        e.call(APPEND_007CB2E0, &args![array, item]);
        return;
    }
    if e.call(NEEDS_GROWTH_00438B90, &args![array]).bool() {
        let reserved = e.call(GROWN_SIZE_009A3910, &args![array]).u32();
        let buffer = e.vcall(array, ALLOCATE_SLOT, &args![reserved]).u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(MOVE_ELEMENTS_0042FB60, &args![array, buffer, old, index]);
        let slot = buffer.wrapping_add(index.wrapping_mul(4));
        e.call(PREPARE_ELEMENTS_00726BF0, &args![array, slot, 1u32]);
        let size = e.get(this, BSSimpleArray::iSize);
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(
            MOVE_ELEMENTS_0042FB60,
            &args![
                array,
                slot.wrapping_add(4),
                old.wrapping_add(index.wrapping_mul(4)),
                size.wrapping_sub(index)
            ],
        );
        e.call(RELEASE_BUFFER_006A8500, &args![array]);
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserved);
    } else {
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let slot = buffer.wrapping_add(index.wrapping_mul(4));
        e.call(
            MOVE_ELEMENTS_0042FB60,
            &args![array, slot.wrapping_add(4), slot, size.wrapping_sub(index)],
        );
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let slot = buffer.wrapping_add(index.wrapping_mul(4));
        e.call(PREPARE_ELEMENTS_00726BF0, &args![array, slot, 1u32]);
    }
    let size = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, size.wrapping_add(1));
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let value = e.mem.u32(item);
    e.mem
        .set_u32(buffer.wrapping_add(index.wrapping_mul(4)), value);
}

/// The `BSScrapArray` vtables (RTTI names read through their complete object locators):
/// `.?AV?$BSScrapArray@PAUPathEdge@PathSmoother@@$0EAA@@`,
/// `.?AV?$BSScrapArray@PAUUniqueVert@PathSmoother@@$0EAA@@` and
/// `.?AV?$BSScrapArray@PAUBoundsEdge@PathSmoother@@$0EAA@@`.
const SCRAP_ARRAY_VTABLE_0106D7EC: u32 = 0x0106_d7ec;
const SCRAP_ARRAY_VTABLE_0106D814: u32 = 0x0106_d814;
const SCRAP_ARRAY_VTABLE_0106D83C: u32 = 0x0106_d83c;
/// The base constructors of the three instances (`006f30e0`, `006f3110`, `006f3140`; unnamed,
/// `thiscall`, no stack words).
const BASE_CONSTRUCT_006F30E0: u32 = 0x006f_30e0;
const BASE_CONSTRUCT_006F3110: u32 = 0x006f_3110;
const BASE_CONSTRUCT_006F3140: u32 = 0x006f_3140;
/// The `MemoryManager` singleton getter (`00401020`, no words) and
/// `MemoryManager::GetThreadScrapHeap` (engine map, `00aa42e0`, `thiscall` on the manager).
const MEMORY_MANAGER_00401020: u32 = 0x0040_1020;
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// `006b3eb0` (unnamed, engine map unit `pathsmootherpov.cpp`, `thiscall`, two stack words).
const ARRAY_INITIALISE_006B3EB0: u32 = 0x006b_3eb0;
/// The scrap heap word of a `BSScrapArray`, `+0x10`.
const SCRAP_HEAP: u32 = 0x10;

/// The constructor the three `BSScrapArray` instances share: the base constructor, the
/// instance's vtable, the heap word (`heap`, or the thread's scrap heap of the memory manager
/// when `heap` is zero) and `006b3eb0(first, second)`.
fn construct_scrap_array(
    e: &mut Engine,
    this: Ptr,
    (base_construct, vtable): (u32, u32),
    (first, second, heap): (u32, u32, u32),
) -> Ptr {
    e.call(base_construct, &args![this]);
    e.mem.set_u32(this.addr(), vtable);
    let heap = if heap != 0 {
        heap
    } else {
        let manager = e.call(MEMORY_MANAGER_00401020, &args![]).u32();
        e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32()
    };
    e.mem.set_u32(this.addr() + SCRAP_HEAP, heap);
    e.call(ARRAY_INITIALISE_006B3EB0, &args![this, first, second]);
    this
}

// Translated from 006f2510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSScrapArray<PathSmoother::PathEdge *,1024>` (by its vtable; unnamed in the
/// engine map; `thiscall`, three stack words, `RET 0xc`): the base constructor `006f30e0`, the
/// vtable `0106d7ec`, the heap word `+0x10` (`heap`, or the thread's scrap heap when zero) and
/// `006b3eb0(first, second)`. Returns `this`.
// Translation incomplete: 006f2510 - the C++ exception frame (FS:[0] chain, one unwind state for the base) is not translated.
pub fn fn_006f2510(e: &mut Engine, this: Ptr, first: u32, second: u32, heap: u32) -> Ptr {
    construct_scrap_array(
        e,
        this,
        (BASE_CONSTRUCT_006F30E0, SCRAP_ARRAY_VTABLE_0106D7EC),
        (first, second, heap),
    )
}

// Translated from 006f2630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSScrapArray<PathSmoother::UniqueVert *,1024>` (by its vtable; unnamed in
/// the engine map; `thiscall`, three stack words, `RET 0xc`): as [`fn_006f2510`] with the base
/// constructor `006f3110` and the vtable `0106d814`. Returns `this`.
// Translation incomplete: 006f2630 - the C++ exception frame (FS:[0] chain, one unwind state for the base) is not translated.
pub fn fn_006f2630(e: &mut Engine, this: Ptr, first: u32, second: u32, heap: u32) -> Ptr {
    construct_scrap_array(
        e,
        this,
        (BASE_CONSTRUCT_006F3110, SCRAP_ARRAY_VTABLE_0106D814),
        (first, second, heap),
    )
}

// Translated from 006f2750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSScrapArray<PathSmoother::BoundsEdge *,1024>` (by its vtable; unnamed in
/// the engine map; `thiscall`, three stack words, `RET 0xc`): as [`fn_006f2510`] with the base
/// constructor `006f3140` and the vtable `0106d83c`. Returns `this`.
// Translation incomplete: 006f2750 - the C++ exception frame (FS:[0] chain, one unwind state for the base) is not translated.
pub fn fn_006f2750(e: &mut Engine, this: Ptr, first: u32, second: u32, heap: u32) -> Ptr {
    construct_scrap_array(
        e,
        this,
        (BASE_CONSTRUCT_006F3140, SCRAP_ARRAY_VTABLE_0106D83C),
        (first, second, heap),
    )
}

// Translated from 006f25d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `BSScrapArray<PathSmoother::PathEdge *,1024>` (by its vtable; unnamed in the
/// engine map; `thiscall`, no stack words): the vtable `0106d7ec`, `008454f0(this, 1)`, then
/// the `BSSimpleArray` base destructor [`fn_006f25b0`].
// Translation incomplete: 006f25d0 - the C++ exception frame (FS:[0] chain, one unwind state for the base) is not translated.
pub fn fn_006f25d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE_0106D7EC);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    fn_006f25b0(e, this);
}

// Translated from 006f26f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `BSScrapArray<PathSmoother::UniqueVert *,1024>` (by its vtable; unnamed in
/// the engine map; `thiscall`, no stack words): the vtable `0106d814`, `008454f0(this, 1)`,
/// then the `BSSimpleArray` base destructor [`fn_006f26d0`].
// Translation incomplete: 006f26f0 - the C++ exception frame (FS:[0] chain, one unwind state for the base) is not translated.
pub fn fn_006f26f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE_0106D814);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    fn_006f26d0(e, this);
}

// Translated from 006f2810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `BSScrapArray<PathSmoother::BoundsEdge *,1024>` (by its vtable; unnamed in
/// the engine map; `thiscall`, no stack words): the vtable `0106d83c`, `008454f0(this, 1)`,
/// then the `BSSimpleArray` base destructor [`fn_006f27f0`].
// Translation incomplete: 006f2810 - the C++ exception frame (FS:[0] chain, one unwind state for the base) is not translated.
pub fn fn_006f2810(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_VTABLE_0106D83C);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
    fn_006f27f0(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_25b0, fn_006f25b0(Ptr)),
        entry!(0x006f_26d0, fn_006f26d0(Ptr)),
        entry!(0x006f_27f0, fn_006f27f0(Ptr)),
        entry!(0x006f_23d0, fn_006f23d0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006f_2510, fn_006f2510(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006f_2630, fn_006f2630(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006f_2750, fn_006f2750(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006f_25d0, fn_006f25d0(Ptr)),
        entry!(0x006f_26f0, fn_006f26f0(Ptr)),
        entry!(0x006f_2810, fn_006f2810(Ptr)),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_006f25b0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006f_25b0, ARRAY_VTABLE_0106D800, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006f26d0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006f_26d0, ARRAY_VTABLE_0106D828, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006f27f0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006f_27f0, ARRAY_VTABLE_0106D850, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod n027_tests {
    use super::*;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// An array with a buffer of eight slots holding `size` items, reserved size 4, its
    /// allocate slot at `00701000` and doubles for the callees.
    fn array_with(e: &mut Engine, size: u32) -> (Ptr<BSSimpleArray>, u32) {
        let array: Ptr<BSSimpleArray> = e.new_object();
        let buffer = e.mem.alloc(0x20);
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iSize, size);
        e.set(array, BSSimpleArray::iReservedSize, 4);
        e.put_vtable(0x0070_0000, &[0, 0x0070_1000]);
        e.mem.set_u32(array.addr(), 0x0070_0000);
        for callee in [
            APPEND_007CB2E0,
            MOVE_ELEMENTS_0042FB60,
            PREPARE_ELEMENTS_00726BF0,
            RELEASE_BUFFER_006A8500,
        ] {
            e.register(callee, nothing);
        }
        e.register(GROWN_SIZE_009A3910, |_, _| 8u32.into_ret());
        (array, buffer)
    }

    #[test]
    fn fn_006f23d0_appends_at_the_end() {
        let mut e = Engine::new();
        let (array, _) = array_with(&mut e, 3);
        let item = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x006f_23d0, &args![array, 3u32, item]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, APPEND_007CB2E0), vec![vec![array.addr(), item]]);
        assert!(calls(&log, MOVE_ELEMENTS_0042FB60).is_empty());
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
    }

    #[test]
    fn fn_006f23d0_inserts_in_place() {
        let mut e = Engine::new();
        let (array, buffer) = array_with(&mut e, 3);
        e.register(NEEDS_GROWTH_00438B90, |_, _| 0x100u32.into_ret());
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x006f_23d0, &args![array, 1u32, item]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MOVE_ELEMENTS_0042FB60),
            vec![vec![array.addr(), buffer + 8, buffer + 4, 2]]
        );
        assert_eq!(
            calls(&log, PREPARE_ELEMENTS_00726BF0),
            vec![vec![array.addr(), buffer + 4, 1]]
        );
        assert!(calls(&log, GROWN_SIZE_009A3910).is_empty());
        assert_eq!(e.get(array, BSSimpleArray::iSize), 4);
        assert_eq!(e.mem.u32(buffer + 4), 0x4444);
    }

    #[test]
    fn fn_006f23d0_grows_into_a_new_buffer() {
        let mut e = Engine::new();
        let (array, old) = array_with(&mut e, 4);
        e.register(NEEDS_GROWTH_00438B90, |_, _| 1u32.into_ret());
        let new = e.mem.alloc(0x40);
        e.register_double(0x0070_1000, move |_, a| {
            assert_eq!(a[1], 8);
            new.into_ret()
        });
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x006f_23d0, &args![array, 2u32, item]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MOVE_ELEMENTS_0042FB60),
            vec![
                vec![array.addr(), new, old, 2],
                vec![array.addr(), new + 12, old + 8, 2]
            ]
        );
        assert_eq!(
            calls(&log, PREPARE_ELEMENTS_00726BF0),
            vec![vec![array.addr(), new + 8, 1]]
        );
        assert_eq!(
            calls(&log, RELEASE_BUFFER_006A8500),
            vec![vec![array.addr()]]
        );
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), new);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 8);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 5);
        assert_eq!(e.mem.u32(new + 8), 0x5555);
    }

    #[test]
    fn scrap_array_constructors_set_vtable_heap_and_initialise() {
        for (address, base, vtable) in [
            (
                0x006f_2510u32,
                BASE_CONSTRUCT_006F30E0,
                SCRAP_ARRAY_VTABLE_0106D7EC,
            ),
            (
                0x006f_2630,
                BASE_CONSTRUCT_006F3110,
                SCRAP_ARRAY_VTABLE_0106D814,
            ),
            (
                0x006f_2750,
                BASE_CONSTRUCT_006F3140,
                SCRAP_ARRAY_VTABLE_0106D83C,
            ),
        ] {
            let mut e = Engine::new();
            e.register(base, nothing);
            e.register(ARRAY_INITIALISE_006B3EB0, nothing);
            e.register(MEMORY_MANAGER_00401020, |_, _| 0x1111u32.into_ret());
            e.register(GET_THREAD_SCRAP_HEAP, |_, a| (a[0] + 0x2222).into_ret());
            let this = e.mem.alloc(0x14);
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(address, &args![this, 7u32, 8u32, 0x9999u32]).u32(),
                this
            );
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, base), vec![vec![this]]);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(e.mem.u32(this + 0x10), 0x9999);
            assert_eq!(
                calls(&log, ARRAY_INITIALISE_006B3EB0),
                vec![vec![this, 7, 8]]
            );
            assert!(calls(&log, GET_THREAD_SCRAP_HEAP).is_empty());
            // No heap given: the thread's scrap heap.
            e.call_log = Some(vec![]);
            e.call(address, &args![this, 1u32, 2u32, 0u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this + 0x10), 0x1111 + 0x2222);
            assert_eq!(calls(&log, GET_THREAD_SCRAP_HEAP), vec![vec![0x1111]]);
        }
    }

    #[test]
    fn scrap_array_destructors_release_then_run_the_base() {
        for (address, vtable, base_vtable) in [
            (
                0x006f_25d0u32,
                SCRAP_ARRAY_VTABLE_0106D7EC,
                ARRAY_VTABLE_0106D800,
            ),
            (
                0x006f_26f0,
                SCRAP_ARRAY_VTABLE_0106D814,
                ARRAY_VTABLE_0106D828,
            ),
            (
                0x006f_2810,
                SCRAP_ARRAY_VTABLE_0106D83C,
                ARRAY_VTABLE_0106D850,
            ),
        ] {
            let mut e = Engine::new();
            let this = e.mem.alloc(0x14);
            let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
            let record = seen.clone();
            e.register_double(ARRAY_RELEASE_008454F0, move |e, a| {
                record.borrow_mut().push((e.mem.u32(a[0]), a.to_vec()));
                Ret::default()
            });
            e.call(address, &args![this]);
            // The instance's vtable for the first release, the base's for the second.
            assert_eq!(
                *seen.borrow(),
                vec![(vtable, vec![this, 1]), (base_vtable, vec![this, 1])]
            );
            assert_eq!(e.mem.u32(this), base_vtable);
        }
    }
}

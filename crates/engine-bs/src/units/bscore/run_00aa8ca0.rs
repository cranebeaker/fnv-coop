//! `BSCore/run_00aa8ca0` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `AbstractHeap`'s constructor, destructor, heap set-up and tear-down (the
//! class by its vtable `010a2bbc`, RTTI `.?AVAbstractHeap@@`). The class's
//! own unit is `BSCore/abstractheap.cpp`, so its fields are read and
//! written at their offsets, named after the Xbox PDB members; the PC
//! offsets match the PDB's up to the critical section at +0x20d8.

#[allow(unused_imports)]
use crate::prelude::*;

/// `IMemoryHeap`'s vtable (RTTI `.?AVIMemoryHeap@@`) and `AbstractHeap`'s
/// (RTTI `.?AVAbstractHeap@@`).
const I_MEMORY_HEAP_VTABLE: u32 = 0x010a_28c8;
const ABSTRACT_HEAP_VTABLE: u32 = 0x010a_2bbc;
/// Import slots of `InitializeCriticalSection` and `DeleteCriticalSection`.
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// `memset(dest, value, count)` (LIBCMT, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;
/// `_exit(code)` (LIBCMT).
const EXIT: u32 = 0x00ec_cec5;
/// Removes a block from a free list (`thiscall(heap, list, block)`).
const FREE_LIST_REMOVE: u32 = 0x00aa_9e50;

/// `AbstractHeap` field offsets (Xbox PDB names, the PC offsets).
const NAME: u32 = 0x04; // pName
const DEFAULT_ALIGNMENT: u32 = 0x08; // iDefaultAlignment
const MIN_FREE_BLOCK_SIZE: u32 = 0x0c; // MinFreeBlockSize
const PAGE_SIZE: u32 = 0x10; // iPageSize
const MEM_HEAP_SIZE: u32 = 0x14; // MemHeapSize
const INITIAL_SIZE: u32 = 0x18; // iInitialSize
const CURRENT_SIZE: u32 = 0x1c; // iCurrentSize
const MEM_HEAP: u32 = 0x30; // pMemHeap
const SMALL_FREE_LISTS: u32 = 0x44; // pSmallFreeList: 0x400 lists of 8 bytes
const LAST_SMALL_LIST: u32 = 0x2044; // pLastSmallList
const LARGE_FREE_LIST: u32 = 0x2048; // LargeFreeList (8 bytes)
const DELAYED_FREE_BLOCKS: u32 = 0x2050; // pDelayedFreeBlocksA
const NEXT_DELAYED_FREE_INDEX: u32 = 0x2054; // iNextDelayedFreeIndex
const SUB_BUCKETS: u32 = 0x2058; // pSubBuckets: 16 of 8 bytes
const CRITICAL_SECTION: u32 = 0x20d8; // CriticalSectionO

// Translated from 00aa8ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AbstractHeap::AbstractHeap(size, initial_size, name, page_size)` (the
/// class by its vtable): the `IMemoryHeap` then the `AbstractHeap` vtable;
/// `pName`, `iDefaultAlignment` = 4, `iPageSize`, `MemHeapSize`,
/// `iInitialSize` and `iCurrentSize` (both `initial_size`); the nine words
/// +0x20..+0x40 cleared; the critical section initialized;
/// `MinFreeBlockSize` = 0x10 (8 + 8); the delayed-free fields cleared.
/// Returns `this`.
pub fn abstract_heap_construct(
    e: &mut Engine,
    this: Ptr,
    size: u32,
    initial_size: u32,
    name: u32,
    page_size: u32,
) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, I_MEMORY_HEAP_VTABLE);
    e.mem.set_u32(at, ABSTRACT_HEAP_VTABLE);
    e.mem.set_u32(at + NAME, name);
    e.mem.set_u32(at + DEFAULT_ALIGNMENT, 4);
    e.mem.set_u32(at + PAGE_SIZE, page_size);
    e.mem.set_u32(at + MEM_HEAP_SIZE, size);
    e.mem.set_u32(at + INITIAL_SIZE, initial_size);
    e.mem.set_u32(at + CURRENT_SIZE, initial_size);
    for offset in (0x20..=0x40).step_by(4) {
        e.mem.set_u32(at + offset, 0);
    }
    e.call(INITIALIZE_CRITICAL_SECTION, &args![at + CRITICAL_SECTION]);
    e.mem.set_u32(at + MIN_FREE_BLOCK_SIZE, 8 + 8);
    e.mem.set_u32(at + DELAYED_FREE_BLOCKS, 0);
    e.mem.set_u32(at + NEXT_DELAYED_FREE_INDEX, 0);
    this
}

// Translated from 00aa8da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AbstractHeap::~AbstractHeap` (the class by its vtable): deletes the
/// critical section between restoring the `AbstractHeap` and the
/// `IMemoryHeap` vtables.
pub fn abstract_heap_destruct(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, ABSTRACT_HEAP_VTABLE);
    e.call(DELETE_CRITICAL_SECTION, &args![at + CRITICAL_SECTION]);
    e.mem.set_u32(at, I_MEMORY_HEAP_VTABLE);
}

// Translated from 00aa8dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The heap set-up (`thiscall`): gets the heap's memory from its vtable
/// slot `+0x30` (given `MemHeapSize` and `iCurrentSize`) into `pMemHeap`,
/// exiting the process (`_exit(0)`) when there is none; points
/// `pLastSmallList` at the last small list (+0x203c); gives each of the 16
/// sub-buckets a zero count and the address of its first small list (64
/// lists apart); clears the 0x400 small lists and the large list.
pub fn fn_00aa8dd0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let size = e.mem.u32(at + MEM_HEAP_SIZE);
    let current = e.mem.u32(at + CURRENT_SIZE);
    let memory = e.vcall(at, 0x30, &args![size, current]).u32();
    e.mem.set_u32(at + MEM_HEAP, memory);
    if e.mem.u32(at + MEM_HEAP) == 0 {
        e.call(EXIT, &args![0u32]);
    }
    e.mem
        .set_u32(at + LAST_SMALL_LIST, at + SMALL_FREE_LISTS + 0x3ff * 8);
    let lists_per_bucket: u32 = 0x40;
    for bucket in 0..0x10u32 {
        e.mem.set_u32(at + SUB_BUCKETS + bucket * 8, 0);
        let first_list = at + SMALL_FREE_LISTS + bucket * lists_per_bucket * 8;
        e.mem.set_u32(at + SUB_BUCKETS + bucket * 8 + 4, first_list);
    }
    e.call(MEMSET, &args![at + SMALL_FREE_LISTS, 0u32, 0x2000u32]);
    e.call(MEMSET, &args![at + LARGE_FREE_LIST, 0u32, 8u32]);
}

// Translated from 00aa8ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The heap tear-down (`thiscall`): empties each of the 0x400 small free
/// lists and then the large one by removing their second word's block
/// (`00aa9e50(list, block)`) until it is null, hands `pMemHeap` to the
/// heap's vtable slot `+0x34`, and clears `pMemHeap` and `MemHeapSize`.
pub fn fn_00aa8ea0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    for index in 0..0x400u32 {
        let list = at + SMALL_FREE_LISTS + index * 8;
        while e.mem.u32(list + 4) != 0 {
            let block = e.mem.u32(list + 4);
            e.call(FREE_LIST_REMOVE, &args![this, list, block]);
        }
    }
    while e.mem.u32(at + LARGE_FREE_LIST + 4) != 0 {
        let block = e.mem.u32(at + LARGE_FREE_LIST + 4);
        e.call(FREE_LIST_REMOVE, &args![this, at + LARGE_FREE_LIST, block]);
    }
    let memory = e.mem.u32(at + MEM_HEAP);
    e.vcall(at, 0x34, &args![memory]);
    e.mem.set_u32(at + MEM_HEAP, 0);
    e.mem.set_u32(at + MEM_HEAP_SIZE, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa8ca0,
            abstract_heap_construct(Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00aa8da0, abstract_heap_destruct(Ptr)),
        entry!(0x00aa8dd0, fn_00aa8dd0(Ptr)),
        entry!(0x00aa8ea0, fn_00aa8ea0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAP_SIZE: u32 = 0x20f4;

    fn no_op(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructor_sets_the_fields() {
        let mut e = Engine::new();
        e.register(INITIALIZE_CRITICAL_SECTION, no_op);
        let this = e.mem.alloc(HEAP_SIZE);
        for offset in (0..0x48).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.mem.set_u32(this + DELAYED_FREE_BLOCKS, 9);
        e.mem.set_u32(this + NEXT_DELAYED_FREE_INDEX, 9);
        e.call_log = Some(vec![]);
        let back = e.call(
            0x00aa_8ca0,
            &args![this, 0x100u32, 0x80u32, 0x5000u32, 0x10000u32],
        );
        assert_eq!(back.u32(), this);
        let words: Vec<u32> = (0..0x11).map(|i| e.mem.u32(this + 4 * i)).collect();
        let mut want = vec![
            ABSTRACT_HEAP_VTABLE,
            0x5000,
            4,
            0x10,
            0x10000,
            0x100,
            0x80,
            0x80,
        ];
        want.extend([0; 9]);
        assert_eq!(words, want);
        assert_eq!(e.mem.u32(this + DELAYED_FREE_BLOCKS), 0);
        assert_eq!(e.mem.u32(this + NEXT_DELAYED_FREE_INDEX), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, INITIALIZE_CRITICAL_SECTION),
            vec![vec![this + CRITICAL_SECTION]]
        );
    }

    #[test]
    fn destructor_deletes_the_section_and_restores_the_base_vtable() {
        let mut e = Engine::new();
        e.register(DELETE_CRITICAL_SECTION, |e, a| {
            // The AbstractHeap vtable is in place while it runs.
            assert_eq!(e.mem.u32(a[0] - CRITICAL_SECTION), ABSTRACT_HEAP_VTABLE);
            Ret::default()
        });
        let this = e.mem.alloc(HEAP_SIZE);
        e.call_log = Some(vec![]);
        e.call(0x00aa_8da0, &args![this]);
        assert_eq!(e.mem.u32(this), I_MEMORY_HEAP_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, DELETE_CRITICAL_SECTION).len(), 1);
    }

    /// A heap whose vtable has slots `+0x30` (answers `memory`) and `+0x34`.
    fn heap(e: &mut Engine, memory: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x30, 0x0f30_0030);
        e.mem.set_u32(vtable + 0x34, 0x0f30_0034);
        e.register_double(0x0f30_0030, move |_, _| memory.into_ret());
        e.register(0x0f30_0034, no_op);
        let this = e.mem.alloc(HEAP_SIZE);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn set_up_gets_memory_and_lays_out_the_lists() {
        let mut e = Engine::new();
        e.register(MEMSET, no_op);
        e.register(EXIT, no_op);
        let this = heap(&mut e, 0x4000_0000);
        e.mem.set_u32(this + MEM_HEAP_SIZE, 0x1000);
        e.mem.set_u32(this + CURRENT_SIZE, 0x800);
        e.call_log = Some(vec![]);
        e.call(0x00aa_8dd0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0f30_0030), vec![vec![this, 0x1000, 0x800]]);
        assert!(calls(&log, EXIT).is_empty());
        assert_eq!(e.mem.u32(this + MEM_HEAP), 0x4000_0000);
        assert_eq!(e.mem.u32(this + LAST_SMALL_LIST), this + 0x203c);
        for bucket in 0..16 {
            assert_eq!(e.mem.u32(this + SUB_BUCKETS + bucket * 8), 0);
            assert_eq!(
                e.mem.u32(this + SUB_BUCKETS + bucket * 8 + 4),
                this + 0x44 + bucket * 0x200
            );
        }
        assert_eq!(
            calls(&log, MEMSET),
            vec![vec![this + 0x44, 0, 0x2000], vec![this + 0x2048, 0, 8]]
        );
        // No memory: exits (the double returns, so the rest still runs).
        let this = heap(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_8dd0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, EXIT), vec![vec![0]]);
    }

    #[test]
    fn tear_down_empties_the_lists_and_frees_the_memory() {
        let mut e = Engine::new();
        // Removing the block clears the list's second word.
        e.register(FREE_LIST_REMOVE, |e, a| {
            e.mem.set_u32(a[1] + 4, 0);
            Ret::default()
        });
        let this = heap(&mut e, 0);
        e.mem.set_u32(this + SMALL_FREE_LISTS + 5 * 8 + 4, 0x1234);
        e.mem.set_u32(this + LARGE_FREE_LIST + 4, 0x5678);
        e.mem.set_u32(this + MEM_HEAP, 0x4000_0000);
        e.mem.set_u32(this + MEM_HEAP_SIZE, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x00aa_8ea0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, FREE_LIST_REMOVE),
            vec![
                vec![this, this + SMALL_FREE_LISTS + 5 * 8, 0x1234],
                vec![this, this + LARGE_FREE_LIST, 0x5678]
            ]
        );
        assert_eq!(calls(&log, 0x0f30_0034), vec![vec![this, 0x4000_0000]]);
        assert_eq!(e.mem.u32(this + MEM_HEAP), 0);
        assert_eq!(e.mem.u32(this + MEM_HEAP_SIZE), 0);
    }
}

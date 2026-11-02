//! `BSCore/memorymanager.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bscore::memorypool::{MemoryPool, POOLS_BY_ADDRESS};
use crate::units::platform::MEMORY_MANAGER;

// Translated from 00aa4030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocation wrapper: forwards only the size but consumes all four stack
/// words (RET 0x10). The final three arguments are unread by this body.
pub fn allocate_with_context(
    e: &mut Engine,
    this: Ptr,
    size: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> Ptr {
    e.call(0x00aa_3e40, &args![this, size]).ptr()
}

/// The CRT `free` (`_free`, `00ecd291`, cdecl, one word).
const CRT_FREE: u32 = 0x00ec_d291;

// Translated from 00aa42c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the sibling of `00aa4030`, the allocation wrapper above): thiscall with two
/// stack words (`RET 8`), the memory manager as `this`. It passes the first word, the block, to
/// the CRT `free` and returns nothing; `this` and the second word are never read.
pub fn fn_00aa42c0(e: &mut Engine, _this: Ptr, block: Ptr, _unused_2: u32) {
    e.call(CRT_FREE, &args![block]);
}

layout! {
    /// `MemoryManager` (Xbox PDB, 0x34c bytes there). The PC build has no `m_rlog`,
    /// `pSysmemBlockList`, `pAllocationLog`, `pUsageTracker` or `kContextTracker`, so every
    /// field below sits 0x40 (`bInitialized`) or 0x20c (the rest) lower than in the PDB, as
    /// the PC code's offsets show; the size is the PDB's less 0x20c (not confirmed on the PC).
    pub struct MemoryManager: 0x140 {
        /// `bInitialized` (Xbox PDB).
        0x000 bInitialized: u8,
        /// `iNumHeaps` (Xbox PDB).
        0x002 iNumHeaps: u16,
        /// `iNumPhysicalHeaps` (Xbox PDB).
        0x004 iNumPhysicalHeaps: u16,
        /// `ppHeaps` (Xbox PDB): `IMemoryHeap**`.
        0x008 ppHeaps: u32,
        /// `ppPhysicalHeaps` (Xbox PDB): `IMemoryHeap**`.
        0x124 ppPhysicalHeaps: u32,
        /// `bAllowPoolUse` (Xbox PDB).
        0x129 bAllowPoolUse: u8,
    }
}

/// `pHeapsByContextA` (Xbox PDB): the 0x108 bytes at `+0xc` the teardown clears.
const HEAPS_BY_CONTEXT: u32 = 0x00c;
const HEAPS_BY_CONTEXT_BYTES: u32 = 0x108;
/// `kThreadScrapHeapMap` (Xbox PDB, a `BSMap<unsigned long,ScrapHeap *>`): its hash size at
/// `+4` and bucket array at `+8`.
const THREAD_SCRAP_HEAP_MAP: u32 = 0x114;
/// The thread block's word at `+0x2b4` (the allocation context the memory manager saves, sets
/// and restores).
const TLS_CONTEXT: u32 = 0x2b4;

/// `MemoryManager::Allocate` (`00aa3e40`, thiscall, one word) and `Deallocate` (`00aa4060`,
/// thiscall, one word), both platform stand-ins.
const ALLOCATE: u32 = 0x00aa_3e40;
const DEALLOCATE: u32 = 0x00aa_4060;
/// `_memset` (`00ec61c0`) and `_memcpy` (`00ec44d0`), cdecl, three words.
const MEMSET: u32 = 0x00ec_61c0;
const MEMCPY: u32 = 0x00ec_44d0;
/// `00483710` (thiscall): the empty base destructor body the map puts in `TESForm`'s unit.
const EMPTY_BASE_00483710: u32 = 0x0048_3710;

/// `MemoryPool::InitPools` (`00aa6840`), `MemoryManager::SpecifyMemoryLayout` (`00866e00`,
/// thiscall) and `MemoryManager::SpecifyPools` (`00866770`, thiscall), the map's names.
const MEMORY_POOL_INIT_POOLS: u32 = 0x00aa_6840;
const SPECIFY_MEMORY_LAYOUT: u32 = 0x0086_6e00;
const SPECIFY_POOLS: u32 = 0x0086_6770;

// Translated from 00aa39c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryManager::Initialize` (Xbox PDB): `MemoryPool::InitPools` (called with `ECX` still
/// `this`, which it does not take), `SpecifyMemoryLayout`, `bInitialized = 1`,
/// `SpecifyPools`, `00483710(this)`, and 0 in the thread block's word at `+0x2b4`.
pub fn memory_manager_initialize(e: &mut Engine, this: Ptr<MemoryManager>) {
    e.call(MEMORY_POOL_INIT_POOLS, &args![]);
    e.call(SPECIFY_MEMORY_LAYOUT, &args![this]);
    e.set(this, MemoryManager::bInitialized, 1);
    e.call(SPECIFY_POOLS, &args![this]);
    e.call(EMPTY_BASE_00483710, &args![this]);
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_CONTEXT, 0);
}

/// `00a0c850` (thiscall, three words: the position, the key and the value out-parameters;
/// the map iterator step), `ScrapHeap`'s destructor body `00aa5460` (thiscall), `00ae91f0`
/// (thiscall on the map; unnamed), `00aa5950` and `00aa68a0` (no arguments) and `00aa4a60`
/// (thiscall on the map: the map's destructor shape).
const MAP_STEP_00A0C850: u32 = 0x00a0_c850;
const SCRAP_HEAP_DTOR_00AA5460: u32 = 0x00aa_5460;
const MAP_CLEAR_00AE91F0: u32 = 0x00ae_91f0;
const RELEASE_00AA5950: u32 = 0x00aa_5950;
const RELEASE_POOLS_00AA68A0: u32 = 0x00aa_68a0;
const MAP_DTOR_00AA4A60: u32 = 0x00aa_4a60;

/// Deletes each non-null heap of the array at `array_field` (as many as the `u16` at
/// `count_field`, read for every step) through its vtable slot 0 with 1, clears each entry
/// (the array pointer read again for every access), then hands the array to `Deallocate` on
/// the singleton. Nothing happens when the array pointer is null.
fn delete_heaps(
    e: &mut Engine,
    this: Ptr<MemoryManager>,
    array_field: Field<MemoryManager, u32>,
    count_field: Field<MemoryManager, u16>,
) {
    if e.get(this, array_field) == 0 {
        return;
    }
    let mut index: u16 = 0;
    while index < e.get(this, count_field) {
        let heap = e.mem.u32(e.get(this, array_field) + index as u32 * 4);
        if heap != 0 {
            e.vcall(heap, 0, &args![1u32]);
        }
        let slot = e.get(this, array_field) + index as u32 * 4;
        e.mem.set_u32(slot, 0);
        index = index.wrapping_add(1);
    }
    let array = e.get(this, array_field);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, array]);
}

// Translated from 00aa3a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aa3a40 - the compiler's exception frame (FS:[0] chain, state at EBP-4, handler 00f1ec47) is not translated
/// Unnamed in the map: the memory manager's teardown (thiscall). Finds the first non-empty
/// bucket of the thread scrap-heap map, then steps through the map with `00a0c850` (position,
/// key and value in a 12-byte stack frame) and, for each non-null `ScrapHeap`, runs
/// `00aa5460` and hands it to `Deallocate` on the singleton. Then `00ae91f0` on the map,
/// `00aa5950()`, `00483710(this)`, `00aa68a0()`, clears `pHeapsByContextA` (0x108 bytes) and
/// `bInitialized`, deletes the `iNumHeaps` heaps of `ppHeaps` and the `iNumPhysicalHeaps` of
/// `ppPhysicalHeaps` (slot 0 with 1) and frees both arrays, and ends with `00aa4a60` on the
/// map. The compiler folded the delete flag of the scrap heaps' destructor to a constant 1, so
/// they are always freed.
pub fn fn_00aa3a40(e: &mut Engine, this: Ptr<MemoryManager>) {
    let map = this.addr() + THREAD_SCRAP_HEAP_MAP;
    let mut first = 0;
    let mut bucket = 0u32;
    while bucket < e.mem.u32(map + 4) {
        let entry = e.mem.u32(e.mem.u32(map + 8) + bucket * 4);
        if entry != 0 {
            first = entry;
            break;
        }
        bucket += 1;
    }
    e.with_stack(0xc, |e, frame| {
        // EBP-0x10: the position; EBP-0x14: the key; EBP-0x18: the value.
        let (position, key, value) = (frame.addr() + 8, frame.addr() + 4, frame.addr());
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.call(MAP_STEP_00A0C850, &args![map, position, key, value]);
            let heap = e.mem.u32(value);
            if heap != 0 {
                e.call(SCRAP_HEAP_DTOR_00AA5460, &args![heap]);
                e.call(DEALLOCATE, &args![MEMORY_MANAGER, heap]);
            }
        }
    });
    e.call(MAP_CLEAR_00AE91F0, &args![map]);
    e.call(RELEASE_00AA5950, &args![]);
    e.call(EMPTY_BASE_00483710, &args![this]);
    e.call(RELEASE_POOLS_00AA68A0, &args![]);
    e.call(
        MEMSET,
        &args![this.addr() + HEAPS_BY_CONTEXT, 0u32, HEAPS_BY_CONTEXT_BYTES],
    );
    e.set(this, MemoryManager::bInitialized, 0);
    delete_heaps(e, this, MemoryManager::ppHeaps, MemoryManager::iNumHeaps);
    delete_heaps(
        e,
        this,
        MemoryManager::ppPhysicalHeaps,
        MemoryManager::iNumPhysicalHeaps,
    );
    e.call(MAP_DTOR_00AA4A60, &args![map]);
}

/// The `MemoryPool` constructor (`00aa66a0`, thiscall, three words) and destructor body
/// (`00aa6800`, thiscall).
const MEMORY_POOL_CTOR_00AA66A0: u32 = 0x00aa_66a0;
const MEMORY_POOL_DTOR_00AA6800: u32 = 0x00aa_6800;

// Translated from 00aa3cc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aa3cc0 - the compiler's exception frame (FS:[0] chain, state at EBP-4, handler 00f1ec75) is not translated
/// Unnamed in the map: creates a `MemoryPool` (thiscall, three words: the block size and two
/// words for the pool's constructor). Saves the thread block's context word (`+0x2b4`) and
/// sets it to 3; a block size below 8 only restores it. Otherwise it clears `bAllowPoolUse`,
/// allocates 0x80 bytes from the singleton, constructs the pool there (`00aa66a0(size, first,
/// second)`, 0 when the allocation failed) and, when the pool's `pHeap` (`+4`) is 0, destroys
/// it (`00aa6800`, then `Deallocate`; the delete flag is folded to 1). Then it sets
/// `bAllowPoolUse` and restores the context word. The pool's `+4` is read even when the
/// allocation failed, as the code does.
pub fn fn_00aa3cc0(
    e: &mut Engine,
    this: Ptr<MemoryManager>,
    block_size: u32,
    first: u32,
    second: u32,
) {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, 3);
    if block_size < 8 {
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, saved);
        return;
    }
    e.set(this, MemoryManager::bAllowPoolUse, 0);
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x80u32]).u32();
    let pool = if block != 0 {
        e.call(
            MEMORY_POOL_CTOR_00AA66A0,
            &args![block, block_size, first, second],
        )
        .u32()
    } else {
        0
    };
    if e.mem.u32(pool.wrapping_add(4)) == 0 && pool != 0 {
        e.call(MEMORY_POOL_DTOR_00AA6800, &args![pool]);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, pool]);
    }
    e.set(this, MemoryManager::bAllowPoolUse, 1);
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
}

/// `_realloc` (`00eccf5d`, cdecl: block, size).
const REALLOC: u32 = 0x00ec_cf5d;

// Translated from 00aa4150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: reallocation (thiscall, two words: the block and the new size, `RET
/// 8`). When `00aa44c0` knows the block's size, allocates `size` bytes with `Allocate` (on
/// `this`), copies the smaller of the two sizes, and frees the old block with `Deallocate` (on
/// `this`); otherwise it is `_realloc(block, size)`. Returns the new block.
pub fn fn_00aa4150(e: &mut Engine, this: Ptr<MemoryManager>, block: u32, size: u32) -> u32 {
    let mut old_size = fn_00aa44c0(e, this, block);
    if old_size == 0 {
        return e.call(REALLOC, &args![block, size]).u32();
    }
    let new_block = e.call(ALLOCATE, &args![this, size]).u32();
    if old_size > size {
        old_size = size;
    }
    e.call(MEMCPY, &args![new_block, block, old_size]);
    e.call(DEALLOCATE, &args![this, block]);
    new_block
}

/// `_malloc` (`00ecd1c7`, cdecl, one word).
const MALLOC: u32 = 0x00ec_d1c7;

// Translated from 00aa4290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (the sibling of `00aa42c0`): thiscall with two stack words (`RET 8`);
/// returns `_malloc(size)`. `this` and the second word are never read.
pub fn fn_00aa4290(e: &mut Engine, _this: Ptr, size: u32, _unused_2: u32) -> u32 {
    e.call(MALLOC, &args![size]).u32()
}

/// The import slot of KERNEL32 `GetCurrentThreadId` (`00fdf1d0`).
const GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
/// `00a1e380` (thiscall, two words: the key and the value out-parameter; the map lookup) and
/// `BSMapBase<void*,BlockMemPage*>::SetAt` (`00aa4860`, thiscall, two words; the map's name for
/// the shared body).
const MAP_GET_AT_00A1E380: u32 = 0x00a1_e380;
const MAP_SET_AT_00AA4860: u32 = 0x00aa_4860;
/// `ScrapHeap::ScrapHeap` (`00aa53f0`, thiscall; the map's name).
const SCRAP_HEAP_CTOR: u32 = 0x00aa_53f0;

// Translated from 00aa42e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aa42e0 - the compiler's exception frame (FS:[0] chain, state at EBP-4, handler 00f1ec9b) is not translated
/// `MemoryManager::GetThreadScrapHeap` (Xbox PDB): looks up the current thread's id in the
/// thread scrap-heap map (`00a1e380`, the result in a stack word cleared first); when there
/// is none, allocates 0x10 bytes from the singleton, constructs a `ScrapHeap` there (0 when
/// the allocation failed) and stores it in the map (`00aa4860`). Returns the heap.
pub fn memory_manager_get_thread_scrap_heap(e: &mut Engine, this: Ptr<MemoryManager>) -> Ptr {
    let thread = e.call(GET_CURRENT_THREAD_ID, &args![]).u32();
    let map = this.addr() + THREAD_SCRAP_HEAP_MAP;
    e.with_stack(4, |e, heap_at| {
        e.mem.set_u32(heap_at.addr(), 0);
        e.call(MAP_GET_AT_00A1E380, &args![map, thread, heap_at]);
        if e.mem.u32(heap_at.addr()) == 0 {
            let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x10u32]).u32();
            let heap = if block != 0 {
                e.call(SCRAP_HEAP_CTOR, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(heap_at.addr(), heap);
            let heap = e.mem.u32(heap_at.addr());
            e.call(MAP_SET_AT_00AA4860, &args![map, thread, heap]);
        }
        Ptr::new(e.mem.u32(heap_at.addr()))
    })
}

/// `00aa45a0` and `00aa4610` (thiscall, one word each: the heap that holds a block, or 0) and
/// `__msize` (`00ecd31f`, cdecl, one word).
const HEAP_FOR_BLOCK_00AA45A0: u32 = 0x00aa_45a0;
const HEAP_FOR_BLOCK_00AA4610: u32 = 0x00aa_4610;
const MSIZE: u32 = 0x00ec_d31f;

// Translated from 00aa44c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: the size of a block (thiscall, one word, `RET 4`). When the pool for
/// the block's 16 MiB (`MemoryPool::pMemoryPoolsByAddress[block >> 24]`) holds it (`pHeap <=
/// block < pHeap + iPoolSize`), the pool's `iAlignedBlockSize`; else, when `00aa45a0` or else
/// `00aa4610` finds a heap for it, that heap's vtable slot `0x24` with the block and 0; else
/// `__msize(block)`.
pub fn fn_00aa44c0(e: &mut Engine, this: Ptr<MemoryManager>, block: u32) -> u32 {
    let pool: Ptr<MemoryPool> = Ptr::new(e.mem.u32(POOLS_BY_ADDRESS + (block >> 24) * 4));
    if !pool.is_null() {
        let start = e.get(pool, MemoryPool::pHeap);
        let inside = block >= start && {
            let start = e.get(pool, MemoryPool::pHeap);
            let size = e.get(pool, MemoryPool::iPoolSize);
            block < start.wrapping_add(size)
        };
        if inside {
            return e.get(pool, MemoryPool::iAlignedBlockSize);
        }
    }
    let heap = e.call(HEAP_FOR_BLOCK_00AA45A0, &args![this, block]).u32();
    if heap != 0 {
        return e.vcall(heap, 0x24, &args![block, 0u32]).u32();
    }
    let heap = e.call(HEAP_FOR_BLOCK_00AA4610, &args![this, block]).u32();
    if heap != 0 {
        return e.vcall(heap, 0x24, &args![block, 0u32]).u32();
    }
    e.call(MSIZE, &args![block]).u32()
}

// Translated from 00aa3a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no code references it; `thiscall`, no stack words): clears the
/// thread block's word at +0x2b4 (the allocation context, `*(*(FS:[0x2c]) + _tls_index * 4)`
/// with the index at `0126fd98`) and sets the byte at `this` to 1. Answers `this` (left in
/// `EAX`).
pub fn fn_00aa3a10(e: &mut Engine, this: Ptr) -> Ptr {
    let thread = e.tls();
    e.mem.set_u32(thread + TLS_CONTEXT, 0);
    e.mem.set_u8(this.addr(), 1);
    this
}

// Translated from 00aa41e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no code references it; `thiscall`, two stack words it does
/// not read, `RET 8`): answers 0.
pub fn fn_00aa41e0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0
}

// Translated from 00aa41f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no code references it; `thiscall`, three stack words it does
/// not read, `RET 0xc`): does nothing.
pub fn fn_00aa41f0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32, _unused_3: u32) {}

// Translated from 00aa4200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: the reallocation the `Reallocate` wrapper at `00aa2340` calls
/// (`thiscall`, five stack words, `RET 0x14`; only the block and the new size are read).
/// When `00aa44c0` knows the block's size, allocates `size` bytes with `Allocate` (on
/// `this`), copies the smaller of the two sizes and frees the old block with `Deallocate`
/// (on `this`); otherwise it is `_realloc(block, size)`. Returns the new block.
pub fn fn_00aa4200(
    e: &mut Engine,
    this: Ptr<MemoryManager>,
    block: u32,
    size: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let mut old_size = fn_00aa44c0(e, this, block);
    if old_size == 0 {
        return e.call(REALLOC, &args![block, size]).u32();
    }
    let new_block = e.call(ALLOCATE, &args![this, size]).u32();
    if old_size > size {
        old_size = size;
    }
    e.call(MEMCPY, &args![new_block, block, old_size]);
    e.call(DEALLOCATE, &args![this, block]);
    new_block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_3a10, fn_00aa3a10(Ptr) -> Ptr),
        entry!(0x00aa_41e0, fn_00aa41e0(Ptr, u32, u32) -> u32),
        entry!(0x00aa_41f0, fn_00aa41f0(Ptr, u32, u32, u32)),
        entry!(
            0x00aa_4200,
            fn_00aa4200(Ptr<MemoryManager>, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00aa4030, allocate_with_context(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00aa42c0, fn_00aa42c0(Ptr, Ptr, u32)),
        entry!(0x00aa_39c0, memory_manager_initialize(Ptr<MemoryManager>)),
        entry!(0x00aa_3a40, fn_00aa3a40(Ptr<MemoryManager>)),
        entry!(0x00aa_3cc0, fn_00aa3cc0(Ptr<MemoryManager>, u32, u32, u32)),
        entry!(
            0x00aa_4150,
            fn_00aa4150(Ptr<MemoryManager>, u32, u32) -> u32
        ),
        entry!(0x00aa_4290, fn_00aa4290(Ptr, u32, u32) -> u32),
        entry!(
            0x00aa_42e0,
            memory_manager_get_thread_scrap_heap(Ptr<MemoryManager>) -> Ptr
        ),
        entry!(0x00aa_44c0, fn_00aa44c0(Ptr<MemoryManager>, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wrapper_consumes_all_words_but_forwards_only_receiver_and_size() {
        let mut e = Engine::new();
        e.register_double(0x00aa3e40, |_, args| {
            assert_eq!(args, &[0x1234, 0x89]);
            0u32.into_ret()
        });
        assert_eq!(e.call(0x00aa4030, &[0x1234, 0x89, 3, 4, 5]).u32(), 0);
    }
}

#[cfg(test)]
mod physics_step_tests {
    use super::*;

    #[test]
    fn free_wrapper_passes_only_the_block_to_the_crt_free() {
        let mut e = Engine::new();
        e.register(CRT_FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let result = e.call(0x00aa42c0, &args![0x1111u32, 0x2222u32, 0x3333u32]);
        assert_eq!(result.u32(), 0);
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|c| c.0 == CRT_FREE).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, vec![0x2222]);
    }

    #[test]
    fn free_wrapper_frees_a_real_heap_block() {
        let mut e = Engine::new();
        let block = e.mem.alloc(32);
        assert!(e.mem.block_size(block).is_some());
        e.call(0x00aa42c0, &args![0u32, block, 0u32]);
        assert!(e.mem.block_size(block).is_none());
    }
}

#[cfg(test)]
mod b001_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, a| a.first().copied().unwrap_or(0).into_ret());
        }
    }

    fn manager(e: &mut Engine) -> Ptr<MemoryManager> {
        let this = e.mem.alloc(0x140);
        e.mem.write(this, &[0; 0x140]);
        Ptr::new(this)
    }

    #[test]
    fn initialize_sets_up_the_pools_and_clears_the_context() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                MEMORY_POOL_INIT_POOLS,
                SPECIFY_MEMORY_LAYOUT,
                SPECIFY_POOLS,
                EMPTY_BASE_00483710,
            ],
        );
        e.register(SPECIFY_POOLS, |e, a| {
            assert_eq!(e.mem.u8(a[0]), 1, "bInitialized before SpecifyPools");
            Ret::default()
        });
        let this = manager(&mut e);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        e.call_log = Some(vec![]);
        e.call(0x00aa_39c0, &args![this]);
        assert_eq!(
            calls_to(&e, MEMORY_POOL_INIT_POOLS),
            vec![Vec::<u32>::new()]
        );
        assert_eq!(calls_to(&e, SPECIFY_MEMORY_LAYOUT), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, SPECIFY_POOLS), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, EMPTY_BASE_00483710), vec![vec![this.addr()]]);
        assert_eq!(e.get(this, MemoryManager::bInitialized), 1);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 0);
    }

    /// A heap object whose vtable slot 0 is the recorded double `0x0050_0100`.
    fn heap(e: &mut Engine, vtable: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn teardown_frees_the_scrap_heaps_and_both_heap_arrays() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                SCRAP_HEAP_DTOR_00AA5460,
                DEALLOCATE,
                MAP_CLEAR_00AE91F0,
                RELEASE_00AA5950,
                EMPTY_BASE_00483710,
                RELEASE_POOLS_00AA68A0,
                MEMSET,
                MAP_DTOR_00AA4A60,
                0x0050_0100,
            ],
        );
        let this = manager(&mut e);
        let map = this.addr() + THREAD_SCRAP_HEAP_MAP;
        // Three buckets, the second holding the first item; the double walks two items and
        // gives a null heap for the second.
        let buckets = e.mem.alloc(12);
        e.mem.write(buckets, &[0; 12]);
        e.mem.set_u32(buckets + 4, 0x7100);
        e.mem.set_u32(map + 4, 3);
        e.mem.set_u32(map + 8, buckets);
        e.register_double(MAP_STEP_00A0C850, |e, a| {
            let position = e.mem.u32(a[1]);
            let (next, value) = if position == 0x7100 {
                (0x7200, 0x8000)
            } else {
                (0, 0)
            };
            e.mem.set_u32(a[1], next);
            e.mem.set_u32(a[2], 0x1d);
            e.mem.set_u32(a[3], value);
            Ret::default()
        });
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, 0x0050_0100);
        let (h1, h2) = (heap(&mut e, vtable), heap(&mut e, vtable));
        let heaps = e.mem.alloc(12);
        e.mem.set_u32(heaps, h1);
        e.mem.set_u32(heaps + 4, 0);
        e.mem.set_u32(heaps + 8, h2);
        e.set(this, MemoryManager::ppHeaps, heaps);
        e.set(this, MemoryManager::iNumHeaps, 3);
        e.set(this, MemoryManager::bInitialized, 1);
        e.call_log = Some(vec![]);
        e.call(0x00aa_3a40, &args![this]);
        let steps = calls_to(&e, MAP_STEP_00A0C850);
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0][0], map);
        assert_eq!(calls_to(&e, SCRAP_HEAP_DTOR_00AA5460), vec![vec![0x8000]]);
        assert_eq!(
            calls_to(&e, DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x8000], vec![MEMORY_MANAGER, heaps]]
        );
        assert_eq!(calls_to(&e, MAP_CLEAR_00AE91F0), vec![vec![map]]);
        assert_eq!(
            calls_to(&e, MEMSET),
            vec![vec![this.addr() + 0xc, 0, 0x108]]
        );
        assert_eq!(calls_to(&e, 0x0050_0100), vec![vec![h1, 1], vec![h2, 1]]);
        assert_eq!(e.mem.bytes(heaps, 12), vec![0; 12]);
        assert_eq!(e.get(this, MemoryManager::bInitialized), 0);
        assert_eq!(calls_to(&e, MAP_DTOR_00AA4A60), vec![vec![map]]);
    }

    #[test]
    fn teardown_with_an_empty_map_and_no_arrays_skips_the_walk() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                MAP_CLEAR_00AE91F0,
                RELEASE_00AA5950,
                EMPTY_BASE_00483710,
                RELEASE_POOLS_00AA68A0,
                MEMSET,
                MAP_DTOR_00AA4A60,
                DEALLOCATE,
            ],
        );
        e.register(MAP_STEP_00A0C850, |_, _| panic!("no item to step to"));
        let this = manager(&mut e);
        let buckets = e.mem.alloc(8);
        e.mem.write(buckets, &[0; 8]);
        e.mem.set_u32(this.addr() + THREAD_SCRAP_HEAP_MAP + 4, 2);
        e.mem
            .set_u32(this.addr() + THREAD_SCRAP_HEAP_MAP + 8, buckets);
        e.call_log = Some(vec![]);
        e.call(0x00aa_3a40, &args![this]);
        assert!(calls_to(&e, DEALLOCATE).is_empty());
        assert_eq!(calls_to(&e, MAP_DTOR_00AA4A60).len(), 1);
    }

    #[test]
    fn pool_creation_keeps_a_reserved_pool_and_destroys_an_empty_one() {
        for reserved in [true, false] {
            let mut e = Engine::new();
            quiet(&mut e, &[MEMORY_POOL_DTOR_00AA6800, DEALLOCATE]);
            let pool = e.mem.alloc(0x80);
            e.mem.write(pool, &[0; 0x80]);
            e.register_double(ALLOCATE, move |_, _| pool.into_ret());
            let marker = if reserved { 0x0100_0000 } else { 0 };
            e.register_double(MEMORY_POOL_CTOR_00AA66A0, move |e, a| {
                let tls = e.tls();
                assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 3);
                e.mem.set_u32(a[0] + 4, marker);
                a[0].into_ret()
            });
            let this = manager(&mut e);
            let tls = e.tls();
            e.mem.set_u32(tls + TLS_CONTEXT, 5);
            e.call_log = Some(vec![]);
            e.call(0x00aa_3cc0, &args![this, 0x20u32, 0x11u32, 0x22u32]);
            assert_eq!(calls_to(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 0x80]]);
            assert_eq!(
                calls_to(&e, MEMORY_POOL_CTOR_00AA66A0),
                vec![vec![pool, 0x20, 0x11, 0x22]]
            );
            let destroyed = calls_to(&e, MEMORY_POOL_DTOR_00AA6800);
            if reserved {
                assert!(destroyed.is_empty());
            } else {
                assert_eq!(destroyed, vec![vec![pool]]);
                assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, pool]]);
            }
            assert_eq!(e.get(this, MemoryManager::bAllowPoolUse), 1);
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 5);
        }
    }

    #[test]
    fn pool_creation_below_eight_bytes_only_restores_the_context() {
        let mut e = Engine::new();
        let this = manager(&mut e);
        e.set(this, MemoryManager::bAllowPoolUse, 7);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 5);
        e.call_log = Some(vec![]);
        e.call(0x00aa_3cc0, &args![this, 7u32, 0u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 5);
        assert_eq!(e.get(this, MemoryManager::bAllowPoolUse), 7);
    }

    /// A pool for the 16 MiB at `0x0300_0000` holding `0x0300_0000..0x0300_1000`, blocks of
    /// 0x40 bytes.
    fn map_pool(e: &mut Engine) {
        e.map(POOLS_BY_ADDRESS & !0xfff, 0x1000);
        let pool = e.mem.alloc(0x80);
        e.mem.write(pool, &[0; 0x80]);
        e.mem.set_u32(pool + 4, 0x0300_0000);
        e.mem.set_u32(pool + 0x50, 0x1000);
        e.mem.set_u32(pool + 0x40, 0x40);
        e.mem.set_u32(POOLS_BY_ADDRESS + 3 * 4, pool);
    }

    #[test]
    fn block_size_from_the_pool_a_heap_or_msize() {
        let mut e = Engine::new();
        map_pool(&mut e);
        let this = manager(&mut e);
        assert_eq!(
            e.call(0x00aa_44c0, &args![this, 0x0300_0fc0u32]).u32(),
            0x40
        );
        // Past the pool's range: the heaps, then __msize.
        e.register(0x0050_0200, |_, a| (a[1] + 1).into_ret());
        let vtable = e.mem.alloc(0x28);
        e.mem.set_u32(vtable + 0x24, 0x0050_0200);
        let found = heap(&mut e, vtable);
        e.register_double(HEAP_FOR_BLOCK_00AA45A0, move |_, a| {
            if a[1] == 0x0300_1000 {
                found.into_ret()
            } else {
                Ret::default()
            }
        });
        e.register_double(HEAP_FOR_BLOCK_00AA4610, move |_, a| {
            if a[1] == 0x0500_0000 {
                found.into_ret()
            } else {
                Ret::default()
            }
        });
        e.register(MSIZE, |_, _| 0x99u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_44c0, &args![this, 0x0300_1000u32]).u32(),
            0x0300_1001
        );
        assert_eq!(calls_to(&e, 0x0050_0200), vec![vec![found, 0x0300_1000, 0]]);
        assert_eq!(
            e.call(0x00aa_44c0, &args![this, 0x0500_0000u32]).u32(),
            0x0500_0001
        );
        assert_eq!(
            e.call(0x00aa_44c0, &args![this, 0x0600_0000u32]).u32(),
            0x99
        );
        assert_eq!(calls_to(&e, MSIZE), vec![vec![0x0600_0000]]);
    }

    #[test]
    fn reallocate_copies_the_smaller_size_or_uses_realloc() {
        let mut e = Engine::new();
        map_pool(&mut e);
        let this = manager(&mut e);
        quiet(&mut e, &[MEMCPY, DEALLOCATE]);
        e.register(ALLOCATE, |_, _| 0x0400_0000u32.into_ret());
        e.register(REALLOC, |_, _| 0x0410_0000u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_4150, &args![this, 0x0300_0040u32, 0x30u32])
                .u32(),
            0x0400_0000
        );
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![this.addr(), 0x30]]);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![0x0400_0000, 0x0300_0040, 0x30]]
        );
        assert_eq!(
            calls_to(&e, DEALLOCATE),
            vec![vec![this.addr(), 0x0300_0040]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00aa_4150, &args![this, 0x0300_0040u32, 0x100u32]);
        assert_eq!(calls_to(&e, MEMCPY)[0][2], 0x40);
        // A block no pool or heap knows: _realloc.
        e.register(HEAP_FOR_BLOCK_00AA45A0, |_, _| Ret::default());
        e.register(HEAP_FOR_BLOCK_00AA4610, |_, _| Ret::default());
        e.register(MSIZE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_4150, &args![this, 0x0600_0000u32, 0x100u32])
                .u32(),
            0x0410_0000
        );
        assert_eq!(calls_to(&e, REALLOC), vec![vec![0x0600_0000, 0x100]]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn malloc_wrapper_passes_only_the_size() {
        let mut e = Engine::new();
        e.register(MALLOC, |_, a| (a[0] + 0x1000).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_4290, &args![1u32, 0x20u32, 3u32]).u32(),
            0x1020
        );
        assert_eq!(calls_to(&e, MALLOC), vec![vec![0x20]]);
    }

    #[test]
    fn thread_scrap_heap_is_looked_up_or_created_and_stored() {
        let mut e = Engine::new();
        e.register(GET_CURRENT_THREAD_ID, |_, _| 0x1234u32.into_ret());
        quiet(&mut e, &[MAP_SET_AT_00AA4860, SCRAP_HEAP_CTOR]);
        e.register(ALLOCATE, |_, _| 0x0700_0000u32.into_ret());
        e.register(MAP_GET_AT_00A1E380, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0, "cleared before the lookup");
            Ret::default()
        });
        let this = manager(&mut e);
        let map = this.addr() + THREAD_SCRAP_HEAP_MAP;
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_42e0, &args![this]).u32(), 0x0700_0000);
        assert_eq!(calls_to(&e, MAP_GET_AT_00A1E380)[0][..2], [map, 0x1234]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 0x10]]);
        assert_eq!(calls_to(&e, SCRAP_HEAP_CTOR), vec![vec![0x0700_0000]]);
        assert_eq!(
            calls_to(&e, MAP_SET_AT_00AA4860),
            vec![vec![map, 0x1234, 0x0700_0000]]
        );
        // Found in the map: returned as it is.
        e.register(MAP_GET_AT_00A1E380, |e, a| {
            e.mem.set_u32(a[2], 0x0710_0000);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_42e0, &args![this]).u32(), 0x0710_0000);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        // A failed allocation stores and returns a null heap.
        e.register(MAP_GET_AT_00A1E380, |_, _| Ret::default());
        e.register(ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_42e0, &args![this]).u32(), 0);
        assert!(calls_to(&e, SCRAP_HEAP_CTOR).is_empty());
        assert_eq!(
            calls_to(&e, MAP_SET_AT_00AA4860),
            vec![vec![map, 0x1234, 0]]
        );
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00aa3a10_clears_the_thread_context_and_marks_this() {
        let mut e = Engine::new();
        let thread = e.tls();
        e.mem.set_u32(thread + TLS_CONTEXT, 7);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0xffff_ff00);
        assert_eq!(e.call(0x00aa_3a10, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(thread + TLS_CONTEXT), 0);
        assert_eq!(e.mem.u32(this), 0xffff_ff01);
    }

    #[test]
    fn fn_00aa41e0_answers_zero_and_fn_00aa41f0_does_nothing() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00aa_41e0, &args![1u32, 2u32, 3u32]).u32(), 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_41f0, &args![1u32, 2u32, 3u32, 4u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_00aa4200_copies_the_smaller_size_or_uses_realloc() {
        let mut e = Engine::new();
        // No pool for the blocks; `00aa45a0` finds a heap whose slot 0x24 says 0x40 bytes
        // for 0x0300_0040, and none for other blocks.
        e.map(POOLS_BY_ADDRESS & !0xfff, 0x1000);
        let heap = e.mem.alloc(4);
        e.put_vtable(0x7000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0x8024]);
        e.mem.set_u32(heap, 0x7000);
        e.register(0x8024, |_, _| 0x40u32.into_ret());
        e.register_double(HEAP_FOR_BLOCK_00AA45A0, move |_, a| {
            if a[1] == 0x0300_0040 {
                heap.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(HEAP_FOR_BLOCK_00AA4610, |_, _| Ret::default());
        e.register(MSIZE, |_, _| Ret::default());
        for callee in [MEMCPY, DEALLOCATE] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |_, _| 0x0400_0000u32.into_ret());
        e.register(REALLOC, |_, _| 0x0410_0000u32.into_ret());
        let this = 0x0500_0000u32;
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x00aa_4200,
                &args![this, 0x0300_0040u32, 0x30u32, 7u32, 8u32, 9u32]
            )
            .u32(),
            0x0400_0000
        );
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![this, 0x30]]);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![0x0400_0000, 0x0300_0040, 0x30]]
        );
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![this, 0x0300_0040]]);
        e.call_log = Some(vec![]);
        e.call(
            0x00aa_4200,
            &args![this, 0x0300_0040u32, 0x100u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(calls_to(&e, MEMCPY)[0][2], 0x40);
        // A block nothing knows: _realloc.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x00aa_4200,
                &args![this, 0x0600_0000u32, 0x100u32, 0u32, 0u32, 0u32]
            )
            .u32(),
            0x0410_0000
        );
        assert_eq!(calls_to(&e, REALLOC), vec![vec![0x0600_0000, 0x100]]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
    }
}

//! `BSCore/memorypool.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `MemoryPool` hands out blocks of one size from a reserved address range
//! (`pHeap`), committing 4 KiB pages with `VirtualAlloc` as it needs them.
//! Free blocks form a doubly linked list (`FreePoolBlock`, written into the
//! free blocks themselves); a per-page `u16` counts the blocks in use, with
//! `0xffff` marking an uncommitted page. Two global tables find pools by
//! block size and by address (one entry per 16 MiB).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::platform::MEMORY_MANAGER;

/// `0040fbf0`: acquires a recursive lock (`this` = the lock; one word, the
/// caller's name). Its release is inlined in every locked function here.
const LOCK: u32 = 0x0040_fbf0;
/// `MemoryManager::Deallocate` (`00aa4060`, `this` = the memory manager).
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `00aa6800`: the pool's destructor body (another unit's range): releases
/// the pages and the page counts.
const POOL_DESTRUCTOR: u32 = 0x00aa_6800;
/// `00aa7260` (`fastcall`, the pool): sorts the free list through `00aa70c0`
/// when it holds more than one block.
const SORT_FREE_LIST: u32 = 0x00aa_7260;
/// `_memset` (CRT, cdecl: block, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// Import slots of `VirtualAlloc` and `VirtualFree` (`kernel32`, stdcall).
const VIRTUAL_ALLOC: u32 = 0x00fd_f114;
const VIRTUAL_FREE: u32 = 0x00fd_f10c;

/// `MEM_COMMIT`, `MEM_DECOMMIT`, `PAGE_READWRITE` (Windows constants the
/// calls pass).
const MEM_COMMIT: u32 = 0x1000;
const MEM_DECOMMIT: u32 = 0x4000;
const PAGE_READWRITE: u32 = 4;
/// Bytes per page the pool commits.
const PAGE_BYTES: u32 = 0x1000;
/// A page's in-use count while the page is not committed.
const PAGE_UNCOMMITTED: u16 = 0xffff;

/// `MemoryPool::pMemoryPoolsBySize` (Xbox PDB, static): 0x100 entries on
/// the PC, indexed by block size / 4.
pub const POOLS_BY_SIZE: u32 = 0x011f_67b8;
/// `MemoryPool::pMemoryPoolsByAddress` (Xbox PDB, static): 0x100 entries
/// on the PC, one per 16 MiB of address space.
pub const POOLS_BY_ADDRESS: u32 = 0x011f_63b8;
/// Entries of each table on the PC.
const TABLE_ENTRIES: u32 = 0x100;

/// Names the callers pass to the lock (never read by it).
const NAME_ALLOCATE: u32 = 0x010a_27dc;
const NAME_FREE: u32 = 0x010a_27f4;
const NAME_FREE_UNUSED_PAGES: u32 = 0x010a_2808;

layout! {
    /// `MemoryPool` (Xbox PDB, 0x30 bytes there); 0x80 bytes on the PC (the
    /// creator `00aa3cc0` allocates 0x80), with the lock at +0x20 and the
    /// counters from +0x40. Offsets are the PC code's. Addresses are kept
    /// as `u32`, since the code does arithmetic on them.
    pub struct MemoryPool: 0x80 {
        /// `pDescription` (Xbox PDB).
        0x00 pDescription: u32,
        /// `pHeap` (Xbox PDB): start of the reserved range.
        0x04 pHeap: u32,
        /// `pFreeHead` (Xbox PDB): first `FreePoolBlock`.
        0x08 pFreeHead: u32,
        /// `CriticalSection` (Xbox PDB, a `BSSpinLock`), word 0:
        /// `uiOwningThread`.
        0x20 uiOwningThread: u32,
        /// `CriticalSection` word 1: `uiLockCount`.
        0x24 uiLockCount: u32,
        /// `iAlignedBlockSize` (Xbox PDB).
        0x40 iAlignedBlockSize: u32,
        /// `iHighCommitedPages` (Xbox PDB).
        0x44 iHighCommitedPages: u32,
        /// `pPageRefCount` (Xbox PDB): `u16` per page.
        0x48 pPageRefCount: u32,
        /// `iNumPages` (Xbox PDB).
        0x4C iNumPages: u32,
        /// `iPoolSize` (Xbox PDB): bytes reserved.
        0x50 iPoolSize: u32,
        /// `iNumFreeBlocks` (Xbox PDB).
        0x54 iNumFreeBlocks: u32,
        /// `iNumCommittedPages` (Xbox PDB).
        0x58 iNumCommittedPages: u32,
    }

    /// `MemoryPool::FreePoolBlock` (Xbox PDB): written into a free block.
    pub struct FreePoolBlock: 0x08 {
        /// `pPrevious` (Xbox PDB).
        0x00 pPrevious: u32,
        /// `pNext` (Xbox PDB).
        0x04 pNext: u32,
    }

    /// `MemoryPoolStats` (Xbox PDB), 0x28 bytes.
    pub struct MemoryPoolStats: 0x28 {
        /// `pDesc` (Xbox PDB).
        0x00 pDesc: u32,
        /// `iOverhead` (Xbox PDB).
        0x04 iOverhead: u32,
        /// `iBlockSize` (Xbox PDB).
        0x08 iBlockSize: u32,
        /// `iReservedMem` (Xbox PDB).
        0x0C iReservedMem: u32,
        /// `iReservedPages` (Xbox PDB).
        0x10 iReservedPages: u32,
        /// `iCommittedMem` (Xbox PDB).
        0x14 iCommittedMem: u32,
        /// `iCommittedPages` (Xbox PDB).
        0x18 iCommittedPages: u32,
        /// `iHighCommittedPages` (Xbox PDB).
        0x1C iHighCommittedPages: u32,
        /// `iFreeBlocks` (Xbox PDB).
        0x20 iFreeBlocks: u32,
        /// `iUsedBlocks` (Xbox PDB).
        0x24 iUsedBlocks: u32,
    }
}

/// Takes the pool's lock (`0040fbf0` on +0x20) under the given name.
fn lock(e: &mut Engine, this: Ptr<MemoryPool>, name: u32) {
    e.call(LOCK, &args![this.addr().wrapping_add(0x20), name]);
}

/// The lock release the compiler inlined: decrements the count and clears
/// the owner when it reaches 0.
fn unlock(e: &mut Engine, this: Ptr<MemoryPool>) {
    let count = e.get(this, MemoryPool::uiLockCount).wrapping_sub(1);
    e.set(this, MemoryPool::uiLockCount, count);
    if count == 0 {
        e.set(this, MemoryPool::uiOwningThread, 0);
    }
}

/// Address of page `index`'s in-use count.
fn page_count_addr(e: &Engine, this: Ptr<MemoryPool>, index: u32) -> u32 {
    e.get(this, MemoryPool::pPageRefCount)
        .wrapping_add(index.wrapping_mul(2))
}

/// The page holding `block`.
fn page_of(e: &Engine, this: Ptr<MemoryPool>, block: u32) -> u32 {
    block.wrapping_sub(e.get(this, MemoryPool::pHeap)) >> 12
}

/// Entry `index` of a 4-byte global table.
fn table_entry(table: u32, index: u32) -> u32 {
    table.wrapping_add(index.wrapping_mul(4))
}

/// The number of 16 MiB address-table entries the pool's range covers.
fn address_entries(e: &Engine, this: Ptr<MemoryPool>) -> u32 {
    let size = e.get(this, MemoryPool::iPoolSize);
    let mut entries = size >> 24;
    if size % 0x0100_0000 != 0 {
        entries += 1;
    }
    entries
}

// Translated from 00aa6610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Commits page `index` of the pool: stores its address in `*page_out` and
/// commits it (`VirtualAlloc(page, 0x1000, MEM_COMMIT, PAGE_READWRITE)`).
/// Returns whether the commit succeeded.
pub fn fn_00aa6610(e: &mut Engine, this: Ptr<MemoryPool>, index: u32, page_out: Ptr) -> bool {
    let page = (index << 12).wrapping_add(e.get(this, MemoryPool::pHeap));
    e.mem.set_u32(page_out.addr(), page);
    let page = e.mem.u32(page_out.addr());
    let result = e
        .call(
            VIRTUAL_ALLOC,
            &args![page, PAGE_BYTES, MEM_COMMIT, PAGE_READWRITE],
        )
        .u32();
    result != 0
}

// Translated from 00aa6650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Decommits one page (`VirtualFree(page, 0x1000, MEM_DECOMMIT)`); the pool
/// and the page index the caller passes are not read. Returns whether it
/// succeeded.
pub fn fn_00aa6650(e: &mut Engine, _this: Ptr<MemoryPool>, _unused_1: u32, page: u32) -> bool {
    e.call(VIRTUAL_FREE, &args![page, PAGE_BYTES, MEM_DECOMMIT])
        .u32()
        != 0
}

// Translated from 00aa6840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryPool::InitPools` (Xbox PDB): clears both pool tables.
pub fn memory_pool_init_pools(e: &mut Engine) {
    for i in 0..TABLE_ENTRIES {
        e.mem.set_u32(table_entry(POOLS_BY_SIZE, i), 0);
    }
    for i in 0..TABLE_ENTRIES {
        e.mem.set_u32(table_entry(POOLS_BY_ADDRESS, i), 0);
    }
}

// Translated from 00aa68a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys every pool of the by-size table: runs the pool's destructor
/// (`00aa6800`), frees it through the memory manager and clears the entry.
pub fn fn_00aa68a0(e: &mut Engine) {
    for i in 0..TABLE_ENTRIES {
        let entry = table_entry(POOLS_BY_SIZE, i);
        let pool = e.mem.u32(entry);
        if pool != 0 {
            e.call(POOL_DESTRUCTOR, &args![pool]);
            // The inlined scalar deleting destructor's flag is the constant 1.
            e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, pool]);
        }
        e.mem.set_u32(entry, 0);
    }
}

// Translated from 00aa6920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the pool in the by-size table at `size / 4`.
pub fn fn_00aa6920(e: &mut Engine, this: Ptr<MemoryPool>, size: u32) {
    e.mem
        .set_u32(table_entry(POOLS_BY_SIZE, size >> 2), this.addr());
}

// Translated from 00aa6950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the by-size table's entry at `size / 4` (`this` is not read).
pub fn fn_00aa6950(e: &mut Engine, _this: Ptr, size: u32) {
    e.mem.set_u32(table_entry(POOLS_BY_SIZE, size >> 2), 0);
}

// Translated from 00aa6980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the pool: in the by-size table at its block size / 4, and in
/// the by-address table for every 16 MiB its range covers (from the entry
/// of `pHeap`, the size rounded up).
pub fn fn_00aa6980(e: &mut Engine, this: Ptr<MemoryPool>) {
    let size_index = e.get(this, MemoryPool::iAlignedBlockSize) >> 2;
    e.mem
        .set_u32(table_entry(POOLS_BY_SIZE, size_index), this.addr());
    let entries = address_entries(e, this);
    let first = e.get(this, MemoryPool::pHeap) >> 24;
    for i in 0..entries {
        e.mem.set_u32(
            table_entry(POOLS_BY_ADDRESS, first.wrapping_add(i)),
            this.addr(),
        );
    }
}

// Translated from 00aa6a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unregisters the pool: clears the entries [`fn_00aa6980`] wrote.
pub fn fn_00aa6a10(e: &mut Engine, this: Ptr<MemoryPool>) {
    let size_index = e.get(this, MemoryPool::iAlignedBlockSize) >> 2;
    e.mem.set_u32(table_entry(POOLS_BY_SIZE, size_index), 0);
    let entries = address_entries(e, this);
    let first = e.get(this, MemoryPool::pHeap) >> 24;
    for i in 0..entries {
        e.mem
            .set_u32(table_entry(POOLS_BY_ADDRESS, first.wrapping_add(i)), 0);
    }
}

// Translated from 00aa6aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryPool::Allocate` (Xbox PDB): under the pool's lock, takes the
/// first free block. With no free block it commits the first uncommitted
/// page (returning 0 when there is none or the commit fails), sets its count
/// to 0, adds all its blocks to the free list and counts the page (raising
/// the high-water mark). The taken block's page count goes up by one. The
/// one stack word (the size the memory manager passes) is not read.
pub fn memory_pool_allocate(e: &mut Engine, this: Ptr<MemoryPool>, _unused_1: u32) -> u32 {
    lock(e, this, NAME_ALLOCATE);
    if e.get(this, MemoryPool::pFreeHead) == 0 {
        let index = fn_00aa6db0(e, this);
        if index == u32::MAX {
            unlock(e, this);
            return 0;
        }
        let blocks_per_page = PAGE_BYTES / e.get(this, MemoryPool::iAlignedBlockSize);
        let committed = e.with_stack(4, |e, page_out| {
            let ok = fn_00aa6610(e, this, index, page_out);
            (ok, e.mem.u32(page_out.addr()))
        });
        let (ok, page) = committed;
        if !ok {
            unlock(e, this);
            return 0;
        }
        let count = page_count_addr(e, this, index);
        e.mem.set_u16(count, 0);
        for i in 0..blocks_per_page {
            let block_size = e.get(this, MemoryPool::iAlignedBlockSize);
            let block = i.wrapping_mul(block_size).wrapping_add(page);
            memory_pool_add_free_block(e, this, block);
        }
        let pages = e.get(this, MemoryPool::iNumCommittedPages).wrapping_add(1);
        e.set(this, MemoryPool::iNumCommittedPages, pages);
        if pages > e.get(this, MemoryPool::iHighCommitedPages) {
            e.set(this, MemoryPool::iHighCommitedPages, pages);
        }
    }
    let mut block = 0;
    let head = e.get(this, MemoryPool::pFreeHead);
    if head != 0 {
        block = head;
        let next = e.get(Ptr::<FreePoolBlock>::new(head), FreePoolBlock::pNext);
        e.set(this, MemoryPool::pFreeHead, next);
        fn_00aa6e60(e, this, head);
        let count = page_count_addr(e, this, page_of(e, this, block));
        let used = e.mem.u16(count).wrapping_add(1);
        e.mem.set_u16(count, used);
    }
    unlock(e, this);
    block
}

// Translated from 00aa6c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryPool::Free` (Xbox PDB): null does nothing; otherwise, under the
/// pool's lock, puts the block at the head of the free list and lowers its
/// page's count.
pub fn memory_pool_free(e: &mut Engine, this: Ptr<MemoryPool>, block: u32) {
    if block == 0 {
        return;
    }
    lock(e, this, NAME_FREE);
    memory_pool_add_free_block(e, this, block);
    let count = page_count_addr(e, this, page_of(e, this, block));
    let used = e.mem.u16(count).wrapping_sub(1);
    e.mem.set_u16(count, used);
    unlock(e, this);
}

// Translated from 00aa6d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the `MemoryPoolStats` at `result` (built in a cleared local, then
/// copied): description, overhead (`2 * pages + 0x80`), block size,
/// reserved bytes and pages, committed bytes and pages, the high-water
/// mark, free blocks, and used blocks (committed bytes / block size - free
/// blocks). Returns `result`.
pub fn fn_00aa6d00(e: &mut Engine, this: Ptr<MemoryPool>, result: Ptr) -> Ptr {
    e.with_stack(0x28, |e, local| {
        e.call(MEMSET, &args![local, 0u32, 0x28u32]);
        let stats: Ptr<MemoryPoolStats> = local.cast();
        let pages = e.get(this, MemoryPool::iNumPages);
        let committed = e.get(this, MemoryPool::iNumCommittedPages);
        let block_size = e.get(this, MemoryPool::iAlignedBlockSize);
        let free_blocks = e.get(this, MemoryPool::iNumFreeBlocks);
        let description = e.get(this, MemoryPool::pDescription);
        e.set(stats, MemoryPoolStats::pDesc, description);
        e.set(
            stats,
            MemoryPoolStats::iOverhead,
            pages.wrapping_mul(2).wrapping_add(0x80),
        );
        e.set(stats, MemoryPoolStats::iBlockSize, block_size);
        let pool_size = e.get(this, MemoryPool::iPoolSize);
        e.set(stats, MemoryPoolStats::iReservedMem, pool_size);
        e.set(stats, MemoryPoolStats::iReservedPages, pages);
        e.set(stats, MemoryPoolStats::iCommittedMem, committed << 12);
        e.set(stats, MemoryPoolStats::iCommittedPages, committed);
        let high = e.get(this, MemoryPool::iHighCommitedPages);
        e.set(stats, MemoryPoolStats::iHighCommittedPages, high);
        e.set(stats, MemoryPoolStats::iFreeBlocks, free_blocks);
        let committed_bytes = e.get(stats, MemoryPoolStats::iCommittedMem);
        let block_size = e.get(this, MemoryPool::iAlignedBlockSize);
        let free_blocks = e.get(this, MemoryPool::iNumFreeBlocks);
        e.set(
            stats,
            MemoryPoolStats::iUsedBlocks,
            (committed_bytes / block_size).wrapping_sub(free_blocks),
        );
        for i in 0..10 {
            let word = e.mem.u32(local.addr() + 4 * i);
            e.mem.set_u32(result.addr().wrapping_add(4 * i), word);
        }
    });
    result
}

// Translated from 00aa6db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first uncommitted page (count `0xffff`), or `0xffffffff` when every
/// page is committed.
pub fn fn_00aa6db0(e: &mut Engine, this: Ptr<MemoryPool>) -> u32 {
    let mut index = 0u32;
    while index < e.get(this, MemoryPool::iNumPages) {
        let count = page_count_addr(e, this, index);
        if e.mem.u16(count) == PAGE_UNCOMMITTED {
            return index;
        }
        index += 1;
    }
    u32::MAX
}

// Translated from 00aa6e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryPool::AddFreeBlock` (Xbox PDB): puts the block at the head of the
/// free list and counts it.
pub fn memory_pool_add_free_block(e: &mut Engine, this: Ptr<MemoryPool>, block: u32) {
    let node: Ptr<FreePoolBlock> = Ptr::new(block);
    e.set(node, FreePoolBlock::pPrevious, 0);
    let head = e.get(this, MemoryPool::pFreeHead);
    e.set(node, FreePoolBlock::pNext, head);
    if head != 0 {
        e.set(
            Ptr::<FreePoolBlock>::new(head),
            FreePoolBlock::pPrevious,
            block,
        );
    }
    e.set(this, MemoryPool::pFreeHead, block);
    let free = e.get(this, MemoryPool::iNumFreeBlocks).wrapping_add(1);
    e.set(this, MemoryPool::iNumFreeBlocks, free);
}

// Translated from 00aa6e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unlinks a free block from its neighbours (not from `pFreeHead`) and
/// lowers the free-block count.
pub fn fn_00aa6e60(e: &mut Engine, this: Ptr<MemoryPool>, block: u32) {
    let node: Ptr<FreePoolBlock> = Ptr::new(block);
    let previous = e.get(node, FreePoolBlock::pPrevious);
    if previous != 0 {
        let next = e.get(node, FreePoolBlock::pNext);
        e.set(
            Ptr::<FreePoolBlock>::new(previous),
            FreePoolBlock::pNext,
            next,
        );
    }
    let next = e.get(node, FreePoolBlock::pNext);
    if next != 0 {
        let previous = e.get(node, FreePoolBlock::pPrevious);
        e.set(
            Ptr::<FreePoolBlock>::new(next),
            FreePoolBlock::pPrevious,
            previous,
        );
    }
    let free = e.get(this, MemoryPool::iNumFreeBlocks).wrapping_sub(1);
    e.set(this, MemoryPool::iNumFreeBlocks, free);
}

// Translated from 00aa6eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases an unused page: optionally sorts the free list first
/// (`00aa7260`), then cuts the page's blocks out of the free list as one run
/// (from the page's first block to its last, which sorting makes adjacent),
/// uncounts them, decommits the page, marks it uncommitted and lowers the
/// committed-page count.
pub fn fn_00aa6eb0(e: &mut Engine, this: Ptr<MemoryPool>, index: u32, sort: bool) {
    let page = (index << 12).wrapping_add(e.get(this, MemoryPool::pHeap));
    if sort {
        e.call(SORT_FREE_LIST, &args![this]);
    }
    let first: Ptr<FreePoolBlock> = Ptr::new(page);
    let blocks_per_page = PAGE_BYTES / e.get(this, MemoryPool::iAlignedBlockSize);
    let last_offset = blocks_per_page
        .wrapping_sub(1)
        .wrapping_mul(e.get(this, MemoryPool::iAlignedBlockSize));
    let last: Ptr<FreePoolBlock> = Ptr::new(last_offset.wrapping_add(page));
    let previous = e.get(first, FreePoolBlock::pPrevious);
    let after = e.get(last, FreePoolBlock::pNext);
    if previous != 0 {
        e.set(
            Ptr::<FreePoolBlock>::new(previous),
            FreePoolBlock::pNext,
            after,
        );
    } else {
        e.set(this, MemoryPool::pFreeHead, after);
    }
    let after = e.get(last, FreePoolBlock::pNext);
    if after != 0 {
        let previous = e.get(first, FreePoolBlock::pPrevious);
        e.set(
            Ptr::<FreePoolBlock>::new(after),
            FreePoolBlock::pPrevious,
            previous,
        );
    }
    let free = e
        .get(this, MemoryPool::iNumFreeBlocks)
        .wrapping_sub(blocks_per_page);
    e.set(this, MemoryPool::iNumFreeBlocks, free);
    fn_00aa6650(e, this, index, page);
    let count = page_count_addr(e, this, index);
    e.mem.set_u16(count, PAGE_UNCOMMITTED);
    let pages = e.get(this, MemoryPool::iNumCommittedPages).wrapping_sub(1);
    e.set(this, MemoryPool::iNumCommittedPages, pages);
}

// Translated from 00aa6f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryPool::FreeUnusedPages` (Xbox PDB): under the pool's lock,
/// releases every committed page with no block in use, sorting the free
/// list once before the first release.
pub fn memory_pool_free_unused_pages(e: &mut Engine, this: Ptr<MemoryPool>) {
    lock(e, this, NAME_FREE_UNUSED_PAGES);
    let mut sorted = false;
    let mut index = 0u32;
    while index < e.get(this, MemoryPool::iNumPages) {
        let count = page_count_addr(e, this, index);
        if e.mem.u16(count) == 0 {
            if !sorted {
                e.call(SORT_FREE_LIST, &args![this]);
                sorted = true;
            }
            fn_00aa6eb0(e, this, index, false);
        }
        index += 1;
    }
    unlock(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa6610, fn_00aa6610(Ptr<MemoryPool>, u32, Ptr) -> bool),
        entry!(0x00aa6650, fn_00aa6650(Ptr<MemoryPool>, u32, u32) -> bool),
        entry!(0x00aa6840, memory_pool_init_pools()),
        entry!(0x00aa68a0, fn_00aa68a0()),
        entry!(0x00aa6920, fn_00aa6920(Ptr<MemoryPool>, u32)),
        entry!(0x00aa6950, fn_00aa6950(Ptr, u32)),
        entry!(0x00aa6980, fn_00aa6980(Ptr<MemoryPool>)),
        entry!(0x00aa6a10, fn_00aa6a10(Ptr<MemoryPool>)),
        entry!(
            0x00aa6aa0,
            memory_pool_allocate(Ptr<MemoryPool>, u32) -> u32
        ),
        entry!(0x00aa6c70, memory_pool_free(Ptr<MemoryPool>, u32)),
        entry!(0x00aa6d00, fn_00aa6d00(Ptr<MemoryPool>, Ptr) -> Ptr),
        entry!(0x00aa6db0, fn_00aa6db0(Ptr<MemoryPool>) -> u32),
        entry!(0x00aa6e00, memory_pool_add_free_block(Ptr<MemoryPool>, u32)),
        entry!(0x00aa6e60, fn_00aa6e60(Ptr<MemoryPool>, u32)),
        entry!(0x00aa6eb0, fn_00aa6eb0(Ptr<MemoryPool>, u32, bool)),
        entry!(0x00aa6f90, memory_pool_free_unused_pages(Ptr<MemoryPool>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_op(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An engine with the lock, the imports, the sort and the tables
    /// doubled or mapped.
    fn pool_engine() -> Engine {
        let mut e = Engine::new();
        e.register(LOCK, |e, a| {
            // Takes the lock as the owner (main thread 1).
            let count = e.mem.u32(a[0] + 4) + 1;
            e.mem.set_u32(a[0] + 4, count);
            e.mem.set_u32(a[0], 1);
            Ret::default()
        });
        e.register(VIRTUAL_ALLOC, |_, a| ret(a[0]));
        e.register(VIRTUAL_FREE, |_, _| ret(1));
        e.register(SORT_FREE_LIST, no_op);
        e.register(POOL_DESTRUCTOR, no_op);
        e.register(MEMORY_MANAGER_DEALLOCATE, no_op);
        e.register(MEMSET, no_op);
        e.map(0x011f_6000, 0x1000);
        e
    }

    /// A pool of `pages` pages of `block_size`-byte blocks, all uncommitted.
    fn pool(e: &mut Engine, pages: u32, block_size: u32) -> Ptr<MemoryPool> {
        let p: Ptr<MemoryPool> = e.new_object();
        let heap = e.mem.alloc(pages * PAGE_BYTES + PAGE_BYTES);
        let heap = (heap + 0xfff) & !0xfff;
        e.set(p, MemoryPool::pHeap, heap);
        e.set(p, MemoryPool::iAlignedBlockSize, block_size);
        e.set(p, MemoryPool::iNumPages, pages);
        e.set(p, MemoryPool::iPoolSize, pages * PAGE_BYTES);
        let counts = e.mem.alloc(pages * 2);
        e.set(p, MemoryPool::pPageRefCount, counts);
        for i in 0..pages {
            e.mem.set_u16(counts + 2 * i, PAGE_UNCOMMITTED);
        }
        p
    }

    fn count(e: &Engine, p: Ptr<MemoryPool>, index: u32) -> u16 {
        e.mem.u16(page_count_addr(e, p, index))
    }

    /// The free list from the head, following `pNext`.
    fn free_list(e: &Engine, p: Ptr<MemoryPool>) -> Vec<u32> {
        let mut out = vec![];
        let mut b = e.get(p, MemoryPool::pFreeHead);
        while b != 0 {
            out.push(b);
            b = e.mem.u32(b + 4);
        }
        out
    }

    #[test]
    fn commit_page_writes_the_address_and_reports_success() {
        let mut e = pool_engine();
        let p = pool(&mut e, 4, 0x40);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00aa_6610, &args![p, 2u32, out]).bool());
        let heap = e.get(p, MemoryPool::pHeap);
        assert_eq!(e.mem.u32(out), heap + 0x2000);
        let log = e.call_log.take().unwrap();
        let call = log.iter().find(|(a, _)| *a == VIRTUAL_ALLOC).unwrap();
        assert_eq!(call.1, vec![heap + 0x2000, 0x1000, 0x1000, 4]);
        e.register(VIRTUAL_ALLOC, |_, _| ret(0));
        assert!(!e.call(0x00aa_6610, &args![p, 2u32, out]).bool());
    }

    #[test]
    fn decommit_page_passes_the_page() {
        let mut e = pool_engine();
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00aa_6650, &args![0u32, 7u32, 0x0012_3000u32])
            .bool());
        let log = e.call_log.take().unwrap();
        let call = log.iter().find(|(a, _)| *a == VIRTUAL_FREE).unwrap();
        assert_eq!(call.1, vec![0x0012_3000, 0x1000, 0x4000]);
        e.register(VIRTUAL_FREE, |_, _| ret(0));
        assert!(!e
            .call(0x00aa_6650, &args![0u32, 7u32, 0x0012_3000u32])
            .bool());
    }

    #[test]
    fn init_pools_clears_both_tables() {
        let mut e = pool_engine();
        e.mem.set_u32(POOLS_BY_SIZE, 5);
        e.mem.set_u32(POOLS_BY_SIZE + 0x3fc, 5);
        e.mem.set_u32(POOLS_BY_ADDRESS + 0x200, 5);
        e.call(0x00aa_6840, &args![]);
        assert_eq!(e.mem.u32(POOLS_BY_SIZE), 0);
        assert_eq!(e.mem.u32(POOLS_BY_SIZE + 0x3fc), 0);
        assert_eq!(e.mem.u32(POOLS_BY_ADDRESS + 0x200), 0);
    }

    #[test]
    fn destroy_pools_destroys_frees_and_clears_each_entry() {
        let mut e = pool_engine();
        e.mem.set_u32(POOLS_BY_SIZE + 8, 0x0050_0000);
        e.mem.set_u32(POOLS_BY_SIZE + 0x40, 0x0060_0000);
        e.call_log = Some(vec![]);
        e.call(0x00aa_68a0, &args![]);
        let log = e.call_log.take().unwrap();
        let destroyed: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == POOL_DESTRUCTOR)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(destroyed, vec![0x0050_0000, 0x0060_0000]);
        let freed: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == MEMORY_MANAGER_DEALLOCATE)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            freed,
            vec![
                vec![MEMORY_MANAGER, 0x0050_0000],
                vec![MEMORY_MANAGER, 0x0060_0000]
            ]
        );
        assert_eq!(e.mem.u32(POOLS_BY_SIZE + 8), 0);
        assert_eq!(e.mem.u32(POOLS_BY_SIZE + 0x40), 0);
    }

    #[test]
    fn set_and_clear_size_entry() {
        let mut e = pool_engine();
        e.call(0x00aa_6920, &args![0x0077_0000u32, 0x20u32]);
        assert_eq!(e.mem.u32(POOLS_BY_SIZE + 0x20), 0x0077_0000);
        // Not a multiple of 4: the index is size / 4.
        e.call(0x00aa_6950, &args![0u32, 0x23u32]);
        assert_eq!(e.mem.u32(POOLS_BY_SIZE + 0x20), 0);
    }

    #[test]
    fn register_and_unregister_cover_every_16_mib() {
        let mut e = pool_engine();
        let p: Ptr<MemoryPool> = e.new_object();
        e.set(p, MemoryPool::pHeap, 0x2000_0000);
        e.set(p, MemoryPool::iAlignedBlockSize, 0x10);
        // 32 MiB + 1 byte: three entries.
        e.set(p, MemoryPool::iPoolSize, 0x0200_0001);
        e.call(0x00aa_6980, &args![p]);
        assert_eq!(e.mem.u32(POOLS_BY_SIZE + 0x10), p.addr());
        for i in 0x20..0x23 {
            assert_eq!(e.mem.u32(POOLS_BY_ADDRESS + 4 * i), p.addr());
        }
        assert_eq!(e.mem.u32(POOLS_BY_ADDRESS + 4 * 0x23), 0);
        e.call(0x00aa_6a10, &args![p]);
        assert_eq!(e.mem.u32(POOLS_BY_SIZE + 0x10), 0);
        for i in 0x20..0x23 {
            assert_eq!(e.mem.u32(POOLS_BY_ADDRESS + 4 * i), 0);
        }

        // Exactly 16 MiB: one entry.
        e.set(p, MemoryPool::iPoolSize, 0x0100_0000);
        e.call(0x00aa_6980, &args![p]);
        assert_eq!(e.mem.u32(POOLS_BY_ADDRESS + 4 * 0x20), p.addr());
        assert_eq!(e.mem.u32(POOLS_BY_ADDRESS + 4 * 0x21), 0);
    }

    #[test]
    fn allocate_commits_a_page_when_the_list_is_empty() {
        let mut e = pool_engine();
        let p = pool(&mut e, 2, 0x400);
        e.set(p, MemoryPool::iHighCommitedPages, 0);
        let heap = e.get(p, MemoryPool::pHeap);
        let block = e.call(0x00aa_6aa0, &args![p, 0x400u32]).u32();
        // Blocks are added in address order at the head: the last one comes
        // out first.
        assert_eq!(block, heap + 0xc00);
        assert_eq!(count(&e, p, 0), 1);
        assert_eq!(e.get(p, MemoryPool::iNumCommittedPages), 1);
        assert_eq!(e.get(p, MemoryPool::iHighCommitedPages), 1);
        assert_eq!(e.get(p, MemoryPool::iNumFreeBlocks), 3);
        assert_eq!(free_list(&e, p), vec![heap + 0x800, heap + 0x400, heap]);
        assert_eq!(e.mem.u32(heap + 0x800), 0, "new head has no previous");
        assert_eq!(e.get(p, MemoryPool::uiLockCount), 0);
        assert_eq!(e.get(p, MemoryPool::uiOwningThread), 0);

        // The next one comes from the list without a commit.
        e.call_log = Some(vec![]);
        let block = e.call(0x00aa_6aa0, &args![p, 0x400u32]).u32();
        assert_eq!(block, heap + 0x800);
        assert_eq!(count(&e, p, 0), 2);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == VIRTUAL_ALLOC));
        let lock_call = log.iter().find(|(a, _)| *a == LOCK).unwrap();
        assert_eq!(lock_call.1, vec![p.addr() + 0x20, NAME_ALLOCATE]);
    }

    #[test]
    fn allocate_returns_0_when_no_page_is_left_or_the_commit_fails() {
        let mut e = pool_engine();
        let p = pool(&mut e, 1, 0x800);
        let counts = e.get(p, MemoryPool::pPageRefCount);
        e.mem.set_u16(counts, 5);
        assert_eq!(e.call(0x00aa_6aa0, &args![p, 0u32]).u32(), 0);
        assert_eq!(e.get(p, MemoryPool::uiLockCount), 0);

        e.mem.set_u16(counts, PAGE_UNCOMMITTED);
        e.register(VIRTUAL_ALLOC, |_, _| ret(0));
        assert_eq!(e.call(0x00aa_6aa0, &args![p, 0u32]).u32(), 0);
        assert_eq!(count(&e, p, 0), PAGE_UNCOMMITTED);
        assert_eq!(e.get(p, MemoryPool::iNumCommittedPages), 0);
    }

    #[test]
    fn allocate_keeps_a_higher_high_water_mark() {
        let mut e = pool_engine();
        let p = pool(&mut e, 2, 0x800);
        e.set(p, MemoryPool::iHighCommitedPages, 5);
        e.call(0x00aa_6aa0, &args![p, 0u32]);
        assert_eq!(e.get(p, MemoryPool::iHighCommitedPages), 5);
    }

    #[test]
    fn free_returns_the_block_and_lowers_the_count() {
        let mut e = pool_engine();
        let p = pool(&mut e, 2, 0x800);
        let heap = e.get(p, MemoryPool::pHeap);
        let a = e.call(0x00aa_6aa0, &args![p, 0u32]).u32();
        let b = e.call(0x00aa_6aa0, &args![p, 0u32]).u32();
        assert_eq!(count(&e, p, 0), 2);
        e.call_log = Some(vec![]);
        e.call(0x00aa_6c70, &args![p, a]);
        assert_eq!(count(&e, p, 0), 1);
        assert_eq!(free_list(&e, p), vec![a]);
        assert_eq!(e.get(p, MemoryPool::uiLockCount), 0);
        let log = e.call_log.take().unwrap();
        let lock_call = log.iter().find(|(a, _)| *a == LOCK).unwrap();
        assert_eq!(lock_call.1[1], NAME_FREE);
        // Null: nothing at all, not even the lock.
        e.call_log = Some(vec![]);
        e.call(0x00aa_6c70, &args![p, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == LOCK));
        assert_eq!(b, heap);
    }

    #[test]
    fn stats_copy_the_counters() {
        let mut e = pool_engine();
        let p: Ptr<MemoryPool> = e.new_object();
        e.set(p, MemoryPool::pDescription, 0x0101_0000);
        e.set(p, MemoryPool::iNumPages, 0x10);
        e.set(p, MemoryPool::iAlignedBlockSize, 0x100);
        e.set(p, MemoryPool::iPoolSize, 0x10000);
        e.set(p, MemoryPool::iNumCommittedPages, 3);
        e.set(p, MemoryPool::iHighCommitedPages, 7);
        e.set(p, MemoryPool::iNumFreeBlocks, 10);
        let result = Ptr::new(e.mem.alloc(0x28));
        e.call_log = Some(vec![]);
        let back = e.call(0x00aa_6d00, &args![p, result]).ptr::<()>();
        assert_eq!(back, result);
        let words: Vec<u32> = (0..10).map(|i| e.mem.u32(result.addr() + 4 * i)).collect();
        assert_eq!(
            words,
            vec![
                0x0101_0000,
                0xa0,
                0x100,
                0x10000,
                0x10,
                0x3000,
                3,
                7,
                10,
                0x30 - 10
            ]
        );
        let log = e.call_log.take().unwrap();
        let memset = log.iter().find(|(a, _)| *a == MEMSET).unwrap();
        assert_eq!(&memset.1[1..], &[0, 0x28]);
    }

    #[test]
    fn first_uncommitted_page() {
        let mut e = pool_engine();
        let p = pool(&mut e, 3, 0x100);
        let counts = e.get(p, MemoryPool::pPageRefCount);
        e.mem.set_u16(counts, 0);
        assert_eq!(e.call(0x00aa_6db0, &args![p]).u32(), 1);
        e.mem.set_u16(counts + 2, 4);
        e.mem.set_u16(counts + 4, 0);
        assert_eq!(e.call(0x00aa_6db0, &args![p]).u32(), u32::MAX);
    }

    #[test]
    fn add_free_block_links_at_the_head() {
        let mut e = pool_engine();
        let p: Ptr<MemoryPool> = e.new_object();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_u32(a, 0xdead);
        e.call(0x00aa_6e00, &args![p, a]);
        assert_eq!(e.mem.u32(a), 0);
        assert_eq!(e.mem.u32(a + 4), 0);
        e.call(0x00aa_6e00, &args![p, b]);
        assert_eq!(e.get(p, MemoryPool::pFreeHead), b);
        assert_eq!(e.mem.u32(b + 4), a);
        assert_eq!(e.mem.u32(a), b);
        assert_eq!(e.get(p, MemoryPool::iNumFreeBlocks), 2);
    }

    #[test]
    fn unlink_joins_the_neighbours() {
        let mut e = pool_engine();
        let p: Ptr<MemoryPool> = e.new_object();
        let [a, b, c] = [e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8)];
        for x in [c, b, a] {
            e.call(0x00aa_6e00, &args![p, x]);
        }
        e.call(0x00aa_6e60, &args![p, b]);
        assert_eq!(e.mem.u32(a + 4), c);
        assert_eq!(e.mem.u32(c), a);
        assert_eq!(e.get(p, MemoryPool::iNumFreeBlocks), 2);
        // The head is not touched by the unlink.
        e.call(0x00aa_6e60, &args![p, a]);
        assert_eq!(e.get(p, MemoryPool::pFreeHead), a);
        assert_eq!(e.mem.u32(c), 0);
        // The tail: only the previous block's link changes.
        e.call(0x00aa_6e60, &args![p, c]);
        assert_eq!(e.get(p, MemoryPool::iNumFreeBlocks), 0);
    }

    #[test]
    fn release_page_cuts_its_blocks_out_and_decommits() {
        let mut e = pool_engine();
        let p = pool(&mut e, 2, 0x800);
        let heap = e.get(p, MemoryPool::pHeap);
        // Commit page 0 (two blocks) and page 1 by allocating three blocks
        // and freeing them all.
        let blocks: Vec<u32> = (0..3)
            .map(|_| e.call(0x00aa_6aa0, &args![p, 0u32]).u32())
            .collect();
        for b in &blocks {
            e.call(0x00aa_6c70, &args![p, *b]);
        }
        // Lay the list out sorted: heap, heap+0x800, heap+0x1000, heap+0x1800.
        let sorted = [heap, heap + 0x800, heap + 0x1000, heap + 0x1800];
        for (i, b) in sorted.iter().enumerate() {
            let prev = if i == 0 { 0 } else { sorted[i - 1] };
            let next = sorted.get(i + 1).copied().unwrap_or(0);
            e.mem.set_u32(*b, prev);
            e.mem.set_u32(*b + 4, next);
        }
        e.set(p, MemoryPool::pFreeHead, heap);
        assert_eq!(e.get(p, MemoryPool::iNumFreeBlocks), 4);

        // Page 1 (the tail of the list): the run's previous block ends it.
        e.call_log = Some(vec![]);
        e.call(0x00aa_6eb0, &args![p, 1u32, true]);
        assert_eq!(free_list(&e, p), vec![heap, heap + 0x800]);
        assert_eq!(e.get(p, MemoryPool::iNumFreeBlocks), 2);
        assert_eq!(count(&e, p, 1), PAGE_UNCOMMITTED);
        assert_eq!(e.get(p, MemoryPool::iNumCommittedPages), 1);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(a, w)| *a == SORT_FREE_LIST && w[0] == p.addr()));
        let free = log.iter().find(|(a, _)| *a == VIRTUAL_FREE).unwrap();
        assert_eq!(free.1[0], heap + 0x1000);

        // Page 0 (the head): the head moves past the run, no sort.
        e.call_log = Some(vec![]);
        e.call(0x00aa_6eb0, &args![p, 0u32, false]);
        assert_eq!(e.get(p, MemoryPool::pFreeHead), 0);
        assert_eq!(e.get(p, MemoryPool::iNumFreeBlocks), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == SORT_FREE_LIST));
    }

    #[test]
    fn release_page_in_the_middle_relinks_both_sides() {
        let mut e = pool_engine();
        let p = pool(&mut e, 3, 0x800);
        let heap = e.get(p, MemoryPool::pHeap);
        let counts = e.get(p, MemoryPool::pPageRefCount);
        let sorted: Vec<u32> = (0..6).map(|i| heap + 0x800 * i).collect();
        for (i, b) in sorted.iter().enumerate() {
            let prev = if i == 0 { 0 } else { sorted[i - 1] };
            let next = sorted.get(i + 1).copied().unwrap_or(0);
            e.mem.set_u32(*b, prev);
            e.mem.set_u32(*b + 4, next);
        }
        for i in 0..3 {
            e.mem.set_u16(counts + 2 * i, 0);
        }
        e.set(p, MemoryPool::pFreeHead, heap);
        e.set(p, MemoryPool::iNumFreeBlocks, 6);
        e.set(p, MemoryPool::iNumCommittedPages, 3);
        e.call(0x00aa_6eb0, &args![p, 1u32, false]);
        assert_eq!(
            free_list(&e, p),
            vec![heap, heap + 0x800, heap + 0x2000, heap + 0x2800]
        );
        assert_eq!(e.mem.u32(heap + 0x2000), heap + 0x800);
    }

    #[test]
    fn free_unused_pages_sorts_once_and_releases_empty_pages() {
        let mut e = pool_engine();
        let p = pool(&mut e, 3, 0x1000);
        let heap = e.get(p, MemoryPool::pHeap);
        let counts = e.get(p, MemoryPool::pPageRefCount);
        // Page 0 empty, page 1 in use, page 2 empty; one block per page.
        let pages = [heap, heap + 0x2000];
        e.mem.set_u32(pages[0], 0);
        e.mem.set_u32(pages[0] + 4, pages[1]);
        e.mem.set_u32(pages[1], pages[0]);
        e.mem.set_u32(pages[1] + 4, 0);
        e.set(p, MemoryPool::pFreeHead, heap);
        e.set(p, MemoryPool::iNumFreeBlocks, 2);
        e.set(p, MemoryPool::iNumCommittedPages, 3);
        e.mem.set_u16(counts, 0);
        e.mem.set_u16(counts + 2, 1);
        e.mem.set_u16(counts + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_6f90, &args![p]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|(a, _)| *a == SORT_FREE_LIST).count(), 1);
        let freed: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == VIRTUAL_FREE)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(freed, vec![heap, heap + 0x2000]);
        assert_eq!(e.get(p, MemoryPool::pFreeHead), 0);
        assert_eq!(e.get(p, MemoryPool::iNumCommittedPages), 1);
        assert_eq!(count(&e, p, 1), 1);
        assert_eq!(e.get(p, MemoryPool::uiLockCount), 0);
        let lock_call = log.iter().find(|(a, _)| *a == LOCK).unwrap();
        assert_eq!(lock_call.1[1], NAME_FREE_UNUSED_PAGES);

        // Nothing to release: no sort.
        e.call_log = Some(vec![]);
        e.call(0x00aa_6f90, &args![p]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == SORT_FREE_LIST));
    }
}

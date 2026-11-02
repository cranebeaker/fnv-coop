//! `Havok SDK/hkfreelistallocator.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB has no entry for `hkFreeListAllocator`, so its layout is described
//! by offsets, each named where a function uses it: `+0` virtual table, `+4` the
//! critical section, `+0x24` the parent allocator, `+0x28` an
//! `hkLargeBlockAllocator`, `+0xec` a table of 0x29 free-list pointers indexed by
//! size / 16, `+0x190` the list of distinct free lists and `+0x234` their count,
//! `+0x238` the next unused element of the pool of 0x29 free lists (0x34 bytes
//! each) that starts at `+0x240` and ends at `+0x23c`, `+0xa94` the soft limit.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkFreeListAllocator`'s virtual table (`010d1464`; the class name is confirmed
/// by the RTTI record before it).
const VTABLE: u32 = 0x010d_1464;
/// `EnterCriticalSection` and `LeaveCriticalSection` (`kernel32` import slots,
/// `stdcall`, one word).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `hkCriticalSection::hkCriticalSection` (`00cf6da0`, `thiscall`, spin count),
/// `hkLargeBlockAllocator::hkLargeBlockAllocator` (`00dd8d40`, `thiscall`, parent),
/// the free list's default constructor (`00dd7110`), constructor wrapper
/// (`00dd7650`: element size, alignment, block size, allocator) and clear
/// (`00dd7200`).
const CRITICAL_SECTION_CONSTRUCTOR: u32 = 0x00cf_6da0;
const LARGE_BLOCK_ALLOCATOR_CONSTRUCTOR: u32 = 0x00dd_8d40;
const FREE_LIST_DEFAULT_CONSTRUCTOR: u32 = 0x00dd_7110;
const FREE_LIST_CONSTRUCTOR: u32 = 0x00dd_7650;
const FREE_LIST_CLEAR: u32 = 0x00dd_7200;

const FREE_LIST_SIZE: u32 = 0x34;
const FREE_LIST_COUNT: u32 = 0x29;
const LARGE_BLOCK_ALLOCATOR: u32 = 0x28;
const SIZE_TABLE: u32 = 0xec;
const LIST_OF_LISTS: u32 = 0x190;
const LIST_COUNT: u32 = 0x234;
const POOL_NEXT: u32 = 0x238;
const POOL_END: u32 = 0x23c;
const POOL: u32 = 0x240;

// Translated from 00d5fb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::setMemorySoftLimit` (Xbox PDB), `thiscall`, `RET 4`: stores
/// `limit` at `+0xa94` between `EnterCriticalSection` and `LeaveCriticalSection` of
/// the section at `+4`.
pub fn hk_free_list_allocator_set_memory_soft_limit(e: &mut Engine, this: Ptr, limit: u32) {
    let section = this.addr() + 4;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    e.mem.set_u32(this.addr() + 0xa94, limit);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d5fc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): gives a free list back to the
/// pool. Clears it (`00dd7200`) and, when it is the last element handed out (it
/// ends at `+0x238`), takes that element back by moving `+0x238` down 0x34.
pub fn fn_00d5fc70(e: &mut Engine, this: Ptr, free_list: u32) {
    e.call(FREE_LIST_CLEAR, &args![free_list]);
    let next = e.mem.u32(this.addr() + POOL_NEXT);
    if free_list.wrapping_add(FREE_LIST_SIZE) == next {
        e.mem
            .set_u32(this.addr() + POOL_NEXT, next.wrapping_sub(FREE_LIST_SIZE));
    }
}

// Translated from 00d60020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: element size, alignment, block
/// size): the free list for blocks of that size. Takes the next free list of the
/// pool (`+0x238`, 0x34 bytes on) and constructs it (`00dd7650`, with the large
/// block allocator at `+0x28`). When a list in the list of lists (`+0x190`, `+0x234`
/// entries) has the same rounded element size (the word at `+4`) the new one is
/// given back (`00d5fc70`) and that list is returned; otherwise the new list is
/// appended and returned. The exe executes `INT3` when the pool is used up (`+0x238`
/// not below `+0x23c`): that is a panic here.
pub fn fn_00d60020(
    e: &mut Engine,
    this: Ptr,
    element_size: u32,
    alignment: u32,
    block_size: u32,
) -> u32 {
    let base = this.addr();
    let next = e.mem.u32(base + POOL_NEXT);
    if next >= e.mem.u32(base + POOL_END) {
        panic!("FalloutNV.exe executes INT3 at 00d60034: the pool of free lists is used up");
    }
    let new_list = next;
    e.mem
        .set_u32(base + POOL_NEXT, new_list.wrapping_add(FREE_LIST_SIZE));
    if new_list != 0 {
        e.call(
            FREE_LIST_CONSTRUCTOR,
            &args![
                new_list,
                element_size,
                alignment,
                block_size,
                base + LARGE_BLOCK_ALLOCATOR
            ],
        );
    }
    let count = e.mem.u32(base + LIST_COUNT) as i32;
    let rounded = e.mem.u32(new_list + 4);
    let mut index = 0u32;
    while (index as i32) < count {
        let list = e.mem.u32(base + LIST_OF_LISTS + index * 4);
        if e.mem.u32(list + 4) == rounded {
            e.call(0x00d5_fc70, &args![base, new_list]);
            return e.mem.u32(base + LIST_OF_LISTS + index * 4);
        }
        index += 1;
    }
    e.mem.set_u32(
        base + LIST_OF_LISTS + (count as u32).wrapping_mul(4),
        new_list,
    );
    let stored = e.mem.u32(base + LIST_COUNT);
    e.mem.set_u32(base + LIST_COUNT, stored.wrapping_add(1));
    new_list
}

// Translated from 00d60270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::hkFreeListAllocator` (Xbox PDB; the map gives the address
/// no name), `thiscall`, `RET 4`: stores the virtual table, constructs the critical
/// section at `+4` (spin count 0xfa0), stores `parent` at `+0x24` and constructs
/// the large block allocator at `+0x28` with it. Constructs the pool of 0x29
/// default free lists at `+0x240`, clears `+0x234`, `+0xa98`, `+0x1c` and `+0x20`,
/// sets the soft limit to `0x7fffffff` (`00d5fb70`) and the pool bounds. Clears the
/// 0x29-entry table at `+0xec` and gives it the free lists for the sizes 0x200
/// (`+0x16c`), 0x100 (`+0x12c`), 0 (`+0xec`) and 0x220 (`+0x210`..`+0x280`,
/// table indexes 0x21 to 0x28), then, from table index 0x28 down to 1, one list
/// (`00d60020`) for every slot still empty: 16 / 32 byte alignment (32 from index
/// 4 up) and a block size of `index * 16` times 16, or 0x1000 below 0x100.
/// Returns `this`.
pub fn hk_free_list_allocator_hk_free_list_allocator(
    e: &mut Engine,
    this: Ptr,
    parent: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.call(CRITICAL_SECTION_CONSTRUCTOR, &args![base + 4, 0xfa0u32]);
    e.mem.set_u32(base + 0x24, parent);
    e.call(
        LARGE_BLOCK_ALLOCATOR_CONSTRUCTOR,
        &args![base + LARGE_BLOCK_ALLOCATOR, parent],
    );
    e.mem.set_u32(base + LIST_COUNT, 0);
    for index in 0..=0x28u32 {
        e.call(
            FREE_LIST_DEFAULT_CONSTRUCTOR,
            &args![base + POOL + index * FREE_LIST_SIZE],
        );
    }
    e.mem.set_u32(base + 0xa98, 0);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x20, 0);
    hk_free_list_allocator_set_memory_soft_limit(e, this, 0x7fff_ffff);
    e.mem.set_u32(base + POOL_NEXT, base + POOL);
    e.mem.set_u32(
        base + POOL_END,
        base + POOL + FREE_LIST_COUNT * FREE_LIST_SIZE,
    );
    for index in 0..FREE_LIST_COUNT {
        e.mem.set_u32(base + SIZE_TABLE + index * 4, 0);
    }
    let list = fn_00d60020(e, this, 0x200, 0x80, 0x1_0000);
    e.mem.set_u32(base + 0x16c, list);
    let list = fn_00d60020(e, this, 0x100, 0x20, 0x8000);
    e.mem.set_u32(base + 0x12c, list);
    let list = fn_00d60020(e, this, 0, 4, 0x100);
    e.mem.set_u32(base + SIZE_TABLE, list);
    let list = fn_00d60020(e, this, 0x220, 0x80, 0x5000);
    for size in 0x210..=0x280u32 {
        e.mem.set_u32(base + SIZE_TABLE + (size >> 4) * 4, list);
    }
    for size in (0x10..=0x280u32).rev().step_by(0x10) {
        let slot = base + SIZE_TABLE + (size >> 4) * 4;
        if e.mem.u32(slot) == 0 {
            let alignment = if size < 0x40 { 0x10 } else { 0x20 };
            let block_size = if size >= 0x100 { size << 4 } else { 0x1000 };
            let list = fn_00d60020(e, this, size, alignment, block_size);
            e.mem.set_u32(slot, list);
        }
    }
    this
}

/// `DeleteCriticalSection` (`kernel32` import slot, `stdcall`, one word).
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// Callees outside this file. The large block allocator (the object at `+0x28`):
/// `getMemoryStatistics` (`00dd8d70`, statistics), the first-block iterator
/// (`00dd8b20`, out iterator of three words), the next-block step (`00dd8b60`,
/// byte out, iterator), `garbageCollect` (`00dd8790`), its quit (`00dd7fc0`) and its
/// destructor (`00dd7ec0`).
const LARGE_BLOCK_GET_STATISTICS: u32 = 0x00dd_8d70;
const LARGE_BLOCK_FIRST_BLOCK: u32 = 0x00dd_8b20;
const LARGE_BLOCK_NEXT_BLOCK: u32 = 0x00dd_8b60;
const LARGE_BLOCK_GARBAGE_COLLECT: u32 = 0x00dd_8790;
const LARGE_BLOCK_QUIT: u32 = 0x00dd_7fc0;
const LARGE_BLOCK_DESTRUCTOR: u32 = 0x00dd_7ec0;
/// `hkFreeList` functions: statistics (`00dd7430`, six-word out), `isOk` (`00dd7140`,
/// byte out), `findGarbage` (`00dd7680`, answers an `int`), `freeAllFreeBlocks`
/// (`00dd7360`), the unnamed `00dd7380` (one word), `addSpace` (`00dd7540`, answers a
/// block), the incremental collection (`00dd7c40`: byte out, count, collected-count
/// out; answers the byte out), `getBlocks` (`00dd7480`: out array, count) and
/// `walkMemory` (`00dd78f0`: callback, id, user word).
const FREE_LIST_GET_STATISTICS: u32 = 0x00dd_7430;
const FREE_LIST_IS_OK: u32 = 0x00dd_7140;
const FREE_LIST_FIND_GARBAGE: u32 = 0x00dd_7680;
const FREE_LIST_FREE_ALL_FREE_BLOCKS: u32 = 0x00dd_7360;
const FREE_LIST_CALL_00DD7380: u32 = 0x00dd_7380;
const FREE_LIST_ADD_SPACE: u32 = 0x00dd_7540;
const FREE_LIST_INCREMENTAL_COLLECT: u32 = 0x00dd_7c40;
const FREE_LIST_GET_BLOCKS: u32 = 0x00dd_7480;
const FREE_LIST_WALK_MEMORY: u32 = 0x00dd_78f0;
/// `hkOstream::printf` (`00cc0410`, cdecl: stream, format) and the format string at
/// `010d1400`.
const OSTREAM_PRINTF: u32 = 0x00cc_0410;
const PRINT_STATISTICS_FORMAT: u32 = 0x010d_1400;
/// `hkSetOutOfMemoryState` (`00d88040`, cdecl, one word).
const SET_OUT_OF_MEMORY_STATE: u32 = 0x00d8_8040;
/// The sort over arrays of block addresses (`00dd7d00`, cdecl: array, first, last,
/// comparison) and the comparison it is given (`00dd70f0`).
const SORT_BLOCKS: u32 = 0x00dd_7d00;
const COMPARE_BLOCKS: u32 = 0x00dd_70f0;
/// `hkMemoryAllocator::~hkMemoryAllocator` (`00cbf1a0`, the base destructor).
const MEMORY_ALLOCATOR_DESTRUCTOR: u32 = 0x00cb_f1a0;

/// The number of nodes of a chain whose first node is `head` (each node's first word is
/// the next node).
fn chain_length(e: &Engine, head: u32) -> u32 {
    let mut count = 0u32;
    let mut node = head;
    while node != 0 {
        node = e.mem.u32(node);
        count += 1;
    }
    count
}

/// The address of the table entry `+0xec + ((size + 15) >> 4) * 4` of the allocator at
/// `base` (an arithmetic shift of the wrapped sum, as the code computes it).
fn size_table_entry(base: u32, size: u32) -> u32 {
    let index = (size.wrapping_add(0xf) as i32) >> 4;
    base.wrapping_add((index as u32).wrapping_mul(4))
        .wrapping_add(SIZE_TABLE)
}

/// Adds the change of `list`'s count (`+0x28`) since `before`, times its element size
/// (`+4`), to the freed total at `+0x1c` of the allocator at `base`.
fn add_freed_bytes(e: &mut Engine, base: u32, list: u32, before: u32) {
    let bytes = e
        .mem
        .u32(list + 0x28)
        .wrapping_sub(before)
        .wrapping_mul(e.mem.u32(list + 4));
    let freed = e.mem.u32(base + 0x1c).wrapping_add(bytes);
    e.mem.set_u32(base + 0x1c, freed);
}

/// The tail every allocation shares: the peak (`+0x20`) takes the memory in use (`+0xe8`
/// less the freed total `+0x1c`) when larger, and when that is not below the soft limit
/// (`+0xa94`) `hkSetOutOfMemoryState(1)` runs.
fn note_memory_in_use(e: &mut Engine, base: u32) {
    let in_use = e.mem.u32(base + 0xe8).wrapping_sub(e.mem.u32(base + 0x1c));
    let mut peak = e.mem.u32(base + 0x20);
    if in_use > peak {
        peak = in_use;
    }
    e.mem.set_u32(base + 0x20, peak);
    if in_use >= e.mem.u32(base + 0xa94) {
        e.call(SET_OUT_OF_MEMORY_STATE, &args![1u32]);
    }
}

// Translated from 00d5f750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::getMemoryStatistics` (Xbox PDB), `thiscall`, `RET 4`: inside the
/// critical section at `+4`, asks the large block allocator (`+0x28`) to add its
/// numbers to `stats`, then adds those of every free list in the list of lists
/// (`+0x190`, `+0x234` entries; each answers six words in a local): the word at `+0xc`
/// of `stats` grows by the sum of the lists' fourth words, `+4` by the sum of their
/// second words less the sum of their first, `+8` is the peak at `+0x20`, and `+0x14`
/// takes a list's element size (`+4`) when that is larger and the list has a chain
/// (`+0`) or unused space (`+0x20` below `+0x24`). `+0x10` is -1 when the parent
/// allocator (`+0x24`, slot `0x18`) answers -1, else `stats + 0xc` plus its answer.
pub fn hk_free_list_allocator_get_memory_statistics(e: &mut Engine, this: Ptr, stats: Ptr) {
    let base = this.addr();
    let stats = stats.addr();
    let section = base + 4;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    e.call(
        LARGE_BLOCK_GET_STATISTICS,
        &args![base + LARGE_BLOCK_ALLOCATOR, stats],
    );
    let mut sum_second = 0u32;
    let mut sum_fourth = 0u32;
    let mut sum_first = 0u32;
    if e.mem.i32(base + LIST_COUNT) > 0 {
        let mut slot = base + LIST_OF_LISTS;
        let mut index = 0i32;
        loop {
            let list = e.mem.u32(slot);
            let numbers = e.with_stack(0x18, |e, out| {
                e.call(FREE_LIST_GET_STATISTICS, &args![list, out]);
                let at = out.addr();
                (e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 12))
            });
            if (e.mem.u32(list) != 0 || e.mem.u32(list + 0x20) < e.mem.u32(list + 0x24))
                && e.mem.u32(list + 4) > e.mem.u32(stats + 0x14)
            {
                let size = e.mem.u32(list + 4);
                e.mem.set_u32(stats + 0x14, size);
            }
            sum_second = sum_second.wrapping_add(numbers.1);
            sum_fourth = sum_fourth.wrapping_add(numbers.2);
            sum_first = sum_first.wrapping_add(numbers.0);
            slot += 4;
            index += 1;
            if index >= e.mem.i32(base + LIST_COUNT) {
                break;
            }
        }
    }
    let total = e.mem.u32(stats + 0xc).wrapping_add(sum_fourth);
    e.mem.set_u32(stats + 0xc, total);
    let used = e
        .mem
        .u32(stats + 4)
        .wrapping_add(sum_second.wrapping_sub(sum_first));
    e.mem.set_u32(stats + 4, used);
    let peak = e.mem.u32(base + 0x20);
    e.mem.set_u32(stats + 8, peak);
    let parent = e.mem.u32(base + 0x24);
    let available = e.vcall(parent, 0x18, &args![]).u32();
    if available == u32::MAX {
        e.mem.set_u32(stats + 0x10, available);
    } else {
        let total = e.mem.u32(stats + 0xc);
        e.mem.set_u32(stats + 0x10, total.wrapping_add(available));
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d5f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::resetPeakMemoryStatistics` (Xbox PDB), `thiscall`: asks
/// `getMemoryStatistics` (slot `0x28`) to fill a local of six zeroed words and takes the
/// peak (`+0x20`) from its second word (the word at `+4`; the decompiler shows the
/// first, the disassembly reads `[ESP + 8]` with the out pointer at `[ESP + 4]`).
pub fn hk_free_list_allocator_reset_peak_memory_statistics(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x18, |e, stats| {
        e.vcall(base, 0x28, &args![stats]);
        let used = e.mem.u32(stats.addr() + 4);
        e.mem.set_u32(base + 0x20, used);
    });
}

// Translated from 00d5f890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::isOk` (Xbox PDB), `thiscall`, `RET 4`: writes 1 to the byte
/// `out`, then asks every free list of the list of lists (`00dd7140`, byte out) whether it
/// is in order; the first that answers 0 makes `out` 0 and ends the walk. Inside the
/// critical section at `+4`. Returns `out`.
pub fn hk_free_list_allocator_is_ok(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let base = this.addr();
    let section = base + 4;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    e.mem.set_u8(out.addr(), 1);
    if e.mem.i32(base + LIST_COUNT) > 0 {
        let mut index = 0i32;
        e.with_stack(4, |e, flag| loop {
            let list = e.mem.u32(base + LIST_OF_LISTS + (index as u32) * 4);
            e.call(FREE_LIST_IS_OK, &args![list, flag]);
            if e.mem.u8(flag.addr()) == 0 {
                e.mem.set_u8(out.addr(), 0);
                break;
            }
            index += 1;
            if index >= e.mem.i32(base + LIST_COUNT) {
                break;
            }
        });
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    out
}

// Translated from 00d5f910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::getAllocatedSize` (Xbox PDB), `thiscall`, `RET 8`
/// (`block`, `size`): 0 for size 0; for sizes up to 0x280 the element size (`+4`) of the
/// free list in the table at `+0xec` (index `(size + 15) >> 4`), unless that entry is
/// null; otherwise (and for larger sizes, as a tail call) what the large block allocator
/// (`+0x28`, slot `0x10`) answers for (`block`, `size`).
pub fn hk_free_list_allocator_get_allocated_size(
    e: &mut Engine,
    this: Ptr,
    block: u32,
    size: u32,
) -> u32 {
    let base = this.addr();
    if (size as i32) <= 0x280 {
        if size == 0 {
            return 0;
        }
        let list = e.mem.u32(size_table_entry(base, size));
        if list != 0 {
            return e.mem.u32(list + 4);
        }
    }
    e.vcall(base + LARGE_BLOCK_ALLOCATOR, 0x10, &args![block, size])
        .u32()
}

// Translated from 00d5f950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::printStatistics` (Xbox PDB), `thiscall`, `RET 4`: prints the
/// format string at `010d1400` to `stream` (`hkOstream::printf`, cdecl).
pub fn hk_free_list_allocator_print_statistics(e: &mut Engine, _this: Ptr, stream: u32) {
    e.call(OSTREAM_PRINTF, &args![stream, PRINT_STATISTICS_FORMAT]);
}

// Translated from 00d5f970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::blockAlloc` (Xbox PDB), `thiscall`, `RET 4`: calls `bufAlloc`
/// (slot `0xc`) with the address of its own argument word, which holds `size`, and
/// answers what it answers.
pub fn hk_free_list_allocator_block_alloc(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), size);
        e.vcall(this.addr(), 0xc, &args![slot]).u32()
    })
}

// Translated from 00d5f980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::blockFree` (Xbox PDB), `thiscall`, `RET 8` (`block`, `size`):
/// does nothing for a null block. Otherwise, inside the critical section at `+4`: for
/// sizes up to 0x280 pushes the block on the free list of its size (the block's first
/// word becomes the old head, the list's head `+0` the block, the list's count `+0x28`
/// grows by one) and adds the element size (`+4`) to the freed total at `+0x1c`; for
/// larger sizes the large block allocator (`+0x28`, slot 8) frees it.
pub fn hk_free_list_allocator_block_free(e: &mut Engine, this: Ptr, block: u32, size: u32) {
    let base = this.addr();
    if block == 0 {
        return;
    }
    let section = base + 4;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    if (size as i32) <= 0x280 {
        let list = e.mem.u32(size_table_entry(base, size));
        let count = e.mem.u32(list + 0x28);
        e.mem.set_u32(list + 0x28, count.wrapping_add(1));
        let head = e.mem.u32(list);
        e.mem.set_u32(block, head);
        let grown = e.mem.u32(list + 0x28).wrapping_sub(count);
        let bytes = grown.wrapping_mul(e.mem.u32(list + 4));
        e.mem.set_u32(list, block);
        let freed = e.mem.u32(base + 0x1c).wrapping_add(bytes);
        e.mem.set_u32(base + 0x1c, freed);
    } else {
        e.vcall(base + LARGE_BLOCK_ALLOCATOR, 8, &args![block]);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d5f9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::blockFreeBatch` (Xbox PDB), `thiscall`, `RET 0xc` (`blocks`,
/// `count`, `size`): inside the critical section at `+4`, for sizes up to 0x280 raises the
/// count of the size's free list (`+0x28`) by `count`, then walks the `count` entries of
/// `blocks` (a signed countdown): a non-null block is pushed on the list's chain, a null
/// one takes its share of the count back. The list's head becomes the last block and
/// the growth times the element size (`+4`) is added to the freed total at `+0x1c`.
/// Larger sizes free every non-null block through the large block allocator (`+0x28`,
/// slot 8).
pub fn hk_free_list_allocator_block_free_batch(
    e: &mut Engine,
    this: Ptr,
    blocks: u32,
    count: i32,
    size: u32,
) {
    let base = this.addr();
    let section = base + 4;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    if (size as i32) <= 0x280 {
        let list = e.mem.u32(size_table_entry(base, size));
        let before = e.mem.u32(list + 0x28);
        let mut head = e.mem.u32(list);
        e.mem
            .set_u32(list + 0x28, before.wrapping_add(count as u32));
        let mut remaining = count.wrapping_sub(1);
        let mut cursor = blocks;
        while remaining >= 0 {
            let block = e.mem.u32(cursor);
            cursor = cursor.wrapping_add(4);
            if block != 0 {
                e.mem.set_u32(block, head);
                head = block;
            } else {
                let taken = e.mem.u32(list + 0x28).wrapping_sub(1);
                e.mem.set_u32(list + 0x28, taken);
            }
            remaining = remaining.wrapping_sub(1);
        }
        let grown = e.mem.u32(list + 0x28).wrapping_sub(before);
        let bytes = grown.wrapping_mul(e.mem.u32(list + 4));
        e.mem.set_u32(list, head);
        let freed = e.mem.u32(base + 0x1c).wrapping_add(bytes);
        e.mem.set_u32(base + 0x1c, freed);
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        return;
    }
    let mut index = 0i32;
    if count > 0 {
        loop {
            let block = e
                .mem
                .u32(blocks.wrapping_add((index as u32).wrapping_mul(4)));
            if block != 0 {
                e.vcall(base + LARGE_BLOCK_ALLOCATOR, 8, &args![block]);
            }
            index += 1;
            if index >= count {
                break;
            }
        }
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d5fab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::reservePagesFromSystemMemory` (Xbox PDB), `thiscall`, `RET 4`:
/// does nothing.
pub fn hk_free_list_allocator_reserve_pages_from_system_memory(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
) {
}

// Translated from 00d5fac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::garbageCollect` (Xbox PDB), `thiscall`: inside the critical
/// section at `+4`, repeats a pass over the list of lists while a pass found a list with
/// a chain at `+0x10` and some list's `findGarbage` (`00dd7680`) has answered a negative
/// number (that flag is never reset, so once set it keeps the passes going). Each list
/// is asked for its garbage, has its free blocks freed (`00dd7360`) and the change of
/// its count (`+0x28`) times its element size (`+4`) is added to the freed total at
/// `+0x1c`. Ends with the large block allocator's `garbageCollect` (`00dd8790`).
pub fn hk_free_list_allocator_garbage_collect(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let section = base + 4;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let mut negative_seen = false;
    loop {
        let mut chain_seen = false;
        if e.mem.i32(base + LIST_COUNT) <= 0 {
            break;
        }
        let mut slot = base + LIST_OF_LISTS;
        let mut index = 0i32;
        loop {
            let list = e.mem.u32(slot);
            let before = e.mem.u32(list + 0x28);
            if e.call(FREE_LIST_FIND_GARBAGE, &args![list]).i32() < 0 {
                negative_seen = true;
            }
            if e.mem.u32(list + 0x10) != 0 {
                chain_seen = true;
            }
            e.call(FREE_LIST_FREE_ALL_FREE_BLOCKS, &args![list]);
            add_freed_bytes(e, base, list, before);
            index += 1;
            slot += 4;
            if index >= e.mem.i32(base + LIST_COUNT) {
                break;
            }
        }
        if !chain_seen || !negative_seen {
            break;
        }
    }
    e.call(
        LARGE_BLOCK_GARBAGE_COLLECT,
        &args![base + LARGE_BLOCK_ALLOCATOR],
    );
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d5fba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::getMemorySoftLimit` (Xbox PDB): the soft limit at `+0xa94`.
pub fn hk_free_list_allocator_get_memory_soft_limit(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xa94)
}

// Translated from 00d5fbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::optimize` (Xbox PDB), `thiscall`: inside the critical section
/// at `+4`, for every free list of the list of lists: when `findGarbage` (`00dd7680`)
/// answers a positive number, calls `00dd7380` with the list's word at `+0x14` less a
/// tenth of it (unsigned division by 10); the change of the list's count (`+0x28`) times
/// its element size (`+4`) is added to the freed total at `+0x1c`.
pub fn hk_free_list_allocator_optimize(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let section = base + 4;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    if e.mem.i32(base + LIST_COUNT) > 0 {
        let mut slot = base + LIST_OF_LISTS;
        let mut index = 0i32;
        loop {
            let list = e.mem.u32(slot);
            let before = e.mem.u32(list + 0x28);
            if e.call(FREE_LIST_FIND_GARBAGE, &args![list]).i32() > 0 {
                let word = e.mem.u32(list + 0x14);
                e.call(FREE_LIST_CALL_00DD7380, &args![list, word - word / 10]);
            }
            add_freed_bytes(e, base, list, before);
            index += 1;
            slot += 4;
            if index >= e.mem.i32(base + LIST_COUNT) {
                break;
            }
        }
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d5fc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::releaseUnusedPagesToSystemMemory` (Xbox PDB), `thiscall`: a tail
/// call of the virtual function at slot `0x40` (`garbageCollect`, `00d5fac0`, by the vtable at
/// `010d1464`) with the same `this`; answers what it answers.
pub fn hk_free_list_allocator_release_unused_pages_to_system_memory(
    e: &mut Engine,
    this: Ptr,
) -> u32 {
    e.vcall(this.addr(), 0x40, &args![]).u32()
}

// Translated from 00d5fc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::canAllocTotal` (Xbox PDB), `thiscall`, `RET 4`: the memory in use
/// is the word at `+0xe8` less the freed total at `+0x1c`; the peak at `+0x20` takes it
/// when it is larger (unsigned). True when the memory in use is below the soft limit
/// (`+0xa94`) less `size` (unsigned).
pub fn hk_free_list_allocator_can_alloc_total(e: &mut Engine, this: Ptr, size: u32) -> bool {
    let base = this.addr();
    let in_use = e.mem.u32(base + 0xe8).wrapping_sub(e.mem.u32(base + 0x1c));
    let mut peak = e.mem.u32(base + 0x20);
    if in_use > peak {
        peak = in_use;
    }
    e.mem.set_u32(base + 0x20, peak);
    in_use < e.mem.u32(base + 0xa94).wrapping_sub(size)
}

// Translated from 00d5fca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::bufAlloc` (Xbox PDB), `thiscall`, `RET 4` (`size`, a pointer to
/// the requested size that is written back rounded): inside the critical section at `+4`,
/// for sizes up to 0x280 takes a block of the size's free list: the head of its chain
/// (`+0`) when there is one (the count at `+0x28` falls by one, the head moves on to the
/// block's first word), else from unused space (`+0x20` below `+0x24`: the next element,
/// `+0x20` grows by the element size `+4`) or from `addSpace` (`00dd7540`). The change
/// of the count times the element size is added to the freed total at `+0x1c` and the
/// element size is written to `*size`. Larger sizes come from the large block allocator
/// (`+0x28`, slot 4). Then the peak and the soft limit are checked (the tail shared with
/// `blockAllocBatch`). Answers the block.
pub fn hk_free_list_allocator_buf_alloc(e: &mut Engine, this: Ptr, size: Ptr) -> u32 {
    let base = this.addr();
    let requested = e.mem.u32(size.addr());
    let section = base + 4;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let block = if (requested as i32) <= 0x280 {
        let list = e.mem.u32(size_table_entry(base, requested));
        let mut taken = e.mem.u32(list);
        let count = e.mem.u32(list + 0x28);
        if taken != 0 {
            e.mem.set_u32(list + 0x28, count.wrapping_sub(1));
            let next = e.mem.u32(taken);
            e.mem.set_u32(list, next);
        } else {
            taken = e.mem.u32(list + 0x20);
            if taken >= e.mem.u32(list + 0x24) {
                taken = e.call(FREE_LIST_ADD_SPACE, &args![list]).u32();
            } else {
                let next = e.mem.u32(list + 4).wrapping_add(taken);
                e.mem.set_u32(list + 0x28, count.wrapping_sub(1));
                e.mem.set_u32(list + 0x20, next);
            }
        }
        add_freed_bytes(e, base, list, count);
        let rounded = e.mem.u32(list + 4);
        e.mem.set_u32(size.addr(), rounded);
        taken
    } else {
        e.vcall(base + LARGE_BLOCK_ALLOCATOR, 4, &args![requested])
            .u32()
    };
    note_memory_in_use(e, base);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    block
}

// Translated from 00d5fd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::blockAllocBatch` (Xbox PDB), `thiscall`, `RET 0xc` (`blocks`,
/// `count`, `size`): inside the critical section at `+4`, fills the `count` words at
/// `blocks`. For sizes up to 0x280 it takes blocks of the size's free list: first from
/// the chain at `+0` (the count at `+0x28` falls by the number taken), then from unused
/// space (`+0x20` below `+0x24`, stepping by the element size `+4`, with the count
/// reduced by the number taken each time the space runs out or the array is full) or,
/// when there is no unused space, one block at a time from `addSpace` (`00dd7540`); the
/// change of the count times the element size is added to the freed total at `+0x1c`.
/// Larger sizes take each block from the large block allocator (`+0x28`, slot 4). Then
/// the peak and the soft limit are checked (the tail shared with `bufAlloc`).
pub fn hk_free_list_allocator_block_alloc_batch(
    e: &mut Engine,
    this: Ptr,
    blocks: u32,
    count: i32,
    size: u32,
) {
    let base = this.addr();
    let section = base + 4;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    if (size as i32) <= 0x280 {
        let list = e.mem.u32(size_table_entry(base, size));
        let before = e.mem.u32(list + 0x28);
        let mut head = e.mem.u32(list);
        let end = blocks.wrapping_add((count as u32).wrapping_mul(4));
        let mut cursor = blocks;
        while cursor < end && head != 0 {
            e.mem.set_u32(cursor, head);
            head = e.mem.u32(head);
            cursor = cursor.wrapping_add(4);
        }
        e.mem.set_u32(list, head);
        let taken = (cursor.wrapping_sub(blocks) as i32) >> 2;
        let left = e.mem.u32(list + 0x28).wrapping_sub(taken as u32);
        e.mem.set_u32(list + 0x28, left);
        while cursor < end {
            let chunk_start = cursor;
            if e.mem.u32(list + 0x20) >= e.mem.u32(list + 0x24) {
                let added = e.call(FREE_LIST_ADD_SPACE, &args![list]).u32();
                e.mem.set_u32(cursor, added);
                cursor = cursor.wrapping_add(4);
            } else {
                while cursor < end {
                    let space = e.mem.u32(list + 0x20);
                    e.mem.set_u32(cursor, space);
                    let step = e.mem.u32(list + 4);
                    let moved = e.mem.u32(list + 0x20).wrapping_add(step);
                    e.mem.set_u32(list + 0x20, moved);
                    cursor = cursor.wrapping_add(4);
                    if e.mem.u32(list + 0x20) >= e.mem.u32(list + 0x24) {
                        break;
                    }
                }
                let taken = (cursor.wrapping_sub(chunk_start) as i32) >> 2;
                let left = e.mem.u32(list + 0x28).wrapping_sub(taken as u32);
                e.mem.set_u32(list + 0x28, left);
            }
        }
        add_freed_bytes(e, base, list, before);
    } else {
        let mut index = 0i32;
        if count > 0 {
            loop {
                let block = e.vcall(base + LARGE_BLOCK_ALLOCATOR, 4, &args![size]).u32();
                e.mem
                    .set_u32(blocks.wrapping_add((index as u32).wrapping_mul(4)), block);
                index += 1;
                if index >= count {
                    break;
                }
            }
        }
    }
    note_memory_in_use(e, base);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d5feb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::incrementalGarbageCollect` (Xbox PDB), `thiscall`, `RET 4`
/// (`count`, raised to at least 1): inside the critical section at `+4`, starting at the
/// list index kept at `+0xa98`, asks each free list of the list of lists for an
/// incremental collection (`00dd7c40`: byte out, the number still wanted, out for the
/// number collected; the byte out is the function's own argument word) and moves to the
/// next list (back to 0 at the end) when the byte is set, then frees the list's free
/// blocks (`00dd7360`) and adds the change of its count (`+0x28`) times its element size
/// (`+4`) to the freed total at `+0x1c`. Stops when nothing more is wanted or the walk
/// is back at the first list; `+0xa98` keeps the index it stopped at.
pub fn hk_free_list_allocator_incremental_garbage_collect(e: &mut Engine, this: Ptr, count: i32) {
    let base = this.addr();
    let section = base + 4;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let mut wanted = if count < 1 { 1 } else { count };
    let start = e.mem.u32(base + 0xa98);
    let mut index = start;
    // The byte out is the exe's argument word, the collected count a local next to it.
    e.with_stack(8, |e, scratch| {
        let flag_slot = scratch.addr();
        let collected_slot = flag_slot + 4;
        e.mem.set_u32(flag_slot, count as u32);
        loop {
            let list = e.mem.u32(base + LIST_OF_LISTS + index.wrapping_mul(4));
            let before = e.mem.u32(list + 0x28);
            let flag = e
                .call(
                    FREE_LIST_INCREMENTAL_COLLECT,
                    &args![list, flag_slot, wanted, collected_slot],
                )
                .u32();
            if e.mem.u8(flag) != 0 {
                index = index.wrapping_add(1);
                if (index as i32) >= e.mem.i32(base + LIST_COUNT) {
                    index = 0;
                }
            }
            e.call(FREE_LIST_FREE_ALL_FREE_BLOCKS, &args![list]);
            wanted = wanted.wrapping_sub(e.mem.u32(collected_slot) as i32);
            add_freed_bytes(e, base, list, before);
            if wanted <= 0 || index == start {
                break;
            }
        }
    });
    e.mem.set_u32(base + 0xa98, index);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d5ff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::quit` (Xbox PDB), `thiscall`: for every non-null entry of the list
/// of lists, in order, gives the list back to the pool (`00d5fc70`) and nulls every later
/// entry that is the same list. Ends by tail-calling the large block allocator's quit
/// (`00dd7fc0`, `this + 0x28`).
pub fn hk_free_list_allocator_quit(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.i32(base + LIST_COUNT) > 0 {
        let mut next_index = 1i32;
        let mut slot = base + LIST_OF_LISTS;
        loop {
            let list = e.mem.u32(slot);
            if list != 0 {
                fn_00d5fc70(e, this, list);
                if next_index < e.mem.i32(base + LIST_COUNT) {
                    let mut later = next_index;
                    let mut at = slot + 4;
                    loop {
                        if e.mem.u32(at) == list {
                            e.mem.set_u32(at, 0);
                        }
                        later += 1;
                        at += 4;
                        if later >= e.mem.i32(base + LIST_COUNT) {
                            break;
                        }
                    }
                }
            }
            next_index += 1;
            slot += 4;
            if next_index.wrapping_sub(1) >= e.mem.i32(base + LIST_COUNT) {
                break;
            }
        }
    }
    e.call(LARGE_BLOCK_QUIT, &args![base + LARGE_BLOCK_ALLOCATOR]);
}

// Translated from 00d5ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::~hkFreeListAllocator` (Xbox PDB), `thiscall`: stores the virtual
/// table, runs `quit` (`00d5ff50`), runs `00dd7200` on the 0x29 free lists of the pool
/// (`+0x240`, 0x34 bytes each) from the last to the first, destroys the large block
/// allocator (`00dd7ec0`), deletes the critical section at `+4` and tail-calls the base
/// destructor (`00cbf1a0`).
pub fn hk_free_list_allocator_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    hk_free_list_allocator_quit(e, this);
    let mut list = base + 0xa94;
    let mut remaining = 0x28i32;
    loop {
        list = list.wrapping_sub(FREE_LIST_SIZE);
        e.call(FREE_LIST_CLEAR, &args![list]);
        remaining -= 1;
        if remaining < 0 {
            break;
        }
    }
    e.call(LARGE_BLOCK_DESTRUCTOR, &args![base + LARGE_BLOCK_ALLOCATOR]);
    e.call(DELETE_CRITICAL_SECTION, &args![base + 4]);
    e.call(MEMORY_ALLOCATOR_DESTRUCTOR, &args![base]);
}

// Translated from 00d600c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::walkMemory` (Xbox PDB), `thiscall`, `RET 8` (`callback`, `user`):
/// counts the blocks in the two chains (`+0x10` and `+0xc`) of every free list of the list
/// of lists and asks the parent allocator (`+0x24`, slot `0x24`) for that many words
/// (answers 1 when it has none). Fills the words with the blocks' addresses
/// (`00dd7480` per list) and, with more than one, sorts them (`00dd7d00`, comparison
/// `00dd70f0`). Then walks the large block allocator's blocks (`00dd8b20` first,
/// `00dd8b60` next): a block whose payload (`block + 0x10`) is the next sorted address is
/// passed over, every other is given to the cdecl `callback(payload, size & ~3, flag,
/// 0, user)`, where `flag` is bit 1 of the size word in the low byte and the high bytes
/// are those of `callback` itself (the exe passes its own argument word). The words are
/// given back (parent slot `0x28`) and every free list is walked (`00dd78f0`:
/// `callback`, its position 1-based, `user`). Answers 0.
pub fn hk_free_list_allocator_walk_memory(
    e: &mut Engine,
    this: Ptr,
    callback: u32,
    user: u32,
) -> u32 {
    let base = this.addr();
    let count = e.mem.i32(base + LIST_COUNT);
    let mut total = 0u32;
    if count > 0 {
        let mut slot = base + LIST_OF_LISTS;
        let mut left = count as u32;
        loop {
            let list = e.mem.u32(slot);
            let second = chain_length(e, e.mem.u32(list + 0x10));
            let first = chain_length(e, e.mem.u32(list + 0xc));
            total = total.wrapping_add(first.wrapping_add(second));
            slot += 4;
            left = left.wrapping_sub(1);
            if left == 0 {
                break;
            }
        }
    }
    let parent = e.mem.u32(base + 0x24);
    let bytes = total.wrapping_mul(4);
    let array = e.vcall(parent, 0x24, &args![bytes]).u32();
    if array == 0 {
        return array.wrapping_add(1);
    }
    if e.mem.i32(base + LIST_COUNT) > 0 {
        let mut slot = base + LIST_OF_LISTS;
        let mut index = 0i32;
        let mut out = array;
        loop {
            let list = e.mem.u32(slot);
            let second = chain_length(e, e.mem.u32(list + 0x10));
            let first = chain_length(e, e.mem.u32(list + 0xc));
            let length = first.wrapping_add(second);
            e.call(FREE_LIST_GET_BLOCKS, &args![list, out, length]);
            index += 1;
            slot += 4;
            out = out.wrapping_add(length.wrapping_mul(4));
            if index >= e.mem.i32(base + LIST_COUNT) {
                break;
            }
        }
    }
    if (total as i32) > 1 {
        e.call(
            SORT_BLOCKS,
            &args![array, 0u32, total.wrapping_sub(1), COMPARE_BLOCKS],
        );
    }
    e.with_stack(0xc, |e, iterator| {
        e.call(
            LARGE_BLOCK_FIRST_BLOCK,
            &args![base + LARGE_BLOCK_ALLOCATOR, iterator],
        );
        let mut cursor = array;
        let mut block = e.mem.u32(iterator.addr());
        if block != 0 {
            e.with_stack(4, |e, has_next| loop {
                let payload = block.wrapping_add(0x10);
                if payload == e.mem.u32(cursor) {
                    cursor = cursor.wrapping_add(4);
                } else {
                    let header = e.mem.u32(block + 4);
                    let flag = (header >> 1) & 1;
                    let flag_word = (callback & 0xffff_ff00) | flag;
                    e.call(
                        callback,
                        &args![payload, header & 0xffff_fffc, flag_word, 0u32, user],
                    );
                }
                e.call(
                    LARGE_BLOCK_NEXT_BLOCK,
                    &args![base + LARGE_BLOCK_ALLOCATOR, has_next, iterator],
                );
                block = e.mem.u32(iterator.addr());
                if block == 0 {
                    break;
                }
            });
        }
    });
    let parent = e.mem.u32(base + 0x24);
    e.vcall(parent, 0x28, &args![array, bytes]);
    if e.mem.i32(base + LIST_COUNT) > 0 {
        let mut slot = base + LIST_OF_LISTS;
        let mut index = 0u32;
        loop {
            let list = e.mem.u32(slot);
            index += 1;
            e.call(FREE_LIST_WALK_MEMORY, &args![list, callback, index, user]);
            slot += 4;
            if (index as i32) >= e.mem.i32(base + LIST_COUNT) {
                break;
            }
        }
    }
    0
}

// Translated from 00d603e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeListAllocator::_scalar_deleting_destructor_` (Xbox PDB), `thiscall`, `RET 4`:
/// runs the destructor (`00d5ffd0`) and answers `this`; the flags word is never read and
/// the memory is not freed here.
pub fn hk_free_list_allocator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    hk_free_list_allocator_destructor(e, this);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5fb70,
            hk_free_list_allocator_set_memory_soft_limit(Ptr, u32)
        ),
        entry!(0x00d5fc70, fn_00d5fc70(Ptr, u32)),
        entry!(0x00d60020, fn_00d60020(Ptr, u32, u32, u32) -> u32),
        entry!(
            0x00d60270,
            hk_free_list_allocator_hk_free_list_allocator(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00d5f750,
            hk_free_list_allocator_get_memory_statistics(Ptr, Ptr)
        ),
        entry!(
            0x00d5f850,
            hk_free_list_allocator_reset_peak_memory_statistics(Ptr)
        ),
        entry!(0x00d5f890, hk_free_list_allocator_is_ok(Ptr, Ptr) -> Ptr),
        entry!(0x00d5f910, hk_free_list_allocator_get_allocated_size(Ptr, u32, u32) -> u32),
        entry!(
            0x00d5f950,
            hk_free_list_allocator_print_statistics(Ptr, u32)
        ),
        entry!(0x00d5f970, hk_free_list_allocator_block_alloc(Ptr, u32) -> u32),
        entry!(0x00d5f980, hk_free_list_allocator_block_free(Ptr, u32, u32)),
        entry!(
            0x00d5f9f0,
            hk_free_list_allocator_block_free_batch(Ptr, u32, i32, u32)
        ),
        entry!(
            0x00d5fab0,
            hk_free_list_allocator_reserve_pages_from_system_memory(Ptr, u32)
        ),
        entry!(0x00d5fac0, hk_free_list_allocator_garbage_collect(Ptr)),
        entry!(0x00d5fba0, hk_free_list_allocator_get_memory_soft_limit(Ptr) -> u32),
        entry!(0x00d5fbb0, hk_free_list_allocator_optimize(Ptr)),
        entry!(0x00d5fc30, hk_free_list_allocator_release_unused_pages_to_system_memory(Ptr) -> u32),
        entry!(0x00d5fc40, hk_free_list_allocator_can_alloc_total(Ptr, u32) -> bool),
        entry!(0x00d5fca0, hk_free_list_allocator_buf_alloc(Ptr, Ptr) -> u32),
        entry!(
            0x00d5fd70,
            hk_free_list_allocator_block_alloc_batch(Ptr, u32, i32, u32)
        ),
        entry!(
            0x00d5feb0,
            hk_free_list_allocator_incremental_garbage_collect(Ptr, i32)
        ),
        entry!(0x00d5ff50, hk_free_list_allocator_quit(Ptr)),
        entry!(0x00d5ffd0, hk_free_list_allocator_destructor(Ptr)),
        entry!(0x00d600c0, hk_free_list_allocator_walk_memory(Ptr, u32, u32) -> u32),
        entry!(0x00d603e0, hk_free_list_allocator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: u32 = 0xaa0;

    fn calls_to(log: &[(u32, Vec<u32>)], target: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == target)
            .map(|call| call.1.clone())
            .collect()
    }

    fn allocator_object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(SIZE + 0x20);
        e.mem.write(object, &vec![0xcd; SIZE as usize]);
        object
    }

    /// The free list constructor double stores the rounded size at +4, as
    /// `element size` rounded to 16.
    fn free_list_double(e: &mut Engine) {
        e.register(FREE_LIST_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 4, (a[1] + 15) & !15);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(FREE_LIST_CLEAR, |_, _| Ret::default());
    }

    #[test]
    fn the_soft_limit_is_stored_inside_the_critical_section() {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        let object = allocator_object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00d5_fb70, &args![object, 0x1234u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|call| call.0).collect();
        assert_eq!(
            order,
            vec![0x00d5_fb70, ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION]
        );
        assert_eq!(log[1].1, vec![object + 4]);
        assert_eq!(log[2].1, vec![object + 4]);
        assert_eq!(e.mem.u32(object + 0xa94), 0x1234);
    }

    #[test]
    fn giving_back_the_last_list_moves_the_pool_down() {
        let mut e = Engine::new();
        free_list_double(&mut e);
        let object = allocator_object(&mut e);
        let last = object + 0x240 + 0x34;
        e.mem.set_u32(object + 0x238, last + 0x34);
        e.call_log = Some(vec![]);
        e.call(0x00d5_fc70, &args![object, last]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FREE_LIST_CLEAR), vec![vec![last]]);
        assert_eq!(e.mem.u32(object + 0x238), last);
        // A list that is not the last one leaves the pool alone.
        e.call(0x00d5_fc70, &args![object, object + 0x300]);
        assert_eq!(e.mem.u32(object + 0x238), last);
    }

    #[test]
    fn a_new_size_appends_a_list_and_a_known_size_gives_it_back() {
        let mut e = Engine::new();
        free_list_double(&mut e);
        let object = allocator_object(&mut e);
        e.mem.set_u32(object + 0x234, 0);
        e.mem.set_u32(object + 0x238, object + 0x240);
        e.mem.set_u32(object + 0x23c, object + 0x240 + 0x34 * 4);
        e.call_log = Some(vec![]);
        let first = e.call(0x00d6_0020, &args![object, 0x40u32, 0x10u32, 0x800u32]);
        assert_eq!(first.u32(), object + 0x240);
        assert_eq!(e.mem.u32(object + 0x234), 1);
        assert_eq!(e.mem.u32(object + 0x190), object + 0x240);
        assert_eq!(e.mem.u32(object + 0x238), object + 0x240 + 0x34);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FREE_LIST_CONSTRUCTOR),
            vec![vec![object + 0x240, 0x40, 0x10, 0x800, object + 0x28]]
        );
        // Another size gets its own list.
        let second = e.call(0x00d6_0020, &args![object, 0x80u32, 0x10u32, 0x800u32]);
        assert_eq!(second.u32(), object + 0x240 + 0x34);
        assert_eq!(e.mem.u32(object + 0x234), 2);
        // The same rounded size (0x35 rounds to 0x40 in the double) returns the
        // first list; the pool element is given back.
        e.call_log = Some(vec![]);
        let again = e.call(0x00d6_0020, &args![object, 0x35u32, 0x10u32, 0x800u32]);
        assert_eq!(again.u32(), object + 0x240);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FREE_LIST_CLEAR),
            vec![vec![object + 0x240 + 0x34 * 2]]
        );
        assert_eq!(e.mem.u32(object + 0x234), 2);
        assert_eq!(e.mem.u32(object + 0x238), object + 0x240 + 0x34 * 2);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00d60034")]
    fn an_exhausted_pool_stops_where_the_exe_executes_int3() {
        let mut e = Engine::new();
        free_list_double(&mut e);
        let object = allocator_object(&mut e);
        e.mem.set_u32(object + 0x238, object + 0x240);
        e.mem.set_u32(object + 0x23c, object + 0x240);
        e.call(0x00d6_0020, &args![object, 0x40u32, 0x10u32, 0x800u32]);
    }

    fn constructor_engine() -> Engine {
        let mut e = Engine::new();
        e.register(CRITICAL_SECTION_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LARGE_BLOCK_ALLOCATOR_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(FREE_LIST_DEFAULT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        // Every list is distinct: rounded size = element size + the list's
        // address, so no pool element is given back.
        e.register(FREE_LIST_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 4, a[0]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(FREE_LIST_CLEAR, |_, _| Ret::default());
        e
    }

    #[test]
    fn the_constructor_builds_the_pool_and_the_size_table() {
        let mut e = constructor_engine();
        let object = allocator_object(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d6_0270, &args![object, 0x7000u32]).u32(), object);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(
            calls_to(&log, CRITICAL_SECTION_CONSTRUCTOR),
            vec![vec![object + 4, 0xfa0]]
        );
        assert_eq!(
            calls_to(&log, LARGE_BLOCK_ALLOCATOR_CONSTRUCTOR),
            vec![vec![object + 0x28, 0x7000]]
        );
        assert_eq!(e.mem.u32(object + 0x24), 0x7000);
        assert_eq!(calls_to(&log, FREE_LIST_DEFAULT_CONSTRUCTOR).len(), 41);
        assert_eq!(e.mem.u32(object + 0xa94), 0x7fff_ffff);
        assert_eq!(e.mem.u32(object + 0xa98), 0);
        assert_eq!(e.mem.u32(object + 0x1c), 0);
        assert_eq!(e.mem.u32(object + 0x20), 0);
        assert_eq!(e.mem.u32(object + 0x23c), object + 0x240 + 0x854);
        // The first four lists, in the order of the calls.
        let made = calls_to(&log, FREE_LIST_CONSTRUCTOR);
        let pool = |n: u32| object + 0x240 + n * 0x34;
        assert_eq!(made[0], vec![pool(0), 0x200, 0x80, 0x1_0000, object + 0x28]);
        assert_eq!(made[1], vec![pool(1), 0x100, 0x20, 0x8000, object + 0x28]);
        assert_eq!(made[2], vec![pool(2), 0, 4, 0x100, object + 0x28]);
        assert_eq!(made[3], vec![pool(3), 0x220, 0x80, 0x5000, object + 0x28]);
        assert_eq!(e.mem.u32(object + 0x16c), pool(0));
        assert_eq!(e.mem.u32(object + 0x12c), pool(1));
        assert_eq!(e.mem.u32(object + 0xec), pool(2));
        // Sizes 0x210 to 0x280 share the fourth list: table indexes 0x21 to 0x28.
        for index in 0x21..=0x28 {
            assert_eq!(e.mem.u32(object + 0xec + index * 4), pool(3));
        }
        // The remaining slots (1 to 0x20, except 0x12c/0x16c = 0x10, 0x16 ... see
        // below) each got a list, from 0x20 down to 1.
        let rest = &made[4..];
        // Table indexes 0x20 down to 1 minus the two already set (0x12c >> 4 is
        // out of table range; 0x12c and 0x16c are +0x40 and +0x80 from +0xec,
        // that is indexes 0x10 and 0x20).
        assert_eq!(rest.len(), 0x20 - 2);
        // First of the rest is index 0x1f (size 0x1f0): alignment 0x20, block
        // size 0x1f00.
        assert_eq!(rest[0][1..], [0x1f0, 0x20, 0x1f00, object + 0x28]);
        // The last is index 1 (size 0x10): alignment 0x10, block size 0x1000.
        assert_eq!(
            rest[rest.len() - 1][1..],
            [0x10, 0x10, 0x1000, object + 0x28]
        );
        // The pool holds 4 + 30 lists in all.
        assert_eq!(e.mem.u32(object + 0x234), 34);
        assert_eq!(e.mem.u32(object + 0x238), pool(34));
    }

    // ---- tests of the second batch (statistics, block functions, walkMemory) ----

    const LARGE_VTABLE: u32 = 0x0900_2000;
    const PARENT_VTABLE: u32 = 0x0900_2100;
    const OWN_VTABLE: u32 = 0x0900_2200;
    const SLOT_TARGET_A: u32 = 0x0900_0101;
    const SLOT_TARGET_B: u32 = 0x0900_0102;
    const SLOT_TARGET_C: u32 = 0x0900_0103;

    fn section_engine() -> Engine {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e
    }

    fn zeroed_allocator(e: &mut Engine) -> u32 {
        e.mem.alloc(0xaa8)
    }

    /// A free list stand-in: 0x80 zeroed bytes (the test sets the words it reads).
    fn list_block(e: &mut Engine) -> u32 {
        e.mem.alloc(0x80)
    }

    /// Puts the vtable `address` (0x80 bytes) with the given (byte offset, target) slots.
    fn vtable_at(e: &mut Engine, address: u32, slots: &[(u32, u32)]) {
        e.map(address, 0x80);
        for (offset, target) in slots {
            e.mem.set_u32(address + offset, *target);
        }
    }

    fn list_at(e: &mut Engine, allocator: u32, index: u32, list: u32) {
        e.mem.set_u32(allocator + 0x190 + 4 * index, list);
    }

    fn called(e: &mut Engine, address: u32) -> Vec<Vec<u32>> {
        calls_to(e.call_log.as_ref().unwrap(), address)
    }

    #[test]
    fn the_statistics_add_up_the_free_lists_and_the_parent() {
        fn copy_statistics(e: &mut Engine, a: &[u32]) -> Ret {
            for i in 0..6 {
                let word = e.mem.u32(a[0] + 0x34 + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            Ret::default()
        }
        let mut e = section_engine();
        e.register(FREE_LIST_GET_STATISTICS, copy_statistics);
        e.register(LARGE_BLOCK_GET_STATISTICS, |_, _| Ret::default());
        let allocator = zeroed_allocator(&mut e);
        let (first, second) = (list_block(&mut e), list_block(&mut e));
        for (list, size, chain, space, words) in [
            (first, 0x40, 0, (1, 2), [10, 100, 0, 7, 0, 0]),
            (second, 0x20, 0x5000, (5, 5), [1, 50, 0, 3, 0, 0]),
        ] {
            e.mem.set_u32(list + 4, size);
            e.mem.set_u32(list, chain);
            e.mem.set_u32(list + 0x20, space.0);
            e.mem.set_u32(list + 0x24, space.1);
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(list + 0x34 + 4 * i as u32, *word);
            }
        }
        e.mem.set_u32(allocator + 0x234, 2);
        list_at(&mut e, allocator, 0, first);
        list_at(&mut e, allocator, 1, second);
        e.mem.set_u32(allocator + 0x20, 0x1234);
        let parent = e.mem.alloc(8);
        e.mem.set_u32(parent, PARENT_VTABLE);
        e.mem.set_u32(allocator + 0x24, parent);
        vtable_at(&mut e, PARENT_VTABLE, &[(0x18, SLOT_TARGET_A)]);
        e.register(SLOT_TARGET_A, |_, _| Ret {
            eax: 0x777,
            ..Ret::default()
        });
        let stats = e.mem.alloc(0x18);
        e.mem.set_u32(stats + 0xc, 1000);
        e.mem.set_u32(stats + 4, 500);
        e.mem.set_u32(stats + 0x14, 0x30);
        e.call_log = Some(vec![]);
        e.call(0x00d5_f750, &args![allocator, stats]);
        assert_eq!(
            called(&mut e, ENTER_CRITICAL_SECTION),
            vec![vec![allocator + 4]]
        );
        assert_eq!(
            called(&mut e, LEAVE_CRITICAL_SECTION),
            vec![vec![allocator + 4]]
        );
        assert_eq!(
            called(&mut e, LARGE_BLOCK_GET_STATISTICS),
            vec![vec![allocator + 0x28, stats]]
        );
        assert_eq!(e.mem.u32(stats + 0xc), 1010);
        assert_eq!(e.mem.u32(stats + 4), 500 + 150 - 11);
        assert_eq!(e.mem.u32(stats + 8), 0x1234);
        assert_eq!(e.mem.u32(stats + 0x14), 0x40);
        assert_eq!(e.mem.u32(stats + 0x10), 1010 + 0x777);
        // A parent that answers -1 makes the total -1.
        e.register(SLOT_TARGET_A, |_, _| Ret {
            eax: u32::MAX,
            ..Ret::default()
        });
        e.call(0x00d5_f750, &args![allocator, stats]);
        assert_eq!(e.mem.u32(stats + 0x10), u32::MAX);
    }

    #[test]
    fn resetting_the_peak_takes_the_second_statistics_word() {
        let mut e = section_engine();
        let allocator = zeroed_allocator(&mut e);
        e.mem.set_u32(allocator, OWN_VTABLE);
        vtable_at(&mut e, OWN_VTABLE, &[(0x28, SLOT_TARGET_A)]);
        e.register(SLOT_TARGET_A, |e, a| {
            for i in 0..6 {
                assert_eq!(e.mem.u32(a[1] + 4 * i), 0);
            }
            e.mem.set_u32(a[1], 0x11);
            e.mem.set_u32(a[1] + 4, 0x55);
            Ret::default()
        });
        e.mem.set_u32(allocator + 0x20, 0x9999);
        e.call(0x00d5_f850, &args![allocator]);
        assert_eq!(e.mem.u32(allocator + 0x20), 0x55);
    }

    #[test]
    fn is_ok_stops_at_the_first_list_that_is_not() {
        let mut e = section_engine();
        e.register(FREE_LIST_IS_OK, |e, a| {
            let answer = e.mem.u8(a[0] + 0x34);
            e.mem.set_u8(a[1], answer);
            Ret::default()
        });
        let allocator = zeroed_allocator(&mut e);
        let lists = [list_block(&mut e), list_block(&mut e), list_block(&mut e)];
        for (i, list) in lists.iter().enumerate() {
            list_at(&mut e, allocator, i as u32, *list);
            e.mem.set_u8(list + 0x34, 1);
        }
        e.mem.set_u32(allocator + 0x234, 3);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d5_f890, &args![allocator, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(called(&mut e, FREE_LIST_IS_OK).len(), 3);
        // The second list fails: the third is not asked.
        e.mem.set_u8(lists[1] + 0x34, 0);
        e.call_log = Some(vec![]);
        e.call(0x00d5_f890, &args![allocator, out]);
        assert_eq!(e.mem.u8(out), 0);
        let asked = called(&mut e, FREE_LIST_IS_OK);
        assert_eq!(asked.len(), 2);
        assert_eq!(asked[1][0], lists[1]);
        // No lists: the answer is 1 and nobody is asked.
        e.mem.set_u32(allocator + 0x234, 0);
        e.call_log = Some(vec![]);
        e.call(0x00d5_f890, &args![allocator, out]);
        assert_eq!(e.mem.u8(out), 1);
        assert!(called(&mut e, FREE_LIST_IS_OK).is_empty());
    }

    #[test]
    fn the_allocated_size_is_the_lists_or_the_large_allocators() {
        let mut e = section_engine();
        let allocator = zeroed_allocator(&mut e);
        e.mem.set_u32(allocator + 0x28, LARGE_VTABLE);
        vtable_at(&mut e, LARGE_VTABLE, &[(0x10, SLOT_TARGET_A)]);
        e.register(SLOT_TARGET_A, |_, a| Ret {
            eax: 0x4242 + a[2],
            ..Ret::default()
        });
        let list = list_block(&mut e);
        e.mem.set_u32(list + 4, 0x30);
        // Size 0x20 and 0x2f round to table index 2.
        e.mem.set_u32(allocator + 0xec + 2 * 4, list);
        assert_eq!(
            e.call(0x00d5_f910, &args![allocator, 0x5000u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(
            e.call(0x00d5_f910, &args![allocator, 0x5000u32, 0x20u32])
                .u32(),
            0x30
        );
        // A size whose table entry is null, and a size above 0x280, go to the large allocator.
        assert_eq!(
            e.call(0x00d5_f910, &args![allocator, 0x5000u32, 0x40u32])
                .u32(),
            0x4242 + 0x40
        );
        assert_eq!(
            e.call(0x00d5_f910, &args![allocator, 0x5000u32, 0x281u32])
                .u32(),
            0x4242 + 0x281
        );
    }

    #[test]
    fn printing_the_statistics_prints_the_format_string() {
        let mut e = Engine::new();
        e.register(OSTREAM_PRINTF, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d5_f950, &args![0x7000u32, 0x1234u32]);
        assert_eq!(
            called(&mut e, OSTREAM_PRINTF),
            vec![vec![0x1234, 0x010d_1400]]
        );
    }

    #[test]
    fn block_alloc_passes_the_address_of_the_size() {
        let mut e = section_engine();
        let allocator = zeroed_allocator(&mut e);
        e.mem.set_u32(allocator, OWN_VTABLE);
        vtable_at(&mut e, OWN_VTABLE, &[(0xc, SLOT_TARGET_A)]);
        e.register(SLOT_TARGET_A, |e, a| Ret {
            eax: e.mem.u32(a[1]) + 0x1000,
            ..Ret::default()
        });
        assert_eq!(
            e.call(0x00d5_f970, &args![allocator, 0x30u32]).u32(),
            0x1030
        );
    }

    #[test]
    fn block_free_pushes_on_the_list_or_goes_to_the_large_allocator() {
        let mut e = section_engine();
        let allocator = zeroed_allocator(&mut e);
        e.mem.set_u32(allocator + 0x28, LARGE_VTABLE);
        vtable_at(&mut e, LARGE_VTABLE, &[(8, SLOT_TARGET_A)]);
        e.register(SLOT_TARGET_A, |_, _| Ret::default());
        let list = list_block(&mut e);
        e.mem.set_u32(list + 4, 0x20);
        e.mem.set_u32(list, 0x7777);
        e.mem.set_u32(list + 0x28, 3);
        e.mem.set_u32(allocator + 0xec + 2 * 4, list);
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(allocator + 0x1c, 0x100);
        e.call_log = Some(vec![]);
        e.call(0x00d5_f980, &args![allocator, block, 0x1fu32]);
        assert_eq!(e.mem.u32(block), 0x7777);
        assert_eq!(e.mem.u32(list), block);
        assert_eq!(e.mem.u32(list + 0x28), 4);
        assert_eq!(e.mem.u32(allocator + 0x1c), 0x120);
        assert_eq!(called(&mut e, ENTER_CRITICAL_SECTION).len(), 1);
        // A null block does nothing at all.
        e.call_log = Some(vec![]);
        e.call(0x00d5_f980, &args![allocator, 0u32, 0x1fu32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // A large size goes to the large block allocator.
        e.call_log = Some(vec![]);
        e.call(0x00d5_f980, &args![allocator, block, 0x400u32]);
        assert_eq!(
            called(&mut e, SLOT_TARGET_A),
            vec![vec![allocator + 0x28, block]]
        );
        assert_eq!(e.mem.u32(list + 0x28), 4);
    }

    #[test]
    fn a_batch_of_blocks_is_pushed_and_nulls_give_their_share_back() {
        let mut e = section_engine();
        let allocator = zeroed_allocator(&mut e);
        e.mem.set_u32(allocator + 0x28, LARGE_VTABLE);
        vtable_at(&mut e, LARGE_VTABLE, &[(8, SLOT_TARGET_A)]);
        e.register(SLOT_TARGET_A, |_, _| Ret::default());
        let list = list_block(&mut e);
        e.mem.set_u32(list + 4, 0x20);
        e.mem.set_u32(list, 0x7777);
        e.mem.set_u32(list + 0x28, 5);
        e.mem.set_u32(allocator + 0xec + 2 * 4, list);
        let first = e.mem.alloc(0x20);
        let third = e.mem.alloc(0x20);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, first);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, third);
        e.call(0x00d5_f9f0, &args![allocator, array, 3u32, 0x20u32]);
        assert_eq!(e.mem.u32(first), 0x7777);
        assert_eq!(e.mem.u32(third), first);
        assert_eq!(e.mem.u32(list), third);
        assert_eq!(e.mem.u32(list + 0x28), 7);
        assert_eq!(e.mem.u32(allocator + 0x1c), 2 * 0x20);
        // Large sizes free every non-null block through the large allocator.
        e.call_log = Some(vec![]);
        e.call(0x00d5_f9f0, &args![allocator, array, 3u32, 0x400u32]);
        assert_eq!(
            called(&mut e, SLOT_TARGET_A),
            vec![vec![allocator + 0x28, first], vec![allocator + 0x28, third]]
        );
    }

    #[test]
    fn reserving_pages_does_nothing() {
        let mut e = section_engine();
        e.call_log = Some(vec![]);
        e.call(0x00d5_fab0, &args![0x7000u32, 0x1000u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn garbage_collection_repeats_while_chains_and_negative_answers_last() {
        fn find_garbage(e: &mut Engine, a: &[u32]) -> Ret {
            Ret {
                eax: e.mem.u32(a[0] + 0x34),
                ..Ret::default()
            }
        }
        fn free_all(e: &mut Engine, a: &[u32]) -> Ret {
            let budget = e.mem.u32(a[0] + 0x38);
            e.mem.set_u32(a[0] + 0x38, budget.wrapping_sub(1));
            e.mem
                .set_u32(a[0] + 0x10, if budget > 1 { 0x1000 } else { 0 });
            let count = e.mem.u32(a[0] + 0x28);
            e.mem.set_u32(a[0] + 0x28, count - 1);
            Ret::default()
        }
        let mut e = section_engine();
        e.register(FREE_LIST_FIND_GARBAGE, find_garbage);
        e.register(FREE_LIST_FREE_ALL_FREE_BLOCKS, free_all);
        e.register(LARGE_BLOCK_GARBAGE_COLLECT, |_, _| Ret::default());
        let allocator = zeroed_allocator(&mut e);
        let list = list_block(&mut e);
        e.mem.set_u32(list + 4, 0x10);
        e.mem.set_u32(list + 0x28, 10);
        e.mem.set_u32(list + 0x34, u32::MAX);
        e.mem.set_u32(list + 0x38, 2);
        e.mem.set_u32(list + 0x10, 0x1000);
        list_at(&mut e, allocator, 0, list);
        e.mem.set_u32(allocator + 0x234, 1);
        e.call_log = Some(vec![]);
        e.call(0x00d5_fac0, &args![allocator]);
        // Passes 1 and 2 leave a chain behind, pass 3 sees none: three passes.
        assert_eq!(called(&mut e, FREE_LIST_FREE_ALL_FREE_BLOCKS).len(), 3);
        assert_eq!(called(&mut e, FREE_LIST_FIND_GARBAGE).len(), 3);
        assert_eq!(
            called(&mut e, LARGE_BLOCK_GARBAGE_COLLECT),
            vec![vec![allocator + 0x28]]
        );
        assert_eq!(e.mem.u32(allocator + 0x1c), 0u32.wrapping_sub(3 * 0x10));
        // Without a negative answer one pass is enough.
        e.mem.set_u32(list + 0x34, 0);
        e.mem.set_u32(list + 0x38, 5);
        e.mem.set_u32(list + 0x10, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x00d5_fac0, &args![allocator]);
        assert_eq!(called(&mut e, FREE_LIST_FREE_ALL_FREE_BLOCKS).len(), 1);
        // No lists: only the large allocator is collected.
        e.mem.set_u32(allocator + 0x234, 0);
        e.call_log = Some(vec![]);
        e.call(0x00d5_fac0, &args![allocator]);
        assert!(called(&mut e, FREE_LIST_FIND_GARBAGE).is_empty());
        assert_eq!(called(&mut e, LARGE_BLOCK_GARBAGE_COLLECT).len(), 1);
    }

    #[test]
    fn the_soft_limit_is_read_back() {
        let mut e = Engine::new();
        let allocator = zeroed_allocator(&mut e);
        e.mem.set_u32(allocator + 0xa94, 0x4321);
        assert_eq!(e.call(0x00d5_fba0, &args![allocator]).u32(), 0x4321);
    }

    #[test]
    fn optimizing_trims_lists_with_positive_garbage_to_nine_tenths() {
        let mut e = section_engine();
        e.register(FREE_LIST_FIND_GARBAGE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x34),
            ..Ret::default()
        });
        e.register(FREE_LIST_CALL_00DD7380, |e, a| {
            let count = e.mem.u32(a[0] + 0x28);
            e.mem.set_u32(a[0] + 0x28, count - 1);
            Ret::default()
        });
        let allocator = zeroed_allocator(&mut e);
        let (first, second) = (list_block(&mut e), list_block(&mut e));
        for (list, garbage) in [(first, 5u32), (second, 0)] {
            e.mem.set_u32(list + 4, 0x20);
            e.mem.set_u32(list + 0x28, 8);
            e.mem.set_u32(list + 0x14, 100);
            e.mem.set_u32(list + 0x34, garbage);
        }
        list_at(&mut e, allocator, 0, first);
        list_at(&mut e, allocator, 1, second);
        e.mem.set_u32(allocator + 0x234, 2);
        e.call_log = Some(vec![]);
        e.call(0x00d5_fbb0, &args![allocator]);
        assert_eq!(
            called(&mut e, FREE_LIST_CALL_00DD7380),
            vec![vec![first, 90]]
        );
        assert_eq!(e.mem.u32(allocator + 0x1c), 0u32.wrapping_sub(0x20));
    }

    #[test]
    fn releasing_unused_pages_is_the_virtual_function() {
        let mut e = section_engine();
        let allocator = zeroed_allocator(&mut e);
        e.mem.set_u32(allocator, OWN_VTABLE);
        vtable_at(&mut e, OWN_VTABLE, &[(0x40, SLOT_TARGET_A)]);
        e.register(SLOT_TARGET_A, |_, a| Ret {
            eax: 0x99 + a[0],
            ..Ret::default()
        });
        assert_eq!(
            e.call(0x00d5_fc30, &args![allocator]).u32(),
            0x99 + allocator
        );
    }

    #[test]
    fn can_alloc_total_compares_with_the_soft_limit_and_raises_the_peak() {
        let mut e = Engine::new();
        let allocator = zeroed_allocator(&mut e);
        e.mem.set_u32(allocator + 0xe8, 1000);
        e.mem.set_u32(allocator + 0x1c, 200);
        e.mem.set_u32(allocator + 0x20, 500);
        e.mem.set_u32(allocator + 0xa94, 0x1000);
        assert!(e.call(0x00d5_fc40, &args![allocator, 0x100u32]).bool());
        assert_eq!(e.mem.u32(allocator + 0x20), 800);
        assert!(!e.call(0x00d5_fc40, &args![allocator, 0xd00u32]).bool());
        // A larger peak stays.
        e.mem.set_u32(allocator + 0x20, 900);
        e.call(0x00d5_fc40, &args![allocator, 0x100u32]);
        assert_eq!(e.mem.u32(allocator + 0x20), 900);
    }

    fn buf_alloc_setup(e: &mut Engine) -> (u32, u32, u32) {
        let allocator = zeroed_allocator(e);
        let list = list_block(e);
        e.mem.set_u32(list + 4, 0x40);
        e.mem.set_u32(list + 0x28, 3);
        e.mem.set_u32(allocator + 0xec + 4 * 4, list);
        e.mem.set_u32(allocator + 0xa94, 0xffff_0000);
        let size = e.mem.alloc(4);
        e.mem.set_u32(size, 0x3c);
        (allocator, list, size)
    }

    #[test]
    fn buf_alloc_takes_from_the_chain_then_the_space_then_add_space() {
        let mut e = section_engine();
        e.register(FREE_LIST_ADD_SPACE, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register(SET_OUT_OF_MEMORY_STATE, |_, _| Ret::default());
        let (allocator, list, size) = buf_alloc_setup(&mut e);
        // From the chain.
        let head = e.mem.alloc(8);
        e.mem.set_u32(head, 0x6666);
        e.mem.set_u32(list, head);
        e.mem.set_u32(allocator + 0xe8, 0x500);
        assert_eq!(e.call(0x00d5_fca0, &args![allocator, size]).u32(), head);
        assert_eq!(e.mem.u32(list), 0x6666);
        assert_eq!(e.mem.u32(list + 0x28), 2);
        assert_eq!(e.mem.u32(allocator + 0x1c), 0u32.wrapping_sub(0x40));
        assert_eq!(e.mem.u32(size), 0x40);
        assert_eq!(e.mem.u32(allocator + 0x20), 0x500 + 0x40);
        // From the unused space.
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 0x20, 0x1000);
        e.mem.set_u32(list + 0x24, 0x2000);
        e.mem.set_u32(size, 0x31);
        assert_eq!(e.call(0x00d5_fca0, &args![allocator, size]).u32(), 0x1000);
        assert_eq!(e.mem.u32(list + 0x20), 0x1040);
        assert_eq!(e.mem.u32(list + 0x28), 1);
        // From addSpace, when the space is used up.
        e.mem.set_u32(list + 0x20, 0x2000);
        assert_eq!(e.call(0x00d5_fca0, &args![allocator, size]).u32(), 0x7777);
        assert_eq!(e.mem.u32(list + 0x28), 1);
    }

    #[test]
    fn buf_alloc_of_a_large_size_uses_the_large_allocator_and_the_limit_is_checked() {
        let mut e = section_engine();
        e.register(SET_OUT_OF_MEMORY_STATE, |_, _| Ret::default());
        let (allocator, _, size) = buf_alloc_setup(&mut e);
        e.mem.set_u32(allocator + 0x28, LARGE_VTABLE);
        vtable_at(&mut e, LARGE_VTABLE, &[(4, SLOT_TARGET_A)]);
        e.register(SLOT_TARGET_A, |_, a| Ret {
            eax: 0x5000 + a[1],
            ..Ret::default()
        });
        e.mem.set_u32(size, 0x400);
        e.mem.set_u32(allocator + 0xe8, 0x100);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d5_fca0, &args![allocator, size]).u32(), 0x5400);
        assert_eq!(e.mem.u32(size), 0x400);
        assert!(called(&mut e, SET_OUT_OF_MEMORY_STATE).is_empty());
        // At the soft limit the out of memory state is raised.
        e.mem.set_u32(allocator + 0xa94, 0x100);
        e.call(0x00d5_fca0, &args![allocator, size]);
        assert_eq!(called(&mut e, SET_OUT_OF_MEMORY_STATE), vec![vec![1]]);
    }

    #[test]
    fn a_batch_of_blocks_comes_from_the_chain_the_space_and_add_space() {
        let mut e = section_engine();
        e.register(FREE_LIST_ADD_SPACE, |_, _| Ret {
            eax: 0x2000,
            ..Ret::default()
        });
        e.register(SET_OUT_OF_MEMORY_STATE, |_, _| Ret::default());
        let (allocator, list, _) = buf_alloc_setup(&mut e);
        e.mem.set_u32(list + 4, 0x10);
        e.mem.set_u32(list + 0x28, 10);
        e.mem.set_u32(allocator + 0xec + 4 * 4, list);
        let (second, first) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(first, second);
        e.mem.set_u32(second, 0);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 0x20, 0x1000);
        e.mem.set_u32(list + 0x24, 0x1020);
        let array = e.mem.alloc(0x20);
        e.call(0x00d5_fd70, &args![allocator, array, 5u32, 0x3cu32]);
        let got: Vec<u32> = (0..5).map(|i| e.mem.u32(array + 4 * i)).collect();
        assert_eq!(got, vec![first, second, 0x1000, 0x1010, 0x2000]);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(list + 0x20), 0x1020);
        assert_eq!(e.mem.u32(list + 0x28), 6);
        assert_eq!(e.mem.u32(allocator + 0x1c), 0u32.wrapping_sub(4 * 0x10));
    }

    #[test]
    fn a_batch_of_a_large_size_asks_the_large_allocator_per_block() {
        let mut e = section_engine();
        e.register(SET_OUT_OF_MEMORY_STATE, |_, _| Ret::default());
        let allocator = zeroed_allocator(&mut e);
        e.mem.set_u32(allocator + 0xa94, 0xffff_0000);
        e.mem.set_u32(allocator + 0x28, LARGE_VTABLE);
        vtable_at(&mut e, LARGE_VTABLE, &[(4, SLOT_TARGET_A)]);
        let mut next = 0x5000u32;
        e.register_double(SLOT_TARGET_A, move |_, a| {
            assert_eq!(a[1], 0x400);
            next += 0x100;
            Ret {
                eax: next,
                ..Ret::default()
            }
        });
        let array = e.mem.alloc(0x10);
        e.call(0x00d5_fd70, &args![allocator, array, 3u32, 0x400u32]);
        assert_eq!(e.mem.u32(array), 0x5100);
        assert_eq!(e.mem.u32(array + 4), 0x5200);
        assert_eq!(e.mem.u32(array + 8), 0x5300);
    }

    #[test]
    fn the_incremental_collection_walks_the_lists_until_enough_is_collected() {
        fn collect(e: &mut Engine, a: &[u32]) -> Ret {
            let step = e.mem.u32(a[0] + 0x34);
            e.mem.set_u8(a[1], e.mem.u8(a[0] + 0x38));
            e.mem.set_u32(a[3], step);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        }
        let mut e = section_engine();
        e.register(FREE_LIST_INCREMENTAL_COLLECT, collect);
        e.register(FREE_LIST_FREE_ALL_FREE_BLOCKS, |_, _| Ret::default());
        let allocator = zeroed_allocator(&mut e);
        let lists = [list_block(&mut e), list_block(&mut e), list_block(&mut e)];
        for (i, (list, collected)) in lists.iter().zip([2u32, 3, 4]).enumerate() {
            e.mem.set_u32(list + 4, 0x10);
            e.mem.set_u32(list + 0x34, collected);
            e.mem.set_u8(list + 0x38, 1);
            list_at(&mut e, allocator, i as u32, *list);
        }
        e.mem.set_u32(allocator + 0x234, 3);
        e.call_log = Some(vec![]);
        e.call(0x00d5_feb0, &args![allocator, 5u32]);
        let asked = called(&mut e, FREE_LIST_INCREMENTAL_COLLECT);
        assert_eq!(asked.len(), 2);
        assert_eq!((asked[0][2], asked[1][2]), (5, 3));
        assert_eq!(e.mem.u32(allocator + 0xa98), 2);
        // Starting at the last list the walk wraps to 0 and stops when it is back at the
        // start; a count below 1 asks for 1.
        e.mem.set_u32(allocator + 0xa98, 2);
        e.call_log = Some(vec![]);
        e.call(0x00d5_feb0, &args![allocator, 0u32.wrapping_sub(7)]);
        let asked = called(&mut e, FREE_LIST_INCREMENTAL_COLLECT);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][2], 1);
        assert_eq!(e.mem.u32(allocator + 0xa98), 0);
        // A byte out of 0 keeps the index, which is the start: the walk stops after one list.
        e.mem.set_u8(lists[0] + 0x38, 0);
        e.mem.set_u32(lists[0] + 0x34, 0);
        e.mem.set_u32(allocator + 0xa98, 0);
        e.call_log = Some(vec![]);
        e.call(0x00d5_feb0, &args![allocator, 1u32]);
        assert_eq!(called(&mut e, FREE_LIST_INCREMENTAL_COLLECT).len(), 1);
        assert_eq!(e.mem.u32(allocator + 0xa98), 0);
    }

    #[test]
    fn quit_gives_each_list_back_once_and_nulls_the_duplicates() {
        let mut e = section_engine();
        e.register(FREE_LIST_CLEAR, |_, _| Ret::default());
        e.register(LARGE_BLOCK_QUIT, |_, _| Ret::default());
        let allocator = zeroed_allocator(&mut e);
        let (first, second) = (list_block(&mut e), list_block(&mut e));
        for (i, list) in [first, second, first, 0, second].into_iter().enumerate() {
            list_at(&mut e, allocator, i as u32, list);
        }
        e.mem.set_u32(allocator + 0x234, 5);
        e.call_log = Some(vec![]);
        e.call(0x00d5_ff50, &args![allocator]);
        assert_eq!(
            called(&mut e, FREE_LIST_CLEAR),
            vec![vec![first], vec![second]]
        );
        assert_eq!(
            called(&mut e, LARGE_BLOCK_QUIT),
            vec![vec![allocator + 0x28]]
        );
        let entries: Vec<u32> = (0..5)
            .map(|i| e.mem.u32(allocator + 0x190 + 4 * i))
            .collect();
        assert_eq!(entries, vec![first, second, 0, 0, 0]);
    }

    fn destructor_engine() -> Engine {
        let mut e = section_engine();
        e.register(FREE_LIST_CLEAR, |_, _| Ret::default());
        e.register(LARGE_BLOCK_QUIT, |_, _| Ret::default());
        e.register(LARGE_BLOCK_DESTRUCTOR, |_, _| Ret::default());
        e.register(DELETE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATOR_DESTRUCTOR, |_, _| Ret::default());
        e
    }

    #[test]
    fn the_destructor_clears_the_pool_backwards_and_runs_the_base_destructor() {
        let mut e = destructor_engine();
        let allocator = zeroed_allocator(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00d5_ffd0, &args![allocator]);
        assert_eq!(e.mem.u32(allocator), VTABLE);
        let cleared = called(&mut e, FREE_LIST_CLEAR);
        assert_eq!(cleared.len(), 41);
        assert_eq!(cleared[0], vec![allocator + 0xa60]);
        assert_eq!(cleared[40], vec![allocator + 0x240]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        let tail = &order[order.len() - 3..];
        assert_eq!(
            tail,
            [
                LARGE_BLOCK_DESTRUCTOR,
                DELETE_CRITICAL_SECTION,
                MEMORY_ALLOCATOR_DESTRUCTOR
            ]
        );
        assert_eq!(
            called(&mut e, DELETE_CRITICAL_SECTION),
            vec![vec![allocator + 4]]
        );
        assert_eq!(
            called(&mut e, LARGE_BLOCK_QUIT),
            vec![vec![allocator + 0x28]]
        );
    }

    #[test]
    fn the_scalar_deleting_destructor_destroys_and_answers_this() {
        let mut e = destructor_engine();
        let allocator = zeroed_allocator(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d6_03e0, &args![allocator, 1u32]).u32(),
            allocator
        );
        assert_eq!(
            called(&mut e, MEMORY_ALLOCATOR_DESTRUCTOR),
            vec![vec![allocator]]
        );
        assert_eq!(e.mem.u32(allocator), VTABLE);
    }

    #[test]
    fn walking_the_memory_skips_free_blocks_and_walks_every_list() {
        fn get_blocks(e: &mut Engine, a: &[u32]) -> Ret {
            // Words `base + 0x100 * i` from the base kept in the list.
            let base = e.mem.u32(a[0] + 0x34);
            for i in 0..a[2] {
                e.mem.set_u32(a[1] + 4 * i, base + 0x100 * i);
            }
            Ret::default()
        }
        fn sort_blocks(e: &mut Engine, a: &[u32]) -> Ret {
            let mut words: Vec<u32> = (0..=a[2]).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            words.sort_unstable();
            for (i, word) in words.into_iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * i as u32, word);
            }
            Ret::default()
        }
        let mut e = section_engine();
        e.register(FREE_LIST_GET_BLOCKS, get_blocks);
        e.register(SORT_BLOCKS, sort_blocks);
        e.register(FREE_LIST_WALK_MEMORY, |_, _| Ret::default());
        // Four large blocks: the payloads of blocks 1 and 2 are in the sorted array.
        let blocks: Vec<u32> = (0..4).map(|_| e.mem.alloc(0x20)).collect();
        for (i, block) in blocks.iter().enumerate() {
            e.mem.set_u32(block + 4, if i == 3 { 0x43 } else { 0x41 });
        }
        let blocks_for_first = blocks.clone();
        e.register_double(LARGE_BLOCK_FIRST_BLOCK, move |e, a| {
            e.mem.set_u32(a[1], blocks_for_first[0]);
            Ret::default()
        });
        let blocks_for_next = blocks.clone();
        let mut position = 0usize;
        e.register_double(LARGE_BLOCK_NEXT_BLOCK, move |e, a| {
            position += 1;
            let next = blocks_for_next.get(position).copied().unwrap_or(0);
            e.mem.set_u32(a[2], next);
            Ret::default()
        });
        let callback = 0x0900_a1c4u32;
        e.register_double(callback, |_, _| Ret::default());
        let allocator = zeroed_allocator(&mut e);
        let (first, second) = (list_block(&mut e), list_block(&mut e));
        // The first list gives b1+0x10, b1+0x110, b1+0x210; the second gives b2+0x10.
        e.mem.set_u32(first + 0x34, blocks[1] + 0x10);
        e.mem.set_u32(second + 0x34, blocks[2] + 0x10);
        let nodes: Vec<u32> = (0..4).map(|_| e.mem.alloc(8)).collect();
        e.mem.set_u32(nodes[0], nodes[1]);
        e.mem.set_u32(first + 0x10, nodes[0]);
        e.mem.set_u32(first + 0xc, nodes[2]);
        e.mem.set_u32(second + 0xc, nodes[3]);
        list_at(&mut e, allocator, 0, first);
        list_at(&mut e, allocator, 1, second);
        e.mem.set_u32(allocator + 0x234, 2);
        let array = e.mem.alloc(0x10);
        let parent = e.mem.alloc(8);
        e.mem.set_u32(parent, PARENT_VTABLE);
        e.mem.set_u32(allocator + 0x24, parent);
        vtable_at(
            &mut e,
            PARENT_VTABLE,
            &[(0x24, SLOT_TARGET_A), (0x28, SLOT_TARGET_B)],
        );
        e.register_double(SLOT_TARGET_A, move |_, a| {
            assert_eq!(a[1], 0x10);
            Ret {
                eax: array,
                ..Ret::default()
            }
        });
        e.register(SLOT_TARGET_B, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d6_00c0, &args![allocator, callback, 0x1234u32])
                .u32(),
            0
        );
        // 3 blocks in the chains of the first list, 1 in the second: 4 words.
        assert_eq!(
            called(&mut e, SORT_BLOCKS),
            vec![vec![array, 0, 3, COMPARE_BLOCKS]]
        );
        assert_eq!(
            called(&mut e, FREE_LIST_GET_BLOCKS),
            vec![vec![first, array, 3], vec![second, array + 12, 1]]
        );
        // The sorted words are b1+0x10, b2+0x10, b1+0x110, b1+0x210: the payloads of blocks 1
        // and 2 are the first two, so only blocks 0 and 3 reach the callback. Block 3 has
        // bit 1 of its size word set.
        let asked = called(&mut e, callback);
        assert_eq!(asked.len(), 2);
        assert_eq!(
            asked[0],
            vec![blocks[0] + 0x10, 0x40, 0x0900_a100, 0, 0x1234]
        );
        assert_eq!(
            asked[1],
            vec![blocks[3] + 0x10, 0x40, 0x0900_a101, 0, 0x1234]
        );
        assert_eq!(
            called(&mut e, FREE_LIST_WALK_MEMORY),
            vec![
                vec![first, callback, 1, 0x1234],
                vec![second, callback, 2, 0x1234]
            ]
        );
        assert_eq!(
            called(&mut e, SLOT_TARGET_B),
            vec![vec![parent, array, 0x10]]
        );
    }

    #[test]
    fn walking_the_memory_gives_up_when_the_parent_has_no_words() {
        let mut e = section_engine();
        let allocator = zeroed_allocator(&mut e);
        let parent = e.mem.alloc(8);
        e.mem.set_u32(parent, PARENT_VTABLE);
        e.mem.set_u32(allocator + 0x24, parent);
        vtable_at(&mut e, PARENT_VTABLE, &[(0x24, SLOT_TARGET_C)]);
        e.register(SLOT_TARGET_C, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d6_00c0, &args![allocator, 0x7000u32, 0u32])
                .u32(),
            1
        );
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }
}

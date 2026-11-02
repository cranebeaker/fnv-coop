//! `BSCore/abstractheap.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The heap keeps blocks with an 8-byte header: the previous block at +0, the size in the low
//! 28 bits of the word at +4 (the high 4 bits are flags; `0x20000000` marks memory that is
//! not committed), and, for a free block, its free-list links at +8 and +0xc (the next free
//! block at +0xc). The data starts at +8.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `AbstractHeap` (Xbox PDB), 0x2100 bytes; the PC code uses the same offsets.
    pub struct AbstractHeap: 0x2100 {
        /// `iDefaultAlignment` (Xbox PDB).
        0x0008 iDefaultAlignment: u32,
        /// `MinFreeBlockSize` (Xbox PDB).
        0x000C MinFreeBlockSize: u32,
        /// `iPageSize` (Xbox PDB): non-zero when the heap commits its memory page by page.
        0x0010 iPageSize: u32,
        /// `MemHeapSize` (Xbox PDB).
        0x0014 MemHeapSize: u32,
        /// `iCurrentSize` (Xbox PDB).
        0x001C iCurrentSize: u32,
        /// `iMemAllocated` (Xbox PDB).
        0x0024 iMemAllocated: u32,
        /// `iMemAllocatedHigh` (Xbox PDB).
        0x0028 iMemAllocatedHigh: u32,
        /// `iBlockMemAllocated` (Xbox PDB).
        0x002C iBlockMemAllocated: u32,
        /// `pMemHeap` (Xbox PDB).
        0x0030 pMemHeap: u32,
        /// `iNumBlocks` (Xbox PDB).
        0x0034 iNumBlocks: u32,
        /// `pBlockHead` (Xbox PDB).
        0x0038 pBlockHead: u32,
        /// `pBlockTail` (Xbox PDB).
        0x003C pBlockTail: u32,
        /// `pSmallFreeList` (Xbox PDB): 1024 8-byte free lists (first block, count).
        0x0044 pSmallFreeList: u32,
        /// `pLastSmallList` (Xbox PDB).
        0x2044 pLastSmallList: u32,
        /// `LargeFreeList` (Xbox PDB).
        0x2048 LargeFreeList: u32,
        /// `pSubBuckets` (Xbox PDB): 8-byte entries, a count at +0 and the first small list
        /// of the bucket at +4.
        0x2058 pSubBuckets: u32,
        /// `CriticalSectionO` (Xbox PDB).
        0x20D8 CriticalSectionO: u32,
    }
}

/// `EnterCriticalSection` import slot (stdcall: the section).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
/// `LeaveCriticalSection` import slot (stdcall: the section).
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `VirtualAlloc` import slot (stdcall: address, size, type, protection).
const VIRTUAL_ALLOC: u32 = 0x00fd_f114;
/// `MEM_COMMIT`.
const MEM_COMMIT: u32 = 0x1000;
/// `PAGE_READWRITE`.
const PAGE_READWRITE: u32 = 4;
/// Removes a block from a free list (`00aa9e50`, thiscall on the heap: the list, the block).
const FREE_LIST_REMOVE: u32 = 0x00aa_9e50;
/// `00aa9d30` (thiscall on the heap: a block); called on a block given back to the free lists.
const FREE_BLOCK_INSERT: u32 = 0x00aa_9d30;
/// `00aa9bd0` (thiscall on the heap: a block); called on the split-off remainder.
const BLOCK_MERGE: u32 = 0x00aa_9bd0;
/// The formatter `00406d00` (cdecl: buffer, size, format, ...).
const FORMAT: u32 = 0x0040_6d00;
/// The message handler pointer the commit messages go to (its virtual at +0x14 takes the
/// message type 2, the text and three zero words).
const MESSAGE_HANDLER: u32 = 0x011f_6388;
/// `"Committing Entire Free Block: %08X, %08X\r\n"`.
const COMMIT_ENTIRE_FREE_BLOCK: u32 = 0x010a_29e4;
/// `"Committing perfectly matched Free Block: %08X, %08X\r\n"`.
const COMMIT_MATCHED_FREE_BLOCK: u32 = 0x010a_2a10;
/// `"Committing memory on end of heap: %08X, %08X\r\n"`.
const COMMIT_END_OF_HEAP: u32 = 0x010a_2a48;
/// The size bits of a block's word at +4.
const SIZE_MASK: u32 = 0x0fff_ffff;
/// The flag bits of a block's word at +4.
const FLAG_MASK: u32 = 0xf000_0000;
/// The not-committed flag.
const NOT_COMMITTED: u32 = 0x2000_0000;

/// Formats a commit message into a 0x100-byte local and gives it to the message handler
/// when there is one.
fn report_commit(e: &mut Engine, format: u32, address: u32, size: u32) {
    e.with_stack(0x100, |e, text| {
        e.call(FORMAT, &args![text, 0x100u32, format, address, size]);
        let handler = e.global::<u32>(MESSAGE_HANDLER);
        if handler != 0 {
            e.vcall(handler, 0x14, &args![2u32, text, 0u32, 0u32, 0u32]);
        }
    });
}

/// `VirtualAlloc(address, size, MEM_COMMIT, PAGE_READWRITE) != 0`.
fn commit(e: &mut Engine, address: u32, size: u32) -> bool {
    e.call(
        VIRTUAL_ALLOC,
        &args![address, size, MEM_COMMIT, PAGE_READWRITE],
    )
    .u32()
        != 0
}

/// After a commit: the compiler folded the flag's new value to false (`XOR ECX,ECX; JZ`), so
/// the not-committed bit is always cleared; the branch that would set it never runs
/// (translated anyway).
fn mark_committed(e: &mut Engine, block: u32) {
    let folded = std::hint::black_box(false);
    let flags = e.mem.u32(block + 4);
    if folded {
        e.mem.set_u32(block + 4, flags | NOT_COMMITTED);
    } else {
        e.mem.set_u32(block + 4, flags & !NOT_COMMITTED);
    }
}

// Translated from 00aaa6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A block header's set-up (thiscall, `RET 8`): the word at +4 cleared and then set to
/// `(word & 0xf0000000) | size` (so just `size`), the previous-block word at +0 cleared. The
/// second word is not read (the code stores 0 into its stack slot).
pub fn fn_00aaa6e0(e: &mut Engine, this: Ptr, size: u32, _unused_2: u32) {
    let block = this.addr();
    e.mem.set_u32(block + 4, 0);
    let word = e.mem.u32(block + 4);
    e.mem.set_u32(block + 4, (word & FLAG_MASK) | size);
    e.mem.set_u32(block, 0);
}

// Translated from 00aa9ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends `block` to the heap's block chain: with a tail, the block's previous-block word
/// (+0) is the tail and the block becomes the tail; with none, it becomes head and tail.
/// Counts one more block (`iNumBlocks`).
pub fn fn_00aa9ca0(e: &mut Engine, this: Ptr<AbstractHeap>, block: Ptr) {
    let tail = e.get(this, AbstractHeap::pBlockTail);
    if tail != 0 {
        e.mem.set_u32(block.addr(), tail);
        e.set(this, AbstractHeap::pBlockTail, block.addr());
    } else {
        e.set(this, AbstractHeap::pBlockHead, block.addr());
        let head = e.get(this, AbstractHeap::pBlockHead);
        e.set(this, AbstractHeap::pBlockTail, head);
    }
    let count = e.get(this, AbstractHeap::iNumBlocks);
    e.set(this, AbstractHeap::iNumBlocks, count.wrapping_add(1));
}

/// The free list `BaseAllocate` takes blocks from: the small list for the size, or, when that
/// one is empty, a later list found through the sub-buckets, or the large list.
fn find_free_list(e: &mut Engine, this: Ptr<AbstractHeap>, size: u32) -> u32 {
    let heap = this.addr();
    let large = heap + 0x2048;
    let alignment = e.get(this, AbstractHeap::iDefaultAlignment);
    let index = (size / alignment).wrapping_sub(1);
    let first = if (index as i32) < 0x400 {
        heap.wrapping_add(index.wrapping_mul(8)).wrapping_add(0x44)
    } else {
        large
    };
    let mut list = first;
    if e.mem.u32(list + 4) != 0 || list == large {
        return list;
    }
    let minimum = e.get(this, AbstractHeap::MinFreeBlockSize);
    list = list.wrapping_add((minimum / alignment).wrapping_mul(8));
    if list > e.get(this, AbstractHeap::pLastSmallList) {
        return large;
    }
    let padded = size.wrapping_add(minimum) as i32;
    let start = if padded > 0x1000 {
        -1
    } else {
        // `IDIV`, then the signed division by 64 (`CDQ; AND EDX,0x3f; ADD; SAR 6`).
        let lists = padded.wrapping_div(alignment as i32).wrapping_sub(1);
        lists / 64
    };
    let mut bucket = start;
    while bucket < 0x10 && e.mem.u32(list + 4) == 0 {
        let entry = heap.wrapping_add((bucket as u32).wrapping_mul(8));
        if e.mem.u32(entry.wrapping_add(0x2058)) != 0 {
            let bucket_first = e.mem.u32(entry.wrapping_add(0x205c));
            if bucket_first > list {
                list = bucket_first;
            }
            while e.mem.u32(list + 4) == 0 {
                // The bucket's end is the next bucket's first list (for the last bucket that
                // word lies in the critical section after the array, as in the exe).
                let end = if bucket < 0x10 {
                    e.mem.u32(entry.wrapping_add(0x2064))
                } else {
                    e.get(this, AbstractHeap::pLastSmallList)
                };
                if list >= end {
                    break;
                }
                list += 8;
            }
        }
        bucket += 1;
    }
    if bucket == 0x10 {
        list = large;
    }
    list
}

// Translated from 00aa8f50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aa8f50 - the /GS stack-cookie check (00ec408c) is not translated
/// `AbstractHeap::BaseAllocate` (Xbox PDB): inside the heap's critical section, rounds `size`
/// up to the alignment (8 for sizes up to 8) and looks for a free block (see
/// `find_free_list`): the first block of that list whose size is exactly `size` or at least
/// `MinFreeBlockSize` larger. That block leaves its list (`00aa9e50`); a larger one is split:
/// its first `size + 0x18` bytes are committed when it was not committed (`VirtualAlloc`,
/// with a commit message), the remainder gets a header (`00aaa6e0`) linked after it (or as
/// the chain's tail), counts as a block and goes back to the free lists (`00aa9d30`,
/// `00aa9bd0`); an exact match is committed (`size + 8` bytes) when needed. The block's size
/// bits become `size`, `iBlockMemAllocated` grows by its size + 8, and its data (+8) is the
/// result. A failed commit gives the block back (`00aa9d30`) and returns 0.
///
/// Without a free block, it takes `size + 8` bytes from the end of the heap: when the heap
/// commits by page and the end passes `iCurrentSize` but not `MemHeapSize`, the heap's
/// virtual at +0x38 grows it (`pMemHeap`, `iCurrentSize`, the size) by the amount it returns;
/// if the end then fits, the block there is committed when paging (0 on failure), gets its
/// header (`00aaa6e0`), joins the chain (`00aa9ca0`), and `iMemAllocated` (and its high mark)
/// and `iBlockMemAllocated` grow. Returns the data address, or 0.
pub fn abstract_heap_base_allocate(e: &mut Engine, this: Ptr<AbstractHeap>, size: u32) -> u32 {
    let heap = this.addr();
    let section = heap + 0x20d8;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let size = if size <= 8 {
        8
    } else {
        let alignment = e.get(this, AbstractHeap::iDefaultAlignment);
        size.wrapping_add(alignment).wrapping_sub(1) & !alignment.wrapping_sub(1)
    };
    let mut result = 0u32;
    let list = find_free_list(e, this, size);
    if e.mem.u32(list + 4) != 0 {
        let mut block = e.mem.u32(list);
        while block != 0 {
            let block_size = e.mem.u32(block + 4) & SIZE_MASK;
            let spare = block_size.wrapping_sub(size) as i32;
            let minimum = e.get(this, AbstractHeap::MinFreeBlockSize) as i32;
            if spare != 0 && spare < minimum {
                block = e.mem.u32(block + 0xc);
                continue;
            }
            let not_committed = e.mem.u32(block + 4) & NOT_COMMITTED != 0;
            e.call(FREE_LIST_REMOVE, &args![heap, list, block]);
            if spare != 0 {
                if not_committed {
                    let length = size.wrapping_add(0x18);
                    if !commit(e, block, length) {
                        e.call(FREE_BLOCK_INSERT, &args![heap, block]);
                        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
                        return 0;
                    }
                    mark_committed(e, block);
                    report_commit(e, COMMIT_ENTIRE_FREE_BLOCK, block, length);
                }
                let rest = block.wrapping_add(size).wrapping_add(8);
                let rest_size = block_size.wrapping_sub(size.wrapping_add(8));
                e.mem.set_u32(rest + 8, 0);
                e.mem.set_u32(rest + 0xc, 0);
                fn_00aaa6e0(e, Ptr::new(rest), rest_size, 0);
                e.mem.set_u32(rest, block);
                if block == e.get(this, AbstractHeap::pBlockTail) {
                    e.set(this, AbstractHeap::pBlockTail, rest);
                } else {
                    let current = e.mem.u32(block + 4) & SIZE_MASK;
                    let next = block.wrapping_add(current).wrapping_add(8);
                    e.mem.set_u32(next, rest);
                }
                let count = e.get(this, AbstractHeap::iNumBlocks);
                e.set(this, AbstractHeap::iNumBlocks, count.wrapping_add(1));
                e.call(FREE_BLOCK_INSERT, &args![heap, rest]);
                e.call(BLOCK_MERGE, &args![heap, rest]);
            } else if not_committed {
                let length = size.wrapping_add(8);
                if !commit(e, block, length) {
                    e.call(FREE_BLOCK_INSERT, &args![heap, block]);
                    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
                    return 0;
                }
                mark_committed(e, block);
                report_commit(e, COMMIT_MATCHED_FREE_BLOCK, block, length);
            }
            let word = e.mem.u32(block + 4);
            e.mem.set_u32(block + 4, (word & FLAG_MASK) | size);
            result = block.wrapping_add(8);
            let used = (e.mem.u32(block + 4) & SIZE_MASK).wrapping_add(8);
            let total = e.get(this, AbstractHeap::iBlockMemAllocated);
            e.set(
                this,
                AbstractHeap::iBlockMemAllocated,
                used.wrapping_add(total),
            );
            break;
        }
    }
    if result == 0 {
        let needed = size.wrapping_add(8);
        let end = e
            .get(this, AbstractHeap::iMemAllocated)
            .wrapping_add(needed);
        if e.get(this, AbstractHeap::iPageSize) != 0
            && end > e.get(this, AbstractHeap::iCurrentSize)
            && end <= e.get(this, AbstractHeap::MemHeapSize)
        {
            let memory = e.get(this, AbstractHeap::pMemHeap);
            let current = e.get(this, AbstractHeap::iCurrentSize);
            let grown = e.vcall(heap, 0x38, &args![memory, current, needed]).u32();
            let current = e.get(this, AbstractHeap::iCurrentSize);
            e.set(
                this,
                AbstractHeap::iCurrentSize,
                grown.wrapping_add(current),
            );
        }
        if end <= e.get(this, AbstractHeap::iCurrentSize) {
            let block = e
                .get(this, AbstractHeap::pMemHeap)
                .wrapping_add(e.get(this, AbstractHeap::iMemAllocated));
            if e.get(this, AbstractHeap::iPageSize) != 0 {
                let length = size.wrapping_add(8);
                if !commit(e, block, length) {
                    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
                    return 0;
                }
                report_commit(e, COMMIT_END_OF_HEAP, block, length);
            }
            fn_00aaa6e0(e, Ptr::new(block), size, 0);
            fn_00aa9ca0(e, this, Ptr::new(block));
            let allocated = e
                .get(this, AbstractHeap::iMemAllocated)
                .wrapping_add(needed);
            e.set(this, AbstractHeap::iMemAllocated, allocated);
            if (allocated as i32) > (e.get(this, AbstractHeap::iMemAllocatedHigh) as i32) {
                e.set(this, AbstractHeap::iMemAllocatedHigh, allocated);
            }
            result = block.wrapping_add(8);
            let total = e.get(this, AbstractHeap::iBlockMemAllocated);
            e.set(
                this,
                AbstractHeap::iBlockMemAllocated,
                total.wrapping_add(needed),
            );
        }
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_a6e0, fn_00aaa6e0(Ptr, u32, u32)),
        entry!(0x00aa_9ca0, fn_00aa9ca0(Ptr<AbstractHeap>, Ptr)),
        entry!(
            0x00aa_8f50,
            abstract_heap_base_allocate(Ptr<AbstractHeap>, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn log_of(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A heap with alignment 8, minimum free block 0x20, and the platform calls doubled.
    fn heap(e: &mut Engine, page_size: u32) -> u32 {
        let heap = e.mem.alloc(0x2100);
        e.mem.set_u32(heap + 8, 8);
        e.mem.set_u32(heap + 0xc, 0x20);
        e.mem.set_u32(heap + 0x10, page_size);
        e.mem.set_u32(heap + 0x2044, heap + 0x44 + 8 * 0x3ff);
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(FREE_LIST_REMOVE, |_, _| Ret::default());
        e.register(FREE_BLOCK_INSERT, |_, _| Ret::default());
        e.register(BLOCK_MERGE, |_, _| Ret::default());
        e.register(FORMAT, |_, _| Ret::default());
        e.map(MESSAGE_HANDLER, 4);
        heap
    }

    #[test]
    fn header_set_up_and_chain_append() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, 0x1111);
        e.mem.set_u32(block + 4, 0xffff_ffff);
        e.call(0x00aa_a6e0, &args![block, 0x40u32, 7u32]);
        assert_eq!((e.mem.u32(block), e.mem.u32(block + 4)), (0, 0x40));
        let heap = e.mem.alloc(0x2100);
        let second = e.mem.alloc(0x10);
        e.call(0x00aa_9ca0, &args![heap, block]);
        assert_eq!(
            (e.mem.u32(heap + 0x38), e.mem.u32(heap + 0x3c)),
            (block, block)
        );
        e.call(0x00aa_9ca0, &args![heap, second]);
        assert_eq!(e.mem.u32(second), block);
        assert_eq!(
            (e.mem.u32(heap + 0x38), e.mem.u32(heap + 0x3c)),
            (block, second)
        );
        assert_eq!(e.mem.u32(heap + 0x34), 2);
    }

    #[test]
    fn allocates_from_the_end_of_the_heap() {
        let mut e = Engine::new();
        let heap = heap(&mut e, 0);
        let memory = e.mem.alloc(0x1000);
        e.mem.set_u32(heap + 0x30, memory);
        e.mem.set_u32(heap + 0x1c, 0x100);
        e.mem.set_u32(heap + 0x24, 0x10);
        e.call_log = Some(vec![]);
        // 0x13 rounds up to 0x18; the block takes 0x20 bytes at +0x10.
        let r = e.call(0x00aa_8f50, &args![heap, 0x13u32]).u32();
        assert_eq!(r, memory + 0x18);
        assert_eq!(e.mem.u32(memory + 0x14), 0x18);
        assert_eq!(e.mem.u32(heap + 0x24), 0x30);
        assert_eq!(e.mem.u32(heap + 0x28), 0x30);
        assert_eq!(e.mem.u32(heap + 0x2c), 0x20);
        assert_eq!(e.mem.u32(heap + 0x3c), memory + 0x10);
        let log = log_of(&mut e, 0x00aa_8f50);
        assert_eq!(
            calls(&log, ENTER_CRITICAL_SECTION),
            vec![vec![heap + 0x20d8]]
        );
        assert_eq!(
            calls(&log, LEAVE_CRITICAL_SECTION),
            vec![vec![heap + 0x20d8]]
        );
        // Too big for what is left: 0.
        e.mem.set_u32(heap + 0x24, 0xf0);
        assert_eq!(e.call(0x00aa_8f50, &args![heap, 0x10u32]).u32(), 0);
    }

    #[test]
    fn grows_and_commits_the_end_of_a_paged_heap() {
        for ok in [1u32, 0] {
            let mut e = Engine::new();
            let heap = heap(&mut e, 0x1000);
            let memory = e.mem.alloc(0x1000);
            e.mem.set_u32(heap + 0x30, memory);
            e.mem.set_u32(heap + 0x14, 0x1000);
            e.mem.set_u32(heap + 0x1c, 0x10);
            let vtable = e.mem.alloc(0x40);
            e.mem.set_u32(vtable + 0x38, 0x00aa_0006);
            e.mem.set_u32(heap, vtable);
            e.register(0x00aa_0006, |_, _| 0x100u32.into_ret());
            e.register_double(VIRTUAL_ALLOC, move |_, _| ok.into_ret());
            let handler_vtable = e.mem.alloc(0x20);
            e.mem.set_u32(handler_vtable + 0x14, 0x00aa_0007);
            let handler = e.mem.alloc(4);
            e.mem.set_u32(handler, handler_vtable);
            e.mem.set_u32(MESSAGE_HANDLER, handler);
            e.register(0x00aa_0007, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            let r = e.call(0x00aa_8f50, &args![heap, 0x20u32]).u32();
            let log = log_of(&mut e, 0x00aa_8f50);
            assert_eq!(
                calls(&log, 0x00aa_0006),
                vec![vec![heap, memory, 0x10, 0x28]]
            );
            assert_eq!(e.mem.u32(heap + 0x1c), 0x110);
            assert_eq!(
                calls(&log, VIRTUAL_ALLOC),
                vec![vec![memory, 0x28, MEM_COMMIT, PAGE_READWRITE]]
            );
            assert_eq!(calls(&log, LEAVE_CRITICAL_SECTION).len(), 1);
            if ok == 0 {
                assert_eq!(r, 0);
                assert!(calls(&log, FORMAT).is_empty());
                continue;
            }
            assert_eq!(r, memory + 8);
            let format = calls(&log, FORMAT);
            assert_eq!(format[0][1..], [0x100, COMMIT_END_OF_HEAP, memory, 0x28]);
            assert_eq!(
                calls(&log, 0x00aa_0007),
                vec![vec![handler, 2, format[0][0], 0, 0, 0]]
            );
        }
    }

    /// A free block of `block_size` in the small list for `list_size`.
    fn free_block(
        e: &mut Engine,
        heap: u32,
        list_size: u32,
        block_size: u32,
        flags: u32,
    ) -> (u32, u32) {
        let list = heap + 0x44 + 8 * (list_size / 8 - 1);
        let block = e.mem.alloc(block_size + 0x40);
        e.mem.set_u32(block + 4, flags | block_size);
        e.mem.set_u32(list, block);
        e.mem.set_u32(list + 4, 1);
        (list, block)
    }

    #[test]
    fn takes_an_exactly_matching_free_block() {
        let mut e = Engine::new();
        let heap = heap(&mut e, 0);
        let (list, block) = free_block(&mut e, heap, 0x40, 0x40, 0x4000_0000);
        e.call_log = Some(vec![]);
        let r = e.call(0x00aa_8f50, &args![heap, 0x40u32]).u32();
        assert_eq!(r, block + 8);
        assert_eq!(e.mem.u32(block + 4), 0x4000_0040);
        assert_eq!(e.mem.u32(heap + 0x2c), 0x48);
        let log = log_of(&mut e, 0x00aa_8f50);
        assert_eq!(calls(&log, FREE_LIST_REMOVE), vec![vec![heap, list, block]]);
        assert!(calls(&log, BLOCK_MERGE).is_empty());
    }

    #[test]
    fn commits_a_matching_block_that_was_not_committed() {
        for ok in [1u32, 0] {
            let mut e = Engine::new();
            let heap = heap(&mut e, 0);
            let (_, block) = free_block(&mut e, heap, 0x40, 0x40, NOT_COMMITTED);
            e.register_double(VIRTUAL_ALLOC, move |_, _| ok.into_ret());
            e.call_log = Some(vec![]);
            let r = e.call(0x00aa_8f50, &args![heap, 0x40u32]).u32();
            let log = log_of(&mut e, 0x00aa_8f50);
            assert_eq!(
                calls(&log, VIRTUAL_ALLOC),
                vec![vec![block, 0x48, MEM_COMMIT, PAGE_READWRITE]]
            );
            if ok == 0 {
                assert_eq!(r, 0);
                assert_eq!(calls(&log, FREE_BLOCK_INSERT), vec![vec![heap, block]]);
                continue;
            }
            assert_eq!(r, block + 8);
            assert_eq!(e.mem.u32(block + 4), 0x40);
            assert_eq!(calls(&log, FORMAT)[0][2], COMMIT_MATCHED_FREE_BLOCK);
        }
    }

    #[test]
    fn splits_a_larger_free_block() {
        let mut e = Engine::new();
        let heap = heap(&mut e, 0);
        // The list for 0x40 is empty: the next list with blocks is the one for 0x40 + 0x20
        // found through the first sub-bucket.
        let (list, block) = free_block(&mut e, heap, 0x80, 0x80, NOT_COMMITTED);
        e.mem.set_u32(heap + 0x2058, 1);
        e.mem.set_u32(heap + 0x205c, heap + 0x44);
        e.mem.set_u32(heap + 0x2064, heap + 0x44 + 8 * 0x40);
        e.mem.set_u32(heap + 0x3c, block);
        e.register(VIRTUAL_ALLOC, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let r = e.call(0x00aa_8f50, &args![heap, 0x40u32]).u32();
        assert_eq!(r, block + 8);
        let rest = block + 0x48;
        assert_eq!(e.mem.u32(rest), block);
        assert_eq!(e.mem.u32(rest + 4), 0x80 - 0x48);
        assert_eq!(e.mem.u32(heap + 0x3c), rest);
        assert_eq!(e.mem.u32(heap + 0x34), 1);
        assert_eq!(e.mem.u32(block + 4), 0x40);
        let log = log_of(&mut e, 0x00aa_8f50);
        assert_eq!(calls(&log, FREE_LIST_REMOVE), vec![vec![heap, list, block]]);
        assert_eq!(
            calls(&log, VIRTUAL_ALLOC),
            vec![vec![block, 0x58, MEM_COMMIT, PAGE_READWRITE]]
        );
        assert_eq!(calls(&log, FORMAT)[0][2], COMMIT_ENTIRE_FREE_BLOCK);
        assert_eq!(calls(&log, FREE_BLOCK_INSERT), vec![vec![heap, rest]]);
        assert_eq!(calls(&log, BLOCK_MERGE), vec![vec![heap, rest]]);
    }

    #[test]
    fn split_links_the_following_block_and_skips_too_small_remainders() {
        let mut e = Engine::new();
        let heap = heap(&mut e, 0);
        // In the large list: a block 8 bytes too big (skipped), then one 0x60 bytes too big.
        let large = heap + 0x2048;
        let small = e.mem.alloc(0x200);
        e.mem.set_u32(small + 4, 0x2008);
        let big = e.mem.alloc(0x2100);
        e.mem.set_u32(big + 4, 0x2060);
        e.mem.set_u32(small + 0xc, big);
        e.mem.set_u32(large, small);
        e.mem.set_u32(large + 4, 2);
        let following = big + 0x2068;
        let r = e.call(0x00aa_8f50, &args![heap, 0x2000u32]).u32();
        assert_eq!(r, big + 8);
        let rest = big + 0x2008;
        assert_eq!(e.mem.u32(rest + 4), 0x58);
        // `big` is not the tail: the block after it (at its old end) points back at the rest.
        assert_eq!(e.mem.u32(following), rest);
        assert_eq!(e.mem.u32(small + 4), 0x2008);
    }
}

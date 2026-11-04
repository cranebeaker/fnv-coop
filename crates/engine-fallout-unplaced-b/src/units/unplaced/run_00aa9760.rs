//! `(unplaced)/run_00aa9760` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The ones translated here free and merge the blocks of an `AbstractHeap`
//! (`BSCore/abstractheap.cpp`; read at its offsets, the layout is
//! `crate::units::bscore::abstractheap::AbstractHeap`). A block has an 8-byte header: the
//! previous block at +0 and the size in the low 28 bits of the word at +4; of the high bits,
//! 0x40000000 marks a free block and 0x20000000 memory that is not committed. The data starts
//! at +8, so the block after one is at `block + size + 8`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `AbstractHeap` fields (Xbox PDB names, PC offsets): `iPageSize` +0x10, `iInitialSize`
/// +0x18, `iCurrentSize` +0x1c, `iMemAllocated` +0x24, `iBlockMemAllocated` +0x2c, `pMemHeap`
/// +0x30, `iNumBlocks` +0x34, `pBlockHead` +0x38, `pBlockTail` +0x3c, `CriticalSectionO`
/// +0x20d8.
const HEAP_PAGE_SIZE: u32 = 0x10;
const HEAP_INITIAL_SIZE: u32 = 0x18;
const HEAP_CURRENT_SIZE: u32 = 0x1c;
const HEAP_MEM_ALLOCATED: u32 = 0x24;
const HEAP_BLOCK_MEM_ALLOCATED: u32 = 0x2c;
const HEAP_MEM: u32 = 0x30;
const HEAP_NUM_BLOCKS: u32 = 0x34;
const HEAP_BLOCK_HEAD: u32 = 0x38;
const HEAP_BLOCK_TAIL: u32 = 0x3c;
const HEAP_CRITICAL_SECTION: u32 = 0x20d8;
/// The heap's virtual at +0x3c (`__thiscall`, three words: the heap memory, its current size
/// and the bytes to give back); it answers the bytes given back.
const HEAP_DECOMMIT_SLOT: u32 = 0x3c;

/// A block's header: the previous block at +0 (read as the block's own address), size and
/// flags at +4.
const BLOCK_SIZE_FLAGS: u32 = 0x4;
const BLOCK_SIZE_MASK: u32 = 0x0fff_ffff;
const BLOCK_FLAGS_MASK: u32 = 0xf000_0000;
const BLOCK_FREE: u32 = 0x4000_0000;
const BLOCK_NOT_COMMITTED: u32 = 0x2000_0000;

// Callees outside this unit.
/// `EnterCriticalSection` and `LeaveCriticalSection` import slots (stdcall: the section).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `00aa9d30` (`__thiscall` on the heap, one word: a block): puts the block on its free list.
const FREE_LIST_ADD: u32 = 0x00aa_9d30;
/// `00aaa660` (`__thiscall` on the heap, one word: a block): takes it off its free list.
const FREE_LIST_REMOVE: u32 = 0x00aa_a660;

/// The size in a block's header.
fn block_size(e: &Engine, block: u32) -> u32 {
    e.mem.u32(block + BLOCK_SIZE_FLAGS) & BLOCK_SIZE_MASK
}

/// Whether a block's header marks it free.
fn is_free(e: &Engine, block: u32) -> bool {
    e.mem.u32(block + BLOCK_SIZE_FLAGS) & BLOCK_FREE != 0
}

// Translated from 00aa98f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, one word: the data pointer): frees an
/// allocation. Nothing for a null pointer; otherwise, inside the heap's critical section, the
/// block (8 bytes before the data) leaves `iBlockMemAllocated` (its size and header), goes on
/// its free list (`00aa9d30`) and is merged with its free neighbours ([`fn_00aa9bd0`]).
pub fn fn_00aa98f0(e: &mut Engine, this: Ptr, data: Ptr) {
    if data.is_null() {
        return;
    }
    let heap = this.addr();
    e.call(ENTER_CRITICAL_SECTION, &args![heap + HEAP_CRITICAL_SECTION]);
    let block = data.addr().wrapping_sub(8);
    let size = block_size(e, block);
    let allocated = e
        .mem
        .u32(heap + HEAP_BLOCK_MEM_ALLOCATED)
        .wrapping_sub(size.wrapping_add(8));
    e.mem.set_u32(heap + HEAP_BLOCK_MEM_ALLOCATED, allocated);
    e.call(FREE_LIST_ADD, &args![this, block]);
    fn_00aa9bd0(e, this, Ptr::new(block));
    e.call(LEAVE_CRITICAL_SECTION, &args![heap + HEAP_CRITICAL_SECTION]);
}

// Translated from 00aa9970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, two words: a block and the block after it):
/// joins two free blocks into the first. Both leave their free lists (`00aaa660`); the first
/// takes the second's "not committed" mark when it has it, and the second's size plus its
/// header; the block after the second points back at the first (or the first becomes the
/// heap's last block); one block fewer, and the joined block goes back on a free list
/// (`00aa9d30`).
pub fn fn_00aa9970(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    let heap = this.addr();
    let (first, second) = (first.addr(), second.addr());
    e.call(FREE_LIST_REMOVE, &args![this, first]);
    e.call(FREE_LIST_REMOVE, &args![this, second]);
    if e.mem.u32(second + BLOCK_SIZE_FLAGS) & BLOCK_NOT_COMMITTED != 0 {
        // The exe tests a constant 1 here (an inlined flag setter given `true`), so only the
        // setting branch runs; the clearing one is kept as the exe has it.
        let mark = true;
        let flags = e.mem.u32(first + BLOCK_SIZE_FLAGS);
        if mark {
            e.mem
                .set_u32(first + BLOCK_SIZE_FLAGS, flags | BLOCK_NOT_COMMITTED);
        } else {
            e.mem
                .set_u32(first + BLOCK_SIZE_FLAGS, flags & !BLOCK_NOT_COMMITTED);
        }
    }
    let first_size = block_size(e, first);
    let second_size = block_size(e, second);
    let flags = e.mem.u32(first + BLOCK_SIZE_FLAGS) & BLOCK_FLAGS_MASK;
    let joined = first_size.wrapping_add(second_size).wrapping_add(8) | flags;
    e.mem.set_u32(first + BLOCK_SIZE_FLAGS, joined);
    if second == e.mem.u32(heap + HEAP_BLOCK_TAIL) {
        e.mem.set_u32(heap + HEAP_BLOCK_TAIL, first);
    } else {
        let size = block_size(e, second);
        let after = second.wrapping_add(size);
        // The block after the second: its previous-block word (+0) is at `second + size + 8`.
        e.mem.set_u32(after.wrapping_add(8), first);
    }
    let blocks = e.mem.u32(heap + HEAP_NUM_BLOCKS).wrapping_sub(1);
    e.mem.set_u32(heap + HEAP_NUM_BLOCKS, blocks);
    e.call(FREE_LIST_ADD, &args![this, first]);
}

// Translated from 00aa9a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, no words): gives back the free end of the
/// heap. While the last block is free and ends exactly at `pMemHeap + iMemAllocated`, it
/// leaves its free list (`00aaa660`), `iMemAllocated` and `iNumBlocks`, and the block before
/// it becomes the last (both ends are cleared when it was the only block). Then, for a heap
/// that commits page by page (`iPageSize` non-zero): the unused size `iCurrentSize -
/// iMemAllocated`, kept so that the size does not fall below `iInitialSize`, is given back
/// through the heap's virtual at +0x3c when it is at least a page, and `iCurrentSize` goes
/// down by what that answers.
pub fn fn_00aa9a70(e: &mut Engine, this: Ptr) {
    let heap = this.addr();
    let mut block = e.mem.u32(heap + HEAP_BLOCK_TAIL);
    while block != 0 && is_free(e, block) {
        let size = block_size(e, block);
        let end = e
            .mem
            .u32(heap + HEAP_MEM)
            .wrapping_add(e.mem.u32(heap + HEAP_MEM_ALLOCATED));
        if block.wrapping_add(size).wrapping_add(8) != end {
            break;
        }
        e.call(FREE_LIST_REMOVE, &args![this, block]);
        let size = block_size(e, block);
        let allocated = e
            .mem
            .u32(heap + HEAP_MEM_ALLOCATED)
            .wrapping_sub(size.wrapping_add(8));
        e.mem.set_u32(heap + HEAP_MEM_ALLOCATED, allocated);
        let blocks = e.mem.u32(heap + HEAP_NUM_BLOCKS).wrapping_sub(1);
        e.mem.set_u32(heap + HEAP_NUM_BLOCKS, blocks);
        if e.mem.u32(heap + HEAP_BLOCK_HEAD) == e.mem.u32(heap + HEAP_BLOCK_TAIL) {
            e.mem.set_u32(heap + HEAP_BLOCK_TAIL, 0);
            e.mem.set_u32(heap + HEAP_BLOCK_HEAD, 0);
            break;
        }
        let previous = e.mem.u32(block);
        e.mem.set_u32(heap + HEAP_BLOCK_TAIL, previous);
        block = e.mem.u32(heap + HEAP_BLOCK_TAIL);
    }
    if e.mem.u32(heap + HEAP_PAGE_SIZE) == 0 {
        return;
    }
    let current = e.mem.u32(heap + HEAP_CURRENT_SIZE);
    let mut unused = current.wrapping_sub(e.mem.u32(heap + HEAP_MEM_ALLOCATED));
    if current.wrapping_sub(unused) < e.mem.u32(heap + HEAP_INITIAL_SIZE) {
        unused = current.wrapping_sub(e.mem.u32(heap + HEAP_INITIAL_SIZE));
    }
    if unused < e.mem.u32(heap + HEAP_PAGE_SIZE) {
        return;
    }
    let memory = e.mem.u32(heap + HEAP_MEM);
    let given_back = e
        .vcall(heap, HEAP_DECOMMIT_SLOT, &args![memory, current, unused])
        .u32();
    let current = e.mem.u32(heap + HEAP_CURRENT_SIZE).wrapping_sub(given_back);
    e.mem.set_u32(heap + HEAP_CURRENT_SIZE, current);
}

// Translated from 00aa9bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, one word: a free block): merges a block with
/// its free neighbours. While the block before it is free, the two are joined into that one
/// ([`fn_00aa9970`]); then, until the block is the heap's last, while the block after it is
/// free, that one is joined in. Finally the free end of the heap is given back
/// ([`fn_00aa9a70`]).
pub fn fn_00aa9bd0(e: &mut Engine, this: Ptr, block: Ptr) {
    let heap = this.addr();
    let mut block = block.addr();
    if block != 0 {
        loop {
            let previous = e.mem.u32(block);
            if previous == 0 || !is_free(e, previous) {
                break;
            }
            fn_00aa9970(e, this, Ptr::new(previous), Ptr::new(block));
            block = previous;
        }
    }
    if block != 0 {
        while block != e.mem.u32(heap + HEAP_BLOCK_TAIL) {
            let size = block_size(e, block);
            // The next block's header word at +4 is at `block + size + 0xc`.
            if e.mem.u32(block.wrapping_add(size).wrapping_add(0xc)) & BLOCK_FREE == 0 {
                break;
            }
            let size = block_size(e, block);
            let next = block.wrapping_add(size).wrapping_add(8);
            fn_00aa9970(e, this, Ptr::new(block), Ptr::new(next));
        }
    }
    fn_00aa9a70(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa98f0, fn_00aa98f0(Ptr, Ptr)),
        entry!(0x00aa9970, fn_00aa9970(Ptr, Ptr, Ptr)),
        entry!(0x00aa9a70, fn_00aa9a70(Ptr)),
        entry!(0x00aa9bd0, fn_00aa9bd0(Ptr, Ptr)),
        entry!(0x00aa_9760, fn_00aa9760(u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
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

    /// Doubles for the free lists (they only record) and the critical section.
    fn doubles(e: &mut Engine) {
        for addr in [
            FREE_LIST_ADD,
            FREE_LIST_REMOVE,
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// A heap whose memory holds blocks of the given sizes and flags back to back; answers
    /// the heap and the blocks. `iMemAllocated` ends after the last block.
    fn heap(e: &mut Engine, blocks: &[(u32, u32)]) -> (u32, Vec<u32>) {
        let heap = e.mem.alloc(0x2100);
        let memory = e.mem.alloc(0x400);
        let mut at = memory;
        let mut previous = 0;
        let mut out = vec![];
        for &(size, flags) in blocks {
            e.mem.set_u32(at, previous);
            e.mem.set_u32(at + BLOCK_SIZE_FLAGS, size | flags);
            out.push(at);
            previous = at;
            at += size + 8;
        }
        e.mem.set_u32(heap + HEAP_MEM, memory);
        e.mem.set_u32(heap + HEAP_MEM_ALLOCATED, at - memory);
        e.mem.set_u32(heap + HEAP_NUM_BLOCKS, blocks.len() as u32);
        e.mem.set_u32(heap + HEAP_BLOCK_HEAD, out[0]);
        e.mem.set_u32(heap + HEAP_BLOCK_TAIL, *out.last().unwrap());
        (heap, out)
    }

    #[test]
    fn fn_00aa9970_joins_two_blocks() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (h, b) = heap(
            &mut e,
            &[
                (0x10, BLOCK_FREE),
                (0x20, BLOCK_FREE | BLOCK_NOT_COMMITTED),
                (0x10, 0),
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x00aa9970, &args![h, b[0], b[1]]);
        assert_eq!(
            calls_to(&e, FREE_LIST_REMOVE),
            vec![vec![h, b[0]], vec![h, b[1]]]
        );
        assert_eq!(calls_to(&e, FREE_LIST_ADD), vec![vec![h, b[0]]]);
        assert_eq!(
            e.mem.u32(b[0] + BLOCK_SIZE_FLAGS),
            0x38 | BLOCK_FREE | BLOCK_NOT_COMMITTED
        );
        assert_eq!(e.mem.u32(b[2]), b[0]);
        assert_eq!(e.mem.u32(h + HEAP_NUM_BLOCKS), 2);
        // The last two: the first becomes the tail.
        e.call(0x00aa9970, &args![h, b[0], b[2]]);
        assert_eq!(e.mem.u32(h + HEAP_BLOCK_TAIL), b[0]);
        assert_eq!(
            e.mem.u32(b[0] + BLOCK_SIZE_FLAGS),
            0x50 | BLOCK_FREE | BLOCK_NOT_COMMITTED
        );
    }

    #[test]
    fn fn_00aa9a70_drops_free_blocks_at_the_end() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (h, b) = heap(&mut e, &[(0x10, 0), (0x10, BLOCK_FREE), (0x20, BLOCK_FREE)]);
        e.call_log = Some(vec![]);
        e.call(0x00aa9a70, &args![h]);
        assert_eq!(
            calls_to(&e, FREE_LIST_REMOVE),
            vec![vec![h, b[2]], vec![h, b[1]]]
        );
        assert_eq!(e.mem.u32(h + HEAP_MEM_ALLOCATED), 0x18);
        assert_eq!(e.mem.u32(h + HEAP_NUM_BLOCKS), 1);
        assert_eq!(e.mem.u32(h + HEAP_BLOCK_TAIL), b[0]);
        assert_eq!(e.mem.u32(h + HEAP_BLOCK_HEAD), b[0]);
    }

    #[test]
    fn fn_00aa9a70_clears_both_ends_and_stops_at_a_gap() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (h, _) = heap(&mut e, &[(0x10, BLOCK_FREE)]);
        e.call(0x00aa9a70, &args![h]);
        assert_eq!(e.mem.u32(h + HEAP_BLOCK_TAIL), 0);
        assert_eq!(e.mem.u32(h + HEAP_BLOCK_HEAD), 0);
        assert_eq!(e.mem.u32(h + HEAP_MEM_ALLOCATED), 0);
        // A free last block that does not end at the allocated end stays.
        let (h, b) = heap(&mut e, &[(0x10, BLOCK_FREE)]);
        e.mem.set_u32(h + HEAP_MEM_ALLOCATED, 0x40);
        e.call(0x00aa9a70, &args![h]);
        assert_eq!(e.mem.u32(h + HEAP_BLOCK_TAIL), b[0]);
    }

    const DECOMMIT_DOUBLE: u32 = 0x0071_0000;

    #[test]
    fn fn_00aa9a70_gives_back_whole_pages() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (h, _) = heap(&mut e, &[(0x18, 0)]);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(h, vtable);
        e.mem.set_u32(vtable + HEAP_DECOMMIT_SLOT, DECOMMIT_DOUBLE);
        e.register(DECOMMIT_DOUBLE, |_, a| Ret {
            eax: a[3] & !0xfff,
            ..Ret::default()
        });
        e.mem.set_u32(h + HEAP_PAGE_SIZE, 0x1000);
        e.mem.set_u32(h + HEAP_INITIAL_SIZE, 0x1000);
        e.mem.set_u32(h + HEAP_CURRENT_SIZE, 0x4000);
        e.call_log = Some(vec![]);
        // Allocated 0x20: unused 0x3fe0 would leave 0x20 < 0x1000, so 0x3000 is offered.
        e.call(0x00aa9a70, &args![h]);
        let memory = e.mem.u32(h + HEAP_MEM);
        assert_eq!(
            calls_to(&e, DECOMMIT_DOUBLE),
            vec![vec![h, memory, 0x4000, 0x3000]]
        );
        assert_eq!(e.mem.u32(h + HEAP_CURRENT_SIZE), 0x1000);
        // Less than a page unused: nothing.
        e.call(0x00aa9a70, &args![h]);
        assert_eq!(calls_to(&e, DECOMMIT_DOUBLE).len(), 1);
        // No page size: nothing either.
        e.mem.set_u32(h + HEAP_CURRENT_SIZE, 0x8000);
        e.mem.set_u32(h + HEAP_PAGE_SIZE, 0);
        e.call(0x00aa9a70, &args![h]);
        assert_eq!(calls_to(&e, DECOMMIT_DOUBLE).len(), 1);
    }

    #[test]
    fn fn_00aa9bd0_merges_both_neighbours() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (h, b) = heap(
            &mut e,
            &[
                (0x10, 0),
                (0x10, BLOCK_FREE),
                (0x10, BLOCK_FREE),
                (0x10, BLOCK_FREE),
                (0x10, 0),
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x00aa9bd0, &args![h, b[2]]);
        // b[1] takes b[2], then b[3]; b[4] is in use, b[0] too.
        assert_eq!(e.mem.u32(b[1] + BLOCK_SIZE_FLAGS), 0x40 | BLOCK_FREE);
        assert_eq!(e.mem.u32(b[4]), b[1]);
        assert_eq!(e.mem.u32(h + HEAP_NUM_BLOCKS), 3);
        assert_eq!(e.mem.u32(h + HEAP_BLOCK_TAIL), b[4]);
    }

    #[test]
    fn fn_00aa98f0_frees_into_the_heap() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (h, b) = heap(&mut e, &[(0x10, 0), (0x20, 0), (0x10, 0)]);
        e.mem.set_u32(h + HEAP_BLOCK_MEM_ALLOCATED, 0x100);
        // The free-list double marks the block free, as 00aa9d30 does.
        e.register(FREE_LIST_ADD, |e, a| {
            let flags = e.mem.u32(a[1] + BLOCK_SIZE_FLAGS);
            e.mem.set_u32(a[1] + BLOCK_SIZE_FLAGS, flags | BLOCK_FREE);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00aa98f0, &args![h, 0u32]);
        assert!(calls_to(&e, ENTER_CRITICAL_SECTION).is_empty());
        e.call(0x00aa98f0, &args![h, b[1] + 8]);
        assert_eq!(e.mem.u32(h + HEAP_BLOCK_MEM_ALLOCATED), 0xd8);
        assert_eq!(calls_to(&e, FREE_LIST_ADD), vec![vec![h, b[1]]]);
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![h + HEAP_CRITICAL_SECTION]]
        );
        assert_eq!(
            calls_to(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![h + HEAP_CRITICAL_SECTION]]
        );
        // Its neighbours are in use: no merge.
        assert_eq!(e.mem.u32(h + HEAP_NUM_BLOCKS), 3);
    }
}

/// The formatter `00406d00` (cdecl: buffer, size, format, ...).
const FORMAT: u32 = 0x0040_6d00;
/// The message handler pointer the report goes to (its virtual at +0x14
/// takes the message type 2, the text and three zero words).
const MESSAGE_HANDLER: u32 = 0x011f_6388;
/// `"AbstractHeap: Checking list for block size %u\n"`.
const CHECKING_LIST_FORMAT: u32 = 0x010a_2a78;
/// `"\tNumber of blocks free: %u\n\tLargest free block: %u\n\tTotal space
/// ..."` (four `%u`: count, largest, total, available physical memory).
const LIST_TOTALS_FORMAT: u32 = 0x010a_2aa8;
/// `GlobalMemoryStatus` (import slot; stdcall: a 0x20-byte `MEMORYSTATUS`).
const IMPORT_GLOBAL_MEMORY_STATUS: u32 = 0x00fd_f144;

/// Formats into the 0x100-byte text and gives it to the message handler.
fn report(e: &mut Engine, text: u32, words: &[u32]) {
    let mut arguments = vec![text, 0x100];
    arguments.extend_from_slice(words);
    e.call(FORMAT, &arguments);
    let handler = e.global::<u32>(MESSAGE_HANDLER);
    e.vcall(handler, 0x14, &args![2u32, text, 0u32, 0u32, 0u32]);
}

// Translated from 00aa9760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reports a free list (thiscall on an `AbstractHeap`, `this` not read; no
/// caller or vtable in the exe): only with a message handler at `011f6388`,
/// reports `"AbstractHeap: Checking list for block size %u"` with `size`,
/// walks the blocks from the list's first block (`*list`, nothing for a null
/// `list`) to a null block, each step to the block after it in memory
/// (`block + size + 8`), counting them, their sizes and the largest, reads
/// `GlobalMemoryStatus`, and reports the count, the largest, the total and
/// the available physical memory (`MEMORYSTATUS` +0x0C).
// Translation incomplete: 00aa9760 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00aa9760(e: &mut Engine, _this: u32, size: u32, list: u32) {
    if e.global::<u32>(MESSAGE_HANDLER) == 0 {
        return;
    }
    let mut count = 0u32;
    let mut largest = 0u32;
    let mut total = 0u32;
    e.with_stack(0x100, |e, text| {
        let text = text.addr();
        report(e, text, &[CHECKING_LIST_FORMAT, size]);
        if list != 0 {
            let mut block = e.mem.u32(list);
            while block != 0 {
                count = count.wrapping_add(1);
                let length = block_size(e, block);
                total = total.wrapping_add(length);
                if length > largest {
                    largest = length;
                }
                let length = block_size(e, block);
                block = block.wrapping_add(length).wrapping_add(8);
            }
        }
        let available = e.with_stack(0x20, |e, status| {
            e.call(IMPORT_GLOBAL_MEMORY_STATUS, &[status.addr()]);
            e.mem.u32(status.addr() + 0xc)
        });
        report(
            e,
            text,
            &[LIST_TOTALS_FORMAT, count, largest, total, available],
        );
    });
}

#[cfg(test)]
mod n079_tests {
    use super::*;

    #[test]
    fn list_report_counts_until_a_null_block() {
        const REPORT: u32 = 0x00de_ad14;
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        e.register(FORMAT, |_, _| 0u32.into_ret());
        e.register(REPORT, |_, _| 0u32.into_ret());
        e.register(IMPORT_GLOBAL_MEMORY_STATUS, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 0x1234);
            0u32.into_ret()
        });
        // Without a handler nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x00aa_9760, &[0, 0x40, 0]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.put_vtable(0x0070_0000, &[0, 0, 0, 0, 0, REPORT]);
        let handler = e.mem.alloc(4);
        e.mem.set_u32(handler, 0x0070_0000);
        e.set_global(MESSAGE_HANDLER, handler);
        // Two blocks at the top of memory: the second one's size leads to
        // the null address.
        e.map(0xffff_f000, 0x1000);
        let first = 0xffff_fe00u32;
        e.mem.set_u32(first + 4, 0x4000_00f8);
        e.mem.set_u32(first + 0x104, 0xf8);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, first);
        e.call_log = Some(vec![]);
        e.call(0x00aa_9760, &[0, 0x40, list]);
        let log = e.call_log.take().unwrap();
        let formats: Vec<_> = log.iter().filter(|(a, _)| *a == FORMAT).collect();
        assert_eq!(formats[0].1[2..], [CHECKING_LIST_FORMAT, 0x40]);
        assert_eq!(
            formats[1].1[2..],
            [LIST_TOTALS_FORMAT, 2, 0xf8, 0x1f0, 0x1234]
        );
        assert_eq!(log.iter().filter(|(a, _)| *a == REPORT).count(), 2);
    }
}

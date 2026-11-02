//! `Havok SDK/hkfreelist.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB has no entry for `hkFreeList`, so its layout is described by
//! offsets only, each named where a function uses it (0x34 bytes in the free-list
//! allocator's pool):
//! `+0` free-list head, `+4` rounded element size, `+8` (cleared by the
//! constructor, never read by these functions), `+0xc` and `+0x10` block lists
//! (a block's first word links to the next, the word at `+0xc` is its size),
//! `+0x14` block size, `+0x18` alignment, `+0x1c` the first element offset,
//! `+0x20`..`+0x2c` counters (`+0x28` and `+0x2c` are the byte counts
//! `_freeBlocks` takes the block sizes from), `+0x30` the allocator the blocks
//! come from.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dd7110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the map gives the address no unit; placed here
/// because it clears the same fields that the constructor `00dd74b0` does):
/// `thiscall`, no stack arguments. Clears every word of the 0x34-byte record
/// except the one at `+8`. Returns `this`.
pub fn fn_00dd7110(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for offset in [
        0x00, 0x04, 0x0c, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30,
    ] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

// Translated from 00dd71c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeList::_freeBlocks` (Xbox PDB), `thiscall`, `RET 4`: walks the chain of
/// blocks from `first` (each block's first word links to the next). For each it
/// takes the block's size (the word at `+0xc`) off the two counters at `+0x2c`
/// and `+0x28`, and frees the block through slot +8 of the allocator at `+0x30`.
/// Returns the number of blocks.
pub fn hk_free_list_free_blocks(e: &mut Engine, this: Ptr, first: u32) -> u32 {
    let base = this.addr();
    let mut count = 0u32;
    let mut block = first;
    while block != 0 {
        let size = e.mem.u32(block + 0xc);
        let total = e.mem.u32(base + 0x2c).wrapping_sub(size);
        e.mem.set_u32(base + 0x2c, total);
        let size = e.mem.u32(block + 0xc);
        let used = e.mem.u32(base + 0x28).wrapping_sub(size);
        e.mem.set_u32(base + 0x28, used);
        let allocator = e.mem.u32(base + 0x30);
        let next = e.mem.u32(block);
        count = count.wrapping_add(1);
        e.vcall(allocator, 8, &args![block]);
        block = next;
    }
    count
}

// Translated from 00dd7200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack arguments): frees the block
/// chains at `+0xc` and then at `+0x10` (`_freeBlocks`, `00dd71c0`), clearing
/// each head, and clears `+0`, `+0x14`, `+0x28`, `+0x2c`, `+0x20`, `+0x24` and
/// `+8`.
pub fn fn_00dd7200(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let blocks = e.mem.u32(base + 0xc);
    hk_free_list_free_blocks(e, this, blocks);
    let blocks = e.mem.u32(base + 0x10);
    e.mem.set_u32(base + 0xc, 0);
    hk_free_list_free_blocks(e, this, blocks);
    for offset in [0x10, 0x00, 0x14, 0x28, 0x2c, 0x20, 0x24, 0x08] {
        e.mem.set_u32(base + offset, 0);
    }
}

// Translated from 00dd73d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words, the caller removes them): the
/// second word, when it is more than 0x10, plus the first; otherwise the first
/// plus 0x10 (both wrapping). The constructor calls it with `0xff0` and the
/// alignment.
pub fn fn_00dd73d0(_e: &mut Engine, base: u32, extra: u32) -> u32 {
    if extra > 0x10 {
        base.wrapping_add(extra)
    } else {
        base.wrapping_add(0x10)
    }
}

// Translated from 00dd74b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`: element size, alignment,
/// block size, allocator): the `hkFreeList` constructor. Clears `+0`, `+8`, `+0xc`,
/// `+0x10`, `+0x20`..`+0x2c`; stores the block size at `+0x14`, the alignment at
/// `+0x18`, the allocator at `+0x30`; the element size at `+4` is the alignment
/// when smaller than it (unsigned), otherwise rounded up to a multiple of it
/// (`(size + alignment - 1) & !(alignment - 1)`); `+0x1c` is `00dd73d0(0xff0,
/// alignment)`. The exe answers that last value in `EAX`, which is returned here.
pub fn fn_00dd74b0(
    e: &mut Engine,
    this: Ptr,
    element_size: u32,
    alignment: u32,
    block_size: u32,
    allocator: u32,
) -> u32 {
    let base = this.addr();
    for offset in [0x00, 0x0c, 0x10, 0x20, 0x24, 0x28, 0x2c, 0x08] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x14, block_size);
    e.mem.set_u32(base + 0x18, alignment);
    e.mem.set_u32(base + 0x30, allocator);
    let rounded = if element_size < alignment {
        alignment
    } else {
        element_size.wrapping_add(alignment).wrapping_sub(1) & !alignment.wrapping_sub(1)
    };
    e.mem.set_u32(base + 4, rounded);
    let first_offset = fn_00dd73d0(e, 0xff0, alignment);
    e.mem.set_u32(base + 0x1c, first_offset);
    first_offset
}

// Translated from 00dd7650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`, the four words of
/// `00dd74b0`): calls the constructor `00dd74b0` with them and returns `this`.
pub fn fn_00dd7650(
    e: &mut Engine,
    this: Ptr,
    element_size: u32,
    alignment: u32,
    block_size: u32,
    allocator: u32,
) -> Ptr {
    fn_00dd74b0(e, this, element_size, alignment, block_size, allocator);
    this
}

/// The `hkAlgorithm` quick sort the garbage collection sorts its pointer arrays with
/// (`00dd7d00`, cdecl: `(array, first, last, compare)`) and the comparison it is given
/// (`00dd70f0`).
const QUICK_SORT: u32 = 0x00dd_7d00;
const COMPARE_ADDRESSES: u32 = 0x00dd_70f0;

/// Follows a chain of words (each element's first word links to the next) from `head` and
/// counts the elements: the loop shape the exe uses, which is 0 for an empty chain.
fn chain_length(e: &Engine, head: u32) -> u32 {
    let mut count = 0u32;
    let mut element = head;
    while element != 0 {
        element = e.mem.u32(element);
        count = count.wrapping_add(1);
    }
    count
}

// Translated from 00dd72a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): walks the block chain from `first` and
/// adds up what the allocator at `+0x30` answers (virtual slot `0x10`, `(block, the word at
/// block + 4)`) for every block. Returns the sum.
pub fn fn_00dd72a0(e: &mut Engine, this: Ptr, first: u32) -> u32 {
    let allocator = e.mem.u32(this.addr() + 0x30);
    let mut sum = 0u32;
    let mut block = first;
    while block != 0 {
        let size = e.mem.u32(block + 4);
        let answer = e.vcall(allocator, 0x10, &args![block, size]).u32();
        block = e.mem.u32(block);
        sum = sum.wrapping_add(answer);
    }
    sum
}

// Translated from 00dd72f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeList::_walkMemoryBlockList` (Xbox PDB), `thiscall`, `RET 0x10`: for every block of
/// the chain from `first` and every element of the block (`count` at `+0xc`, first element at
/// `+8`, spaced by the element size at `+4`) calls `callback(element, element size, flag word,
/// context_a, context_b)` (cdecl, five words). The flag word is `false` in its low byte, as
/// the exe builds it on the stack; its other three bytes are whatever register the exe pushed
/// (the block address's upper bytes on the first call, left over from the callback after
/// that), which only matters to a callee that reads more than the byte, and are passed here as
/// the block address's upper bytes. As in the exe the element count is multiplied by the
/// element size held in a register that is only reloaded after a call.
pub fn hk_free_list_walk_memory_block_list(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    callback: u32,
    context_a: u32,
    context_b: u32,
) {
    let base = this.addr();
    let mut block = first;
    if block == 0 {
        return;
    }
    let mut size = e.mem.u32(base + 4);
    loop {
        let count = e.mem.u32(block + 0xc);
        let mut element = e.mem.u32(block + 8);
        let end = size.wrapping_mul(count).wrapping_add(element);
        if element < end {
            loop {
                let element_size = e.mem.u32(base + 4);
                e.call(
                    callback,
                    &args![
                        element,
                        element_size,
                        block & 0xffff_ff00,
                        context_a,
                        context_b
                    ],
                );
                size = e.mem.u32(base + 4);
                element = element.wrapping_add(size);
                if element >= end {
                    break;
                }
            }
        }
        block = e.mem.u32(block);
        if block == 0 {
            break;
        }
    }
}

// Translated from 00dd7360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeList::freeAllFreeBlocks` (Xbox PDB), thiscall: frees the chain of blocks at `+0x10`
/// (`_freeBlocks`, `00dd71c0`) and clears the head.
pub fn hk_free_list_free_all_free_blocks(e: &mut Engine, this: Ptr) {
    let blocks = e.mem.u32(this.addr() + 0x10);
    hk_free_list_free_blocks(e, this, blocks);
    e.mem.set_u32(this.addr() + 0x10, 0);
}

// Translated from 00dd7380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): walks the chain of blocks at `+0x10` and
/// frees (allocator at `+0x30`, slot 8, one word) every block whose word at `+4` is not above
/// `limit` (unsigned), unlinking it and taking its size (the word at `+0xc`) off the counters at
/// `+0x2c` and `+0x28`. The exe leaves 0 in EAX.
pub fn fn_00dd7380(e: &mut Engine, this: Ptr, limit: u32) {
    let base = this.addr();
    let mut link = base + 0x10;
    let mut block = e.mem.u32(link);
    while block != 0 {
        if e.mem.u32(block + 4) > limit {
            link = block;
            block = e.mem.u32(block);
        } else {
            let next = e.mem.u32(block);
            e.mem.set_u32(link, next);
            let size = e.mem.u32(block + 0xc);
            let total = e.mem.u32(base + 0x2c).wrapping_sub(size);
            e.mem.set_u32(base + 0x2c, total);
            let size = e.mem.u32(block + 0xc);
            let used = e.mem.u32(base + 0x28).wrapping_sub(size);
            e.mem.set_u32(base + 0x28, used);
            let allocator = e.mem.u32(base + 0x30);
            e.vcall(allocator, 8, &args![block]);
            block = next;
        }
    }
}

// Translated from 00dd7430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): fills the record at `out`: `+0` the
/// bytes the allocator reports for the block chains at `+0x10` and `+0xc` (`00dd72a0`
/// each, added), `+0xc` and `+0x10` the element size times the word at `+0x28`, `+4` the element
/// size times (the word at `+0x2c` minus the one at `+0x28`), `+0x14` the element size.
pub fn fn_00dd7430(e: &mut Engine, this: Ptr, out: Ptr) {
    let base = this.addr();
    let spare = e.mem.u32(base + 0x10);
    let first = fn_00dd72a0(e, this, spare);
    let blocks = e.mem.u32(base + 0xc);
    let second = fn_00dd72a0(e, this, blocks);
    let target = out.addr();
    e.mem.set_u32(target, first.wrapping_add(second));
    let used = e.mem.u32(base + 0x28);
    let element_size = e.mem.u32(base + 4);
    let all = e.mem.u32(base + 0x2c);
    let used_bytes = element_size.wrapping_mul(used);
    let free_bytes = all.wrapping_sub(used).wrapping_mul(element_size);
    e.mem.set_u32(target + 0xc, used_bytes);
    e.mem.set_u32(target + 4, free_bytes);
    let element_size = e.mem.u32(base + 4);
    e.mem.set_u32(target + 0x14, element_size);
    e.mem.set_u32(target + 0x10, used_bytes);
}

// Translated from 00dd7480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeList::getBlocks` (Xbox PDB), thiscall, `RET 8` (the second word is never read):
/// writes the addresses of the blocks of the chain at `+0xc`, then of the chain at `+0x10`,
/// one after another into the array at `out`.
pub fn hk_free_list_get_blocks(e: &mut Engine, this: Ptr, out: Ptr, _unused_2: u32) {
    let base = this.addr();
    let mut target = out.addr();
    for head in [e.mem.u32(base + 0xc), e.mem.u32(base + 0x10)] {
        let mut block = head;
        while block != 0 {
            e.mem.set_u32(target, block);
            block = e.mem.u32(block);
            target = target.wrapping_add(4);
        }
    }
}

// Translated from 00dd7510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`: element size, alignment, block size,
/// allocator): frees every block (`00dd7200`) and runs the constructor `00dd74b0` on the
/// four words again. Returns what the constructor returns.
pub fn fn_00dd7510(
    e: &mut Engine,
    this: Ptr,
    element_size: u32,
    alignment: u32,
    block_size: u32,
    allocator: u32,
) -> u32 {
    fn_00dd7200(e, this);
    fn_00dd74b0(e, this, element_size, alignment, block_size, allocator)
}

// Translated from 00dd7540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeList::addSpace` (Xbox PDB), thiscall: makes room for more elements. A spare block
/// from the chain at `+0x10` is taken first: it becomes the current run (`+0x20` its first
/// element, `+0x24` its end), is put on the chain at `+0xc` and `+0x28` goes down by one;
/// then the first element is handed out (`+0x20` moves by one element). With no spare block
/// the block size at `+0x14` is first chosen when it is 0 (`max(0x100 / element size, 1)`
/// elements) or below the first element offset at `+0x1c` (`n = size / element size`,
/// `max(n + n / 2, 8)` elements; `00dd73d0` turns the element bytes and the alignment into the
/// size), a block of that size is asked from the allocator at `+0x30` (slot 4, one word), its
/// allocated size (slot `0x10`) is kept at `+4`, its first aligned element at `+8` and its
/// element count at `+0xc`, and the counters at `+0x2c` and `+0x28` grow; no block, no change.
/// Answers the element handed out (`EAX` = the run's cursor before it moves), or 0 when the
/// allocator gave no block.
pub fn hk_free_list_add_space(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let spare = e.mem.u32(base + 0x10);
    if spare != 0 {
        let next = e.mem.u32(spare);
        e.mem.set_u32(base + 0x10, next);
        let first = e.mem.u32(spare + 8);
        e.mem.set_u32(base + 0x20, first);
        let count = e.mem.u32(spare + 0xc);
        let element_size = e.mem.u32(base + 4);
        let end = count
            .wrapping_mul(element_size)
            .wrapping_add(e.mem.u32(spare + 8));
        e.mem.set_u32(base + 0x24, end);
        let head = e.mem.u32(base + 0xc);
        e.mem.set_u32(spare, head);
        let element_size = e.mem.u32(base + 4);
        let used = e.mem.u32(base + 0x28).wrapping_sub(1);
        e.mem.set_u32(base + 0x28, used);
        e.mem.set_u32(base + 0xc, spare);
        let cursor = e.mem.u32(base + 0x20);
        e.mem
            .set_u32(base + 0x20, element_size.wrapping_add(cursor));
        return cursor;
    }
    let block_size = e.mem.u32(base + 0x14);
    let element_size = e.mem.u32(base + 4);
    let elements = if block_size == 0 {
        Some((0x100 / element_size).max(1))
    } else if block_size < e.mem.u32(base + 0x1c) {
        let count = block_size / element_size;
        Some(if count < 8 { 8 } else { count + (count >> 1) })
    } else {
        None
    };
    if let Some(elements) = elements {
        let alignment = e.mem.u32(base + 0x18);
        let size = fn_00dd73d0(e, element_size.wrapping_mul(elements), alignment);
        e.mem.set_u32(base + 0x14, size);
    }
    let allocator = e.mem.u32(base + 0x30);
    let size = e.mem.u32(base + 0x14);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    let allocator = e.mem.u32(base + 0x30);
    let size = e.mem.u32(base + 0x14);
    let allocated = e.vcall(allocator, 0x10, &args![block, size]).u32();
    e.mem.set_u32(block + 4, allocated);
    let alignment = e.mem.u32(base + 0x18);
    let first = alignment.wrapping_add(block).wrapping_add(0xf) & !alignment.wrapping_sub(1);
    e.mem.set_u32(block + 8, first);
    let element_size = e.mem.u32(base + 4);
    let count = allocated.wrapping_sub(first).wrapping_add(block) / element_size;
    e.mem.set_u32(block + 0xc, count);
    let head = e.mem.u32(base + 0xc);
    e.mem.set_u32(base + 0x20, first);
    let count = e.mem.u32(block + 0xc);
    let element_size = e.mem.u32(base + 4);
    let end = count
        .wrapping_mul(element_size)
        .wrapping_add(e.mem.u32(block + 8));
    e.mem.set_u32(base + 0x24, end);
    e.mem.set_u32(block, head);
    let element_size = e.mem.u32(base + 4);
    e.mem.set_u32(base + 0xc, block);
    let count = e.mem.u32(block + 0xc);
    let total = e.mem.u32(base + 0x2c).wrapping_add(count);
    e.mem.set_u32(base + 0x2c, total);
    let count = e.mem.u32(block + 0xc);
    let cursor = e.mem.u32(base + 0x20);
    let used = e.mem.u32(base + 0x28).wrapping_add(count.wrapping_sub(1));
    e.mem.set_u32(base + 0x28, used);
    e.mem
        .set_u32(base + 0x20, element_size.wrapping_add(cursor));
    cursor
}

/// Collects the addresses of every free element into a new array from the allocator (slot
/// `0x18`, the byte size): the elements of the free chain at `+0` in order, then the
/// remaining run from `+0x20` to `+0x24`; sorts it when it has more than one entry. Shared
/// by `findGarbage` and `walkMemory`, which both clear the run (`findGarbage`) or leave it
/// (`walkMemory`): this part only builds the array and returns `(array, entries)`.
fn free_element_addresses(e: &mut Engine, this: Ptr, entries: u32, clear_run: bool) -> (u32, u32) {
    let base = this.addr();
    let allocator = e.mem.u32(base + 0x30);
    let bytes = entries.wrapping_mul(4);
    let array = e.vcall(allocator, 0x18, &args![bytes]).u32();
    if array == 0 {
        return (0, bytes);
    }
    let mut index = 0u32;
    let mut element = e.mem.u32(base);
    while element != 0 {
        e.mem.set_u32(array + index * 4, element);
        element = e.mem.u32(element);
        index += 1;
    }
    let mut run = e.mem.u32(base + 0x20);
    while run < e.mem.u32(base + 0x24) {
        e.mem.set_u32(array + index * 4, run);
        run = run.wrapping_add(e.mem.u32(base + 4));
        index += 1;
    }
    if clear_run {
        e.mem.set_u32(base + 0x24, 0);
        e.mem.set_u32(base + 0x20, 0);
    }
    if (entries as i32) > 1 {
        e.call(
            QUICK_SORT,
            &args![array, 0u32, entries - 1, COMPARE_ADDRESSES],
        );
    }
    (array, bytes)
}

// Translated from 00dd7680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeList::findGarbage` (Xbox PDB), thiscall: frees the blocks all of whose elements
/// are free. With no block (`+0xc` empty) returns 0. Otherwise the block chain at `+0xc` is
/// copied into an array from the allocator at `+0x30` (slot `0x18`, byte size; 0 gives
/// `-1`) and sorted by address, and a second array is made of the free elements (the free
/// chain at `+0`, then the run from `+0x20` to `+0x24`, which is cleared) sorted too (a
/// failure frees the first array through slot `0x1c`, `(array, bytes)`, and gives `-1`).
/// Each block, in address order, whose elements are all found in the sorted free list is
/// moved to the spare chain at `+0x10` and its entries are zeroed; the surviving blocks are
/// relinked at `+0xc` and the remaining free elements at `+0` (both in address order), both
/// arrays are freed through slot `0x1c` and `+8` is cleared. Returns the number of blocks
/// freed.
pub fn hk_free_list_find_garbage(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let head = e.mem.u32(base + 0xc);
    if head == 0 {
        return head;
    }
    let count = chain_length(e, head);
    let allocator = e.mem.u32(base + 0x30);
    let block_bytes = count.wrapping_mul(4);
    let blocks = e.vcall(allocator, 0x18, &args![block_bytes]).u32();
    if blocks == 0 {
        return 0xffff_ffff;
    }
    let mut block = e.mem.u32(base + 0xc);
    let mut slot = blocks;
    while block != 0 {
        e.mem.set_u32(slot, block);
        block = e.mem.u32(block);
        slot = slot.wrapping_add(4);
    }
    if (count as i32) > 1 {
        e.call(
            QUICK_SORT,
            &args![blocks, 0u32, count - 1, COMPARE_ADDRESSES],
        );
    }
    let run_length =
        e.mem.u32(base + 0x24).wrapping_sub(e.mem.u32(base + 0x20)) / e.mem.u32(base + 4);
    let free_count = run_length.wrapping_add(chain_length(e, e.mem.u32(base)));
    let (free, free_bytes) = free_element_addresses(e, this, free_count, true);
    if free == 0 {
        let allocator = e.mem.u32(base + 0x30);
        e.vcall(allocator, 0x1c, &args![blocks, block_bytes]);
        return 0xffff_ffff;
    }
    let free_end = free_bytes.wrapping_add(free);
    let mut cursor = free;
    let mut freed = 0u32;
    let mut index = 0u32;
    if count != 0 {
        loop {
            if cursor >= free_end {
                break;
            }
            let candidate = e.mem.u32(blocks + index * 4);
            let element_size = e.mem.u32(base + 4);
            let elements = e.mem.u32(candidate + 0xc);
            let mut element = e.mem.u32(candidate + 8);
            let block_end = element_size.wrapping_mul(elements).wrapping_add(element);
            let whole = loop {
                if element != e.mem.u32(cursor) {
                    break element == block_end;
                }
                if element >= block_end {
                    break element == block_end;
                }
                cursor = cursor.wrapping_add(4);
                element = element.wrapping_add(element_size);
                if cursor >= free_end {
                    break element == block_end;
                }
            };
            if whole {
                let spare = e.mem.u32(base + 0x10);
                freed = freed.wrapping_add(1);
                e.mem.set_u32(candidate, spare);
                e.mem.set_u32(base + 0x10, candidate);
                e.mem.set_u32(blocks + index * 4, 0);
                let start = cursor.wrapping_sub(elements.wrapping_mul(4));
                if start < cursor {
                    let words = ((cursor - start - 1) >> 2) + 1;
                    for word in 0..words {
                        e.mem.set_u32(start + word * 4, 0);
                    }
                }
            } else if cursor < free_end {
                while e.mem.u32(cursor) < block_end {
                    cursor = cursor.wrapping_add(4);
                    if cursor >= free_end {
                        break;
                    }
                }
            }
            index += 1;
            if index >= count {
                break;
            }
        }
    }
    let mut tail = base + 0xc;
    for slot in 0..count {
        let survivor = e.mem.u32(blocks + slot * 4);
        if survivor != 0 {
            e.mem.set_u32(tail, survivor);
            tail = survivor;
        }
    }
    e.mem.set_u32(tail, 0);
    let mut tail = base;
    let mut entry = free;
    while entry < free_end {
        let element = e.mem.u32(entry);
        if element != 0 {
            e.mem.set_u32(tail, element);
            tail = element;
        }
        entry = entry.wrapping_add(4);
    }
    e.mem.set_u32(tail, 0);
    let allocator = e.mem.u32(base + 0x30);
    e.vcall(allocator, 0x1c, &args![free, free_bytes]);
    let allocator = e.mem.u32(base + 0x30);
    e.vcall(allocator, 0x1c, &args![blocks, block_bytes]);
    e.mem.set_u32(base + 8, 0);
    freed
}

// Translated from 00dd78f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFreeList::walkMemory` (Xbox PDB), thiscall, `RET 0xc`: `callback(element, element size,
/// flag word, context_a, context_b)` (cdecl) for every element of every spare block (the chain
/// at `+0x10`, through `_walkMemoryBlockList`, flag false) and, when there are blocks (`+0xc`)
/// or free elements (`+0`), for every element of every block in address order with the flag
/// true unless the element is the next free element (the sorted free elements are made as in
/// `findGarbage` but the run at `+0x20` is kept), false for those. The flag word's low byte is
/// the flag; its other bytes are as in `_walkMemoryBlockList` (the block address's upper
/// bytes where the exe pushes the block register, 0 otherwise). Both arrays come from the
/// allocator at `+0x30`; the first array's failure ends the walk, the second's frees the first
/// through slot 8 (one word). Neither array allocation failing frees them through slot `0x1c`.
pub fn hk_free_list_walk_memory(
    e: &mut Engine,
    this: Ptr,
    callback: u32,
    context_a: u32,
    context_b: u32,
) {
    let base = this.addr();
    let spare = e.mem.u32(base + 0x10);
    hk_free_list_walk_memory_block_list(e, this, spare, callback, context_a, context_b);
    let head = e.mem.u32(base + 0xc);
    if head == 0 && e.mem.u32(base) == head {
        return;
    }
    let count = chain_length(e, head);
    let allocator = e.mem.u32(base + 0x30);
    let block_bytes = count.wrapping_mul(4);
    let blocks = e.vcall(allocator, 0x18, &args![block_bytes]).u32();
    if blocks == 0 {
        return;
    }
    let mut block = e.mem.u32(base + 0xc);
    let mut slot = blocks;
    while block != 0 {
        e.mem.set_u32(slot, block);
        block = e.mem.u32(block);
        slot = slot.wrapping_add(4);
    }
    if (count as i32) > 1 {
        e.call(
            QUICK_SORT,
            &args![blocks, 0u32, count - 1, COMPARE_ADDRESSES],
        );
    }
    let run_length =
        e.mem.u32(base + 0x24).wrapping_sub(e.mem.u32(base + 0x20)) / e.mem.u32(base + 4);
    let free_count = run_length.wrapping_add(chain_length(e, e.mem.u32(base)));
    let (free, free_bytes) = free_element_addresses(e, this, free_count, false);
    if free == 0 {
        let allocator = e.mem.u32(base + 0x30);
        e.vcall(allocator, 8, &args![blocks]);
        return;
    }
    let free_end = free.wrapping_add(free_bytes);
    let mut cursor = free;
    let mut index = 0u32;
    let call_for = |e: &mut Engine, element: u32, flag_word: u32| {
        let element_size = e.mem.u32(base + 4);
        e.call(
            callback,
            &args![element, element_size, flag_word, context_a, context_b],
        );
    };
    if count != 0 {
        // Blocks while free elements are left to compare against.
        while index < count && cursor < free_end {
            let candidate = e.mem.u32(blocks + index * 4);
            let elements = e.mem.u32(candidate + 0xc);
            let element_size = e.mem.u32(base + 4);
            let first = e.mem.u32(candidate + 8);
            let block_end = element_size.wrapping_mul(elements).wrapping_add(first);
            let mut element = first;
            while element < block_end {
                // Past the last free element the exe still compares with the word after the
                // array (whatever the heap holds there), as here.
                let is_free = element == e.mem.u32(cursor);
                let flag = candidate & 0xffff_ff00 | u32::from(!is_free);
                call_for(e, element, flag);
                if is_free {
                    cursor = cursor.wrapping_add(4);
                }
                element = element.wrapping_add(e.mem.u32(base + 4));
            }
            index += 1;
        }
        // The remaining blocks cannot have free elements.
        while index < count {
            let candidate = e.mem.u32(blocks + index * 4);
            let mut element = e.mem.u32(candidate + 8);
            let mut counted = 0u32;
            while counted < e.mem.u32(candidate + 0xc) {
                call_for(e, element, 1);
                element = element.wrapping_add(e.mem.u32(base + 4));
                counted += 1;
            }
            index += 1;
        }
    }
    let allocator = e.mem.u32(base + 0x30);
    e.vcall(allocator, 0x1c, &args![free, free_bytes]);
    let allocator = e.mem.u32(base + 0x30);
    e.vcall(allocator, 0x1c, &args![blocks, block_bytes]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd7110, fn_00dd7110(Ptr) -> Ptr),
        entry!(0x00dd71c0, hk_free_list_free_blocks(Ptr, u32) -> u32),
        entry!(0x00dd7200, fn_00dd7200(Ptr)),
        entry!(0x00dd72a0, fn_00dd72a0(Ptr, u32) -> u32),
        entry!(
            0x00dd72f0,
            hk_free_list_walk_memory_block_list(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00dd7360, hk_free_list_free_all_free_blocks(Ptr)),
        entry!(0x00dd7380, fn_00dd7380(Ptr, u32)),
        entry!(0x00dd73d0, fn_00dd73d0(u32, u32) -> u32),
        entry!(0x00dd7430, fn_00dd7430(Ptr, Ptr)),
        entry!(0x00dd7480, hk_free_list_get_blocks(Ptr, Ptr, u32)),
        entry!(0x00dd74b0, fn_00dd74b0(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00dd7510, fn_00dd7510(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00dd7540, hk_free_list_add_space(Ptr) -> u32),
        entry!(0x00dd7650, fn_00dd7650(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00dd7680, hk_free_list_find_garbage(Ptr) -> u32),
        entry!(0x00dd78f0, hk_free_list_walk_memory(Ptr, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOCATOR_FREE: u32 = 0x4500_0000;

    /// An allocator whose free slot (+8) records every block it is given.
    fn allocator(e: &mut Engine) -> u32 {
        e.put_vtable(0x4400_0000, &[0, 0, ALLOCATOR_FREE]);
        e.register(ALLOCATOR_FREE, |_, _| Ret::default());
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4400_0000);
        allocator
    }

    fn calls_to(log: &[(u32, Vec<u32>)], target: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == target)
            .map(|call| call.1.clone())
            .collect()
    }

    /// A block of 0x20 bytes: link, size at +0xc.
    fn block(e: &mut Engine, next: u32, size: u32) -> u32 {
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block, next);
        e.mem.set_u32(block + 0xc, size);
        block
    }

    #[test]
    fn the_default_constructor_clears_all_but_the_word_at_8() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x34);
        e.mem.write(record, &[0xcd; 0x34]);
        assert_eq!(e.call(0x00dd_7110, &args![record]).u32(), record);
        for offset in (0..0x34).step_by(4) {
            let want = if offset == 8 { 0xcdcd_cdcd } else { 0 };
            assert_eq!(e.mem.u32(record + offset), want, "offset {offset:#x}");
        }
    }

    #[test]
    fn free_blocks_walks_the_chain_and_adjusts_the_counters() {
        let mut e = Engine::new();
        let allocator = allocator(&mut e);
        let list = e.mem.alloc(0x34);
        e.mem.set_u32(list + 0x28, 0x100);
        e.mem.set_u32(list + 0x2c, 0x200);
        e.mem.set_u32(list + 0x30, allocator);
        let second = block(&mut e, 0, 0x30);
        let first = block(&mut e, second, 0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dd_71c0, &args![list, first]).u32(), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![vec![allocator, first], vec![allocator, second]]
        );
        assert_eq!(e.mem.u32(list + 0x28), 0xc0);
        assert_eq!(e.mem.u32(list + 0x2c), 0x1c0);
    }

    #[test]
    fn free_blocks_of_an_empty_chain_does_nothing() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dd_71c0, &args![list, 0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn clearing_frees_both_chains_and_zeroes_the_fields() {
        let mut e = Engine::new();
        let allocator = allocator(&mut e);
        let list = e.mem.alloc(0x34);
        e.mem.write(list, &[0xcd; 0x34]);
        e.mem.set_u32(list + 0x28, 0x100);
        e.mem.set_u32(list + 0x2c, 0x100);
        e.mem.set_u32(list + 0x30, allocator);
        let a = block(&mut e, 0, 0x10);
        let b = block(&mut e, 0, 0x20);
        e.mem.set_u32(list + 0xc, a);
        e.mem.set_u32(list + 0x10, b);
        e.call_log = Some(vec![]);
        e.call(0x00dd_7200, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![vec![allocator, a], vec![allocator, b]]
        );
        for offset in [0x00, 0x08, 0x0c, 0x10, 0x14, 0x20, 0x24, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(list + offset), 0, "offset {offset:#x}");
        }
        // Not named by the function: the element size, alignment, first
        // element offset and the allocator.
        assert_eq!(e.mem.u32(list + 4), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(list + 0x18), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(list + 0x1c), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(list + 0x30), allocator);
    }

    #[test]
    fn the_offset_helper_adds_the_larger_of_0x10_and_the_second_word() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00dd_73d0, &args![0xff0u32, 8u32]).u32(), 0x1000);
        assert_eq!(e.call(0x00dd_73d0, &args![0xff0u32, 0x10u32]).u32(), 0x1000);
        assert_eq!(e.call(0x00dd_73d0, &args![0xff0u32, 0x20u32]).u32(), 0x1010);
    }

    #[test]
    fn the_constructor_rounds_the_element_size_to_the_alignment() {
        // (element size, alignment, rounded size)
        for (size, alignment, rounded) in [
            (0u32, 4u32, 4u32),
            (3, 4, 4),
            (4, 4, 4),
            (5, 4, 8),
            (0x220, 0x80, 0x280),
        ] {
            let mut e = Engine::new();
            let list = e.mem.alloc(0x34);
            e.mem.write(list, &[0xcd; 0x34]);
            let result = e.call(
                0x00dd_74b0,
                &args![list, size, alignment, 0x5000u32, 0x6000u32],
            );
            assert_eq!(e.mem.u32(list + 4), rounded, "size {size:#x}");
            assert_eq!(e.mem.u32(list + 0x14), 0x5000);
            assert_eq!(e.mem.u32(list + 0x18), alignment);
            assert_eq!(e.mem.u32(list + 0x30), 0x6000);
            let offset = 0xff0 + alignment.max(0x10);
            assert_eq!(e.mem.u32(list + 0x1c), offset);
            assert_eq!(result.u32(), offset);
            for field in [0x00, 0x08, 0x0c, 0x10, 0x20, 0x24, 0x28, 0x2c] {
                assert_eq!(e.mem.u32(list + field), 0, "field {field:#x}");
            }
        }
    }

    #[test]
    fn the_wrapper_passes_its_words_on_and_returns_this() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x34);
        assert_eq!(
            e.call(
                0x00dd_7650,
                &args![list, 0x30u32, 0x10u32, 0x1000u32, 0x77u32]
            )
            .u32(),
            list
        );
        assert_eq!(e.mem.u32(list + 4), 0x30);
        assert_eq!(e.mem.u32(list + 0x14), 0x1000);
        assert_eq!(e.mem.u32(list + 0x18), 0x10);
        assert_eq!(e.mem.u32(list + 0x30), 0x77);
    }

    // The allocator the memory walks use: slot 4 allocates, slot 8 frees (one word), slot
    // 0x10 reports the allocated size (the size it is given), slot 0x18 allocates an array and
    // slot 0x1c frees one with its size.
    const ALLOC_SLOT_4: u32 = 0x4600_0004;
    const FREE_SLOT_8: u32 = 0x4600_0008;
    const SIZE_SLOT_10: u32 = 0x4600_0010;
    const ARRAY_ALLOC_18: u32 = 0x4600_0018;
    const ARRAY_FREE_1C: u32 = 0x4600_001c;
    const CALLBACK: u32 = 0x4700_0000;

    fn full_allocator(e: &mut Engine) -> u32 {
        e.put_vtable(
            0x4600_0000,
            &[
                0,
                ALLOC_SLOT_4,
                FREE_SLOT_8,
                0,
                SIZE_SLOT_10,
                0,
                ARRAY_ALLOC_18,
                ARRAY_FREE_1C,
            ],
        );
        e.register(ALLOC_SLOT_4, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(FREE_SLOT_8, |_, _| Ret::default());
        e.register(SIZE_SLOT_10, |_, a| Ret {
            eax: a[2],
            ..Ret::default()
        });
        e.register(ARRAY_ALLOC_18, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(ARRAY_FREE_1C, |_, _| Ret::default());
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4600_0000);
        allocator
    }

    /// An ascending sort of the words `first..=last` of an array, standing in for the
    /// exe's quick sort.
    fn install_sort(e: &mut Engine) {
        e.register(0x00dd_7d00, |e, a| {
            let mut words: Vec<u32> = (a[1]..=a[2]).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            words.sort_unstable();
            for (i, word) in words.into_iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * (a[1] + i as u32), word);
            }
            Ret::default()
        });
    }

    /// A list over `allocator` with elements of 0x10 bytes.
    fn list_of(e: &mut Engine, allocator: u32) -> u32 {
        let list = e.mem.alloc(0x34);
        e.mem.set_u32(list + 4, 0x10);
        e.mem.set_u32(list + 0x30, allocator);
        list
    }

    /// A block holding `count` elements from `first` (link `next`).
    fn block_of(e: &mut Engine, next: u32, first: u32, count: u32) -> u32 {
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block, next);
        e.mem.set_u32(block + 8, first);
        e.mem.set_u32(block + 0xc, count);
        block
    }

    /// Chains the elements at `addresses` through their first words, returns the head.
    fn chain_elements(e: &mut Engine, addresses: &[u32]) -> u32 {
        for pair in addresses.windows(2) {
            e.mem.set_u32(pair[0], pair[1]);
        }
        if let Some(last) = addresses.last() {
            e.mem.set_u32(*last, 0);
        }
        addresses.first().copied().unwrap_or(0)
    }

    #[test]
    fn the_block_size_sum_adds_what_the_allocator_reports() {
        let mut e = Engine::new();
        let allocator = full_allocator(&mut e);
        e.register(SIZE_SLOT_10, |_, a| Ret {
            eax: a[2] * 2,
            ..Ret::default()
        });
        let list = list_of(&mut e, allocator);
        let second = block_of(&mut e, 0, 0, 0);
        e.mem.set_u32(second + 4, 20);
        let first = block_of(&mut e, second, 0, 0);
        e.mem.set_u32(first + 4, 10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dd_72a0, &args![list, first]).u32(), 60);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SIZE_SLOT_10),
            vec![vec![allocator, first, 10], vec![allocator, second, 20]]
        );
        assert_eq!(e.call(0x00dd_72a0, &args![list, 0u32]).u32(), 0);
    }

    #[test]
    fn walking_a_block_list_calls_back_for_every_element_of_every_block() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x34);
        e.mem.set_u32(list + 4, 0x10);
        e.register(CALLBACK, |_, _| Ret::default());
        let second = block_of(&mut e, 0, 0x7000, 1);
        let first = block_of(&mut e, second, 0x6000, 2);
        e.call_log = Some(vec![]);
        e.call(
            0x00dd_72f0,
            &args![list, first, CALLBACK, 0xaaaau32, 0xbbbbu32],
        );
        let log = e.call_log.take().unwrap();
        let flag = |block: u32| block & 0xffff_ff00;
        assert_eq!(
            calls_to(&log, CALLBACK),
            vec![
                vec![0x6000, 0x10, flag(first), 0xaaaa, 0xbbbb],
                vec![0x6010, 0x10, flag(first), 0xaaaa, 0xbbbb],
                vec![0x7000, 0x10, flag(second), 0xaaaa, 0xbbbb],
            ]
        );
        // An empty chain does nothing.
        e.call_log = Some(vec![]);
        e.call(0x00dd_72f0, &args![list, 0u32, CALLBACK, 1u32, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn freeing_all_free_blocks_empties_the_spare_chain() {
        let mut e = Engine::new();
        let allocator = allocator(&mut e);
        let list = e.mem.alloc(0x34);
        e.mem.set_u32(list + 0x28, 0x100);
        e.mem.set_u32(list + 0x2c, 0x200);
        e.mem.set_u32(list + 0x30, allocator);
        let second = block(&mut e, 0, 0x30);
        let first = block(&mut e, second, 0x10);
        e.mem.set_u32(list + 0x10, first);
        e.call_log = Some(vec![]);
        e.call(0x00dd_7360, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![vec![allocator, first], vec![allocator, second]]
        );
        assert_eq!(e.mem.u32(list + 0x10), 0);
        assert_eq!(e.mem.u32(list + 0x28), 0xc0);
        assert_eq!(e.mem.u32(list + 0x2c), 0x1c0);
    }

    #[test]
    fn the_small_block_release_frees_spare_blocks_up_to_the_limit() {
        let mut e = Engine::new();
        let allocator = allocator(&mut e);
        let list = e.mem.alloc(0x34);
        e.mem.set_u32(list + 0x28, 0x100);
        e.mem.set_u32(list + 0x2c, 0x200);
        e.mem.set_u32(list + 0x30, allocator);
        let third = block(&mut e, 0, 0x4);
        e.mem.set_u32(third + 4, 7);
        let second = block(&mut e, third, 0x8);
        e.mem.set_u32(second + 4, 50);
        let first = block(&mut e, second, 0x2);
        e.mem.set_u32(first + 4, 5);
        e.mem.set_u32(list + 0x10, first);
        e.call_log = Some(vec![]);
        e.call(0x00dd_7380, &args![list, 10u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![vec![allocator, first], vec![allocator, third]]
        );
        // Only the block above the limit stays on the chain.
        assert_eq!(e.mem.u32(list + 0x10), second);
        assert_eq!(e.mem.u32(second), 0);
        assert_eq!(e.mem.u32(list + 0x28), 0x100 - 2 - 4);
        assert_eq!(e.mem.u32(list + 0x2c), 0x200 - 2 - 4);
        // An empty chain does nothing.
        e.mem.set_u32(list + 0x10, 0);
        e.call(0x00dd_7380, &args![list, 10u32]);
    }

    #[test]
    fn the_statistics_record_adds_the_block_sizes_and_counts_the_elements() {
        let mut e = Engine::new();
        let allocator = full_allocator(&mut e);
        let list = list_of(&mut e, allocator);
        e.mem.set_u32(list + 0x28, 3);
        e.mem.set_u32(list + 0x2c, 10);
        let spare = block_of(&mut e, 0, 0, 0);
        e.mem.set_u32(spare + 4, 0x100);
        let used = block_of(&mut e, 0, 0, 0);
        e.mem.set_u32(used + 4, 0x40);
        e.mem.set_u32(list + 0x10, spare);
        e.mem.set_u32(list + 0xc, used);
        let out = e.mem.alloc(0x20);
        e.call(0x00dd_7430, &args![list, out]);
        assert_eq!(e.mem.u32(out), 0x140);
        assert_eq!(e.mem.u32(out + 0xc), 0x30);
        assert_eq!(e.mem.u32(out + 0x10), 0x30);
        assert_eq!(e.mem.u32(out + 4), 0x70);
        assert_eq!(e.mem.u32(out + 0x14), 0x10);
    }

    #[test]
    fn get_blocks_lists_the_used_blocks_then_the_spare_ones() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x34);
        let used_second = block_of(&mut e, 0, 0, 0);
        let used_first = block_of(&mut e, used_second, 0, 0);
        let spare = block_of(&mut e, 0, 0, 0);
        e.mem.set_u32(list + 0xc, used_first);
        e.mem.set_u32(list + 0x10, spare);
        let out = e.mem.alloc(0x10);
        e.call(0x00dd_7480, &args![list, out, 0u32]);
        let words: Vec<u32> = (0..3).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![used_first, used_second, spare]);
        assert_eq!(e.mem.u32(out + 12), 0);
    }

    #[test]
    fn the_reinitialisation_frees_the_blocks_and_runs_the_constructor() {
        let mut e = Engine::new();
        let allocator = allocator(&mut e);
        let list = e.mem.alloc(0x34);
        e.mem.set_u32(list + 0x30, allocator);
        let spare = block(&mut e, 0, 0x10);
        e.mem.set_u32(list + 0x10, spare);
        e.call_log = Some(vec![]);
        let result = e.call(
            0x00dd_7510,
            &args![list, 0x30u32, 0x10u32, 0x1000u32, 0x77u32],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATOR_FREE), vec![vec![allocator, spare]]);
        assert_eq!(result.u32(), 0x1000);
        assert_eq!(e.mem.u32(list + 4), 0x30);
        assert_eq!(e.mem.u32(list + 0x14), 0x1000);
        assert_eq!(e.mem.u32(list + 0x18), 0x10);
        assert_eq!(e.mem.u32(list + 0x30), 0x77);
        assert_eq!(e.mem.u32(list + 0x10), 0);
    }

    #[test]
    fn add_space_takes_a_spare_block_first() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x34);
        e.mem.set_u32(list + 4, 0x20);
        e.mem.set_u32(list + 0xc, 0x1111);
        e.mem.set_u32(list + 0x28, 5);
        let next_spare = block_of(&mut e, 0, 0, 0);
        let spare = block_of(&mut e, next_spare, 0x9000, 4);
        e.mem.set_u32(list + 0x10, spare);
        let handed_out = e.call(0x00dd_7540, &args![list]).u32();
        // The first element of the run is handed out (`EAX`).
        assert_eq!(handed_out, 0x9000);
        assert_eq!(e.mem.u32(list + 0x10), next_spare);
        assert_eq!(e.mem.u32(list + 0x20), 0x9020);
        assert_eq!(e.mem.u32(list + 0x24), 0x9000 + 4 * 0x20);
        assert_eq!(e.mem.u32(spare), 0x1111);
        assert_eq!(e.mem.u32(list + 0xc), spare);
        assert_eq!(e.mem.u32(list + 0x28), 4);
    }

    /// Runs `addSpace` on a list with 0x20-byte elements, alignment 0x10, the given block size
    /// and first element offset; returns the list and the allocator-visible log.
    fn add_space_with(block_size: u32, first_offset: u32) -> (Engine, u32, Vec<(u32, Vec<u32>)>) {
        let mut e = Engine::new();
        let allocator = full_allocator(&mut e);
        let list = e.mem.alloc(0x34);
        e.mem.set_u32(list + 4, 0x20);
        e.mem.set_u32(list + 0x14, block_size);
        e.mem.set_u32(list + 0x18, 0x10);
        e.mem.set_u32(list + 0x1c, first_offset);
        e.mem.set_u32(list + 0xc, 0x1111);
        e.mem.set_u32(list + 0x28, 5);
        e.mem.set_u32(list + 0x2c, 9);
        e.mem.set_u32(list + 0x30, allocator);
        e.call_log = Some(vec![]);
        e.call(0x00dd_7540, &args![list]);
        let log = e.call_log.take().unwrap();
        (e, list, log)
    }

    /// The fields `addSpace` fills from a block of `size` bytes at `block`.
    fn check_new_block(e: &Engine, list: u32, block: u32, size: u32) {
        let first = (0x10 + block + 0xf) & !0xf;
        let count = size.wrapping_sub(first).wrapping_add(block) / 0x20;
        assert_eq!(e.mem.u32(block + 4), size);
        assert_eq!(e.mem.u32(block + 8), first);
        assert_eq!(e.mem.u32(block + 0xc), count);
        assert_eq!(e.mem.u32(block), 0x1111);
        assert_eq!(e.mem.u32(list + 0xc), block);
        assert_eq!(e.mem.u32(list + 0x20), first + 0x20);
        assert_eq!(e.mem.u32(list + 0x24), first + count * 0x20);
        assert_eq!(e.mem.u32(list + 0x2c), 9 + count);
        assert_eq!(e.mem.u32(list + 0x28), 5 + count - 1);
    }

    #[test]
    fn add_space_chooses_a_default_block_size() {
        // No block size: 0x100 / 0x20 = 8 elements; 8 * 0x20 + 0x10 = 0x110.
        let (e, list, log) = add_space_with(0, 0x1000);
        assert_eq!(e.mem.u32(list + 0x14), 0x110);
        assert_eq!(calls_to(&log, ALLOC_SLOT_4).len(), 1);
        assert_eq!(calls_to(&log, ALLOC_SLOT_4)[0][1], 0x110);
        let block = calls_to(&log, SIZE_SLOT_10)[0][1];
        check_new_block(&e, list, block, 0x110);
    }

    #[test]
    fn add_space_grows_a_block_size_below_the_first_element_offset() {
        // 0x40 / 0x20 = 2 elements, below 8, so 8: 0x110 again.
        let (e, list, _) = add_space_with(0x40, 0x100);
        assert_eq!(e.mem.u32(list + 0x14), 0x110);
        // 0x400 / 0x20 = 0x20 elements, 0x30 with half again: 0x30 * 0x20 + 0x10.
        let (e, list, _) = add_space_with(0x400, 0x1000);
        assert_eq!(e.mem.u32(list + 0x14), 0x610);
    }

    #[test]
    fn add_space_keeps_a_block_size_that_is_big_enough() {
        let (e, list, log) = add_space_with(0x200, 0x100);
        assert_eq!(e.mem.u32(list + 0x14), 0x200);
        let block = calls_to(&log, SIZE_SLOT_10)[0][1];
        assert_eq!(calls_to(&log, ALLOC_SLOT_4)[0][1], 0x200);
        check_new_block(&e, list, block, 0x200);
    }

    #[test]
    fn add_space_changes_nothing_when_the_allocator_has_no_block() {
        let mut e = Engine::new();
        let allocator = full_allocator(&mut e);
        e.register(ALLOC_SLOT_4, |_, _| Ret::default());
        let list = e.mem.alloc(0x34);
        e.mem.set_u32(list + 4, 0x20);
        e.mem.set_u32(list + 0x14, 0x200);
        e.mem.set_u32(list + 0x1c, 0x100);
        e.mem.set_u32(list + 0xc, 0x1111);
        e.mem.set_u32(list + 0x30, allocator);
        e.call(0x00dd_7540, &args![list]);
        assert_eq!(e.mem.u32(list + 0xc), 0x1111);
        assert_eq!(e.mem.u32(list + 0x20), 0);
        assert_eq!(e.mem.u32(list + 0x2c), 0);
    }

    /// The word addresses of the 0x10-byte elements `0, 1, ...` of a region.
    fn elements(region: u32, indexes: &[u32]) -> Vec<u32> {
        indexes.iter().map(|i| region + 0x10 * i).collect()
    }

    #[test]
    fn find_garbage_frees_the_blocks_whose_elements_are_all_free() {
        let mut e = Engine::new();
        let allocator = full_allocator(&mut e);
        install_sort(&mut e);
        let region = e.mem.alloc(0x100);
        let list = list_of(&mut e, allocator);
        // Block A holds the elements 0 and 1, block B the elements 4 and 5.
        let block_a = block_of(&mut e, 0, region, 2);
        let block_b = block_of(&mut e, 0, region + 0x40, 2);
        e.mem.set_u32(block_a, block_b);
        // Free: 0 and 1 (all of A) and 5 (half of B), chained in a different order.
        let free = chain_elements(&mut e, &elements(region, &[5, 0, 1]));
        e.mem.set_u32(list, free);
        e.mem.set_u32(list + 0xc, block_a);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dd_7680, &args![list]).u32(), 1);
        let log = e.call_log.take().unwrap();
        // Block A went to the spare chain, block B stays on the chain at +0xc.
        assert_eq!(e.mem.u32(list + 0x10), block_a);
        assert_eq!(e.mem.u32(block_a), 0);
        assert_eq!(e.mem.u32(list + 0xc), block_b);
        assert_eq!(e.mem.u32(block_b), 0);
        // The free chain keeps only element 5.
        assert_eq!(e.mem.u32(list), region + 0x50);
        assert_eq!(e.mem.u32(region + 0x50), 0);
        assert_eq!(e.mem.u32(list + 8), 0);
        // The arrays (two blocks, three free elements) are freed through slot 0x1c.
        let freed = calls_to(&log, ARRAY_FREE_1C);
        assert_eq!(freed.len(), 2);
        assert_eq!(freed[0][2], 12);
        assert_eq!(freed[1][2], 8);
        assert_eq!(calls_to(&log, ARRAY_ALLOC_18).len(), 2);
    }

    #[test]
    fn find_garbage_counts_the_run_as_free_and_clears_it() {
        let mut e = Engine::new();
        let allocator = full_allocator(&mut e);
        install_sort(&mut e);
        let region = e.mem.alloc(0x100);
        let list = list_of(&mut e, allocator);
        let block_a = block_of(&mut e, 0, region, 2);
        let block_c = block_of(&mut e, 0, region + 0x80, 2);
        e.mem.set_u32(block_a, block_c);
        e.mem.set_u32(list + 0xc, block_a);
        // The run covers all of block C.
        e.mem.set_u32(list + 0x20, region + 0x80);
        e.mem.set_u32(list + 0x24, region + 0xa0);
        assert_eq!(e.call(0x00dd_7680, &args![list]).u32(), 1);
        assert_eq!(e.mem.u32(list + 0x10), block_c);
        assert_eq!(e.mem.u32(list + 0xc), block_a);
        assert_eq!(e.mem.u32(list + 0x20), 0);
        assert_eq!(e.mem.u32(list + 0x24), 0);
        assert_eq!(e.mem.u32(list), 0);
    }

    #[test]
    fn find_garbage_without_blocks_or_memory() {
        let mut e = Engine::new();
        let allocator = full_allocator(&mut e);
        install_sort(&mut e);
        let list = list_of(&mut e, allocator);
        assert_eq!(e.call(0x00dd_7680, &args![list]).u32(), 0);
        let region = e.mem.alloc(0x40);
        let block_a = block_of(&mut e, 0, region, 2);
        e.mem.set_u32(list + 0xc, block_a);
        // The first array cannot be allocated.
        e.register(ARRAY_ALLOC_18, |_, _| Ret::default());
        assert_eq!(e.call(0x00dd_7680, &args![list]).u32(), 0xffff_ffff);
        // The second array cannot: the first is freed again.
        let mut calls = 0;
        e.register_double(ARRAY_ALLOC_18, move |e, a| {
            calls += 1;
            Ret {
                eax: if calls == 1 { e.mem.alloc(a[1]) } else { 0 },
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dd_7680, &args![list]).u32(), 0xffff_ffff);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_FREE_1C).len(), 1);
        assert_eq!(calls_to(&log, ARRAY_FREE_1C)[0][2], 4);
        assert_eq!(e.mem.u32(list + 0xc), block_a);
    }

    #[test]
    fn walk_memory_reports_every_element_with_its_free_flag() {
        let mut e = Engine::new();
        let allocator = full_allocator(&mut e);
        install_sort(&mut e);
        e.register(CALLBACK, |_, _| Ret::default());
        let region = e.mem.alloc(0x100);
        let list = list_of(&mut e, allocator);
        // A spare block (elements at 0x7000, walked first with the flag false), block A
        // (elements 0 and 1) and block B (elements 4 and 5); only element 1 is free.
        let spare = block_of(&mut e, 0, 0x7000, 1);
        let block_a = block_of(&mut e, 0, region, 2);
        let block_b = block_of(&mut e, 0, region + 0x40, 2);
        e.mem.set_u32(block_a, block_b);
        let free = chain_elements(&mut e, &elements(region, &[1]));
        e.mem.set_u32(list, free);
        e.mem.set_u32(list + 0xc, block_a);
        e.mem.set_u32(list + 0x10, spare);
        e.call_log = Some(vec![]);
        e.call(0x00dd_78f0, &args![list, CALLBACK, 0xaaaau32, 0xbbbbu32]);
        let log = e.call_log.take().unwrap();
        let calls = calls_to(&log, CALLBACK);
        let summary: Vec<(u32, u32)> = calls.iter().map(|c| (c[0], c[2] & 0xff)).collect();
        assert_eq!(
            summary,
            vec![
                (0x7000, 0),
                (region, 1),
                (region + 0x10, 0),
                (region + 0x40, 1),
                (region + 0x50, 1),
            ]
        );
        assert!(calls
            .iter()
            .all(|c| c[1] == 0x10 && c[3] == 0xaaaa && c[4] == 0xbbbb));
        assert_eq!(calls_to(&log, ARRAY_FREE_1C).len(), 2);
        // The free chain and the blocks are unchanged.
        assert_eq!(e.mem.u32(list), free);
        assert_eq!(e.mem.u32(list + 0xc), block_a);
    }

    #[test]
    fn walk_memory_stops_early_without_blocks_or_memory() {
        let mut e = Engine::new();
        let allocator = full_allocator(&mut e);
        install_sort(&mut e);
        e.register(CALLBACK, |_, _| Ret::default());
        let list = list_of(&mut e, allocator);
        e.call_log = Some(vec![]);
        e.call(0x00dd_78f0, &args![list, CALLBACK, 1u32, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // With a block but no array memory the walk ends after the spare blocks.
        let region = e.mem.alloc(0x40);
        let block_a = block_of(&mut e, 0, region, 2);
        e.mem.set_u32(list + 0xc, block_a);
        e.register(ARRAY_ALLOC_18, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00dd_78f0, &args![list, CALLBACK, 1u32, 2u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), CALLBACK).is_empty());
        // A second array that cannot be allocated frees the first through slot 8.
        let mut calls = 0;
        e.register_double(ARRAY_ALLOC_18, move |e, a| {
            calls += 1;
            Ret {
                eax: if calls == 1 { e.mem.alloc(a[1]) } else { 0 },
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00dd_78f0, &args![list, CALLBACK, 1u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FREE_SLOT_8).len(), 1);
        assert!(calls_to(&log, CALLBACK).is_empty());
    }
}

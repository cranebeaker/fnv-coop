//! `(unplaced)/run_00aa9d00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Most of them keep the free lists of `AbstractHeap` (`BSCore/abstractheap.cpp`; the layout
//! is `crate::units::bscore::abstractheap::AbstractHeap`, read here at its offsets). A heap
//! block has an 8-byte header: the previous block at +0 and the size in the low 28 bits of
//! the word at +4 (the high bits are flags, 0x40000000 marking a free block); a free block
//! links to its neighbours in its free list at +8 (previous) and +0xc (next). A free list is
//! 8 bytes: its first block at +0 and its last block at +4.

#[allow(unused_imports)]
use crate::prelude::*;

/// `AbstractHeap::iDefaultAlignment` (Xbox PDB) +0x8.
const HEAP_DEFAULT_ALIGNMENT: u32 = 0x8;
/// `AbstractHeap::iNumFreeBlocks` (Xbox PDB) +0x40.
const HEAP_NUM_FREE_BLOCKS: u32 = 0x40;
/// `AbstractHeap::pSmallFreeList` (Xbox PDB) +0x44: 1024 free lists, one per size step.
const HEAP_SMALL_FREE_LISTS: u32 = 0x44;
/// `AbstractHeap::LargeFreeList` (Xbox PDB) +0x2048: the free list of the larger blocks.
const HEAP_LARGE_FREE_LIST: u32 = 0x2048;
/// `AbstractHeap::pSubBuckets` (Xbox PDB) +0x2058: 8-byte entries, a count at +0.
const HEAP_SUB_BUCKETS: u32 = 0x2058;
/// The number of small free lists; a step index from it on uses the large list.
const SMALL_FREE_LIST_COUNT: i32 = 0x400;
/// Sizes above this have no sub-bucket.
const SUB_BUCKET_MAX_SIZE: i32 = 0x1000;

/// A heap block's header: the size-and-flags word (+4), the free-list links (+8, +0xc).
const BLOCK_SIZE_FLAGS: u32 = 0x4;
const BLOCK_FREE_PREVIOUS: u32 = 0x8;
const BLOCK_FREE_NEXT: u32 = 0xc;
const BLOCK_SIZE_MASK: u32 = 0x0fff_ffff;
const BLOCK_FREE: u32 = 0x4000_0000;
/// A free list's last block (+4); its first block is its first word.
const LIST_LAST: u32 = 0x4;

/// The size in a block's header.
fn block_size(e: &Engine, block: u32) -> u32 {
    e.mem.u32(block + BLOCK_SIZE_FLAGS) & BLOCK_SIZE_MASK
}

/// The free list of blocks of `size` bytes: index `size / iDefaultAlignment - 1` (unsigned
/// division; compared signed, so a size below the alignment gives index -1, the 8 bytes
/// before the first small list), the large list from index 0x400 on.
fn free_list_for(e: &Engine, heap: u32, size: u32) -> u32 {
    let alignment = e.mem.u32(heap + HEAP_DEFAULT_ALIGNMENT);
    let index = (size / alignment).wrapping_sub(1) as i32;
    if index >= SMALL_FREE_LIST_COUNT {
        heap + HEAP_LARGE_FREE_LIST
    } else {
        heap.wrapping_add(HEAP_SMALL_FREE_LISTS)
            .wrapping_add((index as u32).wrapping_mul(8))
    }
}

/// The sub-bucket of a size: -1 above 0x1000, else `(size / iDefaultAlignment - 1) / 64`
/// (signed divisions, truncating as `IDIV` and the `CDQ`/`AND 0x3f`/`SAR 6` idiom do).
fn sub_bucket_for(e: &Engine, heap: u32, size: i32) -> i32 {
    if size > SUB_BUCKET_MAX_SIZE {
        -1
    } else {
        let alignment = e.mem.u32(heap + HEAP_DEFAULT_ALIGNMENT) as i32;
        (size / alignment).wrapping_sub(1) / 64
    }
}

// Translated from 00aa9d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, one word): puts a block on its free list.
/// Clears its links, marks it free, appends it after the list's last block (or makes it the
/// only block of an empty list), counts it in `iNumFreeBlocks` and in its sub-bucket
/// ([`fn_00aa9ef0`]).
pub fn fn_00aa9d30(e: &mut Engine, this: Ptr, block: Ptr) {
    let (heap, block) = (this.addr(), block.addr());
    e.mem.set_u32(block + BLOCK_FREE_NEXT, 0);
    e.mem.set_u32(block + BLOCK_FREE_PREVIOUS, 0);
    // The exe tests a constant 1 here (an inlined flag setter given `true`), so only the
    // setting branch runs; the clearing one is kept as the exe has it.
    let mark_free = true;
    let flags = e.mem.u32(block + BLOCK_SIZE_FLAGS);
    if mark_free {
        e.mem.set_u32(block + BLOCK_SIZE_FLAGS, flags | BLOCK_FREE);
    } else {
        e.mem.set_u32(block + BLOCK_SIZE_FLAGS, flags & !BLOCK_FREE);
    }
    let size = block_size(e, block);
    let list = free_list_for(e, heap, size);
    let last = e.mem.u32(list + LIST_LAST);
    if last != 0 {
        e.mem.set_u32(block + BLOCK_FREE_PREVIOUS, last);
        let next = e.mem.u32(last + BLOCK_FREE_NEXT);
        e.mem.set_u32(block + BLOCK_FREE_NEXT, next);
        if next != 0 {
            e.mem.set_u32(next + BLOCK_FREE_PREVIOUS, block);
        }
        e.mem.set_u32(last + BLOCK_FREE_NEXT, block);
        e.mem.set_u32(list + LIST_LAST, block);
    } else {
        e.mem.set_u32(list, block);
        e.mem.set_u32(list + LIST_LAST, block);
    }
    let count = e.mem.u32(heap + HEAP_NUM_FREE_BLOCKS).wrapping_add(1);
    e.mem.set_u32(heap + HEAP_NUM_FREE_BLOCKS, count);
    let size = block_size(e, block);
    fn_00aa9ef0(e, this, size as i32);
}

// Translated from 00aa9ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, one word): adds one to the count of the
/// sub-bucket of `size` (none above 0x1000).
pub fn fn_00aa9ef0(e: &mut Engine, this: Ptr, size: i32) {
    let bucket = sub_bucket_for(e, this.addr(), size);
    if bucket != -1 {
        let at = this
            .addr()
            .wrapping_add(HEAP_SUB_BUCKETS)
            .wrapping_add((bucket as u32).wrapping_mul(8));
        let count = e.mem.u32(at).wrapping_add(1);
        e.mem.set_u32(at, count);
    }
}

// Translated from 00aa9f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, one word): takes one from the count of the
/// sub-bucket of `size` (none above 0x1000).
pub fn fn_00aa9f50(e: &mut Engine, this: Ptr, size: i32) {
    let bucket = sub_bucket_for(e, this.addr(), size);
    if bucket != -1 {
        let at = this
            .addr()
            .wrapping_add(HEAP_SUB_BUCKETS)
            .wrapping_add((bucket as u32).wrapping_mul(8));
        let count = e.mem.u32(at).wrapping_sub(1);
        e.mem.set_u32(at, count);
    }
}

// Translated from 00aaa5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on a free heap block, no words): unlinks the block from its
/// neighbours (the previous block's next becomes this block's next and the other way round)
/// and clears both of its links.
pub fn fn_00aaa5e0(e: &mut Engine, this: Ptr) {
    let block = this.addr();
    if e.mem.u32(block + BLOCK_FREE_PREVIOUS) != 0 {
        let next = e.mem.u32(block + BLOCK_FREE_NEXT);
        let previous = e.mem.u32(block + BLOCK_FREE_PREVIOUS);
        e.mem.set_u32(previous + BLOCK_FREE_NEXT, next);
    }
    if e.mem.u32(block + BLOCK_FREE_NEXT) != 0 {
        let previous = e.mem.u32(block + BLOCK_FREE_PREVIOUS);
        let next = e.mem.u32(block + BLOCK_FREE_NEXT);
        e.mem.set_u32(next + BLOCK_FREE_PREVIOUS, previous);
    }
    e.mem.set_u32(block + BLOCK_FREE_NEXT, 0);
    e.mem.set_u32(block + BLOCK_FREE_PREVIOUS, 0);
}

// Translated from 00aa9e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, two words: a free list and a block): takes the
/// block off the list. Unlinks it ([`fn_00aaa5e0`]), clears its free mark, moves the list's
/// first or last block on to its old neighbour when it was that block, takes it out of its
/// sub-bucket ([`fn_00aa9f50`]) and from `iNumFreeBlocks`.
pub fn fn_00aa9e50(e: &mut Engine, this: Ptr, list: Ptr, block: Ptr) {
    let (list, block) = (list.addr(), block.addr());
    let next = e.mem.u32(block + BLOCK_FREE_NEXT);
    let previous = e.mem.u32(block + BLOCK_FREE_PREVIOUS);
    fn_00aaa5e0(e, Ptr::new(block));
    // The exe tests a constant 0 here (an inlined flag setter given `false`), so only the
    // clearing branch runs; the setting one is kept as the exe has it.
    let mark_free = false;
    let flags = e.mem.u32(block + BLOCK_SIZE_FLAGS);
    if mark_free {
        e.mem.set_u32(block + BLOCK_SIZE_FLAGS, flags | BLOCK_FREE);
    } else {
        e.mem.set_u32(block + BLOCK_SIZE_FLAGS, flags & !BLOCK_FREE);
    }
    if block == e.mem.u32(list) {
        e.mem.set_u32(list, next);
    }
    if block == e.mem.u32(list + LIST_LAST) {
        e.mem.set_u32(list + LIST_LAST, previous);
    }
    let size = block_size(e, block);
    fn_00aa9f50(e, this, size as i32);
    let count = e
        .mem
        .u32(this.addr() + HEAP_NUM_FREE_BLOCKS)
        .wrapping_sub(1);
    e.mem.set_u32(this.addr() + HEAP_NUM_FREE_BLOCKS, count);
}

// Translated from 00aaa660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`__thiscall` on an `AbstractHeap`, one word): takes a free block off the free
/// list of its size ([`fn_00aa9e50`]).
pub fn fn_00aaa660(e: &mut Engine, this: Ptr, block: Ptr) {
    let size = block_size(e, block.addr());
    let list = free_list_for(e, this.addr(), size);
    fn_00aa9e50(e, this, Ptr::new(list), block);
}

// Translated from 00aaa650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 0 and ignores its three stack words (`__thiscall`, `RET 0xC`;
/// `XOR EAX,EAX`). The map gives it no name; it is, for example, slot +0xC
/// of the `BSSaveDataSystemUtility` vtable (`010a4288`), which `0084fc70`
/// calls with `"-SAVE-"`.
pub fn fn_00aaa650(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aaa650, fn_00aaa650(Ptr, u32, u32, u32) -> u32),
        entry!(0x00aa9d30, fn_00aa9d30(Ptr, Ptr)),
        entry!(0x00aa9ef0, fn_00aa9ef0(Ptr, i32)),
        entry!(0x00aa9f50, fn_00aa9f50(Ptr, i32)),
        entry!(0x00aaa5e0, fn_00aaa5e0(Ptr)),
        entry!(0x00aa9e50, fn_00aa9e50(Ptr, Ptr, Ptr)),
        entry!(0x00aaa660, fn_00aaa660(Ptr, Ptr)),
        entry!(0x00aa_9d00, fn_00aa9d00(u32)),
        entry!(0x00aa_9d10, fn_00aa9d10(u32)),
        entry!(0x00aa_9d20, fn_00aa9d20(u32, u32)),
        entry!(0x00aa_9fb0, fn_00aa9fb0(u32)),
        entry!(0x00aa_a0d0, fn_00aaa0d0(u32, u32)),
        entry!(0x00aa_a0e0, fn_00aaa0e0(u32, u32, u32)),
        entry!(0x00aa_a310, fn_00aaa310(u32, u32) -> u32),
        entry!(0x00aa_a430, fn_00aaa430(u32, u32, u32) -> u32),
        entry!(0x00aa_a560, fn_00aaa560(u32, u32) -> i32),
        entry!(0x00aa_a5b0, fn_00aaa5b0(u32, u32) -> u32),
        entry!(0x00aa_a6d0, fn_00aaa6d0(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero_for_any_arguments() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        let back = e.call(0x00aa_a650, &args![this, 0x0107_fabcu32, 1u32, 2u32]);
        assert_eq!(back.u32(), 0);
    }

    /// A heap with alignment 16.
    fn heap(e: &mut Engine) -> u32 {
        let heap = e.mem.alloc(0x2100);
        e.mem.set_u32(heap + HEAP_DEFAULT_ALIGNMENT, 16);
        heap
    }

    /// A block of `size` bytes with the given flags.
    fn block(e: &mut Engine, size: u32, flags: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + BLOCK_SIZE_FLAGS, size | flags);
        block
    }

    fn sub_bucket(e: &Engine, heap: u32, index: u32) -> u32 {
        e.mem.u32(heap + HEAP_SUB_BUCKETS + index * 8)
    }

    #[test]
    fn fn_00aa9d30_appends_to_the_list_of_its_size() {
        let mut e = Engine::new();
        let h = heap(&mut e);
        // Size 0x40: index 3, sub-bucket 0.
        let list = h + HEAP_SMALL_FREE_LISTS + 3 * 8;
        let a = block(&mut e, 0x40, 0x2000_0000);
        e.mem.set_u32(a + BLOCK_FREE_NEXT, 0x77);
        e.call(0x00aa9d30, &args![h, a]);
        assert_eq!(e.mem.u32(a + BLOCK_SIZE_FLAGS), 0x6000_0040);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (a, a));
        assert_eq!(e.mem.u32(a + BLOCK_FREE_NEXT), 0);
        let b = block(&mut e, 0x40, 0);
        // A block after `a` already: the new one goes between them.
        let c = block(&mut e, 0x40, 0);
        e.mem.set_u32(a + BLOCK_FREE_NEXT, c);
        e.call(0x00aa9d30, &args![h, b]);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (a, b));
        assert_eq!(e.mem.u32(a + BLOCK_FREE_NEXT), b);
        assert_eq!(e.mem.u32(b + BLOCK_FREE_PREVIOUS), a);
        assert_eq!(e.mem.u32(b + BLOCK_FREE_NEXT), c);
        assert_eq!(e.mem.u32(c + BLOCK_FREE_PREVIOUS), b);
        assert_eq!(e.mem.u32(h + HEAP_NUM_FREE_BLOCKS), 2);
        assert_eq!(sub_bucket(&e, h, 0), 2);
    }

    #[test]
    fn fn_00aa9d30_puts_large_blocks_on_the_large_list() {
        let mut e = Engine::new();
        let h = heap(&mut e);
        // Size 0x4010: index 0x400, the large list, no sub-bucket.
        let a = block(&mut e, 0x4010, 0);
        e.call(0x00aa9d30, &args![h, a]);
        let list = h + HEAP_LARGE_FREE_LIST;
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (a, a));
        assert!((0..16).all(|i| sub_bucket(&e, h, i) == 0));
    }

    #[test]
    fn sub_bucket_counts_follow_the_size() {
        let mut e = Engine::new();
        let h = heap(&mut e);
        // 0x1000: index 0xff, bucket 3; 0x1010 has none; 8 (below the alignment): bucket 0.
        e.call(0x00aa9ef0, &args![h, 0x1000i32]);
        e.call(0x00aa9ef0, &args![h, 0x1010i32]);
        e.call(0x00aa9ef0, &args![h, 8i32]);
        assert_eq!(sub_bucket(&e, h, 3), 1);
        assert_eq!(sub_bucket(&e, h, 0), 1);
        e.call(0x00aa9f50, &args![h, 0x1000i32]);
        e.call(0x00aa9f50, &args![h, 0x1010i32]);
        e.call(0x00aa9f50, &args![h, 0x10i32]);
        assert_eq!(sub_bucket(&e, h, 3), 0);
        assert_eq!(sub_bucket(&e, h, 0), 0);
        e.call(0x00aa9f50, &args![h, 0x100i32]);
        assert_eq!(sub_bucket(&e, h, 0), u32::MAX);
    }

    #[test]
    fn fn_00aaa5e0_unlinks_a_block() {
        let mut e = Engine::new();
        let (a, b, c) = (
            block(&mut e, 0x10, 0),
            block(&mut e, 0x10, 0),
            block(&mut e, 0x10, 0),
        );
        e.mem.set_u32(a + BLOCK_FREE_NEXT, b);
        e.mem.set_u32(b + BLOCK_FREE_PREVIOUS, a);
        e.mem.set_u32(b + BLOCK_FREE_NEXT, c);
        e.mem.set_u32(c + BLOCK_FREE_PREVIOUS, b);
        e.call(0x00aaa5e0, &args![b]);
        assert_eq!(e.mem.u32(a + BLOCK_FREE_NEXT), c);
        assert_eq!(e.mem.u32(c + BLOCK_FREE_PREVIOUS), a);
        assert_eq!((e.mem.u32(b + 8), e.mem.u32(b + 0xc)), (0, 0));
        // Alone: nothing else is touched.
        e.call(0x00aaa5e0, &args![b]);
        assert_eq!((e.mem.u32(b + 8), e.mem.u32(b + 0xc)), (0, 0));
    }

    #[test]
    fn fn_00aa9e50_and_fn_00aaa660_take_blocks_off_their_list() {
        let mut e = Engine::new();
        let h = heap(&mut e);
        let list = h + HEAP_SMALL_FREE_LISTS + 8;
        let a = block(&mut e, 0x20, 0);
        let b = block(&mut e, 0x20, 0);
        let c = block(&mut e, 0x20, 0);
        for x in [a, b, c] {
            e.call(0x00aa9d30, &args![h, x]);
        }
        assert_eq!(sub_bucket(&e, h, 0), 3);
        // The middle one: the list's ends stay.
        e.call(0x00aa9e50, &args![h, list, b]);
        assert_eq!(e.mem.u32(b + BLOCK_SIZE_FLAGS), 0x20);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (a, c));
        assert_eq!(e.mem.u32(a + BLOCK_FREE_NEXT), c);
        // The last one, through its size.
        e.call(0x00aaa660, &args![h, c]);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (a, a));
        // The only one left.
        e.call(0x00aaa660, &args![h, a]);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (0, 0));
        assert_eq!(e.mem.u32(h + HEAP_NUM_FREE_BLOCKS), 0);
        assert_eq!(sub_bucket(&e, h, 0), 0);
    }
}

/// `AbstractHeap` fields (Xbox PDB names, PC offsets): `iNumBlocks` +0x34,
/// `pBlockHead` +0x38, `pBlockTail` +0x3C.
const HEAP_NUM_BLOCKS: u32 = 0x34;
const HEAP_BLOCK_HEAD: u32 = 0x38;
const HEAP_BLOCK_TAIL: u32 = 0x3c;
/// The words +0x24 and +0x2C the statistics subtract.
const HEAP_MEM_ALLOCATED: u32 = 0x24;
const HEAP_BLOCK_MEM_ALLOCATED: u32 = 0x2c;
/// `AbstractHeap::CriticalSectionO` (Xbox PDB) +0x20D8.
const HEAP_CRITICAL_SECTION: u32 = 0x20d8;
/// `EnterCriticalSection` and `LeaveCriticalSection` (import slots).
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The sub-bucket count the walk in `00aa9fb0` covers (each entry: a count
/// at +0 and a pointer to its first block's slot at +4).
const SUB_BUCKET_COUNT: i32 = 0x10;
/// The heap's destructor `00aa8da0` (thiscall).
const HEAP_DESTRUCTOR: u32 = 0x00aa_8da0;
/// The memory manager object and its free method (thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// A stream's write method: vtable slot +8 (thiscall: buffer, size;
/// answers the bytes written).
const STREAM_WRITE_SLOT: u32 = 0x8;

fn enter(e: &mut Engine, heap: u32) {
    e.call(
        IMPORT_ENTER_CRITICAL_SECTION,
        &[heap + HEAP_CRITICAL_SECTION],
    );
}

fn leave(e: &mut Engine, heap: u32) {
    e.call(
        IMPORT_LEAVE_CRITICAL_SECTION,
        &[heap + HEAP_CRITICAL_SECTION],
    );
}

/// The block after `block` in memory: its data size plus the 8-byte header.
fn next_in_memory(e: &Engine, block: u32) -> u32 {
    block.wrapping_add(block_size(e, block)).wrapping_add(8)
}

// Translated from 00aa9d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method (no caller or vtable in the exe).
pub fn fn_00aa9d00(_e: &mut Engine, _this: u32) {}

// Translated from 00aa9d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method: slot +0x2C of the `AbstractHeap` (`010a2bbc`) and
/// `MemoryHeap` (`010a28fc`) vtables, by their RTTI.
pub fn fn_00aa9d10(_e: &mut Engine, _this: u32) {}

// Translated from 00aa9d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method taking one word it does not read (no caller or vtable in
/// the exe).
pub fn fn_00aa9d20(_e: &mut Engine, _this: u32, _unused_1: u32) {}

// Translated from 00aa9fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the heap's blocks without changing them (slot +0x14 of the
/// `AbstractHeap` and `MemoryHeap` vtables, by their RTTI), inside the
/// critical section: for each of the first 16 sub-buckets (+0x2058, 8 bytes
/// each) with a non-zero count, the blocks from its first-block slot (+4)
/// along their free-list next links (+0xC); then the 1024 small free lists
/// (+0x44) and the large one (+0x2048) the same way, each block given to the
/// empty [`fn_00aaa0d0`]; then the block chain from the tail (+0x3C) back
/// along the previous-block words (+0) to its start.
pub fn fn_00aa9fb0(e: &mut Engine, this: u32) {
    enter(e, this);
    let visit = |e: &mut Engine, mut block: u32| {
        while block != 0 {
            fn_00aaa0d0(e, this, block);
            block = e.mem.u32(block + BLOCK_FREE_NEXT);
        }
    };
    for bucket in 0..SUB_BUCKET_COUNT {
        let entry = this
            .wrapping_add(HEAP_SUB_BUCKETS)
            .wrapping_add((bucket as u32) * 8);
        if e.mem.u32(entry) != 0 {
            let slot = e.mem.u32(entry + 4);
            let first = e.mem.u32(slot);
            visit(e, first);
        }
    }
    for list in 0..SMALL_FREE_LIST_COUNT {
        let first = e.mem.u32(this + HEAP_SMALL_FREE_LISTS + (list as u32) * 8);
        visit(e, first);
    }
    let first = e.mem.u32(this + HEAP_LARGE_FREE_LIST);
    visit(e, first);
    let mut block = e.mem.u32(this + HEAP_BLOCK_TAIL);
    while block != 0 {
        block = e.mem.u32(block);
    }
    leave(e, this);
}

// Translated from 00aaa0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty per-block step of [`fn_00aa9fb0`]'s walk (thiscall, the block not
/// read).
pub fn fn_00aaa0d0(_e: &mut Engine, _this: u32, _block: u32) {}

// Translated from 00aaa0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills heap statistics at `out` (slot +0x18 of the `AbstractHeap`
/// vtable, by its RTTI; also called by `00aa7c60`), inside the critical
/// section: +0x00..+0x14 copy the heap's words +0x04, +0x14, +0x1C, +0x24,
/// +0x34 and +0x40; +0x28 is 0x20F0; +0x2C is 0x2080 (0x2000, then 0x80
/// added); +0x30 the block count times the 8-byte header; +0x34 0; +0x18 the
/// heap's +0x24 minus +0x2C; +0x1C its +0x2C; +0x20 0x0FFFFFFF; +0x24 0.
/// With `full` set, +0x18 and +0x1C are recounted by walking the block chain
/// from the head (+0x38) to the tail (+0x3C): free blocks (and their headers)
/// in +0x18 with the smallest (signed) in +0x20 and the largest in +0x24,
/// used blocks in +0x1C; +0x20 becomes 0 when the copied +0x14 is 0. Last,
/// +0x38 is `out[+4] - out[+0xC] + out[+0x18]`.
pub fn fn_00aaa0e0(e: &mut Engine, this: u32, out: u32, full: u32) {
    enter(e, this);
    for (to, from) in [
        (0u32, 4u32),
        (4, 0x14),
        (8, 0x1c),
        (0xc, 0x24),
        (0x10, 0x34),
        (0x14, 0x40),
    ] {
        let word = e.mem.u32(this + from);
        e.mem.set_u32(out + to, word);
    }
    // The code's two locals: the block count times 0 and 8 minus 0.
    let zero = 0u32;
    let header = 8u32.wrapping_sub(zero);
    let unused_size = e.mem.u32(out + 0x10).wrapping_mul(zero);
    e.mem.set_u32(out + 0x28, 0x20f0);
    e.mem.set_u32(out + 0x2c, 0x2000);
    let reserved = e.mem.u32(out + 0x2c).wrapping_add(0x80);
    e.mem.set_u32(out + 0x2c, reserved);
    let headers = e.mem.u32(this + HEAP_NUM_BLOCKS).wrapping_mul(header);
    e.mem.set_u32(out + 0x30, headers);
    e.mem.set_u32(out + 0x34, unused_size);
    let mut block = e.mem.u32(this + HEAP_BLOCK_HEAD);
    let free = e
        .mem
        .u32(this + HEAP_MEM_ALLOCATED)
        .wrapping_sub(e.mem.u32(this + HEAP_BLOCK_MEM_ALLOCATED));
    e.mem.set_u32(out + 0x18, free);
    let used = e.mem.u32(this + HEAP_BLOCK_MEM_ALLOCATED);
    e.mem.set_u32(out + 0x1c, used);
    e.mem.set_u32(out + 0x20, BLOCK_SIZE_MASK);
    e.mem.set_u32(out + 0x24, 0);
    if full as u8 != 0 {
        e.mem.set_u32(out + 0x18, 0);
        e.mem.set_u32(out + 0x1c, 0);
        while block != 0 {
            let size = block_size(e, block);
            if e.mem.u32(block + BLOCK_SIZE_FLAGS) & BLOCK_FREE != 0 {
                let sum = size
                    .wrapping_add(header)
                    .wrapping_add(e.mem.u32(out + 0x18));
                e.mem.set_u32(out + 0x18, sum);
                if (size as i32) < (e.mem.u32(out + 0x20) as i32) {
                    e.mem.set_u32(out + 0x20, size);
                }
                if (size as i32) > (e.mem.u32(out + 0x24) as i32) {
                    e.mem.set_u32(out + 0x24, size);
                }
            } else {
                let sum = size
                    .wrapping_add(header)
                    .wrapping_add(e.mem.u32(out + 0x1c));
                e.mem.set_u32(out + 0x1c, sum);
            }
            block = if block == e.mem.u32(this + HEAP_BLOCK_TAIL) {
                0
            } else {
                next_in_memory(e, block)
            };
        }
        if e.mem.u32(out + 0x14) == 0 {
            e.mem.set_u32(out + 0x20, 0);
        }
    }
    let spare = e
        .mem
        .u32(out + 4)
        .wrapping_sub(e.mem.u32(out + 0xc))
        .wrapping_add(e.mem.u32(out + 0x18));
    e.mem.set_u32(out + 0x38, spare);
    leave(e, this);
}

// Translated from 00aaa310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the free-list sizes to `stream` (slot +0x1C of the `AbstractHeap`
/// vtable, by its RTTI; also called by `00aa7ca0`): 0 for a null stream;
/// otherwise, inside the critical section, the count 0x401, then for each
/// step `i` below it the pair (block size `(i + 1) * iDefaultAlignment`, the
/// number of blocks on that size's free list: the small list
/// `size / alignment - 1`, the large one from 0x400 on), each through the
/// stream's write (vtable +8). Answers the bytes written.
pub fn fn_00aaa310(e: &mut Engine, this: u32, stream: u32) -> u32 {
    if stream == 0 {
        return 0;
    }
    enter(e, this);
    // The exe's locals: the step count at +0, then the pair (size, blocks).
    let written = e.with_stack(12, |e, locals| {
        let steps = locals.addr();
        let pair = steps + 4;
        e.mem.set_u32(steps, 0x401);
        let mut written = e.vcall(stream, STREAM_WRITE_SLOT, &[steps, 4]).u32();
        let mut step = 0i32;
        while step < e.mem.u32(steps) as i32 {
            let alignment = e.mem.u32(this + HEAP_DEFAULT_ALIGNMENT);
            let size = (step.wrapping_add(1) as u32).wrapping_mul(alignment);
            e.mem.set_u32(pair, size);
            e.mem.set_u32(pair + 4, 0);
            // `DIV`: an alignment of 0 is a divide fault in the exe.
            let index = (size / alignment).wrapping_sub(1) as i32;
            let list = if index >= SMALL_FREE_LIST_COUNT {
                this + HEAP_LARGE_FREE_LIST
            } else {
                this.wrapping_add(HEAP_SMALL_FREE_LISTS)
                    .wrapping_add((index as u32).wrapping_mul(8))
            };
            let mut block = e.mem.u32(list);
            while block != 0 {
                let count = e.mem.u32(pair + 4).wrapping_add(1);
                e.mem.set_u32(pair + 4, count);
                block = e.mem.u32(block + BLOCK_FREE_NEXT);
            }
            let n = e.vcall(stream, STREAM_WRITE_SLOT, &[pair, 8]).u32();
            written = n.wrapping_add(written);
            step += 1;
        }
        written
    });
    leave(e, this);
    written
}

// Translated from 00aaa430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the block map to `stream` (slot +0x20 of the `AbstractHeap`
/// vtable, by its RTTI; also called by `00aa7d00`): 0 for a null stream;
/// otherwise, inside the critical section, with `full` set: the block count
/// (+0x34, from the heap itself) and for each block from the head (+0x38),
/// the next one in memory each time, a 0x18-byte record: 8, its size,
/// whether it is free (0 or 1), two words the code never sets, and -1;
/// without `full`, one zero word. Each through the stream's write (vtable
/// +8). Answers the bytes written.
// Translation incomplete: 00aaa430 - the record's words +0x0C and +0x10 are uninitialized stack in the exe; they are 0 here.
pub fn fn_00aaa430(e: &mut Engine, this: u32, stream: u32, full: u32) -> u32 {
    if stream == 0 {
        return 0;
    }
    enter(e, this);
    let mut written = 0u32;
    if full as u8 != 0 {
        let n = e
            .vcall(stream, STREAM_WRITE_SLOT, &[this + HEAP_NUM_BLOCKS, 4])
            .u32();
        written = n.wrapping_add(written);
        written = e.with_stack(0x18, |e, record| {
            let record = record.addr();
            e.mem.set_u32(record, 8);
            let mut block = e.mem.u32(this + HEAP_BLOCK_HEAD);
            let mut index = 0i32;
            while index < e.mem.u32(this + HEAP_NUM_BLOCKS) as i32 {
                e.mem.set_u32(record + 4, block_size(e, block));
                let free = e.mem.u32(block + BLOCK_SIZE_FLAGS) & BLOCK_FREE != 0;
                e.mem.set_u32(record + 8, u32::from(free));
                e.mem.set_u32(record + 0x14, u32::MAX);
                let n = e.vcall(stream, STREAM_WRITE_SLOT, &[record, 0x18]).u32();
                written = n.wrapping_add(written);
                block = next_in_memory(e, block);
                index += 1;
            }
            written
        });
    } else {
        let n = e.with_stack(4, |e, zero| {
            e.mem.set_u32(zero.addr(), 0);
            e.vcall(stream, STREAM_WRITE_SLOT, &[zero.addr(), 4]).u32()
        });
        written = n.wrapping_add(written);
    }
    leave(e, this);
    written
}

// Translated from 00aaa560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The length of the chain from `node` along the words at +0x4C (not
/// counting `node`), or -1 for a null `node` (no caller or vtable in the
/// exe; `this` is not read).
pub fn fn_00aaa560(e: &mut Engine, _this: u32, node: u32) -> i32 {
    if node == 0 {
        return -1;
    }
    let mut count = 0i32;
    let mut at = node;
    while e.mem.u32(at + 0x4c) != 0 {
        count = count.wrapping_add(1);
        at = e.mem.u32(at + 0x4c);
    }
    count
}

// Translated from 00aaa5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `AbstractHeap` (slot 0 of its vtable
/// `010a2bbc`, by its RTTI): the destructor `00aa8da0`, then with bit 0 of
/// `flags` the object freed through the memory manager. Returns `this`.
pub fn fn_00aaa5b0(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.call(HEAP_DESTRUCTOR, &[this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &[MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00aaa6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `this` (no caller or vtable in the exe).
pub fn fn_00aaa6d0(_e: &mut Engine, this: u32) -> u32 {
    this
}

#[cfg(test)]
mod n079_tests {
    use super::*;

    fn heap_engine() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_ENTER_CRITICAL_SECTION, |_, _| 0u32.into_ret());
        e.register(IMPORT_LEAVE_CRITICAL_SECTION, |_, _| 0u32.into_ret());
        e
    }

    #[test]
    fn empty_methods_do_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00aa_9d00, &[this]);
        e.call(0x00aa_9d10, &[this]);
        e.call(0x00aa_9d20, &[this, 1]);
        e.call(0x00aa_a0d0, &[this, 1]);
        assert_eq!(e.call_log.take().unwrap().len(), 4);
        assert_eq!(e.call(0x00aa_a6d0, &[this]).u32(), this);
        assert_eq!(e.mem.bytes(this, 0x10), vec![0; 0x10]);
    }

    #[test]
    fn walk_reads_every_list_inside_the_lock() {
        let mut e = heap_engine();
        let this = e.mem.alloc(0x2100);
        let blocks: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x10)).collect();
        e.mem.set_u32(blocks[0] + BLOCK_FREE_NEXT, blocks[1]);
        // Sub-bucket 2 counts one block and points at a slot holding it.
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, blocks[0]);
        e.mem.set_u32(this + HEAP_SUB_BUCKETS + 16, 1);
        e.mem.set_u32(this + HEAP_SUB_BUCKETS + 20, slot);
        e.mem
            .set_u32(this + HEAP_SMALL_FREE_LISTS + 8 * 0x3ff, blocks[2]);
        e.mem.set_u32(this + HEAP_LARGE_FREE_LIST, blocks[1]);
        e.mem.set_u32(blocks[2], blocks[1]);
        e.mem.set_u32(this + HEAP_BLOCK_TAIL, blocks[2]);
        let before = e.mem.bytes(this, 0x2100);
        e.call_log = Some(vec![]);
        e.call(0x00aa_9fb0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (IMPORT_ENTER_CRITICAL_SECTION, vec![this + 0x20d8]));
        assert_eq!(log[2], (IMPORT_LEAVE_CRITICAL_SECTION, vec![this + 0x20d8]));
        assert_eq!(e.mem.bytes(this, 0x2100), before);
    }

    /// A heap of three blocks (0x10 used, 0x20 free, 0x08 free) in one
    /// buffer.
    fn three_blocks(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x2100);
        let buffer = e.mem.alloc(0x60);
        e.mem.set_u32(buffer + 4, 0x10);
        e.mem.set_u32(buffer + 0x1c, BLOCK_FREE | 0x20);
        e.mem.set_u32(buffer + 0x44, BLOCK_FREE | 0x08);
        e.mem.set_u32(this + HEAP_BLOCK_HEAD, buffer);
        e.mem.set_u32(this + HEAP_BLOCK_TAIL, buffer + 0x40);
        e.mem.set_u32(this + HEAP_NUM_BLOCKS, 3);
        this
    }

    #[test]
    fn statistics_copy_and_recount() {
        let mut e = heap_engine();
        let this = three_blocks(&mut e);
        for (i, offset) in [4u32, 0x14, 0x1c, 0x24, 0x2c, 0x40].into_iter().enumerate() {
            e.mem.set_u32(this + offset, 0x100 * (i as u32 + 1));
        }
        let out = e.mem.alloc(0x3c);
        e.call(0x00aa_a0e0, &[this, out, 0x100]);
        let words: Vec<u32> = (0..15).map(|i| e.mem.u32(out + 4 * i)).collect();
        let spare = 0x400u32.wrapping_sub(0x500);
        assert_eq!(
            words,
            vec![
                0x100,
                0x200,
                0x300,
                0x400,
                3,
                0x600,
                spare,
                0x500,
                0x0fff_ffff,
                0,
                0x20f0,
                0x2080,
                24,
                0,
                0x200u32.wrapping_sub(0x400).wrapping_add(spare)
            ]
        );
        e.call(0x00aa_a0e0, &[this, out, 1]);
        assert_eq!(e.mem.u32(out + 0x18), 0x28 + 0x10);
        assert_eq!(e.mem.u32(out + 0x1c), 0x18);
        assert_eq!(e.mem.u32(out + 0x20), 0x08);
        assert_eq!(e.mem.u32(out + 0x24), 0x20);
        assert_eq!(e.mem.u32(out + 0x38), 0x200u32.wrapping_sub(0x400) + 0x38);
        // No free blocks counted in +0x14: the smallest is cleared.
        e.mem.set_u32(this + 0x40, 0);
        e.call(0x00aa_a0e0, &[this, out, 1]);
        assert_eq!(e.mem.u32(out + 0x20), 0);
    }

    const WRITE: u32 = 0x00de_ad08;

    /// A stream whose write double records the words written.
    fn stream(e: &mut Engine) -> (u32, std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>) {
        let written = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = written.clone();
        e.register_double(WRITE, move |e, a| {
            let words = (0..a[2] / 4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            record.borrow_mut().push(words);
            a[2].into_ret()
        });
        e.put_vtable(0x0071_0000, &[0, 0, WRITE]);
        let stream = e.mem.alloc(4);
        e.mem.set_u32(stream, 0x0071_0000);
        (stream, written)
    }

    #[test]
    fn free_list_sizes_are_written() {
        let mut e = heap_engine();
        let (stream, written) = stream(&mut e);
        let this = e.mem.alloc(0x2100);
        assert_eq!(e.call(0x00aa_a310, &[this, 0]).u32(), 0);
        e.mem.set_u32(this + HEAP_DEFAULT_ALIGNMENT, 8);
        // Two blocks of 0x18 bytes (list 2), one large one.
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        e.mem.set_u32(a + BLOCK_FREE_NEXT, b);
        e.mem.set_u32(this + HEAP_SMALL_FREE_LISTS + 16, a);
        e.mem.set_u32(this + HEAP_LARGE_FREE_LIST, b);
        assert_eq!(e.call(0x00aa_a310, &[this, stream]).u32(), 4 + 8 * 0x401);
        let written = written.borrow();
        assert_eq!(written.len(), 0x402);
        assert_eq!(written[0], vec![0x401]);
        assert_eq!(written[1], vec![8, 0]);
        assert_eq!(written[3], vec![0x18, 2]);
        assert_eq!(written[0x400], vec![0x2000, 0]);
        assert_eq!(written[0x401], vec![0x2008, 1]);
    }

    #[test]
    fn block_map_is_written() {
        let mut e = heap_engine();
        let (stream, written) = stream(&mut e);
        let this = three_blocks(&mut e);
        assert_eq!(e.call(0x00aa_a430, &[this, 0, 1]).u32(), 0);
        assert_eq!(e.call(0x00aa_a430, &[this, stream, 1]).u32(), 4 + 3 * 0x18);
        assert_eq!(e.call(0x00aa_a430, &[this, stream, 0x100]).u32(), 4);
        let written = written.borrow();
        assert_eq!(written[0], vec![3]);
        assert_eq!(written[1], vec![8, 0x10, 0, 0, 0, u32::MAX]);
        assert_eq!(written[2], vec![8, 0x20, 1, 0, 0, u32::MAX]);
        assert_eq!(written[3], vec![8, 0x08, 1, 0, 0, u32::MAX]);
        assert_eq!(written[4], vec![0]);
    }

    #[test]
    fn chain_length_along_plus_0x4c() {
        let mut e = Engine::new();
        let nodes: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x50)).collect();
        e.mem.set_u32(nodes[0] + 0x4c, nodes[1]);
        e.mem.set_u32(nodes[1] + 0x4c, nodes[2]);
        assert_eq!(e.call(0x00aa_a560, &[0, nodes[0]]).i32(), 2);
        assert_eq!(e.call(0x00aa_a560, &[0, nodes[2]]).i32(), 0);
        assert_eq!(e.call(0x00aa_a560, &[0, 0]).i32(), -1);
    }

    #[test]
    fn heap_deleting_destructor() {
        for (flags, deleted) in [(1u32, true), (0, false)] {
            let mut e = Engine::new();
            e.register(HEAP_DESTRUCTOR, |_, _| 0u32.into_ret());
            e.register(MEMORY_FREE, |_, _| 0u32.into_ret());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00aa_a5b0, &[0x5000, flags]).u32(), 0x5000);
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1], (HEAP_DESTRUCTOR, vec![0x5000]));
            assert_eq!(
                log.contains(&(MEMORY_FREE, vec![MEMORY_MANAGER, 0x5000])),
                deleted
            );
        }
    }
}

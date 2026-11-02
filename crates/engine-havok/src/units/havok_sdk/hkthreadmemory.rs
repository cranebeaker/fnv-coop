//! `Havok SDK/hkthreadmemory.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB has no entry for `hkThreadMemory`, so the offsets are described
//! where the code uses them.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkThreadMemory`'s virtual table (`010cacd0`; the class name is confirmed by
/// the RTTI record before it).
const VTABLE: u32 = 0x010c_acd0;

/// The size class of a block size, from the ladder both table loops of `00d2c100`
/// use (every comparison signed): sizes up to 0x10 are class 1, then up to 0x20,
/// 0x30, 0x40, 0x60, 0x80, 0xa0, 0xc0, 0x100, 0x140, 0x200, 0x280, 0x400, 0x800,
/// 0x1000 and 0x2000 are classes 2 to 16. `None` is where the exe executes `INT3`
/// (a size over 0x2000); the loops never reach it (their sizes stop at 0x2000).
fn size_class(size: i32) -> Option<u32> {
    const LIMITS: [i32; 16] = [
        0x10, 0x20, 0x30, 0x40, 0x60, 0x80, 0xa0, 0xc0, 0x100, 0x140, 0x200, 0x280, 0x400, 0x800,
        0x1000, 0x2000,
    ];
    LIMITS
        .iter()
        .position(|limit| size <= *limit)
        .map(|index| index as u32 + 1)
}

// Translated from 00d2c100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack arguments): fills the
/// size-class tables of an `hkThreadMemory`. For the 41 sizes `16 * n`
/// (`n` = 0 to 0x28) the class goes to the byte table at `+0xd8 + n`; then for
/// the sizes 0x400, 0x800, ... (below 0x2400) the class goes to the word
/// table at `+0x104`, one word each. Every size is also stored in the word at
/// `+0x94 + 4 * class`. The exe's `INT3` for a size over 0x2000 is a panic here;
/// no size of these loops reaches it.
pub fn fn_00d2c100(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for n in 0..=0x28u32 {
        let size = n << 4;
        let class = size_class(size as i32).unwrap_or_else(|| {
            panic!("FalloutNV.exe executes INT3 at 00d2c1fe: size {size:#x} has no size class")
        });
        e.mem.set_u8(base + 0xd8 + n, class as u8);
        e.mem.set_u32(base + 0x94 + class * 4, size);
    }
    let mut size = 0x400u32;
    let mut slot = base + 0x104;
    while (size as i32) < 0x2400 {
        let class = size_class(size as i32).unwrap_or_else(|| {
            panic!("FalloutNV.exe executes INT3 at 00d2c31e: size {size:#x} has no size class")
        });
        e.mem.set_u32(slot, class);
        e.mem.set_u32(base + 0x94 + class * 4, size);
        size += 0x400;
        slot += 4;
    }
}

// Translated from 00d2c3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `first`, `second`): when
/// `first` is not zero it is stored at `+4` and `second` at `+8`; for zero only
/// `+4` is cleared (`+8` is left as it was).
pub fn fn_00d2c3a0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    if first != 0 {
        e.mem.set_u32(this.addr() + 4, first);
        e.mem.set_u32(this.addr() + 8, second);
    } else {
        e.mem.set_u32(this.addr() + 4, 0);
    }
}

// Translated from 00d2c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::hkThreadMemory` (Xbox PDB), `thiscall`, no stack arguments:
/// stores the virtual table, clears the 0x88 bytes from `+0xc` to `+0x93`, fills
/// the size-class tables (`00d2c100`) and calls `00d2c3a0` with `(0, 8)`, which
/// clears the word at `+4`. Returns `this`.
pub fn hk_thread_memory_hk_thread_memory(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    for word in 0..0x22u32 {
        e.mem.set_u32(base + 0xc + word * 4, 0);
    }
    fn_00d2c100(e, this);
    fn_00d2c3a0(e, this, 0, 8);
    this
}

/// The allocator behind a thread memory: the object at `+4`, called through its virtual table.
const ALLOCATOR: u32 = 4;
/// The table's slot `+4`: `blockAlloc(size)`.
const SLOT_BLOCK_ALLOC: u32 = 4;
/// The table's slot `+8`: `blockFree(block, size)`.
const SLOT_BLOCK_FREE: u32 = 8;
/// The allocator's slot `+0x18`: `blockAllocBatch(blocks, count, size)`.
const SLOT_BLOCK_ALLOC_BATCH: u32 = 0x18;
/// The allocator's slot `+0x1c`: `blockFreeBatch(blocks, count, size)`.
const SLOT_BLOCK_FREE_BATCH: u32 = 0x1c;
/// The allocator's slot `+0x24`: `canAllocTotal(size)`.
const SLOT_CAN_ALLOC_TOTAL: u32 = 0x24;
/// The allocator's slot `+0x28`: `getMemoryStatistics(statistics)`.
const SLOT_GET_MEMORY_STATISTICS: u32 = 0x28;
/// The allocator's slot `+0x2c`: `getAllocatedSize(block, size)`.
const SLOT_GET_ALLOCATED_SIZE: u32 = 0x2c;
/// `hkMemUtil::memCpy` (`00d2caa0`, cdecl: destination, source, byte count).
const MEM_CPY: u32 = 0x00d2_caa0;

/// The row limit (`+8`): how many blocks of one size class a row keeps.
const ROW_LIMIT: u32 = 8;
/// A size class's block list head (`+0xc + 8 * class`).
const ROW_HEAD: u32 = 0xc;
/// A size class's block count (`+0x10 + 8 * class`).
const ROW_COUNT: u32 = 0x10;
/// A size class's block size (`+0x94 + 4 * class`).
const CLASS_SIZE: u32 = 0x94;
/// The byte table (`+0xd8`) mapping `(size + 15) >> 4` to a size class, for sizes up to 0x280.
const SMALL_CLASS_TABLE: u32 = 0xd8;
/// The word table (`+0x104`) mapping `(size - 1) >> 10` to a size class, for sizes over 0x280.
const LARGE_CLASS_TABLE: u32 = 0x104;
/// The largest size the rows serve; bigger blocks go straight to the allocator.
const LARGEST_ROW_SIZE: i32 = 0x2000;

/// The address of the word `index` of the array at `base`.
fn word_at(base: u32, index: i32) -> u32 {
    base.wrapping_add((index as u32).wrapping_mul(4))
}

/// The size class of `size` (at most 0x2000) the way the exe looks it up: the signed byte table
/// for sizes up to 0x280, otherwise the word table.
fn class_of(e: &Engine, this: u32, size: i32) -> i32 {
    if size <= 0x280 {
        let index = size.wrapping_add(0xf) >> 4;
        let table = this.wrapping_add(SMALL_CLASS_TABLE);
        i32::from(e.mem.i8(table.wrapping_add(index as u32)))
    } else {
        let index = size.wrapping_sub(1) >> 10;
        e.mem
            .i32(word_at(this.wrapping_add(LARGE_CLASS_TABLE), index))
    }
}

/// The address of the row field `field` (`ROW_HEAD` or `ROW_COUNT`) of size class `class`.
fn row_field(this: u32, class: i32, field: u32) -> u32 {
    this.wrapping_add((class as u32).wrapping_mul(8))
        .wrapping_add(field)
}

/// The address of the block size of size class `class`.
fn class_size_field(this: u32, class: i32) -> u32 {
    word_at(this.wrapping_add(CLASS_SIZE), class)
}

/// Takes the block at the front of the class's list as the loops of this unit do: a block comes
/// off only when the head is not null (the count goes down by one and the head moves on to the
/// block's first word); an empty list gives a null.
fn pop_block(e: &mut Engine, this: u32, class: i32) -> u32 {
    let head = e.mem.u32(row_field(this, class, ROW_HEAD));
    if head != 0 {
        let count = e.mem.i32(row_field(this, class, ROW_COUNT));
        e.mem
            .set_i32(row_field(this, class, ROW_COUNT), count.wrapping_sub(1));
        let next = e.mem.u32(head);
        e.mem.set_u32(row_field(this, class, ROW_HEAD), next);
    }
    head
}

/// Pushes `block` on the front of the class's list.
fn push_block(e: &mut Engine, this: u32, class: i32, block: u32) {
    let head = e.mem.u32(row_field(this, class, ROW_HEAD));
    let count = e.mem.i32(row_field(this, class, ROW_COUNT));
    e.mem
        .set_i32(row_field(this, class, ROW_COUNT), count.wrapping_add(1));
    e.mem.set_u32(block, head);
    e.mem.set_u32(row_field(this, class, ROW_HEAD), block);
    hkmem_check::note_freed(e, block);
}

// Translated from 00d2c000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::bufAlloc` (Xbox PDB), `thiscall`, one stack word (`size`, a pointer to the
/// wanted size): replaces it with the size it points to and tail-calls the table's slot `+4`
/// (`blockAlloc`), whose result is returned.
pub fn hk_thread_memory_buf_alloc(e: &mut Engine, this: Ptr, size: Ptr) -> u32 {
    let wanted = e.mem.u32(size.addr());
    e.vcall(this.addr(), SLOT_BLOCK_ALLOC, &args![wanted]).u32()
}

// Translated from 00d2c020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::bufFree` (Xbox PDB), `thiscall`, two stack words (`block`, `size`):
/// tail-calls the table's slot `+8` (`blockFree`) with the same words.
pub fn hk_thread_memory_buf_free(e: &mut Engine, this: Ptr, block: u32, size: u32) {
    e.vcall(this.addr(), SLOT_BLOCK_FREE, &args![block, size]);
}

// Translated from 00d2c030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::blockAllocBatch` (Xbox PDB), `thiscall`, `RET 0xc`: fills `blocks[0..count]`
/// with blocks of `size` bytes. A size over 0x2000 is handed to the allocator's slot `+0x18`
/// (`blockAllocBatch`) as it is; otherwise the table's slot `+4` (`blockAlloc`) is called once
/// per block (a count of zero or less does nothing).
pub fn hk_thread_memory_block_alloc_batch(
    e: &mut Engine,
    this: Ptr,
    blocks: Ptr,
    count: i32,
    size: i32,
) {
    if size > LARGEST_ROW_SIZE {
        let allocator = e.mem.u32(this.addr() + ALLOCATOR);
        e.vcall(
            allocator,
            SLOT_BLOCK_ALLOC_BATCH,
            &args![blocks, count, size],
        );
        return;
    }
    let mut index = 0i32;
    while index < count {
        let block = e.vcall(this.addr(), SLOT_BLOCK_ALLOC, &args![size]).u32();
        e.mem.set_u32(word_at(blocks.addr(), index), block);
        index += 1;
    }
}

// Translated from 00d2c090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::blockFreeBatch` (Xbox PDB), `thiscall`, `RET 0xc`: gives back
/// `blocks[0..count]`, each of `size` bytes. A size over 0x2000 is handed to the allocator's
/// slot `+0x1c` (`blockFreeBatch`) as it is; otherwise the table's slot `+8` (`blockFree`) is
/// called once per block.
pub fn hk_thread_memory_block_free_batch(
    e: &mut Engine,
    this: Ptr,
    blocks: Ptr,
    count: i32,
    size: i32,
) {
    if size > LARGEST_ROW_SIZE {
        let allocator = e.mem.u32(this.addr() + ALLOCATOR);
        e.vcall(
            allocator,
            SLOT_BLOCK_FREE_BATCH,
            &args![blocks, count, size],
        );
        return;
    }
    let mut index = 0i32;
    while index < count {
        let block = e.mem.u32(word_at(blocks.addr(), index));
        e.vcall(this.addr(), SLOT_BLOCK_FREE, &args![block, size]);
        index += 1;
    }
}

// Translated from 00d2c0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::getMemoryStatistics` (Xbox PDB), `thiscall`, one stack word
/// (`statistics`): tail-calls the allocator's slot `+0x28` with it.
pub fn hk_thread_memory_get_memory_statistics(e: &mut Engine, this: Ptr, statistics: u32) {
    let allocator = e.mem.u32(this.addr() + ALLOCATOR);
    e.vcall(allocator, SLOT_GET_MEMORY_STATISTICS, &args![statistics]);
}

// Translated from 00d2c350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::getCachedSizeUnchecked` (Xbox PDB), `thiscall`, no stack words: the bytes
/// the rows keep, the sum over the size classes 0 to 16 of the block count
/// (`+0x10 + 8 * class`) times the class's block size (`+0x94 + 4 * class`). The exe adds two
/// classes per pass into two sums (even classes in one, odd in the other), class 16 last; the
/// arithmetic wraps.
pub fn hk_thread_memory_get_cached_size_unchecked(e: &mut Engine, this: Ptr) -> i32 {
    let base = this.addr();
    let mut even = 0i32;
    let mut odd = 0i32;
    for pass in 0..8u32 {
        let count_even = e.mem.i32(base + 0x10 + 0x10 * pass);
        let size_even = e.mem.i32(base + 0x94 + 8 * pass);
        let count_odd = e.mem.i32(base + 0x18 + 0x10 * pass);
        let size_odd = e.mem.i32(base + 0x98 + 8 * pass);
        even = even.wrapping_add(size_even.wrapping_mul(count_even));
        odd = odd.wrapping_add(size_odd.wrapping_mul(count_odd));
    }
    let last = e.mem.i32(base + 0xd4).wrapping_mul(e.mem.i32(base + 0x90));
    odd.wrapping_add(even).wrapping_add(last)
}

/// The refill code of `blockAlloc` (`00d2c5a0` jumps to `00d2c3c0` with the size class in its
/// stack word and `this` in ECX; the engine map has no function there), `thiscall`, one stack
/// word (`class`): with the row limit zero it returns what the allocator's slot `+4`
/// (`blockAlloc`) gives for the class's block size; otherwise it asks the allocator's slot
/// `+0x18` (`blockAllocBatch`) for the smaller (signed) of the limit and four blocks, chains
/// all but the first on the row's list and returns the first.
fn block_alloc_refill(e: &mut Engine, this: u32, class: i32) -> u32 {
    let limit = e.mem.i32(this + ROW_LIMIT);
    let allocator = e.mem.u32(this + ALLOCATOR);
    let block_size = e.mem.u32(class_size_field(this, class));
    if limit == 0 {
        return e
            .vcall(allocator, SLOT_BLOCK_ALLOC, &args![block_size])
            .u32();
    }
    let batch = limit.min(4);
    e.with_stack(0x10, |e, blocks| {
        e.vcall(
            allocator,
            SLOT_BLOCK_ALLOC_BATCH,
            &args![blocks, batch, block_size],
        );
        let mut index = 1i32;
        while index < batch {
            let block = e.mem.u32(word_at(blocks.addr(), index));
            push_block(e, this, class, block);
            index += 1;
        }
        e.mem.u32(blocks.addr())
    })
}

// Translated from 00d2c450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::onRowFull` (Xbox PDB), `thiscall`, `RET 8` (`class`, `block`): a row that
/// has reached its limit takes `block` and gives surplus blocks back. With the limit zero the
/// block goes straight to the allocator's slot `+8` (`blockFree`) with the class's size.
/// Otherwise, while the row holds more than half the limit (the limit halved the way `CDQ`
/// and `SAR` do, towards zero), up to four blocks (no more than the surplus) come off the row
/// and go to the allocator's slot `+0x1c` (`blockFreeBatch`); then `block` is pushed.
pub fn hk_thread_memory_on_row_full(e: &mut Engine, this: Ptr, class: i32, block: u32) {
    let base = this.addr();
    let block_size = e.mem.u32(class_size_field(base, class));
    let limit = e.mem.i32(base + ROW_LIMIT);
    let allocator = e.mem.u32(base + ALLOCATOR);
    if limit == 0 {
        e.vcall(allocator, SLOT_BLOCK_FREE, &args![block, block_size]);
        return;
    }
    let half = limit.wrapping_sub(limit >> 31) >> 1;
    let mut count = e.mem.i32(row_field(base, class, ROW_COUNT));
    while count > half {
        let surplus = count.wrapping_sub(half);
        let batch = surplus.min(4);
        e.with_stack(0x10, |e, blocks| {
            let mut index = 0i32;
            while index < batch {
                let popped = pop_block(e, base, class);
                e.mem.set_u32(word_at(blocks.addr(), index), popped);
                index += 1;
            }
            count = count.wrapping_sub(batch);
            e.vcall(
                allocator,
                SLOT_BLOCK_FREE_BATCH,
                &args![blocks, batch, block_size],
            );
        });
    }
    push_block(e, base, class, block);
}

// Translated from 00d2c520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `class`): empties the row of a size class.
/// While its count (read once, then counted down locally) is above zero, up to four blocks
/// come off the front of the list (a null head stores a null) and go to the allocator's slot
/// `+0x1c` (`blockFreeBatch`) with the class's block size.
pub fn fn_00d2c520(e: &mut Engine, this: Ptr, class: i32) {
    let base = this.addr();
    let block_size = e.mem.u32(class_size_field(base, class));
    let mut count = e.mem.i32(row_field(base, class, ROW_COUNT));
    while count > 0 {
        let batch = count.min(4);
        let allocator = e.mem.u32(base + ALLOCATOR);
        e.with_stack(0x10, |e, blocks| {
            let mut index = 0i32;
            while index < batch {
                let popped = pop_block(e, base, class);
                e.mem.set_u32(word_at(blocks.addr(), index), popped);
                index += 1;
            }
            e.vcall(
                allocator,
                SLOT_BLOCK_FREE_BATCH,
                &args![blocks, batch, block_size],
            );
        });
        count = count.wrapping_sub(batch);
    }
}

// Translated from 00d2c5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::blockAlloc` (Xbox PDB), `thiscall`, `RET 4` (`size`): a size over 0x2000
/// is passed to the allocator's slot `+4` unchanged. Otherwise the size class comes from the
/// tables (`class_of`) and the first block of the class's list is taken (the count goes down,
/// the head moves to the block's first word); an empty list is refilled
/// (`block_alloc_refill`, the exe's code at `00d2c3c0`).
pub fn hk_thread_memory_block_alloc(e: &mut Engine, this: Ptr, size: i32) -> u32 {
    // `NV_HKMEM_CHECK` (diagnostic only, reads memory): the lists before and after.
    hkmem_check::at_op(e, "blockAlloc", false, this.addr(), size as u32, 0);
    let block = block_alloc(e, this, size);
    hkmem_check::at_op(e, "blockAlloc", true, this.addr(), size as u32, block);
    block
}

/// The body of `blockAlloc` (`00d2c5a0`).
fn block_alloc(e: &mut Engine, this: Ptr, size: i32) -> u32 {
    let base = this.addr();
    if size > LARGEST_ROW_SIZE {
        let allocator = e.mem.u32(base + ALLOCATOR);
        return e.vcall(allocator, SLOT_BLOCK_ALLOC, &args![size]).u32();
    }
    let class = class_of(e, base, size);
    if e.mem.u32(row_field(base, class, ROW_HEAD)) == 0 {
        return block_alloc_refill(e, base, class);
    }
    pop_block(e, base, class)
}

// Translated from 00d2c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::blockFree` (Xbox PDB), `thiscall`, `RET 8` (`block`, `size`): a null block
/// is ignored. A size over 0x2000 goes to the allocator's slot `+8` unchanged. Otherwise, when
/// the size class's count is below the row limit (signed) the block is pushed on the class's
/// list, and when it is not the block goes to `onRowFull` (`00d2c450`).
pub fn hk_thread_memory_block_free(e: &mut Engine, this: Ptr, block: u32, size: i32) {
    // `NV_HKMEM_CHECK` (diagnostic only, reads memory): the lists before and after.
    hkmem_check::at_op(e, "blockFree", false, this.addr(), block, size as u32);
    block_free(e, this, block, size);
    hkmem_check::at_op(e, "blockFree", true, this.addr(), block, size as u32);
}

/// The body of `blockFree` (`00d2c600`).
fn block_free(e: &mut Engine, this: Ptr, block: u32, size: i32) {
    if block == 0 {
        return;
    }
    let base = this.addr();
    if size > LARGEST_ROW_SIZE {
        let allocator = e.mem.u32(base + ALLOCATOR);
        e.vcall(allocator, SLOT_BLOCK_FREE, &args![block, size]);
        return;
    }
    let class = class_of(e, base, size);
    let count = e.mem.i32(row_field(base, class, ROW_COUNT));
    if count >= e.mem.i32(base + ROW_LIMIT) {
        hk_thread_memory_on_row_full(e, this, class, block);
        return;
    }
    push_block(e, base, class, block);
}

// Translated from 00d2c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::canAllocTotal` (Xbox PDB), `thiscall`, one stack word (`size`): tail-calls
/// the allocator's slot `+0x24` with it and returns its answer.
pub fn hk_thread_memory_can_alloc_total(e: &mut Engine, this: Ptr, size: u32) -> u32 {
    let allocator = e.mem.u32(this.addr() + ALLOCATOR);
    e.vcall(allocator, SLOT_CAN_ALLOC_TOTAL, &args![size]).u32()
}

// Translated from 00d2c680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::getAllocatedSize` (Xbox PDB), `thiscall`, two stack words (`block`,
/// `size`): a size up to 0x2000 is replaced by its size class's block size, then the
/// allocator's slot `+0x2c` is tail-called with the block and that size; its answer is
/// returned.
pub fn hk_thread_memory_get_allocated_size(
    e: &mut Engine,
    this: Ptr,
    block: u32,
    size: i32,
) -> u32 {
    let base = this.addr();
    let mut size = size;
    if size <= LARGEST_ROW_SIZE {
        let class = class_of(e, base, size);
        size = e.mem.i32(class_size_field(base, class));
    }
    let allocator = e.mem.u32(base + ALLOCATOR);
    e.vcall(allocator, SLOT_GET_ALLOCATED_SIZE, &args![block, size])
        .u32()
}

// Translated from 00d2c6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::bufRealloc` (Xbox PDB), `thiscall`, `RET 0xc` (`block`, `old_size`, `size`:
/// a pointer to the wanted size): gets a block of the wanted size from the table's slot `+4`
/// (`blockAlloc`), copies the smaller (signed) of the wanted size (read again after the
/// allocation) and `old_size` bytes into it with `hkMemUtil::memCpy` (`00d2caa0`), gives the old
/// block back through slot `+8` (`blockFree`, with `old_size`) and returns the new block.
pub fn hk_thread_memory_buf_realloc(
    e: &mut Engine,
    this: Ptr,
    block: u32,
    old_size: i32,
    size: Ptr,
) -> u32 {
    let wanted = e.mem.i32(size.addr());
    let new_block = e.vcall(this.addr(), SLOT_BLOCK_ALLOC, &args![wanted]).u32();
    let wanted = e.mem.i32(size.addr());
    let copied = wanted.min(old_size);
    e.call(MEM_CPY, &args![new_block, block, copied]);
    e.vcall(this.addr(), SLOT_BLOCK_FREE, &args![block, old_size]);
    new_block
}

// Translated from 00d2c7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThreadMemory::releaseCachedMemory` (Xbox PDB), `thiscall`, no stack words: empties every
/// row whose count is not zero (`00d2c520`), from size class 16 down to 0.
pub fn hk_thread_memory_release_cached_memory(e: &mut Engine, this: Ptr) {
    // `NV_HKMEM_CHECK` (diagnostic only, reads memory): the lists before and after.
    hkmem_check::at_op(e, "releaseCachedMemory", false, this.addr(), 0, 0);
    release_cached_memory(e, this);
    hkmem_check::at_op(e, "releaseCachedMemory", true, this.addr(), 0, 0);
}

/// The body of `releaseCachedMemory` (`00d2c7a0`).
fn release_cached_memory(e: &mut Engine, this: Ptr) {
    let mut class = 0x10i32;
    while class >= 0 {
        if e.mem.u32(row_field(this.addr(), class, ROW_COUNT)) != 0 {
            fn_00d2c520(e, this, class);
        }
        class -= 1;
    }
}

/// `NV_HKMEM_CHECK`: diagnostic only, not exe behaviour. Reads, never writes, game memory.
///
/// Walks every size-class list of every thread memory seen so far and stops the run (a panic
/// whose message starts `hkmem check:`, after `hkmem:` lines with the details) at the first
/// bad link: a link that is not a 16-byte aligned, mapped address, a list longer or shorter
/// than its count, or a free block whose first word (its link) changed since the last clean
/// check while its list's head and count did not (nothing of the game's may write a block on
/// a free list). Between checks it keeps each list as it found it.
///
/// - Any value (except `0`): a check as `blockAlloc` (`00d2c5a0`), `blockFree` (`00d2c600`)
///   and `releaseCachedMemory` (`00d2c7a0`) start and end. The window is between two of those.
/// - `calls`: also at every call through `Engine::call` (entered and returned), which names
///   the function that was running when a block changed (slow).
pub mod hkmem_check {
    use super::{row_field, ROW_COUNT, ROW_HEAD};
    use crate::prelude::*;
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::fmt::Write as _;

    /// How the check runs.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Mode {
        Off,
        /// At the starts and ends of the allocator's own functions.
        Allocs,
        /// Also between any two calls.
        Calls,
    }

    /// The size classes a thread memory has rows for (`releaseCachedMemory` goes over 0 to 16).
    const CLASSES: i32 = 17;
    /// Where a walk gives up (a cycle) when the count is no guide.
    const MAX_WALK: usize = 1 << 16;

    /// A point the check ran at.
    #[derive(Clone, Copy, Debug)]
    enum Event {
        /// Start or end of an allocator function: its name, `this` and its first two words.
        Op(&'static str, bool, u32, u32, u32),
        /// A call through `Engine::call` entered (`false`) or returned (`true`).
        Call(u32, bool),
    }

    impl std::fmt::Display for Event {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match *self {
                Event::Op(name, end, this, a, b) => write!(
                    f,
                    "{} of {name}({this:08x},{a:08x},{b:08x})",
                    if end { "end" } else { "start" }
                ),
                Event::Call(addr, true) => write!(f, "return from {addr:08x}"),
                Event::Call(addr, false) => write!(f, "entry to {addr:08x}"),
            }
        }
    }

    /// One list as the last clean check left it: head, count and each block with its link.
    #[derive(Default, Clone)]
    struct Row {
        head: u32,
        count: i32,
        links: Vec<(u32, u32)>,
    }

    #[derive(Default)]
    struct State {
        /// The thread memories seen, each with its 17 lists.
        memories: Vec<(u32, Vec<Row>)>,
        /// Who freed each block now on a list (the call stack when `push_block` put it there).
        freed_by: HashMap<u32, Vec<u32>>,
        /// The clean checks so far, the last one's event and the function then running.
        checks: u64,
        last: Option<(Event, u32)>,
    }

    thread_local! {
        static STATE: RefCell<State> = RefCell::new(State::default());
        static FORCED: Cell<Option<Mode>> = const { Cell::new(None) };
    }

    /// The mode from `NV_HKMEM_CHECK` (read once), unless a test forced one on this thread.
    pub fn mode() -> Mode {
        if let Some(mode) = FORCED.with(Cell::get) {
            return mode;
        }
        static MODE: std::sync::OnceLock<Mode> = std::sync::OnceLock::new();
        *MODE.get_or_init(|| match std::env::var("NV_HKMEM_CHECK") {
            Err(_) => Mode::Off,
            Ok(v) if v.is_empty() || v == "0" => Mode::Off,
            Ok(v) if v.eq_ignore_ascii_case("calls") => Mode::Calls,
            Ok(_) => Mode::Allocs,
        })
    }

    /// Sets the mode for this thread and forgets what was seen (tests).
    pub fn force(mode: Mode) {
        FORCED.with(|forced| forced.set(Some(mode)));
        STATE.with(|state| *state.borrow_mut() = State::default());
    }

    /// Notes who freed `block` (called as `push_block` puts it on a list).
    pub(super) fn note_freed(e: &Engine, block: u32) {
        if mode() == Mode::Off {
            return;
        }
        STATE.with(|state| {
            state
                .borrow_mut()
                .freed_by
                .insert(block, e.call_stack.clone())
        });
    }

    thread_local! {
        /// The last contact points freed (`removeContactPointImpl`, 00cfd320): manager, id and
        /// the call stack then, newest last.
        static REMOVED: RefCell<std::collections::VecDeque<(u32, u32, Vec<u32>)>> =
            const { RefCell::new(std::collections::VecDeque::new()) };
    }
    /// How many removals [`note_contact_removed`] keeps.
    const REMOVED_KEEP: usize = 512;

    /// Notes that the contact manager `mgr` freed its contact point `id` (called as
    /// `removeContactPointImpl` starts).
    pub fn note_contact_removed(e: &Engine, mgr: u32, id: u32) {
        if mode() == Mode::Off {
            return;
        }
        REMOVED.with(|removed| {
            let mut removed = removed.borrow_mut();
            if removed.len() >= REMOVED_KEEP {
                removed.pop_front();
            }
            removed.push_back((mgr, id, e.call_stack.clone()));
        });
    }

    /// Checks the id of a process-output entry before `processContactImpl` (00cfd470) copies
    /// the entry's contact point into the atom at `atom + 0x30 + 0x20 * table[id]`: the id must
    /// be inside the manager's id table (data `+0x20`, size `+0x24`) and its slot must be a
    /// point in use (not `0xff`, below the atom's point count). Otherwise the copy lands outside
    /// the points (with `0xff`, 0x2010 bytes past the atom), so the run stops here instead.
    pub fn check_contact_id(e: &mut Engine, mgr: u32, id: u32, entry: u32) {
        if mode() == Mode::Off {
            return;
        }
        let table = e.mem.u32(mgr + 0x20);
        let size = e.mem.u32(mgr + 0x24);
        let atom = e.mem.u32(mgr + 0x3c);
        let count = u32::from(e.mem.u16(atom + 4));
        let reserved = u32::from(e.mem.u16(atom + 6));
        let slot = if id < size {
            Some(u32::from(e.mem.u8(table.wrapping_add(id))))
        } else {
            None
        };
        let why = match slot {
            None => format!("id {id:#x} is past the id table (size {size:#x})"),
            Some(0xff) => format!("id {id:#x} was freed (its slot is 0xff)"),
            Some(slot) if slot >= count => {
                format!("id {id:#x} maps to point {slot:#x}, past the atom's {count:#x} points")
            }
            Some(_) => return,
        };
        e.call_hook = None;
        let target = (slot.unwrap_or(0xff) << 5)
            .wrapping_add(atom)
            .wrapping_add(0x30);
        let mut out = String::new();
        let _ = writeln!(
            out,
            "hkmem: contact manager {mgr:08x}: output entry {entry:08x}: {why}; atom {atom:08x} \
             ({count:#x} points, {reserved:#x} reserved), the copy would write {target:08x}"
        );
        let _ = writeln!(
            out,
            "hkmem: call stack (innermost first): {}",
            stack_text(&e.call_stack)
        );
        let words: Vec<String> = (0..0xc)
            .map(|w| format!("{:08x}", e.mem.u32(entry + 4 * w)))
            .collect();
        let _ = writeln!(out, "hkmem: the entry's words: {}", words.join(" "));
        REMOVED.with(|removed| {
            let removed = removed.borrow();
            let mut any = false;
            for (m, i, stack) in removed.iter().filter(|r| r.0 == mgr && r.1 == id) {
                any = true;
                let _ = writeln!(
                    out,
                    "hkmem: {m:08x} freed id {i:#x} from: {}",
                    stack_text(stack)
                );
            }
            if !any {
                let _ = writeln!(
                    out,
                    "hkmem: no removal of id {id:#x} by {mgr:08x} in the last {REMOVED_KEEP}"
                );
            }
        });
        print!("{out}");
        panic!(
            "hkmem check: contact manager {mgr:08x}: {why}; the copy would write {target:08x} \
             (NV_HKMEM_CHECK; more on the hkmem: lines)"
        );
    }

    /// Checks at the start or end (`end`) of the allocator function `name` on `this`.
    pub(super) fn at_op(e: &mut Engine, name: &'static str, end: bool, this: u32, a: u32, b: u32) {
        let mode = mode();
        if mode == Mode::Off {
            return;
        }
        let known = STATE.with(|state| state.borrow().memories.iter().any(|(m, _)| *m == this));
        if !known {
            STATE.with(|state| {
                state
                    .borrow_mut()
                    .memories
                    .push((this, vec![Row::default(); CLASSES as usize]))
            });
            if mode == Mode::Calls && e.call_hook.is_none() {
                e.call_hook = Some(on_call);
            }
        }
        check(e, Event::Op(name, end, this, a, b));
    }

    /// `Engine::call_hook` in `calls` mode.
    fn on_call(e: &mut Engine, addr: u32, returning: bool) {
        check(e, Event::Call(addr, returning));
    }

    /// The function running after `event`: the callee once entered, the caller once returned.
    fn running_after(e: &Engine, event: Event) -> u32 {
        match event {
            Event::Call(addr, false) => addr,
            _ => e.call_stack.last().copied().unwrap_or(0),
        }
    }

    /// A word and its float, for the report.
    fn word_text(word: u32) -> String {
        format!("{word:08x} (f32 {:e})", f32::from_bits(word))
    }

    /// The words of `[at, at + len)` that are mapped, four to a line.
    fn dump(e: &Engine, out: &mut String, label: &str, at: u32, len: u32) {
        let _ = writeln!(out, "hkmem:   {label} {at:08x}:");
        for line in (0..len).step_by(16) {
            let mut text = String::new();
            for word in 0..4 {
                let addr = at.wrapping_add(line + 4 * word);
                if e.mem.is_mapped(addr) {
                    let value = e.mem.u32(addr);
                    let _ = write!(text, " {value:08x}({:+.4e})", f32::from_bits(value));
                } else {
                    text.push_str(" unmapped");
                }
            }
            let _ = writeln!(out, "hkmem:     +{line:03x}{text}");
        }
    }

    /// Walks every list; stops the run at the first bad link, else keeps the lists as found.
    fn check(e: &mut Engine, event: Event) {
        let mut memories: Vec<(u32, Vec<Row>)> =
            STATE.with(|state| std::mem::take(&mut state.borrow_mut().memories));
        for (this, rows) in &mut memories {
            let this = *this;
            for (class, old) in (0..CLASSES).zip(rows.iter_mut()) {
                let head = e.mem.u32(row_field(this, class, ROW_HEAD));
                let count = e.mem.i32(row_field(this, class, ROW_COUNT));
                if old.head == head && old.count == count {
                    // Unchanged row: the same blocks, each link as it was (no allocation).
                    for (index, &(block, next)) in old.links.iter().enumerate() {
                        let links = &old.links[..index];
                        if !e.mem.is_mapped(block) || !e.mem.is_mapped(block.wrapping_add(3)) {
                            stop(
                                e,
                                event,
                                this,
                                class,
                                count,
                                old,
                                links,
                                block,
                                "is not mapped",
                            );
                        }
                        if e.mem.u32(block) != next {
                            stop(e, event, this, class, count, old, links, block, "");
                        }
                    }
                    continue;
                }
                // The head or the count moved (a block was taken or given back): walk it anew.
                let mut links = Vec::new();
                let mut block = head;
                while block != 0 {
                    let index = links.len();
                    let bad = if block & 0xf != 0 {
                        Some("is not 16-byte aligned")
                    } else if !e.mem.is_mapped(block) || !e.mem.is_mapped(block.wrapping_add(3)) {
                        Some("is not mapped")
                    } else if index >= MAX_WALK || index as i64 >= i64::from(count) {
                        Some("is past the list's count (a cycle or a lost count)")
                    } else {
                        None
                    };
                    if let Some(why) = bad {
                        stop(e, event, this, class, count, old, &links, block, why);
                    }
                    let next = e.mem.u32(block);
                    links.push((block, next));
                    block = next;
                }
                if links.len() as i64 != i64::from(count) {
                    stop(
                        e,
                        event,
                        this,
                        class,
                        count,
                        old,
                        &links,
                        0,
                        "ends before the list's count",
                    );
                }
                *old = Row { head, count, links };
            }
        }
        let running = running_after(e, event);
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.memories = memories;
            state.checks += 1;
            state.last = Some((event, running));
            // Blocks no longer on any list were taken again: forget who freed them.
            if state.freed_by.len() > 4096 {
                let listed: std::collections::HashSet<u32> = state
                    .memories
                    .iter()
                    .flat_map(|(_, rows)| rows.iter().flat_map(|r| r.links.iter().map(|l| l.0)))
                    .collect();
                state.freed_by.retain(|block, _| listed.contains(block));
            }
        });
    }

    /// Prints what was found and stops the run. `why` empty: `block`'s link changed.
    #[allow(clippy::too_many_arguments)]
    fn stop(
        e: &mut Engine,
        event: Event,
        this: u32,
        class: i32,
        count: i32,
        old: &Row,
        links: &[(u32, u32)],
        block: u32,
        why: &str,
    ) -> ! {
        // The check must not run again while the run unwinds.
        e.call_hook = None;
        let size = e.mem.u32(super::class_size_field(this, class));
        let (checks, last, freed_by) = STATE.with(|state| {
            let state = state.borrow();
            let freed_by = |b: u32| state.freed_by.get(&b).cloned();
            let blocks: Vec<(u32, Option<Vec<u32>>)> = links
                .iter()
                .map(|l| l.0)
                .chain(std::iter::once(block))
                .filter(|b| *b != 0)
                .map(|b| (b, freed_by(b)))
                .collect();
            (state.checks, state.last, blocks)
        });
        let mut out = String::new();
        let summary = if why.is_empty() {
            let was = old
                .links
                .iter()
                .find(|l| l.0 == block)
                .map(|l| l.1)
                .unwrap_or(0);
            let now = e.mem.u32(block);
            format!(
                "thread memory {this:08x} class {class} ({size:#x}-byte blocks): free block \
                 {block:08x} (link {} of {count}) was written: its link was {}, now {}",
                links.len(),
                word_text(was),
                word_text(now)
            )
        } else if block == 0 {
            format!(
                "thread memory {this:08x} class {class} ({size:#x}-byte blocks): the list {why} \
                 ({} links, count {count})",
                links.len()
            )
        } else {
            let from = links
                .last()
                .map(|l| format!("the link in block {:08x}", l.0))
                .unwrap_or_else(|| format!("the head (+{:#x})", ROW_HEAD + 8 * class as u32));
            format!(
                "thread memory {this:08x} class {class} ({size:#x}-byte blocks): {from} \
                 (link {} of {count}) is {}, which {why}",
                links.len(),
                word_text(block)
            )
        };
        let _ = writeln!(out, "hkmem: {summary}");
        let _ = writeln!(
            out,
            "hkmem: found at {event}; call stack (innermost first): {}",
            stack_text(&e.call_stack)
        );
        match last {
            Some((last, running)) => {
                let _ = writeln!(
                    out,
                    "hkmem: last clean check (#{checks}) at {last}; between the two, \
                     {running:08x} was running{}",
                    if matches!(event, Event::Call(..)) && matches!(last, Event::Call(..)) {
                        " (in `calls` mode: it, or a function it calls directly, wrote)"
                    } else {
                        ""
                    }
                );
            }
            None => {
                let _ = writeln!(out, "hkmem: no clean check before this one");
            }
        }
        let _ = writeln!(
            out,
            "hkmem: the list at the last clean check: head {:08x}, count {}: {}",
            old.head,
            old.count,
            old.links
                .iter()
                .map(|(b, n)| format!("{b:08x}->{n:08x}"))
                .collect::<Vec<_>>()
                .join(" ")
        );
        for (b, stack) in &freed_by {
            let _ = writeln!(
                out,
                "hkmem: block {b:08x} was put on the list by: {}",
                stack.as_ref().map(|s| stack_text(s)).unwrap_or_else(|| {
                    "(not seen: a refill, or freed before the check began)".into()
                })
            );
        }
        let bad_block = if why.is_empty() {
            Some(block)
        } else {
            links.last().map(|l| l.0)
        };
        if let Some(b) = bad_block {
            if e.mem.is_mapped(b) {
                dump(e, &mut out, "the block's first bytes", b, 0x40);
                dump(
                    e,
                    &mut out,
                    "the bytes before it (the end of the block below)",
                    b.wrapping_sub(0x40),
                    0x40,
                );
            }
        }
        print!("{out}");
        let window = match last {
            Some((last, running)) => {
                format!("found at {event}, clean at {last}; {running:08x} ran between")
            }
            None => format!("found at {event}, first check"),
        };
        panic!("hkmem check: {summary}; {window} (NV_HKMEM_CHECK; more on the hkmem: lines)");
    }

    /// A call stack, innermost first (`inner < ... < outer`, as the run's stop line has it).
    fn stack_text(stack: &[u32]) -> String {
        stack
            .iter()
            .rev()
            .map(|a| format!("{a:08x}"))
            .collect::<Vec<_>>()
            .join(" < ")
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2c100, fn_00d2c100(Ptr)),
        entry!(0x00d2c3a0, fn_00d2c3a0(Ptr, u32, u32)),
        entry!(0x00d2c720, hk_thread_memory_hk_thread_memory(Ptr) -> Ptr),
        entry!(0x00d2c000, hk_thread_memory_buf_alloc(Ptr, Ptr) -> u32),
        entry!(0x00d2c020, hk_thread_memory_buf_free(Ptr, u32, u32)),
        entry!(
            0x00d2c030,
            hk_thread_memory_block_alloc_batch(Ptr, Ptr, i32, i32)
        ),
        entry!(
            0x00d2c090,
            hk_thread_memory_block_free_batch(Ptr, Ptr, i32, i32)
        ),
        entry!(0x00d2c0f0, hk_thread_memory_get_memory_statistics(Ptr, u32)),
        entry!(0x00d2c350, hk_thread_memory_get_cached_size_unchecked(Ptr) -> i32),
        entry!(0x00d2c450, hk_thread_memory_on_row_full(Ptr, i32, u32)),
        entry!(0x00d2c520, fn_00d2c520(Ptr, i32)),
        entry!(0x00d2c5a0, hk_thread_memory_block_alloc(Ptr, i32) -> u32),
        entry!(0x00d2c600, hk_thread_memory_block_free(Ptr, u32, i32)),
        entry!(0x00d2c670, hk_thread_memory_can_alloc_total(Ptr, u32) -> u32),
        entry!(0x00d2c680, hk_thread_memory_get_allocated_size(Ptr, u32, i32) -> u32),
        entry!(0x00d2c6d0, hk_thread_memory_buf_realloc(Ptr, u32, i32, Ptr) -> u32),
        entry!(0x00d2c7a0, hk_thread_memory_release_cached_memory(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_size_class_ladder_matches_the_exes_comparisons() {
        assert_eq!(size_class(0), Some(1));
        assert_eq!(size_class(0x10), Some(1));
        assert_eq!(size_class(0x11), Some(2));
        assert_eq!(size_class(0x50), Some(5));
        assert_eq!(size_class(0x280), Some(12));
        assert_eq!(size_class(0x2000), Some(16));
        assert_eq!(size_class(0x2001), None);
    }

    #[test]
    fn the_tables_are_filled_from_the_sizes() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x138);
        e.mem.write(object, &[0xcd; 0x138]);
        e.call(0x00d2_c100, &args![object]);
        let classes = e.mem.bytes(object + 0xd8, 0x29);
        assert_eq!(classes[0], 1);
        assert_eq!(classes[1], 1);
        assert_eq!(classes[2], 2);
        assert_eq!(classes[5], 5);
        assert_eq!(classes[0x28], 12);
        assert_eq!(e.mem.u8(object + 0x101), 0xcd);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(object + 0x104 + i * 4)).collect();
        assert_eq!(words, vec![13, 14, 15, 15, 16, 16, 16, 16]);
        assert_eq!(e.mem.u32(object + 0x124), 0xcdcd_cdcd);
        // The size of each class: the last (largest) size stored for it.
        assert_eq!(e.mem.u32(object + 0x94 + 4), 0x10);
        assert_eq!(e.mem.u32(object + 0x94 + 12 * 4), 0x280);
        assert_eq!(e.mem.u32(object + 0x94 + 16 * 4), 0x2000);
        assert_eq!(e.mem.u32(object + 0x94 + 13 * 4), 0x400);
        assert_eq!(e.mem.u32(object + 0x94 + 15 * 4), 0x1000);
    }

    #[test]
    fn the_setter_stores_both_words_or_clears_the_first() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.mem.write(object, &[0xcd; 0x20]);
        e.call(0x00d2_c3a0, &args![object, 0x77u32, 0x88u32]);
        assert_eq!(e.mem.u32(object + 4), 0x77);
        assert_eq!(e.mem.u32(object + 8), 0x88);
        e.call(0x00d2_c3a0, &args![object, 0u32, 8u32]);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 8), 0x88);
    }

    #[test]
    fn the_constructor_clears_stores_the_table_and_returns_this() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x138);
        e.mem.write(object, &[0xcd; 0x138]);
        assert_eq!(e.call(0x00d2_c720, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 4), 0);
        // Not named by the constructor: +8.
        assert_eq!(e.mem.u32(object + 8), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        // +0x98 is the class-1 size, written after the clear.
        assert_eq!(e.mem.u32(object + 0x98), 0x10);
        assert_eq!(e.mem.u32(object + 0x90), 0);
        assert_eq!(e.mem.u8(object + 0xd8), 1);
        // Past the second table nothing is written.
        assert_eq!(e.mem.u32(object + 0x124), 0xcdcd_cdcd);
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const THREAD_MEMORY_SIZE: u32 = 0x138;
    const ALLOCATOR_TABLE: u32 = 0x0060_0000;
    const DOUBLES: u32 = 0x0060_1000;
    const BLOCKS: u32 = 0x0090_0000;
    const NEW_BLOCK: u32 = 0xa110;

    /// The address of the double standing in for the allocator's slot at byte offset `slot`.
    fn double_at(slot: u32) -> u32 {
        DOUBLES + slot
    }

    /// The calls the allocator's slot `slot` got: the argument words, with the first words of the
    /// block buffer appended for slot `+0x1c`.
    fn calls_to(calls: &Calls, slot: u32) -> Vec<Vec<u32>> {
        calls
            .borrow()
            .iter()
            .filter(|(addr, _)| *addr == double_at(slot))
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A thread memory (tables filled, its own table holding the real `blockAlloc` and
    /// `blockFree`) in front of an allocator whose slots are doubles that record their calls.
    fn fixture(e: &mut Engine) -> (u32, u32, Calls) {
        e.map(BLOCKS, 0x1_0000);
        e.put_vtable(VTABLE, &[0, 0x00d2_c5a0, 0x00d2_c600]);
        let this = e.mem.alloc(THREAD_MEMORY_SIZE);
        let allocator = e.mem.alloc(8);
        let slots: Vec<u32> = (0..12).map(|slot| double_at(4 * slot)).collect();
        e.put_vtable(ALLOCATOR_TABLE, &slots);
        e.mem.set_u32(allocator, ALLOCATOR_TABLE);
        e.mem.set_u32(this, VTABLE);
        e.mem.set_u32(this + 4, allocator);
        fn_00d2c100(e, Ptr::new(this));
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for slot in [4u32, 8, 0x18, 0x1c, 0x24, 0x28, 0x2c] {
            let calls = calls.clone();
            e.register_double(double_at(slot), move |e, words| {
                let mut recorded = words.to_vec();
                match slot {
                    0x18 => {
                        for index in 0..words[2].min(8) {
                            e.mem
                                .set_u32(words[1] + 4 * index, BLOCKS + 0x8000 + 0x100 * index);
                        }
                    }
                    0x1c => {
                        for index in 0..words[2].min(8) {
                            recorded.push(e.mem.u32(words[1] + 4 * index));
                        }
                    }
                    _ => {}
                }
                calls.borrow_mut().push((double_at(slot), recorded));
                let eax = match slot {
                    4 => NEW_BLOCK,
                    0x24 => 1,
                    0x2c => 0x77,
                    _ => 0,
                };
                Ret {
                    eax,
                    ..Ret::default()
                }
            });
        }
        (this, allocator, calls)
    }

    /// Makes the list of size class `class` hold `count` blocks `BLOCKS + 0x10 * n`, in order.
    fn put_list(e: &mut Engine, this: u32, class: u32, count: u32) {
        for n in 0..count {
            let next = if n + 1 < count {
                BLOCKS + 0x10 * (n + 1)
            } else {
                0
            };
            e.mem.set_u32(BLOCKS + 0x10 * n, next);
        }
        e.mem
            .set_u32(this + 0xc + 8 * class, if count > 0 { BLOCKS } else { 0 });
        e.mem.set_u32(this + 0x10 + 8 * class, count);
    }

    fn row(e: &Engine, this: u32, class: u32) -> (u32, u32) {
        (
            e.mem.u32(this + 0xc + 8 * class),
            e.mem.u32(this + 0x10 + 8 * class),
        )
    }

    #[test]
    fn buf_alloc_reads_the_size_and_allocates_it() {
        let mut e = Engine::new();
        let (this, allocator, calls) = fixture(&mut e);
        let wanted = e.mem.alloc(4);
        e.mem.set_u32(wanted, 0x30);
        let block = e.call(0x00d2_c000, &args![this, wanted]).u32();
        // Empty row, limit zero: the allocator's slot 4 gets the class's block size.
        assert_eq!(block, NEW_BLOCK);
        assert_eq!(calls_to(&calls, 4), vec![vec![allocator, 0x30]]);
    }

    #[test]
    fn buf_free_forwards_both_words() {
        let mut e = Engine::new();
        let (this, allocator, calls) = fixture(&mut e);
        e.call(0x00d2_c020, &args![this, 0x1234u32, 0x4000u32]);
        assert_eq!(calls_to(&calls, 8), vec![vec![allocator, 0x1234, 0x4000]]);
    }

    #[test]
    fn block_alloc_batch_allocates_each_block_or_hands_a_big_size_over() {
        let mut e = Engine::new();
        let (this, allocator, calls) = fixture(&mut e);
        let out = e.mem.alloc(16);
        e.call(0x00d2_c030, &args![this, out, 3i32, 0x20i32]);
        for index in 0..3 {
            assert_eq!(e.mem.u32(out + 4 * index), NEW_BLOCK);
        }
        assert_eq!(e.mem.u32(out + 12), 0);
        assert_eq!(calls_to(&calls, 4).len(), 3);
        // Nothing for a count of zero or less.
        e.call(0x00d2_c030, &args![this, out, 0i32, 0x20i32]);
        e.call(0x00d2_c030, &args![this, out, -2i32, 0x20i32]);
        assert_eq!(calls_to(&calls, 4).len(), 3);
        // A size over 0x2000 goes to slot 0x18 whole.
        e.call(0x00d2_c030, &args![this, out, 2i32, 0x2001i32]);
        assert_eq!(
            calls_to(&calls, 0x18),
            vec![vec![allocator, out, 2, 0x2001]]
        );
        assert_eq!(calls_to(&calls, 4).len(), 3);
    }

    #[test]
    fn block_free_batch_frees_each_block_or_hands_a_big_size_over() {
        let mut e = Engine::new();
        let (this, allocator, calls) = fixture(&mut e);
        let blocks = e.mem.alloc(8);
        e.mem.set_u32(blocks, BLOCKS + 0x40);
        e.mem.set_u32(blocks + 4, BLOCKS + 0x50);
        e.call(0x00d2_c090, &args![this, blocks, 2i32, 0x20i32]);
        // The row limit is zero, so each block reaches the allocator through the row-full path.
        assert_eq!(
            calls_to(&calls, 8),
            vec![
                vec![allocator, BLOCKS + 0x40, 0x20],
                vec![allocator, BLOCKS + 0x50, 0x20]
            ]
        );
        e.call(0x00d2_c090, &args![this, blocks, 0i32, 0x20i32]);
        assert_eq!(calls_to(&calls, 8).len(), 2);
        e.call(0x00d2_c090, &args![this, blocks, 2i32, 0x3000i32]);
        assert_eq!(
            calls_to(&calls, 0x1c),
            vec![vec![
                allocator,
                blocks,
                2,
                0x3000,
                BLOCKS + 0x40,
                BLOCKS + 0x50
            ]]
        );
    }

    #[test]
    fn get_memory_statistics_tail_calls_the_allocator() {
        let mut e = Engine::new();
        let (this, allocator, calls) = fixture(&mut e);
        e.call(0x00d2_c0f0, &args![this, 0x5555u32]);
        assert_eq!(calls_to(&calls, 0x28), vec![vec![allocator, 0x5555]]);
    }

    #[test]
    fn get_cached_size_sums_count_times_size_over_the_classes() {
        let mut e = Engine::new();
        let (this, _, _) = fixture(&mut e);
        let mut expected = 0i32;
        for class in 0..=16u32 {
            let count = 3 * class + 1;
            e.mem.set_u32(this + 0x10 + 8 * class, count);
            expected += (count * e.mem.u32(this + 0x94 + 4 * class)) as i32;
        }
        assert_eq!(
            e.call(0x00d2_c350, &args![this]).i32(),
            expected,
            "classes 0 to 16"
        );
        assert!(expected > 0);
    }

    #[test]
    fn on_row_full_frees_the_surplus_in_batches_and_keeps_the_block() {
        let mut e = Engine::new();
        let (this, allocator, calls) = fixture(&mut e);
        // Limit zero: the block goes straight to the allocator with the class's size.
        e.call(0x00d2_c450, &args![this, 3i32, BLOCKS + 0x200]);
        assert_eq!(
            calls_to(&calls, 8),
            vec![vec![allocator, BLOCKS + 0x200, 0x30]]
        );
        // Limit 8, eight blocks: half is 4, so one batch of four goes and the block is pushed.
        e.mem.set_u32(this + 8, 8);
        put_list(&mut e, this, 3, 8);
        e.call(0x00d2_c450, &args![this, 3i32, BLOCKS + 0x200]);
        let batches = calls_to(&calls, 0x1c);
        assert_eq!(batches.len(), 1);
        assert_eq!(
            batches[0][2..],
            [4, 0x30, BLOCKS, BLOCKS + 0x10, BLOCKS + 0x20, BLOCKS + 0x30]
        );
        assert_eq!(row(&e, this, 3), (BLOCKS + 0x200, 5));
        assert_eq!(e.mem.u32(BLOCKS + 0x200), BLOCKS + 0x40);
        // Limit 20, twenty blocks: half is 10; batches of 4, 4 and 2.
        calls.borrow_mut().clear();
        e.mem.set_u32(this + 8, 20);
        put_list(&mut e, this, 3, 20);
        e.call(0x00d2_c450, &args![this, 3i32, BLOCKS + 0x300]);
        let counts: Vec<u32> = calls_to(&calls, 0x1c).iter().map(|c| c[2]).collect();
        assert_eq!(counts, vec![4, 4, 2]);
        assert_eq!(row(&e, this, 3).1, 11);
    }

    #[test]
    fn the_row_emptier_pops_blocks_and_stores_a_null_for_a_null_head() {
        let mut e = Engine::new();
        let (this, _, calls) = fixture(&mut e);
        put_list(&mut e, this, 2, 6);
        e.call(0x00d2_c520, &args![this, 2i32]);
        let batches = calls_to(&calls, 0x1c);
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0][2..4], [4, 0x20]);
        assert_eq!(batches[1][2..4], [2, 0x20]);
        assert_eq!(batches[1][4..], [BLOCKS + 0x40, BLOCKS + 0x50]);
        assert_eq!(row(&e, this, 2), (0, 0));
        // A count with no blocks behind it: nulls are handed over and the count stays.
        calls.borrow_mut().clear();
        e.mem.set_u32(this + 0x10 + 8 * 2, 3);
        e.call(0x00d2_c520, &args![this, 2i32]);
        let batches = calls_to(&calls, 0x1c);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0][2..], [3, 0x20, 0, 0, 0]);
        assert_eq!(row(&e, this, 2), (0, 3));
    }

    #[test]
    fn block_alloc_takes_from_the_list_or_refills_it() {
        let mut e = Engine::new();
        let (this, allocator, calls) = fixture(&mut e);
        // A block in the class's list: taken, count down, head moves on.
        put_list(&mut e, this, 3, 2);
        assert_eq!(e.call(0x00d2_c5a0, &args![this, 0x25i32]).u32(), BLOCKS);
        assert_eq!(row(&e, this, 3), (BLOCKS + 0x10, 1));
        assert!(calls.borrow().is_empty());
        // Empty list, limit zero: slot 4 with the class's block size.
        put_list(&mut e, this, 3, 0);
        assert_eq!(e.call(0x00d2_c5a0, &args![this, 0x25i32]).u32(), NEW_BLOCK);
        assert_eq!(calls_to(&calls, 4), vec![vec![allocator, 0x30]]);
        // Empty list, limit 6: a batch of four, the first returned, the rest chained.
        e.mem.set_u32(this + 8, 6);
        let first = e.call(0x00d2_c5a0, &args![this, 0x25i32]).u32();
        assert_eq!(first, BLOCKS + 0x8000);
        let batch = calls_to(&calls, 0x18);
        assert_eq!(batch.len(), 1);
        assert_eq!(batch[0][2..], [4, 0x30]);
        assert_eq!(row(&e, this, 3), (BLOCKS + 0x8300, 3));
        assert_eq!(e.mem.u32(BLOCKS + 0x8300), BLOCKS + 0x8200);
        assert_eq!(e.mem.u32(BLOCKS + 0x8200), BLOCKS + 0x8100);
        assert_eq!(e.mem.u32(BLOCKS + 0x8100), 0);
        // A limit of two asks for two.
        put_list(&mut e, this, 3, 0);
        e.mem.set_u32(this + 8, 2);
        e.call(0x00d2_c5a0, &args![this, 0x25i32]);
        assert_eq!(calls_to(&calls, 0x18)[1][2], 2);
        assert_eq!(row(&e, this, 3).1, 1);
        // A big size is passed on unchanged; a size over 0x280 uses the word table.
        e.call(0x00d2_c5a0, &args![this, 0x2001i32]);
        assert_eq!(calls_to(&calls, 4)[1], vec![allocator, 0x2001]);
        put_list(&mut e, this, 13, 1);
        assert_eq!(e.call(0x00d2_c5a0, &args![this, 0x400i32]).u32(), BLOCKS);
    }

    #[test]
    fn block_free_pushes_below_the_limit_and_calls_the_row_full_path_at_it() {
        let mut e = Engine::new();
        let (this, allocator, calls) = fixture(&mut e);
        // A null block does nothing.
        e.call(0x00d2_c600, &args![this, 0u32, 0x20i32]);
        assert!(calls.borrow().is_empty());
        // Below the limit: pushed.
        e.mem.set_u32(this + 8, 2);
        e.call(0x00d2_c600, &args![this, BLOCKS + 0x100, 0x20i32]);
        assert_eq!(row(&e, this, 2), (BLOCKS + 0x100, 1));
        // The second block still fits; the third reaches the limit and takes the row-full path
        // (limit 2, half 1: one block goes, the new block is kept).
        e.call(0x00d2_c600, &args![this, BLOCKS + 0x110, 0x20i32]);
        e.call(0x00d2_c600, &args![this, BLOCKS + 0x120, 0x20i32]);
        assert_eq!(calls_to(&calls, 0x1c).len(), 1);
        assert_eq!(row(&e, this, 2).1, 2);
        // A big size goes to the allocator.
        e.call(0x00d2_c600, &args![this, BLOCKS + 0x130, 0x2001i32]);
        assert_eq!(
            calls_to(&calls, 8),
            vec![vec![allocator, BLOCKS + 0x130, 0x2001]]
        );
    }

    #[test]
    fn can_alloc_total_and_get_allocated_size_forward_to_the_allocator() {
        let mut e = Engine::new();
        let (this, allocator, calls) = fixture(&mut e);
        assert_eq!(e.call(0x00d2_c670, &args![this, 0x999u32]).u32(), 1);
        assert_eq!(calls_to(&calls, 0x24), vec![vec![allocator, 0x999]]);
        // A size up to 0x2000 is rounded up to its class's block size.
        assert_eq!(
            e.call(0x00d2_c680, &args![this, 0x4321u32, 0x25i32]).u32(),
            0x77
        );
        // A bigger one is passed as it is.
        e.call(0x00d2_c680, &args![this, 0x4321u32, 0x3000i32]);
        assert_eq!(
            calls_to(&calls, 0x2c),
            vec![
                vec![allocator, 0x4321, 0x30],
                vec![allocator, 0x4321, 0x3000]
            ]
        );
    }

    #[test]
    fn buf_realloc_copies_the_smaller_size_and_frees_the_old_block() {
        let mut e = Engine::new();
        let (this, allocator, calls) = fixture(&mut e);
        let copies = Rc::new(RefCell::new(Vec::new()));
        let log = copies.clone();
        e.register_double(0x00d2_caa0, move |_, words| {
            log.borrow_mut().push(words.to_vec());
            Ret::default()
        });
        let wanted = e.mem.alloc(4);
        e.mem.set_u32(wanted, 0x40);
        let block = e
            .call(0x00d2_c6d0, &args![this, 0x7000u32, 0x20i32, wanted])
            .u32();
        assert_eq!(block, NEW_BLOCK);
        // Wanted 0x40 against old 0x20: copy 0x20; the old block is freed with `old_size`.
        assert_eq!(copies.borrow()[0], vec![NEW_BLOCK, 0x7000, 0x20]);
        assert_eq!(calls_to(&calls, 8), vec![vec![allocator, 0x7000, 0x20]]);
        // Wanted 0x10 against old 0x20: copy 0x10.
        e.mem.set_u32(wanted, 0x10);
        e.call(0x00d2_c6d0, &args![this, 0x7000u32, 0x20i32, wanted]);
        assert_eq!(copies.borrow()[1], vec![NEW_BLOCK, 0x7000, 0x10]);
    }

    #[test]
    fn release_cached_memory_empties_the_rows_with_a_count_from_the_top() {
        let mut e = Engine::new();
        let (this, _, calls) = fixture(&mut e);
        put_list(&mut e, this, 2, 3);
        e.mem.set_u32(BLOCKS + 0x800, 0);
        e.mem.set_u32(this + 0x10 + 8 * 16, 1);
        e.mem.set_u32(this + 0xc + 8 * 16, BLOCKS + 0x800);
        e.call(0x00d2_c7a0, &args![this]);
        let sizes: Vec<u32> = calls_to(&calls, 0x1c).iter().map(|c| c[3]).collect();
        assert_eq!(sizes, vec![0x2000, 0x20]);
        assert_eq!(row(&e, this, 2), (0, 0));
        assert_eq!(row(&e, this, 16), (0, 0));
    }

    /// What a run stopped with: the panic's message.
    fn stop_message(run: impl FnOnce()) -> String {
        let stop = std::panic::catch_unwind(std::panic::AssertUnwindSafe(run))
            .expect_err("the check should have stopped the run");
        stop.downcast_ref::<String>().cloned().unwrap_or_default()
    }

    /// A thread memory with the check on, the row limit 8 and two 0x400-byte blocks (class 13)
    /// on its list: `BLOCKS + 0x500` at the head, linking to `BLOCKS + 0x100`.
    fn checked(e: &mut Engine, mode: hkmem_check::Mode) -> u32 {
        hkmem_check::force(mode);
        let (this, _, _) = fixture(e);
        e.mem.set_u32(this + 8, 8);
        e.call(0x00d2_c600, &args![this, BLOCKS + 0x100, 0x400i32]);
        e.call(0x00d2_c600, &args![this, BLOCKS + 0x500, 0x400i32]);
        assert_eq!(row(e, this, 13), (BLOCKS + 0x500, 2));
        this
    }

    #[test]
    fn the_hkmem_check_lets_clean_frees_and_allocations_run() {
        let mut e = Engine::new();
        let this = checked(&mut e, hkmem_check::Mode::Calls);
        assert_eq!(
            e.call(0x00d2_c5a0, &args![this, 0x400i32]).u32(),
            BLOCKS + 0x500
        );
        // The taken block is the caller's now: writing it is no fault.
        e.mem.set_u32(BLOCKS + 0x500, 0x3a03_3c7b);
        e.call(0x00d2_c600, &args![this, BLOCKS + 0x500, 0x400i32]);
        assert_eq!(
            e.call(0x00d2_c5a0, &args![this, 0x380i32]).u32(),
            BLOCKS + 0x500
        );
        assert_eq!(
            e.call(0x00d2_c5a0, &args![this, 0x400i32]).u32(),
            BLOCKS + 0x100
        );
        assert_eq!(row(&e, this, 13), (0, 0));
    }

    /// The p178 stop: a free block's link overwritten with float data. The check names the
    /// block, the old and new link and stops at the next allocator call, not at the bad read.
    #[test]
    fn the_hkmem_check_stops_at_a_free_block_that_was_written() {
        let mut e = Engine::new();
        let this = checked(&mut e, hkmem_check::Mode::Allocs);
        e.mem.set_u32(BLOCKS + 0x100, 0x3a03_3c7b);
        let message = stop_message(|| {
            e.call(0x00d2_c5a0, &args![this, 0x20i32]);
        });
        assert!(
            message.starts_with("hkmem check: thread memory"),
            "{message}"
        );
        assert!(
            message.contains("class 13 (0x400-byte blocks): free block 00900100 (link 1 of 2) was written: its link was 00000000"),
            "{message}"
        );
        assert!(message.contains("now 3a033c7b"), "{message}");
        assert!(
            message.contains("found at start of blockAlloc"),
            "{message}"
        );
        assert!(
            message.contains("clean at end of blockFree(")
                && message.contains(",00900500,00000400)"),
            "{message}"
        );
    }

    /// In `calls` mode the check runs between any two calls, so it names the function that
    /// was running when the block changed.
    #[test]
    fn the_hkmem_check_in_calls_mode_names_the_writer() {
        let mut e = Engine::new();
        let this = checked(&mut e, hkmem_check::Mode::Calls);
        e.register_double(0x00ab_c000, |e, _| {
            e.mem.set_u32(BLOCKS + 0x500, 0x3a03_3c7b);
            Ret::default()
        });
        e.register_double(0x00ab_d000, |_, _| Ret::default());
        e.call(0x00ab_d000, &args![]);
        let message = stop_message(|| {
            e.call(0x00ab_c000, &args![]);
        });
        assert!(
            message.contains("free block 00900500 (link 0 of 2) was written"),
            "{message}"
        );
        assert!(
            message.contains("found at return from 00abc000"),
            "{message}"
        );
        assert!(
            message.contains("clean at entry to 00abc000; 00abc000 ran between"),
            "{message}"
        );
        let _ = this;
    }

    /// A bad head word (not a block address) stops the run as the list is walked anew.
    #[test]
    fn the_hkmem_check_stops_at_a_link_that_is_not_a_block() {
        let mut e = Engine::new();
        let this = checked(&mut e, hkmem_check::Mode::Allocs);
        e.mem.set_u32(this + 0xc + 8 * 13, 0x3a03_3c7b);
        e.mem.set_u32(this + 0x10 + 8 * 13, 3);
        let message = stop_message(|| {
            e.call(0x00d2_c5a0, &args![this, 0x400i32]);
        });
        assert!(
            message.contains("the head (+0x74) (link 0 of 3) is 3a033c7b (f32 5.0"),
            "{message}"
        );
        assert!(
            message.contains("which is not 16-byte aligned"),
            "{message}"
        );
    }

    /// The p180w write: an output entry whose id was freed (slot 0xff) would be copied 0x2010
    /// bytes past the atom; the check stops first and names the remover.
    #[test]
    fn the_hkmem_check_stops_at_a_freed_contact_id() {
        let mut e = Engine::new();
        hkmem_check::force(hkmem_check::Mode::Allocs);
        let mgr = e.mem.alloc(0x60);
        let table = e.mem.alloc(8);
        let atom = e.mem.alloc(0x100);
        e.mem.write(table, &[1, 0xff, 0, 0, 0, 0, 0, 0]);
        e.mem.set_u32(mgr + 0x20, table);
        e.mem.set_u32(mgr + 0x24, 3);
        e.mem.set_u32(mgr + 0x3c, atom);
        e.mem.set_u16(atom + 4, 2);
        e.mem.set_u16(atom + 6, 4);
        let entry = e.mem.alloc(0x30);
        // Ids in use pass.
        hkmem_check::check_contact_id(&mut e, mgr, 0, entry);
        hkmem_check::check_contact_id(&mut e, mgr, 2, entry);
        hkmem_check::note_contact_removed(&e, mgr, 1);
        let message = stop_message(|| hkmem_check::check_contact_id(&mut e, mgr, 1, entry));
        assert!(
            message.contains("id 0x1 was freed (its slot is 0xff)"),
            "{message}"
        );
        assert!(
            message.contains(&format!("would write {:08x}", atom + 0x2010)),
            "{message}"
        );
        let message = stop_message(|| hkmem_check::check_contact_id(&mut e, mgr, 3, entry));
        assert!(
            message.contains("past the id table (size 0x3)"),
            "{message}"
        );
    }
}

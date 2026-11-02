//! `BSCore/scrapheap.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `ScrapHeap` is a stack-like allocator over one reserved 8 MiB address range
//! that it commits as needed; every block carries an 8-byte header (size and
//! previous block) and freeing marks the block, then pops the freed blocks off
//! the top. `ScrapHeapManager` keeps up to 64 committed ranges that heaps have
//! given back, so the next heap can reuse them.
//!
//! Several functions in this unit call the manager singleton accessor
//! (`00866d10`, `ScrapHeapManager::Instance` (Xbox PDB), another unit) and
//! `VirtualAlloc`/`VirtualFree` through their import slots.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::platform::MEMORY_MANAGER;

/// `ScrapHeapManager::Instance` (Xbox PDB), `00866d10`, no arguments: returns
/// the manager, creating it first when `011f63b4` is empty.
const MANAGER_INSTANCE: u32 = 0x0086_6d10;
/// The global holding the manager singleton.
const MANAGER_GLOBAL: u32 = 0x011f_63b4;
/// `00866a90`, `__thiscall` on the memory manager (`011f6238`) with three
/// stack words (a zero, the retry count, the address of a flag byte): called
/// when a commit failed; returns the new retry count.
const COMMIT_FAILED: u32 = 0x0086_6a90;
/// `0040fbf0`: acquires a recursive lock (`this` = lock; one unread word).
const LOCK: u32 = 0x0040_fbf0;
/// `MemoryManager::Allocate` (`00aa3e40`) and `Deallocate` (`00aa4060`).
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// Import slots of `VirtualAlloc` and `VirtualFree` (`kernel32`, stdcall).
const VIRTUAL_ALLOC: u32 = 0x00fd_f114;
const VIRTUAL_FREE: u32 = 0x00fd_f10c;

/// `MEM_COMMIT`, `MEM_RESERVE`, `MEM_DECOMMIT`, `MEM_RELEASE`,
/// `PAGE_READWRITE` (Windows constants the calls pass).
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const MEM_DECOMMIT: u32 = 0x4000;
const MEM_RELEASE: u32 = 0x8000;
const PAGE_READWRITE: u32 = 4;

/// Bytes a heap reserves (`0x800000`), and so the largest it can grow to.
const RESERVED_BYTES: u32 = 0x0080_0000;
/// The size `ScrapHeap::ScrapHeap` initializes a heap with.
const DEFAULT_HEAP_SIZE: u32 = 0x1_0000;
/// Heaps no larger than this are never shrunk.
const SHRINK_FLOOR: u32 = 0x1_0000;
/// Bit 31 of a block header's size word: the block has been freed.
const FREED_FLAG: u32 = 0x8000_0000;
/// Size of a block header (`ScrapHeap::Header`).
const HEADER_BYTES: u32 = 8;
/// Committed ranges the manager can keep (`0x40`).
const MAX_BUFFERS: u32 = 0x40;

/// Names the callers pass to the lock (never read by it).
const NAME_REQUEST_BUFFER: u32 = 0x010a_2724;
const NAME_RELEASE_BUFFER: u32 = 0x010a_2748;
const NAME_FREE_ALL_BUFFERS: u32 = 0x010a_276c;
const NAME_FREE_BUFFERS: u32 = 0x010a_2790;

layout! {
    /// `ScrapHeap` (Xbox PDB), 0x14 bytes (`iThreadLock` at +0x10 is not
    /// touched by this unit; 0x14 is also the size `BSScrapMap` gives it).
    /// The pointer fields are kept as `u32` addresses, since the code does
    /// arithmetic on them.
    pub struct ScrapHeap: 0x14 {
        /// `pMemHeap` (Xbox PDB): start of the reserved range.
        0x00 pMemHeap: u32,
        /// `pCurrentStackLoc` (Xbox PDB): where the next block header goes.
        0x04 pCurrentStackLoc: u32,
        /// `pEndHeap` (Xbox PDB): end of the committed part.
        0x08 pEndHeap: u32,
        /// `pLastBlock` (Xbox PDB): header of the newest block.
        0x0C pLastBlock: u32,
    }

    /// `ScrapHeap::Header` (Xbox PDB): in front of every block.
    pub struct ScrapHeapHeader: 0x08 {
        /// `iSize` (Xbox PDB): the block's size, bit 31 set once freed.
        0x00 iSize: u32,
        /// `pPrev` (Xbox PDB): the block allocated before this one.
        0x04 pPrev: u32,
    }

    /// `ScrapHeapBuffer` (Xbox PDB): one committed range kept by the manager.
    pub struct ScrapHeapBuffer: 0x08 {
        /// `pBuffer` (Xbox PDB).
        0x00 pBuffer: u32,
        /// `iCommittedSize` (Xbox PDB).
        0x04 iCommittedSize: u32,
    }

    /// `ScrapHeapManager` (Xbox PDB), 0x240 bytes on the PC (the creation
    /// allocates 0x240): 64 buffers of 8 bytes, then these. The PDB's layout
    /// differs (512-byte table, `BufferLock` at +0x208); the offsets here are
    /// the PC code's.
    pub struct ScrapHeapManager: 0x240 {
        /// `iBufferCount` (Xbox PDB).
        0x200 iBufferCount: u32,
        /// `iBufferTotalSize` (Xbox PDB).
        0x204 iBufferTotalSize: u32,
        /// `BufferLock` (Xbox PDB, a `BSSpinLock`), word 0: `uiOwningThread`.
        0x220 uiOwningThread: u32,
        /// `BufferLock` word 1: `uiLockCount`.
        0x224 uiLockCount: u32,
    }
}

/// The singleton manager (`ScrapHeapManager::Instance`, another unit).
fn instance(e: &mut Engine) -> Ptr<ScrapHeapManager> {
    e.call(MANAGER_INSTANCE, &args![]).ptr()
}

/// Entry `index` of the manager's buffer table.
fn buffer_entry(manager: Ptr<ScrapHeapManager>, index: u32) -> Ptr<ScrapHeapBuffer> {
    manager.cast::<ScrapHeapBuffer>().index(index, 8)
}

/// The manager's lock: the address `BufferLock` sits at.
fn buffer_lock(manager: Ptr<ScrapHeapManager>) -> Ptr {
    manager.byte_add(0x220)
}

/// Takes the buffer lock (`0040fbf0`) under the given call-site name.
fn lock_buffers(e: &mut Engine, manager: Ptr<ScrapHeapManager>, name: u32) {
    e.call(LOCK, &args![buffer_lock(manager), name]);
}

/// The lock release the compiler inlined at the end of every locked
/// function: decrements the count and clears the owner when it reaches 0.
fn unlock_buffers(e: &mut Engine, manager: Ptr<ScrapHeapManager>) {
    let count = e
        .get(manager, ScrapHeapManager::uiLockCount)
        .wrapping_sub(1);
    e.set(manager, ScrapHeapManager::uiLockCount, count);
    if count == 0 {
        e.set(manager, ScrapHeapManager::uiOwningThread, 0);
    }
}

// Translated from 00aa53f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScrapHeap::ScrapHeap` (Xbox PDB): initializes the heap with 0x10000
/// bytes and returns it.
pub fn scrap_heap_scrap_heap(e: &mut Engine, this: Ptr<ScrapHeap>) -> Ptr<ScrapHeap> {
    scrap_heap_init(e, this, DEFAULT_HEAP_SIZE);
    this
}

// Translated from 00aa5410 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor taking a size: rounds it up to a multiple of 0x10000
/// (a value already a multiple is kept; otherwise 0x10000 is added and the
/// low 16 bits cleared), initializes the heap with it and returns it.
pub fn fn_00aa5410(e: &mut Engine, this: Ptr<ScrapHeap>, size: u32) -> Ptr<ScrapHeap> {
    let size = if size & 0xffff != 0 {
        size.wrapping_add(0x1_0000) & 0xffff_0000
    } else {
        size
    };
    scrap_heap_init(e, this, size);
    this
}

// Translated from 00aa5460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the heap's whole range back to the buffer manager
/// (`ReleaseBuffer`, size = end - start) and clears the start and current
/// location (the end and last block stay).
pub fn fn_00aa5460(e: &mut Engine, this: Ptr<ScrapHeap>) {
    let start = e.get(this, ScrapHeap::pMemHeap);
    let end = e.get(this, ScrapHeap::pEndHeap);
    let manager = instance(e);
    scrap_heap_manager_release_buffer(e, manager, start, end.wrapping_sub(start));
    e.set(this, ScrapHeap::pMemHeap, 0);
    e.set(this, ScrapHeap::pCurrentStackLoc, 0);
}

// Translated from 00aa54a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScrapHeap::Allocate` (Xbox PDB): takes `size` bytes from the top of the
/// heap, the returned address aligned to `alignment` (0 = unaligned), after
/// its 8-byte header, which records the size and links the previous block.
/// When the block would reach the end of the committed range and the range
/// is still below 0x800000 bytes, the heap first grows by a doubling of its
/// size that covers the need (`ExpandBuffer`). Returns 0 when it does not
/// fit.
pub fn scrap_heap_allocate(e: &mut Engine, this: Ptr<ScrapHeap>, size: u32, alignment: u32) -> u32 {
    let start = e.get(this, ScrapHeap::pMemHeap);
    let top = e.get(this, ScrapHeap::pCurrentStackLoc);
    let mut data = top.wrapping_add(HEADER_BYTES);
    let misalignment = if alignment != 0 {
        alignment.wrapping_sub(1) & data
    } else {
        0
    };
    let header = if misalignment != 0 {
        data = alignment.wrapping_sub(misalignment).wrapping_add(data);
        data.wrapping_sub(HEADER_BYTES)
    } else {
        top
    };
    let block_end = data.wrapping_add(size);

    let heap_end = e.get(this, ScrapHeap::pEndHeap);
    let committed = heap_end.wrapping_sub(start);
    if block_end >= heap_end && committed < RESERVED_BYTES {
        let needed = block_end.wrapping_sub(heap_end);
        let mut doubled = committed.wrapping_add(committed);
        let mut extra = committed;
        while extra < needed {
            doubled = doubled.wrapping_shl(1);
            extra = doubled.wrapping_sub(committed);
        }
        let manager = instance(e);
        scrap_heap_manager_expand_buffer(e, manager, start, committed, extra);
        let heap_end = e.get(this, ScrapHeap::pEndHeap);
        e.set(this, ScrapHeap::pEndHeap, heap_end.wrapping_add(extra));
    }

    let heap_end = e.get(this, ScrapHeap::pEndHeap);
    if block_end < heap_end {
        let header: Ptr<ScrapHeapHeader> = Ptr::new(header);
        e.set(header, ScrapHeapHeader::iSize, size);
        let last = e.get(this, ScrapHeap::pLastBlock);
        e.set(header, ScrapHeapHeader::pPrev, last);
        e.set(this, ScrapHeap::pLastBlock, header.addr());
        e.set(this, ScrapHeap::pCurrentStackLoc, block_end);
        data
    } else {
        0
    }
}

// Translated from 00aa55e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocation entry with four stack words (`RET 0x10`): the first is the
/// size and the last the alignment, the two between are never read; it
/// forwards to [`scrap_heap_allocate`].
pub fn fn_00aa55e0(
    e: &mut Engine,
    this: Ptr<ScrapHeap>,
    size: u32,
    _unused_1: u32,
    _unused_2: u32,
    alignment: u32,
) -> u32 {
    scrap_heap_allocate(e, this, size, alignment)
}

// Translated from 00aa5610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScrapHeap::Deallocate` (Xbox PDB): marks the block freed (nothing for
/// null or a block already freed), pops freed blocks off the top (the
/// current location becomes the end of the newest live block, or the start
/// of the heap when none is left) and, when more than half of a heap larger
/// than 0x10000 bytes is then unused, decommits that half
/// (`00aa5e90`) and lowers the end by it.
pub fn scrap_heap_deallocate(e: &mut Engine, this: Ptr<ScrapHeap>, block: u32) {
    if block == 0 {
        return;
    }
    let header: Ptr<ScrapHeapHeader> = Ptr::new(block.wrapping_sub(HEADER_BYTES));
    let size_word = e.get(header, ScrapHeapHeader::iSize);
    if size_word & FREED_FLAG != 0 {
        return;
    }
    e.set(header, ScrapHeapHeader::iSize, size_word | FREED_FLAG);

    loop {
        let last = e.get(this, ScrapHeap::pLastBlock);
        if last == 0 {
            let start = e.get(this, ScrapHeap::pMemHeap);
            e.set(this, ScrapHeap::pCurrentStackLoc, start);
            break;
        }
        let last: Ptr<ScrapHeapHeader> = Ptr::new(last);
        let last_size = e.get(last, ScrapHeapHeader::iSize);
        if last_size & FREED_FLAG == 0 {
            let end_of_block = last
                .addr()
                .wrapping_add(last_size)
                .wrapping_add(HEADER_BYTES);
            e.set(this, ScrapHeap::pCurrentStackLoc, end_of_block);
            break;
        }
        let previous = e.get(last, ScrapHeapHeader::pPrev);
        e.set(this, ScrapHeap::pLastBlock, previous);
    }

    let start = e.get(this, ScrapHeap::pMemHeap);
    let heap_end = e.get(this, ScrapHeap::pEndHeap);
    let top = e.get(this, ScrapHeap::pCurrentStackLoc);
    let committed = heap_end.wrapping_sub(start);
    let half = committed >> 1;
    if half < heap_end.wrapping_sub(top) && committed > SHRINK_FLOOR {
        let manager = instance(e);
        scrap_heap_manager_decommit_tail(e, manager, start, committed, half);
        e.set(this, ScrapHeap::pEndHeap, heap_end.wrapping_sub(half));
    }
}

// Translated from 00aa5710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size recorded in a block's header with the freed flag cleared; 0 for
/// null. `this` is not read.
pub fn fn_00aa5710(e: &mut Engine, _this: Ptr<ScrapHeap>, block: u32) -> u32 {
    if block == 0 {
        return 0;
    }
    let header: Ptr<ScrapHeapHeader> = Ptr::new(block.wrapping_sub(HEADER_BYTES));
    e.get(header, ScrapHeapHeader::iSize) & !FREED_FLAG
}

// Translated from 00aa5740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the heap: the current location returns to the start and the
/// last block is cleared. Takes one stack word (`RET 4`) that it never
/// reads.
pub fn fn_00aa5740(e: &mut Engine, this: Ptr<ScrapHeap>, _unused_1: u32) {
    let start = e.get(this, ScrapHeap::pMemHeap);
    e.set(this, ScrapHeap::pCurrentStackLoc, start);
    e.set(this, ScrapHeap::pLastBlock, 0);
}

// Translated from 00aa5770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the block chain from the newest block to the oldest when the heap
/// is not empty and does nothing with it (a compiled-out consistency check:
/// the loop only reads each header's previous-block link).
pub fn fn_00aa5770(e: &mut Engine, this: Ptr<ScrapHeap>) {
    let start = e.get(this, ScrapHeap::pMemHeap);
    let top = e.get(this, ScrapHeap::pCurrentStackLoc);
    if top != start {
        let mut block = e.get(this, ScrapHeap::pLastBlock);
        while block != 0 {
            block = e.get(Ptr::<ScrapHeapHeader>::new(block), ScrapHeapHeader::pPrev);
        }
    }
}

// Translated from 00aa57b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScrapHeap::Init` (Xbox PDB): takes a buffer of `size` bytes from the
/// manager's `RequestBuffer`, which may hand back a larger one and then
/// raises the size in place; the heap spans the size as updated.
pub fn scrap_heap_init(e: &mut Engine, this: Ptr<ScrapHeap>, size: u32) {
    let manager = instance(e);
    // The size is the function's own stack argument, which `RequestBuffer`
    // is given the address of and may overwrite.
    let (start, size) = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), size);
        let start = scrap_heap_manager_request_buffer(e, manager, slot);
        (start, e.mem.u32(slot.addr()))
    });
    e.set(this, ScrapHeap::pMemHeap, start);
    e.set(this, ScrapHeap::pCurrentStackLoc, start);
    e.set(this, ScrapHeap::pEndHeap, start.wrapping_add(size));
    e.set(this, ScrapHeap::pLastBlock, 0);
}

// Translated from 00aa5800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts the blocks not yet freed, walking the chain from the newest
/// block (0 for an empty heap).
pub fn fn_00aa5800(e: &mut Engine, this: Ptr<ScrapHeap>) -> i32 {
    let mut live = 0;
    let start = e.get(this, ScrapHeap::pMemHeap);
    let top = e.get(this, ScrapHeap::pCurrentStackLoc);
    if top != start {
        let mut block = e.get(this, ScrapHeap::pLastBlock);
        while block != 0 {
            let header: Ptr<ScrapHeapHeader> = Ptr::new(block);
            if e.get(header, ScrapHeapHeader::iSize) & FREED_FLAG == 0 {
                live += 1;
            }
            block = e.get(header, ScrapHeapHeader::pPrev);
        }
    }
    live
}

// Translated from 00aa5860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScrapHeapManager` constructor: no buffers, no total, lock free.
pub fn fn_00aa5860(e: &mut Engine, this: Ptr<ScrapHeapManager>) -> Ptr<ScrapHeapManager> {
    e.set(this, ScrapHeapManager::iBufferCount, 0);
    e.set(this, ScrapHeapManager::iBufferTotalSize, 0);
    e.set(this, ScrapHeapManager::uiOwningThread, 0);
    e.set(this, ScrapHeapManager::uiLockCount, 0);
    this
}

// Translated from 00aa58b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every buffer of the manager (`FreeAllBuffers`).
pub fn fn_00aa58b0(e: &mut Engine, this: Ptr<ScrapHeapManager>) {
    scrap_heap_manager_free_all_buffers(e, this);
}

// Translated from 00aa58d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aa58d0 - the compiler's exception frame (FS:[0] chain with the /GS cookie value) is not translated.
/// Creates the manager singleton: allocates 0x240 bytes from the memory
/// manager, constructs the manager in them when the allocation succeeded
/// and stores the result (null when it failed) in the singleton global.
pub fn fn_00aa58d0(e: &mut Engine) {
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x240u32])
        .ptr::<ScrapHeapManager>();
    let manager = if block.is_null() {
        block
    } else {
        fn_00aa5860(e, block)
    };
    e.set_global(MANAGER_GLOBAL, manager.addr());
}

// Translated from 00aa5950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the manager singleton: frees all its buffers, releases its
/// memory and clears the singleton global (also when there was none).
pub fn fn_00aa5950(e: &mut Engine) {
    let manager: Ptr<ScrapHeapManager> = Ptr::new(e.global(MANAGER_GLOBAL));
    if !manager.is_null() {
        fn_00aa58b0(e, manager);
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, manager]);
    }
    e.set_global(MANAGER_GLOBAL, 0u32);
}

// Translated from 00aa59b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScrapHeapManager::RequestBuffer` (Xbox PDB): returns a committed range
/// of at least `*size` bytes. With no kept buffers it reserves and commits a
/// new range (`00aa5ec0`). Otherwise, under the lock, it takes the first
/// kept buffer that is large enough (and then writes that buffer's size to
/// `*size`) or, when none is, the largest one, extended by committing the
/// missing bytes after it (`ExpandBuffer`); `*size` is left as requested in
/// the second case.
pub fn scrap_heap_manager_request_buffer(
    e: &mut Engine,
    this: Ptr<ScrapHeapManager>,
    size: Ptr,
) -> u32 {
    if e.get(this, ScrapHeapManager::iBufferCount) == 0 {
        let wanted = e.mem.u32(size.addr());
        return fn_00aa5ec0(e, this, wanted);
    }
    lock_buffers(e, this, NAME_REQUEST_BUFFER);
    if e.get(this, ScrapHeapManager::iBufferCount) == 0 {
        let wanted = e.mem.u32(size.addr());
        let buffer = fn_00aa5ec0(e, this, wanted);
        unlock_buffers(e, this);
        return buffer;
    }
    let mut largest_size = 0;
    let mut largest_index = 0;
    let mut index = 0;
    while index < e.get(this, ScrapHeapManager::iBufferCount) {
        let entry = buffer_entry(this, index);
        let committed = e.get(entry, ScrapHeapBuffer::iCommittedSize);
        if committed >= e.mem.u32(size.addr()) {
            e.mem.set_u32(size.addr(), committed);
            let buffer = e.get(entry, ScrapHeapBuffer::pBuffer);
            fn_00aa5f50(e, this, index);
            unlock_buffers(e, this);
            return buffer;
        }
        if committed > largest_size {
            largest_size = committed;
            largest_index = index;
        }
        index += 1;
    }
    let buffer = e.get(buffer_entry(this, largest_index), ScrapHeapBuffer::pBuffer);
    fn_00aa5f50(e, this, largest_index);
    let wanted = e.mem.u32(size.addr());
    scrap_heap_manager_expand_buffer(
        e,
        this,
        buffer,
        largest_size,
        wanted.wrapping_sub(largest_size),
    );
    unlock_buffers(e, this);
    buffer
}

// Translated from 00aa5b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScrapHeapManager::ReleaseBuffer` (Xbox PDB): keeps the range for reuse
/// (appends it to the table and adds its size to the total), or releases
/// it (`FreeBuffer`) when 64 are already kept. The full-table release is
/// done once before the lock is taken and again under it, as the code does.
pub fn scrap_heap_manager_release_buffer(
    e: &mut Engine,
    this: Ptr<ScrapHeapManager>,
    buffer: u32,
    size: u32,
) {
    if e.get(this, ScrapHeapManager::iBufferCount) >= MAX_BUFFERS {
        scrap_heap_manager_free_buffer(e, this, buffer, size);
    }
    lock_buffers(e, this, NAME_RELEASE_BUFFER);
    let count = e.get(this, ScrapHeapManager::iBufferCount);
    if count >= MAX_BUFFERS {
        scrap_heap_manager_free_buffer(e, this, buffer, size);
    } else {
        let entry = buffer_entry(this, count);
        e.set(entry, ScrapHeapBuffer::pBuffer, buffer);
        e.set(entry, ScrapHeapBuffer::iCommittedSize, size);
        e.set(this, ScrapHeapManager::iBufferCount, count.wrapping_add(1));
        let total = e.get(this, ScrapHeapManager::iBufferTotalSize);
        e.set(
            this,
            ScrapHeapManager::iBufferTotalSize,
            total.wrapping_add(size),
        );
    }
    unlock_buffers(e, this);
}

// Translated from 00aa5c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScrapHeapManager::FreeAllBuffers` (Xbox PDB): under the lock, releases
/// every kept range and empties the table (nothing to do when it is
/// empty).
pub fn scrap_heap_manager_free_all_buffers(e: &mut Engine, this: Ptr<ScrapHeapManager>) {
    if e.get(this, ScrapHeapManager::iBufferCount) == 0 {
        return;
    }
    lock_buffers(e, this, NAME_FREE_ALL_BUFFERS);
    let mut index = 0;
    while index < e.get(this, ScrapHeapManager::iBufferCount) {
        let entry = buffer_entry(this, index);
        let buffer = e.get(entry, ScrapHeapBuffer::pBuffer);
        let size = e.get(entry, ScrapHeapBuffer::iCommittedSize);
        scrap_heap_manager_free_buffer(e, this, buffer, size);
        index += 1;
    }
    e.set(this, ScrapHeapManager::iBufferCount, 0);
    e.set(this, ScrapHeapManager::iBufferTotalSize, 0);
    unlock_buffers(e, this);
}

// Translated from 00aa5d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScrapHeapManager::FreeBuffers` (Xbox PDB): under the lock, releases the
/// newest kept ranges until at least `wanted` bytes are freed or none is
/// left, subtracts what was freed from the total and returns it (0 without
/// taking the lock when the table is empty).
pub fn scrap_heap_manager_free_buffers(
    e: &mut Engine,
    this: Ptr<ScrapHeapManager>,
    wanted: u32,
) -> u32 {
    if e.get(this, ScrapHeapManager::iBufferCount) == 0 {
        return 0;
    }
    lock_buffers(e, this, NAME_FREE_BUFFERS);
    let mut freed = 0u32;
    while e.get(this, ScrapHeapManager::iBufferCount) != 0 && freed < wanted {
        let last = e.get(this, ScrapHeapManager::iBufferCount).wrapping_sub(1);
        let entry = buffer_entry(this, last);
        let buffer = e.get(entry, ScrapHeapBuffer::pBuffer);
        let size = e.get(entry, ScrapHeapBuffer::iCommittedSize);
        freed = freed.wrapping_add(size);
        scrap_heap_manager_free_buffer(e, this, buffer, size);
        e.set(this, ScrapHeapManager::iBufferCount, last);
    }
    let total = e.get(this, ScrapHeapManager::iBufferTotalSize);
    e.set(
        this,
        ScrapHeapManager::iBufferTotalSize,
        total.wrapping_sub(freed),
    );
    unlock_buffers(e, this);
    freed
}

/// Runs `VirtualAlloc(address, size, MEM_COMMIT, PAGE_READWRITE)` until it
/// succeeds; after each failure calls `00866a90` (which may free memory)
/// with the retry count and takes its result as the new count.
fn commit_until_success(e: &mut Engine, address: u32, size: u32) {
    let mut retries = 0u32;
    e.with_stack(4, |e, flag| loop {
        let committed = e
            .call(
                VIRTUAL_ALLOC,
                &args![address, size, MEM_COMMIT, PAGE_READWRITE],
            )
            .u32();
        if committed != 0 {
            break;
        }
        e.mem.set_u8(flag.addr(), 1);
        retries = e
            .call(COMMIT_FAILED, &args![MEMORY_MANAGER, 0u32, retries, flag])
            .u32();
    });
}

// Translated from 00aa5e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScrapHeapManager::ExpandBuffer` (Xbox PDB): commits `extra` more bytes
/// directly after the `size` committed ones at `base`, retrying until the
/// system grants it.
pub fn scrap_heap_manager_expand_buffer(
    e: &mut Engine,
    _this: Ptr<ScrapHeapManager>,
    base: u32,
    size: u32,
    extra: u32,
) {
    commit_until_success(e, base.wrapping_add(size), extra);
}

// Translated from 00aa5e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Decommits the last `amount` bytes of the `size` committed bytes at
/// `base` (`VirtualFree` with `MEM_DECOMMIT`). The Xbox PDB has no name for
/// it.
pub fn scrap_heap_manager_decommit_tail(
    e: &mut Engine,
    _this: Ptr<ScrapHeapManager>,
    base: u32,
    size: u32,
    amount: u32,
) {
    let tail = base.wrapping_add(size).wrapping_sub(amount);
    e.call(VIRTUAL_FREE, &args![tail, amount, MEM_DECOMMIT]);
}

// Translated from 00aa5ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reserves a new 0x800000-byte range and commits its first `size` bytes
/// (retrying like [`scrap_heap_manager_expand_buffer`]); returns the start.
/// `this` is not read.
pub fn fn_00aa5ec0(e: &mut Engine, _this: Ptr<ScrapHeapManager>, size: u32) -> u32 {
    let base = e
        .call(
            VIRTUAL_ALLOC,
            &args![0u32, RESERVED_BYTES, MEM_RESERVE, PAGE_READWRITE],
        )
        .u32();
    commit_until_success(e, base, size);
    base
}

// Translated from 00aa5f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScrapHeapManager::FreeBuffer` (Xbox PDB): releases the range at
/// `buffer` (`VirtualFree` with `MEM_RELEASE`). The second stack word is
/// never read.
pub fn scrap_heap_manager_free_buffer(
    e: &mut Engine,
    _this: Ptr<ScrapHeapManager>,
    buffer: u32,
    _unused_1: u32,
) {
    e.call(VIRTUAL_FREE, &args![buffer, 0u32, MEM_RELEASE]);
}

// Translated from 00aa5f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes entry `index` from the buffer table: lowers the count, subtracts
/// the entry's size from the total and, unless it was the last entry, moves
/// the last entry into its place.
pub fn fn_00aa5f50(e: &mut Engine, this: Ptr<ScrapHeapManager>, index: u32) {
    let count = e.get(this, ScrapHeapManager::iBufferCount).wrapping_sub(1);
    e.set(this, ScrapHeapManager::iBufferCount, count);
    let entry = buffer_entry(this, index);
    let total = e.get(this, ScrapHeapManager::iBufferTotalSize);
    let size = e.get(entry, ScrapHeapBuffer::iCommittedSize);
    e.set(
        this,
        ScrapHeapManager::iBufferTotalSize,
        total.wrapping_sub(size),
    );
    if index < count {
        let last = buffer_entry(this, count);
        let last_buffer = e.get(last, ScrapHeapBuffer::pBuffer);
        let last_size = e.get(last, ScrapHeapBuffer::iCommittedSize);
        e.set(entry, ScrapHeapBuffer::pBuffer, last_buffer);
        e.set(entry, ScrapHeapBuffer::iCommittedSize, last_size);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa53f0,
            scrap_heap_scrap_heap(Ptr<ScrapHeap>) -> Ptr<ScrapHeap>
        ),
        entry!(
            0x00aa5410,
            fn_00aa5410(Ptr<ScrapHeap>, u32) -> Ptr<ScrapHeap>
        ),
        entry!(0x00aa5460, fn_00aa5460(Ptr<ScrapHeap>)),
        entry!(
            0x00aa54a0,
            scrap_heap_allocate(Ptr<ScrapHeap>, u32, u32) -> u32
        ),
        entry!(
            0x00aa55e0,
            fn_00aa55e0(Ptr<ScrapHeap>, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00aa5610, scrap_heap_deallocate(Ptr<ScrapHeap>, u32)),
        entry!(0x00aa5710, fn_00aa5710(Ptr<ScrapHeap>, u32) -> u32),
        entry!(0x00aa5740, fn_00aa5740(Ptr<ScrapHeap>, u32)),
        entry!(0x00aa5770, fn_00aa5770(Ptr<ScrapHeap>)),
        entry!(0x00aa57b0, scrap_heap_init(Ptr<ScrapHeap>, u32)),
        entry!(0x00aa5800, fn_00aa5800(Ptr<ScrapHeap>) -> i32),
        entry!(
            0x00aa5860,
            fn_00aa5860(Ptr<ScrapHeapManager>) -> Ptr<ScrapHeapManager>
        ),
        entry!(0x00aa58b0, fn_00aa58b0(Ptr<ScrapHeapManager>)),
        entry!(0x00aa58d0, fn_00aa58d0()),
        entry!(0x00aa5950, fn_00aa5950()),
        entry!(
            0x00aa59b0,
            scrap_heap_manager_request_buffer(Ptr<ScrapHeapManager>, Ptr) -> u32
        ),
        entry!(
            0x00aa5b70,
            scrap_heap_manager_release_buffer(Ptr<ScrapHeapManager>, u32, u32)
        ),
        entry!(
            0x00aa5c80,
            scrap_heap_manager_free_all_buffers(Ptr<ScrapHeapManager>)
        ),
        entry!(
            0x00aa5d40,
            scrap_heap_manager_free_buffers(Ptr<ScrapHeapManager>, u32) -> u32
        ),
        entry!(
            0x00aa5e30,
            scrap_heap_manager_expand_buffer(Ptr<ScrapHeapManager>, u32, u32, u32)
        ),
        entry!(
            0x00aa5e90,
            scrap_heap_manager_decommit_tail(Ptr<ScrapHeapManager>, u32, u32, u32)
        ),
        entry!(0x00aa5ec0, fn_00aa5ec0(Ptr<ScrapHeapManager>, u32) -> u32),
        entry!(
            0x00aa5f30,
            scrap_heap_manager_free_buffer(Ptr<ScrapHeapManager>, u32, u32)
        ),
        entry!(0x00aa5f50, fn_00aa5f50(Ptr<ScrapHeapManager>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The kernel and manager callees, doubled: `Instance` returns `manager`,
    /// the lock takes the lock words as the real one does, `VirtualAlloc`
    /// returns its address argument (or `0x5000_0000` for a null address) and
    /// everything else returns 0.
    fn scrap_engine() -> (Engine, Ptr<ScrapHeapManager>) {
        let mut e = Engine::new();
        let manager: Ptr<ScrapHeapManager> = e.new_object();
        e.register_double(MANAGER_INSTANCE, move |_, _| manager.addr().into_ret());
        e.register(LOCK, |e, a| {
            e.mem.set_u32(a[0], 1);
            let count = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, count + 1);
            Ret::default()
        });
        e.register(VIRTUAL_ALLOC, |_, a| {
            let address = if a[0] == 0 { 0x5000_0000 } else { a[0] };
            address.into_ret()
        });
        e.register(VIRTUAL_FREE, |_, _| 1u32.into_ret());
        e.register(COMMIT_FAILED, |_, _| Ret::default());
        (e, manager)
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A heap over `size` bytes at a fresh block of memory; the manager is
    /// the one from [`scrap_engine`].
    fn heap_over(e: &mut Engine, size: u32) -> Ptr<ScrapHeap> {
        let heap: Ptr<ScrapHeap> = e.new_object();
        let start = e.mem.alloc(size);
        e.set(heap, ScrapHeap::pMemHeap, start);
        e.set(heap, ScrapHeap::pCurrentStackLoc, start);
        e.set(heap, ScrapHeap::pEndHeap, start + size);
        heap
    }

    fn add_buffer(e: &mut Engine, manager: Ptr<ScrapHeapManager>, buffer: u32, size: u32) {
        let count = e.get(manager, ScrapHeapManager::iBufferCount);
        let entry = buffer_entry(manager, count);
        e.set(entry, ScrapHeapBuffer::pBuffer, buffer);
        e.set(entry, ScrapHeapBuffer::iCommittedSize, size);
        e.set(manager, ScrapHeapManager::iBufferCount, count + 1);
        let total = e.get(manager, ScrapHeapManager::iBufferTotalSize);
        e.set(manager, ScrapHeapManager::iBufferTotalSize, total + size);
    }

    fn buffers(e: &Engine, manager: Ptr<ScrapHeapManager>) -> Vec<(u32, u32)> {
        let count = e.get(manager, ScrapHeapManager::iBufferCount);
        (0..count)
            .map(|i| {
                let entry = buffer_entry(manager, i);
                (
                    e.get(entry, ScrapHeapBuffer::pBuffer),
                    e.get(entry, ScrapHeapBuffer::iCommittedSize),
                )
            })
            .collect()
    }

    #[test]
    fn constructor_initializes_with_the_default_size() {
        let (mut e, _manager) = scrap_engine();
        let heap: Ptr<ScrapHeap> = e.new_object();
        let back = e.call(0x00aa_53f0, &args![heap]).ptr::<ScrapHeap>();
        assert_eq!(back, heap);
        assert_eq!(e.get(heap, ScrapHeap::pMemHeap), 0x5000_0000);
        assert_eq!(e.get(heap, ScrapHeap::pEndHeap), 0x5001_0000);
    }

    #[test]
    fn sized_constructor_rounds_up_to_64k() {
        let (mut e, _manager) = scrap_engine();
        let heap: Ptr<ScrapHeap> = e.new_object();
        e.call(0x00aa_5410, &args![heap, 0x1_0001u32]);
        assert_eq!(e.get(heap, ScrapHeap::pEndHeap), 0x5002_0000);
        // Already a multiple: kept (and zero stays zero).
        e.call(0x00aa_5410, &args![heap, 0x3_0000u32]);
        assert_eq!(e.get(heap, ScrapHeap::pEndHeap), 0x5003_0000);
        e.call(0x00aa_5410, &args![heap, 0u32]);
        assert_eq!(e.get(heap, ScrapHeap::pEndHeap), 0x5000_0000);
    }

    #[test]
    fn release_everything_hands_the_range_to_the_manager() {
        let (mut e, manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x2000);
        let start = e.get(heap, ScrapHeap::pMemHeap);
        e.call(0x00aa_5460, &args![heap]);
        assert_eq!(buffers(&e, manager), vec![(start, 0x2000)]);
        assert_eq!(e.get(heap, ScrapHeap::pMemHeap), 0);
        assert_eq!(e.get(heap, ScrapHeap::pCurrentStackLoc), 0);
        assert_eq!(e.get(heap, ScrapHeap::pEndHeap), start + 0x2000);
    }

    #[test]
    fn allocate_places_a_header_and_links_blocks() {
        let (mut e, _manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x1000);
        let start = e.get(heap, ScrapHeap::pMemHeap);
        let first = e.call(0x00aa_54a0, &args![heap, 0x20u32, 0u32]).u32();
        assert_eq!(first, start + 8);
        let second = e.call(0x00aa_54a0, &args![heap, 0x10u32, 0u32]).u32();
        assert_eq!(second, start + 0x20 + 16);
        assert_eq!(e.mem.u32(start), 0x20);
        assert_eq!(e.mem.u32(start + 4), 0);
        assert_eq!(e.mem.u32(second - 8), 0x10);
        assert_eq!(e.mem.u32(second - 4), start);
        assert_eq!(e.get(heap, ScrapHeap::pLastBlock), second - 8);
        assert_eq!(e.get(heap, ScrapHeap::pCurrentStackLoc), second + 0x10);
    }

    #[test]
    fn allocate_aligns_the_data_not_the_header() {
        let (mut e, _manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x1000);
        let block = e.call(0x00aa_54a0, &args![heap, 8u32, 0x40u32]).u32();
        assert_eq!(block & 0x3f, 0);
        let header = block - 8;
        assert_eq!(e.mem.u32(header), 8);
        assert_eq!(e.get(heap, ScrapHeap::pLastBlock), header);
        assert_eq!(e.get(heap, ScrapHeap::pCurrentStackLoc), block + 8);
    }

    #[test]
    fn allocate_that_does_not_fit_a_full_size_heap_returns_0() {
        let (mut e, _manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x80_0000);
        e.call_log = Some(vec![]);
        let block = e.call(0x00aa_54a0, &args![heap, 0x90_0000u32, 0u32]).u32();
        assert_eq!(block, 0);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, VIRTUAL_ALLOC).is_empty());
        assert_eq!(e.get(heap, ScrapHeap::pLastBlock), 0);
    }

    #[test]
    fn allocate_grows_the_heap_by_doubling() {
        let (mut e, _manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x1000);
        let start = e.get(heap, ScrapHeap::pMemHeap);
        e.call_log = Some(vec![]);
        // Needs 0x2008 + 0x10 past the end: 0x1000 -> extra 0x3000 (doubling
        // twice: 0x1000, 0x3000).
        let block = e.call(0x00aa_54a0, &args![heap, 0x2000u32, 0u32]).u32();
        let log = e.call_log.take().unwrap();
        // end 0x1000 - block_end 0x2008 => needed 0x1008; extra 0x1000 is
        // too small, then 0x3000.
        assert_eq!(
            calls_to(&log, VIRTUAL_ALLOC),
            vec![vec![start + 0x1000, 0x3000, MEM_COMMIT, PAGE_READWRITE]]
        );
        assert_eq!(block, start + 8);
        assert_eq!(e.get(heap, ScrapHeap::pEndHeap), start + 0x4000);
    }

    #[test]
    fn allocate_with_four_words_uses_first_as_size_and_last_as_alignment() {
        let (mut e, _manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x1000);
        let block = e
            .call(
                0x00aa_55e0,
                &args![heap, 0x10u32, 0xdeadu32, 0xbeefu32, 0x20u32],
            )
            .u32();
        assert_eq!(block & 0x1f, 0);
        assert_eq!(e.mem.u32(block - 8), 0x10);
    }

    #[test]
    fn deallocate_pops_freed_blocks_off_the_top() {
        let (mut e, _manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x1000);
        let start = e.get(heap, ScrapHeap::pMemHeap);
        let a = e.call(0x00aa_54a0, &args![heap, 0x20u32, 0u32]).u32();
        let b = e.call(0x00aa_54a0, &args![heap, 0x20u32, 0u32]).u32();
        let c = e.call(0x00aa_54a0, &args![heap, 0x20u32, 0u32]).u32();
        // Freeing the middle block only marks it.
        e.call(0x00aa_5610, &args![heap, b]);
        assert_eq!(e.mem.u32(b - 8), 0x20 | FREED_FLAG);
        assert_eq!(e.get(heap, ScrapHeap::pLastBlock), c - 8);
        assert_eq!(e.get(heap, ScrapHeap::pCurrentStackLoc), c + 0x20);
        // Freeing the top pops it and the freed one below.
        e.call(0x00aa_5610, &args![heap, c]);
        assert_eq!(e.get(heap, ScrapHeap::pLastBlock), a - 8);
        assert_eq!(e.get(heap, ScrapHeap::pCurrentStackLoc), a + 0x20);
        // Freeing the same block again changes nothing.
        e.call(0x00aa_5610, &args![heap, c]);
        assert_eq!(e.get(heap, ScrapHeap::pCurrentStackLoc), a + 0x20);
        // Freeing the last live block empties the heap.
        e.call(0x00aa_5610, &args![heap, a]);
        assert_eq!(e.get(heap, ScrapHeap::pLastBlock), 0);
        assert_eq!(e.get(heap, ScrapHeap::pCurrentStackLoc), start);
        // Null does nothing.
        e.call(0x00aa_5610, &args![heap, 0u32]);
        assert_eq!(e.get(heap, ScrapHeap::pCurrentStackLoc), start);
    }

    #[test]
    fn deallocate_decommits_half_of_a_mostly_empty_large_heap() {
        let (mut e, _manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x4_0000);
        let start = e.get(heap, ScrapHeap::pMemHeap);
        let a = e.call(0x00aa_54a0, &args![heap, 0x20u32, 0u32]).u32();
        e.call_log = Some(vec![]);
        e.call(0x00aa_5610, &args![heap, a]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, VIRTUAL_FREE),
            vec![vec![start + 0x2_0000, 0x2_0000, MEM_DECOMMIT]]
        );
        assert_eq!(e.get(heap, ScrapHeap::pEndHeap), start + 0x2_0000);
    }

    #[test]
    fn deallocate_keeps_a_heap_of_64k_or_less() {
        let (mut e, _manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x1_0000);
        let a = e.call(0x00aa_54a0, &args![heap, 0x20u32, 0u32]).u32();
        e.call_log = Some(vec![]);
        e.call(0x00aa_5610, &args![heap, a]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, VIRTUAL_FREE).is_empty());
    }

    #[test]
    fn block_size_clears_the_freed_flag() {
        let (mut e, _manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x1000);
        let block = e.call(0x00aa_54a0, &args![heap, 0x30u32, 0u32]).u32();
        assert_eq!(e.call(0x00aa_5710, &args![heap, block]).u32(), 0x30);
        e.mem.set_u32(block - 8, 0x30 | FREED_FLAG);
        assert_eq!(e.call(0x00aa_5710, &args![heap, block]).u32(), 0x30);
        assert_eq!(e.call(0x00aa_5710, &args![heap, 0u32]).u32(), 0);
    }

    #[test]
    fn reset_returns_to_the_start() {
        let (mut e, _manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x1000);
        let start = e.get(heap, ScrapHeap::pMemHeap);
        e.call(0x00aa_54a0, &args![heap, 0x30u32, 0u32]);
        e.call(0x00aa_5740, &args![heap, 0x1234u32]);
        assert_eq!(e.get(heap, ScrapHeap::pCurrentStackLoc), start);
        assert_eq!(e.get(heap, ScrapHeap::pLastBlock), 0);
    }

    #[test]
    fn consistency_walk_changes_nothing() {
        let (mut e, _manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x1000);
        e.call(0x00aa_5770, &args![heap]);
        e.call(0x00aa_54a0, &args![heap, 0x30u32, 0u32]);
        e.call(0x00aa_54a0, &args![heap, 0x30u32, 0u32]);
        let top = e.get(heap, ScrapHeap::pCurrentStackLoc);
        let last = e.get(heap, ScrapHeap::pLastBlock);
        e.call(0x00aa_5770, &args![heap]);
        assert_eq!(e.get(heap, ScrapHeap::pCurrentStackLoc), top);
        assert_eq!(e.get(heap, ScrapHeap::pLastBlock), last);
    }

    #[test]
    fn init_takes_a_fresh_buffer_when_none_is_kept() {
        let (mut e, _manager) = scrap_engine();
        let heap: Ptr<ScrapHeap> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00aa_57b0, &args![heap, 0x2_0000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, VIRTUAL_ALLOC),
            vec![
                vec![0, 0x80_0000, MEM_RESERVE, PAGE_READWRITE],
                vec![0x5000_0000, 0x2_0000, MEM_COMMIT, PAGE_READWRITE],
            ]
        );
        assert_eq!(e.get(heap, ScrapHeap::pMemHeap), 0x5000_0000);
        assert_eq!(e.get(heap, ScrapHeap::pCurrentStackLoc), 0x5000_0000);
        assert_eq!(e.get(heap, ScrapHeap::pEndHeap), 0x5002_0000);
        assert_eq!(e.get(heap, ScrapHeap::pLastBlock), 0);
    }

    #[test]
    fn init_spans_the_size_of_a_larger_kept_buffer() {
        let (mut e, manager) = scrap_engine();
        add_buffer(&mut e, manager, 0x6000_0000, 0x4_0000);
        let heap: Ptr<ScrapHeap> = e.new_object();
        e.call(0x00aa_57b0, &args![heap, 0x1_0000u32]);
        assert_eq!(e.get(heap, ScrapHeap::pMemHeap), 0x6000_0000);
        assert_eq!(e.get(heap, ScrapHeap::pEndHeap), 0x6004_0000);
        assert_eq!(e.get(manager, ScrapHeapManager::iBufferCount), 0);
    }

    #[test]
    fn live_block_count_skips_freed_blocks() {
        let (mut e, _manager) = scrap_engine();
        let heap = heap_over(&mut e, 0x1000);
        assert_eq!(e.call(0x00aa_5800, &args![heap]).i32(), 0);
        let _a = e.call(0x00aa_54a0, &args![heap, 0x20u32, 0u32]).u32();
        let b = e.call(0x00aa_54a0, &args![heap, 0x20u32, 0u32]).u32();
        let _c = e.call(0x00aa_54a0, &args![heap, 0x20u32, 0u32]).u32();
        assert_eq!(e.call(0x00aa_5800, &args![heap]).i32(), 3);
        e.call(0x00aa_5610, &args![heap, b]);
        assert_eq!(e.call(0x00aa_5800, &args![heap]).i32(), 2);
    }

    #[test]
    fn manager_constructor_clears_count_total_and_lock() {
        let mut e = Engine::new();
        let manager: Ptr<ScrapHeapManager> = e.new_object();
        for off in [0x200, 0x204, 0x220, 0x224] {
            e.mem.set_u32(manager.addr() + off, 0xffff_ffff);
        }
        let back = e
            .call(0x00aa_5860, &args![manager])
            .ptr::<ScrapHeapManager>();
        assert_eq!(back, manager);
        for off in [0x200, 0x204, 0x220, 0x224] {
            assert_eq!(e.mem.u32(manager.addr() + off), 0);
        }
    }

    #[test]
    fn free_all_wrapper_releases_every_buffer() {
        let (mut e, manager) = scrap_engine();
        add_buffer(&mut e, manager, 0x6000_0000, 0x1_0000);
        e.call_log = Some(vec![]);
        e.call(0x00aa_58b0, &args![manager]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, VIRTUAL_FREE).len(), 1);
        assert_eq!(e.get(manager, ScrapHeapManager::iBufferCount), 0);
    }

    #[test]
    fn creating_the_manager_constructs_it_and_stores_it() {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        e.call(0x00aa_58d0, &args![]);
        let manager: u32 = e.global(MANAGER_GLOBAL);
        assert_ne!(manager, 0);
        assert_eq!(e.mem.u32(manager + 0x200), 0);
        assert_eq!(e.mem.u32(manager + 0x224), 0);
    }

    #[test]
    fn creating_the_manager_stores_null_when_allocation_fails() {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        e.set_global(MANAGER_GLOBAL, 0x1234u32);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| Ret::default());
        e.call(0x00aa_58d0, &args![]);
        assert_eq!(e.global::<u32>(MANAGER_GLOBAL), 0);
    }

    #[test]
    fn destroying_the_manager_frees_its_buffers_and_memory() {
        let (mut e, manager) = scrap_engine();
        e.map(0x011f_6000, 0x1000);
        e.set_global(MANAGER_GLOBAL, manager.addr());
        add_buffer(&mut e, manager, 0x6000_0000, 0x1_0000);
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00aa_5950, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, VIRTUAL_FREE).len(), 1);
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, manager.addr()]]
        );
        assert_eq!(e.global::<u32>(MANAGER_GLOBAL), 0);
    }

    #[test]
    fn destroying_without_a_manager_only_clears_the_global() {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        e.set_global(MANAGER_GLOBAL, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x00aa_5950, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(e.global::<u32>(MANAGER_GLOBAL), 0);
    }

    #[test]
    fn request_takes_the_first_buffer_that_is_large_enough() {
        let (mut e, manager) = scrap_engine();
        add_buffer(&mut e, manager, 0x6000_0000, 0x1_0000);
        add_buffer(&mut e, manager, 0x6100_0000, 0x4_0000);
        add_buffer(&mut e, manager, 0x6200_0000, 0x8_0000);
        let size: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(size.addr(), 0x2_0000);
        let buffer = e.call(0x00aa_59b0, &args![manager, size]).u32();
        assert_eq!(buffer, 0x6100_0000);
        assert_eq!(e.mem.u32(size.addr()), 0x4_0000);
        // The last entry fills the gap.
        assert_eq!(
            buffers(&e, manager),
            vec![(0x6000_0000, 0x1_0000), (0x6200_0000, 0x8_0000)]
        );
        assert_eq!(e.get(manager, ScrapHeapManager::iBufferTotalSize), 0x9_0000);
        assert_eq!(e.get(manager, ScrapHeapManager::uiLockCount), 0);
    }

    #[test]
    fn request_extends_the_largest_buffer_when_none_is_large_enough() {
        let (mut e, manager) = scrap_engine();
        add_buffer(&mut e, manager, 0x6000_0000, 0x1_0000);
        add_buffer(&mut e, manager, 0x6100_0000, 0x3_0000);
        let size: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(size.addr(), 0x5_0000);
        e.call_log = Some(vec![]);
        let buffer = e.call(0x00aa_59b0, &args![manager, size]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(buffer, 0x6100_0000);
        assert_eq!(e.mem.u32(size.addr()), 0x5_0000);
        assert_eq!(
            calls_to(&log, VIRTUAL_ALLOC),
            vec![vec![0x6103_0000, 0x2_0000, MEM_COMMIT, PAGE_READWRITE]]
        );
        assert_eq!(buffers(&e, manager), vec![(0x6000_0000, 0x1_0000)]);
        // The lock was taken (the double does not take it) and the inlined
        // release brought the count of 1 to 0 and cleared the owner.
        assert_eq!(e.get(manager, ScrapHeapManager::uiLockCount), 0);
        assert_eq!(e.get(manager, ScrapHeapManager::uiOwningThread), 0);
    }

    #[test]
    fn request_without_buffers_reserves_and_commits_without_the_lock() {
        let (mut e, manager) = scrap_engine();
        let size: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(size.addr(), 0x1_0000);
        e.call_log = Some(vec![]);
        let buffer = e.call(0x00aa_59b0, &args![manager, size]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(buffer, 0x5000_0000);
        assert!(calls_to(&log, LOCK).is_empty());
    }

    #[test]
    fn release_keeps_the_buffer_in_the_table() {
        let (mut e, manager) = scrap_engine();
        e.call_log = Some(vec![]);
        e.call(0x00aa_5b70, &args![manager, 0x6000_0000u32, 0x2_0000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(buffers(&e, manager), vec![(0x6000_0000, 0x2_0000)]);
        assert_eq!(e.get(manager, ScrapHeapManager::iBufferTotalSize), 0x2_0000);
        assert_eq!(
            calls_to(&log, LOCK),
            vec![vec![manager.addr() + 0x220, NAME_RELEASE_BUFFER]]
        );
        assert!(calls_to(&log, VIRTUAL_FREE).is_empty());
    }

    #[test]
    fn release_with_a_full_table_frees_the_range_twice() {
        let (mut e, manager) = scrap_engine();
        for i in 0..MAX_BUFFERS {
            add_buffer(&mut e, manager, 0x6000_0000 + i * 0x10_0000, 0x1_0000);
        }
        e.call_log = Some(vec![]);
        e.call(0x00aa_5b70, &args![manager, 0x7000_0000u32, 0x2_0000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, VIRTUAL_FREE),
            vec![
                vec![0x7000_0000, 0, MEM_RELEASE],
                vec![0x7000_0000, 0, MEM_RELEASE]
            ]
        );
        assert_eq!(e.get(manager, ScrapHeapManager::iBufferCount), MAX_BUFFERS);
    }

    #[test]
    fn free_all_buffers_empties_the_table_under_the_lock() {
        let (mut e, manager) = scrap_engine();
        // Empty table: nothing, not even the lock.
        e.call_log = Some(vec![]);
        e.call(0x00aa_5c80, &args![manager]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        add_buffer(&mut e, manager, 0x6000_0000, 0x1_0000);
        add_buffer(&mut e, manager, 0x6100_0000, 0x2_0000);
        e.call_log = Some(vec![]);
        e.call(0x00aa_5c80, &args![manager]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, VIRTUAL_FREE),
            vec![
                vec![0x6000_0000, 0, MEM_RELEASE],
                vec![0x6100_0000, 0, MEM_RELEASE]
            ]
        );
        assert_eq!(calls_to(&log, LOCK).len(), 1);
        assert_eq!(e.get(manager, ScrapHeapManager::iBufferCount), 0);
        assert_eq!(e.get(manager, ScrapHeapManager::iBufferTotalSize), 0);
    }

    #[test]
    fn free_buffers_frees_newest_first_until_enough() {
        let (mut e, manager) = scrap_engine();
        assert_eq!(e.call(0x00aa_5d40, &args![manager, 1u32]).u32(), 0);
        add_buffer(&mut e, manager, 0x6000_0000, 0x1_0000);
        add_buffer(&mut e, manager, 0x6100_0000, 0x2_0000);
        add_buffer(&mut e, manager, 0x6200_0000, 0x4_0000);
        e.call_log = Some(vec![]);
        let freed = e.call(0x00aa_5d40, &args![manager, 0x5_0000u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(freed, 0x6_0000);
        assert_eq!(
            calls_to(&log, VIRTUAL_FREE),
            vec![
                vec![0x6200_0000, 0, MEM_RELEASE],
                vec![0x6100_0000, 0, MEM_RELEASE]
            ]
        );
        assert_eq!(buffers(&e, manager), vec![(0x6000_0000, 0x1_0000)]);
        assert_eq!(e.get(manager, ScrapHeapManager::iBufferTotalSize), 0x1_0000);
        // Asking for more than there is frees all and returns what was there.
        let freed = e.call(0x00aa_5d40, &args![manager, 0x100_0000u32]).u32();
        assert_eq!(freed, 0x1_0000);
        // Asking for nothing frees nothing.
        add_buffer(&mut e, manager, 0x6300_0000, 0x1_0000);
        assert_eq!(e.call(0x00aa_5d40, &args![manager, 0u32]).u32(), 0);
        assert_eq!(e.get(manager, ScrapHeapManager::iBufferCount), 1);
    }

    #[test]
    fn expand_buffer_commits_after_the_committed_part_and_retries() {
        let (mut e, manager) = scrap_engine();
        let mut attempts = 0;
        e.register_double(VIRTUAL_ALLOC, move |_, _| {
            attempts += 1;
            u32::from(attempts >= 3).into_ret()
        });
        let seen_flags = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = seen_flags.clone();
        e.register_double(COMMIT_FAILED, move |e, a| {
            seen.borrow_mut().push((a[0], a[1], a[2], e.mem.u8(a[3])));
            (a[2] + 1).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(
            0x00aa_5e30,
            &args![manager, 0x6000_0000u32, 0x1_0000u32, 0x2_0000u32],
        );
        let log = e.call_log.take().unwrap();
        let allocs = calls_to(&log, VIRTUAL_ALLOC);
        assert_eq!(allocs.len(), 3);
        assert_eq!(
            allocs[0],
            vec![0x6001_0000, 0x2_0000, MEM_COMMIT, PAGE_READWRITE]
        );
        // The retry count threads through the failure handler, flag set.
        assert_eq!(
            *seen_flags.borrow(),
            vec![(MEMORY_MANAGER, 0, 0, 1), (MEMORY_MANAGER, 0, 1, 1)]
        );
    }

    #[test]
    fn decommit_tail_frees_the_end_of_the_committed_part() {
        let (mut e, manager) = scrap_engine();
        e.call_log = Some(vec![]);
        e.call(
            0x00aa_5e90,
            &args![manager, 0x6000_0000u32, 0x4_0000u32, 0x1_0000u32],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, VIRTUAL_FREE),
            vec![vec![0x6003_0000, 0x1_0000, MEM_DECOMMIT]]
        );
    }

    #[test]
    fn reserve_and_commit_retries_the_commit_not_the_reserve() {
        let (mut e, manager) = scrap_engine();
        let mut attempts = 0;
        e.register_double(VIRTUAL_ALLOC, move |_, a| {
            attempts += 1;
            let result = match attempts {
                1 => 0x7000_0000,
                2 => 0,
                _ => a[0],
            };
            result.into_ret()
        });
        e.call_log = Some(vec![]);
        let base = e.call(0x00aa_5ec0, &args![manager, 0x3_0000u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(base, 0x7000_0000);
        let allocs = calls_to(&log, VIRTUAL_ALLOC);
        assert_eq!(allocs.len(), 3);
        assert_eq!(
            allocs[2],
            vec![0x7000_0000, 0x3_0000, MEM_COMMIT, PAGE_READWRITE]
        );
        assert_eq!(calls_to(&log, COMMIT_FAILED).len(), 1);
    }

    #[test]
    fn free_buffer_releases_only_the_address() {
        let (mut e, manager) = scrap_engine();
        e.call_log = Some(vec![]);
        e.call(0x00aa_5f30, &args![manager, 0x6000_0000u32, 0x9999u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, VIRTUAL_FREE),
            vec![vec![0x6000_0000, 0, MEM_RELEASE]]
        );
    }

    #[test]
    fn remove_buffer_moves_the_last_entry_into_the_gap() {
        let (mut e, manager) = scrap_engine();
        add_buffer(&mut e, manager, 0x6000_0000, 0x1_0000);
        add_buffer(&mut e, manager, 0x6100_0000, 0x2_0000);
        add_buffer(&mut e, manager, 0x6200_0000, 0x4_0000);
        e.call(0x00aa_5f50, &args![manager, 0u32]);
        assert_eq!(
            buffers(&e, manager),
            vec![(0x6200_0000, 0x4_0000), (0x6100_0000, 0x2_0000)]
        );
        assert_eq!(e.get(manager, ScrapHeapManager::iBufferTotalSize), 0x6_0000);
        // Removing the last entry just drops it.
        e.call(0x00aa_5f50, &args![manager, 1u32]);
        assert_eq!(buffers(&e, manager), vec![(0x6200_0000, 0x4_0000)]);
        assert_eq!(e.get(manager, ScrapHeapManager::iBufferTotalSize), 0x4_0000);
    }

    /// An engine with the kernel-object and virtual-memory stand-ins
    /// registered at their import slots, as `Engine::with_exe` does (the
    /// helper of `units/platform.rs`'s tests, for the test moved from there).
    fn kernel_engine() -> Engine {
        let mut e = Engine::new();
        // Import slots of FalloutNV.exe 1.4.0.525 (Ghidra `PTR_<name>_<slot>`).
        let slots = [
            (0x00fd_f07c, "CloseHandle"),
            (0x00fd_f0b4, "CreateSemaphoreA"),
            (0x00fd_f0cc, "CreateEventA"),
            (0x00fd_f0d0, "SetEvent"),
            (0x00fd_f10c, "VirtualFree"),
            (0x00fd_f114, "VirtualAlloc"),
            (0x00fd_f11c, "SetThreadPriority"),
            (0x00fd_f120, "GetCurrentThread"),
            (0x00fd_f124, "CreateThread"),
            (0x00fd_f158, "ResumeThread"),
            (0x00fd_f1b4, "ReleaseSemaphore"),
            (0x00fd_f1c8, "WaitForSingleObject"),
            (0x00fd_f1cc, "ResetEvent"),
        ];
        let imports: Vec<crate::exe::Import> = slots
            .iter()
            .map(|&(slot, name)| crate::exe::Import {
                slot,
                dll: "KERNEL32.dll".into(),
                name: name.into(),
            })
            .collect();
        assert_eq!(e.register_imports(&imports), slots.len());
        e
    }

    // Moved from the platform's tests (`units/platform.rs`) with the engine split: it
    // runs this unit's buffer functions, which the core's own tests no longer link.
    #[test]
    fn scrap_heap_buffers_reserve_commit_and_release_through_the_stand_ins() {
        use crate::units::platform::ALLOCATION_GRANULARITY;
        let mut e = kernel_engine();
        // `00aa5ec0`: reserve 0x800000, commit the first 0x10000.
        let base = e.call(0x00aa_5ec0, &args![0u32, 0x1_0000u32]).u32();
        assert_eq!(base % ALLOCATION_GRANULARITY, 0);
        e.mem.set_u32(base + 0xfffc, 1);
        assert!(!e.mem.is_mapped(base + 0x1_0000));
        // `ExpandBuffer` (`00aa5e30`): commit 0x10000 more after them.
        e.call(0x00aa_5e30, &args![0u32, base, 0x1_0000u32, 0x1_0000u32]);
        assert_eq!(e.mem.u32(base + 0x1_fffc), 0);
        // `00aa5e90`: decommit the last 0x10000 again.
        e.call(0x00aa_5e90, &args![0u32, base, 0x2_0000u32, 0x1_0000u32]);
        assert!(!e.mem.is_mapped(base + 0x1_0000));
        assert_eq!(e.mem.u32(base + 0xfffc), 1);
        // `FreeBuffer` (`00aa5f30`): release.
        e.call(0x00aa_5f30, &args![0u32, base, 0x1_0000u32]);
        assert!(!e.mem.is_mapped(base));
        assert!(e.kernel.regions.is_empty());
    }
}

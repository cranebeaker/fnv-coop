//! `NiSystem/nistandardallocator.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiStandardAllocator` (Xbox PDB) keeps blocks of up to 0x100 bytes in the
//! small-object allocator `NiStandardAllocator::ms_pkSmallAlloc` (Xbox PDB,
//! the word at `011f622c`): an `NiSmallObjectAllocator` of 256
//! `NiFixedAllocator`s of 0x100 bytes each, one per block size. Larger blocks
//! go through `00aa1130` (allocate), `00aa1190` (free) and `00aa1250`
//! (reallocate). A block whose size the caller does not give back on
//! deallocation carries it in a 4-byte header before the address the caller
//! sees. The "external" methods go through the game's memory manager
//! (`011f6238`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiStandardAllocator::ms_pkSmallAlloc` (Xbox PDB): the small-object
/// allocator, `011f622c`.
const SMALL_ALLOCATOR: u32 = 0x011f_622c;
/// The memory manager object (`011f6238`).
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall` on the memory manager, one word): its allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00aa4060` (`thiscall` on the memory manager, one word): its deallocate.
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `00aa4150` (`thiscall` on the memory manager: the block and the new size):
/// its reallocate.
const MEMORY_MANAGER_REALLOCATE: u32 = 0x00aa_4150;
/// `00aa1130` (`cdecl`: size, alignment): allocates a block the small-object
/// allocator does not take.
const LARGE_ALLOCATE: u32 = 0x00aa_1130;
/// `00aa1190` (`cdecl`, one word): frees such a block.
const LARGE_FREE: u32 = 0x00aa_1190;
/// `00aa1250` (`cdecl`: block, size, alignment): reallocates such a block.
const LARGE_REALLOCATE: u32 = 0x00aa_1250;
/// `00aa1170` (`cdecl`, one word): frees an `NiFixedAllocator`'s chunk array.
const FREE_CHUNKS: u32 = 0x00aa_1170;
/// `00aa35a0` (`thiscall` on an `NiFixedAllocator`, no stack words): takes a
/// block from it.
const FIXED_ALLOCATE: u32 = 0x00aa_35a0;
/// `00aa34b0` (`thiscall` on an `NiFixedAllocator`, the block): finds the
/// chunk that holds the block.
const FIXED_FIND_CHUNK: u32 = 0x00aa_34b0;
/// `00aa36f0` (`thiscall` on an `NiFixedAllocator`, the block): gives the
/// block back to its chunk.
const FIXED_DEALLOCATE: u32 = 0x00aa_36f0;
/// CRT `memcpy` (`00ec44d0`, `cdecl`: destination, source, size).
const MEMCPY: u32 = 0x00ec_44d0;
/// CRT `memset` (`00ec61c0`, `cdecl`: destination, value, size).
const MEMSET: u32 = 0x00ec_61c0;
/// Import slots of `InitializeCriticalSection`, `DeleteCriticalSection`,
/// `EnterCriticalSection` and `LeaveCriticalSection` (`stdcall`, one word).
const API_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const API_DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const API_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const API_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The allocator's own vtable slots `Allocate` (+4) and `Deallocate` (+8)
/// (Xbox PDB `NiAllocator` slots), which `Reallocate` calls virtually.
const ALLOCATE_SLOT: u32 = 0x4;
const DEALLOCATE_SLOT: u32 = 0x8;

layout! {
    /// `NiFixedAllocator` (Xbox PDB), 0x100 bytes; the offsets are the PC
    /// code's (`00aa3760`, `00aa3560`).
    pub struct NiFixedAllocator: 0x100 {
        /// `m_stBlockSize` (Xbox PDB).
        0x00 m_stBlockSize: u32,
        /// `m_ucNumBlocks` (Xbox PDB).
        0x04 m_ucNumBlocks: u8,
        /// `m_pkChunks` (Xbox PDB): an array of 8-byte `Chunk`s whose first
        /// word is the chunk's data.
        0x08 m_pkChunks: u32,
        /// `m_stNumChunks` (Xbox PDB).
        0x0C m_stNumChunks: u32,
        /// `m_stMaxNumChunks` (Xbox PDB).
        0x10 m_stMaxNumChunks: u32,
        /// `m_pkAllocChunk` (Xbox PDB).
        0x14 m_pkAllocChunk: u32,
        /// `m_pkDeallocChunk` (Xbox PDB).
        0x18 m_pkDeallocChunk: u32,
        /// `m_pkEmptyChunk` (Xbox PDB).
        0x1C m_pkEmptyChunk: u32,
    }
}

/// `NiFixedAllocator::m_kCriticalSection` (Xbox PDB), +0x80: the Win32
/// critical section itself on PC.
const FIXED_CRITICAL_SECTION: u32 = 0x80;
/// `NiSmallObjectAllocator::m_stChunkSize` (Xbox PDB), +0x10000.
const SMALL_CHUNK_SIZE: u32 = 0x10000;

/// The `NiFixedAllocator` of the small-object allocator `small` for blocks of
/// `size` bytes (`small + size * 0x100 - 0x100`, 32-bit arithmetic).
fn fixed_allocator(small: u32, size: u32) -> u32 {
    (size << 8).wrapping_add(small).wrapping_sub(0x100)
}

/// Writes the size header (`*size`, byte by byte as the code does) at `block`
/// and answers the address after it.
fn put_size_header(e: &mut Engine, block: u32, size: u32) -> u32 {
    let bytes = size.to_le_bytes();
    for (i, b) in bytes.iter().enumerate() {
        e.mem.set_u8(block.wrapping_add(i as u32), *b);
    }
    block.wrapping_add(4)
}

// Translated from 00aa3090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStandardAllocator::GetSizeFromAddress` (Xbox PDB), `cdecl`, one word:
/// the size header at the address (read byte by byte).
pub fn ni_standard_allocator_get_size_from_address(e: &mut Engine, address: Ptr) -> u32 {
    let a = address.addr();
    u32::from_le_bytes([
        e.mem.u8(a),
        e.mem.u8(a.wrapping_add(1)),
        e.mem.u8(a.wrapping_add(2)),
        e.mem.u8(a.wrapping_add(3)),
    ])
}

// Translated from 00aa30c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStandardAllocator::Allocate` (Xbox PDB), `thiscall`, `RET 0x1c`: the
/// size and the alignment by address, the event type, whether the caller
/// gives the size back on deallocation, and the file, line and function
/// (not read). With an alignment of 4, no size given back and an event type
/// other than 7 and 8, the size grows by 4 for a size header. Event types
/// other than 7 and 8 with a size up to 0x100 and an alignment of 4 take a
/// block from the small-object allocator's `NiFixedAllocator` for that size
/// (`00aa3810`); everything else goes to `00aa1130(size, alignment)`. A
/// block that got a header has the (grown) size written there and is
/// answered past it.
#[allow(clippy::too_many_arguments)]
pub fn ni_standard_allocator_allocate(
    e: &mut Engine,
    _this: Ptr,
    size: Ptr,
    alignment: Ptr,
    event: u32,
    accurate_size: u8,
    _file: u32,
    _line: u32,
    _function: u32,
) -> Ptr {
    let size = size.addr();
    let alignment = alignment.addr();
    let mut header = false;
    if e.mem.u32(alignment) == 4 && accurate_size == 0 && event != 7 && event != 8 {
        let grown = e.mem.u32(size).wrapping_add(4);
        e.mem.set_u32(size, grown);
        header = true;
    }
    let block = if event != 7 && event != 8 && e.mem.u32(size) <= 0x100 && e.mem.u32(alignment) == 4
    {
        let small = e.mem.u32(SMALL_ALLOCATOR);
        let wanted = e.mem.u32(size);
        fn_00aa3810(e, Ptr::new(small), wanted).addr()
    } else {
        let wanted = e.mem.u32(size);
        let align = e.mem.u32(alignment);
        e.call(LARGE_ALLOCATE, &args![wanted, align]).u32()
    };
    if block != 0 && header {
        let total = e.mem.u32(size);
        return Ptr::new(put_size_header(e, block, total));
    }
    Ptr::new(block)
}

// Translated from 00aa3150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStandardAllocator::Deallocate` (Xbox PDB), `thiscall`, `RET 0xc`: the
/// block, the event type and the size (-1 when not known). A null block does
/// nothing. For event types other than 10 and 8 with an unknown size, the
/// block moves back to its size header and the size is read from it. Sizes
/// up to 0x100 go back to the small-object allocator (`00aa3830`), larger
/// ones to `00aa1190`.
pub fn ni_standard_allocator_deallocate(
    e: &mut Engine,
    _this: Ptr,
    block: Ptr,
    event: u32,
    size: u32,
) {
    let mut block = block.addr();
    if block == 0 {
        return;
    }
    let mut size = size;
    if event != 10 && event != 8 && size == 0xffff_ffff {
        block = block.wrapping_sub(4);
        size = ni_standard_allocator_get_size_from_address(e, Ptr::new(block));
    }
    if size <= 0x100 {
        let small = e.mem.u32(SMALL_ALLOCATOR);
        fn_00aa3830(e, Ptr::new(small), Ptr::new(block), size);
        return;
    }
    e.call(LARGE_FREE, &args![block]);
}

// Translated from 00aa31b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStandardAllocator::Reallocate` (Xbox PDB), `thiscall`, `RET 0x24`: the
/// block, the size and the alignment by address, the event type, whether
/// the caller gives sizes back, the previous size (-1 when not known) and
/// the file, line and function. For event types other than 10 and 8 with an
/// unknown previous size and no size given back, the block moves back to its
/// header and the previous size is read from it. With an alignment of 4, no
/// size given back and an event type other than 7 and 8 the new size grows
/// by 4; when that grown size fits in the known previous size, it becomes
/// the previous size and the same block is answered (past its header).
/// Event types 7 and 8, or both sizes above 0x100, go to
/// `00aa1250(block, size, alignment)`; otherwise the size loses the 4 again,
/// a new block comes from the allocator's own `Allocate` (vtable +4) and,
/// when the previous size is known, the old contents after the header
/// (previous size - 4 bytes) are copied into it and the old block is freed
/// through `Deallocate` (vtable +8) with the event type and previous size;
/// such a block gets no header here. A block that keeps its header flag has
/// the size written there and is answered past it.
#[allow(clippy::too_many_arguments)]
pub fn ni_standard_allocator_reallocate(
    e: &mut Engine,
    this: Ptr,
    block: Ptr,
    size: Ptr,
    alignment: Ptr,
    event: u32,
    accurate_size: u8,
    previous_size: u32,
    file: u32,
    line: u32,
    function: u32,
) -> Ptr {
    let mut block = block.addr();
    let size = size.addr();
    let mut previous = previous_size;
    if event != 10 && event != 8 && previous == 0xffff_ffff && accurate_size == 0 {
        block = block.wrapping_sub(4);
        previous = ni_standard_allocator_get_size_from_address(e, Ptr::new(block));
    }
    let mut header = false;
    if e.mem.u32(alignment.addr()) == 4 && accurate_size == 0 && event != 7 && event != 8 {
        let grown = e.mem.u32(size).wrapping_add(4);
        e.mem.set_u32(size, grown);
        header = true;
    }
    if previous != 0xffff_ffff && e.mem.u32(size) <= previous && header {
        e.mem.set_u32(size, previous);
        return Ptr::new(block.wrapping_add(4));
    }
    let result = if event == 7 || event == 8 || (previous > 0x100 && e.mem.u32(size) > 0x100) {
        let wanted = e.mem.u32(size);
        let align = e.mem.u32(alignment.addr());
        e.call(LARGE_REALLOCATE, &args![block, wanted, align]).u32()
    } else {
        let smaller = e.mem.u32(size).wrapping_add(0xffff_fffc);
        e.mem.set_u32(size, smaller);
        let fresh = e
            .vcall(
                this.addr(),
                ALLOCATE_SLOT,
                &args![
                    size,
                    alignment,
                    event,
                    u32::from(accurate_size),
                    file,
                    line,
                    function
                ],
            )
            .u32();
        header = false;
        if previous != 0xffff_ffff {
            e.call(
                MEMCPY,
                &args![fresh, block.wrapping_add(4), previous.wrapping_sub(4)],
            );
            e.vcall(this.addr(), DEALLOCATE_SLOT, &args![block, event, previous]);
        }
        fresh
    };
    if result != 0 && header {
        let total = e.mem.u32(size);
        return Ptr::new(put_size_header(e, result, total));
    }
    Ptr::new(result)
}

// Translated from 00aa3300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStandardAllocator::AllocateExternal` (Xbox PDB), `thiscall`, `RET 0x1c`
/// (as `Allocate`). Takes `size` bytes from the memory manager (`00aa3e40`)
/// and clears them (`memset`, whose result is the answer). Event type 0xd
/// (an aligned request) asks for `size + alignment` bytes (the alignment's
/// low byte), moves the block up to the next multiple of the alignment
/// (`alignment - (block & (alignment - 1))` bytes, always at least one) and
/// keeps that distance in the byte before the answer.
#[allow(clippy::too_many_arguments)]
pub fn ni_standard_allocator_allocate_external(
    e: &mut Engine,
    _this: Ptr,
    size: Ptr,
    alignment: Ptr,
    event: u32,
    _accurate_size: u8,
    _file: u32,
    _line: u32,
    _function: u32,
) -> Ptr {
    let wanted = e.mem.u32(size.addr());
    if event != 0xd {
        let block = e
            .call(
                MEMORY_MANAGER_ALLOCATE,
                &args![MEMORY_MANAGER_OBJECT, wanted],
            )
            .u32();
        let count = e.mem.u32(size.addr());
        return e.call(MEMSET, &args![block, 0u32, count]).ptr();
    }
    let align = e.mem.u8(alignment.addr());
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, u32::from(align).wrapping_add(wanted)],
        )
        .u32();
    let shift = align.wrapping_sub(align.wrapping_sub(1) & block as u8);
    let aligned = block.wrapping_add(u32::from(shift));
    e.mem.set_u8(aligned.wrapping_sub(1), shift);
    let count = e.mem.u32(size.addr());
    e.call(MEMSET, &args![aligned, 0u32, count]).ptr()
}

// Translated from 00aa3370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStandardAllocator::DeallocateExternal` (Xbox PDB), `thiscall`,
/// `RET 0xc`: the block, the event type and the size (not read). A null
/// block does nothing; for event type 0x10 (an aligned block) the block
/// moves back by the distance kept in the byte before it; then the memory
/// manager frees it (`00aa4060`).
pub fn ni_standard_allocator_deallocate_external(
    e: &mut Engine,
    _this: Ptr,
    block: Ptr,
    event: u32,
    _size: u32,
) {
    let mut block = block.addr();
    if block == 0 {
        return;
    }
    if event == 0x10 {
        let shift = e.mem.u8(block.wrapping_sub(1));
        block = block.wrapping_sub(u32::from(shift));
    }
    e.call(
        MEMORY_MANAGER_DEALLOCATE,
        &args![MEMORY_MANAGER_OBJECT, block],
    );
}

// Translated from 00aa33a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStandardAllocator::ReallocateExternal` (Xbox PDB), `thiscall`,
/// `RET 0x24` (as `Reallocate`). Has the memory manager reallocate the block
/// to `size` bytes (`00aa4150`) and clears the new block's `size` bytes
/// (`memset`, whose result is the answer). Event type 0xe (an aligned block)
/// moves the block back by the distance in the byte before it first, asks for
/// `size + alignment` bytes and aligns the result as `AllocateExternal` does;
/// a null block with that event type only clears `size` bytes at address 0
/// (`memset(0, 0, size)`).
#[allow(clippy::too_many_arguments)]
pub fn ni_standard_allocator_reallocate_external(
    e: &mut Engine,
    _this: Ptr,
    block: Ptr,
    size: Ptr,
    alignment: Ptr,
    event: u32,
    _accurate_size: u8,
    _previous_size: u32,
    _file: u32,
    _line: u32,
    _function: u32,
) -> Ptr {
    let block = block.addr();
    if event != 0xe {
        let wanted = e.mem.u32(size.addr());
        let fresh = e
            .call(
                MEMORY_MANAGER_REALLOCATE,
                &args![MEMORY_MANAGER_OBJECT, block, wanted],
            )
            .u32();
        let count = e.mem.u32(size.addr());
        return e.call(MEMSET, &args![fresh, 0u32, count]).ptr();
    }
    let wanted = e.mem.u32(size.addr());
    let align = e.mem.u8(alignment.addr());
    if block == 0 {
        let count = e.mem.u32(size.addr());
        return e.call(MEMSET, &args![0u32, 0u32, count]).ptr();
    }
    let shift = e.mem.u8(block.wrapping_sub(1));
    let start = block.wrapping_sub(u32::from(shift));
    let fresh = e
        .call(
            MEMORY_MANAGER_REALLOCATE,
            &args![
                MEMORY_MANAGER_OBJECT,
                start,
                u32::from(align).wrapping_add(wanted)
            ],
        )
        .u32();
    let shift = align.wrapping_sub(align.wrapping_sub(1) & fresh as u8);
    let aligned = fresh.wrapping_add(u32::from(shift));
    e.mem.set_u8(aligned.wrapping_sub(1), shift);
    let count = e.mem.u32(size.addr());
    e.call(MEMSET, &args![aligned, 0u32, count]).ptr()
}

// Translated from 00aa3430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStandardAllocator::Initialize` (Xbox PDB), `thiscall`, no stack words:
/// takes 0x10080 bytes from the memory manager for the small-object
/// allocator and constructs it there with a chunk size of 0x6400
/// (`00aa3760`), keeping it in `ms_pkSmallAlloc`; a failed allocation
/// leaves `ms_pkSmallAlloc` null.
pub fn ni_standard_allocator_initialize(e: &mut Engine, _this: Ptr) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x10080u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(SMALL_ALLOCATOR, 0);
        return;
    }
    let small = fn_00aa3760(e, Ptr::new(block), 0x6400);
    e.mem.set_u32(SMALL_ALLOCATOR, small.addr());
}

// Translated from 00aa3460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStandardAllocator::Shutdown` (Xbox PDB), `thiscall`, no stack words:
/// when there is a small-object allocator, destroys its 256
/// `NiFixedAllocator`s from the last to the first (`00aa3560`) and gives its
/// memory back to the memory manager; `ms_pkSmallAlloc` is cleared in any
/// case.
pub fn ni_standard_allocator_shutdown(e: &mut Engine, _this: Ptr) {
    let small = e.mem.u32(SMALL_ALLOCATOR);
    if small != 0 {
        let mut fixed = small.wrapping_add(0x10000);
        for _ in 0..0x100 {
            fixed = fixed.wrapping_sub(0x100);
            fn_00aa3560(e, Ptr::new(fixed));
        }
        e.call(
            MEMORY_MANAGER_DEALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, small],
        );
    }
    e.mem.set_u32(SMALL_ALLOCATOR, 0);
}

// Translated from 00aa3560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiFixedAllocator` destructor body
/// (`thiscall`, no stack words). Frees each chunk's data (the first word of
/// each 8-byte chunk, `00aa1190`), then the chunk array (`00aa1170`), then
/// deletes the critical section at +0x80.
pub fn fn_00aa3560(e: &mut Engine, this: Ptr<NiFixedAllocator>) {
    let mut i = 0u32;
    while i < e.get(this, NiFixedAllocator::m_stNumChunks) {
        let chunks = e.get(this, NiFixedAllocator::m_pkChunks);
        let data = e.mem.u32(chunks.wrapping_add(i.wrapping_mul(8)));
        e.call(LARGE_FREE, &args![data]);
        i = i.wrapping_add(1);
    }
    let chunks = e.get(this, NiFixedAllocator::m_pkChunks);
    e.call(FREE_CHUNKS, &args![chunks]);
    e.call(
        API_DELETE_CRITICAL_SECTION,
        &args![this.addr().wrapping_add(FIXED_CRITICAL_SECTION)],
    );
}

// Translated from 00aa3760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiSmallObjectAllocator` constructor
/// (`thiscall`, `RET 4`: the chunk size). Clears each of the 256
/// `NiFixedAllocator`s' block size, chunk array, chunk counts and chunk
/// pointers and initializes its critical section; stores the chunk size at
/// +0x10000; then sets allocator `i - 1` (for `i` from 1 to 0x100) to blocks
/// of `i` bytes, `0x6400 / i` blocks a chunk (0xff at most, `i * 8` if that
/// came to 0), clearing its chunk array, counts and allocation chunk again.
/// Answers `this`.
pub fn fn_00aa3760(e: &mut Engine, this: Ptr, chunk_size: u32) -> Ptr {
    let base = this.addr();
    for k in 0..0x100u32 {
        let fixed: Ptr<NiFixedAllocator> = Ptr::new(base.wrapping_add(k << 8));
        e.set(fixed, NiFixedAllocator::m_stBlockSize, 0);
        e.set(fixed, NiFixedAllocator::m_pkChunks, 0);
        e.set(fixed, NiFixedAllocator::m_stNumChunks, 0);
        e.set(fixed, NiFixedAllocator::m_stMaxNumChunks, 0);
        e.set(fixed, NiFixedAllocator::m_pkAllocChunk, 0);
        e.set(fixed, NiFixedAllocator::m_pkDeallocChunk, 0);
        e.set(fixed, NiFixedAllocator::m_pkEmptyChunk, 0);
        e.call(
            API_INITIALIZE_CRITICAL_SECTION,
            &args![fixed.addr().wrapping_add(FIXED_CRITICAL_SECTION)],
        );
    }
    e.mem
        .set_u32(base.wrapping_add(SMALL_CHUNK_SIZE), chunk_size);
    for i in 1..=0x100u32 {
        let fixed: Ptr<NiFixedAllocator> = Ptr::new(base.wrapping_add((i - 1) << 8));
        let per_chunk = 0x6400 / i;
        e.set(fixed, NiFixedAllocator::m_stNumChunks, 0);
        e.set(fixed, NiFixedAllocator::m_stMaxNumChunks, 0);
        e.set(fixed, NiFixedAllocator::m_pkChunks, 0);
        e.set(fixed, NiFixedAllocator::m_pkAllocChunk, 0);
        e.set(fixed, NiFixedAllocator::m_stBlockSize, i);
        let blocks = if per_chunk > 0xff {
            0xff
        } else if per_chunk == 0 {
            i.wrapping_mul(8)
        } else {
            per_chunk
        };
        e.set(fixed, NiFixedAllocator::m_ucNumBlocks, blocks as u8);
    }
    this
}

// Translated from 00aa3810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiSmallObjectAllocator`'s allocate
/// (`thiscall`, `RET 4`: the size). Takes a block from the
/// `NiFixedAllocator` for that size (`00aa35a0`) and answers its result.
pub fn fn_00aa3810(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    let fixed = fixed_allocator(this.addr(), size);
    e.call(FIXED_ALLOCATE, &args![fixed]).ptr()
}

// Translated from 00aa3830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiSmallObjectAllocator`'s deallocate
/// (`thiscall`, `RET 8`: the block and its size). Inside the critical section
/// of the `NiFixedAllocator` for that size: finds the block's chunk
/// (`00aa34b0`), keeps it as `m_pkDeallocChunk` and gives the block back
/// (`00aa36f0`).
pub fn fn_00aa3830(e: &mut Engine, this: Ptr, block: Ptr, size: u32) {
    let fixed = fixed_allocator(this.addr(), size);
    let section = fixed.wrapping_add(FIXED_CRITICAL_SECTION);
    e.call(API_ENTER_CRITICAL_SECTION, &args![section]);
    let chunk = e.call(FIXED_FIND_CHUNK, &args![fixed, block]).u32();
    let fixed_ptr: Ptr<NiFixedAllocator> = Ptr::new(fixed);
    e.set(fixed_ptr, NiFixedAllocator::m_pkDeallocChunk, chunk);
    e.call(FIXED_DEALLOCATE, &args![fixed, block]);
    e.call(API_LEAVE_CRITICAL_SECTION, &args![section]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa_3090,
            ni_standard_allocator_get_size_from_address(Ptr) -> u32
        ),
        entry!(
            0x00aa_30c0,
            ni_standard_allocator_allocate(Ptr, Ptr, Ptr, u32, u8, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x00aa_3150,
            ni_standard_allocator_deallocate(Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00aa_31b0,
            ni_standard_allocator_reallocate(Ptr, Ptr, Ptr, Ptr, u32, u8, u32, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x00aa_3300,
            ni_standard_allocator_allocate_external(Ptr, Ptr, Ptr, u32, u8, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x00aa_3370,
            ni_standard_allocator_deallocate_external(Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00aa_33a0,
            ni_standard_allocator_reallocate_external(Ptr, Ptr, Ptr, Ptr, u32, u8, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00aa_3430, ni_standard_allocator_initialize(Ptr)),
        entry!(0x00aa_3460, ni_standard_allocator_shutdown(Ptr)),
        entry!(0x00aa_3560, fn_00aa3560(Ptr<NiFixedAllocator>)),
        entry!(0x00aa_3760, fn_00aa3760(Ptr, u32) -> Ptr),
        entry!(0x00aa_3810, fn_00aa3810(Ptr, u32) -> Ptr),
        entry!(0x00aa_3830, fn_00aa3830(Ptr, Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with every callee doubled: the allocators answer 0x5000
    /// (the small one 0x6000), `memset` answers its destination, the small
    /// allocator sits at 0x40000.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(LARGE_ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(LARGE_REALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(FIXED_ALLOCATE, |_, _| 0x6000u32.into_ret());
        e.register(FIXED_FIND_CHUNK, |_, _| 0x7777u32.into_ret());
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0x5001u32.into_ret());
        e.register(MEMORY_MANAGER_REALLOCATE, |_, _| 0x5003u32.into_ret());
        e.register(MEMSET, |_, a| a[0].into_ret());
        for f in [
            LARGE_FREE,
            FREE_CHUNKS,
            FIXED_DEALLOCATE,
            MEMCPY,
            MEMORY_MANAGER_DEALLOCATE,
            API_INITIALIZE_CRITICAL_SECTION,
            API_DELETE_CRITICAL_SECTION,
            API_ENTER_CRITICAL_SECTION,
            API_LEAVE_CRITICAL_SECTION,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.map(SMALL_ALLOCATOR, 4);
        e.mem.set_u32(SMALL_ALLOCATOR, 0x40000);
        e.map(0x5000, 0x2000);
        e.map(0x40000, 0x10080);
        e.call_log = Some(vec![]);
        e
    }

    /// Two words for a size and an alignment.
    fn request(e: &mut Engine, size: u32, alignment: u32) -> (u32, u32) {
        let p = e.mem.alloc(8);
        e.mem.set_u32(p, size);
        e.mem.set_u32(p + 4, alignment);
        (p, p + 4)
    }

    #[test]
    fn ni_standard_allocator_get_size_from_address_reads_the_header() {
        let mut e = Engine::new();
        let p = e.mem.alloc(8);
        e.mem.set_u32(p, 0x1234_5678);
        assert_eq!(e.call(0x00aa_3090, &args![p]).u32(), 0x1234_5678);
    }

    #[test]
    fn ni_standard_allocator_allocate_small_with_a_header() {
        let mut e = engine();
        let (size, align) = request(&mut e, 0x10, 4);
        let r = e
            .call(
                0x00aa_30c0,
                &args![0u32, size, align, 1u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(r, 0x6004);
        assert_eq!(e.mem.u32(size), 0x14);
        assert_eq!(e.mem.u32(0x6000), 0x14);
        assert_eq!(
            calls(&e, FIXED_ALLOCATE),
            vec![vec![0x40000 + 0x1400 - 0x100]]
        );
    }

    #[test]
    fn ni_standard_allocator_allocate_large_and_event_7() {
        let mut e = engine();
        let (size, align) = request(&mut e, 0x200, 4);
        let r = e
            .call(
                0x00aa_30c0,
                &args![0u32, size, align, 1u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(r, 0x5004);
        assert_eq!(calls(&e, LARGE_ALLOCATE), vec![vec![0x204, 4]]);
        let (size, align) = request(&mut e, 0x10, 4);
        let r = e
            .call(
                0x00aa_30c0,
                &args![0u32, size, align, 7u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(r, 0x5000);
        assert_eq!(e.mem.u32(size), 0x10);
        // A size given back: no header, the small allocator.
        let (size, align) = request(&mut e, 0x10, 4);
        let r = e
            .call(
                0x00aa_30c0,
                &args![0u32, size, align, 1u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(r, 0x6000);
    }

    #[test]
    fn ni_standard_allocator_deallocate_by_header_and_by_size() {
        let mut e = engine();
        e.call(0x00aa_3150, &args![0u32, 0u32, 1u32, 0xffff_ffffu32]);
        assert!(calls(&e, LARGE_FREE).is_empty());
        e.mem.set_u32(0x5000, 0x14);
        e.call(0x00aa_3150, &args![0u32, 0x5004u32, 1u32, 0xffff_ffffu32]);
        assert_eq!(calls(&e, FIXED_FIND_CHUNK), vec![vec![0x41300, 0x5000]]);
        assert_eq!(calls(&e, FIXED_DEALLOCATE), vec![vec![0x41300, 0x5000]]);
        e.call(0x00aa_3150, &args![0u32, 0x5004u32, 10u32, 0x101u32]);
        assert_eq!(calls(&e, LARGE_FREE), vec![vec![0x5004]]);
    }

    #[test]
    fn ni_standard_allocator_reallocate_keeps_a_block_that_fits() {
        let mut e = engine();
        let (size, align) = request(&mut e, 0x8, 4);
        e.mem.set_u32(0x5000, 0x20);
        let r = e
            .call(
                0x00aa_31b0,
                &args![
                    0u32,
                    0x5004u32,
                    size,
                    align,
                    1u32,
                    0u32,
                    0xffff_ffffu32,
                    0u32,
                    0u32,
                    0u32
                ],
            )
            .u32();
        assert_eq!(r, 0x5004);
        assert_eq!(e.mem.u32(size), 0x20);
    }

    #[test]
    fn ni_standard_allocator_reallocate_moves_through_the_vtable() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 4, 0x00ab_0004);
        e.mem.set_u32(vtable + 8, 0x00ab_0008);
        e.register(0x00ab_0004, |_, _| 0x5800u32.into_ret());
        e.register(0x00ab_0008, |_, _| Ret::default());
        let (size, align) = request(&mut e, 0x40, 4);
        e.mem.set_u32(0x5000, 0x20);
        let r = e
            .call(
                0x00aa_31b0,
                &args![
                    this,
                    0x5004u32,
                    size,
                    align,
                    1u32,
                    0u32,
                    0xffff_ffffu32,
                    9u32,
                    8u32,
                    7u32
                ],
            )
            .u32();
        assert_eq!(r, 0x5800);
        assert_eq!(e.mem.u32(size), 0x40);
        assert_eq!(
            calls(&e, 0x00ab_0004),
            vec![vec![this, size, align, 1, 0, 9, 8, 7]]
        );
        assert_eq!(calls(&e, MEMCPY), vec![vec![0x5800, 0x5004, 0x1c]]);
        assert_eq!(calls(&e, 0x00ab_0008), vec![vec![this, 0x5000, 1, 0x20]]);
    }

    #[test]
    fn ni_standard_allocator_reallocate_large_writes_the_header() {
        let mut e = engine();
        let (size, align) = request(&mut e, 0x300, 4);
        let r = e
            .call(
                0x00aa_31b0,
                &args![0u32, 0x5004u32, size, align, 10u32, 0u32, 0x200u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(r, 0x5004);
        assert_eq!(calls(&e, LARGE_REALLOCATE), vec![vec![0x5004, 0x304, 4]]);
        assert_eq!(e.mem.u32(0x5000), 0x304);
    }

    #[test]
    fn ni_standard_allocator_allocate_external_plain_and_aligned() {
        let mut e = engine();
        let (size, align) = request(&mut e, 0x30, 0x10);
        let r = e
            .call(
                0x00aa_3300,
                &args![0u32, size, align, 1u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(r, 0x5001);
        assert_eq!(calls(&e, MEMSET), vec![vec![0x5001, 0, 0x30]]);
        let r = e
            .call(
                0x00aa_3300,
                &args![0u32, size, align, 0xdu32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(r, 0x5010);
        assert_eq!(e.mem.u8(0x500f), 0xf);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_ALLOCATE)[1],
            vec![MEMORY_MANAGER_OBJECT, 0x40]
        );
    }

    #[test]
    fn ni_standard_allocator_deallocate_external_aligned_moves_back() {
        let mut e = engine();
        e.call(0x00aa_3370, &args![0u32, 0u32, 0x10u32, 0u32]);
        assert!(calls(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        e.mem.set_u8(0x500f, 0xf);
        e.call(0x00aa_3370, &args![0u32, 0x5010u32, 0x10u32, 0u32]);
        e.call(0x00aa_3370, &args![0u32, 0x5010u32, 1u32, 0u32]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![
                vec![MEMORY_MANAGER_OBJECT, 0x5001],
                vec![MEMORY_MANAGER_OBJECT, 0x5010]
            ]
        );
    }

    #[test]
    fn ni_standard_allocator_reallocate_external_each_branch() {
        let mut e = engine();
        let (size, align) = request(&mut e, 0x30, 0x10);
        let r = e
            .call(
                0x00aa_33a0,
                &args![0u32, 0x5010u32, size, align, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(r, 0x5003);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_REALLOCATE),
            vec![vec![MEMORY_MANAGER_OBJECT, 0x5010, 0x30]]
        );
        let r = e
            .call(
                0x00aa_33a0,
                &args![0u32, 0u32, size, align, 0xeu32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(r, 0);
        e.mem.set_u8(0x500f, 0xf);
        let r = e
            .call(
                0x00aa_33a0,
                &args![0u32, 0x5010u32, size, align, 0xeu32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(r, 0x5010);
        assert_eq!(e.mem.u8(0x500f), 0xd);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_REALLOCATE)[1],
            vec![MEMORY_MANAGER_OBJECT, 0x5001, 0x40]
        );
        assert_eq!(calls(&e, MEMSET)[1], vec![0, 0, 0x30]);
    }

    #[test]
    fn ni_standard_allocator_initialize_and_shutdown() {
        let mut e = engine();
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0x10_0000u32.into_ret());
        e.map(0x10_0000, 0x10080);
        e.call(0x00aa_3430, &args![0u32]);
        assert_eq!(e.mem.u32(SMALL_ALLOCATOR), 0x10_0000);
        assert_eq!(e.mem.u32(0x11_0000), 0x6400);
        e.call(0x00aa_3460, &args![0u32]);
        assert_eq!(e.mem.u32(SMALL_ALLOCATOR), 0);
        let deleted = calls(&e, API_DELETE_CRITICAL_SECTION);
        assert_eq!(deleted.len(), 0x100);
        assert_eq!(deleted[0], vec![0x10_ff80]);
        assert_eq!(deleted[0xff], vec![0x10_0080]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER_OBJECT, 0x10_0000]]
        );
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.mem.set_u32(SMALL_ALLOCATOR, 5);
        e.call(0x00aa_3430, &args![0u32]);
        assert_eq!(e.mem.u32(SMALL_ALLOCATOR), 0);
        e.call(0x00aa_3460, &args![0u32]);
        assert_eq!(calls(&e, API_DELETE_CRITICAL_SECTION).len(), 0x100);
    }

    #[test]
    fn fn_00aa3560_frees_each_chunk() {
        let mut e = engine();
        let fixed = e.mem.alloc(0x100);
        let chunks = e.mem.alloc(0x10);
        e.mem.set_u32(chunks, 0xa1);
        e.mem.set_u32(chunks + 8, 0xa2);
        e.mem.set_u32(fixed + 8, chunks);
        e.mem.set_u32(fixed + 0xc, 2);
        e.call(0x00aa_3560, &args![fixed]);
        assert_eq!(calls(&e, LARGE_FREE), vec![vec![0xa1], vec![0xa2]]);
        assert_eq!(calls(&e, FREE_CHUNKS), vec![vec![chunks]]);
        assert_eq!(
            calls(&e, API_DELETE_CRITICAL_SECTION),
            vec![vec![fixed + 0x80]]
        );
    }

    #[test]
    fn fn_00aa3760_sets_up_each_block_size() {
        let mut e = engine();
        let small = e.mem.alloc(0x10080);
        e.mem.set_u32(small + 0x1c, 9);
        e.mem.set_u8(small + 0x4, 9);
        assert_eq!(e.call(0x00aa_3760, &args![small, 0x6400u32]).u32(), small);
        assert_eq!(e.mem.u32(small + 0x10000), 0x6400);
        assert_eq!(e.mem.u32(small + 0x1c), 0);
        assert_eq!(e.mem.u32(small), 1);
        assert_eq!(e.mem.u8(small + 4), 0xff);
        assert_eq!(e.mem.u32(small + 0xff00), 0x100);
        assert_eq!(e.mem.u8(small + 0xff04), 0x64);
        assert_eq!(e.mem.u8(small + 0x6404), 0xfd);
        assert_eq!(calls(&e, API_INITIALIZE_CRITICAL_SECTION).len(), 0x100);
    }

    #[test]
    fn fn_00aa3810_and_fn_00aa3830_pick_the_allocator_for_the_size() {
        let mut e = engine();
        assert_eq!(e.call(0x00aa_3810, &args![0x40000u32, 2u32]).u32(), 0x6000);
        assert_eq!(calls(&e, FIXED_ALLOCATE), vec![vec![0x40100]]);
        e.call(0x00aa_3830, &args![0x40000u32, 0x9000u32, 3u32]);
        assert_eq!(calls(&e, API_ENTER_CRITICAL_SECTION), vec![vec![0x40280]]);
        assert_eq!(calls(&e, FIXED_FIND_CHUNK), vec![vec![0x40200, 0x9000]]);
        assert_eq!(e.mem.u32(0x40218), 0x7777);
        assert_eq!(calls(&e, FIXED_DEALLOCATE), vec![vec![0x40200, 0x9000]]);
        assert_eq!(calls(&e, API_LEAVE_CRITICAL_SECTION), vec![vec![0x40280]]);
    }
}

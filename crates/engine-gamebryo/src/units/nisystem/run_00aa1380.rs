//! `NiSystem/run_00aa1380` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Gamebryo allocator reference: the word at `011f6080` points to a word
/// that holds the allocator object (set up by `00aa2020`).
const ALLOCATOR_REFERENCE: u32 = 0x011f_6080;
/// The allocator's allocation method, byte offset in its vtable.
const ALLOCATE_SLOT: u32 = 0x4;

// Translated from 00aa13e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates `size` bytes (a request for 0 is made for 1) through the
/// Gamebryo allocator's virtual allocation method (vtable `+4`), the sibling
/// of `00aa1070`; it passes 1, 1 where that passes 5, 0. The size and an
/// word holding 4 are passed by address, as
/// locals of this function. Returns the allocator's result (`EAX`), which the
/// decompiler does not show: callers use it as the new block.
///
/// Arguments of the call, in order: size address, address of the 4, 1, 1,
/// 0, -1, 0 (their names are not known).
pub fn fn_00aa13e0(e: &mut Engine, size: u32) -> Ptr {
    let size = if size == 0 { 1 } else { size };
    let reference = e.mem.u32(ALLOCATOR_REFERENCE);
    let allocator = e.mem.u32(reference);
    e.with_stack(8, |e, locals| {
        e.mem.set_u32(locals.addr(), size);
        e.mem.set_u32(locals.addr() + 4, 4);
        e.vcall(
            allocator,
            ALLOCATE_SLOT,
            &args![
                locals.addr(),
                locals.addr() + 4,
                1u32,
                1u32,
                0u32,
                0xffff_ffffu32,
                0u32
            ],
        )
        .ptr()
    })
}

/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// Import slot of `GetSystemInfo` (`stdcall`, one word: a 0x24-byte `SYSTEM_INFO`).
const API_GET_SYSTEM_INFO: u32 = 0x00fd_f0f8;
/// Import slot of `QueryPerformanceFrequency` (`stdcall`, one word: a 64-bit integer).
const API_QUERY_PERFORMANCE_FREQUENCY: u32 = 0x00fd_f0a4;
/// `011f6078`: where `00aa1920` keeps the 8-byte object it builds.
const SYSTEM_FACTS_SLOT: u32 = 0x011f_6078;

// Translated from 00aa1920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments; run by `fn_00aa3050`): allocates 8
/// bytes from the memory manager; when that succeeds it stores in them the processor count
/// (`dwNumberOfProcessors`, `+0x14` of the `SYSTEM_INFO` `GetSystemInfo` fills) and the
/// performance counter frequency (`QueryPerformanceFrequency`) as a `float`, and keeps the
/// block in `011f6078` (null when the allocation fails).
pub fn fn_00aa1920(e: &mut Engine) {
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER_OBJECT, 8u32])
        .u32();
    if block == 0 {
        e.mem.set_u32(SYSTEM_FACTS_SLOT, 0);
        return;
    }
    e.with_stack(0x30, |e, frame| {
        let base = frame.addr();
        e.call(API_GET_SYSTEM_INFO, &args![base + 0xc]);
        let processors = e.mem.u32(base + 0x20);
        e.mem.set_u32(block, processors);
        e.call(API_QUERY_PERFORMANCE_FREQUENCY, &args![base + 4]);
        // `FILD qword` then `FSTP float`: one rounding, from the exact integer.
        let low = u64::from(e.mem.u32(base + 4));
        let high = u64::from(e.mem.u32(base + 8));
        let frequency = (high << 32 | low) as i64;
        e.mem.set_u32(block + 4, (frequency as f32).to_bits());
    });
    e.mem.set_u32(SYSTEM_FACTS_SLOT, block);
}

// Translated from 00aa1380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): normalises a path in place.
/// Each `/` becomes `\`, and a `\` that follows a written `\` is dropped
/// from index 2 on (so a leading `\\` survives); the result is terminated
/// where the last written byte ends. A null `path` does nothing.
pub fn fn_00aa1380(e: &mut Engine, path: Ptr) {
    if path.is_null() {
        return;
    }
    let base = path.addr();
    let mut length = 0u32;
    while e.mem.u8(base.wrapping_add(length)) != 0 {
        length += 1;
    }
    let mut written = 0u32;
    let mut after_backslash = false;
    for index in 0..length {
        let mut byte = e.mem.u8(base.wrapping_add(index));
        if byte == b'/' {
            byte = b'\\';
        }
        let is_backslash = byte == b'\\';
        if !(is_backslash && after_backslash && index > 1) {
            e.mem.set_u8(base.wrapping_add(written), byte);
            after_backslash = is_backslash;
            written += 1;
        }
    }
    e.mem.set_u8(base.wrapping_add(written), 0);
}

/// The allocator's deallocation method, byte offset in its vtable.
const DEALLOCATE_SLOT: u32 = 0x8;

/// The allocator object behind [`ALLOCATOR_REFERENCE`].
fn allocator(e: &Engine) -> u32 {
    let reference = e.mem.u32(ALLOCATOR_REFERENCE);
    e.mem.u32(reference)
}

// Translated from 00aa1420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word), the sibling of `00aa13e0`:
/// allocates `size` bytes (0 is asked as 1) through the Gamebryo
/// allocator's vtable +4 with the size and a word holding 4 passed by
/// address, then 2, 0, 0, -1, 0 (where `00aa13e0` passes 1, 1, 0, -1, 0).
/// Returns the allocator's result (`EAX`, which the decompiler drops).
pub fn fn_00aa1420(e: &mut Engine, size: u32) -> Ptr {
    let size = if size == 0 { 1 } else { size };
    let allocator = allocator(e);
    e.with_stack(8, |e, locals| {
        e.mem.set_u32(locals.addr(), size);
        e.mem.set_u32(locals.addr() + 4, 4);
        e.vcall(
            allocator,
            ALLOCATE_SLOT,
            &args![
                locals.addr(),
                locals.addr() + 4,
                2u32,
                0u32,
                0u32,
                0xffff_ffffu32,
                0u32
            ],
        )
        .ptr()
    })
}

// Translated from 00aa1460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): frees `block` through the
/// Gamebryo allocator's vtable +8 with `(block, 3, size)`; a null `block`
/// does nothing.
pub fn fn_00aa1460(e: &mut Engine, block: Ptr, size: u32) {
    if block.is_null() {
        return;
    }
    let allocator = allocator(e);
    e.vcall(allocator, DEALLOCATE_SLOT, &args![block, 3u32, size]);
}

// Translated from 00aa1490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): frees `block` through the
/// Gamebryo allocator's vtable +8 with `(block, 4, -1)`; a null `block`
/// does nothing.
pub fn fn_00aa1490(e: &mut Engine, block: Ptr) {
    if block.is_null() {
        return;
    }
    let allocator = allocator(e);
    e.vcall(
        allocator,
        DEALLOCATE_SLOT,
        &args![block, 4u32, 0xffff_ffffu32],
    );
}

layout! {
    /// `NiFile` (Xbox PDB), 0x30 bytes on PC (the Xbox build's is 0x74; on
    /// PC `BSFile`'s own fields start at +0x30). The names are the PDB's;
    /// the offsets are the PC code's, which differ from +0x24 on (the PDB's
    /// `m_eFileMethod`, `m_bUseDoubleBuffer`, `m_hFile` and the overlapped
    /// fields are not in the PC class).
    pub struct NiFile: 0x30 {
        /// `NiBinaryStream::m_uiAbsoluteCurrentPos` (Xbox PDB).
        0x04 m_uiAbsoluteCurrentPos: u32,
        /// `m_uiBufferAllocSize` (Xbox PDB): the buffer's size.
        0x10 m_uiBufferAllocSize: u32,
        /// `m_uiBufferReadSize` (Xbox PDB): bytes read into the buffer.
        0x14 m_uiBufferReadSize: u32,
        /// `m_uiPos` (Xbox PDB): position in the buffer.
        0x18 m_uiPos: u32,
        /// `m_uiCurrentFilePos` (Xbox PDB).
        0x1C m_uiCurrentFilePos: u32,
        /// `m_pBuffer` (Xbox PDB).
        0x20 m_pBuffer: u32,
        /// `m_pFile` (Xbox PDB, +0x2c there): the CRT `FILE`.
        0x24 m_pFile: u32,
        /// `m_eMode` (Xbox PDB, +0x6c there): 0 reads (what the code treats
        /// as the read mode; the other values write).
        0x28 m_eMode: u32,
        /// `m_bGood` (Xbox PDB, +0x70 there).
        0x2C m_bGood: u8,
    }
}

/// The `NiFile` vtable (RTTI `.?AVNiFile@@`).
const NI_FILE_VTABLE: u32 = 0x010a_2490;
/// `00aa1d10` (thiscall, no stack words): the base constructor `00aa14b0`
/// runs first (placed in `NiXenonRenderer` by the engine map, 20 bytes).
const BINARY_STREAM_CONSTRUCTOR: u32 = 0x00aa_1d10;
/// `00aa1d30` (thiscall, no stack words): the base destructor `00aa1690`
/// ends with (7 bytes).
const BINARY_STREAM_DESTRUCTOR: u32 = 0x00aa_1d30;
/// `00aa8610` (cdecl: buffer, element size, count, `FILE`): the read
/// (`fread`'s argument order).
const FILE_READ: u32 = 0x00aa_8610;
/// `00aa8660` (cdecl: buffer, element size, count, `FILE`): the write.
const FILE_WRITE: u32 = 0x00aa_8660;
/// `00aa85c0` (cdecl: `FILE`, offset, origin): the seek, 0 on success.
const FILE_SEEK: u32 = 0x00aa_85c0;
/// `fclose` (`00ec9907`, cdecl).
const FCLOSE: u32 = 0x00ec_9907;
/// `ftell` (`00ecb562`, cdecl).
const FTELL: u32 = 0x00ec_b562;
/// `memcpy` (`00ec44d0`, cdecl: destination, source, size).
const MEMCPY: u32 = 0x00ec_44d0;
/// `00aa10f0` (cdecl, one word): frees the buffer.
const FREE_BUFFER: u32 = 0x00aa_10f0;
/// `00aa4060` (thiscall on the memory manager, one word): its deallocate.
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00aa14b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an `NiFile` constructor part. Runs the base
/// constructor `00aa1d10` on `this`, clears `m_pBuffer`, `m_pFile` and
/// `m_uiCurrentFilePos`, then installs the `NiFile` vtable. Returns `this`.
pub fn fn_00aa14b0(e: &mut Engine, this: Ptr<NiFile>) -> Ptr<NiFile> {
    e.call(BINARY_STREAM_CONSTRUCTOR, &args![this]);
    e.set(this, NiFile::m_pBuffer, 0);
    e.set(this, NiFile::m_pFile, 0);
    e.set(this, NiFile::m_uiCurrentFilePos, 0);
    e.mem.set_u32(this.addr(), NI_FILE_VTABLE);
    this
}

// Translated from 00aa1570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): reads `count` bytes of
/// the file straight into `buffer` (`00aa8610(buffer, 1, count, m_pFile)`),
/// adds what was read to `m_uiCurrentFilePos` and returns it (`EAX`, which
/// the decompiler drops; `BSFile::CheckIsGood` uses it).
pub fn fn_00aa1570(e: &mut Engine, this: Ptr<NiFile>, buffer: Ptr, count: u32) -> u32 {
    let file = e.get(this, NiFile::m_pFile);
    let read = e.call(FILE_READ, &args![buffer, 1u32, count, file]).u32();
    let position = e.get(this, NiFile::m_uiCurrentFilePos);
    e.set(
        this,
        NiFile::m_uiCurrentFilePos,
        position.wrapping_add(read),
    );
    read
}

// Translated from 00aa15a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall): the buffer flush. In read mode
/// (`m_eMode` 0) the read size is dropped; otherwise the `m_uiPos` buffered
/// bytes are written (`00aa8660(m_pBuffer, 1, m_uiPos, m_pFile)`, skipped
/// when there are none), and a short write clears `m_bGood` and answers
/// false, keeping `m_uiPos`. Otherwise `m_uiPos` is cleared and the answer
/// is true.
pub fn fn_00aa15a0(e: &mut Engine, this: Ptr<NiFile>) -> bool {
    if e.get(this, NiFile::m_eMode) == 0 {
        e.set(this, NiFile::m_uiBufferReadSize, 0);
    } else {
        let pending = e.get(this, NiFile::m_uiPos);
        if pending != 0 {
            let buffer = e.get(this, NiFile::m_pBuffer);
            let file = e.get(this, NiFile::m_pFile);
            let written = e
                .call(FILE_WRITE, &args![buffer, 1u32, pending, file])
                .u32();
            if written != e.get(this, NiFile::m_uiPos) {
                e.set(this, NiFile::m_bGood, 0);
                return false;
            }
        }
    }
    e.set(this, NiFile::m_uiPos, 0);
    true
}

// Translated from 00aa1690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiFile` destructor body. Installs the
/// `NiFile` vtable; when the file is good and open, flushes it
/// (`00aa15a0`) and closes it (`fclose`); frees the buffer (`00aa10f0`) and
/// ends with the base destructor `00aa1d30` (a tail jump). `m_pFile` and
/// `m_pBuffer` keep their values.
pub fn fn_00aa1690(e: &mut Engine, this: Ptr<NiFile>) {
    e.mem.set_u32(this.addr(), NI_FILE_VTABLE);
    if e.get(this, NiFile::m_bGood) != 0 && e.get(this, NiFile::m_pFile) != 0 {
        fn_00aa15a0(e, this);
        let file = e.get(this, NiFile::m_pFile);
        e.call(FCLOSE, &args![file]);
    }
    let buffer = e.get(this, NiFile::m_pBuffer);
    e.call(FREE_BUFFER, &args![buffer]);
    e.call(BINARY_STREAM_DESTRUCTOR, &args![this]);
}

// Translated from 00aa16d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): the `NiFile` seek. Does
/// nothing unless the file is good. A relative seek (`origin` 1) that stays
/// inside the read buffer (`0 <= m_uiPos + offset < m_uiBufferReadSize`,
/// signed) only moves `m_uiPos` and `m_uiAbsoluteCurrentPos`. Otherwise, a
/// relative seek in read mode is first made relative to the file position
/// (`offset += m_uiPos - m_uiBufferReadSize`); then the buffer is flushed
/// (`00aa15a0`), the file seeked (`00aa85c0(m_pFile, offset, origin)`), and
/// `m_bGood` set to whether that answered 0; on success `ftell`'s answer
/// goes to `m_uiCurrentFilePos` and `m_uiAbsoluteCurrentPos`.
pub fn fn_00aa16d0(e: &mut Engine, this: Ptr<NiFile>, offset: i32, origin: i32) {
    if e.get(this, NiFile::m_bGood) == 0 {
        return;
    }
    let mut offset = offset;
    if origin == 1 {
        let position = e.get(this, NiFile::m_uiPos);
        let target = position.wrapping_add(offset as u32) as i32;
        if target >= 0 && target < e.get(this, NiFile::m_uiBufferReadSize) as i32 {
            let absolute = e.get(this, NiFile::m_uiAbsoluteCurrentPos);
            e.set(
                this,
                NiFile::m_uiAbsoluteCurrentPos,
                absolute.wrapping_add(offset as u32),
            );
            e.set(this, NiFile::m_uiPos, target as u32);
            return;
        }
        if e.get(this, NiFile::m_eMode) == 0 {
            let read_size = e.get(this, NiFile::m_uiBufferReadSize);
            offset = offset.wrapping_add(position.wrapping_sub(read_size) as i32);
        }
    }
    fn_00aa15a0(e, this);
    let file = e.get(this, NiFile::m_pFile);
    let failed = e.call(FILE_SEEK, &args![file, offset, origin]).u32();
    let good = failed == 0;
    e.set(this, NiFile::m_bGood, good as u8);
    if good {
        let file = e.get(this, NiFile::m_pFile);
        let position = e.call(FTELL, &args![file]).u32();
        e.set(this, NiFile::m_uiCurrentFilePos, position);
        e.set(this, NiFile::m_uiAbsoluteCurrentPos, position);
    }
}

// Translated from 00aa1750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): the buffered `NiFile`
/// read of `count` bytes into `buffer`; answers the bytes delivered (0 when
/// the file is not good). What the buffer holds past `m_uiPos` is copied
/// first; when that is not enough the buffer is flushed (`00aa15a0`), and a
/// remainder larger than the buffer is read straight into `buffer`
/// (`00aa8610`, counted in `m_uiCurrentFilePos`); otherwise the buffer is
/// refilled (`m_uiBufferAllocSize` bytes asked, the answer kept as
/// `m_uiBufferReadSize`) and as much of the remainder as it got is copied
/// out, moving `m_uiPos`.
pub fn fn_00aa1750(e: &mut Engine, this: Ptr<NiFile>, buffer: Ptr, count: u32) -> u32 {
    if e.get(this, NiFile::m_bGood) == 0 {
        return 0;
    }
    let mut destination = buffer.addr();
    let mut count = count;
    let position = e.get(this, NiFile::m_uiPos);
    let available = e
        .get(this, NiFile::m_uiBufferReadSize)
        .wrapping_sub(position);
    let mut delivered = 0u32;
    if count > available {
        if available != 0 {
            let source = e.get(this, NiFile::m_pBuffer).wrapping_add(position);
            e.call(MEMCPY, &args![destination, source, available]);
            destination = destination.wrapping_add(available);
            count = count.wrapping_sub(available);
            delivered = available;
        }
        fn_00aa15a0(e, this);
        let alloc_size = e.get(this, NiFile::m_uiBufferAllocSize);
        let file = e.get(this, NiFile::m_pFile);
        if count > alloc_size {
            let read = e
                .call(FILE_READ, &args![destination, 1u32, count, file])
                .u32();
            let file_position = e.get(this, NiFile::m_uiCurrentFilePos);
            e.set(
                this,
                NiFile::m_uiCurrentFilePos,
                file_position.wrapping_add(read),
            );
            return read.wrapping_add(delivered);
        }
        let own_buffer = e.get(this, NiFile::m_pBuffer);
        let read = e
            .call(FILE_READ, &args![own_buffer, 1u32, alloc_size, file])
            .u32();
        let file_position = e.get(this, NiFile::m_uiCurrentFilePos);
        e.set(
            this,
            NiFile::m_uiCurrentFilePos,
            file_position.wrapping_add(read),
        );
        e.set(this, NiFile::m_uiBufferReadSize, read);
        if read < count {
            count = read;
        }
    }
    let source = e
        .get(this, NiFile::m_pBuffer)
        .wrapping_add(e.get(this, NiFile::m_uiPos));
    e.call(MEMCPY, &args![destination, source, count]);
    let position = e.get(this, NiFile::m_uiPos);
    e.set(this, NiFile::m_uiPos, position.wrapping_add(count));
    delivered.wrapping_add(count)
}

// Translated from 00aa1810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): the buffered `NiFile`
/// write of `count` bytes from `buffer`; answers the bytes taken (0 when
/// the file is not good or the flush fails). What fits in the buffer's free
/// room is copied in first (the buffer is then full); when that is not
/// enough the buffer is flushed (`00aa15a0`), and a remainder of at least
/// the buffer's size is written straight to the file (`00aa8660`; not
/// counted in `m_uiCurrentFilePos`); otherwise the remainder is copied into
/// the buffer, moving `m_uiPos`.
pub fn fn_00aa1810(e: &mut Engine, this: Ptr<NiFile>, buffer: Ptr, count: u32) -> u32 {
    if e.get(this, NiFile::m_bGood) == 0 {
        return 0;
    }
    let mut source = buffer.addr();
    let mut count = count;
    let position = e.get(this, NiFile::m_uiPos);
    let room = e
        .get(this, NiFile::m_uiBufferAllocSize)
        .wrapping_sub(position);
    let mut taken = 0u32;
    if count > room {
        if room != 0 {
            let destination = e.get(this, NiFile::m_pBuffer).wrapping_add(position);
            e.call(MEMCPY, &args![destination, source, room]);
            let alloc_size = e.get(this, NiFile::m_uiBufferAllocSize);
            source = source.wrapping_add(room);
            count = count.wrapping_sub(room);
            taken = room;
            e.set(this, NiFile::m_uiPos, alloc_size);
        }
        if !fn_00aa15a0(e, this) {
            return 0;
        }
        if count >= e.get(this, NiFile::m_uiBufferAllocSize) {
            let file = e.get(this, NiFile::m_pFile);
            let written = e.call(FILE_WRITE, &args![source, 1u32, count, file]).u32();
            return written.wrapping_add(taken);
        }
    }
    let destination = e
        .get(this, NiFile::m_pBuffer)
        .wrapping_add(e.get(this, NiFile::m_uiPos));
    e.call(MEMCPY, &args![destination, source, count]);
    let position = e.get(this, NiFile::m_uiPos);
    e.set(this, NiFile::m_uiPos, position.wrapping_add(count));
    taken.wrapping_add(count)
}

// Translated from 00aa1980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): frees the 8-byte block
/// `00aa1920` keeps in `011f6078` through the memory manager's deallocate
/// (`00aa4060`). The slot keeps its value.
pub fn fn_00aa1980(e: &mut Engine) {
    let block = e.mem.u32(SYSTEM_FACTS_SLOT);
    e.call(
        MEMORY_MANAGER_DEALLOCATE,
        &args![MEMORY_MANAGER_OBJECT, block],
    );
}

/// `00aa1b30` (`thiscall` on an `NiBinaryStream`, one word): sets the
/// stream's read and write functions (to `00aa19a0` and `00aa19c0`).
const BINARY_STREAM_SET_FUNCTIONS: u32 = 0x00aa_1b30;
/// `00aa1070` (`cdecl`, the size): allocates the buffer.
const ALLOCATE_BUFFER: u32 = 0x00aa_1070;
/// CRT `fopen_s` (`00ec9a5e`, `cdecl`: the `FILE*` slot, the path, the mode).
const FOPEN_S: u32 = 0x00ec_9a5e;
/// CRT `fseek` (`00ecb65a`, `cdecl`: `FILE`, offset, origin).
const FSEEK: u32 = 0x00ec_b65a;
/// The open modes `"rb"`, `"wb"` and `"ab"` in `.rdata`.
const MODE_READ_TEXT: u32 = 0x0106_e79c;
const MODE_WRITE_TEXT: u32 = 0x0106_e798;
const MODE_APPEND_TEXT: u32 = 0x010a_24b0;
/// Import slots of `CreateDirectoryA` and `GetFileAttributesA` (`stdcall`).
const API_CREATE_DIRECTORY: u32 = 0x00fd_f0b8;
const API_GET_FILE_ATTRIBUTES: u32 = 0x00fd_f160;
/// `00aa1de0` (`cdecl`: buffer, byte count, component sizes, their count):
/// swaps the byte order of each component.
const SWAP_COMPONENTS: u32 = 0x00aa_1de0;

// Translated from 00aa14d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiFile` constructor (`thiscall`, `RET 0xc`:
/// the path, the open mode and the buffer size). Runs the base constructor
/// (`00aa1d10`), installs the `NiFile` vtable, sets the stream functions
/// (`00aa1b30` with 0), clears `m_uiCurrentFilePos`, keeps the mode and
/// opens the file with `fopen_s` in `"rb"` (mode 0), `"wb"` (mode 1) or
/// `"ab"` (any other). `m_bGood` says whether that gave a file. The buffer
/// size is kept and the buffer positions cleared; a good file with a
/// non-zero buffer size gets its buffer (`00aa1070`), any other a null one.
/// Answers `this`.
pub fn fn_00aa14d0(
    e: &mut Engine,
    this: Ptr<NiFile>,
    path: Ptr,
    mode: u32,
    buffer_size: u32,
) -> Ptr<NiFile> {
    e.call(BINARY_STREAM_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), NI_FILE_VTABLE);
    e.call(BINARY_STREAM_SET_FUNCTIONS, &args![this, 0u32]);
    e.set(this, NiFile::m_uiCurrentFilePos, 0);
    e.set(this, NiFile::m_eMode, mode);
    let text = match mode {
        0 => MODE_READ_TEXT,
        1 => MODE_WRITE_TEXT,
        _ => MODE_APPEND_TEXT,
    };
    let file_slot = this.addr().wrapping_add(0x24);
    let error = e.call(FOPEN_S, &args![file_slot, path, text]).u32();
    let good = error == 0 && e.get(this, NiFile::m_pFile) != 0;
    e.set(this, NiFile::m_bGood, u8::from(good));
    e.set(this, NiFile::m_uiBufferAllocSize, buffer_size);
    e.set(this, NiFile::m_uiBufferReadSize, 0);
    e.set(this, NiFile::m_uiPos, 0);
    if good && buffer_size != 0 {
        let buffer = e.call(ALLOCATE_BUFFER, &args![buffer_size]).u32();
        e.set(this, NiFile::m_pBuffer, buffer);
        return this;
    }
    e.set(this, NiFile::m_pBuffer, 0);
    this
}

// Translated from 00aa15f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiFile` file size (`thiscall`, no stack
/// words). Answers 0 when `ftell` fails (negative); otherwise seeks to the
/// end (`fseek(file, 0, 2)`), takes `ftell` there, seeks back to the old
/// position (`fseek(file, old, 0)`) and answers the end position, or 0 when
/// it is negative.
pub fn fn_00aa15f0(e: &mut Engine, this: Ptr<NiFile>) -> u32 {
    let file = e.get(this, NiFile::m_pFile);
    let position = e.call(FTELL, &args![file]).i32();
    if position < 0 {
        return 0;
    }
    let file = e.get(this, NiFile::m_pFile);
    e.call(FSEEK, &args![file, 0u32, 2u32]);
    let file = e.get(this, NiFile::m_pFile);
    let end = e.call(FTELL, &args![file]).i32();
    let file = e.get(this, NiFile::m_pFile);
    e.call(FSEEK, &args![file, position, 0u32]);
    if end < 0 {
        0
    } else {
        end as u32
    }
}

// Translated from 00aa1650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the path): `CreateDirectoryA(path,
/// null)`, answering whether it succeeded.
pub fn fn_00aa1650(e: &mut Engine, path: Ptr) -> bool {
    e.call(API_CREATE_DIRECTORY, &args![path, 0u32]).u32() != 0
}

// Translated from 00aa1670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the path): whether the path is a
/// directory, by `GetFileAttributesA` (bit 4, `FILE_ATTRIBUTE_DIRECTORY`);
/// a failed call (-1) answers false. On failure the code clears only `AL`
/// (the rest of `EAX` keeps the -1's bytes); callers read `AL`.
pub fn fn_00aa1670(e: &mut Engine, path: Ptr) -> bool {
    let attributes = e.call(API_GET_FILE_ATTRIBUTES, &args![path]).u32();
    if attributes == 0xffff_ffff {
        return false;
    }
    (attributes >> 4) & 1 != 0
}

// Translated from 00aa18c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiFile` scalar deleting destructor
/// (`thiscall`, `RET 4`: the flags). The destructor body of `00aa1690`
/// inlined (vtable, flush and `fclose` of a good open file, the buffer freed
/// through `00aa10f0`, the base destructor `00aa1d30`), then, with bit 0 of
/// the flags, `this` given back to the memory manager (`00aa4060`). Answers
/// `this`.
pub fn fn_00aa18c0(e: &mut Engine, this: Ptr<NiFile>, flags: u32) -> Ptr<NiFile> {
    e.mem.set_u32(this.addr(), NI_FILE_VTABLE);
    if e.get(this, NiFile::m_bGood) != 0 && e.get(this, NiFile::m_pFile) != 0 {
        fn_00aa15a0(e, this);
        let file = e.get(this, NiFile::m_pFile);
        e.call(FCLOSE, &args![file]);
    }
    let buffer = e.get(this, NiFile::m_pBuffer);
    e.call(FREE_BUFFER, &args![buffer]);
    e.call(BINARY_STREAM_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(
            MEMORY_MANAGER_DEALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, this],
        );
    }
    this
}

// Translated from 00aa19a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra made no function here; read from its
/// bytes): the `NiFile` stream read function `00aa1b30` installs (`cdecl`:
/// the file, the buffer, the count, and the component sizes and their count
/// that the swapping variant `00aa19e0` takes, not read here). Answers
/// `00aa1750`'s result.
pub fn fn_00aa19a0(
    e: &mut Engine,
    file: Ptr<NiFile>,
    buffer: Ptr,
    count: u32,
    _sizes: Ptr,
    _size_count: u32,
) -> u32 {
    fn_00aa1750(e, file, buffer, count)
}

// Translated from 00aa19c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra made no function here; read from its
/// bytes): the `NiFile` stream write function `00aa1b30` installs (`cdecl`:
/// the file, the buffer, the count, and two words not read here, as for
/// `00aa19a0`). Answers `00aa1810`'s result.
pub fn fn_00aa19c0(
    e: &mut Engine,
    file: Ptr<NiFile>,
    buffer: Ptr,
    count: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    fn_00aa1810(e, file, buffer, count)
}

// Translated from 00aa19e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the file, the buffer, the byte count,
/// the component sizes and their count): a read that swaps byte order. A
/// count of 0 answers 0 and reads nothing; otherwise reads through
/// `00aa1750`, swaps the components of the `count` bytes asked for
/// (`00aa1de0(buffer, count, sizes, size count)`) and answers what the read
/// gave.
pub fn fn_00aa19e0(
    e: &mut Engine,
    file: Ptr<NiFile>,
    buffer: Ptr,
    count: u32,
    sizes: Ptr,
    size_count: u32,
) -> u32 {
    if count == 0 {
        return 0;
    }
    let read = fn_00aa1750(e, file, buffer, count);
    e.call(SWAP_COMPONENTS, &args![buffer, count, sizes, size_count]);
    read
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa_14d0,
            fn_00aa14d0(Ptr<NiFile>, Ptr, u32, u32) -> Ptr<NiFile>
        ),
        entry!(0x00aa_15f0, fn_00aa15f0(Ptr<NiFile>) -> u32),
        entry!(0x00aa_1650, fn_00aa1650(Ptr) -> bool),
        entry!(0x00aa_1670, fn_00aa1670(Ptr) -> bool),
        entry!(0x00aa_18c0, fn_00aa18c0(Ptr<NiFile>, u32) -> Ptr<NiFile>),
        entry!(
            0x00aa_19a0,
            fn_00aa19a0(Ptr<NiFile>, Ptr, u32, Ptr, u32) -> u32
        ),
        entry!(
            0x00aa_19c0,
            fn_00aa19c0(Ptr<NiFile>, Ptr, u32, u32, u32) -> u32
        ),
        entry!(
            0x00aa_19e0,
            fn_00aa19e0(Ptr<NiFile>, Ptr, u32, Ptr, u32) -> u32
        ),
        entry!(0x00aa13e0, fn_00aa13e0(u32) -> Ptr),
        entry!(0x00aa_1920, fn_00aa1920()),
        entry!(0x00aa_1380, fn_00aa1380(Ptr)),
        entry!(0x00aa_1420, fn_00aa1420(u32) -> Ptr),
        entry!(0x00aa_1460, fn_00aa1460(Ptr, u32)),
        entry!(0x00aa_1490, fn_00aa1490(Ptr)),
        entry!(0x00aa_14b0, fn_00aa14b0(Ptr<NiFile>) -> Ptr<NiFile>),
        entry!(0x00aa_1570, fn_00aa1570(Ptr<NiFile>, Ptr, u32) -> u32),
        entry!(0x00aa_15a0, fn_00aa15a0(Ptr<NiFile>) -> bool),
        entry!(0x00aa_1690, fn_00aa1690(Ptr<NiFile>)),
        entry!(0x00aa_16d0, fn_00aa16d0(Ptr<NiFile>, i32, i32)),
        entry!(0x00aa_1750, fn_00aa1750(Ptr<NiFile>, Ptr, u32) -> u32),
        entry!(0x00aa_1810, fn_00aa1810(Ptr<NiFile>, Ptr, u32) -> u32),
        entry!(0x00aa_1980, fn_00aa1980()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOCATE: u32 = 0x0070_0000;
    const VTABLE: u32 = 0x0300_0000;

    /// An allocator whose allocation method records the values behind the two
    /// address arguments and the remaining arguments, and returns 0x4242.
    fn engine_with_allocator() -> (Engine, std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>) {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        let allocator = e.mem.alloc(8);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, allocator);
        e.mem.set_u32(ALLOCATOR_REFERENCE, holder);
        e.put_vtable(VTABLE, &[0, ALLOCATE]);
        e.mem.set_u32(allocator, VTABLE);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(ALLOCATE, move |e, a| {
            let mut words = vec![a[0], e.mem.u32(a[1]), e.mem.u32(a[2])];
            words.extend_from_slice(&a[3..]);
            log.borrow_mut().push(words);
            Ret {
                eax: 0x4242,
                ..Ret::default()
            }
        });
        (e, seen)
    }

    #[test]
    fn allocates_through_the_virtual_method() {
        let (mut e, seen) = engine_with_allocator();
        let allocator = e.mem.u32(e.mem.u32(ALLOCATOR_REFERENCE));
        let result = e.call(0x00aa_13e0, &args![0x280u32]).u32();
        assert_eq!(result, 0x4242);
        assert_eq!(
            seen.borrow()[0],
            vec![allocator, 0x280, 4, 1, 1, 0, 0xffff_ffff, 0]
        );
    }

    #[test]
    fn a_zero_size_is_requested_as_one() {
        let (mut e, seen) = engine_with_allocator();
        e.call(0x00aa_13e0, &args![0u32]);
        assert_eq!(seen.borrow()[0][1], 1);
    }
}

#[cfg(test)]
mod system_facts_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        e.register(API_GET_SYSTEM_INFO, |e, a| {
            e.mem.set_u32(a[0] + 0x14, 6);
            1u32.into_ret()
        });
        e.register(API_QUERY_PERFORMANCE_FREQUENCY, |e, a| {
            // 0x1_0000_0001 is not exact as a float: rounds to 4294967296.0.
            e.mem.set_u32(a[0], 1);
            e.mem.set_u32(a[0] + 4, 1);
            1u32.into_ret()
        });
        e
    }

    #[test]
    fn stores_the_processor_count_and_the_frequency_as_a_float() {
        let mut e = engine();
        let block = e.mem.alloc(8);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 8]);
            block.into_ret()
        });
        e.call(0x00aa_1920, &args![]);
        assert_eq!(e.mem.u32(SYSTEM_FACTS_SLOT), block);
        assert_eq!(e.mem.u32(block), 6);
        assert_eq!(f32::from_bits(e.mem.u32(block + 4)), 4294967296.0);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let mut e = engine();
        e.mem.set_u32(SYSTEM_FACTS_SLOT, 0x1234);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00aa_1920, &args![]);
        assert_eq!(e.mem.u32(SYSTEM_FACTS_SLOT), 0);
    }
}

#[cfg(test)]
mod b006_tests {
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

    fn normalised(text: &[u8]) -> Vec<u8> {
        let mut e = Engine::new();
        let path = e.mem.alloc(0x40);
        e.mem.set_cstr(path, text);
        e.call(0x00aa_1380, &args![path]);
        e.mem.cstr(path)
    }

    #[test]
    fn path_slashes_become_single_backslashes() {
        assert_eq!(normalised(b"a//b\\\\c/d"), b"a\\b\\c\\d");
        // A doubled separator at the start is kept (index 1 is not dropped).
        assert_eq!(normalised(b"\\\\server\\x"), b"\\\\server\\x");
        assert_eq!(normalised(b"//\\x"), b"\\\\x");
        assert_eq!(normalised(b""), b"");
        let mut e = Engine::new();
        e.call(0x00aa_1380, &args![0u32]);
    }

    const ALLOCATOR_METHOD: u32 = 0x0070_0000;
    const DEALLOCATOR_METHOD: u32 = 0x0070_0010;

    fn engine_with_allocator() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        let allocator = e.mem.alloc(8);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, allocator);
        e.mem.set_u32(ALLOCATOR_REFERENCE, holder);
        e.put_vtable(0x0300_0000, &[0, ALLOCATOR_METHOD, DEALLOCATOR_METHOD]);
        e.mem.set_u32(allocator, 0x0300_0000);
        e.register(DEALLOCATOR_METHOD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, allocator)
    }

    #[test]
    fn allocate_passes_two_zero_zero() {
        let (mut e, allocator) = engine_with_allocator();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(ALLOCATOR_METHOD, move |e, a| {
            log.borrow_mut()
                .push((e.mem.u32(a[1]), e.mem.u32(a[2]), a[3..].to_vec()));
            0x4242u32.into_ret()
        });
        assert_eq!(e.call(0x00aa_1420, &args![0x30u32]).u32(), 0x4242);
        e.call(0x00aa_1420, &args![0u32]);
        let seen = seen.borrow();
        assert_eq!(seen[0], (0x30, 4, vec![2, 0, 0, 0xffff_ffff, 0]));
        assert_eq!(seen[1].0, 1);
        assert_eq!(calls_to(&e, ALLOCATOR_METHOD)[0][0], allocator);
    }

    #[test]
    fn deallocators_skip_null_and_pass_their_kinds() {
        let (mut e, allocator) = engine_with_allocator();
        e.call(0x00aa_1460, &args![0u32, 9u32]);
        e.call(0x00aa_1490, &args![0u32]);
        assert!(calls_to(&e, DEALLOCATOR_METHOD).is_empty());
        e.call(0x00aa_1460, &args![0x5000u32, 9u32]);
        e.call(0x00aa_1490, &args![0x6000u32]);
        assert_eq!(
            calls_to(&e, DEALLOCATOR_METHOD),
            vec![
                vec![allocator, 0x5000, 3, 9],
                vec![allocator, 0x6000, 4, 0xffff_ffff]
            ]
        );
    }

    /// A good `NiFile` in `mode` with a 0x10-byte buffer and doubles for the
    /// file callees: reads answer `read_answer`, writes `write_answer`.
    fn file(mode: u32, read_answer: u32, write_answer: u32) -> (Engine, Ptr<NiFile>, u32) {
        let mut e = Engine::new();
        let this: Ptr<NiFile> = e.new_object();
        let buffer = e.mem.alloc(0x10);
        e.set(this, NiFile::m_pBuffer, buffer);
        e.set(this, NiFile::m_pFile, 0x7700);
        e.set(this, NiFile::m_uiBufferAllocSize, 0x10);
        e.set(this, NiFile::m_eMode, mode);
        e.set(this, NiFile::m_bGood, 1);
        e.register_double(FILE_READ, move |_, _| read_answer.into_ret());
        e.register_double(FILE_WRITE, move |_, _| write_answer.into_ret());
        e.register(FILE_SEEK, |_, _| 0u32.into_ret());
        e.register(FTELL, |_, _| 0x123u32.into_ret());
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(FCLOSE, |_, _| Ret::default());
        e.register(FREE_BUFFER, |_, _| Ret::default());
        e.register(BINARY_STREAM_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BINARY_STREAM_DESTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, this, buffer)
    }

    #[test]
    fn constructor_clears_the_file_fields() {
        let (mut e, this, _) = file(0, 0, 0);
        e.set(this, NiFile::m_uiCurrentFilePos, 9);
        assert_eq!(e.call(0x00aa_14b0, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), NI_FILE_VTABLE);
        assert_eq!(e.get(this, NiFile::m_pBuffer), 0);
        assert_eq!(e.get(this, NiFile::m_pFile), 0);
        assert_eq!(e.get(this, NiFile::m_uiCurrentFilePos), 0);
        assert_eq!(
            calls_to(&e, BINARY_STREAM_CONSTRUCTOR),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn direct_read_counts_the_file_position() {
        let (mut e, this, _) = file(0, 6, 0);
        e.set(this, NiFile::m_uiCurrentFilePos, 4);
        assert_eq!(e.call(0x00aa_1570, &args![this, 0x900u32, 8u32]).u32(), 6);
        assert_eq!(e.get(this, NiFile::m_uiCurrentFilePos), 10);
        assert_eq!(calls_to(&e, FILE_READ), vec![vec![0x900, 1, 8, 0x7700]]);
    }

    #[test]
    fn flush_in_read_mode_drops_the_buffer() {
        let (mut e, this, _) = file(0, 0, 0);
        e.set(this, NiFile::m_uiBufferReadSize, 7);
        e.set(this, NiFile::m_uiPos, 3);
        assert!(e.call(0x00aa_15a0, &args![this]).bool());
        assert_eq!(e.get(this, NiFile::m_uiBufferReadSize), 0);
        assert_eq!(e.get(this, NiFile::m_uiPos), 0);
        assert!(calls_to(&e, FILE_WRITE).is_empty());
    }

    #[test]
    fn flush_in_write_mode_writes_the_pending_bytes() {
        let (mut e, this, buffer) = file(1, 0, 5);
        assert!(e.call(0x00aa_15a0, &args![this]).bool());
        assert!(calls_to(&e, FILE_WRITE).is_empty());
        e.set(this, NiFile::m_uiPos, 5);
        assert!(e.call(0x00aa_15a0, &args![this]).bool());
        assert_eq!(calls_to(&e, FILE_WRITE), vec![vec![buffer, 1, 5, 0x7700]]);
        assert_eq!(e.get(this, NiFile::m_uiPos), 0);
        // A short write makes the file bad and keeps the position.
        let (mut e, this, _) = file(2, 0, 4);
        e.set(this, NiFile::m_uiPos, 5);
        assert!(!e.call(0x00aa_15a0, &args![this]).bool());
        assert_eq!(e.get(this, NiFile::m_bGood), 0);
        assert_eq!(e.get(this, NiFile::m_uiPos), 5);
    }

    #[test]
    fn destructor_closes_a_good_open_file() {
        let (mut e, this, buffer) = file(0, 0, 0);
        e.call(0x00aa_1690, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_FILE_VTABLE);
        assert_eq!(calls_to(&e, FCLOSE), vec![vec![0x7700]]);
        assert_eq!(calls_to(&e, FREE_BUFFER), vec![vec![buffer]]);
        assert_eq!(
            calls_to(&e, BINARY_STREAM_DESTRUCTOR),
            vec![vec![this.addr()]]
        );
        let (mut e, this, _) = file(0, 0, 0);
        e.set(this, NiFile::m_bGood, 0);
        e.call(0x00aa_1690, &args![this]);
        assert!(calls_to(&e, FCLOSE).is_empty());
        let (mut e, this, _) = file(0, 0, 0);
        e.set(this, NiFile::m_pFile, 0);
        e.call(0x00aa_1690, &args![this]);
        assert!(calls_to(&e, FCLOSE).is_empty());
        assert_eq!(calls_to(&e, FREE_BUFFER).len(), 1);
    }

    #[test]
    fn seek_inside_the_buffer_only_moves_the_positions() {
        let (mut e, this, _) = file(0, 0, 0);
        e.set(this, NiFile::m_uiBufferReadSize, 10);
        e.set(this, NiFile::m_uiPos, 4);
        e.set(this, NiFile::m_uiAbsoluteCurrentPos, 100);
        e.call(0x00aa_16d0, &args![this, -2i32, 1i32]);
        assert_eq!(e.get(this, NiFile::m_uiPos), 2);
        assert_eq!(e.get(this, NiFile::m_uiAbsoluteCurrentPos), 98);
        assert!(calls_to(&e, FILE_SEEK).is_empty());
    }

    #[test]
    fn seek_outside_the_buffer_seeks_the_file() {
        let (mut e, this, _) = file(0, 0, 0);
        e.set(this, NiFile::m_uiBufferReadSize, 10);
        e.set(this, NiFile::m_uiPos, 4);
        e.call(0x00aa_16d0, &args![this, 6i32, 1i32]);
        // Read mode: the offset is made relative to the file: 6 + 4 - 10.
        assert_eq!(calls_to(&e, FILE_SEEK), vec![vec![0x7700, 0, 1]]);
        assert_eq!(e.get(this, NiFile::m_bGood), 1);
        assert_eq!(e.get(this, NiFile::m_uiCurrentFilePos), 0x123);
        assert_eq!(e.get(this, NiFile::m_uiAbsoluteCurrentPos), 0x123);
        assert_eq!(e.get(this, NiFile::m_uiBufferReadSize), 0);
        // Write mode keeps the offset; an absolute seek too.
        let (mut e, this, _) = file(1, 0, 0);
        e.call(0x00aa_16d0, &args![this, -1i32, 1i32]);
        e.call(0x00aa_16d0, &args![this, 40i32, 0i32]);
        assert_eq!(
            calls_to(&e, FILE_SEEK),
            vec![vec![0x7700, 0xffff_ffff, 1], vec![0x7700, 40, 0]]
        );
    }

    #[test]
    fn a_failed_seek_makes_the_file_bad_and_a_bad_file_ignores_seeks() {
        let (mut e, this, _) = file(0, 0, 0);
        e.register(FILE_SEEK, |_, _| 1u32.into_ret());
        e.set(this, NiFile::m_uiCurrentFilePos, 5);
        e.call(0x00aa_16d0, &args![this, 0i32, 2i32]);
        assert_eq!(e.get(this, NiFile::m_bGood), 0);
        assert_eq!(e.get(this, NiFile::m_uiCurrentFilePos), 5);
        assert!(calls_to(&e, FTELL).is_empty());
        e.call(0x00aa_16d0, &args![this, 0i32, 2i32]);
        assert_eq!(calls_to(&e, FILE_SEEK).len(), 1);
    }

    #[test]
    fn read_from_the_buffer_alone() {
        let (mut e, this, buffer) = file(0, 0, 0);
        e.set(this, NiFile::m_uiBufferReadSize, 10);
        e.set(this, NiFile::m_uiPos, 2);
        assert_eq!(e.call(0x00aa_1750, &args![this, 0x900u32, 5u32]).u32(), 5);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x900, buffer + 2, 5]]);
        assert_eq!(e.get(this, NiFile::m_uiPos), 7);
        e.set(this, NiFile::m_bGood, 0);
        assert_eq!(e.call(0x00aa_1750, &args![this, 0x900u32, 5u32]).u32(), 0);
    }

    #[test]
    fn read_refills_the_buffer_and_stops_at_what_it_got() {
        let (mut e, this, buffer) = file(0, 4, 0);
        e.set(this, NiFile::m_uiBufferReadSize, 10);
        e.set(this, NiFile::m_uiPos, 7);
        // 3 come from the buffer, then the refill gets 4 of the 0x10 asked.
        assert_eq!(e.call(0x00aa_1750, &args![this, 0x900u32, 9u32]).u32(), 7);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![0x900, buffer + 7, 3], vec![0x903, buffer, 4]]
        );
        assert_eq!(calls_to(&e, FILE_READ), vec![vec![buffer, 1, 0x10, 0x7700]]);
        assert_eq!(e.get(this, NiFile::m_uiBufferReadSize), 4);
        assert_eq!(e.get(this, NiFile::m_uiCurrentFilePos), 4);
        assert_eq!(e.get(this, NiFile::m_uiPos), 4);
    }

    #[test]
    fn a_large_read_goes_straight_to_the_caller() {
        let (mut e, this, _) = file(0, 0x20, 0);
        assert_eq!(
            e.call(0x00aa_1750, &args![this, 0x900u32, 0x20u32]).u32(),
            0x20
        );
        assert!(calls_to(&e, MEMCPY).is_empty());
        assert_eq!(calls_to(&e, FILE_READ), vec![vec![0x900, 1, 0x20, 0x7700]]);
        assert_eq!(e.get(this, NiFile::m_uiCurrentFilePos), 0x20);
    }

    #[test]
    fn write_into_the_buffer_alone() {
        let (mut e, this, buffer) = file(1, 0, 0);
        e.set(this, NiFile::m_uiPos, 3);
        assert_eq!(e.call(0x00aa_1810, &args![this, 0x900u32, 4u32]).u32(), 4);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![buffer + 3, 0x900, 4]]);
        assert_eq!(e.get(this, NiFile::m_uiPos), 7);
        e.set(this, NiFile::m_bGood, 0);
        assert_eq!(e.call(0x00aa_1810, &args![this, 0x900u32, 4u32]).u32(), 0);
    }

    #[test]
    fn write_fills_flushes_and_buffers_the_rest() {
        let (mut e, this, buffer) = file(1, 0, 0x10);
        e.set(this, NiFile::m_uiPos, 0xc);
        assert_eq!(e.call(0x00aa_1810, &args![this, 0x900u32, 6u32]).u32(), 6);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![buffer + 0xc, 0x900, 4], vec![buffer, 0x904, 2]]
        );
        assert_eq!(
            calls_to(&e, FILE_WRITE),
            vec![vec![buffer, 1, 0x10, 0x7700]]
        );
        assert_eq!(e.get(this, NiFile::m_uiPos), 2);
    }

    #[test]
    fn a_large_write_goes_straight_to_the_file_and_a_failed_flush_stops() {
        let (mut e, this, buffer) = file(1, 0, 0);
        e.register(FILE_WRITE, |_, a| a[2].into_ret());
        e.set(this, NiFile::m_uiPos, 0x10);
        assert_eq!(
            e.call(0x00aa_1810, &args![this, 0x900u32, 0x30u32]).u32(),
            0x30
        );
        assert_eq!(
            calls_to(&e, FILE_WRITE),
            vec![vec![buffer, 1, 0x10, 0x7700], vec![0x900, 1, 0x30, 0x7700]]
        );
        assert!(calls_to(&e, MEMCPY).is_empty());
        assert_eq!(e.get(this, NiFile::m_uiCurrentFilePos), 0);
        let (mut e, this, _) = file(1, 0, 0);
        e.set(this, NiFile::m_uiPos, 0x10);
        assert_eq!(e.call(0x00aa_1810, &args![this, 0x900u32, 1u32]).u32(), 0);
        assert_eq!(e.get(this, NiFile::m_bGood), 0);
    }

    #[test]
    fn frees_the_system_facts_block() {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        e.mem.set_u32(SYSTEM_FACTS_SLOT, 0x5550);
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00aa_1980, &args![]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER_OBJECT, 0x5550]]
        );
    }
}

#[cfg(test)]
mod tests_n084 {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &f in callees {
            e.register(f, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_00aa14d0_opens_in_each_mode_and_takes_a_buffer() {
        for (mode, text) in [
            (0u32, MODE_READ_TEXT),
            (1, MODE_WRITE_TEXT),
            (2, MODE_APPEND_TEXT),
        ] {
            let mut e = Engine::new();
            quiet(
                &mut e,
                &[BINARY_STREAM_CONSTRUCTOR, BINARY_STREAM_SET_FUNCTIONS],
            );
            e.register(FOPEN_S, |e, a| {
                e.mem.set_u32(a[0], 0x777);
                Ret::default()
            });
            e.register(ALLOCATE_BUFFER, |_, _| 0x4400u32.into_ret());
            let this = e.mem.alloc(0x30);
            e.mem.set_u32(this + 0x18, 9);
            e.call_log = Some(vec![]);
            let r = e
                .call(0x00aa_14d0, &args![this, 0x9000u32, mode, 0x100u32])
                .u32();
            assert_eq!(r, this);
            assert_eq!(e.mem.u32(this), NI_FILE_VTABLE);
            assert_eq!(calls(&e, BINARY_STREAM_SET_FUNCTIONS), vec![vec![this, 0]]);
            assert_eq!(calls(&e, FOPEN_S), vec![vec![this + 0x24, 0x9000, text]]);
            assert_eq!(e.mem.u32(this + 0x28), mode);
            assert_eq!(e.mem.u8(this + 0x2c), 1);
            assert_eq!(e.mem.u32(this + 0x10), 0x100);
            assert_eq!(e.mem.u32(this + 0x18), 0);
            assert_eq!(e.mem.u32(this + 0x20), 0x4400);
        }
    }

    #[test]
    fn fn_00aa14d0_without_a_file_has_no_buffer() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[BINARY_STREAM_CONSTRUCTOR, BINARY_STREAM_SET_FUNCTIONS],
        );
        e.register(FOPEN_S, |_, _| 2u32.into_ret());
        e.register(ALLOCATE_BUFFER, |_, _| 0x4400u32.into_ret());
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x20, 5);
        e.call_log = Some(vec![]);
        e.call(0x00aa_14d0, &args![this, 0x9000u32, 0u32, 0x100u32]);
        assert_eq!(e.mem.u8(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert!(calls(&e, ALLOCATE_BUFFER).is_empty());
    }

    #[test]
    fn fn_00aa15f0_measures_and_restores_the_position() {
        let mut e = Engine::new();
        let mut answers = vec![-1i32, 0x30, 0x10].into_iter();
        e.register_double(FTELL, move |_, _| answers.next().unwrap().into_ret());
        e.register(FSEEK, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x24, 0x777);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_15f0, &args![this]).u32(), 0);
        assert!(calls(&e, FSEEK).is_empty());
        assert_eq!(e.call(0x00aa_15f0, &args![this]).u32(), 0x10);
        assert_eq!(
            calls(&e, FSEEK),
            vec![vec![0x777, 0, 2], vec![0x777, 0x30, 0]]
        );
        let mut e2 = Engine::new();
        let mut answers = vec![4i32, -5].into_iter();
        e2.register_double(FTELL, move |_, _| answers.next().unwrap().into_ret());
        e2.register(FSEEK, |_, _| Ret::default());
        let this = e2.mem.alloc(0x30);
        assert_eq!(e2.call(0x00aa_15f0, &args![this]).u32(), 0);
    }

    #[test]
    fn fn_00aa1650_and_fn_00aa1670_ask_windows() {
        let mut e = Engine::new();
        e.register(API_CREATE_DIRECTORY, |_, a| (a[0] & 1).into_ret());
        assert!(e.call(0x00aa_1650, &args![0x9001u32]).bool());
        assert!(!e.call(0x00aa_1650, &args![0x9000u32]).bool());
        e.register(API_GET_FILE_ATTRIBUTES, |_, a| a[0].into_ret());
        assert!(!e.call(0x00aa_1670, &args![0xffff_ffffu32]).bool());
        assert!(e.call(0x00aa_1670, &args![0x10u32]).bool());
        assert!(!e.call(0x00aa_1670, &args![0x20u32]).bool());
    }

    #[test]
    fn fn_00aa18c0_closes_frees_and_deletes_with_bit_0() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                FCLOSE,
                FREE_BUFFER,
                BINARY_STREAM_DESTRUCTOR,
                MEMORY_MANAGER_DEALLOCATE,
            ],
        );
        let this = e.mem.alloc(0x30);
        e.mem.set_u8(this + 0x2c, 1);
        e.mem.set_u32(this + 0x24, 0x777);
        e.mem.set_u32(this + 0x20, 0x4400);
        e.mem.set_u32(this + 0x14, 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_18c0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_FILE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(calls(&e, FCLOSE), vec![vec![0x777]]);
        assert_eq!(calls(&e, FREE_BUFFER), vec![vec![0x4400]]);
        assert_eq!(calls(&e, BINARY_STREAM_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER_OBJECT, this]]
        );
        e.mem.set_u8(this + 0x2c, 0);
        e.call(0x00aa_18c0, &args![this, 0u32]);
        assert_eq!(calls(&e, FCLOSE).len(), 1);
        assert_eq!(calls(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
    }

    /// A good file in read mode with no buffer: reads go straight to the
    /// file (`00aa8610`), writes straight to it (`00aa8660`).
    fn direct_file(e: &mut Engine, mode: u32) -> u32 {
        e.register(FILE_READ, |_, a| a[2].into_ret());
        e.register(FILE_WRITE, |_, a| a[2].into_ret());
        let this = e.mem.alloc(0x30);
        e.mem.set_u8(this + 0x2c, 1);
        e.mem.set_u32(this + 0x24, 0x777);
        e.mem.set_u32(this + 0x28, mode);
        this
    }

    #[test]
    fn fn_00aa19a0_and_fn_00aa19c0_pass_on_the_answer() {
        let mut e = Engine::new();
        let file = direct_file(&mut e, 0);
        e.call_log = Some(vec![]);
        let r = e
            .call(0x00aa_19a0, &args![file, 0x9000u32, 8u32, 0u32, 0u32])
            .u32();
        assert_eq!(r, 8);
        assert_eq!(calls(&e, FILE_READ), vec![vec![0x9000, 1, 8, 0x777]]);
        let file = direct_file(&mut e, 1);
        let r = e
            .call(0x00aa_19c0, &args![file, 0x9000u32, 6u32, 0u32, 0u32])
            .u32();
        assert_eq!(r, 6);
        assert_eq!(calls(&e, FILE_WRITE), vec![vec![0x9000, 1, 6, 0x777]]);
    }

    #[test]
    fn fn_00aa19e0_reads_then_swaps() {
        let mut e = Engine::new();
        e.register(SWAP_COMPONENTS, |_, _| Ret::default());
        let file = direct_file(&mut e, 0);
        e.call_log = Some(vec![]);
        let r = e
            .call(0x00aa_19e0, &args![file, 0x9000u32, 0u32, 0x8000u32, 2u32])
            .u32();
        assert_eq!(r, 0);
        assert!(calls(&e, SWAP_COMPONENTS).is_empty());
        let r = e
            .call(0x00aa_19e0, &args![file, 0x9000u32, 8u32, 0x8000u32, 2u32])
            .u32();
        assert_eq!(r, 8);
        assert_eq!(calls(&e, SWAP_COMPONENTS), vec![vec![0x9000, 8, 0x8000, 2]]);
    }
}

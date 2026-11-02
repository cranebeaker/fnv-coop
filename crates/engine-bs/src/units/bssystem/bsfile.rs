//! `BSSystem/bsfile.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PC `BSFile` vtable (`010a4764`, RTTI `.?AVBSFile@@`) has the Xbox
//! PDB's slots in the PDB's order without `GetDevice`: +0x10
//! `SetEndianSwap` (`00aff990`), +0x1c `GetFileSize` (`00aff610`), +0x20
//! `Open` (`00aff300`), +0x28 `GetSize`, +0x34 `GetLine` (`00affc90`), +0x40
//! `Exist` (`00affc60`), +0x44 `ReadF`, +0x48 `WriteF` (`00aff950`). Slot
//! names other than the engine map's are read from that order and are not
//! otherwise confirmed.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSStringT;
use crate::units::nisystem::run_00aa1380::NiFile;

layout! {
    /// `BSFile` (Xbox PDB), 0x158 bytes on PC: an `NiFile` (0x30 bytes on
    /// PC, its fields through [`NiFile`]) and its own fields, 0x44 lower
    /// than the PDB's from `pFileName` on.
    pub struct BSFile: 0x158 {
        /// `pFileName` (Xbox PDB, +0x88 there): `char[260]`.
        0x044 pFileName: u8,
        /// `iTrueFilePos` (Xbox PDB, +0x194 there).
        0x150 iTrueFilePos: u32,
        /// `iFileSize` (Xbox PDB, +0x198 there).
        0x154 iFileSize: u32,
    }
}

/// The `BSFile` vtable (RTTI `.?AVBSFile@@`).
const BSFILE_VTABLE: u32 = 0x010a_4764;
/// `BSFile` vtable slots (byte offsets, see the module comment).
const GET_FILE_SIZE_SLOT: u32 = 0x1c;
const GET_SIZE_SLOT: u32 = 0x28;
const READ_F_SLOT: u32 = 0x44;
const WRITE_F_SLOT: u32 = 0x48;
/// The read and write functions `00aff990` installs in `m_pfnRead` (+8) and
/// `m_pfnWrite` (+0xc): the plain pair, and the byte-swapping pair
/// (`00affa10`, `00affa60`, not translated here).
const PLAIN_READ: u32 = 0x00af_f9d0;
const PLAIN_WRITE: u32 = 0x00af_f9f0;
const SWAPPING_READ: u32 = 0x00af_fa10;
const SWAPPING_WRITE: u32 = 0x00af_fa60;
/// `_access(path, mode)` (`00ecbf05`, cdecl), -1 on failure.
const ACCESS: u32 = 0x00ec_bf05;
/// `fopen_s(&file, name, mode)` (`00ec9a5e`, cdecl).
const FOPEN_S: u32 = 0x00ec_9a5e;
/// `fclose` (`00ec9907`, cdecl).
const FCLOSE: u32 = 0x00ec_9907;
/// The `NiFile` functions `BSFile` calls (`NiSystem/run_00aa1380`): the
/// buffer flush (thiscall), the direct read (thiscall: buffer, count), the
/// buffered write (thiscall: buffer, count) and the destructor body.
const NI_FILE_FLUSH: u32 = 0x00aa_15a0;
const NI_FILE_READ_DIRECT: u32 = 0x00aa_1570;
const NI_FILE_WRITE: u32 = 0x00aa_1810;
const NI_FILE_DESTRUCTOR: u32 = 0x00aa_1690;
/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, thiscall:
/// size) and deallocate (`00aa4060`, thiscall: block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// The thread block's word at +0x2b4 (the allocation context the memory
/// manager reads); `CheckIsGood` sets it to 0x17 around its allocation.
const ALLOCATION_CONTEXT: u32 = 0x2b4;
const FILE_BUFFER_CONTEXT: u32 = 0x17;
/// `fopen_s` mode strings in `.rdata`.
const MODE_RT: u32 = 0x010a_46cc;
const MODE_RB: u32 = 0x010a_46d0;
const MODE_R_PLUS_T: u32 = 0x010a_46d4;
const MODE_R_PLUS_B: u32 = 0x010a_46d8;
const MODE_A_PLUS_T: u32 = 0x010a_46dc;
const MODE_A_PLUS_B: u32 = 0x010a_46e0;
const MODE_WT: u32 = 0x010a_46e4;
const MODE_WB: u32 = 0x010a_46e8;

fn ni_file(this: Ptr<BSFile>) -> Ptr<NiFile> {
    this.cast()
}

// Translated from 00aff990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0x10 (`SetEndianSwap` by the
/// PDB's slot order): with `swap` set installs `00affa10` / `00affa60` as
/// `m_pfnRead` / `m_pfnWrite`, otherwise `00aff9d0` / `00aff9f0`.
pub fn fn_00aff990(e: &mut Engine, this: Ptr<BSFile>, swap: bool) {
    let (read, write) = if swap {
        (SWAPPING_READ, SWAPPING_WRITE)
    } else {
        (PLAIN_READ, PLAIN_WRITE)
    };
    e.mem.set_u32(this.addr() + 8, read);
    e.mem.set_u32(this.addr() + 0xc, write);
}

// Translated from 00aff9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): the plain `m_pfnRead`:
/// calls `stream`'s vtable +0x44 (`ReadF`) with `(buffer, size)` and passes
/// its answer on (`EAX` is left as the call returns it).
pub fn fn_00aff9d0(e: &mut Engine, stream: Ptr, buffer: Ptr, size: u32) -> u32 {
    e.vcall(stream.addr(), READ_F_SLOT, &args![buffer, size])
        .u32()
}

// Translated from 00aff9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): the plain `m_pfnWrite`:
/// calls `stream`'s vtable +0x48 (`WriteF`) with `(buffer, size)` and
/// passes its answer on.
pub fn fn_00aff9f0(e: &mut Engine, stream: Ptr, buffer: Ptr, size: u32) -> u32 {
    e.vcall(stream.addr(), WRITE_F_SLOT, &args![buffer, size])
        .u32()
}

// Translated from 00aff610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0x1c (`GetFileSize` by the
/// PDB's slot order): when `iFileSize` is 0 calls vtable +0x28 (which is
/// expected to fill it; its answer is not used), then answers `iFileSize`.
pub fn fn_00aff610(e: &mut Engine, this: Ptr<BSFile>) -> u32 {
    if e.get(this, BSFile::iFileSize) == 0 {
        e.vcall(this.addr(), GET_SIZE_SLOT, &args![]);
    }
    e.get(this, BSFile::iFileSize)
}

// Translated from 00aff950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0x48 (`WriteF` by the PDB's
/// slot order): the buffered `NiFile` write `00aa1810(buffer, count)`, whose
/// answer is added to `iTrueFilePos` and returned.
pub fn fn_00aff950(e: &mut Engine, this: Ptr<BSFile>, buffer: Ptr, count: u32) -> u32 {
    let written = e.call(NI_FILE_WRITE, &args![this, buffer, count]).u32();
    let position = e.get(this, BSFile::iTrueFilePos);
    e.set(this, BSFile::iTrueFilePos, position.wrapping_add(written));
    written
}

// Translated from 00affc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFile::Exist` (Xbox PDB): whether `_access(pFileName, 0)` succeeds.
pub fn bs_file_exist(e: &mut Engine, this: Ptr<BSFile>) -> bool {
    let name = this.addr() + BSFile::pFileName.off;
    e.call(ACCESS, &args![name, 0u32]).i32() != -1
}

// Translated from 00affc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0x34 (`GetLine` by the PDB's
/// slot order): reads one byte at a time through vtable +0x44 (`ReadF`)
/// into `buffer`, at most `size - 1` of them, stopping at a short read or
/// at the byte `terminator` (compared as a sign-extended byte against the
/// zero-extended 16-bit `terminator`; neither is stored). The buffer is
/// then terminated; the answer is the sum of the reads' answers.
pub fn fn_00affc90(
    e: &mut Engine,
    this: Ptr<BSFile>,
    buffer: Ptr,
    size: u32,
    terminator: u16,
) -> u32 {
    let mut total = 0u32;
    let mut index = 0u32;
    e.with_stack(4, |e, byte| {
        while index.wrapping_add(1) < size {
            let read = e.vcall(this.addr(), READ_F_SLOT, &args![byte, 1u32]).u32();
            total = total.wrapping_add(read);
            let value = e.mem.u8(byte.addr());
            if read != 1 || i32::from(value as i8) == i32::from(terminator) {
                break;
            }
            e.mem.set_u8(buffer.addr().wrapping_add(index), value);
            index += 1;
        }
    });
    e.mem.set_u8(buffer.addr().wrapping_add(index), 0);
    total
}

// Translated from 00affd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFile::Close` (Xbox PDB): when the file is good and open, flushes it
/// (`00aa15a0`) and closes it (`fclose`); then frees the buffer through the
/// memory manager and clears `m_pBuffer` and `m_pFile`.
pub fn bs_file_close(e: &mut Engine, this: Ptr<BSFile>) {
    let file = ni_file(this);
    if e.get(file, NiFile::m_bGood) != 0 && e.get(file, NiFile::m_pFile) != 0 {
        e.call(NI_FILE_FLUSH, &args![this]);
        let handle = e.get(file, NiFile::m_pFile);
        e.call(FCLOSE, &args![handle]);
    }
    let buffer = e.get(file, NiFile::m_pBuffer);
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
    e.set(file, NiFile::m_pBuffer, 0);
    e.set(file, NiFile::m_pFile, 0);
}

// Translated from 00aff240 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aff240 - the FS:[0] exception frame (the unwind state around Close) is not translated
/// `BSFile::~BSFile` (Xbox PDB): installs the `BSFile` vtable, closes the
/// file ([`bs_file_close`]) and runs the `NiFile` destructor body
/// (`00aa1690`).
pub fn bs_file_destructor(e: &mut Engine, this: Ptr<BSFile>) {
    e.mem.set_u32(this.addr(), BSFILE_VTABLE);
    bs_file_close(e, this);
    e.call(NI_FILE_DESTRUCTOR, &args![this]);
}

// Translated from 00afff20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0: the scalar deleting
/// destructor. Runs [`bs_file_destructor`], then with bit 0 of `flags`
/// frees `this` through the memory manager. Returns `this`.
pub fn fn_00afff20(e: &mut Engine, this: Ptr<BSFile>, flags: u32) -> Ptr<BSFile> {
    bs_file_destructor(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00aff490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aff490 - the FS:[0] exception frame (the unwind state around the allocation) is not translated
/// `BSFile::CheckIsGood` (Xbox PDB): `m_bGood` is whether a `FILE` is open.
/// An open file with a nonzero `m_uiBufferAllocSize` and no buffer gets
/// one, allocated with the thread's allocation context (TLS +0x2b4) set to
/// 0x17 and restored after. A buffer size of -1 means "the whole file":
/// the size is taken from vtable +0x1c (`GetFileSize`), the whole file is
/// read into the buffer (`00aa1570`, read size = size, position 0), and a
/// short read clears `m_bGood`.
pub fn bs_file_check_is_good(e: &mut Engine, this: Ptr<BSFile>) {
    let file = ni_file(this);
    if e.get(file, NiFile::m_pFile) == 0 {
        e.set(file, NiFile::m_bGood, 0);
        return;
    }
    e.set(file, NiFile::m_bGood, 1);
    if e.get(file, NiFile::m_uiBufferAllocSize) == 0 || e.get(file, NiFile::m_pBuffer) != 0 {
        return;
    }
    let context = e.tls() + ALLOCATION_CONTEXT;
    let saved = e.mem.u32(context);
    e.mem.set_u32(context, FILE_BUFFER_CONTEXT);
    let whole = e.get(file, NiFile::m_uiBufferAllocSize) == u32::MAX;
    if whole {
        let size = e.vcall(this.addr(), GET_FILE_SIZE_SLOT, &args![]).u32();
        e.set(file, NiFile::m_uiBufferAllocSize, size);
    }
    let size = e.get(file, NiFile::m_uiBufferAllocSize);
    let buffer = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    e.set(file, NiFile::m_pBuffer, buffer);
    if whole {
        let size = e.get(file, NiFile::m_uiBufferAllocSize);
        e.set(file, NiFile::m_uiBufferReadSize, size);
        e.set(file, NiFile::m_uiPos, 0);
        let buffer = e.get(file, NiFile::m_pBuffer);
        let size = e.get(file, NiFile::m_uiBufferAllocSize);
        let read = e
            .call(NI_FILE_READ_DIRECT, &args![this, buffer, size])
            .u32();
        if read != e.get(file, NiFile::m_uiBufferAllocSize) {
            e.set(file, NiFile::m_bGood, 0);
        }
    }
    let context = e.tls() + ALLOCATION_CONTEXT;
    e.mem.set_u32(context, saved);
}

// Translated from 00aff300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFile::Open` (Xbox PDB; vtable +0x20, two stack words, the first
/// unused): answers true at once when a `FILE` is already open. Otherwise
/// clears `m_uiPos`, `m_uiBufferReadSize` and `m_pBuffer` and opens
/// `pFileName` with `fopen_s` in a mode chosen by `m_eMode` (0 `"rb"`, 1
/// `"r+b"`, anything else `"a+b"`; the `t` forms when `text` is set). Mode 1
/// on a file that does not exist creates it first (`"wb"`/`"wt"`, closed at
/// once) and opens it again. Ends with [`bs_file_check_is_good`] and
/// answers `m_bGood`.
pub fn bs_file_open(e: &mut Engine, this: Ptr<BSFile>, _unused_1: u32, text: bool) -> bool {
    let file = ni_file(this);
    if e.get(file, NiFile::m_pFile) != 0 {
        return true;
    }
    e.set(file, NiFile::m_uiPos, 0);
    e.set(file, NiFile::m_uiBufferReadSize, 0);
    e.set(file, NiFile::m_pBuffer, 0);
    let mode = match (e.get(file, NiFile::m_eMode), text) {
        (0, true) => MODE_RT,
        (0, false) => MODE_RB,
        (1, true) => MODE_R_PLUS_T,
        (1, false) => MODE_R_PLUS_B,
        (_, true) => MODE_A_PLUS_T,
        (_, false) => MODE_A_PLUS_B,
    };
    let slot = this.addr() + NiFile::m_pFile.off;
    let name = this.addr() + BSFile::pFileName.off;
    e.set(file, NiFile::m_pFile, 0);
    e.call(FOPEN_S, &args![slot, name, mode]);
    if e.get(file, NiFile::m_pFile) == 0 && e.get(file, NiFile::m_eMode) == 1 {
        e.set(file, NiFile::m_pFile, 0);
        let create = if text { MODE_WT } else { MODE_WB };
        e.call(FOPEN_S, &args![slot, name, create]);
        if e.get(file, NiFile::m_pFile) != 0 {
            let handle = e.get(file, NiFile::m_pFile);
            e.call(FCLOSE, &args![handle]);
        }
        e.set(file, NiFile::m_pFile, 0);
        e.call(FOPEN_S, &args![slot, name, mode]);
    }
    bs_file_check_is_good(e, this);
    e.get(file, NiFile::m_bGood) != 0
}

/// The byte swap of read and written components (`00aa1de0`, cdecl: buffer, size, the
/// component sizes, their count).
const SWAP_COMPONENTS: u32 = 0x00aa_1de0;
/// `memcpy` (`00ec44d0`, cdecl) and `wcslen` (`00ecdb00`, cdecl).
const MEMCPY: u32 = 0x00ec_44d0;
const WCSLEN: u32 = 0x00ec_db00;
/// `BSStringT<char>` set (`004037f0`, thiscall: text, size) and append
/// (`BSStringT<char>::operator+=`, `00404820`, thiscall: text).
const STRING_SET: u32 = 0x0040_37f0;
const STRING_APPEND: u32 = 0x0040_4820;
/// The `BSStringT<wchar_t>` set (`00afff50`, `BSSystem/run_00afff50`, thiscall: text, size).
const WIDE_STRING_SET: u32 = 0x00af_ff50;
/// `strlen` (`00ec6130`, cdecl).
const STRLEN: u32 = 0x00ec_6130;
/// The `BSFile` vtable slot `Open` (+0x20, two words).
const OPEN_SLOT: u32 = 0x20;

/// Calls `m_pfnRead` (+8) with five words (`this, buffer, size, &count, 1`, the count 1)
/// and adds the answer to `m_uiAbsoluteCurrentPos`; answers it.
fn read_through_pointer(e: &mut Engine, this: Ptr<BSFile>, buffer: u32, size: u32) -> u32 {
    let read = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        let function = e.mem.u32(this.addr() + 8);
        e.call(function, &args![this, buffer, size, count, 1u32])
            .u32()
    });
    let file = ni_file(this);
    let position = e
        .get(file, NiFile::m_uiAbsoluteCurrentPos)
        .wrapping_add(read);
    e.set(file, NiFile::m_uiAbsoluteCurrentPos, position);
    read
}

/// Calls `m_pfnWrite` (+0xc) the same way, adding the answer to `m_uiAbsoluteCurrentPos`
/// and to `iTrueFilePos`; answers it.
fn write_through_pointer(e: &mut Engine, this: Ptr<BSFile>, buffer: u32, size: u32) -> u32 {
    let written = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        let function = e.mem.u32(this.addr() + 0xc);
        e.call(function, &args![this, buffer, size, count, 1u32])
            .u32()
    });
    let file = ni_file(this);
    let position = e
        .get(file, NiFile::m_uiAbsoluteCurrentPos)
        .wrapping_add(written);
    e.set(file, NiFile::m_uiAbsoluteCurrentPos, position);
    let position = e.get(this, BSFile::iTrueFilePos).wrapping_add(written);
    e.set(this, BSFile::iTrueFilePos, position);
    written
}

// Translated from 00aff2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a `BSFile`, one word): true at once when a `FILE`
/// is open (`m_pFile`); otherwise clears `m_uiPos`, `m_uiBufferReadSize` and `m_pBuffer`,
/// takes `file` as its `FILE`, checks it ([`bs_file_check_is_good`]) and answers `m_bGood`.
pub fn fn_00aff2a0(e: &mut Engine, this: Ptr<BSFile>, file: u32) -> bool {
    let stream = ni_file(this);
    if e.get(stream, NiFile::m_pFile) != 0 {
        return true;
    }
    e.set(stream, NiFile::m_uiPos, 0);
    e.set(stream, NiFile::m_uiBufferReadSize, 0);
    e.set(stream, NiFile::m_pBuffer, 0);
    e.set(stream, NiFile::m_pFile, file);
    bs_file_check_is_good(e, this);
    e.get(stream, NiFile::m_bGood) != 0
}

// Translated from 00aff640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a `BSFile`, two words): reads a wide string into
/// the `BSStringT<wchar_t>` `string`. Opens the file first (vtable +0x20 with 0, 0) when no
/// `FILE` is open. Then reads one 2-byte character at a time through `m_pfnRead` into a
/// 0x104-character local; a short read, or more than `limit` bytes read, ends the text
/// with a 0. Each full local (0x104 characters) or the end is handed to the string: the
/// first through its set (`00afff50(local, 0)`), the next ones appended
/// ([`fn_00b00120`]). Answers the bytes read over 2.
// Translation incomplete: 00aff640 - its /GS stack cookie check is not translated
pub fn fn_00aff640(e: &mut Engine, this: Ptr<BSFile>, string: u32, limit: u32) -> u32 {
    if e.get(ni_file(this), NiFile::m_pFile) == 0 {
        e.vcall(this.addr(), OPEN_SLOT, &args![0u32, 0u32]);
    }
    e.with_stack(0x20c, |e, locals| {
        let character = locals.addr();
        let text = character + 4;
        let mut index = 0u32;
        let mut handed = 0u32;
        let mut read = 0u32;
        loop {
            let got = read_through_pointer(e, this, character, 2);
            read = read.wrapping_add(got);
            if got != 2 {
                e.mem.set_u16(character, 0);
            }
            if read > limit {
                e.mem.set_u16(character, 0);
            }
            let c = e.mem.u16(character);
            e.mem.set_u16(text.wrapping_add(index.wrapping_mul(2)), c);
            index = index.wrapping_add(1);
            if index == 0x104 || c == 0 {
                if handed != 0 {
                    fn_00b00120(e, Ptr::new(string), text);
                } else {
                    e.call(WIDE_STRING_SET, &args![string, text, 0u32]);
                }
                handed = handed.wrapping_add(index);
                if e.mem.u16(character) == 0 {
                    break;
                }
                index = 0;
            }
        }
        read >> 1
    })
}

// Translated from 00aff7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFile::Read` (Xbox PDB; thiscall, two words): reads a string into the
/// `BSStringT<char>` `string`, as [`fn_00aff640`] with one byte at a time, a local of 0x103
/// characters plus the 0 written after each part, the first part set (`004037f0(local, 0)`)
/// and the next ones appended (`00404820`). Answers the bytes read.
// Translation incomplete: 00aff7c0 - its /GS stack cookie check is not translated
pub fn bs_file_read(e: &mut Engine, this: Ptr<BSFile>, string: u32, limit: u32) -> u32 {
    if e.get(ni_file(this), NiFile::m_pFile) == 0 {
        e.vcall(this.addr(), OPEN_SLOT, &args![0u32, 0u32]);
    }
    e.with_stack(0x108, |e, locals| {
        let character = locals.addr();
        let text = character + 1;
        let mut index = 0u32;
        let mut handed = 0u32;
        let mut read = 0u32;
        loop {
            let got = read_through_pointer(e, this, character, 1);
            read = read.wrapping_add(got);
            if got != 1 {
                e.mem.set_u8(character, 0);
            }
            if read > limit {
                e.mem.set_u8(character, 0);
            }
            let c = e.mem.u8(character);
            e.mem.set_u8(text.wrapping_add(index), c);
            index = index.wrapping_add(1);
            if index == 0x103 || c == 0 {
                e.mem.set_u8(text.wrapping_add(index), 0);
                if handed != 0 {
                    e.call(STRING_APPEND, &args![string, text]);
                } else {
                    e.call(STRING_SET, &args![string, text, 0u32]);
                }
                handed = handed.wrapping_add(index);
                if e.mem.u8(character) == 0 {
                    break;
                }
                index = 0;
            }
        }
        read
    })
}

// Translated from 00affa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, five words): the byte-swapping `m_pfnRead`: 0 for an
/// empty read; otherwise `stream`'s vtable +0x44 (`ReadF`) with `(buffer, size)`, then, with
/// component sizes, the swap `00aa1de0(buffer, size, sizes, count)`; answers what was read.
pub fn fn_00affa10(
    e: &mut Engine,
    stream: Ptr,
    buffer: u32,
    size: u32,
    sizes: u32,
    count: u32,
) -> u32 {
    if size == 0 {
        return 0;
    }
    let read = e
        .vcall(stream.addr(), READ_F_SLOT, &args![buffer, size])
        .u32();
    if sizes != 0 {
        e.call(SWAP_COMPONENTS, &args![buffer, size, sizes, count]);
    }
    read
}

// Translated from 00affa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, five words): the byte-swapping `m_pfnWrite`: 0 for an
/// empty write; otherwise copies `buffer` into a block from the memory manager, swaps the
/// copy (`00aa1de0`) when there are component sizes, writes it through `stream`'s vtable
/// +0x48 (`WriteF`), frees it and answers what was written.
pub fn fn_00affa60(
    e: &mut Engine,
    stream: Ptr,
    buffer: u32,
    size: u32,
    sizes: u32,
    count: u32,
) -> u32 {
    if size == 0 {
        return 0;
    }
    let copy = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    e.call(MEMCPY, &args![copy, buffer, size]);
    if sizes != 0 {
        e.call(SWAP_COMPONENTS, &args![copy, size, sizes, count]);
    }
    let written = e
        .vcall(stream.addr(), WRITE_F_SLOT, &args![copy, size])
        .u32();
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, copy]);
    written
}

/// The length of a `BSStringT`: `sLen`, or the measured length (`measure`) when it is
/// 0xFFFF.
fn string_length(e: &mut Engine, string: Ptr<BSStringT>, measure: u32) -> u32 {
    if e.get(string, BSStringT::sLen) == 0xffff {
        let text = e.get(string, BSStringT::pString);
        e.call(measure, &args![text]).u32()
    } else {
        u32::from(e.get(string, BSStringT::sLen))
    }
}

// Translated from 00affaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a `BSFile`, two words): writes the wide text of
/// the `BSStringT<wchar_t>` `string` (its length, measured by `wcslen` when `sLen` is
/// 0xFFFF, plus the terminator when `terminator` is set; 2 bytes a character) through
/// `m_pfnWrite`; answers the characters written.
pub fn fn_00affaf0(
    e: &mut Engine,
    this: Ptr<BSFile>,
    string: Ptr<BSStringT>,
    terminator: u8,
) -> u32 {
    let length = string_length(e, string, WCSLEN);
    let text = e.get(string, BSStringT::pString);
    let size = u32::from(terminator != 0).wrapping_add(length) << 1;
    write_through_pointer(e, this, text, size) >> 1
}

// Translated from 00affbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a `BSFile`, two words): writes the text of the
/// `BSStringT<char>` `string` (its length, measured by `strlen` when `sLen` is 0xFFFF, plus
/// the terminator when `terminator` is set) through `m_pfnWrite`; answers the bytes
/// written.
pub fn fn_00affbb0(
    e: &mut Engine,
    this: Ptr<BSFile>,
    string: Ptr<BSStringT>,
    terminator: u8,
) -> u32 {
    let length = string_length(e, string, STRLEN);
    let text = e.get(string, BSStringT::pString);
    let size = u32::from(terminator != 0).wrapping_add(length);
    write_through_pointer(e, this, text, size)
}

// Translated from 00b00120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSStringT<wchar_t>` append (thiscall, one word): nothing for
/// a null `text`; an empty string is set to it (`00afff50(text, 0)`); otherwise the string
/// grows (`00afff50` with its own text and the new length) when the new length passes
/// `sMaxLen`, `text` and its terminator are copied after the current text (`memcpy`), and
/// `sLen` becomes the new length (0xFFFF from 0x10000 on). Lengths are `wcslen` where `sLen`
/// is 0xFFFF. Returns `this`.
pub fn fn_00b00120(e: &mut Engine, this: Ptr<BSStringT>, text: u32) -> Ptr<BSStringT> {
    if text == 0 {
        return this;
    }
    if e.get(this, BSStringT::pString) == 0 {
        e.call(WIDE_STRING_SET, &args![this, text, 0u32]);
        return this;
    }
    let added = if text != 0 {
        e.call(WCSLEN, &args![text]).u32()
    } else {
        0
    };
    let total = string_length(e, this, WCSLEN).wrapping_add(added);
    if total > u32::from(e.get(this, BSStringT::sMaxLen)) {
        let own = e.get(this, BSStringT::pString);
        e.call(WIDE_STRING_SET, &args![this, own, total]);
    }
    let length = string_length(e, this, WCSLEN);
    let end = e
        .get(this, BSStringT::pString)
        .wrapping_add(length.wrapping_mul(2));
    e.call(
        MEMCPY,
        &args![end, text, added.wrapping_mul(2).wrapping_add(2)],
    );
    let stored = if total > 0xffff { 0xffff } else { total as u16 };
    e.set(this, BSStringT::sLen, stored);
    this
}

/// `NiFile`'s base constructor (`00aa14b0`, `thiscall`), `strlen` (`00ec6130`), `strcpy_s`
/// (`00ec65a6`), `BSFile::Open` (`00aff300`, `thiscall`, two words) and `BSFile::Exist`
/// (`00affc60`, `thiscall`, answers `AL`).
const NI_FILE_CONSTRUCT: u32 = 0x00aa_14b0;
const STRCPY_S: u32 = 0x00ec_65a6;
const BS_FILE_OPEN: u32 = 0x00af_f300;
const BS_FILE_EXIST: u32 = 0x00af_fc60;

// Translated from 00b00260 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b00260 - the SEH frame (FS:[0] handler 00f23868, state 0 after the base constructor) is not translated.
/// `BSFile::BSFile` (engine map; unit `BSSystem/bsfile_xenon.cpp`; `thiscall`, four words,
/// `RET 0x10`: path, mode, buffer size, a byte for `Open`): `NiFile`'s constructor, the vtable
/// `010a4764`, no byte swapping (`00aff990(0)`), the mode at +0x28 and the buffer size at +0x10;
/// +0x14, +0x18, +0x20, +0x24, +0x150, +0x14c, +0x148, +0x154, +0x34, +0x3c and +0x40 cleared,
/// the byte +0x30 cleared and +0x38 set to -1. A path of 0x104 characters or more leaves the name
/// at +0x44 empty; a shorter one is copied there (`strcpy_s` into 0x104 bytes) and, for mode 1,
/// the file is opened (`BSFile::Open(0, byte)`). For mode 0 the byte +0x2c is set to whether
/// the file exists (`BSFile::Exist`). Answers `this`.
pub fn bs_file_bs_file(
    e: &mut Engine,
    this: Ptr<BSFile>,
    path: Ptr,
    mode: u32,
    buffer_size: u32,
    open_byte: u8,
) -> Ptr<BSFile> {
    let t = this.addr();
    e.call(NI_FILE_CONSTRUCT, &args![t]);
    e.mem.set_u32(t, BSFILE_VTABLE);
    e.call(0x00af_f990, &args![t, 0u32]);
    e.mem.set_u32(t + 0x28, mode);
    e.mem.set_u32(t + 0x10, buffer_size);
    for offset in [0x14u32, 0x18, 0x20, 0x24, 0x150, 0x14c, 0x148, 0x154] {
        e.mem.set_u32(t + offset, 0);
    }
    e.mem.set_u8(t + 0x30, 0);
    e.mem.set_u32(t + 0x34, 0);
    e.mem.set_u32(t + 0x38, 0xffff_ffff);
    e.mem.set_u32(t + 0x3c, 0);
    e.mem.set_u32(t + 0x40, 0);
    let length = e.call(STRLEN, &args![path]).u32();
    if length >= 0x104 {
        e.mem.set_u8(t + 0x44, 0);
    } else {
        e.call(STRCPY_S, &args![t + 0x44, 0x104u32, path]);
        if mode == 1 {
            e.call(BS_FILE_OPEN, &args![t, 0u32, u32::from(open_byte)]);
        }
    }
    if mode == 0 {
        let exists = e.call(BS_FILE_EXIST, &args![t]).u8();
        e.mem.set_u8(t + 0x2c, exists);
    }
    this
}

/// The `NiFile` read (`00aa1750`, thiscall: buffer, count) and seek (`00aa16d0`, thiscall:
/// offset, origin).
const NI_FILE_READ: u32 = 0x00aa_1750;
const NI_FILE_SEEK: u32 = 0x00aa_16d0;
/// `ftell` (`00ecb562`, cdecl: stream) and `fseek` (`00ecb65a`, cdecl: stream, offset, origin).
const FTELL: u32 = 0x00ec_b562;
const FSEEK: u32 = 0x00ec_b65a;
/// The three seek origins as `NiFile` keeps them (`010a2480`, `010a2484`, `010a2488`: the
/// set, current and end words `00b00590` compares with).
const SEEK_SET_WORD: u32 = 0x010a_2480;
const SEEK_CUR_WORD: u32 = 0x010a_2484;
const SEEK_END_WORD: u32 = 0x010a_2488;

// Translated from 00b00650 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSFile::ReadF` (Xbox PDB; `BSFile` vtable +0x44, `thiscall`, two words, `RET 8`): opens the
/// file first when it has no `FILE` (`m_pFile` +0x24; vtable +0x20 with `(0, 0)`), reads through
/// the `NiFile` buffer (`00aa1750(buffer, size)`) and moves `iTrueFilePos` (+0x150) on by what
/// that answered, which it answers.
pub fn bs_file_read_f(e: &mut Engine, this: Ptr<BSFile>, buffer: Ptr, size: u32) -> u32 {
    if e.get(ni_file(this), NiFile::m_pFile) == 0 {
        e.vcall(this.addr(), OPEN_SLOT, &args![0u32, 0u32]);
    }
    let read = e.call(NI_FILE_READ, &args![this, buffer, size]).u32();
    let position = e.get(this, BSFile::iTrueFilePos).wrapping_add(read);
    e.set(this, BSFile::iTrueFilePos, position);
    read
}

// Translated from 00b00590 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0x14 (`thiscall`, two words, `RET 8`: offset,
/// origin): the seek. The target is the offset for the set origin (`010a2480`; a file in mode
/// 0, read, seeks by the distance from `iTrueFilePos` with the current origin `010a2484`),
/// `iTrueFilePos` plus the offset for the current origin, the size (vtable +0x28) less the
/// offset for the end origin (`010a2488`), else `iTrueFilePos`. When the target differs from
/// `iTrueFilePos` the `NiFile` seek runs (`00aa16d0(offset, origin)`, both as adjusted) and
/// `iTrueFilePos` becomes the target.
pub fn fn_00b00590(e: &mut Engine, this: Ptr<BSFile>, offset: u32, origin: u32) {
    let mut offset = offset;
    let mut origin = origin;
    let target = if origin == e.global::<u32>(SEEK_SET_WORD) {
        let target = offset;
        if e.get(ni_file(this), NiFile::m_eMode) == 0 {
            origin = e.global::<u32>(SEEK_CUR_WORD);
            offset = offset.wrapping_sub(e.get(this, BSFile::iTrueFilePos));
        }
        target
    } else if origin == e.global::<u32>(SEEK_CUR_WORD) {
        e.get(this, BSFile::iTrueFilePos).wrapping_add(offset)
    } else if origin == e.global::<u32>(SEEK_END_WORD) {
        e.vcall(this.addr(), GET_SIZE_SLOT, &args![])
            .u32()
            .wrapping_sub(offset)
    } else {
        e.get(this, BSFile::iTrueFilePos)
    };
    if target != e.get(this, BSFile::iTrueFilePos) {
        e.call(NI_FILE_SEEK, &args![this, offset, origin]);
        e.set(this, BSFile::iTrueFilePos, target);
    }
}

// Translated from 00b004e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `BSFile` vtable +0x28 (`GetSize` by the PDB's slot order;
/// `thiscall`, no stack words, plain `RET`): clears `iFileSize` (+0x154), flushes the `NiFile`
/// buffer of a file in mode 1 (`00aa15a0`), opens the file (vtable +0x20 with `(0, 0)`), and
/// with a `FILE` measures it: `ftell`, `fseek(0, SEEK_END)`, `ftell` into `iFileSize`, and
/// `fseek` back to the first position. Answers `iFileSize`.
pub fn fn_00b004e0(e: &mut Engine, this: Ptr<BSFile>) -> u32 {
    e.set(this, BSFile::iFileSize, 0);
    if e.get(ni_file(this), NiFile::m_eMode) == 1 {
        e.call(NI_FILE_FLUSH, &args![this]);
    }
    e.vcall(this.addr(), OPEN_SLOT, &args![0u32, 0u32]);
    if e.get(ni_file(this), NiFile::m_pFile) != 0 {
        let file = e.get(ni_file(this), NiFile::m_pFile);
        let here = e.call(FTELL, &args![file]).u32();
        let file = e.get(ni_file(this), NiFile::m_pFile);
        e.call(FSEEK, &args![file, 0u32, 2u32]);
        let file = e.get(ni_file(this), NiFile::m_pFile);
        let size = e.call(FTELL, &args![file]).u32();
        e.set(this, BSFile::iFileSize, size);
        let file = e.get(ni_file(this), NiFile::m_pFile);
        e.call(FSEEK, &args![file, here, 0u32]);
    }
    e.get(this, BSFile::iFileSize)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_0650, bs_file_read_f(Ptr<BSFile>, Ptr, u32) -> u32),
        entry!(0x00b0_0590, fn_00b00590(Ptr<BSFile>, u32, u32)),
        entry!(0x00b0_04e0, fn_00b004e0(Ptr<BSFile>) -> u32),
        entry!(
            0x00b0_0260,
            bs_file_bs_file(Ptr<BSFile>, Ptr, u32, u32, u8) -> Ptr<BSFile>
        ),
        entry!(0x00af_f2a0, fn_00aff2a0(Ptr<BSFile>, u32) -> bool),
        entry!(0x00af_f640, fn_00aff640(Ptr<BSFile>, u32, u32) -> u32),
        entry!(0x00af_f7c0, bs_file_read(Ptr<BSFile>, u32, u32) -> u32),
        entry!(0x00af_fa10, fn_00affa10(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00af_fa60, fn_00affa60(Ptr, u32, u32, u32, u32) -> u32),
        entry!(
            0x00af_faf0,
            fn_00affaf0(Ptr<BSFile>, Ptr<BSStringT>, u8) -> u32
        ),
        entry!(
            0x00af_fbb0,
            fn_00affbb0(Ptr<BSFile>, Ptr<BSStringT>, u8) -> u32
        ),
        entry!(
            0x00b0_0120,
            fn_00b00120(Ptr<BSStringT>, u32) -> Ptr<BSStringT>
        ),
        entry!(0x00af_f990, fn_00aff990(Ptr<BSFile>, bool)),
        entry!(0x00af_f9d0, fn_00aff9d0(Ptr, Ptr, u32) -> u32),
        entry!(0x00af_f9f0, fn_00aff9f0(Ptr, Ptr, u32) -> u32),
        entry!(0x00af_f610, fn_00aff610(Ptr<BSFile>) -> u32),
        entry!(0x00af_f950, fn_00aff950(Ptr<BSFile>, Ptr, u32) -> u32),
        entry!(0x00af_fc60, bs_file_exist(Ptr<BSFile>) -> bool),
        entry!(0x00af_fc90, fn_00affc90(Ptr<BSFile>, Ptr, u32, u16) -> u32),
        entry!(0x00af_fd10, bs_file_close(Ptr<BSFile>)),
        entry!(0x00af_f240, bs_file_destructor(Ptr<BSFile>)),
        entry!(0x00af_ff20, fn_00afff20(Ptr<BSFile>, u32) -> Ptr<BSFile>),
        entry!(0x00af_f490, bs_file_check_is_good(Ptr<BSFile>)),
        entry!(0x00af_f300, bs_file_open(Ptr<BSFile>, u32, bool) -> bool),
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

    const VTABLE: u32 = 0x0300_0000;
    const GET_FILE_SIZE: u32 = 0x0070_0000;
    const GET_SIZE: u32 = 0x0070_0010;
    const READ_F: u32 = 0x0070_0020;
    const WRITE_F: u32 = 0x0070_0030;

    /// A `BSFile` with a test vtable whose slots +0x1c, +0x28, +0x44 and
    /// +0x48 are the doubles above (they answer 0x40, 0, 7 and 9), and
    /// doubles for every callee outside this file.
    fn engine() -> (Engine, Ptr<BSFile>) {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        let mut slots = vec![0u32; 0x4c / 4 + 1];
        slots[GET_FILE_SIZE_SLOT as usize / 4] = GET_FILE_SIZE;
        slots[GET_SIZE_SLOT as usize / 4] = GET_SIZE;
        slots[READ_F_SLOT as usize / 4] = READ_F;
        slots[WRITE_F_SLOT as usize / 4] = WRITE_F;
        e.put_vtable(VTABLE, &slots);
        let this: Ptr<BSFile> = e.new_object();
        e.mem.set_u32(this.addr(), VTABLE);
        e.register(GET_FILE_SIZE, |_, _| 0x40u32.into_ret());
        e.register(GET_SIZE, |_, _| Ret::default());
        e.register(READ_F, |_, _| 7u32.into_ret());
        e.register(WRITE_F, |_, _| 9u32.into_ret());
        for callee in [
            ACCESS,
            FOPEN_S,
            FCLOSE,
            NI_FILE_FLUSH,
            NI_FILE_READ_DIRECT,
            NI_FILE_WRITE,
            NI_FILE_DESTRUCTOR,
            MEMORY_MANAGER_ALLOCATE,
            MEMORY_MANAGER_DEALLOCATE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn set_endian_swap_installs_the_read_and_write_pair() {
        let (mut e, this) = engine();
        e.call(0x00af_f990, &args![this, true]);
        assert_eq!(e.mem.u32(this.addr() + 8), SWAPPING_READ);
        assert_eq!(e.mem.u32(this.addr() + 0xc), SWAPPING_WRITE);
        e.call(0x00af_f990, &args![this, false]);
        assert_eq!(e.mem.u32(this.addr() + 8), PLAIN_READ);
        assert_eq!(e.mem.u32(this.addr() + 0xc), PLAIN_WRITE);
    }

    #[test]
    fn the_plain_pair_calls_read_f_and_write_f() {
        let (mut e, this) = engine();
        assert_eq!(e.call(0x00af_f9d0, &args![this, 0x900u32, 3u32]).u32(), 7);
        assert_eq!(e.call(0x00af_f9f0, &args![this, 0x900u32, 3u32]).u32(), 9);
        assert_eq!(calls_to(&e, READ_F), vec![vec![this.addr(), 0x900, 3]]);
        assert_eq!(calls_to(&e, WRITE_F), vec![vec![this.addr(), 0x900, 3]]);
    }

    #[test]
    fn get_file_size_asks_once_the_size_is_known() {
        let (mut e, this) = engine();
        e.register(GET_SIZE, |e, a| {
            e.mem.set_u32(a[0] + 0x154, 0x99);
            Ret::default()
        });
        assert_eq!(e.call(0x00af_f610, &args![this]).u32(), 0x99);
        assert_eq!(e.call(0x00af_f610, &args![this]).u32(), 0x99);
        assert_eq!(calls_to(&e, GET_SIZE).len(), 1);
    }

    #[test]
    fn write_f_counts_the_true_file_position() {
        let (mut e, this) = engine();
        e.register(NI_FILE_WRITE, |_, _| 5u32.into_ret());
        e.set(this, BSFile::iTrueFilePos, 10);
        assert_eq!(e.call(0x00af_f950, &args![this, 0x900u32, 6u32]).u32(), 5);
        assert_eq!(e.get(this, BSFile::iTrueFilePos), 15);
        assert_eq!(
            calls_to(&e, NI_FILE_WRITE),
            vec![vec![this.addr(), 0x900, 6]]
        );
    }

    #[test]
    fn exist_asks_access_for_the_file_name() {
        let (mut e, this) = engine();
        assert!(e.call(0x00af_fc60, &args![this]).bool());
        assert_eq!(calls_to(&e, ACCESS), vec![vec![this.addr() + 0x44, 0]]);
        e.register(ACCESS, |_, _| u32::MAX.into_ret());
        assert!(!e.call(0x00af_fc60, &args![this]).bool());
    }

    /// `ReadF` serving the bytes of `text` one at a time, then short reads.
    fn serve(e: &mut Engine, text: &'static [u8]) {
        let next = std::rc::Rc::new(std::cell::Cell::new(0usize));
        e.register_double(READ_F, move |e, a| {
            let at = next.get();
            if at >= text.len() {
                return 0u32.into_ret();
            }
            e.mem.set_u8(a[1], text[at]);
            next.set(at + 1);
            1u32.into_ret()
        });
    }

    #[test]
    fn get_line_stops_at_the_terminator() {
        let (mut e, this) = engine();
        serve(&mut e, b"ab\ncd");
        let buffer = e.mem.alloc(0x10);
        let read = e
            .call(0x00af_fc90, &args![this, buffer, 0x10u32, 0x0au16])
            .u32();
        assert_eq!(read, 3);
        assert_eq!(e.mem.cstr(buffer), b"ab");
    }

    #[test]
    fn get_line_stops_at_the_size_and_at_a_short_read() {
        let (mut e, this) = engine();
        serve(&mut e, b"abcdef");
        let buffer = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x00af_fc90, &args![this, buffer, 4u32, 0x0au16])
                .u32(),
            3
        );
        assert_eq!(e.mem.cstr(buffer), b"abc");
        let (mut e, this) = engine();
        serve(&mut e, b"xy");
        let buffer = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x00af_fc90, &args![this, buffer, 0x10u32, 0x0au16])
                .u32(),
            2
        );
        assert_eq!(e.mem.cstr(buffer), b"xy");
    }

    #[test]
    fn get_line_compares_a_signed_byte_with_the_terminator() {
        // 0xff sign-extends to -1, which never equals a 16-bit terminator.
        let (mut e, this) = engine();
        serve(&mut e, b"\xff\xff");
        let buffer = e.mem.alloc(0x10);
        e.call(0x00af_fc90, &args![this, buffer, 3u32, 0xffu16]);
        assert_eq!(e.mem.cstr(buffer), b"\xff\xff");
    }

    #[test]
    fn close_flushes_and_closes_a_good_file_then_frees_the_buffer() {
        let (mut e, this) = engine();
        let file = this.cast::<NiFile>();
        e.set(file, NiFile::m_bGood, 1);
        e.set(file, NiFile::m_pFile, 0x7700);
        e.set(file, NiFile::m_pBuffer, 0x6600);
        e.call(0x00af_fd10, &args![this]);
        assert_eq!(calls_to(&e, NI_FILE_FLUSH), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, FCLOSE), vec![vec![0x7700]]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x6600]]
        );
        assert_eq!(e.get(file, NiFile::m_pFile), 0);
        assert_eq!(e.get(file, NiFile::m_pBuffer), 0);
        // A bad file is not flushed or closed.
        let (mut e, this) = engine();
        e.set(this.cast::<NiFile>(), NiFile::m_pFile, 0x7700);
        e.call(0x00af_fd10, &args![this]);
        assert!(calls_to(&e, FCLOSE).is_empty());
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
    }

    #[test]
    fn destructor_closes_and_runs_the_ni_file_destructor() {
        let (mut e, this) = engine();
        e.call(0x00af_f240, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), BSFILE_VTABLE);
        assert_eq!(calls_to(&e, NI_FILE_DESTRUCTOR), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
    }

    #[test]
    fn deleting_destructor_frees_with_bit_zero() {
        let (mut e, this) = engine();
        assert_eq!(e.call(0x00af_ff20, &args![this, 0u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
        e.call(0x00af_ff20, &args![this, 1u32]);
        let frees = calls_to(&e, MEMORY_MANAGER_DEALLOCATE);
        assert_eq!(frees.len(), 3);
        assert_eq!(frees[2], vec![MEMORY_MANAGER, this.addr()]);
    }

    #[test]
    fn check_is_good_without_a_file_is_bad() {
        let (mut e, this) = engine();
        e.set(this.cast::<NiFile>(), NiFile::m_bGood, 1);
        e.call(0x00af_f490, &args![this]);
        assert_eq!(e.get(this.cast::<NiFile>(), NiFile::m_bGood), 0);
    }

    #[test]
    fn check_is_good_allocates_the_buffer_in_context_0x17() {
        let (mut e, this) = engine();
        let file = this.cast::<NiFile>();
        e.set(file, NiFile::m_pFile, 0x7700);
        e.set(file, NiFile::m_uiBufferAllocSize, 0x20);
        let context = e.tls() + ALLOCATION_CONTEXT;
        e.mem.set_u32(context, 5);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, _| {
            let context = e.tls() + ALLOCATION_CONTEXT;
            assert_eq!(e.mem.u32(context), FILE_BUFFER_CONTEXT);
            0x6600u32.into_ret()
        });
        e.call(0x00af_f490, &args![this]);
        assert_eq!(e.get(file, NiFile::m_bGood), 1);
        assert_eq!(e.get(file, NiFile::m_pBuffer), 0x6600);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x20]]
        );
        assert_eq!(e.mem.u32(context), 5);
        assert!(calls_to(&e, NI_FILE_READ_DIRECT).is_empty());
        // A buffer already there, or no buffer size, allocates nothing.
        e.call(0x00af_f490, &args![this]);
        e.set(file, NiFile::m_pBuffer, 0);
        e.set(file, NiFile::m_uiBufferAllocSize, 0);
        e.call(0x00af_f490, &args![this]);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).len(), 1);
    }

    #[test]
    fn check_is_good_reads_the_whole_file_for_size_minus_one() {
        let (mut e, this) = engine();
        let file = this.cast::<NiFile>();
        e.set(file, NiFile::m_pFile, 0x7700);
        e.set(file, NiFile::m_uiBufferAllocSize, u32::MAX);
        e.set(file, NiFile::m_uiPos, 3);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0x6600u32.into_ret());
        e.register(NI_FILE_READ_DIRECT, |_, _| 0x40u32.into_ret());
        e.call(0x00af_f490, &args![this]);
        assert_eq!(e.get(file, NiFile::m_uiBufferAllocSize), 0x40);
        assert_eq!(e.get(file, NiFile::m_uiBufferReadSize), 0x40);
        assert_eq!(e.get(file, NiFile::m_uiPos), 0);
        assert_eq!(
            calls_to(&e, NI_FILE_READ_DIRECT),
            vec![vec![this.addr(), 0x6600, 0x40]]
        );
        assert_eq!(e.get(file, NiFile::m_bGood), 1);
        // A short read makes the file bad.
        let (mut e, this) = engine();
        let file = this.cast::<NiFile>();
        e.set(file, NiFile::m_pFile, 0x7700);
        e.set(file, NiFile::m_uiBufferAllocSize, u32::MAX);
        e.register(NI_FILE_READ_DIRECT, |_, _| 0x3fu32.into_ret());
        e.call(0x00af_f490, &args![this]);
        assert_eq!(e.get(file, NiFile::m_bGood), 0);
    }

    /// `fopen_s` opening the given answers in turn (written to `*slot`).
    fn opens(e: &mut Engine, answers: &'static [u32]) {
        let next = std::rc::Rc::new(std::cell::Cell::new(0usize));
        e.register_double(FOPEN_S, move |e, a| {
            let at = next.get();
            e.mem.set_u32(a[0], answers[at]);
            next.set(at + 1);
            Ret::default()
        });
    }

    #[test]
    fn open_picks_the_mode_from_the_open_mode_and_text_flag() {
        for (mode, text, want) in [
            (0u32, false, MODE_RB),
            (0, true, MODE_RT),
            (1, false, MODE_R_PLUS_B),
            (1, true, MODE_R_PLUS_T),
            (2, false, MODE_A_PLUS_B),
            (5, true, MODE_A_PLUS_T),
        ] {
            let (mut e, this) = engine();
            let file = this.cast::<NiFile>();
            e.set(file, NiFile::m_eMode, mode);
            e.set(file, NiFile::m_uiPos, 3);
            e.set(file, NiFile::m_uiBufferReadSize, 4);
            opens(&mut e, &[0x7700]);
            assert!(e.call(0x00af_f300, &args![this, 0u32, text]).bool());
            assert_eq!(
                calls_to(&e, FOPEN_S),
                vec![vec![this.addr() + 0x24, this.addr() + 0x44, want]]
            );
            assert_eq!(e.get(file, NiFile::m_uiPos), 0);
            assert_eq!(e.get(file, NiFile::m_uiBufferReadSize), 0);
            assert_eq!(e.get(file, NiFile::m_bGood), 1);
        }
    }

    #[test]
    fn open_of_an_open_file_answers_true() {
        let (mut e, this) = engine();
        e.set(this.cast::<NiFile>(), NiFile::m_pFile, 0x7700);
        assert!(e.call(0x00af_f300, &args![this, 0u32, false]).bool());
        assert!(calls_to(&e, FOPEN_S).is_empty());
    }

    #[test]
    fn open_for_update_creates_a_missing_file() {
        let (mut e, this) = engine();
        e.set(this.cast::<NiFile>(), NiFile::m_eMode, 1);
        opens(&mut e, &[0, 0x7100, 0x7700]);
        assert!(e.call(0x00af_f300, &args![this, 0u32, false]).bool());
        let modes: Vec<u32> = calls_to(&e, FOPEN_S).iter().map(|c| c[2]).collect();
        assert_eq!(modes, vec![MODE_R_PLUS_B, MODE_WB, MODE_R_PLUS_B]);
        assert_eq!(calls_to(&e, FCLOSE), vec![vec![0x7100]]);
        assert_eq!(e.get(this.cast::<NiFile>(), NiFile::m_pFile), 0x7700);
        // Creation failing too: nothing to close, and the file stays bad.
        let (mut e, this) = engine();
        e.set(this.cast::<NiFile>(), NiFile::m_eMode, 1);
        opens(&mut e, &[0, 0, 0]);
        assert!(!e.call(0x00af_f300, &args![this, 0u32, true]).bool());
        let modes: Vec<u32> = calls_to(&e, FOPEN_S).iter().map(|c| c[2]).collect();
        assert_eq!(modes, vec![MODE_R_PLUS_T, MODE_WT, MODE_R_PLUS_T]);
        assert!(calls_to(&e, FCLOSE).is_empty());
        // Read mode does not retry.
        let (mut e, this) = engine();
        opens(&mut e, &[0]);
        assert!(!e.call(0x00af_f300, &args![this, 0u32, false]).bool());
        assert_eq!(calls_to(&e, FOPEN_S).len(), 1);
    }
}

#[cfg(test)]
mod n083_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    const VTABLE: u32 = 0x0300_0000;
    const OPEN: u32 = 0x0070_0020;
    const READ_F: u32 = 0x0070_0044;
    const WRITE_F: u32 = 0x0070_0048;
    const READ_POINTER: u32 = 0x0070_0100;
    const WRITE_POINTER: u32 = 0x0070_0104;

    /// A `BSFile` with a test vtable (+0x20 `Open`, +0x44 `ReadF` answering 7, +0x48
    /// `WriteF` answering 9) and an open `FILE`; `m_pfnRead` serves `data` from its start,
    /// `m_pfnWrite` answers the size it is given.
    fn file(e: &mut Engine, data: Vec<u8>) -> Ptr<BSFile> {
        let mut slots = vec![0u32; 0x4c / 4 + 1];
        slots[OPEN_SLOT as usize / 4] = OPEN;
        slots[READ_F_SLOT as usize / 4] = READ_F;
        slots[WRITE_F_SLOT as usize / 4] = WRITE_F;
        e.put_vtable(VTABLE, &slots);
        let this: Ptr<BSFile> = e.new_object();
        e.mem.set_u32(this.addr(), VTABLE);
        e.set(ni_file(this), NiFile::m_pFile, 0x5000);
        e.register(OPEN, |_, _| Ret::default());
        e.register(READ_F, |_, _| 7u32.into_ret());
        e.register(WRITE_F, |_, _| 9u32.into_ret());
        e.mem.set_u32(this.addr() + 8, READ_POINTER);
        e.mem.set_u32(this.addr() + 0xc, WRITE_POINTER);
        let at = Rc::new(RefCell::new(0usize));
        e.register_double(READ_POINTER, move |e, w| {
            let from = *at.borrow();
            let size = w[2] as usize;
            let end = (from + size).min(data.len());
            e.mem.write(w[1], &data[from..end]);
            *at.borrow_mut() = end;
            ((end - from) as u32).into_ret()
        });
        e.register(WRITE_POINTER, |_, w| w[2].into_ret());
        e.call_log = Some(vec![]);
        this
    }

    #[test]
    fn open_with_a_file_checks_it() {
        let mut e = Engine::new();
        let this = file(&mut e, vec![]);
        assert!(e.call(0x00af_f2a0, &args![this, 0x6000u32]).bool());
        assert_eq!(e.get(ni_file(this), NiFile::m_pFile), 0x5000);
        e.set(ni_file(this), NiFile::m_pFile, 0);
        e.set(ni_file(this), NiFile::m_uiPos, 3);
        e.set(ni_file(this), NiFile::m_pBuffer, 4);
        assert!(e.call(0x00af_f2a0, &args![this, 0x6000u32]).bool());
        assert_eq!(e.get(ni_file(this), NiFile::m_pFile), 0x6000);
        assert_eq!(e.get(ni_file(this), NiFile::m_uiPos), 0);
        assert_eq!(e.get(ni_file(this), NiFile::m_pBuffer), 0);
        e.set(ni_file(this), NiFile::m_pFile, 0);
        assert!(!e.call(0x00af_f2a0, &args![this, 0u32]).bool());
    }

    fn wide(text: &str) -> Vec<u8> {
        text.encode_utf16().flat_map(u16::to_le_bytes).collect()
    }

    #[test]
    fn wide_string_is_read_up_to_its_end_or_the_limit() {
        let mut e = Engine::new();
        let this = file(&mut e, wide("hi\0rest"));
        e.register(WIDE_STRING_SET, |_, _| Ret::default());
        let string = e.mem.alloc(8);
        e.register_double(WIDE_STRING_SET, |e, w| {
            let text: Vec<u16> = (0..4).map(|i| e.mem.u16(w[1] + 2 * i)).collect();
            assert_eq!(text[..3], [u16::from(b'h'), u16::from(b'i'), 0]);
            Ret::default()
        });
        assert_eq!(e.call(0x00af_f640, &args![this, string, 0x100u32]).u32(), 3);
        assert_eq!(calls_to(&e, WIDE_STRING_SET)[0][2], 0);
        assert!(calls_to(&e, OPEN).is_empty());
        assert_eq!(e.get(ni_file(this), NiFile::m_uiAbsoluteCurrentPos), 6);
        // A limit of 2 bytes ends the text after one character; no file opens it.
        let this = file(&mut e, wide("hi\0"));
        e.set(ni_file(this), NiFile::m_pFile, 0);
        e.register_double(WIDE_STRING_SET, |e, w| {
            assert_eq!([e.mem.u16(w[1]), e.mem.u16(w[1] + 2)], [u16::from(b'h'), 0]);
            Ret::default()
        });
        assert_eq!(e.call(0x00af_f640, &args![this, string, 2u32]).u32(), 2);
        assert_eq!(calls_to(&e, OPEN), vec![vec![this.addr(), 0, 0]]);
    }

    #[test]
    fn string_is_read_in_parts() {
        let mut e = Engine::new();
        let mut data = vec![b'a'; 0x105];
        data.push(0);
        let this = file(&mut e, data);
        let parts = Rc::new(RefCell::new(Vec::<Vec<u8>>::new()));
        let seen = parts.clone();
        e.register_double(STRING_SET, move |e, w| {
            seen.borrow_mut().push(e.mem.cstr(w[1]));
            Ret::default()
        });
        let seen = parts.clone();
        e.register_double(STRING_APPEND, move |e, w| {
            seen.borrow_mut().push(e.mem.cstr(w[1]));
            Ret::default()
        });
        let string = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00af_f7c0, &args![this, string, 0x1000u32]).u32(),
            0x106
        );
        assert_eq!(calls_to(&e, STRING_SET).len(), 1);
        assert_eq!(calls_to(&e, STRING_SET)[0][2], 0);
        assert_eq!(calls_to(&e, STRING_APPEND).len(), 1);
        assert_eq!(parts.borrow()[0], vec![b'a'; 0x103]);
        assert_eq!(parts.borrow()[1], vec![b'a'; 2]);
        // A short read ends the text.
        let this = file(&mut e, b"ab".to_vec());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00af_f7c0, &args![this, string, 0x1000u32]).u32(),
            2
        );
        assert_eq!(parts.borrow()[2], b"ab");
    }

    #[test]
    fn swapping_read_and_write() {
        let mut e = Engine::new();
        let this = file(&mut e, vec![]);
        e.register(SWAP_COMPONENTS, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0x7000u32.into_ret());
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.register(MEMCPY, |_, _| Ret::default());
        assert_eq!(
            e.call(0x00af_fa10, &args![this, 0x100u32, 0u32, 1u32, 2u32])
                .u32(),
            0
        );
        assert!(calls_to(&e, READ_F).is_empty());
        assert_eq!(
            e.call(0x00af_fa10, &args![this, 0x100u32, 8u32, 0u32, 2u32])
                .u32(),
            7
        );
        assert!(calls_to(&e, SWAP_COMPONENTS).is_empty());
        e.call(0x00af_fa10, &args![this, 0x100u32, 8u32, 0x200u32, 2u32]);
        assert_eq!(calls_to(&e, READ_F)[1], [this.addr(), 0x100, 8]);
        assert_eq!(
            calls_to(&e, SWAP_COMPONENTS),
            vec![vec![0x100, 8, 0x200, 2]]
        );
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00af_fa60, &args![this, 0x100u32, 0u32, 1u32, 2u32])
                .u32(),
            0
        );
        assert!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
        assert_eq!(
            e.call(0x00af_fa60, &args![this, 0x100u32, 8u32, 0x200u32, 2u32])
                .u32(),
            9
        );
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x7000, 0x100, 8]]);
        assert_eq!(
            calls_to(&e, SWAP_COMPONENTS),
            vec![vec![0x7000, 8, 0x200, 2]]
        );
        assert_eq!(calls_to(&e, WRITE_F), vec![vec![this.addr(), 0x7000, 8]]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x7000]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00af_fa60, &args![this, 0x100u32, 8u32, 0u32, 2u32]);
        assert!(calls_to(&e, SWAP_COMPONENTS).is_empty());
    }

    #[test]
    fn strings_are_written_with_or_without_terminator() {
        let mut e = Engine::new();
        let this = file(&mut e, vec![]);
        e.register(WCSLEN, |_, _| 5u32.into_ret());
        e.register(STRLEN, |_, _| 6u32.into_ret());
        let string: Ptr<BSStringT> = e.new_object();
        e.set(string, BSStringT::pString, 0x8000);
        e.set(string, BSStringT::sLen, 3);
        assert_eq!(e.call(0x00af_faf0, &args![this, string, 0u8]).u32(), 3);
        assert_eq!(calls_to(&e, WRITE_POINTER)[0][1..3], [0x8000, 6]);
        assert_eq!(e.call(0x00af_faf0, &args![this, string, 1u8]).u32(), 4);
        e.set(string, BSStringT::sLen, 0xffff);
        assert_eq!(e.call(0x00af_faf0, &args![this, string, 1u8]).u32(), 6);
        assert_eq!(calls_to(&e, WCSLEN), vec![vec![0x8000]]);
        assert_eq!(e.call(0x00af_fbb0, &args![this, string, 1u8]).u32(), 7);
        assert_eq!(calls_to(&e, STRLEN), vec![vec![0x8000]]);
        e.set(string, BSStringT::sLen, 3);
        assert_eq!(e.call(0x00af_fbb0, &args![this, string, 0u8]).u32(), 3);
        // Each write moves both positions.
        assert_eq!(e.get(this, BSFile::iTrueFilePos), 6 + 8 + 12 + 7 + 3);
        assert_eq!(
            e.get(ni_file(this), NiFile::m_uiAbsoluteCurrentPos),
            6 + 8 + 12 + 7 + 3
        );
    }

    #[test]
    fn wide_append_sets_grows_and_copies() {
        let mut e = Engine::new();
        e.register(WIDE_STRING_SET, |_, _| Ret::default());
        e.register(MEMCPY, |_, _| Ret::default());
        e.register_double(WCSLEN, |_, w| (w[0] & 0xff).into_ret());
        e.call_log = Some(vec![]);
        let string: Ptr<BSStringT> = e.new_object();
        // Null text: nothing.
        assert_eq!(
            e.call(0x00b0_0120, &args![string, 0u32]).u32(),
            string.addr()
        );
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // Empty string: set.
        e.call(0x00b0_0120, &args![string, 0x9003u32]);
        assert_eq!(
            calls_to(&e, WIDE_STRING_SET),
            vec![vec![string.addr(), 0x9003, 0]]
        );
        // Room enough: copied after the text.
        e.set(string, BSStringT::pString, 0x8000);
        e.set(string, BSStringT::sLen, 2);
        e.set(string, BSStringT::sMaxLen, 8);
        e.call_log = Some(vec![]);
        e.call(0x00b0_0120, &args![string, 0x9003u32]);
        assert!(calls_to(&e, WIDE_STRING_SET).is_empty());
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x8004, 0x9003, 8]]);
        assert_eq!(e.get(string, BSStringT::sLen), 5);
        // Past sMaxLen: grown first; a length of 0xFFFF is measured.
        e.set(string, BSStringT::sMaxLen, 4);
        e.set(string, BSStringT::sLen, 0xffff);
        e.call_log = Some(vec![]);
        e.call(0x00b0_0120, &args![string, 0x9007u32]);
        assert_eq!(
            calls_to(&e, WIDE_STRING_SET),
            vec![vec![string.addr(), 0x8000, 7]]
        );
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x8000, 0x9007, 16]]);
        assert_eq!(e.get(string, BSStringT::sLen), 7);
        // From 0x10000 on the length word holds 0xFFFF.
        e.set(string, BSStringT::sLen, 0xfff0);
        e.set(string, BSStringT::sMaxLen, 0xffff);
        e.call(0x00b0_0120, &args![string, 0x9020u32]);
        assert_eq!(e.get(string, BSStringT::sLen), 0xffff);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        for f in [NI_FILE_CONSTRUCT, 0x00af_f990, BS_FILE_OPEN] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(BS_FILE_EXIST, |_, _| 1u32.into_ret());
        let file = e.mem.alloc(0x158);
        let path = e.mem.alloc(0x200);
        e.mem.set_cstr(path, b"Data.txt");
        (e, file, path)
    }

    #[test]
    fn mode_0_copies_the_name_and_asks_whether_it_exists() {
        let (mut e, file, path) = engine();
        e.call_log = Some(vec![]);
        let made = e
            .call(0x00b0_0260, &args![file, path, 0u32, 0x4000u32, 1u32])
            .u32();
        assert_eq!(made, file);
        assert_eq!(e.mem.u32(file), BSFILE_VTABLE);
        assert_eq!(e.mem.u32(file + 0x10), 0x4000);
        assert_eq!(e.mem.u32(file + 0x38), 0xffff_ffff);
        assert_eq!(e.mem.cstr(file + 0x44), b"Data.txt".to_vec());
        assert_eq!(e.mem.u8(file + 0x2c), 1);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != BS_FILE_OPEN));
    }

    #[test]
    fn mode_1_opens_and_a_long_name_stays_empty() {
        let (mut e, file, path) = engine();
        e.call_log = Some(vec![]);
        e.call(0x00b0_0260, &args![file, path, 1u32, 0u32, 1u32]);
        let log = e.call_log.take().unwrap();
        let opens: Vec<_> = log.iter().filter(|c| c.0 == BS_FILE_OPEN).collect();
        assert_eq!(opens[0].1, vec![file, 0, 1]);
        let long = vec![b'a'; 0x104];
        e.mem.set_cstr(path, &long);
        e.call(0x00b0_0260, &args![file, path, 1u32, 0u32, 1u32]);
        assert_eq!(e.mem.u8(file + 0x44), 0);
    }
}

#[cfg(test)]
mod tests_read_seek_size {
    use super::*;

    const VTABLE: u32 = 0x0300_0000;
    const OPEN_DOUBLE: u32 = 0x0e10_0000;
    const SIZE_DOUBLE: u32 = 0x0e10_0010;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A `BSFile` whose vtable's `Open` and `GetSize` are doubles (size 0x100), with the seek
    /// origin words 0, 1, 2 and the `NiFile` read answering its count.
    fn engine() -> (Engine, Ptr<BSFile>) {
        let mut e = Engine::new();
        e.map(0x010a_2000, 0x1000);
        e.set_global(SEEK_SET_WORD, 0u32);
        e.set_global(SEEK_CUR_WORD, 1u32);
        e.set_global(SEEK_END_WORD, 2u32);
        let mut slots = vec![0u32; 0x14];
        slots[(OPEN_SLOT / 4) as usize] = OPEN_DOUBLE;
        slots[(GET_SIZE_SLOT / 4) as usize] = SIZE_DOUBLE;
        e.put_vtable(VTABLE, &slots);
        e.register(OPEN_DOUBLE, |_, _| 1u32.into_ret());
        e.register(SIZE_DOUBLE, |_, _| 0x100u32.into_ret());
        e.register(NI_FILE_READ, |_, a| a[2].into_ret());
        e.register(NI_FILE_SEEK, |_, _| Ret::default());
        e.register(NI_FILE_FLUSH, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x158);
        e.mem.set_u32(this, VTABLE);
        e.call_log = Some(vec![]);
        (e, Ptr::new(this))
    }

    #[test]
    fn read_f_opens_a_closed_file_and_counts_the_position() {
        let (mut e, this) = engine();
        let buffer = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00b0_0650, &args![this, buffer, 8u32]).u32(), 8);
        assert_eq!(calls_to(&e, OPEN_DOUBLE), [vec![this.addr(), 0, 0]]);
        e.set(ni_file(this), NiFile::m_pFile, 0x7700);
        assert_eq!(e.call(0x00b0_0650, &args![this, buffer, 4u32]).u32(), 4);
        assert_eq!(calls_to(&e, OPEN_DOUBLE).len(), 1);
        assert_eq!(e.get(this, BSFile::iTrueFilePos), 12);
    }

    #[test]
    fn the_seek_moves_only_when_the_target_differs() {
        let (mut e, this) = engine();
        e.set(this, BSFile::iTrueFilePos, 0x10);
        // Set, read mode: by the distance with the current origin.
        e.call(0x00b0_0590, &args![this, 0x30u32, 0u32]);
        assert_eq!(calls_to(&e, NI_FILE_SEEK), [vec![this.addr(), 0x20, 1]]);
        assert_eq!(e.get(this, BSFile::iTrueFilePos), 0x30);
        // The same place: nothing.
        e.call(0x00b0_0590, &args![this, 0x30u32, 0u32]);
        e.call(0x00b0_0590, &args![this, 0u32, 1u32]);
        e.call(0x00b0_0590, &args![this, 0u32, 7u32]);
        assert_eq!(calls_to(&e, NI_FILE_SEEK).len(), 1);
        // From the end of 0x100 bytes; a write-mode set keeps its origin.
        e.call(0x00b0_0590, &args![this, 0x10u32, 2u32]);
        assert_eq!(calls_to(&e, NI_FILE_SEEK)[1], vec![this.addr(), 0x10, 2]);
        assert_eq!(e.get(this, BSFile::iTrueFilePos), 0xf0);
        e.set(ni_file(this), NiFile::m_eMode, 1);
        e.call(0x00b0_0590, &args![this, 0x40u32, 0u32]);
        assert_eq!(calls_to(&e, NI_FILE_SEEK)[2], vec![this.addr(), 0x40, 0]);
    }

    #[test]
    fn get_size_measures_the_file_and_goes_back() {
        let (mut e, this) = engine();
        // No FILE: 0, after trying to open.
        assert_eq!(e.call(0x00b0_04e0, &args![this]).u32(), 0);
        assert_eq!(calls_to(&e, OPEN_DOUBLE).len(), 1);
        e.set(ni_file(this), NiFile::m_pFile, 0x7700);
        e.set(ni_file(this), NiFile::m_eMode, 1);
        let mut tells = 0;
        e.register_double(FTELL, move |_, _| {
            tells += 1;
            (if tells == 1 { 0x24u32 } else { 0x5000 }).into_ret()
        });
        e.register(FSEEK, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00b0_04e0, &args![this]).u32(), 0x5000);
        assert_eq!(calls_to(&e, NI_FILE_FLUSH), [vec![this.addr()]]);
        assert_eq!(
            calls_to(&e, FSEEK),
            [vec![0x7700, 0, 2], vec![0x7700, 0x24, 0]]
        );
    }
}

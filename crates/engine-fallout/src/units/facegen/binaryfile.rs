//! `FaceGen/binaryfile.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! FaceGen's binary files: `FutBinaryFileC` (a file name, an 8-character file id such as
//! `"FRCTL001"` and a C `FILE*`) and the game's `BSFaceGenBinaryFile` over it, which reads
//! through an `NiFile` (+0x40) opened by the file finder or shared from a `LoadedFile`
//! (+0x44). This build keeps the C++ library's debug checks: a failed check calls
//! `_invalid_parameter_noinfo` (`00ec7c56`) and the code goes on.
//!
//! The library's `std::string` is 0x1C bytes: its characters are inline at +4 while the
//! capacity (+0x18) is below 0x10, else at the pointer at +4; its length is at +0x14. A string
//! passed by value is seven argument words ([`StringArg`]); the callee destroys it.

#[allow(unused_imports)]
use crate::prelude::*;

/// The string's length (+0x14) and capacity (+0x18).
pub(crate) const STRING_LENGTH: u32 = 0x14;
pub(crate) const STRING_CAPACITY: u32 = 0x18;
/// The string tidy (`0044a730`, `thiscall`, two words): `(0, 0)` makes a string empty, `(1,
/// 0)` destroys one.
pub(crate) const STRING_TIDY: u32 = 0x0044_a730;
/// The string's assign from part of another (`0044a580`, `thiscall`: source, offset, count).
pub(crate) const STRING_ASSIGN: u32 = 0x0044_a580;
/// The string class's `npos` (`.rdata`).
pub(crate) const STRING_NPOS: u32 = 0x0101_73f0;
/// `_invalid_parameter_noinfo` (`00ec7c56`, no arguments): the library's failed check.
pub(crate) const INVALID_PARAMETER: u32 = 0x00ec_7c56;
/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, the block).
pub(crate) const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
pub(crate) const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;

/// `fclose` (`00ec9907`, `cdecl`) and `strlen` (`00ec6130`, `cdecl`).
const FCLOSE: u32 = 0x00ec_9907;
const STRLEN: u32 = 0x00ec_6130;
/// The empty C string `""`.
const EMPTY_STRING: u32 = 0x0101_1584;
/// The string's assign from characters (`0044a7f0`, `thiscall`: characters, count).
const STRING_ASSIGN_CHARS: u32 = 0x0044_a7f0;
/// `memcmp` (`00ec4835`, `cdecl`), `memmove_s` (`00ec7ce3`, `cdecl`: destination, size,
/// source, count) and the library's out-of-range throw (`00ec30e8`, no arguments; it does not
/// return in the game).
const MEMCMP: u32 = 0x00ec_4835;
const MEMMOVE_S: u32 = 0x00ec_7ce3;
const THROW_OUT_OF_RANGE: u32 = 0x00ec_30e8;
/// `FutBinaryFileC`'s constructor (`00ac3df0`, `thiscall`: the file id string by value).
const FUT_BINARY_FILE_CONSTRUCT: u32 = 0x00ac_3df0;
/// `BSFaceGenBinaryFile`'s vtable (RTTI `BSFaceGenBinaryFile`).
const BS_FACE_GEN_BINARY_FILE_VTABLE: u32 = 0x010a_2c38;
/// The loaded-file map (`011c3b3c`, may be null) and its lookup (`004483a0`, which the map
/// names `LoadedFile::IncRefCount`; `thiscall` on the map with the file name, answering the
/// `LoadedFile` or 0).
const LOADED_FILE_MAP: u32 = 0x011c_3b3c;
const LOADED_FILE_LOOKUP: u32 = 0x0044_83a0;
/// The `LoadedFile` reference functions: `0044b0f0` (`thiscall`) takes one, `0044b180`
/// (`thiscall`) gives one back.
const LOADED_FILE_ADD_REF: u32 = 0x0044_b0f0;
const LOADED_FILE_RELEASE: u32 = 0x0044_b180;
/// `LoadedFile` (Xbox PDB): `pFile` +0x08 and `bFileUsed` +0x0D.
const LOADED_FILE_FILE: u32 = 0x08;
const LOADED_FILE_USED: u32 = 0x0d;
/// `FileFinder::GetFile_ov2` (`00afdf20`, `cdecl`: name, 0, buffer size 0x2800, open mode).
const FILE_FINDER_GET_FILE: u32 = 0x00af_df20;
/// `futSetLastError` (`00ac6430`, `cdecl`: the error code and the file name string by value).
const SET_LAST_ERROR: u32 = 0x00ac_6430;
/// `fr2Assert` (`00aafb90`, `cdecl`: source file name, line) and the source file name
/// (`"D:\_Fallout3\Platforms\Common\Code\SDK\FaceGen\binaryFile.cpp"`).
const FR2_ASSERT: u32 = 0x00aa_fb90;
const BINARY_FILE_SOURCE: u32 = 0x010a_2d98;
/// The byte vector helpers (unit `run_00aba3c0` and `run_00ab9eb0`): `00aba0d0` constructs
/// one of a size (`thiscall`, the size), `00abc0a0` frees its elements (`thiscall`) and
/// `00abc120` frees its proxy (`thiscall`).
const BYTE_VECTOR_CONSTRUCT: u32 = 0x00ab_a0d0;
const BYTE_VECTOR_TIDY: u32 = 0x00ab_c0a0;
const BYTE_VECTOR_FREE_PROXY: u32 = 0x00ab_c120;
/// The vector base constructor (`00ad33c0`, `thiscall`, one word: the allocator by value)
/// and the vector's buy (`00ac03a0`, `thiscall`, the count; answers whether it allocated).
const VECTOR_BASE_CONSTRUCT: u32 = 0x00ad_33c0;
const VECTOR_BUY: u32 = 0x00ac_03a0;
/// `BSFaceGenBinaryFile::read` (vtable slot +0x10).
const SLOT_READ: u32 = 0x10;

/// The library vector (0x18 bytes on PC): its container proxy (+0x00), allocator (+0x08) and
/// first, last and end pointers (+0x0C, +0x10, +0x14).
pub(crate) const VECTOR_PROXY: u32 = 0x00;
pub(crate) const VECTOR_FIRST: u32 = 0x0c;
pub(crate) const VECTOR_LAST: u32 = 0x10;

layout! {
    /// `BSFaceGenBinaryFile` (Xbox PDB), 0x48 bytes: a `FutBinaryFileC` (0x40 bytes) with the
    /// game's file and loaded file.
    pub struct BSFaceGenBinaryFile: 0x48 {
        /// `FutBinaryFileC::m_fname` (Xbox PDB): a string.
        0x04 m_fname: u32,
        /// `FutBinaryFileC::fileId` (Xbox PDB): a string.
        0x20 fileId: u32,
        /// `fileId`'s length.
        0x34 fileIdLength: u32,
        /// `FutBinaryFileC::fptr` (Xbox PDB): the C `FILE*`.
        0x3C fptr: u32,
        /// `pFile` (Xbox PDB): `NiFile*`.
        0x40 pFile: u32,
        /// `pLoadedFile` (Xbox PDB): `LoadedFile*`.
        0x44 pLoadedFile: u32,
    }
}

/// `NiBinaryStream` (Xbox PDB) fields the reads use: `m_uiAbsoluteCurrentPos` +0x04 and the
/// read function `m_pfnRead` +0x08 (`cdecl`: the stream, buffer, byte count, the address of
/// the element size, the element count; answers the bytes read).
const STREAM_POSITION: u32 = 0x04;
const STREAM_READ_FUNCTION: u32 = 0x08;
/// The stream's virtual +0x44 (`thiscall`: buffer, byte count; answers the bytes written).
const STREAM_WRITE: u32 = 0x44;

/// A `std::string` passed by value: its seven words, in memory order.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StringArg(pub [u32; 7]);

impl StringArg {
    /// The seven words of the string at `address`.
    pub fn load(e: &Engine, address: u32) -> Self {
        let mut words = [0u32; 7];
        for (i, word) in words.iter_mut().enumerate() {
            *word = e.mem.u32(address.wrapping_add(4 * i as u32));
        }
        StringArg(words)
    }

    /// Writes the seven words to `address`.
    pub fn store(self, e: &mut Engine, address: u32) {
        for (i, word) in self.0.iter().enumerate() {
            e.mem.set_u32(address.wrapping_add(4 * i as u32), *word);
        }
    }
}

impl Arg for StringArg {
    const WORDS: usize = 7;
    fn take(words: &[u32], i: &mut usize) -> Self {
        let mut out = [0u32; 7];
        for word in out.iter_mut() {
            *word = u32::take(words, i);
        }
        StringArg(out)
    }
    fn put(self, out: &mut Vec<u32>) {
        out.extend_from_slice(&self.0);
    }
}

/// The address of a string's characters.
pub(crate) fn string_chars(e: &Engine, string: u32) -> u32 {
    if e.mem.u32(string.wrapping_add(STRING_CAPACITY)) < 0x10 {
        string.wrapping_add(4)
    } else {
        e.mem.u32(string.wrapping_add(4))
    }
}

/// A by-value copy of the string at `source`, built as the callers build one in the callee's
/// argument area: made empty (`0044a730(0, 0)`), then assigned the whole source
/// (`0044a580(source, 0, npos)`). The callee destroys the copy.
pub(crate) fn string_copy_arg(e: &mut Engine, source: u32) -> StringArg {
    e.with_stack(0x1c, |e, block| {
        let block = block.addr();
        e.call(STRING_TIDY, &args![block, 0u32, 0u32]);
        let npos: u32 = e.global(STRING_NPOS);
        e.call(STRING_ASSIGN, &args![block, source, 0u32, npos]);
        StringArg::load(e, block)
    })
}

/// Runs `f` with a by-value string argument in memory (its argument slot) at the address `f`
/// receives.
pub(crate) fn with_string_arg<R>(
    e: &mut Engine,
    string: StringArg,
    f: impl FnOnce(&mut Engine, u32) -> R,
) -> R {
    e.with_stack(0x1c, |e, block| {
        string.store(e, block.addr());
        f(e, block.addr())
    })
}

/// `futSetLastError(code, <copy of the string at name>)`.
fn set_last_error(e: &mut Engine, code: u32, name: u32) {
    let copy = string_copy_arg(e, name);
    e.call(SET_LAST_ERROR, &args![code, copy]);
}

/// A failed debug check when `ok` is false.
fn check(e: &mut Engine, ok: bool) {
    if !ok {
        e.call(INVALID_PARAMETER, &[]);
    }
}

/// The vector's element count (last - first).
fn vector_size(e: &Engine, vector: u32) -> u32 {
    e.mem
        .u32(vector + VECTOR_LAST)
        .wrapping_sub(e.mem.u32(vector + VECTOR_FIRST))
}

// Translated from 00ac5430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FutBinaryFileC::close` (Xbox PDB): closes the C file when open (`fclose`), empties the
/// file name (`0044a7f0` with `""` and its `strlen`) and clears the file pointer.
pub fn fut_binary_file_c_close(e: &mut Engine, this: Ptr<BSFaceGenBinaryFile>) {
    let file = e.get(this, BSFaceGenBinaryFile::fptr);
    if file != 0 {
        e.call(FCLOSE, &args![file]);
    }
    let length = e.call(STRLEN, &args![EMPTY_STRING]).u32();
    e.call(
        STRING_ASSIGN_CHARS,
        &args![this.addr() + 4, EMPTY_STRING, length],
    );
    e.set(this, BSFaceGenBinaryFile::fptr, 0);
}

// Translated from 00ac5490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac5490 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `BSFaceGenBinaryFile::BSFaceGenBinaryFile` (Xbox PDB; `thiscall`, the file id string by
/// value): constructs the `FutBinaryFileC` (`00ac3df0`) with a copy of the id, sets the
/// class's vtable, clears the file and the loaded file, and destroys the id. Returns `this`.
pub fn bs_face_gen_binary_file_constructor(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    file_id: StringArg,
) -> Ptr<BSFaceGenBinaryFile> {
    with_string_arg(e, file_id, |e, id| {
        let copy = string_copy_arg(e, id);
        e.call(FUT_BINARY_FILE_CONSTRUCT, &args![this, copy]);
        e.mem.set_u32(this.addr(), BS_FACE_GEN_BINARY_FILE_VTABLE);
        e.set(this, BSFaceGenBinaryFile::pFile, 0);
        e.set(this, BSFaceGenBinaryFile::pLoadedFile, 0);
        e.call(STRING_TIDY, &args![id, 1u32, 0u32]);
    });
    this
}

// Translated from 00ac5540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac5540 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `BSFaceGenBinaryFile::openForRead` (Xbox PDB; `thiscall`: the file name string by value and
/// a byte). The open mode is 1, or 0x100 when the byte is set. With a loaded-file map
/// (`011c3b3c`) the name is looked up there (`004483a0`); a loaded file found takes a reference
/// (`0044b0f0`), becomes `pLoadedFile` with a second reference, is marked used (+0x0D) and its
/// `BSFile` (+0x08) becomes `pFile`; otherwise the file finder opens the file
/// (`00afdf20(name, 0, 0x2800, mode)`). Without a file: `futSetLastError(4, name)`, false.
///
/// Then the header (the id's length plus one bytes, in a byte vector, `00aba0d0`) is read
/// (`readHeader`); a failed read answers false. A copy of the header (`00ac5f00`) gets
/// character 5 of the id, and its C string must equal the id (`00ac6100`), else
/// `futSetLastError(2, name)`, `00ac5c00`, false. When the header's character 5 (the version)
/// is above `'0'` it must equal the id's, else `futSetLastError(4, name)`, `00ac5c00`, false.
/// Otherwise the id takes the header's version, the name is stored as `m_fname`, and the
/// answer is whether there is a file. Every path frees the vectors, gives back the loaded
/// file's first reference and destroys the name.
pub fn bs_face_gen_binary_file_open_for_read(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    name: StringArg,
    shared: u8,
) -> bool {
    with_string_arg(e, name, |e, name| {
        let mode: u32 = if shared != 0 { 0x100 } else { 1 };
        let map = e.global::<u32>(LOADED_FILE_MAP);
        let loaded = if map != 0 {
            let chars = string_chars(e, name);
            e.call(LOADED_FILE_LOOKUP, &args![map, chars]).u32()
        } else {
            0
        };
        if loaded != 0 {
            e.call(LOADED_FILE_ADD_REF, &args![loaded]);
        }
        if loaded != 0 {
            e.set(this, BSFaceGenBinaryFile::pLoadedFile, loaded);
            let shared_file = e.get(this, BSFaceGenBinaryFile::pLoadedFile);
            e.call(LOADED_FILE_ADD_REF, &args![shared_file]);
            let shared_file = e.get(this, BSFaceGenBinaryFile::pLoadedFile);
            e.mem.set_u8(shared_file.wrapping_add(LOADED_FILE_USED), 1);
            let file = e.mem.u32(shared_file.wrapping_add(LOADED_FILE_FILE));
            e.set(this, BSFaceGenBinaryFile::pFile, file);
        } else {
            let chars = string_chars(e, name);
            let file = e
                .call(FILE_FINDER_GET_FILE, &args![chars, 0u32, 0x2800u32, mode])
                .u32();
            e.set(this, BSFaceGenBinaryFile::pFile, file);
        }
        let release_and_destroy = |e: &mut Engine| {
            if loaded != 0 {
                e.call(LOADED_FILE_RELEASE, &args![loaded]);
            }
            e.call(STRING_TIDY, &args![name, 1u32, 0u32]);
        };
        if e.get(this, BSFaceGenBinaryFile::pFile) == 0 {
            set_last_error(e, 4, name);
            release_and_destroy(e);
            return false;
        }
        e.with_stack(0x30, |e, vectors| {
            let header = vectors.addr();
            let copy = vectors.addr() + 0x18;
            let length = e.get(this, BSFaceGenBinaryFile::fileIdLength);
            e.call(
                BYTE_VECTOR_CONSTRUCT,
                &args![header, length.wrapping_add(1)],
            );
            let length = e.get(this, BSFaceGenBinaryFile::fileIdLength);
            check(e, vector_size(e, header) != 0);
            let first = e.mem.u32(header + VECTOR_FIRST);
            let read_ok = bs_face_gen_binary_file_read_header(e, this, first, length, 1);
            if !read_ok {
                e.call(BYTE_VECTOR_TIDY, &args![header]);
                e.call(BYTE_VECTOR_FREE_PROXY, &args![header]);
                release_and_destroy(e);
                return false;
            }
            fn_00ac5f00(e, Ptr::new(copy), Ptr::new(header));
            let id = this.addr() + 0x20;
            check(e, e.mem.u32(id + STRING_LENGTH) >= 5);
            let id_chars = string_chars(e, id);
            check(e, vector_size(e, copy) > 5);
            let copy_first = e.mem.u32(copy + VECTOR_FIRST);
            let version = e.mem.u8(id_chars.wrapping_add(5));
            e.mem.set_u8(copy_first.wrapping_add(5), version);
            check(e, vector_size(e, copy) != 0);
            let copy_first = e.mem.u32(copy + VECTOR_FIRST);
            let copy_length = e.call(STRLEN, &args![copy_first]).u32();
            let id_length = e.mem.u32(id + STRING_LENGTH);
            let compare = fn_00ac6100(e, Ptr::new(id), 0, id_length, copy_first, copy_length);
            let destroy_vectors = |e: &mut Engine| {
                e.call(BYTE_VECTOR_TIDY, &args![copy]);
                e.call(BYTE_VECTOR_FREE_PROXY, &args![copy]);
                e.call(BYTE_VECTOR_TIDY, &args![header]);
                e.call(BYTE_VECTOR_FREE_PROXY, &args![header]);
            };
            if compare != 0 {
                set_last_error(e, 2, name);
                fn_00ac5c00(e, this);
                destroy_vectors(e);
                release_and_destroy(e);
                return false;
            }
            check(e, vector_size(e, header) > 5);
            let header_first = e.mem.u32(header + VECTOR_FIRST);
            if e.mem.i8(header_first.wrapping_add(5)) as i32 > 0x30 {
                check(e, e.mem.u32(id + STRING_LENGTH) >= 5);
                let id_chars = string_chars(e, id);
                check(e, vector_size(e, header) > 5);
                let header_first = e.mem.u32(header + VECTOR_FIRST);
                if e.mem.i8(id_chars.wrapping_add(5)) != e.mem.i8(header_first.wrapping_add(5)) {
                    set_last_error(e, 4, name);
                    fn_00ac5c00(e, this);
                    destroy_vectors(e);
                    release_and_destroy(e);
                    return false;
                }
            }
            check(e, vector_size(e, header) > 5);
            let header_first = e.mem.u32(header + VECTOR_FIRST);
            check(e, e.mem.u32(id + STRING_LENGTH) >= 5);
            let id_chars = string_chars(e, id);
            let version = e.mem.u8(header_first.wrapping_add(5));
            e.mem.set_u8(id_chars.wrapping_add(5), version);
            let npos: u32 = e.global(STRING_NPOS);
            e.call(STRING_ASSIGN, &args![this.addr() + 4, name, 0u32, npos]);
            let opened = e.get(this, BSFaceGenBinaryFile::pFile) != 0;
            e.call(BYTE_VECTOR_TIDY, &args![copy]);
            let proxy = e.mem.u32(copy + VECTOR_PROXY);
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, proxy]);
            e.call(BYTE_VECTOR_TIDY, &args![header]);
            let proxy = e.mem.u32(header + VECTOR_PROXY);
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, proxy]);
            release_and_destroy(e);
            opened
        })
    })
}

// Translated from 00ac5c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: closes the game's file. With a loaded file, `pFile` is cleared,
/// the loaded file's reference given back (`0044b180`) and `pLoadedFile` cleared; otherwise
/// the file (when set) is deleted through its scalar deleting destructor (vtable slot 0, flag
/// 1) and `pFile` cleared.
pub fn fn_00ac5c00(e: &mut Engine, this: Ptr<BSFaceGenBinaryFile>) {
    if e.get(this, BSFaceGenBinaryFile::pLoadedFile) != 0 {
        e.set(this, BSFaceGenBinaryFile::pFile, 0);
        let loaded = e.get(this, BSFaceGenBinaryFile::pLoadedFile);
        e.call(LOADED_FILE_RELEASE, &args![loaded]);
        e.set(this, BSFaceGenBinaryFile::pLoadedFile, 0);
        return;
    }
    let file = e.get(this, BSFaceGenBinaryFile::pFile);
    if file != 0 {
        e.vcall(file, 0, &args![1u32]);
    }
    e.set(this, BSFaceGenBinaryFile::pFile, 0);
}

// Translated from 00ac5c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`: buffer, count): writes `count * 2` bytes from
/// `buffer` through the file's virtual +0x44, adds the bytes written to the stream's position
/// (+4) and answers whether all were written; false without a file.
pub fn fn_00ac5c70(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    buffer: u32,
    count: u32,
) -> bool {
    if e.get(this, BSFaceGenBinaryFile::pFile) == 0 {
        return false;
    }
    let bytes = count.wrapping_shl(1);
    let file = e.get(this, BSFaceGenBinaryFile::pFile);
    let written = e.vcall(file, STREAM_WRITE, &args![buffer, bytes]).u32();
    let file = e.get(this, BSFaceGenBinaryFile::pFile);
    let position = e.mem.u32(file + STREAM_POSITION);
    e.mem
        .set_u32(file + STREAM_POSITION, position.wrapping_add(written));
    bytes == written
}

/// One read through the stream's read function: `(stream, buffer, bytes, &size, 1)`, the
/// position advanced by the bytes read, which are returned.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, bytes: u32, size_slot: u32) -> u32 {
    let read_function = e.mem.u32(stream + STREAM_READ_FUNCTION);
    let read = e
        .call(
            read_function,
            &args![stream, buffer, bytes, size_slot, 1u32],
        )
        .u32();
    let position = e.mem.u32(stream + STREAM_POSITION);
    e.mem
        .set_u32(stream + STREAM_POSITION, position.wrapping_add(read));
    read
}

// Translated from 00ac5cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenBinaryFile::read` (Xbox PDB; `thiscall`: buffer, element size, count): reads
/// `size * count` bytes through the stream's read function (`NiBinaryStream::m_pfnRead`,
/// +0x08), which gets the address of the element size (the argument's slot) for its byte
/// swapping. Sizes 1, 2 and 4 are passed as they are; any other size becomes 4 when it is a
/// multiple of 4 (0 included), else 1 after `fr2Assert(binaryFile.cpp, 0x1fc)`. Answers
/// whether every byte was read; false without a file.
pub fn bs_face_gen_binary_file_read(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    buffer: u32,
    size: u32,
    count: u32,
) -> bool {
    if e.get(this, BSFaceGenBinaryFile::pFile) == 0 {
        return false;
    }
    let bytes = size.wrapping_mul(count);
    let read = e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, size);
        if size != 0 && (size <= 2 || size == 4) {
            let stream = e.get(this, BSFaceGenBinaryFile::pFile);
            return stream_read(e, stream, buffer, bytes, slot);
        }
        if size % 4 == 0 {
            e.mem.set_u32(slot, 4);
        } else {
            e.call(FR2_ASSERT, &args![BINARY_FILE_SOURCE, 0x1fcu32]);
            e.mem.set_u32(slot, 1);
        }
        let stream = e.get(this, BSFaceGenBinaryFile::pFile);
        stream_read(e, stream, buffer, bytes, slot)
    });
    bytes == read
}

// Translated from 00ac5e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenBinaryFile::readHeader` (Xbox PDB; `thiscall`: buffer, size, count): reads
/// `size * count` bytes through the virtual `read` (slot +0x10) with an element size of 1. A
/// failed read sets `futSetLastError(1, m_fname)`, closes the file (`00ac5c00`) and answers
/// false; false without a file.
pub fn bs_face_gen_binary_file_read_header(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    buffer: u32,
    size: u32,
    count: u32,
) -> bool {
    if e.get(this, BSFaceGenBinaryFile::pFile) == 0 {
        return false;
    }
    let bytes = size.wrapping_mul(count);
    if e.vcall(this.addr(), SLOT_READ, &args![buffer, 1u32, bytes])
        .u8()
        != 0
    {
        return true;
    }
    set_last_error(e, 1, this.addr() + 4);
    fn_00ac5c00(e, this);
    false
}

// Translated from 00ac5f00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac5f00 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `std::vector<char>`'s copy constructor (the map's name; `thiscall`, the source vector).
/// Builds the vector base (`00ad33c0`, given the word the code pushes as the allocator: `this`)
/// and buys room for the source's element count (`00ac03a0`). When that succeeds, the
/// source's end and begin iterators are checked (each must lie within the source; a failed
/// check calls `_invalid_parameter_noinfo`) and the elements copied to the new first pointer
/// (`00ac61c0`, given the iterators' proxy and pointer words, the destination and the address
/// of the allocator at +8); the result is the new last pointer. Returns `this`.
pub fn fn_00ac5f00(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let base = this.addr();
    let source = source.addr();
    e.call(VECTOR_BASE_CONSTRUCT, &args![base, base]);
    let count = vector_size(e, source);
    if e.call(VECTOR_BUY, &args![base, count]).u8() == 0 {
        return this;
    }
    let end = e.mem.u32(source + VECTOR_LAST);
    let ok = source != 0
        && e.mem.u32(source + VECTOR_FIRST) <= end
        && end <= e.mem.u32(source + VECTOR_LAST);
    check(e, ok);
    let end_proxy = e.mem.u32(source + VECTOR_PROXY);
    let begin = e.mem.u32(source + VECTOR_FIRST);
    let ok = source != 0
        && e.mem.u32(source + VECTOR_FIRST) <= begin
        && begin <= e.mem.u32(source + VECTOR_LAST);
    check(e, ok);
    let begin_proxy = e.mem.u32(source + VECTOR_PROXY);
    let destination = e.mem.u32(base + VECTOR_FIRST);
    let last = fn_00ac61c0(e, begin_proxy, begin, end_proxy, end, destination, base + 8);
    e.mem.set_u32(base + VECTOR_LAST, last);
    this
}

// Translated from 00ac6100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::compare(offset, count, characters, length)` (the map's name; `thiscall`,
/// four words): an offset past the string's length calls the library's out-of-range throw
/// (`00ec30e8`, which does not return in the game; the code after it goes on). The count is
/// clipped to the characters left; `memcmp` compares the shorter of the two lengths, and on
/// equal bytes the result is -1, 0 or 1 by the lengths.
pub fn fn_00ac6100(
    e: &mut Engine,
    this: Ptr,
    offset: u32,
    count: u32,
    characters: u32,
    length: u32,
) -> i32 {
    let string = this.addr();
    if e.mem.u32(string + STRING_LENGTH) < offset {
        e.call(THROW_OUT_OF_RANGE, &[]);
    }
    let mut count = count;
    if e.mem.u32(string + STRING_LENGTH).wrapping_sub(offset) < count {
        count = e.mem.u32(string + STRING_LENGTH).wrapping_sub(offset);
    }
    let compared = if count < length { count } else { length };
    let chars = string_chars(e, string);
    let result = e
        .call(
            MEMCMP,
            &args![chars.wrapping_add(offset), characters, compared],
        )
        .i32();
    if result != 0 {
        return result;
    }
    if count < length {
        -1
    } else {
        i32::from(count != length)
    }
}

// Translated from 00ac61c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words: the first iterator's proxy and pointer, the
/// last iterator's proxy and pointer, the destination and the allocator's address; only the
/// two pointers and the destination are read): copies the bytes from first to last to the
/// destination with `memmove_s` (when there are any) and returns the destination's end.
pub fn fn_00ac61c0(
    e: &mut Engine,
    _unused_1: u32,
    first: u32,
    _unused_3: u32,
    last: u32,
    destination: u32,
    _unused_6: u32,
) -> u32 {
    let count = last.wrapping_sub(first);
    let end = destination.wrapping_add(count);
    if count != 0 {
        e.call(MEMMOVE_S, &args![destination, count, first, count]);
    }
    end
}

/// `fopen` (`00ec9a47`, `cdecl`: name, mode), `fwrite` (`00ecb086`), `fread` (`00ecb3a8`; both
/// `cdecl`: buffer, size, count, file) and `fseek` (`00ecb65a`, `cdecl`: file, offset, origin).
const FOPEN: u32 = 0x00ec_9a47;
const FWRITE: u32 = 0x00ec_b086;
const FREAD: u32 = 0x00ec_b3a8;
const FSEEK: u32 = 0x00ec_b65a;
/// The open modes `"wb"` (`0106e798`), `"rb"` (`0106e79c`), `"r+b"` (`010a2dd8`) and `"a+b"`
/// (`010a2ddc`).
const MODE_WRITE: u32 = 0x0106_e798;
const MODE_READ: u32 = 0x0106_e79c;
const MODE_UPDATE: u32 = 0x010a_2dd8;
const MODE_APPEND: u32 = 0x010a_2ddc;
/// The game file's seek (`NiBinaryStream` vtable slot +0x14: offset, origin) and the origin
/// word `00ac5e90` passes (`010a2480`).
const STREAM_SEEK: u32 = 0x14;
const SEEK_ORIGIN: u32 = 0x010a_2480;
/// `FutBinaryFileC::fileId` (+0x20).
const FILE_ID: u32 = 0x20;

/// The header check the open functions share: character 5 of the file id (asserting its length
/// is at least 5) is copied into `vector`'s bytes (asserting more than 5), and the id is
/// compared (`00ac6100` from offset 0, the whole id) with the vector's C string (`strlen`).
/// Answers the compare's result (0 when equal).
fn compare_with_id(e: &mut Engine, this: u32, vector: u32) -> i32 {
    let id = this + FILE_ID;
    check(e, e.mem.u32(id + STRING_LENGTH) >= 5);
    let id_chars = string_chars(e, id);
    check(e, vector_size(e, vector) > 5);
    let first = e.mem.u32(vector + VECTOR_FIRST);
    let version = e.mem.u8(id_chars.wrapping_add(5));
    e.mem.set_u8(first.wrapping_add(5), version);
    check(e, vector_size(e, vector) != 0);
    let first = e.mem.u32(vector + VECTOR_FIRST);
    let length = e.call(STRLEN, &args![first]).u32();
    let id_length = e.mem.u32(id + STRING_LENGTH);
    fn_00ac6100(e, Ptr::new(id), 0, id_length, first, length)
}

/// Makes the header vector (the id's length plus one bytes, `00aba0d0`) and reads the header
/// into it (`readHeader(first, length, 1)`, its size asserted not 0 first). Answers whether
/// the read succeeded.
fn read_header_into(e: &mut Engine, this: Ptr<BSFaceGenBinaryFile>, header: u32) -> bool {
    let length = e.get(this, BSFaceGenBinaryFile::fileIdLength);
    e.call(
        BYTE_VECTOR_CONSTRUCT,
        &args![header, length.wrapping_add(1)],
    );
    let length = e.get(this, BSFaceGenBinaryFile::fileIdLength);
    check(e, vector_size(e, header) != 0);
    let first = e.mem.u32(header + VECTOR_FIRST);
    fut_binary_file_c_read_header(e, this, first, length, 1)
}

/// Frees a byte vector: its elements (`00abc0a0`), then its proxy through the memory manager.
fn free_byte_vector(e: &mut Engine, vector: u32) {
    e.call(BYTE_VECTOR_TIDY, &args![vector]);
    let proxy = e.mem.u32(vector + VECTOR_PROXY);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, proxy]);
}

/// Frees a byte vector: its elements (`00abc0a0`), then its proxy (`00abc120`).
fn tidy_byte_vector(e: &mut Engine, vector: u32) {
    e.call(BYTE_VECTOR_TIDY, &args![vector]);
    e.call(BYTE_VECTOR_FREE_PROXY, &args![vector]);
}

// Translated from 00ac43b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac43b0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `FutBinaryFileC::openForRead` (Xbox PDB; `thiscall`, the file name string by value). Closes
/// the file, opens the name with `fopen(name, "rb")`; without a file `futSetLastError(4,
/// name)`, false. The header is read into a byte vector of the id's length plus one
/// (`readHeader`; a failed read answers false). A copy of it (`00ac5f00`) gets character 5 of
/// the id and must equal the id as a C string, else `futSetLastError(2, name)`, `close`,
/// false. When the header's character 5 (the version) is above `'0'` it must equal the id's,
/// else `futSetLastError(4, name)`, `close`, false. Otherwise the id takes the header's version,
/// the name becomes `m_fname` and the answer is true. Every path frees the vectors it made and
/// destroys the name.
pub fn fut_binary_file_c_open_for_read(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    name: StringArg,
) -> bool {
    with_string_arg(e, name, |e, name| {
        fut_binary_file_c_close(e, this);
        let chars = string_chars(e, name);
        let file = e.call(FOPEN, &args![chars, MODE_READ]).u32();
        e.set(this, BSFaceGenBinaryFile::fptr, file);
        if e.get(this, BSFaceGenBinaryFile::fptr) == 0 {
            set_last_error(e, 4, name);
            e.call(STRING_TIDY, &args![name, 1u32, 0u32]);
            return false;
        }
        e.with_stack(0x30, |e, vectors| {
            let header = vectors.addr();
            let copy = vectors.addr() + 0x18;
            if !read_header_into(e, this, header) {
                tidy_byte_vector(e, header);
                e.call(STRING_TIDY, &args![name, 1u32, 0u32]);
                return false;
            }
            fn_00ac5f00(e, Ptr::new(copy), Ptr::new(header));
            let fail = |e: &mut Engine, code: u32| {
                set_last_error(e, code, name);
                fut_binary_file_c_close(e, this);
                tidy_byte_vector(e, copy);
                tidy_byte_vector(e, header);
                e.call(STRING_TIDY, &args![name, 1u32, 0u32]);
                false
            };
            if compare_with_id(e, this.addr(), copy) != 0 {
                return fail(e, 2);
            }
            let id = this.addr() + FILE_ID;
            check(e, vector_size(e, header) > 5);
            let header_first = e.mem.u32(header + VECTOR_FIRST);
            if e.mem.i8(header_first.wrapping_add(5)) as i32 > 0x30 {
                check(e, e.mem.u32(id + STRING_LENGTH) >= 5);
                let id_chars = string_chars(e, id);
                check(e, vector_size(e, header) > 5);
                let header_first = e.mem.u32(header + VECTOR_FIRST);
                if e.mem.i8(id_chars.wrapping_add(5)) != e.mem.i8(header_first.wrapping_add(5)) {
                    return fail(e, 4);
                }
            }
            check(e, vector_size(e, header) > 5);
            let header_first = e.mem.u32(header + VECTOR_FIRST);
            check(e, e.mem.u32(id + STRING_LENGTH) >= 5);
            let id_chars = string_chars(e, id);
            let version = e.mem.u8(header_first.wrapping_add(5));
            e.mem.set_u8(id_chars.wrapping_add(5), version);
            let npos: u32 = e.global(STRING_NPOS);
            e.call(STRING_ASSIGN, &args![this.addr() + 4, name, 0u32, npos]);
            tidy_byte_vector(e, copy);
            free_byte_vector(e, header);
            e.call(STRING_TIDY, &args![name, 1u32, 0u32]);
            true
        })
    })
}

// Translated from 00ac4910 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac4910 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `FutBinaryFileC::openForOverwrite` (Xbox PDB; `thiscall`, the file name string by value).
/// Closes the file and opens the name with `fopen(name, "wb")`; without a file
/// `futSetLastError(5, name)`, false. Then the id's characters are written as the header
/// (`writeHeader(id, length, 1)`); a failed write sets `futSetLastError(5, name)`, closes and
/// answers false. Otherwise the name becomes `m_fname` and the answer is true. The name is
/// destroyed on every path.
pub fn fut_binary_file_c_open_for_overwrite(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    name: StringArg,
) -> bool {
    with_string_arg(e, name, |e, name| {
        fut_binary_file_c_close(e, this);
        let chars = string_chars(e, name);
        let file = e.call(FOPEN, &args![chars, MODE_WRITE]).u32();
        e.set(this, BSFaceGenBinaryFile::fptr, file);
        if e.get(this, BSFaceGenBinaryFile::fptr) == 0 {
            set_last_error(e, 5, name);
            e.call(STRING_TIDY, &args![name, 1u32, 0u32]);
            return false;
        }
        let length = e.get(this, BSFaceGenBinaryFile::fileIdLength);
        let id_chars = string_chars(e, this.addr() + FILE_ID);
        if !fut_binary_file_c_write_header(e, this, id_chars, length, 1) {
            set_last_error(e, 5, name);
            fut_binary_file_c_close(e, this);
            e.call(STRING_TIDY, &args![name, 1u32, 0u32]);
            return false;
        }
        let npos: u32 = e.global(STRING_NPOS);
        e.call(STRING_ASSIGN, &args![this.addr() + 4, name, 0u32, npos]);
        e.call(STRING_TIDY, &args![name, 1u32, 0u32]);
        true
    })
}

// Translated from 00ac4b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac4b00 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: `FutBinaryFileC`'s open for update (`thiscall`, the address of the
/// file name string). Closes the file and opens the name with `fopen(name, "r+b")`; without a
/// file `futSetLastError(4, name)`, false. The header is read into a byte vector (a failed read
/// answers false); a copy of it (`00ac5f00`) gets character 5 of the id and must equal the id as
/// a C string, else `futSetLastError(2, name)`, `close`, false. Otherwise the id's character 5
/// becomes `'0'`, the name becomes `m_fname` and the answer is true.
pub fn fn_00ac4b00(e: &mut Engine, this: Ptr<BSFaceGenBinaryFile>, name: u32) -> bool {
    fut_binary_file_c_close(e, this);
    let chars = string_chars(e, name);
    let file = e.call(FOPEN, &args![chars, MODE_UPDATE]).u32();
    e.set(this, BSFaceGenBinaryFile::fptr, file);
    if e.get(this, BSFaceGenBinaryFile::fptr) == 0 {
        set_last_error(e, 4, name);
        return false;
    }
    e.with_stack(0x30, |e, vectors| {
        let header = vectors.addr();
        let copy = vectors.addr() + 0x18;
        if !read_header_into(e, this, header) {
            free_byte_vector(e, header);
            return false;
        }
        fn_00ac5f00(e, Ptr::new(copy), Ptr::new(header));
        if compare_with_id(e, this.addr(), copy) != 0 {
            set_last_error(e, 2, name);
            fut_binary_file_c_close(e, this);
            free_byte_vector(e, copy);
            free_byte_vector(e, header);
            return false;
        }
        let id = this.addr() + FILE_ID;
        check(e, e.mem.u32(id + STRING_LENGTH) >= 5);
        let id_chars = string_chars(e, id);
        e.mem.set_u8(id_chars.wrapping_add(5), b'0');
        let npos: u32 = e.global(STRING_NPOS);
        e.call(STRING_ASSIGN, &args![this.addr() + 4, name, 0u32, npos]);
        tidy_byte_vector(e, copy);
        free_byte_vector(e, header);
        true
    })
}

// Translated from 00ac4f00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac4f00 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: `FutBinaryFileC`'s open for appending (`thiscall`, the address of
/// the file name string). Closes the file and opens the name with `fopen(name, "a+b")`; without
/// a file `futSetLastError(4, name)`, false. The header is read into a byte vector (a failed
/// read answers false), which gets character 5 of the id and must equal the id as a C string,
/// else `futSetLastError(2, name)`, `close`, false. Otherwise the name becomes `m_fname` and
/// the answer is true.
pub fn fn_00ac4f00(e: &mut Engine, this: Ptr<BSFaceGenBinaryFile>, name: u32) -> bool {
    fut_binary_file_c_close(e, this);
    let chars = string_chars(e, name);
    let file = e.call(FOPEN, &args![chars, MODE_APPEND]).u32();
    e.set(this, BSFaceGenBinaryFile::fptr, file);
    if e.get(this, BSFaceGenBinaryFile::fptr) == 0 {
        set_last_error(e, 4, name);
        return false;
    }
    e.with_stack(0x18, |e, header| {
        let header = header.addr();
        if !read_header_into(e, this, header) {
            free_byte_vector(e, header);
            return false;
        }
        if compare_with_id(e, this.addr(), header) != 0 {
            set_last_error(e, 2, name);
            fut_binary_file_c_close(e, this);
            tidy_byte_vector(e, header);
            return false;
        }
        let npos: u32 = e.global(STRING_NPOS);
        e.call(STRING_ASSIGN, &args![this.addr() + 4, name, 0u32, npos]);
        free_byte_vector(e, header);
        true
    })
}

/// `FutBinaryFileC::writeHeader` and `readHeader` share this shape: without a file false; a
/// transfer of fewer than `count` elements sets `futSetLastError(code, m_fname)`, closes the
/// file and answers false.
fn transfer(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    function: u32,
    code: u32,
    buffer: u32,
    size: u32,
    count: u32,
) -> bool {
    let file = e.get(this, BSFaceGenBinaryFile::fptr);
    if file == 0 {
        return false;
    }
    let done = e.call(function, &args![buffer, size, count, file]).u32();
    if done == count {
        return true;
    }
    set_last_error(e, code, this.addr() + 4);
    fut_binary_file_c_close(e, this);
    false
}

// Translated from 00ac5220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FutBinaryFileC::writeHeader` (Xbox PDB; `thiscall`: buffer, size, count): `fwrite` to the
/// C file; fewer than `count` written sets `futSetLastError(6, m_fname)` and closes. False
/// without a file.
pub fn fut_binary_file_c_write_header(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    buffer: u32,
    size: u32,
    count: u32,
) -> bool {
    transfer(e, this, FWRITE, 6, buffer, size, count)
}

// Translated from 00ac52b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FutBinaryFileC::readHeader` (Xbox PDB; `thiscall`: buffer, size, count): `fread` from the C
/// file; fewer than `count` read sets `futSetLastError(1, m_fname)` and closes. False without a
/// file.
pub fn fut_binary_file_c_read_header(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    buffer: u32,
    size: u32,
    count: u32,
) -> bool {
    transfer(e, this, FREAD, 1, buffer, size, count)
}

// Translated from 00ac5340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FutBinaryFileC::write` (Xbox PDB; `thiscall`: buffer, size, count): `writeHeader`.
pub fn fut_binary_file_c_write(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    buffer: u32,
    size: u32,
    count: u32,
) -> bool {
    fut_binary_file_c_write_header(e, this, buffer, size, count)
}

// Translated from 00ac5370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FutBinaryFileC::read` (Xbox PDB; `thiscall`: buffer, size, count): `readHeader`.
pub fn fut_binary_file_c_read(
    e: &mut Engine,
    this: Ptr<BSFaceGenBinaryFile>,
    buffer: u32,
    size: u32,
    count: u32,
) -> bool {
    fut_binary_file_c_read_header(e, this, buffer, size, count)
}

// Translated from 00ac53a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FutBinaryFileC::seek` (Xbox PDB; `thiscall`, the offset): `fseek(file, offset, 0)`; a
/// failure sets `futSetLastError(7, m_fname)`, closes and answers false. False without a file.
pub fn fut_binary_file_c_seek(e: &mut Engine, this: Ptr<BSFaceGenBinaryFile>, offset: u32) -> bool {
    let file = e.get(this, BSFaceGenBinaryFile::fptr);
    if file == 0 {
        return false;
    }
    if e.call(FSEEK, &args![file, offset, 0u32]).u32() == 0 {
        return true;
    }
    set_last_error(e, 7, this.addr() + 4);
    fut_binary_file_c_close(e, this);
    false
}

// Translated from 00ac5e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSFaceGenBinaryFile`'s seek (`thiscall`, the offset): without a
/// game file false; otherwise the file's seek (its vtable slot +0x14) gets the offset and the
/// origin word at `010a2480`, and the answer is true.
pub fn fn_00ac5e90(e: &mut Engine, this: Ptr<BSFaceGenBinaryFile>, offset: u32) -> bool {
    if e.get(this, BSFaceGenBinaryFile::pFile) == 0 {
        return false;
    }
    let origin: u32 = e.global(SEEK_ORIGIN);
    let file = e.get(this, BSFaceGenBinaryFile::pFile);
    e.vcall(file, STREAM_SEEK, &args![offset, origin]);
    true
}

// Translated from 00ac5ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the library signature matcher names it after a destructor with
/// the same code): a `FILE*` holder's destructor (`thiscall`): `fclose` of the word at +0 when
/// it is set.
pub fn fn_00ac5ed0(e: &mut Engine, this: Ptr) {
    let file = e.mem.u32(this.addr());
    if file != 0 {
        e.call(FCLOSE, &args![file]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ac_5430,
            fut_binary_file_c_close(Ptr<BSFaceGenBinaryFile>)
        ),
        entry!(
            0x00ac_5490,
            bs_face_gen_binary_file_constructor(
                Ptr<BSFaceGenBinaryFile>,
                StringArg,
            ) -> Ptr<BSFaceGenBinaryFile>
        ),
        entry!(
            0x00ac_5540,
            bs_face_gen_binary_file_open_for_read(Ptr<BSFaceGenBinaryFile>, StringArg, u8) -> bool
        ),
        entry!(0x00ac_5c00, fn_00ac5c00(Ptr<BSFaceGenBinaryFile>)),
        entry!(
            0x00ac_5c70,
            fn_00ac5c70(Ptr<BSFaceGenBinaryFile>, u32, u32) -> bool
        ),
        entry!(
            0x00ac_5cf0,
            bs_face_gen_binary_file_read(Ptr<BSFaceGenBinaryFile>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00ac_5e00,
            bs_face_gen_binary_file_read_header(Ptr<BSFaceGenBinaryFile>, u32, u32, u32) -> bool
        ),
        entry!(0x00ac_5f00, fn_00ac5f00(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_6100, fn_00ac6100(Ptr, u32, u32, u32, u32) -> i32),
        entry!(0x00ac_61c0, fn_00ac61c0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(
            0x00ac_43b0,
            fut_binary_file_c_open_for_read(Ptr<BSFaceGenBinaryFile>, StringArg) -> bool
        ),
        entry!(
            0x00ac_4910,
            fut_binary_file_c_open_for_overwrite(Ptr<BSFaceGenBinaryFile>, StringArg) -> bool
        ),
        entry!(
            0x00ac_4b00,
            fn_00ac4b00(Ptr<BSFaceGenBinaryFile>, u32) -> bool
        ),
        entry!(
            0x00ac_4f00,
            fn_00ac4f00(Ptr<BSFaceGenBinaryFile>, u32) -> bool
        ),
        entry!(
            0x00ac_5220,
            fut_binary_file_c_write_header(Ptr<BSFaceGenBinaryFile>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00ac_52b0,
            fut_binary_file_c_read_header(Ptr<BSFaceGenBinaryFile>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00ac_5340,
            fut_binary_file_c_write(Ptr<BSFaceGenBinaryFile>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00ac_5370,
            fut_binary_file_c_read(Ptr<BSFaceGenBinaryFile>, u32, u32, u32) -> bool
        ),
        entry!(
            0x00ac_53a0,
            fut_binary_file_c_seek(Ptr<BSFaceGenBinaryFile>, u32) -> bool
        ),
        entry!(
            0x00ac_5e90,
            fn_00ac5e90(Ptr<BSFaceGenBinaryFile>, u32) -> bool
        ),
        entry!(0x00ac_5ed0, fn_00ac5ed0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const READ_FN: u32 = 0x0040_6000;
    const FILE_VTABLE: u32 = 0x0090_8000;
    const STREAM_WRITE_FN: u32 = 0x0040_6010;
    const STREAM_DELETE_FN: u32 = 0x0040_6020;
    const STREAM_VTABLE: u32 = 0x0090_8100;

    /// Writes a short (inline) string.
    fn put_string(e: &mut Engine, at: u32, text: &[u8]) {
        e.mem.set_u32(at, 0);
        let mut chars = text.to_vec();
        chars.push(0);
        e.mem.write(at + 4, &chars);
        e.mem.set_u32(at + STRING_LENGTH, text.len() as u32);
        e.mem.set_u32(at + STRING_CAPACITY, 15);
    }

    fn string_text(e: &Engine, at: u32) -> Vec<u8> {
        let length = e.mem.u32(at + STRING_LENGTH);
        e.mem.bytes(string_chars(e, at), length)
    }

    /// Doubles for the string helpers (short strings only), the C helpers and the checks.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x10000);
        e.map(0x011c_3000, 0x1000);
        e.set_global(STRING_NPOS, u32::MAX);
        e.register(STRING_TIDY, |e, a| {
            if a[1] == 0 {
                put_string(e, a[0], b"");
            }
            Ret::default()
        });
        e.register(STRING_ASSIGN, |e, a| {
            let text = string_text(e, a[1]);
            put_string(e, a[0], &text);
            Ret::default()
        });
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(MEMCMP, |e, a| {
            let left = e.mem.bytes(a[0], a[2]);
            let right = e.mem.bytes(a[1], a[2]);
            let r = match left.cmp(&right) {
                std::cmp::Ordering::Less => -1i32,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            r.into_ret()
        });
        e.register(MEMMOVE_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        for addr in [
            INVALID_PARAMETER,
            SET_LAST_ERROR,
            FR2_ASSERT,
            FCLOSE,
            STRING_ASSIGN_CHARS,
            LOADED_FILE_ADD_REF,
            LOADED_FILE_RELEASE,
            MEMORY_MANAGER_FREE,
            BYTE_VECTOR_TIDY,
            BYTE_VECTOR_FREE_PROXY,
            VECTOR_BASE_CONSTRUCT,
            FUT_BINARY_FILE_CONSTRUCT,
            THROW_OUT_OF_RANGE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        // A byte vector of the size; the buy allocates the count.
        e.register(BYTE_VECTOR_CONSTRUCT, |e, a| {
            let block = e.mem.alloc(a[1].max(1));
            e.mem.set_u32(a[0] + VECTOR_FIRST, block);
            e.mem.set_u32(a[0] + VECTOR_LAST, block + a[1]);
            Ret::default()
        });
        e.register(VECTOR_BUY, |e, a| {
            let block = e.mem.alloc(a[1].max(1));
            e.mem.set_u32(a[0] + VECTOR_FIRST, block);
            e.mem.set_u32(a[0] + VECTOR_LAST, block);
            1u32.into_ret()
        });
        e
    }

    fn new_file(e: &mut Engine) -> Ptr<BSFaceGenBinaryFile> {
        let file: Ptr<BSFaceGenBinaryFile> = e.new_object();
        put_string(e, file.addr() + 4, b"");
        put_string(e, file.addr() + 0x20, b"FRCTL001");
        e.put_vtable(FILE_VTABLE, &[0, 0, 0, 0, 0x00ac_5cf0]);
        e.mem.set_u32(file.addr(), FILE_VTABLE);
        file
    }

    /// A stream whose read function copies `header` into the buffer.
    fn new_stream(e: &mut Engine, header: &[u8]) -> u32 {
        let header = header.to_vec();
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream, STREAM_VTABLE);
        e.mem.set_u32(stream + STREAM_READ_FUNCTION, READ_FN);
        e.register_double(READ_FN, move |e, a| {
            let n = (a[2] as usize).min(header.len());
            e.mem.write(a[1], &header[..n]);
            (n as u32).into_ret()
        });
        e.register(STREAM_WRITE_FN, |_, a| a[2].into_ret());
        e.register(STREAM_DELETE_FN, |_, _| Ret::default());
        let mut slots = vec![STREAM_DELETE_FN; 0x12];
        slots[0x11] = STREAM_WRITE_FN;
        e.put_vtable(STREAM_VTABLE, &slots);
        stream
    }

    fn name_arg(e: &mut Engine, text: &[u8]) -> StringArg {
        let at = e.mem.alloc(0x1c);
        put_string(e, at, text);
        StringArg::load(e, at)
    }

    #[test]
    fn close_closes_and_empties_the_name() {
        let mut e = engine();
        let file = new_file(&mut e);
        e.set(file, BSFaceGenBinaryFile::fptr, 0x5555);
        e.mem.set_u8(EMPTY_STRING, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ac_5430, &args![file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FCLOSE), vec![vec![0x5555]]);
        assert_eq!(
            calls_to(&log, STRING_ASSIGN_CHARS),
            vec![vec![file.addr() + 4, EMPTY_STRING, 0]]
        );
        assert_eq!(e.get(file, BSFaceGenBinaryFile::fptr), 0);
        e.call_log = Some(vec![]);
        e.call(0x00ac_5430, &args![file]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FCLOSE).is_empty());
    }

    #[test]
    fn constructor_builds_the_base_with_a_copy_and_destroys_the_id() {
        let mut e = engine();
        let file: Ptr<BSFaceGenBinaryFile> = e.new_object();
        e.set(file, BSFaceGenBinaryFile::pFile, 7);
        e.set(file, BSFaceGenBinaryFile::pLoadedFile, 8);
        let id = name_arg(&mut e, b"FRCTL001");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ac_5490, &args![file, id]).u32(), file.addr());
        let log = e.call_log.take().unwrap();
        let base = calls_to(&log, FUT_BINARY_FILE_CONSTRUCT);
        assert_eq!(base.len(), 1);
        assert_eq!(base[0][0], file.addr());
        assert_eq!(&base[0][1..], &id.0[..]);
        assert_eq!(e.mem.u32(file.addr()), BS_FACE_GEN_BINARY_FILE_VTABLE);
        assert_eq!(e.get(file, BSFaceGenBinaryFile::pFile), 0);
        assert_eq!(e.get(file, BSFaceGenBinaryFile::pLoadedFile), 0);
        let tidies = calls_to(&log, STRING_TIDY);
        assert_eq!(tidies.last().unwrap()[1..], [1, 0]);
    }

    fn open(e: &mut Engine, file: Ptr<BSFaceGenBinaryFile>, shared: u8) -> (bool, Log) {
        let name = name_arg(e, b"face.ctl");
        e.call_log = Some(vec![]);
        let ok = e.call(0x00ac_5540, &args![file, name, shared]).bool();
        (ok, e.call_log.take().unwrap())
    }

    #[test]
    fn open_without_a_file_reports_error_4() {
        let mut e = engine();
        e.register(FILE_FINDER_GET_FILE, |_, _| Ret::default());
        let file = new_file(&mut e);
        let (ok, log) = open(&mut e, file, 1);
        assert!(!ok);
        assert_eq!(
            calls_to(&log, FILE_FINDER_GET_FILE)[0][1..],
            [0, 0x2800, 0x100]
        );
        assert_eq!(calls_to(&log, SET_LAST_ERROR)[0][0], 4);
        assert!(calls_to(&log, BYTE_VECTOR_CONSTRUCT).is_empty());
    }

    #[test]
    fn open_reads_and_checks_the_header() {
        let mut e = engine();
        let stream = new_stream(&mut e, b"FRCTL001\0");
        e.register_double(FILE_FINDER_GET_FILE, move |_, _| stream.into_ret());
        let file = new_file(&mut e);
        let (ok, log) = open(&mut e, file, 0);
        assert!(ok);
        assert_eq!(calls_to(&log, FILE_FINDER_GET_FILE)[0][3], 1);
        assert!(calls_to(&log, SET_LAST_ERROR).is_empty());
        assert_eq!(calls_to(&log, BYTE_VECTOR_CONSTRUCT)[0][1], 9);
        assert_eq!(string_text(&e, file.addr() + 4), b"face.ctl".to_vec());
        assert_eq!(string_text(&e, file.addr() + 0x20), b"FRCTL001".to_vec());
        assert_eq!(calls_to(&log, MEMORY_MANAGER_FREE).len(), 2);
        assert_eq!(e.get(file, BSFaceGenBinaryFile::pFile), stream);
    }

    #[test]
    fn open_takes_an_older_version_and_refuses_another_id() {
        // A version character at or below `0` in the header: the id takes it.
        let mut e = engine();
        let stream = new_stream(&mut e, b"FRCTL 01\0");
        e.register_double(FILE_FINDER_GET_FILE, move |_, _| stream.into_ret());
        let file = new_file(&mut e);
        let (ok, _) = open(&mut e, file, 0);
        assert!(ok);
        assert_eq!(string_text(&e, file.addr() + 0x20), b"FRCTL 01".to_vec());

        // A newer version than the id's: error 4 and the file closed.
        let mut e = engine();
        let stream = new_stream(&mut e, b"FRCTL101\0");
        e.register_double(FILE_FINDER_GET_FILE, move |_, _| stream.into_ret());
        let file = new_file(&mut e);
        let (ok, log) = open(&mut e, file, 0);
        assert!(!ok);
        assert_eq!(calls_to(&log, SET_LAST_ERROR)[0][0], 4);
        assert_eq!(calls_to(&log, STREAM_DELETE_FN), vec![vec![stream, 1]]);

        // Another id: error 2.
        let mut e = engine();
        let stream = new_stream(&mut e, b"FREGM001\0");
        e.register_double(FILE_FINDER_GET_FILE, move |_, _| stream.into_ret());
        let file = new_file(&mut e);
        let (ok, log) = open(&mut e, file, 0);
        assert!(!ok);
        assert_eq!(calls_to(&log, SET_LAST_ERROR)[0][0], 2);
        assert_eq!(calls_to(&log, BYTE_VECTOR_TIDY).len(), 2);
    }

    #[test]
    fn open_uses_a_loaded_file_and_fails_on_a_short_read() {
        let mut e = engine();
        let stream = new_stream(&mut e, b"FRC");
        let loaded = e.mem.alloc(0x10);
        e.mem.set_u32(loaded + LOADED_FILE_FILE, stream);
        e.set_global(LOADED_FILE_MAP, 0x00b0_0000u32);
        e.register_double(LOADED_FILE_LOOKUP, move |_, _| loaded.into_ret());
        let file = new_file(&mut e);
        let (ok, log) = open(&mut e, file, 0);
        assert!(!ok);
        assert_eq!(calls_to(&log, LOADED_FILE_LOOKUP)[0][0], 0x00b0_0000);
        assert_eq!(
            calls_to(&log, LOADED_FILE_ADD_REF),
            vec![vec![loaded], vec![loaded]]
        );
        assert_eq!(
            calls_to(&log, LOADED_FILE_RELEASE),
            vec![vec![loaded], vec![loaded]]
        );
        assert_eq!(e.mem.u8(loaded + LOADED_FILE_USED), 1);
        // The failed header read: error 1 and the loaded file closed.
        assert_eq!(calls_to(&log, SET_LAST_ERROR)[0][0], 1);
        assert_eq!(e.get(file, BSFaceGenBinaryFile::pLoadedFile), 0);
        assert_eq!(e.get(file, BSFaceGenBinaryFile::pFile), 0);
    }

    #[test]
    fn close_file_deletes_or_releases() {
        let mut e = engine();
        let stream = new_stream(&mut e, b"");
        let file = new_file(&mut e);
        e.set(file, BSFaceGenBinaryFile::pFile, stream);
        e.call_log = Some(vec![]);
        e.call(0x00ac_5c00, &args![file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STREAM_DELETE_FN), vec![vec![stream, 1]]);
        assert_eq!(e.get(file, BSFaceGenBinaryFile::pFile), 0);
        e.set(file, BSFaceGenBinaryFile::pFile, stream);
        e.set(file, BSFaceGenBinaryFile::pLoadedFile, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00ac_5c00, &args![file]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, STREAM_DELETE_FN).is_empty());
        assert_eq!(calls_to(&log, LOADED_FILE_RELEASE), vec![vec![0x4444]]);
        assert_eq!(e.get(file, BSFaceGenBinaryFile::pFile), 0);
        assert_eq!(e.get(file, BSFaceGenBinaryFile::pLoadedFile), 0);
    }

    #[test]
    fn write_doubles_the_count_and_advances() {
        let mut e = engine();
        let file = new_file(&mut e);
        assert!(!e.call(0x00ac_5c70, &args![file, 0x100u32, 3u32]).bool());
        let stream = new_stream(&mut e, b"");
        e.set(file, BSFaceGenBinaryFile::pFile, stream);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ac_5c70, &args![file, 0x100u32, 3u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, STREAM_WRITE_FN),
            vec![vec![stream, 0x100, 6]]
        );
        assert_eq!(e.mem.u32(stream + STREAM_POSITION), 6);
        e.register(STREAM_WRITE_FN, |_, _| 1u32.into_ret());
        assert!(!e.call(0x00ac_5c70, &args![file, 0x100u32, 3u32]).bool());
        assert_eq!(e.mem.u32(stream + STREAM_POSITION), 7);
    }

    #[test]
    fn read_passes_the_element_size_by_address() {
        let mut e = engine();
        let file = new_file(&mut e);
        let buffer = e.mem.alloc(0x40);
        assert!(!e.call(0x00ac_5cf0, &args![file, buffer, 4u32, 2u32]).bool());
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_READ_FUNCTION, READ_FN);
        e.set(file, BSFaceGenBinaryFile::pFile, stream);
        let sizes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = sizes.clone();
        e.register_double(READ_FN, move |e, a| {
            seen.borrow_mut().push((a[2], e.mem.u32(a[3]), a[4]));
            a[2].into_ret()
        });
        e.call_log = Some(vec![]);
        for size in [1u32, 2, 4, 8, 0, 3] {
            assert!(e.call(0x00ac_5cf0, &args![file, buffer, size, 2u32]).bool());
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(
            *sizes.borrow(),
            vec![
                (2, 1, 1),
                (4, 2, 1),
                (8, 4, 1),
                (16, 4, 1),
                (0, 4, 1),
                (6, 1, 1)
            ]
        );
        assert_eq!(
            calls_to(&log, FR2_ASSERT),
            vec![vec![BINARY_FILE_SOURCE, 0x1fc]]
        );
        assert_eq!(e.mem.u32(stream + STREAM_POSITION), 36);
        // A short read answers false.
        e.register(READ_FN, |_, _| 1u32.into_ret());
        assert!(!e.call(0x00ac_5cf0, &args![file, buffer, 4u32, 2u32]).bool());
    }

    #[test]
    fn read_header_reads_bytes_and_reports_failures() {
        let mut e = engine();
        let file = new_file(&mut e);
        let buffer = e.mem.alloc(0x40);
        assert!(!e.call(0x00ac_5e00, &args![file, buffer, 8u32, 1u32]).bool());
        let stream = new_stream(&mut e, b"FRCTL001");
        e.set(file, BSFaceGenBinaryFile::pFile, stream);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ac_5e00, &args![file, buffer, 4u32, 2u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x00ac_5cf0),
            vec![vec![file.addr(), buffer, 1, 8]]
        );
        assert_eq!(e.mem.bytes(buffer, 8), b"FRCTL001".to_vec());
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00ac_5e00, &args![file, buffer, 0x10u32, 1u32])
            .bool());
        let log = e.call_log.take().unwrap();
        let error = calls_to(&log, SET_LAST_ERROR);
        assert_eq!(error.len(), 1);
        assert_eq!(error[0][0], 1);
        assert_eq!(e.get(file, BSFaceGenBinaryFile::pFile), 0);
    }

    #[test]
    fn vector_copy_copies_the_elements() {
        let mut e = engine();
        let source = e.mem.alloc(0x18);
        let block = e.mem.alloc(6);
        e.mem.write(block, b"abcdef");
        e.mem.set_u32(source + VECTOR_PROXY, 0x99);
        e.mem.set_u32(source + VECTOR_FIRST, block);
        e.mem.set_u32(source + VECTOR_LAST, block + 6);
        let copy = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ac_5f00, &args![copy, source]).u32(), copy);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, VECTOR_BASE_CONSTRUCT),
            vec![vec![copy, copy]]
        );
        assert_eq!(calls_to(&log, VECTOR_BUY), vec![vec![copy, 6]]);
        assert!(calls_to(&log, INVALID_PARAMETER).is_empty());
        let first = e.mem.u32(copy + VECTOR_FIRST);
        assert_eq!(e.mem.u32(copy + VECTOR_LAST), first + 6);
        assert_eq!(e.mem.bytes(first, 6), b"abcdef".to_vec());
        // A failed buy copies nothing.
        e.register(VECTOR_BUY, |_, _| Ret::default());
        let other = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00ac_5f00, &args![other, source]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MEMMOVE_S).is_empty());
    }

    #[test]
    fn compare_orders_by_bytes_then_length() {
        let mut e = engine();
        let string = e.mem.alloc(0x1c);
        put_string(&mut e, string, b"FRCTL001");
        let text = e.mem.alloc(16);
        e.mem.write(text, b"FRCTL001X");
        let cmp = |e: &mut Engine, offset: u32, count: u32, length: u32| {
            e.call(0x00ac_6100, &args![string, offset, count, text, length])
                .i32()
        };
        assert_eq!(cmp(&mut e, 0, 8, 8), 0);
        assert_eq!(cmp(&mut e, 0, 8, 9), -1);
        assert_eq!(cmp(&mut e, 0, 8, 7), 1);
        // The count is clipped to the length.
        assert_eq!(cmp(&mut e, 0, 100, 8), 0);
        e.mem.write(text, b"FRCTL002");
        assert_eq!(cmp(&mut e, 0, 8, 8), -1);
        // An offset past the length throws (and goes on).
        e.call_log = Some(vec![]);
        cmp(&mut e, 9, 0, 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, THROW_OUT_OF_RANGE).len(), 1);
    }

    #[test]
    fn copy_range_moves_and_returns_the_end() {
        let mut e = engine();
        let source = e.mem.alloc(8);
        e.mem.write(source, b"xyz");
        let destination = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let end = e
            .call(
                0x00ac_61c0,
                &args![1u32, source, 2u32, source + 3, destination, 3u32],
            )
            .u32();
        assert_eq!(end, destination + 3);
        assert_eq!(e.mem.bytes(destination, 3), b"xyz".to_vec());
        let empty = e
            .call(
                0x00ac_61c0,
                &args![1u32, source, 2u32, source, destination, 3u32],
            )
            .u32();
        assert_eq!(empty, destination);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MEMMOVE_S).len(), 1);
    }

    const C_FILE: u32 = 0x0050_0000;

    /// The C file doubles: `fopen` answers `opened` (0 for a failure), `fread` copies `header`
    /// and answers the count (0 when it has not enough bytes), `fwrite` answers `written`.
    fn c_file_engine(opened: u32, header: &'static [u8], written: u32) -> Engine {
        let mut e = engine();
        e.register_double(FOPEN, move |_, _| opened.into_ret());
        e.register_double(FREAD, move |e, a| {
            let bytes = (a[1] * a[2]) as usize;
            if bytes > header.len() {
                return Ret::default();
            }
            e.mem.write(a[0], &header[..bytes]);
            a[2].into_ret()
        });
        e.register_double(FWRITE, move |_, _| written.into_ret());
        e
    }

    fn error_codes(log: &Log) -> Vec<u32> {
        calls_to(log, SET_LAST_ERROR).iter().map(|w| w[0]).collect()
    }

    #[test]
    fn c_file_open_for_read_checks_the_header() {
        for (opened, header, codes, ok) in [
            (C_FILE, &b"FRCTL001"[..], vec![], true),
            (C_FILE, &b"FRXTL001"[..], vec![2], false),
            (C_FILE, &b"FRCTL101"[..], vec![4], false),
            (C_FILE, &b"FRC"[..], vec![1], false),
            (0, &b"FRCTL001"[..], vec![4], false),
        ] {
            let mut e = c_file_engine(opened, header, 1);
            let file = new_file(&mut e);
            let name = name_arg(&mut e, b"a.ctl");
            e.call_log = Some(vec![]);
            let answer = e.call(0x00ac_43b0, &args![file, name]).bool();
            let log = e.call_log.take().unwrap();
            assert_eq!(answer, ok, "{header:?}");
            assert_eq!(error_codes(&log), codes, "{header:?}");
            assert_eq!(calls_to(&log, FOPEN)[0][1], MODE_READ);
            if ok {
                assert_eq!(string_text(&e, file.addr() + 4), b"a.ctl".to_vec());
                assert_eq!(e.get(file, BSFaceGenBinaryFile::fptr), C_FILE);
                assert_eq!(calls_to(&log, MEMORY_MANAGER_FREE).len(), 1);
                assert_eq!(calls_to(&log, BYTE_VECTOR_FREE_PROXY).len(), 1);
            } else if opened != 0 && header.len() == 8 {
                // A failed check closes the file.
                assert_eq!(calls_to(&log, FCLOSE), vec![vec![C_FILE]]);
                assert_eq!(calls_to(&log, BYTE_VECTOR_FREE_PROXY).len(), 2);
            }
            // The name is destroyed on every path.
            assert_eq!(calls_to(&log, STRING_TIDY).last().unwrap()[1..], [1, 0]);
        }
    }

    #[test]
    fn c_file_open_for_overwrite_writes_the_id() {
        for (opened, written, ok) in [(C_FILE, 1u32, true), (C_FILE, 0, false), (0, 1, false)] {
            let mut e = c_file_engine(opened, b"", written);
            let file = new_file(&mut e);
            let name = name_arg(&mut e, b"b.egm");
            e.call_log = Some(vec![]);
            let answer = e.call(0x00ac_4910, &args![file, name]).bool();
            let log = e.call_log.take().unwrap();
            assert_eq!(answer, ok);
            assert_eq!(calls_to(&log, FOPEN)[0][1], MODE_WRITE);
            if ok {
                let writes = calls_to(&log, FWRITE);
                assert_eq!(writes[0][1..], [8, 1, C_FILE]);
                assert_eq!(e.mem.bytes(writes[0][0], 8), b"FRCTL001".to_vec());
                assert_eq!(string_text(&e, file.addr() + 4), b"b.egm".to_vec());
                assert!(error_codes(&log).is_empty());
            } else if opened != 0 {
                // The failed write reports 6, then the open 5.
                assert_eq!(error_codes(&log), vec![6, 5]);
            } else {
                assert_eq!(error_codes(&log), vec![5]);
                assert!(calls_to(&log, FWRITE).is_empty());
            }
        }
    }

    #[test]
    fn c_file_open_for_update_and_append() {
        for (addr, mode) in [(0x00ac_4b00u32, MODE_UPDATE), (0x00ac_4f00, MODE_APPEND)] {
            for (header, codes, ok) in [
                (&b"FRCTL001"[..], vec![], true),
                (&b"FRCTX001"[..], vec![2], false),
                (&b"F"[..], vec![1], false),
            ] {
                let mut e = c_file_engine(C_FILE, header, 1);
                let file = new_file(&mut e);
                // The id's version is 1: the header gets it before the compare.
                put_string(&mut e, file.addr() + 0x20, b"FRCTL101");
                let name = e.mem.alloc(0x1c);
                put_string(&mut e, name, b"c.tri");
                e.call_log = Some(vec![]);
                let answer = e.call(addr, &args![file, name]).bool();
                let log = e.call_log.take().unwrap();
                assert_eq!(answer, ok, "{addr:x} {header:?}");
                assert_eq!(error_codes(&log), codes, "{addr:x} {header:?}");
                assert_eq!(calls_to(&log, FOPEN)[0][1], mode);
                if ok {
                    assert_eq!(string_text(&e, file.addr() + 4), b"c.tri".to_vec());
                    // The update resets the id's version to '0'; the append keeps it.
                    let id = string_text(&e, file.addr() + 0x20);
                    assert_eq!(id[5], if addr == 0x00ac_4b00 { b'0' } else { b'1' });
                }
            }
            let mut e = c_file_engine(0, b"", 1);
            let file = new_file(&mut e);
            let name = e.mem.alloc(0x1c);
            put_string(&mut e, name, b"c.tri");
            e.call_log = Some(vec![]);
            assert!(!e.call(addr, &args![file, name]).bool());
            let log = e.call_log.take().unwrap();
            assert_eq!(error_codes(&log), vec![4]);
            assert!(calls_to(&log, FREAD).is_empty());
        }
    }

    #[test]
    fn c_file_transfers_and_seek_report_failures() {
        let mut e = c_file_engine(C_FILE, b"abcd", 2);
        let file = new_file(&mut e);
        let buffer = e.mem.alloc(8);
        // No file: false and nothing called.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ac_5370, &args![file, buffer, 2u32, 2u32]).bool());
        assert!(!e.call(0x00ac_53a0, &args![file, 4u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FREAD).is_empty());
        e.set(file, BSFaceGenBinaryFile::fptr, C_FILE);
        assert!(e.call(0x00ac_5370, &args![file, buffer, 2u32, 2u32]).bool());
        assert_eq!(e.mem.bytes(buffer, 4), b"abcd".to_vec());
        assert!(e.call(0x00ac_5340, &args![file, buffer, 1u32, 2u32]).bool());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ac_52b0, &args![file, buffer, 4u32, 2u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(error_codes(&log), vec![1]);
        assert_eq!(e.get(file, BSFaceGenBinaryFile::fptr), 0);
        e.set(file, BSFaceGenBinaryFile::fptr, C_FILE);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ac_5220, &args![file, buffer, 1u32, 3u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(error_codes(&log), vec![6]);
        // The seek: 0 from fseek is success.
        e.register(FSEEK, |_, a| u32::from(a[1] != 4).into_ret());
        e.set(file, BSFaceGenBinaryFile::fptr, C_FILE);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ac_53a0, &args![file, 4u32]).bool());
        assert!(!e.call(0x00ac_53a0, &args![file, 5u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FSEEK),
            vec![vec![C_FILE, 4, 0], vec![C_FILE, 5, 0]]
        );
        assert_eq!(error_codes(&log), vec![7]);
    }

    #[test]
    fn game_file_seek_and_file_holder_destructor() {
        let mut e = engine();
        e.map(0x010a_2000, 0x1000);
        e.set_global(SEEK_ORIGIN, 0u32);
        let file = new_file(&mut e);
        assert!(!e.call(0x00ac_5e90, &args![file, 9u32]).bool());
        let stream = e.mem.alloc(0x10);
        e.put_vtable(0x0090_8200, &[0, 0, 0, 0, 0, 0x0040_6030]);
        e.mem.set_u32(stream, 0x0090_8200);
        e.register(0x0040_6030, |_, _| Ret::default());
        e.set(file, BSFaceGenBinaryFile::pFile, stream);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ac_5e90, &args![file, 9u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0040_6030), vec![vec![stream, 9, 0]]);
        let holder = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00ac_5ed0, &args![holder]);
        e.mem.set_u32(holder, C_FILE);
        e.call(0x00ac_5ed0, &args![holder]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FCLOSE), vec![vec![C_FILE]]);
    }
}

//! `BSSystem/archive.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PC classes, from their vtables' RTTI and the code's offsets:
//! `Archive` (vtables `010a45ac` for its `BSFile` part and `010a459c` for its
//! `NiRefObject` part at +0x158), `ArchiveFile` (`010a45fc`) and
//! `CompressedArchiveFile` (`010a464c`). Their `BSFile` base is 0x158 bytes on
//! PC (0x1a0 in the Xbox PDB), so the PDB's offsets past it are 0x48 lower;
//! `Archive`'s critical section is 0x18 bytes on PC (0x28 in the PDB), so its
//! fields after it are 0x58 lower (the PC `Archive` is 0x1d0 bytes, the size
//! its deleting destructor frees).
//!
//! The `ArchiveFile` vtable's slots follow `BSFile`'s (see `bsfile.rs`): +0x10
//! `SetEndianSwap` (`00afbfa0`), +0x14 the seek (`00afb670`), +0x20 `Open`
//! (`00afb5f0`), +0x28 `GetSize` (`00afb780`), +0x44 `ReadF` (`00afbbf0`); the
//! `CompressedArchiveFile` vtable has `00afc620`, `00afc080`, `00afc0c0` at
//! +0x1c, `00afc060`, `00afc0a0` and `00afc0e0` there.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bssystem::bsfile::BSFile;
use crate::units::nisystem::run_00aa1380::NiFile;

layout! {
    /// `Archive` (Xbox PDB), 0x1d0 bytes on PC: a `BSFile`, an `NiRefObject`
    /// at +0x158 and the `BSArchive` header at +0x160 (PC offsets; the PDB's
    /// are 0x48 higher up to the critical section and 0x58 higher after it).
    pub struct Archive: 0x1d0 {
        /// The vtable of the `NiRefObject` part (`010a459c`).
        0x158 ref_object_vtable: u32,
        /// `NiRefObject::m_uiRefCount` (Xbox PDB).
        0x15C m_uiRefCount: u32,
        /// `BSArchiveHeader::iTag` (Xbox PDB).
        0x160 iTag: u32,
        /// `BSArchiveHeader::iVersion` (Xbox PDB).
        0x164 iVersion: u32,
        /// `BSArchiveHeader::iHeaderSize` (Xbox PDB).
        0x168 iHeaderSize: u32,
        /// `BSArchiveHeader::oFlags` (Xbox PDB).
        0x16C oFlags: u32,
        /// `BSArchiveHeader::iDirectories` (Xbox PDB).
        0x170 iDirectories: u32,
        /// `BSArchiveHeader::iFiles` (Xbox PDB).
        0x174 iFiles: u32,
        /// `BSArchiveHeader::iDirectoryNamesLength` (Xbox PDB).
        0x178 iDirectoryNamesLength: u32,
        /// `BSArchiveHeader::iFileNamesLength` (Xbox PDB).
        0x17C iFileNamesLength: u32,
        /// `BSArchiveHeader::iArchiveType` (Xbox PDB).
        0x180 iArchiveType: u16,
        /// `BSArchive::pDirectories` (Xbox PDB): 16-byte directory entries,
        /// each with its file entries' array at +0xc.
        0x184 pDirectories: u32,
        /// `ArchiveFileTime` (Xbox PDB).
        0x188 ArchiveFileTime: u64,
        /// `iFileNameArrayOffset` (Xbox PDB).
        0x190 iFileNameArrayOffset: u32,
        /// `iLastDirectoryIndex` (Xbox PDB).
        0x194 iLastDirectoryIndex: u32,
        /// `iLastFileIndex` (Xbox PDB).
        0x198 iLastFileIndex: u32,
        /// `ArchiveCriticalSection` (Xbox PDB): a Windows critical section here.
        0x19C ArchiveCriticalSection: u32,
        /// `cArchiveFlags` (Xbox PDB).
        0x1B4 cArchiveFlags: u8,
        /// `pDirectoryStringArray` (Xbox PDB).
        0x1B8 pDirectoryStringArray: u32,
        /// `pDirectoryStringOffsets` (Xbox PDB).
        0x1BC pDirectoryStringOffsets: u32,
        /// `pFileNameStringArray` (Xbox PDB).
        0x1C0 pFileNameStringArray: u32,
        /// `pFileNameStringOffsets` (Xbox PDB): one array per directory.
        0x1C4 pFileNameStringOffsets: u32,
        /// `m_iRuntimeID` (Xbox PDB).
        0x1C8 m_iRuntimeID: u32,
    }

    /// `ArchiveFile` (Xbox PDB), 0x160 bytes on PC: a `BSFile` reading one
    /// file of an archive.
    pub struct ArchiveFile: 0x160 {
        /// `spArchive` (Xbox PDB): an `NiPointer<Archive>`.
        0x158 spArchive: u32,
        /// `iArchiveOffset` (Xbox PDB).
        0x15C iArchiveOffset: u32,
    }

    /// `CompressedArchiveFile` (Xbox PDB) on PC: an `ArchiveFile` that
    /// inflates its data (the PDB's `bXMemCodec` is not in the PC class).
    pub struct CompressedArchiveFile: 0x178 {
        /// `pZStream` (Xbox PDB): the zlib stream (0x1be0 bytes allocated).
        0x160 pZStream: u32,
        /// `pUncompressedBuffer` (Xbox PDB).
        0x164 pUncompressedBuffer: u32,
        /// `iUncompressedFileSize` (Xbox PDB).
        0x168 iUncompressedFileSize: u32,
        /// `iUncompressedBufferSize` (Xbox PDB).
        0x16C iUncompressedBufferSize: u32,
        /// `iUncompressedBufferPos` (Xbox PDB).
        0x170 iUncompressedBufferPos: u32,
        /// `iUncompressedBufferReadSize` (Xbox PDB).
        0x174 iUncompressedBufferReadSize: u32,
    }
}

/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, thiscall:
/// size) and deallocate (`00aa4060`, thiscall: block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// Import slots (KERNEL32): `InterlockedIncrement`, `InterlockedDecrement`,
/// `EnterCriticalSection`, `LeaveCriticalSection`, `DeleteCriticalSection`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// CRT: `strlen`, `strcpy_s`, `strcat_s`, `_strnicmp` (folded body
/// `00ec7ec0`), `_stricmp`, `memcpy`, `_access`, all cdecl.
const STRLEN: u32 = 0x00ec_6130;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRCAT_S: u32 = 0x00ec_6bd2;
const STRNICMP: u32 = 0x00ec_7ec0;
const STRICMP: u32 = 0x00ec_68e4;
const MEMCPY: u32 = 0x00ec_44d0;
const ACCESS: u32 = 0x00ec_bf05;
/// `00ec8feb` (cdecl: string, size): `00af4200` runs it on its result (an
/// in-place string conversion; not identified further).
const STRING_IN_PLACE: u32 = 0x00ec_8feb;
/// The `NiRefObject` vtable slot (+4) called when a count drops to 0.
const DELETE_THIS_SLOT: u32 = 0x4;
/// Offset of the `NiRefObject` part of an `Archive`, and of its count.
const ARCHIVE_REF_OBJECT: u32 = 0x158;
const ARCHIVE_REF_COUNT: u32 = 0x15c;

/// Globals of this unit (the PDB's `ArchiveManager` statics are not mapped
/// to them one by one; each is described by what the code does with it).
/// A byte `00af4460` sets.
const FLAG_SET_BY_00AF4460: u32 = 0x011f_8154;
/// The `ArchiveFile` buffer size cap (`00af4540` sets 0x20000).
const ARCHIVE_FILE_BUFFER_SIZE: u32 = 0x011f_8164;
/// The three words `00af4470` sets.
const WORDS_SET_BY_00AF4470: u32 = 0x011a_cdfc;
/// A byte `00af43a0` sets; `FileFinder` reads it too.
const FLAG_011ACDF8: u32 = 0x011a_cdf8;
/// A byte `00af4490` sets; `FileFinder` and `00afb190` read it too.
const FLAG_011ACDF9: u32 = 0x011a_cdf9;
/// The string copies `00af4490` and `00af43a0` allocate.
const STRING_011F8158: u32 = 0x011f_8158;
const STRING_011F815C: u32 = 0x011f_815c;
/// A `char[0x104]` path `00af43a0` fills.
const PATH_011F8050: u32 = 0x011f_8050;
/// The head node of the list of opened archives (`BSSimpleList` of
/// `NiPointer<Archive>`; 0 when there is none).
const ARCHIVE_LIST: u32 = 0x011f_8160;
/// The archive type table `00af7db0` searches: 8-byte entries, a
/// `char[4]` extension and the type (`ArchiveTypeExtension`, Xbox PDB).
const ARCHIVE_TYPE_TABLE: u32 = 0x011a_ce08;
/// `NiRefObject`'s object count (decremented by the `Archive` destructor)
/// and vtable (installed there before the `BSFile` destructor runs).
const REF_OBJECT_COUNT: u32 = 0x011f_4400;
const REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
/// The vtables of this unit's classes.
const ARCHIVE_VTABLE: u32 = 0x010a_45ac;
const ARCHIVE_REF_OBJECT_VTABLE: u32 = 0x010a_459c;
const ARCHIVE_FILE_VTABLE: u32 = 0x010a_45fc;
const COMPRESSED_ARCHIVE_FILE_VTABLE: u32 = 0x010a_464c;
/// String literals in `.rdata`.
const TEXT_DATA_LOWER: u32 = 0x010a_4334;
const TEXT_EMPTY: u32 = 0x010a_433a;
const TEXT_TEXTURES: u32 = 0x010a_433c;
const TEXT_SPT: u32 = 0x010a_4348;
const TEXT_TREES: u32 = 0x010a_434c;
const TEXT_MESHES: u32 = 0x010a_4354;
const TEXT_MESHES_PREFIX: u32 = 0x010a_435c;
const TEXT_DATA_PREFIX: u32 = 0x010a_4364;
const TEXT_DATA_CAPITAL: u32 = 0x010a_4588;
const TEXT_ZLIB_VERSION: u32 = 0x010a_4590;

/// Other units' functions.
/// `BSFile::~BSFile` (`00aff240`) and the `BSFile` base constructor
/// (`00aff180`, thiscall, no stack words).
const BSFILE_DESTRUCTOR: u32 = 0x00af_f240;
const BSFILE_BASE_CONSTRUCTOR: u32 = 0x00af_f180;
/// `BSFile` `SetEndianSwap` (`00aff990`).
const BSFILE_SET_ENDIAN_SWAP: u32 = 0x00af_f990;
/// The plain `m_pfnWrite` (`00aff9f0`).
const BSFILE_PLAIN_WRITE: u32 = 0x00af_f9f0;
/// `NiFile`'s flush (`00aa15a0`), direct read (`00aa1570`), seek
/// (`00aa16d0`) and buffer getter (`00aa1ac0`, thiscall, `m_pBuffer`).
const NI_FILE_FLUSH: u32 = 0x00aa_15a0;
const NI_FILE_READ_DIRECT: u32 = 0x00aa_1570;
const NI_FILE_SEEK: u32 = 0x00aa_16d0;
const NI_FILE_GET_BUFFER: u32 = 0x00aa_1ac0;
/// `NiFile`'s relative seek origin (`010a2484`, a global the seek reads).
const NI_FILE_SEEK_CURRENT: u32 = 0x010a_2484;
/// `00afb7a0`'s vtable call on the archive (slot +0x18; its answer is
/// not used).
const ARCHIVE_SLOT_18: u32 = 0x18;
/// `00b01800` (thiscall on an `ArchiveFile`: buffer, size): the read
/// `ArchiveFile::ReadF` clamps and counts.
const ARCHIVE_FILE_READ: u32 = 0x00b0_1800;
/// zlib (`00b43fe0` `inflateInit_`, `00b44000` `inflate`, `00b45db0`
/// `inflateEnd`, by their arguments and use; all cdecl).
const INFLATE_INIT: u32 = 0x00b4_3fe0;
const INFLATE: u32 = 0x00b4_4000;
const INFLATE_END: u32 = 0x00b4_5db0;
/// `00afdbc0` (thiscall on a directory entry, one word): whether it holds
/// the given name hash.
const DIRECTORY_HAS: u32 = 0x00af_dbc0;
/// `00afcfe0` (thiscall on a hash, no words): a byte of it `00afcf00`
/// compares.
const HASH_FLAG: u32 = 0x00af_cfe0;
/// `00aff100` (no arguments): the object whose +0x5c state `00af59f0`
/// moves between 1 and 2 around its work.
const STATE_OWNER: u32 = 0x00af_f100;
const STATE_OFFSET: u32 = 0x5c;

/// Adds a reference to `archive` (`InterlockedIncrement` of its count).
fn add_ref(e: &mut Engine, archive: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![archive + ARCHIVE_REF_COUNT]);
}

/// Releases a reference to `archive`: `InterlockedDecrement` of its count,
/// and the `NiRefObject` part's vtable +4 when it reaches 0.
fn release(e: &mut Engine, archive: u32) {
    let count = archive + ARCHIVE_REF_COUNT;
    if e.call(INTERLOCKED_DECREMENT, &args![count]).u32() == 0 {
        e.vcall(archive + ARCHIVE_REF_OBJECT, DELETE_THIS_SLOT, &args![]);
    }
}

fn free(e: &mut Engine, block: u32) {
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32()
}

// Translated from 00af59e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArchiveManager::CleanupArchiveStringsAfterStartup` (Xbox PDB): empty on
/// PC.
pub fn archive_manager_cleanup_archive_strings_after_startup(_e: &mut Engine) {}

// Translated from 00af4460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): stores the byte `value` in
/// `011f8154`.
pub fn fn_00af4460(e: &mut Engine, value: u8) {
    e.set_global(FLAG_SET_BY_00AF4460, value);
}

// Translated from 00af4540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): sets the `ArchiveFile` buffer
/// size cap (`011f8164`) to 0x20000.
pub fn fn_00af4540(e: &mut Engine) {
    e.set_global(ARCHIVE_FILE_BUFFER_SIZE, 0x20000u32);
}

// Translated from 00af4470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): stores them in
/// `011acdfc`, `011ace00` and `011ace04`.
pub fn fn_00af4470(e: &mut Engine, first: u32, second: u32, third: u32) {
    e.set_global(WORDS_SET_BY_00AF4470, first);
    e.set_global(WORDS_SET_BY_00AF4470 + 4, second);
    e.set_global(WORDS_SET_BY_00AF4470 + 8, third);
}

// Translated from 00af4490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): stores the byte `flag` in
/// `011acdf9`; a non-null `text` is copied into a new block (`strlen + 1`
/// bytes from the memory manager, `strcpy_s`) kept in `011f8158`.
pub fn fn_00af4490(e: &mut Engine, flag: u8, text: Ptr) {
    e.set_global(FLAG_011ACDF9, flag);
    if text.is_null() {
        return;
    }
    let size = e.call(STRLEN, &args![text]).u32().wrapping_add(1);
    let block = allocate(e, size);
    e.set_global(STRING_011F8158, block);
    let copy = e.global::<u32>(STRING_011F8158);
    e.call(STRCPY_S, &args![copy, size, text]);
}

// Translated from 00af43a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words): stores the byte `flag` in
/// `011acdf8`; copies `list` into a new block kept in `011f815c` (a null
/// `list` gets a 0-byte block holding an empty string); copies `path` (or
/// `""`) into the `char[0x104]` at `011f8050`; stores `size` in the buffer
/// size cap `011f8164`.
pub fn fn_00af43a0(e: &mut Engine, flag: u8, path: Ptr, list: Ptr, size: u32) {
    e.set_global(FLAG_011ACDF8, flag);
    if list.is_null() {
        let block = allocate(e, 0);
        e.set_global(STRING_011F815C, block);
        let copy = e.global::<u32>(STRING_011F815C);
        e.mem.set_u8(copy, 0);
    } else {
        let length = e.call(STRLEN, &args![list]).u32().wrapping_add(1);
        let block = allocate(e, length);
        e.set_global(STRING_011F815C, block);
        let copy = e.global::<u32>(STRING_011F815C);
        e.call(STRCPY_S, &args![copy, length, list]);
    }
    let source = if path.is_null() {
        TEXT_EMPTY
    } else {
        path.addr()
    };
    e.call(STRCPY_S, &args![PATH_011F8050, 0x104u32, source]);
    e.set_global(ARCHIVE_FILE_BUFFER_SIZE, size);
}

// Translated from 00af4170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FindTexturePathOffset` (engine map; cdecl, two words): how many leading
/// characters of `path` to skip: a drive (`?:` plus one more character), a
/// `\`, a `data\` (case-insensitive), and with `textures` set a
/// `textures\` after that.
pub fn find_texture_path_offset(e: &mut Engine, path: Ptr, textures: bool) -> u32 {
    let start = path.addr();
    let mut at = start;
    if e.mem.u8(at.wrapping_add(1)) == b':' {
        at = at.wrapping_add(3);
    }
    if e.mem.u8(at) == b'\\' {
        at = at.wrapping_add(1);
    }
    if e.call(STRNICMP, &args![at, TEXT_DATA_LOWER, 5u32]).i32() == 0 {
        at = at.wrapping_add(5);
    }
    if textures && e.call(STRNICMP, &args![at, TEXT_TEXTURES, 9u32]).i32() == 0 {
        at = at.wrapping_add(9);
    }
    at.wrapping_sub(start)
}

// Translated from 00af4200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): writes `path` without
/// its drive, `\` and `data\` ([`find_texture_path_offset`] without the
/// textures step) into `out` (emptied first, then `strcat_s`), runs
/// `00ec8feb(out, size)` on it and answers true.
pub fn fn_00af4200(e: &mut Engine, path: Ptr, out: Ptr, size: u32) -> bool {
    e.mem.set_u8(out.addr(), 0);
    let offset = find_texture_path_offset(e, path, false);
    let rest = path.addr().wrapping_add(offset);
    e.call(STRCAT_S, &args![out, size, rest]);
    e.call(STRING_IN_PLACE, &args![out, size]);
    true
}

// Translated from 00af4250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): the model path for
/// `name` in `out`. A name longer than 3 ending in `spt` (case-insensitive)
/// becomes `"Trees"` followed by the name; a name starting with `meshes\`
/// is copied as it is; any other gets `meshes\` in front.
pub fn fn_00af4250(e: &mut Engine, name: Ptr, out: Ptr, size: u32) {
    let length = e.call(STRLEN, &args![name]).u32();
    if length > 3 {
        let extension = name.addr().wrapping_add(length).wrapping_sub(3);
        if e.call(STRICMP, &args![extension, TEXT_SPT]).i32() == 0 {
            e.call(STRCPY_S, &args![out, size, TEXT_TREES]);
            e.call(STRCAT_S, &args![out, size, name]);
            return;
        }
    }
    if e.call(STRNICMP, &args![name, TEXT_MESHES, 7u32]).i32() != 0 {
        e.call(STRCPY_S, &args![out, size, TEXT_MESHES_PREFIX]);
        e.call(STRCAT_S, &args![out, size, name]);
    } else {
        e.call(STRCPY_S, &args![out, size, name]);
    }
}

// Translated from 00af4310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): copies `path` into
/// `out` without a drive (`?:` and one more character, when the path is
/// longer than 3) and then without a leading `data\` (case-insensitive).
pub fn fn_00af4310(e: &mut Engine, path: Ptr, out: Ptr, size: u32) {
    let mut at = path.addr();
    let length = e.call(STRLEN, &args![at]).u32();
    if length > 3 && e.mem.u8(at.wrapping_add(1)) == b':' {
        at = at.wrapping_add(3);
    }
    if e.call(STRNICMP, &args![at, TEXT_DATA_PREFIX, 5u32]).i32() == 0 {
        at = at.wrapping_add(5);
    }
    e.call(STRCPY_S, &args![out, size, at]);
}

// Translated from 00af7db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): the archive type of a file
/// extension. One leading `.` (or a `.` second) is skipped; the table at
/// `011ace08` (8-byte entries, `char[4]` then the type) is searched by
/// halving between 0 and 0x1a with `_stricmp`, stopping when the step
/// becomes 0 or the names match. Answers the matching entry's type, or 0x100.
pub fn fn_00af7db0(e: &mut Engine, extension: Ptr) -> u32 {
    let mut name = extension.addr();
    if e.mem.u8(name) == b'.' {
        name = name.wrapping_add(1);
    } else if e.mem.u8(name.wrapping_add(1)) == b'.' {
        name = name.wrapping_add(2);
    }
    let mut low = 0u32;
    let mut high = 0x1au32;
    let mut middle = 0u32;
    let mut step = 1u32;
    let mut compared = 0i32;
    while step != 0 {
        step = high.wrapping_sub(low) >> 1;
        middle = low.wrapping_add(step);
        let entry = ARCHIVE_TYPE_TABLE.wrapping_add(middle.wrapping_mul(8));
        compared = e.call(STRICMP, &args![name, entry]).i32();
        if compared > 0 {
            low = middle;
        } else if compared < 0 {
            high = middle;
        } else {
            break;
        }
    }
    if compared == 0 {
        e.mem
            .u32(ARCHIVE_TYPE_TABLE.wrapping_add(middle.wrapping_mul(8)) + 4)
    } else {
        0x100
    }
}

// Translated from 00af7d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArchiveManager::GetArchiveTypeFromFileExtension` (Xbox PDB): the type of
/// the last three characters of `name` ([`fn_00af7db0`]).
pub fn archive_manager_get_archive_type_from_file_extension(e: &mut Engine, name: Ptr) -> u32 {
    let length = e.call(STRLEN, &args![name]).u32();
    let extension = name.addr().wrapping_add(length).wrapping_sub(3);
    fn_00af7db0(e, Ptr::new(extension))
}

// Translated from 00af6320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): whether `00af6160(path,
/// types)` (the archive lookup of this unit) answers non-null.
pub fn fn_00af6320(e: &mut Engine, path: Ptr, types: u32) -> bool {
    fn_00af6160(e, path, types) != 0
}

// Translated from 00afb170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words; `this` unused): with
/// `kind` 1 answers the byte `value` as it is, otherwise whether `kind` is
/// nonzero.
pub fn fn_00afb170(_e: &mut Engine, _unused_0: Ptr, value: u8, kind: u32) -> u8 {
    if kind == 1 {
        value
    } else {
        (kind != 0) as u8
    }
}

// Translated from 00afb2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`, two words): whether
/// the file `entry` has data: false when `any` is clear and bit 31 of the
/// entry's word at +0xc differs from bit 3 of `cArchiveFlags`; otherwise
/// whether the word's low 31 bits are nonzero.
pub fn fn_00afb2a0(e: &mut Engine, this: Ptr<Archive>, entry: Ptr, any: bool) -> bool {
    let word = e.mem.u32(entry.addr() + 0xc);
    if !any {
        let entry_bit = word & 0x8000_0000 != 0;
        let archive_bit = e.get(this, Archive::cArchiveFlags) & 8 != 0;
        if entry_bit != archive_bit {
            return false;
        }
    }
    e.mem.u32(entry.addr() + 0xc) & 0x7fff_ffff != 0
}

// Translated from 00afb190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afb190 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// Unnamed in the engine map (thiscall on an `Archive`, unused; two words):
/// while the byte `011acdf9` is set and bit 31 of the `entry`'s word at +8
/// is clear, looks for `"Data\" + name` on disk (`_access`). Found: the low
/// 31 bits of the entry's word at +0xc are cleared and the answer is true.
/// Otherwise (also for a null `name`) bit 31 of the word at +8 is set and
/// the answer is false.
///
/// The exe's "otherwise" also holds a branch clearing that bit instead,
/// behind a constant-true test; it is kept below.
pub fn fn_00afb190(e: &mut Engine, _unused_0: Ptr<Archive>, entry: Ptr, name: Ptr) -> bool {
    if e.global::<u8>(FLAG_011ACDF9) == 0 || e.mem.u32(entry.addr() + 8) & 0x8000_0000 != 0 {
        return false;
    }
    if !name.is_null() {
        let found = e.with_stack(0x104, |e, local| {
            e.call(STRCPY_S, &args![local, 0x104u32, TEXT_DATA_CAPITAL]);
            e.call(STRCAT_S, &args![local, 0x104u32, name]);
            e.call(ACCESS, &args![local, 0u32]).i32() != -1
        });
        if found {
            let word = e.mem.u32(entry.addr() + 0xc);
            e.mem.set_u32(entry.addr() + 0xc, word & 0x8000_0000);
            return true;
        }
    }
    // `MOV EDX,1; TEST EDX,EDX; JZ`: the set branch always runs.
    let mark = true;
    let word = e.mem.u32(entry.addr() + 8);
    if mark {
        e.mem.set_u32(entry.addr() + 8, word | 0x8000_0000);
    } else {
        e.mem.set_u32(entry.addr() + 8, word & 0x7fff_ffff);
    }
    false
}

// Translated from 00afce90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word): compares two 64-bit
/// values (low word first, compared as unsigned high then low words):
/// -1 when `this` is smaller than `other`, 1 when larger, 0 when equal.
pub fn fn_00afce90(e: &mut Engine, this: Ptr, other: Ptr) -> i32 {
    let (low, high) = (e.mem.u32(this.addr()), e.mem.u32(this.addr() + 4));
    let (other_low, other_high) = (e.mem.u32(other.addr()), e.mem.u32(other.addr() + 4));
    if high < other_high || (high == other_high && low < other_low) {
        return -1;
    }
    if high > other_high || (high == other_high && low > other_low) {
        return 1;
    }
    0
}

// Translated from 00afcf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word): compares two name
/// hashes. When the low 7 bits of `this`'s byte 3 are `*` (0x2a) the answer
/// is 0 if `00afcfe0` gives both the same byte, else 1. Otherwise the words
/// at +4 decide (-1 smaller, 1 larger, unsigned); when equal, differing
/// bytes at +2 answer 1, and equal ones compare `00afcfe0` as above.
pub fn fn_00afcf00(e: &mut Engine, this: Ptr, other: Ptr) -> i32 {
    let same_flag = |e: &mut Engine| {
        let first = e.call(HASH_FLAG, &args![this]).u8();
        let second = e.call(HASH_FLAG, &args![other]).u8();
        first == second
    };
    if e.mem.u8(this.addr() + 3) & 0x7f == 0x2a {
        return if same_flag(e) { 0 } else { 1 };
    }
    let word = e.mem.u32(this.addr() + 4);
    let other_word = e.mem.u32(other.addr() + 4);
    if word < other_word {
        return -1;
    }
    if word > other_word {
        return 1;
    }
    if e.mem.u8(this.addr() + 2) != e.mem.u8(other.addr() + 2) {
        return 1;
    }
    if same_flag(e) {
        0
    } else {
        1
    }
}

// Translated from 00afd8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSArchiveHeader` constructor: tag
/// `"BSA\0"` (0x00415342), version 0x68, header size 0x24, everything else
/// 0 (the flags first). Returns `this`.
pub fn fn_00afd8b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base, 0x0041_5342);
    e.mem.set_u32(base + 4, 0x68);
    e.mem.set_u32(base + 8, 0x24);
    for offset in [0x10, 0x14, 0x18, 0x1c] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u16(base + 0x20, 0);
    this
}

// Translated from 00afdc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a `BSArchive`, one word): whether
/// any of its `iDirectories` 16-byte directory entries (`pDirectories`,
/// +0x24) holds `hash` (`00afdbc0` on each, in order).
pub fn fn_00afdc00(e: &mut Engine, this: Ptr, hash: u32) -> bool {
    let mut index = 0u32;
    while index < e.mem.u32(this.addr() + 0x10) {
        let entry = e
            .mem
            .u32(this.addr() + 0x24)
            .wrapping_add(index.wrapping_mul(16));
        if e.call(DIRECTORY_HAS, &args![entry, hash]).bool() {
            return true;
        }
        index += 1;
    }
    false
}

// Translated from 00afa6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`, three words): the
/// file entry for a directory hash and a file hash: the directory index from
/// `00af9c50(directory, &index, name)`, then the file index from
/// `00afa120(index, file, &file_index, name, 1)`; answers the address of
/// that 16-byte file entry (the directory entry's array at +0xc), or null
/// when either lookup fails.
pub fn fn_00afa6e0(
    e: &mut Engine,
    this: Ptr<Archive>,
    directory: Ptr,
    file: Ptr,
    name: Ptr,
) -> u32 {
    e.with_stack(8, |e, locals| {
        let directory_index = locals.addr() + 4;
        let file_index = locals.addr();
        if !fn_00af9c50(e, this, directory, Ptr::new(directory_index), name) {
            return 0;
        }
        let index = e.mem.u32(directory_index);
        if !fn_00afa120(e, this, index, file, Ptr::new(file_index), name, true) {
            return 0;
        }
        let index = e.mem.u32(directory_index);
        let directories = e.get(this, Archive::pDirectories);
        let files = e
            .mem
            .u32(directories.wrapping_add(index << 4).wrapping_add(0xc));
        (e.mem.u32(file_index) << 4).wrapping_add(files)
    })
}

// Translated from 00af9bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`, five words):
/// whether `00af9c50(directory, index_out, name)` finds the directory and
/// then `00afa120(*index_out, file, file_index_out, name, 0)` the file.
pub fn fn_00af9bf0(
    e: &mut Engine,
    this: Ptr<Archive>,
    directory: Ptr,
    file: Ptr,
    index_out: Ptr,
    file_index_out: Ptr,
    name: Ptr,
) -> bool {
    if !fn_00af9c50(e, this, directory, index_out, name) {
        return false;
    }
    let index = e.mem.u32(index_out.addr());
    fn_00afa120(e, this, index, file, file_index_out, name, false)
}

// Translated from 00af9230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`): unless bit 2 of
/// `cArchiveFlags` is set, frees `pDirectoryStringArray` and
/// `pDirectoryStringOffsets` (each when not null) and clears both; then
/// clears bit 4 of the flags.
///
/// The exe's tail holds a branch setting bit 4 instead behind a
/// constant-false test (`XOR EDX,EDX; JZ`); it is kept below.
pub fn fn_00af9230(e: &mut Engine, this: Ptr<Archive>) {
    if e.get(this, Archive::cArchiveFlags) & 4 == 0 {
        let strings = e.get(this, Archive::pDirectoryStringArray);
        if strings != 0 {
            free(e, strings);
        }
        let offsets = e.get(this, Archive::pDirectoryStringOffsets);
        if offsets != 0 {
            free(e, offsets);
        }
        e.set(this, Archive::pDirectoryStringArray, 0);
        e.set(this, Archive::pDirectoryStringOffsets, 0);
    }
    let retained = false;
    let flags = e.get(this, Archive::cArchiveFlags);
    if retained {
        e.set(this, Archive::cArchiveFlags, flags | 0x10);
    } else {
        e.set(this, Archive::cArchiveFlags, flags & 0xef);
    }
}

// Translated from 00af92f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`, one word): unless
/// bit 2 of `cArchiveFlags` is set, frees `pFileNameStringArray` (when not
/// null) and clears it, and with `keep_offsets` clear frees each of the
/// `iDirectories` arrays of `pFileNameStringOffsets` and the array itself
/// (when not null), clearing it; then clears bit 5 of the flags.
///
/// The exe's tail holds a branch setting bit 5 instead behind a
/// constant-false test (`XOR EAX,EAX; JZ`); it is kept below.
pub fn fn_00af92f0(e: &mut Engine, this: Ptr<Archive>, keep_offsets: bool) {
    if e.get(this, Archive::cArchiveFlags) & 4 == 0 {
        let strings = e.get(this, Archive::pFileNameStringArray);
        if strings != 0 {
            free(e, strings);
        }
        e.set(this, Archive::pFileNameStringArray, 0);
        if e.get(this, Archive::pFileNameStringOffsets) != 0 && !keep_offsets {
            let mut index = 0u32;
            while index < e.get(this, Archive::iDirectories) {
                let offsets = e.get(this, Archive::pFileNameStringOffsets);
                let block = e.mem.u32(offsets.wrapping_add(index.wrapping_mul(4)));
                free(e, block);
                index += 1;
            }
            let offsets = e.get(this, Archive::pFileNameStringOffsets);
            free(e, offsets);
            e.set(this, Archive::pFileNameStringOffsets, 0);
        }
    }
    let retained = false;
    let flags = e.get(this, Archive::cArchiveFlags);
    if retained {
        e.set(this, Archive::cArchiveFlags, flags | 0x20);
    } else {
        e.set(this, Archive::cArchiveFlags, flags & 0xdf);
    }
}

// Translated from 00af9410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`): unless bit 2 of
/// `cArchiveFlags` is set, frees each directory entry's file array (+0xc of
/// each of the `iDirectories` 16-byte entries) and `pDirectories` itself
/// (when not null), then clears `pDirectories`.
pub fn fn_00af9410(e: &mut Engine, this: Ptr<Archive>) {
    if e.get(this, Archive::cArchiveFlags) & 4 != 0 {
        return;
    }
    if e.get(this, Archive::pDirectories) != 0 {
        let mut index = 0u32;
        while index < e.get(this, Archive::iDirectories) {
            let directories = e.get(this, Archive::pDirectories);
            let files = e
                .mem
                .u32(directories.wrapping_add(index << 4).wrapping_add(0xc));
            free(e, files);
            index += 1;
        }
        let directories = e.get(this, Archive::pDirectories);
        free(e, directories);
    }
    e.set(this, Archive::pDirectories, 0);
}

// Translated from 00af8e60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af8e60 - the FS:[0] exception frame (the unwind states between the base parts) is not translated
/// Unnamed in the engine map: the `Archive` destructor body. Installs the
/// `Archive` vtables (+0 and +0x158), frees the directory strings
/// ([`fn_00af9230`]), the file names ([`fn_00af92f0`] with 0) and the
/// directories ([`fn_00af9410`]), deletes the critical section, installs
/// the `NiRefObject` vtable at +0x158 and lowers `NiRefObject`'s object count
/// (`011f4400`), and ends with `BSFile::~BSFile`.
pub fn fn_00af8e60(e: &mut Engine, this: Ptr<Archive>) {
    e.mem.set_u32(this.addr(), ARCHIVE_VTABLE);
    e.set(this, Archive::ref_object_vtable, ARCHIVE_REF_OBJECT_VTABLE);
    fn_00af9230(e, this);
    fn_00af92f0(e, this, false);
    fn_00af9410(e, this);
    let section = this.addr() + Archive::ArchiveCriticalSection.off;
    e.call(DELETE_CRITICAL_SECTION, &args![section]);
    e.set(this, Archive::ref_object_vtable, REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![REF_OBJECT_COUNT]);
    e.call(BSFILE_DESTRUCTOR, &args![this]);
}

// Translated from 00afc680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `Archive` vtable +0: the scalar deleting
/// destructor. Runs [`fn_00af8e60`], then with bit 0 of `flags` frees `this`
/// through `00aa1460(this, 0x1d0)` (the Gamebryo allocator's free with the
/// size). Returns `this`.
pub fn fn_00afc680(e: &mut Engine, this: Ptr<Archive>, flags: u32) -> Ptr<Archive> {
    fn_00af8e60(e, this);
    if flags & 1 != 0 {
        e.call(GAMEBRYO_FREE_SIZED, &args![this, 0x1d0u32]);
    }
    this
}

/// `00aa1460` (cdecl: block, size): the Gamebryo allocator's free.
const GAMEBRYO_FREE_SIZED: u32 = 0x00aa_1460;

// Translated from 00afce80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiRefObject` part's vtable +0 (`this` at
/// `Archive` +0x158): moves `this` back to the `Archive` and runs
/// [`fn_00afc680`] (a tail jump). Returns what it returns.
pub fn fn_00afce80(e: &mut Engine, this: u32, flags: u32) -> Ptr<Archive> {
    fn_00afc680(e, Ptr::new(this.wrapping_sub(ARCHIVE_REF_OBJECT)), flags)
}

// Translated from 00afc720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPointer<Archive>` destructor:
/// releases the archive `*this` points to, if any.
pub fn fn_00afc720(e: &mut Engine, this: Ptr) {
    let archive = e.mem.u32(this.addr());
    if archive != 0 {
        release(e, archive);
    }
}

// Translated from 00afd1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word): `NiPointer<Archive>`
/// assignment from a plain pointer: when it changes, releases the old
/// archive, stores the new one and adds a reference to it (each when not
/// null). Returns `this`.
pub fn fn_00afd1b0(e: &mut Engine, this: Ptr, archive: u32) -> Ptr {
    let slot = this.addr();
    if e.mem.u32(slot) != archive {
        let old = e.mem.u32(slot);
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, archive);
        let new = e.mem.u32(slot);
        if new != 0 {
            add_ref(e, new);
        }
    }
    this
}

// Translated from 00afc770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word): `NiPointer<Archive>`
/// assignment from another `NiPointer` (`*other`), as [`fn_00afd1b0`].
/// Returns `this`.
pub fn fn_00afc770(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let slot = this.addr();
    if e.mem.u32(slot) != e.mem.u32(other.addr()) {
        let old = e.mem.u32(slot);
        if old != 0 {
            release(e, old);
        }
        let archive = e.mem.u32(other.addr());
        e.mem.set_u32(slot, archive);
        let new = e.mem.u32(slot);
        if new != 0 {
            add_ref(e, new);
        }
    }
    this
}

// Translated from 00afc880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a `BSSimpleList<NiPointer<Archive>>`
/// head node, one word): adds the archive `*item` at the front. Nothing for
/// a null archive; an empty head just takes it; otherwise a new 8-byte node
/// (memory manager) takes the head's archive (with a reference) and its
/// next, becomes the head's next, and the head takes the new archive. A
/// failed allocation writes through a null node, as the exe does.
pub fn fn_00afc880(e: &mut Engine, this: Ptr, item: Ptr) {
    if e.mem.u32(item.addr()) == 0 {
        return;
    }
    if e.mem.u32(this.addr()) != 0 {
        let node = allocate(e, 8);
        if node != 0 {
            let archive = e.mem.u32(this.addr());
            e.mem.set_u32(node, archive);
            if e.mem.u32(node) != 0 {
                let archive = e.mem.u32(node);
                add_ref(e, archive);
            }
            e.mem.set_u32(node + 4, 0);
        }
        let next = e.mem.u32(this.addr() + 4);
        e.mem.set_u32(node.wrapping_add(4), next);
        e.mem.set_u32(this.addr() + 4, node);
    }
    fn_00afc770(e, this, item);
}

// Translated from 00afcc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a `BSSimpleList<NiPointer<Archive>>`
/// head node, one word): adds the archive `*item` at the end. Nothing for a
/// null archive; an empty last node just takes it; otherwise a new 8-byte
/// node holding it (with a reference) is linked after the last one (null
/// when the allocation fails).
pub fn fn_00afcc10(e: &mut Engine, this: Ptr, item: Ptr) {
    if e.mem.u32(item.addr()) == 0 {
        return;
    }
    let mut last = this.addr();
    while e.mem.u32(last + 4) != 0 {
        last = e.mem.u32(last + 4);
    }
    if e.mem.u32(last) == 0 {
        fn_00afc770(e, Ptr::new(last), item);
        return;
    }
    let node = allocate(e, 8);
    if node != 0 {
        let archive = e.mem.u32(item.addr());
        e.mem.set_u32(node, archive);
        if e.mem.u32(node) != 0 {
            let archive = e.mem.u32(node);
            add_ref(e, archive);
        }
        e.mem.set_u32(node + 4, 0);
    }
    e.mem.set_u32(last + 4, node);
}

// Translated from 00afcaf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afcaf0 - the FS:[0] exception frame (the unwind state of the NiPointer temporary) is not translated
/// Unnamed in the engine map (thiscall on a `BSSimpleList<NiPointer<Archive>>`
/// head node): empties the list. Each following node is unlinked (its next
/// cleared first), destroyed ([`fn_00afc7f0`]) and freed; then the head's
/// archive is assigned a null `NiPointer` temporary ([`fn_00afc770`]), which
/// is then destroyed.
///
/// The exe also tests the temporary (just set to null) before taking a
/// reference to it; that dead test is kept.
pub fn fn_00afcaf0(e: &mut Engine, this: Ptr) {
    while e.mem.u32(this.addr() + 4) != 0 {
        let node = e.mem.u32(this.addr() + 4);
        let after = e.mem.u32(node + 4);
        e.mem.set_u32(node + 4, 0);
        let node = e.mem.u32(this.addr() + 4);
        if node != 0 {
            fn_00afc7f0(e, Ptr::new(node));
            free(e, node);
        }
        e.mem.set_u32(this.addr() + 4, after);
    }
    e.with_stack(4, |e, temporary| {
        e.mem.set_u32(temporary.addr(), 0);
        let held = e.mem.u32(temporary.addr());
        if held != 0 {
            add_ref(e, held);
        }
        fn_00afc770(e, this, temporary);
        let held = e.mem.u32(temporary.addr());
        if held != 0 {
            release(e, held);
        }
    });
}

// Translated from 00afc7f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afc7f0 - the FS:[0] exception frame (the unwind state around RemoveAll) is not translated
/// Unnamed in the engine map: the `BSSimpleList<NiPointer<Archive>>` node
/// destructor: empties the list from this node ([`fn_00afcaf0`]), then
/// releases the node's archive, if any.
pub fn fn_00afc7f0(e: &mut Engine, this: Ptr) {
    fn_00afcaf0(e, this);
    let archive = e.mem.u32(this.addr());
    if archive != 0 {
        release(e, archive);
    }
}

// Translated from 00af59f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): runs [`fn_00af8f10`] on every
/// archive of the list `011f8160`, front to back, when there is a list.
/// Unless the state at +0x5c of `00aff100`'s object is 2 or 3 already, a
/// state 1 is raised to 2 for the walk and a state 2 lowered back to 1
/// after it.
pub fn fn_00af59f0(e: &mut Engine) {
    if e.global::<u32>(ARCHIVE_LIST) == 0 {
        return;
    }
    let owner = e.call(STATE_OWNER, &args![]).u32();
    let state = e.mem.u32(owner + STATE_OFFSET);
    let busy = state == 2 || state == 3;
    if !busy {
        let owner = e.call(STATE_OWNER, &args![]).u32();
        if e.mem.u32(owner + STATE_OFFSET) == 1 {
            e.mem.set_u32(owner + STATE_OFFSET, 2);
        }
    }
    let mut node = e.global::<u32>(ARCHIVE_LIST);
    while node != 0 {
        let archive = e.mem.u32(node);
        fn_00af8f10(e, Ptr::new(archive));
        node = e.mem.u32(node + 4);
    }
    if !busy {
        let owner = e.call(STATE_OWNER, &args![]).u32();
        if e.mem.u32(owner + STATE_OFFSET) == 2 {
            e.mem.set_u32(owner + STATE_OFFSET, 1);
        }
    }
}

/// CRT helpers of the hash checks: `_splitpath_s` (`00ec7f1e`, nine words),
/// `_strupr_s` (`00ecbc5b`, two words), `strtok_s` (`00ecc5f8`, three
/// words); the `"."` separators they pass and the warning `005b5e40`
/// prints (cdecl, a format and its words).
const SPLITPATH_S: u32 = 0x00ec_7f1e;
const STRUPR_S: u32 = 0x00ec_bc5b;
const STRTOK_S: u32 = 0x00ec_c5f8;
const TEXT_DOT_FIRST: u32 = 0x010a_4548;
const TEXT_DOT_SECOND: u32 = 0x010a_454c;
const WARNING: u32 = 0x005b_5e40;
/// `"DEFAULT: HashMap Collision between %s and %s"`, the copy `00af9c50`
/// passes and the one `00afa120` passes.
const TEXT_DIRECTORY_COLLISION: u32 = 0x010a_4518;
const TEXT_FILE_COLLISION: u32 = 0x010a_4550;
/// `00af94c0` (thiscall on an `Archive`, one word: the directory index):
/// the directory's name.
const DIRECTORY_NAME: u32 = 0x00af_94c0;

/// The binary search `00af9c50` and `00afa120` share (inlined in both):
/// halves between 0 and `count` over the 16-byte entries at `entries`,
/// comparing `hash` with each ([`fn_00afce90`]), until the step is 0 or the
/// hashes match. Answers the last index tried and the last comparison.
fn search(e: &mut Engine, hash: Ptr, entries: u32, count: u32) -> (u32, i32) {
    let mut low = 0u32;
    let mut high = count;
    let mut middle = 0u32;
    let mut step = 1u32;
    let mut compared = 0i32;
    while step != 0 {
        step = high.wrapping_sub(low) >> 1;
        middle = low.wrapping_add(step);
        let entry = entries.wrapping_add(middle << 4);
        compared = fn_00afce90(e, hash, Ptr::new(entry));
        if compared > 0 {
            low = middle;
        } else if compared < 0 {
            high = middle;
        } else {
            break;
        }
    }
    (middle, compared)
}

/// Copies `text` into a new block of `strlen + 1` bytes (the length asked
/// again for the copy, as the exe does).
fn duplicate_measured(e: &mut Engine, block: u32, text: u32) {
    let size = e.call(STRLEN, &args![text]).u32().wrapping_add(1);
    e.call(STRCPY_S, &args![block, size, text]);
}

/// Upper-cases `text` in place (`_strupr_s(text, strlen + 1)`).
fn upper(e: &mut Engine, text: u32) {
    let size = e.call(STRLEN, &args![text]).u32().wrapping_add(1);
    e.call(STRUPR_S, &args![text, size]);
}

// Translated from 00af9c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af9c50 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// Unnamed in the engine map (thiscall on an `Archive`, three words): finds
/// the directory whose 8-byte hash equals `*hash` and stores its index in
/// `*index_out`. False at once when bit 0 of `cArchiveFlags` is set. The
/// last found directory (`iLastDirectoryIndex`) is tried first; otherwise
/// `iLastFileIndex` becomes -1 and the directories are searched by halving
/// (a find there becomes the last directory). After a search find, with a
/// `name`, the byte `011f8154` set and bit 0 of `oFlags`, the directory part
/// of `name` (`_splitpath_s`, less its last character) is compared
/// case-insensitively (both upper-cased copies, `_stricmp`) with the
/// archive's name for that directory (`00af94c0`); a mismatch prints the
/// collision warning and makes the answer false.
pub fn fn_00af9c50(
    e: &mut Engine,
    this: Ptr<Archive>,
    hash: Ptr,
    index_out: Ptr,
    name: Ptr,
) -> bool {
    if e.get(this, Archive::cArchiveFlags) & 1 != 0 {
        return false;
    }
    let mut found = false;
    let last = e.get(this, Archive::iLastDirectoryIndex);
    if last < e.get(this, Archive::iDirectories) {
        let entry = (last << 4).wrapping_add(e.get(this, Archive::pDirectories));
        if fn_00afce90(e, hash, Ptr::new(entry)) == 0 {
            e.mem.set_u32(index_out.addr(), last);
            found = true;
        }
    }
    if found {
        return true;
    }
    e.set(this, Archive::iLastFileIndex, u32::MAX);
    let count = e.get(this, Archive::iDirectories);
    if count == 0 {
        return false;
    }
    let entries = e.get(this, Archive::pDirectories);
    let (index, compared) = search(e, hash, entries, count);
    if compared != 0 {
        return false;
    }
    e.mem.set_u32(index_out.addr(), index);
    e.set(this, Archive::iLastDirectoryIndex, index);
    found = true;
    if name.is_null()
        || e.global::<u8>(FLAG_SET_BY_00AF4460) == 0
        || e.get(this, Archive::oFlags) & 1 == 0
    {
        return found;
    }
    e.with_stack(0x104, |e, directory| {
        let directory = directory.addr();
        e.call(
            SPLITPATH_S,
            &args![name, 0u32, 0u32, directory, 0x104u32, 0u32, 0u32, 0u32, 0u32],
        );
        let stored = e.call(DIRECTORY_NAME, &args![this, index]).u32();
        let size = e.call(STRLEN, &args![stored]).u32().wrapping_add(1);
        let first = allocate(e, size);
        let size = e.call(STRLEN, &args![directory]).u32().wrapping_add(1);
        let second = allocate(e, size);
        duplicate_measured(e, first, stored);
        duplicate_measured(e, second, directory);
        let length = e.call(STRLEN, &args![second]).u32();
        e.mem.set_u8(second.wrapping_add(length).wrapping_sub(1), 0);
        upper(e, first);
        upper(e, second);
        if e.call(STRICMP, &args![first, second]).i32() != 0 {
            e.call(WARNING, &args![TEXT_DIRECTORY_COLLISION, stored, directory]);
            found = false;
        }
        free(e, first);
        free(e, second);
    });
    found
}

// Translated from 00afa120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afa120 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// Unnamed in the engine map (thiscall on an `Archive`, five words): finds
/// the file whose 8-byte hash equals `*hash` in directory `directory` (its
/// entry's file count at +8, files at +0xc) and stores its index in
/// `*index_out`. False at once when bit 0 of `cArchiveFlags` is set. The
/// last found file (`iLastFileIndex`) is tried first: it counts when it has
/// data ([`fn_00afb2a0`] with `any`) and the hash matches, unless
/// [`fn_00afb190`] finds it on disk, which answers false at once. Otherwise
/// the files are searched by halving; a match that has data and is not on
/// disk is the find (it becomes the last file). After a search find, with
/// a `name`, the byte `011f8154` set and bit 1 of `oFlags`, the file name
/// part of `name` (`_splitpath_s`) and the archive's name for the file
/// ([`fn_00af96d0`]), each cut at its first `.` (`strtok_s`) and
/// upper-cased, are compared with `_stricmp`; a mismatch prints the
/// collision warning and makes the answer false.
pub fn fn_00afa120(
    e: &mut Engine,
    this: Ptr<Archive>,
    directory: u32,
    hash: Ptr,
    index_out: Ptr,
    name: Ptr,
    any: bool,
) -> bool {
    if e.get(this, Archive::cArchiveFlags) & 1 != 0 {
        return false;
    }
    let mut found = false;
    let entry = (directory << 4).wrapping_add(e.get(this, Archive::pDirectories));
    let last = e.get(this, Archive::iLastFileIndex);
    if last < e.mem.u32(entry + 8) {
        let file = (last << 4).wrapping_add(e.mem.u32(entry + 0xc));
        if fn_00afb2a0(e, this, Ptr::new(file), any) && fn_00afce90(e, hash, Ptr::new(file)) == 0 {
            if fn_00afb190(e, this, Ptr::new(file), name) {
                return false;
            }
            e.mem.set_u32(index_out.addr(), last);
            found = true;
        }
    }
    if found {
        return true;
    }
    let count = e.mem.u32(entry + 8);
    if count == 0 {
        return false;
    }
    let files = e.mem.u32(entry + 0xc);
    let (index, compared) = search(e, hash, files, count);
    if compared != 0 {
        return false;
    }
    let file = Ptr::new(files.wrapping_add(index << 4));
    if !fn_00afb2a0(e, this, file, any) || fn_00afb190(e, this, file, name) {
        return false;
    }
    e.mem.set_u32(index_out.addr(), index);
    e.set(this, Archive::iLastFileIndex, index);
    found = true;
    if name.is_null()
        || e.global::<u8>(FLAG_SET_BY_00AF4460) == 0
        || e.get(this, Archive::oFlags) & 2 == 0
    {
        return found;
    }
    e.with_stack(0x108, |e, frame| {
        let file_name = frame.addr();
        let context = frame.addr() + 0x104;
        e.call(
            SPLITPATH_S,
            &args![name, 0u32, 0u32, 0u32, 0u32, file_name, 0x104u32, 0u32, 0u32],
        );
        let stored = fn_00af96d0(e, this, directory, index);
        let size = e.call(STRLEN, &args![stored]).u32().wrapping_add(1);
        let first = allocate(e, size);
        let size = e.call(STRLEN, &args![file_name]).u32().wrapping_add(1);
        let second = allocate(e, size);
        duplicate_measured(e, first, stored);
        duplicate_measured(e, second, file_name);
        e.call(STRTOK_S, &args![first, TEXT_DOT_FIRST, context]);
        e.call(STRTOK_S, &args![second, TEXT_DOT_SECOND, context]);
        upper(e, first);
        upper(e, second);
        if e.call(STRICMP, &args![first, second]).i32() != 0 {
            e.call(WARNING, &args![TEXT_FILE_COLLISION, stored, file_name]);
            found = false;
        }
        free(e, first);
        free(e, second);
    });
    found
}

// Translated from 00afb5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `ArchiveFile` vtable +0x20 (`Open`, two
/// stack words, unused): true at once when there is a buffer. Otherwise
/// clears `m_uiPos`, `m_uiBufferReadSize` and `m_pBuffer`, allocates a
/// buffer of `m_uiBufferAllocSize` bytes when that is nonzero, marks the
/// file good and answers `m_bGood`.
pub fn fn_00afb5f0(e: &mut Engine, this: Ptr<ArchiveFile>, _unused_1: u8, _unused_2: u8) -> bool {
    let file = this.cast::<NiFile>();
    if e.get(file, NiFile::m_pBuffer) != 0 {
        return true;
    }
    e.set(file, NiFile::m_uiPos, 0);
    e.set(file, NiFile::m_uiBufferReadSize, 0);
    e.set(file, NiFile::m_pBuffer, 0);
    if e.get(file, NiFile::m_uiBufferAllocSize) != 0 && e.get(file, NiFile::m_pBuffer) == 0 {
        let size = e.get(file, NiFile::m_uiBufferAllocSize);
        let buffer = allocate(e, size);
        e.set(file, NiFile::m_pBuffer, buffer);
    }
    e.set(file, NiFile::m_bGood, 1);
    e.get(file, NiFile::m_bGood) != 0
}

// Translated from 00afc060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `CompressedArchiveFile` vtable +0x20: the
/// `ArchiveFile` open ([`fn_00afb5f0`]) with the same two bytes.
pub fn fn_00afc060(e: &mut Engine, this: Ptr<ArchiveFile>, first: u8, second: u8) -> bool {
    fn_00afb5f0(e, this, first, second)
}

// Translated from 00afb670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `ArchiveFile` vtable +0x14 (the seek, two
/// words): moves `iTrueFilePos` within `[0, iFileSize]`: origin 1 by
/// `offset` (clamped above), 2 to `iFileSize - |offset|` (the magnitude
/// clamped to the size), anything else to `offset` (clamped). `m_uiPos`
/// moves by the same amount, and a position at or past `m_uiBufferReadSize`
/// flushes the buffer (`00aa15a0`).
pub fn fn_00afb670(e: &mut Engine, this: Ptr<ArchiveFile>, offset: u32, origin: u32) {
    let bs_file = this.cast::<BSFile>();
    let file = this.cast::<NiFile>();
    let before = e.get(bs_file, BSFile::iTrueFilePos);
    let size = e.get(bs_file, BSFile::iFileSize);
    match origin {
        1 => {
            let position = e.get(bs_file, BSFile::iTrueFilePos).wrapping_add(offset);
            e.set(bs_file, BSFile::iTrueFilePos, position);
            if e.get(bs_file, BSFile::iTrueFilePos) > e.get(bs_file, BSFile::iFileSize) {
                let size = e.get(bs_file, BSFile::iFileSize);
                e.set(bs_file, BSFile::iTrueFilePos, size);
            }
        }
        2 => {
            let mut distance = offset;
            if (distance as i32) < 0 {
                distance = distance.wrapping_neg();
            }
            if distance > size {
                distance = e.get(bs_file, BSFile::iFileSize);
            }
            let size = e.get(bs_file, BSFile::iFileSize);
            e.set(bs_file, BSFile::iTrueFilePos, size.wrapping_sub(distance));
        }
        _ => {
            let target = if offset > size {
                e.get(bs_file, BSFile::iFileSize)
            } else {
                offset
            };
            e.set(bs_file, BSFile::iTrueFilePos, target);
        }
    }
    let moved = e.get(bs_file, BSFile::iTrueFilePos).wrapping_sub(before);
    let position = moved.wrapping_add(e.get(file, NiFile::m_uiPos));
    e.set(file, NiFile::m_uiPos, position);
    if e.get(file, NiFile::m_uiPos) >= e.get(file, NiFile::m_uiBufferReadSize) {
        e.call(NI_FILE_FLUSH, &args![this]);
    }
}

// Translated from 00afb780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `ArchiveFile` vtable +0x28 (`GetSize`):
/// `iFileSize`.
pub fn fn_00afb780(e: &mut Engine, this: Ptr<ArchiveFile>) -> u32 {
    e.get(this.cast::<BSFile>(), BSFile::iFileSize)
}

// Translated from 00afc0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `CompressedArchiveFile` vtable +0x28
/// (`GetSize`): `iUncompressedFileSize`.
pub fn fn_00afc0a0(e: &mut Engine, this: Ptr<CompressedArchiveFile>) -> u32 {
    e.get(this, CompressedArchiveFile::iUncompressedFileSize)
}

// Translated from 00afc0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `CompressedArchiveFile` vtable +0x1c
/// (`GetFileSize`): `iUncompressedFileSize` (the same body as
/// [`fn_00afc0a0`]).
pub fn fn_00afc0c0(e: &mut Engine, this: Ptr<CompressedArchiveFile>) -> u32 {
    e.get(this, CompressedArchiveFile::iUncompressedFileSize)
}

// Translated from 00afc080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `CompressedArchiveFile` vtable +0x14 (the
/// seek, two words): reads `offset` bytes into nowhere through vtable +0x44
/// (`ReadF(null, offset)`); `origin` is not read.
pub fn fn_00afc080(e: &mut Engine, this: Ptr<CompressedArchiveFile>, offset: u32, _unused_2: u32) {
    e.vcall(this.addr(), 0x44, &args![0u32, offset]);
}

// Translated from 00afbbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `ArchiveFile` vtable +0x44 (`ReadF`, two
/// words): reads at most what is left of the file (`iFileSize -
/// iTrueFilePos`) through `00b01800`, adds what was read to `iTrueFilePos`
/// and answers it.
pub fn fn_00afbbf0(e: &mut Engine, this: Ptr<ArchiveFile>, buffer: Ptr, size: u32) -> u32 {
    let bs_file = this.cast::<BSFile>();
    let mut size = size;
    let file_size = e.get(bs_file, BSFile::iFileSize);
    if size.wrapping_add(e.get(bs_file, BSFile::iTrueFilePos)) > file_size {
        let file_size = e.get(bs_file, BSFile::iFileSize);
        size = file_size.wrapping_sub(e.get(bs_file, BSFile::iTrueFilePos));
    }
    let read = e.call(ARCHIVE_FILE_READ, &args![this, buffer, size]).u32();
    let position = e.get(bs_file, BSFile::iTrueFilePos).wrapping_add(read);
    e.set(bs_file, BSFile::iTrueFilePos, position);
    read
}

// Translated from 00afbf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): the `ArchiveFile`
/// `m_pfnRead`: [`fn_00afbbf0`] on `stream`.
pub fn fn_00afbf80(e: &mut Engine, stream: Ptr<ArchiveFile>, buffer: Ptr, size: u32) -> u32 {
    fn_00afbbf0(e, stream, buffer, size)
}

// Translated from 00afbfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `ArchiveFile` vtable +0x10 (`SetEndianSwap`):
/// with `swap` set the `BSFile` one (`00aff990`); otherwise installs
/// `00afbf80` as `m_pfnRead` and the plain `00aff9f0` as `m_pfnWrite`.
pub fn fn_00afbfa0(e: &mut Engine, this: Ptr<ArchiveFile>, swap: bool) {
    if swap {
        e.call(BSFILE_SET_ENDIAN_SWAP, &args![this, swap]);
    } else {
        e.mem.set_u32(this.addr() + 8, 0x00af_bf80);
        e.mem.set_u32(this.addr() + 0xc, BSFILE_PLAIN_WRITE);
    }
}

// Translated from 00afc620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `CompressedArchiveFile` vtable +0x10
/// (`SetEndianSwap`): as [`fn_00afbfa0`] with `00afc660` as `m_pfnRead`.
pub fn fn_00afc620(e: &mut Engine, this: Ptr<CompressedArchiveFile>, swap: bool) {
    if swap {
        e.call(BSFILE_SET_ENDIAN_SWAP, &args![this, swap]);
    } else {
        e.mem.set_u32(this.addr() + 8, 0x00af_c660);
        e.mem.set_u32(this.addr() + 0xc, BSFILE_PLAIN_WRITE);
    }
}

// Translated from 00afc660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): the
/// `CompressedArchiveFile` `m_pfnRead`: [`fn_00afc0e0`] on `stream`.
pub fn fn_00afc660(
    e: &mut Engine,
    stream: Ptr<CompressedArchiveFile>,
    buffer: Ptr,
    size: u32,
) -> u32 {
    fn_00afc0e0(e, stream, buffer, size)
}

// Translated from 00afb520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afb520 - the FS:[0] exception frame (the unwind state of spArchive) is not translated
/// Unnamed in the engine map: the `ArchiveFile` destructor body. Installs
/// the `ArchiveFile` vtable, frees the buffer (when there is one) and
/// clears it, marks the file not good, releases `spArchive` and ends with
/// `BSFile::~BSFile`.
pub fn fn_00afb520(e: &mut Engine, this: Ptr<ArchiveFile>) {
    let file = this.cast::<NiFile>();
    e.mem.set_u32(this.addr(), ARCHIVE_FILE_VTABLE);
    let buffer = e.get(file, NiFile::m_pBuffer);
    if buffer != 0 {
        free(e, buffer);
    }
    e.set(file, NiFile::m_pBuffer, 0);
    e.set(file, NiFile::m_bGood, 0);
    let archive = e.get(this, ArchiveFile::spArchive);
    if archive != 0 {
        release(e, archive);
    }
    e.call(BSFILE_DESTRUCTOR, &args![this]);
}

// Translated from 00afc6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `ArchiveFile` vtable +0: the scalar deleting
/// destructor ([`fn_00afb520`], then with bit 0 of `flags` the memory
/// manager frees `this`). Returns `this`.
pub fn fn_00afc6c0(e: &mut Engine, this: Ptr<ArchiveFile>, flags: u32) -> Ptr<ArchiveFile> {
    fn_00afb520(e, this);
    if flags & 1 != 0 {
        free(e, this.addr());
    }
    this
}

// Translated from 00afbfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `CompressedArchiveFile` destructor body.
/// Installs its vtable; ends the zlib stream (`00b45db0`) and frees it, and
/// frees the uncompressed buffer (each when there is one); then the
/// `ArchiveFile` destructor ([`fn_00afb520`]).
pub fn fn_00afbfe0(e: &mut Engine, this: Ptr<CompressedArchiveFile>) {
    e.mem.set_u32(this.addr(), COMPRESSED_ARCHIVE_FILE_VTABLE);
    let stream = e.get(this, CompressedArchiveFile::pZStream);
    if stream != 0 {
        e.call(INFLATE_END, &args![stream]);
        free(e, stream);
    }
    let buffer = e.get(this, CompressedArchiveFile::pUncompressedBuffer);
    if buffer != 0 {
        free(e, buffer);
    }
    fn_00afb520(e, this.cast());
}

// Translated from 00afc6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `CompressedArchiveFile` vtable +0: the scalar
/// deleting destructor ([`fn_00afbfe0`], then with bit 0 of `flags` the
/// memory manager frees `this`). Returns `this`.
pub fn fn_00afc6f0(
    e: &mut Engine,
    this: Ptr<CompressedArchiveFile>,
    flags: u32,
) -> Ptr<CompressedArchiveFile> {
    fn_00afbfe0(e, this);
    if flags & 1 != 0 {
        free(e, this.addr());
    }
    this
}

/// The archive flag (`oFlags` bit 8) under which each file's data starts
/// with its name (a length byte and the characters).
const NAMES_IN_DATA: u32 = 0x100;

// Translated from 00afb300 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afb300 - the FS:[0] exception frame (the unwind states of the base and spArchive) is not translated
/// Unnamed in the engine map: the `ArchiveFile` constructor (thiscall, five
/// words): the `BSFile` base (`00aff180`), the `ArchiveFile` vtable, a null
/// `spArchive` (the exe tests it for a reference anyway), the name copied
/// into `pFileName` (when given), no buffer, `spArchive = archive`
/// ([`fn_00afd1b0`]), `iArchiveOffset = offset`, `iFileSize = size`, the
/// plain read/write pair ([`fn_00afbfa0`] with 0), a buffer of `size` bytes
/// capped at `011f8164` (no cap when `whole` is -1), and the open
/// ([`fn_00afb5f0`]). When `whole` is -1 or the archive has names in its
/// data (`oFlags` bit 8), the first buffer is read at once ([`fn_00afb7a0`])
/// and with names in the data the name (a length byte, then the
/// characters) is copied into `pFileName` and skipped (`m_uiPos` and
/// `iTrueFilePos`). Returns `this`.
pub fn fn_00afb300(
    e: &mut Engine,
    this: Ptr<ArchiveFile>,
    name: Ptr,
    archive: u32,
    offset: u32,
    size: u32,
    whole: u32,
) -> Ptr<ArchiveFile> {
    let file = this.cast::<NiFile>();
    let bs_file = this.cast::<BSFile>();
    e.call(BSFILE_BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), ARCHIVE_FILE_VTABLE);
    e.set(this, ArchiveFile::spArchive, 0);
    let held = e.get(this, ArchiveFile::spArchive);
    if held != 0 {
        add_ref(e, held);
    }
    let file_name = this.addr() + BSFile::pFileName.off;
    if !name.is_null() {
        e.call(STRCPY_S, &args![file_name, 0x104u32, name]);
    }
    e.set(file, NiFile::m_pBuffer, 0);
    let pointer = this.addr() + ArchiveFile::spArchive.off;
    fn_00afd1b0(e, Ptr::new(pointer), archive);
    e.set(this, ArchiveFile::iArchiveOffset, offset);
    e.set(bs_file, BSFile::iFileSize, size);
    fn_00afbfa0(e, this, false);
    let cap = e.global::<u32>(ARCHIVE_FILE_BUFFER_SIZE);
    let buffer_size = if size > cap && whole != u32::MAX {
        e.global::<u32>(ARCHIVE_FILE_BUFFER_SIZE)
    } else {
        size
    };
    e.set(file, NiFile::m_uiBufferAllocSize, buffer_size);
    fn_00afb5f0(e, this, 0, 0);
    let names_in_data = |e: &mut Engine| {
        let archive = e.get(this, ArchiveFile::spArchive);
        e.mem.u32(archive + Archive::oFlags.off) & NAMES_IN_DATA != 0
    };
    if whole == u32::MAX || names_in_data(e) {
        let buffer = e.get(file, NiFile::m_pBuffer);
        let alloc_size = e.get(file, NiFile::m_uiBufferAllocSize);
        let read = fn_00afb7a0(e, this, Ptr::new(buffer), alloc_size, 0);
        e.set(file, NiFile::m_uiBufferReadSize, read);
        if names_in_data(e) {
            let buffer = e.get(file, NiFile::m_pBuffer);
            let position = e.get(file, NiFile::m_uiPos);
            let length = u32::from(e.mem.u8(buffer.wrapping_add(position)));
            let buffer = e.get(file, NiFile::m_pBuffer);
            let position = e.get(file, NiFile::m_uiPos);
            let source = buffer.wrapping_add(position).wrapping_add(1);
            e.call(MEMCPY, &args![file_name, source, length]);
            e.mem.set_u8(file_name.wrapping_add(length), 0);
            let position = e.get(file, NiFile::m_uiPos);
            e.set(
                file,
                NiFile::m_uiPos,
                length.wrapping_add(position).wrapping_add(1),
            );
            e.set(bs_file, BSFile::iTrueFilePos, length.wrapping_add(1));
        }
    }
    this
}

/// `inflateInit_`'s stream size argument and the stream block's size.
const Z_STREAM_SIZE: u32 = 0x38;
const Z_STREAM_BLOCK_SIZE: u32 = 0x1be0;

// Translated from 00afc430 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afc430 - the FS:[0] exception frame (the unwind state of the ArchiveFile base) is not translated
/// Unnamed in the engine map: the `CompressedArchiveFile` constructor
/// (thiscall, five words, as [`fn_00afb300`]'s). After the `ArchiveFile`
/// part it installs its vtable and read pair ([`fn_00afc620`] with 0) and
/// reads the 4-byte uncompressed size into `iUncompressedFileSize`
/// ([`fn_00afbbf0`]). The zlib stream is a 0x1be0-byte block with its words
/// at +0, +4, +0x20, +0x24, +0x28 cleared and +0x1c pointing just past the
/// 0x38-byte stream; `inflateInit_(stream, "1.2.1", 0x38)`. On failure the
/// stream is ended (`00b45db0`), freed and cleared; on success the
/// uncompressed buffer is `min(iUncompressedFileSize, 011f8164)` bytes and
/// its position and read size are 0. Returns `this`.
pub fn fn_00afc430(
    e: &mut Engine,
    this: Ptr<CompressedArchiveFile>,
    name: Ptr,
    archive: u32,
    offset: u32,
    size: u32,
    whole: u32,
) -> Ptr<CompressedArchiveFile> {
    fn_00afb300(e, this.cast(), name, archive, offset, size, whole);
    e.mem.set_u32(this.addr(), COMPRESSED_ARCHIVE_FILE_VTABLE);
    fn_00afc620(e, this, false);
    let size_field = this.addr() + CompressedArchiveFile::iUncompressedFileSize.off;
    fn_00afbbf0(e, this.cast(), Ptr::new(size_field), 4);
    let block = allocate(e, Z_STREAM_BLOCK_SIZE);
    e.set(this, CompressedArchiveFile::pZStream, block);
    for offset in [0x20, 0x24, 0x28, 4, 0] {
        let stream = e.get(this, CompressedArchiveFile::pZStream);
        e.mem.set_u32(stream + offset, 0);
    }
    let stream = e.get(this, CompressedArchiveFile::pZStream);
    e.mem
        .set_u32(stream + 0x1c, block.wrapping_add(Z_STREAM_SIZE));
    let stream = e.get(this, CompressedArchiveFile::pZStream);
    let failed = e
        .call(
            INFLATE_INIT,
            &args![stream, TEXT_ZLIB_VERSION, Z_STREAM_SIZE],
        )
        .u32();
    if failed != 0 {
        let stream = e.get(this, CompressedArchiveFile::pZStream);
        e.call(INFLATE_END, &args![stream]);
        let stream = e.get(this, CompressedArchiveFile::pZStream);
        free(e, stream);
        e.set(this, CompressedArchiveFile::pZStream, 0);
        return this;
    }
    let uncompressed = e.get(this, CompressedArchiveFile::iUncompressedFileSize);
    let cap = e.global::<u32>(ARCHIVE_FILE_BUFFER_SIZE);
    let buffer_size = if uncompressed < cap {
        uncompressed
    } else {
        cap
    };
    e.set(
        this,
        CompressedArchiveFile::iUncompressedBufferSize,
        buffer_size,
    );
    let buffer_size = e.get(this, CompressedArchiveFile::iUncompressedBufferSize);
    let buffer = allocate(e, buffer_size);
    e.set(this, CompressedArchiveFile::pUncompressedBuffer, buffer);
    e.set(this, CompressedArchiveFile::iUncompressedBufferPos, 0);
    e.set(this, CompressedArchiveFile::iUncompressedBufferReadSize, 0);
    this
}

// Translated from 00afb7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `ArchiveFile`, three words):
/// reads `count` bytes of this file's data, at `iArchiveOffset +
/// iTrueFilePos + skip` in the archive, into `buffer`, under the archive's
/// critical section. Calls the archive's vtable +0x18 first (its answer is
/// not used). What the archive's own `NiFile` buffer already holds of that
/// range (its buffer covers the `m_uiBufferReadSize` bytes before
/// `m_uiCurrentFilePos`, compared signed) is copied first; when that is all
/// of it the count is answered. Otherwise the archive's buffer is dropped,
/// its file seeked to the position (relative, origin `010a2484`, when it is
/// not there already; its `iTrueFilePos` then holds the position), the
/// count is clamped to what is left of this file, and when the position is
/// inside the archive (`< iFileSize`) a short read goes through the
/// archive's buffer (`00aa1570` into it, its read size and position both
/// set to what was read, then copied out) and a long one straight into
/// `buffer`. The archive's `iTrueFilePos` ends at the position plus what was
/// read. Answers the bytes delivered.
pub fn fn_00afb7a0(
    e: &mut Engine,
    this: Ptr<ArchiveFile>,
    buffer: Ptr,
    count: u32,
    skip: u32,
) -> u32 {
    let bs_file = this.cast::<BSFile>();
    let mut destination = buffer.addr();
    let mut count = count;
    let mut delivered_first = 0u32;
    let mut position = e
        .get(this, ArchiveFile::iArchiveOffset)
        .wrapping_add(e.get(bs_file, BSFile::iTrueFilePos))
        .wrapping_add(skip);
    let archive = e.get(this, ArchiveFile::spArchive);
    e.vcall(archive, ARCHIVE_SLOT_18, &args![]);
    let section = |e: &mut Engine, import: u32| {
        let archive = e.get(this, ArchiveFile::spArchive);
        if archive != 0 {
            let section = archive + Archive::ArchiveCriticalSection.off;
            e.call(import, &args![section]);
        }
    };
    section(e, ENTER_CRITICAL_SECTION);
    let archive_file = |e: &Engine| Ptr::<NiFile>::new(e.get(this, ArchiveFile::spArchive));
    let file_position = e.get(archive_file(e), NiFile::m_uiCurrentFilePos);
    let buffered = e.get(archive_file(e), NiFile::m_uiBufferReadSize);
    let start = file_position.wrapping_sub(buffered);
    if (start as i32) <= (position as i32) && (file_position as i32) > (position as i32) {
        let skip_in_buffer = position.wrapping_sub(start);
        let read_size = e.get(archive_file(e), NiFile::m_uiBufferReadSize);
        let mut available = read_size.wrapping_sub(skip_in_buffer);
        if (available as i32) > (count as i32) {
            available = count;
        }
        let own = archive_file(e);
        let source = e
            .call(NI_FILE_GET_BUFFER, &args![own])
            .u32()
            .wrapping_add(skip_in_buffer);
        e.call(MEMCPY, &args![destination, source, available]);
        destination = destination.wrapping_add(available);
        if count == available {
            section(e, LEAVE_CRITICAL_SECTION);
            return count;
        }
        count = count.wrapping_sub(available);
        position = position.wrapping_add(available);
        delivered_first = available;
    }
    let own = archive_file(e);
    e.set(own, NiFile::m_uiPos, 0);
    e.set(own, NiFile::m_uiBufferReadSize, 0);
    let distance = position.wrapping_sub(file_position);
    if distance != 0 {
        let origin = e.global::<u32>(NI_FILE_SEEK_CURRENT);
        e.call(NI_FILE_SEEK, &args![own, distance, origin]);
        let own_bs_file = own.cast::<BSFile>();
        e.set(own_bs_file, BSFile::iTrueFilePos, position);
    }
    let file_end = e.get(bs_file, BSFile::iFileSize);
    if e.get(bs_file, BSFile::iTrueFilePos)
        .wrapping_add(count)
        .wrapping_add(skip)
        > file_end
    {
        let used = e.get(bs_file, BSFile::iTrueFilePos).wrapping_add(skip);
        count = e.get(bs_file, BSFile::iFileSize).wrapping_sub(used);
    }
    let mut delivered = 0u32;
    let mut read = 0u32;
    let own = archive_file(e);
    let archive_size = e.get(own.cast::<BSFile>(), BSFile::iFileSize);
    if position < archive_size {
        if count < e.get(own, NiFile::m_uiBufferAllocSize) {
            let own_buffer = e.call(NI_FILE_GET_BUFFER, &args![own]).u32();
            read = e
                .call(NI_FILE_READ_DIRECT, &args![own, own_buffer, count])
                .u32();
            let own = archive_file(e);
            e.set(own, NiFile::m_uiPos, read);
            let own = archive_file(e);
            e.set(own, NiFile::m_uiBufferReadSize, read);
            delivered = if read < count { read } else { count };
            let own = archive_file(e);
            let own_buffer = e.call(NI_FILE_GET_BUFFER, &args![own]).u32();
            e.call(MEMCPY, &args![destination, own_buffer, delivered]);
        } else {
            read = e
                .call(NI_FILE_READ_DIRECT, &args![own, destination, count])
                .u32();
            delivered = read;
        }
    }
    let own = archive_file(e);
    e.set(
        own.cast::<BSFile>(),
        BSFile::iTrueFilePos,
        position.wrapping_add(read),
    );
    section(e, LEAVE_CRITICAL_SECTION);
    delivered.wrapping_add(delivered_first)
}

/// zlib's `Z_SYNC_FLUSH`, the flush `00afc0e0` asks `inflate` for, and the
/// `z_stream` fields it sets (`next_in` +0, `avail_in` +4, `next_out` +0xc,
/// `avail_out` +0x10).
const Z_SYNC_FLUSH: u32 = 2;
const Z_NEXT_IN: u32 = 0x0;
const Z_AVAIL_IN: u32 = 0x4;
const Z_NEXT_OUT: u32 = 0xc;
const Z_AVAIL_OUT: u32 = 0x10;

// Translated from 00afc0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `CompressedArchiveFile` vtable +0x44
/// (`ReadF`, two words): delivers `count` uncompressed bytes into `buffer`
/// (none are copied for a null `buffer`, but they are counted). What the
/// uncompressed buffer holds is used first. Then, until enough is
/// delivered, the uncompressed buffer is refilled: `inflate(stream, 2)` over
/// the compressed `NiFile` buffer (input = `m_pBuffer + m_uiPos`, the
/// `m_uiBufferReadSize - m_uiPos` bytes left; consumed bytes move `m_uiPos`
/// and `iTrueFilePos`), refilling that buffer when it runs dry (flush, then
/// [`fn_00afb7a0`] of `m_uiBufferAllocSize` bytes), until the output is full;
/// an error (-2, 2, -3 or -4) ends the stream (`00b45db0`) and the read,
/// stream end (1) stops the refill. An empty refill answers what was
/// delivered so far.
pub fn fn_00afc0e0(
    e: &mut Engine,
    this: Ptr<CompressedArchiveFile>,
    buffer: Ptr,
    count: u32,
) -> u32 {
    let file = this.cast::<NiFile>();
    let bs_file = this.cast::<BSFile>();
    let mut destination = buffer.addr();
    let mut remaining = count;
    let mut delivered = 0u32;
    let stream = e.get(this, CompressedArchiveFile::pZStream);
    let available = e
        .get(this, CompressedArchiveFile::iUncompressedBufferReadSize)
        .wrapping_sub(e.get(this, CompressedArchiveFile::iUncompressedBufferPos));
    if remaining <= available {
        if !buffer.is_null() {
            let source = e
                .get(this, CompressedArchiveFile::pUncompressedBuffer)
                .wrapping_add(e.get(this, CompressedArchiveFile::iUncompressedBufferPos));
            e.call(MEMCPY, &args![destination, source, remaining]);
        }
        let position = e.get(this, CompressedArchiveFile::iUncompressedBufferPos);
        e.set(
            this,
            CompressedArchiveFile::iUncompressedBufferPos,
            position.wrapping_add(remaining),
        );
        return delivered.wrapping_add(remaining);
    }
    if available != 0 {
        if !buffer.is_null() {
            let source = e
                .get(this, CompressedArchiveFile::pUncompressedBuffer)
                .wrapping_add(e.get(this, CompressedArchiveFile::iUncompressedBufferPos));
            e.call(MEMCPY, &args![destination, source, available]);
            destination = destination.wrapping_add(available);
        }
        remaining = remaining.wrapping_sub(available);
        delivered = available;
    }
    e.set(this, CompressedArchiveFile::iUncompressedBufferPos, 0);
    e.set(this, CompressedArchiveFile::iUncompressedBufferReadSize, 0);
    while remaining != 0 {
        let buffer_size = e.get(this, CompressedArchiveFile::iUncompressedBufferSize);
        e.mem.set_u32(stream + Z_AVAIL_OUT, buffer_size);
        let output = e.get(this, CompressedArchiveFile::pUncompressedBuffer);
        e.mem.set_u32(stream + Z_NEXT_OUT, output);
        while e.mem.u32(stream + Z_AVAIL_OUT) != 0 {
            let left = e
                .get(file, NiFile::m_uiBufferReadSize)
                .wrapping_sub(e.get(file, NiFile::m_uiPos));
            e.mem.set_u32(stream + Z_AVAIL_IN, left);
            let input = e
                .get(file, NiFile::m_pBuffer)
                .wrapping_add(e.get(file, NiFile::m_uiPos));
            e.mem.set_u32(stream + Z_NEXT_IN, input);
            if left != 0 {
                let status = e.call(INFLATE, &args![stream, Z_SYNC_FLUSH]).i32();
                if matches!(status, -2 | 2 | -3 | -4) {
                    e.call(INFLATE_END, &args![stream]);
                    remaining = 0;
                    break;
                }
                if status == 1 {
                    break;
                }
            }
            let consumed = left.wrapping_sub(e.mem.u32(stream + Z_AVAIL_IN));
            let position = consumed.wrapping_add(e.get(file, NiFile::m_uiPos));
            e.set(file, NiFile::m_uiPos, position);
            let true_position = consumed.wrapping_add(e.get(bs_file, BSFile::iTrueFilePos));
            e.set(bs_file, BSFile::iTrueFilePos, true_position);
            if e.mem.u32(stream + Z_AVAIL_IN) == 0 {
                e.call(NI_FILE_FLUSH, &args![this]);
                let own_buffer = e.get(file, NiFile::m_pBuffer);
                let alloc_size = e.get(file, NiFile::m_uiBufferAllocSize);
                let read = fn_00afb7a0(e, this.cast(), Ptr::new(own_buffer), alloc_size, 0);
                e.set(file, NiFile::m_uiBufferReadSize, read);
            }
        }
        let produced = e
            .get(this, CompressedArchiveFile::iUncompressedBufferSize)
            .wrapping_sub(e.mem.u32(stream + Z_AVAIL_OUT));
        e.set(
            this,
            CompressedArchiveFile::iUncompressedBufferReadSize,
            produced,
        );
        let produced = e.get(this, CompressedArchiveFile::iUncompressedBufferReadSize);
        if produced == 0 {
            return delivered;
        }
        if remaining > produced {
            remaining = remaining.wrapping_sub(produced);
            delivered = delivered.wrapping_add(produced);
            if !buffer.is_null() {
                let source = e.get(this, CompressedArchiveFile::pUncompressedBuffer);
                e.call(MEMCPY, &args![destination, source, produced]);
                destination = destination.wrapping_add(produced);
            }
        } else {
            if !buffer.is_null() {
                let source = e.get(this, CompressedArchiveFile::pUncompressedBuffer);
                e.call(MEMCPY, &args![destination, source, remaining]);
            }
            let position = e.get(this, CompressedArchiveFile::iUncompressedBufferPos);
            e.set(
                this,
                CompressedArchiveFile::iUncompressedBufferPos,
                position.wrapping_add(remaining),
            );
            delivered = delivered.wrapping_add(remaining);
            remaining = 0;
        }
    }
    delivered
}

/// `NiFile`'s absolute seek origin (`010a2480`, a global the code reads).
const NI_FILE_SEEK_SET: u32 = 0x010a_2480;
/// The `BSFile` seek, vtable +0x14.
const SEEK_SLOT: u32 = 0x14;
/// The word `00af96d0` hands `00afb170` (`011ace04`, the third word
/// `00af4470` sets).
const NAME_TABLE_MODE: u32 = 0x011a_ce04;

/// Calls the stream's `m_pfnRead` (+8) the way `00af96d0` does, with five
/// words (`stream, buffer, size, &component_count, 1`) and the component
/// count 1, and adds the answer to `m_uiAbsoluteCurrentPos`.
fn read_through_pointer(e: &mut Engine, this: Ptr<Archive>, buffer: u32, size: u32) {
    let read = e.with_stack(4, |e, component| {
        e.mem.set_u32(component.addr(), 1);
        let function = e.mem.u32(this.addr() + 8);
        e.call(function, &args![this, buffer, size, component, 1u32])
            .u32()
    });
    let file = this.cast::<NiFile>();
    let position = e
        .get(file, NiFile::m_uiAbsoluteCurrentPos)
        .wrapping_add(read);
    e.set(file, NiFile::m_uiAbsoluteCurrentPos, position);
}

// Translated from 00af96d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`, two words): the
/// name of file `file` of directory `directory`. With bit 5 of
/// `cArchiveFlags` (the names kept) it is `pFileNameStringArray +
/// pFileNameStringOffsets[directory][file]`. Without names in the archive
/// (`oFlags` bit 1) it is null. Otherwise `pFileNameStringArray` is
/// replaced by a fresh 0x100-byte block and the name is read into it from
/// the file: when `00afb170(oFlags bit 5, word 011ace04)` answers true, one
/// byte at a time through `m_pfnRead` from `iFileNameArrayOffset +
/// pFileNameStringOffsets[directory][file]` up to and including its 0;
/// otherwise the whole name table (`iFileNamesLength` bytes at
/// `iFileNameArrayOffset`, read into a temporary block) is walked name by
/// name past every file of the directories before `directory` and the
/// first `file` of this one, and the name found is copied (`strcpy_s`, at
/// most 0x100). Walking off the table's end leaves an empty name. Answers
/// the block.
pub fn fn_00af96d0(e: &mut Engine, this: Ptr<Archive>, directory: u32, file: u32) -> u32 {
    let offset_of = |e: &Engine| {
        let offsets = e.get(this, Archive::pFileNameStringOffsets);
        let row = e.mem.u32(offsets.wrapping_add(directory.wrapping_mul(4)));
        e.mem.u32(row.wrapping_add(file.wrapping_mul(4)))
    };
    if e.get(this, Archive::cArchiveFlags) & 0x20 != 0 {
        let offset = offset_of(e);
        return e
            .get(this, Archive::pFileNameStringArray)
            .wrapping_add(offset);
    }
    if e.get(this, Archive::oFlags) & 2 == 0 {
        return 0;
    }
    let old = e.get(this, Archive::pFileNameStringArray);
    if old != 0 {
        free(e, old);
        e.set(this, Archive::pFileNameStringArray, 0);
    }
    let mut index = 0u32;
    let block = allocate(e, 0x100);
    e.set(this, Archive::pFileNameStringArray, block);
    let mode = e.global::<u32>(NAME_TABLE_MODE);
    let names_flag = (e.get(this, Archive::oFlags) & 0x20 != 0) as u8;
    if fn_00afb170(e, this.cast(), names_flag, mode) != 0 {
        let position = e
            .get(this, Archive::iFileNameArrayOffset)
            .wrapping_add(offset_of(e));
        let origin = e.global::<u32>(NI_FILE_SEEK_SET);
        e.vcall(this.addr(), SEEK_SLOT, &args![position, origin]);
        let target = e
            .get(this, Archive::pFileNameStringArray)
            .wrapping_add(index);
        read_through_pointer(e, this, target, 1);
        loop {
            let names = e.get(this, Archive::pFileNameStringArray);
            let byte = e.mem.u8(names.wrapping_add(index));
            index = index.wrapping_add(1);
            if byte == 0 {
                break;
            }
            let target = e
                .get(this, Archive::pFileNameStringArray)
                .wrapping_add(index);
            read_through_pointer(e, this, target, 1);
        }
        return e.get(this, Archive::pFileNameStringArray);
    }
    let size = e.get(this, Archive::iFileNamesLength).wrapping_add(1);
    let table = allocate(e, size);
    let start = e.get(this, Archive::iFileNameArrayOffset);
    let origin = e.global::<u32>(NI_FILE_SEEK_SET);
    e.vcall(this.addr(), SEEK_SLOT, &args![start, origin]);
    let length = e.get(this, Archive::iFileNamesLength);
    read_through_pointer(e, this, table, length);
    let length = e.get(this, Archive::iFileNamesLength);
    e.mem.set_u8(table.wrapping_add(length), 0);
    let mut cursor = table;
    let step = |e: &mut Engine, cursor: &mut u32| -> bool {
        let length = e.call(STRLEN, &args![*cursor]).u32();
        *cursor = cursor.wrapping_add(length).wrapping_add(1);
        let end = table.wrapping_add(e.get(this, Archive::iFileNamesLength));
        *cursor >= end
    };
    let mut directory_index = 0u32;
    while directory_index < directory {
        let mut file_index = 0u32;
        loop {
            let directories = e.get(this, Archive::pDirectories);
            let count = e.mem.u32(
                directories
                    .wrapping_add(directory_index << 4)
                    .wrapping_add(8),
            );
            if file_index >= count {
                break;
            }
            if step(e, &mut cursor) {
                let names = e.get(this, Archive::pFileNameStringArray);
                e.mem.set_u8(names, 0);
                free(e, table);
                return e.get(this, Archive::pFileNameStringArray);
            }
            file_index += 1;
        }
        directory_index += 1;
    }
    let mut file_index = 0u32;
    while file_index < file {
        if step(e, &mut cursor) {
            let names = e.get(this, Archive::pFileNameStringArray);
            e.mem.set_u8(names, 0);
            free(e, table);
            return e.get(this, Archive::pFileNameStringArray);
        }
        file_index += 1;
    }
    let names = e.get(this, Archive::pFileNameStringArray);
    e.call(STRCPY_S, &args![names, 0x100u32, cursor]);
    free(e, table);
    e.get(this, Archive::pFileNameStringArray)
}

/// `ArchiveManager::TrimFileName` (`00b014b0`, cdecl, one word): the path
/// the lookups use.
const TRIM_FILE_NAME: u32 = 0x00b0_14b0;
/// `00afd270` (cdecl: path, directory hash out, file hash out): hashes a
/// path's directory and file name (8 bytes each).
const HASH_PATH: u32 = 0x00af_d270;
/// The critical section of the archive list (`011f8170`).
const ARCHIVE_LIST_LOCK: u32 = 0x011f_8170;
/// The archive type mask that means "any type".
const ANY_ARCHIVE_TYPE: u32 = 0xffff;

/// The hashes and indices the list lookups keep on their stack: the
/// directory hash (+0), the file hash (+8), the directory index (+0x10) and
/// the file index (+0x14).
const LOOKUP_LOCALS: u32 = 0x18;

/// Whether the archive's type bits (`iArchiveType`) meet `mask`.
fn type_matches(e: &Engine, archive: u32, mask: u32) -> bool {
    let types = u32::from(e.mem.u16(archive + Archive::iArchiveType.off));
    (types & mask) & 0xffff != 0
}

// Translated from 00af6160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af6160 - the FS:[0] exception frame (the unwind state of the NiPointer per archive) is not translated
/// Unnamed in the engine map (cdecl, two words): the archive that holds
/// `path`, or null (null too when there is no archive list). The path is
/// trimmed (`00b014b0`), a `types` of 0xffff becomes the type of its
/// extension ([`archive_manager_get_archive_type_from_file_extension`]),
/// it is hashed (`00afd270`), and under the list's critical section
/// (`011f8170`) each listed archive whose type bits meet `types` is asked
/// ([`fn_00af9bf0`]); each is held by a reference while asked. Answers the
/// first archive that has the file.
pub fn fn_00af6160(e: &mut Engine, path: Ptr, types: u32) -> u32 {
    if e.global::<u32>(ARCHIVE_LIST) == 0 {
        return 0;
    }
    let path = e.call(TRIM_FILE_NAME, &args![path]).u32();
    let mut types = types;
    if types == ANY_ARCHIVE_TYPE {
        types = archive_manager_get_archive_type_from_file_extension(e, Ptr::new(path));
    }
    e.with_stack(LOOKUP_LOCALS, |e, locals| {
        let base = locals.addr();
        e.call(HASH_PATH, &args![path, base, base + 8]);
        e.call(ENTER_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
        let mut node = e.global::<u32>(ARCHIVE_LIST);
        while node != 0 {
            let archive = e.mem.u32(node);
            if archive != 0 {
                add_ref(e, archive);
            }
            if archive != 0
                && type_matches(e, archive, types)
                && fn_00af9bf0(
                    e,
                    Ptr::new(archive),
                    Ptr::new(base),
                    Ptr::new(base + 8),
                    Ptr::new(base + 0x10),
                    Ptr::new(base + 0x14),
                    Ptr::new(path),
                )
            {
                e.call(LEAVE_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
                release(e, archive);
                return archive;
            }
            node = e.mem.u32(node + 4);
            if archive != 0 {
                release(e, archive);
            }
        }
        e.call(LEAVE_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
        0
    })
}

// Translated from 00af5fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af5fa0 - the FS:[0] exception frame (the unwind state of the NiPointer per archive) is not translated
/// `ArchiveManager::GetFile` (Xbox PDB; cdecl, three words): a file object
/// for `path` from the first listed archive that has it (as
/// [`fn_00af6160`] finds it, without the list's critical section), made by
/// [`fn_00afa550`] with the found indices, `passed` and the trimmed path;
/// null when no archive has it.
pub fn archive_manager_get_file(e: &mut Engine, path: Ptr, passed: u32, types: u32) -> u32 {
    e.with_stack(LOOKUP_LOCALS, |e, locals| {
        let base = locals.addr();
        e.mem.set_u32(base + 0x10, 0);
        e.mem.set_u32(base + 0x14, 0);
        let path = e.call(TRIM_FILE_NAME, &args![path]).u32();
        let mut types = types;
        if types == ANY_ARCHIVE_TYPE {
            types = archive_manager_get_archive_type_from_file_extension(e, Ptr::new(path));
        }
        e.call(HASH_PATH, &args![path, base, base + 8]);
        let mut node = e.global::<u32>(ARCHIVE_LIST);
        while node != 0 {
            let archive = e.mem.u32(node);
            if archive != 0 {
                add_ref(e, archive);
            }
            if archive != 0
                && type_matches(e, archive, types)
                && fn_00af9bf0(
                    e,
                    Ptr::new(archive),
                    Ptr::new(base),
                    Ptr::new(base + 8),
                    Ptr::new(base + 0x10),
                    Ptr::new(base + 0x14),
                    Ptr::new(path),
                )
            {
                let directory = e.mem.u32(base + 0x10);
                let file = e.mem.u32(base + 0x14);
                let made = fn_00afa550(e, Ptr::new(archive), directory, file, passed, path);
                release(e, archive);
                return made;
            }
            node = e.mem.u32(node + 4);
            if archive != 0 {
                release(e, archive);
            }
        }
        0
    })
}

/// The `ArchiveFile` and `CompressedArchiveFile` allocation sizes.
const ARCHIVE_FILE_SIZE: u32 = 0x160;
const COMPRESSED_ARCHIVE_FILE_SIZE: u32 = 0x178;

/// Makes the file object for the 16-byte file `entry` of `archive` (shared
/// by `00afa550` and `00afa750`, inlined in both): an `ArchiveFile` when
/// bit 2 of the archive's `oFlags` equals bit 30 of the entry's size word
/// (+8), a `CompressedArchiveFile` otherwise, each constructed with
/// `(path, archive, offset (+0xc, low 31 bits), size (+8, low 30 bits),
/// passed)`; null when the allocation fails.
fn make_file(
    e: &mut Engine,
    archive: Ptr<Archive>,
    entry: u32,
    offset: u32,
    passed: u32,
    path: u32,
) -> u32 {
    let archive_bit = e.get(archive, Archive::oFlags) & 4 != 0;
    let entry_bit = e.mem.u32(entry + 8) & 0x4000_0000 != 0;
    if archive_bit == entry_bit {
        let block = allocate(e, ARCHIVE_FILE_SIZE);
        if block == 0 {
            return 0;
        }
        let size = e.mem.u32(entry + 8) & 0x3fff_ffff;
        fn_00afb300(
            e,
            Ptr::new(block),
            Ptr::new(path),
            archive.addr(),
            offset,
            size,
            passed,
        )
        .addr()
    } else {
        let block = allocate(e, COMPRESSED_ARCHIVE_FILE_SIZE);
        if block == 0 {
            return 0;
        }
        let size = e.mem.u32(entry + 8) & 0x3fff_ffff;
        fn_00afc430(
            e,
            Ptr::new(block),
            Ptr::new(path),
            archive.addr(),
            offset,
            size,
            passed,
        )
        .addr()
    }
}

// Translated from 00afa550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afa550 - the FS:[0] exception frame (the unwind states of the two constructors) is not translated
/// Unnamed in the engine map (thiscall on an `Archive`, four words): the
/// file object for file `file` of directory `directory`: null when the
/// entry's address is 0 or it has no data ([`fn_00afb2a0`] without `any`);
/// otherwise an `ArchiveFile` or a `CompressedArchiveFile` as the
/// archive's and the entry's compression bits say (see `make_file`).
pub fn fn_00afa550(
    e: &mut Engine,
    this: Ptr<Archive>,
    directory: u32,
    file: u32,
    passed: u32,
    path: u32,
) -> u32 {
    let directories = e.get(this, Archive::pDirectories);
    let files = e
        .mem
        .u32(directories.wrapping_add(directory << 4).wrapping_add(0xc));
    let entry = (file << 4).wrapping_add(files);
    if entry == 0 || !fn_00afb2a0(e, this, Ptr::new(entry), false) {
        return 0;
    }
    let offset = e.mem.u32(entry + 0xc) & 0x7fff_ffff;
    make_file(e, this, entry, offset, passed, path)
}

// Translated from 00afa750 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afa750 - the FS:[0] exception frame (the unwind states of the two constructors) is not translated
/// Unnamed in the engine map (thiscall on an `Archive`, three words): as
/// [`fn_00afa550`] for a file `entry` given directly (null for a null
/// entry).
pub fn fn_00afa750(e: &mut Engine, this: Ptr<Archive>, entry: Ptr, passed: u32, path: u32) -> u32 {
    if entry.is_null() || !fn_00afb2a0(e, this, entry, false) {
        return 0;
    }
    let offset = e.mem.u32(entry.addr() + 0xc) & 0x7fff_ffff;
    make_file(e, this, entry.addr(), offset, passed, path)
}

// Translated from 00af6910 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af6910 - the FS:[0] exception frame (the unwind state of the NiPointer per archive) is not translated
/// Unnamed in the engine map (cdecl, two words): under the list's critical
/// section, the first listed archive whose `BSArchive` part has a
/// directory with `hash` ([`fn_00afdc00`]) among those whose type bits meet
/// `types` (or all, for 0xffff); then, unless `types` is 0xffff, among
/// those whose type bits do not. Each is held by a reference while asked.
/// Answers the archive or null.
///
/// For `types` 0xffff with no find the exe returns without leaving the
/// critical section; so does this.
pub fn fn_00af6910(e: &mut Engine, hash: Ptr, types: u32) -> u32 {
    e.call(ENTER_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
    let mut node = e.global::<u32>(ARCHIVE_LIST);
    while node != 0 {
        let archive = e.mem.u32(node);
        if archive != 0 {
            add_ref(e, archive);
        }
        if archive != 0
            && (type_matches(e, archive, types) || types == ANY_ARCHIVE_TYPE)
            && fn_00afdc00(e, Ptr::new(archive + Archive::iTag.off), hash.addr())
        {
            e.call(LEAVE_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
            release(e, archive);
            return archive;
        }
        node = e.mem.u32(node + 4);
        if archive != 0 {
            release(e, archive);
        }
    }
    if types == ANY_ARCHIVE_TYPE {
        return 0;
    }
    let mut node = e.global::<u32>(ARCHIVE_LIST);
    while node != 0 {
        let archive = e.mem.u32(node);
        if archive != 0 {
            add_ref(e, archive);
        }
        if archive != 0
            && !type_matches(e, archive, types)
            && fn_00afdc00(e, Ptr::new(archive + Archive::iTag.off), hash.addr())
        {
            e.call(LEAVE_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
            release(e, archive);
            return archive;
        }
        node = e.mem.u32(node + 4);
        if archive != 0 {
            release(e, archive);
        }
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
    0
}

// Translated from 00af6340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af6340 - the FS:[0] exception frame (the unwind state of the NiPointer) is not translated
/// Unnamed in the engine map (cdecl, four words; the first unused): the
/// file object for the file `entry` from the archive that has the entry's
/// directory hash ([`fn_00af6910`] with `entry` as the hash and any type),
/// held by a reference while [`fn_00afa750`] makes it; null without such an
/// archive. The exe also computes bit 31 of the entry's word at +0xc into
/// a local it never reads.
pub fn fn_00af6340(e: &mut Engine, _unused_0: u32, entry: Ptr, passed: u32, path: u32) -> u32 {
    let _unread = e.mem.u32(entry.addr() + 0xc) & 0x8000_0000 != 0;
    let mut made = 0u32;
    let archive = fn_00af6910(e, entry, ANY_ARCHIVE_TYPE);
    if archive != 0 {
        add_ref(e, archive);
    }
    if archive != 0 {
        made = fn_00afa750(e, Ptr::new(archive), entry, passed, path);
    }
    if archive != 0 {
        release(e, archive);
    }
    made
}

// Translated from 00af6540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af6540 - the FS:[0] exception frame (the unwind state of the NiPointer per archive) is not translated
/// Unnamed in the engine map (cdecl, four words): the first file entry
/// [`fn_00afa6e0`] finds for `(directory, file, name)` in a listed archive
/// whose type bits have bit `type_index` (shifted as the CPU shifts, by
/// the low 5 bits); each archive is held by a reference while asked; null
/// when none has it.
pub fn fn_00af6540(e: &mut Engine, type_index: u8, directory: Ptr, file: Ptr, name: Ptr) -> u32 {
    let mut node = e.global::<u32>(ARCHIVE_LIST);
    while node != 0 {
        let archive = e.mem.u32(node);
        if archive != 0 {
            add_ref(e, archive);
        }
        if archive != 0 {
            let bit = 1u32 << (u32::from(type_index) & 0x1f);
            if type_matches(e, archive, bit) {
                let found = fn_00afa6e0(e, Ptr::new(archive), directory, file, name);
                if found != 0 {
                    release(e, archive);
                    return found;
                }
            }
        }
        node = e.mem.u32(node + 4);
        if archive != 0 {
            release(e, archive);
        }
    }
    0
}

// Translated from 00af8f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`): builds the
/// archive's file name tables when it has files and one of
/// `pFileNameStringArray` / `pFileNameStringOffsets` is missing. Clears
/// bit 5 of `cArchiveFlags`, frees the old per-directory offset arrays and
/// their array (when there is one) and the string block, allocates a
/// string block of `iFileNamesLength` bytes and an array of
/// `iDirectories` pointers (sizes that overflow become 0xffffffff). With
/// names in the archive (`oFlags` bit 1) the name table is read from
/// `iFileNameArrayOffset` (seek, then `m_pfnRead`) and for each directory
/// an array of its files' name offsets is built (each name's `strlen + 1`
/// apart). Finally sets bit 5 of the flags.
///
/// The exe's flag updates each hold a second branch behind a constant
/// test (`XOR ECX,ECX; JZ` and `MOV ECX,1; TEST; JZ`); both are kept.
pub fn fn_00af8f10(e: &mut Engine, this: Ptr<Archive>) {
    if e.get(this, Archive::iFiles) == 0 {
        return;
    }
    if e.get(this, Archive::pFileNameStringArray) != 0
        && e.get(this, Archive::pFileNameStringOffsets) != 0
    {
        return;
    }
    let set_first = false;
    let flags = e.get(this, Archive::cArchiveFlags);
    if set_first {
        e.set(this, Archive::cArchiveFlags, flags | 0x20);
    } else {
        e.set(this, Archive::cArchiveFlags, flags & 0xdf);
    }
    if e.get(this, Archive::pFileNameStringOffsets) != 0 {
        let mut index = 0u32;
        while index < e.get(this, Archive::iDirectories) {
            let offsets = e.get(this, Archive::pFileNameStringOffsets);
            let row = e.mem.u32(offsets.wrapping_add(index.wrapping_mul(4)));
            free(e, row);
            index += 1;
        }
        let offsets = e.get(this, Archive::pFileNameStringOffsets);
        free(e, offsets);
    }
    let strings = e.get(this, Archive::pFileNameStringArray);
    free(e, strings);
    let length = e.get(this, Archive::iFileNamesLength);
    let block = allocate(e, length);
    e.set(this, Archive::pFileNameStringArray, block);
    let directories = e.get(this, Archive::iDirectories);
    // `MUL` by 4 with `SETO`/`NEG`/`OR`: all ones on overflow.
    #[allow(clippy::manual_saturating_arithmetic)]
    let size = directories.checked_mul(4).unwrap_or(u32::MAX);
    let block = allocate(e, size);
    e.set(this, Archive::pFileNameStringOffsets, block);
    if e.get(this, Archive::oFlags) & 2 != 0 {
        let start = e.get(this, Archive::iFileNameArrayOffset);
        let origin = e.global::<u32>(NI_FILE_SEEK_SET);
        e.vcall(this.addr(), SEEK_SLOT, &args![start, origin]);
        let length = e.get(this, Archive::iFileNamesLength);
        let strings = e.get(this, Archive::pFileNameStringArray);
        read_through_pointer(e, this, strings, length);
        let mut offset = 0u32;
        let mut directory = 0u32;
        while directory < e.get(this, Archive::iDirectories) {
            let entries = e.get(this, Archive::pDirectories);
            let count = e
                .mem
                .u32(entries.wrapping_add(directory << 4).wrapping_add(8));
            #[allow(clippy::manual_saturating_arithmetic)]
            let size = count.checked_mul(4).unwrap_or(u32::MAX);
            let row = allocate(e, size);
            let offsets = e.get(this, Archive::pFileNameStringOffsets);
            e.mem
                .set_u32(offsets.wrapping_add(directory.wrapping_mul(4)), row);
            let mut file = 0u32;
            loop {
                let entries = e.get(this, Archive::pDirectories);
                let count = e
                    .mem
                    .u32(entries.wrapping_add(directory << 4).wrapping_add(8));
                if file >= count {
                    break;
                }
                let offsets = e.get(this, Archive::pFileNameStringOffsets);
                let row = e.mem.u32(offsets.wrapping_add(directory.wrapping_mul(4)));
                e.mem
                    .set_u32(row.wrapping_add(file.wrapping_mul(4)), offset);
                let name = e
                    .get(this, Archive::pFileNameStringArray)
                    .wrapping_add(offset);
                let length = e.call(STRLEN, &args![name]).u32();
                offset = offset.wrapping_add(length).wrapping_add(1);
                file += 1;
            }
            directory += 1;
        }
    }
    let set_last = true;
    let flags = e.get(this, Archive::cArchiveFlags);
    if set_last {
        e.set(this, Archive::cArchiveFlags, flags | 0x20);
    } else {
        e.set(this, Archive::cArchiveFlags, flags & 0xdf);
    }
}

/// Two `BSSimpleArray`s of allocated blocks `00af5820` empties and deletes
/// (each a pointer to the array object: buffer at +4, size at +8).
const BLOCK_ARRAY_011F8168: u32 = 0x011f_8168;
const BLOCK_ARRAY_011F816C: u32 = 0x011f_816c;
/// Two rows of nine `NiPointer<Archive>` (`011f8190` and 0x24 after it)
/// `00af5820` clears.
const ARCHIVE_POINTER_ROWS: u32 = 0x011f_8190;
const ARCHIVE_POINTER_ROW_SIZE: u32 = 0x24;

// Translated from 00af5820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the archive shut-down. Empties
/// the archive list ([`fn_00afcaf0`]), destroys its head node
/// ([`fn_00afc7f0`]) and frees it, and clears `011f8160`; frees every
/// block of the arrays behind `011f8168` and `011f816c` and deletes each
/// array (vtable +0 with 1); clears the 2 x 9 archive pointers at
/// `011f8190` ([`fn_00afd1b0`] with null, the two rows interleaved).
pub fn fn_00af5820(e: &mut Engine) {
    if e.global::<u32>(ARCHIVE_LIST) != 0 {
        let list = e.global::<u32>(ARCHIVE_LIST);
        fn_00afcaf0(e, Ptr::new(list));
        let list = e.global::<u32>(ARCHIVE_LIST);
        if list != 0 {
            fn_00afc7f0(e, Ptr::new(list));
            free(e, list);
        }
        e.set_global(ARCHIVE_LIST, 0u32);
    }
    for holder in [BLOCK_ARRAY_011F8168, BLOCK_ARRAY_011F816C] {
        if e.global::<u32>(holder) == 0 {
            continue;
        }
        let mut index = 0u32;
        loop {
            let array = e.global::<u32>(holder);
            if index >= e.mem.u32(array + 8) {
                break;
            }
            let array = e.global::<u32>(holder);
            let blocks = e.mem.u32(array + 4);
            let block = e.mem.u32(blocks.wrapping_add(index.wrapping_mul(4)));
            free(e, block);
            index += 1;
        }
        let array = e.global::<u32>(holder);
        if array != 0 {
            e.vcall(array, 0, &args![1u32]);
        }
    }
    for index in 0..9u32 {
        let first = ARCHIVE_POINTER_ROWS + index * 4;
        fn_00afd1b0(e, Ptr::new(first), 0);
        let second = ARCHIVE_POINTER_ROWS + ARCHIVE_POINTER_ROW_SIZE + index * 4;
        fn_00afd1b0(e, Ptr::new(second), 0);
    }
}

// Translated from 00af5320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af5320 - the FS:[0] exception frame (the unwind state of the NiPointer per archive) is not translated
/// Unnamed in the engine map (cdecl, one word): the listed archive whose
/// file name (vtable +0x18) equals `name` (`_stricmp`), searched under the
/// list's critical section with a reference held per archive; null when
/// there is no list or no such archive.
pub fn fn_00af5320(e: &mut Engine, name: Ptr) -> u32 {
    if e.global::<u32>(ARCHIVE_LIST) == 0 {
        return 0;
    }
    e.call(ENTER_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
    let mut node = e.global::<u32>(ARCHIVE_LIST);
    while node != 0 {
        let archive = e.mem.u32(node);
        if archive != 0 {
            add_ref(e, archive);
            let own_name = e.vcall(archive, ARCHIVE_SLOT_18, &args![]).u32();
            if e.call(STRICMP, &args![own_name, name]).i32() == 0 {
                e.call(LEAVE_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
                release(e, archive);
                return archive;
            }
        }
        node = e.mem.u32(node + 4);
        if archive != 0 {
            release(e, archive);
        }
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
    0
}

/// Import slots (KERNEL32): `GetTickCount` and `InitializeCriticalSection`.
const GET_TICK_COUNT: u32 = 0x00fd_f060;
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// CRT (cdecl): `strstr`, `strrchr`, `fopen_s`, `feof`, `fgets`, `_stat64i32`.
const STRSTR: u32 = 0x00ec_7750;
const STRRCHR: u32 = 0x00ec_6e30;
const FOPEN_S: u32 = 0x00ec_9a5e;
const FEOF: u32 = 0x00ec_dacc;
const FGETS: u32 = 0x00ec_c104;
const STAT64I32: u32 = 0x00ec_923c;
/// `00406d00` (cdecl, a buffer, its size, a format and its words): the
/// formatted print `00af4550` builds the plugin list's path with.
const PRINT_TO_BUFFER: u32 = 0x0040_6d00;
/// `FileFinder::GetFile` (`00afdf00`, cdecl: path, mode, passed word) and
/// the finder lookup `00afe0d0` (thiscall on the finder: path, out, flags,
/// extra), both in `filefinder.cpp`; the `FileFinder` pointer.
const FILE_FINDER_GET_FILE: u32 = 0x00af_df00;
const FILE_FINDER_LOOKUP: u32 = 0x00af_e0d0;
const FILE_FINDER: u32 = 0x011f_81dc;
/// `BSFile::BSFile` (`00b00260`, thiscall: name, mode, buffer size, 0) and
/// `BSFile::Open` (`00aff300`, thiscall, two words).
const BSFILE_CONSTRUCTOR: u32 = 0x00b0_0260;
const BSFILE_OPEN: u32 = 0x00af_f300;
/// `00affd80` (thiscall on a `BSFile`, one word: a buffer size).
const BSFILE_SET_BUFFER_SIZE: u32 = 0x00af_fd80;
/// The `BSFile` vtable slots `Open` (+0x20), `GetSize` (+0x28) and
/// `GetLine` (+0x34), and the deleting destructor (+0).
const OPEN_SLOT: u32 = 0x20;
const GET_SIZE_SLOT: u32 = 0x28;
const GET_LINE_SLOT: u32 = 0x34;
const DELETING_DESTRUCTOR_SLOT: u32 = 0;
/// `00afcce0` (thiscall, no words): the constructor of the 0x10-byte hash
/// arrays `011f8168` / `011f816c` hold.
const HASH_ARRAY_CONSTRUCTOR: u32 = 0x00af_cce0;
const HASH_ARRAY_SIZE: u32 = 0x10;
/// `00afd390` (thiscall on an 8-byte block: a name, a flag word): makes a
/// name hash.
const HASH_CONSTRUCTOR: u32 = 0x00af_d390;
/// `00a11d20` (thiscall on an array, one word: the address of the element):
/// appends an element.
const ARRAY_ADD: u32 = 0x00a1_1d20;
/// `005ae3d0` (thiscall on a list, one word: the address of the element):
/// adds a name to the file list.
const LIST_ADD_NAME: u32 = 0x005a_e3d0;
/// The strings `00af5ab0` and `00afad00` use.
const TEXT_BACKSLASH: u32 = 0x010a_445c;
const TEXT_DATA_INVALIDATION: u32 = 0x010a_4580;
const TEXT_EMPTY_SECOND: u32 = 0x010a_4346;
/// `00b01510` (thiscall on an `Archive`, one word: an 8-byte local) and
/// `00b01590` (thiscall on an `Archive`: path, prefix, that local): the
/// invalidation by file time `00afad00` falls back on.
const INVALIDATION_PREPARE: u32 = 0x00b0_1510;
const INVALIDATION_BY_TIME: u32 = 0x00b0_1590;

// Translated from 00af5ab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af5ab0 - the FS:[0] exception frame (the unwind states of the allocations) and the /GS stack cookie check (00ec408c) are not translated
/// Unnamed in the engine map (cdecl, one word): reads the archive
/// invalidation file `path` (nothing for a null path). The file comes from
/// `FileFinder::GetFile(path, 0, 0x4000)`; a missing file or a failed open
/// (vtable +0x20 with `(0, 0)`) is deleted (vtable +0 with 1) and nothing
/// more happens. Each line (vtable +0x34 `GetLine` into a 0x104-byte
/// buffer, ending at `\r`, until it answers 0) is followed by reading one
/// more byte through `m_pfnRead` (the `\n`). A line with a `\` names a
/// directory: its hash (`00afd390(name, 1)`, a leading `\` skipped) goes
/// into the array at `011f816c`; any other line names a file, hashed with
/// `00afd390(name, 0)` into the array at `011f8168`. Each array is made
/// (0x10 bytes, `00afcce0`) when first needed. The file is deleted at the
/// end.
pub fn fn_00af5ab0(e: &mut Engine, path: Ptr) {
    if path.is_null() {
        return;
    }
    let file = e
        .call(FILE_FINDER_GET_FILE, &args![path, 0u32, 0x4000u32])
        .u32();
    if file == 0 || !e.vcall(file, OPEN_SLOT, &args![0u32, 0u32]).bool() {
        if file != 0 {
            e.vcall(file, DELETING_DESTRUCTOR_SLOT, &args![1u32]);
        }
        return;
    }
    e.with_stack(0x114, |e, frame| {
        let line = frame.addr();
        let extra_byte = frame.addr() + 0x104;
        let component = frame.addr() + 0x108;
        let element = frame.addr() + 0x10c;
        loop {
            let read = e
                .vcall(file, GET_LINE_SLOT, &args![line, 0x104u32, 0xdu32])
                .u32();
            if read == 0 {
                break;
            }
            e.mem.set_u32(component, 1);
            let function = e.mem.u32(file + 8);
            let got = e
                .call(function, &args![file, extra_byte, 1u32, component, 1u32])
                .u32();
            let position = e.mem.u32(file + 4).wrapping_add(got);
            e.mem.set_u32(file + 4, position);
            let directory = e.call(STRSTR, &args![line, TEXT_BACKSLASH]).u32() != 0;
            let (holder, name, flag) = if directory {
                let name = if e.mem.u8(line) == b'\\' {
                    line + 1
                } else {
                    line
                };
                (BLOCK_ARRAY_011F816C, name, 1u32)
            } else {
                (BLOCK_ARRAY_011F8168, line, 0u32)
            };
            if e.global::<u32>(holder) == 0 {
                let block = allocate(e, HASH_ARRAY_SIZE);
                let array = if block == 0 {
                    0
                } else {
                    e.call(HASH_ARRAY_CONSTRUCTOR, &args![block]).u32()
                };
                e.set_global(holder, array);
            }
            let block = allocate(e, 8);
            let hash = if block == 0 {
                0
            } else {
                e.call(HASH_CONSTRUCTOR, &args![block, name, flag]).u32()
            };
            e.mem.set_u32(element, hash);
            let array = e.global::<u32>(holder);
            e.call(ARRAY_ADD, &args![array, element]);
        }
    });
    e.vcall(file, DELETING_DESTRUCTOR_SLOT, &args![1u32]);
}

/// Whether one of the `count` hashes of the array object at `array`
/// (buffer at +4) equals the 8-byte value at `value` ([`fn_00afce90`] with
/// the hash as `this`).
fn hash_listed(e: &mut Engine, array: u32, value: u32) -> bool {
    let mut index = 0u32;
    while index < e.mem.u32(array + 8) {
        let hash = e
            .mem
            .u32(e.mem.u32(array + 4).wrapping_add(index.wrapping_mul(4)));
        if fn_00afce90(e, Ptr::new(hash), Ptr::new(value)) == 0 {
            return true;
        }
        index += 1;
    }
    false
}

/// Keeps bit 31 of a file entry's word at +0xc and clears the rest.
fn clear_entry_offset(e: &mut Engine, entry: u32) {
    let word = e.mem.u32(entry + 0xc);
    e.mem.set_u32(entry + 0xc, word & 0x8000_0000);
}

// Translated from 00afad00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afad00 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// Unnamed in the engine map (thiscall on an `Archive`): applies the
/// archive invalidation hashes and answers how many file entries were
/// invalidated (offset cleared, bit 31 of +0xc kept); 0 when neither hash
/// array exists. An archive without directory or file names (`oFlags`
/// bits 0 and 1) goes through `00b01510` and `00b01590(011f8050, "",
/// local)` instead. Otherwise, per directory: one listed in `011f816c`
/// invalidates all its files; else, when `"Data\" + directory name` exists
/// on disk, each of its files listed in `011f8168` is invalidated.
pub fn fn_00afad00(e: &mut Engine, this: Ptr<Archive>) -> u32 {
    let mut invalidated = 0u32;
    if e.global::<u32>(BLOCK_ARRAY_011F816C) == 0 && e.global::<u32>(BLOCK_ARRAY_011F8168) == 0 {
        return 0;
    }
    let flags = e.get(this, Archive::oFlags);
    if flags & 1 == 0 || flags & 2 == 0 {
        return e.with_stack(8, |e, local| {
            e.call(INVALIDATION_PREPARE, &args![this, local]);
            e.call(
                INVALIDATION_BY_TIME,
                &args![this, PATH_011F8050, TEXT_EMPTY_SECOND, local],
            )
            .u32()
        });
    }
    let mut index = 0u32;
    while index < e.get(this, Archive::iDirectories) {
        let directory = (index << 4).wrapping_add(e.get(this, Archive::pDirectories));
        let directories = e.global::<u32>(BLOCK_ARRAY_011F816C);
        if directories != 0 && hash_listed(e, directories, directory) {
            let mut file = 0u32;
            while file < e.mem.u32(directory + 8) {
                let entry = (file << 4).wrapping_add(e.mem.u32(directory + 0xc));
                clear_entry_offset(e, entry);
                file += 1;
            }
            invalidated = invalidated.wrapping_add(e.mem.u32(directory + 8));
            index += 1;
            continue;
        }
        let on_disk = e.with_stack(0x104, |e, local| {
            e.call(STRCPY_S, &args![local, 0x104u32, TEXT_DATA_INVALIDATION]);
            let name = e.call(DIRECTORY_NAME, &args![this, index]).u32();
            e.call(STRCAT_S, &args![local, 0x104u32, name]);
            e.call(ACCESS, &args![local, 0u32]).i32() != -1
        });
        if on_disk {
            let mut file = 0u32;
            while file < e.mem.u32(directory + 8) {
                let entry = (file << 4).wrapping_add(e.mem.u32(directory + 0xc));
                let files = e.global::<u32>(BLOCK_ARRAY_011F8168);
                if files != 0 && hash_listed(e, files, entry) {
                    clear_entry_offset(e, entry);
                    invalidated = invalidated.wrapping_add(1);
                }
                file += 1;
            }
        }
        index += 1;
    }
    invalidated
}

// Translated from 00afd020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`): whether the
/// archive keeps its name tables. Yes when the byte `011acdf9` or
/// `011f8154` is set, or `oFlags` bit 7, or [`fn_00afb170`] says so for
/// one of `(oFlags bit 3, 011acdfc)`, `(bit 4, 011ace00)`, `(bit 5,
/// 011ace04)`; but then an archive of type bit 4 (`iArchiveType & 0x10`)
/// needs `(bit 4, 011ace00)` or `(bit 5, 011ace04)` too.
pub fn fn_00afd020(e: &mut Engine, this: Ptr<Archive>) -> bool {
    let ask = |e: &mut Engine, bit: u32, word: u32| {
        let flag = (e.get(this, Archive::oFlags) & bit != 0) as u8;
        let mode = e.global::<u32>(word);
        fn_00afb170(e, this.cast(), flag, mode) != 0
    };
    let wanted = e.global::<u8>(FLAG_011ACDF9) != 0
        || e.global::<u8>(FLAG_SET_BY_00AF4460) != 0
        || e.get(this, Archive::oFlags) & 0x80 != 0
        || ask(e, 8, WORDS_SET_BY_00AF4470)
        || ask(e, 0x10, WORDS_SET_BY_00AF4470 + 4)
        || ask(e, 0x20, WORDS_SET_BY_00AF4470 + 8);
    if !wanted {
        return false;
    }
    if e.get(this, Archive::iArchiveType) & 0x10 != 0
        && !ask(e, 0x10, WORDS_SET_BY_00AF4470 + 4)
        && !ask(e, 0x20, WORDS_SET_BY_00AF4470 + 8)
    {
        return false;
    }
    true
}

// Translated from 00af6e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): sorts the pointers
/// `array[low..=high]` to 8-byte hashes in place by their values
/// ([`fn_00afce90`], ascending): a quicksort with the first element as the
/// pivot, signed indices, recursing on both sides.
pub fn fn_00af6e50(e: &mut Engine, array: Ptr, low: i32, high: i32) {
    let base = array.addr();
    let slot = |index: i32| base.wrapping_add((index as u32).wrapping_mul(4));
    let (first, last) = (low, high);
    let (mut low, mut high) = (low, high);
    let pivot = e.mem.u32(slot(low));
    while low < high {
        loop {
            let value = e.mem.u32(slot(high));
            if fn_00afce90(e, Ptr::new(value), Ptr::new(pivot)) < 0 || low >= high {
                break;
            }
            high -= 1;
        }
        if low != high {
            let value = e.mem.u32(slot(high));
            e.mem.set_u32(slot(low), value);
            low += 1;
        }
        loop {
            let value = e.mem.u32(slot(low));
            if fn_00afce90(e, Ptr::new(value), Ptr::new(pivot)) > 0 || low >= high {
                break;
            }
            low += 1;
        }
        if low != high {
            let value = e.mem.u32(slot(low));
            e.mem.set_u32(slot(high), value);
            high -= 1;
        }
    }
    e.mem.set_u32(slot(low), pivot);
    let middle = low;
    if first < middle {
        fn_00af6e50(e, array, first, middle - 1);
    }
    if last > middle {
        fn_00af6e50(e, array, middle + 1, last);
    }
}

// Translated from 00af7060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af7060 - the FS:[0] exception frame (the unwind state of the hash allocations) is not translated
/// Unnamed in the engine map (cdecl, seven words): adds to `list` the files
/// of `archive` in the directory `*directory_hash` whose hash matches the
/// pattern hash `*pattern_hash` ([`fn_00afcf00`]; the file hashes are in
/// order, so a smaller one ends the walk) and that are not in it yet, under
/// the archive's critical section. The names already in the list are
/// hashed once (`count` of them, an array of `00afd390(name, 0)` hashes
/// sorted with [`fn_00af6e50`]) when `names` is null; a file is new when a
/// halving search of that array finds no equal hash. A new file is added
/// as a new block holding `prefix` followed by its name ([`fn_00af96d0`]),
/// through `005ae3d0(list, &block)`. Answers the hash array (made here or
/// the one given).
pub fn fn_00af7060(
    e: &mut Engine,
    archive: Ptr<Archive>,
    count: u32,
    names: u32,
    directory_hash: Ptr,
    pattern_hash: Ptr,
    list: u32,
    prefix: Ptr,
) -> u32 {
    let mut names = names;
    let section = archive.addr() + Archive::ArchiveCriticalSection.off;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    e.with_stack(8, |e, locals| {
        let directory_index = locals.addr();
        let block_slot = locals.addr() + 4;
        if !fn_00af9c50(
            e,
            archive,
            directory_hash,
            Ptr::new(directory_index),
            Ptr::new(0),
        ) {
            return;
        }
        if names == 0 && count != 0 {
            #[allow(clippy::manual_saturating_arithmetic)]
            let size = count.checked_mul(4).unwrap_or(u32::MAX);
            names = allocate(e, size);
            let mut node = list;
            for index in 0..count {
                let item = e.mem.u32(node);
                let block = allocate(e, 8);
                let hash = if block == 0 {
                    0
                } else {
                    e.call(HASH_CONSTRUCTOR, &args![block, item, 0u32]).u32()
                };
                e.mem
                    .set_u32(names.wrapping_add(index.wrapping_mul(4)), hash);
                node = e.mem.u32(node + 4);
            }
            fn_00af6e50(e, Ptr::new(names), 0, count.wrapping_sub(1) as i32);
        }
        let index = e.mem.u32(directory_index);
        let directory = (index << 4).wrapping_add(e.get(archive, Archive::pDirectories));
        let mut file = 0u32;
        while file < e.mem.u32(directory + 8) {
            let entry = (file << 4).wrapping_add(e.mem.u32(directory + 0xc));
            if fn_00afb2a0(e, archive, Ptr::new(entry), false) {
                let compared = fn_00afcf00(e, pattern_hash, Ptr::new(entry));
                if compared == 0 {
                    let mut listed = false;
                    if count != 0 {
                        let mut low = 0u32;
                        let mut high = count;
                        let mut step = 1u32;
                        while step != 0 {
                            step = high.wrapping_sub(low) >> 1;
                            let middle = low.wrapping_add(step);
                            let name = e.mem.u32(names.wrapping_add(middle.wrapping_mul(4)));
                            let order = fn_00afce90(e, Ptr::new(entry), Ptr::new(name));
                            if order > 0 {
                                low = middle;
                            } else if order < 0 {
                                high = middle;
                            } else {
                                listed = true;
                                break;
                            }
                        }
                    }
                    if !listed {
                        let index = e.mem.u32(directory_index);
                        let name = fn_00af96d0(e, archive, index, file);
                        let prefix_length = e.call(STRLEN, &args![prefix]).u32();
                        let name_length = e.call(STRLEN, &args![name]).u32();
                        let size = prefix_length.wrapping_add(name_length).wrapping_add(1);
                        let block = allocate(e, size);
                        e.mem.set_u32(block_slot, block);
                        e.call(STRCPY_S, &args![block, size, prefix]);
                        e.call(STRCAT_S, &args![block, size, name]);
                        e.call(LIST_ADD_NAME, &args![list, block_slot]);
                    }
                } else if compared < 0 {
                    break;
                }
            }
            file += 1;
        }
    });
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    names
}

// Translated from 00af6ba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af6ba0 - the FS:[0] exception frame (the unwind state of the NiPointer per archive) and the /GS stack cookie check (00ec408c) are not translated
/// `ArchiveManager::AddToFileList` (Xbox PDB; cdecl, four words): adds the
/// archived files matching `pattern` (trimmed with `00b014b0`, hashed with
/// `00afd270`) to `list`, each prefixed with the directory part of `path`
/// (up to and including its last `\`). A `types` of 0xffff becomes the
/// pattern's archive type. Every listed archive of a matching type is asked
/// ([`fn_00af7060`]) with the number of non-empty entries of `list` and the
/// hash array the first call makes, which is freed at the end. Nothing
/// without an archive list.
pub fn archive_manager_add_to_file_list(
    e: &mut Engine,
    list: u32,
    pattern: Ptr,
    path: Ptr,
    types: u32,
) {
    if e.global::<u32>(ARCHIVE_LIST) == 0 {
        return;
    }
    let pattern = e.call(TRIM_FILE_NAME, &args![pattern]).u32();
    e.with_stack(0x114, |e, frame| {
        let prefix = frame.addr();
        let hashes = frame.addr() + 0x104;
        e.call(STRCPY_S, &args![prefix, 0x104u32, path]);
        let last = e.call(STRRCHR, &args![prefix, 0x5cu32]).u32();
        if last != 0 {
            e.mem.set_u8(last + 1, 0);
        }
        let mut types = types;
        if types == ANY_ARCHIVE_TYPE {
            types = archive_manager_get_archive_type_from_file_extension(e, Ptr::new(pattern));
        }
        let mut names = 0u32;
        e.call(HASH_PATH, &args![pattern, hashes, hashes + 8]);
        let mut count = 0u32;
        let mut node = list;
        while node != 0 {
            if e.mem.u32(node) != 0 {
                count += 1;
            }
            node = e.mem.u32(node + 4);
        }
        let mut node = e.global::<u32>(ARCHIVE_LIST);
        while node != 0 {
            let archive = e.mem.u32(node);
            if archive != 0 {
                add_ref(e, archive);
            }
            if archive != 0 && type_matches(e, archive, types) {
                names = fn_00af7060(
                    e,
                    Ptr::new(archive),
                    count,
                    names,
                    Ptr::new(hashes),
                    Ptr::new(hashes + 8),
                    list,
                    Ptr::new(prefix),
                );
            }
            node = e.mem.u32(node + 4);
            if archive != 0 {
                release(e, archive);
            }
        }
        if names != 0 {
            for index in 0..count {
                let hash = e.mem.u32(names.wrapping_add(index.wrapping_mul(4)));
                free(e, hash);
            }
            free(e, names);
        }
    });
}

/// The strings `00af4550` uses.
const TEXT_PLUGINS_FORMAT: u32 = 0x010a_436c;
const TEXT_EMPTY_THIRD: u32 = 0x010a_433b;
const TEXT_READ_MODE: u32 = 0x010a_437c;
const TEXT_ESP: u32 = 0x010a_4380;
const TEXT_ESM: u32 = 0x010a_4388;
const TEXT_BSA: u32 = 0x010a_4390;
const TEXT_DATA_PLUGIN: u32 = 0x010a_4398;
const TEXT_COMMA_SPACE: u32 = 0x010a_43a0;
const TEXT_COMMA_FIRST: u32 = 0x010a_43a4;
const TEXT_COMMA_NEXT: u32 = 0x010a_43a8;
/// Per archive type, the buffer size an archive of that type gets at
/// least (`011aced8`, nine words).
const TYPE_BUFFER_SIZES: u32 = 0x011a_ced8;
/// The size of the archive list buffer `00af4550` builds.
const MASTER_LIST_SIZE: u32 = 0x8000;

// Translated from 00af4550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af4550 - the FS:[0] exception frame (the unwind state of the NiPointer per archive) and the /GS stack cookie check (00ec408c) are not translated
/// `ArchiveManager::OpenMasterArchives` (Xbox PDB): answers whether every
/// listed archive opened (true at once when the byte `011acdf8` is clear).
/// Reads the invalidation file `011f8158` ([`fn_00af5ab0`]), sorts the 0x1a
/// archive type extensions at `011ace08` by name (selection sort,
/// `_stricmp`), copies the master archive list `011f815c` into a 0x8000-byte
/// block and appends `", " + <plugin>.bsa` for each plugin line of
/// `Plugins.txt` (`"%sPlugins.txt"` with `""`, opened `"r"` and never
/// closed; lines starting `#`, empty or `\n` are skipped; `.esp` or `.esm`
/// becomes `.bsa`) whose archive exists under `Data\` (a `\n` two from the
/// end of the list is cut first). Then each comma-separated name (`strtok_s`,
/// leading spaces, tabs and newlines skipped) is opened
/// ([`archive_manager_open_archive`]); a failure makes the answer false;
/// an opened archive becomes the archive of each of its 9 type bits that
/// has none yet (`011f8190`), and its buffer grows to that type's size
/// (`011aced8`) when smaller (`00affd80`). The block is freed; the elapsed
/// `GetTickCount` time is computed and not used.
pub fn archive_manager_open_master_archives(e: &mut Engine) -> bool {
    let mut opened_all = true;
    if e.global::<u8>(FLAG_011ACDF8) == 0 {
        return opened_all;
    }
    let invalidation = e.global::<u32>(STRING_011F8158);
    fn_00af5ab0(e, Ptr::new(invalidation));
    for index in 0..0x19u32 {
        let mut smallest = index;
        let mut other = index + 1;
        while other < 0x1a {
            let first = ARCHIVE_TYPE_TABLE + other * 8;
            let second = ARCHIVE_TYPE_TABLE + smallest * 8;
            if e.call(STRICMP, &args![first, second]).i32() < 0 {
                smallest = other;
            }
            other += 1;
        }
        if index != smallest {
            let (a, b) = (
                ARCHIVE_TYPE_TABLE + index * 8,
                ARCHIVE_TYPE_TABLE + smallest * 8,
            );
            let (name, kind) = (e.mem.u32(a), e.mem.u32(a + 4));
            let (other_name, other_kind) = (e.mem.u32(b), e.mem.u32(b + 4));
            e.mem.set_u32(a, other_name);
            e.mem.set_u32(a + 4, other_kind);
            e.mem.set_u32(b, name);
            e.mem.set_u32(b + 4, kind);
        }
    }
    let _start = e.call(GET_TICK_COUNT, &args![]).u32();
    e.with_stack(0x320, |e, frame| {
        let list_slot = frame.addr();
        let file_slot = frame.addr() + 4;
        let plugins_path = frame.addr() + 8;
        let archive_path = frame.addr() + 0x10c;
        let line = frame.addr() + 0x210;
        let list = allocate(e, MASTER_LIST_SIZE);
        e.mem.set_u32(list_slot, list);
        let master = e.global::<u32>(STRING_011F815C);
        e.call(STRCPY_S, &args![list, MASTER_LIST_SIZE, master]);
        e.call(
            PRINT_TO_BUFFER,
            &args![
                plugins_path,
                0x104u32,
                TEXT_PLUGINS_FORMAT,
                TEXT_EMPTY_THIRD
            ],
        );
        e.mem.set_u32(file_slot, 0);
        e.call(FOPEN_S, &args![file_slot, plugins_path, TEXT_READ_MODE]);
        if e.mem.u32(file_slot) != 0 {
            loop {
                let file = e.mem.u32(file_slot);
                if e.call(FEOF, &args![file]).u32() != 0 {
                    break;
                }
                let file = e.mem.u32(file_slot);
                e.call(FGETS, &args![line, 0x104u32, file]);
                let first = e.mem.u8(line);
                if first == b'#' || first == 0 || first == b'\n' {
                    continue;
                }
                let mut extension = e.call(STRSTR, &args![line, TEXT_ESP]).u32();
                if extension == 0 {
                    extension = e.call(STRSTR, &args![line, TEXT_ESM]).u32();
                }
                if extension == 0 {
                    continue;
                }
                let room = 0x104u32.wrapping_sub(extension.wrapping_sub(line));
                e.call(STRCPY_S, &args![extension, room, TEXT_BSA]);
                e.call(STRCPY_S, &args![archive_path, 0x104u32, TEXT_DATA_PLUGIN]);
                e.call(STRCAT_S, &args![archive_path, 0x104u32, line]);
                if e.call(ACCESS, &args![archive_path, 0u32]).i32() == -1 {
                    continue;
                }
                let list = e.mem.u32(list_slot);
                let length = e.call(STRLEN, &args![list]).u32();
                if e.mem.u8(list.wrapping_add(length).wrapping_sub(2)) == b'\n' {
                    let list = e.mem.u32(list_slot);
                    let length = e.call(STRLEN, &args![list]).u32();
                    e.mem.set_u8(list.wrapping_add(length).wrapping_sub(2), 0);
                }
                let list = e.mem.u32(list_slot);
                e.call(STRCAT_S, &args![list, 0x104u32, TEXT_COMMA_SPACE]);
                let list = e.mem.u32(list_slot);
                e.call(STRCAT_S, &args![list, 0x104u32, line]);
            }
        }
        let block = e.mem.u32(list_slot);
        let mut name = e.mem.u32(list_slot);
        e.call(STRTOK_S, &args![name, TEXT_COMMA_FIRST, list_slot]);
        while e.mem.u8(name) != 0 {
            while matches!(e.mem.u8(name), b' ' | b'\t' | b'\n') {
                name = name.wrapping_add(1);
            }
            let archive = archive_manager_open_archive(e, Ptr::new(name), 0, 0);
            if archive != 0 {
                add_ref(e, archive);
            }
            if archive == 0 {
                opened_all = false;
            } else {
                for kind in 0..9u32 {
                    let types = u32::from(e.mem.u16(archive + Archive::iArchiveType.off));
                    let slot = ARCHIVE_POINTER_ROWS + kind * 4;
                    if (types & (1 << kind)) & 0xffff == 0 || e.mem.u32(slot) != 0 {
                        continue;
                    }
                    fn_00afd1b0(e, Ptr::new(slot), archive);
                    let wanted = e.mem.u32(TYPE_BUFFER_SIZES + kind * 4);
                    if wanted > e.mem.u32(archive + NiFile::m_uiBufferAllocSize.off) {
                        let wanted = e.mem.u32(TYPE_BUFFER_SIZES + kind * 4);
                        e.call(BSFILE_SET_BUFFER_SIZE, &args![archive, wanted]);
                    }
                }
            }
            name = e.mem.u32(list_slot);
            e.call(STRTOK_S, &args![name, TEXT_COMMA_NEXT, list_slot]);
            if archive != 0 {
                release(e, archive);
            }
        }
        free(e, block);
    });
    let _end = e.call(GET_TICK_COUNT, &args![]).u32();
    opened_all
}

/// The strings `00af4be0` matches archive names against: the copies for the
/// new name and for the listed names (each `"Fallo"`, `"DeadM"`,
/// `"Hones"`, `"Lones"`, `"OldWo"`, `"Updat"`), and its two warnings.
const RANK_NAMES: [u32; 6] = [
    0x010a_43ac,
    0x010a_43b4,
    0x010a_43bc,
    0x010a_43c4,
    0x010a_43cc,
    0x010a_43d4,
];
const LISTED_RANK_NAMES: [u32; 6] = [
    0x010a_442c,
    0x010a_4434,
    0x010a_443c,
    0x010a_4444,
    0x010a_444c,
    0x010a_4454,
];
const TEXT_USER_ARCHIVE_FIRST: u32 = 0x010a_43dc;
const TEXT_USER_ARCHIVE_SECOND: u32 = 0x010a_4404;
/// `00aa13e0` (cdecl, one word): the Gamebryo allocation the archive is
/// made in (0x1d0 bytes).
const GAMEBRYO_ALLOCATE: u32 = 0x00aa_13e0;
/// `00b011b0` (thiscall on `00aff100`'s object, one word: a path) and that
/// object's vtable +0x18 (path, out): the path redirection `00af4be0`
/// applies.
const REDIRECTS_PATH: u32 = 0x00b0_11b0;
const REDIRECT_SLOT: u32 = 0x18;

// Translated from 00af4be0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af4be0 - the FS:[0] exception frame (the unwind states of the archive and the NiPointer temporaries) and the /GS stack cookie check (00ec408c) are not translated
/// `ArchiveManager::OpenArchive` (Xbox PDB; cdecl, three words): opens the
/// archive `name` and puts it in the archive list; answers it, or null
/// (also while the byte `011acdf8` is clear, or when the finder,
/// `00afe0d0(name, found, 1, -1)`, does not find it). The found path may be
/// redirected (`00aff100`'s `00b011b0`, then its vtable +0x18). The
/// `Archive` (0x1d0 bytes from `00aa13e0`, [`fn_00af7eb0`] with buffer size
/// 0x40000) is deleted again when it is not good or flagged invalid
/// (`cArchiveFlags` bit 0). The list (`011f8160`) is made when missing.
/// The name ranks the archive: `Fallo` 0, `DeadM` 1, `Hones` 2, `Lones` 3,
/// `OldWo` 4, `Updat` 5, anything else (or an empty name) 6 with a
/// warning. Rank 0 goes to the end of the list; any other goes in front of
/// the first listed archive whose name says it ranks before it (or at the
/// front when none does), as the exe's thresholds decide. A nonzero
/// `types` replaces the archive's type bits.
pub fn archive_manager_open_archive(e: &mut Engine, name: Ptr, types: u16, flag: u8) -> u32 {
    if e.global::<u8>(FLAG_011ACDF8) == 0 {
        return 0;
    }
    e.with_stack(0x20c, |e, frame| {
        let found = frame.addr();
        let path = frame.addr() + 0x104;
        let temporary = frame.addr() + 0x208;
        e.call(STRCPY_S, &args![found, 0x104u32, name]);
        let finder = e.global::<u32>(FILE_FINDER);
        let located = if finder == 0 {
            0
        } else {
            e.call(
                FILE_FINDER_LOOKUP,
                &args![finder, name, found, 1u32, u32::MAX],
            )
            .u32()
        };
        if located == 0 {
            return 0;
        }
        let owner = e.call(STATE_OWNER, &args![]).u32();
        if e.call(REDIRECTS_PATH, &args![owner, found]).bool() {
            let owner = e.call(STATE_OWNER, &args![]).u32();
            e.vcall(owner, REDIRECT_SLOT, &args![found, path]);
        } else {
            e.call(STRCPY_S, &args![path, 0x104u32, found]);
        }
        let block = e.call(GAMEBRYO_ALLOCATE, &args![0x1d0u32]).u32();
        let archive = if block == 0 {
            0
        } else {
            fn_00af7eb0(e, Ptr::new(block), Ptr::new(path), 0x40000, 0, flag).addr()
        };
        if archive == 0
            || e.mem.u8(archive + NiFile::m_bGood.off) == 0
            || e.mem.u8(archive + Archive::cArchiveFlags.off) & 1 != 0
        {
            if archive != 0 {
                e.vcall(archive, DELETING_DESTRUCTOR_SLOT, &args![1u32]);
            }
            return 0;
        }
        if e.global::<u32>(ARCHIVE_LIST) == 0 {
            let node = allocate(e, 8);
            let list = if node == 0 {
                0
            } else {
                e.mem.set_u32(node, 0);
                let held = e.mem.u32(node);
                if held != 0 {
                    add_ref(e, held);
                }
                e.mem.set_u32(node + 4, 0);
                node
            };
            e.set_global(ARCHIVE_LIST, list);
        }
        let rank: i32 = if name.is_null() || e.call(STRLEN, &args![name]).u32() == 0 {
            e.call(WARNING, &args![TEXT_USER_ARCHIVE_SECOND]);
            6
        } else {
            let mut rank = 6;
            for (index, text) in RANK_NAMES.iter().enumerate() {
                if e.call(STRSTR, &args![name, *text]).u32() != 0 {
                    rank = index as i32;
                    break;
                }
            }
            if rank == 6 {
                e.call(WARNING, &args![TEXT_USER_ARCHIVE_FIRST]);
            }
            rank
        };
        let insert = |e: &mut Engine, node: u32, tail: bool| {
            e.mem.set_u32(temporary, archive);
            if archive != 0 {
                add_ref(e, archive);
            }
            if tail {
                fn_00afcc10(e, Ptr::new(node), Ptr::new(temporary));
            } else {
                fn_00afc880(e, Ptr::new(node), Ptr::new(temporary));
            }
            let held = e.mem.u32(temporary);
            if held != 0 {
                release(e, held);
            }
        };
        if rank <= 0 {
            let list = e.global::<u32>(ARCHIVE_LIST);
            insert(e, list, true);
        } else {
            let mut node = e.global::<u32>(ARCHIVE_LIST);
            let mut placed = false;
            while node != 0 && e.mem.u32(node) != 0 {
                let listed_name = e.mem.u32(node).wrapping_add(BSFile::pFileName.off);
                if listed_name != 0 && e.call(STRLEN, &args![listed_name]).u32() != 0 {
                    let has = |e: &mut Engine, which: usize| {
                        e.call(STRSTR, &args![listed_name, LISTED_RANK_NAMES[which]])
                            .u32()
                            != 0
                    };
                    placed = has(e, 0)
                        || has(e, 1)
                        || (has(e, 2) && rank > 1)
                        || (has(e, 3) && rank > 2)
                        || (has(e, 4) && rank > 3)
                        || (has(e, 5) && rank > 3);
                }
                if placed {
                    insert(e, node, false);
                    break;
                }
                node = e.mem.u32(node + 4);
            }
            if !placed {
                let list = e.global::<u32>(ARCHIVE_LIST);
                insert(e, list, false);
            }
        }
        if types != 0 {
            e.mem.set_u16(archive + Archive::iArchiveType.off, types);
        }
        archive
    })
}

/// The archive run-time id counter (`011f81d8`).
const NEXT_RUNTIME_ID: u32 = 0x011f_81d8;
/// The BSA tag `"BSA\0"` and the newest version this code reads.
const BSA_TAG: u32 = 0x0041_5342;
const BSA_VERSION: u32 = 0x68;
/// `00af7c40` (cdecl: directory entry, file entry, the archive's type
/// bits): run for every file when `00af7eb0`'s last word is set.
const PER_FILE: u32 = 0x00af_7c40;
/// The `BSFile` word at +0x38 (the read position the platform stand-in
/// keeps) that `00af7eb0` takes the file name table's offset from.
const BSFILE_READ_POSITION: u32 = 0x38;
/// `_stat64i32`'s buffer size and its `st_mtime` (8 bytes at +0x20).
const STAT_SIZE: u32 = 0x30;
const STAT_MTIME: u32 = 0x20;

/// Allocates `count` 16-byte entries (a size that overflows becomes
/// 0xffffffff) and clears the words at +8 and +0xc of each, counting down
/// while the count stays non-negative as a signed value; null when the
/// allocation fails.
fn allocate_entries(e: &mut Engine, count: u32) -> u32 {
    let size = count.checked_mul(16).unwrap_or(u32::MAX);
    let block = allocate(e, size);
    if block == 0 {
        return 0;
    }
    let mut left = count;
    let mut entry = block;
    loop {
        left = left.wrapping_sub(1);
        if (left as i32) < 0 {
            break;
        }
        e.mem.set_u32(entry + 8, 0);
        e.mem.set_u32(entry + 0xc, 0);
        entry = entry.wrapping_add(0x10);
    }
    block
}

/// `count * 4` with the exe's all-ones on overflow (`MUL`, `SETO`, `NEG`,
/// `OR`).
fn words(count: u32) -> u32 {
    count.checked_mul(4).unwrap_or(u32::MAX)
}

/// Sets or clears `bit` of `cArchiveFlags` the way the exe's
/// constant-tested pairs do: `set` is the constant the exe tests.
fn update_archive_flag(e: &mut Engine, this: Ptr<Archive>, bit: u8, set: bool) {
    let flags = e.get(this, Archive::cArchiveFlags);
    if set {
        e.set(this, Archive::cArchiveFlags, flags | bit);
    } else {
        e.set(this, Archive::cArchiveFlags, flags & !bit);
    }
}

// Translated from 00af7eb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af7eb0 - the FS:[0] exception frame (the unwind states of the base parts) and the /GS stack cookie check (00ec408c) are not translated
/// Unnamed in the engine map: the `Archive` constructor (thiscall, four
/// words). Builds the header ([`fn_00afd8b0`], no directories), the `BSFile`
/// part (`00b00260(name, 0, buffer_size, 0)`), the `NiRefObject` part
/// (count 0, `NiRefObject`'s object count raised), the `Archive` vtables and
/// its critical section; takes the next run-time id; clears the last
/// directory and file (-1), the tables and the flags; opens the file
/// (`BSFile::Open(0, 0)`). Nothing more happens to a file that is not good.
/// `headers_only` sets bit 3 of the flags. With the plain read pair
/// (`00aff990(0)`) the 0x24-byte header is read; a tag other than `BSA\0`
/// or a version above 0x68 sets bit 0 (invalid) and stops; a version below
/// 0x68 clears `oFlags` bit 8. Unless bit 3 is set, the directories are
/// read (16 bytes each), with directory names (`oFlags` bit 0) and file
/// names (bit 1) kept in tables when [`fn_00afd020`] says so (flag bits 4
/// and 5), each directory's name read (into the table or a scratch buffer)
/// and its file entries read; the file name table's offset is the `BSFile`
/// word at +0x38 (or `iTrueFilePos` when that is -1), and the names are
/// read and indexed when kept. The archive's file time is `_stat64i32`'s
/// `st_mtime`. Unless bit 3 is set, the invalidation is applied
/// ([`fn_00afad00`], when the byte `011acdf9` is set) and with `per_file`
/// `00af7c40` runs for every file. Finally the buffer is set to 0x4000
/// bytes (`00affd80`) and the size read (vtable +0x28). Returns `this`.
///
/// The exe's flag updates each hold a second branch behind a constant
/// test; they are kept (`update_archive_flag`).
pub fn fn_00af7eb0(
    e: &mut Engine,
    this: Ptr<Archive>,
    name: Ptr,
    buffer_size: u32,
    headers_only: u8,
    per_file: u8,
) -> Ptr<Archive> {
    let header = this.addr() + Archive::iTag.off;
    fn_00afd8b0(e, Ptr::new(header));
    e.set(this, Archive::pDirectories, 0);
    e.call(
        BSFILE_CONSTRUCTOR,
        &args![this, name, 0u32, buffer_size, 0u32],
    );
    e.set(this, Archive::ref_object_vtable, REF_OBJECT_VTABLE);
    e.set(this, Archive::m_uiRefCount, 0);
    e.call(INTERLOCKED_INCREMENT, &args![REF_OBJECT_COUNT]);
    e.mem.set_u32(this.addr(), ARCHIVE_VTABLE);
    e.set(this, Archive::ref_object_vtable, ARCHIVE_REF_OBJECT_VTABLE);
    let section = this.addr() + Archive::ArchiveCriticalSection.off;
    e.call(INITIALIZE_CRITICAL_SECTION, &args![section]);
    let id = e.global::<u32>(NEXT_RUNTIME_ID);
    e.set(this, Archive::m_iRuntimeID, id);
    let next = e.global::<u32>(NEXT_RUNTIME_ID).wrapping_add(1);
    e.set_global(NEXT_RUNTIME_ID, next);
    e.set(this, Archive::iLastDirectoryIndex, u32::MAX);
    e.set(this, Archive::iLastFileIndex, u32::MAX);
    e.set(this, Archive::pDirectories, 0);
    e.set(this, Archive::pDirectoryStringArray, 0);
    e.set(this, Archive::pDirectoryStringOffsets, 0);
    e.set(this, Archive::pFileNameStringArray, 0);
    e.set(this, Archive::pFileNameStringOffsets, 0);
    e.set(this, Archive::cArchiveFlags, 0);
    e.set(this, Archive::iFileNameArrayOffset, 0);
    e.call(BSFILE_OPEN, &args![this, 0u32, 0u32]);
    if e.mem.u8(this.addr() + NiFile::m_bGood.off) == 0 {
        return this;
    }
    if headers_only != 0 {
        update_archive_flag(e, this, 8, true);
    }
    e.call(BSFILE_SET_ENDIAN_SWAP, &args![this, 0u32]);
    let header = if this.is_null() { 0 } else { header };
    read_through_pointer(e, this, header, 0x24);
    let valid =
        e.get(this, Archive::iTag) == BSA_TAG && e.get(this, Archive::iVersion) <= BSA_VERSION;
    if !valid {
        update_archive_flag(e, this, 1, true);
        return this;
    }
    if e.get(this, Archive::iVersion) < BSA_VERSION {
        // `XOR EDX,EDX; JZ`: the clearing branch always runs.
        let set = false;
        let flags = e.get(this, Archive::oFlags);
        let flags = if set { flags | 0x100 } else { flags & !0x100 };
        e.set(this, Archive::oFlags, flags);
    }
    if e.get(this, Archive::cArchiveFlags) & 8 == 0 {
        read_tables(e, this);
    }
    e.with_stack(STAT_SIZE, |e, stat| {
        let file_name = this.addr() + BSFile::pFileName.off;
        e.call(STAT64I32, &args![file_name, stat]);
        let low = e.mem.u32(stat.addr() + STAT_MTIME);
        let high = e.mem.u32(stat.addr() + STAT_MTIME + 4);
        e.mem
            .set_u32(this.addr() + Archive::ArchiveFileTime.off, low);
        e.mem
            .set_u32(this.addr() + Archive::ArchiveFileTime.off + 4, high);
    });
    if e.get(this, Archive::cArchiveFlags) & 8 == 0 {
        if e.global::<u8>(FLAG_011ACDF9) != 0 {
            let _start = e.call(GET_TICK_COUNT, &args![]).u32();
            fn_00afad00(e, this);
            let _end = e.call(GET_TICK_COUNT, &args![]).u32();
        }
        if per_file != 0 {
            let mut directory = 0u32;
            while directory < e.get(this, Archive::iDirectories) {
                let mut file = 0u32;
                loop {
                    let entries = e.get(this, Archive::pDirectories);
                    let entry = entries.wrapping_add(directory << 4);
                    if file >= e.mem.u32(entry + 8) {
                        break;
                    }
                    let types = u32::from(e.get(this, Archive::iArchiveType));
                    let file_entry = (file << 4).wrapping_add(e.mem.u32(entry + 0xc));
                    e.call(PER_FILE, &args![entry, file_entry, types]);
                    file += 1;
                }
                directory += 1;
            }
        }
    }
    e.call(BSFILE_SET_BUFFER_SIZE, &args![this, 0x4000u32]);
    e.vcall(this.addr(), GET_SIZE_SLOT, &args![]);
    this
}

/// The table reading of `00af7eb0` (its middle, inlined there): see its
/// doc comment.
fn read_tables(e: &mut Engine, this: Ptr<Archive>) {
    let _start = e.call(GET_TICK_COUNT, &args![]).u32();
    let old = e.get(this, Archive::pDirectories);
    free(e, old);
    let count = e.get(this, Archive::iDirectories);
    let directories = allocate_entries(e, count);
    e.set(this, Archive::pDirectories, directories);
    let count = e.get(this, Archive::iDirectories);
    let directories = e.get(this, Archive::pDirectories);
    read_through_pointer(e, this, directories, count << 4);
    if e.get(this, Archive::oFlags) & 1 != 0 && fn_00afd020(e, this) {
        update_archive_flag(e, this, 0x10, true);
        let old = e.get(this, Archive::pDirectoryStringArray);
        free(e, old);
        let old = e.get(this, Archive::pDirectoryStringOffsets);
        free(e, old);
        let length = e.get(this, Archive::iDirectoryNamesLength);
        let block = allocate(e, length);
        e.set(this, Archive::pDirectoryStringArray, block);
        let count = e.get(this, Archive::iDirectories);
        let block = allocate(e, words(count));
        e.set(this, Archive::pDirectoryStringOffsets, block);
    }
    if e.get(this, Archive::oFlags) & 2 != 0 && fn_00afd020(e, this) {
        update_archive_flag(e, this, 0x20, true);
        let old = e.get(this, Archive::pFileNameStringArray);
        free(e, old);
        if e.get(this, Archive::pFileNameStringOffsets) != 0 {
            let mut index = 0u32;
            while index < e.get(this, Archive::iDirectories) {
                let offsets = e.get(this, Archive::pFileNameStringOffsets);
                let row = e.mem.u32(offsets.wrapping_add(index.wrapping_mul(4)));
                free(e, row);
                index += 1;
            }
            let offsets = e.get(this, Archive::pFileNameStringOffsets);
            free(e, offsets);
        }
        let length = e.get(this, Archive::iFileNamesLength);
        let block = allocate(e, length);
        e.set(this, Archive::pFileNameStringArray, block);
        let count = e.get(this, Archive::iDirectories);
        let block = allocate(e, words(count));
        e.set(this, Archive::pFileNameStringOffsets, block);
    }
    e.with_stack(0x108, |e, frame| {
        let length_byte = frame.addr();
        let scratch = frame.addr() + 4;
        let mut name_offset = 0u32;
        let mut directory = 0u32;
        while directory < e.get(this, Archive::iDirectories) {
            if e.get(this, Archive::oFlags) & 1 != 0 {
                read_through_pointer(e, this, length_byte, 1);
                let length = u32::from(e.mem.u8(length_byte));
                if fn_00afd020(e, this) {
                    let target = e
                        .get(this, Archive::pDirectoryStringArray)
                        .wrapping_add(name_offset);
                    read_through_pointer(e, this, target, length);
                    let offsets = e.get(this, Archive::pDirectoryStringOffsets);
                    e.mem
                        .set_u32(offsets.wrapping_add(directory.wrapping_mul(4)), name_offset);
                    name_offset = length.wrapping_add(name_offset);
                } else {
                    read_through_pointer(e, this, scratch, length);
                }
            }
            let entries = e.get(this, Archive::pDirectories);
            let count = e
                .mem
                .u32(entries.wrapping_add(directory << 4).wrapping_add(8));
            let files = allocate_entries(e, count);
            let entries = e.get(this, Archive::pDirectories);
            let count = e
                .mem
                .u32(entries.wrapping_add(directory << 4).wrapping_add(8));
            read_through_pointer(e, this, files, count << 4);
            let entry = (directory << 4).wrapping_add(e.get(this, Archive::pDirectories));
            e.mem.set_u32(entry + 0xc, files);
            directory += 1;
        }
    });
    let mut file_name_offset = 0u32;
    if e.get(this, Archive::oFlags) & 2 != 0 {
        let position = e.mem.u32(this.addr() + BSFILE_READ_POSITION);
        let start = if position != u32::MAX {
            position
        } else {
            e.get(this.cast::<BSFile>(), BSFile::iTrueFilePos)
        };
        e.set(this, Archive::iFileNameArrayOffset, start);
        if fn_00afd020(e, this) {
            let strings = e.get(this, Archive::pFileNameStringArray);
            let length = e.get(this, Archive::iFileNamesLength);
            read_through_pointer(e, this, strings, length);
            let mut directory = 0u32;
            while directory < e.get(this, Archive::iDirectories) {
                let entries = e.get(this, Archive::pDirectories);
                let count = e
                    .mem
                    .u32(entries.wrapping_add(directory << 4).wrapping_add(8));
                let row = allocate(e, words(count));
                let offsets = e.get(this, Archive::pFileNameStringOffsets);
                e.mem
                    .set_u32(offsets.wrapping_add(directory.wrapping_mul(4)), row);
                let mut file = 0u32;
                loop {
                    let entries = e.get(this, Archive::pDirectories);
                    let count = e
                        .mem
                        .u32(entries.wrapping_add(directory << 4).wrapping_add(8));
                    if file >= count {
                        break;
                    }
                    let offsets = e.get(this, Archive::pFileNameStringOffsets);
                    let row = e.mem.u32(offsets.wrapping_add(directory.wrapping_mul(4)));
                    e.mem
                        .set_u32(row.wrapping_add(file.wrapping_mul(4)), file_name_offset);
                    let name = e
                        .get(this, Archive::pFileNameStringArray)
                        .wrapping_add(file_name_offset);
                    let length = e.call(STRLEN, &args![name]).u32();
                    file_name_offset = file_name_offset.wrapping_add(length).wrapping_add(1);
                    file += 1;
                }
                directory += 1;
            }
        }
    }
    let _end = e.call(GET_TICK_COUNT, &args![]).u32();
}

/// `00afac20` (archive.cpp, unnamed in the engine map; `thiscall` on the archive, three
/// words): what [`fn_00af7c40`] runs for each archive whose type bits meet its mask.
const ARCHIVE_PER_FILE: u32 = 0x00af_ac20;

// Translated from 00af7c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af7c40 - the FS:[0] exception frame (the unwind state of the NiPointer per archive) is not translated
/// Unnamed in the engine map (cdecl, three words): under the archive list's critical
/// section (`011f8170`), walks the list of opened archives (`011f8160`) and runs `00afac20`
/// with `first`, `second` and 0 on each archive whose type bits (`iArchiveType`) meet
/// `types`, holding a reference to the archive while it does. The walk stops at a node
/// whose item and next pointer are both null (an empty list head). As in the exe, a null
/// item in a node that is not empty still has its type bits read.
pub fn fn_00af7c40(e: &mut Engine, first: u32, second: u32, types: u32) {
    e.call(ENTER_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
    let mut node = e.global::<u32>(ARCHIVE_LIST);
    while node != 0 {
        let empty = e.mem.u32(node + 4) == 0 && e.mem.u32(node) == 0;
        if empty {
            break;
        }
        let archive = e.mem.u32(node);
        if archive != 0 {
            add_ref(e, archive);
        }
        if type_matches(e, archive, types) {
            e.call(ARCHIVE_PER_FILE, &args![archive, first, second, 0u32]);
        }
        node = e.mem.u32(node + 4);
        if archive != 0 {
            release(e, archive);
        }
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
}

/// The single `NiPointer<Archive>` `00af5550` clears besides the two rows (`011f8188`).
const ARCHIVE_POINTER_011F8188: u32 = 0x011f_8188;
/// `BSRandom` (`BSCore/bsrandom.cpp`): the generator `00af7400` draws from (`011c4180`), its
/// construction guard word (bit 0 at `011c4b4c`), its constructor (`00aa5140`, thiscall) and
/// `BSRandom::UnsignedInt` (`00aa5230`, thiscall, one word: the bound).
const RANDOM_GENERATOR: u32 = 0x011c_4180;
const RANDOM_GENERATOR_GUARD: u32 = 0x011c_4b4c;
const BS_RANDOM_CONSTRUCTOR: u32 = 0x00aa_5140;
const BS_RANDOM_UNSIGNED_INT: u32 = 0x00aa_5230;
/// The memory manager's reallocation (`00aa4150`, thiscall: block, size).
const MEMORY_MANAGER_REALLOCATE: u32 = 0x00aa_4150;
/// `BSSimpleArray<BSHash*,1024>`'s vtable (`010a469c`), its initial set-up (`00afcde0`,
/// thiscall: reserve, size) and the clear its destructor runs (`00a1bfa0`, thiscall, one
/// word; the engine map names it `BSSimpleArray<ButtonIcon,1024>::Clear`, a folded body).
const HASH_ARRAY_VTABLE: u32 = 0x010a_469c;
const HASH_ARRAY_INITIAL_SETUP: u32 = 0x00af_cde0;
const SIMPLE_ARRAY_CLEAR_00A1BFA0: u32 = 0x00a1_bfa0;
/// The `Archive` vtable slot (+0x18) that answers its name, and the `BSFile` slots the
/// `ArchiveFile` reads go through on the archive: the seek (+0x14), and the two transfers
/// (+0x2c, +0x30; buffer and size).
const NAME_SLOT: u32 = 0x18;
const TRANSFER_SLOT_2C: u32 = 0x2c;
const TRANSFER_SLOT_30: u32 = 0x30;
/// The `BSFile` word at +0x38 the `ArchiveFile` transfers read as a position (-1 when the
/// true file position `iTrueFilePos` holds instead); not named here.
const POSITION_OVERRIDE: u32 = 0x38;

/// The address of `NiPointer` `index` of row `row` of the two rows at `011f8190`.
fn archive_pointer(row: u32, index: u32) -> u32 {
    ARCHIVE_POINTER_ROWS
        .wrapping_add(row.wrapping_mul(ARCHIVE_POINTER_ROW_SIZE))
        .wrapping_add(index.wrapping_mul(4))
}

// Translated from 00af4390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): answers false.
pub fn fn_00af4390(_e: &mut Engine) -> bool {
    false
}

// Translated from 00af44f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): gives the two string copies `011f8158` and
/// `011f815c` back to the memory manager (each when not null; the words are not cleared).
pub fn fn_00af44f0(e: &mut Engine) {
    if e.global::<u32>(STRING_011F8158) != 0 {
        let block = e.global::<u32>(STRING_011F8158);
        free(e, block);
    }
    if e.global::<u32>(STRING_011F815C) != 0 {
        let block = e.global::<u32>(STRING_011F815C);
        free(e, block);
    }
}

// Translated from 00af5480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): under the archive list's lock, adds
/// `archive` at the end of the list `011f8160` ([`fn_00afcc10`] with an `NiPointer`
/// temporary holding a reference) when there is a list.
// Translation incomplete: 00af5480 - its FS:[0] exception frame (unwinding the NiPointer temporary) is not translated
pub fn fn_00af5480(e: &mut Engine, archive: u32) {
    e.call(ENTER_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
    if e.global::<u32>(ARCHIVE_LIST) != 0 {
        e.with_stack(4, |e, temporary| {
            e.mem.set_u32(temporary.addr(), archive);
            let held = e.mem.u32(temporary.addr());
            if held != 0 {
                add_ref(e, held);
            }
            let head = e.global::<u32>(ARCHIVE_LIST);
            fn_00afcc10(e, Ptr::new(head), temporary);
            let held = e.mem.u32(temporary.addr());
            if held != 0 {
                release(e, held);
            }
        });
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
}

// Translated from 00af5550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): takes `archive` out. False without a list or
/// an archive; otherwise, under the archive list's lock, removes it from the list
/// ([`fn_00afc960`] with an `NiPointer` temporary), clears every pointer to it in the two
/// rows at `011f8190` and in `011f8188` ([`fn_00afd1b0`] with null) and answers true.
// Translation incomplete: 00af5550 - its FS:[0] exception frame (unwinding the NiPointer temporary) is not translated
pub fn fn_00af5550(e: &mut Engine, archive: u32) -> bool {
    if e.global::<u32>(ARCHIVE_LIST) == 0 || archive == 0 {
        return false;
    }
    e.call(ENTER_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
    e.with_stack(4, |e, temporary| {
        e.mem.set_u32(temporary.addr(), archive);
        let held = e.mem.u32(temporary.addr());
        if held != 0 {
            add_ref(e, held);
        }
        let head = e.global::<u32>(ARCHIVE_LIST);
        fn_00afc960(e, Ptr::new(head), temporary);
        let held = e.mem.u32(temporary.addr());
        if held != 0 {
            release(e, held);
        }
    });
    for i in 0..9u32 {
        if e.mem.u32(archive_pointer(0, i)) == archive {
            fn_00afd1b0(e, Ptr::new(archive_pointer(0, i)), 0);
        }
        if e.mem.u32(archive_pointer(1, i)) == archive {
            fn_00afd1b0(e, Ptr::new(archive_pointer(1, i)), 0);
        }
    }
    if e.global::<u32>(ARCHIVE_POINTER_011F8188) == archive {
        fn_00afd1b0(e, Ptr::new(ARCHIVE_POINTER_011F8188), 0);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![ARCHIVE_LIST_LOCK]);
    true
}

// Translated from 00af56d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): false without a list or an archive;
/// otherwise takes `archive` out ([`fn_00af5550`]) and answers true.
pub fn fn_00af56d0(e: &mut Engine, archive: u32) -> bool {
    if e.global::<u32>(ARCHIVE_LIST) == 0 || archive == 0 {
        return false;
    }
    fn_00af5550(e, archive);
    true
}

// Translated from 00af5700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): takes out the opened archive named `name`.
/// With a list, `name` is copied into a `char[0x104]` local, which the `FileFinder` lookup
/// (`00afe0d0(name, local, 1, -1)`, none without a finder) rewrites; when it answers
/// non-zero, the first archive of the list whose name (vtable +0x18) equals the local
/// (`_stricmp`) is taken out ([`fn_00af5550`]) and the answer is true; false otherwise.
// Translation incomplete: 00af5700 - its /GS stack cookie check is not translated
pub fn fn_00af5700(e: &mut Engine, name: u32) -> bool {
    if e.global::<u32>(ARCHIVE_LIST) == 0 {
        return false;
    }
    e.with_stack(0x10c, |e, local| {
        let local = local.addr();
        e.call(STRCPY_S, &args![local, 0x104u32, name]);
        let finder = e.global::<u32>(FILE_FINDER);
        let found = if finder != 0 {
            e.call(
                FILE_FINDER_LOOKUP,
                &args![finder, name, local, 1u32, 0xffff_ffffu32],
            )
            .u32()
        } else {
            0
        };
        if found == 0 {
            return false;
        }
        let mut node = e.global::<u32>(ARCHIVE_LIST);
        while node != 0 {
            let archive = e.mem.u32(node);
            if archive != 0 {
                let own = e.vcall(archive, NAME_SLOT, &args![]).u32();
                if e.call(STRICMP, &args![own, local]).i32() == 0 {
                    fn_00af5550(e, archive);
                    return true;
                }
            }
            node = e.mem.u32(node + 4);
        }
        false
    })
}

// Translated from 00af5e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArchiveManager::InvalidateFile` (Xbox PDB; cdecl, one word): with a list, trims the path
/// (`TrimFileName`), hashes its directory and file (`00afd270`) and asks each archive of the
/// list in turn (holding a reference) to invalidate the file ([`fn_00afac20`]); answers
/// whether one did.
// Translation incomplete: 00af5e40 - its FS:[0] exception frame (unwinding the held reference) is not translated
pub fn archive_manager_invalidate_file(e: &mut Engine, path: u32) -> bool {
    if e.global::<u32>(ARCHIVE_LIST) == 0 {
        return false;
    }
    let path = e.call(TRIM_FILE_NAME, &args![path]).u32();
    e.with_stack(0x10, |e, hashes| {
        let directory = hashes.addr();
        let file = directory + 8;
        e.call(HASH_PATH, &args![path, directory, file]);
        let mut node = e.global::<u32>(ARCHIVE_LIST);
        while node != 0 {
            let archive = e.mem.u32(node);
            if archive != 0 {
                add_ref(e, archive);
            }
            if archive != 0
                && fn_00afac20(
                    e,
                    Ptr::new(archive),
                    Ptr::new(directory),
                    Ptr::new(file),
                    path,
                )
            {
                if archive != 0 {
                    release(e, archive);
                }
                return true;
            }
            node = e.mem.u32(node + 4);
            if archive != 0 {
                release(e, archive);
            }
        }
        false
    })
}

// Translated from 00af6430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words): the file entry ([`fn_00afa6e0`] with the
/// last three words) in the archive of slot `index`: the first row's (`011f8190`) when it is
/// set, else the second row's; 0 without an archive. A reference is held around the lookup.
// Translation incomplete: 00af6430 - its FS:[0] exception frame (unwinding the held reference) is not translated
pub fn fn_00af6430(e: &mut Engine, index: u32, directory: u32, file: u32, name: u32) -> u32 {
    let mut result = 0;
    let slot = if e.mem.u32(archive_pointer(0, index)) != 0 {
        archive_pointer(0, index)
    } else {
        archive_pointer(1, index)
    };
    let archive = e.mem.u32(slot);
    if archive != 0 {
        add_ref(e, archive);
    }
    if archive != 0 {
        result = fn_00afa6e0(
            e,
            Ptr::new(archive),
            Ptr::new(directory),
            Ptr::new(file),
            Ptr::new(name),
        );
    }
    if archive != 0 {
        release(e, archive);
    }
    result
}

// Translated from 00af6690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words): the first file entry
/// ([`fn_00afa6e0`] with the last three words) found in an archive of the list whose
/// `iArchiveType` shares a bit with the low 16 bits of `types`, each archive held by a
/// reference while it is asked; 0 when none has it.
// Translation incomplete: 00af6690 - its FS:[0] exception frame (unwinding the held reference) is not translated
pub fn fn_00af6690(e: &mut Engine, types: u32, directory: u32, file: u32, name: u32) -> u32 {
    let mut node = e.global::<u32>(ARCHIVE_LIST);
    while node != 0 {
        let archive = e.mem.u32(node);
        if archive != 0 {
            add_ref(e, archive);
        }
        if archive != 0 {
            let kind = u32::from(e.get(Ptr::<Archive>::new(archive), Archive::iArchiveType));
            if kind & types & 0xffff != 0 {
                let entry = fn_00afa6e0(
                    e,
                    Ptr::new(archive),
                    Ptr::new(directory),
                    Ptr::new(file),
                    Ptr::new(name),
                );
                if entry != 0 {
                    if archive != 0 {
                        release(e, archive);
                    }
                    return entry;
                }
            }
        }
        node = e.mem.u32(node + 4);
        if archive != 0 {
            release(e, archive);
        }
    }
    0
}

// Translated from 00af67d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): the name of the file of `entry` in the
/// archive of slot `index` (the second row of `011f8190` when bit 31 of the entry's word
/// +0xc is set, the first otherwise): with an archive, under its lock (+0x19c), the name
/// [`fn_00af9ba0`] finds is copied into `out` (`strcpy_s`, 0x104); answers whether there was
/// one. A reference is held around it.
// Translation incomplete: 00af67d0 - its FS:[0] exception frame (unwinding the held reference) is not translated
pub fn fn_00af67d0(e: &mut Engine, index: u32, entry: u32, out: u32) -> bool {
    let mut found = false;
    let row = u32::from(e.mem.u32(entry.wrapping_add(0xc)) & 0x8000_0000 != 0);
    let archive = e.mem.u32(archive_pointer(row, index));
    if archive != 0 {
        add_ref(e, archive);
    }
    if archive != 0 {
        e.call(ENTER_CRITICAL_SECTION, &args![archive + 0x19c]);
        let name = fn_00af9ba0(e, Ptr::new(archive), entry);
        if name != 0 {
            e.call(STRCPY_S, &args![out, 0x104u32, name]);
            found = true;
        }
        e.call(LEAVE_CRITICAL_SECTION, &args![archive + 0x19c]);
    }
    if archive != 0 {
        release(e, archive);
    }
    found
}

/// The search `00af7400` and `00af7800` share: the path trimmed (`TrimFileName`), the
/// archive type taken from its extension when `types` is 0xFFFF and it does not end in `\`,
/// its hash made (`00afd390(path, 1)`), then the first archive of the list of a matching
/// type whose directory search ([`fn_00af9c50`]) finds the hash. Then, with an archive, a
/// directory entry and files in it, under the archive's lock unless bit 5 of its
/// `cArchiveFlags` is set, `choose` picks a file of the entry, whose name
/// ([`fn_00af96d0`]) replaces the part of the trimmed path after its last `\` in `out`.
/// Without a `\` the answer is false and the lock is not left (as the exe does).
fn pick_file_in_directory(
    e: &mut Engine,
    path: u32,
    out: u32,
    types: u32,
    choose: fn(&mut Engine, u32) -> u32,
) -> bool {
    let path = e.call(TRIM_FILE_NAME, &args![path]).u32();
    let length = e.call(STRLEN, &args![path]).u32();
    let mut types = types;
    if types == 0xffff && e.mem.u8(path.wrapping_add(length).wrapping_sub(1)) != b'\\' {
        types = archive_manager_get_archive_type_from_file_extension(e, Ptr::new(path));
    }
    // Locals: the hash (+0), the directory index (+8), the found archive (+0xc) and the
    // archive being asked (+0x10), both `NiPointer`s.
    e.with_stack(0x14, |e, locals| {
        let hash = locals.addr();
        let index = hash + 8;
        let found = hash + 0xc;
        let current = hash + 0x10;
        e.call(HASH_CONSTRUCTOR, &args![hash, path, 1u32]);
        let mut entry = 0u32;
        // The found pointer starts null; the exe tests it before taking a reference.
        if e.mem.u32(found) != 0 {
            let held = e.mem.u32(found);
            add_ref(e, held);
        }
        if e.mem.u32(found) == 0 {
            let mut node = e.global::<u32>(ARCHIVE_LIST);
            while node != 0 {
                let archive = e.mem.u32(node);
                e.mem.set_u32(current, archive);
                if archive != 0 {
                    add_ref(e, archive);
                }
                let archive = e.mem.u32(current);
                let matches = archive != 0 && {
                    let kind =
                        u32::from(e.get(Ptr::<Archive>::new(archive), Archive::iArchiveType));
                    kind & types & 0xffff != 0
                        && fn_00af9c50(
                            e,
                            Ptr::new(archive),
                            Ptr::new(hash),
                            Ptr::new(index),
                            Ptr::NULL,
                        )
                };
                if matches {
                    fn_00afc770(e, Ptr::new(found), Ptr::new(current));
                    let archive = Ptr::<Archive>::new(e.mem.u32(found));
                    entry =
                        (e.mem.u32(index) << 4).wrapping_add(e.get(archive, Archive::pDirectories));
                    let held = e.mem.u32(current);
                    if held != 0 {
                        release(e, held);
                    }
                    break;
                }
                node = e.mem.u32(node + 4);
                let held = e.mem.u32(current);
                if held != 0 {
                    release(e, held);
                }
            }
        }
        let archive = e.mem.u32(found);
        let release_found = |e: &mut Engine| {
            let held = e.mem.u32(found);
            if held != 0 {
                release(e, held);
            }
        };
        if archive == 0 || entry == 0 || e.mem.u32(entry + 8) == 0 {
            release_found(e);
            return false;
        }
        let lock = e.get(Ptr::<Archive>::new(archive), Archive::cArchiveFlags) & 0x20 == 0;
        if lock {
            e.call(ENTER_CRITICAL_SECTION, &args![archive + 0x19c]);
        }
        let file = choose(e, entry);
        let directory = e.mem.u32(index);
        let name = fn_00af96d0(e, Ptr::new(archive), directory, file);
        e.call(STRCPY_S, &args![out, 0x104u32, path]);
        let slash = e.call(STRRCHR, &args![out, 0x5cu32]).u32();
        if slash == 0 {
            release_found(e);
            return false;
        }
        let room = 0x104u32
            .wrapping_sub(slash.wrapping_sub(out))
            .wrapping_sub(1);
        e.call(STRCPY_S, &args![slash.wrapping_add(1), room, name]);
        if lock {
            let archive = e.mem.u32(found);
            e.call(LEAVE_CRITICAL_SECTION, &args![archive + 0x19c]);
        }
        release_found(e);
        true
    })
}

/// `00af7400`'s pick: a random file of the directory entry (`BSRandom::UnsignedInt` with the
/// entry's file count, the generator `011c4180` constructed on first use).
fn random_file(e: &mut Engine, entry: u32) -> u32 {
    let count = e.mem.u32(entry + 8);
    let guard = e.global::<u32>(RANDOM_GENERATOR_GUARD);
    if guard & 1 == 0 {
        e.set_global(RANDOM_GENERATOR_GUARD, guard | 1);
        e.call(BS_RANDOM_CONSTRUCTOR, &args![RANDOM_GENERATOR]);
    }
    e.call(BS_RANDOM_UNSIGNED_INT, &args![RANDOM_GENERATOR, count])
        .u32()
}

/// `00af7800`'s pick: the first file of the directory entry with the smallest low 30 bits
/// of its word +8.
fn smallest_file(e: &mut Engine, entry: u32) -> u32 {
    let mut chosen = 0u32;
    let mut smallest = 0xffff_ffffu32;
    let mut i = 0u32;
    while i < e.mem.u32(entry + 8) {
        let file = (i << 4).wrapping_add(e.mem.u32(entry + 0xc));
        let value = e.mem.u32(file.wrapping_add(8)) & 0x3fff_ffff;
        if value < smallest {
            smallest = value;
            chosen = i;
        }
        i = i.wrapping_add(1);
    }
    chosen
}

// Translated from 00af7400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): writes into `out` the path of a random
/// file of the directory `path` names in the opened archives (see
/// [`pick_file_in_directory`]; the file drawn by `BSRandom::UnsignedInt`); answers whether
/// it did.
// Translation incomplete: 00af7400 - its FS:[0] exception frame (unwinding the NiPointer locals and the generator's construction) is not translated
pub fn fn_00af7400(e: &mut Engine, path: u32, out: u32, types: u32) -> bool {
    pick_file_in_directory(e, path, out, types, random_file)
}

// Translated from 00af7800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words): as [`fn_00af7400`], the file picked being
/// the first one with the smallest low 30 bits of its entry word +8.
// Translation incomplete: 00af7800 - its FS:[0] exception frame (unwinding the NiPointer locals) is not translated
pub fn fn_00af7800(e: &mut Engine, path: u32, out: u32, types: u32) -> bool {
    pick_file_in_directory(e, path, out, types, smallest_file)
}

// Translated from 00af7e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): whether the hash of `name`
/// (`00afd390(name, 0)`) compares equal ([`fn_00afcf00`] answers 0) with the hash at
/// `hash`.
pub fn fn_00af7e80(e: &mut Engine, name: u32, hash: u32) -> bool {
    e.with_stack(8, |e, local| {
        e.call(HASH_CONSTRUCTOR, &args![local, name, 0u32]);
        fn_00afcf00(e, Ptr::new(hash), local) == 0
    })
}

// Translated from 00af94c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`, one word): the name of directory
/// `directory`. With bit 4 of `cArchiveFlags` (the names kept) it is
/// `pDirectoryStringArray + pDirectoryStringOffsets[directory]`; without names in the
/// archive (`oFlags` bit 0) it is null. Otherwise it is read from the file: a seek (vtable
/// +0x14, origin `010a2480`) past the header and the directory entries
/// (`iDirectories * 16 + 0x24`), then for each directory before `directory` its length byte
/// (through `m_pfnRead`) and a seek (origin `010a2484`) past that many bytes and its file
/// entries; its own length byte; `pDirectoryStringArray` replaced by a block of that length
/// (memory manager) that the name is read into.
pub fn fn_00af94c0(e: &mut Engine, this: Ptr<Archive>, directory: u32) -> u32 {
    if e.get(this, Archive::cArchiveFlags) & 0x10 != 0 {
        let offsets = e.get(this, Archive::pDirectoryStringOffsets);
        let offset = e.mem.u32(offsets.wrapping_add(directory.wrapping_mul(4)));
        return e
            .get(this, Archive::pDirectoryStringArray)
            .wrapping_add(offset);
    }
    if e.get(this, Archive::oFlags) & 1 == 0 {
        return 0;
    }
    let directories = e.get(this, Archive::iDirectories);
    let origin = e.global::<u32>(NI_FILE_SEEK_SET);
    e.vcall(
        this.addr(),
        SEEK_SLOT,
        &args![(directories << 4).wrapping_add(0x24), origin],
    );
    e.with_stack(4, |e, length| {
        let length = length.addr();
        let mut i = 0u32;
        while i < directory {
            read_through_pointer(e, this, length, 1);
            let entries = e.get(this, Archive::pDirectories);
            let files = e.mem.u32(entries.wrapping_add(i << 4).wrapping_add(8));
            let origin = e.global::<u32>(NI_FILE_SEEK_CURRENT);
            let skip = u32::from(e.mem.u8(length)).wrapping_add(files << 4);
            e.vcall(this.addr(), SEEK_SLOT, &args![skip, origin]);
            i = i.wrapping_add(1);
        }
        read_through_pointer(e, this, length, 1);
        if e.get(this, Archive::pDirectoryStringArray) != 0 {
            let old = e.get(this, Archive::pDirectoryStringArray);
            free(e, old);
            e.set(this, Archive::pDirectoryStringArray, 0);
        }
        let size = u32::from(e.mem.u8(length));
        let block = allocate(e, size);
        e.set(this, Archive::pDirectoryStringArray, block);
        let size = u32::from(e.mem.u8(length));
        let block = e.get(this, Archive::pDirectoryStringArray);
        read_through_pointer(e, this, block, size);
        e.get(this, Archive::pDirectoryStringArray)
    })
}

// Translated from 00af9ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`, one word): the index of the
/// directory whose file entries (16 bytes each, from its word +0xc, its word +8 of them)
/// hold the address `entry`; -1 when none does.
pub fn fn_00af9ad0(e: &mut Engine, this: Ptr<Archive>, entry: u32) -> u32 {
    let mut i = 0u32;
    while i < e.get(this, Archive::iDirectories) {
        let directory = (i << 4).wrapping_add(e.get(this, Archive::pDirectories));
        let first = e.mem.u32(directory.wrapping_add(0xc));
        let size = e.mem.u32(directory.wrapping_add(8)) << 4;
        if entry >= first && entry < first.wrapping_add(size) {
            return i;
        }
        i = i.wrapping_add(1);
    }
    0xffff_ffff
}

// Translated from 00af9b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`, two words): the index of the file
/// entry `entry` in directory `directory` (`(entry - first entry) >> 4`).
pub fn fn_00af9b60(e: &mut Engine, this: Ptr<Archive>, directory: u32, entry: u32) -> u32 {
    let record = (directory << 4).wrapping_add(e.get(this, Archive::pDirectories));
    let first = e.mem.u32(record.wrapping_add(0xc));
    entry.wrapping_sub(first) >> 4
}

// Translated from 00af9ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`, one word): the name of the file of
/// `entry` ([`fn_00af96d0`] with its directory, [`fn_00af9ad0`], and its index there,
/// [`fn_00af9b60`]); null when no directory holds the entry.
pub fn fn_00af9ba0(e: &mut Engine, this: Ptr<Archive>, entry: u32) -> u32 {
    let directory = fn_00af9ad0(e, this, entry);
    if directory == 0xffff_ffff {
        return 0;
    }
    let file = fn_00af9b60(e, this, directory, entry);
    fn_00af96d0(e, this, directory, file)
}

// Translated from 00afa8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`, three words): a name collision
/// check. With the byte `011f8154` set and bits 0 and 1 of `oFlags`, `path` is split
/// (`_splitpath_s`: directory and file name, 0x104 each) and, when the hashes find a file
/// ([`fn_00af9bf0`]), the archive's directory name ([`fn_00af94c0`]) and file name
/// ([`fn_00af96d0`]) are copied (memory-manager blocks of `strlen + 1`, `strcpy_s` given
/// `strlen` as the size) and the four strings upper-cased (`_strupr_s`, given their
/// lengths). A differing directory frees both copies and answers the archive's directory
/// name; a differing file name frees both and answers the archive's file name; otherwise 0
/// (the copies are kept, as the exe does).
// Translation incomplete: 00afa8e0 - its /GS stack cookie check is not translated
pub fn fn_00afa8e0(
    e: &mut Engine,
    this: Ptr<Archive>,
    directory: u32,
    file: u32,
    path: u32,
) -> u32 {
    if e.global::<u8>(FLAG_SET_BY_00AF4460) == 0 {
        return 0;
    }
    if e.get(this, Archive::oFlags) & 1 == 0 || e.get(this, Archive::oFlags) & 2 == 0 {
        return 0;
    }
    e.with_stack(0x210, |e, locals| {
        let split_directory = locals.addr();
        let split_name = split_directory + 0x108;
        let indices = split_directory + 0x208;
        e.call(
            SPLITPATH_S,
            &args![
                path,
                0u32,
                0u32,
                split_directory,
                0x104u32,
                split_name,
                0x104u32,
                0u32,
                0u32
            ],
        );
        if !fn_00af9bf0(
            e,
            this,
            Ptr::new(directory),
            Ptr::new(file),
            Ptr::new(indices),
            Ptr::new(indices + 4),
            Ptr::NULL,
        ) {
            return 0;
        }
        let directory_index = e.mem.u32(indices);
        let own_directory = fn_00af94c0(e, this, directory_index);
        let file_index = e.mem.u32(indices + 4);
        let directory_index = e.mem.u32(indices);
        let own_name = fn_00af96d0(e, this, directory_index, file_index);
        let length = e.call(STRLEN, &args![own_directory]).u32();
        let directory_copy = allocate(e, length.wrapping_add(1));
        let length = e.call(STRLEN, &args![own_name]).u32();
        let name_copy = allocate(e, length.wrapping_add(1));
        let length = e.call(STRLEN, &args![own_directory]).u32();
        e.call(STRCPY_S, &args![directory_copy, length, own_directory]);
        let length = e.call(STRLEN, &args![own_name]).u32();
        e.call(STRCPY_S, &args![name_copy, length, own_name]);
        for text in [directory_copy, name_copy, split_directory, split_name] {
            let length = e.call(STRLEN, &args![text]).u32();
            e.call(STRUPR_S, &args![text, length]);
        }
        if e.call(STRICMP, &args![directory_copy, split_directory])
            .i32()
            != 0
        {
            free(e, directory_copy);
            free(e, name_copy);
            return own_directory;
        }
        if e.call(STRICMP, &args![name_copy, split_name]).i32() != 0 {
            free(e, directory_copy);
            free(e, name_copy);
            return own_name;
        }
        0
    })
}

/// Clears the low 31 bits of a file entry's word +0xc (its offset), keeping bit 31.
fn clear_offset_keep_top_bit(e: &mut Engine, file: u32) {
    let word = e.mem.u32(file.wrapping_add(0xc));
    e.mem.set_u32(file.wrapping_add(0xc), word & 0x8000_0000);
}

// Translated from 00afac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`, three words; the third is not read):
/// when the hashes find a file ([`fn_00af9bf0`] with no name), clears its entry's offset
/// (word +0xc less bit 31) and answers true.
pub fn fn_00afac20(
    e: &mut Engine,
    this: Ptr<Archive>,
    directory: Ptr,
    file: Ptr,
    _unused_3: u32,
) -> bool {
    e.with_stack(8, |e, indices| {
        let directory_index = indices.addr();
        let file_index = directory_index + 4;
        if !fn_00af9bf0(
            e,
            this,
            directory,
            file,
            Ptr::new(directory_index),
            Ptr::new(file_index),
            Ptr::NULL,
        ) {
            return false;
        }
        let record =
            (e.mem.u32(directory_index) << 4).wrapping_add(e.get(this, Archive::pDirectories));
        let entry = (e.mem.u32(file_index) << 4).wrapping_add(e.mem.u32(record.wrapping_add(0xc)));
        clear_offset_keep_top_bit(e, entry);
        true
    })
}

// Translated from 00afac90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `Archive`, three words; the third is not read):
/// when directory `directory` holds the file hash ([`fn_00afa120`] with no name, not any),
/// clears its entry's offset (word +0xc less bit 31) and answers true.
pub fn fn_00afac90(
    e: &mut Engine,
    this: Ptr<Archive>,
    directory: u32,
    file: Ptr,
    _unused_3: u32,
) -> bool {
    e.with_stack(4, |e, index| {
        if !fn_00afa120(e, this, directory, file, index, Ptr::NULL, false) {
            return false;
        }
        let record = (directory << 4).wrapping_add(e.get(this, Archive::pDirectories));
        let entry =
            (e.mem.u32(index.addr()) << 4).wrapping_add(e.mem.u32(record.wrapping_add(0xc)));
        clear_offset_keep_top_bit(e, entry);
        true
    })
}

/// The `ArchiveFile` transfer `00afbc70` (slot +0x30) and `00afbde0` (slot +0x2c) share:
/// asks the archive its name (vtable +0x18, the answer unused), takes its lock (when there
/// is an archive), seeks it (vtable +0x14, origin 0) to this file's position plus
/// `iArchiveOffset` unless it is there already (each position the word +0x38, or
/// `iTrueFilePos` when that is -1), runs the transfer with `buffer` and `iFileSize`, adds
/// its answer to `iTrueFilePos`, leaves the lock and answers it.
fn archive_file_transfer(e: &mut Engine, this: Ptr<ArchiveFile>, buffer: u32, slot: u32) -> u32 {
    let at = this.addr();
    let position_of = |e: &Engine, file: u32| {
        let word = e.mem.u32(file.wrapping_add(POSITION_OVERRIDE));
        if word == 0xffff_ffff {
            e.get(Ptr::<BSFile>::new(file), BSFile::iTrueFilePos)
        } else {
            word
        }
    };
    let archive = e.get(this, ArchiveFile::spArchive);
    e.vcall(archive, NAME_SLOT, &args![]);
    if e.get(this, ArchiveFile::spArchive) != 0 {
        let archive = e.get(this, ArchiveFile::spArchive);
        e.call(ENTER_CRITICAL_SECTION, &args![archive.wrapping_add(0x19c)]);
    }
    let position = e
        .get(this, ArchiveFile::iArchiveOffset)
        .wrapping_add(position_of(e, at));
    let archive = e.get(this, ArchiveFile::spArchive);
    if position_of(e, archive) != position {
        e.vcall(archive, SEEK_SLOT, &args![position, 0u32]);
    }
    let archive = e.get(this, ArchiveFile::spArchive);
    let size = e.get(this.cast::<BSFile>(), BSFile::iFileSize);
    let done = e.vcall(archive, slot, &args![buffer, size]).u32();
    let file = this.cast::<BSFile>();
    let moved = e.get(file, BSFile::iTrueFilePos).wrapping_add(done);
    e.set(file, BSFile::iTrueFilePos, moved);
    if e.get(this, ArchiveFile::spArchive) != 0 {
        let archive = e.get(this, ArchiveFile::spArchive);
        e.call(LEAVE_CRITICAL_SECTION, &args![archive.wrapping_add(0x19c)]);
    }
    done
}

// Translated from 00afbc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `ArchiveFile`, two words; the second is not
/// read): the whole-file transfer through the archive's vtable +0x30 (see
/// [`archive_file_transfer`]).
pub fn fn_00afbc70(e: &mut Engine, this: Ptr<ArchiveFile>, buffer: u32, _unused_2: u32) -> u32 {
    archive_file_transfer(e, this, buffer, TRANSFER_SLOT_30)
}

// Translated from 00afbde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `ArchiveFile`, two words; the second is not
/// read): the whole-file transfer through the archive's vtable +0x2c (see
/// [`archive_file_transfer`]).
pub fn fn_00afbde0(e: &mut Engine, this: Ptr<ArchiveFile>, buffer: u32, _unused_2: u32) -> u32 {
    archive_file_transfer(e, this, buffer, TRANSFER_SLOT_2C)
}

// Translated from 00afbf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words, none read): answers 0.
pub fn fn_00afbf50(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0
}

// Translated from 00afbf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `ArchiveFile`): whether it has an archive
/// (`spArchive` not null).
pub fn fn_00afbf60(e: &mut Engine, this: Ptr<ArchiveFile>) -> bool {
    e.get(this, ArchiveFile::spArchive) != 0
}

// Translated from 00afc3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words, none read): answers 0.
pub fn fn_00afc3e0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0
}

// Translated from 00afc3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words, none read): answers 0.
pub fn fn_00afc3f0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0
}

// Translated from 00afc400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words, none read): answers 0.
pub fn fn_00afc400(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0
}

// Translated from 00afc410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on an `ArchiveFile`): whether it has an archive
/// (`spArchive` not null).
pub fn fn_00afc410(e: &mut Engine, this: Ptr<ArchiveFile>) -> bool {
    e.get(this, ArchiveFile::spArchive) != 0
}

// Translated from 00afc960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a `BSSimpleList<NiPointer<Archive>>` head node,
/// one word): removes the archive `*item`. Nothing for a null archive or an empty list.
/// Found in the head, the next node's archive moves into the head ([`fn_00afc770`]) and
/// that node is unlinked, destroyed ([`fn_00afc7f0`]) and freed, or, with no next node, the
/// head's archive is cleared ([`fn_00afd1b0`] with null); found further on, its node is
/// unlinked, destroyed and freed.
pub fn fn_00afc960(e: &mut Engine, this: Ptr, item: Ptr) {
    let head = this.addr();
    if e.mem.u32(item.addr()) == 0 {
        return;
    }
    if e.mem.u32(head + 4) == 0 && e.mem.u32(head) == 0 {
        return;
    }
    let mut node = head;
    let mut previous = head;
    while node != 0 && e.mem.u32(node) != e.mem.u32(item.addr()) {
        previous = node;
        node = e.mem.u32(node + 4);
    }
    if node == 0 {
        return;
    }
    if node == head {
        if e.mem.u32(head + 4) != 0 {
            let next = e.mem.u32(head + 4);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(head + 4, after);
            fn_00afc770(e, this, Ptr::new(next));
            e.mem.set_u32(next + 4, 0);
            if next != 0 {
                fn_00afc7f0(e, Ptr::new(next));
                free(e, next);
            }
        } else {
            fn_00afd1b0(e, this, 0);
        }
    } else {
        let after = e.mem.u32(node + 4);
        e.mem.set_u32(previous + 4, after);
        e.mem.set_u32(node + 4, 0);
        if node != 0 {
            fn_00afc7f0(e, Ptr::new(node));
            free(e, node);
        }
    }
}

// Translated from 00afcce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<BSHash*,1024>` constructor (thiscall): its
/// vtable (`010a469c`) and the initial set-up `00afcde0(0, 0)`. Returns `this`.
pub fn fn_00afcce0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), HASH_ARRAY_VTABLE);
    e.call(HASH_ARRAY_INITIAL_SETUP, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00afcd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSSimpleArray<BSHash*,1024>::_Allocate` shape (thiscall, one
/// word, `this` not read): `count` pointers from the memory manager.
pub fn fn_00afcd10(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    allocate(e, count << 2)
}

// Translated from 00afcd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSSimpleArray<BSHash*,1024>::_Deallocate` shape (thiscall,
/// one word, `this` not read): gives `buffer` back to the memory manager, passing its answer
/// on.
pub fn fn_00afcd40(e: &mut Engine, _this: Ptr, buffer: u32) -> u32 {
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, buffer])
        .u32()
}

// Translated from 00afcd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSSimpleArray<BSHash*,1024>::_Reallocate` shape (thiscall,
/// two words, `this` not read): the memory manager's reallocation of `buffer` to `count`
/// pointers (`00aa4150`), whose answer it passes on.
pub fn fn_00afcd60(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) -> u32 {
    e.call(
        MEMORY_MANAGER_REALLOCATE,
        &args![MEMORY_MANAGER, buffer, count << 2],
    )
    .u32()
}

// Translated from 00afcdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<BSHash*,1024>` destructor (thiscall): its
/// vtable (`010a469c`), then the clear `00a1bfa0(1)`, whose answer it passes on.
pub fn fn_00afcdc0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), HASH_ARRAY_VTABLE);
    e.call(SIMPLE_ARRAY_CLEAR_00A1BFA0, &args![this, 1u32])
        .u32()
}

// Translated from 00afcd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BSHash*,1024>::scalar deleting destructor` (engine map; thiscall, one
/// word): destroys the array ([`fn_00afcdc0`]) and, with bit 0 of `flags`, gives it back to
/// the memory manager. Answers `this`.
pub fn bs_simple_array_bs_hash_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00afcdc0(e, this);
    if flags & 1 != 0 {
        free(e, this.addr());
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_4390, fn_00af4390() -> bool),
        entry!(0x00af_44f0, fn_00af44f0()),
        entry!(0x00af_5480, fn_00af5480(u32)),
        entry!(0x00af_5550, fn_00af5550(u32) -> bool),
        entry!(0x00af_56d0, fn_00af56d0(u32) -> bool),
        entry!(0x00af_5700, fn_00af5700(u32) -> bool),
        entry!(0x00af_5e40, archive_manager_invalidate_file(u32) -> bool),
        entry!(0x00af_6430, fn_00af6430(u32, u32, u32, u32) -> u32),
        entry!(0x00af_6690, fn_00af6690(u32, u32, u32, u32) -> u32),
        entry!(0x00af_67d0, fn_00af67d0(u32, u32, u32) -> bool),
        entry!(0x00af_7400, fn_00af7400(u32, u32, u32) -> bool),
        entry!(0x00af_7800, fn_00af7800(u32, u32, u32) -> bool),
        entry!(0x00af_7e80, fn_00af7e80(u32, u32) -> bool),
        entry!(0x00af_94c0, fn_00af94c0(Ptr<Archive>, u32) -> u32),
        entry!(0x00af_9ad0, fn_00af9ad0(Ptr<Archive>, u32) -> u32),
        entry!(0x00af_9b60, fn_00af9b60(Ptr<Archive>, u32, u32) -> u32),
        entry!(0x00af_9ba0, fn_00af9ba0(Ptr<Archive>, u32) -> u32),
        entry!(0x00af_a8e0, fn_00afa8e0(Ptr<Archive>, u32, u32, u32) -> u32),
        entry!(
            0x00af_ac20,
            fn_00afac20(Ptr<Archive>, Ptr, Ptr, u32) -> bool
        ),
        entry!(
            0x00af_ac90,
            fn_00afac90(Ptr<Archive>, u32, Ptr, u32) -> bool
        ),
        entry!(0x00af_bc70, fn_00afbc70(Ptr<ArchiveFile>, u32, u32) -> u32),
        entry!(0x00af_bde0, fn_00afbde0(Ptr<ArchiveFile>, u32, u32) -> u32),
        entry!(0x00af_bf50, fn_00afbf50(Ptr, u32, u32) -> u32),
        entry!(0x00af_bf60, fn_00afbf60(Ptr<ArchiveFile>) -> bool),
        entry!(0x00af_c3e0, fn_00afc3e0(Ptr, u32, u32) -> u32),
        entry!(0x00af_c3f0, fn_00afc3f0(Ptr, u32, u32) -> u32),
        entry!(0x00af_c400, fn_00afc400(Ptr, u32, u32) -> u32),
        entry!(0x00af_c410, fn_00afc410(Ptr<ArchiveFile>) -> bool),
        entry!(0x00af_c960, fn_00afc960(Ptr, Ptr)),
        entry!(0x00af_cce0, fn_00afcce0(Ptr) -> Ptr),
        entry!(0x00af_cd10, fn_00afcd10(Ptr, u32) -> u32),
        entry!(0x00af_cd40, fn_00afcd40(Ptr, u32) -> u32),
        entry!(0x00af_cd60, fn_00afcd60(Ptr, u32, u32) -> u32),
        entry!(
            0x00af_cd90,
            bs_simple_array_bs_hash_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00af_cdc0, fn_00afcdc0(Ptr) -> u32),
        entry!(0x00af_7c40, fn_00af7c40(u32, u32, u32)),
        entry!(
            0x00af_59e0,
            archive_manager_cleanup_archive_strings_after_startup()
        ),
        entry!(0x00af_4460, fn_00af4460(u8)),
        entry!(0x00af_4540, fn_00af4540()),
        entry!(0x00af_4470, fn_00af4470(u32, u32, u32)),
        entry!(0x00af_4490, fn_00af4490(u8, Ptr)),
        entry!(0x00af_43a0, fn_00af43a0(u8, Ptr, Ptr, u32)),
        entry!(0x00af_4170, find_texture_path_offset(Ptr, bool) -> u32),
        entry!(0x00af_4200, fn_00af4200(Ptr, Ptr, u32) -> bool),
        entry!(0x00af_4250, fn_00af4250(Ptr, Ptr, u32)),
        entry!(0x00af_4310, fn_00af4310(Ptr, Ptr, u32)),
        entry!(0x00af_7db0, fn_00af7db0(Ptr) -> u32),
        entry!(
            0x00af_7d80,
            archive_manager_get_archive_type_from_file_extension(Ptr) -> u32
        ),
        entry!(0x00af_6320, fn_00af6320(Ptr, u32) -> bool),
        entry!(0x00af_b170, fn_00afb170(Ptr, u8, u32) -> u8),
        entry!(0x00af_b2a0, fn_00afb2a0(Ptr<Archive>, Ptr, bool) -> bool),
        entry!(0x00af_b190, fn_00afb190(Ptr<Archive>, Ptr, Ptr) -> bool),
        entry!(0x00af_ce90, fn_00afce90(Ptr, Ptr) -> i32),
        entry!(0x00af_cf00, fn_00afcf00(Ptr, Ptr) -> i32),
        entry!(0x00af_d8b0, fn_00afd8b0(Ptr) -> Ptr),
        entry!(0x00af_dc00, fn_00afdc00(Ptr, u32) -> bool),
        entry!(0x00af_a6e0, fn_00afa6e0(Ptr<Archive>, Ptr, Ptr, Ptr) -> u32),
        entry!(
            0x00af_9bf0,
            fn_00af9bf0(Ptr<Archive>, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00af_9230, fn_00af9230(Ptr<Archive>)),
        entry!(0x00af_92f0, fn_00af92f0(Ptr<Archive>, bool)),
        entry!(0x00af_9410, fn_00af9410(Ptr<Archive>)),
        entry!(0x00af_8e60, fn_00af8e60(Ptr<Archive>)),
        entry!(0x00af_c680, fn_00afc680(Ptr<Archive>, u32) -> Ptr<Archive>),
        entry!(0x00af_ce80, fn_00afce80(u32, u32) -> Ptr<Archive>),
        entry!(0x00af_c720, fn_00afc720(Ptr)),
        entry!(0x00af_d1b0, fn_00afd1b0(Ptr, u32) -> Ptr),
        entry!(0x00af_c770, fn_00afc770(Ptr, Ptr) -> Ptr),
        entry!(0x00af_c880, fn_00afc880(Ptr, Ptr)),
        entry!(0x00af_cc10, fn_00afcc10(Ptr, Ptr)),
        entry!(0x00af_caf0, fn_00afcaf0(Ptr)),
        entry!(0x00af_c7f0, fn_00afc7f0(Ptr)),
        entry!(0x00af_59f0, fn_00af59f0()),
        entry!(
            0x00af_9c50,
            fn_00af9c50(Ptr<Archive>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00af_a120,
            fn_00afa120(Ptr<Archive>, u32, Ptr, Ptr, Ptr, bool) -> bool
        ),
        entry!(0x00af_b5f0, fn_00afb5f0(Ptr<ArchiveFile>, u8, u8) -> bool),
        entry!(0x00af_c060, fn_00afc060(Ptr<ArchiveFile>, u8, u8) -> bool),
        entry!(0x00af_b670, fn_00afb670(Ptr<ArchiveFile>, u32, u32)),
        entry!(0x00af_b780, fn_00afb780(Ptr<ArchiveFile>) -> u32),
        entry!(0x00af_c0a0, fn_00afc0a0(Ptr<CompressedArchiveFile>) -> u32),
        entry!(0x00af_c0c0, fn_00afc0c0(Ptr<CompressedArchiveFile>) -> u32),
        entry!(
            0x00af_c080,
            fn_00afc080(Ptr<CompressedArchiveFile>, u32, u32)
        ),
        entry!(0x00af_bbf0, fn_00afbbf0(Ptr<ArchiveFile>, Ptr, u32) -> u32),
        entry!(0x00af_bf80, fn_00afbf80(Ptr<ArchiveFile>, Ptr, u32) -> u32),
        entry!(0x00af_bfa0, fn_00afbfa0(Ptr<ArchiveFile>, bool)),
        entry!(0x00af_c620, fn_00afc620(Ptr<CompressedArchiveFile>, bool)),
        entry!(
            0x00af_c660,
            fn_00afc660(Ptr<CompressedArchiveFile>, Ptr, u32) -> u32
        ),
        entry!(0x00af_b520, fn_00afb520(Ptr<ArchiveFile>)),
        entry!(
            0x00af_c6c0,
            fn_00afc6c0(Ptr<ArchiveFile>, u32) -> Ptr<ArchiveFile>
        ),
        entry!(0x00af_bfe0, fn_00afbfe0(Ptr<CompressedArchiveFile>)),
        entry!(
            0x00af_c6f0,
            fn_00afc6f0(Ptr<CompressedArchiveFile>, u32) -> Ptr<CompressedArchiveFile>
        ),
        entry!(
            0x00af_b300,
            fn_00afb300(Ptr<ArchiveFile>, Ptr, u32, u32, u32, u32) -> Ptr<ArchiveFile>
        ),
        entry!(
            0x00af_c430,
            fn_00afc430(
                Ptr<CompressedArchiveFile>,
                Ptr,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<CompressedArchiveFile>
        ),
        entry!(
            0x00af_b7a0,
            fn_00afb7a0(Ptr<ArchiveFile>, Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00af_c0e0,
            fn_00afc0e0(Ptr<CompressedArchiveFile>, Ptr, u32) -> u32
        ),
        entry!(0x00af_96d0, fn_00af96d0(Ptr<Archive>, u32, u32) -> u32),
        entry!(0x00af_6160, fn_00af6160(Ptr, u32) -> u32),
        entry!(0x00af_5fa0, archive_manager_get_file(Ptr, u32, u32) -> u32),
        entry!(
            0x00af_a550,
            fn_00afa550(Ptr<Archive>, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00af_a750, fn_00afa750(Ptr<Archive>, Ptr, u32, u32) -> u32),
        entry!(0x00af_6910, fn_00af6910(Ptr, u32) -> u32),
        entry!(0x00af_6340, fn_00af6340(u32, Ptr, u32, u32) -> u32),
        entry!(0x00af_6540, fn_00af6540(u8, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00af_8f10, fn_00af8f10(Ptr<Archive>)),
        entry!(0x00af_5820, fn_00af5820()),
        entry!(0x00af_5320, fn_00af5320(Ptr) -> u32),
        entry!(0x00af_5ab0, fn_00af5ab0(Ptr)),
        entry!(0x00af_ad00, fn_00afad00(Ptr<Archive>) -> u32),
        entry!(0x00af_d020, fn_00afd020(Ptr<Archive>) -> bool),
        entry!(0x00af_6e50, fn_00af6e50(Ptr, i32, i32)),
        entry!(
            0x00af_7060,
            fn_00af7060(Ptr<Archive>, u32, u32, Ptr, Ptr, u32, Ptr) -> u32
        ),
        entry!(
            0x00af_6ba0,
            archive_manager_add_to_file_list(u32, Ptr, Ptr, u32)
        ),
        entry!(0x00af_4550, archive_manager_open_master_archives() -> bool),
        entry!(0x00af_4be0, archive_manager_open_archive(Ptr, u16, u8) -> u32),
        entry!(
            0x00af_7eb0,
            fn_00af7eb0(Ptr<Archive>, Ptr, u32, u8, u8) -> Ptr<Archive>
        ),
    ]
}

#[cfg(test)]
mod tests {
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

    fn lower(e: &Engine, at: u32, limit: usize) -> Vec<u8> {
        let mut text = e.mem.cstr(at);
        text.truncate(limit);
        text.iter().map(u8::to_ascii_lowercase).collect()
    }

    fn order(first: &[u8], second: &[u8]) -> i32 {
        match first.cmp(second) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    const REF_VTABLE: u32 = 0x0300_0000;
    const DELETE_THIS: u32 = 0x0070_0000;

    /// An engine with this unit's globals mapped and working doubles for
    /// the CRT string helpers, the memory manager, the interlocked
    /// operations and the critical sections. `_access` finds the paths in
    /// the returned list.
    fn engine() -> (Engine, Rc<RefCell<Vec<Vec<u8>>>>) {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.map(0x011f_4000, 0x5000);
        e.map(0x010a_2000, 0x3000);
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRNICMP, |e, a| {
            let n = a[2] as usize;
            order(&lower(e, a[0], n), &lower(e, a[1], n)).into_ret()
        });
        e.register(STRICMP, |e, a| {
            order(&lower(e, a[0], usize::MAX), &lower(e, a[1], usize::MAX)).into_ret()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(STRSTR, |e, a| {
            let hay = e.mem.cstr(a[0]);
            let needle = e.mem.cstr(a[1]);
            let at = hay
                .windows(needle.len().max(1))
                .position(|w| w == needle.as_slice());
            at.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            let at = text.iter().rposition(|&c| u32::from(c) == a[1]);
            at.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            e.mem.alloc(a[1].max(1)).into_ret()
        });
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            value.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            value.into_ret()
        });
        for section in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            DELETE_CRITICAL_SECTION,
            INITIALIZE_CRITICAL_SECTION,
        ] {
            e.register(section, |_, _| Ret::default());
        }
        e.put_vtable(REF_VTABLE, &[0, DELETE_THIS]);
        e.register(DELETE_THIS, |_, _| Ret::default());
        let existing = Rc::new(RefCell::new(Vec::<Vec<u8>>::new()));
        let seen = existing.clone();
        e.register_double(ACCESS, move |e, a| {
            let text = e.mem.cstr(a[0]);
            let found = seen.borrow().contains(&text);
            (if found { 0 } else { u32::MAX }).into_ret()
        });
        e.call_log = Some(vec![]);
        (e, existing)
    }

    fn text(e: &mut Engine, value: &[u8]) -> u32 {
        let at = e.mem.alloc(0x110);
        e.mem.set_cstr(at, value);
        at
    }

    /// Writes the `.rdata` strings this unit passes.
    fn strings(e: &mut Engine) {
        for (at, value) in [
            (TEXT_DATA_LOWER, &b"data\\"[..]),
            (TEXT_TEXTURES, b"textures\\"),
            (TEXT_SPT, b"spt"),
            (TEXT_TREES, b"Trees"),
            (TEXT_MESHES, b"meshes\\"),
            (TEXT_MESHES_PREFIX, b"meshes\\"),
            (TEXT_DATA_PREFIX, b"data\\"),
            (TEXT_DATA_CAPITAL, b"Data\\"),
            (TEXT_DATA_INVALIDATION, b"Data\\"),
            (TEXT_BACKSLASH, b"\\"),
        ] {
            e.mem.set_cstr(at, value);
        }
    }

    /// An `Archive` with a reference-counting part whose delete slot is
    /// the double `DELETE_THIS`.
    fn archive(e: &mut Engine) -> Ptr<Archive> {
        let this: Ptr<Archive> = e.new_object();
        e.set(this, Archive::ref_object_vtable, REF_VTABLE);
        this
    }

    /// Gives `archive` directories: each a hash and its files, each file a
    /// hash, a size word and an offset word.
    fn directories(e: &mut Engine, archive: Ptr<Archive>, layout: &[(u64, &[(u64, u32, u32)])]) {
        let table = e.mem.alloc(16 * layout.len() as u32 + 16);
        for (index, (hash, files)) in layout.iter().enumerate() {
            let entry = table + 16 * index as u32;
            e.mem.set_u64(entry, *hash);
            e.mem.set_u32(entry + 8, files.len() as u32);
            let block = e.mem.alloc(16 * files.len() as u32 + 16);
            for (file, (hash, size, offset)) in files.iter().enumerate() {
                let at = block + 16 * file as u32;
                e.mem.set_u64(at, *hash);
                e.mem.set_u32(at + 8, *size);
                e.mem.set_u32(at + 0xc, *offset);
            }
            e.mem.set_u32(entry + 0xc, block);
        }
        e.set(archive, Archive::pDirectories, table);
        e.set(archive, Archive::iDirectories, layout.len() as u32);
        e.set(archive, Archive::iLastDirectoryIndex, u32::MAX);
        e.set(archive, Archive::iLastFileIndex, u32::MAX);
    }

    fn hash(e: &mut Engine, value: u64) -> u32 {
        let at = e.mem.alloc(8);
        e.mem.set_u64(at, value);
        at
    }

    #[test]
    fn cleanup_archive_strings_does_nothing() {
        let (mut e, _) = engine();
        e.call(0x00af_59e0, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn setters_store_their_globals() {
        let (mut e, _) = engine();
        e.call(0x00af_4460, &args![7u8]);
        assert_eq!(e.global::<u8>(FLAG_SET_BY_00AF4460), 7);
        e.call(0x00af_4540, &args![]);
        assert_eq!(e.global::<u32>(ARCHIVE_FILE_BUFFER_SIZE), 0x20000);
        e.call(0x00af_4470, &args![1u32, 2u32, 3u32]);
        assert_eq!(e.global::<u32>(WORDS_SET_BY_00AF4470), 1);
        assert_eq!(e.global::<u32>(WORDS_SET_BY_00AF4470 + 4), 2);
        assert_eq!(e.global::<u32>(NAME_TABLE_MODE), 3);
    }

    #[test]
    fn invalidation_name_is_copied_when_given() {
        let (mut e, _) = engine();
        e.call(0x00af_4490, &args![1u8, 0u32]);
        assert_eq!(e.global::<u8>(FLAG_011ACDF9), 1);
        assert!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
        let name = text(&mut e, b"inv.txt");
        e.call(0x00af_4490, &args![0u8, name]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 8]]
        );
        let copy = e.global::<u32>(STRING_011F8158);
        assert_eq!(e.mem.cstr(copy), b"inv.txt");
    }

    #[test]
    fn archive_settings_are_stored() {
        let (mut e, _) = engine();
        strings(&mut e);
        e.call(0x00af_43a0, &args![1u8, 0u32, 0u32, 0x1000u32]);
        assert_eq!(e.global::<u8>(FLAG_011ACDF8), 1);
        let list = e.global::<u32>(STRING_011F815C);
        assert_eq!(e.mem.cstr(list), b"");
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE)[0],
            vec![MEMORY_MANAGER, 0]
        );
        assert_eq!(e.mem.cstr(PATH_011F8050), b"");
        assert_eq!(e.global::<u32>(ARCHIVE_FILE_BUFFER_SIZE), 0x1000);
        let path = text(&mut e, b"c:\\game\\");
        let names = text(&mut e, b"a.bsa, b.bsa");
        e.call(0x00af_43a0, &args![0u8, path, names, 5u32]);
        let list = e.global::<u32>(STRING_011F815C);
        assert_eq!(e.mem.cstr(list), b"a.bsa, b.bsa");
        assert_eq!(e.mem.cstr(PATH_011F8050), b"c:\\game\\");
    }

    #[test]
    fn texture_path_offset_skips_the_prefixes() {
        let (mut e, _) = engine();
        strings(&mut e);
        let path = text(&mut e, b"c:\\Data\\Textures\\a.dds");
        assert_eq!(e.call(0x00af_4170, &args![path, true]).u32(), 17);
        assert_eq!(e.call(0x00af_4170, &args![path, false]).u32(), 8);
        let path = text(&mut e, b"\\textures\\a.dds");
        assert_eq!(e.call(0x00af_4170, &args![path, true]).u32(), 10);
        let path = text(&mut e, b"a.dds");
        assert_eq!(e.call(0x00af_4170, &args![path, true]).u32(), 0);
    }

    #[test]
    fn relative_path_drops_drive_and_data() {
        let (mut e, _) = engine();
        strings(&mut e);
        e.register(STRING_IN_PLACE, |_, _| Ret::default());
        let path = text(&mut e, b"c:\\data\\meshes\\a.nif");
        let out = e.mem.alloc(0x40);
        e.mem.set_cstr(out, b"junk");
        assert!(e.call(0x00af_4200, &args![path, out, 0x40u32]).bool());
        assert_eq!(e.mem.cstr(out), b"meshes\\a.nif");
        assert_eq!(calls_to(&e, STRING_IN_PLACE), vec![vec![out, 0x40]]);
    }

    #[test]
    fn model_paths_get_their_folder() {
        let (mut e, _) = engine();
        strings(&mut e);
        let out = e.mem.alloc(0x40);
        for (name, want) in [
            (&b"\\oak.spt"[..], &b"Trees\\oak.spt"[..]),
            (b"meshes\\a.nif", b"meshes\\a.nif"),
            (b"a.nif", b"meshes\\a.nif"),
            (b"spt", b"meshes\\spt"),
        ] {
            let name = text(&mut e, name);
            e.call(0x00af_4250, &args![name, out, 0x40u32]);
            assert_eq!(e.mem.cstr(out), want);
        }
    }

    #[test]
    fn copy_without_drive_and_data() {
        let (mut e, _) = engine();
        strings(&mut e);
        let out = e.mem.alloc(0x40);
        for (path, want) in [
            (&b"d:\\Data\\x.nif"[..], &b"x.nif"[..]),
            (b"data\\x.nif", b"x.nif"),
            (b"c:x", b"c:x"),
            (b"x.nif", b"x.nif"),
        ] {
            let path = text(&mut e, path);
            e.call(0x00af_4310, &args![path, out, 0x40u32]);
            assert_eq!(e.mem.cstr(out), want);
        }
    }

    /// Fills the 0x1a-entry extension table with sorted names `a`..`z`
    /// whose type is their index plus 1.
    fn type_table(e: &mut Engine) {
        for index in 0..0x1au32 {
            let at = ARCHIVE_TYPE_TABLE + index * 8;
            e.mem.set_cstr(at, &[b'a' + index as u8]);
            e.mem.set_u32(at + 4, index + 1);
        }
    }

    #[test]
    fn extension_type_is_found_by_halving() {
        let (mut e, _) = engine();
        type_table(&mut e);
        for (name, want) in [
            (&b".m"[..], 13u32),
            (b"x.m", 13),
            (b"b", 2),
            (b"y", 25),
            (b"zz", 0x100),
        ] {
            let name = text(&mut e, name);
            assert_eq!(e.call(0x00af_7db0, &args![name]).u32(), want);
        }
        // The last entry is reached by the final halving step.
        let name = text(&mut e, b"z");
        assert_eq!(e.call(0x00af_7db0, &args![name]).u32(), 26);
        let name = text(&mut e, b"file.c");
        assert_eq!(e.call(0x00af_7d80, &args![name]).u32(), 3);
    }

    #[test]
    fn kind_one_passes_the_byte_through() {
        let (mut e, _) = engine();
        assert_eq!(e.call(0x00af_b170, &args![0u32, 0x55u8, 1u32]).u8(), 0x55);
        assert_eq!(e.call(0x00af_b170, &args![0u32, 0x55u8, 0u32]).u8(), 0);
        assert_eq!(e.call(0x00af_b170, &args![0u32, 0u8, 9u32]).u8(), 1);
    }

    #[test]
    fn entry_has_data_checks_the_compression_bit() {
        let (mut e, _) = engine();
        let this = archive(&mut e);
        let entry = e.mem.alloc(16);
        e.mem.set_u32(entry + 0xc, 0x8000_0010);
        assert!(!e.call(0x00af_b2a0, &args![this, entry, false]).bool());
        assert!(e.call(0x00af_b2a0, &args![this, entry, true]).bool());
        e.set(this, Archive::cArchiveFlags, 8);
        assert!(e.call(0x00af_b2a0, &args![this, entry, false]).bool());
        e.mem.set_u32(entry + 0xc, 0x8000_0000);
        assert!(!e.call(0x00af_b2a0, &args![this, entry, false]).bool());
    }

    #[test]
    fn loose_override_marks_the_entry() {
        let (mut e, existing) = engine();
        strings(&mut e);
        existing.borrow_mut().push(b"Data\\a.nif".to_vec());
        let this = archive(&mut e);
        let entry = e.mem.alloc(16);
        e.mem.set_u32(entry + 8, 0x10);
        e.mem.set_u32(entry + 0xc, 0x8000_0123);
        let name = text(&mut e, b"a.nif");
        // The byte 011acdf9 is clear: nothing happens.
        assert!(!e.call(0x00af_b190, &args![this, entry, name]).bool());
        assert_eq!(e.mem.u32(entry + 8), 0x10);
        e.set_global(FLAG_011ACDF9, 1u8);
        assert!(e.call(0x00af_b190, &args![this, entry, name]).bool());
        assert_eq!(e.mem.u32(entry + 0xc), 0x8000_0000);
        let other = text(&mut e, b"b.nif");
        assert!(!e.call(0x00af_b190, &args![this, entry, other]).bool());
        assert_eq!(e.mem.u32(entry + 8), 0x8000_0010);
        // Marked entries are skipped.
        assert!(!e.call(0x00af_b190, &args![this, entry, name]).bool());
        let entry = e.mem.alloc(16);
        assert!(!e.call(0x00af_b190, &args![this, entry, 0u32]).bool());
        assert_eq!(e.mem.u32(entry + 8), 0x8000_0000);
    }

    #[test]
    fn hash_compare_orders_high_then_low_words() {
        let (mut e, _) = engine();
        let a = hash(&mut e, 0x1_0000_0005);
        let b = hash(&mut e, 0x1_0000_0006);
        let c = hash(&mut e, 0x2_0000_0000);
        assert_eq!(e.call(0x00af_ce90, &args![a, b]).i32(), -1);
        assert_eq!(e.call(0x00af_ce90, &args![b, a]).i32(), 1);
        assert_eq!(e.call(0x00af_ce90, &args![c, b]).i32(), 1);
        assert_eq!(e.call(0x00af_ce90, &args![a, a]).i32(), 0);
    }

    #[test]
    fn name_hash_compare_handles_wildcards() {
        let (mut e, _) = engine();
        let flags = Rc::new(RefCell::new(Vec::<u8>::new()));
        let given = flags.clone();
        e.register_double(HASH_FLAG, move |_, _| {
            given.borrow_mut().remove(0).into_ret()
        });
        let wildcard = hash(&mut e, 0x0000_0000_2a00_0000);
        let other = hash(&mut e, 0x0000_0005_0000_0000);
        *flags.borrow_mut() = vec![1u8, 1];
        assert_eq!(e.call(0x00af_cf00, &args![wildcard, other]).i32(), 0);
        *flags.borrow_mut() = vec![1u8, 0];
        assert_eq!(e.call(0x00af_cf00, &args![wildcard, other]).i32(), 1);
        let small = hash(&mut e, 0x0000_0004_0000_0000);
        assert_eq!(e.call(0x00af_cf00, &args![small, other]).i32(), -1);
        assert_eq!(e.call(0x00af_cf00, &args![other, small]).i32(), 1);
        let same_word = hash(&mut e, 0x0000_0005_0001_0000);
        assert_eq!(e.call(0x00af_cf00, &args![same_word, other]).i32(), 1);
        *flags.borrow_mut() = vec![3u8, 3];
        assert_eq!(e.call(0x00af_cf00, &args![other, other]).i32(), 0);
    }

    #[test]
    fn header_constructor_fills_the_bsa_defaults() {
        let (mut e, _) = engine();
        let header = e.mem.alloc(0x24);
        e.mem.write(header, &[0xff; 0x24]);
        assert_eq!(e.call(0x00af_d8b0, &args![header]).u32(), header);
        assert_eq!(e.mem.u32(header), 0x0041_5342);
        assert_eq!(e.mem.u32(header + 4), 0x68);
        assert_eq!(e.mem.u32(header + 8), 0x24);
        assert_eq!(e.mem.bytes(header + 0xc, 0x16), vec![0; 0x16]);
    }

    #[test]
    fn directory_hash_search_asks_each_directory() {
        let (mut e, _) = engine();
        let this = archive(&mut e);
        directories(&mut e, this, &[(1, &[]), (2, &[]), (3, &[])]);
        e.register(DIRECTORY_HAS, |e, a| (e.mem.u64(a[0]) == 2).into_ret());
        let header = this.addr() + Archive::iTag.off;
        assert!(e.call(0x00af_dc00, &args![header, 0x77u32]).bool());
        let asked = calls_to(&e, DIRECTORY_HAS);
        assert_eq!(asked.len(), 2);
        assert_eq!(asked[0][1], 0x77);
        e.set(this, Archive::iDirectories, 1);
        assert!(!e.call(0x00af_dc00, &args![header, 0x77u32]).bool());
    }

    /// A list node holding `archive` (with a reference) and `next`.
    fn node(e: &mut Engine, archive: u32, next: u32) -> u32 {
        let at = e.mem.alloc(8);
        e.mem.set_u32(at, archive);
        e.mem.set_u32(at + 4, next);
        if archive != 0 {
            let count = e.mem.u32(archive + ARCHIVE_REF_COUNT) + 1;
            e.mem.set_u32(archive + ARCHIVE_REF_COUNT, count);
        }
        at
    }

    fn count(e: &Engine, archive: Ptr<Archive>) -> u32 {
        e.get(archive, Archive::m_uiRefCount)
    }

    #[test]
    fn pointer_destructor_releases_and_deletes_at_zero() {
        let (mut e, _) = engine();
        let a = archive(&mut e);
        e.set(a, Archive::m_uiRefCount, 2);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, a.addr());
        e.call(0x00af_c720, &args![slot]);
        assert_eq!(count(&e, a), 1);
        assert!(calls_to(&e, DELETE_THIS).is_empty());
        e.call(0x00af_c720, &args![slot]);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![a.addr() + 0x158]]);
        e.mem.set_u32(slot, 0);
        e.call(0x00af_c720, &args![slot]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 2);
    }

    #[test]
    fn pointer_assignments_swap_references() {
        let (mut e, _) = engine();
        let (a, b) = (archive(&mut e), archive(&mut e));
        e.set(a, Archive::m_uiRefCount, 1);
        let slot = e.mem.alloc(4);
        assert_eq!(e.call(0x00af_d1b0, &args![slot, a]).u32(), slot);
        assert_eq!(count(&e, a), 2);
        e.call(0x00af_d1b0, &args![slot, a]);
        assert_eq!(count(&e, a), 2);
        let other = e.mem.alloc(4);
        e.mem.set_u32(other, b.addr());
        assert_eq!(e.call(0x00af_c770, &args![slot, other]).u32(), slot);
        assert_eq!((count(&e, a), count(&e, b)), (1, 1));
        assert_eq!(e.mem.u32(slot), b.addr());
        e.call(0x00af_d1b0, &args![slot, 0u32]);
        assert_eq!(count(&e, b), 0);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![b.addr() + 0x158]]);
    }

    #[test]
    fn add_head_and_add_tail_link_new_nodes() {
        let (mut e, _) = engine();
        let (a, b, c) = (archive(&mut e), archive(&mut e), archive(&mut e));
        let head = node(&mut e, 0, 0);
        let item = e.mem.alloc(4);
        // A null item adds nothing; an empty head takes the item.
        e.call(0x00af_c880, &args![head, item]);
        assert!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
        e.mem.set_u32(item, a.addr());
        e.call(0x00af_c880, &args![head, item]);
        assert_eq!(e.mem.u32(head), a.addr());
        e.mem.set_u32(item, b.addr());
        e.call(0x00af_c880, &args![head, item]);
        let second = e.mem.u32(head + 4);
        assert_eq!((e.mem.u32(head), e.mem.u32(second)), (b.addr(), a.addr()));
        assert_eq!((count(&e, a), count(&e, b)), (1, 1));
        e.mem.set_u32(item, c.addr());
        e.call(0x00af_cc10, &args![head, item]);
        let third = e.mem.u32(second + 4);
        assert_eq!(e.mem.u32(third), c.addr());
        assert_eq!(e.mem.u32(third + 4), 0);
        assert_eq!(count(&e, c), 1);
        // A tail add on an empty list fills the head.
        let empty = node(&mut e, 0, 0);
        e.call(0x00af_cc10, &args![empty, item]);
        assert_eq!(e.mem.u32(empty), c.addr());
        assert_eq!(count(&e, c), 2);
    }

    #[test]
    fn remove_all_frees_the_nodes_and_clears_the_head() {
        let (mut e, _) = engine();
        let (a, b) = (archive(&mut e), archive(&mut e));
        let second = node(&mut e, b.addr(), 0);
        let head = node(&mut e, a.addr(), second);
        e.call(0x00af_caf0, &args![head]);
        assert_eq!(e.mem.u32(head), 0);
        assert_eq!(e.mem.u32(head + 4), 0);
        assert_eq!((count(&e, a), count(&e, b)), (0, 0));
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, second]]
        );
        // The node destructor empties, then releases its own archive.
        let lone = node(&mut e, a.addr(), 0);
        e.call(0x00af_c7f0, &args![lone]);
        assert_eq!(count(&e, a), 0);
    }

    #[test]
    fn names_are_rebuilt_for_every_listed_archive() {
        let (mut e, _) = engine();
        let a = archive(&mut e);
        // Already built: nothing to do. The owner state 1 goes to 2 and back.
        e.set(a, Archive::iFiles, 1);
        e.set(a, Archive::pFileNameStringArray, 1);
        e.set(a, Archive::pFileNameStringOffsets, 1);
        let owner = e.mem.alloc(0x60);
        e.mem.set_u32(owner + STATE_OFFSET, 1);
        let states = Rc::new(RefCell::new(vec![]));
        let seen = states.clone();
        e.register_double(STATE_OWNER, move |e, _| {
            seen.borrow_mut().push(e.mem.u32(owner + STATE_OFFSET));
            owner.into_ret()
        });
        e.call(0x00af_59f0, &args![]);
        assert!(calls_to(&e, STATE_OWNER).is_empty());
        let list = node(&mut e, a.addr(), 0);
        e.set_global(ARCHIVE_LIST, list);
        e.call(0x00af_59f0, &args![]);
        assert_eq!(*states.borrow(), vec![1, 1, 2]);
        assert_eq!(e.mem.u32(owner + STATE_OFFSET), 1);
        // A busy owner (state 3) is left alone.
        e.mem.set_u32(owner + STATE_OFFSET, 3);
        states.borrow_mut().clear();
        e.call(0x00af_59f0, &args![]);
        assert_eq!(*states.borrow(), vec![3]);
    }

    #[test]
    fn name_tables_are_built_from_the_file() {
        let (mut e, _) = engine();
        let a = archive(&mut e);
        directories(
            &mut e,
            a,
            &[(1, &[(0, 0, 0), (0, 0, 0)]), (2, &[(0, 0, 0)])],
        );
        e.set(a, Archive::iFiles, 3);
        e.set(a, Archive::iFileNamesLength, 8);
        e.set(a, Archive::iFileNameArrayOffset, 0x40);
        e.set(a, Archive::oFlags, 2);
        let mut slots = vec![0u32; 6];
        slots[5] = 0x0070_0010;
        e.put_vtable(0x0300_0100, &slots);
        e.mem.set_u32(a.addr(), 0x0300_0100);
        e.register(0x0070_0010, |_, _| Ret::default());
        e.mem.set_u32(a.addr() + 8, 0x0070_0020);
        e.register(0x0070_0020, |e, a| {
            e.mem.write(a[1], b"ab\0c\0de\0");
            a[2].into_ret()
        });
        e.call(0x00af_8f10, &args![a]);
        let row = |e: &Engine, d: u32| {
            let offsets = e.get(a, Archive::pFileNameStringOffsets);
            e.mem.u32(offsets + 4 * d)
        };
        assert_eq!(e.mem.u32(row(&e, 0)), 0);
        assert_eq!(e.mem.u32(row(&e, 0) + 4), 3);
        assert_eq!(e.mem.u32(row(&e, 1)), 5);
        assert_eq!(e.get(a, Archive::cArchiveFlags), 0x20);
        assert_eq!(calls_to(&e, 0x0070_0010), vec![vec![a.addr(), 0x40, 0]]);
        assert_eq!(e.mem.u32(a.addr() + 4), 8);
        // Without names only the blocks are made.
        e.set(a, Archive::oFlags, 0);
        e.set(a, Archive::pFileNameStringArray, 0);
        e.call(0x00af_8f10, &args![a]);
        assert_eq!(calls_to(&e, 0x0070_0020).len(), 1);
        // No files: nothing.
        e.set(a, Archive::iFiles, 0);
        e.call_log = Some(vec![]);
        e.call(0x00af_8f10, &args![a]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn table_frees_respect_the_kept_flag() {
        let (mut e, _) = engine();
        let a = archive(&mut e);
        directories(&mut e, a, &[(1, &[(0, 0, 0)]), (2, &[])]);
        let files = e.mem.u32(e.get(a, Archive::pDirectories) + 0xc);
        e.set(a, Archive::pDirectoryStringArray, 0x10);
        e.set(a, Archive::pDirectoryStringOffsets, 0x20);
        e.set(a, Archive::pFileNameStringArray, 0x30);
        let rows = e.mem.alloc(8);
        e.mem.set_u32(rows, 0x40);
        e.mem.set_u32(rows + 4, 0x50);
        e.set(a, Archive::pFileNameStringOffsets, rows);
        e.set(a, Archive::cArchiveFlags, 0x34);
        // Bit 2 keeps everything; the flag bits still clear.
        e.call(0x00af_9230, &args![a]);
        e.call(0x00af_92f0, &args![a, false]);
        e.call(0x00af_9410, &args![a]);
        assert!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        assert_eq!(e.get(a, Archive::cArchiveFlags), 4);
        e.set(a, Archive::cArchiveFlags, 0x30);
        e.call(0x00af_9230, &args![a]);
        e.call(0x00af_92f0, &args![a, true]);
        let freed: Vec<u32> = calls_to(&e, MEMORY_MANAGER_DEALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(freed, vec![0x10, 0x20, 0x30]);
        assert_eq!(e.get(a, Archive::pFileNameStringOffsets), rows);
        e.call(0x00af_92f0, &args![a, false]);
        e.call(0x00af_9410, &args![a]);
        let freed: Vec<u32> = calls_to(&e, MEMORY_MANAGER_DEALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        let directories = e.mem.u32(a.addr() + 0x184);
        assert_eq!(directories, 0);
        assert_eq!(&freed[3..6], &[0x40, 0x50, rows]);
        assert_eq!(freed[6], files);
        assert_eq!(e.get(a, Archive::cArchiveFlags), 0);
    }

    #[test]
    fn archive_destructor_tears_down_in_order() {
        let (mut e, _) = engine();
        e.register(BSFILE_DESTRUCTOR, |_, _| Ret::default());
        e.register(GAMEBRYO_FREE_SIZED, |_, _| Ret::default());
        let a = archive(&mut e);
        e.set_global(REF_OBJECT_COUNT, 3u32);
        e.call(0x00af_8e60, &args![a]);
        assert_eq!(e.mem.u32(a.addr()), ARCHIVE_VTABLE);
        assert_eq!(e.get(a, Archive::ref_object_vtable), REF_OBJECT_VTABLE);
        assert_eq!(e.global::<u32>(REF_OBJECT_COUNT), 2);
        assert_eq!(
            calls_to(&e, DELETE_CRITICAL_SECTION),
            vec![vec![a.addr() + 0x19c]]
        );
        assert_eq!(calls_to(&e, BSFILE_DESTRUCTOR), vec![vec![a.addr()]]);
        assert_eq!(e.call(0x00af_c680, &args![a, 0u32]).u32(), a.addr());
        assert!(calls_to(&e, GAMEBRYO_FREE_SIZED).is_empty());
        e.call(0x00af_c680, &args![a, 1u32]);
        assert_eq!(
            calls_to(&e, GAMEBRYO_FREE_SIZED),
            vec![vec![a.addr(), 0x1d0]]
        );
        // The NiRefObject part's slot moves back to the Archive.
        assert_eq!(
            e.call(0x00af_ce80, &args![a.addr() + 0x158, 1u32]).u32(),
            a.addr()
        );
        assert_eq!(calls_to(&e, GAMEBRYO_FREE_SIZED).len(), 2);
    }

    /// An archive whose m_pfnRead serves `table` from offset `start`, with
    /// a seek double at vtable +0x14 that records the position.
    fn readable(e: &mut Engine, table: &'static [u8], start: u32) -> Ptr<Archive> {
        let a = archive(e);
        let mut slots = vec![0u32; 0x2c / 4 + 1];
        slots[SEEK_SLOT as usize / 4] = 0x0070_0110;
        e.put_vtable(0x0300_0200, &slots);
        e.mem.set_u32(a.addr(), 0x0300_0200);
        let position = Rc::new(RefCell::new(0u32));
        let at = position.clone();
        e.register_double(0x0070_0110, move |_, w| {
            *at.borrow_mut() = w[1];
            Ret::default()
        });
        e.mem.set_u32(a.addr() + 8, 0x0070_0120);
        e.register_double(0x0070_0120, move |e, w| {
            let mut from = (*position.borrow() - start) as usize;
            let size = w[2] as usize;
            let bytes: Vec<u8> = (0..size)
                .map(|i| *table.get(from + i).unwrap_or(&0))
                .collect();
            e.mem.write(w[1], &bytes);
            from += size;
            *position.borrow_mut() = start + from as u32;
            w[2].into_ret()
        });
        a
    }

    #[test]
    fn file_name_comes_from_the_kept_table_or_the_file() {
        let (mut e, _) = engine();
        let a = readable(&mut e, b"one\0two\0six\0", 0x80);
        directories(
            &mut e,
            a,
            &[(1, &[(0, 0, 0), (0, 0, 0)]), (2, &[(0, 0, 0)])],
        );
        e.set(a, Archive::iFileNamesLength, 12);
        e.set(a, Archive::iFileNameArrayOffset, 0x80);
        let rows = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(4);
        e.mem.set_u32(first + 4, 4);
        e.mem.set_u32(second, 8);
        e.mem.set_u32(rows, first);
        e.mem.set_u32(rows + 4, second);
        e.set(a, Archive::pFileNameStringOffsets, rows);
        // No names in the archive: null.
        assert_eq!(e.call(0x00af_96d0, &args![a, 0u32, 1u32]).u32(), 0);
        e.set(a, Archive::oFlags, 2);
        // Walking the whole table (mode 0 answers false).
        let name = e.call(0x00af_96d0, &args![a, 1u32, 0u32]).u32();
        assert_eq!(e.mem.cstr(name), b"six");
        let name = e.call(0x00af_96d0, &args![a, 0u32, 1u32]).u32();
        assert_eq!(e.mem.cstr(name), b"two");
        // Off the end: empty.
        let name = e.call(0x00af_96d0, &args![a, 1u32, 1u32]).u32();
        assert_eq!(e.mem.cstr(name), b"");
        // One name at a time from its offset (mode 1 passes the flag byte).
        e.set_global(NAME_TABLE_MODE, 1u32);
        e.set(a, Archive::oFlags, 0x22);
        let name = e.call(0x00af_96d0, &args![a, 1u32, 0u32]).u32();
        assert_eq!(e.mem.cstr(name), b"six");
        // Kept names are answered from the table directly.
        e.set(a, Archive::cArchiveFlags, 0x20);
        let table = e.get(a, Archive::pFileNameStringArray);
        assert_eq!(e.call(0x00af_96d0, &args![a, 0u32, 1u32]).u32(), table + 4);
    }

    /// An archive with two directories (hashes 0x10 and 0x20), the second
    /// holding files with hashes 5 and 9 (sizes and offsets given).
    fn searchable(e: &mut Engine) -> Ptr<Archive> {
        let a = archive(e);
        directories(
            e,
            a,
            &[
                (0x10, &[(1, 0x10, 0x100)]),
                (0x20, &[(5, 0x20, 0x200), (9, 0x4000_0030, 0x8000_0300)]),
            ],
        );
        a
    }

    #[test]
    fn directory_lookup_tries_the_last_then_halves() {
        let (mut e, _) = engine();
        let a = searchable(&mut e);
        let out = e.mem.alloc(4);
        let wanted = hash(&mut e, 0x20);
        assert!(e.call(0x00af_9c50, &args![a, wanted, out, 0u32]).bool());
        assert_eq!(e.mem.u32(out), 1);
        assert_eq!(e.get(a, Archive::iLastDirectoryIndex), 1);
        assert_eq!(e.get(a, Archive::iLastFileIndex), u32::MAX);
        // The cached directory answers without touching the last file.
        e.set(a, Archive::iLastFileIndex, 0);
        assert!(e.call(0x00af_9c50, &args![a, wanted, out, 0u32]).bool());
        assert_eq!(e.get(a, Archive::iLastFileIndex), 0);
        let missing = hash(&mut e, 0x15);
        assert!(!e.call(0x00af_9c50, &args![a, missing, out, 0u32]).bool());
        assert_eq!(e.get(a, Archive::iLastFileIndex), u32::MAX);
        e.set(a, Archive::cArchiveFlags, 1);
        assert!(!e.call(0x00af_9c50, &args![a, wanted, out, 0u32]).bool());
    }

    #[test]
    fn directory_lookup_checks_names_for_collisions() {
        let (mut e, _) = engine();
        let a = searchable(&mut e);
        e.set_global(FLAG_SET_BY_00AF4460, 1u8);
        e.set(a, Archive::oFlags, 1);
        e.register(SPLITPATH_S, |e, w| {
            e.mem.set_cstr(w[3], b"meshes\\");
            Ret::default()
        });
        e.register(STRUPR_S, |e, w| {
            let upper: Vec<u8> = e.mem.cstr(w[0]).to_ascii_uppercase();
            e.mem.set_cstr(w[0], &upper);
            Ret::default()
        });
        e.register(WARNING, |_, _| Ret::default());
        let stored = text(&mut e, b"meshes");
        e.register_double(DIRECTORY_NAME, move |_, _| stored.into_ret());
        let out = e.mem.alloc(4);
        let wanted = hash(&mut e, 0x20);
        let name = text(&mut e, b"meshes\\a.nif");
        assert!(e.call(0x00af_9c50, &args![a, wanted, out, name]).bool());
        assert!(calls_to(&e, WARNING).is_empty());
        assert_eq!(calls_to(&e, DIRECTORY_NAME), vec![vec![a.addr(), 1]]);
        let other = text(&mut e, b"textures");
        e.register_double(DIRECTORY_NAME, move |_, _| other.into_ret());
        e.set(a, Archive::iLastDirectoryIndex, u32::MAX);
        assert!(!e.call(0x00af_9c50, &args![a, wanted, out, name]).bool());
        assert_eq!(calls_to(&e, WARNING).len(), 1);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 4);
    }

    #[test]
    fn file_lookup_honours_data_and_loose_files() {
        let (mut e, existing) = engine();
        strings(&mut e);
        let a = searchable(&mut e);
        let out = e.mem.alloc(4);
        let wanted = hash(&mut e, 9);
        // The entry for 9 has bit 31 of +0xc set: only `any` finds it.
        assert!(!e
            .call(0x00af_a120, &args![a, 1u32, wanted, out, 0u32, false])
            .bool());
        assert!(e
            .call(0x00af_a120, &args![a, 1u32, wanted, out, 0u32, true])
            .bool());
        assert_eq!(e.mem.u32(out), 1);
        assert_eq!(e.get(a, Archive::iLastFileIndex), 1);
        // The cached file; a loose copy on disk answers false at once.
        e.set_global(FLAG_011ACDF9, 1u8);
        existing.borrow_mut().push(b"Data\\x.nif".to_vec());
        let name = text(&mut e, b"x.nif");
        assert!(!e
            .call(0x00af_a120, &args![a, 1u32, wanted, out, name, true])
            .bool());
        // The search's find on disk answers false too (it is marked so).
        let five = hash(&mut e, 5);
        assert!(!e
            .call(0x00af_a120, &args![a, 1u32, five, out, name, false])
            .bool());
        // A fresh archive with loose files off: the search's find counts.
        e.set_global(FLAG_011ACDF9, 0u8);
        let b = searchable(&mut e);
        assert!(e
            .call(0x00af_a120, &args![b, 1u32, five, out, name, false])
            .bool());
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.get(b, Archive::iLastFileIndex), 0);
        e.set(b, Archive::cArchiveFlags, 1);
        assert!(!e
            .call(0x00af_a120, &args![b, 1u32, five, out, 0u32, false])
            .bool());
    }

    #[test]
    fn file_lookup_checks_names_for_collisions() {
        let (mut e, _) = engine();
        let a = searchable(&mut e);
        e.set_global(FLAG_SET_BY_00AF4460, 1u8);
        e.set(a, Archive::oFlags, 2);
        e.register(SPLITPATH_S, |e, w| {
            e.mem.set_cstr(w[5], b"a.nif");
            Ret::default()
        });
        e.register(STRTOK_S, |e, w| {
            let mut text = e.mem.cstr(w[0]);
            if let Some(dot) = text.iter().position(|&c| c == b'.') {
                text.truncate(dot);
            }
            e.mem.set_cstr(w[0], &text);
            Ret::default()
        });
        e.register(STRUPR_S, |_, _| Ret::default());
        e.register(WARNING, |_, _| Ret::default());
        // The archive's name for the file comes from the kept table.
        e.set(a, Archive::cArchiveFlags, 0x20);
        let table = text(&mut e, b"A.dds");
        e.set(a, Archive::pFileNameStringArray, table);
        let rows = e.mem.alloc(8);
        let row = e.mem.alloc(8);
        e.mem.set_u32(rows + 4, row);
        e.set(a, Archive::pFileNameStringOffsets, rows);
        let out = e.mem.alloc(4);
        let five = hash(&mut e, 5);
        let name = text(&mut e, b"meshes\\a.nif");
        assert!(e
            .call(0x00af_a120, &args![a, 1u32, five, out, name, false])
            .bool());
        assert!(calls_to(&e, WARNING).is_empty());
        e.mem.set_cstr(table, b"b.nif");
        e.set(a, Archive::iLastFileIndex, u32::MAX);
        assert!(!e
            .call(0x00af_a120, &args![a, 1u32, five, out, name, false])
            .bool());
        assert_eq!(calls_to(&e, WARNING).len(), 1);
    }

    #[test]
    fn entry_lookups_combine_directory_and_file() {
        let (mut e, _) = engine();
        let a = searchable(&mut e);
        let directory = hash(&mut e, 0x20);
        let five = hash(&mut e, 5);
        let entry = e.call(0x00af_a6e0, &args![a, directory, five, 0u32]).u32();
        let files = e.mem.u32(e.get(a, Archive::pDirectories) + 0x1c);
        assert_eq!(entry, files);
        let missing = hash(&mut e, 6);
        assert_eq!(
            e.call(0x00af_a6e0, &args![a, directory, missing, 0u32])
                .u32(),
            0
        );
        let outs = e.mem.alloc(8);
        assert!(e
            .call(
                0x00af_9bf0,
                &args![a, directory, five, outs, outs + 4, 0u32]
            )
            .bool());
        assert_eq!((e.mem.u32(outs), e.mem.u32(outs + 4)), (1, 0));
        let nowhere = hash(&mut e, 0x30);
        assert!(!e
            .call(0x00af_9bf0, &args![a, nowhere, five, outs, outs + 4, 0u32])
            .bool());
    }

    /// Doubles for the path helpers the manager's lookups use: the trim
    /// answers its path, the hash writes the directory hash 0x20 and the
    /// file hash 5.
    fn manager(e: &mut Engine) {
        e.register(TRIM_FILE_NAME, |_, w| w[0].into_ret());
        e.register(HASH_PATH, |e, w| {
            e.mem.set_u64(w[1], 0x20);
            e.mem.set_u64(w[2], 5);
            Ret::default()
        });
    }

    #[test]
    fn archive_for_a_path_is_searched_by_type() {
        let (mut e, _) = engine();
        manager(&mut e);
        let path = text(&mut e, b"meshes\\a.nif");
        assert_eq!(e.call(0x00af_6160, &args![path, 1u32]).u32(), 0);
        let (first, second) = (searchable(&mut e), searchable(&mut e));
        e.mem.set_u16(first.addr() + 0x180, 2);
        e.mem.set_u16(second.addr() + 0x180, 1);
        let tail = node(&mut e, second.addr(), 0);
        let head = node(&mut e, first.addr(), tail);
        e.set_global(ARCHIVE_LIST, head);
        assert_eq!(e.call(0x00af_6160, &args![path, 1u32]).u32(), second.addr());
        assert_eq!(e.call(0x00af_6160, &args![path, 3u32]).u32(), first.addr());
        assert_eq!(e.call(0x00af_6160, &args![path, 4u32]).u32(), 0);
        assert_eq!((count(&e, first), count(&e, second)), (1, 1));
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION).len(),
            calls_to(&e, LEAVE_CRITICAL_SECTION).len()
        );
        assert!(e.call(0x00af_6320, &args![path, 1u32]).bool());
        assert!(!e.call(0x00af_6320, &args![path, 4u32]).bool());
        // 0xffff takes the type from the extension.
        type_table(&mut e);
        let named = text(&mut e, b"x.b");
        assert_eq!(
            e.call(0x00af_6160, &args![named, 0xffffu32]).u32(),
            first.addr()
        );
    }

    #[test]
    fn manager_get_file_makes_the_file_object() {
        let (mut e, _) = engine();
        manager(&mut e);
        e.register(BSFILE_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BSFILE_SET_ENDIAN_SWAP, |_, _| Ret::default());
        let a = searchable(&mut e);
        e.mem.set_u16(a.addr() + 0x180, 1);
        let head = node(&mut e, a.addr(), 0);
        e.set_global(ARCHIVE_LIST, head);
        e.set_global(ARCHIVE_FILE_BUFFER_SIZE, 0x1000u32);
        let path = text(&mut e, b"meshes\\a.nif");
        let made = e.call(0x00af_5fa0, &args![path, 7u32, 1u32]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.mem.u32(made), ARCHIVE_FILE_VTABLE);
        assert_eq!(e.mem.u32(made + 0x158), a.addr());
        assert_eq!(e.mem.u32(made + 0x15c), 0x200);
        assert_eq!(e.mem.u32(made + 0x154), 0x20);
        assert_eq!(e.mem.cstr(made + 0x44), b"meshes\\a.nif");
        assert_eq!(count(&e, a), 2);
        assert_eq!(e.call(0x00af_5fa0, &args![path, 7u32, 2u32]).u32(), 0);
        assert!(calls_to(&e, ENTER_CRITICAL_SECTION).is_empty());
    }

    #[test]
    fn file_objects_follow_the_compression_bits() {
        let (mut e, _) = engine();
        e.register(BSFILE_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BSFILE_SET_ENDIAN_SWAP, |_, _| Ret::default());
        e.register(ARCHIVE_FILE_READ, |e, w| {
            e.mem.set_u32(w[1], 0x40);
            4u32.into_ret()
        });
        e.register(INFLATE_INIT, |_, _| 0u32.into_ret());
        let a = searchable(&mut e);
        e.set_global(ARCHIVE_FILE_BUFFER_SIZE, 0x1000u32);
        let path = text(&mut e, b"x");
        let plain = e.call(0x00af_a550, &args![a, 1u32, 0u32, 0u32, path]).u32();
        assert_eq!(e.mem.u32(plain), ARCHIVE_FILE_VTABLE);
        // The second file has the opposite compression bit: compressed.
        let packed = e.call(0x00af_a550, &args![a, 1u32, 1u32, 0u32, path]).u32();
        assert_eq!(packed, 0, "bit 31 of +0xc against the flags: no data");
        e.set(a, Archive::cArchiveFlags, 8);
        let packed = e.call(0x00af_a550, &args![a, 1u32, 1u32, 0u32, path]).u32();
        assert_eq!(e.mem.u32(packed), COMPRESSED_ARCHIVE_FILE_VTABLE);
        assert_eq!(e.mem.u32(packed + 0x15c), 0x300);
        assert_eq!(e.mem.u32(packed + 0x154), 0x30);
        assert_eq!(e.mem.u32(packed + 0x168), 0x40);
        let sizes: Vec<u32> = calls_to(&e, MEMORY_MANAGER_ALLOCATE)
            .iter()
            .map(|c| c[1])
            .filter(|&s| s == 0x160 || s == 0x178)
            .collect();
        assert_eq!(sizes, vec![0x160, 0x178]);
        // The same by entry address; a null entry makes nothing.
        e.set(a, Archive::cArchiveFlags, 0);
        let entry = e.mem.u32(e.get(a, Archive::pDirectories) + 0x1c);
        let by_entry = e.call(0x00af_a750, &args![a, entry, 0u32, path]).u32();
        assert_eq!(e.mem.u32(by_entry), ARCHIVE_FILE_VTABLE);
        assert_eq!(e.call(0x00af_a750, &args![a, 0u32, 0u32, path]).u32(), 0);
    }

    #[test]
    fn directory_owner_lookups_use_type_masks() {
        let (mut e, _) = engine();
        e.register(DIRECTORY_HAS, |e, w| {
            (e.mem.u64(w[0]) == e.mem.u64(w[1])).into_ret()
        });
        let path = text(&mut e, b"x");
        let (first, second) = (searchable(&mut e), searchable(&mut e));
        e.mem.set_u16(first.addr() + 0x180, 2);
        e.mem.set_u16(second.addr() + 0x180, 1);
        let tail = node(&mut e, second.addr(), 0);
        let head = node(&mut e, first.addr(), tail);
        e.set_global(ARCHIVE_LIST, head);
        let wanted = hash(&mut e, 0x20);
        assert_eq!(
            e.call(0x00af_6910, &args![wanted, 1u32]).u32(),
            second.addr()
        );
        assert_eq!(
            e.call(0x00af_6910, &args![wanted, 0xffffu32]).u32(),
            first.addr()
        );
        // A type nobody has falls back to the others.
        assert_eq!(
            e.call(0x00af_6910, &args![wanted, 4u32]).u32(),
            first.addr()
        );
        let missing = hash(&mut e, 0x30);
        let leaves_before = calls_to(&e, LEAVE_CRITICAL_SECTION).len();
        assert_eq!(e.call(0x00af_6910, &args![missing, 0xffffu32]).u32(), 0);
        assert_eq!(calls_to(&e, LEAVE_CRITICAL_SECTION).len(), leaves_before);
        assert_eq!(e.call(0x00af_6910, &args![missing, 1u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, LEAVE_CRITICAL_SECTION).len(),
            leaves_before + 1
        );
        // The entry's archive makes the file object.
        e.register(BSFILE_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BSFILE_SET_ENDIAN_SWAP, |_, _| Ret::default());
        e.set_global(ARCHIVE_FILE_BUFFER_SIZE, 0x1000u32);
        let entry = e.mem.u32(e.get(first, Archive::pDirectories) + 0x1c);
        e.mem.set_u64(entry, 0x20);
        let made = e.call(0x00af_6340, &args![0u32, entry, 0u32, path]).u32();
        assert_eq!(e.mem.u32(made + 0x158), first.addr());
        e.mem.set_u64(entry, 0x30);
        assert_eq!(
            e.call(0x00af_6340, &args![0u32, entry, 0u32, path]).u32(),
            0
        );
    }

    #[test]
    fn typed_entry_lookup_walks_the_list() {
        let (mut e, _) = engine();
        let (first, second) = (searchable(&mut e), searchable(&mut e));
        e.mem.set_u16(first.addr() + 0x180, 2);
        e.mem.set_u16(second.addr() + 0x180, 1);
        let tail = node(&mut e, second.addr(), 0);
        let head = node(&mut e, first.addr(), tail);
        e.set_global(ARCHIVE_LIST, head);
        let directory = hash(&mut e, 0x20);
        let five = hash(&mut e, 5);
        let found = e
            .call(0x00af_6540, &args![0u8, directory, five, 0u32])
            .u32();
        assert_eq!(
            found,
            e.mem.u32(e.get(second, Archive::pDirectories) + 0x1c)
        );
        let found = e
            .call(0x00af_6540, &args![0x21u8, directory, five, 0u32])
            .u32();
        assert_eq!(found, e.mem.u32(e.get(first, Archive::pDirectories) + 0x1c));
        assert_eq!(
            e.call(0x00af_6540, &args![3u8, directory, five, 0u32])
                .u32(),
            0
        );
        assert_eq!((count(&e, first), count(&e, second)), (1, 1));
    }

    #[test]
    fn archive_by_file_name() {
        let (mut e, _) = engine();
        let name = text(&mut e, b"Fallout - Meshes.bsa");
        assert_eq!(e.call(0x00af_5320, &args![name]).u32(), 0);
        let a = archive(&mut e);
        let mut slots = vec![0u32; ARCHIVE_SLOT_18 as usize / 4 + 1];
        slots[ARCHIVE_SLOT_18 as usize / 4] = 0x0070_0200;
        e.put_vtable(0x0300_0300, &slots);
        e.mem.set_u32(a.addr(), 0x0300_0300);
        let own = text(&mut e, b"fallout - meshes.bsa");
        e.register_double(0x0070_0200, move |_, _| own.into_ret());
        let head = node(&mut e, a.addr(), 0);
        e.set_global(ARCHIVE_LIST, head);
        assert_eq!(e.call(0x00af_5320, &args![name]).u32(), a.addr());
        let other = text(&mut e, b"x.bsa");
        assert_eq!(e.call(0x00af_5320, &args![other]).u32(), 0);
        assert_eq!(calls_to(&e, LEAVE_CRITICAL_SECTION).len(), 2);
    }

    fn archive_file(e: &mut Engine, size: u32) -> Ptr<ArchiveFile> {
        let this: Ptr<ArchiveFile> = e.new_object();
        e.set(this.cast::<BSFile>(), BSFile::iFileSize, size);
        this
    }

    #[test]
    fn archive_file_open_allocates_the_buffer_once() {
        let (mut e, _) = engine();
        let this = archive_file(&mut e, 0x40);
        let file = this.cast::<NiFile>();
        e.set(file, NiFile::m_uiBufferAllocSize, 0x20);
        e.set(file, NiFile::m_uiPos, 3);
        assert!(e.call(0x00af_b5f0, &args![this, 0u8, 0u8]).bool());
        assert_ne!(e.get(file, NiFile::m_pBuffer), 0);
        assert_eq!(e.get(file, NiFile::m_uiPos), 0);
        assert_eq!(e.get(file, NiFile::m_bGood), 1);
        assert!(e.call(0x00af_c060, &args![this, 1u8, 1u8]).bool());
        assert_eq!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).len(), 1);
        let other = archive_file(&mut e, 0);
        assert!(e.call(0x00af_b5f0, &args![other, 0u8, 0u8]).bool());
        assert_eq!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).len(), 1);
    }

    #[test]
    fn archive_file_seek_clamps_and_moves_the_buffer_position() {
        let (mut e, _) = engine();
        e.register(NI_FILE_FLUSH, |_, _| 1u32.into_ret());
        let this = archive_file(&mut e, 0x100);
        let bs = this.cast::<BSFile>();
        let file = this.cast::<NiFile>();
        e.set(file, NiFile::m_uiBufferReadSize, 0x80);
        e.call(0x00af_b670, &args![this, 0x10u32, 0u32]);
        assert_eq!(e.get(bs, BSFile::iTrueFilePos), 0x10);
        assert_eq!(e.get(file, NiFile::m_uiPos), 0x10);
        e.call(0x00af_b670, &args![this, 0x500u32, 1u32]);
        assert_eq!(e.get(bs, BSFile::iTrueFilePos), 0x100);
        assert_eq!(e.get(file, NiFile::m_uiPos), 0x100);
        assert_eq!(calls_to(&e, NI_FILE_FLUSH).len(), 1);
        e.call(0x00af_b670, &args![this, (-0x30i32) as u32, 2u32]);
        assert_eq!(e.get(bs, BSFile::iTrueFilePos), 0xd0);
        e.call(0x00af_b670, &args![this, 0x900u32, 2u32]);
        assert_eq!(e.get(bs, BSFile::iTrueFilePos), 0);
        e.call(0x00af_b670, &args![this, 0x900u32, 0u32]);
        assert_eq!(e.get(bs, BSFile::iTrueFilePos), 0x100);
    }

    #[test]
    fn sizes_and_compressed_seek() {
        let (mut e, _) = engine();
        let this = archive_file(&mut e, 0x55);
        assert_eq!(e.call(0x00af_b780, &args![this]).u32(), 0x55);
        let packed: Ptr<CompressedArchiveFile> = e.new_object();
        e.set(packed, CompressedArchiveFile::iUncompressedFileSize, 0x99);
        assert_eq!(e.call(0x00af_c0a0, &args![packed]).u32(), 0x99);
        assert_eq!(e.call(0x00af_c0c0, &args![packed]).u32(), 0x99);
        let mut slots = vec![0u32; 0x48 / 4];
        slots[0x44 / 4] = 0x0070_0300;
        e.put_vtable(0x0300_0400, &slots);
        e.mem.set_u32(packed.addr(), 0x0300_0400);
        e.register(0x0070_0300, |_, _| Ret::default());
        e.call(0x00af_c080, &args![packed, 0x12u32, 2u32]);
        assert_eq!(
            calls_to(&e, 0x0070_0300),
            vec![vec![packed.addr(), 0, 0x12]]
        );
    }

    #[test]
    fn read_f_clamps_to_the_file_and_counts() {
        let (mut e, _) = engine();
        e.register(ARCHIVE_FILE_READ, |_, w| w[2].into_ret());
        let this = archive_file(&mut e, 0x30);
        let bs = this.cast::<BSFile>();
        e.set(bs, BSFile::iTrueFilePos, 0x20);
        assert_eq!(
            e.call(0x00af_bbf0, &args![this, 0x900u32, 0x40u32]).u32(),
            0x10
        );
        assert_eq!(e.get(bs, BSFile::iTrueFilePos), 0x30);
        e.set(bs, BSFile::iTrueFilePos, 0);
        assert_eq!(e.call(0x00af_bf80, &args![this, 0x900u32, 8u32]).u32(), 8);
        assert_eq!(
            calls_to(&e, ARCHIVE_FILE_READ),
            vec![vec![this.addr(), 0x900, 0x10], vec![this.addr(), 0x900, 8]]
        );
    }

    #[test]
    fn endian_swap_installs_the_read_pairs() {
        let (mut e, _) = engine();
        e.register(BSFILE_SET_ENDIAN_SWAP, |_, _| Ret::default());
        let this = archive_file(&mut e, 0);
        e.call(0x00af_bfa0, &args![this, false]);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x00af_bf80);
        assert_eq!(e.mem.u32(this.addr() + 0xc), BSFILE_PLAIN_WRITE);
        e.call(0x00af_c620, &args![this, false]);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x00af_c660);
        e.call(0x00af_bfa0, &args![this, true]);
        e.call(0x00af_c620, &args![this, true]);
        assert_eq!(
            calls_to(&e, BSFILE_SET_ENDIAN_SWAP),
            vec![vec![this.addr(), 1], vec![this.addr(), 1]]
        );
    }

    #[test]
    fn archive_file_destructors_release_and_free() {
        let (mut e, _) = engine();
        e.register(BSFILE_DESTRUCTOR, |_, _| Ret::default());
        e.register(INFLATE_END, |_, _| Ret::default());
        let a = archive(&mut e);
        e.set(a, Archive::m_uiRefCount, 2);
        let this = archive_file(&mut e, 0);
        e.set(this, ArchiveFile::spArchive, a.addr());
        e.set(this.cast::<NiFile>(), NiFile::m_pBuffer, 0x6600);
        e.set(this.cast::<NiFile>(), NiFile::m_bGood, 1);
        assert_eq!(e.call(0x00af_c6c0, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(count(&e, a), 1);
        assert_eq!(e.get(this.cast::<NiFile>(), NiFile::m_bGood), 0);
        let freed: Vec<u32> = calls_to(&e, MEMORY_MANAGER_DEALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(freed, vec![0x6600, this.addr()]);
        let packed: Ptr<CompressedArchiveFile> = e.new_object();
        e.set(packed, CompressedArchiveFile::pZStream, 0x7700);
        e.set(packed, CompressedArchiveFile::pUncompressedBuffer, 0x7800);
        e.call(0x00af_c6f0, &args![packed, 0u32]);
        assert_eq!(e.mem.u32(packed.addr()), ARCHIVE_FILE_VTABLE);
        assert_eq!(calls_to(&e, INFLATE_END), vec![vec![0x7700]]);
        let freed: Vec<u32> = calls_to(&e, MEMORY_MANAGER_DEALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(&freed[2..], &[0x7700, 0x7800]);
        assert_eq!(calls_to(&e, BSFILE_DESTRUCTOR).len(), 2);
    }

    /// An archive whose `NiFile` buffer the archive file reads through.
    fn backing(e: &mut Engine, data: &'static [u8]) -> Ptr<Archive> {
        let a = archive(e);
        let mut slots = vec![0u32; ARCHIVE_SLOT_18 as usize / 4 + 1];
        slots[ARCHIVE_SLOT_18 as usize / 4] = 0x0070_0400;
        e.put_vtable(0x0300_0500, &slots);
        e.mem.set_u32(a.addr(), 0x0300_0500);
        e.register(0x0070_0400, |_, _| Ret::default());
        let buffer = e.mem.alloc(0x40);
        let file = a.cast::<NiFile>();
        e.set(file, NiFile::m_pBuffer, buffer);
        e.set(file, NiFile::m_uiBufferAllocSize, 0x10);
        e.set(a.cast::<BSFile>(), BSFile::iFileSize, data.len() as u32);
        e.register(NI_FILE_GET_BUFFER, |e, w| e.mem.u32(w[0] + 0x20).into_ret());
        e.register(NI_FILE_SEEK, |e, w| {
            let position = e.mem.u32(w[0] + 0x1c).wrapping_add(w[1]);
            e.mem.set_u32(w[0] + 0x1c, position);
            Ret::default()
        });
        e.register_double(NI_FILE_READ_DIRECT, move |e, w| {
            let position = e.mem.u32(w[0] + 0x1c) as usize;
            let size = (w[2] as usize).min(data.len().saturating_sub(position));
            e.mem.write(w[1], &data[position..position + size]);
            e.mem.set_u32(w[0] + 0x1c, (position + size) as u32);
            (size as u32).into_ret()
        });
        a
    }

    #[test]
    fn data_read_uses_the_archive_buffer_then_the_file() {
        let (mut e, _) = engine();
        let a = backing(&mut e, b"0123456789abcdefghijklmnopqrstuvwxyz");
        let this = archive_file(&mut e, 0x20);
        e.set(this, ArchiveFile::spArchive, a.addr());
        e.set(this, ArchiveFile::iArchiveOffset, 4);
        let out = e.mem.alloc(0x40);
        // A short read refills the archive's buffer.
        assert_eq!(e.call(0x00af_b7a0, &args![this, out, 3u32, 0u32]).u32(), 3);
        assert_eq!(e.mem.bytes(out, 3), b"456");
        let seeks = calls_to(&e, NI_FILE_SEEK);
        assert_eq!(seeks, vec![vec![a.addr(), 4, 0]]);
        // The seek moved the archive file to 4, the read went on to 7.
        let own = a.cast::<NiFile>();
        assert_eq!(e.get(own, NiFile::m_uiBufferReadSize), 3);
        assert_eq!(e.get(a.cast::<BSFile>(), BSFile::iTrueFilePos), 7);
        // Inside the buffered range: copied from the buffer alone.
        e.set(own, NiFile::m_uiCurrentFilePos, 7);
        e.set(this.cast::<BSFile>(), BSFile::iTrueFilePos, 1);
        let out = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00af_b7a0, &args![this, out, 2u32, 0u32]).u32(), 2);
        assert_eq!(e.mem.bytes(out, 2), b"56");
        assert_eq!(calls_to(&e, NI_FILE_READ_DIRECT).len(), 1);
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION).len(),
            calls_to(&e, LEAVE_CRITICAL_SECTION).len()
        );
        // A long read goes straight into the caller's buffer.
        e.set(this.cast::<BSFile>(), BSFile::iTrueFilePos, 0);
        let out = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x00af_b7a0, &args![this, out, 0x18u32, 0u32]).u32(),
            0x18
        );
        let direct = calls_to(&e, NI_FILE_READ_DIRECT);
        assert_eq!(direct[1][1], out + 3);
    }

    #[test]
    fn data_read_stops_at_the_archive_end() {
        let (mut e, _) = engine();
        let a = backing(&mut e, b"0123");
        let this = archive_file(&mut e, 0x10);
        e.set(this, ArchiveFile::spArchive, a.addr());
        e.set(this, ArchiveFile::iArchiveOffset, 8);
        let out = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00af_b7a0, &args![this, out, 4u32, 0u32]).u32(), 0);
        assert!(calls_to(&e, NI_FILE_READ_DIRECT).is_empty());
        assert_eq!(e.get(a.cast::<BSFile>(), BSFile::iTrueFilePos), 8);
    }

    #[test]
    fn constructors_set_up_plain_and_compressed_files() {
        let (mut e, _) = engine();
        e.register(BSFILE_BASE_CONSTRUCTOR, |_, _| Ret::default());
        let a = backing(&mut e, b"\x03abcrest of the data");
        e.set(a, Archive::oFlags, NAMES_IN_DATA);
        e.set_global(ARCHIVE_FILE_BUFFER_SIZE, 0x10u32);
        let block = e.mem.alloc(0x160);
        let name = text(&mut e, b"x.nif");
        let made = e
            .call(0x00af_b300, &args![block, name, a, 0u32, 0x14u32, 0u32])
            .u32();
        assert_eq!(made, block);
        let file = Ptr::<NiFile>::new(block);
        // Capped buffer, read at once because names are in the data.
        assert_eq!(e.get(file, NiFile::m_uiBufferAllocSize), 0x10);
        assert_eq!(e.mem.cstr(block + 0x44), b"abc");
        assert_eq!(e.get(file, NiFile::m_uiPos), 4);
        assert_eq!(e.mem.u32(block + 0x150), 4);
        assert_eq!(count(&e, a), 1);
        // A whole-file read (-1) is not capped.
        e.set(a, Archive::oFlags, 0);
        let block = e.mem.alloc(0x160);
        e.call(0x00af_b300, &args![block, 0u32, a, 0u32, 0x14u32, u32::MAX]);
        assert_eq!(
            e.get(Ptr::<NiFile>::new(block), NiFile::m_uiBufferAllocSize),
            0x14
        );
        assert_eq!(e.mem.cstr(block + 0x44), b"");
        // The compressed one: inflateInit failing drops the stream.
        e.register(INFLATE_INIT, |_, _| 1u32.into_ret());
        e.register(INFLATE_END, |_, _| Ret::default());
        e.register(ARCHIVE_FILE_READ, |e, w| {
            e.mem.set_u32(w[1], 0x99);
            4u32.into_ret()
        });
        let block = e.mem.alloc(0x178);
        e.call(0x00af_c430, &args![block, 0u32, a, 0u32, 0x14u32, 0u32]);
        let packed = Ptr::<CompressedArchiveFile>::new(block);
        assert_eq!(e.mem.u32(block), COMPRESSED_ARCHIVE_FILE_VTABLE);
        assert_eq!(e.get(packed, CompressedArchiveFile::pZStream), 0);
        assert_eq!(
            e.get(packed, CompressedArchiveFile::iUncompressedFileSize),
            0x99
        );
        assert_eq!(calls_to(&e, INFLATE_END).len(), 1);
        let init = calls_to(&e, INFLATE_INIT);
        assert_eq!(&init[0][1..], &[TEXT_ZLIB_VERSION, 0x38]);
    }

    #[test]
    fn compressed_read_inflates_into_the_buffer() {
        let (mut e, _) = engine();
        let a = backing(&mut e, b"compressedcompressed");
        let packed: Ptr<CompressedArchiveFile> = e.new_object();
        let file = packed.cast::<NiFile>();
        e.set(
            packed.cast::<ArchiveFile>(),
            ArchiveFile::spArchive,
            a.addr(),
        );
        e.set(packed.cast::<BSFile>(), BSFile::iFileSize, 0x14);
        let input = e.mem.alloc(0x10);
        e.set(file, NiFile::m_pBuffer, input);
        e.set(file, NiFile::m_uiBufferAllocSize, 0x10);
        e.set(file, NiFile::m_uiBufferReadSize, 0x10);
        let stream = e.mem.alloc(0x40);
        let output = e.mem.alloc(8);
        e.set(packed, CompressedArchiveFile::pZStream, stream);
        e.set(packed, CompressedArchiveFile::pUncompressedBuffer, output);
        e.set(packed, CompressedArchiveFile::iUncompressedBufferSize, 8);
        // Each inflate takes 2 input bytes and fills 4 output bytes with
        // the letter `a` plus its call number.
        let calls = Rc::new(RefCell::new(0u8));
        let counter = calls.clone();
        e.register_double(INFLATE, move |e, w| {
            let z = w[0];
            let n = *counter.borrow();
            *counter.borrow_mut() += 1;
            let out = e.mem.u32(z + Z_NEXT_OUT);
            e.mem.write(out, &[b'a' + n; 4]);
            e.mem.set_u32(z + Z_NEXT_OUT, out + 4);
            let left = e.mem.u32(z + Z_AVAIL_OUT) - 4;
            e.mem.set_u32(z + Z_AVAIL_OUT, left);
            let avail = e.mem.u32(z + Z_AVAIL_IN) - 2;
            e.mem.set_u32(z + Z_AVAIL_IN, avail);
            0u32.into_ret()
        });
        let out = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00af_c0e0, &args![packed, out, 6u32]).u32(), 6);
        assert_eq!(e.mem.bytes(out, 6), b"aaaabb");
        assert_eq!(
            e.get(packed, CompressedArchiveFile::iUncompressedBufferPos),
            6
        );
        assert_eq!(
            e.get(packed, CompressedArchiveFile::iUncompressedBufferReadSize),
            8
        );
        assert_eq!(e.get(file, NiFile::m_uiPos), 4);
        assert_eq!(e.get(packed.cast::<BSFile>(), BSFile::iTrueFilePos), 4);
        // The rest of the buffer first, then a null buffer counts only.
        assert_eq!(e.call(0x00af_c660, &args![packed, out, 2u32]).u32(), 2);
        assert_eq!(e.mem.bytes(out, 2), b"bb");
        assert_eq!(e.call(0x00af_c0e0, &args![packed, 0u32, 4u32]).u32(), 4);
        assert_eq!(*calls.borrow(), 4);
        // An inflate error ends the stream; stream end stops the refill.
        e.register(INFLATE, |_, _| (-3i32).into_ret());
        e.register(INFLATE_END, |_, _| Ret::default());
        assert_eq!(e.call(0x00af_c0e0, &args![packed, out, 8u32]).u32(), 4);
        assert_eq!(calls_to(&e, INFLATE_END), vec![vec![stream]]);
        e.register(INFLATE, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00af_c0e0, &args![packed, out, 8u32]).u32(), 0);
    }

    #[test]
    fn shut_down_empties_the_list_arrays_and_pointers() {
        let (mut e, _) = engine();
        let (a, b) = (archive(&mut e), archive(&mut e));
        let tail = node(&mut e, b.addr(), 0);
        let head = node(&mut e, a.addr(), tail);
        e.set_global(ARCHIVE_LIST, head);
        let mut slots = vec![0u32; 1];
        slots[0] = 0x0070_0500;
        e.put_vtable(0x0300_0600, &slots);
        e.register(0x0070_0500, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        let blocks = e.mem.alloc(8);
        e.mem.set_u32(blocks, 0x5100);
        e.mem.set_u32(blocks + 4, 0x5200);
        e.mem.set_u32(array, 0x0300_0600);
        e.mem.set_u32(array + 4, blocks);
        e.mem.set_u32(array + 8, 2);
        e.set_global(BLOCK_ARRAY_011F816C, array);
        let held = archive(&mut e);
        e.set(held, Archive::m_uiRefCount, 1);
        e.set_global(ARCHIVE_POINTER_ROWS + 0x24 + 8, held.addr());
        e.call(0x00af_5820, &args![]);
        assert_eq!(e.global::<u32>(ARCHIVE_LIST), 0);
        assert_eq!((count(&e, a), count(&e, b)), (0, 0));
        let freed: Vec<u32> = calls_to(&e, MEMORY_MANAGER_DEALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(freed, vec![tail, head, 0x5100, 0x5200]);
        assert_eq!(calls_to(&e, 0x0070_0500), vec![vec![array, 1]]);
        assert_eq!(e.global::<u32>(ARCHIVE_POINTER_ROWS + 0x2c), 0);
        assert_eq!(count(&e, held), 0);
    }

    #[test]
    fn invalidation_file_fills_both_hash_arrays() {
        let (mut e, _) = engine();
        strings(&mut e);
        let file = e.mem.alloc(0x20);
        let mut slots = vec![0u32; GET_LINE_SLOT as usize / 4 + 1];
        slots[0] = 0x0070_0600;
        slots[OPEN_SLOT as usize / 4] = 0x0070_0610;
        slots[GET_LINE_SLOT as usize / 4] = 0x0070_0620;
        e.put_vtable(0x0300_0700, &slots);
        e.mem.set_u32(file, 0x0300_0700);
        e.mem.set_u32(file + 8, 0x0070_0630);
        e.register(0x0070_0600, |_, _| Ret::default());
        e.register(0x0070_0610, |_, _| 1u32.into_ret());
        let lines = Rc::new(RefCell::new(vec![&b"\\meshes\\x"[..], b"a.nif"]));
        e.register_double(0x0070_0620, move |e, w| {
            let mut lines = lines.borrow_mut();
            if lines.is_empty() {
                return 0u32.into_ret();
            }
            let line = lines.remove(0);
            e.mem.set_cstr(w[1], line);
            (line.len() as u32).into_ret()
        });
        e.register(0x0070_0630, |_, _| 1u32.into_ret());
        e.register_double(FILE_FINDER_GET_FILE, move |_, _| file.into_ret());
        e.register(HASH_ARRAY_CONSTRUCTOR, |_, w| w[0].into_ret());
        e.register(HASH_CONSTRUCTOR, |e, w| {
            let name = e.mem.cstr(w[1]);
            (u32::from(name[0]) << 8 | w[2]).into_ret()
        });
        e.register(ARRAY_ADD, |_, _| Ret::default());
        let path = text(&mut e, b"ArchiveInvalidation.txt");
        e.call(0x00af_5ab0, &args![path]);
        let directories = e.global::<u32>(BLOCK_ARRAY_011F816C);
        let files = e.global::<u32>(BLOCK_ARRAY_011F8168);
        let adds = calls_to(&e, ARRAY_ADD);
        assert_eq!(adds.len(), 2);
        assert_eq!(adds[0][0], directories);
        assert_eq!(adds[1][0], files);
        let hashes = calls_to(&e, HASH_CONSTRUCTOR);
        assert_eq!(hashes[0][2], 1);
        assert_eq!(hashes[1][2], 0);
        // The directory name skips its leading backslash.
        assert_eq!(hashes[0][1], hashes[1][1] + 1);
        assert_eq!(e.mem.u32(file + 4), 2);
        assert_eq!(calls_to(&e, 0x0070_0600), vec![vec![file, 1]]);
        assert_eq!(calls_to(&e, FILE_FINDER_GET_FILE)[0], vec![path, 0, 0x4000]);
        // A file that does not open is deleted at once.
        e.register(0x0070_0610, |_, _| 0u32.into_ret());
        e.call(0x00af_5ab0, &args![path]);
        assert_eq!(calls_to(&e, 0x0070_0600).len(), 2);
        assert_eq!(calls_to(&e, 0x0070_0620).len(), 3);
        e.call(0x00af_5ab0, &args![0u32]);
        assert_eq!(calls_to(&e, FILE_FINDER_GET_FILE).len(), 2);
    }

    /// A hash array object holding the given 8-byte values.
    fn hash_array(e: &mut Engine, values: &[u64]) -> u32 {
        let array = e.mem.alloc(0x10);
        let blocks = e.mem.alloc(4 * values.len() as u32 + 4);
        for (index, value) in values.iter().enumerate() {
            let at = hash(e, *value);
            e.mem.set_u32(blocks + 4 * index as u32, at);
        }
        e.mem.set_u32(array + 4, blocks);
        e.mem.set_u32(array + 8, values.len() as u32);
        array
    }

    #[test]
    fn invalidation_clears_listed_entries() {
        let (mut e, existing) = engine();
        strings(&mut e);
        assert_eq!(e.call(0x00af_ad00, &args![0u32]).u32(), 0);
        let a = searchable(&mut e);
        e.set(a, Archive::oFlags, 3);
        let directories = hash_array(&mut e, &[0x10]);
        let files = hash_array(&mut e, &[9]);
        e.set_global(BLOCK_ARRAY_011F816C, directories);
        e.set_global(BLOCK_ARRAY_011F8168, files);
        let name = text(&mut e, b"meshes");
        e.register_double(DIRECTORY_NAME, move |_, _| name.into_ret());
        existing.borrow_mut().push(b"Data\\meshes".to_vec());
        assert_eq!(e.call(0x00af_ad00, &args![a]).u32(), 2);
        let table = e.get(a, Archive::pDirectories);
        let first = e.mem.u32(table + 0xc);
        let second = e.mem.u32(table + 0x1c);
        assert_eq!(e.mem.u32(first + 0xc), 0);
        assert_eq!(e.mem.u32(second + 0xc), 0x200);
        assert_eq!(e.mem.u32(second + 0x1c), 0x8000_0000);
        // Without both name flags the time-based pass answers.
        e.set(a, Archive::oFlags, 1);
        e.register(INVALIDATION_PREPARE, |_, _| Ret::default());
        e.register(INVALIDATION_BY_TIME, |_, _| 7u32.into_ret());
        assert_eq!(e.call(0x00af_ad00, &args![a]).u32(), 7);
        let by_time = calls_to(&e, INVALIDATION_BY_TIME);
        assert_eq!(
            &by_time[0][..3],
            &[a.addr(), PATH_011F8050, TEXT_EMPTY_SECOND]
        );
    }

    #[test]
    fn name_tables_are_kept_by_flags_and_modes() {
        let (mut e, _) = engine();
        let a = archive(&mut e);
        assert!(!e.call(0x00af_d020, &args![a]).bool());
        e.set(a, Archive::oFlags, 0x80);
        assert!(e.call(0x00af_d020, &args![a]).bool());
        e.set(a, Archive::oFlags, 0x10);
        e.set_global(WORDS_SET_BY_00AF4470 + 4, 1u32);
        assert!(e.call(0x00af_d020, &args![a]).bool());
        // Type bit 4 needs the second or third word's answer.
        e.mem.set_u16(a.addr() + 0x180, 0x10);
        assert!(e.call(0x00af_d020, &args![a]).bool());
        e.set(a, Archive::oFlags, 8);
        e.set_global(WORDS_SET_BY_00AF4470, 1u32);
        e.set_global(WORDS_SET_BY_00AF4470 + 4, 0u32);
        assert!(!e.call(0x00af_d020, &args![a]).bool());
        e.set_global(FLAG_011ACDF9, 1u8);
        e.set_global(NAME_TABLE_MODE, 5u32);
        assert!(e.call(0x00af_d020, &args![a]).bool());
    }

    #[test]
    fn hashes_sort_ascending() {
        let (mut e, _) = engine();
        let values = [7u64, 0x1_0000_0000, 3, 9, 3, 1];
        let array = e.mem.alloc(4 * values.len() as u32);
        for (index, value) in values.iter().enumerate() {
            let at = hash(&mut e, *value);
            e.mem.set_u32(array + 4 * index as u32, at);
        }
        e.call(0x00af_6e50, &args![array, 0i32, 5i32]);
        let sorted: Vec<u64> = (0..6)
            .map(|i| e.mem.u64(e.mem.u32(array + 4 * i)))
            .collect();
        assert_eq!(sorted, vec![1, 3, 3, 7, 9, 0x1_0000_0000]);
    }

    #[test]
    fn matching_files_are_added_once() {
        let (mut e, _) = engine();
        let a = searchable(&mut e);
        e.set(a, Archive::cArchiveFlags, 0x20);
        let names = text(&mut e, b"a.nif\0b.nif\0");
        e.set(a, Archive::pFileNameStringArray, names);
        let rows = e.mem.alloc(8);
        let row = e.mem.alloc(8);
        e.mem.set_u32(row + 4, 6);
        e.mem.set_u32(rows + 4, row);
        e.set(a, Archive::pFileNameStringOffsets, rows);
        e.register(HASH_FLAG, |_, _| 0u32.into_ret());
        e.register(HASH_CONSTRUCTOR, |e, w| {
            e.mem.set_u64(w[0], u64::from(e.mem.u32(w[1])));
            w[0].into_ret()
        });
        e.register(LIST_ADD_NAME, |_, _| Ret::default());
        // The list already holds a name whose hash is 9.
        let existing = e.mem.alloc(4);
        e.mem.set_u32(existing, 9);
        let list = node(&mut e, 0, 0);
        e.mem.set_u32(list, existing);
        let directory = hash(&mut e, 0x20);
        // The pattern hash matches every file: word +4 equal, byte 2 equal.
        let pattern = hash(&mut e, 0);
        let prefix = text(&mut e, b"meshes\\");
        let made = e
            .call(
                0x00af_7060,
                &args![a, 1u32, 0u32, directory, pattern, list, prefix],
            )
            .u32();
        assert_ne!(made, 0);
        let added = calls_to(&e, LIST_ADD_NAME);
        assert_eq!(added.len(), 1);
        let block = e.mem.u32(added[0][1]);
        assert_eq!(e.mem.cstr(block), b"meshes\\a.nif");
        let section = a.addr() + 0x19c;
        assert_eq!(calls_to(&e, ENTER_CRITICAL_SECTION), vec![vec![section]]);
        assert_eq!(calls_to(&e, LEAVE_CRITICAL_SECTION), vec![vec![section]]);
        // The array is reused, and an unknown directory adds nothing.
        let again = e
            .call(
                0x00af_7060,
                &args![a, 1u32, made, directory, pattern, list, prefix],
            )
            .u32();
        assert_eq!(again, made);
        assert_eq!(calls_to(&e, HASH_CONSTRUCTOR).len(), 1);
        let nowhere = hash(&mut e, 0x30);
        e.call(
            0x00af_7060,
            &args![a, 1u32, made, nowhere, pattern, list, prefix],
        );
        assert_eq!(calls_to(&e, LIST_ADD_NAME).len(), 2);
    }

    #[test]
    fn add_to_file_list_asks_each_archive_and_frees_the_hashes() {
        let (mut e, _) = engine();
        manager(&mut e);
        let pattern = text(&mut e, b"*.nif");
        let path = text(&mut e, b"meshes\\*.nif");
        let list = node(&mut e, 0, 0);
        e.call(0x00af_6ba0, &args![list, pattern, path, 1u32]);
        assert!(calls_to(&e, TRIM_FILE_NAME).is_empty());
        let a = searchable(&mut e);
        e.mem.set_u16(a.addr() + 0x180, 1);
        e.set(a, Archive::cArchiveFlags, 0x20);
        let names = text(&mut e, b"a.nif\0b.nif\0");
        e.set(a, Archive::pFileNameStringArray, names);
        let rows = e.mem.alloc(8);
        let row = e.mem.alloc(8);
        e.mem.set_u32(rows + 4, row);
        e.set(a, Archive::pFileNameStringOffsets, rows);
        let head = node(&mut e, a.addr(), 0);
        e.set_global(ARCHIVE_LIST, head);
        e.register(HASH_FLAG, |_, _| 0u32.into_ret());
        e.register(HASH_CONSTRUCTOR, |_, w| w[0].into_ret());
        e.register(LIST_ADD_NAME, |_, _| Ret::default());
        e.register(HASH_PATH, |e, w| {
            e.mem.set_u64(w[1], 0x20);
            e.mem.set_u64(w[2], 0);
            Ret::default()
        });
        let item = text(&mut e, b"z");
        e.mem.set_u32(list, item);
        e.call(0x00af_6ba0, &args![list, pattern, path, 1u32]);
        let added = calls_to(&e, LIST_ADD_NAME);
        assert_eq!(added.len(), 1);
        let block = e.mem.u32(added[0][1]);
        assert_eq!(e.mem.cstr(block), b"meshes\\a.nif");
        // One hash made, then the hash and its array freed.
        assert_eq!(calls_to(&e, HASH_CONSTRUCTOR).len(), 1);
        let freed = calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len();
        assert!(freed >= 2);
        assert_eq!(count(&e, a), 1);
    }

    const ARCHIVE_DELETE: u32 = 0x0070_0700;
    const ARCHIVE_GET_SIZE: u32 = 0x0070_0710;
    const HEADER_READER: u32 = 0x0070_0720;

    /// Doubles for everything the `Archive` constructor calls outside this
    /// file. The `BSFile` constructor copies the name to +0x44, sets the
    /// read position word (+0x38) to -1 and installs `HEADER_READER` as
    /// `m_pfnRead`, which serves `data` in order; the open marks the file
    /// good when `good` is set.
    fn constructible(e: &mut Engine, data: Vec<u8>, good: bool) {
        e.register(BSFILE_CONSTRUCTOR, |e, w| {
            let name = e.mem.cstr(w[1]);
            e.mem.set_cstr(w[0] + 0x44, &name);
            e.mem.set_u32(w[0] + BSFILE_READ_POSITION, u32::MAX);
            e.mem.set_u32(w[0] + 8, HEADER_READER);
            w[0].into_ret()
        });
        e.register_double(BSFILE_OPEN, move |e, w| {
            e.mem.set_u8(w[0] + 0x2c, good as u8);
            1u32.into_ret()
        });
        e.register(BSFILE_SET_ENDIAN_SWAP, |_, _| Ret::default());
        e.register(BSFILE_SET_BUFFER_SIZE, |_, _| Ret::default());
        e.register(STAT64I32, |e, w| {
            e.mem.set_u64(w[1] + STAT_MTIME, 0x1234_5678_9abc);
            0u32.into_ret()
        });
        e.register(GET_TICK_COUNT, |_, _| 5u32.into_ret());
        let mut slots = vec![0u32; GET_SIZE_SLOT as usize / 4 + 1];
        slots[0] = ARCHIVE_DELETE;
        slots[GET_SIZE_SLOT as usize / 4] = ARCHIVE_GET_SIZE;
        e.put_vtable(ARCHIVE_VTABLE, &slots);
        e.register(ARCHIVE_DELETE, |_, _| Ret::default());
        e.register(ARCHIVE_GET_SIZE, |_, _| Ret::default());
        let data = Rc::new(RefCell::new(data));
        e.register_double(HEADER_READER, move |e, w| {
            let mut data = data.borrow_mut();
            let size = (w[2] as usize).min(data.len());
            let bytes: Vec<u8> = data.drain(..size).collect();
            e.mem.write(w[1], &bytes);
            (size as u32).into_ret()
        });
    }

    /// A BSA with one directory (hash 0x20, name `abc`) holding one file
    /// (hash 5, size 0x10, offset 0x40, name `a.nif`).
    fn bsa(version: u32, flags: u32) -> Vec<u8> {
        let mut out = vec![];
        for word in [0x0041_5342u32, version, 0x24, flags, 1, 1, 4, 6] {
            out.extend(word.to_le_bytes());
        }
        out.extend([1u8, 0, 0, 0]);
        out.extend(0x20u64.to_le_bytes());
        out.extend([1u32, 0].iter().flat_map(|w| w.to_le_bytes()));
        if flags & 1 != 0 {
            out.push(4);
            out.extend(b"abc\0");
        }
        out.extend(5u64.to_le_bytes());
        out.extend([0x10u32, 0x40].iter().flat_map(|w| w.to_le_bytes()));
        if flags & 2 != 0 {
            out.extend(b"a.nif\0");
        }
        out
    }

    #[test]
    fn archive_constructor_reads_the_tables() {
        let (mut e, _) = engine();
        constructible(&mut e, bsa(0x68, 3), true);
        e.set_global(FLAG_011ACDF9, 1u8);
        e.set_global(NEXT_RUNTIME_ID, 7u32);
        let block = e.mem.alloc(0x1d0);
        let name = text(&mut e, b"Data\\Fallout - Meshes.bsa");
        let a = e
            .call(0x00af_7eb0, &args![block, name, 0x40000u32, 0u8, 0u8])
            .u32();
        assert_eq!(a, block);
        let a = Ptr::<Archive>::new(a);
        assert_eq!(e.mem.u32(block), ARCHIVE_VTABLE);
        assert_eq!(
            e.get(a, Archive::ref_object_vtable),
            ARCHIVE_REF_OBJECT_VTABLE
        );
        assert_eq!(e.get(a, Archive::m_iRuntimeID), 7);
        assert_eq!(e.global::<u32>(NEXT_RUNTIME_ID), 8);
        assert_eq!(e.get(a, Archive::cArchiveFlags), 0x30);
        let directories = e.get(a, Archive::pDirectories);
        assert_eq!(e.mem.u64(directories), 0x20);
        let files = e.mem.u32(directories + 0xc);
        assert_eq!(e.mem.u64(files), 5);
        assert_eq!(e.mem.u32(files + 0xc), 0x40);
        let names = e.get(a, Archive::pDirectoryStringArray);
        assert_eq!(e.mem.cstr(names), b"abc");
        let names = e.get(a, Archive::pFileNameStringArray);
        assert_eq!(e.mem.cstr(names), b"a.nif");
        let rows = e.get(a, Archive::pFileNameStringOffsets);
        assert_eq!(e.mem.u32(e.mem.u32(rows)), 0);
        assert_eq!(e.mem.u64(block + 0x188), 0x1234_5678_9abc);
        assert_eq!(
            calls_to(&e, BSFILE_CONSTRUCTOR),
            vec![vec![block, name, 0, 0x40000, 0]]
        );
        assert_eq!(
            calls_to(&e, BSFILE_SET_BUFFER_SIZE),
            vec![vec![block, 0x4000]]
        );
        assert_eq!(calls_to(&e, ARCHIVE_GET_SIZE).len(), 1);
        assert_eq!(
            calls_to(&e, INITIALIZE_CRITICAL_SECTION),
            vec![vec![block + 0x19c]]
        );
    }

    #[test]
    fn archive_constructor_rejects_bad_headers_and_honours_its_flags() {
        let (mut e, _) = engine();
        let mut bad = bsa(0x68, 0);
        bad[0] = b'X';
        constructible(&mut e, bad, true);
        let block = e.mem.alloc(0x1d0);
        let name = text(&mut e, b"x.bsa");
        e.call(0x00af_7eb0, &args![block, name, 0x40000u32, 0u8, 0u8]);
        assert_eq!(e.mem.u8(block + 0x1b4), 1);
        assert!(calls_to(&e, STAT64I32).is_empty());
        // A file that does not open stops after the open.
        let (mut e, _) = engine();
        constructible(&mut e, bsa(0x68, 0), false);
        let block = e.mem.alloc(0x1d0);
        let name = text(&mut e, b"x.bsa");
        e.call(0x00af_7eb0, &args![block, name, 0x40000u32, 0u8, 0u8]);
        assert!(calls_to(&e, BSFILE_SET_ENDIAN_SWAP).is_empty());
        // Version 0x67 clears oFlags bit 8; headers only skips the tables.
        let (mut e, _) = engine();
        constructible(&mut e, bsa(0x67, 0x103), true);
        let block = e.mem.alloc(0x1d0);
        let name = text(&mut e, b"x.bsa");
        e.call(0x00af_7eb0, &args![block, name, 0x40000u32, 1u8, 0u8]);
        let a = Ptr::<Archive>::new(block);
        assert_eq!(e.get(a, Archive::oFlags), 3);
        assert_eq!(e.get(a, Archive::cArchiveFlags), 8);
        assert_eq!(e.get(a, Archive::pDirectories), 0);
        assert_eq!(calls_to(&e, STAT64I32).len(), 1);
        assert_eq!(calls_to(&e, BSFILE_SET_BUFFER_SIZE).len(), 1);
        // Per-file work runs for every file when asked; names not kept.
        let (mut e, _) = engine();
        constructible(&mut e, bsa(0x68, 0), true);
        e.register(PER_FILE, |_, _| Ret::default());
        let block = e.mem.alloc(0x1d0);
        let name = text(&mut e, b"x.bsa");
        e.call(0x00af_7eb0, &args![block, name, 0x40000u32, 0u8, 1u8]);
        let per_file = calls_to(&e, PER_FILE);
        assert_eq!(per_file.len(), 1);
        assert_eq!(per_file[0][2], 1);
        let a = Ptr::<Archive>::new(block);
        assert_eq!(e.get(a, Archive::cArchiveFlags), 0);
    }

    /// Doubles for the open path: the finder finds every name except
    /// `missing.bsa` and copies it as the found path; nothing is redirected;
    /// the archives' bytes are `data`, read in order.
    fn openable(e: &mut Engine, data: Vec<u8>) {
        constructible(e, data, true);
        let finder = e.mem.alloc(0x10);
        e.set_global(FILE_FINDER, finder);
        e.register(FILE_FINDER_LOOKUP, |e, w| {
            let name = e.mem.cstr(w[1]);
            if name == b"missing.bsa" {
                return 0u32.into_ret();
            }
            e.mem.set_cstr(w[2], &name);
            1u32.into_ret()
        });
        let owner = e.mem.alloc(0x60);
        e.register_double(STATE_OWNER, move |_, _| owner.into_ret());
        e.register(REDIRECTS_PATH, |_, _| 0u32.into_ret());
        e.register(GAMEBRYO_ALLOCATE, |e, w| e.mem.alloc(w[0]).into_ret());
        e.register(WARNING, |_, _| Ret::default());
        for (index, value) in [
            &b"Fallo"[..],
            b"DeadM",
            b"Hones",
            b"Lones",
            b"OldWo",
            b"Updat",
        ]
        .iter()
        .enumerate()
        {
            e.mem.set_cstr(RANK_NAMES[index], value);
            e.mem.set_cstr(LISTED_RANK_NAMES[index], value);
        }
        e.set_global(FLAG_011ACDF8, 1u8);
    }

    fn listed(e: &Engine) -> Vec<Vec<u8>> {
        let mut out = vec![];
        let mut node = e.global::<u32>(ARCHIVE_LIST);
        while node != 0 && e.mem.u32(node) != 0 {
            out.push(e.mem.cstr(e.mem.u32(node) + 0x44));
            node = e.mem.u32(node + 4);
        }
        out
    }

    #[test]
    fn open_archive_ranks_the_list() {
        let (mut e, _) = engine();
        let name = text(&mut e, b"Fallout - Meshes.bsa");
        assert_eq!(e.call(0x00af_4be0, &args![name, 0u16, 0u8]).u32(), 0);
        openable(&mut e, [bsa(0x68, 0), bsa(0x68, 0), bsa(0x68, 0)].concat());
        let first = e.call(0x00af_4be0, &args![name, 0u16, 0u8]).u32();
        assert_ne!(first, 0);
        assert_eq!(listed(&e), vec![b"Fallout - Meshes.bsa".to_vec()]);
        let mine = text(&mut e, b"MyMod.bsa");
        let second = e.call(0x00af_4be0, &args![mine, 4u16, 0u8]).u32();
        assert_eq!(e.mem.u16(second + 0x180), 4);
        assert_eq!(calls_to(&e, WARNING).len(), 1);
        assert_eq!(
            listed(&e),
            vec![b"MyMod.bsa".to_vec(), b"Fallout - Meshes.bsa".to_vec()]
        );
        let update = text(&mut e, b"Update.bsa");
        e.call(0x00af_4be0, &args![update, 0u16, 0u8]);
        assert_eq!(listed(&e)[1], b"Update.bsa".to_vec());
        let ptr = Ptr::<Archive>::new(first);
        assert_eq!(count(&e, ptr), 1);
        let missing = text(&mut e, b"missing.bsa");
        assert_eq!(e.call(0x00af_4be0, &args![missing, 0u16, 0u8]).u32(), 0);
    }

    #[test]
    fn open_archive_deletes_a_bad_archive() {
        let (mut e, _) = engine();
        let mut bad = bsa(0x68, 0);
        bad[0] = b'X';
        openable(&mut e, bad);
        let name = text(&mut e, b"Fallout - Sound.bsa");
        assert_eq!(e.call(0x00af_4be0, &args![name, 0u16, 0u8]).u32(), 0);
        assert_eq!(calls_to(&e, ARCHIVE_DELETE).len(), 1);
        assert_eq!(e.global::<u32>(ARCHIVE_LIST), 0);
    }

    #[test]
    fn master_archives_open_from_the_list() {
        let (mut e, _) = engine();
        assert!(e.call(0x00af_4550, &args![]).bool());
        assert!(calls_to(&e, GET_TICK_COUNT).is_empty());
        openable(&mut e, [bsa(0x68, 0), bsa(0x68, 0)].concat());
        // The extension table is sorted by name.
        for index in 0..0x1au32 {
            let at = ARCHIVE_TYPE_TABLE + index * 8;
            e.mem.set_cstr(at, &[b'z' - index as u8]);
            e.mem.set_u32(at + 4, index);
        }
        for (at, value) in [(TEXT_COMMA_FIRST, &b","[..]), (TEXT_COMMA_NEXT, b",")] {
            e.mem.set_cstr(at, value);
        }
        e.register(PRINT_TO_BUFFER, |_, _| Ret::default());
        e.register(FOPEN_S, |e, w| {
            e.mem.set_u32(w[0], 0);
            0u32.into_ret()
        });
        e.register(STRTOK_S, |e, w| {
            let start = if w[0] != 0 { w[0] } else { e.mem.u32(w[2]) };
            let mut at = start;
            while e.mem.u8(at) != 0 && e.mem.u8(at) != b',' {
                at += 1;
            }
            if e.mem.u8(at) == b',' {
                e.mem.set_u8(at, 0);
                at += 1;
            }
            e.mem.set_u32(w[2], at);
            start.into_ret()
        });
        let list = text(&mut e, b"Fallout - Meshes.bsa, missing.bsa");
        e.set_global(STRING_011F815C, list);
        e.set_global(TYPE_BUFFER_SIZES, 0x8000u32);
        assert!(!e.call(0x00af_4550, &args![]).bool());
        assert_eq!(e.mem.cstr(ARCHIVE_TYPE_TABLE), b"a");
        assert_eq!(e.mem.u32(ARCHIVE_TYPE_TABLE + 4), 0x19);
        assert_eq!(e.mem.cstr(ARCHIVE_TYPE_TABLE + 0x19 * 8), b"z");
        let first = e.global::<u32>(ARCHIVE_POINTER_ROWS);
        assert_ne!(first, 0);
        assert_eq!(e.mem.cstr(first + 0x44), b"Fallout - Meshes.bsa");
        assert_eq!(
            calls_to(&e, BSFILE_SET_BUFFER_SIZE)
                .iter()
                .filter(|c| c[1] == 0x8000)
                .count(),
            1
        );
        let opens = calls_to(&e, FOPEN_S);
        assert_eq!(opens[0][2], TEXT_READ_MODE);
    }

    #[test]
    fn per_file_runs_on_each_archive_of_the_type_under_the_lock() {
        let (mut e, _) = engine();
        e.register(ARCHIVE_PER_FILE, |_, _| Ret::default());
        let (first, second) = (archive(&mut e), archive(&mut e));
        e.mem.set_u16(first.addr() + 0x180, 2);
        e.mem.set_u16(second.addr() + 0x180, 1);
        // A node with neither an archive nor a next pointer ends the walk.
        let empty = node(&mut e, 0, 0);
        let tail = node(&mut e, second.addr(), empty);
        let head = node(&mut e, first.addr(), tail);
        e.set_global(ARCHIVE_LIST, head);
        e.call_log = Some(vec![]);
        e.call(0x00af_7c40, &args![0x11u32, 0x22u32, 3u32]);
        assert_eq!(
            calls_to(&e, ARCHIVE_PER_FILE),
            vec![
                vec![first.addr(), 0x11, 0x22, 0],
                vec![second.addr(), 0x11, 0x22, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![ARCHIVE_LIST_LOCK]]
        );
        assert_eq!(
            calls_to(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![ARCHIVE_LIST_LOCK]]
        );
        // Each reference taken is given back; the counts are as before.
        assert_eq!((count(&e, first), count(&e, second)), (1, 1));
        assert_eq!(calls_to(&e, INTERLOCKED_INCREMENT).len(), 2);
        e.call_log = Some(vec![]);
        e.call(0x00af_7c40, &args![0x11u32, 0x22u32, 2u32]);
        assert_eq!(
            calls_to(&e, ARCHIVE_PER_FILE),
            vec![vec![first.addr(), 0x11, 0x22, 0]]
        );
        e.set_global(ARCHIVE_LIST, empty);
        e.call_log = Some(vec![]);
        e.call(0x00af_7c40, &args![0x11u32, 0x22u32, 3u32]);
        assert!(calls_to(&e, ARCHIVE_PER_FILE).is_empty());
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
    }

    // Batch n083.

    #[test]
    fn n083_constant_answers() {
        let (mut e, _) = engine();
        assert!(!e.call(0x00af_4390, &args![]).bool());
        for addr in [0x00af_bf50u32, 0x00af_c3e0, 0x00af_c3f0, 0x00af_c400] {
            assert_eq!(e.call(addr, &args![0x1000u32, 1u32, 2u32]).u32(), 0);
        }
        let file = archive_file(&mut e, 0);
        for addr in [0x00af_bf60u32, 0x00af_c410] {
            assert!(!e.call(addr, &args![file]).bool());
            e.set(file, ArchiveFile::spArchive, 0x1234);
            assert!(e.call(addr, &args![file]).bool());
            e.set(file, ArchiveFile::spArchive, 0);
        }
    }

    #[test]
    fn n083_string_copies_are_freed() {
        let (mut e, _) = engine();
        e.call(0x00af_44f0, &args![]);
        assert!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        e.set_global(STRING_011F8158, 0x5000u32);
        e.set_global(STRING_011F815C, 0x6000u32);
        e.call(0x00af_44f0, &args![]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x5000], vec![MEMORY_MANAGER, 0x6000]]
        );
        assert_eq!(e.global::<u32>(STRING_011F8158), 0x5000);
    }

    #[test]
    fn n083_archive_is_added_at_the_end_under_the_lock() {
        let (mut e, _) = engine();
        let first = archive(&mut e);
        let added = archive(&mut e);
        // No list: only the lock.
        e.call(0x00af_5480, &args![added]);
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![ARCHIVE_LIST_LOCK]]
        );
        assert_eq!(
            calls_to(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![ARCHIVE_LIST_LOCK]]
        );
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
        let head = node(&mut e, first.addr(), 0);
        e.set_global(ARCHIVE_LIST, head);
        e.call(0x00af_5480, &args![added]);
        let second = e.mem.u32(head + 4);
        assert_eq!(e.mem.u32(second), added.addr());
        // The list keeps one reference; the temporary's is given back.
        assert_eq!(count(&e, added), 1);
    }

    #[test]
    fn n083_archive_is_taken_out_of_the_list_and_the_rows() {
        let (mut e, _) = engine();
        let first = archive(&mut e);
        let gone = archive(&mut e);
        // No list or no archive: false.
        assert!(!e.call(0x00af_5550, &args![gone]).bool());
        assert!(!e.call(0x00af_56d0, &args![gone]).bool());
        let tail = node(&mut e, gone.addr(), 0);
        let head = node(&mut e, first.addr(), tail);
        e.set_global(ARCHIVE_LIST, head);
        assert!(!e.call(0x00af_5550, &args![0u32]).bool());
        for slot in [
            archive_pointer(0, 2),
            archive_pointer(1, 8),
            ARCHIVE_POINTER_011F8188,
        ] {
            e.mem.set_u32(slot, gone.addr());
            add_ref(&mut e, gone.addr());
        }
        e.mem.set_u32(archive_pointer(0, 3), first.addr());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00af_56d0, &args![gone]).bool());
        assert_eq!(e.mem.u32(head + 4), 0);
        for slot in [
            archive_pointer(0, 2),
            archive_pointer(1, 8),
            ARCHIVE_POINTER_011F8188,
        ] {
            assert_eq!(e.mem.u32(slot), 0);
        }
        assert_eq!(e.mem.u32(archive_pointer(0, 3)), first.addr());
        assert_eq!(count(&e, gone), 0);
        assert_eq!(calls_to(&e, DELETE_THIS).len(), 1);
        assert_eq!(calls_to(&e, ENTER_CRITICAL_SECTION).len(), 1);
        assert_eq!(calls_to(&e, LEAVE_CRITICAL_SECTION).len(), 1);
    }

    #[test]
    fn n083_archive_is_taken_out_by_name() {
        let (mut e, _) = engine();
        let a = archive(&mut e);
        let name = text(&mut e, b"Fallout - Misc.bsa");
        e.put_vtable(0x0300_0400, &[0, 0, 0, 0, 0, 0, 0x0070_0400]);
        e.mem.set_u32(a.addr(), 0x0300_0400);
        e.register_double(0x0070_0400, move |_, _| name.into_ret());
        let head = node(&mut e, a.addr(), 0);
        let wanted = text(&mut e, b"fallout - misc.bsa");
        // No list: false.
        assert!(!e.call(0x00af_5700, &args![wanted]).bool());
        e.set_global(ARCHIVE_LIST, head);
        // No finder: false.
        assert!(!e.call(0x00af_5700, &args![wanted]).bool());
        e.set_global(FILE_FINDER, 0x0300_0500u32);
        e.register(FILE_FINDER_LOOKUP, |_, w| u32::from(w[3] == 1).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00af_5700, &args![wanted]).bool());
        let lookups = calls_to(&e, FILE_FINDER_LOOKUP);
        assert_eq!(lookups[0][..2], [0x0300_0500, wanted]);
        assert_eq!(lookups[0][3..], [1, 0xffff_ffff]);
        assert_eq!(e.mem.u32(head), 0);
        // A finder that finds nothing: false.
        e.register(FILE_FINDER_LOOKUP, |_, _| Ret::default());
        assert!(!e.call(0x00af_5700, &args![wanted]).bool());
    }

    #[test]
    fn n083_invalidate_file_clears_the_entry_offset() {
        let (mut e, _) = engine();
        manager(&mut e);
        let a = searchable(&mut e);
        let path = text(&mut e, b"meshes\\a.nif");
        assert!(!e.call(0x00af_5e40, &args![path]).bool());
        let head = node(&mut e, a.addr(), 0);
        e.set_global(ARCHIVE_LIST, head);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00af_5e40, &args![path]).bool());
        let files = e.mem.u32(e.get(a, Archive::pDirectories) + 16 + 0xc);
        assert_eq!(e.mem.u32(files + 0xc), 0);
        assert_eq!(count(&e, a), 1);
        // A file no archive has: false, the references balanced.
        e.register(HASH_PATH, |e, w| {
            e.mem.set_u64(w[1], 0x20);
            e.mem.set_u64(w[2], 7);
            Ret::default()
        });
        assert!(!e.call(0x00af_5e40, &args![path]).bool());
        assert_eq!(count(&e, a), 1);
        // The top bit of the offset is kept (it matches bit 3 of the archive's flags).
        e.set(a, Archive::cArchiveFlags, 8);
        e.register(HASH_PATH, |e, w| {
            e.mem.set_u64(w[1], 0x20);
            e.mem.set_u64(w[2], 9);
            Ret::default()
        });
        assert!(e.call(0x00af_5e40, &args![path]).bool());
        assert_eq!(e.mem.u32(files + 16 + 0xc), 0x8000_0000);
    }

    #[test]
    fn n083_entry_lookups_by_slot_and_type() {
        let (mut e, _) = engine();
        let a = searchable(&mut e);
        e.set(a, Archive::iArchiveType, 2);
        let directory = hash(&mut e, 0x20);
        let file = hash(&mut e, 9);
        let files = e.mem.u32(e.get(a, Archive::pDirectories) + 16 + 0xc);
        // Slot 4: neither row set, then the second row, then the first.
        assert_eq!(
            e.call(0x00af_6430, &args![4u32, directory, file, 0u32])
                .u32(),
            0
        );
        e.mem.set_u32(archive_pointer(1, 4), a.addr());
        let entry = e
            .call(0x00af_6430, &args![4u32, directory, file, 0u32])
            .u32();
        assert_eq!(entry, files + 16);
        e.mem.set_u32(archive_pointer(0, 4), a.addr());
        let entry = e
            .call(0x00af_6430, &args![4u32, directory, file, 0u32])
            .u32();
        assert_eq!(entry, files + 16);
        assert_eq!(count(&e, a), 0);
        // The list, by type.
        let head = node(&mut e, a.addr(), 0);
        e.set_global(ARCHIVE_LIST, head);
        assert_eq!(
            e.call(0x00af_6690, &args![1u32, directory, file, 0u32])
                .u32(),
            0
        );
        let entry = e
            .call(0x00af_6690, &args![0x1_0002u32, directory, file, 0u32])
            .u32();
        assert_eq!(entry, files + 16);
        let missing = hash(&mut e, 7);
        assert_eq!(
            e.call(0x00af_6690, &args![2u32, directory, missing, 0u32])
                .u32(),
            0
        );
        assert_eq!(count(&e, a), 1);
    }

    /// Gives `a` kept file names: `names` for directory 1's files, from `table`.
    fn kept_names(e: &mut Engine, a: Ptr<Archive>, table: &[u8], offsets: &[u32]) {
        e.set(a, Archive::cArchiveFlags, 0x20);
        let block = e.mem.alloc(table.len() as u32 + 4);
        e.mem.write(block, table);
        e.set(a, Archive::pFileNameStringArray, block);
        let rows = e.mem.alloc(8);
        let row = e.mem.alloc(4 * offsets.len() as u32 + 4);
        for (i, offset) in offsets.iter().enumerate() {
            e.mem.set_u32(row + 4 * i as u32, *offset);
        }
        e.mem.set_u32(rows + 4, row);
        e.set(a, Archive::pFileNameStringOffsets, rows);
    }

    #[test]
    fn n083_entry_names_are_copied_under_the_lock() {
        let (mut e, _) = engine();
        let a = searchable(&mut e);
        kept_names(&mut e, a, b"a.nif\0b.nif\0", &[0, 6]);
        let files = e.mem.u32(e.get(a, Archive::pDirectories) + 16 + 0xc);
        let out = e.mem.alloc(0x104);
        assert!(!e.call(0x00af_67d0, &args![1u32, files, out]).bool());
        // Entry 1 has bit 31 of its word +0xc: the second row.
        e.mem.set_u32(archive_pointer(1, 1), a.addr());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00af_67d0, &args![1u32, files + 16, out]).bool());
        assert_eq!(e.mem.cstr(out), b"b.nif");
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![a.addr() + 0x19c]]
        );
        assert_eq!(
            calls_to(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![a.addr() + 0x19c]]
        );
        // An entry outside the archive: no name, still locked and left.
        let stray = e.mem.alloc(16);
        e.mem.set_u32(stray + 0xc, 0x8000_0000);
        assert!(!e.call(0x00af_67d0, &args![1u32, stray, out]).bool());
        assert_eq!(count(&e, a), 0);
    }

    #[test]
    fn n083_directory_files_are_picked_into_the_path() {
        let (mut e, _) = engine();
        manager(&mut e);
        let a = searchable(&mut e);
        e.set(a, Archive::iArchiveType, 1);
        kept_names(&mut e, a, b"a.nif\0b.nif\0", &[0, 6]);
        e.register(HASH_CONSTRUCTOR, |e, w| {
            e.mem.set_u64(w[0], 0x20);
            Ret::default()
        });
        e.register(BS_RANDOM_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BS_RANDOM_UNSIGNED_INT, |_, w| (w[1] - 1).into_ret());
        e.map(RANDOM_GENERATOR_GUARD, 4);
        let path = text(&mut e, b"meshes\\dir\\");
        let out = e.mem.alloc(0x104);
        // No list: false.
        assert!(!e.call(0x00af_7400, &args![path, out, 1u32]).bool());
        let head = node(&mut e, a.addr(), 0);
        e.set_global(ARCHIVE_LIST, head);
        // A type the archive lacks: false.
        assert!(!e.call(0x00af_7400, &args![path, out, 2u32]).bool());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00af_7400, &args![path, out, 0xffffu32]).bool());
        assert_eq!(e.mem.cstr(out), b"meshes\\dir\\b.nif");
        assert_eq!(
            calls_to(&e, BS_RANDOM_CONSTRUCTOR),
            vec![vec![RANDOM_GENERATOR]]
        );
        assert_eq!(
            calls_to(&e, BS_RANDOM_UNSIGNED_INT),
            vec![vec![RANDOM_GENERATOR, 2]]
        );
        // Bit 5 of the flags (names kept): no lock.
        assert!(calls_to(&e, ENTER_CRITICAL_SECTION).is_empty());
        assert_eq!(e.global::<u32>(RANDOM_GENERATOR_GUARD), 1);
        assert_eq!(count(&e, a), 1);
        // The generator is made once.
        e.call_log = Some(vec![]);
        e.call(0x00af_7400, &args![path, out, 1u32]);
        assert!(calls_to(&e, BS_RANDOM_CONSTRUCTOR).is_empty());
        // The smallest size word (low 30 bits) wins: 0x4000_0030 is 0x30 > 0x20.
        e.call_log = Some(vec![]);
        assert!(e.call(0x00af_7800, &args![path, out, 1u32]).bool());
        assert_eq!(e.mem.cstr(out), b"meshes\\dir\\a.nif");
        assert!(calls_to(&e, BS_RANDOM_UNSIGNED_INT).is_empty());
        let files = e.mem.u32(e.get(a, Archive::pDirectories) + 16 + 0xc);
        e.mem.set_u32(files + 8, 0x40);
        assert!(e.call(0x00af_7800, &args![path, out, 1u32]).bool());
        assert_eq!(e.mem.cstr(out), b"meshes\\dir\\b.nif");
        // A path without a `\`: false.
        let bare = text(&mut e, b"dir");
        assert!(!e.call(0x00af_7800, &args![bare, out, 1u32]).bool());
        assert_eq!(count(&e, a), 1);
        // Names not kept (and none in the archive): locked around the pick, the name null.
        e.set(a, Archive::cArchiveFlags, 0);
        e.register(STRCPY_S, |e, a| {
            if a[2] != 0 {
                let text = e.mem.cstr(a[2]);
                e.mem.set_cstr(a[0], &text);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x00af_7800, &args![path, out, 1u32]).bool());
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![a.addr() + 0x19c]]
        );
        assert_eq!(
            calls_to(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![a.addr() + 0x19c]]
        );
        let copies = calls_to(&e, STRCPY_S);
        assert_eq!(copies[1][1..], [0x104 - 10 - 1, 0]);
        // Without a `\` the lock is entered and never left.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00af_7800, &args![bare, out, 1u32]).bool());
        assert!(calls_to(&e, LEAVE_CRITICAL_SECTION).is_empty());
    }

    #[test]
    fn n083_name_hash_equality() {
        let (mut e, _) = engine();
        e.register(HASH_CONSTRUCTOR, |e, w| {
            e.mem.set_u64(w[0], 0x0000_0005_0000_0061);
            Ret::default()
        });
        e.register(HASH_FLAG, |_, _| Ret::default());
        let same = hash(&mut e, 0x0000_0005_0000_0061);
        let other = hash(&mut e, 0x0000_0006_0000_0061);
        let name = text(&mut e, b"a");
        assert!(e.call(0x00af_7e80, &args![name, same]).bool());
        assert!(!e.call(0x00af_7e80, &args![name, other]).bool());
        assert_eq!(calls_to(&e, HASH_CONSTRUCTOR)[0][1..], [name, 0]);
    }

    #[test]
    fn n083_directory_names_come_from_the_table_or_the_file() {
        let (mut e, _) = engine();
        // Two directory names in the file after the 0x24-byte header and the two
        // 16-byte directory entries: a length byte, the name, the file entries.
        const TABLE: &[u8] = b"\x04ab\\\x000123456789abcdef\x03cd\0";
        let a = archive(&mut e);
        let mut slots = vec![0u32; 6];
        slots[SEEK_SLOT as usize / 4] = 0x0070_0714;
        e.put_vtable(0x0300_0700, &slots);
        e.mem.set_u32(a.addr(), 0x0300_0700);
        e.set_global(NI_FILE_SEEK_SET, 0u32);
        e.set_global(NI_FILE_SEEK_CURRENT, 1u32);
        let position = Rc::new(RefCell::new(0u32));
        let at = position.clone();
        e.register_double(0x0070_0714, move |_, w| {
            let mut p = at.borrow_mut();
            *p = if w[2] == 0 {
                w[1]
            } else {
                p.wrapping_add(w[1])
            };
            Ret::default()
        });
        e.mem.set_u32(a.addr() + 8, 0x0070_0720);
        e.register_double(0x0070_0720, move |e, w| {
            let from = (*position.borrow() - 0x44) as usize;
            e.mem.write(w[1], &TABLE[from..from + w[2] as usize]);
            *position.borrow_mut() += w[2];
            w[2].into_ret()
        });
        directories(&mut e, a, &[(1, &[(0, 0, 0)]), (2, &[])]);
        assert_eq!(e.call(0x00af_94c0, &args![a, 1u32]).u32(), 0);
        e.set(a, Archive::oFlags, 1);
        let name = e.call(0x00af_94c0, &args![a, 1u32]).u32();
        assert_eq!(e.mem.cstr(name), b"cd");
        assert_eq!(e.get(a, Archive::pDirectoryStringArray), name);
        // The block before is given back.
        e.call_log = Some(vec![]);
        let again = e.call(0x00af_94c0, &args![a, 0u32]).u32();
        assert_eq!(e.mem.cstr(again), b"ab\\");
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, name]]
        );
        // Kept names: the table plus the offset.
        e.set(a, Archive::cArchiveFlags, 0x10);
        let offsets = e.mem.alloc(8);
        e.mem.set_u32(offsets + 4, 7);
        e.set(a, Archive::pDirectoryStringOffsets, offsets);
        e.set(a, Archive::pDirectoryStringArray, 0x9000);
        assert_eq!(e.call(0x00af_94c0, &args![a, 1u32]).u32(), 0x9007);
    }

    #[test]
    fn n083_entry_directory_and_index() {
        let (mut e, _) = engine();
        let a = searchable(&mut e);
        kept_names(&mut e, a, b"a.nif\0b.nif\0", &[0, 6]);
        let directories = e.get(a, Archive::pDirectories);
        let first = e.mem.u32(directories + 0xc);
        let second = e.mem.u32(directories + 16 + 0xc);
        assert_eq!(e.call(0x00af_9ad0, &args![a, first]).u32(), 0);
        assert_eq!(e.call(0x00af_9ad0, &args![a, second + 16]).u32(), 1);
        assert_eq!(e.call(0x00af_9ad0, &args![a, second + 32]).u32(), u32::MAX);
        assert_eq!(e.call(0x00af_9b60, &args![a, 1u32, second + 16]).u32(), 1);
        let name = e.call(0x00af_9ba0, &args![a, second + 16]).u32();
        assert_eq!(e.mem.cstr(name), b"b.nif");
        assert_eq!(e.call(0x00af_9ba0, &args![a, 0x10u32]).u32(), 0);
    }

    #[test]
    fn n083_collision_check_answers_the_differing_name() {
        let (mut e, _) = engine();
        let a = searchable(&mut e);
        kept_names(&mut e, a, b"a.nif\0b.nif\0", &[0, 6]);
        // Bit 3 matches the file's offset bit 31; bits 4 and 5 keep the names.
        e.set(a, Archive::cArchiveFlags, 0x38);
        let names = text(&mut e, b"meshes");
        let offsets = e.mem.alloc(8);
        e.set(a, Archive::pDirectoryStringArray, names);
        e.set(a, Archive::pDirectoryStringOffsets, offsets);
        e.register(SPLITPATH_S, |e, w| {
            let path = e.mem.cstr(w[0]);
            let cut = path.iter().rposition(|&c| c == b'\\').unwrap();
            e.mem.set_cstr(w[3], &path[..cut]);
            e.mem.set_cstr(w[5], &path[cut + 1..]);
            Ret::default()
        });
        e.register(STRUPR_S, |e, w| {
            let upper: Vec<u8> = e.mem.cstr(w[0]).to_ascii_uppercase();
            e.mem.set_cstr(w[0], &upper);
            Ret::default()
        });
        let directory = hash(&mut e, 0x20);
        let file = hash(&mut e, 9);
        let path = text(&mut e, b"meshes\\B.nif");
        // The flag byte off, or oFlags lacking a bit: 0 and nothing asked.
        assert_eq!(
            e.call(0x00af_a8e0, &args![a, directory, file, path]).u32(),
            0
        );
        e.set_global(FLAG_SET_BY_00AF4460, 1u8);
        e.set(a, Archive::oFlags, 1);
        assert_eq!(
            e.call(0x00af_a8e0, &args![a, directory, file, path]).u32(),
            0
        );
        assert!(calls_to(&e, SPLITPATH_S).is_empty());
        e.set(a, Archive::oFlags, 3);
        // Same names: 0, the copies kept.
        assert_eq!(
            e.call(0x00af_a8e0, &args![a, directory, file, path]).u32(),
            0
        );
        assert!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        // A differing file name: the archive's file name.
        let other = text(&mut e, b"meshes\\c.nif");
        let r = e.call(0x00af_a8e0, &args![a, directory, file, other]).u32();
        assert_eq!(e.mem.cstr(r), b"b.nif");
        assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), 2);
        // A differing directory: the archive's directory name.
        let elsewhere = text(&mut e, b"textures\\b.nif");
        let r = e
            .call(0x00af_a8e0, &args![a, directory, file, elsewhere])
            .u32();
        assert_eq!(r, names);
        // Hashes not found: 0.
        let missing = hash(&mut e, 7);
        assert_eq!(
            e.call(0x00af_a8e0, &args![a, directory, missing, other])
                .u32(),
            0
        );
    }

    #[test]
    fn n083_directory_file_offset_is_cleared() {
        let (mut e, _) = engine();
        let a = searchable(&mut e);
        let files = e.mem.u32(e.get(a, Archive::pDirectories) + 16 + 0xc);
        e.set(a, Archive::cArchiveFlags, 8);
        let file = hash(&mut e, 9);
        assert!(e.call(0x00af_ac90, &args![a, 1u32, file, 0u32]).bool());
        assert_eq!(e.mem.u32(files + 16 + 0xc), 0x8000_0000);
        let missing = hash(&mut e, 7);
        assert!(!e.call(0x00af_ac90, &args![a, 1u32, missing, 0u32]).bool());
    }

    #[test]
    fn n083_archive_file_transfers_seek_then_move() {
        let (mut e, _) = engine();
        let a = archive(&mut e);
        let mut slots = vec![0u32; 13];
        slots[NAME_SLOT as usize / 4] = 0x0070_0518;
        slots[SEEK_SLOT as usize / 4] = 0x0070_0514;
        slots[TRANSFER_SLOT_2C as usize / 4] = 0x0070_052c;
        slots[TRANSFER_SLOT_30 as usize / 4] = 0x0070_0530;
        e.put_vtable(0x0300_0600, &slots);
        e.mem.set_u32(a.addr(), 0x0300_0600);
        e.register(0x0070_0518, |_, _| Ret::default());
        e.register(0x0070_0514, |_, _| Ret::default());
        e.register(0x0070_052c, |_, w| (w[2] / 2).into_ret());
        e.register(0x0070_0530, |_, w| w[2].into_ret());
        let file = archive_file(&mut e, 0x40);
        e.set(file, ArchiveFile::spArchive, a.addr());
        e.set(file, ArchiveFile::iArchiveOffset, 0x100);
        e.mem.set_u32(file.addr() + POSITION_OVERRIDE, 0xffff_ffff);
        e.set(file.cast::<BSFile>(), BSFile::iTrueFilePos, 8);
        e.mem.set_u32(a.addr() + POSITION_OVERRIDE, 0xffff_ffff);
        e.set(a.cast::<BSFile>(), BSFile::iTrueFilePos, 0x108);
        // In place already: no seek.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00af_bc70, &args![file, 0x5000u32, 0u32]).u32(),
            0x40
        );
        assert!(calls_to(&e, 0x0070_0514).is_empty());
        assert_eq!(
            calls_to(&e, 0x0070_0530),
            vec![vec![a.addr(), 0x5000, 0x40]]
        );
        assert_eq!(e.get(file.cast::<BSFile>(), BSFile::iTrueFilePos), 0x48);
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![a.addr() + 0x19c]]
        );
        assert_eq!(
            calls_to(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![a.addr() + 0x19c]]
        );
        assert_eq!(calls_to(&e, 0x0070_0518).len(), 1);
        // Elsewhere (the word +0x38 standing for the position): a seek first.
        e.mem.set_u32(file.addr() + POSITION_OVERRIDE, 0x10);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00af_bde0, &args![file, 0x5000u32, 0u32]).u32(),
            0x20
        );
        assert_eq!(calls_to(&e, 0x0070_0514), vec![vec![a.addr(), 0x110, 0]]);
        assert_eq!(
            calls_to(&e, 0x0070_052c),
            vec![vec![a.addr(), 0x5000, 0x40]]
        );
        assert_eq!(e.get(file.cast::<BSFile>(), BSFile::iTrueFilePos), 0x68);
    }

    #[test]
    fn n083_list_remove_handles_head_and_later_nodes() {
        let (mut e, _) = engine();
        let first = archive(&mut e);
        let second = archive(&mut e);
        let third = archive(&mut e);
        let item = e.mem.alloc(4);
        // Null item or empty list: nothing.
        let empty = node(&mut e, 0, 0);
        e.call(0x00af_c960, &args![empty, item]);
        e.mem.set_u32(item, first.addr());
        e.call(0x00af_c960, &args![empty, item]);
        assert!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        let c = node(&mut e, third.addr(), 0);
        let b = node(&mut e, second.addr(), c);
        let head = node(&mut e, first.addr(), b);
        // A later node: unlinked and freed.
        e.mem.set_u32(item, second.addr());
        e.call(0x00af_c960, &args![head, item]);
        assert_eq!(e.mem.u32(head + 4), c);
        assert_eq!(count(&e, second), 0);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, b]]
        );
        // The head: the next archive moves in.
        e.mem.set_u32(item, first.addr());
        e.call(0x00af_c960, &args![head, item]);
        assert_eq!(e.mem.u32(head), third.addr());
        assert_eq!(e.mem.u32(head + 4), 0);
        assert_eq!(count(&e, first), 0);
        assert_eq!(count(&e, third), 1);
        // The head alone: its archive cleared.
        e.mem.set_u32(item, third.addr());
        e.call(0x00af_c960, &args![head, item]);
        assert_eq!(e.mem.u32(head), 0);
        assert_eq!(count(&e, third), 0);
        // Not in the list: nothing.
        let lone = node(&mut e, first.addr(), 0);
        e.mem.set_u32(item, second.addr());
        e.call_log = Some(vec![]);
        e.call(0x00af_c960, &args![lone, item]);
        assert!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
    }

    #[test]
    fn n083_hash_array_methods() {
        let (mut e, _) = engine();
        e.register(HASH_ARRAY_INITIAL_SETUP, |_, _| Ret::default());
        e.register(SIMPLE_ARRAY_CLEAR_00A1BFA0, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_REALLOCATE, |_, _| 0x7777u32.into_ret());
        let array = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00af_cce0, &args![array]).u32(), array);
        assert_eq!(e.mem.u32(array), HASH_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&e, HASH_ARRAY_INITIAL_SETUP),
            vec![vec![array, 0, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00af_cd10, &args![array, 3u32]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 12]]
        );
        e.call(0x00af_cd40, &args![array, 0x4000u32]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x4000]]
        );
        assert_eq!(
            e.call(0x00af_cd60, &args![array, 0x4000u32, 5u32]).u32(),
            0x7777
        );
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_REALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x4000, 20]]
        );
        e.call_log = Some(vec![]);
        e.mem.set_u32(array, 0);
        e.call(0x00af_cdc0, &args![array]);
        assert_eq!(e.mem.u32(array), HASH_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&e, SIMPLE_ARRAY_CLEAR_00A1BFA0),
            vec![vec![array, 1]]
        );
        for (flags, freed) in [(0u32, 0usize), (1, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00af_cd90, &args![array, flags]).u32(), array);
            assert_eq!(calls_to(&e, SIMPLE_ARRAY_CLEAR_00A1BFA0).len(), 1);
            assert_eq!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).len(), freed);
        }
    }
}

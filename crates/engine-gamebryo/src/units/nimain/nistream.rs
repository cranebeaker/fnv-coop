//! `NiMain/nistream.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiStream` is the Gamebryo model-file loader (`NiStream::Load` and the
//! `NiStream::LoadStream` it runs): it reads the header, the table of class
//! names, the object records, then links and post-links them. The per-class
//! `LoadBinary`/`LinkObject`/`PostLinkObject` functions are called through the
//! objects' virtual tables and are not part of this unit.
//!
//! The stream reads and writes through an `NiBinaryStream`: slot `+4` of its
//! virtual table says whether it is usable, `+8` seeks, `+0xC` gives the
//! position and `+0x10` sets the endian swap; the object also holds the read
//! function (`+8`) and write function (`+0xC`) as plain function pointers, which
//! the code calls as `function(stream, buffer, bytes, element sizes, count)`
//! and adds the byte count they return to the stream's position at `+4`
//! (`stream_transfer`).
//!
//! The pooled strings (`NiFixedString`) are handles to the characters; the
//! reference count is the word 8 bytes in front of them, and the handle at
//! `0109b220` stands for the empty string (never counted).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTArray, NiTPointerMap};
use crate::units::platform::MEMORY_MANAGER;

/// Import slots of `InterlockedIncrement`, `InterlockedDecrement`,
/// `EnterCriticalSection` and `LeaveCriticalSection`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The critical section (`NiStream::ms_kCleanupCriticalSection`, Xbox PDB)
/// the stream's clean-up functions hold.
const CLEANUP_CRITICAL_SECTION: u32 = 0x011f_4800;
/// The handle of the empty `NiFixedString`.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// `NiStream::ms_pkLoaders` (Xbox PDB): the map from class name to the
/// function that creates an object of that class.
const LOADERS: u32 = 0x011f_4784;
/// `NiStream::ms_pkPostProcessFunctions` (Xbox PDB): the array of functions
/// run on every top-level object after a load.
const POST_PROCESS_FUNCTIONS: u32 = 0x011f_4788;
/// `NiStream::bUseDefaultPath` (Xbox PDB): a byte the constructor and the
/// start-up function set to 1.
const USE_DEFAULT_PATH: u32 = 0x011f_4780;
/// The file versions the stream accepts: the smallest and the largest
/// (`NiStream::ms_uiNifMinVersion`, `ms_uiNifMaxVersion`) and the largest and
/// smallest user defined versions (`ms_uiNifMaxUserDefinedVersion`,
/// `ms_uiNifMinUserDefinedVersion`, Xbox PDB), named by how `LoadHeader`
/// compares against them.
const NIF_MIN_VERSION: u32 = 0x011a_9b08;
const NIF_MAX_VERSION: u32 = 0x011a_9b0c;
const NIF_MAX_USER_VERSION: u32 = 0x011a_9b10;
const NIF_MIN_USER_VERSION: u32 = 0x011f_478c;
/// A byte the code reads to know whether the platform is little endian
/// (`00aa1d40` returns it): `LoadHeader` fails on a file written big endian
/// when it is not zero.
const ENDIAN_FLAG_FUNCTION: u32 = 0x00aa_1d40;

/// The shared pool the node free list of the maps uses (`0040fbf0` locks it,
/// `011c5f80` is the lock, `011c5f84` its count, `011c5f58` the free list).
const POOL_LOCK: u32 = 0x011c_5f80;
const POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const POOL_FREE_LIST: u32 = 0x011c_5f58;
/// `0040fbf0` (`__thiscall(lock, 0)`), the call every pool operation starts
/// with.
const POOL_LOCK_FUNCTION: u32 = 0x0040_fbf0;

// Callees outside this unit.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `00aa1070`/`00aa10f0`: the cdecl allocate and free wrappers.
const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;
/// `operator new` (`00aa13e0`, cdecl, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `NiFixedString::NiFixedString()` (`007613f0`), the element constructor of
/// the fixed string array.
const FIXED_STRING_CONSTRUCTOR: u32 = 0x0076_13f0;
/// `00a5b690`: the pooled string handle of a C string (null for null).
const FIXED_STRING_FROM_CSTRING: u32 = 0x00a5_b690;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRNCPY_S: u32 = 0x00ec_8d5f;
const STRCAT_S: u32 = 0x00ec_6bd2;
const STRSTR: u32 = 0x00ec_7750;
const STRTOK_S: u32 = 0x00ec_c5f8;
const ATOL: u32 = 0x00ec_a6d3;
const MEMSET: u32 = 0x00ec_61c0;
/// `strcpy_s` as a forwarding function (`00404d80`), used for some error
/// messages.
const STRCPY_S_FORWARD: u32 = 0x0040_4d80;
const NI_PATH_STANDARDIZE: u32 = 0x00aa_2a00;
const NI_FILE_GET_FILE: u32 = 0x00aa_1a20;
const NI_MEM_STREAM_CONSTRUCT: u32 = 0x00aa_2fb0;
const NI_MEM_STREAM_CONSTRUCT_EMPTY: u32 = 0x00aa_2ff0;
const NI_MEM_STREAM_DESTRUCT: u32 = 0x00aa_2d30;
/// `00aa2d60`: takes the buffer of a memory stream.
const NI_MEM_STREAM_TAKE_BUFFER: u32 = 0x00aa_2d60;
/// `00aa1f10`: `__thiscall(stream, buffer, size)` reads a line.
const READ_LINE: u32 = 0x00aa_1f10;
/// `00aa1fd0`: `__thiscall(stream, string)` writes the characters of a
/// string without the terminator.
const WRITE_STRING: u32 = 0x00aa_1fd0;
/// `NiStreamLoadBinary<unsigned int>` (`00a26df0`), `NiStreamLoadBinary<unsigned
/// char>` with one byte (`00a643b0`) and with a count (`00a471d0`), all cdecl
/// on the stream.
const LOAD_BINARY_U32: u32 = 0x00a2_6df0;
const LOAD_BINARY_U8: u32 = 0x00a6_43b0;
const LOAD_BINARY_BYTES: u32 = 0x00a4_71d0;
/// `NiTLargeArray<NiPointer<NiObject>>::SetSize` and `::SetAt`
/// (engine map names), `__thiscall(size)` and `__thiscall(index, pointer to
/// the NiPointer)`.
const LARGE_ARRAY_SET_SIZE: u32 = 0x00a6_4fe0;
const LARGE_ARRAY_SET_AT: u32 = 0x00cc_1a10;
/// `NiTSet<..>::Realloc` (`00a311e0`, `__thiscall(new capacity)`).
const SET_REALLOC: u32 = 0x00a3_11e0;
/// `__thiscall(size)` that sizes the object sizes array (`00864280`).
const SIZES_ARRAY_SET_SIZE: u32 = 0x0086_4280;
/// The `NiTPrimitiveArray<NiObjectGroup*>` functions: `SetSize(size)`
/// (`00a90180`) and `SetAt(index, pointer to the value)` (`00ba3aa0`), and the
/// `NiObjectGroup` constructor (`00a8a620`, `__thiscall(size)`).
const GROUPS_SET_SIZE: u32 = 0x00a9_0180;
const GROUPS_SET_AT: u32 = 0x00ba_3aa0;
const OBJECT_GROUP_CONSTRUCT: u32 = 0x00a8_a620;
/// `NiTMapBase<..>::GetAt(key, pointer to the result) -> bool` on the loader
/// map and the registration map (`00a53c30`) and on the name table of the
/// save (`00c74990`); `RemoveAll` of those maps (`00b601a0`).
const MAP_GET_AT: u32 = 0x00a5_3c30;
const SAVE_MAP_GET_AT: u32 = 0x00c7_4990;
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
/// `NiTMapBase<..>::SetAt(key, value)` of the registration map (`00b57ea0`)
/// and `RemoveAt(key)` of the loader map (`00e91fd0`).
const REGISTER_MAP_SET_AT: u32 = 0x00b5_7ea0;
const LOADER_MAP_REMOVE_AT: u32 = 0x00e9_1fd0;
/// `NiTStringTemplateMap::SetAt(name, function)` of the loader map
/// (`00a642a0`) and `NiTPrimitiveArray::Add(pointer to the function)` of the
/// post process array (`00a64330`).
const LOADER_MAP_SET_AT: u32 = 0x00a6_42a0;
const POST_PROCESS_ADD: u32 = 0x00a6_4330;
/// `00a644a0`: frees the storage of the fixed strings array (cdecl).
const FIXED_STRINGS_FREE_STORAGE: u32 = 0x00a6_44a0;
/// `00a47600`: frees the storage of an object array (cdecl).
const OBJECT_ARRAY_FREE_STORAGE: u32 = 0x00a4_7600;
/// `00aa3040`: the call on the stream's thread handle made when the
/// background load status is 3 (a `SuspendThread` wrapper in the decompiler).
const THREAD_SUSPEND: u32 = 0x00aa_3040;
/// `BSTexturePalette::SetTexture` (`00a61c50`, cdecl) and `GetTexture`
/// (`00a61b90`, cdecl), engine map names.
const TEXTURE_PALETTE_SET_TEXTURE: u32 = 0x00a6_1c50;
const TEXTURE_PALETTE_GET_TEXTURE: u32 = 0x00a6_1b90;
/// `NiTPointerListBase::AddHead`'s node allocator (`0043a010`).
const POOL_ALLOCATE_NODE: u32 = 0x0043_a010;

/// The runtime class record of `NiNode` (`011f4280`), the class `00a63d10`
/// looks for in the chain of an object's class records.
const NODE_CLASS_RECORD: u32 = 0x011f_4280;

// Virtual table slots of `NiStream` (Xbox PDB `vslot` list, checked against
// the calls in this unit).
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_LOAD: u32 = 0x04;
const SLOT_SAVE: u32 = 0x10;
const SLOT_LOAD_HEADER: u32 = 0x34;
const SLOT_SAVE_HEADER: u32 = 0x38;
const SLOT_LOAD_STREAM: u32 = 0x3c;
const SLOT_SAVE_STREAM: u32 = 0x40;
const SLOT_REGISTER_OBJECTS: u32 = 0x44;
const SLOT_LOAD_TOP_LEVEL_OBJECTS: u32 = 0x48;
const SLOT_SAVE_TOP_LEVEL_OBJECTS: u32 = 0x4c;
const SLOT_LOAD_OBJECT: u32 = 0x50;
const SLOT_PRE_SAVE_OBJECT_SIZE_TABLE: u32 = 0x54;
const SLOT_SAVE_OBJECT_SIZE_TABLE: u32 = 0x58;
const SLOT_LOAD_OBJECT_SIZE_TABLE: u32 = 0x5c;
const SLOT_SAVE_LINK_ID: u32 = 0x30;
// Slots of the objects (`NiObject` and its classes).
const OBJECT_SLOT_GET_RTTI: u32 = 0x08;
const OBJECT_SLOT_AS_NODE: u32 = 0x0c;
const OBJECT_SLOT_LOAD_BINARY: u32 = 0x4c;
const OBJECT_SLOT_LINK_OBJECT: u32 = 0x50;
const OBJECT_SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const OBJECT_SLOT_SAVE_BINARY: u32 = 0x58;
const OBJECT_SLOT_POST_LINK_OBJECT: u32 = 0x6c;
const OBJECT_SLOT_IS_NULL_PLACEHOLDER: u32 = 0x70;
const OBJECT_SLOT_GET_RTTI_NAME: u32 = 0x74;
const OBJECT_SLOT_GET_BLOCK_ALLOCATION_SIZE: u32 = 0x7c;
const OBJECT_SLOT_GET_GROUP: u32 = 0x80;
// Slots of the binary stream.
const STREAM_SLOT_IS_VALID: u32 = 0x04;
const STREAM_SLOT_SEEK: u32 = 0x08;
const STREAM_SLOT_GET_POSITION: u32 = 0x0c;
const STREAM_SLOT_SET_ENDIAN_SWAP: u32 = 0x10;
/// The stream's read and write function pointers.
const STREAM_READ_FUNCTION: u32 = 0x08;
const STREAM_WRITE_FUNCTION: u32 = 0x0c;
// Slots of the maps (`NiTMapBase`).
const MAP_SLOT_HASH: u32 = 0x04;
const MAP_SLOT_KEYS_EQUAL: u32 = 0x08;
const MAP_SLOT_SET_VALUE: u32 = 0x0c;
/// `bCopyKeys`: the byte at `+0x10` of the string maps.
const MAP_COPY_KEYS: u32 = 0x10;

// Virtual tables the code writes into objects (names by the class the
// object is, where the code and the Xbox PDB agree; otherwise by role).
const NI_STREAM_VTABLE: u32 = 0x0109_bdcc;
const GROUPS_ARRAY_VTABLE_109BD90: u32 = 0x0109_bd90;
const GROUPS_ARRAY_BASE_VTABLE_109BC3C: u32 = 0x0109_bc3c;
const OBJECT_ARRAY_VTABLE_109BF6C: u32 = 0x0109_bf6c;
const OBJECT_ARRAY_BASE_VTABLE_109BD98: u32 = 0x0109_bd98;
const SIZES_ARRAY_VTABLE_1081EF0: u32 = 0x0108_1ef0;
const SIZES_ARRAY_BASE_VTABLE_1081FC8: u32 = 0x0108_1fc8;
const FIXED_STRINGS_ARRAY_VTABLE_109BF74: u32 = 0x0109_bf74;
const FIXED_STRINGS_ARRAY_BASE_VTABLE_109BD4C: u32 = 0x0109_bd4c;
const REGISTER_MAP_BASE_VTABLE_109BC64: u32 = 0x0109_bc64;
const REGISTER_MAP_VTABLE_109BD54: u32 = 0x0109_bd54;
const LOADER_MAP_ROOT_VTABLE_109BC84: u32 = 0x0109_bc84;
const LOADER_MAP_MIDDLE_VTABLE_109BCA4: u32 = 0x0109_bca4;
const LOADER_MAP_BASE_VTABLE_109BCC4: u32 = 0x0109_bcc4;
const LOADER_MAP_VTABLE_109BF4C: u32 = 0x0109_bf4c;
const NAME_MAP_ROOT_VTABLE_109BCEC: u32 = 0x0109_bcec;
const NAME_MAP_MIDDLE_VTABLE_109BD0C: u32 = 0x0109_bd0c;
const NAME_MAP_BASE_VTABLE_109BD2C: u32 = 0x0109_bd2c;
const NAME_MAP_VTABLE_109BF7C: u32 = 0x0109_bf7c;
const POST_PROCESS_ARRAY_VTABLE_109BD88: u32 = 0x0109_bd88;

// Texts in the exe's data.
const TEXT_CANNOT_OPEN_FILE: u32 = 0x0109_bd70;
const TEXT_HEADER_LINE: u32 = 0x0109_bd9c;
const TEXT_FILE_FORMAT: u32 = 0x0109_bee8;
const TEXT_NOT_A_NIF_FILE: u32 = 0x0109_bed8;
const TEXT_VERSION_TOO_OLD: u32 = 0x0109_bec0;
const TEXT_UNKNOWN_VERSION: u32 = 0x0109_bea8;
const TEXT_ENDIAN_MISMATCH: u32 = 0x0109_be94;
const TEXT_USER_VERSION_TOO_OLD: u32 = 0x0109_be6c;
const TEXT_UNKNOWN_USER_VERSION: u32 = 0x0109_be48;
const TEXT_BS_VERSION_TOO_OLD: u32 = 0x0109_be2c;
const TEXT_OLDER_FILES: u32 = 0x0109_bef4;
const TEXT_NO_CREATE_FUNCTION: u32 = 0x0109_bc40;
const TEXT_DOT: u32 = 0x0104_4c28;
/// The four bytes `SaveFixedString` writes for an empty string (all ones).
const NO_STRING_INDEX: u32 = 0x0109_bc30;

/// Error codes `LoadHeader` and the loaders put in `m_uiLastError`.
const ERROR_CANNOT_OPEN: u32 = 1;
const ERROR_NOT_A_NIF_FILE: u32 = 2;
const ERROR_VERSION_TOO_OLD: u32 = 3;
const ERROR_UNKNOWN_VERSION: u32 = 4;
const ERROR_NO_CREATE_FUNCTION: u32 = 5;
const ERROR_ENDIAN_MISMATCH: u32 = 6;
/// `m_eBackgroundLoadStatus` values the code compares: 2 cancels the load,
/// 3 asks for the thread to be suspended (and becomes 4).
const STATUS_CANCELLED: u32 = 2;
const STATUS_SUSPEND_REQUESTED: u32 = 3;
const STATUS_SUSPENDED: u32 = 4;
/// Size of the error message buffer.
const ERROR_MESSAGE_SIZE: u32 = 0x104;

layout! {
    /// `NiTLargeArray<T>` (Xbox PDB), 0x18 bytes: vtable, element pointer and
    /// four 32-bit counts. (Not in `crate::types`; the unit contains its
    /// instances.)
    pub struct NiTLargeArray: 0x18 {
        /// `m_pBase` (Xbox PDB): the elements, 4 bytes each here.
        0x04 m_pBase: u32,
        /// `m_uiMaxSize` (Xbox PDB): capacity.
        0x08 m_uiMaxSize: u32,
        /// `m_uiSize` (Xbox PDB): one past the highest used index.
        0x0C m_uiSize: u32,
        /// `m_uiESize` (Xbox PDB): number of non-empty elements.
        0x10 m_uiESize: u32,
        /// `m_uiGrowBy` (Xbox PDB).
        0x14 m_uiGrowBy: u32,
    }

    /// `NiTPrimitiveSet<unsigned int>` (Xbox PDB), 0xC bytes (no vtable).
    pub struct NiTPrimitiveSet: 0x0C {
        /// `m_pBase` (Xbox PDB).
        0x00 m_pBase: u32,
        /// `m_uiAlloced` (Xbox PDB): capacity.
        0x04 m_uiAlloced: u32,
        /// `m_uiUsed` (Xbox PDB).
        0x08 m_uiUsed: u32,
    }

    /// `NiBinaryStream` (Xbox PDB), 0x10 bytes.
    pub struct NiBinaryStream: 0x10 {
        /// `m_uiAbsoluteCurrentPos` (Xbox PDB).
        0x04 m_uiAbsoluteCurrentPos: u32,
        /// `m_pfnRead` (Xbox PDB).
        0x08 m_pfnRead: u32,
        /// `m_pfnWrite` (Xbox PDB).
        0x0C m_pfnWrite: u32,
    }

    /// `NiStream` (Xbox PDB), 0x5C4 bytes on the Xbox and on PC (the
    /// offsets below are the PC code's, which match the PDB's).
    pub struct NiStream: 0x5C4 {
        /// `m_BSStreamHeader.uiVersion` (Xbox PDB).
        0x004 m_BSStreamHeader: u32,
        /// `m_BSStreamHeader.pAuthor` (Xbox PDB), 64 characters.
        0x008 pAuthor: u8,
        /// `m_BSStreamHeader.pProcessScript` (Xbox PDB), 64 characters.
        0x048 pProcessScript: u8,
        /// `m_BSStreamHeader.pExportScript` (Xbox PDB), 64 characters.
        0x088 pExportScript: u8,
        /// `m_kGroups` (Xbox PDB): `NiTPrimitiveArray<NiObjectGroup*>`.
        0x0C8 m_kGroups: Inline<NiTArray>,
        /// `m_uiNifFileVersion` (Xbox PDB).
        0x0D8 m_uiNifFileVersion: u32,
        /// `m_uiNifFileUserDefinedVersion` (Xbox PDB).
        0x0DC m_uiNifFileUserDefinedVersion: u32,
        /// `m_acFileName` (Xbox PDB), 260 characters.
        0x0E0 m_acFileName: u8,
        /// `m_bSaveLittleEndian` (Xbox PDB).
        0x1E4 m_bSaveLittleEndian: u8,
        /// `m_bSourceIsLittleEndian` (Xbox PDB).
        0x1E5 m_bSourceIsLittleEndian: u8,
        /// `m_kObjects` (Xbox PDB): `NiTLargeObjectArray<NiPointer<NiObject>>`.
        0x1EC m_kObjects: Inline<NiTLargeArray>,
        /// `m_kObjectSizes` (Xbox PDB): `NiTLargePrimitiveArray<unsigned int>`.
        0x204 m_kObjectSizes: Inline<NiTLargeArray>,
        /// `m_kTopObjects` (Xbox PDB).
        0x21C m_kTopObjects: Inline<NiTLargeArray>,
        /// `m_kFixedStrings` (Xbox PDB): `NiTLargeObjectArray<NiFixedString>`.
        0x234 m_kFixedStrings: Inline<NiTLargeArray>,
        /// `m_pkIstr` (Xbox PDB): the stream being read.
        0x24C m_pkIstr: u32,
        /// `m_pkOstr` (Xbox PDB): the stream being written.
        0x250 m_pkOstr: u32,
        /// `m_kLinkIDs` (Xbox PDB).
        0x254 m_kLinkIDs: Inline<NiTPrimitiveSet>,
        /// `m_uiLinkIndex` (Xbox PDB).
        0x260 m_uiLinkIndex: u32,
        /// `m_kLinkIDBlocks` (Xbox PDB).
        0x264 m_kLinkIDBlocks: Inline<NiTPrimitiveSet>,
        /// `m_uiLinkBlockIndex` (Xbox PDB).
        0x270 m_uiLinkBlockIndex: u32,
        /// `m_kRegisterMap` (Xbox PDB): `NiTPointerMap<NiObject const*, unsigned int>`.
        0x274 m_kRegisterMap: Inline<NiTPointerMap>,
        /// `m_eBackgroundLoadStatus` (Xbox PDB).
        0x28C m_eBackgroundLoadStatus: u32,
        /// `m_uiLoad` (Xbox PDB): index of the next object to load.
        0x294 m_uiLoad: u32,
        /// `m_uiLink` (Xbox PDB): index of the next object to link.
        0x298 m_uiLink: u32,
        /// `m_uiPostLink` (Xbox PDB): index of the next object to post-link.
        0x29C m_uiPostLink: u32,
        /// `m_pkThread` (Xbox PDB).
        0x2A0 m_pkThread: u32,
        /// `m_pkBGLoadProc` (Xbox PDB).
        0x2A4 m_pkBGLoadProc: u32,
        /// `m_ePriority` (Xbox PDB).
        0x2A8 m_ePriority: u32,
        /// `m_kAffinity` (Xbox PDB), two words.
        0x2AC m_kAffinity: u32,
        /// `m_uiLastError` (Xbox PDB).
        0x3B8 m_uiLastError: u32,
        /// `m_acLastErrorMessage` (Xbox PDB), 260 characters.
        0x3BC m_acLastErrorMessage: u8,
        /// `m_acFilePath` (Xbox PDB), 260 characters.
        0x4C0 m_acFilePath: u8,
    }
}

// ---------------------------------------------------------------------------
// Helpers shared by the functions of this unit.

fn empty_handle(e: &Engine) -> u32 {
    e.global(EMPTY_FIXED_STRING)
}

/// Counts one more reference to a pooled string.
fn handle_add_ref(e: &mut Engine, handle: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![handle.wrapping_sub(8)]);
}

/// Drops one reference to a pooled string.
fn handle_release(e: &mut Engine, handle: u32) {
    e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
}

/// `InterlockedIncrement` of the reference count at `+4`.
fn add_ref(e: &mut Engine, object: u32) {
    if object == 0 {
        return;
    }
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

/// `InterlockedDecrement` of the reference count at `+4`; `DeleteThis`
/// (slot 4) runs when it reaches zero.
fn release(e: &mut Engine, object: u32) {
    if object == 0 {
        return;
    }
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if count == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(ALLOCATE, &args![size]).u32()
}

fn free(e: &mut Engine, block: u32) {
    e.call(FREE, &args![block]);
}

/// `MemoryManager::Allocate` on the singleton.
fn manager_allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32()
}

/// `MemoryManager::Deallocate` on the singleton (what `operator delete`
/// does).
fn manager_deallocate(e: &mut Engine, block: u32) {
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

/// Calls the stream's read or write function pointer once
/// (`function(stream, buffer, bytes, element sizes, 1)`, `sizes` pointing at
/// a local with `element_size`) and adds the byte count it returns to the
/// stream's position.
fn stream_transfer(
    e: &mut Engine,
    stream: u32,
    member: u32,
    buffer: u32,
    bytes: u32,
    element_size: u32,
) {
    e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), element_size);
        let function = e.mem.u32(stream.wrapping_add(member));
        let done = e
            .call(function, &args![stream, buffer, bytes, sizes, 1u32])
            .u32();
        let position = e.mem.u32(stream.wrapping_add(4));
        e.mem
            .set_u32(stream.wrapping_add(4), position.wrapping_add(done));
    });
}

/// Reads `bytes` into `buffer` from the stream being loaded.
fn read_into(e: &mut Engine, this: Ptr<NiStream>, buffer: u32, bytes: u32, element_size: u32) {
    let stream = e.get(this, NiStream::m_pkIstr);
    stream_transfer(e, stream, STREAM_READ_FUNCTION, buffer, bytes, element_size);
}

/// Writes `bytes` from `buffer` to the stream being saved.
fn write_from(e: &mut Engine, this: Ptr<NiStream>, buffer: u32, bytes: u32, element_size: u32) {
    let stream = e.get(this, NiStream::m_pkOstr);
    stream_transfer(
        e,
        stream,
        STREAM_WRITE_FUNCTION,
        buffer,
        bytes,
        element_size,
    );
}

/// Reads a four-byte value into a zeroed local.
fn read_u32(e: &mut Engine, this: Ptr<NiStream>) -> u32 {
    e.with_stack(4, |e, local| {
        read_into(e, this, local.addr(), 4, 4);
        e.mem.u32(local.addr())
    })
}

/// Reads a two-byte value into a zeroed local.
fn read_u16(e: &mut Engine, this: Ptr<NiStream>) -> u16 {
    e.with_stack(4, |e, local| {
        read_into(e, this, local.addr(), 2, 2);
        e.mem.u16(local.addr())
    })
}

/// Writes a four-byte value held in a local.
fn write_u32(e: &mut Engine, this: Ptr<NiStream>, value: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        write_from(e, this, local.addr(), 4, 4);
    });
}

/// Writes a two-byte value held in a local.
fn write_u16(e: &mut Engine, this: Ptr<NiStream>, value: u16) {
    e.with_stack(4, |e, local| {
        e.mem.set_u16(local.addr(), value);
        write_from(e, this, local.addr(), 2, 2);
    });
}

/// Records an error: the code, then the message copied with `copy`
/// (`__cdecl(destination, 0x104, text)`).
fn set_error(e: &mut Engine, this: Ptr<NiStream>, code: u32, copy: u32, text: u32) {
    e.set(this, NiStream::m_uiLastError, code);
    let message = this.byte_add(NiStream::m_acLastErrorMessage.off);
    e.call(copy, &args![message, ERROR_MESSAGE_SIZE, text]);
}

/// Appends a value to a `NiTPrimitiveSet<unsigned int>`: doubles the
/// capacity (1 from 0) through `Realloc` when it is full.
fn set_add(e: &mut Engine, set: Ptr<NiTPrimitiveSet>, value: u32) {
    let capacity = e.get(set, NiTPrimitiveSet::m_uiAlloced);
    if e.get(set, NiTPrimitiveSet::m_uiUsed) == capacity {
        let grown = if capacity == 0 {
            1
        } else {
            capacity.wrapping_add(capacity)
        };
        e.call(SET_REALLOC, &args![set, grown]);
    }
    let used = e.get(set, NiTPrimitiveSet::m_uiUsed);
    let base = e.get(set, NiTPrimitiveSet::m_pBase);
    e.mem
        .set_u32(base.wrapping_add(used.wrapping_mul(4)), value);
    e.set(set, NiTPrimitiveSet::m_uiUsed, used.wrapping_add(1));
}

/// The inlined `NiTLargePrimitiveArray<unsigned int>::SetAt` the code uses
/// for the object sizes: grows the used size to `index + 1` when the index
/// is past it, keeps the count of non-zero elements, stores the value.
fn sizes_set_at(e: &mut Engine, sizes: Ptr<NiTLargeArray>, index: u32, value: u32) {
    let used = e.get(sizes, NiTLargeArray::m_uiSize);
    let base = e.get(sizes, NiTLargeArray::m_pBase);
    let count = e.get(sizes, NiTLargeArray::m_uiESize);
    if index >= used {
        e.set(sizes, NiTLargeArray::m_uiSize, index.wrapping_add(1));
        if value != 0 {
            e.set(sizes, NiTLargeArray::m_uiESize, count.wrapping_add(1));
        }
    } else {
        let old = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        if value != 0 {
            if old == 0 {
                e.set(sizes, NiTLargeArray::m_uiESize, count.wrapping_add(1));
            }
        } else if old != 0 {
            e.set(sizes, NiTLargeArray::m_uiESize, count.wrapping_sub(1));
        }
    }
    let base = e.get(sizes, NiTLargeArray::m_pBase);
    e.mem
        .set_u32(base.wrapping_add(index.wrapping_mul(4)), value);
}

/// Appends an object to a `NiTLargeArray<NiPointer<NiObject>>`: the array
/// grows by its grow-by when full, and the array holds a counted reference.
fn object_array_append(e: &mut Engine, array: Ptr<NiTLargeArray>, object: u32) {
    let index = e.get(array, NiTLargeArray::m_uiSize);
    if index >= e.get(array, NiTLargeArray::m_uiMaxSize) {
        let grow = e.get(array, NiTLargeArray::m_uiGrowBy);
        e.call(
            LARGE_ARRAY_SET_SIZE,
            &args![array, grow.wrapping_add(index)],
        );
    }
    e.with_stack(4, |e, holder| {
        e.mem.set_u32(holder.addr(), object);
        e.call(LARGE_ARRAY_SET_AT, &args![array, index, holder]);
    });
}

// ---------------------------------------------------------------------------

// Translated from 00a632e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::SetLastError` (Xbox PDB): stores the error code.
pub fn ni_stream_set_last_error(e: &mut Engine, this: Ptr<NiStream>, error: u32) {
    e.set(this, NiStream::m_uiLastError, error);
}

// Translated from 00a632f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::Load` (Xbox PDB) from a memory buffer: wraps the buffer in an
/// `NiMemStream` (a 0x24-byte local) and loads from it through virtual
/// function `+4`. Returns that function's result.
pub fn ni_stream_load(e: &mut Engine, this: Ptr<NiStream>, buffer: u32, size: u32) -> bool {
    e.with_stack(0x24, |e, memory_stream| {
        e.call(NI_MEM_STREAM_CONSTRUCT, &args![memory_stream, buffer, size]);
        let loaded = e
            .vcall(this.addr(), SLOT_LOAD, &args![memory_stream])
            .bool();
        e.call(NI_MEM_STREAM_DESTRUCT, &args![memory_stream]);
        loaded
    })
}

// Translated from 00a63330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::Load_ov2` (Xbox PDB), from a binary stream: when the stream
/// says it is usable (slot `+4`) it becomes `m_pkIstr` for the length of the
/// call to `LoadStream` (slot `+0x3c`), whose result is returned. An unusable
/// stream clears `m_pkIstr` and fails.
pub fn ni_stream_load_ov2(e: &mut Engine, this: Ptr<NiStream>, stream: Ptr) -> bool {
    let usable = e
        .vcall(stream.addr(), STREAM_SLOT_IS_VALID, &args![])
        .bool();
    if !usable {
        e.set(this, NiStream::m_pkIstr, 0);
        return false;
    }
    e.set(this, NiStream::m_pkIstr, stream.addr());
    let loaded = e.vcall(this.addr(), SLOT_LOAD_STREAM, &args![]).bool();
    e.set(this, NiStream::m_pkIstr, 0);
    loaded
}

// Translated from 00a63380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::Save` (Xbox PDB), to a binary stream: it becomes `m_pkOstr`
/// for the length of the call to `SaveStream` (slot `+0x40`), whose result
/// (in `AL`; the decompiler shows no value) is returned.
pub fn ni_stream_save(e: &mut Engine, this: Ptr<NiStream>, stream: Ptr) -> bool {
    e.set(this, NiStream::m_pkOstr, stream.addr());
    let saved = e.vcall(this.addr(), SLOT_SAVE_STREAM, &args![]).bool();
    e.set(this, NiStream::m_pkOstr, 0);
    saved
}

// Translated from 00a633b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::SetTexture` (Xbox PDB): `BSTexturePalette::SetTexture(texture,
/// 0)`; the stream itself (`ECX`) is not used.
pub fn ni_stream_set_texture(e: &mut Engine, _this: Ptr<NiStream>, texture: u32) {
    e.call(TEXTURE_PALETTE_SET_TEXTURE, &args![texture, 0u32]);
}

// Translated from 00a633d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::GetTexture` (Xbox PDB): `BSTexturePalette::GetTexture(key,
/// out)`, whose `AL` is returned; the stream itself (`ECX`) is not used.
pub fn ni_stream_get_texture(e: &mut Engine, _this: Ptr<NiStream>, key: u32, out: Ptr) -> bool {
    e.call(TEXTURE_PALETTE_GET_TEXTURE, &args![key, out]).bool()
}

// Translated from 00a633f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Index of a fixed string (the `NiFixedString` at `string`) in the stream's
/// table of fixed strings, `0xffffffff` for the empty string or when it is
/// not there.
pub fn fn_00a633f0(e: &mut Engine, this: Ptr<NiStream>, string: Ptr) -> u32 {
    let handle = e.mem.u32(string.addr());
    if handle == empty_handle(e) {
        return u32::MAX;
    }
    let strings = this.at(NiStream::m_kFixedStrings);
    let count = e.get(strings, NiTLargeArray::m_uiSize);
    let base = e.get(strings, NiTLargeArray::m_pBase);
    for index in 0..count {
        if e.mem.u32(base.wrapping_add(index.wrapping_mul(4))) == handle {
            return index;
        }
    }
    u32::MAX
}

// Translated from 00a63430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::GetObjectFromLinkID` (Xbox PDB): takes the next link id of
/// the stream (`m_uiLinkIndex`) and returns the object it names, null for
/// the id `0xffffffff`.
pub fn ni_stream_get_object_from_link_id(e: &mut Engine, this: Ptr<NiStream>) -> u32 {
    let index = e.get(this, NiStream::m_uiLinkIndex);
    e.set(this, NiStream::m_uiLinkIndex, index.wrapping_add(1));
    let ids = e.get(this.at(NiStream::m_kLinkIDs), NiTPrimitiveSet::m_pBase);
    let id = e.mem.u32(ids.wrapping_add(index.wrapping_mul(4)));
    if id == u32::MAX {
        return 0;
    }
    let objects = e.get(this.at(NiStream::m_kObjects), NiTLargeArray::m_pBase);
    e.mem.u32(objects.wrapping_add(id.wrapping_mul(4)))
}

// Translated from 00a63460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::GetNumberOfLinkIDs` (Xbox PDB): takes the next entry of the
/// link id block counts (`m_uiLinkBlockIndex`).
pub fn ni_stream_get_number_of_link_ids(e: &mut Engine, this: Ptr<NiStream>) -> u32 {
    let index = e.get(this, NiStream::m_uiLinkBlockIndex);
    e.set(this, NiStream::m_uiLinkBlockIndex, index.wrapping_add(1));
    let blocks = e.get(this.at(NiStream::m_kLinkIDBlocks), NiTPrimitiveSet::m_pBase);
    e.mem.u32(blocks.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00a63480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::GetLinkIDFromObject` (Xbox PDB): the id the registration map
/// (`m_kRegisterMap`) has for an object, `0xffffffff` for a null object or
/// one that is not registered.
pub fn ni_stream_get_link_id_from_object(e: &mut Engine, this: Ptr<NiStream>, object: u32) -> u32 {
    if object == 0 {
        return u32::MAX;
    }
    e.with_stack(4, |e, id| {
        e.mem.set_u32(id.addr(), u32::MAX);
        let map = this.at(NiStream::m_kRegisterMap);
        e.call(MAP_GET_AT, &args![map, object, id]);
        e.mem.u32(id.addr())
    })
}

// Translated from 00a634b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates an object of the class with this name (cdecl): looks the name up
/// in the loader map and returns what the creation function returns, 0 when
/// there is no such class.
pub fn fn_00a634b0(e: &mut Engine, name: u32) -> u32 {
    e.with_stack(4, |e, function| {
        let loaders = e.mem.u32(LOADERS);
        let found = e.call(MAP_GET_AT, &args![loaders, name, function]).bool();
        if !found {
            return 0;
        }
        let creator = e.mem.u32(function.addr());
        e.call(creator, &args![]).u32()
    })
}

// Translated from 00a634e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::GetGroupFromID` (Xbox PDB): element `id` of the groups array.
pub fn ni_stream_get_group_from_id(e: &mut Engine, this: Ptr<NiStream>, id: u32) -> u32 {
    let groups = e.get(this.at(NiStream::m_kGroups), NiTArray::m_pBase);
    e.mem.u32(groups.wrapping_add(id.wrapping_mul(4)))
}

// Translated from 00a634f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Index of a group in the groups array, 0 when it is not there.
pub fn fn_00a634f0(e: &mut Engine, this: Ptr<NiStream>, group: u32) -> u32 {
    let groups = this.at(NiStream::m_kGroups);
    let count = e.get(groups, NiTArray::m_usSize) as u32;
    let base = e.get(groups, NiTArray::m_pBase);
    for index in 0..count {
        if e.mem.u32(base.wrapping_add(index.wrapping_mul(4))) == group {
            return index;
        }
    }
    0
}

// Translated from 00a63520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes a class name from the loader map (cdecl): `00e91fd0` on
/// `ms_pkLoaders`, whose result is returned.
pub fn fn_00a63520(e: &mut Engine, name: u32) -> u32 {
    let loaders = e.mem.u32(LOADERS);
    e.call(LOADER_MAP_REMOVE_AT, &args![loaders, name]).u32()
}

// Translated from 00a63540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes a function from the post process array (cdecl): the first
/// element equal to it is cleared (the count of used elements goes down by
/// one when it was set) and, when it was the last used slot, the size goes
/// down by one.
pub fn fn_00a63540(e: &mut Engine, function: u32) {
    let array = Ptr::<NiTArray>::new(e.mem.u32(POST_PROCESS_FUNCTIONS));
    let size = e.get(array, NiTArray::m_usSize) as u32;
    let base = e.get(array, NiTArray::m_pBase);
    for index in 0..size {
        if e.mem.u32(base.wrapping_add(index.wrapping_mul(4))) != function {
            continue;
        }
        if index < e.get(array, NiTArray::m_usSize) as u32 {
            let base = e.get(array, NiTArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let old = e.mem.u32(slot);
            e.mem.set_u32(slot, 0);
            if old != 0 {
                let count = e.get(array, NiTArray::m_usESize);
                e.set(array, NiTArray::m_usESize, count.wrapping_sub(1));
            }
            let last = e.get(array, NiTArray::m_usSize).wrapping_sub(1);
            if index == last as u32 {
                e.set(array, NiTArray::m_usSize, last);
            }
        }
        return;
    }
}

// Translated from 00a635b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::RegisterObjects` (Xbox PDB): asks every top-level object to
/// register itself and its children (virtual function `+0x54`,
/// `RegisterStreamables`) with this stream.
pub fn ni_stream_register_objects(e: &mut Engine, this: Ptr<NiStream>) {
    let top = this.at(NiStream::m_kTopObjects);
    let mut index = 0u32;
    while index < e.get(top, NiTLargeArray::m_uiSize) {
        let base = e.get(top, NiTLargeArray::m_pBase);
        let object = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        e.vcall(object, OBJECT_SLOT_REGISTER_STREAMABLES, &args![this]);
        index += 1;
    }
}

// Translated from 00a635e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::Save_ov2` (Xbox PDB), to a memory buffer: saves into an empty
/// `NiMemStream` (a 0x24-byte local) through virtual function `+0x10`, writes
/// the end of the written data (`+0x18`) to `*size` and takes the stream's
/// buffer (`00aa2d60`) into `*buffer`. Returns the result of the save.
pub fn ni_stream_save_ov2(e: &mut Engine, this: Ptr<NiStream>, buffer: Ptr, size: Ptr) -> bool {
    e.with_stack(0x24, |e, memory_stream| {
        e.call(NI_MEM_STREAM_CONSTRUCT_EMPTY, &args![memory_stream]);
        let saved = e
            .vcall(this.addr(), SLOT_SAVE, &args![memory_stream])
            .bool();
        let end = e.mem.u32(memory_stream.addr().wrapping_add(0x18));
        e.mem.set_u32(size.addr(), end);
        let data = e
            .call(NI_MEM_STREAM_TAKE_BUFFER, &args![memory_stream])
            .u32();
        e.mem.set_u32(buffer.addr(), data);
        e.call(NI_MEM_STREAM_DESTRUCT, &args![memory_stream]);
        saved
    })
}

// Translated from 00a63630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a63630 - the /GS stack cookie check (00ec408c) is not translated.
/// `NiStream::GetVersionFromString` (Xbox PDB, cdecl): copies the text into
/// a 16-byte local, splits it at dots (`strtok_s`) and packs the numbers
/// into a version, the first in the top byte: "20.2.0.7" is `0x14020007`.
/// The shift count steps down by 8 from 24 and the processor takes it
/// modulo 32, so a fifth number is shifted by 24 again.
pub fn ni_stream_get_version_from_string(e: &mut Engine, text: Ptr) -> u32 {
    e.with_stack(0x14, |e, locals| {
        let copy = locals;
        let context = locals.byte_add(0x10);
        e.call(STRCPY_S, &args![copy, 0x10u32, text]);
        let mut shift: i32 = 0x18;
        let mut version = 0u32;
        let mut token = e.call(STRTOK_S, &args![copy, TEXT_DOT, context]).u32();
        while token != 0 {
            let number = e.call(ATOL, &args![token]).u32();
            version |= number << (shift as u32 & 31);
            shift = shift.wrapping_sub(8);
            token = e.call(STRTOK_S, &args![0u32, TEXT_DOT, context]).u32();
        }
        version
    })
}

// Translated from 00a636b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands out the default path once: while `bUseDefaultPath` is set, copies
/// the stream's file path (`+0x4c0`, at most 0x103 characters) into the
/// 0x104-byte buffer and clears the flag. The second stack word is not
/// read.
pub fn fn_00a636b0(e: &mut Engine, this: Ptr<NiStream>, buffer: Ptr, _unused_2: u32) -> bool {
    if e.mem.u8(USE_DEFAULT_PATH) == 0 {
        return false;
    }
    let path = this.byte_add(NiStream::m_acFilePath.off);
    e.call(STRNCPY_S, &args![buffer, 0x104u32, path, 0x103u32]);
    e.mem.set_u8(USE_DEFAULT_PATH, 0);
    true
}

// Translated from 00a636f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the stream's file path (`+0x4c0`): a non-empty text is copied (at
/// most 0x103 characters) and standardized (`NiPath::Standardize`), a null or
/// empty one empties the path.
pub fn fn_00a636f0(e: &mut Engine, this: Ptr<NiStream>, path: Ptr) {
    let destination = this.byte_add(NiStream::m_acFilePath.off);
    if !path.is_null() && e.mem.u8(path.addr()) != 0 {
        e.call(STRNCPY_S, &args![destination, 0x104u32, path, 0x103u32]);
        e.call(NI_PATH_STANDARDIZE, &args![destination]);
    } else {
        e.mem.set_u8(destination.addr(), 0);
    }
}

// Translated from 00a63730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeObjectArray<NiFixedString>::SetAt` (instance in this unit): keeps
/// the used size (index + 1 when the index is past it) and the count of
/// non-empty elements, takes a reference on the new string and drops the one
/// on the old, then stores the handle at `*value`.
pub fn fn_00a63730(e: &mut Engine, this: Ptr<NiTLargeArray>, index: u32, value: Ptr) {
    let handle = e.mem.u32(value.addr());
    let used = e.get(this, NiTLargeArray::m_uiSize);
    let count = e.get(this, NiTLargeArray::m_uiESize);
    if index >= used {
        e.set(this, NiTLargeArray::m_uiSize, index.wrapping_add(1));
        if handle != empty_handle(e) {
            e.set(this, NiTLargeArray::m_uiESize, count.wrapping_add(1));
        }
    } else {
        let base = e.get(this, NiTLargeArray::m_pBase);
        let old = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        if handle != empty_handle(e) {
            if old == empty_handle(e) {
                e.set(this, NiTLargeArray::m_uiESize, count.wrapping_add(1));
            }
        } else if old != empty_handle(e) {
            e.set(this, NiTLargeArray::m_uiESize, count.wrapping_sub(1));
        }
    }
    let base = e.get(this, NiTLargeArray::m_pBase);
    let slot = base.wrapping_add(index.wrapping_mul(4));
    if e.mem.u32(slot) != handle {
        if handle != empty_handle(e) {
            handle_add_ref(e, handle);
        }
        let old = e.mem.u32(slot);
        if old != empty_handle(e) {
            handle_release(e, old);
        }
        e.mem.set_u32(slot, handle);
    }
}

// Translated from 00a637d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeObjectArray<NiFixedString>::RemoveAll` (instance in this unit):
/// every non-empty element is replaced by the empty string (its reference
/// dropped), and the size and the count of used elements go to 0.
pub fn fn_00a637d0(e: &mut Engine, this: Ptr<NiTLargeArray>) {
    let used = e.get(this, NiTLargeArray::m_uiSize);
    if used != 0 {
        let mut empty = empty_handle(e);
        let mut index = 0u32;
        loop {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let handle = e.mem.u32(slot);
            let temporary = empty;
            if handle != empty {
                handle_release(e, handle);
                e.mem.set_u32(slot, empty);
                empty = empty_handle(e);
                // The temporary `NiFixedString` of the assignment is
                // destroyed against the current empty handle.
                if temporary != empty {
                    handle_release(e, temporary);
                    empty = empty_handle(e);
                }
            }
            index = index.wrapping_add(1);
            if index >= e.get(this, NiTLargeArray::m_uiSize) {
                break;
            }
        }
    }
    e.set(this, NiTLargeArray::m_uiSize, 0);
    e.set(this, NiTLargeArray::m_uiESize, 0);
}

// Translated from 00a63840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hash function of the pointer maps: the key modulo the number of buckets
/// (`+4`). A map without buckets divides by zero, as the processor would
/// trap on.
pub fn fn_00a63840(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32) -> u32 {
    key % e.get(this, NiTPointerMap::m_uiHashSize)
}

/// The body both string-keyed pointer-map destructors share: when the map
/// copies its keys (`+0x10`), frees every node's key (`+4`) bucket by
/// bucket.
fn free_copied_keys(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    if e.mem.u8(this.addr().wrapping_add(MAP_COPY_KEYS)) == 0 {
        return;
    }
    let buckets = e.get(this, NiTPointerMap::m_uiHashSize);
    for bucket in 0..buckets {
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        let mut node = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
        while node != 0 {
            let key = e.mem.u32(node.wrapping_add(4));
            let next = e.mem.u32(node);
            free(e, key);
            node = next;
        }
    }
}

// Translated from 00a63850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `NiTStringTemplateMap<NiTPointerMap<char const*,
/// NiObject*(*)()>, ...>` (the loader map): frees the copied keys, then runs
/// the base destructors one level at a time (each writes its virtual table
/// first, as the compiler does, and removes all nodes through `00b601a0`)
/// and frees the bucket array.
pub fn fn_00a63850(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), LOADER_MAP_BASE_VTABLE_109BCC4);
    free_copied_keys(e, this);
    e.mem.set_u32(this.addr(), LOADER_MAP_MIDDLE_VTABLE_109BCA4);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), LOADER_MAP_ROOT_VTABLE_109BC84);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    free(e, table);
}

// Translated from 00a638c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<char const*, unsigned short>::DeleteItem` (Xbox PDB): gives
/// a node back to the shared pool of twelve-byte nodes. Under the pool lock
/// (`0040fbf0`) the node's value (`u16` at `+8`) and key (`+4`) are cleared and
/// it is pushed on the free list; the lock count goes down and the owner is
/// cleared at zero. The map itself (`ECX`) is not used.
pub fn ni_t_pointer_map_delete_item(e: &mut Engine, _this: Ptr<NiTPointerMap>, item: Ptr) {
    e.mem.set_u16(item.addr().wrapping_add(8), 0);
    e.call(POOL_LOCK_FUNCTION, &args![POOL_LOCK, 0u32]);
    e.mem.set_u32(item.addr().wrapping_add(4), 0);
    let head = e.mem.u32(POOL_FREE_LIST);
    e.mem.set_u32(item.addr(), head);
    let count = e.mem.u32(POOL_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(POOL_LOCK_COUNT, count);
    e.mem.set_u32(POOL_FREE_LIST, item.addr());
    if count == 0 {
        e.mem.set_u32(POOL_LOCK, 0);
    }
}

// Translated from 00a63910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `NiTStringTemplateMap<NiTPointerMap<char const*,
/// unsigned short>, unsigned short>` (the type name map of the save): like
/// [`fn_00a63850`] with the virtual tables of this map.
pub fn fn_00a63910(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), NAME_MAP_BASE_VTABLE_109BD2C);
    free_copied_keys(e, this);
    e.mem.set_u32(this.addr(), NAME_MAP_MIDDLE_VTABLE_109BD0C);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), NAME_MAP_ROOT_VTABLE_109BCEC);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    free(e, table);
}

// Translated from 00a63980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTPointerMap<char const*, unsigned short>,
/// unsigned short>::SetValue` (Xbox PDB): fills a node. A map that copies its
/// keys (`+0x10`) allocates a copy of the key (`00aa1070`, `strcpy_s`); the
/// value is the low 16 bits of the third word.
pub fn ni_t_string_template_map_set_value(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    item: Ptr,
    key: u32,
    value: u16,
) {
    if e.mem.u8(this.addr().wrapping_add(MAP_COPY_KEYS)) != 0 {
        let mut length = 0u32;
        while e.mem.u8(key.wrapping_add(length)) != 0 {
            length += 1;
        }
        let size = length.wrapping_add(1);
        let copy = allocate(e, size);
        e.mem.set_u32(item.addr().wrapping_add(4), copy);
        e.call(STRCPY_S, &args![copy, size, key]);
        e.mem.set_u16(item.addr().wrapping_add(8), value);
        return;
    }
    e.mem.set_u32(item.addr().wrapping_add(4), key);
    e.mem.set_u16(item.addr().wrapping_add(8), value);
}

/// What the scalar deleting destructors below share after their body:
/// frees the object through the memory manager when bit 0 of the flags is
/// set.
fn delete_if_flagged(e: &mut Engine, this: u32, flags: u32) {
    if flags & 1 != 0 {
        manager_deallocate(e, this);
    }
}

// Translated from 00a639e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of a pointer map of the root class: the root
/// virtual table, `RemoveAll`, the bucket array freed, and the object freed
/// when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00a639e0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), LOADER_MAP_ROOT_VTABLE_109BC84);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    free(e, table);
    delete_if_flagged(e, this.addr(), flags);
    this
}

// Translated from 00a63a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the middle class of the loader map: two
/// `RemoveAll` passes under the middle and root virtual tables.
pub fn fn_00a63a20(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), LOADER_MAP_MIDDLE_VTABLE_109BCA4);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), LOADER_MAP_ROOT_VTABLE_109BC84);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    free(e, table);
    delete_if_flagged(e, this.addr(), flags);
    this
}

// Translated from 00a63a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, char const*, unsigned
/// short>::_scalar_deleting_destructor_` (Xbox PDB): the root virtual table
/// of the name map, `RemoveAll`, the bucket array freed, the object freed
/// when bit 0 of `flags` is set.
pub fn ni_t_map_base_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), NAME_MAP_ROOT_VTABLE_109BCEC);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    free(e, table);
    delete_if_flagged(e, this.addr(), flags);
    this
}

// Translated from 00a63aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<char const*, unsigned short>::_scalar_deleting_destructor_`
/// (Xbox PDB): two `RemoveAll` passes under the middle and root virtual
/// tables of the name map, the bucket array freed, the object freed when bit
/// 0 of `flags` is set.
pub fn ni_t_pointer_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), NAME_MAP_MIDDLE_VTABLE_109BD0C);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), NAME_MAP_ROOT_VTABLE_109BCEC);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    free(e, table);
    delete_if_flagged(e, this.addr(), flags);
    this
}

// Translated from 00a63ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates an array of `count` `NiFixedString`s the way `new[]` does: the
/// count in front, the elements constructed (`007613f0`) through the vector
/// constructor iterator. Returns the first element, null when the allocation
/// fails. The byte size saturates to `0xffffffff` on overflow.
pub fn fn_00a63ae0(e: &mut Engine, count: u32) -> u32 {
    let bytes = count.saturating_mul(4);
    let total = bytes.saturating_add(4);
    let block = manager_allocate(e, total);
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    let first = block.wrapping_add(4);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![first, 4u32, count, FIXED_STRING_CONSTRUCTOR],
    );
    first
}

// Translated from 00a63b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the groups array's base class: its virtual
/// table, the element array freed (`+4`), the object freed when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00a63b30(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr<NiTArray> {
    let base = e.get(this, NiTArray::m_pBase);
    e.mem.set_u32(this.addr(), GROUPS_ARRAY_BASE_VTABLE_109BC3C);
    free(e, base);
    delete_if_flagged(e, this.addr(), flags);
    this
}

// Translated from 00a63b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the registration map's base class: its
/// virtual table, `RemoveAll`, the bucket array freed, the object freed when
/// bit 0 of `flags` is set. Returns `this`.
pub fn fn_00a63b60(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), REGISTER_MAP_BASE_VTABLE_109BC64);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    free(e, table);
    delete_if_flagged(e, this.addr(), flags);
    this
}

// Translated from 00a63ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the registration map (the type of
/// `m_kRegisterMap`): two `RemoveAll` passes under its virtual table and its
/// base's, the bucket array freed, the object freed when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_00a63ba0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), REGISTER_MAP_VTABLE_109BD54);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), REGISTER_MAP_BASE_VTABLE_109BC64);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    free(e, table);
    delete_if_flagged(e, this.addr(), flags);
    this
}

// Translated from 00a63be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::Load_ov3` (Xbox PDB), from a file path: stores the standardized
/// path as the stream's file name (`+0xe0`) and, when not empty, as its file
/// path (`+0x4c0`), opens the file (`NiFile::GetFile(name, 0, 0x8000)`) and,
/// when it opens and says it is usable, loads from it through virtual
/// function `+4` and deletes it. Otherwise the error is 1, "Cannot open
/// file.", and the result false.
pub fn ni_stream_load_ov3(e: &mut Engine, this: Ptr<NiStream>, path: Ptr) -> bool {
    let name = this.byte_add(NiStream::m_acFileName.off);
    e.call(STRCPY_S, &args![name, 0x104u32, path]);
    e.call(NI_PATH_STANDARDIZE, &args![name]);
    let file_path = this.byte_add(NiStream::m_acFilePath.off);
    if e.mem.u8(name.addr()) != 0 {
        e.call(STRNCPY_S, &args![file_path, 0x104u32, name, 0x103u32]);
        e.call(NI_PATH_STANDARDIZE, &args![file_path]);
    } else {
        e.mem.set_u8(file_path.addr(), 0);
    }
    let file = e
        .call(NI_FILE_GET_FILE, &args![name, 0u32, 0x8000u32])
        .u32();
    if file != 0 {
        let usable = e.vcall(file, STREAM_SLOT_IS_VALID, &args![]).bool();
        if usable {
            let loaded = e.vcall(this.addr(), SLOT_LOAD, &args![file]).bool();
            e.vcall(file, 0, &args![1u32]);
            return loaded;
        }
        e.vcall(file, 0, &args![1u32]);
    }
    e.set(this, NiStream::m_uiLastError, ERROR_CANNOT_OPEN);
    let message = this.byte_add(NiStream::m_acLastErrorMessage.off);
    e.call(
        STRCPY_S,
        &args![message, ERROR_MESSAGE_SIZE, TEXT_CANNOT_OPEN_FILE],
    );
    false
}

// Translated from 00a63cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks an object and everything below it: the flags word at `+0x30` of
/// the object gets bit 4 cleared and bits 1 to 3 set, then the same for
/// each non-null child of the node the object answers to virtual function
/// `+0xC` (`AsNode`; the child count is the `u16` at `+0xa6`, the children
/// at `+0xa0`). The stream (`ECX`) is only passed on.
pub fn fn_00a63cb0(e: &mut Engine, this: Ptr<NiStream>, object: Ptr) {
    let flags = e.mem.u32(object.addr().wrapping_add(0x30));
    e.mem.set_u32(
        object.addr().wrapping_add(0x30),
        (flags & 0xffff_ffef) | 0xe,
    );
    let node = e.vcall(object.addr(), OBJECT_SLOT_AS_NODE, &args![]).u32();
    if node == 0 {
        return;
    }
    let mut index = 0u32;
    while index < e.mem.u16(node.wrapping_add(0xa6)) as u32 {
        let children = e.mem.u32(node.wrapping_add(0xa0));
        let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
        if child != 0 {
            fn_00a63cb0(e, this, Ptr::new(child));
        }
        index += 1;
    }
}

// Translated from 00a63d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// For files older than version `0x0401000c`, runs [`fn_00a63cb0`] on every
/// loaded object whose class chain (virtual function `+8`, then the base
/// class link at `+4`) reaches the class record at `011f4280` and whose word
/// at `+0x18` is 0.
pub fn fn_00a63d10(e: &mut Engine, this: Ptr<NiStream>) {
    if e.get(this, NiStream::m_uiNifFileVersion) >= 0x0401_000c {
        return;
    }
    let objects = this.at(NiStream::m_kObjects);
    let mut index = 0u32;
    while index < e.get(objects, NiTLargeArray::m_uiSize) {
        let base = e.get(objects, NiTLargeArray::m_pBase);
        let object = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        if object != 0 {
            let mut class = e.vcall(object, OBJECT_SLOT_GET_RTTI, &args![]).u32();
            while class != 0 {
                if class == NODE_CLASS_RECORD {
                    if e.mem.u32(object.wrapping_add(0x18)) == 0 {
                        fn_00a63cb0(e, this, Ptr::new(object));
                    }
                    break;
                }
                class = e.mem.u32(class.wrapping_add(4));
            }
        }
        index += 1;
    }
}

// Translated from 00a63d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::SaveTopLevelObjects` (Xbox PDB): writes the number of top-level
/// objects, then asks the stream (virtual function `+0x30`, `SaveLinkID`) to
/// write the link id of each.
pub fn ni_stream_save_top_level_objects(e: &mut Engine, this: Ptr<NiStream>) {
    let top = this.at(NiStream::m_kTopObjects);
    let count = e.get(top, NiTLargeArray::m_uiSize);
    write_u32(e, this, count);
    for index in 0..count {
        let base = e.get(top, NiTLargeArray::m_pBase);
        let object = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        e.vcall(this.addr(), SLOT_SAVE_LINK_ID, &args![object]);
    }
}

// Translated from 00a63df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::PreSaveObjectSizeTable` (Xbox PDB): remembers the position of
/// the output stream (its virtual function `+0xc`) and writes a zero for each
/// object, room for the table of sizes. Returns the position.
pub fn ni_stream_pre_save_object_size_table(e: &mut Engine, this: Ptr<NiStream>) -> u32 {
    let stream = e.get(this, NiStream::m_pkOstr);
    let position = e.vcall(stream, STREAM_SLOT_GET_POSITION, &args![]).u32();
    let count = e.get(this.at(NiStream::m_kObjects), NiTLargeArray::m_uiSize);
    for _ in 0..count {
        write_u32(e, this, 0);
    }
    position
}

// Translated from 00a63e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::SaveObjectSizeTable` (Xbox PDB): goes back to the position
/// the room was reserved at (virtual function `+8` of the output stream, with
/// the distance from where it is now) and writes the size of each object.
/// Always true.
pub fn ni_stream_save_object_size_table(
    e: &mut Engine,
    this: Ptr<NiStream>,
    position: u32,
) -> bool {
    let stream = e.get(this, NiStream::m_pkOstr);
    let current = e.vcall(stream, STREAM_SLOT_GET_POSITION, &args![]).u32();
    let stream = e.get(this, NiStream::m_pkOstr);
    e.vcall(
        stream,
        STREAM_SLOT_SEEK,
        &args![position.wrapping_sub(current)],
    );
    let sizes = this.at(NiStream::m_kObjectSizes);
    let count = e.get(sizes, NiTLargeArray::m_uiSize);
    for index in 0..count {
        let base = e.get(sizes, NiTLargeArray::m_pBase);
        let size = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        write_u32(e, this, size);
    }
    true
}

// Translated from 00a63ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the table of fixed strings: the count, the length of the longest
/// string, then for each its length and its characters (the length is the
/// word in front of the characters). The loop counters are 16-bit.
pub fn fn_00a63ee0(e: &mut Engine, this: Ptr<NiStream>) {
    let strings = this.at(NiStream::m_kFixedStrings);
    let count = e.get(strings, NiTLargeArray::m_uiSize);
    write_u32(e, this, count);
    let mut longest = 0u32;
    let mut counter = 0u32;
    if e.get(strings, NiTLargeArray::m_uiSize) != 0 {
        let mut index = 0u32;
        loop {
            let base = e.get(strings, NiTLargeArray::m_pBase);
            let handle = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if handle != empty_handle(e) {
                let length = e.mem.u32(handle.wrapping_sub(4));
                if longest < length {
                    longest = length;
                }
            }
            counter = counter.wrapping_add(1);
            index = counter & 0xffff;
            if index >= e.get(strings, NiTLargeArray::m_uiSize) {
                break;
            }
        }
    }
    write_u32(e, this, longest);
    counter = 0;
    if e.get(strings, NiTLargeArray::m_uiSize) != 0 {
        let mut index = 0u32;
        loop {
            let base = e.get(strings, NiTLargeArray::m_pBase);
            let handle = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            let length = if handle == empty_handle(e) {
                0
            } else {
                e.mem.u32(handle.wrapping_sub(4))
            };
            write_u32(e, this, length);
            write_from(e, this, handle, length, 1);
            counter = counter.wrapping_add(1);
            index = counter & 0xffff;
            if index >= e.get(strings, NiTLargeArray::m_uiSize) {
                break;
            }
        }
    }
}

// Translated from 00a64010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the groups: their number (one less than the array holds, since
/// element 0 is unused) and the first word of each group from element 1 on
/// (their sizes).
pub fn fn_00a64010(e: &mut Engine, this: Ptr<NiStream>) {
    let groups = this.at(NiStream::m_kGroups);
    let count = e.get(groups, NiTArray::m_usSize) as u32;
    write_u32(e, this, count.wrapping_sub(1));
    let mut index = 1u32;
    if 1 < e.get(groups, NiTArray::m_usSize) {
        loop {
            let base = e.get(groups, NiTArray::m_pBase);
            let group = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            let first = e.mem.u32(group);
            write_u32(e, this, first);
            index += 1;
            if index >= e.get(groups, NiTArray::m_usSize) as u32 {
                break;
            }
        }
    }
}

// Translated from 00a640b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::Save_ov3` (Xbox PDB), to a file path: like
/// [`ni_stream_load_ov3`] (the file is opened with `NiFile::GetFile(name, 1,
/// 0x8000)` and saved to through virtual function `+0x10`), but a file that
/// cannot be opened only makes it return false, with no error recorded.
pub fn ni_stream_save_ov3(e: &mut Engine, this: Ptr<NiStream>, path: Ptr) -> bool {
    let name = this.byte_add(NiStream::m_acFileName.off);
    e.call(STRCPY_S, &args![name, 0x104u32, path]);
    e.call(NI_PATH_STANDARDIZE, &args![name]);
    let file_path = this.byte_add(NiStream::m_acFilePath.off);
    if e.mem.u8(name.addr()) != 0 {
        e.call(STRNCPY_S, &args![file_path, 0x104u32, name, 0x103u32]);
        e.call(NI_PATH_STANDARDIZE, &args![file_path]);
    } else {
        e.mem.set_u8(file_path.addr(), 0);
    }
    let file = e
        .call(NI_FILE_GET_FILE, &args![name, 1u32, 0x8000u32])
        .u32();
    if file == 0 {
        return false;
    }
    let usable = e.vcall(file, STREAM_SLOT_IS_VALID, &args![]).bool();
    if !usable {
        e.vcall(file, 0, &args![1u32]);
        return false;
    }
    let saved = e.vcall(this.addr(), SLOT_SAVE, &args![file]).bool();
    e.vcall(file, 0, &args![1u32]);
    saved
}

// Translated from 00a64160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a C string: its length (0 for a null pointer), then, when it is
/// not 0, the characters without the terminator.
pub fn fn_00a64160(e: &mut Engine, this: Ptr<NiStream>, string: u32) {
    let mut length = 0u32;
    if string != 0 {
        while e.mem.u8(string.wrapping_add(length)) != 0 {
            length += 1;
        }
    }
    write_u32(e, this, length);
    if length as i32 > 0 {
        write_from(e, this, string, length, 1);
    }
}

// Translated from 00a641f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::SaveFixedString` (Xbox PDB): writes the index of the fixed
/// string at `string` in the table, or four bytes of ones for the empty
/// string.
pub fn ni_stream_save_fixed_string(e: &mut Engine, this: Ptr<NiStream>, string: Ptr) {
    if e.mem.u32(string.addr()) == empty_handle(e) {
        write_from(e, this, NO_STRING_INDEX, 4, 4);
        return;
    }
    let index = fn_00a633f0(e, this, string);
    write_u32(e, this, index);
}

// Translated from 00a64250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<NiPointer<NiObject>>::RemoveAll` (instance in this unit):
/// drops the array's reference on every object (deleting those whose count
/// reaches 0), clears the slots, and sets the count of used elements and the
/// size to 0.
pub fn fn_00a64250(e: &mut Engine, this: Ptr<NiTLargeArray>) {
    let mut index = 0u32;
    while index < e.get(this, NiTLargeArray::m_uiSize) {
        let base = e.get(this, NiTLargeArray::m_pBase);
        let slot = base.wrapping_add(index.wrapping_mul(4));
        let object = e.mem.u32(slot);
        if object != 0 {
            release(e, object);
            e.mem.set_u32(slot, 0);
        }
        index += 1;
    }
    e.set(this, NiTLargeArray::m_uiESize, 0);
    e.set(this, NiTLargeArray::m_uiSize, 0);
}

// Translated from 00a643e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, char const*, unsigned
/// short>::SetAt` (Xbox PDB): sets the value of a key. The hash and the
/// comparison are virtual functions of the map (`+4`, `+8`); an existing node
/// keeps its key when the map copies keys, otherwise gets the new one. A new
/// node comes from the pool (`0043a010` on the map's allocator at `+0xc`),
/// is filled by `SetValue` (`+0xc`) and put at the head of its bucket.
pub fn fn_00a643e0(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, value: u32) {
    let hash = e.vcall(this.addr(), MAP_SLOT_HASH, &args![key]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut node = e.mem.u32(table.wrapping_add(hash.wrapping_mul(4)));
    while node != 0 {
        let node_key = e.mem.u32(node.wrapping_add(4));
        let equal = e
            .vcall(this.addr(), MAP_SLOT_KEYS_EQUAL, &args![key, node_key])
            .bool();
        if equal {
            if e.mem.u8(this.addr().wrapping_add(MAP_COPY_KEYS)) == 0 {
                e.mem.set_u32(node.wrapping_add(4), key);
            }
            e.mem.set_u16(node.wrapping_add(8), value as u16);
            return;
        }
        node = e.mem.u32(node);
    }
    let allocator = this.addr().wrapping_add(0xc);
    let item = e.call(POOL_ALLOCATE_NODE, &args![allocator]).u32();
    e.vcall(this.addr(), MAP_SLOT_SET_VALUE, &args![item, key, value]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let head = e.mem.u32(table.wrapping_add(hash.wrapping_mul(4)));
    e.mem.set_u32(item, head);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.mem
        .set_u32(table.wrapping_add(hash.wrapping_mul(4)), item);
    let count = e.get(this, NiTPointerMap::m_uiCount);
    e.set(this, NiTPointerMap::m_uiCount, count.wrapping_add(1));
}

/// Enters the stream clean-up critical section.
fn enter_cleanup(e: &mut Engine) {
    e.call(ENTER_CRITICAL_SECTION, &args![CLEANUP_CRITICAL_SECTION]);
}

/// Leaves the stream clean-up critical section.
fn leave_cleanup(e: &mut Engine) {
    e.call(LEAVE_CRITICAL_SECTION, &args![CLEANUP_CRITICAL_SECTION]);
}

// Translated from 00a644f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::RemoveAllObjects` (Xbox PDB): under the clean-up critical
/// section, empties the top-level objects, the fixed strings and the sizes.
pub fn ni_stream_remove_all_objects(e: &mut Engine, this: Ptr<NiStream>) {
    enter_cleanup(e);
    fn_00a64250(e, this.at(NiStream::m_kTopObjects));
    fn_00a637d0(e, this.at(NiStream::m_kFixedStrings));
    let sizes = this.at(NiStream::m_kObjectSizes);
    let count = e.get(sizes, NiTLargeArray::m_uiSize);
    for index in 0..count {
        let base = e.get(sizes, NiTLargeArray::m_pBase);
        e.mem.set_u32(base.wrapping_add(index.wrapping_mul(4)), 0);
    }
    e.set(sizes, NiTLargeArray::m_uiSize, 0);
    e.set(sizes, NiTLargeArray::m_uiESize, 0);
    leave_cleanup(e);
}

// Translated from 00a64550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::ChangeObject` (Xbox PDB): puts an object (counted) in the
/// slot of the objects array that is being loaded (`m_uiLoad`).
pub fn ni_stream_change_object(e: &mut Engine, this: Ptr<NiStream>, object: u32) {
    add_ref(e, object);
    let index = e.get(this, NiStream::m_uiLoad);
    e.with_stack(4, |e, holder| {
        e.mem.set_u32(holder.addr(), object);
        let objects = this.at(NiStream::m_kObjects);
        e.call(LARGE_ARRAY_SET_AT, &args![objects, index, holder]);
    });
    release(e, object);
}

// Translated from 00a645b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::ReadLinkID` (Xbox PDB): reads a link id and adds it to the
/// set of link ids.
pub fn ni_stream_read_link_id(e: &mut Engine, this: Ptr<NiStream>) {
    let id = read_u32(e, this);
    set_add(e, this.at(NiStream::m_kLinkIDs), id);
}

// Translated from 00a64620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::ResolveLinkID` (Xbox PDB): reads a link id and returns the
/// object it names, null for the id `0xffffffff`.
pub fn ni_stream_resolve_link_id(e: &mut Engine, this: Ptr<NiStream>) -> u32 {
    let id = read_u32(e, this);
    if id == u32::MAX {
        return 0;
    }
    let base = e.get(this.at(NiStream::m_kObjects), NiTLargeArray::m_pBase);
    e.mem.u32(base.wrapping_add(id.wrapping_mul(4)))
}

// Translated from 00a64670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::ReadMultipleLinkIDs` (Xbox PDB): reads a count, records it
/// in the set of block sizes, and reads that many link ids. Answers the
/// count (`NiFlipController::LoadBinary`, `00a4f6b3`, keeps it).
// 00a646f5 RET: EAX is the count local on both paths (00a646d4, 00a646e7).
pub fn ni_stream_read_multiple_link_ids(e: &mut Engine, this: Ptr<NiStream>) -> u32 {
    let count = read_u32(e, this);
    set_add(e, this.at(NiStream::m_kLinkIDBlocks), count);
    for _ in 0..count {
        ni_stream_read_link_id(e, this);
    }
    count // 00a646d4/00a646e7: MOV EAX,[ESP+8] (the count), left in EAX at 00a646f5.
}

// Translated from 00a64700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::SetNumberOfLinkIDs` (Xbox PDB): adds a block size to the
/// set of block sizes.
pub fn ni_stream_set_number_of_link_ids(e: &mut Engine, this: Ptr<NiStream>, count: u32) {
    set_add(e, this.at(NiStream::m_kLinkIDBlocks), count);
}

// Translated from 00a64740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::LoadObjectSizeTable` (Xbox PDB): sizes the table to the
/// number of objects, then reads one size per object. Always true.
pub fn ni_stream_load_object_size_table(e: &mut Engine, this: Ptr<NiStream>) -> bool {
    let sizes = this.at(NiStream::m_kObjectSizes);
    let objects = this.at(NiStream::m_kObjects);
    let count = e.get(objects, NiTLargeArray::m_uiSize);
    e.call(SIZES_ARRAY_SET_SIZE, &args![sizes, count]);
    let mut index = 0u32;
    while index < e.get(objects, NiTLargeArray::m_uiSize) {
        let size = read_u32(e, this);
        sizes_set_at(e, sizes, index, size);
        index += 1;
    }
    true
}

// Translated from 00a647f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the groups: the number (one more is allocated, element 0 stays
/// empty), then for each one its size, from which an `NiObjectGroup` is made
/// (`00a8a620` on 0x10 bytes from `operator new`) and put in the groups
/// array, keeping the array's size and count of used elements.
pub fn fn_00a647f0(e: &mut Engine, this: Ptr<NiStream>) {
    let groups = this.at(NiStream::m_kGroups);
    let count = read_u32(e, this).wrapping_add(1);
    e.call(GROUPS_SET_SIZE, &args![groups, count]);
    e.with_stack(4, |e, zero| {
        e.call(GROUPS_SET_AT, &args![groups, 0u32, zero]);
    });
    let mut index = 1u32;
    while index < count {
        let size = read_u32(e, this);
        let memory = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let group = if memory == 0 {
            0
        } else {
            e.call(OBJECT_GROUP_CONSTRUCT, &args![memory, size]).u32()
        };
        if index < e.get(groups, NiTArray::m_usSize) as u32 {
            let base = e.get(groups, NiTArray::m_pBase);
            let old = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            let used = e.get(groups, NiTArray::m_usESize);
            if group == 0 {
                if old != 0 {
                    e.set(groups, NiTArray::m_usESize, used.wrapping_sub(1));
                }
            } else if old == 0 {
                e.set(groups, NiTArray::m_usESize, used.wrapping_add(1));
            }
        } else {
            e.set(groups, NiTArray::m_usSize, (index as u16).wrapping_add(1));
            if group != 0 {
                let used = e.get(groups, NiTArray::m_usESize);
                e.set(groups, NiTArray::m_usESize, used.wrapping_add(1));
            }
        }
        let base = e.get(groups, NiTArray::m_pBase);
        e.mem
            .set_u32(base.wrapping_add(index.wrapping_mul(4)), group);
        index += 1;
    }
}

// Translated from 00a64900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::RegisterLoader` (Xbox PDB, cdecl): adds a class name and its
/// creation function to `ms_pkLoaders`. Returns 0.
pub fn ni_stream_register_loader(e: &mut Engine, name: u32, creator: u32) -> u32 {
    let loaders = e.mem.u32(LOADERS);
    e.call(LOADER_MAP_SET_AT, &args![loaders, name, creator]);
    0
}

// Translated from 00a64920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::RegisterPostProcessFunction` (Xbox PDB, cdecl): adds a
/// function to the post process array; the result is what the array's `Add`
/// returns (the index it put the function at).
pub fn ni_stream_register_post_process_function(e: &mut Engine, function: u32) -> u32 {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), function);
        let array = e.mem.u32(POST_PROCESS_FUNCTIONS);
        e.call(POST_PROCESS_ADD, &args![array, local]).u32()
    })
}

// Translated from 00a64940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::SaveHeader` (Xbox PDB): writes the header text, the file
/// version, the endian byte, the user defined version, the number of
/// objects and, when the user defined version is not 0, the Bethesda header
/// (version and the three length-prefixed strings). The output stream's
/// endian swap is set off for the header and on again at the end for a stream
/// that is not little endian.
pub fn ni_stream_save_header(e: &mut Engine, this: Ptr<NiStream>) {
    let stream = e.get(this, NiStream::m_pkOstr);
    e.vcall(stream, STREAM_SLOT_SET_ENDIAN_SWAP, &args![0u32]);
    let stream = e.get(this, NiStream::m_pkOstr);
    e.call(WRITE_STRING, &args![stream, TEXT_HEADER_LINE]);
    write_from(e, this, NIF_MAX_VERSION, 4, 4);
    let endian = this.addr().wrapping_add(NiStream::m_bSaveLittleEndian.off);
    write_from(e, this, endian, 1, 1);
    write_from(e, this, NIF_MAX_USER_VERSION, 4, 4);
    let count = e.get(this.at(NiStream::m_kObjects), NiTLargeArray::m_uiSize);
    write_u32(e, this, count);
    if e.mem.u32(NIF_MAX_USER_VERSION) != 0 {
        let version = this.addr().wrapping_add(NiStream::m_BSStreamHeader.off);
        write_from(e, this, version, 4, 4);
        for field in [
            NiStream::pAuthor.off,
            NiStream::pProcessScript.off,
            NiStream::pExportScript.off,
        ] {
            let text = this.addr().wrapping_add(field);
            let mut length = 0u32;
            while e.mem.u8(text.wrapping_add(length)) != 0 {
                length += 1;
            }
            let prefixed = (length as u8).wrapping_add(1);
            e.with_stack(4, |e, local| {
                e.mem.set_u8(local.addr(), prefixed);
                write_from(e, this, local.addr(), 1, 1);
            });
            write_from(e, this, text, prefixed as u32, 1);
        }
    }
    let swap = (e.mem.u8(endian) ^ 1) as u32;
    let stream = e.get(this, NiStream::m_pkOstr);
    e.vcall(stream, STREAM_SLOT_SET_ENDIAN_SWAP, &args![swap]);
}

// Translated from 00a64b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Prepares the groups for a save: the array is sized to 1 with a null
/// element 0; then each distinct group of an object (virtual function
/// `+0x80`) not already in the array at an index other than 0 is appended
/// with its first word cleared; then each object adds its allocation size
/// (virtual function `+0x7c`) to its group's first word.
pub fn fn_00a64b80(e: &mut Engine, this: Ptr<NiStream>) {
    let groups = this.at(NiStream::m_kGroups);
    e.call(GROUPS_SET_SIZE, &args![groups, 1u32]);
    e.with_stack(4, |e, zero| {
        e.call(GROUPS_SET_AT, &args![groups, 0u32, zero]);
    });
    let objects = this.at(NiStream::m_kObjects);
    let mut index = 0u32;
    if e.get(objects, NiTLargeArray::m_uiSize) != 0 {
        loop {
            let base = e.get(objects, NiTLargeArray::m_pBase);
            let object = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if object != 0 {
                let group = e.vcall(object, OBJECT_SLOT_GET_GROUP, &args![]).u32();
                if group != 0 {
                    let mut known = false;
                    if e.get(groups, NiTArray::m_usSize) != 0 {
                        let mut at = 0u32;
                        let table = e.get(groups, NiTArray::m_pBase);
                        loop {
                            if group == e.mem.u32(table.wrapping_add(at.wrapping_mul(4))) {
                                known = at != 0;
                                break;
                            }
                            at += 1;
                            if at >= e.get(groups, NiTArray::m_usSize) as u32 {
                                break;
                            }
                        }
                    }
                    if !known {
                        let size = e.get(groups, NiTArray::m_usSize) as u32;
                        if e.get(groups, NiTArray::m_usMaxSize) as u32 <= size {
                            let grow = e.get(groups, NiTArray::m_usGrowBy) as u32;
                            e.call(GROUPS_SET_SIZE, &args![groups, grow.wrapping_add(size)]);
                        }
                        e.with_stack(4, |e, holder| {
                            e.mem.set_u32(holder.addr(), group);
                            e.call(GROUPS_SET_AT, &args![groups, size, holder]);
                        });
                        e.mem.set_u32(group, 0);
                    }
                }
            }
            index += 1;
            if index >= e.get(objects, NiTLargeArray::m_uiSize) {
                break;
            }
        }
    }
    let mut index = 0u32;
    if e.get(objects, NiTLargeArray::m_uiSize) != 0 {
        loop {
            let base = e.get(objects, NiTLargeArray::m_pBase);
            let object = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if object != 0 {
                let group = e.vcall(object, OBJECT_SLOT_GET_GROUP, &args![]).u32();
                if group != 0 {
                    let before = e.mem.u32(group);
                    let size = e
                        .vcall(object, OBJECT_SLOT_GET_BLOCK_ALLOCATION_SIZE, &args![])
                        .u32();
                    e.mem.set_u32(group, size.wrapping_add(before));
                }
            }
            index += 1;
            if index >= e.get(objects, NiTLargeArray::m_uiSize) {
                break;
            }
        }
    }
}

// Translated from 00a64ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::FreeLoadData` (Xbox PDB): under the clean-up critical
/// section, empties the objects array and resets the sets of link ids and
/// block sizes and their indices.
pub fn ni_stream_free_load_data(e: &mut Engine, this: Ptr<NiStream>) {
    enter_cleanup(e);
    fn_00a64250(e, this.at(NiStream::m_kObjects));
    reset_link_data(e, this);
    leave_cleanup(e);
}

/// Sets the used counts of the two sets of link data and both link indices
/// to 0.
fn reset_link_data(e: &mut Engine, this: Ptr<NiStream>) {
    e.set(this.at(NiStream::m_kLinkIDs), NiTPrimitiveSet::m_uiUsed, 0);
    e.set(
        this.at(NiStream::m_kLinkIDBlocks),
        NiTPrimitiveSet::m_uiUsed,
        0,
    );
    e.set(this, NiStream::m_uiLinkIndex, 0);
    e.set(this, NiStream::m_uiLinkBlockIndex, 0);
}

// Translated from 00a64ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::LoadCString` (Xbox PDB): reads a length and, when it is
/// positive, allocates that many characters plus the terminator
/// (`00aa1070`), stores the pointer at `*out` and reads the characters;
/// otherwise stores null.
pub fn ni_stream_load_c_string(e: &mut Engine, this: Ptr<NiStream>, out: Ptr) {
    let length = read_u32(e, this) as i32;
    if length > 0 {
        let text = allocate(e, (length as u32).wrapping_add(1));
        e.mem.set_u32(out.addr(), text);
        read_into(e, this, text, length as u32, 1);
        e.mem.set_u8(text.wrapping_add(length as u32), 0);
    } else {
        e.mem.set_u32(out.addr(), 0);
    }
}

// Translated from 00a64d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads a length and that many characters into `destination`, then the
/// terminator. The length is not checked against the buffer.
pub fn fn_00a64d80(e: &mut Engine, this: Ptr<NiStream>, destination: u32) {
    let length = read_u32(e, this);
    read_into(e, this, destination, length, 1);
    e.mem.set_u8(destination.wrapping_add(length), 0);
}

/// Replaces the handle at `*slot` with the pooled string of `text`
/// (`00a5b690`; null for a null `text`) and drops the reference on the old
/// one unless it is the empty string.
fn replace_with_pooled(e: &mut Engine, slot: u32, old: u32, text: u32) {
    let handle = e.call(FIXED_STRING_FROM_CSTRING, &args![text]).u32();
    e.mem.set_u32(slot, handle);
    if old != empty_handle(e) {
        handle_release(e, old);
    }
}

// Translated from 00a64df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::LoadFixedString` (Xbox PDB): reads an index into the table of
/// fixed strings and stores that string at `*out` (references kept); the
/// index `0xffffffff` stores the empty string (nothing happens when `*out`
/// is already 0).
pub fn ni_stream_load_fixed_string(e: &mut Engine, this: Ptr<NiStream>, out: Ptr) {
    let index = read_u32(e, this);
    if index == u32::MAX {
        let old = e.mem.u32(out.addr());
        if old != 0 {
            replace_with_pooled(e, out.addr(), old, 0);
        }
        return;
    }
    let base = e.get(this.at(NiStream::m_kFixedStrings), NiTLargeArray::m_pBase);
    let handle = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
    if e.mem.u32(out.addr()) != handle {
        if handle != empty_handle(e) {
            handle_add_ref(e, handle);
        }
        let current = e.mem.u32(out.addr());
        if current != empty_handle(e) {
            handle_release(e, current);
        }
        e.mem.set_u32(out.addr(), handle);
    }
}

// Translated from 00a64ea0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a64ea0 - the /GS stack cookie check (00ec408c) is not translated.
/// `NiStream::LoadCStringAsFixedString` (Xbox PDB): reads a length and that
/// many characters, and stores the pooled string of them at `*out`. Up to
/// 1023 characters are read into a local buffer, longer ones into an
/// allocation; a length of 0 or less stores the pooled null string (only when
/// `*out` is not already null). Nothing changes when `*out` already is the
/// buffer's address.
pub fn ni_stream_load_c_string_as_fixed_string(e: &mut Engine, this: Ptr<NiStream>, out: Ptr) {
    let length = read_u32(e, this) as i32;
    if length >= 0x400 {
        let buffer = allocate(e, (length as u32).wrapping_add(1));
        read_into(e, this, buffer, length as u32, 1);
        e.mem.set_u8(buffer.wrapping_add(length as u32), 0);
        let old = e.mem.u32(out.addr());
        if old != buffer {
            replace_with_pooled(e, out.addr(), old, buffer);
        }
        free(e, buffer);
    } else if length > 0 {
        e.with_stack(0x400, |e, local| {
            read_into(e, this, local.addr(), length as u32, 1);
            e.mem.set_u8(local.addr().wrapping_add(length as u32), 0);
            let old = e.mem.u32(out.addr());
            if old != local.addr() {
                replace_with_pooled(e, out.addr(), old, local.addr());
            }
        });
    } else {
        let old = e.mem.u32(out.addr());
        if old != 0 {
            replace_with_pooled(e, out.addr(), old, 0);
        }
    }
}

// Translated from 00a65140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeObjectArray<NiFixedString>::SetSize` (instance in this unit):
/// does nothing for the capacity it has. Shrinking first empties the slots
/// from `new_size` up (counted references dropped, count of used elements
/// down). The elements are then moved to a new array of `new_size` strings
/// (`00a63ae0`; none for 0), and the old array is destroyed from its last
/// element and freed through the memory manager.
pub fn fn_00a65140(e: &mut Engine, this: Ptr<NiTLargeArray>, new_size: u32) {
    if new_size == e.get(this, NiTLargeArray::m_uiMaxSize) {
        return;
    }
    if new_size < e.get(this, NiTLargeArray::m_uiSize) {
        let mut index = new_size;
        loop {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let empty = empty_handle(e);
            if e.mem.u32(slot) != empty {
                let old = e.mem.u32(slot);
                handle_release(e, old);
                e.mem.set_u32(slot, empty);
                if empty != empty_handle(e) {
                    handle_release(e, empty);
                }
                let count = e.get(this, NiTLargeArray::m_uiESize);
                e.set(this, NiTLargeArray::m_uiESize, count.wrapping_sub(1));
            }
            index += 1;
            if index >= e.get(this, NiTLargeArray::m_uiSize) {
                break;
            }
        }
        e.set(this, NiTLargeArray::m_uiSize, new_size);
    }
    let old_base = e.get(this, NiTLargeArray::m_pBase);
    e.set(this, NiTLargeArray::m_uiMaxSize, new_size);
    if new_size == 0 {
        e.set(this, NiTLargeArray::m_pBase, 0);
    } else {
        let created = fn_00a63ae0(e, new_size);
        e.set(this, NiTLargeArray::m_pBase, created);
        let mut index = 0u32;
        if e.get(this, NiTLargeArray::m_uiSize) != 0 {
            loop {
                let value = e.mem.u32(old_base.wrapping_add(index.wrapping_mul(4)));
                let base = e.get(this, NiTLargeArray::m_pBase);
                let slot = base.wrapping_add(index.wrapping_mul(4));
                if e.mem.u32(slot) != value {
                    if value != empty_handle(e) {
                        handle_add_ref(e, value);
                    }
                    let current = e.mem.u32(slot);
                    if current != empty_handle(e) {
                        handle_release(e, current);
                    }
                    e.mem.set_u32(slot, value);
                }
                index += 1;
                if index >= e.get(this, NiTLargeArray::m_uiSize) {
                    break;
                }
            }
        }
        let mut index = e.get(this, NiTLargeArray::m_uiSize);
        while index < e.get(this, NiTLargeArray::m_uiMaxSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let empty = empty_handle(e);
            let current = e.mem.u32(slot);
            if current != empty {
                handle_release(e, current);
                e.mem.set_u32(slot, empty);
                if empty != empty_handle(e) {
                    handle_release(e, empty);
                }
            }
            index += 1;
        }
    }
    if old_base != 0 {
        let count = e.mem.u32(old_base.wrapping_sub(4)) as i32;
        let mut element = count;
        while element > 0 {
            element -= 1;
            let handle = e
                .mem
                .u32(old_base.wrapping_add((element as u32).wrapping_mul(4)));
            if handle != empty_handle(e) {
                handle_release(e, handle);
            }
        }
        manager_deallocate(e, old_base.wrapping_sub(4));
    }
}

// Translated from 00a652d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the object arrays (`m_kObjects`,
/// `m_kTopObjects`): its virtual table, the storage freed (`00a47600` on the
/// element pointer), the object freed when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_00a652d0(e: &mut Engine, this: Ptr<NiTLargeArray>, flags: u32) -> Ptr<NiTLargeArray> {
    e.mem.set_u32(this.addr(), OBJECT_ARRAY_BASE_VTABLE_109BD98);
    let base = e.get(this, NiTLargeArray::m_pBase);
    e.call(OBJECT_ARRAY_FREE_STORAGE, &args![base]);
    delete_if_flagged(e, this.addr(), flags);
    this
}

// Translated from 00a65300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::~NiStream` (Xbox PDB) body: deletes the input and output
/// streams and the background load procedure it holds (virtual function 0
/// with 1), removes all objects, and destroys the members in reverse order:
/// the registration map, the sets of link data, the fixed strings, the top
/// objects, the sizes, the objects and the groups.
pub fn ni_stream_destructor(e: &mut Engine, this: Ptr<NiStream>) {
    e.mem.set_u32(this.addr(), NI_STREAM_VTABLE);
    for member in [NiStream::m_pkIstr, NiStream::m_pkOstr] {
        let stream = e.get(this, member);
        if stream != 0 {
            e.vcall(stream, 0, &args![1u32]);
        }
    }
    ni_stream_remove_all_objects(e, this);
    let procedure = e.get(this, NiStream::m_pkBGLoadProc);
    if procedure != 0 {
        e.vcall(procedure, 0, &args![1u32]);
    }
    let map = this.at(NiStream::m_kRegisterMap);
    e.mem.set_u32(map.addr(), REGISTER_MAP_VTABLE_109BD54);
    e.call(MAP_REMOVE_ALL, &args![map]);
    e.mem.set_u32(map.addr(), REGISTER_MAP_BASE_VTABLE_109BC64);
    e.call(MAP_REMOVE_ALL, &args![map]);
    let table = e.get(map, NiTPointerMap::m_ppkHashTable);
    free(e, table);
    let blocks = e.get(this.at(NiStream::m_kLinkIDBlocks), NiTPrimitiveSet::m_pBase);
    free(e, blocks);
    let ids = e.get(this.at(NiStream::m_kLinkIDs), NiTPrimitiveSet::m_pBase);
    free(e, ids);
    let strings = this.at(NiStream::m_kFixedStrings);
    e.mem
        .set_u32(strings.addr(), FIXED_STRINGS_ARRAY_BASE_VTABLE_109BD4C);
    let base = e.get(strings, NiTLargeArray::m_pBase);
    e.call(FIXED_STRINGS_FREE_STORAGE, &args![base]);
    let top = this.at(NiStream::m_kTopObjects);
    e.mem.set_u32(top.addr(), OBJECT_ARRAY_BASE_VTABLE_109BD98);
    let base = e.get(top, NiTLargeArray::m_pBase);
    e.call(OBJECT_ARRAY_FREE_STORAGE, &args![base]);
    let sizes = this.at(NiStream::m_kObjectSizes);
    e.mem.set_u32(sizes.addr(), SIZES_ARRAY_BASE_VTABLE_1081FC8);
    let base = e.get(sizes, NiTLargeArray::m_pBase);
    free(e, base);
    let objects = this.at(NiStream::m_kObjects);
    e.mem
        .set_u32(objects.addr(), OBJECT_ARRAY_BASE_VTABLE_109BD98);
    let base = e.get(objects, NiTLargeArray::m_pBase);
    e.call(OBJECT_ARRAY_FREE_STORAGE, &args![base]);
    let groups = this.at(NiStream::m_kGroups);
    e.mem
        .set_u32(groups.addr(), GROUPS_ARRAY_BASE_VTABLE_109BC3C);
    let base = e.get(groups, NiTArray::m_pBase);
    free(e, base);
}

// Translated from 00a65400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a65400 - the /GS stack cookie check (00ec408c) is not translated.
/// `NiStream::LoadHeader` (Xbox PDB): reads the header line (up to 0x80
/// characters, `00aa1f10`) and requires "File Format" in it, then the
/// version, checking it against the supported range (errors 2 to 4), the
/// endian byte (error 6), the user defined version (errors 3 and 4), the
/// number of objects (sizes the objects array) and, for the versions that
/// have it, the Bethesda header (a version and three strings; error 3 when
/// that version is below 0x15). Finally sets the input stream's endian swap
/// from the endian byte. Returns false after an error.
pub fn ni_stream_load_header(e: &mut Engine, this: Ptr<NiStream>) -> bool {
    e.with_stack(0x84, |e, line| {
        let stream = e.get(this, NiStream::m_pkIstr);
        e.call(READ_LINE, &args![stream, line, 0x80u32]);
        let found = e.call(STRSTR, &args![line, TEXT_FILE_FORMAT]).u32();
        if found == 0 {
            set_error(e, this, ERROR_NOT_A_NIF_FILE, STRCPY_S, TEXT_NOT_A_NIF_FILE);
            return false;
        }
        header_after_banner(e, this)
    })
}

/// The rest of [`ni_stream_load_header`], after the banner line is checked.
fn header_after_banner(e: &mut Engine, this: Ptr<NiStream>) -> bool {
    let stream = e.get(this, NiStream::m_pkIstr);
    e.vcall(stream, STREAM_SLOT_SET_ENDIAN_SWAP, &args![0u32]);
    let version_at = this.addr().wrapping_add(NiStream::m_uiNifFileVersion.off);
    read_into(e, this, version_at, 4, 4);
    let version = e.get(this, NiStream::m_uiNifFileVersion);
    if version < e.mem.u32(NIF_MIN_VERSION) {
        set_error(
            e,
            this,
            ERROR_VERSION_TOO_OLD,
            STRCPY_S,
            TEXT_VERSION_TOO_OLD,
        );
        return false;
    }
    if e.mem.u32(NIF_MAX_VERSION) < version {
        set_error(
            e,
            this,
            ERROR_UNKNOWN_VERSION,
            STRCPY_S,
            TEXT_UNKNOWN_VERSION,
        );
        return false;
    }
    let endian = this
        .addr()
        .wrapping_add(NiStream::m_bSourceIsLittleEndian.off);
    e.mem.set_u8(endian, 1);
    if version >= 0x1400_0003 {
        e.call(LOAD_BINARY_U8, &args![this, endian]);
    }
    if e.mem.u8(endian) != 1 && e.call(ENDIAN_FLAG_FUNCTION, &args![]).bool() {
        set_error(
            e,
            this,
            ERROR_ENDIAN_MISMATCH,
            STRCPY_S_FORWARD,
            TEXT_ENDIAN_MISMATCH,
        );
        return false;
    }
    let user_at = this
        .addr()
        .wrapping_add(NiStream::m_uiNifFileUserDefinedVersion.off);
    if e.get(this, NiStream::m_uiNifFileVersion) >= 0x0a00_0108 {
        e.call(LOAD_BINARY_U32, &args![this, user_at]);
        if e.get(this, NiStream::m_uiNifFileVersion) == 0x0a00_0165 {
            e.mem.set_u32(user_at, 1);
        }
    }
    let user = e.mem.u32(user_at);
    if user < e.mem.u32(NIF_MIN_USER_VERSION) {
        set_error(
            e,
            this,
            ERROR_VERSION_TOO_OLD,
            STRCPY_S,
            TEXT_USER_VERSION_TOO_OLD,
        );
        return false;
    }
    if e.mem.u32(NIF_MAX_USER_VERSION) < user {
        set_error(
            e,
            this,
            ERROR_UNKNOWN_VERSION,
            STRCPY_S_FORWARD,
            TEXT_UNKNOWN_USER_VERSION,
        );
        return false;
    }
    let objects = e.with_stack(4, |e, count| {
        e.call(LOAD_BINARY_U32, &args![this, count]);
        e.mem.u32(count.addr())
    });
    e.call(
        LARGE_ARRAY_SET_SIZE,
        &args![this.at(NiStream::m_kObjects), objects],
    );
    let version = e.get(this, NiStream::m_uiNifFileVersion);
    e.set(this, NiStream::m_BSStreamHeader, 0);
    let user = e.get(this, NiStream::m_uiNifFileUserDefinedVersion);
    if version > 0x0a00_0100 && (version <= 0x0a00_0108 || user != 0) {
        if version > 0x0a00_0101 {
            let at = this.addr().wrapping_add(NiStream::m_BSStreamHeader.off);
            e.call(LOAD_BINARY_U32, &args![this, at]);
            if e.get(this, NiStream::m_uiNifFileVersion) == 0x0a00_0102 {
                e.set(this, NiStream::m_uiNifFileVersion, 0x0a00_0101);
            }
        }
        for field in [
            NiStream::pAuthor.off,
            NiStream::pProcessScript.off,
            NiStream::pExportScript.off,
        ] {
            e.with_stack(4, |e, length| {
                e.call(LOAD_BINARY_U8, &args![this, length]);
                let length = e.mem.u8(length.addr()) as u32;
                let destination = this.addr().wrapping_add(field);
                e.call(LOAD_BINARY_BYTES, &args![this, destination, length]);
            });
        }
        if e.get(this, NiStream::m_BSStreamHeader) < 0x15 {
            set_error(
                e,
                this,
                ERROR_VERSION_TOO_OLD,
                STRCPY_S_FORWARD,
                TEXT_BS_VERSION_TOO_OLD,
            );
            return false;
        }
    }
    let swap = (e.mem.u8(endian) != 1) as u32;
    let stream = e.get(this, NiStream::m_pkIstr);
    e.vcall(stream, STREAM_SLOT_SET_ENDIAN_SWAP, &args![swap]);
    true
}

// Translated from 00a65720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::LoadTopLevelObjects` (Xbox PDB): reads the number of top-level
/// objects, sizes the array and puts in it, for each id read, the object it
/// names (null for the id `0xffffffff`), counted.
pub fn ni_stream_load_top_level_objects(e: &mut Engine, this: Ptr<NiStream>) {
    let top = this.at(NiStream::m_kTopObjects);
    let count = read_u32(e, this);
    e.call(LARGE_ARRAY_SET_SIZE, &args![top, count]);
    for index in 0..count {
        let id = read_u32(e, this);
        let object = if id == u32::MAX {
            0
        } else {
            let base = e.get(this.at(NiStream::m_kObjects), NiTLargeArray::m_pBase);
            e.mem.u32(base.wrapping_add(id.wrapping_mul(4)))
        };
        add_ref(e, object);
        e.with_stack(4, |e, holder| {
            e.mem.set_u32(holder.addr(), object);
            e.call(LARGE_ARRAY_SET_AT, &args![top, index, holder]);
        });
        release(e, object);
    }
}

// Translated from 00a65800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a65800 - the /GS stack cookie check (00ec408c) is not translated.
/// `NiStream::LoadRTTI` (Xbox PDB): reads the table of class names and, for
/// each, looks up its creation function in `ms_pkLoaders` (an unknown class
/// records error 5 and a null function; "bhkConvexListShape" is always
/// null; files older than 20.1.0.1 with "NiControllerManager" fail with error
/// 3). Then, for each object of the file, reads its class index (the high bit
/// marks a placeholder from version 20.2.0.5 on), creates the object with
/// that class's function and stores it, counted, in the objects array. A
/// null function fails the load unless the object is a placeholder, which
/// stores null.
pub fn ni_stream_load_rtti(e: &mut Engine, this: Ptr<NiStream>) -> bool {
    let count = read_u16(e, this) as u32;
    let table = allocate(e, count.wrapping_mul(4));
    let mut index = 0u32;
    while index < count {
        let length = read_u32(e, this);
        let loaded = e.with_stack(0x100 + 0x100, |e, name| {
            read_into(e, this, name.addr(), length, 1);
            e.mem.set_u8(name.addr().wrapping_add(length), 0);
            let version = e.get(this, NiStream::m_uiNifFileVersion);
            let text = e.mem.cstr(name.addr());
            if version < 0x1401_0001 && text == b"NiControllerManager" {
                set_error(e, this, ERROR_VERSION_TOO_OLD, STRCPY_S, TEXT_OLDER_FILES);
                return false;
            }
            let slot = table.wrapping_add(index.wrapping_mul(4));
            let loaders = e.mem.u32(LOADERS);
            let found = e.call(MAP_GET_AT, &args![loaders, name, slot]).bool();
            if !found {
                e.set(this, NiStream::m_uiLastError, ERROR_NO_CREATE_FUNCTION);
                let message = this.byte_add(NiStream::m_acLastErrorMessage.off);
                e.call(STRCPY_S, &args![message, ERROR_MESSAGE_SIZE, name]);
                e.call(
                    STRCAT_S,
                    &args![message, ERROR_MESSAGE_SIZE, TEXT_NO_CREATE_FUNCTION],
                );
                e.mem.set_u32(slot, 0);
            }
            if text == b"bhkConvexListShape" {
                e.mem.set_u32(slot, 0);
            }
            true
        });
        if !loaded {
            return false;
        }
        index += 1;
    }
    let objects = this.at(NiStream::m_kObjects);
    let mut index = 0u32;
    if e.get(objects, NiTLargeArray::m_uiMaxSize) != 0 {
        loop {
            let mut class = read_u16(e, this) as u32;
            let mut placeholder = false;
            if 0x1402_0004 < e.get(this, NiStream::m_uiNifFileVersion) && class & 0x8000 != 0 {
                class &= 0x7fff;
                placeholder = true;
            }
            let function = e.mem.u32(table.wrapping_add(class.wrapping_mul(4)));
            if function == 0 {
                if !placeholder {
                    free(e, table);
                    return false;
                }
                e.with_stack(4, |e, holder| {
                    e.call(LARGE_ARRAY_SET_AT, &args![objects, index, holder]);
                });
            } else {
                let object = e.call(function, &args![]).u32();
                add_ref(e, object);
                e.with_stack(4, |e, holder| {
                    e.mem.set_u32(holder.addr(), object);
                    e.call(LARGE_ARRAY_SET_AT, &args![objects, index, holder]);
                });
                release(e, object);
            }
            index += 1;
            if index >= e.get(objects, NiTLargeArray::m_uiMaxSize) {
                break;
            }
        }
    }
    free(e, table);
    true
}

// Translated from 00a65b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the table of fixed strings: the count (sizes the array), the length
/// of the longest string (a buffer of that plus one is allocated), then for
/// each (16-bit counter) a length and the characters, which are pooled
/// (`00a5b690`) and set at the index. Always true.
pub fn fn_00a65b00(e: &mut Engine, this: Ptr<NiStream>) -> bool {
    let strings = this.at(NiStream::m_kFixedStrings);
    let count = read_u32(e, this);
    fn_00a65140(e, strings, count);
    let longest = read_u32(e, this);
    let buffer = allocate(e, longest.wrapping_add(1));
    let mut counter = 0u32;
    if count != 0 {
        let mut index = 0u32;
        loop {
            let length = read_u32(e, this);
            read_into(e, this, buffer, length, 1);
            e.mem.set_u8(buffer.wrapping_add(length), 0);
            let handle = if buffer == 0 {
                empty_handle(e)
            } else {
                e.call(FIXED_STRING_FROM_CSTRING, &args![buffer]).u32()
            };
            e.with_stack(4, |e, holder| {
                e.mem.set_u32(holder.addr(), handle);
                fn_00a63730(e, strings, index, holder);
            });
            if handle != empty_handle(e) {
                handle_release(e, handle);
            }
            counter = counter.wrapping_add(1);
            index = counter & 0xffff;
            if index >= count {
                break;
            }
        }
    }
    free(e, buffer);
    true
}

/// What `LoadStream` does when the load is stopped or has ended: the same
/// clean-up as `FreeLoadData`.
fn clean_up_after_load(e: &mut Engine, this: Ptr<NiStream>) {
    enter_cleanup(e);
    fn_00a64250(e, this.at(NiStream::m_kObjects));
    reset_link_data(e, this);
    leave_cleanup(e);
}

/// The check each pass of the load loops makes: status 3 (suspend requested)
/// becomes 4 and the loading thread is suspended.
fn suspend_if_requested(e: &mut Engine, this: Ptr<NiStream>) {
    if e.get(this, NiStream::m_eBackgroundLoadStatus) == STATUS_SUSPEND_REQUESTED {
        e.set(this, NiStream::m_eBackgroundLoadStatus, STATUS_SUSPENDED);
        let thread = e.get(this, NiStream::m_pkThread);
        e.call(THREAD_SUSPEND, &args![thread]);
    }
}

// Translated from 00a65c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::LoadStream` (Xbox PDB): loads a whole file from the input
/// stream. Loads the header (virtual function `+0x34`), resets the load
/// indices and objects, reads the class table (version 5.0.0.1 and later),
/// the object size table (20.2.0.5), the fixed strings (20.1.0.1) and the
/// groups (5.0.0.6). Then loads each object with its `LoadBinary` (an object
/// slot that is empty is skipped by seeking over its size; before 5.0.0.1
/// the stream reads each object itself, virtual function `+0x50`, and a
/// failure ends the load), reads the top-level objects (`+0x48`), runs
/// `LinkObject` and `PostLinkObject` on every object, and runs the post
/// process functions on every top-level object. The status
/// `m_eBackgroundLoadStatus` is checked on every pass: 2 stops the load
/// (false), 3 suspends the thread. On success the node flags of old files are
/// fixed up (`00a63d10`) and the loaded data freed. Returns whether the
/// load completed.
pub fn ni_stream_load_stream(e: &mut Engine, this: Ptr<NiStream>) -> bool {
    if !e.vcall(this.addr(), SLOT_LOAD_HEADER, &args![]).bool() {
        return false;
    }
    e.set(this, NiStream::m_uiPostLink, 0);
    e.set(this, NiStream::m_uiLink, 0);
    e.set(this, NiStream::m_uiLoad, 0);
    ni_stream_remove_all_objects(e, this);
    let version = e.get(this, NiStream::m_uiNifFileVersion);
    let has_class_table = version >= 0x0500_0001;
    if has_class_table && !ni_stream_load_rtti(e, this) {
        return false;
    }
    if e.get(this, NiStream::m_uiNifFileVersion) > 0x1402_0004
        && !e
            .vcall(this.addr(), SLOT_LOAD_OBJECT_SIZE_TABLE, &args![])
            .bool()
    {
        return false;
    }
    if e.get(this, NiStream::m_uiNifFileVersion) > 0x1401_0000 && !fn_00a65b00(e, this) {
        return false;
    }
    if e.get(this, NiStream::m_uiNifFileVersion) > 0x0500_0005 {
        fn_00a647f0(e, this);
    }
    let objects = this.at(NiStream::m_kObjects);
    let total = e.get(objects, NiTLargeArray::m_uiMaxSize);
    while e.get(this, NiStream::m_uiLoad) < total {
        if e.get(this, NiStream::m_eBackgroundLoadStatus) == STATUS_CANCELLED {
            clean_up_after_load(e, this);
            return false;
        }
        suspend_if_requested(e, this);
        if has_class_table {
            let index = e.get(this, NiStream::m_uiLoad);
            let base = e.get(objects, NiTLargeArray::m_pBase);
            let object = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if object == 0 {
                let stream = e.get(this, NiStream::m_pkIstr);
                e.vcall(stream, STREAM_SLOT_GET_POSITION, &args![]);
                let sizes = e.get(this.at(NiStream::m_kObjectSizes), NiTLargeArray::m_pBase);
                let size = e.mem.u32(sizes.wrapping_add(index.wrapping_mul(4)));
                let stream = e.get(this, NiStream::m_pkIstr);
                e.vcall(stream, STREAM_SLOT_SEEK, &args![size]);
            } else {
                e.vcall(object, OBJECT_SLOT_LOAD_BINARY, &args![this]);
            }
        } else if !e.vcall(this.addr(), SLOT_LOAD_OBJECT, &args![]).bool() {
            ni_stream_free_load_data(e, this);
            return false;
        }
        let index = e.get(this, NiStream::m_uiLoad);
        e.set(this, NiStream::m_uiLoad, index.wrapping_add(1));
    }
    e.vcall(this.addr(), SLOT_LOAD_TOP_LEVEL_OBJECTS, &args![]);
    for (member, slot) in [
        (NiStream::m_uiLink, OBJECT_SLOT_LINK_OBJECT),
        (NiStream::m_uiPostLink, OBJECT_SLOT_POST_LINK_OBJECT),
    ] {
        while e.get(this, member) < total {
            suspend_if_requested(e, this);
            let index = e.get(this, member);
            let base = e.get(objects, NiTLargeArray::m_pBase);
            let object = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
            if object != 0 {
                e.vcall(object, slot, &args![this]);
            }
            let index = e.get(this, member);
            e.set(this, member, index.wrapping_add(1));
        }
    }
    if e.get(this, NiStream::m_eBackgroundLoadStatus) != STATUS_CANCELLED {
        suspend_if_requested(e, this);
        run_post_process_functions(e, this);
        if e.get(this, NiStream::m_eBackgroundLoadStatus) != STATUS_CANCELLED {
            fn_00a63d10(e, this);
            clean_up_after_load(e, this);
            return true;
        }
    }
    clean_up_after_load(e, this);
    false
}

/// Calls every function of the post process array on every top-level
/// object, as `function(stream, object)`.
fn run_post_process_functions(e: &mut Engine, this: Ptr<NiStream>) {
    let mut array = Ptr::<NiTArray>::new(e.mem.u32(POST_PROCESS_FUNCTIONS));
    let top = this.at(NiStream::m_kTopObjects);
    if e.get(array, NiTArray::m_usESize) == 0 || e.get(top, NiTLargeArray::m_uiSize) == 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let base = e.get(top, NiTLargeArray::m_pBase);
        let object = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        if object != 0 && e.get(array, NiTArray::m_usSize) != 0 {
            let mut at = 0u32;
            loop {
                let functions = e.get(array, NiTArray::m_pBase);
                let function = e.mem.u32(functions.wrapping_add(at.wrapping_mul(4)));
                if function != 0 {
                    e.call(function, &args![this, object]);
                    array = Ptr::new(e.mem.u32(POST_PROCESS_FUNCTIONS));
                }
                at += 1;
                if at >= e.get(array, NiTArray::m_usSize) as u32 {
                    break;
                }
            }
        }
        index += 1;
        if index >= e.get(top, NiTLargeArray::m_uiSize) {
            break;
        }
    }
}

// Translated from 00a65f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeObjectArray<NiFixedString>::Add` (instance in this unit): puts a
/// fixed string in the first empty slot below the used size (counted, count
/// of used elements up) or else at the end, growing the array by its grow-by
/// when it is full. Returns the index; the empty string is not added
/// (`0xffffffff`).
pub fn fn_00a65f80(e: &mut Engine, this: Ptr<NiTLargeArray>, value: Ptr) -> u32 {
    let handle = e.mem.u32(value.addr());
    if handle == empty_handle(e) {
        return u32::MAX;
    }
    let size = e.get(this, NiTLargeArray::m_uiSize);
    let mut index = 0u32;
    if size != 0 {
        loop {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            if e.mem.u32(slot) == empty_handle(e) {
                if e.mem.u32(slot) != handle {
                    if handle != empty_handle(e) {
                        handle_add_ref(e, handle);
                    }
                    let old = e.mem.u32(slot);
                    if old != empty_handle(e) {
                        handle_release(e, old);
                    }
                    e.mem.set_u32(slot, handle);
                }
                let count = e.get(this, NiTLargeArray::m_uiESize);
                e.set(this, NiTLargeArray::m_uiESize, count.wrapping_add(1));
                return index;
            }
            index += 1;
            if index >= size {
                break;
            }
        }
    }
    if e.get(this, NiTLargeArray::m_uiMaxSize) <= size {
        let grow = e.get(this, NiTLargeArray::m_uiGrowBy);
        fn_00a65140(e, this, grow.wrapping_add(size));
    }
    fn_00a63730(e, this, size, value);
    size
}

// Translated from 00a66030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the loader map: [`fn_00a63850`], then the
/// object freed when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00a66030(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_00a63850(e, this);
    delete_if_flagged(e, this.addr(), flags);
    this
}

// Translated from 00a66050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the type name map: [`fn_00a63910`], then
/// the object freed when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00a66050(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_00a63910(e, this);
    delete_if_flagged(e, this.addr(), flags);
    this
}

// Translated from 00a66070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the fixed strings array: its base virtual
/// table, the storage freed (`00a644a0`), the object freed when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00a66070(e: &mut Engine, this: Ptr<NiTLargeArray>, flags: u32) -> Ptr<NiTLargeArray> {
    e.mem
        .set_u32(this.addr(), FIXED_STRINGS_ARRAY_BASE_VTABLE_109BD4C);
    let base = e.get(this, NiTLargeArray::m_pBase);
    e.call(FIXED_STRINGS_FREE_STORAGE, &args![base]);
    delete_if_flagged(e, this.addr(), flags);
    this
}

// Translated from 00a660a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static initialization of `NiStream`: creates `ms_pkLoaders` (a 0x14-byte
/// map of 0x3b buckets that copies no keys, virtual table `0109bf4c`) and
/// `ms_pkPostProcessFunctions` (a 0x10-byte array growing by 3, virtual table
/// `0109bd88`) with the memory manager, and sets `bUseDefaultPath`. A failed
/// allocation stores null.
pub fn fn_00a660a0(e: &mut Engine) {
    let map = manager_allocate(e, 0x14);
    if map != 0 {
        e.mem.set_u32(map, LOADER_MAP_ROOT_VTABLE_109BC84);
        e.mem.set_u32(map.wrapping_add(4), 0x3b);
        e.mem.set_u32(map.wrapping_add(0xc), 0);
        let buckets = allocate(e, 0xec);
        e.mem.set_u32(map.wrapping_add(8), buckets);
        let count = e.mem.u32(map.wrapping_add(4));
        e.call(MEMSET, &args![buckets, 0u32, count.wrapping_mul(4)]);
        e.mem.set_u8(map.wrapping_add(MAP_COPY_KEYS), 0);
        e.mem.set_u32(map, LOADER_MAP_VTABLE_109BF4C);
        e.mem.set_u32(LOADERS, map);
    } else {
        e.mem.set_u32(LOADERS, 0);
    }
    let array = manager_allocate(e, 0x10);
    if array != 0 {
        e.mem.set_u16(array.wrapping_add(0xe), 3);
        e.mem.set_u32(array.wrapping_add(4), 0);
        e.mem.set_u16(array.wrapping_add(8), 0);
        e.mem.set_u16(array.wrapping_add(0xa), 0);
        e.mem.set_u16(array.wrapping_add(0xc), 0);
        e.mem.set_u32(array, POST_PROCESS_ARRAY_VTABLE_109BD88);
        e.mem.set_u32(POST_PROCESS_FUNCTIONS, array);
    } else {
        e.mem.set_u32(POST_PROCESS_FUNCTIONS, 0);
    }
    e.mem.set_u8(USE_DEFAULT_PATH, 1);
}

// Translated from 00a66150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::NiStream` (Xbox PDB): sets the members to their starting
/// values. The version is the newest the stream supports (`0x011a9b0c`) and
/// the user defined version the largest it accepts (`0x011a9b10`); the
/// registration map gets 0x25 buckets (`00aa1070`, zeroed) and its final
/// virtual table. Returns `this`.
pub fn ni_stream_constructor(e: &mut Engine, this: Ptr<NiStream>) -> Ptr<NiStream> {
    let at = this.addr();
    let put =
        |e: &mut Engine, offset: u32, value: u32| e.mem.set_u32(at.wrapping_add(offset), value);
    put(e, 0, NI_STREAM_VTABLE);
    put(e, 4, 0x22);
    for offset in [0x08, 0x48, 0x88] {
        e.mem.set_u8(at.wrapping_add(offset), 0);
    }
    put(e, 0xcc, 0);
    put(e, 0xc8, GROUPS_ARRAY_VTABLE_109BD90);
    for (offset, value) in [(0xd0, 0u16), (0xd6, 1), (0xd4, 0), (0xd2, 0)] {
        e.mem.set_u16(at.wrapping_add(offset), value);
    }
    put(e, 0x1f4, 0);
    put(e, 0x200, 0x400);
    put(e, 0x1f8, 0);
    put(e, 0x1fc, 0);
    put(e, 0x1f0, 0);
    put(e, 0x1ec, OBJECT_ARRAY_VTABLE_109BF6C);
    put(e, 0x20c, 0);
    put(e, 0x218, 1);
    put(e, 0x210, 0);
    put(e, 0x214, 0);
    put(e, 0x208, 0);
    put(e, 0x204, SIZES_ARRAY_VTABLE_1081EF0);
    put(e, 0x224, 0);
    put(e, 0x230, 1);
    put(e, 0x228, 0);
    put(e, 0x22c, 0);
    put(e, 0x220, 0);
    put(e, 0x21c, OBJECT_ARRAY_VTABLE_109BF6C);
    put(e, 0x23c, 0);
    put(e, 0x248, 0x80);
    put(e, 0x240, 0);
    put(e, 0x244, 0);
    put(e, 0x238, 0);
    put(e, 0x234, FIXED_STRINGS_ARRAY_VTABLE_109BF74);
    for offset in [0x254, 0x258, 0x25c, 0x264, 0x268, 0x26c] {
        put(e, offset, 0);
    }
    put(e, 0x274, REGISTER_MAP_BASE_VTABLE_109BC64);
    put(e, 0x278, 0x25);
    put(e, 0x280, 0);
    let buckets = allocate(e, 0x94);
    put(e, 0x27c, buckets);
    e.call(MEMSET, &args![buckets, 0u32, 0x25u32 * 4]);
    put(e, 0x274, REGISTER_MAP_VTABLE_109BD54);
    put(e, 0x2a0, 0);
    put(e, 0x2a4, 0);
    put(e, 0x2a8, 2);
    put(e, 0x2ac, u32::MAX);
    put(e, 0x2b0, u32::MAX);
    put(e, 0x3b8, 0);
    e.mem.set_u8(at.wrapping_add(0x3bc), 0);
    put(e, 0x24c, 0);
    put(e, 0x250, 0);
    let version = e.mem.u32(NIF_MAX_VERSION);
    put(e, 0xd8, version);
    let user = e.mem.u32(NIF_MAX_USER_VERSION);
    put(e, 0xdc, user);
    e.mem.set_u8(at.wrapping_add(0x1e4), 1);
    e.mem.set_u8(at.wrapping_add(0xe0), 0);
    put(e, 0x260, 0);
    put(e, 0x270, 0);
    e.mem.set_u8(at.wrapping_add(0x1e5), 1);
    e.mem.set_u8(USE_DEFAULT_PATH, 1);
    put(e, 0x28c, 0);
    put(e, 0x3b8, 0);
    e.mem.set_u8(at.wrapping_add(0x3bc), 0);
    this
}

// Translated from 00a66350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::_scalar_deleting_destructor_` (Xbox PDB): [`ni_stream_destructor`],
/// then the stream freed when bit 0 of `flags` is set. Returns `this`.
pub fn ni_stream_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiStream>,
    flags: u32,
) -> Ptr<NiStream> {
    ni_stream_destructor(e, this);
    delete_if_flagged(e, this.addr(), flags);
    this
}

// Translated from 00a66370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::InsertObject` (Xbox PDB): appends an object, counted, to the
/// top-level objects, growing the array by its grow-by when it is full.
pub fn ni_stream_insert_object(e: &mut Engine, this: Ptr<NiStream>, object: u32) {
    add_ref(e, object);
    object_array_append(e, this.at(NiStream::m_kTopObjects), object);
    release(e, object);
}

// Translated from 00a663e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::RegisterSaveObject` (Xbox PDB): false when the object already
/// has an id in the registration map; otherwise gives it the next id (the
/// number of objects so far), appends it, counted, to the objects array, and
/// returns true.
pub fn ni_stream_register_save_object(e: &mut Engine, this: Ptr<NiStream>, object: u32) -> bool {
    let map = this.at(NiStream::m_kRegisterMap);
    let known = e.with_stack(4, |e, id| {
        e.call(MAP_GET_AT, &args![map, object, id]).bool()
    });
    if known {
        return false;
    }
    let objects = this.at(NiStream::m_kObjects);
    let next = e.get(objects, NiTLargeArray::m_uiSize);
    e.call(REGISTER_MAP_SET_AT, &args![map, object, next]);
    add_ref(e, object);
    object_array_append(e, objects, object);
    release(e, object);
    true
}

// Translated from 00a66480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::RegisterFixedString` (Xbox PDB): adds a fixed string to the
/// table unless it is already there (the empty string is never added).
/// Always true.
pub fn ni_stream_register_fixed_string(e: &mut Engine, this: Ptr<NiStream>, string: Ptr) -> bool {
    let strings = this.at(NiStream::m_kFixedStrings);
    let handle = e.mem.u32(string.addr());
    if handle != empty_handle(e) {
        let count = e.get(strings, NiTLargeArray::m_uiSize);
        let base = e.get(strings, NiTLargeArray::m_pBase);
        for index in 0..count {
            if e.mem.u32(base.wrapping_add(index.wrapping_mul(4))) == handle {
                return true;
            }
        }
    }
    fn_00a65f80(e, strings, string);
    true
}

// Translated from 00a664e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a664e0 - the /GS stack cookie check (00ec408c) is not translated.
/// `NiStream::LoadObject` (Xbox PDB): reads a class name (`00a64d80`, into a
/// local buffer), creates an object of that class with its creation function
/// from `ms_pkLoaders` (error 5 and false when there is none), appends it,
/// counted, to the objects array and has it load itself (virtual function
/// `+0x4c`). Returns whether the class was found.
pub fn ni_stream_load_object(e: &mut Engine, this: Ptr<NiStream>) -> bool {
    e.with_stack(0x100 + 0x100, |e, name| {
        fn_00a64d80(e, this, name.addr());
        let loaders = e.mem.u32(LOADERS);
        let function = e.with_stack(4, |e, function| {
            let found = e.call(MAP_GET_AT, &args![loaders, name, function]).bool();
            found.then(|| e.mem.u32(function.addr()))
        });
        let Some(function) = function else {
            e.set(this, NiStream::m_uiLastError, ERROR_NO_CREATE_FUNCTION);
            let message = this.byte_add(NiStream::m_acLastErrorMessage.off);
            e.call(STRCPY_S, &args![message, ERROR_MESSAGE_SIZE, name]);
            e.call(
                STRCAT_S,
                &args![message, ERROR_MESSAGE_SIZE, TEXT_NO_CREATE_FUNCTION],
            );
            return false;
        };
        let object = e.call(function, &args![]).u32();
        add_ref(e, object);
        object_array_append(e, this.at(NiStream::m_kObjects), object);
        release(e, object);
        e.vcall(object, OBJECT_SLOT_LOAD_BINARY, &args![this]);
        true
    })
}

// Translated from 00a665f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the table of class names of a save and the class index of every
/// object. A local name-to-index map (0x25 buckets, final virtual table
/// `0109bf7c`) is filled with the name of each object's class (virtual
/// function `+0x74`; the index is the number of names so far, `00a643e0`);
/// the names are written in index order (count as 16 bits, each through
/// `00a64160`); then each object writes the index of its class, with
/// `0x8000` set when the object says it is a placeholder (virtual function
/// `+0x70`). An index the map does not give stays what the last one was (2
/// at first). The map is destroyed at the end.
pub fn fn_00a665f0(e: &mut Engine, this: Ptr<NiStream>) {
    e.with_stack(0x14, |e, map| {
        let map = map.cast::<NiTPointerMap>();
        e.mem.set_u32(map.addr(), NAME_MAP_ROOT_VTABLE_109BCEC);
        e.set(map, NiTPointerMap::m_uiHashSize, 0x25);
        e.set(map, NiTPointerMap::m_uiCount, 0);
        let buckets = allocate(e, 0x94);
        e.set(map, NiTPointerMap::m_ppkHashTable, buckets);
        e.call(MEMSET, &args![buckets, 0u32, 0x25u32 * 4]);
        e.mem.set_u8(map.addr().wrapping_add(MAP_COPY_KEYS), 0);
        e.mem.set_u32(map.addr(), NAME_MAP_VTABLE_109BF7C);
        let objects = this.at(NiStream::m_kObjects);
        let mut index = 0u32;
        if e.get(objects, NiTLargeArray::m_uiSize) != 0 {
            loop {
                let base = e.get(objects, NiTLargeArray::m_pBase);
                let object = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
                let name = class_name_of(e, object);
                let known = e.with_stack(4, |e, found| {
                    e.call(SAVE_MAP_GET_AT, &args![map, name, found]).bool()
                });
                if !known {
                    let count = e.get(map, NiTPointerMap::m_uiCount);
                    fn_00a643e0(e, map, name, count);
                }
                index += 1;
                if index >= e.get(objects, NiTLargeArray::m_uiSize) {
                    break;
                }
            }
        }
        let total = e.get(map, NiTPointerMap::m_uiCount) & 0xffff;
        let names = allocate(e, total.wrapping_mul(4));
        let hash_size = e.get(map, NiTPointerMap::m_uiHashSize);
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        let mut node = 0u32;
        let mut bucket = 0u32;
        while bucket < hash_size {
            node = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
            if node != 0 {
                break;
            }
            bucket += 1;
        }
        while node != 0 {
            let key = e.mem.u32(node.wrapping_add(4));
            let value = e.mem.u16(node.wrapping_add(8)) as u32;
            node = e.mem.u32(node);
            if node == 0 {
                let mut next = e.vcall(map.addr(), MAP_SLOT_HASH, &args![key]).u32();
                loop {
                    next = next.wrapping_add(1);
                    if hash_size <= next {
                        node = 0;
                        break;
                    }
                    node = e.mem.u32(table.wrapping_add(next.wrapping_mul(4)));
                    if node != 0 {
                        break;
                    }
                }
            }
            e.mem
                .set_u32(names.wrapping_add(value.wrapping_mul(4)), key);
        }
        write_u16(e, this, total as u16);
        if total as u16 != 0 {
            let mut at = 0u32;
            loop {
                let name = e.mem.u32(names.wrapping_add(at.wrapping_mul(4)));
                fn_00a64160(e, this, name);
                at += 1;
                if at >= total {
                    break;
                }
            }
        }
        free(e, names);
        let mut written = 2u32;
        let mut index = 0u32;
        if e.get(objects, NiTLargeArray::m_uiSize) != 0 {
            loop {
                let base = e.get(objects, NiTLargeArray::m_pBase);
                let object = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
                let name = class_name_of(e, object);
                let hash = e.vcall(map.addr(), MAP_SLOT_HASH, &args![name]).u32();
                let table = e.get(map, NiTPointerMap::m_ppkHashTable);
                let mut node = e.mem.u32(table.wrapping_add(hash.wrapping_mul(4)));
                while node != 0 {
                    let node_key = e.mem.u32(node.wrapping_add(4));
                    let equal = e
                        .vcall(map.addr(), MAP_SLOT_KEYS_EQUAL, &args![name, node_key])
                        .bool();
                    if equal {
                        written = e.mem.u16(node.wrapping_add(8)) as u32;
                        break;
                    }
                    node = e.mem.u32(node);
                }
                if e.vcall(object, OBJECT_SLOT_IS_NULL_PLACEHOLDER, &args![])
                    .bool()
                {
                    written |= 0x8000;
                }
                write_u16(e, this, written as u16);
                index += 1;
                if index >= e.get(objects, NiTLargeArray::m_uiSize) {
                    break;
                }
            }
        }
        fn_00a63910(e, map);
    });
}

/// The class name of an object: the string its virtual function `+0x74`
/// points at (the function returns the address of the name's pointer).
fn class_name_of(e: &mut Engine, object: u32) -> u32 {
    let at = e.vcall(object, OBJECT_SLOT_GET_RTTI_NAME, &args![]).u32();
    e.mem.u32(at)
}

// Translated from 00a66850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStream::SaveStream` (Xbox PDB): registers the objects (virtual
/// function `+0x44`), sizes the size table to the number of objects, writes
/// the header (`+0x38`), the class names, the reserved size table (`+0x54`),
/// the fixed strings and the groups; then saves each object (virtual
/// function `+0x58`) noting how many bytes it wrote; writes the top-level
/// objects (`+0x4c`), fills in the size table (`+0x58`), and frees the
/// objects and the registration map. Always true.
pub fn ni_stream_save_stream(e: &mut Engine, this: Ptr<NiStream>) -> bool {
    e.vcall(this.addr(), SLOT_REGISTER_OBJECTS, &args![]);
    let objects = this.at(NiStream::m_kObjects);
    let count = e.get(objects, NiTLargeArray::m_uiSize);
    e.call(
        SIZES_ARRAY_SET_SIZE,
        &args![this.at(NiStream::m_kObjectSizes), count],
    );
    e.vcall(this.addr(), SLOT_SAVE_HEADER, &args![]);
    fn_00a665f0(e, this);
    let position = e
        .vcall(this.addr(), SLOT_PRE_SAVE_OBJECT_SIZE_TABLE, &args![])
        .u32();
    fn_00a63ee0(e, this);
    fn_00a64b80(e, this);
    fn_00a64010(e, this);
    let sizes = this.at(NiStream::m_kObjectSizes);
    let mut index = 0u32;
    while index < e.get(objects, NiTLargeArray::m_uiSize) {
        let stream = e.get(this, NiStream::m_pkOstr);
        let start = e.vcall(stream, STREAM_SLOT_GET_POSITION, &args![]).u32();
        let base = e.get(objects, NiTLargeArray::m_pBase);
        let object = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        e.vcall(object, OBJECT_SLOT_SAVE_BINARY, &args![this]);
        let stream = e.get(this, NiStream::m_pkOstr);
        let end = e.vcall(stream, STREAM_SLOT_GET_POSITION, &args![]).u32();
        sizes_set_at(e, sizes, index, end.wrapping_sub(start));
        index += 1;
    }
    e.vcall(this.addr(), SLOT_SAVE_TOP_LEVEL_OBJECTS, &args![]);
    e.vcall(this.addr(), SLOT_SAVE_OBJECT_SIZE_TABLE, &args![position]);
    fn_00a64250(e, objects);
    e.call(MAP_REMOVE_ALL, &args![this.at(NiStream::m_kRegisterMap)]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a632e0, ni_stream_set_last_error(Ptr<NiStream>, u32)),
        entry!(0x00a632f0, ni_stream_load(Ptr<NiStream>, u32, u32) -> bool),
        entry!(0x00a63330, ni_stream_load_ov2(Ptr<NiStream>, Ptr) -> bool),
        entry!(0x00a63380, ni_stream_save(Ptr<NiStream>, Ptr) -> bool),
        entry!(0x00a633b0, ni_stream_set_texture(Ptr<NiStream>, u32)),
        entry!(
            0x00a633d0,
            ni_stream_get_texture(Ptr<NiStream>, u32, Ptr) -> bool
        ),
        entry!(0x00a633f0, fn_00a633f0(Ptr<NiStream>, Ptr) -> u32),
        entry!(
            0x00a63430,
            ni_stream_get_object_from_link_id(Ptr<NiStream>) -> u32
        ),
        entry!(
            0x00a63460,
            ni_stream_get_number_of_link_ids(Ptr<NiStream>) -> u32
        ),
        entry!(
            0x00a63480,
            ni_stream_get_link_id_from_object(Ptr<NiStream>, u32) -> u32
        ),
        entry!(0x00a634b0, fn_00a634b0(u32) -> u32),
        entry!(
            0x00a634e0,
            ni_stream_get_group_from_id(Ptr<NiStream>, u32) -> u32
        ),
        entry!(0x00a634f0, fn_00a634f0(Ptr<NiStream>, u32) -> u32),
        entry!(0x00a63520, fn_00a63520(u32) -> u32),
        entry!(0x00a63540, fn_00a63540(u32)),
        entry!(0x00a635b0, ni_stream_register_objects(Ptr<NiStream>)),
        entry!(
            0x00a635e0,
            ni_stream_save_ov2(Ptr<NiStream>, Ptr, Ptr) -> bool
        ),
        entry!(0x00a63630, ni_stream_get_version_from_string(Ptr) -> u32),
        entry!(0x00a636b0, fn_00a636b0(Ptr<NiStream>, Ptr, u32) -> bool),
        entry!(0x00a636f0, fn_00a636f0(Ptr<NiStream>, Ptr)),
        entry!(0x00a63730, fn_00a63730(Ptr<NiTLargeArray>, u32, Ptr)),
        entry!(0x00a637d0, fn_00a637d0(Ptr<NiTLargeArray>)),
        entry!(0x00a63840, fn_00a63840(Ptr<NiTPointerMap>, u32) -> u32),
        entry!(0x00a63850, fn_00a63850(Ptr<NiTPointerMap>)),
        entry!(
            0x00a638c0,
            ni_t_pointer_map_delete_item(Ptr<NiTPointerMap>, Ptr)
        ),
        entry!(0x00a63910, fn_00a63910(Ptr<NiTPointerMap>)),
        entry!(
            0x00a63980,
            ni_t_string_template_map_set_value(Ptr<NiTPointerMap>, Ptr, u32, u16)
        ),
        entry!(
            0x00a639e0,
            fn_00a639e0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00a63a20,
            fn_00a63a20(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00a63a60,
            ni_t_map_base_scalar_deleting_destructor(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00a63aa0,
            ni_t_pointer_map_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00a63ae0, fn_00a63ae0(u32) -> u32),
        entry!(0x00a63b30, fn_00a63b30(Ptr<NiTArray>, u32) -> Ptr<NiTArray>),
        entry!(
            0x00a63b60,
            fn_00a63b60(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00a63ba0,
            fn_00a63ba0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00a63be0, ni_stream_load_ov3(Ptr<NiStream>, Ptr) -> bool),
        entry!(0x00a63cb0, fn_00a63cb0(Ptr<NiStream>, Ptr)),
        entry!(0x00a63d10, fn_00a63d10(Ptr<NiStream>)),
        entry!(0x00a63d80, ni_stream_save_top_level_objects(Ptr<NiStream>)),
        entry!(
            0x00a63df0,
            ni_stream_pre_save_object_size_table(Ptr<NiStream>) -> u32
        ),
        entry!(
            0x00a63e60,
            ni_stream_save_object_size_table(Ptr<NiStream>, u32) -> bool
        ),
        entry!(0x00a63ee0, fn_00a63ee0(Ptr<NiStream>)),
        entry!(0x00a64010, fn_00a64010(Ptr<NiStream>)),
        entry!(0x00a640b0, ni_stream_save_ov3(Ptr<NiStream>, Ptr) -> bool),
        entry!(0x00a64160, fn_00a64160(Ptr<NiStream>, u32)),
        entry!(0x00a641f0, ni_stream_save_fixed_string(Ptr<NiStream>, Ptr)),
        entry!(0x00a64250, fn_00a64250(Ptr<NiTLargeArray>)),
        entry!(0x00a643e0, fn_00a643e0(Ptr<NiTPointerMap>, u32, u32)),
        entry!(0x00a644f0, ni_stream_remove_all_objects(Ptr<NiStream>)),
        entry!(0x00a64550, ni_stream_change_object(Ptr<NiStream>, u32)),
        entry!(0x00a645b0, ni_stream_read_link_id(Ptr<NiStream>)),
        entry!(0x00a64620, ni_stream_resolve_link_id(Ptr<NiStream>) -> u32),
        // 00a646f5 RET: EAX holds the count read (00a646d4/00a646e7).
        entry!(
            0x00a64670,
            ni_stream_read_multiple_link_ids(Ptr<NiStream>) -> u32
        ),
        entry!(
            0x00a64700,
            ni_stream_set_number_of_link_ids(Ptr<NiStream>, u32)
        ),
        entry!(
            0x00a64740,
            ni_stream_load_object_size_table(Ptr<NiStream>) -> bool
        ),
        entry!(0x00a647f0, fn_00a647f0(Ptr<NiStream>)),
        entry!(0x00a64900, ni_stream_register_loader(u32, u32) -> u32),
        entry!(0x00a64920, ni_stream_register_post_process_function(u32) -> u32),
        entry!(0x00a64940, ni_stream_save_header(Ptr<NiStream>)),
        entry!(0x00a64b80, fn_00a64b80(Ptr<NiStream>)),
        entry!(0x00a64ca0, ni_stream_free_load_data(Ptr<NiStream>)),
        entry!(0x00a64ce0, ni_stream_load_c_string(Ptr<NiStream>, Ptr)),
        entry!(0x00a64d80, fn_00a64d80(Ptr<NiStream>, u32)),
        entry!(0x00a64df0, ni_stream_load_fixed_string(Ptr<NiStream>, Ptr)),
        entry!(
            0x00a64ea0,
            ni_stream_load_c_string_as_fixed_string(Ptr<NiStream>, Ptr)
        ),
        entry!(0x00a65140, fn_00a65140(Ptr<NiTLargeArray>, u32)),
        entry!(
            0x00a652d0,
            fn_00a652d0(Ptr<NiTLargeArray>, u32) -> Ptr<NiTLargeArray>
        ),
        entry!(0x00a65300, ni_stream_destructor(Ptr<NiStream>)),
        entry!(0x00a65400, ni_stream_load_header(Ptr<NiStream>) -> bool),
        entry!(0x00a65720, ni_stream_load_top_level_objects(Ptr<NiStream>)),
        entry!(0x00a65800, ni_stream_load_rtti(Ptr<NiStream>) -> bool),
        entry!(0x00a65b00, fn_00a65b00(Ptr<NiStream>) -> bool),
        entry!(0x00a65c60, ni_stream_load_stream(Ptr<NiStream>) -> bool),
        entry!(0x00a65f80, fn_00a65f80(Ptr<NiTLargeArray>, Ptr) -> u32),
        entry!(
            0x00a66030,
            fn_00a66030(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00a66050,
            fn_00a66050(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00a66070,
            fn_00a66070(Ptr<NiTLargeArray>, u32) -> Ptr<NiTLargeArray>
        ),
        entry!(0x00a660a0, fn_00a660a0()),
        entry!(
            0x00a66150,
            ni_stream_constructor(Ptr<NiStream>) -> Ptr<NiStream>
        ),
        entry!(
            0x00a66350,
            ni_stream_scalar_deleting_destructor(Ptr<NiStream>, u32) -> Ptr<NiStream>
        ),
        entry!(0x00a66370, ni_stream_insert_object(Ptr<NiStream>, u32)),
        entry!(
            0x00a663e0,
            ni_stream_register_save_object(Ptr<NiStream>, u32) -> bool
        ),
        entry!(
            0x00a66480,
            ni_stream_register_fixed_string(Ptr<NiStream>, Ptr) -> bool
        ),
        entry!(0x00a664e0, ni_stream_load_object(Ptr<NiStream>) -> bool),
        entry!(0x00a665f0, fn_00a665f0(Ptr<NiStream>)),
        entry!(0x00a66850, ni_stream_save_stream(Ptr<NiStream>) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Pool = Rc<RefCell<Vec<(Vec<u8>, u32)>>>;

    /// Read and write functions of the fake binary stream, and its virtual
    /// functions.
    const FAKE_READ: u32 = 0x0720_0000;
    const FAKE_WRITE: u32 = 0x0720_0004;
    const FAKE_DELETE: u32 = 0x0720_0008;
    const FAKE_VALID: u32 = 0x0720_000c;
    const FAKE_SEEK: u32 = 0x0720_0010;
    const FAKE_POSITION: u32 = 0x0720_0014;
    const FAKE_ENDIAN: u32 = 0x0720_0018;
    /// Where the generated test doubles for object methods start.
    const FIRST_METHOD: u32 = 0x0710_0000;
    /// Some class creation functions and the class record the tests use.
    const CREATE_FIRST: u32 = 0x0731_0000;
    const CREATE_SECOND: u32 = 0x0731_0004;

    /// An engine with the platform imports, doubles for every callee outside
    /// this file, a fake binary stream that reads from `input` and writes to
    /// `output`, and a stream object using it.
    struct Fx {
        e: Engine,
        log: Log,
        input: Rc<RefCell<VecDeque<u8>>>,
        output: Rc<RefCell<Vec<u8>>>,
        line: Rc<RefCell<Vec<u8>>>,
        ns: Ptr<NiStream>,
        bs: u32,
        empty: u32,
        methods: u32,
    }

    fn make_handle(e: &mut Engine, text: &[u8]) -> u32 {
        let block = e.mem.alloc(text.len() as u32 + 9);
        e.mem.set_u32(block, 1);
        e.mem.set_u32(block + 4, text.len() as u32);
        e.mem.set_cstr(block + 8, text);
        block + 8
    }

    fn rec_on(e: &mut Engine, log: &Log, addr: u32, ret: u32) {
        let log = log.clone();
        e.register_double(addr, move |_, a| {
            log.borrow_mut().push((addr, a.to_vec()));
            ret.into_ret()
        });
    }

    fn install_crt(e: &mut Engine) {
        let copy: AbiFn = |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            0u32.into_ret()
        };
        e.register(STRCPY_S, copy);
        e.register(STRCPY_S_FORWARD, copy);
        e.register(STRNCPY_S, |e, a| {
            let mut text = e.mem.cstr(a[2]);
            text.truncate(a[3] as usize);
            e.mem.set_cstr(a[0], &text);
            0u32.into_ret()
        });
        e.register(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            0u32.into_ret()
        });
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e.register(STRSTR, |e, a| {
            let hay = e.mem.cstr(a[0]);
            let needle = e.mem.cstr(a[1]);
            hay.windows(needle.len())
                .position(|w| w == needle.as_slice())
                .map_or(0, |i| a[0] + i as u32)
                .into_ret()
        });
        e.register(ATOL, |e, a| {
            let text = String::from_utf8_lossy(&e.mem.cstr(a[0])).into_owned();
            (text.trim().parse::<i32>().unwrap_or(0) as u32).into_ret()
        });
    }

    impl Fx {
        fn new() -> Fx {
            let mut e = Engine::new();
            for page in [
                0x0109_b000,
                0x0104_4000,
                0x0109_5000,
                0x0108_1000,
                0x011a_9000,
                0x011f_4000,
                0x011c_5000,
            ] {
                e.map(page, 0x1000);
            }
            e.register(INTERLOCKED_INCREMENT, |e, a| {
                let count = e.mem.u32(a[0]).wrapping_add(1);
                e.mem.set_u32(a[0], count);
                count.into_ret()
            });
            e.register(INTERLOCKED_DECREMENT, |e, a| {
                let count = e.mem.u32(a[0]).wrapping_sub(1);
                e.mem.set_u32(a[0], count);
                count.into_ret()
            });
            e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(FREE, |e, a| {
                e.mem.free(a[0]);
                Ret::default()
            });
            e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
            e.register(MEMORY_MANAGER_DEALLOCATE, |e, a| {
                e.mem.free(a[1]);
                Ret::default()
            });
            install_crt(&mut e);
            let log: Log = Rc::default();
            let input: Rc<RefCell<VecDeque<u8>>> = Rc::default();
            let output: Rc<RefCell<Vec<u8>>> = Rc::default();
            let line: Rc<RefCell<Vec<u8>>> = Rc::default();
            let strings: Pool = Rc::default();
            let empty = make_handle(&mut e, b"");
            e.mem.set_u32(EMPTY_FIXED_STRING, empty);
            // The pooled string of a C string: a counted handle (null gives
            // the empty string).
            e.register_double(FIXED_STRING_FROM_CSTRING, move |e, a| {
                if a[0] == 0 {
                    return empty.into_ret();
                }
                let text = e.mem.cstr(a[0]);
                let mut strings = strings.borrow_mut();
                if let Some((_, handle)) = strings.iter().find(|(t, _)| *t == text) {
                    let count = e.mem.u32(handle - 8) + 1;
                    e.mem.set_u32(handle - 8, count);
                    return (*handle).into_ret();
                }
                let handle = make_handle(e, &text);
                strings.push((text, handle));
                handle.into_ret()
            });
            e.register_double(VECTOR_CONSTRUCTOR_ITERATOR, move |e, a| {
                for i in 0..a[2] {
                    e.mem.set_u32(a[0] + i * 4, empty);
                }
                Ret::default()
            });
            // The fake binary stream.
            let queue = input.clone();
            e.register_double(FAKE_READ, move |e, a| {
                let mut queue = queue.borrow_mut();
                for i in 0..a[2] {
                    e.mem.set_u8(a[1] + i, queue.pop_front().unwrap_or(0));
                }
                a[2].into_ret()
            });
            let written = output.clone();
            e.register_double(FAKE_WRITE, move |e, a| {
                written.borrow_mut().extend(e.mem.bytes(a[1], a[2]));
                a[2].into_ret()
            });
            e.register(FAKE_POSITION, |e, a| e.mem.u32(a[0] + 4).into_ret());
            rec_on(&mut e, &log, FAKE_DELETE, 0);
            rec_on(&mut e, &log, FAKE_VALID, 1);
            rec_on(&mut e, &log, FAKE_SEEK, 0);
            rec_on(&mut e, &log, FAKE_ENDIAN, 0);
            let bs_table = e.mem.alloc(0x40);
            for (slot, target) in [
                (0, FAKE_DELETE),
                (4, FAKE_VALID),
                (8, FAKE_SEEK),
                (0xc, FAKE_POSITION),
                (0x10, FAKE_ENDIAN),
            ] {
                e.mem.set_u32(bs_table + slot, target);
            }
            let bs = e.mem.alloc(0x20);
            e.mem.set_u32(bs, bs_table);
            e.mem.set_u32(bs + 8, FAKE_READ);
            e.mem.set_u32(bs + 0xc, FAKE_WRITE);
            let table = e.mem.alloc(0x100);
            let ns = Ptr::new(e.mem.alloc(NiStream::SIZE));
            e.mem.set_u32(ns.addr(), table);
            e.mem.set_u32(ns.addr() + 0x24c, bs);
            e.mem.set_u32(ns.addr() + 0x250, bs);
            Fx {
                e,
                log,
                input,
                output,
                line,
                ns,
                bs,
                empty,
                methods: 0,
            }
        }

        /// A recording double with this result at `addr`.
        fn rec(&mut self, addr: u32, ret: u32) {
            rec_on(&mut self.e, &self.log, addr, ret);
        }

        /// The argument lists of the calls to `addr`, in order.
        fn calls(&self, addr: u32) -> Vec<Vec<u32>> {
            self.log
                .borrow()
                .iter()
                .filter(|(a, _)| *a == addr)
                .map(|(_, args)| args.clone())
                .collect()
        }

        /// A zeroed object with its own virtual table.
        fn object(&mut self) -> u32 {
            let table = self.e.mem.alloc(0x100);
            let object = self.e.mem.alloc(0x100);
            self.e.mem.set_u32(object, table);
            object
        }

        /// Puts a recording double with this result in slot `slot` of the
        /// object's virtual table; returns its address.
        fn method(&mut self, object: u32, slot: u32, ret: u32) -> u32 {
            self.methods += 1;
            let addr = FIRST_METHOD + self.methods * 4;
            self.rec(addr, ret);
            let table = self.e.mem.u32(object);
            self.e.mem.set_u32(table + slot, addr);
            addr
        }

        fn ns_method(&mut self, slot: u32, ret: u32) -> u32 {
            self.method(self.ns.addr(), slot, ret)
        }

        fn cs(&mut self, text: &str) -> u32 {
            let at = self.e.mem.alloc(text.len() as u32 + 1);
            self.e.mem.set_cstr(at, text.as_bytes());
            at
        }

        fn handle(&mut self, text: &str) -> u32 {
            make_handle(&mut self.e, text.as_bytes())
        }

        fn refs(&self, handle: u32) -> u32 {
            self.e.mem.u32(handle - 8)
        }

        /// A word in memory holding `value` (a `NiFixedString` or a
        /// `NiPointer` on the stack).
        fn cell(&mut self, value: u32) -> u32 {
            let at = self.e.mem.alloc(4);
            self.e.mem.set_u32(at, value);
            at
        }

        fn get(&self, offset: u32) -> u32 {
            self.e.mem.u32(self.ns.addr() + offset)
        }

        fn put(&mut self, offset: u32, value: u32) {
            self.e.mem.set_u32(self.ns.addr() + offset, value);
        }

        fn at(&self, offset: u32) -> u32 {
            self.ns.addr() + offset
        }

        fn feed(&mut self, words: &[u32]) {
            for word in words {
                self.input.borrow_mut().extend(word.to_le_bytes());
            }
        }

        fn feed_bytes(&mut self, bytes: &[u8]) {
            self.input.borrow_mut().extend(bytes.iter().copied());
        }

        fn written(&self) -> Vec<u8> {
            self.output.borrow().clone()
        }

        /// A `NiTLargeArray` at `at` with these elements, room for `max`
        /// and grow-by 4. The storage is laid out like a `new[]` block (the
        /// count in front), as the fixed strings array needs.
        fn array(&mut self, at: u32, items: &[u32], max: u32) -> u32 {
            let block = self.e.mem.alloc(max * 4 + 4);
            self.e.mem.set_u32(block, max);
            let base = block + 4;
            for i in 0..max {
                self.e.mem.set_u32(base + i * 4, self.empty);
            }
            for (i, value) in items.iter().enumerate() {
                self.e.mem.set_u32(base + i as u32 * 4, *value);
            }
            let used = items
                .iter()
                .filter(|v| **v != 0 && **v != self.empty)
                .count() as u32;
            self.e.mem.set_u32(at + 4, base);
            self.e.mem.set_u32(at + 8, max);
            self.e.mem.set_u32(at + 0xc, items.len() as u32);
            self.e.mem.set_u32(at + 0x10, used);
            self.e.mem.set_u32(at + 0x14, 4);
            base
        }

        /// Like [`Fx::array`] for arrays whose empty slots are null.
        fn pointer_array(&mut self, at: u32, items: &[u32], max: u32) -> u32 {
            let base = self.array(at, items, max);
            for i in items.len() as u32..max {
                self.e.mem.set_u32(base + i * 4, 0);
            }
            base
        }

        /// A `NiTArray` (16-bit counts) at `at`.
        fn small_array(&mut self, at: u32, items: &[u32], max: u16) -> u32 {
            let base = self.e.mem.alloc(max as u32 * 4 + 4);
            for (i, value) in items.iter().enumerate() {
                self.e.mem.set_u32(base + i as u32 * 4, *value);
            }
            let used = items.iter().filter(|v| **v != 0).count() as u16;
            self.e.mem.set_u32(at + 4, base);
            self.e.mem.set_u16(at + 8, max);
            self.e.mem.set_u16(at + 0xa, items.len() as u16);
            self.e.mem.set_u16(at + 0xc, used);
            self.e.mem.set_u16(at + 0xe, 3);
            base
        }

        /// A `NiTPrimitiveSet` at `at` holding these values.
        fn set_of(&mut self, at: u32, items: &[u32]) -> u32 {
            let base = self.e.mem.alloc(items.len() as u32 * 4 + 4);
            for (i, value) in items.iter().enumerate() {
                self.e.mem.set_u32(base + i as u32 * 4, *value);
            }
            self.e.mem.set_u32(at, base);
            self.e.mem.set_u32(at + 4, items.len() as u32);
            self.e.mem.set_u32(at + 8, items.len() as u32);
            base
        }

        /// The word at index `index` of the array storage at `base`.
        fn word(&self, base: u32, index: u32) -> u32 {
            self.e.mem.u32(base + index * 4)
        }

        /// A 0x14-byte string map with `buckets` buckets.
        fn map_object(&mut self, buckets: u32) -> (u32, u32) {
            let map = self.e.mem.alloc(0x14);
            let table = self.e.mem.alloc(buckets * 4);
            self.e.mem.set_u32(map + 4, buckets);
            self.e.mem.set_u32(map + 8, table);
            (map, table)
        }

        /// Makes the doubles of `LARGE_ARRAY_SET_SIZE` and
        /// `LARGE_ARRAY_SET_AT` do what the array functions do (storage, size
        /// and capacity), and records the calls.
        fn install_array_ops(&mut self) {
            let log = self.log.clone();
            self.e.register_double(LARGE_ARRAY_SET_SIZE, move |e, a| {
                log.borrow_mut().push((LARGE_ARRAY_SET_SIZE, a.to_vec()));
                let old = e.mem.u32(a[0] + 4);
                let base = e.mem.alloc(a[1] * 4 + 4);
                for i in 0..e.mem.u32(a[0] + 0xc).min(a[1]) {
                    let value = e.mem.u32(old + i * 4);
                    e.mem.set_u32(base + i * 4, value);
                }
                e.mem.set_u32(a[0] + 4, base);
                e.mem.set_u32(a[0] + 8, a[1]);
                Ret::default()
            });
            let log = self.log.clone();
            self.e.register_double(LARGE_ARRAY_SET_AT, move |e, a| {
                log.borrow_mut().push((LARGE_ARRAY_SET_AT, a.to_vec()));
                let value = e.mem.u32(a[2]);
                let base = e.mem.u32(a[0] + 4);
                e.mem.set_u32(base + a[1] * 4, value);
                if a[1] >= e.mem.u32(a[0] + 0xc) {
                    e.mem.set_u32(a[0] + 0xc, a[1] + 1);
                }
                Ret::default()
            });
        }

        /// Makes the three `NiStreamLoadBinary` functions read from the
        /// input queue.
        fn install_binary_loaders(&mut self) {
            for (addr, size) in [(LOAD_BINARY_U32, 4), (LOAD_BINARY_U8, 1)] {
                let queue = self.input.clone();
                self.e.register_double(addr, move |e, a| {
                    for i in 0..size {
                        let byte = queue.borrow_mut().pop_front().unwrap_or(0);
                        e.mem.set_u8(a[1] + i, byte);
                    }
                    Ret::default()
                });
            }
            let queue = self.input.clone();
            self.e.register_double(LOAD_BINARY_BYTES, move |e, a| {
                for i in 0..a[2] {
                    let byte = queue.borrow_mut().pop_front().unwrap_or(0);
                    e.mem.set_u8(a[1] + i, byte);
                }
                Ret::default()
            });
        }

        /// Watches `MAP_REMOVE_ALL`: the virtual table of the map at each
        /// call.
        fn watch_remove_all(&mut self) -> Rc<RefCell<Vec<u32>>> {
            let seen: Rc<RefCell<Vec<u32>>> = Rc::default();
            let list = seen.clone();
            self.e.register_double(MAP_REMOVE_ALL, move |e, a| {
                list.borrow_mut().push(e.mem.u32(a[0]));
                Ret::default()
            });
            seen
        }

        /// Makes `MAP_GET_AT` look names up in this list (the value is
        /// written to the third argument).
        fn install_lookup(&mut self, entries: &[(&str, u32)]) {
            let table: Vec<(Vec<u8>, u32)> = entries
                .iter()
                .map(|(name, value)| (name.as_bytes().to_vec(), *value))
                .collect();
            let log = self.log.clone();
            self.e.register_double(MAP_GET_AT, move |e, a| {
                log.borrow_mut().push((MAP_GET_AT, a.to_vec()));
                let name = e.mem.cstr(a[1]);
                match table.iter().find(|(n, _)| *n == name) {
                    Some((_, value)) => {
                        e.mem.set_u32(a[2], *value);
                        1u32.into_ret()
                    }
                    None => 0u32.into_ret(),
                }
            });
        }
    }

    /// The bytes of these little-endian words.
    fn words(list: &[u32]) -> Vec<u8> {
        list.iter().flat_map(|w| w.to_le_bytes()).collect()
    }

    // -----------------------------------------------------------------
    // Calls through the stream's virtual functions and the small accessors.

    #[test]
    fn set_last_error_stores_the_code() {
        let mut fx = Fx::new();
        ni_stream_set_last_error(&mut fx.e, fx.ns, 7);
        assert_eq!(fx.get(0x3b8), 7);
    }

    #[test]
    fn load_from_a_buffer_wraps_it_in_a_memory_stream() {
        for result in [1u32, 0] {
            let mut fx = Fx::new();
            fx.rec(NI_MEM_STREAM_CONSTRUCT, 0);
            fx.rec(NI_MEM_STREAM_DESTRUCT, 0);
            let load = fx.ns_method(SLOT_LOAD, result);
            assert_eq!(ni_stream_load(&mut fx.e, fx.ns, 0x1000, 0x20), result == 1);
            let made = fx.calls(NI_MEM_STREAM_CONSTRUCT);
            assert_eq!(made.len(), 1);
            assert_eq!(made[0][1..], [0x1000, 0x20]);
            let stream = made[0][0];
            assert_eq!(fx.calls(load), vec![vec![fx.ns.addr(), stream]]);
            assert_eq!(fx.calls(NI_MEM_STREAM_DESTRUCT), vec![vec![stream]]);
        }
    }

    #[test]
    fn load_from_a_stream_sets_the_input_for_the_load() {
        let mut fx = Fx::new();
        let seen = Rc::new(Cell::new(0));
        let ns = fx.ns.addr();
        let load_stream = fx.ns_method(SLOT_LOAD_STREAM, 1);
        let watcher = seen.clone();
        fx.e.register_double(load_stream, move |e, _| {
            watcher.set(e.mem.u32(ns + 0x24c));
            1u32.into_ret()
        });
        let stream = fx.object();
        fx.method(stream, STREAM_SLOT_IS_VALID, 1);
        assert!(ni_stream_load_ov2(&mut fx.e, fx.ns, Ptr::new(stream)));
        assert_eq!(seen.get(), stream);
        assert_eq!(fx.get(0x24c), 0);
        // A stream that is not usable is never loaded from.
        let mut fx = Fx::new();
        let load_stream = fx.ns_method(SLOT_LOAD_STREAM, 1);
        let stream = fx.object();
        fx.method(stream, STREAM_SLOT_IS_VALID, 0);
        assert!(!ni_stream_load_ov2(&mut fx.e, fx.ns, Ptr::new(stream)));
        assert!(fx.calls(load_stream).is_empty());
        assert_eq!(fx.get(0x24c), 0);
    }

    #[test]
    fn save_to_a_stream_sets_the_output_for_the_save() {
        let mut fx = Fx::new();
        let seen = Rc::new(Cell::new(0));
        let ns = fx.ns.addr();
        let save_stream = fx.ns_method(SLOT_SAVE_STREAM, 1);
        let watcher = seen.clone();
        fx.e.register_double(save_stream, move |e, _| {
            watcher.set(e.mem.u32(ns + 0x250));
            1u32.into_ret()
        });
        assert!(ni_stream_save(&mut fx.e, fx.ns, Ptr::new(0x1234)));
        assert_eq!(seen.get(), 0x1234);
        assert_eq!(fx.get(0x250), 0);
    }

    #[test]
    fn set_texture_passes_the_texture_and_zero() {
        let mut fx = Fx::new();
        fx.rec(TEXTURE_PALETTE_SET_TEXTURE, 0);
        ni_stream_set_texture(&mut fx.e, fx.ns, 0x55);
        assert_eq!(fx.calls(TEXTURE_PALETTE_SET_TEXTURE), vec![vec![0x55, 0]]);
    }

    #[test]
    fn get_texture_returns_the_palette_answer() {
        for answer in [1u32, 0] {
            let mut fx = Fx::new();
            fx.rec(TEXTURE_PALETTE_GET_TEXTURE, answer);
            let got = ni_stream_get_texture(&mut fx.e, fx.ns, 0x55, Ptr::new(0x66));
            assert_eq!(got, answer == 1);
            assert_eq!(
                fx.calls(TEXTURE_PALETTE_GET_TEXTURE),
                vec![vec![0x55, 0x66]]
            );
        }
    }

    #[test]
    fn fixed_string_index_finds_the_handle() {
        let mut fx = Fx::new();
        let (a, b, c) = (fx.handle("a"), fx.handle("b"), fx.handle("c"));
        fx.array(fx.at(0x234), &[fx.empty, a, b], 4);
        for (handle, index) in [(a, 1), (b, 2), (c, u32::MAX), (fx.empty, u32::MAX)] {
            let string = Ptr::new(fx.cell(handle));
            assert_eq!(fn_00a633f0(&mut fx.e, fx.ns, string), index);
        }
    }

    #[test]
    fn object_from_link_id_takes_the_next_id() {
        let mut fx = Fx::new();
        fx.set_of(fx.at(0x254), &[1, u32::MAX]);
        fx.pointer_array(fx.at(0x1ec), &[0xa, 0xb], 2);
        assert_eq!(ni_stream_get_object_from_link_id(&mut fx.e, fx.ns), 0xb);
        assert_eq!(ni_stream_get_object_from_link_id(&mut fx.e, fx.ns), 0);
        assert_eq!(fx.get(0x260), 2);
    }

    #[test]
    fn number_of_link_ids_takes_the_next_block_size() {
        let mut fx = Fx::new();
        fx.set_of(fx.at(0x264), &[3, 5]);
        assert_eq!(ni_stream_get_number_of_link_ids(&mut fx.e, fx.ns), 3);
        assert_eq!(ni_stream_get_number_of_link_ids(&mut fx.e, fx.ns), 5);
        assert_eq!(fx.get(0x270), 2);
    }

    #[test]
    fn link_id_from_object_asks_the_register_map() {
        let mut fx = Fx::new();
        fx.e.register_double(MAP_GET_AT, |e, a| {
            if a[1] == 0x77 {
                e.mem.set_u32(a[2], 7);
                return 1u32.into_ret();
            }
            0u32.into_ret()
        });
        assert_eq!(
            ni_stream_get_link_id_from_object(&mut fx.e, fx.ns, 0),
            u32::MAX
        );
        assert_eq!(ni_stream_get_link_id_from_object(&mut fx.e, fx.ns, 0x77), 7);
        assert_eq!(
            ni_stream_get_link_id_from_object(&mut fx.e, fx.ns, 0x88),
            u32::MAX
        );
    }

    #[test]
    fn create_by_name_calls_the_creation_function() {
        let mut fx = Fx::new();
        fx.e.mem.set_u32(LOADERS, 0x1234);
        fx.install_lookup(&[("NiNode", CREATE_FIRST)]);
        fx.rec(CREATE_FIRST, 0x55);
        let name = fx.cs("NiNode");
        assert_eq!(fn_00a634b0(&mut fx.e, name), 0x55);
        assert_eq!(fx.calls(MAP_GET_AT)[0][..2], [0x1234, name]);
        let other = fx.cs("Other");
        assert_eq!(fn_00a634b0(&mut fx.e, other), 0);
        assert_eq!(fx.calls(CREATE_FIRST).len(), 1);
    }

    #[test]
    fn group_from_id_indexes_the_groups() {
        let mut fx = Fx::new();
        fx.small_array(fx.at(0xc8), &[0, 0x11, 0x22], 4);
        assert_eq!(ni_stream_get_group_from_id(&mut fx.e, fx.ns, 2), 0x22);
    }

    #[test]
    fn group_index_is_zero_when_absent() {
        let mut fx = Fx::new();
        fx.small_array(fx.at(0xc8), &[0x10, 0x11, 0x22], 4);
        assert_eq!(fn_00a634f0(&mut fx.e, fx.ns, 0x22), 2);
        assert_eq!(fn_00a634f0(&mut fx.e, fx.ns, 0x11), 1);
        assert_eq!(fn_00a634f0(&mut fx.e, fx.ns, 0x10), 0);
        assert_eq!(fn_00a634f0(&mut fx.e, fx.ns, 0x99), 0);
    }

    #[test]
    fn remove_loader_passes_the_map_and_name() {
        let mut fx = Fx::new();
        fx.e.mem.set_u32(LOADERS, 0x1234);
        fx.rec(LOADER_MAP_REMOVE_AT, 9);
        assert_eq!(fn_00a63520(&mut fx.e, 0x55), 9);
        assert_eq!(fx.calls(LOADER_MAP_REMOVE_AT), vec![vec![0x1234, 0x55]]);
    }

    #[test]
    fn remove_post_process_function_clears_its_slot() {
        let mut fx = Fx::new();
        let array = fx.e.mem.alloc(0x10);
        fx.e.mem.set_u32(POST_PROCESS_FUNCTIONS, array);
        let base = fx.small_array(array, &[0x10, 0x20, 0x30], 4);
        fn_00a63540(&mut fx.e, 0x30);
        assert_eq!(fx.word(base, 2), 0);
        assert_eq!(fx.e.mem.u16(array + 0xc), 2);
        assert_eq!(fx.e.mem.u16(array + 0xa), 2);
        // A slot before the last one leaves the size alone.
        fn_00a63540(&mut fx.e, 0x10);
        assert_eq!(fx.word(base, 0), 0);
        assert_eq!(fx.e.mem.u16(array + 0xc), 1);
        assert_eq!(fx.e.mem.u16(array + 0xa), 2);
        // A function that is not there changes nothing.
        fn_00a63540(&mut fx.e, 0x99);
        assert_eq!(fx.e.mem.u16(array + 0xc), 1);
        assert_eq!(fx.word(base, 1), 0x20);
    }

    #[test]
    fn register_objects_asks_each_top_level_object() {
        let mut fx = Fx::new();
        let (first, second) = (fx.object(), fx.object());
        let a = fx.method(first, OBJECT_SLOT_REGISTER_STREAMABLES, 0);
        let b = fx.method(second, OBJECT_SLOT_REGISTER_STREAMABLES, 0);
        fx.pointer_array(fx.at(0x21c), &[first, second], 2);
        ni_stream_register_objects(&mut fx.e, fx.ns);
        assert_eq!(fx.calls(a), vec![vec![first, fx.ns.addr()]]);
        assert_eq!(fx.calls(b), vec![vec![second, fx.ns.addr()]]);
    }

    #[test]
    fn save_to_a_buffer_returns_the_memory_stream_buffer() {
        let mut fx = Fx::new();
        fx.rec(NI_MEM_STREAM_CONSTRUCT_EMPTY, 0);
        fx.rec(NI_MEM_STREAM_DESTRUCT, 0);
        fx.rec(NI_MEM_STREAM_TAKE_BUFFER, 0x1234);
        let save = fx.ns_method(SLOT_SAVE, 1);
        fx.e.register_double(save, |e, a| {
            e.mem.set_u32(a[1] + 0x18, 0x40);
            1u32.into_ret()
        });
        let (buffer, size) = (fx.cell(0), fx.cell(0));
        assert!(ni_stream_save_ov2(
            &mut fx.e,
            fx.ns,
            Ptr::new(buffer),
            Ptr::new(size)
        ));
        assert_eq!(fx.e.mem.u32(buffer), 0x1234);
        assert_eq!(fx.e.mem.u32(size), 0x40);
        let stream = fx.calls(NI_MEM_STREAM_CONSTRUCT_EMPTY)[0][0];
        assert_eq!(fx.calls(NI_MEM_STREAM_TAKE_BUFFER), vec![vec![stream]]);
        assert_eq!(fx.calls(NI_MEM_STREAM_DESTRUCT), vec![vec![stream]]);
    }

    #[test]
    fn version_from_string_packs_the_numbers() {
        let mut fx = Fx::new();
        fx.e.register(STRTOK_S, |e, a| {
            let mut at = if a[0] != 0 { a[0] } else { e.mem.u32(a[2]) };
            while e.mem.u8(at) == b'.' {
                at += 1;
            }
            if e.mem.u8(at) == 0 {
                return 0u32.into_ret();
            }
            let start = at;
            while e.mem.u8(at) != 0 && e.mem.u8(at) != b'.' {
                at += 1;
            }
            if e.mem.u8(at) == b'.' {
                e.mem.set_u8(at, 0);
                at += 1;
            }
            e.mem.set_u32(a[2], at);
            start.into_ret()
        });
        fx.e.mem.set_cstr(TEXT_DOT, b".");
        let text = fx.cs("20.2.0.7");
        assert_eq!(
            ni_stream_get_version_from_string(&mut fx.e, Ptr::new(text)),
            0x1402_0007
        );
        let text = fx.cs("4.2.2.0");
        assert_eq!(
            ni_stream_get_version_from_string(&mut fx.e, Ptr::new(text)),
            0x0402_0200
        );
    }

    #[test]
    fn default_path_is_handed_out_once() {
        let mut fx = Fx::new();
        let path = fx.at(0x4c0);
        fx.e.mem.set_cstr(path, b"dir\\a.nif");
        let buffer = fx.e.mem.alloc(0x104);
        fx.e.mem.set_u8(USE_DEFAULT_PATH, 0);
        assert!(!fn_00a636b0(&mut fx.e, fx.ns, Ptr::new(buffer), 0));
        assert_eq!(fx.e.mem.cstr(buffer), b"");
        fx.e.mem.set_u8(USE_DEFAULT_PATH, 1);
        assert!(fn_00a636b0(&mut fx.e, fx.ns, Ptr::new(buffer), 0));
        assert_eq!(fx.e.mem.cstr(buffer), b"dir\\a.nif");
        assert_eq!(fx.e.mem.u8(USE_DEFAULT_PATH), 0);
    }

    #[test]
    fn set_file_path_standardizes_a_non_empty_path() {
        let mut fx = Fx::new();
        fx.rec(NI_PATH_STANDARDIZE, 0);
        let path = fx.at(0x4c0);
        let text = fx.cs("x.nif");
        fn_00a636f0(&mut fx.e, fx.ns, Ptr::new(text));
        assert_eq!(fx.e.mem.cstr(path), b"x.nif");
        assert_eq!(fx.calls(NI_PATH_STANDARDIZE), vec![vec![path]]);
        // An empty or null path empties the stored one.
        let empty = fx.cs("");
        fn_00a636f0(&mut fx.e, fx.ns, Ptr::new(empty));
        assert_eq!(fx.e.mem.u8(path), 0);
        fx.e.mem.set_cstr(path, b"keep");
        fn_00a636f0(&mut fx.e, fx.ns, Ptr::NULL);
        assert_eq!(fx.e.mem.u8(path), 0);
        assert_eq!(fx.calls(NI_PATH_STANDARDIZE).len(), 1);
    }

    #[test]
    fn fixed_string_set_at_keeps_counts_and_references() {
        let mut fx = Fx::new();
        let (h1, h2, h3) = (fx.handle("a"), fx.handle("b"), fx.handle("c"));
        let at = fx.at(0x234);
        let base = fx.array(at, &[fx.empty, h1], 8);
        let array = Ptr::new(at);
        // Past the used size: the size grows and the count goes up.
        let value = fx.cell(h2);
        fn_00a63730(&mut fx.e, array, 3, Ptr::new(value));
        assert_eq!(fx.e.mem.u32(at + 0xc), 4);
        assert_eq!(fx.e.mem.u32(at + 0x10), 2);
        assert_eq!(fx.word(base, 3), h2);
        assert_eq!(fx.refs(h2), 2);
        // Over a non-empty element: the counts stay, references move.
        let value = fx.cell(h3);
        fn_00a63730(&mut fx.e, array, 1, Ptr::new(value));
        assert_eq!(fx.e.mem.u32(at + 0x10), 2);
        assert_eq!((fx.refs(h3), fx.refs(h1)), (2, 0));
        // The empty string over a non-empty element: the count goes down.
        let value = fx.cell(fx.empty);
        fn_00a63730(&mut fx.e, array, 1, Ptr::new(value));
        assert_eq!(fx.e.mem.u32(at + 0x10), 1);
        assert_eq!(fx.word(base, 1), fx.empty);
        assert_eq!(fx.refs(h3), 1);
        // A non-empty string over an empty element: the count goes up.
        let value = fx.cell(h3);
        fn_00a63730(&mut fx.e, array, 0, Ptr::new(value));
        assert_eq!(fx.e.mem.u32(at + 0x10), 2);
    }

    #[test]
    fn fixed_string_remove_all_releases_every_string() {
        let mut fx = Fx::new();
        let (h1, h2) = (fx.handle("a"), fx.handle("b"));
        let at = fx.at(0x234);
        let base = fx.array(at, &[h1, fx.empty, h2], 4);
        fn_00a637d0(&mut fx.e, Ptr::new(at));
        assert_eq!((fx.refs(h1), fx.refs(h2)), (0, 0));
        assert_eq!(fx.word(base, 0), fx.empty);
        assert_eq!(fx.word(base, 2), fx.empty);
        assert_eq!((fx.e.mem.u32(at + 0xc), fx.e.mem.u32(at + 0x10)), (0, 0));
        // An empty array stays empty.
        fn_00a637d0(&mut fx.e, Ptr::new(at));
        assert_eq!(fx.e.mem.u32(at + 0xc), 0);
    }

    #[test]
    fn hash_is_the_key_modulo_the_bucket_count() {
        let mut fx = Fx::new();
        let (map, _) = fx.map_object(0x25);
        assert_eq!(fn_00a63840(&mut fx.e, Ptr::new(map), 100), 100 % 0x25);
        assert_eq!(fn_00a63840(&mut fx.e, Ptr::new(map), 0x25), 0);
    }

    /// A map of `buckets` buckets whose one bucket holds two nodes with
    /// allocated keys; the byte at `+0x10` says whether it copies keys.
    fn map_with_keys(fx: &mut Fx, copies: bool) -> (u32, u32, [u32; 2]) {
        let (map, table) = fx.map_object(3);
        let keys = [fx.cs("first"), fx.cs("second")];
        let second = fx.e.mem.alloc(0x10);
        fx.e.mem.set_u32(second + 4, keys[1]);
        let first = fx.e.mem.alloc(0x10);
        fx.e.mem.set_u32(first, second);
        fx.e.mem.set_u32(first + 4, keys[0]);
        fx.e.mem.set_u32(table + 4, first);
        fx.e.mem.set_u8(map + 0x10, copies as u8);
        (map, table, keys)
    }

    #[test]
    fn loader_map_destructor_frees_copied_keys() {
        let mut fx = Fx::new();
        let seen = fx.watch_remove_all();
        let (map, table, keys) = map_with_keys(&mut fx, true);
        fn_00a63850(&mut fx.e, Ptr::new(map));
        assert_eq!(
            *seen.borrow(),
            [
                LOADER_MAP_MIDDLE_VTABLE_109BCA4,
                LOADER_MAP_ROOT_VTABLE_109BC84
            ]
        );
        assert_eq!(fx.e.mem.u32(map), LOADER_MAP_ROOT_VTABLE_109BC84);
        assert!(keys.iter().all(|k| fx.e.mem.block_size(*k).is_none()));
        assert!(fx.e.mem.block_size(table).is_none());
        // A map that does not copy keys leaves them alone.
        let mut fx = Fx::new();
        fx.watch_remove_all();
        let (map, table, keys) = map_with_keys(&mut fx, false);
        fn_00a63850(&mut fx.e, Ptr::new(map));
        assert!(keys.iter().all(|k| fx.e.mem.block_size(*k).is_some()));
        assert!(fx.e.mem.block_size(table).is_none());
    }

    #[test]
    fn delete_item_returns_the_node_to_the_pool() {
        let mut fx = Fx::new();
        fx.rec(POOL_LOCK_FUNCTION, 0);
        fx.e.mem.set_u32(POOL_FREE_LIST, 0x55);
        fx.e.mem.set_u32(POOL_LOCK_COUNT, 2);
        fx.e.mem.set_u32(POOL_LOCK, 0x77);
        let (map, _) = fx.map_object(3);
        let item = fx.e.mem.alloc(0x10);
        fx.e.mem.set_u32(item + 4, 5);
        fx.e.mem.set_u16(item + 8, 3);
        ni_t_pointer_map_delete_item(&mut fx.e, Ptr::new(map), Ptr::new(item));
        assert_eq!(fx.calls(POOL_LOCK_FUNCTION), vec![vec![POOL_LOCK, 0]]);
        assert_eq!((fx.e.mem.u32(item + 4), fx.e.mem.u16(item + 8)), (0, 0));
        assert_eq!(fx.e.mem.u32(item), 0x55);
        assert_eq!(fx.e.mem.u32(POOL_FREE_LIST), item);
        assert_eq!(
            (fx.e.mem.u32(POOL_LOCK_COUNT), fx.e.mem.u32(POOL_LOCK)),
            (1, 0x77)
        );
        // The last user clears the owner.
        ni_t_pointer_map_delete_item(&mut fx.e, Ptr::new(map), Ptr::new(item));
        assert_eq!(
            (fx.e.mem.u32(POOL_LOCK_COUNT), fx.e.mem.u32(POOL_LOCK)),
            (0, 0)
        );
    }

    #[test]
    fn name_map_destructor_frees_copied_keys() {
        let mut fx = Fx::new();
        let seen = fx.watch_remove_all();
        let (map, table, keys) = map_with_keys(&mut fx, true);
        fn_00a63910(&mut fx.e, Ptr::new(map));
        assert_eq!(
            *seen.borrow(),
            [NAME_MAP_MIDDLE_VTABLE_109BD0C, NAME_MAP_ROOT_VTABLE_109BCEC]
        );
        assert_eq!(fx.e.mem.u32(map), NAME_MAP_ROOT_VTABLE_109BCEC);
        assert!(keys.iter().all(|k| fx.e.mem.block_size(*k).is_none()));
        assert!(fx.e.mem.block_size(table).is_none());
    }

    #[test]
    fn set_value_copies_the_key_only_when_the_map_does() {
        let mut fx = Fx::new();
        let (map, _) = fx.map_object(3);
        let item = fx.e.mem.alloc(0x10);
        let key = fx.cs("abc");
        fx.e.mem.set_u8(map + 0x10, 1);
        ni_t_string_template_map_set_value(&mut fx.e, Ptr::new(map), Ptr::new(item), key, 9);
        let copy = fx.e.mem.u32(item + 4);
        assert_ne!(copy, key);
        assert_eq!(fx.e.mem.cstr(copy), b"abc");
        assert_eq!(fx.e.mem.u16(item + 8), 9);
        fx.e.mem.set_u8(map + 0x10, 0);
        ni_t_string_template_map_set_value(&mut fx.e, Ptr::new(map), Ptr::new(item), key, 4);
        assert_eq!(fx.e.mem.u32(item + 4), key);
        assert_eq!(fx.e.mem.u16(item + 8), 4);
    }

    /// Runs a scalar deleting destructor of a map with and without the
    /// delete flag and checks the virtual tables `RemoveAll` saw.
    fn check_map_delete(
        destroy: fn(&mut Engine, Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>,
        tables: &[u32],
    ) {
        for flags in [0u32, 1] {
            let mut fx = Fx::new();
            let seen = fx.watch_remove_all();
            let (map, buckets) = fx.map_object(3);
            let result = destroy(&mut fx.e, Ptr::new(map), flags);
            assert_eq!(result.addr(), map);
            assert_eq!(*seen.borrow(), tables);
            assert!(fx.e.mem.block_size(buckets).is_none());
            assert_eq!(fx.e.mem.block_size(map).is_none(), flags == 1);
            assert_eq!(fx.e.mem.u32(map + 4), 3);
        }
    }

    #[test]
    fn root_map_deleting_destructor() {
        check_map_delete(fn_00a639e0, &[LOADER_MAP_ROOT_VTABLE_109BC84]);
    }

    #[test]
    fn middle_map_deleting_destructor() {
        check_map_delete(
            fn_00a63a20,
            &[
                LOADER_MAP_MIDDLE_VTABLE_109BCA4,
                LOADER_MAP_ROOT_VTABLE_109BC84,
            ],
        );
    }

    #[test]
    fn name_map_root_deleting_destructor() {
        check_map_delete(
            ni_t_map_base_scalar_deleting_destructor,
            &[NAME_MAP_ROOT_VTABLE_109BCEC],
        );
    }

    #[test]
    fn name_map_middle_deleting_destructor() {
        check_map_delete(
            ni_t_pointer_map_scalar_deleting_destructor,
            &[NAME_MAP_MIDDLE_VTABLE_109BD0C, NAME_MAP_ROOT_VTABLE_109BCEC],
        );
    }

    #[test]
    fn register_map_base_deleting_destructor() {
        check_map_delete(fn_00a63b60, &[REGISTER_MAP_BASE_VTABLE_109BC64]);
    }

    #[test]
    fn register_map_deleting_destructor() {
        check_map_delete(
            fn_00a63ba0,
            &[
                REGISTER_MAP_VTABLE_109BD54,
                REGISTER_MAP_BASE_VTABLE_109BC64,
            ],
        );
    }

    #[test]
    fn loader_map_deleting_destructor_runs_the_body() {
        check_map_delete(
            fn_00a66030,
            &[
                LOADER_MAP_MIDDLE_VTABLE_109BCA4,
                LOADER_MAP_ROOT_VTABLE_109BC84,
            ],
        );
    }

    #[test]
    fn name_map_deleting_destructor_runs_the_body() {
        check_map_delete(
            fn_00a66050,
            &[NAME_MAP_MIDDLE_VTABLE_109BD0C, NAME_MAP_ROOT_VTABLE_109BCEC],
        );
    }

    #[test]
    fn fixed_string_array_allocation_counts_its_elements() {
        let mut fx = Fx::new();
        let sizes: Rc<RefCell<Vec<u32>>> = Rc::default();
        let list = sizes.clone();
        fx.e.register_double(MEMORY_MANAGER_ALLOCATE, move |e, a| {
            list.borrow_mut().push(a[1]);
            if a[1] == u32::MAX {
                return 0u32.into_ret();
            }
            e.mem.alloc(a[1]).into_ret()
        });
        let first = fn_00a63ae0(&mut fx.e, 3);
        assert_eq!(*sizes.borrow(), [16]);
        assert_eq!(fx.e.mem.u32(first - 4), 3);
        assert_eq!(fx.word(first, 2), fx.empty);
        // A count whose byte size overflows asks for the largest size and
        // gets nothing.
        assert_eq!(fn_00a63ae0(&mut fx.e, 0x4000_0000), 0);
        assert_eq!(fn_00a63ae0(&mut fx.e, 0x3fff_ffff), 0);
        assert_eq!(sizes.borrow()[1..], [u32::MAX, u32::MAX]);
    }

    #[test]
    fn groups_base_destructor_frees_the_storage() {
        for flags in [0u32, 1] {
            let mut fx = Fx::new();
            let array = fx.e.mem.alloc(0x10);
            let storage = fx.e.mem.alloc(0x10);
            fx.e.mem.set_u32(array + 4, storage);
            let result = fn_00a63b30(&mut fx.e, Ptr::new(array), flags);
            assert_eq!(result.addr(), array);
            assert_eq!(fx.e.mem.u32(array), GROUPS_ARRAY_BASE_VTABLE_109BC3C);
            assert!(fx.e.mem.block_size(storage).is_none());
            assert_eq!(fx.e.mem.block_size(array).is_none(), flags == 1);
        }
    }

    #[test]
    fn object_array_destructor_frees_the_storage() {
        for flags in [0u32, 1] {
            let mut fx = Fx::new();
            fx.rec(OBJECT_ARRAY_FREE_STORAGE, 0);
            let array = fx.e.mem.alloc(0x18);
            fx.e.mem.set_u32(array + 4, 0x4321);
            let result = fn_00a652d0(&mut fx.e, Ptr::new(array), flags);
            assert_eq!(result.addr(), array);
            assert_eq!(fx.e.mem.u32(array), OBJECT_ARRAY_BASE_VTABLE_109BD98);
            assert_eq!(fx.calls(OBJECT_ARRAY_FREE_STORAGE), vec![vec![0x4321]]);
            assert_eq!(fx.e.mem.block_size(array).is_none(), flags == 1);
        }
    }

    #[test]
    fn fixed_string_array_destructor_frees_the_storage() {
        for flags in [0u32, 1] {
            let mut fx = Fx::new();
            fx.rec(FIXED_STRINGS_FREE_STORAGE, 0);
            let array = fx.e.mem.alloc(0x18);
            fx.e.mem.set_u32(array + 4, 0x4321);
            let result = fn_00a66070(&mut fx.e, Ptr::new(array), flags);
            assert_eq!(result.addr(), array);
            assert_eq!(fx.e.mem.u32(array), FIXED_STRINGS_ARRAY_BASE_VTABLE_109BD4C);
            assert_eq!(fx.calls(FIXED_STRINGS_FREE_STORAGE), vec![vec![0x4321]]);
            assert_eq!(fx.e.mem.block_size(array).is_none(), flags == 1);
        }
    }

    // -----------------------------------------------------------------
    // Files, the object graph and the save side.

    /// A file object whose virtual functions are doubles: slot 4 says
    /// whether it is usable, slot 0 is the deleting destructor.
    fn file_object(fx: &mut Fx, usable: u32) -> (u32, u32) {
        let file = fx.object();
        fx.method(file, STREAM_SLOT_IS_VALID, usable);
        let delete = fx.method(file, 0, 0);
        (file, delete)
    }

    #[test]
    fn load_from_a_file_opens_it_and_loads() {
        let mut fx = Fx::new();
        fx.rec(NI_PATH_STANDARDIZE, 0);
        let (file, delete) = file_object(&mut fx, 1);
        fx.rec(NI_FILE_GET_FILE, file);
        let load = fx.ns_method(SLOT_LOAD, 1);
        let path = fx.cs("m\\a.nif");
        assert!(ni_stream_load_ov3(&mut fx.e, fx.ns, Ptr::new(path)));
        let name = fx.at(0xe0);
        assert_eq!(fx.e.mem.cstr(name), b"m\\a.nif");
        assert_eq!(fx.e.mem.cstr(fx.at(0x4c0)), b"m\\a.nif");
        assert_eq!(fx.calls(NI_FILE_GET_FILE), vec![vec![name, 0, 0x8000]]);
        assert_eq!(fx.calls(load), vec![vec![fx.ns.addr(), file]]);
        assert_eq!(fx.calls(delete), vec![vec![file, 1]]);
        assert_eq!(fx.get(0x3b8), 0);
    }

    #[test]
    fn load_from_a_file_reports_a_file_that_does_not_open() {
        let mut fx = Fx::new();
        fx.rec(NI_PATH_STANDARDIZE, 0);
        fx.e.mem
            .set_cstr(TEXT_CANNOT_OPEN_FILE, b"Cannot open file.");
        fx.rec(NI_FILE_GET_FILE, 0);
        let load = fx.ns_method(SLOT_LOAD, 1);
        let path = fx.cs("");
        assert!(!ni_stream_load_ov3(&mut fx.e, fx.ns, Ptr::new(path)));
        assert_eq!(fx.get(0x3b8), 1);
        assert_eq!(fx.e.mem.cstr(fx.at(0x3bc)), b"Cannot open file.");
        assert!(fx.calls(load).is_empty());
        // An empty path leaves the file path empty (and is not
        // standardized a second time).
        assert_eq!(fx.e.mem.u8(fx.at(0x4c0)), 0);
        assert_eq!(fx.calls(NI_PATH_STANDARDIZE).len(), 1);
    }

    #[test]
    fn load_from_a_file_deletes_an_unusable_file() {
        let mut fx = Fx::new();
        fx.rec(NI_PATH_STANDARDIZE, 0);
        fx.e.mem
            .set_cstr(TEXT_CANNOT_OPEN_FILE, b"Cannot open file.");
        let (file, delete) = file_object(&mut fx, 0);
        fx.rec(NI_FILE_GET_FILE, file);
        let load = fx.ns_method(SLOT_LOAD, 1);
        let path = fx.cs("a.nif");
        assert!(!ni_stream_load_ov3(&mut fx.e, fx.ns, Ptr::new(path)));
        assert_eq!(fx.calls(delete), vec![vec![file, 1]]);
        assert_eq!(fx.get(0x3b8), 1);
        assert!(fx.calls(load).is_empty());
    }

    #[test]
    fn save_to_a_file_opens_it_for_writing() {
        let mut fx = Fx::new();
        fx.rec(NI_PATH_STANDARDIZE, 0);
        let (file, delete) = file_object(&mut fx, 1);
        fx.rec(NI_FILE_GET_FILE, file);
        let save = fx.ns_method(SLOT_SAVE, 1);
        let path = fx.cs("m\\a.nif");
        assert!(ni_stream_save_ov3(&mut fx.e, fx.ns, Ptr::new(path)));
        let name = fx.at(0xe0);
        assert_eq!(fx.calls(NI_FILE_GET_FILE), vec![vec![name, 1, 0x8000]]);
        assert_eq!(fx.calls(save), vec![vec![fx.ns.addr(), file]]);
        assert_eq!(fx.calls(delete), vec![vec![file, 1]]);
        assert_eq!(fx.e.mem.cstr(fx.at(0x4c0)), b"m\\a.nif");
    }

    #[test]
    fn save_to_a_file_fails_quietly() {
        // No file.
        let mut fx = Fx::new();
        fx.rec(NI_PATH_STANDARDIZE, 0);
        fx.rec(NI_FILE_GET_FILE, 0);
        let path = fx.cs("a.nif");
        assert!(!ni_stream_save_ov3(&mut fx.e, fx.ns, Ptr::new(path)));
        assert_eq!(fx.get(0x3b8), 0);
        // A file that is not usable is deleted.
        let mut fx = Fx::new();
        fx.rec(NI_PATH_STANDARDIZE, 0);
        let (file, delete) = file_object(&mut fx, 0);
        fx.rec(NI_FILE_GET_FILE, file);
        let save = fx.ns_method(SLOT_SAVE, 1);
        let path = fx.cs("a.nif");
        assert!(!ni_stream_save_ov3(&mut fx.e, fx.ns, Ptr::new(path)));
        assert_eq!(fx.calls(delete), vec![vec![file, 1]]);
        assert!(fx.calls(save).is_empty());
        assert_eq!(fx.get(0x3b8), 0);
    }

    #[test]
    fn mark_node_flags_walks_the_children() {
        let mut fx = Fx::new();
        let (parent, node, child, other) = (fx.object(), fx.object(), fx.object(), fx.object());
        fx.method(parent, OBJECT_SLOT_AS_NODE, node);
        fx.method(child, OBJECT_SLOT_AS_NODE, 0);
        fx.method(other, OBJECT_SLOT_AS_NODE, 0);
        let children = fx.e.mem.alloc(0x10);
        fx.e.mem.set_u32(children, child);
        fx.e.mem.set_u32(children + 8, other);
        fx.e.mem.set_u32(node + 0xa0, children);
        fx.e.mem.set_u16(node + 0xa6, 3);
        for object in [parent, child, other] {
            fx.e.mem.set_u32(object + 0x30, 0x11);
        }
        fn_00a63cb0(&mut fx.e, fx.ns, Ptr::new(parent));
        for object in [parent, child, other] {
            assert_eq!(fx.e.mem.u32(object + 0x30), 0x0f);
        }
    }

    #[test]
    fn old_files_get_their_nodes_marked() {
        for (version, marked) in [(0x0401_000b, true), (0x0401_000c, false)] {
            let mut fx = Fx::new();
            let (plain, skipped, other) = (fx.object(), fx.object(), fx.object());
            let node_class = fx.e.mem.alloc(8);
            fx.e.mem.set_u32(node_class + 4, NODE_CLASS_RECORD);
            let other_class = fx.e.mem.alloc(8);
            for object in [plain, skipped] {
                fx.method(object, OBJECT_SLOT_GET_RTTI, node_class);
                fx.method(object, OBJECT_SLOT_AS_NODE, 0);
            }
            fx.method(other, OBJECT_SLOT_GET_RTTI, other_class);
            fx.e.mem.set_u32(skipped + 0x18, 5);
            fx.pointer_array(fx.at(0x1ec), &[plain, 0, skipped, other], 4);
            fx.put(0xd8, version);
            fn_00a63d10(&mut fx.e, fx.ns);
            assert_eq!(fx.e.mem.u32(plain + 0x30) != 0, marked);
            assert_eq!(fx.e.mem.u32(skipped + 0x30), 0);
            assert_eq!(fx.e.mem.u32(other + 0x30), 0);
        }
    }

    #[test]
    fn save_top_level_objects_writes_the_count_and_the_ids() {
        let mut fx = Fx::new();
        let save_id = fx.ns_method(SLOT_SAVE_LINK_ID, 0);
        fx.pointer_array(fx.at(0x21c), &[0x11, 0x22], 2);
        ni_stream_save_top_level_objects(&mut fx.e, fx.ns);
        assert_eq!(fx.written(), words(&[2]));
        let ns = fx.ns.addr();
        assert_eq!(fx.calls(save_id), vec![vec![ns, 0x11], vec![ns, 0x22]]);
    }

    #[test]
    fn pre_save_reserves_room_for_the_size_table() {
        let mut fx = Fx::new();
        fx.e.mem.set_u32(fx.bs + 4, 0x10);
        fx.pointer_array(fx.at(0x1ec), &[1, 2, 3], 3);
        assert_eq!(ni_stream_pre_save_object_size_table(&mut fx.e, fx.ns), 0x10);
        assert_eq!(fx.written(), words(&[0, 0, 0]));
    }

    #[test]
    fn save_object_size_table_goes_back_and_writes_the_sizes() {
        let mut fx = Fx::new();
        fx.e.mem.set_u32(fx.bs + 4, 0x30);
        fx.array(fx.at(0x204), &[4, 8], 2);
        assert!(ni_stream_save_object_size_table(&mut fx.e, fx.ns, 0x10));
        assert_eq!(fx.calls(FAKE_SEEK), vec![vec![fx.bs, 0xffff_ffe0]]);
        assert_eq!(fx.written(), words(&[4, 8]));
    }

    #[test]
    fn save_fixed_string_table_writes_lengths_and_characters() {
        let mut fx = Fx::new();
        let (ab, c) = (fx.handle("ab"), fx.handle("c"));
        fx.array(fx.at(0x234), &[ab, fx.empty, c], 3);
        fn_00a63ee0(&mut fx.e, fx.ns);
        let mut expected = words(&[3, 2, 2]);
        expected.extend(b"ab");
        expected.extend(words(&[0, 1]));
        expected.extend(b"c");
        assert_eq!(fx.written(), expected);
    }

    #[test]
    fn save_groups_writes_their_count_and_sizes() {
        let mut fx = Fx::new();
        let (first, second) = (fx.e.mem.alloc(8), fx.e.mem.alloc(8));
        fx.e.mem.set_u32(first, 5);
        fx.e.mem.set_u32(second, 9);
        fx.small_array(fx.at(0xc8), &[0, first, second], 3);
        fn_00a64010(&mut fx.e, fx.ns);
        assert_eq!(fx.written(), words(&[2, 5, 9]));
        // Only the unused element 0: nothing but the count.
        let mut fx = Fx::new();
        fx.small_array(fx.at(0xc8), &[0], 1);
        fn_00a64010(&mut fx.e, fx.ns);
        assert_eq!(fx.written(), words(&[0]));
    }

    #[test]
    fn save_c_string_writes_the_length_and_the_characters() {
        let mut fx = Fx::new();
        let text = fx.cs("abc");
        fn_00a64160(&mut fx.e, fx.ns, text);
        let mut expected = words(&[3]);
        expected.extend(b"abc");
        assert_eq!(fx.written(), expected);
        fx.output.borrow_mut().clear();
        fn_00a64160(&mut fx.e, fx.ns, 0);
        assert_eq!(fx.written(), words(&[0]));
        fx.output.borrow_mut().clear();
        let empty = fx.cs("");
        fn_00a64160(&mut fx.e, fx.ns, empty);
        assert_eq!(fx.written(), words(&[0]));
    }

    #[test]
    fn save_fixed_string_writes_its_index() {
        let mut fx = Fx::new();
        fx.e.mem.set_u32(NO_STRING_INDEX, u32::MAX);
        let (a, b) = (fx.handle("a"), fx.handle("b"));
        fx.array(fx.at(0x234), &[fx.empty, a, b], 3);
        let string = fx.cell(fx.empty);
        ni_stream_save_fixed_string(&mut fx.e, fx.ns, Ptr::new(string));
        assert_eq!(fx.written(), words(&[u32::MAX]));
        let string = fx.cell(b);
        ni_stream_save_fixed_string(&mut fx.e, fx.ns, Ptr::new(string));
        assert_eq!(fx.written()[4..], words(&[2]));
    }

    #[test]
    fn object_array_remove_all_drops_the_references() {
        let mut fx = Fx::new();
        let (last, shared) = (fx.object(), fx.object());
        fx.e.mem.set_u32(last + 4, 1);
        fx.e.mem.set_u32(shared + 4, 2);
        let delete = fx.method(last, SLOT_DELETE_THIS, 0);
        let kept = fx.method(shared, SLOT_DELETE_THIS, 0);
        let at = fx.at(0x1ec);
        let base = fx.pointer_array(at, &[last, 0, shared], 4);
        fn_00a64250(&mut fx.e, Ptr::new(at));
        assert_eq!(fx.calls(delete), vec![vec![last]]);
        assert!(fx.calls(kept).is_empty());
        assert_eq!(fx.e.mem.u32(shared + 4), 1);
        assert_eq!((fx.word(base, 0), fx.word(base, 2)), (0, 0));
        assert_eq!((fx.e.mem.u32(at + 0xc), fx.e.mem.u32(at + 0x10)), (0, 0));
    }

    /// Gives a map the virtual functions the tests need: hash (key modulo
    /// the bucket count), equality and `SetValue`, which store the key and
    /// value in the node.
    fn map_with_functions(fx: &mut Fx, copies: bool) -> (u32, u32) {
        let (map, table) = fx.map_object(3);
        let functions = fx.e.mem.alloc(0x20);
        fx.e.mem.set_u32(map, functions);
        fx.e.mem.set_u8(map + 0x10, copies as u8);
        fx.e.register(0x0720_0100, |_, a| (a[1] % 3).into_ret());
        fx.e.register(0x0720_0104, |_, a| {
            ((a[1] % 100 == a[2] % 100) as u32).into_ret()
        });
        fx.e.register(0x0720_0108, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u16(a[1] + 8, a[3] as u16);
            Ret::default()
        });
        for (slot, target) in [(4, 0x0720_0100), (8, 0x0720_0104), (0xc, 0x0720_0108)] {
            fx.e.mem.set_u32(functions + slot, target);
        }
        (map, table)
    }

    #[test]
    fn map_set_at_adds_a_node_or_updates_one() {
        let mut fx = Fx::new();
        let (map, table) = map_with_functions(&mut fx, false);
        let node = fx.e.mem.alloc(0x10);
        fx.rec(POOL_ALLOCATE_NODE, node);
        fn_00a643e0(&mut fx.e, Ptr::new(map), 4, 7);
        assert_eq!(fx.calls(POOL_ALLOCATE_NODE), vec![vec![map + 0xc]]);
        assert_eq!(fx.word(table, 1), node);
        assert_eq!((fx.e.mem.u32(node + 4), fx.e.mem.u16(node + 8)), (4, 7));
        assert_eq!(fx.e.mem.u32(map + 0xc), 1);
        // The same key again only changes the value.
        fn_00a643e0(&mut fx.e, Ptr::new(map), 4, 9);
        assert_eq!(fx.calls(POOL_ALLOCATE_NODE).len(), 1);
        assert_eq!(fx.e.mem.u16(node + 8), 9);
        assert_eq!(fx.e.mem.u32(map + 0xc), 1);
        // Another key in the same bucket goes in front.
        let second = fx.e.mem.alloc(0x10);
        fx.rec(POOL_ALLOCATE_NODE, second);
        fn_00a643e0(&mut fx.e, Ptr::new(map), 7, 1);
        assert_eq!(fx.word(table, 1), second);
        assert_eq!(fx.e.mem.u32(second), node);
        assert_eq!(fx.e.mem.u32(map + 0xc), 2);
    }

    #[test]
    fn map_set_at_keeps_the_key_of_a_map_that_copies_keys() {
        let mut fx = Fx::new();
        let (map, table) = map_with_functions(&mut fx, true);
        let node = fx.e.mem.alloc(0x10);
        // The key 104 is equal to 4 for the map's comparison.
        fx.e.mem.set_u32(node + 4, 104);
        fx.e.mem.set_u32(table + 4, node);
        fn_00a643e0(&mut fx.e, Ptr::new(map), 4, 5);
        assert_eq!(fx.e.mem.u32(node + 4), 104);
        assert_eq!(fx.e.mem.u16(node + 8), 5);
        // A map that does not copy keys stores the new one.
        let (map, table) = map_with_functions(&mut fx, false);
        fx.e.mem.set_u32(node + 4, 104);
        fx.e.mem.set_u32(table + 4, node);
        fn_00a643e0(&mut fx.e, Ptr::new(map), 4, 6);
        assert_eq!(fx.e.mem.u32(node + 4), 4);
    }

    #[test]
    fn remove_all_objects_empties_the_load_data() {
        let mut fx = Fx::new();
        fx.rec(ENTER_CRITICAL_SECTION, 0);
        fx.rec(LEAVE_CRITICAL_SECTION, 0);
        let object = fx.object();
        fx.e.mem.set_u32(object + 4, 1);
        let delete = fx.method(object, SLOT_DELETE_THIS, 0);
        fx.pointer_array(fx.at(0x21c), &[object], 2);
        let handle = fx.handle("a");
        fx.array(fx.at(0x234), &[handle], 2);
        let sizes = fx.array(fx.at(0x204), &[5, 6], 2);
        ni_stream_remove_all_objects(&mut fx.e, fx.ns);
        assert_eq!(fx.calls(delete), vec![vec![object]]);
        assert_eq!(fx.refs(handle), 0);
        assert_eq!((fx.word(sizes, 0), fx.word(sizes, 1)), (0, 0));
        for offset in [0x228, 0x240, 0x210, 0x214] {
            assert_eq!(fx.get(offset), 0);
        }
        assert_eq!(
            fx.calls(ENTER_CRITICAL_SECTION),
            vec![vec![CLEANUP_CRITICAL_SECTION]]
        );
        assert_eq!(
            fx.calls(LEAVE_CRITICAL_SECTION),
            vec![vec![CLEANUP_CRITICAL_SECTION]]
        );
    }

    #[test]
    fn change_object_stores_it_at_the_load_index() {
        let mut fx = Fx::new();
        let object = fx.object();
        fx.e.mem.set_u32(object + 4, 1);
        fx.put(0x294, 5);
        let during = Rc::new(Cell::new((0, 0)));
        let seen = during.clone();
        fx.e.register_double(LARGE_ARRAY_SET_AT, move |e, a| {
            seen.set((e.mem.u32(a[2]), e.mem.u32(object + 4)));
            // The array's own reference.
            e.mem.set_u32(object + 4, e.mem.u32(object + 4) + 1);
            assert_eq!(a[1], 5);
            Ret::default()
        });
        ni_stream_change_object(&mut fx.e, fx.ns, object);
        assert_eq!(during.get(), (object, 2));
        assert_eq!(fx.e.mem.u32(object + 4), 2);
    }

    /// Makes `SET_REALLOC` grow a set to the requested capacity.
    fn install_set_realloc(fx: &mut Fx) {
        fx.e.register(SET_REALLOC, |e, a| {
            let (set, capacity) = (a[0], a[1]);
            let old = e.mem.u32(set);
            let base = e.mem.alloc(capacity * 4);
            for i in 0..e.mem.u32(set + 8) {
                let value = e.mem.u32(old + i * 4);
                e.mem.set_u32(base + i * 4, value);
            }
            e.mem.set_u32(set, base);
            e.mem.set_u32(set + 4, capacity);
            Ret::default()
        });
    }

    #[test]
    fn read_link_id_adds_it_to_the_set() {
        let mut fx = Fx::new();
        install_set_realloc(&mut fx);
        fx.feed(&[0x1234, 0x5678]);
        ni_stream_read_link_id(&mut fx.e, fx.ns);
        ni_stream_read_link_id(&mut fx.e, fx.ns);
        let base = fx.get(0x254);
        assert_eq!((fx.get(0x258), fx.get(0x25c)), (2, 2));
        assert_eq!((fx.word(base, 0), fx.word(base, 1)), (0x1234, 0x5678));
    }

    #[test]
    fn resolve_link_id_finds_the_object() {
        let mut fx = Fx::new();
        fx.pointer_array(fx.at(0x1ec), &[0xa, 0xb], 2);
        fx.feed(&[1, u32::MAX]);
        assert_eq!(ni_stream_resolve_link_id(&mut fx.e, fx.ns), 0xb);
        assert_eq!(ni_stream_resolve_link_id(&mut fx.e, fx.ns), 0);
    }

    #[test]
    fn read_multiple_link_ids_reads_a_block() {
        let mut fx = Fx::new();
        install_set_realloc(&mut fx);
        fx.feed(&[2, 10, 20]);
        ni_stream_read_multiple_link_ids(&mut fx.e, fx.ns);
        let (blocks, ids) = (fx.get(0x264), fx.get(0x254));
        assert_eq!((fx.get(0x26c), fx.word(blocks, 0)), (1, 2));
        assert_eq!(fx.get(0x25c), 2);
        assert_eq!((fx.word(ids, 0), fx.word(ids, 1)), (10, 20));
    }

    /// `00a646f5`: the count read is left in `EAX` (`00a646d4`/`00a646e7`), which
    /// `NiFlipController::LoadBinary` keeps (`00a4f6b3`); an empty block answers 0.
    #[test]
    fn read_multiple_link_ids_answers_the_count() {
        let mut fx = Fx::new();
        install_set_realloc(&mut fx);
        fx.feed(&[2, 10, 20, 0]);
        assert_eq!(fx.e.call(0x00a6_4670, &args![fx.ns]).u32(), 2);
        assert_eq!(fx.e.call(0x00a6_4670, &args![fx.ns]).u32(), 0);
        assert_eq!(fx.get(0x25c), 2);
    }

    #[test]
    fn set_number_of_link_ids_adds_a_block_size() {
        let mut fx = Fx::new();
        install_set_realloc(&mut fx);
        ni_stream_set_number_of_link_ids(&mut fx.e, fx.ns, 6);
        let blocks = fx.get(0x264);
        assert_eq!((fx.get(0x26c), fx.word(blocks, 0)), (1, 6));
    }

    #[test]
    fn load_object_size_table_reads_one_size_per_object() {
        let mut fx = Fx::new();
        fx.rec(SIZES_ARRAY_SET_SIZE, 0);
        fx.pointer_array(fx.at(0x1ec), &[1, 2, 3], 3);
        let sizes = fx.array(fx.at(0x204), &[], 4);
        fx.feed(&[5, 0, 7]);
        assert!(ni_stream_load_object_size_table(&mut fx.e, fx.ns));
        assert_eq!(fx.calls(SIZES_ARRAY_SET_SIZE), vec![vec![fx.at(0x204), 3]]);
        assert_eq!(fx.get(0x210), 3);
        assert_eq!(fx.get(0x214), 2);
        assert_eq!([0, 1, 2].map(|i| fx.word(sizes, i)), [5, 0, 7]);
    }

    #[test]
    fn load_groups_creates_a_group_per_size() {
        let mut fx = Fx::new();
        fx.rec(GROUPS_SET_SIZE, 0);
        fx.rec(GROUPS_SET_AT, 0);
        fx.e.register_double(OBJECT_GROUP_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        let groups = fx.at(0xc8);
        let base = fx.small_array(groups, &[], 4);
        fx.feed(&[2, 10, 20]);
        fn_00a647f0(&mut fx.e, fx.ns);
        assert_eq!(fx.calls(GROUPS_SET_SIZE), vec![vec![groups, 3]]);
        assert_eq!(fx.calls(GROUPS_SET_AT).len(), 1);
        assert_eq!(fx.calls(GROUPS_SET_AT)[0][..2], [groups, 0]);
        let (first, second) = (fx.word(base, 1), fx.word(base, 2));
        assert_eq!((fx.e.mem.u32(first), fx.e.mem.u32(second)), (10, 20));
        assert_eq!(fx.e.mem.u16(groups + 0xa), 3);
        assert_eq!(fx.e.mem.u16(groups + 0xc), 2);
    }

    #[test]
    fn register_loader_adds_to_the_loader_map() {
        let mut fx = Fx::new();
        fx.e.mem.set_u32(LOADERS, 0x1234);
        fx.rec(LOADER_MAP_SET_AT, 1);
        assert_eq!(ni_stream_register_loader(&mut fx.e, 0x55, 0x66), 0);
        assert_eq!(fx.calls(LOADER_MAP_SET_AT), vec![vec![0x1234, 0x55, 0x66]]);
    }

    #[test]
    fn register_post_process_function_passes_the_function_by_address() {
        let mut fx = Fx::new();
        fx.e.mem.set_u32(POST_PROCESS_FUNCTIONS, 0x4321);
        let seen = Rc::new(Cell::new(0));
        let watcher = seen.clone();
        fx.e.register_double(POST_PROCESS_ADD, move |e, a| {
            assert_eq!(a[0], 0x4321);
            watcher.set(e.mem.u32(a[1]));
            3u32.into_ret()
        });
        assert_eq!(ni_stream_register_post_process_function(&mut fx.e, 0x77), 3);
        assert_eq!(seen.get(), 0x77);
    }

    /// Fills the strings the header writes.
    fn header_strings(fx: &mut Fx) {
        for (offset, text) in [(8, "a"), (0x48, "bc"), (0x88, "")] {
            let at = fx.at(offset);
            fx.e.mem.set_cstr(at, text.as_bytes());
        }
    }

    #[test]
    fn save_header_writes_the_versions_and_strings() {
        let mut fx = Fx::new();
        fx.rec(WRITE_STRING, 0);
        fx.e.mem.set_u32(NIF_MAX_VERSION, 0x1402_0007);
        fx.e.mem.set_u32(NIF_MAX_USER_VERSION, 11);
        fx.put(0x1e4, 1);
        fx.put(4, 0x22);
        header_strings(&mut fx);
        fx.pointer_array(fx.at(0x1ec), &[1, 2], 2);
        ni_stream_save_header(&mut fx.e, fx.ns);
        let mut expected = vec![7, 0, 2, 0x14, 1];
        expected.extend(words(&[11, 2, 0x22]));
        expected.extend([2, b'a', 0, 3, b'b', b'c', 0, 1, 0]);
        assert_eq!(fx.written(), expected);
        assert_eq!(fx.calls(WRITE_STRING), vec![vec![fx.bs, TEXT_HEADER_LINE]]);
        assert_eq!(fx.calls(FAKE_ENDIAN), vec![vec![fx.bs, 0], vec![fx.bs, 0]]);
    }

    #[test]
    fn save_header_without_user_version_has_no_extra_header() {
        let mut fx = Fx::new();
        fx.rec(WRITE_STRING, 0);
        fx.e.mem.set_u32(NIF_MAX_VERSION, 0x1402_0007);
        fx.e.mem.set_u32(NIF_MAX_USER_VERSION, 0);
        fx.put(0x1e4, 0);
        fx.pointer_array(fx.at(0x1ec), &[1], 1);
        ni_stream_save_header(&mut fx.e, fx.ns);
        let mut expected = vec![7, 0, 2, 0x14, 0];
        expected.extend(words(&[0, 1]));
        assert_eq!(fx.written(), expected);
        // Big endian output switches the swap on again at the end.
        assert_eq!(fx.calls(FAKE_ENDIAN), vec![vec![fx.bs, 0], vec![fx.bs, 1]]);
    }

    #[test]
    fn group_preparation_collects_groups_and_their_sizes() {
        let mut fx = Fx::new();
        fx.rec(GROUPS_SET_SIZE, 0);
        // SetAt: stores the group and grows the size like the array does.
        fx.e.register_double(GROUPS_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let group = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, group);
            if a[1] >= e.mem.u16(a[0] + 0xa) as u32 {
                e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            }
            Ret::default()
        });
        let groups = fx.at(0xc8);
        let base = fx.small_array(groups, &[], 8);
        let (g1, g2) = (fx.e.mem.alloc(8), fx.e.mem.alloc(8));
        fx.e.mem.set_u32(g1, 99);
        fx.e.mem.set_u32(g2, 99);
        let mut objects = vec![];
        for (group, size) in [(g1, 10), (g1, 20), (0, 3), (g2, 5)] {
            let object = fx.object();
            fx.method(object, OBJECT_SLOT_GET_GROUP, group);
            fx.method(object, OBJECT_SLOT_GET_BLOCK_ALLOCATION_SIZE, size);
            objects.push(object);
        }
        objects.insert(1, 0);
        fx.pointer_array(fx.at(0x1ec), &objects, 5);
        fn_00a64b80(&mut fx.e, fx.ns);
        assert_eq!(fx.calls(GROUPS_SET_SIZE)[0], vec![groups, 1]);
        assert_eq!([0, 1, 2].map(|i| fx.word(base, i)), [0, g1, g2]);
        assert_eq!(fx.e.mem.u16(groups + 0xa), 3);
        assert_eq!((fx.e.mem.u32(g1), fx.e.mem.u32(g2)), (30, 5));
    }

    #[test]
    fn free_load_data_empties_the_objects_and_the_link_data() {
        let mut fx = Fx::new();
        fx.rec(ENTER_CRITICAL_SECTION, 0);
        fx.rec(LEAVE_CRITICAL_SECTION, 0);
        let object = fx.object();
        fx.e.mem.set_u32(object + 4, 1);
        let delete = fx.method(object, SLOT_DELETE_THIS, 0);
        fx.pointer_array(fx.at(0x1ec), &[object], 1);
        for offset in [0x25c, 0x26c, 0x260, 0x270] {
            fx.put(offset, 3);
        }
        ni_stream_free_load_data(&mut fx.e, fx.ns);
        assert_eq!(fx.calls(delete).len(), 1);
        for offset in [0x25c, 0x26c, 0x260, 0x270, 0x1f8] {
            assert_eq!(fx.get(offset), 0);
        }
        assert_eq!(fx.calls(ENTER_CRITICAL_SECTION).len(), 1);
        assert_eq!(fx.calls(LEAVE_CRITICAL_SECTION).len(), 1);
    }

    #[test]
    fn load_c_string_allocates_the_characters() {
        let mut fx = Fx::new();
        let out = fx.cell(0xdead);
        fx.feed(&[3]);
        fx.feed_bytes(b"abc");
        ni_stream_load_c_string(&mut fx.e, fx.ns, Ptr::new(out));
        let text = fx.e.mem.u32(out);
        assert_eq!(fx.e.mem.cstr(text), b"abc");
        fx.feed(&[0]);
        ni_stream_load_c_string(&mut fx.e, fx.ns, Ptr::new(out));
        assert_eq!(fx.e.mem.u32(out), 0);
        fx.feed(&[u32::MAX]);
        fx.e.mem.set_u32(out, 0xdead);
        ni_stream_load_c_string(&mut fx.e, fx.ns, Ptr::new(out));
        assert_eq!(fx.e.mem.u32(out), 0);
    }

    #[test]
    fn load_length_prefixed_text_terminates_it() {
        let mut fx = Fx::new();
        let buffer = fx.e.mem.alloc(16);
        fx.e.mem.write(buffer, &[b'z'; 16]);
        fx.feed(&[2]);
        fx.feed_bytes(b"hi");
        fn_00a64d80(&mut fx.e, fx.ns, buffer);
        assert_eq!(fx.e.mem.cstr(buffer), b"hi");
    }

    #[test]
    fn load_fixed_string_takes_the_indexed_string() {
        let mut fx = Fx::new();
        let (a, b) = (fx.handle("a"), fx.handle("b"));
        fx.array(fx.at(0x234), &[fx.empty, a, b], 3);
        let out = fx.cell(fx.empty);
        fx.feed(&[1]);
        ni_stream_load_fixed_string(&mut fx.e, fx.ns, Ptr::new(out));
        assert_eq!((fx.e.mem.u32(out), fx.refs(a)), (a, 2));
        // The same string again changes nothing; another one replaces it.
        fx.feed(&[1, 2]);
        ni_stream_load_fixed_string(&mut fx.e, fx.ns, Ptr::new(out));
        assert_eq!(fx.refs(a), 2);
        ni_stream_load_fixed_string(&mut fx.e, fx.ns, Ptr::new(out));
        assert_eq!((fx.e.mem.u32(out), fx.refs(a), fx.refs(b)), (b, 1, 2));
    }

    #[test]
    fn load_fixed_string_with_no_index_clears_the_string() {
        let mut fx = Fx::new();
        let a = fx.handle("a");
        let out = fx.cell(a);
        fx.feed(&[u32::MAX]);
        ni_stream_load_fixed_string(&mut fx.e, fx.ns, Ptr::new(out));
        assert_eq!((fx.e.mem.u32(out), fx.refs(a)), (fx.empty, 0));
        // Nothing to clear when it is null.
        let out = fx.cell(0);
        fx.feed(&[u32::MAX]);
        ni_stream_load_fixed_string(&mut fx.e, fx.ns, Ptr::new(out));
        assert_eq!(fx.e.mem.u32(out), 0);
    }

    #[test]
    fn load_c_string_as_fixed_string_pools_the_characters() {
        let mut fx = Fx::new();
        let old = fx.handle("old");
        let out = fx.cell(old);
        fx.feed(&[3]);
        fx.feed_bytes(b"abc");
        ni_stream_load_c_string_as_fixed_string(&mut fx.e, fx.ns, Ptr::new(out));
        let new = fx.e.mem.u32(out);
        assert_eq!(fx.e.mem.cstr(new), b"abc");
        assert_eq!(fx.refs(old), 0);
        // A long text goes through an allocation that is freed again.
        let long = vec![b'x'; 0x400];
        fx.feed(&[0x400]);
        fx.feed_bytes(&long);
        ni_stream_load_c_string_as_fixed_string(&mut fx.e, fx.ns, Ptr::new(out));
        assert_eq!(fx.e.mem.cstr(fx.e.mem.u32(out)), long);
        assert_eq!(fx.refs(new), 0);
        // No characters: the pooled null string replaces a non-null one.
        fx.feed(&[0]);
        ni_stream_load_c_string_as_fixed_string(&mut fx.e, fx.ns, Ptr::new(out));
        assert_eq!(fx.e.mem.u32(out), fx.empty);
        let none = fx.cell(0);
        fx.feed(&[u32::MAX]);
        ni_stream_load_c_string_as_fixed_string(&mut fx.e, fx.ns, Ptr::new(none));
        assert_eq!(fx.e.mem.u32(none), 0);
    }

    #[test]
    fn fixed_string_set_size_moves_the_strings_to_new_storage() {
        let mut fx = Fx::new();
        let (a, b) = (fx.handle("a"), fx.handle("b"));
        let at = fx.at(0x234);
        let old = fx.array(at, &[a, b], 2);
        // The size it already has: nothing happens.
        fn_00a65140(&mut fx.e, Ptr::new(at), 2);
        assert_eq!(fx.e.mem.u32(at + 4), old);
        fn_00a65140(&mut fx.e, Ptr::new(at), 5);
        let new = fx.e.mem.u32(at + 4);
        assert_ne!(new, old);
        assert_eq!(fx.e.mem.u32(at + 8), 5);
        assert_eq!((fx.word(new, 0), fx.word(new, 1)), (a, b));
        assert_eq!(fx.word(new, 4), fx.empty);
        assert_eq!((fx.refs(a), fx.refs(b)), (1, 1));
        assert!(fx.e.mem.block_size(old - 4).is_none());
        assert_eq!(fx.e.mem.u32(at + 0xc), 2);
    }

    #[test]
    fn fixed_string_set_size_shrinks_and_releases_the_tail() {
        let mut fx = Fx::new();
        let (a, b, c) = (fx.handle("a"), fx.handle("b"), fx.handle("c"));
        let at = fx.at(0x234);
        fx.array(at, &[a, b, c], 3);
        fn_00a65140(&mut fx.e, Ptr::new(at), 1);
        let new = fx.e.mem.u32(at + 4);
        assert_eq!(fx.word(new, 0), a);
        assert_eq!((fx.e.mem.u32(at + 0xc), fx.e.mem.u32(at + 0x10)), (1, 1));
        assert_eq!((fx.refs(a), fx.refs(b), fx.refs(c)), (1, 0, 0));
        // Zero elements: no storage at all.
        fn_00a65140(&mut fx.e, Ptr::new(at), 0);
        assert_eq!((fx.e.mem.u32(at + 4), fx.e.mem.u32(at + 8)), (0, 0));
        assert_eq!(fx.refs(a), 0);
    }

    /// Sets up the members `~NiStream` takes apart.
    fn destructible_stream(fx: &mut Fx) -> [u32; 4] {
        for offset in [0x264, 0x254] {
            let base = fx.e.mem.alloc(8);
            fx.e.mem.set_u32(fx.ns.addr() + offset, base);
        }
        let groups = fx.e.mem.alloc(8);
        fx.put(0xcc, groups);
        let sizes = fx.e.mem.alloc(8);
        fx.put(0x208, sizes);
        let buckets = fx.e.mem.alloc(8);
        fx.put(0x27c, buckets);
        let procedure = fx.object();
        fx.put(0x2a4, procedure);
        fx.put(0x1f0, 0x1111);
        fx.put(0x220, 0x2222);
        fx.put(0x238, 0x3333);
        fx.rec(OBJECT_ARRAY_FREE_STORAGE, 0);
        fx.rec(FIXED_STRINGS_FREE_STORAGE, 0);
        fx.rec(ENTER_CRITICAL_SECTION, 0);
        fx.rec(LEAVE_CRITICAL_SECTION, 0);
        [groups, sizes, buckets, procedure]
    }

    #[test]
    fn destructor_takes_the_members_apart() {
        let mut fx = Fx::new();
        let watched = fx.watch_remove_all();
        let [groups, sizes, buckets, procedure] = destructible_stream(&mut fx);
        let (blocks, ids) = (fx.get(0x264), fx.get(0x254));
        let delete = fx.method(procedure, 0, 0);
        ni_stream_destructor(&mut fx.e, fx.ns);
        assert_eq!(fx.get(0), NI_STREAM_VTABLE);
        // Both streams are deleted, and the background procedure.
        assert_eq!(fx.calls(FAKE_DELETE), vec![vec![fx.bs, 1], vec![fx.bs, 1]]);
        assert_eq!(fx.calls(delete), vec![vec![procedure, 1]]);
        assert_eq!(
            *watched.borrow(),
            [
                REGISTER_MAP_VTABLE_109BD54,
                REGISTER_MAP_BASE_VTABLE_109BC64
            ]
        );
        for block in [groups, sizes, buckets, blocks, ids] {
            assert!(fx.e.mem.block_size(block).is_none());
        }
        assert_eq!(
            fx.calls(OBJECT_ARRAY_FREE_STORAGE),
            vec![vec![0x2222], vec![0x1111]]
        );
        assert_eq!(fx.calls(FIXED_STRINGS_FREE_STORAGE), vec![vec![0x3333]]);
        assert_eq!(fx.get(0xc8), GROUPS_ARRAY_BASE_VTABLE_109BC3C);
        assert_eq!(fx.get(0x204), SIZES_ARRAY_BASE_VTABLE_1081FC8);
        assert_eq!(fx.get(0x1ec), OBJECT_ARRAY_BASE_VTABLE_109BD98);
        assert_eq!(fx.get(0x21c), OBJECT_ARRAY_BASE_VTABLE_109BD98);
        assert_eq!(fx.get(0x234), FIXED_STRINGS_ARRAY_BASE_VTABLE_109BD4C);
    }

    #[test]
    fn stream_deleting_destructor_deletes_on_request() {
        for flags in [0u32, 1] {
            let mut fx = Fx::new();
            fx.watch_remove_all();
            let [_, _, _, procedure] = destructible_stream(&mut fx);
            fx.method(procedure, 0, 0);
            let result = ni_stream_scalar_deleting_destructor(&mut fx.e, fx.ns, flags);
            assert_eq!(result.addr(), fx.ns.addr());
            assert_eq!(fx.get(0), NI_STREAM_VTABLE);
            assert_eq!(fx.e.mem.block_size(fx.ns.addr()).is_none(), flags == 1);
        }
    }

    // -----------------------------------------------------------------
    // The header, the class table, the whole load, and the rest.

    /// The texts the code copies into error messages.
    fn put_texts(fx: &mut Fx) {
        for (at, text) in [
            (TEXT_FILE_FORMAT, "File Format"),
            (TEXT_NOT_A_NIF_FILE, "Not a NIF file"),
            (TEXT_VERSION_TOO_OLD, "NIF version is too old."),
            (TEXT_UNKNOWN_VERSION, "Unknown NIF version."),
            (TEXT_ENDIAN_MISMATCH, "Endian mismatch."),
            (
                TEXT_USER_VERSION_TOO_OLD,
                "NIF user defined version is too old.",
            ),
            (
                TEXT_UNKNOWN_USER_VERSION,
                "Unknown NIF user defined version.",
            ),
            (TEXT_BS_VERSION_TOO_OLD, "NIF BSversion is too old"),
            (
                TEXT_OLDER_FILES,
                "Older files may not contain animation data. Reexport required.",
            ),
            (TEXT_NO_CREATE_FUNCTION, ": cannot find create function."),
        ] {
            fx.e.mem.set_cstr(at, text.as_bytes());
        }
    }

    /// A stream set up to load a header: the supported versions are
    /// 4.2.1.0 to 20.2.0.7 with user versions 0 to 11, and the header line
    /// says "File Format".
    fn header_fixture() -> Fx {
        let mut fx = Fx::new();
        fx.install_binary_loaders();
        put_texts(&mut fx);
        fx.e.mem.set_u32(NIF_MIN_VERSION, 0x0402_0100);
        fx.e.mem.set_u32(NIF_MAX_VERSION, 0x1402_0007);
        fx.e.mem.set_u32(NIF_MAX_USER_VERSION, 11);
        fx.e.mem.set_u32(NIF_MIN_USER_VERSION, 0);
        fx.rec(LARGE_ARRAY_SET_SIZE, 0);
        fx.rec(ENDIAN_FLAG_FUNCTION, 0);
        *fx.line.borrow_mut() = b"Gamebryo File Format, Version 20.2.0.7".to_vec();
        let line = fx.line.clone();
        fx.e.register_double(READ_LINE, move |e, a| {
            e.mem.set_cstr(a[1], &line.borrow());
            Ret::default()
        });
        fx
    }

    /// Feeds a header: the version, the endian byte, the user version, the
    /// number of objects, the Bethesda version and its three strings.
    fn feed_header(fx: &mut Fx, version: u32, endian: u8, user: u32, bs_version: u32) {
        fx.feed(&[version]);
        fx.feed_bytes(&[endian]);
        fx.feed(&[user, 5, bs_version]);
        for text in ["ab", "c", ""] {
            fx.feed_bytes(&[text.len() as u8]);
            fx.feed_bytes(text.as_bytes());
        }
    }

    #[test]
    fn load_header_reads_the_whole_header() {
        let mut fx = header_fixture();
        feed_header(&mut fx, 0x1402_0007, 1, 11, 0x22);
        assert!(ni_stream_load_header(&mut fx.e, fx.ns));
        assert_eq!(fx.get(0xd8), 0x1402_0007);
        assert_eq!(fx.get(0xdc), 11);
        assert_eq!(fx.get(4), 0x22);
        assert_eq!(fx.e.mem.u8(fx.at(0x1e5)), 1);
        assert_eq!(fx.calls(LARGE_ARRAY_SET_SIZE), vec![vec![fx.at(0x1ec), 5]]);
        assert_eq!(fx.e.mem.cstr(fx.at(8)), b"ab");
        assert_eq!(fx.e.mem.cstr(fx.at(0x48)), b"c");
        assert_eq!(fx.e.mem.cstr(fx.at(0x88)), b"");
        assert_eq!(fx.calls(FAKE_ENDIAN), vec![vec![fx.bs, 0], vec![fx.bs, 0]]);
        assert_eq!(fx.get(0x3b8), 0);
    }

    #[test]
    fn load_header_rejects_a_file_that_is_not_a_nif() {
        let mut fx = header_fixture();
        *fx.line.borrow_mut() = b"garbage".to_vec();
        assert!(!ni_stream_load_header(&mut fx.e, fx.ns));
        assert_eq!(fx.get(0x3b8), 2);
        assert_eq!(fx.e.mem.cstr(fx.at(0x3bc)), b"Not a NIF file");
    }

    #[test]
    fn load_header_checks_the_version_range() {
        for (version, error, message) in [
            (0x0402_0000, 3, "NIF version is too old."),
            (0x2000_0000, 4, "Unknown NIF version."),
        ] {
            let mut fx = header_fixture();
            fx.feed(&[version]);
            assert!(!ni_stream_load_header(&mut fx.e, fx.ns));
            assert_eq!(fx.get(0x3b8), error);
            assert_eq!(fx.e.mem.cstr(fx.at(0x3bc)), message.as_bytes());
        }
    }

    #[test]
    fn load_header_checks_the_endian_byte() {
        let mut fx = header_fixture();
        fx.rec(ENDIAN_FLAG_FUNCTION, 1);
        feed_header(&mut fx, 0x1402_0007, 0, 11, 0x22);
        assert!(!ni_stream_load_header(&mut fx.e, fx.ns));
        assert_eq!(fx.get(0x3b8), 6);
        assert_eq!(fx.e.mem.cstr(fx.at(0x3bc)), b"Endian mismatch.");
        // On a big-endian platform the swap is switched on instead.
        let mut fx = header_fixture();
        feed_header(&mut fx, 0x1402_0007, 0, 11, 0x22);
        assert!(ni_stream_load_header(&mut fx.e, fx.ns));
        assert_eq!(fx.calls(FAKE_ENDIAN).last(), Some(&vec![fx.bs, 1]));
    }

    #[test]
    fn load_header_checks_the_user_defined_version() {
        for (min, user, error, message) in [
            (3, 2, 3, "NIF user defined version is too old."),
            (0, 12, 4, "Unknown NIF user defined version."),
        ] {
            let mut fx = header_fixture();
            fx.e.mem.set_u32(NIF_MIN_USER_VERSION, min);
            feed_header(&mut fx, 0x1402_0007, 1, user, 0x22);
            assert!(!ni_stream_load_header(&mut fx.e, fx.ns));
            assert_eq!(fx.get(0x3b8), error);
            assert_eq!(fx.e.mem.cstr(fx.at(0x3bc)), message.as_bytes());
        }
    }

    #[test]
    fn load_header_checks_the_bethesda_version() {
        let mut fx = header_fixture();
        feed_header(&mut fx, 0x1402_0007, 1, 11, 0x10);
        assert!(!ni_stream_load_header(&mut fx.e, fx.ns));
        assert_eq!(fx.get(0x3b8), 3);
        assert_eq!(fx.e.mem.cstr(fx.at(0x3bc)), b"NIF BSversion is too old");
    }

    #[test]
    fn load_header_of_an_old_version_has_no_endian_or_user_version() {
        let mut fx = header_fixture();
        fx.feed(&[0x0a00_0102, 5, 0x22]);
        fx.feed_bytes(&[2, b'x', b'y', 0, 0]);
        assert!(ni_stream_load_header(&mut fx.e, fx.ns));
        // The version 10.0.1.2 is read as 10.0.1.1.
        assert_eq!(fx.get(0xd8), 0x0a00_0101);
        assert_eq!(fx.get(0xdc), 0);
        assert_eq!(fx.get(4), 0x22);
        assert_eq!(fx.calls(LARGE_ARRAY_SET_SIZE), vec![vec![fx.at(0x1ec), 5]]);
        assert_eq!(fx.e.mem.cstr(fx.at(8)), b"xy");
    }

    #[test]
    fn load_top_level_objects_stores_the_named_objects() {
        let mut fx = Fx::new();
        fx.install_array_ops();
        let (first, second) = (fx.object(), fx.object());
        fx.e.mem.set_u32(second + 4, 5);
        fx.pointer_array(fx.at(0x1ec), &[first, second], 2);
        fx.feed(&[2, 1, u32::MAX]);
        ni_stream_load_top_level_objects(&mut fx.e, fx.ns);
        let top = fx.at(0x21c);
        assert_eq!(fx.calls(LARGE_ARRAY_SET_SIZE), vec![vec![top, 2]]);
        let base = fx.get(0x220);
        assert_eq!((fx.word(base, 0), fx.word(base, 1)), (second, 0));
        assert_eq!(fx.get(0x228), 2);
        assert_eq!(fx.e.mem.u32(second + 4), 5);
    }

    /// A stream with a class table lookup and creation functions, ready to
    /// run `LoadRTTI` on `objects` objects.
    fn rtti_fixture(version: u32, objects: u32) -> (Fx, u32, u32) {
        let mut fx = Fx::new();
        put_texts(&mut fx);
        fx.install_array_ops();
        fx.install_lookup(&[
            ("NiNode", CREATE_FIRST),
            ("NiExtra", CREATE_SECOND),
            ("bhkConvexListShape", CREATE_FIRST),
            ("NiControllerManager", CREATE_FIRST),
        ]);
        let (first, second) = (fx.object(), fx.object());
        fx.e.mem.set_u32(first + 4, 5);
        fx.e.mem.set_u32(second + 4, 5);
        fx.rec(CREATE_FIRST, first);
        fx.rec(CREATE_SECOND, second);
        fx.pointer_array(fx.at(0x1ec), &[], objects);
        fx.put(0xd8, version);
        (fx, first, second)
    }

    /// Feeds a class table.
    fn feed_class_table(fx: &mut Fx, names: &[&str]) {
        fx.feed_bytes(&(names.len() as u16).to_le_bytes());
        for name in names {
            fx.feed(&[name.len() as u32]);
            fx.feed_bytes(name.as_bytes());
        }
    }

    #[test]
    fn load_rtti_creates_the_objects_of_their_classes() {
        let (mut fx, first, second) = rtti_fixture(0x1402_0007, 3);
        feed_class_table(&mut fx, &["NiNode", "NiExtra"]);
        fx.feed_bytes(&[0, 0, 1, 0, 1, 0]);
        assert!(ni_stream_load_rtti(&mut fx.e, fx.ns));
        let base = fx.get(0x1f0);
        assert_eq!([0, 1, 2].map(|i| fx.word(base, i)), [first, second, second]);
        assert_eq!(fx.get(0x3b8), 0);
        // The array's references are the double's business; the temporary
        // one is given back.
        assert_eq!(fx.e.mem.u32(first + 4), 5);
        assert_eq!(fx.e.mem.u32(second + 4), 5);
    }

    #[test]
    fn load_rtti_reports_an_unknown_class() {
        let (mut fx, _, _) = rtti_fixture(0x1402_0007, 1);
        feed_class_table(&mut fx, &["Foo"]);
        fx.feed_bytes(&[0, 0]);
        assert!(!ni_stream_load_rtti(&mut fx.e, fx.ns));
        assert_eq!(fx.get(0x3b8), 5);
        assert_eq!(
            fx.e.mem.cstr(fx.at(0x3bc)),
            b"Foo: cannot find create function."
        );
        // A placeholder object of that class is stored as null and the load
        // goes on.
        let (mut fx, _, _) = rtti_fixture(0x1402_0007, 1);
        feed_class_table(&mut fx, &["Foo"]);
        fx.feed_bytes(&[0, 0x80]);
        assert!(ni_stream_load_rtti(&mut fx.e, fx.ns));
        let base = fx.get(0x1f0);
        assert_eq!(fx.word(base, 0), 0);
        assert_eq!(fx.calls(LARGE_ARRAY_SET_AT).len(), 1);
    }

    #[test]
    fn load_rtti_refuses_old_files_with_controller_managers() {
        let (mut fx, _, _) = rtti_fixture(0x1400_0000, 1);
        feed_class_table(&mut fx, &["NiControllerManager"]);
        assert!(!ni_stream_load_rtti(&mut fx.e, fx.ns));
        assert_eq!(fx.get(0x3b8), 3);
        assert_eq!(
            fx.e.mem.cstr(fx.at(0x3bc)),
            b"Older files may not contain animation data. Reexport required."
        );
        // Newer files may have them.
        let (mut fx, first, _) = rtti_fixture(0x1401_0001, 1);
        feed_class_table(&mut fx, &["NiControllerManager"]);
        fx.feed_bytes(&[0, 0]);
        assert!(ni_stream_load_rtti(&mut fx.e, fx.ns));
        assert_eq!(fx.word(fx.get(0x1f0), 0), first);
    }

    #[test]
    fn load_rtti_never_creates_convex_list_shapes() {
        let (mut fx, _, _) = rtti_fixture(0x1402_0007, 1);
        feed_class_table(&mut fx, &["bhkConvexListShape"]);
        fx.feed_bytes(&[0, 0]);
        assert!(!ni_stream_load_rtti(&mut fx.e, fx.ns));
        assert!(fx.calls(CREATE_FIRST).is_empty());
    }

    #[test]
    fn load_fixed_strings_pools_each_string() {
        let mut fx = Fx::new();
        fx.feed(&[2, 3, 2]);
        fx.feed_bytes(b"ab");
        fx.feed(&[1]);
        fx.feed_bytes(b"c");
        assert!(fn_00a65b00(&mut fx.e, fx.ns));
        assert_eq!((fx.get(0x240), fx.get(0x238) != 0), (2, true));
        let base = fx.get(0x238);
        let (ab, c) = (fx.word(base, 0), fx.word(base, 1));
        assert_eq!(fx.e.mem.cstr(ab), b"ab");
        assert_eq!(fx.e.mem.cstr(c), b"c");
        assert_eq!((fx.refs(ab), fx.refs(c)), (1, 1));
        assert_eq!(fx.get(0x244), 2);
    }

    /// Everything `LoadStream` needs for a file of this version with two
    /// objects of one class: the object (with recording methods), a post
    /// process function, and the input of the class table, the fixed
    /// strings and the groups.
    fn load_fixture(version: u32) -> (Fx, u32, u32) {
        let mut fx = Fx::new();
        put_texts(&mut fx);
        fx.install_array_ops();
        fx.install_lookup(&[("NiNode", CREATE_FIRST)]);
        fx.rec(ENTER_CRITICAL_SECTION, 0);
        fx.rec(LEAVE_CRITICAL_SECTION, 0);
        fx.rec(GROUPS_SET_SIZE, 0);
        fx.rec(GROUPS_SET_AT, 0);
        let object = fx.object();
        fx.e.mem.set_u32(object + 4, 10);
        fx.rec(CREATE_FIRST, object);
        for slot in [
            OBJECT_SLOT_LOAD_BINARY,
            OBJECT_SLOT_LINK_OBJECT,
            OBJECT_SLOT_POST_LINK_OBJECT,
        ] {
            fx.method(object, slot, 0);
        }
        fx.method(object, SLOT_DELETE_THIS, 0);
        fx.ns_method(SLOT_LOAD_HEADER, 1);
        fx.ns_method(SLOT_LOAD_OBJECT_SIZE_TABLE, 1);
        fx.put(0xd8, version);
        fx.pointer_array(fx.at(0x1ec), &[], 2);
        // The top-level objects: the double of the virtual function stores
        // the object, as the real one reads it.
        let top = fx.at(0x21c);
        let storage = fx.e.mem.alloc(8);
        let top_levels = fx.ns_method(SLOT_LOAD_TOP_LEVEL_OBJECTS, 0);
        fx.e.register_double(top_levels, move |e, _| {
            e.mem.set_u32(storage, object);
            e.mem.set_u32(top + 4, storage);
            e.mem.set_u32(top + 8, 2);
            e.mem.set_u32(top + 0xc, 1);
            e.mem.set_u32(top + 0x10, 1);
            Ret::default()
        });
        // A post process function.
        let array = fx.e.mem.alloc(0x10);
        fx.small_array(array, &[CREATE_SECOND], 2);
        fx.e.mem.set_u32(POST_PROCESS_FUNCTIONS, array);
        fx.rec(CREATE_SECOND, 0);
        // The input after the header: class table, fixed strings, groups.
        feed_class_table(&mut fx, &["NiNode"]);
        fx.feed_bytes(&[0, 0, 0, 0]);
        fx.feed(&[0, 0, 0]);
        let table = fx.e.mem.u32(object);
        let load_binary = fx.e.mem.u32(table + OBJECT_SLOT_LOAD_BINARY);
        (fx, object, load_binary)
    }

    #[test]
    fn load_stream_loads_links_and_post_links_every_object() {
        let (mut fx, object, load_binary) = load_fixture(0x1402_0007);
        assert!(ni_stream_load_stream(&mut fx.e, fx.ns));
        let ns = fx.ns.addr();
        let table = fx.e.mem.u32(object);
        let link = fx.e.mem.u32(table + OBJECT_SLOT_LINK_OBJECT);
        let post_link = fx.e.mem.u32(table + OBJECT_SLOT_POST_LINK_OBJECT);
        for addr in [load_binary, link, post_link] {
            assert_eq!(fx.calls(addr), vec![vec![object, ns]; 2]);
        }
        assert_eq!((fx.get(0x294), fx.get(0x298), fx.get(0x29c)), (2, 2, 2));
        // The post process function ran on the top-level object.
        assert_eq!(fx.calls(CREATE_SECOND), vec![vec![ns, object]]);
        // The loaded data is freed at the end, under the lock.
        assert_eq!(fx.get(0x1f8), 0);
        assert_eq!(fx.calls(ENTER_CRITICAL_SECTION).len(), 2);
        assert_eq!(fx.calls(LEAVE_CRITICAL_SECTION).len(), 2);
    }

    #[test]
    fn load_stream_stops_when_the_header_fails() {
        let (mut fx, object, load_binary) = load_fixture(0x1402_0007);
        let header = fx.e.mem.u32(fx.e.mem.u32(fx.ns.addr()) + SLOT_LOAD_HEADER);
        fx.rec(header, 0);
        assert!(!ni_stream_load_stream(&mut fx.e, fx.ns));
        assert!(fx.calls(load_binary).is_empty());
        assert!(fx.calls(ENTER_CRITICAL_SECTION).is_empty());
        assert_eq!(fx.e.mem.u32(object + 4), 10);
    }

    #[test]
    fn load_stream_can_be_cancelled() {
        let (mut fx, _, load_binary) = load_fixture(0x1402_0007);
        fx.put(0x28c, STATUS_CANCELLED);
        assert!(!ni_stream_load_stream(&mut fx.e, fx.ns));
        assert!(fx.calls(load_binary).is_empty());
        assert_eq!(fx.get(0x1f8), 0);
    }

    #[test]
    fn load_stream_suspends_the_thread_when_asked_to() {
        let (mut fx, _, _) = load_fixture(0x1402_0007);
        fx.rec(THREAD_SUSPEND, 0);
        fx.put(0x28c, STATUS_SUSPEND_REQUESTED);
        fx.put(0x2a0, 0x77);
        assert!(ni_stream_load_stream(&mut fx.e, fx.ns));
        assert_eq!(fx.calls(THREAD_SUSPEND), vec![vec![0x77]]);
        assert_eq!(fx.get(0x28c), STATUS_SUSPENDED);
    }

    #[test]
    fn load_stream_of_an_old_file_loads_each_object_itself() {
        let (mut fx, _, load_binary) = load_fixture(0x0402_0200);
        let load_object = fx.ns_method(SLOT_LOAD_OBJECT, 1);
        // Two objects to load (the array's capacity is the count).
        assert!(ni_stream_load_stream(&mut fx.e, fx.ns));
        assert_eq!(fx.calls(load_object).len(), 2);
        assert!(fx.calls(load_binary).is_empty());
        // The object loader failing ends the load and frees the data.
        let (mut fx, object, _) = load_fixture(0x0402_0200);
        let load_object = fx.ns_method(SLOT_LOAD_OBJECT, 0);
        assert!(!ni_stream_load_stream(&mut fx.e, fx.ns));
        assert_eq!(fx.calls(load_object).len(), 1);
        assert_eq!(fx.calls(ENTER_CRITICAL_SECTION).len(), 2);
        assert_eq!(fx.e.mem.u32(object + 4), 10);
    }

    #[test]
    fn load_stream_skips_over_objects_that_were_not_created() {
        let (mut fx, object, load_binary) = load_fixture(0x1402_0007);
        // One class table entry that has no creation function; the first
        // object is its placeholder, the second a normal one.
        fx.input.borrow_mut().clear();
        feed_class_table(&mut fx, &["NiNode", "Foo"]);
        fx.feed_bytes(&[1, 0x80, 0, 0]);
        fx.feed(&[0, 0, 0]);
        let sizes = fx.e.mem.alloc(8);
        fx.put(0x208, sizes);
        let table_load =
            fx.e.mem
                .u32(fx.e.mem.u32(fx.ns.addr()) + SLOT_LOAD_OBJECT_SIZE_TABLE);
        fx.e.register_double(table_load, move |e, _| {
            e.mem.set_u32(sizes, 0x44);
            1u32.into_ret()
        });
        assert!(ni_stream_load_stream(&mut fx.e, fx.ns));
        assert_eq!(fx.calls(load_binary), vec![vec![object, fx.ns.addr()]]);
        assert_eq!(fx.calls(FAKE_SEEK), vec![vec![fx.bs, 0x44]]);
    }

    #[test]
    fn fixed_string_add_fills_the_first_free_slot() {
        let mut fx = Fx::new();
        let handles: Vec<u32> = ["a", "b", "c", "d", "e"].map(|t| fx.handle(t)).to_vec();
        let at = fx.at(0x234);
        let base = fx.array(at, &[handles[0], fx.empty, handles[1]], 4);
        let array = Ptr::new(at);
        let value = fx.cell(handles[2]);
        assert_eq!(fn_00a65f80(&mut fx.e, array, Ptr::new(value)), 1);
        assert_eq!(fx.word(base, 1), handles[2]);
        assert_eq!(fx.refs(handles[2]), 2);
        assert_eq!(fx.e.mem.u32(at + 0x10), 3);
        // No free slot: it goes to the end.
        let value = fx.cell(handles[3]);
        assert_eq!(fn_00a65f80(&mut fx.e, array, Ptr::new(value)), 3);
        assert_eq!(fx.e.mem.u32(at + 0xc), 4);
        // No room either: the array grows by its grow-by first.
        let value = fx.cell(handles[4]);
        assert_eq!(fn_00a65f80(&mut fx.e, array, Ptr::new(value)), 4);
        assert_eq!(fx.e.mem.u32(at + 8), 8);
        assert_eq!(fx.word(fx.e.mem.u32(at + 4), 4), handles[4]);
        // The empty string is not added.
        let value = fx.cell(fx.empty);
        assert_eq!(fn_00a65f80(&mut fx.e, array, Ptr::new(value)), u32::MAX);
    }

    #[test]
    fn static_initialization_creates_the_loader_map_and_the_post_process_array() {
        let mut fx = Fx::new();
        fn_00a660a0(&mut fx.e);
        let map = fx.e.mem.u32(LOADERS);
        assert_eq!(fx.e.mem.u32(map), LOADER_MAP_VTABLE_109BF4C);
        assert_eq!((fx.e.mem.u32(map + 4), fx.e.mem.u32(map + 0xc)), (0x3b, 0));
        assert_eq!(fx.e.mem.u8(map + 0x10), 0);
        let buckets = fx.e.mem.u32(map + 8);
        assert_eq!(fx.e.mem.block_size(buckets), Some(0xf0));
        assert!(fx.e.mem.bytes(buckets, 0xec).iter().all(|b| *b == 0));
        let array = fx.e.mem.u32(POST_PROCESS_FUNCTIONS);
        assert_eq!(fx.e.mem.u32(array), POST_PROCESS_ARRAY_VTABLE_109BD88);
        assert_eq!(fx.e.mem.u16(array + 0xe), 3);
        assert_eq!(fx.e.mem.u32(array + 4), 0);
        assert_eq!(fx.e.mem.u8(USE_DEFAULT_PATH), 1);
        // When the allocations fail both globals are null.
        let mut fx = Fx::new();
        fx.e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        fx.e.mem.set_u32(LOADERS, 7);
        fx.e.mem.set_u32(POST_PROCESS_FUNCTIONS, 7);
        fn_00a660a0(&mut fx.e);
        assert_eq!(fx.e.mem.u32(LOADERS), 0);
        assert_eq!(fx.e.mem.u32(POST_PROCESS_FUNCTIONS), 0);
        assert_eq!(fx.e.mem.u8(USE_DEFAULT_PATH), 1);
    }

    #[test]
    fn constructor_sets_the_starting_values() {
        let mut fx = Fx::new();
        fx.e.mem.set_u32(NIF_MAX_VERSION, 0x1402_0007);
        fx.e.mem.set_u32(NIF_MAX_USER_VERSION, 11);
        fx.e.mem
            .write(fx.ns.addr(), &vec![0xaa; NiStream::SIZE as usize]);
        let result = ni_stream_constructor(&mut fx.e, fx.ns);
        assert_eq!(result.addr(), fx.ns.addr());
        for (offset, value) in [
            (0x000, NI_STREAM_VTABLE),
            (0x004, 0x22),
            (0x0c8, GROUPS_ARRAY_VTABLE_109BD90),
            (0x0cc, 0),
            (0x0d8, 0x1402_0007),
            (0x0dc, 11),
            (0x1ec, OBJECT_ARRAY_VTABLE_109BF6C),
            (0x1f0, 0),
            (0x1f4, 0),
            (0x1f8, 0),
            (0x200, 0x400),
            (0x204, SIZES_ARRAY_VTABLE_1081EF0),
            (0x218, 1),
            (0x21c, OBJECT_ARRAY_VTABLE_109BF6C),
            (0x230, 1),
            (0x234, FIXED_STRINGS_ARRAY_VTABLE_109BF74),
            (0x248, 0x80),
            (0x24c, 0),
            (0x250, 0),
            (0x254, 0),
            (0x264, 0),
            (0x260, 0),
            (0x270, 0),
            (0x274, REGISTER_MAP_VTABLE_109BD54),
            (0x278, 0x25),
            (0x280, 0),
            (0x28c, 0),
            (0x2a0, 0),
            (0x2a4, 0),
            (0x2a8, 2),
            (0x2ac, u32::MAX),
            (0x2b0, u32::MAX),
            (0x3b8, 0),
        ] {
            assert_eq!(fx.get(offset), value, "offset {offset:#x}");
        }
        for offset in [0x08, 0x48, 0x88, 0xe0, 0x3bc] {
            assert_eq!(fx.e.mem.u8(fx.at(offset)), 0, "offset {offset:#x}");
        }
        for offset in [0x1e4, 0x1e5] {
            assert_eq!(fx.e.mem.u8(fx.at(offset)), 1, "offset {offset:#x}");
        }
        assert_eq!(fx.e.mem.u8(USE_DEFAULT_PATH), 1);
        // The groups array: sizes 0, grow-by 1.
        assert_eq!(fx.e.mem.u16(fx.at(0xd0)), 0);
        assert_eq!(fx.e.mem.u16(fx.at(0xd2)), 0);
        assert_eq!(fx.e.mem.u16(fx.at(0xd4)), 0);
        assert_eq!(fx.e.mem.u16(fx.at(0xd6)), 1);
        let buckets = fx.get(0x27c);
        assert_eq!(fx.e.mem.block_size(buckets), Some(0x98));
        assert!(fx.e.mem.bytes(buckets, 0x94).iter().all(|b| *b == 0));
    }

    #[test]
    fn insert_object_appends_to_the_top_level_objects() {
        let mut fx = Fx::new();
        fx.rec(LARGE_ARRAY_SET_SIZE, 0);
        fx.rec(LARGE_ARRAY_SET_AT, 0);
        let object = fx.object();
        fx.e.mem.set_u32(object + 4, 5);
        let top = fx.at(0x21c);
        // Full: grow by the grow-by first.
        fx.array(top, &[1, 2], 2);
        ni_stream_insert_object(&mut fx.e, fx.ns, object);
        assert_eq!(fx.calls(LARGE_ARRAY_SET_SIZE), vec![vec![top, 6]]);
        assert_eq!(fx.calls(LARGE_ARRAY_SET_AT).len(), 1);
        assert_eq!(fx.calls(LARGE_ARRAY_SET_AT)[0][..2], [top, 2]);
        assert_eq!(fx.e.mem.u32(object + 4), 5);
        // With room: no growing.
        let mut fx = Fx::new();
        fx.rec(LARGE_ARRAY_SET_SIZE, 0);
        fx.rec(LARGE_ARRAY_SET_AT, 0);
        let top = fx.at(0x21c);
        fx.array(top, &[1], 3);
        ni_stream_insert_object(&mut fx.e, fx.ns, 0);
        assert!(fx.calls(LARGE_ARRAY_SET_SIZE).is_empty());
        assert_eq!(fx.calls(LARGE_ARRAY_SET_AT)[0][..2], [top, 1]);
    }

    #[test]
    fn register_save_object_gives_new_objects_the_next_id() {
        let mut fx = Fx::new();
        fx.rec(LARGE_ARRAY_SET_SIZE, 0);
        fx.rec(LARGE_ARRAY_SET_AT, 0);
        fx.rec(REGISTER_MAP_SET_AT, 0);
        fx.e.register_double(MAP_GET_AT, |_, a| ((a[1] == 0x111) as u32).into_ret());
        let object = fx.object();
        fx.e.mem.set_u32(object + 4, 5);
        let (map, objects) = (fx.at(0x274), fx.at(0x1ec));
        fx.array(objects, &[1, 2], 4);
        // Already registered: nothing changes.
        assert!(!ni_stream_register_save_object(&mut fx.e, fx.ns, 0x111));
        assert!(fx.calls(REGISTER_MAP_SET_AT).is_empty());
        assert!(fx.calls(LARGE_ARRAY_SET_AT).is_empty());
        // New: gets the id 2 and is appended.
        assert!(ni_stream_register_save_object(&mut fx.e, fx.ns, object));
        assert_eq!(fx.calls(REGISTER_MAP_SET_AT), vec![vec![map, object, 2]]);
        assert_eq!(fx.calls(LARGE_ARRAY_SET_AT)[0][..2], [objects, 2]);
        assert!(fx.calls(LARGE_ARRAY_SET_SIZE).is_empty());
        assert_eq!(fx.e.mem.u32(object + 4), 5);
    }

    #[test]
    fn register_fixed_string_adds_only_new_strings() {
        let mut fx = Fx::new();
        let (a, b) = (fx.handle("a"), fx.handle("b"));
        let at = fx.at(0x234);
        let base = fx.array(at, &[fx.empty, a], 4);
        // Already there: nothing happens.
        let string = fx.cell(a);
        assert!(ni_stream_register_fixed_string(
            &mut fx.e,
            fx.ns,
            Ptr::new(string)
        ));
        assert_eq!(fx.refs(a), 1);
        // New: goes in the first free slot.
        let string = fx.cell(b);
        assert!(ni_stream_register_fixed_string(
            &mut fx.e,
            fx.ns,
            Ptr::new(string)
        ));
        assert_eq!(fx.word(base, 0), b);
        assert_eq!(fx.refs(b), 2);
        // The empty string is accepted and not added.
        let string = fx.cell(fx.empty);
        assert!(ni_stream_register_fixed_string(
            &mut fx.e,
            fx.ns,
            Ptr::new(string)
        ));
        assert_eq!(fx.e.mem.u32(at + 0x10), 2);
    }

    #[test]
    fn load_object_creates_the_named_class() {
        let mut fx = Fx::new();
        put_texts(&mut fx);
        fx.install_array_ops();
        fx.install_lookup(&[("Ab", CREATE_FIRST)]);
        let object = fx.object();
        fx.e.mem.set_u32(object + 4, 5);
        fx.rec(CREATE_FIRST, object);
        let load = fx.method(object, OBJECT_SLOT_LOAD_BINARY, 0);
        fx.pointer_array(fx.at(0x1ec), &[], 4);
        fx.feed(&[2]);
        fx.feed_bytes(b"Ab");
        assert!(ni_stream_load_object(&mut fx.e, fx.ns));
        assert_eq!(fx.word(fx.get(0x1f0), 0), object);
        assert_eq!(fx.calls(LARGE_ARRAY_SET_AT)[0][..2], [fx.at(0x1ec), 0]);
        assert_eq!(fx.calls(load), vec![vec![object, fx.ns.addr()]]);
        assert_eq!(fx.e.mem.u32(object + 4), 5);
        // An unknown class is an error.
        fx.feed(&[3]);
        fx.feed_bytes(b"Zzz");
        assert!(!ni_stream_load_object(&mut fx.e, fx.ns));
        assert_eq!(fx.get(0x3b8), 5);
        assert_eq!(
            fx.e.mem.cstr(fx.at(0x3bc)),
            b"Zzz: cannot find create function."
        );
        assert_eq!(fx.calls(load).len(), 1);
    }

    /// Doubles for the name-to-index map the save builds on the stack:
    /// hash, comparison and `SetValue` (the virtual functions of its virtual
    /// table), the lookup, and the node pool.
    fn install_name_map(fx: &mut Fx) {
        fx.e.register(0x0720_0200, |e, a| {
            (e.mem.u8(a[1]) as u32 % 0x25).into_ret()
        });
        fx.e.register(0x0720_0204, |e, a| {
            ((e.mem.cstr(a[1]) == e.mem.cstr(a[2])) as u32).into_ret()
        });
        fx.e.register(0x0720_0208, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u16(a[1] + 8, a[3] as u16);
            Ret::default()
        });
        for (slot, target) in [(4, 0x0720_0200), (8, 0x0720_0204), (0xc, 0x0720_0208)] {
            fx.e.mem.set_u32(NAME_MAP_VTABLE_109BF7C + slot, target);
        }
        fx.e.register(SAVE_MAP_GET_AT, |e, a| {
            let name = e.mem.cstr(a[1]);
            let table = e.mem.u32(a[0] + 8);
            let mut node = e.mem.u32(table + (name[0] as u32 % 0x25) * 4);
            while node != 0 {
                if e.mem.cstr(e.mem.u32(node + 4)) == name {
                    return 1u32.into_ret();
                }
                node = e.mem.u32(node);
            }
            0u32.into_ret()
        });
        fx.e.register(POOL_ALLOCATE_NODE, |e, _| e.mem.alloc(0x10).into_ret());
    }

    /// Objects whose class names are these (virtual function `+0x74`
    /// returns the address of a pointer to the name); the second is a
    /// placeholder.
    fn named_objects(fx: &mut Fx, names: &[&str]) -> Vec<u32> {
        let mut objects = vec![];
        for (index, name) in names.iter().enumerate() {
            let object = fx.object();
            let text = fx.cs(name);
            let holder = fx.cell(text);
            fx.method(object, OBJECT_SLOT_GET_RTTI_NAME, holder);
            fx.method(object, OBJECT_SLOT_IS_NULL_PLACEHOLDER, (index == 1) as u32);
            fx.e.mem.set_u32(object + 4, 5);
            objects.push(object);
        }
        objects
    }

    #[test]
    fn save_rtti_writes_the_class_names_and_indices() {
        let mut fx = Fx::new();
        install_name_map(&mut fx);
        fx.rec(MAP_REMOVE_ALL, 0);
        let objects = named_objects(&mut fx, &["A", "B", "A"]);
        fx.pointer_array(fx.at(0x1ec), &objects, 3);
        fn_00a665f0(&mut fx.e, fx.ns);
        let mut expected = vec![2, 0];
        expected.extend(words(&[1]));
        expected.push(b'A');
        expected.extend(words(&[1]));
        expected.push(b'B');
        expected.extend([0, 0, 1, 0x80, 0, 0]);
        assert_eq!(fx.written(), expected);
        // The local map is destroyed at the end.
        assert_eq!(fx.calls(MAP_REMOVE_ALL).len(), 2);
    }

    #[test]
    fn save_stream_writes_every_part_in_order() {
        let mut fx = Fx::new();
        install_name_map(&mut fx);
        fx.rec(MAP_REMOVE_ALL, 0);
        fx.rec(GROUPS_SET_SIZE, 0);
        fx.e.register_double(GROUPS_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let group = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, group);
            if a[1] >= e.mem.u16(a[0] + 0xa) as u32 {
                e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            }
            Ret::default()
        });
        fx.rec(SIZES_ARRAY_SET_SIZE, 0);
        let objects = named_objects(&mut fx, &["A", "B"]);
        for object in &objects {
            fx.method(*object, OBJECT_SLOT_GET_GROUP, 0);
        }
        fx.pointer_array(fx.at(0x1ec), &objects, 2);
        fx.small_array(fx.at(0xc8), &[], 4);
        let sizes = fx.array(fx.at(0x204), &[], 2);
        let ordered = [
            fx.ns_method(SLOT_REGISTER_OBJECTS, 0),
            fx.ns_method(SLOT_SAVE_HEADER, 0),
            fx.ns_method(SLOT_PRE_SAVE_OBJECT_SIZE_TABLE, 0x100),
            fx.ns_method(SLOT_SAVE_TOP_LEVEL_OBJECTS, 0),
            fx.ns_method(SLOT_SAVE_OBJECT_SIZE_TABLE, 1),
        ];
        // Each object writes some bytes (a different count each).
        for (object, count) in objects.iter().zip([8u32, 12]) {
            let save = fx.method(*object, OBJECT_SLOT_SAVE_BINARY, 0);
            let bs = fx.bs;
            fx.e.register_double(save, move |e, _| {
                let position = e.mem.u32(bs + 4);
                e.mem.set_u32(bs + 4, position + count);
                Ret::default()
            });
        }
        assert!(ni_stream_save_stream(&mut fx.e, fx.ns));
        let order: Vec<u32> = fx
            .log
            .borrow()
            .iter()
            .map(|(addr, _)| *addr)
            .filter(|addr| ordered.contains(addr))
            .collect();
        assert_eq!(order, ordered);
        assert_eq!(fx.calls(ordered[4]), vec![vec![fx.ns.addr(), 0x100]]);
        assert_eq!(fx.calls(SIZES_ARRAY_SET_SIZE), vec![vec![fx.at(0x204), 2]]);
        assert_eq!((fx.word(sizes, 0), fx.word(sizes, 1)), (8, 12));
        assert_eq!(fx.get(0x1f8), 0);
        assert!(fx
            .calls(MAP_REMOVE_ALL)
            .iter()
            .any(|args| args == &vec![fx.at(0x274)]));
    }
}

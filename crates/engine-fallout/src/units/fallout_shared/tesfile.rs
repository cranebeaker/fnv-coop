//! `fallout shared/tesfile.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESFile` is one plugin or master file (`.esm`/`.esp`): its names, the
//! open `BSFile`, a cursor over the record stream (the current `FORM`
//! header, the offsets of the form and of the chunk inside it), the file
//! header information and the lists of masters.
//!
//! Notes for the next session (session 1 translated the first 40 functions
//! of the queue, `00470650` to `00472660`; session 2 the next 40, `004726b0`
//! to `00473d00`, adding [`FormGroup`] (the 0x1C-byte entry of
//! `m_grouplist`) and the helpers `seek_from_end` and `count_form`;
//! session 3 the last 25, `00473d20` to `00474880`: the unit is finished.
//! It uses [`NiTPointerMap`] and [`NiTArray`] from `crate::types` for the
//! file maps and the key array; the global file map and its user count are
//! at [`GLOBAL_FILE_MAP`] and [`GLOBAL_FILE_MAP_USERS`]. Earlier code still
//! reaches `DecompressCurrentForm` and the thread-safe map constructor
//! `004744a0` by address (tests double them); that works unchanged.
//! Session 1 still reaches `GetTESChunk`,
//! `ReadFormHeader`, `ReadChunkHeader`, `CloseAllOpenGroups` and
//! `FreeDecompressedForm` by address (their tests put doubles there);
//! that works unchanged now that they are translated:
//! - Layouts: [`TESFile`] (0x42C bytes, Xbox PDB names, PC offsets checked
//!   against this unit's code), [`Form`] (the 0x18-byte record header,
//!   `FORM`), [`FileHeader`], [`PathBuffer`] and [`Win32FindData`]. Fields
//!   nobody reads yet are declared anyway so the next session finds them.
//! - The private helpers below the constants (`list_item`, `list_next`,
//!   `seek`, `log`, `report_set_file_pointer_failure`, ...) wrap calls this
//!   file makes over and over. Every callee outside the 40 functions of
//!   this session is called by its address, including the later functions
//!   of this unit (`GetTESChunk`, `ReadFormHeader`, ...); switch them to
//!   direct calls when they are translated.
//! - A `BSSimpleList<T>` is its head node (item at +0, next at +4):
//!   `006815c0(node)` answers the node's own address (the item slot),
//!   `00726070(node)` the next node, `00464df0(this)` the address of
//!   `listMasters` (`+0x3EC`) and `00470b90(this)` that of
//!   `listMastersData` (`+0x3F4`).
//! - The `BSFile` is used through its vtable: `+0x14` seeks to
//!   `(offset, whence)` with `whence` read from the global at
//!   [`SEEK_MODE`] (0 in the exe), `+0x20` is called with `(1, 0)` after the
//!   open, `+0x28` answers the file size and `+0` is the deleting
//!   destructor. The PC table is not the Xbox PDB's, so the slots are
//!   described, not named. `BSFile +0x2C` (byte) is the "file is ready"
//!   flag; `+0x38` and `+0x150` are two copies of the current position
//!   (`004720a0`, `00472380`).
//! - The record tags compared by the stream code (`01187014`, `01187020`,
//!   `011873c8`) are filled in at run time (zero in the exe on disk), so
//!   tests set them.
//! - The decompiler names `Concurrency::details::QuickBitSet`,
//!   `std::_Fiopen` and `CArray<...>::RemoveAll` in this unit are wrong:
//!   they are the inlined `BSSimpleList`/`BSStringT` constructors and
//!   destructors and the compiler's `memset`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleList, BSStringT, NiTArray, NiTPointerMap};

/// `BSSimpleList` node accessor (`006815c0`, returns its `ECX`): the node's
/// own address, which is the address of its item slot.
pub(crate) const LIST_ITEM_SLOT: u32 = 0x0068_15c0;
/// `BSSimpleList` next node (`00726070`): the word at `node + 4`.
pub(crate) const LIST_NEXT: u32 = 0x0072_6070;
/// `BSSimpleList` removal of the head node (`0063f7b0`).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// `BSSimpleList` clear (`00470470`): frees every node after the head and
/// zeroes the head's item.
const LIST_CLEAR: u32 = 0x0047_0470;
/// `BSSimpleList` destructor (`0046ffb0`), which calls [`LIST_CLEAR`].
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `BSSimpleList` constructor (`0096a2d0`).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `BSStringT<char>` constructor and destructor.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `operator new` and `operator delete`.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The logging call (`005b5e40`): a format string address, then arguments.
const LOG: u32 = 0x005b_5e40;
/// CRT wrappers: `memset(dst, value, size)`, `memcpy(dst, src, size)` and
/// `memcmp(a, b, size)`.
const MEMSET: u32 = 0x0040_3d30;
const MEMCPY: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
/// `strcpy_s(dst, size, src)` and `strcat_s(dst, size, src)` wrappers; the
/// `snprintf`-style `(dst, size, format, argument)`; `strchr(text, char)`;
/// the case-insensitive compare `(a, b)` (0 when equal).
const STRCPY_S: u32 = 0x0040_6d30;
const STRCAT_S: u32 = 0x0040_6d50;
const SNPRINTF: u32 = 0x0040_6d00;
const STRCHR: u32 = 0x0043_b9d0;
const STRICMP: u32 = 0x0040_4dc0;
/// In-place byte swap of a 32-bit and of a 16-bit value, `(pointer, 0)`.
const SWAP_U32: u32 = 0x0040_1080;
const SWAP_U16: u32 = 0x0040_7a90;
/// `BSFile::BSFile(this; path, mode, buffer size, 0)`.
const BSFILE_CONSTRUCT: u32 = 0x00b0_0260;
/// `BSFile::ReadF(this; buffer, size)`: bytes read.
const BSFILE_READ: u32 = 0x00b0_0650;
/// `BSSystemFile::DeleteFileA(path)` (cdecl): 0 on success.
const DELETE_SYSTEM_FILE: u32 = 0x00af_f0b0;
/// CRT `rename(old, new)` and `_errno()`.
const RENAME: u32 = 0x00ec_862c;
const ERRNO: u32 = 0x00ec_85e3;
/// `BSFile +0x2C`: the "file is ready" byte.
const FILE_IS_READY: u32 = 0x0089_05f0;
/// `this + 0x20` of a `TESFile`: its file name.
const FILE_NAME: u32 = 0x0089_1170;
/// `bhkLiquidAction::QCinfoSize` (Xbox PDB name of folded code): answers
/// 0x18, the size of a record header.
const FORM_HEADER_SIZE: u32 = 0x0047_1840;
/// The byte at `+0x61D` of the data handler, which `CloseTES` reads.
const HANDLER_FLAG: u32 = 0x0042_26e0;
/// `TESForm::GetFormTypeFromFormString(tag)` (Xbox PDB).
const TYPE_FROM_FORM_TAG: u32 = 0x0048_6890;
/// Callees later in this unit or without a unit: `CloseAllOpenGroups`, the
/// release of the decompressed form buffer (`+0x420`), two clean-ups, a
/// stub returning 0, the header status check, `GetTESChunk`,
/// `ReadFormHeader` and `ReadChunkHeader`.
const CLOSE_ALL_GROUPS: u32 = 0x0047_3830;
const FREE_DECOMPRESSED_FORM: u32 = 0x0047_3960;
const DESTRUCT_CLEANUP_A: u32 = 0x0047_3d90;
const DESTRUCT_CLEANUP_B: u32 = 0x0047_3a10;
const STUB_RETURNS_ZERO: u32 = 0x0047_3880;
const READ_HEADER_STATUS: u32 = 0x0047_1400;
const GET_TES_CHUNK: u32 = 0x0047_26b0;
const READ_FORM_HEADER: u32 = 0x0047_2bc0;
const READ_CHUNK_HEADER: u32 = 0x0047_2d30;
/// The master list accessor of `tesdatahandler.cpp` (`00464df0`).
const MASTERS_LIST: u32 = 0x0046_4df0;
/// Header version getter (`004694c0`, `float` in `ST0`).
const HEADER_VERSION: u32 = 0x0046_94c0;
/// Returns 1 whatever its `this` (`008d0360`).
const ALWAYS_TRUE: u32 = 0x008d_0360;
/// Sets the byte global at `01202d63` (`00461300`), done before an error
/// is reported.
const NOTE_FAILURE: u32 = 0x0046_1300;

// Callees and data of the second session (`004726b0` to `00473d00`).
// Callees outside this unit are described by what their body does; the
// engine map's names for several of them belong to other, folded code.
/// `bMustEndianConvert` getter (`00401680`): the byte at `+0x299`.
const MUST_ENDIAN_CONVERT: u32 = 0x0040_1680;
/// Swaps the byte order of a six-byte chunk header (`u32` id, `u16` size)
/// in place (`004141e0`, `this` = the header).
const CHUNK_HEADER_ENDIAN: u32 = 0x0041_41e0;
/// Moves the cursor past the current chunk and reads the next chunk header
/// (`004726f0`, translated in `units/unplaced`): used after an `XXXX`
/// chunk.
const SKIP_TO_NEXT_CHUNK: u32 = 0x0047_26f0;
/// `HighProcess::GetPostAnimationActions` (Xbox PDB name of folded code):
/// answers the `u32` at `+0x424`, the size of the decompressed form buffer.
const DECOMPRESSED_FORM_SIZE: u32 = 0x0047_27d0;
/// `TESFile::DecompressCurrentForm` (Xbox PDB), later in this unit.
const DECOMPRESS_CURRENT_FORM: u32 = 0x0047_40a0;
/// The thread-safe file map's constructor (`004744a0(this, 0x25)`), later in
/// this unit: a hash table of 0x25 buckets.
const THREAD_FILE_MAP_CONSTRUCT: u32 = 0x0047_44a0;
/// The thread-safe file map's lookup: `(map; key, &value)`.
const THREAD_FILE_MAP_LOOKUP: u32 = 0x0085_3130;
/// The thread-safe file map's `SetAt(map; key, value)` (the engine map's
/// `NiTMapBase<..CombatThreat_P..>::SetAt` is folded code).
const THREAD_FILE_MAP_SET_AT: u32 = 0x0084_4700;
/// The map's iteration: `begin(map)` answers the first position and
/// `next(map; &position, &key, &value)` steps it.
const THREAD_FILE_MAP_BEGIN: u32 = 0x004b_9ba0;
const THREAD_FILE_MAP_NEXT: u32 = 0x006b_7f20;
/// Clears the map (`00438af0`).
const THREAD_FILE_MAP_CLEAR: u32 = 0x0043_8af0;
/// A call on a `TESFile` (`00483710`) made on each file of the map before
/// it is deleted, and on a new thread-safe file after it is stored.
const RELEASE_FILE: u32 = 0x0048_3710;
/// Scalar deleting destructor `(object; 1)` (`004601a0`).
const SCALAR_DELETE: u32 = 0x0046_01a0;
/// `BSSimpleList::AddHead(list; &item)` (`005ae3d0`).
const LIST_ADD_HEAD: u32 = 0x005a_e3d0;
/// `BSSimpleList::IsEmpty(list)` (`008256d0`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `BSFile` write `(file; data, size, &one, 1)` (`0044e120`); answers the
/// number of bytes written.
const BSFILE_WRITE: u32 = 0x0044_e120;
/// `TESForm` accessors used by the writer: the form type index (the byte at
/// `+4`), the flags word, and the form id (`+0xC`).
const FORM_TYPE_INDEX: u32 = 0x0040_1170;
const FORM_FLAGS: u32 = 0x0044_ddc0;
const FORM_ID: u32 = 0x0084_e3a0;
/// `TESForm` calls made by `LoadForm`, described by their arguments.
const FORM_SET_TYPE: u32 = 0x004f_15a0;
const FORM_IS_FLAGGED: u32 = 0x0040_77c0;
const FORM_SET_LOAD_FLAGS: u32 = 0x0040_3550;
const FORM_FILE_FLAGS: u32 = 0x0052_24a0;
/// The word at +0x24C of its `this` (`008d8ac0`, `RET` with no argument
/// popped; the engine map's name for the folded body is
/// `MiddleHighProcess::GetFaceSkinnedNode`): on a `TESFile`, the current
/// record's form id (`m_currentform.iFormID`).
const RECORD_FORM_ID: u32 = 0x008d_8ac0;
const FORM_SET_FILE: u32 = 0x0048_4f50;
/// `TESForm::FreeFormBuffer` (Xbox PDB): releases the buffer of the form
/// that was written.
const FORM_FREE_FORM_BUFFER: u32 = 0x0048_5b30;
/// `(masked form id) -> bool` check used by `ReadFormHeader` (`00484b40`,
/// `tesform.cpp`).
const FORM_ID_NEEDS_STRIPPING: u32 = 0x0048_4b40;
/// The current thread id (`0040fc90`) and the thread id of the procedure
/// owner (`0044edb0`, `this` = the global at [`PROCEDURE_OWNER`]).
const CURRENT_THREAD: u32 = 0x0040_fc90;
const OWNER_THREAD: u32 = 0x0044_edb0;
const PROCEDURE_OWNER: u32 = 0x011d_ea0c;
/// Byte at `+0x61A` of the data handler (`004516b0`, `this` = the
/// handler).
const HANDLER_GROUPS_FLAG: u32 = 0x0045_16b0;
/// `MessageHandler::IncDisableWarningCount(flag)` (Xbox PDB).
const WARNING_COUNT: u32 = 0x0043_b2b0;
/// CRT `sprintf(buffer, format, ...)`.
const SPRINTF: u32 = 0x00ec_623a;
/// The data handler's file table, `(handler) -> table` (`0045dfc0`: `+0x210`).
const HANDLER_FILE_TABLE: u32 = 0x0045_dfc0;
/// `(this) -> directory` of a `TESFile` (`00462e80`: `+0x124`).
const FILE_DIRECTORY: u32 = 0x0046_2e80;
/// The table of record tags by form type index: 12-byte entries whose first
/// word is the tag.
const FORM_TAG_TABLE: u32 = 0x0118_7008;
/// Size of a group entry (`FORM_GROUP`) and of a record header.
const GROUP_ENTRY_SIZE: u32 = 0x1c;

// Messages of the second session.
const MESSAGE_CHUNK_TOO_BIG: u32 = 0x0101_a0c0;
const MESSAGE_CHUNK_SECOND_READ_FAILED: u32 = 0x0101_a078;
const MESSAGE_CHUNK_FIRST_READ_FAILED: u32 = 0x0101_a130;
const MESSAGE_WRITE_ERROR: u32 = 0x0101_a174;
const MESSAGE_CREATE_GROUP_FAILED: u32 = 0x0101_a198;
const MESSAGE_VERSION_TOO_HIGH: u32 = 0x0101_a1d4;
/// The `double` 1.34 (the highest header version this exe loads).
const HIGHEST_HEADER_VERSION: u32 = 0x0101_a208;

// Callees and data of the third session (`00473d20` to `00474880`): the
// global file-key map, the zlib stream and the container helpers.
/// The global map from a key (a temporary id) to a `TESFile`, created by
/// [`fn_00473f20`] and destroyed when the last file registered in it goes.
const GLOBAL_FILE_MAP: u32 = 0x011c_40b0;
/// How many files are registered in [`GLOBAL_FILE_MAP`].
const GLOBAL_FILE_MAP_USERS: u32 = 0x011c_40b4;
/// `m_Flags` bit 5 (0x20): this file is registered in [`GLOBAL_FILE_MAP`].
const FLAG_REGISTERED: u32 = 0x20;
/// The map's remove-by-key (`00405430`, `(map; key)`), the key array's
/// element accessor (`00877a30`, `(array; index)`: the address of the
/// element slot) and its `SetAtGrow` (`00470000`, `(array; index,
/// &element)`, the element passed by address).
const FILE_MAP_REMOVE_AT: u32 = 0x0040_5430;
const KEY_ARRAY_ELEMENT: u32 = 0x0087_7a30;
const KEY_ARRAY_SET_AT_GROW: u32 = 0x0047_0000;
/// The scope guard around the decompression: `00404eb0(guard; kind, 1,
/// source file, line)` and `00404ee0(guard)`.
const SCOPE_GUARD_OPEN: u32 = 0x0040_4eb0;
const SCOPE_GUARD_CLOSE: u32 = 0x0040_4ee0;
const SCOPE_GUARD_KIND: u32 = 0x30;
const SOURCE_FILE_NAME: u32 = 0x0101_a2e0;
const DECOMPRESS_LINE: u32 = 0xf8e;
/// zlib 1.2.1 (`inflateInit_(stream, version, stream size)`,
/// `inflate(stream, flush)`, `inflateEnd(stream)`), all cdecl; the version
/// string and the size of a `z_stream`.
const INFLATE_INIT: u32 = 0x00b4_3fe0;
const INFLATE: u32 = 0x00b4_4000;
const INFLATE_END: u32 = 0x00b4_5db0;
const ZLIB_VERSION: u32 = 0x0101_a29c;
const Z_STREAM_SIZE: u32 = 0x38;
/// Size of the inflate state area the game reserves on its stack.
const INFLATE_STATE_SIZE: u32 = 7084;
/// zlib return codes the decompression tells apart.
const Z_STREAM_END: i32 = 1;
const Z_NEED_DICT: i32 = 2;
const Z_STREAM_ERROR: i32 = -2;
const Z_DATA_ERROR: i32 = -3;
const Z_MEM_ERROR: i32 = -4;
const MESSAGE_COMPRESSED_READ_FAILED: u32 = 0x0101_a2a4;
const MESSAGE_INFLATE_INIT_FAILED: u32 = 0x0101_a268;
const MESSAGE_INFLATE_FAILED: u32 = 0x0101_a240;
const MESSAGE_INFLATE_NOT_TERMINATED: u32 = 0x0101_a210;
/// The memory manager: `00401020()` answers the singleton, whose
/// `Allocate(size)` is `00aa3e40` and `Deallocate(block)` `00aa4060`.
const MEMORY_MANAGER_GET: u32 = 0x0040_1020;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// Allocation of a byte count and release of a block (`00aa1070(bytes)`,
/// `00aa10f0(block)`), cdecl.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// Allocation of `count` pointers (`0096afc0(count)`) and release of an
/// array's elements (`004ede70(block)`), cdecl.
const ALLOCATE_POINTERS: u32 = 0x0096_afc0;
const FREE_ARRAY_ELEMENTS: u32 = 0x004e_de70;
/// Vtables the container constructors store: the `NiTPointerMap` family
/// (`NiTMapBase<NiTPointerAllocator>` base, then the derived class), the
/// `NiTMap` family, and the key array's base and derived classes.
const VTABLE_POINTER_MAP: u32 = 0x0101_a324;
const VTABLE_POINTER_MAP_BASE: u32 = 0x0101_a364;
const VTABLE_MAP: u32 = 0x0101_a344;
const VTABLE_MAP_BASE: u32 = 0x0101_a384;
const VTABLE_KEY_ARRAY_BASE: u32 = 0x0101_a3a4;
const VTABLE_KEY_ARRAY: u32 = 0x0101_a3ac;
/// The zlib allocation and release hooks of the decompression
/// ([`fn_00474460`], [`fn_00474480`]), stored in the `z_stream`.
const ZALLOC_HOOK: u32 = 0x0047_4460;
const ZFREE_HOOK: u32 = 0x0047_4480;
/// Bucket count of the global file map.
const GLOBAL_FILE_MAP_BUCKETS: u32 = 0x3e9;

/// The word at this address is the `whence` the code passes to `BSFile`'s
/// seek (0 in the exe).
pub(crate) const SEEK_MODE: u32 = 0x010a_2480;
/// The `whence` that seeks from the end of the file (2 in the exe).
const SEEK_FROM_END: u32 = 0x010a_2488;
/// `"XXXX"` as a little-endian word: a chunk whose data is the real size of
/// the chunk after it.
const CHUNK_ID_XXXX: u32 = 0x5858_5858;
/// Bits of a form's flags word that are saved in the record header.
const SAVED_FORM_FLAGS_MASK: u32 = 0x3003_2fe0;
/// `TES_RETURN_CODE` 10: a write to the file came up short.
const RETURN_CODE_WRITE_ERROR: u32 = 10;
/// Size of a `TESFile` (the allocation of a per-thread copy).
const TES_FILE_SIZE: u32 = 0x42c;
/// `(form) -> text`: the name of the form's type (`00440e30`: the string of
/// the form's type index in the tag table at `01187004`).
const FORM_TYPE_NAME: u32 = 0x0044_0e30;
/// Globals written by the form that is being saved: the buffer pointer
/// (`011c54cc`) and its size (`011c54d0`).
const SAVE_BUFFER: u32 = 0x011c_54cc;
const SAVE_BUFFER_SIZE: u32 = 0x011c_54d0;
/// The `TESDataHandler` singleton pointer.
pub(crate) const DATA_HANDLER: u32 = 0x011c_3f2c;
/// A global the stream code passes as `this` to [`ALWAYS_TRUE`].
const STREAM_OBJECT: u32 = 0x011f_6078;
/// `TESFile +0x22C`'s initial value.
const DEFAULT_BUFFER_SIZE: u32 = 0x0118_6740;
/// The `float` the constructor stores as the header version.
const DEFAULT_HEADER_VERSION: u32 = 0x0101_9dd4;
/// The `float` stored as the header version when a file reports another one
/// than [`EXPECTED_HEADER_VERSION`] (a `double`, 0.85).
const FALLBACK_HEADER_VERSION: u32 = 0x0101_9de0;
const EXPECTED_HEADER_VERSION: u32 = 0x0101_9de8;
/// `".tes"`: the extension appended when a file is not found.
const TES_EXTENSION: u32 = 0x0101_9dd8;
/// Record tags compared against the first word of `m_currentform`; they are
/// set at run time. `01187014` is compared when the file's first record is
/// read (the endian check); for the other two only the header is skipped,
/// not `length` bytes more.
pub(crate) const RECORD_TAG_01187014: u32 = 0x0118_7014;
pub(crate) const RECORD_TAG_01187020: u32 = 0x0118_7020;
pub(crate) const RECORD_TAG_011873C8: u32 = 0x0118_73c8;
/// `"HEDR"` as a little-endian word: the id of a file's first chunk.
const CHUNK_ID_HEDR: u32 = 0x5244_4548;

// Messages (format strings in the exe).
const MESSAGE_NOT_VALID_TES_FILE: u32 = 0x0101_9da4;
const MESSAGE_CLOSE_FAILED: u32 = 0x0101_9d7c;
const MESSAGE_RENAME_FAILED: u32 = 0x0101_9df0;
const MESSAGE_DELETE_FAILED: u32 = 0x0101_9e68;
const MESSAGE_MISSING_MASTER: u32 = 0x0101_9f0c;
const MESSAGE_NEXT_FORM_BAD_FORM: u32 = 0x0101_9f30;
const MESSAGE_NEXT_FORM_SEEK_FAILED: u32 = 0x0101_9f84;
const MESSAGE_SET_OFFSET_SEEK_FAILED: u32 = 0x0101_9fc0;
const MESSAGE_SET_OFFSET_CHUNK_SEEK_FAILED: u32 = 0x0101_a000;
const MESSAGE_SET_OFFSET_CHUNK_COMPRESSED: u32 = 0x0101_a040;
/// The format `00471af0` builds a master's path with.
const MASTER_PATH_FORMAT: u32 = 0x0101_9f08;

// Windows imports, by import slot.
const FIND_CLOSE: u32 = 0x00fd_f064;
const FIND_FIRST_FILE: u32 = 0x00fd_f070;
const LSTRCAT: u32 = 0x00fd_f074;
const LSTRCPY: u32 = 0x00fd_f078;
const LOCAL_FREE: u32 = 0x00fd_f08c;
const FORMAT_MESSAGE: u32 = 0x00fd_f090;
const GET_LAST_ERROR: u32 = 0x00fd_f094;
/// `FindFirstFileA`'s failure value.
const INVALID_HANDLE: u32 = 0xffff_ffff;
/// Size of a path buffer.
const PATH_BUFFER_SIZE: u32 = 0x104;
/// Size of a `WIN32_FIND_DATAA`.
const FIND_DATA_SIZE: u32 = 0x140;
/// Size of the `BSFile` the open allocates.
const BSFILE_SIZE: u32 = 0x158;

/// `TES_RETURN_CODE` values this unit stores in `m_lastError`.
const RETURN_CODE_NOT_FOUND: u32 = 2;
const RETURN_CODE_FAILED_TO_WRITE: u32 = 9;
const RETURN_CODE_FOUND_AS_TES: u32 = 0xc;

/// `m_Flags` bits (`TESFile +0x3E8`). `GetMaster`, `GetOptimizedFile` and
/// `GetActive` name three of them; the others are described by the code
/// that sets them.
const FLAG_MASTER: u32 = 0x01;
/// Set when the file's find data changed since the last open.
const FLAG_FIND_DATA_CHANGED: u32 = 0x02;
const FLAG_BIT_4: u32 = 0x04;
const FLAG_ACTIVE: u32 = 0x08;
const FLAG_OPTIMIZED: u32 = 0x10;
const FLAG_BIT_40: u32 = 0x40;
/// Bit of the record flags (`FORM +8`) of a compressed record.
const RECORD_FLAG_COMPRESSED: u32 = 0x0004_0000;
/// Bit of the record flags that `NextForm` skips records with, when asked.
const RECORD_FLAG_SKIPPED_BY_NEXT_FORM: u32 = 0x1000;

layout! {
    /// `FORM` (Xbox PDB): the 0x18-byte header of a record in the stream.
    #[allow(clippy::upper_case_acronyms)]
    pub struct Form: 0x18 {
        /// `form` (Xbox PDB): the record's four-character tag as a word.
        0x00 form: u32,
        /// `length` (Xbox PDB).
        0x04 length: u32,
        /// `flags` (Xbox PDB).
        0x08 flags: u32,
        /// `iFormID` (Xbox PDB).
        0x0C iFormID: u32,
        /// `iVersionControl` (Xbox PDB).
        0x10 iVersionControl: u32,
        /// `sFormVersion` (Xbox PDB).
        0x14 sFormVersion: u16,
        /// `sVCVersion` (Xbox PDB).
        0x16 sVCVersion: u16,
    }

    /// `FILE_HEADER` (Xbox PDB), 0xC bytes.
    pub struct FileHeader: 0x0C {
        /// `fVersion` (Xbox PDB).
        0x00 fVersion: f32,
        /// `iFormCount` (Xbox PDB).
        0x04 iFormCount: u32,
        /// `iNextFormID` (Xbox PDB).
        0x08 iNextFormID: u32,
    }

    /// `FORM_GROUP` (Xbox PDB), 0x1C bytes: a group entry of
    /// `m_grouplist`.
    pub struct FormGroup: 0x1C {
        /// `GroupData` (Xbox PDB): the group's record header.
        0x00 GroupData: Inline<Form>,
        /// `iGroupOffset` (Xbox PDB): the file offset the header was
        /// written at.
        0x18 iGroupOffset: u32,
    }

    /// A `char[260]` path or name buffer embedded in an object.
    pub struct PathBuffer: 0x104 {}

    /// `_WIN32_FIND_DATAA` (0x140 bytes): only the sizes are read here.
    pub struct Win32FindData: 0x140 {
        /// `nFileSizeHigh`.
        0x1C nFileSizeHigh: u32,
        /// `nFileSizeLow`.
        0x20 nFileSizeLow: u32,
    }

    /// `TESFile` (Xbox PDB), 0x42C bytes on both builds; names are the Xbox
    /// PDB's, offsets were checked against this unit's code.
    pub struct TESFile: 0x42C {
        /// `m_lastError` (Xbox PDB): a `TES_RETURN_CODE`.
        0x000 m_lastError: u32,
        /// `pThreadSafeParent` (Xbox PDB).
        0x004 pThreadSafeParent: Ptr,
        /// `pThreadSafeFileMap` (Xbox PDB).
        0x008 pThreadSafeFileMap: Ptr,
        /// `m_pLockedFile` (Xbox PDB): the temporary `BSFile` being saved.
        0x00C m_pLockedFile: Ptr,
        /// `m_pFile` (Xbox PDB): the open `BSFile`.
        0x010 m_pFile: Ptr,
        /// `m_Filename` (Xbox PDB): `char[260]`.
        0x020 m_Filename: Inline<PathBuffer>,
        /// `m_Path` (Xbox PDB): `char[260]`.
        0x124 m_Path: Inline<PathBuffer>,
        /// `m_pBuffer` (Xbox PDB).
        0x228 m_pBuffer: Ptr,
        /// `m_uiBufferAllocSize` (Xbox PDB).
        0x22C m_uiBufferAllocSize: u32,
        /// `m_firstCellOffset` (Xbox PDB).
        0x230 m_firstCellOffset: u32,
        /// `m_currCellOffset` (Xbox PDB).
        0x234 m_currCellOffset: u32,
        /// `m_pCurrCell` (Xbox PDB).
        0x238 m_pCurrCell: Ptr,
        /// `m_currRefOffset` (Xbox PDB).
        0x23C m_currRefOffset: u32,
        /// `m_currentform` (Xbox PDB): the current record's header.
        0x240 m_currentform: Inline<Form>,
        /// `m_currentchunkID` (Xbox PDB).
        0x258 m_currentchunkID: u32,
        /// `m_actualChunkSize` (Xbox PDB).
        0x25C m_actualChunkSize: u32,
        /// `m_filesize` (Xbox PDB).
        0x260 m_filesize: u32,
        /// `m_fileoffset` (Xbox PDB): the offset of the current record.
        0x264 m_fileoffset: u32,
        /// `m_formoffset` (Xbox PDB): the cursor inside the record.
        0x268 m_formoffset: u32,
        /// `m_chunkoffset` (Xbox PDB).
        0x26C m_chunkoffset: u32,
        /// `m_saveform` (Xbox PDB).
        0x270 m_saveform: Inline<Form>,
        /// `m_saveformoffset` (Xbox PDB).
        0x288 m_saveformoffset: u32,
        /// `m_savechunkoffset` (Xbox PDB).
        0x28C m_savechunkoffset: u32,
        /// `m_grouplist` (Xbox PDB): a `BSSimpleList<FORM_GROUP *>`.
        0x290 m_grouplist: Inline<BSSimpleList>,
        /// `bHasGroups` (Xbox PDB).
        0x298 bHasGroups: bool,
        /// `bMustEndianConvert` (Xbox PDB).
        0x299 bMustEndianConvert: bool,
        /// `bCloseFileOverride` (Xbox PDB).
        0x29A bCloseFileOverride: bool,
        /// `m_FileInfo` (Xbox PDB): the last `WIN32_FIND_DATA` of the file.
        0x29C m_FileInfo: Inline<Win32FindData>,
        /// `fileHeaderInfo` (Xbox PDB).
        0x3DC fileHeaderInfo: Inline<FileHeader>,
        /// `m_Flags` (Xbox PDB).
        0x3E8 m_Flags: u32,
        /// `listMasters` (Xbox PDB): a `BSSimpleList<char *>`.
        0x3EC listMasters: Inline<BSSimpleList>,
        /// `listMastersData` (Xbox PDB): a `BSSimpleList<_ULARGE_INTEGER *>`.
        0x3F4 listMastersData: Inline<BSSimpleList>,
        /// `iMasterCount` (Xbox PDB).
        0x3FC iMasterCount: u32,
        /// `m_pMasterPtrs` (Xbox PDB): a `TESFile *` per master, built by
        /// `GenIndexTable`.
        0x400 m_pMasterPtrs: Ptr,
        /// `DeletedFormTime` (Xbox PDB): a `_FILETIME`.
        0x404 DeletedFormTime: u64,
        /// `cCompileIndex` (Xbox PDB).
        0x40C cCompileIndex: u8,
        /// `cCreatedBy` (Xbox PDB): a `BSStringT<char>`.
        0x410 cCreatedBy: Inline<BSStringT>,
        /// `cSummary` (Xbox PDB): a `BSStringT<char>`.
        0x418 cSummary: Inline<BSStringT>,
        /// `pDecompressedFormBuffer` (Xbox PDB).
        0x420 pDecompressedFormBuffer: Ptr,
        /// `iDecompressedFormBufferSize` (Xbox PDB).
        0x424 iDecompressedFormBufferSize: u32,
        /// `bCached` (Xbox PDB).
        0x428 bCached: bool,
        /// `bCaching` (Xbox PDB).
        0x429 bCaching: bool,
        /// `bDLC` (Xbox PDB).
        0x42A bDLC: bool,
    }
}

/// The address of the item slot of a list node (`006815c0`).
fn list_item_slot(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_ITEM_SLOT, &args![node]).u32()
}

/// The item of a list node.
fn list_item(e: &mut Engine, node: u32) -> u32 {
    let slot = list_item_slot(e, node);
    e.mem.u32(slot)
}

/// The node after `node` (`00726070`).
fn list_next(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_NEXT, &args![node]).u32()
}

/// `listMasters` of `this` (`00464df0`).
fn masters_list(e: &mut Engine, this: Ptr<TESFile>) -> u32 {
    e.call(MASTERS_LIST, &args![this]).u32()
}

fn delete(e: &mut Engine, block: u32) {
    e.call(OPERATOR_DELETE, &args![block]);
}

/// Deletes every non-null item of the list starting at `node`.
fn delete_list_items(e: &mut Engine, mut node: u32) {
    while node != 0 {
        let item = list_item(e, node);
        if item != 0 {
            delete(e, item);
        }
        node = list_next(e, node);
    }
}

/// The logging call with a format address and its arguments.
fn log(e: &mut Engine, words: &[u32]) {
    e.call(LOG, words);
}

/// Seeks the `BSFile` to `offset` (vtable slot `+0x14`, `whence` from the
/// global at [`SEEK_MODE`]).
fn seek(e: &mut Engine, file: Ptr, offset: u32) {
    let whence: u32 = e.global(SEEK_MODE);
    e.vcall(file.addr(), 0x14, &args![offset, whence]);
}

/// Seeks the `BSFile` to `offset` counted from the end of the file (vtable
/// slot `+0x14`, `whence` from the global at [`SEEK_FROM_END`]).
fn seek_from_end(e: &mut Engine, file: Ptr, offset: u32) {
    let whence: u32 = e.global(SEEK_FROM_END);
    e.vcall(file.addr(), 0x14, &args![offset, whence]);
}

/// `GetLastError` + `FormatMessageA(0x1300, 0, error, 0x400, &text, 0, 0)`,
/// then the log call with `format` and the text, then `LocalFree(text)`:
/// the report of a failed `SetFilePointer`.
fn report_set_file_pointer_failure(e: &mut Engine, format: u32) {
    e.with_stack(4, |e, text| {
        let error = e.call(GET_LAST_ERROR, &args![]).u32();
        e.call(
            FORMAT_MESSAGE,
            &args![0x1300u32, 0u32, error, 0x400u32, text, 0u32, 0u32],
        );
        let message = e.mem.u32(text.addr());
        log(e, &args![format, message]);
        e.call(LOCAL_FREE, &args![message]);
    });
}

/// Zeroes the current record header (`m_currentform`).
fn clear_current_form(e: &mut Engine, this: Ptr<TESFile>) {
    e.call(
        MEMSET,
        &args![this.at(TESFile::m_currentform), 0i32, 0x18u32],
    );
}

fn modify_flags(e: &mut Engine, this: Ptr<TESFile>, set: u32, clear: u32) {
    let flags = e.get(this, TESFile::m_Flags);
    e.set(this, TESFile::m_Flags, (flags | set) & !clear);
}

fn flag_is_set(e: &Engine, this: Ptr<TESFile>, mask: u32) -> bool {
    e.get(this, TESFile::m_Flags) & mask != 0
}

// Translated from 00470650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORM::Endian` (Xbox PDB): swaps the byte order of the five words and the
/// two halfwords of a record header in place.
pub fn form_endian(e: &mut Engine, this: Ptr<Form>) {
    for offset in [0x00u32, 0x04, 0x08, 0x0c, 0x10] {
        let word = this.byte_add(offset);
        e.call(SWAP_U32, &args![word, 0u32]);
    }
    for offset in [0x14u32, 0x16] {
        let half = this.byte_add(offset);
        e.call(SWAP_U16, &args![half, 0u32]);
    }
}

// Translated from 00470710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::TESFile` (Xbox PDB): constructs the file object, opens the
/// named file to read its header information, and closes it again. Returns
/// `this`. The compiler's exception-unwinding frame is not translated.
pub fn tes_file_tes_file(
    e: &mut Engine,
    this: Ptr<TESFile>,
    directory: Ptr,
    file_name: Ptr,
    open_mode: u32,
) -> Ptr<TESFile> {
    e.call(LIST_CONSTRUCT, &args![this.at(TESFile::m_grouplist)]);
    e.call(LIST_CONSTRUCT, &args![this.at(TESFile::listMasters)]);
    e.call(LIST_CONSTRUCT, &args![this.at(TESFile::listMastersData)]);
    e.call(STRING_CONSTRUCT, &args![this.at(TESFile::cCreatedBy)]);
    e.call(STRING_CONSTRUCT, &args![this.at(TESFile::cSummary)]);
    e.set(this, TESFile::bCached, false);
    e.set(this, TESFile::bCloseFileOverride, false);
    e.set(this, TESFile::pDecompressedFormBuffer, Ptr::NULL);
    e.set(this, TESFile::iDecompressedFormBufferSize, 0);
    e.set(this, TESFile::m_firstCellOffset, 0);
    e.set(this, TESFile::m_currCellOffset, 0);
    e.set(this, TESFile::m_pCurrCell, Ptr::NULL);
    e.set(this, TESFile::pThreadSafeParent, Ptr::NULL);
    e.set(this, TESFile::pThreadSafeFileMap, Ptr::NULL);
    e.set(this, TESFile::bMustEndianConvert, false);
    e.set(this, TESFile::m_lastError, 0);
    e.set(this, TESFile::m_filesize, 0);
    e.set(this, TESFile::m_fileoffset, 0);
    e.set(this, TESFile::m_formoffset, 0);
    e.set(this, TESFile::m_chunkoffset, 0);
    e.set(this, TESFile::m_Flags, 0);
    e.set(this, TESFile::m_pMasterPtrs, Ptr::NULL);
    e.set(this, TESFile::m_pLockedFile, Ptr::NULL);
    e.set(this, TESFile::m_pFile, Ptr::NULL);
    e.set(this, TESFile::iMasterCount, 0);
    e.set(this, TESFile::cCompileIndex, 0xff);
    e.set(this, TESFile::m_currRefOffset, 0);
    e.set(this, TESFile::m_saveformoffset, 0);
    e.set(this, TESFile::m_savechunkoffset, 0);
    e.set(this, TESFile::m_pBuffer, Ptr::NULL);
    let buffer_size: u32 = e.global(DEFAULT_BUFFER_SIZE);
    e.set(this, TESFile::m_uiBufferAllocSize, buffer_size);
    clear_current_form(e, this);
    e.call(
        MEMSET,
        &args![this.at(TESFile::m_FileInfo), 0i32, FIND_DATA_SIZE],
    );
    e.call(MEMSET, &args![this.byte_add(0x404), 0i32, 8u32]);
    e.set(this, TESFile::m_actualChunkSize, 0);
    e.set(this, TESFile::m_currentchunkID, 0);
    let header = this.at(TESFile::fileHeaderInfo);
    e.call(MEMSET, &args![header, 0i32, 0xcu32]);
    e.set(header, FileHeader::iNextFormID, 0x800);
    let version: f32 = e.global(DEFAULT_HEADER_VERSION);
    e.set(header, FileHeader::fVersion, version);

    if tes_file_open_tes_ov2(e, this, directory, file_name, open_mode, 0) {
        let info = this.at(TESFile::m_FileInfo);
        let size_high = e.get(info, Win32FindData::nFileSizeHigh);
        let size_low = e.get(info, Win32FindData::nFileSizeLow);
        if (size_high != 0 || size_low != 0) && e.call(READ_HEADER_STATUS, &args![this]).i32() != 0
        {
            log(e, &args![MESSAGE_NOT_VALID_TES_FILE, file_name]);
        }
        if !tes_file_close_tes(e, this) {
            log(e, &args![MESSAGE_CLOSE_FAILED, file_name]);
        }
    }
    this
}

// Translated from 004709f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESFile` destructor body: closes the file (with the close override
/// set), deletes the strings of both master lists and empties them, frees
/// the buffer and the master pointer table, and destroys the members. The
/// compiler's exception-unwinding frame is not translated.
pub fn fn_004709f0(e: &mut Engine, this: Ptr<TESFile>) {
    e.set(this, TESFile::bCloseFileOverride, true);
    tes_file_close_tes(e, this);
    let first = masters_list(e, this);
    delete_list_items(e, first);
    let list = masters_list(e, this);
    e.call(LIST_CLEAR, &args![list]);
    let first = fn_00470b90(e, this).addr();
    delete_list_items(e, first);
    let list = fn_00470b90(e, this);
    e.call(LIST_CLEAR, &args![list]);
    let buffer = e.get(this, TESFile::m_pBuffer);
    delete(e, buffer.addr());
    let master_pointers = e.get(this, TESFile::m_pMasterPtrs);
    delete(e, master_pointers.addr());
    e.set(this, TESFile::m_pMasterPtrs, Ptr::NULL);
    e.call(DESTRUCT_CLEANUP_A, &args![this]);
    e.call(DESTRUCT_CLEANUP_B, &args![this]);
    e.call(STRING_DESTRUCT, &args![this.at(TESFile::cSummary)]);
    e.call(STRING_DESTRUCT, &args![this.at(TESFile::cCreatedBy)]);
    e.call(LIST_DESTRUCT, &args![this.at(TESFile::listMastersData)]);
    e.call(LIST_DESTRUCT, &args![this.at(TESFile::listMasters)]);
    e.call(LIST_DESTRUCT, &args![this.at(TESFile::m_grouplist)]);
}

// Translated from 00470b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of `listMastersData` (`this + 0x3F4`).
pub fn fn_00470b90(_e: &mut Engine, this: Ptr<TESFile>) -> Ptr<BSSimpleList> {
    this.at(TESFile::listMastersData)
}

// Translated from 00470bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 0xF (cdecl, no arguments).
pub fn fn_00470bb0(_e: &mut Engine) -> u32 {
    0xf
}

// Translated from 00470bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the first string of `listMasters`, drops its node, and repeats
/// until the list is empty; the same for `listMastersData`; then zeroes
/// `iMasterCount`.
pub fn fn_00470bc0(e: &mut Engine, this: Ptr<TESFile>) {
    loop {
        let list = masters_list(e, this);
        if list_item(e, list) == 0 {
            break;
        }
        let list = masters_list(e, this);
        let item = list_item(e, list);
        delete(e, item);
        let list = masters_list(e, this);
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
    loop {
        let list = fn_00470b90(e, this).addr();
        if list_item(e, list) == 0 {
            break;
        }
        let list = fn_00470b90(e, this).addr();
        let item = list_item(e, list);
        delete(e, item);
        let list = fn_00470b90(e, this).addr();
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
    e.set(this, TESFile::iMasterCount, 0);
}

// Translated from 00470c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::OpenTES` (Xbox PDB): opens the file named by `m_Path` and
/// `m_Filename` (see [`tes_file_open_tes_ov2`]).
pub fn tes_file_open_tes(
    e: &mut Engine,
    this: Ptr<TESFile>,
    open_mode: u32,
    write_flag: u8,
) -> bool {
    let directory = this.at(TESFile::m_Path).cast();
    let file_name = this.at(TESFile::m_Filename).cast();
    tes_file_open_tes_ov2(e, this, directory, file_name, open_mode, write_flag)
}

// Translated from 00470ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::OpenTES` (Xbox PDB, second overload): remembers `directory`
/// and `file_name`, finds the file (with `.tes` appended when it is not
/// found as named, which only sets `m_lastError`), opens a `BSFile` on it
/// unless one is open, records the file size, reads the first record header
/// when `open_mode` is 0 and rewinds. If the first chunk is not `HEDR` and
/// the header version is not 0.85 it stores the fallback version and opens
/// again. `write_flag` forces `open_mode` 1 and calls the stub at
/// `00473880`. Answers false when the file could not be opened.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn tes_file_open_tes_ov2(
    e: &mut Engine,
    this: Ptr<TESFile>,
    directory: Ptr,
    file_name: Ptr,
    open_mode: u32,
    write_flag: u8,
) -> bool {
    if directory.is_null() || file_name.is_null() {
        return false;
    }
    // The path buffer and two `WIN32_FIND_DATAA` of the game's stack frame.
    let scratch = e.mem.alloc(PATH_BUFFER_SIZE + 2 * FIND_DATA_SIZE);
    let result = open_with_scratch(
        e, this, directory, file_name, open_mode, write_flag, scratch,
    );
    e.mem.free(scratch);
    result
}

fn open_with_scratch(
    e: &mut Engine,
    this: Ptr<TESFile>,
    directory: Ptr,
    file_name: Ptr,
    open_mode: u32,
    write_flag: u8,
    scratch: u32,
) -> bool {
    let path = scratch;
    let found = scratch + PATH_BUFFER_SIZE;
    let found_with_extension = found + FIND_DATA_SIZE;
    let mut open_mode = open_mode;
    if write_flag != 0 {
        open_mode = 1;
    }
    let remembered_path: Ptr = this.at(TESFile::m_Path).cast();
    let remembered_name: Ptr = this.at(TESFile::m_Filename).cast();
    e.call(STRCPY_S, &args![path, PATH_BUFFER_SIZE, directory]);
    e.call(
        STRCPY_S,
        &args![remembered_path, PATH_BUFFER_SIZE, directory],
    );
    e.call(STRCAT_S, &args![path, PATH_BUFFER_SIZE, file_name]);
    e.call(
        STRCPY_S,
        &args![remembered_name, PATH_BUFFER_SIZE, file_name],
    );

    if e.get(this, TESFile::m_pFile).is_null() {
        let handle = e.call(FIND_FIRST_FILE, &args![path, found]).u32();
        if handle == INVALID_HANDLE {
            // Not found as named: look for the name with ".tes" instead.
            e.call(LSTRCPY, &args![path, remembered_path]);
            let dot = e.call(STRCHR, &args![remembered_name, 0x2eu32]).u32();
            if dot != 0 {
                e.mem.set_u8(dot, 0);
                e.call(LSTRCAT, &args![path, remembered_name]);
                e.call(LSTRCAT, &args![path, TES_EXTENSION]);
                e.mem.set_u8(dot, b'.');
            } else {
                e.call(LSTRCAT, &args![path, remembered_name]);
                e.call(LSTRCAT, &args![path, TES_EXTENSION]);
            }
            let handle = e
                .call(FIND_FIRST_FILE, &args![path, found_with_extension])
                .u32();
            if handle == INVALID_HANDLE {
                e.set(this, TESFile::m_lastError, RETURN_CODE_NOT_FOUND);
            } else {
                e.set(this, TESFile::m_lastError, RETURN_CODE_FOUND_AS_TES);
                e.call(FIND_CLOSE, &args![handle]);
            }
            return false;
        }
        e.call(FIND_CLOSE, &args![handle]);
        let info = this.at(TESFile::m_FileInfo);
        if e.call(MEMCMP, &args![info, found, FIND_DATA_SIZE]).i32() != 0 {
            modify_flags(e, this, FLAG_FIND_DATA_CHANGED, 0);
            e.call(MEMCPY, &args![info, found, FIND_DATA_SIZE]);
        }
        let block = e.call(OPERATOR_NEW, &args![BSFILE_SIZE]).u32();
        let file = if block != 0 {
            let buffer_size = e.get(this, TESFile::m_uiBufferAllocSize);
            e.call(
                BSFILE_CONSTRUCT,
                &args![block, path, open_mode, buffer_size, 0u32],
            )
            .u32()
        } else {
            0
        };
        e.set(this, TESFile::m_pFile, Ptr::new(file));
        if file == 0 {
            return false;
        }
        e.vcall(file, 0x20, &args![1u32, 0u32]);
        if !e.call(FILE_IS_READY, &args![file]).bool() {
            let error_slot = e.call(ERRNO, &args![]).u32();
            let error = e.mem.u32(error_slot);
            if error == 2 {
                e.set(this, TESFile::m_lastError, RETURN_CODE_NOT_FOUND);
            } else if error == 0xd {
                e.set(this, TESFile::m_lastError, RETURN_CODE_FAILED_TO_WRITE);
            }
            return false;
        }
    }

    let file = e.get(this, TESFile::m_pFile);
    if e.call(FILE_IS_READY, &args![file]).bool() && write_flag != 0 {
        e.call(STUB_RETURNS_ZERO, &args![this]);
    }
    let file = e.get(this, TESFile::m_pFile);
    let size = e.vcall(file.addr(), 0x28, &args![]).u32();
    e.set(this, TESFile::m_filesize, size);
    if open_mode == 0 {
        fn_00471e60(e, this);
    }
    tes_file_tes_rewind(e, this, (open_mode == 0) as u8);
    let mut opened = true;
    if e.get(this, TESFile::m_currentchunkID) != CHUNK_ID_HEDR {
        let version = e.call(HEADER_VERSION, &args![this]).f32();
        let expected: f64 = e.global(EXPECTED_HEADER_VERSION);
        if version as f64 != expected {
            let fallback: f32 = e.global(FALLBACK_HEADER_VERSION);
            fn_00471110(e, this, fallback);
            opened = tes_file_open_tes_ov2(e, this, directory, file_name, open_mode, 0);
        }
    }
    opened
}

// Translated from 00471110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the header version (`fileHeaderInfo.fVersion`).
pub fn fn_00471110(e: &mut Engine, this: Ptr<TESFile>, version: f32) {
    e.set(
        this.at(TESFile::fileHeaderInfo),
        FileHeader::fVersion,
        x87_quiet(version),
    );
}

// Translated from 00471130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::CloseTES` (Xbox PDB): closes the groups and the file, and, when
/// a temporary save file is open, closes it, deletes the original and
/// renames the temporary file over it. Answers false when the delete or the
/// rename failed (and sets `m_lastError` to 9). When the close override is
/// off and the data handler's flag byte is 0 nothing is done and the answer
/// is true.
pub fn tes_file_close_tes(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    let handler: u32 = e.global(DATA_HANDLER);
    if !e.get(this, TESFile::bCloseFileOverride)
        && handler != 0
        && e.call(HANDLER_FLAG, &args![handler]).u8() == 0
    {
        return true;
    }
    e.call(CLOSE_ALL_GROUPS, &args![this]);
    let file = e.get(this, TESFile::m_pFile);
    if !file.is_null() {
        e.vcall(file.addr(), 0, &args![1u32]);
        e.set(this, TESFile::m_pFile, Ptr::NULL);
    }
    e.call(FREE_DECOMPRESSED_FORM, &args![this]);
    let locked = e.get(this, TESFile::m_pLockedFile);
    if locked.is_null() {
        return true;
    }
    let scratch = e.mem.alloc(2 * PATH_BUFFER_SIZE);
    let result = replace_with_saved_file(e, this, scratch);
    e.mem.free(scratch);
    result
}

/// The second half of `CloseTES`: `scratch` holds the original file's path
/// and the temporary file's path.
fn replace_with_saved_file(e: &mut Engine, this: Ptr<TESFile>, scratch: u32) -> bool {
    let original = scratch;
    let temporary = scratch + PATH_BUFFER_SIZE;
    let path: Ptr = this.at(TESFile::m_Path).cast();
    let name: Ptr = this.at(TESFile::m_Filename).cast();
    e.call(STRCPY_S, &args![original, PATH_BUFFER_SIZE, path]);
    e.call(STRCAT_S, &args![original, PATH_BUFFER_SIZE, name]);
    e.call(STRCPY_S, &args![temporary, PATH_BUFFER_SIZE, path]);
    let dot = e.call(STRCHR, &args![name, 0x2eu32]).u32();
    if dot != 0 {
        e.mem.set_u8(dot, 0);
        e.call(STRCAT_S, &args![temporary, PATH_BUFFER_SIZE, name]);
        e.call(STRCAT_S, &args![temporary, PATH_BUFFER_SIZE, TES_EXTENSION]);
        e.mem.set_u8(dot, b'.');
    } else {
        e.call(STRCAT_S, &args![temporary, PATH_BUFFER_SIZE, name]);
        e.call(STRCAT_S, &args![temporary, PATH_BUFFER_SIZE, TES_EXTENSION]);
    }
    let locked = e.get(this, TESFile::m_pLockedFile);
    e.vcall(locked.addr(), 0, &args![1u32]);
    e.set(this, TESFile::m_pLockedFile, Ptr::NULL);
    if e.call(DELETE_SYSTEM_FILE, &args![original]).u32() != 0 {
        e.call(NOTE_FAILURE, &args![]);
        e.set(this, TESFile::m_lastError, RETURN_CODE_FAILED_TO_WRITE);
        log(e, &args![MESSAGE_DELETE_FAILED, original, temporary]);
        return false;
    }
    if e.call(RENAME, &args![temporary, original]).i32() != 0 {
        e.call(NOTE_FAILURE, &args![]);
        e.set(this, TESFile::m_lastError, RETURN_CODE_FAILED_TO_WRITE);
        log(e, &args![MESSAGE_RENAME_FAILED, temporary]);
        return false;
    }
    true
}

// Translated from 00471850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `iMasterCount`.
pub fn fn_00471850(e: &mut Engine, this: Ptr<TESFile>) -> u32 {
    e.get(this, TESFile::iMasterCount)
}

// Translated from 00471870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GenIndexTable` (Xbox PDB): rebuilds `m_pMasterPtrs`, one
/// `TESFile *` per master name, by looking each name up (case-insensitively)
/// in the `files` list (a `BSSimpleList<TESFile *>` head). A master that is
/// not in the list gets a null entry, is reported when `report_missing` is
/// set, and makes the answer false.
pub fn tes_file_gen_index_table(
    e: &mut Engine,
    this: Ptr<TESFile>,
    files: Ptr,
    report_missing: u8,
) -> bool {
    let mut master_node = masters_list(e, this);
    let count = fn_00471850(e, this);
    let old_table = e.get(this, TESFile::m_pMasterPtrs);
    if !old_table.is_null() {
        delete(e, old_table.addr());
    }
    e.set(this, TESFile::m_pMasterPtrs, Ptr::NULL);
    if count == 0 {
        return true;
    }
    let bytes = if (count as u64 * 4) >> 32 != 0 {
        u32::MAX
    } else {
        count * 4
    };
    let table = e.call(OPERATOR_NEW, &args![bytes]).u32();
    e.set(this, TESFile::m_pMasterPtrs, Ptr::new(table));
    let mut index = 0u32;
    let mut all_found = true;
    while master_node != 0 && list_item(e, master_node) != 0 {
        let mut file_node = files.addr();
        loop {
            if file_node == 0 || list_item(e, file_node) == 0 {
                break;
            }
            let file = list_item(e, file_node);
            let name = e.call(FILE_NAME, &args![file]).u32();
            let master = list_item(e, master_node);
            if e.call(STRICMP, &args![master, name]).i32() == 0 {
                e.mem
                    .set_u32(table.wrapping_add(index.wrapping_mul(4)), file);
                break;
            }
            file_node = list_next(e, file_node);
            if file_node == 0 || list_item(e, file_node) == 0 {
                e.mem.set_u32(table.wrapping_add(index.wrapping_mul(4)), 0);
                if report_missing != 0 {
                    let missing = list_item(e, master_node);
                    log(e, &args![MESSAGE_MISSING_MASTER, missing]);
                }
                all_found = false;
            }
        }
        index += 1;
        master_node = list_next(e, master_node);
    }
    all_found
}

// Translated from 00471a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetIndexFile` (Xbox PDB): file number `index` of the load
/// order seen from this file: 0 is the file itself, 1 and up are
/// `m_pMasterPtrs[index - 1]`; null past the master count or without a
/// table.
pub fn tes_file_get_index_file(e: &mut Engine, this: Ptr<TESFile>, index: u32) -> Ptr {
    if index > fn_00471850(e, this) {
        return Ptr::NULL;
    }
    if index == 0 {
        return this.cast();
    }
    let table = e.get(this, TESFile::m_pMasterPtrs);
    if table.is_null() {
        return Ptr::NULL;
    }
    Ptr::new(
        e.mem.u32(
            table
                .addr()
                .wrapping_add(index.wrapping_mul(4))
                .wrapping_sub(4),
        ),
    )
}

// Translated from 00471a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `position`th (1-based) master name of `listMasters`; 0 when the list
/// is empty or shorter.
pub fn fn_00471a60(e: &mut Engine, this: Ptr<TESFile>, position: u32) -> u32 {
    let mut node = masters_list(e, this);
    if list_item(e, node) == 0 {
        return 0;
    }
    let mut step = 1;
    while step < position {
        node = list_next(e, node);
        if node == 0 {
            return 0;
        }
        step += 1;
    }
    list_item(e, node)
}

// Translated from 00471ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The master count (`iMasterCount`), through [`fn_00471850`].
pub fn fn_00471ad0(e: &mut Engine, this: Ptr<TESFile>) -> u32 {
    fn_00471850(e, this)
}

// Translated from 00471af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when a master's file on disk no longer has the size recorded in
/// `listMastersData`: walks the two lists together, formats each master's
/// path, and compares the size found (or, for a missing file, the recorded
/// size against zero).
pub fn fn_00471af0(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    let mut name_node = masters_list(e, this);
    let mut data_node = fn_00470b90(e, this).addr();
    let mut changed = false;
    // The path buffer and the `WIN32_FIND_DATAA` of the game's stack frame.
    let path = e.mem.alloc(PATH_BUFFER_SIZE + FIND_DATA_SIZE);
    let found = Ptr::<Win32FindData>::new(path + PATH_BUFFER_SIZE);
    while name_node != 0
        && list_item(e, name_node) != 0
        && data_node != 0
        && list_item(e, data_node) != 0
    {
        let name = list_item(e, name_node);
        e.call(
            SNPRINTF,
            &args![path, PATH_BUFFER_SIZE, MASTER_PATH_FORMAT, name],
        );
        let recorded = list_item(e, data_node);
        let handle = e.call(FIND_FIRST_FILE, &args![path, found]).u32();
        let recorded_low = e.mem.u32(recorded);
        let recorded_high = e.mem.u32(recorded + 4);
        if handle == INVALID_HANDLE {
            if recorded_low != 0 || recorded_high != 0 {
                changed = true;
            }
        } else {
            e.call(FIND_CLOSE, &args![handle]);
            let low = e.get(found, Win32FindData::nFileSizeLow);
            let high = e.get(found, Win32FindData::nFileSizeHigh);
            if low != recorded_low || high != recorded_high {
                changed = true;
            }
        }
        name_node = list_next(e, name_node);
        data_node = list_next(e, data_node);
    }
    e.mem.free(path);
    changed
}

// Translated from 00471c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetMaster` (Xbox PDB): bit 0 of `m_Flags`.
pub fn tes_file_get_master(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    flag_is_set(e, this, FLAG_MASTER)
}

// Translated from 00471c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 0 of `m_Flags` (the master flag).
pub fn fn_00471c50(e: &mut Engine, this: Ptr<TESFile>, on: bool) {
    if on {
        modify_flags(e, this, FLAG_MASTER, 0);
    } else {
        modify_flags(e, this, 0, FLAG_MASTER);
    }
}

// Translated from 00471ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetOptimizedFile` (Xbox PDB): bit 4 (0x10) of `m_Flags`.
pub fn tes_file_get_optimized_file(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    flag_is_set(e, this, FLAG_OPTIMIZED)
}

// Translated from 00471cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 2 (0x04) of `m_Flags`.
pub fn fn_00471cd0(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    flag_is_set(e, this, FLAG_BIT_4)
}

// Translated from 00471d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets bit 2 (0x04) of `m_Flags`; clearing it also clears bit 3 (0x08, the
/// active flag).
pub fn fn_00471d00(e: &mut Engine, this: Ptr<TESFile>, on: bool) {
    if on {
        modify_flags(e, this, FLAG_BIT_4, 0);
    } else {
        modify_flags(e, this, 0, FLAG_BIT_4);
        modify_flags(e, this, 0, FLAG_ACTIVE);
    }
}

// Translated from 00471d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetActive` (Xbox PDB): bit 3 (0x08) of `m_Flags`.
pub fn tes_file_get_active(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    flag_is_set(e, this, FLAG_ACTIVE)
}

// Translated from 00471d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 3 (0x08) of `m_Flags`, the active flag.
pub fn fn_00471d90(e: &mut Engine, this: Ptr<TESFile>, on: bool) {
    if on {
        modify_flags(e, this, FLAG_ACTIVE, 0);
    } else {
        modify_flags(e, this, 0, FLAG_ACTIVE);
    }
}

// Translated from 00471de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 6 (0x40) of `m_Flags`.
pub fn fn_00471de0(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    flag_is_set(e, this, FLAG_BIT_40)
}

// Translated from 00471e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 6 (0x40) of `m_Flags`.
pub fn fn_00471e10(e: &mut Engine, this: Ptr<TESFile>, on: bool) {
    if on {
        modify_flags(e, this, FLAG_BIT_40, 0);
    } else {
        modify_flags(e, this, 0, FLAG_BIT_40);
    }
}

// Translated from 00471e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the first record header of the open file: seeks to the start,
/// reads 0x18 bytes into `m_currentform`, and, when its tag is not the one
/// at [`RECORD_TAG_01187014`], swaps the header's byte order and checks
/// again. Sets bit 6 of `m_Flags` from the always-true answer of
/// `008d0360` (inverted after the swap), and `bMustEndianConvert` when that
/// bit differs from the answer.
pub fn fn_00471e60(e: &mut Engine, this: Ptr<TESFile>) {
    let file = e.get(this, TESFile::m_pFile);
    if file.is_null() {
        return;
    }
    seek(e, file, 0);
    clear_current_form(e, this);
    let form = this.at(TESFile::m_currentform);
    if e.call(BSFILE_READ, &args![file, form, 0x18u32]).u32() == 0 {
        return;
    }
    let tag: u32 = e.global(RECORD_TAG_01187014);
    if e.get(form, Form::form) == tag {
        let stream = fn_00471f70(e);
        let answer = e.call(ALWAYS_TRUE, &args![stream]).u8();
        fn_00471e10(e, this, answer != 0);
    } else {
        form_endian(e, form);
        if e.get(form, Form::form) == tag {
            let stream = fn_00471f70(e);
            let answer = e.call(ALWAYS_TRUE, &args![stream]).u8();
            fn_00471e10(e, this, answer == 0);
        }
    }
    e.set(this, TESFile::bMustEndianConvert, false);
    let bit = fn_00471de0(e, this) as u8;
    let stream = fn_00471f70(e);
    let answer = e.call(ALWAYS_TRUE, &args![stream]).u8();
    if bit != answer {
        e.set(this, TESFile::bMustEndianConvert, true);
    }
}

// Translated from 00471f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The global at `011f6078` (cdecl, no arguments), the object the stream
/// code asks [`ALWAYS_TRUE`].
pub fn fn_00471f70(e: &mut Engine) -> u32 {
    e.global(STREAM_OBJECT)
}

// Translated from 00471f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::TESRewind` (Xbox PDB): seeks the open file to its start, zeroes
/// the three offsets and the current record header, and, when `load_first`
/// is set, reads the first record ([`tes_file_get_tes_form`]).
pub fn tes_file_tes_rewind(e: &mut Engine, this: Ptr<TESFile>, load_first: u8) {
    let file = e.get(this, TESFile::m_pFile);
    if !file.is_null() {
        seek(e, file, 0);
    }
    e.set(this, TESFile::m_fileoffset, 0);
    e.set(this, TESFile::m_formoffset, 0);
    e.set(this, TESFile::m_chunkoffset, 0);
    clear_current_form(e, this);
    if load_first != 0 {
        tes_file_get_tes_form(e, this);
    }
}

// Translated from 00472000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the cursor to the first chunk of the current record: zeroes the
/// offset inside the record, seeks past the record header unless the record
/// is compressed, clears the current chunk id and size, and reads the chunk
/// header ([`GET_TES_CHUNK`]).
pub fn fn_00472000(e: &mut Engine, this: Ptr<TESFile>) {
    e.set(this, TESFile::m_formoffset, 0);
    e.set(this, TESFile::m_chunkoffset, 0);
    let header_size = e.call(FORM_HEADER_SIZE, &args![this]).u32();
    let position = e
        .get(this, TESFile::m_fileoffset)
        .wrapping_add(header_size)
        .wrapping_add(e.get(this, TESFile::m_formoffset));
    if !fn_00472100(e, this) {
        let file = e.get(this, TESFile::m_pFile);
        seek(e, file, position);
        // The game reads the position back here and does not use it.
        fn_004720a0(e, file);
    }
    fn_004720d0(e, this);
    e.call(GET_TES_CHUNK, &args![this]);
}

// Translated from 004720a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The current position of a `BSFile`: `+0x38` unless it is -1, else
/// `+0x150`.
pub fn fn_004720a0(e: &mut Engine, file: Ptr) -> u32 {
    let first = e.mem.u32(file.addr() + 0x38);
    if first != u32::MAX {
        first
    } else {
        e.mem.u32(file.addr() + 0x150)
    }
}

// Translated from 004720d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the current chunk id and chunk size.
pub fn fn_004720d0(e: &mut Engine, this: Ptr<TESFile>) {
    e.set(this, TESFile::m_currentchunkID, 0);
    e.set(this, TESFile::m_actualChunkSize, 0);
}

// Translated from 00472100 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the current record is compressed: its tag is not the one at
/// [`RECORD_TAG_01187020`] and bit 18 (0x40000) of its flags is set.
pub fn fn_00472100(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    let form = this.at(TESFile::m_currentform);
    let tag: u32 = e.global(RECORD_TAG_01187020);
    e.get(form, Form::form) != tag && e.get(form, Form::flags) & RECORD_FLAG_COMPRESSED != 0
}

// Translated from 00472150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::NextForm` (Xbox PDB): frees the decompressed form buffer and
/// moves to the next record ([`fn_004721d0`]); with `skip_flagged` it keeps
/// moving while the record (not one of the two header-only tags) has bit 12
/// (0x1000) of its flags set. Answers whether a record was read.
pub fn tes_file_next_form(e: &mut Engine, this: Ptr<TESFile>, skip_flagged: u8) -> bool {
    e.call(FREE_DECOMPRESSED_FORM, &args![this]);
    let mut read = fn_004721d0(e, this);
    let form = this.at(TESFile::m_currentform);
    while read
        && skip_flagged != 0
        && e.get(form, Form::form) != e.global::<u32>(RECORD_TAG_01187020)
        && e.get(form, Form::form) != e.global::<u32>(RECORD_TAG_011873C8)
        && e.get(form, Form::flags) & RECORD_FLAG_SKIPPED_BY_NEXT_FORM != 0
    {
        read = fn_004721d0(e, this);
    }
    read
}

// Translated from 004721d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances `m_fileoffset` past the current record (its header only for the
/// two header-only tags, else the header and `length`), seeks the file
/// there and reads the next record header. Answers false at the end of the
/// file (the header is zeroed), when the seek fails (reported with the
/// system's message) or when no record header could be read (reported).
pub fn fn_004721d0(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    let form = this.at(TESFile::m_currentform);
    let tag = e.get(form, Form::form);
    let header_size = e.call(FORM_HEADER_SIZE, &args![this]).u32();
    let fileoffset = e.get(this, TESFile::m_fileoffset);
    if tag == e.global::<u32>(RECORD_TAG_01187020) || tag == e.global::<u32>(RECORD_TAG_011873C8) {
        e.set(
            this,
            TESFile::m_fileoffset,
            header_size.wrapping_add(fileoffset),
        );
    } else {
        let length = e.get(form, Form::length);
        e.set(
            this,
            TESFile::m_fileoffset,
            header_size.wrapping_add(length).wrapping_add(fileoffset),
        );
    }
    if e.get(this, TESFile::m_fileoffset) >= e.get(this, TESFile::m_filesize) {
        clear_current_form(e, this);
        return false;
    }
    let target = e.get(this, TESFile::m_fileoffset);
    let file = e.get(this, TESFile::m_pFile);
    if fn_00472380(e, file) != target {
        seek(e, file, target);
        let position = fn_00472380(e, file);
        e.set(this, TESFile::m_fileoffset, position);
    }
    if e.get(this, TESFile::m_fileoffset) == u32::MAX {
        report_set_file_pointer_failure(e, MESSAGE_NEXT_FORM_SEEK_FAILED);
        return false;
    }
    e.set(this, TESFile::m_formoffset, 0);
    e.set(this, TESFile::m_chunkoffset, 0);
    clear_current_form(e, this);
    tes_file_get_tes_form_no_ret(e, this);
    if e.get(form, Form::form) == 0 {
        let name: Ptr = this.at(TESFile::m_Filename).cast();
        let offset = e.get(this, TESFile::m_fileoffset);
        log(e, &args![MESSAGE_NEXT_FORM_BAD_FORM, name, offset]);
        return false;
    }
    true
}

// Translated from 00472380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFile +0x150`: the file's current position.
pub fn fn_00472380(e: &mut Engine, file: Ptr) -> u32 {
    e.mem.u32(file.addr() + 0x150)
}

// Translated from 004723a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::SetOffset` (Xbox PDB): frees the decompressed form buffer,
/// seeks to `offset` and reads the record header there. Answers false when
/// the offset is at or past the end of the file (the header is zeroed) or
/// the seek fails (reported with the system's message).
pub fn tes_file_set_offset(e: &mut Engine, this: Ptr<TESFile>, offset: u32) -> bool {
    e.call(FREE_DECOMPRESSED_FORM, &args![this]);
    e.set(this, TESFile::m_fileoffset, offset);
    if e.get(this, TESFile::m_fileoffset) >= e.get(this, TESFile::m_filesize) {
        clear_current_form(e, this);
        return false;
    }
    let target = e.get(this, TESFile::m_fileoffset);
    let file = e.get(this, TESFile::m_pFile);
    seek(e, file, target);
    let position = fn_004720a0(e, file);
    e.set(this, TESFile::m_fileoffset, position);
    if position == u32::MAX {
        report_set_file_pointer_failure(e, MESSAGE_SET_OFFSET_SEEK_FAILED);
        return false;
    }
    e.set(this, TESFile::m_formoffset, 0);
    e.set(this, TESFile::m_chunkoffset, 0);
    clear_current_form(e, this);
    tes_file_get_tes_form(e, this);
    true
}

// Translated from 004724c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The file offset of the cursor inside the current record:
/// `m_fileoffset + 0x18 + m_formoffset`.
pub fn fn_004724c0(e: &mut Engine, this: Ptr<TESFile>) -> u32 {
    let header_size = e.call(FORM_HEADER_SIZE, &args![this]).u32();
    e.get(this, TESFile::m_fileoffset)
        .wrapping_add(header_size)
        .wrapping_add(e.get(this, TESFile::m_formoffset))
}

// Translated from 004724f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::SetOffsetChunk` (Xbox PDB): moves the cursor to the absolute
/// file offset `offset`, which must lie inside the current record, and
/// reads the chunk header there. Refused (and reported) for a compressed
/// record; answers false for an offset outside the record or when the seek
/// fails.
pub fn tes_file_set_offset_chunk(e: &mut Engine, this: Ptr<TESFile>, offset: u32) -> bool {
    if fn_00472100(e, this) {
        log(e, &args![MESSAGE_SET_OFFSET_CHUNK_COMPRESSED]);
        return false;
    }
    let header_size = e.call(FORM_HEADER_SIZE, &args![this]).u32();
    let inside = offset
        .wrapping_sub(e.get(this, TESFile::m_fileoffset))
        .wrapping_sub(header_size);
    e.set(this, TESFile::m_formoffset, inside);
    let form = this.at(TESFile::m_currentform);
    if inside >= e.get(form, Form::length) {
        fn_004720d0(e, this);
        clear_current_form(e, this);
        return false;
    }
    let file = e.get(this, TESFile::m_pFile);
    seek(e, file, offset);
    if fn_004720a0(e, file) == u32::MAX {
        report_set_file_pointer_failure(e, MESSAGE_SET_OFFSET_CHUNK_SEEK_FAILED);
        return false;
    }
    e.set(this, TESFile::m_chunkoffset, 0);
    fn_004720d0(e, this);
    e.call(GET_TES_CHUNK, &args![this]);
    true
}

// Translated from 00472620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetTESFormNoRet` (Xbox PDB): when no record header is loaded,
/// reads one ([`READ_FORM_HEADER`]) and, if that worked, clears the chunk
/// and reads its header ([`READ_CHUNK_HEADER`]).
pub fn tes_file_get_tes_form_no_ret(e: &mut Engine, this: Ptr<TESFile>) {
    let form = this.at(TESFile::m_currentform);
    if e.get(form, Form::form) == 0 && e.call(READ_FORM_HEADER, &args![this]).bool() {
        fn_004720d0(e, this);
        e.call(READ_CHUNK_HEADER, &args![this]);
    }
}

// Translated from 00472660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetTESForm` (Xbox PDB): loads the current record header if
/// none is loaded (answering 0 when none can be read, else reading the
/// first chunk header too), then answers the form type of its tag
/// (`TESForm::GetFormTypeFromFormString`).
pub fn tes_file_get_tes_form(e: &mut Engine, this: Ptr<TESFile>) -> u32 {
    let form = this.at(TESFile::m_currentform);
    if e.get(form, Form::form) == 0 {
        if !e.call(READ_FORM_HEADER, &args![this]).bool() {
            return 0;
        }
        fn_004720d0(e, this);
        e.call(GET_TES_CHUNK, &args![this]);
    }
    let tag = e.get(form, Form::form);
    e.call(TYPE_FROM_FORM_TAG, &args![tag]).u32()
}

// Translated from 004726b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetTESChunk` (Xbox PDB): reads the chunk header if none is
/// loaded (answering 0 when it cannot be read), then answers the current
/// chunk id.
pub fn tes_file_get_tes_chunk(e: &mut Engine, this: Ptr<TESFile>) -> u32 {
    if e.get(this, TESFile::m_currentchunkID) == 0 && !tes_file_read_chunk_header(e, this) {
        return 0;
    }
    e.get(this, TESFile::m_currentchunkID)
}

// Translated from 004727f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetChunkData` (Xbox PDB): reads a four-byte chunk (the buffer
/// is a `u32`) and swaps its byte order when the file must be converted.
/// The answer is that of [`tes_file_get_chunk_data_ov3`]; the decompiler
/// shows it as `void` but the code returns its `AL`.
pub fn tes_file_get_chunk_data(e: &mut Engine, this: Ptr<TESFile>, buffer: Ptr) -> bool {
    let read = tes_file_get_chunk_data_ov3(e, this, buffer, 4);
    if e.call(MUST_ENDIAN_CONVERT, &args![this]).bool() {
        e.call(SWAP_U32, &args![buffer, 0u32]);
    }
    read
}

// Translated from 00472840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetChunkData_ov2` (Xbox PDB): reads a two-byte chunk (the
/// buffer is a `u16`) and swaps its byte order when the file must be
/// converted.
pub fn tes_file_get_chunk_data_ov2(e: &mut Engine, this: Ptr<TESFile>, buffer: Ptr) -> bool {
    let read = tes_file_get_chunk_data_ov3(e, this, buffer, 2);
    if e.call(MUST_ENDIAN_CONVERT, &args![this]).bool() {
        e.call(SWAP_U16, &args![buffer, 0u32]);
    }
    read
}

// Translated from 00472890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetChunkData_ov3` (Xbox PDB): copies the current chunk's data
/// into `buffer`. A chunk without data answers true at once. If the cursor
/// into the chunk (`m_chunkoffset`) is not at its start, an uncompressed
/// record is first seeked to the chunk's data; then the data is read from
/// the file (or copied from the decompressed form). With a `max_size`
/// below the chunk's size only `max_size - 1` bytes are read, the last byte
/// of the buffer is zeroed and the truncation is reported. Answers false
/// when a file read comes up short (reported with the system's message);
/// the decompiler shows the function as `void`.
// Translation incomplete: 00472890 - the /GS stack cookie check (00ec408c) is not translated.
pub fn tes_file_get_chunk_data_ov3(
    e: &mut Engine,
    this: Ptr<TESFile>,
    buffer: Ptr,
    max_size: u32,
) -> bool {
    let chunk_size = e.get(this, TESFile::m_actualChunkSize);
    if chunk_size == 0 {
        return true;
    }
    let compressed = fn_00472100(e, this);
    if e.get(this, TESFile::m_chunkoffset) != 0 {
        if !compressed {
            let header_size = e.call(FORM_HEADER_SIZE, &args![this]).u32();
            let position = e
                .get(this, TESFile::m_fileoffset)
                .wrapping_add(header_size)
                .wrapping_add(e.get(this, TESFile::m_formoffset))
                .wrapping_add(6);
            let file = e.get(this, TESFile::m_pFile);
            seek(e, file, position);
        }
        e.set(this, TESFile::m_chunkoffset, 0);
    }
    if max_size != 0 && chunk_size > max_size {
        let wanted = max_size - 1;
        e.mem
            .set_u8(buffer.addr().wrapping_add(max_size).wrapping_sub(1), 0);
        if !compressed {
            let file = e.get(this, TESFile::m_pFile);
            let read = e.call(BSFILE_READ, &args![file, buffer, wanted]).u32();
            e.set(this, TESFile::m_chunkoffset, read);
            if read != wanted {
                report_set_file_pointer_failure(e, MESSAGE_CHUNK_FIRST_READ_FAILED);
                return false;
            }
        } else {
            let offset = e.get(this, TESFile::m_formoffset).wrapping_add(6);
            let source = fn_00473930(e, this).wrapping_add(offset);
            e.call(MEMCPY, &args![buffer, source, wanted]);
            e.set(this, TESFile::m_chunkoffset, wanted);
        }
        // The chunk id and the record tag as NUL-terminated text.
        let chunk_id = e.get(this, TESFile::m_currentchunkID);
        let form = this.at(TESFile::m_currentform);
        let form_tag = e.get(form, Form::form);
        let form_id = e.get(form, Form::iFormID);
        e.with_stack(0x10, |e, text| {
            e.mem.set_u32(text.addr(), chunk_id);
            e.mem.set_u8(text.addr() + 4, 0);
            e.mem.set_u32(text.addr() + 8, form_tag);
            e.mem.set_u8(text.addr() + 12, 0);
            log(
                e,
                &args![
                    MESSAGE_CHUNK_TOO_BIG,
                    chunk_size,
                    text,
                    text.addr() + 8,
                    form_id,
                    max_size,
                    buffer
                ],
            );
        });
        return true;
    }
    if !compressed {
        let file = e.get(this, TESFile::m_pFile);
        let read = e.call(BSFILE_READ, &args![file, buffer, chunk_size]).u32();
        e.set(this, TESFile::m_chunkoffset, read);
        if read != chunk_size {
            report_set_file_pointer_failure(e, MESSAGE_CHUNK_SECOND_READ_FAILED);
            return false;
        }
    } else {
        let offset = e.get(this, TESFile::m_formoffset).wrapping_add(6);
        let source = fn_00473930(e, this).wrapping_add(offset);
        e.call(MEMCPY, &args![buffer, source, chunk_size]);
        e.set(this, TESFile::m_chunkoffset, chunk_size);
    }
    true
}

// Translated from 00472bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::ReadFormHeader` (Xbox PDB): reads the 0x18-byte record header
/// at the file's position into `m_currentform` (answering false when the
/// file is not open, and zeroing the header when the read comes up short),
/// converts its byte order if needed and, for a record that is not one of
/// the two header-only tags, replaces the mod index (top byte) of its form
/// id by the compile index of the master it names (or this file's own),
/// then strips the index if the id check answers true.
pub fn tes_file_read_form_header(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    let file = e.get(this, TESFile::m_pFile);
    if file.is_null() {
        return false;
    }
    let form = this.at(TESFile::m_currentform);
    let read = e.call(BSFILE_READ, &args![file, form, 0x18u32]).u32();
    if read != 0x18 {
        e.call(MEMSET, &args![form, 0u32, 0x18u32]);
        return false;
    }
    if e.call(MUST_ENDIAN_CONVERT, &args![this]).bool() {
        form_endian(e, form);
    }
    let tag = e.get(form, Form::form);
    if tag != e.global::<u32>(RECORD_TAG_01187020) && tag != e.global::<u32>(RECORD_TAG_011873C8) {
        let mut master = Ptr::NULL;
        if !e.get(this, TESFile::m_pMasterPtrs).is_null() {
            let index = (e.get(form, Form::iFormID) >> 24).wrapping_add(1);
            master = tes_file_get_index_file(e, this, index);
        }
        let mod_index = if !master.is_null() {
            fn_00473250(e, master.cast())
        } else {
            e.get(this, TESFile::cCompileIndex)
        };
        let form_id = e.get(form, Form::iFormID);
        e.set(
            form,
            Form::iFormID,
            (mod_index as u32) << 24 | form_id & 0x00ff_ffff,
        );
        let stripped = e.get(form, Form::iFormID) & 0x00ff_ffff;
        if e.call(FORM_ID_NEEDS_STRIPPING, &args![stripped]).bool() {
            e.set(form, Form::iFormID, stripped);
        }
    }
    true
}

// Translated from 00472d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::ReadChunkHeader` (Xbox PDB): reads the six-byte chunk header
/// (`u32` id, `u16` size) at the cursor, from the file or from the
/// decompressed form, into `m_currentchunkID` and `m_actualChunkSize`
/// (swapping the byte order if needed). Answers false, with the chunk
/// cleared, when it cannot be read. An `XXXX` chunk holds the real size of
/// the chunk after it: it is read (four bytes), the cursor moves past it and
/// the size is stored.
pub fn tes_file_read_chunk_header(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    e.with_stack(0x10, |e, header| {
        if !fn_00472100(e, this) {
            let file = e.get(this, TESFile::m_pFile);
            let read = e.call(BSFILE_READ, &args![file, header, 6u32]).u32();
            if read != 6 {
                fn_004720d0(e, this);
                return false;
            }
        } else {
            let offset = e.get(this, TESFile::m_formoffset);
            let buffer = fn_00473930(e, this);
            if buffer == 0 || offset >= e.call(DECOMPRESSED_FORM_SIZE, &args![this]).u32() {
                fn_004720d0(e, this);
                return false;
            }
            e.call(MEMCPY, &args![header, buffer.wrapping_add(offset), 6u32]);
        }
        if e.call(MUST_ENDIAN_CONVERT, &args![this]).bool() {
            e.call(CHUNK_HEADER_ENDIAN, &args![header]);
        }
        let id = e.mem.u32(header.addr());
        e.set(this, TESFile::m_currentchunkID, id);
        let size = e.mem.u16(header.addr() + 4) as u32;
        e.set(this, TESFile::m_actualChunkSize, size);
        if id == CHUNK_ID_XXXX {
            let real_size = header.byte_add(8);
            e.mem.set_u32(real_size.addr(), 0);
            tes_file_get_chunk_data_ov3(e, this, real_size, 0);
            if e.call(MUST_ENDIAN_CONVERT, &args![this]).bool() {
                e.call(SWAP_U32, &args![real_size, 0u32]);
            }
            e.call(SKIP_TO_NEXT_CHUNK, &args![this]);
            let value = e.mem.u32(real_size.addr());
            e.set(this, TESFile::m_actualChunkSize, value);
        }
        true
    })
}

// Translated from 00472e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::StartForm` (Xbox PDB): begins writing `form`. Closes and opens
/// the groups it needs, fills `m_saveform` (record tag of the form's type,
/// length 0, the form's flags masked with `0x30032FE0`, its form id and the
/// form version), remembers the file offset it is written at
/// (`m_saveformoffset`) and writes the header there.
pub fn tes_file_start_form(e: &mut Engine, this: Ptr<TESFile>, form: Ptr) {
    tes_file_start_and_end_groups_for_form(e, this, form);
    let save = this.at(TESFile::m_saveform);
    let flags = e.call(FORM_FLAGS, &args![form]).u32();
    e.set(save, Form::flags, flags & SAVED_FORM_FLAGS_MASK);
    let type_index = e.call(FORM_TYPE_INDEX, &args![form]).u32();
    let tag = e.mem.u32(FORM_TAG_TABLE + type_index.wrapping_mul(12));
    e.set(save, Form::form, tag);
    let form_id = e.call(FORM_ID, &args![form]).u32();
    e.set(save, Form::iFormID, form_id);
    let version = fn_00470bb0(e) as u16;
    e.set(save, Form::sFormVersion, version);
    e.set(save, Form::sVCVersion, 0);
    e.set(save, Form::iVersionControl, 0);
    e.set(save, Form::length, 0);
    let file = e.get(this, TESFile::m_pFile);
    seek_from_end(e, file, 0);
    let position = fn_004720a0(e, file);
    e.set(this, TESFile::m_saveformoffset, position);
    e.set(this, TESFile::m_savechunkoffset, 0);
    tes_file_easy_write(e, this, save.addr(), 0x18);
}

// Translated from 00472f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::LoadForm` (Xbox PDB): gives `form` the type of the current
/// record ([`tes_file_get_tes_form`]), sets its load flags (a word taken
/// from this file, plus `0x4000` when the form's flag test answers true),
/// gives it the record's form id through its virtual slot `+0x128`
/// (`TESForm::SetFormID(id, 1)`: the 1 pushed for the form id getter, which
/// returns without popping it, is that call's second argument) and records
/// this file as its file.
pub fn tes_file_load_form(e: &mut Engine, this: Ptr<TESFile>, form: Ptr) {
    let record_type = tes_file_get_tes_form(e, this);
    e.call(FORM_SET_TYPE, &args![form, record_type]);
    let flags = e.call(FORM_FILE_FLAGS, &args![this]).u32();
    if e.call(FORM_IS_FLAGGED, &args![form]).bool() {
        e.call(FORM_SET_LOAD_FLAGS, &args![form, flags | 0x4000]);
    } else {
        e.call(FORM_SET_LOAD_FLAGS, &args![form, flags]);
    }
    let id = e.call(RECORD_FORM_ID, &args![this]).u32();
    e.vcall(form.addr(), 0x128, &args![id, 1u32]);
    e.call(FORM_SET_FILE, &args![form, this]);
}

// Translated from 00472fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::AddTESForm` (Xbox PDB): writes the form that was saved into
/// the global buffer (`011c54cc`, size `011c54d0`) at the end of the file,
/// after closing and opening the groups `form` needs, and counts it in
/// `fileHeaderInfo.iFormCount` when a buffer was written. Answers the
/// result of [`tes_file_easy_write`] and releases the form's buffer.
pub fn tes_file_add_tes_form(e: &mut Engine, this: Ptr<TESFile>, form: Ptr) -> u32 {
    tes_file_start_and_end_groups_for_form(e, this, form);
    let file = e.get(this, TESFile::m_pFile);
    seek_from_end(e, file, 0);
    let size = fn_00473080(e, form);
    let data = fn_00473070(e, form);
    let result = tes_file_easy_write(e, this, data, size);
    if fn_00473070(e, form) != 0 && fn_00473080(e, form) != 0 {
        count_form(e, this);
    }
    e.call(FORM_FREE_FORM_BUFFER, &args![form]);
    result
}

/// `fileHeaderInfo.iFormCount += 1`.
fn count_form(e: &mut Engine, this: Ptr<TESFile>) {
    let header = this.at(TESFile::fileHeaderInfo);
    let count = e.get(header, FileHeader::iFormCount);
    e.set(header, FileHeader::iFormCount, count.wrapping_add(1));
}

// Translated from 00473070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the global at `011c54cc`: the buffer of the form being saved.
pub fn fn_00473070(e: &mut Engine, _unused_this: Ptr) -> u32 {
    e.global(SAVE_BUFFER)
}

// Translated from 00473080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the global at `011c54d0`: the size of the buffer of the form
/// being saved.
pub fn fn_00473080(e: &mut Engine, _unused_this: Ptr) -> u32 {
    e.global(SAVE_BUFFER_SIZE)
}

// Translated from 00473090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts one more form in `fileHeaderInfo.iFormCount`, copies the saved
/// chunk offset into the save header's length (`m_saveform.length`), seeks
/// back to the offset the header was written at (`m_saveformoffset`) and
/// writes the header again. Answers the write result.
pub fn fn_00473090(e: &mut Engine, this: Ptr<TESFile>) -> u32 {
    count_form(e, this);
    let save = this.at(TESFile::m_saveform);
    let length = e.get(this, TESFile::m_savechunkoffset);
    e.set(save, Form::length, length);
    let file = e.get(this, TESFile::m_pFile);
    let position = e.get(this, TESFile::m_saveformoffset);
    seek(e, file, position);
    tes_file_easy_write(e, this, save.addr(), 0x18)
}

// Translated from 00473110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::EasyWrite` (Xbox PDB): writes `size` bytes at `data` to the
/// file. Stores the result in `m_lastError` and answers it: 0 when `data`
/// is null or everything was written, else 10 (reported) after a short
/// write.
pub fn tes_file_easy_write(e: &mut Engine, this: Ptr<TESFile>, data: u32, size: u32) -> u32 {
    if data != 0 {
        let file = e.get(this, TESFile::m_pFile);
        if fn_00473180(e, file, data, size) < size {
            let error_slot = e.call(ERRNO, &args![]).u32();
            let _error = e.mem.u32(error_slot);
            e.set(this, TESFile::m_lastError, RETURN_CODE_WRITE_ERROR);
            log(e, &args![MESSAGE_WRITE_ERROR]);
            return e.get(this, TESFile::m_lastError);
        }
    }
    e.set(this, TESFile::m_lastError, 0);
    e.get(this, TESFile::m_lastError)
}

// Translated from 00473180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes `size` bytes at `data` to the `BSFile` (`0044e120` with a count of
/// 1 and the flag 1); answers the number of bytes written.
pub fn fn_00473180(e: &mut Engine, this: Ptr, data: u32, size: u32) -> u32 {
    e.with_stack(4, |e, one| {
        e.mem.set_u32(one.addr(), 1);
        e.call(BSFILE_WRITE, &args![this, data, size, one, 1u32])
            .u32()
    })
}

// Translated from 004731c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the top byte of `m_Flags`.
pub fn fn_004731c0(e: &mut Engine, this: Ptr<TESFile>, value: u32) {
    let flags = e.get(this, TESFile::m_Flags) & 0x00ff_ffff;
    e.set(this, TESFile::m_Flags, flags | value << 24);
}

// Translated from 00473210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the compile index: the top byte of `fileHeaderInfo.iNextFormID` and
/// `cCompileIndex`.
pub fn fn_00473210(e: &mut Engine, this: Ptr<TESFile>, index: u8) {
    let header = this.at(TESFile::fileHeaderInfo);
    let next = e.get(header, FileHeader::iNextFormID);
    e.set(
        header,
        FileHeader::iNextFormID,
        (index as u32) << 24 | next & 0x00ff_ffff,
    );
    e.set(this, TESFile::cCompileIndex, index);
}

// Translated from 00473250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `cCompileIndex`.
pub fn fn_00473250(e: &mut Engine, this: Ptr<TESFile>) -> u8 {
    e.get(this, TESFile::cCompileIndex)
}

// Translated from 00473270 (decompiled, FalloutNV.exe 1.4.0.525)
/// (cdecl, one argument) Destroys every item of the `BSSimpleList` whose
/// head node is `list`: while the list is not empty, deletes the head's
/// item (a non-null one through its scalar deleting destructor) and removes
/// the head.
pub fn fn_00473270(e: &mut Engine, list: Ptr) {
    while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let item = list_item(e, list.addr());
        if item != 0 {
            e.call(SCALAR_DELETE, &args![item, 1u32]);
        }
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
}

// Translated from 004732d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the item at the head of `m_grouplist`: the innermost open group
/// (a `FORM_GROUP`), or 0.
pub fn fn_004732d0(e: &mut Engine, this: Ptr<TESFile>) -> u32 {
    let list = this.at(TESFile::m_grouplist).addr();
    list_item(e, list)
}

// Translated from 004732f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the address of `m_grouplist`.
pub fn fn_004732f0(_e: &mut Engine, this: Ptr<TESFile>) -> Ptr<BSSimpleList> {
    this.at(TESFile::m_grouplist)
}

// Translated from 00473310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::StartGroup` (Xbox PDB): pushes a copy of the group header
/// `group_header` on `m_grouplist` ([`fn_00473440`]) and, when the file is
/// open, writes it at the end of the file, remembering the offset in the
/// entry (`iGroupOffset`) and counting the record in `iFormCount`.
pub fn tes_file_start_group(e: &mut Engine, this: Ptr<TESFile>, group_header: Ptr) {
    if group_header.is_null() {
        return;
    }
    fn_00473440(e, this, group_header);
    let file = e.get(this, TESFile::m_pFile);
    if !file.is_null() {
        let group = Ptr::<FormGroup>::new(fn_004732d0(e, this));
        seek_from_end(e, file, 0);
        let position = fn_004720a0(e, file);
        e.set(group, FormGroup::iGroupOffset, position);
        tes_file_easy_write(e, this, group.addr(), 0x18);
        count_form(e, this);
    }
}

// Translated from 004733a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::EndGroup` (Xbox PDB): closes the innermost group. When the
/// file is open, stores the group's length (the file's end minus the offset
/// it started at) in its header and rewrites the header there. Then removes
/// the entry ([`fn_00473490`]).
pub fn tes_file_end_group(e: &mut Engine, this: Ptr<TESFile>) {
    let group = Ptr::<FormGroup>::new(fn_004732d0(e, this));
    if group.is_null() {
        return;
    }
    let file = e.get(this, TESFile::m_pFile);
    if !file.is_null() {
        seek_from_end(e, file, 0);
        let end = fn_004720a0(e, file);
        let start = e.get(group, FormGroup::iGroupOffset);
        e.set(
            group.at(FormGroup::GroupData),
            Form::length,
            end.wrapping_sub(start),
        );
        seek(e, file, start);
        tes_file_easy_write(e, this, group.addr(), 0x18);
    }
    fn_00473490(e, this);
}

// Translated from 00473440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a 0x1C-byte `FORM_GROUP`, adds it at the head of `m_grouplist`
/// and copies the 0x18-byte header `header` into it. Does nothing for a null
/// header.
pub fn fn_00473440(e: &mut Engine, this: Ptr<TESFile>, header: Ptr) {
    if header.is_null() {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![GROUP_ENTRY_SIZE]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), block);
        e.call(LIST_ADD_HEAD, &args![this.at(TESFile::m_grouplist), slot]);
        let entry = e.mem.u32(slot.addr());
        e.call(MEMCPY, &args![entry, header, 0x18u32]);
    });
}

// Translated from 00473490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the innermost group entry: drops the head node of `m_grouplist`
/// and deletes the entry. Does nothing when there is none.
pub fn fn_00473490(e: &mut Engine, this: Ptr<TESFile>) {
    let group = fn_004732d0(e, this);
    if group != 0 {
        e.call(LIST_REMOVE_HEAD, &args![this.at(TESFile::m_grouplist)]);
        delete(e, group);
    }
}

// Translated from 004734d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::FindForm` (Xbox PDB): positions the cursor on the record of
/// `form`. Answers false for a null form, a file with no groups, a closed
/// file or one that is not ready. If the form's virtual slot `+0x7C` (given
/// this file) answers true, the form is taken as found. Otherwise the file
/// is rewound and its records are scanned: group records the form's slot
/// `+0x110` (`header, 1, 0`) accepts are entered, the others are skipped
/// ([`fn_00473660`]); a record with the tag of the form's type (slot
/// `+0x8C` indexes the tag table) and the form's id ends the scan with true.
pub fn tes_file_find_form(e: &mut Engine, this: Ptr<TESFile>, form: Ptr) -> bool {
    if form.is_null() || !fn_00473640(e, this) {
        return false;
    }
    let file = e.get(this, TESFile::m_pFile);
    if file.is_null() || !e.call(FILE_IS_READY, &args![file]).bool() {
        return false;
    }
    if e.vcall(form.addr(), 0x7c, &args![this]).bool() {
        return true;
    }
    tes_file_tes_rewind(e, this, 1);
    let current = this.at(TESFile::m_currentform);
    while e.get(current, Form::form) == e.global::<u32>(RECORD_TAG_01187014) {
        tes_file_next_form(e, this, 1);
    }
    let type_index = e.vcall(form.addr(), 0x8c, &args![]).u32();
    let wanted_tag = e.mem.u32(FORM_TAG_TABLE + type_index.wrapping_mul(12));
    let wanted_id = e.call(FORM_ID, &args![form]).u32();
    loop {
        let tag = e.get(current, Form::form);
        if tag == 0 {
            return false;
        }
        if tag == e.global::<u32>(RECORD_TAG_01187020) {
            if e.vcall(form.addr(), 0x110, &args![current, 1u32, 0u32])
                .bool()
            {
                tes_file_next_form(e, this, 1);
            } else {
                fn_00473660(e, this);
            }
        } else if tag == wanted_tag && e.get(current, Form::iFormID) == wanted_id {
            return true;
        } else {
            tes_file_next_form(e, this, 1);
        }
    }
}

// Translated from 00473640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `bHasGroups`.
pub fn fn_00473640(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    e.get(this, TESFile::bHasGroups)
}

// Translated from 00473660 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the current record is a group header (the tag at
/// [`RECORD_TAG_01187020`]), clears its tag, takes the header size off its
/// length and moves on to the next record ([`fn_004721d0`]), answering
/// whether one was read; else false.
pub fn fn_00473660(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    let form = this.at(TESFile::m_currentform);
    if e.get(form, Form::form) != e.global::<u32>(RECORD_TAG_01187020) {
        return false;
    }
    e.set(form, Form::form, 0);
    let header_size = e.call(FORM_HEADER_SIZE, &args![this]).u32();
    let length = e.get(form, Form::length);
    e.set(form, Form::length, length.wrapping_sub(header_size));
    fn_004721d0(e, this)
}

// Translated from 004736c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::StartAndEndGroupsForForm` (Xbox PDB): makes the open groups fit
/// `form` before it is written (nothing for a null form or one whose type
/// index is 1). The form's virtual slot `+0x110` (`group, flag, 1`) tells
/// whether a group is acceptable for it: the open groups are ended up to the
/// outermost one it does not accept with flag 1; then groups are started
/// (slot `+0x114` builds a group header for the current innermost group)
/// until the innermost one is accepted with flag 0. When the form cannot
/// build a group header with the group tag, the failure is reported.
pub fn tes_file_start_and_end_groups_for_form(e: &mut Engine, this: Ptr<TESFile>, form: Ptr) {
    if form.is_null() || e.call(FORM_TYPE_INDEX, &args![form]).u32() == 1 {
        return;
    }
    let mut group;
    let mut outermost_refused = 0u32;
    let mut node = fn_004732f0(e, this).addr();
    while node != 0 {
        group = list_item(e, node);
        if group != 0
            && !e
                .vcall(form.addr(), 0x110, &args![group, 1u32, 1u32])
                .bool()
        {
            outermost_refused = group;
        }
        node = list_next(e, node);
    }
    if outermost_refused != 0 {
        group = fn_004732d0(e, this);
        while group != 0 {
            tes_file_end_group(e, this);
            if group == outermost_refused {
                group = 0;
            } else {
                group = fn_004732d0(e, this);
            }
        }
    }
    group = fn_004732d0(e, this);
    loop {
        if group != 0
            && e.vcall(form.addr(), 0x110, &args![group, 0u32, 1u32])
                .bool()
        {
            return;
        }
        let made = e.with_stack(0x18, |e, header| {
            e.vcall(form.addr(), 0x114, &args![header, group]);
            if e.get(header.cast::<Form>(), Form::form) == e.global::<u32>(RECORD_TAG_01187020) {
                tes_file_start_group(e, this, header);
                true
            } else {
                false
            }
        });
        if !made {
            let form_id = e.call(FORM_ID, &args![form]).u32();
            let name = e.vcall(form.addr(), 0x130, &args![]).u32();
            let type_name = e.call(FORM_TYPE_NAME, &args![form]).u32();
            log(
                e,
                &args![MESSAGE_CREATE_GROUP_FAILED, type_name, name, form_id],
            );
            return;
        }
        group = fn_004732d0(e, this);
    }
}

// Translated from 00473830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::CloseAllOpenGroups` (Xbox PDB): while a group is open, ends it
/// properly ([`tes_file_end_group`]) if the data handler's byte at `+0x61A`
/// is set, else just removes its entry ([`fn_00473490`]).
pub fn tes_file_close_all_open_groups(e: &mut Engine, this: Ptr<TESFile>) {
    while fn_004732d0(e, this) != 0 {
        let handler: u32 = e.global(DATA_HANDLER);
        if e.call(HANDLER_GROUPS_FLAG, &args![handler]).bool() {
            tes_file_end_group(e, this);
        } else {
            fn_00473490(e, this);
        }
    }
}

// Translated from 00473880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers false.
pub fn fn_00473880(_e: &mut Engine, _unused_this: Ptr<TESFile>) -> bool {
    false
}

// Translated from 004738a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::IsFileVersionTooHigh` (Xbox PDB): true when the header version
/// is above 1.34 (the `double` at `0101a208`), which is reported with
/// warnings disabled around the report ("File %s is a higher version than
/// this EXE can load.").
pub fn tes_file_is_file_version_too_high(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    let version = e.call(HEADER_VERSION, &args![this]).f32() as f64;
    let highest: f64 = e.global(HIGHEST_HEADER_VERSION);
    if version > highest {
        e.call(WARNING_COUNT, &args![0u32]);
        let name = e.call(FILE_NAME, &args![this]).u32();
        e.with_stack(PATH_BUFFER_SIZE, |e, text| {
            e.call(SPRINTF, &args![text, MESSAGE_VERSION_TOO_HIGH, name]);
            log(e, &args![text]);
        });
        e.call(WARNING_COUNT, &args![1u32]);
        true
    } else {
        false
    }
}

// Translated from 00473930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the decompressed form buffer (`pDecompressedFormBuffer`),
/// decompressing the current form first when there is none.
pub fn fn_00473930(e: &mut Engine, this: Ptr<TESFile>) -> u32 {
    if e.get(this, TESFile::pDecompressedFormBuffer).is_null() {
        e.call(DECOMPRESS_CURRENT_FORM, &args![this]);
    }
    e.get(this, TESFile::pDecompressedFormBuffer).addr()
}

// Translated from 00473960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the decompressed form buffer, if any, and clears its pointer and
/// size.
pub fn fn_00473960(e: &mut Engine, this: Ptr<TESFile>) {
    let buffer = e.get(this, TESFile::pDecompressedFormBuffer);
    if !buffer.is_null() {
        e.call(OPERATOR_DELETE, &args![buffer]);
        e.set(this, TESFile::pDecompressedFormBuffer, Ptr::NULL);
        e.set(this, TESFile::iDecompressedFormBufferSize, 0);
    }
}

// Translated from 004739b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetThreadSafeFile` (Xbox PDB): the file to use on the current
/// thread. Goes up `pThreadSafeParent` to the root file ([`fn_00473c70`]);
/// if the current thread is the procedure owner's, answers that root, else
/// the root's per-thread copy ([`tes_file_get_thread_safe_file_for_thread`]).
pub fn tes_file_get_thread_safe_file(e: &mut Engine, this: Ptr<TESFile>) -> Ptr<TESFile> {
    let mut root = this;
    while !fn_00473c70(e, root).is_null() {
        root = fn_00473c70(e, root);
    }
    if !root.is_null() {
        let thread = e.call(CURRENT_THREAD, &args![]).u32();
        let owner: u32 = e.global(PROCEDURE_OWNER);
        if thread == e.call(OWNER_THREAD, &args![owner]).u32() {
            return root;
        }
    }
    let thread = e.call(CURRENT_THREAD, &args![]).u32();
    tes_file_get_thread_safe_file_for_thread(e, root, thread)
}

// Translated from 00473a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the per-thread file copies: walks `pThreadSafeFileMap`
/// releasing and deleting each file, clears the map, deletes it through its
/// virtual destructor and zeroes the pointer. Does nothing without a map.
pub fn fn_00473a10(e: &mut Engine, this: Ptr<TESFile>) {
    let map = e.get(this, TESFile::pThreadSafeFileMap);
    if map.is_null() {
        return;
    }
    e.with_stack(12, |e, slots| {
        let position = slots;
        let key = slots.byte_add(4);
        let value = slots.byte_add(8);
        let first = e.call(THREAD_FILE_MAP_BEGIN, &args![map]).u32();
        e.mem.set_u32(position.addr(), first);
        while e.mem.u32(position.addr()) != 0 {
            e.mem.set_u32(key.addr(), 0);
            e.mem.set_u32(value.addr(), 0);
            e.call(THREAD_FILE_MAP_NEXT, &args![map, position, key, value]);
            let file = e.mem.u32(value.addr());
            e.call(RELEASE_FILE, &args![file]);
            if file != 0 {
                e.call(SCALAR_DELETE, &args![file, 1u32]);
            }
        }
    });
    e.call(THREAD_FILE_MAP_CLEAR, &args![map]);
    let map = e.get(this, TESFile::pThreadSafeFileMap);
    if !map.is_null() {
        e.vcall(map.addr(), 0, &args![1u32]);
    }
    e.set(this, TESFile::pThreadSafeFileMap, Ptr::NULL);
}

// Translated from 00473ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetThreadSafeFileForThread` (Xbox PDB): this file's own copy
/// for thread `thread`. Looks it up in `pThreadSafeFileMap`; if there is
/// none, builds one: a new `TESFile` with this file's directory and name,
/// the same master flag and compile index, an index table built against the
/// data handler's file table, this file's root as parent, opened, and
/// stored in the map (which is created first when needed). The compiler's
/// exception-unwinding frame is not translated.
pub fn tes_file_get_thread_safe_file_for_thread(
    e: &mut Engine,
    this: Ptr<TESFile>,
    thread: u32,
) -> Ptr<TESFile> {
    let mut copy = 0u32;
    let map = e.get(this, TESFile::pThreadSafeFileMap);
    if !map.is_null() {
        copy = e.with_stack(4, |e, slot| {
            e.call(THREAD_FILE_MAP_LOOKUP, &args![map, thread, slot]);
            e.mem.u32(slot.addr())
        });
    }
    if copy == 0 {
        let block = e.call(OPERATOR_NEW, &args![TES_FILE_SIZE]).u32();
        let created = if block != 0 {
            let name = e.call(FILE_NAME, &args![this]).u32();
            let directory = e.call(FILE_DIRECTORY, &args![this]).u32();
            tes_file_tes_file(e, Ptr::new(block), Ptr::new(directory), Ptr::new(name), 0)
        } else {
            Ptr::NULL
        };
        let new_file = created;
        let master = tes_file_get_master(e, this);
        fn_00471c50(e, new_file, master);
        let compile_index = fn_00473250(e, this);
        fn_00473210(e, new_file, compile_index);
        let handler: u32 = e.global(DATA_HANDLER);
        let table = e.call(HANDLER_FILE_TABLE, &args![handler]).u32();
        tes_file_gen_index_table(e, new_file, Ptr::new(table), 0);
        fn_00473cb0(e, new_file, this);
        tes_file_open_tes(e, new_file, 0, 0);
        if e.get(this, TESFile::pThreadSafeFileMap).is_null() {
            let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
            let table = if block != 0 {
                e.call(THREAD_FILE_MAP_CONSTRUCT, &args![block, 0x25u32])
                    .u32()
            } else {
                0
            };
            e.set(this, TESFile::pThreadSafeFileMap, Ptr::new(table));
        }
        let map = e.get(this, TESFile::pThreadSafeFileMap);
        e.call(THREAD_FILE_MAP_SET_AT, &args![map, thread, new_file]);
        e.call(RELEASE_FILE, &args![new_file]);
        copy = new_file.addr();
    }
    Ptr::new(copy)
}

// Translated from 00473c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Goes up `pThreadSafeParent` from `this`'s parent to the root (the file
/// whose parent is null); answers 0 when `this` has no parent.
pub fn fn_00473c70(e: &mut Engine, this: Ptr<TESFile>) -> Ptr<TESFile> {
    let mut current = e.get(this, TESFile::pThreadSafeParent);
    while !current.is_null() {
        let next = e.get(current.cast::<TESFile>(), TESFile::pThreadSafeParent);
        if next.is_null() {
            break;
        }
        current = next;
    }
    current.cast()
}

// Translated from 00473cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `pThreadSafeParent` to the root of `other` (`other` itself when it
/// has no parent).
pub fn fn_00473cb0(e: &mut Engine, this: Ptr<TESFile>, other: Ptr<TESFile>) {
    let mut root = other;
    loop {
        let next = e.get(root, TESFile::pThreadSafeParent);
        if next.is_null() {
            break;
        }
        root = next.cast();
    }
    e.set(this, TESFile::pThreadSafeParent, root.cast());
}

// Translated from 00473ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets `m_uiBufferAllocSize` to its default (the global at `01186740`);
/// the word it is passed is not read.
pub fn fn_00473ce0(e: &mut Engine, this: Ptr<TESFile>, _unused_1: u32) {
    let size: u32 = e.global(DEFAULT_BUFFER_SIZE);
    e.set(this, TESFile::m_uiBufferAllocSize, size);
}

// Translated from 00473d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers true; takes two words it does not read.
pub fn fn_00473d00(_e: &mut Engine, _unused_this: Ptr, _unused_1: u32, _unused_2: u32) -> bool {
    true
}

// Translated from 00473d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 5 (0x20) of `m_Flags`: the file is registered in the
/// global file map.
pub fn fn_00473d20(e: &mut Engine, this: Ptr<TESFile>, on: bool) {
    if on {
        modify_flags(e, this, FLAG_REGISTERED, 0);
    } else {
        modify_flags(e, this, 0, FLAG_REGISTERED);
    }
}

// Translated from 00473d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 5 (0x20) of `m_Flags`: the file is registered in the global file map.
pub fn fn_00473d70(e: &mut Engine, this: Ptr<TESFile>) -> bool {
    flag_is_set(e, this, FLAG_REGISTERED)
}

// Translated from 00473d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unregisters the file from the global file map. Does nothing unless bit 5
/// of `m_Flags` is set; clears it and counts the file out. When it was the
/// last registered file the map is cleared, deleted through its virtual
/// destructor and forgotten; otherwise every key whose value is this file is
/// collected into a temporary key array and removed from the map. The
/// compiler's exception-unwinding frame is not translated.
pub fn fn_00473d90(e: &mut Engine, this: Ptr<TESFile>) {
    if !fn_00473d70(e, this) {
        return;
    }
    fn_00473d20(e, this, false);
    let users = e.global::<u32>(GLOBAL_FILE_MAP_USERS).wrapping_sub(1);
    e.set_global(GLOBAL_FILE_MAP_USERS, users);
    if users == 0 {
        let map: u32 = e.global(GLOBAL_FILE_MAP);
        e.call(THREAD_FILE_MAP_CLEAR, &args![map]);
        let map: u32 = e.global(GLOBAL_FILE_MAP);
        if map != 0 {
            e.vcall(map, 0, &args![1u32]);
        }
        e.set_global(GLOBAL_FILE_MAP, 0u32);
        return;
    }
    // The game's locals: the key array (0x10 bytes), the iteration position,
    // the key and the value.
    e.with_stack(0x20, |e, block| {
        let array = block.cast::<NiTArray>();
        let position = block.byte_add(0x10);
        let key = block.byte_add(0x14);
        let value = block.byte_add(0x18);
        fn_00474790(e, array, 100, 100);
        let mut count = 0u32;
        let map: u32 = e.global(GLOBAL_FILE_MAP);
        let first = e.call(THREAD_FILE_MAP_BEGIN, &args![map]).u32();
        e.mem.set_u32(position.addr(), first);
        while e.mem.u32(position.addr()) != 0 {
            e.mem.set_u32(value.addr(), 0);
            e.mem.set_u32(key.addr(), 0);
            let map: u32 = e.global(GLOBAL_FILE_MAP);
            e.call(THREAD_FILE_MAP_NEXT, &args![map, position, key, value]);
            if e.mem.u32(value.addr()) == this.addr() {
                e.call(KEY_ARRAY_SET_AT_GROW, &args![array, count, key]);
                count += 1;
            }
        }
        for index in 0..count {
            let slot = e.call(KEY_ARRAY_ELEMENT, &args![array, index]).u32();
            let removed_key = e.mem.u32(slot);
            let map: u32 = e.global(GLOBAL_FILE_MAP);
            e.call(FILE_MAP_REMOVE_AT, &args![map, removed_key]);
        }
        fn_00473f00(e, array);
    });
}

// Translated from 00473f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the key array of [`fn_00473d90`] (a call of the base
/// destructor [`fn_00474760`]).
pub fn fn_00473f00(e: &mut Engine, this: Ptr<NiTArray>) {
    fn_00474760(e, this);
}

// Translated from 00473f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers `file` in the global file map under `key`: creates the map
/// (0x3E9 buckets) first when there is none, stores the pair, and, when the
/// file was not yet marked as registered, marks it and counts it in.
/// cdecl; the compiler's exception-unwinding frame is not translated.
pub fn fn_00473f20(e: &mut Engine, key: u32, file: Ptr<TESFile>) {
    if e.global::<u32>(GLOBAL_FILE_MAP) == 0 {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let map = if block != 0 {
            fn_004744d0(e, Ptr::new(block), GLOBAL_FILE_MAP_BUCKETS).addr()
        } else {
            0
        };
        e.set_global(GLOBAL_FILE_MAP, map);
    }
    let map: u32 = e.global(GLOBAL_FILE_MAP);
    e.call(THREAD_FILE_MAP_SET_AT, &args![map, key, file]);
    if !fn_00473d70(e, file) {
        fn_00473d20(e, file, true);
        let users = e.global::<u32>(GLOBAL_FILE_MAP_USERS).wrapping_add(1);
        e.set_global(GLOBAL_FILE_MAP_USERS, users);
    }
}

// Translated from 00473ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True unless `key` is registered in the global file map to a file other
/// than the root of `file`'s parent chain (`file` itself when it has no
/// parent). True when there is no map, `key` is 0, or the key is unknown or
/// maps to 0. cdecl.
pub fn fn_00473ff0(e: &mut Engine, key: u32, file: Ptr<TESFile>) -> bool {
    let map: u32 = e.global(GLOBAL_FILE_MAP);
    if map == 0 || key == 0 {
        return true;
    }
    let (found, registered) = e.with_stack(4, |e, slot| {
        let found = e
            .call(THREAD_FILE_MAP_LOOKUP, &args![map, key, slot])
            .bool();
        (found, e.mem.u32(slot.addr()))
    });
    if !found || registered == 0 {
        return true;
    }
    let mut expected = fn_00473c70(e, file).addr();
    if expected == 0 {
        expected = file.addr();
    }
    registered == expected
}

// Translated from 00474060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::GetFileForTempID` (Xbox PDB): the file registered under `key`
/// in the global file map, or 0 (also without a map). cdecl.
pub fn tes_file_get_file_for_temp_id(e: &mut Engine, key: u32) -> u32 {
    let map: u32 = e.global(GLOBAL_FILE_MAP);
    if map == 0 {
        return 0;
    }
    e.with_stack(4, |e, slot| {
        if e.call(THREAD_FILE_MAP_LOOKUP, &args![map, key, slot])
            .bool()
        {
            e.mem.u32(slot.addr())
        } else {
            0
        }
    })
}

// Translated from 004740a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFile::DecompressCurrentForm` (Xbox PDB): inflates the current
/// compressed record into a new `pDecompressedFormBuffer`. Works only on a
/// record with data whose tag is not the header-only tag at
/// [`RECORD_TAG_01187020`], that [`fn_00472100`] calls compressed and whose
/// tag names a form type: reads `length` bytes of the record into a scratch
/// buffer (a word with the decompressed size, converted from the file's byte
/// order when needed, then the zlib data), allocates the decompressed
/// buffer and inflates into it with zlib 1.2.1 (allocator hooks
/// [`fn_00474460`] and [`fn_00474480`]). Any failure (short read, zlib
/// error, stream that does not terminate) is logged, frees the decompressed
/// buffer and leaves the file without one. The scratch buffer is always
/// released. The memory-context scope guard (`00404eb0`) wraps all of it;
/// the compiler's exception-unwinding frame is not translated.
pub fn tes_file_decompress_current_form(e: &mut Engine, this: Ptr<TESFile>) {
    // The game's locals: the guard (4 bytes), the size word (4), the
    // `z_stream` (0x38) and the inflate state area.
    e.with_stack(8 + Z_STREAM_SIZE + INFLATE_STATE_SIZE, |e, block| {
        e.call(
            SCOPE_GUARD_OPEN,
            &args![
                block,
                SCOPE_GUARD_KIND,
                1u32,
                SOURCE_FILE_NAME,
                DECOMPRESS_LINE
            ],
        );
        decompress_current_form_inner(e, this, block);
        e.call(SCOPE_GUARD_CLOSE, &args![block]);
    });
}

/// The body of [`tes_file_decompress_current_form`], inside the scope guard.
fn decompress_current_form_inner(e: &mut Engine, this: Ptr<TESFile>, block: Ptr) {
    let size_word = block.byte_add(4);
    let stream = block.byte_add(8);
    let state_area = block.byte_add(8 + Z_STREAM_SIZE);
    let form = this.at(TESFile::m_currentform);
    if e.get(form, Form::length) == 0 {
        return;
    }
    let header_only_tag: u32 = e.global(RECORD_TAG_01187020);
    if e.get(form, Form::form) == header_only_tag || !fn_00472100(e, this) {
        return;
    }
    let tag = e.get(form, Form::form);
    let form_type = e.call(TYPE_FROM_FORM_TAG, &args![tag]).u32() as u8;
    if form_type == 0 {
        return;
    }
    let length = e.get(form, Form::length);
    let raw = e.call(OPERATOR_NEW, &args![length.wrapping_add(1)]).u32();
    if raw == 0 {
        return;
    }
    let file = e.get(this, TESFile::m_pFile);
    let read = e.call(BSFILE_READ, &args![file, raw, length]).u32();
    if read != e.get(form, Form::length) {
        log(e, &args![MESSAGE_COMPRESSED_READ_FAILED]);
        delete(e, raw);
        return;
    }
    // The first word of the data is the decompressed size.
    let first = e.mem.u32(raw);
    e.mem.set_u32(size_word.addr(), first);
    let compressed = raw + 4;
    if e.call(MUST_ENDIAN_CONVERT, &args![this]).bool() {
        e.call(SWAP_U32, &args![size_word, 0u32]);
    }
    let decompressed_size = e.mem.u32(size_word.addr());
    let terminator = raw + e.get(form, Form::length);
    e.mem.set_u8(terminator, 0);
    let output = e.call(OPERATOR_NEW, &args![decompressed_size]).u32();
    e.set(this, TESFile::pDecompressedFormBuffer, Ptr::new(output));
    e.set(
        this,
        TESFile::iDecompressedFormBufferSize,
        decompressed_size,
    );
    // `z_stream`: next_in +0, avail_in +4, next_out +0xC, avail_out +0x10,
    // state +0x1C, zalloc +0x20, zfree +0x24, opaque +0x28.
    e.mem.set_u32(stream.addr() + 0x20, ZALLOC_HOOK);
    e.mem.set_u32(stream.addr() + 0x24, ZFREE_HOOK);
    e.mem.set_u32(stream.addr() + 0x28, 0);
    e.mem.set_u32(stream.addr() + 0x04, 0);
    e.mem.set_u32(stream.addr(), 0);
    e.mem.set_u32(stream.addr() + 0x1C, state_area.addr());
    let init = e
        .call(INFLATE_INIT, &args![stream, ZLIB_VERSION, Z_STREAM_SIZE])
        .u32() as i32;
    if init != 0 {
        inflate_failed(e, this, raw, stream, MESSAGE_INFLATE_INIT_FAILED);
        return;
    }
    let packed_size = e.get(form, Form::length);
    e.mem
        .set_u32(stream.addr() + 0x04, packed_size.wrapping_sub(4));
    e.mem.set_u32(stream.addr(), compressed);
    e.mem.set_u32(stream.addr() + 0x10, decompressed_size);
    let output = e.get(this, TESFile::pDecompressedFormBuffer).addr();
    e.mem.set_u32(stream.addr() + 0x0C, output);
    let result = e.call(INFLATE, &args![stream, 0u32]).u32() as i32;
    if matches!(
        result,
        Z_STREAM_ERROR | Z_NEED_DICT | Z_DATA_ERROR | Z_MEM_ERROR
    ) {
        inflate_failed(e, this, raw, stream, MESSAGE_INFLATE_FAILED);
        return;
    }
    if result != Z_STREAM_END {
        inflate_failed(e, this, raw, stream, MESSAGE_INFLATE_NOT_TERMINATED);
        return;
    }
    e.call(INFLATE_END, &args![stream]);
    delete(e, raw);
}

/// The failure exit of the decompression: ends the zlib stream, logs
/// `message`, frees the decompressed buffer and the scratch buffer.
fn inflate_failed(e: &mut Engine, this: Ptr<TESFile>, raw: u32, stream: Ptr, message: u32) {
    e.call(INFLATE_END, &args![stream]);
    log(e, &args![message]);
    fn_00473960(e, this);
    delete(e, raw);
}

// Translated from 00474460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The zlib allocation hook of [`tes_file_decompress_current_form`]
/// (`zalloc(opaque, items, size)`): `items * size` bytes from the memory
/// manager. The opaque word is not read.
pub fn fn_00474460(e: &mut Engine, _unused_opaque: u32, items: u32, size: u32) -> u32 {
    let manager = e.call(MEMORY_MANAGER_GET, &args![]).u32();
    e.call(
        MEMORY_MANAGER_ALLOCATE,
        &args![manager, items.wrapping_mul(size)],
    )
    .u32()
}

// Translated from 00474480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The zlib release hook (`zfree(opaque, block)`): gives `block` back to
/// the memory manager. The opaque word is not read.
pub fn fn_00474480(e: &mut Engine, _unused_opaque: u32, block: u32) {
    let manager = e.call(MEMORY_MANAGER_GET, &args![]).u32();
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![manager, block]);
}

// Translated from 004744a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTPointerMap` of the per-thread file copies (the
/// thread-safe file map): base constructor with `hash_size` buckets, then
/// its own vtable. Answers `this`.
pub fn fn_004744a0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_00474560(e, this, hash_size);
    e.mem.set_u32(this.addr(), VTABLE_POINTER_MAP);
    this
}

// Translated from 004744d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTMap` of the global file map: base constructor with
/// `hash_size` buckets, then its own vtable. Answers `this`.
pub fn fn_004744d0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_00474660(e, this, hash_size);
    e.mem.set_u32(this.addr(), VTABLE_MAP);
    this
}

// Translated from 00474500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, TESFile *>::_scalar_deleting_destructor_`
/// (Xbox PDB): destroys the map ([`fn_004745d0`]) and, when bit 0 of
/// `flags` is set, frees it. Answers `this`.
pub fn ni_t_pointer_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_004745d0(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00474530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, TESFile *>::_scalar_deleting_destructor_` (Xbox
/// PDB): destroys the map ([`fn_004746d0`]) and, when bit 0 of `flags` is
/// set, frees it. Answers `this`.
pub fn ni_t_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_004746d0(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

/// The shared body of the two map base constructors: stores `vtable`, the
/// bucket count and a zero item count, and allocates the bucket array
/// (`hash_size` pointers) cleared to zero.
fn construct_map_base(e: &mut Engine, this: Ptr<NiTPointerMap>, vtable: u32, hash_size: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.set(this, NiTPointerMap::m_uiHashSize, hash_size);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let bytes = hash_size.wrapping_shl(2);
    let buckets = e.call(ALLOCATE_BLOCK, &args![bytes]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, buckets);
    let buckets = e.get(this, NiTPointerMap::m_ppkHashTable);
    let bytes = e.get(this, NiTPointerMap::m_uiHashSize).wrapping_shl(2);
    e.call(MEMSET, &args![buckets, 0u32, bytes]);
}

// Translated from 00474560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of [`fn_004744a0`]'s map
/// (`NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, TESFile *>`):
/// see [`construct_map_base`]. Answers `this`.
pub fn fn_00474560(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_base(e, this, VTABLE_POINTER_MAP_BASE, hash_size);
    this
}

// Translated from 004745d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of [`fn_004744a0`]'s map: sets its vtable, clears it
/// (`00438af0`), then runs the base destructor [`fn_00474630`]. (The
/// decompiler's `ctype<char>` name is a folded destructor.) The compiler's
/// exception-unwinding frame is not translated.
pub fn fn_004745d0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), VTABLE_POINTER_MAP);
    e.call(THREAD_FILE_MAP_CLEAR, &args![this]);
    fn_00474630(e, this);
}

// Translated from 00474630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base destructor of [`fn_004744a0`]'s map: sets the base vtable, clears
/// the map (`00438af0`) and frees the bucket array.
pub fn fn_00474630(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), VTABLE_POINTER_MAP_BASE);
    e.call(THREAD_FILE_MAP_CLEAR, &args![this]);
    let buckets = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(FREE_BLOCK, &args![buckets]);
}

// Translated from 00474660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of [`fn_004744d0`]'s map
/// (`NiTMapBase<DFALL<NiTMapItem<unsigned int, TESFile *>>, ...>`): see
/// [`construct_map_base`]. Answers `this`.
pub fn fn_00474660(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    construct_map_base(e, this, VTABLE_MAP_BASE, hash_size);
    this
}

// Translated from 004746d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of [`fn_004744d0`]'s map: sets its vtable, clears it
/// (`00438af0`), then runs the base destructor [`fn_00474730`]. The
/// compiler's exception-unwinding frame is not translated.
pub fn fn_004746d0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), VTABLE_MAP);
    e.call(THREAD_FILE_MAP_CLEAR, &args![this]);
    fn_00474730(e, this);
}

// Translated from 00474730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base destructor of [`fn_004744d0`]'s map: sets the base vtable, clears
/// the map (`00438af0`) and frees the bucket array.
pub fn fn_00474730(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), VTABLE_MAP_BASE);
    e.call(THREAD_FILE_MAP_CLEAR, &args![this]);
    let buckets = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(FREE_BLOCK, &args![buckets]);
}

// Translated from 00474760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the key array (`std::basic_streambuf` is the decompiler's
/// wrong name for a folded destructor): sets the base vtable and releases
/// the element block (`004ede70`).
pub fn fn_00474760(e: &mut Engine, this: Ptr<NiTArray>) {
    e.mem.set_u32(this.addr(), VTABLE_KEY_ARRAY_BASE);
    let elements = e.get(this, NiTArray::m_pBase);
    e.call(FREE_ARRAY_ELEMENTS, &args![elements]);
}

// Translated from 00474790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the key array of [`fn_00473d90`]: base constructor
/// [`fn_00474880`] with `(max_size, grow_by)`, then its own vtable. Answers
/// `this`.
pub fn fn_00474790(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u32,
    grow_by: u32,
) -> Ptr<NiTArray> {
    fn_00474880(e, this, max_size, grow_by);
    e.mem.set_u32(this.addr(), VTABLE_KEY_ARRAY);
    this
}

// Translated from 004747c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, TESFile *>::
/// _scalar_deleting_destructor_` (Xbox PDB): the base destructor
/// [`fn_00474630`] and, when bit 0 of `flags` is set, the release of the
/// object. Answers `this`.
pub fn ni_t_map_base_pointer_allocator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00474630(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 004747f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned int, TESFile *>>, unsigned int,
/// TESFile *>::_scalar_deleting_destructor_` (Xbox PDB): the base destructor
/// [`fn_00474730`] and, when bit 0 of `flags` is set, the release of the
/// object. Answers `this`.
pub fn ni_t_map_base_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00474730(e, this);
    if flags & 1 != 0 {
        delete(e, this.addr());
    }
    this
}

// Translated from 00474880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of the key array: stores the base vtable, `max_size`
/// and `grow_by` (both 16-bit), zero size and element count, and allocates
/// room for `max_size` pointers (none when `max_size` is 0). Answers `this`.
pub fn fn_00474880(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u32,
    grow_by: u32,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), VTABLE_KEY_ARRAY_BASE);
    e.set(this, NiTArray::m_usMaxSize, max_size as u16);
    e.set(this, NiTArray::m_usGrowBy, grow_by as u16);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    let max = e.get(this, NiTArray::m_usMaxSize);
    if max == 0 {
        e.set(this, NiTArray::m_pBase, 0);
    } else {
        let elements = e.call(ALLOCATE_POINTERS, &args![max as u32]).u32();
        e.set(this, NiTArray::m_pBase, elements);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00470650, form_endian(Ptr<Form>)),
        entry!(
            0x00470710,
            tes_file_tes_file(Ptr<TESFile>, Ptr, Ptr, u32) -> Ptr<TESFile>
        ),
        entry!(0x004709f0, fn_004709f0(Ptr<TESFile>)),
        entry!(0x00470b90, fn_00470b90(Ptr<TESFile>) -> Ptr<BSSimpleList>),
        entry!(0x00470bb0, fn_00470bb0() -> u32),
        entry!(0x00470bc0, fn_00470bc0(Ptr<TESFile>)),
        entry!(0x00470c70, tes_file_open_tes(Ptr<TESFile>, u32, u8) -> bool),
        entry!(
            0x00470ca0,
            tes_file_open_tes_ov2(Ptr<TESFile>, Ptr, Ptr, u32, u8) -> bool
        ),
        entry!(0x00471110, fn_00471110(Ptr<TESFile>, f32)),
        entry!(0x00471130, tes_file_close_tes(Ptr<TESFile>) -> bool),
        entry!(0x00471850, fn_00471850(Ptr<TESFile>) -> u32),
        entry!(
            0x00471870,
            tes_file_gen_index_table(Ptr<TESFile>, Ptr, u8) -> bool
        ),
        entry!(
            0x00471a10,
            tes_file_get_index_file(Ptr<TESFile>, u32) -> Ptr
        ),
        entry!(0x00471a60, fn_00471a60(Ptr<TESFile>, u32) -> u32),
        entry!(0x00471ad0, fn_00471ad0(Ptr<TESFile>) -> u32),
        entry!(0x00471af0, fn_00471af0(Ptr<TESFile>) -> bool),
        entry!(0x00471c20, tes_file_get_master(Ptr<TESFile>) -> bool),
        entry!(0x00471c50, fn_00471c50(Ptr<TESFile>, bool)),
        entry!(
            0x00471ca0,
            tes_file_get_optimized_file(Ptr<TESFile>) -> bool
        ),
        entry!(0x00471cd0, fn_00471cd0(Ptr<TESFile>) -> bool),
        entry!(0x00471d00, fn_00471d00(Ptr<TESFile>, bool)),
        entry!(0x00471d60, tes_file_get_active(Ptr<TESFile>) -> bool),
        entry!(0x00471d90, fn_00471d90(Ptr<TESFile>, bool)),
        entry!(0x00471de0, fn_00471de0(Ptr<TESFile>) -> bool),
        entry!(0x00471e10, fn_00471e10(Ptr<TESFile>, bool)),
        entry!(0x00471e60, fn_00471e60(Ptr<TESFile>)),
        entry!(0x00471f70, fn_00471f70() -> u32),
        entry!(0x00471f80, tes_file_tes_rewind(Ptr<TESFile>, u8)),
        entry!(0x00472000, fn_00472000(Ptr<TESFile>)),
        entry!(0x004720a0, fn_004720a0(Ptr) -> u32),
        entry!(0x004720d0, fn_004720d0(Ptr<TESFile>)),
        entry!(0x00472100, fn_00472100(Ptr<TESFile>) -> bool),
        entry!(0x00472150, tes_file_next_form(Ptr<TESFile>, u8) -> bool),
        entry!(0x004721d0, fn_004721d0(Ptr<TESFile>) -> bool),
        entry!(0x00472380, fn_00472380(Ptr) -> u32),
        entry!(0x004723a0, tes_file_set_offset(Ptr<TESFile>, u32) -> bool),
        entry!(0x004724c0, fn_004724c0(Ptr<TESFile>) -> u32),
        entry!(
            0x004724f0,
            tes_file_set_offset_chunk(Ptr<TESFile>, u32) -> bool
        ),
        entry!(0x00472620, tes_file_get_tes_form_no_ret(Ptr<TESFile>)),
        entry!(0x00472660, tes_file_get_tes_form(Ptr<TESFile>) -> u32),
        entry!(0x004726b0, tes_file_get_tes_chunk(Ptr<TESFile>) -> u32),
        entry!(
            0x004727f0,
            tes_file_get_chunk_data(Ptr<TESFile>, Ptr) -> bool
        ),
        entry!(
            0x00472840,
            tes_file_get_chunk_data_ov2(Ptr<TESFile>, Ptr) -> bool
        ),
        entry!(
            0x00472890,
            tes_file_get_chunk_data_ov3(Ptr<TESFile>, Ptr, u32) -> bool
        ),
        entry!(0x00472bc0, tes_file_read_form_header(Ptr<TESFile>) -> bool),
        entry!(0x00472d30, tes_file_read_chunk_header(Ptr<TESFile>) -> bool),
        entry!(0x00472e60, tes_file_start_form(Ptr<TESFile>, Ptr)),
        entry!(0x00472f60, tes_file_load_form(Ptr<TESFile>, Ptr)),
        entry!(0x00472fe0, tes_file_add_tes_form(Ptr<TESFile>, Ptr) -> u32),
        entry!(0x00473070, fn_00473070(Ptr) -> u32),
        entry!(0x00473080, fn_00473080(Ptr) -> u32),
        entry!(0x00473090, fn_00473090(Ptr<TESFile>) -> u32),
        entry!(
            0x00473110,
            tes_file_easy_write(Ptr<TESFile>, u32, u32) -> u32
        ),
        entry!(0x00473180, fn_00473180(Ptr, u32, u32) -> u32),
        entry!(0x004731c0, fn_004731c0(Ptr<TESFile>, u32)),
        entry!(0x00473210, fn_00473210(Ptr<TESFile>, u8)),
        entry!(0x00473250, fn_00473250(Ptr<TESFile>) -> u8),
        entry!(0x00473270, fn_00473270(Ptr)),
        entry!(0x004732d0, fn_004732d0(Ptr<TESFile>) -> u32),
        entry!(0x004732f0, fn_004732f0(Ptr<TESFile>) -> Ptr<BSSimpleList>),
        entry!(0x00473310, tes_file_start_group(Ptr<TESFile>, Ptr)),
        entry!(0x004733a0, tes_file_end_group(Ptr<TESFile>)),
        entry!(0x00473440, fn_00473440(Ptr<TESFile>, Ptr)),
        entry!(0x00473490, fn_00473490(Ptr<TESFile>)),
        entry!(0x004734d0, tes_file_find_form(Ptr<TESFile>, Ptr) -> bool),
        entry!(0x00473640, fn_00473640(Ptr<TESFile>) -> bool),
        entry!(0x00473660, fn_00473660(Ptr<TESFile>) -> bool),
        entry!(
            0x004736c0,
            tes_file_start_and_end_groups_for_form(Ptr<TESFile>, Ptr)
        ),
        entry!(0x00473830, tes_file_close_all_open_groups(Ptr<TESFile>)),
        entry!(0x00473880, fn_00473880(Ptr<TESFile>) -> bool),
        entry!(
            0x004738a0,
            tes_file_is_file_version_too_high(Ptr<TESFile>) -> bool
        ),
        entry!(0x00473930, fn_00473930(Ptr<TESFile>) -> u32),
        entry!(0x00473960, fn_00473960(Ptr<TESFile>)),
        entry!(
            0x004739b0,
            tes_file_get_thread_safe_file(Ptr<TESFile>) -> Ptr<TESFile>
        ),
        entry!(0x00473a10, fn_00473a10(Ptr<TESFile>)),
        entry!(
            0x00473ae0,
            tes_file_get_thread_safe_file_for_thread(Ptr<TESFile>, u32) -> Ptr<TESFile>
        ),
        entry!(0x00473c70, fn_00473c70(Ptr<TESFile>) -> Ptr<TESFile>),
        entry!(0x00473cb0, fn_00473cb0(Ptr<TESFile>, Ptr<TESFile>)),
        entry!(0x00473ce0, fn_00473ce0(Ptr<TESFile>, u32)),
        entry!(0x00473d00, fn_00473d00(Ptr, u32, u32) -> bool),
        entry!(0x00473d20, fn_00473d20(Ptr<TESFile>, bool)),
        entry!(0x00473d70, fn_00473d70(Ptr<TESFile>) -> bool),
        entry!(0x00473d90, fn_00473d90(Ptr<TESFile>)),
        entry!(0x00473f00, fn_00473f00(Ptr<NiTArray>)),
        entry!(0x00473f20, fn_00473f20(u32, Ptr<TESFile>)),
        entry!(0x00473ff0, fn_00473ff0(u32, Ptr<TESFile>) -> bool),
        entry!(0x00474060, tes_file_get_file_for_temp_id(u32) -> u32),
        entry!(0x004740a0, tes_file_decompress_current_form(Ptr<TESFile>)),
        entry!(0x00474460, fn_00474460(u32, u32, u32) -> u32),
        entry!(0x00474480, fn_00474480(u32, u32)),
        entry!(
            0x004744a0,
            fn_004744a0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x004744d0,
            fn_004744d0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00474500,
            ni_t_pointer_map_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00474530,
            ni_t_map_scalar_deleting_destructor(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00474560,
            fn_00474560(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x004745d0, fn_004745d0(Ptr<NiTPointerMap>)),
        entry!(0x00474630, fn_00474630(Ptr<NiTPointerMap>)),
        entry!(
            0x00474660,
            fn_00474660(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x004746d0, fn_004746d0(Ptr<NiTPointerMap>)),
        entry!(0x00474730, fn_00474730(Ptr<NiTPointerMap>)),
        entry!(0x00474760, fn_00474760(Ptr<NiTArray>)),
        entry!(
            0x00474790,
            fn_00474790(Ptr<NiTArray>, u32, u32) -> Ptr<NiTArray>
        ),
        entry!(
            0x004747c0,
            ni_t_map_base_pointer_allocator_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            )
                -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x004747f0,
            ni_t_map_base_scalar_deleting_destructor(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00474880,
            fn_00474880(Ptr<NiTArray>, u32, u32) -> Ptr<NiTArray>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A mapped page of test configuration: word 0 is whether the fake
    /// `BSFile` reports ready, word 1 the size its size method answers,
    /// word 2 the position its seek forces when nonzero, word 3 the `errno`.
    const CONFIG: u32 = 0x0200_0000;
    const CONFIG_READY: u32 = CONFIG;
    const CONFIG_SIZE: u32 = CONFIG + 4;
    const CONFIG_SEEK_RESULT: u32 = CONFIG + 8;
    const CONFIG_ERRNO: u32 = CONFIG + 12;
    /// The fake `BSFile` vtable and the addresses of its methods.
    const FAKE_VTABLE: u32 = 0x0200_1000;
    const FAKE_DESTRUCTOR: u32 = 0x7100_0000;
    const FAKE_SEEK: u32 = 0x7100_0010;
    const FAKE_OPEN: u32 = 0x7100_0020;
    const FAKE_SIZE: u32 = 0x7100_0030;
    /// Tags the tests give the record tag globals.
    const TAG_FILE_HEADER: u32 = 0x3453_4554;
    const TAG_HEADER_ONLY: u32 = 0x5055_5247;
    const TAG_OTHER_HEADER_ONLY: u32 = 0x5055_5248;
    const TAG_RECORD: u32 = 0x5252_4552;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with the exe's data pages this unit reads, the list and
    /// string helpers the game has elsewhere, and a fake file system and
    /// `BSFile`. Everything else a test needs it registers itself.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_9000,
            0x0101_a000,
            0x010a_2000,
            0x0118_6000,
            0x0118_7000,
            0x011c_3000,
            0x011f_6000,
            CONFIG,
            FAKE_VTABLE,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(RECORD_TAG_01187014, TAG_FILE_HEADER);
        e.set_global(RECORD_TAG_01187020, TAG_HEADER_ONLY);
        e.set_global(RECORD_TAG_011873C8, TAG_OTHER_HEADER_ONLY);
        e.set_global(DEFAULT_BUFFER_SIZE, 0x4000u32);
        e.set_global(DEFAULT_HEADER_VERSION, 1.34f32);
        e.set_global(FALLBACK_HEADER_VERSION, 0.85f32);
        e.set_global(EXPECTED_HEADER_VERSION, 0.85f64);
        e.mem.set_cstr(TES_EXTENSION, b".tes");
        e.mem.set_u32(CONFIG_READY, 1);
        e.mem.set_u32(CONFIG_SIZE, 0x1234);

        e.register(LIST_ITEM_SLOT, |_, a| ret(a[0]));
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        e.register(LIST_CLEAR, |_, _| Ret::default());
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_DESTRUCT, |_, _| Ret::default());
        e.register(STRCPY_S, |e, a| e.call(0x00ec_65a6, &a[..3]));
        e.register(STRCAT_S, |e, a| e.call(0x00ec_6bd2, &a[..3]));
        e.register(SNPRINTF, |e, a| {
            // The format is `"%s"`: the master's name.
            let name = e.mem.cstr(a[3]);
            e.mem.set_cstr(a[0], &name);
            Ret::default()
        });
        e.register(STRICMP, |e, a| e.call(0x00ec_68e4, &a[..2]));
        e.register(FILE_NAME, |_, a| ret(a[0] + 0x20));
        e.register(SWAP_U32, |e, a| {
            let value = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], value.swap_bytes());
            ret(value.swap_bytes())
        });
        e.register(SWAP_U16, |e, a| {
            let value = e.mem.u16(a[0]);
            e.mem.set_u16(a[0], value.swap_bytes());
            ret(value.swap_bytes() as u32)
        });
        e.register(FORM_HEADER_SIZE, |_, _| ret(0x18));
        e.register(FREE_DECOMPRESSED_FORM, |_, _| Ret::default());
        e.register(ALWAYS_TRUE, |_, _| ret(1));

        // Windows: a directory with `Data\Fallout.esm` (4096 bytes) and
        // `Data\Mod.tes`.
        e.register(FIND_FIRST_FILE, |e, a| {
            let path = e.mem.cstr(a[0]);
            if path.ends_with(b"Fallout.esm") {
                e.mem.set_u32(a[1] + 0x1c, 0);
                e.mem.set_u32(a[1] + 0x20, 0x1000);
                ret(5)
            } else if path.ends_with(b"Mod.tes") {
                ret(7)
            } else {
                ret(INVALID_HANDLE)
            }
        });
        e.register(FIND_CLOSE, |_, _| ret(1));
        e.register(GET_LAST_ERROR, |_, _| ret(5));
        e.register(FORMAT_MESSAGE, |e, a| {
            let text = e.mem.alloc(8);
            e.mem.set_cstr(text, b"boom");
            e.mem.set_u32(a[4], text);
            ret(4)
        });
        e.register(LOCAL_FREE, |_, _| Ret::default());
        e.register(LSTRCPY, |e, a| e.call(0x00ec_6370, &a[..2]));
        e.register(LSTRCAT, |e, a| e.call(0x00ec_6bd2, &[a[0], 0x104, a[1]]));
        e.register(ERRNO, |_, _| ret(CONFIG_ERRNO));

        // The fake `BSFile`.
        e.put_vtable(
            FAKE_VTABLE,
            &[
                FAKE_DESTRUCTOR,
                0,
                0,
                0,
                0,
                FAKE_SEEK,
                0,
                0,
                FAKE_OPEN,
                0,
                FAKE_SIZE,
            ],
        );
        e.register(FAKE_DESTRUCTOR, |_, _| Ret::default());
        e.register(FAKE_SEEK, |e, a| {
            let forced = e.mem.u32(CONFIG_SEEK_RESULT);
            let position = if forced != 0 { forced } else { a[1] };
            e.mem.set_u32(a[0] + 0x38, position);
            e.mem.set_u32(a[0] + 0x150, position);
            Ret::default()
        });
        e.register(FAKE_OPEN, |_, _| Ret::default());
        e.register(FAKE_SIZE, |e, _| ret(e.mem.u32(CONFIG_SIZE)));
        e.register(BSFILE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], FAKE_VTABLE);
            let ready = e.mem.u32(CONFIG_READY);
            e.mem.set_u8(a[0] + 0x2c, ready as u8);
            ret(a[0])
        });
        e.register(FILE_IS_READY, |e, a| ret(e.mem.u8(a[0] + 0x2c) as u32));
        e
    }

    fn new_file(e: &mut Engine) -> Ptr<TESFile> {
        e.new_object()
    }

    /// A `BSFile` stand-in with the fake vtable.
    fn fake_bsfile(e: &mut Engine) -> Ptr {
        let file = Ptr::new(e.mem.alloc(0x160));
        e.mem.set_u32(file.addr(), FAKE_VTABLE);
        e.mem.set_u8(file.addr() + 0x2c, 1);
        file
    }

    /// A `BSSimpleList` of the given items; returns the head node.
    fn build_list(e: &mut Engine, items: &[u32]) -> u32 {
        let head = e.mem.alloc(8);
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        head
    }

    /// A C string on the heap.
    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let block = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(block, s);
        block
    }

    fn set_name(e: &mut Engine, file: Ptr<TESFile>, path: &[u8], name: &[u8]) {
        e.mem.set_cstr(file.addr() + 0x124, path);
        e.mem.set_cstr(file.addr() + 0x20, name);
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn current_form(file: Ptr<TESFile>) -> Ptr<Form> {
        file.at(TESFile::m_currentform)
    }

    fn set_tag(e: &mut Engine, file: Ptr<TESFile>, tag: u32, length: u32, flags: u32) {
        let form = current_form(file);
        e.set(form, Form::form, tag);
        e.set(form, Form::length, length);
        e.set(form, Form::flags, flags);
    }

    #[test]
    fn layout_offsets_are_the_pc_ones() {
        assert_eq!(TESFile::SIZE, 0x42c);
        assert_eq!(TESFile::m_currentform.off, 0x240);
        assert_eq!(
            TESFile::m_FileInfo.off + Win32FindData::nFileSizeHigh.off,
            0x2b8
        );
        assert_eq!(
            TESFile::fileHeaderInfo.off + FileHeader::iNextFormID.off,
            0x3e4
        );
    }

    #[test]
    fn form_endian_swaps_five_words_and_two_halfwords() {
        let mut e = engine();
        let form: Ptr<Form> = e.new_object();
        e.set(form, Form::form, 0x1122_3344);
        e.set(form, Form::length, 0x0000_0010);
        e.set(form, Form::flags, 0x0100_0000);
        e.set(form, Form::iFormID, 0xaabb_ccdd);
        e.set(form, Form::iVersionControl, 1);
        e.set(form, Form::sFormVersion, 0x1234);
        e.set(form, Form::sVCVersion, 0x00ff);
        e.call_log = Some(vec![]);
        e.call(0x0047_0650, &args![form]);
        assert_eq!(e.get(form, Form::form), 0x4433_2211);
        assert_eq!(e.get(form, Form::length), 0x1000_0000);
        assert_eq!(e.get(form, Form::flags), 1);
        assert_eq!(e.get(form, Form::iFormID), 0xddcc_bbaa);
        assert_eq!(e.get(form, Form::iVersionControl), 0x0100_0000);
        assert_eq!(e.get(form, Form::sFormVersion), 0x3412);
        assert_eq!(e.get(form, Form::sVCVersion), 0xff00);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SWAP_U32).len(), 5);
        assert_eq!(calls_to(&log, SWAP_U16).len(), 2);
    }

    #[test]
    fn constructor_initialises_and_gives_up_when_the_file_is_missing() {
        let mut e = engine();
        let file = new_file(&mut e);
        let directory = text(&mut e, b"Data\\");
        let name = text(&mut e, b"Gone.esm");
        // Scribble over what the constructor must reset.
        e.set(file, TESFile::m_filesize, 77);
        e.set(file, TESFile::m_Flags, 0xff);
        e.call_log = Some(vec![]);
        let result = e.call(0x0047_0710, &args![file, directory, name, 0u32]);
        assert_eq!(result.ptr::<TESFile>(), file);
        assert_eq!(e.get(file, TESFile::m_filesize), 0);
        assert_eq!(e.get(file, TESFile::m_Flags), 0);
        assert_eq!(e.get(file, TESFile::cCompileIndex), 0xff);
        assert_eq!(e.get(file, TESFile::m_uiBufferAllocSize), 0x4000);
        let header = file.at(TESFile::fileHeaderInfo);
        assert_eq!(e.get(header, FileHeader::fVersion), 1.34);
        assert_eq!(e.get(header, FileHeader::iNextFormID), 0x800);
        // `Gone.esm` and `Gone.tes` do not exist.
        assert_eq!(e.get(file, TESFile::m_lastError), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LIST_CONSTRUCT).len(), 3);
        assert_eq!(calls_to(&log, STRING_CONSTRUCT).len(), 2);
        assert!(calls_to(&log, CLOSE_ALL_GROUPS).is_empty());
    }

    #[test]
    fn constructor_checks_the_header_and_closes_the_opened_file() {
        let mut e = engine();
        e.register(CLOSE_ALL_GROUPS, |_, _| Ret::default());
        e.register(READ_HEADER_STATUS, |_, _| ret(1));
        let file = new_file(&mut e);
        let directory = text(&mut e, b"Data\\");
        let name = text(&mut e, b"Fallout.esm");
        e.register(BSFILE_READ, |e, a| {
            e.mem.set_u32(a[1], TAG_FILE_HEADER);
            ret(0x18)
        });
        e.register(READ_FORM_HEADER, |e, a| {
            e.mem.set_u32(a[0] + 0x240, TAG_RECORD);
            ret(1)
        });
        e.register(TYPE_FROM_FORM_TAG, |_, _| ret(7));
        e.register(GET_TES_CHUNK, |e, a| {
            e.mem.set_u32(a[0] + 0x258, CHUNK_ID_HEDR);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0047_0710, &args![file, directory, name, 0u32]);
        let log = e.call_log.take().unwrap();
        // The file was found and had a size, the header check failed, the
        // complaint names the file, and the file was closed.
        assert_eq!(
            calls_to(&log, LOG),
            vec![vec![MESSAGE_NOT_VALID_TES_FILE, name]]
        );
        assert_eq!(e.get(file, TESFile::m_pFile), Ptr::NULL);
        assert_eq!(calls_to(&log, CLOSE_ALL_GROUPS).len(), 1);
    }

    #[test]
    fn destructor_frees_both_lists_and_the_tables() {
        let mut e = engine();
        e.register(CLOSE_ALL_GROUPS, |_, _| Ret::default());
        e.register(DESTRUCT_CLEANUP_A, |_, _| Ret::default());
        e.register(DESTRUCT_CLEANUP_B, |_, _| Ret::default());
        let file = new_file(&mut e);
        let master_a = e.mem.alloc(8);
        let master_b = e.mem.alloc(8);
        let size = e.mem.alloc(8);
        let buffer = e.mem.alloc(8);
        let table = e.mem.alloc(8);
        let masters = build_list(&mut e, &[master_a, master_b]);
        let sizes = build_list(&mut e, &[size]);
        e.mem.write(file.addr() + 0x3ec, &e.mem.bytes(masters, 8));
        e.mem.write(file.addr() + 0x3f4, &e.mem.bytes(sizes, 8));
        e.set(file, TESFile::m_pBuffer, Ptr::new(buffer));
        e.set(file, TESFile::m_pMasterPtrs, Ptr::new(table));
        e.call_log = Some(vec![]);
        e.call(0x0047_09f0, &args![file]);
        let log = e.call_log.take().unwrap();
        assert!(e.get(file, TESFile::bCloseFileOverride));
        assert_eq!(e.get(file, TESFile::m_pMasterPtrs), Ptr::NULL);
        for freed in [master_a, master_b, size, buffer, table] {
            assert_eq!(e.mem.block_size(freed), None, "{freed:08x} still allocated");
        }
        assert_eq!(calls_to(&log, LIST_CLEAR).len(), 2);
        assert_eq!(calls_to(&log, LIST_DESTRUCT).len(), 3);
        assert_eq!(
            calls_to(&log, STRING_DESTRUCT),
            vec![vec![file.addr() + 0x418], vec![file.addr() + 0x410]]
        );
    }

    #[test]
    fn list_of_master_sizes_is_at_3f4() {
        let mut e = engine();
        let file = new_file(&mut e);
        let list = e.call(0x0047_0b90, &args![file]).u32();
        assert_eq!(list, file.addr() + 0x3f4);
    }

    #[test]
    fn answers_fifteen() {
        let mut e = engine();
        assert_eq!(e.call(0x0047_0bb0, &args![]).u32(), 0xf);
    }

    #[test]
    fn master_lists_are_emptied_and_the_count_reset() {
        let mut e = engine();
        let file = new_file(&mut e);
        let master_a = e.mem.alloc(8);
        let master_b = e.mem.alloc(8);
        let size = e.mem.alloc(8);
        let masters = build_list(&mut e, &[master_a, master_b]);
        let sizes = build_list(&mut e, &[size]);
        e.mem.write(file.addr() + 0x3ec, &e.mem.bytes(masters, 8));
        e.mem.write(file.addr() + 0x3f4, &e.mem.bytes(sizes, 8));
        e.set(file, TESFile::iMasterCount, 2);
        e.call_log = Some(vec![]);
        e.call(0x0047_0bc0, &args![file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(file, TESFile::iMasterCount), 0);
        assert_eq!(e.mem.u32(file.addr() + 0x3ec), 0);
        assert_eq!(e.mem.u32(file.addr() + 0x3f4), 0);
        let deleted: Vec<u32> = calls_to(&log, OPERATOR_DELETE)
            .iter()
            .map(|args| args[0])
            .collect();
        assert_eq!(deleted, vec![master_a, master_b, size]);
        assert_eq!(calls_to(&log, LIST_REMOVE_HEAD).len(), 3);
    }

    #[test]
    fn open_tes_opens_the_remembered_path_and_name() {
        let mut e = engine();
        let file = new_file(&mut e);
        set_name(&mut e, file, b"Data\\", b"Gone.esm");
        assert!(!e.call(0x0047_0c70, &args![file, 1u32, 0u8]).bool());
        assert_eq!(e.get(file, TESFile::m_lastError), 2);
        assert_eq!(e.mem.cstr(file.addr() + 0x124), b"Data\\");
        assert_eq!(e.mem.cstr(file.addr() + 0x20), b"Gone.esm");
    }

    #[test]
    fn open_needs_a_directory_and_a_name() {
        let mut e = engine();
        let file = new_file(&mut e);
        let name = text(&mut e, b"Fallout.esm");
        assert!(!e
            .call(0x0047_0ca0, &args![file, 0u32, name, 1u32, 0u8])
            .bool());
        assert!(!e
            .call(0x0047_0ca0, &args![file, name, 0u32, 1u32, 0u8])
            .bool());
        assert_eq!(e.get(file, TESFile::m_pFile), Ptr::NULL);
    }

    #[test]
    fn open_finds_the_file_and_creates_the_bsfile() {
        let mut e = engine();
        let file = new_file(&mut e);
        e.set(file, TESFile::m_uiBufferAllocSize, 0x4000);
        e.set(file, TESFile::m_currentchunkID, CHUNK_ID_HEDR);
        let directory = text(&mut e, b"Data\\");
        let name = text(&mut e, b"Fallout.esm");
        e.call_log = Some(vec![]);
        let opened = e.call(0x0047_0ca0, &args![file, directory, name, 1u32, 0u8]);
        assert!(opened.bool());
        let log = e.call_log.take().unwrap();
        let created = calls_to(&log, BSFILE_CONSTRUCT);
        assert_eq!(created.len(), 1);
        let args = &created[0];
        assert_eq!(e.mem.cstr(args[1]), b"Data\\Fallout.esm");
        assert_eq!(&args[2..], &[1, 0x4000, 0]);
        assert_eq!(e.mem.cstr(file.addr() + 0x124), b"Data\\");
        assert_eq!(e.mem.cstr(file.addr() + 0x20), b"Fallout.esm");
        assert_eq!(e.get(file, TESFile::m_pFile).addr(), args[0]);
        assert_eq!(e.get(file, TESFile::m_filesize), 0x1234);
        // The find data is new: remembered, and flagged.
        assert_eq!(
            e.get(file.at(TESFile::m_FileInfo), Win32FindData::nFileSizeLow),
            0x1000
        );
        assert_eq!(e.get(file, TESFile::m_Flags) & FLAG_FIND_DATA_CHANGED, 2);
        assert_eq!(calls_to(&log, FIND_CLOSE), vec![vec![5]]);
        // Mode 1 does not read the header.
        assert!(calls_to(&log, BSFILE_READ).is_empty());
    }

    #[test]
    fn open_reports_missing_files_in_the_error_code() {
        let mut e = engine();
        let file = new_file(&mut e);
        let directory = text(&mut e, b"Data\\");
        let with_extension = text(&mut e, b"Mod.esm");
        let missing = text(&mut e, b"Nothing.esm");
        // `Mod.tes` exists, `Mod.esm` does not.
        assert!(!e
            .call(
                0x0047_0ca0,
                &args![file, directory, with_extension, 1u32, 0u8]
            )
            .bool());
        assert_eq!(e.get(file, TESFile::m_lastError), 0xc);
        // The name keeps its extension.
        assert_eq!(e.mem.cstr(file.addr() + 0x20), b"Mod.esm");
        assert!(!e
            .call(0x0047_0ca0, &args![file, directory, missing, 1u32, 0u8])
            .bool());
        assert_eq!(e.get(file, TESFile::m_lastError), 2);
        assert_eq!(e.get(file, TESFile::m_pFile), Ptr::NULL);
    }

    #[test]
    fn open_maps_a_failed_bsfile_to_an_error_code() {
        let mut e = engine();
        let directory = text(&mut e, b"Data\\");
        let name = text(&mut e, b"Fallout.esm");
        e.mem.set_u32(CONFIG_READY, 0);
        for (errno, code) in [(2u32, 2u32), (0xd, 9), (5, 0)] {
            let file = new_file(&mut e);
            e.mem.set_u32(CONFIG_ERRNO, errno);
            assert!(!e
                .call(0x0047_0ca0, &args![file, directory, name, 1u32, 0u8])
                .bool());
            assert_eq!(e.get(file, TESFile::m_lastError), code);
            // The `BSFile` object stays set.
            assert!(!e.get(file, TESFile::m_pFile).is_null());
        }
    }

    #[test]
    fn open_in_mode_zero_reads_the_header_and_retries_with_the_fallback_version() {
        let mut e = engine();
        e.register(BSFILE_READ, |e, a| {
            e.mem.set_u32(a[1], TAG_FILE_HEADER);
            ret(0x18)
        });
        e.register(READ_FORM_HEADER, |e, a| {
            e.mem.set_u32(a[0] + 0x240, TAG_RECORD);
            ret(1)
        });
        e.register(TYPE_FROM_FORM_TAG, |_, _| ret(7));
        // The first chunk is not `HEDR` the first time, then it is.
        let mut reads = 0;
        e.register_double(GET_TES_CHUNK, move |e, a| {
            reads += 1;
            let id = if reads == 1 { 0x1234 } else { CHUNK_ID_HEDR };
            e.mem.set_u32(a[0] + 0x258, id);
            Ret::default()
        });
        let file = new_file(&mut e);
        let directory = text(&mut e, b"Data\\");
        let name = text(&mut e, b"Fallout.esm");
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0047_0ca0, &args![file, directory, name, 0u32, 0u8])
            .bool());
        let log = e.call_log.take().unwrap();
        // 1.34 is not 0.85: the fallback version is stored and the open
        // runs again on the already open file.
        let header = file.at(TESFile::fileHeaderInfo);
        assert_eq!(e.get(header, FileHeader::fVersion), 0.85f32);
        assert_eq!(calls_to(&log, BSFILE_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&log, GET_TES_CHUNK).len(), 2);
        assert!(!e.get(file, TESFile::bMustEndianConvert));
        assert_eq!(e.get(file, TESFile::m_currentchunkID), CHUNK_ID_HEDR);
    }

    #[test]
    fn write_flag_forces_mode_one_and_calls_the_stub() {
        let mut e = engine();
        e.register(STUB_RETURNS_ZERO, |_, _| ret(0));
        let file = new_file(&mut e);
        e.set(file, TESFile::m_currentchunkID, CHUNK_ID_HEDR);
        let directory = text(&mut e, b"Data\\");
        let name = text(&mut e, b"Fallout.esm");
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0047_0ca0, &args![file, directory, name, 0u32, 1u8])
            .bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, BSFILE_CONSTRUCT)[0][2], 1);
        assert_eq!(calls_to(&log, STUB_RETURNS_ZERO).len(), 1);
    }

    #[test]
    fn header_version_setter_stores_the_float() {
        let mut e = engine();
        let file = new_file(&mut e);
        e.call(0x0047_1110, &args![file, 0.94f32]);
        assert_eq!(e.mem.f32(file.addr() + 0x3dc), 0.94);
    }

    #[test]
    fn close_does_nothing_when_the_handler_says_not_to() {
        let mut e = engine();
        e.register(HANDLER_FLAG, |_, _| ret(0));
        e.set_global(DATA_HANDLER, 0x1234u32);
        let file = new_file(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_1130, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2, "{log:?}");
        assert_eq!(log[1].0, HANDLER_FLAG);
    }

    #[test]
    fn close_destroys_the_bsfile_and_closes_the_groups() {
        let mut e = engine();
        e.register(CLOSE_ALL_GROUPS, |_, _| Ret::default());
        e.register(HANDLER_FLAG, |_, _| ret(1));
        e.set_global(DATA_HANDLER, 0x1234u32);
        let file = new_file(&mut e);
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_1130, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(file, TESFile::m_pFile), Ptr::NULL);
        assert_eq!(calls_to(&log, CLOSE_ALL_GROUPS).len(), 1);
        assert_eq!(
            calls_to(&log, FAKE_DESTRUCTOR),
            vec![vec![bsfile.addr(), 1]]
        );
        assert_eq!(calls_to(&log, FREE_DECOMPRESSED_FORM).len(), 1);
    }

    #[test]
    fn close_replaces_the_original_with_the_saved_temporary_file() {
        let mut e = engine();
        e.register(CLOSE_ALL_GROUPS, |_, _| Ret::default());
        e.register(DELETE_SYSTEM_FILE, |_, _| ret(0));
        e.register(RENAME, |_, _| ret(0));
        let file = new_file(&mut e);
        set_name(&mut e, file, b"Data\\", b"Mod.esp");
        let locked = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pLockedFile, locked);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_1130, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(file, TESFile::m_pLockedFile), Ptr::NULL);
        assert_eq!(
            calls_to(&log, FAKE_DESTRUCTOR),
            vec![vec![locked.addr(), 1]]
        );
        let deleted = calls_to(&log, DELETE_SYSTEM_FILE);
        assert_eq!(e.mem.cstr(deleted[0][0]), b"Data\\Mod.esp");
        // `rename(temporary, original)`.
        let renamed = calls_to(&log, RENAME);
        assert_eq!(e.mem.cstr(renamed[0][0]), b"Data\\Mod.tes");
        assert_eq!(e.mem.cstr(renamed[0][1]), b"Data\\Mod.esp");
        assert!(calls_to(&log, LOG).is_empty());
        // The name's dot was put back.
        assert_eq!(e.mem.cstr(file.addr() + 0x20), b"Mod.esp");
    }

    #[test]
    fn close_without_an_extension_appends_it_to_the_temporary_name() {
        let mut e = engine();
        e.register(CLOSE_ALL_GROUPS, |_, _| Ret::default());
        e.register(DELETE_SYSTEM_FILE, |_, _| ret(0));
        e.register(RENAME, |_, _| ret(0));
        let file = new_file(&mut e);
        set_name(&mut e, file, b"Data\\", b"Mod");
        let locked = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pLockedFile, locked);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_1130, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        let renamed = calls_to(&log, RENAME);
        assert_eq!(e.mem.cstr(renamed[0][0]), b"Data\\Mod.tes");
    }

    #[test]
    fn close_reports_a_failed_delete() {
        let mut e = engine();
        e.register(CLOSE_ALL_GROUPS, |_, _| Ret::default());
        e.register(DELETE_SYSTEM_FILE, |_, _| ret(1));
        e.register(RENAME, |_, _| ret(0));
        e.register(NOTE_FAILURE, |_, _| Ret::default());
        let file = new_file(&mut e);
        set_name(&mut e, file, b"Data\\", b"Mod.esp");
        let locked = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pLockedFile, locked);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_1130, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(file, TESFile::m_lastError), 9);
        assert!(calls_to(&log, RENAME).is_empty());
        assert_eq!(calls_to(&log, NOTE_FAILURE).len(), 1);
        let complaint = calls_to(&log, LOG);
        assert_eq!(complaint[0][0], MESSAGE_DELETE_FAILED);
        assert_eq!(e.mem.cstr(complaint[0][1]), b"Data\\Mod.esp");
        assert_eq!(e.mem.cstr(complaint[0][2]), b"Data\\Mod.tes");
    }

    #[test]
    fn close_reports_a_failed_rename() {
        let mut e = engine();
        e.register(CLOSE_ALL_GROUPS, |_, _| Ret::default());
        e.register(DELETE_SYSTEM_FILE, |_, _| ret(0));
        e.register(RENAME, |_, _| ret(1));
        e.register(NOTE_FAILURE, |_, _| Ret::default());
        let file = new_file(&mut e);
        set_name(&mut e, file, b"Data\\", b"Mod.esp");
        let locked = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pLockedFile, locked);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_1130, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(file, TESFile::m_lastError), 9);
        let complaint = calls_to(&log, LOG);
        assert_eq!(complaint[0][0], MESSAGE_RENAME_FAILED);
        assert_eq!(e.mem.cstr(complaint[0][1]), b"Data\\Mod.tes");
    }

    #[test]
    fn master_count_is_read() {
        let mut e = engine();
        let file = new_file(&mut e);
        e.set(file, TESFile::iMasterCount, 3);
        assert_eq!(e.call(0x0047_1850, &args![file]).u32(), 3);
        assert_eq!(e.call(0x0047_1ad0, &args![file]).u32(), 3);
    }

    /// A file with `names` as masters and a list of loaded files named
    /// `loaded`; returns (file, list head, loaded files).
    fn index_setup(
        e: &mut Engine,
        names: &[&[u8]],
        loaded: &[&[u8]],
    ) -> (Ptr<TESFile>, u32, Vec<u32>) {
        let file = new_file(e);
        let name_blocks: Vec<u32> = names.iter().map(|n| text(e, n)).collect();
        let masters = build_list(e, &name_blocks);
        e.mem.write(file.addr() + 0x3ec, &e.mem.bytes(masters, 8));
        e.set(file, TESFile::iMasterCount, names.len() as u32);
        let mut files = Vec::new();
        for name in loaded {
            let other = new_file(e);
            e.mem.set_cstr(other.addr() + 0x20, name);
            files.push(other.addr());
        }
        let list = build_list(e, &files);
        (file, list, files)
    }

    #[test]
    fn index_table_maps_each_master_to_its_loaded_file() {
        let mut e = engine();
        let (file, list, files) = index_setup(&mut e, &[b"B.esm", b"a.esm"], &[b"A.esm", b"B.esm"]);
        let stale = e.mem.alloc(0x40);
        e.set(file, TESFile::m_pMasterPtrs, Ptr::new(stale));
        assert!(e.call(0x0047_1870, &args![file, list, 1u8]).bool());
        assert_eq!(e.mem.block_size(stale), None);
        let table = e.get(file, TESFile::m_pMasterPtrs).addr();
        // Names compare without case.
        assert_eq!(e.mem.u32(table), files[1]);
        assert_eq!(e.mem.u32(table + 4), files[0]);
    }

    #[test]
    fn index_table_reports_missing_masters() {
        let mut e = engine();
        let (file, list, files) =
            index_setup(&mut e, &[b"A.esm", b"Gone.esm"], &[b"A.esm", b"B.esm"]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_1870, &args![file, list, 1u8]).bool());
        let log = e.call_log.take().unwrap();
        let table = e.get(file, TESFile::m_pMasterPtrs).addr();
        assert_eq!(e.mem.u32(table), files[0]);
        assert_eq!(e.mem.u32(table + 4), 0);
        let complaint = calls_to(&log, LOG);
        assert_eq!(complaint.len(), 1);
        assert_eq!(e.mem.cstr(complaint[0][1]), b"Gone.esm");
        // Silent when not asked.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_1870, &args![file, list, 0u8]).bool());
        assert!(calls_to(&e.call_log.take().unwrap(), LOG).is_empty());
    }

    #[test]
    fn index_table_without_masters_frees_the_old_table() {
        let mut e = engine();
        let file = new_file(&mut e);
        let stale = e.mem.alloc(8);
        e.set(file, TESFile::m_pMasterPtrs, Ptr::new(stale));
        let list = build_list(&mut e, &[]);
        assert!(e.call(0x0047_1870, &args![file, list, 1u8]).bool());
        assert_eq!(e.get(file, TESFile::m_pMasterPtrs), Ptr::NULL);
        assert_eq!(e.mem.block_size(stale), None);
    }

    #[test]
    fn index_file_zero_is_the_file_itself() {
        let mut e = engine();
        let (file, list, files) = index_setup(&mut e, &[b"A.esm", b"B.esm"], &[b"A.esm", b"B.esm"]);
        // Without a table everything but zero is null.
        assert_eq!(
            e.call(0x0047_1a10, &args![file, 0u32]).ptr::<()>(),
            file.cast()
        );
        assert_eq!(e.call(0x0047_1a10, &args![file, 1u32]).u32(), 0);
        e.call(0x0047_1870, &args![file, list, 0u8]);
        assert_eq!(e.call(0x0047_1a10, &args![file, 1u32]).u32(), files[0]);
        assert_eq!(e.call(0x0047_1a10, &args![file, 2u32]).u32(), files[1]);
        // Past the master count.
        assert_eq!(e.call(0x0047_1a10, &args![file, 3u32]).u32(), 0);
    }

    #[test]
    fn nth_master_name_counts_from_one() {
        let mut e = engine();
        let (file, _, _) = index_setup(&mut e, &[b"A.esm", b"B.esm"], &[]);
        let first = e.call(0x0047_1a60, &args![file, 1u32]).u32();
        let second = e.call(0x0047_1a60, &args![file, 2u32]).u32();
        assert_eq!(e.mem.cstr(first), b"A.esm");
        assert_eq!(e.mem.cstr(second), b"B.esm");
        assert_eq!(e.call(0x0047_1a60, &args![file, 3u32]).u32(), 0);
        // Position 0 behaves as 1.
        assert_eq!(e.call(0x0047_1a60, &args![file, 0u32]).u32(), first);
        let (empty, _, _) = index_setup(&mut e, &[], &[]);
        assert_eq!(e.call(0x0047_1a60, &args![empty, 1u32]).u32(), 0);
    }

    /// A file whose master "Fallout.esm" was recorded with the given size.
    fn size_check_setup(e: &mut Engine, name: &[u8], low: u32, high: u32) -> Ptr<TESFile> {
        let file = new_file(e);
        let master = text(e, name);
        let masters = build_list(e, &[master]);
        let recorded = e.mem.alloc(8);
        e.mem.set_u32(recorded, low);
        e.mem.set_u32(recorded + 4, high);
        let sizes = build_list(e, &[recorded]);
        e.mem.write(file.addr() + 0x3ec, &e.mem.bytes(masters, 8));
        e.mem.write(file.addr() + 0x3f4, &e.mem.bytes(sizes, 8));
        file
    }

    #[test]
    fn size_check_compares_the_recorded_size_with_the_file_on_disk() {
        let mut e = engine();
        let same = size_check_setup(&mut e, b"Fallout.esm", 0x1000, 0);
        assert!(!e.call(0x0047_1af0, &args![same]).bool());
        let grown = size_check_setup(&mut e, b"Fallout.esm", 0x0f00, 0);
        assert!(e.call(0x0047_1af0, &args![grown]).bool());
        let high_differs = size_check_setup(&mut e, b"Fallout.esm", 0x1000, 1);
        assert!(e.call(0x0047_1af0, &args![high_differs]).bool());
    }

    #[test]
    fn size_check_treats_a_missing_file_as_size_zero() {
        let mut e = engine();
        let never_had_one = size_check_setup(&mut e, b"Gone.esm", 0, 0);
        assert!(!e.call(0x0047_1af0, &args![never_had_one]).bool());
        let lost = size_check_setup(&mut e, b"Gone.esm", 0x10, 0);
        assert!(e.call(0x0047_1af0, &args![lost]).bool());
        let empty = new_file(&mut e);
        let sizes = build_list(&mut e, &[]);
        let names = build_list(&mut e, &[]);
        e.mem.write(empty.addr() + 0x3ec, &e.mem.bytes(names, 8));
        e.mem.write(empty.addr() + 0x3f4, &e.mem.bytes(sizes, 8));
        assert!(!e.call(0x0047_1af0, &args![empty]).bool());
    }

    #[test]
    fn flag_getters_and_setters_use_their_bits() {
        let mut e = engine();
        let file = new_file(&mut e);
        // (getter, setter, bit)
        let pairs: [(u32, Option<u32>, u32); 5] = [
            (0x0047_1c20, Some(0x0047_1c50), 0x01),
            (0x0047_1cd0, Some(0x0047_1d00), 0x04),
            (0x0047_1d60, Some(0x0047_1d90), 0x08),
            (0x0047_1de0, Some(0x0047_1e10), 0x40),
            (0x0047_1ca0, None, 0x10),
        ];
        for (getter, setter, bit) in pairs {
            e.set(file, TESFile::m_Flags, 0);
            assert!(!e.call(getter, &args![file]).bool());
            e.set(file, TESFile::m_Flags, bit);
            assert!(e.call(getter, &args![file]).bool());
            e.set(file, TESFile::m_Flags, !bit);
            assert!(!e.call(getter, &args![file]).bool());
            if let Some(setter) = setter {
                e.set(file, TESFile::m_Flags, 0);
                e.call(setter, &args![file, true]);
                assert_eq!(e.get(file, TESFile::m_Flags), bit);
                e.call(setter, &args![file, false]);
                assert_eq!(e.get(file, TESFile::m_Flags), 0);
            }
        }
    }

    #[test]
    fn clearing_bit_four_also_clears_the_active_bit() {
        let mut e = engine();
        let file = new_file(&mut e);
        e.set(file, TESFile::m_Flags, 0x4 | 0x8 | 0x1);
        e.call(0x0047_1d00, &args![file, false]);
        assert_eq!(e.get(file, TESFile::m_Flags), 0x1);
        // Setting it leaves the others alone.
        e.set(file, TESFile::m_Flags, 0x8);
        e.call(0x0047_1d00, &args![file, true]);
        assert_eq!(e.get(file, TESFile::m_Flags), 0xc);
        // Clearing the active bit alone leaves bit four.
        e.call(0x0047_1d90, &args![file, false]);
        assert_eq!(e.get(file, TESFile::m_Flags), 0x4);
    }

    #[test]
    fn stream_object_is_the_global() {
        let mut e = engine();
        e.set_global(STREAM_OBJECT, 0x4242u32);
        assert_eq!(e.call(0x0047_1f70, &args![]).u32(), 0x4242);
    }

    #[test]
    fn first_header_with_the_expected_tag_needs_no_conversion() {
        let mut e = engine();
        e.register(BSFILE_READ, |e, a| {
            e.mem.set_u32(a[1], TAG_FILE_HEADER);
            ret(0x18)
        });
        let file = new_file(&mut e);
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        e.set(file, TESFile::bMustEndianConvert, true);
        e.set(file, TESFile::m_Flags, 0);
        e.call_log = Some(vec![]);
        e.call(0x0047_1e60, &args![file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FAKE_SEEK), vec![vec![bsfile.addr(), 0, 0]]);
        assert_eq!(e.get(file, TESFile::m_Flags), FLAG_BIT_40);
        assert!(!e.get(file, TESFile::bMustEndianConvert));
        assert!(calls_to(&log, SWAP_U32).is_empty());
    }

    #[test]
    fn first_header_in_the_other_byte_order_is_swapped_and_flagged() {
        let mut e = engine();
        e.register(BSFILE_READ, |e, a| {
            e.mem.set_u32(a[1], TAG_FILE_HEADER.swap_bytes());
            ret(0x18)
        });
        let file = new_file(&mut e);
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        e.set(file, TESFile::m_Flags, FLAG_BIT_40);
        e.call(0x0047_1e60, &args![file]);
        assert_eq!(e.get(current_form(file), Form::form), TAG_FILE_HEADER);
        // Bit 6 became the inverse of the answer, which then differs.
        assert_eq!(e.get(file, TESFile::m_Flags), 0);
        assert!(e.get(file, TESFile::bMustEndianConvert));
    }

    #[test]
    fn first_header_with_an_unknown_tag_leaves_the_bit_alone() {
        let mut e = engine();
        e.register(BSFILE_READ, |e, a| {
            e.mem.set_u32(a[1], 0x0bad_f00d);
            ret(0x18)
        });
        let file = new_file(&mut e);
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        e.call(0x0047_1e60, &args![file]);
        // Bit 6 is 0, the answer is 1: they differ.
        assert!(e.get(file, TESFile::bMustEndianConvert));
        assert_eq!(e.get(file, TESFile::m_Flags), 0);
    }

    #[test]
    fn first_header_does_nothing_without_a_file_or_a_read() {
        let mut e = engine();
        let file = new_file(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0047_1e60, &args![file]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.register(BSFILE_READ, |_, _| ret(0));
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        e.set(file, TESFile::bMustEndianConvert, true);
        e.call(0x0047_1e60, &args![file]);
        assert!(e.get(file, TESFile::bMustEndianConvert));
    }

    #[test]
    fn rewind_goes_back_to_the_start() {
        let mut e = engine();
        e.register(READ_FORM_HEADER, |e, a| {
            e.mem.set_u32(a[0] + 0x240, TAG_RECORD);
            ret(1)
        });
        e.register(GET_TES_CHUNK, |_, _| Ret::default());
        e.register(TYPE_FROM_FORM_TAG, |_, _| ret(7));
        let file = new_file(&mut e);
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        e.set(file, TESFile::m_fileoffset, 100);
        e.set(file, TESFile::m_formoffset, 10);
        e.set(file, TESFile::m_chunkoffset, 4);
        set_tag(&mut e, file, 0x1111, 5, 6);
        e.call_log = Some(vec![]);
        e.call(0x0047_1f80, &args![file, 0u8]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FAKE_SEEK), vec![vec![bsfile.addr(), 0, 0]]);
        assert_eq!(e.get(file, TESFile::m_fileoffset), 0);
        assert_eq!(e.get(file, TESFile::m_formoffset), 0);
        assert_eq!(e.get(file, TESFile::m_chunkoffset), 0);
        assert_eq!(e.get(current_form(file), Form::form), 0);
        assert!(calls_to(&log, READ_FORM_HEADER).is_empty());
        // Asked to load the first record, it does.
        e.call(0x0047_1f80, &args![file, 1u8]);
        assert_eq!(e.get(current_form(file), Form::form), TAG_RECORD);
    }

    #[test]
    fn rewind_works_without_an_open_file() {
        let mut e = engine();
        let file = new_file(&mut e);
        e.set(file, TESFile::m_fileoffset, 100);
        e.call_log = Some(vec![]);
        e.call(0x0047_1f80, &args![file, 0u8]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FAKE_SEEK).is_empty());
        assert_eq!(e.get(file, TESFile::m_fileoffset), 0);
    }

    #[test]
    fn first_chunk_of_a_plain_record_is_found_after_its_header() {
        let mut e = engine();
        e.register(GET_TES_CHUNK, |_, _| Ret::default());
        let file = new_file(&mut e);
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        e.set(file, TESFile::m_fileoffset, 0x100);
        e.set(file, TESFile::m_formoffset, 9);
        e.set(file, TESFile::m_chunkoffset, 9);
        e.set(file, TESFile::m_currentchunkID, 5);
        e.set(file, TESFile::m_actualChunkSize, 5);
        set_tag(&mut e, file, TAG_RECORD, 0x40, 0);
        e.call_log = Some(vec![]);
        e.call(0x0047_2000, &args![file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FAKE_SEEK),
            vec![vec![bsfile.addr(), 0x118, 0]]
        );
        assert_eq!(e.get(file, TESFile::m_formoffset), 0);
        assert_eq!(e.get(file, TESFile::m_chunkoffset), 0);
        assert_eq!(e.get(file, TESFile::m_currentchunkID), 0);
        assert_eq!(e.get(file, TESFile::m_actualChunkSize), 0);
        assert_eq!(calls_to(&log, GET_TES_CHUNK), vec![vec![file.addr()]]);
    }

    #[test]
    fn first_chunk_of_a_compressed_record_does_not_seek() {
        let mut e = engine();
        e.register(GET_TES_CHUNK, |_, _| Ret::default());
        let file = new_file(&mut e);
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        set_tag(&mut e, file, TAG_RECORD, 0x40, RECORD_FLAG_COMPRESSED);
        e.call_log = Some(vec![]);
        e.call(0x0047_2000, &args![file]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FAKE_SEEK).is_empty());
        assert_eq!(calls_to(&log, GET_TES_CHUNK).len(), 1);
    }

    #[test]
    fn bsfile_position_prefers_the_first_copy_unless_it_is_minus_one() {
        let mut e = engine();
        let bsfile = fake_bsfile(&mut e);
        e.mem.set_u32(bsfile.addr() + 0x38, 0x77);
        e.mem.set_u32(bsfile.addr() + 0x150, 0x99);
        assert_eq!(e.call(0x0047_20a0, &args![bsfile]).u32(), 0x77);
        e.mem.set_u32(bsfile.addr() + 0x38, u32::MAX);
        assert_eq!(e.call(0x0047_20a0, &args![bsfile]).u32(), 0x99);
        assert_eq!(e.call(0x0047_2380, &args![bsfile]).u32(), 0x99);
    }

    #[test]
    fn clearing_the_chunk_zeroes_id_and_size() {
        let mut e = engine();
        let file = new_file(&mut e);
        e.set(file, TESFile::m_currentchunkID, 0x1234);
        e.set(file, TESFile::m_actualChunkSize, 0x55);
        e.call(0x0047_20d0, &args![file]);
        assert_eq!(e.get(file, TESFile::m_currentchunkID), 0);
        assert_eq!(e.get(file, TESFile::m_actualChunkSize), 0);
    }

    #[test]
    fn compressed_means_the_flag_on_a_tag_that_is_not_header_only() {
        let mut e = engine();
        let file = new_file(&mut e);
        set_tag(&mut e, file, TAG_RECORD, 0, 0);
        assert!(!e.call(0x0047_2100, &args![file]).bool());
        set_tag(&mut e, file, TAG_RECORD, 0, RECORD_FLAG_COMPRESSED);
        assert!(e.call(0x0047_2100, &args![file]).bool());
        set_tag(&mut e, file, TAG_HEADER_ONLY, 0, RECORD_FLAG_COMPRESSED);
        assert!(!e.call(0x0047_2100, &args![file]).bool());
    }

    /// A file at the start of a 0x1000-byte stream whose reads produce
    /// records with the given flags in turn.
    fn stream_file(e: &mut Engine, record_flags: &'static [u32]) -> Ptr<TESFile> {
        let mut next = 0;
        e.register_double(READ_FORM_HEADER, move |e, a| {
            let flags = record_flags[next.min(record_flags.len() - 1)];
            next += 1;
            e.mem.set_u32(a[0] + 0x240, TAG_RECORD);
            e.mem.set_u32(a[0] + 0x244, 0x20);
            e.mem.set_u32(a[0] + 0x248, flags);
            ret(1)
        });
        e.register(READ_CHUNK_HEADER, |_, _| Ret::default());
        let file = new_file(e);
        let bsfile = fake_bsfile(e);
        e.set(file, TESFile::m_pFile, bsfile);
        e.set(file, TESFile::m_filesize, 0x1000);
        set_tag(e, file, TAG_RECORD, 0x20, 0);
        file
    }

    #[test]
    fn next_form_reads_one_record() {
        let mut e = engine();
        let file = stream_file(&mut e, &[0x1000, 0]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_2150, &args![file, 0u8]).bool());
        let log = e.call_log.take().unwrap();
        // 0x18 + 0x20 bytes on.
        assert_eq!(e.get(file, TESFile::m_fileoffset), 0x38);
        assert_eq!(calls_to(&log, READ_FORM_HEADER).len(), 1);
        assert_eq!(calls_to(&log, FREE_DECOMPRESSED_FORM).len(), 1);
        // The record has the skip bit but nobody asked.
        assert_eq!(e.get(current_form(file), Form::flags), 0x1000);
    }

    #[test]
    fn next_form_can_skip_records_with_the_skip_bit() {
        let mut e = engine();
        let file = stream_file(&mut e, &[0x1000, 0x1000, 0]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_2150, &args![file, 1u8]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, READ_FORM_HEADER).len(), 3);
        assert_eq!(e.get(current_form(file), Form::flags), 0);
        assert_eq!(e.get(file, TESFile::m_fileoffset), 0x38 * 3);
    }

    #[test]
    fn next_form_stops_at_the_end_of_the_file() {
        let mut e = engine();
        let file = stream_file(&mut e, &[0]);
        e.set(file, TESFile::m_fileoffset, 0xfd0);
        assert!(!e.call(0x0047_2150, &args![file, 1u8]).bool());
        assert_eq!(e.get(current_form(file), Form::form), 0);
    }

    #[test]
    fn advancing_over_a_header_only_tag_skips_just_the_header() {
        let mut e = engine();
        let file = stream_file(&mut e, &[0]);
        e.set(file, TESFile::m_fileoffset, 0x100);
        set_tag(&mut e, file, TAG_HEADER_ONLY, 0x500, 0);
        assert!(e.call(0x0047_21d0, &args![file]).bool());
        assert_eq!(e.get(file, TESFile::m_fileoffset), 0x118);
        set_tag(&mut e, file, TAG_OTHER_HEADER_ONLY, 0x500, 0);
        assert!(e.call(0x0047_21d0, &args![file]).bool());
        assert_eq!(e.get(file, TESFile::m_fileoffset), 0x130);
    }

    #[test]
    fn advancing_seeks_and_reads_the_next_header() {
        let mut e = engine();
        let file = stream_file(&mut e, &[0]);
        e.set(file, TESFile::m_fileoffset, 0x100);
        e.set(file, TESFile::m_formoffset, 5);
        e.set(file, TESFile::m_chunkoffset, 5);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_21d0, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        let bsfile = e.get(file, TESFile::m_pFile);
        assert_eq!(e.get(file, TESFile::m_fileoffset), 0x138);
        assert_eq!(
            calls_to(&log, FAKE_SEEK),
            vec![vec![bsfile.addr(), 0x138, 0]]
        );
        assert_eq!(e.get(file, TESFile::m_formoffset), 0);
        assert_eq!(e.get(file, TESFile::m_chunkoffset), 0);
    }

    #[test]
    fn advancing_does_not_seek_when_the_file_is_already_there() {
        let mut e = engine();
        let file = stream_file(&mut e, &[0]);
        let bsfile = e.get(file, TESFile::m_pFile);
        e.mem.set_u32(bsfile.addr() + 0x150, 0x38);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_21d0, &args![file]).bool());
        assert!(calls_to(&e.call_log.take().unwrap(), FAKE_SEEK).is_empty());
    }

    #[test]
    fn advancing_reports_a_failed_seek() {
        let mut e = engine();
        let file = stream_file(&mut e, &[0]);
        e.mem.set_u32(CONFIG_SEEK_RESULT, u32::MAX);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_21d0, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        let complaint = calls_to(&log, LOG);
        assert_eq!(complaint[0][0], MESSAGE_NEXT_FORM_SEEK_FAILED);
        assert_eq!(e.mem.cstr(complaint[0][1]), b"boom");
        assert_eq!(calls_to(&log, LOCAL_FREE), vec![vec![complaint[0][1]]]);
        let format = calls_to(&log, FORMAT_MESSAGE);
        assert_eq!(&format[0][..4], &[0x1300, 0, 5, 0x400]);
    }

    #[test]
    fn advancing_reports_a_record_that_cannot_be_read() {
        let mut e = engine();
        e.register(READ_FORM_HEADER, |_, _| ret(0));
        let file = new_file(&mut e);
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        e.set(file, TESFile::m_filesize, 0x1000);
        set_name(&mut e, file, b"Data\\", b"Mod.esp");
        set_tag(&mut e, file, TAG_RECORD, 0x20, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_21d0, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOG),
            vec![vec![MESSAGE_NEXT_FORM_BAD_FORM, file.addr() + 0x20, 0x38]]
        );
    }

    #[test]
    fn set_offset_seeks_and_loads_the_record_there() {
        let mut e = engine();
        e.register(READ_FORM_HEADER, |e, a| {
            e.mem.set_u32(a[0] + 0x240, TAG_RECORD);
            ret(1)
        });
        e.register(GET_TES_CHUNK, |_, _| Ret::default());
        e.register(TYPE_FROM_FORM_TAG, |_, _| ret(7));
        let file = new_file(&mut e);
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        e.set(file, TESFile::m_filesize, 0x1000);
        e.set(file, TESFile::m_formoffset, 8);
        e.set(file, TESFile::m_chunkoffset, 8);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_23a0, &args![file, 0x200u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(file, TESFile::m_fileoffset), 0x200);
        assert_eq!(
            calls_to(&log, FAKE_SEEK),
            vec![vec![bsfile.addr(), 0x200, 0]]
        );
        assert_eq!(e.get(file, TESFile::m_formoffset), 0);
        assert_eq!(e.get(current_form(file), Form::form), TAG_RECORD);
        assert_eq!(calls_to(&log, FREE_DECOMPRESSED_FORM).len(), 1);
    }

    #[test]
    fn set_offset_refuses_the_end_of_the_file() {
        let mut e = engine();
        let file = new_file(&mut e);
        e.set(file, TESFile::m_filesize, 0x1000);
        set_tag(&mut e, file, TAG_RECORD, 1, 2);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_23a0, &args![file, 0x1000u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FAKE_SEEK).is_empty());
        assert_eq!(e.get(current_form(file), Form::form), 0);
        assert_eq!(e.get(file, TESFile::m_fileoffset), 0x1000);
    }

    #[test]
    fn set_offset_reports_a_failed_seek() {
        let mut e = engine();
        let file = new_file(&mut e);
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        e.set(file, TESFile::m_filesize, 0x1000);
        e.mem.set_u32(CONFIG_SEEK_RESULT, u32::MAX);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_23a0, &args![file, 0x10u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOG)[0][0], MESSAGE_SET_OFFSET_SEEK_FAILED);
        assert_eq!(e.get(file, TESFile::m_fileoffset), u32::MAX);
    }

    #[test]
    fn cursor_offset_is_file_offset_plus_header_plus_form_offset() {
        let mut e = engine();
        let file = new_file(&mut e);
        e.set(file, TESFile::m_fileoffset, 0x100);
        e.set(file, TESFile::m_formoffset, 0x23);
        assert_eq!(e.call(0x0047_24c0, &args![file]).u32(), 0x13b);
    }

    #[test]
    fn set_offset_chunk_refuses_a_compressed_record() {
        let mut e = engine();
        let file = new_file(&mut e);
        set_tag(&mut e, file, TAG_RECORD, 0x100, RECORD_FLAG_COMPRESSED);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_24f0, &args![file, 0x120u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOG),
            vec![vec![MESSAGE_SET_OFFSET_CHUNK_COMPRESSED]]
        );
        assert!(calls_to(&log, FAKE_SEEK).is_empty());
    }

    #[test]
    fn set_offset_chunk_moves_inside_the_record() {
        let mut e = engine();
        e.register(GET_TES_CHUNK, |_, _| Ret::default());
        let file = new_file(&mut e);
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        e.set(file, TESFile::m_fileoffset, 0x100);
        e.set(file, TESFile::m_chunkoffset, 3);
        e.set(file, TESFile::m_currentchunkID, 9);
        set_tag(&mut e, file, TAG_RECORD, 0x100, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_24f0, &args![file, 0x150u32]).bool());
        let log = e.call_log.take().unwrap();
        // 0x150 - 0x100 - 0x18.
        assert_eq!(e.get(file, TESFile::m_formoffset), 0x38);
        assert_eq!(
            calls_to(&log, FAKE_SEEK),
            vec![vec![bsfile.addr(), 0x150, 0]]
        );
        assert_eq!(e.get(file, TESFile::m_chunkoffset), 0);
        assert_eq!(e.get(file, TESFile::m_currentchunkID), 0);
        assert_eq!(calls_to(&log, GET_TES_CHUNK).len(), 1);
    }

    #[test]
    fn set_offset_chunk_refuses_an_offset_outside_the_record() {
        let mut e = engine();
        let file = new_file(&mut e);
        e.set(file, TESFile::m_fileoffset, 0x100);
        set_tag(&mut e, file, TAG_RECORD, 0x20, 0);
        e.set(file, TESFile::m_currentchunkID, 9);
        e.call_log = Some(vec![]);
        // 0x100 + 0x18 + 0x20 is one past the last byte.
        assert!(!e.call(0x0047_24f0, &args![file, 0x138u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FAKE_SEEK).is_empty());
        assert_eq!(e.get(current_form(file), Form::form), 0);
        assert_eq!(e.get(file, TESFile::m_currentchunkID), 0);
        // An offset before the record wraps and is refused too.
        set_tag(&mut e, file, TAG_RECORD, 0x20, 0);
        assert!(!e.call(0x0047_24f0, &args![file, 0x10u32]).bool());
    }

    #[test]
    fn set_offset_chunk_reports_a_failed_seek() {
        let mut e = engine();
        let file = new_file(&mut e);
        let bsfile = fake_bsfile(&mut e);
        e.set(file, TESFile::m_pFile, bsfile);
        set_tag(&mut e, file, TAG_RECORD, 0x100, 0);
        e.mem.set_u32(CONFIG_SEEK_RESULT, u32::MAX);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_24f0, &args![file, 0x20u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOG)[0][0],
            MESSAGE_SET_OFFSET_CHUNK_SEEK_FAILED
        );
    }

    #[test]
    fn form_without_return_loads_header_and_chunk_only_when_empty() {
        let mut e = engine();
        e.register(READ_FORM_HEADER, |e, a| {
            e.mem.set_u32(a[0] + 0x240, TAG_RECORD);
            ret(1)
        });
        e.register(READ_CHUNK_HEADER, |_, _| Ret::default());
        let file = new_file(&mut e);
        e.set(file, TESFile::m_currentchunkID, 5);
        e.call_log = Some(vec![]);
        e.call(0x0047_2620, &args![file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(current_form(file), Form::form), TAG_RECORD);
        assert_eq!(e.get(file, TESFile::m_currentchunkID), 0);
        assert_eq!(calls_to(&log, READ_CHUNK_HEADER).len(), 1);
        // Already loaded: nothing is read.
        e.call_log = Some(vec![]);
        e.call(0x0047_2620, &args![file]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn form_without_return_stops_when_the_header_cannot_be_read() {
        let mut e = engine();
        e.register(READ_FORM_HEADER, |_, _| ret(0));
        e.register(READ_CHUNK_HEADER, |_, _| Ret::default());
        let file = new_file(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0047_2620, &args![file]);
        assert!(calls_to(&e.call_log.take().unwrap(), READ_CHUNK_HEADER).is_empty());
    }

    #[test]
    fn form_type_comes_from_the_record_tag() {
        let mut e = engine();
        e.register(TYPE_FROM_FORM_TAG, |_, a| ret(a[0] & 0xff));
        e.register(READ_FORM_HEADER, |e, a| {
            e.mem.set_u32(a[0] + 0x240, TAG_RECORD);
            ret(1)
        });
        e.register(GET_TES_CHUNK, |_, _| Ret::default());
        let file = new_file(&mut e);
        // Already loaded.
        set_tag(&mut e, file, 0x1234_5603, 0, 0);
        assert_eq!(e.call(0x0047_2660, &args![file]).u32(), 3);
        // Not loaded: read first.
        e.set(current_form(file), Form::form, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_2660, &args![file]).u32(), TAG_RECORD & 0xff);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), GET_TES_CHUNK).len(),
            1
        );
    }

    #[test]
    fn form_type_is_zero_when_no_record_can_be_read() {
        let mut e = engine();
        e.register(READ_FORM_HEADER, |_, _| ret(0));
        e.register(TYPE_FROM_FORM_TAG, |_, _| ret(99));
        let file = new_file(&mut e);
        assert_eq!(e.call(0x0047_2660, &args![file]).u32(), 0);
    }

    // ---- Second session: GetTESChunk to the end of the writer code. ----

    use std::cell::RefCell;
    use std::rc::Rc;

    /// The vtable of the fake `TESForm` objects, and the addresses of the
    /// doubles the tests put into its slots.
    const FORM_VTABLE: u32 = 0x0200_2000;
    const FAKE_SLOT_A: u32 = 0x7100_0100;
    const FAKE_SLOT_B: u32 = 0x7100_0110;
    const FAKE_SLOT_C: u32 = 0x7100_0120;
    const FAKE_SLOT_D: u32 = 0x7100_0130;
    /// Tag a test gives group headers.
    const TAG_GROUP: u32 = TAG_HEADER_ONLY;

    /// `engine()` plus the pages and getters the second session reads.
    fn engine2() -> Engine {
        let mut e = engine();
        for page in [0x011c_5000, 0x011d_e000, FORM_VTABLE] {
            e.map(page, 0x1000);
        }
        e.set_global(SEEK_FROM_END, 2u32);
        e.set_global(HIGHEST_HEADER_VERSION, 1.34f32 as f64);
        e.register(MUST_ENDIAN_CONVERT, |e, a| {
            ret(e.mem.u8(a[0] + 0x299) as u32)
        });
        e.register(CHUNK_HEADER_ENDIAN, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id.swap_bytes());
            let size = e.mem.u16(a[0] + 4);
            e.mem.set_u16(a[0] + 4, size.swap_bytes());
            Ret::default()
        });
        e.register(FILE_DIRECTORY, |_, a| ret(a[0] + 0x124));
        e.register(TYPE_FROM_FORM_TAG, |_, _| ret(99));
        e
    }

    /// A fake `TESForm`: an object with the fake vtable whose slots (byte
    /// offsets) hold the given function addresses.
    fn form_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr {
        for (offset, function) in slots {
            e.mem.set_u32(FORM_VTABLE + offset, *function);
        }
        let form = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(form.addr(), FORM_VTABLE);
        form
    }

    /// A file with a fake open `BSFile`.
    fn open_file(e: &mut Engine) -> (Ptr<TESFile>, Ptr) {
        let file = new_file(e);
        let bsfile = fake_bsfile(e);
        e.set(file, TESFile::m_pFile, bsfile);
        (file, bsfile)
    }

    /// A `BSFile::ReadF` double serving `data` in order.
    fn serve_reads(e: &mut Engine, data: Vec<u8>) {
        let mut position = 0usize;
        e.register_double(BSFILE_READ, move |e, a| {
            let count = (a[2] as usize).min(data.len() - position);
            e.mem.write(a[1], &data[position..position + count]);
            position += count;
            ret(count as u32)
        });
    }

    /// A `BSFile` write double that keeps what each write contained and
    /// answers `answer` (or the full size when `u32::MAX`).
    fn log_writes(e: &mut Engine, answer: u32) -> Rc<RefCell<Vec<Vec<u8>>>> {
        let writes = Rc::new(RefCell::new(Vec::new()));
        let sink = writes.clone();
        e.register_double(BSFILE_WRITE, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 1);
            assert_eq!(a[4], 1);
            sink.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            ret(if answer == u32::MAX { a[2] } else { answer })
        });
        writes
    }

    /// Doubles that put a group head on the list: `AddHead(list; &item)`
    /// stores the item in the head node.
    fn list_add_head_double(e: &mut Engine) {
        e.register(LIST_ADD_HEAD, |e, a| {
            let item = e.mem.u32(a[1]);
            let old = e.mem.u32(a[0]);
            if old != 0 {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, old);
                let next = e.mem.u32(a[0] + 4);
                e.mem.set_u32(node + 4, next);
                e.mem.set_u32(a[0] + 4, node);
            }
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
    }

    /// Sets the size of the current chunk.
    fn set_chunk(e: &mut Engine, file: Ptr<TESFile>, size: u32) {
        e.set(file, TESFile::m_actualChunkSize, size);
    }

    /// Puts the list of group entries (addresses) into `m_grouplist`.
    fn set_groups(e: &mut Engine, file: Ptr<TESFile>, groups: &[u32]) {
        let head = build_list(e, groups);
        e.mem.write(file.addr() + 0x290, &e.mem.bytes(head, 8));
    }

    fn group_entry(e: &mut Engine, id: u32) -> u32 {
        let group = e.mem.alloc(0x1c);
        e.mem.set_u32(group, TAG_GROUP);
        e.mem.set_u32(group + 0x18, id);
        group
    }

    #[test]
    fn get_tes_chunk_reads_the_header_only_when_none_is_loaded() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        e.set(file, TESFile::m_currentchunkID, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_26b0, &args![file]).u32(), 0x1234);
        assert!(calls_to(&e.call_log.take().unwrap(), BSFILE_READ).is_empty());
        // None loaded and the read fails.
        e.set(file, TESFile::m_currentchunkID, 0);
        serve_reads(&mut e, vec![]);
        assert_eq!(e.call(0x0047_26b0, &args![file]).u32(), 0);
        // None loaded and the read works.
        serve_reads(&mut e, b"EDID\x05\x00".to_vec());
        assert_eq!(
            e.call(0x0047_26b0, &args![file]).u32(),
            u32::from_le_bytes(*b"EDID")
        );
        assert_eq!(e.get(file, TESFile::m_actualChunkSize), 5);
    }

    #[test]
    fn get_chunk_data_swaps_a_word_when_the_file_must_be_converted() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        set_chunk(&mut e, file, 4);
        serve_reads(&mut e, vec![1, 2, 3, 4]);
        let buffer: Ptr = Ptr::new(e.mem.alloc(8));
        assert!(e.call(0x0047_27f0, &args![file, buffer]).bool());
        assert_eq!(e.mem.u32(buffer.addr()), 0x0403_0201);
        // Converted.
        e.set(file, TESFile::bMustEndianConvert, true);
        e.set(file, TESFile::m_chunkoffset, 0);
        serve_reads(&mut e, vec![1, 2, 3, 4]);
        assert!(e.call(0x0047_27f0, &args![file, buffer]).bool());
        assert_eq!(e.mem.u32(buffer.addr()), 0x0102_0304);
    }

    #[test]
    fn get_chunk_data_ov2_swaps_a_halfword_when_the_file_must_be_converted() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        set_chunk(&mut e, file, 2);
        e.set(file, TESFile::bMustEndianConvert, true);
        serve_reads(&mut e, vec![1, 2]);
        let buffer: Ptr = Ptr::new(e.mem.alloc(8));
        assert!(e.call(0x0047_2840, &args![file, buffer]).bool());
        assert_eq!(e.mem.u16(buffer.addr()), 0x0102);
        // A failed read is the answer, and nothing is swapped.
        e.set(file, TESFile::m_chunkoffset, 0);
        e.set(file, TESFile::bMustEndianConvert, false);
        serve_reads(&mut e, vec![9]);
        e.mem.set_u16(buffer.addr(), 0);
        assert!(!e.call(0x0047_2840, &args![file, buffer]).bool());
    }

    #[test]
    fn chunk_data_of_an_empty_chunk_is_trivially_read() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        let buffer: Ptr = Ptr::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_2890, &args![file, buffer, 0u32]).bool());
        assert!(calls_to(&e.call_log.take().unwrap(), BSFILE_READ).is_empty());
    }

    #[test]
    fn chunk_data_reads_the_whole_chunk_from_the_file() {
        let mut e = engine2();
        let (file, bsfile) = open_file(&mut e);
        set_chunk(&mut e, file, 5);
        e.set(file, TESFile::m_fileoffset, 0x100);
        e.set(file, TESFile::m_formoffset, 0x10);
        // The cursor is not at the chunk's data: seek there first.
        e.set(file, TESFile::m_chunkoffset, 3);
        serve_reads(&mut e, b"hello".to_vec());
        let buffer: Ptr = Ptr::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_2890, &args![file, buffer, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        // 0x100 + 0x18 (record header) + 0x10 + 6 (chunk header) = 0x12E.
        assert_eq!(
            calls_to(&log, FAKE_SEEK),
            vec![vec![bsfile.addr(), 0x12e, 0]]
        );
        assert_eq!(e.mem.bytes(buffer.addr(), 5), b"hello");
        assert_eq!(e.get(file, TESFile::m_chunkoffset), 5);
    }

    #[test]
    fn chunk_data_reports_a_short_read() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        set_chunk(&mut e, file, 5);
        serve_reads(&mut e, b"hel".to_vec());
        let buffer: Ptr = Ptr::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_2890, &args![file, buffer, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        let complaint = calls_to(&log, LOG);
        assert_eq!(complaint[0][0], MESSAGE_CHUNK_SECOND_READ_FAILED);
        assert_eq!(e.get(file, TESFile::m_chunkoffset), 3);
    }

    #[test]
    fn chunk_data_truncates_to_the_buffer_size() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        set_chunk(&mut e, file, 8);
        e.set(
            file,
            TESFile::m_currentchunkID,
            u32::from_le_bytes(*b"EDID"),
        );
        set_tag(&mut e, file, u32::from_le_bytes(*b"NPC_"), 0x40, 0);
        e.set(current_form(file), Form::iFormID, 0x0100_0007);
        serve_reads(&mut e, b"abcdefgh".to_vec());
        let buffer: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.write(buffer.addr(), &[0xee; 8]);
        // Keep what the report was given: the texts are on the stack.
        let seen = Rc::new(RefCell::new(None));
        let sink = seen.clone();
        e.register_double(LOG, move |e, a| {
            *sink.borrow_mut() = Some((
                a[0],
                a[1],
                e.mem.cstr(a[2]),
                e.mem.cstr(a[3]),
                a[4],
                a[5],
                a[6],
            ));
            Ret::default()
        });
        assert!(e.call(0x0047_2890, &args![file, buffer, 4u32]).bool());
        // Three bytes read and the fourth zeroed.
        assert_eq!(e.mem.bytes(buffer.addr(), 5), b"abc\0\xee");
        assert_eq!(e.get(file, TESFile::m_chunkoffset), 3);
        assert_eq!(
            seen.borrow().clone().unwrap(),
            (
                MESSAGE_CHUNK_TOO_BIG,
                8,
                b"EDID".to_vec(),
                b"NPC_".to_vec(),
                0x0100_0007,
                4,
                buffer.addr()
            )
        );
    }

    #[test]
    fn chunk_data_reports_a_short_read_of_a_truncated_chunk() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        set_chunk(&mut e, file, 8);
        serve_reads(&mut e, b"a".to_vec());
        let buffer: Ptr = Ptr::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_2890, &args![file, buffer, 4u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOG)[0][0], MESSAGE_CHUNK_FIRST_READ_FAILED);
    }

    #[test]
    fn chunk_data_of_a_compressed_record_comes_from_the_decompressed_form() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        set_tag(&mut e, file, TAG_RECORD, 0x40, RECORD_FLAG_COMPRESSED);
        let decompressed = e.mem.alloc(0x20);
        e.mem.write(decompressed + 0x16, b"world!");
        e.set(
            file,
            TESFile::pDecompressedFormBuffer,
            Ptr::new(decompressed),
        );
        e.set(file, TESFile::m_formoffset, 0x10);
        set_chunk(&mut e, file, 6);
        let buffer: Ptr = Ptr::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_2890, &args![file, buffer, 0u32]).bool());
        assert!(calls_to(&e.call_log.take().unwrap(), BSFILE_READ).is_empty());
        assert_eq!(e.mem.bytes(buffer.addr(), 6), b"world!");
        assert_eq!(e.get(file, TESFile::m_chunkoffset), 6);
        // Truncated: only max - 1 bytes are copied.
        let small: Ptr = Ptr::new(e.mem.alloc(8));
        e.register(LOG, |_, _| Ret::default());
        assert!(e.call(0x0047_2890, &args![file, small, 3u32]).bool());
        assert_eq!(e.mem.bytes(small.addr(), 3), b"wo\0");
    }

    /// 0x18 bytes of a record header.
    fn header_bytes(tag: u32, length: u32, flags: u32, id: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        for word in [tag, length, flags, id, 0, 0] {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        bytes
    }

    #[test]
    fn read_form_header_needs_an_open_file_and_a_whole_header() {
        let mut e = engine2();
        let file = new_file(&mut e);
        assert!(!e.call(0x0047_2bc0, &args![file]).bool());
        let (file, _) = open_file(&mut e);
        serve_reads(&mut e, vec![0x55; 5]);
        set_tag(&mut e, file, TAG_RECORD, 1, 1);
        assert!(!e.call(0x0047_2bc0, &args![file]).bool());
        // The header is zeroed.
        assert_eq!(e.mem.bytes(file.addr() + 0x240, 0x18), vec![0; 0x18]);
    }

    #[test]
    fn read_form_header_puts_the_compile_index_into_the_form_id() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        e.set(file, TESFile::cCompileIndex, 0x07);
        e.register(FORM_ID_NEEDS_STRIPPING, |_, a| {
            ret((a[0] == 0x00ab_cdef) as u32)
        });
        serve_reads(&mut e, header_bytes(TAG_RECORD, 0x20, 0, 0x0200_1234));
        assert!(e.call(0x0047_2bc0, &args![file]).bool());
        let form = current_form(file);
        assert_eq!(e.get(form, Form::form), TAG_RECORD);
        assert_eq!(e.get(form, Form::iFormID), 0x0700_1234);
        // The id check answers true: the index is stripped.
        serve_reads(&mut e, header_bytes(TAG_RECORD, 0x20, 0, 0x0100_abcd));
        serve_reads(&mut e, header_bytes(TAG_RECORD, 0x20, 0, 0x01ab_cdef));
        assert!(e.call(0x0047_2bc0, &args![file]).bool());
        assert_eq!(e.get(form, Form::iFormID), 0x00ab_cdef);
    }

    #[test]
    fn read_form_header_takes_the_index_of_the_master_a_form_id_names() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        e.set(file, TESFile::cCompileIndex, 0x07);
        e.set(file, TESFile::iMasterCount, 2);
        let master_a = new_file(&mut e);
        let master_b = new_file(&mut e);
        e.set(master_b, TESFile::cCompileIndex, 0x03);
        let table = e.mem.alloc(8);
        e.mem.set_u32(table, master_a.addr());
        e.mem.set_u32(table + 4, master_b.addr());
        e.set(file, TESFile::m_pMasterPtrs, Ptr::new(table));
        e.register(FORM_ID_NEEDS_STRIPPING, |_, _| ret(0));
        // Index byte 1 names the second master (counting this file as 0
        // then adding 1 to the byte).
        serve_reads(&mut e, header_bytes(TAG_RECORD, 0x20, 0, 0x0100_0042));
        assert!(e.call(0x0047_2bc0, &args![file]).bool());
        assert_eq!(e.get(current_form(file), Form::iFormID), 0x0300_0042);
        // A header-only tag keeps its id.
        serve_reads(&mut e, header_bytes(TAG_HEADER_ONLY, 0x20, 0, 0x0100_0042));
        assert!(e.call(0x0047_2bc0, &args![file]).bool());
        assert_eq!(e.get(current_form(file), Form::iFormID), 0x0100_0042);
    }

    #[test]
    fn read_form_header_converts_a_foreign_byte_order() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        e.set(file, TESFile::bMustEndianConvert, true);
        e.register(FORM_ID_NEEDS_STRIPPING, |_, _| ret(0));
        let mut swapped = header_bytes(TAG_RECORD, 0x20, 0, 0x0000_0042);
        for word in swapped.chunks_mut(4).take(5) {
            word.reverse();
        }
        swapped[0x14..0x16].reverse();
        serve_reads(&mut e, swapped);
        assert!(e.call(0x0047_2bc0, &args![file]).bool());
        let form = current_form(file);
        assert_eq!(e.get(form, Form::form), TAG_RECORD);
        assert_eq!(e.get(form, Form::length), 0x20);
    }

    #[test]
    fn read_chunk_header_reads_six_bytes_from_the_file() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        serve_reads(&mut e, b"EDID\x09\x00".to_vec());
        assert!(e.call(0x0047_2d30, &args![file]).bool());
        assert_eq!(
            e.get(file, TESFile::m_currentchunkID),
            u32::from_le_bytes(*b"EDID")
        );
        assert_eq!(e.get(file, TESFile::m_actualChunkSize), 9);
        // A short read clears the chunk.
        serve_reads(&mut e, b"ED".to_vec());
        assert!(!e.call(0x0047_2d30, &args![file]).bool());
        assert_eq!(e.get(file, TESFile::m_currentchunkID), 0);
        assert_eq!(e.get(file, TESFile::m_actualChunkSize), 0);
    }

    #[test]
    fn read_chunk_header_converts_a_foreign_byte_order() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        e.set(file, TESFile::bMustEndianConvert, true);
        serve_reads(&mut e, b"DIDE\x00\x09".to_vec());
        assert!(e.call(0x0047_2d30, &args![file]).bool());
        assert_eq!(
            e.get(file, TESFile::m_currentchunkID),
            u32::from_le_bytes(*b"EDID")
        );
        assert_eq!(e.get(file, TESFile::m_actualChunkSize), 9);
    }

    #[test]
    fn read_chunk_header_of_a_compressed_record_comes_from_the_buffer() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        set_tag(&mut e, file, TAG_RECORD, 0x40, RECORD_FLAG_COMPRESSED);
        let decompressed = e.mem.alloc(0x20);
        e.mem.write(decompressed + 4, b"DATA\x03\x00");
        e.set(
            file,
            TESFile::pDecompressedFormBuffer,
            Ptr::new(decompressed),
        );
        e.set(file, TESFile::m_formoffset, 4);
        e.register(DECOMPRESSED_FORM_SIZE, |_, _| ret(0x20));
        assert!(e.call(0x0047_2d30, &args![file]).bool());
        assert_eq!(
            e.get(file, TESFile::m_currentchunkID),
            u32::from_le_bytes(*b"DATA")
        );
        assert_eq!(e.get(file, TESFile::m_actualChunkSize), 3);
        // Past the end of the decompressed form.
        e.set(file, TESFile::m_formoffset, 0x20);
        assert!(!e.call(0x0047_2d30, &args![file]).bool());
        assert_eq!(e.get(file, TESFile::m_currentchunkID), 0);
    }

    #[test]
    fn read_chunk_header_of_an_xxxx_chunk_takes_the_size_from_its_data() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        serve_reads(&mut e, b"XXXX\x04\x00\x00\x01\x00\x00".to_vec());
        e.register(SKIP_TO_NEXT_CHUNK, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_2d30, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SKIP_TO_NEXT_CHUNK), vec![vec![file.addr()]]);
        assert_eq!(e.get(file, TESFile::m_actualChunkSize), 0x100);
    }

    #[test]
    fn start_form_fills_the_save_header_and_writes_it() {
        let mut e = engine2();
        let (file, bsfile) = open_file(&mut e);
        let form = form_with_slots(&mut e, &[]);
        e.register(FORM_TYPE_INDEX, |_, _| ret(1));
        e.register(FORM_FLAGS, |_, _| ret(0xffff_ffff));
        e.register(FORM_ID, |_, _| ret(0x0100_0abc));
        e.mem.set_u32(FORM_TAG_TABLE + 12, TAG_RECORD);
        e.mem.set_u32(CONFIG_SEEK_RESULT, 0x400);
        let writes = log_writes(&mut e, u32::MAX);
        e.call_log = Some(vec![]);
        e.call(0x0047_2e60, &args![file, form]);
        let log = e.call_log.take().unwrap();
        let save = file.at(TESFile::m_saveform);
        assert_eq!(e.get(save, Form::form), TAG_RECORD);
        assert_eq!(e.get(save, Form::length), 0);
        assert_eq!(e.get(save, Form::flags), 0x3003_2fe0);
        assert_eq!(e.get(save, Form::iFormID), 0x0100_0abc);
        assert_eq!(e.get(save, Form::iVersionControl), 0);
        assert_eq!(e.get(save, Form::sFormVersion), 0xf);
        assert_eq!(e.get(save, Form::sVCVersion), 0);
        assert_eq!(e.get(file, TESFile::m_saveformoffset), 0x400);
        assert_eq!(e.get(file, TESFile::m_savechunkoffset), 0);
        // Seeked from the end (whence 2), then wrote the 0x18-byte header.
        assert_eq!(calls_to(&log, FAKE_SEEK), vec![vec![bsfile.addr(), 0, 2]]);
        assert_eq!(writes.borrow().len(), 1);
        assert_eq!(writes.borrow()[0].len(), 0x18);
        assert_eq!(&writes.borrow()[0][..4], &TAG_RECORD.to_le_bytes());
    }

    #[test]
    fn load_form_sets_the_type_the_load_flags_and_the_file() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        set_tag(&mut e, file, TAG_RECORD, 0x20, 0);
        e.register_double(FAKE_SLOT_A, |_, _| Ret::default());
        let form = form_with_slots(&mut e, &[(0x128, FAKE_SLOT_A)]);
        e.register(FORM_SET_TYPE, |_, _| Ret::default());
        e.register(FORM_FILE_FLAGS, |_, _| ret(0x10));
        e.register(FORM_SET_LOAD_FLAGS, |_, _| Ret::default());
        e.register(RECORD_FORM_ID, |_, _| ret(0x77));
        e.register(FORM_SET_FILE, |_, _| Ret::default());
        e.register(FORM_IS_FLAGGED, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0047_2f60, &args![file, form]);
        let log = e.call_log.take().unwrap();
        // The type of the record (the double of `GetFormTypeFromFormString`
        // answers 99).
        assert_eq!(calls_to(&log, FORM_SET_TYPE), vec![vec![form.addr(), 99]]);
        assert_eq!(
            calls_to(&log, FORM_SET_LOAD_FLAGS),
            vec![vec![form.addr(), 0x4010]]
        );
        assert_eq!(calls_to(&log, RECORD_FORM_ID), vec![vec![file.addr()]]);
        assert_eq!(
            calls_to(&log, FAKE_SLOT_A),
            vec![vec![form.addr(), 0x77, 1]]
        );
        assert_eq!(
            calls_to(&log, FORM_SET_FILE),
            vec![vec![form.addr(), file.addr()]]
        );
        // A form the test does not flag gets the plain word.
        e.register(FORM_IS_FLAGGED, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0047_2f60, &args![file, form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FORM_SET_LOAD_FLAGS),
            vec![vec![form.addr(), 0x10]]
        );
    }

    #[test]
    fn add_tes_form_writes_the_saved_buffer_and_counts_the_form() {
        let mut e = engine2();
        let (file, bsfile) = open_file(&mut e);
        let form = form_with_slots(&mut e, &[]);
        e.register(FORM_TYPE_INDEX, |_, _| ret(1));
        e.register(FORM_FREE_FORM_BUFFER, |_, _| Ret::default());
        let buffer = e.mem.alloc(0x10);
        e.mem.write(buffer, b"0123456789abcdef");
        e.set_global(SAVE_BUFFER, buffer);
        e.set_global(SAVE_BUFFER_SIZE, 0x10u32);
        let writes = log_writes(&mut e, u32::MAX);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_2fe0, &args![file, form]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FAKE_SEEK), vec![vec![bsfile.addr(), 0, 2]]);
        assert_eq!(writes.borrow()[0], b"0123456789abcdef");
        assert_eq!(
            e.get(file.at(TESFile::fileHeaderInfo), FileHeader::iFormCount),
            1
        );
        assert_eq!(
            calls_to(&log, FORM_FREE_FORM_BUFFER),
            vec![vec![form.addr()]]
        );
        // No buffer: nothing is written or counted.
        e.set_global(SAVE_BUFFER, 0u32);
        assert_eq!(e.call(0x0047_2fe0, &args![file, form]).u32(), 0);
        assert_eq!(writes.borrow().len(), 1);
        assert_eq!(
            e.get(file.at(TESFile::fileHeaderInfo), FileHeader::iFormCount),
            1
        );
        // A short write is the answer.
        e.set_global(SAVE_BUFFER, buffer);
        log_writes(&mut e, 3);
        assert_eq!(e.call(0x0047_2fe0, &args![file, form]).u32(), 10);
    }

    #[test]
    fn the_save_buffer_getters_answer_the_globals() {
        let mut e = engine2();
        e.set_global(SAVE_BUFFER, 0x1234u32);
        e.set_global(SAVE_BUFFER_SIZE, 0x56u32);
        assert_eq!(e.call(0x0047_3070, &args![0u32]).u32(), 0x1234);
        assert_eq!(e.call(0x0047_3080, &args![0u32]).u32(), 0x56);
    }

    #[test]
    fn rewriting_the_save_header_seeks_back_and_writes_it_again() {
        let mut e = engine2();
        let (file, bsfile) = open_file(&mut e);
        e.set(file, TESFile::m_saveformoffset, 0x30);
        e.set(file, TESFile::m_savechunkoffset, 0x77);
        let save = file.at(TESFile::m_saveform);
        e.set(save, Form::form, TAG_RECORD);
        let writes = log_writes(&mut e, u32::MAX);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_3090, &args![file]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FAKE_SEEK),
            vec![vec![bsfile.addr(), 0x30, 0]]
        );
        assert_eq!(e.get(save, Form::length), 0x77);
        assert_eq!(
            &writes.borrow()[0][..8],
            &[0x52, 0x45, 0x52, 0x52, 0x77, 0, 0, 0]
        );
        assert_eq!(
            e.get(file.at(TESFile::fileHeaderInfo), FileHeader::iFormCount),
            1
        );
    }

    #[test]
    fn easy_write_reports_a_short_write() {
        let mut e = engine2();
        let (file, _) = open_file(&mut e);
        let data = e.mem.alloc(0x20);
        let writes = log_writes(&mut e, u32::MAX);
        assert_eq!(e.call(0x0047_3110, &args![file, data, 0x20u32]).u32(), 0);
        assert_eq!(writes.borrow()[0].len(), 0x20);
        assert_eq!(e.get(file, TESFile::m_lastError), 0);
        // Nothing to write is fine.
        assert_eq!(e.call(0x0047_3110, &args![file, 0u32, 0x20u32]).u32(), 0);
        assert_eq!(writes.borrow().len(), 1);
        // Short: error 10 and a report.
        log_writes(&mut e, 0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_3110, &args![file, data, 0x20u32]).u32(), 10);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOG), vec![vec![MESSAGE_WRITE_ERROR]]);
        assert_eq!(e.get(file, TESFile::m_lastError), 10);
    }

    #[test]
    fn bsfile_write_passes_a_count_of_one() {
        let mut e = engine2();
        let bsfile = fake_bsfile(&mut e);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(BSFILE_WRITE, move |e, a| {
            sink.borrow_mut().push((a.to_vec(), e.mem.u32(a[3])));
            ret(0x44)
        });
        assert_eq!(
            e.call(0x0047_3180, &args![bsfile, 0x1000u32, 0x44u32])
                .u32(),
            0x44
        );
        let seen = seen.borrow();
        assert_eq!(seen[0].0[..3], [bsfile.addr(), 0x1000, 0x44]);
        assert_eq!(seen[0].0[4], 1);
        assert_eq!(seen[0].1, 1);
    }

    #[test]
    fn the_top_byte_setters_keep_the_rest() {
        let mut e = engine2();
        let file = new_file(&mut e);
        e.set(file, TESFile::m_Flags, 0x1234_0011);
        e.call(0x0047_31c0, &args![file, 0x7fu32]);
        assert_eq!(e.get(file, TESFile::m_Flags), 0x7f34_0011);
        let header = file.at(TESFile::fileHeaderInfo);
        e.set(header, FileHeader::iNextFormID, 0x0100_0800);
        e.call(0x0047_3210, &args![file, 0x09u8]);
        assert_eq!(e.get(header, FileHeader::iNextFormID), 0x0900_0800);
        assert_eq!(e.get(file, TESFile::cCompileIndex), 9);
        assert_eq!(e.call(0x0047_3250, &args![file]).u8(), 9);
    }

    #[test]
    fn destroying_a_list_deletes_every_item_and_empties_it() {
        let mut e = engine2();
        e.register(LIST_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        let deleted = Rc::new(RefCell::new(Vec::new()));
        let sink = deleted.clone();
        e.register_double(SCALAR_DELETE, move |_, a| {
            sink.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let list = build_list(&mut e, &[0x111, 0, 0x222]);
        e.call(0x0047_3270, &args![list]);
        assert_eq!(*deleted.borrow(), vec![(0x111, 1), (0x222, 1)]);
        assert_eq!(e.mem.u32(list), 0);
    }

    #[test]
    fn the_group_list_accessors() {
        let mut e = engine2();
        let file = new_file(&mut e);
        assert_eq!(e.call(0x0047_32f0, &args![file]).u32(), file.addr() + 0x290);
        assert_eq!(e.call(0x0047_32d0, &args![file]).u32(), 0);
        set_groups(&mut e, file, &[0x1000, 0x2000]);
        assert_eq!(e.call(0x0047_32d0, &args![file]).u32(), 0x1000);
    }

    #[test]
    fn start_group_stores_a_copy_and_writes_its_header() {
        let mut e = engine2();
        let (file, bsfile) = open_file(&mut e);
        list_add_head_double(&mut e);
        let header = e.mem.alloc(0x18);
        e.mem.set_u32(header, TAG_GROUP);
        e.mem.set_u32(header + 4, 0x40);
        e.mem.set_u32(CONFIG_SEEK_RESULT, 0x600);
        let writes = log_writes(&mut e, u32::MAX);
        e.call_log = Some(vec![]);
        e.call(0x0047_3310, &args![file, header]);
        let log = e.call_log.take().unwrap();
        let group = Ptr::<FormGroup>::new(e.call(0x0047_32d0, &args![file]).u32());
        assert!(!group.is_null());
        assert_eq!(e.get(group.at(FormGroup::GroupData), Form::form), TAG_GROUP);
        assert_eq!(e.get(group.at(FormGroup::GroupData), Form::length), 0x40);
        assert_eq!(e.get(group, FormGroup::iGroupOffset), 0x600);
        assert_eq!(calls_to(&log, FAKE_SEEK), vec![vec![bsfile.addr(), 0, 2]]);
        assert_eq!(
            writes.borrow()[0][..8],
            [0x47, 0x52, 0x55, 0x50, 0x40, 0, 0, 0]
        );
        assert_eq!(
            e.get(file.at(TESFile::fileHeaderInfo), FileHeader::iFormCount),
            1
        );
        // A null header does nothing.
        e.call(0x0047_3310, &args![file, 0u32]);
        assert_eq!(writes.borrow().len(), 1);
    }

    #[test]
    fn start_group_without_a_file_only_pushes_the_group() {
        let mut e = engine2();
        let file = new_file(&mut e);
        list_add_head_double(&mut e);
        let header = e.mem.alloc(0x18);
        e.mem.set_u32(header, TAG_GROUP);
        e.call(0x0047_3310, &args![file, header]);
        assert_ne!(e.call(0x0047_32d0, &args![file]).u32(), 0);
        assert_eq!(
            e.get(file.at(TESFile::fileHeaderInfo), FileHeader::iFormCount),
            0
        );
    }

    #[test]
    fn end_group_stores_the_length_and_rewrites_the_header() {
        let mut e = engine2();
        let (file, bsfile) = open_file(&mut e);
        let group = group_entry(&mut e, 0x100);
        set_groups(&mut e, file, &[group]);
        e.mem.set_u32(CONFIG_SEEK_RESULT, 0x500);
        let writes = log_writes(&mut e, u32::MAX);
        e.call_log = Some(vec![]);
        e.call(0x0047_33a0, &args![file]);
        let log = e.call_log.take().unwrap();
        // End of the file at 0x500, the group started at 0x100.
        assert_eq!(
            calls_to(&log, FAKE_SEEK),
            vec![vec![bsfile.addr(), 0, 2], vec![bsfile.addr(), 0x100, 0]]
        );
        assert_eq!(writes.borrow()[0].len(), 0x18);
        assert_eq!(writes.borrow()[0][4..8], 0x400u32.to_le_bytes());
        // The entry is gone.
        assert_eq!(e.call(0x0047_32d0, &args![file]).u32(), 0);
        assert_eq!(e.mem.block_size(group), None);
        // No group: nothing happens.
        e.call(0x0047_33a0, &args![file]);
        assert_eq!(writes.borrow().len(), 1);
    }

    #[test]
    fn end_group_without_a_file_just_drops_the_entry() {
        let mut e = engine2();
        let file = new_file(&mut e);
        let group = group_entry(&mut e, 0x100);
        set_groups(&mut e, file, &[group]);
        e.call(0x0047_33a0, &args![file]);
        assert_eq!(e.call(0x0047_32d0, &args![file]).u32(), 0);
        assert_eq!(e.mem.block_size(group), None);
    }

    #[test]
    fn a_group_entry_is_a_copy_of_the_header_added_at_the_head() {
        let mut e = engine2();
        let file = new_file(&mut e);
        let added = Rc::new(RefCell::new(Vec::new()));
        let sink = added.clone();
        e.register_double(LIST_ADD_HEAD, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let header = e.mem.alloc(0x18);
        e.mem.write(header, &[7; 0x18]);
        e.call(0x0047_3440, &args![file, header]);
        let added = added.borrow();
        assert_eq!(added[0].0, file.addr() + 0x290);
        // A 0x1C-byte entry holding the 0x18 bytes of the header.
        assert!(e.mem.block_size(added[0].1).unwrap() >= 0x1c);
        assert_eq!(e.mem.bytes(added[0].1, 0x18), vec![7; 0x18]);
        drop(added);
        // A null header does nothing.
        e.call(0x0047_3440, &args![file, 0u32]);
    }

    #[test]
    fn removing_a_group_entry_drops_the_node_and_frees_it() {
        let mut e = engine2();
        let file = new_file(&mut e);
        // Nothing open: nothing happens.
        e.call(0x0047_3490, &args![file]);
        let first = group_entry(&mut e, 1);
        let second = group_entry(&mut e, 2);
        set_groups(&mut e, file, &[first, second]);
        e.call(0x0047_3490, &args![file]);
        assert_eq!(e.mem.block_size(first), None);
        assert_eq!(e.call(0x0047_32d0, &args![file]).u32(), second);
    }

    /// A file whose reads produce the given `(tag, form id)` records in turn
    /// (the last one forever), at the start of a 0x1000-byte stream.
    fn record_stream(e: &mut Engine, records: Vec<(u32, u32)>) -> Ptr<TESFile> {
        let mut next = 0;
        e.register_double(READ_FORM_HEADER, move |e, a| {
            let (tag, id) = records[next.min(records.len() - 1)];
            next += 1;
            e.mem.set_u32(a[0] + 0x240, tag);
            e.mem.set_u32(a[0] + 0x244, 0x20);
            e.mem.set_u32(a[0] + 0x248, 0);
            e.mem.set_u32(a[0] + 0x24c, id);
            ret((tag != 0) as u32)
        });
        e.register(READ_CHUNK_HEADER, |_, _| Ret::default());
        e.register(GET_TES_CHUNK, |_, _| Ret::default());
        let (file, _) = open_file(e);
        e.set(file, TESFile::m_filesize, 0x1000);
        e.set(file, TESFile::bHasGroups, true);
        file
    }

    #[test]
    fn find_form_refuses_what_cannot_be_searched() {
        let mut e = engine2();
        let form = form_with_slots(&mut e, &[(0x7c, FAKE_SLOT_A)]);
        e.register(FAKE_SLOT_A, |_, _| ret(1));
        let file = record_stream(&mut e, vec![(TAG_RECORD, 1)]);
        assert!(!e.call(0x0047_34d0, &args![file, 0u32]).bool());
        // No groups in the file.
        e.set(file, TESFile::bHasGroups, false);
        assert!(!e.call(0x0047_34d0, &args![file, form]).bool());
        e.set(file, TESFile::bHasGroups, true);
        // The file is not ready.
        let bsfile = e.get(file, TESFile::m_pFile);
        e.mem.set_u8(bsfile.addr() + 0x2c, 0);
        assert!(!e.call(0x0047_34d0, &args![file, form]).bool());
        e.mem.set_u8(bsfile.addr() + 0x2c, 1);
        // The form says it is in the file.
        assert!(e.call(0x0047_34d0, &args![file, form]).bool());
        // No file at all.
        e.set(file, TESFile::m_pFile, Ptr::NULL);
        assert!(!e.call(0x0047_34d0, &args![file, form]).bool());
    }

    #[test]
    fn find_form_scans_the_records_for_its_tag_and_id() {
        let mut e = engine2();
        let form = form_with_slots(
            &mut e,
            &[
                (0x7c, FAKE_SLOT_A),
                (0x8c, FAKE_SLOT_B),
                (0x110, FAKE_SLOT_C),
            ],
        );
        e.register(FAKE_SLOT_A, |_, _| ret(0));
        e.register(FAKE_SLOT_B, |_, _| ret(5));
        // The groups the form accepts are entered.
        e.register(FAKE_SLOT_C, |_, _| ret(1));
        e.register(FORM_ID, |_, _| ret(0x42));
        e.mem.set_u32(FORM_TAG_TABLE + 5 * 12, TAG_RECORD);
        let file = record_stream(
            &mut e,
            vec![
                (TAG_RECORD, 0x41),
                (TAG_GROUP, 0),
                (TAG_OTHER_HEADER_ONLY, 0),
                (TAG_RECORD, 0x42),
            ],
        );
        assert!(e.call(0x0047_34d0, &args![file, form]).bool());
        assert_eq!(e.get(current_form(file), Form::iFormID), 0x42);
    }

    #[test]
    fn find_form_skips_the_groups_the_form_refuses() {
        let mut e = engine2();
        let form = form_with_slots(
            &mut e,
            &[
                (0x7c, FAKE_SLOT_A),
                (0x8c, FAKE_SLOT_B),
                (0x110, FAKE_SLOT_C),
            ],
        );
        e.register(FAKE_SLOT_A, |_, _| ret(0));
        e.register(FAKE_SLOT_B, |_, _| ret(5));
        let asked = Rc::new(RefCell::new(Vec::new()));
        let sink = asked.clone();
        e.register_double(FAKE_SLOT_C, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(0)
        });
        e.register(FORM_ID, |_, _| ret(0x42));
        e.mem.set_u32(FORM_TAG_TABLE + 5 * 12, TAG_RECORD);
        let file = record_stream(&mut e, vec![(TAG_GROUP, 0), (TAG_RECORD, 0x42)]);
        assert!(e.call(0x0047_34d0, &args![file, form]).bool());
        // The group header was offered to slot +0x110 with (1, 0).
        let asked = asked.borrow();
        assert_eq!(asked[0][0], form.addr());
        assert_eq!(asked[0][1], file.addr() + 0x240);
        assert_eq!(asked[0][2..], [1, 0]);
    }

    #[test]
    fn find_form_gives_up_at_the_end_of_the_records() {
        let mut e = engine2();
        let form = form_with_slots(&mut e, &[(0x7c, FAKE_SLOT_A), (0x8c, FAKE_SLOT_B)]);
        e.register(FAKE_SLOT_A, |_, _| ret(0));
        e.register(FAKE_SLOT_B, |_, _| ret(5));
        e.register(FORM_ID, |_, _| ret(0x42));
        e.mem.set_u32(FORM_TAG_TABLE + 5 * 12, TAG_RECORD);
        e.register(LOG, |_, _| Ret::default());
        let file = record_stream(&mut e, vec![(TAG_RECORD, 0x41), (0, 0)]);
        assert!(!e.call(0x0047_34d0, &args![file, form]).bool());
    }

    #[test]
    fn has_groups_is_a_flag() {
        let mut e = engine2();
        let file = new_file(&mut e);
        assert!(!e.call(0x0047_3640, &args![file]).bool());
        e.set(file, TESFile::bHasGroups, true);
        assert!(e.call(0x0047_3640, &args![file]).bool());
    }

    #[test]
    fn skipping_a_group_header_moves_to_the_next_record() {
        let mut e = engine2();
        let file = record_stream(&mut e, vec![(TAG_GROUP, 0), (TAG_RECORD, 5)]);
        e.call(0x0047_2150, &args![file, 0u8]);
        set_tag(&mut e, file, TAG_GROUP, 0x50, 0);
        // The record is a group: its tag is cleared, the header size comes
        // off its length and the next record is read.
        assert!(e.call(0x0047_3660, &args![file]).bool());
        assert_eq!(e.get(current_form(file), Form::form), TAG_RECORD);
        assert_eq!(
            e.get(file, TESFile::m_fileoffset),
            0x18 + 0x18 + 0x50 - 0x18
        );
        // Anything else is left alone.
        assert!(!e.call(0x0047_3660, &args![file]).bool());
        assert_eq!(e.get(current_form(file), Form::form), TAG_RECORD);
    }

    /// Registers at `FAKE_SLOT_A` a form slot `+0x110` double that accepts the
    /// groups whose offset field is above `limit`.
    fn accept_groups_with_offset_above(e: &mut Engine, limit: u32) {
        e.register_double(FAKE_SLOT_A, move |e, a| {
            // (form; group, flag, third)
            ret((e.mem.u32(a[1] + 0x18) > limit) as u32)
        });
    }

    #[test]
    fn groups_for_a_form_are_left_alone_for_a_null_form_or_type_one() {
        let mut e = engine2();
        let file = new_file(&mut e);
        e.call(0x0047_36c0, &args![file, 0u32]);
        let form = form_with_slots(&mut e, &[]);
        e.register(FORM_TYPE_INDEX, |_, _| ret(1));
        e.call(0x0047_36c0, &args![file, form]);
    }

    #[test]
    fn groups_the_form_refuses_are_ended() {
        let mut e = engine2();
        let file = new_file(&mut e);
        let form = form_with_slots(&mut e, &[(0x110, FAKE_SLOT_A)]);
        e.register(FORM_TYPE_INDEX, |_, _| ret(5));
        accept_groups_with_offset_above(&mut e, 1);
        let inner = group_entry(&mut e, 1);
        let outer = group_entry(&mut e, 2);
        set_groups(&mut e, file, &[inner, outer]);
        e.call(0x0047_36c0, &args![file, form]);
        // The inner group was ended, the outer one is accepted.
        assert_eq!(e.mem.block_size(inner), None);
        assert_eq!(e.call(0x0047_32d0, &args![file]).u32(), outer);
    }

    #[test]
    fn missing_groups_are_started() {
        let mut e = engine2();
        let file = new_file(&mut e);
        list_add_head_double(&mut e);
        let form = form_with_slots(&mut e, &[(0x110, FAKE_SLOT_A), (0x114, FAKE_SLOT_B)]);
        e.register(FORM_TYPE_INDEX, |_, _| ret(5));
        // Any group that exists is accepted.
        e.register(FAKE_SLOT_A, |_, _| ret(1));
        let built = Rc::new(RefCell::new(Vec::new()));
        let sink = built.clone();
        e.register_double(FAKE_SLOT_B, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[1], TAG_GROUP);
            Ret::default()
        });
        e.call(0x0047_36c0, &args![file, form]);
        // The form built one header (no group existed yet) and it was
        // started.
        assert_eq!(built.borrow().len(), 1);
        assert_eq!(built.borrow()[0][0], form.addr());
        assert_eq!(built.borrow()[0][2], 0);
        assert_ne!(e.call(0x0047_32d0, &args![file]).u32(), 0);
    }

    #[test]
    fn a_form_that_cannot_build_its_group_is_reported() {
        let mut e = engine2();
        let file = new_file(&mut e);
        let form = form_with_slots(
            &mut e,
            &[
                (0x110, FAKE_SLOT_A),
                (0x114, FAKE_SLOT_B),
                (0x130, FAKE_SLOT_C),
            ],
        );
        e.register(FORM_TYPE_INDEX, |_, _| ret(5));
        e.register(FAKE_SLOT_A, |_, _| ret(0));
        // The header it builds does not carry the group tag.
        e.register(FAKE_SLOT_B, |_, _| Ret::default());
        e.register(FAKE_SLOT_C, |_, _| ret(0x2222));
        e.register(FORM_TYPE_NAME, |_, _| ret(0x1111));
        e.register(FORM_ID, |_, _| ret(0xabc));
        e.call_log = Some(vec![]);
        e.call(0x0047_36c0, &args![file, form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOG),
            vec![vec![MESSAGE_CREATE_GROUP_FAILED, 0x1111, 0x2222, 0xabc]]
        );
    }

    #[test]
    fn close_all_open_groups_ends_or_drops_each_group() {
        let mut e = engine2();
        e.set_global(DATA_HANDLER, 0x1234u32);
        let (file, _) = open_file(&mut e);
        let first = group_entry(&mut e, 1);
        let second = group_entry(&mut e, 2);
        set_groups(&mut e, file, &[first, second]);
        let writes = log_writes(&mut e, u32::MAX);
        e.register(HANDLER_GROUPS_FLAG, |_, a| {
            assert_eq!(a[0], 0x1234);
            ret(1)
        });
        e.call(0x0047_3830, &args![file]);
        // Each group was ended properly: its header was rewritten.
        assert_eq!(writes.borrow().len(), 2);
        assert_eq!(e.call(0x0047_32d0, &args![file]).u32(), 0);
        // With the handler's byte clear the entries are just dropped.
        let third = group_entry(&mut e, 3);
        set_groups(&mut e, file, &[third]);
        e.register(HANDLER_GROUPS_FLAG, |_, _| ret(0));
        e.call(0x0047_3830, &args![file]);
        assert_eq!(writes.borrow().len(), 2);
        assert_eq!(e.call(0x0047_32d0, &args![file]).u32(), 0);
        assert_eq!(e.mem.block_size(third), None);
    }

    #[test]
    fn the_stub_answers_false() {
        let mut e = engine2();
        assert!(!e.call(0x0047_3880, &args![0u32]).bool());
    }

    #[test]
    fn a_header_version_above_1_34_is_reported() {
        let mut e = engine2();
        let file = new_file(&mut e);
        e.set_global(HIGHEST_HEADER_VERSION, 1.34f32 as f64);
        e.set(
            file.at(TESFile::fileHeaderInfo),
            FileHeader::fVersion,
            1.34f32,
        );
        e.register(HEADER_VERSION, |e, a| Ret {
            st0: e.mem.f32(a[0] + 0x3dc) as f64,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_38a0, &args![file]).bool());
        assert!(calls_to(&e.call_log.take().unwrap(), LOG).is_empty());
        e.set(
            file.at(TESFile::fileHeaderInfo),
            FileHeader::fVersion,
            1.5f32,
        );
        e.register(WARNING_COUNT, |_, _| Ret::default());
        e.register(SPRINTF, |e, a| {
            assert_eq!(a[1], MESSAGE_VERSION_TOO_HIGH);
            let name = e.mem.cstr(a[2]);
            let mut text = b"File ".to_vec();
            text.extend_from_slice(&name);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        let reported = Rc::new(RefCell::new(Vec::new()));
        let sink = reported.clone();
        e.register_double(LOG, move |e, a| {
            sink.borrow_mut().push(e.mem.cstr(a[0]));
            Ret::default()
        });
        set_name(&mut e, file, b"Data\\", b"New.esm");
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_38a0, &args![file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, WARNING_COUNT), vec![vec![0], vec![1]]);
        assert_eq!(*reported.borrow(), vec![b"File New.esm".to_vec()]);
    }

    #[test]
    fn the_decompressed_buffer_is_made_on_demand() {
        let mut e = engine2();
        let file = new_file(&mut e);
        e.register(DECOMPRESS_CURRENT_FORM, |e, a| {
            e.mem.set_u32(a[0] + 0x420, 0x5000);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_3930, &args![file]).u32(), 0x5000);
        assert_eq!(e.call(0x0047_3930, &args![file]).u32(), 0x5000);
        // Only the first call decompressed.
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), DECOMPRESS_CURRENT_FORM).len(),
            1
        );
    }

    #[test]
    fn freeing_the_decompressed_buffer_clears_it() {
        let mut e = engine2();
        let file = new_file(&mut e);
        fn_00473960(&mut e, file);
        let buffer = e.mem.alloc(0x40);
        e.set(file, TESFile::pDecompressedFormBuffer, Ptr::new(buffer));
        e.set(file, TESFile::iDecompressedFormBufferSize, 0x40);
        fn_00473960(&mut e, file);
        assert_eq!(e.mem.block_size(buffer), None);
        assert_eq!(e.get(file, TESFile::pDecompressedFormBuffer), Ptr::NULL);
        assert_eq!(e.get(file, TESFile::iDecompressedFormBufferSize), 0);
    }

    #[test]
    fn the_root_file_is_used_on_the_owner_thread() {
        let mut e = engine2();
        let root = new_file(&mut e);
        let child = new_file(&mut e);
        e.set(child, TESFile::pThreadSafeParent, root.cast());
        e.set_global(PROCEDURE_OWNER, 0x5555u32);
        e.register(CURRENT_THREAD, |_, _| ret(7));
        e.register(OWNER_THREAD, |_, a| {
            assert_eq!(a[0], 0x5555);
            ret(7)
        });
        // A file without parent is its own root.
        assert_eq!(e.call(0x0047_39b0, &args![root]).u32(), root.addr());
        assert_eq!(e.call(0x0047_39b0, &args![child]).u32(), root.addr());
    }

    #[test]
    fn another_thread_gets_the_per_thread_copy() {
        let mut e = engine2();
        let root = new_file(&mut e);
        let copy = new_file(&mut e);
        let map = e.mem.alloc(0x10);
        e.set(root, TESFile::pThreadSafeFileMap, Ptr::new(map));
        e.register(CURRENT_THREAD, |_, _| ret(8));
        e.register(OWNER_THREAD, |_, _| ret(7));
        let copy_address = copy.addr();
        e.register_double(THREAD_FILE_MAP_LOOKUP, move |e, a| {
            e.mem.set_u32(a[2], copy_address);
            ret(1)
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_39b0, &args![root]).u32(), copy.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, THREAD_FILE_MAP_LOOKUP)[0][..2], [map, 8]);
    }

    #[test]
    fn destroying_the_per_thread_copies_deletes_them_and_the_map() {
        let mut e = engine2();
        let file = new_file(&mut e);
        // Without a map nothing happens.
        e.call(0x0047_3a10, &args![file]);
        let map_vtable = 0x0200_2800;
        e.map(map_vtable, 0x100);
        e.mem.set_u32(map_vtable, FAKE_SLOT_D);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, map_vtable);
        e.set(file, TESFile::pThreadSafeFileMap, Ptr::new(map));
        e.register(THREAD_FILE_MAP_BEGIN, |_, _| ret(1));
        let copy = 0x6000u32;
        e.register_double(THREAD_FILE_MAP_NEXT, move |e, a| {
            // The position is cleared after the only entry.
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[3], copy);
            Ret::default()
        });
        e.register(RELEASE_FILE, |_, _| Ret::default());
        e.register(SCALAR_DELETE, |_, _| Ret::default());
        e.register(THREAD_FILE_MAP_CLEAR, |_, _| Ret::default());
        e.register(FAKE_SLOT_D, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_3a10, &args![file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RELEASE_FILE), vec![vec![copy]]);
        assert_eq!(calls_to(&log, SCALAR_DELETE), vec![vec![copy, 1]]);
        assert_eq!(calls_to(&log, THREAD_FILE_MAP_CLEAR), vec![vec![map]]);
        // The map is deleted through its virtual destructor with flag 1.
        assert_eq!(calls_to(&log, FAKE_SLOT_D), vec![vec![map, 1]]);
        assert_eq!(e.get(file, TESFile::pThreadSafeFileMap), Ptr::NULL);
    }

    #[test]
    fn a_missing_per_thread_copy_is_built_and_stored() {
        let mut e = engine2();
        e.register(CLOSE_ALL_GROUPS, |_, _| Ret::default());
        e.register(READ_HEADER_STATUS, |_, _| ret(0));
        e.register(BSFILE_READ, |e, a| {
            e.mem.set_u32(a[1], TAG_FILE_HEADER);
            ret(0x18)
        });
        e.register(READ_FORM_HEADER, |e, a| {
            e.mem.set_u32(a[0] + 0x240, TAG_RECORD);
            ret(1)
        });
        e.register(TYPE_FROM_FORM_TAG, |_, _| ret(7));
        e.register(GET_TES_CHUNK, |e, a| {
            e.mem.set_u32(a[0] + 0x258, CHUNK_ID_HEDR);
            Ret::default()
        });
        e.register(HANDLER_FILE_TABLE, |_, a| ret(a[0] + 0x210));
        e.register(RELEASE_FILE, |_, _| Ret::default());
        e.map(0x7000, 0x1000);
        e.set_global(DATA_HANDLER, 0x7000u32);
        let this = new_file(&mut e);
        set_name(&mut e, this, b"Data\\", b"Fallout.esm");
        e.set(this, TESFile::cCompileIndex, 5);
        e.set(this, TESFile::m_Flags, FLAG_MASTER);
        let map = e.mem.alloc(0x10);
        e.register_double(THREAD_FILE_MAP_CONSTRUCT, move |_, a| {
            assert_eq!(a[1], 0x25);
            ret(map)
        });
        let stored = Rc::new(RefCell::new(Vec::new()));
        let sink = stored.clone();
        e.register_double(THREAD_FILE_MAP_SET_AT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let copy = e.call(0x0047_3ae0, &args![this, 9u32]).ptr::<TESFile>();
        let log = e.call_log.take().unwrap();
        assert!(!copy.is_null());
        assert_ne!(copy, this);
        // A new file for the same name, flagged like this one, whose
        // parent is this file's root.
        assert_eq!(e.mem.cstr(copy.addr() + 0x20), b"Fallout.esm");
        assert_eq!(e.get(copy, TESFile::cCompileIndex), 5);
        assert_eq!(e.get(copy, TESFile::m_Flags) & FLAG_MASTER, FLAG_MASTER);
        assert_eq!(e.get(copy, TESFile::pThreadSafeParent), this.cast());
        assert!(!e.get(copy, TESFile::m_pFile).is_null());
        // The map was created, then the copy was stored under the thread.
        assert_eq!(e.get(this, TESFile::pThreadSafeFileMap), Ptr::new(map));
        assert_eq!(*stored.borrow(), vec![vec![map, 9, copy.addr()]]);
        assert_eq!(calls_to(&log, RELEASE_FILE), vec![vec![copy.addr()]]);
    }

    #[test]
    fn a_known_per_thread_copy_is_answered_without_building() {
        let mut e = engine2();
        let this = new_file(&mut e);
        let map = e.mem.alloc(0x10);
        e.set(this, TESFile::pThreadSafeFileMap, Ptr::new(map));
        e.register(THREAD_FILE_MAP_LOOKUP, |e, a| {
            e.mem.set_u32(a[2], 0x7777);
            ret(1)
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_3ae0, &args![this, 3u32]).u32(), 0x7777);
        assert!(calls_to(&e.call_log.take().unwrap(), BSFILE_CONSTRUCT).is_empty());
    }

    #[test]
    fn the_root_of_the_parent_chain() {
        let mut e = engine2();
        let root = new_file(&mut e);
        let middle = new_file(&mut e);
        let leaf = new_file(&mut e);
        e.set(middle, TESFile::pThreadSafeParent, root.cast());
        e.set(leaf, TESFile::pThreadSafeParent, middle.cast());
        assert_eq!(e.call(0x0047_3c70, &args![leaf]).u32(), root.addr());
        assert_eq!(e.call(0x0047_3c70, &args![middle]).u32(), root.addr());
        // No parent: none.
        assert_eq!(e.call(0x0047_3c70, &args![root]).u32(), 0);
    }

    #[test]
    fn a_parent_is_always_stored_as_the_root() {
        let mut e = engine2();
        let root = new_file(&mut e);
        let middle = new_file(&mut e);
        let this = new_file(&mut e);
        e.set(middle, TESFile::pThreadSafeParent, root.cast());
        e.call(0x0047_3cb0, &args![this, middle]);
        assert_eq!(e.get(this, TESFile::pThreadSafeParent), root.cast());
        e.call(0x0047_3cb0, &args![this, root]);
        assert_eq!(e.get(this, TESFile::pThreadSafeParent), root.cast());
    }

    #[test]
    fn the_buffer_size_is_reset_to_its_default() {
        let mut e = engine2();
        let file = new_file(&mut e);
        e.set(file, TESFile::m_uiBufferAllocSize, 5);
        e.call(0x0047_3ce0, &args![file, 0x99u32]);
        assert_eq!(e.get(file, TESFile::m_uiBufferAllocSize), 0x4000);
    }

    #[test]
    fn the_always_true_method_ignores_its_arguments() {
        let mut e = engine2();
        assert!(e.call(0x0047_3d00, &args![0u32, 1u32, 2u32]).bool());
    }

    // Tests of the third session (`00473d20` to `00474880`).

    /// The vtable of the fake global map: slot 0 is the deleting destructor.
    const MAP_VTABLE: u32 = 0x0200_2000;
    const MAP_DESTRUCTOR: u32 = 0x7100_0100;
    /// Where the tests keep what their doubles saw.
    const SEEN: u32 = CONFIG + 0x100;

    /// An engine with the page of the global map pointer, the scope guard
    /// and the allocation callees doubled.
    fn engine3() -> Engine {
        let mut e = engine2();
        for page in [0x011c_4000, MAP_VTABLE] {
            e.map(page, 0x1000);
        }
        e.register(SCOPE_GUARD_OPEN, |_, _| Ret::default());
        e.register(SCOPE_GUARD_CLOSE, |_, _| Ret::default());
        e.register(ALLOCATE_BLOCK, |e, a| ret(e.mem.alloc(a[0])));
        e.register(FREE_BLOCK, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(ALLOCATE_POINTERS, |e, a| ret(e.mem.alloc(a[0] * 4)));
        e.register(FREE_ARRAY_ELEMENTS, |e, a| e.call(FREE_BLOCK, &a[..1]));
        e.register(THREAD_FILE_MAP_CLEAR, |_, _| Ret::default());
        e.put_vtable(MAP_VTABLE, &[MAP_DESTRUCTOR]);
        e
    }

    /// Doubles for the key array: `SetAtGrow` stores the element the
    /// pointer points to; `00877a30` answers the slot's address.
    fn key_array_doubles(e: &mut Engine) {
        e.register(KEY_ARRAY_SET_AT_GROW, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let element = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], element);
            ret(a[1])
        });
        e.register(KEY_ARRAY_ELEMENT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            ret(base + 4 * a[1])
        });
    }

    /// A fake global map holding `entries` (key, file); iteration doubles
    /// walk them. Sets the global pointer to a map object with the fake
    /// vtable and returns it.
    fn global_map(e: &mut Engine, entries: Vec<(u32, u32)>) -> u32 {
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, MAP_VTABLE);
        e.set_global(GLOBAL_FILE_MAP, map);
        let walk = entries.clone();
        e.register_double(THREAD_FILE_MAP_BEGIN, move |_, _| {
            ret(if walk.is_empty() { 0 } else { 1 })
        });
        let walk = entries.clone();
        e.register_double(THREAD_FILE_MAP_NEXT, move |e, a| {
            let index = e.mem.u32(a[1]) as usize - 1;
            e.mem.set_u32(a[2], walk[index].0);
            e.mem.set_u32(a[3], walk[index].1);
            let next = if index + 1 < walk.len() { index + 2 } else { 0 };
            e.mem.set_u32(a[1], next as u32);
            Ret::default()
        });
        e.register_double(THREAD_FILE_MAP_LOOKUP, move |e, a| {
            match entries.iter().find(|(key, _)| *key == a[1]) {
                Some((_, file)) => {
                    e.mem.set_u32(a[2], *file);
                    ret(1)
                }
                None => ret(0),
            }
        });
        map
    }

    #[test]
    fn registered_bit_is_bit_five() {
        let mut e = engine3();
        let file = new_file(&mut e);
        assert!(!e.call(0x0047_3d70, &args![file]).bool());
        e.call(0x0047_3d20, &args![file, true]);
        assert_eq!(e.get(file, TESFile::m_Flags), 0x20);
        assert!(e.call(0x0047_3d70, &args![file]).bool());
        e.set(file, TESFile::m_Flags, 0xffff_ffff);
        e.call(0x0047_3d20, &args![file, false]);
        assert_eq!(e.get(file, TESFile::m_Flags), 0xffff_ffdf);
        assert!(!e.call(0x0047_3d70, &args![file]).bool());
    }

    #[test]
    fn unregistering_an_unregistered_file_does_nothing() {
        let mut e = engine3();
        let file = new_file(&mut e);
        e.set_global(GLOBAL_FILE_MAP_USERS, 3u32);
        e.call_log = Some(vec![]);
        e.call(0x0047_3d90, &args![file]);
        assert_eq!(e.global::<u32>(GLOBAL_FILE_MAP_USERS), 3);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn unregistering_the_last_file_deletes_the_map() {
        let mut e = engine3();
        let file = new_file(&mut e);
        e.set(file, TESFile::m_Flags, 0x20);
        let map = global_map(&mut e, vec![]);
        e.set_global(GLOBAL_FILE_MAP_USERS, 1u32);
        e.register(MAP_DESTRUCTOR, |e, a| {
            e.mem.set_u32(SEEN, a[0]);
            e.mem.set_u32(SEEN + 4, a[1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0047_3d90, &args![file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(file, TESFile::m_Flags), 0);
        assert_eq!(e.global::<u32>(GLOBAL_FILE_MAP_USERS), 0);
        assert_eq!(e.global::<u32>(GLOBAL_FILE_MAP), 0);
        assert_eq!(calls_to(&log, THREAD_FILE_MAP_CLEAR), vec![vec![map]]);
        assert_eq!(e.mem.u32(SEEN), map);
        assert_eq!(e.mem.u32(SEEN + 4), 1);
        assert!(calls_to(&log, FILE_MAP_REMOVE_AT).is_empty());
    }

    #[test]
    fn unregistering_one_of_several_removes_only_its_keys() {
        let mut e = engine3();
        key_array_doubles(&mut e);
        e.register(FILE_MAP_REMOVE_AT, |_, _| Ret::default());
        let file = new_file(&mut e);
        let other = new_file(&mut e);
        e.set(file, TESFile::m_Flags, 0x20 | 0x01);
        let map = global_map(
            &mut e,
            vec![
                (11, file.addr()),
                (12, other.addr()),
                (13, file.addr()),
                (14, 0),
            ],
        );
        e.set_global(GLOBAL_FILE_MAP_USERS, 2u32);
        e.call_log = Some(vec![]);
        e.call(0x0047_3d90, &args![file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(file, TESFile::m_Flags), 0x01);
        assert_eq!(e.global::<u32>(GLOBAL_FILE_MAP_USERS), 1);
        assert_eq!(e.global::<u32>(GLOBAL_FILE_MAP), map);
        assert_eq!(
            calls_to(&log, FILE_MAP_REMOVE_AT),
            vec![vec![map, 11], vec![map, 13]]
        );
        assert!(calls_to(&log, THREAD_FILE_MAP_CLEAR).is_empty());
    }

    #[test]
    fn registering_creates_the_map_and_counts_the_file_once() {
        let mut e = engine3();
        let file = new_file(&mut e);
        e.register(THREAD_FILE_MAP_SET_AT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_3f20, &args![77u32, file]);
        let map = e.global::<u32>(GLOBAL_FILE_MAP);
        assert_ne!(map, 0);
        assert_eq!(e.mem.u32(map), VTABLE_MAP);
        assert_eq!(e.mem.u32(map + 4), 0x3e9);
        assert_ne!(e.mem.u32(map + 8), 0);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        assert_eq!(e.global::<u32>(GLOBAL_FILE_MAP_USERS), 1);
        assert!(e.call(0x0047_3d70, &args![file]).bool());
        // A second key for the same file: same map, same count.
        e.call(0x0047_3f20, &args![78u32, file]);
        assert_eq!(e.global::<u32>(GLOBAL_FILE_MAP), map);
        assert_eq!(e.global::<u32>(GLOBAL_FILE_MAP_USERS), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, THREAD_FILE_MAP_SET_AT),
            vec![vec![map, 77, file.addr()], vec![map, 78, file.addr()]]
        );
    }

    #[test]
    fn registering_a_second_file_counts_it() {
        let mut e = engine3();
        let first = new_file(&mut e);
        let second = new_file(&mut e);
        e.register(THREAD_FILE_MAP_SET_AT, |_, _| Ret::default());
        e.call(0x0047_3f20, &args![1u32, first]);
        e.call(0x0047_3f20, &args![2u32, second]);
        assert_eq!(e.global::<u32>(GLOBAL_FILE_MAP_USERS), 2);
    }

    #[test]
    fn key_check_passes_without_a_map_a_key_or_an_entry() {
        let mut e = engine3();
        let file = new_file(&mut e);
        // No map.
        assert!(e.call(0x0047_3ff0, &args![5u32, file]).bool());
        // A map, but key 0.
        global_map(&mut e, vec![(5, 0x1234)]);
        assert!(e.call(0x0047_3ff0, &args![0u32, file]).bool());
        // Unknown key, and a key that maps to 0.
        assert!(e.call(0x0047_3ff0, &args![6u32, file]).bool());
        global_map(&mut e, vec![(5, 0)]);
        assert!(e.call(0x0047_3ff0, &args![5u32, file]).bool());
    }

    #[test]
    fn key_check_compares_with_the_root_of_the_parent_chain() {
        let mut e = engine3();
        let root = new_file(&mut e);
        let leaf = new_file(&mut e);
        e.set(leaf, TESFile::pThreadSafeParent, root.cast());
        global_map(&mut e, vec![(5, root.addr())]);
        assert!(e.call(0x0047_3ff0, &args![5u32, leaf]).bool());
        global_map(&mut e, vec![(5, 0x9999)]);
        assert!(!e.call(0x0047_3ff0, &args![5u32, leaf]).bool());
        global_map(&mut e, vec![(5, root.addr())]);
        assert!(e.call(0x0047_3ff0, &args![5u32, root]).bool());
        global_map(&mut e, vec![(5, leaf.addr())]);
        assert!(!e.call(0x0047_3ff0, &args![5u32, root]).bool());
    }

    #[test]
    fn file_for_a_temporary_id() {
        let mut e = engine3();
        // No map: 0.
        assert_eq!(e.call(0x0047_4060, &args![5u32]).u32(), 0);
        global_map(&mut e, vec![(5, 0x4321)]);
        assert_eq!(e.call(0x0047_4060, &args![5u32]).u32(), 0x4321);
        assert_eq!(e.call(0x0047_4060, &args![6u32]).u32(), 0);
    }

    /// A file at the start of a compressed record of `length` bytes whose
    /// data is `data` (size word first).
    fn compressed_file(e: &mut Engine, length: u32, data: Vec<u8>) -> Ptr<TESFile> {
        let (file, _) = open_file(e);
        set_tag(e, file, TAG_RECORD, length, RECORD_FLAG_COMPRESSED);
        serve_reads(e, data);
        file
    }

    /// Doubles for zlib: `init` is what `inflateInit_` answers, `inflate`
    /// what `inflate` answers; `inflate` also writes `unpacked` to the
    /// output and copies the stream words to [`SEEN`].
    fn zlib_doubles(e: &mut Engine, init: i32, inflate: i32, unpacked: &'static [u8]) {
        e.register_double(INFLATE_INIT, move |e, a| {
            for i in 0..14 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(SEEN + 0x40 + 4 * i, word);
            }
            ret(init as u32)
        });
        e.register_double(INFLATE, move |e, a| {
            for i in 0..14 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(SEEN + 4 * i, word);
            }
            let output = e.mem.u32(a[0] + 0x0c);
            e.mem.write(output, unpacked);
            ret(inflate as u32)
        });
        e.register(INFLATE_END, |_, _| Ret::default());
    }

    #[test]
    fn decompressing_inflates_the_record_into_a_new_buffer() {
        let mut e = engine3();
        zlib_doubles(&mut e, 0, 1, b"hello");
        let mut data = 5u32.to_le_bytes().to_vec();
        data.extend_from_slice(&[0xaa; 8]);
        let file = compressed_file(&mut e, 12, data);
        e.call_log = Some(vec![]);
        e.call(0x0047_40a0, &args![file]);
        let log = e.call_log.take().unwrap();
        let buffer = e.get(file, TESFile::pDecompressedFormBuffer);
        assert!(!buffer.is_null());
        assert_eq!(e.get(file, TESFile::iDecompressedFormBufferSize), 5);
        assert_eq!(e.mem.bytes(buffer.addr(), 5), b"hello");
        // The stream as `inflate` saw it: next_in, avail_in, next_out,
        // avail_out, the allocator hooks and the version/size given to init.
        assert_eq!(e.mem.u32(SEEN + 4), 8);
        assert_eq!(e.mem.u32(SEEN + 0x0c), buffer.addr());
        assert_eq!(e.mem.u32(SEEN + 0x10), 5);
        assert_eq!(e.mem.u32(SEEN + 0x20), 0x0047_4460);
        assert_eq!(e.mem.u32(SEEN + 0x24), 0x0047_4480);
        assert_eq!(e.mem.u32(SEEN + 0x28), 0);
        assert_ne!(e.mem.u32(SEEN + 0x1c), 0);
        let scratch = e.mem.u32(SEEN) - 4;
        assert_eq!(e.mem.u8(scratch + 12), 0, "terminator after the data");
        let init = calls_to(&log, INFLATE_INIT);
        assert_eq!(init.len(), 1);
        assert_eq!(init[0][1..], [ZLIB_VERSION, 0x38]);
        assert_eq!(calls_to(&log, INFLATE_END).len(), 1);
        assert_eq!(calls_to(&log, INFLATE)[0][1], 0);
        assert_eq!(
            calls_to(&log, SCOPE_GUARD_OPEN)[0][1..],
            [0x30, 1, SOURCE_FILE_NAME, 0xf8e]
        );
        assert_eq!(calls_to(&log, SCOPE_GUARD_CLOSE).len(), 1);
        assert!(calls_to(&log, LOG).is_empty());
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![scratch]]);
    }

    #[test]
    fn decompressing_swaps_the_size_word_of_a_foreign_file() {
        let mut e = engine3();
        zlib_doubles(&mut e, 0, 1, b"hi");
        let mut data = 2u32.to_be_bytes().to_vec();
        data.extend_from_slice(&[0xaa; 4]);
        let file = compressed_file(&mut e, 8, data);
        e.set(file, TESFile::bMustEndianConvert, true);
        e.call(0x0047_40a0, &args![file]);
        assert_eq!(e.get(file, TESFile::iDecompressedFormBufferSize), 2);
        assert_eq!(e.mem.u32(SEEN + 4), 4);
    }

    #[test]
    fn decompressing_skips_records_that_are_not_compressed() {
        let mut e = engine3();
        zlib_doubles(&mut e, 0, 1, b"x");
        let file = compressed_file(&mut e, 12, vec![0; 12]);
        e.call_log = Some(vec![]);
        // No data.
        set_tag(&mut e, file, TAG_RECORD, 0, RECORD_FLAG_COMPRESSED);
        e.call(0x0047_40a0, &args![file]);
        // The header-only tag.
        set_tag(&mut e, file, TAG_HEADER_ONLY, 12, RECORD_FLAG_COMPRESSED);
        e.call(0x0047_40a0, &args![file]);
        // Not flagged compressed.
        set_tag(&mut e, file, TAG_RECORD, 12, 0);
        e.call(0x0047_40a0, &args![file]);
        // A tag without a form type.
        set_tag(&mut e, file, TAG_RECORD, 12, RECORD_FLAG_COMPRESSED);
        e.register(TYPE_FROM_FORM_TAG, |_, _| ret(0));
        e.call(0x0047_40a0, &args![file]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, BSFILE_READ).is_empty());
        assert!(e.get(file, TESFile::pDecompressedFormBuffer).is_null());
        // Every attempt still opened and closed the scope guard.
        assert_eq!(calls_to(&log, SCOPE_GUARD_OPEN).len(), 4);
        assert_eq!(calls_to(&log, SCOPE_GUARD_CLOSE).len(), 4);
    }

    #[test]
    fn decompressing_reports_a_short_read() {
        let mut e = engine3();
        zlib_doubles(&mut e, 0, 1, b"x");
        let file = compressed_file(&mut e, 12, vec![1, 0, 0, 0, 7]);
        e.call_log = Some(vec![]);
        e.call(0x0047_40a0, &args![file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOG),
            vec![vec![MESSAGE_COMPRESSED_READ_FAILED]]
        );
        assert!(calls_to(&log, INFLATE_INIT).is_empty());
        assert!(e.get(file, TESFile::pDecompressedFormBuffer).is_null());
        assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn decompressing_reports_zlib_failures_and_drops_the_buffer() {
        let cases: [(i32, i32, u32); 6] = [
            (-2, 0, MESSAGE_INFLATE_INIT_FAILED),
            (0, -2, MESSAGE_INFLATE_FAILED),
            (0, 2, MESSAGE_INFLATE_FAILED),
            (0, -3, MESSAGE_INFLATE_FAILED),
            (0, -4, MESSAGE_INFLATE_FAILED),
            (0, 0, MESSAGE_INFLATE_NOT_TERMINATED),
        ];
        for (init, inflate, message) in cases {
            let mut e = engine3();
            zlib_doubles(&mut e, init, inflate, b"x");
            let mut data = 5u32.to_le_bytes().to_vec();
            data.extend_from_slice(&[0; 8]);
            let file = compressed_file(&mut e, 12, data);
            e.call_log = Some(vec![]);
            e.call(0x0047_40a0, &args![file]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, LOG), vec![vec![message]], "{init} {inflate}");
            assert_eq!(calls_to(&log, INFLATE_END).len(), 1);
            assert!(e.get(file, TESFile::pDecompressedFormBuffer).is_null());
            assert_eq!(e.get(file, TESFile::iDecompressedFormBufferSize), 0);
            // The decompressed buffer and the scratch buffer were freed.
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), 2);
            assert_eq!(calls_to(&log, SCOPE_GUARD_CLOSE).len(), 1);
        }
    }

    #[test]
    fn zlib_hooks_use_the_memory_manager() {
        let mut e = engine3();
        e.call_log = Some(vec![]);
        let block = e.call(0x0047_4460, &args![0u32, 3u32, 8u32]).u32();
        assert_ne!(block, 0);
        e.call(0x0047_4480, &args![0u32, block]);
        let log = e.call_log.take().unwrap();
        let manager = e.call(MEMORY_MANAGER_GET, &args![]).u32();
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_ALLOCATE),
            vec![vec![manager, 24]]
        );
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![manager, block]]
        );
    }

    #[test]
    fn map_constructors_store_vtable_size_and_cleared_buckets() {
        let mut e = engine3();
        for (constructor, vtable, base_vtable) in [
            (0x0047_44a0u32, VTABLE_POINTER_MAP, VTABLE_POINTER_MAP_BASE),
            (0x0047_44d0, VTABLE_MAP, VTABLE_MAP_BASE),
        ] {
            let map = e.mem.alloc(0x10);
            assert_eq!(e.call(constructor, &args![map, 0x25u32]).u32(), map);
            assert_eq!(e.mem.u32(map), vtable);
            assert_eq!(e.mem.u32(map + 4), 0x25);
            assert_eq!(e.mem.u32(map + 0xc), 0);
            let buckets = e.mem.u32(map + 8);
            assert!(e.mem.bytes(buckets, 0x25 * 4).iter().all(|b| *b == 0));
            // The base constructors alone store the base vtable.
            let base = e.mem.alloc(0x10);
            let base_constructor = if vtable == VTABLE_MAP {
                0x0047_4660u32
            } else {
                0x0047_4560
            };
            e.call(base_constructor, &args![base, 7u32]);
            assert_eq!(e.mem.u32(base), base_vtable);
            assert_eq!(e.mem.u32(base + 4), 7);
        }
    }

    #[test]
    fn map_destructors_clear_and_free_the_buckets() {
        let mut e = engine3();
        for (constructor, destructor, base_destructor, vtable, base_vtable) in [
            (
                0x0047_44a0u32,
                0x0047_45d0u32,
                0x0047_4630u32,
                VTABLE_POINTER_MAP,
                VTABLE_POINTER_MAP_BASE,
            ),
            (
                0x0047_44d0,
                0x0047_46d0,
                0x0047_4730,
                VTABLE_MAP,
                VTABLE_MAP_BASE,
            ),
        ] {
            let map = e.mem.alloc(0x10);
            e.call(constructor, &args![map, 5u32]);
            let buckets = e.mem.u32(map + 8);
            e.call_log = Some(vec![]);
            e.call(destructor, &args![map]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(map), base_vtable);
            // The destructor clears the map, and so does the base destructor.
            assert_eq!(
                calls_to(&log, THREAD_FILE_MAP_CLEAR),
                vec![vec![map], vec![map]]
            );
            assert_eq!(calls_to(&log, FREE_BLOCK), vec![vec![buckets]]);
            // The base destructor alone.
            let other = e.mem.alloc(0x10);
            e.call(constructor, &args![other, 5u32]);
            e.mem.set_u32(other, vtable);
            e.call(base_destructor, &args![other]);
            assert_eq!(e.mem.u32(other), base_vtable);
        }
    }

    #[test]
    fn scalar_deleting_destructors_free_only_when_asked() {
        let mut e = engine3();
        for address in [0x0047_4500u32, 0x0047_4530, 0x0047_47c0, 0x0047_47f0] {
            let (constructor, hash) = if address == 0x0047_4500 || address == 0x0047_47c0 {
                (0x0047_44a0u32, 3u32)
            } else {
                (0x0047_44d0, 3)
            };
            let map = e.mem.alloc(0x10);
            e.call(constructor, &args![map, hash]);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![map, 0u32]).u32(), map);
            assert!(calls_to(&e.call_log.take().unwrap(), OPERATOR_DELETE).is_empty());
            e.call(constructor, &args![map, hash]);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![map, 1u32]).u32(), map);
            assert_eq!(
                calls_to(&e.call_log.take().unwrap(), OPERATOR_DELETE),
                vec![vec![map]]
            );
        }
    }

    #[test]
    fn key_array_constructors_and_destructor() {
        let mut e = engine3();
        let array = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x0047_4790, &args![array, 100u32, 50u32]).u32(),
            array
        );
        assert_eq!(e.mem.u32(array), VTABLE_KEY_ARRAY);
        let elements = e.mem.u32(array + 4);
        assert_ne!(elements, 0);
        assert_eq!(e.mem.u16(array + 8), 100);
        assert_eq!(e.mem.u16(array + 0xa), 0);
        assert_eq!(e.mem.u16(array + 0xc), 0);
        assert_eq!(e.mem.u16(array + 0xe), 50);
        // The base constructor: only the base vtable; the 16-bit fields
        // take the low half of the words.
        let base = e.mem.alloc(0x10);
        e.call(0x0047_4880, &args![base, 0x1_0004u32, 2u32]);
        assert_eq!(e.mem.u32(base), VTABLE_KEY_ARRAY_BASE);
        assert_eq!(e.mem.u16(base + 8), 4);
        // No room: no element block.
        let empty = e.mem.alloc(0x10);
        e.mem.set_u32(empty + 4, 0xdead);
        e.call(0x0047_4790, &args![empty, 0u32, 8u32]);
        assert_eq!(e.mem.u32(empty + 4), 0);
        // The destructors.
        e.call_log = Some(vec![]);
        e.call(0x0047_3f00, &args![array]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(array), VTABLE_KEY_ARRAY_BASE);
        assert_eq!(calls_to(&log, FREE_ARRAY_ELEMENTS), vec![vec![elements]]);
        e.call_log = Some(vec![]);
        e.call(0x0047_4760, &args![base]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), FREE_ARRAY_ELEMENTS).len(),
            1
        );
    }
}

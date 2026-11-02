//! `fallout shared/extradatalist.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! This unit holds `BaseExtraList` / `ExtraDataList` (the list of extra data
//! hung on a reference, a cell or an inventory entry), the small
//! `BSExtraData` subclasses whose constructors the compiler placed here, and
//! the very large `ExtraDataList::AddExtraCopy`, the save writer
//! (`fn_00412970`), `ExtraDataList::Load` and `ExtraDataList::InitItem`.
//! 624 functions in all: translated so far, in address order, `0040f680` to
//! `0041a540` (the first 120 the queue listed, in three sessions; the third
//! also did `004168a0`, the rotation matrix builder, which the first two
//! left, and the single-value getters and setters of `004181c0` to
//! `00418a80`; the fourth did the last 40 of the address range up to
//! `0041a6a0`, `00418ab0` to `0041a540`: the worn, seed, weapon mod,
//! animation, lock, teleport, marker, count, health, script, scale and sound
//! getters and setters, with the destructors of the data they replace).
//!
//! The functions from `0041a6a0` on belong to the other part files of the
//! unit. What a translator of this file needs is up here:
//!
//! - the layouts and the constants (the lock, the dirty counter, the thread
//!   cache, then, in the second block, the callees of the list operations,
//!   the save writer, the loader and `InitItem`) below;
//! - the type byte `cEtype` of an extra data is the index into the 21-byte
//!   bitmap `iFlags` of the list (`fn_0040fee0` sets and clears a bit,
//!   `base_extra_list_has_extra` tests it) and into the per-thread cache
//!   (`fn_0040f9e0`, `fn_0040fa40`);
//! - the extra data types are `EXTRA_DATA_TYPE` of the Xbox PDB (`0x03`
//!   `EXTRA_WATERTYPE`, `0x2A` `EXTRA_LOCK`, `0x53` `EXTRA_ACTIVATE_REF`,
//!   ...); the save writer and the loader name the chunks by their four
//!   letters (`chunk_tag(b"XCWT")`), and the tests build a list of extra
//!   data with `list_with_payloads` and read what a function did from the
//!   call log;
//! - where an identical-code fold put a wrong name on a function in the
//!   engine map (`BSSimpleList<REF_ACTIVATE_DATA_P>::AddHead` on `00414010`,
//!   `ExtraDataList::SetRadiation` on the rank setter `004198a0`,
//!   `D3DTexture_LockRect` on `0059bb30`, `VATS::GetCount` on `005ae380`),
//!   the function is `fn_<addr>` or the callee is named after its body.
//!
//! The compiler's exception-unwinding frames (the `FS:[0]` chains of the
//! constructors that build a `BSSimpleList`, of `AddExtraCopy`, of the save
//! writer, of `Load` and of `CopyListForReference`) are not translated.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleList;

// ---------------------------------------------------------------------------
// Layouts

layout! {
    /// `BSExtraData` (Xbox PDB), 0x0C bytes: vtable at +0 (slot 0 the scalar
    /// deleting destructor, slot 1 `Compare`), then the type and the next
    /// extra data of the list.
    pub struct BSExtraData: 0x0C {
        /// `cEtype` (Xbox PDB): the extra data type (`0x43` is
        /// `ExtraNorthRotation`, `0x65` `ExtraReflectedRefs`, ...).
        0x04 cEtype: u8,
        /// `pNext` (Xbox PDB): the next extra data of the list.
        0x08 pNext: Ptr,
    }

    /// `BaseExtraList` (Xbox PDB), 0x20 bytes: vtable at +0, the head of the
    /// chain of `BSExtraData`, and a bitmap of the types the chain holds.
    pub struct BaseExtraList: 0x20 {
        /// `pHead` (Xbox PDB): the first `BSExtraData`.
        0x04 pHead: Ptr,
        /// First byte of `iFlags` (Xbox PDB, `u8[21]`): bit `type & 7` of byte
        /// `type >> 3` says the chain holds an extra data of that type.
        0x08 iFlags: u8,
    }

    /// `ExtraDataList` (Xbox PDB): a `BaseExtraList` with its own vtable and
    /// no fields of its own.
    pub struct ExtraDataList: 0x20 {
        /// `pHead` (Xbox PDB), as in `BaseExtraList`.
        0x04 pHead: Ptr,
    }

    /// The statics of `BaseExtraList` that are `__declspec(thread)` in the PC
    /// build, as laid out in the TLS block (`e.tls()`): the last list looked
    /// up, the dirty counter it was valid for, and a cache of the extra data
    /// found in it by type. The Xbox PDB names these `pLastExtraList`,
    /// `iThreadDirty` and `ppLastExtraData`.
    pub struct BaseExtraListThreadCache: 0x25C {
        /// `pLastExtraList` (Xbox PDB).
        0x08 pLastExtraList: Ptr,
        /// `iThreadDirty` (Xbox PDB): the value of the global `iDirty` the
        /// cache was last cleared at.
        0x0C iThreadDirty: u32,
        /// `ppLastExtraData` (Xbox PDB): first of `0x93` pointers, one per
        /// extra data type, 0 for not cached.
        0x10 ppLastExtraData: Ptr,
    }

    /// `ExtraNorthRotation` (Xbox PDB), type `0x43`, 0x10 bytes.
    pub struct ExtraNorthRotation: 0x10 {
        /// `fNorthRot` (Xbox PDB).
        0x0C fNorthRot: f32,
    }

    /// `ExtraDetachTime` (Xbox PDB), 0x10 bytes: the 0x0B-type extra data
    /// `fn_0040f6b0` constructs has this shape (one 32-bit value at +0x0C).
    pub struct ExtraDetachTime: 0x10 {
        /// `iTime` (Xbox PDB).
        0x0C iTime: u32,
    }

    /// `ExtraReflectedRefs` (type `0x65`), `ExtraReflectorRefs` (`0x66`),
    /// `ExtraWaterLightRefs` (`0x84`) and `ExtraLitWaterRefs` (`0x85`)
    /// (Xbox PDB), 0x14 bytes each: a `BSExtraData` and the head node of a
    /// `BSSimpleList` of references.
    pub struct ExtraRefList: 0x14 {
        /// `RefList` (Xbox PDB): the inline head node (item, next).
        0x0C RefList: Inline<BSSimpleList>,
    }
}

// ---------------------------------------------------------------------------
// Constants

/// `BaseExtraList::ExtraCritSection` (Xbox PDB), the `BSSpinLock` that guards
/// every list operation.
pub(crate) const EXTRA_CRIT_SECTION: u32 = 0x011c_3920;
/// `BSSpinLock::Lock(const char *name)` on [`EXTRA_CRIT_SECTION`].
pub(crate) const LOCK: u32 = 0x0040_fbf0;
/// `BSSpinLock::Unlock()`.
pub(crate) const UNLOCK: u32 = 0x0040_fba0;
/// `BaseExtraList::iDirty` (Xbox PDB): bumped whenever a list loses an
/// extra data, so every thread's cache is dropped.
pub(crate) const DIRTY: u32 = 0x011c_38e4;
/// `BSExtraData::BSExtraData(type)`: sets the base vtable, the type, and a
/// null next. The subclass constructors call it, then set their own vtable.
pub(crate) const BS_EXTRA_DATA_INIT: u32 = 0x0040_ec80;
/// `cEtype` getter (`MOV AL,[ECX+4]`; the engine map files it under
/// `tesregiondata.cpp`, identical code folded).
pub(crate) const GET_TYPE: u32 = 0x004f_1540;
/// `pNext` getter (`MOV EAX,[ECX+8]`).
pub(crate) const GET_NEXT: u32 = 0x0044_ddc0;
/// `pNext` setter (`MOV [ECX+8],arg`).
pub(crate) const SET_NEXT: u32 = 0x0040_3550;
/// `MOV EAX,[ECX+4]`: `BSSimpleList`'s `m_pkNext`, and the `pHead` of a
/// `BaseExtraList` where `RemoveExtra_ov2` starts its walk.
pub(crate) const SIMPLE_LIST_NEXT: u32 = 0x0072_6070;
/// `MOV EAX,ECX`: a `BSSimpleList` node's address, which is the address of its
/// item slot (the item is read through the returned pointer).
pub(crate) const SIMPLE_LIST_ITEM: u32 = 0x0068_15c0;
/// `MOV EAX,[ECX]`: reads the word a handle points at.
pub(crate) const READ_WORD: u32 = 0x0055_9450;
/// `memset(dst, value, size)`.
pub(crate) const MEMSET: u32 = 0x0040_3d30;
/// `memcpy(dst, src, size)` (the game's wrapper).
pub(crate) const MEMCPY: u32 = 0x0040_1460;
/// `operator new(size)`.
pub(crate) const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(block)` (`platform`).
pub(crate) const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `__RTDynamicCast(object, vfDelta, srcType, targetType, isReference)`.
pub(crate) const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;

/// Size of the bitmap `iFlags`, in bytes.
pub(crate) const FLAGS_LEN: u32 = 0x15;
/// Entries of `ppLastExtraData`; also the first type that is not cached.
pub(crate) const CACHE_ENTRIES: u32 = 0x93;
/// Bytes of `ppLastExtraData` (`CACHE_ENTRIES * 4`).
pub(crate) const CACHE_BYTES: u32 = 0x24c;

/// Lock names the functions pass to [`LOCK`] (strings in the exe).
pub(crate) const NAME_REMOVE_ALL: u32 = 0x0101_4304;
pub(crate) const NAME_REMOVE_ALL_DEFAULT: u32 = 0x0101_4320;
pub(crate) const NAME_ITEMS_IN_LIST: u32 = 0x0101_4344;
pub(crate) const NAME_ADD_EXTRA: u32 = 0x0101_4364;
/// `"BaseExtraList::RemoveExtra()"`, used by both overloads.
pub(crate) const NAME_REMOVE_EXTRA: u32 = 0x0101_4380;
pub(crate) const NAME_GET_EXTRA_DATA: u32 = 0x0101_43a0;
pub(crate) const NAME_GET_PREV_EXTRA_DATA: u32 = 0x0101_43c0;
/// `"AI: No Copy function available for Extra Data type %i."`.
pub(crate) const NO_COPY_MESSAGE: u32 = 0x0101_43ec;
/// Reports a message through the game's log (`005b5e40`, cdecl, printf-like).
pub(crate) const LOG_MESSAGE: u32 = 0x005b_5e40;

/// Vtables set by the constructors of this unit.
pub(crate) const VTABLE_EXTRA_NORTH_ROTATION: u32 = 0x0101_42a0;
pub(crate) const VTABLE_EXTRA_DETACH_TIME: u32 = 0x0101_42ac;
pub(crate) const VTABLE_BASE_EXTRA_LIST: u32 = 0x0101_4300;
pub(crate) const VTABLE_EXTRA_DATA_LIST: u32 = 0x0101_43e8;
pub(crate) const VTABLE_EXTRA_REFLECTED_REFS: u32 = 0x0101_4428;
pub(crate) const VTABLE_EXTRA_REFLECTOR_REFS: u32 = 0x0101_4434;
pub(crate) const VTABLE_EXTRA_WATER_LIGHT_REFS: u32 = 0x0101_4440;
pub(crate) const VTABLE_EXTRA_LIT_WATER_REFS: u32 = 0x0101_444c;
pub(crate) const VTABLE_EXTRA_TYPE_92: u32 = 0x0101_4458;

// Second batch: lock names, callees and helpers (list operations, the save
// writer `fn_00412970`, the loader `extra_data_list_load`, `InitItem`).

/// `BSExtraData::~BSExtraData` (`0040ecb0`), run by the subclass destructors
/// after they set their own vtable.
pub(crate) const BS_EXTRA_DATA_DESTROY: u32 = 0x0040_ecb0;
/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 (a leading `f64`
/// argument here).
pub(crate) const FTOL: u32 = 0x00ec_62c0;

pub(crate) const NAME_COPY_LIST: u32 = 0x0101_4468;
pub(crate) const NAME_REMOVE_ALL_COPYABLE: u32 = 0x0101_4484;
pub(crate) const NAME_REMOVE_NON_PERSISTENT: u32 = 0x0101_44ac;
pub(crate) const NAME_COPY_LIST_FOR_CONTAINER: u32 = 0x0101_44dc;
pub(crate) const NAME_DUPLICATE_FOR_CONTAINER: u32 = 0x0101_4504;
pub(crate) const NAME_COPY_LIST_FOR_REFERENCE: u32 = 0x0101_4534;
pub(crate) const NAME_COMPARE_LIST_FOR_CONTAINER: u32 = 0x0101_455c;
pub(crate) const NAME_COMPARE_LIST: u32 = 0x0101_4588;

/// The global word `RemoveNonPersistentCellData` passes as `this` to
/// `fn_004121b0`, which does not use it.
pub(crate) const THREAD_STATE_OWNER: u32 = 0x011d_df38;
/// Offset in the TLS block of the word whose bit 2 `fn_004121b0` tests.
pub(crate) const THREAD_FLAGS_OFFSET: u32 = 0x294;

/// `ExtraDataList::GetScript` (`00418800`).
pub(crate) const GET_SCRIPT: u32 = 0x0041_8800;
/// `ExtraScript::ExtraScript(script)` (`00432000`, `this` = the new block).
pub(crate) const EXTRA_SCRIPT_INIT: u32 = 0x0043_2000;
/// `005abf60` on a script, whose result `00419f80` takes.
pub(crate) const SCRIPT_GET_RESULT: u32 = 0x005a_bf60;
/// `00419f80`: the list's setter for the script result (`AddExtraCopy` uses it
/// for the second word of a type `0D` extra data).
pub(crate) const EXTRA_DATA_LIST_SET_SCRIPT_RESULT: u32 = 0x0041_9f80;

/// `Swap32(pointer, 0)` (`00401080`): byte-swaps the word at `pointer` in
/// place (cdecl; the byte argument is not used).
pub(crate) const SWAP_DWORD: u32 = 0x0040_1080;
/// `Swap16(pointer, 0)` (`00407a90`).
pub(crate) const SWAP_WORD: u32 = 0x0040_7a90;

// The save writer's callees.
/// `TESForm::iFormID` getter (`MOV EAX,[ECX+0x0C]`).
pub(crate) const FORM_ID: u32 = 0x0084_e3a0;
/// `TESForm::AddChunk(tag, word)`: one word, swapped first on a big-endian
/// target.
pub(crate) const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// `TESForm::AddChunk(tag, byte)`.
pub(crate) const ADD_CHUNK_BYTE: u32 = 0x0048_58f0;
/// `TESForm::AddChunk(tag)`: a chunk without data.
pub(crate) const ADD_CHUNK_EMPTY: u32 = 0x0048_56d0;
/// `TESForm::__AddChunkData(tag, data, size)`.
pub(crate) const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `TESForm::AddChunkArray_ov2(tag, data, count)` (`00485710`).
pub(crate) const ADD_CHUNK_ARRAY: u32 = 0x0048_5710;
/// `TESForm::AddChunkArray(tag, data, size)` (`004856f0`).
pub(crate) const ADD_CHUNK_ARRAY_RAW: u32 = 0x0048_56f0;
/// The save writer's big-endian flag (`MOV AL,[011c54ba]`).
pub(crate) const IS_BIG_ENDIAN: u32 = 0x0040_1500;
/// Byte-swap of the word at `this` (`Swap32`, `00503210`).
pub(crate) const SWAP_WORD_AT: u32 = 0x0050_3210;
/// Byte-swap of the word at `this + 4` (`0060ce80`).
pub(crate) const SWAP_WORD_AT_4: u32 = 0x0060_ce80;
/// Byte-swap of the words at `this` and `this + 4` (`00462230`).
pub(crate) const SWAP_TWO_WORDS: u32 = 0x0046_2230;
/// Count of the non-null items of a `BSSimpleList` (`005ae380`).
pub(crate) const LIST_COUNT: u32 = 0x005a_e380;
/// `BSSimpleList::IsEmpty`: no item and no next (`008256d0`).
pub(crate) const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `RagDollData::Save` (`004d93c0`).
pub(crate) const RAGDOLL_DATA_SAVE: u32 = 0x004d_93c0;
/// The teleport data's save (`0043a420`).
pub(crate) const TELEPORT_DATA_SAVE: u32 = 0x0043_a420;
/// The map marker data's save (`00438c10`).
pub(crate) const MAP_MARKER_DATA_SAVE: u32 = 0x0043_8c10;
/// The `GetSeed` of the list (`00418b40`): the type `0x31` byte, 0xFF if none.
pub(crate) const EXTRA_DATA_LIST_GET_SEED: u32 = 0x0041_8b40;
/// Constructor of the decal data (`0055a400`).
pub(crate) const DECAL_DATA_INIT: u32 = 0x0055_a400;
/// `BSSimpleList` node constructor (`0096a2d0`): item and next to 0.
pub(crate) const SIMPLE_LIST_INIT: u32 = 0x0096_a2d0;
/// Copy of the 8-byte member of the navmesh portal data (`0069a690`).
pub(crate) const NAVMESH_PORTAL_COPY: u32 = 0x0069_a690;
/// `MultiBoundMarkerData` save (`00438f90`).
pub(crate) const MULTIBOUND_MARKER_DATA_SAVE: u32 = 0x0043_8f90;
/// `PackageEventAction::Save` (`0067dc80`).
pub(crate) const PACKAGE_EVENT_ACTION_SAVE: u32 = 0x0067_dc80;
/// `BSOcclusionPlane` size member getter: the address `this + 0x14`.
pub(crate) const PLANE_HALF_EXTENTS: u32 = 0x007d_6bb0;
/// `NiMatrix3` to axis and angle (`00a58550`): `(matrix, &angle, &x, &y, &z)`.
pub(crate) const MATRIX_TO_AXIS_ANGLE: u32 = 0x00a5_8550;
/// `AudioMarkerData` save (`00589560`).
pub(crate) const AUDIO_MARKER_DATA_SAVE: u32 = 0x0058_9560;
/// `AudioBuoyMarkerData` save (`00483710`).
pub(crate) const AUDIO_BUOY_MARKER_DATA_SAVE: u32 = 0x0048_3710;
/// `BSStringT::GetLength` (`004048e0`).
pub(crate) const STRING_LENGTH: u32 = 0x0040_48e0;
/// `ExtraDataList::GetActivateTextOverride(out)` (`0041ec80`): copies the
/// activate text of the type `0x53` extra data (or an empty string) into
/// `out` and returns `out`.
pub(crate) const GET_ACTIVATE_TEXT: u32 = 0x0041_ec80;
/// `BSStringT` destructor (`004037d0`).
pub(crate) const STRING_DESTROY: u32 = 0x0040_37d0;
/// Constructor of the package start location record (`006d5320`).
pub(crate) const PATH_LOCATION_INIT: u32 = 0x006d_5320;

// The loader's callees.
/// `TESFile::GetTESChunk`: the type of the current chunk (`004726b0`).
pub(crate) const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::GetChunkData(&value)`: reads the 4-byte value (`004727f0`).
pub(crate) const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData(buffer, size)` (`00472890`).
pub(crate) const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESFile::GetChunkDataSize` (`00401660`, `MOV EAX,[ECX+0x25C]`).
pub(crate) const GET_CHUNK_SIZE: u32 = 0x0040_1660;
/// The file's byte-swap flag (`00401680`, `MOV AL,[ECX+0x299]`).
pub(crate) const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
/// `TESFile::NextChunk`-like skip to the next chunk (`004726f0`).
pub(crate) const SKIP_CHUNK: u32 = 0x0047_26f0;
/// `TESFile` file name field address, `this + 0x20` (`00891170`).
pub(crate) const FILE_NAME: u32 = 0x0089_1170;
/// `TESForm::AddCompileIndex(&id, file)` (`00485d50`, cdecl).
pub(crate) const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The data handler singleton pointer (`011c3f2c`).
pub(crate) const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `DataHandler::pRegionList` getter (`004169d0`, `MOV EAX,[ECX+0x1D8]`).
pub(crate) const DATA_HANDLER_REGIONS: u32 = 0x0041_69d0;
/// `TESRegionList::TESRegionList(bool)` (`004f6320`).
pub(crate) const REGION_LIST_INIT: u32 = 0x004f_6320;
/// Region of a `TESRegionList` by form id (`004f66c0`).
pub(crate) const REGION_LIST_FIND: u32 = 0x004f_66c0;
/// Adds a region to a `TESRegionList` (`004f6600`).
pub(crate) const REGION_LIST_ADD: u32 = 0x004f_6600;
/// `ExtraDataList::SetRegionList` (`0041bbd0`).
pub(crate) const SET_REGION_LIST: u32 = 0x0041_bbd0;
/// `"MASTERFILE: Invalid Extra Data - Region List in file \"%s\"."`
pub(crate) const MESSAGE_BAD_REGION_LIST: u32 = 0x0101_45d8;
/// `"MASTERFILE: Failed to load RagDoll Data."`
pub(crate) const MESSAGE_BAD_RAGDOLL: u32 = 0x0101_45ac;
/// The float default of the third distant-data value (`010145a8`).
pub(crate) const LOD_DEFAULT: u32 = 0x0101_45a8;

pub(crate) const SET_HEALTH: u32 = 0x0041_9970;
pub(crate) const SET_USES: u32 = 0x0041_9a20;
pub(crate) const SET_COUNT: u32 = 0x0041_9ad0;
/// `ExtraDataList::GetAmmo` (`0042eb40`): the type `0x6E` extra data.
pub(crate) const GET_AMMO: u32 = 0x0042_eb40;
/// `ExtraDataList::SetAmmo(ammo, count)` (`0042eb60`).
pub(crate) const SET_AMMO: u32 = 0x0042_eb60;
pub(crate) const SET_SEED: u32 = 0x0041_ac30;
/// `ExtraLock::ExtraLock(lock data)` (`00430c70`, `this` = the new block).
pub(crate) const EXTRA_LOCK_INIT: u32 = 0x0043_0c70;
/// `REFR_LOCK::SetLocked(bool)` (`00430a90`).
pub(crate) const REFR_LOCK_SET_LOCKED: u32 = 0x0043_0a90;
pub(crate) const SET_RADIO_DATA: u32 = 0x0041_8310;
pub(crate) const GET_TELEPORT: u32 = 0x0041_8460;
pub(crate) const DOOR_TELEPORT_DATA_INIT: u32 = 0x0043_a160;
pub(crate) const SET_TELEPORT: u32 = 0x0041_9120;
pub(crate) const DOOR_TELEPORT_DATA_LOAD: u32 = 0x0043_a4e0;
pub(crate) const GET_MAP_MARKER: u32 = 0x0041_8490;
pub(crate) const MAP_MARKER_DATA_INIT: u32 = 0x0043_8bb0;
pub(crate) const SET_MAP_MARKER: u32 = 0x0041_9250;
pub(crate) const MAP_MARKER_DATA_LOAD: u32 = 0x0043_8ca0;
pub(crate) const GET_AUDIO_MARKER: u32 = 0x0041_84c0;
pub(crate) const AUDIO_MARKER_DATA_INIT: u32 = 0x0058_9450;
pub(crate) const SET_AUDIO_MARKER: u32 = 0x0041_9380;
pub(crate) const AUDIO_MARKER_DATA_LOAD: u32 = 0x0058_9600;
pub(crate) const GET_AUDIO_BUOY_MARKER: u32 = 0x0041_84f0;
pub(crate) const AUDIO_BUOY_MARKER_DATA_INIT: u32 = 0x0068_0890;
pub(crate) const SET_AUDIO_BUOY_MARKER: u32 = 0x0041_94b0;
pub(crate) const AUDIO_BUOY_MARKER_DATA_LOAD: u32 = 0x0058_9890;
pub(crate) const SET_PACKAGE_START_LOCATION: u32 = 0x0041_ad00;
/// `ExtraRagDollData::ExtraRagDollData` (`00432cb0`).
pub(crate) const EXTRA_RAGDOLL_DATA_INIT: u32 = 0x0043_2cb0;
/// `RagDollData::RagDollData` (`004d9330`).
pub(crate) const RAGDOLL_DATA_INIT: u32 = 0x004d_9330;
/// `RagDollData::Load(file)` (`004d94a0`), true if it loaded.
pub(crate) const RAGDOLL_DATA_LOAD: u32 = 0x004d_94a0;
pub(crate) const SET_DISTANT_DATA: u32 = 0x0041_d950;
pub(crate) const SET_ENABLE_STATE_PARENT: u32 = 0x0041_da40;
pub(crate) const SET_ENABLE_STATE_FLAGS: u32 = 0x0041_dc70;
/// Constructor of the type `0x53` extra data (`004338b0`, `0x20` bytes).
pub(crate) const ACTIVATE_REF_INIT: u32 = 0x0043_38b0;
/// `ExtraDataList::SetActivateTextOverride(char *)` (`0041ece0`).
pub(crate) const SET_ACTIVATE_TEXT_OVERRIDE: u32 = 0x0041_ece0;
/// `BSSimpleList::AddHead(&item)` (`005ae3d0`).
pub(crate) const LIST_ADD_HEAD: u32 = 0x005a_e3d0;
pub(crate) const ADD_DECAL_REF: u32 = 0x0041_f080;
pub(crate) const MULTIBOUND_MARKER_DATA_INIT: u32 = 0x0043_8f50;
pub(crate) const MULTIBOUND_MARKER_DATA_LOAD: u32 = 0x0043_8fc0;
pub(crate) const SET_MULTIBOUND_DATA: u32 = 0x0042_1f30;
pub(crate) const NAVMESH_PORTAL_INIT: u32 = 0x0069_2870;
pub(crate) const NAVMESH_PORTAL_LOAD: u32 = 0x0069_dfd0;
pub(crate) const SET_NAVMESH_PORTAL: u32 = 0x0042_e2c0;
pub(crate) const NAVMESH_PORTAL_EXTRA_DESTROY: u32 = 0x0043_2f30;
pub(crate) const ADD_REFLECTOR_REF: u32 = 0x0041_f1a0;
pub(crate) const ADD_REFLECTED_REF: u32 = 0x0041_f330;
pub(crate) const ADD_LIT_WATER_REF: u32 = 0x0041_f940;
/// `BGSPrimitive` factory `(type, radii, color)` (`004a4e90`, cdecl).
pub(crate) const PRIMITIVE_CREATE: u32 = 0x004a_4e90;
pub(crate) const ADD_PRIMITIVE: u32 = 0x0041_fa60;
/// Allocation of `0xFC` bytes for the occlusion plane (`00aa13e0`, cdecl).
pub(crate) const ALLOCATE_ALIGNED: u32 = 0x00aa_13e0;
/// `BSOcclusionPlane::BSOcclusionPlane` (`00c335d0`).
pub(crate) const OCCLUSION_PLANE_INIT: u32 = 0x00c3_35d0;
/// `NiMatrix3::MakeRotation(angle, x, y, z)` (`004168a0`).
pub(crate) const MATRIX_MAKE_ROTATION: u32 = 0x0041_68a0;
pub(crate) const SET_OCCLUSION_PLANE: u32 = 0x0042_2150;
pub(crate) const PATROL_REF_DATA_INIT: u32 = 0x0067_c690;
pub(crate) const SET_PATROL_REF_DATA: u32 = 0x0041_fc10;
pub(crate) const SET_OCCLUSION_PLANE_REF_DATA: u32 = 0x0041_fec0;
pub(crate) const SET_PORTAL_REF_DATA: u32 = 0x0042_0210;
pub(crate) const SET_ROOM_REF_DATA: u32 = 0x0042_0440;
pub(crate) const SET_COLLISION_DATA: u32 = 0x0042_0fd0;
pub(crate) const GET_PATROL_REF_DATA: u32 = 0x0041_fe90;
pub(crate) const PACKAGE_EVENT_ACTION_LOAD: u32 = 0x0067_dd20;
pub(crate) const SET_IGNORED_BY_SANDBOX: u32 = 0x0042_f200;
/// Stores its argument at `this + 0x0C` (`0041fd00`).
pub(crate) const SET_SPECIAL_RENDER_WORD: u32 = 0x0041_fd00;
/// `BGSSaveFormBuffer::GetForm` (`007af430`).
pub(crate) const SAVE_BUFFER_GET_FORM: u32 = 0x007a_f430;
/// `TESForm::GetFormType` (`00401170`, `MOVZX EAX,[ECX+4]`).
pub(crate) const FORM_TYPE: u32 = 0x0040_1170;

// InitItem's callees.
pub(crate) const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// Form by id (`004839c0`, cdecl).
pub(crate) const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `reference + 0x44`, the reference's embedded `ExtraDataList` (`005d43c0`).
pub(crate) const REFERENCE_EXTRA_LIST: u32 = 0x005d_43c0;
/// `TESObjectREFR::pBaseForm` getter (`004181e0`, via `007af430`: `+0x20`).
pub(crate) const REFERENCE_BASE_FORM: u32 = 0x0041_81e0;
pub(crate) const ACTOR_BASE_GET_HEALTH: u32 = 0x005f_0b00;
pub(crate) const DOOR_TELEPORT_DATA_INIT_ITEM: u32 = 0x0043_a590;
pub(crate) const MAP_MARKER_GET_VISIBLE: u32 = 0x0043_8ed0;
pub(crate) const MAP_MARKER_GET_TRAVEL_LOC: u32 = 0x0043_8ef0;
pub(crate) const MAP_MARKER_SET_TRAVEL_LOC: u32 = 0x0044_de80;
pub(crate) const MAP_MARKER_GET_REPUTATION: u32 = 0x0044_edb0;
pub(crate) const MAP_MARKER_SET_REPUTATION: u32 = 0x0043_7730;
pub(crate) const DATA_HANDLER_GET_REPUTATION: u32 = 0x0046_1630;
pub(crate) const CHECK_ENABLE_PARENT_LOOP: u32 = 0x0056_aac0;
pub(crate) const ENABLE_PARENT_ADD_CHILD: u32 = 0x0041_dcd0;
pub(crate) const LINKED_REF_ADD_CHILD: u32 = 0x0041_e530;
pub(crate) const DECAL_REFS_INIT_ITEM: u32 = 0x0043_3db0;
pub(crate) const REFLECTOR_REFS_INIT_ITEM: u32 = 0x0043_3ed0;
pub(crate) const LIT_WATER_REFS_INIT_ITEM: u32 = 0x0043_4050;
pub(crate) const REFERENCE_GET_INTERIOR: u32 = 0x0057_5d10;
pub(crate) const PATROL_REF_DATA_INIT_ITEM: u32 = 0x0067_c6c0;
pub(crate) const GET_OCCLUSION_PLANE: u32 = 0x0042_2120;
pub(crate) const SET_LINKED_PLANE: u32 = 0x0041_81c0;
pub(crate) const PORTAL_ADD_REFERENCE: u32 = 0x0042_0ce0;
pub(crate) const LIST_SET_ITEM: u32 = 0x0072_6c60;
pub(crate) const IMPACT_SWAP_INIT_ITEM: u32 = 0x0058_f210;
pub(crate) const AUDIO_MARKER_GET_CONTROLLER: u32 = 0x0059_bb30;
pub(crate) const AUDIO_MARKER_SET_CONTROLLER: u32 = 0x0070_37c0;
pub(crate) const ADD_ACTIVATE_REF_CHILD: u32 = 0x0041_edd0;
pub(crate) const LIST_REMOVE_AFTER: u32 = 0x0090_5330;
pub(crate) const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// Type descriptors for `__RTDynamicCast`: `TESKey` and the controller of an
/// audio marker.
pub(crate) const RTTI_TES_KEY: u32 = 0x0118_41b4;
pub(crate) const RTTI_MEDIA_LOCATION_CONTROLLER: u32 = 0x0118_418c;

pub(crate) const MESSAGE_RAGDOLL_ON_LIVE_ACTOR: u32 = 0x0101_4b88;
pub(crate) const MESSAGE_LOCK_KEY_MISSING: u32 = 0x0101_5060;
pub(crate) const MESSAGE_ENABLE_PARENT_MISSING: u32 = 0x0101_4c90;
pub(crate) const MESSAGE_ENABLE_PARENT_LOOP: u32 = 0x0101_4c50;
pub(crate) const MESSAGE_LINKED_REF_MISSING: u32 = 0x0101_4b28;
pub(crate) const MESSAGE_EMPTY_DECALS: u32 = 0x0101_4890;
pub(crate) const MESSAGE_MULTIBOUND_REF_MISSING: u32 = 0x0101_46b8;
pub(crate) const MESSAGE_EMPTY_REFLECTOR_REFS: u32 = 0x0101_4684;
pub(crate) const MESSAGE_EMITTANCE_SOURCE_MISSING: u32 = 0x0101_4728;
pub(crate) const MESSAGE_RADIO_POSITION_MISSING: u32 = 0x0101_50f8;
pub(crate) const MESSAGE_RADIO_POSITION_INTERIOR: u32 = 0x0101_50b0;
pub(crate) const MESSAGE_PLANE_REF_MISSING: u32 = 0x0101_4ab8;
pub(crate) const MESSAGE_NO_OCCLUSION_PLANE: u32 = 0x0101_4a80;
pub(crate) const MESSAGE_PORTAL_REF_MISSING: u32 = 0x0101_4a18;
pub(crate) const MESSAGE_PORTAL_ROOMS_SAME: u32 = 0x0101_49c0;
pub(crate) const MESSAGE_PORTAL_ROOMS_NULL: u32 = 0x0101_4984;
pub(crate) const MESSAGE_ROOM_REF_MISSING: u32 = 0x0101_4920;
pub(crate) const MESSAGE_EMPTY_LIT_WATER: u32 = 0x0101_4658;
pub(crate) const MESSAGE_CONTROLLER_MISSING: u32 = 0x0101_5000;
pub(crate) const MESSAGE_ACTIVATE_REF_MISSING: u32 = 0x0101_48ec;
pub(crate) const MESSAGE_EMPTY_ACTIVATE_PARENT: u32 = 0x0101_48b8;

/// The four letters of a chunk name as the word the exe pushes
/// (`chunk_tag(b"XCWT")` is `0x54574358`).
pub(crate) const fn chunk_tag(name: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*name)
}

/// Copies `bytes` bytes (a multiple of 4) from `source` to `target`.
pub(crate) fn copy_block(e: &mut Engine, source: u32, target: u32, bytes: u32) {
    for offset in (0..bytes).step_by(4) {
        let word = e.mem.u32(source.wrapping_add(offset));
        e.mem.set_u32(target.wrapping_add(offset), word);
    }
}

/// Vtable of the type `0x5A` extra data.
pub(crate) const VTABLE_EXTRA_TYPE_5A: u32 = 0x0101_4618;
// ---------------------------------------------------------------------------
// Helpers

pub(crate) fn lock(e: &mut Engine, name: u32) {
    e.call(LOCK, &args![EXTRA_CRIT_SECTION, name]);
}

pub(crate) fn unlock(e: &mut Engine) {
    e.call(UNLOCK, &args![EXTRA_CRIT_SECTION]);
}

pub(crate) fn get_type(e: &mut Engine, extra: Ptr<BSExtraData>) -> u8 {
    e.call(GET_TYPE, &args![extra]).u8()
}

pub(crate) fn get_next(e: &mut Engine, extra: Ptr<BSExtraData>) -> Ptr<BSExtraData> {
    e.call(GET_NEXT, &args![extra]).ptr()
}

pub(crate) fn set_next(e: &mut Engine, extra: Ptr<BSExtraData>, next: Ptr<BSExtraData>) {
    e.call(SET_NEXT, &args![extra, next]);
}

/// The thread's `BaseExtraList` statics (the TLS block).
pub(crate) fn thread_cache(e: &mut Engine) -> Ptr<BaseExtraListThreadCache> {
    Ptr::new(e.tls())
}

/// Address of cache entry `index` (`ppLastExtraData[index]`).
pub(crate) fn cache_entry(cache: Ptr<BaseExtraListThreadCache>, index: u32) -> u32 {
    cache.addr() + 0x10 + index.wrapping_mul(4)
}

/// `memset` of the whole `ppLastExtraData` array to null.
pub(crate) fn clear_cache_entries(e: &mut Engine, cache: Ptr<BaseExtraListThreadCache>) {
    e.call(MEMSET, &args![cache_entry(cache, 0), 0u32, CACHE_BYTES]);
}

/// `iDirty` and this thread's `iThreadDirty` both go up by one, so the other
/// threads (and this one) drop what they cached.
pub(crate) fn bump_dirty_counters(e: &mut Engine) {
    let dirty = e.global::<u32>(DIRTY).wrapping_add(1);
    e.set_global(DIRTY, dirty);
    let cache = thread_cache(e);
    let thread_dirty = e
        .get(cache, BaseExtraListThreadCache::iThreadDirty)
        .wrapping_add(1);
    e.set(cache, BaseExtraListThreadCache::iThreadDirty, thread_dirty);
}

/// `delete extra`: the scalar deleting destructor, slot 0 of the vtable.
pub(crate) fn delete_extra(e: &mut Engine, extra: Ptr<BSExtraData>) {
    if !extra.is_null() {
        e.vcall(extra.addr(), 0, &args![1u32]);
    }
}

/// The destructors of the extra data subclasses all end the same way:
/// the real destructor, then `operator delete` when bit 0 of `flags` is set.
pub(crate) fn finish_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0040f680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraNorthRotation` (its only caller is
/// `SetNorthRotation`): a `BSExtraData` of type `0x43` with a north
/// rotation of 0.0. Returns `this`.
pub fn fn_0040f680(e: &mut Engine, this: Ptr<ExtraNorthRotation>) -> Ptr<ExtraNorthRotation> {
    e.call(BS_EXTRA_DATA_INIT, &args![this, 0x43u32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_NORTH_ROTATION);
    e.set(this, ExtraNorthRotation::fNorthRot, 0.0);
    this
}

// Translated from 0040f6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x0B` extra data, a `BSExtraData` with one
/// 32-bit value (zero) at +0x0C; the Xbox PDB class of that shape is
/// `ExtraDetachTime` (`iTime`). Returns `this`.
pub fn fn_0040f6b0(e: &mut Engine, this: Ptr<ExtraDetachTime>) -> Ptr<ExtraDetachTime> {
    e.call(BS_EXTRA_DATA_INIT, &args![this, 0x0Bu32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_DETACH_TIME);
    e.set(this, ExtraDetachTime::iTime, 0);
    this
}

// Translated from 0040f700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSExtraData::Compare` (Xbox PDB): true when the two differ, which for
/// the base class means their types differ; a null `other` always differs.
pub fn bs_extra_data_compare(
    e: &mut Engine,
    this: Ptr<BSExtraData>,
    other: Ptr<BSExtraData>,
) -> bool {
    if other.is_null() {
        return true;
    }
    let this_type = get_type(e, this);
    let other_type = get_type(e, other);
    this_type != other_type
}

// Translated from 0040f740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::BaseExtraList` (inlined into the unit): sets the vtable,
/// clears the head and the 21-byte type bitmap. Returns `this`.
pub fn fn_0040f740(e: &mut Engine, this: Ptr<BaseExtraList>) -> Ptr<BaseExtraList> {
    e.mem.set_u32(this.addr(), VTABLE_BASE_EXTRA_LIST);
    e.set(this, BaseExtraList::pHead, Ptr::NULL);
    let flags = this.byte_add(8);
    e.call(MEMSET, &args![flags, 0u32, FLAGS_LEN]);
    this
}

// Translated from 0040f780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::_scalar_deleting_destructor_`: runs the destructor and
/// frees the object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0040f780(e: &mut Engine, this: Ptr<BaseExtraList>, flags: u32) -> Ptr<BaseExtraList> {
    fn_0040f7b0(e, this);
    finish_scalar_deleting_destructor(e, this.cast(), flags).cast()
}

// Translated from 0040f7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::~BaseExtraList` (Xbox PDB name of the destructor): sets
/// the vtable and destroys every extra data of the list.
pub fn fn_0040f7b0(e: &mut Engine, this: Ptr<BaseExtraList>) {
    e.mem.set_u32(this.addr(), VTABLE_BASE_EXTRA_LIST);
    base_extra_list_remove_all(e, this, true);
}

// Translated from 0040f7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::ClearLastExtra` (Xbox PDB): bumps the dirty counters and,
/// if `list` is the list this thread last looked at, forgets the cached
/// extra data of `extra_type`.
pub fn base_extra_list_clear_last_extra(e: &mut Engine, list: Ptr<BaseExtraList>, extra_type: i32) {
    bump_dirty_counters(e);
    let cache = thread_cache(e);
    if list
        == e.get(cache, BaseExtraListThreadCache::pLastExtraList)
            .cast()
        && (0..CACHE_ENTRIES as i32).contains(&extra_type)
    {
        e.mem.set_u32(cache_entry(cache, extra_type as u32), 0);
    }
}

// Translated from 0040f860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::ClearLastExtraAll` (Xbox PDB): bumps the dirty counters
/// and, if `list` is the list this thread last looked at, empties the whole
/// cache and forgets the list.
pub fn base_extra_list_clear_last_extra_all(e: &mut Engine, list: Ptr<BaseExtraList>) {
    bump_dirty_counters(e);
    let cache = thread_cache(e);
    if list
        == e.get(cache, BaseExtraListThreadCache::pLastExtraList)
            .cast()
    {
        clear_cache_entries(e, cache);
        e.set(cache, BaseExtraListThreadCache::pLastExtraList, Ptr::NULL);
    }
}

// Translated from 0040f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops this thread's cache when the global dirty counter has moved since
/// the cache was filled: forgets the list, clears the entries, and takes the
/// current counter.
pub fn fn_0040f900(e: &mut Engine) {
    let cache = thread_cache(e);
    let dirty = e.global::<u32>(DIRTY);
    if e.get(cache, BaseExtraListThreadCache::iThreadDirty) != dirty {
        e.set(cache, BaseExtraListThreadCache::pLastExtraList, Ptr::NULL);
        clear_cache_entries(e, cache);
        e.set(cache, BaseExtraListThreadCache::iThreadDirty, dirty);
    }
}

// Translated from 0040f980 (decompiled, FalloutNV.exe 1.4.0.525)
/// True for the extra data types that `AddExtra` puts at the head of the
/// chain instead of the end: `0x0C`, `0x0D`, `0x0E`, `0x15` and `0x2B`
/// (a jump table in the exe).
pub fn fn_0040f980(_e: &mut Engine, extra_type: u32) -> bool {
    matches!(extra_type, 0x0c | 0x0d | 0x0e | 0x15 | 0x2b)
}

// Translated from 0040f9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The extra data of `extra_type` this thread has cached for `list`, or null
/// (not the list cached, type out of range, or nothing cached yet). Drops a
/// stale cache first.
pub fn fn_0040f9e0(e: &mut Engine, list: Ptr<BaseExtraList>, extra_type: i32) -> Ptr<BSExtraData> {
    let mut found = Ptr::NULL;
    let cache = thread_cache(e);
    if list
        == e.get(cache, BaseExtraListThreadCache::pLastExtraList)
            .cast()
    {
        fn_0040f900(e);
        if (0..CACHE_ENTRIES as i32).contains(&extra_type) {
            found = Ptr::new(e.mem.u32(cache_entry(cache, extra_type as u32)));
        }
    }
    found
}

// Translated from 0040fa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Caches `extra` as the extra data of its type for `list` in this thread's
/// cache, switching the cache to `list` (and emptying it) when it held
/// another list. Nothing happens for a null list or extra data. The index is
/// the type byte as it is, with no range check.
pub fn fn_0040fa40(e: &mut Engine, list: Ptr<BaseExtraList>, extra: Ptr<BSExtraData>) {
    if list.is_null() || extra.is_null() {
        return;
    }
    let cache = thread_cache(e);
    if list
        != e.get(cache, BaseExtraListThreadCache::pLastExtraList)
            .cast()
    {
        clear_cache_entries(e, cache);
        e.set(cache, BaseExtraListThreadCache::pLastExtraList, list.cast());
    }
    let extra_type = get_type(e, extra);
    e.mem
        .set_u32(cache_entry(cache, extra_type as u32), extra.addr());
}

// Translated from 0040fae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::RemoveAll` (Xbox PDB): empties the list. With `destroy`
/// every extra data is unlinked and deleted; without it the chain is just
/// forgotten. The type bitmap is cleared either way.
pub fn base_extra_list_remove_all(e: &mut Engine, this: Ptr<BaseExtraList>, destroy: bool) {
    lock(e, NAME_REMOVE_ALL);
    base_extra_list_clear_last_extra_all(e, this);
    if destroy {
        let mut current: Ptr<BSExtraData> = e.get(this, BaseExtraList::pHead).cast();
        while !current.is_null() {
            let doomed = current;
            current = get_next(e, current);
            e.set(this, BaseExtraList::pHead, current.cast());
            if destroy {
                delete_extra(e, doomed);
            }
        }
    } else {
        e.set(this, BaseExtraList::pHead, Ptr::NULL);
    }
    let flags = this.byte_add(8);
    e.call(MEMSET, &args![flags, 0u32, FLAGS_LEN]);
    unlock(e);
}

// Translated from 0040fcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::RemoveAllDefault` (Xbox PDB): removes every extra data
/// except the persistent kinds (types `0x2E`, `0x38`, `0x4F`, `0x52`, `0x54`,
/// `0x58`, `0x65`, `0x7B`, `0x84`, from a jump table in the exe), deleting
/// them when `destroy` is set, and clears their bits in the type bitmap.
pub fn base_extra_list_remove_all_default(e: &mut Engine, this: Ptr<BaseExtraList>, destroy: bool) {
    lock(e, NAME_REMOVE_ALL_DEFAULT);
    base_extra_list_clear_last_extra_all(e, this);
    let mut kept: Ptr<BSExtraData> = Ptr::NULL;
    let mut current: Ptr<BSExtraData> = e.get(this, BaseExtraList::pHead).cast();
    while !current.is_null() {
        let extra_type = get_type(e, current);
        let remove = !matches!(
            extra_type,
            0x2e | 0x38 | 0x4f | 0x52 | 0x54 | 0x58 | 0x65 | 0x7b | 0x84
        );
        let next = e.mem.u32(current.addr() + 8);
        if remove {
            if kept.is_null() {
                e.set(this, BaseExtraList::pHead, Ptr::new(next));
            } else {
                set_next(e, kept, Ptr::new(next));
            }
            if destroy {
                delete_extra(e, current);
            }
            fn_0040fee0(e, this, extra_type, false);
        } else {
            kept = current;
        }
        current = Ptr::new(next);
    }
    unlock(e);
}

// Translated from 0040fe20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::ItemsInList` (Xbox PDB): the number of extra data in the
/// chain.
pub fn base_extra_list_items_in_list(e: &mut Engine, this: Ptr<BaseExtraList>) -> i32 {
    lock(e, NAME_ITEMS_IN_LIST);
    let mut count = 0;
    let mut current: Ptr<BSExtraData> = e.get(this, BaseExtraList::pHead).cast();
    while !current.is_null() {
        count += 1;
        current = get_next(e, current);
    }
    unlock(e);
    count
}

// Translated from 0040fe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::HasExtra` (Xbox PDB): whether the type bitmap has the bit
/// of `extra_type` (false for a type whose byte is past the 21-byte bitmap).
pub fn base_extra_list_has_extra(e: &mut Engine, this: Ptr<BaseExtraList>, extra_type: u8) -> bool {
    let byte_index = (extra_type >> 3) as u32;
    if byte_index >= FLAGS_LEN {
        return false;
    }
    let flags = e.mem.u8(this.addr() + 8 + byte_index);
    flags & (1 << (extra_type & 7)) != 0
}

// Translated from 0040fee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set`) or clears the bit of `extra_type` in the type bitmap; a type
/// whose byte is past the 21-byte bitmap is ignored.
pub fn fn_0040fee0(e: &mut Engine, this: Ptr<BaseExtraList>, extra_type: u8, set: bool) {
    let byte_index = (extra_type >> 3) as u32;
    if byte_index >= FLAGS_LEN {
        return;
    }
    let address = this.addr() + 8 + byte_index;
    let bit = 1u8 << (extra_type & 7);
    let flags = e.mem.u8(address);
    e.mem
        .set_u8(address, if set { flags | bit } else { !bit & flags });
}

// Translated from 0040ff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::AddExtra` (Xbox PDB): links `extra` into the chain and
/// sets its type bit. The types of `fn_0040f980` go to the head (the old head
/// becomes the new extra data's next), every other type to the end. Returns
/// `extra`.
pub fn base_extra_list_add_extra(
    e: &mut Engine,
    this: Ptr<BaseExtraList>,
    extra: Ptr<BSExtraData>,
) -> Ptr<BSExtraData> {
    lock(e, NAME_ADD_EXTRA);
    let extra_type = get_type(e, extra);
    let head: Ptr<BSExtraData> = e.get(this, BaseExtraList::pHead).cast();
    if fn_0040f980(e, extra_type as u32) {
        if !head.is_null() {
            set_next(e, extra, head);
        }
        e.set(this, BaseExtraList::pHead, extra.cast());
    } else if head.is_null() {
        e.set(this, BaseExtraList::pHead, extra.cast());
    } else {
        let mut last = head;
        loop {
            let next = get_next(e, last);
            if next.is_null() {
                break;
            }
            last = get_next(e, last);
        }
        set_next(e, last, extra);
    }
    let extra_type = get_type(e, extra);
    fn_0040fee0(e, this, extra_type, true);
    unlock(e);
    extra
}

// Translated from 00410020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::RemoveExtra` (Xbox PDB), the overload that takes the extra
/// data: unlinks `extra` from the chain, drops it from the caches, then
/// deletes it (`destroy`) or only detaches it (next set to null), clears its
/// type bit and bumps the dirty counters. A null `extra` does nothing.
pub fn base_extra_list_remove_extra(
    e: &mut Engine,
    this: Ptr<BaseExtraList>,
    extra: Ptr<BSExtraData>,
    destroy: bool,
) {
    if extra.is_null() {
        return;
    }
    lock(e, NAME_REMOVE_EXTRA);
    let extra_type = get_type(e, extra);
    let previous = base_extra_list_get_prev_extra_data(e, this, extra_type);
    if previous.is_null() {
        let next = get_next(e, extra);
        e.set(this, BaseExtraList::pHead, next.cast());
    } else {
        let next = get_next(e, extra);
        set_next(e, previous, next);
    }
    base_extra_list_clear_last_extra(e, this, extra_type as i32);
    if destroy {
        delete_extra(e, extra);
    } else {
        set_next(e, extra, Ptr::NULL);
    }
    fn_0040fee0(e, this, extra_type, false);
    bump_dirty_counters(e);
    unlock(e);
}

// Translated from 00410140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::RemoveExtra` (Xbox PDB, `_ov2`), the overload that takes a
/// type: finds the first extra data of that type, unlinks and deletes it, and
/// clears the type bit whether or not one was found.
pub fn base_extra_list_remove_extra_ov2(e: &mut Engine, this: Ptr<BaseExtraList>, extra_type: u8) {
    lock(e, NAME_REMOVE_EXTRA);
    let mut previous: Ptr<BSExtraData> = Ptr::NULL;
    let mut current: Ptr<BSExtraData> = e.call(SIMPLE_LIST_NEXT, &args![this]).ptr();
    while !current.is_null() && get_type(e, current) != extra_type {
        previous = current;
        current = get_next(e, current);
    }
    if !current.is_null() {
        if previous.is_null() {
            let next = get_next(e, current);
            e.set(this, BaseExtraList::pHead, next.cast());
        } else {
            let next = get_next(e, current);
            set_next(e, previous, next);
        }
        base_extra_list_clear_last_extra(e, this, extra_type as i32);
        delete_extra(e, current);
    }
    fn_0040fee0(e, this, extra_type, false);
    unlock(e);
}

// Translated from 00410220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::GetExtraData` (Xbox PDB): the first extra data of
/// `extra_type`, or null. Answers from the type bitmap and this thread's
/// cache when it can; otherwise walks the chain under the lock and caches
/// what it finds.
pub fn base_extra_list_get_extra_data(
    e: &mut Engine,
    this: Ptr<BaseExtraList>,
    extra_type: u8,
) -> Ptr<BSExtraData> {
    if !base_extra_list_has_extra(e, this, extra_type) {
        return Ptr::NULL;
    }
    let mut found = fn_0040f9e0(e, this, extra_type as i32);
    if found.is_null() {
        lock(e, NAME_GET_EXTRA_DATA);
        let mut current: Ptr<BSExtraData> = e.get(this, BaseExtraList::pHead).cast();
        while !current.is_null() {
            if get_type(e, current) == extra_type {
                fn_0040fa40(e, this, current);
                found = current;
                break;
            }
            current = get_next(e, current);
        }
        unlock(e);
    }
    found
}

// Translated from 004102d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BaseExtraList::GetPrevExtraData` (Xbox PDB): the extra data that comes
/// right before the first one of `extra_type`; null when that one is the head,
/// when the type bitmap says there is none, or when the walk runs off the end
/// (then it is the last extra data).
pub fn base_extra_list_get_prev_extra_data(
    e: &mut Engine,
    this: Ptr<BaseExtraList>,
    extra_type: u8,
) -> Ptr<BSExtraData> {
    if !base_extra_list_has_extra(e, this, extra_type) {
        return Ptr::NULL;
    }
    lock(e, NAME_GET_PREV_EXTRA_DATA);
    let mut current: Ptr<BSExtraData> = e.get(this, BaseExtraList::pHead).cast();
    let mut previous: Ptr<BSExtraData> = Ptr::NULL;
    while !current.is_null() && get_type(e, current) != extra_type {
        previous = current;
        current = get_next(e, current);
    }
    unlock(e);
    previous
}

// Translated from 00410360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::ExtraDataList` (Xbox PDB): the `BaseExtraList`
/// constructor, then the `ExtraDataList` vtable. Returns `this`.
pub fn extra_data_list_extra_data_list(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<ExtraDataList> {
    fn_0040f740(e, this.cast());
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_DATA_LIST);
    this
}

// Translated from 00410380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::_scalar_deleting_destructor_`: runs the destructor and
/// frees the object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00410380(e: &mut Engine, this: Ptr<ExtraDataList>, flags: u32) -> Ptr<ExtraDataList> {
    fn_004103b0(e, this);
    finish_scalar_deleting_destructor(e, this.cast(), flags).cast()
}

// Translated from 004103b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::~ExtraDataList`: sets the `ExtraDataList` vtable, then
/// runs the `BaseExtraList` destructor.
pub fn fn_004103b0(e: &mut Engine, this: Ptr<ExtraDataList>) {
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_DATA_LIST);
    fn_0040f7b0(e, this.cast());
}

// ---------------------------------------------------------------------------
// AddExtraCopy

/// Copies whose setter takes the 32-bit word at `extra + 0x0C`:
/// `list.Setter(word)`. (type, setter address)
pub(crate) const COPY_WORD: &[(u8, u32)] = &[
    (0x03, 0x0041_e160),
    (0x07, 0x0041_bd10),
    (0x08, 0x0041_c190),
    (0x14, 0x0041_d5b0),
    (0x1a, 0x0041_cc90),
    (0x1c, 0x0041_c7f0),
    (0x1e, 0x0042_1540),
    (0x20, 0x0041_8550),
    (0x21, 0x0041_9700),
    (0x22, 0x0041_97d0),
    (0x23, 0x0041_98a0),
    (0x3b, 0x0041_e250),
    (0x3c, 0x0042_1430),
    (0x3f, 0x0041_9d10),
    (0x44, 0x0042_1b70),
    (0x46, 0x0041_9dc0),
    (0x51, 0x0041_e440),
    (0x59, 0x0041_c290),
    (0x63, 0x0042_1e40),
    (0x67, 0x0042_1d50),
    (0x74, 0x0042_1c60),
    (0x81, 0x0041_c090),
];

/// Copies whose setter takes the `float` at `extra + 0x0C`.
pub(crate) const COPY_FLOAT: &[(u8, u32)] = &[
    (0x25, 0x0041_9970),
    (0x27, 0x0041_9bb0),
    (0x28, 0x0041_9c60),
    (0x30, 0x0041_9fb0),
    (0x5c, 0x0042_2220),
    (0x5d, 0x0042_2350),
    (0x7a, 0x0041_b580),
];

/// Copies whose setter takes the byte at `extra + 0x0C`.
pub(crate) const COPY_BYTE: &[(u8, u32)] = &[
    (0x0e, 0x0041_b3d0),
    (0x26, 0x0041_9a20),
    (0x31, 0x0041_ac30),
    (0x4a, 0x0042_dde0),
];

/// Copies whose setter takes the word that `READ_WORD(extra + 0x0C)` reads.
pub(crate) const COPY_READ_WORD: &[(u8, u32)] = &[
    (0x02, 0x0041_b7e0),
    (0x61, 0x0042_2050),
    (0x71, 0x0042_2150),
    (0x79, 0x0042_0f00),
];

/// Copies whose setter takes the address of the payload, `extra + 0x0C`.
pub(crate) const COPY_PAYLOAD_ADDRESS: &[(u8, u32)] = &[(0x13, 0x0041_d950), (0x68, 0x0041_8310)];

/// Copies that make a new payload object: (type, size, constructor,
/// copy-from-source `object.Copy(source word)`, list setter).
pub(crate) const COPY_NEW_OBJECT: &[(u8, u32, u32, u32, u32)] = &[
    (0x2b, 0x20, 0x0043_a160, 0x0043_a810, 0x0041_9120),
    (0x2c, 0x14, 0x0043_8bb0, 0x0043_8df0, 0x0041_9250),
    (0x6f, 0x14, 0x0067_c650, 0x0067_c6f0, 0x0041_fc10),
    (0x8c, 0x15c, 0x0058_efd0, 0x0058_f4c0, 0x0041_c390),
    (0x90, 0x34, 0x0058_9450, 0x0058_9770, 0x0041_9380),
    (0x91, 0x08, 0x0068_0890, 0x0045_34f0, 0x0041_94b0),
];

/// `new` and construct: allocates `size` bytes and runs `construct` on the
/// block, or gives null when the allocation failed.
pub(crate) fn new_object(
    e: &mut Engine,
    size: u32,
    construct: impl FnOnce(&mut Engine, u32) -> u32,
) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        0
    } else {
        construct(e, block)
    }
}

/// The extra data of `extra_type` in `list`, created with `new_object` and
/// added when the list has none.
pub(crate) fn get_or_add_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    size: u32,
    construct: impl FnOnce(&mut Engine, u32) -> u32,
) -> Ptr<BSExtraData> {
    let mut existing = base_extra_list_get_extra_data(e, list.cast(), extra_type);
    if existing.is_null() {
        existing = Ptr::new(new_object(e, size, construct));
        base_extra_list_add_extra(e, list.cast(), existing);
    }
    existing
}

/// The word of the source extra data's payload at `extra + offset`.
pub(crate) fn payload(e: &Engine, extra: Ptr<BSExtraData>, offset: u32) -> u32 {
    e.mem.u32(extra.addr() + offset)
}

/// Copies the 32-bit words `[from, to)` (byte offsets) of `source` to the same
/// offsets of `target`.
pub(crate) fn copy_words(e: &mut Engine, source: u32, target: u32, from: u32, to: u32) {
    for offset in (from..to).step_by(4) {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(target + offset, word);
    }
}

// Translated from 004103d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::AddExtraCopy` (Xbox PDB): puts a copy of `extra` into this
/// list, using the setter of the list that matches the extra data's type
/// (`this.SetX(payload of extra)`), or for the types that keep a list or a
/// structure, adding to the existing extra data of that type (creating it
/// first) or building a copy of the structure. Does nothing for a null
/// `extra` or one `IsCopyableExtra` (`00411f20`) refuses. A type with no
/// copy function is reported to the log.
///
/// The types are dispatched by a jump table in the exe (82 types, `0x02` to
/// `0x92`); the setters of the same shape are the `COPY_*` tables above, the
/// rest are the arms of the `match`. `payload(extra, 0x0C)` is the first word
/// after the `BSExtraData` base.
pub fn extra_data_list_add_extra_copy(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    extra: Ptr<BSExtraData>,
) {
    if extra.is_null() || !e.call(0x0041_1f20, &args![this, extra]).bool() {
        return;
    }
    let extra_type = get_type(e, extra);
    let list = this.addr();

    if let Some(&(_, setter)) = COPY_WORD.iter().find(|entry| entry.0 == extra_type) {
        let word = payload(e, extra, 0x0c);
        e.call(setter, &args![list, word]);
        return;
    }
    if let Some(&(_, setter)) = COPY_FLOAT.iter().find(|entry| entry.0 == extra_type) {
        let value = f32::from_bits(payload(e, extra, 0x0c));
        e.call(setter, &args![list, value]);
        return;
    }
    if let Some(&(_, setter)) = COPY_BYTE.iter().find(|entry| entry.0 == extra_type) {
        let byte = e.mem.u8(extra.addr() + 0x0c);
        e.call(setter, &args![list, byte]);
        return;
    }
    if let Some(&(_, setter)) = COPY_READ_WORD.iter().find(|entry| entry.0 == extra_type) {
        let word = e.call(READ_WORD, &args![extra.addr() + 0x0c]).u32();
        e.call(setter, &args![list, word]);
        return;
    }
    if let Some(&(_, setter)) = COPY_PAYLOAD_ADDRESS
        .iter()
        .find(|entry| entry.0 == extra_type)
    {
        e.call(setter, &args![list, extra.addr() + 0x0c]);
        return;
    }
    if let Some(&(_, size, construct, copy, setter)) =
        COPY_NEW_OBJECT.iter().find(|entry| entry.0 == extra_type)
    {
        let source = payload(e, extra, 0x0c);
        let object = new_object(e, size, |e, block| e.call(construct, &args![block]).u32());
        e.call(copy, &args![object, source]);
        e.call(setter, &args![list, object]);
        return;
    }

    match extra_type {
        0x0a => {
            // SetCanopyShadowMask(word, handle, &result) hands back the
            // structure it made through `result`; the two words at +0x14 and
            // +0x18 of the source are then copied into its first two words.
            let word = payload(e, extra, 0x0c);
            let handle = e.call(READ_WORD, &args![extra.addr() + 0x10]).u32();
            let result = e.mem.alloc(4);
            e.call(0x0041_c490, &args![list, word, handle, result]);
            let created = e.mem.u32(result);
            let second = payload(e, extra, 0x18);
            e.mem.set_u32(created + 4, second);
            let first = payload(e, extra, 0x14);
            e.mem.set_u32(created, first);
            e.mem.free(result);
        }
        0x0d => {
            let (first, second) = (payload(e, extra, 0x0c), payload(e, extra, 0x10));
            e.call(0x0041_9ed0, &args![list, first]);
            e.call(0x0041_9f80, &args![list, second]);
        }
        0x16 => {
            e.call(0x0041_aa20, &args![list, 1u32, 0u32]);
        }
        0x17 => {
            e.call(0x0041_aa20, &args![list, 1u32, 1u32]);
        }
        0x18 => {
            let word = payload(e, extra, 0x0c);
            let first_cast = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![word, 0u32, 0x0118_3028u32, 0x0118_3fd0u32, 0u32],
                )
                .u32();
            let second_cast = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![word, 0u32, 0x0118_3028u32, 0x0118_3fb4u32, 0u32],
                )
                .u32();
            let last = f32::from_bits(payload(e, extra, 0x1c));
            e.call(
                0x0041_ad00,
                &args![list, first_cast, second_cast, extra.addr() + 0x10, last],
            );
        }
        0x19 => {
            let words = [0x0cu32, 0x10, 0x14].map(|offset| payload(e, extra, offset));
            let bytes = [0x18u32, 0x19, 0x1a].map(|offset| e.mem.u8(extra.addr() + offset));
            e.call(
                0x0041_c930,
                &args![list, words[0], words[1], words[2], bytes[0], bytes[1], bytes[2]],
            );
        }
        0x1b => {
            let mut node = payload(e, extra, 0x0c);
            while node != 0 {
                let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
                let entry = e.mem.u32(item);
                if entry != 0 {
                    let (word, flag) = (e.mem.u32(entry), e.mem.u8(entry + 4));
                    e.call(0x0041_d700, &args![list, word, flag]);
                }
                node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
            }
        }
        0x1d => {
            let mut node = payload(e, extra, 0x0c);
            while node != 0 {
                let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
                if e.mem.u32(item) == 0 {
                    break;
                }
                let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
                let follower = e.mem.u32(item);
                e.call(0x0042_2480, &args![list, follower]);
                node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
            }
        }
        0x2a => {
            // ExtraLock: a copy of the 0x14-byte lock structure.
            let source = payload(e, extra, 0x0c);
            let object = new_object(e, 0x14, |e, block| fn_00411b00(e, Ptr::new(block)).addr());
            e.call(MEMCPY, &args![object, source, 0x14u32]);
            e.call(0x0041_9050, &args![list, object]);
        }
        0x2f => {
            let word = payload(e, extra, 0x0c);
            let flag = e.mem.u8(extra.addr() + 0x10);
            e.call(0x0041_d280, &args![list, word]);
            e.call(0x0041_d330, &args![list, flag]);
        }
        0x37 => {
            let word = payload(e, extra, 0x0c);
            let flag = e.mem.u8(extra.addr() + 0x10);
            e.call(0x0041_da40, &args![list, word]);
            e.call(0x0041_dc70, &args![list, flag]);
        }
        0x24 => {
            let short = e.mem.u16(extra.addr() + 0x0c);
            e.call(0x0041_9ad0, &args![list, short]);
        }
        0x3e => {
            e.call(0x0041_ab70, &args![list, 1u32]);
        }
        0x48 => {
            let value = e.mem.f32(extra.addr() + 0x0c);
            e.call(0x0042_2750, &args![list, 1u32, value]);
        }
        0x4c => {
            let existing = get_or_add_extra(e, this, 0x4c, 0x30, |e, block| {
                e.call(0x0043_5fa0, &args![block, 0u32]).u32()
            });
            // The payload from +0x0C to +0x2F (nine words) is copied whole.
            copy_words(e, extra.addr(), existing.addr(), 0x0c, 0x30);
        }
        0x53 => {
            let existing = get_or_add_extra(e, this, 0x53, 0x20, |e, block| {
                e.call(0x0043_38b0, &args![block]).u32()
            });
            e.call(0x0043_3b70, &args![existing, extra]);
        }
        0x55 => {}
        0x57 => {
            let existing = get_or_add_extra(e, this, 0x57, 0x14, |e, block| {
                e.call(0x0043_3ca0, &args![block]).u32()
            });
            e.call(0x0043_42b0, &args![existing, extra]);
        }
        0x5a => {
            e.call(0x0042_e2c0, &args![list, extra]);
        }
        0x5e => {
            e.call(0x0042_e760, &args![list]);
            let target = e.call(0x0042_e800, &args![list]).u32();
            let mut node = payload(e, extra, 0x0c);
            while node != 0 {
                let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
                if e.mem.u32(item) == 0 {
                    break;
                }
                let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
                let entry = e.mem.u32(item);
                if entry != 0 {
                    // The entry is a word and a signed byte.
                    let (word, small) = (e.mem.u32(entry), e.mem.i8(entry + 4) as i32);
                    e.call(0x0043_6f80, &args![target, word, small]);
                }
                node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
            }
        }
        0x62 => {
            let source = payload(e, extra, 0x0c);
            let mut object = 0;
            if source != 0 {
                object = new_object(e, 0xc, |e, block| e.call(0x0043_8f50, &args![block]).u32());
                e.call(0x0043_9120, &args![object, source]);
            }
            e.call(0x0042_1f30, &args![list, object]);
        }
        0x65 => {
            let existing = get_or_add_extra(e, this, 0x65, 0x14, |e, block| {
                fn_00411b40(e, Ptr::new(block)).addr()
            });
            e.call(0x0043_46f0, &args![existing, extra]);
        }
        0x66 => {
            let existing = get_or_add_extra(e, this, 0x66, 0x14, |e, block| {
                fn_00411be0(e, Ptr::new(block)).addr()
            });
            e.call(0x0043_4c80, &args![existing, extra]);
        }
        0x6b => {
            let word = payload(e, extra, 0x0c);
            let converted = e.call(0x004a_4d40, &args![word]).u32();
            e.call(0x0041_fa60, &args![list, converted]);
        }
        0x6e => {
            let (first, second) = (payload(e, extra, 0x0c), payload(e, extra, 0x10));
            e.call(0x0042_eb60, &args![list, first, second]);
        }
        0x72 => {
            let word = payload(e, extra, 0x0c);
            let object = new_object(e, 4, |e, block| fn_00411e00(e, Ptr::new(block)).addr());
            let read = e.call(READ_WORD, &args![word]).u32();
            e.call(0x0053_7e90, &args![object, read]);
            e.call(0x0042_0fd0, &args![list, object]);
        }
        0x73 => {
            let mut node = payload(e, extra, 0x0c);
            while node != 0 {
                let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
                if e.mem.u32(item) == 0 {
                    break;
                }
                let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
                let entry = e.mem.u32(item);
                e.call(0x0042_ef20, &args![list, entry]);
                node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
            }
        }
        0x76 => {
            let source = payload(e, extra, 0x0c);
            let object = new_object(e, 0x10, |e, block| fn_00411dc0(e, Ptr::new(block)).addr());
            copy_words(e, source, object, 0, 0x10);
            e.call(0x0041_fec0, &args![list, object]);
        }
        0x77 => {
            let source = payload(e, extra, 0x0c);
            for index in 0..2u32 {
                let value = e.mem.u32(source + 4 * index);
                e.call(0x0042_0a60, &args![list, index, value]);
            }
        }
        0x78 => {
            // The original builds a temporary (0x0045cec0 destroys it) when
            // `extra` is null, which the check at the top rules out.
            let temporary = e.mem.alloc(4);
            let address = if extra.is_null() {
                e.call(0x0063_3c90, &args![temporary, 0u32]).u32()
            } else {
                extra.addr() + 0x0c
            };
            let word = e.call(READ_WORD, &args![address]).u32();
            if extra.is_null() {
                e.call(0x0045_cec0, &args![temporary]);
            }
            e.mem.free(temporary);
            e.call(0x0042_0e00, &args![list, word]);
        }
        0x7b => {
            let room_data = payload(e, extra, 0x0c);
            let mut node = room_data + 8;
            while node != 0 {
                let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
                if e.mem.u32(item) == 0 {
                    break;
                }
                let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
                let entry = e.mem.u32(item);
                e.call(0x0042_0c10, &args![list, entry]);
                node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
            }
            let flag = e.mem.u8(room_data + 0x10) != 0;
            e.call(0x0042_0870, &args![list, flag]);
        }
        0x80 => {
            e.call(0x0042_f200, &args![list, 1u32]);
        }
        0x84 => {
            let existing = get_or_add_extra(e, this, 0x84, 0x14, |e, block| {
                fn_00411c80(e, Ptr::new(block)).addr()
            });
            e.call(0x0043_4a70, &args![existing, extra]);
        }
        0x85 => {
            let existing = get_or_add_extra(e, this, 0x85, 0x14, |e, block| {
                fn_00411d20(e, Ptr::new(block)).addr()
            });
            e.call(0x0043_4ee0, &args![existing, extra]);
        }
        0x8b => {
            // The get-or-create is a call (0042f420) that adds the new extra
            // data to the list itself.
            let target = e.call(0x0042_f420, &args![list]).u32();
            copy_words(e, extra.addr(), target, 0x10, 0x1c);
            let word = payload(e, extra, 0x1c);
            e.mem.set_u32(target + 0x1c, word);
            let first = payload(e, extra, 0x0c);
            e.mem.set_u32(target + 0x0c, first);
            e.call(0x0047_0470, &args![target + 0x20]);
            let mut node = extra.addr() + 0x20;
            while node != 0 {
                let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
                if e.mem.u32(item) != 0 {
                    let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
                    e.call(0x0090_5820, &args![target + 0x20, item]);
                }
                node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
            }
        }
        0x8d => {
            // Each weapon mod slot that is active in the source is set here.
            for mask in [1u8, 2, 4] {
                if fn_00411e20(e, extra, mask) {
                    e.call(0x0042_e380, &args![list, mask as u32]);
                }
            }
        }
        0x92 => {
            let existing = get_or_add_extra(e, this, 0x92, 0x14, |e, block| {
                fn_00411e40(e, Ptr::new(block)).addr()
            });
            e.vcall(existing.addr(), 0x0c, &args![extra]);
        }
        _ => {
            e.call(LOG_MESSAGE, &args![NO_COPY_MESSAGE, extra_type as u32]);
        }
    }
}

// Translated from 00411b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the 0x14-byte structure behind `ExtraLock` (its callers are
/// `DoorLock`, `SetFromDoorRef`, `AddLock`): byte +0x00, word +0x04, byte
/// +0x08, words +0x0C and +0x10, all zero. Returns `this`.
pub fn fn_00411b00(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u8(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

/// The constructors of `ExtraReflectedRefs` & co: the base constructor with
/// the type, the vtable, and an empty `BSSimpleList` (item and next null,
/// `0096a2d0`) at +0x0C.
pub(crate) fn construct_ref_list_extra(
    e: &mut Engine,
    this: Ptr<ExtraRefList>,
    extra_type: u32,
    vtable: u32,
) -> Ptr<ExtraRefList> {
    e.call(BS_EXTRA_DATA_INIT, &args![this, extra_type]);
    e.mem.set_u32(this.addr(), vtable);
    e.call(0x0096_a2d0, &args![this.byte_add(0x0c)]);
    this
}

// Translated from 00411b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraReflectedRefs` (type `0x65`; the exception-unwinding
/// frame is not translated). Returns `this`.
pub fn fn_00411b40(e: &mut Engine, this: Ptr<ExtraRefList>) -> Ptr<ExtraRefList> {
    construct_ref_list_extra(e, this, 0x65, VTABLE_EXTRA_REFLECTED_REFS)
}

// Translated from 00411bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraReflectedRefs::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`00434560`) and frees the object when bit 0 of `flags` is set.
/// Returns `this`.
pub fn extra_reflected_refs_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraRefList>,
    flags: u32,
) -> Ptr<ExtraRefList> {
    e.call(0x0043_4560, &args![this]);
    finish_scalar_deleting_destructor(e, this.cast(), flags).cast()
}

// Translated from 00411be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraReflectorRefs` (type `0x66`). Returns `this`.
pub fn fn_00411be0(e: &mut Engine, this: Ptr<ExtraRefList>) -> Ptr<ExtraRefList> {
    construct_ref_list_extra(e, this, 0x66, VTABLE_EXTRA_REFLECTOR_REFS)
}

// Translated from 00411c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraReflectorRefs::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`00434af0`) and frees the object when bit 0 of `flags` is set.
/// Returns `this`.
pub fn extra_reflector_refs_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraRefList>,
    flags: u32,
) -> Ptr<ExtraRefList> {
    e.call(0x0043_4af0, &args![this]);
    finish_scalar_deleting_destructor(e, this.cast(), flags).cast()
}

// Translated from 00411c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraWaterLightRefs` (type `0x84`). Returns `this`.
pub fn fn_00411c80(e: &mut Engine, this: Ptr<ExtraRefList>) -> Ptr<ExtraRefList> {
    construct_ref_list_extra(e, this, 0x84, VTABLE_EXTRA_WATER_LIGHT_REFS)
}

// Translated from 00411cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraWaterLightRefs::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`00434950`) and frees the object when bit 0 of `flags` is set.
/// Returns `this`.
pub fn extra_water_light_refs_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraRefList>,
    flags: u32,
) -> Ptr<ExtraRefList> {
    e.call(0x0043_4950, &args![this]);
    finish_scalar_deleting_destructor(e, this.cast(), flags).cast()
}

// Translated from 00411d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraLitWaterRefs` (type `0x85`). Returns `this`.
pub fn fn_00411d20(e: &mut Engine, this: Ptr<ExtraRefList>) -> Ptr<ExtraRefList> {
    construct_ref_list_extra(e, this, 0x85, VTABLE_EXTRA_LIT_WATER_REFS)
}

// Translated from 00411d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraLitWaterRefs::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`00434dc0`) and frees the object when bit 0 of `flags` is set.
/// Returns `this`.
pub fn extra_lit_water_refs_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ExtraRefList>,
    flags: u32,
) -> Ptr<ExtraRefList> {
    e.call(0x0043_4dc0, &args![this]);
    finish_scalar_deleting_destructor(e, this.cast(), flags).cast()
}

// Translated from 00411dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a 16-byte structure of four words (the payload `AddExtraCopy`
/// copies for type `0x76`) to zero. Returns `this`.
pub fn fn_00411dc0(e: &mut Engine, this: Ptr) -> Ptr {
    for index in 0..4 {
        e.mem.set_u32(this.addr() + index * 4, 0);
    }
    this
}

// Translated from 00411e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the 4-byte structure `AddExtraCopy` builds for type `0x72`
/// to the value `0x16`. Returns `this`.
pub fn fn_00411e00(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x16);
    this
}

// Translated from 00411e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any bit of `mask` is set in the byte at +0x0C of the extra data:
/// the weapon mod slots of `ExtraWeaponModFlags` (`cWeaponModsActive`, Xbox
/// PDB; its other caller is `GetWeaponModSlotActive`).
pub fn fn_00411e20(e: &mut Engine, this: Ptr<BSExtraData>, mask: u8) -> bool {
    e.mem.u8(this.addr() + 0x0c) & mask != 0
}

// Translated from 00411e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x92` extra data (`0x14` bytes, `AddExtraCopy`
/// copies it with its virtual slot `+0x0C`): a `BSExtraData` whose word at
/// +0x0C is zero. Returns `this`.
pub fn fn_00411e40(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, 0x92u32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_TYPE_92);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 00411e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the type `0x92` extra data: runs its
/// destructor (`00411ea0`, the next function of this unit) and frees the
/// object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00411e70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0041_1ea0, &args![this]);
    finish_scalar_deleting_destructor(e, this, flags)
}

// ---------------------------------------------------------------------------
// Second batch: list operations, the save writer, structures

// Translated from 00411ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the type `0x92` extra data (called by `fn_00411e70`): sets
/// its vtable, then runs the `BSExtraData` destructor (`0040ecb0`).
pub fn fn_00411ea0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_TYPE_92);
    e.call(BS_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 00411ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::CopyList` (Xbox PDB): empties this list of its copyable
/// extra data (`RemoveAllCopyableExtra`, deleting them), then adds a copy of
/// every extra data of `source` (`AddExtraCopy`, which skips the ones
/// `IsCopyableExtra` refuses). A null `source` only empties.
pub fn extra_data_list_copy_list(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    source: Ptr<ExtraDataList>,
) {
    extra_data_list_remove_all_copyable_extra(e, this, true);
    lock(e, NAME_COPY_LIST);
    if !source.is_null() {
        let mut current: Ptr<BSExtraData> = e.get(source, ExtraDataList::pHead).cast();
        while !current.is_null() {
            extra_data_list_add_extra_copy(e, this, current);
            current = get_next(e, current);
        }
    }
    unlock(e);
}

// Translated from 00411f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::IsCopyableExtra` (Xbox PDB): false for the extra data
/// types a copy leaves out (`01`, `02`, `04`, `0C`, `0F`, `10`, `11`, `15`,
/// `29`, `34`, `40`, `4B`, `52`, `54`, `55`, `58`: the jump table at
/// `00411f70`), true for every other type and for a null `extra`.
pub fn extra_data_list_is_copyable_extra(
    e: &mut Engine,
    _this: Ptr<ExtraDataList>,
    extra: Ptr<BSExtraData>,
) -> bool {
    if extra.is_null() {
        return true;
    }
    let extra_type = get_type(e, extra);
    !matches!(
        extra_type,
        0x01 | 0x02
            | 0x04
            | 0x0c
            | 0x0f
            | 0x10
            | 0x11
            | 0x15
            | 0x29
            | 0x34
            | 0x40
            | 0x4b
            | 0x52
            | 0x54
            | 0x55
            | 0x58
    )
}

// Translated from 00411fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveAllCopyableExtra` (Xbox PDB): unlinks every extra
/// data `IsCopyableExtra` accepts, deleting it when `destroy` is set. The type
/// bitmap is not touched.
pub fn extra_data_list_remove_all_copyable_extra(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    destroy: bool,
) {
    lock(e, NAME_REMOVE_ALL_COPYABLE);
    let mut next: Ptr<BSExtraData> = e.get(this, ExtraDataList::pHead).cast();
    let mut kept: Ptr<BSExtraData> = Ptr::NULL;
    base_extra_list_clear_last_extra_all(e, this.cast());
    while !next.is_null() {
        let current = next;
        next = get_next(e, current);
        if !extra_data_list_is_copyable_extra(e, this, current) {
            kept = current;
        } else {
            if current == e.get(this, ExtraDataList::pHead).cast() {
                e.set(this, ExtraDataList::pHead, next.cast());
            }
            if !kept.is_null() {
                set_next(e, kept, next);
            }
            if destroy {
                delete_extra(e, current);
            }
        }
    }
    unlock(e);
}

// Translated from 004120b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveNonPersistentCellData` (Xbox PDB): deletes every
/// extra data of the list except type `04` (always kept) and types `01` and
/// `02` (kept when `fn_004121b0` says so, that is when bit 2 of the thread
/// word at TLS `+0x294` is set); every other type, and type 0, is unlinked and
/// deleted. The type bitmap is not touched.
pub fn extra_data_list_remove_non_persistent_cell_data(e: &mut Engine, this: Ptr<ExtraDataList>) {
    lock(e, NAME_REMOVE_NON_PERSISTENT);
    base_extra_list_clear_last_extra_all(e, this.cast());
    let mut kept: Ptr<BSExtraData> = Ptr::NULL;
    let mut next: Ptr<BSExtraData> = e.get(this, ExtraDataList::pHead).cast();
    while !next.is_null() {
        let current = next;
        next = get_next(e, current);
        let extra_type = get_type(e, current);
        let keep = match extra_type {
            1 | 2 => {
                let thread_state = e.global::<u32>(THREAD_STATE_OWNER);
                fn_004121b0(e, thread_state)
            }
            4 => true,
            _ => false,
        };
        if keep {
            kept = current;
            continue;
        }
        if current == e.get(this, ExtraDataList::pHead).cast() {
            e.set(this, ExtraDataList::pHead, next.cast());
        }
        if !kept.is_null() {
            set_next(e, kept, next);
        }
        delete_extra(e, current);
    }
    unlock(e);
}

// Translated from 004121b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 2 of the thread-local word at `+0x294` of the TLS block (`this`, the
/// global at `011ddf38`, is not used): the flag `RemoveNonPersistentCellData`
/// consults.
pub fn fn_004121b0(e: &mut Engine, _this: u32) -> bool {
    let tls = e.tls();
    e.mem.u32(tls + THREAD_FLAGS_OFFSET) & 4 != 0
}

// Translated from 004121e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::CopyListForContainer` (Xbox PDB): copies into this list the
/// extra data of `source` that belong with an inventory item: type `0D` and
/// the types `16`, `17`, `1C`, `21`, `22`, `23`, `25`, `26`, `27`, `28`, `2F`,
/// `30`, `3F`, `47`, `4A`, `8D` (the jump table at `004122e4`). With
/// `copy_only` false the copied extra data is also removed from `source`
/// (type `0D` unlinked only, the others deleted, except type `30`), and the
/// walk restarts from the new head.
pub fn extra_data_list_copy_list_for_container(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    source: Ptr<ExtraDataList>,
    copy_only: bool,
) {
    lock(e, NAME_COPY_LIST_FOR_CONTAINER);
    let mut current: Ptr<BSExtraData> = e.get(source, ExtraDataList::pHead).cast();
    while !current.is_null() {
        let mut advance = true;
        let extra_type = get_type(e, current);
        match extra_type {
            0x0d => {
                extra_data_list_add_extra_copy(e, this, current);
                if !copy_only {
                    base_extra_list_remove_extra(e, source.cast(), current, false);
                    current = e.get(source, ExtraDataList::pHead).cast();
                    advance = false;
                }
            }
            0x16 | 0x17 | 0x1c | 0x21 | 0x22 | 0x23 | 0x25 | 0x26 | 0x27 | 0x28 | 0x2f | 0x30
            | 0x3f | 0x47 | 0x4a | 0x8d => {
                extra_data_list_add_extra_copy(e, this, current);
                if get_type(e, current) != 0x30 && !copy_only {
                    base_extra_list_remove_extra(e, source.cast(), current, true);
                    current = e.get(source, ExtraDataList::pHead).cast();
                    advance = false;
                }
            }
            _ => {}
        }
        if !current.is_null() && advance {
            current = get_next(e, current);
        }
    }
    unlock(e);
}

// Translated from 00412380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::DuplicateExtraListForContainer` (Xbox PDB): like
/// `CopyListForContainer` without removing anything from `source`, and with
/// type `24` added to the copied types (`0D`, `16`, `17`, `1C`, `21` to `28`,
/// `2F`, `30`, `3F`, `47`, `4A`, `8D`: the jump table at `00412404`).
pub fn extra_data_list_duplicate_extra_list_for_container(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    source: Ptr<ExtraDataList>,
) {
    lock(e, NAME_DUPLICATE_FOR_CONTAINER);
    let mut current: Ptr<BSExtraData> = e.get(source, ExtraDataList::pHead).cast();
    while !current.is_null() {
        let extra_type = get_type(e, current);
        if matches!(
            extra_type,
            0x0d | 0x16
                | 0x17
                | 0x1c
                | 0x21
                | 0x22
                | 0x23
                | 0x24
                | 0x25
                | 0x26
                | 0x27
                | 0x28
                | 0x2f
                | 0x30
                | 0x3f
                | 0x47
                | 0x4a
                | 0x8d
        ) {
            extra_data_list_add_extra_copy(e, this, current);
        }
        current = get_next(e, current);
    }
    unlock(e);
}

// Translated from 00412490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::CopyListForReference` (Xbox PDB): copies into this list the
/// extra data of `source` that a reference keeps when it is split: types
/// `16`, `1C`, `21`, `22`, `23`, `25`, `26`, `27`, `28`, `2F`, `30`, `3F`,
/// `8D` (copied; with `remove_from_source` also deleted from `source`, and the
/// walk restarts from its head) and type `0D`, the script: with
/// `remove_from_source` it is copied and unlinked from `source` (not deleted);
/// without it a new `ExtraScript` (`0x14` bytes) is built from
/// `source.GetScript()` (`00418800`), added to this list, and `00419f80` is
/// given the result of `005abf60` on the script. The jump table is at
/// `0041262c`. The compiler's exception-unwinding frame is not translated.
pub fn extra_data_list_copy_list_for_reference(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    source: Ptr<ExtraDataList>,
    remove_from_source: bool,
) {
    lock(e, NAME_COPY_LIST_FOR_REFERENCE);
    let mut current: Ptr<BSExtraData> = e.get(source, ExtraDataList::pHead).cast();
    while !current.is_null() {
        let mut advance = true;
        let extra_type = get_type(e, current);
        match extra_type {
            0x0d => {
                if !remove_from_source {
                    let script = e.call(GET_SCRIPT, &args![source]).u32();
                    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
                    let created = if block == 0 {
                        0
                    } else {
                        e.call(EXTRA_SCRIPT_INIT, &args![block, script]).u32()
                    };
                    base_extra_list_add_extra(e, this.cast(), Ptr::new(created));
                    let script = e.call(GET_SCRIPT, &args![source]).u32();
                    let compiled = e.call(SCRIPT_GET_RESULT, &args![script]).u32();
                    e.call(EXTRA_DATA_LIST_SET_SCRIPT_RESULT, &args![this, compiled]);
                } else {
                    extra_data_list_add_extra_copy(e, this, current);
                    base_extra_list_remove_extra(e, source.cast(), current, false);
                    current = e.get(source, ExtraDataList::pHead).cast();
                    advance = false;
                }
            }
            0x16 | 0x1c | 0x21 | 0x22 | 0x23 | 0x25 | 0x26 | 0x27 | 0x28 | 0x2f | 0x30 | 0x3f
            | 0x8d => {
                extra_data_list_add_extra_copy(e, this, current);
                if remove_from_source {
                    base_extra_list_remove_extra(e, source.cast(), current, true);
                    current = e.get(source, ExtraDataList::pHead).cast();
                }
                advance = !remove_from_source;
            }
            _ => {}
        }
        if !current.is_null() && advance {
            current = get_next(e, current);
        }
    }
    unlock(e);
}

/// True when `list` has no extra data of the type of `extra`, or has one whose
/// virtual `Compare` (slot `+4`) says it differs from `extra`.
pub(crate) fn differs_from_extra_in(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra: Ptr<BSExtraData>,
) -> bool {
    let extra_type = get_type(e, extra);
    let found = base_extra_list_get_extra_data(e, list.cast(), extra_type);
    if found.is_null() {
        return true;
    }
    e.vcall(found.addr(), 4, &args![extra]).bool()
}

// Translated from 004126c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::CompareListForContainer` (Xbox PDB): true when `other` (an
/// inventory entry's list) differs from this list. Unless
/// `allow_script_and_owner` is set, an extra data of type `0D` or `21` in
/// `other` is a difference at once. With `skip_type_16`, type `16` is
/// passed over. Types `24` and `4A` are always passed over; for each other
/// extra data in `other` this list must hold one of the same type (else a
/// difference) that `Compare`s equal.
pub fn extra_data_list_compare_list_for_container(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    other: Ptr<ExtraDataList>,
    allow_script_and_owner: bool,
    skip_type_16: bool,
) -> bool {
    lock(e, NAME_COMPARE_LIST_FOR_CONTAINER);
    let mut differs = false;
    let mut current: Ptr<BSExtraData> = e.get(other, ExtraDataList::pHead).cast();
    while !current.is_null() {
        if !allow_script_and_owner {
            if get_type(e, current) == 0x0d {
                differs = true;
                break;
            }
            if get_type(e, current) == 0x21 {
                differs = true;
                break;
            }
        }
        if get_type(e, current) == 0x16 && skip_type_16 {
            current = get_next(e, current);
            continue;
        }
        if get_type(e, current) != 0x24 && get_type(e, current) != 0x4a {
            let extra_type = get_type(e, current);
            let found = base_extra_list_get_extra_data(e, this.cast(), extra_type);
            if found.is_null() {
                differs = true;
                break;
            }
            if e.vcall(found.addr(), 4, &args![current]).bool() {
                differs = true;
                break;
            }
        }
        current = get_next(e, current);
    }
    unlock(e);
    differs
}

// Translated from 004127e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::CompareList` (Xbox PDB): true when the two lists differ in
/// their copyable extra data. Every copyable extra data of `other` must have a
/// `Compare`-equal counterpart of its type in this list; then the same the
/// other way round; and when the two lists hold a different number of extra
/// data, every copyable extra data of this list must exist in `other`.
pub fn extra_data_list_compare_list(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    other: Ptr<ExtraDataList>,
) -> bool {
    lock(e, NAME_COMPARE_LIST);
    let mut differs = false;
    let mut current: Ptr<BSExtraData> = e.get(other, ExtraDataList::pHead).cast();
    while !current.is_null() {
        if extra_data_list_is_copyable_extra(e, this, current)
            && differs_from_extra_in(e, this, current)
        {
            differs = true;
            break;
        }
        current = get_next(e, current);
    }
    if !differs {
        let mut current: Ptr<BSExtraData> = e.get(this, ExtraDataList::pHead).cast();
        while !current.is_null() {
            if extra_data_list_is_copyable_extra(e, this, current)
                && differs_from_extra_in(e, other, current)
            {
                differs = true;
                break;
            }
            current = get_next(e, current);
        }
    }
    let this_count = base_extra_list_items_in_list(e, this.cast());
    let other_count = base_extra_list_items_in_list(e, other.cast());
    if this_count != other_count {
        let mut current: Ptr<BSExtraData> = e.get(this, ExtraDataList::pHead).cast();
        while !current.is_null() {
            if extra_data_list_is_copyable_extra(e, this, current) {
                let extra_type = get_type(e, current);
                if base_extra_list_get_extra_data(e, other.cast(), extra_type).is_null() {
                    differs = true;
                    break;
                }
            }
            current = get_next(e, current);
        }
    }
    unlock(e);
    differs
}

// ---------------------------------------------------------------------------
// Small structures: copies, byte swaps, constructors

// Translated from 00413f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at +8 of the object: for the save writer's
/// occlusion plane, its centre (`kCenter`, Xbox PDB `BSOcclusionPlane`, three
/// floats).
pub fn fn_00413f40(_e: &mut Engine, this: u32) -> u32 {
    this.wrapping_add(8)
}

// Translated from 00413f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 36 bytes (nine words, a 3x3 matrix) at +0x1c of the object to
/// `out`: the rotation of an occlusion plane (`kRotation`, Xbox PDB, which
/// puts it at +0x20). Returns `out`.
pub fn fn_00413f60(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_words(e, this.addr() + 0x1c, out.addr(), 0, 36);
    out
}

// Translated from 00413f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 16 bytes (four words) at +8 of the object to `out`: the colour
/// of a `BGSPrimitive` (`Color`, an `NiColorA`, Xbox PDB). Returns `out`.
pub fn fn_00413f90(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_words(e, this.addr() + 8, out.addr(), 0, 16);
    out
}

// Translated from 00413fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 12 bytes (three words) at +0x18 of the object to `out`: the
/// radii of a `BGSPrimitive` (`Radii`, an `NiPoint3`, Xbox PDB). Returns
/// `out`.
pub fn fn_00413fc0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_words(e, this.addr() + 0x18, out.addr(), 0, 12);
    out
}

// Translated from 00413ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the `float` that `source` points at into the word at `this`
/// (through the x87 stack, so a signalling NaN comes out quiet). Returns
/// `this`.
pub fn fn_00413ff0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let bits = x87_quiet_bits(e.mem.u32(source.addr()));
    e.mem.set_u32(this.addr(), bits);
    this
}

// Translated from 00414010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a two-word structure to an empty list node: item 0, next 0.
/// (The engine map names this `BSSimpleList<REF_ACTIVATE_DATA_P>::AddHead`
/// because the linker folded identical code; the body is a constructor of a
/// node with a null item and a null next.) Returns `this`.
pub fn fn_00414010(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// Byte-swaps in place the words `[0, count)` of the structure at `this`
/// with `Swap32(ptr, 0)` (`00401080`), in order.
pub(crate) fn swap_words(e: &mut Engine, this: Ptr, count: u32) {
    for index in 0..count {
        e.call(SWAP_DWORD, &args![this.addr() + index * 4, 0u32]);
    }
}

// Translated from 00414030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps (`Swap32`, `00401080`) the seven words at +0 to +0x18 of a
/// `REF_DECAL_DATA` (`0x1c` bytes, Xbox PDB).
pub fn fn_00414030(e: &mut Engine, this: Ptr) {
    swap_words(e, this, 7);
}

// Translated from 004140f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the nine words (`0x24` bytes) of the occlusion plane record
/// (extents, centre, axis, angle).
pub fn fn_004140f0(e: &mut Engine, this: Ptr) {
    swap_words(e, this, 9);
}

// Translated from 004141e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the word at +0 (`Swap32`) and the 16-bit value at +4
/// (`Swap16`, `00407a90`).
pub fn fn_004141e0(e: &mut Engine, this: Ptr) {
    e.call(SWAP_DWORD, &args![this, 0u32]);
    e.call(SWAP_WORD, &args![this.addr() + 4, 0u32]);
}

// Translated from 00414220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the four words at +0 to +0x0c (the first of a `RADIO_DATA` or
/// of a `WORLD_LOCATION`).
pub fn fn_00414220(e: &mut Engine, this: Ptr) {
    swap_words(e, this, 4);
}

// Translated from 00414290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the eight words (`0x20` bytes) of the primitive record (radii,
/// colour, type).
pub fn fn_00414290(e: &mut Engine, this: Ptr) {
    swap_words(e, this, 8);
}

// Translated from 00414370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps four words in a loop over an index (the occlusion plane
/// reference record).
pub fn fn_00414370(e: &mut Engine, this: Ptr) {
    swap_words(e, this, 4);
}

// Translated from 004143c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a two-word structure to zero (word 1 first, then word 0): the
/// portal reference record. Returns `this`.
pub fn fn_004143c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr(), 0);
    this
}

// Translated from 004143f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 32-byte primitive record (radii as an `NiPoint3` at +0,
/// colour as an `NiColorA` at +0x0c, type at +0x1c): the point's constructor
/// does nothing, the colour's (`00414430`) stores four zeros. Returns `this`.
pub fn fn_004143f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SIMPLE_LIST_ITEM, &args![this]);
    fn_00414430(e, this.byte_add(0x0c), 0.0, 0.0, 0.0, 0.0);
    this
}

// Translated from 00414430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores four `float`s at +0 to +0x0c (through the x87 stack): the
/// constructor of an `NiColorA`. Returns `this`.
pub fn fn_00414430(e: &mut Engine, this: Ptr, a: f32, b: f32, c: f32, d: f32) -> Ptr {
    for (index, value) in [a, b, c, d].into_iter().enumerate() {
        let bits = x87_quiet_bits(value.to_bits());
        e.mem.set_u32(this.addr() + index as u32 * 4, bits);
    }
    this
}

// Translated from 00414470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 36-byte occlusion plane structure: only runs the
/// do-nothing member constructors (`006815c0`) at +0, +8 and +0x14. Returns
/// `this`.
pub fn fn_00414470(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SIMPLE_LIST_ITEM, &args![this]);
    e.call(SIMPLE_LIST_ITEM, &args![this.addr() + 8]);
    e.call(SIMPLE_LIST_ITEM, &args![this.addr() + 0x14]);
    this
}

// Translated from 00416870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores three `float`s at +0, +4 and +8 (through the x87 stack): the
/// constructor of an `NiPoint3`. Returns `this`.
pub fn fn_00416870(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32) -> Ptr {
    for (index, value) in [x, y, z].into_iter().enumerate() {
        let bits = x87_quiet_bits(value.to_bits());
        e.mem.set_u32(this.addr() + index as u32 * 4, bits);
    }
    this
}

// Translated from 004169a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `sincos`: stores the sine of `angle` at `sine` and the cosine at `cosine`
/// (both `float`), through the x87 `FSINCOS` and two scratch floats (`FSTP`
/// the cosine, then the sine). For a finite `|angle| >= 2^63` `FSINCOS` sets
/// C2 and pushes nothing: the first `FSTP` stores the angle itself as the
/// cosine and the second pops an empty register, storing the default NaN
/// `0xffc00000` as the sine.
pub fn fn_004169a0(e: &mut Engine, angle: f32, sine: Ptr, cosine: Ptr) {
    let bits = angle.to_bits();
    let loaded = x87_load32(bits);
    let (sine_slot, cosine_slot) = if loaded.is_finite() && loaded.abs() >= TWO_POW_63 {
        (0xffc0_0000, x87_store32(loaded))
    } else {
        let (sin, cos) = x87_fsincos(X87Ext::load32(bits));
        (sin.store32(), cos.store32())
    };
    e.mem.set_u32(sine.addr(), x87_quiet_bits(sine_slot));
    e.mem.set_u32(cosine.addr(), x87_quiet_bits(cosine_slot));
}

/// `2^63`: the bound of the operand of `FSIN`/`FCOS`/`FSINCOS` (Intel SDM vol.
/// 2: out of range for `|x| >= 2^63`).
const TWO_POW_63: f64 = 9_223_372_036_854_775_808.0;

// Translated from 004169d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x1D8 of the object (the region list of the data handler
/// singleton, read by the loader through `011c3f2c`).
pub fn fn_004169d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x1d8)
}

/// The two flag bytes the setters of the occlusion plane set.
pub(crate) const PLANE_DIRTY_VERTICES: u32 = 0xe4;
pub(crate) const PLANE_DIRTY_PLANES: u32 = 0xe5;

// Translated from 004169f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane` centre setter: copies the three words at `center` to
/// +8 of the plane (`kCenter`, Xbox PDB), and marks the cached vertices
/// (+0xE4) and planes (+0xE5) dirty.
pub fn fn_004169f0(e: &mut Engine, this: Ptr, center: Ptr) {
    copy_block(e, center.addr(), this.addr() + 8, 12);
    e.mem.set_u8(this.addr() + PLANE_DIRTY_VERTICES, 1);
    e.mem.set_u8(this.addr() + PLANE_DIRTY_PLANES, 1);
}

// Translated from 00416a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane` size setter: copies the two words at `half_extents` to
/// +0x14 of the plane (`kHalfExtents`, Xbox PDB), and marks the cached
/// vertices and planes dirty.
pub fn fn_00416a30(e: &mut Engine, this: Ptr, half_extents: Ptr) {
    let first = e.mem.u32(half_extents.addr());
    let second = e.mem.u32(half_extents.addr() + 4);
    e.mem.set_u32(this.addr() + 0x14, first);
    e.mem.set_u32(this.addr() + 0x18, second);
    e.mem.set_u8(this.addr() + PLANE_DIRTY_VERTICES, 1);
    e.mem.set_u8(this.addr() + PLANE_DIRTY_PLANES, 1);
}

// Translated from 00416a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane` rotation setter: copies the nine words (a 3x3 matrix)
/// at `rotation` to +0x1C of the plane (`kRotation`, Xbox PDB), and marks the
/// cached vertices and planes dirty.
pub fn fn_00416a70(e: &mut Engine, this: Ptr, rotation: Ptr) {
    copy_block(e, rotation.addr(), this.addr() + 0x1c, 36);
    e.mem.set_u8(this.addr() + PLANE_DIRTY_VERTICES, 1);
    e.mem.set_u8(this.addr() + PLANE_DIRTY_PLANES, 1);
}

// Translated from 00416ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes an 8-byte structure (a word and a 16-bit value, the old
/// navmesh portal record the loader reads) to zero. Returns `this`.
pub fn fn_00416ab0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u16(this.addr() + 4, 0);
    this
}

// Translated from 00416ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the type `0x5A` extra data (`0x14` bytes, the navmesh
/// portal extra data): a `BSExtraData` whose member at +0x0C is copied (`word`
/// then the 16-bit value in the low half of `short_part`) from the 8-byte
/// value the caller passes by value (`0069a690`). Returns `this`. The
/// compiler's exception-unwinding frame is not translated.
pub fn fn_00416ad0(e: &mut Engine, this: Ptr, word: u32, short_part: u32) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, 0x5au32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_TYPE_5A);
    e.with_stack(8, |e, value| {
        e.mem.set_u32(value.addr(), word);
        e.mem.set_u32(value.addr() + 4, short_part);
        e.call(NAVMESH_PORTAL_COPY, &args![this.addr() + 0x0c, value]);
    });
    this
}

// Translated from 00416b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `RADIO_DATA` structure (`0x10` bytes, Xbox PDB):
/// `fRadius` 0.0, `eRangeType` 1, `fStaticPct` 0.0, `pPositionRef` null.
/// Returns `this`.
pub fn fn_00416b40(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 1);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 00416b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `RoomLinkedRefData` structure (`0x14` bytes, Xbox PDB):
/// the two list heads (`PortalList` at +0 and `RoomList` at +8) are built
/// with `0096a2d0`. Returns `this`. The compiler's exception-unwinding frame
/// is not translated.
pub fn fn_00416b80(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SIMPLE_LIST_INIT, &args![this]);
    e.call(SIMPLE_LIST_INIT, &args![this.addr() + 8]);
    this
}

// ---------------------------------------------------------------------------
// The save writer

/// Reads the form id of the form at `form` (`TESForm::iFormID`, +0x0C, via
/// the getter `0084e3a0`).
pub(crate) fn form_id(e: &mut Engine, form: u32) -> u32 {
    e.call(FORM_ID, &args![form]).u32()
}

/// Adds a chunk holding one word (`TESForm::AddChunk(tag, value)`,
/// `00485910`; the word is byte-swapped first on a big-endian target).
pub(crate) fn add_chunk_word(e: &mut Engine, tag: u32, value: u32) {
    e.call(ADD_CHUNK_WORD, &args![tag, value]);
}

/// Adds a chunk holding one byte (`004858f0`).
pub(crate) fn add_chunk_byte(e: &mut Engine, tag: u32, value: u8) {
    e.call(ADD_CHUNK_BYTE, &args![tag, value]);
}

/// Adds a chunk with no data (`004856d0`).
pub(crate) fn add_chunk_empty(e: &mut Engine, tag: u32) {
    e.call(ADD_CHUNK_EMPTY, &args![tag]);
}

/// Adds a chunk of `size` bytes at `data` (`TESForm::__AddChunkData`,
/// `00485990`).
pub(crate) fn add_chunk_data(e: &mut Engine, tag: u32, data: u32, size: u32) {
    e.call(ADD_CHUNK_DATA, &args![tag, data, size]);
}

/// Adds a chunk holding `count` words at `data` (`TESForm::AddChunkArray`,
/// `00485710`).
pub(crate) fn add_chunk_array(e: &mut Engine, tag: u32, data: u32, count: u32) {
    e.call(ADD_CHUNK_ARRAY, &args![tag, data, count]);
}

/// The save writer's "target is big-endian" flag (byte at `011c54ba`, read
/// through `00401500`).
pub(crate) fn is_big_endian(e: &mut Engine) -> bool {
    e.call(IS_BIG_ENDIAN, &args![]).bool()
}

/// The item of a `BSSimpleList` node (`006815c0` returns the node's own
/// address, the item is the word there).
pub(crate) fn list_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

/// The next node of a `BSSimpleList` node (`00726070`).
pub(crate) fn list_next(e: &mut Engine, node: u32) -> u32 {
    e.call(SIMPLE_LIST_NEXT, &args![node]).u32()
}

/// Adds the chunk `tag` with the form id of `form`.
pub(crate) fn add_form_chunk(e: &mut Engine, tag: u32, form: u32) {
    let id = form_id(e, form);
    add_chunk_word(e, tag, id);
}

/// Adds the chunk `tag` with the form id of `form` when it is not null.
pub(crate) fn add_optional_form_chunk(e: &mut Engine, tag: u32, form: u32) {
    if form != 0 {
        add_form_chunk(e, tag, form);
    }
}

/// Adds `tag` for a structure at `data` of `size` bytes, byte-swapping it
/// around the call with `swap` on a big-endian target.
pub(crate) fn add_swapped_chunk_data(
    e: &mut Engine,
    tag: u32,
    data: Ptr,
    size: u32,
    swap: fn(&mut Engine, Ptr),
) {
    if is_big_endian(e) {
        swap(e, data);
    }
    add_chunk_data(e, tag, data.addr(), size);
    if is_big_endian(e) {
        swap(e, data);
    }
}

/// Byte-swap of a single word at `this` (`Swap32`, `00503210`'s body).
pub(crate) fn swap_word_at(e: &mut Engine, this: Ptr) {
    e.call(SWAP_WORD_AT, &args![this]);
}

/// Byte-swap of the word at +4 (`0060ce80`, the lock structure's key).
pub(crate) fn swap_word_at_4(e: &mut Engine, this: Ptr) {
    e.call(SWAP_WORD_AT_4, &args![this]);
}

/// Byte-swap of two words (`00462230`).
pub(crate) fn swap_two_words(e: &mut Engine, this: Ptr) {
    e.call(SWAP_TWO_WORDS, &args![this]);
}

// Translated from 00412970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The save writer of the list (Xbox name not in the map): walks the chain
/// under the lock and, for each extra data of a type that is saved, adds the
/// chunk(s) that `ExtraDataList::Load` (`004144a0`) reads back. The types
/// are dispatched by the jump table at `00413dc8` (type `3` to `0x92`); the
/// chunk names are the four letters the exe pushes (`b"XCWT"` is
/// `0x54574358`). Forms are saved as their form id (`0084e3a0`); structures
/// the game keeps in memory are copied to a local record, byte-swapped around
/// the call when the target is big-endian (the flag at `011c54ba`; never set
/// in the PC build) and added with `TESForm::__AddChunkData` (`00485990`).
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_00412970(e: &mut Engine, this: Ptr<ExtraDataList>) {
    lock(e, 0);
    let mut current: Ptr<BSExtraData> = e.get(this, ExtraDataList::pHead).cast();
    while !current.is_null() {
        let extra_type = get_type(e, current);
        save_extra_data(e, this, current, extra_type);
        current = get_next(e, current);
    }
    unlock(e);
}

/// One arm of the save writer's switch.
pub(crate) fn save_extra_data(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra: Ptr<BSExtraData>,
    extra_type: u8,
) {
    // The word at +0x0C, which most of the arms use; the few arms of
    // extra data without one (type 0x61, 0x80) never look at it, so it is
    // read as 0 when that address is not mapped.
    let mut word = [0u8; 4];
    let value = match e.mem.try_read(extra.addr() + 0x0c, &mut word) {
        Ok(()) => u32::from_le_bytes(word),
        Err(_) => 0,
    };
    match extra_type {
        // EXTRA_WATERTYPE
        0x03 => add_form_chunk(e, chunk_tag(b"XCWT"), value),
        // EXTRA_REGIONLIST: a list of regions saved as an array of form ids.
        0x04 => {
            let mut node = if value == 0 { 0 } else { value + 4 };
            let count = e.call(LIST_COUNT, &args![node]).u32();
            if count != 0 {
                let size = count.saturating_mul(4);
                let buffer = e.call(OPERATOR_NEW, &args![size]).u32();
                let mut index = 0;
                while node != 0 {
                    let region = list_item(e, node);
                    let id = form_id(e, region);
                    e.mem.set_u32(buffer.wrapping_add(index * 4), id);
                    index += 1;
                    node = list_next(e, node);
                }
                add_chunk_array(e, chunk_tag(b"XCLR"), buffer, count);
                e.call(OPERATOR_DELETE, &args![buffer]);
            }
        }
        // EXTRA_CELLMUSICTYPE
        0x07 => add_form_chunk(e, chunk_tag(b"XCMO"), value),
        // EXTRA_CLIMATE
        0x08 => add_form_chunk(e, chunk_tag(b"XCCM"), value),
        // EXTRA_ACTION
        0x0e => {
            let action = e.mem.u8(extra.addr() + 0x0c);
            add_chunk_word(e, chunk_tag(b"XACT"), action as u32);
        }
        // EXTRA_DISTANTDATA: three words.
        0x13 => e.with_stack(12, |e, record| {
            copy_block(e, extra.addr() + 0x0c, record.addr(), 12);
            add_chunk_array(e, chunk_tag(b"XLOD"), record.addr(), 3);
        }),
        // EXTRA_RAGDOLLDATA: RagDollData::Save (`004d93c0`).
        0x14 => {
            e.call(RAGDOLL_DATA_SAVE, &args![value]);
        }
        // EXTRA_PACKAGESTARTLOC: WORLD_LOCATION (0x14 bytes): the location
        // form as a form id, then the three words at +0x10 of the extra data;
        // the last word (`fZRot`) is never set.
        0x18 => e.with_stack(0x14, |e, record| {
            e.call(PATH_LOCATION_INIT, &args![record]);
            let id = form_id(e, value);
            e.mem.set_u32(record.addr(), id);
            copy_block(e, extra.addr() + 0x10, record.addr() + 4, 12);
            add_swapped_chunk_data(e, chunk_tag(b"XPSL"), record, 0x14, fn_00414220);
        }),
        // EXTRA_LEVCREA_MOD
        0x1e => add_chunk_word(e, chunk_tag(b"XLCM"), value),
        // EXTRA_OWNERSHIP
        0x21 => add_form_chunk(e, chunk_tag(b"XOWN"), value),
        // EXTRA_GLOBAL
        0x22 => add_form_chunk(e, chunk_tag(b"XGLB"), value),
        // EXTRA_RANK
        0x23 => add_chunk_word(e, chunk_tag(b"XRNK"), value),
        // EXTRA_COUNT: a signed 16-bit value.
        0x24 => {
            let count = e.mem.i16(extra.addr() + 0x0c);
            add_chunk_word(e, chunk_tag(b"XCNT"), count as i32 as u32);
        }
        // EXTRA_HEALTH: a float saved as an integer (`_ftol2`).
        0x25 => {
            let health = e.mem.f32(extra.addr() + 0x0c);
            let whole = e.call(FTOL, &args![health as f64]).u32();
            add_chunk_word(e, chunk_tag(b"XHLT"), whole);
        }
        // EXTRA_USES
        0x26 => {
            let uses = e.mem.u8(extra.addr() + 0x0c);
            add_chunk_word(e, chunk_tag(b"XUSE"), uses as u32);
        }
        // EXTRA_TIMELEFT
        0x27 => add_chunk_word(e, chunk_tag(b"XTIM"), x87_quiet_bits(value)),
        // EXTRA_CHARGE
        0x28 => add_chunk_word(e, chunk_tag(b"XCHG"), x87_quiet_bits(value)),
        // EXTRA_LOCK: a REFR_LOCK (0x14 bytes) with the key as a form id.
        0x2a => e.with_stack(0x14, |e, record| {
            fn_00411b00(e, record);
            let lock_data = value;
            let flags = e.mem.u8(lock_data + 8);
            e.mem.set_u8(record.addr() + 8, flags);
            let base_level = e.mem.u8(lock_data);
            e.mem.set_u8(record.addr(), base_level);
            let key = e.mem.u32(lock_data + 4);
            let key_id = if key == 0 { 0 } else { form_id(e, key) };
            e.mem.set_u32(record.addr() + 4, key_id);
            add_swapped_chunk_data(e, chunk_tag(b"XLOC"), record, 0x14, |e, record| {
                swap_word_at_4(e, record)
            });
        }),
        // EXTRA_TELEPORT
        0x2b => {
            e.call(TELEPORT_DATA_SAVE, &args![value]);
        }
        // EXTRA_MAPMARKER
        0x2c => {
            add_chunk_empty(e, chunk_tag(b"XMRK"));
            e.call(MAP_MARKER_DATA_SAVE, &args![value]);
        }
        // EXTRA_SEED: the byte `GetSeed` (`00418b40`) returns for the list.
        0x31 => {
            let seed = e.call(EXTRA_DATA_LIST_GET_SEED, &args![list]).u8();
            add_chunk_byte(e, chunk_tag(b"XSED"), seed);
        }
        // EXTRA_ENABLESTATEPARENT: the parent as a form id and its flags byte.
        0x37 => {
            if value != 0 {
                e.with_stack(8, |e, record| {
                    let id = form_id(e, value);
                    e.mem.set_u32(record.addr(), id);
                    let flags = e.mem.u8(extra.addr() + 0x10);
                    e.mem.set_u8(record.addr() + 4, flags);
                    add_swapped_chunk_data(e, chunk_tag(b"XESP"), record, 8, swap_word_at);
                });
            }
        }
        // EXTRA_TELEPORTMARKER
        0x3b => add_optional_form_chunk(e, chunk_tag(b"XRTM"), value),
        // EXTRA_MERCHANTCONTAINER
        0x3c => add_optional_form_chunk(e, chunk_tag(b"XMRC"), value),
        // EXTRA_POISON
        0x3f => add_form_chunk(e, chunk_tag(b"XPSN"), value),
        // EXTRA_XTARGET
        0x44 => add_optional_form_chunk(e, chunk_tag(b"XTRG"), value),
        // EXTRA_LINKED_REF
        0x51 => add_optional_form_chunk(e, chunk_tag(b"XLKR"), value),
        // EXTRA_ACTIVATE_REF
        0x53 => save_activate_ref(e, list, extra),
        // EXTRA_DECAL_REFS: each REF_DECAL_DATA (0x1c bytes) with the decal
        // reference as a form id.
        0x57 => {
            let mut node = extra.addr() + 0x0c;
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let decal = list_item(e, node);
                e.with_stack(0x1c, |e, record| {
                    e.call(DECAL_DATA_INIT, &args![record]);
                    e.call(MEMCPY, &args![record, decal, 0x1cu32]);
                    let reference = e.mem.u32(decal);
                    let id = form_id(e, reference);
                    e.mem.set_u32(record.addr(), id);
                    add_swapped_chunk_data(e, chunk_tag(b"XDCR"), record, 0x1c, fn_00414030);
                });
                node = list_next(e, node);
            }
        }
        // EXTRA_IMAGESPACE
        0x59 => add_form_chunk(e, chunk_tag(b"XCIM"), value),
        // EXTRA_NAVMESH_PORTAL: an 8-byte copy of the member with the form id
        // of its first word.
        0x5a => {
            let member = extra.addr() + 0x0c;
            e.with_stack(8, |e, record| {
                e.call(NAVMESH_PORTAL_COPY, &args![record, member]);
                let first = e.mem.u32(member);
                if first != 0 {
                    let id = form_id(e, first);
                    e.mem.set_u32(record.addr(), id);
                    add_swapped_chunk_data(e, chunk_tag(b"XNDP"), record, 8, fn_004141e0);
                }
            });
        }
        // EXTRA_RADIUS
        0x5c => add_chunk_word(e, chunk_tag(b"XRDS"), x87_quiet_bits(value)),
        // EXTRA_RADIATION
        0x5d => add_chunk_word(e, chunk_tag(b"XRAD"), x87_quiet_bits(value)),
        // EXTRA_MULTIBOUND
        0x61 => add_chunk_empty(e, chunk_tag(b"XMBP")),
        // EXTRA_MULTIBOUND_DATA: MultiBoundMarkerData::Save (`00438f90`).
        0x62 => {
            if value != 0 {
                e.call(MULTIBOUND_MARKER_DATA_SAVE, &args![value]);
            }
        }
        // EXTRA_MULTIBOUND_REF
        0x63 => add_optional_form_chunk(e, chunk_tag(b"XMBR"), value),
        // EXTRA_REFLECTOR_REFS: each entry is a reference and a word (8
        // bytes), saved with the reference as a form id.
        0x66 => {
            let mut node = extra.addr() + 0x0c;
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let data = list_item(e, node);
                e.with_stack(8, |e, record| {
                    e.call(SIMPLE_LIST_INIT, &args![record]);
                    e.call(MEMCPY, &args![record, data, 8u32]);
                    let reference = e.mem.u32(data);
                    let id = form_id(e, reference);
                    e.mem.set_u32(record.addr(), id);
                    add_swapped_chunk_data(e, chunk_tag(b"XPWR"), record, 8, swap_two_words);
                });
                node = list_next(e, node);
            }
        }
        // EXTRA_EMITTANCE_SOURCE
        0x67 => add_optional_form_chunk(e, chunk_tag(b"XEMI"), value),
        // EXTRA_RADIO_DATA: RADIO_DATA (0x10 bytes), the position reference
        // at +0x0C swapped for its form id while it is written.
        0x68 => {
            let data = extra.addr() + 0x0c;
            let position_ref = e.mem.u32(extra.addr() + 0x18);
            if position_ref != 0 {
                let id = form_id(e, position_ref);
                e.mem.set_u32(data + 0x0c, id);
            }
            add_swapped_chunk_data(e, chunk_tag(b"XRDO"), Ptr::new(data), 0x10, fn_00414220);
            e.mem.set_u32(data + 0x0c, position_ref);
        }
        // EXTRA_PRIMITIVE: radii, colour and type of the BGSPrimitive.
        0x6b => e.with_stack(0x20, |e, record| {
            fn_004143f0(e, record);
            let primitive = Ptr::new(value);
            e.with_stack(12, |e, radii| {
                let radii = fn_00413fc0(e, primitive, radii);
                copy_block(e, radii.addr(), record.addr(), 12);
            });
            e.with_stack(16, |e, color| {
                let color = fn_00413f90(e, primitive, color);
                copy_block(e, color.addr(), record.addr() + 0x0c, 16);
            });
            let primitive_type = list_next(e, value);
            e.mem.set_u32(record.addr() + 0x1c, primitive_type);
            add_swapped_chunk_data(e, chunk_tag(b"XPRM"), record, 0x20, fn_00414290);
        }),
        // EXTRA_AMMO: the ammo as a form id (0 if none), then the count word.
        0x6e => {
            let count = e.mem.u32(extra.addr() + 0x10);
            let ammo_id = if value == 0 { 0 } else { form_id(e, value) };
            add_chunk_word(e, chunk_tag(b"XAMT"), ammo_id);
            add_chunk_word(e, chunk_tag(b"XAMC"), count);
        }
        // EXTRA_PATROL_REF_DATA: the time at the reference, then the
        // PackageEventAction (`0067dc80`, tag `XPPA`).
        0x6f => {
            e.with_stack(4, |e, record| {
                fn_00413ff0(e, record, Ptr::new(value));
                add_swapped_chunk_data(e, chunk_tag(b"XPRD"), record, 4, swap_word_at);
            });
            e.call(
                PACKAGE_EVENT_ACTION_SAVE,
                &args![value.wrapping_add(4), chunk_tag(b"XPPA")],
            );
        }
        // EXTRA_OCCLUSION_PLANE and EXTRA_PORTAL: the plane's half extents,
        // centre and rotation as an axis and an angle.
        0x71 => save_plane(e, extra, chunk_tag(b"XOCP")),
        0x78 => save_plane(e, extra, chunk_tag(b"XPTL")),
        // EXTRA_COLLISION_DATA: the word the extra data points at.
        0x72 => {
            let handle = Ptr::new(value);
            if is_big_endian(e) {
                swap_word_at(e, handle);
            }
            add_chunk_data(e, chunk_tag(b"XTRI"), value, 4);
            if is_big_endian(e) {
                swap_word_at(e, handle);
            }
        }
        // EXTRA_ENCOUNTERZONE
        0x74 => add_optional_form_chunk(e, chunk_tag(b"XEZN"), value),
        // EXTRA_OCCLUSION_PLANE_REF_DATA: four linked references.
        0x76 => e.with_stack(16, |e, record| {
            fn_00411dc0(e, record);
            for index in 0..4 {
                let reference = e.mem.u32(value + index * 4);
                let id = if reference == 0 {
                    0
                } else {
                    form_id(e, reference)
                };
                e.mem.set_u32(record.addr() + index * 4, id);
            }
            add_swapped_chunk_data(e, chunk_tag(b"XORD"), record, 0x10, fn_00414370);
        }),
        // EXTRA_PORTAL_REF_DATA: two linked references.
        0x77 => e.with_stack(8, |e, record| {
            fn_004143c0(e, record);
            for index in 0..2 {
                let reference = e.mem.u32(value + index * 4);
                let id = if reference == 0 {
                    0
                } else {
                    form_id(e, reference)
                };
                e.mem.set_u32(record.addr() + index * 4, id);
            }
            add_swapped_chunk_data(e, chunk_tag(b"XPOD"), record, 8, swap_two_words);
        }),
        // EXTRA_HEALTH_PERC
        0x7a => add_chunk_word(e, chunk_tag(b"XHLP"), x87_quiet_bits(value)),
        // EXTRA_ROOM_REF_DATA: RoomLinkedRefData, the count of linked portals
        // with the master flag in bit 16, then each linked room as a form id.
        0x7b => {
            let mut count = e.call(LIST_COUNT, &args![value + 8]).u32();
            if e.mem.u8(value + 0x10) != 0 {
                count |= 0x10000;
            }
            add_chunk_word(e, chunk_tag(b"XRMR"), count);
            let mut node = value + 8;
            while node != 0 && list_item(e, node) != 0 {
                let room = list_item(e, node);
                let id = form_id(e, room);
                add_chunk_word(e, chunk_tag(b"XLRM"), id);
                node = list_next(e, node);
            }
        }
        // EXTRA_IGNORED_BY_SANDBOX
        0x80 => add_chunk_empty(e, chunk_tag(b"XIBS")),
        // EXTRA_CELL_ACOUSTIC_SPACE
        0x81 => add_form_chunk(e, chunk_tag(b"XCAS"), value),
        // EXTRA_LIT_WATER_REFS
        0x85 => {
            let mut node = extra.addr() + 0x0c;
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let reference = list_item(e, node);
                let id = form_id(e, reference);
                add_chunk_word(e, chunk_tag(b"XLTW"), id);
                node = list_next(e, node);
            }
        }
        // EXTRA_WEAPON_MOD_SLOTS: the active slots byte.
        0x8d => {
            let slots = e.mem.u8(extra.addr() + 0x0c);
            add_chunk_byte(e, chunk_tag(b"XHLT"), slots);
        }
        // EXTRA_AUDIOMARKER
        0x90 => {
            add_chunk_empty(e, chunk_tag(b"MMRK"));
            e.call(AUDIO_MARKER_DATA_SAVE, &args![value]);
        }
        // EXTRA_AUDIOBUOYMARKER
        0x91 => {
            add_chunk_empty(e, chunk_tag(b"AMRK"));
            e.call(AUDIO_BUOY_MARKER_DATA_SAVE, &args![value]);
        }
        // EXTRA_SPECIAL_RENDER_FLAGS: the form id getter applied to the extra
        // data itself (so the word at +0x0C), then the float at +0x10.
        0x92 => {
            let id = form_id(e, extra.addr());
            add_chunk_word(e, chunk_tag(b"XSRF"), id);
            let second = payload(e, extra, 0x10);
            add_chunk_word(e, chunk_tag(b"XSRD"), x87_quiet_bits(second));
        }
        _ => {}
    }
}

/// `EXTRA_ACTIVATE_REF`: the flags byte (`XAPD`), each REF_ACTIVATE_DATA of
/// the parent list (`XAPR`), then the activate text override (`XATO`).
pub(crate) fn save_activate_ref(e: &mut Engine, list: Ptr<ExtraDataList>, extra: Ptr<BSExtraData>) {
    let flags = e.mem.u8(extra.addr() + 0x14);
    add_chunk_byte(e, chunk_tag(b"XAPD"), flags);
    let mut node = extra.addr() + 0x0c;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let data = list_item(e, node);
        node = list_next(e, node);
        e.with_stack(8, |e, record| {
            fn_00414010(e, record);
            e.call(MEMCPY, &args![record, data, 8u32]);
            let reference = e.mem.u32(record.addr());
            let id = form_id(e, reference);
            e.mem.set_u32(record.addr(), id);
            add_swapped_chunk_data(e, chunk_tag(b"XAPR"), record, 8, swap_two_words);
        });
    }
    let text = extra.addr() + 0x18;
    if e.call(STRING_LENGTH, &args![text]).u32() != 0 {
        // ActivateTextOverride, copied twice by `GetActivateTextOverride`
        // (`0041ec80`): the first copy gives the length, the second the
        // characters; the length + 1 stays on the stack as the array size.
        e.with_stack(8, |e, first| {
            e.with_stack(8, |e, second| {
                let first = e.call(GET_ACTIVATE_TEXT, &args![list, first]).u32();
                let length = e.call(STRING_LENGTH, &args![first]).u32();
                let size = length.wrapping_add(1);
                let second = e.call(GET_ACTIVATE_TEXT, &args![list, second]).u32();
                let characters = e.call(READ_WORD, &args![second]).u32();
                e.call(
                    ADD_CHUNK_ARRAY_RAW,
                    &args![chunk_tag(b"XATO"), characters, size],
                );
                e.call(STRING_DESTROY, &args![second]);
                e.call(STRING_DESTROY, &args![first]);
            });
        });
    }
}

/// `EXTRA_OCCLUSION_PLANE` / `EXTRA_PORTAL`: the record of nine words: the
/// plane's half extents (two words), centre (three), then the rotation as an
/// axis (three floats) and an angle from `NiMatrix3` (`00a58550`).
pub(crate) fn save_plane(e: &mut Engine, extra: Ptr<BSExtraData>, tag: u32) {
    e.with_stack(0x24, |e, record| {
        fn_00414470(e, record);
        let plane = e.call(READ_WORD, &args![extra.addr() + 0x0c]).u32();
        let half_extents = e.call(PLANE_HALF_EXTENTS, &args![plane]).u32();
        copy_block(e, half_extents, record.addr(), 8);
        let center = fn_00413f40(e, plane);
        copy_block(e, center, record.addr() + 8, 12);
        e.with_stack(0x24, |e, matrix| {
            let matrix = fn_00413f60(e, Ptr::new(plane), matrix);
            e.call(
                MATRIX_TO_AXIS_ANGLE,
                &args![
                    matrix,
                    record.addr() + 0x20,
                    record.addr() + 0x14,
                    record.addr() + 0x18,
                    record.addr() + 0x1c
                ],
            );
        });
        add_swapped_chunk_data(e, tag, record, 0x24, fn_004140f0);
    });
}

// ---------------------------------------------------------------------------
// The loader

/// Chunks whose data is one word handed to a setter of the list:
/// `list.Setter(word)`. (chunk, setter)
pub(crate) const LOAD_WORD_SETTERS: &[(&[u8; 4], u32)] = &[
    (b"XACT", 0x0041_b3d0),
    (b"XOWN", 0x0041_9700),
    (b"XGLB", 0x0041_97d0),
    (b"XRNK", 0x0041_98a0),
    (b"XPSN", 0x0041_9d10),
    (b"XCMO", 0x0041_bd10),
    (b"XCAS", 0x0041_c090),
    (b"XCCM", 0x0041_c190),
    (b"XCIM", 0x0041_c290),
    (b"XCWT", 0x0041_e160),
    (b"XRTM", 0x0041_e250),
    (b"XLKR", 0x0041_e440),
    (b"XMRC", 0x0042_1430),
    (b"XTRG", 0x0042_1b70),
    (b"XEZN", 0x0042_1c60),
    (b"XEMI", 0x0042_1d50),
    (b"XMBR", 0x0042_1e40),
    (b"XLCM", 0x0042_1540),
];

/// Chunks whose data is one `float` handed to a setter of the list (the value
/// goes through the x87 stack). (chunk, setter)
pub(crate) const LOAD_FLOAT_SETTERS: &[(&[u8; 4], u32)] = &[
    (b"XHLP", 0x0041_b580),
    (b"XTIM", 0x0041_9bb0),
    (b"XCHG", 0x0041_9c60),
    (b"XRDS", 0x0042_2220),
    (b"XRAD", 0x0042_2350),
];

/// `TESFile::GetChunkData(file, &value)` (`004727f0`): reads the 4-byte
/// value of the current chunk into a local that starts at zero.
pub(crate) fn read_chunk_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, local| {
        e.call(GET_CHUNK_WORD, &args![file, local]);
        e.mem.u32(local.addr())
    })
}

/// `TESFile::GetChunkData(file, buffer, size)` (`00472890`).
pub(crate) fn read_chunk_data(e: &mut Engine, file: Ptr, buffer: u32, size: u32) {
    e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
}

/// Whether the file's chunk data must be byte-swapped (the byte at `+0x299`
/// of the `TESFile`, read through `00401680`).
pub(crate) fn file_needs_swap(e: &mut Engine, file: Ptr) -> bool {
    e.call(FILE_NEEDS_SWAP, &args![file]).bool()
}

/// `Swap32(address, 0)` (`00401080`).
pub(crate) fn swap_dword_at(e: &mut Engine, address: u32) {
    e.call(SWAP_DWORD, &args![address, 0u32]);
}

/// The activate-parent extra data (type `0x53`), created and added when the
/// list has none (the `0x20`-byte constructor `004338b0`).
pub(crate) fn get_or_add_activate_ref(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
) -> Ptr<BSExtraData> {
    get_or_add_extra(e, list, 0x53, 0x20, |e, block| {
        e.call(ACTIVATE_REF_INIT, &args![block]).u32()
    })
}

/// A sub-object kept behind a getter/setter pair of the list (the teleport
/// data, the map marker, the audio markers): the getter's result, or a new
/// object of `size` bytes built by `construct` and given to the setter.
pub(crate) fn get_or_make_member(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    getter: u32,
    size: u32,
    construct: u32,
    setter: u32,
) -> u32 {
    let mut member = e.call(getter, &args![list]).u32();
    if member == 0 {
        member = new_object(e, size, |e, block| e.call(construct, &args![block]).u32());
        e.call(setter, &args![list, member]);
    }
    member
}

// Translated from 004144a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::Load` (Xbox PDB): reads the extra data chunk the file is
/// positioned at (`TESFile::GetTESChunk`, `004726b0`) and applies it to this
/// list: the counterpart of the save writer `fn_00412970`. `file` is the
/// `TESFile`, `buffer` the `BGSSaveFormBuffer` (its form, `+0x20`, is looked
/// at by the `XPWR` and `XLTW` chunks, which are skipped for a form of type
/// `0x23`). Chunks are matched by their four letters (`b"XCWT"` is
/// `0x54574358`); the table at the top of this section covers those that are
/// one word or one float for a setter, the rest are the arms of the `match`.
/// A chunk this function does not know is ignored. The compiler's
/// exception-unwinding frame is not translated.
pub fn extra_data_list_load(e: &mut Engine, this: Ptr<ExtraDataList>, file: Ptr, buffer: Ptr) {
    let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
    let name = chunk.to_le_bytes();
    if let Some(&(_, setter)) = LOAD_WORD_SETTERS.iter().find(|entry| *entry.0 == name) {
        let word = read_chunk_word(e, file);
        e.call(setter, &args![this, word]);
        return;
    }
    if let Some(&(_, setter)) = LOAD_FLOAT_SETTERS.iter().find(|entry| *entry.0 == name) {
        let value = x87_quiet_bits(read_chunk_word(e, file));
        e.call(setter, &args![this, f32::from_bits(value)]);
        return;
    }
    match &name {
        // EXTRA_HEALTH: the file holds an integer, the setter takes a float.
        b"XHLT" => {
            let health = read_chunk_word(e, file) as i32;
            e.call(SET_HEALTH, &args![this, health as f32]);
        }
        // EXTRA_USES
        b"XUSE" => {
            let uses = read_chunk_word(e, file) as u8;
            e.call(SET_USES, &args![this, uses]);
        }
        // EXTRA_COUNT
        b"XCNT" => {
            let count = read_chunk_word(e, file) as u16;
            e.call(SET_COUNT, &args![this, count]);
        }
        // EXTRA_AMMO, count: sets the count of the ammo the list already has.
        b"XAMC" => {
            let count = read_chunk_word(e, file);
            if e.call(GET_AMMO, &args![this]).u32() != 0 {
                let ammo = e.call(GET_AMMO, &args![this]).u32();
                let ammo_form = e.mem.u32(ammo + 0x0c);
                e.call(SET_AMMO, &args![this, ammo_form, count]);
            }
        }
        // EXTRA_AMMO, the ammo form id with a count of 999.
        b"XAMT" => {
            let ammo_id = read_chunk_word(e, file);
            e.call(SET_AMMO, &args![this, ammo_id, 0x3e7u32]);
        }
        // EXTRA_LOCK: a REFR_LOCK (0x14 bytes) read into the ExtraLock's.
        b"XLOC" => {
            let mut extra = base_extra_list_get_extra_data(e, this.cast(), 0x2a);
            let lock_data;
            if extra.is_null() {
                let made = new_object(e, 0x14, |e, block| fn_00411b00(e, Ptr::new(block)).addr());
                lock_data = made;
                let created = new_object(e, 0x10, |e, block| {
                    e.call(EXTRA_LOCK_INIT, &args![block, made]).u32()
                });
                extra = Ptr::new(created);
                base_extra_list_add_extra(e, this.cast(), extra);
            } else {
                lock_data = e.mem.u32(extra.addr() + 0x0c);
            }
            read_chunk_data(e, file, lock_data, 0x14);
            if file_needs_swap(e, file) {
                swap_word_at_4(e, Ptr::new(lock_data));
            }
            e.call(REFR_LOCK_SET_LOCKED, &args![lock_data, 1u32]);
        }
        // EXTRA_RADIO_DATA: a RADIO_DATA (0x10 bytes) given to the setter.
        b"XRDO" => e.with_stack(0x10, |e, radio| {
            fn_00416b40(e, radio);
            read_chunk_data(e, file, radio.addr(), 0x10);
            if file_needs_swap(e, file) {
                fn_00414220(e, radio);
            }
            e.call(SET_RADIO_DATA, &args![this, radio]);
        }),
        // EXTRA_TELEPORT
        b"XTEL" => {
            let teleport = get_or_make_member(
                e,
                this,
                GET_TELEPORT,
                0x20,
                DOOR_TELEPORT_DATA_INIT,
                SET_TELEPORT,
            );
            e.call(DOOR_TELEPORT_DATA_LOAD, &args![teleport, file]);
        }
        // EXTRA_MAPMARKER
        b"XMRK" => {
            let marker = get_or_make_member(
                e,
                this,
                GET_MAP_MARKER,
                0x14,
                MAP_MARKER_DATA_INIT,
                SET_MAP_MARKER,
            );
            e.call(MAP_MARKER_DATA_LOAD, &args![marker, file]);
        }
        // EXTRA_AUDIOMARKER
        b"MMRK" => {
            let marker = get_or_make_member(
                e,
                this,
                GET_AUDIO_MARKER,
                0x34,
                AUDIO_MARKER_DATA_INIT,
                SET_AUDIO_MARKER,
            );
            e.call(AUDIO_MARKER_DATA_LOAD, &args![marker, file]);
        }
        // EXTRA_AUDIOBUOYMARKER
        b"AMRK" => {
            let marker = get_or_make_member(
                e,
                this,
                GET_AUDIO_BUOY_MARKER,
                0x08,
                AUDIO_BUOY_MARKER_DATA_INIT,
                SET_AUDIO_BUOY_MARKER,
            );
            e.call(AUDIO_BUOY_MARKER_DATA_LOAD, &args![marker, file]);
        }
        // EXTRA_SEED: one byte, from a 4-byte chunk or a 1-byte one.
        b"XSED" => {
            let mut seed = 0u8;
            if e.call(GET_CHUNK_SIZE, &args![file]).u32() == 4 {
                seed = read_chunk_word(e, file) as u8;
            } else {
                e.with_stack(1, |e, local| {
                    read_chunk_data(e, file, local.addr(), 1);
                    seed = e.mem.u8(local.addr());
                });
            }
            e.call(SET_SEED, &args![this, seed]);
        }
        // EXTRA_REGIONLIST: the region form ids, looked up in the data
        // handler's region list and gathered in a new TESRegionList.
        b"XCLR" => {
            let size = e.call(GET_CHUNK_SIZE, &args![file]).u32();
            let count = size >> 2;
            if size % 4 != 0 {
                let name = e.call(FILE_NAME, &args![file]).u32();
                e.call(LOG_MESSAGE, &args![MESSAGE_BAD_REGION_LIST, name]);
                return;
            }
            let region_list = new_object(e, 0x10, |e, block| {
                e.call(REGION_LIST_INIT, &args![block, 0u32]).u32()
            });
            let ids = e.call(OPERATOR_NEW, &args![count.saturating_mul(4)]).u32();
            read_chunk_data(e, file, ids, size);
            let mut index = 0;
            while index < count {
                let slot = ids.wrapping_add(index * 4);
                if file_needs_swap(e, file) {
                    swap_dword_at(e, slot);
                }
                e.call(ADD_COMPILE_INDEX, &args![slot, file]);
                let id = e.mem.u32(slot);
                let data_handler = e.global::<u32>(DATA_HANDLER);
                let known_regions = e.call(DATA_HANDLER_REGIONS, &args![data_handler]).u32();
                let region = e.call(REGION_LIST_FIND, &args![known_regions, id]).u32();
                if region != 0 {
                    e.call(REGION_LIST_ADD, &args![region_list, region]);
                }
                index += 1;
            }
            e.call(SET_REGION_LIST, &args![this, region_list]);
            e.call(OPERATOR_DELETE, &args![ids]);
        }
        // EXTRA_PACKAGESTARTLOC: a WORLD_LOCATION (0x14 bytes).
        b"XPSL" => e.with_stack(0x14, |e, record| {
            e.call(PATH_LOCATION_INIT, &args![record]);
            e.call(MEMSET, &args![record, 0u32, 0x14u32]);
            read_chunk_data(e, file, record.addr(), 0x14);
            if file_needs_swap(e, file) {
                fn_00414220(e, record);
            }
            let location = e.mem.u32(record.addr());
            let z_rotation = x87_quiet_bits(e.mem.u32(record.addr() + 0x10));
            e.call(
                SET_PACKAGE_START_LOCATION,
                &args![
                    this,
                    location,
                    0u32,
                    record.addr() + 4,
                    f32::from_bits(z_rotation)
                ],
            );
        }),
        // EXTRA_RAGDOLLDATA (the two chunk names share the arm).
        b"XRGD" | b"XRGB" => {
            let mut extra = base_extra_list_get_extra_data(e, this.cast(), 0x14);
            if extra.is_null() {
                let created = new_object(e, 0x10, |e, block| {
                    e.call(EXTRA_RAGDOLL_DATA_INIT, &args![block]).u32()
                });
                extra = Ptr::new(created);
                let data = new_object(e, 0x14, |e, block| {
                    e.call(RAGDOLL_DATA_INIT, &args![block]).u32()
                });
                e.mem.set_u32(extra.addr() + 0x0c, data);
                base_extra_list_add_extra(e, this.cast(), extra);
            }
            let ragdoll = e.mem.u32(extra.addr() + 0x0c);
            if !e.call(RAGDOLL_DATA_LOAD, &args![ragdoll, file]).bool() {
                e.call(LOG_MESSAGE, &args![MESSAGE_BAD_RAGDOLL]);
                base_extra_list_remove_extra(e, this.cast(), extra, true);
            }
        }
        // EXTRA_DISTANTDATA: three floats (the first two zero, the third the
        // constant at `010145a8` unless the chunk gives it).
        b"XLOD" => e.with_stack(12, |e, local| {
            let third = x87_quiet_bits(e.global::<u32>(LOD_DEFAULT));
            fn_00416870(e, local, 0.0, 0.0, f32::from_bits(third));
            read_chunk_data(e, file, local.addr(), 12);
            if file_needs_swap(e, file) {
                for index in 0..3 {
                    swap_dword_at(e, local.addr() + index * 4);
                }
            }
            e.call(SET_DISTANT_DATA, &args![this, local]);
        }),
        // EXTRA_ENABLESTATEPARENT: the parent's form id and its flags byte.
        b"XESP" => e.with_stack(8, |e, local| {
            e.call(MEMSET, &args![local, 0u32, 8u32]);
            read_chunk_data(e, file, local.addr(), 8);
            if file_needs_swap(e, file) {
                swap_word_at(e, local);
            }
            let parent = e.mem.u32(local.addr());
            e.call(SET_ENABLE_STATE_PARENT, &args![this, parent]);
            let flags = e.mem.u8(local.addr() + 4);
            e.call(SET_ENABLE_STATE_FLAGS, &args![this, flags]);
        }),
        // EXTRA_ACTIVATE_REF, the flags byte.
        b"XAPD" => {
            let extra = get_or_add_activate_ref(e, this);
            if e.call(GET_CHUNK_SIZE, &args![file]).u32() == 4 {
                let flags = read_chunk_word(e, file) as u8;
                e.mem.set_u8(extra.addr() + 0x14, flags);
            } else {
                read_chunk_data(e, file, extra.addr() + 0x14, 1);
            }
        }
        // EXTRA_ACTIVATE_REF, the text override.
        b"XATO" => {
            get_or_add_activate_ref(e, this);
            let size = e.call(GET_CHUNK_SIZE, &args![file]).u32() as i32;
            if size > 1 {
                let text = e.call(OPERATOR_NEW, &args![size as u32]).u32();
                read_chunk_data(e, file, text, size as u32);
                e.call(SET_ACTIVATE_TEXT_OVERRIDE, &args![this, text]);
                e.call(OPERATOR_DELETE, &args![text]);
            }
        }
        // EXTRA_ACTIVATE_REF, one REF_ACTIVATE_DATA of the parent list.
        b"XAPR" => {
            let extra = get_or_add_activate_ref(e, this);
            let data = new_object(e, 8, |e, block| fn_00414010(e, Ptr::new(block)).addr());
            read_chunk_data(e, file, data, 8);
            if file_needs_swap(e, file) {
                swap_two_words(e, Ptr::new(data));
            }
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), data);
                e.call(LIST_ADD_HEAD, &args![extra.addr() + 0x0c, local]);
            });
        }
        // EXTRA_ACTIVATE_REF, the old form of a parent list entry.
        b"XACR" => {
            let extra = get_or_add_activate_ref(e, this);
            e.with_stack(0x0c, |e, old| {
                e.call(MEMSET, &args![old, 0u32, 0x0cu32]);
                read_chunk_data(e, file, old.addr(), 0x0c);
                if file_needs_swap(e, file) {
                    swap_dword_at(e, old.addr());
                    swap_dword_at(e, old.addr() + 4);
                }
                let node = new_object(e, 8, |e, block| fn_00414010(e, Ptr::new(block)).addr());
                let reference = e.mem.u32(old.addr());
                e.mem.set_u32(node, reference);
                let delay = x87_quiet_bits(e.mem.u32(old.addr() + 4));
                e.mem.set_u32(node + 4, delay);
                e.with_stack(4, |e, local| {
                    e.mem.set_u32(local.addr(), node);
                    e.call(LIST_ADD_HEAD, &args![extra.addr() + 0x0c, local]);
                });
                let flags = e.mem.u8(old.addr() + 8);
                e.mem.set_u8(extra.addr() + 0x14, flags);
            });
        }
        // EXTRA_DECAL_REFS: one REF_DECAL_DATA (0x1c bytes).
        b"XDCR" => e.with_stack(0x1c, |e, decal| {
            e.call(DECAL_DATA_INIT, &args![decal]);
            e.call(MEMSET, &args![decal, 0u32, 0x1cu32]);
            read_chunk_data(e, file, decal.addr(), 0x1c);
            if file_needs_swap(e, file) {
                fn_00414030(e, decal);
            }
            let reference = e.mem.u32(decal.addr());
            e.call(
                ADD_DECAL_REF,
                &args![this, reference, decal.addr() + 4, decal.addr() + 0x10],
            );
        }),
        // EXTRA_MULTIBOUND_DATA
        b"XMBO" => {
            let bound = new_object(e, 0x0c, |e, block| {
                e.call(MULTIBOUND_MARKER_DATA_INIT, &args![block]).u32()
            });
            e.call(MULTIBOUND_MARKER_DATA_LOAD, &args![bound, file]);
            e.call(SET_MULTIBOUND_DATA, &args![this, bound]);
        }
        // Skipped chunk.
        b"XPCI" => {
            e.call(SKIP_CHUNK, &args![file]);
        }
        // EXTRA_NAVMESH_PORTAL: the chunk is read whole and handed to the
        // portal's own loader.
        b"XNVP" => e.with_stack(8, |e, portal| {
            e.call(NAVMESH_PORTAL_INIT, &args![portal]);
            let size = e.call(GET_CHUNK_SIZE, &args![file]).u32();
            let data = e.call(OPERATOR_NEW, &args![size]).u32();
            read_chunk_data(e, file, data, size);
            e.call(NAVMESH_PORTAL_LOAD, &args![portal, buffer, file, data]);
            e.call(OPERATOR_DELETE, &args![data]);
        }),
        // EXTRA_NAVMESH_PORTAL, the old form: an 8-byte value turned into a
        // type `0x5A` extra data on the stack and given to `0042e2c0`.
        b"XNDP" => e.with_stack(8, |e, value| {
            fn_00416ab0(e, value);
            read_chunk_data(e, file, value.addr(), 8);
            if file_needs_swap(e, file) {
                fn_004141e0(e, value);
            }
            let first = e.mem.u32(value.addr());
            let second = e.mem.u16(value.addr() + 4) as u32;
            e.with_stack(0x14, |e, extra| {
                fn_00416ad0(e, extra, first, second);
                e.call(SET_NAVMESH_PORTAL, &args![this, extra]);
                e.call(NAVMESH_PORTAL_EXTRA_DESTROY, &args![extra]);
            });
        }),
        // EXTRA_PORTAL_REF_DATA (`XPWR`) and EXTRA_LIT_WATER_REFS (`XLTW`)
        // saved in a form of type 0x23 are not loaded.
        b"XPWR" => {
            if !form_is_type_0x23(e, buffer) {
                e.with_stack(8, |e, record| {
                    e.call(SIMPLE_LIST_INIT, &args![record]);
                    e.call(MEMSET, &args![record, 0u32, 8u32]);
                    read_chunk_data(e, file, record.addr(), 8);
                    if file_needs_swap(e, file) {
                        swap_dword_at(e, record.addr());
                        swap_dword_at(e, record.addr() + 4);
                    }
                    let first = e.mem.u32(record.addr());
                    let flags = e.mem.u32(record.addr() + 4);
                    if flags & 1 != 0 {
                        e.call(ADD_REFLECTOR_REF, &args![this, first, 1u32]);
                    }
                    if flags & 2 != 0 {
                        e.call(ADD_REFLECTED_REF, &args![this, first, 1u32]);
                    }
                });
            }
        }
        b"XLTW" => {
            if !form_is_type_0x23(e, buffer) {
                let reference = read_chunk_word(e, file);
                e.call(ADD_LIT_WATER_REF, &args![this, reference, 1u32]);
            }
        }
        // EXTRA_PRIMITIVE: radii, colour and type.
        b"XPRM" => e.with_stack(0x20, |e, record| {
            fn_004143f0(e, record);
            e.call(MEMSET, &args![record, 0u32, 0x20u32]);
            read_chunk_data(e, file, record.addr(), 0x20);
            if file_needs_swap(e, file) {
                fn_00414290(e, record);
            }
            let primitive_type = e.mem.u32(record.addr() + 0x1c);
            let primitive = e
                .call(
                    PRIMITIVE_CREATE,
                    &args![primitive_type, record, record.addr() + 0x0c],
                )
                .u32();
            if primitive != 0 {
                e.call(ADD_PRIMITIVE, &args![this, primitive]);
            }
        }),
        // EXTRA_OCCLUSION_PLANE: half extents, centre and axis-angle rotation
        // build a BSOcclusionPlane (`0xFC` bytes) given to the setter.
        b"XOCP" => e.with_stack(0x24, |e, record| {
            fn_00414470(e, record);
            e.call(MEMSET, &args![record, 0u32, 0x24u32]);
            read_chunk_data(e, file, record.addr(), 0x24);
            if file_needs_swap(e, file) {
                fn_004140f0(e, record);
            }
            let block = e.call(ALLOCATE_ALIGNED, &args![0xfcu32]).u32();
            let plane = if block == 0 {
                0
            } else {
                e.call(OCCLUSION_PLANE_INIT, &args![block]).u32()
            };
            fn_00416a30(e, Ptr::new(plane), record);
            fn_004169f0(e, Ptr::new(plane), record.byte_add(8));
            e.with_stack(0x24, |e, matrix| {
                e.call(SIMPLE_LIST_ITEM, &args![matrix]);
                let angle = x87_quiet_bits(e.mem.u32(record.addr() + 0x20));
                let axis_x = x87_quiet_bits(e.mem.u32(record.addr() + 0x14));
                let axis_y = x87_quiet_bits(e.mem.u32(record.addr() + 0x18));
                let axis_z = x87_quiet_bits(e.mem.u32(record.addr() + 0x1c));
                e.call(
                    MATRIX_MAKE_ROTATION,
                    &args![
                        matrix,
                        f32::from_bits(angle),
                        f32::from_bits(axis_x),
                        f32::from_bits(axis_y),
                        f32::from_bits(axis_z)
                    ],
                );
                fn_00416a70(e, Ptr::new(plane), matrix);
            });
            e.call(SET_OCCLUSION_PLANE, &args![this, plane]);
        }),
        // EXTRA_PORTAL: the record is read and dropped.
        b"XPTL" => e.with_stack(0x24, |e, record| {
            fn_00414470(e, record);
            e.call(MEMSET, &args![record, 0u32, 0x24u32]);
            read_chunk_data(e, file, record.addr(), 0x24);
            if file_needs_swap(e, file) {
                fn_004140f0(e, record);
            }
        }),
        // EXTRA_PATROL_REF_DATA, the time at the reference.
        b"XPRD" => e.with_stack(4, |e, local| {
            e.call(SIMPLE_LIST_ITEM, &args![local]);
            read_chunk_data(e, file, local.addr(), 4);
            if file_needs_swap(e, file) {
                swap_word_at(e, local);
            }
            let patrol = new_object(e, 0x14, |e, block| {
                e.call(PATROL_REF_DATA_INIT, &args![block, local]).u32()
            });
            e.call(SET_PATROL_REF_DATA, &args![this, patrol]);
        }),
        // EXTRA_OCCLUSION_PLANE_REF_DATA: four form ids.
        b"XORD" => {
            let data = new_object(e, 0x10, |e, block| fn_00411dc0(e, Ptr::new(block)).addr());
            read_chunk_data(e, file, data, 0x10);
            if file_needs_swap(e, file) {
                fn_00414370(e, Ptr::new(data));
            }
            e.call(SET_OCCLUSION_PLANE_REF_DATA, &args![this, data]);
        }
        // EXTRA_PORTAL_REF_DATA: two form ids.
        b"XPOD" => {
            let data = new_object(e, 8, |e, block| fn_004143c0(e, Ptr::new(block)).addr());
            read_chunk_data(e, file, data, 8);
            if file_needs_swap(e, file) {
                swap_two_words(e, Ptr::new(data));
            }
            e.call(SET_PORTAL_REF_DATA, &args![this, data]);
        }
        // EXTRA_ROOM_REF_DATA: the count of linked portals with the master
        // flag in bit 16, then that many form ids (each in its own chunk).
        b"XRMR" => {
            let data = new_object(e, 0x14, |e, block| fn_00416b80(e, Ptr::new(block)).addr());
            let word = read_chunk_word(e, file);
            e.mem.set_u8(data + 0x10, (word >> 16) as u8);
            let count = word & 0xffff;
            let mut index = 0;
            while index < count {
                e.call(SKIP_CHUNK, &args![file]);
                let room = read_chunk_word(e, file);
                e.with_stack(4, |e, local| {
                    e.mem.set_u32(local.addr(), room);
                    e.call(LIST_ADD_HEAD, &args![data + 8, local]);
                });
                index += 1;
            }
            e.call(SET_ROOM_REF_DATA, &args![this, data]);
        }
        // EXTRA_COLLISION_DATA: one word.
        b"XTRI" => {
            let data = new_object(e, 4, |e, block| fn_00411e00(e, Ptr::new(block)).addr());
            read_chunk_data(e, file, data, 4);
            if file_needs_swap(e, file) {
                swap_word_at(e, Ptr::new(data));
            }
            e.call(SET_COLLISION_DATA, &args![this, data]);
        }
        // EXTRA_PATROL_REF_DATA's PackageEventAction.
        b"XPPA" => {
            let patrol = e.call(GET_PATROL_REF_DATA, &args![this]).u32();
            e.call(SKIP_CHUNK, &args![file]);
            e.call(
                PACKAGE_EVENT_ACTION_LOAD,
                &args![patrol.wrapping_add(4), file],
            );
        }
        // EXTRA_IGNORED_BY_SANDBOX
        b"XIBS" => {
            e.call(SET_IGNORED_BY_SANDBOX, &args![this, 1u32]);
        }
        // Type 0x92, the word at +0x0C.
        b"XSRF" => {
            let extra = get_or_add_special_render_flags(e, this);
            let value = read_chunk_word(e, file);
            e.call(SET_SPECIAL_RENDER_WORD, &args![extra, value]);
        }
        // Type 0x92, the float at +0x10.
        b"XSRD" => {
            let extra = get_or_add_special_render_flags(e, this);
            let value = x87_quiet_bits(read_chunk_word(e, file));
            e.mem.set_u32(extra.addr() + 0x10, value);
        }
        // Two chunks that are read into a byte and dropped.
        b"XCMT" | b"XCET" => {
            e.with_stack(1, |e, local| {
                read_chunk_data(e, file, local.addr(), 1);
            });
        }
        // Any other chunk is left alone.
        _ => {}
    }
}

/// The type `0x92` extra data, created (`fn_00411e40`, `0x14` bytes) and added
/// when the list has none.
pub(crate) fn get_or_add_special_render_flags(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
) -> Ptr<BSExtraData> {
    let existing = base_extra_list_get_extra_data(e, list.cast(), 0x92);
    if !existing.is_null() {
        return existing;
    }
    let created = new_object(e, 0x14, |e, block| fn_00411e40(e, Ptr::new(block)).addr());
    base_extra_list_add_extra(e, list.cast(), Ptr::new(created))
}

/// Whether the form of the save buffer (`BGSSaveFormBuffer::GetForm`,
/// `007af430`) exists and has form type `0x23` (`TESForm::GetFormType`,
/// `00401170`).
pub(crate) fn form_is_type_0x23(e: &mut Engine, buffer: Ptr) -> bool {
    let form = e.call(SAVE_BUFFER_GET_FORM, &args![buffer]).u32();
    if form == 0 {
        return false;
    }
    let form = e.call(SAVE_BUFFER_GET_FORM, &args![buffer]).u32();
    e.call(FORM_TYPE, &args![form]).u32() == 0x23
}

// ---------------------------------------------------------------------------
// InitItem

/// Type descriptor of `TESForm` for `__RTDynamicCast` (the source type of
/// every cast the master file fix-ups make).
pub(crate) const RTTI_TES_FORM: u32 = 0x0118_3028;
/// Type descriptor of `TESObjectREFR`.
pub(crate) const RTTI_TES_OBJECT_REFR: u32 = 0x0118_41cc;

/// The arms of `InitItem` that replace the form id at `+0x0C` of the extra
/// data by the form it names and remove the extra data when there is none:
/// (type, `__RTDynamicCast` target or 0 for no cast, message with the id).
pub(crate) const INIT_ITEM_FORM_FIXUPS: &[(u8, u32, u32)] = &[
    // EXTRA_WATERTYPE
    (0x03, 0x0118_4118, 0x0101_4db0),
    // EXTRA_CELLMUSICTYPE
    (0x07, 0x0118_40fc, 0x0101_4d60),
    // EXTRA_CLIMATE
    (0x08, 0x0118_4170, 0x0101_4eb8),
    // EXTRA_PACKAGESTARTLOC: the cell, not cast.
    (0x18, 0, 0x0101_4cf0),
    // EXTRA_OWNERSHIP: the owner, not cast.
    (0x21, 0, 0x0101_4fa8),
    // EXTRA_GLOBAL
    (0x22, 0x0118_3ad4, 0x0101_4f50),
    // EXTRA_TELEPORTMARKER
    (0x3b, RTTI_TES_OBJECT_REFR, 0x0101_4be0),
    // EXTRA_MERCHANTCONTAINER
    (0x3c, RTTI_TES_OBJECT_REFR, 0x0101_4830),
    // EXTRA_POISON
    (0x3f, 0x0118_30cc, 0x0101_4f08),
    // EXTRA_XTARGET
    (0x44, RTTI_TES_OBJECT_REFR, 0x0101_47d8),
    // EXTRA_IMAGESPACE
    (0x59, 0x0118_4154, 0x0101_4e60),
    // EXTRA_AMMO
    (0x6e, 0x0118_40c4, 0x0101_4620),
    // EXTRA_ENCOUNTERZONE
    (0x74, 0x0118_40dc, 0x0101_4788),
    // EXTRA_CELL_ACOUSTIC_SPACE
    (0x81, 0x0118_4134, 0x0101_4e00),
];

/// `TESForm::AddCompileIndex(&id, file)` (`00485d50`, cdecl): turns an id as
/// the file stores it into the id of the load order, in a local copy.
pub(crate) fn add_compile_index(e: &mut Engine, id: u32, file: u32) -> u32 {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), id);
        e.call(ADD_COMPILE_INDEX, &args![local, file]);
        e.mem.u32(local.addr())
    })
}

/// `__RTDynamicCast(form, 0, TESForm, target, 0)`.
pub(crate) fn dynamic_cast(e: &mut Engine, form: u32, target: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, RTTI_TES_FORM, target, 0u32],
    )
    .u32()
}

/// The id as the file stores it becomes the form: `AddCompileIndex`, the
/// lookup by id (`004839c0`) and, when `target` is not 0, the dynamic cast.
/// Returns the id of the load order and the form.
pub(crate) fn resolve_form_id(
    e: &mut Engine,
    file: u32,
    stored_id: u32,
    target: u32,
) -> (u32, u32) {
    let id = add_compile_index(e, stored_id, file);
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    let form = if target == 0 {
        form
    } else {
        dynamic_cast(e, form, target)
    };
    (id, form)
}

/// `RemoveExtra(extra, destroy)` on the list being fixed up.
pub(crate) fn remove_and_delete(e: &mut Engine, list: Ptr<ExtraDataList>, extra: Ptr<BSExtraData>) {
    base_extra_list_remove_extra(e, list.cast(), extra, true);
}

/// The `TESObjectREFR` embedded `ExtraDataList` (`005d43c0` gives
/// `reference + 0x44`).
pub(crate) fn reference_extra_list(e: &mut Engine, reference: u32) -> u32 {
    e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32()
}

// Translated from 00416be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::InitItem` (Xbox PDB): the fix-up of a list loaded from the
/// master file, for the reference `reference`: walks the chain under the lock
/// and, for each extra data of a type that holds form ids, replaces the id (as
/// the file stores it, `TESForm::AddCompileIndex`) by the form it names; when
/// the form is missing, the extra data is logged ("MASTERFILE: Unable to find
/// ...") and removed. Some types also link the target reference back to this
/// one (`LinkedRef`, `EnableStateParent`, the activate and portal lists).
/// The types are dispatched by the jump table at `0041809c`; the arms that
/// only resolve one id are the table [`INIT_ITEM_FORM_FIXUPS`].
///
/// The next extra data is read before each one is handled, so an extra data
/// removed by its arm does not break the walk. The type `0x77` arm keeps
/// reading the extra data it has just removed (as the game's code does).
/// The compiler's exception-unwinding frame is not translated.
pub fn extra_data_list_init_item(e: &mut Engine, this: Ptr<ExtraDataList>, reference: Ptr) {
    lock(e, 0);
    let mut current: Ptr<BSExtraData> = e.get(this, ExtraDataList::pHead).cast();
    let file = e
        .call(TES_FORM_GET_FILE, &args![reference, 0xffff_ffffu32])
        .u32();
    while !current.is_null() {
        let next = get_next(e, current);
        let extra_type = get_type(e, current);
        init_item_extra(e, this, reference, file, current, extra_type);
        current = next;
    }
    unlock(e);
}

/// One arm of `InitItem`'s switch.
pub(crate) fn init_item_extra(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    reference: Ptr,
    file: u32,
    extra: Ptr<BSExtraData>,
    extra_type: u8,
) {
    if let Some(&(_, target, message)) = INIT_ITEM_FORM_FIXUPS
        .iter()
        .find(|entry| entry.0 == extra_type)
    {
        let stored = payload(e, extra, 0x0c);
        let (id, form) = resolve_form_id(e, file, stored, target);
        e.mem.set_u32(extra.addr() + 0x0c, form);
        if form == 0 {
            e.call(LOG_MESSAGE, &args![message, id]);
            remove_and_delete(e, this, extra);
        }
        return;
    }
    match extra_type {
        // EXTRA_RAGDOLLDATA: a living actor must not have one (only logged).
        0x14 => {
            if e.vcall(reference.addr(), 0x100, &args![]).bool() {
                let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
                if base_form != 0 {
                    let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
                    let health = e.call(ACTOR_BASE_GET_HEALTH, &args![base_form]).i32();
                    if health > 0 {
                        let id = form_id(e, reference.addr());
                        e.call(LOG_MESSAGE, &args![MESSAGE_RAGDOLL_ON_LIVE_ACTOR, id]);
                    }
                }
            }
        }
        // EXTRA_LOCK: the key becomes the TESKey (`00ec43fb` cast to
        // 0x011841b4) or the lock is removed.
        0x2a => {
            let lock_data = payload(e, extra, 0x0c);
            let mut remove = false;
            let key = e.mem.u32(lock_data + 4);
            if key != 0 {
                let (id, resolved) = resolve_form_id(e, file, key, RTTI_TES_KEY);
                e.mem.set_u32(lock_data + 4, resolved);
                if resolved == 0 {
                    e.call(LOG_MESSAGE, &args![MESSAGE_LOCK_KEY_MISSING, id]);
                    remove = true;
                }
            }
            if remove {
                remove_and_delete(e, this, extra);
            }
        }
        // EXTRA_TELEPORT: DoorTeleportData::InitItem decides.
        0x2b => {
            let teleport = payload(e, extra, 0x0c);
            if !e
                .call(DOOR_TELEPORT_DATA_INIT_ITEM, &args![teleport, reference])
                .bool()
            {
                remove_and_delete(e, this, extra);
            }
        }
        // EXTRA_MAPMARKER: a travel location that is not visible is cleared;
        // the reputation id becomes the reputation form.
        0x2c => {
            let marker = payload(e, extra, 0x0c);
            if e.call(MAP_MARKER_GET_TRAVEL_LOC, &args![marker]).bool()
                && !e.call(MAP_MARKER_GET_VISIBLE, &args![marker]).bool()
            {
                e.call(MAP_MARKER_SET_TRAVEL_LOC, &args![marker, 0u32]);
            }
            let reputation_id = e.call(MAP_MARKER_GET_REPUTATION, &args![marker]).u32();
            if reputation_id != 0 {
                let data_handler = e.global::<u32>(DATA_HANDLER);
                let reputation = e
                    .call(
                        DATA_HANDLER_GET_REPUTATION,
                        &args![data_handler, reputation_id],
                    )
                    .u32();
                e.call(MAP_MARKER_SET_REPUTATION, &args![marker, reputation]);
            }
        }
        // EXTRA_ENABLESTATEPARENT: the parent reference, and a check that the
        // chain of parents does not loop back to `reference`.
        0x37 => {
            let stored = payload(e, extra, 0x0c);
            let (id, parent) = resolve_form_id(e, file, stored, RTTI_TES_OBJECT_REFR);
            e.mem.set_u32(extra.addr() + 0x0c, parent);
            if parent == 0 {
                e.call(LOG_MESSAGE, &args![MESSAGE_ENABLE_PARENT_MISSING, id]);
                remove_and_delete(e, this, extra);
            } else {
                let as_reference = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![reference, 0u32, RTTI_TES_FORM, RTTI_TES_OBJECT_REFR, 0u32],
                    )
                    .u32();
                if as_reference != 0
                    && e.call(CHECK_ENABLE_PARENT_LOOP, &args![as_reference])
                        .bool()
                {
                    let parent_list = reference_extra_list(e, parent);
                    e.call(ENABLE_PARENT_ADD_CHILD, &args![parent_list, as_reference]);
                } else {
                    e.call(LOG_MESSAGE, &args![MESSAGE_ENABLE_PARENT_LOOP]);
                    remove_and_delete(e, this, extra);
                }
            }
        }
        // EXTRA_LINKED_REF: the linked reference, which is told about this one.
        0x51 => {
            let stored = payload(e, extra, 0x0c);
            let (id, linked) = resolve_form_id(e, file, stored, RTTI_TES_OBJECT_REFR);
            e.mem.set_u32(extra.addr() + 0x0c, linked);
            if linked == 0 {
                e.call(LOG_MESSAGE, &args![MESSAGE_LINKED_REF_MISSING, id]);
                remove_and_delete(e, this, extra);
            } else {
                let linked_list = reference_extra_list(e, linked);
                e.call(LINKED_REF_ADD_CHILD, &args![linked_list, reference]);
            }
        }
        // EXTRA_ACTIVATE_REF
        0x53 => init_activate_ref(e, this, reference, file, extra),
        // EXTRA_DECAL_REFS
        0x57 => {
            e.call(DECAL_REFS_INIT_ITEM, &args![extra, reference]);
            if e.call(LIST_IS_EMPTY, &args![extra.addr() + 0x0c]).bool() {
                e.call(LOG_MESSAGE, &args![MESSAGE_EMPTY_DECALS]);
                remove_and_delete(e, this, extra);
            }
        }
        // EXTRA_MULTIBOUND_REF: the multibound reference, which must not be
        // the reference itself.
        0x63 => {
            let stored = payload(e, extra, 0x0c);
            let (id, bound) = resolve_form_id(e, file, stored, RTTI_TES_OBJECT_REFR);
            e.mem.set_u32(extra.addr() + 0x0c, bound);
            if bound == reference.addr() {
                e.mem.set_u32(extra.addr() + 0x0c, 0);
            }
            if e.mem.u32(extra.addr() + 0x0c) == 0 {
                let reference_id = form_id(e, reference.addr());
                e.call(
                    LOG_MESSAGE,
                    &args![MESSAGE_MULTIBOUND_REF_MISSING, id, reference_id],
                );
                remove_and_delete(e, this, extra);
            }
        }
        // EXTRA_REFLECTOR_REFS
        0x66 => {
            e.call(REFLECTOR_REFS_INIT_ITEM, &args![extra, reference]);
            if e.call(LIST_IS_EMPTY, &args![extra.addr() + 0x0c]).bool() {
                e.call(LOG_MESSAGE, &args![MESSAGE_EMPTY_REFLECTOR_REFS]);
                remove_and_delete(e, this, extra);
            }
        }
        // EXTRA_EMITTANCE_SOURCE: a light (form type 0x1E) or an activator
        // (form type 0x37) only.
        0x67 => {
            let stored = payload(e, extra, 0x0c);
            let (id, source) = resolve_form_id(e, file, stored, 0);
            e.mem.set_u32(extra.addr() + 0x0c, source);
            if source != 0 {
                let form_type = e.call(FORM_TYPE, &args![source]).u32();
                if form_type != 0x1e && form_type != 0x37 {
                    e.mem.set_u32(extra.addr() + 0x0c, 0);
                }
            }
            if e.mem.u32(extra.addr() + 0x0c) == 0 {
                e.call(LOG_MESSAGE, &args![MESSAGE_EMITTANCE_SOURCE_MISSING, id]);
                remove_and_delete(e, this, extra);
            }
        }
        // EXTRA_RADIO_DATA: the position reference of the RADIO_DATA.
        0x68 => {
            let data = extra.addr() + 0x0c;
            let stored = e.mem.u32(data + 0x0c);
            if stored != 0 {
                let radio_file = e
                    .call(TES_FORM_GET_FILE, &args![reference, 0xffff_ffffu32])
                    .u32();
                let (id, position) = resolve_form_id(e, radio_file, stored, RTTI_TES_OBJECT_REFR);
                e.mem.set_u32(data + 0x0c, position);
                if position == 0 {
                    e.call(LOG_MESSAGE, &args![MESSAGE_RADIO_POSITION_MISSING, id]);
                } else if e.call(REFERENCE_GET_INTERIOR, &args![position]).bool() {
                    e.call(LOG_MESSAGE, &args![MESSAGE_RADIO_POSITION_INTERIOR, id]);
                }
            }
        }
        // EXTRA_PATROL_REF_DATA
        0x6f => {
            let patrol = e.call(GET_PATROL_REF_DATA, &args![this]).u32();
            if patrol != 0 {
                e.call(PATROL_REF_DATA_INIT_ITEM, &args![patrol, reference]);
            }
        }
        // EXTRA_OCCLUSION_PLANE_REF_DATA: four linked references, each
        // linked to the plane of its own list. (The id read from the table is
        // overwritten with 0 before it is resolved, as the code does.)
        0x76 => {
            let table = payload(e, extra, 0x0c);
            for index in 0..4u32 {
                let _stored = e.mem.u32(table + index * 4);
                let (id, linked) = resolve_form_id(e, file, 0, RTTI_TES_OBJECT_REFR);
                e.mem.set_u32(table + index * 4, linked);
                if id != 0 && e.mem.u32(table + index * 4) == 0 {
                    e.call(LOG_MESSAGE, &args![MESSAGE_PLANE_REF_MISSING, id]);
                    remove_and_delete(e, this, extra);
                    break;
                }
                let plane = e.call(GET_OCCLUSION_PLANE, &args![this]).u32();
                if plane == 0 {
                    e.call(LOG_MESSAGE, &args![MESSAGE_NO_OCCLUSION_PLANE]);
                } else {
                    let linked = e.mem.u32(table + index * 4);
                    if linked != 0 {
                        let linked_list = reference_extra_list(e, linked);
                        let linked_plane = e.call(GET_OCCLUSION_PLANE, &args![linked_list]).u32();
                        e.call(SET_LINKED_PLANE, &args![plane, index, linked_plane]);
                    }
                }
            }
        }
        // EXTRA_PORTAL_REF_DATA: two linked references.
        0x77 => {
            let data = extra;
            for index in 0..2u32 {
                let table = payload(e, data, 0x0c);
                let stored = e.mem.u32(table + index * 4);
                let (id, linked) = resolve_form_id(e, file, stored, RTTI_TES_OBJECT_REFR);
                let table = payload(e, data, 0x0c);
                e.mem.set_u32(table + index * 4, linked);
                if id != 0 && e.mem.u32(table + index * 4) == 0 {
                    e.call(LOG_MESSAGE, &args![MESSAGE_PORTAL_REF_MISSING, id]);
                    remove_and_delete(e, this, extra);
                    break;
                }
            }
            let table = payload(e, data, 0x0c);
            if e.mem.u32(table) == e.mem.u32(table + 4) {
                let first = e.mem.u32(table);
                if first != 0 {
                    let first_id = form_id(e, first);
                    let reference_id = form_id(e, reference.addr());
                    e.call(
                        LOG_MESSAGE,
                        &args![MESSAGE_PORTAL_ROOMS_SAME, reference_id, first_id],
                    );
                    let table = payload(e, data, 0x0c);
                    e.mem.set_u32(table + 4, 0);
                } else {
                    let reference_id = form_id(e, reference.addr());
                    e.call(LOG_MESSAGE, &args![MESSAGE_PORTAL_ROOMS_NULL, reference_id]);
                }
            }
            for index in 0..2u32 {
                let table = payload(e, data, 0x0c);
                let linked = e.mem.u32(table + index * 4);
                if linked != 0 {
                    let linked_list = reference_extra_list(e, linked);
                    e.call(PORTAL_ADD_REFERENCE, &args![linked_list, reference]);
                }
            }
        }
        // EXTRA_ROOM_REF_DATA: the linked rooms of the list at +8.
        0x7b => {
            let data = payload(e, extra, 0x0c);
            let mut node = data + 8;
            while node != 0 && list_item(e, node) != 0 {
                let stored = list_item(e, node);
                let (id, room) = resolve_form_id(e, file, stored, RTTI_TES_OBJECT_REFR);
                if id != 0 && room == 0 {
                    e.call(LOG_MESSAGE, &args![MESSAGE_ROOM_REF_MISSING, id]);
                    remove_and_delete(e, this, extra);
                    break;
                }
                if id != 0 {
                    e.with_stack(4, |e, slot| {
                        e.mem.set_u32(slot.addr(), room);
                        e.call(LIST_SET_ITEM, &args![node, slot]);
                    });
                }
                node = list_next(e, node);
            }
        }
        // EXTRA_IMPACTSWAP
        0x8c => {
            let swap = payload(e, extra, 0x0c);
            if swap != 0 {
                e.call(IMPACT_SWAP_INIT_ITEM, &args![swap, reference]);
            }
        }
        // EXTRA_LIT_WATER_REFS
        0x85 => {
            e.call(LIT_WATER_REFS_INIT_ITEM, &args![extra, reference]);
            if e.call(LIST_IS_EMPTY, &args![extra.addr() + 0x0c]).bool() {
                e.call(LOG_MESSAGE, &args![MESSAGE_EMPTY_LIT_WATER]);
                remove_and_delete(e, this, extra);
            }
        }
        // EXTRA_AUDIOMARKER: the controller id of the AudioMarkerData
        // becomes the controller form.
        0x90 => {
            let marker = payload(e, extra, 0x0c);
            let stored = e.call(AUDIO_MARKER_GET_CONTROLLER, &args![marker]).u32();
            let controller_file = e
                .call(TES_FORM_GET_FILE, &args![reference, 0xffff_ffffu32])
                .u32();
            let (id, controller) =
                resolve_form_id(e, controller_file, stored, RTTI_MEDIA_LOCATION_CONTROLLER);
            if controller != 0 {
                e.call(AUDIO_MARKER_SET_CONTROLLER, &args![marker, id]);
            } else {
                e.call(LOG_MESSAGE, &args![MESSAGE_CONTROLLER_MISSING, id]);
            }
        }
        // EXTRA_AUDIOBUOYMARKER: nothing is done (the word is read only).
        0x91 => {}
        _ => {}
    }
}

/// `EXTRA_ACTIVATE_REF` of `InitItem`: resolves the reference of each
/// REF_ACTIVATE_DATA of the parent list, drops the entries whose reference
/// is gone (deleting their data) and tells the others' lists about this
/// reference (`AddActivateRefChild`, `0041edd0`); an extra data left with no
/// entries, no flags and no text is removed.
pub(crate) fn init_activate_ref(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    reference: Ptr,
    file: u32,
    extra: Ptr<BSExtraData>,
) {
    let mut node = extra.addr() + 0x0c;
    let mut previous = 0u32;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let data = list_item(e, node);
        let stored = e.mem.u32(data);
        let (id, activator) = resolve_form_id(e, file, stored, RTTI_TES_OBJECT_REFR);
        e.mem.set_u32(data, activator);
        if activator == 0 {
            e.call(LOG_MESSAGE, &args![MESSAGE_ACTIVATE_REF_MISSING, id]);
            if previous != 0 {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), data);
                    e.call(LIST_REMOVE_AFTER, &args![previous, slot]);
                });
                node = list_next(e, previous);
            } else {
                e.call(LIST_REMOVE_HEAD, &args![node]);
            }
            e.call(OPERATOR_DELETE, &args![data]);
        } else {
            let activator_list = reference_extra_list(e, activator);
            e.call(ADD_ACTIVATE_REF_CHILD, &args![activator_list, reference]);
            previous = node;
            node = list_next(e, node);
        }
    }
    if e.call(LIST_IS_EMPTY, &args![extra.addr() + 0x0c]).bool()
        && e.mem.u8(extra.addr() + 0x14) == 0
        && e.call(STRING_LENGTH, &args![extra.addr() + 0x18]).u32() == 0
    {
        e.call(LOG_MESSAGE, &args![MESSAGE_EMPTY_ACTIVATE_PARENT]);
        remove_and_delete(e, this, extra);
    }
}
// ---------------------------------------------------------------------------
// Third batch: the rotation matrix builder, the occlusion plane link setter,
// and the getters and setters of single-value extra data (`004168a0`,
// `004181c0` to `00418a80`).

/// `EXTRA_DATA_TYPE` values (Xbox PDB) that more than one function of this
/// batch uses.
pub(crate) const EXTRA_GHOST: u8 = 0x1f;
pub(crate) const EXTRA_ORIGINAL_REFERENCE: u8 = 0x20;
pub(crate) const EXTRA_LEVELED_ITEM: u8 = 0x2f;
pub(crate) const EXTRA_RADIO_DATA: u8 = 0x68;
/// `-1.0f` in the exe's read-only data: the default of the health and charge
/// getters.
pub(crate) const MINUS_ONE: u32 = 0x0101_2054;
/// `ExtraRadioData`'s vtable (slot 0 `0041b680`, slot 1 `Compare` `00437290`).
pub(crate) const VTABLE_EXTRA_RADIO_DATA: u32 = 0x0101_5138;
/// Copy of a `RADIO_DATA` record (`00437240`, `this` = destination, one
/// argument: the source).
pub(crate) const RADIO_DATA_COPY: u32 = 0x0043_7240;
/// `ExtraOriginalReference::ExtraOriginalReference(reference)` (`00431950`,
/// `this` = the new block).
pub(crate) const EXTRA_ORIGINAL_REFERENCE_INIT: u32 = 0x0043_1950;
/// `BGSSaveFormBuffer::GetForm` (`007af430`): `MOV EAX,[ECX+0x20]`.
pub(crate) const SAVE_FORM_BUFFER_GET_FORM: u32 = 0x007a_f430;

layout! {
    /// `BSSoundHandle` (Xbox PDB), 0x0C bytes: the id of the playing sound,
    /// whether success is assumed, and the assumed state.
    pub struct BSSoundHandle: 0x0C {
        /// `iSoundID` (Xbox PDB); `0xFFFFFFFF` for no sound.
        0x00 iSoundID: u32,
        /// `bAssumeSuccess` (Xbox PDB).
        0x04 bAssumeSuccess: u8,
        /// `eState` (Xbox PDB), `BSSoundHandle::ASSUMED_STATE`.
        0x08 eState: u32,
    }
}

/// The first extra data of `extra_type` in the list (`GetExtraData`).
pub(crate) fn find_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
) -> Ptr<BSExtraData> {
    base_extra_list_get_extra_data(e, list.cast(), extra_type)
}

/// The word at +0x0C of the first extra data of `extra_type`, or `default`
/// when the list has none.
pub(crate) fn extra_word_or(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    default: u32,
) -> u32 {
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        default
    } else {
        payload(e, extra, 0x0c)
    }
}

/// The `float` at +0x0C of the first extra data of `extra_type`, or the
/// `float` with the bits `default`; loaded and returned through the x87
/// stack, so a signalling NaN comes out quiet.
pub(crate) fn extra_float_or(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    default: u32,
) -> f32 {
    let bits = extra_word_or(e, list, extra_type, default);
    f32::from_bits(x87_quiet_bits(bits))
}

/// A sound getter's body: copies the `BSSoundHandle` at +0x0C of the first
/// extra data of `extra_type` into `out` (`00418900`), or stores the empty
/// handle (`004188d0`). Returns `out`.
pub(crate) fn extra_sound_or_empty(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    out: Ptr,
) -> Ptr {
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        fn_004188d0(e, out)
    } else {
        fn_00418900(e, out, Ptr::new(extra.addr() + 0x0c))
    }
}

// Translated from 004168a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 3x3 rotation matrix of `angle` radians about the axis `(x, y, z)`
/// (Rodrigues' formula, nine `float`s at `this`: `c + x*x*(1-c)`,
/// `x*y*(1-c) + z*s`, `x*z*(1-c) - y*s`, `x*y*(1-c) - z*s`, ...). The sine
/// and cosine come from `004169a0`. The code computes on the x87 stack and
/// rounds to `float` where it stores a temporary; the translation computes
/// in `f64` and rounds at the same stores.
pub fn fn_004168a0(e: &mut Engine, this: Ptr, angle: f32, x: f32, y: f32, z: f32) {
    let (sine, cosine) = e.with_stack(8, |e, scratch| {
        fn_004169a0(e, angle, scratch, Ptr::new(scratch.addr() + 4));
        (
            e.mem.f32(scratch.addr()) as f64,
            e.mem.f32(scratch.addr() + 4) as f64,
        )
    });
    let (x, y, z) = (x as f64, y as f64, z as f64);
    let one_minus_cos = (1.0 - cosine) as f32 as f64;
    let xx = (x * x) as f32 as f64;
    let yy = (y * y) as f32 as f64;
    let zz = (z * z) as f32 as f64;
    let xy = (x * y * one_minus_cos) as f32 as f64;
    let xz = (x * z * one_minus_cos) as f32 as f64;
    let yz = (y * z * one_minus_cos) as f32 as f64;
    let x_sin = (x * sine) as f32 as f64;
    let y_sin = (y * sine) as f32 as f64;
    let z_sin = (z * sine) as f32 as f64;
    let matrix = [
        xx * one_minus_cos + cosine,
        xy + z_sin,
        xz - y_sin,
        xy - z_sin,
        yy * one_minus_cos + cosine,
        yz + x_sin,
        xz + y_sin,
        yz - x_sin,
        zz * one_minus_cos + cosine,
    ];
    for (index, value) in matrix.into_iter().enumerate() {
        e.mem.set_f32(this.addr() + index as u32 * 4, value as f32);
    }
}

// Translated from 004181c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane` linked plane setter: stores `linked_plane` in slot
/// `index` of the array at +0xEC of the plane (no bounds check; the loader
/// passes 0 to 3).
pub fn fn_004181c0(e: &mut Engine, this: Ptr, index: u32, linked_plane: u32) {
    let slot = this
        .addr()
        .wrapping_add(0xec)
        .wrapping_add(index.wrapping_mul(4));
    e.mem.set_u32(slot, linked_plane);
}

// Translated from 004181e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base form of a reference: the code calls `BGSSaveFormBuffer::GetForm`
/// (`007af430`), which is `MOV EAX,[ECX+0x20]`, and returns its result.
pub fn fn_004181e0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SAVE_FORM_BUFFER_GET_FORM, &args![this]).u32()
}

// Translated from 00418200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetAnimSave` (Xbox PDB): the extra data of type `0x2D`
/// (`EXTRA_ANIM_SAVE` in the Xbox enum), or null.
pub fn extra_data_list_get_anim_save(e: &mut Engine, this: Ptr<ExtraDataList>) -> Ptr<BSExtraData> {
    find_extra(e, this, 0x2d)
}

// Translated from 00418220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetAnimation` (Xbox PDB): the word at +0x0C of the type
/// `0x10` extra data (`EXTRA_ANIM`), or 0.
pub fn extra_data_list_get_animation(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x10, 0)
}

// Translated from 00418250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x0C of the type `0x29` extra data (`EXTRA_LIGHT` in the Xbox
/// enum), or 0.
pub fn fn_00418250(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x29, 0)
}

// Translated from 00418280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetSpellEffectLight` (Xbox PDB): the word at +0x0C of the
/// type `0x40` extra data (`EXTRA_MAGIC_LIGHT` in the Xbox enum), or 0.
pub fn extra_data_list_get_spell_effect_light(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x40, 0)
}

// Translated from 004182b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x0C of the type `0x2A` extra data (`EXTRA_LOCK` in the Xbox
/// enum), or 0.
pub fn fn_004182b0(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x2a, 0)
}

// Translated from 004182e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the `RADIO_DATA` record inside the type `0x68` extra data
/// (`ExtraRadioData`, record at +0x0C), or 0.
pub fn fn_004182e0(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_RADIO_DATA);
    if extra.is_null() {
        0
    } else {
        extra.addr() + 0x0c
    }
}

// Translated from 00418310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the radio data of the list from the `RADIO_DATA` record at `data`:
/// a null `data` removes the type `0x68` extra data; otherwise the record is
/// copied (`00437240`) into the existing extra data, or into a new
/// `ExtraRadioData` (`0x1C` bytes, built by `004183e0`) that is added to the
/// list. The compiler's exception-unwinding frame is not translated.
pub fn fn_00418310(e: &mut Engine, this: Ptr<ExtraDataList>, data: Ptr) {
    if data.is_null() {
        base_extra_list_remove_extra_ov2(e, this.cast(), EXTRA_RADIO_DATA);
        return;
    }
    let existing = find_extra(e, this, EXTRA_RADIO_DATA);
    if !existing.is_null() {
        e.call(RADIO_DATA_COPY, &args![existing.addr() + 0x0c, data]);
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
    let extra = if block == 0 {
        0
    } else {
        fn_004183e0(e, Ptr::new(block)).addr()
    };
    e.call(RADIO_DATA_COPY, &args![extra.wrapping_add(0x0c), data]);
    base_extra_list_add_extra(e, this.cast(), Ptr::new(extra));
}

// Translated from 004183e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraRadioData` (type `0x68`, `0x1C` bytes; its vtable has
/// `ExtraRadioData::Compare`, `00437290`, in slot 1): the `BSExtraData` base,
/// the vtable, the `RADIO_DATA` constructor (`00416b40`) on the record at
/// +0x0C, and then a zero fill of the record's 16 bytes. Returns `this`. The
/// compiler's exception-unwinding frame is not translated.
pub fn fn_004183e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, EXTRA_RADIO_DATA as u32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_RADIO_DATA);
    fn_00416b40(e, Ptr::new(this.addr() + 0x0c));
    e.call(MEMSET, &args![this.addr() + 0x0c, 0u32, 0x10u32]);
    this
}

// Translated from 00418460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x0C of the type `0x2B` extra data (`EXTRA_TELEPORT` in the
/// Xbox enum), or 0.
pub fn fn_00418460(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x2b, 0)
}

// Translated from 00418490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x0C of the type `0x2C` extra data (`EXTRA_MAPMARKER` in the
/// Xbox enum), or 0.
pub fn fn_00418490(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x2c, 0)
}

// Translated from 004184c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x0C of the type `0x90` extra data (`EXTRA_AUDIOMARKER` in
/// the Xbox enum), or 0.
pub fn fn_004184c0(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x90, 0)
}

// Translated from 004184f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x0C of the type `0x91` extra data (`EXTRA_AUDIOBUOYMARKER`
/// in the Xbox enum), or 0.
pub fn fn_004184f0(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x91, 0)
}

// Translated from 00418520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetContainerChanges` (Xbox PDB): the word at +0x0C of the
/// type `0x15` extra data (`EXTRA_CONTAINER_CHANGES` in the Xbox enum), or 0.
pub fn extra_data_list_get_container_changes(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x15, 0)
}

// Translated from 00418550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the original reference of the list to `reference`: stores it in the
/// type `0x20` extra data when there is one, and then, in every case, builds
/// a new `0x10`-byte extra data of that type (`00431950`) and adds it to the
/// list (the code does not skip this when it updated an existing one). The
/// compiler's exception-unwinding frame is not translated.
pub fn fn_00418550(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) {
    let existing = find_extra(e, this, EXTRA_ORIGINAL_REFERENCE);
    if !existing.is_null() {
        e.mem.set_u32(existing.addr() + 0x0c, reference);
    }
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let extra = if block == 0 {
        0
    } else {
        e.call(EXTRA_ORIGINAL_REFERENCE_INIT, &args![block, reference])
            .u32()
    };
    base_extra_list_add_extra(e, this.cast(), Ptr::new(extra));
}

// Translated from 00418600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveOriginalReferenceExtra` (Xbox PDB): removes and
/// deletes the type `0x20` extra data, if the list has one.
pub fn extra_data_list_remove_original_reference_extra(e: &mut Engine, this: Ptr<ExtraDataList>) {
    let extra = find_extra(e, this, EXTRA_ORIGINAL_REFERENCE);
    if !extra.is_null() {
        base_extra_list_remove_extra(e, this.cast(), extra, true);
    }
}

// Translated from 00418630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetOriginalReference` (Xbox PDB): the word at +0x0C of the
/// type `0x20` extra data (`EXTRA_ORIGINAL_REFERENCE`), or 0.
pub fn extra_data_list_get_original_reference(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, EXTRA_ORIGINAL_REFERENCE, 0)
}

// Translated from 00418660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x0C of the type `0x21` extra data (`EXTRA_OWNERSHIP` in the
/// Xbox enum), or 0.
pub fn fn_00418660(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x21, 0)
}

// Translated from 00418690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetGlobal` (Xbox PDB): the word at +0x0C of the type `0x22`
/// extra data (`EXTRA_GLOBAL` in the Xbox enum), or 0.
pub fn extra_data_list_get_global(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x22, 0)
}

// Translated from 004186c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetRank` (Xbox PDB): `iRank` of the type `0x23` extra data
/// (`EXTRA_RANK` in the Xbox enum), or -1.
pub fn extra_data_list_get_rank(e: &mut Engine, this: Ptr<ExtraDataList>) -> i32 {
    extra_word_or(e, this, 0x23, 0xffff_ffff) as i32
}

// Translated from 004186f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetHealth` (Xbox PDB): `fHealth` of the type `0x25` extra
/// data (`EXTRA_HEALTH` in the Xbox enum), or the `float` at `01012054`
/// (-1.0) when the list has none. Returned in `ST0`.
pub fn extra_data_list_get_health(e: &mut Engine, this: Ptr<ExtraDataList>) -> f32 {
    let default = e.mem.u32(MINUS_ONE);
    extra_float_or(e, this, 0x25, default)
}

// Translated from 00418720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetLeveledItem` (Xbox PDB): the type `0x2F` extra data
/// itself (`EXTRA_LEVELITEM` in the Xbox enum), or null.
pub fn extra_data_list_get_leveled_item(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> Ptr<BSExtraData> {
    find_extra(e, this, EXTRA_LEVELED_ITEM)
}

// Translated from 00418750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::HasLeveledItem` (Xbox PDB): whether the list has a type
/// `0x2F` extra data (`BaseExtraList::HasExtra`; the result is `AL`).
pub fn extra_data_list_has_leveled_item(e: &mut Engine, this: Ptr<ExtraDataList>) -> bool {
    base_extra_list_has_extra(e, this.cast(), EXTRA_LEVELED_ITEM)
}

// Translated from 00418770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetCount` (Xbox PDB): `iCount` (a `short`) of the type
/// `0x24` extra data (`EXTRA_COUNT` in the Xbox enum), or 1. Returned in
/// `AX`.
pub fn extra_data_list_get_count(e: &mut Engine, this: Ptr<ExtraDataList>) -> u16 {
    extra_word_or(e, this, 0x24, 1) as u16
}

// Translated from 004187a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fCharge` of the type `0x28` extra data (`EXTRA_CHARGE` in the Xbox enum),
/// or the `float` at `01012054` (-1.0). Returned in `ST0`.
pub fn fn_004187a0(e: &mut Engine, this: Ptr<ExtraDataList>) -> f32 {
    let default = e.mem.u32(MINUS_ONE);
    extra_float_or(e, this, 0x28, default)
}

// Translated from 004187d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetPoison` (Xbox PDB): the word at +0x0C of the type
/// `0x3F` extra data (`EXTRA_POISON` in the Xbox enum), or 0.
pub fn extra_data_list_get_poison(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x3f, 0)
}

// Translated from 00418800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetScript` (Xbox PDB): the word at +0x0C of the type `0x0D`
/// extra data (`EXTRA_SCRIPT` in the Xbox enum), or 0.
pub fn extra_data_list_get_script(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    extra_word_or(e, this, 0x0d, 0)
}

// Translated from 00418830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetScriptLocals` (Xbox PDB): the word at +0x10 of the type
/// `0x0D` extra data (`EXTRA_SCRIPT` in the Xbox enum), or 0.
pub fn extra_data_list_get_script_locals(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, 0x0d);
    if extra.is_null() {
        0
    } else {
        payload(e, extra, 0x10)
    }
}

// Translated from 00418860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fScale` of the type `0x30` extra data (`EXTRA_SCALE` in the Xbox enum),
/// or 1.0. Returned in `ST0`.
pub fn fn_00418860(e: &mut Engine, this: Ptr<ExtraDataList>) -> f32 {
    extra_float_or(e, this, 0x30, 1.0f32.to_bits())
}

// Translated from 00418890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetSound` (Xbox PDB): copies the `BSSoundHandle` of the type
/// `0x4F` extra data (`EXTRA_SOUND` in the Xbox enum, record at +0x0C) into
/// `out`, or stores the empty handle (`fn_004188d0`). Returns `out`.
pub fn extra_data_list_get_sound(e: &mut Engine, this: Ptr<ExtraDataList>, out: Ptr) -> Ptr {
    extra_sound_or_empty(e, this, 0x4f, out)
}

// Translated from 004188d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the empty `BSSoundHandle` at `this`: `iSoundID` 0xFFFFFFFF,
/// `bAssumeSuccess` false, `eState` 0 (the default constructor). Returns
/// `this`.
pub fn fn_004188d0(e: &mut Engine, this: Ptr) -> Ptr {
    let handle: Ptr<BSSoundHandle> = this.cast();
    e.set(handle, BSSoundHandle::iSoundID, 0xffff_ffff);
    e.set(handle, BSSoundHandle::bAssumeSuccess, 0);
    e.set(handle, BSSoundHandle::eState, 0);
    this
}

// Translated from 00418900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a `BSSoundHandle` (the copy constructor): `iSoundID`, the
/// `bAssumeSuccess` byte and `eState` of `source` to `this`. Returns `this`.
pub fn fn_00418900(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let from: Ptr<BSSoundHandle> = source.cast();
    let to: Ptr<BSSoundHandle> = this.cast();
    let sound_id = e.get(from, BSSoundHandle::iSoundID);
    e.set(to, BSSoundHandle::iSoundID, sound_id);
    let assume_success = e.get(from, BSSoundHandle::bAssumeSuccess);
    e.set(to, BSSoundHandle::bAssumeSuccess, assume_success);
    let state = e.get(from, BSSoundHandle::eState);
    e.set(to, BSSoundHandle::eState, state);
    this
}

// Translated from 00418940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetCreatureAwakeSound` (Xbox PDB): as `GetSound`, for the
/// type `0x7D` extra data (`EXTRA_CREATURE_AWAKE_SOUND` in the Xbox enum).
/// Returns `out`.
pub fn extra_data_list_get_creature_awake_sound(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    out: Ptr,
) -> Ptr {
    extra_sound_or_empty(e, this, 0x7d, out)
}

// Translated from 00418980 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `GetSound`, for the type `0x8A` extra data
/// (`EXTRA_CREATURE_MOVEMENT_SOUND` in the Xbox enum). Returns `out`.
pub fn fn_00418980(e: &mut Engine, this: Ptr<ExtraDataList>, out: Ptr) -> Ptr {
    extra_sound_or_empty(e, this, 0x8a, out)
}

// Translated from 004189c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetWeaponIdleSound` (Xbox PDB): as `GetSound`, for the type
/// `0x83` extra data (`EXTRA_WEAPON_IDLE_SOUND` in the Xbox enum). Returns
/// `out`.
pub fn extra_data_list_get_weapon_idle_sound(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    out: Ptr,
) -> Ptr {
    extra_sound_or_empty(e, this, 0x83, out)
}

// Translated from 00418a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetWeaponAttackSound` (Xbox PDB): as `GetSound`, for the
/// type `0x86` extra data (`EXTRA_WEAPON_ATTACK_SOUND` in the Xbox enum).
/// Returns `out`.
pub fn extra_data_list_get_weapon_attack_sound(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    out: Ptr,
) -> Ptr {
    extra_sound_or_empty(e, this, 0x86, out)
}

// Translated from 00418a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetActivateLoopSound` (Xbox PDB): as `GetSound`, for the
/// type `0x87` extra data (`EXTRA_ACTIVATE_LOOP_SOUND` in the Xbox enum).
/// Returns `out`.
pub fn extra_data_list_get_activate_loop_sound(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    out: Ptr,
) -> Ptr {
    extra_sound_or_empty(e, this, 0x87, out)
}

// Translated from 00418a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the list has a type `0x1F` extra data (`EXTRA_GHOST` in the Xbox
/// enum): `BaseExtraList::HasExtra`, as a `bool`.
pub fn fn_00418a80(e: &mut Engine, this: Ptr<ExtraDataList>) -> bool {
    base_extra_list_has_extra(e, this.cast(), EXTRA_GHOST)
}

// ---------------------------------------------------------------------------
// Fourth batch: `00418ab0` to `0041a540` (the getters and setters of the
// worn, seed, weapon mod, animation, lock, teleport, marker, count, health,
// script, scale and sound extra data).

/// Extra data types (`EXTRA_DATA_TYPE` of the Xbox PDB; the number is the
/// `cEtype` byte the PC code uses) of this batch.
pub(crate) const EXTRA_SCRIPT: u8 = 0x0d;
pub(crate) const EXTRA_ACTION: u8 = 0x0e;
pub(crate) const EXTRA_STARTING_POSITION: u8 = 0x0f;
pub(crate) const EXTRA_ANIM: u8 = 0x10;
pub(crate) const EXTRA_CONTAINER_CHANGES: u8 = 0x15;
pub(crate) const EXTRA_WORN: u8 = 0x16;
pub(crate) const EXTRA_WORN_LEFT: u8 = 0x17;
pub(crate) const EXTRA_PACKAGE_START_LOCATION: u8 = 0x18;
pub(crate) const EXTRA_OWNERSHIP: u8 = 0x21;
pub(crate) const EXTRA_GLOBAL: u8 = 0x22;
pub(crate) const EXTRA_RANK: u8 = 0x23;
pub(crate) const EXTRA_COUNT: u8 = 0x24;
pub(crate) const EXTRA_HEALTH: u8 = 0x25;
pub(crate) const EXTRA_USES: u8 = 0x26;
pub(crate) const EXTRA_TIME_LEFT: u8 = 0x27;
pub(crate) const EXTRA_CHARGE: u8 = 0x28;
pub(crate) const EXTRA_LOCK: u8 = 0x2a;
pub(crate) const EXTRA_TELEPORT: u8 = 0x2b;
pub(crate) const EXTRA_MAP_MARKER: u8 = 0x2c;
pub(crate) const EXTRA_SCALE: u8 = 0x30;
pub(crate) const EXTRA_SEED: u8 = 0x31;
pub(crate) const EXTRA_CAN_NOT_WEAR: u8 = 0x3e;
pub(crate) const EXTRA_POISON: u8 = 0x3f;
pub(crate) const EXTRA_TYPE_46: u8 = 0x46;
pub(crate) const EXTRA_CREATURE_AWAKE_SOUND: u8 = 0x7d;
pub(crate) const EXTRA_WEAPON_IDLE_SOUND: u8 = 0x83;
pub(crate) const EXTRA_WEAPON_ATTACK_SOUND: u8 = 0x86;
pub(crate) const EXTRA_CREATURE_MOVEMENT_SOUND: u8 = 0x8a;
pub(crate) const EXTRA_WEAPON_MOD_SLOTS: u8 = 0x8d;
pub(crate) const EXTRA_AUDIO_MARKER: u8 = 0x90;
pub(crate) const EXTRA_AUDIO_BUOY_MARKER: u8 = 0x91;

/// Constructors of the extra data these setters build (`this` = the new
/// block; the other words are the value the setter was given).
pub(crate) const EXTRA_ANIM_INIT: u32 = 0x0043_00c0;
pub(crate) const EXTRA_STARTING_POSITION_INIT: u32 = 0x0043_0780;
pub(crate) const EXTRA_TELEPORT_INIT: u32 = 0x0043_1230;
pub(crate) const EXTRA_MAP_MARKER_INIT: u32 = 0x0043_1360;
pub(crate) const EXTRA_AUDIO_MARKER_INIT: u32 = 0x0043_14c0;
pub(crate) const EXTRA_AUDIO_BUOY_MARKER_INIT: u32 = 0x0043_1620;
pub(crate) const EXTRA_ACTION_INIT: u32 = 0x0043_1780;
pub(crate) const EXTRA_CONTAINER_CHANGES_INIT: u32 = 0x0043_1830;
pub(crate) const EXTRA_OWNERSHIP_INIT: u32 = 0x0043_1a40;
pub(crate) const EXTRA_GLOBAL_INIT: u32 = 0x0043_1ae0;
pub(crate) const EXTRA_RANK_INIT: u32 = 0x0043_1b80;
pub(crate) const EXTRA_COUNT_INIT: u32 = 0x0043_1c20;
pub(crate) const EXTRA_HEALTH_INIT: u32 = 0x0043_1d10;
pub(crate) const EXTRA_USES_INIT: u32 = 0x0043_1e20;
pub(crate) const EXTRA_TIME_LEFT_INIT: u32 = 0x0043_1ec0;
pub(crate) const EXTRA_CHARGE_INIT: u32 = 0x0043_1f60;
pub(crate) const EXTRA_SCALE_INIT: u32 = 0x0043_2220;
pub(crate) const EXTRA_POISON_INIT: u32 = 0x0043_5150;
pub(crate) const EXTRA_ITEM_DROPPER_INIT: u32 = 0x0043_5ec0;
/// Constructors of the four sound extra data; each takes a `BSSoundHandle`
/// by value (three words).
pub(crate) const EXTRA_CREATURE_AWAKE_SOUND_INIT: u32 = 0x0043_61e0;
pub(crate) const EXTRA_CREATURE_MOVEMENT_SOUND_INIT: u32 = 0x0043_6300;
pub(crate) const EXTRA_WEAPON_IDLE_SOUND_INIT: u32 = 0x0043_6420;
pub(crate) const EXTRA_WEAPON_ATTACK_SOUND_INIT: u32 = 0x0043_6540;

/// The scalar deleting destructor `00418d20` of the object an `ExtraAnim`
/// holds (`this`, then flags: bit 0 deletes the block).
pub(crate) const ANIM_SCALAR_DELETING_DESTRUCTOR: u32 = 0x0041_8d20;
/// The destructor `fn_00419220` runs on the teleport data before it deletes.
pub(crate) const TELEPORT_DATA_DESTRUCTOR: u32 = 0x0066_65a0;
/// The destructor `fn_00419350` runs on the map marker data.
pub(crate) const MAP_MARKER_DATA_DESTRUCTOR: u32 = 0x0043_8bf0;
/// The destructor `fn_00419480` runs on the audio marker data.
pub(crate) const AUDIO_MARKER_DATA_DESTRUCTOR: u32 = 0x0058_9500;
/// The scalar deleting destructor of the object an `ExtraAudioBuoyMarker`
/// holds (`this`, then flags).
pub(crate) const AUDIO_BUOY_MARKER_DATA_SCALAR_DELETING_DESTRUCTOR: u32 = 0x007b_3fa0;
/// `TESObjectREFR::SetTargeted(bool)` (`this` = the reference).
pub(crate) const REFERENCE_SET_TARGETED: u32 = 0x0056_4db0;
/// `BSSoundHandle::~BSSoundHandle` (`00483710`, an empty function).
pub(crate) const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
/// `1.0` as a `double` in the exe's read-only data: the value the scale
/// setter compares with.
pub(crate) const ONE_DOUBLE: u32 = 0x0101_2070;

/// A new extra data of `size` bytes, built by the constructor at `construct`
/// (`this` = the block, then `construct_args`) and added to the list. As in
/// the code, a failed allocation gives a null extra data that is added
/// anyway. Returns the extra data.
pub(crate) fn add_new_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    size: u32,
    construct: u32,
    construct_args: &[u32],
) -> Ptr<BSExtraData> {
    let extra = new_object(e, size, |e, block| {
        let mut words = vec![block];
        words.extend_from_slice(construct_args);
        e.call(construct, &words).u32()
    });
    base_extra_list_add_extra(e, list.cast(), Ptr::new(extra));
    Ptr::new(extra)
}

/// The shape of the plain setters: when the list has an extra data of
/// `extra_type`, `store` writes the value into it; otherwise one of `size`
/// bytes is built from `construct_word` by `construct` and added.
pub(crate) fn set_or_add_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    size: u32,
    construct: u32,
    construct_word: u32,
    store: impl FnOnce(&mut Engine, Ptr<BSExtraData>),
) {
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        add_new_extra(e, list, size, construct, &[construct_word]);
    } else {
        store(e, extra);
    }
}

/// The shape of the setters whose value `none` means "no extra data":
/// an existing extra data is deleted for `none` and overwritten (the word
/// at +0x0C) otherwise; a missing one is built (a `0x10`-byte extra data)
/// unless the value is `none`.
pub(crate) fn set_or_remove_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    value: u32,
    none: u32,
) {
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        if value != none {
            add_new_extra(e, list, 0x10, construct, &[value]);
        }
    } else if value == none {
        base_extra_list_remove_extra(e, list.cast(), extra, true);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, value);
    }
}

/// The shape of the setters of a pointer the extra data owns (teleport, map
/// marker, audio marker, audio buoy marker): a null `value` deletes an
/// existing extra data, a non-null one replaces the held pointer (after
/// `destroy_old` ran on a non-null old one) or builds the extra data. The
/// result is the extra data; after a deletion it is the deleted one when
/// `keep_deleted` (the teleport setter) and null otherwise.
pub(crate) fn set_owned_pointer_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    value: u32,
    keep_deleted: bool,
    destroy_old: impl FnOnce(&mut Engine, u32),
) -> Ptr<BSExtraData> {
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        if value == 0 {
            return Ptr::NULL;
        }
        return add_new_extra(e, list, 0x10, construct, &[value]);
    }
    if value == 0 {
        base_extra_list_remove_extra(e, list.cast(), extra, true);
        return if keep_deleted { extra } else { Ptr::NULL };
    }
    let old = payload(e, extra, 0x0c);
    if old != 0 {
        destroy_old(e, old);
    }
    e.mem.set_u32(extra.addr() + 0x0c, value);
    extra
}

// Translated from 00418ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetWorn` (Xbox PDB): with `left_only` set, whether the list
/// has a type `0x17` extra data (`EXTRA_WORN_LEFT`); otherwise whether it has
/// a type `0x16` (`EXTRA_WORN`) or a type `0x17` one.
pub fn extra_data_list_get_worn(e: &mut Engine, this: Ptr<ExtraDataList>, left_only: bool) -> bool {
    if left_only {
        return base_extra_list_has_extra(e, this.cast(), EXTRA_WORN_LEFT);
    }
    base_extra_list_has_extra(e, this.cast(), EXTRA_WORN)
        || base_extra_list_has_extra(e, this.cast(), EXTRA_WORN_LEFT)
}

// Translated from 00418b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetCanNotWear` (Xbox PDB): whether the list has a type
/// `0x3E` extra data (`EXTRA_CANNOTWEAR` in the Xbox enum).
pub fn extra_data_list_get_can_not_wear(e: &mut Engine, this: Ptr<ExtraDataList>) -> bool {
    base_extra_list_has_extra(e, this.cast(), EXTRA_CAN_NOT_WEAR)
}

// Translated from 00418b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `iSeed` (a byte) of the type `0x31` extra data (`ExtraSeed`, `EXTRA_SEED`
/// in the Xbox enum), or `0xFF` when the list has none.
pub fn fn_00418b40(e: &mut Engine, this: Ptr<ExtraDataList>) -> u8 {
    let extra = find_extra(e, this, EXTRA_SEED);
    if extra.is_null() {
        0xff
    } else {
        e.mem.u8(extra.addr() + 0x0c)
    }
}

// Translated from 00418b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetPackageStartLocation` (Xbox PDB): the address of the
/// record at +0x0C of the type `0x18` extra data (`EXTRA_PACKAGESTARTLOC` in
/// the Xbox enum), or 0.
pub fn extra_data_list_get_package_start_location(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_PACKAGE_START_LOCATION);
    if extra.is_null() {
        0
    } else {
        extra.addr() + 0x0c
    }
}

// Translated from 00418ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::HasWeaponMods` (Xbox PDB): false when the list has no type
/// `0x8D` extra data (`EXTRA_WEAPON_MOD_SLOTS` in the Xbox enum); otherwise
/// whether the byte at +0x0C of it is nonzero (`fn_00418be0`).
pub fn extra_data_list_has_weapon_mods(e: &mut Engine, this: Ptr<ExtraDataList>) -> bool {
    let extra = find_extra(e, this, EXTRA_WEAPON_MOD_SLOTS);
    if extra.is_null() {
        false
    } else {
        fn_00418be0(e, extra)
    }
}

// Translated from 00418be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the byte at +0x0C of the extra data is nonzero.
pub fn fn_00418be0(e: &mut Engine, this: Ptr<BSExtraData>) -> bool {
    e.mem.u8(this.addr() + 0x0c) != 0
}

// Translated from 00418c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetWeaponModSlotActive` (Xbox PDB): false when the list has
/// no type `0x8D` extra data; otherwise `fn_00411e20` (the mask test) on it
/// with `slot`.
pub fn extra_data_list_get_weapon_mod_slot_active(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    slot: u8,
) -> bool {
    let extra = find_extra(e, this, EXTRA_WEAPON_MOD_SLOTS);
    if extra.is_null() {
        false
    } else {
        fn_00411e20(e, extra, slot)
    }
}

// Translated from 00418c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetAnimPtr` (Xbox PDB): sets the animation of the type
/// `0x10` extra data (`ExtraAnim`, `pAnimation` at +0x0C). An existing one
/// first runs the destructor `00418d20` (with flags 1) on the animation it
/// held, when it held one, then takes the new pointer; otherwise an
/// `ExtraAnim` (`0x10` bytes, built by `004300c0`) is added. Returns the extra
/// data. The compiler's exception-unwinding frame is not translated.
pub fn extra_data_list_set_anim_ptr(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    animation: u32,
) -> Ptr<BSExtraData> {
    let extra = find_extra(e, this, EXTRA_ANIM);
    if extra.is_null() {
        return add_new_extra(e, this, 0x10, EXTRA_ANIM_INIT, &[animation]);
    }
    let old = payload(e, extra, 0x0c);
    if old != 0 {
        e.call(ANIM_SCALAR_DELETING_DESTRUCTOR, &args![old, 1u32]);
    }
    e.mem.set_u32(extra.addr() + 0x0c, animation);
    extra
}

// Translated from 00418d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The type `0x0F` extra data (`ExtraStartingPosition`, `0x24` bytes, a
/// `FILE_POS_ROT` at +0x0C): returns the list's own when it has one (the
/// argument is not used then); otherwise builds one from `position` with
/// `00430780`, adds it and returns it.
pub fn fn_00418d50(e: &mut Engine, this: Ptr<ExtraDataList>, position: Ptr) -> Ptr<BSExtraData> {
    let extra = find_extra(e, this, EXTRA_STARTING_POSITION);
    if !extra.is_null() {
        return extra;
    }
    add_new_extra(
        e,
        this,
        0x24,
        EXTRA_STARTING_POSITION_INIT,
        &[position.addr()],
    )
}

// Translated from 00419050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetLockPtr` (Xbox PDB): sets the `REFR_LOCK` of the type
/// `0x2A` extra data (`ExtraLock`, `pLock` at +0x0C). An existing one deletes
/// the block it held (`operator delete`, called even for a null pointer) and
/// takes the new one; otherwise an `ExtraLock` (`0x10` bytes, `00430c70`) is
/// added. Returns the extra data.
pub fn extra_data_list_set_lock_ptr(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    lock: u32,
) -> Ptr<BSExtraData> {
    let extra = find_extra(e, this, EXTRA_LOCK);
    if extra.is_null() {
        return add_new_extra(e, this, 0x10, EXTRA_LOCK_INIT, &[lock]);
    }
    let old = payload(e, extra, 0x0c);
    e.call(OPERATOR_DELETE, &args![old]);
    e.mem.set_u32(extra.addr() + 0x0c, lock);
    extra
}

// Translated from 00419120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the teleport data of the type `0x2B` extra data (`ExtraTeleport`,
/// `pTeleportData`, a `DoorTeleportData*`, at +0x0C). A null `data` deletes an
/// existing extra data and returns that (deleted) extra data, or returns null
/// when there is none; a non-null `data` replaces the held pointer, first
/// running `fn_00419220` (flags 1) on the old one when it is non-null, or
/// builds an `ExtraTeleport` (`0x10` bytes, `00431230`) and adds it. Returns
/// the extra data.
pub fn fn_00419120(e: &mut Engine, this: Ptr<ExtraDataList>, data: u32) -> Ptr<BSExtraData> {
    set_owned_pointer_extra(
        e,
        this,
        EXTRA_TELEPORT,
        EXTRA_TELEPORT_INIT,
        data,
        true,
        |e, old| {
            fn_00419220(e, Ptr::new(old), 1);
        },
    )
}

// Translated from 00419220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the teleport data the type `0x2B` extra data
/// holds: runs `006665a0` on `this`, then deletes the block when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00419220(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(TELEPORT_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00419250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the map marker data of the type `0x2C` extra data (`ExtraMapMarker`,
/// `pMapData`, a `MapMarkerData*`, at +0x0C). A null `data` deletes an
/// existing extra data; a non-null one replaces the held pointer, first
/// running `fn_00419350` (flags 1) on the old one when it is non-null, or
/// builds an `ExtraMapMarker` (`0x10` bytes, `00431360`) and adds it. Returns
/// the extra data, or null after a deletion or when there is none.
pub fn fn_00419250(e: &mut Engine, this: Ptr<ExtraDataList>, data: u32) -> Ptr<BSExtraData> {
    set_owned_pointer_extra(
        e,
        this,
        EXTRA_MAP_MARKER,
        EXTRA_MAP_MARKER_INIT,
        data,
        false,
        |e, old| {
            fn_00419350(e, Ptr::new(old), 1);
        },
    )
}

// Translated from 00419350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the map marker data the type `0x2C` extra
/// data holds: runs `00438bf0` on `this`, then deletes the block when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00419350(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MAP_MARKER_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00419380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the audio marker data of the type `0x90` extra data
/// (`ExtraAudioMarker`, `pAudioData`, an `AudioMarkerData*`, at +0x0C). A null
/// `data` deletes an existing extra data; a non-null one replaces the held
/// pointer, first running `fn_00419480` (flags 1) on the old one when it is
/// non-null, or builds an `ExtraAudioMarker` (`0x10` bytes, `004314c0`) and
/// adds it. Returns the extra data, or null after a deletion or when there is
/// none.
pub fn fn_00419380(e: &mut Engine, this: Ptr<ExtraDataList>, data: u32) -> Ptr<BSExtraData> {
    set_owned_pointer_extra(
        e,
        this,
        EXTRA_AUDIO_MARKER,
        EXTRA_AUDIO_MARKER_INIT,
        data,
        false,
        |e, old| {
            fn_00419480(e, Ptr::new(old), 1);
        },
    )
}

// Translated from 00419480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the audio marker data the type `0x90` extra
/// data holds: runs `00589500` on `this`, then deletes the block when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00419480(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(AUDIO_MARKER_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004194b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the data of the type `0x91` extra data (`ExtraAudioBuoyMarker`,
/// `pAudioBuoyData`, an `AudioBuoyMarkerData*`, at +0x0C). A null `data`
/// deletes an existing extra data; a non-null one replaces the held pointer,
/// first running the scalar deleting destructor `007b3fa0` (flags 1) on the
/// old one when it is non-null, or builds an `ExtraAudioBuoyMarker` (`0x10`
/// bytes, `00431620`) and adds it. Returns the extra data, or null after a
/// deletion or when there is none.
pub fn fn_004194b0(e: &mut Engine, this: Ptr<ExtraDataList>, data: u32) -> Ptr<BSExtraData> {
    set_owned_pointer_extra(
        e,
        this,
        EXTRA_AUDIO_BUOY_MARKER,
        EXTRA_AUDIO_BUOY_MARKER_INIT,
        data,
        false,
        |e, old| {
            e.call(
                AUDIO_BUOY_MARKER_DATA_SCALAR_DELETING_DESTRUCTOR,
                &args![old, 1u32],
            );
        },
    )
}

// Translated from 004195b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The type `0x0E` extra data (`ExtraAction`, `0x14` bytes: `eAction` at +0x0C,
/// `pActionRef` at +0x10): returns the list's own when it has one; otherwise
/// builds one with `00431780` (no argument), adds it and returns it.
pub fn fn_004195b0(e: &mut Engine, this: Ptr<ExtraDataList>) -> Ptr<BSExtraData> {
    let extra = find_extra(e, this, EXTRA_ACTION);
    if !extra.is_null() {
        return extra;
    }
    add_new_extra(e, this, 0x14, EXTRA_ACTION_INIT, &[])
}

// Translated from 00419650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetContainerChanges` (Xbox PDB): does nothing for a null
/// `changes`; otherwise stores it (`pChanges` at +0x0C) in the type `0x15`
/// extra data (`ExtraContainerChanges`), or builds one (`0x10` bytes,
/// `00431830`) and adds it. A null value never removes the extra data.
pub fn extra_data_list_set_container_changes(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    changes: u32,
) {
    if changes == 0 {
        return;
    }
    set_or_add_extra(
        e,
        this,
        EXTRA_CONTAINER_CHANGES,
        0x10,
        EXTRA_CONTAINER_CHANGES_INIT,
        changes,
        |e, extra| e.mem.set_u32(extra.addr() + 0x0c, changes),
    );
}

// Translated from 00419700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the owner of the type `0x21` extra data (`ExtraOwnership`, `pOwner` at
/// +0x0C): a null `owner` deletes an existing extra data (and builds none),
/// otherwise the pointer is stored, or an `ExtraOwnership` (`0x10` bytes,
/// `00431a40`) is added.
pub fn fn_00419700(e: &mut Engine, this: Ptr<ExtraDataList>, owner: u32) {
    set_or_remove_extra(e, this, EXTRA_OWNERSHIP, EXTRA_OWNERSHIP_INIT, owner, 0);
}

// Translated from 004197d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the global of the type `0x22` extra data (`ExtraGlobal`, `pGlobal` at
/// +0x0C): a null `global` deletes an existing extra data (and builds none),
/// otherwise the pointer is stored, or an `ExtraGlobal` (`0x10` bytes,
/// `00431ae0`) is added.
pub fn fn_004197d0(e: &mut Engine, this: Ptr<ExtraDataList>, global: u32) {
    set_or_remove_extra(e, this, EXTRA_GLOBAL, EXTRA_GLOBAL_INIT, global, 0);
}

// Translated from 004198a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `iRank` of the type `0x23` extra data (`ExtraRank`): `-1` deletes an
/// existing extra data (and builds none), any other value is stored, or an
/// `ExtraRank` (`0x10` bytes, `00431b80`) is added. The engine map names this
/// body `ExtraDataList::SetRadiation`; it sets the rank (the getter at
/// `004186c0` reads the same type and defaults to -1).
pub fn fn_004198a0(e: &mut Engine, this: Ptr<ExtraDataList>, rank: i32) {
    set_or_remove_extra(e, this, EXTRA_RANK, EXTRA_RANK_INIT, rank as u32, u32::MAX);
}

// Translated from 00419970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetHealth` (Xbox PDB): stores `health` (`fHealth` at +0x0C)
/// in the type `0x25` extra data (`ExtraHealth`), or builds one (`0x10` bytes,
/// `00431d10`) and adds it. The float goes through the x87 stack on the way,
/// so a signalling NaN comes out quiet.
pub fn extra_data_list_set_health(e: &mut Engine, this: Ptr<ExtraDataList>, health: f32) {
    let bits = x87_quiet_bits(health.to_bits());
    set_or_add_extra(
        e,
        this,
        EXTRA_HEALTH,
        0x10,
        EXTRA_HEALTH_INIT,
        bits,
        |e, extra| e.mem.set_u32(extra.addr() + 0x0c, bits),
    );
}

// Translated from 00419a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `uses` (`cUses`, a byte at +0x0C) in the type `0x26` extra data
/// (`ExtraUses`), or builds one (`0x10` bytes, `00431e20`, given the byte
/// zero-extended) and adds it.
pub fn fn_00419a20(e: &mut Engine, this: Ptr<ExtraDataList>, uses: u8) {
    set_or_add_extra(
        e,
        this,
        EXTRA_USES,
        0x10,
        EXTRA_USES_INIT,
        uses as u32,
        |e, extra| e.mem.set_u8(extra.addr() + 0x0c, uses),
    );
}

// Translated from 00419ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetCount` (Xbox PDB): a `count` of 0 or 1 deletes an
/// existing type `0x24` extra data (`ExtraCount`) and builds none; any other
/// value is stored (`iCount`, a `short` at +0x0C) or builds an `ExtraCount`
/// (`0x10` bytes, `00431c20`, given the value zero-extended) and adds it.
pub fn extra_data_list_set_count(e: &mut Engine, this: Ptr<ExtraDataList>, count: i16) {
    let extra = find_extra(e, this, EXTRA_COUNT);
    if count == 0 || count == 1 {
        if !extra.is_null() {
            base_extra_list_remove_extra(e, this.cast(), extra, true);
        }
    } else if extra.is_null() {
        add_new_extra(e, this, 0x10, EXTRA_COUNT_INIT, &[count as u16 as u32]);
    } else {
        e.mem.set_u16(extra.addr() + 0x0c, count as u16);
    }
}

// Translated from 00419bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `time_left` (`fTime` at +0x0C) in the type `0x27` extra data
/// (`ExtraTimeLeft`), or builds one (`0x10` bytes, `00431ec0`) and adds it.
/// The float goes through the x87 stack on the way, so a signalling NaN comes
/// out quiet.
pub fn fn_00419bb0(e: &mut Engine, this: Ptr<ExtraDataList>, time_left: f32) {
    let bits = x87_quiet_bits(time_left.to_bits());
    set_or_add_extra(
        e,
        this,
        EXTRA_TIME_LEFT,
        0x10,
        EXTRA_TIME_LEFT_INIT,
        bits,
        |e, extra| e.mem.set_u32(extra.addr() + 0x0c, bits),
    );
}

// Translated from 00419c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `charge` (`fCharge` at +0x0C) in the type `0x28` extra data
/// (`ExtraCharge`), or builds one (`0x10` bytes, `00431f60`) and adds it. The
/// float goes through the x87 stack on the way, so a signalling NaN comes out
/// quiet.
pub fn fn_00419c60(e: &mut Engine, this: Ptr<ExtraDataList>, charge: f32) {
    let bits = x87_quiet_bits(charge.to_bits());
    set_or_add_extra(
        e,
        this,
        EXTRA_CHARGE,
        0x10,
        EXTRA_CHARGE_INIT,
        bits,
        |e, extra| e.mem.set_u32(extra.addr() + 0x0c, bits),
    );
}

// Translated from 00419d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `poison` (`pPoison` at +0x0C) in the type `0x3F` extra data
/// (`ExtraPoison`), or builds one (`0x10` bytes, `00435150`) and adds it. The
/// engine map names this body `ExtraDataList::SetRank`; it sets the poison
/// (the getter at `004187d0` reads the same type).
pub fn fn_00419d10(e: &mut Engine, this: Ptr<ExtraDataList>, poison: u32) {
    set_or_add_extra(
        e,
        this,
        EXTRA_POISON,
        0x10,
        EXTRA_POISON_INIT,
        poison,
        |e, extra| e.mem.set_u32(extra.addr() + 0x0c, poison),
    );
}

// Translated from 00419dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetItemDropper` (Xbox PDB): sets the reference of the type
/// `0x46` extra data (the Xbox enum has `EXTRA_ITEMDROPPER` at `0x39` and
/// `EXTRA_HEAD_TRACK_TARGET` at `0x46`; this code and
/// `ExtraDataList::GetHeadTrackTargetExtra` both use `0x46`). A null `dropper`
/// deletes an existing extra data and does nothing more. Otherwise, and also
/// for a null `dropper` when there is none (an extra data holding null is
/// built), the reference is stored in the existing extra data or in a new one
/// (`0x10` bytes, `00435ec0`); then a non-null `dropper` gets
/// `TESObjectREFR::SetTargeted(true)` (`00564db0`).
pub fn extra_data_list_set_item_dropper(e: &mut Engine, this: Ptr<ExtraDataList>, dropper: u32) {
    let extra = find_extra(e, this, EXTRA_TYPE_46);
    if dropper == 0 && !extra.is_null() {
        base_extra_list_remove_extra(e, this.cast(), extra, true);
        return;
    }
    if extra.is_null() {
        add_new_extra(e, this, 0x10, EXTRA_ITEM_DROPPER_INIT, &[dropper]);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, dropper);
    }
    if dropper != 0 {
        e.call(REFERENCE_SET_TARGETED, &args![dropper, 1u32]);
    }
}

// Translated from 00419ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetHeadTrackTargetExtra` (Xbox PDB): the word at +0x0C of
/// the type `0x46` extra data, or 0.
pub fn extra_data_list_get_head_track_target_extra(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
) -> u32 {
    extra_word_or(e, this, EXTRA_TYPE_46, 0)
}

// Translated from 00419ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetScript` (Xbox PDB): stores `script` (`pScript` at
/// +0x0C) in the type `0x0D` extra data (`ExtraScript`), or builds one (`0x14`
/// bytes, `00432000`) and adds it.
pub fn extra_data_list_set_script(e: &mut Engine, this: Ptr<ExtraDataList>, script: u32) {
    set_or_add_extra(
        e,
        this,
        EXTRA_SCRIPT,
        0x14,
        EXTRA_SCRIPT_INIT,
        script,
        |e, extra| e.mem.set_u32(extra.addr() + 0x0c, script),
    );
}

// Translated from 00419f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `locals` (`pScriptVars` at +0x10) in the type `0x0D` extra data
/// (`ExtraScript`) when the list has one; does nothing otherwise.
pub fn fn_00419f80(e: &mut Engine, this: Ptr<ExtraDataList>, locals: u32) {
    let extra = find_extra(e, this, EXTRA_SCRIPT);
    if !extra.is_null() {
        e.mem.set_u32(extra.addr() + 0x10, locals);
    }
}

// Translated from 00419fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetScale` (Xbox PDB): a `scale` equal to the `double` 1.0
/// at `01012070` deletes an existing type `0x30` extra data (`ExtraScale`) and
/// builds none; any other value (a NaN included) is stored (`fScale` at
/// +0x0C) or builds an `ExtraScale` (`0x10` bytes, `00432220`) and adds it.
/// The float goes through the x87 stack on the way, so a signalling NaN comes
/// out quiet.
pub fn extra_data_list_set_scale(e: &mut Engine, this: Ptr<ExtraDataList>, scale: f32) {
    let extra = find_extra(e, this, EXTRA_SCALE);
    let is_one = scale as f64 == e.mem.f64(ONE_DOUBLE);
    let bits = x87_quiet_bits(scale.to_bits());
    if extra.is_null() {
        if !is_one {
            add_new_extra(e, this, 0x10, EXTRA_SCALE_INIT, &[bits]);
        }
    } else if is_one {
        base_extra_list_remove_extra(e, this.cast(), extra, true);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, bits);
    }
}

/// The body of the four sound setters. With an extra data of `extra_type`
/// in the list: a sound handle whose id equals that of an empty handle
/// (`fn_0041a1f0` against a fresh `fn_0041a250` handle) deletes the extra
/// data, any other is copied into it (`fn_00418900`, the `BSSoundHandle` at
/// +0x0C). Without one, a non-empty handle (`fn_0041a220`) builds an extra
/// data of `0x18` bytes with `construct`, which takes the handle by value (a
/// copy made by `fn_00418900` on the stack, three words), and adds it; an
/// empty handle does nothing. The temporary empty handles are destroyed by
/// `00483710`.
pub(crate) fn set_sound_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    sound: Ptr<BSSoundHandle>,
) {
    let extra = find_extra(e, list, extra_type);
    if !extra.is_null() {
        let is_empty = e.with_stack(0x0c, |e, empty| {
            fn_0041a250(e, empty);
            let equal = fn_0041a1f0(e, sound.cast(), empty);
            e.call(SOUND_HANDLE_DESTRUCTOR, &args![empty]);
            equal
        });
        if is_empty {
            base_extra_list_remove_extra(e, list.cast(), extra, true);
        } else {
            fn_00418900(e, Ptr::new(extra.addr() + 0x0c), sound.cast());
        }
        return;
    }
    let differs = e.with_stack(0x0c, |e, empty| {
        fn_0041a250(e, empty);
        let differs = fn_0041a220(e, sound.cast(), empty);
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![empty]);
        differs
    });
    if !differs {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
    let new_extra = if block == 0 {
        0
    } else {
        // The handle is passed by value: a copy on the stack, three words.
        let words = e.with_stack(0x0c, |e, copy| {
            fn_00418900(e, copy, sound.cast());
            [
                e.mem.u32(copy.addr()),
                e.mem.u32(copy.addr() + 4),
                e.mem.u32(copy.addr() + 8),
            ]
        });
        e.call(construct, &args![block, words[0], words[1], words[2]])
            .u32()
    };
    base_extra_list_add_extra(e, list.cast(), Ptr::new(new_extra));
}

// Translated from 0041a090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetCreatureAwakeSound` (Xbox PDB): sets the `BSSoundHandle`
/// at +0x0C of the type `0x7D` extra data (`ExtraCreatureAwakeSound`, `0x18`
/// bytes, built by `004361e0`); see `set_sound_extra` for the cases. `sound`
/// is a pointer to the handle.
pub fn extra_data_list_set_creature_awake_sound(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    sound: Ptr<BSSoundHandle>,
) {
    set_sound_extra(
        e,
        this,
        EXTRA_CREATURE_AWAKE_SOUND,
        EXTRA_CREATURE_AWAKE_SOUND_INIT,
        sound,
    );
}

// Translated from 0041a1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the sound handle `this` and `other` have the same `iSoundID`
/// (each read through `00559450`, `other` first); the other fields are not
/// compared.
pub fn fn_0041a1f0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let other_id = e.call(READ_WORD, &args![other]).u32();
    let this_id = e.call(READ_WORD, &args![this]).u32();
    other_id == this_id
}

// Translated from 0041a220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the sound handle `this` and `other` have different `iSoundID`s
/// (each read through `00559450`, `other` first).
pub fn fn_0041a220(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let other_id = e.call(READ_WORD, &args![other]).u32();
    let this_id = e.call(READ_WORD, &args![this]).u32();
    other_id != this_id
}

// Translated from 0041a250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default constructor of a `BSSoundHandle` (as `fn_004188d0`): `iSoundID`
/// 0xFFFFFFFF, `bAssumeSuccess` false, `eState` 0. Returns `this`.
pub fn fn_0041a250(e: &mut Engine, this: Ptr) -> Ptr {
    let handle: Ptr<BSSoundHandle> = this.cast();
    e.set(handle, BSSoundHandle::iSoundID, 0xffff_ffff);
    e.set(handle, BSSoundHandle::bAssumeSuccess, 0);
    e.set(handle, BSSoundHandle::eState, 0);
    this
}

// Translated from 0041a280 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `ExtraDataList::SetCreatureAwakeSound`, for the type `0x8A` extra data
/// (`EXTRA_CREATURE_MOVEMENT_SOUND` in the Xbox enum, built by `00436300`).
pub fn fn_0041a280(e: &mut Engine, this: Ptr<ExtraDataList>, sound: Ptr<BSSoundHandle>) {
    set_sound_extra(
        e,
        this,
        EXTRA_CREATURE_MOVEMENT_SOUND,
        EXTRA_CREATURE_MOVEMENT_SOUND_INIT,
        sound,
    );
}

// Translated from 0041a3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetWeaponIdleSound` (Xbox PDB): as
/// `ExtraDataList::SetCreatureAwakeSound`, for the type `0x83` extra data
/// (`EXTRA_WEAPON_IDLE_SOUND` in the Xbox enum, built by `00436420`).
pub fn extra_data_list_set_weapon_idle_sound(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    sound: Ptr<BSSoundHandle>,
) {
    set_sound_extra(
        e,
        this,
        EXTRA_WEAPON_IDLE_SOUND,
        EXTRA_WEAPON_IDLE_SOUND_INIT,
        sound,
    );
}

// Translated from 0041a540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetWeaponAttackSound` (Xbox PDB): as
/// `ExtraDataList::SetCreatureAwakeSound`, for the type `0x86` extra data
/// (`EXTRA_WEAPON_ATTACK_SOUND` in the Xbox enum, built by `00436540`).
pub fn extra_data_list_set_weapon_attack_sound(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    sound: Ptr<BSSoundHandle>,
) {
    set_sound_extra(
        e,
        this,
        EXTRA_WEAPON_ATTACK_SOUND,
        EXTRA_WEAPON_ATTACK_SOUND_INIT,
        sound,
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0040f680,
            fn_0040f680(Ptr<ExtraNorthRotation>) -> Ptr<ExtraNorthRotation>
        ),
        entry!(
            0x0040f6b0,
            fn_0040f6b0(Ptr<ExtraDetachTime>) -> Ptr<ExtraDetachTime>
        ),
        entry!(
            0x0040f700,
            bs_extra_data_compare(Ptr<BSExtraData>, Ptr<BSExtraData>) -> bool
        ),
        entry!(
            0x0040f740,
            fn_0040f740(Ptr<BaseExtraList>) -> Ptr<BaseExtraList>
        ),
        entry!(
            0x0040f780,
            fn_0040f780(Ptr<BaseExtraList>, u32) -> Ptr<BaseExtraList>
        ),
        entry!(0x0040f7b0, fn_0040f7b0(Ptr<BaseExtraList>)),
        entry!(
            0x0040f7d0,
            base_extra_list_clear_last_extra(Ptr<BaseExtraList>, i32)
        ),
        entry!(
            0x0040f860,
            base_extra_list_clear_last_extra_all(Ptr<BaseExtraList>)
        ),
        entry!(0x0040f900, fn_0040f900()),
        entry!(0x0040f980, fn_0040f980(u32) -> bool),
        entry!(
            0x0040f9e0,
            fn_0040f9e0(Ptr<BaseExtraList>, i32) -> Ptr<BSExtraData>
        ),
        entry!(
            0x0040fa40,
            fn_0040fa40(Ptr<BaseExtraList>, Ptr<BSExtraData>)
        ),
        entry!(
            0x0040fae0,
            base_extra_list_remove_all(Ptr<BaseExtraList>, bool)
        ),
        entry!(
            0x0040fcb0,
            base_extra_list_remove_all_default(Ptr<BaseExtraList>, bool)
        ),
        entry!(
            0x0040fe20,
            base_extra_list_items_in_list(Ptr<BaseExtraList>) -> i32
        ),
        entry!(
            0x0040fe80,
            base_extra_list_has_extra(Ptr<BaseExtraList>, u8) -> bool
        ),
        entry!(0x0040fee0, fn_0040fee0(Ptr<BaseExtraList>, u8, bool)),
        entry!(
            0x0040ff60,
            base_extra_list_add_extra(Ptr<BaseExtraList>, Ptr<BSExtraData>) -> Ptr<BSExtraData>
        ),
        entry!(
            0x00410020,
            base_extra_list_remove_extra(Ptr<BaseExtraList>, Ptr<BSExtraData>, bool)
        ),
        entry!(
            0x00410140,
            base_extra_list_remove_extra_ov2(Ptr<BaseExtraList>, u8)
        ),
        entry!(
            0x00410220,
            base_extra_list_get_extra_data(Ptr<BaseExtraList>, u8) -> Ptr<BSExtraData>
        ),
        entry!(
            0x004102d0,
            base_extra_list_get_prev_extra_data(Ptr<BaseExtraList>, u8) -> Ptr<BSExtraData>
        ),
        entry!(
            0x00410360,
            extra_data_list_extra_data_list(Ptr<ExtraDataList>) -> Ptr<ExtraDataList>
        ),
        entry!(
            0x00410380,
            fn_00410380(Ptr<ExtraDataList>, u32) -> Ptr<ExtraDataList>
        ),
        entry!(0x004103b0, fn_004103b0(Ptr<ExtraDataList>)),
        entry!(
            0x004103d0,
            extra_data_list_add_extra_copy(Ptr<ExtraDataList>, Ptr<BSExtraData>)
        ),
        entry!(0x00411b00, fn_00411b00(Ptr) -> Ptr),
        entry!(
            0x00411b40,
            fn_00411b40(Ptr<ExtraRefList>) -> Ptr<ExtraRefList>
        ),
        entry!(
            0x00411bb0,
            extra_reflected_refs_scalar_deleting_destructor(
                Ptr<ExtraRefList>,
                u32,
            ) -> Ptr<ExtraRefList>
        ),
        entry!(
            0x00411be0,
            fn_00411be0(Ptr<ExtraRefList>) -> Ptr<ExtraRefList>
        ),
        entry!(
            0x00411c50,
            extra_reflector_refs_scalar_deleting_destructor(
                Ptr<ExtraRefList>,
                u32,
            ) -> Ptr<ExtraRefList>
        ),
        entry!(
            0x00411c80,
            fn_00411c80(Ptr<ExtraRefList>) -> Ptr<ExtraRefList>
        ),
        entry!(
            0x00411cf0,
            extra_water_light_refs_scalar_deleting_destructor(
                Ptr<ExtraRefList>,
                u32,
            ) -> Ptr<ExtraRefList>
        ),
        entry!(
            0x00411d20,
            fn_00411d20(Ptr<ExtraRefList>) -> Ptr<ExtraRefList>
        ),
        entry!(
            0x00411d90,
            extra_lit_water_refs_scalar_deleting_destructor(
                Ptr<ExtraRefList>,
                u32,
            ) -> Ptr<ExtraRefList>
        ),
        entry!(0x00411dc0, fn_00411dc0(Ptr) -> Ptr),
        entry!(0x00411e00, fn_00411e00(Ptr) -> Ptr),
        entry!(0x00411e20, fn_00411e20(Ptr<BSExtraData>, u8) -> bool),
        entry!(0x00411e40, fn_00411e40(Ptr) -> Ptr),
        entry!(0x00411e70, fn_00411e70(Ptr, u32) -> Ptr),
        entry!(0x00411ea0, fn_00411ea0(Ptr)),
        entry!(
            0x00411ec0,
            extra_data_list_copy_list(Ptr<ExtraDataList>, Ptr<ExtraDataList>)
        ),
        entry!(
            0x00411f20,
            extra_data_list_is_copyable_extra(Ptr<ExtraDataList>, Ptr<BSExtraData>) -> bool
        ),
        entry!(
            0x00411fd0,
            extra_data_list_remove_all_copyable_extra(Ptr<ExtraDataList>, bool)
        ),
        entry!(
            0x004120b0,
            extra_data_list_remove_non_persistent_cell_data(Ptr<ExtraDataList>)
        ),
        entry!(0x004121b0, fn_004121b0(u32) -> bool),
        entry!(
            0x004121e0,
            extra_data_list_copy_list_for_container(Ptr<ExtraDataList>, Ptr<ExtraDataList>, bool)
        ),
        entry!(
            0x00412380,
            extra_data_list_duplicate_extra_list_for_container(
                Ptr<ExtraDataList>,
                Ptr<ExtraDataList>,
            )
        ),
        entry!(
            0x00412490,
            extra_data_list_copy_list_for_reference(Ptr<ExtraDataList>, Ptr<ExtraDataList>, bool)
        ),
        entry!(
            0x004126c0,
            extra_data_list_compare_list_for_container(
                Ptr<ExtraDataList>,
                Ptr<ExtraDataList>,
                bool,
                bool,
            ) -> bool
        ),
        entry!(
            0x004127e0,
            extra_data_list_compare_list(Ptr<ExtraDataList>, Ptr<ExtraDataList>) -> bool
        ),
        entry!(0x00412970, fn_00412970(Ptr<ExtraDataList>)),
        entry!(0x00413f40, fn_00413f40(u32) -> u32),
        entry!(0x00413f60, fn_00413f60(Ptr, Ptr) -> Ptr),
        entry!(0x00413f90, fn_00413f90(Ptr, Ptr) -> Ptr),
        entry!(0x00413fc0, fn_00413fc0(Ptr, Ptr) -> Ptr),
        entry!(0x00413ff0, fn_00413ff0(Ptr, Ptr) -> Ptr),
        entry!(0x00414010, fn_00414010(Ptr) -> Ptr),
        entry!(0x00414030, fn_00414030(Ptr)),
        entry!(0x004140f0, fn_004140f0(Ptr)),
        entry!(0x004141e0, fn_004141e0(Ptr)),
        entry!(0x00414220, fn_00414220(Ptr)),
        entry!(0x00414290, fn_00414290(Ptr)),
        entry!(0x00414370, fn_00414370(Ptr)),
        entry!(0x004143c0, fn_004143c0(Ptr) -> Ptr),
        entry!(0x004143f0, fn_004143f0(Ptr) -> Ptr),
        entry!(0x00414430, fn_00414430(Ptr, f32, f32, f32, f32) -> Ptr),
        entry!(0x00414470, fn_00414470(Ptr) -> Ptr),
        entry!(
            0x004144a0,
            extra_data_list_load(Ptr<ExtraDataList>, Ptr, Ptr)
        ),
        entry!(0x00416870, fn_00416870(Ptr, f32, f32, f32) -> Ptr),
        entry!(0x004169a0, fn_004169a0(f32, Ptr, Ptr)),
        entry!(0x004169d0, fn_004169d0(Ptr) -> u32),
        entry!(0x004169f0, fn_004169f0(Ptr, Ptr)),
        entry!(0x00416a30, fn_00416a30(Ptr, Ptr)),
        entry!(0x00416a70, fn_00416a70(Ptr, Ptr)),
        entry!(0x00416ab0, fn_00416ab0(Ptr) -> Ptr),
        entry!(0x00416ad0, fn_00416ad0(Ptr, u32, u32) -> Ptr),
        entry!(0x00416b40, fn_00416b40(Ptr) -> Ptr),
        entry!(0x00416b80, fn_00416b80(Ptr) -> Ptr),
        entry!(
            0x00416be0,
            extra_data_list_init_item(Ptr<ExtraDataList>, Ptr)
        ),
        entry!(0x004168a0, fn_004168a0(Ptr, f32, f32, f32, f32)),
        entry!(0x004181c0, fn_004181c0(Ptr, u32, u32)),
        entry!(0x004181e0, fn_004181e0(Ptr) -> u32),
        entry!(
            0x00418200,
            extra_data_list_get_anim_save(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(
            0x00418220,
            extra_data_list_get_animation(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x00418250, fn_00418250(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x00418280,
            extra_data_list_get_spell_effect_light(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x004182b0, fn_004182b0(Ptr<ExtraDataList>) -> u32),
        entry!(0x004182e0, fn_004182e0(Ptr<ExtraDataList>) -> u32),
        entry!(0x00418310, fn_00418310(Ptr<ExtraDataList>, Ptr)),
        entry!(0x004183e0, fn_004183e0(Ptr) -> Ptr),
        entry!(0x00418460, fn_00418460(Ptr<ExtraDataList>) -> u32),
        entry!(0x00418490, fn_00418490(Ptr<ExtraDataList>) -> u32),
        entry!(0x004184c0, fn_004184c0(Ptr<ExtraDataList>) -> u32),
        entry!(0x004184f0, fn_004184f0(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x00418520,
            extra_data_list_get_container_changes(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x00418550, fn_00418550(Ptr<ExtraDataList>, u32)),
        entry!(
            0x00418600,
            extra_data_list_remove_original_reference_extra(Ptr<ExtraDataList>)
        ),
        entry!(
            0x00418630,
            extra_data_list_get_original_reference(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x00418660, fn_00418660(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x00418690,
            extra_data_list_get_global(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x004186c0,
            extra_data_list_get_rank(Ptr<ExtraDataList>) -> i32
        ),
        entry!(
            0x004186f0,
            extra_data_list_get_health(Ptr<ExtraDataList>) -> f32
        ),
        entry!(
            0x00418720,
            extra_data_list_get_leveled_item(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(
            0x00418750,
            extra_data_list_has_leveled_item(Ptr<ExtraDataList>) -> bool
        ),
        entry!(
            0x00418770,
            extra_data_list_get_count(Ptr<ExtraDataList>) -> u16
        ),
        entry!(0x004187a0, fn_004187a0(Ptr<ExtraDataList>) -> f32),
        entry!(
            0x004187d0,
            extra_data_list_get_poison(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x00418800,
            extra_data_list_get_script(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x00418830,
            extra_data_list_get_script_locals(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x00418860, fn_00418860(Ptr<ExtraDataList>) -> f32),
        entry!(
            0x00418890,
            extra_data_list_get_sound(Ptr<ExtraDataList>, Ptr) -> Ptr
        ),
        entry!(0x004188d0, fn_004188d0(Ptr) -> Ptr),
        entry!(0x00418900, fn_00418900(Ptr, Ptr) -> Ptr),
        entry!(
            0x00418940,
            extra_data_list_get_creature_awake_sound(Ptr<ExtraDataList>, Ptr) -> Ptr
        ),
        entry!(0x00418980, fn_00418980(Ptr<ExtraDataList>, Ptr) -> Ptr),
        entry!(
            0x004189c0,
            extra_data_list_get_weapon_idle_sound(Ptr<ExtraDataList>, Ptr) -> Ptr
        ),
        entry!(
            0x00418a00,
            extra_data_list_get_weapon_attack_sound(Ptr<ExtraDataList>, Ptr) -> Ptr
        ),
        entry!(
            0x00418a40,
            extra_data_list_get_activate_loop_sound(Ptr<ExtraDataList>, Ptr) -> Ptr
        ),
        entry!(0x00418a80, fn_00418a80(Ptr<ExtraDataList>) -> bool),
        entry!(
            0x00418ab0,
            extra_data_list_get_worn(Ptr<ExtraDataList>, bool) -> bool
        ),
        entry!(
            0x00418b10,
            extra_data_list_get_can_not_wear(Ptr<ExtraDataList>) -> bool
        ),
        entry!(0x00418b40, fn_00418b40(Ptr<ExtraDataList>) -> u8),
        entry!(
            0x00418b70,
            extra_data_list_get_package_start_location(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x00418ba0,
            extra_data_list_has_weapon_mods(Ptr<ExtraDataList>) -> bool
        ),
        entry!(0x00418be0, fn_00418be0(Ptr<BSExtraData>) -> bool),
        entry!(
            0x00418c00,
            extra_data_list_get_weapon_mod_slot_active(Ptr<ExtraDataList>, u8) -> bool
        ),
        entry!(
            0x00418c40,
            extra_data_list_set_anim_ptr(Ptr<ExtraDataList>, u32) -> Ptr<BSExtraData>
        ),
        entry!(
            0x00418d50,
            fn_00418d50(Ptr<ExtraDataList>, Ptr) -> Ptr<BSExtraData>
        ),
        entry!(
            0x00419050,
            extra_data_list_set_lock_ptr(Ptr<ExtraDataList>, u32) -> Ptr<BSExtraData>
        ),
        entry!(
            0x00419120,
            fn_00419120(Ptr<ExtraDataList>, u32) -> Ptr<BSExtraData>
        ),
        entry!(0x00419220, fn_00419220(Ptr, u32) -> Ptr),
        entry!(
            0x00419250,
            fn_00419250(Ptr<ExtraDataList>, u32) -> Ptr<BSExtraData>
        ),
        entry!(0x00419350, fn_00419350(Ptr, u32) -> Ptr),
        entry!(
            0x00419380,
            fn_00419380(Ptr<ExtraDataList>, u32) -> Ptr<BSExtraData>
        ),
        entry!(0x00419480, fn_00419480(Ptr, u32) -> Ptr),
        entry!(
            0x004194b0,
            fn_004194b0(Ptr<ExtraDataList>, u32) -> Ptr<BSExtraData>
        ),
        entry!(
            0x004195b0,
            fn_004195b0(Ptr<ExtraDataList>) -> Ptr<BSExtraData>
        ),
        entry!(
            0x00419650,
            extra_data_list_set_container_changes(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x00419700, fn_00419700(Ptr<ExtraDataList>, u32)),
        entry!(0x004197d0, fn_004197d0(Ptr<ExtraDataList>, u32)),
        entry!(0x004198a0, fn_004198a0(Ptr<ExtraDataList>, i32)),
        entry!(
            0x00419970,
            extra_data_list_set_health(Ptr<ExtraDataList>, f32)
        ),
        entry!(0x00419a20, fn_00419a20(Ptr<ExtraDataList>, u8)),
        entry!(
            0x00419ad0,
            extra_data_list_set_count(Ptr<ExtraDataList>, i16)
        ),
        entry!(0x00419bb0, fn_00419bb0(Ptr<ExtraDataList>, f32)),
        entry!(0x00419c60, fn_00419c60(Ptr<ExtraDataList>, f32)),
        entry!(0x00419d10, fn_00419d10(Ptr<ExtraDataList>, u32)),
        entry!(
            0x00419dc0,
            extra_data_list_set_item_dropper(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x00419ea0,
            extra_data_list_get_head_track_target_extra(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x00419ed0,
            extra_data_list_set_script(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x00419f80, fn_00419f80(Ptr<ExtraDataList>, u32)),
        entry!(
            0x00419fb0,
            extra_data_list_set_scale(Ptr<ExtraDataList>, f32)
        ),
        entry!(
            0x0041a090,
            extra_data_list_set_creature_awake_sound(Ptr<ExtraDataList>, Ptr<BSSoundHandle>)
        ),
        entry!(0x0041a1f0, fn_0041a1f0(Ptr, Ptr) -> bool),
        entry!(0x0041a220, fn_0041a220(Ptr, Ptr) -> bool),
        entry!(0x0041a250, fn_0041a250(Ptr) -> Ptr),
        entry!(
            0x0041a280,
            fn_0041a280(Ptr<ExtraDataList>, Ptr<BSSoundHandle>)
        ),
        entry!(
            0x0041a3e0,
            extra_data_list_set_weapon_idle_sound(Ptr<ExtraDataList>, Ptr<BSSoundHandle>)
        ),
        entry!(
            0x0041a540,
            extra_data_list_set_weapon_attack_sound(Ptr<ExtraDataList>, Ptr<BSSoundHandle>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Test vtable of the extra data (slot 0 the scalar deleting destructor,
    /// slot 3 the virtual copy `AddExtraCopy` calls for type `0x92`).
    const VTABLE: u32 = 0x0200_0000;
    const DESTRUCTOR: u32 = 0x0200_1000;
    const VIRTUAL_COPY: u32 = 0x0200_1004;

    fn returns(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, address: u32) {
        e.register(address, |_, _| Ret::default());
    }

    /// An engine with working doubles for the small callees every function of
    /// the unit uses: the extra data accessors, `memset`/`memcpy`, the lock
    /// (logged only), `operator new` and `delete`, the `BSExtraData`
    /// constructor, and the list node accessors.
    fn extra_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.put_vtable(VTABLE, &[DESTRUCTOR, 0, 0, VIRTUAL_COPY]);
        e.register(DESTRUCTOR, |_, _| Ret::default());
        e.register(VIRTUAL_COPY, |_, _| Ret::default());
        e.register(GET_TYPE, |e, a| returns(e.mem.u8(a[0] + 4) as u32));
        e.register(GET_NEXT, |e, a| returns(e.mem.u32(a[0] + 8)));
        e.register(SET_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(SIMPLE_LIST_NEXT, |e, a| returns(e.mem.u32(a[0] + 4)));
        e.register(SIMPLE_LIST_ITEM, |_, a| returns(a[0]));
        e.register(READ_WORD, |e, a| returns(e.mem.u32(a[0])));
        e.register(MEMSET, |e, a| {
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            for offset in 0..a[2] {
                let byte = e.mem.u8(a[1] + offset);
                e.mem.set_u8(a[0] + offset, byte);
            }
            Ret::default()
        });
        e.register(BS_EXTRA_DATA_INIT, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            e.mem.set_u32(a[0] + 8, 0);
            returns(a[0])
        });
        e.register(OPERATOR_NEW, |e, a| returns(e.mem.alloc(a[0])));
        stub(&mut e, LOCK);
        stub(&mut e, UNLOCK);
        stub(&mut e, OPERATOR_DELETE);
        stub(&mut e, 0x0096_a2d0);
        e
    }

    /// An extra data of type `extra_type` with room for any payload.
    fn extra_of_type(e: &mut Engine, extra_type: u8) -> Ptr<BSExtraData> {
        let extra: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(extra.addr(), VTABLE);
        e.set(extra, BSExtraData::cEtype, extra_type);
        extra
    }

    /// A list holding extra data of the given types, in order, with the type
    /// bitmap filled in.
    fn list_with(e: &mut Engine, types: &[u8]) -> (Ptr<BaseExtraList>, Vec<Ptr<BSExtraData>>) {
        let list: Ptr<BaseExtraList> = e.new_object();
        let extras: Vec<Ptr<BSExtraData>> = types.iter().map(|&ty| extra_of_type(e, ty)).collect();
        for (index, extra) in extras.iter().enumerate() {
            if let Some(next) = extras.get(index + 1) {
                e.set(*extra, BSExtraData::pNext, next.cast());
            }
            fn_0040fee0(e, list, types[index], true);
        }
        if let Some(first) = extras.first() {
            e.set(list, BaseExtraList::pHead, first.cast());
        }
        (list, extras)
    }

    fn chain_types(e: &Engine, list: Ptr<BaseExtraList>) -> Vec<u8> {
        let mut types = vec![];
        let mut current: Ptr<BSExtraData> = e.get(list, BaseExtraList::pHead).cast();
        while !current.is_null() {
            types.push(e.get(current, BSExtraData::cEtype));
            current = e.get(current, BSExtraData::pNext).cast();
        }
        types
    }

    fn flag_bytes(e: &Engine, list: Ptr<BaseExtraList>) -> Vec<u8> {
        e.mem.bytes(list.addr() + 8, FLAGS_LEN)
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The objects the log shows deleted (the destructor called with 1).
    fn deleted(log: &Log) -> Vec<u32> {
        calls_to(log, DESTRUCTOR).iter().map(|a| a[0]).collect()
    }

    fn assert_locked_and_released(log: &Log) {
        let locks = calls_to(log, LOCK);
        assert!(!locks.is_empty());
        assert_eq!(locks.len(), calls_to(log, UNLOCK).len());
        assert!(locks.iter().all(|a| a[0] == EXTRA_CRIT_SECTION));
    }

    fn cache_of(e: &mut Engine) -> Ptr<BaseExtraListThreadCache> {
        thread_cache(e)
    }

    #[test]
    fn north_rotation_constructor_sets_type_vtable_and_zero() {
        let mut e = extra_engine();
        let extra: Ptr<ExtraNorthRotation> = e.new_object();
        e.set(extra, ExtraNorthRotation::fNorthRot, 3.5);
        let result = e.call(0x0040_f680, &args![extra]).ptr::<()>();
        assert_eq!(result.addr(), extra.addr());
        assert_eq!(e.mem.u8(extra.addr() + 4), 0x43);
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_42a0);
        assert_eq!(e.get(extra, ExtraNorthRotation::fNorthRot), 0.0);
    }

    #[test]
    fn detach_time_constructor_sets_type_vtable_and_zero() {
        let mut e = extra_engine();
        let extra: Ptr<ExtraDetachTime> = e.new_object();
        e.set(extra, ExtraDetachTime::iTime, 99);
        e.call(0x0040_f6b0, &args![extra]);
        assert_eq!(e.mem.u8(extra.addr() + 4), 0x0b);
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_42ac);
        assert_eq!(e.get(extra, ExtraDetachTime::iTime), 0);
    }

    #[test]
    fn compare_is_true_for_a_null_other_or_a_different_type() {
        let mut e = extra_engine();
        let first = extra_of_type(&mut e, 0x10);
        let same = extra_of_type(&mut e, 0x10);
        let other = extra_of_type(&mut e, 0x11);
        assert!(e.call(0x0040_f700, &args![first, Ptr::<()>::NULL]).bool());
        assert!(!e.call(0x0040_f700, &args![first, same]).bool());
        assert!(e.call(0x0040_f700, &args![first, other]).bool());
    }

    #[test]
    fn base_extra_list_constructor_clears_head_and_bitmap() {
        let mut e = extra_engine();
        let list: Ptr<BaseExtraList> = e.new_object();
        e.mem.write(list.addr() + 4, &[0xff; 0x1c]);
        e.call(0x0040_f740, &args![list]);
        assert_eq!(e.mem.u32(list.addr()), 0x0101_4300);
        assert_eq!(e.get(list, BaseExtraList::pHead), Ptr::NULL);
        assert_eq!(flag_bytes(&e, list), vec![0u8; 0x15]);
        // Bytes after the bitmap are not the constructor's.
        assert_eq!(e.mem.u8(list.addr() + 0x1d), 0xff);
    }

    #[test]
    fn base_extra_list_scalar_deleting_destructor_frees_only_on_bit_zero() {
        let mut e = extra_engine();
        let (list, extras) = list_with(&mut e, &[0x10, 0x11]);
        e.call_log = Some(vec![]);
        let result = e.call(0x0040_f780, &args![list, 0u32]).ptr::<()>();
        let log = e.call_log.take().unwrap();
        assert_eq!(result.addr(), list.addr());
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        assert_eq!(deleted(&log), vec![extras[0].addr(), extras[1].addr()]);
        assert_eq!(e.mem.u32(list.addr()), 0x0101_4300);

        e.call_log = Some(vec![]);
        e.call(0x0040_f780, &args![list, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![list.addr()]]);
    }

    #[test]
    fn base_extra_list_destructor_sets_vtable_and_destroys_the_chain() {
        let mut e = extra_engine();
        let (list, extras) = list_with(&mut e, &[0x20]);
        e.call_log = Some(vec![]);
        e.call(0x0040_f7b0, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(list.addr()), 0x0101_4300);
        assert_eq!(deleted(&log), vec![extras[0].addr()]);
        assert_eq!(e.get(list, BaseExtraList::pHead), Ptr::NULL);
    }

    #[test]
    fn clear_last_extra_forgets_only_the_cached_entry_of_the_cached_list() {
        let mut e = extra_engine();
        let list: Ptr<BaseExtraList> = e.new_object();
        let other: Ptr<BaseExtraList> = e.new_object();
        let cache = cache_of(&mut e);
        e.set(cache, BaseExtraListThreadCache::pLastExtraList, list.cast());
        e.mem.set_u32(cache_entry(cache, 5), 0x1234);
        e.mem.set_u32(cache_entry(cache, 6), 0x5678);

        e.call(0x0040_f7d0, &args![other, 5i32]);
        assert_eq!(e.mem.u32(cache_entry(cache, 5)), 0x1234);
        e.call(0x0040_f7d0, &args![list, 5i32]);
        assert_eq!(e.mem.u32(cache_entry(cache, 5)), 0);
        assert_eq!(e.mem.u32(cache_entry(cache, 6)), 0x5678);
        // Out of range: nothing cleared, no fault.
        e.call(0x0040_f7d0, &args![list, -1i32]);
        e.call(0x0040_f7d0, &args![list, 0x93i32]);
        assert_eq!(e.mem.u32(cache_entry(cache, 6)), 0x5678);
        // Four calls: both counters moved by four.
        assert_eq!(e.global::<u32>(DIRTY), 4);
        assert_eq!(e.get(cache, BaseExtraListThreadCache::iThreadDirty), 4);
    }

    #[test]
    fn clear_last_extra_all_empties_the_cache_of_the_cached_list() {
        let mut e = extra_engine();
        let list: Ptr<BaseExtraList> = e.new_object();
        let other: Ptr<BaseExtraList> = e.new_object();
        let cache = cache_of(&mut e);
        e.set(cache, BaseExtraListThreadCache::pLastExtraList, list.cast());
        e.mem.set_u32(cache_entry(cache, 0), 1);
        e.mem.set_u32(cache_entry(cache, 0x92), 2);

        e.call(0x0040_f860, &args![other]);
        assert_eq!(e.mem.u32(cache_entry(cache, 0)), 1);
        e.call(0x0040_f860, &args![list]);
        assert_eq!(e.mem.u32(cache_entry(cache, 0)), 0);
        assert_eq!(e.mem.u32(cache_entry(cache, 0x92)), 0);
        assert_eq!(
            e.get(cache, BaseExtraListThreadCache::pLastExtraList),
            Ptr::NULL
        );
        assert_eq!(e.global::<u32>(DIRTY), 2);
        assert_eq!(e.get(cache, BaseExtraListThreadCache::iThreadDirty), 2);
    }

    #[test]
    fn stale_thread_cache_is_dropped_and_a_fresh_one_kept() {
        let mut e = extra_engine();
        let list: Ptr<BaseExtraList> = e.new_object();
        let cache = cache_of(&mut e);
        e.set(cache, BaseExtraListThreadCache::pLastExtraList, list.cast());
        e.mem.set_u32(cache_entry(cache, 3), 0x77);
        e.set(cache, BaseExtraListThreadCache::iThreadDirty, 5);
        e.set_global(DIRTY, 5u32);
        e.call(0x0040_f900, &args![]);
        assert_eq!(e.mem.u32(cache_entry(cache, 3)), 0x77);
        assert_eq!(
            e.get(cache, BaseExtraListThreadCache::pLastExtraList),
            list.cast()
        );

        e.set_global(DIRTY, 9u32);
        e.call(0x0040_f900, &args![]);
        assert_eq!(e.mem.u32(cache_entry(cache, 3)), 0);
        assert_eq!(
            e.get(cache, BaseExtraListThreadCache::pLastExtraList),
            Ptr::NULL
        );
        assert_eq!(e.get(cache, BaseExtraListThreadCache::iThreadDirty), 9);
    }

    #[test]
    fn head_inserted_types_are_exactly_the_jump_table_entries() {
        let mut e = extra_engine();
        let yes: Vec<u32> = (0..0x100u32)
            .filter(|&t| e.call(0x0040_f980, &args![t]).bool())
            .collect();
        assert_eq!(yes, vec![0x0c, 0x0d, 0x0e, 0x15, 0x2b]);
    }

    #[test]
    fn cached_lookup_needs_the_cached_list_a_valid_type_and_a_fresh_cache() {
        let mut e = extra_engine();
        let list: Ptr<BaseExtraList> = e.new_object();
        let other: Ptr<BaseExtraList> = e.new_object();
        let cache = cache_of(&mut e);
        e.set(cache, BaseExtraListThreadCache::pLastExtraList, list.cast());
        e.mem.set_u32(cache_entry(cache, 0x10), 0xabcd);
        assert_eq!(e.call(0x0040_f9e0, &args![list, 0x10i32]).u32(), 0xabcd);
        assert_eq!(e.call(0x0040_f9e0, &args![other, 0x10i32]).u32(), 0);
        assert_eq!(e.call(0x0040_f9e0, &args![list, 0x93i32]).u32(), 0);
        assert_eq!(e.call(0x0040_f9e0, &args![list, -1i32]).u32(), 0);
        // A global dirty counter that moved on drops the cache first.
        e.set_global(DIRTY, 1u32);
        assert_eq!(e.call(0x0040_f9e0, &args![list, 0x10i32]).u32(), 0);
    }

    #[test]
    fn caching_an_extra_switches_the_cached_list() {
        let mut e = extra_engine();
        let list: Ptr<BaseExtraList> = e.new_object();
        let other: Ptr<BaseExtraList> = e.new_object();
        let first = extra_of_type(&mut e, 0x20);
        let second = extra_of_type(&mut e, 0x21);
        let cache = cache_of(&mut e);

        e.call(0x0040_fa40, &args![Ptr::<()>::NULL, first]);
        e.call(0x0040_fa40, &args![list, Ptr::<()>::NULL]);
        assert_eq!(e.mem.u32(cache_entry(cache, 0x20)), 0);

        e.call(0x0040_fa40, &args![list, first]);
        assert_eq!(e.mem.u32(cache_entry(cache, 0x20)), first.addr());
        e.call(0x0040_fa40, &args![list, second]);
        assert_eq!(e.mem.u32(cache_entry(cache, 0x20)), first.addr());
        assert_eq!(e.mem.u32(cache_entry(cache, 0x21)), second.addr());
        // Another list: the cache starts over.
        e.call(0x0040_fa40, &args![other, second]);
        assert_eq!(e.mem.u32(cache_entry(cache, 0x20)), 0);
        assert_eq!(e.mem.u32(cache_entry(cache, 0x21)), second.addr());
        assert_eq!(
            e.get(cache, BaseExtraListThreadCache::pLastExtraList),
            other.cast()
        );
    }

    #[test]
    fn remove_all_destroys_the_chain_or_only_forgets_it() {
        let mut e = extra_engine();
        let (list, extras) = list_with(&mut e, &[0x10, 0x20, 0x30]);
        e.call_log = Some(vec![]);
        e.call(0x0040_fae0, &args![list, true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            deleted(&log),
            extras.iter().map(|x| x.addr()).collect::<Vec<_>>()
        );
        assert_eq!(e.get(list, BaseExtraList::pHead), Ptr::NULL);
        assert_eq!(flag_bytes(&e, list), vec![0u8; 0x15]);
        assert_locked_and_released(&log);
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_REMOVE_ALL);

        let (list, _) = list_with(&mut e, &[0x10, 0x20]);
        e.call_log = Some(vec![]);
        e.call(0x0040_fae0, &args![list, false]);
        let log = e.call_log.take().unwrap();
        assert!(deleted(&log).is_empty());
        assert_eq!(e.get(list, BaseExtraList::pHead), Ptr::NULL);
        assert_eq!(flag_bytes(&e, list), vec![0u8; 0x15]);
    }

    #[test]
    fn remove_all_default_keeps_the_persistent_types() {
        let mut e = extra_engine();
        // 0x2e and 0x65 stay; 0x10, 0x20 and 0x30 go, wherever they are.
        let (list, extras) = list_with(&mut e, &[0x10, 0x2e, 0x20, 0x65, 0x30]);
        e.call_log = Some(vec![]);
        e.call(0x0040_fcb0, &args![list, true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(chain_types(&e, list), vec![0x2e, 0x65]);
        assert_eq!(
            deleted(&log),
            vec![extras[0].addr(), extras[2].addr(), extras[4].addr()]
        );
        assert!(!base_extra_list_has_extra(&mut e, list, 0x10));
        assert!(base_extra_list_has_extra(&mut e, list, 0x2e));
        assert!(base_extra_list_has_extra(&mut e, list, 0x65));
        assert_locked_and_released(&log);
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_REMOVE_ALL_DEFAULT);

        // Without destroy the removed ones are unlinked but not deleted.
        let (list, _) = list_with(&mut e, &[0x10, 0x84]);
        e.call_log = Some(vec![]);
        e.call(0x0040_fcb0, &args![list, false]);
        let log = e.call_log.take().unwrap();
        assert!(deleted(&log).is_empty());
        assert_eq!(chain_types(&e, list), vec![0x84]);
    }

    #[test]
    fn items_in_list_counts_the_chain_under_the_lock() {
        let mut e = extra_engine();
        let (empty, _) = list_with(&mut e, &[]);
        let (list, _) = list_with(&mut e, &[1, 2, 3]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040_fe20, &args![empty]).i32(), 0);
        assert_eq!(e.call(0x0040_fe20, &args![list]).i32(), 3);
        let log = e.call_log.take().unwrap();
        assert_locked_and_released(&log);
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_ITEMS_IN_LIST);
    }

    #[test]
    fn has_extra_tests_the_bit_and_ignores_types_past_the_bitmap() {
        let mut e = extra_engine();
        let list: Ptr<BaseExtraList> = e.new_object();
        e.mem.set_u8(list.addr() + 8 + 2, 0b0000_0100);
        e.mem.set_u8(list.addr() + 8 + 20, 0b1000_0000);
        assert!(e.call(0x0040_fe80, &args![list, 0x12u8]).bool());
        assert!(!e.call(0x0040_fe80, &args![list, 0x11u8]).bool());
        assert!(!e.call(0x0040_fe80, &args![list, 0x13u8]).bool());
        // Last bit of the last byte: type 0xA7. Type 0xA8 is past the end.
        assert!(e.call(0x0040_fe80, &args![list, 0xa7u8]).bool());
        e.mem.set_u8(list.addr() + 8 + 21, 0xff);
        assert!(!e.call(0x0040_fe80, &args![list, 0xa8u8]).bool());
    }

    #[test]
    fn flag_setter_sets_clears_and_ignores_types_past_the_bitmap() {
        let mut e = extra_engine();
        let list: Ptr<BaseExtraList> = e.new_object();
        e.call(0x0040_fee0, &args![list, 0x2bu8, true]);
        assert_eq!(e.mem.u8(list.addr() + 8 + 5), 0b0000_1000);
        e.call(0x0040_fee0, &args![list, 0x2cu8, true]);
        assert_eq!(e.mem.u8(list.addr() + 8 + 5), 0b0001_1000);
        e.call(0x0040_fee0, &args![list, 0x2bu8, false]);
        assert_eq!(e.mem.u8(list.addr() + 8 + 5), 0b0001_0000);
        e.call(0x0040_fee0, &args![list, 0xffu8, true]);
        assert_eq!(flag_bytes(&e, list)[5], 0b0001_0000);
        assert_eq!(e.mem.u8(list.addr() + 8 + 0x1f), 0);
    }

    #[test]
    fn add_extra_appends_ordinary_types_and_sets_their_bit() {
        let mut e = extra_engine();
        let (list, _) = list_with(&mut e, &[]);
        let first = extra_of_type(&mut e, 0x10);
        let second = extra_of_type(&mut e, 0x11);
        let third = extra_of_type(&mut e, 0x12);
        e.call_log = Some(vec![]);
        let result = e.call(0x0040_ff60, &args![list, first]).ptr::<()>();
        assert_eq!(result.addr(), first.addr());
        e.call(0x0040_ff60, &args![list, second]);
        e.call(0x0040_ff60, &args![list, third]);
        let log = e.call_log.take().unwrap();
        assert_eq!(chain_types(&e, list), vec![0x10, 0x11, 0x12]);
        assert_eq!(e.get(list, BaseExtraList::pHead), first.cast());
        assert!(base_extra_list_has_extra(&mut e, list, 0x11));
        assert_locked_and_released(&log);
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_ADD_EXTRA);
    }

    #[test]
    fn add_extra_puts_the_head_types_in_front() {
        let mut e = extra_engine();
        let (list, _) = list_with(&mut e, &[0x10, 0x11]);
        let front = extra_of_type(&mut e, 0x2b);
        e.call(0x0040_ff60, &args![list, front]);
        assert_eq!(chain_types(&e, list), vec![0x2b, 0x10, 0x11]);
        assert!(base_extra_list_has_extra(&mut e, list, 0x2b));
        // Into an empty list too.
        let (empty, _) = list_with(&mut e, &[]);
        let alone = extra_of_type(&mut e, 0x0e);
        e.call(0x0040_ff60, &args![empty, alone]);
        assert_eq!(chain_types(&e, empty), vec![0x0e]);
    }

    #[test]
    fn remove_extra_unlinks_deletes_and_bumps_the_counters() {
        let mut e = extra_engine();
        let (list, extras) = list_with(&mut e, &[0x10, 0x11, 0x12]);
        let cache = cache_of(&mut e);
        e.set(cache, BaseExtraListThreadCache::pLastExtraList, list.cast());
        e.mem.set_u32(cache_entry(cache, 0x11), extras[1].addr());
        e.call_log = Some(vec![]);
        e.call(0x0041_0020, &args![list, extras[1], true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(chain_types(&e, list), vec![0x10, 0x12]);
        assert_eq!(deleted(&log), vec![extras[1].addr()]);
        assert!(!base_extra_list_has_extra(&mut e, list, 0x11));
        assert_eq!(e.mem.u32(cache_entry(cache, 0x11)), 0);
        // ClearLastExtra and the final bump: two each.
        assert_eq!(e.global::<u32>(DIRTY), 2);
        assert_eq!(e.get(cache, BaseExtraListThreadCache::iThreadDirty), 2);
        assert_locked_and_released(&log);
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_REMOVE_EXTRA);
    }

    #[test]
    fn remove_extra_of_the_head_without_destroy_detaches_it() {
        let mut e = extra_engine();
        let (list, extras) = list_with(&mut e, &[0x10, 0x11]);
        e.call_log = Some(vec![]);
        e.call(0x0041_0020, &args![list, extras[0], false]);
        let log = e.call_log.take().unwrap();
        assert!(deleted(&log).is_empty());
        assert_eq!(chain_types(&e, list), vec![0x11]);
        assert_eq!(e.get(extras[0], BSExtraData::pNext), Ptr::NULL);
        assert!(!base_extra_list_has_extra(&mut e, list, 0x10));
    }

    #[test]
    fn remove_extra_of_null_does_nothing() {
        let mut e = extra_engine();
        let (list, _) = list_with(&mut e, &[0x10]);
        e.call_log = Some(vec![]);
        e.call(0x0041_0020, &args![list, Ptr::<()>::NULL, true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(chain_types(&e, list), vec![0x10]);
    }

    #[test]
    fn remove_extra_by_type_deletes_the_first_match_and_always_clears_the_bit() {
        let mut e = extra_engine();
        let (list, extras) = list_with(&mut e, &[0x10, 0x11, 0x12]);
        e.call_log = Some(vec![]);
        e.call(0x0041_0140, &args![list, 0x11u8]);
        let log = e.call_log.take().unwrap();
        assert_eq!(chain_types(&e, list), vec![0x10, 0x12]);
        assert_eq!(deleted(&log), vec![extras[1].addr()]);
        assert!(!base_extra_list_has_extra(&mut e, list, 0x11));
        assert_locked_and_released(&log);
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_REMOVE_EXTRA);

        // The head.
        e.call(0x0041_0140, &args![list, 0x10u8]);
        assert_eq!(chain_types(&e, list), vec![0x12]);

        // A type the chain does not hold: nothing deleted, bit cleared anyway.
        fn_0040fee0(&mut e, list, 0x40, true);
        e.call_log = Some(vec![]);
        e.call(0x0041_0140, &args![list, 0x40u8]);
        let log = e.call_log.take().unwrap();
        assert!(deleted(&log).is_empty());
        assert_eq!(chain_types(&e, list), vec![0x12]);
        assert!(!base_extra_list_has_extra(&mut e, list, 0x40));
    }

    #[test]
    fn get_extra_data_answers_from_the_bitmap_the_cache_or_a_walk() {
        let mut e = extra_engine();
        let (list, extras) = list_with(&mut e, &[0x10, 0x11, 0x12]);
        let cache = cache_of(&mut e);

        // Not in the bitmap: no lock, no walk.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0041_0220, &args![list, 0x40u8]).u32(), 0);
        assert!(e.call_log.take().unwrap().iter().all(|(a, _)| *a != LOCK));

        // In the bitmap, not cached: walks under the lock and caches it.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0041_0220, &args![list, 0x11u8]).u32(),
            extras[1].addr()
        );
        let log = e.call_log.take().unwrap();
        assert_locked_and_released(&log);
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_GET_EXTRA_DATA);
        assert_eq!(e.mem.u32(cache_entry(cache, 0x11)), extras[1].addr());
        assert_eq!(
            e.get(cache, BaseExtraListThreadCache::pLastExtraList),
            list.cast()
        );

        // Cached now: answered without the lock.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0041_0220, &args![list, 0x11u8]).u32(),
            extras[1].addr()
        );
        assert!(e.call_log.take().unwrap().iter().all(|(a, _)| *a != LOCK));

        // In the bitmap but not in the chain (a stale bit): null.
        fn_0040fee0(&mut e, list, 0x50, true);
        assert_eq!(e.call(0x0041_0220, &args![list, 0x50u8]).u32(), 0);
    }

    #[test]
    fn get_prev_extra_data_is_the_one_before_or_null() {
        let mut e = extra_engine();
        let (list, extras) = list_with(&mut e, &[0x10, 0x11, 0x12]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0041_02d0, &args![list, 0x12u8]).u32(),
            extras[1].addr()
        );
        let log = e.call_log.take().unwrap();
        assert_locked_and_released(&log);
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_GET_PREV_EXTRA_DATA);
        // The head has no previous; an absent type returns before the lock.
        assert_eq!(e.call(0x0041_02d0, &args![list, 0x10u8]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0041_02d0, &args![list, 0x60u8]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn extra_data_list_constructor_and_destructors() {
        let mut e = extra_engine();
        let list: Ptr<ExtraDataList> = e.new_object();
        e.mem.write(list.addr() + 4, &[0xff; 0x19]);
        let result = e.call(0x0041_0360, &args![list]).ptr::<()>();
        assert_eq!(result.addr(), list.addr());
        assert_eq!(e.mem.u32(list.addr()), 0x0101_43e8);
        assert_eq!(e.get(list, ExtraDataList::pHead), Ptr::NULL);
        assert_eq!(e.mem.bytes(list.addr() + 8, 0x15), vec![0u8; 0x15]);

        let extra = extra_of_type(&mut e, 0x10);
        base_extra_list_add_extra(&mut e, list.cast(), extra);
        e.call_log = Some(vec![]);
        e.call(0x0041_03b0, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(deleted(&log), vec![extra.addr()]);
        // The base destructor leaves its own vtable.
        assert_eq!(e.mem.u32(list.addr()), 0x0101_4300);

        let extra = extra_of_type(&mut e, 0x10);
        base_extra_list_add_extra(&mut e, list.cast(), extra);
        e.call_log = Some(vec![]);
        e.call(0x0041_0380, &args![list, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![list.addr()]]);
    }

    // --- AddExtraCopy ---------------------------------------------------

    const IS_COPYABLE: u32 = 0x0041_1f20;

    /// An engine where every setter in the copy tables is a stub, and the
    /// copyability check answers yes.
    fn copy_engine() -> Engine {
        let mut e = extra_engine();
        e.register(IS_COPYABLE, |_, _| returns(1));
        for table in [
            COPY_WORD,
            COPY_FLOAT,
            COPY_BYTE,
            COPY_READ_WORD,
            COPY_PAYLOAD_ADDRESS,
        ] {
            for &(_, setter) in table {
                stub(&mut e, setter);
            }
        }
        for &(_, _, construct, copy, setter) in COPY_NEW_OBJECT {
            e.register(construct, |_, a| returns(a[0]));
            stub(&mut e, copy);
            stub(&mut e, setter);
        }
        e
    }

    fn copy_list(e: &mut Engine) -> Ptr<ExtraDataList> {
        let list: Ptr<ExtraDataList> = e.new_object();
        list
    }

    fn run_copy(e: &mut Engine, list: Ptr<ExtraDataList>, extra: Ptr<BSExtraData>) -> Log {
        e.call_log = Some(vec![]);
        e.call(0x0041_03d0, &args![list, extra]);
        let mut log = e.call_log.take().unwrap();
        // The first entry is the call under test itself.
        log.remove(0);
        log
    }

    /// The calls of a copy except the ones every copy makes (copyability and
    /// type).
    fn copy_calls(log: &Log) -> Log {
        log.iter()
            .filter(|(callee, _)| *callee != IS_COPYABLE && *callee != GET_TYPE)
            .cloned()
            .collect()
    }

    #[test]
    fn copy_does_nothing_for_null_or_uncopyable_extra_data() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        let log = run_copy(&mut e, list, Ptr::NULL);
        assert!(log.is_empty());

        e.register(IS_COPYABLE, |_, _| returns(0));
        let extra = extra_of_type(&mut e, 0x03);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(log, vec![(IS_COPYABLE, vec![list.addr(), extra.addr()])]);
    }

    #[test]
    fn copy_word_types_pass_the_payload_word_to_their_setter() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        for &(extra_type, setter) in COPY_WORD {
            let extra = extra_of_type(&mut e, extra_type);
            e.mem.set_u32(extra.addr() + 0x0c, 0x1234_5678);
            let log = run_copy(&mut e, list, extra);
            assert_eq!(
                copy_calls(&log),
                vec![(setter, vec![list.addr(), 0x1234_5678])],
                "type {extra_type:#x}"
            );
        }
    }

    #[test]
    fn copy_float_byte_and_short_types_pass_the_payload_by_size() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        for &(extra_type, setter) in COPY_FLOAT {
            let extra = extra_of_type(&mut e, extra_type);
            e.mem.set_f32(extra.addr() + 0x0c, 2.5);
            let log = run_copy(&mut e, list, extra);
            assert_eq!(
                copy_calls(&log),
                vec![(setter, vec![list.addr(), 2.5f32.to_bits()])],
                "type {extra_type:#x}"
            );
        }
        for &(extra_type, setter) in COPY_BYTE {
            let extra = extra_of_type(&mut e, extra_type);
            e.mem.set_u32(extra.addr() + 0x0c, 0xaabb_ccdd);
            let log = run_copy(&mut e, list, extra);
            assert_eq!(
                copy_calls(&log),
                vec![(setter, vec![list.addr(), 0xdd])],
                "type {extra_type:#x}"
            );
        }
        // Type 0x24 is a 16-bit payload.
        stub(&mut e, 0x0041_9ad0);
        let extra = extra_of_type(&mut e, 0x24);
        e.mem.set_u32(extra.addr() + 0x0c, 0xaabb_ccdd);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            copy_calls(&log),
            vec![(0x0041_9ad0, vec![list.addr(), 0xccdd])]
        );
        // Type 0x48 passes 1 and then the float.
        stub(&mut e, 0x0042_2750);
        let extra = extra_of_type(&mut e, 0x48);
        e.mem.set_f32(extra.addr() + 0x0c, -4.0);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            copy_calls(&log),
            vec![(0x0042_2750, vec![list.addr(), 1, (-4.0f32).to_bits()])]
        );
    }

    #[test]
    fn copy_handle_and_address_types() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        // The setter gets the word the payload address points at.
        for &(extra_type, setter) in COPY_READ_WORD {
            let extra = extra_of_type(&mut e, extra_type);
            e.mem.set_u32(extra.addr() + 0x0c, 0x4242);
            let log = run_copy(&mut e, list, extra);
            assert_eq!(
                copy_calls(&log),
                vec![
                    (READ_WORD, vec![extra.addr() + 0x0c]),
                    (setter, vec![list.addr(), 0x4242])
                ],
                "type {extra_type:#x}"
            );
        }
        for &(extra_type, setter) in COPY_PAYLOAD_ADDRESS {
            let extra = extra_of_type(&mut e, extra_type);
            let log = run_copy(&mut e, list, extra);
            assert_eq!(
                copy_calls(&log),
                vec![(setter, vec![list.addr(), extra.addr() + 0x0c])],
                "type {extra_type:#x}"
            );
        }
    }

    #[test]
    fn copy_new_object_types_build_copy_and_set() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        for &(extra_type, size, construct, copy, setter) in COPY_NEW_OBJECT {
            let extra = extra_of_type(&mut e, extra_type);
            e.mem.set_u32(extra.addr() + 0x0c, 0x9999);
            let log = run_copy(&mut e, list, extra);
            let calls = copy_calls(&log);
            assert_eq!(calls.len(), 4, "type {extra_type:#x}");
            assert_eq!(calls[0], (OPERATOR_NEW, vec![size]));
            let object = calls[1].1[0];
            assert_eq!(calls[1].0, construct);
            assert_eq!(calls[2], (copy, vec![object, 0x9999]));
            assert_eq!(calls[3], (setter, vec![list.addr(), object]));
        }
    }

    #[test]
    fn copy_of_a_failed_allocation_goes_on_with_null() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        let extra = extra_of_type(&mut e, 0x2b);
        e.mem.set_u32(extra.addr() + 0x0c, 7);
        let log = run_copy(&mut e, list, extra);
        // No construction; the copy and the setter still get the null.
        assert_eq!(
            copy_calls(&log),
            vec![
                (OPERATOR_NEW, vec![0x20]),
                (0x0043_a810, vec![0, 7]),
                (0x0041_9120, vec![list.addr(), 0]),
            ]
        );
    }

    #[test]
    fn copy_of_types_without_payload() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        for (extra_type, callee, expected) in [
            (0x16u8, 0x0041_aa20u32, vec![1u32, 0]),
            (0x17, 0x0041_aa20, vec![1, 1]),
            (0x3e, 0x0041_ab70, vec![1]),
            (0x80, 0x0042_f200, vec![1]),
        ] {
            stub(&mut e, callee);
            let extra = extra_of_type(&mut e, extra_type);
            let log = run_copy(&mut e, list, extra);
            let mut args = vec![list.addr()];
            args.extend(expected);
            assert_eq!(copy_calls(&log), vec![(callee, args)]);
        }
        // Type 0x55 has nothing to copy.
        let extra = extra_of_type(&mut e, 0x55);
        let log = run_copy(&mut e, list, extra);
        assert!(copy_calls(&log).is_empty());
        // Type 0x5a hands over the whole extra data.
        stub(&mut e, 0x0042_e2c0);
        let extra = extra_of_type(&mut e, 0x5a);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            copy_calls(&log),
            vec![(0x0042_e2c0, vec![list.addr(), extra.addr()])]
        );
    }

    #[test]
    fn copy_of_two_part_payloads() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        for callee in [
            0x0041_9ed0u32,
            0x0041_9f80,
            0x0041_d280,
            0x0041_d330,
            0x0041_da40,
            0x0041_dc70,
            0x0042_eb60,
        ] {
            stub(&mut e, callee);
        }
        let extra = extra_of_type(&mut e, 0x0d);
        e.mem.set_u32(extra.addr() + 0x0c, 11);
        e.mem.set_u32(extra.addr() + 0x10, 12);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            copy_calls(&log),
            vec![
                (0x0041_9ed0, vec![list.addr(), 11]),
                (0x0041_9f80, vec![list.addr(), 12])
            ]
        );
        for (extra_type, word_setter, byte_setter) in [
            (0x2fu8, 0x0041_d280u32, 0x0041_d330u32),
            (0x37, 0x0041_da40, 0x0041_dc70),
        ] {
            let extra = extra_of_type(&mut e, extra_type);
            e.mem.set_u32(extra.addr() + 0x0c, 21);
            e.mem.set_u32(extra.addr() + 0x10, 0x1ff);
            let log = run_copy(&mut e, list, extra);
            assert_eq!(
                copy_calls(&log),
                vec![
                    (word_setter, vec![list.addr(), 21]),
                    (byte_setter, vec![list.addr(), 0xff])
                ]
            );
        }
        let extra = extra_of_type(&mut e, 0x6e);
        e.mem.set_u32(extra.addr() + 0x0c, 31);
        e.mem.set_u32(extra.addr() + 0x10, 32);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            copy_calls(&log),
            vec![(0x0042_eb60, vec![list.addr(), 31, 32])]
        );
    }

    #[test]
    fn copy_of_canopy_shadow_mask_fills_the_structure_the_setter_made() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        let created = e.mem.alloc(8);
        e.register_double(0x0041_c490, move |e, a| {
            // The out parameter receives the structure.
            e.mem.set_u32(a[3], created);
            Ret::default()
        });
        let extra = extra_of_type(&mut e, 0x0a);
        e.mem.set_u32(extra.addr() + 0x0c, 0x61);
        e.mem.set_u32(extra.addr() + 0x10, 0x62);
        e.mem.set_u32(extra.addr() + 0x14, 0x63);
        e.mem.set_u32(extra.addr() + 0x18, 0x64);
        let log = run_copy(&mut e, list, extra);
        let calls = calls_to(&log, 0x0041_c490);
        assert_eq!(calls.len(), 1);
        // Handle read from +0x10's address.
        assert_eq!(&calls[0][..3], &[list.addr(), 0x61, 0x62]);
        assert_eq!(e.mem.u32(created), 0x63);
        assert_eq!(e.mem.u32(created + 4), 0x64);
    }

    #[test]
    fn copy_of_package_start_location_casts_twice() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        // The cast double answers with the target type it was given.
        e.register(RT_DYNAMIC_CAST, |_, a| returns(a[3]));
        stub(&mut e, 0x0041_ad00);
        let extra = extra_of_type(&mut e, 0x18);
        e.mem.set_u32(extra.addr() + 0x0c, 0x7000);
        e.mem.set_f32(extra.addr() + 0x1c, 1.5);
        let log = run_copy(&mut e, list, extra);
        let casts = calls_to(&log, RT_DYNAMIC_CAST);
        assert_eq!(casts[0], vec![0x7000, 0, 0x0118_3028, 0x0118_3fd0, 0]);
        assert_eq!(casts[1], vec![0x7000, 0, 0x0118_3028, 0x0118_3fb4, 0]);
        assert_eq!(
            calls_to(&log, 0x0041_ad00),
            vec![vec![
                list.addr(),
                0x0118_3fd0,
                0x0118_3fb4,
                extra.addr() + 0x10,
                1.5f32.to_bits()
            ]]
        );
    }

    #[test]
    fn copy_of_package_data_passes_three_words_and_three_bytes() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        stub(&mut e, 0x0041_c930);
        let extra = extra_of_type(&mut e, 0x19);
        for (offset, word) in [(0x0c, 1), (0x10, 2), (0x14, 3)] {
            e.mem.set_u32(extra.addr() + offset, word);
        }
        e.mem.write(extra.addr() + 0x18, &[4, 5, 6]);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            calls_to(&log, 0x0041_c930),
            vec![vec![list.addr(), 1, 2, 3, 4, 5, 6]]
        );
    }

    /// A `BSSimpleList` of `items`: nodes of (item, next), returns the head.
    fn simple_list(e: &mut Engine, items: &[u32]) -> u32 {
        let nodes: Vec<u32> = items.iter().map(|_| e.mem.alloc(8)).collect();
        for (index, node) in nodes.iter().enumerate() {
            e.mem.set_u32(*node, items[index]);
            e.mem
                .set_u32(*node + 4, nodes.get(index + 1).copied().unwrap_or(0));
        }
        nodes.first().copied().unwrap_or(0)
    }

    #[test]
    fn copy_of_list_types_walks_the_items() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        // 0x1d: items until the first null.
        stub(&mut e, 0x0042_2480);
        let extra = extra_of_type(&mut e, 0x1d);
        let head = simple_list(&mut e, &[0x11, 0x22, 0, 0x44]);
        e.mem.set_u32(extra.addr() + 0x0c, head);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            calls_to(&log, 0x0042_2480),
            vec![vec![list.addr(), 0x11], vec![list.addr(), 0x22]]
        );
        // 0x73: the same walk over the list at +0x0C.
        stub(&mut e, 0x0042_ef20);
        let extra = extra_of_type(&mut e, 0x73);
        let head = simple_list(&mut e, &[5, 6]);
        e.mem.set_u32(extra.addr() + 0x0c, head);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            calls_to(&log, 0x0042_ef20),
            vec![vec![list.addr(), 5], vec![list.addr(), 6]]
        );
        // An empty list: no calls.
        e.mem.set_u32(extra.addr() + 0x0c, 0);
        let log = run_copy(&mut e, list, extra);
        assert!(calls_to(&log, 0x0042_ef20).is_empty());
    }

    #[test]
    fn copy_of_pair_list_skips_null_entries_and_does_not_stop() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        stub(&mut e, 0x0041_d700);
        let pair_one = e.mem.alloc(8);
        e.mem.set_u32(pair_one, 100);
        e.mem.set_u8(pair_one + 4, 1);
        let pair_two = e.mem.alloc(8);
        e.mem.set_u32(pair_two, 200);
        e.mem.set_u8(pair_two + 4, 2);
        let extra = extra_of_type(&mut e, 0x1b);
        let head = simple_list(&mut e, &[pair_one, 0, pair_two]);
        e.mem.set_u32(extra.addr() + 0x0c, head);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            calls_to(&log, 0x0041_d700),
            vec![vec![list.addr(), 100, 1], vec![list.addr(), 200, 2]]
        );
    }

    #[test]
    fn copy_of_type_5e_passes_the_signed_byte() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        stub(&mut e, 0x0042_e760);
        e.register(0x0042_e800, |_, _| returns(0x5000));
        stub(&mut e, 0x0043_6f80);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 300);
        e.mem.set_u8(entry + 4, 0xfe);
        let extra = extra_of_type(&mut e, 0x5e);
        let head = simple_list(&mut e, &[entry, 0]);
        e.mem.set_u32(extra.addr() + 0x0c, head);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            calls_to(&log, 0x0043_6f80),
            vec![vec![0x5000, 300, (-2i32) as u32]]
        );
        assert_eq!(calls_to(&log, 0x0042_e760), vec![vec![list.addr()]]);
    }

    #[test]
    fn copy_of_room_data_walks_the_inline_list_then_sets_the_master_flag() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        stub(&mut e, 0x0042_0c10);
        stub(&mut e, 0x0042_0870);
        // The room data: a list head node inline at +8, the flag at +0x10.
        let room = e.mem.alloc(0x20);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x88);
        e.mem.set_u32(room + 8, 0x77);
        e.mem.set_u32(room + 12, second);
        e.mem.set_u8(room + 0x10, 1);
        let extra = extra_of_type(&mut e, 0x7b);
        e.mem.set_u32(extra.addr() + 0x0c, room);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            calls_to(&log, 0x0042_0c10),
            vec![vec![list.addr(), 0x77], vec![list.addr(), 0x88]]
        );
        assert_eq!(calls_to(&log, 0x0042_0870), vec![vec![list.addr(), 1]]);
    }

    #[test]
    fn copy_of_lock_builds_and_copies_the_structure() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        stub(&mut e, 0x0041_9050);
        let source = e.mem.alloc(0x14);
        for index in 0..5 {
            e.mem.set_u32(source + 4 * index, 0x10 + index);
        }
        let extra = extra_of_type(&mut e, 0x2a);
        e.mem.set_u32(extra.addr() + 0x0c, source);
        let log = run_copy(&mut e, list, extra);
        let set = calls_to(&log, 0x0041_9050);
        assert_eq!(set.len(), 1);
        let object = set[0][1];
        assert_eq!(set[0][0], list.addr());
        assert_eq!(calls_to(&log, MEMCPY), vec![vec![object, source, 0x14]]);
        assert_eq!(e.mem.bytes(object, 0x14), e.mem.bytes(source, 0x14));
    }

    #[test]
    fn copy_of_a_four_word_structure_and_a_two_word_table() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        stub(&mut e, 0x0041_fec0);
        let source = e.mem.alloc(16);
        for index in 0..4 {
            e.mem.set_u32(source + 4 * index, 0xa0 + index);
        }
        let extra = extra_of_type(&mut e, 0x76);
        e.mem.set_u32(extra.addr() + 0x0c, source);
        let log = run_copy(&mut e, list, extra);
        let set = calls_to(&log, 0x0041_fec0);
        assert_eq!(set.len(), 1);
        for index in 0..4 {
            assert_eq!(e.mem.u32(set[0][1] + 4 * index), 0xa0 + index);
        }
        // The structure was constructed (zeroed) by the unit's own function.
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);

        stub(&mut e, 0x0042_0a60);
        let table = e.mem.alloc(8);
        e.mem.set_u32(table, 41);
        e.mem.set_u32(table + 4, 42);
        let extra = extra_of_type(&mut e, 0x77);
        e.mem.set_u32(extra.addr() + 0x0c, table);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            calls_to(&log, 0x0042_0a60),
            vec![vec![list.addr(), 0, 41], vec![list.addr(), 1, 42]]
        );
    }

    #[test]
    fn copy_of_type_62_and_72_and_6b() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        // 0x62: a null source gives a null object and no copy.
        for callee in [0x0043_8f50u32, 0x0043_9120, 0x0042_1f30] {
            stub(&mut e, callee);
        }
        e.register(0x0043_8f50, |_, a| returns(a[0]));
        let extra = extra_of_type(&mut e, 0x62);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(calls_to(&log, 0x0042_1f30), vec![vec![list.addr(), 0]]);
        assert!(calls_to(&log, 0x0043_9120).is_empty());
        e.mem.set_u32(extra.addr() + 0x0c, 0x333);
        let log = run_copy(&mut e, list, extra);
        let copy = calls_to(&log, 0x0043_9120);
        assert_eq!(copy.len(), 1);
        assert_eq!(copy[0][1], 0x333);
        assert_eq!(
            calls_to(&log, 0x0042_1f30),
            vec![vec![list.addr(), copy[0][0]]]
        );

        // 0x72: a 4-byte object initialised to 0x16, then given the word the
        // payload handle reads.
        for callee in [0x0053_7e90u32, 0x0042_0fd0] {
            stub(&mut e, callee);
        }
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, 0x999);
        let extra = extra_of_type(&mut e, 0x72);
        e.mem.set_u32(extra.addr() + 0x0c, handle);
        let log = run_copy(&mut e, list, extra);
        let call = calls_to(&log, 0x0053_7e90);
        assert_eq!(call[0][1], 0x999);
        assert_eq!(e.mem.u32(call[0][0]), 0x16);
        assert_eq!(
            calls_to(&log, 0x0042_0fd0),
            vec![vec![list.addr(), call[0][0]]]
        );

        // 0x6b: cdecl conversion of the payload word, then the setter.
        e.register(0x004a_4d40, |_, a| returns(a[0] + 1));
        stub(&mut e, 0x0041_fa60);
        let extra = extra_of_type(&mut e, 0x6b);
        e.mem.set_u32(extra.addr() + 0x0c, 50);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(calls_to(&log, 0x0041_fa60), vec![vec![list.addr(), 51]]);
    }

    #[test]
    fn copy_of_portal_reads_the_payload_handle() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        stub(&mut e, 0x0042_0e00);
        let extra = extra_of_type(&mut e, 0x78);
        e.mem.set_u32(extra.addr() + 0x0c, 0x1357);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(calls_to(&log, READ_WORD), vec![vec![extra.addr() + 0x0c]]);
        assert_eq!(calls_to(&log, 0x0042_0e00), vec![vec![list.addr(), 0x1357]]);
    }

    #[test]
    fn copy_of_ref_list_types_adds_the_extra_once_and_copies_into_it() {
        for (extra_type, copy) in [
            (0x65u8, 0x0043_46f0u32),
            (0x66, 0x0043_4c80),
            (0x84, 0x0043_4a70),
            (0x85, 0x0043_4ee0),
        ] {
            let mut e = copy_engine();
            let list = copy_list(&mut e);
            stub(&mut e, copy);
            let source = extra_of_type(&mut e, extra_type);
            let log = run_copy(&mut e, list, source);
            // A new one, constructed by this unit, added to the list.
            assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
            let made = chain_types(&e, list.cast());
            assert_eq!(made, vec![extra_type]);
            let created: Ptr<BSExtraData> = e.get(list, ExtraDataList::pHead).cast();
            assert!(base_extra_list_has_extra(&mut e, list.cast(), extra_type));
            assert_eq!(
                calls_to(&log, copy),
                vec![vec![created.addr(), source.addr()]]
            );

            // The second copy finds it and allocates nothing.
            let log = run_copy(&mut e, list, source);
            assert!(calls_to(&log, OPERATOR_NEW).is_empty());
            assert_eq!(
                calls_to(&log, copy),
                vec![vec![created.addr(), source.addr()]]
            );
            assert_eq!(chain_types(&e, list.cast()), vec![extra_type]);
        }
    }

    #[test]
    fn copy_of_type_4c_creates_then_overwrites_nine_words() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        e.register(0x0043_5fa0, |e, a| {
            e.mem.set_u8(a[0] + 4, 0x4c);
            e.mem.set_u32(a[0] + 8, 0);
            returns(a[0])
        });
        let source = extra_of_type(&mut e, 0x4c);
        for index in 3..12u32 {
            e.mem.set_u32(source.addr() + 4 * index, 0x500 + index);
        }
        e.mem.set_u32(source.addr() + 0x30, 0xdead);
        let log = run_copy(&mut e, list, source);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x30]]);
        let created: Ptr<BSExtraData> = e.get(list, ExtraDataList::pHead).cast();
        assert_eq!(calls_to(&log, 0x0043_5fa0), vec![vec![created.addr(), 0]]);
        for index in 3..12u32 {
            assert_eq!(e.mem.u32(created.addr() + 4 * index), 0x500 + index);
        }
    }

    #[test]
    fn copy_of_type_53_and_57_use_their_own_copy_calls() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        for (extra_type, size, construct, copy) in [
            (0x53u8, 0x20u32, 0x0043_38b0u32, 0x0043_3b70u32),
            (0x57, 0x14, 0x0043_3ca0, 0x0043_42b0),
        ] {
            e.register_double(construct, move |e, a| {
                e.mem.set_u8(a[0] + 4, extra_type);
                e.mem.set_u32(a[0] + 8, 0);
                returns(a[0])
            });
            stub(&mut e, copy);
            let source = extra_of_type(&mut e, extra_type);
            let log = run_copy(&mut e, list, source);
            assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![size]]);
            let created = calls_to(&log, construct)[0][0];
            assert_eq!(calls_to(&log, copy), vec![vec![created, source.addr()]]);
        }
        assert_eq!(chain_types(&e, list.cast()), vec![0x53, 0x57]);
    }

    #[test]
    fn copy_of_type_92_copies_through_the_virtual_slot() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        e.put_vtable(VTABLE_EXTRA_TYPE_92, &[DESTRUCTOR, 0, 0, VIRTUAL_COPY]);
        let source = extra_of_type(&mut e, 0x92);
        let log = run_copy(&mut e, list, source);
        let created: Ptr<BSExtraData> = e.get(list, ExtraDataList::pHead).cast();
        assert_eq!(e.get(created, BSExtraData::cEtype), 0x92);
        assert_eq!(
            calls_to(&log, VIRTUAL_COPY),
            vec![vec![created.addr(), source.addr()]]
        );
        // Again: the existing one is used.
        let log = run_copy(&mut e, list, source);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(
            calls_to(&log, VIRTUAL_COPY),
            vec![vec![created.addr(), source.addr()]]
        );
    }

    #[test]
    fn copy_of_swim_breadcrumbs_copies_fields_and_walks_the_list() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        let target = e.mem.alloc(0x40);
        e.register_double(0x0042_f420, move |_, _| returns(target));
        stub(&mut e, 0x0047_0470);
        stub(&mut e, 0x0090_5820);
        let source = extra_of_type(&mut e, 0x8b);
        for index in 3..8u32 {
            e.mem.set_u32(source.addr() + 4 * index, 0x600 + index);
        }
        // The inline list head at +0x20: an item, then a null item, then one.
        e.mem.set_u32(source.addr() + 0x20, 0x31);
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(source.addr() + 0x24, second);
        e.mem.set_u32(second, 0);
        e.mem.set_u32(second + 4, third);
        e.mem.set_u32(third, 0x33);
        let log = run_copy(&mut e, list, source);
        for index in 3..8u32 {
            assert_eq!(e.mem.u32(target + 4 * index), 0x600 + index);
        }
        assert_eq!(calls_to(&log, 0x0047_0470), vec![vec![target + 0x20]]);
        assert_eq!(
            calls_to(&log, 0x0090_5820),
            vec![
                vec![target + 0x20, source.addr() + 0x20],
                vec![target + 0x20, third]
            ]
        );
    }

    #[test]
    fn copy_of_weapon_mod_flags_sets_each_active_slot() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        stub(&mut e, 0x0042_e380);
        let extra = extra_of_type(&mut e, 0x8d);
        e.mem.set_u8(extra.addr() + 0x0c, 0b101);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            calls_to(&log, 0x0042_e380),
            vec![vec![list.addr(), 1], vec![list.addr(), 4]]
        );
    }

    #[test]
    fn copy_of_an_unknown_type_logs_a_message() {
        let mut e = copy_engine();
        let list = copy_list(&mut e);
        stub(&mut e, LOG_MESSAGE);
        let extra = extra_of_type(&mut e, 0x04);
        let log = run_copy(&mut e, list, extra);
        assert_eq!(
            copy_calls(&log),
            vec![(LOG_MESSAGE, vec![NO_COPY_MESSAGE, 4])]
        );
    }

    // --- small structures and constructors --------------------------------

    #[test]
    fn lock_structure_is_zeroed() {
        let mut e = extra_engine();
        let block = Ptr::new(e.mem.alloc(0x14));
        e.mem.write(block.addr(), &[0xff; 0x14]);
        let result = e.call(0x0041_1b00, &args![block]).ptr::<()>();
        assert_eq!(result, block);
        assert_eq!(e.mem.u8(block.addr()), 0);
        assert_eq!(e.mem.u32(block.addr() + 4), 0);
        assert_eq!(e.mem.u8(block.addr() + 8), 0);
        assert_eq!(e.mem.u32(block.addr() + 0x0c), 0);
        assert_eq!(e.mem.u32(block.addr() + 0x10), 0);
        // Bytes between the fields are not touched.
        assert_eq!(e.mem.u8(block.addr() + 1), 0xff);
        assert_eq!(e.mem.u8(block.addr() + 9), 0xff);
    }

    #[test]
    fn ref_list_extra_constructors_set_type_vtable_and_an_empty_list() {
        let mut e = extra_engine();
        e.register(0x0096_a2d0, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            returns(a[0])
        });
        for (constructor, extra_type, vtable) in [
            (0x0041_1b40u32, 0x65u8, 0x0101_4428u32),
            (0x0041_1be0, 0x66, 0x0101_4434),
            (0x0041_1c80, 0x84, 0x0101_4440),
            (0x0041_1d20, 0x85, 0x0101_444c),
        ] {
            let extra: Ptr<ExtraRefList> = e.new_object();
            e.mem.write(extra.addr() + 0x0c, &[0xff; 8]);
            e.call_log = Some(vec![]);
            let result = e.call(constructor, &args![extra]).ptr::<()>();
            let log = e.call_log.take().unwrap();
            assert_eq!(result.addr(), extra.addr());
            assert_eq!(e.mem.u8(extra.addr() + 4), extra_type);
            assert_eq!(e.mem.u32(extra.addr()), vtable);
            assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0);
            assert_eq!(e.mem.u32(extra.addr() + 0x10), 0);
            assert_eq!(calls_to(&log, 0x0096_a2d0), vec![vec![extra.addr() + 0x0c]]);
        }
    }

    #[test]
    fn ref_list_extra_destructors_run_theirs_and_free_on_bit_zero() {
        let mut e = extra_engine();
        for (function, destructor) in [
            (0x0041_1bb0u32, 0x0043_4560u32),
            (0x0041_1c50, 0x0043_4af0),
            (0x0041_1cf0, 0x0043_4950),
            (0x0041_1d90, 0x0043_4dc0),
        ] {
            stub(&mut e, destructor);
            let extra: Ptr<ExtraRefList> = e.new_object();
            e.call_log = Some(vec![]);
            let result = e.call(function, &args![extra, 0u32]).ptr::<()>();
            let log = e.call_log.take().unwrap();
            assert_eq!(result.addr(), extra.addr());
            assert_eq!(calls_to(&log, destructor), vec![vec![extra.addr()]]);
            assert!(calls_to(&log, OPERATOR_DELETE).is_empty());

            e.call_log = Some(vec![]);
            e.call(function, &args![extra, 3u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![extra.addr()]]);
        }
    }

    #[test]
    fn four_word_structure_is_zeroed_and_the_value_structure_is_0x16() {
        let mut e = extra_engine();
        let block = Ptr::new(e.mem.alloc(0x14));
        e.mem.write(block.addr(), &[0xff; 0x14]);
        e.call(0x0041_1dc0, &args![block]);
        assert_eq!(e.mem.bytes(block.addr(), 16), vec![0u8; 16]);
        assert_eq!(e.mem.u32(block.addr() + 16), 0xffff_ffff);

        let result = e.call(0x0041_1e00, &args![block]).ptr::<()>();
        assert_eq!(result, block);
        assert_eq!(e.mem.u32(block.addr()), 0x16);
    }

    #[test]
    fn weapon_mod_flag_test_masks_the_byte_at_0c() {
        let mut e = extra_engine();
        let extra = extra_of_type(&mut e, 0x8d);
        e.mem.set_u32(extra.addr() + 0x0c, 0xffff_ff02);
        assert!(e.call(0x0041_1e20, &args![extra, 2u8]).bool());
        assert!(!e.call(0x0041_1e20, &args![extra, 1u8]).bool());
        assert!(e.call(0x0041_1e20, &args![extra, 3u8]).bool());
        assert!(!e.call(0x0041_1e20, &args![extra, 0xfdu8]).bool());
    }

    #[test]
    fn type_92_constructor_and_scalar_deleting_destructor() {
        let mut e = extra_engine();
        let extra = Ptr::new(e.mem.alloc(0x14));
        e.mem.write(extra.addr() + 0x0c, &[0xff; 4]);
        let result = e.call(0x0041_1e40, &args![extra]).ptr::<()>();
        assert_eq!(result, extra);
        assert_eq!(e.mem.u8(extra.addr() + 4), 0x92);
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_4458);
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0);

        stub(&mut e, 0x0041_1ea0);
        e.call_log = Some(vec![]);
        let result = e.call(0x0041_1e70, &args![extra, 0u32]).ptr::<()>();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, extra);
        assert_eq!(calls_to(&log, 0x0041_1ea0), vec![vec![extra.addr()]]);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());

        e.call_log = Some(vec![]);
        e.call(0x0041_1e70, &args![extra, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![extra.addr()]]);
    }

    // === second batch ======================================================

    use std::cell::RefCell;
    use std::rc::Rc;

    type Shared<T> = Rc<RefCell<T>>;

    /// The setters `AddExtraCopy` hands a copy to for the types the list
    /// tests use (script `0D`, worn `16`, owner `21`, health-like float `30`,
    /// count `24`, water `03`, music `07`), all stubs here.
    const SCRIPT_SETTER_1: u32 = 0x0041_9ed0;
    const SCRIPT_SETTER_2: u32 = 0x0041_9f80;
    const WORN_SETTER: u32 = 0x0041_aa20;
    const OWNER_SETTER: u32 = 0x0041_9700;
    const FLOAT_30_SETTER: u32 = 0x0041_9fb0;
    const COUNT_SETTER: u32 = 0x0041_9ad0;
    const WATER_SETTER: u32 = 0x0041_e160;
    const MUSIC_SETTER: u32 = 0x0041_bd10;
    const COPY_SETTERS: [u32; 8] = [
        SCRIPT_SETTER_1,
        SCRIPT_SETTER_2,
        WORN_SETTER,
        OWNER_SETTER,
        FLOAT_30_SETTER,
        COUNT_SETTER,
        WATER_SETTER,
        MUSIC_SETTER,
    ];

    /// An engine where `AddExtraCopy` runs for real and the setters it calls
    /// here are stubs.
    fn list_engine() -> Engine {
        let mut e = copy_engine();
        for setter in COPY_SETTERS {
            stub(&mut e, setter);
        }
        e
    }

    /// The copies a run made, as `(setter, arguments)` in order.
    fn copies_made(log: &Log) -> Vec<(u32, Vec<u32>)> {
        log.iter()
            .filter(|(callee, _)| COPY_SETTERS.contains(callee))
            .cloned()
            .collect()
    }

    fn words(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(address + 4 * i)).collect()
    }

    fn as_extra_list(list: Ptr<BaseExtraList>) -> Ptr<ExtraDataList> {
        list.cast()
    }

    /// Slot 1 of the test vtable is `Compare`: true when the words at +0x0C
    /// differ.
    const COMPARE: u32 = 0x0200_1008;

    fn compare_engine() -> Engine {
        let mut e = extra_engine();
        e.mem.set_u32(VTABLE + 4, COMPARE);
        e.register(COMPARE, |e, a| {
            returns((e.mem.u32(a[0] + 0x0c) != e.mem.u32(a[1] + 0x0c)) as u32)
        });
        e
    }

    /// A list of extra data of the given `(type, word at +0x0C)`.
    fn list_with_payloads(
        e: &mut Engine,
        entries: &[(u8, u32)],
    ) -> (Ptr<ExtraDataList>, Vec<Ptr<BSExtraData>>) {
        let types: Vec<u8> = entries.iter().map(|entry| entry.0).collect();
        let (list, extras) = list_with(e, &types);
        for (extra, entry) in extras.iter().zip(entries) {
            e.mem.set_u32(extra.addr() + 0x0c, entry.1);
        }
        (as_extra_list(list), extras)
    }

    #[test]
    fn type_92_destructor_sets_its_vtable_then_runs_the_base_destructor() {
        let mut e = extra_engine();
        stub(&mut e, BS_EXTRA_DATA_DESTROY);
        let extra: Ptr = Ptr::new(e.mem.alloc(0x14));
        e.mem.set_u32(extra.addr(), 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0041_1ea0, &args![extra]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_4458);
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_DESTROY),
            vec![vec![extra.addr()]]
        );
    }

    #[test]
    fn copy_list_empties_the_copyable_extra_data_then_copies_the_sources() {
        let mut e = list_engine();
        // 0x01 is not copyable and stays; 0x20 and 0x30 are removed.
        let (list, extras) = list_with(&mut e, &[0x01, 0x20, 0x30]);
        let (source, _) = list_with_payloads(&mut e, &[(0x03, 0x111), (0x07, 0x222)]);
        e.call_log = Some(vec![]);
        e.call(0x0041_1ec0, &args![list, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(chain_types(&e, list), vec![0x01]);
        assert_eq!(deleted(&log), vec![extras[1].addr(), extras[2].addr()]);
        assert_eq!(
            copies_made(&log),
            vec![
                (WATER_SETTER, vec![list.addr(), 0x111]),
                (MUSIC_SETTER, vec![list.addr(), 0x222])
            ]
        );
        let names: Vec<u32> = calls_to(&log, LOCK).iter().map(|a| a[1]).collect();
        assert_eq!(names[..2], [NAME_REMOVE_ALL_COPYABLE, NAME_COPY_LIST]);
        assert_locked_and_released(&log);

        // A null source only empties the list.
        e.call_log = Some(vec![]);
        e.call(0x0041_1ec0, &args![list, Ptr::<()>::NULL]);
        let log = e.call_log.take().unwrap();
        assert!(copies_made(&log).is_empty());
        assert_eq!(chain_types(&e, list), vec![0x01]);
    }
    #[test]
    fn copyability_is_false_exactly_for_the_jump_table_types() {
        let mut e = extra_engine();
        let list: Ptr<ExtraDataList> = e.new_object();
        let refused = [
            0x01, 0x02, 0x04, 0x0c, 0x0f, 0x10, 0x11, 0x15, 0x29, 0x34, 0x40, 0x4b, 0x52, 0x54,
            0x55, 0x58,
        ];
        for extra_type in 0..=255u8 {
            let extra = extra_of_type(&mut e, extra_type);
            let copyable = e.call(0x0041_1f20, &args![list, extra]).bool();
            assert_eq!(copyable, !refused.contains(&extra_type), "{extra_type:#x}");
        }
        assert!(e.call(0x0041_1f20, &args![list, Ptr::<()>::NULL]).bool());
    }

    #[test]
    fn remove_all_copyable_unlinks_the_copyable_ones_and_deletes_on_request() {
        let mut e = extra_engine();
        let (list, extras) = list_with(&mut e, &[0x01, 0x20, 0x30, 0x4b, 0x40, 0x31]);
        e.call_log = Some(vec![]);
        e.call(0x0041_1fd0, &args![list, true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(chain_types(&e, list), vec![0x01, 0x4b, 0x40]);
        assert_eq!(
            deleted(&log),
            vec![extras[1].addr(), extras[2].addr(), extras[5].addr()]
        );
        // The type bitmap is left alone; the caches and counters are reset.
        assert!(base_extra_list_has_extra(&mut e, list, 0x20));
        assert_eq!(e.global::<u32>(DIRTY), 1);
        assert_locked_and_released(&log);
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_REMOVE_ALL_COPYABLE);

        // Without `destroy`, a removed head is unlinked only.
        let (list, _) = list_with(&mut e, &[0x20, 0x01, 0x30]);
        e.call_log = Some(vec![]);
        e.call(0x0041_1fd0, &args![list, false]);
        let log = e.call_log.take().unwrap();
        assert_eq!(chain_types(&e, list), vec![0x01]);
        assert!(deleted(&log).is_empty());
    }

    #[test]
    fn remove_non_persistent_keeps_type_4_and_1_or_2_only_when_the_thread_flag_is_set() {
        let mut e = extra_engine();
        e.map(0x011d_d000, 0x1000);
        let tls = e.tls();
        e.mem.set_u32(tls + 0x294, 0);
        let (list, extras) = list_with(&mut e, &[0x20, 0x04, 0x01, 0x02, 0x00, 0x30]);
        e.call_log = Some(vec![]);
        e.call(0x0041_20b0, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(chain_types(&e, list), vec![0x04]);
        assert_eq!(
            deleted(&log),
            [0usize, 2, 3, 4, 5].map(|i| extras[i].addr()).to_vec()
        );
        assert_locked_and_released(&log);
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_REMOVE_NON_PERSISTENT);

        e.mem.set_u32(tls + 0x294, 4);
        let (list, _) = list_with(&mut e, &[0x01, 0x20, 0x02, 0x03]);
        e.call(0x0041_20b0, &args![list]);
        assert_eq!(chain_types(&e, list), vec![0x01, 0x02]);
    }

    #[test]
    fn thread_flag_is_bit_2_of_the_word_at_0x294() {
        let mut e = extra_engine();
        let tls = e.tls();
        for (word, expected) in [(4u32, true), (3, false), (0xfffb, false), (0xffff, true)] {
            e.mem.set_u32(tls + 0x294, word);
            assert_eq!(e.call(0x0041_21b0, &args![0u32]).bool(), expected);
        }
    }

    /// A source list for the container tests: script `0D` (words `0xa1` and
    /// `0xa2`), worn `16`, float `30` (1.0), `20` (not a container type),
    /// owner `21` (`0xb1`).
    fn container_source(e: &mut Engine) -> (Ptr<ExtraDataList>, Vec<Ptr<BSExtraData>>) {
        let (list, extras) = list_with(e, &[0x0d, 0x16, 0x30, 0x20, 0x21]);
        e.mem.set_u32(extras[0].addr() + 0x0c, 0xa1);
        e.mem.set_u32(extras[0].addr() + 0x10, 0xa2);
        e.mem.set_u32(extras[2].addr() + 0x0c, 1.0f32.to_bits());
        e.mem.set_u32(extras[4].addr() + 0x0c, 0xb1);
        (as_extra_list(list), extras)
    }

    /// The copies of each element of [`container_source`] for target `t`.
    fn copy_of_script(t: Ptr<ExtraDataList>) -> Vec<(u32, Vec<u32>)> {
        vec![
            (SCRIPT_SETTER_1, vec![t.addr(), 0xa1]),
            (SCRIPT_SETTER_2, vec![t.addr(), 0xa2]),
        ]
    }
    fn copy_of_worn(t: Ptr<ExtraDataList>) -> Vec<(u32, Vec<u32>)> {
        vec![(WORN_SETTER, vec![t.addr(), 1, 0])]
    }
    fn copy_of_float(t: Ptr<ExtraDataList>) -> Vec<(u32, Vec<u32>)> {
        vec![(FLOAT_30_SETTER, vec![t.addr(), 1.0f32.to_bits()])]
    }
    fn copy_of_owner(t: Ptr<ExtraDataList>) -> Vec<(u32, Vec<u32>)> {
        vec![(OWNER_SETTER, vec![t.addr(), 0xb1])]
    }

    #[test]
    fn copy_for_container_copies_the_container_types_and_can_move_them() {
        let mut e = list_engine();
        let (source, extras) = container_source(&mut e);
        let target: Ptr<ExtraDataList> = e.new_object();

        // Copy only: nothing leaves the source.
        e.call_log = Some(vec![]);
        e.call(0x0041_21e0, &args![target, source, true]);
        let log = e.call_log.take().unwrap();
        let expected = [
            copy_of_script(target),
            copy_of_worn(target),
            copy_of_float(target),
            copy_of_owner(target),
        ]
        .concat();
        assert_eq!(copies_made(&log), expected);
        assert_eq!(
            chain_types(&e, source.cast()),
            vec![0x0d, 0x16, 0x30, 0x20, 0x21]
        );
        assert!(deleted(&log).is_empty());

        // Move: 0D is unlinked, the others deleted except 30, and the walk
        // restarts from the head after each removal (so 30 is copied twice).
        e.call_log = Some(vec![]);
        e.call(0x0041_21e0, &args![target, source, false]);
        let log = e.call_log.take().unwrap();
        let expected = [
            copy_of_script(target),
            copy_of_worn(target),
            copy_of_float(target),
            copy_of_owner(target),
            copy_of_float(target),
        ]
        .concat();
        assert_eq!(copies_made(&log), expected);
        assert_eq!(chain_types(&e, source.cast()), vec![0x30, 0x20]);
        assert_eq!(deleted(&log), vec![extras[1].addr(), extras[4].addr()]);
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_COPY_LIST_FOR_CONTAINER);
        assert_locked_and_released(&log);
    }

    #[test]
    fn duplicate_for_container_copies_without_touching_the_source() {
        let mut e = list_engine();
        let (source, _) = list_with(&mut e, &[0x0d, 0x24, 0x20, 0x4b, 0x16]);
        let target: Ptr<ExtraDataList> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0041_2380, &args![target, source]);
        let log = e.call_log.take().unwrap();
        // Type 24 (count) is copied here but not by `CopyListForContainer`.
        let count_copy = (COUNT_SETTER, vec![target.addr(), 0]);
        let expected = [
            vec![
                (SCRIPT_SETTER_1, vec![target.addr(), 0]),
                (SCRIPT_SETTER_2, vec![target.addr(), 0]),
            ],
            vec![count_copy],
            copy_of_worn(target),
        ]
        .concat();
        assert_eq!(copies_made(&log), expected);
        assert_eq!(
            chain_types(&e, source.cast()),
            vec![0x0d, 0x24, 0x20, 0x4b, 0x16]
        );
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_DUPLICATE_FOR_CONTAINER);
        assert_locked_and_released(&log);
    }

    #[test]
    fn copy_for_reference_moves_the_kept_types_out_of_the_source() {
        let mut e = list_engine();
        let (source, extras) = container_source(&mut e);
        let target: Ptr<ExtraDataList> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0041_2490, &args![target, source, true]);
        let log = e.call_log.take().unwrap();
        let expected = [
            copy_of_script(target),
            copy_of_worn(target),
            copy_of_float(target),
            copy_of_owner(target),
        ]
        .concat();
        assert_eq!(copies_made(&log), expected);
        assert_eq!(chain_types(&e, source.cast()), vec![0x20]);
        // 0D is only unlinked; the others are deleted (here the walk
        // restarts from the head after each one).
        assert_eq!(
            deleted(&log),
            vec![extras[1].addr(), extras[2].addr(), extras[4].addr()]
        );
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_COPY_LIST_FOR_REFERENCE);
        assert_locked_and_released(&log);
    }

    #[test]
    fn copy_for_reference_without_moving_keeps_the_source_and_makes_a_new_script() {
        let mut e = list_engine();
        e.register(GET_SCRIPT, |_, _| returns(0x5c01));
        e.register(EXTRA_SCRIPT_INIT, |e, a| {
            e.mem.set_u8(a[0] + 4, 0x0d);
            e.mem.set_u32(a[0] + 0x0c, a[1]);
            returns(a[0])
        });
        e.register(SCRIPT_GET_RESULT, |_, a| returns(a[0] + 0x100));
        stub(&mut e, EXTRA_DATA_LIST_SET_SCRIPT_RESULT);
        let types = [0x16, 0x0d, 0x20];
        let (source, _) = list_with(&mut e, &types);
        let target: Ptr<ExtraDataList> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0041_2490, &args![target, source, false]);
        let log = e.call_log.take().unwrap();
        // Only the 16 is copied (the final call is the script result setter,
        // `00419f80`, which `AddExtraCopy` also uses for type 0D); the source
        // keeps all three.
        let mut expected = copy_of_worn(target);
        expected.push((SCRIPT_SETTER_2, vec![target.addr(), 0x5c01 + 0x100]));
        assert_eq!(copies_made(&log), expected);
        assert_eq!(chain_types(&e, source.cast()), types.to_vec());
        // The new script extra data (0x14 bytes) is in the target list.
        assert_eq!(chain_types(&e, target.cast()), vec![0x0d]);
        let created: Ptr<BSExtraData> = e.get(target, ExtraDataList::pHead).cast();
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(
            calls_to(&log, EXTRA_SCRIPT_INIT),
            vec![vec![created.addr(), 0x5c01]]
        );
        assert_eq!(calls_to(&log, GET_SCRIPT).len(), 2);
        assert_eq!(
            calls_to(&log, EXTRA_DATA_LIST_SET_SCRIPT_RESULT),
            vec![vec![target.addr(), 0x5c01 + 0x100]]
        );
    }
    #[test]
    fn compare_for_container_walks_the_other_list() {
        let mut e = compare_engine();
        let (this, _) = list_with_payloads(&mut e, &[(0x20, 5), (0x30, 7), (0x16, 1)]);
        let compare = |e: &mut Engine, entries: &[(u8, u32)], script: bool, skip16: bool| {
            let (other, _) = list_with_payloads(e, entries);
            e.call(0x0041_26c0, &args![this, other, script, skip16])
                .bool()
        };
        // Equal on the common types.
        assert!(!compare(&mut e, &[(0x20, 5), (0x30, 7)], false, false));
        // A different value, a type this list lacks, and ignored types 24, 4A.
        assert!(compare(&mut e, &[(0x20, 5), (0x30, 8)], false, false));
        assert!(compare(&mut e, &[(0x20, 5), (0x40, 7)], false, false));
        assert!(!compare(
            &mut e,
            &[(0x24, 9), (0x4a, 9), (0x20, 5)],
            false,
            false
        ));
        // Script (0D) and owner (21): a difference at once unless allowed.
        assert!(compare(&mut e, &[(0x0d, 1)], false, false));
        assert!(compare(&mut e, &[(0x21, 1)], false, false));
        assert!(compare(&mut e, &[(0x0d, 1)], true, false));
        // Type 16 can be skipped.
        assert!(compare(&mut e, &[(0x16, 2)], false, false));
        assert!(!compare(&mut e, &[(0x16, 2)], false, true));
        // Allowed owners are compared like the others.
        let (this_with_owner, _) = list_with_payloads(&mut e, &[(0x21, 3)]);
        let (other, _) = list_with_payloads(&mut e, &[(0x21, 3)]);
        assert!(!e
            .call(0x0041_26c0, &args![this_with_owner, other, true, false])
            .bool());
        // An empty list never differs; the lock is taken and released.
        e.call_log = Some(vec![]);
        let (empty, _) = list_with_payloads(&mut e, &[]);
        assert!(!e
            .call(0x0041_26c0, &args![this, empty, false, false])
            .bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_COMPARE_LIST_FOR_CONTAINER);
        assert_locked_and_released(&log);
    }

    #[test]
    fn compare_list_compares_the_copyable_extra_data_both_ways() {
        let mut e = compare_engine();
        let (this, _) = list_with_payloads(&mut e, &[(0x20, 1), (0x30, 2)]);
        let run = |e: &mut Engine, entries: &[(u8, u32)]| {
            let (other, _) = list_with_payloads(e, entries);
            e.call(0x0041_27e0, &args![this, other]).bool()
        };
        assert!(!run(&mut e, &[(0x20, 1), (0x30, 2)]));
        // Not copyable extra data (01) only counts toward the item count.
        assert!(!run(&mut e, &[(0x20, 1), (0x30, 2), (0x01, 9)]));
        // A different payload, a missing type, an extra type.
        assert!(run(&mut e, &[(0x20, 1), (0x30, 3)]));
        assert!(run(&mut e, &[(0x20, 1)]));
        assert!(run(&mut e, &[(0x20, 1), (0x30, 2), (0x41, 4)]));
        // Equal counts, but a type missing on this side.
        assert!(run(&mut e, &[(0x20, 1), (0x41, 2)]));
        // Count mismatch with a copyable type of this list absent in the other.
        let (this_one, _) = list_with_payloads(&mut e, &[(0x20, 1)]);
        let (other, _) = list_with_payloads(&mut e, &[(0x01, 1), (0x02, 1)]);
        assert!(e.call(0x0041_27e0, &args![this_one, other]).bool());
        e.call_log = Some(vec![]);
        let (other, _) = list_with_payloads(&mut e, &[(0x20, 1)]);
        assert!(!e.call(0x0041_27e0, &args![this_one, other]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOCK)[0][1], NAME_COMPARE_LIST);
        assert_locked_and_released(&log);
    }

    // --- small structures --------------------------------------------------

    /// An engine where `Swap32` and `Swap16` byte-swap in place.
    fn swap_engine() -> Engine {
        let mut e = extra_engine();
        e.register(SWAP_DWORD, |e, a| {
            let value = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], value.swap_bytes());
            Ret::default()
        });
        e.register(SWAP_WORD, |e, a| {
            let value = e.mem.u16(a[0]);
            e.mem.set_u16(a[0], value.swap_bytes());
            Ret::default()
        });
        e
    }

    /// Word `index` of a numbered block: all different, none a byte palindrome.
    fn pattern(index: u32) -> u32 {
        0x1000_0000 + (index + 1) * 0x0001_0203
    }

    /// A block of `count` words `pattern(0)`, `pattern(1)`, ... and 16 spare bytes.
    fn numbered_block(e: &mut Engine, count: u32) -> Ptr {
        let block: Ptr = Ptr::new(e.mem.alloc(count * 4 + 16));
        for index in 0..count {
            e.mem.set_u32(block.addr() + index * 4, pattern(index));
        }
        block
    }

    #[test]
    fn address_of_the_member_at_8() {
        let mut e = extra_engine();
        assert_eq!(e.call(0x0041_3f40, &args![0x1000u32]).u32(), 0x1008);
    }

    #[test]
    fn matrix_vector_and_colour_getters_copy_their_words() {
        let mut e = extra_engine();
        let object = numbered_block(&mut e, 16);
        let out: Ptr = Ptr::new(e.mem.alloc(0x40));
        // Nine words at +0x1c.
        let result = e.call(0x0041_3f60, &args![object, out]).ptr::<()>();
        assert_eq!(result, out);
        let expected: Vec<u32> = (7..16).map(|n| pattern(n as u32)).collect();
        assert_eq!(words(&e, out.addr(), 9), expected);
        assert_eq!(e.mem.u32(out.addr() + 36), 0);
        // Four words at +8.
        let out: Ptr = Ptr::new(e.mem.alloc(0x40));
        let result = e.call(0x0041_3f90, &args![object, out]).ptr::<()>();
        assert_eq!(result, out);
        let expected: Vec<u32> = (2..6).map(|n| pattern(n as u32)).collect();
        assert_eq!(words(&e, out.addr(), 4), expected);
        assert_eq!(e.mem.u32(out.addr() + 16), 0);
        // Three words at +0x18.
        let out: Ptr = Ptr::new(e.mem.alloc(0x40));
        let result = e.call(0x0041_3fc0, &args![object, out]).ptr::<()>();
        assert_eq!(result, out);
        let expected: Vec<u32> = (6..9).map(|n| pattern(n as u32)).collect();
        assert_eq!(words(&e, out.addr(), 3), expected);
        assert_eq!(e.mem.u32(out.addr() + 12), 0);
    }

    #[test]
    fn float_copy_goes_through_the_x87_stack() {
        let mut e = extra_engine();
        let source: Ptr = Ptr::new(e.mem.alloc(8));
        let target: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_f32(source.addr(), 1.5);
        let result = e.call(0x0041_3ff0, &args![target, source]).ptr::<()>();
        assert_eq!(result, target);
        assert_eq!(e.mem.f32(target.addr()), 1.5);
        // A signalling NaN comes out quiet; a quiet NaN and infinity do not change.
        for (bits, expected) in [
            (0x7f80_0001u32, 0x7fc0_0001u32),
            (0xff80_0001, 0xffc0_0001),
            (0x7fc0_0001, 0x7fc0_0001),
            (0x7f80_0000, 0x7f80_0000),
            (0x0000_0001, 0x0000_0001),
        ] {
            e.mem.set_u32(source.addr(), bits);
            e.call(0x0041_3ff0, &args![target, source]);
            assert_eq!(e.mem.u32(target.addr()), expected, "{bits:#x}");
        }
    }

    #[test]
    fn empty_node_and_zero_pairs() {
        let mut e = extra_engine();
        let node: Ptr = Ptr::new(e.mem.alloc(16));
        e.mem.write(node.addr(), &[0xff; 16]);
        let result = e.call(0x0041_4010, &args![node]).ptr::<()>();
        assert_eq!(result, node);
        assert_eq!(words(&e, node.addr(), 3), vec![0, 0, 0xffff_ffff]);
        e.mem.write(node.addr(), &[0xff; 16]);
        let result = e.call(0x0041_43c0, &args![node]).ptr::<()>();
        assert_eq!(result, node);
        assert_eq!(words(&e, node.addr(), 3), vec![0, 0, 0xffff_ffff]);
    }

    #[test]
    fn byte_swaps_cover_the_words_of_their_structure() {
        // (function, words swapped)
        for (function, count) in [
            (0x0041_4030u32, 7u32),
            (0x0041_40f0, 9),
            (0x0041_4220, 4),
            (0x0041_4290, 8),
            (0x0041_4370, 4),
        ] {
            let mut e = swap_engine();
            let block = numbered_block(&mut e, 12);
            e.call_log = Some(vec![]);
            e.call(function, &args![block]);
            let log = e.call_log.take().unwrap();
            let swapped: Vec<u32> = calls_to(&log, SWAP_DWORD).iter().map(|a| a[0]).collect();
            let expected: Vec<u32> = (0..count).map(|i| block.addr() + 4 * i).collect();
            assert_eq!(swapped, expected, "{function:#x}");
            assert!(calls_to(&log, SWAP_DWORD).iter().all(|a| a[1] == 0));
            for index in 0..12 {
                let original = pattern(index);
                let expected = if index < count {
                    original.swap_bytes()
                } else {
                    original
                };
                assert_eq!(e.mem.u32(block.addr() + 4 * index), expected);
            }
        }
    }

    #[test]
    fn byte_swap_of_a_word_and_a_short() {
        let mut e = swap_engine();
        let block: Ptr = Ptr::new(e.mem.alloc(16));
        e.mem.set_u32(block.addr(), 0x0102_0304);
        e.mem.set_u32(block.addr() + 4, 0xaabb_ccdd);
        e.call(0x0041_41e0, &args![block]);
        assert_eq!(e.mem.u32(block.addr()), 0x0403_0201);
        // Only the low 16 bits at +4 are swapped.
        assert_eq!(e.mem.u32(block.addr() + 4), 0xaabb_ddcc);
    }

    #[test]
    fn primitive_record_and_colour_constructors() {
        let mut e = extra_engine();
        let record: Ptr = Ptr::new(e.mem.alloc(0x24));
        e.mem.write(record.addr(), &[0xff; 0x24]);
        e.call_log = Some(vec![]);
        let result = e.call(0x0041_43f0, &args![record]).ptr::<()>();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, record);
        // The colour at +0x0c is four zeros; the rest is untouched.
        assert_eq!(words(&e, record.addr(), 9)[3..7], [0, 0, 0, 0]);
        assert_eq!(e.mem.u32(record.addr()), 0xffff_ffff);
        assert_eq!(e.mem.u32(record.addr() + 0x1c), 0xffff_ffff);
        assert_eq!(calls_to(&log, SIMPLE_LIST_ITEM), vec![vec![record.addr()]]);

        // Four floats through the x87 stack.
        let target: Ptr = Ptr::new(e.mem.alloc(16));
        let result = e
            .call(
                0x0041_4430,
                &args![target, 1.0f32, 2.0f32, f32::from_bits(0x7f80_0001), -4.5f32],
            )
            .ptr::<()>();
        assert_eq!(result, target);
        assert_eq!(
            words(&e, target.addr(), 4),
            vec![
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                0x7fc0_0001,
                (-4.5f32).to_bits()
            ]
        );
    }

    #[test]
    fn occlusion_plane_constructor_runs_three_member_constructors() {
        let mut e = extra_engine();
        let record: Ptr = Ptr::new(e.mem.alloc(0x30));
        e.call_log = Some(vec![]);
        let result = e.call(0x0041_4470, &args![record]).ptr::<()>();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, record);
        assert_eq!(
            calls_to(&log, SIMPLE_LIST_ITEM),
            vec![
                vec![record.addr()],
                vec![record.addr() + 8],
                vec![record.addr() + 0x14]
            ]
        );
    }

    #[test]
    fn point_constructor_and_sincos() {
        let mut e = extra_engine();
        let point: Ptr = Ptr::new(e.mem.alloc(16));
        let result = e
            .call(
                0x0041_6870,
                &args![point, 1.0f32, -2.5f32, f32::from_bits(0xff80_0001)],
            )
            .ptr::<()>();
        assert_eq!(result, point);
        assert_eq!(
            words(&e, point.addr(), 3),
            vec![1.0f32.to_bits(), (-2.5f32).to_bits(), 0xffc0_0001]
        );
        assert_eq!(e.mem.u32(point.addr() + 12), 0);

        let sine: Ptr = Ptr::new(e.mem.alloc(8));
        let cosine: Ptr = Ptr::new(e.mem.alloc(8));
        e.call(0x0041_69a0, &args![0.0f32, sine, cosine]);
        assert_eq!(e.mem.f32(sine.addr()), 0.0);
        assert_eq!(e.mem.f32(cosine.addr()), 1.0);
        let angle = std::f32::consts::FRAC_PI_2;
        e.call(0x0041_69a0, &args![angle, sine, cosine]);
        assert_eq!(e.mem.f32(sine.addr()), 1.0);
        assert_eq!(e.mem.f32(cosine.addr()), (angle as f64).cos() as f32);
        e.call(0x0041_69a0, &args![-1.0f32, sine, cosine]);
        assert_eq!(e.mem.f32(sine.addr()), ((-1.0f64).sin()) as f32);
        assert_eq!(e.mem.f32(cosine.addr()), ((1.0f64).cos()) as f32);
    }

    #[test]
    fn sincos_out_of_range_stores_the_angle_and_the_default_nan() {
        // `FSINCOS` leaves an operand with |x| >= 2^63 alone and pushes nothing: the cosine
        // slot gets the angle, the sine slot the default NaN of the empty-register pop.
        let mut e = extra_engine();
        let sine: Ptr = Ptr::new(e.mem.alloc(8));
        let cosine: Ptr = Ptr::new(e.mem.alloc(8));
        for angle in [1e30f32, -1e30, f32::MAX] {
            e.call(0x0041_69a0, &args![angle, sine, cosine]);
            assert_eq!(e.mem.u32(sine.addr()), 0xffc0_0000);
            assert_eq!(e.mem.u32(cosine.addr()), angle.to_bits());
        }
        // A NaN angle comes back quiet in both.
        e.call(
            0x0041_69a0,
            &args![f32::from_bits(0x7f80_0001), sine, cosine],
        );
        assert_eq!(e.mem.u32(sine.addr()), 0x7fc0_0001);
        assert_eq!(e.mem.u32(cosine.addr()), 0x7fc0_0001);
    }

    #[test]
    fn region_list_getter_reads_the_word_at_0x1d8() {
        let mut e = extra_engine();
        let object: Ptr = Ptr::new(e.mem.alloc(0x200));
        e.mem.set_u32(object.addr() + 0x1d8, 0xcafe);
        assert_eq!(e.call(0x0041_69d0, &args![object]).u32(), 0xcafe);
    }

    #[test]
    fn occlusion_plane_setters_copy_and_mark_the_plane_dirty() {
        let mut e = extra_engine();
        let source = numbered_block(&mut e, 9);
        for (function, offset, count) in [
            (0x0041_69f0u32, 8u32, 3u32),
            (0x0041_6a30, 0x14, 2),
            (0x0041_6a70, 0x1c, 9),
        ] {
            let plane: Ptr = Ptr::new(e.mem.alloc(0x100));
            e.call(function, &args![plane, source]);
            let expected: Vec<u32> = (0..count).map(pattern).collect();
            assert_eq!(words(&e, plane.addr() + offset, count), expected);
            assert_eq!(e.mem.u32(plane.addr() + offset + count * 4), 0);
            assert_eq!(e.mem.u8(plane.addr() + 0xe4), 1);
            assert_eq!(e.mem.u8(plane.addr() + 0xe5), 1);
            assert_eq!(e.mem.u8(plane.addr() + 0xe6), 0);
        }
    }

    #[test]
    fn short_record_constructor_clears_a_word_and_a_short() {
        let mut e = extra_engine();
        let record: Ptr = Ptr::new(e.mem.alloc(16));
        e.mem.write(record.addr(), &[0xff; 16]);
        let result = e.call(0x0041_6ab0, &args![record]).ptr::<()>();
        assert_eq!(result, record);
        assert_eq!(e.mem.u32(record.addr()), 0);
        assert_eq!(e.mem.u16(record.addr() + 4), 0);
        assert_eq!(e.mem.u16(record.addr() + 6), 0xffff);
    }

    #[test]
    fn type_5a_constructor_copies_the_value_passed_by_value() {
        let mut e = extra_engine();
        e.register(NAVMESH_PORTAL_COPY, |e, a| {
            let first = e.mem.u32(a[1]);
            let second = e.mem.u16(a[1] + 4);
            e.mem.set_u32(a[0], first);
            e.mem.set_u16(a[0] + 4, second);
            returns(a[0])
        });
        let extra: Ptr = Ptr::new(e.mem.alloc(0x14));
        e.call_log = Some(vec![]);
        let result = e
            .call(0x0041_6ad0, &args![extra, 0x1234_5678u32, 0xdead_beefu32])
            .ptr::<()>();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, extra);
        assert_eq!(e.mem.u8(extra.addr() + 4), 0x5a);
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_4618);
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0x1234_5678);
        // Only the low 16 bits of the second word are copied.
        assert_eq!(e.mem.u16(extra.addr() + 0x10), 0xbeef);
        assert_eq!(e.mem.u16(extra.addr() + 0x12), 0);
        let copy = calls_to(&log, NAVMESH_PORTAL_COPY);
        assert_eq!(copy.len(), 1);
        assert_eq!(copy[0][0], extra.addr() + 0x0c);
    }

    #[test]
    fn radio_data_and_room_data_constructors() {
        let mut e = extra_engine();
        let radio: Ptr = Ptr::new(e.mem.alloc(0x14));
        e.mem.write(radio.addr(), &[0xff; 0x14]);
        let result = e.call(0x0041_6b40, &args![radio]).ptr::<()>();
        assert_eq!(result, radio);
        assert_eq!(words(&e, radio.addr(), 5), vec![0, 1, 0, 0, 0xffff_ffff]);

        e.register(SIMPLE_LIST_INIT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            returns(a[0])
        });
        let room: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.call_log = Some(vec![]);
        let result = e.call(0x0041_6b80, &args![room]).ptr::<()>();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, room);
        assert_eq!(
            calls_to(&log, SIMPLE_LIST_INIT),
            vec![vec![room.addr()], vec![room.addr() + 8]]
        );
    }

    // --- the save writer ---------------------------------------------------

    /// `(chunk name, data)` in the order the writer added them.
    type Chunks = Shared<Vec<(String, Vec<u8>)>>;

    const BIG_ENDIAN_FLAG: u32 = 0x011c_54ba;

    fn chunk_name(tag: u32) -> String {
        String::from_utf8_lossy(&tag.to_le_bytes()).into_owned()
    }

    fn word_bytes(words: &[u32]) -> Vec<u8> {
        words.iter().flat_map(|w| w.to_le_bytes()).collect()
    }

    fn chunk(name: &str, data: Vec<u8>) -> (String, Vec<u8>) {
        (name.to_string(), data)
    }

    /// An engine with working doubles for what the save writer calls: the
    /// form id getter, the chunk adders (recording what was added), the
    /// byte swaps, the big-endian flag (the byte at `011c54ba`) and the list
    /// helpers.
    fn save_engine() -> (Engine, Chunks) {
        let mut e = swap_engine();
        e.map(0x011c_5000, 0x1000);
        let chunks: Chunks = Rc::default();
        e.register(IS_BIG_ENDIAN, |e, _| {
            returns(e.mem.u8(BIG_ENDIAN_FLAG) as u32)
        });
        e.register(FORM_ID, |e, a| returns(e.mem.u32(a[0] + 0x0c)));
        let log = chunks.clone();
        e.register_double(ADD_CHUNK_WORD, move |_, a| {
            log.borrow_mut()
                .push(chunk(&chunk_name(a[0]), word_bytes(&[a[1]])));
            Ret::default()
        });
        let log = chunks.clone();
        e.register_double(ADD_CHUNK_BYTE, move |_, a| {
            log.borrow_mut()
                .push(chunk(&chunk_name(a[0]), vec![a[1] as u8]));
            Ret::default()
        });
        let log = chunks.clone();
        e.register_double(ADD_CHUNK_EMPTY, move |_, a| {
            log.borrow_mut().push(chunk(&chunk_name(a[0]), vec![]));
            Ret::default()
        });
        let log = chunks.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            let data = e.mem.bytes(a[1], a[2]);
            log.borrow_mut().push(chunk(&chunk_name(a[0]), data));
            Ret::default()
        });
        let log = chunks.clone();
        e.register_double(ADD_CHUNK_ARRAY, move |e, a| {
            let data = e.mem.bytes(a[1], a[2] * 4);
            log.borrow_mut().push(chunk(&chunk_name(a[0]), data));
            Ret::default()
        });
        let log = chunks.clone();
        e.register_double(ADD_CHUNK_ARRAY_RAW, move |e, a| {
            let data = e.mem.bytes(a[1], a[2]);
            log.borrow_mut().push(chunk(&chunk_name(a[0]), data));
            Ret::default()
        });
        e.register(SWAP_WORD_AT, |e, a| {
            let value = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], value.swap_bytes());
            Ret::default()
        });
        e.register(SWAP_WORD_AT_4, |e, a| {
            let value = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, value.swap_bytes());
            Ret::default()
        });
        e.register(SWAP_TWO_WORDS, |e, a| {
            for offset in [0, 4] {
                let value = e.mem.u32(a[0] + offset);
                e.mem.set_u32(a[0] + offset, value.swap_bytes());
            }
            Ret::default()
        });
        e.register(LIST_COUNT, |e, a| {
            let mut count = 0;
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    count += 1;
                }
                node = e.mem.u32(node + 4);
            }
            returns(count)
        });
        e.register(LIST_IS_EMPTY, |e, a| {
            returns((e.mem.u32(a[0] + 4) == 0 && e.mem.u32(a[0]) == 0) as u32)
        });
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            returns(value as i32 as u32)
        });
        e.register(SIMPLE_LIST_INIT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            returns(a[0])
        });
        e.register(NAVMESH_PORTAL_COPY, |e, a| {
            let first = e.mem.u32(a[1]);
            let second = e.mem.u16(a[1] + 4);
            e.mem.set_u32(a[0], first);
            e.mem.set_u16(a[0] + 4, second);
            returns(a[0])
        });
        e.register(EXTRA_DATA_LIST_GET_SEED, |_, _| returns(0x7b));
        e.register(STRING_LENGTH, |e, a| returns(e.mem.u16(a[0] + 4) as u32));
        e.register(PLANE_HALF_EXTENTS, |_, a| returns(a[0] + 0x14));
        for address in [
            RAGDOLL_DATA_SAVE,
            TELEPORT_DATA_SAVE,
            MAP_MARKER_DATA_SAVE,
            MULTIBOUND_MARKER_DATA_SAVE,
            AUDIO_MARKER_DATA_SAVE,
            AUDIO_BUOY_MARKER_DATA_SAVE,
            PACKAGE_EVENT_ACTION_SAVE,
            PATH_LOCATION_INIT,
            DECAL_DATA_INIT,
            STRING_DESTROY,
        ] {
            stub(&mut e, address);
        }
        (e, chunks)
    }

    /// A form (`TESForm`-like object) whose id is `id`.
    fn form_with_id(e: &mut Engine, id: u32) -> u32 {
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form + 0x0c, id);
        form
    }

    /// An extra data of `extra_type` with the given `(offset, word)` pairs.
    fn extra_with(e: &mut Engine, extra_type: u8, words: &[(u32, u32)]) -> Ptr<BSExtraData> {
        let extra = extra_of_type(e, extra_type);
        for &(offset, word) in words {
            e.mem.set_u32(extra.addr() + offset, word);
        }
        extra
    }

    /// Links the extra data into a list and runs the save writer on it.
    fn save_extras(e: &mut Engine, extras: &[Ptr<BSExtraData>]) {
        let list: Ptr<ExtraDataList> = e.new_object();
        for (index, extra) in extras.iter().enumerate() {
            if let Some(next) = extras.get(index + 1) {
                e.set(*extra, BSExtraData::pNext, next.cast());
            }
        }
        if let Some(first) = extras.first() {
            e.set(list, ExtraDataList::pHead, first.cast());
        }
        e.call(0x0041_2970, &args![list]);
    }

    /// Runs the save writer on one extra data and returns the chunks.
    fn save_one(
        e: &mut Engine,
        chunks: &Chunks,
        extra: Ptr<BSExtraData>,
    ) -> Vec<(String, Vec<u8>)> {
        chunks.borrow_mut().clear();
        save_extras(e, &[extra]);
        chunks.borrow().clone()
    }

    #[test]
    fn save_writes_the_form_id_of_the_form_types() {
        let (mut e, chunks) = save_engine();
        let mut extras = vec![];
        let cases = [
            (0x03u8, "XCWT"),
            (0x07, "XCMO"),
            (0x08, "XCCM"),
            (0x21, "XOWN"),
            (0x22, "XGLB"),
            (0x3f, "XPSN"),
            (0x59, "XCIM"),
            (0x81, "XCAS"),
        ];
        for (index, &(extra_type, _)) in cases.iter().enumerate() {
            let form = form_with_id(&mut e, 0x1000 + index as u32);
            extras.push(extra_with(&mut e, extra_type, &[(0x0c, form)]));
        }
        e.call_log = Some(vec![]);
        save_extras(&mut e, &extras);
        let log = e.call_log.take().unwrap();
        let expected: Vec<_> = cases
            .iter()
            .enumerate()
            .map(|(index, &(_, name))| chunk(name, word_bytes(&[0x1000 + index as u32])))
            .collect();
        assert_eq!(*chunks.borrow(), expected);
        // One lock with no name, released at the end.
        assert_eq!(calls_to(&log, LOCK), vec![vec![EXTRA_CRIT_SECTION, 0]]);
        assert_locked_and_released(&log);
    }

    #[test]
    fn save_skips_the_optional_forms_that_are_null() {
        let (mut e, chunks) = save_engine();
        let cases = [
            (0x3bu8, "XRTM"),
            (0x3c, "XMRC"),
            (0x44, "XTRG"),
            (0x51, "XLKR"),
            (0x63, "XMBR"),
            (0x67, "XEMI"),
            (0x74, "XEZN"),
        ];
        for &(extra_type, name) in &cases {
            let null = extra_with(&mut e, extra_type, &[(0x0c, 0)]);
            assert!(save_one(&mut e, &chunks, null).is_empty(), "{name}");
            let form = form_with_id(&mut e, 0x77);
            let extra = extra_with(&mut e, extra_type, &[(0x0c, form)]);
            assert_eq!(
                save_one(&mut e, &chunks, extra),
                vec![chunk(name, word_bytes(&[0x77]))],
                "{name}"
            );
        }
    }

    #[test]
    fn save_writes_the_scalar_types() {
        let (mut e, chunks) = save_engine();
        let bits = |value: f32| value.to_bits();
        let cases: Vec<(u8, u32, &str, Vec<u8>)> = vec![
            (0x0e, 0x1234_5678, "XACT", word_bytes(&[0x78])),
            (0x1e, 0xdead_beef, "XLCM", word_bytes(&[0xdead_beef])),
            (0x23, 0x0000_0007, "XRNK", word_bytes(&[7])),
            (0x24, 0x0000_fffe, "XCNT", word_bytes(&[0xffff_fffe])),
            (0x25, bits(12.75), "XHLT", word_bytes(&[12])),
            (0x25, bits(-3.5), "XHLT", word_bytes(&[-3i32 as u32])),
            (0x26, 0x1234_5609, "XUSE", word_bytes(&[9])),
            (0x27, bits(2.5), "XTIM", word_bytes(&[bits(2.5)])),
            (0x28, 0x7f80_0001, "XCHG", word_bytes(&[0x7fc0_0001])),
            (0x5c, bits(-1.25), "XRDS", word_bytes(&[bits(-1.25)])),
            (0x5d, bits(0.5), "XRAD", word_bytes(&[bits(0.5)])),
            (0x7a, 0xff80_0001, "XHLP", word_bytes(&[0xffc0_0001])),
            (0x8d, 0xffff_ff05, "XHLT", vec![5]),
        ];
        for (extra_type, word, name, data) in cases {
            let extra = extra_with(&mut e, extra_type, &[(0x0c, word)]);
            assert_eq!(
                save_one(&mut e, &chunks, extra),
                vec![chunk(name, data)],
                "type {extra_type:#x}"
            );
        }
    }

    #[test]
    fn save_writes_the_region_list_as_an_array_of_form_ids() {
        let (mut e, chunks) = save_engine();
        // The extra data points at an object whose list starts at +4.
        let region_list = e.mem.alloc(0x20);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let (form_a, form_b) = (form_with_id(&mut e, 0xa1), form_with_id(&mut e, 0xb2));
        e.mem.set_u32(region_list + 4, form_a);
        e.mem.set_u32(region_list + 8, first);
        e.mem.set_u32(first, form_b);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, 0);
        // (the third node has no item: counted as no region, but the walk
        // would ask it for a form id, so the list ends before it)
        e.mem.set_u32(first + 4, 0);
        let extra = extra_with(&mut e, 0x04, &[(0x0c, region_list)]);
        e.call_log = Some(vec![]);
        let saved = save_one(&mut e, &chunks, extra);
        let log = e.call_log.take().unwrap();
        assert_eq!(saved, vec![chunk("XCLR", word_bytes(&[0xa1, 0xb2]))]);
        // The 8-byte buffer is freed after the chunk is added.
        let size = calls_to(&log, OPERATOR_NEW);
        assert_eq!(size, vec![vec![8]]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), 1);

        // No regions (an empty first node), or no list at all: nothing.
        let empty_list = e.mem.alloc(0x20);
        let extra = extra_with(&mut e, 0x04, &[(0x0c, empty_list)]);
        assert!(save_one(&mut e, &chunks, extra).is_empty());
        let extra = extra_with(&mut e, 0x04, &[(0x0c, 0)]);
        assert!(save_one(&mut e, &chunks, extra).is_empty());
    }

    #[test]
    fn save_writes_the_distant_data_as_three_words() {
        let (mut e, chunks) = save_engine();
        let extra = extra_with(&mut e, 0x13, &[(0x0c, 1), (0x10, 2), (0x14, 3)]);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XLOD", word_bytes(&[1, 2, 3]))]
        );
    }

    #[test]
    fn save_hands_the_member_to_the_saver_of_its_class() {
        let (mut e, chunks) = save_engine();
        for (extra_type, name, saver) in [
            (0x14u8, None, RAGDOLL_DATA_SAVE),
            (0x2b, None, TELEPORT_DATA_SAVE),
            (0x2c, Some("XMRK"), MAP_MARKER_DATA_SAVE),
            (0x90, Some("MMRK"), AUDIO_MARKER_DATA_SAVE),
            (0x91, Some("AMRK"), AUDIO_BUOY_MARKER_DATA_SAVE),
        ] {
            let extra = extra_with(&mut e, extra_type, &[(0x0c, 0x4242)]);
            e.call_log = Some(vec![]);
            let saved = save_one(&mut e, &chunks, extra);
            let log = e.call_log.take().unwrap();
            let expected: Vec<_> = name.map(|n| chunk(n, vec![])).into_iter().collect();
            assert_eq!(saved, expected, "type {extra_type:#x}");
            assert_eq!(calls_to(&log, saver), vec![vec![0x4242]]);
        }
        // The multibound data is only saved when there is some.
        let extra = extra_with(&mut e, 0x62, &[(0x0c, 0)]);
        e.call_log = Some(vec![]);
        save_one(&mut e, &chunks, extra);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MULTIBOUND_MARKER_DATA_SAVE).is_empty());
        let extra = extra_with(&mut e, 0x62, &[(0x0c, 0x99)]);
        e.call_log = Some(vec![]);
        save_one(&mut e, &chunks, extra);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MULTIBOUND_MARKER_DATA_SAVE),
            vec![vec![0x99]]
        );
    }

    #[test]
    fn save_writes_the_chunks_without_data() {
        let (mut e, chunks) = save_engine();
        for (extra_type, name) in [(0x61u8, "XMBP"), (0x80, "XIBS")] {
            let extra = extra_of_type(&mut e, extra_type);
            assert_eq!(save_one(&mut e, &chunks, extra), vec![chunk(name, vec![])]);
        }
        // Types that are not saved leave nothing.
        for extra_type in [0x00u8, 0x05, 0x0b, 0x43, 0x93, 0xff] {
            let extra = extra_of_type(&mut e, extra_type);
            assert!(save_one(&mut e, &chunks, extra).is_empty());
        }
        // The seed byte comes from `GetSeed`.
        let extra = extra_of_type(&mut e, 0x31);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XSED", vec![0x7b])]
        );
    }

    #[test]
    fn save_package_start_location_is_five_words_with_the_cell_as_a_form_id() {
        let (mut e, chunks) = save_engine();
        let cell = form_with_id(&mut e, 0xce11);
        let extra = extra_with(
            &mut e,
            0x18,
            &[(0x0c, cell), (0x10, 0x11), (0x14, 0x22), (0x18, 0x33)],
        );
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XPSL", word_bytes(&[0xce11, 0x11, 0x22, 0x33, 0]))]
        );
        // On a big-endian target the first four words are swapped.
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        let saved = save_one(&mut e, &chunks, extra);
        let swapped: Vec<u32> = [0xce11u32, 0x11, 0x22, 0x33]
            .iter()
            .map(|w| w.swap_bytes())
            .collect();
        assert_eq!(
            saved,
            vec![chunk(
                "XPSL",
                [word_bytes(&swapped), word_bytes(&[0])].concat()
            )]
        );
    }

    #[test]
    fn save_lock_copies_the_level_flags_and_key_id() {
        let (mut e, chunks) = save_engine();
        let key = form_with_id(&mut e, 0x4b);
        let lock_data = e.mem.alloc(0x14);
        e.mem.set_u8(lock_data, 0x55);
        e.mem.set_u32(lock_data + 4, key);
        e.mem.set_u8(lock_data + 8, 0x99);
        e.mem.set_u32(lock_data + 0x0c, 0x1111);
        e.mem.set_u32(lock_data + 0x10, 0x2222);
        let extra = extra_with(&mut e, 0x2a, &[(0x0c, lock_data)]);
        // The tries and times unlocked are not copied.
        let mut expected = vec![0x55, 0, 0, 0];
        expected.extend(word_bytes(&[0x4b]));
        expected.extend([0x99, 0, 0, 0]);
        expected.extend(word_bytes(&[0, 0]));
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XLOC", expected)]
        );

        // No key: 0. Big-endian: only the key id is swapped.
        e.mem.set_u32(lock_data + 4, 0);
        let mut expected = vec![0x55, 0, 0, 0];
        expected.extend(word_bytes(&[0]));
        expected.extend([0x99, 0, 0, 0]);
        expected.extend(word_bytes(&[0, 0]));
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XLOC", expected)]
        );
        e.mem.set_u32(lock_data + 4, key);
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        let saved = save_one(&mut e, &chunks, extra);
        assert_eq!(saved[0].1[4..8], 0x4bu32.swap_bytes().to_le_bytes());
        assert_eq!(saved[0].1[0], 0x55);
    }

    #[test]
    fn save_enable_state_parent_is_the_form_id_and_the_flags_byte() {
        let (mut e, chunks) = save_engine();
        let parent = form_with_id(&mut e, 0x9a);
        let extra = extra_with(&mut e, 0x37, &[(0x0c, parent), (0x10, 0x0102_0304)]);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk(
                "XESP",
                [word_bytes(&[0x9a]), vec![4, 0, 0, 0]].concat()
            )]
        );
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        let saved = save_one(&mut e, &chunks, extra);
        assert_eq!(saved[0].1[..4], 0x9au32.swap_bytes().to_le_bytes());
        // No parent: nothing.
        e.set_global(BIG_ENDIAN_FLAG, 0u8);
        let orphan = extra_with(&mut e, 0x37, &[(0x0c, 0)]);
        assert!(save_one(&mut e, &chunks, orphan).is_empty());
    }

    /// An inline list head at `extra + 0x0c` with the given item pointers
    /// (nodes after the first are allocated).
    fn put_inline_list(e: &mut Engine, extra: Ptr<BSExtraData>, items: &[u32]) {
        let mut node = extra.addr() + 0x0c;
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if index + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    #[test]
    fn save_decal_refs_writes_one_record_per_decal() {
        let (mut e, chunks) = save_engine();
        let first_ref = form_with_id(&mut e, 0xd1);
        let second_ref = form_with_id(&mut e, 0xd2);
        let first = e.mem.alloc(0x1c);
        let second = e.mem.alloc(0x1c);
        e.mem.set_u32(first, first_ref);
        e.mem.set_u32(second, second_ref);
        for index in 1..7 {
            e.mem.set_u32(first + 4 * index, 0x100 + index);
            e.mem.set_u32(second + 4 * index, 0x200 + index);
        }
        let extra = extra_of_type(&mut e, 0x57);
        put_inline_list(&mut e, extra, &[first, second]);
        let record = |id: u32, base: u32| {
            let mut words = vec![id];
            words.extend((1..7).map(|index| base + index));
            chunk("XDCR", word_bytes(&words))
        };
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![record(0xd1, 0x100), record(0xd2, 0x200)]
        );
        // Big-endian: all seven words are swapped.
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        let saved = save_one(&mut e, &chunks, extra);
        assert_eq!(saved[0].1[..4], 0xd1u32.swap_bytes().to_le_bytes());
        assert_eq!(saved[0].1[24..], 0x106u32.swap_bytes().to_le_bytes());
        // An empty list writes nothing.
        e.set_global(BIG_ENDIAN_FLAG, 0u8);
        let empty = extra_of_type(&mut e, 0x57);
        assert!(save_one(&mut e, &chunks, empty).is_empty());
    }

    #[test]
    fn save_reflector_refs_and_lit_water_refs_walk_their_lists() {
        let (mut e, chunks) = save_engine();
        let reference = form_with_id(&mut e, 0x71);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, reference);
        e.mem.set_u32(pair + 4, 0x5eed);
        let extra = extra_of_type(&mut e, 0x66);
        put_inline_list(&mut e, extra, &[pair]);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XPWR", word_bytes(&[0x71, 0x5eed]))]
        );
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        let saved = save_one(&mut e, &chunks, extra);
        assert_eq!(
            saved[0].1,
            word_bytes(&[0x71u32.swap_bytes(), 0x5eedu32.swap_bytes()])
        );
        e.set_global(BIG_ENDIAN_FLAG, 0u8);

        let (water_a, water_b) = (form_with_id(&mut e, 0xa), form_with_id(&mut e, 0xb));
        let extra = extra_of_type(&mut e, 0x85);
        put_inline_list(&mut e, extra, &[water_a, water_b]);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![
                chunk("XLTW", word_bytes(&[0xa])),
                chunk("XLTW", word_bytes(&[0xb]))
            ]
        );
    }

    #[test]
    fn save_radio_data_swaps_the_position_reference_for_its_form_id() {
        let (mut e, chunks) = save_engine();
        let position = form_with_id(&mut e, 0xf00d);
        let extra = extra_with(
            &mut e,
            0x68,
            &[
                (0x0c, 0x1111),
                (0x10, 0x2222),
                (0x14, 0x3333),
                (0x18, position),
            ],
        );
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XRDO", word_bytes(&[0x1111, 0x2222, 0x3333, 0xf00d]))]
        );
        // The extra data gets its reference back.
        assert_eq!(e.mem.u32(extra.addr() + 0x18), position);
        // No position reference: the word stays 0.
        e.mem.set_u32(extra.addr() + 0x18, 0);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XRDO", word_bytes(&[0x1111, 0x2222, 0x3333, 0]))]
        );
    }

    #[test]
    fn save_primitive_is_radii_colour_and_type() {
        let (mut e, chunks) = save_engine();
        let primitive = e.mem.alloc(0x40);
        e.mem.set_u32(primitive + 4, 3);
        for index in 0..4 {
            e.mem.set_u32(primitive + 8 + 4 * index, 0xc0 + index);
        }
        for index in 0..3 {
            e.mem.set_u32(primitive + 0x18 + 4 * index, 0xa0 + index);
        }
        let extra = extra_with(&mut e, 0x6b, &[(0x0c, primitive)]);
        let expected = [0xa0, 0xa1, 0xa2, 0xc0, 0xc1, 0xc2, 0xc3, 3];
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XPRM", word_bytes(&expected))]
        );
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        let swapped: Vec<u32> = expected.iter().map(|w: &u32| w.swap_bytes()).collect();
        assert_eq!(save_one(&mut e, &chunks, extra)[0].1, word_bytes(&swapped));
    }

    #[test]
    fn save_ammo_writes_the_form_id_then_the_count() {
        let (mut e, chunks) = save_engine();
        let ammo = form_with_id(&mut e, 0xa44);
        let extra = extra_with(&mut e, 0x6e, &[(0x0c, ammo), (0x10, 25)]);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![
                chunk("XAMT", word_bytes(&[0xa44])),
                chunk("XAMC", word_bytes(&[25]))
            ]
        );
        let extra = extra_with(&mut e, 0x6e, &[(0x0c, 0), (0x10, 7)]);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![
                chunk("XAMT", word_bytes(&[0])),
                chunk("XAMC", word_bytes(&[7]))
            ]
        );
    }

    #[test]
    fn save_patrol_data_writes_the_time_and_hands_over_the_package_action() {
        let (mut e, chunks) = save_engine();
        let patrol = e.mem.alloc(0x20);
        e.mem.set_f32(patrol, 6.5);
        let extra = extra_with(&mut e, 0x6f, &[(0x0c, patrol)]);
        e.call_log = Some(vec![]);
        let saved = save_one(&mut e, &chunks, extra);
        let log = e.call_log.take().unwrap();
        assert_eq!(saved, vec![chunk("XPRD", word_bytes(&[6.5f32.to_bits()]))]);
        assert_eq!(
            calls_to(&log, PACKAGE_EVENT_ACTION_SAVE),
            vec![vec![patrol + 4, 0x4150_5058]]
        );
    }

    #[test]
    fn save_planes_write_extents_centre_and_axis_angle() {
        let (mut e, chunks) = save_engine();
        e.register(MATRIX_TO_AXIS_ANGLE, |e, a| {
            // (matrix, angle, x, y, z)
            e.mem.set_f32(a[1], 0.75);
            e.mem.set_f32(a[2], 1.0);
            e.mem.set_f32(a[3], 0.0);
            e.mem.set_f32(a[4], -1.0);
            returns(1)
        });
        let plane = e.mem.alloc(0x100);
        e.mem.set_u32(plane + 0x14, 0xe1);
        e.mem.set_u32(plane + 0x18, 0xe2);
        for index in 0..3 {
            e.mem.set_u32(plane + 8 + 4 * index, 0xc0 + index);
        }
        for index in 0..9 {
            e.mem.set_u32(plane + 0x1c + 4 * index, 0x500 + index);
        }
        let expected = [
            0xe1,
            0xe2,
            0xc0,
            0xc1,
            0xc2,
            1.0f32.to_bits(),
            0.0f32.to_bits(),
            (-1.0f32).to_bits(),
            0.75f32.to_bits(),
        ];
        for (extra_type, name) in [(0x71u8, "XOCP"), (0x78, "XPTL")] {
            let extra = extra_with(&mut e, extra_type, &[(0x0c, plane)]);
            e.call_log = Some(vec![]);
            let saved = save_one(&mut e, &chunks, extra);
            let log = e.call_log.take().unwrap();
            assert_eq!(saved, vec![chunk(name, word_bytes(&expected))]);
            // The matrix handed over is a copy of the nine words at +0x1c.
            let angles = calls_to(&log, MATRIX_TO_AXIS_ANGLE);
            assert_eq!(angles.len(), 1);
            assert_ne!(angles[0][0], plane + 0x1c);
        }
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        let extra = extra_with(&mut e, 0x71, &[(0x0c, plane)]);
        let saved = save_one(&mut e, &chunks, extra);
        assert_eq!(saved[0].1[..4], 0xe1u32.swap_bytes().to_le_bytes());
        assert_eq!(
            saved[0].1[32..],
            0.75f32.to_bits().swap_bytes().to_le_bytes()
        );
    }

    #[test]
    fn save_collision_data_swaps_the_word_in_place_around_the_chunk() {
        let (mut e, chunks) = save_engine();
        let word = e.mem.alloc(8);
        e.mem.set_u32(word, 0x0102_0304);
        let extra = extra_with(&mut e, 0x72, &[(0x0c, word)]);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XTRI", word_bytes(&[0x0102_0304]))]
        );
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XTRI", word_bytes(&[0x0403_0201]))]
        );
        assert_eq!(e.mem.u32(word), 0x0102_0304);
    }

    #[test]
    fn save_linked_reference_tables_write_form_ids_or_zero() {
        let (mut e, chunks) = save_engine();
        let (a, c) = (form_with_id(&mut e, 0x1a), form_with_id(&mut e, 0x1c));
        let table = e.mem.alloc(16);
        for (index, form) in [a, 0, c, 0].iter().enumerate() {
            e.mem.set_u32(table + 4 * index as u32, *form);
        }
        let extra = extra_with(&mut e, 0x76, &[(0x0c, table)]);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XORD", word_bytes(&[0x1a, 0, 0x1c, 0]))]
        );
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, a);
        let extra = extra_with(&mut e, 0x77, &[(0x0c, pair)]);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk("XPOD", word_bytes(&[0x1a, 0]))]
        );
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        let saved = save_one(&mut e, &chunks, extra);
        assert_eq!(saved[0].1, word_bytes(&[0x1au32.swap_bytes(), 0]));
    }

    #[test]
    fn save_room_data_writes_the_count_with_the_master_flag_then_each_room() {
        let (mut e, chunks) = save_engine();
        let (room_a, room_b) = (form_with_id(&mut e, 0x2a), form_with_id(&mut e, 0x2b));
        let data = e.mem.alloc(0x20);
        let second = e.mem.alloc(8);
        e.mem.set_u32(data + 8, room_a);
        e.mem.set_u32(data + 12, second);
        e.mem.set_u32(second, room_b);
        let extra = extra_with(&mut e, 0x7b, &[(0x0c, data)]);
        let expected = vec![
            chunk("XRMR", word_bytes(&[2])),
            chunk("XLRM", word_bytes(&[0x2a])),
            chunk("XLRM", word_bytes(&[0x2b])),
        ];
        assert_eq!(save_one(&mut e, &chunks, extra), expected);
        // The master flag (byte at +0x10) goes to bit 16 of the count.
        e.mem.set_u8(data + 0x10, 1);
        let saved = save_one(&mut e, &chunks, extra);
        assert_eq!(saved[0], chunk("XRMR", word_bytes(&[0x1_0002])));
        // An item that is null ends the walk.
        e.mem.set_u32(second, 0);
        let saved = save_one(&mut e, &chunks, extra);
        assert_eq!(saved.len(), 2);
    }

    #[test]
    fn save_activate_ref_writes_flags_entries_and_the_text_override() {
        let (mut e, chunks) = save_engine();
        let (ref_a, ref_b) = (form_with_id(&mut e, 0xaa), form_with_id(&mut e, 0xbb));
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, ref_a);
        e.mem.set_u32(first + 4, 0x1);
        e.mem.set_u32(second, ref_b);
        e.mem.set_u32(second + 4, 0x2);
        let extra = extra_of_type(&mut e, 0x53);
        put_inline_list(&mut e, extra, &[first, second]);
        e.mem.set_u8(extra.addr() + 0x14, 0x80);
        let expected = vec![
            chunk("XAPD", vec![0x80]),
            chunk("XAPR", word_bytes(&[0xaa, 1])),
            chunk("XAPR", word_bytes(&[0xbb, 2])),
        ];
        assert_eq!(save_one(&mut e, &chunks, extra), expected);

        // With a text override: copied twice by the list's accessor.
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"Hi!");
        e.mem.set_u16(extra.addr() + 0x18 + 4, 3);
        e.register_double(GET_ACTIVATE_TEXT, move |e, a| {
            e.mem.set_u32(a[1], text);
            e.mem.set_u16(a[1] + 4, 3);
            returns(a[1])
        });
        e.call_log = Some(vec![]);
        let saved = save_one(&mut e, &chunks, extra);
        let log = e.call_log.take().unwrap();
        assert_eq!(saved.len(), 4);
        assert_eq!(saved[3], chunk("XATO", b"Hi!\0".to_vec()));
        assert_eq!(calls_to(&log, GET_ACTIVATE_TEXT).len(), 2);
        assert_eq!(calls_to(&log, STRING_DESTROY).len(), 2);
    }

    #[test]
    fn save_navmesh_portal_writes_the_member_with_its_form_id() {
        let (mut e, chunks) = save_engine();
        let target = form_with_id(&mut e, 0x7a);
        let extra = extra_with(&mut e, 0x5a, &[(0x0c, target), (0x10, 0x0002_0009)]);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![chunk(
                "XNDP",
                [word_bytes(&[0x7a]), vec![9, 0, 0, 0]].concat()
            )]
        );
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        let saved = save_one(&mut e, &chunks, extra);
        assert_eq!(saved[0].1[..4], 0x7au32.swap_bytes().to_le_bytes());
        assert_eq!(saved[0].1[4..6], 0x0009u16.swap_bytes().to_le_bytes());
        // No target: nothing is written.
        let empty = extra_with(&mut e, 0x5a, &[(0x0c, 0)]);
        assert!(save_one(&mut e, &chunks, empty).is_empty());
    }

    #[test]
    fn save_type_92_writes_the_word_and_the_float() {
        let (mut e, chunks) = save_engine();
        let extra = extra_with(&mut e, 0x92, &[(0x0c, 0x1234), (0x10, 0x7f80_0001)]);
        assert_eq!(
            save_one(&mut e, &chunks, extra),
            vec![
                chunk("XSRF", word_bytes(&[0x1234])),
                chunk("XSRD", word_bytes(&[0x7fc0_0001]))
            ]
        );
    }

    // --- the loader ----------------------------------------------------------

    use std::collections::VecDeque;

    /// What the test `TESFile` offers the loader.
    #[derive(Default)]
    struct FileState {
        /// The current chunk (four letters).
        chunk: [u8; 4],
        /// The words `GetChunkData(&value)` hands out in turn.
        words: VecDeque<u32>,
        /// The bytes `GetChunkData(buffer, size)` copies (as many as asked).
        data: Vec<u8>,
        /// The size of the current chunk.
        size: u32,
        /// The file's byte-swap flag.
        swap: bool,
    }

    /// The loader's `this`, the test `TESFile` and the engine.
    struct Loader {
        e: Engine,
        state: Shared<FileState>,
        file: Ptr,
        list: Ptr<ExtraDataList>,
    }

    impl Loader {
        fn new() -> Loader {
            let mut e = swap_engine();
            e.map(0x0101_4000, 0x1000);
            e.set_global(LOD_DEFAULT, 0.97f32);
            let state: Shared<FileState> = Rc::default();
            let file: Ptr = Ptr::new(e.mem.alloc(0x300));
            let shared = state.clone();
            e.register_double(GET_TES_CHUNK, move |_, _| {
                returns(u32::from_le_bytes(shared.borrow().chunk))
            });
            let shared = state.clone();
            e.register_double(GET_CHUNK_WORD, move |e, a| {
                let word = shared.borrow_mut().words.pop_front().unwrap_or(0);
                e.mem.set_u32(a[1], word);
                returns(1)
            });
            let shared = state.clone();
            e.register_double(GET_CHUNK_DATA, move |e, a| {
                let data = shared.borrow().data.clone();
                let count = (a[2] as usize).min(data.len());
                e.mem.write(a[1], &data[..count]);
                returns(1)
            });
            let shared = state.clone();
            e.register_double(GET_CHUNK_SIZE, move |_, _| returns(shared.borrow().size));
            let shared = state.clone();
            e.register_double(FILE_NEEDS_SWAP, move |_, _| {
                returns(shared.borrow().swap as u32)
            });
            e.register(SWAP_WORD_AT, |e, a| {
                let value = e.mem.u32(a[0]);
                e.mem.set_u32(a[0], value.swap_bytes());
                Ret::default()
            });
            e.register(SWAP_WORD_AT_4, |e, a| {
                let value = e.mem.u32(a[0] + 4);
                e.mem.set_u32(a[0] + 4, value.swap_bytes());
                Ret::default()
            });
            e.register(SWAP_TWO_WORDS, |e, a| {
                for offset in [0, 4] {
                    let value = e.mem.u32(a[0] + offset);
                    e.mem.set_u32(a[0] + offset, value.swap_bytes());
                }
                Ret::default()
            });
            let list = e.new_object();
            Loader {
                e,
                state,
                file,
                list,
            }
        }

        /// Loads one chunk whose `GetChunkData(&value)` words, bytes and size
        /// are given; returns the calls made.
        fn run(&mut self, chunk: &[u8; 4], words: &[u32], data: &[u8], size: u32) -> Log {
            self.run_with_buffer(chunk, words, data, size, Ptr::NULL)
        }

        fn run_with_buffer(
            &mut self,
            chunk: &[u8; 4],
            words: &[u32],
            data: &[u8],
            size: u32,
            buffer: Ptr,
        ) -> Log {
            {
                let mut state = self.state.borrow_mut();
                state.chunk = *chunk;
                state.words = words.iter().copied().collect();
                state.data = data.to_vec();
                state.size = size;
            }
            self.e.call_log = Some(vec![]);
            self.e
                .call(0x0041_44a0, &args![self.list, self.file, buffer]);
            self.e.call_log.take().unwrap()
        }

        fn swapping(&self, swap: bool) {
            self.state.borrow_mut().swap = swap;
        }
    }

    /// A double for `address` that records its arguments and `count` words at
    /// the pointer in argument `index` (as they are when it is called).
    fn capture_block(
        e: &mut Engine,
        address: u32,
        index: usize,
        count: u32,
    ) -> Shared<Vec<(Vec<u32>, Vec<u32>)>> {
        let seen: Shared<Vec<(Vec<u32>, Vec<u32>)>> = Rc::default();
        let log = seen.clone();
        e.register_double(address, move |e, a| {
            let block = words(e, a[index], count);
            log.borrow_mut().push((a.to_vec(), block));
            Ret::default()
        });
        seen
    }

    fn bytes_of(words: &[u32]) -> Vec<u8> {
        word_bytes(words)
    }

    #[test]
    fn load_hands_one_word_to_the_setter_of_each_word_chunk() {
        for &(name, setter) in LOAD_WORD_SETTERS {
            let mut l = Loader::new();
            stub(&mut l.e, setter);
            let log = l.run(name, &[0x1234_5678], &[], 4);
            assert_eq!(
                calls_to(&log, setter),
                vec![vec![l.list.addr(), 0x1234_5678]],
                "{}",
                String::from_utf8_lossy(name)
            );
        }
    }

    #[test]
    fn load_hands_one_float_to_the_setter_of_each_float_chunk() {
        for &(name, setter) in LOAD_FLOAT_SETTERS {
            let mut l = Loader::new();
            stub(&mut l.e, setter);
            let log = l.run(name, &[2.5f32.to_bits()], &[], 4);
            assert_eq!(
                calls_to(&log, setter),
                vec![vec![l.list.addr(), 2.5f32.to_bits()]]
            );
            // A signalling NaN comes back quiet (x87 load and store).
            let log = l.run(name, &[0x7f80_0001], &[], 4);
            assert_eq!(
                calls_to(&log, setter),
                vec![vec![l.list.addr(), 0x7fc0_0001]]
            );
            // The local starts at 0.0 when the file gives nothing.
            let log = l.run(name, &[], &[], 4);
            assert_eq!(calls_to(&log, setter), vec![vec![l.list.addr(), 0]]);
        }
    }

    #[test]
    fn load_health_uses_and_count_convert_the_word() {
        let mut l = Loader::new();
        for setter in [SET_HEALTH, SET_USES, SET_COUNT] {
            stub(&mut l.e, setter);
        }
        // The file holds an integer; the setter takes a float.
        let log = l.run(b"XHLT", &[0xffff_fffd], &[], 4);
        assert_eq!(
            calls_to(&log, SET_HEALTH),
            vec![vec![l.list.addr(), (-3.0f32).to_bits()]]
        );
        let log = l.run(b"XHLT", &[16_777_217], &[], 4);
        assert_eq!(
            calls_to(&log, SET_HEALTH),
            vec![vec![l.list.addr(), 16_777_216.0f32.to_bits()]]
        );
        let log = l.run(b"XUSE", &[0x1234_5609], &[], 4);
        assert_eq!(calls_to(&log, SET_USES), vec![vec![l.list.addr(), 9]]);
        let log = l.run(b"XCNT", &[0x1234_ff02], &[], 4);
        assert_eq!(calls_to(&log, SET_COUNT), vec![vec![l.list.addr(), 0xff02]]);
    }

    #[test]
    fn load_ammo_chunks_set_the_count_or_the_ammo_and_999() {
        let mut l = Loader::new();
        stub(&mut l.e, SET_AMMO);
        let ammo_extra: Ptr = Ptr::new(l.e.mem.alloc(0x20));
        l.e.mem.set_u32(ammo_extra.addr() + 0x0c, 0xa44);
        let found = Rc::new(RefCell::new(0u32));
        let shared = found.clone();
        l.e.register_double(GET_AMMO, move |_, _| returns(*shared.borrow()));
        // XAMC: nothing without an ammo extra data.
        let log = l.run(b"XAMC", &[33], &[], 4);
        assert!(calls_to(&log, SET_AMMO).is_empty());
        *found.borrow_mut() = ammo_extra.addr();
        let log = l.run(b"XAMC", &[33], &[], 4);
        assert_eq!(
            calls_to(&log, SET_AMMO),
            vec![vec![l.list.addr(), 0xa44, 33]]
        );
        // XAMT: the ammo id with a count of 999.
        let log = l.run(b"XAMT", &[0xa55], &[], 4);
        assert_eq!(
            calls_to(&log, SET_AMMO),
            vec![vec![l.list.addr(), 0xa55, 0x3e7]]
        );
    }

    #[test]
    fn load_lock_reads_a_refr_lock_into_a_new_or_the_existing_extra_data() {
        let mut l = Loader::new();
        l.e.register(EXTRA_LOCK_INIT, |e, a| {
            e.mem.set_u8(a[0] + 4, 0x2a);
            e.mem.set_u32(a[0] + 0x0c, a[1]);
            returns(a[0])
        });
        stub(&mut l.e, REFR_LOCK_SET_LOCKED);
        let data: Vec<u8> = (1..=20).collect();
        let log = l.run(b"XLOC", &[], &data, 0x14);
        assert_eq!(chain_types(&l.e, l.list.cast()), vec![0x2a]);
        let extra: Ptr<BSExtraData> = l.e.get(l.list, ExtraDataList::pHead).cast();
        let lock_data = l.e.mem.u32(extra.addr() + 0x0c);
        assert_eq!(l.e.mem.bytes(lock_data, 0x14), data);
        // The lock data (0x14) and the extra data (0x10) were allocated.
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14], vec![0x10]]);
        assert_eq!(
            calls_to(&log, REFR_LOCK_SET_LOCKED),
            vec![vec![lock_data, 1]]
        );
        assert!(calls_to(&log, SWAP_WORD_AT_4).is_empty());

        // Again: the lock data of the extra data is reused, and swapped first
        // on a file that needs it.
        l.swapping(true);
        let log = l.run(b"XLOC", &[], &[0xff; 0x14], 0x14);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(chain_types(&l.e, l.list.cast()), vec![0x2a]);
        assert_eq!(l.e.mem.u32(lock_data + 4), 0xffff_ffff);
        assert_eq!(calls_to(&log, SWAP_WORD_AT_4), vec![vec![lock_data]]);
    }

    #[test]
    fn load_radio_data_starts_from_its_defaults() {
        let mut l = Loader::new();
        let seen = capture_block(&mut l.e, SET_RADIO_DATA, 1, 4);
        let log = l.run(b"XRDO", &[], &[], 0x10);
        assert_eq!(seen.borrow()[0].1, vec![0, 1, 0, 0]);
        assert!(calls_to(&log, SWAP_DWORD).is_empty());
        l.swapping(true);
        let data = bytes_of(&[0x0102_0304, 2, 0x0a0b_0c0d, 0x1111_2222]);
        l.run(b"XRDO", &[], &data, 0x10);
        assert_eq!(
            seen.borrow()[1].1,
            vec![0x0403_0201, 0x0200_0000, 0x0d0c_0b0a, 0x2222_1111]
        );
        assert_eq!(seen.borrow()[1].0[0], l.list.addr());
    }

    #[test]
    fn load_members_are_built_when_missing_and_then_loaded() {
        // (chunk, getter, size, constructor, setter, loader)
        let cases = [
            (
                b"XTEL",
                GET_TELEPORT,
                0x20,
                DOOR_TELEPORT_DATA_INIT,
                SET_TELEPORT,
                DOOR_TELEPORT_DATA_LOAD,
            ),
            (
                b"XMRK",
                GET_MAP_MARKER,
                0x14,
                MAP_MARKER_DATA_INIT,
                SET_MAP_MARKER,
                MAP_MARKER_DATA_LOAD,
            ),
            (
                b"MMRK",
                GET_AUDIO_MARKER,
                0x34,
                AUDIO_MARKER_DATA_INIT,
                SET_AUDIO_MARKER,
                AUDIO_MARKER_DATA_LOAD,
            ),
            (
                b"AMRK",
                GET_AUDIO_BUOY_MARKER,
                0x08,
                AUDIO_BUOY_MARKER_DATA_INIT,
                SET_AUDIO_BUOY_MARKER,
                AUDIO_BUOY_MARKER_DATA_LOAD,
            ),
        ];
        for (name, getter, size, constructor, setter, loader) in cases {
            let mut l = Loader::new();
            let existing = Rc::new(RefCell::new(0u32));
            let shared = existing.clone();
            l.e.register_double(getter, move |_, _| returns(*shared.borrow()));
            l.e.register(constructor, |_, a| returns(a[0]));
            stub(&mut l.e, setter);
            stub(&mut l.e, loader);
            let log = l.run(name, &[], &[], 0);
            let made = calls_to(&log, constructor);
            assert_eq!(made.len(), 1);
            assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![size]]);
            assert_eq!(
                calls_to(&log, setter),
                vec![vec![l.list.addr(), made[0][0]]]
            );
            assert_eq!(
                calls_to(&log, loader),
                vec![vec![made[0][0], l.file.addr()]]
            );

            // An existing member is loaded in place.
            *existing.borrow_mut() = 0x7777;
            let log = l.run(name, &[], &[], 0);
            assert!(calls_to(&log, OPERATOR_NEW).is_empty());
            assert!(calls_to(&log, setter).is_empty());
            assert_eq!(calls_to(&log, loader), vec![vec![0x7777, l.file.addr()]]);
        }
    }

    #[test]
    fn load_seed_is_a_byte_from_a_word_or_a_one_byte_chunk() {
        let mut l = Loader::new();
        stub(&mut l.e, SET_SEED);
        let log = l.run(b"XSED", &[0x1234_56ab], &[], 4);
        assert_eq!(calls_to(&log, SET_SEED), vec![vec![l.list.addr(), 0xab]]);
        let log = l.run(b"XSED", &[0x1234_56ab], &[0x42], 1);
        assert_eq!(calls_to(&log, SET_SEED), vec![vec![l.list.addr(), 0x42]]);
        let log = l.run(b"XSED", &[], &[], 1);
        assert_eq!(calls_to(&log, SET_SEED), vec![vec![l.list.addr(), 0]]);
    }

    #[test]
    fn load_region_list_gathers_the_known_regions() {
        let mut l = Loader::new();
        l.e.set_global(DATA_HANDLER, 0x1111u32);
        l.e.register(REGION_LIST_INIT, |_, a| returns(a[0]));
        l.e.register(DATA_HANDLER_REGIONS, |_, a| returns(a[0] + 0x1000));
        // The file's ids become ids of the load order (+0x0100_0000).
        l.e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x0100_0000);
            Ret::default()
        });
        // Only id 0x0100_0002 is a known region.
        l.e.register(REGION_LIST_FIND, |_, a| {
            returns(if a[1] == 0x0100_0002 { 0x5e91 } else { 0 })
        });
        stub(&mut l.e, REGION_LIST_ADD);
        stub(&mut l.e, SET_REGION_LIST);
        let data = bytes_of(&[1, 2, 3]);
        let log = l.run(b"XCLR", &[], &data, 12);
        let region_list = calls_to(&log, REGION_LIST_INIT);
        assert_eq!(region_list.len(), 1);
        assert_eq!(region_list[0][1], 0);
        assert_eq!(
            calls_to(&log, REGION_LIST_FIND),
            vec![
                vec![0x1111 + 0x1000, 0x0100_0001],
                vec![0x1111 + 0x1000, 0x0100_0002],
                vec![0x1111 + 0x1000, 0x0100_0003]
            ]
        );
        assert_eq!(
            calls_to(&log, REGION_LIST_ADD),
            vec![vec![region_list[0][0], 0x5e91]]
        );
        assert_eq!(
            calls_to(&log, SET_REGION_LIST),
            vec![vec![l.list.addr(), region_list[0][0]]]
        );
        // The buffer of three ids is allocated and freed.
        let buffer = calls_to(&log, OPERATOR_NEW);
        assert_eq!(buffer.last().unwrap(), &vec![12]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), 1);

        // A file that needs it has each id swapped first.
        l.swapping(true);
        let log = l.run(b"XCLR", &[], &bytes_of(&[0x0100_0000]), 4);
        assert_eq!(calls_to(&log, SWAP_DWORD).len(), 1);
        assert_eq!(calls_to(&log, ADD_COMPILE_INDEX).len(), 1);
    }

    #[test]
    fn load_region_list_of_a_bad_size_only_logs_the_file_name() {
        let mut l = Loader::new();
        l.e.register(FILE_NAME, |_, a| returns(a[0] + 0x20));
        stub(&mut l.e, LOG_MESSAGE);
        let log = l.run(b"XCLR", &[], &[], 6);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_BAD_REGION_LIST, l.file.addr() + 0x20]]
        );
        assert!(calls_to(&log, REGION_LIST_INIT).is_empty());
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn load_package_start_location_passes_the_record_to_the_setter() {
        let mut l = Loader::new();
        stub(&mut l.e, PATH_LOCATION_INIT);
        let seen = capture_block(&mut l.e, SET_PACKAGE_START_LOCATION, 3, 3);
        let data = bytes_of(&[0xce11, 1, 2, 3, 0x7f80_0001]);
        l.run(b"XPSL", &[], &data, 0x14);
        let (args, block) = seen.borrow()[0].clone();
        assert_eq!(args[..3], [l.list.addr(), 0xce11, 0]);
        assert_eq!(block, vec![1, 2, 3]);
        // The z rotation word goes through the x87 stack.
        assert_eq!(args[4], 0x7fc0_0001);
        // On a big-endian file the first four words are swapped.
        l.swapping(true);
        l.run(b"XPSL", &[], &data, 0x14);
        let (args, block) = seen.borrow()[1].clone();
        assert_eq!(args[1], 0xce11u32.swap_bytes());
        assert_eq!(
            block,
            vec![1u32.swap_bytes(), 2u32.swap_bytes(), 3u32.swap_bytes()]
        );
        assert_eq!(args[4], 0x7fc0_0001);
    }

    #[test]
    fn load_ragdoll_is_built_once_and_removed_when_it_fails() {
        let mut l = Loader::new();
        l.e.register(EXTRA_RAGDOLL_DATA_INIT, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            e.mem.set_u8(a[0] + 4, 0x14);
            returns(a[0])
        });
        l.e.register(RAGDOLL_DATA_INIT, |_, a| returns(a[0]));
        stub(&mut l.e, LOG_MESSAGE);
        let loads = Rc::new(RefCell::new(true));
        let shared = loads.clone();
        l.e.register_double(RAGDOLL_DATA_LOAD, move |_, _| {
            returns(*shared.borrow() as u32)
        });
        let log = l.run(b"XRGD", &[], &[], 0);
        assert_eq!(chain_types(&l.e, l.list.cast()), vec![0x14]);
        let extra: Ptr<BSExtraData> = l.e.get(l.list, ExtraDataList::pHead).cast();
        let ragdoll = l.e.mem.u32(extra.addr() + 0x0c);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10], vec![0x14]]);
        assert_eq!(
            calls_to(&log, RAGDOLL_DATA_LOAD),
            vec![vec![ragdoll, l.file.addr()]]
        );
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());

        // The other chunk name reaches the same code; the existing extra data
        // is used, and a failed load removes (and deletes) it.
        *loads.borrow_mut() = false;
        let log = l.run(b"XRGB", &[], &[], 0);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&log, LOG_MESSAGE), vec![vec![MESSAGE_BAD_RAGDOLL]]);
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(chain_types(&l.e, l.list.cast()).is_empty());
    }

    #[test]
    fn load_distant_data_defaults_its_third_float() {
        let mut l = Loader::new();
        let seen = capture_block(&mut l.e, SET_DISTANT_DATA, 1, 3);
        l.run(b"XLOD", &[], &[], 12);
        assert_eq!(seen.borrow()[0].1, vec![0, 0, 0.97f32.to_bits()]);
        let data = bytes_of(&[1, 2, 3]);
        let log = l.run(b"XLOD", &[], &data, 12);
        assert_eq!(seen.borrow()[1].1, vec![1, 2, 3]);
        assert!(calls_to(&log, SWAP_DWORD).is_empty());
        l.swapping(true);
        let log = l.run(b"XLOD", &[], &data, 12);
        assert_eq!(calls_to(&log, SWAP_DWORD).len(), 3);
        assert_eq!(
            seen.borrow()[2].1,
            vec![1u32.swap_bytes(), 2u32.swap_bytes(), 3u32.swap_bytes()]
        );
    }

    #[test]
    fn load_enable_state_parent_hands_the_id_and_the_flags_byte() {
        let mut l = Loader::new();
        stub(&mut l.e, SET_ENABLE_STATE_PARENT);
        stub(&mut l.e, SET_ENABLE_STATE_FLAGS);
        let mut data = bytes_of(&[0x0102_0304]);
        data.extend([0x81, 0xee, 0xee, 0xee]);
        let log = l.run(b"XESP", &[], &data, 8);
        assert_eq!(
            calls_to(&log, SET_ENABLE_STATE_PARENT),
            vec![vec![l.list.addr(), 0x0102_0304]]
        );
        assert_eq!(
            calls_to(&log, SET_ENABLE_STATE_FLAGS),
            vec![vec![l.list.addr(), 0x81]]
        );
        l.swapping(true);
        let log = l.run(b"XESP", &[], &data, 8);
        assert_eq!(
            calls_to(&log, SET_ENABLE_STATE_PARENT),
            vec![vec![l.list.addr(), 0x0403_0201]]
        );
    }

    fn activate_engine() -> Loader {
        let mut l = Loader::new();
        l.e.register(ACTIVATE_REF_INIT, |e, a| {
            e.mem.set_u8(a[0] + 4, 0x53);
            returns(a[0])
        });
        l
    }

    #[test]
    fn load_activate_flags_use_a_word_or_a_byte_chunk() {
        let mut l = activate_engine();
        let log = l.run(b"XAPD", &[0x1234_5680], &[], 4);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x20]]);
        assert_eq!(chain_types(&l.e, l.list.cast()), vec![0x53]);
        let extra: Ptr<BSExtraData> = l.e.get(l.list, ExtraDataList::pHead).cast();
        assert_eq!(l.e.mem.u8(extra.addr() + 0x14), 0x80);
        // Existing extra data, a one-byte chunk.
        let log = l.run(b"XAPD", &[], &[0x42], 1);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(l.e.mem.u8(extra.addr() + 0x14), 0x42);
    }

    #[test]
    fn load_activate_text_override_is_read_whole_and_freed() {
        let mut l = activate_engine();
        let texts: Shared<Vec<Vec<u8>>> = Rc::default();
        let log = texts.clone();
        l.e.register_double(SET_ACTIVATE_TEXT_OVERRIDE, move |e, a| {
            log.borrow_mut().push(e.mem.cstr(a[1]));
            Ret::default()
        });
        let run = l.run(b"XATO", &[], b"Use\0", 4);
        assert_eq!(*texts.borrow(), vec![b"Use".to_vec()]);
        let buffer = calls_to(&run, SET_ACTIVATE_TEXT_OVERRIDE)[0][1];
        assert_eq!(calls_to(&run, OPERATOR_DELETE), vec![vec![buffer]]);
        // Sizes up to 1 read nothing, but the extra data is still ensured.
        let mut l = activate_engine();
        stub(&mut l.e, SET_ACTIVATE_TEXT_OVERRIDE);
        let run = l.run(b"XATO", &[], b"\0", 1);
        assert!(calls_to(&run, SET_ACTIVATE_TEXT_OVERRIDE).is_empty());
        assert_eq!(chain_types(&l.e, l.list.cast()), vec![0x53]);
    }

    #[test]
    fn load_activate_entries_are_added_to_the_parent_list() {
        let mut l = activate_engine();
        let seen = capture_block(&mut l.e, LIST_ADD_HEAD, 1, 1);
        // XAPR: 8 bytes, [reference id, delay].
        l.run(b"XAPR", &[], &bytes_of(&[0xaa, 0xbb]), 8);
        let extra: Ptr<BSExtraData> = l.e.get(l.list, ExtraDataList::pHead).cast();
        let (args, block) = seen.borrow()[0].clone();
        assert_eq!(args[0], extra.addr() + 0x0c);
        assert_eq!(words(&l.e, block[0], 2), vec![0xaa, 0xbb]);
        l.swapping(true);
        l.run(b"XAPR", &[], &bytes_of(&[0xaa, 0xbb]), 8);
        let (_, block) = seen.borrow()[1].clone();
        assert_eq!(
            words(&l.e, block[0], 2),
            vec![0xaau32.swap_bytes(), 0xbbu32.swap_bytes()]
        );
        l.swapping(false);

        // XACR: the old record [reference id, delay, flags byte].
        let mut old = bytes_of(&[0x5a, 0x7f80_0001]);
        old.extend([0x09, 0, 0, 0]);
        l.run(b"XACR", &[], &old, 12);
        let (args, block) = seen.borrow()[2].clone();
        assert_eq!(args[0], extra.addr() + 0x0c);
        // The delay is a float, so a signalling NaN comes back quiet.
        assert_eq!(words(&l.e, block[0], 2), vec![0x5a, 0x7fc0_0001]);
        assert_eq!(l.e.mem.u8(extra.addr() + 0x14), 9);
        l.swapping(true);
        l.run(b"XACR", &[], &old, 12);
        let (_, block) = seen.borrow()[3].clone();
        assert_eq!(l.e.mem.u32(block[0]), 0x5au32.swap_bytes());
    }

    #[test]
    fn load_decal_adds_the_reference_with_its_intersection_and_normal() {
        let mut l = Loader::new();
        stub(&mut l.e, DECAL_DATA_INIT);
        let seen = capture_block(&mut l.e, ADD_DECAL_REF, 2, 3);
        let data = bytes_of(&[0xde, 1, 2, 3, 4, 5, 6]);
        l.run(b"XDCR", &[], &data, 0x1c);
        let (args, block) = seen.borrow()[0].clone();
        assert_eq!(args[..2], [l.list.addr(), 0xde]);
        assert_eq!(args[3] - args[2], 12);
        assert_eq!(block, vec![1, 2, 3]);
        assert_eq!(words(&l.e, args[3], 3), vec![4, 5, 6]);
        l.swapping(true);
        let log = l.run(b"XDCR", &[], &data, 0x1c);
        assert_eq!(calls_to(&log, SWAP_DWORD).len(), 7);
        assert_eq!(seen.borrow()[1].0[1], 0xdeu32.swap_bytes());
    }

    #[test]
    fn load_multibound_data_is_built_loaded_and_set() {
        let mut l = Loader::new();
        l.e.register(MULTIBOUND_MARKER_DATA_INIT, |_, a| returns(a[0]));
        stub(&mut l.e, MULTIBOUND_MARKER_DATA_LOAD);
        stub(&mut l.e, SET_MULTIBOUND_DATA);
        let log = l.run(b"XMBO", &[], &[], 0);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x0c]]);
        let bound = calls_to(&log, MULTIBOUND_MARKER_DATA_INIT)[0][0];
        assert_eq!(
            calls_to(&log, MULTIBOUND_MARKER_DATA_LOAD),
            vec![vec![bound, l.file.addr()]]
        );
        assert_eq!(
            calls_to(&log, SET_MULTIBOUND_DATA),
            vec![vec![l.list.addr(), bound]]
        );
    }

    #[test]
    fn load_skips_and_drops_the_chunks_nothing_uses() {
        let mut l = Loader::new();
        stub(&mut l.e, SKIP_CHUNK);
        let log = l.run(b"XPCI", &[], &[], 0);
        assert_eq!(calls_to(&log, SKIP_CHUNK), vec![vec![l.file.addr()]]);
        for name in [b"XCMT", b"XCET"] {
            let log = l.run(name, &[], &[0x33], 1);
            assert_eq!(
                calls_to(&log, GET_CHUNK_DATA).len(),
                1,
                "{}",
                String::from_utf8_lossy(name)
            );
        }
        // PTL: the plane record is read and dropped.
        let log = l.run(b"XPTL", &[], &[0u8; 0x24], 0x24);
        assert_eq!(calls_to(&log, GET_CHUNK_DATA).len(), 1);
        assert_eq!(chain_types(&l.e, l.list.cast()), Vec::<u8>::new());
        // An unknown chunk and no chunk at all do nothing.
        let log = l.run(b"ZZZZ", &[], &[], 0);
        assert_eq!(log.len(), 2);
        let log = l.run(&[0; 4], &[], &[], 0);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn load_navmesh_portal_chunk_goes_to_the_portal_loader() {
        let mut l = Loader::new();
        stub(&mut l.e, NAVMESH_PORTAL_INIT);
        stub(&mut l.e, NAVMESH_PORTAL_LOAD);
        let buffer: Ptr = Ptr::new(l.e.mem.alloc(0x30));
        let log = l.run_with_buffer(b"XNVP", &[], &[1, 2, 3, 4, 5, 6], 6, buffer);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![6]]);
        let load = calls_to(&log, NAVMESH_PORTAL_LOAD);
        assert_eq!(load.len(), 1);
        assert_eq!(load[0][1..3], [buffer.addr(), l.file.addr()]);
        let data = load[0][3];
        assert_eq!(l.e.mem.bytes(data, 6), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![data]]);
    }

    #[test]
    fn load_old_navmesh_portal_builds_a_type_5a_extra_data_on_the_stack() {
        let mut l = Loader::new();
        l.e.register(NAVMESH_PORTAL_COPY, |e, a| {
            let first = e.mem.u32(a[1]);
            let second = e.mem.u16(a[1] + 4);
            e.mem.set_u32(a[0], first);
            e.mem.set_u16(a[0] + 4, second);
            returns(a[0])
        });
        stub(&mut l.e, NAVMESH_PORTAL_EXTRA_DESTROY);
        let seen = capture_block(&mut l.e, SET_NAVMESH_PORTAL, 1, 5);
        let data = bytes_of(&[0x0102_0304, 0x0000_0506]);
        let log = l.run(b"XNDP", &[], &data, 8);
        let (args, block) = seen.borrow()[0].clone();
        assert_eq!(args[0], l.list.addr());
        // [vtable, type + padding, next, word, short]
        assert_eq!(block[0], 0x0101_4618);
        assert_eq!(block[1] & 0xff, 0x5a);
        assert_eq!(block[3], 0x0102_0304);
        assert_eq!(block[4] & 0xffff, 0x0506);
        assert_eq!(
            calls_to(&log, NAVMESH_PORTAL_EXTRA_DESTROY),
            vec![vec![args[1]]]
        );
        // Swapped for a file that needs it: the word and the short.
        l.swapping(true);
        l.run(b"XNDP", &[], &data, 8);
        let (_, block) = seen.borrow()[1].clone();
        assert_eq!(block[3], 0x0403_0201);
        assert_eq!(block[4] & 0xffff, 0x0605);
    }

    /// The save buffer's form, as `BGSSaveFormBuffer::GetForm` finds it.
    fn buffer_with_form(l: &mut Loader, form_type: Option<u8>) -> Ptr {
        let form = Rc::new(RefCell::new(0u32));
        if let Some(form_type) = form_type {
            let block = l.e.mem.alloc(0x20);
            l.e.mem.set_u8(block + 4, form_type);
            *form.borrow_mut() = block;
        }
        let shared = form.clone();
        l.e.register_double(SAVE_BUFFER_GET_FORM, move |_, _| returns(*shared.borrow()));
        l.e.register(FORM_TYPE, |e, a| returns(e.mem.u8(a[0] + 4) as u32));
        Ptr::new(l.e.mem.alloc(0x30))
    }

    #[test]
    fn load_reflector_and_water_chunks_are_skipped_for_forms_of_type_0x23() {
        for (form_type, loaded) in [(None, true), (Some(0x20u8), true), (Some(0x23), false)] {
            let mut l = Loader::new();
            let buffer = buffer_with_form(&mut l, form_type);
            stub(&mut l.e, SIMPLE_LIST_INIT);
            for setter in [ADD_REFLECTOR_REF, ADD_REFLECTED_REF, ADD_LIT_WATER_REF] {
                stub(&mut l.e, setter);
            }
            // XPWR: [reference id, flags]; both flags set both setters.
            let log = l.run_with_buffer(b"XPWR", &[], &bytes_of(&[0x77, 3]), 8, buffer);
            let reflector = calls_to(&log, ADD_REFLECTOR_REF);
            let reflected = calls_to(&log, ADD_REFLECTED_REF);
            assert_eq!(reflector.len(), loaded as usize);
            assert_eq!(reflected.len(), loaded as usize);
            if loaded {
                assert_eq!(reflector[0], vec![l.list.addr(), 0x77, 1]);
                assert_eq!(reflected[0], vec![l.list.addr(), 0x77, 1]);
            }
            // XLTW: one word.
            let log = l.run_with_buffer(b"XLTW", &[0x88], &[], 4, buffer);
            let water = calls_to(&log, ADD_LIT_WATER_REF);
            assert_eq!(water.len(), loaded as usize);
            if loaded {
                assert_eq!(water[0], vec![l.list.addr(), 0x88, 1]);
            }
        }
    }

    #[test]
    fn load_reflector_flags_pick_the_setters_and_swap_on_demand() {
        let mut l = Loader::new();
        let buffer = buffer_with_form(&mut l, None);
        stub(&mut l.e, SIMPLE_LIST_INIT);
        stub(&mut l.e, ADD_REFLECTOR_REF);
        stub(&mut l.e, ADD_REFLECTED_REF);
        for (flags, first, second) in [(0u32, 0, 0), (1, 1, 0), (2, 0, 1)] {
            let log = l.run_with_buffer(b"XPWR", &[], &bytes_of(&[0x77, flags]), 8, buffer);
            assert_eq!(calls_to(&log, ADD_REFLECTOR_REF).len(), first);
            assert_eq!(calls_to(&log, ADD_REFLECTED_REF).len(), second);
        }
        l.swapping(true);
        let log = l.run_with_buffer(
            b"XPWR",
            &[],
            &bytes_of(&[0x77, 1u32.swap_bytes()]),
            8,
            buffer,
        );
        assert_eq!(calls_to(&log, SWAP_DWORD).len(), 2);
        assert_eq!(
            calls_to(&log, ADD_REFLECTOR_REF),
            vec![vec![l.list.addr(), 0x77u32.swap_bytes(), 1]]
        );
    }

    #[test]
    fn load_primitive_is_created_from_the_record_and_added() {
        let mut l = Loader::new();
        let made = Rc::new(RefCell::new(0x9999u32));
        let shared = made.clone();
        let seen: Shared<Vec<(Vec<u32>, Vec<u32>)>> = Rc::default();
        let log = seen.clone();
        l.e.register_double(PRIMITIVE_CREATE, move |e, a| {
            log.borrow_mut().push((a.to_vec(), words(e, a[1], 8)));
            returns(*shared.borrow())
        });
        stub(&mut l.e, ADD_PRIMITIVE);
        // [radii x3][colour x4][type]
        let data = bytes_of(&[1, 2, 3, 4, 5, 6, 7, 3]);
        let log = l.run(b"XPRM", &[], &data, 0x20);
        let (args, record) = seen.borrow()[0].clone();
        assert_eq!(args[0], 3);
        assert_eq!(args[2] - args[1], 0x0c);
        assert_eq!(record, vec![1, 2, 3, 4, 5, 6, 7, 3]);
        assert_eq!(
            calls_to(&log, ADD_PRIMITIVE),
            vec![vec![l.list.addr(), 0x9999]]
        );
        // Nothing is added when the factory gives none.
        *made.borrow_mut() = 0;
        let log = l.run(b"XPRM", &[], &data, 0x20);
        assert!(calls_to(&log, ADD_PRIMITIVE).is_empty());
        l.swapping(true);
        l.run(b"XPRM", &[], &data, 0x20);
        assert_eq!(seen.borrow()[2].0[0], 3u32.swap_bytes());
    }

    #[test]
    fn load_occlusion_plane_is_built_from_the_record() {
        let mut l = Loader::new();
        l.e.register(ALLOCATE_ALIGNED, |e, a| returns(e.mem.alloc(a[0])));
        l.e.register(OCCLUSION_PLANE_INIT, |_, a| returns(a[0]));
        // MakeRotation(matrix, angle, x, y, z): put them in the first words.
        l.e.register(MATRIX_MAKE_ROTATION, |e, a| {
            for (index, word) in a[1..5].iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * index as u32, *word);
            }
            Ret::default()
        });
        stub(&mut l.e, SET_OCCLUSION_PLANE);
        // [extents x2][centre x3][axis x3][angle]
        let data = bytes_of(&[
            0xe1,
            0xe2,
            0xc1,
            0xc2,
            0xc3,
            1.0f32.to_bits(),
            2.0f32.to_bits(),
            3.0f32.to_bits(),
            0.5f32.to_bits(),
        ]);
        let log = l.run(b"XOCP", &[], &data, 0x24);
        let plane = calls_to(&log, SET_OCCLUSION_PLANE)[0][1];
        assert_eq!(calls_to(&log, ALLOCATE_ALIGNED), vec![vec![0xfc]]);
        assert_eq!(words(&l.e, plane + 0x14, 2), vec![0xe1, 0xe2]);
        assert_eq!(words(&l.e, plane + 8, 3), vec![0xc1, 0xc2, 0xc3]);
        // The rotation: angle first, then the axis.
        assert_eq!(
            words(&l.e, plane + 0x1c, 5),
            vec![
                0.5f32.to_bits(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                0
            ]
        );
        assert_eq!(l.e.mem.u8(plane + 0xe4), 1);
        assert_eq!(l.e.mem.u8(plane + 0xe5), 1);
        assert_eq!(l.e.mem.u8(plane + 0xe6), 0);
        assert_eq!(
            calls_to(&log, SET_OCCLUSION_PLANE),
            vec![vec![l.list.addr(), plane]]
        );
    }

    #[test]
    fn load_patrol_data_takes_its_float_by_pointer_and_the_package_action_follows() {
        let mut l = Loader::new();
        let seen: Shared<Vec<u32>> = Rc::default();
        let log = seen.clone();
        l.e.register_double(PATROL_REF_DATA_INIT, move |e, a| {
            log.borrow_mut().push(e.mem.u32(a[1]));
            returns(a[0])
        });
        stub(&mut l.e, SET_PATROL_REF_DATA);
        let log = l.run(b"XPRD", &[], &bytes_of(&[0x0102_0304]), 4);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        let created = calls_to(&log, PATROL_REF_DATA_INIT)[0][0];
        assert_eq!(
            calls_to(&log, SET_PATROL_REF_DATA),
            vec![vec![l.list.addr(), created]]
        );
        l.swapping(true);
        l.run(b"XPRD", &[], &bytes_of(&[0x0102_0304]), 4);
        assert_eq!(*seen.borrow(), vec![0x0102_0304, 0x0403_0201]);

        // XPPA: the PackageEventAction of the patrol data (at +4).
        let patrol: Ptr = Ptr::new(l.e.mem.alloc(0x20));
        l.e.register_double(GET_PATROL_REF_DATA, move |_, _| returns(patrol.addr()));
        stub(&mut l.e, SKIP_CHUNK);
        stub(&mut l.e, PACKAGE_EVENT_ACTION_LOAD);
        let log = l.run(b"XPPA", &[], &[], 0);
        assert_eq!(
            calls_to(&log, PACKAGE_EVENT_ACTION_LOAD),
            vec![vec![patrol.addr() + 4, l.file.addr()]]
        );
        assert_eq!(calls_to(&log, SKIP_CHUNK).len(), 1);
    }
    #[test]
    fn load_reference_tables_are_read_into_new_blocks() {
        let mut l = Loader::new();
        stub(&mut l.e, SET_OCCLUSION_PLANE_REF_DATA);
        stub(&mut l.e, SET_PORTAL_REF_DATA);
        stub(&mut l.e, SET_COLLISION_DATA);
        let data = bytes_of(&[1, 2, 3, 4]);
        let log = l.run(b"XORD", &[], &data, 16);
        let set = calls_to(&log, SET_OCCLUSION_PLANE_REF_DATA);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(words(&l.e, set[0][1], 4), vec![1, 2, 3, 4]);
        let log = l.run(b"XPOD", &[], &data, 8);
        let set = calls_to(&log, SET_PORTAL_REF_DATA);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![8]]);
        assert_eq!(words(&l.e, set[0][1], 2), vec![1, 2]);
        let log = l.run(b"XTRI", &[], &data, 4);
        let set = calls_to(&log, SET_COLLISION_DATA);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![4]]);
        assert_eq!(words(&l.e, set[0][1], 1), vec![1]);
        // A swapping file: four words, two words, one word.
        l.swapping(true);
        let log = l.run(b"XORD", &[], &data, 16);
        let set = calls_to(&log, SET_OCCLUSION_PLANE_REF_DATA);
        assert_eq!(l.e.mem.u32(set[0][1] + 12), 4u32.swap_bytes());
        let log = l.run(b"XPOD", &[], &data, 8);
        let set = calls_to(&log, SET_PORTAL_REF_DATA);
        assert_eq!(
            words(&l.e, set[0][1], 2),
            vec![1u32.swap_bytes(), 2u32.swap_bytes()]
        );
        let log = l.run(b"XTRI", &[], &data, 4);
        let set = calls_to(&log, SET_COLLISION_DATA);
        assert_eq!(l.e.mem.u32(set[0][1]), 1u32.swap_bytes());
    }

    #[test]
    fn load_room_data_reads_a_count_then_one_form_id_per_chunk() {
        let mut l = Loader::new();
        l.e.register(SIMPLE_LIST_INIT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            returns(a[0])
        });
        stub(&mut l.e, SKIP_CHUNK);
        let seen = capture_block(&mut l.e, LIST_ADD_HEAD, 1, 1);
        stub(&mut l.e, SET_ROOM_REF_DATA);
        let log = l.run(b"XRMR", &[0x0003_0002, 0x31, 0x32], &[], 4);
        let set = calls_to(&log, SET_ROOM_REF_DATA);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        let data = set[0][1];
        // The master flag byte is bits 16-23 of the first word.
        assert_eq!(l.e.mem.u8(data + 0x10), 3);
        let added: Vec<(u32, u32)> = seen
            .borrow()
            .iter()
            .map(|(args, block)| (args[0], block[0]))
            .collect();
        assert_eq!(added, vec![(data + 8, 0x31), (data + 8, 0x32)]);
        assert_eq!(calls_to(&log, SKIP_CHUNK).len(), 2);
    }

    #[test]
    fn load_type_92_chunks_create_the_extra_data_once() {
        let mut l = Loader::new();
        stub(&mut l.e, SET_SPECIAL_RENDER_WORD);
        let log = l.run(b"XSRF", &[0x1234], &[], 4);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(chain_types(&l.e, l.list.cast()), vec![0x92]);
        let extra: Ptr<BSExtraData> = l.e.get(l.list, ExtraDataList::pHead).cast();
        assert_eq!(
            calls_to(&log, SET_SPECIAL_RENDER_WORD),
            vec![vec![extra.addr(), 0x1234]]
        );
        let log = l.run(b"XSRD", &[0x7f80_0001], &[], 4);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(l.e.mem.u32(extra.addr() + 0x10), 0x7fc0_0001);
        // XSRD alone also creates it.
        let mut l = Loader::new();
        let log = l.run(b"XSRD", &[2.0f32.to_bits()], &[], 4);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        let extra: Ptr<BSExtraData> = l.e.get(l.list, ExtraDataList::pHead).cast();
        assert_eq!(l.e.mem.u32(extra.addr() + 0x10), 2.0f32.to_bits());
    }

    #[test]
    fn load_ignored_by_sandbox_sets_the_flag() {
        let mut l = Loader::new();
        stub(&mut l.e, SET_IGNORED_BY_SANDBOX);
        let log = l.run(b"XIBS", &[], &[], 0);
        assert_eq!(
            calls_to(&log, SET_IGNORED_BY_SANDBOX),
            vec![vec![l.list.addr(), 1]]
        );
    }

    // --- InitItem --------------------------------------------------------------

    use std::collections::{HashMap, HashSet};

    /// What the fix-up looks up and which casts fail.
    #[derive(Default)]
    struct InitState {
        /// Form by (load order) id.
        forms: HashMap<u32, u32>,
        /// `(form, target)` casts that give null.
        bad_casts: HashSet<(u32, u32)>,
        /// What the reference's virtual slot `+0x100` answers.
        is_actor: bool,
    }

    /// The id a file id becomes in the test (`AddCompileIndex` adds 0x1000 to
    /// every id but 0).
    fn compiled(id: u32) -> u32 {
        if id == 0 {
            0
        } else {
            id + 0x1000
        }
    }

    const ACTOR_CHECK: u32 = 0x0200_2000;
    const REFERENCE_VTABLE: u32 = 0x0300_0000;

    /// An engine with doubles for what `InitItem` calls on every extra data,
    /// the reference it fixes up and the state the doubles read.
    fn init_engine() -> (Engine, Shared<InitState>, Ptr) {
        let mut e = extra_engine();
        let state: Shared<InitState> = Rc::default();
        e.register(TES_FORM_GET_FILE, |_, _| returns(0xf11e));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], compiled(id));
            Ret::default()
        });
        let shared = state.clone();
        e.register_double(LOOKUP_FORM, move |_, a| {
            returns(shared.borrow().forms.get(&a[0]).copied().unwrap_or(0))
        });
        let shared = state.clone();
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            let failed = a[0] == 0 || shared.borrow().bad_casts.contains(&(a[0], a[3]));
            returns(if failed { 0 } else { a[0] })
        });
        let shared = state.clone();
        e.register_double(ACTOR_CHECK, move |_, _| {
            returns(shared.borrow().is_actor as u32)
        });
        let mut slots = vec![0; 0x41];
        slots[0x100 / 4] = ACTOR_CHECK;
        e.put_vtable(REFERENCE_VTABLE, &slots);
        stub(&mut e, LOG_MESSAGE);
        e.register(REFERENCE_EXTRA_LIST, |_, a| returns(a[0] + 0x44));
        e.register(FORM_ID, |e, a| returns(e.mem.u32(a[0] + 0x0c)));
        let reference: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u32(reference.addr(), REFERENCE_VTABLE);
        e.mem.set_u32(reference.addr() + 0x0c, 0xbeef);
        (e, state, reference)
    }

    /// A form object (0x100 bytes, so that `+0x44` is inside it) with an id.
    fn init_form(e: &mut Engine, state: &Shared<InitState>, file_id: u32) -> u32 {
        let form = e.mem.alloc(0x100);
        e.mem.set_u32(form + 0x0c, compiled(file_id));
        state.borrow_mut().forms.insert(compiled(file_id), form);
        form
    }

    fn run_init(e: &mut Engine, list: Ptr<ExtraDataList>, reference: Ptr) -> Log {
        e.call_log = Some(vec![]);
        e.call(0x0041_6be0, &args![list, reference]);
        e.call_log.take().unwrap()
    }

    #[test]
    fn init_item_resolves_the_simple_form_ids_and_removes_the_missing_ones() {
        for &(extra_type, target, message) in INIT_ITEM_FORM_FIXUPS {
            let (mut e, state, reference) = init_engine();
            let form = init_form(&mut e, &state, 5);
            let (list, extras) = list_with_payloads(&mut e, &[(extra_type, 5)]);
            let log = run_init(&mut e, list, reference);
            assert_eq!(
                e.mem.u32(extras[0].addr() + 0x0c),
                form,
                "type {extra_type:#x}"
            );
            assert_eq!(chain_types(&e, list.cast()), vec![extra_type]);
            assert!(calls_to(&log, LOG_MESSAGE).is_empty());
            // The file id is compiled first, then looked up, then cast.
            assert_eq!(calls_to(&log, LOOKUP_FORM), vec![vec![compiled(5)]]);
            let casts = calls_to(&log, RT_DYNAMIC_CAST);
            if target == 0 {
                assert!(casts.is_empty());
            } else {
                assert_eq!(casts, vec![vec![form, 0, RTTI_TES_FORM, target, 0]]);
            }
            // The file name call happens once, for the reference.
            assert_eq!(
                calls_to(&log, TES_FORM_GET_FILE),
                vec![vec![reference.addr(), 0xffff_ffff]]
            );

            // A missing form: logged with the compiled id, extra data removed.
            let (list, extras) = list_with_payloads(&mut e, &[(extra_type, 6)]);
            let log = run_init(&mut e, list, reference);
            assert_eq!(
                calls_to(&log, LOG_MESSAGE),
                vec![vec![message, compiled(6)]],
                "type {extra_type:#x}"
            );
            assert!(chain_types(&e, list.cast()).is_empty());
            assert_eq!(deleted(&log), vec![extras[0].addr()]);
            assert_eq!(e.mem.u32(extras[0].addr() + 0x0c), 0);

            // A form of the wrong class (the cast gives null): the same.
            if target != 0 {
                state.borrow_mut().bad_casts.insert((form, target));
                let (list, _) = list_with_payloads(&mut e, &[(extra_type, 5)]);
                let log = run_init(&mut e, list, reference);
                assert_eq!(
                    calls_to(&log, LOG_MESSAGE),
                    vec![vec![message, compiled(5)]]
                );
                assert!(chain_types(&e, list.cast()).is_empty());
            }
        }
    }

    #[test]
    fn init_item_walks_the_whole_list_even_when_an_extra_data_removes_itself() {
        let (mut e, state, reference) = init_engine();
        let form = init_form(&mut e, &state, 5);
        // 0x03 (missing, removed), 0x20 (not handled), 0x07 (found), 0x61.
        let (list, extras) =
            list_with_payloads(&mut e, &[(0x03, 99), (0x20, 1), (0x07, 5), (0x61, 2)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(chain_types(&e, list.cast()), vec![0x20, 0x07, 0x61]);
        assert_eq!(e.mem.u32(extras[2].addr() + 0x0c), form);
        assert_eq!(e.mem.u32(extras[1].addr() + 0x0c), 1);
        assert_eq!(e.mem.u32(extras[3].addr() + 0x0c), 2);
        assert_eq!(calls_to(&log, LOCK)[0], vec![EXTRA_CRIT_SECTION, 0]);
        assert_locked_and_released(&log);
        // An empty list only locks.
        let (empty, _) = list_with_payloads(&mut e, &[]);
        let log = run_init(&mut e, empty, reference);
        assert_eq!(calls_to(&log, LOCK).len(), 1);
        assert!(calls_to(&log, LOOKUP_FORM).is_empty());
    }

    #[test]
    fn init_item_logs_a_ragdoll_on_a_living_actor() {
        let (mut e, state, reference) = init_engine();
        e.register(REFERENCE_BASE_FORM, |_, a| returns(a[0] + 0x20));
        let base = reference.addr() + 0x20;
        let health = Rc::new(RefCell::new(5i32));
        let shared = health.clone();
        e.register_double(ACTOR_BASE_GET_HEALTH, move |_, _| {
            returns(*shared.borrow() as u32)
        });
        let (list, _) = list_with_payloads(&mut e, &[(0x14, 0)]);
        // Not an actor.
        let log = run_init(&mut e, list, reference);
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
        assert!(calls_to(&log, ACTOR_BASE_GET_HEALTH).is_empty());
        // An actor with health 5: logged with its id; the extra data stays.
        state.borrow_mut().is_actor = true;
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_RAGDOLL_ON_LIVE_ACTOR, 0xbeef]]
        );
        assert_eq!(calls_to(&log, ACTOR_BASE_GET_HEALTH), vec![vec![base]]);
        assert_eq!(chain_types(&e, list.cast()), vec![0x14]);
        // Health 0 or less: nothing logged.
        *health.borrow_mut() = 0;
        let log = run_init(&mut e, list, reference);
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
        *health.borrow_mut() = -4;
        let log = run_init(&mut e, list, reference);
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
        // No base form: not even asked.
        e.register(REFERENCE_BASE_FORM, |_, _| returns(0));
        *health.borrow_mut() = 9;
        let log = run_init(&mut e, list, reference);
        assert!(calls_to(&log, ACTOR_BASE_GET_HEALTH).is_empty());
    }

    #[test]
    fn init_item_lock_resolves_the_key_or_removes_the_lock() {
        let (mut e, state, reference) = init_engine();
        let key = init_form(&mut e, &state, 0x4b);
        let lock_data = e.mem.alloc(0x14);
        e.mem.set_u32(lock_data + 4, 0x4b);
        let (list, _) = list_with_payloads(&mut e, &[(0x2a, lock_data)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(e.mem.u32(lock_data + 4), key);
        assert_eq!(chain_types(&e, list.cast()), vec![0x2a]);
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![key, 0, RTTI_TES_FORM, RTTI_TES_KEY, 0]]
        );
        // A key that is not there.
        e.mem.set_u32(lock_data + 4, 0x4c);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_LOCK_KEY_MISSING, compiled(0x4c)]]
        );
        assert!(chain_types(&e, list.cast()).is_empty());
        // No key: nothing to do.
        e.mem.set_u32(lock_data + 4, 0);
        let (list, _) = list_with_payloads(&mut e, &[(0x2a, lock_data)]);
        let log = run_init(&mut e, list, reference);
        assert!(calls_to(&log, LOOKUP_FORM).is_empty());
        assert_eq!(chain_types(&e, list.cast()), vec![0x2a]);
    }

    #[test]
    fn init_item_teleport_data_decides_whether_the_extra_data_stays() {
        let (mut e, _state, reference) = init_engine();
        let accept = Rc::new(RefCell::new(true));
        let shared = accept.clone();
        e.register_double(DOOR_TELEPORT_DATA_INIT_ITEM, move |_, _| {
            returns(*shared.borrow() as u32)
        });
        let (list, _) = list_with_payloads(&mut e, &[(0x2b, 0x7e1)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, DOOR_TELEPORT_DATA_INIT_ITEM),
            vec![vec![0x7e1, reference.addr()]]
        );
        assert_eq!(chain_types(&e, list.cast()), vec![0x2b]);
        *accept.borrow_mut() = false;
        let log = run_init(&mut e, list, reference);
        assert!(chain_types(&e, list.cast()).is_empty());
        assert_eq!(deleted(&log).len(), 1);
    }

    #[test]
    fn init_item_map_marker_clears_a_hidden_travel_location_and_resolves_the_reputation() {
        let (mut e, _state, reference) = init_engine();
        e.set_global(DATA_HANDLER, 0x4444u32);
        let travel = Rc::new(RefCell::new((true, false, 0u32)));
        let shared = travel.clone();
        e.register_double(MAP_MARKER_GET_TRAVEL_LOC, move |_, _| {
            returns(shared.borrow().0 as u32)
        });
        let shared = travel.clone();
        e.register_double(MAP_MARKER_GET_VISIBLE, move |_, _| {
            returns(shared.borrow().1 as u32)
        });
        let shared = travel.clone();
        e.register_double(MAP_MARKER_GET_REPUTATION, move |_, _| {
            returns(shared.borrow().2)
        });
        stub(&mut e, MAP_MARKER_SET_TRAVEL_LOC);
        stub(&mut e, MAP_MARKER_SET_REPUTATION);
        e.register(DATA_HANDLER_GET_REPUTATION, |_, a| returns(a[1] + 0x100));
        let (list, _) = list_with_payloads(&mut e, &[(0x2c, 0x3a3)]);
        // A travel location that is not visible is cleared.
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, MAP_MARKER_SET_TRAVEL_LOC),
            vec![vec![0x3a3, 0]]
        );
        assert!(calls_to(&log, DATA_HANDLER_GET_REPUTATION).is_empty());
        // Visible: kept. No travel location: not even asked if visible.
        travel.borrow_mut().1 = true;
        let log = run_init(&mut e, list, reference);
        assert!(calls_to(&log, MAP_MARKER_SET_TRAVEL_LOC).is_empty());
        travel.borrow_mut().0 = false;
        let log = run_init(&mut e, list, reference);
        assert!(calls_to(&log, MAP_MARKER_GET_VISIBLE).is_empty());
        // A reputation id becomes the reputation.
        travel.borrow_mut().2 = 0x55;
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, DATA_HANDLER_GET_REPUTATION),
            vec![vec![0x4444, 0x55]]
        );
        assert_eq!(
            calls_to(&log, MAP_MARKER_SET_REPUTATION),
            vec![vec![0x3a3, 0x155]]
        );
    }

    #[test]
    fn init_item_enable_state_parent_checks_the_parent_chain() {
        let (mut e, state, reference) = init_engine();
        let parent = init_form(&mut e, &state, 0x9);
        stub(&mut e, ENABLE_PARENT_ADD_CHILD);
        let looped = Rc::new(RefCell::new(false));
        let shared = looped.clone();
        e.register_double(CHECK_ENABLE_PARENT_LOOP, move |_, _| {
            returns(!*shared.borrow() as u32)
        });
        let (list, _) = list_with_payloads(&mut e, &[(0x37, 9)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(chain_types(&e, list.cast()), vec![0x37]);
        assert_eq!(
            calls_to(&log, ENABLE_PARENT_ADD_CHILD),
            vec![vec![parent + 0x44, reference.addr()]]
        );
        assert_eq!(
            calls_to(&log, CHECK_ENABLE_PARENT_LOOP),
            vec![vec![reference.addr()]]
        );
        // The parent chain loops back: logged, removed.
        *looped.borrow_mut() = true;
        let (list, _) = list_with_payloads(&mut e, &[(0x37, 9)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_ENABLE_PARENT_LOOP]]
        );
        assert!(chain_types(&e, list.cast()).is_empty());
        // The reference is not a TESObjectREFR: the same.
        *looped.borrow_mut() = false;
        state
            .borrow_mut()
            .bad_casts
            .insert((reference.addr(), RTTI_TES_OBJECT_REFR));
        let (list, _) = list_with_payloads(&mut e, &[(0x37, 9)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_ENABLE_PARENT_LOOP]]
        );
        assert!(calls_to(&log, CHECK_ENABLE_PARENT_LOOP).is_empty());
        // The parent is missing.
        let (list, _) = list_with_payloads(&mut e, &[(0x37, 77)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_ENABLE_PARENT_MISSING, compiled(77)]]
        );
    }

    #[test]
    fn init_item_linked_ref_tells_the_target_about_the_reference() {
        let (mut e, state, reference) = init_engine();
        let target = init_form(&mut e, &state, 0x11);
        stub(&mut e, LINKED_REF_ADD_CHILD);
        let (list, _) = list_with_payloads(&mut e, &[(0x51, 0x11)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LINKED_REF_ADD_CHILD),
            vec![vec![target + 0x44, reference.addr()]]
        );
        assert_eq!(chain_types(&e, list.cast()), vec![0x51]);
        let (list, _) = list_with_payloads(&mut e, &[(0x51, 0x12)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_LINKED_REF_MISSING, compiled(0x12)]]
        );
        assert!(calls_to(&log, LINKED_REF_ADD_CHILD).is_empty());
    }

    /// An activate parent extra data with one entry per form id (0 for one
    /// that cannot be found); returns it with the entry data blocks.
    fn activate_extra(
        e: &mut Engine,
        ids: &[u32],
        flags: u8,
    ) -> (Ptr<ExtraDataList>, Ptr<BSExtraData>, Vec<u32>) {
        let (list, extras) = list_with_payloads(e, &[(0x53, 0)]);
        let extra = extras[0];
        let mut data_blocks = vec![];
        let mut node = extra.addr() + 0x0c;
        for (index, id) in ids.iter().enumerate() {
            let data = e.mem.alloc(8);
            e.mem.set_u32(data, *id);
            data_blocks.push(data);
            e.mem.set_u32(node, data);
            if index + 1 < ids.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        e.mem.set_u8(extra.addr() + 0x14, flags);
        (list, extra, data_blocks)
    }

    /// Doubles for the list helpers of the activate arm: removal after a
    /// node unlinks the next one; removal at the head pulls the next node's
    /// item and link up.
    fn list_removal_doubles(e: &mut Engine) {
        e.register(LIST_REMOVE_AFTER, |e, a| {
            let removed = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(removed + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let item = if next == 0 { 0 } else { e.mem.u32(next) };
            let after = if next == 0 { 0 } else { e.mem.u32(next + 4) };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(LIST_IS_EMPTY, |e, a| {
            returns((e.mem.u32(a[0] + 4) == 0 && e.mem.u32(a[0]) == 0) as u32)
        });
        e.register(STRING_LENGTH, |e, a| returns(e.mem.u16(a[0] + 4) as u32));
    }

    #[test]
    fn init_item_activate_ref_drops_the_entries_whose_reference_is_gone() {
        let (mut e, state, reference) = init_engine();
        list_removal_doubles(&mut e);
        stub(&mut e, ADD_ACTIVATE_REF_CHILD);
        let (ref_a, ref_c) = (
            init_form(&mut e, &state, 0xa),
            init_form(&mut e, &state, 0xc),
        );
        // Resolved, missing (after a resolved one), resolved.
        let (list, extra, blocks) = activate_extra(&mut e, &[0xa, 0xb, 0xc], 0);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, ADD_ACTIVATE_REF_CHILD),
            vec![
                vec![ref_a + 0x44, reference.addr()],
                vec![ref_c + 0x44, reference.addr()]
            ]
        );
        assert_eq!(
            calls_to(&log, LOG_MESSAGE)[0],
            vec![MESSAGE_ACTIVATE_REF_MISSING, compiled(0xb)]
        );
        // The entry's data block is freed; the others got their form.
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![blocks[1]]]);
        assert_eq!(e.mem.u32(blocks[0]), ref_a);
        assert_eq!(e.mem.u32(blocks[2]), ref_c);
        // Two entries remain in the list; the extra data stays.
        let first = extra.addr() + 0x0c;
        let second = e.mem.u32(first + 4);
        assert_eq!(e.mem.u32(second), blocks[2]);
        assert_eq!(e.mem.u32(second + 4), 0);
        assert_eq!(chain_types(&e, list.cast()), vec![0x53]);
        assert_eq!(calls_to(&log, LIST_REMOVE_AFTER).len(), 1);
        assert!(calls_to(&log, LIST_REMOVE_HEAD).is_empty());
    }

    #[test]
    fn init_item_activate_ref_removes_a_missing_head_and_an_empty_extra_data() {
        let (mut e, state, reference) = init_engine();
        list_removal_doubles(&mut e);
        stub(&mut e, ADD_ACTIVATE_REF_CHILD);
        let ref_c = init_form(&mut e, &state, 0xc);
        // The first entry is missing: removed at the head, the next one moves up.
        let (list, _, blocks) = activate_extra(&mut e, &[0xb, 0xc], 0);
        let log = run_init(&mut e, list, reference);
        assert_eq!(calls_to(&log, LIST_REMOVE_HEAD).len(), 1);
        assert!(calls_to(&log, LIST_REMOVE_AFTER).is_empty());
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![blocks[0]]]);
        assert_eq!(
            calls_to(&log, ADD_ACTIVATE_REF_CHILD),
            vec![vec![ref_c + 0x44, reference.addr()]]
        );
        assert_eq!(chain_types(&e, list.cast()), vec![0x53]);

        // Everything missing, no flags, no text: logged and removed.
        let (list, _, _) = activate_extra(&mut e, &[0xb], 0);
        let log = run_init(&mut e, list, reference);
        let messages: Vec<u32> = calls_to(&log, LOG_MESSAGE).iter().map(|a| a[0]).collect();
        assert_eq!(
            messages,
            vec![MESSAGE_ACTIVATE_REF_MISSING, MESSAGE_EMPTY_ACTIVATE_PARENT]
        );
        assert!(chain_types(&e, list.cast()).is_empty());

        // With a flag byte, or with a text override, it stays.
        let (list, _, _) = activate_extra(&mut e, &[0xb], 1);
        run_init(&mut e, list, reference);
        assert_eq!(chain_types(&e, list.cast()), vec![0x53]);
        let (list, extra, _) = activate_extra(&mut e, &[0xb], 0);
        e.mem.set_u16(extra.addr() + 0x18 + 4, 2);
        run_init(&mut e, list, reference);
        assert_eq!(chain_types(&e, list.cast()), vec![0x53]);
    }

    #[test]
    fn init_item_empty_list_extra_data_are_removed() {
        for (extra_type, init, message) in [
            (0x57u8, DECAL_REFS_INIT_ITEM, MESSAGE_EMPTY_DECALS),
            (0x66, REFLECTOR_REFS_INIT_ITEM, MESSAGE_EMPTY_REFLECTOR_REFS),
            (0x85, LIT_WATER_REFS_INIT_ITEM, MESSAGE_EMPTY_LIT_WATER),
        ] {
            let (mut e, _state, reference) = init_engine();
            stub(&mut e, init);
            e.register(LIST_IS_EMPTY, |e, a| {
                returns((e.mem.u32(a[0] + 4) == 0 && e.mem.u32(a[0]) == 0) as u32)
            });
            let (list, extras) = list_with_payloads(&mut e, &[(extra_type, 0)]);
            let log = run_init(&mut e, list, reference);
            assert_eq!(
                calls_to(&log, init),
                vec![vec![extras[0].addr(), reference.addr()]]
            );
            assert_eq!(calls_to(&log, LOG_MESSAGE), vec![vec![message]]);
            assert!(chain_types(&e, list.cast()).is_empty());
            // With an entry it stays.
            let (list, _) = list_with_payloads(&mut e, &[(extra_type, 0x1234)]);
            let log = run_init(&mut e, list, reference);
            assert!(calls_to(&log, LOG_MESSAGE).is_empty());
            assert_eq!(chain_types(&e, list.cast()), vec![extra_type]);
            assert_eq!(calls_to(&log, init).len(), 1);
        }
    }

    #[test]
    fn init_item_multibound_ref_may_not_be_the_reference_itself() {
        let (mut e, state, reference) = init_engine();
        let bound = init_form(&mut e, &state, 0x20);
        state
            .borrow_mut()
            .forms
            .insert(compiled(0x21), reference.addr());
        let (list, extras) = list_with_payloads(&mut e, &[(0x63, 0x20)]);
        run_init(&mut e, list, reference);
        assert_eq!(e.mem.u32(extras[0].addr() + 0x0c), bound);
        assert_eq!(chain_types(&e, list.cast()), vec![0x63]);
        // The reference itself: cleared, logged with both ids, removed.
        let (list, _) = list_with_payloads(&mut e, &[(0x63, 0x21)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_MULTIBOUND_REF_MISSING, compiled(0x21), 0xbeef]]
        );
        assert!(chain_types(&e, list.cast()).is_empty());
    }

    #[test]
    fn init_item_emittance_source_must_be_a_light_or_an_activator() {
        let (mut e, state, reference) = init_engine();
        for (form_type, kept) in [(0x1eu8, true), (0x37, true), (0x20, false)] {
            let form = init_form(&mut e, &state, 0x30);
            e.mem.set_u8(form + 4, form_type);
            e.register(FORM_TYPE, |e, a| returns(e.mem.u8(a[0] + 4) as u32));
            let (list, extras) = list_with_payloads(&mut e, &[(0x67, 0x30)]);
            let log = run_init(&mut e, list, reference);
            assert_eq!(chain_types(&e, list.cast()).len(), kept as usize);
            if kept {
                assert_eq!(e.mem.u32(extras[0].addr() + 0x0c), form);
                assert!(calls_to(&log, LOG_MESSAGE).is_empty());
            } else {
                assert_eq!(
                    calls_to(&log, LOG_MESSAGE),
                    vec![vec![MESSAGE_EMITTANCE_SOURCE_MISSING, compiled(0x30)]]
                );
            }
            // No class check by a cast for this type.
            assert!(calls_to(&log, RT_DYNAMIC_CAST).is_empty());
        }
        // A missing form is never asked for its type.
        let (list, _) = list_with_payloads(&mut e, &[(0x67, 0x31)]);
        let log = run_init(&mut e, list, reference);
        assert!(calls_to(&log, FORM_TYPE).is_empty());
        assert_eq!(calls_to(&log, LOG_MESSAGE).len(), 1);
    }

    #[test]
    fn init_item_radio_data_resolves_the_position_reference_only_logging_problems() {
        let (mut e, state, reference) = init_engine();
        let position = init_form(&mut e, &state, 0x40);
        let interior = Rc::new(RefCell::new(false));
        let shared = interior.clone();
        e.register_double(REFERENCE_GET_INTERIOR, move |_, _| {
            returns(*shared.borrow() as u32)
        });
        let (list, extras) = list_with_payloads(&mut e, &[(0x68, 0)]);
        // The position reference is at +0x18 (RADIO_DATA +0x0C).
        e.mem.set_u32(extras[0].addr() + 0x18, 0x40);
        let log = run_init(&mut e, list, reference);
        assert_eq!(e.mem.u32(extras[0].addr() + 0x18), position);
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
        assert_eq!(chain_types(&e, list.cast()), vec![0x68]);
        // An interior reference is logged; the extra data stays.
        *interior.borrow_mut() = true;
        e.mem.set_u32(extras[0].addr() + 0x18, 0x40);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_RADIO_POSITION_INTERIOR, compiled(0x40)]]
        );
        // A missing one is logged, the word becomes 0; still not removed.
        e.mem.set_u32(extras[0].addr() + 0x18, 0x41);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_RADIO_POSITION_MISSING, compiled(0x41)]]
        );
        assert_eq!(e.mem.u32(extras[0].addr() + 0x18), 0);
        assert_eq!(chain_types(&e, list.cast()), vec![0x68]);
        // No position reference: nothing is looked up.
        let log = run_init(&mut e, list, reference);
        assert!(calls_to(&log, LOOKUP_FORM).is_empty());
    }

    #[test]
    fn init_item_patrol_data_is_initialised_when_there_is_some() {
        let (mut e, _state, reference) = init_engine();
        let patrol = Rc::new(RefCell::new(0u32));
        let shared = patrol.clone();
        e.register_double(GET_PATROL_REF_DATA, move |_, _| returns(*shared.borrow()));
        stub(&mut e, PATROL_REF_DATA_INIT_ITEM);
        let (list, _) = list_with_payloads(&mut e, &[(0x6f, 0)]);
        let log = run_init(&mut e, list, reference);
        assert!(calls_to(&log, PATROL_REF_DATA_INIT_ITEM).is_empty());
        *patrol.borrow_mut() = 0x7a7a;
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, PATROL_REF_DATA_INIT_ITEM),
            vec![vec![0x7a7a, reference.addr()]]
        );
    }

    #[test]
    fn init_item_occlusion_plane_refs_keep_the_game_s_zero_id_quirk() {
        let (mut e, state, reference) = init_engine();
        // The id read from each table slot is discarded: id 0 is compiled
        // and looked up whatever the table says.
        let zero_form = init_form(&mut e, &state, 0);
        state.borrow_mut().forms.insert(0, zero_form);
        let table = e.mem.alloc(16);
        for index in 0..4 {
            e.mem.set_u32(table + 4 * index, 0x300 + index);
        }
        let (list, _) = list_with_payloads(&mut e, &[(0x76, table)]);
        // No plane in this list: logged once per slot.
        e.register(GET_OCCLUSION_PLANE, |_, _| returns(0));
        stub(&mut e, SET_LINKED_PLANE);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOOKUP_FORM),
            vec![vec![0], vec![0], vec![0], vec![0]]
        );
        let messages: Vec<u32> = calls_to(&log, LOG_MESSAGE).iter().map(|a| a[0]).collect();
        assert_eq!(messages, vec![MESSAGE_NO_OCCLUSION_PLANE; 4]);
        assert_eq!(words(&e, table, 4), vec![zero_form; 4]);
        assert_eq!(chain_types(&e, list.cast()), vec![0x76]);
    }

    #[test]
    fn init_item_occlusion_plane_refs_link_the_planes_of_the_targets() {
        let (mut e, state, reference) = init_engine();
        let zero_form = init_form(&mut e, &state, 0);
        state.borrow_mut().forms.insert(0, zero_form);
        let table = e.mem.alloc(16);
        let (list, _) = list_with_payloads(&mut e, &[(0x76, table)]);
        e.register(GET_OCCLUSION_PLANE, |_, a| returns(a[0] + 0x10));
        stub(&mut e, SET_LINKED_PLANE);
        run_init(&mut e, list, reference);
        let log = run_init(&mut e, list, reference);
        // The plane of the list is `this + 0x10` here; each slot links the
        // plane of its target's list (`target + 0x44 + 0x10`).
        let links = calls_to(&log, SET_LINKED_PLANE);
        assert_eq!(links.len(), 4);
        for (index, link) in links.iter().enumerate() {
            assert_eq!(link[0], list.addr() + 0x10);
            assert_eq!(link[1], index as u32);
            assert_eq!(link[2], zero_form + 0x44 + 0x10);
        }
    }

    #[test]
    fn init_item_portal_refs_fix_the_pair_and_link_both_targets() {
        let (mut e, state, reference) = init_engine();
        let (room_a, room_b) = (
            init_form(&mut e, &state, 0xa),
            init_form(&mut e, &state, 0xb),
        );
        stub(&mut e, PORTAL_ADD_REFERENCE);
        let table = e.mem.alloc(8);
        e.mem.set_u32(table, 0xa);
        e.mem.set_u32(table + 4, 0xb);
        let (list, _) = list_with_payloads(&mut e, &[(0x77, table)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(words(&e, table, 2), vec![room_a, room_b]);
        assert_eq!(
            calls_to(&log, PORTAL_ADD_REFERENCE),
            vec![
                vec![room_a + 0x44, reference.addr()],
                vec![room_b + 0x44, reference.addr()]
            ]
        );
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());

        // Both rooms the same: logged with both ids, the second one cleared.
        e.mem.set_u32(table, 0xa);
        e.mem.set_u32(table + 4, 0xa);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_PORTAL_ROOMS_SAME, 0xbeef, compiled(0xa)]]
        );
        assert_eq!(words(&e, table, 2), vec![room_a, 0]);
        assert_eq!(
            calls_to(&log, PORTAL_ADD_REFERENCE),
            vec![vec![room_a + 0x44, reference.addr()]]
        );

        // Both null: logged; nothing linked.
        e.mem.set_u32(table, 0);
        e.mem.set_u32(table + 4, 0);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_PORTAL_ROOMS_NULL, 0xbeef]]
        );
        assert!(calls_to(&log, PORTAL_ADD_REFERENCE).is_empty());

        // A room that is not there: logged and the extra data removed.
        e.mem.set_u32(table, 0xc);
        e.mem.set_u32(table + 4, 0xb);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE)[0],
            vec![MESSAGE_PORTAL_REF_MISSING, compiled(0xc)]
        );
        assert!(chain_types(&e, list.cast()).is_empty());
    }

    #[test]
    fn init_item_room_refs_replace_each_linked_room_in_the_list() {
        let (mut e, state, reference) = init_engine();
        let (room_a, room_b) = (
            init_form(&mut e, &state, 0xa),
            init_form(&mut e, &state, 0xb),
        );
        e.register(LIST_SET_ITEM, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let data = e.mem.alloc(0x20);
        let second = e.mem.alloc(8);
        e.mem.set_u32(data + 8, 0xa);
        e.mem.set_u32(data + 12, second);
        e.mem.set_u32(second, 0xb);
        let (list, _) = list_with_payloads(&mut e, &[(0x7b, data)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(e.mem.u32(data + 8), room_a);
        assert_eq!(e.mem.u32(second), room_b);
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
        assert_eq!(chain_types(&e, list.cast()), vec![0x7b]);
        // A room that cannot be found removes the extra data and stops.
        e.mem.set_u32(data + 8, 0xa);
        e.mem.set_u32(second, 0xc);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_ROOM_REF_MISSING, compiled(0xc)]]
        );
        assert!(chain_types(&e, list.cast()).is_empty());
        assert_eq!(calls_to(&log, LIST_SET_ITEM).len(), 1);
    }

    #[test]
    fn init_item_impact_swap_and_audio_markers() {
        let (mut e, state, reference) = init_engine();
        stub(&mut e, IMPACT_SWAP_INIT_ITEM);
        let (list, _) = list_with_payloads(&mut e, &[(0x8c, 0), (0x91, 0x55)]);
        let log = run_init(&mut e, list, reference);
        assert!(calls_to(&log, IMPACT_SWAP_INIT_ITEM).is_empty());
        let (list, _) = list_with_payloads(&mut e, &[(0x8c, 0x6a6a)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, IMPACT_SWAP_INIT_ITEM),
            vec![vec![0x6a6a, reference.addr()]]
        );

        // The audio marker's controller id becomes the controller's.
        let controller = init_form(&mut e, &state, 0x77);
        let stored = Rc::new(RefCell::new(0x77u32));
        let shared = stored.clone();
        e.register_double(AUDIO_MARKER_GET_CONTROLLER, move |_, _| {
            returns(*shared.borrow())
        });
        stub(&mut e, AUDIO_MARKER_SET_CONTROLLER);
        let (list, _) = list_with_payloads(&mut e, &[(0x90, 0x9a9a)]);
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![
                controller,
                0,
                RTTI_TES_FORM,
                RTTI_MEDIA_LOCATION_CONTROLLER,
                0
            ]]
        );
        assert_eq!(
            calls_to(&log, AUDIO_MARKER_SET_CONTROLLER),
            vec![vec![0x9a9a, compiled(0x77)]]
        );
        // A controller that is not there is only logged.
        *stored.borrow_mut() = 0x78;
        let log = run_init(&mut e, list, reference);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_CONTROLLER_MISSING, compiled(0x78)]]
        );
        assert!(calls_to(&log, AUDIO_MARKER_SET_CONTROLLER).is_empty());
        assert_eq!(chain_types(&e, list.cast()), vec![0x90]);
    }

    // -----------------------------------------------------------------------
    // Third batch: `004168a0`, `004181c0` to `00418a80`.

    /// `extra_engine` plus the page of the exe's read-only data that holds the
    /// -1.0 the health and charge getters default to.
    fn getter_engine() -> Engine {
        let mut e = extra_engine();
        e.map(0x0101_2000, 0x1000);
        e.set_global(MINUS_ONE, (-1.0f32).to_bits());
        e
    }

    /// A type that is not `extra_type`, for the extra data a getter must skip.
    fn other_type(extra_type: u8) -> u8 {
        if extra_type == 0x01 {
            0x02
        } else {
            0x01
        }
    }

    /// Checks a getter of `address` that returns the word at +0x0C of the
    /// extra data of `extra_type`: found among others, missing, empty list.
    fn check_word_getter(address: u32, extra_type: u8, default: u32) {
        let mut e = getter_engine();
        let decoy = other_type(extra_type);
        let (list, _) = list_with_payloads(&mut e, &[(decoy, 0x1111), (extra_type, 0x2222_3333)]);
        assert_eq!(e.call(address, &args![list]).u32(), 0x2222_3333);
        let (list, _) = list_with_payloads(&mut e, &[(decoy, 0x1111)]);
        assert_eq!(e.call(address, &args![list]).u32(), default);
        let (list, _) = list_with_payloads(&mut e, &[]);
        assert_eq!(e.call(address, &args![list]).u32(), default);
    }

    /// As `check_word_getter`, for a getter that returns a `float`.
    fn check_float_getter(address: u32, extra_type: u8, default: f32) {
        let mut e = getter_engine();
        let decoy = other_type(extra_type);
        let (list, _) =
            list_with_payloads(&mut e, &[(decoy, 0x1111), (extra_type, 2.5f32.to_bits())]);
        assert_eq!(e.call(address, &args![list]).f32(), 2.5);
        let (list, _) = list_with_payloads(&mut e, &[(decoy, 0x1111)]);
        assert_eq!(e.call(address, &args![list]).f32(), default);
        let (list, _) = list_with_payloads(&mut e, &[]);
        assert_eq!(e.call(address, &args![list]).f32(), default);
    }

    /// Checks a sound getter of `address` for `extra_type`: the handle is
    /// copied from +0x0C, or the empty handle is stored; `out` is returned.
    fn check_sound_getter(address: u32, extra_type: u8) {
        let mut e = getter_engine();
        let decoy = other_type(extra_type);
        let (list, extras) = list_with_payloads(&mut e, &[(decoy, 0x1111), (extra_type, 0x77)]);
        e.mem.set_u8(extras[1].addr() + 0x10, 1);
        e.mem.set_u32(extras[1].addr() + 0x14, 3);
        let out: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.write(out.addr(), &[0x5a; 0x10]);
        let result = e.call(address, &args![list, out]).ptr::<()>();
        assert_eq!(result, out);
        assert_eq!(
            (
                e.mem.u32(out.addr()),
                e.mem.u8(out.addr() + 4),
                e.mem.u32(out.addr() + 8)
            ),
            (0x77, 1, 3)
        );
        // Not copied: the bytes past the handle.
        assert_eq!(e.mem.u32(out.addr() + 0x0c), 0x5a5a_5a5a);

        let (list, _) = list_with_payloads(&mut e, &[(decoy, 0x1111)]);
        e.mem.write(out.addr(), &[0x5a; 0x10]);
        let result = e.call(address, &args![list, out]).ptr::<()>();
        assert_eq!(result, out);
        assert_eq!(
            (
                e.mem.u32(out.addr()),
                e.mem.u8(out.addr() + 4),
                e.mem.u32(out.addr() + 8)
            ),
            (0xffff_ffff, 0, 0)
        );
    }

    fn close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 1e-6,
            "{actual} is not close to {expected}"
        );
    }

    #[test]
    fn rotation_matrix_of_a_zero_angle_is_the_identity() {
        let mut e = extra_engine();
        let matrix: Ptr = Ptr::new(e.mem.alloc(0x28));
        e.mem.write(matrix.addr(), &[0xee; 0x28]);
        e.call(0x0041_68a0, &args![matrix, 0.0f32, 0.0f32, 0.0f32, 1.0f32]);
        let expected = [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        for (index, value) in expected.into_iter().enumerate() {
            assert_eq!(e.mem.f32(matrix.addr() + index as u32 * 4), value);
        }
        // The tenth word is not the matrix's.
        assert_eq!(e.mem.u32(matrix.addr() + 36), 0xeeee_eeee);
    }

    #[test]
    fn rotation_matrix_about_an_axis_follows_the_formula() {
        let mut e = extra_engine();
        let matrix: Ptr = Ptr::new(e.mem.alloc(0x28));
        let quarter_turn = std::f32::consts::FRAC_PI_2;
        // About z: x axis to y axis (row 1 holds z*s above the diagonal).
        e.call(
            0x0041_68a0,
            &args![matrix, quarter_turn, 0.0f32, 0.0f32, 1.0f32],
        );
        let about_z = [0.0f32, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        for (index, value) in about_z.into_iter().enumerate() {
            close(e.mem.f32(matrix.addr() + index as u32 * 4), value);
        }
        // About the diagonal (1, 1, 1)/sqrt(3) by half a turn.
        let axis = 1.0f32 / 3.0f32.sqrt();
        e.call(
            0x0041_68a0,
            &args![matrix, std::f32::consts::PI, axis, axis, axis],
        );
        let expected = [
            -1.0 / 3.0,
            2.0 / 3.0,
            2.0 / 3.0,
            2.0 / 3.0,
            -1.0 / 3.0,
            2.0 / 3.0,
            2.0 / 3.0,
            2.0 / 3.0,
            -1.0 / 3.0,
        ];
        for (index, value) in expected.into_iter().enumerate() {
            close(e.mem.f32(matrix.addr() + index as u32 * 4), value);
        }
    }

    #[test]
    fn linked_plane_setter_stores_into_the_array_at_0xec() {
        let mut e = extra_engine();
        let plane: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.call(0x0041_81c0, &args![plane, 2u32, 0xabcdu32]);
        assert_eq!(e.mem.u32(plane.addr() + 0xf4), 0xabcd);
        assert_eq!(e.mem.u32(plane.addr() + 0xec), 0);
        e.call(0x0041_81c0, &args![plane, 0u32, 7u32]);
        assert_eq!(e.mem.u32(plane.addr() + 0xec), 7);
    }

    #[test]
    fn base_form_getter_returns_what_the_form_buffer_getter_returns() {
        let mut e = extra_engine();
        e.register(SAVE_FORM_BUFFER_GET_FORM, |e, a| {
            returns(e.mem.u32(a[0] + 0x20))
        });
        let object: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(object.addr() + 0x20, 0x1357);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0041_81e0, &args![object]).u32(), 0x1357);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SAVE_FORM_BUFFER_GET_FORM),
            vec![vec![object.addr()]]
        );
    }

    #[test]
    fn anim_save_getter_returns_the_extra_data_itself() {
        let mut e = getter_engine();
        let (list, extras) = list_with_payloads(&mut e, &[(0x01, 1), (0x2d, 2)]);
        assert_eq!(e.call(0x0041_8200, &args![list]).u32(), extras[1].addr());
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1)]);
        assert_eq!(e.call(0x0041_8200, &args![list]).u32(), 0);
    }

    #[test]
    fn animation_getter_reads_type_0x10() {
        check_word_getter(0x0041_8220, 0x10, 0);
    }

    #[test]
    fn type_0x29_getter() {
        check_word_getter(0x0041_8250, 0x29, 0);
    }

    #[test]
    fn spell_effect_light_getter_reads_type_0x40() {
        check_word_getter(0x0041_8280, 0x40, 0);
    }

    #[test]
    fn type_0x2a_getter() {
        check_word_getter(0x0041_82b0, 0x2a, 0);
    }

    #[test]
    fn radio_data_getter_returns_the_record_address() {
        let mut e = getter_engine();
        let (list, extras) = list_with_payloads(&mut e, &[(0x01, 1), (0x68, 2)]);
        assert_eq!(
            e.call(0x0041_82e0, &args![list]).u32(),
            extras[1].addr() + 0x0c
        );
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1)]);
        assert_eq!(e.call(0x0041_82e0, &args![list]).u32(), 0);
    }

    #[test]
    fn radio_data_setter_removes_copies_or_adds() {
        // A null record removes the extra data (and deletes it).
        let mut e = getter_engine();
        let (list, extras) = list_with_payloads(&mut e, &[(0x01, 1), (0x68, 2)]);
        e.call_log = Some(vec![]);
        e.call(0x0041_8310, &args![list, Ptr::<()>::NULL]);
        let log = e.call_log.take().unwrap();
        assert_eq!(chain_types(&e, list.cast()), vec![0x01]);
        assert_eq!(deleted(&log), vec![extras[1].addr()]);

        // With an extra data of the type: the record is copied into it.
        let (list, extras) = list_with_payloads(&mut e, &[(0x68, 2), (0x01, 1)]);
        stub(&mut e, RADIO_DATA_COPY);
        let data: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.call_log = Some(vec![]);
        e.call(0x0041_8310, &args![list, data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, RADIO_DATA_COPY),
            vec![vec![extras[0].addr() + 0x0c, data.addr()]]
        );
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(chain_types(&e, list.cast()), vec![0x68, 0x01]);

        // Without one: a new extra data is built, filled and added.
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1)]);
        e.call_log = Some(vec![]);
        e.call(0x0041_8310, &args![list, data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x1c]]);
        let types = chain_types(&e, list.cast());
        assert_eq!(types.len(), 2);
        assert!(types.contains(&0x68));
        let new_extra = calls_to(&log, RADIO_DATA_COPY)[0][0] - 0x0c;
        assert_eq!(
            calls_to(&log, RADIO_DATA_COPY),
            vec![vec![new_extra + 0x0c, data.addr()]]
        );
        assert_eq!(e.mem.u8(new_extra + 4), 0x68);
        assert_eq!(e.mem.u32(new_extra), 0x0101_5138);
        assert!(flag_bytes(&e, list.cast())[0x68 >> 3] & (1 << (0x68 & 7)) != 0);
    }

    #[test]
    fn radio_data_extra_constructor_zeroes_the_record() {
        let mut e = extra_engine();
        let extra: Ptr = Ptr::new(e.mem.alloc(0x1c));
        e.mem.write(extra.addr(), &[0xcd; 0x1c]);
        e.call_log = Some(vec![]);
        let result = e.call(0x0041_83e0, &args![extra]).ptr::<()>();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, extra);
        assert_eq!(e.mem.u32(extra.addr()), 0x0101_5138);
        assert_eq!(e.mem.u8(extra.addr() + 4), 0x68);
        assert_eq!(e.mem.u32(extra.addr() + 8), 0);
        assert_eq!(e.mem.bytes(extra.addr() + 0x0c, 0x10), vec![0u8; 0x10]);
        assert_eq!(
            calls_to(&log, MEMSET),
            vec![vec![extra.addr() + 0x0c, 0, 0x10]]
        );
    }

    #[test]
    fn teleport_getter_reads_type_0x2b() {
        check_word_getter(0x0041_8460, 0x2b, 0);
    }

    #[test]
    fn map_marker_getter_reads_type_0x2c() {
        check_word_getter(0x0041_8490, 0x2c, 0);
    }

    #[test]
    fn audio_marker_getter_reads_type_0x90() {
        check_word_getter(0x0041_84c0, 0x90, 0);
    }

    #[test]
    fn audio_buoy_marker_getter_reads_type_0x91() {
        check_word_getter(0x0041_84f0, 0x91, 0);
    }

    #[test]
    fn container_changes_getter_reads_type_0x15() {
        check_word_getter(0x0041_8520, 0x15, 0);
    }

    #[test]
    fn original_reference_setter_updates_and_always_adds_a_new_extra_data() {
        let mut e = getter_engine();
        e.register(EXTRA_ORIGINAL_REFERENCE_INIT, |e, a| {
            e.mem.set_u8(a[0] + 4, 0x20);
            e.mem.set_u32(a[0] + 8, 0);
            e.mem.set_u32(a[0] + 0x0c, a[1]);
            returns(a[0])
        });
        // No extra data of the type yet: one is built and added.
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1)]);
        e.call_log = Some(vec![]);
        e.call(0x0041_8550, &args![list, 0x4444u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        let created = calls_to(&log, EXTRA_ORIGINAL_REFERENCE_INIT);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][1], 0x4444);
        assert_eq!(chain_types(&e, list.cast()).len(), 2);
        let first = find_extra(&mut e, list, 0x20);
        assert_eq!(first.addr(), created[0][0]);
        assert_eq!(e.mem.u32(first.addr() + 0x0c), 0x4444);

        // With one: its word is updated, and another one is added anyway.
        let (list, extras) = list_with_payloads(&mut e, &[(0x20, 0x1000), (0x01, 1)]);
        e.call(0x0041_8550, &args![list, 0x5555u32]);
        assert_eq!(e.mem.u32(extras[0].addr() + 0x0c), 0x5555);
        let types = chain_types(&e, list.cast());
        assert_eq!(types.iter().filter(|&&ty| ty == 0x20).count(), 2);
        assert_eq!(types.len(), 3);
    }

    #[test]
    fn remove_original_reference_extra_deletes_the_extra_data() {
        let mut e = getter_engine();
        let (list, extras) = list_with_payloads(&mut e, &[(0x01, 1), (0x20, 2)]);
        e.call_log = Some(vec![]);
        e.call(0x0041_8600, &args![list]);
        let log = e.call_log.take().unwrap();
        assert_eq!(chain_types(&e, list.cast()), vec![0x01]);
        assert_eq!(deleted(&log), vec![extras[1].addr()]);
        // Nothing to remove: nothing deleted.
        e.call_log = Some(vec![]);
        e.call(0x0041_8600, &args![list]);
        let log = e.call_log.take().unwrap();
        assert!(deleted(&log).is_empty());
        assert_eq!(chain_types(&e, list.cast()), vec![0x01]);
    }

    #[test]
    fn original_reference_getter_reads_type_0x20() {
        check_word_getter(0x0041_8630, 0x20, 0);
    }

    #[test]
    fn ownership_getter_reads_type_0x21() {
        check_word_getter(0x0041_8660, 0x21, 0);
    }

    #[test]
    fn global_getter_reads_type_0x22() {
        check_word_getter(0x0041_8690, 0x22, 0);
    }

    #[test]
    fn rank_getter_defaults_to_minus_one() {
        check_word_getter(0x0041_86c0, 0x23, 0xffff_ffff);
        let mut e = getter_engine();
        let (list, _) = list_with_payloads(&mut e, &[]);
        assert_eq!(e.call(0x0041_86c0, &args![list]).i32(), -1);
    }

    #[test]
    fn health_getter_defaults_to_the_float_at_01012054() {
        check_float_getter(0x0041_86f0, 0x25, -1.0);
        // The default is read from the exe's data, not built in.
        let mut e = getter_engine();
        e.set_global(MINUS_ONE, 7.5f32.to_bits());
        let (list, _) = list_with_payloads(&mut e, &[]);
        assert_eq!(e.call(0x0041_86f0, &args![list]).f32(), 7.5);
    }

    #[test]
    fn leveled_item_getter_returns_the_extra_data_itself() {
        let mut e = getter_engine();
        let (list, extras) = list_with_payloads(&mut e, &[(0x01, 1), (0x2f, 2)]);
        assert_eq!(e.call(0x0041_8720, &args![list]).u32(), extras[1].addr());
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1)]);
        assert_eq!(e.call(0x0041_8720, &args![list]).u32(), 0);
    }

    #[test]
    fn has_leveled_item_tests_the_type_bit() {
        let mut e = getter_engine();
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1), (0x2f, 2)]);
        assert!(e.call(0x0041_8750, &args![list]).bool());
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1)]);
        assert!(!e.call(0x0041_8750, &args![list]).bool());
    }

    #[test]
    fn count_getter_is_a_short_and_defaults_to_one() {
        let mut e = getter_engine();
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1), (0x24, 0x1234_0007)]);
        assert_eq!(e.call(0x0041_8770, &args![list]).u16(), 7);
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1)]);
        assert_eq!(e.call(0x0041_8770, &args![list]).u16(), 1);
    }

    #[test]
    fn charge_getter_defaults_to_minus_one() {
        check_float_getter(0x0041_87a0, 0x28, -1.0);
    }

    #[test]
    fn poison_getter_reads_type_0x3f() {
        check_word_getter(0x0041_87d0, 0x3f, 0);
    }

    #[test]
    fn script_getter_reads_type_0x0d() {
        check_word_getter(0x0041_8800, 0x0d, 0);
    }

    #[test]
    fn script_locals_getter_reads_the_word_at_0x10() {
        let mut e = getter_engine();
        let (list, extras) = list_with_payloads(&mut e, &[(0x01, 1), (0x0d, 0x5000)]);
        e.mem.set_u32(extras[1].addr() + 0x10, 0x6000);
        assert_eq!(e.call(0x0041_8830, &args![list]).u32(), 0x6000);
        assert_eq!(e.call(0x0041_8800, &args![list]).u32(), 0x5000);
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1)]);
        assert_eq!(e.call(0x0041_8830, &args![list]).u32(), 0);
    }

    #[test]
    fn scale_getter_defaults_to_one() {
        check_float_getter(0x0041_8860, 0x30, 1.0);
    }

    #[test]
    fn sound_getter_copies_the_handle_of_type_0x4f() {
        check_sound_getter(0x0041_8890, 0x4f);
    }

    #[test]
    fn empty_sound_handle_constructor() {
        let mut e = extra_engine();
        let handle: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.write(handle.addr(), &[0x33; 0x10]);
        let result = e.call(0x0041_88d0, &args![handle]).ptr::<()>();
        assert_eq!(result, handle);
        assert_eq!(e.mem.u32(handle.addr()), 0xffff_ffff);
        assert_eq!(e.mem.u8(handle.addr() + 4), 0);
        assert_eq!(e.mem.u32(handle.addr() + 8), 0);
        // The three bytes after the flag byte are not written.
        assert_eq!(e.mem.u8(handle.addr() + 5), 0x33);
        assert_eq!(e.mem.u32(handle.addr() + 0x0c), 0x3333_3333);
    }

    #[test]
    fn sound_handle_copy_constructor() {
        let mut e = extra_engine();
        let source: Ptr = Ptr::new(e.mem.alloc(0x10));
        let target: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(source.addr(), 0x99);
        e.mem.set_u8(source.addr() + 4, 1);
        e.mem.set_u8(source.addr() + 5, 0x77);
        e.mem.set_u32(source.addr() + 8, 4);
        e.mem.write(target.addr(), &[0x33; 0x10]);
        let result = e.call(0x0041_8900, &args![target, source]).ptr::<()>();
        assert_eq!(result, target);
        assert_eq!(e.mem.u32(target.addr()), 0x99);
        assert_eq!(e.mem.u8(target.addr() + 4), 1);
        assert_eq!(e.mem.u32(target.addr() + 8), 4);
        // Only the flag byte is copied, not the padding after it.
        assert_eq!(e.mem.u8(target.addr() + 5), 0x33);
    }

    #[test]
    fn creature_awake_sound_getter_reads_type_0x7d() {
        check_sound_getter(0x0041_8940, 0x7d);
    }

    #[test]
    fn type_0x8a_sound_getter() {
        check_sound_getter(0x0041_8980, 0x8a);
    }

    #[test]
    fn weapon_idle_sound_getter_reads_type_0x83() {
        check_sound_getter(0x0041_89c0, 0x83);
    }

    #[test]
    fn weapon_attack_sound_getter_reads_type_0x86() {
        check_sound_getter(0x0041_8a00, 0x86);
    }

    #[test]
    fn activate_loop_sound_getter_reads_type_0x87() {
        check_sound_getter(0x0041_8a40, 0x87);
    }

    #[test]
    fn type_0x1f_presence_test() {
        let mut e = getter_engine();
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1), (0x1f, 2)]);
        assert!(e.call(0x0041_8a80, &args![list]).bool());
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1)]);
        assert!(!e.call(0x0041_8a80, &args![list]).bool());
    }

    // -----------------------------------------------------------------------
    // Fourth batch: `00418ab0` to `0041a540`.

    /// The extra data constructors the setters call, with the type each sets.
    const CONSTRUCTORS: &[(u32, u8)] = &[
        (EXTRA_ANIM_INIT, 0x10),
        (EXTRA_STARTING_POSITION_INIT, 0x0f),
        (EXTRA_LOCK_INIT, 0x2a),
        (EXTRA_TELEPORT_INIT, 0x2b),
        (EXTRA_MAP_MARKER_INIT, 0x2c),
        (EXTRA_AUDIO_MARKER_INIT, 0x90),
        (EXTRA_AUDIO_BUOY_MARKER_INIT, 0x91),
        (EXTRA_ACTION_INIT, 0x0e),
        (EXTRA_CONTAINER_CHANGES_INIT, 0x15),
        (EXTRA_OWNERSHIP_INIT, 0x21),
        (EXTRA_GLOBAL_INIT, 0x22),
        (EXTRA_RANK_INIT, 0x23),
        (EXTRA_COUNT_INIT, 0x24),
        (EXTRA_HEALTH_INIT, 0x25),
        (EXTRA_USES_INIT, 0x26),
        (EXTRA_TIME_LEFT_INIT, 0x27),
        (EXTRA_CHARGE_INIT, 0x28),
        (EXTRA_SCALE_INIT, 0x30),
        (EXTRA_POISON_INIT, 0x3f),
        (EXTRA_ITEM_DROPPER_INIT, 0x46),
        (EXTRA_SCRIPT_INIT, 0x0d),
        (EXTRA_CREATURE_AWAKE_SOUND_INIT, 0x7d),
        (EXTRA_CREATURE_MOVEMENT_SOUND_INIT, 0x8a),
        (EXTRA_WEAPON_IDLE_SOUND_INIT, 0x83),
        (EXTRA_WEAPON_ATTACK_SOUND_INIT, 0x86),
    ];

    /// `getter_engine` plus: constructor doubles that set the vtable, the
    /// type and the argument words at +0x0C, +0x10 and +0x14; stubs for the
    /// destructors and `SetTargeted`; and the exe's `1.0` double.
    fn setter_engine() -> Engine {
        let mut e = getter_engine();
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        for &(address, extra_type) in CONSTRUCTORS {
            e.register_double(address, move |e, a| {
                e.mem.set_u32(a[0], VTABLE);
                e.mem.set_u8(a[0] + 4, extra_type);
                e.mem.set_u32(a[0] + 8, 0);
                for (index, word) in a[1..].iter().enumerate() {
                    e.mem.set_u32(a[0] + 0x0c + 4 * index as u32, *word);
                }
                returns(a[0])
            });
        }
        for address in [
            ANIM_SCALAR_DELETING_DESTRUCTOR,
            TELEPORT_DATA_DESTRUCTOR,
            MAP_MARKER_DATA_DESTRUCTOR,
            AUDIO_MARKER_DATA_DESTRUCTOR,
            AUDIO_BUOY_MARKER_DATA_SCALAR_DELETING_DESTRUCTOR,
            REFERENCE_SET_TARGETED,
            SOUND_HANDLE_DESTRUCTOR,
        ] {
            stub(&mut e, address);
        }
        e
    }

    /// Runs `call` on the engine with the call log on and returns the log.
    fn logged(e: &mut Engine, call: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        call(e);
        e.call_log.take().unwrap()
    }

    /// The size passed to `operator new` in the log, once.
    fn allocated_size(log: &Log) -> u32 {
        let sizes = calls_to(log, OPERATOR_NEW);
        assert_eq!(sizes.len(), 1);
        sizes[0][0]
    }

    /// The types in the list's chain, sorted (some types are added at the head
    /// of the chain, the others at the end).
    fn sorted_types(e: &Engine, list: Ptr<ExtraDataList>) -> Vec<u8> {
        let mut types = chain_types(e, list.cast());
        types.sort();
        types
    }

    /// The word at +0x0C of the extra data of `extra_type` in the list.
    fn payload_of(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> u32 {
        let extra = find_extra(e, list, extra_type);
        assert!(!extra.is_null());
        e.mem.u32(extra.addr() + 0x0c)
    }

    #[test]
    fn worn_getter_distinguishes_the_left_flag() {
        let mut e = setter_engine();
        let (right, _) = list_with_payloads(&mut e, &[(0x16, 0)]);
        assert!(e.call(0x0041_8ab0, &args![right, false]).bool());
        assert!(!e.call(0x0041_8ab0, &args![right, true]).bool());
        let (left, _) = list_with_payloads(&mut e, &[(0x17, 0)]);
        assert!(e.call(0x0041_8ab0, &args![left, false]).bool());
        assert!(e.call(0x0041_8ab0, &args![left, true]).bool());
        let (none, _) = list_with_payloads(&mut e, &[(0x01, 0)]);
        assert!(!e.call(0x0041_8ab0, &args![none, false]).bool());
        assert!(!e.call(0x0041_8ab0, &args![none, true]).bool());
    }

    #[test]
    fn can_not_wear_tests_the_type_bit() {
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 0), (0x3e, 0)]);
        assert!(e.call(0x0041_8b10, &args![list]).bool());
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 0)]);
        assert!(!e.call(0x0041_8b10, &args![list]).bool());
    }

    #[test]
    fn seed_getter_is_a_byte_and_defaults_to_ff() {
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 0), (0x31, 0x1122_3344)]);
        assert_eq!(e.call(0x0041_8b40, &args![list]).u8(), 0x44);
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 0)]);
        assert_eq!(e.call(0x0041_8b40, &args![list]).u8(), 0xff);
    }

    #[test]
    fn package_start_location_is_the_address_of_the_record() {
        let mut e = setter_engine();
        let (list, extras) = list_with_payloads(&mut e, &[(0x01, 0), (0x18, 0)]);
        assert_eq!(
            e.call(0x0041_8b70, &args![list]).u32(),
            extras[1].addr() + 0x0c
        );
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 0)]);
        assert_eq!(e.call(0x0041_8b70, &args![list]).u32(), 0);
    }

    #[test]
    fn weapon_mods_need_the_extra_data_and_a_nonzero_byte() {
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[(0x8d, 0x0000_0001)]);
        assert!(e.call(0x0041_8ba0, &args![list]).bool());
        // Only the byte counts.
        let (list, _) = list_with_payloads(&mut e, &[(0x8d, 0x0000_0100)]);
        assert!(!e.call(0x0041_8ba0, &args![list]).bool());
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 1)]);
        assert!(!e.call(0x0041_8ba0, &args![list]).bool());
    }

    #[test]
    fn nonzero_byte_test_of_an_extra_data() {
        let mut e = setter_engine();
        let (_, extras) = list_with_payloads(&mut e, &[(0x8d, 0x0000_0200), (0x8d, 0x0000_0002)]);
        assert!(!e.call(0x0041_8be0, &args![extras[0]]).bool());
        assert!(e.call(0x0041_8be0, &args![extras[1]]).bool());
    }

    #[test]
    fn weapon_mod_slot_active_masks_the_byte() {
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[(0x8d, 0b0101)]);
        assert!(e.call(0x0041_8c00, &args![list, 4u32]).bool());
        assert!(!e.call(0x0041_8c00, &args![list, 2u32]).bool());
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 0b0101)]);
        assert!(!e.call(0x0041_8c00, &args![list, 4u32]).bool());
    }

    #[test]
    fn anim_setter_builds_or_replaces_the_animation() {
        let mut e = setter_engine();
        // No extra data: built and added.
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 0)]);
        let mut result: Ptr = Ptr::NULL;
        let log = logged(&mut e, |e| {
            result = e.call(0x0041_8c40, &args![list, 0x7000u32]).ptr();
        });
        assert_eq!(allocated_size(&log), 0x10);
        assert_eq!(
            calls_to(&log, EXTRA_ANIM_INIT),
            vec![vec![result.addr(), 0x7000]]
        );
        assert_eq!(chain_types(&e, list.cast()), vec![0x01, 0x10]);
        assert_eq!(payload_of(&mut e, list, 0x10), 0x7000);
        // An existing one with an animation: the old one is destroyed first.
        let log = logged(&mut e, |e| {
            let again = e.call(0x0041_8c40, &args![list, 0x8000u32]).ptr::<()>();
            assert_eq!(again, result.cast());
        });
        assert_eq!(
            calls_to(&log, ANIM_SCALAR_DELETING_DESTRUCTOR),
            vec![vec![0x7000, 1]]
        );
        assert_eq!(payload_of(&mut e, list, 0x10), 0x8000);
        // An existing one without: nothing to destroy.
        let (list, _) = list_with_payloads(&mut e, &[(0x10, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_8c40, &args![list, 0x9000u32]);
        });
        assert!(calls_to(&log, ANIM_SCALAR_DELETING_DESTRUCTOR).is_empty());
        assert_eq!(payload_of(&mut e, list, 0x10), 0x9000);
    }

    #[test]
    fn starting_position_is_built_once() {
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[]);
        let mut first: Ptr = Ptr::NULL;
        let log = logged(&mut e, |e| {
            first = e.call(0x0041_8d50, &args![list, 0x5555u32]).ptr();
        });
        assert_eq!(allocated_size(&log), 0x24);
        assert_eq!(
            calls_to(&log, EXTRA_STARTING_POSITION_INIT),
            vec![vec![first.addr(), 0x5555]]
        );
        assert_eq!(chain_types(&e, list.cast()), vec![0x0f]);
        // With one in the list, it is returned and the argument is ignored.
        let log = logged(&mut e, |e| {
            let second = e.call(0x0041_8d50, &args![list, 0x6666u32]).ptr::<()>();
            assert_eq!(second, first.cast());
        });
        assert!(calls_to(&log, EXTRA_STARTING_POSITION_INIT).is_empty());
        assert_eq!(payload_of(&mut e, list, 0x0f), 0x5555);
    }

    #[test]
    fn lock_setter_deletes_the_old_lock_even_when_null() {
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_9050, &args![list, 0x7000u32]);
        });
        assert_eq!(allocated_size(&log), 0x10);
        assert_eq!(chain_types(&e, list.cast()), vec![0x2a]);
        assert_eq!(payload_of(&mut e, list, 0x2a), 0x7000);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        // Replaced: the old block is deleted.
        let log = logged(&mut e, |e| {
            e.call(0x0041_9050, &args![list, 0x8000u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![0x7000]]);
        assert_eq!(payload_of(&mut e, list, 0x2a), 0x8000);
        // A null old lock is still handed to `operator delete`.
        let (list, _) = list_with_payloads(&mut e, &[(0x2a, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_9050, &args![list, 0u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![0]]);
        assert_eq!(chain_types(&e, list.cast()), vec![0x2a]);
    }

    /// Checks one of the owned-pointer setters (`address`, extra data type
    /// `extra_type`, built by `construct`): nothing for null with no extra
    /// data; built for a value; the old value destroyed through
    /// `destroy` (called with its arguments) on replacement, not when the
    /// old one is null; deleted for null. Returns what the setter returned
    /// for the null value on an existing extra data.
    fn check_owned_pointer_setter(
        address: u32,
        extra_type: u8,
        construct: u32,
        destroy: impl Fn(&Log, u32),
    ) -> Ptr<BSExtraData> {
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 0)]);
        // Null and nothing there: nothing.
        let log = logged(&mut e, |e| {
            assert!(e.call(address, &args![list, 0u32]).ptr::<()>().is_null());
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(chain_types(&e, list.cast()), vec![0x01]);
        // A value and nothing there: built and added.
        let mut built: Ptr = Ptr::NULL;
        let log = logged(&mut e, |e| {
            built = e.call(address, &args![list, 0x7000u32]).ptr();
        });
        assert_eq!(allocated_size(&log), 0x10);
        assert_eq!(calls_to(&log, construct), vec![vec![built.addr(), 0x7000]]);
        assert_eq!(sorted_types(&e, list), vec![0x01, extra_type]);
        // Replaced: the old data is destroyed, the new one stored.
        let log = logged(&mut e, |e| {
            let again = e.call(address, &args![list, 0x8000u32]).ptr::<()>();
            assert_eq!(again, built.cast());
        });
        destroy(&log, 0x7000);
        assert_eq!(payload_of(&mut e, list, extra_type), 0x8000);
        // An old null pointer is not destroyed.
        let (other, _) = list_with_payloads(&mut e, &[(extra_type, 0)]);
        let log = logged(&mut e, |e| {
            e.call(address, &args![other, 0x9000u32]);
        });
        destroy(&log, 0);
        assert_eq!(payload_of(&mut e, other, extra_type), 0x9000);
        // Null: the extra data is deleted.
        let mut returned: Ptr<BSExtraData> = Ptr::NULL;
        let log = logged(&mut e, |e| {
            returned = e.call(address, &args![list, 0u32]).ptr();
        });
        assert_eq!(deleted(&log), vec![built.addr()]);
        assert_eq!(chain_types(&e, list.cast()), vec![0x01]);
        returned
    }

    /// `destroy` check of the setters that run a scalar deleting destructor
    /// of the file (`destructor` called, then the block deleted) on a
    /// non-null old pointer.
    fn destroyed_through(destructor: u32) -> impl Fn(&Log, u32) {
        move |log, old| {
            if old == 0 {
                assert!(calls_to(log, destructor).is_empty());
                assert!(calls_to(log, OPERATOR_DELETE).is_empty());
            } else {
                assert_eq!(calls_to(log, destructor), vec![vec![old]]);
                assert_eq!(calls_to(log, OPERATOR_DELETE), vec![vec![old]]);
            }
        }
    }

    #[test]
    fn teleport_setter_returns_the_deleted_extra_data() {
        let mut e = setter_engine();
        let returned = check_owned_pointer_setter(
            0x0041_9120,
            0x2b,
            EXTRA_TELEPORT_INIT,
            destroyed_through(TELEPORT_DATA_DESTRUCTOR),
        );
        // The result after a deletion is the (deleted) extra data, not null.
        assert!(!returned.is_null());
        let (list, extras) = list_with_payloads(&mut e, &[(0x2b, 0x44)]);
        let result = e.call(0x0041_9120, &args![list, 0u32]).ptr::<()>();
        assert_eq!(result, extras[0].cast());
    }

    #[test]
    fn teleport_data_destructor_deletes_on_flag() {
        let mut e = setter_engine();
        let block: Ptr = Ptr::new(e.mem.alloc(0x10));
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041_9220, &args![block, 1u32]).ptr::<()>(), block);
        });
        assert_eq!(
            calls_to(&log, TELEPORT_DATA_DESTRUCTOR),
            vec![vec![block.addr()]]
        );
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![block.addr()]]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_9220, &args![block, 0u32]);
        });
        assert_eq!(calls_to(&log, TELEPORT_DATA_DESTRUCTOR).len(), 1);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        // Only bit 0 counts.
        let log = logged(&mut e, |e| {
            e.call(0x0041_9220, &args![block, 2u32]);
        });
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn map_marker_setter_returns_null_after_deleting() {
        let returned = check_owned_pointer_setter(
            0x0041_9250,
            0x2c,
            EXTRA_MAP_MARKER_INIT,
            destroyed_through(MAP_MARKER_DATA_DESTRUCTOR),
        );
        assert!(returned.is_null());
    }

    #[test]
    fn map_marker_data_destructor_deletes_on_flag() {
        let mut e = setter_engine();
        let block: Ptr = Ptr::new(e.mem.alloc(0x10));
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041_9350, &args![block, 1u32]).ptr::<()>(), block);
        });
        assert_eq!(
            calls_to(&log, MAP_MARKER_DATA_DESTRUCTOR),
            vec![vec![block.addr()]]
        );
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![block.addr()]]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_9350, &args![block, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn audio_marker_setter_returns_null_after_deleting() {
        let returned = check_owned_pointer_setter(
            0x0041_9380,
            0x90,
            EXTRA_AUDIO_MARKER_INIT,
            destroyed_through(AUDIO_MARKER_DATA_DESTRUCTOR),
        );
        assert!(returned.is_null());
    }

    #[test]
    fn audio_marker_data_destructor_deletes_on_flag() {
        let mut e = setter_engine();
        let block: Ptr = Ptr::new(e.mem.alloc(0x10));
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041_9480, &args![block, 1u32]).ptr::<()>(), block);
        });
        assert_eq!(
            calls_to(&log, AUDIO_MARKER_DATA_DESTRUCTOR),
            vec![vec![block.addr()]]
        );
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![block.addr()]]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_9480, &args![block, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn audio_buoy_marker_setter_uses_the_scalar_deleting_destructor() {
        let returned = check_owned_pointer_setter(
            0x0041_94b0,
            0x91,
            EXTRA_AUDIO_BUOY_MARKER_INIT,
            |log, old| {
                let calls = calls_to(log, AUDIO_BUOY_MARKER_DATA_SCALAR_DELETING_DESTRUCTOR);
                if old == 0 {
                    assert!(calls.is_empty());
                } else {
                    assert_eq!(calls, vec![vec![old, 1]]);
                }
                // The setter does not delete the block itself.
                assert!(calls_to(log, OPERATOR_DELETE).is_empty());
            },
        );
        assert!(returned.is_null());
    }

    #[test]
    fn action_extra_data_is_built_once_without_arguments() {
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[]);
        let mut first: Ptr = Ptr::NULL;
        let log = logged(&mut e, |e| {
            first = e.call(0x0041_95b0, &args![list]).ptr();
        });
        assert_eq!(allocated_size(&log), 0x14);
        assert_eq!(calls_to(&log, EXTRA_ACTION_INIT), vec![vec![first.addr()]]);
        assert_eq!(chain_types(&e, list.cast()), vec![0x0e]);
        let log = logged(&mut e, |e| {
            let second = e.call(0x0041_95b0, &args![list]).ptr::<()>();
            assert_eq!(second, first.cast());
        });
        assert!(calls_to(&log, EXTRA_ACTION_INIT).is_empty());
    }

    #[test]
    fn container_changes_setter_ignores_null() {
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[]);
        e.call(0x0041_9650, &args![list, 0u32]);
        assert!(chain_types(&e, list.cast()).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x0041_9650, &args![list, 0x7000u32]);
        });
        assert_eq!(allocated_size(&log), 0x10);
        assert_eq!(chain_types(&e, list.cast()), vec![0x15]);
        assert_eq!(payload_of(&mut e, list, 0x15), 0x7000);
        e.call(0x0041_9650, &args![list, 0x8000u32]);
        assert_eq!(payload_of(&mut e, list, 0x15), 0x8000);
        // A null value leaves the existing extra data alone.
        let log = logged(&mut e, |e| {
            e.call(0x0041_9650, &args![list, 0u32]);
        });
        assert!(deleted(&log).is_empty());
        assert_eq!(payload_of(&mut e, list, 0x15), 0x8000);
    }

    /// Checks a setter (`address`, extra data type `extra_type`, built by
    /// `construct`) of a word where `none` removes the extra data.
    fn check_removing_setter(address: u32, extra_type: u8, construct: u32, none: u32) {
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 0)]);
        // `none` and nothing there: nothing is built.
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, none]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(chain_types(&e, list.cast()), vec![0x01]);
        // A value and nothing there: built and added.
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0x1234u32]);
        });
        assert_eq!(allocated_size(&log), 0x10);
        assert_eq!(calls_to(&log, construct).len(), 1);
        assert_eq!(calls_to(&log, construct)[0][1], 0x1234);
        assert_eq!(sorted_types(&e, list), vec![0x01, extra_type]);
        // Overwritten.
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0x5678u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(deleted(&log).is_empty());
        assert_eq!(payload_of(&mut e, list, extra_type), 0x5678);
        // `none`: deleted.
        let extra = find_extra(&mut e, list, extra_type);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, none]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert_eq!(chain_types(&e, list.cast()), vec![0x01]);
    }

    #[test]
    fn owner_setter_removes_on_null() {
        check_removing_setter(0x0041_9700, 0x21, EXTRA_OWNERSHIP_INIT, 0);
    }

    #[test]
    fn global_setter_removes_on_null() {
        check_removing_setter(0x0041_97d0, 0x22, EXTRA_GLOBAL_INIT, 0);
    }

    #[test]
    fn rank_setter_removes_on_minus_one() {
        check_removing_setter(0x0041_98a0, 0x23, EXTRA_RANK_INIT, 0xffff_ffff);
        // Zero is a rank like any other.
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[(0x23, 5)]);
        e.call(0x0041_98a0, &args![list, 0u32]);
        assert_eq!(payload_of(&mut e, list, 0x23), 0);
    }

    /// Checks a setter (`address`, `extra_type`, built by `construct` with
    /// `size` bytes) that stores the word `bits` at +0x0C and never removes.
    fn check_plain_setter(address: u32, extra_type: u8, construct: u32, size: u32, bits: u32) {
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 0)]);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, bits]);
        });
        assert_eq!(allocated_size(&log), size);
        assert_eq!(calls_to(&log, construct).len(), 1);
        assert_eq!(calls_to(&log, construct)[0][1], bits);
        assert_eq!(sorted_types(&e, list), vec![0x01, extra_type]);
        assert_eq!(payload_of(&mut e, list, extra_type), bits);
        // An existing extra data takes the value without a new one.
        let (list, _) = list_with_payloads(&mut e, &[(extra_type, 0x77)]);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, bits]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(payload_of(&mut e, list, extra_type), bits);
        // A zero value is stored, not a removal.
        e.call(address, &args![list, 0u32]);
        assert_eq!(chain_types(&e, list.cast()), vec![extra_type]);
        assert_eq!(payload_of(&mut e, list, extra_type), 0);
    }

    #[test]
    fn health_setter_stores_the_float() {
        check_plain_setter(
            0x0041_9970,
            0x25,
            EXTRA_HEALTH_INIT,
            0x10,
            75.5f32.to_bits(),
        );
    }

    #[test]
    fn float_setters_quiet_a_signalling_nan() {
        for (address, extra_type) in [
            (0x0041_9970u32, 0x25u8),
            (0x0041_9bb0, 0x27),
            (0x0041_9c60, 0x28),
        ] {
            let mut e = setter_engine();
            let (list, _) = list_with_payloads(&mut e, &[(extra_type, 0)]);
            e.call(address, &args![list, 0x7f80_0001u32]);
            assert_eq!(payload_of(&mut e, list, extra_type), 0x7fc0_0001);
            let (list, _) = list_with_payloads(&mut e, &[]);
            let log = logged(&mut e, |e| {
                e.call(address, &args![list, 0x7f80_0001u32]);
            });
            assert_eq!(
                calls_to(
                    &log,
                    CONSTRUCTORS.iter().find(|c| c.1 == extra_type).unwrap().0
                )[0][1],
                0x7fc0_0001
            );
        }
    }

    #[test]
    fn uses_setter_stores_a_byte() {
        let mut e = setter_engine();
        let (list, _) = list_with_payloads(&mut e, &[(0x26, 0xaabb_ccdd)]);
        e.call(0x0041_9a20, &args![list, 0x0000_1207u32]);
        // The byte is replaced, the rest of the word is not.
        assert_eq!(payload_of(&mut e, list, 0x26), 0xaabb_cc07);
        let (list, _) = list_with_payloads(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_9a20, &args![list, 0x0000_1207u32]);
        });
        assert_eq!(allocated_size(&log), 0x10);
        // The constructor gets the byte, zero-extended.
        assert_eq!(calls_to(&log, EXTRA_USES_INIT)[0][1], 0x07);
        assert_eq!(chain_types(&e, list.cast()), vec![0x26]);
        assert_eq!(payload_of(&mut e, list, 0x26), 0x07);
    }

    #[test]
    fn count_setter_removes_for_zero_and_one() {
        let mut e = setter_engine();
        // 0 and 1: nothing is built.
        for count in [0u32, 1] {
            let (list, _) = list_with_payloads(&mut e, &[(0x01, 0)]);
            let log = logged(&mut e, |e| {
                e.call(0x0041_9ad0, &args![list, count]);
            });
            assert!(calls_to(&log, OPERATOR_NEW).is_empty());
            assert_eq!(chain_types(&e, list.cast()), vec![0x01]);
        }
        // 0 and 1: an existing extra data is deleted.
        for count in [0u32, 1] {
            let (list, extras) = list_with_payloads(&mut e, &[(0x01, 0), (0x24, 9)]);
            let log = logged(&mut e, |e| {
                e.call(0x0041_9ad0, &args![list, count]);
            });
            assert_eq!(deleted(&log), vec![extras[1].addr()]);
            assert_eq!(chain_types(&e, list.cast()), vec![0x01]);
        }
        // Other values: built with the value zero-extended; a negative count
        // is a count.
        let (list, _) = list_with_payloads(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_9ad0, &args![list, -2i16]);
        });
        assert_eq!(allocated_size(&log), 0x10);
        assert_eq!(calls_to(&log, EXTRA_COUNT_INIT)[0][1], 0xfffe);
        assert_eq!(chain_types(&e, list.cast()), vec![0x24]);
        // Stored: only the short.
        let (list, _) = list_with_payloads(&mut e, &[(0x24, 0x1234_5678)]);
        e.call(0x0041_9ad0, &args![list, 5u32]);
        assert_eq!(payload_of(&mut e, list, 0x24), 0x1234_0005);
    }

    #[test]
    fn time_left_setter_stores_the_float() {
        check_plain_setter(
            0x0041_9bb0,
            0x27,
            EXTRA_TIME_LEFT_INIT,
            0x10,
            12.25f32.to_bits(),
        );
    }

    #[test]
    fn charge_setter_stores_the_float() {
        check_plain_setter(
            0x0041_9c60,
            0x28,
            EXTRA_CHARGE_INIT,
            0x10,
            300.0f32.to_bits(),
        );
    }

    #[test]
    fn poison_setter_stores_the_pointer() {
        check_plain_setter(0x0041_9d10, 0x3f, EXTRA_POISON_INIT, 0x10, 0x00a0_0000);
    }

    #[test]
    fn item_dropper_setter_marks_the_reference_targeted() {
        let mut e = setter_engine();
        // Built: the constructor, then `SetTargeted(true)` on the reference.
        let (list, _) = list_with_payloads(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_9dc0, &args![list, 0x7000u32]);
        });
        assert_eq!(allocated_size(&log), 0x10);
        assert_eq!(calls_to(&log, EXTRA_ITEM_DROPPER_INIT)[0][1], 0x7000);
        assert_eq!(
            calls_to(&log, REFERENCE_SET_TARGETED),
            vec![vec![0x7000, 1]]
        );
        assert_eq!(chain_types(&e, list.cast()), vec![0x46]);
        // Replaced.
        let log = logged(&mut e, |e| {
            e.call(0x0041_9dc0, &args![list, 0x8000u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(
            calls_to(&log, REFERENCE_SET_TARGETED),
            vec![vec![0x8000, 1]]
        );
        assert_eq!(payload_of(&mut e, list, 0x46), 0x8000);
        // Null: the extra data is deleted and nothing is targeted.
        let log = logged(&mut e, |e| {
            e.call(0x0041_9dc0, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log).len(), 1);
        assert!(calls_to(&log, REFERENCE_SET_TARGETED).is_empty());
        assert!(chain_types(&e, list.cast()).is_empty());
        // Null with nothing there: an extra data holding null is built.
        let log = logged(&mut e, |e| {
            e.call(0x0041_9dc0, &args![list, 0u32]);
        });
        assert_eq!(calls_to(&log, EXTRA_ITEM_DROPPER_INIT)[0][1], 0);
        assert!(calls_to(&log, REFERENCE_SET_TARGETED).is_empty());
        assert_eq!(chain_types(&e, list.cast()), vec![0x46]);
    }

    #[test]
    fn head_track_target_getter_reads_type_0x46() {
        check_word_getter(0x0041_9ea0, 0x46, 0);
    }

    #[test]
    fn script_setter_stores_or_builds_a_script_extra_data() {
        check_plain_setter(0x0041_9ed0, 0x0d, EXTRA_SCRIPT_INIT, 0x14, 0x00b0_0000);
    }

    #[test]
    fn script_locals_setter_needs_an_existing_script_extra_data() {
        let mut e = setter_engine();
        let (list, extras) = list_with_payloads(&mut e, &[(0x0d, 0x1111)]);
        e.call(0x0041_9f80, &args![list, 0x2222u32]);
        assert_eq!(e.mem.u32(extras[0].addr() + 0x10), 0x2222);
        assert_eq!(e.mem.u32(extras[0].addr() + 0x0c), 0x1111);
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_9f80, &args![list, 0x2222u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(chain_types(&e, list.cast()), vec![0x01]);
    }

    #[test]
    fn scale_setter_removes_for_exactly_one() {
        let mut e = setter_engine();
        // 1.0 and nothing there: nothing is built.
        let (list, _) = list_with_payloads(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_9fb0, &args![list, 1.0f32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // Another value: built.
        let log = logged(&mut e, |e| {
            e.call(0x0041_9fb0, &args![list, 1.5f32]);
        });
        assert_eq!(allocated_size(&log), 0x10);
        assert_eq!(calls_to(&log, EXTRA_SCALE_INIT)[0][1], 1.5f32.to_bits());
        assert_eq!(chain_types(&e, list.cast()), vec![0x30]);
        // Stored.
        e.call(0x0041_9fb0, &args![list, 0.25f32]);
        assert_eq!(payload_of(&mut e, list, 0x30), 0.25f32.to_bits());
        // 1.0: deleted.
        let extra = find_extra(&mut e, list, 0x30);
        let log = logged(&mut e, |e| {
            e.call(0x0041_9fb0, &args![list, 1.0f32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(chain_types(&e, list.cast()).is_empty());
        // The 1.0 is the double in the exe's data.
        e.mem.set_f64(ONE_DOUBLE, 2.0);
        let (list, _) = list_with_payloads(&mut e, &[(0x30, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_9fb0, &args![list, 2.0f32]);
        });
        assert_eq!(deleted(&log).len(), 1);
        // A NaN is not one: it is stored (quieted).
        let (list, _) = list_with_payloads(&mut e, &[(0x30, 0)]);
        e.call(0x0041_9fb0, &args![list, f32::from_bits(0x7f80_0001)]);
        assert_eq!(payload_of(&mut e, list, 0x30), 0x7fc0_0001);
    }

    #[test]
    fn sound_handle_comparisons_look_at_the_id_only() {
        let mut e = setter_engine();
        let a: Ptr = Ptr::new(e.mem.alloc(0x0c));
        let b: Ptr = Ptr::new(e.mem.alloc(0x0c));
        e.mem.set_u32(a.addr(), 7);
        e.mem.set_u8(a.addr() + 4, 1);
        e.mem.set_u32(a.addr() + 8, 3);
        e.mem.set_u32(b.addr(), 7);
        assert!(e.call(0x0041_a1f0, &args![a, b]).bool());
        assert!(!e.call(0x0041_a220, &args![a, b]).bool());
        e.mem.set_u32(b.addr(), 8);
        assert!(!e.call(0x0041_a1f0, &args![a, b]).bool());
        assert!(e.call(0x0041_a220, &args![a, b]).bool());
        // Both read the word through the helper, the argument first.
        let log = logged(&mut e, |e| {
            e.call(0x0041_a1f0, &args![a, b]);
        });
        assert_eq!(
            calls_to(&log, READ_WORD),
            vec![vec![b.addr()], vec![a.addr()]]
        );
    }

    #[test]
    fn empty_sound_handle_default_constructor() {
        let mut e = setter_engine();
        let handle: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.write(handle.addr(), &[0x33; 0x10]);
        let result = e.call(0x0041_a250, &args![handle]).ptr::<()>();
        assert_eq!(result, handle);
        assert_eq!(e.mem.u32(handle.addr()), 0xffff_ffff);
        assert_eq!(e.mem.u8(handle.addr() + 4), 0);
        assert_eq!(e.mem.u32(handle.addr() + 8), 0);
        assert_eq!(e.mem.u8(handle.addr() + 5), 0x33);
    }

    /// Checks a sound setter (`address`, `extra_type`, built by `construct`)
    /// with a non-empty handle, a replacement, an empty handle on an
    /// existing and on a missing extra data.
    fn check_sound_setter(address: u32, extra_type: u8, construct: u32) {
        let mut e = setter_engine();
        let handle: Ptr = Ptr::new(e.mem.alloc(0x0c));
        e.mem.set_u32(handle.addr(), 0x99);
        e.mem.set_u8(handle.addr() + 4, 1);
        e.mem.set_u32(handle.addr() + 8, 4);
        // Missing, non-empty: built with a by-value copy of the handle.
        let (list, _) = list_with_payloads(&mut e, &[(0x01, 0)]);
        let mut built = None;
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, handle]);
            built = Some(find_extra(e, list, extra_type));
        });
        let built = built.unwrap();
        assert_eq!(allocated_size(&log), 0x18);
        assert_eq!(
            calls_to(&log, construct),
            vec![vec![built.addr(), 0x99, 1, 4]]
        );
        // The empty handle that was compared with is destroyed.
        assert_eq!(calls_to(&log, SOUND_HANDLE_DESTRUCTOR).len(), 1);
        assert_eq!(sorted_types(&e, list), vec![0x01, extra_type]);
        // Existing, non-empty: the handle is copied, the padding is not.
        let (list, extras) = list_with_payloads(&mut e, &[(extra_type, 0x55)]);
        e.mem.set_u8(extras[0].addr() + 0x11, 0x77);
        e.call(address, &args![list, handle]);
        assert_eq!(
            (
                e.mem.u32(extras[0].addr() + 0x0c),
                e.mem.u8(extras[0].addr() + 0x10),
                e.mem.u8(extras[0].addr() + 0x11),
                e.mem.u32(extras[0].addr() + 0x14)
            ),
            (0x99, 1, 0x77, 4)
        );
        // Existing, empty (only the id counts): deleted.
        e.mem.set_u32(handle.addr(), 0xffff_ffff);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, handle]);
        });
        assert_eq!(deleted(&log), vec![extras[0].addr()]);
        assert_eq!(calls_to(&log, SOUND_HANDLE_DESTRUCTOR).len(), 1);
        assert!(chain_types(&e, list.cast()).is_empty());
        // Missing, empty: nothing.
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, handle]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(chain_types(&e, list.cast()).is_empty());
    }

    #[test]
    fn creature_awake_sound_setter() {
        check_sound_setter(0x0041_a090, 0x7d, EXTRA_CREATURE_AWAKE_SOUND_INIT);
    }

    #[test]
    fn creature_movement_sound_setter() {
        check_sound_setter(0x0041_a280, 0x8a, EXTRA_CREATURE_MOVEMENT_SOUND_INIT);
    }

    #[test]
    fn weapon_idle_sound_setter() {
        check_sound_setter(0x0041_a3e0, 0x83, EXTRA_WEAPON_IDLE_SOUND_INIT);
    }

    #[test]
    fn weapon_attack_sound_setter() {
        check_sound_setter(0x0041_a540, 0x86, EXTRA_WEAPON_ATTACK_SOUND_INIT);
    }
}

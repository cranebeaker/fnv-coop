//! `fallout shared/tesobjectcell.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds `TESObjectCELL` (409 functions to translate). The first
//! session translated the first 40 open or traced functions by address,
//! `005415b0` to `00544490`: the constructor and destructors, the form-record
//! `Save`/`Load`/`InitItem`/`Copy`/`Compare`/`CreateDuplicateForm` overrides, the
//! group-record helpers (`SavesBefore`, `BelongsInGroup`, `CreateGroupData`),
//! the exterior block keys and the flag setters. The second translated the
//! next 40, `005444c0` to `00545960`: the exterior and interior data
//! accessors (`INTERIOR_DATA` colours and distances, which a lighting
//! template can replace), and the `LOADED_CELL_DATA` block (its constructor,
//! destructor, the build that sorts a cell's references, and the handlers
//! that add and remove emittance, scripted, activating and water references
//! and the multibound nodes of marker references). The third translated the
//! next 40, `00545a30` to `00547610`: `AttachMultiBoundNodes`, the portal
//! graph, `Get3D` and `Load3D` (the node tree of a cell), six small functions
//! of the scene graph node class that the compiler emitted here, the
//! ownership accessors and checks, the land, region list, water height and
//! acoustic space accessors. The next session continues with the first
//! `open` function of this unit's range after `00547610`.
//!
//! ## Layout (PC build)
//!
//! The PC `TESForm` part is 0x18 bytes (the `TESFullName` component starts
//! there), 0x10 less than the Xbox one, so every Xbox offset up to `pNavMeshes`
//! is 0x10 lower here, and the fields from `iCriticalQueuedRefCount` on are
//! 0x20 higher (the PC build adds fields between `pNavMeshes` and the lock).
//! The fields below are named after the Xbox PDB field whose place and initial
//! value they have; offsets are the PC ones as read from the constructor.
//!
//! ## Helpers the translations call by address
//!
//! Almost every callee is a one-line accessor of the original source. The
//! ones used here are named by what their bodies do (the exe has no symbol
//! for them): `004077c0`, `00440d80`, `00460340`, `004013e0` test bits of the
//! form flags (`+0x08`); `00425fd0` is "cell is interior"
//! (`cCellFlags & 1`); `005516c0` tests form flag 0x400 (the cell's
//! persistent flag); `008256d0`, `006815c0` and `00726070` are the
//! `BSSimpleList` iterator (end test, item address, next node);
//! `009604f0` returns the list of references (`this + 0xAC`); `00559450`
//! reads a smart-pointer slot. The array calls on the local reference array
//! of `SaveReferences` are the `NiTArray` ones (`005594b0` constructor,
//! `0096ad30` SetSize, `00559490` SetGrowBy, `009938d0` Add, `00600bc0`
//! Compact, `009938b0` element count, `00877a30` element address,
//! `0096ae90` SetAt, `00559460` destructor).
//!
//! `__RTDynamicCast` (`00ec43fb`) is called as the CRT does, with the
//! RTTI type descriptors the exe passes; the descriptors are named after
//! what the code does with the result.
//!
//! x87: the one float computation (`InitItem`, rotation times -1.0) rounds
//! to `f32` once, as the code stores a `float`.
//!
//! Not translated: the C++ exception-unwinding frames (`FS:[0]` chains) of
//! the functions that have them (constructor, destructor, `SaveReferences`,
//! `Load`, `InitItem`, `CreateDuplicateForm`, `fn_00541a60`) and the stack
//! cookie check of `Load`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleList;
use crate::units::fallout_shared::tesobjectrefr::TESObjectREFR;

/// A four-character chunk tag as the record reader compares it (the bytes
/// in file order, read as a little-endian word).
pub(crate) const fn tag(name: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*name)
}

pub(crate) const DATA: u32 = tag(b"DATA");
pub(crate) const EXTERIOR_COORDINATES: u32 = tag(b"XCLC");
pub(crate) const INTERIOR_LIGHTING: u32 = tag(b"XCLL");
pub(crate) const WATER_HEIGHT: u32 = tag(b"XCLW");
pub(crate) const LIGHTING_TEMPLATE_ID: u32 = tag(b"LTMP");
pub(crate) const INHERITANCE_FLAGS: u32 = tag(b"LNAM");
pub(crate) const WATER_NOISE_TEXTURE: u32 = tag(b"XNAM");
pub(crate) const FULL_NAME: u32 = tag(b"FULL");
pub(crate) const EDITOR_ID: u32 = tag(b"EDID");
pub(crate) const OBJECT_BOUNDS: u32 = tag(b"OBND");
pub(crate) const IMPACT_SWAP_FIRST: u32 = tag(b"IMPF");
pub(crate) const IMPACT_SWAP_SECOND: u32 = tag(b"IMPS");

/// The chunk tags `Load` hands to `ExtraDataList::Load`.
pub(crate) const EXTRA_DATA_TAGS: [u32; 76] = [
    tag(b"XPPA"),
    tag(b"XRGB"),
    tag(b"XGLB"),
    tag(b"XLMB"),
    tag(b"XAMC"),
    tag(b"XLOC"),
    tag(b"XMRC"),
    tag(b"XRAD"),
    tag(b"XSED"),
    tag(b"XRGD"),
    tag(b"XLOD"),
    tag(b"XPOD"),
    tag(b"XAPD"),
    tag(b"XORD"),
    tag(b"XPRD"),
    tag(b"XSRD"),
    tag(b"XUSE"),
    tag(b"XSRF"),
    tag(b"XCHG"),
    tag(b"XTRG"),
    tag(b"XPCI"),
    tag(b"XEMI"),
    tag(b"XTRI"),
    tag(b"XRNK"),
    tag(b"AMRK"),
    tag(b"MMRK"),
    tag(b"XMRK"),
    tag(b"XEDL"),
    tag(b"XTEL"),
    tag(b"XSOL"),
    tag(b"XPSL"),
    tag(b"XPTL"),
    tag(b"RNAM"),
    tag(b"XCCM"),
    tag(b"XLCM"),
    tag(b"XCIM"),
    tag(b"XTIM"),
    tag(b"XCLM"),
    tag(b"XPRM"),
    tag(b"XRTM"),
    tag(b"XPSN"),
    tag(b"XOWN"),
    tag(b"XEZN"),
    tag(b"XMBO"),
    tag(b"XRDO"),
    tag(b"XCMO"),
    tag(b"XROO"),
    tag(b"XATO"),
    tag(b"XMBP"),
    tag(b"XOCP"),
    tag(b"XNDP"),
    tag(b"XCLP"),
    tag(b"XHLP"),
    tag(b"XESP"),
    tag(b"XNVP"),
    tag(b"XMBR"),
    tag(b"XACR"),
    tag(b"XDCR"),
    tag(b"XLKR"),
    tag(b"XCLR"),
    tag(b"XRMR"),
    tag(b"XAPR"),
    tag(b"XPWR"),
    tag(b"XCAS"),
    tag(b"XIBS"),
    tag(b"XRDS"),
    tag(b"XHRS"),
    tag(b"XACT"),
    tag(b"XCET"),
    tag(b"XHLT"),
    tag(b"XWLT"),
    tag(b"XAMT"),
    tag(b"XCMT"),
    tag(b"XCNT"),
    tag(b"XCWT"),
    tag(b"XLTW"),
];

// Vtables the constructor and destructor store.
pub(crate) const CELL_VTABLE: u32 = 0x0102_e9b4;
pub(crate) const CELL_FULL_NAME_VTABLE: u32 = 0x0102_e9a0;

// Form and cell accessors (see the header).
/// `cFormType` (`+0x04`) of a form.
pub(crate) const FORM_TYPE: u32 = 0x0040_1170;
/// `iFormFlags & 0x4000`.
pub(crate) const FORM_FLAG_4000: u32 = 0x0040_77c0;
/// `iFormFlags & 0x20`.
pub(crate) const FORM_FLAG_20: u32 = 0x0044_0d80;
/// `iFormFlags & 0x2`.
pub(crate) const FORM_FLAG_2: u32 = 0x0046_0340;
/// `iFormFlags & 0x8`.
pub(crate) const FORM_FLAG_8: u32 = 0x0040_13e0;
/// `iFormFlags` (`+0x08`).
pub(crate) const FORM_FLAGS: u32 = 0x0044_ddc0;
/// `iFormID` (`+0x0C`).
pub(crate) const FORM_ID: u32 = 0x0084_e3a0;
/// `TESObjectCELL` "is interior": `cCellFlags & 1`.
pub(crate) const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
/// The cell's persistent flag: `iFormFlags & 0x400`.
pub(crate) const CELL_PERSISTENT_FLAG: u32 = 0x0055_16c0;
/// `TESObjectCELL::GetWorldSpace` (Xbox PDB): `pWorldSpace` for an exterior
/// cell, null for an interior one.
pub(crate) const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
/// The exterior data (`pCellData`) of an exterior cell, null for an interior.
pub(crate) const CELL_EXTERIOR_DATA: u32 = 0x0054_45d0;
/// The interior data (`pCellData`) of an interior cell, null for an exterior.
pub(crate) const CELL_INTERIOR_DATA: u32 = 0x0054_4600;
/// `TESObjectCELL::CreateCellData` (Xbox PDB).
pub(crate) const CELL_CREATE_CELL_DATA: u32 = 0x0054_4630;
/// `TESObjectCELL::GetDataX` / `GetDataY` (Xbox PDB).
pub(crate) const CELL_GET_DATA_X: u32 = 0x0054_4c30;
pub(crate) const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
/// Reads / writes the lighting template pointer (`+0xD8`).
pub(crate) const CELL_GET_LIGHTING_TEMPLATE: u32 = 0x0055_8b40;
pub(crate) const CELL_SET_LIGHTING_TEMPLATE: u32 = 0x0055_8b60;
/// Reads the lighting-template inheritance flags (`+0xDC`).
pub(crate) const CELL_GET_INHERITANCE_FLAGS: u32 = 0x0049_7280;
/// Writes the cell flags byte (`+0x24`).
pub(crate) const CELL_SET_CELL_FLAGS: u32 = 0x0046_1310;
/// The cell's `ExtraDataList` (`this + 0x28`).
pub(crate) const CELL_EXTRA_DATA_LIST: u32 = 0x0046_10d0;
/// `TESObjectCELL::SetLand` (Xbox PDB).
pub(crate) const CELL_SET_LAND: u32 = 0x0054_70a0;
/// `TESObjectCELL::AddReference` (Xbox PDB): `(reference, 0)`.
pub(crate) const CELL_ADD_REFERENCE: u32 = 0x0054_8230;
/// Sets `fWaterHeight` (argument: the new value).
pub(crate) const CELL_SET_WATER_HEIGHT: u32 = 0x0054_7440;
/// Replaces the cell's `NavMeshArray` (`+0x64`), deleting the old one.
pub(crate) const CELL_SET_NAV_MESHES: u32 = 0x0055_7760;
/// Unidentified helpers the destructor and the loader call.
pub(crate) const CELL_CLEAR_STATE: u32 = 0x0054_5c10;
pub(crate) const CELL_RELEASE_STATE: u32 = 0x0054_cd20;
pub(crate) const CELL_CLEAR_REFERENCES: u32 = 0x0054_5030;
pub(crate) const CELL_ERASE: u32 = 0x0055_10b0;
pub(crate) const CELL_SET_LOADED_MASTER_DATA: u32 = 0x0054_def0;
/// The cell-ref lock (`this + 0x80`): enter (with the name string) and leave.
pub(crate) const LOCK_ENTER: u32 = 0x0040_fbf0;
pub(crate) const LOCK_LEAVE: u32 = 0x0040_fba0;
/// `BSSimpleList` constructor, destructor body and clear (on a stack list)
/// and the list-iterator accessors.
pub(crate) const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
pub(crate) const LIST_CLEAR: u32 = 0x0047_0470;
pub(crate) const LIST_DESTRUCT: u32 = 0x0046_ffb0;
pub(crate) const LIST_PUSH_FRONT: u32 = 0x005a_e3d0;
pub(crate) const CELL_REFERENCE_LIST: u32 = 0x0096_04f0;
pub(crate) const LIST_IS_END: u32 = 0x0082_56d0;
pub(crate) const LIST_ITEM_ADDRESS: u32 = 0x0068_15c0;
pub(crate) const LIST_NEXT: u32 = 0x0072_6070;
/// Smart-pointer slot reader / constructor / assignment / destructor.
pub(crate) const SLOT_GET: u32 = 0x0055_9450;
pub(crate) const SLOT_CONSTRUCT: u32 = 0x0063_3c90;
pub(crate) const SLOT_ASSIGN: u32 = 0x0066_b0d0;
pub(crate) const SLOT_RELEASE: u32 = 0x0045_cec0;
/// Heap allocation / free through the memory manager.
pub(crate) const ALLOCATE: u32 = 0x0040_1000;
pub(crate) const DEALLOCATE: u32 = 0x0040_1030;
/// `memcpy(dest, source, count)` wrapper, CRT `memcmp` and `strlen`.
pub(crate) const MEMORY_COPY: u32 = 0x0040_1460;
pub(crate) const CRT_MEMCMP: u32 = 0x00ec_4835;
pub(crate) const CRT_STRLEN: u32 = 0x00ec_6130;
/// `__RTDynamicCast(object, 0, from, to, 0)`.
pub(crate) const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `printf`-style diagnostic output (a stub that returns 0 in this build).
pub(crate) const DEBUG_PRINT: u32 = 0x005b_5e40;
/// `MessageHandler::IncDisableWarningCount(bool)` (Xbox PDB).
pub(crate) const INC_DISABLE_WARNING_COUNT: u32 = 0x0043_b2b0;

// TESForm record helpers.
pub(crate) const FORM_CONSTRUCT: u32 = 0x0048_3370;
pub(crate) const FORM_DESTRUCT: u32 = 0x0048_3630;
/// Writes `cFormType` (`+0x04`); the constructor passes 0x39.
pub(crate) const FORM_SET_FORM_TYPE: u32 = 0x004f_15a0;
/// Sets (non-zero argument) or clears `iFormFlags` bit 8.
pub(crate) const FORM_SET_FLAG_8: u32 = 0x0048_4ab0;
pub(crate) const FORM_START_FORM: u32 = 0x0048_55a0;
pub(crate) const FORM_CLOSE_FORM: u32 = 0x0048_5680;
pub(crate) const FORM_ADD_CHUNK_BYTE: u32 = 0x0048_58f0;
pub(crate) const FORM_ADD_CHUNK_WORD: u32 = 0x0048_5910;
pub(crate) const FORM_ADD_CHUNK_DATA: u32 = 0x0048_5990;
pub(crate) const FORM_ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
pub(crate) const FORM_LOAD_FORM: u32 = 0x0048_5110;
pub(crate) const FORM_GET_FILE: u32 = 0x0048_4e60;
pub(crate) const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
pub(crate) const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
pub(crate) const FORM_COMPARE_FALLBACK: u32 = 0x0048_4020;
pub(crate) const FORM_BELONGS_IN_GROUP_FALLBACK: u32 = 0x0048_4150;
pub(crate) const FORM_BELONGS_IN_GROUP_INTERIOR: u32 = 0x0048_54e0;
pub(crate) const FORM_DUPLICATE: u32 = 0x0048_67a0;
pub(crate) const FORM_SAVE_TO_FILE: u32 = 0x0048_3d20;
pub(crate) const FORM_LOOK_UP: u32 = 0x0048_39c0;
pub(crate) const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
pub(crate) const FORM_ID_IN_WORLD_SPACE: u32 = 0x0048_5be0;
/// `TESFullName::Save` / loader, and the full-name accessors.
pub(crate) const FULL_NAME_SAVE: u32 = 0x0048_7010;
pub(crate) const FULL_NAME_LOAD: u32 = 0x0048_7050;
pub(crate) const FULL_NAME_LENGTH: u32 = 0x0048_cee0;
pub(crate) const TEXT_GET_STRING: u32 = 0x0040_8da0;
pub(crate) const FULL_NAME_SET: u32 = 0x0048_9100;
/// `TESTexture` constructor and destructor (the water noise texture).
pub(crate) const TEXTURE_CONSTRUCT: u32 = 0x0048_e270;
pub(crate) const TEXTURE_DESTRUCT: u32 = 0x0048_e2e0;
/// `ExtraDataList` constructor, destructor, `InitItem`, `Save`, `Load`,
/// `CopyList`, `CompareList`, `RemoveNonPersistentCellData`,
/// `GetRegionList`, `SetNorthRotation`, impact-swap data accessors.
pub(crate) const EXTRA_LIST_CONSTRUCT: u32 = 0x0041_0360;
pub(crate) const EXTRA_LIST_DESTRUCT: u32 = 0x0041_03b0;
pub(crate) const EXTRA_LIST_INIT_ITEM: u32 = 0x0041_6be0;
pub(crate) const EXTRA_LIST_SAVE: u32 = 0x0041_2970;
pub(crate) const EXTRA_LIST_LOAD: u32 = 0x0041_44a0;
pub(crate) const EXTRA_LIST_COPY_LIST: u32 = 0x0041_1ec0;
pub(crate) const EXTRA_LIST_COMPARE_LIST: u32 = 0x0041_27e0;
pub(crate) const EXTRA_LIST_REMOVE_NON_PERSISTENT: u32 = 0x0041_20b0;
pub(crate) const EXTRA_LIST_GET_REGION_LIST: u32 = 0x0041_bce0;
pub(crate) const EXTRA_LIST_SET_NORTH_ROTATION: u32 = 0x0042_1a70;
pub(crate) const EXTRA_LIST_GET_IMPACT_SWAP: u32 = 0x0041_c460;
pub(crate) const EXTRA_LIST_SET_IMPACT_SWAP: u32 = 0x0041_c390;
/// Impact swap data: constructor (size 0x15c), `Save`, loader.
pub(crate) const IMPACT_SWAP_CONSTRUCT: u32 = 0x0058_efd0;
pub(crate) const IMPACT_SWAP_SAVE: u32 = 0x0058_f080;
pub(crate) const IMPACT_SWAP_LOAD: u32 = 0x0058_f180;
pub(crate) const REGION_LIST_RELEASE: u32 = 0x004f_6640;
/// Endian swap of the cell data in memory: flag, exterior swap and interior
/// swap.
pub(crate) const ENDIAN_SWAP_ENABLED: u32 = 0x0040_1500;
pub(crate) const SWAP_EXTERIOR_DATA: u32 = 0x0046_2230;
pub(crate) const SWAP_INTERIOR_DATA: u32 = 0x0052_6790;

// TESFile accessors used by Save and Load.
pub(crate) const FILE_IS_MASTER: u32 = 0x0047_1c20;
pub(crate) const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
pub(crate) const FILE_GET_CHUNK: u32 = 0x0047_26b0;
pub(crate) const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
pub(crate) const FILE_GET_CHUNK_DATA: u32 = 0x0047_27f0;
pub(crate) const FILE_GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
pub(crate) const FILE_ADD_FORM: u32 = 0x0047_2fe0;
pub(crate) const FILE_START_GROUP: u32 = 0x0047_3310;
pub(crate) const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
pub(crate) const FILE_SWAP_ENDIAN: u32 = 0x0040_1680;
pub(crate) const FILE_MASTER_DATA: u32 = 0x0046_7bb0;

// NavMeshArray (`+0x64`) and its smart-pointer elements.
pub(crate) const NAV_MESH_ARRAY_CONSTRUCT: u32 = 0x0046_94e0;
pub(crate) const NAV_MESH_ARRAY_COUNT: u32 = 0x0062_0b80;
pub(crate) const NAV_MESH_ARRAY_GET: u32 = 0x0046_4f60;
pub(crate) const NAV_MESH_ARRAY_ADD: u32 = 0x0046_9500;
/// `NiPointer<NavMesh>` helpers: constructor from a raw pointer, default
/// constructor, assignment, copy constructor, destructor, raw pointer read.
pub(crate) const NAV_POINTER_FROM_RAW: u32 = 0x0046_4fc0;
pub(crate) const NAV_POINTER_CONSTRUCT: u32 = 0x0042_fb00;
pub(crate) const NAV_POINTER_ASSIGN: u32 = 0x0042_f4c0;
pub(crate) const NAV_POINTER_COPY: u32 = 0x0042_fa20;
pub(crate) const NAV_POINTER_RELEASE: u32 = 0x0042_fa40;
pub(crate) const NAV_POINTER_GET: u32 = 0x0045_8b50;
/// Returns the `NavMeshArray` pointer `this + 0x64` (`pNavMeshes`).
pub(crate) const CELL_NAV_MESHES: u32 = 0x0070_ec90;

// Save / load of the whole game (InitItem's tail).
pub(crate) const GAME_LOADER_FLAG_SETTER: u32 = 0x0046_23f0;
pub(crate) const GAME_LOAD_FORM: u32 = 0x0084_95d0;
pub(crate) const GAME_LOAD_NOTIFY: u32 = 0x0085_8730;
pub(crate) const GAME_FINISH_A: u32 = 0x0085_9690;
pub(crate) const GAME_FINISH_B: u32 = 0x0085_8af0;
pub(crate) const GAME_FINISH_C: u32 = 0x0085_f850;
pub(crate) const GAME_FLAG_READ: u32 = 0x0047_c850;
pub(crate) const GAME_FLAG_TEST: u32 = 0x0087_27b0;
pub(crate) const GAME_FLAG_WRITE: u32 = 0x0045_34f0;
pub(crate) const REFERENCE_PARENT_CELL: u32 = 0x008d_6f30;
pub(crate) const SAVE_FORM_BUFFER_GET_FORM: u32 = 0x007a_f430;
pub(crate) const REFERENCE_ROTATION: u32 = 0x0043_0830;
/// The thread's error count (`TLS + 0x2B8`): getter and setter.
pub(crate) const ERROR_COUNT_GET: u32 = 0x0046_e8a0;
pub(crate) const ERROR_COUNT_SET: u32 = 0x004f_ffe0;
/// `TESObjectREFR::GetRefPersists` / `SetRefPersists(bool)` (Xbox PDB).
pub(crate) const REFERENCE_GET_PERSISTS: u32 = 0x0056_53d0;
pub(crate) const REFERENCE_SET_PERSISTS: u32 = 0x0056_5480;
/// `TESObjectLAND::SetCell` (Xbox PDB).
pub(crate) const LAND_SET_CELL: u32 = 0x0053_40e0;
/// `TESWorldSpace::ReleaseCell` (Xbox PDB).
pub(crate) const WORLD_SPACE_RELEASE_CELL: u32 = 0x0058_7760;
/// Packs two 16-bit block coordinates into one key.
pub(crate) const PACK_BLOCK_COORDINATES: u32 = 0x0058_7410;

/// `TESFullName` constructor and destructor (the component at `this + 0x18`).
pub(crate) const CELL_FULL_NAME_CONSTRUCT: u32 = 0x0040_2d00;
pub(crate) const CELL_FULL_NAME_DESTRUCT: u32 = 0x0059_1300;
/// Byte at `+0x61D` of the loader singleton.
pub(crate) const LOADER_STATE_FLAG_61D: u32 = 0x0042_26e0;
/// `NavMeshArray` destructor body: clears the elements, then the base.
pub(crate) const NAV_MESH_ARRAY_CLEAR: u32 = 0x0069_bca0;
pub(crate) const NAV_MESH_ARRAY_BASE_DESTRUCT: u32 = 0x0042_f830;
/// `NiTArray` operations on the local array of `SaveReferences`.
pub(crate) const NI_ARRAY_CONSTRUCT: u32 = 0x0055_94b0;
pub(crate) const NI_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
pub(crate) const NI_ARRAY_SET_GROW_BY: u32 = 0x0055_9490;
pub(crate) const NI_ARRAY_ADD: u32 = 0x0099_38d0;
pub(crate) const NI_ARRAY_COMPACT: u32 = 0x0060_0bc0;
pub(crate) const NI_ARRAY_COUNT: u32 = 0x0099_38b0;
pub(crate) const NI_ARRAY_ELEMENT_ADDRESS: u32 = 0x0087_7a30;
pub(crate) const NI_ARRAY_SET_AT: u32 = 0x0096_ae90;
pub(crate) const NI_ARRAY_DESTRUCT: u32 = 0x0055_9460;
/// Tests cell flag 0x20 (`cCellFlags`).
pub(crate) const CELL_FLAG_20: u32 = 0x0050_2180;

// Statics.
/// `TESObjectCELL` statics `InitStatics` and `fn_00541c00` use.
pub(crate) const STATIC_SETTINGS_SOURCE: u32 = 0x011f_426c;
pub(crate) const STATIC_FLAG: u32 = 0x011c_a08c;
pub(crate) const STATIC_POINTER_SLOT: u32 = 0x011c_a0d8;
pub(crate) const STATIC_OBJECT: u32 = 0x011c_a088;
pub(crate) const STATIC_SETTING_BLOCK: u32 = 0x011c_a094;
pub(crate) const STATIC_RESET_WORD: u32 = 0x011c_c54c;
pub(crate) const STATIC_SETTING_BUILDER: u32 = 0x0055_4010;
pub(crate) const STATIC_SETTING_GET: u32 = 0x0040_3e20;
pub(crate) const STATIC_SETTING_SET: u32 = 0x004e_d780;
pub(crate) const STATIC_RELEASE_HELPER: u32 = 0x0062_42f0;
/// `2048.0` as a `double` and as a `float`.
pub(crate) const SETTING_LIMIT_DOUBLE: u32 = 0x0101_6968;
pub(crate) const SETTING_LIMIT_FLOAT: u32 = 0x0101_8bfc;
/// `FLT_MAX` (the default water height).
pub(crate) const DEFAULT_WATER_HEIGHT: u32 = 0x0101_6970;
/// `-1.0` as a `double` (the north rotation factor).
pub(crate) const MINUS_ONE: u32 = 0x0101_a6b0;
/// `"TESObjectCELL::CellRefLockEnter()"`.
pub(crate) const LOCK_NAME: u32 = 0x0102_eaec;

// Globals read by the record code.
/// The `GRUP` tag word of group headers, and the form ids of the persistent
/// world / interior groups.
pub(crate) const GROUP_TAG: u32 = 0x0118_7020;
pub(crate) const EXTERIOR_PARENT_LABEL: u32 = 0x0118_7314;
pub(crate) const INTERIOR_PARENT_LABEL: u32 = 0x0118_72b4;
pub(crate) const PLAYER_HELPER_FORM: u32 = 0x011c_a254;
/// Singleton pointers: the object `004226e0` / `00542d20` read, the game
/// loader (`0046 23f0`, `008495d0`) and the save-game object.
pub(crate) const LOADER_STATE_POINTER: u32 = 0x011c_3f2c;
pub(crate) const GAME_LOADER_POINTER: u32 = 0x011d_df38;
pub(crate) const SAVE_GAME_POINTER: u32 = 0x011d_e45c;

// Messages.
pub(crate) const CELL_NAME_TOO_LONG: u32 = 0x0102_ece8;
pub(crate) const INIT_ERRORS_EXTERIOR_WORLD: u32 = 0x0102_ec58;
pub(crate) const INIT_ERRORS_EXTERIOR_UNKNOWN_WORLD: u32 = 0x0102_ebd0;
pub(crate) const INIT_ERRORS_INTERIOR: u32 = 0x0102_eb60;
pub(crate) const LIGHTING_TEMPLATE_MISSING: u32 = 0x0102_eb10;

// RTTI type descriptors passed to `__RTDynamicCast`.
/// The source type of every cast (`TESForm`).
pub(crate) const RTTI_TES_FORM: u32 = 0x0118_3028;
/// Cast target: `TESObjectCELL` (cells cast to themselves in `Copy`).
pub(crate) const RTTI_CELL: u32 = 0x0118_3fb4;
/// Cast target: `TESObjectLAND` (the duplicated land).
pub(crate) const RTTI_LAND: u32 = 0x0118_ac10;
/// Cast target: the nav mesh class whose `+0x24` holds its cell.
pub(crate) const RTTI_NAV_MESH: u32 = 0x0118_b6b4;
/// Cast target: `TESObjectREFR`.
pub(crate) const RTTI_REFERENCE: u32 = 0x0118_41cc;
/// Cast target for a lighting template.
pub(crate) const RTTI_LIGHTING_TEMPLATE: u32 = 0x0118_a8d0;
/// Cast target of the "world" group labels (`TESWorldSpace`).
pub(crate) const RTTI_WORLD_SPACE: u32 = 0x0118_3fd0;
/// Cast target in `fn_00543230`: an object whose virtual slot 0 gives the
/// cell it belongs to.
pub(crate) const RTTI_CELL_OWNED: u32 = 0x0118_ac2c;

layout! {
    /// `TESObjectCELL` (Xbox PDB), 0xC0 bytes on the Xbox, 0xE0 on the PC.
    /// The vtable is at +0x00; `TESFullName` (a second vtable at +0x18,
    /// `cFullName` at +0x1C) and the `ExtraDataList` at +0x28 are other
    /// classes.
    pub struct TESObjectCELL: 0xe0 {
        /// `iFormFlags` (`TESForm`, Xbox PDB).
        0x08 iFormFlags: u32,
        /// `iFormID` (`TESForm`, Xbox PDB).
        0x0C iFormID: u32,
        /// `cCellFlags` (Xbox PDB): bit 0 interior, 0x20 and 0x40 set by
        /// the two setters, 0x80 and 0x01 tested by `Load`.
        0x24 cCellFlags: u8,
        /// `cCellGameFlags` (Xbox PDB).
        0x25 cCellGameFlags: u8,
        /// `cCellState` (Xbox PDB).
        0x26 cCellState: u8,
        /// `pCellData` (Xbox PDB): the exterior or the interior data.
        0x48 pCellData: Ptr,
        /// `pCellLand` (Xbox PDB): `TESObjectLAND*`.
        0x4C pCellLand: Ptr,
        /// `fWaterHeight` (Xbox PDB).
        0x50 fWaterHeight: f32,
        /// `bAutoWaterLoaded` (Xbox PDB).
        0x54 bAutoWaterLoaded: bool,
        /// `pNavMeshes` (Xbox PDB): `NavMeshArray*`.
        0x64 pNavMeshes: Ptr,
        /// `iCriticalQueuedRefCount` (Xbox PDB).
        0xA0 iCriticalQueuedRefCount: i32,
        /// `iQueuedRefCount` (Xbox PDB).
        0xA4 iQueuedRefCount: i32,
        /// `sNumRefsWithVisibleDistant` (Xbox PDB).
        0xA8 sNumRefsWithVisibleDistant: i16,
        /// `sNumLoadedRefsWithVisibleDistant` (Xbox PDB).
        0xAA sNumLoadedRefsWithVisibleDistant: i16,
        /// `spLightMarkerNode` (Xbox PDB): `NiPointer<NiNode>` slot.
        0xB4 spLightMarkerNode: Ptr,
        /// `spSoundMarkerNode` (Xbox PDB).
        0xB8 spSoundMarkerNode: Ptr,
        /// `spMultiBoundNode` (Xbox PDB).
        0xBC spMultiBoundNode: Ptr,
        /// `pWorldSpace` (Xbox PDB, with `iTempDataOffset`).
        0xC0 pWorldSpace: Ptr,
        /// `pLoadedData` (Xbox PDB).
        0xC4 pLoadedData: Ptr,
        /// `fLodFadeInPercent` (Xbox PDB).
        0xC8 fLodFadeInPercent: f32,
        /// `bLODFadingIn` (Xbox PDB).
        0xCC bLODFadingIn: bool,
        /// `bFadedIn` (Xbox PDB).
        0xCD bFadedIn: bool,
        /// `bFadingToHighDetail` (Xbox PDB).
        0xCE bFadingToHighDetail: bool,
        /// `bFadingToLowDetail` (Xbox PDB).
        0xCF bFadingToLowDetail: bool,
        /// `bDisplayHighDetail` (Xbox PDB).
        0xD0 bDisplayHighDetail: bool,
        /// `bCellDetached` (Xbox PDB).
        0xD1 bCellDetached: bool,
        /// `bUpdateTerrain` (Xbox PDB).
        0xD2 bUpdateTerrain: bool,
        /// `spPortalGraph` (Xbox PDB): `NiPointer<BSPortalGraph>` slot.
        0xD4 spPortalGraph: Ptr,
        /// `pLightingTemplate` (Xbox PDB).
        0xD8 pLightingTemplate: Ptr,
        /// `iLightingTemplateInheritanceFlags` (Xbox PDB).
        0xDC iLightingTemplateInheritanceFlags: u32,
    }
}

/// Offsets of the sub-objects inside the cell that are other classes.
pub(crate) const FULL_NAME_OFFSET: u32 = 0x18;
pub(crate) const EXTRA_DATA_OFFSET: u32 = 0x28;
pub(crate) const WATER_NOISE_TEXTURE_OFFSET: u32 = 0x58;
pub(crate) const CELL_LOCK_OFFSET: u32 = 0x80;
pub(crate) const REFERENCE_LIST_OFFSET: u32 = 0xac;

/// `__RTDynamicCast` of `object` from `TESForm` to `target`.
pub(crate) fn dynamic_cast(e: &mut Engine, object: Ptr, target: u32) -> Ptr {
    e.call(
        RT_DYNAMIC_CAST,
        &args![object, 0u32, RTTI_TES_FORM, target, 0u32],
    )
    .ptr()
}

pub(crate) fn is_interior(e: &mut Engine, cell: Ptr) -> bool {
    e.call(CELL_IS_INTERIOR, &args![cell]).bool()
}

pub(crate) fn persistent_flag(e: &mut Engine, cell: Ptr) -> bool {
    e.call(CELL_PERSISTENT_FLAG, &args![cell]).bool()
}

pub(crate) fn flag_4000(e: &mut Engine, form: Ptr) -> bool {
    e.call(FORM_FLAG_4000, &args![form]).bool()
}

pub(crate) fn flag_20(e: &mut Engine, form: Ptr) -> bool {
    e.call(FORM_FLAG_20, &args![form]).bool()
}

pub(crate) fn flag_2(e: &mut Engine, form: Ptr) -> bool {
    e.call(FORM_FLAG_2, &args![form]).bool()
}

pub(crate) fn form_id(e: &mut Engine, form: Ptr) -> u32 {
    e.call(FORM_ID, &args![form]).u32()
}

pub(crate) fn world_space(e: &mut Engine, cell: Ptr) -> Ptr {
    e.call(CELL_GET_WORLD_SPACE, &args![cell]).ptr()
}

pub(crate) fn slot_get(e: &mut Engine, slot: Ptr) -> Ptr {
    e.call(SLOT_GET, &args![slot]).ptr()
}

pub(crate) fn slot_assign(e: &mut Engine, slot: Ptr, value: Ptr) {
    e.call(SLOT_ASSIGN, &args![slot, value]);
}

/// The cell's list of references (`this + 0xAC`), through its accessor.
pub(crate) fn reference_list(e: &mut Engine, cell: Ptr) -> Ptr {
    e.call(CELL_REFERENCE_LIST, &args![cell]).ptr()
}

/// One step of the `BSSimpleList` walk: false at the end of the list.
pub(crate) fn list_is_end(e: &mut Engine, node: Ptr) -> bool {
    node.is_null() || e.call(LIST_IS_END, &args![node]).bool()
}

/// The item of a list node (the word at the address the accessor returns).
pub(crate) fn list_item(e: &mut Engine, node: Ptr) -> Ptr {
    let address = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
    Ptr::new(e.mem.u32(address))
}

pub(crate) fn list_next(e: &mut Engine, node: Ptr) -> Ptr {
    e.call(LIST_NEXT, &args![node]).ptr()
}

/// A group header as the group helpers read and write it: `[0]` the `GRUP`
/// tag, `[1]` zero, `[2]` the label, `[3]` the group type, `[4]` zero.
pub(crate) fn write_group_header(e: &mut Engine, header: Ptr, group_type: u32, label: u32) {
    let tag = e.global::<u32>(GROUP_TAG);
    let at = header.addr();
    e.mem.set_u32(at, tag);
    e.mem.set_u32(at + 0xc, group_type);
    e.mem.set_u32(at + 8, label);
    e.mem.set_u32(at + 4, 0);
    e.mem.set_u32(at + 0x10, 0);
}

// Translated from 005415b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::TESObjectCELL` (Xbox PDB): builds the base form, the
/// full-name component, the extra data list, the water noise texture, the
/// lock and reference list, the four smart-pointer slots, and sets the
/// default field values (form type 0x39, water height `FLT_MAX`, lighting
/// template inheritance flags 0x9F, form flag 8). Returns `this`.
pub fn tes_object_cell_tes_object_cell(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
) -> Ptr<TESObjectCELL> {
    e.call(FORM_CONSTRUCT, &args![this]);
    e.call(
        CELL_FULL_NAME_CONSTRUCT,
        &args![this.byte_add(FULL_NAME_OFFSET)],
    );
    e.mem.set_u32(this.addr(), CELL_VTABLE);
    e.mem
        .set_u32(this.addr() + FULL_NAME_OFFSET, CELL_FULL_NAME_VTABLE);
    e.call(
        EXTRA_LIST_CONSTRUCT,
        &args![this.byte_add(EXTRA_DATA_OFFSET)],
    );
    e.call(
        TEXTURE_CONSTRUCT,
        &args![this.byte_add(WATER_NOISE_TEXTURE_OFFSET)],
    );
    e.call(LIST_CONSTRUCT, &args![this.byte_add(CELL_LOCK_OFFSET)]);
    e.call(LIST_CONSTRUCT, &args![this.byte_add(REFERENCE_LIST_OFFSET)]);
    for slot in [
        TESObjectCELL::spLightMarkerNode,
        TESObjectCELL::spSoundMarkerNode,
        TESObjectCELL::spMultiBoundNode,
        TESObjectCELL::spPortalGraph,
    ] {
        e.call(SLOT_CONSTRUCT, &args![this.byte_add(slot.off), 0u32]);
    }
    e.set(this, TESObjectCELL::cCellFlags, 0);
    e.set(this, TESObjectCELL::cCellState, 0);
    e.set(this, TESObjectCELL::iLightingTemplateInheritanceFlags, 0x9f);
    e.set(this, TESObjectCELL::pLightingTemplate, Ptr::NULL);
    e.set(this, TESObjectCELL::pCellData, Ptr::NULL);
    e.set(this, TESObjectCELL::pCellLand, Ptr::NULL);
    let water_height: f32 = e.global(DEFAULT_WATER_HEIGHT);
    e.set(this, TESObjectCELL::fWaterHeight, water_height);
    e.set(this, TESObjectCELL::pNavMeshes, Ptr::NULL);
    e.call(FORM_SET_FORM_TYPE, &args![this, 0x39u32]);
    e.set(this, TESObjectCELL::pWorldSpace, Ptr::NULL);
    e.set(this, TESObjectCELL::cCellGameFlags, 0);
    e.set(this, TESObjectCELL::pLoadedData, Ptr::NULL);
    e.set(this, TESObjectCELL::iQueuedRefCount, 0);
    e.set(this, TESObjectCELL::iCriticalQueuedRefCount, 0);
    e.set(this, TESObjectCELL::sNumRefsWithVisibleDistant, 0);
    e.set(this, TESObjectCELL::sNumLoadedRefsWithVisibleDistant, 0);
    e.set(this, TESObjectCELL::fLodFadeInPercent, 0.0);
    e.set(this, TESObjectCELL::bLODFadingIn, false);
    e.set(this, TESObjectCELL::bFadedIn, false);
    e.set(this, TESObjectCELL::bAutoWaterLoaded, false);
    e.set(this, TESObjectCELL::bFadingToLowDetail, false);
    e.set(this, TESObjectCELL::bFadingToHighDetail, false);
    e.set(this, TESObjectCELL::bDisplayHighDetail, false);
    e.set(this, TESObjectCELL::bCellDetached, false);
    e.set(this, TESObjectCELL::bUpdateTerrain, false);
    e.call(FORM_SET_FLAG_8, &args![this, 1u32]);
    slot_assign(
        e,
        this.byte_add(TESObjectCELL::spPortalGraph.off),
        Ptr::NULL,
    );
    // The water noise texture's virtual slot 0 (`InitializeDataComponent`).
    e.vcall(this.addr() + WATER_NOISE_TEXTURE_OFFSET, 0, &[]);
    this
}

// Translated from 005417e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell's scalar deleting destructor (virtual slot 0x10): runs
/// `fn_00541810`, then frees the cell when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_005417e0(e: &mut Engine, this: Ptr<TESObjectCELL>, flags: u32) -> Ptr<TESObjectCELL> {
    fn_00541810(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 00541810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::~TESObjectCELL` body. Resets the vtables, then, for a
/// form without flag 0x4000, releases the cell's state, deletes its land and
/// its nav mesh array and releases the region list of its extra data; unless
/// the loader singleton says otherwise it releases the cell from its world
/// space; clears the references, unregisters the cell and destroys the
/// members in reverse order of construction.
pub fn fn_00541810(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.mem.set_u32(this.addr(), CELL_VTABLE);
    e.mem
        .set_u32(this.addr() + FULL_NAME_OFFSET, CELL_FULL_NAME_VTABLE);
    e.call(CELL_CLEAR_STATE, &args![this]);
    if !flag_4000(e, this.cast()) {
        e.call(CELL_RELEASE_STATE, &args![this]);
        let land = e.get(this, TESObjectCELL::pCellLand);
        if !land.is_null() {
            // Virtual slot 0x10 with the delete flag: the scalar deleting
            // destructor.
            e.vcall(land.addr(), 0x10, &args![1u32]);
            e.set(this, TESObjectCELL::pCellLand, Ptr::NULL);
        }
        let nav_meshes = e.get(this, TESObjectCELL::pNavMeshes);
        if !nav_meshes.is_null() {
            fn_00541a30(e, nav_meshes, 1);
            e.set(this, TESObjectCELL::pNavMeshes, Ptr::NULL);
        }
        let region_list: Ptr = e
            .call(
                EXTRA_LIST_GET_REGION_LIST,
                &args![this.byte_add(EXTRA_DATA_OFFSET)],
            )
            .ptr();
        if !region_list.is_null() {
            e.call(REGION_LIST_RELEASE, &args![region_list]);
            e.vcall(region_list.addr(), 0, &args![1u32]);
        }
    }
    let loader = e.global::<u32>(LOADER_STATE_POINTER);
    if !e.call(LOADER_STATE_FLAG_61D, &args![loader]).bool() {
        let world = world_space(e, this.cast());
        if !world.is_null() {
            e.call(WORLD_SPACE_RELEASE_CELL, &args![world, this]);
        }
    }
    e.call(CELL_CLEAR_REFERENCES, &args![this]);
    e.call(CELL_ERASE, &args![this]);
    fn_00541b00(e, this);
    // Members are destroyed in the reverse order of the constructor.
    for slot in [
        TESObjectCELL::spPortalGraph,
        TESObjectCELL::spMultiBoundNode,
        TESObjectCELL::spSoundMarkerNode,
        TESObjectCELL::spLightMarkerNode,
    ] {
        e.call(SLOT_RELEASE, &args![this.byte_add(slot.off)]);
    }
    e.call(LIST_DESTRUCT, &args![this.byte_add(REFERENCE_LIST_OFFSET)]);
    e.call(
        TEXTURE_DESTRUCT,
        &args![this.byte_add(WATER_NOISE_TEXTURE_OFFSET)],
    );
    e.call(
        EXTRA_LIST_DESTRUCT,
        &args![this.byte_add(EXTRA_DATA_OFFSET)],
    );
    e.call(
        CELL_FULL_NAME_DESTRUCT,
        &args![this.byte_add(FULL_NAME_OFFSET)],
    );
    e.call(FORM_DESTRUCT, &args![this]);
}

// Translated from 00541a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the cell's `NavMeshArray`: runs
/// `fn_00541a60`, then frees the array when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00541a30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00541a60(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 00541a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the `NavMeshArray`: `0069bca0`, then `0042f830`
/// (which restores the base vtable and runs the base destructor).
pub fn fn_00541a60(e: &mut Engine, this: Ptr) {
    e.call(NAV_MESH_ARRAY_CLEAR, &args![this]);
    e.call(NAV_MESH_ARRAY_BASE_DESTRUCT, &args![this]);
}

// Translated from 00541ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CellRefLockEnter`: enters the cell's reference lock (`this + 0x80`),
/// passing the name `"TESObjectCELL::CellRefLockEnter()"`.
pub fn fn_00541ac0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.call(
        LOCK_ENTER,
        &args![this.byte_add(CELL_LOCK_OFFSET), LOCK_NAME],
    );
}

// Translated from 00541ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Leaves the cell's reference lock (`this + 0x80`).
pub fn fn_00541ae0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.call(LOCK_LEAVE, &args![this.byte_add(CELL_LOCK_OFFSET)]);
}

// Translated from 00541b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the cell's data: removes the non-persistent cell data from the
/// extra data list, frees `pCellData` (the code frees the same field in both
/// the interior and the exterior case), clears the lighting template and the
/// inheritance flags.
pub fn fn_00541b00(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.call(
        EXTRA_LIST_REMOVE_NON_PERSISTENT,
        &args![this.byte_add(EXTRA_DATA_OFFSET)],
    );
    // The original tests `IsInterior` and frees `pCellData` in both
    // branches (two differently typed `delete`s).
    let _ = is_interior(e, this.cast());
    let data = e.get(this, TESObjectCELL::pCellData);
    e.call(DEALLOCATE, &args![data]);
    e.set(this, TESObjectCELL::pCellData, Ptr::NULL);
    e.call(CELL_SET_LIGHTING_TEMPLATE, &args![this, 0u32]);
    e.set(this, TESObjectCELL::iLightingTemplateInheritanceFlags, 0);
}

// Translated from 00541b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::InitStatics` (Xbox PDB): resets the static flag, builds
/// the static pointer from the three settings words at `011f426c`, and raises
/// the setting at `011ca094` to 2048.0 when it is above the limit.
pub fn tes_object_cell_init_statics(e: &mut Engine) {
    let words = [
        e.global::<u32>(STATIC_SETTINGS_SOURCE),
        e.global::<u32>(STATIC_SETTINGS_SOURCE + 4),
        e.global::<u32>(STATIC_SETTINGS_SOURCE + 8),
    ];
    e.set_global(STATIC_FLAG, 0u32);
    let built = e.with_stack(12, |e, block| {
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(block.addr() + 4 * i as u32, *word);
        }
        e.call(STATIC_SETTING_BUILDER, &args![block]).ptr::<()>()
    });
    e.call(SLOT_ASSIGN, &args![STATIC_POINTER_SLOT, built]);
    let setting = e
        .call(STATIC_SETTING_GET, &args![STATIC_SETTING_BLOCK])
        .u32();
    let value = e.mem.f32(setting);
    let limit: f64 = e.global(SETTING_LIMIT_DOUBLE);
    // Not taken when the value is below, equal to, or unordered with the limit.
    if (value as f64) > limit {
        let new_value: f32 = e.global(SETTING_LIMIT_FLOAT);
        e.call(STATIC_SETTING_SET, &args![STATIC_SETTING_BLOCK, new_value]);
    }
}

// Translated from 00541c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the statics: when the static object at `011ca088` exists, hands
/// it to its release helper (with the value `fn_00541c80` makes of the
/// static pointer) and deletes it; clears the pointer slot at `011ca0d8` and
/// resets the word at `011cc54c` (`fn_00541cd0`).
pub fn fn_00541c00(e: &mut Engine) {
    let object = e.global::<u32>(STATIC_OBJECT);
    if object != 0 {
        let pointer = slot_get(e, Ptr::new(STATIC_POINTER_SLOT));
        let value = fn_00541c80(e, pointer);
        e.call(
            STATIC_RELEASE_HELPER,
            &args![e.global::<u32>(STATIC_OBJECT), value],
        );
        let object = e.global::<u32>(STATIC_OBJECT);
        if object != 0 {
            // Virtual slot 0 with the delete flag: the scalar deleting
            // destructor.
            e.vcall(object, 0, &args![1u32]);
        }
        e.set_global(STATIC_OBJECT, 0u32);
    }
    slot_assign(e, Ptr::new(STATIC_POINTER_SLOT), Ptr::NULL);
    fn_00541cd0(e);
}

// Translated from 00541c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00541cb0` of `object`, or 0 for a null object.
pub fn fn_00541c80(e: &mut Engine, object: Ptr) -> u32 {
    if object.is_null() {
        0
    } else {
        fn_00541cb0(e, object)
    }
}

// Translated from 00541cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the object's virtual slot 0x94 and returns its result.
pub fn fn_00541cb0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0x94, &[]).u32()
}

// Translated from 00541cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `011cc54c`.
pub fn fn_00541cd0(e: &mut Engine) {
    e.set_global(STATIC_RESET_WORD, 0u32);
}

// Translated from 00541ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell's form-record `Save` (virtual slot 0x2c). Starts the record,
/// writes the full name, a `DATA` byte (the cell flags), the exterior
/// (`XCLC`, 12 bytes) or interior (`XCLL`, 44 bytes) data (byte-swapped in
/// memory around the write when the swap flag is on; interiors also save
/// their impact swap data), then `LTMP` (the lighting template's form id, or
/// 0), `LNAM`, `XCLW`, the `XNAM` water noise texture name and the extra data
/// list, and closes the record.
pub fn fn_00541ce0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.call(FORM_START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![this.byte_add(FULL_NAME_OFFSET)]);
    let flags = e.get(this, TESObjectCELL::cCellFlags);
    e.call(FORM_ADD_CHUNK_BYTE, &args![DATA, flags as u32]);
    if is_interior(e, this.cast()) {
        let data = e.call(CELL_INTERIOR_DATA, &args![this]).ptr::<()>();
        if e.call(ENDIAN_SWAP_ENABLED, &[]).bool() {
            e.call(SWAP_INTERIOR_DATA, &args![data]);
        }
        e.call(
            FORM_ADD_CHUNK_DATA,
            &args![INTERIOR_LIGHTING, data, 0x2cu32],
        );
        if e.call(ENDIAN_SWAP_ENABLED, &[]).bool() {
            e.call(SWAP_INTERIOR_DATA, &args![data]);
        }
        let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
        let swap = e.call(EXTRA_LIST_GET_IMPACT_SWAP, &args![list]).ptr::<()>();
        if !swap.is_null() {
            e.call(IMPACT_SWAP_SAVE, &args![swap]);
        }
    } else {
        let data = e.call(CELL_EXTERIOR_DATA, &args![this]).ptr::<()>();
        if e.call(ENDIAN_SWAP_ENABLED, &[]).bool() {
            e.call(SWAP_EXTERIOR_DATA, &args![data]);
        }
        e.call(
            FORM_ADD_CHUNK_DATA,
            &args![EXTERIOR_COORDINATES, data, 0xcu32],
        );
        if e.call(ENDIAN_SWAP_ENABLED, &[]).bool() {
            e.call(SWAP_EXTERIOR_DATA, &args![data]);
        }
    }
    let template_id = if e.call(CELL_GET_LIGHTING_TEMPLATE, &args![this]).u32() != 0 {
        let template = e.call(CELL_GET_LIGHTING_TEMPLATE, &args![this]).ptr::<()>();
        form_id(e, template)
    } else {
        0
    };
    e.call(
        FORM_ADD_CHUNK_WORD,
        &args![LIGHTING_TEMPLATE_ID, template_id],
    );
    let inheritance = e.get(this, TESObjectCELL::iLightingTemplateInheritanceFlags);
    e.call(FORM_ADD_CHUNK_WORD, &args![INHERITANCE_FLAGS, inheritance]);
    let height = e.get(this, TESObjectCELL::fWaterHeight);
    e.call(FORM_ADD_CHUNK_WORD, &args![WATER_HEIGHT, height]);
    let texture = this.byte_add(WATER_NOISE_TEXTURE_OFFSET);
    let length = e
        .call(FULL_NAME_LENGTH, &args![texture])
        .u32()
        .wrapping_add(1);
    let name = e.call(TEXT_GET_STRING, &args![texture]).u32();
    e.call(
        FORM_ADD_CHUNK_ARRAY,
        &args![WATER_NOISE_TEXTURE, name, length],
    );
    e.call(EXTRA_LIST_SAVE, &args![this.byte_add(EXTRA_DATA_OFFSET)]);
    e.call(FORM_CLOSE_FORM, &args![this]);
}

// Translated from 00541e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::Save` (Xbox PDB): saves the cell, its group header and its
/// references into `file`; false for a deleted cell or one with nothing to
/// save in this file.
///
/// A cell is saved when `file` is a master, when the cell has form flag 2, or
/// when one of its references (walked under the reference lock) comes from
/// `file`, has no file, or has flag 2. Saving runs the form `Save` (virtual
/// slot 0x2c), adds the form to the file, starts the cell's group (type 6,
/// label = the cell's form id) and, for a master file or a cell that comes
/// from `file`, saves the references.
pub fn tes_object_cell_save(e: &mut Engine, this: Ptr<TESObjectCELL>, file: Ptr) -> bool {
    if e.call(FORM_FLAG_20, &args![this]).bool() {
        return false;
    }
    let mut wanted = false;
    fn_00541ac0(e, this);
    if e.call(FILE_IS_MASTER, &args![file]).bool() || flag_2(e, this.cast()) {
        wanted = true;
    } else {
        let mut node = reference_list(e, this.cast());
        while !list_is_end(e, node) {
            let reference = list_item(e, node);
            node = list_next(e, node);
            let reference_file = e
                .call(FORM_GET_FILE, &args![reference, 0xffff_ffffu32])
                .ptr::<()>();
            if reference_file == file || reference_file.is_null() || flag_2(e, reference) {
                wanted = true;
                break;
            }
        }
    }
    fn_00541ae0(e, this);
    if !wanted {
        return false;
    }
    e.vcall(this.addr(), 0x2c, &[]);
    e.call(FILE_ADD_FORM, &args![file, this]);
    start_cell_group(e, this, file);
    if !e.call(FILE_IS_MASTER, &args![file]).bool()
        || e.call(FORM_GET_FILE, &args![this, 0u32]).ptr::<()>() == file
    {
        tes_object_cell_save_references(e, this, file);
    }
    true
}

/// Starts the cell's group (type 6, label = the cell's form id) in `file`.
pub(crate) fn start_cell_group(e: &mut Engine, this: Ptr<TESObjectCELL>, file: Ptr) {
    e.with_stack(0x14, |e, header| {
        write_group_header(e, header, 6, 0);
        let id = e.call(FORM_ID, &args![this]).u32();
        e.mem.set_u32(header.addr() + 8, id);
        e.call(FILE_START_GROUP, &args![file, header]);
    });
}

// Translated from 00541fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::SaveEdit` (Xbox PDB): saves the cell form (virtual slot
/// 0x2c), adds it to `file`, starts its group and saves all references.
/// Always true.
pub fn tes_object_cell_save_edit(e: &mut Engine, this: Ptr<TESObjectCELL>, file: Ptr) -> bool {
    e.vcall(this.addr(), 0x2c, &[]);
    e.call(FILE_ADD_FORM, &args![file, this]);
    start_cell_group(e, this, file);
    tes_object_cell_save_references(e, this, file);
    true
}

// Translated from 00542040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::SaveReferences` (Xbox PDB): collects the references to
/// save (skipping those with form flag 0x4000, deleted ones that do not come
/// from a master file, ones from other files without flag 2, and persistent
/// references of an exterior cell that is not itself persistent) into a local
/// `NiTArray`, compacts it, orders it with a selection sort driven by each
/// form's virtual `SavesBefore` (slot 0x3c), and saves each form to `file`.
pub fn tes_object_cell_save_references(e: &mut Engine, this: Ptr<TESObjectCELL>, file: Ptr) {
    // The local `NiTArray<TESObjectREFR *>` is 0x10 bytes.
    e.with_stack(0x10, |e, array| {
        e.call(NI_ARRAY_CONSTRUCT, &args![array, 0u32, 1u32]);
        e.call(NI_ARRAY_SET_SIZE, &args![array, 0x32u32]);
        e.call(NI_ARRAY_SET_GROW_BY, &args![array, 0x32u32]);
        fn_00541ac0(e, this);
        let mut node = reference_list(e, this.cast());
        while !list_is_end(e, node) {
            let reference = list_item(e, node);
            node = list_next(e, node);
            let mut keep = true;
            if flag_4000(e, reference) {
                keep = false;
            }
            if flag_20(e, reference) {
                let reference_file = e.call(FORM_GET_FILE, &args![reference, 0u32]).ptr::<()>();
                if !reference_file.is_null()
                    && !e.call(FILE_IS_MASTER, &args![reference_file]).bool()
                {
                    keep = false;
                }
            }
            let source = e
                .call(FORM_GET_FILE, &args![reference, 0xffff_ffffu32])
                .ptr::<()>();
            if file != source && !flag_2(e, reference) {
                keep = false;
            }
            if !is_interior(e, this.cast())
                && e.call(REFERENCE_GET_PERSISTS, &args![reference]).bool()
                && !persistent_flag(e, this.cast())
            {
                keep = false;
            }
            if keep {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), reference.addr());
                    e.call(NI_ARRAY_ADD, &args![array, slot]);
                });
            }
        }
        fn_00541ae0(e, this);
        e.call(NI_ARRAY_COMPACT, &args![array]);
        let count = e.call(NI_ARRAY_COUNT, &args![array]).u32() as i32;
        // Selection sort: after pass `i`, element `i` is the one no later
        // element is ordered before.
        let mut i = 0;
        while i < count {
            let mut first = array_element(e, array, i as u32);
            let mut j = i + 1;
            while j < count {
                let second = array_element(e, array, j as u32);
                let before = e.vcall(second.addr(), 0x3c, &args![first]).bool();
                if before {
                    set_array_element(e, array, i as u32, second);
                    set_array_element(e, array, j as u32, first);
                    first = second;
                }
                j += 1;
            }
            i += 1;
        }
        for i in 0..count {
            let form = array_element(e, array, i as u32);
            if !form.is_null() {
                e.call(FORM_SAVE_TO_FILE, &args![form, file]);
            }
        }
        e.call(NI_ARRAY_DESTRUCT, &args![array]);
    });
}

/// Element `index` of the local reference array, through the element-address
/// accessor.
pub(crate) fn array_element(e: &mut Engine, array: Ptr, index: u32) -> Ptr {
    let address = e.call(NI_ARRAY_ELEMENT_ADDRESS, &args![array, index]).u32();
    Ptr::new(e.mem.u32(address))
}

/// `SetAt(index, &value)` on the local reference array.
pub(crate) fn set_array_element(e: &mut Engine, array: Ptr, index: u32, value: Ptr) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value.addr());
        e.call(NI_ARRAY_SET_AT, &args![array, index, slot]);
    });
}

// Translated from 005422b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::basic_streambuf<>::~basic_streambuf` as the exe instantiates it for
/// the `NiTArray` destructor: calls `00559460` on `this`.
pub fn fn_005422b0(e: &mut Engine, this: Ptr) {
    e.call(NI_ARRAY_DESTRUCT, &args![this]);
}

// Translated from 005422d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell's form-record `Load` (virtual slot 0x20): false unless the
/// record in `file` is a cell (form type 0x39). Loads the base form, then
/// reads chunks until the file has no more:
///
/// - `DATA`: the cell flags (one byte, from a 4-byte chunk if that is its
///   size); bit 2 is set when bit 1 is clear; an interior with bit 0x80 gets
///   the default water height; bit 0x40 is cleared when the loader singleton
///   says so; the cell data is created, and an interior with master data
///   gets it stored.
/// - `XCLC` / `XCLL`: the exterior (12 bytes) / interior (44 bytes) data into
///   the cell's data block, byte-swapped around the read when the file needs
///   it.
/// - `XCLW`, `LNAM`, `LTMP`: the water height, the inheritance flags and the
///   lighting template id.
/// - `EDID` (virtual slot 0x134), `OBND` (slot 0xE0), `FULL` (full name),
///   `XNAM` (water noise texture name), `IMPF`/`IMPS` (the impact swap data,
///   created on first use) and the 76 extra-data tags (`ExtraDataList::Load`).
///
/// Finally clears form flag 8 and returns true.
pub fn fn_005422d0(e: &mut Engine, this: Ptr<TESObjectCELL>, file: Ptr) -> bool {
    if e.call(FILE_GET_FORM_TYPE, &args![file]).u8() != 0x39 {
        return false;
    }
    let mut master_data = 0u32;
    if e.call(FILE_IS_MASTER, &args![file]).bool() {
        master_data = e.call(FILE_MASTER_DATA, &args![file]).u32();
    }
    e.call(FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        match chunk {
            EDITOR_ID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                let buffer = e.mem.alloc(size.max(4));
                e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                e.vcall(this.addr(), 0x134, &args![buffer]);
                e.mem.free(buffer);
            }
            OBJECT_BOUNDS => {
                e.vcall(this.addr(), 0xe0, &args![file]);
            }
            FULL_NAME => {
                let full_name = if this.is_null() {
                    0
                } else {
                    this.addr() + FULL_NAME_OFFSET
                };
                e.call(FULL_NAME_LOAD, &args![full_name, file]);
            }
            DATA => load_cell_flags(e, this, file, master_data),
            IMPACT_SWAP_FIRST | IMPACT_SWAP_SECOND => {
                let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
                let mut swap = e.call(EXTRA_LIST_GET_IMPACT_SWAP, &args![list]).ptr::<()>();
                if swap.is_null() {
                    let memory = e.call(ALLOCATE, &args![0x15cu32]).ptr::<()>();
                    swap = if memory.is_null() {
                        Ptr::NULL
                    } else {
                        e.call(IMPACT_SWAP_CONSTRUCT, &args![memory]).ptr()
                    };
                    let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
                    e.call(EXTRA_LIST_SET_IMPACT_SWAP, &args![list, swap]);
                }
                e.call(IMPACT_SWAP_LOAD, &args![swap, file]);
            }
            LIGHTING_TEMPLATE_ID => {
                let mut id = 0u32;
                e.with_stack(4, |e, slot| {
                    e.call(FILE_GET_CHUNK_DATA, &args![file, slot]);
                    id = e.mem.u32(slot.addr());
                });
                e.set(this, TESObjectCELL::pLightingTemplate, Ptr::new(id));
            }
            WATER_NOISE_TEXTURE => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                if size != 0 {
                    let buffer = e.mem.alloc(size);
                    e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, buffer, size]);
                    e.call(
                        FULL_NAME_SET,
                        &args![this.byte_add(WATER_NOISE_TEXTURE_OFFSET), buffer],
                    );
                    e.mem.free(buffer);
                }
            }
            INHERITANCE_FLAGS => {
                e.call(
                    FILE_GET_CHUNK_DATA,
                    &args![
                        file,
                        this.byte_add(TESObjectCELL::iLightingTemplateInheritanceFlags.off)
                    ],
                );
            }
            WATER_HEIGHT => {
                e.call(
                    FILE_GET_CHUNK_DATA,
                    &args![file, this.byte_add(TESObjectCELL::fWaterHeight.off)],
                );
            }
            EXTERIOR_COORDINATES => {
                let data = e.call(CELL_EXTERIOR_DATA, &args![this]).ptr::<()>();
                load_cell_data(e, file, data, 0xc, SWAP_EXTERIOR_DATA);
            }
            INTERIOR_LIGHTING => {
                let data = e.call(CELL_INTERIOR_DATA, &args![this]).ptr::<()>();
                load_cell_data(e, file, data, 0x2c, SWAP_INTERIOR_DATA);
            }
            other if EXTRA_DATA_TAGS.contains(&other) => {
                e.call(
                    EXTRA_LIST_LOAD,
                    &args![this.byte_add(EXTRA_DATA_OFFSET), file, this],
                );
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.call(FORM_SET_FLAG_8, &args![this, 0u32]);
    true
}

/// `Load`'s `DATA` chunk (see `fn_005422d0`).
pub(crate) fn load_cell_flags(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    file: Ptr,
    master_data: u32,
) {
    if e.call(FILE_CHUNK_SIZE, &args![file]).u32() == 4 {
        let mut word = 0u32;
        e.with_stack(4, |e, slot| {
            e.call(FILE_GET_CHUNK_DATA, &args![file, slot]);
            word = e.mem.u32(slot.addr());
        });
        e.set(this, TESObjectCELL::cCellFlags, word as u8);
    } else {
        e.call(
            FILE_GET_CHUNK_DATA_SIZED,
            &args![file, this.byte_add(TESObjectCELL::cCellFlags.off), 1u32],
        );
    }
    let flags = e.get(this, TESObjectCELL::cCellFlags);
    if flags & 1 == 0 {
        e.set(this, TESObjectCELL::cCellFlags, flags | 2);
    }
    let flags = e.get(this, TESObjectCELL::cCellFlags);
    if flags & 1 != 0 && flags & 0x80 != 0 {
        let height: f32 = e.global(DEFAULT_WATER_HEIGHT);
        e.call(CELL_SET_WATER_HEIGHT, &args![this, height]);
    }
    let loader = e.global::<u32>(LOADER_STATE_POINTER);
    if !fn_00542d20(e, Ptr::new(loader)) {
        let flags = e.get(this, TESObjectCELL::cCellFlags);
        e.set(this, TESObjectCELL::cCellFlags, flags & 0xbf);
    }
    e.call(CELL_CREATE_CELL_DATA, &args![this]);
    if is_interior(e, this.cast()) && master_data != 0 {
        e.call(CELL_SET_LOADED_MASTER_DATA, &args![this, master_data]);
    }
}

/// `Load`'s `XCLC` / `XCLL` chunks: reads `size` bytes into the cell data
/// block, swapping it before the read when the chunk is not exactly `size`
/// bytes and the file needs swapping, and after it when it needs swapping.
pub(crate) fn load_cell_data(e: &mut Engine, file: Ptr, data: Ptr, size: u32, swap: u32) {
    if data.is_null() {
        return;
    }
    if e.call(FILE_SWAP_ENDIAN, &args![file]).bool()
        && e.call(FILE_CHUNK_SIZE, &args![file]).u32() != size
    {
        e.call(swap, &args![data]);
    }
    e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, data, size]);
    if e.call(FILE_SWAP_ENDIAN, &args![file]).bool() {
        e.call(swap, &args![data]);
    }
}

// Translated from 00542d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x619` of the loader singleton (`011c3f2c`).
pub fn fn_00542d20(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x619) != 0
}

// Translated from 00542d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::InitItem` (Xbox PDB) (virtual slot 0x88).
///
/// Unless form flag 8 is set: counts the errors raised meanwhile (the
/// thread's error count is saved, set to 0, and read back afterwards), mutes
/// warnings, initializes the extra data list, clears the full name of an
/// exterior cell, sets flag 8, warns about a full name over 33 characters,
/// reports a non-zero error count with a message that names the cell (and its
/// world space and coordinates for an exterior one), and resolves the
/// lighting template by form id (warning when it is missing).
///
/// Then, with the loader's flag switched off, it initializes every reference
/// (those without flag 8 go through the form `InitItem`, `00858730` and the
/// game loader; the player helper form of an interior sets the north
/// rotation to minus its rotation; a reference that moved to another cell
/// restarts the walk) and every nav mesh, restores the flag, and finishes the
/// save-game object's pending load.
pub fn tes_object_cell_init_item(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let cell: Ptr = this.cast();
    if !e.call(FORM_FLAG_8, &args![cell]).bool() {
        let saved_errors = e.call(ERROR_COUNT_GET, &[]).u32();
        e.call(INC_DISABLE_WARNING_COUNT, &args![1u32]);
        e.call(ERROR_COUNT_SET, &args![0u32]);
        e.call(
            EXTRA_LIST_INIT_ITEM,
            &args![this.byte_add(EXTRA_DATA_OFFSET), this],
        );
        if !is_interior(e, cell) {
            e.call(FULL_NAME_SET, &args![this.byte_add(FULL_NAME_OFFSET), 0u32]);
        }
        e.call(FORM_SET_FLAG_8, &args![this, 1u32]);
        let full_name = this.byte_add(FULL_NAME_OFFSET);
        if e.call(TEXT_GET_STRING, &args![full_name]).u32() != 0 {
            let text = e.call(TEXT_GET_STRING, &args![full_name]).u32();
            if e.call(CRT_STRLEN, &args![text]).u32() > 0x21 {
                let id = form_id(e, cell);
                let editor_id = e.vcall(this.addr(), 0x130, &[]).u32();
                let text = e.call(TEXT_GET_STRING, &args![full_name]).u32();
                e.call(DEBUG_PRINT, &args![CELL_NAME_TOO_LONG, text, editor_id, id]);
            }
        }
        e.call(INC_DISABLE_WARNING_COUNT, &args![0u32]);
        let errors = e.call(ERROR_COUNT_GET, &[]).u32();
        e.call(ERROR_COUNT_SET, &args![saved_errors]);
        if errors != 0 {
            report_init_errors(e, this);
        }
        resolve_lighting_template(e, this);
    }
    initialize_references(e, this);
}

/// The error message `InitItem` prints for a cell with errors.
pub(crate) fn report_init_errors(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let cell: Ptr = this.cast();
    let world = world_space(e, cell);
    if !is_interior(e, cell) {
        if !world.is_null() {
            let world_id = form_id(e, world);
            let world_name = e.vcall(world.addr(), 0x130, &[]).u32();
            let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
            let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
            let id = form_id(e, cell);
            let name = e.vcall(this.addr(), 0x130, &[]).u32();
            e.call(
                DEBUG_PRINT,
                &args![
                    INIT_ERRORS_EXTERIOR_WORLD,
                    name,
                    id,
                    x,
                    y,
                    world_name,
                    world_id
                ],
            );
        } else {
            let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
            let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
            let id = form_id(e, cell);
            let name = e.vcall(this.addr(), 0x130, &[]).u32();
            e.call(
                DEBUG_PRINT,
                &args![INIT_ERRORS_EXTERIOR_UNKNOWN_WORLD, name, id, x, y],
            );
        }
    } else {
        let id = form_id(e, cell);
        let name = e.vcall(this.addr(), 0x130, &[]).u32();
        e.call(DEBUG_PRINT, &args![INIT_ERRORS_INTERIOR, name, id]);
    }
}

/// Looks the lighting template up by the id stored in `pLightingTemplate`
/// (made file-relative with `TESForm::AddCompileIndex`) and stores the
/// resolved object; warns when the id does not resolve.
pub(crate) fn resolve_lighting_template(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let cell: Ptr = this.cast();
    if e.call(CELL_GET_LIGHTING_TEMPLATE, &args![cell]).u32() == 0 {
        return;
    }
    let stored = e.call(CELL_GET_LIGHTING_TEMPLATE, &args![cell]).u32();
    let file = e.call(FORM_GET_FILE, &args![cell, 0xffff_ffffu32]).u32();
    e.with_stack(4, |e, id_slot| {
        e.mem.set_u32(id_slot.addr(), stored);
        e.call(FORM_ADD_COMPILE_INDEX, &args![id_slot, file]);
        let id = e.mem.u32(id_slot.addr());
        let form = e.call(FORM_LOOK_UP, &args![id]).ptr::<()>();
        let template = dynamic_cast(e, form, RTTI_LIGHTING_TEMPLATE);
        e.call(CELL_SET_LIGHTING_TEMPLATE, &args![cell, template]);
        if template.is_null() && id != 0 {
            let cell_id = form_id(e, cell);
            let name = e.vcall(cell.addr(), 0x130, &[]).u32();
            e.call(
                DEBUG_PRINT,
                &args![LIGHTING_TEMPLATE_MISSING, id, name, cell_id],
            );
        }
    });
}

/// The reference and nav mesh walk at the end of `InitItem`.
pub(crate) fn initialize_references(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let cell: Ptr = this.cast();
    let loader = Ptr::<()>::new(e.global::<u32>(GAME_LOADER_POINTER));
    let save_game = Ptr::<()>::new(e.global::<u32>(SAVE_GAME_POINTER));
    let previous_flag = e.call(GAME_LOADER_FLAG_SETTER, &args![loader, 0u32]).u8();
    let mut node = reference_list(e, cell);
    let mut previous = Ptr::NULL;
    while !node.is_null() && !list_is_end(e, node) {
        let reference = list_item(e, node);
        if !e.call(FORM_FLAG_8, &args![reference]).bool() {
            e.vcall(reference.addr(), 0x88, &[]);
            e.call(GAME_LOAD_NOTIFY, &args![save_game, reference]);
            e.call(GAME_LOAD_FORM, &args![loader, reference]);
        }
        if is_interior(e, cell) {
            let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![reference]).u32();
            if form == e.global::<u32>(PLAYER_HELPER_FORM) {
                let rotation = e.call(REFERENCE_ROTATION, &args![reference]).u32();
                let factor: f64 = e.global(MINUS_ONE);
                let north = (e.mem.f32(rotation + 8) as f64 * factor) as f32;
                let list = e.call(CELL_EXTRA_DATA_LIST, &args![cell]).ptr::<()>();
                e.call(EXTRA_LIST_SET_NORTH_ROTATION, &args![list, north]);
            }
        }
        if !persistent_flag(e, cell)
            && e.call(REFERENCE_PARENT_CELL, &args![reference]).ptr::<()>() != cell
        {
            // The reference left this cell: continue after the previous node
            // (or from the head).
            node = if previous.is_null() {
                reference_list(e, cell)
            } else {
                list_next(e, previous)
            };
        } else {
            previous = node;
            node = list_next(e, node);
        }
    }
    if !e.call(CELL_NAV_MESHES, &args![cell]).ptr::<()>().is_null() {
        let mut index = 0u32;
        loop {
            let nav_meshes = e.call(CELL_NAV_MESHES, &args![cell]).ptr::<()>();
            let count = e.call(NAV_MESH_ARRAY_COUNT, &args![nav_meshes]).u32();
            if index >= count {
                break;
            }
            let nav_meshes = e.call(CELL_NAV_MESHES, &args![cell]).ptr::<()>();
            e.with_stack(4, |e, pointer| {
                e.call(NAV_MESH_ARRAY_GET, &args![nav_meshes, pointer, index]);
                let nav_mesh = slot_get(e, pointer);
                e.vcall(nav_mesh.addr(), 0x88, &[]);
                let raw = e.call(NAV_POINTER_GET, &args![pointer]).u32();
                e.call(GAME_LOAD_FORM, &args![loader, raw]);
                e.call(NAV_POINTER_RELEASE, &args![pointer]);
            });
            index += 1;
        }
    }
    e.call(
        GAME_LOADER_FLAG_SETTER,
        &args![loader, previous_flag as u32],
    );
    if e.call(GAME_FLAG_READ, &args![save_game]).bool() {
        e.call(GAME_FINISH_A, &args![save_game, cell]);
        if !e.call(GAME_FLAG_READ, &args![save_game]).bool()
            && e.call(GAME_FLAG_TEST, &args![save_game]).bool()
        {
            let saved = e.call(GAME_FLAG_READ, &args![save_game]).u8();
            e.call(GAME_FLAG_WRITE, &args![save_game, 1u32]);
            e.call(GAME_FINISH_B, &args![save_game, 0u32, 0u32, 0u32]);
            e.call(GAME_FINISH_C, &args![save_game, 0u32]);
            e.call(GAME_FLAG_WRITE, &args![save_game, saved as u32]);
        }
    }
}

// Translated from 00543230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell's `SavesBefore` (virtual slot 0x3c): whether this cell is saved
/// before `other`, by form type.
///
/// - Cell (0x39): an interior cell before an exterior one; otherwise cells
///   of the same kind are ordered by world space (the world space's own
///   `SavesBefore`), persistence, block key, sub-block key and form id.
/// - Types 0x3a to 0x40, 0x42, 0x43, 0x69: the cast object's cell (virtual
///   slot 0) is this cell, or this cell's `SavesBefore` of it.
/// - Type 0x41: always true for an interior cell, else the world space's
///   `SavesBefore(other)`.
/// - Anything else: `00484020`.
pub fn fn_00543230(e: &mut Engine, this: Ptr<TESObjectCELL>, other: Ptr) -> bool {
    let cell: Ptr = this.cast();
    let form_type = e.call(FORM_TYPE, &args![other]).u32();
    match form_type {
        0x39 => {
            let other_cell = dynamic_cast(e, other, RTTI_CELL);
            if other_cell.is_null() {
                return false;
            }
            let this_interior = is_interior(e, cell);
            let other_interior = is_interior(e, other_cell);
            if this_interior && !other_interior {
                return true;
            }
            if this_interior != other_interior {
                return false;
            }
            let mut compare_keys = true;
            let mut result = false;
            if !this_interior {
                let world = world_space(e, cell);
                let other_world = world_space(e, other_cell);
                if world != other_world {
                    result = e.vcall(world.addr(), 0x3c, &args![other_world]).bool();
                    compare_keys = false;
                } else if persistent_flag(e, cell) != persistent_flag(e, other_cell) {
                    result = persistent_flag(e, cell);
                    compare_keys = false;
                }
            }
            if compare_keys {
                let (a, b) = (fn_005441b0(e, this), fn_005441b0(e, other_cell.cast()));
                if a < b {
                    return true;
                }
                if a == b {
                    let (c, d) = (fn_00544210(e, this), fn_00544210(e, other_cell.cast()));
                    if c < d {
                        return true;
                    }
                    if c == d && form_id(e, cell) < form_id(e, other_cell) {
                        return true;
                    }
                }
            }
            result
        }
        0x3a..=0x40 | 0x42 | 0x43 | 0x69 => {
            let owned = dynamic_cast(e, other, RTTI_CELL_OWNED);
            let owner = e.vcall(owned.addr(), 0, &[]).ptr::<()>();
            if owner == cell {
                true
            } else {
                let owner = e.vcall(owned.addr(), 0, &[]).u32();
                e.vcall(this.addr(), 0x3c, &args![owner]).bool()
            }
        }
        0x41 => {
            if !is_interior(e, cell) {
                let world = world_space(e, cell);
                e.vcall(world.addr(), 0x3c, &args![other]).bool()
            } else {
                true
            }
        }
        _ => e.call(FORM_COMPARE_FALLBACK, &args![this, other]).bool(),
    }
}

// Translated from 005434d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell's `SavesBefore` for a group header (virtual slot 0x38): whether
/// the cell is saved before `header`'s group, by group type
/// (`header[3]`, label `header[2]`).
///
/// - 0: `00484150`. 1: an interior cell is before it; an exterior cell is
///   when it is before the world space the label names. 2: an interior
///   block key lower than the label. 3: sub-block key lower than the label.
///   4: not before for an interior cell; true for a persistent exterior
///   cell; else block key lower than the label. 5: the same with the
///   sub-block key. 6, 8, 9: before the cell the label names. 7: always.
/// - Not a group header (`header` null, or its tag is not `GRUP`): false.
pub fn fn_005434d0(e: &mut Engine, this: Ptr<TESObjectCELL>, header: Ptr) -> bool {
    let cell: Ptr = this.cast();
    if header.is_null() || e.mem.u32(header.addr()) != e.global::<u32>(GROUP_TAG) {
        return false;
    }
    let label = e.mem.u32(header.addr() + 8);
    match e.mem.u32(header.addr() + 0xc) {
        0 => e
            .call(FORM_BELONGS_IN_GROUP_FALLBACK, &args![this, header])
            .bool(),
        1 => {
            if is_interior(e, cell) {
                true
            } else {
                let form = e.call(FORM_LOOK_UP, &args![label]).ptr::<()>();
                let world = dynamic_cast(e, form, RTTI_WORLD_SPACE);
                !world.is_null() && e.vcall(this.addr(), 0x3c, &args![world]).bool()
            }
        }
        2 => is_interior(e, cell) && fn_005441b0(e, this) < label,
        3 => is_interior(e, cell) && fn_00544210(e, this) < label,
        4 => {
            if is_interior(e, cell) || persistent_flag(e, cell) {
                true
            } else {
                fn_005441b0(e, this) < label
            }
        }
        5 => {
            if is_interior(e, cell) || persistent_flag(e, cell) {
                true
            } else {
                fn_00544210(e, this) < label
            }
        }
        6 | 8 | 9 => {
            let form = e.call(FORM_LOOK_UP, &args![label]).ptr::<()>();
            let other_cell = dynamic_cast(e, form, RTTI_CELL);
            !other_cell.is_null() && e.vcall(this.addr(), 0x3c, &args![other_cell]).bool()
        }
        7 => true,
        _ => false,
    }
}

// Translated from 005436e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::CreateDuplicateForm` (Xbox PDB) (virtual slot 0x40).
/// `flag` is not used; `arg` is passed down to every duplication.
///
/// Null for an interior cell. Otherwise duplicates the form (`004867a0`),
/// casts it to a cell, then duplicates the land (setting it on the new cell
/// and the new cell on it, marking it altered) and the nav mesh array (each
/// non-deleted nav mesh duplicated, pointed at the new cell and added to a
/// new array), and finally duplicates the references that are not flagged
/// 0x4000 and are either in an interior or persistent cell or not persistent
/// themselves, adding each to the new cell (carrying over the cell's
/// persistent flag) and marking it altered. Returns the new cell.
pub fn tes_object_cell_create_duplicate_form(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    flag: u32,
    arg: u32,
) -> Ptr<TESObjectCELL> {
    let _ = flag;
    let cell: Ptr = this.cast();
    if is_interior(e, cell) {
        return Ptr::NULL;
    }
    let copy = e.call(FORM_DUPLICATE, &args![this, 0u32, arg]).ptr::<()>();
    let new_cell = dynamic_cast(e, copy, RTTI_CELL);
    let land = e.get(this, TESObjectCELL::pCellLand);
    if !land.is_null() {
        let duplicate = e.vcall(land.addr(), 0x40, &args![0u32, arg]).ptr::<()>();
        let new_land = dynamic_cast(e, duplicate, RTTI_LAND);
        if !new_land.is_null() {
            e.call(CELL_SET_LAND, &args![new_cell, new_land]);
            e.call(LAND_SET_CELL, &args![new_land, new_cell]);
            e.vcall(new_land.addr(), 0xc8, &args![1u32]);
        }
    }
    let nav_meshes = e.get(this, TESObjectCELL::pNavMeshes);
    if !nav_meshes.is_null() {
        duplicate_nav_meshes(e, this, new_cell, arg);
    }
    fn_00541ac0(e, this);
    e.with_stack(8, |e, list| {
        e.call(LIST_CONSTRUCT, &args![list]);
        let mut node = reference_list(e, cell);
        while !list_is_end(e, node) {
            let reference = list_item(e, node);
            if !flag_4000(e, reference)
                && (is_interior(e, cell)
                    || persistent_flag(e, cell)
                    || !e.call(REFERENCE_GET_PERSISTS, &args![reference]).bool())
            {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), reference.addr());
                    e.call(LIST_PUSH_FRONT, &args![list, slot]);
                });
            }
            node = list_next(e, node);
        }
        fn_00541ae0(e, this);
        let mut node = list;
        while !list_is_end(e, node) {
            let reference = list_item(e, node);
            let duplicate = e
                .vcall(reference.addr(), 0x40, &args![0u32, arg])
                .ptr::<()>();
            let new_reference = dynamic_cast(e, duplicate, RTTI_REFERENCE);
            if !new_reference.is_null() {
                let persists = persistent_flag(e, new_cell);
                e.call(
                    REFERENCE_SET_PERSISTS,
                    &args![new_reference, persists as u32],
                );
                e.call(CELL_ADD_REFERENCE, &args![new_cell, new_reference, 0u32]);
                e.vcall(new_reference.addr(), 0xc8, &args![1u32]);
            }
            node = list_next(e, node);
        }
        e.call(LIST_CLEAR, &args![list]);
        e.call(LIST_DESTRUCT, &args![list]);
    });
    new_cell.cast()
}

/// The nav mesh part of `CreateDuplicateForm`.
pub(crate) fn duplicate_nav_meshes(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    new_cell: Ptr,
    arg: u32,
) {
    let memory = e.call(ALLOCATE, &args![0x10u32]).ptr::<()>();
    let array = if memory.is_null() {
        Ptr::NULL
    } else {
        e.call(NAV_MESH_ARRAY_CONSTRUCT, &args![memory]).ptr::<()>()
    };
    e.call(CELL_SET_NAV_MESHES, &args![new_cell, array]);
    // `NiPointer<NavMesh>` temporaries on the stack.
    let collected = e.mem.alloc(4);
    let element = e.mem.alloc(4);
    let wrapped = e.mem.alloc(4);
    let by_value = e.mem.alloc(4);
    e.call(NAV_POINTER_CONSTRUCT, &args![collected]);
    let nav_meshes = e.get(this, TESObjectCELL::pNavMeshes);
    let mut index = 0u32;
    while index < e.call(NAV_MESH_ARRAY_COUNT, &args![nav_meshes]).u32() {
        let slot = e
            .call(NAV_MESH_ARRAY_GET, &args![nav_meshes, element, index])
            .ptr::<()>();
        let nav_mesh = slot_get(e, slot);
        e.call(NAV_POINTER_RELEASE, &args![element]);
        if !nav_mesh.is_null() && !flag_20(e, nav_mesh) {
            let duplicate = e
                .vcall(nav_mesh.addr(), 0x40, &args![0u32, arg])
                .ptr::<()>();
            let new_nav_mesh = dynamic_cast(e, duplicate, RTTI_NAV_MESH);
            if !new_nav_mesh.is_null() {
                // The duplicate's cell field (`+0x24`).
                e.mem.set_u32(new_nav_mesh.addr() + 0x24, new_cell.addr());
                let pointer = e
                    .call(NAV_POINTER_FROM_RAW, &args![wrapped, new_nav_mesh])
                    .ptr::<()>();
                e.call(NAV_POINTER_ASSIGN, &args![collected, pointer]);
                e.call(NAV_POINTER_RELEASE, &args![wrapped]);
                e.call(NAV_POINTER_COPY, &args![by_value, collected]);
                let copy = e.mem.u32(by_value);
                let new_nav_meshes = e.mem.u32(new_cell.addr() + TESObjectCELL::pNavMeshes.off);
                // The callee takes the smart pointer by value and releases it.
                e.call(NAV_MESH_ARRAY_ADD, &args![new_nav_meshes, copy]);
            }
        }
        index += 1;
    }
    e.call(NAV_POINTER_RELEASE, &args![collected]);
    for block in [collected, element, wrapped, by_value] {
        e.mem.free(block);
    }
}

// Translated from 00543ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::Copy` (Xbox PDB) (virtual slot 0x108): copies `source`
/// (cast to a cell; nothing happens for another type) into this cell: the
/// form components, the extra data list, the cell flags, the water height,
/// the cell data block (12 or 44 bytes, by interior-ness), the lighting
/// template and inheritance flags, and the water noise texture component.
pub fn tes_object_cell_copy(e: &mut Engine, this: Ptr<TESObjectCELL>, source: Ptr) {
    let cell: Ptr = this.cast();
    let source_cell = dynamic_cast(e, source, RTTI_CELL);
    if source_cell.is_null() {
        return;
    }
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, source_cell]);
    let source_list = e
        .call(CELL_EXTRA_DATA_LIST, &args![source_cell])
        .ptr::<()>();
    let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
    e.call(EXTRA_LIST_COPY_LIST, &args![list, source_list]);
    // The original frees `pCellData` in both branches of an `IsInterior` test.
    let _ = is_interior(e, cell);
    let data = e.get(this, TESObjectCELL::pCellData);
    e.call(DEALLOCATE, &args![data]);
    e.set(this, TESObjectCELL::pCellData, Ptr::NULL);
    let flags = fn_00543c30(e, source_cell.cast());
    e.call(CELL_SET_CELL_FLAGS, &args![this, flags as u32]);
    let height = e
        .mem
        .f32(source_cell.addr() + TESObjectCELL::fWaterHeight.off);
    e.set(this, TESObjectCELL::fWaterHeight, height);
    e.call(CELL_CREATE_CELL_DATA, &args![this]);
    if is_interior(e, cell) {
        let to = e.call(CELL_INTERIOR_DATA, &args![this]).u32();
        let from = e.call(CELL_INTERIOR_DATA, &args![source_cell]).u32();
        if to != 0 && from != 0 {
            e.call(MEMORY_COPY, &args![to, from, 0x2cu32]);
        }
    } else {
        let to = e.call(CELL_EXTERIOR_DATA, &args![this]).u32();
        let from = e.call(CELL_EXTERIOR_DATA, &args![source_cell]).u32();
        if to != 0 && from != 0 {
            e.call(MEMORY_COPY, &args![to, from, 0xcu32]);
        }
    }
    let template = e
        .call(CELL_GET_LIGHTING_TEMPLATE, &args![source_cell])
        .u32();
    e.call(CELL_SET_LIGHTING_TEMPLATE, &args![this, template]);
    let inheritance = e
        .call(CELL_GET_INHERITANCE_FLAGS, &args![source_cell])
        .u32();
    e.set(
        this,
        TESObjectCELL::iLightingTemplateInheritanceFlags,
        inheritance,
    );
    // The water noise texture's virtual `CopyComponent` (slot 8).
    e.vcall(
        this.addr() + WATER_NOISE_TEXTURE_OFFSET,
        8,
        &args![source_cell.addr() + WATER_NOISE_TEXTURE_OFFSET],
    );
}

// Translated from 00543c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell flags byte (`cCellFlags`).
pub fn fn_00543c30(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u8 {
    e.get(this, TESObjectCELL::cCellFlags)
}

// Translated from 00543c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell's `Compare` (virtual slot 0x10c): true when `other` differs.
/// True for a non-cell, for different form components, cell flags (compared
/// as signed bytes), inheritance flags, lighting templates or water heights
/// (a NaN counts as different), different cell data blocks (when both exist),
/// different extra data lists, or different water noise texture components.
pub fn fn_00543c50(e: &mut Engine, this: Ptr<TESObjectCELL>, other: Ptr) -> bool {
    let cell: Ptr = this.cast();
    let other_cell = dynamic_cast(e, other, RTTI_CELL);
    if other_cell.is_null() {
        return true;
    }
    if e.call(FORM_COMPARE_ALL_COMPONENTS, &args![this, other_cell])
        .bool()
    {
        return true;
    }
    if fn_00543c30(e, this) as i8 != fn_00543c30(e, other_cell.cast()) as i8 {
        return true;
    }
    let other_inheritance = e.call(CELL_GET_INHERITANCE_FLAGS, &args![other_cell]).u32();
    if e.get(this, TESObjectCELL::iLightingTemplateInheritanceFlags) != other_inheritance {
        return true;
    }
    if e.call(CELL_GET_LIGHTING_TEMPLATE, &args![this]).u32()
        != e.call(CELL_GET_LIGHTING_TEMPLATE, &args![other_cell]).u32()
    {
        return true;
    }
    let this_height = e.get(this, TESObjectCELL::fWaterHeight);
    let other_height = e
        .mem
        .f32(other_cell.addr() + TESObjectCELL::fWaterHeight.off);
    if this_height != other_height {
        return true;
    }
    let (accessor, size) = if is_interior(e, cell) {
        (CELL_INTERIOR_DATA, 0x2cu32)
    } else {
        (CELL_EXTERIOR_DATA, 0xcu32)
    };
    let mine = e.call(accessor, &args![this]).u32();
    let theirs = e.call(accessor, &args![other_cell]).u32();
    if mine != 0 && theirs != 0 {
        let order = e.call(CRT_MEMCMP, &args![mine, theirs, size]).u32();
        if order != 0 {
            return true;
        }
    }
    let other_list = e.call(CELL_EXTRA_DATA_LIST, &args![other_cell]).ptr::<()>();
    let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
    if e.call(EXTRA_LIST_COMPARE_LIST, &args![list, other_list])
        .bool()
    {
        return true;
    }
    e.vcall(
        this.addr() + WATER_NOISE_TEXTURE_OFFSET,
        0xc,
        &args![other_cell.addr() + WATER_NOISE_TEXTURE_OFFSET],
    )
    .bool()
}

// Translated from 00543df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell's `BelongsInGroup` (virtual slot 0x110): whether the cell is
/// stored in the group `header` describes. `with_children` and `strict` are
/// the two byte arguments the code receives.
///
/// By group type (`header[3]`, label `header[2]`): 0 (top-level): for
/// `with_children`, `004854e0` (interior) or the world space's `BelongsInGroup`
/// (exterior); 1 (world children): an exterior cell whose world space
/// contains the label, when `with_children` or the cell is persistent;
/// 2 and 4: not persistent, `with_children`, and the label equals the block
/// key; 3 and 5: not persistent and the label equals the sub-block key.
/// False when `header` is null or not a group header.
pub fn fn_00543df0(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    header: Ptr,
    with_children: u8,
    strict: u8,
) -> bool {
    let cell: Ptr = this.cast();
    if header.is_null() || e.mem.u32(header.addr()) != e.global::<u32>(GROUP_TAG) {
        return false;
    }
    let world = world_space(e, cell);
    let label = e.mem.u32(header.addr() + 8);
    match e.mem.u32(header.addr() + 0xc) {
        0 => {
            if with_children != 0 {
                if is_interior(e, cell) {
                    return e
                        .call(
                            FORM_BELONGS_IN_GROUP_INTERIOR,
                            &args![this, header, with_children as u32, strict as u32],
                        )
                        .bool();
                }
                return e
                    .vcall(
                        world.addr(),
                        0x110,
                        &args![header, with_children as u32, strict as u32],
                    )
                    .bool();
            }
            false
        }
        1 => {
            !is_interior(e, cell)
                && e.call(FORM_ID_IN_WORLD_SPACE, &args![world, label]).bool()
                && (with_children != 0 || persistent_flag(e, cell))
        }
        2 | 4 => !persistent_flag(e, cell) && with_children != 0 && label == fn_005441b0(e, this),
        3 | 5 => !persistent_flag(e, cell) && label == fn_00544210(e, this),
        _ => false,
    }
}

// Translated from 00543f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell's `CreateGroupData` (virtual slot 0x114): writes into `out` the
/// header of the group a cell goes into under the group `parent` describes
/// (or the top-level one when `parent` is null for an interior cell); `out`
/// is left with only its first word cleared when no group applies.
///
/// Exterior: parent type 0 with the exterior label gives type 1 (label: the
/// world space's form id); parent type 1 naming this cell's world space gives
/// type 4 (block key) and parent type 4 with this cell's block key gives
/// type 5 (sub-block key), both for non-persistent cells only. Interior:
/// no parent gives type 0 (label `011872b4`); parent type 0 with that label
/// gives type 2 (block key); parent type 2 with this cell's block key gives
/// type 3 (sub-block key).
pub fn fn_00543f50(e: &mut Engine, this: Ptr<TESObjectCELL>, out: Ptr, parent: Ptr) {
    let cell: Ptr = this.cast();
    if out.is_null() {
        return;
    }
    e.mem.set_u32(out.addr(), 0);
    if !is_interior(e, cell) {
        if parent.is_null() {
            return;
        }
        let label = e.mem.u32(parent.addr() + 8);
        match e.mem.u32(parent.addr() + 0xc) {
            0 => {
                if label == e.global::<u32>(EXTERIOR_PARENT_LABEL) {
                    let world = world_space(e, cell);
                    let id = form_id(e, world);
                    write_group_header(e, out, 1, id);
                }
            }
            1 => {
                let world = world_space(e, cell);
                if label == form_id(e, world) && !persistent_flag(e, cell) {
                    let key = fn_005441b0(e, this);
                    write_group_header(e, out, 4, key);
                }
            }
            4 if label == fn_005441b0(e, this) && !persistent_flag(e, cell) => {
                let key = fn_00544210(e, this);
                write_group_header(e, out, 5, key);
            }
            _ => {}
        }
    } else if parent.is_null() {
        let label = e.global::<u32>(INTERIOR_PARENT_LABEL);
        write_group_header(e, out, 0, label);
    } else {
        let label = e.mem.u32(parent.addr() + 8);
        match e.mem.u32(parent.addr() + 0xc) {
            0 if label == e.global::<u32>(INTERIOR_PARENT_LABEL) => {
                let key = fn_005441b0(e, this);
                write_group_header(e, out, 2, key);
            }
            2 if label == fn_005441b0(e, this) => {
                let key = fn_00544210(e, this);
                write_group_header(e, out, 3, key);
            }
            _ => {}
        }
    }
}

// Translated from 005441b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell's block key: for an exterior cell
/// `CalcExtGroupBlockKey(x, y)`; for an interior one the last decimal digit
/// of the form id's low 24 bits.
pub fn fn_005441b0(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    let cell: Ptr = this.cast();
    if !is_interior(e, cell) {
        let y = e.call(CELL_GET_DATA_Y, &args![cell]).i32();
        let x = e.call(CELL_GET_DATA_X, &args![cell]).i32();
        tes_object_cell_calc_ext_group_block_key(e, x, y)
    } else {
        (form_id(e, cell) & 0x00ff_ffff) % 10
    }
}

// Translated from 00544210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell's sub-block key: for an exterior cell
/// `CalcExtGroupSubBlockKey(x, y)`; for an interior one the second-to-last
/// decimal digit of the form id's low 24 bits.
pub fn fn_00544210(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    let cell: Ptr = this.cast();
    if !is_interior(e, cell) {
        let y = e.call(CELL_GET_DATA_Y, &args![cell]).i32();
        let x = e.call(CELL_GET_DATA_X, &args![cell]).i32();
        tes_object_cell_calc_ext_group_sub_block_key(e, x, y)
    } else {
        ((form_id(e, cell) & 0x00ff_ffff) % 100) / 10
    }
}

// Translated from 00544280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::CalcExtGroupBlockKey` (Xbox PDB): packs the block
/// coordinates `x >> 5` and `y >> 5` (arithmetic shifts, 16 bits each).
pub fn tes_object_cell_calc_ext_group_block_key(e: &mut Engine, x: i32, y: i32) -> u32 {
    let (block_x, block_y) = ((x >> 5) as u16, (y >> 5) as u16);
    e.call(
        PACK_BLOCK_COORDINATES,
        &args![block_x as u32, block_y as u32],
    )
    .u32()
}

// Translated from 005442c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::CalcExtGroupSubBlockKey` (Xbox PDB): packs `x >> 3` and
/// `y >> 3` (arithmetic shifts, 16 bits each).
pub fn tes_object_cell_calc_ext_group_sub_block_key(e: &mut Engine, x: i32, y: i32) -> u32 {
    let (block_x, block_y) = ((x >> 3) as u16, (y >> 3) as u16);
    e.call(
        PACK_BLOCK_COORDINATES,
        &args![block_x as u32, block_y as u32],
    )
    .u32()
}

// Translated from 00544300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::SetInterior` (Xbox PDB): sets or clears bit 0 of the cell
/// flags.
pub fn tes_object_cell_set_interior(e: &mut Engine, this: Ptr<TESObjectCELL>, interior: bool) {
    let flags = e.get(this, TESObjectCELL::cCellFlags);
    let flags = if interior { flags | 1 } else { flags & !1 };
    e.set(this, TESObjectCELL::cCellFlags, flags);
}

// Translated from 00544340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 0x20 of the cell flags, then calls the form's virtual
/// slot 0x48 with 2 (`AddChange`).
pub fn fn_00544340(e: &mut Engine, this: Ptr<TESObjectCELL>, set: bool) {
    let flags = e.get(this, TESObjectCELL::cCellFlags);
    let flags = if set { flags | 0x20 } else { flags & !0x20 };
    e.set(this, TESObjectCELL::cCellFlags, flags);
    e.vcall(this.addr(), 0x48, &args![2u32]);
}

// Translated from 00544390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::SetTempPublic` (Xbox PDB): sets or clears bit 0x40 of the
/// cell flags, then calls the form's virtual slot 0x48 with 2.
pub fn tes_object_cell_set_temp_public(e: &mut Engine, this: Ptr<TESObjectCELL>, set: bool) {
    let flags = e.get(this, TESObjectCELL::cCellFlags);
    let flags = if set { flags | 0x40 } else { flags & !0x40 };
    e.set(this, TESObjectCELL::cCellFlags, flags);
    e.vcall(this.addr(), 0x48, &args![2u32]);
}

// Translated from 005443e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether cell flag 0x20 (`00502180`) or cell flag 0x40 (`fn_00544420`) is
/// set.
pub fn fn_005443e0(e: &mut Engine, this: Ptr<TESObjectCELL>) -> bool {
    e.call(CELL_FLAG_20, &args![this]).bool() || fn_00544420(e, this)
}

// Translated from 00544420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cell flag 0x40 (`SetTempPublic`'s bit).
pub fn fn_00544420(e: &mut Engine, this: Ptr<TESObjectCELL>) -> bool {
    e.get(this, TESObjectCELL::cCellFlags) & 0x40 != 0
}

// Translated from 00544440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 1 of `cCellGameFlags`, then calls the form's virtual
/// slot 0x48 with 2.
pub fn fn_00544440(e: &mut Engine, this: Ptr<TESObjectCELL>, set: bool) {
    let flags = e.get(this, TESObjectCELL::cCellGameFlags);
    let flags = if set { flags | 1 } else { flags & !1 };
    e.set(this, TESObjectCELL::cCellGameFlags, flags);
    e.vcall(this.addr(), 0x48, &args![2u32]);
}

// Translated from 00544490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether form flag 0x20000 is set (`iFormFlags`, through `0044ddc0`).
pub fn fn_00544490(e: &mut Engine, this: Ptr) -> bool {
    e.call(FORM_FLAGS, &args![this]).u32() & 0x20000 != 0
}

// ---- the loaded data of a cell and the interior lighting accessors ----
//
// Functions `005444c0` to `00545960`. `pLoadedData` (`LOADED_CELL_DATA`,
// 0x64 bytes) is built by `fn_00544ce0` when a cell's references are loaded
// and holds the lists and maps the reference handlers below fill. The
// interior lighting accessors read `INTERIOR_DATA` (`pCellData` of an
// interior cell), or the lighting template's value when the cell inherits
// that field (a bit of `iLightingTemplateInheritanceFlags`).

/// `TESWorldSpace` test of form flag 0x80000 (`iFormFlags`).
pub(crate) const WORLD_SPACE_FLAG_80000: u32 = 0x0058_6230;
/// `TESWorldSpace` test of bit 2 of the byte at `+0x4C`.
pub(crate) const WORLD_SPACE_FLAG_4C_2: u32 = 0x0058_6210;
/// `INTERIOR_DATA` constructor (a cell's interior data, 0x2C bytes).
pub(crate) const INTERIOR_DATA_CONSTRUCT: u32 = 0x0052_6640;
/// `EXTERIOR_DATA` constructor (a cell's exterior data, 0xC bytes).
pub(crate) const EXTERIOR_DATA_CONSTRUCT: u32 = 0x0054_0720;
/// Whether the cell inherits a field of its lighting template: tests `mask`
/// against `iLightingTemplateInheritanceFlags`.
pub(crate) const CELL_INHERITS_LIGHTING_FIELD: u32 = 0x0055_8b80;
/// The lighting template's accessors for the fields the cell can inherit.
pub(crate) const TEMPLATE_AMBIENT: u32 = WORD_AT_18;
pub(crate) const TEMPLATE_DIRECTIONAL: u32 = 0x0044_1110;
pub(crate) const TEMPLATE_FOG: u32 = 0x007a_f430;
pub(crate) const TEMPLATE_FOG_NEAR: u32 = 0x0052_6ac0;
pub(crate) const TEMPLATE_FOG_FAR: u32 = 0x0052_6ae0;
pub(crate) const TEMPLATE_DIRECTIONAL_XY: u32 = 0x0055_b980;
pub(crate) const TEMPLATE_DIRECTIONAL_Z: u32 = 0x0067_1d10;
pub(crate) const TEMPLATE_CLIP_DISTANCE: u32 = 0x009a_9350;
pub(crate) const TEMPLATE_FOG_POWER: u32 = 0x0059_8040;
/// `255.0` (`double`): the divisor that turns a colour byte into a float.
pub(crate) const COLOUR_BYTE_SCALE: u32 = 0x0101_e568;
/// `3000.0` (`double`): bound size above which an emittance reference is a
/// large animated reference.
pub(crate) const LARGE_BOUND_SIZE: u32 = 0x0102_ed48;
/// `LOADED_CELL_DATA` constructors: the map constructors (hash size
/// argument) and the matching destructors.
pub(crate) const MAP_CONSTRUCT_REFERENCE_NODE: u32 = 0x0055_8d40;
pub(crate) const MAP_CONSTRUCT_FORM_REFERENCE: u32 = 0x0055_8d70;
pub(crate) const MAP_CONSTRUCT_MULTI_BOUND: u32 = 0x0055_8da0;
pub(crate) const MAP_DESTRUCT_REFERENCE_NODE: u32 = 0x0055_8f20;
pub(crate) const MAP_DESTRUCT_FORM_REFERENCE: u32 = 0x0055_9020;
pub(crate) const MAP_DESTRUCT_MULTI_BOUND: u32 = 0x0055_91d0;
/// `NiTMapBase` operations: `SetAt(key, value)`, `RemoveAt(key)`,
/// `GetAt(key, &value)`, the first-slot iterator and
/// `GetNext(&position, &key, &value)`.
pub(crate) const MAP_SET_AT: u32 = 0x0084_4700;
pub(crate) const MAP_REMOVE_AT: u32 = 0x0040_5430;
pub(crate) const MAP_GET_AT: u32 = 0x006c_62d0;
pub(crate) const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
pub(crate) const MAP_GET_NEXT: u32 = 0x0055_9120;
/// `SetAt(key, smart pointer by value)` on the multibound map.
pub(crate) const MAP_SET_AT_MULTI_BOUND: u32 = 0x006c_6920;
/// `BSSimpleList` operations: remove the item whose address is passed, and
/// insert after a node; and `pop front` (removes the head).
pub(crate) const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
pub(crate) const LIST_INSERT_AFTER: u32 = 0x0090_5820;
pub(crate) const LIST_POP_FRONT: u32 = 0x0063_f7b0;
/// The word at `+4` of an object (also the list-node `next` accessor).
pub(crate) const WORD_AT_4: u32 = 0x0072_6070;
/// The word at `+0x18` of an object (the owner of the cell's 3D node).
pub(crate) const WORD_AT_18: u32 = 0x0096_11e0;
/// Reference predicates and accessors (`TESObjectREFR`).
pub(crate) const REFERENCE_IS_MARKER_FORM: u32 = 0x0043_9f90;
pub(crate) const REFERENCE_EXTRA_DATA_LIST: u32 = 0x005d_43c0;
pub(crate) const REFERENCE_LINKED_NODE_OWNER: u32 = 0x0056_9ac0;
pub(crate) const REFERENCE_EMITTANCE_SOURCE: u32 = 0x0056_9580;
pub(crate) const REFERENCE_IS_SCRIPTED: u32 = 0x0056_56d0;
pub(crate) const REFERENCE_IS_ACTIVATING_CHILDREN: u32 = 0x0056_a250;
pub(crate) const REFERENCE_GET_BASE_FORM: u32 = 0x007a_f430;
pub(crate) const REFERENCE_GET_MULTI_BOUND_ROOM: u32 = 0x0056_99b0;
pub(crate) const REFERENCE_GET_MULTI_BOUND: u32 = 0x0056_9920;
pub(crate) const REFERENCE_CLEAR_MULTI_BOUND: u32 = 0x0056_9990;
pub(crate) const REFERENCE_GET_ORIENTATION: u32 = 0x0056_fa00;
/// `TESBoundObject::GetBoundSize` (Xbox PDB), `float` in ST0.
pub(crate) const FORM_GET_BOUND_SIZE: u32 = 0x0050_ebf0;
/// `cFormType` of a form (`00401170`) and the type of lights.
pub(crate) const FORM_TYPE_LIGHT: u32 = 0x1e;
/// `ExtraDataList::GetRoom` / `GetPrimitive` (Xbox PDB).
pub(crate) const EXTRA_DATA_GET_ROOM: u32 = 0x0042_0ed0;
pub(crate) const EXTRA_DATA_GET_PRIMITIVE: u32 = 0x0041_fbe0;
/// Multibound node and multibound data accessors.
pub(crate) const MULTI_BOUND_DATA_OF_ROOM: u32 = 0x0066_29f0;
pub(crate) const MULTI_BOUND_SET_DATA: u32 = 0x0043_9920;
pub(crate) const MULTI_BOUND_PRIMITIVE_SLOT: u32 = 0x0043_b230;
pub(crate) const MULTI_BOUND_SET_PRIMITIVE: u32 = 0x004a_ddc0;
pub(crate) const PRIMITIVE_SET_ROTATION: u32 = 0x0043_96b0;
/// Allocates a block for a node (cdecl `size`), and the
/// `BSMultiBoundNode` constructor.
pub(crate) const NODE_ALLOCATE: u32 = 0x00aa_13e0;
pub(crate) const MULTI_BOUND_NODE_CONSTRUCT: u32 = 0x00c4_6970;
/// `BSShaderManager::CloneMaterialPropertyRecurse` (Xbox PDB), cdecl.
pub(crate) const CLONE_MATERIAL_PROPERTY: u32 = 0x00b5_7c60;
/// `NiAVObject::UpdateProperties` / `GetProperty` (Xbox PDB).
pub(crate) const NODE_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
pub(crate) const NODE_GET_PROPERTY: u32 = 0x00a5_9d30;
/// Applies a property to a node (cdecl `(node, property)`).
pub(crate) const SHADER_APPLY_PROPERTY: u32 = 0x00b5_5480;
/// `TESWorldSpace` (`this`, cell): looks the cell's grid coordinates up in
/// the map at `+0x68` of the world space and returns the entry (a list of
/// references), 0 without the map or an entry.
pub(crate) const WORLD_SPACE_CELL_REFERENCES: u32 = 0x0058_7870;
/// Tests bit 2 of the word at `+0x244` of the object it is called on (the
/// game loader singleton at `011ddf38`).
pub(crate) const LOADER_FLAG_244_2: u32 = 0x0042_ce10;
/// Run on the cell by `fn_00545030` before it asks the owner of the cell's
/// 3D node to remove the node (a function of the unit's later range).
pub(crate) const CELL_DETACH_LOADED_3D: u32 = 0x0054_5c10;
/// The form pointer `fn_00545740` compares with a reference's base form (the
/// other marker form, `011ca230`, is only tested by `00439f90`).
pub(crate) const MARKER_FORM_SECOND: u32 = 0x011c_a238;
/// Singleton whose `+0x760` word `fn_005453b0` falls back to.
pub(crate) const EMITTANCE_FALLBACK_OWNER_POINTER: u32 = 0x011d_ea3c;

layout! {
    /// `EXTERIOR_DATA` (Xbox PDB), 0xC bytes: the data of an exterior cell.
    pub struct ExteriorData: 0x0c {
        /// `iCellX` (Xbox PDB).
        0x00 iCellX: i32,
        /// `iCellY` (Xbox PDB).
        0x04 iCellY: i32,
        /// `cLandHideFlags` (Xbox PDB).
        0x08 cLandHideFlags: i8,
    }

    /// `INTERIOR_DATA` (Xbox PDB), 0x2C bytes: the data of an interior cell.
    pub struct InteriorData: 0x2c {
        /// `iAmbient` (Xbox PDB): packed colour.
        0x00 iAmbient: u32,
        /// `iDirectional` (Xbox PDB): packed colour.
        0x04 iDirectional: u32,
        /// `iFog` (Xbox PDB): packed colour.
        0x08 iFog: u32,
        /// `fFogNear` (Xbox PDB).
        0x0C fFogNear: f32,
        /// `fFogFar` (Xbox PDB).
        0x10 fFogFar: f32,
        /// `iDirectionalXY` (Xbox PDB).
        0x14 iDirectionalXY: u32,
        /// `iDirectionalZ` (Xbox PDB).
        0x18 iDirectionalZ: u32,
        /// `fDirectionalFade` (Xbox PDB).
        0x1C fDirectionalFade: f32,
        /// `fClipDist` (Xbox PDB).
        0x20 fClipDist: f32,
        /// `fFogPower` (Xbox PDB).
        0x24 fFogPower: f32,
        /// `iInteriorOffset` (Xbox PDB).
        0x28 iInteriorOffset: u32,
    }

    /// `NiTMap<K, V>` (Xbox PDB), 0x10 bytes for every `K` and `V`.
    pub struct NiTMap: 0x10 {
        /// `m_uiHashSize` (Xbox PDB).
        0x04 m_uiHashSize: u32,
        /// `m_ppkHashTable` (Xbox PDB).
        0x08 m_ppkHashTable: u32,
    }

    /// `LOADED_CELL_DATA` (Xbox PDB), 0x64 bytes (the same on the PC).
    pub struct LoadedCellData: 0x64 {
        /// `spCell3D` (Xbox PDB): `NiPointer<NiNode>` slot.
        0x00 spCell3D: Ptr,
        /// `LargeAnimatedRefs` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x04 LargeAnimatedRefs: Inline<BSSimpleList>,
        /// `AnimatedRefMap` (Xbox PDB): `NiTMap<TESObjectREFR *, NiNode *>`.
        0x0C AnimatedRefMap: Inline<NiTMap>,
        /// `EmittanceSourceRefMap` (Xbox PDB):
        /// `NiTMap<TESForm *, TESObjectREFR *>`.
        0x1C EmittanceSourceRefMap: Inline<NiTMap>,
        /// `EmittanceLightRefMap` (Xbox PDB):
        /// `NiTMap<TESObjectREFR *, NiNode *>`.
        0x2C EmittanceLightRefMap: Inline<NiTMap>,
        /// `MultiboundRefMap` (Xbox PDB):
        /// `NiTMap<TESObjectREFR *, NiPointer<BSMultiBoundNode>>`.
        0x3C MultiboundRefMap: Inline<NiTMap>,
        /// `ScriptedRefs` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x4C ScriptedRefs: Inline<BSSimpleList>,
        /// `ActivatingRefs` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x54 ActivatingRefs: Inline<BSSimpleList>,
        /// `WaterRefs` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x5C WaterRefs: Inline<BSSimpleList>,
    }
}

/// The address of the embedded sub-object `field` of a loaded-data block.
pub(crate) fn loaded_part<U>(
    loaded: Ptr<LoadedCellData>,
    field: Field<LoadedCellData, Inline<U>>,
) -> Ptr {
    loaded.byte_add(field.off)
}

/// The cell's `pLoadedData`.
pub(crate) fn loaded_data(e: &Engine, this: Ptr<TESObjectCELL>) -> Ptr<LoadedCellData> {
    e.get(this, TESObjectCELL::pLoadedData).cast()
}

/// Calls a list operation that takes the address of an item
/// (`BSSimpleList::AddHead`, `Remove`, `InsertAfter`): the game passes the
/// address of a local holding the item.
pub(crate) fn list_operation(e: &mut Engine, function: u32, list: Ptr, item: Ptr) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item.addr());
        e.call(function, &args![list, slot]);
    });
}

// Translated from 005444c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For an interior cell: whether form flag 0x80000 is set; for an exterior
/// cell: the same flag of its world space, false without one.
pub fn fn_005444c0(e: &mut Engine, this: Ptr<TESObjectCELL>) -> bool {
    if is_interior(e, this.cast()) {
        e.call(FORM_FLAGS, &args![this]).u32() & 0x80000 != 0
    } else {
        let world = world_space(e, this.cast());
        !world.is_null() && e.call(WORLD_SPACE_FLAG_80000, &args![world]).bool()
    }
}

// Translated from 00544520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 2 of `cCellFlags`, inverted for an interior cell; when that is false,
/// the world space's bit 2 of the byte at `+0x4C`, false without a world
/// space.
pub fn fn_00544520(e: &mut Engine, this: Ptr<TESObjectCELL>) -> bool {
    let mut flag = e.get(this, TESObjectCELL::cCellFlags) & 0x4 != 0;
    if is_interior(e, this.cast()) {
        flag = !flag;
    }
    if flag {
        return true;
    }
    let world = world_space(e, this.cast());
    !world.is_null() && e.call(WORLD_SPACE_FLAG_4C_2, &args![world]).bool()
}

// Translated from 00544590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `bit & 31` of the exterior data's `cLandHideFlags` (a `char`,
/// sign-extended) is set; false for a cell without exterior data.
pub fn fn_00544590(e: &mut Engine, this: Ptr<TESObjectCELL>, bit: u32) -> bool {
    let data: Ptr<ExteriorData> = fn_005445d0(e, this).cast();
    if data.is_null() {
        return false;
    }
    let mask = 1u32 << (bit & 0x1f);
    let flags = e.get(data, ExteriorData::cLandHideFlags) as i32 as u32;
    flags & mask != 0
}

// Translated from 005445d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The exterior data (`pCellData`) of an exterior cell, null for an
/// interior one.
pub fn fn_005445d0(e: &mut Engine, this: Ptr<TESObjectCELL>) -> Ptr {
    if is_interior(e, this.cast()) {
        Ptr::NULL
    } else {
        e.get(this, TESObjectCELL::pCellData)
    }
}

// Translated from 00544600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interior data (`pCellData`) of an interior cell, null for an
/// exterior one.
pub fn fn_00544600(e: &mut Engine, this: Ptr<TESObjectCELL>) -> Ptr {
    if is_interior(e, this.cast()) {
        e.get(this, TESObjectCELL::pCellData)
    } else {
        Ptr::NULL
    }
}

// Translated from 00544630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::CreateCellData` (Xbox PDB): frees the old `pCellData`
/// (the code frees it the same way for an interior and an exterior cell) and
/// builds a new one: `INTERIOR_DATA` (0x2C bytes) for an interior cell,
/// `EXTERIOR_DATA` (0xC bytes) otherwise. The exception-unwinding frame is
/// not translated.
pub fn tes_object_cell_create_cell_data(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let old = e.get(this, TESObjectCELL::pCellData);
    if !old.is_null() {
        // Both arms of the interior test free the same pointer.
        is_interior(e, this.cast());
        e.call(DEALLOCATE, &args![old]);
        e.set(this, TESObjectCELL::pCellData, Ptr::NULL);
    }
    let (size, construct) = if is_interior(e, this.cast()) {
        (InteriorData::SIZE, INTERIOR_DATA_CONSTRUCT)
    } else {
        (ExteriorData::SIZE, EXTERIOR_DATA_CONSTRUCT)
    };
    let block = e.call(ALLOCATE, &args![size]).ptr::<()>();
    let data = if block.is_null() {
        Ptr::NULL
    } else {
        e.call(construct, &args![block]).ptr()
    };
    e.set(this, TESObjectCELL::pCellData, data);
}

/// The lighting template of the cell when the cell inherits the field
/// selected by `mask` and has a template (the code asks for the template
/// twice).
pub(crate) fn inherited_template(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    mask: u32,
) -> Option<Ptr> {
    if e.call(CELL_INHERITS_LIGHTING_FIELD, &args![this, mask])
        .bool()
        && !e
            .call(CELL_GET_LIGHTING_TEMPLATE, &args![this])
            .ptr::<()>()
            .is_null()
    {
        Some(e.call(CELL_GET_LIGHTING_TEMPLATE, &args![this]).ptr())
    } else {
        None
    }
}

/// A word of the interior data, or the lighting template's value for it
/// (`template_accessor`) when the cell inherits it (`mask`); 0 for a cell
/// without interior data.
pub(crate) fn interior_word(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    mask: u32,
    field: Field<InteriorData, u32>,
    template_accessor: u32,
) -> u32 {
    let data: Ptr<InteriorData> = fn_00544600(e, this).cast();
    if data.is_null() {
        return 0;
    }
    match inherited_template(e, this, mask) {
        Some(template) => e.call(template_accessor, &args![template]).u32(),
        None => e.get(data, field),
    }
}

/// The `float` counterpart of [`interior_word`]; `missing` is the value for
/// a cell without interior data.
pub(crate) fn interior_float(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    mask: u32,
    field: Field<InteriorData, f32>,
    template_accessor: u32,
    missing: f32,
) -> f32 {
    let data: Ptr<InteriorData> = fn_00544600(e, this).cast();
    if data.is_null() {
        return missing;
    }
    match inherited_template(e, this, mask) {
        Some(template) => e.call(template_accessor, &args![template]).f32(),
        None => e.get(data, field),
    }
}

/// Writes the three channels of a packed colour (bytes 0, 1 and 2) as
/// `float`s in `0..=1` at `out`.
pub(crate) fn unpack_colour(e: &mut Engine, packed: u32, out: Ptr) {
    let scale: f64 = e.global(COLOUR_BYTE_SCALE);
    let channels = [packed & 0xff, (packed >> 8) & 0xff, (packed >> 16) & 0xff];
    for (i, channel) in channels.into_iter().enumerate() {
        e.mem
            .set_f32(out.addr() + 4 * i as u32, (channel as f64 / scale) as f32);
    }
}

// Translated from 00544750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interior ambient colour (`iAmbient`, packed), from the lighting
/// template when the cell inherits it (inheritance bit 0x1).
pub fn fn_00544750(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    interior_word(e, this, 0x1, InteriorData::iAmbient, TEMPLATE_AMBIENT)
}

// Translated from 005447b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The ambient colour as three `float`s at `out`.
pub fn fn_005447b0(e: &mut Engine, this: Ptr<TESObjectCELL>, out: Ptr) {
    let packed = fn_00544750(e, this);
    unpack_colour(e, packed, out);
}

// Translated from 00544830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interior directional colour (`iDirectional`, packed), from the
/// lighting template when the cell inherits it (bit 0x2).
pub fn fn_00544830(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    interior_word(
        e,
        this,
        0x2,
        InteriorData::iDirectional,
        TEMPLATE_DIRECTIONAL,
    )
}

// Translated from 00544890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The directional colour as three `float`s at `out`.
pub fn fn_00544890(e: &mut Engine, this: Ptr<TESObjectCELL>, out: Ptr) {
    let packed = fn_00544830(e, this);
    unpack_colour(e, packed, out);
}

// Translated from 00544910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interior `iDirectionalXY`, from the lighting template when the cell
/// inherits it (bit 0x20).
pub fn fn_00544910(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    interior_word(
        e,
        this,
        0x20,
        InteriorData::iDirectionalXY,
        TEMPLATE_DIRECTIONAL_XY,
    )
}

// Translated from 00544970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interior `iDirectionalZ`, from the lighting template when the cell
/// inherits it (bit 0x20, the same bit as `iDirectionalXY`).
pub fn fn_00544970(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    interior_word(
        e,
        this,
        0x20,
        InteriorData::iDirectionalZ,
        TEMPLATE_DIRECTIONAL_Z,
    )
}

// Translated from 005449d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interior fog colour (`iFog`, packed), from the lighting template when
/// the cell inherits it (bit 0x4).
pub fn fn_005449d0(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    interior_word(e, this, 0x4, InteriorData::iFog, TEMPLATE_FOG)
}

// Translated from 00544a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fog colour as three `float`s at `out`.
pub fn fn_00544a30(e: &mut Engine, this: Ptr<TESObjectCELL>, out: Ptr) {
    let packed = fn_005449d0(e, this);
    unpack_colour(e, packed, out);
}

// Translated from 00544ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interior `fFogNear` (0.0 without interior data), from the lighting
/// template when the cell inherits it (bit 0x8).
pub fn fn_00544ab0(e: &mut Engine, this: Ptr<TESObjectCELL>) -> f32 {
    interior_float(e, this, 0x8, InteriorData::fFogNear, TEMPLATE_FOG_NEAR, 0.0)
}

// Translated from 00544b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interior `fFogFar` (0.0 without interior data), from the lighting
/// template when the cell inherits it (bit 0x10).
pub fn fn_00544b10(e: &mut Engine, this: Ptr<TESObjectCELL>) -> f32 {
    interior_float(e, this, 0x10, InteriorData::fFogFar, TEMPLATE_FOG_FAR, 0.0)
}

// Translated from 00544b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interior `fFogPower` (1.0 without interior data), from the lighting
/// template when the cell inherits it (bit 0x100).
pub fn fn_00544b70(e: &mut Engine, this: Ptr<TESObjectCELL>) -> f32 {
    interior_float(
        e,
        this,
        0x100,
        InteriorData::fFogPower,
        TEMPLATE_FOG_POWER,
        1.0,
    )
}

// Translated from 00544bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interior `fClipDist` (0.0 without interior data), from the lighting
/// template when the cell inherits it (bit 0x80).
pub fn fn_00544bd0(e: &mut Engine, this: Ptr<TESObjectCELL>) -> f32 {
    interior_float(
        e,
        this,
        0x80,
        InteriorData::fClipDist,
        TEMPLATE_CLIP_DISTANCE,
        0.0,
    )
}

// Translated from 00544c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetDataX` (Xbox PDB): `iCellX` of the exterior data, 0 for
/// a cell without it.
pub fn tes_object_cell_get_data_x(e: &mut Engine, this: Ptr<TESObjectCELL>) -> i32 {
    let data: Ptr<ExteriorData> = fn_005445d0(e, this).cast();
    if data.is_null() {
        0
    } else {
        e.get(data, ExteriorData::iCellX)
    }
}

// Translated from 00544c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetDataY` (Xbox PDB): `iCellY` of the exterior data, 0 for
/// a cell without it.
pub fn tes_object_cell_get_data_y(e: &mut Engine, this: Ptr<TESObjectCELL>) -> i32 {
    let data: Ptr<ExteriorData> = fn_005445d0(e, this).cast();
    if data.is_null() {
        0
    } else {
        e.get(data, ExteriorData::iCellY)
    }
}

// Translated from 00544c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::SetDataCoord` (Xbox PDB): stores the grid coordinates in
/// the exterior data; nothing for an interior cell or one without data.
pub fn tes_object_cell_set_data_coord(e: &mut Engine, this: Ptr<TESObjectCELL>, x: i32, y: i32) {
    if is_interior(e, this.cast()) {
        return;
    }
    let data: Ptr<ExteriorData> = fn_005445d0(e, this).cast();
    if !data.is_null() {
        e.set(data, ExteriorData::iCellX, x);
        e.set(data, ExteriorData::iCellY, y);
    }
}

// Translated from 00544ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the cell's loaded data (when it has none) and sorts its
/// references into it. Under the reference lock: every reference that is not
/// flag 0x20 (or is while bit 2 of the game loader's word at `+0x244` is
/// set) is handled by kind. A reference whose base form is one of the two
/// marker forms gets its multibound node (`fn_00545740`) and, when it has a
/// linked-node owner (`00569ac0`), is queued in a local list. Any other
/// reference that is not an actor but is scripted (`005656d0`) is added to
/// the loaded data's `ScriptedRefs`; one that is activating children
/// (`TESObjectREFR::IsActivatingChildren`) to `ActivatingRefs`. After the
/// lock is left, each queued reference's node and its owner's node (both
/// through `fn_00545960`) are linked through the owner node's virtual slot
/// 0xDC, and the world space's references for the cell get their multibound
/// nodes. The exception-unwinding frame is not translated.
pub fn fn_00544ce0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    if loaded_data(e, this).is_null() {
        let block = e.call(ALLOCATE, &args![LoadedCellData::SIZE]).ptr::<()>();
        let created = if block.is_null() {
            Ptr::NULL
        } else {
            fn_00544f60(e, block.cast()).cast()
        };
        e.set(this, TESObjectCELL::pLoadedData, created);
    }
    fn_00541ac0(e, this);
    // Locals: the queue (a `BSSimpleList`, 8 bytes) and the reference whose
    // address the list calls take.
    e.with_stack(0xc, |e, frame| {
        let queue = frame;
        let held = frame.byte_add(8);
        e.call(LIST_CONSTRUCT, &args![queue]);

        let mut node = reference_list(e, this.cast());
        while !list_is_end(e, node) {
            let reference = list_item(e, node);
            e.mem.set_u32(held.addr(), reference.addr());
            node = list_next(e, node);
            if flag_20(e, reference)
                && !e
                    .call(
                        LOADER_FLAG_244_2,
                        &args![e.global::<u32>(GAME_LOADER_POINTER)],
                    )
                    .bool()
            {
                continue;
            }
            if e.call(REFERENCE_IS_MARKER_FORM, &args![reference]).bool() {
                fn_00545740(e, this, reference.cast());
                if e.call(REFERENCE_LINKED_NODE_OWNER, &args![reference]).u32() != 0 {
                    e.call(LIST_PUSH_FRONT, &args![queue, held]);
                }
            } else {
                let loaded = loaded_data(e, this);
                if !e.vcall(reference.addr(), 0x100, &[]).bool()
                    && e.call(REFERENCE_IS_SCRIPTED, &args![reference]).bool()
                {
                    let scripted = loaded_part(loaded, LoadedCellData::ScriptedRefs);
                    e.call(LIST_PUSH_FRONT, &args![scripted, held]);
                }
                if e.call(REFERENCE_IS_ACTIVATING_CHILDREN, &args![reference])
                    .bool()
                {
                    let activating = loaded_part(loaded, LoadedCellData::ActivatingRefs);
                    e.call(LIST_PUSH_FRONT, &args![activating, held]);
                }
            }
        }
        fn_00541ae0(e, this);

        while !e.call(LIST_IS_END, &args![queue]).bool() {
            let address = e.call(LIST_ITEM_ADDRESS, &args![queue]).u32();
            let reference = Ptr::<()>::new(e.mem.u32(address));
            e.call(LIST_POP_FRONT, &args![queue]);
            let owner = e.call(REFERENCE_LINKED_NODE_OWNER, &args![reference]).ptr();
            let reference_node = fn_00545960(e, this, reference.cast());
            let owner_node = fn_00545960(e, this, owner);
            if reference_node != owner_node {
                e.vcall(owner_node.addr(), 0xdc, &args![reference_node, 1u32]);
            }
        }

        let world = world_space(e, this.cast());
        if !world.is_null() {
            let mut node = e
                .call(WORLD_SPACE_CELL_REFERENCES, &args![world, this])
                .ptr();
            while !list_is_end(e, node) {
                let reference = list_item(e, node);
                fn_00545740(e, this, reference.cast());
                node = list_next(e, node);
            }
        }
        e.call(LIST_DESTRUCT, &args![queue]);
    });
}

// Translated from 00544f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LOADED_CELL_DATA` constructor: the null `spCell3D` slot, the three
/// reference lists, and the four maps (0x25 buckets each). Returns `this`.
/// The exception-unwinding frame is not translated.
pub fn fn_00544f60(e: &mut Engine, this: Ptr<LoadedCellData>) -> Ptr<LoadedCellData> {
    e.call(SLOT_CONSTRUCT, &args![this, 0u32]);
    e.call(
        LIST_CONSTRUCT,
        &args![loaded_part(this, LoadedCellData::LargeAnimatedRefs)],
    );
    let maps = [
        (LoadedCellData::AnimatedRefMap, MAP_CONSTRUCT_REFERENCE_NODE),
        (
            LoadedCellData::EmittanceSourceRefMap,
            MAP_CONSTRUCT_FORM_REFERENCE,
        ),
        (
            LoadedCellData::EmittanceLightRefMap,
            MAP_CONSTRUCT_REFERENCE_NODE,
        ),
        (LoadedCellData::MultiboundRefMap, MAP_CONSTRUCT_MULTI_BOUND),
    ];
    for (map, construct) in maps {
        e.call(construct, &args![loaded_part(this, map), 0x25u32]);
    }
    for list in [
        LoadedCellData::ScriptedRefs,
        LoadedCellData::ActivatingRefs,
        LoadedCellData::WaterRefs,
    ] {
        e.call(LIST_CONSTRUCT, &args![loaded_part(this, list)]);
    }
    this
}

// Translated from 00545030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the cell's loaded data: empties `ScriptedRefs` and
/// `ActivatingRefs`; walks the multibound map, and for every entry that has
/// a node whose word at `+4` is 2 (the map holds the last other reference)
/// calls `00569990` on the key; then, if the cell has a 3D node, runs
/// `00545c10` and asks the owner of that node (`009611e0`) to remove it
/// (virtual slot 0xE8); finally destroys and frees the loaded data and
/// clears `pLoadedData`. The exception-unwinding frame is not translated.
pub fn fn_00545030(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let loaded = loaded_data(e, this);
    if loaded.is_null() {
        return;
    }
    e.call(
        LIST_CLEAR,
        &args![loaded_part(loaded, LoadedCellData::ScriptedRefs)],
    );
    e.call(
        LIST_CLEAR,
        &args![loaded_part(loaded, LoadedCellData::ActivatingRefs)],
    );
    let map = loaded_part(loaded, LoadedCellData::MultiboundRefMap);
    // Locals: the iteration position, the key and the value slot.
    e.with_stack(0xc, |e, frame| {
        let position = frame;
        let key = frame.byte_add(4);
        let slot = frame.byte_add(8);
        let first = e.call(MAP_FIRST_POSITION, &args![map]).u32();
        e.mem.set_u32(position.addr(), first);
        while e.mem.u32(position.addr()) != 0 {
            e.call(SLOT_CONSTRUCT, &args![slot, 0u32]);
            e.mem.set_u32(key.addr(), 0);
            e.call(MAP_GET_NEXT, &args![map, position, key, slot]);
            let reference = e.mem.u32(key.addr());
            if reference != 0 && !slot_get(e, slot).is_null() {
                let node = slot_get(e, slot);
                let owner = e.call(MULTI_BOUND_DATA_OF_ROOM, &args![node]).u32();
                if e.call(WORD_AT_4, &args![owner]).u32() == 2 {
                    e.call(REFERENCE_CLEAR_MULTI_BOUND, &args![reference]);
                }
            }
            e.call(SLOT_RELEASE, &args![slot]);
        }
    });
    if !slot_get(e, loaded.cast()).is_null() {
        e.call(CELL_DETACH_LOADED_3D, &args![this]);
        let node = slot_get(e, loaded.cast());
        let owner = e.call(WORD_AT_18, &args![node]).ptr::<()>();
        if !owner.is_null() {
            let node = slot_get(e, loaded.cast());
            e.vcall(owner.addr(), 0xe8, &args![node]);
        }
    }
    let loaded = loaded_data(e, this);
    if !loaded.is_null() {
        fn_005451d0(e, loaded, 1);
    }
    e.set(this, TESObjectCELL::pLoadedData, Ptr::NULL);
}

// Translated from 005451d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LOADED_CELL_DATA` scalar deleting destructor: destroys the members
/// (`fn_00545200`) and frees the block when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_005451d0(e: &mut Engine, this: Ptr<LoadedCellData>, flags: u32) -> Ptr<LoadedCellData> {
    fn_00545200(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 00545200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LOADED_CELL_DATA` destructor: the lists and maps in reverse order of
/// construction, then the `spCell3D` slot. The exception-unwinding frame is
/// not translated.
pub fn fn_00545200(e: &mut Engine, this: Ptr<LoadedCellData>) {
    for list in [
        LoadedCellData::WaterRefs,
        LoadedCellData::ActivatingRefs,
        LoadedCellData::ScriptedRefs,
    ] {
        e.call(LIST_DESTRUCT, &args![loaded_part(this, list)]);
    }
    let maps = [
        (LoadedCellData::MultiboundRefMap, MAP_DESTRUCT_MULTI_BOUND),
        (
            LoadedCellData::EmittanceLightRefMap,
            MAP_DESTRUCT_REFERENCE_NODE,
        ),
        (
            LoadedCellData::EmittanceSourceRefMap,
            MAP_DESTRUCT_FORM_REFERENCE,
        ),
        (LoadedCellData::AnimatedRefMap, MAP_DESTRUCT_REFERENCE_NODE),
    ];
    for (map, destruct) in maps {
        e.call(destruct, &args![loaded_part(this, map)]);
    }
    e.call(
        LIST_DESTRUCT,
        &args![loaded_part(this, LoadedCellData::LargeAnimatedRefs)],
    );
    e.call(SLOT_RELEASE, &args![this]);
}

// Translated from 005452c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers an emittance reference: when the reference has a node (virtual
/// slot 0x1D0), maps the reference to the value its node's virtual slot 0xC
/// returns in `AnimatedRefMap`, and when its base form's bound size
/// (`TESBoundObject::GetBoundSize`) exceeds 3000.0 adds it to
/// `LargeAnimatedRefs`.
pub fn fn_005452c0(e: &mut Engine, this: Ptr<TESObjectCELL>, reference: Ptr<TESObjectREFR>) {
    let loaded = loaded_data(e, this);
    if loaded.is_null() || reference.is_null() {
        return;
    }
    if e.vcall(reference.addr(), 0x1d0, &[]).u32() == 0 {
        return;
    }
    let node = e.vcall(reference.addr(), 0x1d0, &[]).ptr::<()>();
    let value = e.vcall(node.addr(), 0xc, &[]).u32();
    e.call(
        MAP_SET_AT,
        &args![
            loaded_part(loaded, LoadedCellData::AnimatedRefMap),
            reference,
            value
        ],
    );
    let form = e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32();
    let bound_size = e.call(FORM_GET_BOUND_SIZE, &args![form]).f64();
    let limit: f64 = e.global(LARGE_BOUND_SIZE);
    if bound_size > limit {
        let list = loaded_part(loaded, LoadedCellData::LargeAnimatedRefs);
        list_operation(e, LIST_PUSH_FRONT, list, reference.cast());
    }
}

// Translated from 00545360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::RemoveEmittanceRef` (Xbox PDB): removes the reference from
/// `AnimatedRefMap` and from `LargeAnimatedRefs`.
pub fn tes_object_cell_remove_emittance_ref(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    reference: Ptr<TESObjectREFR>,
) {
    let loaded = loaded_data(e, this);
    if loaded.is_null() {
        return;
    }
    e.call(
        MAP_REMOVE_AT,
        &args![
            loaded_part(loaded, LoadedCellData::AnimatedRefMap),
            reference
        ],
    );
    let list = loaded_part(loaded, LoadedCellData::LargeAnimatedRefs);
    list_operation(e, LIST_REMOVE_ITEM, list, reference.cast());
}

// Translated from 005453b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers a light or emittance source: for a reference with a node
/// (virtual slot 0x1D0), a light (base form type 0x1E) is mapped in
/// `EmittanceLightRefMap` to the value of its node's virtual slot 0xC;
/// any other base form has its node's materials cloned
/// (`CloneMaterialPropertyRecurse`) and properties updated, and, when its
/// emittance source (`00569580`, or the `+0x760` word of the singleton at
/// `011dea3c`) has a value (virtual slot 0xC0), the node's property 2 is
/// fetched, the shader applies it (`00b55480(node, value)`), and the
/// source is mapped to the reference in `EmittanceSourceRefMap`.
pub fn fn_005453b0(e: &mut Engine, this: Ptr<TESObjectCELL>, reference: Ptr<TESObjectREFR>) {
    let loaded = loaded_data(e, this);
    if loaded.is_null() || reference.is_null() {
        return;
    }
    if e.vcall(reference.addr(), 0x1d0, &[]).u32() == 0 {
        return;
    }
    let node = e.vcall(reference.addr(), 0x1d0, &[]).ptr::<()>();
    let form = e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32();
    if e.call(FORM_TYPE, &args![form]).u32() == FORM_TYPE_LIGHT {
        let value = e.vcall(node.addr(), 0xc, &[]).u32();
        e.call(
            MAP_SET_AT,
            &args![
                loaded_part(loaded, LoadedCellData::EmittanceLightRefMap),
                reference,
                value
            ],
        );
        return;
    }
    e.call(CLONE_MATERIAL_PROPERTY, &args![node]);
    e.call(NODE_UPDATE_PROPERTIES, &args![node]);
    let mut source = e
        .call(REFERENCE_EMITTANCE_SOURCE, &args![reference])
        .ptr::<()>();
    if source.is_null() {
        let owner = Ptr::new(e.global::<u32>(EMITTANCE_FALLBACK_OWNER_POINTER));
        source = fn_005454d0(e, owner);
    }
    if source.is_null() {
        return;
    }
    let value = e.vcall(source.addr(), 0xc0, &[]).u32();
    if value == 0 {
        return;
    }
    e.call(NODE_GET_PROPERTY, &args![node, 2u32]);
    e.call(SHADER_APPLY_PROPERTY, &args![node, value]);
    e.call(
        MAP_SET_AT,
        &args![
            loaded_part(loaded, LoadedCellData::EmittanceSourceRefMap),
            source,
            reference
        ],
    );
}

// Translated from 005454d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x760` of the object (a field of the singleton at
/// `011dea3c`).
pub fn fn_005454d0(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0x760))
}

// Translated from 005454f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Undoes `fn_005453b0`: a light (base form type 0x1E) is removed from
/// `EmittanceLightRefMap` by the reference; any other reference is removed
/// from `EmittanceSourceRefMap` by its emittance source (`00569580`) when it
/// has one.
pub fn fn_005454f0(e: &mut Engine, this: Ptr<TESObjectCELL>, reference: Ptr<TESObjectREFR>) {
    let loaded = loaded_data(e, this);
    if loaded.is_null() {
        return;
    }
    let form = e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32();
    if e.call(FORM_TYPE, &args![form]).u32() == FORM_TYPE_LIGHT {
        e.call(
            MAP_REMOVE_AT,
            &args![
                loaded_part(loaded, LoadedCellData::EmittanceLightRefMap),
                reference
            ],
        );
    } else {
        let source = e.call(REFERENCE_EMITTANCE_SOURCE, &args![reference]).u32();
        if source != 0 {
            e.call(
                MAP_REMOVE_AT,
                &args![
                    loaded_part(loaded, LoadedCellData::EmittanceSourceRefMap),
                    source
                ],
            );
        }
    }
}

// Translated from 00545560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the reference to the front of `ScriptedRefs`.
pub fn fn_00545560(e: &mut Engine, this: Ptr<TESObjectCELL>, reference: Ptr<TESObjectREFR>) {
    let loaded = loaded_data(e, this);
    if !loaded.is_null() {
        let list = loaded_part(loaded, LoadedCellData::ScriptedRefs);
        list_operation(e, LIST_PUSH_FRONT, list, reference.cast());
    }
}

// Translated from 00545590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the reference from `ScriptedRefs`.
pub fn fn_00545590(e: &mut Engine, this: Ptr<TESObjectCELL>, reference: Ptr<TESObjectREFR>) {
    let loaded = loaded_data(e, this);
    if !loaded.is_null() {
        let list = loaded_part(loaded, LoadedCellData::ScriptedRefs);
        list_operation(e, LIST_REMOVE_ITEM, list, reference.cast());
    }
}

// Translated from 005455c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::AddActivatingRef` (Xbox PDB): unless the reference is
/// already in `ActivatingRefs`, calls its virtual slot 0x48 with 0x4000000
/// and links it in after the last node of the list (at the front when the
/// list is empty).
pub fn tes_object_cell_add_activating_ref(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    reference: Ptr<TESObjectREFR>,
) {
    let loaded = loaded_data(e, this);
    if loaded.is_null() {
        return;
    }
    let list = loaded_part(loaded, LoadedCellData::ActivatingRefs);
    let mut node = list;
    let mut previous = Ptr::<()>::NULL;
    while !list_is_end(e, node) {
        if list_item(e, node).addr() == reference.addr() {
            return;
        }
        previous = node;
        node = list_next(e, node);
    }
    e.vcall(reference.addr(), 0x48, &args![0x0400_0000u32]);
    if previous.is_null() {
        list_operation(e, LIST_PUSH_FRONT, list, reference.cast());
    } else {
        list_operation(e, LIST_INSERT_AFTER, previous, reference.cast());
    }
}

// Translated from 00545670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the reference from `ActivatingRefs` and calls its virtual slot
/// 0x4C with 0x4000000 (the reference's change flag).
pub fn fn_00545670(e: &mut Engine, this: Ptr<TESObjectCELL>, reference: Ptr<TESObjectREFR>) {
    let loaded = loaded_data(e, this);
    if !loaded.is_null() {
        let list = loaded_part(loaded, LoadedCellData::ActivatingRefs);
        list_operation(e, LIST_REMOVE_ITEM, list, reference.cast());
    }
    e.vcall(reference.addr(), 0x4c, &args![0x0400_0000u32]);
}

// Translated from 005456b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the reference to the front of `WaterRefs`.
pub fn fn_005456b0(e: &mut Engine, this: Ptr<TESObjectCELL>, reference: Ptr<TESObjectREFR>) {
    let loaded = loaded_data(e, this);
    if !loaded.is_null() {
        let list = loaded_part(loaded, LoadedCellData::WaterRefs);
        list_operation(e, LIST_PUSH_FRONT, list, reference.cast());
    }
}

// Translated from 005456e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the reference from `WaterRefs`.
pub fn fn_005456e0(e: &mut Engine, this: Ptr<TESObjectCELL>, reference: Ptr<TESObjectREFR>) {
    let loaded = loaded_data(e, this);
    if !loaded.is_null() {
        let list = loaded_part(loaded, LoadedCellData::WaterRefs);
        list_operation(e, LIST_REMOVE_ITEM, list, reference.cast());
    }
}

// Translated from 00545710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the loaded data's `WaterRefs` list, null without loaded
/// data.
pub fn fn_00545710(e: &mut Engine, this: Ptr<TESObjectCELL>) -> Ptr {
    let loaded = loaded_data(e, this);
    if loaded.is_null() {
        Ptr::NULL
    } else {
        loaded_part(loaded, LoadedCellData::WaterRefs)
    }
}

// Translated from 00545740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives a marker reference its multibound node and registers it. Does
/// nothing (null) for a cell without loaded data, a null reference or one
/// whose base form is not one of the two marker forms (`00439f90`). The room
/// node is the reference's `ExtraDataList::GetRoom`; without one, a
/// reference of the second marker form uses its `GetMultiBoundRoom` (null
/// ends the function), any other gets a new `BSMultiBoundNode` (0xB4 bytes)
/// that is given the reference's `GetMultiBound`. The node's primitive (the
/// bound shape: the multibound data's slot, or one built by the reference's
/// `ExtraDataList::GetPrimitive` from the three words at `+0x24` of the reference) gets the reference's
/// virtual slot 0x1F4 value (slot 0xB8), and, when its type (slot 0x8C) is 2,
/// the reference's orientation. The room is then stored in
/// `MultiboundRefMap` under the reference, its virtual slot 0xBC is called,
/// and it is returned. The exception-unwinding frame is not translated.
pub fn fn_00545740(e: &mut Engine, this: Ptr<TESObjectCELL>, reference: Ptr<TESObjectREFR>) -> Ptr {
    let loaded = loaded_data(e, this);
    if loaded.is_null()
        || reference.is_null()
        || !e.call(REFERENCE_IS_MARKER_FORM, &args![reference]).bool()
    {
        return Ptr::NULL;
    }
    let extra_list = e.call(REFERENCE_EXTRA_DATA_LIST, &args![reference]).u32();
    let mut room = e.call(EXTRA_DATA_GET_ROOM, &args![extra_list]).ptr::<()>();
    let multi_bound: Ptr;
    if room.is_null() {
        let form = e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32();
        if form == e.global::<u32>(MARKER_FORM_SECOND) {
            room = e
                .call(REFERENCE_GET_MULTI_BOUND_ROOM, &args![reference])
                .ptr();
            if room.is_null() {
                return Ptr::NULL;
            }
            multi_bound = e.call(MULTI_BOUND_DATA_OF_ROOM, &args![room]).ptr();
        } else {
            let block = e.call(NODE_ALLOCATE, &args![0xb4u32]).ptr::<()>();
            room = if block.is_null() {
                Ptr::NULL
            } else {
                e.call(MULTI_BOUND_NODE_CONSTRUCT, &args![block]).ptr()
            };
            multi_bound = e.call(REFERENCE_GET_MULTI_BOUND, &args![reference]).ptr();
            e.call(MULTI_BOUND_SET_DATA, &args![room, multi_bound]);
        }
    } else {
        multi_bound = e.call(MULTI_BOUND_DATA_OF_ROOM, &args![room]).ptr();
    }
    let mut primitive = e
        .call(MULTI_BOUND_PRIMITIVE_SLOT, &args![multi_bound])
        .ptr::<()>();
    if primitive.is_null() {
        let extra_list = e.call(REFERENCE_EXTRA_DATA_LIST, &args![reference]).u32();
        let shape = e
            .call(EXTRA_DATA_GET_PRIMITIVE, &args![extra_list])
            .ptr::<()>();
        if !shape.is_null() {
            let rotation = e.call(REFERENCE_ROTATION, &args![reference]).u32();
            // The three words are copied to a local and passed by address.
            primitive = e.with_stack(12, |e, copy| {
                for word in 0..3 {
                    let value = e.mem.u32(rotation + 4 * word);
                    e.mem.set_u32(copy.addr() + 4 * word, value);
                }
                e.vcall(shape.addr(), 0x14, &args![copy]).ptr()
            });
            e.call(MULTI_BOUND_SET_PRIMITIVE, &args![multi_bound, primitive]);
        }
    }
    if !primitive.is_null() {
        let transform = e.vcall(reference.addr(), 0x1f4, &[]).u32();
        e.vcall(primitive.addr(), 0xb8, &args![transform]);
        if e.vcall(primitive.addr(), 0x8c, &[]).u32() == 2 {
            // `NiMatrix3` out parameter, 9 words.
            e.with_stack(0x24, |e, matrix| {
                let orientation = e
                    .call(REFERENCE_GET_ORIENTATION, &args![reference, matrix])
                    .u32();
                e.call(PRIMITIVE_SET_ROTATION, &args![primitive, orientation]);
            });
        }
        e.call(MULTI_BOUND_SET_DATA, &args![room, multi_bound]);
        // The room goes into the map as a `NiPointer` passed by value.
        e.with_stack(4, |e, pointer| {
            e.call(SLOT_CONSTRUCT, &args![pointer, room]);
            let value = e.mem.u32(pointer.addr());
            let map = loaded_part(loaded, LoadedCellData::MultiboundRefMap);
            e.call(MAP_SET_AT_MULTI_BOUND, &args![map, reference, value]);
        });
    }
    e.vcall(room.addr(), 0xbc, &[]);
    room
}

// Translated from 00545960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The multibound node of a marker reference: looked up in `MultiboundRefMap`
/// (`GetAt`), and, when absent or null, built by `fn_00545740` and stored in
/// a local `NiPointer` slot that is released at the end. Null for a cell
/// without loaded data, a null reference or a reference that is not a marker.
/// The exception-unwinding frame is not translated.
pub fn fn_00545960(e: &mut Engine, this: Ptr<TESObjectCELL>, reference: Ptr<TESObjectREFR>) -> Ptr {
    let loaded = loaded_data(e, this);
    if loaded.is_null()
        || reference.is_null()
        || !e.call(REFERENCE_IS_MARKER_FORM, &args![reference]).bool()
    {
        return Ptr::NULL;
    }
    e.with_stack(4, |e, slot| {
        e.call(SLOT_CONSTRUCT, &args![slot, 0u32]);
        let map = loaded_part(loaded, LoadedCellData::MultiboundRefMap);
        let found = e.call(MAP_GET_AT, &args![map, reference, slot]).bool();
        if !found || slot_get(e, slot).is_null() {
            let built = fn_00545740(e, this, reference);
            slot_assign(e, slot, built);
        }
        let result = slot_get(e, slot);
        e.call(SLOT_RELEASE, &args![slot]);
        result
    })
}

// ---- the cell's 3D, its ownership, land, regions and water ----
//
// Functions `00545a30` to `00547610`. The 3D of a cell is a `NiNode` that
// `Load3D` builds (the root, its child nodes, the multibound nodes and the
// markers) and stores in `LOADED_CELL_DATA::spCell3D`. The accessors that
// follow read the cell's extra data list (`this + 0x28`, through `004610d0`)
// for the owner and the detach time, or the cell's land, region list, water
// height and acoustic space. Six small functions of the range (`00546780`,
// `005467c0`, `005467e0`, `00546890`, `005468b0`, `005468d0`) belong to the
// scene graph node class (they write the node's flags, bound and a field at
// `+0xB0`); the compiler emitted them in this unit.

/// The child of the node the cell's `Get3D` returns, at the given index
/// (`00456fc0`, `ret 4`): `0043b4a0(node, index)`.
pub(crate) const CELL_CHILD_NODE: u32 = 0x0045_6fc0;
/// The allocation scope object (a 4-byte local): `(this, 0x1A or 0x1B, 1,
/// file, line)`, and its destructor.
pub(crate) const SCOPE_ENTER: u32 = 0x0040_4eb0;
pub(crate) const SCOPE_LEAVE: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\TESObjectCELL.cpp"`.
pub(crate) const SOURCE_FILE: u32 = 0x0102_ed68;
/// `"Shared Portal Geometry"`.
pub(crate) const PORTAL_GEOMETRY_NAME: u32 = 0x0102_ed50;
/// Three zero floats, and the 3 x 3 identity matrix (nine floats).
pub(crate) const ZERO_VECTOR: u32 = 0x011f_426c;
pub(crate) const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// `NiNode` constructor (`(this, 0)`, 0xAC bytes).
pub(crate) const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// Copies the three words at the argument to `this + 0x58`, and the nine
/// words at the argument to `this + 0x34` (the node's local translation and
/// rotation).
pub(crate) const NODE_SET_TRANSLATION: u32 = 0x0044_0460;
pub(crate) const NODE_SET_ROTATION: u32 = 0x0043_fa80;
/// Sets or clears the bit given by the second argument in the word at
/// `this + 0x30` (the node's flags): `(this, set, mask)`; `00450f90` is it
/// with the mask 1.
pub(crate) const NODE_SET_FLAG_BITS: u32 = 0x0043_b370;
pub(crate) const NODE_SET_FLAG: u32 = 0x0045_0f90;
/// `NiNode::AttachChild(child, replace)` is the node's virtual slot `0xDC`.
pub(crate) const NODE_ATTACH_CHILD_SLOT: u32 = 0xdc;
/// The 9-byte update data (`(this, time, byte, byte)`) and the node's
/// update with it.
pub(crate) const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
pub(crate) const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `BSMultiBound` (0x10 bytes) and `BSMultiBoundAABB` (0x24 bytes)
/// constructors.
pub(crate) const MULTI_BOUND_CONSTRUCT: u32 = 0x00c3_5e00;
pub(crate) const MULTI_BOUND_AABB_CONSTRUCT: u32 = 0x00c3_7ec0;
/// The fixed string constructor `(this, text)` (returns `this`), its
/// destructor, and the node's set-name.
pub(crate) const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
pub(crate) const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
pub(crate) const NODE_SET_NAME: u32 = 0x00a5_b950;
/// `cCellState` setter (`(this, byte)`, `MOV [ECX+0x26],AL`).
pub(crate) const CELL_SET_STATE: u32 = 0x0045_12a0;
/// The work `Load3D` runs on the new cell 3D.
pub(crate) const CELL_FINISH_3D: u32 = 0x0055_2dc0;
/// `BGSSaveLoadGame::LoadCell` (Xbox PDB), `(loader, cell)`; and the loader's
/// follow-up call `(loader, 0)`.
pub(crate) const SAVE_LOAD_CELL: u32 = 0x0084_9b10;
pub(crate) const LOADER_AFTER_LOAD: u32 = 0x0084_92b0;
/// `BSPortalGraph` constructor (0x78 bytes), and the world space's portal
/// graph (`this + 0x6C` slot, built on first use).
pub(crate) const PORTAL_GRAPH_CONSTRUCT: u32 = 0x00c5_a9d0;
pub(crate) const WORLD_SPACE_PORTAL_GRAPH: u32 = 0x0058_31d0;
/// Releases the world space's portal graph slot (when its word at `+4` is 1).
pub(crate) const WORLD_SPACE_RELEASE_PORTAL_GRAPH: u32 = 0x0058_3270;
/// The entry `n` of the table at `011f91c8` (cdecl), the word at `+0x1E0` of
/// that entry (the current portal graph), and its setter `(entry, graph)`.
pub(crate) const GLOBAL_TABLE_ENTRY: u32 = 0x0045_0b80;
pub(crate) const CURRENT_PORTAL_GRAPH: u32 = 0x0045_4b30;
pub(crate) const SET_CURRENT_PORTAL_GRAPH: u32 = 0x00b5_ddf0;
/// Extra data accessors on `ExtraDataList` (`this` = the list):
/// the owner (extra data type 0x21), its setter, the ownership global
/// (`ExtraDataList::GetGlobal`) and rank (`ExtraDataList::GetRank`), the
/// detach time (type 0xB) and its setter, the extra data of type 1 and its
/// reset, the one of type 0x74, 7 and 8, and the acoustic space (type 0x81).
pub(crate) const EXTRA_LIST_GET_OWNER: u32 = 0x0041_8660;
pub(crate) const EXTRA_LIST_SET_OWNER: u32 = 0x0041_9700;
pub(crate) const EXTRA_LIST_GET_GLOBAL: u32 = 0x0041_8690;
pub(crate) const EXTRA_LIST_GET_RANK: u32 = 0x0041_86c0;
pub(crate) const EXTRA_LIST_GET_DETACH_TIME: u32 = 0x0042_1820;
pub(crate) const EXTRA_LIST_SET_DETACH_TIME: u32 = 0x0042_1850;
pub(crate) const EXTRA_LIST_GET_TYPE_1: u32 = 0x0041_b9a0;
pub(crate) const EXTRA_LIST_RESET_TYPE_1: u32 = 0x0041_b8d0;
pub(crate) const EXTRA_LIST_GET_TYPE_74: u32 = 0x0042_1c30;
pub(crate) const EXTRA_LIST_GET_TYPE_7: u32 = 0x0041_bde0;
pub(crate) const EXTRA_LIST_GET_TYPE_8: u32 = 0x0041_c260;
pub(crate) const EXTRA_LIST_GET_ACOUSTIC_SPACE: u32 = 0x0041_c160;
pub(crate) const EXTRA_LIST_SET_REGION_LIST: u32 = 0x0041_bbd0;
/// `TESRegionList` constructor `(this, 0)`, 0x10 bytes.
pub(crate) const REGION_LIST_CONSTRUCT: u32 = 0x004f_6320;
/// `TESObjectLAND` constructor (0x2C bytes).
pub(crate) const LAND_CONSTRUCT: u32 = 0x0053_3120;
/// The world space's accessors: the word at `+0x18` of a form is the
/// owner's `WORD_AT_18`; the world space's owner-form (`00458400`), the
/// default water height (`004536e0`, the float at `+0xCC`), and the three
/// accessors used by `00546c20`, `005474b0` and `005475b0`.
pub(crate) const WORLD_SPACE_OWNER_FORM: u32 = 0x0045_8400;
pub(crate) const WORLD_SPACE_WATER_HEIGHT: u32 = 0x0045_36e0;
pub(crate) const WORLD_SPACE_TYPE_7_VALUE: u32 = 0x0058_6150;
pub(crate) const WORLD_SPACE_ACOUSTIC_VALUE: u32 = 0x0058_5fe0;
/// Tests bit 1 of `cCellFlags` (`004518e0`) and bit 7 (`00454b10`).
pub(crate) const CELL_FLAG_2: u32 = 0x0045_18e0;
pub(crate) const CELL_FLAG_80: u32 = 0x0045_4b10;
/// The word at `+0xC4` of a cell (`pLoadedData`).
pub(crate) const CELL_LOADED_DATA: u32 = 0x0070_58c0;
/// `TESActorBaseData::GetFactionRank` (Xbox PDB): `(this, faction, flag)`.
pub(crate) const ACTOR_BASE_GET_FACTION_RANK: u32 = 0x0047_d680;
/// `0040eb10(a, b, c)` (cdecl, three floats): whether `a` and `b` differ by
/// no more than `c`.
pub(crate) const FLOAT_NEAR: u32 = 0x0040_eb10;
/// Sets the radius (float at `+0xC`) of the bound `this` (`(this, radius)`),
/// and copies three words and a radius into it (`(this, vector, radius)`);
/// `006240d0` is the body `00546890` calls.
pub(crate) const BOUND_SET_RADIUS: u32 = 0x0063_f790;
pub(crate) const BOUND_SET_CENTER_AND_RADIUS: u32 = 0x004a_51d0;
pub(crate) const BOUND_BASE_CONSTRUCT: u32 = 0x0062_40d0;
/// `00867e30(this)`: the integer `00546c70` passes to `fn_00546b10`, computed
/// from the object at `011de7b8`.
pub(crate) const DETACH_TIME_SOURCE: u32 = 0x0086_7e30;
pub(crate) const DETACH_TIME_SOURCE_OBJECT: u32 = 0x011d_e7b8;
/// The child of a node at an index (`(node, index)`, `ret 4`), and the
/// address of the sub-object at `+0xC` of a multibound primitive.
pub(crate) const NODE_CHILD_AT: u32 = 0x0043_b4a0;
pub(crate) const PRIMITIVE_PART: u32 = 0x0048_d150;
/// The owner `00546ca0`, `00546da0` and `00546ee0` compare the actor with.
pub(crate) const ACTOR_SINGLETON_POINTER: u32 = 0x011d_ea3c;
/// RTTI type descriptors of the ownership checks: the source types
/// `00546ca0` casts the actor's form from and `00546ee0` casts the form
/// from, and the two targets (the owner is a character base or a faction).
pub(crate) const RTTI_OWNER_SOURCE_FORM: u32 = 0x0118_3108;
pub(crate) const RTTI_OWNER_SOURCE_ARGUMENT: u32 = 0x0118_46e8;
pub(crate) const RTTI_OWNER_CHARACTER: u32 = 0x0118_3a1c;
pub(crate) const RTTI_OWNER_FACTION: u32 = 0x0118_4704;
/// The word at `011c9520` that `00546a90` returns.
pub(crate) const OWNER_EXCLUDED_FORM: u32 = 0x011c_9520;
/// Statics of `fn_005474b0`: the guard word, a copy of the three words at
/// `011f426c`, and the last value stored.
pub(crate) const STATIC_RECORD_GUARD: u32 = 0x011c_a1d0;
pub(crate) const STATIC_RECORD_COPY: u32 = 0x011c_a1c4;
pub(crate) const STATIC_RECORD_LAST: u32 = 0x011c_a1c0;
/// `-FLT_MAX` (float); `FLT_MAX` as a double (the "unset" water height); and
/// `0.001` (float).
pub(crate) const LOWEST_FLOAT: u32 = 0x0101_5f5c;
pub(crate) const UNSET_WATER_HEIGHT_DOUBLE: u32 = 0x0102_31b0;
pub(crate) const WATER_HEIGHT_TOLERANCE: u32 = 0x0101_7d00;

/// Allocates (cdecl `size`) and constructs a `NiNode` with the node
/// constructor, as every `new NiNode` of `Load3D` does. Null when the
/// allocation fails.
pub(crate) fn new_ni_node(e: &mut Engine) -> Ptr {
    let block = e.call(NODE_ALLOCATE, &args![0xacu32]).ptr::<()>();
    if block.is_null() {
        Ptr::NULL
    } else {
        e.call(NODE_CONSTRUCT, &args![block, 0u32]).ptr()
    }
}

/// `parent->AttachChild(child, true)` (virtual slot `0xDC`).
pub(crate) fn attach_child(e: &mut Engine, parent: Ptr, child: Ptr) {
    e.vcall(parent.addr(), NODE_ATTACH_CHILD_SLOT, &args![child, 1u32]);
}

/// What `Load3D` does to a new node before it gets its flag: the two flag
/// setters, in this order.
pub(crate) fn prepare_node(e: &mut Engine, node: Ptr) {
    fn_00546780(e, node, true);
    fn_005467c0(e, node, true);
}

// Translated from 00545a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::AttachMultiBoundNodes` (Xbox PDB): for every entry of the
/// loaded data's `MultiboundRefMap` whose node has no parent (the word at
/// `+0x18`), attaches the node to the child at index 7 of the cell's 3D
/// (`AttachChild(node, true)`, virtual slot `0xDC`). Does nothing without
/// that child or loaded data. The exception-unwinding frame is not
/// translated.
pub fn tes_object_cell_attach_multi_bound_nodes(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let parent = e.call(CELL_CHILD_NODE, &args![this, 7u32]).ptr::<()>();
    if parent.is_null() || loaded_data(e, this).is_null() {
        return;
    }
    // Locals: the iteration position, the key and the value slot.
    e.with_stack(0xc, |e, frame| {
        let position = frame;
        let key = frame.byte_add(4);
        let slot = frame.byte_add(8);
        let loaded = loaded_data(e, this);
        let map = loaded_part(loaded, LoadedCellData::MultiboundRefMap);
        let first = e.call(MAP_FIRST_POSITION, &args![map]).u32();
        e.mem.set_u32(position.addr(), first);
        while e.mem.u32(position.addr()) != 0 {
            e.call(SLOT_CONSTRUCT, &args![slot, 0u32]);
            e.mem.set_u32(key.addr(), 0);
            let loaded = loaded_data(e, this);
            let map = loaded_part(loaded, LoadedCellData::MultiboundRefMap);
            e.call(MAP_GET_NEXT, &args![map, position, key, slot]);
            if !slot_get(e, slot).is_null() {
                let node = slot_get(e, slot);
                if e.call(WORD_AT_18, &args![node]).u32() == 0 {
                    let node = slot_get(e, slot);
                    attach_child(e, parent, node);
                }
            }
            e.call(SLOT_RELEASE, &args![slot]);
        }
    });
}

// Translated from 00545b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell's portal graph (`spPortalGraph`, `this + 0xD4`), built on first
/// use: an interior cell gets a new 0x78-byte object, an exterior cell the
/// graph of its world space. Returns the graph. The exception-unwinding
/// frame is not translated.
pub fn fn_00545b30(e: &mut Engine, this: Ptr<TESObjectCELL>) -> Ptr {
    let slot = this.byte_add(TESObjectCELL::spPortalGraph.off);
    if slot_get(e, slot).is_null() {
        if is_interior(e, this.cast()) {
            let block = e.call(NODE_ALLOCATE, &args![0x78u32]).ptr::<()>();
            let graph = if block.is_null() {
                Ptr::NULL
            } else {
                e.call(PORTAL_GRAPH_CONSTRUCT, &args![block]).ptr::<()>()
            };
            slot_assign(e, slot, graph);
        } else {
            let world = world_space(e, this.cast());
            let graph = e.call(WORLD_SPACE_PORTAL_GRAPH, &args![world]).ptr::<()>();
            slot_assign(e, slot, graph);
        }
    }
    slot_get(e, slot)
}

// Translated from 00545c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops the cell's portal graph. With a graph: an interior cell whose graph
/// is the current one (the word at `+0x1E0` of the table entry 0) resets the
/// current graph to null; the slot is cleared; a cell with a world space
/// releases the world space's graph slot (`00583270`).
pub fn fn_00545c10(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let slot = this.byte_add(TESObjectCELL::spPortalGraph.off);
    if slot_get(e, slot).is_null() {
        return;
    }
    if is_interior(e, this.cast()) {
        let entry = e.call(GLOBAL_TABLE_ENTRY, &args![0u32]).ptr::<()>();
        let current = e.call(CURRENT_PORTAL_GRAPH, &args![entry]).u32();
        if current == slot_get(e, slot).addr() {
            let entry = e.call(GLOBAL_TABLE_ENTRY, &args![0u32]).ptr::<()>();
            e.call(SET_CURRENT_PORTAL_GRAPH, &args![entry, 0u32]);
        }
    }
    slot_assign(e, slot, Ptr::NULL);
    if !world_space(e, this.cast()).is_null() {
        let world = world_space(e, this.cast());
        e.call(WORLD_SPACE_RELEASE_PORTAL_GRAPH, &args![world]);
    }
}

// Translated from 00545cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::Get3D` (Xbox PDB): the cell's 3D node (`spCell3D`, the
/// first slot of the loaded data), null without loaded data.
pub fn tes_object_cell_get_3d(e: &mut Engine, this: Ptr<TESObjectCELL>) -> Ptr {
    let loaded = loaded_data(e, this);
    if loaded.is_null() {
        Ptr::NULL
    } else {
        slot_get(e, loaded.cast())
    }
}

// Translated from 00545cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::Load3D` (Xbox PDB): the cell's 3D node. Returns the
/// existing one when the cell has it. Otherwise builds it: sets the cell
/// state to 2 and `bCellDetached` to false, creates the root `NiNode`
/// (translation zero, identity rotation) and its children, each with the two
/// flag setters (`00546780`, `005467c0`) and a flag from `005468f0(n)`
/// (`00450f90`): children 0, 1 (also `005468b0`), 2 (the multibound parent,
/// which gets four `BSMultiBoundNode`s with a `BSMultiBound` of a
/// `BSMultiBoundAABB` each and the field `+0xB0` set to 1), 3, a child with
/// flag 0, 5, 6, 7 and 8 (the last two attached twice), and the two markers
/// under child 1, whose slots are `spLightMarkerNode` (`005468f0(9)`) and
/// `spSoundMarkerNode` (`005468f0(0xA)`). The root is updated, the loaded
/// data is created and given the root (`spCell3D`), the multibound nodes are
/// attached and `00552dc0` runs; unless the loader reports bit 2 of its
/// `+0x244` word, the loader's flag is cleared around `LoadCell` and, when
/// bit 0x10 of that word is clear, the loader's follow-up call. The cell state
/// is then 3. In every case the portal graph is built and, when it holds no
/// geometry node (`fn_00546930`), gets a new `NiNode` named "Shared Portal
/// Geometry". The exception-unwinding frame is not translated.
pub fn tes_object_cell_load_3d(e: &mut Engine, this: Ptr<TESObjectCELL>) -> Ptr {
    let scope = Ptr::<()>::new(e.mem.alloc(4));
    e.call(
        SCOPE_ENTER,
        &args![scope, 0x1au32, 1u32, SOURCE_FILE, 0xd84u32],
    );
    let mut root = tes_object_cell_get_3d(e, this);
    if root.is_null() {
        e.call(CELL_SET_STATE, &args![this, 2u32]);
        e.set(this, TESObjectCELL::bCellDetached, false);
        root = new_ni_node(e);
        e.call(NODE_SET_TRANSLATION, &args![root, ZERO_VECTOR]);
        e.call(NODE_SET_ROTATION, &args![root, IDENTITY_MATRIX]);
        fn_005467c0(e, root, true);
        fn_00546780(e, root, true);
        // Child 0.
        let node = new_ni_node(e);
        prepare_node(e, node);
        let flag = fn_005468f0(e, 0);
        e.call(NODE_SET_FLAG, &args![node, flag]);
        attach_child(e, root, node);
        // Child 1, the parent of the two markers.
        let parent = new_ni_node(e);
        prepare_node(e, parent);
        fn_005468b0(e, parent, true);
        let flag = fn_005468f0(e, 1);
        e.call(NODE_SET_FLAG, &args![parent, flag]);
        attach_child(e, root, parent);
        // The light marker and the sound marker, kept in the cell's slots.
        for (index, slot) in [
            (9u8, TESObjectCELL::spLightMarkerNode),
            (0xa, TESObjectCELL::spSoundMarkerNode),
        ] {
            let node = new_ni_node(e);
            prepare_node(e, node);
            let flag = fn_005468f0(e, index);
            e.call(NODE_SET_FLAG, &args![node, flag]);
            attach_child(e, parent, node);
            slot_assign(e, this.byte_add(slot.off), node);
        }
        // Child 2, the parent of the multibound nodes.
        let multi_bound_parent = new_ni_node(e);
        prepare_node(e, multi_bound_parent);
        let flag = fn_005468f0(e, 2);
        e.call(NODE_SET_FLAG, &args![multi_bound_parent, flag]);
        attach_child(e, root, multi_bound_parent);
        for _ in 0..4 {
            let block = e.call(NODE_ALLOCATE, &args![0xb4u32]).ptr::<()>();
            let node = if block.is_null() {
                Ptr::NULL
            } else {
                e.call(MULTI_BOUND_NODE_CONSTRUCT, &args![block])
                    .ptr::<()>()
            };
            fn_005468d0(e, node, 1);
            let block = e.call(NODE_ALLOCATE, &args![0x10u32]).ptr::<()>();
            let bound = if block.is_null() {
                Ptr::NULL
            } else {
                e.call(MULTI_BOUND_CONSTRUCT, &args![block]).ptr::<()>()
            };
            let block = e.call(NODE_ALLOCATE, &args![0x24u32]).ptr::<()>();
            let shape = if block.is_null() {
                Ptr::NULL
            } else {
                e.call(MULTI_BOUND_AABB_CONSTRUCT, &args![block])
                    .ptr::<()>()
            };
            e.call(MULTI_BOUND_SET_PRIMITIVE, &args![bound, shape]);
            e.call(MULTI_BOUND_SET_DATA, &args![node, bound]);
            attach_child(e, multi_bound_parent, node);
        }
        // Children 3 to 8.
        let node = new_ni_node(e);
        prepare_node(e, node);
        let flag = fn_005468f0(e, 3);
        e.call(NODE_SET_FLAG, &args![node, flag]);
        attach_child(e, root, node);
        let node = new_ni_node(e);
        prepare_node(e, node);
        e.call(NODE_SET_FLAG, &args![node, 0u32]);
        attach_child(e, root, node);
        for (index, times) in [(5u8, 1), (6, 1), (7, 2), (8, 2)] {
            let node = new_ni_node(e);
            prepare_node(e, node);
            let flag = fn_005468f0(e, index);
            e.call(NODE_SET_FLAG, &args![node, flag]);
            for _ in 0..times {
                attach_child(e, root, node);
            }
        }
        // The root is updated with the 9-byte update data (time 0.0).
        e.with_stack(12, |e, update| {
            e.call(UPDATE_DATA_CONSTRUCT, &args![update, 0.0f32, 0u32, 0u32]);
            e.call(NODE_UPDATE, &args![root, update]);
        });
        fn_00544ce0(e, this);
        let loaded = loaded_data(e, this);
        slot_assign(e, loaded.cast(), root);
        tes_object_cell_attach_multi_bound_nodes(e, this);
        e.call(CELL_FINISH_3D, &args![this]);
        let loader: u32 = e.global(GAME_LOADER_POINTER);
        if !e.call(LOADER_FLAG_244_2, &args![loader]).bool() {
            let previous = e.call(GAME_LOADER_FLAG_SETTER, &args![loader, 0u32]).u8();
            e.call(SAVE_LOAD_CELL, &args![loader, this]);
            if !fn_00546950(e, Ptr::new(loader)) {
                e.call(LOADER_AFTER_LOAD, &args![loader, 0u32]);
            }
            e.call(GAME_LOADER_FLAG_SETTER, &args![loader, previous]);
        }
        e.call(CELL_SET_STATE, &args![this, 3u32]);
    }
    let graph = fn_00545b30(e, this);
    if fn_00546930(e, graph).is_null() {
        let geometry = new_ni_node(e);
        e.with_stack(4, |e, name| {
            let name = e
                .call(FIXED_STRING_CONSTRUCT, &args![name, PORTAL_GEOMETRY_NAME])
                .ptr::<()>();
            e.call(NODE_SET_NAME, &args![geometry, name]);
            e.call(FIXED_STRING_DESTRUCT, &args![name]);
        });
        fn_00546910(e, graph, geometry);
    }
    e.call(SCOPE_LEAVE, &args![scope]);
    e.mem.free(scope.addr());
    root
}

// Translated from 00546780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag bit 0x800 of the node (`0043b370`), makes sure it
/// has a bound (`fn_005467e0`) and, when setting, gives the bound the
/// radius 1.0 (`0063f790` on the pointer at `+0x20`).
pub fn fn_00546780(e: &mut Engine, this: Ptr, set: bool) {
    e.call(NODE_SET_FLAG_BITS, &args![this, set, 0x800u32]);
    fn_005467e0(e, this);
    if set {
        let bound = e.mem.u32(this.addr() + 0x20);
        e.call(BOUND_SET_RADIUS, &args![bound, 1.0f32]);
    }
}

// Translated from 005467c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag bit 0x2000 of the node (`0043b370`).
pub fn fn_005467c0(e: &mut Engine, this: Ptr, set: bool) {
    e.call(NODE_SET_FLAG_BITS, &args![this, set, 0x2000u32]);
}

// Translated from 005467e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives a node without a bound (the pointer at `+0x20`) one: a 0x10-byte
/// block (`00546890` is called on it and does nothing to its contents) set
/// through the node's virtual slot `0x78`, then given the three words of
/// `011f426c` and the radius 0.0 (`004a51d0`). The exception-unwinding frame
/// is not translated.
pub fn fn_005467e0(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 0x20) != 0 {
        return;
    }
    let block = e.call(ALLOCATE, &args![0x10u32]).ptr::<()>();
    let bound = if block.is_null() {
        Ptr::NULL
    } else {
        fn_00546890(e, block, 0x100, 0)
    };
    e.vcall(this.addr(), 0x78, &args![bound]);
    let bound = e.mem.u32(this.addr() + 0x20);
    e.call(
        BOUND_SET_CENTER_AND_RADIUS,
        &args![bound, ZERO_VECTOR, 0.0f32],
    );
}

// Translated from 00546890 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor-shaped function (`std::_String_alloc<..>::_Getal` by the
/// map's folded name): calls `006240d0` and returns `this`. Its two stack
/// words are not read.
pub fn fn_00546890(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) -> Ptr {
    e.call(BOUND_BASE_CONSTRUCT, &args![this]);
    this
}

// Translated from 005468b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag bit 0x200000 of the node (`0043b370`).
pub fn fn_005468b0(e: &mut Engine, this: Ptr, set: bool) {
    e.call(NODE_SET_FLAG_BITS, &args![this, set, 0x20_0000u32]);
}

// Translated from 005468d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes `value` to the word at `+0xB0` of the multibound node.
pub fn fn_005468d0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xb0, value);
}

// Translated from 005468f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `index & 0x1F` of the word at `011ca08c` is set (cdecl).
pub fn fn_005468f0(e: &mut Engine, index: u8) -> bool {
    let word: u32 = e.global(STATIC_FLAG);
    (1u32 << (index & 0x1f)) & word != 0
}

// Translated from 00546910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the smart-pointer slot at `+0x4C` (`0066b0d0`).
pub fn fn_00546910(e: &mut Engine, this: Ptr, value: Ptr) {
    slot_assign(e, this.byte_add(0x4c), value);
}

// Translated from 00546930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object in the smart-pointer slot at `+0x4C`.
pub fn fn_00546930(e: &mut Engine, this: Ptr) -> Ptr {
    slot_get(e, this.byte_add(0x4c))
}

// Translated from 00546950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x10 of the word at `+0x244` (of the game loader) is set.
pub fn fn_00546950(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x244) & 0x10 != 0
}

// Translated from 00546970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unloads the cell's runtime state: when the extra data list has its
/// type-1 data and the object at `011ca088` exists, tells that object about
/// the first (`fn_00541c80`, then `006242f0`); sets the cell state to 1,
/// releases the loaded data (`fn_00545030`), resets the type-1 extra data
/// (`0041b8d0(list, 0)`), clears the two reference counters
/// (`fn_00546a00`, `fn_00546a20`), sets the cell state to 0 and
/// `bCellDetached` to false.
pub fn fn_00546970(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let list = this.byte_add(EXTRA_DATA_OFFSET);
    let extra = e.call(EXTRA_LIST_GET_TYPE_1, &args![list]).ptr::<()>();
    let object: u32 = e.global(STATIC_OBJECT);
    if !extra.is_null() && object != 0 {
        let value = fn_00541c80(e, extra);
        e.call(STATIC_RELEASE_HELPER, &args![object, value]);
    }
    e.call(CELL_SET_STATE, &args![this, 1u32]);
    fn_00545030(e, this);
    e.call(EXTRA_LIST_RESET_TYPE_1, &args![list, 0u32]);
    fn_00546a00(e, this);
    fn_00546a20(e, this);
    e.call(CELL_SET_STATE, &args![this, 0u32]);
    e.set(this, TESObjectCELL::bCellDetached, false);
}

// Translated from 00546a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears `iQueuedRefCount`.
pub fn fn_00546a00(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.set(this, TESObjectCELL::iQueuedRefCount, 0);
}

// Translated from 00546a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears `iCriticalQueuedRefCount`.
pub fn fn_00546a20(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.set(this, TESObjectCELL::iCriticalQueuedRefCount, 0);
}

// Translated from 00546a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetOwner` (Xbox PDB): the owner of the cell's extra data
/// list; without one, the form at `+0x18` of the owner `fn_00546c20` finds
/// when that is not the form `fn_00546a90` returns.
pub fn tes_object_cell_get_owner(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
    let mut owner = e.call(EXTRA_LIST_GET_OWNER, &args![list]).u32();
    if owner == 0 {
        let candidate = fn_00546c20(e, this);
        if candidate != 0 && candidate != fn_00546a90(e) {
            owner = e.call(WORD_AT_18, &args![candidate]).u32();
        }
    }
    owner
}

// Translated from 00546a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c9520`.
pub fn fn_00546a90(e: &mut Engine) -> u32 {
    e.global(OWNER_EXCLUDED_FORM)
}

// Translated from 00546aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetOwnershipGlobal` (Xbox PDB): the global of the cell's
/// extra data list (`ExtraDataList::GetGlobal`).
pub fn tes_object_cell_get_ownership_global(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
    e.call(EXTRA_LIST_GET_GLOBAL, &args![list]).u32()
}

// Translated from 00546ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetOwnershipRank` (Xbox PDB): `ExtraDataList::GetRank`,
/// with -1 (no rank) turned into 0.
pub fn tes_object_cell_get_ownership_rank(e: &mut Engine, this: Ptr<TESObjectCELL>) -> i32 {
    let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
    let rank = e.call(EXTRA_LIST_GET_RANK, &args![list]).i32();
    if rank == -1 {
        0
    } else {
        rank
    }
}

// Translated from 00546af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetDetachTime` (Xbox PDB): the detach time of the cell's
/// extra data list (`ExtraDataList::GetDetachTime`).
pub fn tes_object_cell_get_detach_time(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
    e.call(EXTRA_LIST_GET_DETACH_TIME, &args![list]).u32()
}

// Translated from 00546b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the cell's detach time and form flags. Unless the loader reports
/// bit 2 of its `+0x244` word and `time` is non-zero and `keep` is zero,
/// stores `time` in the extra data list (`ExtraDataList` setter `00421850`).
/// A zero `time` then clears flags through the cell's virtual slot `0x4C`
/// (`0x70000000`); otherwise slot `0x48` sets `0x40000000`, and, for an
/// exterior cell, `0x20000000` when both grid coordinates are within
/// -128..127 and `0x10000000` when not.
pub fn fn_00546b10(e: &mut Engine, this: Ptr<TESObjectCELL>, time: u32, keep: bool) {
    let loader: u32 = e.global(GAME_LOADER_POINTER);
    if !e.call(LOADER_FLAG_244_2, &args![loader]).bool() || time == 0 || keep {
        let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
        e.call(EXTRA_LIST_SET_DETACH_TIME, &args![list, time]);
    }
    if time == 0 {
        e.vcall(this.addr(), 0x4c, &args![0x7000_0000u32]);
        return;
    }
    e.vcall(this.addr(), 0x48, &args![0x4000_0000u32]);
    if !is_interior(e, this.cast()) {
        let x = tes_object_cell_get_data_x(e, this);
        let y = tes_object_cell_get_data_y(e, this);
        let bit = if (-0x80..=0x7f).contains(&x) && (-0x80..=0x7f).contains(&y) {
            0x2000_0000u32
        } else {
            0x1000_0000
        };
        e.vcall(this.addr(), 0x48, &args![bit]);
    }
}

// Translated from 00546bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the cell's owner (`00419700` on its extra data list) and the form
/// flag 8 through virtual slot `0x48`.
pub fn fn_00546bf0(e: &mut Engine, this: Ptr<TESObjectCELL>, owner: u32) {
    let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
    e.call(EXTRA_LIST_SET_OWNER, &args![list, owner]);
    e.vcall(this.addr(), 0x48, &args![8u32]);
}

// Translated from 00546c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The owner candidate of the cell: the extra data of type 0x74, or, when
/// absent, what `00458400` gives for the cell's world space (if it has one).
pub fn fn_00546c20(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
    let mut result = e.call(EXTRA_LIST_GET_TYPE_74, &args![list]).u32();
    if result == 0 && !world_space(e, this.cast()).is_null() {
        let world = world_space(e, this.cast());
        result = e.call(WORLD_SPACE_OWNER_FORM, &args![world]).u32();
    }
    result
}

// Translated from 00546c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00546b10` with the value `00867e30` computes from the object at
/// `011de7b8`, and `keep` false.
pub fn fn_00546c70(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let time = e
        .call(DETACH_TIME_SOURCE, &args![DETACH_TIME_SOURCE_OBJECT])
        .u32();
    fn_00546b10(e, this, time, false);
}

// Translated from 00546ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor may use the cell's things freely: false without an
/// owner or when the actor's virtual slot `0x218` says false; an owner that
/// is a character base (cast target `01183a1c`) must be the actor's form
/// (`007af430`); an owner that is a faction (`01184704`) needs the actor's
/// faction rank (`TESActorBaseData::GetFactionRank` on the actor's base data
/// at `+0x30`, with the flag "actor is the singleton at `011dea3c`") to be at
/// least the cell's ownership rank.
pub fn fn_00546ca0(e: &mut Engine, this: Ptr<TESObjectCELL>, actor: Ptr) -> bool {
    let owner = Ptr::new(tes_object_cell_get_owner(e, this));
    if owner.is_null() {
        return false;
    }
    if !e.vcall(actor.addr(), 0x218, &[]).bool() {
        return false;
    }
    let character = dynamic_cast(e, owner, RTTI_OWNER_CHARACTER);
    if !character.is_null() {
        let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![actor]).u32();
        return character.addr() == form;
    }
    let faction = dynamic_cast(e, owner, RTTI_OWNER_FACTION);
    if faction.is_null() {
        return false;
    }
    let rank = tes_object_cell_get_ownership_rank(e, this);
    let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![actor]).ptr::<()>();
    let base = cast_from(e, form, RTTI_OWNER_SOURCE_FORM, RTTI_OWNER_CHARACTER);
    let singleton: u32 = e.global(ACTOR_SINGLETON_POINTER);
    let actor_rank = e
        .call(
            ACTOR_BASE_GET_FACTION_RANK,
            &args![base.byte_add(0x30), faction, actor.addr() == singleton],
        )
        .i32();
    actor_rank >= rank
}

// Translated from 00546da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The opposite check, for taking things: false when the actor's virtual
/// slot `0x304` (called without arguments) is true, without an owner, for a
/// cell `fn_005443e0` reports, with an ownership global, or when the actor's
/// slot `0x218` is false. An owner that is a character base must differ from
/// the actor's form; a faction owner needs the actor's faction rank to be
/// below the cell's ownership rank.
pub fn fn_00546da0(e: &mut Engine, this: Ptr<TESObjectCELL>, actor: Ptr) -> bool {
    if e.vcall(actor.addr(), 0x304, &[]).bool() {
        return false;
    }
    let owner = Ptr::new(tes_object_cell_get_owner(e, this));
    if owner.is_null() || fn_005443e0(e, this) {
        return false;
    }
    if tes_object_cell_get_ownership_global(e, this) != 0
        || !e.vcall(actor.addr(), 0x218, &[]).bool()
    {
        return false;
    }
    let character = dynamic_cast(e, owner, RTTI_OWNER_CHARACTER);
    if !character.is_null() {
        let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![actor]).u32();
        return character.addr() != form;
    }
    let faction = dynamic_cast(e, owner, RTTI_OWNER_FACTION);
    if faction.is_null() {
        return false;
    }
    let rank = tes_object_cell_get_ownership_rank(e, this);
    let form = e.call(SAVE_FORM_BUFFER_GET_FORM, &args![actor]).ptr::<()>();
    let base = cast_from(e, form, RTTI_OWNER_SOURCE_FORM, RTTI_OWNER_CHARACTER);
    let singleton: u32 = e.global(ACTOR_SINGLETON_POINTER);
    let actor_rank = e
        .call(
            ACTOR_BASE_GET_FACTION_RANK,
            &args![base.byte_add(0x30), faction, actor.addr() == singleton],
        )
        .i32();
    actor_rank < rank
}

// Translated from 00546ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the form owns the cell: false without an owner; an owner that is
/// a character base must be `form`; a faction owner needs `form` (cast from
/// `011846e8` to the character base) to have a faction rank (flag false) at
/// least the cell's ownership rank, and false when the cast fails.
pub fn fn_00546ee0(e: &mut Engine, this: Ptr<TESObjectCELL>, form: Ptr) -> bool {
    let owner = Ptr::new(tes_object_cell_get_owner(e, this));
    if owner.is_null() {
        return false;
    }
    let character = dynamic_cast(e, owner, RTTI_OWNER_CHARACTER);
    if !character.is_null() {
        return character.addr() == form.addr();
    }
    let faction = dynamic_cast(e, owner, RTTI_OWNER_FACTION);
    if faction.is_null() {
        return false;
    }
    let rank = tes_object_cell_get_ownership_rank(e, this);
    let base = cast_from(e, form, RTTI_OWNER_SOURCE_ARGUMENT, RTTI_OWNER_CHARACTER);
    if base.is_null() {
        return false;
    }
    let form_rank = e
        .call(
            ACTOR_BASE_GET_FACTION_RANK,
            &args![base.byte_add(0x30), faction, false],
        )
        .i32();
    form_rank >= rank
}

/// `__RTDynamicCast` of `object` from the type descriptor `source` to
/// `target`.
pub(crate) fn cast_from(e: &mut Engine, object: Ptr, source: u32, target: u32) -> Ptr {
    e.call(RT_DYNAMIC_CAST, &args![object, 0u32, source, target, 0u32])
        .ptr()
}

// Translated from 00546fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetLand` (Xbox PDB): null for an interior cell or one
/// with the persistent flag; the cell's `pCellLand`, built on first use (a
/// new 0x2C-byte `TESObjectLAND` given the cell with `SetCell`, inside an
/// allocation scope). The exception-unwinding frame is not translated.
pub fn tes_object_cell_get_land(e: &mut Engine, this: Ptr<TESObjectCELL>) -> Ptr {
    if is_interior(e, this.cast()) || persistent_flag(e, this.cast()) {
        return Ptr::NULL;
    }
    if e.get(this, TESObjectCELL::pCellLand).is_null() {
        // 00547014: the scope guard is the 4-byte local at EBP-0x10.
        e.with_stack(4, |e, scope| {
            e.call(
                SCOPE_ENTER,
                &args![scope, 0x1bu32, 1u32, SOURCE_FILE, 0xfb6u32],
            );
            let block = e.call(ALLOCATE, &args![0x2cu32]).ptr::<()>();
            let land = if block.is_null() {
                Ptr::NULL
            } else {
                e.call(LAND_CONSTRUCT, &args![block]).ptr::<()>()
            };
            e.set(this, TESObjectCELL::pCellLand, land);
            let land = e.get(this, TESObjectCELL::pCellLand);
            e.call(LAND_SET_CELL, &args![land, this]);
            e.call(SCOPE_LEAVE, &args![scope]);
        });
    }
    e.get(this, TESObjectCELL::pCellLand)
}

// Translated from 005470a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::SetLand` (Xbox PDB): for an exterior cell whose land
/// differs, destroys the old land (virtual slot `0x10` with 1, the scalar
/// deleting destructor) and stores the new one.
pub fn tes_object_cell_set_land(e: &mut Engine, this: Ptr<TESObjectCELL>, land: Ptr) {
    if is_interior(e, this.cast()) {
        return;
    }
    let old = e.get(this, TESObjectCELL::pCellLand);
    if old.addr() == land.addr() {
        return;
    }
    if !old.is_null() {
        e.vcall(old.addr(), 0x10, &args![1u32]);
    }
    e.set(this, TESObjectCELL::pCellLand, land);
}

// Translated from 00547110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetRegionList` (Xbox PDB): null for an interior cell;
/// the extra data list's region list, which `create` makes (a new 0x10-byte
/// `TESRegionList`, stored with `SetRegionList`) when it is absent. The
/// exception-unwinding frame is not translated.
pub fn tes_object_cell_get_region_list(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    create: bool,
) -> Ptr {
    if is_interior(e, this.cast()) {
        return Ptr::NULL;
    }
    let list = this.byte_add(EXTRA_DATA_OFFSET);
    let mut regions = e.call(EXTRA_LIST_GET_REGION_LIST, &args![list]).ptr::<()>();
    if regions.is_null() && create {
        let block = e.call(ALLOCATE, &args![0x10u32]).ptr::<()>();
        regions = if block.is_null() {
            Ptr::NULL
        } else {
            e.call(REGION_LIST_CONSTRUCT, &args![block, 0u32]).ptr()
        };
        e.call(EXTRA_LIST_SET_REGION_LIST, &args![list, regions]);
    }
    regions
}

// Translated from 005471e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetWaterHeight` (Xbox PDB): `-FLT_MAX` for a cell
/// without bit 1 of `cCellFlags` or an interior cell; `fWaterHeight` unless
/// it is `FLT_MAX` (unset); then the default water height of the cell's world
/// space (`004536e0`), 0.0 without a world space.
pub fn tes_object_cell_get_water_height(e: &mut Engine, this: Ptr<TESObjectCELL>) -> f32 {
    if !e.call(CELL_FLAG_2, &args![this]).bool() || is_interior(e, this.cast()) {
        return e.global(LOWEST_FLOAT);
    }
    let height = e.get(this, TESObjectCELL::fWaterHeight);
    let unset: f64 = e.global(UNSET_WATER_HEIGHT_DOUBLE);
    if f64::from(height) != unset {
        return height;
    }
    let world = world_space(e, this.cast());
    if world.is_null() {
        0.0
    } else {
        e.call(WORLD_SPACE_WATER_HEIGHT, &args![world]).f32()
    }
}

// Translated from 00547250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The water height at a position: stores the cell's water height in
/// `*height` (`-FLT_MAX` and false for an interior cell without bit 1 of
/// `cCellFlags`), then walks the loaded data's `WaterRefs` list; for each
/// reference it takes the first child of its 3D's child, whose shape's
/// bound primitive (type 1 or 2 replaces the position's third float with the
/// float at `+8` of the primitive's sub-object at `+0xC`) is asked if it contains
/// the position (virtual slot `0x108`); a reference that does raises `*height`
/// to the float at `+8` of its virtual slot `0x1F4` result and the result
/// becomes true. The reference's virtual slots `0x1D8` and `0x1DC` are
/// called with two 12-byte buffers whose contents are never read.
pub fn fn_00547250(e: &mut Engine, this: Ptr<TESObjectCELL>, position: Ptr, height: Ptr) -> bool {
    if is_interior(e, this.cast()) && !e.call(CELL_FLAG_2, &args![this]).bool() {
        let lowest: f32 = e.global(LOWEST_FLOAT);
        e.mem.set_f32(height.addr(), lowest);
        return false;
    }
    let water = tes_object_cell_get_water_height(e, this);
    e.mem.set_f32(height.addr(), water);
    // Locals: the position (3 floats), two 12-byte buffers, and the copy of
    // the reference's rotation the function never reads.
    e.with_stack(0x30, |e, frame| {
        let local_position = frame;
        let first_buffer = frame.byte_add(0xc);
        let second_buffer = frame.byte_add(0x18);
        let rotation_copy = frame.byte_add(0x24);
        for word in 0..3 {
            let value = e.mem.u32(position.addr() + 4 * word);
            e.mem.set_u32(local_position.addr() + 4 * word, value);
        }
        let mut found = false;
        let loaded = e.call(CELL_LOADED_DATA, &args![this]).ptr::<()>();
        if loaded.is_null() {
            return found;
        }
        let mut node = loaded.byte_add(LoadedCellData::WaterRefs.off);
        while !node.is_null() {
            if list_item(e, node).is_null() {
                break;
            }
            let reference = list_item(e, node);
            e.vcall(reference.addr(), 0x1d8, &args![first_buffer]);
            e.vcall(reference.addr(), 0x1dc, &args![second_buffer]);
            let rotation = e.call(REFERENCE_ROTATION, &args![reference]).u32();
            for word in 0..3 {
                let value = e.mem.u32(rotation + 4 * word);
                e.mem.set_u32(rotation_copy.addr() + 4 * word, value);
            }
            let node_3d = e.vcall(reference.addr(), 0x1d0, &[]).ptr::<()>();
            let holder = e.vcall(node_3d.addr(), 0xc, &[]).ptr::<()>();
            let child = e.call(NODE_CHILD_AT, &args![holder, 0u32]).ptr::<()>();
            let shape = e.vcall(child.addr(), 0x14, &[]).ptr::<()>();
            if !shape.is_null() {
                let data = e.call(MULTI_BOUND_DATA_OF_ROOM, &args![shape]).ptr::<()>();
                let primitive = e.call(MULTI_BOUND_PRIMITIVE_SLOT, &args![data]).ptr::<()>();
                for kind in [1u32, 2] {
                    if e.vcall(primitive.addr(), 0x8c, &[]).u32() == kind {
                        let part = e.call(PRIMITIVE_PART, &args![primitive]).u32();
                        let value = e.mem.u32(part + 8);
                        e.mem.set_u32(local_position.addr() + 8, value);
                    }
                }
                if e.vcall(shape.addr(), 0x108, &args![local_position]).bool() {
                    let result = e.vcall(reference.addr(), 0x1f4, &[]).u32();
                    let top = e.mem.f32(result + 8);
                    if e.mem.f32(height.addr()) < top {
                        e.mem.set_f32(height.addr(), top);
                    }
                    found = true;
                }
            }
            node = list_next(e, node);
        }
        found
    })
}

// Translated from 00547440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fWaterHeight`: `FLT_MAX` (unset) when the cell has a world space
/// whose default water height (`004536e0`) is within 0.001 of `height`
/// (`0040eb10`), `height` otherwise.
pub fn fn_00547440(e: &mut Engine, this: Ptr<TESObjectCELL>, height: f32) {
    let world = world_space(e, this.cast());
    if !world.is_null() {
        let default_height = e.call(WORLD_SPACE_WATER_HEIGHT, &args![world]).f32();
        let tolerance: f32 = e.global(WATER_HEIGHT_TOLERANCE);
        if e.call(FLOAT_NEAR, &args![default_height, height, tolerance])
            .bool()
        {
            let unset: f32 = e.global(DEFAULT_WATER_HEIGHT);
            e.set(this, TESObjectCELL::fWaterHeight, unset);
            return;
        }
    }
    e.set(this, TESObjectCELL::fWaterHeight, height);
}

// Translated from 005474b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the cell's lock (`fn_00541ac0` / `fn_00541ae0`): the extra data of
/// type 7 of the cell; without it, for an exterior cell with a world space,
/// what `00586150` gives for the world space (also stored in the static at
/// `011ca1c0`); 0 otherwise. The first call fills the static record at
/// `011ca1c4` with the three words of `011f426c`. Returns the value. The
/// stack word the callers push is never read (`RET 0x4` at `0054757f`).
pub fn fn_005474b0(e: &mut Engine, this: Ptr<TESObjectCELL>, _unused_argument: u32) -> u32 {
    fn_00541ac0(e, this);
    let guard: u32 = e.global(STATIC_RECORD_GUARD);
    if guard & 1 == 0 {
        e.set_global(STATIC_RECORD_GUARD, guard | 1);
        for word in 0..3 {
            let value = e.mem.u32(ZERO_VECTOR + 4 * word);
            e.mem.set_u32(STATIC_RECORD_COPY + 4 * word, value);
        }
    }
    let mut result = 0;
    let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
    let value = e.call(EXTRA_LIST_GET_TYPE_7, &args![list]).u32();
    if value != 0 || is_interior(e, this.cast()) {
        fn_00541ae0(e, this);
        return value;
    }
    // The code also tests its result local, which is still 0 here.
    if !is_interior(e, this.cast()) {
        let world = world_space(e, this.cast());
        if !world.is_null() {
            result = e.call(WORLD_SPACE_TYPE_7_VALUE, &args![world]).u32();
            e.set_global(STATIC_RECORD_LAST, result);
        }
    }
    fn_00541ae0(e, this);
    result
}

// Translated from 00547590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetAcousticSpace` (Xbox PDB): the acoustic space of the
/// cell's extra data list (type 0x81).
pub fn tes_object_cell_get_acoustic_space(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
    e.call(EXTRA_LIST_GET_ACOUSTIC_SPACE, &args![list]).u32()
}

// Translated from 005475b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An interior cell: 0 unless bit 7 of `cCellFlags` is set (`00454b10`),
/// else the extra data of type 8 of the cell; an exterior cell: what
/// `00585fe0` gives for its world space.
pub fn fn_005475b0(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    if is_interior(e, this.cast()) {
        if !e.call(CELL_FLAG_80, &args![this]).bool() {
            return 0;
        }
        let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
        e.call(EXTRA_LIST_GET_TYPE_8, &args![list]).u32()
    } else {
        let world = world_space(e, this.cast());
        e.call(WORLD_SPACE_ACOUSTIC_VALUE, &args![world]).u32()
    }
}

// Translated from 00547610 (decompiled, FalloutNV.exe 1.4.0.525)
/// For an interior cell, the extra data accessor `0041c460` on the cell's
/// list (the one `EXTRA_LIST_GET_IMPACT_SWAP` names); 0 for an exterior cell.
pub fn fn_00547610(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    if is_interior(e, this.cast()) {
        let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).ptr::<()>();
        e.call(EXTRA_LIST_GET_IMPACT_SWAP, &args![list]).u32()
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x005415b0,
            tes_object_cell_tes_object_cell(Ptr<TESObjectCELL>) -> Ptr<TESObjectCELL>
        ),
        entry!(
            0x005417e0,
            fn_005417e0(Ptr<TESObjectCELL>, u32) -> Ptr<TESObjectCELL>
        ),
        entry!(0x00541810, fn_00541810(Ptr<TESObjectCELL>)),
        entry!(0x00541a30, fn_00541a30(Ptr, u32) -> Ptr),
        entry!(0x00541a60, fn_00541a60(Ptr)),
        entry!(0x00541ac0, fn_00541ac0(Ptr<TESObjectCELL>)),
        entry!(0x00541ae0, fn_00541ae0(Ptr<TESObjectCELL>)),
        entry!(0x00541b00, fn_00541b00(Ptr<TESObjectCELL>)),
        entry!(0x00541b80, tes_object_cell_init_statics()),
        entry!(0x00541c00, fn_00541c00()),
        entry!(0x00541c80, fn_00541c80(Ptr) -> u32),
        entry!(0x00541cb0, fn_00541cb0(Ptr) -> u32),
        entry!(0x00541cd0, fn_00541cd0()),
        entry!(0x00541ce0, fn_00541ce0(Ptr<TESObjectCELL>)),
        entry!(
            0x00541e80,
            tes_object_cell_save(Ptr<TESObjectCELL>, Ptr) -> bool
        ),
        entry!(
            0x00541fd0,
            tes_object_cell_save_edit(Ptr<TESObjectCELL>, Ptr) -> bool
        ),
        entry!(
            0x00542040,
            tes_object_cell_save_references(Ptr<TESObjectCELL>, Ptr)
        ),
        entry!(0x005422b0, fn_005422b0(Ptr)),
        entry!(0x005422d0, fn_005422d0(Ptr<TESObjectCELL>, Ptr) -> bool),
        entry!(0x00542d20, fn_00542d20(Ptr) -> bool),
        entry!(0x00542d40, tes_object_cell_init_item(Ptr<TESObjectCELL>)),
        entry!(0x00543230, fn_00543230(Ptr<TESObjectCELL>, Ptr) -> bool),
        entry!(0x005434d0, fn_005434d0(Ptr<TESObjectCELL>, Ptr) -> bool),
        entry!(
            0x005436e0,
            tes_object_cell_create_duplicate_form(
                Ptr<TESObjectCELL>,
                u32,
                u32,
            ) -> Ptr<TESObjectCELL>
        ),
        entry!(0x00543ab0, tes_object_cell_copy(Ptr<TESObjectCELL>, Ptr)),
        entry!(0x00543c30, fn_00543c30(Ptr<TESObjectCELL>) -> u8),
        entry!(0x00543c50, fn_00543c50(Ptr<TESObjectCELL>, Ptr) -> bool),
        entry!(
            0x00543df0,
            fn_00543df0(Ptr<TESObjectCELL>, Ptr, u8, u8) -> bool
        ),
        entry!(0x00543f50, fn_00543f50(Ptr<TESObjectCELL>, Ptr, Ptr)),
        entry!(0x005441b0, fn_005441b0(Ptr<TESObjectCELL>) -> u32),
        entry!(0x00544210, fn_00544210(Ptr<TESObjectCELL>) -> u32),
        entry!(
            0x00544280,
            tes_object_cell_calc_ext_group_block_key(i32, i32) -> u32
        ),
        entry!(
            0x005442c0,
            tes_object_cell_calc_ext_group_sub_block_key(i32, i32) -> u32
        ),
        entry!(
            0x00544300,
            tes_object_cell_set_interior(Ptr<TESObjectCELL>, bool)
        ),
        entry!(0x00544340, fn_00544340(Ptr<TESObjectCELL>, bool)),
        entry!(
            0x00544390,
            tes_object_cell_set_temp_public(Ptr<TESObjectCELL>, bool)
        ),
        entry!(0x005443e0, fn_005443e0(Ptr<TESObjectCELL>) -> bool),
        entry!(0x00544420, fn_00544420(Ptr<TESObjectCELL>) -> bool),
        entry!(0x00544440, fn_00544440(Ptr<TESObjectCELL>, bool)),
        entry!(0x00544490, fn_00544490(Ptr) -> bool),
        entry!(0x005444c0, fn_005444c0(Ptr<TESObjectCELL>) -> bool),
        entry!(0x00544520, fn_00544520(Ptr<TESObjectCELL>) -> bool),
        entry!(0x00544590, fn_00544590(Ptr<TESObjectCELL>, u32) -> bool),
        entry!(0x005445d0, fn_005445d0(Ptr<TESObjectCELL>) -> Ptr),
        entry!(0x00544600, fn_00544600(Ptr<TESObjectCELL>) -> Ptr),
        entry!(
            0x00544630,
            tes_object_cell_create_cell_data(Ptr<TESObjectCELL>)
        ),
        entry!(0x00544750, fn_00544750(Ptr<TESObjectCELL>) -> u32),
        entry!(0x005447b0, fn_005447b0(Ptr<TESObjectCELL>, Ptr)),
        entry!(0x00544830, fn_00544830(Ptr<TESObjectCELL>) -> u32),
        entry!(0x00544890, fn_00544890(Ptr<TESObjectCELL>, Ptr)),
        entry!(0x00544910, fn_00544910(Ptr<TESObjectCELL>) -> u32),
        entry!(0x00544970, fn_00544970(Ptr<TESObjectCELL>) -> u32),
        entry!(0x005449d0, fn_005449d0(Ptr<TESObjectCELL>) -> u32),
        entry!(0x00544a30, fn_00544a30(Ptr<TESObjectCELL>, Ptr)),
        entry!(0x00544ab0, fn_00544ab0(Ptr<TESObjectCELL>) -> f32),
        entry!(0x00544b10, fn_00544b10(Ptr<TESObjectCELL>) -> f32),
        entry!(0x00544b70, fn_00544b70(Ptr<TESObjectCELL>) -> f32),
        entry!(0x00544bd0, fn_00544bd0(Ptr<TESObjectCELL>) -> f32),
        entry!(
            0x00544c30,
            tes_object_cell_get_data_x(Ptr<TESObjectCELL>) -> i32
        ),
        entry!(
            0x00544c60,
            tes_object_cell_get_data_y(Ptr<TESObjectCELL>) -> i32
        ),
        entry!(
            0x00544c90,
            tes_object_cell_set_data_coord(Ptr<TESObjectCELL>, i32, i32)
        ),
        entry!(0x00544ce0, fn_00544ce0(Ptr<TESObjectCELL>)),
        entry!(
            0x00544f60,
            fn_00544f60(Ptr<LoadedCellData>) -> Ptr<LoadedCellData>
        ),
        entry!(0x00545030, fn_00545030(Ptr<TESObjectCELL>)),
        entry!(
            0x005451d0,
            fn_005451d0(Ptr<LoadedCellData>, u32) -> Ptr<LoadedCellData>
        ),
        entry!(0x00545200, fn_00545200(Ptr<LoadedCellData>)),
        entry!(
            0x005452c0,
            fn_005452c0(Ptr<TESObjectCELL>, Ptr<TESObjectREFR>)
        ),
        entry!(
            0x00545360,
            tes_object_cell_remove_emittance_ref(Ptr<TESObjectCELL>, Ptr<TESObjectREFR>)
        ),
        entry!(
            0x005453b0,
            fn_005453b0(Ptr<TESObjectCELL>, Ptr<TESObjectREFR>)
        ),
        entry!(0x005454d0, fn_005454d0(Ptr) -> Ptr),
        entry!(
            0x005454f0,
            fn_005454f0(Ptr<TESObjectCELL>, Ptr<TESObjectREFR>)
        ),
        entry!(
            0x00545560,
            fn_00545560(Ptr<TESObjectCELL>, Ptr<TESObjectREFR>)
        ),
        entry!(
            0x00545590,
            fn_00545590(Ptr<TESObjectCELL>, Ptr<TESObjectREFR>)
        ),
        entry!(
            0x005455c0,
            tes_object_cell_add_activating_ref(Ptr<TESObjectCELL>, Ptr<TESObjectREFR>)
        ),
        entry!(
            0x00545670,
            fn_00545670(Ptr<TESObjectCELL>, Ptr<TESObjectREFR>)
        ),
        entry!(
            0x005456b0,
            fn_005456b0(Ptr<TESObjectCELL>, Ptr<TESObjectREFR>)
        ),
        entry!(
            0x005456e0,
            fn_005456e0(Ptr<TESObjectCELL>, Ptr<TESObjectREFR>)
        ),
        entry!(0x00545710, fn_00545710(Ptr<TESObjectCELL>) -> Ptr),
        entry!(
            0x00545740,
            fn_00545740(Ptr<TESObjectCELL>, Ptr<TESObjectREFR>) -> Ptr
        ),
        entry!(
            0x00545960,
            fn_00545960(Ptr<TESObjectCELL>, Ptr<TESObjectREFR>) -> Ptr
        ),
        entry!(
            0x00545a30,
            tes_object_cell_attach_multi_bound_nodes(Ptr<TESObjectCELL>)
        ),
        entry!(0x00545b30, fn_00545b30(Ptr<TESObjectCELL>) -> Ptr),
        entry!(0x00545c10, fn_00545c10(Ptr<TESObjectCELL>)),
        entry!(
            0x00545cb0,
            tes_object_cell_get_3d(Ptr<TESObjectCELL>) -> Ptr
        ),
        entry!(
            0x00545cf0,
            tes_object_cell_load_3d(Ptr<TESObjectCELL>) -> Ptr
        ),
        entry!(0x00546780, fn_00546780(Ptr, bool)),
        entry!(0x005467c0, fn_005467c0(Ptr, bool)),
        entry!(0x005467e0, fn_005467e0(Ptr)),
        entry!(0x00546890, fn_00546890(Ptr, u32, u32) -> Ptr),
        entry!(0x005468b0, fn_005468b0(Ptr, bool)),
        entry!(0x005468d0, fn_005468d0(Ptr, u32)),
        entry!(0x005468f0, fn_005468f0(u8) -> bool),
        entry!(0x00546910, fn_00546910(Ptr, Ptr)),
        entry!(0x00546930, fn_00546930(Ptr) -> Ptr),
        entry!(0x00546950, fn_00546950(Ptr) -> bool),
        entry!(0x00546970, fn_00546970(Ptr<TESObjectCELL>)),
        entry!(0x00546a00, fn_00546a00(Ptr<TESObjectCELL>)),
        entry!(0x00546a20, fn_00546a20(Ptr<TESObjectCELL>)),
        entry!(
            0x00546a40,
            tes_object_cell_get_owner(Ptr<TESObjectCELL>) -> u32
        ),
        entry!(0x00546a90, fn_00546a90() -> u32),
        entry!(
            0x00546aa0,
            tes_object_cell_get_ownership_global(Ptr<TESObjectCELL>) -> u32
        ),
        entry!(
            0x00546ac0,
            tes_object_cell_get_ownership_rank(Ptr<TESObjectCELL>) -> i32
        ),
        entry!(
            0x00546af0,
            tes_object_cell_get_detach_time(Ptr<TESObjectCELL>) -> u32
        ),
        entry!(0x00546b10, fn_00546b10(Ptr<TESObjectCELL>, u32, bool)),
        entry!(0x00546bf0, fn_00546bf0(Ptr<TESObjectCELL>, u32)),
        entry!(0x00546c20, fn_00546c20(Ptr<TESObjectCELL>) -> u32),
        entry!(0x00546c70, fn_00546c70(Ptr<TESObjectCELL>)),
        entry!(0x00546ca0, fn_00546ca0(Ptr<TESObjectCELL>, Ptr) -> bool),
        entry!(0x00546da0, fn_00546da0(Ptr<TESObjectCELL>, Ptr) -> bool),
        entry!(0x00546ee0, fn_00546ee0(Ptr<TESObjectCELL>, Ptr) -> bool),
        entry!(
            0x00546fb0,
            tes_object_cell_get_land(Ptr<TESObjectCELL>) -> Ptr
        ),
        entry!(
            0x005470a0,
            tes_object_cell_set_land(Ptr<TESObjectCELL>, Ptr)
        ),
        entry!(
            0x00547110,
            tes_object_cell_get_region_list(Ptr<TESObjectCELL>, bool) -> Ptr
        ),
        entry!(
            0x005471e0,
            tes_object_cell_get_water_height(Ptr<TESObjectCELL>) -> f32
        ),
        entry!(
            0x00547250,
            fn_00547250(Ptr<TESObjectCELL>, Ptr, Ptr) -> bool
        ),
        entry!(0x00547440, fn_00547440(Ptr<TESObjectCELL>, f32)),
        entry!(0x005474b0, fn_005474b0(Ptr<TESObjectCELL>, u32) -> u32),
        entry!(
            0x00547590,
            tes_object_cell_get_acoustic_space(Ptr<TESObjectCELL>) -> u32
        ),
        entry!(0x005475b0, fn_005475b0(Ptr<TESObjectCELL>) -> u32),
        entry!(0x00547610, fn_00547610(Ptr<TESObjectCELL>) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// The vtable of every test object. Slot `n` leads to the double at
    /// `fake(n)`; the doubles read their answers from words at the end of
    /// the object (`KEY` and friends), so one object type serves as a cell,
    /// a reference, a world space, a land or a nav mesh.
    const TEST_VTABLE: u32 = 0x0130_0000;
    const KEY: u32 = 0xe8;
    const EDITOR_ID_WORD: u32 = 0xec;
    const DUPLICATE: u32 = 0xf0;
    const SLOT_ZERO_RESULT: u32 = 0xf4;
    const BELONGS: u32 = 0xf8;
    const SLOT_94_RESULT: u32 = 0xfc;

    fn fake(slot: u32) -> u32 {
        0x7000_0000 + slot
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The addresses called, in order, without the call under test.
    fn sequence(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter().skip(1).map(|(a, _)| *a).collect()
    }

    /// Registers do-nothing doubles returning 0.
    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// A double that returns `value`.
    fn returns(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    /// An engine with the pages the cell code reads globals from, the test
    /// vtable and doubles for the accessors that only read a field, behaving
    /// as the game's do.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_6000,
            0x0101_8000,
            0x0101_a000,
            0x0118_7000,
            0x011c_3000,
            0x011c_a000,
            0x011c_c000,
            0x011d_d000,
            0x011d_e000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        let slots: Vec<u32> = (0..=0x134 / 4).map(|i| fake(4 * i)).collect();
        e.put_vtable(TEST_VTABLE, &slots);
        // Virtual methods: those that answer read the object's own words.
        quiet(
            &mut e,
            &[
                fake(0x10),
                fake(0x20),
                fake(0x2c),
                fake(0x48),
                fake(0x88),
                fake(0xc8),
                fake(0xe0),
                fake(0x134),
            ],
        );
        e.register(fake(0x3c), |e, a| {
            u32::from(e.mem.u32(a[0] + KEY) < e.mem.u32(a[1] + KEY)).into_ret()
        });
        e.register(fake(0x40), |e, a| e.mem.u32(a[0] + DUPLICATE).into_ret());
        e.register(fake(0), |e, a| {
            e.mem.u32(a[0] + SLOT_ZERO_RESULT).into_ret()
        });
        e.register(fake(0x94), |e, a| {
            e.mem.u32(a[0] + SLOT_94_RESULT).into_ret()
        });
        e.register(fake(0x110), |e, a| e.mem.u32(a[0] + BELONGS).into_ret());
        e.register(fake(0x130), |e, a| {
            e.mem.u32(a[0] + EDITOR_ID_WORD).into_ret()
        });
        // Field readers.
        e.register(FORM_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(FORM_FLAGS, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(FORM_FLAG_4000, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & 0x4000 != 0).into_ret()
        });
        e.register(FORM_FLAG_20, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & 0x20 != 0).into_ret()
        });
        e.register(FORM_FLAG_2, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & 0x2 != 0).into_ret()
        });
        e.register(FORM_FLAG_8, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & 0x8 != 0).into_ret()
        });
        e.register(CELL_IS_INTERIOR, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x24) & 1 != 0).into_ret()
        });
        e.register(CELL_PERSISTENT_FLAG, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & 0x400 != 0).into_ret()
        });
        e.register(CELL_GET_WORLD_SPACE, |e, a| {
            if e.mem.u8(a[0] + 0x24) & 1 != 0 {
                0u32.into_ret()
            } else {
                e.mem.u32(a[0] + 0xc0).into_ret()
            }
        });
        e.register(CELL_EXTERIOR_DATA, |e, a| {
            if e.mem.u8(a[0] + 0x24) & 1 != 0 {
                0u32.into_ret()
            } else {
                e.mem.u32(a[0] + 0x48).into_ret()
            }
        });
        e.register(CELL_INTERIOR_DATA, |e, a| {
            if e.mem.u8(a[0] + 0x24) & 1 != 0 {
                e.mem.u32(a[0] + 0x48).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(CELL_GET_DATA_X, |e, a| {
            let data = e.mem.u32(a[0] + 0x48);
            if data == 0 {
                0u32.into_ret()
            } else {
                e.mem.u32(data).into_ret()
            }
        });
        e.register(CELL_GET_DATA_Y, |e, a| {
            let data = e.mem.u32(a[0] + 0x48);
            if data == 0 {
                0u32.into_ret()
            } else {
                e.mem.u32(data + 4).into_ret()
            }
        });
        e.register(CELL_GET_LIGHTING_TEMPLATE, |e, a| {
            e.mem.u32(a[0] + 0xd8).into_ret()
        });
        e.register(CELL_SET_LIGHTING_TEMPLATE, |e, a| {
            e.mem.set_u32(a[0] + 0xd8, a[1]);
            Ret::default()
        });
        e.register(CELL_GET_INHERITANCE_FLAGS, |e, a| {
            e.mem.u32(a[0] + 0xdc).into_ret()
        });
        e.register(CELL_EXTRA_DATA_LIST, |_, a| (a[0] + 0x28).into_ret());
        e.register(CELL_SET_CELL_FLAGS, |e, a| {
            e.mem.set_u8(a[0] + 0x24, a[1] as u8);
            Ret::default()
        });
        e.register(SLOT_GET, |e, a| e.mem.u32(a[0]).into_ret());
        for store in [SLOT_ASSIGN, SLOT_CONSTRUCT] {
            e.register(store, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                a[0].into_ret()
            });
        }
        e.register(SLOT_RELEASE, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(DEALLOCATE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(MEMORY_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(CRT_MEMCMP, |e, a| {
            let (x, y) = (e.mem.bytes(a[0], a[2]), e.mem.bytes(a[1], a[2]));
            (x.cmp(&y) as i32).into_ret()
        });
        e.register(CRT_STRLEN, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        // BSSimpleList: a node is { item, next }; the list head is the first
        // node; a list is at its end when node.item and node.next are both 0.
        e.register(CELL_REFERENCE_LIST, |_, a| (a[0] + 0xac).into_ret());
        e.register(LIST_IS_END, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(CELL_PERSISTENT_FLAG, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & 0x400 != 0).into_ret()
        });
        e.register(REFERENCE_GET_PERSISTS, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & 0x800 != 0).into_ret()
        });
        e.register(FORM_GET_FILE, |e, a| {
            // File of the form: +0x80 for the last file (-1), +0x84 for the
            // first (0).
            if a[1] == 0xffff_ffff {
                e.mem.u32(a[0] + 0x80).into_ret()
            } else {
                e.mem.u32(a[0] + 0x84).into_ret()
            }
        });
        e.register(FILE_IS_MASTER, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x10) != 0).into_ret()
        });
        e.register(PACK_BLOCK_COORDINATES, |_, a| {
            ((a[0] as i16 as i32 as u32) << 16 | (a[1] & 0xffff)).into_ret()
        });
        e.set_global(GROUP_TAG, 0x5055_5247u32);
        e
    }

    /// A zeroed 0x100-byte object with the test vtable.
    fn object(e: &mut Engine) -> Ptr {
        let block = e.mem.alloc(0x100);
        e.mem.set_u32(block, TEST_VTABLE);
        Ptr::new(block)
    }

    /// A cell (an object that is also laid out as a cell).
    fn cell(e: &mut Engine, interior: bool) -> Ptr<TESObjectCELL> {
        let cell = object(e);
        e.mem.set_u8(cell.addr() + 0x24, interior as u8);
        cell.cast()
    }

    /// Fills the cell's reference list (`+0xAC`) with `items`: the first node
    /// is the list head, the others are heap nodes.
    fn fill_list(e: &mut Engine, head: u32, items: &[Ptr]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, item.addr());
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    fn set_references(e: &mut Engine, cell: Ptr<TESObjectCELL>, items: &[Ptr]) {
        fill_list(e, cell.addr() + 0xac, items);
    }

    /// A string in memory.
    fn text(e: &mut Engine, string: &str) -> Ptr {
        let block = e.mem.alloc(string.len() as u32 + 1);
        e.mem.set_cstr(block, string.as_bytes());
        Ptr::new(block)
    }

    // ---- the small functions ---------------------------------------

    #[test]
    fn cell_flag_setters_set_and_clear_their_bits() {
        let mut e = engine();
        let cell = cell(&mut e, false);
        e.call(0x0054_4300, &args![cell, true]);
        assert_eq!(e.get(cell, TESObjectCELL::cCellFlags), 0x01);
        e.call(0x0054_4300, &args![cell, false]);
        assert_eq!(e.get(cell, TESObjectCELL::cCellFlags), 0x00);

        // The 0x20 and 0x40 setters also call virtual slot 0x48 with 2.
        e.call_log = Some(vec![]);
        e.call(0x0054_4340, &args![cell, true]);
        assert_eq!(e.get(cell, TESObjectCELL::cCellFlags), 0x20);
        e.call(0x0054_4390, &args![cell, true]);
        assert_eq!(e.get(cell, TESObjectCELL::cCellFlags), 0x60);
        e.call(0x0054_4340, &args![cell, false]);
        e.call(0x0054_4390, &args![cell, false]);
        assert_eq!(e.get(cell, TESObjectCELL::cCellFlags), 0x00);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, fake(0x48)), vec![vec![cell.addr(), 2]; 4]);

        // The game-flags setter changes bit 0 of the other byte.
        e.call_log = Some(vec![]);
        e.set(cell, TESObjectCELL::cCellGameFlags, 0xf0);
        e.call(0x0054_4440, &args![cell, true]);
        assert_eq!(e.get(cell, TESObjectCELL::cCellGameFlags), 0xf1);
        e.call(0x0054_4440, &args![cell, false]);
        assert_eq!(e.get(cell, TESObjectCELL::cCellGameFlags), 0xf0);
        assert_eq!(calls_to(&e.call_log.take().unwrap(), fake(0x48)).len(), 2);
    }

    #[test]
    fn cell_flag_readers() {
        let mut e = engine();
        let cell = cell(&mut e, false);
        assert!(!e.call(0x0054_4420, &args![cell]).bool());
        e.set(cell, TESObjectCELL::cCellFlags, 0x40);
        assert!(e.call(0x0054_4420, &args![cell]).bool());
        assert_eq!(e.call(0x0054_3c30, &args![cell]).u8(), 0x40);

        // "Public": cell flag 0x20 (through its accessor) or 0x40.
        e.register(CELL_FLAG_20, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x24) & 0x20 != 0).into_ret()
        });
        assert!(e.call(0x0054_43e0, &args![cell]).bool());
        e.set(cell, TESObjectCELL::cCellFlags, 0x20);
        assert!(e.call(0x0054_43e0, &args![cell]).bool());
        e.set(cell, TESObjectCELL::cCellFlags, 0x01);
        assert!(!e.call(0x0054_43e0, &args![cell]).bool());

        // Form flag 0x20000 through the flags accessor.
        let form = object(&mut e);
        assert!(!e.call(0x0054_4490, &args![form]).bool());
        e.mem.set_u32(form.addr() + 8, 0x0002_0000);
        assert!(e.call(0x0054_4490, &args![form]).bool());
    }

    #[test]
    fn block_keys_shift_arithmetically_and_pack_16_bits() {
        let mut e = engine();
        // (x >> 5, y >> 5) as 16-bit values, x in the high half.
        assert_eq!(
            e.call(0x0054_4280, &args![64i32, 33i32]).u32(),
            (2u32 << 16) | 1
        );
        // Negative coordinates keep their sign through the shift: -1 >> 5 = -1.
        assert_eq!(
            e.call(0x0054_4280, &args![-1i32, -33i32]).u32(),
            0xffff_fffeu32
        );
        // The sub-block key shifts by 3.
        assert_eq!(
            e.call(0x0054_42c0, &args![17i32, 8i32]).u32(),
            (2u32 << 16) | 1
        );
    }

    #[test]
    fn block_and_sub_block_keys_of_a_cell() {
        let mut e = engine();
        // Exterior: from the cell's coordinates (x, y at pCellData).
        let exterior = cell(&mut e, false);
        let data = e.mem.alloc(12);
        e.mem.set_i32(data, 70);
        e.mem.set_i32(data + 4, -40);
        e.mem.set_u32(exterior.addr() + 0x48, data);
        e.call_log = Some(vec![]);
        let block = e.call(0x0054_41b0, &args![exterior]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(
            block,
            ((70 >> 5) as u32) << 16 | ((-40i32 >> 5) as u16 as u32)
        );
        // Y is fetched before X, as the code evaluates its arguments.
        assert_eq!(
            sequence(&log)
                .into_iter()
                .filter(|a| *a == CELL_GET_DATA_X || *a == CELL_GET_DATA_Y)
                .collect::<Vec<_>>(),
            vec![CELL_GET_DATA_Y, CELL_GET_DATA_X]
        );
        let sub = e.call(0x0054_4210, &args![exterior]).u32();
        assert_eq!(
            sub,
            ((70 >> 3) as u32) << 16 | ((-40i32 >> 3) as u16 as u32)
        );

        // Interior: digits of the form id's low 24 bits.
        let interior = cell(&mut e, true);
        e.mem.set_u32(interior.addr() + 0xc, 0xff00_1234);
        assert_eq!(e.call(0x0054_41b0, &args![interior]).u32(), 0x1234 % 10);
        assert_eq!(
            e.call(0x0054_4210, &args![interior]).u32(),
            (0x1234 % 100) / 10
        );
    }

    #[test]
    fn noise_counter_reset_words() {
        let mut e = engine();
        e.set_global(STATIC_RESET_WORD, 9u32);
        e.call(0x0054_1cd0, &[]);
        assert_eq!(e.global::<u32>(STATIC_RESET_WORD), 0);
    }

    #[test]
    fn static_object_helpers() {
        let mut e = engine();
        let object = object(&mut e);
        e.mem.set_u32(object.addr() + SLOT_94_RESULT, 0x5151);
        assert_eq!(e.call(0x0054_1cb0, &args![object]).u32(), 0x5151);
        assert_eq!(e.call(0x0054_1c80, &args![object]).u32(), 0x5151);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0054_1c80, &args![Ptr::<()>::NULL]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn form_type_reader_and_flag_8() {
        let mut e = engine();
        let cell = cell(&mut e, false);
        e.mem.set_u8(cell.addr() + 4, 0x39);
        // 0x542d20 reads the byte at +0x619 of the loader singleton.
        let loader = e.mem.alloc(0x700);
        e.mem.set_u8(loader + 0x619, 1);
        assert!(e.call(0x0054_2d20, &args![Ptr::<()>::new(loader)]).bool());
        e.mem.set_u8(loader + 0x619, 0);
        assert!(!e.call(0x0054_2d20, &args![Ptr::<()>::new(loader)]).bool());
    }

    #[test]
    fn lock_helpers_use_the_lock_at_plus_0x80() {
        let mut e = engine();
        quiet(&mut e, &[LOCK_ENTER, LOCK_LEAVE]);
        let cell = cell(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0054_1ac0, &args![cell]);
        e.call(0x0054_1ae0, &args![cell]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOCK_ENTER),
            vec![vec![cell.addr() + 0x80, LOCK_NAME]]
        );
        assert_eq!(calls_to(&log, LOCK_LEAVE), vec![vec![cell.addr() + 0x80]]);
    }

    #[test]
    fn array_destructor_forwarder() {
        let mut e = engine();
        quiet(&mut e, &[NI_ARRAY_DESTRUCT]);
        e.call_log = Some(vec![]);
        e.call(0x0054_22b0, &args![Ptr::<()>::new(0x4000)]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), NI_ARRAY_DESTRUCT),
            vec![vec![0x4000]]
        );
    }

    // ---- constructor, destructors, statics --------------------------

    #[test]
    fn constructor_sets_the_default_state() {
        let mut e = engine();
        e.set_global(DEFAULT_WATER_HEIGHT, f32::MAX);
        // The sub-object constructors do nothing; the water noise texture's
        // constructor installs a vtable whose slot 0 the constructor calls.
        quiet(
            &mut e,
            &[
                FORM_CONSTRUCT,
                CELL_FULL_NAME_CONSTRUCT,
                EXTRA_LIST_CONSTRUCT,
                LIST_CONSTRUCT,
                FORM_SET_FLAG_8,
                fake(0x1000),
            ],
        );
        e.register(FORM_SET_FORM_TYPE, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret::default()
        });
        e.put_vtable(0x0131_0000, &[fake(0x1000)]);
        e.register(TEXTURE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0131_0000);
            Ret::default()
        });
        // A cell whose bytes are all 0xFF, so every store is visible.
        let cell: Ptr<TESObjectCELL> = e.new_object();
        e.mem.write(cell.addr(), &[0xff; 0xe0]);
        e.call_log = Some(vec![]);
        let back = e.call(0x0054_15b0, &args![cell]).ptr::<TESObjectCELL>();
        assert_eq!(back, cell);
        let log = e.call_log.take().unwrap();

        assert_eq!(e.mem.u32(cell.addr()), CELL_VTABLE);
        assert_eq!(e.mem.u32(cell.addr() + 0x18), CELL_FULL_NAME_VTABLE);
        assert_eq!(e.get(cell, TESObjectCELL::cCellFlags), 0);
        assert_eq!(e.get(cell, TESObjectCELL::cCellGameFlags), 0);
        assert_eq!(e.get(cell, TESObjectCELL::cCellState), 0);
        assert_eq!(
            e.get(cell, TESObjectCELL::iLightingTemplateInheritanceFlags),
            0x9f
        );
        assert!(e.get(cell, TESObjectCELL::pLightingTemplate).is_null());
        assert!(e.get(cell, TESObjectCELL::pCellData).is_null());
        assert!(e.get(cell, TESObjectCELL::pCellLand).is_null());
        assert_eq!(e.get(cell, TESObjectCELL::fWaterHeight), f32::MAX);
        assert!(e.get(cell, TESObjectCELL::pNavMeshes).is_null());
        assert!(e.get(cell, TESObjectCELL::pWorldSpace).is_null());
        assert!(e.get(cell, TESObjectCELL::pLoadedData).is_null());
        assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 0);
        assert_eq!(e.get(cell, TESObjectCELL::iCriticalQueuedRefCount), 0);
        assert_eq!(e.get(cell, TESObjectCELL::sNumRefsWithVisibleDistant), 0);
        assert_eq!(
            e.get(cell, TESObjectCELL::sNumLoadedRefsWithVisibleDistant),
            0
        );
        assert_eq!(e.get(cell, TESObjectCELL::fLodFadeInPercent), 0.0);
        for flag in [
            TESObjectCELL::bLODFadingIn,
            TESObjectCELL::bFadedIn,
            TESObjectCELL::bAutoWaterLoaded,
            TESObjectCELL::bFadingToLowDetail,
            TESObjectCELL::bFadingToHighDetail,
            TESObjectCELL::bDisplayHighDetail,
            TESObjectCELL::bCellDetached,
            TESObjectCELL::bUpdateTerrain,
        ] {
            assert!(!e.get(cell, flag));
        }
        // The four smart pointers are constructed empty, then the portal
        // graph is assigned null again.
        for off in [0xb4u32, 0xb8, 0xbc, 0xd4] {
            assert!(calls_to(&log, SLOT_CONSTRUCT).contains(&vec![cell.addr() + off, 0]));
            assert_eq!(e.mem.u32(cell.addr() + off), 0);
        }
        assert_eq!(
            calls_to(&log, SLOT_ASSIGN),
            vec![vec![cell.addr() + 0xd4, 0]]
        );
        // Form type 0x39, form flag 8, the lock and the reference list.
        assert_eq!(
            calls_to(&log, FORM_SET_FORM_TYPE),
            vec![vec![cell.addr(), 0x39]]
        );
        assert_eq!(e.mem.u8(cell.addr() + 4), 0x39);
        assert_eq!(calls_to(&log, FORM_SET_FLAG_8), vec![vec![cell.addr(), 1]]);
        assert_eq!(
            calls_to(&log, LIST_CONSTRUCT),
            vec![vec![cell.addr() + 0x80], vec![cell.addr() + 0xac]]
        );
        // The base form comes first; the texture's virtual slot 0 last.
        assert_eq!(sequence(&log)[0], FORM_CONSTRUCT);
        assert_eq!(*sequence(&log).last().unwrap(), fake(0x1000));
    }

    /// Doubles for everything the destructor calls, with a land, a nav mesh
    /// array and a region list present.
    fn destructor_engine() -> (Engine, Ptr<TESObjectCELL>) {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                CELL_CLEAR_STATE,
                CELL_RELEASE_STATE,
                REGION_LIST_RELEASE,
                WORLD_SPACE_RELEASE_CELL,
                CELL_CLEAR_REFERENCES,
                CELL_ERASE,
                EXTRA_LIST_REMOVE_NON_PERSISTENT,
                LIST_DESTRUCT,
                TEXTURE_DESTRUCT,
                EXTRA_LIST_DESTRUCT,
                CELL_FULL_NAME_DESTRUCT,
                FORM_DESTRUCT,
                NAV_MESH_ARRAY_CLEAR,
                NAV_MESH_ARRAY_BASE_DESTRUCT,
            ],
        );
        let cell = cell(&mut e, false);
        let land = object(&mut e);
        let nav_meshes = e.mem.alloc(0x10);
        let region = object(&mut e);
        let world = object(&mut e);
        e.set(cell, TESObjectCELL::pCellLand, land);
        e.set(cell, TESObjectCELL::pNavMeshes, Ptr::new(nav_meshes));
        e.set(cell, TESObjectCELL::pWorldSpace, world);
        let data = e.mem.alloc(0xc);
        e.set(cell, TESObjectCELL::pCellData, Ptr::new(data));
        // The region list is returned by the extra data list accessor.
        e.mem.set_u32(SCRATCH, region.addr());
        e.register(EXTRA_LIST_GET_REGION_LIST, |e, _| {
            e.mem.u32(SCRATCH).into_ret()
        });
        // The loader singleton.
        let loader = e.mem.alloc(0x700);
        e.set_global(LOADER_STATE_POINTER, loader);
        e.register(LOADER_STATE_FLAG_61D, |e, a| {
            (e.mem.u8(a[0] + 0x61d) as u32).into_ret()
        });
        (e, cell)
    }

    /// A mapped word the doubles use for state.
    const SCRATCH: u32 = 0x011c_c600;

    #[test]
    fn destructor_releases_everything_of_a_live_exterior_cell() {
        let (mut e, cell) = destructor_engine();
        let land = e.get(cell, TESObjectCELL::pCellLand);
        let nav_meshes = e.get(cell, TESObjectCELL::pNavMeshes);
        let region = Ptr::<()>::new(e.mem.u32(SCRATCH));
        let world = e.get(cell, TESObjectCELL::pWorldSpace);
        let data = e.get(cell, TESObjectCELL::pCellData);
        e.call_log = Some(vec![]);
        e.call(0x0054_1810, &args![cell]);
        let log = e.call_log.take().unwrap();

        assert_eq!(e.mem.u32(cell.addr()), CELL_VTABLE);
        assert_eq!(
            sequence(&log),
            vec![
                CELL_CLEAR_STATE,
                FORM_FLAG_4000,
                CELL_RELEASE_STATE,
                fake(0x10), // the land's deleting destructor
                // the nav mesh array's deleting destructor, then the free
                NAV_MESH_ARRAY_CLEAR,
                NAV_MESH_ARRAY_BASE_DESTRUCT,
                DEALLOCATE,
                EXTRA_LIST_GET_REGION_LIST,
                REGION_LIST_RELEASE,
                fake(0), // the region list's deleting destructor
                LOADER_STATE_FLAG_61D,
                CELL_GET_WORLD_SPACE,
                WORLD_SPACE_RELEASE_CELL,
                CELL_CLEAR_REFERENCES,
                CELL_ERASE,
                EXTRA_LIST_REMOVE_NON_PERSISTENT,
                CELL_IS_INTERIOR,
                DEALLOCATE, // pCellData
                CELL_SET_LIGHTING_TEMPLATE,
                SLOT_RELEASE,
                SLOT_RELEASE,
                SLOT_RELEASE,
                SLOT_RELEASE,
                LIST_DESTRUCT,
                TEXTURE_DESTRUCT,
                EXTRA_LIST_DESTRUCT,
                CELL_FULL_NAME_DESTRUCT,
                FORM_DESTRUCT,
            ]
        );
        assert_eq!(calls_to(&log, fake(0x10)), vec![vec![land.addr(), 1]]);
        assert_eq!(calls_to(&log, fake(0)), vec![vec![region.addr(), 1]]);
        assert_eq!(
            calls_to(&log, WORLD_SPACE_RELEASE_CELL),
            vec![vec![world.addr(), cell.addr()]]
        );
        // The nav mesh array is freed with the flag, and so is the data.
        assert_eq!(e.mem.block_size(nav_meshes.addr()), None);
        assert_eq!(e.mem.block_size(data.addr()), None);
        assert!(e.get(cell, TESObjectCELL::pCellLand).is_null());
        assert!(e.get(cell, TESObjectCELL::pNavMeshes).is_null());
        assert!(e.get(cell, TESObjectCELL::pCellData).is_null());
        // The slots are released in the reverse order of construction.
        let released: Vec<u32> = calls_to(&log, SLOT_RELEASE).iter().map(|c| c[0]).collect();
        assert_eq!(
            released,
            vec![
                cell.addr() + 0xd4,
                cell.addr() + 0xbc,
                cell.addr() + 0xb8,
                cell.addr() + 0xb4
            ]
        );
    }

    #[test]
    fn destructor_skips_the_release_work_for_a_flagged_form() {
        let (mut e, cell) = destructor_engine();
        e.mem.set_u32(cell.addr() + 8, 0x4000);
        // The loader says the cell must stay registered with its world.
        let loader = e.global::<u32>(LOADER_STATE_POINTER);
        e.mem.set_u8(loader + 0x61d, 1);
        e.call_log = Some(vec![]);
        e.call(0x0054_1810, &args![cell]);
        let log = e.call_log.take().unwrap();
        let seq = sequence(&log);
        assert!(!seq.contains(&CELL_RELEASE_STATE));
        assert!(!seq.contains(&fake(0x10)));
        assert!(!seq.contains(&EXTRA_LIST_GET_REGION_LIST));
        assert!(!seq.contains(&WORLD_SPACE_RELEASE_CELL));
        assert!(!seq.contains(&CELL_GET_WORLD_SPACE));
        // The land and nav meshes are left in place.
        assert!(!e.get(cell, TESObjectCELL::pCellLand).is_null());
        assert!(!e.get(cell, TESObjectCELL::pNavMeshes).is_null());
        assert!(seq.contains(&FORM_DESTRUCT));
    }

    #[test]
    fn destructor_without_a_world_space_does_not_release_the_cell() {
        let (mut e, cell) = destructor_engine();
        e.set(cell, TESObjectCELL::pWorldSpace, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0054_1810, &args![cell]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, WORLD_SPACE_RELEASE_CELL).is_empty());
        assert_eq!(calls_to(&log, CELL_GET_WORLD_SPACE).len(), 1);
    }

    #[test]
    fn scalar_deleting_destructors_free_on_request() {
        let (mut e, cell) = destructor_engine();
        e.call_log = Some(vec![]);
        let back = e.call(0x0054_17e0, &args![cell, 0u32]).ptr::<()>();
        assert_eq!(back.addr(), cell.addr());
        let log = e.call_log.take().unwrap();
        assert!(e.mem.block_size(cell.addr()).is_some());
        assert!(!calls_to(&log, DEALLOCATE)
            .iter()
            .any(|c| c[0] == cell.addr()));
        // Bit 0 of the flags decides; other bits do not.
        let (mut e, cell) = destructor_engine();
        e.call_log = Some(vec![]);
        e.call(0x0054_17e0, &args![cell, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(*log.last().unwrap(), (DEALLOCATE, vec![cell.addr()]));
        assert_eq!(e.mem.block_size(cell.addr()), None);

        // The nav mesh array's form.
        let mut e = engine();
        quiet(
            &mut e,
            &[NAV_MESH_ARRAY_CLEAR, NAV_MESH_ARRAY_BASE_DESTRUCT],
        );
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0054_1a30, &args![Ptr::<()>::new(array), 0u32])
                .u32(),
            array
        );
        assert_eq!(
            sequence(&e.call_log.take().unwrap()),
            vec![NAV_MESH_ARRAY_CLEAR, NAV_MESH_ARRAY_BASE_DESTRUCT]
        );
        assert!(e.mem.block_size(array).is_some());
        e.call(0x0054_1a30, &args![Ptr::<()>::new(array), 1u32]);
        assert_eq!(e.mem.block_size(array), None);
    }

    #[test]
    fn clearing_the_cell_data() {
        let mut e = engine();
        quiet(&mut e, &[EXTRA_LIST_REMOVE_NON_PERSISTENT]);
        for interior in [true, false] {
            let cell = cell(&mut e, interior);
            let data = e.mem.alloc(0x2c);
            e.set(cell, TESObjectCELL::pCellData, Ptr::new(data));
            e.set(cell, TESObjectCELL::pLightingTemplate, Ptr::new(0x1234));
            e.set(cell, TESObjectCELL::iLightingTemplateInheritanceFlags, 0x9f);
            e.call_log = Some(vec![]);
            e.call(0x0054_1b00, &args![cell]);
            let log = e.call_log.take().unwrap();
            // The same field is freed whatever the cell's kind.
            assert_eq!(calls_to(&log, DEALLOCATE), vec![vec![data]]);
            assert_eq!(
                calls_to(&log, EXTRA_LIST_REMOVE_NON_PERSISTENT),
                vec![vec![cell.addr() + 0x28]]
            );
            assert!(e.get(cell, TESObjectCELL::pCellData).is_null());
            assert!(e.get(cell, TESObjectCELL::pLightingTemplate).is_null());
            assert_eq!(
                e.get(cell, TESObjectCELL::iLightingTemplateInheritanceFlags),
                0
            );
        }
    }

    #[test]
    fn init_statics_raises_a_setting_above_2048() {
        for (value, raised) in [
            (3000.0f32, true),
            (2048.0, false),
            (1000.0, false),
            (f32::NAN, false),
        ] {
            let mut e = engine();
            // The three settings words the builder gets.
            for (i, word) in [0x1111u32, 0x2222, 0x3333].iter().enumerate() {
                e.set_global(STATIC_SETTINGS_SOURCE + 4 * i as u32, *word);
            }
            e.set_global(STATIC_FLAG, 7u32);
            e.set_global(SETTING_LIMIT_DOUBLE, 2048.0f64);
            e.set_global(SETTING_LIMIT_FLOAT, 2048.0f32);
            // The builder returns a pointer and keeps what it was given.
            let given = Rc::new(RefCell::new(vec![]));
            let seen = given.clone();
            e.register_double(STATIC_SETTING_BUILDER, move |e, a| {
                *seen.borrow_mut() = e.mem.bytes(a[0], 12);
                0x4444u32.into_ret()
            });
            let setting = e.mem.alloc(4);
            e.mem.set_f32(setting, value);
            returns(&mut e, STATIC_SETTING_GET, setting);
            quiet(&mut e, &[STATIC_SETTING_SET]);
            e.call_log = Some(vec![]);
            e.call(0x0054_1b80, &[]);
            let log = e.call_log.take().unwrap();
            let words: Vec<u32> = given
                .borrow()
                .chunks(4)
                .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect();
            assert_eq!(words, vec![0x1111, 0x2222, 0x3333]);
            assert_eq!(e.global::<u32>(STATIC_FLAG), 0);
            assert_eq!(e.mem.u32(STATIC_POINTER_SLOT), 0x4444);
            assert_eq!(
                calls_to(&log, STATIC_SETTING_SET),
                if raised {
                    vec![vec![STATIC_SETTING_BLOCK, 2048.0f32.to_bits()]]
                } else {
                    vec![]
                }
            );
        }
    }

    #[test]
    fn releasing_the_statics() {
        let mut e = engine();
        let held = object(&mut e).addr();
        e.set_global(STATIC_OBJECT, held);
        e.set_global(STATIC_RESET_WORD, 5u32);
        quiet(&mut e, &[STATIC_RELEASE_HELPER]);
        let target = object_with_answer(&mut e, 0x77);
        e.mem.set_u32(STATIC_POINTER_SLOT, target.addr());
        e.call_log = Some(vec![]);
        e.call(0x0054_1c00, &[]);
        let log = e.call_log.take().unwrap();
        // The helper gets the static object and the value the pointed
        // object's slot 0x94 produced; the static object is deleted.
        assert_eq!(
            calls_to(&log, STATIC_RELEASE_HELPER),
            vec![vec![held, 0x77]]
        );
        assert_eq!(calls_to(&log, fake(0)), vec![vec![held, 1]]);
        assert_eq!(e.global::<u32>(STATIC_OBJECT), 0);
        assert_eq!(e.mem.u32(STATIC_POINTER_SLOT), 0);
        assert_eq!(e.global::<u32>(STATIC_RESET_WORD), 0);

        // Without a static object only the slot and the word are cleared.
        e.set_global(STATIC_RESET_WORD, 5u32);
        e.mem.set_u32(STATIC_POINTER_SLOT, target.addr());
        e.call_log = Some(vec![]);
        e.call(0x0054_1c00, &[]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, STATIC_RELEASE_HELPER).is_empty());
        assert_eq!(e.mem.u32(STATIC_POINTER_SLOT), 0);
        assert_eq!(e.global::<u32>(STATIC_RESET_WORD), 0);
    }

    /// An object whose virtual slot 0x94 answers `answer`.
    fn object_with_answer(e: &mut Engine, answer: u32) -> Ptr {
        let target = object(e);
        e.mem.set_u32(target.addr() + SLOT_94_RESULT, answer);
        target
    }

    // ---- saving ----------------------------------------------------

    /// Doubles for the record-writing helpers; the chunk calls are only
    /// recorded.
    fn save_engine() -> Engine {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                FORM_START_FORM,
                FULL_NAME_SAVE,
                FORM_ADD_CHUNK_BYTE,
                FORM_ADD_CHUNK_DATA,
                FORM_ADD_CHUNK_WORD,
                FORM_ADD_CHUNK_ARRAY,
                EXTRA_LIST_SAVE,
                FORM_CLOSE_FORM,
                SWAP_INTERIOR_DATA,
                SWAP_EXTERIOR_DATA,
                IMPACT_SWAP_SAVE,
            ],
        );
        returns(&mut e, ENDIAN_SWAP_ENABLED, 0);
        returns(&mut e, EXTRA_LIST_GET_IMPACT_SWAP, 0);
        returns(&mut e, FULL_NAME_LENGTH, 4);
        returns(&mut e, TEXT_GET_STRING, 0x0900_0000);
        e
    }

    #[test]
    fn saving_the_interior_record() {
        let mut e = save_engine();
        returns(&mut e, ENDIAN_SWAP_ENABLED, 1);
        returns(&mut e, EXTRA_LIST_GET_IMPACT_SWAP, 0x6000);
        let cell = cell(&mut e, true);
        e.set(cell, TESObjectCELL::cCellFlags, 0x81);
        let data = e.mem.alloc(0x2c);
        e.set(cell, TESObjectCELL::pCellData, Ptr::new(data));
        let template = object(&mut e);
        e.mem.set_u32(template.addr() + 0xc, 0xab);
        e.set(cell, TESObjectCELL::pLightingTemplate, template);
        e.set(cell, TESObjectCELL::iLightingTemplateInheritanceFlags, 0x9f);
        e.set(cell, TESObjectCELL::fWaterHeight, 12.5);
        e.call_log = Some(vec![]);
        e.call(0x0054_1ce0, &args![cell]);
        let log = e.call_log.take().unwrap();
        let swap = ENDIAN_SWAP_ENABLED;
        assert_eq!(
            sequence(&log),
            vec![
                FORM_START_FORM,
                FULL_NAME_SAVE,
                FORM_ADD_CHUNK_BYTE,
                CELL_IS_INTERIOR,
                CELL_INTERIOR_DATA,
                swap,
                SWAP_INTERIOR_DATA,
                FORM_ADD_CHUNK_DATA,
                swap,
                SWAP_INTERIOR_DATA,
                CELL_EXTRA_DATA_LIST,
                EXTRA_LIST_GET_IMPACT_SWAP,
                IMPACT_SWAP_SAVE,
                CELL_GET_LIGHTING_TEMPLATE,
                CELL_GET_LIGHTING_TEMPLATE,
                FORM_ID,
                FORM_ADD_CHUNK_WORD,
                FORM_ADD_CHUNK_WORD,
                FORM_ADD_CHUNK_WORD,
                FULL_NAME_LENGTH,
                TEXT_GET_STRING,
                FORM_ADD_CHUNK_ARRAY,
                EXTRA_LIST_SAVE,
                FORM_CLOSE_FORM,
            ]
        );
        assert_eq!(
            calls_to(&log, FULL_NAME_SAVE),
            vec![vec![cell.addr() + 0x18]]
        );
        assert_eq!(calls_to(&log, FORM_ADD_CHUNK_BYTE), vec![vec![DATA, 0x81]]);
        assert_eq!(
            calls_to(&log, FORM_ADD_CHUNK_DATA),
            vec![vec![INTERIOR_LIGHTING, data, 0x2c]]
        );
        assert_eq!(
            calls_to(&log, SWAP_INTERIOR_DATA),
            vec![vec![data], vec![data]]
        );
        assert_eq!(calls_to(&log, IMPACT_SWAP_SAVE), vec![vec![0x6000]]);
        assert_eq!(
            calls_to(&log, FORM_ADD_CHUNK_WORD),
            vec![
                vec![LIGHTING_TEMPLATE_ID, 0xab],
                vec![INHERITANCE_FLAGS, 0x9f],
                vec![WATER_HEIGHT, 12.5f32.to_bits()],
            ]
        );
        // The texture name goes out with its length plus the terminator.
        assert_eq!(
            calls_to(&log, FORM_ADD_CHUNK_ARRAY),
            vec![vec![WATER_NOISE_TEXTURE, 0x0900_0000, 5]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_LIST_SAVE),
            vec![vec![cell.addr() + 0x28]]
        );
    }

    #[test]
    fn saving_the_exterior_record() {
        let mut e = save_engine();
        let cell = cell(&mut e, false);
        let data = e.mem.alloc(0xc);
        e.set(cell, TESObjectCELL::pCellData, Ptr::new(data));
        e.call_log = Some(vec![]);
        e.call(0x0054_1ce0, &args![cell]);
        let log = e.call_log.take().unwrap();
        // No byte swapping, no impact swap; a missing lighting template is
        // saved as id 0.
        assert_eq!(
            calls_to(&log, FORM_ADD_CHUNK_DATA),
            vec![vec![EXTERIOR_COORDINATES, data, 0xc]]
        );
        assert!(calls_to(&log, SWAP_EXTERIOR_DATA).is_empty());
        assert!(calls_to(&log, EXTRA_LIST_GET_IMPACT_SWAP).is_empty());
        assert_eq!(
            calls_to(&log, FORM_ADD_CHUNK_WORD)[0],
            vec![LIGHTING_TEMPLATE_ID, 0]
        );
        assert_eq!(calls_to(&log, FORM_ADD_CHUNK_BYTE), vec![vec![DATA, 0]]);

        // With the swap flag on, the exterior block is swapped around the write.
        returns(&mut e, ENDIAN_SWAP_ENABLED, 1);
        e.call_log = Some(vec![]);
        e.call(0x0054_1ce0, &args![cell]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SWAP_EXTERIOR_DATA),
            vec![vec![data], vec![data]]
        );
    }

    /// Doubles for a small `NiTArray` of pointers (+4 base, +0xA size, +0xC
    /// element count) and the file calls `SaveReferences` makes.
    fn array_doubles(e: &mut Engine) {
        quiet(
            e,
            &[LOCK_ENTER, LOCK_LEAVE, FORM_SAVE_TO_FILE, NI_ARRAY_DESTRUCT],
        );
        e.register(NI_ARRAY_CONSTRUCT, |e, a| {
            e.mem.write(a[0], &[0; 0x10]);
            a[0].into_ret()
        });
        e.register(NI_ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(NI_ARRAY_SET_GROW_BY, |e, a| {
            e.mem.set_u16(a[0] + 0xe, a[1] as u16);
            Ret::default()
        });
        e.register(NI_ARRAY_ADD, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(base + 4 * size as u32, value);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            e.mem.set_u16(a[0] + 0xc, e.mem.u16(a[0] + 0xc) + 1);
            (size as u32).into_ret()
        });
        quiet(e, &[NI_ARRAY_COMPACT]);
        e.register(NI_ARRAY_COUNT, |e, a| {
            (e.mem.u16(a[0] + 0xc) as u32).into_ret()
        });
        e.register(NI_ARRAY_ELEMENT_ADDRESS, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(NI_ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(e.mem.u32(a[0] + 4) + 4 * a[1], value);
            Ret::default()
        });
    }

    /// A file object (the master flag at +0x10).
    fn new_file(e: &mut Engine, master: bool) -> Ptr {
        let file = e.mem.alloc(0x300);
        e.mem.set_u8(file + 0x10, master as u8);
        Ptr::new(file)
    }

    /// A reference with form flags, the file it was last saved in (`+0x80`),
    /// its first file (`+0x84`) and a sort key.
    fn reference(e: &mut Engine, flags: u32, file: Ptr, first_file: Ptr, key: u32) -> Ptr {
        let reference = object(e);
        e.mem.set_u32(reference.addr() + 8, flags);
        e.mem.set_u32(reference.addr() + 0x80, file.addr());
        e.mem.set_u32(reference.addr() + 0x84, first_file.addr());
        e.mem.set_u32(reference.addr() + KEY, key);
        reference
    }

    #[test]
    fn saving_references_filters_sorts_and_saves() {
        let mut e = engine();
        array_doubles(&mut e);
        let target = new_file(&mut e, false);
        let other = new_file(&mut e, false);
        let master = new_file(&mut e, true);
        let none = Ptr::NULL;
        // Kept: normal references, a deleted one from a master file, and one
        // from another file that has form flag 2.
        let r_a = reference(&mut e, 0, target, none, 3);
        let r_b = reference(&mut e, 0, target, none, 1);
        let r_deleted_master = reference(&mut e, 0x20, target, master, 2);
        let r_other_flag2 = reference(&mut e, 2, other, none, 0);
        // Skipped: flag 0x4000, deleted from a non-master file, from another
        // file without flag 2, persistent reference in a non-persistent
        // exterior cell.
        let r_4000 = reference(&mut e, 0x4000, target, none, 9);
        let r_deleted_other = reference(&mut e, 0x20, target, other, 9);
        let r_other = reference(&mut e, 0, other, none, 9);
        let r_persistent = reference(&mut e, 0x800, target, none, 9);
        let cell = cell(&mut e, false);
        set_references(
            &mut e,
            cell,
            &[
                r_a,
                r_4000,
                r_b,
                r_deleted_other,
                r_other,
                r_persistent,
                r_deleted_master,
                r_other_flag2,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0054_2040, &args![cell, target]);
        let log = e.call_log.take().unwrap();
        let saved: Vec<Vec<u32>> = calls_to(&log, FORM_SAVE_TO_FILE);
        // Ascending by key: 0, 1, 2, 3.
        assert_eq!(
            saved,
            vec![
                vec![r_other_flag2.addr(), target.addr()],
                vec![r_b.addr(), target.addr()],
                vec![r_deleted_master.addr(), target.addr()],
                vec![r_a.addr(), target.addr()],
            ]
        );
        // The array is built with 50 slots and grows by 50, the lock is held
        // only while collecting.
        assert_eq!(calls_to(&log, NI_ARRAY_CONSTRUCT)[0][1..], [0, 1]);
        assert_eq!(calls_to(&log, NI_ARRAY_SET_SIZE)[0][1], 0x32);
        assert_eq!(calls_to(&log, NI_ARRAY_SET_GROW_BY)[0][1], 0x32);
        let seq = sequence(&log);
        let enter = seq.iter().position(|a| *a == LOCK_ENTER).unwrap();
        let leave = seq.iter().position(|a| *a == LOCK_LEAVE).unwrap();
        let compact = seq.iter().position(|a| *a == NI_ARRAY_COMPACT).unwrap();
        let first_save = seq.iter().position(|a| *a == FORM_SAVE_TO_FILE).unwrap();
        assert!(enter < leave && leave < compact && compact < first_save);
        assert_eq!(calls_to(&log, NI_ARRAY_DESTRUCT).len(), 1);

        // An interior cell keeps persistent references too.
        let interior = cell_interior_with(&mut e, &[r_persistent]);
        e.call_log = Some(vec![]);
        e.call(0x0054_2040, &args![interior, target]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FORM_SAVE_TO_FILE),
            vec![vec![r_persistent.addr(), target.addr()]]
        );
    }

    fn cell_interior_with(e: &mut Engine, items: &[Ptr]) -> Ptr<TESObjectCELL> {
        let interior = cell(e, true);
        set_references(e, interior, items);
        interior
    }

    /// Records the group header (five words) passed to `StartGroup`.
    fn record_groups(e: &mut Engine) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let headers = Rc::new(RefCell::new(vec![]));
        let seen = headers.clone();
        e.register_double(FILE_START_GROUP, move |e, a| {
            let words = (0..5).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            seen.borrow_mut().push(words);
            Ret::default()
        });
        headers
    }

    #[test]
    fn saving_a_cell_in_a_master_file() {
        let mut e = engine();
        array_doubles(&mut e);
        quiet(&mut e, &[FILE_ADD_FORM]);
        let headers = record_groups(&mut e);
        let master = new_file(&mut e, true);
        let cell = cell(&mut e, false);
        e.mem.set_u32(cell.addr() + 0xc, 0x0100_00ab);
        // The cell's first file is the master: its references are saved too.
        e.mem.set_u32(cell.addr() + 0x84, master.addr());
        let reference = reference(&mut e, 0, master, Ptr::NULL, 1);
        set_references(&mut e, cell, &[reference]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_1e80, &args![cell, master]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            sequence(&log)[..4],
            [FORM_FLAG_20, LOCK_ENTER, FILE_IS_MASTER, LOCK_LEAVE]
        );
        // Form save (virtual slot 0x2c), then the file takes the form and the
        // group starts: ('GRUP', 0, the cell's id, type 6, 0).
        assert_eq!(calls_to(&log, fake(0x2c)), vec![vec![cell.addr()]]);
        assert_eq!(
            calls_to(&log, FILE_ADD_FORM),
            vec![vec![master.addr(), cell.addr()]]
        );
        assert_eq!(headers.borrow()[0], vec![0x5055_5247, 0, 0x0100_00ab, 6, 0]);
        assert_eq!(
            calls_to(&log, FORM_SAVE_TO_FILE),
            vec![vec![reference.addr(), master.addr()]]
        );

        // The cell came from another file: no references are saved.
        let elsewhere = new_file(&mut e, false);
        e.mem.set_u32(cell.addr() + 0x84, elsewhere.addr());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_1e80, &args![cell, master]).bool());
        assert!(calls_to(&e.call_log.take().unwrap(), FORM_SAVE_TO_FILE).is_empty());
    }

    #[test]
    fn saving_a_cell_depends_on_its_references() {
        let mut e = engine();
        array_doubles(&mut e);
        quiet(&mut e, &[FILE_ADD_FORM, FILE_START_GROUP]);
        let target = new_file(&mut e, false);
        let other = new_file(&mut e, false);
        // Only references from another file, none with flag 2: not saved.
        let cell = cell(&mut e, false);
        let foreign = reference(&mut e, 0, other, Ptr::NULL, 1);
        set_references(&mut e, cell, &[foreign]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0054_1e80, &args![cell, target]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, fake(0x2c)).is_empty());
        // The lock is left again.
        assert_eq!(
            calls_to(&log, LOCK_ENTER).len(),
            calls_to(&log, LOCK_LEAVE).len()
        );

        // A reference without a file makes the cell wanted...
        let nowhere = reference(&mut e, 0, Ptr::NULL, Ptr::NULL, 1);
        let cell2 = cell_with(&mut e, false, &[foreign, nowhere]);
        assert!(e.call(0x0054_1e80, &args![cell2, target]).bool());
        // ...as does one from this very file, or the cell's own flag 2.
        let ours = reference(&mut e, 0, target, Ptr::NULL, 1);
        let cell3 = cell_with(&mut e, false, &[ours]);
        assert!(e.call(0x0054_1e80, &args![cell3, target]).bool());
        let cell4 = cell_with(&mut e, false, &[foreign]);
        e.mem.set_u32(cell4.addr() + 8, 2);
        assert!(e.call(0x0054_1e80, &args![cell4, target]).bool());

        // A deleted cell is never saved.
        let deleted = cell_with(&mut e, false, &[ours]);
        e.mem.set_u32(deleted.addr() + 8, 0x20);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0054_1e80, &args![deleted, target]).bool());
        assert_eq!(sequence(&e.call_log.take().unwrap()), vec![FORM_FLAG_20]);
    }

    fn cell_with(e: &mut Engine, interior: bool, items: &[Ptr]) -> Ptr<TESObjectCELL> {
        let cell = cell(e, interior);
        set_references(e, cell, items);
        cell
    }

    #[test]
    fn saving_an_edit_copy_always_saves_references() {
        let mut e = engine();
        array_doubles(&mut e);
        quiet(&mut e, &[FILE_ADD_FORM]);
        let headers = record_groups(&mut e);
        let target = new_file(&mut e, false);
        let cell = cell_with(&mut e, false, &[]);
        e.mem.set_u32(cell.addr() + 0xc, 0x77);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_1fd0, &args![cell, target]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, fake(0x2c)), vec![vec![cell.addr()]]);
        assert_eq!(
            calls_to(&log, FILE_ADD_FORM),
            vec![vec![target.addr(), cell.addr()]]
        );
        assert_eq!(headers.borrow()[0], vec![0x5055_5247, 0, 0x77, 6, 0]);
        assert_eq!(calls_to(&log, NI_ARRAY_CONSTRUCT).len(), 1);
    }

    // ---- loading ---------------------------------------------------

    /// The chunk the file under test is on, and the ones still to come.
    struct Script {
        queue: VecDeque<(u32, Vec<u8>)>,
        current: Vec<u8>,
    }

    /// Doubles for the TESFile accessors and the helpers `Load` calls. The
    /// file yields `chunks` in order (each as tag, bytes); `swap` is the
    /// file's endian-swap flag. Returns the script so tests can look at what
    /// is left.
    fn load_engine(
        form_type: u8,
        chunks: &[(u32, Vec<u8>)],
        swap: bool,
    ) -> (Engine, Rc<RefCell<Script>>) {
        let mut e = engine();
        let script = Rc::new(RefCell::new(Script {
            queue: chunks.iter().cloned().collect(),
            current: vec![],
        }));
        e.register_double(FILE_GET_FORM_TYPE, move |_, _| {
            (form_type as u32).into_ret()
        });
        returns(&mut e, FILE_MASTER_DATA, 0x5150);
        returns(&mut e, FILE_SWAP_ENDIAN, swap as u32);
        let s = script.clone();
        e.register_double(FILE_GET_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            match s.queue.pop_front() {
                Some((tag, bytes)) => {
                    s.current = bytes;
                    tag.into_ret()
                }
                None => 0u32.into_ret(),
            }
        });
        let s = script.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            u32::from(!s.borrow().queue.is_empty()).into_ret()
        });
        let s = script.clone();
        e.register_double(FILE_CHUNK_SIZE, move |_, _| {
            (s.borrow().current.len() as u32).into_ret()
        });
        let s = script.clone();
        e.register_double(FILE_GET_CHUNK_DATA, move |e, a| {
            e.mem.write(a[1], &s.borrow().current);
            Ret::default()
        });
        let s = script.clone();
        e.register_double(FILE_GET_CHUNK_DATA_SIZED, move |e, a| {
            let s = s.borrow();
            let count = (a[2] as usize).min(s.current.len());
            e.mem.write(a[1], &s.current[..count]);
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                FORM_LOAD_FORM,
                FORM_SET_FLAG_8,
                CELL_CREATE_CELL_DATA,
                CELL_SET_WATER_HEIGHT,
                CELL_SET_LOADED_MASTER_DATA,
                FULL_NAME_LOAD,
                FULL_NAME_SET,
                EXTRA_LIST_LOAD,
                EXTRA_LIST_SET_IMPACT_SWAP,
                IMPACT_SWAP_LOAD,
                SWAP_INTERIOR_DATA,
                SWAP_EXTERIOR_DATA,
            ],
        );
        returns(&mut e, EXTRA_LIST_GET_IMPACT_SWAP, 0);
        e.register(IMPACT_SWAP_CONSTRUCT, |_, a| a[0].into_ret());
        e.set_global(DEFAULT_WATER_HEIGHT, f32::MAX);
        let loader = e.mem.alloc(0x700);
        e.set_global(LOADER_STATE_POINTER, loader);
        (e, script)
    }

    fn loading_cell(e: &mut Engine, interior: bool) -> Ptr<TESObjectCELL> {
        let cell = cell(e, interior);
        e.mem.set_u8(cell.addr() + 4, 0x39);
        cell
    }

    #[test]
    fn load_rejects_other_records_and_loads_the_base_form_first() {
        let (mut e, _) = load_engine(0x3a, &[(DATA, vec![1, 0, 0, 0])], false);
        let cell = loading_cell(&mut e, false);
        let file = new_file(&mut e, false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0054_22d0, &args![cell, file]).bool());
        assert_eq!(
            sequence(&e.call_log.take().unwrap()),
            vec![FILE_GET_FORM_TYPE]
        );

        // A cell record: the form is loaded, the chunks read until none is
        // left, and form flag 8 is cleared.
        let (mut e, _) = load_engine(0x39, &[(tag(b"ZZZZ"), vec![])], false);
        let cell = loading_cell(&mut e, false);
        let file = new_file(&mut e, false);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_22d0, &args![cell, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            sequence(&log),
            vec![
                FILE_GET_FORM_TYPE,
                FILE_IS_MASTER,
                FORM_LOAD_FORM,
                FILE_GET_CHUNK,
                FILE_NEXT_CHUNK,
                FORM_SET_FLAG_8,
            ]
        );
        assert_eq!(
            calls_to(&log, FORM_LOAD_FORM),
            vec![vec![cell.addr(), file.addr()]]
        );
        assert_eq!(calls_to(&log, FORM_SET_FLAG_8), vec![vec![cell.addr(), 0]]);
    }

    #[test]
    fn load_reads_the_cell_flags_of_an_interior() {
        // A 4-byte DATA chunk: interior (bit 0) with the water flag (0x80).
        let (mut e, _) = load_engine(0x39, &[(DATA, vec![0x81, 0, 0, 0])], false);
        let file = new_file(&mut e, true);
        let cell = loading_cell(&mut e, true);
        // The loader singleton's byte +0x619 is set: bit 0x40 stays.
        let loader = e.global::<u32>(LOADER_STATE_POINTER);
        e.mem.set_u8(loader + 0x619, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_22d0, &args![cell, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(cell, TESObjectCELL::cCellFlags), 0x81);
        // Interior with 0x80: the default water height is set.
        assert_eq!(
            calls_to(&log, CELL_SET_WATER_HEIGHT),
            vec![vec![cell.addr(), f32::MAX.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, CELL_CREATE_CELL_DATA),
            vec![vec![cell.addr()]]
        );
        // The file is a master: its master data goes to the interior cell.
        assert_eq!(
            calls_to(&log, CELL_SET_LOADED_MASTER_DATA),
            vec![vec![cell.addr(), 0x5150]]
        );
        // The master data was fetched before the form was loaded.
        let seq = sequence(&log);
        let master_data = seq.iter().position(|a| *a == FILE_MASTER_DATA).unwrap();
        let load_form = seq.iter().position(|a| *a == FORM_LOAD_FORM).unwrap();
        assert!(master_data < load_form);
    }

    #[test]
    fn load_reads_the_cell_flags_of_an_exterior() {
        // A 1-byte DATA chunk: bit 0 clear sets bit 1; the loader byte is
        // clear, so bit 0x40 goes away.
        let (mut e, _) = load_engine(0x39, &[(DATA, vec![0x40])], false);
        let file = new_file(&mut e, false);
        let cell = loading_cell(&mut e, false);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_22d0, &args![cell, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(cell, TESObjectCELL::cCellFlags), 0x02);
        assert!(calls_to(&log, CELL_SET_WATER_HEIGHT).is_empty());
        assert!(calls_to(&log, CELL_SET_LOADED_MASTER_DATA).is_empty());
        // The 1-byte form reads straight into the flags byte.
        assert_eq!(
            calls_to(&log, FILE_GET_CHUNK_DATA_SIZED),
            vec![vec![file.addr(), cell.addr() + 0x24, 1]]
        );
    }

    #[test]
    fn load_reads_and_swaps_the_cell_data_blocks() {
        let exterior_chunk = vec![(EXTERIOR_COORDINATES, (0..12).collect::<Vec<u8>>())];
        // The file needs swapping and the chunk has the expected size: the
        // block is swapped once, after the read.
        let (mut e, _) = load_engine(0x39, &exterior_chunk, true);
        let file = new_file(&mut e, false);
        let cell = loading_cell(&mut e, false);
        let data = e.mem.alloc(12);
        e.set(cell, TESObjectCELL::pCellData, Ptr::new(data));
        e.call_log = Some(vec![]);
        e.call(0x0054_22d0, &args![cell, file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.bytes(data, 12), (0..12).collect::<Vec<u8>>());
        assert_eq!(calls_to(&log, SWAP_EXTERIOR_DATA), vec![vec![data]]);

        // A chunk of another size is swapped before the read as well.
        let short = vec![(EXTERIOR_COORDINATES, vec![9; 8])];
        let (mut e, _) = load_engine(0x39, &short, true);
        let file = new_file(&mut e, false);
        let cell = loading_cell(&mut e, false);
        let data = e.mem.alloc(12);
        e.set(cell, TESObjectCELL::pCellData, Ptr::new(data));
        e.call_log = Some(vec![]);
        e.call(0x0054_22d0, &args![cell, file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SWAP_EXTERIOR_DATA),
            vec![vec![data], vec![data]]
        );
        assert_eq!(
            calls_to(&log, FILE_GET_CHUNK_DATA_SIZED),
            vec![vec![file.addr(), data, 12]]
        );

        // Without the swap flag nothing is swapped; the interior block is
        // 44 bytes; no block, nothing read.
        let interior_chunk = vec![(INTERIOR_LIGHTING, vec![7; 44])];
        let (mut e, _) = load_engine(0x39, &interior_chunk, false);
        let file = new_file(&mut e, false);
        let cell = loading_cell(&mut e, true);
        let data = e.mem.alloc(44);
        e.set(cell, TESObjectCELL::pCellData, Ptr::new(data));
        e.call_log = Some(vec![]);
        e.call(0x0054_22d0, &args![cell, file]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SWAP_INTERIOR_DATA).is_empty());
        assert_eq!(e.mem.bytes(data, 44), vec![7; 44]);
        let (mut e, _) = load_engine(0x39, &interior_chunk, false);
        let file = new_file(&mut e, false);
        let cell = loading_cell(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0054_22d0, &args![cell, file]);
        assert!(calls_to(&e.call_log.take().unwrap(), FILE_GET_CHUNK_DATA_SIZED).is_empty());
    }

    #[test]
    fn load_reads_the_simple_chunks() {
        let chunks = vec![
            (WATER_HEIGHT, 7.5f32.to_le_bytes().to_vec()),
            (INHERITANCE_FLAGS, 0x1234u32.to_le_bytes().to_vec()),
            (LIGHTING_TEMPLATE_ID, 0x0100_0042u32.to_le_bytes().to_vec()),
        ];
        let (mut e, _) = load_engine(0x39, &chunks, false);
        let file = new_file(&mut e, false);
        let cell = loading_cell(&mut e, false);
        e.call(0x0054_22d0, &args![cell, file]);
        assert_eq!(e.get(cell, TESObjectCELL::fWaterHeight), 7.5);
        assert_eq!(
            e.get(cell, TESObjectCELL::iLightingTemplateInheritanceFlags),
            0x1234
        );
        assert_eq!(
            e.get(cell, TESObjectCELL::pLightingTemplate).addr(),
            0x0100_0042
        );
    }

    #[test]
    fn load_hands_names_bounds_and_textures_to_their_owners() {
        let chunks = vec![
            (EDITOR_ID, b"CellEdid\0".to_vec()),
            (OBJECT_BOUNDS, vec![0; 4]),
            (FULL_NAME, vec![]),
            (WATER_NOISE_TEXTURE, b"water.dds\0".to_vec()),
            (WATER_NOISE_TEXTURE, vec![]),
        ];
        let (mut e, _) = load_engine(0x39, &chunks, false);
        // The editor id is passed to virtual slot 0x134 as a C string.
        let seen = Rc::new(RefCell::new(vec![]));
        let strings = seen.clone();
        e.register_double(fake(0x134), move |e, a| {
            strings.borrow_mut().push(e.mem.cstr(a[1]));
            Ret::default()
        });
        let names = Rc::new(RefCell::new(vec![]));
        let texture_names = names.clone();
        e.register_double(FULL_NAME_SET, move |e, a| {
            texture_names.borrow_mut().push((a[0], e.mem.cstr(a[1])));
            Ret::default()
        });
        let file = new_file(&mut e, false);
        let cell = loading_cell(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0054_22d0, &args![cell, file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(*seen.borrow(), vec![b"CellEdid".to_vec()]);
        assert_eq!(
            calls_to(&log, fake(0xe0)),
            vec![vec![cell.addr(), file.addr()]]
        );
        assert_eq!(
            calls_to(&log, FULL_NAME_LOAD),
            vec![vec![cell.addr() + 0x18, file.addr()]]
        );
        // The empty texture chunk is ignored.
        assert_eq!(
            *names.borrow(),
            vec![(cell.addr() + 0x58, b"water.dds".to_vec())]
        );
    }

    #[test]
    fn load_creates_the_impact_swap_data_once_and_forwards_extra_data() {
        let chunks = vec![
            (IMPACT_SWAP_FIRST, vec![1]),
            (tag(b"XOWN"), vec![2]),
            (tag(b"ZZZZ"), vec![3]),
            (IMPACT_SWAP_SECOND, vec![4]),
        ];
        let (mut e, _) = load_engine(0x39, &chunks, false);
        // The first lookup finds nothing, later ones find the stored object.
        let stored = Rc::new(RefCell::new(0u32));
        let slot = stored.clone();
        e.register_double(EXTRA_LIST_GET_IMPACT_SWAP, move |_, _| {
            (*slot.borrow()).into_ret()
        });
        let slot = stored.clone();
        e.register_double(EXTRA_LIST_SET_IMPACT_SWAP, move |_, a| {
            *slot.borrow_mut() = a[1];
            Ret::default()
        });
        let file = new_file(&mut e, false);
        let cell = loading_cell(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0054_22d0, &args![cell, file]);
        let log = e.call_log.take().unwrap();
        // One allocation of 0x15c bytes, one construction, two loads.
        let allocated = calls_to(&log, ALLOCATE);
        assert_eq!(allocated.len(), 1);
        assert_eq!(allocated[0], vec![0x15c]);
        assert_eq!(calls_to(&log, IMPACT_SWAP_CONSTRUCT).len(), 1);
        let object = *stored.borrow();
        assert_ne!(object, 0);
        assert_eq!(
            calls_to(&log, IMPACT_SWAP_LOAD),
            vec![vec![object, file.addr()]; 2]
        );
        // The extra-data chunk goes to the cell's extra data list.
        assert_eq!(
            calls_to(&log, EXTRA_LIST_LOAD),
            vec![vec![cell.addr() + 0x28, file.addr(), cell.addr()]]
        );
    }

    #[test]
    fn load_knows_76_extra_data_tags() {
        assert_eq!(EXTRA_DATA_TAGS.len(), 76);
        let mut sorted = EXTRA_DATA_TAGS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 76, "no tag is listed twice");
        // None of the tags the other arms handle is among them.
        for own in [
            DATA,
            EXTERIOR_COORDINATES,
            INTERIOR_LIGHTING,
            WATER_HEIGHT,
            LIGHTING_TEMPLATE_ID,
            INHERITANCE_FLAGS,
            WATER_NOISE_TEXTURE,
            FULL_NAME,
            EDITOR_ID,
            OBJECT_BOUNDS,
            IMPACT_SWAP_FIRST,
            IMPACT_SWAP_SECOND,
        ] {
            assert!(!EXTRA_DATA_TAGS.contains(&own));
        }
        assert_eq!(tag(b"DATA"), 0x4154_4144);
    }

    // ---- InitItem --------------------------------------------------

    /// Words the `InitItem` doubles keep: the thread's error count, the
    /// number of errors an extra data `InitItem` raises, the lookup result and
    /// the cast result for the lighting template.
    const ERRORS: u32 = 0x011c_c700;
    const ERRORS_RAISED: u32 = 0x011c_c704;
    const LOOKED_UP: u32 = 0x011c_c708;
    const TEMPLATE_CAST: u32 = 0x011c_c70c;

    fn init_engine() -> Engine {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                INC_DISABLE_WARNING_COUNT,
                DEBUG_PRINT,
                GAME_LOAD_FORM,
                GAME_LOAD_NOTIFY,
                EXTRA_LIST_SET_NORTH_ROTATION,
                GAME_FINISH_A,
                GAME_FINISH_B,
                GAME_FINISH_C,
                GAME_FLAG_WRITE,
                NAV_POINTER_RELEASE,
                FORM_ADD_COMPILE_INDEX,
            ],
        );
        e.register(FULL_NAME_SET, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(FORM_SET_FLAG_8, |e, a| {
            let flags = e.mem.u32(a[0] + 8);
            e.mem
                .set_u32(a[0] + 8, if a[1] != 0 { flags | 8 } else { flags & !8 });
            Ret::default()
        });
        e.register(TEXT_GET_STRING, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ERROR_COUNT_GET, |e, _| e.mem.u32(ERRORS).into_ret());
        e.register(ERROR_COUNT_SET, |e, a| {
            e.mem.set_u32(ERRORS, a[0]);
            Ret::default()
        });
        // The extra data's `InitItem` is where errors are raised.
        e.register(EXTRA_LIST_INIT_ITEM, |e, _| {
            let raised = e.mem.u32(ERRORS_RAISED);
            e.mem.set_u32(ERRORS, raised);
            Ret::default()
        });
        e.register(FORM_LOOK_UP, |e, _| e.mem.u32(LOOKED_UP).into_ret());
        e.register(RT_DYNAMIC_CAST, |e, a| {
            if a[3] == RTTI_LIGHTING_TEMPLATE {
                e.mem.u32(TEMPLATE_CAST).into_ret()
            } else {
                a[0].into_ret()
            }
        });
        returns(&mut e, GAME_LOADER_FLAG_SETTER, 1);
        returns(&mut e, GAME_FLAG_READ, 0);
        returns(&mut e, GAME_FLAG_TEST, 0);
        // The loader and save-game singletons.
        let loader = e.mem.alloc(0x40);
        let save_game = e.mem.alloc(0x40);
        e.set_global(GAME_LOADER_POINTER, loader);
        e.set_global(SAVE_GAME_POINTER, save_game);
        e.set_global(PLAYER_HELPER_FORM, 0x1111u32);
        e.set_global(MINUS_ONE, -1.0f64);
        // Nav meshes: the array's count is its word at +8; element `i` is
        // the object at `array + 0x10 + 4 * i`.
        e.register(CELL_NAV_MESHES, |e, a| e.mem.u32(a[0] + 0x64).into_ret());
        e.register(NAV_MESH_ARRAY_COUNT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(NAV_MESH_ARRAY_GET, |e, a| {
            let mesh = e.mem.u32(a[0] + 0x10 + 4 * a[2]);
            e.mem.set_u32(a[1], mesh);
            a[1].into_ret()
        });
        e.register(NAV_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(REFERENCE_PARENT_CELL, |e, a| {
            e.mem.u32(a[0] + 0x78).into_ret()
        });
        e.register(SAVE_FORM_BUFFER_GET_FORM, |e, a| {
            e.mem.u32(a[0] + 0x70).into_ret()
        });
        e.register(REFERENCE_ROTATION, |_, a| (a[0] + 0x20).into_ret());
        e
    }

    /// A cell for `InitItem`: form id, editor id string, full name text.
    fn init_cell(e: &mut Engine, interior: bool, full_name: &str) -> Ptr<TESObjectCELL> {
        let cell = cell(e, interior);
        e.mem.set_u32(cell.addr() + 0xc, 0x1234);
        let editor_id = text(e, "CellEditorId");
        e.mem
            .set_u32(cell.addr() + EDITOR_ID_WORD, editor_id.addr());
        let name = text(e, full_name);
        e.mem.set_u32(cell.addr() + 0x1c, name.addr());
        cell
    }

    fn debug_prints(log: &[(u32, Vec<u32>)]) -> Vec<Vec<u32>> {
        calls_to(log, DEBUG_PRINT)
    }

    #[test]
    fn init_item_of_an_initialized_cell_only_walks_the_children() {
        let mut e = init_engine();
        let cell = init_cell(&mut e, false, "Name");
        e.mem.set_u32(cell.addr() + 8, 0x8);
        e.call_log = Some(vec![]);
        e.call(0x0054_2d40, &args![cell]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            sequence(&log),
            vec![
                FORM_FLAG_8,
                GAME_LOADER_FLAG_SETTER,
                CELL_REFERENCE_LIST,
                LIST_IS_END,
                CELL_NAV_MESHES,
                GAME_LOADER_FLAG_SETTER,
                GAME_FLAG_READ,
            ]
        );
        let loader = e.global::<u32>(GAME_LOADER_POINTER);
        assert_eq!(
            calls_to(&log, GAME_LOADER_FLAG_SETTER),
            vec![vec![loader, 0], vec![loader, 1]]
        );
    }

    #[test]
    fn init_item_of_an_interior_warns_about_a_long_name_and_errors() {
        let mut e = init_engine();
        let long_name = "N".repeat(40);
        let cell = init_cell(&mut e, true, &long_name);
        e.mem.set_u32(ERRORS, 5);
        e.mem.set_u32(ERRORS_RAISED, 2);
        // A template id that resolves to an object.
        e.mem.set_u32(cell.addr() + 0xd8, 0x0100_0007);
        let template = object(&mut e);
        e.mem.set_u32(LOOKED_UP, template.addr());
        e.mem.set_u32(TEMPLATE_CAST, template.addr());
        e.call_log = Some(vec![]);
        e.call(0x0054_2d40, &args![cell]);
        let log = e.call_log.take().unwrap();
        let name = e.mem.u32(cell.addr() + 0x1c);
        let editor_id = e.mem.u32(cell.addr() + EDITOR_ID_WORD);
        assert_eq!(
            debug_prints(&log),
            vec![
                vec![CELL_NAME_TOO_LONG, name, editor_id, 0x1234],
                vec![INIT_ERRORS_INTERIOR, editor_id, 0x1234],
            ]
        );
        // The error count was saved, cleared, read back and restored; the
        // warnings were muted in between.
        assert_eq!(calls_to(&log, ERROR_COUNT_SET), vec![vec![0], vec![5]]);
        assert_eq!(
            calls_to(&log, INC_DISABLE_WARNING_COUNT),
            vec![vec![1], vec![0]]
        );
        // An interior keeps its name; flag 8 is set; the template resolved.
        assert!(calls_to(&log, FULL_NAME_SET).is_empty());
        assert_ne!(e.mem.u32(cell.addr() + 8) & 8, 0);
        assert_eq!(e.get(cell, TESObjectCELL::pLightingTemplate), template);
        assert_eq!(
            calls_to(&log, EXTRA_LIST_INIT_ITEM),
            vec![vec![cell.addr() + 0x28, cell.addr()]]
        );
    }

    #[test]
    fn init_item_of_an_exterior_names_its_world_and_missing_template() {
        let mut e = init_engine();
        let cell = init_cell(&mut e, false, "Goodsprings");
        let data = e.mem.alloc(12);
        e.mem.set_i32(data, 3);
        e.mem.set_i32(data + 4, -4);
        e.mem.set_u32(cell.addr() + 0x48, data);
        let world = object(&mut e);
        e.mem.set_u32(world.addr() + 0xc, 0x55);
        let world_editor_id = text(&mut e, "WastelandNV");
        e.mem
            .set_u32(world.addr() + EDITOR_ID_WORD, world_editor_id.addr());
        e.mem.set_u32(cell.addr() + 0xc0, world.addr());
        e.mem.set_u32(ERRORS_RAISED, 1);
        e.mem.set_u32(cell.addr() + 0xd8, 0x0100_0007);
        // The id does not resolve to a template.
        e.mem.set_u32(LOOKED_UP, 0x4000);
        e.mem.set_u32(TEMPLATE_CAST, 0);
        e.call_log = Some(vec![]);
        e.call(0x0054_2d40, &args![cell]);
        let log = e.call_log.take().unwrap();
        let editor_id = e.mem.u32(cell.addr() + EDITOR_ID_WORD);
        assert_eq!(
            debug_prints(&log),
            vec![
                vec![
                    INIT_ERRORS_EXTERIOR_WORLD,
                    editor_id,
                    0x1234,
                    3,
                    (-4i32) as u32,
                    world_editor_id.addr(),
                    0x55,
                ],
                vec![LIGHTING_TEMPLATE_MISSING, 0x0100_0007, editor_id, 0x1234],
            ]
        );
        // An exterior cell loses its full name.
        assert_eq!(
            calls_to(&log, FULL_NAME_SET),
            vec![vec![cell.addr() + 0x18, 0]]
        );
        assert!(e.get(cell, TESObjectCELL::pLightingTemplate).is_null());
        // The template id was made file-relative first.
        assert_eq!(calls_to(&log, FORM_ADD_COMPILE_INDEX).len(), 1);
    }

    #[test]
    fn init_item_of_an_exterior_without_a_world_space() {
        let mut e = init_engine();
        let cell = init_cell(&mut e, false, "Lost");
        let data = e.mem.alloc(12);
        e.mem.set_i32(data, 7);
        e.mem.set_i32(data + 4, 9);
        e.mem.set_u32(cell.addr() + 0x48, data);
        e.mem.set_u32(ERRORS_RAISED, 1);
        e.call_log = Some(vec![]);
        e.call(0x0054_2d40, &args![cell]);
        let log = e.call_log.take().unwrap();
        let editor_id = e.mem.u32(cell.addr() + EDITOR_ID_WORD);
        assert_eq!(
            debug_prints(&log),
            vec![vec![
                INIT_ERRORS_EXTERIOR_UNKNOWN_WORLD,
                editor_id,
                0x1234,
                7,
                9
            ]]
        );
    }

    #[test]
    fn init_item_without_errors_prints_nothing() {
        let mut e = init_engine();
        let cell = init_cell(&mut e, true, "Short");
        e.call_log = Some(vec![]);
        e.call(0x0054_2d40, &args![cell]);
        let log = e.call_log.take().unwrap();
        assert!(debug_prints(&log).is_empty());
        // A name of exactly 33 characters is still fine.
        let mut e = init_engine();
        let cell = init_cell(&mut e, true, &"N".repeat(33));
        e.call_log = Some(vec![]);
        e.call(0x0054_2d40, &args![cell]);
        assert!(debug_prints(&e.call_log.take().unwrap()).is_empty());
    }

    /// A reference for the `InitItem` walk: form flags, parent cell (`+0x78`),
    /// the form `GetForm` reports (`+0x70`) and a rotation float at `+0x28`.
    fn walked_reference(e: &mut Engine, flags: u32, parent: Ptr, form: u32, rotation: f32) -> Ptr {
        let reference = object(e);
        e.mem.set_u32(reference.addr() + 8, flags);
        e.mem.set_u32(reference.addr() + 0x78, parent.addr());
        e.mem.set_u32(reference.addr() + 0x70, form);
        e.mem.set_f32(reference.addr() + 0x28, rotation);
        reference
    }

    #[test]
    fn init_item_initializes_every_reference_and_follows_the_ones_that_move() {
        let mut e = init_engine();
        let cell = init_cell(&mut e, false, "Name");
        e.mem.set_u32(cell.addr() + 8, 0x8);
        let elsewhere = cell_with(&mut e, false, &[]);
        let r1 = walked_reference(&mut e, 0, cell.cast(), 0, 0.0);
        // Already initialized: only the move check applies.
        let r2 = walked_reference(&mut e, 8, elsewhere.cast(), 0, 0.0);
        let r3 = walked_reference(&mut e, 0, cell.cast(), 0, 0.0);
        set_references(&mut e, cell, &[r1, r2, r3]);
        // r2 leaves the cell: its node is unlinked, as the game does.
        let node1 = cell.addr() + 0xac;
        let node2 = e.mem.u32(node1 + 4);
        let node3 = e.mem.u32(node2 + 4);
        e.mem.set_u32(SCRATCH, node1);
        e.mem.set_u32(SCRATCH + 4, node3);
        e.register_double(REFERENCE_PARENT_CELL, move |e, a| {
            let parent = e.mem.u32(a[0] + 0x78);
            if parent != e.mem.u32(SCRATCH + 8) {
                // Seen for the first time: the reference is gone.
                let (first, third) = (e.mem.u32(SCRATCH), e.mem.u32(SCRATCH + 4));
                e.mem.set_u32(first + 4, third);
            }
            parent.into_ret()
        });
        e.mem.set_u32(SCRATCH + 8, cell.addr());
        e.call_log = Some(vec![]);
        e.call(0x0054_2d40, &args![cell]);
        let log = e.call_log.take().unwrap();
        let loader = e.global::<u32>(GAME_LOADER_POINTER);
        let save_game = e.global::<u32>(SAVE_GAME_POINTER);
        // r1 and r3 go through the form InitItem (slot 0x88), the save-game
        // notification and the loader; r2 does not.
        assert_eq!(
            calls_to(&log, fake(0x88)),
            vec![vec![r1.addr()], vec![r3.addr()]]
        );
        assert_eq!(
            calls_to(&log, GAME_LOAD_NOTIFY),
            vec![vec![save_game, r1.addr()], vec![save_game, r3.addr()]]
        );
        assert_eq!(
            calls_to(&log, GAME_LOAD_FORM),
            vec![vec![loader, r1.addr()], vec![loader, r3.addr()]]
        );
        // After r2 moved, the walk went on with r3 (the node after r1).
        assert_eq!(calls_to(&log, REFERENCE_PARENT_CELL).len(), 3);
    }

    #[test]
    fn init_item_derives_the_north_rotation_from_the_player_helper() {
        let mut e = init_engine();
        let cell = init_cell(&mut e, true, "Name");
        e.mem.set_u32(cell.addr() + 8, 0x8);
        let helper = walked_reference(&mut e, 8, cell.cast(), 0x1111, 1.5);
        let other = walked_reference(&mut e, 8, cell.cast(), 0x2222, 9.0);
        set_references(&mut e, cell, &[other, helper]);
        e.call_log = Some(vec![]);
        e.call(0x0054_2d40, &args![cell]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, EXTRA_LIST_SET_NORTH_ROTATION),
            vec![vec![cell.addr() + 0x28, (-1.5f32).to_bits()]]
        );
        // An exterior cell never does that.
        let mut e = init_engine();
        let cell = init_cell(&mut e, false, "Name");
        e.mem.set_u32(cell.addr() + 8, 0x8);
        let helper = walked_reference(&mut e, 8, cell.cast(), 0x1111, 1.5);
        set_references(&mut e, cell, &[helper]);
        e.call_log = Some(vec![]);
        e.call(0x0054_2d40, &args![cell]);
        assert!(calls_to(&e.call_log.take().unwrap(), EXTRA_LIST_SET_NORTH_ROTATION).is_empty());
    }

    #[test]
    fn init_item_initializes_the_nav_meshes() {
        let mut e = init_engine();
        let cell = init_cell(&mut e, false, "Name");
        e.mem.set_u32(cell.addr() + 8, 0x8);
        let array = e.mem.alloc(0x20);
        e.mem.set_u32(array + 8, 2);
        let mesh_a = object(&mut e);
        let mesh_b = object(&mut e);
        e.mem.set_u32(array + 0x10, mesh_a.addr());
        e.mem.set_u32(array + 0x14, mesh_b.addr());
        e.set(cell, TESObjectCELL::pNavMeshes, Ptr::new(array));
        e.call_log = Some(vec![]);
        e.call(0x0054_2d40, &args![cell]);
        let log = e.call_log.take().unwrap();
        let loader = e.global::<u32>(GAME_LOADER_POINTER);
        assert_eq!(
            calls_to(&log, fake(0x88)),
            vec![vec![mesh_a.addr()], vec![mesh_b.addr()]]
        );
        assert_eq!(
            calls_to(&log, GAME_LOAD_FORM),
            vec![vec![loader, mesh_a.addr()], vec![loader, mesh_b.addr()]]
        );
        assert_eq!(calls_to(&log, NAV_POINTER_RELEASE).len(), 2);
    }

    #[test]
    fn init_item_finishes_a_pending_save_game_load() {
        let mut e = init_engine();
        let cell = init_cell(&mut e, false, "Name");
        e.mem.set_u32(cell.addr() + 8, 0x8);
        // The flag reads 1 (pending), then 0 (cleared by the first step),
        // then 5 (the value to restore).
        let reads = Rc::new(RefCell::new(VecDeque::from([1u32, 0, 5])));
        let queue = reads.clone();
        e.register_double(GAME_FLAG_READ, move |_, _| {
            queue.borrow_mut().pop_front().unwrap_or(0).into_ret()
        });
        returns(&mut e, GAME_FLAG_TEST, 1);
        e.call_log = Some(vec![]);
        e.call(0x0054_2d40, &args![cell]);
        let log = e.call_log.take().unwrap();
        let save_game = e.global::<u32>(SAVE_GAME_POINTER);
        let tail: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .skip_while(|(a, _)| *a != GAME_FINISH_A)
            .collect();
        assert_eq!(
            tail,
            vec![
                (GAME_FINISH_A, vec![save_game, cell.addr()]),
                (GAME_FLAG_READ, vec![save_game]),
                (GAME_FLAG_TEST, vec![save_game]),
                (GAME_FLAG_READ, vec![save_game]),
                (GAME_FLAG_WRITE, vec![save_game, 1]),
                (GAME_FINISH_B, vec![save_game, 0, 0, 0]),
                (GAME_FINISH_C, vec![save_game, 0]),
                (GAME_FLAG_WRITE, vec![save_game, 5]),
            ]
        );
    }

    // ---- SavesBefore and group helpers -----------------------------

    /// A cell for the ordering code: exterior coordinates, form id, key.
    fn ordered_cell(
        e: &mut Engine,
        interior: bool,
        coordinates: (i32, i32),
        id: u32,
        world: Ptr,
    ) -> Ptr<TESObjectCELL> {
        let cell = cell(e, interior);
        e.mem.set_u8(cell.addr() + 4, 0x39);
        e.mem.set_u32(cell.addr() + 0xc, id);
        let data = e.mem.alloc(12);
        e.mem.set_i32(data, coordinates.0);
        e.mem.set_i32(data + 4, coordinates.1);
        e.mem.set_u32(cell.addr() + 0x48, data);
        e.mem.set_u32(cell.addr() + 0xc0, world.addr());
        cell
    }

    fn world_with_key(e: &mut Engine, key: u32) -> Ptr {
        let world = object(e);
        e.mem.set_u32(world.addr() + KEY, key);
        world
    }

    #[test]
    fn cells_save_in_a_fixed_order() {
        let mut e = engine();
        let world = world_with_key(&mut e, 1);
        let interior = ordered_cell(&mut e, true, (0, 0), 0x100, Ptr::NULL);
        let exterior = ordered_cell(&mut e, false, (0, 0), 0x100, world);
        // Interior cells come before exterior ones, never the reverse.
        assert!(e.call(0x0054_3230, &args![interior, exterior]).bool());
        assert!(!e.call(0x0054_3230, &args![exterior, interior]).bool());

        // Exterior cells of different world spaces: the world space decides.
        let later_world = world_with_key(&mut e, 2);
        let in_later = ordered_cell(&mut e, false, (0, 0), 0x100, later_world);
        assert!(e.call(0x0054_3230, &args![exterior, in_later]).bool());
        assert!(!e.call(0x0054_3230, &args![in_later, exterior]).bool());

        // Same world: a persistent cell comes first.
        let persistent = ordered_cell(&mut e, false, (99, 99), 0x200, world);
        e.mem.set_u32(persistent.addr() + 8, 0x400);
        assert!(e.call(0x0054_3230, &args![persistent, exterior]).bool());
        assert!(!e.call(0x0054_3230, &args![exterior, persistent]).bool());

        // Then by block key, sub-block key and form id.
        let near = ordered_cell(&mut e, false, (0, 0), 0x100, world);
        let other_block = ordered_cell(&mut e, false, (64, 0), 0x050, world);
        assert!(e.call(0x0054_3230, &args![near, other_block]).bool());
        assert!(!e.call(0x0054_3230, &args![other_block, near]).bool());
        let other_sub_block = ordered_cell(&mut e, false, (8, 0), 0x050, world);
        assert!(e.call(0x0054_3230, &args![near, other_sub_block]).bool());
        assert!(!e.call(0x0054_3230, &args![other_sub_block, near]).bool());
        let same_place = ordered_cell(&mut e, false, (1, 1), 0x300, world);
        assert!(e.call(0x0054_3230, &args![near, same_place]).bool());
        assert!(!e.call(0x0054_3230, &args![same_place, near]).bool());
        // The very same position and id: not before.
        let twin = ordered_cell(&mut e, false, (1, 1), 0x100, world);
        assert!(!e.call(0x0054_3230, &args![near, twin]).bool());

        // Two interiors: by the digits of the form id.
        let first = ordered_cell(&mut e, true, (0, 0), 0x1_0001, Ptr::NULL);
        let second = ordered_cell(&mut e, true, (0, 0), 0x1_0002, Ptr::NULL);
        assert!(e.call(0x0054_3230, &args![first, second]).bool());
        assert!(!e.call(0x0054_3230, &args![second, first]).bool());
    }

    #[test]
    fn a_form_that_does_not_cast_to_a_cell_is_not_after_it() {
        let mut e = engine();
        let cell = ordered_cell(&mut e, false, (0, 0), 1, Ptr::NULL);
        let other = ordered_cell(&mut e, false, (0, 0), 2, Ptr::NULL);
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(!e.call(0x0054_3230, &args![cell, other]).bool());
    }

    #[test]
    fn cells_order_against_forms_of_other_types() {
        let mut e = engine();
        let world = world_with_key(&mut e, 1);
        let cell = ordered_cell(&mut e, false, (0, 0), 1, world);
        // A reference-like form (type 0x3a): its owner cell (virtual slot 0)
        // is this cell, or compares by this cell's own SavesBefore.
        let owned = object(&mut e);
        e.mem.set_u8(owned.addr() + 4, 0x3a);
        e.mem.set_u32(owned.addr() + SLOT_ZERO_RESULT, cell.addr());
        assert!(e.call(0x0054_3230, &args![cell, owned]).bool());
        let owner = ordered_cell(&mut e, false, (0, 0), 1, world);
        e.mem.set_u32(owner.addr() + KEY, 5);
        e.mem.set_u32(cell.addr() + KEY, 2);
        e.mem.set_u32(owned.addr() + SLOT_ZERO_RESULT, owner.addr());
        assert!(e.call(0x0054_3230, &args![cell, owned]).bool());
        e.mem.set_u32(cell.addr() + KEY, 9);
        assert!(!e.call(0x0054_3230, &args![cell, owned]).bool());
        for form_type in [0x3b, 0x3f, 0x40, 0x42, 0x43, 0x69] {
            e.mem.set_u8(owned.addr() + 4, form_type);
            e.mem.set_u32(owned.addr() + SLOT_ZERO_RESULT, cell.addr());
            assert!(
                e.call(0x0054_3230, &args![cell, owned]).bool(),
                "{form_type:#x}"
            );
        }

        // Type 0x41: always true for an interior cell, otherwise the world
        // space decides.
        let marker = object(&mut e);
        e.mem.set_u8(marker.addr() + 4, 0x41);
        e.mem.set_u32(marker.addr() + KEY, 3);
        let interior = ordered_cell(&mut e, true, (0, 0), 1, Ptr::NULL);
        assert!(e.call(0x0054_3230, &args![interior, marker]).bool());
        assert!(e.call(0x0054_3230, &args![cell, marker]).bool());
        let late_world = world_with_key(&mut e, 4);
        let late_cell = ordered_cell(&mut e, false, (0, 0), 1, late_world);
        assert!(!e.call(0x0054_3230, &args![late_cell, marker]).bool());

        // Any other type goes to the fallback.
        let other = object(&mut e);
        e.mem.set_u8(other.addr() + 4, 0x10);
        returns(&mut e, FORM_COMPARE_FALLBACK, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_3230, &args![cell, other]).bool());
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), FORM_COMPARE_FALLBACK),
            vec![vec![cell.addr(), other.addr()]]
        );
    }

    /// A group header for the cell's group helpers.
    fn header(e: &mut Engine, group_type: u32, label: u32) -> Ptr {
        let header = e.mem.alloc(0x14);
        e.mem.set_u32(header, 0x5055_5247);
        e.mem.set_u32(header + 8, label);
        e.mem.set_u32(header + 0xc, group_type);
        Ptr::new(header)
    }

    /// `SavesBefore(header)` of `cell` for a group of the given type and label.
    fn before(e: &mut Engine, cell: Ptr<TESObjectCELL>, group_type: u32, label: u32) -> bool {
        let header = header(e, group_type, label);
        e.call(0x0054_34d0, &args![cell, header]).bool()
    }

    /// Makes `form_lookup(label)` answer `form`.
    fn place(e: &mut Engine, label: u32, form: Ptr) {
        e.mem.set_u32(LOOKUP_TABLE + 4 * (label & 0xf), form.addr());
    }

    const LOOKUP_TABLE: u32 = 0x011c_c800;

    #[test]
    fn cells_order_against_group_headers() {
        let mut e = engine();
        let world = world_with_key(&mut e, 1);
        e.register(FORM_LOOK_UP, |e, a| {
            e.mem.u32(LOOKUP_TABLE + 4 * (a[0] & 0xf)).into_ret()
        });
        // Form id 0x105: block key 1, sub-block key 6 as an interior.
        let exterior = ordered_cell(&mut e, false, (0, 0), 0x105, world);
        let interior = ordered_cell(&mut e, true, (0, 0), 0x105, Ptr::NULL);
        // Not a group header.
        assert!(!e
            .call(0x0054_34d0, &args![exterior, Ptr::<()>::NULL])
            .bool());
        let bad = header(&mut e, 7, 0);
        e.mem.set_u32(bad.addr(), 0x1234);
        assert!(!e.call(0x0054_34d0, &args![exterior, bad]).bool());

        // Type 0 goes to the generic code, type 7 is always true, an
        // unknown type is false.
        returns(&mut e, FORM_BELONGS_IN_GROUP_FALLBACK, 1);
        assert!(before(&mut e, exterior, 0, 0));
        assert!(before(&mut e, exterior, 7, 0));
        assert!(!before(&mut e, exterior, 10, 0));

        // Type 1 (world children): an interior is before; an exterior cell
        // by the world space the label names.
        let later_world = world_with_key(&mut e, 5);
        place(&mut e, 3, later_world);
        assert!(before(&mut e, interior, 1, 3));
        assert!(before(&mut e, exterior, 1, 3));
        let earlier_world = world_with_key(&mut e, 0);
        place(&mut e, 3, earlier_world);
        assert!(!before(&mut e, exterior, 1, 3));

        // Types 2 and 3: interior only, key lower than the label.
        assert!(!before(&mut e, exterior, 2, 5));
        assert!(before(&mut e, interior, 2, 2));
        assert!(!before(&mut e, interior, 2, 1));
        assert!(before(&mut e, interior, 3, 7));
        assert!(!before(&mut e, interior, 3, 6));
        // Types 4 and 5: an interior and a persistent cell are before;
        // otherwise the exterior key decides (block key 2<<16, sub-block
        // key 8<<16 for x = 64).
        assert!(before(&mut e, interior, 4, 0));
        assert!(before(&mut e, interior, 5, 0));
        let far = ordered_cell(&mut e, false, (64, 0), 0x100, world);
        assert!(!before(&mut e, far, 4, 0x1_0000));
        assert!(before(&mut e, far, 4, 0x3_0000));
        assert!(!before(&mut e, far, 5, 0x8_0000));
        assert!(before(&mut e, far, 5, 0x9_0000));
        e.mem.set_u32(far.addr() + 8, 0x400);
        assert!(before(&mut e, far, 4, 0));
        assert!(before(&mut e, far, 5, 0));

        // Types 6, 8 and 9: before the cell the label names.
        let big = ordered_cell(&mut e, false, (0, 0), 0x105, world);
        e.mem.set_u32(big.addr() + KEY, 9);
        e.mem.set_u32(exterior.addr() + KEY, 1);
        place(&mut e, 4, big.cast());
        for kind in [6, 8, 9] {
            assert!(before(&mut e, exterior, kind, 4));
            assert!(!before(&mut e, big, kind, 4));
        }
        // A label that names no cell: not before.
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(!before(&mut e, exterior, 6, 4));
        assert!(!before(&mut e, exterior, 1, 3));
    }

    // ---- duplicating, copying, comparing ----------------------------

    #[test]
    fn duplicating_an_interior_gives_nothing() {
        let mut e = engine();
        let cell = cell(&mut e, true);
        e.call_log = Some(vec![]);
        let copy = e.call(0x0054_36e0, &args![cell, 0u32, 7u32]).ptr::<()>();
        assert!(copy.is_null());
        assert_eq!(
            sequence(&e.call_log.take().unwrap()),
            vec![CELL_IS_INTERIOR]
        );
    }

    /// Doubles for `CreateDuplicateForm`: the duplicate of every object is
    /// the object's `DUPLICATE` word, and the nav mesh and list helpers keep
    /// their state in memory.
    fn duplicate_engine() -> Engine {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                LOCK_ENTER,
                LOCK_LEAVE,
                CELL_SET_LAND,
                LAND_SET_CELL,
                CELL_ADD_REFERENCE,
                REFERENCE_SET_PERSISTS,
                NAV_POINTER_CONSTRUCT,
                NAV_POINTER_RELEASE,
                NAV_MESH_ARRAY_ADD,
                LIST_CLEAR,
                LIST_DESTRUCT,
            ],
        );
        e.register(FORM_DUPLICATE, |e, a| {
            e.mem.u32(a[0] + DUPLICATE).into_ret()
        });
        e.register(NAV_MESH_ARRAY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(CELL_SET_NAV_MESHES, |e, a| {
            e.mem.set_u32(a[0] + 0x64, a[1]);
            Ret::default()
        });
        e.register(NAV_MESH_ARRAY_COUNT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(NAV_MESH_ARRAY_GET, |e, a| {
            let mesh = e.mem.u32(a[0] + 0x10 + 4 * a[2]);
            e.mem.set_u32(a[1], mesh);
            a[1].into_ret()
        });
        e.register(NAV_POINTER_FROM_RAW, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(NAV_POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            a[0].into_ret()
        });
        e.register(NAV_POINTER_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            a[0].into_ret()
        });
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.write(a[0], &[0; 8]);
            a[0].into_ret()
        });
        // The list push appends: it fills the head first, then new nodes.
        e.register(LIST_PUSH_FRONT, |e, a| {
            let value = e.mem.u32(a[1]);
            let mut node = a[0];
            if e.mem.u32(node) != 0 {
                while e.mem.u32(node + 4) != 0 {
                    node = e.mem.u32(node + 4);
                }
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
            e.mem.set_u32(node, value);
            Ret::default()
        });
        e
    }

    /// A duplicate for `original` (an object whose `DUPLICATE` word points at
    /// the new object).
    fn with_duplicate(e: &mut Engine, original: Ptr) -> Ptr {
        let copy = object(e);
        e.mem.set_u32(original.addr() + DUPLICATE, copy.addr());
        copy
    }

    #[test]
    fn duplicating_an_exterior_cell_duplicates_land_nav_meshes_and_references() {
        let mut e = duplicate_engine();
        let cell = cell(&mut e, false);
        let new_cell = with_duplicate(&mut e, cell.cast());
        // The land: its duplicate gets the new cell and the cell gets it.
        let land = object(&mut e);
        e.set(cell, TESObjectCELL::pCellLand, land);
        let new_land = with_duplicate(&mut e, land);
        // Two nav meshes, one of them deleted.
        let array = e.mem.alloc(0x20);
        e.mem.set_u32(array + 8, 2);
        let mesh = object(&mut e);
        let deleted_mesh = object(&mut e);
        e.mem.set_u32(deleted_mesh.addr() + 8, 0x20);
        e.mem.set_u32(array + 0x10, mesh.addr());
        e.mem.set_u32(array + 0x14, deleted_mesh.addr());
        e.set(cell, TESObjectCELL::pNavMeshes, Ptr::new(array));
        let new_mesh = with_duplicate(&mut e, mesh);
        let new_array = e.mem.alloc(0x10);
        // The new array comes from the allocator (0x10 bytes).
        e.register_double(ALLOCATE, move |_, _| new_array.into_ret());
        // References: kept ones are duplicated; 0x4000 and persistent ones
        // are not (the cell is not persistent).
        let kept = reference(&mut e, 0, Ptr::NULL, Ptr::NULL, 0);
        let new_kept = with_duplicate(&mut e, kept);
        let flagged = reference(&mut e, 0x4000, Ptr::NULL, Ptr::NULL, 0);
        let persistent = reference(&mut e, 0x800, Ptr::NULL, Ptr::NULL, 0);
        let kept2 = reference(&mut e, 0, Ptr::NULL, Ptr::NULL, 0);
        let new_kept2 = with_duplicate(&mut e, kept2);
        set_references(&mut e, cell, &[kept, flagged, persistent, kept2]);
        // The duplicated cell is persistent: that flag is carried over.
        e.mem.set_u32(new_cell.addr() + 8, 0x400);
        e.call_log = Some(vec![]);
        let result = e.call(0x0054_36e0, &args![cell, 0u32, 7u32]).ptr::<()>();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, new_cell);

        // The base duplication, then the land.
        assert_eq!(
            calls_to(&log, FORM_DUPLICATE),
            vec![vec![cell.addr(), 0, 7]]
        );
        assert_eq!(calls_to(&log, fake(0x40))[0], vec![land.addr(), 0, 7]);
        assert_eq!(
            calls_to(&log, CELL_SET_LAND),
            vec![vec![new_cell.addr(), new_land.addr()]]
        );
        assert_eq!(
            calls_to(&log, LAND_SET_CELL),
            vec![vec![new_land.addr(), new_cell.addr()]]
        );
        assert_eq!(calls_to(&log, fake(0xc8))[0], vec![new_land.addr(), 1]);
        // The nav mesh array is a new 0x10-byte object set on the new cell;
        // only the live mesh is duplicated, pointed at the new cell, and added.
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x10]]);
        assert_eq!(e.mem.u32(new_cell.addr() + 0x64), new_array);
        assert_eq!(e.mem.u32(new_mesh.addr() + 0x24), new_cell.addr());
        assert_eq!(
            calls_to(&log, NAV_MESH_ARRAY_ADD),
            vec![vec![new_array, new_mesh.addr()]]
        );
        // References: the lock is held while collecting, the kept ones are
        // duplicated, made persistent like the new cell, added and marked.
        assert_eq!(calls_to(&log, LOCK_ENTER).len(), 1);
        assert_eq!(
            calls_to(&log, fake(0x40))[2..],
            [vec![kept.addr(), 0, 7], vec![kept2.addr(), 0, 7]]
        );
        assert_eq!(
            calls_to(&log, REFERENCE_SET_PERSISTS),
            vec![vec![new_kept.addr(), 1], vec![new_kept2.addr(), 1]]
        );
        assert_eq!(
            calls_to(&log, CELL_ADD_REFERENCE),
            vec![
                vec![new_cell.addr(), new_kept.addr(), 0],
                vec![new_cell.addr(), new_kept2.addr(), 0]
            ]
        );
        assert_eq!(
            calls_to(&log, fake(0xc8))[1..],
            [vec![new_kept.addr(), 1], vec![new_kept2.addr(), 1]]
        );
        assert_eq!(calls_to(&log, LIST_CLEAR).len(), 1);
        assert_eq!(calls_to(&log, LIST_DESTRUCT).len(), 1);
    }

    #[test]
    fn duplicating_a_persistent_exterior_cell_keeps_persistent_references() {
        let mut e = duplicate_engine();
        let cell = cell(&mut e, false);
        e.mem.set_u32(cell.addr() + 8, 0x400);
        let new_cell = with_duplicate(&mut e, cell.cast());
        let persistent = reference(&mut e, 0x800, Ptr::NULL, Ptr::NULL, 0);
        let new_reference = with_duplicate(&mut e, persistent);
        set_references(&mut e, cell, &[persistent]);
        e.call_log = Some(vec![]);
        e.call(0x0054_36e0, &args![cell, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CELL_ADD_REFERENCE),
            vec![vec![new_cell.addr(), new_reference.addr(), 0]]
        );
        // No land and no nav meshes: neither is duplicated.
        assert!(calls_to(&log, CELL_SET_LAND).is_empty());
        assert!(calls_to(&log, ALLOCATE).is_empty());
    }

    /// Cells set up for `Copy` and `Compare`.
    fn copy_pair(interior: bool) -> (Engine, Ptr<TESObjectCELL>, Ptr<TESObjectCELL>) {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                FORM_COPY_ALL_COMPONENTS,
                EXTRA_LIST_COPY_LIST,
                CELL_CREATE_CELL_DATA,
            ],
        );
        let size = if interior { 0x2c } else { 0xc };
        let source = cell(&mut e, interior);
        let this = cell(&mut e, interior);
        for (c, fill) in [(source, 0xa5u8), (this, 0x11)] {
            let data = e.mem.alloc(size);
            e.mem.write(data, &vec![fill; size as usize]);
            e.mem.set_u32(c.addr() + 0x48, data);
        }
        e.mem.set_u8(source.addr() + 0x24, 0x80 | interior as u8);
        e.mem.set_f32(source.addr() + 0x50, 42.5);
        e.mem.set_u32(source.addr() + 0xd8, 0x7000);
        e.mem.set_u32(source.addr() + 0xdc, 0x33);
        // The water noise texture component's vtable (slot 2 copies, slot 3
        // compares), one per cell.
        for (c, table) in [(source, 0x0132_0000u32), (this, 0x0132_0100)] {
            e.put_vtable(table, &[0, 0, fake(0x1002), fake(0x1003)]);
            e.mem.set_u32(c.addr() + 0x58, table);
        }
        quiet(&mut e, &[fake(0x1002)]);
        e.register(fake(0x1003), |e, _| e.mem.u32(SCRATCH + 0x40).into_ret());
        (e, this, source)
    }

    #[test]
    fn copying_a_cell_copies_flags_data_and_lighting() {
        for interior in [false, true] {
            let (mut e, this, source) = copy_pair(interior);
            let old_data = e.mem.u32(this.addr() + 0x48);
            e.call_log = Some(vec![]);
            e.call(0x0054_3ab0, &args![this, source]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, FORM_COPY_ALL_COMPONENTS),
                vec![vec![this.addr(), source.addr()]]
            );
            assert_eq!(
                calls_to(&log, EXTRA_LIST_COPY_LIST),
                vec![vec![this.addr() + 0x28, source.addr() + 0x28]]
            );
            // The old data block was freed; the data is created again (the
            // double does not allocate) and the contents copied across.
            assert_eq!(e.mem.block_size(old_data), None);
            assert_eq!(
                e.get(this, TESObjectCELL::cCellFlags),
                0x80 | interior as u8
            );
            assert_eq!(e.get(this, TESObjectCELL::fWaterHeight), 42.5);
            assert_eq!(e.get(this, TESObjectCELL::pLightingTemplate).addr(), 0x7000);
            assert_eq!(
                e.get(this, TESObjectCELL::iLightingTemplateInheritanceFlags),
                0x33
            );
            // The texture component is copied by its virtual slot 2.
            assert_eq!(
                calls_to(&log, fake(0x1002)),
                vec![vec![this.addr() + 0x58, source.addr() + 0x58]]
            );
        }
    }

    #[test]
    fn copying_the_data_block_needs_both_blocks() {
        let (mut e, this, source) = copy_pair(true);
        // `CreateCellData` allocates the new block in the game; here the
        // double does it.
        e.register(CELL_CREATE_CELL_DATA, |e, a| {
            let data = e.mem.alloc(0x2c);
            e.mem.set_u32(a[0] + 0x48, data);
            Ret::default()
        });
        e.call(0x0054_3ab0, &args![this, source]);
        let new = e.mem.u32(this.addr() + 0x48);
        let from = e.mem.u32(source.addr() + 0x48);
        assert_eq!(e.mem.bytes(new, 0x2c), e.mem.bytes(from, 0x2c));
        // No data block on the source: nothing is copied.
        let (mut e, this, source) = copy_pair(false);
        e.mem.set_u32(source.addr() + 0x48, 0);
        e.register(CELL_CREATE_CELL_DATA, |e, a| {
            let data = e.mem.alloc(0xc);
            e.mem.set_u32(a[0] + 0x48, data);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0054_3ab0, &args![this, source]);
        assert!(calls_to(&e.call_log.take().unwrap(), MEMORY_COPY).is_empty());
    }

    #[test]
    fn copying_from_something_that_is_not_a_cell_does_nothing() {
        let (mut e, this, source) = copy_pair(false);
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0054_3ab0, &args![this, source]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn comparing_cells_finds_every_kind_of_difference() {
        for interior in [false, true] {
            let (mut e, this, source) = copy_pair(interior);
            quiet(
                &mut e,
                &[FORM_COMPARE_ALL_COMPONENTS, EXTRA_LIST_COMPARE_LIST],
            );
            let size = if interior { 0x2c } else { 0xc };
            // Make the pair equal first.
            let from = e.mem.u32(source.addr() + 0x48);
            let to = e.mem.u32(this.addr() + 0x48);
            let bytes = e.mem.bytes(from, size);
            e.mem.write(to, &bytes);
            for (offset, width) in [(0x24u32, 1u32), (0x50, 4), (0xd8, 4), (0xdc, 4)] {
                let value = e.mem.bytes(source.addr() + offset, width);
                e.mem.write(this.addr() + offset, &value);
            }
            let different = |e: &mut Engine| e.call(0x0054_3c50, &args![this, source]).bool();
            assert!(!different(&mut e), "equal cells");

            // Each difference alone makes the cells different.
            let flags = e.mem.u8(this.addr() + 0x24);
            e.mem.set_u8(this.addr() + 0x24, flags ^ 0x40);
            assert!(different(&mut e), "cell flags");
            e.mem.set_u8(this.addr() + 0x24, flags);
            e.mem.set_u32(this.addr() + 0xdc, 0x44);
            assert!(different(&mut e), "inheritance flags");
            e.mem.set_u32(this.addr() + 0xdc, 0x33);
            e.mem.set_u32(this.addr() + 0xd8, 0x7001);
            assert!(different(&mut e), "lighting template");
            e.mem.set_u32(this.addr() + 0xd8, 0x7000);
            e.mem.set_f32(this.addr() + 0x50, 1.0);
            assert!(different(&mut e), "water height");
            e.mem.set_f32(this.addr() + 0x50, f32::NAN);
            e.mem.set_f32(source.addr() + 0x50, f32::NAN);
            assert!(different(&mut e), "a NaN height is different from itself");
            e.mem.set_f32(this.addr() + 0x50, 42.5);
            e.mem.set_f32(source.addr() + 0x50, 42.5);
            assert!(!different(&mut e));
            e.mem.set_u8(to, e.mem.u8(to) ^ 1);
            assert!(different(&mut e), "data block");
            e.mem.set_u8(to, e.mem.u8(to) ^ 1);
            // A missing block on either side is not a difference.
            e.mem.set_u32(this.addr() + 0x48, 0);
            assert!(!different(&mut e));
            e.mem.set_u32(this.addr() + 0x48, to);
            returns(&mut e, FORM_COMPARE_ALL_COMPONENTS, 1);
            assert!(different(&mut e), "form components");
            returns(&mut e, FORM_COMPARE_ALL_COMPONENTS, 0);
            returns(&mut e, EXTRA_LIST_COMPARE_LIST, 1);
            assert!(different(&mut e), "extra data");
            returns(&mut e, EXTRA_LIST_COMPARE_LIST, 0);
            // The texture component has the last word.
            e.mem.set_u32(SCRATCH + 0x40, 1);
            assert!(different(&mut e), "texture component");
            e.mem.set_u32(SCRATCH + 0x40, 0);
            // Not a cell at all.
            e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
            assert!(different(&mut e), "not a cell");
        }
    }

    // ---- BelongsInGroup and CreateGroupData -------------------------

    /// `BelongsInGroup(header, a, b)` of `cell` for a group of the given type
    /// and label.
    fn belongs_to(
        e: &mut Engine,
        cell: Ptr<TESObjectCELL>,
        group_type: u32,
        label: u32,
        with_children: u8,
        strict: u8,
    ) -> bool {
        let header = header(e, group_type, label);
        e.call(0x0054_3df0, &args![cell, header, with_children, strict])
            .bool()
    }

    #[test]
    fn cells_belong_in_the_groups_that_name_them() {
        let mut e = engine();
        let world = world_with_key(&mut e, 1);
        e.mem.set_u32(world.addr() + BELONGS, 1);
        let exterior = ordered_cell(&mut e, false, (64, 0), 0x105, world);
        let interior = ordered_cell(&mut e, true, (0, 0), 0x105, Ptr::NULL);
        // Not a header.
        assert!(!e
            .call(0x0054_3df0, &args![exterior, Ptr::<()>::NULL, 1u8, 1u8])
            .bool());
        let bad = header(&mut e, 0, 0);
        e.mem.set_u32(bad.addr(), 9);
        assert!(!e.call(0x0054_3df0, &args![exterior, bad, 1u8, 1u8]).bool());

        // Type 0: only with the first flag. An interior asks the generic
        // code, an exterior its world space (virtual slot 0x110).
        let top = header(&mut e, 0, 0);
        returns(&mut e, FORM_BELONGS_IN_GROUP_INTERIOR, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_3df0, &args![interior, top, 1u8, 7u8]).bool());
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), FORM_BELONGS_IN_GROUP_INTERIOR),
            vec![vec![interior.addr(), top.addr(), 1, 7]]
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_3df0, &args![exterior, top, 1u8, 7u8]).bool());
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), fake(0x110)),
            vec![vec![world.addr(), top.addr(), 1, 7]]
        );
        assert!(!e.call(0x0054_3df0, &args![exterior, top, 0u8, 7u8]).bool());
        assert!(!e.call(0x0054_3df0, &args![interior, top, 0u8, 7u8]).bool());

        // Type 1: an exterior cell whose world space contains the label.
        returns(&mut e, FORM_ID_IN_WORLD_SPACE, 1);
        assert!(belongs_to(&mut e, exterior, 1, 0x77, 1, 0));
        assert!(
            !belongs_to(&mut e, exterior, 1, 0x77, 0, 0),
            "needs the flag or persistence"
        );
        e.mem.set_u32(exterior.addr() + 8, 0x400);
        assert!(belongs_to(&mut e, exterior, 1, 0x77, 0, 0));
        e.mem.set_u32(exterior.addr() + 8, 0);
        assert!(!belongs_to(&mut e, interior, 1, 0x77, 1, 0));
        returns(&mut e, FORM_ID_IN_WORLD_SPACE, 0);
        assert!(!belongs_to(&mut e, exterior, 1, 0x77, 1, 0));

        // Types 2 and 4: the block key, only when the flag is set and the
        // cell is not persistent. Types 3 and 5: the sub-block key.
        let block_key = 2u32 << 16;
        let sub_key = 8u32 << 16;
        assert!(belongs_to(&mut e, exterior, 2, block_key, 1, 0));
        assert!(belongs_to(&mut e, exterior, 4, block_key, 1, 0));
        assert!(!belongs_to(&mut e, exterior, 4, block_key, 0, 0));
        assert!(!belongs_to(&mut e, exterior, 4, block_key + 1, 1, 0));
        assert!(belongs_to(&mut e, exterior, 3, sub_key, 0, 0));
        assert!(belongs_to(&mut e, exterior, 5, sub_key, 0, 0));
        assert!(!belongs_to(&mut e, exterior, 5, sub_key + 1, 0, 0));
        e.mem.set_u32(exterior.addr() + 8, 0x400);
        assert!(!belongs_to(&mut e, exterior, 2, block_key, 1, 0));
        assert!(!belongs_to(&mut e, exterior, 3, sub_key, 1, 0));
        // An unknown type.
        assert!(!belongs_to(&mut e, exterior, 6, 0, 1, 1));
    }

    #[test]
    fn cells_build_the_header_of_their_group() {
        let mut e = engine();
        e.set_global(EXTERIOR_PARENT_LABEL, 0xe0e0u32);
        e.set_global(INTERIOR_PARENT_LABEL, 0xa1a1u32);
        let world = world_with_key(&mut e, 1);
        e.mem.set_u32(world.addr() + 0xc, 0x3c);
        let exterior = ordered_cell(&mut e, false, (64, 0), 0x105, world);
        let interior = ordered_cell(&mut e, true, (0, 0), 0x105, Ptr::NULL);
        let words = |e: &Engine, out: Ptr| -> Vec<u32> {
            (0..5).map(|i| e.mem.u32(out.addr() + 4 * i)).collect()
        };
        let tag = 0x5055_5247u32;
        let build = |e: &mut Engine, cell: Ptr<TESObjectCELL>, parent: Ptr| -> Vec<u32> {
            let out = e.mem.alloc(0x14);
            e.mem.write(out, &[0xee; 0x14]);
            e.call(0x0054_3f50, &args![cell, Ptr::<()>::new(out), parent]);
            words(e, Ptr::new(out))
        };
        let block_key = 2u32 << 16;
        let sub_key = 8u32 << 16;

        // A null output is ignored (nothing to check but that it does not crash).
        e.call(
            0x0054_3f50,
            &args![exterior, Ptr::<()>::NULL, Ptr::<()>::NULL],
        );

        // Exterior: without a parent only the first word is cleared.
        let cleared = build(&mut e, exterior, Ptr::NULL);
        assert_eq!(cleared[0], 0);
        assert_eq!(cleared[1], 0xeeee_eeee);
        // World, block, sub-block groups.
        let top = header(&mut e, 0, 0xe0e0);
        assert_eq!(build(&mut e, exterior, top), vec![tag, 0, 0x3c, 1, 0]);
        let wrong = header(&mut e, 0, 0x1);
        assert_eq!(build(&mut e, exterior, wrong)[0], 0);
        let children = header(&mut e, 1, 0x3c);
        assert_eq!(
            build(&mut e, exterior, children),
            vec![tag, 0, block_key, 4, 0]
        );
        let other_world = header(&mut e, 1, 0x3d);
        assert_eq!(build(&mut e, exterior, other_world)[0], 0);
        let blocks = header(&mut e, 4, block_key);
        assert_eq!(build(&mut e, exterior, blocks), vec![tag, 0, sub_key, 5, 0]);
        let wrong_block = header(&mut e, 4, 1);
        assert_eq!(build(&mut e, exterior, wrong_block)[0], 0);
        // A persistent cell has no block group.
        e.mem.set_u32(exterior.addr() + 8, 0x400);
        assert_eq!(build(&mut e, exterior, children)[0], 0);
        assert_eq!(build(&mut e, exterior, blocks)[0], 0);
        // Other group types of the parent give nothing.
        let sub_blocks = header(&mut e, 5, sub_key);
        assert_eq!(build(&mut e, exterior, sub_blocks)[0], 0);

        // Interior: no parent gives the top-level interior group.
        assert_eq!(
            build(&mut e, interior, Ptr::NULL),
            vec![tag, 0, 0xa1a1, 0, 0]
        );
        let interior_top = header(&mut e, 0, 0xa1a1);
        // Id 0x105: block key 1, sub-block key 6.
        assert_eq!(build(&mut e, interior, interior_top), vec![tag, 0, 1, 2, 0]);
        let wrong_top = header(&mut e, 0, 0xa1a2);
        assert_eq!(build(&mut e, interior, wrong_top)[0], 0);
        let interior_blocks = header(&mut e, 2, 1);
        assert_eq!(
            build(&mut e, interior, interior_blocks),
            vec![tag, 0, 6, 3, 0]
        );
        let wrong_blocks = header(&mut e, 2, 2);
        assert_eq!(build(&mut e, interior, wrong_blocks)[0], 0);
        let sub = header(&mut e, 3, 6);
        assert_eq!(build(&mut e, interior, sub)[0], 0);
    }

    // ---- the loaded data and the interior lighting accessors ----------

    /// The vtable of the wide test objects: slot `s` (up to `0x1f4`) leads to
    /// the double at `wide(s)`, which answers with the word at
    /// `SLOT_ANSWERS + s` of the object, so one object type serves as a
    /// reference, a node, a room, a bound shape or an owner.
    const WIDE_VTABLE: u32 = 0x0130_4000;
    const WIDE_SIZE: u32 = 0x400;
    const SLOT_ANSWERS: u32 = 0x200;
    // Words the doubles of the reference helpers read from a wide object.
    const IS_MARKER: u32 = 0x100;
    const LINKED_OWNER: u32 = 0x104;
    const IS_SCRIPTED: u32 = 0x108;
    const IS_ACTIVATING: u32 = 0x10c;
    const ROOM: u32 = 0x110;
    const BASE_FORM: u32 = 0x114;
    const MULTI_BOUND_ROOM: u32 = 0x118;
    const MULTI_BOUND_DATA: u32 = 0x11c;
    const PRIMITIVE: u32 = 0x120;
    const SHAPE: u32 = 0x124;
    const CACHED_NODE: u32 = 0x128;
    const EMITTANCE_SOURCE: u32 = 0x12c;

    fn wide(slot: u32) -> u32 {
        0x7200_0000 + slot
    }

    /// The test engine plus the wide vtable and doubles for the reference,
    /// list, map and node helpers the loaded-data functions call.
    fn wide_engine() -> Engine {
        let mut e = engine();
        e.map(0x0102_e000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        let slots: Vec<u32> = (0..=0x1f4 / 4).map(|i| wide(4 * i)).collect();
        e.put_vtable(WIDE_VTABLE, &slots);
        for i in 0..=0x1f4u32 / 4 {
            e.register_double(wide(4 * i), move |e, a| {
                e.mem.u32(a[0] + SLOT_ANSWERS + 4 * i).into_ret()
            });
        }
        let readers: [(u32, u32); 9] = [
            (REFERENCE_IS_MARKER_FORM, IS_MARKER),
            (REFERENCE_LINKED_NODE_OWNER, LINKED_OWNER),
            (REFERENCE_IS_SCRIPTED, IS_SCRIPTED),
            (REFERENCE_IS_ACTIVATING_CHILDREN, IS_ACTIVATING),
            (REFERENCE_GET_BASE_FORM, BASE_FORM),
            (REFERENCE_GET_MULTI_BOUND_ROOM, MULTI_BOUND_ROOM),
            (REFERENCE_GET_MULTI_BOUND, MULTI_BOUND_DATA),
            (MULTI_BOUND_DATA_OF_ROOM, MULTI_BOUND_DATA),
            (MULTI_BOUND_PRIMITIVE_SLOT, PRIMITIVE),
        ];
        for (address, word) in readers {
            e.register_double(address, move |e, a| e.mem.u32(a[0] + word).into_ret());
        }
        e.register(REFERENCE_EMITTANCE_SOURCE, |e, a| {
            e.mem.u32(a[0] + EMITTANCE_SOURCE).into_ret()
        });
        e.register(REFERENCE_EXTRA_DATA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(EXTRA_DATA_GET_ROOM, |e, a| {
            e.mem.u32(a[0] - 0x44 + ROOM).into_ret()
        });
        e.register(EXTRA_DATA_GET_PRIMITIVE, |e, a| {
            e.mem.u32(a[0] - 0x44 + SHAPE).into_ret()
        });
        e.register(NODE_ALLOCATE, |e, _| e.mem.alloc(WIDE_SIZE).into_ret());
        e.register(MULTI_BOUND_NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], WIDE_VTABLE);
            a[0].into_ret()
        });
        e.register(MAP_GET_AT, |e, a| {
            let cached = e.mem.u32(a[1] + CACHED_NODE);
            if cached != 0 {
                e.mem.set_u32(a[2], cached);
            }
            u32::from(cached != 0).into_ret()
        });
        e.register(REFERENCE_GET_ORIENTATION, |_, a| a[1].into_ret());
        e.register(LOADER_FLAG_244_2, |e, a| {
            u32::from(e.mem.u32(a[0] + 0x244) & 2 != 0).into_ret()
        });
        quiet(
            &mut e,
            &[
                LOCK_ENTER,
                LOCK_LEAVE,
                LIST_CONSTRUCT,
                LIST_CLEAR,
                LIST_DESTRUCT,
                MULTI_BOUND_SET_DATA,
                MULTI_BOUND_SET_PRIMITIVE,
                PRIMITIVE_SET_ROTATION,
                MAP_SET_AT,
                MAP_SET_AT_MULTI_BOUND,
                MAP_REMOVE_AT,
                MAP_CONSTRUCT_REFERENCE_NODE,
                MAP_CONSTRUCT_FORM_REFERENCE,
                MAP_CONSTRUCT_MULTI_BOUND,
                MAP_DESTRUCT_REFERENCE_NODE,
                MAP_DESTRUCT_FORM_REFERENCE,
                MAP_DESTRUCT_MULTI_BOUND,
                REFERENCE_CLEAR_MULTI_BOUND,
                CELL_DETACH_LOADED_3D,
                CLONE_MATERIAL_PROPERTY,
                NODE_UPDATE_PROPERTIES,
                NODE_GET_PROPERTY,
                SHADER_APPLY_PROPERTY,
                LIST_REMOVE_ITEM,
                LIST_INSERT_AFTER,
            ],
        );
        // A `BSSimpleList` head is { item, next }, empty when both are 0.
        e.register(LIST_PUSH_FRONT, |e, a| {
            let value = e.mem.u32(a[1]);
            let (item, next) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
            if item == 0 && next == 0 {
                e.mem.set_u32(a[0], value);
            } else {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, item);
                e.mem.set_u32(node + 4, next);
                e.mem.set_u32(a[0], value);
                e.mem.set_u32(a[0] + 4, node);
            }
            Ret::default()
        });
        e.register(LIST_POP_FRONT, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
                e.mem.free(next);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        e.set_global(COLOUR_BYTE_SCALE, 255.0f64);
        e.set_global(LARGE_BOUND_SIZE, 3000.0f64);
        e
    }

    fn wide_object(e: &mut Engine) -> Ptr {
        let block = e.mem.alloc(WIDE_SIZE);
        e.mem.set_u32(block, WIDE_VTABLE);
        Ptr::new(block)
    }

    /// Sets the answer of virtual slot `slot` of a wide object.
    fn answer(e: &mut Engine, object: Ptr, slot: u32, value: u32) {
        e.mem.set_u32(object.addr() + SLOT_ANSWERS + slot, value);
    }

    fn word(e: &mut Engine, object: Ptr, offset: u32, value: u32) {
        e.mem.set_u32(object.addr() + offset, value);
    }

    /// A cell with loaded data (zeroed).
    fn loaded_cell(e: &mut Engine, interior: bool) -> (Ptr<TESObjectCELL>, Ptr<LoadedCellData>) {
        let cell = cell(e, interior);
        let loaded = e.new_object::<LoadedCellData>();
        e.set(cell, TESObjectCELL::pLoadedData, loaded.cast());
        (cell, loaded)
    }

    /// The items of a `BSSimpleList` whose head is at `head`.
    fn list_items(e: &Engine, head: u32) -> Vec<u32> {
        let mut items = vec![];
        let mut node = head;
        while node != 0 && !(e.mem.u32(node) == 0 && e.mem.u32(node + 4) == 0) {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    /// A cell, for the checkers whose own variable is called `cell`.
    fn new_cell(e: &mut Engine, interior: bool) -> Ptr<TESObjectCELL> {
        cell(e, interior)
    }

    fn take_log(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()
    }

    /// An interior cell with zeroed interior data.
    fn lit_cell(e: &mut Engine) -> (Ptr<TESObjectCELL>, Ptr<InteriorData>) {
        let cell = cell(e, true);
        let data = e.new_object::<InteriorData>();
        e.set(cell, TESObjectCELL::pCellData, data.cast());
        e.register(CELL_INHERITS_LIGHTING_FIELD, |e, a| {
            u32::from(e.mem.u32(a[0] + 0xdc) & a[1] != 0).into_ret()
        });
        (cell, data)
    }

    #[test]
    fn world_flag_80000_comes_from_the_cell_or_its_world_space() {
        let mut e = engine();
        returns(&mut e, WORLD_SPACE_FLAG_80000, 1);
        let interior = cell(&mut e, true);
        let exterior = cell(&mut e, false);
        let world = object(&mut e);
        e.set(exterior, TESObjectCELL::pWorldSpace, world);
        // An interior cell tests its own flag.
        assert!(!e.call(0x0054_44c0, &args![interior]).bool());
        e.mem.set_u32(interior.addr() + 8, 0x80000);
        assert!(e.call(0x0054_44c0, &args![interior]).bool());
        // An exterior cell asks its world space, if it has one.
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_44c0, &args![exterior]).bool());
        assert_eq!(
            calls_to(&take_log(&mut e), WORLD_SPACE_FLAG_80000),
            vec![vec![world.addr()]]
        );
        returns(&mut e, WORLD_SPACE_FLAG_80000, 0);
        assert!(!e.call(0x0054_44c0, &args![exterior]).bool());
        e.set(exterior, TESObjectCELL::pWorldSpace, Ptr::NULL);
        returns(&mut e, WORLD_SPACE_FLAG_80000, 1);
        assert!(!e.call(0x0054_44c0, &args![exterior]).bool());
    }

    #[test]
    fn cell_flag_4_is_inverted_for_interiors_and_falls_back_to_the_world_space() {
        let mut e = engine();
        returns(&mut e, WORLD_SPACE_FLAG_4C_2, 1);
        let interior = cell(&mut e, true);
        let exterior = cell(&mut e, false);
        let world = object(&mut e);
        e.set(exterior, TESObjectCELL::pWorldSpace, world);
        // Interior: bit 2 clear gives true without asking the world space.
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_4520, &args![interior]).bool());
        assert!(calls_to(&take_log(&mut e), WORLD_SPACE_FLAG_4C_2).is_empty());
        // Interior with the bit set: false, there is no world space.
        e.mem.set_u8(interior.addr() + 0x24, 0x05);
        assert!(!e.call(0x0054_4520, &args![interior]).bool());
        // Exterior: the bit gives true, otherwise the world space decides.
        e.mem.set_u8(exterior.addr() + 0x24, 0x04);
        assert!(e.call(0x0054_4520, &args![exterior]).bool());
        e.mem.set_u8(exterior.addr() + 0x24, 0x00);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_4520, &args![exterior]).bool());
        assert_eq!(
            calls_to(&take_log(&mut e), WORLD_SPACE_FLAG_4C_2),
            vec![vec![world.addr()]]
        );
        returns(&mut e, WORLD_SPACE_FLAG_4C_2, 0);
        assert!(!e.call(0x0054_4520, &args![exterior]).bool());
        e.set(exterior, TESObjectCELL::pWorldSpace, Ptr::NULL);
        assert!(!e.call(0x0054_4520, &args![exterior]).bool());
    }

    #[test]
    fn land_hide_flags_are_tested_bit_by_bit_with_sign_extension() {
        let mut e = engine();
        let exterior = cell(&mut e, false);
        let interior = cell(&mut e, true);
        // No data: false.
        assert!(!fn_00544590(&mut e, exterior, 0));
        let data = e.new_object::<ExteriorData>();
        e.set(exterior, TESObjectCELL::pCellData, data.cast());
        e.set(interior, TESObjectCELL::pCellData, data.cast());
        e.set(data, ExteriorData::cLandHideFlags, 0b0000_0100);
        assert!(fn_00544590(&mut e, exterior, 2));
        assert!(!fn_00544590(&mut e, exterior, 3));
        // The shift count is taken modulo 32.
        assert!(fn_00544590(&mut e, exterior, 34));
        // The `char` is sign-extended: a negative flags byte sets bits 8..32.
        e.set(data, ExteriorData::cLandHideFlags, -128);
        assert!(fn_00544590(&mut e, exterior, 7));
        assert!(fn_00544590(&mut e, exterior, 20));
        assert!(!fn_00544590(&mut e, exterior, 0));
        // An interior cell has no exterior data.
        assert!(!fn_00544590(&mut e, interior, 7));
    }

    #[test]
    fn exterior_and_interior_data_are_told_apart_by_the_interior_flag() {
        let mut e = engine();
        let exterior = cell(&mut e, false);
        let interior = cell(&mut e, true);
        let data = e.new_object::<ExteriorData>();
        e.set(exterior, TESObjectCELL::pCellData, data.cast());
        e.set(interior, TESObjectCELL::pCellData, data.cast());
        assert_eq!(fn_005445d0(&mut e, exterior), data.cast());
        assert_eq!(fn_005445d0(&mut e, interior), Ptr::NULL);
    }

    #[test]
    fn interior_data_is_only_for_interior_cells() {
        let mut e = engine();
        let exterior = cell(&mut e, false);
        let interior = cell(&mut e, true);
        let data = e.new_object::<InteriorData>();
        e.set(exterior, TESObjectCELL::pCellData, data.cast());
        e.set(interior, TESObjectCELL::pCellData, data.cast());
        assert_eq!(fn_00544600(&mut e, interior), data.cast());
        assert_eq!(fn_00544600(&mut e, exterior), Ptr::NULL);
    }

    #[test]
    fn creating_cell_data_replaces_the_old_data_by_kind() {
        let mut e = engine();
        e.register(EXTERIOR_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(INTERIOR_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        let exterior = cell(&mut e, false);
        let old = e.mem.alloc(0xc);
        e.set(exterior, TESObjectCELL::pCellData, Ptr::new(old));
        e.call_log = Some(vec![]);
        e.call(0x0054_4630, &args![exterior]);
        let log = take_log(&mut e);
        let fresh = e.get(exterior, TESObjectCELL::pCellData);
        assert_ne!(fresh.addr(), 0);
        assert_eq!(fresh.addr(), e.mem.u32(exterior.addr() + 0x48));
        assert_eq!(calls_to(&log, DEALLOCATE), vec![vec![old]]);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0xc]]);
        assert_eq!(
            calls_to(&log, EXTERIOR_DATA_CONSTRUCT),
            vec![vec![fresh.addr()]]
        );
        assert!(calls_to(&log, INTERIOR_DATA_CONSTRUCT).is_empty());

        // An interior cell without data allocates the larger block and frees
        // nothing.
        let interior = cell(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0054_4630, &args![interior]);
        let log = take_log(&mut e);
        let fresh = e.get(interior, TESObjectCELL::pCellData);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x2c]]);
        assert!(calls_to(&log, DEALLOCATE).is_empty());
        assert_eq!(
            calls_to(&log, INTERIOR_DATA_CONSTRUCT),
            vec![vec![fresh.addr()]]
        );

        // A failed allocation leaves the data null and builds nothing.
        returns(&mut e, ALLOCATE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0054_4630, &args![interior]);
        assert_eq!(e.get(interior, TESObjectCELL::pCellData), Ptr::NULL);
        assert!(calls_to(&take_log(&mut e), INTERIOR_DATA_CONSTRUCT).is_empty());
    }

    /// Checks an accessor that returns a word of the interior data or the
    /// lighting template's value (`template_accessor`) when the cell
    /// inherits it (`mask`).
    fn check_word_accessor(
        address: u32,
        mask: u32,
        field: Field<InteriorData, u32>,
        template_accessor: u32,
    ) {
        let mut e = wide_engine();
        let (cell, data) = lit_cell(&mut e);
        e.set(data, field, 0x1122_3344);
        assert_eq!(e.call(address, &args![cell]).u32(), 0x1122_3344);
        // The inheritance bit without a lighting template changes nothing.
        e.mem.set_u32(cell.addr() + 0xdc, mask);
        assert_eq!(e.call(address, &args![cell]).u32(), 0x1122_3344);
        // With a template the accessor of the template answers.
        let template = object(&mut e);
        e.mem.set_u32(cell.addr() + 0xd8, template.addr());
        returns(&mut e, template_accessor, 0x5566_7788);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &args![cell]).u32(), 0x5566_7788);
        assert_eq!(
            calls_to(&take_log(&mut e), template_accessor),
            vec![vec![template.addr()]]
        );
        // The other bits do not inherit it.
        e.mem.set_u32(cell.addr() + 0xdc, !mask);
        assert_eq!(e.call(address, &args![cell]).u32(), 0x1122_3344);
        // An exterior cell has no interior data.
        let exterior = new_cell(&mut e, false);
        assert_eq!(e.call(address, &args![exterior]).u32(), 0);
    }

    /// The `float` counterpart of [`check_word_accessor`]; `missing` is the
    /// result for a cell without interior data.
    fn check_float_accessor(
        address: u32,
        mask: u32,
        field: Field<InteriorData, f32>,
        template_accessor: u32,
        missing: f32,
    ) {
        let mut e = wide_engine();
        let (cell, data) = lit_cell(&mut e);
        e.set(data, field, 2.5);
        assert_eq!(e.call(address, &args![cell]).f32(), 2.5);
        e.mem.set_u32(cell.addr() + 0xdc, mask);
        assert_eq!(e.call(address, &args![cell]).f32(), 2.5);
        let template = object(&mut e);
        e.mem.set_u32(cell.addr() + 0xd8, template.addr());
        e.register_double(template_accessor, |_, _| 7.75f32.into_ret());
        assert_eq!(e.call(address, &args![cell]).f32(), 7.75);
        e.mem.set_u32(cell.addr() + 0xdc, !mask);
        assert_eq!(e.call(address, &args![cell]).f32(), 2.5);
        let exterior = new_cell(&mut e, false);
        assert_eq!(e.call(address, &args![exterior]).f32(), missing);
    }

    /// Checks a colour accessor: the packed value `0x00ff8040` becomes
    /// `(0x40, 0x80, 0xff) / 255`, from the data or the template.
    fn check_colour_accessor(
        address: u32,
        mask: u32,
        field: Field<InteriorData, u32>,
        template_accessor: u32,
    ) {
        let mut e = wide_engine();
        let (cell, data) = lit_cell(&mut e);
        let out = e.mem.alloc(12);
        let read = |e: &Engine| -> Vec<f32> { (0..3).map(|i| e.mem.f32(out + 4 * i)).collect() };
        let expected = |packed: u32| -> Vec<f32> {
            [packed & 0xff, (packed >> 8) & 0xff, (packed >> 16) & 0xff]
                .iter()
                .map(|c| (*c as f64 / 255.0) as f32)
                .collect()
        };
        e.set(data, field, 0x00ff_8040);
        e.call(address, &args![cell, Ptr::<()>::new(out)]);
        assert_eq!(read(&e), expected(0x00ff_8040));
        assert_eq!(read(&e)[2], 1.0);
        // Inherited from the lighting template.
        let template = object(&mut e);
        e.mem.set_u32(cell.addr() + 0xd8, template.addr());
        e.mem.set_u32(cell.addr() + 0xdc, mask);
        returns(&mut e, template_accessor, 0x0010_2030);
        e.call(address, &args![cell, Ptr::<()>::new(out)]);
        assert_eq!(read(&e), expected(0x0010_2030));
        // The high byte of the packed colour is ignored.
        returns(&mut e, template_accessor, 0xff10_2030);
        e.call(address, &args![cell, Ptr::<()>::new(out)]);
        assert_eq!(read(&e), expected(0x0010_2030));
    }

    #[test]
    fn ambient_colour_word() {
        check_word_accessor(0x0054_4750, 0x1, InteriorData::iAmbient, TEMPLATE_AMBIENT);
    }

    #[test]
    fn ambient_colour_floats() {
        check_colour_accessor(0x0054_47b0, 0x1, InteriorData::iAmbient, TEMPLATE_AMBIENT);
    }

    #[test]
    fn directional_colour_word() {
        check_word_accessor(
            0x0054_4830,
            0x2,
            InteriorData::iDirectional,
            TEMPLATE_DIRECTIONAL,
        );
    }

    #[test]
    fn directional_colour_floats() {
        check_colour_accessor(
            0x0054_4890,
            0x2,
            InteriorData::iDirectional,
            TEMPLATE_DIRECTIONAL,
        );
    }

    #[test]
    fn directional_xy_word() {
        check_word_accessor(
            0x0054_4910,
            0x20,
            InteriorData::iDirectionalXY,
            TEMPLATE_DIRECTIONAL_XY,
        );
    }

    #[test]
    fn directional_z_word() {
        check_word_accessor(
            0x0054_4970,
            0x20,
            InteriorData::iDirectionalZ,
            TEMPLATE_DIRECTIONAL_Z,
        );
    }

    #[test]
    fn fog_colour_word() {
        check_word_accessor(0x0054_49d0, 0x4, InteriorData::iFog, TEMPLATE_FOG);
    }

    #[test]
    fn fog_colour_floats() {
        check_colour_accessor(0x0054_4a30, 0x4, InteriorData::iFog, TEMPLATE_FOG);
    }

    #[test]
    fn fog_near_distance() {
        check_float_accessor(
            0x0054_4ab0,
            0x8,
            InteriorData::fFogNear,
            TEMPLATE_FOG_NEAR,
            0.0,
        );
    }

    #[test]
    fn fog_far_distance() {
        check_float_accessor(
            0x0054_4b10,
            0x10,
            InteriorData::fFogFar,
            TEMPLATE_FOG_FAR,
            0.0,
        );
    }

    #[test]
    fn fog_power_defaults_to_one() {
        check_float_accessor(
            0x0054_4b70,
            0x100,
            InteriorData::fFogPower,
            TEMPLATE_FOG_POWER,
            1.0,
        );
    }

    #[test]
    fn clip_distance() {
        check_float_accessor(
            0x0054_4bd0,
            0x80,
            InteriorData::fClipDist,
            TEMPLATE_CLIP_DISTANCE,
            0.0,
        );
    }

    #[test]
    fn grid_coordinates_read_the_exterior_data() {
        let mut e = engine();
        let exterior = cell(&mut e, false);
        let interior = cell(&mut e, true);
        assert_eq!(tes_object_cell_get_data_x(&mut e, exterior), 0);
        assert_eq!(tes_object_cell_get_data_y(&mut e, exterior), 0);
        let data = e.new_object::<ExteriorData>();
        e.set(data, ExteriorData::iCellX, -7);
        e.set(data, ExteriorData::iCellY, 12);
        e.set(exterior, TESObjectCELL::pCellData, data.cast());
        e.set(interior, TESObjectCELL::pCellData, data.cast());
        assert_eq!(tes_object_cell_get_data_x(&mut e, exterior), -7);
        assert_eq!(tes_object_cell_get_data_y(&mut e, exterior), 12);
        // An interior cell has no exterior data.
        assert_eq!(tes_object_cell_get_data_x(&mut e, interior), 0);
        assert_eq!(tes_object_cell_get_data_y(&mut e, interior), 0);
    }

    #[test]
    fn grid_coordinates_are_stored_in_the_exterior_data_only() {
        let mut e = engine();
        let exterior = cell(&mut e, false);
        let interior = cell(&mut e, true);
        // Nothing to store into: no crash.
        tes_object_cell_set_data_coord(&mut e, exterior, 1, 2);
        let data = e.new_object::<ExteriorData>();
        e.set(exterior, TESObjectCELL::pCellData, data.cast());
        tes_object_cell_set_data_coord(&mut e, exterior, -3, 40);
        assert_eq!(e.get(data, ExteriorData::iCellX), -3);
        assert_eq!(e.get(data, ExteriorData::iCellY), 40);
        // An interior cell ignores it.
        e.set(interior, TESObjectCELL::pCellData, data.cast());
        tes_object_cell_set_data_coord(&mut e, interior, 9, 9);
        assert_eq!(e.get(data, ExteriorData::iCellX), -3);
        // The registered form takes the same words.
        e.call(0x0054_4c90, &args![exterior, 5i32, 6i32]);
        assert_eq!(e.get(data, ExteriorData::iCellY), 6);
    }

    #[test]
    fn building_the_loaded_data_constructs_every_member() {
        let mut e = wide_engine();
        let block = e.mem.alloc(LoadedCellData::SIZE);
        let loaded = Ptr::<()>::new(block);
        e.call_log = Some(vec![]);
        let result = e.call(0x0054_4f60, &args![loaded]);
        assert_eq!(result.u32(), block);
        let log = take_log(&mut e);
        assert_eq!(
            sequence(&log),
            vec![
                SLOT_CONSTRUCT,
                LIST_CONSTRUCT,
                MAP_CONSTRUCT_REFERENCE_NODE,
                MAP_CONSTRUCT_FORM_REFERENCE,
                MAP_CONSTRUCT_REFERENCE_NODE,
                MAP_CONSTRUCT_MULTI_BOUND,
                LIST_CONSTRUCT,
                LIST_CONSTRUCT,
                LIST_CONSTRUCT,
            ]
        );
        assert_eq!(calls_to(&log, SLOT_CONSTRUCT), vec![vec![block, 0]]);
        assert_eq!(
            calls_to(&log, LIST_CONSTRUCT),
            vec![
                vec![block + 0x04],
                vec![block + 0x4c],
                vec![block + 0x54],
                vec![block + 0x5c]
            ]
        );
        // The maps, each with 0x25 buckets.
        assert_eq!(
            calls_to(&log, MAP_CONSTRUCT_REFERENCE_NODE),
            vec![vec![block + 0x0c, 0x25], vec![block + 0x2c, 0x25],]
        );
        assert_eq!(
            calls_to(&log, MAP_CONSTRUCT_FORM_REFERENCE),
            vec![vec![block + 0x1c, 0x25]]
        );
        assert_eq!(
            calls_to(&log, MAP_CONSTRUCT_MULTI_BOUND),
            vec![vec![block + 0x3c, 0x25]]
        );
    }

    /// A marker reference with a room (and a bound shape on the room's
    /// multibound data), as `fn_00545740` handles it.
    fn marker_reference(e: &mut Engine) -> (Ptr, Ptr, Ptr, Ptr) {
        let reference = wide_object(e);
        let room = wide_object(e);
        let data = wide_object(e);
        let primitive = wide_object(e);
        word(e, reference, IS_MARKER, 1);
        word(e, reference, ROOM, room.addr());
        word(e, room, MULTI_BOUND_DATA, data.addr());
        word(e, data, PRIMITIVE, primitive.addr());
        (reference, room, data, primitive)
    }

    #[test]
    fn building_the_loaded_data_sorts_the_references() {
        let mut e = wide_engine();
        let cell = cell(&mut e, false);
        let loader = e.mem.alloc(0x250);
        e.set_global(GAME_LOADER_POINTER, loader);
        let world = wide_object(&mut e);
        e.set(cell, TESObjectCELL::pWorldSpace, world);

        let plain = wide_object(&mut e);
        word(&mut e, plain, IS_SCRIPTED, 1);
        word(&mut e, plain, IS_ACTIVATING, 1);
        let actor = wide_object(&mut e);
        answer(&mut e, actor, 0x100, 1);
        word(&mut e, actor, IS_SCRIPTED, 1);
        let hidden = wide_object(&mut e);
        word(&mut e, hidden, 8, 0x20);
        word(&mut e, hidden, IS_SCRIPTED, 1);
        word(&mut e, hidden, IS_ACTIVATING, 1);
        // A marker reference whose owner is another marker with a node.
        let (linked, linked_room, _, _) = marker_reference(&mut e);
        let (owner, owner_room, _, _) = marker_reference(&mut e);
        word(&mut e, linked, LINKED_OWNER, owner.addr());
        word(&mut e, owner, CACHED_NODE, owner_room.addr());
        // A marker reference without an owner.
        let (lonely, _, _, _) = marker_reference(&mut e);
        // A reference of the world space's list for this cell.
        let (from_world, _, _, _) = marker_reference(&mut e);
        let world_list = e.mem.alloc(8);
        e.mem.set_u32(world_list, from_world.addr());
        e.register_double(WORLD_SPACE_CELL_REFERENCES, move |_, _| {
            world_list.into_ret()
        });

        set_references(&mut e, cell, &[plain, actor, hidden, linked, lonely]);
        e.call_log = Some(vec![]);
        e.call(0x0054_4ce0, &args![cell]);
        let log = take_log(&mut e);

        // The loaded data was built (0x64 bytes) and stored.
        let loaded = e.get(cell, TESObjectCELL::pLoadedData);
        assert_ne!(loaded.addr(), 0);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x64]]);
        // Scripted and activating references: the plain one only; the actor
        // is not scripted and the hidden one (flag 0x20 and no loader flag)
        // is skipped.
        assert_eq!(list_items(&e, loaded.addr() + 0x4c), vec![plain.addr()]);
        assert_eq!(list_items(&e, loaded.addr() + 0x54), vec![plain.addr()]);
        // The references are handled under the lock.
        let order = sequence(&log);
        let enter = order.iter().position(|a| *a == LOCK_ENTER).unwrap();
        let leave = order.iter().position(|a| *a == LOCK_LEAVE).unwrap();
        assert!(enter < leave);
        let stored: Vec<Vec<u32>> = calls_to(&log, MAP_SET_AT_MULTI_BOUND);
        let stored_refs: Vec<u32> = stored.iter().map(|call| call[1]).collect();
        // `linked` and `lonely` in the first loop, `linked` again for the
        // queue (its node is not cached), `from_world` last.
        assert_eq!(
            stored_refs,
            vec![
                linked.addr(),
                lonely.addr(),
                linked.addr(),
                from_world.addr()
            ]
        );
        assert!(stored.iter().all(|call| call[0] == loaded.addr() + 0x3c));
        // After the lock: the owner's node is told about the reference's node
        // (virtual slot 0xdc with the node and 1).
        let linking = calls_to(&log, wide(0xdc));
        assert_eq!(
            linking,
            vec![vec![owner_room.addr(), linked_room.addr(), 1]]
        );
        let linking_at = order.iter().position(|a| *a == wide(0xdc)).unwrap();
        assert!(leave < linking_at);

        // With the loader's flag 2 the hidden reference is handled too, and
        // loaded data that exists is kept.
        e.mem.set_u32(loader + 0x244, 2);
        let again = e.get(cell, TESObjectCELL::pLoadedData);
        e.call_log = Some(vec![]);
        e.call(0x0054_4ce0, &args![cell]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, ALLOCATE).is_empty());
        assert_eq!(e.get(cell, TESObjectCELL::pLoadedData), again);
        let activating = list_items(&e, again.addr() + 0x54);
        assert_eq!(activating.len(), 3);
        assert!(activating.contains(&hidden.addr()));
    }

    #[test]
    fn releasing_the_loaded_data_clears_what_it_holds() {
        let mut e = wide_engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        // Two multibound entries: one whose node's data has two owners left
        // (cleared), one with three (kept), then entries without a key or a
        // node.
        let (data_a, data_b) = (wide_object(&mut e), wide_object(&mut e));
        word(&mut e, data_a, 4, 2);
        word(&mut e, data_b, 4, 3);
        let (node_a, node_b, node_c) = (
            wide_object(&mut e),
            wide_object(&mut e),
            wide_object(&mut e),
        );
        word(&mut e, node_a, MULTI_BOUND_DATA, data_a.addr());
        word(&mut e, node_b, MULTI_BOUND_DATA, data_b.addr());
        word(&mut e, node_c, MULTI_BOUND_DATA, data_a.addr());
        let entries = std::rc::Rc::new(RefCell::new(VecDeque::from(vec![
            (0xa0u32, node_a.addr()),
            (0xb0, node_b.addr()),
            (0, node_c.addr()),
            (0xd0, 0),
        ])));
        returns(&mut e, MAP_FIRST_POSITION, 1);
        let queue = entries.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let (key, node) = queue.borrow_mut().pop_front().unwrap();
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], node);
            e.mem.set_u32(a[1], u32::from(!queue.borrow().is_empty()));
            Ret::default()
        });
        e.register(WORD_AT_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        // The cell has a 3D node whose owner removes it.
        let node_3d = wide_object(&mut e);
        let owner = wide_object(&mut e);
        e.mem.set_u32(loaded.addr(), node_3d.addr());
        returns(&mut e, WORD_AT_18, owner.addr());
        e.call_log = Some(vec![]);
        e.call(0x0054_5030, &args![cell]);
        let log = take_log(&mut e);

        assert_eq!(
            calls_to(&log, LIST_CLEAR),
            vec![vec![loaded.addr() + 0x4c], vec![loaded.addr() + 0x54]]
        );
        // Only the entry with two owners left is cleared.
        assert_eq!(
            calls_to(&log, REFERENCE_CLEAR_MULTI_BOUND),
            vec![vec![0xa0]]
        );
        assert_eq!(calls_to(&log, SLOT_RELEASE).len(), 5);
        // The 3D node: the detach work, then the owner's slot 0xe8.
        assert_eq!(
            calls_to(&log, CELL_DETACH_LOADED_3D),
            vec![vec![cell.addr()]]
        );
        assert_eq!(
            calls_to(&log, wide(0xe8)),
            vec![vec![owner.addr(), node_3d.addr()]]
        );
        // The loaded data is destroyed and freed.
        assert_eq!(calls_to(&log, DEALLOCATE), vec![vec![loaded.addr()]]);
        assert_eq!(e.get(cell, TESObjectCELL::pLoadedData), Ptr::NULL);

        // A cell without 3D node: no detach. Without loaded data: nothing.
        let (cell, loaded) = loaded_cell(&mut e, false);
        returns(&mut e, MAP_FIRST_POSITION, 0);
        e.call_log = Some(vec![]);
        e.call(0x0054_5030, &args![cell]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, CELL_DETACH_LOADED_3D).is_empty());
        assert_eq!(calls_to(&log, DEALLOCATE), vec![vec![loaded.addr()]]);
        e.call_log = Some(vec![]);
        e.call(0x0054_5030, &args![cell]);
        assert_eq!(take_log(&mut e).len(), 1);
    }

    #[test]
    fn the_loaded_data_destructor_frees_only_on_request() {
        let mut e = wide_engine();
        let block = e.mem.alloc(LoadedCellData::SIZE);
        e.call_log = Some(vec![]);
        let result = e.call(0x0054_51d0, &args![Ptr::<()>::new(block), 0u32]);
        assert_eq!(result.u32(), block);
        let log = take_log(&mut e);
        assert!(calls_to(&log, DEALLOCATE).is_empty());
        assert_eq!(calls_to(&log, SLOT_RELEASE), vec![vec![block]]);
        e.call_log = Some(vec![]);
        // Bit 0 of the flags frees the block, the other bits do not.
        e.call(0x0054_51d0, &args![Ptr::<()>::new(block), 3u32]);
        assert_eq!(calls_to(&take_log(&mut e), DEALLOCATE), vec![vec![block]]);
        e.call_log = Some(vec![]);
        e.call(0x0054_51d0, &args![Ptr::<()>::new(block), 2u32]);
        assert!(calls_to(&take_log(&mut e), DEALLOCATE).is_empty());
    }

    #[test]
    fn the_loaded_data_members_are_destroyed_in_reverse_order() {
        let mut e = wide_engine();
        let block = e.mem.alloc(LoadedCellData::SIZE);
        e.call_log = Some(vec![]);
        e.call(0x0054_5200, &args![Ptr::<()>::new(block)]);
        let log = take_log(&mut e);
        assert_eq!(
            log.iter().skip(1).cloned().collect::<Vec<_>>(),
            vec![
                (LIST_DESTRUCT, vec![block + 0x5c]),
                (LIST_DESTRUCT, vec![block + 0x54]),
                (LIST_DESTRUCT, vec![block + 0x4c]),
                (MAP_DESTRUCT_MULTI_BOUND, vec![block + 0x3c]),
                (MAP_DESTRUCT_REFERENCE_NODE, vec![block + 0x2c]),
                (MAP_DESTRUCT_FORM_REFERENCE, vec![block + 0x1c]),
                (MAP_DESTRUCT_REFERENCE_NODE, vec![block + 0x0c]),
                (LIST_DESTRUCT, vec![block + 0x04]),
                (SLOT_RELEASE, vec![block]),
            ]
        );
    }

    #[test]
    fn emittance_references_are_mapped_and_large_ones_listed() {
        let mut e = wide_engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        let reference = wide_object(&mut e);
        let node = wide_object(&mut e);
        let form = object(&mut e);
        word(&mut e, reference, BASE_FORM, form.addr());
        answer(&mut e, node, 0xc, 0x77);
        returns(&mut e, FORM_GET_BOUND_SIZE, 0);
        let bound = |e: &mut Engine, size: f64| {
            e.register_double(FORM_GET_BOUND_SIZE, move |_, _| size.into_ret());
        };

        // No node: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0054_52c0, &args![cell, reference]);
        assert_eq!(take_log(&mut e).len(), 2);
        // A node: the map gets the value of the node's slot 0xc.
        answer(&mut e, reference, 0x1d0, node.addr());
        bound(&mut e, 3000.0);
        e.call_log = Some(vec![]);
        e.call(0x0054_52c0, &args![cell, reference]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, MAP_SET_AT),
            vec![vec![loaded.addr() + 0x0c, reference.addr(), 0x77]]
        );
        // Not above 3000.0: not a large reference.
        assert_eq!(list_items(&e, loaded.addr() + 4), Vec::<u32>::new());
        // Above 3000.0: listed in LargeAnimatedRefs.
        bound(&mut e, 3000.5);
        e.call(0x0054_52c0, &args![cell, reference]);
        assert_eq!(list_items(&e, loaded.addr() + 4), vec![reference.addr()]);
        // A NaN bound size is not above it either.
        e.mem.set_u32(loaded.addr() + 4, 0);
        bound(&mut e, f64::NAN);
        e.call(0x0054_52c0, &args![cell, reference]);
        assert_eq!(list_items(&e, loaded.addr() + 4), Vec::<u32>::new());
        // Without loaded data or a reference: nothing.
        e.set(cell, TESObjectCELL::pLoadedData, Ptr::NULL);
        e.call(0x0054_52c0, &args![cell, reference]);
        let (cell, _) = loaded_cell(&mut e, false);
        e.call(0x0054_52c0, &args![cell, Ptr::<()>::NULL]);
    }

    #[test]
    fn removing_an_emittance_reference_clears_the_map_and_the_list() {
        let mut e = wide_engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        let reference = wide_object(&mut e);
        let removed = std::rc::Rc::new(RefCell::new(vec![]));
        let seen = removed.clone();
        e.register_double(LIST_REMOVE_ITEM, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0054_5360, &args![cell, reference]);
        assert_eq!(
            calls_to(&take_log(&mut e), MAP_REMOVE_AT),
            vec![vec![loaded.addr() + 0x0c, reference.addr()]]
        );
        assert_eq!(
            *removed.borrow(),
            vec![(loaded.addr() + 4, reference.addr())]
        );
        // Without loaded data nothing is called.
        e.set(cell, TESObjectCELL::pLoadedData, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0054_5360, &args![cell, reference]);
        assert_eq!(take_log(&mut e).len(), 1);
    }

    #[test]
    fn lights_and_other_sources_are_registered_differently() {
        let mut e = wide_engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        let reference = wide_object(&mut e);
        let node = wide_object(&mut e);
        let form = object(&mut e);
        word(&mut e, reference, BASE_FORM, form.addr());
        answer(&mut e, node, 0xc, 0x77);

        // No node: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0054_53b0, &args![cell, reference]);
        assert_eq!(sequence(&take_log(&mut e)), vec![wide(0x1d0)]);

        // A light (form type 0x1e): mapped in EmittanceLightRefMap.
        answer(&mut e, reference, 0x1d0, node.addr());
        e.mem.set_u8(form.addr() + 4, 0x1e);
        e.call_log = Some(vec![]);
        e.call(0x0054_53b0, &args![cell, reference]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, MAP_SET_AT),
            vec![vec![loaded.addr() + 0x2c, reference.addr(), 0x77]]
        );
        assert!(calls_to(&log, CLONE_MATERIAL_PROPERTY).is_empty());

        // Another form: materials cloned, properties updated, and the
        // source (virtual slot 0xc0 gives a value) mapped to the reference.
        e.mem.set_u8(form.addr() + 4, 0x10);
        let source = wide_object(&mut e);
        answer(&mut e, source, 0xc0, 0x99);
        word(&mut e, reference, EMITTANCE_SOURCE, source.addr());
        e.call_log = Some(vec![]);
        e.call(0x0054_53b0, &args![cell, reference]);
        let log = take_log(&mut e);
        assert_eq!(
            sequence(&log)
                .into_iter()
                .filter(|a| {
                    [
                        CLONE_MATERIAL_PROPERTY,
                        NODE_UPDATE_PROPERTIES,
                        NODE_GET_PROPERTY,
                        SHADER_APPLY_PROPERTY,
                        MAP_SET_AT,
                    ]
                    .contains(a)
                })
                .collect::<Vec<_>>(),
            vec![
                CLONE_MATERIAL_PROPERTY,
                NODE_UPDATE_PROPERTIES,
                NODE_GET_PROPERTY,
                SHADER_APPLY_PROPERTY,
                MAP_SET_AT
            ]
        );
        assert_eq!(
            calls_to(&log, CLONE_MATERIAL_PROPERTY),
            vec![vec![node.addr()]]
        );
        assert_eq!(
            calls_to(&log, NODE_GET_PROPERTY),
            vec![vec![node.addr(), 2]]
        );
        assert_eq!(
            calls_to(&log, SHADER_APPLY_PROPERTY),
            vec![vec![node.addr(), 0x99]]
        );
        assert_eq!(
            calls_to(&log, MAP_SET_AT),
            vec![vec![loaded.addr() + 0x1c, source.addr(), reference.addr()]]
        );

        // A source whose slot 0xc0 gives nothing: only the clone and the
        // update happen.
        answer(&mut e, source, 0xc0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0054_53b0, &args![cell, reference]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, CLONE_MATERIAL_PROPERTY).len(), 1);
        assert!(calls_to(&log, MAP_SET_AT).is_empty());

        // Without a source of its own, the singleton's word at +0x760.
        word(&mut e, reference, EMITTANCE_SOURCE, 0);
        let singleton = e.mem.alloc(0x768);
        e.mem.set_u32(singleton + 0x760, source.addr());
        e.set_global(EMITTANCE_FALLBACK_OWNER_POINTER, singleton);
        answer(&mut e, source, 0xc0, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x0054_53b0, &args![cell, reference]);
        assert_eq!(
            calls_to(&take_log(&mut e), MAP_SET_AT),
            vec![vec![loaded.addr() + 0x1c, source.addr(), reference.addr()]]
        );
        // With no source at all nothing is mapped.
        e.mem.set_u32(singleton + 0x760, 0);
        e.call_log = Some(vec![]);
        e.call(0x0054_53b0, &args![cell, reference]);
        assert!(calls_to(&take_log(&mut e), MAP_SET_AT).is_empty());
    }

    #[test]
    fn a_word_at_0x760_is_read_from_the_object() {
        let mut e = engine();
        let block = e.mem.alloc(0x768);
        e.mem.set_u32(block + 0x760, 0xabcd);
        assert_eq!(
            e.call(0x0054_54d0, &args![Ptr::<()>::new(block)]).u32(),
            0xabcd
        );
    }

    #[test]
    fn emittance_sources_are_removed_by_kind() {
        let mut e = wide_engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        let reference = wide_object(&mut e);
        let form = object(&mut e);
        word(&mut e, reference, BASE_FORM, form.addr());
        // A light: removed from EmittanceLightRefMap by the reference.
        e.mem.set_u8(form.addr() + 4, 0x1e);
        e.call_log = Some(vec![]);
        e.call(0x0054_54f0, &args![cell, reference]);
        assert_eq!(
            calls_to(&take_log(&mut e), MAP_REMOVE_AT),
            vec![vec![loaded.addr() + 0x2c, reference.addr()]]
        );
        // Another form without a source: nothing removed.
        e.mem.set_u8(form.addr() + 4, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0054_54f0, &args![cell, reference]);
        assert!(calls_to(&take_log(&mut e), MAP_REMOVE_AT).is_empty());
        // With a source: removed from EmittanceSourceRefMap by the source.
        word(&mut e, reference, EMITTANCE_SOURCE, 0x4321);
        e.call_log = Some(vec![]);
        e.call(0x0054_54f0, &args![cell, reference]);
        assert_eq!(
            calls_to(&take_log(&mut e), MAP_REMOVE_AT),
            vec![vec![loaded.addr() + 0x1c, 0x4321]]
        );
        // Without loaded data nothing is called.
        e.set(cell, TESObjectCELL::pLoadedData, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0054_54f0, &args![cell, reference]);
        assert_eq!(take_log(&mut e).len(), 1);
    }

    /// Checks an add / remove pair on the list at `offset` of the loaded data.
    fn check_list_pair(add: u32, remove: u32, offset: u32) {
        let mut e = wide_engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        let (first, second) = (wide_object(&mut e), wide_object(&mut e));
        e.call(add, &args![cell, first]);
        e.call(add, &args![cell, second]);
        assert_eq!(
            list_items(&e, loaded.addr() + offset),
            vec![second.addr(), first.addr()]
        );
        // Removal passes the address of a slot holding the reference.
        let removed = std::rc::Rc::new(RefCell::new(vec![]));
        let seen = removed.clone();
        e.register_double(LIST_REMOVE_ITEM, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(remove, &args![cell, first]);
        assert_eq!(
            *removed.borrow(),
            vec![(loaded.addr() + offset, first.addr())]
        );
        // Without loaded data neither does anything.
        e.set(cell, TESObjectCELL::pLoadedData, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(add, &args![cell, first]);
        e.call(remove, &args![cell, first]);
        assert_eq!(take_log(&mut e).len(), 2);
    }

    #[test]
    fn scripted_references_are_added_to_their_list() {
        check_list_pair(0x0054_5560, 0x0054_5590, 0x4c);
    }

    #[test]
    fn water_references_are_added_to_their_list() {
        check_list_pair(0x0054_56b0, 0x0054_56e0, 0x5c);
    }

    #[test]
    fn activating_references_are_linked_after_the_last_one() {
        let mut e = wide_engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        let (first, second, third) = (
            wide_object(&mut e),
            wide_object(&mut e),
            wide_object(&mut e),
        );
        let inserted = std::rc::Rc::new(RefCell::new(vec![]));
        let seen = inserted.clone();
        e.register_double(LIST_INSERT_AFTER, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        // An empty list: pushed at the front, after the change flag call.
        e.call_log = Some(vec![]);
        e.call(0x0054_55c0, &args![cell, first]);
        let log = take_log(&mut e);
        assert_eq!(
            sequence(&log),
            vec![LIST_IS_END, wide(0x48), LIST_PUSH_FRONT]
        );
        assert_eq!(
            calls_to(&log, wide(0x48)),
            vec![vec![first.addr(), 0x0400_0000]]
        );
        assert_eq!(list_items(&e, loaded.addr() + 0x54), vec![first.addr()]);
        // A reference already in the list changes nothing.
        e.call_log = Some(vec![]);
        e.call(0x0054_55c0, &args![cell, first]);
        assert_eq!(
            sequence(&take_log(&mut e)),
            vec![LIST_IS_END, LIST_ITEM_ADDRESS]
        );
        // Another one is inserted after the last node.
        let head = loaded.addr() + 0x54;
        e.mem.set_u32(head + 4, 0); // still a single node
        e.call(0x0054_55c0, &args![cell, second]);
        assert_eq!(*inserted.borrow(), vec![(head, second.addr())]);
        // With two nodes it is the second node that is passed.
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, third.addr());
        e.mem.set_u32(head + 4, node);
        inserted.borrow_mut().clear();
        e.call(0x0054_55c0, &args![cell, second]);
        assert_eq!(*inserted.borrow(), vec![(node, second.addr())]);
        // Without loaded data nothing happens.
        e.set(cell, TESObjectCELL::pLoadedData, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0054_55c0, &args![cell, first]);
        assert_eq!(take_log(&mut e).len(), 1);
    }

    #[test]
    fn removing_an_activating_reference_always_calls_slot_0x4c() {
        let mut e = wide_engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        let reference = wide_object(&mut e);
        let removed = std::rc::Rc::new(RefCell::new(vec![]));
        let seen = removed.clone();
        e.register_double(LIST_REMOVE_ITEM, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0054_5670, &args![cell, reference]);
        let log = take_log(&mut e);
        assert_eq!(
            *removed.borrow(),
            vec![(loaded.addr() + 0x54, reference.addr())]
        );
        assert_eq!(
            calls_to(&log, wide(0x4c)),
            vec![vec![reference.addr(), 0x0400_0000]]
        );
        // Without loaded data only the virtual call is made.
        e.set(cell, TESObjectCELL::pLoadedData, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0054_5670, &args![cell, reference]);
        assert_eq!(sequence(&take_log(&mut e)), vec![wide(0x4c)]);
    }

    #[test]
    fn the_water_list_is_returned_when_there_is_loaded_data() {
        let mut e = engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        assert_eq!(
            e.call(0x0054_5710, &args![cell]).u32(),
            loaded.addr() + 0x5c
        );
        e.set(cell, TESObjectCELL::pLoadedData, Ptr::NULL);
        assert_eq!(e.call(0x0054_5710, &args![cell]).u32(), 0);
    }

    #[test]
    fn marker_references_get_their_room_registered() {
        let mut e = wide_engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        let (reference, room, data, primitive) = marker_reference(&mut e);
        answer(&mut e, reference, 0x1f4, 0x6006);
        let build = |e: &mut Engine, reference: Ptr| -> (Ptr, Vec<(u32, Vec<u32>)>) {
            e.call_log = Some(vec![]);
            let room = e.call(0x0054_5740, &args![cell, reference]).ptr::<()>();
            (room, take_log(e))
        };

        // A room, its data and a primitive: the primitive takes the
        // reference's transform, the room goes into the map, slot 0xbc runs.
        let (result, log) = build(&mut e, reference);
        assert_eq!(result, room);
        assert_eq!(
            calls_to(&log, wide(0xb8)),
            vec![vec![primitive.addr(), 0x6006]]
        );
        assert!(calls_to(&log, REFERENCE_GET_ORIENTATION).is_empty());
        assert_eq!(
            calls_to(&log, MULTI_BOUND_SET_DATA),
            vec![vec![room.addr(), data.addr()]]
        );
        assert_eq!(
            calls_to(&log, MAP_SET_AT_MULTI_BOUND),
            vec![vec![loaded.addr() + 0x3c, reference.addr(), room.addr()]]
        );
        assert_eq!(calls_to(&log, wide(0xbc)), vec![vec![room.addr()]]);
        assert_eq!(sequence(&log).last(), Some(&wide(0xbc)));

        // A primitive of type 2 also gets the reference's orientation.
        answer(&mut e, primitive, 0x8c, 2);
        let (_, log) = build(&mut e, reference);
        let orientation = calls_to(&log, REFERENCE_GET_ORIENTATION);
        assert_eq!(orientation.len(), 1);
        assert_eq!(orientation[0][0], reference.addr());
        assert_eq!(
            calls_to(&log, PRIMITIVE_SET_ROTATION),
            vec![vec![primitive.addr(), orientation[0][1]]]
        );
        answer(&mut e, primitive, 0x8c, 0);

        // No primitive in the data: built from the extra data's shape at the
        // reference's three words, then stored in the data.
        word(&mut e, data, PRIMITIVE, 0);
        let shape = wide_object(&mut e);
        let made = wide_object(&mut e);
        word(&mut e, reference, SHAPE, shape.addr());
        e.register(REFERENCE_ROTATION, |_, a| (a[0] + 0x20).into_ret());
        for (i, value) in [11u32, 22, 33].into_iter().enumerate() {
            word(&mut e, reference, 0x20 + 4 * i as u32, value);
        }
        let seen = std::rc::Rc::new(RefCell::new(vec![]));
        let copy = seen.clone();
        e.register_double(wide(0x14), move |e, a| {
            *copy.borrow_mut() = (0..3).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            made.addr().into_ret()
        });
        let (_, log) = build(&mut e, reference);
        assert_eq!(*seen.borrow(), vec![11, 22, 33]);
        assert_eq!(
            calls_to(&log, MULTI_BOUND_SET_PRIMITIVE),
            vec![vec![data.addr(), made.addr()]]
        );
        assert_eq!(calls_to(&log, wide(0xb8)), vec![vec![made.addr(), 0x6006]]);
        // No shape either: no primitive work, but the room is still
        // returned and notified; nothing goes into the map.
        word(&mut e, reference, SHAPE, 0);
        let (result, log) = build(&mut e, reference);
        assert_eq!(result, room);
        assert!(calls_to(&log, MAP_SET_AT_MULTI_BOUND).is_empty());
        assert_eq!(calls_to(&log, wide(0xbc)), vec![vec![room.addr()]]);
    }

    #[test]
    fn marker_references_without_a_room_get_one_by_form() {
        let mut e = wide_engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        let reference = wide_object(&mut e);
        let room_from_reference = wide_object(&mut e);
        let data = wide_object(&mut e);
        let primitive = wide_object(&mut e);
        word(&mut e, reference, IS_MARKER, 1);
        word(&mut e, room_from_reference, MULTI_BOUND_DATA, data.addr());
        word(&mut e, data, PRIMITIVE, primitive.addr());
        word(&mut e, reference, MULTI_BOUND_DATA, data.addr());
        let marker_form = 0x5151u32;
        e.set_global(MARKER_FORM_SECOND, marker_form);

        // The second marker form takes the reference's multibound room; none
        // ends the function.
        word(&mut e, reference, BASE_FORM, marker_form);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0054_5740, &args![cell, reference]).u32(), 0);
        assert!(calls_to(&take_log(&mut e), MAP_SET_AT_MULTI_BOUND).is_empty());
        word(
            &mut e,
            reference,
            MULTI_BOUND_ROOM,
            room_from_reference.addr(),
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x0054_5740, &args![cell, reference]).ptr::<()>();
        assert_eq!(result, room_from_reference);
        let log = take_log(&mut e);
        assert!(calls_to(&log, NODE_ALLOCATE).is_empty());
        assert_eq!(
            calls_to(&log, MAP_SET_AT_MULTI_BOUND),
            vec![vec![
                loaded.addr() + 0x3c,
                reference.addr(),
                room_from_reference.addr()
            ]]
        );

        // Any other form gets a new node (0xb4 bytes) and the reference's
        // multibound data.
        word(&mut e, reference, BASE_FORM, 0x7777);
        e.call_log = Some(vec![]);
        let result = e.call(0x0054_5740, &args![cell, reference]).ptr::<()>();
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, NODE_ALLOCATE), vec![vec![0xb4]]);
        let built = calls_to(&log, MULTI_BOUND_NODE_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][0], result.addr());
        assert_eq!(
            calls_to(&log, MULTI_BOUND_SET_DATA),
            vec![
                vec![result.addr(), data.addr()],
                vec![result.addr(), data.addr()]
            ]
        );
        assert_eq!(calls_to(&log, wide(0xbc)), vec![vec![result.addr()]]);
    }

    #[test]
    fn marker_registration_needs_loaded_data_a_reference_and_a_marker_form() {
        let mut e = wide_engine();
        let (cell, _) = loaded_cell(&mut e, false);
        let (reference, _, _, _) = marker_reference(&mut e);
        // Not a marker.
        word(&mut e, reference, IS_MARKER, 0);
        assert_eq!(e.call(0x0054_5740, &args![cell, reference]).u32(), 0);
        assert_eq!(e.call(0x0054_5960, &args![cell, reference]).u32(), 0);
        // No reference.
        assert_eq!(e.call(0x0054_5740, &args![cell, Ptr::<()>::NULL]).u32(), 0);
        assert_eq!(e.call(0x0054_5960, &args![cell, Ptr::<()>::NULL]).u32(), 0);
        // No loaded data.
        word(&mut e, reference, IS_MARKER, 1);
        e.set(cell, TESObjectCELL::pLoadedData, Ptr::NULL);
        assert_eq!(e.call(0x0054_5740, &args![cell, reference]).u32(), 0);
        assert_eq!(e.call(0x0054_5960, &args![cell, reference]).u32(), 0);
    }

    #[test]
    fn the_node_of_a_marker_reference_is_cached_or_built() {
        let mut e = wide_engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        let (reference, room, _, _) = marker_reference(&mut e);
        let cached = wide_object(&mut e);

        // In the map: returned as it is, nothing built.
        word(&mut e, reference, CACHED_NODE, cached.addr());
        e.call_log = Some(vec![]);
        let result = e.call(0x0054_5960, &args![cell, reference]);
        assert_eq!(result.u32(), cached.addr());
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, MAP_GET_AT)[0][..2].to_vec(),
            vec![loaded.addr() + 0x3c, reference.addr()]
        );
        assert!(calls_to(&log, MULTI_BOUND_SET_DATA).is_empty());
        assert_eq!(calls_to(&log, SLOT_RELEASE).len(), 1);
        assert_eq!(sequence(&log).last(), Some(&SLOT_RELEASE));

        // Not in the map: built by `fn_00545740` and kept in the slot.
        word(&mut e, reference, CACHED_NODE, 0);
        e.call_log = Some(vec![]);
        let result = e.call(0x0054_5960, &args![cell, reference]);
        assert_eq!(result.u32(), room.addr());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, MAP_SET_AT_MULTI_BOUND).len(), 1);
        assert_eq!(calls_to(&log, SLOT_ASSIGN)[0][1], room.addr());
        assert_eq!(calls_to(&log, SLOT_RELEASE).len(), 1);

        // The map answers but the slot is null: built as well.
        e.register(MAP_GET_AT, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let result = e.call(0x0054_5960, &args![cell, reference]);
        assert_eq!(result.u32(), room.addr());
        assert_eq!(calls_to(&take_log(&mut e), SLOT_ASSIGN).len(), 1);
    }

    // ---- tests of the 3D, ownership, land, region and water functions ----

    const ACTOR_VTABLE: u32 = 0x0130_8000;
    /// Words of the test cells and objects of this section.
    const KIND_WORD: u32 = 0x1f0;
    const FACTION_RANK_WORD: u32 = 0x1f4;
    const CANDIDATE_WORD: u32 = 0xe4;
    const CELL_WORD_A: u32 = 0xe0;
    const CELL_WORD_B: u32 = 0xe4;
    const CELL_WORD_C: u32 = 0xe8;

    fn actor_slot(slot: u32) -> u32 {
        0x7300_0000 + slot
    }

    /// The wide engine plus the pages the globals of this section live in,
    /// the actors' vtable and doubles for the accessors that only read a
    /// word.
    fn scene_engine() -> Engine {
        let mut e = wide_engine();
        // `wide_engine` stands in for the cell's 3D detach (`00545c10`),
        // which this section translates: use the translation.
        for (address, function) in funcs() {
            if address == CELL_DETACH_LOADED_3D {
                e.register(address, function);
            }
        }
        for page in [0x0101_5000, 0x0101_7000, 0x0102_3000, 0x011c_9000] {
            e.map(page, 0x1000);
        }
        e.set_global(LOWEST_FLOAT, f32::MIN);
        e.set_global(UNSET_WATER_HEIGHT_DOUBLE, f64::from(f32::MAX));
        e.set_global(WATER_HEIGHT_TOLERANCE, 0.001f32);
        e.set_global(DEFAULT_WATER_HEIGHT, f32::MAX);
        // The actors' vtable: slots 0x218 and 0x304 answer with the words of
        // the same offset.
        let slots: Vec<u32> = (0..=0x304 / 4).map(|i| actor_slot(4 * i)).collect();
        e.put_vtable(ACTOR_VTABLE, &slots);
        for slot in [0x218, 0x304] {
            e.register_double(actor_slot(slot), move |e, a| {
                e.mem.u32(a[0] + slot).into_ret()
            });
        }
        e.register(CELL_FLAG_20, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x24) & 0x20 != 0).into_ret()
        });
        e.register(CELL_FLAG_2, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x24) & 2 != 0).into_ret()
        });
        e.register(CELL_FLAG_80, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x24) & 0x80 != 0).into_ret()
        });
        e.register(WORD_AT_18, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e
    }

    /// An actor-like object (0x400 bytes) with the actor vtable.
    fn actor(e: &mut Engine) -> Ptr {
        let block = e.mem.alloc(0x400);
        e.mem.set_u32(block, ACTOR_VTABLE);
        Ptr::new(block)
    }

    /// Doubles of the extra data accessors the ownership functions use; the
    /// cell's extra data list is at `+0x28`, so they read words of the cell.
    fn ownership_doubles(e: &mut Engine) {
        e.register(EXTRA_LIST_GET_OWNER, |e, a| {
            e.mem.u32(a[0] - 0x28 + CELL_WORD_A).into_ret()
        });
        e.register(EXTRA_LIST_GET_TYPE_74, |e, a| {
            e.mem.u32(a[0] - 0x28 + CELL_WORD_B).into_ret()
        });
        e.register(EXTRA_LIST_GET_RANK, |e, a| {
            e.mem.u32(a[0] - 0x28 + CELL_WORD_C).into_ret()
        });
        e.register(EXTRA_LIST_GET_GLOBAL, |e, a| {
            e.mem.u32(a[0] - 0x28 + 0xec).into_ret()
        });
        e.register(WORLD_SPACE_OWNER_FORM, |e, a| {
            e.mem.u32(a[0] + CANDIDATE_WORD).into_ret()
        });
        // A cast succeeds when the object's `KIND_WORD` is the target.
        e.register(RT_DYNAMIC_CAST, |e, a| {
            if e.mem.u32(a[0] + KIND_WORD) == a[3] {
                a[0].into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(ACTOR_BASE_GET_FACTION_RANK, |e, a| {
            e.mem.u32(a[0] - 0x30 + FACTION_RANK_WORD).into_ret()
        });
    }

    #[test]
    fn multibound_nodes_without_a_parent_are_attached_to_the_cells_child() {
        let mut e = scene_engine();
        let (cell, _) = loaded_cell(&mut e, false);
        let parent = wide_object(&mut e);
        let parent_address = parent.addr();
        e.register_double(CELL_CHILD_NODE, move |_, a| {
            assert_eq!(a[1], 7);
            parent_address.into_ret()
        });
        let (free, taken, other_free) = (
            wide_object(&mut e),
            wide_object(&mut e),
            wide_object(&mut e),
        );
        word(&mut e, taken, 0x18, 0x1234);
        let entries = Rc::new(RefCell::new(VecDeque::from(vec![
            free.addr(),
            taken.addr(),
            0,
            other_free.addr(),
        ])));
        returns(&mut e, MAP_FIRST_POSITION, 1);
        let queue = entries.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let node = queue.borrow_mut().pop_front().unwrap();
            e.mem.set_u32(a[3], node);
            e.mem.set_u32(a[1], u32::from(!queue.borrow().is_empty()));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0054_5a30, &args![cell]);
        let log = take_log(&mut e);
        // Only the nodes without a parent are attached, with the flag 1.
        assert_eq!(
            calls_to(&log, wide(0xdc)),
            vec![
                vec![parent.addr(), free.addr(), 1],
                vec![parent.addr(), other_free.addr(), 1]
            ]
        );
        assert_eq!(calls_to(&log, SLOT_RELEASE).len(), 4);

        // No child node, or no loaded data: nothing is walked.
        e.register(CELL_CHILD_NODE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0054_5a30, &args![cell]);
        assert!(calls_to(&take_log(&mut e), MAP_FIRST_POSITION).is_empty());
        e.register_double(CELL_CHILD_NODE, move |_, _| parent_address.into_ret());
        e.set(cell, TESObjectCELL::pLoadedData, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0054_5a30, &args![cell]);
        assert!(calls_to(&take_log(&mut e), MAP_FIRST_POSITION).is_empty());
    }

    #[test]
    fn the_portal_graph_is_built_on_first_use() {
        let mut e = scene_engine();
        let slot = TESObjectCELL::spPortalGraph.off;
        e.register(PORTAL_GRAPH_CONSTRUCT, |_, a| a[0].into_ret());
        let graph = wide_object(&mut e);
        let graph_address = graph.addr();
        e.register_double(WORLD_SPACE_PORTAL_GRAPH, move |_, _| {
            graph_address.into_ret()
        });

        // An interior cell builds a 0x78-byte object.
        let interior = cell(&mut e, true);
        e.call_log = Some(vec![]);
        let built = e.call(0x0054_5b30, &args![interior]).u32();
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, NODE_ALLOCATE), vec![vec![0x78]]);
        assert_eq!(calls_to(&log, PORTAL_GRAPH_CONSTRUCT), vec![vec![built]]);
        assert_eq!(e.mem.u32(interior.addr() + slot), built);

        // Built once: the second call only reads the slot (twice).
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0054_5b30, &args![interior]).u32(), built);
        assert_eq!(take_log(&mut e).len(), 1 + 2);

        // An exterior cell takes the graph of its world space.
        let exterior = cell(&mut e, false);
        let world = wide_object(&mut e);
        e.set(exterior, TESObjectCELL::pWorldSpace, world);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0054_5b30, &args![exterior]).u32(), graph.addr());
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, WORLD_SPACE_PORTAL_GRAPH),
            vec![vec![world.addr()]]
        );
        assert_eq!(e.mem.u32(exterior.addr() + slot), graph.addr());
    }

    #[test]
    fn dropping_the_portal_graph_resets_the_current_one() {
        let mut e = scene_engine();
        let slot = TESObjectCELL::spPortalGraph.off;
        let entry = wide_object(&mut e);
        let entry_address = entry.addr();
        e.register_double(GLOBAL_TABLE_ENTRY, move |_, a| {
            assert_eq!(a[0], 0);
            entry_address.into_ret()
        });
        e.register(CURRENT_PORTAL_GRAPH, |e, a| {
            e.mem.u32(a[0] + 0x1e0).into_ret()
        });
        quiet(
            &mut e,
            &[SET_CURRENT_PORTAL_GRAPH, WORLD_SPACE_RELEASE_PORTAL_GRAPH],
        );
        let graph = wide_object(&mut e);

        // No graph: only the slot is read.
        let interior = cell(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0054_5c10, &args![interior]);
        assert_eq!(take_log(&mut e).len(), 1 + 1);

        // The cell's graph is the current one: the current one is reset.
        e.mem.set_u32(interior.addr() + slot, graph.addr());
        e.mem.set_u32(entry.addr() + 0x1e0, graph.addr());
        e.call_log = Some(vec![]);
        e.call(0x0054_5c10, &args![interior]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, SET_CURRENT_PORTAL_GRAPH),
            vec![vec![entry.addr(), 0]]
        );
        assert_eq!(e.mem.u32(interior.addr() + slot), 0);
        assert!(calls_to(&log, WORLD_SPACE_RELEASE_PORTAL_GRAPH).is_empty());

        // Another graph is current: left alone.
        e.mem.set_u32(interior.addr() + slot, graph.addr());
        e.mem.set_u32(entry.addr() + 0x1e0, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x0054_5c10, &args![interior]);
        assert!(calls_to(&take_log(&mut e), SET_CURRENT_PORTAL_GRAPH).is_empty());

        // An exterior cell releases its world space's graph.
        let exterior = cell(&mut e, false);
        let world = wide_object(&mut e);
        e.set(exterior, TESObjectCELL::pWorldSpace, world);
        e.mem.set_u32(exterior.addr() + slot, graph.addr());
        e.call_log = Some(vec![]);
        e.call(0x0054_5c10, &args![exterior]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, GLOBAL_TABLE_ENTRY).is_empty());
        assert_eq!(
            calls_to(&log, WORLD_SPACE_RELEASE_PORTAL_GRAPH),
            vec![vec![world.addr()]]
        );
        assert_eq!(e.mem.u32(exterior.addr() + slot), 0);
    }

    #[test]
    fn the_cells_3d_is_the_first_slot_of_its_loaded_data() {
        let mut e = scene_engine();
        let (cell, loaded) = loaded_cell(&mut e, false);
        assert_eq!(e.call(0x0054_5cb0, &args![cell]).u32(), 0);
        let node = wide_object(&mut e);
        e.mem.set_u32(loaded.addr(), node.addr());
        assert_eq!(e.call(0x0054_5cb0, &args![cell]).u32(), node.addr());
        e.set(cell, TESObjectCELL::pLoadedData, Ptr::NULL);
        assert_eq!(e.call(0x0054_5cb0, &args![cell]).u32(), 0);
    }

    /// Everything `Load3D` calls outside the file, as doubles. Returns the
    /// nodes created, in order, and the loader.
    fn load_doubles(e: &mut Engine) -> (Rc<RefCell<Vec<u32>>>, Ptr) {
        let created = Rc::new(RefCell::new(vec![]));
        let record = created.clone();
        e.register_double(NODE_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], WIDE_VTABLE);
            record.borrow_mut().push(a[0]);
            a[0].into_ret()
        });
        quiet(
            e,
            &[
                SCOPE_ENTER,
                SCOPE_LEAVE,
                CELL_SET_STATE,
                NODE_SET_TRANSLATION,
                NODE_SET_ROTATION,
                NODE_SET_FLAG_BITS,
                NODE_SET_FLAG,
                BOUND_SET_RADIUS,
                BOUND_SET_CENTER_AND_RADIUS,
                BOUND_BASE_CONSTRUCT,
                UPDATE_DATA_CONSTRUCT,
                NODE_UPDATE,
                CELL_FINISH_3D,
                SAVE_LOAD_CELL,
                LOADER_AFTER_LOAD,
                FIXED_STRING_DESTRUCT,
                NODE_SET_NAME,
            ],
        );
        for constructor in [MULTI_BOUND_CONSTRUCT, MULTI_BOUND_AABB_CONSTRUCT] {
            e.register(constructor, |_, a| a[0].into_ret());
        }
        e.register(PORTAL_GRAPH_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        // The node stores the bound it is given at `+0x20`.
        e.register(wide(0x78), |e, a| {
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        e.register(CELL_CHILD_NODE, |_, _| 0u32.into_ret());
        e.register(GAME_LOADER_FLAG_SETTER, |_, _| 0x5au32.into_ret());
        let loader = wide_object(e);
        e.set_global(GAME_LOADER_POINTER, loader.addr());
        // Flags 1, 3, 6 and 9 set.
        e.set_global(STATIC_FLAG, 0b10_0100_1010u32);
        (created, loader)
    }

    #[test]
    fn load_3d_builds_the_node_tree_of_a_cell() {
        let mut e = scene_engine();
        let (created, loader) = load_doubles(&mut e);
        let cell = cell(&mut e, true);
        e.call_log = Some(vec![]);
        let root = e.call(0x0054_5cf0, &args![cell]).u32();
        let log = take_log(&mut e);
        let nodes = created.borrow().clone();

        // Twelve nodes for the tree and one for the portal geometry.
        assert_eq!(nodes.len(), 13);
        assert_eq!(root, nodes[0]);
        let (n1, n2, n3, n4, n5) = (nodes[1], nodes[2], nodes[3], nodes[4], nodes[5]);
        let (n6, n7, n8, n9, n10, n11) =
            (nodes[6], nodes[7], nodes[8], nodes[9], nodes[10], nodes[11]);
        let geometry = nodes[12];
        assert_eq!(calls_to(&log, NODE_ALLOCATE)[0], vec![0xac]);

        // The cell state goes 2, then 3; the cell is not detached.
        assert_eq!(
            calls_to(&log, CELL_SET_STATE),
            vec![vec![cell.addr(), 2], vec![cell.addr(), 3]]
        );
        assert!(!e.get(cell, TESObjectCELL::bCellDetached));
        assert_eq!(
            calls_to(&log, SCOPE_ENTER)[0][1..],
            [0x1a, 1, SOURCE_FILE, 0xd84]
        );
        assert_eq!(calls_to(&log, SCOPE_LEAVE).len(), 1);
        // The root: zero translation, identity rotation.
        assert_eq!(
            calls_to(&log, NODE_SET_TRANSLATION),
            vec![vec![root, ZERO_VECTOR]]
        );
        assert_eq!(
            calls_to(&log, NODE_SET_ROTATION),
            vec![vec![root, IDENTITY_MATRIX]]
        );

        // The attach calls: (parent, child, 1).
        let multi_bound: Vec<u32> = calls_to(&log, MULTI_BOUND_NODE_CONSTRUCT)
            .iter()
            .map(|call| call[0])
            .collect();
        assert_eq!(multi_bound.len(), 4);
        let mut expected: Vec<Vec<u32>> = vec![
            vec![root, n1, 1],
            vec![root, n2, 1],
            vec![n2, n3, 1],
            vec![n2, n4, 1],
            vec![root, n5, 1],
        ];
        expected.extend(multi_bound.iter().map(|node| vec![n5, *node, 1]));
        expected.extend([
            vec![root, n6, 1],
            vec![root, n7, 1],
            vec![root, n8, 1],
            vec![root, n9, 1],
            vec![root, n10, 1],
            vec![root, n10, 1],
            vec![root, n11, 1],
            vec![root, n11, 1],
        ]);
        assert_eq!(calls_to(&log, wide(0xdc)), expected);
        // The markers are kept in the cell's slots.
        assert_eq!(e.get(cell, TESObjectCELL::spLightMarkerNode).addr(), n3);
        assert_eq!(e.get(cell, TESObjectCELL::spSoundMarkerNode).addr(), n4);
        // Each multibound node: field +0xB0 is 1, its data is a `BSMultiBound`
        // that holds a `BSMultiBoundAABB`.
        for node in &multi_bound {
            assert_eq!(e.mem.u32(node + 0xb0), 1);
        }
        assert_eq!(calls_to(&log, MULTI_BOUND_SET_PRIMITIVE).len(), 4);
        assert_eq!(calls_to(&log, MULTI_BOUND_SET_DATA).len(), 4);
        assert_eq!(calls_to(&log, NODE_ALLOCATE).len(), 13 + 4 * 3 + 1);
        // The loaded data (0x64 bytes) was built; each tree node got a bound
        // (0x10 bytes) through its virtual slot 0x78.
        let allocations = calls_to(&log, ALLOCATE);
        assert_eq!(allocations.iter().filter(|call| call[0] == 0x64).count(), 1);
        assert_eq!(
            allocations.iter().filter(|call| call[0] == 0x10).count(),
            12
        );
        assert_eq!(calls_to(&log, wide(0x78)).len(), 12);
        assert_eq!(calls_to(&log, BOUND_SET_RADIUS).len(), 12);

        // The flags follow `005468f0` of the indices 0, 1, 9, 10, 2, 3, then
        // 0 itself, then 5, 6, 7, 8 (bits 1, 3, 6 and 9 are set).
        let flags: Vec<u32> = calls_to(&log, NODE_SET_FLAG)
            .iter()
            .map(|call| call[1])
            .collect();
        assert_eq!(flags, vec![0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0]);
        // Two flag setters per node (and the extra one of child 1).
        let bit_calls = calls_to(&log, NODE_SET_FLAG_BITS);
        assert_eq!(bit_calls.len(), 25);
        assert_eq!(bit_calls[0], vec![root, 1, 0x2000]);
        assert_eq!(bit_calls[1], vec![root, 1, 0x800]);
        assert_eq!(bit_calls[4], vec![n2, 1, 0x800]);
        assert_eq!(bit_calls[5], vec![n2, 1, 0x2000]);
        assert_eq!(bit_calls[6], vec![n2, 1, 0x20_0000]);

        // The root is updated, stored as the loaded data's `spCell3D`, and
        // the portal graph gets its geometry.
        let loaded = e.get(cell, TESObjectCELL::pLoadedData);
        assert_eq!(e.mem.u32(loaded.addr()), root);
        let update = calls_to(&log, UPDATE_DATA_CONSTRUCT);
        assert_eq!(update[0][1..], [0, 0, 0]);
        assert_eq!(calls_to(&log, NODE_UPDATE)[0][1], update[0][0]);
        assert_eq!(calls_to(&log, NODE_UPDATE)[0][0], root);
        let graph = e.mem.u32(cell.addr() + TESObjectCELL::spPortalGraph.off);
        assert_eq!(e.mem.u32(graph + 0x4c), geometry);
        let name = calls_to(&log, FIXED_STRING_CONSTRUCT);
        assert_eq!(name[0][1], PORTAL_GEOMETRY_NAME);
        assert_eq!(
            calls_to(&log, NODE_SET_NAME),
            vec![vec![geometry, name[0][0]]]
        );
        assert_eq!(calls_to(&log, FIXED_STRING_DESTRUCT).len(), 1);

        // The loader's flag is cleared around `LoadCell` (the previous value
        // is restored) and its follow-up runs.
        assert_eq!(
            calls_to(&log, GAME_LOADER_FLAG_SETTER),
            vec![vec![loader.addr(), 0], vec![loader.addr(), 0x5a]]
        );
        assert_eq!(
            calls_to(&log, SAVE_LOAD_CELL),
            vec![vec![loader.addr(), cell.addr()]]
        );
        assert_eq!(
            calls_to(&log, LOADER_AFTER_LOAD),
            vec![vec![loader.addr(), 0]]
        );
        assert_eq!(calls_to(&log, CELL_FINISH_3D), vec![vec![cell.addr()]]);

        // A cell that has its 3D: returned as it is, the portal graph keeps
        // its geometry, nothing is built.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0054_5cf0, &args![cell]).u32(), root);
        let log = take_log(&mut e);
        assert_eq!(created.borrow().len(), 13);
        assert!(calls_to(&log, CELL_SET_STATE).is_empty());
        assert!(calls_to(&log, wide(0xdc)).is_empty());
    }

    #[test]
    fn load_3d_leaves_the_loader_alone_when_it_says_so() {
        let mut e = scene_engine();
        let (_, loader) = load_doubles(&mut e);
        // Bit 2 of the loader's word: no `LoadCell` at all.
        e.mem.set_u32(loader.addr() + 0x244, 2);
        let first = cell(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0054_5cf0, &args![first]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, GAME_LOADER_FLAG_SETTER).is_empty());
        assert!(calls_to(&log, SAVE_LOAD_CELL).is_empty());
        // Bit 0x10: `LoadCell` runs without the follow-up.
        e.mem.set_u32(loader.addr() + 0x244, 0x10);
        let second = cell(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0054_5cf0, &args![second]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, SAVE_LOAD_CELL).len(), 1);
        assert!(calls_to(&log, LOADER_AFTER_LOAD).is_empty());
        assert_eq!(calls_to(&log, GAME_LOADER_FLAG_SETTER).len(), 2);
    }

    fn flag_node(e: &mut Engine) -> (Ptr, Ptr) {
        let bound = wide_object(e);
        e.register(NODE_SET_FLAG_BITS, |e, a| {
            let flags = e.mem.u32(a[0] + 0x30);
            let flags = if a[1] & 0xff != 0 {
                flags | a[2]
            } else {
                flags & !a[2]
            };
            e.mem.set_u32(a[0] + 0x30, flags);
            Ret::default()
        });
        quiet(e, &[BOUND_SET_RADIUS]);
        let node = wide_object(e);
        e.mem.set_u32(node.addr() + 0x20, bound.addr());
        (node, bound)
    }

    #[test]
    fn flag_0x800_comes_with_a_bound_of_radius_1() {
        let mut e = scene_engine();
        let (node, bound) = flag_node(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0054_6780, &args![node, true]);
        assert_eq!(e.mem.u32(node.addr() + 0x30), 0x800);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, BOUND_SET_RADIUS),
            vec![vec![bound.addr(), 1.0f32.to_bits()]]
        );
        // Cleared: no radius.
        e.call_log = Some(vec![]);
        e.call(0x0054_6780, &args![node, false]);
        assert_eq!(e.mem.u32(node.addr() + 0x30), 0);
        assert!(calls_to(&take_log(&mut e), BOUND_SET_RADIUS).is_empty());
    }

    #[test]
    fn flag_0x2000_is_set_and_cleared() {
        let mut e = scene_engine();
        let (node, _) = flag_node(&mut e);
        e.mem.set_u32(node.addr() + 0x30, 0x1);
        e.call(0x0054_67c0, &args![node, true]);
        assert_eq!(e.mem.u32(node.addr() + 0x30), 0x2001);
        e.call(0x0054_67c0, &args![node, false]);
        assert_eq!(e.mem.u32(node.addr() + 0x30), 0x1);
    }

    #[test]
    fn flag_0x200000_is_set_and_cleared() {
        let mut e = scene_engine();
        let (node, _) = flag_node(&mut e);
        e.mem.set_u32(node.addr() + 0x30, 0x1);
        e.call(0x0054_68b0, &args![node, true]);
        assert_eq!(e.mem.u32(node.addr() + 0x30), 0x20_0001);
        e.call(0x0054_68b0, &args![node, false]);
        assert_eq!(e.mem.u32(node.addr() + 0x30), 0x1);
    }

    #[test]
    fn a_node_without_a_bound_gets_one() {
        let mut e = scene_engine();
        quiet(&mut e, &[BOUND_SET_CENTER_AND_RADIUS, BOUND_BASE_CONSTRUCT]);
        e.register(wide(0x78), |e, a| {
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        let node = wide_object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0054_67e0, &args![node]);
        let log = take_log(&mut e);
        let bound = e.mem.u32(node.addr() + 0x20);
        assert_ne!(bound, 0);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, BOUND_BASE_CONSTRUCT), vec![vec![bound]]);
        assert_eq!(calls_to(&log, wide(0x78)), vec![vec![node.addr(), bound]]);
        assert_eq!(
            calls_to(&log, BOUND_SET_CENTER_AND_RADIUS),
            vec![vec![bound, ZERO_VECTOR, 0.0f32.to_bits()]]
        );
        // A node that has a bound is left alone.
        e.call_log = Some(vec![]);
        e.call(0x0054_67e0, &args![node]);
        assert_eq!(take_log(&mut e).len(), 1);
    }

    #[test]
    fn the_bound_constructor_shape_returns_this() {
        let mut e = scene_engine();
        quiet(&mut e, &[BOUND_BASE_CONSTRUCT]);
        e.call_log = Some(vec![]);
        let result = e.call(0x0054_6890, &args![0x4444u32, 0x100u32, 0u32]);
        assert_eq!(result.u32(), 0x4444);
        assert_eq!(
            calls_to(&take_log(&mut e), BOUND_BASE_CONSTRUCT),
            vec![vec![0x4444]]
        );
    }

    #[test]
    fn the_multibound_node_field_is_written() {
        let mut e = scene_engine();
        let node = wide_object(&mut e);
        e.call(0x0054_68d0, &args![node, 0x77u32]);
        assert_eq!(e.mem.u32(node.addr() + 0xb0), 0x77);
    }

    #[test]
    fn a_static_flag_is_tested_by_index() {
        let mut e = scene_engine();
        e.set_global(STATIC_FLAG, 0b101u32);
        assert!(e.call(0x0054_68f0, &args![0u8]).bool());
        assert!(!e.call(0x0054_68f0, &args![1u8]).bool());
        assert!(e.call(0x0054_68f0, &args![2u8]).bool());
        // Only the low five bits of the index count.
        assert!(e.call(0x0054_68f0, &args![0x20u32]).bool());
        assert!(!e.call(0x0054_68f0, &args![0x21u32]).bool());
    }

    #[test]
    fn the_node_slot_at_4c_is_stored() {
        let mut e = scene_engine();
        let object = wide_object(&mut e);
        let node = wide_object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0054_6910, &args![object, node]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, SLOT_ASSIGN),
            vec![vec![object.addr() + 0x4c, node.addr()]]
        );
        assert_eq!(e.mem.u32(object.addr() + 0x4c), node.addr());
    }

    #[test]
    fn the_node_slot_at_4c_is_read() {
        let mut e = scene_engine();
        let object = wide_object(&mut e);
        assert_eq!(e.call(0x0054_6930, &args![object]).u32(), 0);
        e.mem.set_u32(object.addr() + 0x4c, 0x1234);
        assert_eq!(e.call(0x0054_6930, &args![object]).u32(), 0x1234);
    }

    #[test]
    fn the_loader_bit_0x10_is_tested() {
        let mut e = scene_engine();
        let loader = wide_object(&mut e);
        assert!(!e.call(0x0054_6950, &args![loader]).bool());
        e.mem.set_u32(loader.addr() + 0x244, 0x10);
        assert!(e.call(0x0054_6950, &args![loader]).bool());
        e.mem.set_u32(loader.addr() + 0x244, 0xffff_ffef);
        assert!(!e.call(0x0054_6950, &args![loader]).bool());
    }

    #[test]
    fn unloading_a_cell_resets_its_runtime_state() {
        let mut e = scene_engine();
        let (cell, _) = loaded_cell(&mut e, false);
        let helper = wide_object(&mut e);
        e.set_global(STATIC_OBJECT, helper.addr());
        let extra = wide_object(&mut e);
        answer(&mut e, extra, 0x94, 0x4321);
        let extra_address = extra.addr();
        e.register_double(EXTRA_LIST_GET_TYPE_1, move |_, _| extra_address.into_ret());
        quiet(
            &mut e,
            &[
                EXTRA_LIST_RESET_TYPE_1,
                STATIC_RELEASE_HELPER,
                CELL_SET_STATE,
            ],
        );
        returns(&mut e, MAP_FIRST_POSITION, 0);
        e.set(cell, TESObjectCELL::iQueuedRefCount, 5);
        e.set(cell, TESObjectCELL::iCriticalQueuedRefCount, 6);
        e.set(cell, TESObjectCELL::bCellDetached, true);
        e.call_log = Some(vec![]);
        e.call(0x0054_6970, &args![cell]);
        let log = take_log(&mut e);
        let list = cell.addr() + 0x28;
        // The object at 011ca088 is told the extra data's value (slot 0x94).
        assert_eq!(
            calls_to(&log, STATIC_RELEASE_HELPER),
            vec![vec![helper.addr(), 0x4321]]
        );
        assert_eq!(
            calls_to(&log, CELL_SET_STATE),
            vec![vec![cell.addr(), 1], vec![cell.addr(), 0]]
        );
        assert_eq!(calls_to(&log, EXTRA_LIST_RESET_TYPE_1), vec![vec![list, 0]]);
        // The loaded data was released, the counters cleared.
        assert_eq!(e.get(cell, TESObjectCELL::pLoadedData), Ptr::NULL);
        assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 0);
        assert_eq!(e.get(cell, TESObjectCELL::iCriticalQueuedRefCount), 0);
        assert!(!e.get(cell, TESObjectCELL::bCellDetached));

        // Without the type-1 extra data, the helper is not told.
        e.register(EXTRA_LIST_GET_TYPE_1, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0054_6970, &args![cell]);
        assert!(calls_to(&take_log(&mut e), STATIC_RELEASE_HELPER).is_empty());
    }

    #[test]
    fn the_queued_reference_counter_is_cleared() {
        let mut e = scene_engine();
        let cell = cell(&mut e, false);
        e.set(cell, TESObjectCELL::iQueuedRefCount, 3);
        e.set(cell, TESObjectCELL::iCriticalQueuedRefCount, 4);
        e.call(0x0054_6a00, &args![cell]);
        assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 0);
        assert_eq!(e.get(cell, TESObjectCELL::iCriticalQueuedRefCount), 4);
    }

    #[test]
    fn the_critical_queued_reference_counter_is_cleared() {
        let mut e = scene_engine();
        let cell = cell(&mut e, false);
        e.set(cell, TESObjectCELL::iQueuedRefCount, 3);
        e.set(cell, TESObjectCELL::iCriticalQueuedRefCount, 4);
        e.call(0x0054_6a20, &args![cell]);
        assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 3);
        assert_eq!(e.get(cell, TESObjectCELL::iCriticalQueuedRefCount), 0);
    }

    #[test]
    fn the_owner_comes_from_the_extra_data_or_the_candidate() {
        let mut e = scene_engine();
        ownership_doubles(&mut e);
        e.set_global(OWNER_EXCLUDED_FORM, 0x99u32);
        let cell = cell(&mut e, false);
        let candidate = wide_object(&mut e);
        word(&mut e, candidate, 0x18, 0x5151);

        // The extra data has the owner: the candidate is not looked up.
        e.mem.set_u32(cell.addr() + CELL_WORD_A, 0x4242);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0054_6a40, &args![cell]).u32(), 0x4242);
        assert!(calls_to(&take_log(&mut e), EXTRA_LIST_GET_TYPE_74).is_empty());

        // Without it, the word at +0x18 of the candidate.
        e.mem.set_u32(cell.addr() + CELL_WORD_A, 0);
        e.mem.set_u32(cell.addr() + CELL_WORD_B, candidate.addr());
        assert_eq!(e.call(0x0054_6a40, &args![cell]).u32(), 0x5151);
        // The excluded candidate and a missing one give no owner.
        e.set_global(OWNER_EXCLUDED_FORM, candidate.addr());
        assert_eq!(e.call(0x0054_6a40, &args![cell]).u32(), 0);
        e.mem.set_u32(cell.addr() + CELL_WORD_B, 0);
        assert_eq!(e.call(0x0054_6a40, &args![cell]).u32(), 0);
    }

    #[test]
    fn the_excluded_owner_form_is_read_from_its_global() {
        let mut e = scene_engine();
        e.set_global(OWNER_EXCLUDED_FORM, 0xabcdu32);
        assert_eq!(e.call(0x0054_6a90, &[]).u32(), 0xabcd);
    }

    #[test]
    fn the_ownership_global_is_the_extra_datas() {
        let mut e = scene_engine();
        ownership_doubles(&mut e);
        let cell = cell(&mut e, false);
        e.mem.set_u32(cell.addr() + 0xec, 0x777);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0054_6aa0, &args![cell]).u32(), 0x777);
        assert_eq!(
            calls_to(&take_log(&mut e), EXTRA_LIST_GET_GLOBAL),
            vec![vec![cell.addr() + 0x28]]
        );
    }

    #[test]
    fn the_ownership_rank_turns_minus_one_into_zero() {
        let mut e = scene_engine();
        ownership_doubles(&mut e);
        let cell = cell(&mut e, false);
        e.mem.set_u32(cell.addr() + CELL_WORD_C, 0xffff_ffff);
        assert_eq!(e.call(0x0054_6ac0, &args![cell]).i32(), 0);
        e.mem.set_u32(cell.addr() + CELL_WORD_C, 3);
        assert_eq!(e.call(0x0054_6ac0, &args![cell]).i32(), 3);
        e.mem.set_u32(cell.addr() + CELL_WORD_C, (-2i32) as u32);
        assert_eq!(e.call(0x0054_6ac0, &args![cell]).i32(), -2);
    }

    #[test]
    fn the_detach_time_is_the_extra_datas() {
        let mut e = scene_engine();
        e.register(EXTRA_LIST_GET_DETACH_TIME, |_, a| (a[0] + 1).into_ret());
        let cell = cell(&mut e, false);
        assert_eq!(e.call(0x0054_6af0, &args![cell]).u32(), cell.addr() + 0x29);
    }

    #[test]
    fn setting_the_detach_time_sets_the_form_flags() {
        let mut e = scene_engine();
        quiet(&mut e, &[EXTRA_LIST_SET_DETACH_TIME, fake(0x4c)]);
        let loader = wide_object(&mut e);
        e.set_global(GAME_LOADER_POINTER, loader.addr());
        let exterior = cell(&mut e, false);
        let data = e.new_object::<ExteriorData>();
        e.set(exterior, TESObjectCELL::pCellData, data.cast());
        // Bit 2 of the loader's word is clear: the time is stored. Within
        // -128..127 on both axes: flag 0x20000000 after 0x40000000.
        e.call_log = Some(vec![]);
        e.call(0x0054_6b10, &args![exterior, 77u32, false]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, EXTRA_LIST_SET_DETACH_TIME),
            vec![vec![exterior.addr() + 0x28, 77]]
        );
        assert_eq!(
            calls_to(&log, fake(0x48)),
            vec![
                vec![exterior.addr(), 0x4000_0000],
                vec![exterior.addr(), 0x2000_0000]
            ]
        );
        // Outside the range on either axis: 0x10000000.
        for (x, y) in [(128, 0), (-129, 0), (0, 128), (0, -129)] {
            e.set(data, ExteriorData::iCellX, x);
            e.set(data, ExteriorData::iCellY, y);
            e.call_log = Some(vec![]);
            e.call(0x0054_6b10, &args![exterior, 77u32, false]);
            assert_eq!(
                calls_to(&take_log(&mut e), fake(0x48))[1],
                vec![exterior.addr(), 0x1000_0000]
            );
        }
        // The edges are inside.
        for (x, y) in [(127, -128), (-128, 127)] {
            e.set(data, ExteriorData::iCellX, x);
            e.set(data, ExteriorData::iCellY, y);
            e.call_log = Some(vec![]);
            e.call(0x0054_6b10, &args![exterior, 77u32, false]);
            assert_eq!(
                calls_to(&take_log(&mut e), fake(0x48))[1],
                vec![exterior.addr(), 0x2000_0000]
            );
        }
        // An interior cell only gets 0x40000000.
        let interior = cell(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0054_6b10, &args![interior, 5u32, false]);
        assert_eq!(
            calls_to(&take_log(&mut e), fake(0x48)),
            vec![vec![interior.addr(), 0x4000_0000]]
        );
        // A zero time clears 0x70000000 through slot 0x4c.
        e.call_log = Some(vec![]);
        e.call(0x0054_6b10, &args![exterior, 0u32, false]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, fake(0x4c)),
            vec![vec![exterior.addr(), 0x7000_0000]]
        );
        assert!(calls_to(&log, fake(0x48)).is_empty());
        // With the loader's bit 2, the time is stored only when it is zero
        // or `keep` is set.
        e.mem.set_u32(loader.addr() + 0x244, 2);
        e.call_log = Some(vec![]);
        e.call(0x0054_6b10, &args![exterior, 9u32, false]);
        assert!(calls_to(&take_log(&mut e), EXTRA_LIST_SET_DETACH_TIME).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0054_6b10, &args![exterior, 9u32, true]);
        assert_eq!(
            calls_to(&take_log(&mut e), EXTRA_LIST_SET_DETACH_TIME).len(),
            1
        );
        e.call_log = Some(vec![]);
        e.call(0x0054_6b10, &args![exterior, 0u32, false]);
        assert_eq!(
            calls_to(&take_log(&mut e), EXTRA_LIST_SET_DETACH_TIME),
            vec![vec![exterior.addr() + 0x28, 0]]
        );
    }

    #[test]
    fn setting_the_owner_sets_form_flag_8() {
        let mut e = scene_engine();
        quiet(&mut e, &[EXTRA_LIST_SET_OWNER]);
        let cell = cell(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0054_6bf0, &args![cell, 0x1357u32]);
        let log = take_log(&mut e);
        assert_eq!(
            sequence(&log),
            vec![CELL_EXTRA_DATA_LIST, EXTRA_LIST_SET_OWNER, fake(0x48)]
        );
        assert_eq!(
            calls_to(&log, EXTRA_LIST_SET_OWNER),
            vec![vec![cell.addr() + 0x28, 0x1357]]
        );
        assert_eq!(calls_to(&log, fake(0x48)), vec![vec![cell.addr(), 8]]);
    }

    #[test]
    fn the_owner_candidate_is_the_extra_data_or_the_worlds() {
        let mut e = scene_engine();
        ownership_doubles(&mut e);
        let interior = cell(&mut e, true);
        let exterior = cell(&mut e, false);
        let world = wide_object(&mut e);
        e.set(exterior, TESObjectCELL::pWorldSpace, world);
        word(&mut e, world, CANDIDATE_WORD, 0x8080);
        // The extra data wins.
        e.mem.set_u32(exterior.addr() + CELL_WORD_B, 0x6060);
        assert_eq!(e.call(0x0054_6c20, &args![exterior]).u32(), 0x6060);
        // Otherwise the world space's (an interior cell has none).
        e.mem.set_u32(exterior.addr() + CELL_WORD_B, 0);
        assert_eq!(e.call(0x0054_6c20, &args![exterior]).u32(), 0x8080);
        assert_eq!(e.call(0x0054_6c20, &args![interior]).u32(), 0);
        e.set(exterior, TESObjectCELL::pWorldSpace, Ptr::NULL);
        assert_eq!(e.call(0x0054_6c20, &args![exterior]).u32(), 0);
    }

    #[test]
    fn the_detach_time_is_set_from_the_source_object() {
        let mut e = scene_engine();
        quiet(&mut e, &[EXTRA_LIST_SET_DETACH_TIME]);
        let loader = wide_object(&mut e);
        e.set_global(GAME_LOADER_POINTER, loader.addr());
        returns(&mut e, DETACH_TIME_SOURCE, 42);
        let cell = cell(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0054_6c70, &args![cell]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, DETACH_TIME_SOURCE),
            vec![vec![DETACH_TIME_SOURCE_OBJECT]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_LIST_SET_DETACH_TIME),
            vec![vec![cell.addr() + 0x28, 42]]
        );
        assert_eq!(
            calls_to(&log, fake(0x48)),
            vec![vec![cell.addr(), 0x4000_0000]]
        );
    }

    /// A cell owned by an object of the given kind (the RTTI target its
    /// casts succeed for) with an ownership rank.
    fn owned_cell(e: &mut Engine, owner_kind: u32, rank: u32) -> (Ptr<TESObjectCELL>, Ptr) {
        let cell = cell(e, false);
        let owner = wide_object(e);
        e.mem.set_u32(owner.addr() + KIND_WORD, owner_kind);
        e.mem.set_u32(cell.addr() + CELL_WORD_A, owner.addr());
        e.mem.set_u32(cell.addr() + CELL_WORD_C, rank);
        (cell, owner)
    }

    #[test]
    fn the_actor_may_use_what_its_form_or_faction_owns() {
        let mut e = scene_engine();
        ownership_doubles(&mut e);
        let actor = actor(&mut e);
        word(&mut e, actor, 0x218, 1);
        let form = wide_object(&mut e);
        word(&mut e, actor, BASE_FORM, form.addr());

        // No owner.
        let free = cell(&mut e, false);
        assert!(!e.call(0x0054_6ca0, &args![free, actor]).bool());
        // The actor's slot 0x218 says no.
        let (cell, owner) = owned_cell(&mut e, RTTI_OWNER_CHARACTER, 0);
        word(&mut e, actor, 0x218, 0);
        assert!(!e.call(0x0054_6ca0, &args![cell, actor]).bool());
        word(&mut e, actor, 0x218, 1);
        // A character owner must be the actor's form.
        assert!(!e.call(0x0054_6ca0, &args![cell, actor]).bool());
        word(&mut e, actor, BASE_FORM, owner.addr());
        assert!(e.call(0x0054_6ca0, &args![cell, actor]).bool());

        // A faction owner: the rank of the actor's base data (a cast of the
        // actor's form) against the cell's ownership rank.
        let (cell, faction) = owned_cell(&mut e, RTTI_OWNER_FACTION, 2);
        let base = wide_object(&mut e);
        e.mem.set_u32(base.addr() + KIND_WORD, RTTI_OWNER_CHARACTER);
        word(&mut e, actor, BASE_FORM, base.addr());
        e.mem.set_u32(base.addr() + FACTION_RANK_WORD, 1);
        assert!(!e.call(0x0054_6ca0, &args![cell, actor]).bool());
        e.mem.set_u32(base.addr() + FACTION_RANK_WORD, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_6ca0, &args![cell, actor]).bool());
        let log = take_log(&mut e);
        // The actor is not the singleton: the flag is false.
        assert_eq!(
            calls_to(&log, ACTOR_BASE_GET_FACTION_RANK),
            vec![vec![base.addr() + 0x30, faction.addr(), 0]]
        );
        let casts = calls_to(&log, RT_DYNAMIC_CAST);
        assert_eq!(
            casts[2][2..4],
            [RTTI_OWNER_SOURCE_FORM, RTTI_OWNER_CHARACTER]
        );
        // The singleton gets the flag.
        e.set_global(ACTOR_SINGLETON_POINTER, actor.addr());
        e.mem.set_u32(base.addr() + FACTION_RANK_WORD, 9);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_6ca0, &args![cell, actor]).bool());
        assert_eq!(
            calls_to(&take_log(&mut e), ACTOR_BASE_GET_FACTION_RANK)[0][2],
            1
        );
        // An owner that is neither.
        let (cell, _) = owned_cell(&mut e, 0x1, 2);
        assert!(!e.call(0x0054_6ca0, &args![cell, actor]).bool());
    }

    #[test]
    fn the_actor_may_not_take_what_another_owns() {
        let mut e = scene_engine();
        ownership_doubles(&mut e);
        let actor = actor(&mut e);
        word(&mut e, actor, 0x218, 1);
        let form = wide_object(&mut e);
        word(&mut e, actor, BASE_FORM, form.addr());

        // The actor's slot 0x304 (called without arguments) vetoes.
        let (cell, owner) = owned_cell(&mut e, RTTI_OWNER_CHARACTER, 0);
        word(&mut e, actor, 0x304, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0054_6da0, &args![cell, actor]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, actor_slot(0x304)), vec![vec![actor.addr()]]);
        word(&mut e, actor, 0x304, 0);
        // A character owner: it must differ from the actor's form.
        assert!(e.call(0x0054_6da0, &args![cell, actor]).bool());
        word(&mut e, actor, BASE_FORM, owner.addr());
        assert!(!e.call(0x0054_6da0, &args![cell, actor]).bool());
        word(&mut e, actor, BASE_FORM, form.addr());
        // Cell flag 0x20 (then 0x40) makes it false; so does an ownership
        // global and the actor's slot 0x218.
        e.mem.set_u8(cell.addr() + 0x24, 0x20);
        assert!(!e.call(0x0054_6da0, &args![cell, actor]).bool());
        e.mem.set_u8(cell.addr() + 0x24, 0x40);
        assert!(!e.call(0x0054_6da0, &args![cell, actor]).bool());
        e.mem.set_u8(cell.addr() + 0x24, 0);
        e.mem.set_u32(cell.addr() + 0xec, 7);
        assert!(!e.call(0x0054_6da0, &args![cell, actor]).bool());
        e.mem.set_u32(cell.addr() + 0xec, 0);
        word(&mut e, actor, 0x218, 0);
        assert!(!e.call(0x0054_6da0, &args![cell, actor]).bool());
        word(&mut e, actor, 0x218, 1);
        // No owner.
        let free = self::cell(&mut e, false);
        assert!(!e.call(0x0054_6da0, &args![free, actor]).bool());

        // A faction owner: true when the actor's rank is below the cell's.
        let (cell, _) = owned_cell(&mut e, RTTI_OWNER_FACTION, 2);
        let base = wide_object(&mut e);
        e.mem.set_u32(base.addr() + KIND_WORD, RTTI_OWNER_CHARACTER);
        word(&mut e, actor, BASE_FORM, base.addr());
        e.mem.set_u32(base.addr() + FACTION_RANK_WORD, 2);
        assert!(!e.call(0x0054_6da0, &args![cell, actor]).bool());
        e.mem.set_u32(base.addr() + FACTION_RANK_WORD, 1);
        assert!(e.call(0x0054_6da0, &args![cell, actor]).bool());
        let (cell, _) = owned_cell(&mut e, 0x1, 2);
        assert!(!e.call(0x0054_6da0, &args![cell, actor]).bool());
    }

    #[test]
    fn a_form_owns_a_cell_by_identity_or_faction_rank() {
        let mut e = scene_engine();
        ownership_doubles(&mut e);
        let form = wide_object(&mut e);

        // No owner.
        let free = cell(&mut e, false);
        assert!(!e.call(0x0054_6ee0, &args![free, form]).bool());
        // A character owner: the form itself.
        let (cell, owner) = owned_cell(&mut e, RTTI_OWNER_CHARACTER, 0);
        assert!(!e.call(0x0054_6ee0, &args![cell, form]).bool());
        assert!(e.call(0x0054_6ee0, &args![cell, owner]).bool());
        // A faction owner: the form cast from 011846e8 must have the rank.
        let (cell, faction) = owned_cell(&mut e, RTTI_OWNER_FACTION, 3);
        e.mem.set_u32(form.addr() + KIND_WORD, RTTI_OWNER_CHARACTER);
        e.mem.set_u32(form.addr() + FACTION_RANK_WORD, 2);
        assert!(!e.call(0x0054_6ee0, &args![cell, form]).bool());
        e.mem.set_u32(form.addr() + FACTION_RANK_WORD, 3);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0054_6ee0, &args![cell, form]).bool());
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, ACTOR_BASE_GET_FACTION_RANK),
            vec![vec![form.addr() + 0x30, faction.addr(), 0]]
        );
        let casts = calls_to(&log, RT_DYNAMIC_CAST);
        assert_eq!(
            casts[2][2..4],
            [RTTI_OWNER_SOURCE_ARGUMENT, RTTI_OWNER_CHARACTER]
        );
        // A form that is not a character base.
        e.mem.set_u32(form.addr() + KIND_WORD, 0);
        assert!(!e.call(0x0054_6ee0, &args![cell, form]).bool());
        // An owner of neither kind.
        let (cell, _) = owned_cell(&mut e, 0x1, 3);
        assert!(!e.call(0x0054_6ee0, &args![cell, form]).bool());
    }

    #[test]
    fn an_exterior_cells_land_is_built_on_first_use() {
        let mut e = scene_engine();
        e.register(LAND_CONSTRUCT, |_, a| a[0].into_ret());
        quiet(&mut e, &[SCOPE_ENTER, SCOPE_LEAVE, LAND_SET_CELL]);
        let exterior = cell(&mut e, false);
        e.call_log = Some(vec![]);
        let land = e.call(0x0054_6fb0, &args![exterior]).u32();
        let log = take_log(&mut e);
        assert_ne!(land, 0);
        assert_eq!(e.get(exterior, TESObjectCELL::pCellLand).addr(), land);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x2c]]);
        assert_eq!(calls_to(&log, LAND_CONSTRUCT), vec![vec![land]]);
        assert_eq!(
            calls_to(&log, LAND_SET_CELL),
            vec![vec![land, exterior.addr()]]
        );
        assert_eq!(
            calls_to(&log, SCOPE_ENTER)[0][1..],
            [0x1b, 1, SOURCE_FILE, 0xfb6]
        );
        assert_eq!(calls_to(&log, SCOPE_LEAVE).len(), 1);
        // Built once.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0054_6fb0, &args![exterior]).u32(), land);
        assert!(calls_to(&take_log(&mut e), ALLOCATE).is_empty());
        // Interior cells and persistent cells have no land.
        let interior = cell(&mut e, true);
        assert_eq!(e.call(0x0054_6fb0, &args![interior]).u32(), 0);
        let persistent = cell(&mut e, false);
        e.mem.set_u32(persistent.addr() + 8, 0x400);
        assert_eq!(e.call(0x0054_6fb0, &args![persistent]).u32(), 0);
        assert_eq!(e.get(persistent, TESObjectCELL::pCellLand), Ptr::NULL);
    }

    #[test]
    fn the_land_scope_guard_is_a_local_of_the_frame() {
        // 00547014/00547077: the guard is the stack local at EBP-0x10, not a heap block.
        let mut e = scene_engine();
        e.register(LAND_CONSTRUCT, |_, a| a[0].into_ret());
        quiet(&mut e, &[SCOPE_ENTER, SCOPE_LEAVE, LAND_SET_CELL]);
        let exterior = cell(&mut e, false);
        let guard = e.next_stack_local(4);
        e.call_log = Some(vec![]);
        let land = tes_object_cell_get_land(&mut e, exterior);
        let log = take_log(&mut e);
        assert!(!land.is_null());
        assert_eq!(calls_to(&log, SCOPE_ENTER)[0][0], guard);
        assert_eq!(calls_to(&log, SCOPE_LEAVE), vec![vec![guard]]);
        assert_eq!(e.mem.block_size(guard), None);
    }

    #[test]
    fn replacing_the_land_destroys_the_old_one() {
        let mut e = scene_engine();
        let old = wide_object(&mut e);
        let new = wide_object(&mut e);
        let exterior = cell(&mut e, false);
        e.set(exterior, TESObjectCELL::pCellLand, old);
        e.call_log = Some(vec![]);
        e.call(0x0054_70a0, &args![exterior, new]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, wide(0x10)), vec![vec![old.addr(), 1]]);
        assert_eq!(e.get(exterior, TESObjectCELL::pCellLand), new);
        // The same land again: nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x0054_70a0, &args![exterior, new]);
        assert!(calls_to(&take_log(&mut e), wide(0x10)).is_empty());
        // No old land: just stored.
        let bare = cell(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0054_70a0, &args![bare, new]);
        assert!(calls_to(&take_log(&mut e), wide(0x10)).is_empty());
        assert_eq!(e.get(bare, TESObjectCELL::pCellLand), new);
        // An interior cell ignores it.
        let interior = cell(&mut e, true);
        e.call(0x0054_70a0, &args![interior, new]);
        assert_eq!(e.get(interior, TESObjectCELL::pCellLand), Ptr::NULL);
    }

    #[test]
    fn the_region_list_is_created_on_request() {
        let mut e = scene_engine();
        e.register(EXTRA_LIST_GET_REGION_LIST, |e, a| {
            e.mem.u32(a[0] - 0x28 + CELL_WORD_A).into_ret()
        });
        e.register(REGION_LIST_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(EXTRA_LIST_SET_REGION_LIST, |e, a| {
            e.mem.set_u32(a[0] - 0x28 + CELL_WORD_A, a[1]);
            Ret::default()
        });
        let exterior = cell(&mut e, false);
        let interior = cell(&mut e, true);
        // Absent and not asked for.
        assert_eq!(e.call(0x0054_7110, &args![exterior, false]).u32(), 0);
        // Asked for: a 0x10-byte list is built and stored.
        e.call_log = Some(vec![]);
        let list = e.call(0x0054_7110, &args![exterior, true]).u32();
        let log = take_log(&mut e);
        assert_ne!(list, 0);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, REGION_LIST_CONSTRUCT), vec![vec![list, 0]]);
        assert_eq!(
            calls_to(&log, EXTRA_LIST_SET_REGION_LIST),
            vec![vec![exterior.addr() + 0x28, list]]
        );
        // Present: returned without building.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0054_7110, &args![exterior, true]).u32(), list);
        assert!(calls_to(&take_log(&mut e), ALLOCATE).is_empty());
        // An interior cell has none.
        assert_eq!(e.call(0x0054_7110, &args![interior, true]).u32(), 0);
    }

    #[test]
    fn the_water_height_follows_the_flag_the_cell_and_the_world() {
        let mut e = scene_engine();
        let world = wide_object(&mut e);
        e.register(WORLD_SPACE_WATER_HEIGHT, |_, _| Ret {
            st0: 64.0,
            ..Ret::default()
        });
        let water = |e: &mut Engine, cell: Ptr<TESObjectCELL>| -> f32 {
            e.call(0x0054_71e0, &args![cell]).f32()
        };
        let exterior = cell(&mut e, false);
        e.set(exterior, TESObjectCELL::pWorldSpace, world);
        e.set(exterior, TESObjectCELL::fWaterHeight, f32::MAX);
        // Without bit 1 of the cell flags: the lowest float.
        assert_eq!(water(&mut e, exterior), f32::MIN);
        e.mem.set_u8(exterior.addr() + 0x24, 2);
        // The unset height falls back to the world space's.
        assert_eq!(water(&mut e, exterior), 64.0);
        // A set height wins.
        e.set(exterior, TESObjectCELL::fWaterHeight, -12.5);
        assert_eq!(water(&mut e, exterior), -12.5);
        // A NaN is not the unset value.
        e.set(exterior, TESObjectCELL::fWaterHeight, f32::NAN);
        assert!(water(&mut e, exterior).is_nan());
        // Without a world space: 0.
        e.set(exterior, TESObjectCELL::fWaterHeight, f32::MAX);
        e.set(exterior, TESObjectCELL::pWorldSpace, Ptr::NULL);
        assert_eq!(water(&mut e, exterior), 0.0);
        // An interior cell is the lowest float whatever its flags.
        let interior = cell(&mut e, true);
        e.mem.set_u8(interior.addr() + 0x24, 3);
        assert_eq!(water(&mut e, interior), f32::MIN);
    }

    /// A water reference: a 3D whose first child's shape contains the
    /// position or not, with a primitive, and a top at `+8` of its `0x1f4`
    /// result. Returns the reference, the shape and the primitive.
    fn water_reference(e: &mut Engine, contains: bool, top: f32) -> (Ptr, Ptr, Ptr) {
        let reference = wide_object(e);
        let node_3d = wide_object(e);
        let holder = wide_object(e);
        let child = wide_object(e);
        let shape = wide_object(e);
        let data = wide_object(e);
        let primitive = wide_object(e);
        let result = wide_object(e);
        answer(e, reference, 0x1d0, node_3d.addr());
        answer(e, node_3d, 0xc, holder.addr());
        word(e, holder, 0x7c, child.addr());
        answer(e, child, 0x14, shape.addr());
        word(e, shape, MULTI_BOUND_DATA, data.addr());
        word(e, data, PRIMITIVE, primitive.addr());
        answer(e, shape, 0x108, u32::from(contains));
        answer(e, reference, 0x1f4, result.addr());
        e.mem.set_f32(result.addr() + 8, top);
        (reference, shape, primitive)
    }

    #[test]
    fn the_water_height_at_a_position_takes_the_highest_containing_reference() {
        let mut e = scene_engine();
        // The child of the holder at index 0: the word at +0x7c.
        e.register(NODE_CHILD_AT, |e, a| {
            assert_eq!(a[1], 0);
            e.mem.u32(a[0] + 0x7c).into_ret()
        });
        e.register(CELL_LOADED_DATA, |e, a| e.mem.u32(a[0] + 0xc4).into_ret());
        e.register(REFERENCE_ROTATION, |_, a| (a[0] + 0x24).into_ret());
        e.register(PRIMITIVE_PART, |_, a| (a[0] + 0xc).into_ret());
        e.register(WORLD_SPACE_WATER_HEIGHT, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        // The shapes record the position they are asked about.
        let asked = Rc::new(RefCell::new(vec![]));
        let record = asked.clone();
        e.register_double(wide(0x108), move |e, a| {
            record
                .borrow_mut()
                .push((e.mem.f32(a[1]), e.mem.f32(a[1] + 4), e.mem.f32(a[1] + 8)));
            e.mem.u32(a[0] + SLOT_ANSWERS + 0x108).into_ret()
        });
        let (cell, loaded) = loaded_cell(&mut e, false);
        e.mem.set_u8(cell.addr() + 0x24, 2);
        e.set(cell, TESObjectCELL::fWaterHeight, f32::MAX);
        let world = wide_object(&mut e);
        e.set(cell, TESObjectCELL::pWorldSpace, world);
        // Four references: a primitive of type 1 (third coordinate 7.5), a
        // shape that does not contain the position, a primitive of type 2
        // (8.5) with the highest top, and a lower one.
        let (low, _, low_primitive) = water_reference(&mut e, true, 20.0);
        answer(&mut e, low_primitive, 0x8c, 1);
        e.mem.set_f32(low_primitive.addr() + 0x14, 7.5);
        let (outside, _, _) = water_reference(&mut e, false, 99.0);
        let (high, _, high_primitive) = water_reference(&mut e, true, 35.0);
        answer(&mut e, high_primitive, 0x8c, 2);
        e.mem.set_f32(high_primitive.addr() + 0x14, 8.5);
        let (lower, _, _) = water_reference(&mut e, true, 5.0);
        fill_list(&mut e, loaded.addr() + 0x5c, &[low, outside, high, lower]);

        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        let height = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let found = e.call(0x0054_7250, &args![cell, position, height]).bool();
        let log = take_log(&mut e);
        assert!(found);
        // The height starts at the world's 10.0 and is raised to the highest
        // containing reference's top.
        assert_eq!(e.mem.f32(height), 35.0);
        // The shapes were asked about x, y and the third coordinate the
        // primitives gave (kept from one reference to the next).
        assert_eq!(
            *asked.borrow(),
            vec![
                (1.0, 2.0, 7.5),
                (1.0, 2.0, 7.5),
                (1.0, 2.0, 8.5),
                (1.0, 2.0, 8.5)
            ]
        );
        // The caller's position is not changed.
        assert_eq!(e.mem.f32(position + 8), 3.0);
        // Both buffers of slots 0x1d8 and 0x1dc were passed for every
        // reference.
        assert_eq!(calls_to(&log, wide(0x1d8)).len(), 4);
        assert_eq!(calls_to(&log, wide(0x1dc)).len(), 4);
    }

    #[test]
    fn the_water_height_at_a_position_without_water_references() {
        let mut e = scene_engine();
        e.register(CELL_LOADED_DATA, |e, a| e.mem.u32(a[0] + 0xc4).into_ret());
        e.register(WORLD_SPACE_WATER_HEIGHT, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        let position = e.mem.alloc(12);
        let height = e.mem.alloc(4);
        // An interior cell without bit 1: -FLT_MAX and false.
        let interior = cell(&mut e, true);
        e.mem.set_f32(height, 1.0);
        assert!(!e
            .call(0x0054_7250, &args![interior, position, height])
            .bool());
        assert_eq!(e.mem.f32(height), f32::MIN);
        // An exterior cell with the flag but no loaded data: the height of
        // the cell, and false.
        let exterior = cell(&mut e, false);
        e.mem.set_u8(exterior.addr() + 0x24, 2);
        e.set(exterior, TESObjectCELL::fWaterHeight, 4.5);
        assert!(!e
            .call(0x0054_7250, &args![exterior, position, height])
            .bool());
        assert_eq!(e.mem.f32(height), 4.5);
        // Loaded data with an empty water list.
        let (with_data, _) = loaded_cell(&mut e, false);
        e.mem.set_u8(with_data.addr() + 0x24, 2);
        e.set(with_data, TESObjectCELL::fWaterHeight, 6.0);
        assert!(!e
            .call(0x0054_7250, &args![with_data, position, height])
            .bool());
        assert_eq!(e.mem.f32(height), 6.0);
    }

    #[test]
    fn setting_the_water_height_keeps_the_unset_value_for_the_worlds_height() {
        let mut e = scene_engine();
        let world = wide_object(&mut e);
        let heights = Rc::new(RefCell::new(vec![]));
        let record = heights.clone();
        e.register(WORLD_SPACE_WATER_HEIGHT, |_, _| Ret {
            st0: 100.0,
            ..Ret::default()
        });
        e.register_double(FLOAT_NEAR, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            u32::from((first - second).abs() <= f32::from_bits(a[2])).into_ret()
        });
        let exterior = cell(&mut e, false);
        e.set(exterior, TESObjectCELL::pWorldSpace, world);
        // Within 0.001 of the world's height: unset (FLT_MAX).
        e.call(0x0054_7440, &args![exterior, 100.0005f32]);
        assert_eq!(e.get(exterior, TESObjectCELL::fWaterHeight), f32::MAX);
        assert_eq!(
            heights.borrow()[0],
            vec![
                100.0f32.to_bits(),
                100.0005f32.to_bits(),
                0.001f32.to_bits()
            ]
        );
        // Further away: stored.
        e.call(0x0054_7440, &args![exterior, 120.0f32]);
        assert_eq!(e.get(exterior, TESObjectCELL::fWaterHeight), 120.0);
        // Without a world space: stored without the comparison.
        let before = heights.borrow().len();
        e.set(exterior, TESObjectCELL::pWorldSpace, Ptr::NULL);
        e.call(0x0054_7440, &args![exterior, 100.0f32]);
        assert_eq!(e.get(exterior, TESObjectCELL::fWaterHeight), 100.0);
        assert_eq!(heights.borrow().len(), before);
    }

    #[test]
    fn the_type_7_extra_data_falls_back_to_the_worlds_value() {
        let mut e = scene_engine();
        e.register(EXTRA_LIST_GET_TYPE_7, |e, a| {
            e.mem.u32(a[0] - 0x28 + CELL_WORD_A).into_ret()
        });
        e.register(WORLD_SPACE_TYPE_7_VALUE, |_, a| (a[0] + 1).into_ret());
        e.mem.set_u32(ZERO_VECTOR, 0x11);
        e.mem.set_u32(ZERO_VECTOR + 4, 0x22);
        e.mem.set_u32(ZERO_VECTOR + 8, 0x33);
        let exterior = cell(&mut e, false);
        let world = wide_object(&mut e);
        e.set(exterior, TESObjectCELL::pWorldSpace, world);
        // The first call fills the static record.
        e.call_log = Some(vec![]);
        let result = e.call(0x0054_74b0, &args![exterior, 0u32]).u32();
        let log = take_log(&mut e);
        assert_eq!(result, world.addr() + 1);
        assert_eq!(e.global::<u32>(STATIC_RECORD_GUARD) & 1, 1);
        assert_eq!(e.global::<u32>(STATIC_RECORD_COPY), 0x11);
        assert_eq!(e.global::<u32>(STATIC_RECORD_COPY + 4), 0x22);
        assert_eq!(e.global::<u32>(STATIC_RECORD_COPY + 8), 0x33);
        assert_eq!(e.global::<u32>(STATIC_RECORD_LAST), world.addr() + 1);
        // The lock is held around it.
        let order = sequence(&log);
        assert_eq!(order.first(), Some(&LOCK_ENTER));
        assert_eq!(order.last(), Some(&LOCK_LEAVE));
        // The record is filled once.
        e.mem.set_u32(ZERO_VECTOR, 0x99);
        e.call(0x0054_74b0, &args![exterior, 0u32]);
        assert_eq!(e.global::<u32>(STATIC_RECORD_COPY), 0x11);
        // The extra data wins; an interior cell returns its value (or 0).
        e.mem.set_u32(exterior.addr() + CELL_WORD_A, 0x5c5c);
        e.set_global(STATIC_RECORD_LAST, 0u32);
        assert_eq!(e.call(0x0054_74b0, &args![exterior, 0u32]).u32(), 0x5c5c);
        assert_eq!(e.global::<u32>(STATIC_RECORD_LAST), 0);
        let interior = cell(&mut e, true);
        assert_eq!(e.call(0x0054_74b0, &args![interior, 0u32]).u32(), 0);
        e.mem.set_u32(interior.addr() + CELL_WORD_A, 0x6d6d);
        assert_eq!(e.call(0x0054_74b0, &args![interior, 0u32]).u32(), 0x6d6d);
        // An exterior cell without a world space: 0.
        e.mem.set_u32(exterior.addr() + CELL_WORD_A, 0);
        e.set(exterior, TESObjectCELL::pWorldSpace, Ptr::NULL);
        assert_eq!(e.call(0x0054_74b0, &args![exterior, 0u32]).u32(), 0);
    }

    #[test]
    fn the_acoustic_space_is_the_extra_datas() {
        let mut e = scene_engine();
        e.register(EXTRA_LIST_GET_ACOUSTIC_SPACE, |_, a| (a[0] + 2).into_ret());
        let cell = cell(&mut e, false);
        assert_eq!(e.call(0x0054_7590, &args![cell]).u32(), cell.addr() + 0x2a);
    }

    #[test]
    fn the_type_8_extra_data_needs_an_interior_cell_with_flag_0x80() {
        let mut e = scene_engine();
        e.register(EXTRA_LIST_GET_TYPE_8, |_, a| (a[0] + 3).into_ret());
        e.register(WORLD_SPACE_ACOUSTIC_VALUE, |_, a| (a[0] + 4).into_ret());
        let interior = cell(&mut e, true);
        // Flag 0x80 clear: 0.
        e.mem.set_u8(interior.addr() + 0x24, 1);
        assert_eq!(e.call(0x0054_75b0, &args![interior]).u32(), 0);
        e.mem.set_u8(interior.addr() + 0x24, 0x81);
        assert_eq!(
            e.call(0x0054_75b0, &args![interior]).u32(),
            interior.addr() + 0x2b
        );
        // An exterior cell asks its world space.
        let exterior = cell(&mut e, false);
        let world = wide_object(&mut e);
        e.set(exterior, TESObjectCELL::pWorldSpace, world);
        assert_eq!(
            e.call(0x0054_75b0, &args![exterior]).u32(),
            world.addr() + 4
        );
    }

    #[test]
    fn the_last_extra_data_accessor_needs_an_interior_cell() {
        let mut e = scene_engine();
        e.register(EXTRA_LIST_GET_IMPACT_SWAP, |_, a| (a[0] + 5).into_ret());
        let interior = cell(&mut e, true);
        let exterior = cell(&mut e, false);
        assert_eq!(
            e.call(0x0054_7610, &args![interior]).u32(),
            interior.addr() + 0x2d
        );
        assert_eq!(e.call(0x0054_7610, &args![exterior]).u32(), 0);
    }
}

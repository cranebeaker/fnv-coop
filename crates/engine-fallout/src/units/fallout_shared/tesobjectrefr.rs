//! `fallout shared/tesobjectrefr.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! # Layout of a reference on the PC
//!
//! A `TESObjectREFR` is 0x68 bytes on the PC (0x78 on the Xbox): the Xbox PDB
//! offsets of everything after `TESForm` are 0x10 lower here, because the PC
//! `TESForm` is 0x18 bytes (no editor ID, no version-control fields). The
//! `TESChildCell` base is only a vtable pointer at +0x18. The embedded
//! `ExtraDataList` (`m_Extra`) is at +0x44; the game reaches it through
//! `005d43c0` ([`GET_EXTRA_LIST`], `this + 0x44`, also used on other
//! references), and so do the translations.
//!
//! Fields of `TESForm` read here are read at their PC offset with a comment
//! (the form's own unit declares them): `iFormFlags` +0x08, `iFormID` +0x0C.
//!
//! # Where this file stops
//!
//! This file holds three batches of 40 functions each: `00426720` to
//! `0055e730` (the first batch; `0055e230` was left for the second), then
//! `0055e230` and `0055e940` to `005612a0` (the game-load and game-save
//! handlers, the sequence and Havok records of a reference, and the helpers
//! of the Havok glue the linker placed here), and then `00561440` to
//! `00563890` (the rest of the Havok glue, the editor location, the
//! buffer-based save and load handlers and their Havok and animation
//! records). That finishes the address range of this file (everything below
//! `00563d80`); the unit's functions from `00563d80` on belong to the next
//! part file.
//!
//! Several functions in this range are not `TESObjectREFR` methods but small
//! `ExtraDataList`, list-node and manager helpers the linker placed in this
//! unit; their `this` is documented per function. Callees outside this
//! translation (including the unit's own later functions, such as
//! `005653d0 GetRefPersists`) are called by address.
//!
//! The vtable slots called here (`vcall`) are PC offsets taken from the
//! code; the PC vtable has one more entry than the Xbox one between `+0x130`
//! and `+0x1c4`, so slot names from the Xbox PDB are quoted only where the
//! PC body matches.
//!
//! # Compiled-out code
//!
//! Some functions test `flags & 0` (a debug feature compiled out); those
//! branches never run and are not translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESObjectREFR`'s vtable (`0102f55c`) and `TESChildCell`'s (`0102f550`).
pub(crate) const REFR_VTABLE: u32 = 0x0102_f55c;
pub(crate) const CHILD_CELL_VTABLE: u32 = 0x0102_f550;

/// `this + 0x44`: the reference's embedded `ExtraDataList` (also called on
/// other references and on list nodes; the game never inlines it).
pub(crate) const GET_EXTRA_LIST: u32 = 0x005d_43c0;
/// `__RTDynamicCast(object, 0, from, to, 0)` (cdecl).
pub(crate) const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors the casts use (`.?AVTESForm@@` and so on).
pub(crate) const TYPE_TES_FORM: u32 = 0x0118_3028;
pub(crate) const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
pub(crate) const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
pub(crate) const TYPE_TES_OBJECT_CELL: u32 = 0x0118_3fb4;
pub(crate) const TYPE_TES_CHILD_CELL: u32 = 0x0118_ac2c;
pub(crate) const TYPE_BS_EXTRA_DATA: u32 = 0x0118_3b2c;
pub(crate) const TYPE_EXTRA_TELEPORT: u32 = 0x0118_4430;
pub(crate) const TYPE_EXTRA_ENABLE_STATE_PARENT: u32 = 0x0118_4d7c;
pub(crate) const TYPE_EXTRA_RANDOM_TELEPORT_MARKER: u32 = 0x0118_4dcc;
pub(crate) const TYPE_EXTRA_LINKED_REF: u32 = 0x0118_4e1c;
pub(crate) const TYPE_EXTRA_ACTIVATE_REF: u32 = 0x0118_4e84;
pub(crate) const TYPE_EXTRA_DECAL_REFS: u32 = 0x0118_4ea4;
pub(crate) const TYPE_EXTRA_MERCHANT_CONTAINER: u32 = 0x0118_4ec4;
pub(crate) const TYPE_EXTRA_MULTI_BOUND_REF: u32 = 0x0118_4f50;

/// `BaseExtraList` methods (the receiver is the list, `this + 0x44` of a
/// reference).
pub(crate) const EXTRA_GET_DATA: u32 = 0x0041_0220; // GetExtraData(type) -> BSExtraData*
pub(crate) const EXTRA_REMOVE: u32 = 0x0041_0020; // RemoveExtra(extra, free)
pub(crate) const EXTRA_ADD: u32 = 0x0040_ff60; // AddExtra(extra)
pub(crate) const EXTRA_REMOVE_ALL: u32 = 0x0040_fae0; // RemoveAll(keep_defaults)
pub(crate) const EXTRA_REMOVE_ALL_DEFAULT: u32 = 0x0040_fcb0; // RemoveAllDefault(keep_defaults)
pub(crate) const EXTRA_REMOVE_TYPE: u32 = 0x0041_0140; // RemoveExtra_ov2(type)
pub(crate) const EXTRA_GET_PERSISTENT_CELL: u32 = 0x0041_d460;
pub(crate) const EXTRA_SET_PERSISTENT_CELL: u32 = 0x0041_d390;
/// `TESObjectCELL::RemoveReference(cell, ref)`.
pub(crate) const CELL_REMOVE_REFERENCE: u32 = 0x0054_ca90;
/// `BSSimpleList` helpers: the node's item slot (identity on the node), the
/// next node, whether a node is empty.
pub(crate) const LIST_ITEM_SLOT: u32 = 0x0068_15c0;
pub(crate) const LIST_NEXT: u32 = 0x0072_6070;
pub(crate) const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Removes the item at a pointer from a `BSSimpleList` (`list`, `&item`).
pub(crate) const LIST_REMOVE: u32 = 0x0090_5330;
/// `*(u32 *)this`: the first dword of a pointer wrapper or `BSStringT`.
pub(crate) const READ_FIRST_DWORD: u32 = 0x0055_9450;
/// `TESForm` accessors.
pub(crate) const FORM_TYPE: u32 = 0x0040_1170; // cFormType (byte at +4)
pub(crate) const FORM_ID: u32 = 0x0084_e3a0; // iFormID (+0x0C)
pub(crate) const FORM_IS_DELETED: u32 = 0x0044_0d80; // iFormFlags & 0x20
pub(crate) const FORM_IS_DISABLED: u32 = 0x0044_0da0; // iFormFlags & 0x800
/// The reference's base object (`data.pObjectReference`, +0x20; the map
/// names the body `BGSSaveFormBuffer::GetForm (Xbox PDB)`).
pub(crate) const GET_BASE_FORM: u32 = 0x007a_f430;
/// The reference's parent cell (`pParentCell`, +0x40).
pub(crate) const GET_PARENT_CELL: u32 = 0x008d_6f30;
/// `&this->data` (`OBJ_REFR`, +0x20).
pub(crate) const GET_DATA: u32 = 0x0089_1170;
/// Constructor/destructor of the 8-byte `BSStringT` the game keeps on its
/// stack, and `BSStringT::Format(fmt, ...)`.
pub(crate) const STRING_CONSTRUCT: u32 = 0x0040_37b0;
pub(crate) const STRING_DESTRUCT: u32 = 0x0040_37d0;
pub(crate) const STRING_FORMAT: u32 = 0x0040_6f60;
/// The CRT (called by address): `sprintf`, `memcmp`, `memset` (via the
/// wrapper at `00403d30`), `_finite` and `_isnan` (which take a `double`).
pub(crate) const SPRINTF: u32 = 0x00ec_623a;
pub(crate) const MEMCMP: u32 = 0x00ec_4835;
pub(crate) const MEMSET: u32 = 0x0040_3d30;
pub(crate) const CRT_FINITE: u32 = 0x00ec_7595;
pub(crate) const CRT_ISNAN: u32 = 0x00ec_75b1;
/// The error logger (`Error(fmt, ...)`) and the message log (`005b5e40`).
pub(crate) const ERROR_LOG: u32 = 0x0040_fbe0;
pub(crate) const MESSAGE: u32 = 0x005b_5e40;
/// `TESObjectREFR::GetRefPersists` (translated later in this unit).
pub(crate) const GET_REF_PERSISTS: u32 = 0x0056_53d0;
/// Whether the current thread's loading flag (TLS +0x294 bit 2) is set; the
/// game calls it on `011ddf38`'s object (it does not use `this`).
pub(crate) const LOADING_FLAG: u32 = 0x0041_21b0;

/// Writers of the save record (`TESForm::AddChunk (value)` and
/// `__AddChunkData`), whether the build swaps endianness on save, and the
/// double `1.0`.
pub(crate) const ADD_CHUNK_VALUE: u32 = 0x0048_5910;
pub(crate) const ADD_CHUNK_DATA: u32 = 0x0048_5990;
pub(crate) const ENDIAN_SWAP_ON_SAVE: u32 = 0x0040_1500;
pub(crate) const CONSTANT_ONE: u32 = 0x0101_2070;

/// Globals the code reads.
pub(crate) const GLOBAL_PLAYER: u32 = 0x011d_ea3c; // PlayerCharacter *
pub(crate) const GLOBAL_SAVE_LOAD: u32 = 0x011d_e45c; // TESSaveLoadGame *
pub(crate) const GLOBAL_DATA_HANDLER: u32 = 0x011c_3f2c; // TESDataHandler *
pub(crate) const GLOBAL_LOADING_OBJECT: u32 = 0x011d_df38; // object whose 4121b0 flag says "loading"
pub(crate) const GLOBAL_TEMP_REFR_MANAGER: u32 = 0x011d_ea10;
pub(crate) const GLOBAL_LAST_REFR: u32 = 0x011d_ea24; // reference remembered by 0055ac70/0055ac80
pub(crate) const GLOBAL_STORED_BY_825C00: u32 = 0x011c_a28c;
pub(crate) const GLOBAL_IO_MANAGER: u32 = 0x0120_2d98;
pub(crate) const GLOBAL_DEBUG_SWITCHES: u32 = 0x011d_e4e8; // an object, passed as `this`
/// Static objects passed as `this`.
pub(crate) const OBJECT_PROCESS_LISTS: u32 = 0x011e_0e80; // ProcessLists
pub(crate) const OBJECT_FURNITURE_MANAGER: u32 = 0x011e_043c;
pub(crate) const OBJECT_ACTOR_LISTS: u32 = 0x011f_6394;
/// Getters (in `teswater.cpp`) that return the addresses of two global
/// lists (`011ca13c`, `011ca144`) the destructor removes the reference from.
pub(crate) const WATER_LIST_A_GETTER: u32 = 0x004e_3260;
pub(crate) const WATER_LIST_B_GETTER: u32 = 0x004e_c7b0;
/// The `float` the loaded-data water height is reset to.
pub(crate) const LOADED_DATA_DEFAULT_HEIGHT: u32 = 0x0101_5f5c;
/// The zero vector (three dwords) at `011f426c`.
pub(crate) const ZERO_VECTOR: u32 = 0x011f_426c;
/// The `float` tolerance `Copy` compares radii with (0.001).
pub(crate) const RADIUS_EPSILON: u32 = 0x0101_7d00;
/// The global holding the vtable address of the 0x14-byte group-data record
/// that `fn_0055e5e0` fills.
pub(crate) const GROUP_DATA_VTABLE_PTR: u32 = 0x0118_7020;
/// The empty string.
pub(crate) const EMPTY_STRING: u32 = 0x0101_1584;

/// Message and format strings in the exe.
pub(crate) const MISSING_BASE_FORMAT: u32 = 0x0102_f828;
pub(crate) const LOAD_TERMINATED_MESSAGE: u32 = 0x0102_f7ec;
pub(crate) const PATROL_NOT_COMPILED_FORMAT: u32 = 0x0102_f7a0;
pub(crate) const LEVELED_BASE_MESSAGE: u32 = 0x0102_fbb0;
pub(crate) const CORRUPT_LOCATION_MESSAGE: u32 = 0x0102_fb68;
pub(crate) const CORRUPT_ANGLE_MESSAGE: u32 = 0x0102_fb20;
pub(crate) const SHARED_DATA_REMOVED_MESSAGE: u32 = 0x0102_f870;
pub(crate) const INIT_ITEM_ERROR_CELL: u32 = 0x0102_fa78;
pub(crate) const INIT_ITEM_ERROR_WORLD: u32 = 0x0102_f9b0;
pub(crate) const INIT_ITEM_ERROR_NO_CELL: u32 = 0x0102_f910;
pub(crate) const DETAILED_FORMAT: u32 = 0x0102_f8b8;
pub(crate) const DETAILED_BASE_FORMAT: u32 = 0x0102_f8f8;
pub(crate) const DETAILED_WORLD_FORMAT: u32 = 0x0102_f8dc;
pub(crate) const SAVE_SIZE_FORMAT: u32 = 0x0101_2cb0;
pub(crate) const SAVE_SIZE_SHORT_FORMAT: u32 = 0x0101_2c78;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\TESObjectREFR.cpp"`
/// and the line `GetSaveSize` reports.
pub(crate) const SOURCE_FILE: u32 = 0x0102_fc20;
pub(crate) const SOURCE_LINE: u32 = 0x0ad0;

/// A four-character record or chunk tag as the game's 32-bit constant.
pub(crate) const fn tag(name: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*name)
}

/// The chunk kinds `Load` hands to `ExtraDataList::Load`.
pub(crate) const EXTRA_DATA_CHUNKS: [u32; 76] = [
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

layout! {
    /// `OBJ_REFR` (Xbox PDB), 0x1C bytes: the reference's base object,
    /// rotation and position (two `NiPoint3`).
    #[allow(non_camel_case_types)]
    pub struct OBJ_REFR: 0x1c {
        /// `pObjectReference` (Xbox PDB): `TESBoundObject*`.
        0x00 pObjectReference: Ptr,
        /// `Angle` (Xbox PDB), `NiPoint3`.
        0x04 AngleX: f32,
        0x08 AngleY: f32,
        0x0C AngleZ: f32,
        /// `Location` (Xbox PDB), `NiPoint3`.
        0x10 LocationX: f32,
        0x14 LocationY: f32,
        0x18 LocationZ: f32,
    }

    /// `LOADED_REF_DATA` (Xbox PDB), 0x1C bytes, only the fields used so far.
    #[allow(non_camel_case_types)]
    pub struct LOADED_REF_DATA: 0x1c {
        /// `pCurrentWaterObject` (Xbox PDB): `TESObjectREFR*`.
        0x00 pCurrentWaterObject: Ptr,
        /// `iUnderwaterCount` (Xbox PDB).
        0x04 iUnderwaterCount: i32,
        /// `fRelevantWaterHeight` (Xbox PDB).
        0x08 fRelevantWaterHeight: f32,
        /// `fCachedRadius` (Xbox PDB).
        0x0C fCachedRadius: f32,
        /// `spPhantom` (Xbox PDB): `NiPointer<bhkPhantom>`.
        0x18 spPhantom: Ptr,
    }

    /// `TESObjectREFR` (Xbox PDB), 0x68 bytes on the PC; offsets are the Xbox
    /// ones minus 0x10 (see the module notes). The `TESForm` base is the first
    /// 0x18 bytes.
    pub struct TESObjectREFR: 0x68 {
        /// Vtable pointer of the `TESChildCell` base.
        0x18 vfptrTESChildCell: u32,
        /// `pRandomSound` (Xbox PDB): `TESSound*`.
        0x1C pRandomSound: Ptr,
        /// `data` (Xbox PDB): the embedded [`OBJ_REFR`].
        0x20 data: Inline<OBJ_REFR>,
        /// `fRefScale` (Xbox PDB).
        0x3C fRefScale: f32,
        /// `pParentCell` (Xbox PDB): `TESObjectCELL*`.
        0x40 pParentCell: Ptr,
        /// `pLoadedData` (Xbox PDB): `LOADED_REF_DATA*`.
        0x64 pLoadedData: Ptr,
    }
}

/// `m_Extra` (Xbox PDB), the embedded `ExtraDataList`, at +0x44.
pub(crate) const EXTRA_OFFSET: u32 = 0x44;
/// `TESChildCell` base subobject, at +0x18.
pub(crate) const CHILD_CELL_OFFSET: u32 = 0x18;

pub(crate) fn extra_list(e: &mut Engine, refr: u32) -> u32 {
    e.call(GET_EXTRA_LIST, &args![refr]).u32()
}

pub(crate) fn dynamic_cast(e: &mut Engine, object: u32, to: u32) -> u32 {
    e.call(DYNAMIC_CAST, &args![object, 0u32, TYPE_TES_FORM, to, 0u32])
        .u32()
}

/// `IsActor` (virtual at +0x100) on `actor`, false for a null pointer: the
/// guard these helpers all start with.
pub(crate) fn is_actor(e: &mut Engine, actor: u32) -> bool {
    actor != 0 && e.vcall(actor, 0x100, &args![]).bool()
}

/// The cell the `TESChildCell` base reports (its virtual at slot 0).
pub(crate) fn save_parent_cell(e: &mut Engine, refr: Ptr<TESObjectREFR>) -> u32 {
    e.vcall(refr.addr() + CHILD_CELL_OFFSET, 0, &args![]).u32()
}

pub(crate) fn global_ptr(e: &Engine, addr: u32) -> u32 {
    e.global::<u32>(addr)
}

/// `BGSSaveLoadReferencesMap::Lookup(old, &new)` (`00853130`, on the map):
/// the new pointer for `old`, 0 when it has none.
pub(crate) fn lookup_new_reference(e: &mut Engine, map: u32, old: u32) -> u32 {
    e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), 0);
        let found = e.call(0x0085_3130, &args![map, old, out]).bool();
        if found {
            e.mem.u32(out.addr())
        } else {
            0
        }
    })
}

/// [`lookup_new_reference`], cast to `TESObjectREFR`; 0 when either fails.
pub(crate) fn resolve_reference(e: &mut Engine, map: u32, old: u32) -> u32 {
    let new = lookup_new_reference(e, map, old);
    if new == 0 {
        return 0;
    }
    dynamic_cast(e, new, TYPE_TES_OBJECT_REFR)
}

/// The body of `TESObjectREFR::Load` (`0055af90`): `id_slot` is the stack word
/// `TESFile::GetChunkData` writes the base object's form ID to.
pub(crate) fn load_record(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    file: u32,
    id_slot: u32,
) -> bool {
    let me = this.addr();
    let data = this.at(TESObjectREFR::data);
    e.with_stack(0x18, |e, scratch| {
        e.call(0x0069_2710, &args![scratch]);
    });
    e.call(0x0048_4ab0, &args![this, 0u32]);
    let mut base_missing = false;
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);

    if e.call(FORM_IS_DELETED, &args![this]).bool() {
        let cell = e
            .call(EXTRA_GET_PERSISTENT_CELL, &args![me + EXTRA_OFFSET])
            .u32();
        let loading = global_ptr(e, GLOBAL_LOADING_OBJECT);
        if loading == 0 || !e.call(LOADING_FLAG, &args![loading]).bool() {
            e.call(0x0056_b0b0, &args![this]);
        }
        e.call(EXTRA_REMOVE_ALL, &args![me + EXTRA_OFFSET, 1u32]);
        if cell != 0 {
            e.call(EXTRA_SET_PERSISTENT_CELL, &args![me + EXTRA_OFFSET, cell]);
        }
        return e.call(GET_BASE_FORM, &args![this]).u32() != 0;
    }

    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        if EXTRA_DATA_CHUNKS.contains(&chunk) {
            let list = extra_list(e, me);
            e.call(0x0041_44a0, &args![list, file, this]);
        } else if chunk == tag(b"EDID") {
            let size = e.call(0x0040_1660, &args![file]).u32();
            // `_alloca(chunk size)`
            e.with_stack(size.max(1), |e, text| {
                e.call(0x0047_2890, &args![file, text, 0x200u32]);
                e.vcall(me, 0x134, &args![text]);
            });
        } else if chunk == tag(b"OBND") {
            e.vcall(me, 0xe0, &args![file]);
        } else if chunk == tag(b"XSCL") {
            e.call(0x0047_27f0, &args![file, me + 0x3c]);
        } else if chunk == tag(b"NAME") {
            e.call(0x0047_27f0, &args![file, id_slot]);
            e.call(0x0048_5d50, &args![id_slot, file]);
            let form_id = e.mem.u32(id_slot);
            let form = e.call(0x0048_39c0, &args![form_id]).u32();
            let base = dynamic_cast(e, form, TYPE_TES_BOUND_OBJECT);
            let mut accept = true;
            let current = e.get(data, OBJ_REFR::pObjectReference);
            let loading = global_ptr(e, GLOBAL_LOADING_OBJECT);
            if !current.is_null() && e.call(LOADING_FLAG, &args![loading]).bool() {
                let current_id = e.call(FORM_ID, &args![current]).u32();
                let handler = global_ptr(e, GLOBAL_DATA_HANDLER);
                if e.call(0x0046_9860, &args![handler, current_id]).bool() {
                    accept = false;
                }
            }
            if accept {
                e.set(data, OBJ_REFR::pObjectReference, Ptr::new(base));
                if base == 0 {
                    let my_id = e.call(FORM_ID, &args![this]).u32();
                    e.call(MESSAGE, &args![MISSING_BASE_FORMAT, form_id, my_id]);
                    base_missing = true;
                }
            }
        } else if chunk == tag(b"ONAM") {
            e.call(0x0057_2d50, &args![this, 8u32]);
        } else if chunk == tag(b"DATA") {
            if !base_missing {
                e.with_stack(0x18, |e, buffer| {
                    e.call(0x0069_2710, &args![buffer]);
                    e.vcall(me, 0xc4, &args![0u32]);
                    e.call(0x0047_2890, &args![file, buffer, 0x18u32]);
                    if e.call(0x0040_1680, &args![file]).bool() {
                        e.call(0x0069_3dc0, &args![buffer]);
                    }
                    for i in 0..3 {
                        let position = e.mem.u32(buffer.addr() + 4 * i);
                        e.mem.set_u32(me + 0x30 + 4 * i, position);
                        let rotation = e.mem.u32(buffer.addr() + 0xc + 4 * i);
                        e.mem.set_u32(me + 0x24 + 4 * i, rotation);
                    }
                });
            }
        } else if chunk == tag(b"RCLR") {
            // skipped
        } else {
            e.call(MESSAGE, &args![LOAD_TERMINATED_MESSAGE]);
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }

    let list = extra_list(e, me);
    let patrol = e.call(0x0041_fe90, &args![list]).u32();
    if patrol != 0 {
        let script = e.call(0x008d_0430, &args![patrol]).u32();
        if script != 0 {
            let info = e.call(0x0050_0940, &args![script]).u32();
            if e.mem.u32(info + 8) == 0
                && fn_0055b980(e, Ptr::new(script)) != 0
                && !fn_0055b9a0(e, Ptr::new(script))
            {
                e.with_stack(8, |e, text| {
                    e.call(STRING_CONSTRUCT, &args![text]);
                    let name = e.vcall(me, 0x130, &args![]).u32();
                    e.call(
                        STRING_FORMAT,
                        &args![text, PATROL_NOT_COMPILED_FORMAT, name],
                    );
                    let line = e.call(READ_FIRST_DWORD, &args![text]).u32();
                    e.call(MESSAGE, &args![line]);
                    e.call(STRING_DESTRUCT, &args![text]);
                });
            }
        }
    }
    !e.get(data, OBJ_REFR::pObjectReference).is_null()
}

/// `Get3D` (virtual at +0x1d0): the reference's loaded 3D, null when none.
pub(crate) fn get_3d(e: &mut Engine, refr: u32) -> u32 {
    e.vcall(refr, 0x1d0, &args![]).u32()
}

/// The reference's world position as `GetLocationOnReference`-style
/// accessor (virtual at +0x1f4, returns `&data.Location`).
pub(crate) fn location_of(e: &mut Engine, refr: u32) -> u32 {
    e.vcall(refr, 0x1f4, &args![]).u32()
}

/// The tail the collision reset shares between `Copy`'s places: resets the
/// 3D node's simulation (`00c6bd00(node, 1)`), then gives it a zero velocity
/// (`NiPoint3(0, 0, 0)` built at `scratch`, 12 bytes).
pub(crate) fn reset_collision(e: &mut Engine, refr: u32, scratch: u32) {
    let node = get_3d(e, refr);
    e.call(0x00c6_bd00, &args![node, 1u32]);
    e.call(0x0043_d410, &args![scratch, 0.0f32, 0u32, 0u32]);
    let node = get_3d(e, refr);
    e.call(0x00a5_9c60, &args![node, scratch]);
}

/// One pass over the water-light list of `source`'s extra data for `Copy`:
/// every `(light owner, ...)` entry has its light moved: with `remove` the
/// light is taken out of the shadow scene (`ShadowSceneNode::RemoveLight`
/// after a membership test), without it the light is added
/// (`0057c730`). Each entry is finally pointed at `this`
/// (`ExtraDataList::SetWaterLightRef`, `0041f840`, with `remove` as flag).
pub(crate) fn move_water_lights(e: &mut Engine, this: u32, source_list: u32, remove: bool) {
    let list = extra_list(e, source_list);
    let mut node = e.call(0x0041_f810, &args![list]).u32();
    loop {
        if node == 0 {
            break;
        }
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let own_list = extra_list(e, this);
        let light = e.call(0x0041_8250, &args![own_list]).u32();
        if light != 0 && e.call(READ_FIRST_DWORD, &args![light]).u32() != 0 {
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let owner = e.mem.u32(slot);
            if get_3d(e, owner) != 0 {
                let scene = global_ptr(e, GLOBAL_TEMP_REFR_MANAGER);
                if e.call(0x0070_ec90, &args![scene]).u32() != 0 {
                    let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                    let owner = e.mem.u32(slot);
                    let scene_node = e.call(0x0070_ec90, &args![scene]).u32();
                    let attached = e.call(0x004e_8030, &args![scene_node, owner]).u32();
                    let parent = e.call(0x0040_30b0, &args![attached]).u32();
                    if parent != 0 && e.call(0x008d_8520, &args![parent]).u32() == 0xd {
                        let lights = e.call(0x00a5_9d30, &args![attached, 3u32]).u32();
                        let handle = e.call(READ_FIRST_DWORD, &args![light]).u32();
                        e.with_stack(4, |e, slot| {
                            e.mem.set_u32(slot.addr(), handle);
                            if remove {
                                if e.call(0x0049_c680, &args![lights + 0x128, slot, 0u32])
                                    .u32()
                                    == 0
                                {
                                    let handle = e.call(READ_FIRST_DWORD, &args![light]).u32();
                                    e.mem.set_u32(slot.addr(), handle);
                                    e.call(0x004e_d8c0, &args![lights + 0x128, slot]);
                                    e.mem.set_u8(lights + 0x83, 1);
                                }
                                let handle = e.call(READ_FIRST_DWORD, &args![light]).u32();
                                let manager = e.call(0x0045_0b80, &args![0u32]).u32();
                                e.call(0x00b5_eed0, &args![manager, handle]);
                            } else {
                                e.call(0x0057_c730, &args![lights + 0x128, slot]);
                            }
                        });
                    }
                }
            }
        }
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        let item_extra = extra_list(e, item);
        e.call(0x0041_f840, &args![item_extra, this, remove as u32]);
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
}

/// Whether a `float` coordinate is finite and not a NaN, by the CRT's
/// `_finite` (`00ec7595`) and `_isnan` (`00ec75b1`), which take a `double`.
pub(crate) fn is_not_finite(e: &mut Engine, bits: u32) -> bool {
    let value = f32::from_bits(bits) as f64;
    e.call(CRT_FINITE, &args![value]).i32() == 0
}

pub(crate) fn is_nan_float(e: &mut Engine, bits: u32) -> bool {
    let value = f32::from_bits(bits) as f64;
    e.call(CRT_ISNAN, &args![value]).i32() != 0
}

/// `_finite` on all three, then `_isnan` on all three, as `InitItem` tests
/// a position or a rotation; true when any is infinite or a NaN.
pub(crate) fn vector_is_corrupt(e: &mut Engine, at: u32) -> bool {
    let words = [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)];
    words.iter().any(|w| is_not_finite(e, *w)) || words.iter().any(|w| is_nan_float(e, *w))
}

/// Overwrites three dwords at `at` with the zero vector the game keeps at
/// `011f426c`.
pub(crate) fn reset_vector(e: &mut Engine, at: u32) {
    for i in 0..3 {
        let word = e.global::<u32>(ZERO_VECTOR + 4 * i);
        e.mem.set_u32(at + 4 * i, word);
    }
}

/// The error text `InitItem` writes when its warning count went up, in three
/// shapes: the cell is a plain one (`00425fd0` holds), the cell has
/// coordinates in a world, or there is no cell.
pub(crate) fn report_init_item_errors(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    if cell != 0 {
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        if e.call(0x0042_5fd0, &args![cell]).bool() {
            let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
            let base = e.call(GET_BASE_FORM, &args![this]).u32();
            let cell_for_id = e.call(GET_PARENT_CELL, &args![this]).u32();
            let cell_id = e.call(FORM_ID, &args![cell_for_id]).u32();
            let cell_name = e.vcall(cell, 0x130, &args![]).u32();
            let ref_id = e.call(FORM_ID, &args![this]).u32();
            let ref_name = e.call(0x0040_1280, &args![this]).u32();
            let base_again = e.call(GET_BASE_FORM, &args![this]).u32();
            let base_id = e.call(FORM_ID, &args![base_again]).u32();
            let base_name = e.vcall(base, 0x130, &args![]).u32();
            e.call(
                MESSAGE,
                &args![
                    INIT_ITEM_ERROR_CELL,
                    base_name,
                    base_id,
                    ref_name,
                    ref_id,
                    cell_name,
                    cell_id
                ],
            );
            return;
        }
    }
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    if cell != 0 {
        let world = e.call(0x0057_5d70, &args![this]).u32();
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        let world_again = e.call(0x0057_5d70, &args![this]).u32();
        let world_id = e.call(FORM_ID, &args![world_again]).u32();
        let world_name = e.vcall(world, 0x130, &args![]).u32();
        let cell_for_y = e.call(GET_PARENT_CELL, &args![this]).u32();
        let y = e.call(0x0054_4c60, &args![cell_for_y]).u32();
        let cell_for_x = e.call(GET_PARENT_CELL, &args![this]).u32();
        let x = e.call(0x0054_4c30, &args![cell_for_x]).u32();
        let cell_for_id = e.call(GET_PARENT_CELL, &args![this]).u32();
        let cell_id = e.call(FORM_ID, &args![cell_for_id]).u32();
        let cell_name = e.vcall(cell, 0x130, &args![]).u32();
        let ref_id = e.call(FORM_ID, &args![this]).u32();
        let ref_name = e.call(0x0040_1280, &args![this]).u32();
        let base_again = e.call(GET_BASE_FORM, &args![this]).u32();
        let base_id = e.call(FORM_ID, &args![base_again]).u32();
        let base_name = e.vcall(base, 0x130, &args![]).u32();
        e.call(
            MESSAGE,
            &args![
                INIT_ITEM_ERROR_WORLD,
                base_name,
                base_id,
                ref_name,
                ref_id,
                cell_name,
                cell_id,
                x,
                y,
                world_name,
                world_id
            ],
        );
    } else {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        let ref_id = e.call(FORM_ID, &args![this]).u32();
        let ref_name = e.call(0x0040_1280, &args![this]).u32();
        let base_again = e.call(GET_BASE_FORM, &args![this]).u32();
        let base_id = e.call(FORM_ID, &args![base_again]).u32();
        let base_name = e.vcall(base, 0x130, &args![]).u32();
        e.call(
            MESSAGE,
            &args![
                INIT_ITEM_ERROR_NO_CELL,
                base_name,
                base_id,
                ref_name,
                ref_id
            ],
        );
    }
}

// Translated from 00426720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reference-change handler on an `ExtraDataList` (`this`; the map names the
/// body `TESObjectREFR::SetUnderwater (Xbox PDB)`, but it does not touch
/// water, so it is kept as `fn_00426720`). For the change flag `0x20000` and
/// a `target` that is not an actor it fetches the extra of type 0x2b's
/// pointer (`00418460`) and hands it to the `TESForm` helper `00483710`. The
/// other flag test is `flags & 0` and compiled out.
pub fn fn_00426720(e: &mut Engine, this: Ptr, flags: u32, _unused: u32, target: Ptr) {
    let actor = is_actor(e, target.addr());
    if !actor && flags & 0x20000 != 0 {
        let entry = e.call(0x0041_8460, &args![this]).u32();
        e.call(0x0048_3710, &args![entry]);
    }
}

// Translated from 004267d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reference-change handler on an `ExtraDataList` (`this`): for the change
/// `flags`, clears extras of the `target` reference. Seven of its flag tests
/// are `flags & 0` and compiled out.
///
/// * `0x20000` on a non-actor with a teleport extra removes it;
/// * `0x10000000` removes the last finished sequence;
/// * when `0055f5b0` on the save/load object says so, an actor target loses
///   its package, follower and other extras (and the item-dropper link), and
///   any other target hands its dropped item back.
pub fn fn_004267d0(e: &mut Engine, this: Ptr, flags: u32, target: Ptr) {
    let actor = is_actor(e, target.addr());
    if flags & 0x20000 != 0 && !actor {
        let teleport = e.call(0x0041_8460, &args![this]).u32();
        if teleport != 0 {
            e.call(0x0041_ae90, &args![this]);
        }
    }
    if flags & 0x1000_0000 != 0 {
        e.call(0x0042_2920, &args![this]);
    }
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    if e.call(0x0055_f5b0, &args![save_load]).bool() {
        if actor {
            e.call(0x0041_cc60, &args![this]);
            e.call(0x0041_d930, &args![this]);
            e.call(0x0041_b060, &args![this]);
            e.call(0x0042_2720, &args![this]);
            e.call(0x0042_2670, &args![this]);
            e.call(EXTRA_REMOVE_TYPE, &args![this, 0x3au32]);
            e.call(0x0041_9dc0, &args![this, 0u32]);
            e.call(0x0041_aff0, &args![this]);
            e.call(0x0042_e040, &args![this]);
            e.call(0x0042_16b0, &args![this]);
        } else {
            let dropper = e.call(0x0041_de00, &args![this]).u32();
            if dropper != 0 {
                let dropper_extra = extra_list(e, dropper);
                e.call(0x0041_e0d0, &args![dropper_extra, target]);
                e.call(0x0041_de40, &args![this, 0u32]);
            }
        }
    }
}

// Translated from 004269c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Handler on an `ExtraDataList` (`this`) for an actor `target`: if the list
/// has a persistent cell (`0041d460`) and the actor has a parent cell
/// (`008d6f30`), clears the list's persistent cell and removes the actor from
/// that cell.
pub fn fn_004269c0(e: &mut Engine, this: Ptr, _unused: u32, target: Ptr) {
    if !is_actor(e, target.addr()) {
        return;
    }
    let cell = e.call(EXTRA_GET_PERSISTENT_CELL, &args![this]).u32();
    if cell != 0 && e.call(GET_PARENT_CELL, &args![target]).u32() != 0 {
        e.call(EXTRA_SET_PERSISTENT_CELL, &args![this, 0u32]);
        e.call(CELL_REMOVE_REFERENCE, &args![cell, target]);
    }
}

// Translated from 004b0130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<TESObjectREFR *,bool>::DeleteItem` (Xbox PDB): clears the
/// item's `bool` value (+8) and gives the item back to the map's allocator
/// (the pool at `this + 0xC`).
pub fn ni_t_pointer_map_tes_object_refr_p_bool_delete_item(e: &mut Engine, this: Ptr, item: Ptr) {
    e.mem.set_u8(item.addr() + 8, 0);
    e.call(0x0045_cee0, &args![this.addr() + 0xc, item]);
}

// Translated from 0055a2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::TESObjectREFR` (Xbox PDB): constructs the form base and
/// the child-cell base, sets both vtables, constructs the `OBJ_REFR` points
/// and the `ExtraDataList`, sets the form type 0x3a, clears the loaded data
/// pointer, resets the data (`0055ac90`) and clears the base object, parent
/// cell and random sound.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 0055a2f0 - compiler exception-unwind cleanup remains omitted.
pub fn tes_object_refr_tes_object_refr(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
) -> Ptr<TESObjectREFR> {
    e.call(0x0048_3370, &args![this]);
    e.call(0x0053_3240, &args![this.addr() + CHILD_CELL_OFFSET]);
    e.mem.set_u32(this.addr(), REFR_VTABLE);
    e.set(this, TESObjectREFR::vfptrTESChildCell, CHILD_CELL_VTABLE);
    fn_0055a400(e, this.at(TESObjectREFR::data));
    e.call(0x0041_0360, &args![this.addr() + EXTRA_OFFSET]);
    e.call(0x004f_15a0, &args![this, 0x3au32]);
    e.set(this, TESObjectREFR::pLoadedData, Ptr::NULL);
    fn_0055ac90(e, this);
    let data = this.at(TESObjectREFR::data);
    e.set(data, OBJ_REFR::pObjectReference, Ptr::NULL);
    e.set(this, TESObjectREFR::pParentCell, Ptr::NULL);
    e.set(this, TESObjectREFR::pRandomSound, Ptr::NULL);
    this
}

// Translated from 0055a3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual at +0x1e4 (`GetAnimation` in the Xbox PDB order): the animation
/// the reference's `ExtraDataList` holds (`00418220`).
pub fn fn_0055a3b0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let extra = extra_list(e, this.addr());
    e.call(0x0041_8220, &args![extra]).u32()
}

// Translated from 0055a3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::__vecDelDtor` (Xbox PDB), the scalar deleting destructor
/// (virtual at +0x10): runs the destructor (`0055a430`) and, when bit 0 of
/// `flags` is set, frees the object with `operator delete` (`00401030`).
pub fn fn_0055a3d0(e: &mut Engine, this: Ptr<TESObjectREFR>, flags: u32) -> Ptr<TESObjectREFR> {
    fn_0055a430(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0055a400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `OBJ_REFR` constructor: constructs its two `NiPoint3` (at +4 and +0x10,
/// whose constructor `006815c0` does nothing). `this` is the `OBJ_REFR`.
pub fn fn_0055a400(e: &mut Engine, this: Ptr<OBJ_REFR>) -> Ptr<OBJ_REFR> {
    e.call(LIST_ITEM_SLOT, &args![this.addr() + 4]);
    e.call(LIST_ITEM_SLOT, &args![this.addr() + 0x10]);
    this
}

// Translated from 0055a430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The body of `~TESObjectREFR` (the destructor `0055a3d0` calls). Resets the
/// vtables, takes the reference out of everything that tracks it (furniture
/// users, the garbage collector, the terrain tree, water lists, process
/// lists, the player's follower and enemy slots, `ExtraDataList`
/// cross-links, the parent and persistent cells and the navmesh obstacle
/// manager), clears the data (`0055ad20`) and runs the `ExtraDataList` and
/// `TESForm` destructors.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_0055a430(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    e.mem.set_u32(me, REFR_VTABLE);
    e.set(this, TESObjectREFR::vfptrTESChildCell, CHILD_CELL_VTABLE);
    e.call(0x0057_acc0, &args![this]);

    let is_furniture = e.call(0x0056_8680, &args![this]).bool();
    if is_furniture {
        e.call(0x0045_38e0, &args![OBJECT_FURNITURE_MANAGER, 0u32]);
        let stored = e.call(0x0082_5c00, &args![OBJECT_ACTOR_LISTS]).u32();
        e.set_global(GLOBAL_STORED_BY_825C00, stored);
        let mut node = e.call(0x0096_e290, &args![OBJECT_PROCESS_LISTS]).u32();
        while node != 0 {
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let item = e.mem.u32(slot);
            if item != 0 && e.call(0x008d_8520, &args![item]).u32() != 0 {
                let target = e.call(0x008d_8520, &args![item]).u32();
                e.vcall(target, 0x4cc, &args![this]);
            }
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
        e.call(0x0082_f1f0, &args![OBJECT_FURNITURE_MANAGER]);
    }
    if e.call(FORM_TYPE, &args![this]).i32() == 0x27 {
        let stored = e.call(0x0082_5c00, &args![OBJECT_ACTOR_LISTS]).u32();
        e.set_global(GLOBAL_STORED_BY_825C00, stored);
    }
    // GarbageCollector::Remove (cdecl)
    e.call(0x0086_8270, &args![this]);

    if e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        if e.call(0x0054_9580, &args![base]).bool() {
            let world = e.call(0x0057_5d70, &args![this]).u32();
            if world != 0 && e.call(0x0058_6170, &args![world]).u32() != 0 {
                let terrain = e.call(0x0058_6170, &args![world]).u32();
                e.call(0x006f_cf30, &args![terrain, this]);
            }
        }
    }

    let loaded = e.get(this, TESObjectREFR::pLoadedData);
    if !loaded.is_null() {
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        let tes = if cell != 0 {
            let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
            e.call(0x0045_43c0, &args![cell]).u32()
        } else {
            0
        };
        if tes != 0 {
            let phantom = e.call(READ_FIRST_DWORD, &args![loaded.addr() + 0x18]).u32();
            e.call(0x0053_80d0, &args![tes, phantom]);
        }
        let loaded = e.get(this, TESObjectREFR::pLoadedData);
        e.call(0x0066_b0d0, &args![loaded.addr() + 0x18, 0u32]);
    }

    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    let saved_version = e.call(0x008a_81c0, &args![save_load]).u8();
    e.call(0x008a_8150, &args![save_load, 0u32]);

    if !e.call(0x0040_77c0, &args![this]).bool() {
        e.call(0x0057_6760, &args![this]);
        e.call(0x008d_15b0, &args![this]);
        e.call(0x0086_23a0, &args![save_load, this]);
        let data_handler = global_ptr(e, GLOBAL_DATA_HANDLER);
        if !e.call(0x0042_26e0, &args![data_handler]).bool() {
            let enable_parent = e.call(0x0056_a9f0, &args![this]).u32();
            if enable_parent != 0 {
                let extra = extra_list(e, enable_parent);
                e.call(0x0041_dda0, &args![extra, this]);
            }
            let mut node = e.call(0x0056_ac90, &args![this]).u32();
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                let child = e.mem.u32(slot);
                e.call(0x0056_aa10, &args![child, 0u32]);
                node = e.call(0x0056_ac90, &args![this]).u32();
            }
        }
        let extra = extra_list(e, me);
        let dropper = e.call(0x0041_de00, &args![extra]).u32();
        if dropper != 0 {
            let dropper_extra = extra_list(e, dropper);
            e.call(0x0041_e0d0, &args![dropper_extra, this]);
        }
        let extra = extra_list(e, me);
        let mut node = e.call(0x0041_df90, &args![extra]).u32();
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let dropped = e.mem.u32(slot);
            let dropped_extra = extra_list(e, dropped);
            e.call(0x0041_de40, &args![dropped_extra, 0u32]);
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let dropped = e.mem.u32(slot);
            if e.call(0x0056_7790, &args![dropped]).u32() == me {
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                let dropped = e.mem.u32(slot);
                e.call(0x0056_7c50, &args![dropped]);
            }
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
        let data_handler = global_ptr(e, GLOBAL_DATA_HANDLER);
        if !e.call(0x0042_26e0, &args![data_handler]).bool() {
            let extra = extra_list(e, me);
            let ash_pile = e.call(0x0041_e310, &args![extra]).u32();
            if ash_pile != 0 {
                let ash_extra = extra_list(e, ash_pile);
                e.call(0x0041_e340, &args![ash_extra, 0u32]);
            }
            let extra = extra_list(e, me);
            let mut node = e.call(0x0041_f170, &args![extra]).u32();
            while node != 0 {
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                if e.mem.u32(slot) == 0 {
                    break;
                }
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                let entry = e.mem.u32(slot);
                node = e.call(LIST_NEXT, &args![node]).u32();
                let first = e.mem.u32(entry);
                let first_extra = extra_list(e, first);
                e.call(0x0041_f4c0, &args![first_extra, this, 0u32]);
                let first_extra = extra_list(e, first);
                e.call(0x0041_f650, &args![first_extra, this, 0u32]);
            }
            // Script::RemoveDelayedScriptActionReference (cdecl)
            e.call(0x005a_a5d0, &args![this]);
            for getter in [WATER_LIST_A_GETTER, WATER_LIST_B_GETTER] {
                let list = e.call(getter, &args![]).u32();
                e.with_stack(4, |e, local| {
                    e.mem.set_u32(local.addr(), me);
                    e.call(LIST_REMOVE, &args![list, local]);
                });
            }
            if e.call(0x0056_4d80, &args![this]).bool() {
                let kind = e.call(FORM_TYPE, &args![this]).i32();
                let player = global_ptr(e, GLOBAL_PLAYER);
                if (0x3b..=0x3c).contains(&kind) {
                    e.call(0x0096_3d60, &args![player, this]);
                } else {
                    e.call(0x0097_6680, &args![OBJECT_PROCESS_LISTS, this]);
                    e.call(0x0096_3d60, &args![player, this]);
                    e.call(0x0096_f600, &args![OBJECT_PROCESS_LISTS, this, 1u32]);
                    e.call(0x0097_5320, &args![OBJECT_PROCESS_LISTS, this]);
                    e.call(0x0097_8660, &args![OBJECT_PROCESS_LISTS, this]);
                }
            }
            if !e.call(0x0057_25b0, &args![this]).bool() {
                e.call(0x0097_49b0, &args![OBJECT_PROCESS_LISTS, this]);
            }
        }
        if fn_0055abe0(e, this) {
            let player = global_ptr(e, GLOBAL_PLAYER);
            fn_0055ac40(e, Ptr::new(player), this);
        }
        let data_handler = global_ptr(e, GLOBAL_DATA_HANDLER);
        if !e.call(0x0042_26e0, &args![data_handler]).bool()
            && e.call(0x0056_8680, &args![this]).bool()
        {
            let manager = global_ptr(e, GLOBAL_TEMP_REFR_MANAGER);
            let list = fn_0055ac00(e, Ptr::new(manager));
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), me);
                e.call(LIST_REMOVE, &args![list, local]);
            });
        }
        if e.call(0x0045_2370, &args![this]).bool() {
            e.call(0x0047_7410, &args![this]);
        }
    }

    let linked = e.call(0x0056_9b80, &args![this]).u32();
    if linked != 0 {
        let linked_extra = extra_list(e, linked);
        e.call(0x0041_e600, &args![linked_extra, this]);
    }
    let extra = extra_list(e, me);
    let mut node = e.call(0x0041_e500, &args![extra]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let child = e.mem.u32(slot);
        e.call(0x0056_9ba0, &args![child, 0u32]);
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    let extra = extra_list(e, me);
    let mut node = e.call(0x0041_e780, &args![extra]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let entry = e.mem.u32(slot);
        let first = e.mem.u32(entry);
        let first_extra = extra_list(e, first);
        e.call(0x0041_ef20, &args![first_extra, this]);
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    let extra = extra_list(e, me);
    let mut node = e.call(0x0041_eda0, &args![extra]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let entry = e.mem.u32(slot);
        let first = e.mem.u32(entry);
        let first_extra = extra_list(e, first);
        e.call(0x0041_e960, &args![first_extra, this]);
        node = e.call(LIST_NEXT, &args![node]).u32();
    }

    // TESObjectREFR::SetDelete(true), then ClearData (direct, not virtual)
    e.call(0x0056_4930, &args![this, 1u32]);
    tes_object_refr_clear_data(e, this);

    let parent = e.get(this, TESObjectREFR::pParentCell);
    if !parent.is_null() {
        e.call(CELL_REMOVE_REFERENCE, &args![parent, this]);
    }
    if e.call(GET_REF_PERSISTS, &args![this]).bool() {
        let data_handler = global_ptr(e, GLOBAL_DATA_HANDLER);
        if data_handler != 0 && !e.call(0x0042_26e0, &args![data_handler]).bool() {
            let persistent = e.call(0x0057_5dc0, &args![me + CHILD_CELL_OFFSET]).u32();
            if persistent != 0 {
                e.call(CELL_REMOVE_REFERENCE, &args![persistent, this]);
            }
        }
    }
    if e.call(GET_BASE_FORM, &args![this]).u32() != 0 && e.call(0x0056_4bc0, &args![this]).bool() {
        let manager = e.call(0x006c_0720, &args![]).u32();
        e.call(0x006c_0c80, &args![manager, this]);
    }
    if e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        if e.call(FORM_TYPE, &args![base]).i32() == 0x1c {
            let manager = e.call(0x006c_0720, &args![]).u32();
            e.call(0x006c_1060, &args![manager, this]);
        }
    }
    // Interface::... (cdecl)
    e.call(0x0070_5b30, &args![this]);
    if fn_0055ac70(e) == me {
        fn_0055ac80(e);
    }
    let data_handler = global_ptr(e, GLOBAL_DATA_HANDLER);
    if !e.call(0x0042_26e0, &args![data_handler]).bool() {
        if e.call(0x0070_38d0, &args![]).u32() == me {
            // Interface::SetEnemyActor(0) (cdecl)
            e.call(0x0070_38b0, &args![0u32]);
        }
        let player = global_ptr(e, GLOBAL_PLAYER);
        if player != 0 {
            e.call(0x0096_2cd0, &args![player, this]);
        }
    }
    let player = global_ptr(e, GLOBAL_PLAYER);
    if e.call(0x0089_f4e0, &args![player]).u32() == me {
        fn_0055ac20(e, Ptr::new(player));
    }
    e.call(0x008a_8150, &args![save_load, saved_version as u32]);
    e.call(0x0041_03b0, &args![me + EXTRA_OFFSET]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0055abe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x400000` of the form's `iFormFlags` (+0x08) is set.
pub fn fn_0055abe0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    // TESForm::iFormFlags (Xbox PDB) +0x08
    e.mem.u32(this.addr() + 0x08) & 0x40_0000 != 0
}

// Translated from 0055ac00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of the list at +0x94 of the object `this` (the global at
/// `011dea10`; the destructor removes furniture from it).
pub fn fn_0055ac00(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x94)
}

// Translated from 0055ac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the field at +0x638 of `this` (the player; the destructor calls it
/// when the reference held there is the one being destroyed).
pub fn fn_0055ac20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x638, 0);
}

// Translated from 0055ac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// If `item` is not null, removes it from the list at `this + 0x84c`
/// (`this` is the player; `00905330` takes the address of the item slot).
pub fn fn_0055ac40(e: &mut Engine, this: Ptr, item: Ptr<TESObjectREFR>) {
    if item.is_null() {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item.addr());
        e.call(LIST_REMOVE, &args![this.addr() + 0x84c, slot]);
    });
}

// Translated from 0055ac70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reference remembered in the global at `011dea24`.
pub fn fn_0055ac70(e: &mut Engine) -> u32 {
    e.global::<u32>(GLOBAL_LAST_REFR)
}

// Translated from 0055ac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forgets the reference remembered in `011dea24`.
pub fn fn_0055ac80(e: &mut Engine) {
    e.set_global(GLOBAL_LAST_REFR, 0u32);
}

// Translated from 0055ac90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the reference's `OBJ_REFR` data (all but the base object pointer
/// is zeroed), its scale to 1.0 and, if it has loaded data, the water object
/// and water height; then, unless the loading object says the game is
/// loading (`004121b0`), destroys the loaded-data object (`0057c300`).
pub fn fn_0055ac90(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let data = this.at(TESObjectREFR::data);
    let base = e.get(data, OBJ_REFR::pObjectReference);
    e.call(MEMSET, &args![data, 0u32, 0x1cu32]);
    e.set(data, OBJ_REFR::pObjectReference, base);
    e.set(this, TESObjectREFR::fRefScale, 1.0f32);
    let loaded: Ptr<LOADED_REF_DATA> = e.get(this, TESObjectREFR::pLoadedData).cast();
    if !loaded.is_null() {
        e.set(loaded, LOADED_REF_DATA::pCurrentWaterObject, Ptr::NULL);
        let height: f32 = e.global(LOADED_DATA_DEFAULT_HEIGHT);
        e.set(loaded, LOADED_REF_DATA::fRelevantWaterHeight, height);
    }
    let loading = global_ptr(e, GLOBAL_LOADING_OBJECT);
    if loading == 0 || !e.call(LOADING_FLAG, &args![loading]).bool() {
        e.call(0x0057_c300, &args![this]);
    }
}

// Translated from 0055ad20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::ClearData` (Xbox PDB), virtual at +0x18: resets the
/// reference while keeping the extras that matter. Restores the leveled
/// creature base unless loading, kills its light, remembers the persistent
/// cell and the extra of type 0x5b tied to the 3D (taking it out of the
/// list), removes the reference from all water, wipes the extra list
/// (`RemoveAllDefault` when loading, else `RemoveAll`), puts the 0x5b extra
/// and the persistent cell back, drops the 3D (virtual at +0x1cc,
/// `Set3D(0, 1)`) unless loading and finally removes the extra again.
pub fn tes_object_refr_clear_data(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let loading_object = global_ptr(e, GLOBAL_LOADING_OBJECT);
    if loading_object == 0 || !e.call(LOADING_FLAG, &args![loading_object]).bool() {
        e.call(0x0056_b0b0, &args![this]);
    }
    e.call(0x0057_2a50, &args![this, 0u32]);
    let cell = e
        .call(EXTRA_GET_PERSISTENT_CELL, &args![me + EXTRA_OFFSET])
        .u32();
    let loading = e.call(LOADING_FLAG, &args![loading_object]).bool();
    let mut extra = 0;
    if e.vcall(me, 0x1d0, &args![]).u32() != 0 {
        let list = extra_list(e, me);
        extra = e.call(EXTRA_GET_DATA, &args![list, 0x5bu32]).u32();
        if extra != 0 {
            let list = extra_list(e, me);
            e.call(EXTRA_REMOVE, &args![list, extra, 0u32]);
        }
    }
    e.call(0x0057_b520, &args![this, 0u32]);
    if loading {
        e.call(EXTRA_REMOVE_ALL_DEFAULT, &args![me + EXTRA_OFFSET, 1u32]);
    } else {
        e.call(EXTRA_REMOVE_ALL, &args![me + EXTRA_OFFSET, 1u32]);
    }
    if extra != 0 {
        let list = extra_list(e, me);
        e.call(EXTRA_ADD, &args![list, extra]);
    }
    if cell != 0 {
        e.call(EXTRA_SET_PERSISTENT_CELL, &args![me + EXTRA_OFFSET, cell]);
    }
    let loading_object = global_ptr(e, GLOBAL_LOADING_OBJECT);
    if !e.call(LOADING_FLAG, &args![loading_object]).bool() {
        e.vcall(me, 0x1cc, &args![0u32, 1u32]);
    }
    if extra != 0 {
        let list = extra_list(e, me);
        e.call(EXTRA_REMOVE, &args![list, extra, 1u32]);
    }
}

// Translated from 0055ae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::Save` (Xbox PDB), virtual at +0x2c: writes the reference
/// into the open save record. A form flagged deleted (`iFormFlags & 0x20`)
/// is just closed. Otherwise it writes `NAME` (the base object's form ID),
/// the extra data (`00412970`), `XSCL` when the scale is not 1.0, `ONAM` when
/// the extra-data test `00572d30(8)` says so, and `DATA`: position then
/// rotation (24 bytes, byte-swapped around the write when the build swaps
/// endianness).
pub fn tes_object_refr_save(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    e.call(0x0048_55a0, &args![this]);
    if e.call(FORM_IS_DELETED, &args![this]).bool() {
        e.call(0x0048_5680, &args![this]);
        return;
    }
    let data = this.at(TESObjectREFR::data);
    let base = e.get(data, OBJ_REFR::pObjectReference);
    let base_id = e.call(FORM_ID, &args![base]).u32();
    e.call(ADD_CHUNK_VALUE, &args![tag(b"NAME"), base_id]);
    let list = extra_list(e, me);
    e.call(0x0041_2970, &args![list]);
    let scale = e.get(this, TESObjectREFR::fRefScale);
    let one: f64 = e.global(CONSTANT_ONE);
    if scale as f64 != one {
        e.call(ADD_CHUNK_VALUE, &args![tag(b"XSCL"), scale]);
    }
    if e.call(0x0057_2d30, &args![this, 8u32]).bool() {
        e.call(0x0048_56d0, &args![tag(b"ONAM")]);
    }
    e.with_stack(0x18, |e, buffer| {
        e.call(0x0069_2710, &args![buffer]);
        // Position (+0x30) first, then rotation (+0x24), as raw words.
        for i in 0..3 {
            let position = e.mem.u32(me + 0x30 + 4 * i);
            e.mem.set_u32(buffer.addr() + 4 * i, position);
            let rotation = e.mem.u32(me + 0x24 + 4 * i);
            e.mem.set_u32(buffer.addr() + 0xc + 4 * i, rotation);
        }
        if e.call(ENDIAN_SWAP_ON_SAVE, &args![]).bool() {
            e.call(0x0069_3dc0, &args![buffer]);
        }
        e.call(ADD_CHUNK_DATA, &args![tag(b"DATA"), buffer, 0x18u32]);
        if e.call(ENDIAN_SWAP_ON_SAVE, &args![]).bool() {
            e.call(0x0069_3dc0, &args![buffer]);
        }
    });
    e.call(0x0048_5680, &args![this]);
}

// Translated from 0055af90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::Load` (Xbox PDB), virtual at +0x20: reads the reference
/// record from `file`.
///
/// After `TESForm::LoadForm`, a form flagged deleted just drops its extras
/// (keeping the persistent cell) and returns whether it has a base object.
/// Otherwise it reads chunks until the record ends: `NAME` (base object,
/// looked up through the compile index; refused while loading when the
/// existing base is already known; a bad one logs `MASTERFILE: Missing/Invalid
/// base object` and ignores the `DATA` chunk), `XSCL` (scale), `ONAM`,
/// `DATA` (position then rotation, byte-swapped when the build does), `EDID`
/// (editor ID via the virtual at +0x134), `OBND` (virtual at +0xe0) and 76
/// extra-data chunk kinds ([`EXTRA_DATA_CHUNKS`]), which go to
/// `ExtraDataList::Load`. `RCLR` is skipped; an unknown chunk logs `abnormally
/// terminated`. Afterwards a patrol point script that has text but is not
/// compiled is reported. Returns whether the reference has a base object.
///
/// The compiler's exception-unwinding frame is not translated; a local the
/// code sets after `NAME` but never reads is left out. The body is
/// [`load_record`] (a helper kept at the top of the file).
pub fn tes_object_refr_load(e: &mut Engine, this: Ptr<TESObjectREFR>, file: Ptr) -> bool {
    e.with_stack(4, |e, id_slot| {
        load_record(e, this, file.addr(), id_slot.addr())
    })
}

// Translated from 0055b980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at +0x2c of the object `this` (a patrol-point script holder;
/// `Load` tests it for the script's text).
pub fn fn_0055b980(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x2c)
}

// Translated from 0055b9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x2a of the object `this` (whether its script is compiled).
pub fn fn_0055b9a0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x2a) != 0
}

// Translated from 0055b9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual at +0x44 (`PostDuplicateProcess` in the Xbox PDB order): after a
/// reference has been duplicated through the save/load references map `map`
/// (`BGSSaveLoadReferencesMap`), rewires the duplicate's cross references.
/// For this reference's duplicate `target`: its enable parent (and the
/// parent's children), its other linked reference, the two sets of extra
/// references by slot (0 and 1; 0 to 3), the activate-ref children (with
/// their float), the decal refs, the multi-bound reference, and finally the
/// teleport data (door link), whose destination position is the linked
/// door's position plus the offset of this door's teleport point.
pub fn fn_0055b9c0(e: &mut Engine, this: Ptr<TESObjectREFR>, map: Ptr) {
    let me = this.addr();
    let map = map.addr();
    if map == 0 {
        return;
    }
    let old = lookup_new_reference(e, map, me);
    if old == 0 {
        return;
    }
    let target = dynamic_cast(e, old, TYPE_TES_OBJECT_REFR);
    if target == 0 {
        return;
    }

    let enable_parent = e.call(0x0056_a9f0, &args![this]).u32();
    if enable_parent != 0 {
        let parent = resolve_reference(e, map, enable_parent);
        if parent != 0 {
            e.call(0x0056_aa10, &args![target, parent]);
            let flag = e.call(0x0056_aa70, &args![this]).u8();
            e.call(0x0056_aa90, &args![target, flag as u32]);
            let parent_extra = extra_list(e, parent);
            e.call(0x0041_dcd0, &args![parent_extra, target]);
        }
    }

    let linked = e.call(0x0056_9b20, &args![this]).u32();
    if linked != 0 {
        let resolved = resolve_reference(e, map, linked);
        if resolved != 0 {
            e.call(0x0056_9b40, &args![target, resolved]);
        }
    }

    let parent_ref = e.call(0x0056_9b80, &args![this]).u32();
    if parent_ref != 0 {
        let resolved = resolve_reference(e, map, parent_ref);
        if resolved != 0 {
            e.call(0x0056_9ba0, &args![target, resolved]);
            let resolved_extra = extra_list(e, resolved);
            e.call(0x0041_e530, &args![resolved_extra, target]);
        }
    }

    for slot in 0..2u32 {
        let list = extra_list(e, me);
        let old_ref = e.call(0x0042_0bc0, &args![list, slot]).u32();
        if old_ref != 0 {
            let new = lookup_new_reference(e, map, old_ref);
            if new != 0 {
                let resolved = dynamic_cast(e, new, TYPE_TES_OBJECT_REFR);
                if resolved != 0 {
                    let target_extra = extra_list(e, target);
                    e.call(0x0042_0a60, &args![target_extra, slot, resolved]);
                }
            } else {
                // no mapping: the old reference stays
                let target_extra = extra_list(e, target);
                e.call(0x0042_0a60, &args![target_extra, slot, old_ref]);
            }
        }
    }
    for slot in 0..4u32 {
        let list = extra_list(e, me);
        let old_ref = e.call(0x0042_0130, &args![list, slot]).u32();
        if old_ref != 0 {
            let resolved = resolve_reference(e, map, old_ref);
            if resolved != 0 {
                let target_extra = extra_list(e, target);
                e.call(0x0042_0160, &args![target_extra, resolved, slot]);
            }
        }
    }

    let list = extra_list(e, me);
    let mut node = e.call(0x0041_e780, &args![list]).u32();
    let mut any = false;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let entry = e.mem.u32(slot);
        node = e.call(LIST_NEXT, &args![node]).u32();
        let old_child = e.mem.u32(entry);
        let child = resolve_reference(e, map, old_child);
        if child != 0 {
            let target_extra = extra_list(e, target);
            e.call(0x0041_e7f0, &args![target_extra, child]);
            let value = e.mem.f32(entry + 4);
            let target_extra = extra_list(e, target);
            e.call(0x0041_ea30, &args![target_extra, child, value]);
            any = true;
            let child_extra = extra_list(e, child);
            e.call(0x0041_edd0, &args![child_extra, target]);
        }
    }
    if any {
        let list = extra_list(e, me);
        let flag = e.call(0x0041_eb60, &args![list]).u8();
        let target_extra = extra_list(e, target);
        e.call(0x0041_eba0, &args![target_extra, flag as u32]);
    }

    let list = extra_list(e, me);
    let mut node = e.call(0x0041_f050, &args![list]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let entry = e.mem.u32(slot);
        let old_decal = e.mem.u32(entry);
        let decal = resolve_reference(e, map, old_decal);
        if decal != 0 {
            e.call(0x0056_a470, &args![target, decal, entry + 4, entry + 0x10]);
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }

    let bound = e.call(0x0056_a990, &args![this]).u32();
    if bound != 0 {
        let resolved = resolve_reference(e, map, bound);
        if resolved != 0 {
            e.call(0x0056_a9b0, &args![target, resolved]);
        }
    }

    let teleport = e.call(0x0056_8e50, &args![this]).u32();
    if teleport == 0 {
        return;
    }
    let door = e.call(READ_FIRST_DWORD, &args![teleport]).u32();
    if door == 0 {
        return;
    }
    let resolved = resolve_reference(e, map, door);
    if resolved == 0 {
        return;
    }
    let new_teleport = e.call(0x0056_8e70, &args![target]).u32();
    e.call(0x0053_7e90, &args![new_teleport, resolved]);
    let point = e.call(0x0046_0140, &args![teleport]).u32();
    e.with_stack(0x24, |e, frame| {
        let offset = frame.addr();
        let moved = frame.addr() + 0xc;
        let point_copy = frame.addr() + 0x18;
        for i in 0..3 {
            let value = e.mem.u32(point + 4 * i);
            e.mem.set_u32(point_copy + 4 * i, value);
        }
        let door = e.call(READ_FIRST_DWORD, &args![teleport]).u32();
        // NiPoint3 offset = (teleport + 4) - door->Location
        let door_location = e.vcall(door, 0x1f4, &args![]).u32();
        let teleport_base = e.call(0x0071_7e50, &args![teleport]).u32();
        e.call(0x0043_9ef0, &args![teleport_base, offset, door_location]);
        // NiPoint3 moved = resolved->Location + offset
        let location = e.vcall(resolved, 0x1f4, &args![]).u32();
        e.call(0x0043_9e90, &args![location, moved, offset]);
        e.call(0x006f_4ed0, &args![new_teleport, moved]);
        e.call(0x0043_a280, &args![new_teleport, point_copy]);
    });
}

// Translated from 0055c050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::CreateDuplicateForm` (Xbox PDB), virtual at +0x40: sets
/// aside eight of the reference's extras (teleport, enable-state parent,
/// random teleport marker, two kinds of type 0x51, decal refs, merchant
/// container and multi-bound ref), clears its parent cell, duplicates the
/// form through `TESForm::CreateDuplicateForm` (`004867a0`, with the second
/// argument `map`) and puts the extras and the parent cell back. The copy
/// gets its 3D dropped (`Set3D(0, 1)`) and is made non-persistent.
pub fn tes_object_refr_create_duplicate_form(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    _unused: u32,
    map: Ptr,
) -> Ptr<TESObjectREFR> {
    let me = this.addr();
    const SET_ASIDE: [(u32, u32); 8] = [
        (0x2b, TYPE_EXTRA_TELEPORT),
        (0x37, TYPE_EXTRA_ENABLE_STATE_PARENT),
        (0x3b, TYPE_EXTRA_RANDOM_TELEPORT_MARKER),
        (0x51, TYPE_EXTRA_LINKED_REF),
        (0x51, TYPE_EXTRA_ACTIVATE_REF),
        (0x57, TYPE_EXTRA_DECAL_REFS),
        (0x3c, TYPE_EXTRA_MERCHANT_CONTAINER),
        (0x63, TYPE_EXTRA_MULTI_BOUND_REF),
    ];
    let mut held = [0u32; 8];
    for (i, (kind, descriptor)) in SET_ASIDE.iter().enumerate() {
        let list = extra_list(e, me);
        let extra = e.call(EXTRA_GET_DATA, &args![list, *kind]).u32();
        let typed = e
            .call(
                DYNAMIC_CAST,
                &args![extra, 0u32, TYPE_BS_EXTRA_DATA, *descriptor, 0u32],
            )
            .u32();
        held[i] = typed;
        if typed != 0 {
            let list = extra_list(e, me);
            e.call(EXTRA_REMOVE, &args![list, typed, 0u32]);
        }
    }
    let parent_cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    e.vcall(me, 0x228, &args![0u32]);
    let duplicate = e.call(0x0048_67a0, &args![this, 0u32, map]).u32();
    let duplicate = dynamic_cast(e, duplicate, TYPE_TES_OBJECT_REFR);
    for extra in held {
        if extra != 0 {
            let list = extra_list(e, me);
            e.call(EXTRA_ADD, &args![list, extra]);
        }
    }
    e.vcall(me, 0x228, &args![parent_cell]);
    e.vcall(duplicate, 0x1cc, &args![0u32, 1u32]);
    e.call(0x0056_5480, &args![duplicate, 0u32]);
    Ptr::new(duplicate)
}

// Translated from 0055c3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::Copy` (Xbox PDB), virtual at +0x108: makes this
/// reference a copy of the form `source` (cast to `TESObjectREFR`; anything
/// else does nothing).
///
/// It copies the form components, the 0x1c bytes of `OBJ_REFR` data, the
/// persistence flag and the extra-data list (`ExtraDataList::CopyList`),
/// sets the scale, moves the cell registration (`AddReference` to the
/// source's cell, `RemoveReference` from the old one), rebuilds the links
/// the extras hold (enable parent, linked refs, activate-ref children, the
/// water lights, the teleport door pair), and rebuilds the 3D when the
/// copied data differs (`dirty`: another base object, a different scale
/// for a non-actor, a different stack count for ammo, a different radius
/// or another `00569580`).
///
/// `Set3D` (virtual at +0x1cc) is called at `0055cde0` with only one
/// argument pushed: the exe lets the second argument be whatever lay in the
/// stack (the caller's saved `ESI`); the translation passes 0.
pub fn tes_object_refr_copy(e: &mut Engine, this: Ptr<TESObjectREFR>, source: Ptr) {
    let me = this.addr();
    let data = this.at(TESObjectREFR::data);
    let other = dynamic_cast(e, source.addr(), TYPE_TES_OBJECT_REFR);
    if other == 0 {
        return;
    }
    let mut dirty = false;
    if !e.call(0x0040_77c0, &args![this]).bool() {
        let parent = e.call(0x0056_a9f0, &args![this]).u32();
        if parent != 0 {
            let parent = e.call(0x0056_a9f0, &args![this]).u32();
            let parent_extra = extra_list(e, parent);
            e.call(0x0041_dda0, &args![parent_extra, this]);
        }
    }
    let persists = e.call(GET_REF_PERSISTS, &args![other]).u8();
    e.call(0x0056_5480, &args![this, persists as u32]);
    e.call(0x0048_51b0, &args![this, source]);
    if e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
        let mine = e.call(GET_BASE_FORM, &args![this]).u32();
        let theirs = e.call(GET_BASE_FORM, &args![other]).u32();
        if mine != theirs {
            dirty = true;
        }
    }
    let other_data = e.call(GET_DATA, &args![other]).u32();
    e.call(0x0040_1460, &args![data, other_data, 0x1cu32]);

    'compare: {
        let mine = e.call(0x0059_8040, &args![this]).f32() as f64;
        let theirs = e.call(0x0059_8040, &args![other]).f32() as f64;
        if mine != theirs && !is_actor(e, me) {
            dirty = true;
            break 'compare;
        }
        if e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
            let base = e.call(GET_BASE_FORM, &args![this]).u32();
            if e.call(FORM_TYPE, &args![base]).i32() == 0x29 {
                let list = extra_list(e, me);
                let mine = e.call(0x0041_8770, &args![list]).u16() as i16;
                let other_list = extra_list(e, other);
                let theirs = e.call(0x0041_8770, &args![other_list]).u16() as i16;
                if mine != theirs {
                    dirty = true;
                    break 'compare;
                }
            }
        }
        let epsilon: f32 = e.global(RADIUS_EPSILON);
        let their_radius = e.call(0x0056_8cb0, &args![other]).f32();
        let my_radius = e.call(0x0056_8cb0, &args![this]).f32();
        if !e
            .call(0x0040_eb10, &args![my_radius, their_radius, epsilon])
            .bool()
        {
            dirty = true;
            break 'compare;
        }
        let mine = e.call(0x0056_9580, &args![this]).u32();
        let theirs = e.call(0x0056_9580, &args![other]).u32();
        if mine != theirs {
            dirty = true;
        }
    }

    if get_3d(e, me) != 0 {
        let location = location_of(e, me);
        let node = get_3d(e, me);
        e.call(0x0044_0460, &args![node, location]);
        // a 3x3 matrix (0x30 bytes) and a point (12 bytes) on the stack
        e.with_stack(0x30 + 12, |e, frame| {
            let rotation = e.call(0x0056_fa00, &args![this, frame]).u32();
            let node = get_3d(e, me);
            e.call(0x0043_fa80, &args![node, rotation]);
            reset_collision(e, me, frame.addr() + 0x30);
        });
    }

    let their_cell = e.call(GET_PARENT_CELL, &args![other]).u32();
    if their_cell != 0 {
        let cell = e.call(GET_PARENT_CELL, &args![other]).u32();
        e.call(0x0054_8230, &args![cell, this, 0u32]);
        if get_3d(e, me) == 0 {
            let loader = e.global::<u32>(GLOBAL_IO_MANAGER);
            e.call(0x0045_6520, &args![loader]);
        }
    } else if e.call(GET_PARENT_CELL, &args![this]).u32() != 0 {
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        e.call(CELL_REMOVE_REFERENCE, &args![cell, this]);
    }

    let teleport = e.call(0x0056_8e50, &args![this]).u32();
    if teleport != 0 {
        let theirs = e.call(0x0056_8e50, &args![other]).u32();
        let same = theirs != 0
            && e.call(READ_FIRST_DWORD, &args![theirs]).u32()
                == e.call(READ_FIRST_DWORD, &args![teleport]).u32();
        if !same {
            let door = e.call(READ_FIRST_DWORD, &args![teleport]).u32();
            e.call(0x0056_8f30, &args![door]);
            e.call(0x0053_7e90, &args![teleport, 0u32]);
        }
    }

    if e.call(0x0056_9b80, &args![this]).u32() != 0 {
        let parent = e.call(0x0056_9b80, &args![this]).u32();
        let parent_extra = extra_list(e, parent);
        e.call(0x0041_e600, &args![parent_extra, this]);
        e.call(0x0056_9be0, &args![this, 0u32]);
    }

    let list = extra_list(e, me);
    if e.call(0x0041_e780, &args![list]).u32() != 0 {
        let list = extra_list(e, me);
        let mut node = e.call(0x0041_e780, &args![list]).u32();
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let entry = e.mem.u32(slot);
            node = e.call(LIST_NEXT, &args![node]).u32();
            let first = e.mem.u32(entry);
            let first_extra = extra_list(e, first);
            e.call(0x0041_ef20, &args![first_extra, this]);
        }
        e.call(0x0056_9df0, &args![this, 0u32]);
    }

    if e.call(0x0040_77c0, &args![other]).bool() {
        let list = extra_list(e, me);
        let mut node = e.call(0x0041_f170, &args![list]).u32();
        while node != 0 {
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let entry = e.mem.u32(slot);
            let first = e.mem.u32(entry);
            let first_extra = extra_list(e, first);
            e.call(0x0041_f4c0, &args![first_extra, this, 0u32]);
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let entry = e.mem.u32(slot);
            let first = e.mem.u32(entry);
            let first_extra = extra_list(e, first);
            e.call(0x0041_f650, &args![first_extra, this, 0u32]);
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
        move_water_lights(e, me, me, false);
    }

    let other_extra = extra_list(e, other);
    let my_extra = extra_list(e, me);
    e.call(0x0041_1ec0, &args![my_extra, other_extra]);

    if e.call(0x0040_77c0, &args![other]).bool() {
        let other_extra = extra_list(e, other);
        let mut node = e.call(0x0041_f170, &args![other_extra]).u32();
        while node != 0 {
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let entry = e.mem.u32(slot);
            if e.mem.u32(entry + 4) & 1 != 0 {
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                let entry = e.mem.u32(slot);
                let first = e.mem.u32(entry);
                let first_extra = extra_list(e, first);
                e.call(0x0041_f4c0, &args![first_extra, this, 1u32]);
            }
            let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
            let entry = e.mem.u32(slot);
            if e.mem.u32(entry + 4) & 2 != 0 {
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                let entry = e.mem.u32(slot);
                let first = e.mem.u32(entry);
                let first_extra = extra_list(e, first);
                e.call(0x0041_f650, &args![first_extra, this, 1u32]);
            }
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
        move_water_lights(e, me, other, true);
    }

    if e.call(0x0056_9b80, &args![this]).u32() != 0 {
        let parent = e.call(0x0056_9b80, &args![this]).u32();
        let parent_extra = extra_list(e, parent);
        e.call(0x0041_e530, &args![parent_extra, this]);
        e.call(0x0056_9cb0, &args![this, 1u32, 0u32]);
    }

    if !e.call(0x0040_77c0, &args![this]).bool() {
        let list = extra_list(e, me);
        if e.call(0x0041_e780, &args![list]).u32() != 0 {
            let list = extra_list(e, me);
            let mut node = e.call(0x0041_e780, &args![list]).u32();
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                let entry = e.mem.u32(slot);
                node = e.call(LIST_NEXT, &args![node]).u32();
                let first = e.mem.u32(entry);
                let first_extra = extra_list(e, first);
                e.call(0x0041_edd0, &args![first_extra, this]);
            }
            e.call(0x0056_a1e0, &args![this, 1u32]);
        }
    }

    let scale = e.call(0x0059_8040, &args![other]).f32();
    e.call(0x0056_7490, &args![this, scale]);
    if dirty {
        e.vcall(me, 0x1cc, &args![0u32, 1u32]);
    }

    if get_3d(e, other) == 0 {
        e.vcall(me, 0x1cc, &args![0u32, 1u32]);
    } else {
        if e.call(0x0040_77c0, &args![this]).bool() {
            let base = e.get(data, OBJ_REFR::pObjectReference);
            if !base.is_null() && e.call(FORM_TYPE, &args![base]).i32() == 0x25 {
                // 0055cdb9: `PUSH 1; PUSH 0; PUSH this`, then the base's +0x14c, which
                // pops two words (`this`, 0; TESBoundObject 0050ee50 is `RET 8`); the
                // 1 left on the stack is the second word of Set3D (+0x1cc, `RET 8`),
                // after the node pushed at 0055cde0.
                let node = e.vcall(base.addr(), 0x14c, &args![this, 0u32]).u32();
                e.vcall(me, 0x1cc, &args![node, 1u32]);
            }
        }
        let mut placed = false;
        if get_3d(e, me) == 0 && e.call(GET_PARENT_CELL, &args![this]).u32() != 0 {
            let scene = global_ptr(e, GLOBAL_TEMP_REFR_MANAGER);
            let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
            if e.call(0x0045_11e0, &args![scene, cell, 0u32]).bool() {
                let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
                e.call(0x0045_1ef0, &args![scene, this, cell, 0u32, 0u32]);
                placed = true;
            }
        }
        if !placed {
            let base = e.get(data, OBJ_REFR::pObjectReference);
            if base.is_null() || e.call(FORM_TYPE, &args![base]).i32() != 0x25 {
                e.with_stack(0x30 + 12, |e, frame| {
                    let node = e.vcall(me, 0x1c8, &args![0u32]).u32();
                    let location = location_of(e, me);
                    e.call(0x0044_0460, &args![node, location]);
                    let rotation = e.call(0x0056_fa00, &args![this, frame]).u32();
                    e.call(0x0043_fa80, &args![node, rotation]);
                    let scale = e.call(0x0056_7400, &args![this]).f32();
                    e.call(0x0044_0490, &args![node, scale]);
                    let node_of_other = get_3d(e, other);
                    if e.call(0x0096_11e0, &args![node_of_other]).u32() != 0 {
                        let node_of_other = get_3d(e, other);
                        let collision = e.call(0x0096_11e0, &args![node_of_other]).u32();
                        e.vcall(collision, 0xdc, &args![node, 1u32]);
                    }
                });
            }
        }
        if get_3d(e, me) != 0 && e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
            let base = e.call(GET_BASE_FORM, &args![this]).u32();
            if e.call(FORM_TYPE, &args![base]).i32() == 0x1e {
                let node = get_3d(e, me);
                let base = e.call(GET_BASE_FORM, &args![this]).u32();
                e.call(0x0050_d810, &args![base, this, node, 0u32]);
            }
        }
        let open = e.call(0x0057_2d30, &args![this, 8u32]).bool();
        e.call(0x0047_aec0, &args![this, open as u32, 1u32]);
        e.with_stack(0x18, |e, scratch| {
            reset_collision(e, me, scratch.addr());
        });
        let node = get_3d(e, me);
        e.call(0x00a5_a040, &args![node]);
    }

    if e.call(0x0040_77c0, &args![other]).bool() {
        e.call(0x0057_ada0, &args![this]);
    }
    let mut teleport = e.call(0x0056_8e50, &args![this]).u32();
    if teleport != 0 && !e.call(0x0040_77c0, &args![this]).bool() {
        let door = e.call(READ_FIRST_DWORD, &args![teleport]).u32();
        if door == 0 {
            e.call(0x0056_8f30, &args![this]);
            teleport = 0;
        } else {
            if !e.call(GET_REF_PERSISTS, &args![door]).bool() {
                e.call(0x0056_5480, &args![door, 1u32]);
            }
            let mut door_teleport = e.call(0x0056_8e50, &args![door]).u32();
            if door_teleport == 0 {
                door_teleport = e.call(0x0056_8e70, &args![door]).u32();
            }
            e.call(0x0053_7e90, &args![door_teleport, this]);
            let mut removed = false;
            if e.call(0x0056_9140, &args![this]).u32() != 0
                && e.call(0x0056_9140, &args![door]).u32() != 0
            {
                let door_extra = extra_list(e, door);
                e.call(0x0041_ae70, &args![door_extra]);
                removed = true;
            }
            if e.call(0x0056_7770, &args![this]).u32() != 0
                && e.call(0x0056_7770, &args![door]).u32() != 0
            {
                e.call(0x0056_7c50, &args![door]);
                removed = true;
            }
            if removed {
                e.call(MESSAGE, &args![SHARED_DATA_REMOVED_MESSAGE]);
            }
        }
    }
    let _ = teleport;
    if !e.call(0x0040_77c0, &args![this]).bool() {
        let parent = e.call(0x0056_a9f0, &args![this]).u32();
        if parent != 0 {
            let parent = e.call(0x0056_a9f0, &args![this]).u32();
            let parent_extra = extra_list(e, parent);
            e.call(0x0041_dcd0, &args![parent_extra, this]);
        }
    }
}

// Translated from 0055d230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual at +0x10c (`Compare` in the Xbox PDB order): whether `other`
/// differs from this reference. True when `other` is not a reference, when
/// the form components differ (`TESForm::CompareAllComponents`), when the
/// 0x1c bytes of `OBJ_REFR` data differ (`memcmp`), when the parent cells
/// differ, when the extra lists differ (`ExtraDataList::CompareList`) or when
/// the float `00598040` returns (the scale) differs (a NaN counts as
/// different).
pub fn fn_0055d230(e: &mut Engine, this: Ptr<TESObjectREFR>, other: Ptr) -> bool {
    let me = this.addr();
    let other = dynamic_cast(e, other.addr(), TYPE_TES_OBJECT_REFR);
    if other == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, other]).bool() {
        return true;
    }
    let other_data = e.call(GET_DATA, &args![other]).u32();
    if e.call(MEMCMP, &args![me + 0x20, other_data, 0x1cu32]).i32() != 0 {
        return true;
    }
    let mine = e.call(GET_PARENT_CELL, &args![this]).u32();
    let theirs = e.call(GET_PARENT_CELL, &args![other]).u32();
    if mine != theirs {
        return true;
    }
    let other_extra = extra_list(e, other);
    if e.call(0x0041_27e0, &args![me + EXTRA_OFFSET, other_extra])
        .bool()
    {
        return true;
    }
    let mine = e.call(0x0059_8040, &args![this]).f32() as f64;
    let theirs = e.call(0x0059_8040, &args![other]).f32() as f64;
    mine != theirs
}

// Translated from 0055d310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::HasContainer` (Xbox PDB): the container component of the
/// base object, or null. A base object of form type 0x1b (container) gives
/// its component at +0x30; types 0x2a and 0x2b (actor bases) give theirs at
/// +0x64.
pub fn tes_object_refr_has_container(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let mut component = 0;
    if e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        let kind = e.call(FORM_TYPE, &args![base]).i32();
        if kind == 0x1b {
            let base = e.call(GET_BASE_FORM, &args![this]).u32();
            component = if base != 0 { base + 0x30 } else { 0 };
        } else if kind > 0x29 && kind <= 0x2b {
            let base = e.call(GET_BASE_FORM, &args![this]).u32();
            component = if base != 0 { base + 0x64 } else { 0 };
        }
    }
    component
}

// Translated from 0055d3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `form` is this reference's base object or one of the two bases its
/// extra data remembers (`004216f0` and `00421720`, the leveled creature's
/// original base and its template).
pub fn fn_0055d3b0(e: &mut Engine, this: Ptr<TESObjectREFR>, form: Ptr) -> bool {
    let list = extra_list(e, this.addr());
    let original = e.call(0x0042_16f0, &args![list]).u32();
    let list = extra_list(e, this.addr());
    let template = e.call(0x0042_1720, &args![list]).u32();
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    base == form.addr() || original == form.addr() || template == form.addr()
}

// Translated from 0055d420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual at +0xc8 (`SetAltered` in the Xbox PDB order): unless the form is
/// flagged (`004077c0`), with `flag` set it first marks the cell the
/// `TESChildCell` base reports as altered (`SetAltered(true)` on it), then
/// passes `flag` to `TESForm`'s own handler (`00484730`).
pub fn fn_0055d420(e: &mut Engine, this: Ptr<TESObjectREFR>, flag: u8) {
    if !e.call(0x0040_77c0, &args![this]).bool() && flag != 0 {
        let cell = save_parent_cell(e, this);
        if cell != 0 {
            e.vcall(cell, 0xc8, &args![1u32]);
        }
    }
    e.call(0x0048_4730, &args![this, flag as u32]);
}

// Translated from 0055d480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual at +0x130 (`GetObjectTypeName` in the Xbox PDB order): the name of
/// the reference. A reference for which `00474cb0` is true gives
/// `BaseProcess::GetCurrentHeadTrackTypeString`'s body (`00401280`). One
/// without a base object, or while the data handler says so (`004226e0`),
/// gives the empty string. Otherwise it is the base object's own name
/// (virtual +0x130), or, when that is empty, the leveled creature's original
/// base's.
pub fn fn_0055d480(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    if e.call(0x0047_4cb0, &args![this]).u32() != 0 {
        return e.call(0x0040_1280, &args![this]).u32();
    }
    let data = this.at(TESObjectREFR::data);
    let base = e.get(data, OBJ_REFR::pObjectReference);
    let handler = global_ptr(e, GLOBAL_DATA_HANDLER);
    if base.is_null() || e.call(0x0042_26e0, &args![handler]).bool() {
        return EMPTY_STRING;
    }
    let mut name = e.vcall(base.addr(), 0x130, &args![]).u32();
    if name == 0 || e.mem.i8(name) == 0 {
        let list = extra_list(e, this.addr());
        let original = e.call(0x0042_16f0, &args![list]).u32();
        if original != 0 {
            name = e.vcall(original, 0x130, &args![]).u32();
        }
    }
    name
}

// Translated from 0055d520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reference's full name: `TESFullName::GetFullName` of the base object.
pub fn fn_0055d520(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let data = e.call(GET_DATA, &args![this]).u32();
    let base = e.mem.u32(data);
    e.call(0x0048_2720, &args![base]).u32()
}

// Translated from 0055d540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetFormDetailedString` (Xbox PDB), virtual at +0x90:
/// writes `"<kind> Form '<name>' (<id>)<to base...> in Cell <cell string>"`
/// into the `BSStringT` `out`. It builds a first fragment describing the base
/// object (`" to %s form '%s' (%08X)"`), a second describing the world space
/// (`" in WorldSpace '%s' (%08X)"`; formatted but not used in the final
/// text), asks the parent cell (virtual +0x90) for its own string, and
/// formats the lot with `BSStringT::Format`.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn tes_object_refr_get_form_detailed_string(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    out: Ptr,
) {
    // Two 0x110-byte `char` buffers on the stack.
    e.with_stack(0x220, |e, buffers| {
        let first = buffers.addr();
        let second = buffers.addr() + 0x110;
        e.mem.set_u8(first, 0);
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        if base != 0 {
            let base_id = e.call(FORM_ID, &args![base]).u32();
            let name = e.vcall(base, 0x130, &args![]).u32();
            let kind = e.call(0x0044_0e30, &args![base]).u32();
            e.call(
                SPRINTF,
                &args![first, DETAILED_BASE_FORMAT, kind, name, base_id],
            );
        }
        e.mem.set_u8(second, 0);
        let world = e.call(0x0057_5d70, &args![this]).u32();
        if world != 0 {
            let world_id = e.call(FORM_ID, &args![world]).u32();
            let name = e.vcall(world, 0x130, &args![]).u32();
            e.call(
                SPRINTF,
                &args![second, DETAILED_WORLD_FORMAT, name, world_id],
            );
        }
        e.with_stack(8, |e, cell_text| {
            e.call(STRING_CONSTRUCT, &args![cell_text]);
            let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
            if cell != 0 {
                e.vcall(cell, 0x90, &args![cell_text]);
            }
            let cell_string = e.call(READ_FIRST_DWORD, &args![cell_text]).u32();
            let id = e.call(FORM_ID, &args![this]).u32();
            let head_track = e.call(0x0040_1280, &args![this]).u32();
            let kind = e.call(0x0044_0e30, &args![this]).u32();
            e.call(
                STRING_FORMAT,
                &args![
                    out,
                    DETAILED_FORMAT,
                    kind,
                    head_track,
                    id,
                    first,
                    cell_string
                ],
            );
            e.call(STRING_DESTRUCT, &args![cell_text]);
        });
    });
}

// Translated from 0055d6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a method (cdecl, two arguments): if `object` is not null and its
/// virtual at +0x10 gives a non-null `target`, makes the target's flag 9
/// agree with `refr`'s `00565090` test. When the test holds, flag 9 is set on
/// the target and `0054b800` is run on it; when it does not and flag 9 was
/// set, the flag becomes the LOD multiplier of the reference's base
/// (`TES::GetLODMult`, `0045c6b0`) and `00476ab0` runs on the target.
pub fn fn_0055d6d0(e: &mut Engine, refr: Ptr<TESObjectREFR>, object: Ptr) {
    if object.is_null() {
        return;
    }
    let target = e.vcall(object.addr(), 0x10, &args![]).u32();
    if target == 0 {
        return;
    }
    let wanted = e.call(0x0056_5090, &args![refr]).u8();
    let has_flag = (e.call(0x0070_58c0, &args![target]).i32() == 9) as u8;
    if wanted == has_flag {
        return;
    }
    if wanted != 0 {
        e.call(0x0070_5b10, &args![target, 9u32]);
        e.call(0x0054_b800, &args![target]);
    } else {
        let base = e.call(GET_BASE_FORM, &args![refr]).u32();
        let lod = e.call(0x0045_c6b0, &args![base]).u32();
        e.call(0x0070_5b10, &args![target, lod]);
        e.call(0x0047_6ab0, &args![target]);
    }
}

// Translated from 0055d760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::InitItem` (Xbox PDB), virtual at +0x88: finishes loading
/// a reference. Unless `004013e0` says to skip: radio-station activators
/// get their station set up, warnings are counted (`MessageHandler::
/// IncDisableWarningCount`) while it runs, a leveled actor base is reported,
/// a deleted base marks the reference deleted, a corrupt position or rotation
/// (infinite or NaN) is reported and reset to the zero vector, and the
/// `ExtraDataList` is initialized (`00416be0`) with health scaled by the
/// base's health, ammo stacks of one counted from the base, a multi-bound
/// marker kept in step with a non-unit scale (and `TESWorldSpace::
/// AddMultiBoundRef`), actors given their life state, process-list entry,
/// guard flag and the player the camera-speed hook, non-actors told their
/// starting position, the enable-state parent propagated, the rotation
/// clamped, and the `MASTERFILE: Errors were encountered during InitItem`
/// message written (three shapes: with a cell and a world, a cell only, no
/// cell) when warnings were raised meanwhile. Finally it initializes the
/// inventory changes of a container, the script (`InitScript`) and hides the
/// terrain tree of a disabled landscape object.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn tes_object_refr_init_item(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let data = this.at(TESObjectREFR::data);
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    if e.call(0x004f_ede0, &args![base, 0u32]).bool() {
        e.call(0x004f_f150, &args![this]);
    }
    let warnings_before = e.call(0x0046_e8a0, &args![]).u32();
    e.call(0x0043_b2b0, &args![1u32]);
    e.call(0x004f_ffe0, &args![0u32]);

    let base = e.get(data, OBJ_REFR::pObjectReference);
    if !base.is_null() {
        let kind = e.call(FORM_TYPE, &args![base]).i32();
        if (0x2c..=0x2d).contains(&kind) {
            e.call(MESSAGE, &args![LEVELED_BASE_MESSAGE]);
        }
    }
    let base = e.get(data, OBJ_REFR::pObjectReference);
    if !base.is_null() && e.call(FORM_IS_DELETED, &args![base]).bool() {
        e.vcall(me, 0xc4, &args![1u32]);
    }

    if vector_is_corrupt(e, me + 0x30) {
        e.call(MESSAGE, &args![CORRUPT_LOCATION_MESSAGE]);
        reset_vector(e, me + 0x30);
    }
    if vector_is_corrupt(e, me + 0x24) {
        e.call(MESSAGE, &args![CORRUPT_ANGLE_MESSAGE]);
        reset_vector(e, me + 0x24);
    }

    let list = extra_list(e, me);
    e.call(0x0041_6be0, &args![list, this]);
    let list = extra_list(e, me);
    let health_percent = e.call(0x0041_b550, &args![list]).f32();
    let one: f64 = e.global(CONSTANT_ONE);
    if health_percent as f64 != one {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        let health = e.call(0x0048_73d0, &args![base]).u32();
        let health = health as f32;
        let new_health = (health_percent as f64 * health as f64) as f32;
        let list = extra_list(e, me);
        e.call(0x0041_9970, &args![list, new_health]);
    }
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    if e.call(FORM_TYPE, &args![base]).i32() == 0x29 {
        let list = extra_list(e, me);
        if e.call(0x0041_8770, &args![list]).u16() as i16 == 1 {
            let base = e.call(GET_BASE_FORM, &args![this]).u32();
            let count = e.call(FORM_TYPE, &args![base + 0x80]).u32();
            let list = extra_list(e, me);
            e.call(0x0041_9ad0, &args![list, count]);
        }
    }

    if e.call(0x0043_9f90, &args![this]).bool() {
        let scale = e.call(0x0056_7400, &args![this]).f32();
        if scale as f64 != one {
            let mut marker = e.call(0x0056_9840, &args![this]).u32();
            if marker == 0 {
                let block = e.call(0x0040_1000, &args![0xcu32]).u32();
                marker = if block != 0 {
                    e.call(0x0043_8f50, &args![block]).u32()
                } else {
                    0
                };
                let list = extra_list(e, me);
                e.call(0x0042_1f30, &args![list, marker]);
            }
            let position = e.call(LIST_ITEM_SLOT, &args![marker]).u32();
            e.with_stack(12, |e, local| {
                for i in 0..3 {
                    let word = e.mem.u32(position + 4 * i);
                    e.mem.set_u32(local.addr() + 4 * i, word);
                }
                let scale = e.call(0x0056_7400, &args![this]).f32();
                e.call(0x0043_9180, &args![local, scale]);
                e.call(0x0098_ddd0, &args![marker, local]);
            });
            e.call(0x0056_7490, &args![this, 1.0f32]);
        }
        let world = e.call(0x0057_5d70, &args![this]).u32();
        if world != 0 {
            e.call(0x0058_78d0, &args![world, this]);
        }
    }

    let actor = if e.vcall(me, 0x100, &args![]).bool() {
        me
    } else {
        0
    };
    if actor == 0 {
        if e.call(0x0057_2c80, &args![this]).bool() {
            let zero = [
                e.global::<u32>(ZERO_VECTOR),
                e.global::<u32>(ZERO_VECTOR + 4),
                e.global::<u32>(ZERO_VECTOR + 8),
            ];
            e.vcall(me, 0x174, &args![zero[0], zero[1], zero[2]]);
        }
    } else {
        if !e.vcall(actor, 0x290, &args![]).bool() {
            e.vcall(actor, 0x46c, &args![]);
        }
        if !e.vcall(actor, 0x360, &args![]).bool() && e.call(0x0057_22c0, &args![this, 0u32]).bool()
        {
            e.call(0x008a_1800, &args![actor, 2u32]);
            e.call(0x0096_d470, &args![OBJECT_PROCESS_LISTS, actor, 3u32]);
        }
        if e.vcall(actor, 0x218, &args![]).bool() {
            let class = e.call(0x0088_4350, &args![actor]).u32();
            let mut guard = 0u8;
            if class != 0 {
                guard = e.call(0x005f_6e60, &args![class]).u8();
            }
            e.vcall(actor, 0x308, &args![guard as u32]);
        }
        let actor_data = e.call(0x0041_81e0, &args![actor]).u32();
        let base_data = actor_data + 0x30;
        if e.call(0x0047_cdb0, &args![base_data]).bool()
            && !e.call(0x0056_afc0, &args![this]).bool()
        {
            e.call(0x0047_ce10, &args![base_data, this]);
        }
        let flag = fn_0055e200(e, Ptr::new(base_data));
        e.vcall(actor, 0x1f8, &args![flag as u32]);
        let player = global_ptr(e, GLOBAL_PLAYER);
        if actor == player {
            let movement = if player != 0 { player + 0xa4 } else { 0 };
            let speed = e
                .call(
                    0x0064_7f00,
                    &args![movement, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32],
                )
                .f32();
            e.call(0x0055_e230, &args![speed]);
        }
    }

    let saved = {
        let loading = global_ptr(e, GLOBAL_LOADING_OBJECT);
        e.call(0x0046_23f0, &args![loading, 1u32]).u8()
    };
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    let first = e.call(0x0047_c850, &args![save_load]).bool();
    let run = !first || e.call(0x0047_c850, &args![save_load]).bool();
    if run {
        let enable_parent = e.call(0x0056_a9f0, &args![this]).u32();
        if enable_parent != 0 {
            let previous = e.call(0x0047_c850, &args![save_load]).u8();
            if !e.call(0x0047_c850, &args![save_load]).bool() {
                e.call(0x0045_34f0, &args![save_load, 0u32]);
            }
            if e.call(0x0056_aa70, &args![this]).bool() {
                let disabled = e.call(FORM_IS_DISABLED, &args![enable_parent]).bool();
                e.call(0x0048_4af0, &args![this, (!disabled) as u32]);
            } else {
                let disabled = e.call(FORM_IS_DISABLED, &args![enable_parent]).u8();
                e.call(0x0048_4af0, &args![this, disabled as u32]);
            }
            if e.call(FORM_IS_DISABLED, &args![this]).bool()
                && e.call(0x0056_4e60, &args![this]).bool()
                && e.call(GET_PARENT_CELL, &args![this]).u32() != 0
            {
                let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
                fn_0055e1d0(e, Ptr::new(cell));
            }
            e.call(0x0045_34f0, &args![save_load, previous as u32]);
        }
    }
    e.call(0x0056_ac00, &args![this]);
    e.call(
        0x0046_23f0,
        &args![global_ptr(e, GLOBAL_LOADING_OBJECT), saved as u32],
    );
    let base = e.get(data, OBJ_REFR::pObjectReference);
    let destructible = e.call(0x0047_53d0, &args![base]).u8();
    e.call(0x0048_46a0, &args![this, destructible as u32]);
    let angle = e.mem.f32(me + 0x2c);
    let clamped = e.call(0x004b_1480, &args![angle]).f32();
    e.mem.set_f32(me + 0x2c, clamped);
    e.call(0x0048_4ab0, &args![this, 1u32]);
    if e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        if e.call(FORM_TYPE, &args![base]).i32() != 0x28 {
            let list = extra_list(e, me);
            if e.call(EXTRA_GET_DATA, &args![list, 0x6eu32]).u32() != 0 {
                let list = extra_list(e, me);
                e.call(EXTRA_REMOVE_TYPE, &args![list, 0x6eu32]);
            }
        }
    }
    e.call(0x0043_b2b0, &args![0u32]);
    let warnings = e.call(0x0046_e8a0, &args![]).u32();
    e.call(0x004f_ffe0, &args![warnings_before]);
    if warnings != 0 {
        report_init_item_errors(e, this);
    }

    let container = tes_object_refr_has_container(e, this);
    if container != 0 {
        let changes = e.call(0x004b_f220, &args![this]).u32();
        e.call(0x004d_1440, &args![changes]);
        e.call(0x004d_1610, &args![changes]);
        e.call(0x004d_1960, &args![changes]);
        if e.call(0x0042_cde0, &args![changes]).bool() {
            let list = extra_list(e, me);
            e.call(0x0041_aeb0, &args![list]);
        }
    }
    e.call(0x0056_5730, &args![this]);
    e.call(0x0046_a010, &args![this, 0u32]);
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    if e.call(0x0054_9580, &args![base]).bool() && e.call(FORM_IS_DISABLED, &args![this]).bool() {
        let world = e.call(0x0057_5d70, &args![this]).u32();
        if world != 0 {
            let terrain = e.call(0x0058_6170, &args![world]).u32();
            e.call(0x006f_cfa0, &args![terrain, this, 1u32]);
        }
    }
}

// Translated from 0055e1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Decrements the 16-bit counter at +0xa8 of `this` (`InitItem` calls it on
/// the reference's parent cell).
pub fn fn_0055e1d0(e: &mut Engine, this: Ptr) {
    let count = e.mem.u16(this.addr() + 0xa8);
    e.mem.set_u16(this.addr() + 0xa8, count.wrapping_sub(1));
}

// Translated from 0055e200 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `00461580(0x200)` on `this` is false.
pub fn fn_0055e200(e: &mut Engine, this: Ptr) -> bool {
    !e.call(0x0046_1580, &args![this, 0x200u32]).bool()
}

// Translated from 0055e250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual at +0x3c (`SavesBefore` in the Xbox PDB order): whether this
/// reference's record is written before `form`'s in a save.
///
/// A form whose type `005548a0` rejects is left to the parent cell's own
/// virtual +0x3c. Otherwise `form` is cast to the child-cell interface and
/// asked for its cell: if that differs from this reference's cell, the cell
/// decides (virtual +0x3c on this reference's cell, given the other cell).
/// With the same cell, form types 0x3a to 0x40 and 0x69 are before only when
/// this reference persists and `form` (as a reference) does not; types 0x42
/// and 0x43 whenever this one persists; other types never.
pub fn fn_0055e250(e: &mut Engine, this: Ptr<TESObjectREFR>, form: Ptr) -> bool {
    let kind = e.call(FORM_TYPE, &args![form]).u32();
    if !e.call(0x0055_48a0, &args![kind]).bool() {
        let cell = save_parent_cell(e, this);
        return e.vcall(cell, 0x3c, &args![form]).bool();
    }
    let child = dynamic_cast(e, form.addr(), TYPE_TES_CHILD_CELL);
    let their_cell = e.vcall(child, 0, &args![]).u32();
    let my_cell = save_parent_cell(e, this);
    if their_cell != my_cell {
        let cell = save_parent_cell(e, this);
        return e.vcall(cell, 0x3c, &args![their_cell]).bool();
    }
    let kind = e.call(FORM_TYPE, &args![form]).u32();
    match kind {
        0x3a..=0x40 | 0x69 => {
            let as_reference = dynamic_cast(e, form.addr(), TYPE_TES_OBJECT_REFR);
            if !e.call(GET_REF_PERSISTS, &args![this]).bool() {
                false
            } else {
                as_reference != 0 && !e.call(GET_REF_PERSISTS, &args![as_reference]).bool()
            }
        }
        0x42 | 0x43 => e.call(GET_REF_PERSISTS, &args![this]).bool(),
        _ => false,
    }
}

// Translated from 0055e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual at +0x38 (`SavesBefore` in the Xbox PDB order): the same question
/// for a group-data record `group` (the record `fn_0055e5e0` fills; null or a
/// record of another class gives false). For kinds 8 and 9 it is true only
/// when the record's form is this reference's own cell (looked up and cast
/// to `TESObjectCELL`), this reference persists and the kind is 9. Any other
/// kind is left to the parent cell's virtual +0x38.
pub fn fn_0055e3f0(e: &mut Engine, this: Ptr<TESObjectREFR>, group: Ptr) -> bool {
    if group.is_null() || e.mem.u32(group.addr()) != e.global::<u32>(GROUP_DATA_VTABLE_PTR) {
        return false;
    }
    let kind = e.mem.u32(group.addr() + 0xc);
    if (8..=9).contains(&kind) {
        let form_id = e.mem.u32(group.addr() + 8);
        let form = e.call(0x0048_39c0, &args![form_id]).u32();
        let cell = dynamic_cast(e, form, TYPE_TES_OBJECT_CELL);
        if cell == 0 {
            return false;
        }
        if save_parent_cell(e, this) != cell {
            return false;
        }
        if e.call(GET_REF_PERSISTS, &args![this]).bool() {
            return e.mem.u32(group.addr() + 0xc) == 9;
        }
        false
    } else {
        let cell = save_parent_cell(e, this);
        e.vcall(cell, 0x38, &args![group]).bool()
    }
}

// Translated from 0055e4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual at +0x110 (`BelongsInGroup` in the Xbox PDB order): whether this
/// reference belongs in the group-data record `group` (the class
/// `fn_0055e5e0` produces; anything else gives false). `first_flag` and
/// `second_flag` are the caller's two bytes.
///
/// * kinds 6, 8 and 9: kind 6 needs `first_flag`; then the record's form must
///   be the parent cell (`00485be0`). Then it is true when `second_flag` is
///   clear or the kind is 6, and otherwise kind 8 when this reference persists
///   or kind 9 when it does not;
/// * every other kind, with `first_flag` set, is handed to the parent cell's
///   virtual +0x110.
pub fn fn_0055e4d0(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    group: Ptr,
    first_flag: u8,
    second_flag: u8,
) -> bool {
    if group.is_null() || e.mem.u32(group.addr()) != e.global::<u32>(GROUP_DATA_VTABLE_PTR) {
        return false;
    }
    let cell = save_parent_cell(e, this);
    let kind = e.mem.u32(group.addr() + 0xc);
    if kind == 6 || (kind > 7 && kind <= 9) {
        if kind == 6 && first_flag == 0 {
            return false;
        }
        let form_id = e.mem.u32(group.addr() + 8);
        if !e.call(0x0048_5be0, &args![cell, form_id]).bool() {
            return false;
        }
        if second_flag == 0 || e.mem.u32(group.addr() + 0xc) == 6 {
            return true;
        }
        if e.call(GET_REF_PERSISTS, &args![this]).bool() {
            e.mem.u32(group.addr() + 0xc) == 8
        } else {
            e.mem.u32(group.addr() + 0xc) == 9
        }
    } else if first_flag != 0 {
        e.vcall(
            cell,
            0x110,
            &args![group, first_flag as u32, second_flag as u32],
        )
        .bool()
    } else {
        false
    }
}

// Translated from 0055e5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual at +0x114 (`CreateGroupData` in the Xbox PDB order): fills the
/// group-data record `out` (0x14 bytes: vtable, +4, form ID +8, kind +0xc,
/// +0x10) for the group record `parent`. `out[0]` is cleared first. A parent
/// of kind 1 (when `005516c0` holds), 1, 3 or 5 whose form is the parent
/// cell's `00544210` gives kind 6; a parent of kind 6 whose form is this
/// reference's parent cell gives kind 8 when this reference persists, else 9.
pub fn fn_0055e5e0(e: &mut Engine, this: Ptr<TESObjectREFR>, out: Ptr, parent: Ptr) {
    if out.is_null() {
        return;
    }
    e.mem.set_u32(out.addr(), 0);
    if parent.is_null() {
        return;
    }
    let cell = save_parent_cell(e, this);
    let cell_id = e.call(FORM_ID, &args![cell]).u32();
    let fill = |e: &mut Engine, kind: u32| {
        let vtable = e.global::<u32>(GROUP_DATA_VTABLE_PTR);
        e.mem.set_u32(out.addr(), vtable);
        e.mem.set_u32(out.addr() + 0xc, kind);
        e.mem.set_u32(out.addr() + 8, cell_id);
        e.mem.set_u32(out.addr() + 4, 0);
        e.mem.set_u32(out.addr() + 0x10, 0);
    };
    let parent_kind = e.mem.u32(parent.addr() + 0xc).wrapping_sub(1);
    match parent_kind {
        0 | 2 | 4 => {
            let mut matches = false;
            if parent_kind == 0 && e.call(0x0055_16c0, &args![cell]).bool() {
                matches = true;
            }
            let wanted = e.call(0x0054_4210, &args![cell]).u32();
            if e.mem.u32(parent.addr() + 8) == wanted {
                matches = true;
            }
            if matches {
                fill(e, 6);
            }
        }
        5 if e.mem.u32(parent.addr() + 8) == cell_id => {
            let kind = if e.call(GET_REF_PERSISTS, &args![this]).bool() {
                8
            } else {
                9
            };
            fill(e, kind);
        }
        _ => {}
    }
}

// Translated from 0055e730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetSaveSize` (Xbox PDB), virtual at +0x50: the number of
/// bytes `Save` writes for the change flags `flags` (16 bits, wrapping).
/// Starts from `TESForm`'s size (`00484bf0`) and adds: 6 when the game uses
/// save-game blocks, the container changes' size (flag 0x20), the actor
/// extras' size (for an actor), `2 +` `0055f880` (flag 0x10000000 on a
/// non-actor), `2 +` `00560350` (flag 4) and 4 (flag 0x10 with a save
/// version of 0x43 or more). With the debug switch (`011de4e8`'s byte) set it
/// logs the size it computed (`GetSaveSize(): ...`).
pub fn fn_0055e730(e: &mut Engine, this: Ptr<TESObjectREFR>, flags: u32) -> u16 {
    let me = this.addr();
    let base = e.call(0x0048_4bf0, &args![this, flags]).u16();
    let mut total: u16 = base;
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    if e.call(0x0086_2110, &args![save_load]).bool() {
        total = total.wrapping_add(4);
        total = total.wrapping_add(2);
    }
    if flags & 0x20 != 0 {
        let list = extra_list(e, me);
        let changes = e.call(0x0041_8520, &args![list]).u32();
        let size = e.call(0x004d_3960, &args![changes]).u16();
        total = total.wrapping_add(size);
    }
    // (the code also tests `flags & 0`, which is never set)
    if is_actor(e, me) {
        let list = extra_list(e, me);
        let size = e.call(0x0042_2c40, &args![list, flags, this]).u16();
        total = total.wrapping_add(size);
    }
    if flags & 0x1000_0000 != 0 && !is_actor(e, me) {
        total = total.wrapping_add(2);
        let size = e.call(0x0055_f880, &args![this]).u16();
        total = total.wrapping_add(size);
    }
    if flags & 4 != 0 {
        total = total.wrapping_add(2);
        let size = e.call(0x0056_0350, &args![this, 0u32]).u16();
        total = total.wrapping_add(size);
    }
    let version = e.call(0x008d_f040, &args![save_load]).u8();
    if version >= 0x43 && flags & 0x10 != 0 {
        total = total.wrapping_add(4);
    }
    let switch = e.call(0x0040_8d60, &args![GLOBAL_DEBUG_SWITCHES]).u32();
    if e.mem.u8(switch) != 0 {
        let world = e.call(0x004f_d3e0, &args![save_load]).u32();
        let written = (total as u32).wrapping_sub(base as u32);
        if world != 0 {
            let form_id = e.mem.u32(world);
            let form = e.call(0x0048_39c0, &args![form_id]).u32();
            let name = e.vcall(form, 0x130, &args![]).u32();
            let extra = e.mem.u32(world + 5);
            e.call(
                ERROR_LOG,
                &args![
                    SAVE_SIZE_FORMAT,
                    written,
                    form_id,
                    name,
                    extra,
                    SOURCE_LINE,
                    SOURCE_FILE
                ],
            );
        } else {
            e.call(
                ERROR_LOG,
                &args![SAVE_SIZE_SHORT_FORMAT, written, SOURCE_LINE, SOURCE_FILE],
            );
        }
    }
    total
}

// ===========================================================================
// 0055e230 to 005612a0: the running-speed setter, the game-load/save
// handlers and the Havok (rigid body) record of a reference.

/// The `float` global `0055e230` sets (the player's running speed, which
/// the character controller in `crates/physics` reads), and the `double`
/// `0.0` it compares with.
pub(crate) const RUNNING_SPEED: u32 = 0x0126_7bc4;
pub(crate) const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The `double` `-1.0` that marks "use the default blend time", and the
/// `float` default `011c3c08` replaces it with (the callers pass the
/// `float` `-1.0` at `01012054`).
pub(crate) const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
pub(crate) const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
pub(crate) const DEFAULT_BLEND_TIME: u32 = 0x011c_3c08;
/// `FLT_MAX` (`0102f510`), negated for the sequence's time offset.
pub(crate) const FLOAT_MAX: u32 = 0x0102_f510;
/// Doubles and the `float` of `fn_0055ffa0`'s time stepping: the divisor
/// `20.0`, the smallest step as a `double` and as a `float`.
pub(crate) const STEP_DIVISOR: u32 = 0x0102_fc70;
pub(crate) const MIN_STEP_DOUBLE: u32 = 0x0102_fc68;
pub(crate) const MIN_STEP_FLOAT: u32 = 0x0102_fc64;
/// The `float` constants of `fn_005612a0`: `0.5`, `3.0` and `0.0`.
pub(crate) const HALF: u32 = 0x0101_6248;
pub(crate) const THREE: u32 = 0x0101_7718;
pub(crate) const ZERO_FLOAT: u32 = 0x0101_1d78;

/// The game's `TESSaveLoadGame` stream (`this` is the object at
/// [`GLOBAL_SAVE_LOAD`]): write `size` bytes from a buffer
/// (`008579b0`), read `size` bytes into one (`008579e0`), skip `size`
/// bytes (`00857bd0`), the current position, a pointer into the buffer
/// (`00825c00`), and the current save version byte (`008df040`).
pub(crate) const SAVE_WRITE: u32 = 0x0085_79b0;
pub(crate) const SAVE_READ: u32 = 0x0085_79e0;
pub(crate) const SAVE_SKIP: u32 = 0x0085_7bd0;
pub(crate) const SAVE_POSITION: u32 = 0x0082_5c00;
pub(crate) const SAVE_VERSION: u32 = 0x008d_f040;
/// `TESSaveLoadGame::UseSaveGameBlocks` (Xbox PDB).
pub(crate) const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// Returns false in this build (`XOR AL,AL`); the code asks it before
/// anything it keeps only for the Xbox.
pub(crate) const SAVE_LOAD_UNAVAILABLE: u32 = 0x0047_c850;
/// The form header of the form being saved (`[this + 0x88]`,
/// `m_pCurrentlySavingFormHeader`) and being loaded (`[this + 0x84]`,
/// `m_pCurrentlyLoadingFormHeader`); the header holds the form ID at +0,
/// the flags at +5 and the version byte at +9.
pub(crate) const SAVING_FORM_HEADER: u32 = 0x004f_d3e0;
pub(crate) const LOADING_FORM_HEADER: u32 = 0x004f_d3c0;
/// `TESForm::SaveGameDataOLD(buffer, size)` and
/// `TESForm::LoadGameDataOLD(buffer, size)` (Xbox PDB): write or read
/// through the game's stream.
pub(crate) const FORM_SAVE_DATA: u32 = 0x0048_4ce0;
pub(crate) const FORM_LOAD_DATA: u32 = 0x0048_4d00;
/// The form-level step `SaveGame` starts with (`0048xxx` counterpart of
/// `TESForm::LoadGame`, `00484c50`).
pub(crate) const FORM_SAVE_GAME: u32 = 0x0048_4c20;
pub(crate) const FORM_LOAD_GAME: u32 = 0x0048_4c50;
/// `TESForm::SetEmpty(empty)` and `TESForm::SetDisabled(disabled)`
/// (Xbox PDB).
pub(crate) const FORM_SET_EMPTY: u32 = 0x0048_4580;
pub(crate) const FORM_SET_DISABLED: u32 = 0x0048_4af0;
/// `ExtraDataList::SaveGame(flags, reference)` (`004235e0`) and
/// `ExtraDataList::LoadGame(flags, flags2, reference)` (`00424960`).
pub(crate) const EXTRA_SAVE_GAME: u32 = 0x0042_35e0;
pub(crate) const EXTRA_LOAD_GAME: u32 = 0x0042_4960;
/// `ExtraDataList::GetContainerChanges` (Xbox PDB), the save of those
/// changes (`004d3ab0`), and `InventoryChanges::GetInventoryChanges`
/// (Xbox PDB, cdecl: the reference).
pub(crate) const EXTRA_GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
pub(crate) const CONTAINER_CHANGES_SAVE: u32 = 0x004d_3ab0;
pub(crate) const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::LoadGame` (Xbox PDB).
pub(crate) const INVENTORY_CHANGES_LOAD: u32 = 0x004d_3cc0;
/// `BaseExtraList` flag test for `type` (`0041b3a0`, returns whether the
/// list has any of the bits), set (`0041b470`) and clear (`0041b440`).
pub(crate) const EXTRA_FLAG_TEST: u32 = 0x0041_b3a0;
pub(crate) const EXTRA_FLAG_SET: u32 = 0x0041_b470;
pub(crate) const EXTRA_FLAG_CLEAR: u32 = 0x0041_b440;
/// `ExtraDataList::RemoveLastFinishedSequence` (Xbox PDB) and
/// `ExtraDataList::GetLastFinishedSequence` (Xbox PDB).
pub(crate) const EXTRA_REMOVE_LAST_SEQUENCE: u32 = 0x0042_2920;
pub(crate) const EXTRA_GET_LAST_SEQUENCE: u32 = 0x0042_28f0;
/// `TESObjectREFR::RemoveWeapon` (Xbox PDB), and two helpers `LoadGame`
/// calls after it on the same reference.
pub(crate) const REMOVE_WEAPON: u32 = 0x0057_1b50;
pub(crate) const FORM_PREPARE: u32 = 0x0045_34f0;
pub(crate) const FORM_STEP: u32 = 0x0048_3710;
/// The functions of the reference's enable-state parent: `list`'s parent
/// (`0056a9f0`, a form or null) and whether this reference follows its
/// parent's state (`0056aa70`).
pub(crate) const ENABLE_PARENT: u32 = 0x0056_a9f0;
pub(crate) const FOLLOWS_ENABLE_PARENT: u32 = 0x0056_aa70;
/// `ProcessLists::PrintLists` is the name the map gives `008d0600`; the
/// body is `LoadGame`'s/`FinishInitLoadGame`'s first step (called with the
/// two flag words).
pub(crate) const LOAD_FIRST_STEP: u32 = 0x008d_0600;
/// `TESForm::...` flag tests: `IsDisabled`-like (`iFormFlags & 0x800`) is
/// [`FORM_IS_DISABLED`]; the deleted test is [`FORM_IS_DELETED`].
/// The Havok reference's loaded 3D: `0043fcd0(reference)` returns the
/// `NiAVObject` (the thread's cached one when it is the one being loaded,
/// else `pLoadedData->m_spData3D`).
pub(crate) const GET_LOADED_3D: u32 = 0x0043_fcd0;
/// `[this + 0xc]` of a 3D object (its first controller).
pub(crate) const GET_CONTROLLER: u32 = 0x0043_b230;
/// A checked cast: `(type record, object)`, cdecl, null for a null object.
/// With the record at [`MANAGER_TYPE`] it turns a controller into its
/// controller manager, with [`RIGID_BODY_TYPE`] a collision body reference
/// into its rigid body.
pub(crate) const CHECKED_CAST: u32 = 0x0065_3270;
pub(crate) const MANAGER_TYPE: u32 = 0x011f_36ac;
/// `NiControllerManager` methods (the map names two): `GetSequenceAt(i)`
/// (`00495d20`), the count (`00495d00`), `DeactivateAll(time)`
/// (`0048fef0`), `0047aa40(flag)`, `0047aab0(sequence, ...)`, "has
/// sequences to save" (`004f05a0`), and the lookup by `NiFixedString`
/// (`0047a520`).
pub(crate) const MANAGER_SEQUENCE_AT: u32 = 0x0049_5d20;
pub(crate) const MANAGER_SEQUENCE_COUNT: u32 = 0x0049_5d00;
pub(crate) const MANAGER_DEACTIVATE_ALL: u32 = 0x0048_fef0;
pub(crate) const MANAGER_SET_FLAG: u32 = 0x0047_aa40;
pub(crate) const MANAGER_ACTIVATE: u32 = 0x0047_aab0;
pub(crate) const MANAGER_HAS_SEQUENCES: u32 = 0x004f_05a0;
pub(crate) const MANAGER_FIND_SEQUENCE: u32 = 0x0047_a520;
/// `NiFixedString` constructor from a C string (`00438170`, `this`, the
/// text) and destructor (`004381b0`).
pub(crate) const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
pub(crate) const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// Pointers the exe keeps to the names of three sequences: "Unequip"
/// (`01197b5c`) and the entries 0 (`011977d8`) and 2 (`01197820`) of the
/// same table (stride 0x24, a name pointer first).
pub(crate) const NAME_UNEQUIP: u32 = 0x0119_7b5c;
pub(crate) const NAME_SEQUENCE_A: u32 = 0x0119_77d8;
pub(crate) const NAME_SEQUENCE_B: u32 = 0x0119_7820;
/// The table itself (`011977d8`, stride 0x24).
pub(crate) const NAME_TABLE: u32 = 0x0119_77d8;
/// A sequence's methods: its name holder (`00413f40` = `this + 8`),
/// the C string of a name holder (`0043b1b0`), `0044a670` (string
/// length, cdecl), whether it is a generic-location one (`008041a0`,
/// the map's `LowProcess::GetGenericLocation`), `004efb10` (bytes the
/// sequence's data takes), `004efb20(blend)` (saves it), `004efbc0(blend)`
/// (loads it), `004efaa0()` (the size of an empty record) and the time
/// offset (`00759450`, `[this + 0x2c]`, in ST0), `00639aa0` (`[this +
/// 0x48]`, in ST0), `0098adb0(offset)` (sets `[this + 0x48]`).
pub(crate) const SEQUENCE_NAME_HOLDER: u32 = 0x0041_3f40;
pub(crate) const NAME_TEXT: u32 = 0x0043_b1b0;
pub(crate) const STRING_LENGTH: u32 = 0x0044_a670;
pub(crate) const SEQUENCE_IS_GENERIC: u32 = 0x0080_41a0;
pub(crate) const SEQUENCE_SAVE_SIZE: u32 = 0x004e_fb10;
pub(crate) const SEQUENCE_SAVE: u32 = 0x004e_fb20;
pub(crate) const SEQUENCE_LOAD: u32 = 0x004e_fbc0;
pub(crate) const EMPTY_SEQUENCE_SIZE: u32 = 0x004e_faa0;
pub(crate) const SEQUENCE_OFFSET_TIME: u32 = 0x0075_9450;
pub(crate) const SEQUENCE_DURATION: u32 = 0x0063_9aa0;
pub(crate) const SEQUENCE_SET_OFFSET: u32 = 0x0098_adb0;
/// `strcmp`-like compare of two C strings (`00408b20`, cdecl, 0 when equal)
/// and `"Arrow"`.
pub(crate) const STRING_COMPARE: u32 = 0x0040_8b20;
pub(crate) const ARROW: u32 = 0x0102_031c;
/// `0043d410(this, value, a, b)`: the 9-byte record `00a59c60` and
/// `00a59c90` take (a `float` and two bytes), and the two functions that
/// take it on a 3D object.
pub(crate) const MAKE_VELOCITY: u32 = 0x0043_d410;
pub(crate) const SET_3D_VELOCITY: u32 = 0x00a5_9c60;
pub(crate) const ADD_3D_VELOCITY: u32 = 0x00a5_9c90;
/// `NiAVObject`-side functions of the Havok walk: the collision root of a
/// 3D (`004a8b00`), the walker over a node's collision objects
/// (`00c68900(node, visitor, callback)`), and `00c6a350(node, a, b, c, d)`
/// (the map's `bhkWorld::SetMotion`-like call), `00c8f210(node, a, b)`
/// and `00c9b670(node, vector, a, time, b)` (the knock-down), and
/// `00c7c150(controller, 1)` (disable a ragdoll animation).
pub(crate) const COLLISION_ROOT: u32 = 0x004a_8b00;
pub(crate) const WALK_COLLISION: u32 = 0x00c6_8900;
pub(crate) const SET_MOTION: u32 = 0x00c6_a350;
pub(crate) const SET_FIXED: u32 = 0x00c8_f210;
pub(crate) const KNOCK_DOWN: u32 = 0x00c9_b670;
pub(crate) const DISABLE_RAGDOLL_ANIM: u32 = 0x00c7_c150;
pub(crate) const SET_HAVOK_WEAPON: u32 = 0x008a_5f20;
/// Rigid body glue: the entity of a body (`004ae750`), the entity's
/// activity flag writer (`00c9bff0(entity, &flag)`), the body of a
/// collision node (`006fa820`, the first word of its `+0x10`), and the
/// checked cast of that to a rigid body (`00653270` with
/// [`RIGID_BODY_TYPE`]).
pub(crate) const BODY_ENTITY: u32 = 0x004a_e750;
pub(crate) const ENTITY_ACTIVE_FLAG: u32 = 0x00c9_bff0;
pub(crate) const NODE_BODY_REF: u32 = 0x006f_a820;
pub(crate) const RIGID_BODY_TYPE: u32 = 0x0126_81c0;
/// `0043b300(type record, object)`: whether `object` is of that type
/// (cdecl); the two records the Havok callbacks test.
pub(crate) const IS_OF_TYPE: u32 = 0x0043_b300;
pub(crate) const WEAPON_NODE_TYPE: u32 = 0x0126_817c;
pub(crate) const SKIPPED_NODE_TYPE: u32 = 0x011f_9140;
/// `[this + 8]` of a collision node (`0044ddc0`): the object it belongs to.
pub(crate) const NODE_OBJECT: u32 = 0x0044_ddc0;
/// The default constructors the compiler calls on a stack `NiPoint3` or
/// `hkVector4` (`006815c0`, which just returns `this`) and on a quaternion
/// (`006240d0`).
pub(crate) const VECTOR_CONSTRUCT: u32 = LIST_ITEM_SLOT;
pub(crate) const QUATERNION_CONSTRUCT: u32 = 0x0062_40d0;
/// `hkVector4` to `NiPoint3` (`00458620(dest, source)`, cdecl),
/// `NiPoint3` to `hkVector4` (`004a3e00(dest, source)`, cdecl),
/// `hkVector4` to `NiPoint3` for velocities (`004a3970(dest, source)`),
/// `NiQuaternion` to `hkQuaternion` (`00561500(dest, source)`, cdecl)
/// and the 16-byte copy (`004a3c90(dest, source)`).
pub(crate) const VECTOR_TO_NI: u32 = 0x0045_8620;
pub(crate) const NI_TO_VECTOR: u32 = 0x004a_3e00;
pub(crate) const VECTOR_TO_NI_VELOCITY: u32 = 0x004a_3970;
pub(crate) const NI_QUATERNION_TO_HAVOK: u32 = 0x0056_1500;
pub(crate) const COPY_16_BYTES: u32 = 0x004a_3c90;
/// A rigid body's linear/angular velocity pieces: `009d9f40` and
/// `0045c650`, and the zero vector `00458b20` returns (`01267e30`).
pub(crate) const BODY_VECTOR_PART: u32 = 0x009d_9f40;
pub(crate) const BODY_VECTOR_FINISH: u32 = 0x0045_c650;
pub(crate) const ZERO_VECTOR_GETTER: u32 = 0x0045_8b20;
/// The quaternion setters `00560c80` calls on its destination.
pub(crate) const QUATERNION_SET_0: u32 = 0x004f_5d90;
pub(crate) const QUATERNION_SET_2: u32 = 0x0063_f790;
pub(crate) const QUATERNION_SET_3: u32 = 0x0052_ce20;
/// The rigid-body functions the Havok callbacks use (a flag setter and two
/// vector setters; the save writes the two vectors in the order the load
/// reads them back):
/// `00561580(body, flag)`, `005615d0(body, vector)`, `00561630(body,
/// vector)`, the load handler for older saves (`00561860`) and
/// `00496080(x, 0x14, time)` / `004964d0(x)` (the animation group clear).
pub(crate) const BODY_FLAG_SETTER: u32 = 0x0056_1580;
pub(crate) const BODY_SET_FIRST_VECTOR: u32 = 0x0056_15d0;
pub(crate) const BODY_SET_SECOND_VECTOR: u32 = 0x0056_1630;
pub(crate) const LOAD_OLD_HAVOK: u32 = 0x0056_1860;
pub(crate) const CLEAR_ANIM_GROUP: u32 = 0x0049_6080;
pub(crate) const ANIM_FINISH: u32 = 0x0049_64d0;
/// `NiQuaternion::FromRotation(matrix)` (`00a6df40`, `this` the
/// quaternion) and `TESObjectREFR::GetOrientation(matrix)` (`0056fa00`).
pub(crate) const QUATERNION_FROM_ROTATION: u32 = 0x00a6_df40;
pub(crate) const GET_ORIENTATION: u32 = 0x0056_fa00;
/// The reference's location accessor, set functions of the 3D
/// (`00440460(node, location)`, `0043fa80(node, orientation)`),
/// `bhkWorld::UpdatePosition(node, a, b)` (`00c69f50`, cdecl) and
/// `0046a010(reference, flag)`.
pub(crate) const SET_3D_LOCATION: u32 = 0x0044_0460;
pub(crate) const SET_3D_ORIENTATION: u32 = 0x0043_fa80;
pub(crate) const UPDATE_POSITION: u32 = 0x00c6_9f50;
pub(crate) const FINISH_INIT_LAST: u32 = 0x0046_a010;
/// `TESObjectREFR` handlers `fn_0055f240` calls: `00574920(this, flag)`,
/// `BGSOpenCloseForm::IsOpenCloseForm(form)` (`0047a490`, cdecl),
/// `BGSOpenCloseForm::SetOpenState(reference, state, flag)`
/// (`0047aec0`, cdecl), the open-state getter (`00572d30(this, 8)`),
/// the "was in the middle of a transition" test (`00632ce0`),
/// `TESObjectREFR::RestoreRagDollData` (`00577330`), `00579ac0`, `00477ba0`.
pub(crate) const RESET_STATE: u32 = 0x0057_4920;
pub(crate) const IS_OPEN_CLOSE_FORM: u32 = 0x0047_a490;
pub(crate) const SET_OPEN_STATE: u32 = 0x0047_aec0;
pub(crate) const GET_OPEN_STATE: u32 = 0x0057_2d30;
pub(crate) const SAVE_LOAD_TEST_632CE0: u32 = 0x0063_2ce0;
pub(crate) const RESTORE_RAGDOLL_DATA: u32 = 0x0057_7330;
pub(crate) const REFR_DISABLE_FIX: u32 = 0x0057_9ac0;
pub(crate) const FORM_IS_KIND_477BA0: u32 = 0x0047_7ba0;
/// `0055f970`'s other callees: the head-controller test `004b5bf0`
/// (`HasMorpherController`, cdecl), `0040_4dc0(sequence name, "Unequip")`
/// (cdecl compare) and `004b5c80(object)` (cdecl, bool).
pub(crate) const HAS_MORPHER_CONTROLLER: u32 = 0x004b_5bf0;
pub(crate) const SEQUENCE_NAME_COMPARE: u32 = 0x0040_4dc0;
pub(crate) const IS_ACTOR_3D: u32 = 0x004b_5c80;
/// `0042_6020(list, flags, flags2, reference, base)`: the extra-data
/// handler `fn_0055f5f0` ends with.
pub(crate) const EXTRA_AFTER_LOAD: u32 = 0x0042_6020;
/// `InventoryChanges` handlers `fn_0055f5f0` and `FinishInitLoadGame`
/// call on the changes.
pub(crate) const INVENTORY_CHANGES_STEP_A: u32 = 0x004d_4030;
pub(crate) const INVENTORY_CHANGES_STEP_B: u32 = 0x004d_1960;
/// `00567490(reference, scale)`: the scale setter `LoadGame` calls.
pub(crate) const SET_SCALE: u32 = 0x0056_7490;
/// `0085f2b0(game, reference, size)` and `0085f5a0(game, reference,
/// size)`: the loaders of the older `0x10000000` and `4` records.
pub(crate) const LOAD_OLD_RECORD_A: u32 = 0x0085_f2b0;
pub(crate) const LOAD_OLD_RECORD_B: u32 = 0x0085_f5a0;

/// The tag the save game writes ahead of a reference's block: the constant
/// `0x424c4f4b` ("BLOK" read as a big-endian constant), which the file holds
/// as the bytes `KOLB`.
pub(crate) const BLOCK_TAG: u32 = tag(b"KOLB");
/// The source line the save and load of this unit report.
pub(crate) const SAVE_GAME_LINE: u32 = 0x0b01;
pub(crate) const LOAD_GAME_HEADER_LINE: u32 = 0x0b10;
pub(crate) const LOAD_GAME_END_LINE: u32 = 0x0b8f;
/// Message formats (exe strings) of `SaveGame`, `LoadGame` and the Havok
/// load.
pub(crate) const SAVE_GAME_FORMAT: u32 = 0x0101_53a0;
pub(crate) const SAVE_GAME_SHORT_FORMAT: u32 = 0x0101_536c;
pub(crate) const SAVE_BLOCK_TOO_LARGE_MESSAGE: u32 = 0x0101_5318;
pub(crate) const BLOCK_HEADER_FORM_FORMAT: u32 = 0x0101_5718;
pub(crate) const BLOCK_HEADER_FORMAT: u32 = 0x0101_56a8;
pub(crate) const LOAD_OVERRUN_FORM_FORMAT: u32 = 0x0101_5588;
pub(crate) const LOAD_UNDERRUN_FORM_FORMAT: u32 = 0x0101_5500;
pub(crate) const LOAD_OVERRUN_FORMAT: u32 = 0x0101_54a0;
pub(crate) const LOAD_UNDERRUN_FORMAT: u32 = 0x0101_5440;
pub(crate) const HAVOK_NO_3D_FORMAT: u32 = 0x0102_fc78;
pub(crate) const HAVOK_BONE_COUNT_FORMAT: u32 = 0x0102_fd38;
pub(crate) const HAVOK_WEAPON_BONE_FORMAT: u32 = 0x0102_fcc8;
pub(crate) const TRUE_TEXT: u32 = 0x0102_fd2c;
pub(crate) const FALSE_TEXT: u32 = 0x0102_fd24;

layout! {
    /// `HavokSaveData` (Xbox PDB), 0x14 bytes: what the save and load of a
    /// reference's rigid bodies keep while they walk its collision nodes.
    /// `cFlags`: 1 = active bodies only, 2 = the collision root's own body
    /// was seen, 4 = both active and inactive bodies, 8 = a node of the
    /// type at `0126817c` was seen (the load message calls it the weapon
    /// bone).
    pub struct HavokSaveData: 0x14 {
        /// `cFlags` (Xbox PDB).
        0x00 cFlags: u8,
        /// `sActiveBoneCount` (Xbox PDB).
        0x02 sActiveBoneCount: u16,
        /// `sInactiveBoneCount` (Xbox PDB).
        0x04 sInactiveBoneCount: u16,
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x08 pRef: Ptr,
        /// `pObj3D` (Xbox PDB): `NiAVObject*`.
        0x0C pObj3D: Ptr,
        /// `pCollisionRoot` (Xbox PDB): `bhkNiCollisionObject*`.
        0x10 pCollisionRoot: Ptr,
    }

    /// The 0x10-byte record the game builds on its stack and hands to the
    /// walker over a node's collision objects (`00c68900`): it descends
    /// into the children when `bRecurse` is set, tests `iMode` only for
    /// zero, and passes the whole record to the callback. Not in the Xbox
    /// PDB (a local type); the names describe the use.
    pub struct CollisionWalkContext: 0x10 {
        /// Non-zero: descend into the children.
        0x04 bRecurse: u8,
        /// `0x12` in every use here.
        0x08 iMode: u32,
        /// The callback's own data (a [`HavokSaveData`]).
        0x0C pUserData: Ptr,
    }
}

/// The reference's loaded 3D (`NiAVObject*`), null when it has none.
pub(crate) fn loaded_3d(e: &mut Engine, refr: u32) -> u32 {
    e.call(GET_LOADED_3D, &args![refr]).u32()
}

/// The first controller of a 3D object (`[obj + 0xc]`).
pub(crate) fn controller_of(e: &mut Engine, obj3d: u32) -> u32 {
    e.call(GET_CONTROLLER, &args![obj3d]).u32()
}

/// The controller manager the 3D's first controller casts to, or 0.
pub(crate) fn manager_of(e: &mut Engine, controller: u32) -> u32 {
    e.call(CHECKED_CAST, &args![MANAGER_TYPE, controller]).u32()
}

/// `manager_of` for a reference whose 3D has a controller; 0 otherwise
/// (the lookup `fn_0055f880` and `fn_0055f900` start with).
pub(crate) fn manager_of_reference(e: &mut Engine, this: u32) -> u32 {
    let obj3d = loaded_3d(e, this);
    let mut manager = 0;
    if obj3d != 0 && controller_of(e, obj3d) != 0 {
        let controller = controller_of(e, obj3d);
        manager = manager_of(e, controller);
    }
    manager
}

/// The C string of a sequence's name (`0043b1b0(00413f40(sequence))`).
pub(crate) fn sequence_name(e: &mut Engine, sequence: u32) -> u32 {
    let holder = e.call(SEQUENCE_NAME_HOLDER, &args![sequence]).u32();
    e.call(NAME_TEXT, &args![holder]).u32()
}

/// The sequence of `manager` whose name is the C string at the pointer
/// stored in the global `name_global`: builds the `NiFixedString`, looks
/// it up (`0047a520`) and destroys it.
pub(crate) fn find_sequence(e: &mut Engine, manager: u32, name_global: u32) -> u32 {
    let name = e.global::<u32>(name_global);
    e.with_stack(4, |e, fixed| {
        let handle = e.call(FIXED_STRING_CONSTRUCT, &args![fixed, name]).u32();
        let sequence = e.call(MANAGER_FIND_SEQUENCE, &args![manager, handle]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
        sequence
    })
}

/// The `float` result of a sequence getter in ST0.
pub(crate) fn float_of(e: &mut Engine, addr: u32, object: u32) -> f32 {
    e.call(addr, &args![object]).f32()
}

/// Gives a 3D object its velocity record: `0043d410(record, value, 1, 0)`
/// (`value` is the sequence's offset time) then `00a59c60(node, record)`.
pub(crate) fn set_velocity_record(e: &mut Engine, obj3d: u32, value: f32, a: u32, b: u32) {
    e.with_stack(0xc, |e, record| {
        e.call(MAKE_VELOCITY, &args![record, value, a, b]);
        e.call(SET_3D_VELOCITY, &args![obj3d, record]);
    });
}

/// Moved to the core (the Havok crate calls it too); here under its old path.
pub(crate) use engine_core::x87::rsqrt_estimate;

/// Drops the weapon of an actor that has one drawn: the sequence both
/// `LoadGame` and `fn_0055f240` run when the container changed.
pub(crate) fn drop_drawn_weapon(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    if is_actor(e, me) && e.vcall(me, 0x21c, &args![]).bool() {
        e.call(REMOVE_WEAPON, &args![this]);
        e.call(FORM_PREPARE, &args![this, 1u32]);
        e.call(FORM_STEP, &args![this]);
        e.call(FORM_STEP, &args![this]);
    }
}

/// Sets the reference's disabled state from its enable-state parent
/// `parent` (`0056a9f0`'s result): the same as the parent's, or the
/// opposite when the reference follows it inverted (`0056aa70`).
pub(crate) fn follow_enable_parent(e: &mut Engine, this: Ptr<TESObjectREFR>, parent: u32) {
    if e.call(FOLLOWS_ENABLE_PARENT, &args![this]).bool() {
        let disabled = e.call(FORM_IS_DISABLED, &args![parent]).bool();
        e.call(FORM_SET_DISABLED, &args![this, !disabled]);
    } else {
        let disabled = e.call(FORM_IS_DISABLED, &args![parent]).bool();
        e.call(FORM_SET_DISABLED, &args![this, disabled]);
    }
}

/// The open/close extra flag step `LoadGame`'s sibling `fn_0055f240` and
/// `LoadGame` share: if the list has bit 8, sets it (`0041b470`), else
/// clears it (`0041b440`).
pub(crate) fn copy_open_state_flag(e: &mut Engine, me: u32) {
    let list = extra_list(e, me);
    if e.call(EXTRA_FLAG_TEST, &args![list, 8u32]).bool() {
        let list = extra_list(e, me);
        e.call(EXTRA_FLAG_SET, &args![list, 8u32]);
    } else {
        let list = extra_list(e, me);
        e.call(EXTRA_FLAG_CLEAR, &args![list, 8u32]);
    }
}

// Translated from 0055e230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Keeps the player's running speed: stores `speed` in the `float` global
/// `01267bc4` (read by the character controller, see `crates/physics`)
/// unless it is not above zero.
pub fn fn_0055e230(e: &mut Engine, speed: f32) {
    let zero: f64 = e.global(ZERO_DOUBLE);
    if speed as f64 > zero {
        e.set_global(RUNNING_SPEED, speed);
    }
}

// Translated from 0055e940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SaveGame` (Xbox PDB), virtual: writes the change flags
/// `flags` of the reference into the save game.
///
/// Starts with the form's own step (`00484c20`). With save blocks
/// (`TESSaveLoadGame::UseSaveGameBlocks`) it frames its data in a block:
/// the tag `BLOK`, then a 16-bit size that it patches at the end (and
/// reports, as a message, when the block is larger than 0xFFFF bytes).
/// In the block: the container changes (flag 0x20), the extra data of an
/// actor, the controller data (flag 0x10000000, non-actors) with its
/// 16-bit size, the Havok record (flag 4) with its size, and the scale
/// (flag 0x10, version 0x43 or later). With the debug switch set it logs
/// the size it wrote (`SaveGame(): ...`).
pub fn tes_object_refr_save_game(e: &mut Engine, this: Ptr<TESObjectREFR>, flags: u32) {
    let me = this.addr();
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    e.call(FORM_SAVE_GAME, &args![this, flags]);
    e.with_stack(0x30, |e, frame| {
        // The game's locals: the block size, the tag, a size word and the
        // Havok record.
        let size_slot = frame.addr();
        let tag_slot = frame.addr() + 4;
        let word_slot = frame.addr() + 8;
        let record: Ptr<HavokSaveData> = Ptr::new(frame.addr() + 0x10);
        let mut size_field = 0u32;
        let mut entry_position = e.call(SAVE_POSITION, &args![save_load]).u32();
        let switches = e.call(0x0040_8d60, &args![GLOBAL_DEBUG_SWITCHES]).u32();
        if e.mem.u8(switches) != 0 {
            entry_position = e.call(SAVE_POSITION, &args![save_load]).u32();
        }
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
            e.mem.set_u32(tag_slot, BLOCK_TAG);
            e.call(SAVE_WRITE, &args![save_load, tag_slot, 4u32]);
            size_field = e.call(SAVE_POSITION, &args![save_load]).u32();
            e.call(SAVE_WRITE, &args![save_load, size_slot, 2u32]);
        }
        if flags & 0x20 != 0 {
            let list = extra_list(e, me);
            let changes = e.call(EXTRA_GET_CONTAINER_CHANGES, &args![list]).u32();
            e.call(CONTAINER_CHANGES_SAVE, &args![changes]);
        }
        // (the code also tests `flags & 0`, which is never set)
        if is_actor(e, me) {
            let list = extra_list(e, me);
            e.call(EXTRA_SAVE_GAME, &args![list, flags, this]);
        }
        if flags & 0x1000_0000 != 0 && !is_actor(e, me) {
            let size = fn_0055f880(e, this);
            e.mem.set_u16(word_slot, size);
            e.call(FORM_SAVE_DATA, &args![this, word_slot, 2u32]);
            if e.mem.u16(word_slot) != 0 {
                fn_0055f900(e, this);
            }
        }
        if flags & 4 != 0 {
            fn_0055ebf0(e, record);
            let size = fn_00560350(e, this, record);
            e.mem.set_u16(word_slot, size);
            e.call(FORM_SAVE_DATA, &args![this, word_slot, 2u32]);
            if e.mem.u16(word_slot) != 0 {
                fn_005604b0(e, this, record);
            }
        }
        if e.call(SAVE_VERSION, &args![save_load]).u8() >= 0x43 && flags & 0x10 != 0 {
            e.call(
                FORM_SAVE_DATA,
                &args![this, me + TESObjectREFR::fRefScale.off, 4u32],
            );
        }
        let switches = e.call(0x0040_8d60, &args![GLOBAL_DEBUG_SWITCHES]).u32();
        if e.mem.u8(switches) != 0 {
            let position = e.call(SAVE_POSITION, &args![save_load]).u32();
            let header = e.call(SAVING_FORM_HEADER, &args![save_load]).u32();
            let written = position.wrapping_sub(entry_position);
            if header != 0 {
                let form_id = e.mem.u32(header);
                let form = e.call(0x0048_39c0, &args![form_id]).u32();
                let name = e.vcall(form, 0x130, &args![]).u32();
                let extra = e.mem.u32(header + 5);
                e.call(
                    ERROR_LOG,
                    &args![
                        SAVE_GAME_FORMAT,
                        written,
                        form_id,
                        name,
                        extra,
                        SAVE_GAME_LINE,
                        SOURCE_FILE
                    ],
                );
            } else {
                e.call(
                    ERROR_LOG,
                    &args![SAVE_GAME_SHORT_FORMAT, written, SAVE_GAME_LINE, SOURCE_FILE],
                );
            }
        }
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
            let position = e.call(SAVE_POSITION, &args![save_load]).u32();
            if position > size_field.wrapping_add(0xffff) {
                e.call(
                    MESSAGE,
                    &args![SAVE_BLOCK_TOO_LARGE_MESSAGE, SOURCE_FILE, SAVE_GAME_LINE],
                );
            }
            e.mem
                .set_u16(size_field, position.wrapping_sub(size_field) as u16);
        }
    });
}

// Translated from 0055ebf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HavokSaveData` constructor: clears the flags, both counts and the
/// three pointers; returns `this`.
pub fn fn_0055ebf0(e: &mut Engine, this: Ptr<HavokSaveData>) -> Ptr<HavokSaveData> {
    e.set(this, HavokSaveData::cFlags, 0);
    e.set(this, HavokSaveData::sActiveBoneCount, 0);
    e.set(this, HavokSaveData::sInactiveBoneCount, 0);
    e.set(this, HavokSaveData::pRef, Ptr::NULL);
    e.set(this, HavokSaveData::pObj3D, Ptr::NULL);
    e.set(this, HavokSaveData::pCollisionRoot, Ptr::NULL);
    this
}

// Translated from 0055ec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::LoadGame` (Xbox PDB), virtual: reads the change flags
/// `flags` of the reference back from the save game (`flags2` is the
/// second flag word the callers pass).
///
/// Reads and checks the `BLOK` block header when save blocks are in use
/// (messages `SAVELOAD: (LoadGame Buffer error) ...` on a bad tag), then
/// follows the order of `SaveGame`: the base form step, the disabled
/// state from the enable-state parent, the container changes, the extra
/// data, the controller data, the Havok record, the scale and the
/// open/close flags. It ends by clearing the loaded water data and, with
/// blocks, reports a read that ended before (underrun) or after (overrun)
/// the block's recorded size.
pub fn tes_object_refr_load_game(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    flags: u32,
    flags2: u32,
) {
    let me = this.addr();
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    e.call(FORM_LOAD_GAME, &args![this, flags, flags2]);
    if !is_actor(e, me) && flags & 0x20_0000 != 0 {
        e.call(FORM_SET_EMPTY, &args![this, 1u32]);
    }
    e.with_stack(0x10, |e, frame| {
        let tag_slot = frame.addr();
        let size_slot = frame.addr() + 4;
        let word_slot = frame.addr() + 8;
        e.mem.set_u16(size_slot, 0);
        let mut block_start = 0u32;
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
            e.call(SAVE_READ, &args![save_load, tag_slot, 4u32]);
            if e.mem.u32(tag_slot) != BLOCK_TAG {
                let header = e.call(LOADING_FORM_HEADER, &args![save_load]).u32();
                if header != 0 {
                    let form_id = e.mem.u32(header);
                    let form = e.call(0x0048_39c0, &args![form_id]).u32();
                    let name = e.vcall(form, 0x130, &args![]).u32();
                    let version = u32::from(e.mem.u8(header + 9));
                    let header_flags = e.mem.u32(header + 5);
                    e.call(
                        MESSAGE,
                        &args![
                            BLOCK_HEADER_FORM_FORMAT,
                            SOURCE_FILE,
                            LOAD_GAME_HEADER_LINE,
                            form_id,
                            name,
                            version,
                            header_flags
                        ],
                    );
                } else {
                    let version = u32::from(e.call(SAVE_VERSION, &args![save_load]).u8());
                    e.call(
                        MESSAGE,
                        &args![
                            BLOCK_HEADER_FORMAT,
                            SOURCE_FILE,
                            LOAD_GAME_HEADER_LINE,
                            version
                        ],
                    );
                }
            }
            block_start = e.call(SAVE_POSITION, &args![save_load]).u32();
            e.call(SAVE_READ, &args![save_load, size_slot, 2u32]);
        }
        if e.call(SAVE_LOAD_UNAVAILABLE, &args![save_load]).bool() {
            let parent = e.call(ENABLE_PARENT, &args![this]).u32();
            if parent != 0 {
                follow_enable_parent(e, this, parent);
            }
        }
        if flags & 1 != 0
            && (e.call(FORM_IS_DISABLED, &args![this]).bool()
                || e.call(FORM_IS_DELETED, &args![this]).bool())
        {
            e.vcall(me, 0x1cc, &args![0u32, 1u32]);
        }
        if flags & 0x20 != 0 && tes_object_refr_has_container(e, this) != 0 {
            drop_drawn_weapon(e, this);
            let list = extra_list(e, me);
            e.call(0x0041_aeb0, &args![list]);
            let actor = if is_actor(e, me) { me } else { 0 };
            if actor != 0 {
                e.call(0x008a_dc50, &args![actor]);
            }
            let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
            e.call(INVENTORY_CHANGES_LOAD, &args![changes]);
        }
        let list = extra_list(e, me);
        fn_004269c0(e, Ptr::new(list), flags | flags2, this.cast());
        let mut mask = 0u32;
        if e.call(SAVE_VERSION, &args![save_load]).u8() < 0x43 {
            mask |= 0x10;
        }
        if flags & mask != 0 || is_actor(e, me) {
            let list = extra_list(e, me);
            e.call(EXTRA_LOAD_GAME, &args![list, flags, flags2, this]);
        }
        if flags & 0x1000_0000 != 0 && !is_actor(e, me) {
            e.call(FORM_LOAD_DATA, &args![this, word_slot, 2u32]);
            let size = e.mem.u16(word_slot);
            if size != 0 {
                e.call(LOAD_OLD_RECORD_A, &args![save_load, this, u32::from(size)]);
            }
        }
        if flags & 4 != 0 {
            e.call(FORM_LOAD_DATA, &args![this, word_slot, 2u32]);
            let size = e.mem.u16(word_slot);
            if size != 0 {
                // (the code also tests `flags & 0`, which is never set)
                e.call(LOAD_OLD_RECORD_B, &args![save_load, this, u32::from(size)]);
            }
        }
        if e.call(SAVE_VERSION, &args![save_load]).u8() >= 0x43 && flags & 0x10 != 0 {
            e.call(
                FORM_LOAD_DATA,
                &args![this, me + TESObjectREFR::fRefScale.off, 4u32],
            );
            let scale = e.get(this, TESObjectREFR::fRefScale);
            e.call(SET_SCALE, &args![this, scale]);
        }
        if flags & 0x40_0000 != 0 {
            copy_open_state_flag(e, me);
        }
        if flags & 0x80_0000 != 0 {
            let list = extra_list(e, me);
            e.call(EXTRA_REMOVE_LAST_SEQUENCE, &args![list]);
        }
        let loaded: Ptr<LOADED_REF_DATA> = e.get(this, TESObjectREFR::pLoadedData).cast();
        if !loaded.is_null() {
            let height: f32 = e.global(LOADED_DATA_DEFAULT_HEIGHT);
            e.set(loaded, LOADED_REF_DATA::fRelevantWaterHeight, height);
            e.set(loaded, LOADED_REF_DATA::iUnderwaterCount, 0);
            e.set(loaded, LOADED_REF_DATA::pCurrentWaterObject, Ptr::NULL);
        }
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
            let position = e.call(SAVE_POSITION, &args![save_load]).u32();
            let header = e.call(LOADING_FORM_HEADER, &args![save_load]).u32();
            let expected = u32::from(e.mem.u16(size_slot)).wrapping_add(block_start);
            if header != 0 {
                let form_id = e.mem.u32(header);
                let form = e.call(0x0048_39c0, &args![form_id]).u32();
                if position > expected {
                    let name = e.vcall(form, 0x130, &args![]).u32();
                    let version = u32::from(e.mem.u8(header + 9));
                    let header_flags = e.mem.u32(header + 5);
                    e.call(
                        MESSAGE,
                        &args![
                            LOAD_OVERRUN_FORM_FORMAT,
                            position.wrapping_sub(expected),
                            SOURCE_FILE,
                            LOAD_GAME_END_LINE,
                            form_id,
                            name,
                            version,
                            header_flags
                        ],
                    );
                } else if position < expected {
                    let name = e.vcall(form, 0x130, &args![]).u32();
                    let version = u32::from(e.mem.u8(header + 9));
                    let header_flags = e.mem.u32(header + 5);
                    e.call(
                        MESSAGE,
                        &args![
                            LOAD_UNDERRUN_FORM_FORMAT,
                            expected.wrapping_sub(position),
                            SOURCE_FILE,
                            LOAD_GAME_END_LINE,
                            form_id,
                            name,
                            version,
                            header_flags
                        ],
                    );
                }
            } else if position > expected {
                let version = u32::from(e.call(SAVE_VERSION, &args![save_load]).u8());
                e.call(
                    MESSAGE,
                    &args![
                        LOAD_OVERRUN_FORMAT,
                        position.wrapping_sub(expected),
                        SOURCE_FILE,
                        LOAD_GAME_END_LINE,
                        version
                    ],
                );
            } else if position < expected {
                let version = u32::from(e.call(SAVE_VERSION, &args![save_load]).u8());
                e.call(
                    MESSAGE,
                    &args![
                        LOAD_UNDERRUN_FORMAT,
                        expected.wrapping_sub(position),
                        SOURCE_FILE,
                        LOAD_GAME_END_LINE,
                        version
                    ],
                );
            }
        }
    });
}

// Translated from 0055f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second step of loading a reference (`flags` as in `LoadGame`),
/// virtual: the form's own step (`004534f0`), the controller data
/// (`fn_0055f970`, flag 0x10000000, non-actors), the actor extras
/// (`fn_004267d0`), the weapon and state reset after a container change
/// (flag 0x20), and, for open/close forms, the open state; then the
/// ragdoll data, and the disabled/deleted fix for three form types.
///
/// Branches that test `flags & 0` are compiled out and not translated;
/// the call to `0047c850`, whose result only feeds them, is kept.
pub fn fn_0055f240(e: &mut Engine, this: Ptr<TESObjectREFR>, flags: u32) {
    let me = this.addr();
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    e.call(FORM_PREPARE, &args![this, flags]);
    if !is_actor(e, me) && flags & 0x20_0000 != 0 {
        e.call(FORM_SET_EMPTY, &args![this, 0u32]);
    }
    e.call(SAVE_LOAD_UNAVAILABLE, &args![save_load]);
    if flags & 0x1000_0000 != 0 && !is_actor(e, me) {
        fn_0055f970(e, this);
    }
    if is_actor(e, me) {
        let list = extra_list(e, me);
        fn_004267d0(e, Ptr::new(list), flags, this.cast());
    }
    if flags & 0x20 != 0 {
        drop_drawn_weapon(e, this);
        if !e.call(SAVE_LOAD_UNAVAILABLE, &args![save_load]).bool() {
            e.call(RESET_STATE, &args![this, 0u32]);
        } else {
            e.vcall(me, 0x208, &args![0u32]);
        }
    }
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    if e.call(IS_OPEN_CLOSE_FORM, &args![base]).bool() {
        if flags & 0x40_0000 != 0 && !e.call(SAVE_LOAD_TEST_632CE0, &args![save_load]).bool() {
            copy_open_state_flag(e, me);
        }
        if fn_0055f5b0(e, Ptr::new(save_load)) {
            let state = e.call(GET_OPEN_STATE, &args![this, 8u32]).u8();
            e.call(SET_OPEN_STATE, &args![this, u32::from(state), 1u32]);
        }
    }
    if fn_0055f5b0(e, Ptr::new(save_load)) && e.vcall(me, 0x22c, &args![0u32]).bool() {
        e.call(RESTORE_RAGDOLL_DATA, &args![this, 0u32]);
    }
    if fn_0055f5b0(e, Ptr::new(save_load)) && e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        let kind = e.call(FORM_TYPE, &args![base]).i32();
        if (kind == 0x15 || kind == 0xd || kind == 0x1c)
            && (e.call(FORM_IS_DISABLED, &args![this]).bool()
                || e.call(FORM_IS_DELETED, &args![this]).bool()
                || (kind == 0x1c && e.call(FORM_IS_KIND_477BA0, &args![this]).bool()))
        {
            e.call(REFR_DISABLE_FIX, &args![this, 0u32]);
        }
    }
}

// Translated from 0055f5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this` is the `TESSaveLoadGame`: true when its revert state
/// (`m_iRevertState`, Xbox PDB, +0x48) is zero.
pub fn fn_0055f5b0(e: &mut Engine, this: Ptr) -> bool {
    // TESSaveLoadGame::m_iRevertState (Xbox PDB) +0x48
    e.mem.u32(this.addr() + 0x48) == 0
}

// Translated from 0055f5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The load step after the references exist (`flags`, `flags2` as in
/// `LoadGame`): the base step (`008d0600`), the disabled state from the
/// enable-state parent, the container changes' step (flag 0x20), the
/// virtual at +0xfc (whose result only feeds compiled-out code), and, for
/// an actor, the extra-data handler `00426020`.
pub fn fn_0055f5f0(e: &mut Engine, this: Ptr<TESObjectREFR>, flags: u32, flags2: u32) {
    let me = this.addr();
    e.call(LOAD_FIRST_STEP, &args![this, flags, flags2]);
    let parent = e.call(ENABLE_PARENT, &args![this]).u32();
    if parent != 0 {
        follow_enable_parent(e, this, parent);
    }
    if (flags & 1 != 0 || parent != 0)
        && (e.call(FORM_IS_DISABLED, &args![this]).bool()
            || e.call(FORM_IS_DELETED, &args![this]).bool())
    {
        e.vcall(me, 0x1cc, &args![0u32, 1u32]);
    }
    if flags & 0x20 != 0 && tes_object_refr_has_container(e, this) != 0 {
        let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
        e.call(INVENTORY_CHANGES_STEP_A, &args![changes]);
    }
    // The result of the virtual at +0xfc only decides branches that test
    // `flags & 0` (compiled out).
    e.vcall(me, 0xfc, &args![]);
    if is_actor(e, me) {
        let list = extra_list(e, me);
        let base = e.get(this.at(TESObjectREFR::data), OBJ_REFR::pObjectReference);
        e.call(EXTRA_AFTER_LOAD, &args![list, flags, flags2, this, base]);
    }
}

// Translated from 0055f780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::FinishInitLoadGame` (Xbox PDB): the base step
/// (`008d0600`), the container changes' step (flag 0x20), moves the
/// loaded 3D to the reference's location and orientation, updates its
/// Havok position and gives it a zero velocity, runs `SetUnderwater` for
/// actors, and ends with `0046a010(this, 0)`.
pub fn tes_object_refr_finish_init_load_game(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    flags: u32,
    flags2: u32,
) {
    let me = this.addr();
    e.call(LOAD_FIRST_STEP, &args![this, flags, flags2]);
    if flags & 0x20 != 0 && tes_object_refr_has_container(e, this) != 0 {
        let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
        e.call(INVENTORY_CHANGES_STEP_B, &args![changes]);
    }
    let obj3d = loaded_3d(e, me);
    if obj3d != 0 {
        let location = location_of(e, me);
        e.call(SET_3D_LOCATION, &args![obj3d, location]);
        e.with_stack(0x24, |e, matrix| {
            let orientation = e.call(GET_ORIENTATION, &args![this, matrix]).u32();
            e.call(SET_3D_ORIENTATION, &args![obj3d, orientation]);
        });
        e.call(UPDATE_POSITION, &args![obj3d, 1u32, 0u32]);
        set_velocity_record(e, obj3d, 0.0, 0, 0);
    }
    // (the code also tests `flags & 0`, which is never set)
    if is_actor(e, me) {
        let list = extra_list(e, me);
        fn_00426720(e, Ptr::new(list), flags, flags2, this.cast());
    }
    e.call(FINISH_INIT_LAST, &args![this, 0u32]);
}

// Translated from 0055f880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size of the controller data `fn_0055f900` saves for the reference
/// (`SaveGame`, flag 0x10000000): that of the controller manager of its
/// loaded 3D (`fn_0055fdb0`), 2 when there is none.
pub fn fn_0055f880(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u16 {
    let manager = manager_of_reference(e, this.addr());
    0u16.wrapping_add(fn_0055fdb0(e, Ptr::new(manager)))
}

// Translated from 0055f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the controller data of the reference's loaded 3D (`fn_0055fe70`),
/// with the default blend time marker `-1.0`.
pub fn fn_0055f900(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let manager = manager_of_reference(e, this.addr());
    let marker: f32 = e.global(MINUS_ONE_FLOAT);
    fn_0055fe70(e, Ptr::new(manager), marker);
}

// Translated from 0055f970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts the animation of a freshly loaded reference back in its rest
/// state. If the last finished sequence of the reference is not named
/// "Unequip", or its controller manager has a sequence named "Unequip"
/// that is a generic-location one, or its 3D has a morpher controller, the
/// reference's location is set from its base object (virtual +0x178) and
/// the virtuals +0x1cc and +0x1c4 run. Then the two table entries 0 and 2
/// are looked up in the manager: the manager is deactivated and the found
/// sequences are restarted with the offset `-FLT_MAX`, their offset time
/// as the 3D's velocity record; with neither found the manager's
/// sequence 0 is restarted instead. It ends by deactivating the manager
/// again.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_0055f970(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let obj3d = loaded_3d(e, me);
    if obj3d != 0 {
        let mut reset = false;
        let list = extra_list(e, me);
        let last = e.call(EXTRA_GET_LAST_SEQUENCE, &args![list]).u32();
        if last != 0 {
            let unequip = e.global::<u32>(NAME_UNEQUIP);
            if e.call(SEQUENCE_NAME_COMPARE, &args![last, unequip]).u32() == 0 {
                reset = true;
            }
        }
        if !reset && controller_of(e, obj3d) != 0 {
            let controller = controller_of(e, obj3d);
            let manager = manager_of(e, controller);
            if manager != 0 {
                let sequence = find_sequence(e, manager, NAME_UNEQUIP);
                if sequence != 0 && e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() != 0 {
                    reset = true;
                }
            }
        }
        if !reset && e.call(HAS_MORPHER_CONTROLLER, &args![obj3d]).bool() {
            reset = true;
        }
        if reset {
            let base = e.get(this.at(TESObjectREFR::data), OBJ_REFR::pObjectReference);
            let location = e.vcall(base.addr(), 0x178, &args![this]).u32();
            e.vcall(me, 0x1cc, &args![location, 1u32]);
            e.vcall(me, 0x1c4, &args![]);
        }
    }
    let obj3d = loaded_3d(e, me);
    if obj3d != 0 && controller_of(e, obj3d) != 0 {
        let controller = controller_of(e, obj3d);
        let manager = manager_of(e, controller);
        if manager != 0 {
            let first = find_sequence(e, manager, NAME_SEQUENCE_A);
            let second = find_sequence(e, manager, NAME_SEQUENCE_B);
            e.call(MANAGER_DEACTIVATE_ALL, &args![manager, 0.0f32]);
            if first != 0 || second != 0 {
                e.call(MANAGER_SET_FLAG, &args![manager, 1u32]);
                for sequence in [first, second] {
                    if sequence != 0 {
                        if e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() == 0 {
                            e.call(
                                MANAGER_ACTIVATE,
                                &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
                            );
                        }
                        restart_offset(e, obj3d, sequence);
                    }
                }
            } else {
                e.call(MANAGER_SET_FLAG, &args![manager, 1u32]);
                let sequence = e.call(MANAGER_SEQUENCE_AT, &args![manager, 0u32]).u32();
                if sequence != 0 {
                    e.call(
                        MANAGER_ACTIVATE,
                        &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
                    );
                    restart_offset(e, obj3d, sequence);
                }
                e.call(MANAGER_DEACTIVATE_ALL, &args![manager, 0.0f32]);
                e.call(MANAGER_SET_FLAG, &args![manager, 0u32]);
            }
        }
    }
    let controller = if obj3d != 0 {
        controller_of(e, obj3d)
    } else {
        0
    };
    let manager = manager_of(e, controller);
    if manager != 0 {
        e.call(MANAGER_DEACTIVATE_ALL, &args![manager, 0.0f32]);
    }
}

/// The tail `fn_0055f970` runs for a restarted sequence: sets its offset
/// to `-FLT_MAX` and gives the 3D a velocity record made from the
/// sequence's offset time (`00759450`).
pub(crate) fn restart_offset(e: &mut Engine, obj3d: u32, sequence: u32) {
    let minimum: f32 = e.global(FLOAT_MAX);
    e.call(SEQUENCE_SET_OFFSET, &args![sequence, -minimum]);
    let time = float_of(e, SEQUENCE_OFFSET_TIME, sequence);
    set_velocity_record(e, obj3d, time, 1, 0);
}

// Translated from 0055fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size `fn_0055fe70` writes for the sequences of `manager`: 2 for the
/// count, and for each sequence with a non-zero `008041a0`, 1 + the length
/// of its name + what `004efb10` reports (16-bit sum).
pub fn fn_0055fdb0(e: &mut Engine, manager: Ptr) -> u16 {
    let manager = manager.addr();
    let mut size: u16 = 2;
    if manager != 0 && e.call(MANAGER_HAS_SEQUENCES, &args![manager]).bool() {
        let mut index = 0u32;
        while index < e.call(MANAGER_SEQUENCE_COUNT, &args![manager]).u32() {
            let sequence = e.call(MANAGER_SEQUENCE_AT, &args![manager, index]).u32();
            if sequence != 0 && e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() != 0 {
                size = size.wrapping_add(1);
                let name = sequence_name(e, sequence);
                let length = e.call(STRING_LENGTH, &args![name]).u32();
                size = size.wrapping_add(length as u16);
                size = size.wrapping_add(e.call(SEQUENCE_SAVE_SIZE, &args![sequence]).u16());
            }
            index += 1;
        }
    }
    size
}

// Translated from 0055fe70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the sequences of `manager` into the save game: a 16-bit count
/// (patched at the end), and for each sequence with a non-zero
/// `008041a0` the length byte, the name and the sequence's own data
/// (`004efb20(blend)`). A `blend` of `-1.0` stands for the default at
/// `011c3c08`.
pub fn fn_0055fe70(e: &mut Engine, manager: Ptr, blend: f32) {
    let manager = manager.addr();
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    let mut blend = blend;
    let marker: f64 = e.global(MINUS_ONE_DOUBLE);
    if blend as f64 == marker {
        blend = e.global(DEFAULT_BLEND_TIME);
    }
    e.with_stack(4, |e, slots| {
        let count_slot = slots.addr();
        let length_slot = slots.addr() + 2;
        e.mem.set_u16(count_slot, 0);
        let count_field = e.call(SAVE_POSITION, &args![save_load]).u32();
        e.call(SAVE_WRITE, &args![save_load, count_slot, 2u32]);
        if manager != 0 && e.call(MANAGER_HAS_SEQUENCES, &args![manager]).bool() {
            let mut index = 0u32;
            while index < e.call(MANAGER_SEQUENCE_COUNT, &args![manager]).u32() {
                let sequence = e.call(MANAGER_SEQUENCE_AT, &args![manager, index]).u32();
                if sequence != 0 && e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() != 0 {
                    let name = sequence_name(e, sequence);
                    let length = e.call(STRING_LENGTH, &args![name]).u8();
                    e.mem.set_u8(length_slot, length);
                    e.call(SAVE_WRITE, &args![save_load, length_slot, 1u32]);
                    let name = sequence_name(e, sequence);
                    e.call(SAVE_WRITE, &args![save_load, name, u32::from(length)]);
                    e.call(SEQUENCE_SAVE, &args![sequence, blend]);
                    let count = e.mem.u16(count_slot);
                    e.mem.set_u16(count_slot, count.wrapping_add(1));
                }
                index += 1;
            }
        }
        let count = e.mem.u16(count_slot);
        e.mem.set_u16(count_field, count);
    });
}

// Translated from 0055ffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads what `fn_0055fe70` saved: a 16-bit count (0 when above 65000)
/// and, per entry, the name (an index into the table at `011977d8` in
/// saves of versions 0x15 and 0x16, a length-prefixed text otherwise) and
/// the sequence's own data. The entry's sequence is found in `manager` by
/// name (`00408b20`), restarted if it is a generic-location one, loaded
/// (`004efbc0(blend)`), and, when `object` passes `004b5c80`, the object is
/// given a run of velocity records stepping through the sequence's
/// duration; an entry without a sequence is skipped in the stream
/// (`004efaa0` bytes). `blend` of `-1.0` stands for `011c3c08`. After the
/// entries the object gets one more record with the blend time.
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn fn_0055ffa0(e: &mut Engine, manager: Ptr, object: Ptr, blend: f32) {
    let manager = manager.addr();
    let object = object.addr();
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    let mut blend = blend;
    let marker: f64 = e.global(MINUS_ONE_DOUBLE);
    if blend as f64 == marker {
        blend = e.global(DEFAULT_BLEND_TIME);
    }
    e.with_stack(0x140, |e, frame| {
        let count_slot = frame.addr();
        let index_slot = frame.addr() + 4;
        let length_slot = frame.addr() + 8;
        let record = frame.addr() + 0x10;
        let buffer = frame.addr() + 0x20;
        e.call(SAVE_READ, &args![save_load, count_slot, 2u32]);
        if u32::from(e.mem.u16(count_slot)) > 65000 {
            e.mem.set_u16(count_slot, 0);
        }
        let mut sequence_count = 0u32;
        if manager != 0 {
            sequence_count = e.call(MANAGER_SEQUENCE_COUNT, &args![manager]).u32();
            if e.mem.u16(count_slot) != 0 {
                e.call(MANAGER_SET_FLAG, &args![manager, 1u32]);
            }
        }
        let mut any = false;
        let mut entry = 0i32;
        while entry < i32::from(e.mem.u16(count_slot)) {
            if e.call(SAVE_VERSION, &args![save_load]).u8() >= 0x15
                && e.call(SAVE_VERSION, &args![save_load]).u8() < 0x17
            {
                e.call(SAVE_READ, &args![save_load, index_slot, 4u32]);
                let index = e.mem.i32(index_slot);
                if index < 0xf5 {
                    let text = e
                        .mem
                        .u32(NAME_TABLE.wrapping_add((index as u32).wrapping_mul(0x24)));
                    e.call(0x0040_6d30, &args![buffer, 0x104u32, text]);
                } else {
                    e.call(MEMSET, &args![buffer, 0u32, 0x104u32]);
                }
            }
            if e.call(SAVE_VERSION, &args![save_load]).u8() < 0x15
                || e.call(SAVE_VERSION, &args![save_load]).u8() >= 0x17
            {
                e.call(SAVE_READ, &args![save_load, length_slot, 1u32]);
                e.call(MEMSET, &args![buffer, 0u32, 0x104u32]);
                let length = u32::from(e.mem.u8(length_slot));
                e.call(SAVE_READ, &args![save_load, buffer, length]);
            }
            let mut found = false;
            if manager != 0 {
                let mut position = 0u32;
                while position < sequence_count {
                    let sequence = e.call(MANAGER_SEQUENCE_AT, &args![manager, position]).u32();
                    if sequence != 0 {
                        let name = sequence_name(e, sequence);
                        if e.call(STRING_COMPARE, &args![name, buffer]).u32() == 0 {
                            if e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() == 0 {
                                e.call(
                                    MANAGER_ACTIVATE,
                                    &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
                                );
                            }
                            e.call(SEQUENCE_LOAD, &args![sequence, blend]);
                            if object != 0 && e.call(IS_ACTOR_3D, &args![object]).bool() {
                                let duration = float_of(e, SEQUENCE_DURATION, sequence);
                                let total = (duration as f64 + blend as f64) as f32;
                                let mut start = (blend as f64 - total as f64) as f32;
                                let zero: f64 = e.global(ZERO_DOUBLE);
                                if (start as f64) < zero {
                                    start = 0.0;
                                }
                                let divisor: f64 = e.global(STEP_DIVISOR);
                                let mut step = (total as f64 / divisor) as f32;
                                let smallest: f64 = e.global(MIN_STEP_DOUBLE);
                                if (step as f64) < smallest {
                                    step = e.global(MIN_STEP_FLOAT);
                                }
                                let mut time = start;
                                while (time as f64) < blend as f64 {
                                    e.call(MAKE_VELOCITY, &args![record, time, 0u32, 0u32]);
                                    e.call(ADD_3D_VELOCITY, &args![object, record]);
                                    time = (time as f64 + step as f64) as f32;
                                }
                            }
                            found = true;
                            any = true;
                            break;
                        }
                    }
                    position += 1;
                }
            }
            if !found {
                let size = e.call(EMPTY_SEQUENCE_SIZE, &args![]).u16();
                e.call(SAVE_SKIP, &args![save_load, u32::from(size)]);
            }
            entry += 1;
        }
        if any && object != 0 {
            e.call(MAKE_VELOCITY, &args![record, blend, 1u32, 0u32]);
            e.call(SET_3D_VELOCITY, &args![object, record]);
        }
    });
}

// Translated from 00560350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size of the Havok record `fn_005604b0` saves for the reference
/// (16-bit sum), and a fill of `out` (a [`HavokSaveData`], or a local one
/// when `out` is null): 0 when the reference has no loaded 3D. Otherwise
/// 3, plus per active body `0x18` (when any) and `0x1c` per body, with the
/// counts and flags `fn_00560870` collects while it walks the 3D's
/// collision nodes; the root's own body is not counted twice (flag 2).
/// `out`'s flags get 4 when there are active and inactive bodies, or 1 when
/// only active ones.
pub fn fn_00560350(e: &mut Engine, this: Ptr<TESObjectREFR>, out: Ptr<HavokSaveData>) -> u16 {
    let mut total: u16 = 0;
    let obj3d = loaded_3d(e, this.addr());
    if obj3d != 0 {
        total = total.wrapping_add(1);
        total = total.wrapping_add(2);
        e.with_stack(0x14 + 0x10, |e, scratch| {
            let local: Ptr<HavokSaveData> = scratch.cast();
            let walk: Ptr<CollisionWalkContext> = Ptr::new(scratch.addr() + 0x14);
            fn_0055ebf0(e, local);
            let record = if out.is_null() { local } else { out };
            e.set(record, HavokSaveData::pRef, this.cast());
            e.set(record, HavokSaveData::pObj3D, Ptr::new(obj3d));
            let root = e.call(COLLISION_ROOT, &args![obj3d]).u32();
            e.set(record, HavokSaveData::pCollisionRoot, Ptr::new(root));
            e.set(walk, CollisionWalkContext::iMode, 0x12);
            e.set(walk, CollisionWalkContext::bRecurse, 1);
            e.set(walk, CollisionWalkContext::pUserData, record.cast());
            e.call(WALK_COLLISION, &args![obj3d, walk, 0x0056_0870u32]);
            let active = e.get(record, HavokSaveData::sActiveBoneCount);
            let inactive = e.get(record, HavokSaveData::sInactiveBoneCount);
            let bones = active.wrapping_add(inactive);
            if active != 0 && inactive != 0 {
                let flags = e.get(record, HavokSaveData::cFlags);
                e.set(record, HavokSaveData::cFlags, flags | 4);
                total = total.wrapping_add(bones);
            } else if active != 0 {
                let flags = e.get(record, HavokSaveData::cFlags);
                e.set(record, HavokSaveData::cFlags, flags | 1);
            }
            if active != 0 {
                total = total.wrapping_add((u32::from(active) * 0x18) as u16);
            }
            let mut counted = bones;
            if e.get(record, HavokSaveData::cFlags) & 2 != 0 {
                counted = counted.wrapping_sub(1);
            }
            total = total.wrapping_add((u32::from(counted) * 0x1c) as u16);
        });
    }
    total
}

// Translated from 005604b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the Havok record of the reference: the flag byte of `record`, the
/// 16-bit body count (active + inactive), then the bodies themselves by
/// walking the loaded 3D's collision nodes with `fn_00560a10`. Nothing
/// without a loaded 3D.
pub fn fn_005604b0(e: &mut Engine, this: Ptr<TESObjectREFR>, record: Ptr<HavokSaveData>) {
    let obj3d = loaded_3d(e, this.addr());
    if obj3d != 0 {
        e.call(FORM_SAVE_DATA, &args![this, record, 1u32]);
        e.with_stack(0x14, |e, frame| {
            let count_slot = frame.addr();
            let walk: Ptr<CollisionWalkContext> = Ptr::new(frame.addr() + 4);
            let active = e.get(record, HavokSaveData::sActiveBoneCount);
            let inactive = e.get(record, HavokSaveData::sInactiveBoneCount);
            e.mem.set_u16(count_slot, active.wrapping_add(inactive));
            e.call(FORM_SAVE_DATA, &args![this, count_slot, 2u32]);
            e.set(walk, CollisionWalkContext::iMode, 0x12);
            e.set(walk, CollisionWalkContext::bRecurse, 1);
            e.set(walk, CollisionWalkContext::pUserData, record.cast());
            e.call(WALK_COLLISION, &args![obj3d, walk, 0x0056_0a10u32]);
        });
    }
}

// Translated from 00560530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the Havok record of the reference (`size` is the record's size in
/// the save game). Saves older than version 0x51 go to `00561860`. Without
/// a loaded 3D the record is skipped (with a message). Otherwise it reads
/// the flag byte and the saved body count, tells an actor whether it had
/// its weapon bone (flag 8), and compares the saved count with the
/// current one (`fn_00560350`): a different count skips the rest of the
/// record (`size - 3` bytes), logs why, and knocks an actor down
/// (disabling its ragdoll animation first); the same count walks the
/// collision nodes with `fn_00560e70`, which reads each body, freezes an
/// actor's rigid bodies when it is fixed, and gives the 3D a zero velocity
/// record.
pub fn fn_00560530(e: &mut Engine, this: Ptr<TESObjectREFR>, size: u16) {
    let me = this.addr();
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    if e.call(SAVE_VERSION, &args![save_load]).u8() < 0x51 {
        e.call(LOAD_OLD_HAVOK, &args![this, u32::from(size)]);
        return;
    }
    let obj3d = loaded_3d(e, me);
    if obj3d == 0 {
        let form_id = e.call(FORM_ID, &args![this]).u32();
        let name = e.vcall(me, 0x130, &args![]).u32();
        e.call(MESSAGE, &args![HAVOK_NO_3D_FORMAT, name, form_id]);
        e.call(SAVE_SKIP, &args![save_load, u32::from(size)]);
        return;
    }
    e.with_stack(0x80, |e, frame| {
        let saved: Ptr<HavokSaveData> = frame.cast();
        let current: Ptr<HavokSaveData> = Ptr::new(frame.addr() + 0x20);
        let count_slot = frame.addr() + 0x40;
        let vector = frame.addr() + 0x50;
        let walk: Ptr<CollisionWalkContext> = Ptr::new(frame.addr() + 0x60);
        fn_0055ebf0(e, saved);
        e.call(FORM_LOAD_DATA, &args![this, saved, 1u32]);
        e.call(FORM_LOAD_DATA, &args![this, count_slot, 2u32]);
        let actor = if is_actor(e, me) { me } else { 0 };
        if actor != 0 {
            let had_weapon = e.get(saved, HavokSaveData::cFlags) & 8 != 0;
            e.call(SET_HAVOK_WEAPON, &args![actor, had_weapon]);
        }
        fn_0055ebf0(e, current);
        fn_00560350(e, this, current);
        let current_count = e
            .get(current, HavokSaveData::sActiveBoneCount)
            .wrapping_add(e.get(current, HavokSaveData::sInactiveBoneCount));
        let saved_count = e.mem.u16(count_slot);
        if saved_count != current_count {
            let form_id = e.call(FORM_ID, &args![this]).u32();
            let name = e.vcall(me, 0x130, &args![]).u32();
            e.call(
                MESSAGE,
                &args![
                    HAVOK_BONE_COUNT_FORMAT,
                    name,
                    form_id,
                    u32::from(saved_count),
                    u32::from(current_count)
                ],
            );
            let saved_weapon = e.get(saved, HavokSaveData::cFlags) & 8;
            let current_weapon = e.get(current, HavokSaveData::cFlags) & 8;
            if saved_weapon != current_weapon {
                let current_text = if current_weapon != 0 {
                    TRUE_TEXT
                } else {
                    FALSE_TEXT
                };
                let saved_text = if saved_weapon != 0 {
                    TRUE_TEXT
                } else {
                    FALSE_TEXT
                };
                e.call(
                    MESSAGE,
                    &args![HAVOK_WEAPON_BONE_FORMAT, saved_text, current_text],
                );
            }
            e.call(
                SAVE_SKIP,
                &args![save_load, u32::from(size).wrapping_sub(3)],
            );
            if actor != 0 {
                let ragdoll = e.mem.u32(actor + 0xac);
                if ragdoll != 0 {
                    e.call(DISABLE_RAGDOLL_ANIM, &args![ragdoll, 1u32]);
                }
                e.call(KNOCK_DOWN, &args![obj3d, ZERO_VECTOR, 1u32, 0.0f32, 0u32]);
            }
        } else {
            e.set(saved, HavokSaveData::pRef, this.cast());
            e.set(saved, HavokSaveData::pObj3D, Ptr::new(obj3d));
            let root = e.call(COLLISION_ROOT, &args![obj3d]).u32();
            e.set(saved, HavokSaveData::pCollisionRoot, Ptr::new(root));
            e.set(walk, CollisionWalkContext::iMode, 0x12);
            e.set(walk, CollisionWalkContext::bRecurse, 1);
            e.set(walk, CollisionWalkContext::pUserData, saved.cast());
            if is_actor(e, me) {
                e.call(SET_MOTION, &args![obj3d, 1u32, 1u32, 0u32, 1u32]);
                e.call(MAKE_VELOCITY, &args![vector, 0.0f32, 0u32, 0u32]);
                e.call(SET_3D_VELOCITY, &args![obj3d, vector]);
            }
            e.call(WALK_COLLISION, &args![obj3d, walk, 0x0056_0e70u32]);
            if actor != 0 && e.vcall(actor, 0x234, &args![]).bool() {
                e.call(SET_FIXED, &args![obj3d, 1u32, 1u32]);
                e.call(SET_MOTION, &args![obj3d, 1u32, 1u32, 0u32, 1u32]);
                let group = e.vcall(actor, 0x1e4, &args![]).u32();
                if group != 0 {
                    e.call(CLEAR_ANIM_GROUP, &args![group, 0x14u32, 0.0f32]);
                    e.call(ANIM_FINISH, &args![group]);
                }
            }
            if !is_actor(e, me) {
                e.call(MAKE_VELOCITY, &args![vector, 0.0f32, 0u32, 0u32]);
                e.call(SET_3D_VELOCITY, &args![obj3d, vector]);
            }
        }
    });
}

/// The test the three Havok callbacks start with, after finding the
/// node's object: `true` means ignore this node. A node that is not the
/// collision root and whose object is named "Arrow" is ignored; so is one
/// whose object is of the type at `011f9140` while the reference is an
/// actor.
pub(crate) fn skip_collision_node(
    e: &mut Engine,
    node: u32,
    object: u32,
    state: Ptr<HavokSaveData>,
) -> bool {
    let root = e.get(state, HavokSaveData::pCollisionRoot).addr();
    if node != root && object != 0 && sequence_name(e, object) != 0 {
        let name = sequence_name(e, object);
        if e.call(STRING_COMPARE, &args![name, ARROW]).u32() == 0 {
            return true;
        }
    }
    let reference = e.get(state, HavokSaveData::pRef).addr();
    e.vcall(reference, 0x100, &args![]).bool()
        && e.call(IS_OF_TYPE, &args![SKIPPED_NODE_TYPE, object]).bool()
}

/// The rigid body of a collision node: its body reference (`006fa820`)
/// cast to a rigid body, 0 when either is missing.
pub(crate) fn node_rigid_body(e: &mut Engine, body_ref: u32) -> u32 {
    if body_ref == 0 {
        return 0;
    }
    e.call(CHECKED_CAST, &args![RIGID_BODY_TYPE, body_ref])
        .u32()
}

// Translated from 00560870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback `fn_00560350` hands to the collision walker (`00c68900`)
/// to count the bodies: for each collision node `node` (with the walk's
/// context, whose user data is the [`HavokSaveData`]) that is not skipped,
/// finds its rigid body and counts it as active or inactive. Sets flag 8
/// when the node is of the type at `0126817c`, and flag 2 when it is the
/// collision root.
pub fn fn_00560870(e: &mut Engine, node: Ptr, walk: Ptr<CollisionWalkContext>) {
    let state: Ptr<HavokSaveData> = e.get(walk, CollisionWalkContext::pUserData).cast();
    if e.call(IS_OF_TYPE, &args![WEAPON_NODE_TYPE, node]).bool() {
        let flags = e.get(state, HavokSaveData::cFlags);
        e.set(state, HavokSaveData::cFlags, flags | 8);
    }
    let object = e.call(NODE_OBJECT, &args![node]).u32();
    if skip_collision_node(e, node.addr(), object, state) {
        return;
    }
    let body_ref = e.call(NODE_BODY_REF, &args![node]).u32();
    if body_ref == 0 {
        return;
    }
    let body = node_rigid_body(e, body_ref);
    if body == 0 {
        return;
    }
    if node == e.get(state, HavokSaveData::pCollisionRoot) {
        let flags = e.get(state, HavokSaveData::cFlags);
        e.set(state, HavokSaveData::cFlags, flags | 2);
    }
    if bhk_rigid_body_is_active(e, Ptr::new(body)) {
        let active = e.get(state, HavokSaveData::sActiveBoneCount);
        e.set(
            state,
            HavokSaveData::sActiveBoneCount,
            active.wrapping_add(1),
        );
    } else {
        let inactive = e.get(state, HavokSaveData::sInactiveBoneCount);
        e.set(
            state,
            HavokSaveData::sInactiveBoneCount,
            inactive.wrapping_add(1),
        );
    }
}

// Translated from 005609b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::IsActive` (Xbox PDB): whether the body's Havok entity
/// (`004ae750`) reports itself active; false for a body without an entity.
pub fn bhk_rigid_body_is_active(e: &mut Engine, this: Ptr) -> bool {
    let entity = e.call(BODY_ENTITY, &args![this]).u32();
    if entity == 0 {
        return false;
    }
    e.with_stack(4, |e, flag| {
        let reported = e.call(ENTITY_ACTIVE_FLAG, &args![entity, flag]).u32();
        fn_005609f0(e, Ptr::new(reported))
    })
}

// Translated from 005609f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the byte at `this` is non-zero.
pub fn fn_005609f0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.i8(this.addr()) != 0
}

// Translated from 00560a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback `fn_005604b0` hands to the collision walker to save the
/// bodies: for each collision node that is not skipped and has a rigid
/// body it writes (except for the collision root) the body's position (12
/// bytes) and rotation (16), then, if flag 4 of the record is set, a byte
/// telling whether the body is active, and for an active body its two
/// velocity vectors (12 bytes each).
pub fn fn_00560a10(e: &mut Engine, node: Ptr, walk: Ptr<CollisionWalkContext>) {
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    let state: Ptr<HavokSaveData> = e.get(walk, CollisionWalkContext::pUserData).cast();
    let object = e.call(NODE_OBJECT, &args![node]).u32();
    if skip_collision_node(e, node.addr(), object, state) {
        return;
    }
    let body_ref = e.call(NODE_BODY_REF, &args![node]).u32();
    if body_ref == 0 {
        return;
    }
    let body = node_rigid_body(e, body_ref);
    if body == 0 {
        return;
    }
    if node != e.get(state, HavokSaveData::pCollisionRoot) {
        e.with_stack(0x20, |e, scratch| {
            let position = scratch.addr();
            let rotation = scratch.addr() + 0x10;
            e.call(VECTOR_CONSTRUCT, &args![position]);
            e.call(VECTOR_CONSTRUCT, &args![rotation]);
            bhk_rigid_body_get_position(e, Ptr::new(body), Ptr::new(position));
            bhk_rigid_body_get_rotation(e, Ptr::new(body), Ptr::new(rotation));
            e.call(SAVE_WRITE, &args![save_load, position, 0xcu32]);
            e.call(SAVE_WRITE, &args![save_load, rotation, 0x10u32]);
        });
    }
    let active = bhk_rigid_body_is_active(e, Ptr::new(body));
    if e.get(state, HavokSaveData::cFlags) & 4 != 0 {
        e.with_stack(4, |e, byte| {
            e.mem.set_u8(byte.addr(), active as u8);
            e.call(SAVE_WRITE, &args![save_load, byte, 1u32]);
        });
    }
    if active {
        e.with_stack(0x20, |e, scratch| {
            let first = scratch.addr();
            let second = scratch.addr() + 0x10;
            e.call(VECTOR_CONSTRUCT, &args![first]);
            e.call(VECTOR_CONSTRUCT, &args![second]);
            fn_00560d50(e, Ptr::new(body), Ptr::new(first));
            fn_00560de0(e, Ptr::new(body), Ptr::new(second));
            e.call(SAVE_WRITE, &args![save_load, first, 0xcu32]);
            e.call(SAVE_WRITE, &args![save_load, second, 0xcu32]);
        });
    }
}

// Translated from 00560bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::GetPosition` (Xbox PDB): the body's position (the
/// virtual at +0xd4 fills an `hkVector4`) converted to the `NiPoint3` at
/// `result`.
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn bhk_rigid_body_get_position(e: &mut Engine, this: Ptr, result: Ptr) {
    e.with_stack(0x18, |e, vector| {
        e.call(VECTOR_CONSTRUCT, &args![vector]);
        let position = e.vcall(this.addr(), 0xd4, &args![vector]).u32();
        e.call(VECTOR_TO_NI, &args![result, position]);
    });
}

// Translated from 00560c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::GetRotation` (Xbox PDB): the body's rotation (the
/// virtual at +0xd8 fills an `hkQuaternion`) converted to the
/// `NiQuaternion` at `result` by `fn_00560c80`.
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn bhk_rigid_body_get_rotation(e: &mut Engine, this: Ptr, result: Ptr) {
    e.with_stack(0x18, |e, quaternion| {
        e.call(QUATERNION_CONSTRUCT, &args![quaternion]);
        let rotation = e.vcall(this.addr(), 0xd8, &args![quaternion]).u32();
        fn_00560c80(e, result, Ptr::new(rotation));
    });
}

// Translated from 00560c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts the `hkQuaternion` at `source` (four `float`s) into the
/// quaternion at `dest` by calling its four component setters with
/// `source[0]` to `source[3]`; returns `dest`.
pub fn fn_00560c80(e: &mut Engine, dest: Ptr, source: Ptr) -> Ptr {
    let at = fn_00560d10(e, source, 0);
    let value = e.mem.f32(at.addr());
    e.call(QUATERNION_SET_0, &args![dest, value]);
    let at = fn_00560d10(e, source, 1);
    let value = e.mem.f32(at.addr());
    fn_00560cf0(e, dest, value);
    let at = fn_00560d10(e, source, 2);
    let value = e.mem.f32(at.addr());
    e.call(QUATERNION_SET_2, &args![dest, value]);
    let at = fn_00560d10(e, source, 3);
    let value = e.mem.f32(at.addr());
    e.call(QUATERNION_SET_3, &args![dest, value]);
    dest
}

// Translated from 00560cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the `float` at `this + 8`.
pub fn fn_00560cf0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 8, x87_quiet(value));
}

// Translated from 00560d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of element `index` of the `float` array at `this`
/// (`fn_00560d30`).
pub fn fn_00560d10(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    fn_00560d30(e, this, index)
}

// Translated from 00560d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + index * 4`.
pub fn fn_00560d30(_e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    Ptr::new(this.addr().wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00560d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts the body's first velocity vector (`fn_00560d80`) to the
/// `NiPoint3` at `dest` (`00458620`).
pub fn fn_00560d50(e: &mut Engine, this: Ptr, dest: Ptr) {
    let vector = fn_00560d80(e, this);
    e.call(VECTOR_TO_NI, &args![dest, vector]);
}

// Translated from 00560d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The body's first velocity vector: that of its entity (`fn_00560dc0`),
/// or the zero vector (`00458b20`) when it has none.
pub fn fn_00560d80(e: &mut Engine, this: Ptr) -> Ptr {
    let entity = e.call(BODY_ENTITY, &args![this]).u32();
    if entity == 0 {
        e.call(ZERO_VECTOR_GETTER, &args![]).ptr()
    } else {
        fn_00560dc0(e, Ptr::new(entity))
    }
}

// Translated from 00560dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks from the entity to its first velocity vector: `009d9f40(this)`,
/// then `0045c650` on that.
pub fn fn_00560dc0(e: &mut Engine, this: Ptr) -> Ptr {
    let part = e.call(BODY_VECTOR_PART, &args![this]).u32();
    e.call(BODY_VECTOR_FINISH, &args![part]).ptr()
}

// Translated from 00560de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts the body's second velocity vector (`fn_00560e10`) to the
/// `NiPoint3` at `dest` (`004a3970`).
pub fn fn_00560de0(e: &mut Engine, this: Ptr, dest: Ptr) {
    let vector = fn_00560e10(e, this);
    e.call(VECTOR_TO_NI_VELOCITY, &args![dest, vector]);
}

// Translated from 00560e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The body's second velocity vector: that of its entity (`fn_00560e50`),
/// or the zero vector (`00458b20`) when it has none.
pub fn fn_00560e10(e: &mut Engine, this: Ptr) -> Ptr {
    let entity = e.call(BODY_ENTITY, &args![this]).u32();
    if entity == 0 {
        e.call(ZERO_VECTOR_GETTER, &args![]).ptr()
    } else {
        fn_00560e50(e, Ptr::new(entity))
    }
}

// Translated from 00560e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks from the entity to its second velocity vector: `009d9f40` applied
/// twice.
pub fn fn_00560e50(e: &mut Engine, this: Ptr) -> Ptr {
    let part = e.call(BODY_VECTOR_PART, &args![this]).u32();
    e.call(BODY_VECTOR_PART, &args![part]).ptr()
}

// Translated from 00560e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback `fn_00560530` hands to the collision walker to load the
/// bodies, the reverse of `fn_00560a10`: for each collision node that is
/// not skipped and has a rigid body, sets the body's position and rotation
/// from the stream (for the collision root, from the reference's own
/// location and orientation, clearing flag 2), then reads the active byte
/// (flag 4) and, for an active body, the two velocity vectors; an
/// inactive one gets zero velocities.
pub fn fn_00560e70(e: &mut Engine, node: Ptr, walk: Ptr<CollisionWalkContext>) {
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    let state: Ptr<HavokSaveData> = e.get(walk, CollisionWalkContext::pUserData).cast();
    let body_ref = e.call(NODE_BODY_REF, &args![node]).u32();
    e.call(0x004d_9fa0, &args![node, 0u32]);
    let object = e.call(NODE_OBJECT, &args![node]).u32();
    if skip_collision_node(e, node.addr(), object, state) || body_ref == 0 {
        return;
    }
    let body = node_rigid_body(e, body_ref);
    if body == 0 {
        return;
    }
    let was_set = e.vcall(body, 0x94, &args![]).u32() != 0;
    if was_set {
        e.call(BODY_FLAG_SETTER, &args![body, 0u32]);
    }
    e.with_stack(0xa0, |e, frame| {
        if e.get(state, HavokSaveData::cFlags) & 2 != 0 {
            let flags = e.get(state, HavokSaveData::cFlags);
            e.set(state, HavokSaveData::cFlags, flags & 0xfd);
            let reference = e.get(state, HavokSaveData::pRef).addr();
            let location = e.vcall(reference, 0x1f4, &args![]).u32();
            fn_005610f0(e, Ptr::new(body), Ptr::new(location));
            let quaternion = frame.addr();
            let matrix = frame.addr() + 0x20;
            e.call(VECTOR_CONSTRUCT, &args![quaternion]);
            let orientation = e.call(GET_ORIENTATION, &args![reference, matrix]).u32();
            e.call(QUATERNION_FROM_ROTATION, &args![quaternion, orientation]);
            fn_00561150(e, Ptr::new(body), Ptr::new(quaternion));
        } else {
            let position = frame.addr() + 0x50;
            let rotation = frame.addr() + 0x60;
            e.call(VECTOR_CONSTRUCT, &args![position]);
            e.call(VECTOR_CONSTRUCT, &args![rotation]);
            e.call(SAVE_READ, &args![save_load, position, 0xcu32]);
            e.call(SAVE_READ, &args![save_load, rotation, 0x10u32]);
            fn_005610f0(e, Ptr::new(body), Ptr::new(position));
            fn_00561150(e, Ptr::new(body), Ptr::new(rotation));
        }
        let mut active = 0u8;
        if e.get(state, HavokSaveData::cFlags) & 4 != 0 {
            let byte = frame.addr() + 0x48;
            e.call(SAVE_READ, &args![save_load, byte, 1u32]);
            active = e.mem.u8(byte);
        } else if e.get(state, HavokSaveData::cFlags) & 1 != 0 {
            active = 1;
        }
        if active != 0 {
            let first = frame.addr() + 0x70;
            let second = frame.addr() + 0x80;
            e.call(VECTOR_CONSTRUCT, &args![first]);
            e.call(VECTOR_CONSTRUCT, &args![second]);
            e.call(SAVE_READ, &args![save_load, first, 0xcu32]);
            e.call(SAVE_READ, &args![save_load, second, 0xcu32]);
            e.call(BODY_SET_FIRST_VECTOR, &args![body, first]);
            e.call(BODY_SET_SECOND_VECTOR, &args![body, second]);
            e.call(BODY_FLAG_SETTER, &args![body, 1u32]);
        } else {
            e.call(BODY_SET_FIRST_VECTOR, &args![body, ZERO_VECTOR]);
            e.call(BODY_SET_SECOND_VECTOR, &args![body, ZERO_VECTOR]);
            if was_set {
                e.call(BODY_FLAG_SETTER, &args![body, 0u32]);
            }
        }
    });
}

// Translated from 005610f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the body's position (virtual +0xdc) from the `NiPoint3` at
/// `vector`, converted to an `hkVector4` (`004a3e00`).
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn fn_005610f0(e: &mut Engine, this: Ptr, vector: Ptr) {
    e.with_stack(0x18, |e, scratch| {
        e.call(VECTOR_CONSTRUCT, &args![scratch]);
        let converted = e.call(NI_TO_VECTOR, &args![scratch, vector]).u32();
        e.vcall(this.addr(), 0xdc, &args![converted]);
    });
}

// Translated from 00561150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the body's rotation (virtual +0xe0) from the `NiQuaternion` at
/// `quaternion`: converted to an `hkQuaternion` (`00561500`) and
/// normalized (`fn_005611c0`).
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn fn_00561150(e: &mut Engine, this: Ptr, quaternion: Ptr) {
    e.with_stack(0x18, |e, scratch| {
        e.call(QUATERNION_CONSTRUCT, &args![scratch]);
        e.call(NI_QUATERNION_TO_HAVOK, &args![scratch, quaternion]);
        fn_005611c0(e, scratch);
        e.vcall(this.addr(), 0xe0, &args![scratch]);
    });
}

// Translated from 005611c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Normalizes the `hkQuaternion` at `this` (`fn_005611e0`).
pub fn fn_005611c0(e: &mut Engine, this: Ptr) {
    fn_005611e0(e, this);
}

// Translated from 005611e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Multiplies the four `float`s at `this` by `fn_005612a0`'s factor
/// (`MULPS` with the factor splatted by `fn_00561240`): the quaternion
/// divided by its length, with the length from one Newton step on the
/// `RSQRTSS` estimate.
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn fn_005611e0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x20, |e, scratch| {
        let factor = Ptr::new(scratch.addr());
        let splat = Ptr::new(scratch.addr() + 0x10);
        let factor = fn_005612a0(e, this, factor);
        let splat = fn_00561240(e, factor, splat);
        for lane in 0..4 {
            let value = e.mem.f32(this.addr() + 4 * lane);
            let by = e.mem.f32(splat.addr() + 4 * lane);
            e.mem.set_f32(this.addr() + 4 * lane, by * value);
        }
    });
}

// Translated from 00561240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the first `float` of the vector at `this` into all four lanes of
/// the vector at `dest` (`SHUFPS`); returns `dest`.
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn fn_00561240(e: &mut Engine, this: Ptr, dest: Ptr) -> Ptr {
    let bits = e.mem.u32(this.addr());
    for lane in 0..4 {
        e.mem.set_u32(dest.addr() + 4 * lane, bits);
    }
    dest
}

// Translated from 005612a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The factor that normalizes the four-lane vector at `this`: with `s` the
/// squared length (the first lane of `00561440(this, out, this)`, whose
/// four lanes all hold the dot product), `r = rsqrt(s)` (the `RSQRTSS`
/// estimate; 0 when `s` is 0) and `r * 0.5 * (3 - s * r * r)`, one Newton
/// step. Written to the first lane of `out` (the others are zero) through
/// `004a3c90`; returns `out`. All in single precision (SSE).
///
/// `RSQRTSS` is an estimate that differs between CPUs: the translation
/// uses the host's instruction on x86_64 (exact elsewhere).
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn fn_005612a0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let half: f32 = e.global(HALF);
    let three: f32 = e.global(THREE);
    let zero: f32 = e.global(ZERO_FLOAT);
    e.with_stack(0x20, |e, scratch| {
        let dot = e.call(0x0056_1440, &args![this, scratch, this]).u32();
        let dot = e.call(VECTOR_CONSTRUCT, &args![dot]).u32();
        let squared = e.mem.f32(dot);
        let estimate = rsqrt_estimate(squared);
        let times = squared * estimate;
        let times = estimate * times;
        let correction = three - times;
        let scaled = half * estimate;
        let newton = scaled * correction;
        let result = scratch.addr() + 0x10;
        e.mem
            .set_f32(result, if squared == zero { 0.0 } else { newton });
        for lane in 1..4 {
            e.mem.set_u32(result + 4 * lane, 0);
        }
        e.call(COPY_16_BYTES, &args![out, result]);
    });
    out
}

// ===========================================================================
// 00561440 to 00563890: the vector helpers and rigid-body setters of the
// Havok glue, the load of the older Havok record, the editor location,
// `Update3DPosition`, the buffer-based game-save and game-load handlers
// (`BGSSaveGameBuffer` / `BGSLoadFormBuffer`) and the Havok and animation
// records they write and read. The next function of the unit after this
// range is `00563d80`.

layout! {
    /// `BGSHavokSaveData` (Xbox PDB), 0x20 bytes: what the buffer-based save
    /// and load of a reference's rigid bodies keep while they walk its
    /// collision nodes. `cFlags`: 1 = active bodies only, 2 = the collision
    /// root's own body was seen, 4 = active and inactive bodies, or a
    /// keyframed one.
    pub struct BGSHavokSaveData: 0x20 {
        /// `cFlags` (Xbox PDB).
        0x00 cFlags: u8,
        /// `iActiveBoneCount` (Xbox PDB).
        0x04 iActiveBoneCount: u32,
        /// `iInactiveBoneCount` (Xbox PDB).
        0x08 iInactiveBoneCount: u32,
        /// `iKeyFramedBoneCount` (Xbox PDB).
        0x0C iKeyFramedBoneCount: u32,
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x10 pRef: Ptr,
        /// `pObj3D` (Xbox PDB): `NiAVObject*`.
        0x14 pObj3D: Ptr,
        /// `pCollisionRoot` (Xbox PDB): `bhkNiCollisionObject*`.
        0x18 pCollisionRoot: Ptr,
        /// `pSaveLoadBuffer` (Xbox PDB): the save or load buffer the
        /// callbacks write to or read from.
        0x1C pSaveLoadBuffer: Ptr,
    }

    /// The 0x1c-byte record `fn_00561860` builds on its stack and hands to
    /// the collision walker (`00c68900`) for the older Havok record: the same
    /// start as [`CollisionWalkContext`] (`bRecurse`, `iMode`), then the
    /// collision root, the reference and the flags `fn_00561b40` tests. Not
    /// in the Xbox PDB (a local type); the names describe the use.
    pub struct OldHavokLoadWalk: 0x1c {
        /// Non-zero: descend into the children.
        0x04 bRecurse: u8,
        /// `0x12`.
        0x08 iMode: u32,
        /// The reference's collision root (`004a8b00` of its 3D).
        0x0C pCollisionRoot: Ptr,
        /// The `TESObjectREFR` being loaded.
        0x10 pRef: Ptr,
        /// Bit 0: the saved record has velocities; bit 1: the pose is the
        /// reference's own (cleared once used).
        0x14 iFlags: u32,
        /// Always 0.
        0x18 iSpare: u32,
    }
}

/// `00a29680`: a bare `RET` in this build. `fn_00561580` and `fn_00561690`
/// call it on the body before and after they change its entity.
pub(crate) const BODY_LOCK_MARKER: u32 = 0x00a2_9680;
/// `hkpEntity::activate` and `hkpEntity::deactivate` (the map's names).
pub(crate) const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
pub(crate) const ENTITY_DEACTIVATE: u32 = 0x00c9_c240;
/// `NiPoint3` to `hkVector4` for the angular velocity
/// (`00553fc0(dest, source)`, cdecl): copies three floats and zeroes the
/// fourth lane.
pub(crate) const NI_POINT_TO_VECTOR_ZERO_W: u32 = 0x0055_3fc0;
/// The four component getters `fn_00561500` calls on the `NiQuaternion`
/// (each returns a `float` in `ST0`: the values at +4, +8, +0xc and +0, in
/// the order the `hkQuaternion` takes them).
pub(crate) const QUATERNION_COMPONENTS: [u32; 4] =
    [0x006b_9130, 0x0048_8d50, 0x0084_d030, 0x006a_7f50];
/// The SSE helpers `fn_00561730` chains (all `thiscall` on a stack
/// `hkVector4`): `00458a10(out; a, b)` is a `SUBPS`, `00458ad0(v; v)` an
/// `ANDPS`, `004589c0(v; float)` fills all four lanes, `00458a60(a; out, b)`
/// a `CMPLEPS` and `00458990(mask vector; bits)` tests that all the bits
/// are set in the mask.
pub(crate) const VECTOR_SUBTRACT: u32 = 0x0045_8a10;
pub(crate) const VECTOR_ABSOLUTE: u32 = 0x0045_8ad0;
pub(crate) const VECTOR_SPLAT: u32 = 0x0045_89c0;
pub(crate) const VECTOR_COMPARE: u32 = 0x0045_8a60;
pub(crate) const VECTOR_ALL_SET: u32 = 0x0045_8990;
/// The tolerance `0.001` the velocity setters pass (the same `float` as
/// [`RADIUS_EPSILON`]).
pub(crate) const VELOCITY_EPSILON: u32 = RADIUS_EPSILON;
/// `bhkCharacterController::SetPosition_ov2` (`00c6e390`, takes an
/// `hkVector4`) and `MobileObject::GetCharController` (`009306d0`).
pub(crate) const CONTROLLER_SET_POSITION: u32 = 0x00c6_e390;
pub(crate) const GET_CHARACTER_CONTROLLER: u32 = 0x0093_06d0;
/// `ExtraDataList::GetStartingWorldOrCell` (`0041b320`),
/// `TESObjectREFR::GetWorldSpace` (`00575d70`), the cell test `00425fd0`
/// that `InitItem` also applies to a parent cell, and
/// `TESObjectCELL::GetWorldSpace` (`0054ddd0`).
pub(crate) const EXTRA_GET_STARTING_SPACE: u32 = 0x0041_b320;
pub(crate) const GET_WORLD_SPACE: u32 = 0x0057_5d70;
pub(crate) const CELL_TEST: u32 = 0x0042_5fd0;
pub(crate) const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
/// The `TES` singleton and `TES::IsCellLoaded(TES; cell, flag)`
/// (`004511e0`).
pub(crate) const GLOBAL_TES: u32 = 0x011d_ea10;
pub(crate) const TES_IS_CELL_LOADED: u32 = 0x0045_11e0;
/// `GetLocationCellOrWorld(form, &cell, &world)` (`00846190`, cdecl),
/// `TESWorldSpace::GetCellFromWorldCoord(world; position)` (`00587550`),
/// `TESObjectREFR::SetLocationOnReference(this; position)` (`00575830`), the
/// rotation setter (`00575700`, three `float`s by value) and
/// `TESObjectREFR::MoveRefToNewSpace(this, cell, world)` (`00573800`, cdecl).
pub(crate) const GET_LOCATION_CELL_OR_WORLD: u32 = 0x0084_6190;
pub(crate) const WORLD_CELL_FROM_COORD: u32 = 0x0058_7550;
pub(crate) const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
pub(crate) const SET_ROTATION: u32 = 0x0057_5700;
pub(crate) const MOVE_REF_TO_NEW_SPACE: u32 = 0x0057_3800;
/// The `BGSSaveLoadGame` singleton (`iGlobalFlags` at +0x244, Xbox PDB) and
/// `BGSSaveLoadGame::GetChange(this; reference, change flags)`
/// (`0084a6d0`), with the change flags built by `008c71b0(cell; value)`.
pub(crate) const GLOBAL_SAVE_LOAD_GAME: u32 = 0x011d_df38;
pub(crate) const SAVE_LOAD_GET_CHANGE: u32 = 0x0084_a6d0;
pub(crate) const CHANGE_FLAGS_CONSTRUCT: u32 = 0x008c_71b0;
/// `BGSLoadFormBuffer::Header.iChangeFlags` (`00428110(buffer; out)`
/// copies the `BGSChangeFlags` at +0x17) and `iOldChangeFlags`
/// (`0042ce30(buffer; out)` copies the one at +0x2c); both return `out`.
/// `004280f0(flags; mask)` tells whether any bit of `mask` is set.
pub(crate) const BUFFER_CHANGE_FLAGS: u32 = 0x0042_8110;
pub(crate) const BUFFER_OLD_CHANGE_FLAGS: u32 = 0x0042_ce30;
pub(crate) const CHANGE_FLAGS_TEST: u32 = 0x0042_80f0;
/// The steps `SaveGame` and `LoadGame` of this unit start with:
/// `TESForm::SaveGame(buffer)` (`00484d60`), `TESForm::LoadGame(buffer)`
/// (`00484da0`) and the empty step `004534f0(this; buffer)`.
pub(crate) const FORM_SAVE_GAME_BUFFER: u32 = 0x0048_4d60;
pub(crate) const FORM_LOAD_GAME_BUFFER: u32 = 0x0048_4da0;
pub(crate) const EMPTY_BUFFER_STEP: u32 = 0x0045_34f0;
/// Buffer functions: `BGSSaveGameBuffer::SaveData(buffer; source, size, 0)`
/// (`00865e50`), `BGSLoadGameBuffer::LoadData(buffer; dest, size)`
/// (`00864980`), `BGSSaveGameBuffer::SaveString(buffer; text, 0)`
/// (`00865e70`), `BGSLoadGameBuffer::LoadString(buffer; dest)` (`008649a0`),
/// `StartVariableSizedValue` (`00865f20`), `SaveVariableSizedValue(buffer;
/// value)` (`00865f60`), `SaveVariableSizedValue_ov2(buffer; size, token)`
/// (`00865ff0`), `LoadVariableSizedValue` (`00864a60`) and
/// `BGSLoadGameSubBuffer::SaveGame(sub buffer; buffer)` (`00865510`).
pub(crate) const BUFFER_SAVE_DATA: u32 = 0x0086_5e50;
pub(crate) const BUFFER_LOAD_DATA: u32 = 0x0086_4980;
pub(crate) const BUFFER_SAVE_STRING: u32 = 0x0086_5e70;
pub(crate) const BUFFER_LOAD_STRING: u32 = 0x0086_49a0;
pub(crate) const BUFFER_START_VARIABLE: u32 = 0x0086_5f20;
pub(crate) const BUFFER_SAVE_VARIABLE: u32 = 0x0086_5f60;
pub(crate) const BUFFER_SAVE_VARIABLE_OV2: u32 = 0x0086_5ff0;
pub(crate) const BUFFER_LOAD_VARIABLE: u32 = 0x0086_4a60;
pub(crate) const SUB_BUFFER_SAVE_GAME: u32 = 0x0086_5510;
/// `SaveGameWarning(format, ...)` (`00845cc0`, cdecl): the buffer-based
/// loaders report with it where the older ones use [`MESSAGE`].
pub(crate) const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// `BGSSaveLoadGame::QueueSubBuffer(this; 0, buffer, 0)` (`0084a810`).
pub(crate) const QUEUE_SUB_BUFFER: u32 = 0x0084_a810;
/// `ExtraDataList` handlers: `SaveGame_ov2(list; buffer)` (`00426a30`),
/// `LoadGame_ov2(list; buffer)` (`00428150`), `0042ceb0(list; buffer, base)`,
/// `FinishLoadGame(list; buffer)` (`0042d9e0`), `0042dae0(list; buffer)`,
/// `0042dd90(list; buffer)`, `0041aeb0(list)`, `GetSavedAnimation(list)`
/// (`00422a10`) and `00422850(list; name)`, which stores the name of the last
/// finished sequence.
pub(crate) const EXTRA_SAVE_GAME_OV2: u32 = 0x0042_6a30;
pub(crate) const EXTRA_LOAD_GAME_OV2: u32 = 0x0042_8150;
pub(crate) const EXTRA_LOAD_GAME_WITH_BASE: u32 = 0x0042_ceb0;
pub(crate) const EXTRA_FINISH_LOAD_GAME: u32 = 0x0042_d9e0;
pub(crate) const EXTRA_LOAD_STEP: u32 = 0x0042_dae0;
pub(crate) const EXTRA_LOAD_LAST_STEP: u32 = 0x0042_dd90;
pub(crate) const EXTRA_STEP_BEFORE_CHANGES: u32 = 0x0041_aeb0;
pub(crate) const EXTRA_GET_SAVED_ANIMATION: u32 = 0x0042_2a10;
pub(crate) const EXTRA_SET_LAST_SEQUENCE: u32 = 0x0042_2850;
/// `InventoryChanges` handlers taking the buffer (`this` is the result of
/// [`GET_INVENTORY_CHANGES`]): `SaveGame` (`004d4090`), `LoadGame`
/// (`004d4160`) and `004d42f0`.
pub(crate) const INVENTORY_CHANGES_SAVE_GAME: u32 = 0x004d_4090;
pub(crate) const INVENTORY_CHANGES_LOAD_GAME: u32 = 0x004d_4160;
pub(crate) const INVENTORY_CHANGES_FINISH: u32 = 0x004d_42f0;
/// `00452370(this)`, the test the load handlers apply to a reference before
/// the destruction stage.
pub(crate) const REFR_HEALTH_GATE: u32 = 0x0045_2370;
/// `ExtraDataList::GetObjectHealth` (`0041b6b0`, `float` in `ST0`),
/// `BGSDestructibleObjectForm::GetDestructionForm(base)` (`00475400`,
/// cdecl), `UpdateCurrentDamageStage(form; reference, flag)` (`00476f50`)
/// and `TESObjectREFR::UpdateAddonNodeSounds(this, flag)` (`0057a3c0`).
pub(crate) const EXTRA_GET_OBJECT_HEALTH: u32 = 0x0041_b6b0;
pub(crate) const GET_DESTRUCTION_FORM: u32 = 0x0047_5400;
pub(crate) const UPDATE_DAMAGE_STAGE: u32 = 0x0047_6f50;
pub(crate) const UPDATE_ADDON_NODE_SOUNDS: u32 = 0x0057_a3c0;
/// `TESHavokUtilities::Should3DRagDoll(reference, 3D)` (`0062c3c0`, cdecl).
pub(crate) const SHOULD_3D_RAGDOLL: u32 = 0x0062_c3c0;
/// `TESObjectCELL::AddActivatingRef(cell; reference)` (`005455c0`),
/// `00545670(cell; reference)` and `TESObjectCELL::AttachReference3D(cell;
/// reference, flag)` (`00548880`).
pub(crate) const CELL_ADD_ACTIVATING_REF: u32 = 0x0054_55c0;
pub(crate) const CELL_REFERENCE_STEP: u32 = 0x0054_5670;
pub(crate) const CELL_ATTACH_REFERENCE_3D: u32 = 0x0054_8880;
/// `TESObjectREFR::ClearAction(this, 4)` (`00572db0`), `00572d50(this, 4)`,
/// `BGSOpenCloseForm::SnapHavokTo3D(reference)` (`0047b360`, cdecl) and the
/// navmesh obstacle manager (`006c0720` returns it; `OnDoorClose`
/// `006c0f10` and `006c0dc0` take the reference).
pub(crate) const REFR_CLEAR_ACTION: u32 = 0x0057_2db0;
pub(crate) const REFR_SET_ACTION: u32 = 0x0057_2d50;
pub(crate) const SNAP_HAVOK_TO_3D: u32 = 0x0047_b360;
pub(crate) const GET_OBSTACLE_MANAGER: u32 = 0x006c_0720;
pub(crate) const OBSTACLE_ON_DOOR_CLOSE: u32 = 0x006c_0f10;
pub(crate) const OBSTACLE_ON_DOOR_OPEN: u32 = 0x006c_0dc0;
/// `0042ce10(BGSSaveLoadGame)`, `00437b90(reference)`, `00450ff0(cell)`,
/// `TES::GetCellPriority(TES; cell, 0)` (`00458be0`) and
/// `ModelLoader::QueueReference(loader; reference, priority, 1)`
/// (`00444850`, loader at `011c3b3c`).
pub(crate) const SAVE_LOAD_GAME_TEST: u32 = 0x0042_ce10;
pub(crate) const REFR_3D_TEST: u32 = 0x0043_7b90;
pub(crate) const CELL_PRIORITY_TEST: u32 = 0x0045_0ff0;
pub(crate) const TES_GET_CELL_PRIORITY: u32 = 0x0045_8be0;
pub(crate) const GLOBAL_MODEL_LOADER: u32 = 0x011c_3b3c;
pub(crate) const MODEL_LOADER_QUEUE_REFERENCE: u32 = 0x0044_4850;
/// `00549580(base)`, `TESWorldSpace::GetTerrainManager(world)` (`00586170`)
/// and `BGSTerrainManager::HideTree(manager; reference, 1)` (`006fcfa0`).
pub(crate) const BASE_FORM_TEST: u32 = 0x0054_9580;
pub(crate) const WORLD_GET_TERRAIN_MANAGER: u32 = 0x0058_6170;
pub(crate) const TERRAIN_HIDE_TREE: u32 = 0x006f_cfa0;
/// The texts `fn_005636e0` swaps (`01011f30` and `0101abac`, which the
/// decompiler shows as "Close") and the `strcpy_s`-like
/// `00406d30(dest, size, source)` (cdecl).
pub(crate) const TEXT_OPEN_SIDE: u32 = 0x0101_1f30;
pub(crate) const TEXT_CLOSE_SIDE: u32 = 0x0101_abac;
pub(crate) const STRING_COPY: u32 = 0x0040_6d30;
/// Functions after this range that `fn_005636e0` and `fn_00563650` call:
/// `00563d80(this; manager, name, time)`, `SaveControllerManager`
/// (`00563f30(manager, buffer, blend)`, cdecl) and `00564010(manager,
/// buffer, blend)` (cdecl, the loader that follows `SaveControllerManager`).
pub(crate) const ANIMATION_LOAD_STEP: u32 = 0x0056_3d80;
pub(crate) const SAVE_CONTROLLER_MANAGER: u32 = 0x0056_3f30;
pub(crate) const CONTROLLER_MANAGER_LOADER: u32 = 0x0056_4010;
/// `00517630(body)` is compared with 4 (the Xbox name of the counter it
/// feeds, `iKeyFramedBoneCount`, suggests the keyframed motion type);
/// `004d9fa0(node, 0)` is the step all three loading callbacks take first;
/// `00437bd0(reference)` is the first of four tests `LoadHavokData` makes
/// of an actor.
pub(crate) const BODY_MOTION_KIND: u32 = 0x0051_7630;
pub(crate) const RAGDOLL_NODE_STEP: u32 = 0x004d_9fa0;
pub(crate) const REFR_STATE_TEST: u32 = 0x0043_7bd0;
/// A `bhkRigidBody` virtual (`+0xe4`) that `fn_00563380` calls with 4 when
/// bit 2 of a body's saved kind byte is set.
pub(crate) const BODY_SLOT_SET_KIND: u32 = 0xe4;
/// `0047ab40(reference, sequence)` (cdecl), the shadow-scene getter
/// `00450b80(0)` and `ShadowSceneNode::AddObject(scene; object)`
/// (`00b5eeb0`).
pub(crate) const GENERIC_SEQUENCE_STEP: u32 = 0x0047_ab40;
pub(crate) const SCENE_GETTER: u32 = 0x0045_0b80;
pub(crate) const SCENE_ADD_OBJECT: u32 = 0x00b5_eeb0;
/// The address of `LOADED_REF_DATA::m_spData3D` inside the loaded data, which
/// `00559450` dereferences.
pub(crate) const LOADED_DATA_3D_OFFSET: u32 = 0x14;
/// The extra-data type `GetEditorLocation` asks for (the editor location)
/// and the form type byte it compares a default space with.
pub(crate) const EXTRA_TYPE_EDITOR_LOCATION: u32 = 0xf;
pub(crate) const FORM_TYPE_CELL: u32 = 0x39;

/// Runs the default constructor the compiler calls on each stack
/// `NiPoint3` or `hkVector4` in `vectors`.
pub(crate) fn construct_vectors(e: &mut Engine, vectors: &[u32]) {
    for vector in vectors {
        e.call(VECTOR_CONSTRUCT, &args![*vector]);
    }
}

/// Whether the change flags `reader` copies out of `buffer`
/// ([`BUFFER_CHANGE_FLAGS`] or [`BUFFER_OLD_CHANGE_FLAGS`]) have any bit of
/// `mask`: the game copies them into a 4-byte local (`reader` returns it) and
/// asks `004280f0(flags; mask)`.
pub(crate) fn buffer_changed(e: &mut Engine, reader: u32, buffer: u32, mask: u32) -> bool {
    e.with_stack(4, |e, local| {
        let flags = e.call(reader, &args![buffer, local]).u32();
        e.call(CHANGE_FLAGS_TEST, &args![flags, mask]).bool()
    })
}

/// The change-flag mask for the extra data of a reference: `0xa4021c40`,
/// plus `0x3fc00` added (not or-ed) for an actor, which gives `0xa4061840`.
pub(crate) fn extra_change_mask(actor: bool) -> u32 {
    let extra = if actor { 0x3_fc00 } else { 0 };
    0xa402_1c40u32.wrapping_add(extra)
}

/// The save version byte of the save-load stream (`008df040`).
pub(crate) fn save_version(e: &mut Engine, save_load: u32) -> u8 {
    e.call(SAVE_VERSION, &args![save_load]).u8()
}

/// The part of the destruction-stage step the load handlers share: the
/// reference has loaded data (`00452370` holds) whose pointer at +0x14 is
/// set.
pub(crate) fn has_loaded_3d_data(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    if !e.call(REFR_HEALTH_GATE, &args![this]).bool() {
        return false;
    }
    let loaded = e.get(this, TESObjectREFR::pLoadedData);
    !loaded.is_null()
        && e.call(
            READ_FIRST_DWORD,
            &args![loaded.addr() + LOADED_DATA_3D_OFFSET],
        )
        .u32()
            != 0
}

/// Updates the damage stage from the base form's destruction form
/// (`UpdateCurrentDamageStage(reference, flag)`); when it changed, the
/// addon node sounds are updated.
pub(crate) fn update_damage_stage(e: &mut Engine, this: Ptr<TESObjectREFR>, flag: u32) {
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    let destruction = e.call(GET_DESTRUCTION_FORM, &args![base]).u32();
    if e.call(UPDATE_DAMAGE_STAGE, &args![destruction, this, flag])
        .bool()
    {
        e.call(UPDATE_ADDON_NODE_SOUNDS, &args![this, 0u32]);
    }
}

// Translated from 00561440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dot product of the two four-lane vectors `this` and `other`
/// (`MULPS`, then two shuffle-and-add steps), written to all four lanes of
/// the vector at `out` through `004a3c90` (the 16-byte copy); returns `out`.
/// The sums are added in the order `(p3 + p1) + (p2 + p0)` the shuffles give.
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn fn_00561440(e: &mut Engine, this: Ptr, out: Ptr, other: Ptr) -> Ptr {
    let mut products = [0f32; 4];
    for (lane, product) in products.iter_mut().enumerate() {
        let at = 4 * lane as u32;
        *product = e.mem.f32(this.addr() + at) * e.mem.f32(other.addr() + at);
    }
    let low = products[2] + products[0];
    let high = products[3] + products[1];
    let sum = high + low;
    e.with_stack(0x10, |e, result| {
        for lane in 0..4 {
            e.mem.set_f32(result.addr() + 4 * lane, sum);
        }
        e.call(COPY_16_BYTES, &args![out, result]);
    });
    out
}

// Translated from 00561500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts the `NiQuaternion` at `source` into the `hkQuaternion` at
/// `dest` (cdecl): the component getters of [`QUATERNION_COMPONENTS`]
/// (`ST0`, rounded to `float`) are stored into elements 0 to 3 of `dest`;
/// returns `dest`.
pub fn fn_00561500(e: &mut Engine, dest: Ptr, source: Ptr) -> Ptr {
    for (index, getter) in QUATERNION_COMPONENTS.iter().enumerate() {
        let value = e.call(*getter, &args![source]).f32();
        let at = fn_00560d30(e, dest, index as u32);
        e.mem.set_f32(at.addr(), value);
    }
    dest
}

// Translated from 00561580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Activates (`active` non-zero, `hkpEntity::activate`) or deactivates
/// (`hkpEntity::deactivate`) the Havok entity of the rigid body `this`,
/// between the two calls of the empty lock marker [`BODY_LOCK_MARKER`];
/// nothing for a body without an entity.
pub fn fn_00561580(e: &mut Engine, this: Ptr, active: u8) {
    let entity = e.call(BODY_ENTITY, &args![this]).u32();
    if entity != 0 {
        e.call(BODY_LOCK_MARKER, &args![this]);
        if active != 0 {
            e.call(ENTITY_ACTIVATE, &args![entity]);
        } else {
            e.call(ENTITY_DEACTIVATE, &args![entity]);
        }
        e.call(BODY_LOCK_MARKER, &args![this]);
    }
}

// Translated from 005615d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the body's linear velocity from the `NiPoint3` at `vector`:
/// converted to an `hkVector4` (`004a3e00`) and handed to `fn_00561690`.
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn fn_005615d0(e: &mut Engine, this: Ptr, vector: Ptr) {
    e.with_stack(0x10, |e, converted| {
        e.call(VECTOR_CONSTRUCT, &args![converted]);
        let result = e.call(NI_TO_VECTOR, &args![converted, vector]).u32();
        fn_00561690(e, this, Ptr::new(result));
    });
}

// Translated from 00561630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the body's angular velocity from the `NiPoint3` at `vector`:
/// converted to an `hkVector4` with a zero fourth lane (`00553fc0`) and
/// handed to `fn_005617c0`.
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn fn_00561630(e: &mut Engine, this: Ptr, vector: Ptr) {
    e.with_stack(0x10, |e, converted| {
        e.call(VECTOR_CONSTRUCT, &args![converted]);
        let result = e
            .call(NI_POINT_TO_VECTOR_ZERO_W, &args![converted, vector])
            .u32();
        fn_005617c0(e, this, Ptr::new(result));
    });
}

// Translated from 00561690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the linear velocity `velocity` (an `hkVector4`) of the body's entity
/// (`004ae750`) with the lock marker around it; nothing without an entity.
pub fn fn_00561690(e: &mut Engine, this: Ptr, velocity: Ptr) {
    let entity = e.call(BODY_ENTITY, &args![this]).u32();
    if entity != 0 {
        e.call(BODY_LOCK_MARKER, &args![this]);
        hkp_rigid_body_set_linear_velocity(e, Ptr::new(entity), velocity);
        e.call(BODY_LOCK_MARKER, &args![this]);
    }
}

// Translated from 005616d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::setLinearVelocity` (Xbox PDB): unless `velocity` is
/// within `0.001` ([`VELOCITY_EPSILON`]) of the entity's current linear
/// velocity (`fn_00560dc0`, compared by `fn_00561730`), activates the entity
/// and sets the velocity through the motion's virtual at +0x40.
pub fn hkp_rigid_body_set_linear_velocity(e: &mut Engine, this: Ptr, velocity: Ptr) {
    let epsilon: f32 = e.global(VELOCITY_EPSILON);
    let current = fn_00560dc0(e, this);
    if fn_00561730(e, velocity, current, epsilon) == 0 {
        e.call(ENTITY_ACTIVATE, &args![this]);
        let motion = e.call(BODY_VECTOR_PART, &args![this]).u32();
        e.vcall(motion, 0x40, &args![velocity]);
    }
}

// Translated from 00561730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the vector `this` is within `epsilon` of the vector `other` in
/// all four lanes: the difference (`SUBPS`), made absolute (`ANDPS`), is
/// compared lane by lane (`CMPLEPS`) with `epsilon` in all lanes, and all
/// four result bits must be set (mask `0xf`). Returns the 0 or 1 of
/// `00458990`.
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn fn_00561730(e: &mut Engine, this: Ptr, other: Ptr, epsilon: f32) -> u32 {
    e.with_stack(0x30, |e, frame| {
        let difference = frame.addr();
        let tolerance = frame.addr() + 0x10;
        let compared = frame.addr() + 0x20;
        e.call(VECTOR_CONSTRUCT, &args![difference]);
        e.call(VECTOR_SUBTRACT, &args![difference, this, other]);
        e.call(VECTOR_ABSOLUTE, &args![difference, difference]);
        e.call(VECTOR_CONSTRUCT, &args![tolerance]);
        e.call(VECTOR_SPLAT, &args![tolerance, epsilon]);
        let mask = e
            .call(VECTOR_COMPARE, &args![difference, compared, tolerance])
            .u32();
        e.call(VECTOR_ALL_SET, &args![mask, 0xfu32]).u32()
    })
}

// Translated from 005617c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the angular velocity `velocity` (an `hkVector4`) of the body's
/// entity with the lock marker around it; nothing without an entity.
pub fn fn_005617c0(e: &mut Engine, this: Ptr, velocity: Ptr) {
    let entity = e.call(BODY_ENTITY, &args![this]).u32();
    if entity != 0 {
        e.call(BODY_LOCK_MARKER, &args![this]);
        hkp_rigid_body_set_angular_velocity(e, Ptr::new(entity), velocity);
        e.call(BODY_LOCK_MARKER, &args![this]);
    }
}

// Translated from 00561800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRigidBody::setAngularVelocity` (Xbox PDB): like
/// [`hkp_rigid_body_set_linear_velocity`] against the entity's second
/// velocity vector (`fn_00560e50`), setting through the motion's virtual at
/// +0x44.
pub fn hkp_rigid_body_set_angular_velocity(e: &mut Engine, this: Ptr, velocity: Ptr) {
    let epsilon: f32 = e.global(VELOCITY_EPSILON);
    let current = fn_00560e50(e, this);
    if fn_00561730(e, velocity, current, epsilon) == 0 {
        e.call(ENTITY_ACTIVATE, &args![this]);
        let motion = e.call(BODY_VECTOR_PART, &args![this]).u32();
        e.vcall(motion, 0x44, &args![velocity]);
    }
}

// Translated from 00561860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the Havok record of a reference from a save older than version
/// 0x51 (`fn_00560530` sends them here); `size` is the record's size.
///
/// Without a loaded 3D the record is skipped, with a message. Otherwise it
/// reads the flags: saves before version 0x16 and 0x16 to 0x2a read a single
/// byte that sets bit 0 when it is non-zero (before 0x16 it is read before
/// the 3D is looked up), 0x2b and later read the flags byte itself. From
/// version 0x18 it reads the saved bone count and compares it with the
/// current one (`fn_00560350`): a different count skips the rest of the
/// record (`size - 2`), logs why and knocks an actor down. The same count
/// walks the collision nodes with `fn_00561b40`, freezes an actor's rigid
/// bodies when it is fixed (virtual +0x234) and gives the 3D a zero
/// velocity record.
pub fn fn_00561860(e: &mut Engine, this: Ptr<TESObjectREFR>, size: u16) {
    let me = this.addr();
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    e.with_stack(0x80, |e, frame| {
        let flags_slot = frame.addr();
        let legacy_slot = frame.addr() + 1;
        let saved_count_slot = frame.addr() + 2;
        let record: Ptr<HavokSaveData> = Ptr::new(frame.addr() + 0x10);
        let walk: Ptr<OldHavokLoadWalk> = Ptr::new(frame.addr() + 0x30);
        let vector = frame.addr() + 0x50;
        if save_version(e, save_load) < 0x16 {
            e.call(FORM_LOAD_DATA, &args![this, legacy_slot, 1u32]);
            if e.mem.u8(legacy_slot) != 0 {
                let flags = e.mem.u8(flags_slot);
                e.mem.set_u8(flags_slot, flags | 1);
            }
        }
        let obj3d = loaded_3d(e, me);
        if obj3d == 0 {
            let form_id = e.call(FORM_ID, &args![this]).u32();
            let name = e.vcall(me, 0x130, &args![]).u32();
            e.call(MESSAGE, &args![HAVOK_NO_3D_FORMAT, name, form_id]);
            e.call(SAVE_SKIP, &args![save_load, u32::from(size)]);
            return;
        }
        let actor = if is_actor(e, me) { me } else { 0 };
        if save_version(e, save_load) >= 0x2b {
            e.call(FORM_LOAD_DATA, &args![this, flags_slot, 1u32]);
        }
        if save_version(e, save_load) >= 0x16 && save_version(e, save_load) < 0x2b {
            e.call(FORM_LOAD_DATA, &args![this, legacy_slot, 1u32]);
            if e.mem.u8(legacy_slot) != 0 {
                let flags = e.mem.u8(flags_slot);
                e.mem.set_u8(flags_slot, flags | 1);
            }
        }
        if save_version(e, save_load) >= 0x18 {
            e.call(FORM_LOAD_DATA, &args![this, saved_count_slot, 1u32]);
            fn_0055ebf0(e, record);
            fn_00560350(e, this, record);
            let active = e.get(record, HavokSaveData::sActiveBoneCount);
            let inactive = e.get(record, HavokSaveData::sInactiveBoneCount);
            let current = (u32::from(active) + u32::from(inactive)) as u8;
            let saved = e.mem.u8(saved_count_slot);
            if current != saved {
                let form_id = e.call(FORM_ID, &args![this]).u32();
                let name = e.vcall(me, 0x130, &args![]).u32();
                e.call(
                    MESSAGE,
                    &args![
                        HAVOK_BONE_COUNT_FORMAT,
                        name,
                        form_id,
                        u32::from(saved),
                        u32::from(current)
                    ],
                );
                e.call(
                    SAVE_SKIP,
                    &args![save_load, u32::from(size).wrapping_sub(2)],
                );
                if actor != 0 {
                    let ragdoll = e.mem.u32(actor + 0xac);
                    if ragdoll != 0 {
                        e.call(DISABLE_RAGDOLL_ANIM, &args![ragdoll, 1u32]);
                    }
                    e.call(KNOCK_DOWN, &args![obj3d, ZERO_VECTOR, 1u32, 0.0f32, 0u32]);
                }
                return;
            }
        }
        e.set(walk, OldHavokLoadWalk::iMode, 0x12);
        e.set(walk, OldHavokLoadWalk::bRecurse, 1);
        let root = e.call(COLLISION_ROOT, &args![obj3d]).u32();
        e.set(walk, OldHavokLoadWalk::pCollisionRoot, Ptr::new(root));
        e.set(walk, OldHavokLoadWalk::pRef, this.cast());
        let flags = u32::from(e.mem.u8(flags_slot));
        e.set(walk, OldHavokLoadWalk::iFlags, flags);
        e.set(walk, OldHavokLoadWalk::iSpare, 0);
        e.call(WALK_COLLISION, &args![obj3d, walk, 0x0056_1b40u32]);
        if actor != 0 && e.vcall(actor, 0x234, &args![]).bool() {
            e.call(SET_FIXED, &args![obj3d, 1u32, 1u32]);
            e.call(SET_MOTION, &args![obj3d, 1u32, 1u32, 0u32, 1u32]);
            let group = e.vcall(actor, 0x1e4, &args![]).u32();
            if group != 0 {
                e.call(CLEAR_ANIM_GROUP, &args![group, 0x14u32, 0.0f32]);
                e.call(ANIM_FINISH, &args![group]);
            }
        }
        e.call(MAKE_VELOCITY, &args![vector, 0.0f32, 0u32, 0u32]);
        e.call(SET_3D_VELOCITY, &args![obj3d, vector]);
    });
}

// Translated from 00561b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback `fn_00561860` hands to the collision walker (`walk` is its
/// [`OldHavokLoadWalk`]): for each collision node `node` that is not skipped
/// and has a rigid body it reads the pose and the velocities from the save
/// game. A node whose object is named "Arrow" is skipped, and for an
/// actor so is one of the type at `011f9140`; an actor otherwise gets the
/// node's virtual +0xb0 (1, 0, 0) first.
///
/// The pose comes from the stream, unless flag 2 is set (saves from
/// version 0x2b) or, in older saves, the node is the collision root: then
/// it is the reference's own location and orientation and flag 2 is
/// cleared. The velocities are the two vectors of the stream and the body
/// is activated when bit 0 of the flags is set, else zero vectors.
pub fn fn_00561b40(e: &mut Engine, node: Ptr, walk: Ptr<OldHavokLoadWalk>) {
    let save_load = global_ptr(e, GLOBAL_SAVE_LOAD);
    let body_ref = e.call(NODE_BODY_REF, &args![node]).u32();
    e.call(RAGDOLL_NODE_STEP, &args![node, 0u32]);
    if e.call(NODE_OBJECT, &args![node]).u32() != 0 {
        let object = e.call(NODE_OBJECT, &args![node]).u32();
        if sequence_name(e, object) != 0 {
            let object = e.call(NODE_OBJECT, &args![node]).u32();
            let name = sequence_name(e, object);
            if e.call(STRING_COMPARE, &args![name, ARROW]).u32() == 0 {
                return;
            }
        }
    }
    let reference = e.get(walk, OldHavokLoadWalk::pRef).addr();
    if e.vcall(reference, 0x100, &args![]).bool() {
        let object = e.call(NODE_OBJECT, &args![node]).u32();
        if e.call(IS_OF_TYPE, &args![SKIPPED_NODE_TYPE, object]).bool() {
            return;
        }
        e.vcall(node.addr(), 0xb0, &args![1u32, 0u32, 0u32]);
    }
    if body_ref == 0 {
        return;
    }
    let body = node_rigid_body(e, body_ref);
    if body == 0 {
        return;
    }
    let body = Ptr::new(body);
    e.with_stack(0x80, |e, frame| {
        let position = frame.addr();
        let first = frame.addr() + 0x10;
        let second = frame.addr() + 0x20;
        let rotation = frame.addr() + 0x30;
        let quaternion = frame.addr() + 0x40;
        let matrix = frame.addr() + 0x50;
        construct_vectors(e, &[position, first, second, rotation]);
        let flags = e.get(walk, OldHavokLoadWalk::iFlags);
        let mut from_stream = flags & 2 == 0;
        if save_version(e, save_load) < 0x2b {
            from_stream = node != e.get(walk, OldHavokLoadWalk::pCollisionRoot);
        }
        if from_stream {
            e.call(SAVE_READ, &args![save_load, position, 0xcu32]);
            e.call(SAVE_READ, &args![save_load, rotation, 0x10u32]);
            fn_005610f0(e, body, Ptr::new(position));
            fn_00561150(e, body, Ptr::new(rotation));
        } else {
            e.set(walk, OldHavokLoadWalk::iFlags, flags & !2);
            let location = e.vcall(reference, 0x1f4, &args![]).u32();
            fn_005610f0(e, body, Ptr::new(location));
            e.call(VECTOR_CONSTRUCT, &args![quaternion]);
            let orientation = e.call(GET_ORIENTATION, &args![reference, matrix]).u32();
            e.call(QUATERNION_FROM_ROTATION, &args![quaternion, orientation]);
            fn_00561150(e, body, Ptr::new(quaternion));
        }
        if e.get(walk, OldHavokLoadWalk::iFlags) & 1 == 0 {
            fn_005615d0(e, body, Ptr::new(ZERO_VECTOR));
            fn_00561630(e, body, Ptr::new(ZERO_VECTOR));
        } else {
            e.call(SAVE_READ, &args![save_load, first, 0xcu32]);
            e.call(SAVE_READ, &args![save_load, second, 0xcu32]);
            fn_005615d0(e, body, Ptr::new(first));
            fn_00561630(e, body, Ptr::new(second));
            fn_00561580(e, body, 1);
        }
    });
}

// Translated from 00561d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetOriginalOpenByDefaultState` (Xbox PDB): bit 8 of the
/// reference's extra-data flags, inverted when the save-load game already
/// has the change `0x400000` (the open-state change) recorded for it
/// (`BGSSaveLoadGame::GetChange`, with the change flags built by
/// `008c71b0`).
pub fn tes_object_refr_get_original_open_by_default_state(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
) -> bool {
    let me = this.addr();
    let flags = e.with_stack(4, |e, cell| {
        e.call(CHANGE_FLAGS_CONSTRUCT, &args![cell, 0x0040_0000u32]);
        e.mem.u32(cell.addr())
    });
    let save_load_game = global_ptr(e, GLOBAL_SAVE_LOAD_GAME);
    let recorded = e
        .call(SAVE_LOAD_GET_CHANGE, &args![save_load_game, this, flags])
        .bool();
    let list = extra_list(e, me);
    let open = e.call(EXTRA_FLAG_TEST, &args![list, 8u32]).bool();
    if recorded {
        !open
    } else {
        open
    }
}

// Translated from 00561df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetEditorLocation` (Xbox PDB), virtual at +0x138: the
/// position and rotation the reference was placed with in the editor, from
/// its editor-location extra data (type 0xf); false without it.
///
/// `space` receives the world space or cell it applies to: the extra's
/// starting space, else the reference's world space (`00575d70`), else its
/// parent cell, else `default_cell` (a cell of form type 0x39 stands for
/// itself when `00425fd0` holds, else for its world space; any other form
/// stands for itself); false when none of them exists.
pub fn tes_object_refr_get_editor_location(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    position: Ptr,
    rotation: Ptr,
    space: Ptr,
    default_cell: Ptr,
) -> bool {
    let me = this.addr();
    let list = extra_list(e, me);
    let record = e
        .call(EXTRA_GET_DATA, &args![list, EXTRA_TYPE_EDITOR_LOCATION])
        .u32();
    if record == 0 {
        return false;
    }
    let list = extra_list(e, me);
    let starting = e.call(EXTRA_GET_STARTING_SPACE, &args![list]).u32();
    e.mem.set_u32(space.addr(), starting);
    if e.mem.u32(space.addr()) == 0 {
        let world = e.call(GET_WORLD_SPACE, &args![this]).u32();
        e.mem.set_u32(space.addr(), world);
        if e.mem.u32(space.addr()) == 0 {
            let parent = e.get(this, TESObjectREFR::pParentCell);
            if !parent.is_null() {
                e.mem.set_u32(space.addr(), parent.addr());
            } else if default_cell.is_null() {
                return false;
            } else if e.call(FORM_TYPE, &args![default_cell]).u32() == FORM_TYPE_CELL {
                if e.call(CELL_TEST, &args![default_cell]).bool() {
                    e.mem.set_u32(space.addr(), default_cell.addr());
                } else {
                    let world = e.call(CELL_GET_WORLD_SPACE, &args![default_cell]).u32();
                    e.mem.set_u32(space.addr(), world);
                }
            } else {
                e.mem.set_u32(space.addr(), default_cell.addr());
            }
        }
    }
    for word in 0..3 {
        let value = e.mem.u32(record + 0xc + 4 * word);
        e.mem.set_u32(position.addr() + 4 * word, value);
    }
    for word in 0..3 {
        let value = e.mem.u32(record + 0x18 + 4 * word);
        e.mem.set_u32(rotation.addr() + 4 * word, value);
    }
    true
}

// Translated from 00561ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::MoveToEditorLocation` (Xbox PDB): puts the reference
/// back where the editor placed it. Asks the editor location (virtual
/// +0x138) and resolves the cell it is in (`GetLocationCellOrWorld`, and
/// `GetCellFromWorldCoord` for an exterior). A reference that does not
/// persist needs that cell to be loaded. Sets the location, then the
/// rotation, moves the reference to its cell (an interior one, by
/// `00425fd0`) or world space and updates its 3D position. False when the
/// reference has no editor location or its cell is not loaded.
pub fn tes_object_refr_move_to_editor_location(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    default_cell: Ptr,
) -> bool {
    let me = this.addr();
    e.with_stack(0x30, |e, frame| {
        let position = frame.addr();
        let rotation = frame.addr() + 0xc;
        let space = frame.addr() + 0x18;
        let cell_slot = frame.addr() + 0x1c;
        let world_slot = frame.addr() + 0x20;
        construct_vectors(e, &[position, rotation]);
        e.mem.set_u32(space, 0);
        let found = e
            .vcall(me, 0x138, &args![position, rotation, space, default_cell])
            .bool();
        if !found {
            return false;
        }
        e.mem.set_u32(cell_slot, 0);
        e.mem.set_u32(world_slot, 0);
        let target = e.mem.u32(space);
        e.call(
            GET_LOCATION_CELL_OR_WORLD,
            &args![target, cell_slot, world_slot],
        );
        if e.mem.u32(world_slot) != 0 {
            let world = e.mem.u32(world_slot);
            let cell = e.call(WORLD_CELL_FROM_COORD, &args![world, position]).u32();
            e.mem.set_u32(cell_slot, cell);
        }
        if !e.call(GET_REF_PERSISTS, &args![this]).bool() {
            let cell = e.mem.u32(cell_slot);
            if cell == 0 {
                return false;
            }
            let tes = global_ptr(e, GLOBAL_TES);
            if !e.call(TES_IS_CELL_LOADED, &args![tes, cell, 0u32]).bool() {
                return false;
            }
        }
        e.call(SET_LOCATION_ON_REFERENCE, &args![this, position]);
        let words = [
            e.mem.u32(rotation),
            e.mem.u32(rotation + 4),
            e.mem.u32(rotation + 8),
        ];
        e.call(SET_ROTATION, &args![this, words[0], words[1], words[2]]);
        let cell = e.mem.u32(cell_slot);
        if cell != 0 && e.call(CELL_TEST, &args![cell]).bool() {
            e.call(MOVE_REF_TO_NEW_SPACE, &args![this, cell, 0u32]);
        } else {
            let world = e.mem.u32(world_slot);
            e.call(MOVE_REF_TO_NEW_SPACE, &args![this, 0u32, world]);
        }
        tes_object_refr_update_3d_position(e, this);
        true
    })
}

// Translated from 00562020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::Update3DPosition` (Xbox PDB): moves the reference's
/// loaded 3D to its location and orientation. For a mobile object
/// (virtual +0xfc) with a character controller the controller is moved to
/// the location first (`bhk_character_controller_set_position`). Then the
/// 3D gets the location (`00440460`) and orientation (`0043fa80`), the
/// Havok world is updated (`00c69f50`) and the 3D gets a zero velocity
/// record. Nothing without a loaded 3D.
pub fn tes_object_refr_update_3d_position(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let obj3d = loaded_3d(e, me);
    if obj3d == 0 {
        return;
    }
    if e.vcall(me, 0xfc, &args![]).bool() {
        let controller = e.call(GET_CHARACTER_CONTROLLER, &args![this]).u32();
        if controller != 0 {
            bhk_character_controller_set_position(e, Ptr::new(controller), Ptr::new(me + 0x30));
        }
    }
    e.call(SET_3D_LOCATION, &args![obj3d, me + 0x30]);
    e.with_stack(0x30, |e, frame| {
        let matrix = frame.addr();
        let record = frame.addr() + 0x24;
        let orientation = e.call(GET_ORIENTATION, &args![this, matrix]).u32();
        e.call(SET_3D_ORIENTATION, &args![obj3d, orientation]);
        e.call(UPDATE_POSITION, &args![obj3d, 1u32, 0u32]);
        e.call(MAKE_VELOCITY, &args![record, 0.0f32, 0u32, 0u32]);
        e.call(SET_3D_VELOCITY, &args![obj3d, record]);
    });
}

// Translated from 005620e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::SetPosition` (Xbox PDB): converts the `NiPoint3`
/// at `position` to an `hkVector4` (`004a3e00`) and sets it as the
/// controller's position (`SetPosition_ov2`, `00c6e390`).
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn bhk_character_controller_set_position(e: &mut Engine, this: Ptr, position: Ptr) {
    e.with_stack(0x10, |e, converted| {
        e.call(VECTOR_CONSTRUCT, &args![converted]);
        let result = e.call(NI_TO_VECTOR, &args![converted, position]).u32();
        e.call(CONTROLLER_SET_POSITION, &args![this, result]);
    });
}

// Translated from 00562140 (decompiled, FalloutNV.exe 1.4.0.525)
/// After the empty step `004534f0`, when the buffer's change flags have
/// `0x10000000`, the reference is not an actor, has a loaded 3D, the
/// save-load game's global flags do not have bit 4 (`fn_005621d0`) and
/// `fn_00563530` is false: clears `0x10000000` from the buffer's change
/// flags (`fn_005621f0`).
pub fn fn_00562140(e: &mut Engine, this: Ptr<TESObjectREFR>, buffer: Ptr) {
    let me = this.addr();
    e.call(EMPTY_BUFFER_STEP, &args![this, buffer]);
    if !buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), 0x1000_0000) {
        return;
    }
    if is_actor(e, me) || loaded_3d(e, me) == 0 {
        return;
    }
    let save_load_game = global_ptr(e, GLOBAL_SAVE_LOAD_GAME);
    if fn_005621d0(e, Ptr::new(save_load_game)) {
        return;
    }
    if fn_00563530(e, this) {
        return;
    }
    fn_005621f0(e, buffer, 0x1000_0000);
}

// Translated from 005621d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 4 of `BGSSaveLoadGame::iGlobalFlags` (+0x244, Xbox PDB) is
/// set.
pub fn fn_005621d0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x244) & 4 != 0
}

// Translated from 005621f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the bits of `mask` from the change flags
/// (`BGSLoadFormBuffer::Header.iChangeFlags`, at +0x17) of the buffer
/// (`fn_00562210`).
pub fn fn_005621f0(e: &mut Engine, this: Ptr, mask: u32) {
    fn_00562210(e, Ptr::new(this.addr() + 0x17), mask);
}

// Translated from 00562210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSChangeFlags` clear: `*this &= !mask`.
pub fn fn_00562210(e: &mut Engine, this: Ptr, mask: u32) {
    let flags = e.mem.u32(this.addr());
    e.mem.set_u32(this.addr(), !mask & flags);
}

// Translated from 00562230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SaveGame` (Xbox PDB, the overload taking the save
/// buffer): after the form's own step (`00484d60`) writes, for the change
/// flags of `buffer`: the scale (flag 0x10, 4 bytes at +0x3c), the extra data
/// (`0xa4021c40`, plus `0x3fc00` for an actor), the inventory changes
/// (`0x8000020`) and, for a non-actor with flag 0x10000000, the animation:
/// the saved animation of the extra data is either handed to a sub buffer
/// when the reference has no 3D (`00865510`) or written as a sized value
/// around `SaveAnimation` (`tes_object_refr_save_animation`).
pub fn tes_object_refr_save_game_ov2(e: &mut Engine, this: Ptr<TESObjectREFR>, buffer: Ptr) {
    let me = this.addr();
    e.call(FORM_SAVE_GAME_BUFFER, &args![this, buffer]);
    if buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), 0x10) {
        e.call(BUFFER_SAVE_DATA, &args![buffer, me + 0x3c, 4u32, 0u32]);
    }
    let actor = e.vcall(me, 0x100, &args![]).bool();
    let mask = extra_change_mask(actor);
    if buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), mask) {
        let list = extra_list(e, me);
        e.call(EXTRA_SAVE_GAME_OV2, &args![list, buffer]);
    }
    if buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), 0x0800_0020) {
        let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
        e.call(INVENTORY_CHANGES_SAVE_GAME, &args![changes, buffer]);
    }
    if buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), 0x1000_0000) && !is_actor(e, me) {
        let list = extra_list(e, me);
        let animation = e.call(EXTRA_GET_SAVED_ANIMATION, &args![list]).u32();
        e.with_stack(4, |e, cell| {
            e.call(CHANGE_FLAGS_CONSTRUCT, &args![cell, animation]);
            if loaded_3d(e, me) == 0 && e.call(READ_FIRST_DWORD, &args![cell]).u32() != 0 {
                e.call(SUB_BUFFER_SAVE_GAME, &args![cell, buffer]);
            } else {
                let token = e.call(BUFFER_START_VARIABLE, &args![buffer]).u32();
                let before = e.call(FORM_ID, &args![buffer]).u32();
                tes_object_refr_save_animation(e, this, buffer);
                let after = e.call(FORM_ID, &args![buffer]).u32();
                e.call(
                    BUFFER_SAVE_VARIABLE_OV2,
                    &args![buffer, after.wrapping_sub(before), token],
                );
            }
        });
    }
}

// Translated from 005623d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::LoadGame` (the overload taking the load buffer; the map
/// has no name for it): after the form's own step (`00484da0`), for the
/// change flags of `buffer`: 0x200000 empties a non-actor form, 0x10 reads
/// the scale (4 bytes at +0x3c) and applies it (`00567490`), the extra data
/// (`0xa4021c40`, plus `0x3fc00` for an actor), the inventory changes
/// (`0x8000020`), a queued sub buffer (0x10000000, non-actors) and the
/// open-state flag (0x400000, non-actors). It then applies a destruction
/// stage when the reference has loaded data and its health is not -1, and
/// ends with `fn_0055d6d0` on the loaded 3D.
pub fn fn_005623d0(e: &mut Engine, this: Ptr<TESObjectREFR>, buffer: Ptr) {
    let me = this.addr();
    e.call(FORM_LOAD_GAME_BUFFER, &args![this, buffer]);
    let actor = e.vcall(me, 0x100, &args![]).bool();
    if !actor && buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), 0x0020_0000) {
        e.call(FORM_SET_EMPTY, &args![this, 1u32]);
    }
    if buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), 0x10) {
        e.call(BUFFER_LOAD_DATA, &args![buffer, me + 0x3c, 4u32]);
        let scale = e.mem.f32(me + 0x3c);
        e.call(SET_SCALE, &args![this, scale]);
    }
    let mask = extra_change_mask(actor);
    if buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), mask) {
        let list = extra_list(e, me);
        e.call(EXTRA_LOAD_GAME_OV2, &args![list, buffer]);
    }
    if buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), 0x0800_0020) {
        let list = extra_list(e, me);
        e.call(EXTRA_STEP_BEFORE_CHANGES, &args![list]);
        let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
        e.call(INVENTORY_CHANGES_LOAD_GAME, &args![changes, buffer]);
    }
    if buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), 0x1000_0000) && !is_actor(e, me) {
        let save_load_game = global_ptr(e, GLOBAL_SAVE_LOAD_GAME);
        e.call(QUEUE_SUB_BUFFER, &args![save_load_game, 0u32, buffer, 0u32]);
    }
    if !is_actor(e, me) && buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), 0x0040_0000) {
        copy_open_state_flag(e, me);
    }
    if has_loaded_3d_data(e, this) {
        let list = extra_list(e, me);
        let health = e.call(EXTRA_GET_OBJECT_HEALTH, &args![list]).f32();
        let marker: f64 = e.global(MINUS_ONE_DOUBLE);
        if health as f64 != marker && e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
            update_damage_stage(e, this, 0);
        }
    }
    let obj3d = get_3d(e, me);
    fn_0055d6d0(e, this, Ptr::new(obj3d));
}

// Translated from 00562660 (decompiled, FalloutNV.exe 1.4.0.525)
/// After the empty step (`004534f0`): takes the disabled state from the
/// enable-state parent (`follow_enable_parent`); loads the extra data
/// (`0042ceb0`, flags `0xa4021c40` plus `0x3fc00` for an actor, with the
/// base object) and the inventory changes (`0x8000020`, `004d42f0`); and
/// hides the terrain tree of a disabled exterior object whose base form
/// passes `00549580`.
pub fn fn_00562660(e: &mut Engine, this: Ptr<TESObjectREFR>, buffer: Ptr) {
    let me = this.addr();
    e.call(EMPTY_BUFFER_STEP, &args![this, buffer]);
    let parent = e.call(ENABLE_PARENT, &args![this]).u32();
    if parent != 0 {
        follow_enable_parent(e, this, parent);
    }
    let actor = e.vcall(me, 0x100, &args![]).bool();
    let mask = extra_change_mask(actor);
    if buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), mask) {
        let base = e.get(this.at(TESObjectREFR::data), OBJ_REFR::pObjectReference);
        let list = extra_list(e, me);
        e.call(EXTRA_LOAD_GAME_WITH_BASE, &args![list, buffer, base]);
    }
    if buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), 0x0800_0020) {
        let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
        e.call(INVENTORY_CHANGES_FINISH, &args![changes, buffer]);
    }
    if e.call(GET_BASE_FORM, &args![this]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        if e.call(BASE_FORM_TEST, &args![base]).bool()
            && e.call(FORM_IS_DISABLED, &args![this]).bool()
        {
            let world = e.call(GET_WORLD_SPACE, &args![this]).u32();
            if world != 0 {
                let manager = e.call(WORLD_GET_TERRAIN_MANAGER, &args![world]).u32();
                e.call(TERRAIN_HIDE_TREE, &args![manager, this, 1u32]);
            }
        }
    }
}

// Translated from 005627c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::FinishLoadGame` (Xbox PDB): after the empty step
/// (`004534f0`), finishes the extra data (`0xa4021c40`, plus `0x3fc00` for an
/// actor); for a non-actor with change flag 0x800000 puts the open/close
/// state back (`tes_object_refr_get_original_open_by_default_state` picks
/// which way: `ClearAction(4)` or `00572d50(4)`, then `SnapHavokTo3D` and
/// the navmesh obstacle manager's `OnDoorClose` or its open counterpart);
/// adds the reference to its cell's activating references (flag
/// 0x4000000); and, unless the save-load game says so (`0042ce10`), either
/// runs the virtual +0x1cc (when `00437b90` holds) or queues the reference
/// for loading (`ModelLoader::QueueReference`) when it has no 3D and its
/// cell passes `00450ff0`.
pub fn tes_object_refr_finish_load_game(e: &mut Engine, this: Ptr<TESObjectREFR>, buffer: Ptr) {
    let me = this.addr();
    e.call(EMPTY_BUFFER_STEP, &args![this, buffer]);
    let actor = e.vcall(me, 0x100, &args![]).bool();
    let mask = extra_change_mask(actor);
    if buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), mask) {
        let list = extra_list(e, me);
        e.call(EXTRA_FINISH_LOAD_GAME, &args![list, buffer]);
    }
    if !is_actor(e, me) && buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), 0x0080_0000) {
        if tes_object_refr_get_original_open_by_default_state(e, this) {
            e.call(REFR_CLEAR_ACTION, &args![this, 4u32]);
            e.call(SNAP_HAVOK_TO_3D, &args![this]);
            let manager = e.call(GET_OBSTACLE_MANAGER, &args![]).u32();
            e.call(OBSTACLE_ON_DOOR_CLOSE, &args![manager, this]);
        } else {
            e.call(REFR_SET_ACTION, &args![this, 4u32]);
            e.call(SNAP_HAVOK_TO_3D, &args![this]);
            let manager = e.call(GET_OBSTACLE_MANAGER, &args![]).u32();
            e.call(OBSTACLE_ON_DOOR_OPEN, &args![manager, this]);
        }
    }
    let parent = e.get(this, TESObjectREFR::pParentCell);
    if !parent.is_null() && buffer_changed(e, BUFFER_CHANGE_FLAGS, buffer.addr(), 0x0400_0000) {
        e.call(CELL_ADD_ACTIVATING_REF, &args![parent, this]);
    }
    let save_load_game = global_ptr(e, GLOBAL_SAVE_LOAD_GAME);
    if e.call(SAVE_LOAD_GAME_TEST, &args![save_load_game]).bool() {
        return;
    }
    if e.call(REFR_3D_TEST, &args![this]).bool() {
        e.vcall(me, 0x1cc, &args![0u32, 0u32]);
    } else if get_3d(e, me) == 0 {
        let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
        if cell != 0 {
            let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
            if e.call(CELL_PRIORITY_TEST, &args![cell]).bool() {
                let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
                let tes = global_ptr(e, GLOBAL_TES);
                let priority = e.call(TES_GET_CELL_PRIORITY, &args![tes, cell, 0u32]).u32();
                let loader = global_ptr(e, GLOBAL_MODEL_LOADER);
                e.call(
                    MODEL_LOADER_QUEUE_REFERENCE,
                    &args![loader, this, priority, 1u32],
                );
            }
        }
    }
}

// Translated from 005629a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second step of loading a reference from the buffer, with the
/// buffer's old change flags (`BGSLoadFormBuffer::iOldChangeFlags`). After
/// the empty step (`004534f0`) it resets the loaded data's cached radius
/// to -1.0 (+0xc); empties a non-actor form for flag 0x200000
/// (`SetEmpty(0)`); remembers whether the extra data has a health other
/// than -1; loads the extra data (`0042dae0`); applies the destruction
/// stage when it had a health; runs `fn_00563890` for flag 0x10000000 on a
/// non-actor; runs `0042dd90`; for flag 0x8000020 either calls the virtual
/// +0x208 (when the buffer has bit 4 in its `iFlags`, `fn_00562d00`) or
/// resets the state (`00574920`, with 0 when flag 0x20 is set, else 1); for
/// a non-actor copies the open-state flag (0x400000) and sets the open
/// state; restores the rag doll data of a non-actor whose 3D should be a
/// rag doll; adds the reference to its cell's activating references
/// (0x4000000) and ends with `fn_0055d6d0`.
pub fn fn_005629a0(e: &mut Engine, this: Ptr<TESObjectREFR>, buffer: Ptr) {
    let me = this.addr();
    e.call(EMPTY_BUFFER_STEP, &args![this, buffer]);
    let loaded = e.get(this, TESObjectREFR::pLoadedData);
    if !loaded.is_null() {
        let marker: f32 = e.global(MINUS_ONE_FLOAT);
        e.set(
            loaded.cast::<LOADED_REF_DATA>(),
            LOADED_REF_DATA::fCachedRadius,
            marker,
        );
    }
    let actor = e.vcall(me, 0x100, &args![]).bool();
    if !actor && buffer_changed(e, BUFFER_OLD_CHANGE_FLAGS, buffer.addr(), 0x0020_0000) {
        e.call(FORM_SET_EMPTY, &args![this, 0u32]);
    }
    let list = extra_list(e, me);
    let health = e.call(EXTRA_GET_OBJECT_HEALTH, &args![list]).f32();
    let marker: f64 = e.global(MINUS_ONE_DOUBLE);
    let has_health = health as f64 != marker;
    let mask = extra_change_mask(actor);
    if buffer_changed(e, BUFFER_OLD_CHANGE_FLAGS, buffer.addr(), mask) {
        let list = extra_list(e, me);
        e.call(EXTRA_LOAD_STEP, &args![list, buffer]);
    }
    if has_health && has_loaded_3d_data(e, this) {
        update_damage_stage(e, this, 1);
    }
    if buffer_changed(e, BUFFER_OLD_CHANGE_FLAGS, buffer.addr(), 0x1000_0000) && !is_actor(e, me) {
        fn_00563890(e, this);
    }
    let list = extra_list(e, me);
    e.call(EXTRA_LOAD_LAST_STEP, &args![list, buffer]);
    if buffer_changed(e, BUFFER_OLD_CHANGE_FLAGS, buffer.addr(), 0x0800_0020) {
        if fn_00562d00(e, buffer) {
            e.vcall(me, 0x208, &args![0u32]);
        } else if buffer_changed(e, BUFFER_OLD_CHANGE_FLAGS, buffer.addr(), 0x20) {
            e.call(RESET_STATE, &args![this, 0u32]);
        } else {
            e.call(RESET_STATE, &args![this, 1u32]);
        }
    }
    if !is_actor(e, me) {
        if buffer_changed(e, BUFFER_OLD_CHANGE_FLAGS, buffer.addr(), 0x0040_0000) {
            copy_open_state_flag(e, me);
        }
        let state = e.call(GET_OPEN_STATE, &args![this, 8u32]).u8();
        e.call(SET_OPEN_STATE, &args![this, u32::from(state), 1u32]);
    }
    if get_3d(e, me) != 0 {
        let obj3d = get_3d(e, me);
        if e.call(SHOULD_3D_RAGDOLL, &args![this, obj3d]).bool() && !is_actor(e, me) {
            e.call(RESTORE_RAGDOLL_DATA, &args![this, 0u32]);
        }
    }
    let parent = e.get(this, TESObjectREFR::pParentCell);
    if !parent.is_null() && buffer_changed(e, BUFFER_OLD_CHANGE_FLAGS, buffer.addr(), 0x0400_0000) {
        e.call(CELL_REFERENCE_STEP, &args![parent, this]);
    }
    let obj3d = get_3d(e, me);
    fn_0055d6d0(e, this, Ptr::new(obj3d));
}

// Translated from 00562d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 4 of `BGSLoadFormBuffer::iFlags` (+0x28, Xbox PDB) is set.
pub fn fn_00562d00(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x28) & 4 != 0
}

// Translated from 00562d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts the rigid bodies of the reference's loaded 3D into the
/// [`BGSHavokSaveData`] `record` (`fn_00563120` does the counting for each
/// collision node) and sets its flags: 4 when there are active and
/// inactive bodies or any keyframed one, else 1 when there are active ones.
/// Fills the reference, the 3D and the collision root first; nothing
/// without a loaded 3D.
pub fn fn_00562d20(e: &mut Engine, this: Ptr<TESObjectREFR>, record: Ptr<BGSHavokSaveData>) {
    let me = this.addr();
    let obj3d = loaded_3d(e, me);
    if obj3d == 0 {
        return;
    }
    e.set(record, BGSHavokSaveData::pRef, this.cast());
    e.set(record, BGSHavokSaveData::pObj3D, Ptr::new(obj3d));
    let root = e.call(COLLISION_ROOT, &args![obj3d]).u32();
    e.set(record, BGSHavokSaveData::pCollisionRoot, Ptr::new(root));
    e.with_stack(0x10, |e, frame| {
        let walk: Ptr<CollisionWalkContext> = frame.cast();
        e.set(walk, CollisionWalkContext::iMode, 0x12);
        e.set(walk, CollisionWalkContext::bRecurse, 1);
        e.set(walk, CollisionWalkContext::pUserData, record.cast());
        e.call(WALK_COLLISION, &args![obj3d, walk, 0x0056_3120u32]);
    });
    let active = e.get(record, BGSHavokSaveData::iActiveBoneCount);
    let inactive = e.get(record, BGSHavokSaveData::iInactiveBoneCount);
    let flags = e.get(record, BGSHavokSaveData::cFlags);
    if active != 0 && inactive != 0 {
        e.set(record, BGSHavokSaveData::cFlags, flags | 4);
    } else if active != 0 {
        e.set(record, BGSHavokSaveData::cFlags, flags | 1);
    }
    if e.get(record, BGSHavokSaveData::iKeyFramedBoneCount) != 0 {
        let flags = e.get(record, BGSHavokSaveData::cFlags);
        e.set(record, BGSHavokSaveData::cFlags, flags | 4);
    }
}

// Translated from 00562de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SaveHavokData` (Xbox PDB): writes the Havok record of
/// the reference into the save `buffer`. Counts the bodies (`fn_00562d20`),
/// writes the flags byte, the sized value (active + inactive count) and
/// then walks the collision nodes with `fn_00563220`, which writes each
/// body. Nothing without a loaded 3D.
pub fn tes_object_refr_save_havok_data(e: &mut Engine, this: Ptr<TESObjectREFR>, buffer: Ptr) {
    let me = this.addr();
    let obj3d = loaded_3d(e, me);
    if obj3d == 0 {
        return;
    }
    e.with_stack(0x30, |e, frame| {
        let record: Ptr<BGSHavokSaveData> = frame.cast();
        let walk: Ptr<CollisionWalkContext> = Ptr::new(frame.addr() + 0x20);
        fn_00562e70(e, record);
        e.set(record, BGSHavokSaveData::pSaveLoadBuffer, buffer);
        fn_00562d20(e, this, record);
        e.call(BUFFER_SAVE_DATA, &args![buffer, record, 1u32, 0u32]);
        let total = e
            .get(record, BGSHavokSaveData::iActiveBoneCount)
            .wrapping_add(e.get(record, BGSHavokSaveData::iInactiveBoneCount));
        e.call(BUFFER_SAVE_VARIABLE, &args![buffer, total]);
        e.set(walk, CollisionWalkContext::iMode, 0x12);
        e.set(walk, CollisionWalkContext::bRecurse, 1);
        e.set(walk, CollisionWalkContext::pUserData, record.cast());
        e.call(WALK_COLLISION, &args![obj3d, walk, 0x0056_3220u32]);
    });
}

// Translated from 00562e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of [`BGSHavokSaveData`]: clears all 0x20 bytes
/// (the flags byte, then seven words); returns `this`.
pub fn fn_00562e70(e: &mut Engine, this: Ptr<BGSHavokSaveData>) -> Ptr<BGSHavokSaveData> {
    e.mem.set_u8(this.addr(), 0);
    for word in 1..8 {
        e.mem.set_u32(this.addr() + 4 * word, 0);
    }
    this
}

// Translated from 00562ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::LoadHavokData` (Xbox PDB): reads the Havok record
/// `tes_object_refr_save_havok_data` wrote. Without a loaded 3D only warns
/// (`SaveGameWarning`). Otherwise reads the flags byte and the saved body
/// count and compares it with the current one (`fn_00562d20`): a different
/// count warns and knocks an actor down (disabling its ragdoll animation
/// first); the same count sets the 3D's motion (`SetMotion`, kind 4 for an
/// actor that fails all four of `00437bd0`, virtual +0x22c(0), +0x2e8 and
/// +0x230, else 1), gives it a zero velocity record and walks the collision
/// nodes with `fn_00563380`, ending with the zero velocity record again.
pub fn tes_object_refr_load_havok_data(e: &mut Engine, this: Ptr<TESObjectREFR>, buffer: Ptr) {
    let me = this.addr();
    let obj3d = loaded_3d(e, me);
    if obj3d == 0 {
        let form_id = e.call(FORM_ID, &args![this]).u32();
        let name = e.vcall(me, 0x130, &args![]).u32();
        e.call(SAVE_GAME_WARNING, &args![HAVOK_NO_3D_FORMAT, name, form_id]);
        return;
    }
    e.with_stack(0x60, |e, frame| {
        let flags_slot = frame.addr();
        let record: Ptr<BGSHavokSaveData> = Ptr::new(frame.addr() + 0x10);
        let walk: Ptr<CollisionWalkContext> = Ptr::new(frame.addr() + 0x30);
        let velocity = frame.addr() + 0x40;
        e.call(BUFFER_LOAD_DATA, &args![buffer, flags_slot, 1u32]);
        let saved = e.call(BUFFER_LOAD_VARIABLE, &args![buffer]).u32();
        fn_00562e70(e, record);
        e.set(record, BGSHavokSaveData::pSaveLoadBuffer, buffer);
        fn_00562d20(e, this, record);
        let current = e
            .get(record, BGSHavokSaveData::iActiveBoneCount)
            .wrapping_add(e.get(record, BGSHavokSaveData::iInactiveBoneCount));
        if saved != current {
            let form_id = e.call(FORM_ID, &args![this]).u32();
            let name = e.vcall(me, 0x130, &args![]).u32();
            e.call(
                SAVE_GAME_WARNING,
                &args![HAVOK_BONE_COUNT_FORMAT, name, form_id, saved, current],
            );
            let actor = if is_actor(e, me) { me } else { 0 };
            if actor != 0 {
                let ragdoll = e.mem.u32(actor + 0xac);
                if ragdoll != 0 {
                    e.call(DISABLE_RAGDOLL_ANIM, &args![ragdoll, 1u32]);
                }
                e.call(KNOCK_DOWN, &args![obj3d, ZERO_VECTOR, 1u32, 0.0f32, 0u32]);
            }
            return;
        }
        let flags = e.mem.u8(flags_slot);
        e.set(record, BGSHavokSaveData::cFlags, flags);
        e.set(walk, CollisionWalkContext::iMode, 0x12);
        e.set(walk, CollisionWalkContext::bRecurse, 1);
        e.set(walk, CollisionWalkContext::pUserData, record.cast());
        let mut motion = 1u32;
        if is_actor(e, me) {
            let moving = e.call(REFR_STATE_TEST, &args![this]).bool()
                || e.vcall(me, 0x22c, &args![0u32]).bool()
                || e.vcall(me, 0x2e8, &args![]).bool()
                || e.vcall(me, 0x230, &args![]).bool();
            if !moving {
                motion = 4;
            }
        }
        e.call(SET_MOTION, &args![obj3d, motion, 1u32, 0u32, 0u32]);
        e.call(MAKE_VELOCITY, &args![velocity, 0.0f32, 0u32, 0u32]);
        e.call(SET_3D_VELOCITY, &args![obj3d, velocity]);
        e.call(WALK_COLLISION, &args![obj3d, walk, 0x0056_3380u32]);
        e.call(SET_3D_VELOCITY, &args![obj3d, velocity]);
    });
}

// Translated from 00563120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback `fn_00562d20` hands to the collision walker to count the
/// bodies into the [`BGSHavokSaveData`] in the walk's user data: for each
/// collision node with a rigid body it sets flag 2 when the node is the
/// collision root, counts the body as active or inactive and, when its
/// motion kind (`00517630`) is 4 and `fn_005631e0(0x200)` is false, as a
/// keyframed one.
pub fn fn_00563120(e: &mut Engine, node: Ptr, walk: Ptr<CollisionWalkContext>) {
    let state: Ptr<BGSHavokSaveData> = e.get(walk, CollisionWalkContext::pUserData).cast();
    let body_ref = e.call(NODE_BODY_REF, &args![node]).u32();
    if body_ref == 0 {
        return;
    }
    let body = node_rigid_body(e, body_ref);
    if body == 0 {
        return;
    }
    if node == e.get(state, BGSHavokSaveData::pCollisionRoot) {
        let flags = e.get(state, BGSHavokSaveData::cFlags);
        e.set(state, BGSHavokSaveData::cFlags, flags | 2);
    }
    if bhk_rigid_body_is_active(e, Ptr::new(body)) {
        let count = e.get(state, BGSHavokSaveData::iActiveBoneCount);
        e.set(
            state,
            BGSHavokSaveData::iActiveBoneCount,
            count.wrapping_add(1),
        );
    } else {
        let count = e.get(state, BGSHavokSaveData::iInactiveBoneCount);
        e.set(
            state,
            BGSHavokSaveData::iInactiveBoneCount,
            count.wrapping_add(1),
        );
    }
    if e.call(BODY_MOTION_KIND, &args![body]).u32() == 4 && !fn_005631e0(e, Ptr::new(body), 0x200) {
        let count = e.get(state, BGSHavokSaveData::iKeyFramedBoneCount);
        e.set(
            state,
            BGSHavokSaveData::iKeyFramedBoneCount,
            count.wrapping_add(1),
        );
    }
}

// Translated from 005631e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `fn_00563200(this, mask)` is non-zero.
pub fn fn_005631e0(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    fn_00563200(e, this, mask) != 0
}

// Translated from 00563200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x10` ANDed with `mask` (applied to a rigid body
/// here; the same body serves other classes through identical-code
/// folding).
pub fn fn_00563200(e: &mut Engine, this: Ptr, mask: u32) -> u32 {
    e.mem.u32(this.addr() + 0x10) & mask
}

// Translated from 00563220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback `tes_object_refr_save_havok_data` hands to the collision
/// walker to write the bodies into the buffer of the [`BGSHavokSaveData`]
/// in the walk's user data: for each collision node with a rigid body it
/// writes (except for the collision root) the position (12 bytes) and
/// rotation (16); if flag 4 is set a kind byte (bit 0: active; bit 1:
/// motion kind 4 and `fn_005631e0(0x200)` false); and for an active body
/// its two velocity vectors (12 bytes each).
pub fn fn_00563220(e: &mut Engine, node: Ptr, walk: Ptr<CollisionWalkContext>) {
    let state: Ptr<BGSHavokSaveData> = e.get(walk, CollisionWalkContext::pUserData).cast();
    let buffer = e.get(state, BGSHavokSaveData::pSaveLoadBuffer).addr();
    let body_ref = e.call(NODE_BODY_REF, &args![node]).u32();
    if body_ref == 0 {
        return;
    }
    let body = node_rigid_body(e, body_ref);
    if body == 0 {
        return;
    }
    let body = Ptr::new(body);
    if node != e.get(state, BGSHavokSaveData::pCollisionRoot) {
        e.with_stack(0x20, |e, frame| {
            let position = frame.addr();
            let rotation = frame.addr() + 0x10;
            construct_vectors(e, &[position, rotation]);
            bhk_rigid_body_get_position(e, body, Ptr::new(position));
            bhk_rigid_body_get_rotation(e, body, Ptr::new(rotation));
            e.call(BUFFER_SAVE_DATA, &args![buffer, position, 0xcu32, 0u32]);
            e.call(BUFFER_SAVE_DATA, &args![buffer, rotation, 0x10u32, 0u32]);
        });
    }
    let active = bhk_rigid_body_is_active(e, body);
    if e.get(state, BGSHavokSaveData::cFlags) & 4 != 0 {
        e.with_stack(4, |e, kind| {
            let mut bits = 0u8;
            if active {
                bits |= 1;
            }
            if e.call(BODY_MOTION_KIND, &args![body]).u32() == 4 && !fn_005631e0(e, body, 0x200) {
                bits |= 2;
            }
            e.mem.set_u8(kind.addr(), bits);
            e.call(BUFFER_SAVE_DATA, &args![buffer, kind, 1u32, 0u32]);
        });
    }
    if active {
        e.with_stack(0x20, |e, frame| {
            let first = frame.addr();
            let second = frame.addr() + 0x10;
            construct_vectors(e, &[first, second]);
            fn_00560d50(e, body, Ptr::new(first));
            fn_00560de0(e, body, Ptr::new(second));
            e.call(BUFFER_SAVE_DATA, &args![buffer, first, 0xcu32, 0u32]);
            e.call(BUFFER_SAVE_DATA, &args![buffer, second, 0xcu32, 0u32]);
        });
    }
}

// Translated from 00563380 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback `tes_object_refr_load_havok_data` hands to the collision
/// walker to read the bodies, the reverse of `fn_00563220`: for each
/// collision node with a rigid body it sets the pose from the buffer (for
/// the collision root, when flag 2 is set, from the reference's own location
/// and orientation, clearing the flag), then reads the kind byte if flag 4
/// is set (bit 0 makes the body active, bit 1 calls the body's virtual
/// +0xe4 with 4) and, for an active body (flag 1 or bit 0), the two
/// velocity vectors, and activates the body.
pub fn fn_00563380(e: &mut Engine, node: Ptr, walk: Ptr<CollisionWalkContext>) {
    let state: Ptr<BGSHavokSaveData> = e.get(walk, CollisionWalkContext::pUserData).cast();
    let buffer = e.get(state, BGSHavokSaveData::pSaveLoadBuffer).addr();
    let body_ref = e.call(NODE_BODY_REF, &args![node]).u32();
    e.call(RAGDOLL_NODE_STEP, &args![node, 0u32]);
    if body_ref == 0 {
        return;
    }
    let body = node_rigid_body(e, body_ref);
    if body == 0 {
        return;
    }
    let body = Ptr::new(body);
    e.with_stack(0x80, |e, frame| {
        let position = frame.addr();
        let rotation = frame.addr() + 0x10;
        let matrix = frame.addr() + 0x20;
        let kind = frame.addr() + 0x50;
        let first = frame.addr() + 0x60;
        let second = frame.addr() + 0x70;
        construct_vectors(e, &[position, rotation]);
        let flags = e.get(state, BGSHavokSaveData::cFlags);
        if flags & 2 != 0 {
            e.set(state, BGSHavokSaveData::cFlags, flags & !2);
            let reference = e.get(state, BGSHavokSaveData::pRef).addr();
            let location = e.vcall(reference, 0x1f4, &args![]).u32();
            for word in 0..3 {
                let value = e.mem.u32(location + 4 * word);
                e.mem.set_u32(position + 4 * word, value);
            }
            let orientation = e.call(GET_ORIENTATION, &args![reference, matrix]).u32();
            e.call(QUATERNION_FROM_ROTATION, &args![rotation, orientation]);
        } else {
            e.call(BUFFER_LOAD_DATA, &args![buffer, position, 0xcu32]);
            e.call(BUFFER_LOAD_DATA, &args![buffer, rotation, 0x10u32]);
        }
        fn_00561150(e, body, Ptr::new(rotation));
        fn_005610f0(e, body, Ptr::new(position));
        let mut active = e.get(state, BGSHavokSaveData::cFlags) & 1 != 0;
        if e.get(state, BGSHavokSaveData::cFlags) & 4 != 0 {
            e.mem.set_u8(kind, 0);
            e.call(BUFFER_LOAD_DATA, &args![buffer, kind, 1u32]);
            let bits = e.mem.u8(kind);
            if bits & 1 != 0 {
                active = true;
            }
            if bits & 2 != 0 {
                e.vcall(body.addr(), BODY_SLOT_SET_KIND, &args![4u32]);
            }
        }
        if active {
            construct_vectors(e, &[first, second]);
            e.call(BUFFER_LOAD_DATA, &args![buffer, first, 0xcu32]);
            e.call(BUFFER_LOAD_DATA, &args![buffer, second, 0xcu32]);
            fn_005615d0(e, body, Ptr::new(first));
            fn_00561630(e, body, Ptr::new(second));
            fn_00561580(e, body, 1);
        }
    });
}

// Translated from 00563530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the reference's animation state is saved with its Havok data:
/// true when the extra data has a last finished sequence, or when the
/// loaded data's 3D (+0x14) has a controller manager with a generic
/// sequence (`008041a0`) whose name is neither of the two table entries
/// (`011977d8`, `01197820`); false otherwise.
pub fn fn_00563530(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let me = this.addr();
    let list = extra_list(e, me);
    if e.call(EXTRA_GET_LAST_SEQUENCE, &args![list]).u32() != 0 {
        return true;
    }
    let loaded = e.get(this, TESObjectREFR::pLoadedData);
    if loaded.is_null() {
        return false;
    }
    if e.call(
        READ_FIRST_DWORD,
        &args![loaded.addr() + LOADED_DATA_3D_OFFSET],
    )
    .u32()
        == 0
    {
        return false;
    }
    let obj3d = e
        .call(
            READ_FIRST_DWORD,
            &args![loaded.addr() + LOADED_DATA_3D_OFFSET],
        )
        .u32();
    let controller = controller_of(e, obj3d);
    let manager = manager_of(e, controller);
    if manager == 0 || !e.call(MANAGER_HAS_SEQUENCES, &args![manager]).bool() {
        return false;
    }
    let count = e.call(MANAGER_SEQUENCE_COUNT, &args![manager]).u32();
    for index in 0..count {
        let sequence = e.call(MANAGER_SEQUENCE_AT, &args![manager, index]).u32();
        if e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() == 0 {
            continue;
        }
        let first = e.global::<u32>(NAME_SEQUENCE_A);
        let name = sequence_name(e, sequence);
        if e.call(SEQUENCE_NAME_COMPARE, &args![name, first]).u32() == 0 {
            continue;
        }
        let second = e.global::<u32>(NAME_SEQUENCE_B);
        let name = sequence_name(e, sequence);
        if e.call(SEQUENCE_NAME_COMPARE, &args![name, second]).u32() == 0 {
            continue;
        }
        return true;
    }
    false
}

// Translated from 00563650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SaveAnimation` (Xbox PDB): writes the last finished
/// sequence's name (`BGSSaveGameBuffer::SaveString`) and the controller
/// manager of the loaded 3D (`SaveControllerManager`, `00563f30`) with the
/// default blend marker -1.0.
pub fn tes_object_refr_save_animation(e: &mut Engine, this: Ptr<TESObjectREFR>, buffer: Ptr) {
    let me = this.addr();
    let mut manager = 0;
    let obj3d = loaded_3d(e, me);
    if obj3d != 0 && controller_of(e, obj3d) != 0 {
        let controller = controller_of(e, obj3d);
        manager = manager_of(e, controller);
    }
    let list = extra_list(e, me);
    let last = e.call(EXTRA_GET_LAST_SEQUENCE, &args![list]).u32();
    e.call(BUFFER_SAVE_STRING, &args![buffer, last, 0u32]);
    let marker: f32 = e.global(MINUS_ONE_FLOAT);
    e.call(SAVE_CONTROLLER_MANAGER, &args![manager, buffer, marker]);
}

// Translated from 005636e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads what `tes_object_refr_save_animation` wrote: the name of the last
/// finished sequence (`BGSLoadGameBuffer::LoadString`, up to 0x104 bytes),
/// stored in the extra data (`00422850`) when it is not empty. For an
/// open/close form the names of the two sides (`01011f30` and `0101abac`)
/// are swapped (`strcpy_s`-like `00406d30`) and the sequence is loaded
/// (`00563d80`) with a time of -`FLT_MAX` when it was swapped, else
/// `FLT_MAX`. Then the controller manager is loaded (`00564010`) with the
/// default blend marker -1.0.
///
/// The stack-protector cookie of the compiled function is not translated.
pub fn fn_005636e0(e: &mut Engine, this: Ptr<TESObjectREFR>, buffer: Ptr) {
    let me = this.addr();
    let mut manager = 0;
    let obj3d = loaded_3d(e, me);
    if obj3d != 0 && controller_of(e, obj3d) != 0 {
        let controller = controller_of(e, obj3d);
        manager = manager_of(e, controller);
    }
    e.with_stack(0x110, |e, name| {
        e.call(BUFFER_LOAD_STRING, &args![buffer, name]);
        if e.call(STRING_LENGTH, &args![name]).u32() != 0 {
            let list = extra_list(e, me);
            e.call(EXTRA_SET_LAST_SEQUENCE, &args![list, name]);
            let mut swapped = false;
            let base = e.call(GET_BASE_FORM, &args![this]).u32();
            if e.call(IS_OPEN_CLOSE_FORM, &args![base]).bool() {
                if e.call(SEQUENCE_NAME_COMPARE, &args![name, TEXT_OPEN_SIDE])
                    .u32()
                    == 0
                {
                    e.call(STRING_COPY, &args![name, 0x104u32, TEXT_CLOSE_SIDE]);
                    swapped = true;
                } else if e
                    .call(SEQUENCE_NAME_COMPARE, &args![name, TEXT_CLOSE_SIDE])
                    .u32()
                    == 0
                {
                    e.call(STRING_COPY, &args![name, 0x104u32, TEXT_OPEN_SIDE]);
                    swapped = true;
                }
            }
            let maximum: f32 = e.global(FLOAT_MAX);
            let time = if swapped { -maximum } else { maximum };
            e.call(ANIMATION_LOAD_STEP, &args![this, manager, name, time]);
        }
    });
    let marker: f32 = e.global(MINUS_ONE_FLOAT);
    e.call(CONTROLLER_MANAGER_LOADER, &args![manager, buffer, marker]);
}

// Translated from 00563890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts the animation of a freshly loaded reference back in its rest
/// state, like `fn_0055f970` with a different second half. If the last
/// finished sequence of the reference is "Unequip", or its controller
/// manager has a generic-location sequence named "Unequip", or its 3D has
/// a morpher controller, the reference's location is set from its base
/// object (virtual +0x178), the virtuals +0x1cc and +0x1c4 run, and a
/// loaded parent cell re-attaches the 3D (`AttachReference3D`). Then, for
/// the manager: each generic sequence is handled by `0047ab40`, the manager
/// is deactivated, its sequence 0 is restarted with the offset -`FLT_MAX`
/// (its offset time is the 3D's velocity record), the manager is deactivated
/// again and the two table sequences (`011977d8`, `01197820`) found by name
/// are restarted the same way. It ends by deactivating the manager once
/// more and, for a reference reset in the first part, adding its 3D to the
/// shadow scene (`ShadowSceneNode::AddObject`).
///
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_00563890(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let mut reset = false;
    let obj3d = loaded_3d(e, me);
    if obj3d != 0 {
        let list = extra_list(e, me);
        let last = e.call(EXTRA_GET_LAST_SEQUENCE, &args![list]).u32();
        if last != 0 {
            let unequip = e.global::<u32>(NAME_UNEQUIP);
            if e.call(SEQUENCE_NAME_COMPARE, &args![last, unequip]).u32() == 0 {
                reset = true;
            }
        }
        if !reset && controller_of(e, obj3d) != 0 {
            let controller = controller_of(e, obj3d);
            let manager = manager_of(e, controller);
            if manager != 0 {
                let sequence = find_sequence(e, manager, NAME_UNEQUIP);
                if sequence != 0 && e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() != 0 {
                    reset = true;
                }
            }
        }
        if !reset && e.call(HAS_MORPHER_CONTROLLER, &args![obj3d]).bool() {
            reset = true;
        }
        if reset {
            let base = e.get(this.at(TESObjectREFR::data), OBJ_REFR::pObjectReference);
            let location = e.vcall(base.addr(), 0x178, &args![this]).u32();
            e.vcall(me, 0x1cc, &args![location, 1u32]);
            e.vcall(me, 0x1c4, &args![]);
            let parent = e.get(this, TESObjectREFR::pParentCell);
            if !parent.is_null() {
                let tes = global_ptr(e, GLOBAL_TES);
                if e.call(TES_IS_CELL_LOADED, &args![tes, parent, 0u32]).bool() {
                    e.call(CELL_ATTACH_REFERENCE_3D, &args![parent, this, 0u32]);
                }
            }
        }
    }
    let obj3d = loaded_3d(e, me);
    if obj3d != 0 && controller_of(e, obj3d) != 0 {
        let controller = controller_of(e, obj3d);
        let manager = manager_of(e, controller);
        if manager != 0 {
            let mut index = 0u32;
            while index < e.call(MANAGER_SEQUENCE_COUNT, &args![manager]).u32() {
                let sequence = e.call(MANAGER_SEQUENCE_AT, &args![manager, index]).u32();
                if sequence != 0 && e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() != 0 {
                    e.call(GENERIC_SEQUENCE_STEP, &args![this, sequence]);
                }
                index += 1;
            }
            e.call(MANAGER_DEACTIVATE_ALL, &args![manager, 0.0f32]);
            e.call(MANAGER_SET_FLAG, &args![manager, 1u32]);
            let first_sequence = e.call(MANAGER_SEQUENCE_AT, &args![manager, 0u32]).u32();
            if first_sequence != 0 {
                e.call(
                    MANAGER_ACTIVATE,
                    &args![manager, first_sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
                );
                restart_offset(e, obj3d, first_sequence);
            }
            e.call(MANAGER_DEACTIVATE_ALL, &args![manager, 0.0f32]);
            e.call(MANAGER_SET_FLAG, &args![manager, 0u32]);
            let first = find_sequence(e, manager, NAME_SEQUENCE_A);
            let second = find_sequence(e, manager, NAME_SEQUENCE_B);
            if first != 0 || second != 0 {
                e.call(MANAGER_SET_FLAG, &args![manager, 1u32]);
                for sequence in [first, second] {
                    if sequence != 0 {
                        if e.call(SEQUENCE_IS_GENERIC, &args![sequence]).u32() == 0 {
                            e.call(
                                MANAGER_ACTIVATE,
                                &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
                            );
                        }
                        restart_offset(e, obj3d, sequence);
                    }
                }
            }
        }
    }
    let controller = if obj3d != 0 {
        controller_of(e, obj3d)
    } else {
        0
    };
    let manager = manager_of(e, controller);
    if manager != 0 {
        e.call(MANAGER_DEACTIVATE_ALL, &args![manager, 0.0f32]);
    }
    if reset && obj3d != 0 {
        let scene = e.call(SCENE_GETTER, &args![0u32]).u32();
        e.call(SCENE_ADD_OBJECT, &args![scene, obj3d]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00426720, fn_00426720(Ptr, u32, u32, Ptr)),
        entry!(0x004267d0, fn_004267d0(Ptr, u32, Ptr)),
        entry!(0x004269c0, fn_004269c0(Ptr, u32, Ptr)),
        entry!(
            0x004b0130,
            ni_t_pointer_map_tes_object_refr_p_bool_delete_item(Ptr, Ptr)
        ),
        entry!(
            0x0055a2f0,
            tes_object_refr_tes_object_refr(Ptr<TESObjectREFR>) -> Ptr<TESObjectREFR>
        ),
        entry!(0x0055a3b0, fn_0055a3b0(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x0055a3d0,
            fn_0055a3d0(Ptr<TESObjectREFR>, u32) -> Ptr<TESObjectREFR>
        ),
        entry!(0x0055a400, fn_0055a400(Ptr<OBJ_REFR>) -> Ptr<OBJ_REFR>),
        entry!(0x0055a430, fn_0055a430(Ptr<TESObjectREFR>)),
        entry!(0x0055abe0, fn_0055abe0(Ptr<TESObjectREFR>) -> bool),
        entry!(0x0055ac00, fn_0055ac00(Ptr) -> Ptr),
        entry!(0x0055ac20, fn_0055ac20(Ptr)),
        entry!(0x0055ac40, fn_0055ac40(Ptr, Ptr<TESObjectREFR>)),
        entry!(0x0055ac70, fn_0055ac70() -> u32),
        entry!(0x0055ac80, fn_0055ac80()),
        entry!(0x0055ac90, fn_0055ac90(Ptr<TESObjectREFR>)),
        entry!(0x0055ad20, tes_object_refr_clear_data(Ptr<TESObjectREFR>)),
        entry!(0x0055ae70, tes_object_refr_save(Ptr<TESObjectREFR>)),
        entry!(
            0x0055af90,
            tes_object_refr_load(Ptr<TESObjectREFR>, Ptr) -> bool
        ),
        entry!(0x0055b980, fn_0055b980(Ptr) -> u32),
        entry!(0x0055b9a0, fn_0055b9a0(Ptr) -> bool),
        entry!(0x0055b9c0, fn_0055b9c0(Ptr<TESObjectREFR>, Ptr)),
        entry!(
            0x0055c050,
            tes_object_refr_create_duplicate_form(
                Ptr<TESObjectREFR>,
                u32,
                Ptr,
            ) -> Ptr<TESObjectREFR>
        ),
        entry!(0x0055c3e0, tes_object_refr_copy(Ptr<TESObjectREFR>, Ptr)),
        entry!(0x0055d230, fn_0055d230(Ptr<TESObjectREFR>, Ptr) -> bool),
        entry!(
            0x0055d310,
            tes_object_refr_has_container(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(0x0055d3b0, fn_0055d3b0(Ptr<TESObjectREFR>, Ptr) -> bool),
        entry!(0x0055d420, fn_0055d420(Ptr<TESObjectREFR>, u8)),
        entry!(0x0055d480, fn_0055d480(Ptr<TESObjectREFR>) -> u32),
        entry!(0x0055d520, fn_0055d520(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x0055d540,
            tes_object_refr_get_form_detailed_string(Ptr<TESObjectREFR>, Ptr)
        ),
        entry!(0x0055d6d0, fn_0055d6d0(Ptr<TESObjectREFR>, Ptr)),
        entry!(0x0055d760, tes_object_refr_init_item(Ptr<TESObjectREFR>)),
        entry!(0x0055e1d0, fn_0055e1d0(Ptr)),
        entry!(0x0055e200, fn_0055e200(Ptr) -> bool),
        entry!(0x0055e250, fn_0055e250(Ptr<TESObjectREFR>, Ptr) -> bool),
        entry!(0x0055e3f0, fn_0055e3f0(Ptr<TESObjectREFR>, Ptr) -> bool),
        entry!(
            0x0055e4d0,
            fn_0055e4d0(Ptr<TESObjectREFR>, Ptr, u8, u8) -> bool
        ),
        entry!(0x0055e5e0, fn_0055e5e0(Ptr<TESObjectREFR>, Ptr, Ptr)),
        entry!(0x0055e730, fn_0055e730(Ptr<TESObjectREFR>, u32) -> u16),
        entry!(0x0055e230, fn_0055e230(f32)),
        entry!(
            0x0055e940,
            tes_object_refr_save_game(Ptr<TESObjectREFR>, u32)
        ),
        entry!(
            0x0055ebf0,
            fn_0055ebf0(Ptr<HavokSaveData>) -> Ptr<HavokSaveData>
        ),
        entry!(
            0x0055ec40,
            tes_object_refr_load_game(Ptr<TESObjectREFR>, u32, u32)
        ),
        entry!(0x0055f240, fn_0055f240(Ptr<TESObjectREFR>, u32)),
        entry!(0x0055f5b0, fn_0055f5b0(Ptr) -> bool),
        entry!(0x0055f5f0, fn_0055f5f0(Ptr<TESObjectREFR>, u32, u32)),
        entry!(
            0x0055f780,
            tes_object_refr_finish_init_load_game(Ptr<TESObjectREFR>, u32, u32)
        ),
        entry!(0x0055f880, fn_0055f880(Ptr<TESObjectREFR>) -> u16),
        entry!(0x0055f900, fn_0055f900(Ptr<TESObjectREFR>)),
        entry!(0x0055f970, fn_0055f970(Ptr<TESObjectREFR>)),
        entry!(0x0055fdb0, fn_0055fdb0(Ptr) -> u16),
        entry!(0x0055fe70, fn_0055fe70(Ptr, f32)),
        entry!(0x0055ffa0, fn_0055ffa0(Ptr, Ptr, f32)),
        entry!(
            0x00560350,
            fn_00560350(Ptr<TESObjectREFR>, Ptr<HavokSaveData>) -> u16
        ),
        entry!(
            0x005604b0,
            fn_005604b0(Ptr<TESObjectREFR>, Ptr<HavokSaveData>)
        ),
        entry!(0x00560530, fn_00560530(Ptr<TESObjectREFR>, u16)),
        entry!(0x00560870, fn_00560870(Ptr, Ptr<CollisionWalkContext>)),
        entry!(0x005609b0, bhk_rigid_body_is_active(Ptr) -> bool),
        entry!(0x005609f0, fn_005609f0(Ptr) -> bool),
        entry!(0x00560a10, fn_00560a10(Ptr, Ptr<CollisionWalkContext>)),
        entry!(0x00560bc0, bhk_rigid_body_get_position(Ptr, Ptr)),
        entry!(0x00560c20, bhk_rigid_body_get_rotation(Ptr, Ptr)),
        entry!(0x00560c80, fn_00560c80(Ptr, Ptr) -> Ptr),
        entry!(0x00560cf0, fn_00560cf0(Ptr, f32)),
        entry!(0x00560d10, fn_00560d10(Ptr, u32) -> Ptr),
        entry!(0x00560d30, fn_00560d30(Ptr, u32) -> Ptr),
        entry!(0x00560d50, fn_00560d50(Ptr, Ptr)),
        entry!(0x00560d80, fn_00560d80(Ptr) -> Ptr),
        entry!(0x00560dc0, fn_00560dc0(Ptr) -> Ptr),
        entry!(0x00560de0, fn_00560de0(Ptr, Ptr)),
        entry!(0x00560e10, fn_00560e10(Ptr) -> Ptr),
        entry!(0x00560e50, fn_00560e50(Ptr) -> Ptr),
        entry!(0x00560e70, fn_00560e70(Ptr, Ptr<CollisionWalkContext>)),
        entry!(0x005610f0, fn_005610f0(Ptr, Ptr)),
        entry!(0x00561150, fn_00561150(Ptr, Ptr)),
        entry!(0x005611c0, fn_005611c0(Ptr)),
        entry!(0x005611e0, fn_005611e0(Ptr)),
        entry!(0x00561240, fn_00561240(Ptr, Ptr) -> Ptr),
        entry!(0x005612a0, fn_005612a0(Ptr, Ptr) -> Ptr),
        entry!(0x00561440, fn_00561440(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00561500, fn_00561500(Ptr, Ptr) -> Ptr),
        entry!(0x00561580, fn_00561580(Ptr, u8)),
        entry!(0x005615d0, fn_005615d0(Ptr, Ptr)),
        entry!(0x00561630, fn_00561630(Ptr, Ptr)),
        entry!(0x00561690, fn_00561690(Ptr, Ptr)),
        entry!(0x005616d0, hkp_rigid_body_set_linear_velocity(Ptr, Ptr)),
        entry!(0x00561730, fn_00561730(Ptr, Ptr, f32) -> u32),
        entry!(0x005617c0, fn_005617c0(Ptr, Ptr)),
        entry!(0x00561800, hkp_rigid_body_set_angular_velocity(Ptr, Ptr)),
        entry!(0x00561860, fn_00561860(Ptr<TESObjectREFR>, u16)),
        entry!(0x00561b40, fn_00561b40(Ptr, Ptr<OldHavokLoadWalk>)),
        entry!(
            0x00561d90,
            tes_object_refr_get_original_open_by_default_state(Ptr<TESObjectREFR>) -> bool
        ),
        entry!(
            0x00561df0,
            tes_object_refr_get_editor_location(Ptr<TESObjectREFR>, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00561ef0,
            tes_object_refr_move_to_editor_location(Ptr<TESObjectREFR>, Ptr) -> bool
        ),
        entry!(
            0x00562020,
            tes_object_refr_update_3d_position(Ptr<TESObjectREFR>)
        ),
        entry!(0x005620e0, bhk_character_controller_set_position(Ptr, Ptr)),
        entry!(0x00562140, fn_00562140(Ptr<TESObjectREFR>, Ptr)),
        entry!(0x005621d0, fn_005621d0(Ptr) -> bool),
        entry!(0x005621f0, fn_005621f0(Ptr, u32)),
        entry!(0x00562210, fn_00562210(Ptr, u32)),
        entry!(
            0x00562230,
            tes_object_refr_save_game_ov2(Ptr<TESObjectREFR>, Ptr)
        ),
        entry!(0x005623d0, fn_005623d0(Ptr<TESObjectREFR>, Ptr)),
        entry!(0x00562660, fn_00562660(Ptr<TESObjectREFR>, Ptr)),
        entry!(
            0x005627c0,
            tes_object_refr_finish_load_game(Ptr<TESObjectREFR>, Ptr)
        ),
        entry!(0x005629a0, fn_005629a0(Ptr<TESObjectREFR>, Ptr)),
        entry!(0x00562d00, fn_00562d00(Ptr) -> bool),
        entry!(
            0x00562d20,
            fn_00562d20(Ptr<TESObjectREFR>, Ptr<BGSHavokSaveData>)
        ),
        entry!(
            0x00562de0,
            tes_object_refr_save_havok_data(Ptr<TESObjectREFR>, Ptr)
        ),
        entry!(
            0x00562e70,
            fn_00562e70(Ptr<BGSHavokSaveData>) -> Ptr<BGSHavokSaveData>
        ),
        entry!(
            0x00562ed0,
            tes_object_refr_load_havok_data(Ptr<TESObjectREFR>, Ptr)
        ),
        entry!(0x00563120, fn_00563120(Ptr, Ptr<CollisionWalkContext>)),
        entry!(0x005631e0, fn_005631e0(Ptr, u32) -> bool),
        entry!(0x00563200, fn_00563200(Ptr, u32) -> u32),
        entry!(0x00563220, fn_00563220(Ptr, Ptr<CollisionWalkContext>)),
        entry!(0x00563380, fn_00563380(Ptr, Ptr<CollisionWalkContext>)),
        entry!(0x00563530, fn_00563530(Ptr<TESObjectREFR>) -> bool),
        entry!(
            0x00563650,
            tes_object_refr_save_animation(Ptr<TESObjectREFR>, Ptr)
        ),
        entry!(0x005636e0, fn_005636e0(Ptr<TESObjectREFR>, Ptr)),
        entry!(0x00563890, fn_00563890(Ptr<TESObjectREFR>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Registers `addrs` as doubles that do nothing and return 0.
    fn stub(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// A double that returns `value` in `eax`.
    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// An engine with the small accessors the unit leans on registered with
    /// the bodies the exe gives them, and the pages of the globals and
    /// vtables the tests set mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        // 005d43c0: this + 0x44
        e.register(GET_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        // 006815c0: returns its `this`
        e.register(LIST_ITEM_SLOT, |_, a| a[0].into_ret());
        // 00726070: the node's next pointer
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        // 00559450: the first dword
        e.register(READ_FIRST_DWORD, |e, a| e.mem.u32(a[0]).into_ret());
        // 008d6f30, 007af430, 00891170: pParentCell, pObjectReference, &data
        e.register(GET_PARENT_CELL, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(GET_DATA, |_, a| (a[0] + 0x20).into_ret());
        // 00401170 and 0084e3a0: the form type byte and the form ID
        e.register(FORM_TYPE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        // form flag tests: 0x20 deleted, 0x800 disabled, 0x4000, 0x8
        e.register(FORM_IS_DELETED, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x20 != 0).into_ret()
        });
        e.register(FORM_IS_DISABLED, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x800 != 0).into_ret()
        });
        e.register(0x0040_77c0, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x4000 != 0).into_ret()
        });
        e.register(0x0040_13e0, |e, a| {
            (e.mem.u32(a[0] + 8) & 8 != 0).into_ret()
        });
        // 0041210b's flag test (the loading flag) is off unless a test turns it on
        stub(&mut e, &[LOADING_FLAG]);
        // the pages of the globals and vtables
        for page in [
            0x0101_2000,
            0x0101_5000,
            0x0101_7000,
            0x0102_f000,
            0x011c_a000,
            0x011c_3000,
            0x011d_d000,
            0x011d_e000,
            0x011d_f000,
            0x011f_4000,
            0x0118_7000,
            0x0120_2000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(CONSTANT_ONE, 1.0f64);
        e
    }

    /// The `TESObjectREFR` vtable with the given (byte offset, target) slots,
    /// and a child-cell vtable whose slot 0 is `00575bb0`-like `target`.
    fn refr_vtable(e: &mut Engine, slots: &[(u32, u32)], cell_getter: u32) {
        for (offset, target) in slots {
            e.mem.set_u32(REFR_VTABLE + offset, *target);
        }
        e.mem.set_u32(CHILD_CELL_VTABLE, cell_getter);
    }

    fn new_refr(e: &mut Engine) -> Ptr<TESObjectREFR> {
        let refr: Ptr<TESObjectREFR> = e.new_object();
        e.mem.set_u32(refr.addr(), REFR_VTABLE);
        e.set(refr, TESObjectREFR::vfptrTESChildCell, CHILD_CELL_VTABLE);
        refr
    }

    /// An object with a vtable of its own: `slots` as (byte offset, target).
    fn object_with_vtable(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x500);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        e.mem.set_u32(object, vtable);
        object
    }

    /// A form object with a type byte, ID and flags at the PC offsets.
    fn form(e: &mut Engine, kind: u8, id: u32, flags: u32) -> u32 {
        let form = e.mem.alloc(0x100);
        e.mem.set_u8(form + 4, kind);
        e.mem.set_u32(form + 8, flags);
        e.mem.set_u32(form + 0xc, id);
        form
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn addresses(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter().map(|(a, _)| *a).collect()
    }

    // 0x100 slot: IsActor is true for a non-null `actor` object made by this.
    fn actor_object(e: &mut Engine) -> u32 {
        e.register(0x00ff_0001, |_, _| true.into_ret());
        object_with_vtable(e, 0x20, &[(0x100, 0x00ff_0001)])
    }

    fn plain_object(e: &mut Engine) -> u32 {
        e.register(0x00ff_0002, |_, _| false.into_ret());
        object_with_vtable(e, 0x20, &[(0x100, 0x00ff_0002)])
    }

    // ---- 00426720 -------------------------------------------------------

    #[test]
    fn handler_00426720_acts_on_flag_20000_for_non_actors_only() {
        let mut e = engine();
        stub(&mut e, &[0x0048_3710]);
        returns(&mut e, 0x0041_8460, 0x1234);
        let this = e.mem.alloc(0x40);
        let plain = plain_object(&mut e);
        let actor = actor_object(&mut e);

        e.call_log = Some(vec![]);
        e.call(0x0042_6720, &args![this, 0x20000u32, 0u32, plain]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0041_8460), vec![vec![this]]);
        assert_eq!(calls_to(&log, 0x0048_3710), vec![vec![0x1234]]);

        // a null target counts as a non-actor
        e.call_log = Some(vec![]);
        e.call(0x0042_6720, &args![this, 0x20000u32, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0048_3710).len(), 1);

        // an actor target, or another flag, does nothing
        e.call_log = Some(vec![]);
        e.call(0x0042_6720, &args![this, 0x20000u32, 0u32, actor]);
        e.call(0x0042_6720, &args![this, 0x1u32, 0u32, plain]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0048_3710).is_empty());
    }

    // ---- 004267d0 -------------------------------------------------------

    fn handler_004267d0_engine() -> (Engine, u32, u32, u32) {
        let mut e = engine();
        stub(
            &mut e,
            &[
                0x0041_ae90,
                0x0042_2920,
                0x0041_cc60,
                0x0041_d930,
                0x0041_b060,
                0x0042_2720,
                0x0042_2670,
                EXTRA_REMOVE_TYPE,
                0x0041_9dc0,
                0x0041_aff0,
                0x0042_e040,
                0x0042_16b0,
                0x0041_e0d0,
                0x0041_de40,
                0x0055_f5b0,
            ],
        );
        returns(&mut e, 0x0041_8460, 0);
        returns(&mut e, 0x0041_de00, 0);
        let save_load = e.mem.alloc(8);
        e.set_global(GLOBAL_SAVE_LOAD, save_load);
        let this = e.mem.alloc(0x40);
        let plain = plain_object(&mut e);
        let actor = actor_object(&mut e);
        (e, this, plain, actor)
    }

    #[test]
    fn handler_004267d0_removes_teleport_and_finished_sequence() {
        let (mut e, this, plain, _) = handler_004267d0_engine();
        returns(&mut e, 0x0041_8460, 7);
        e.call_log = Some(vec![]);
        e.call(0x0042_67d0, &args![this, 0x1002_0000u32, plain]);
        let log = e.call_log.take().unwrap();
        // teleport extra present: removed; then the finished sequence
        assert_eq!(calls_to(&log, 0x0041_ae90), vec![vec![this]]);
        assert_eq!(calls_to(&log, 0x0042_2920), vec![vec![this]]);

        // no teleport pointer: not removed
        returns(&mut e, 0x0041_8460, 0);
        e.call_log = Some(vec![]);
        e.call(0x0042_67d0, &args![this, 0x20000u32, plain]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0041_ae90).is_empty());
        assert!(calls_to(&log, 0x0042_2920).is_empty());
    }

    #[test]
    fn handler_004267d0_clears_an_actors_extras_when_the_save_object_asks() {
        let (mut e, this, _, actor) = handler_004267d0_engine();
        returns(&mut e, 0x0055_f5b0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0042_67d0, &args![this, 0u32, actor]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = addresses(&log)
            .into_iter()
            .skip(1)
            .filter(|a| *a != 0x00ff_0001)
            .collect();
        assert_eq!(
            order,
            vec![
                0x0055_f5b0,
                0x0041_cc60,
                0x0041_d930,
                0x0041_b060,
                0x0042_2720,
                0x0042_2670,
                EXTRA_REMOVE_TYPE,
                0x0041_9dc0,
                0x0041_aff0,
                0x0042_e040,
                0x0042_16b0
            ]
        );
        assert_eq!(calls_to(&log, EXTRA_REMOVE_TYPE), vec![vec![this, 0x3a]]);
        assert_eq!(calls_to(&log, 0x0041_9dc0), vec![vec![this, 0]]);
    }

    #[test]
    fn handler_004267d0_returns_a_non_actors_dropped_item() {
        let (mut e, this, plain, _) = handler_004267d0_engine();
        returns(&mut e, 0x0055_f5b0, 1);
        let dropper = e.mem.alloc(0x80);
        returns(&mut e, 0x0041_de00, dropper);
        e.call_log = Some(vec![]);
        e.call(0x0042_67d0, &args![this, 0u32, plain]);
        let log = e.call_log.take().unwrap();
        // RemoveDroppedItem on the dropper's extra list with the target,
        // then AddDroppedItem(0) on this list
        assert_eq!(
            calls_to(&log, 0x0041_e0d0),
            vec![vec![dropper + 0x44, plain]]
        );
        assert_eq!(calls_to(&log, 0x0041_de40), vec![vec![this, 0]]);

        // the save object says no: nothing at all
        returns(&mut e, 0x0055_f5b0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0042_67d0, &args![this, 0u32, plain]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0041_e0d0).is_empty());
    }

    // ---- 004269c0 -------------------------------------------------------

    #[test]
    fn handler_004269c0_moves_an_actor_out_of_its_cell() {
        let mut e = engine();
        stub(&mut e, &[EXTRA_SET_PERSISTENT_CELL, CELL_REMOVE_REFERENCE]);
        let this = e.mem.alloc(0x40);
        let persistent = e.mem.alloc(0x10);
        returns(&mut e, EXTRA_GET_PERSISTENT_CELL, persistent);
        let actor = actor_object(&mut e);
        e.mem.set_u32(actor + 0x40, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x0042_69c0, &args![this, 0u32, actor]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, EXTRA_SET_PERSISTENT_CELL),
            vec![vec![this, 0]]
        );
        assert_eq!(
            calls_to(&log, CELL_REMOVE_REFERENCE),
            vec![vec![persistent, actor]]
        );
    }

    #[test]
    fn handler_004269c0_ignores_non_actors_and_actors_without_a_cell() {
        let mut e = engine();
        stub(&mut e, &[EXTRA_SET_PERSISTENT_CELL, CELL_REMOVE_REFERENCE]);
        let this = e.mem.alloc(0x40);
        returns(&mut e, EXTRA_GET_PERSISTENT_CELL, 0x1111);
        let plain = plain_object(&mut e);
        let actor = actor_object(&mut e); // parent cell field is 0
        e.call_log = Some(vec![]);
        e.call(0x0042_69c0, &args![this, 0u32, plain]);
        e.call(0x0042_69c0, &args![this, 0u32, actor]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, CELL_REMOVE_REFERENCE).is_empty());
        // no persistent cell either
        returns(&mut e, EXTRA_GET_PERSISTENT_CELL, 0);
        e.mem.set_u32(actor + 0x40, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x0042_69c0, &args![this, 0u32, actor]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, CELL_REMOVE_REFERENCE).is_empty());
    }

    // ---- 004b0130 -------------------------------------------------------

    #[test]
    fn pointer_map_delete_item_clears_the_value_and_frees_the_node() {
        let mut e = engine();
        stub(&mut e, &[0x0045_cee0]);
        let map = e.mem.alloc(0x20);
        let item = e.mem.alloc(0x10);
        e.mem.set_u8(item + 8, 1);
        e.call_log = Some(vec![]);
        e.call(0x004b_0130, &args![map, item]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(item + 8), 0);
        assert_eq!(calls_to(&log, 0x0045_cee0), vec![vec![map + 0xc, item]]);
    }

    // ---- 0055a2f0 / 0055a400 / 0055ac90 ----------------------------------

    fn reset_engine() -> Engine {
        let mut e = engine();
        // 00403d30: memset(dst, value, count)
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        stub(&mut e, &[0x0057_c300]);
        e.set_global(LOADED_DATA_DEFAULT_HEIGHT, -2000.0f32);
        e
    }

    #[test]
    fn data_reset_zeroes_all_but_the_base_object_and_restores_the_scale() {
        let mut e = reset_engine();
        let refr = new_refr(&mut e);
        let data = refr.at(TESObjectREFR::data);
        e.set(data, OBJ_REFR::pObjectReference, Ptr::new(0x7777));
        e.set(data, OBJ_REFR::AngleY, 3.0f32);
        e.set(data, OBJ_REFR::LocationZ, 9.0f32);
        e.set(refr, TESObjectREFR::fRefScale, 2.5f32);
        e.call_log = Some(vec![]);
        e.call(0x0055_ac90, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(data, OBJ_REFR::pObjectReference), Ptr::new(0x7777));
        assert_eq!(e.get(data, OBJ_REFR::AngleY), 0.0);
        assert_eq!(e.get(data, OBJ_REFR::LocationZ), 0.0);
        assert_eq!(e.get(refr, TESObjectREFR::fRefScale), 1.0);
        assert_eq!(calls_to(&log, MEMSET), vec![vec![data.addr(), 0, 0x1c]]);
        // no loading object: the leveled creature base is restored
        assert_eq!(calls_to(&log, 0x0057_c300), vec![vec![refr.addr()]]);
    }

    #[test]
    fn data_reset_resets_the_water_state_of_loaded_data() {
        let mut e = reset_engine();
        let refr = new_refr(&mut e);
        let loaded: Ptr<LOADED_REF_DATA> = e.new_object();
        e.set(
            loaded,
            LOADED_REF_DATA::pCurrentWaterObject,
            Ptr::new(0x4444),
        );
        e.set(loaded, LOADED_REF_DATA::fRelevantWaterHeight, 12.0f32);
        e.set(refr, TESObjectREFR::pLoadedData, loaded.cast());
        e.call(0x0055_ac90, &args![refr]);
        assert!(e
            .get(loaded, LOADED_REF_DATA::pCurrentWaterObject)
            .is_null());
        assert_eq!(
            e.get(loaded, LOADED_REF_DATA::fRelevantWaterHeight),
            -2000.0
        );
    }

    #[test]
    fn data_reset_keeps_the_leveled_base_while_loading() {
        let mut e = reset_engine();
        let refr = new_refr(&mut e);
        let loading_object = e.mem.alloc(8);
        e.set_global(GLOBAL_LOADING_OBJECT, loading_object);
        returns(&mut e, LOADING_FLAG, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_ac90, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0057_c300).is_empty());
        // a loading object that is not loading restores it
        returns(&mut e, LOADING_FLAG, 0);
        e.call_log = Some(vec![]);
        e.call(0x0055_ac90, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0057_c300).len(), 1);
    }

    #[test]
    fn point_pair_constructor_builds_both_points() {
        let mut e = engine();
        let data = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        let back = e.call(0x0055_a400, &args![data]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, data);
        assert_eq!(
            calls_to(&log, LIST_ITEM_SLOT),
            vec![vec![data + 4], vec![data + 0x10]]
        );
    }

    #[test]
    fn constructor_builds_a_blank_reference() {
        let mut e = reset_engine();
        stub(&mut e, &[0x0048_3370, 0x0053_3240, 0x0041_0360]);
        // 004f15a0: TESForm::cFormType = arg
        e.register(0x004f_15a0, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret::default()
        });
        let refr: Ptr<TESObjectREFR> = e.new_object();
        e.mem.set_u32(refr.addr() + 0x20, 0x99);
        e.mem.set_u32(refr.addr() + 0x40, 0x99);
        e.mem.set_u32(refr.addr() + 0x1c, 0x99);
        e.call_log = Some(vec![]);
        let back = e.call(0x0055_a2f0, &args![refr]).ptr::<TESObjectREFR>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, refr);
        assert_eq!(e.mem.u32(refr.addr()), REFR_VTABLE);
        assert_eq!(
            e.get(refr, TESObjectREFR::vfptrTESChildCell),
            CHILD_CELL_VTABLE
        );
        assert_eq!(e.mem.u8(refr.addr() + 4), 0x3a);
        assert_eq!(e.get(refr, TESObjectREFR::fRefScale), 1.0);
        assert!(e.get(refr, TESObjectREFR::pParentCell).is_null());
        assert!(e.get(refr, TESObjectREFR::pRandomSound).is_null());
        assert!(e.get(refr, TESObjectREFR::pLoadedData).is_null());
        assert!(e
            .get(refr.at(TESObjectREFR::data), OBJ_REFR::pObjectReference)
            .is_null());
        // form base first, then the child cell at +0x18, the extra list at +0x44
        assert_eq!(addresses(&log)[1..3], [0x0048_3370, 0x0053_3240]);
        assert_eq!(calls_to(&log, 0x0053_3240), vec![vec![refr.addr() + 0x18]]);
        assert_eq!(calls_to(&log, 0x0041_0360), vec![vec![refr.addr() + 0x44]]);
    }

    // ---- 0055a3b0 / 0055a3d0 --------------------------------------------

    #[test]
    fn animation_comes_from_the_extra_list() {
        let mut e = engine();
        returns(&mut e, 0x0041_8220, 0xabc);
        let refr = new_refr(&mut e);
        e.call_log = Some(vec![]);
        let animation = e.call(0x0055_a3b0, &args![refr]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(animation, 0xabc);
        assert_eq!(calls_to(&log, 0x0041_8220), vec![vec![refr.addr() + 0x44]]);
    }

    // ---- 0055abe0 .. 0055ac80 ----------------------------------------------

    #[test]
    fn small_helpers() {
        let mut e = engine();
        let refr = new_refr(&mut e);
        // iFormFlags bit 0x400000
        e.mem.set_u32(refr.addr() + 8, 0x40_0000);
        assert!(e.call(0x0055_abe0, &args![refr]).bool());
        e.mem.set_u32(refr.addr() + 8, 0xffbf_ffff);
        assert!(!e.call(0x0055_abe0, &args![refr]).bool());
        // the list at +0x94
        assert_eq!(e.call(0x0055_ac00, &args![0x1000u32]).u32(), 0x1094);
        // the player's +0x638 is cleared
        let player = e.mem.alloc(0x700);
        e.mem.set_u32(player + 0x638, 5);
        e.call(0x0055_ac20, &args![player]);
        assert_eq!(e.mem.u32(player + 0x638), 0);
        // the remembered reference
        e.set_global(GLOBAL_LAST_REFR, 0x4242u32);
        assert_eq!(e.call(0x0055_ac70, &args![]).u32(), 0x4242);
        e.call(0x0055_ac80, &args![]);
        assert_eq!(e.call(0x0055_ac70, &args![]).u32(), 0);
    }

    #[test]
    fn follower_removal_passes_the_address_of_the_item_slot() {
        let mut e = engine();
        let seen = std::rc::Rc::new(std::cell::Cell::new((0u32, 0u32, 0u32)));
        let seen2 = seen.clone();
        e.register_double(LIST_REMOVE, move |e, a| {
            seen2.set((a[0], a[1], e.mem.u32(a[1])));
            Ret::default()
        });
        let player = e.mem.alloc(0x900);
        e.call(0x0055_ac40, &args![player, 0x1234u32]);
        assert_eq!(seen.get(), (player + 0x84c, seen.get().1, 0x1234));
        // a null reference does nothing
        seen.set((0, 0, 0));
        e.call(0x0055_ac40, &args![player, 0u32]);
        assert_eq!(seen.get(), (0, 0, 0));
    }

    // ---- a reference with a working set of virtuals ------------------------

    /// Byte offset in a test reference that the `Get3D` double reads (the
    /// loaded 3D pointer a test wants the reference to report).
    const FAKE_3D: u32 = 0x60;

    /// A reference whose vtable has `Get3D` (reads `FAKE_3D`), `Set3D`,
    /// `IsActor` (false), `SetDelete`, `SetParentCell`, the name getter and
    /// the location getter (`this + 0x30`, like `00436aa0`), and whose
    /// child-cell base reports its `pParentCell`.
    fn standard_refr(e: &mut Engine) -> Ptr<TESObjectREFR> {
        e.register(0x00ff_0010, |e, a| e.mem.u32(a[0] + FAKE_3D).into_ret());
        stub(
            e,
            &[
                0x00ff_0011,
                0x00ff_0013,
                0x00ff_0014,
                0x00ff_0015,
                0x00ff_0019,
            ],
        );
        e.register(0x00ff_0012, |_, _| false.into_ret());
        e.register(0x00ff_0016, |_, a| (a[0] + 0x30).into_ret());
        e.register(0x00ff_0017, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        returns(e, 0x00ff_0018, 0x0102_f000);
        e.register(0x00ff_0030, |_, _| 0x4e4eu32.into_ret());
        refr_vtable(
            e,
            &[
                (0x1d0, 0x00ff_0010),
                (0x1cc, 0x00ff_0011),
                (0x100, 0x00ff_0012),
                (0xc4, 0x00ff_0013),
                (0x228, 0x00ff_0014),
                (0x134, 0x00ff_0015),
                (0x1f4, 0x00ff_0016),
                (0x130, 0x00ff_0018),
                (0xe0, 0x00ff_0019),
                (0x1c8, 0x00ff_0030),
            ],
            0x00ff_0017,
        );
        new_refr(e)
    }

    // ---- 0055a430: the destructor ---------------------------------------

    const DESTRUCTOR_CALLEES: &[u32] = &[
        0x0057_acc0,
        0x0056_8680,
        0x0045_38e0,
        0x0082_5c00,
        0x0096_e290,
        0x008d_8520,
        0x0082_f1f0,
        0x0086_8270,
        0x0054_9580,
        0x0057_5d70,
        0x0058_6170,
        0x006f_cf30,
        0x0045_43c0,
        0x0053_80d0,
        0x0066_b0d0,
        0x008a_81c0,
        0x008a_8150,
        0x0057_6760,
        0x008d_15b0,
        0x0086_23a0,
        0x0042_26e0,
        0x0056_a9f0,
        0x0041_dda0,
        0x0056_ac90,
        0x0056_aa10,
        0x0041_de00,
        0x0041_e0d0,
        0x0041_df90,
        0x0041_de40,
        0x0056_7790,
        0x0056_7c50,
        0x0041_e310,
        0x0041_e340,
        0x0041_f170,
        0x0041_f4c0,
        0x0041_f650,
        0x005a_a5d0,
        WATER_LIST_A_GETTER,
        WATER_LIST_B_GETTER,
        LIST_REMOVE,
        0x0056_4d80,
        0x0096_3d60,
        0x0097_6680,
        0x0096_f600,
        0x0097_5320,
        0x0097_8660,
        0x0057_25b0,
        0x0097_49b0,
        0x0045_2370,
        0x0047_7410,
        0x0056_9b80,
        0x0041_e600,
        0x0041_e500,
        0x0056_9ba0,
        0x0041_e780,
        0x0041_ef20,
        0x0041_eda0,
        0x0041_e960,
        0x0056_4930,
        GET_REF_PERSISTS,
        0x0057_5dc0,
        CELL_REMOVE_REFERENCE,
        0x0056_4bc0,
        0x006c_0720,
        0x006c_0c80,
        0x006c_1060,
        0x0070_5b30,
        0x0070_38d0,
        0x0070_38b0,
        0x0096_2cd0,
        0x0089_f4e0,
        0x0041_03b0,
        0x0048_3630,
        LIST_IS_EMPTY,
        // ClearData and what it calls
        0x0056_b0b0,
        0x0057_2a50,
        0x0057_b520,
        EXTRA_REMOVE_ALL,
        EXTRA_REMOVE_ALL_DEFAULT,
        EXTRA_ADD,
        EXTRA_SET_PERSISTENT_CELL,
    ];

    fn destructor_engine() -> Engine {
        let mut e = reset_engine();
        stub(&mut e, DESTRUCTOR_CALLEES);
        returns(&mut e, EXTRA_GET_PERSISTENT_CELL, 0);
        returns(&mut e, EXTRA_GET_DATA, 0);
        stub(&mut e, &[EXTRA_REMOVE]);
        e
    }

    #[test]
    fn destructor_resets_vtables_and_ends_with_the_base_destructors() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        e.mem.set_u32(refr.addr(), 0x1111);
        e.mem.set_u32(refr.addr() + 0x18, 0x2222);
        e.call_log = Some(vec![]);
        e.call(0x0055_a430, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(refr.addr()), REFR_VTABLE);
        assert_eq!(e.mem.u32(refr.addr() + 0x18), CHILD_CELL_VTABLE);
        let order = addresses(&log);
        // the extra list destructor, then TESForm's, are last
        assert_eq!(order[order.len() - 2..], [0x0041_03b0, 0x0048_3630]);
        assert_eq!(calls_to(&log, 0x0041_03b0), vec![vec![refr.addr() + 0x44]]);
        // SetDelete(true), the reference is removed from the garbage collector
        assert_eq!(calls_to(&log, 0x0056_4930), vec![vec![refr.addr(), 1]]);
        assert_eq!(calls_to(&log, 0x0086_8270), vec![vec![refr.addr()]]);
        // ClearData ran: RemoveAll(this + 0x44, 1) and Set3D(0, 1) through the vtable
        assert_eq!(
            calls_to(&log, EXTRA_REMOVE_ALL),
            vec![vec![refr.addr() + 0x44, 1]]
        );
        assert_eq!(calls_to(&log, 0x00ff_0011), vec![vec![refr.addr(), 0, 1]]);
        // the save/load object's flag was cleared and restored
        assert_eq!(calls_to(&log, 0x008a_8150).len(), 2);
        assert_eq!(calls_to(&log, 0x008a_8150)[0][1], 0);
    }

    #[test]
    fn destructor_asks_each_furniture_user_with_a_process_to_forget_it() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        returns(&mut e, 0x0056_8680, 1);
        // two list nodes: item then next
        let second = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        let actor_a = e.mem.alloc(8);
        let actor_b = e.mem.alloc(8);
        e.mem.set_u32(first, actor_a);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, actor_b);
        returns(&mut e, 0x0096_e290, first);
        // actor A has a process whose slot 0x4cc is a recorder; B has none
        e.register(0x00ff_0020, |_, _| Ret::default());
        let process = object_with_vtable(&mut e, 0x20, &[(0x4cc, 0x00ff_0020)]);
        e.register_double(0x008d_8520, move |_, a| Ret {
            eax: if a[0] == actor_a { process } else { 0 },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0055_a430, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x00ff_0020),
            vec![vec![process, refr.addr()]]
        );
        assert_eq!(
            calls_to(&log, 0x0045_38e0),
            vec![vec![OBJECT_FURNITURE_MANAGER, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x0082_f1f0),
            vec![vec![OBJECT_FURNITURE_MANAGER]]
        );
    }

    #[test]
    fn destructor_takes_the_reference_out_of_the_water_lists() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        returns(&mut e, WATER_LIST_A_GETTER, 0x0aaa);
        returns(&mut e, WATER_LIST_B_GETTER, 0x0bbb);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen2 = seen.clone();
        e.register_double(LIST_REMOVE, move |e, a| {
            seen2.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x0055_a430, &args![refr]);
        let me = refr.addr();
        assert_eq!(*seen.borrow(), vec![(0x0aaa, me), (0x0bbb, me)]);
    }

    #[test]
    fn destructor_clears_the_players_slots_that_point_at_it() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        let me = refr.addr();
        let player = e.mem.alloc(0x900);
        e.set_global(GLOBAL_PLAYER, player);
        e.mem.set_u32(player + 0x638, me);
        returns(&mut e, 0x0089_f4e0, me);
        returns(&mut e, 0x0070_38d0, me);
        e.set_global(GLOBAL_LAST_REFR, me);
        e.call_log = Some(vec![]);
        e.call(0x0055_a430, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(player + 0x638), 0);
        assert_eq!(e.global::<u32>(GLOBAL_LAST_REFR), 0);
        // the enemy actor is cleared and the player told
        assert_eq!(calls_to(&log, 0x0070_38b0), vec![vec![0]]);
        assert_eq!(calls_to(&log, 0x0096_2cd0), vec![vec![player, me]]);

        // a different reference in the slots leaves them alone
        let other = standard_refr(&mut e);
        e.mem.set_u32(player + 0x638, me);
        e.set_global(GLOBAL_LAST_REFR, me);
        e.call(0x0055_a430, &args![other]);
        assert_eq!(e.mem.u32(player + 0x638), me);
        assert_eq!(e.global::<u32>(GLOBAL_LAST_REFR), me);
    }

    #[test]
    fn destructor_skips_the_unlinking_for_a_flagged_form() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        // iFormFlags & 0x4000
        e.mem.set_u32(refr.addr() + 8, 0x4000);
        e.call_log = Some(vec![]);
        e.call(0x0055_a430, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0057_6760).is_empty());
        assert!(calls_to(&log, 0x0096_3d60).is_empty());
        // but the cells and the base destructors still run
        assert_eq!(calls_to(&log, 0x0048_3630).len(), 1);
    }

    #[test]
    fn destructor_removes_the_reference_from_its_parent_and_persistent_cells() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        let cell = e.mem.alloc(0x10);
        let persistent_cell = e.mem.alloc(0x10);
        e.set(refr, TESObjectREFR::pParentCell, Ptr::new(cell));
        returns(&mut e, GET_REF_PERSISTS, 1);
        let handler = e.mem.alloc(8);
        e.set_global(GLOBAL_DATA_HANDLER, handler);
        returns(&mut e, 0x0057_5dc0, persistent_cell);
        e.call_log = Some(vec![]);
        e.call(0x0055_a430, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CELL_REMOVE_REFERENCE),
            vec![vec![cell, refr.addr()], vec![persistent_cell, refr.addr()]]
        );
        assert_eq!(calls_to(&log, 0x0057_5dc0), vec![vec![refr.addr() + 0x18]]);
    }

    #[test]
    fn destructor_hands_loaded_data_back_to_the_physics_world() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        let loaded: Ptr<LOADED_REF_DATA> = e.new_object();
        e.set(loaded, LOADED_REF_DATA::spPhantom, Ptr::new(0x6666));
        e.set(refr, TESObjectREFR::pLoadedData, loaded.cast());
        let cell = e.mem.alloc(0x50);
        e.set(refr, TESObjectREFR::pParentCell, Ptr::new(cell));
        returns(&mut e, 0x0045_43c0, 0x0cc0);
        e.call_log = Some(vec![]);
        e.call(0x0055_a430, &args![refr]);
        let log = e.call_log.take().unwrap();
        // the phantom handle goes to the owner the cell reports, then is released
        assert_eq!(calls_to(&log, 0x0053_80d0), vec![vec![0x0cc0, 0x6666]]);
        assert_eq!(
            calls_to(&log, 0x0066_b0d0),
            vec![vec![loaded.addr() + 0x18, 0]]
        );
    }

    #[test]
    fn destructor_without_a_cell_does_not_look_for_a_world_owner() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        let loaded: Ptr<LOADED_REF_DATA> = e.new_object();
        e.set(refr, TESObjectREFR::pLoadedData, loaded.cast());
        e.call_log = Some(vec![]);
        e.call(0x0055_a430, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0053_80d0).is_empty());
        assert_eq!(calls_to(&log, 0x0066_b0d0).len(), 1);
    }

    #[test]
    fn destructor_unlinks_extra_data_cross_references() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        let me = refr.addr();
        // the enable-state parent keeps this reference as a child
        let parent = e.mem.alloc(0x80);
        returns(&mut e, 0x0056_a9f0, parent);
        // a dropped item whose owner is this reference
        let dropped = e.mem.alloc(0x80);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, dropped);
        returns(&mut e, 0x0041_df90, node);
        returns(&mut e, 0x0056_7790, me);
        // an item dropper
        let dropper = e.mem.alloc(0x80);
        returns(&mut e, 0x0041_de00, dropper);
        let handler = e.mem.alloc(8);
        e.set_global(GLOBAL_DATA_HANDLER, handler);
        e.call_log = Some(vec![]);
        e.call(0x0055_a430, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0041_dda0), vec![vec![parent + 0x44, me]]);
        assert_eq!(calls_to(&log, 0x0041_e0d0), vec![vec![dropper + 0x44, me]]);
        assert_eq!(calls_to(&log, 0x0041_de40), vec![vec![dropped + 0x44, 0]]);
        assert_eq!(calls_to(&log, 0x0056_7c50), vec![vec![dropped]]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_the_object_only_when_asked() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x0055_a3d0, &args![refr, 0u32])
            .ptr::<TESObjectREFR>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, refr);
        assert!(calls_to(&log, 0x0040_1030).is_empty());
        assert_eq!(calls_to(&log, 0x0048_3630).len(), 1);

        let heap = e.mem.alloc(0x68);
        e.mem.set_u32(heap, REFR_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x0055_a3d0, &args![heap, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0040_1030), vec![vec![heap]]);
        // freed by the platform allocator
        assert_eq!(e.mem.block_size(heap), None);
    }

    // ---- 0055ad20: ClearData ---------------------------------------------

    #[test]
    fn clear_data_wipes_extras_and_drops_the_3d() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        let me = refr.addr();
        e.call_log = Some(vec![]);
        e.call(0x0055_ad20, &args![refr]);
        let log = e.call_log.take().unwrap();
        // not loading (no loading object): the leveled base is restored,
        // the light killed, the water list left
        assert_eq!(calls_to(&log, 0x0056_b0b0), vec![vec![me]]);
        assert_eq!(calls_to(&log, 0x0057_2a50), vec![vec![me, 0]]);
        assert_eq!(calls_to(&log, 0x0057_b520), vec![vec![me, 0]]);
        assert_eq!(calls_to(&log, EXTRA_REMOVE_ALL), vec![vec![me + 0x44, 1]]);
        assert!(calls_to(&log, EXTRA_REMOVE_ALL_DEFAULT).is_empty());
        assert_eq!(calls_to(&log, 0x00ff_0011), vec![vec![me, 0, 1]]);
        // no 3D: the 0x5b extra is not looked up
        assert!(calls_to(&log, EXTRA_GET_DATA).is_empty());
    }

    #[test]
    fn clear_data_keeps_the_3d_extra_and_the_persistent_cell_across_the_wipe() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        let me = refr.addr();
        e.mem.set_u32(me + FAKE_3D, 0x3333);
        returns(&mut e, EXTRA_GET_PERSISTENT_CELL, 0x0c11);
        returns(&mut e, EXTRA_GET_DATA, 0x0e22);
        e.call_log = Some(vec![]);
        e.call(0x0055_ad20, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, EXTRA_GET_DATA), vec![vec![me + 0x44, 0x5b]]);
        assert_eq!(
            calls_to(&log, EXTRA_REMOVE),
            vec![vec![me + 0x44, 0x0e22, 0], vec![me + 0x44, 0x0e22, 1]]
        );
        assert_eq!(calls_to(&log, EXTRA_ADD), vec![vec![me + 0x44, 0x0e22]]);
        assert_eq!(
            calls_to(&log, EXTRA_SET_PERSISTENT_CELL),
            vec![vec![me + 0x44, 0x0c11]]
        );
        // the wipe is between removing and adding back
        let order = addresses(&log);
        let at = |a: u32| order.iter().position(|x| *x == a).unwrap();
        assert!(at(EXTRA_REMOVE) < at(EXTRA_REMOVE_ALL));
        assert!(at(EXTRA_REMOVE_ALL) < at(EXTRA_ADD));
    }

    #[test]
    fn clear_data_while_loading_keeps_defaults_and_the_3d() {
        let mut e = destructor_engine();
        let refr = standard_refr(&mut e);
        let me = refr.addr();
        let loading_object = e.mem.alloc(8);
        e.set_global(GLOBAL_LOADING_OBJECT, loading_object);
        returns(&mut e, LOADING_FLAG, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_ad20, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0056_b0b0).is_empty());
        assert_eq!(
            calls_to(&log, EXTRA_REMOVE_ALL_DEFAULT),
            vec![vec![me + 0x44, 1]]
        );
        assert!(calls_to(&log, EXTRA_REMOVE_ALL).is_empty());
        assert!(calls_to(&log, 0x00ff_0011).is_empty());
    }

    // ---- 0055ae70: Save ----------------------------------------------------

    #[test]
    fn save_of_a_deleted_form_only_opens_and_closes_it() {
        let mut e = engine();
        stub(&mut e, &[0x0048_55a0, 0x0048_5680]);
        let refr = standard_refr(&mut e);
        e.mem.set_u32(refr.addr() + 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0055_ae70, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log)[1..],
            [0x0048_55a0, FORM_IS_DELETED, 0x0048_5680]
        );
    }

    fn save_engine() -> (Engine, Ptr<TESObjectREFR>) {
        let mut e = engine();
        stub(
            &mut e,
            &[
                0x0048_55a0,
                0x0048_5680,
                ADD_CHUNK_VALUE,
                0x0041_2970,
                0x0048_56d0,
                0x0069_2710,
                0x0069_3dc0,
                ADD_CHUNK_DATA,
                ENDIAN_SWAP_ON_SAVE,
                0x0057_2d30,
            ],
        );
        let refr = standard_refr(&mut e);
        let base = form(&mut e, 0x28, 0x0001_2345, 0);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(base),
        );
        e.set(refr, TESObjectREFR::fRefScale, 1.0f32);
        (e, refr)
    }

    #[test]
    fn save_writes_the_base_object_and_the_extras() {
        let (mut e, refr) = save_engine();
        e.call_log = Some(vec![]);
        e.call(0x0055_ae70, &args![refr]);
        let log = e.call_log.take().unwrap();
        // NAME with the base object's form ID; no XSCL for a unit scale, no ONAM
        assert_eq!(
            calls_to(&log, ADD_CHUNK_VALUE),
            vec![vec![tag(b"NAME"), 0x0001_2345]]
        );
        assert_eq!(calls_to(&log, 0x0041_2970), vec![vec![refr.addr() + 0x44]]);
        assert!(calls_to(&log, 0x0048_56d0).is_empty());
        let order = addresses(&log);
        assert_eq!(order[1], 0x0048_55a0);
        assert_eq!(*order.last().unwrap(), 0x0048_5680);
    }

    #[test]
    fn save_writes_scale_and_ownership_chunks_when_set() {
        let (mut e, refr) = save_engine();
        e.set(refr, TESObjectREFR::fRefScale, 1.5f32);
        returns(&mut e, 0x0057_2d30, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_ae70, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ADD_CHUNK_VALUE),
            vec![
                vec![tag(b"NAME"), 0x0001_2345],
                vec![tag(b"XSCL"), 1.5f32.to_bits()]
            ]
        );
        assert_eq!(calls_to(&log, 0x0057_2d30), vec![vec![refr.addr(), 8]]);
        assert_eq!(calls_to(&log, 0x0048_56d0), vec![vec![tag(b"ONAM")]]);
    }

    #[test]
    fn save_writes_position_then_rotation_in_the_data_chunk() {
        let (mut e, refr) = save_engine();
        let me = refr.addr();
        for i in 0..3u32 {
            e.mem.set_f32(me + 0x30 + 4 * i, 100.0 + i as f32);
            e.mem.set_f32(me + 0x24 + 4 * i, 1.0 + i as f32);
        }
        let written = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let written2 = written.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            written2.borrow_mut().push((a[0], bytes));
            Ret::default()
        });
        e.call(0x0055_ae70, &args![refr]);
        let data = written.borrow();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].0, tag(b"DATA"));
        let floats: Vec<f32> = data[0]
            .1
            .chunks(4)
            .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
            .collect();
        assert_eq!(floats, vec![100.0, 101.0, 102.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    fn save_swaps_the_data_chunk_around_the_write_when_the_build_does() {
        let (mut e, refr) = save_engine();
        returns(&mut e, ENDIAN_SWAP_ON_SAVE, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_ae70, &args![refr]);
        let log = e.call_log.take().unwrap();
        let order = addresses(&log);
        let write = order.iter().position(|a| *a == ADD_CHUNK_DATA).unwrap();
        let swaps: Vec<usize> = order
            .iter()
            .enumerate()
            .filter(|(_, a)| **a == 0x0069_3dc0)
            .map(|(i, _)| i)
            .collect();
        assert_eq!(swaps.len(), 2);
        assert!(swaps[0] < write && write < swaps[1]);
    }

    // ---- 0055af90: Load ----------------------------------------------------

    type Shared<T> = std::rc::Rc<std::cell::RefCell<T>>;

    /// An engine whose `TESFile` doubles serve `chunks`: each entry is a
    /// chunk tag and the payload bytes `GetChunkData` copies out for it.
    /// `forms` maps a form ID to the object `LookupFormByID` returns.
    fn load_engine(
        chunks: Vec<(u32, Vec<u8>)>,
        forms: Vec<(u32, u32)>,
    ) -> (Engine, Ptr<TESObjectREFR>, u32) {
        let mut e = engine();
        stub(
            &mut e,
            &[
                0x0069_2710,
                0x0048_4ab0,
                0x0048_5110,
                0x0048_5d50,
                0x0069_3dc0,
                0x0041_44a0,
                MESSAGE,
                0x0057_2d50,
                0x0040_1680,
                0x0041_fe90,
                EXTRA_REMOVE_ALL,
                0x0056_b0b0,
                EXTRA_SET_PERSISTENT_CELL,
            ],
        );
        returns(&mut e, EXTRA_GET_PERSISTENT_CELL, 0);
        // identity cast, except a null stays null
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        let queue: Shared<std::collections::VecDeque<(u32, Vec<u8>)>> =
            std::rc::Rc::new(std::cell::RefCell::new(chunks.into_iter().collect()));
        let current: Shared<Vec<u8>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let q = queue.clone();
        let c = current.clone();
        e.register_double(0x0047_26b0, move |_, _| match q.borrow_mut().pop_front() {
            Some((tag, payload)) => {
                *c.borrow_mut() = payload;
                Ret {
                    eax: tag,
                    ..Ret::default()
                }
            }
            None => Ret::default(),
        });
        let q = queue.clone();
        e.register_double(0x0047_26f0, move |_, _| (!q.borrow().is_empty()).into_ret());
        let c = current.clone();
        e.register_double(0x0047_27f0, move |e, a| {
            e.mem.write(a[1], &c.borrow());
            Ret::default()
        });
        let c = current.clone();
        e.register_double(0x0047_2890, move |e, a| {
            let payload = c.borrow();
            let n = payload.len().min(a[2] as usize);
            e.mem.write(a[1], &payload[..n]);
            Ret::default()
        });
        e.register_double(0x0048_39c0, move |_, a| {
            forms
                .iter()
                .find(|(id, _)| *id == a[0])
                .map_or(0, |(_, form)| *form)
                .into_ret()
        });
        let chunk_size = e.mem.alloc(8);
        let _ = chunk_size;
        returns(&mut e, 0x0040_1660, 0x40);
        let refr = standard_refr(&mut e);
        e.mem.set_u32(refr.addr() + 0xc, 0x00aa_0001);
        let file = e.mem.alloc(0x20);
        (e, refr, file)
    }

    fn le(words: &[u32]) -> Vec<u8> {
        words.iter().flat_map(|w| w.to_le_bytes()).collect()
    }

    #[test]
    fn load_reads_base_object_scale_and_placement() {
        let base = 0x4000_0000u32;
        let mut payload = Vec::new();
        for f in [10.0f32, 20.0, 30.0, 0.5, 1.5, 2.5] {
            payload.extend_from_slice(&f.to_le_bytes());
        }
        let (mut e, refr, file) = load_engine(
            vec![
                (tag(b"NAME"), le(&[0x0001_0001])),
                (tag(b"XSCL"), 2.0f32.to_le_bytes().to_vec()),
                (tag(b"DATA"), payload),
            ],
            vec![(0x0001_0001, base)],
        );
        e.call_log = Some(vec![]);
        let loaded = e.call(0x0055_af90, &args![refr, file]).bool();
        let log = e.call_log.take().unwrap();
        assert!(loaded);
        let data = refr.at(TESObjectREFR::data);
        assert_eq!(e.get(data, OBJ_REFR::pObjectReference), Ptr::new(base));
        assert_eq!(e.get(refr, TESObjectREFR::fRefScale), 2.0);
        // the DATA chunk is position first, then rotation
        assert_eq!(e.get(data, OBJ_REFR::LocationX), 10.0);
        assert_eq!(e.get(data, OBJ_REFR::LocationZ), 30.0);
        assert_eq!(e.get(data, OBJ_REFR::AngleX), 0.5);
        assert_eq!(e.get(data, OBJ_REFR::AngleZ), 2.5);
        // LoadForm(this, file), then the chunk reader's compile index fix-up
        assert_eq!(calls_to(&log, 0x0048_5110), vec![vec![refr.addr(), file]]);
        assert_eq!(calls_to(&log, 0x0048_5d50).len(), 1);
        // SetDelete(false) before the placement is read
        assert_eq!(calls_to(&log, 0x00ff_0013), vec![vec![refr.addr(), 0]]);
        assert_eq!(
            calls_to(&log, 0x0047_2890),
            vec![vec![file, calls_to(&log, 0x0047_2890)[0][1], 0x18]]
        );
    }

    #[test]
    fn load_with_a_missing_base_object_logs_it_and_skips_the_placement() {
        let mut payload = Vec::new();
        for f in [10.0f32, 20.0, 30.0, 0.5, 1.5, 2.5] {
            payload.extend_from_slice(&f.to_le_bytes());
        }
        let (mut e, refr, file) = load_engine(
            vec![(tag(b"NAME"), le(&[0x00de_ad00])), (tag(b"DATA"), payload)],
            vec![],
        );
        e.call_log = Some(vec![]);
        let loaded = e.call(0x0055_af90, &args![refr, file]).bool();
        let log = e.call_log.take().unwrap();
        assert!(!loaded);
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![MISSING_BASE_FORMAT, 0x00de_ad00, 0x00aa_0001]]
        );
        let data = refr.at(TESObjectREFR::data);
        assert_eq!(e.get(data, OBJ_REFR::LocationX), 0.0);
        assert!(calls_to(&log, 0x0047_2890).is_empty());
    }

    #[test]
    fn load_keeps_a_known_base_while_loading_and_replaces_an_unknown_one() {
        for known in [true, false] {
            let (mut e, refr, file) = load_engine(
                vec![(tag(b"NAME"), le(&[0x0001_0002]))],
                vec![(0x0001_0002, 0x4000_0100)],
            );
            let data = refr.at(TESObjectREFR::data);
            let old_base = form(&mut e, 0x28, 0x0001_0001, 0);
            e.set(data, OBJ_REFR::pObjectReference, Ptr::new(old_base));
            let loading_object = e.mem.alloc(8);
            e.set_global(GLOBAL_LOADING_OBJECT, loading_object);
            let handler = e.mem.alloc(8);
            e.set_global(GLOBAL_DATA_HANDLER, handler);
            returns(&mut e, LOADING_FLAG, 1);
            // 00469860(handler, id): is the old base already known?
            returns(&mut e, 0x0046_9860, known as u32);
            e.call_log = Some(vec![]);
            e.call(0x0055_af90, &args![refr, file]);
            let log = e.call_log.take().unwrap();
            let expected = if known { old_base } else { 0x4000_0100 };
            assert_eq!(e.get(data, OBJ_REFR::pObjectReference), Ptr::new(expected));
            assert_eq!(
                calls_to(&log, 0x0046_9860),
                vec![vec![handler, 0x0001_0001]]
            );
        }
    }

    #[test]
    fn load_hands_every_extra_data_chunk_kind_to_the_extra_list() {
        for kind in EXTRA_DATA_CHUNKS {
            let (mut e, refr, file) = load_engine(vec![(kind, vec![])], vec![]);
            e.call_log = Some(vec![]);
            e.call(0x0055_af90, &args![refr, file]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, 0x0041_44a0),
                vec![vec![refr.addr() + 0x44, file, refr.addr()]],
                "chunk {kind:08x}"
            );
            assert!(calls_to(&log, MESSAGE).is_empty());
        }
    }

    #[test]
    fn load_reports_an_unknown_chunk_and_skips_rclr() {
        let (mut e, refr, file) =
            load_engine(vec![(tag(b"ZZZZ"), vec![]), (tag(b"RCLR"), vec![])], vec![]);
        e.call_log = Some(vec![]);
        e.call(0x0055_af90, &args![refr, file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MESSAGE), vec![vec![LOAD_TERMINATED_MESSAGE]]);
        assert!(calls_to(&log, 0x0041_44a0).is_empty());
    }

    #[test]
    fn load_passes_the_editor_id_and_bounds_to_the_virtuals() {
        let (mut e, refr, file) = load_engine(
            vec![
                (tag(b"EDID"), b"MyRefr\0".to_vec()),
                (tag(b"OBND"), vec![]),
                (tag(b"ONAM"), vec![]),
            ],
            vec![],
        );
        e.call_log = Some(vec![]);
        e.call(0x0055_af90, &args![refr, file]);
        let log = e.call_log.take().unwrap();
        // the editor ID is read into a stack buffer of the chunk's size (0x40 here)
        let read = calls_to(&log, 0x0047_2890);
        assert_eq!(read.len(), 1);
        assert_eq!(read[0][0], file);
        assert_eq!(read[0][2], 0x200);
        // virtual +0x134 got that buffer; +0xe0 got the file; ONAM sets flag 8
        assert_eq!(
            calls_to(&log, 0x00ff_0015),
            vec![vec![refr.addr(), read[0][1]]]
        );
        assert_eq!(calls_to(&log, 0x00ff_0019), vec![vec![refr.addr(), file]]);
        assert_eq!(calls_to(&log, 0x0057_2d50), vec![vec![refr.addr(), 8]]);
    }

    #[test]
    fn load_of_a_deleted_form_drops_its_extras_but_keeps_the_persistent_cell() {
        let (mut e, refr, file) = load_engine(vec![(tag(b"NAME"), le(&[1]))], vec![(1, 0x4000)]);
        e.mem.set_u32(refr.addr() + 8, 0x20);
        returns(&mut e, EXTRA_GET_PERSISTENT_CELL, 0x0c0c);
        let data = refr.at(TESObjectREFR::data);
        e.set(data, OBJ_REFR::pObjectReference, Ptr::new(0x4000_0010));
        e.call_log = Some(vec![]);
        let loaded = e.call(0x0055_af90, &args![refr, file]).bool();
        let log = e.call_log.take().unwrap();
        assert!(loaded);
        assert_eq!(calls_to(&log, 0x0056_b0b0), vec![vec![refr.addr()]]);
        assert_eq!(
            calls_to(&log, EXTRA_REMOVE_ALL),
            vec![vec![refr.addr() + 0x44, 1]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_SET_PERSISTENT_CELL),
            vec![vec![refr.addr() + 0x44, 0x0c0c]]
        );
        // the chunks are not read at all
        assert!(calls_to(&log, 0x0047_26b0).is_empty());
    }

    #[test]
    fn load_reports_a_patrol_script_that_has_text_but_is_not_compiled() {
        let (mut e, refr, file) = load_engine(vec![], vec![]);
        returns(&mut e, 0x0041_fe90, 0x0111);
        let script = e.mem.alloc(0x40);
        e.mem.set_u32(script + 0x2c, 0x0222);
        e.mem.set_u8(script + 0x2a, 0);
        let info = e.mem.alloc(0x10);
        returns(&mut e, 0x008d_0430, script);
        returns(&mut e, 0x0050_0940, info);
        stub(&mut e, &[STRING_CONSTRUCT, STRING_FORMAT, STRING_DESTRUCT]);
        e.register(READ_FIRST_DWORD, |_, _| 0x0777u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0055_af90, &args![refr, file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MESSAGE), vec![vec![0x0777]]);
        let format = calls_to(&log, STRING_FORMAT);
        assert_eq!(format.len(), 1);
        assert_eq!(format[0][1], PATROL_NOT_COMPILED_FORMAT);
        // a compiled script, or one without text, stays quiet
        e.mem.set_u8(script + 0x2a, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_af90, &args![refr, file]);
        assert!(calls_to(&e.call_log.take().unwrap(), MESSAGE).is_empty());
        e.mem.set_u8(script + 0x2a, 0);
        e.mem.set_u32(script + 0x2c, 0);
        e.call_log = Some(vec![]);
        e.call(0x0055_af90, &args![refr, file]);
        assert!(calls_to(&e.call_log.take().unwrap(), MESSAGE).is_empty());
    }

    #[test]
    fn script_holder_accessors_read_their_fields() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x2c, 0x1234);
        e.mem.set_u8(object + 0x2a, 1);
        assert_eq!(e.call(0x0055_b980, &args![object]).u32(), 0x1234);
        assert!(e.call(0x0055_b9a0, &args![object]).bool());
        e.mem.set_u8(object + 0x2a, 0);
        assert!(!e.call(0x0055_b9a0, &args![object]).bool());
    }

    // ---- 0055b9c0: the references map pass ---------------------------------

    /// An engine whose `BGSSaveLoadReferencesMap::Lookup` (`00853130`)
    /// maps the old references in `pairs` to new ones.
    fn remap_engine(pairs: Vec<(u32, u32)>) -> (Engine, Ptr<TESObjectREFR>, u32, u32) {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register_double(0x0085_3130, move |e, a| {
            // (map, old, &new)
            match pairs.iter().find(|(old, _)| *old == a[1]) {
                Some((_, new)) => {
                    e.mem.set_u32(a[2], *new);
                    true.into_ret()
                }
                None => false.into_ret(),
            }
        });
        stub(
            &mut e,
            &[
                0x0056_a9f0,
                0x0056_aa10,
                0x0056_aa70,
                0x0056_aa90,
                0x0041_dcd0,
                0x0056_9b20,
                0x0056_9b40,
                0x0056_9b80,
                0x0056_9ba0,
                0x0041_e530,
                0x0042_0bc0,
                0x0042_0a60,
                0x0042_0130,
                0x0042_0160,
                0x0041_e780,
                0x0041_e7f0,
                0x0041_ea30,
                0x0041_eba0,
                0x0041_eb60,
                0x0041_edd0,
                0x0041_f050,
                0x0056_a470,
                0x0056_a990,
                0x0056_a9b0,
                0x0056_8e50,
                LIST_IS_EMPTY,
            ],
        );
        let refr = standard_refr(&mut e);
        let map = e.mem.alloc(0x20);
        let target = e.mem.alloc(0x80);
        (e, refr, map, target)
    }

    #[test]
    fn references_pass_needs_a_map_and_a_mapping_for_the_reference() {
        let (mut e, refr, map, _) = remap_engine(vec![]);
        e.call_log = Some(vec![]);
        e.call(0x0055_b9c0, &args![refr, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // a map that does not know the reference: one lookup, nothing else
        e.call_log = Some(vec![]);
        e.call(0x0055_b9c0, &args![refr, map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(addresses(&log), vec![0x0055_b9c0, 0x0085_3130]);
        assert_eq!(calls_to(&log, 0x0085_3130)[0][..2], [map, refr.addr()]);
    }

    #[test]
    fn references_pass_moves_the_enable_parent_to_the_duplicates_parent() {
        let (mut e, refr, map, target) = remap_engine(vec![]);
        let me = refr.addr();
        let old_parent = e.mem.alloc(0x80);
        let new_parent = e.mem.alloc(0x80);
        // rebuild the engine's lookup with the real addresses
        e.register_double(0x0085_3130, move |e, a| {
            let new = if a[1] == me {
                target
            } else if a[1] == old_parent {
                new_parent
            } else {
                return false.into_ret();
            };
            e.mem.set_u32(a[2], new);
            true.into_ret()
        });
        returns(&mut e, 0x0056_a9f0, old_parent);
        returns(&mut e, 0x0056_aa70, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_b9c0, &args![refr, map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0056_aa10), vec![vec![target, new_parent]]);
        assert_eq!(calls_to(&log, 0x0056_aa90), vec![vec![target, 1]]);
        assert_eq!(
            calls_to(&log, 0x0041_dcd0),
            vec![vec![new_parent + 0x44, target]]
        );
    }

    #[test]
    fn references_pass_keeps_unmapped_slot_references_and_replaces_mapped_ones() {
        let (mut e, refr, map, target) = remap_engine(vec![]);
        let me = refr.addr();
        let (unmapped, mapped_old, mapped_new) = (0x0a00u32, 0x0b00u32, e.mem.alloc(0x80));
        e.register_double(0x0085_3130, move |e, a| {
            let new = if a[1] == me {
                target
            } else if a[1] == mapped_old {
                mapped_new
            } else {
                return false.into_ret();
            };
            e.mem.set_u32(a[2], new);
            true.into_ret()
        });
        // slot 0 holds an unmapped reference, slot 1 a mapped one
        e.register(0x0042_0bc0, |_, a| {
            let slot = a[1];
            [0x0a00u32, 0x0b00][slot as usize].into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0055_b9c0, &args![refr, map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0042_0a60),
            vec![
                vec![target + 0x44, 0, unmapped],
                vec![target + 0x44, 1, mapped_new]
            ]
        );
    }

    #[test]
    fn references_pass_remaps_the_four_slot_references_when_they_are_mapped() {
        let (mut e, refr, map, target) = remap_engine(vec![]);
        let me = refr.addr();
        let new_ref = e.mem.alloc(0x80);
        e.register_double(0x0085_3130, move |e, a| {
            let new = match a[1] {
                x if x == me => target,
                0x0c00 => new_ref,
                _ => return false.into_ret(),
            };
            e.mem.set_u32(a[2], new);
            true.into_ret()
        });
        e.register(0x0042_0130, |_, a| {
            // only slot 2 has a reference
            if a[1] == 2 { 0x0c00u32 } else { 0 }.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0055_b9c0, &args![refr, map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0042_0160),
            vec![vec![target + 0x44, new_ref, 2]]
        );
    }

    #[test]
    fn references_pass_rewires_activate_children_and_decals() {
        let (mut e, refr, map, target) = remap_engine(vec![]);
        let me = refr.addr();
        let child_new = e.mem.alloc(0x80);
        let decal_new = e.mem.alloc(0x80);
        e.register_double(0x0085_3130, move |e, a| {
            let new = match a[1] {
                x if x == me => target,
                0x0d00 => child_new,
                0x0e00 => decal_new,
                _ => return false.into_ret(),
            };
            e.mem.set_u32(a[2], new);
            true.into_ret()
        });
        // activate children: one node whose item points at {old child, 0.75}
        let child_entry = e.mem.alloc(8);
        e.mem.set_u32(child_entry, 0x0d00);
        e.mem.set_f32(child_entry + 4, 0.75);
        let child_node = e.mem.alloc(8);
        e.mem.set_u32(child_node, child_entry);
        returns(&mut e, 0x0041_e780, child_node);
        returns(&mut e, 0x0041_eb60, 3);
        // decals: {old ref, position at +4, rotation at +0x10}
        let decal_entry = e.mem.alloc(0x20);
        e.mem.set_u32(decal_entry, 0x0e00);
        let decal_node = e.mem.alloc(8);
        e.mem.set_u32(decal_node, decal_entry);
        returns(&mut e, 0x0041_f050, decal_node);
        e.call_log = Some(vec![]);
        e.call(0x0055_b9c0, &args![refr, map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0041_e7f0),
            vec![vec![target + 0x44, child_new]]
        );
        assert_eq!(
            calls_to(&log, 0x0041_ea30),
            vec![vec![target + 0x44, child_new, 0.75f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, 0x0041_edd0),
            vec![vec![child_new + 0x44, target]]
        );
        assert_eq!(calls_to(&log, 0x0041_eba0), vec![vec![target + 0x44, 3]]);
        assert_eq!(
            calls_to(&log, 0x0056_a470),
            vec![vec![target, decal_new, decal_entry + 4, decal_entry + 0x10]]
        );
    }

    #[test]
    fn references_pass_places_the_duplicated_door_link() {
        let (mut e, refr, map, target) = remap_engine(vec![]);
        let me = refr.addr();
        // the linked door (old) with a location, its replacement with another
        let old_door = standard_refr(&mut e);
        let new_door = standard_refr(&mut e);
        for (i, v) in [100.0f32, 200.0, 300.0].iter().enumerate() {
            e.mem.set_f32(old_door.addr() + 0x30 + 4 * i as u32, *v);
        }
        for (i, v) in [1000.0f32, 2000.0, 3000.0].iter().enumerate() {
            e.mem.set_f32(new_door.addr() + 0x30 + 4 * i as u32, *v);
        }
        let (old_door_addr, new_door_addr) = (old_door.addr(), new_door.addr());
        e.register_double(0x0085_3130, move |e, a| {
            let new = if a[1] == me {
                target
            } else if a[1] == old_door_addr {
                new_door_addr
            } else {
                return false.into_ret();
            };
            e.mem.set_u32(a[2], new);
            true.into_ret()
        });
        // this reference's teleport data: {door, point (+4), ..., +0x10 point}
        let teleport = e.mem.alloc(0x20);
        e.mem.set_u32(teleport, old_door_addr);
        for (i, v) in [110.0f32, 220.0, 330.0].iter().enumerate() {
            e.mem.set_f32(teleport + 4 + 4 * i as u32, *v);
        }
        for (i, v) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.mem.set_f32(teleport + 0x10 + 4 * i as u32, *v);
        }
        returns(&mut e, 0x0056_8e50, teleport);
        let new_teleport = e.mem.alloc(0x20);
        returns(&mut e, 0x0056_8e70, new_teleport);
        stub(&mut e, &[0x0053_7e90, 0x006f_4ed0, 0x0043_a280]);
        e.register(0x0046_0140, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0071_7e50, |_, a| (a[0] + 4).into_ret());
        // NiPoint3 minus and plus (result is the first stack argument)
        let point =
            |e: &Engine, p: u32| -> [f32; 3] { [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)] };
        e.register_double(0x0043_9ef0, move |e, a| {
            let (this, result, other) = (point(e, a[0]), a[1], point(e, a[2]));
            for i in 0..3 {
                e.mem.set_f32(result + 4 * i as u32, this[i] - other[i]);
            }
            result.into_ret()
        });
        e.register_double(0x0043_9e90, move |e, a| {
            let (this, result, other) = (point(e, a[0]), a[1], point(e, a[2]));
            for i in 0..3 {
                e.mem.set_f32(result + 4 * i as u32, this[i] + other[i]);
            }
            result.into_ret()
        });
        let moved = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let copied = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let (moved2, copied2) = (moved.clone(), copied.clone());
        e.register_double(0x006f_4ed0, move |e, a| {
            moved2.borrow_mut().push((a[0], point(e, a[1])));
            Ret::default()
        });
        e.register_double(0x0043_a280, move |e, a| {
            copied2.borrow_mut().push((a[0], point(e, a[1])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0055_b9c0, &args![refr, map]);
        let log = e.call_log.take().unwrap();
        // the new teleport is created on the duplicate and linked to the new door
        assert_eq!(calls_to(&log, 0x0056_8e70), vec![vec![target]]);
        assert_eq!(
            calls_to(&log, 0x0053_7e90),
            vec![vec![new_teleport, new_door_addr]]
        );
        // new door location + (teleport point - old door location)
        assert_eq!(
            *moved.borrow(),
            vec![(new_teleport, [1010.0, 2020.0, 3030.0])]
        );
        assert_eq!(*copied.borrow(), vec![(new_teleport, [7.0, 8.0, 9.0])]);
    }

    // ---- 0055c050: CreateDuplicateForm --------------------------------------

    #[test]
    fn create_duplicate_sets_extras_aside_and_puts_them_back() {
        let mut e = destructor_engine();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        let refr = standard_refr(&mut e);
        let me = refr.addr();
        let parent = e.mem.alloc(0x10);
        e.set(refr, TESObjectREFR::pParentCell, Ptr::new(parent));
        // extras of types 0x2b, 0x51 and 0x63 exist
        e.register(EXTRA_GET_DATA, |_, a| {
            match a[1] {
                0x2b => 0x2b00u32,
                0x51 => 0x5100,
                0x63 => 0x6300,
                _ => 0,
            }
            .into_ret()
        });
        stub(&mut e, &[EXTRA_REMOVE, EXTRA_ADD, 0x0056_5480]);
        let duplicate = standard_refr(&mut e);
        returns(&mut e, 0x0048_67a0, duplicate.addr());
        let map = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x0055_c050, &args![refr, 0u32, map])
            .ptr::<TESObjectREFR>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, duplicate);
        let list = me + 0x44;
        // the 0x51 extra is fetched (and removed) twice, for two classes
        assert_eq!(
            calls_to(&log, EXTRA_REMOVE),
            vec![
                vec![list, 0x2b00, 0],
                vec![list, 0x5100, 0],
                vec![list, 0x5100, 0],
                vec![list, 0x6300, 0]
            ]
        );
        assert_eq!(
            calls_to(&log, EXTRA_ADD),
            vec![
                vec![list, 0x2b00],
                vec![list, 0x5100],
                vec![list, 0x5100],
                vec![list, 0x6300]
            ]
        );
        // parent cell cleared for the duplication, then restored
        assert_eq!(
            calls_to(&log, 0x00ff_0014),
            vec![vec![me, 0], vec![me, parent]]
        );
        assert_eq!(calls_to(&log, 0x0048_67a0), vec![vec![me, 0, map]]);
        // the copy loses its 3D and its persistence
        assert_eq!(
            calls_to(&log, 0x00ff_0011),
            vec![vec![duplicate.addr(), 0, 1]]
        );
        assert_eq!(calls_to(&log, 0x0056_5480), vec![vec![duplicate.addr(), 0]]);
    }

    #[test]
    fn create_duplicate_without_extras_leaves_the_lists_alone() {
        let mut e = destructor_engine();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        let refr = standard_refr(&mut e);
        stub(&mut e, &[EXTRA_REMOVE, EXTRA_ADD, 0x0056_5480]);
        let duplicate = standard_refr(&mut e);
        returns(&mut e, 0x0048_67a0, duplicate.addr());
        e.call_log = Some(vec![]);
        e.call(0x0055_c050, &args![refr, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, EXTRA_REMOVE).is_empty());
        assert!(calls_to(&log, EXTRA_ADD).is_empty());
        // eight lookups, one per kind
        assert_eq!(calls_to(&log, EXTRA_GET_DATA).len(), 8);
    }

    // ---- 0055c3e0: Copy ----------------------------------------------------

    const COPY_CALLEES: &[u32] = &[
        0x0040_30b0,
        0x0041_1ec0,
        0x0041_8250,
        0x0041_ae70,
        0x0041_dcd0,
        0x0041_dda0,
        0x0041_e530,
        0x0041_e600,
        0x0041_e780,
        0x0041_edd0,
        0x0041_ef20,
        0x0041_f170,
        0x0041_f4c0,
        0x0041_f650,
        0x0041_f810,
        0x0041_f840,
        0x0043_d410,
        0x0043_fa80,
        0x0044_0460,
        0x0044_0490,
        0x0045_0b80,
        0x0045_11e0,
        0x0045_1ef0,
        0x0045_6520,
        0x0047_aec0,
        0x0048_51b0,
        0x0049_c680,
        0x004e_8030,
        0x004e_d8c0,
        0x0050_d810,
        0x0053_7e90,
        0x0054_8230,
        0x0056_5480,
        0x0056_7490,
        0x0056_7770,
        0x0056_7c50,
        0x0056_8e50,
        0x0056_8e70,
        0x0056_8f30,
        0x0056_9140,
        0x0056_9580,
        0x0056_9b80,
        0x0056_9be0,
        0x0056_9cb0,
        0x0056_9df0,
        0x0056_a1e0,
        0x0056_a9f0,
        0x0056_fa00,
        0x0057_2d30,
        0x0057_ada0,
        0x0057_c730,
        0x0070_ec90,
        0x008d_8520,
        0x0096_11e0,
        0x00a5_9c60,
        0x00a5_9d30,
        0x00a5_a040,
        0x00b5_eed0,
        0x00c6_bd00,
        LIST_IS_EMPTY,
        CELL_REMOVE_REFERENCE,
        GET_REF_PERSISTS,
        MESSAGE,
    ];

    /// `this` and the source reference of a `Copy`, with every callee stubbed
    /// and the few accessors the logic depends on given real bodies: the
    /// scale getter (`float` at +0x3c), `memcpy`, the stack-count getter and
    /// a radius getter (equal for both).
    fn copy_engine() -> (Engine, Ptr<TESObjectREFR>, Ptr<TESObjectREFR>) {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        stub(&mut e, COPY_CALLEES);
        // 00598040 / 00567400: the scale; 00568cb0: a radius
        e.register(0x0059_8040, |e, a| e.mem.f32(a[0] + 0x3c).into_ret());
        e.register(0x0056_7400, |e, a| e.mem.f32(a[0] + 0x3c).into_ret());
        e.register(0x0056_8cb0, |_, _| 5.0f32.into_ret());
        returns(&mut e, 0x0040_eb10, 1);
        e.register(0x0040_1460, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        returns(&mut e, 0x0041_8770, 0);
        let this = standard_refr(&mut e);
        let other = standard_refr(&mut e);
        e.set(this, TESObjectREFR::fRefScale, 1.0f32);
        e.set(other, TESObjectREFR::fRefScale, 1.0f32);
        (e, this, other)
    }

    fn set3d_calls(log: &[(u32, Vec<u32>)], refr: Ptr<TESObjectREFR>) -> Vec<Vec<u32>> {
        calls_to(log, 0x00ff_0011)
            .into_iter()
            .filter(|c| c[0] == refr.addr())
            .collect()
    }

    #[test]
    fn copy_ignores_a_source_that_is_not_a_reference() {
        let (mut e, this, _) = copy_engine();
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, 0x1234u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn copy_takes_over_the_data_persistence_cell_and_scale() {
        let (mut e, this, other) = copy_engine();
        let (me, them) = (this.addr(), other.addr());
        let cell = e.mem.alloc(0x40);
        e.set(other, TESObjectREFR::pParentCell, Ptr::new(cell));
        e.set(other, TESObjectREFR::fRefScale, 1.0f32);
        let their_data = other.at(TESObjectREFR::data);
        e.set(their_data, OBJ_REFR::LocationX, 42.0f32);
        e.set(their_data, OBJ_REFR::AngleY, 0.25f32);
        returns(&mut e, GET_REF_PERSISTS, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        // persistence, then the form components
        assert_eq!(calls_to(&log, 0x0056_5480), vec![vec![me, 1]]);
        assert_eq!(calls_to(&log, 0x0048_51b0), vec![vec![me, them]]);
        // 0x1c bytes of OBJ_REFR data
        assert_eq!(
            calls_to(&log, 0x0040_1460),
            vec![vec![me + 0x20, them + 0x20, 0x1c]]
        );
        assert_eq!(
            e.get(this.at(TESObjectREFR::data), OBJ_REFR::LocationX),
            42.0
        );
        assert_eq!(e.get(this.at(TESObjectREFR::data), OBJ_REFR::AngleY), 0.25);
        // the source's cell takes the reference; with no 3D the I/O manager is kicked
        assert_eq!(calls_to(&log, 0x0054_8230), vec![vec![cell, me, 0]]);
        e.set_global(GLOBAL_IO_MANAGER, 0x0ab0u32);
        assert_eq!(calls_to(&log, 0x0045_6520).len(), 1);
        // the scale is set from the source's
        assert_eq!(
            calls_to(&log, 0x0056_7490),
            vec![vec![me, 1.0f32.to_bits()]]
        );
        // nothing changed that needs a new 3D, and the source has none: Set3D(0, 1) once
        assert_eq!(set3d_calls(&log, this), vec![vec![me, 0, 1]]);
    }

    #[test]
    fn copy_removes_the_reference_from_its_old_cell_when_the_source_has_none() {
        let (mut e, this, other) = copy_engine();
        let old = e.mem.alloc(0x40);
        e.set(this, TESObjectREFR::pParentCell, Ptr::new(old));
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CELL_REMOVE_REFERENCE),
            vec![vec![old, this.addr()]]
        );
        assert!(calls_to(&log, 0x0054_8230).is_empty());
    }

    #[test]
    fn copy_rebuilds_the_3d_when_the_copy_differs() {
        // each change marks the reference dirty: a second Set3D(0, 1)
        type Change = Box<dyn Fn(&mut Engine, Ptr<TESObjectREFR>, Ptr<TESObjectREFR>)>;
        let cases: Vec<(&str, Change)> = vec![
            (
                "other base object",
                Box::new(|e, this, _| {
                    let base = form(e, 0x28, 1, 0);
                    e.set(
                        this.at(TESObjectREFR::data),
                        OBJ_REFR::pObjectReference,
                        Ptr::new(base),
                    );
                }),
            ),
            (
                "other scale on a non-actor",
                Box::new(|e, _, other| {
                    e.set(other, TESObjectREFR::fRefScale, 2.0f32);
                }),
            ),
            (
                "other ammo count",
                Box::new(|e, this, _| {
                    let base = form(e, 0x29, 1, 0);
                    e.set(
                        this.at(TESObjectREFR::data),
                        OBJ_REFR::pObjectReference,
                        Ptr::new(base),
                    );
                    // the stack counts differ
                    e.register(0x0041_8770, |_, a| (a[0] & 0xff).into_ret());
                }),
            ),
            (
                "other radius",
                Box::new(|e, _, _| {
                    returns(e, 0x0040_eb10, 0);
                }),
            ),
            (
                "other 00569580 value",
                Box::new(|e, _, _| {
                    e.register(0x0056_9580, |_, a| a[0].into_ret());
                }),
            ),
        ];
        for (name, change) in cases {
            let (mut e, this, other) = copy_engine();
            change(&mut e, this, other);
            e.call_log = Some(vec![]);
            e.call(0x0055_c3e0, &args![this, other]);
            let log = e.call_log.take().unwrap();
            assert_eq!(set3d_calls(&log, this).len(), 2, "{name}");
        }
    }

    #[test]
    fn copy_ignores_a_different_scale_for_an_actor() {
        let (mut e, this, other) = copy_engine();
        e.set(other, TESObjectREFR::fRefScale, 2.0f32);
        e.register(0x00ff_0012, |_, _| true.into_ret()); // IsActor, for both
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(set3d_calls(&log, this).len(), 1);
        assert_eq!(
            calls_to(&log, 0x0056_7490),
            vec![vec![this.addr(), 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn copy_gives_the_left_over_flag_to_set3d_with_the_base_node() {
        let (mut e, this, other) = copy_engine();
        let me = this.addr();
        e.mem.set_u32(other.addr() + FAKE_3D, 0x3d01); // the source has a 3D
        returns(&mut e, 0x0040_77c0, 1);
        let base = form(&mut e, 0x25, 1, 0);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(base, vtable);
        e.mem.set_u32(vtable + 0x14c, 0x00ff_0040);
        returns(&mut e, 0x00ff_0040, 0x3e3e);
        for refr in [this, other] {
            e.set(
                refr.at(TESObjectREFR::data),
                OBJ_REFR::pObjectReference,
                Ptr::new(base),
            );
        }
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        // 0055cdb9..0055cdf5: PUSH 1; PUSH 0; PUSH this; the base's +0x14c pops (this, 0);
        // the node pushed next and the 1 left on the stack are Set3D's two words.
        assert_eq!(calls_to(&log, 0x00ff_0040), vec![vec![base, me, 0]]);
        assert!(set3d_calls(&log, this).contains(&vec![me, 0x3e3e, 1]));
    }

    #[test]
    fn copy_keeps_the_existing_3d_in_step_with_the_new_position() {
        let (mut e, this, other) = copy_engine();
        let (me, them) = (this.addr(), other.addr());
        e.mem.set_u32(me + FAKE_3D, 0x3d00); // this has a 3D
        e.mem.set_u32(them + FAKE_3D, 0x3d01); // so does the source
        returns(&mut e, 0x0056_fa00, 0x0fa0);
        let cell = e.mem.alloc(0x40);
        e.set(other, TESObjectREFR::pParentCell, Ptr::new(cell));
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        // the node is moved to the reference's location and given its matrix
        assert_eq!(calls_to(&log, 0x0044_0460)[0], vec![0x3d00, me + 0x30]);
        assert_eq!(calls_to(&log, 0x0043_fa80)[0], vec![0x3d00, 0x0fa0]);
        // the simulation reset with a zero velocity and no I/O manager kick
        assert_eq!(calls_to(&log, 0x00c6_bd00)[0], vec![0x3d00, 1]);
        assert_eq!(
            calls_to(&log, 0x0043_d410)[0][1..],
            [0.0f32.to_bits(), 0, 0]
        );
        assert!(calls_to(&log, 0x0045_6520).is_empty());
        // with a 3D, no cell kick: the cell's AddReference still ran
        assert_eq!(calls_to(&log, 0x0054_8230), vec![vec![cell, me, 0]]);
        // finally the properties of the 3D are refreshed
        assert_eq!(calls_to(&log, 0x00a5_a040), vec![vec![0x3d00]]);
        assert_eq!(set3d_calls(&log, this).len(), 0);
    }

    #[test]
    fn copy_builds_a_new_3d_for_a_copy_whose_source_has_one() {
        let (mut e, this, other) = copy_engine();
        let (me, them) = (this.addr(), other.addr());
        e.mem.set_u32(them + FAKE_3D, 0x3d01);
        returns(&mut e, 0x0056_fa00, 0x0fa0);
        // the source's 3D has a collision object with a virtual at +0xdc
        e.register(0x00ff_0031, |_, _| Ret::default());
        let collision = object_with_vtable(&mut e, 0x20, &[(0xdc, 0x00ff_0031)]);
        returns(&mut e, 0x0096_11e0, collision);
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0044_0460), vec![vec![0x4e4e, me + 0x30]]);
        assert_eq!(calls_to(&log, 0x0043_fa80), vec![vec![0x4e4e, 0x0fa0]]);
        assert_eq!(
            calls_to(&log, 0x0044_0490),
            vec![vec![0x4e4e, 1.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, 0x00ff_0031),
            vec![vec![collision, 0x4e4e, 1]]
        );
    }

    #[test]
    fn copy_regenerates_the_dynamic_light_of_a_light_base() {
        let (mut e, this, other) = copy_engine();
        let (me, them) = (this.addr(), other.addr());
        e.mem.set_u32(them + FAKE_3D, 0x3d01);
        e.mem.set_u32(me + FAKE_3D, 0x3d00);
        let light = form(&mut e, 0x1e, 0x1111, 0);
        e.set(
            this.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(light),
        );
        e.set(
            other.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(light),
        );
        returns(&mut e, 0x0057_2d30, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0050_d810),
            vec![vec![light, me, 0x3d00, 0]]
        );
        // the open flag was set from the extra data
        assert_eq!(calls_to(&log, 0x0047_aec0), vec![vec![me, 1, 1]]);
    }

    #[test]
    fn copy_recreates_the_teleport_data_when_the_door_differs() {
        let (mut e, this, other) = copy_engine();
        let (me, them) = (this.addr(), other.addr());
        let mine = e.mem.alloc(0x20);
        let theirs = e.mem.alloc(0x20);
        e.mem.set_u32(mine, 0xd001); // this teleport's door
        e.mem.set_u32(theirs, 0xd002); // the source's
        e.register_double(0x0056_8e50, move |_, a| {
            if a[0] == me { mine } else { theirs }.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        // the old link is dropped (the old door told) and the teleport reset
        assert_eq!(calls_to(&log, 0x0056_8f30)[0], vec![0xd001]);
        assert_eq!(calls_to(&log, 0x0053_7e90)[0], vec![mine, 0]);
        let _ = them;
    }

    #[test]
    fn copy_keeps_the_teleport_data_when_the_door_is_the_same() {
        let (mut e, this, other) = copy_engine();
        let me = this.addr();
        let mine = e.mem.alloc(0x20);
        let theirs = e.mem.alloc(0x20);
        e.mem.set_u32(mine, 0xd001);
        e.mem.set_u32(theirs, 0xd001);
        e.register_double(0x0056_8e50, move |_, a| {
            if a[0] == me { mine } else { theirs }.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0053_7e90)
            .iter()
            .all(|c| c[1] != 0 || c[0] != mine));
    }

    #[test]
    fn copy_ends_by_linking_the_enable_parent() {
        let (mut e, this, other) = copy_engine();
        let me = this.addr();
        let parent = e.mem.alloc(0x80);
        returns(&mut e, 0x0056_a9f0, parent);
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        // first the old parent forgets the reference, at the end the new link is made
        assert_eq!(calls_to(&log, 0x0041_dda0), vec![vec![parent + 0x44, me]]);
        assert_eq!(calls_to(&log, 0x0041_dcd0), vec![vec![parent + 0x44, me]]);
        let order = addresses(&log);
        let forget = order.iter().position(|a| *a == 0x0041_dda0).unwrap();
        let link = order.iter().position(|a| *a == 0x0041_dcd0).unwrap();
        assert!(forget < link);
    }

    #[test]
    fn copy_clears_conflicting_shared_door_data_and_logs_it() {
        let (mut e, this, other) = copy_engine();
        let me = this.addr();
        let door = standard_refr(&mut e);
        let teleport = e.mem.alloc(0x20);
        e.mem.set_u32(teleport, door.addr());
        returns(&mut e, 0x0056_8e50, teleport);
        // the source's data is different, the door has its own teleport
        let door_teleport = e.mem.alloc(0x20);
        e.register_double(0x0056_8e50, move |_, a| {
            if a[0] == door.addr() {
                door_teleport
            } else {
                teleport
            }
            .into_ret()
        });
        // both have a lock-like record (0569140) and a 00567770 record
        returns(&mut e, 0x0056_9140, 1);
        returns(&mut e, 0x0056_7770, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        // the teleport on the door is linked back to this reference
        assert_eq!(
            calls_to(&log, 0x0053_7e90).last().unwrap(),
            &vec![door_teleport, me]
        );
        assert_eq!(calls_to(&log, 0x0041_ae70), vec![vec![door.addr() + 0x44]]);
        assert_eq!(calls_to(&log, 0x0056_7c50), vec![vec![door.addr()]]);
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![SHARED_DATA_REMOVED_MESSAGE]]
        );
        // a door that is not persistent becomes persistent
        assert_eq!(
            calls_to(&log, 0x0056_5480).last().unwrap(),
            &vec![door.addr(), 1]
        );
    }

    #[test]
    fn copy_moves_water_lights_to_the_copy_and_off_the_source() {
        let (mut e, this, other) = copy_engine();
        let (me, them) = (this.addr(), other.addr());
        // a form flag on the source makes it walk the extra lists
        e.mem.set_u32(them + 8, 0x4000);
        let scene = e.mem.alloc(8);
        e.set_global(GLOBAL_TEMP_REFR_MANAGER, scene);
        returns(&mut e, 0x0070_ec90, 0x5c5c);
        let owner = standard_refr(&mut e);
        e.mem.set_u32(owner.addr() + FAKE_3D, 0x3d77);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, owner.addr());
        returns(&mut e, 0x0041_f810, node);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, 0x11ee);
        returns(&mut e, 0x0041_8250, holder);
        returns(&mut e, 0x004e_8030, 0x7a7a);
        returns(&mut e, 0x0040_30b0, 0x8b8b);
        returns(&mut e, 0x008d_8520, 0xd);
        let lights = e.mem.alloc(0x200);
        returns(&mut e, 0x00a5_9d30, lights);
        returns(&mut e, 0x0045_0b80, 0x9c9c);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        for addr in [0x0057_c730u32, 0x004e_d8c0, 0x0049_c680] {
            let s = seen.clone();
            e.register_double(addr, move |e, a| {
                s.borrow_mut().push((addr, a[0], e.mem.u32(a[1])));
                Ret::default()
            });
        }
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        // the first pass (list of this) adds the light; the second (list of the
        // source) removes it from the scene after checking and marking
        assert_eq!(
            *seen.borrow(),
            vec![
                (0x0057_c730, lights + 0x128, 0x11ee),
                (0x0049_c680, lights + 0x128, 0x11ee),
                (0x004e_d8c0, lights + 0x128, 0x11ee),
            ]
        );
        assert_eq!(e.mem.u8(lights + 0x83), 1);
        assert_eq!(calls_to(&log, 0x0045_0b80), vec![vec![0]]);
        assert_eq!(calls_to(&log, 0x00b5_eed0), vec![vec![0x9c9c, 0x11ee]]);
        assert_eq!(
            calls_to(&log, 0x0041_f840),
            vec![
                vec![owner.addr() + 0x44, me, 0],
                vec![owner.addr() + 0x44, me, 1]
            ]
        );
        // the scene node came from the scene manager's object and the owner
        assert_eq!(calls_to(&log, 0x004e_8030)[0], vec![0x5c5c, owner.addr()]);
    }

    #[test]
    fn copy_relinks_the_extra_data_references_of_both_lists() {
        let (mut e, this, other) = copy_engine();
        let (me, them) = (this.addr(), other.addr());
        e.mem.set_u32(them + 8, 0x4000);
        // the link lists: this one entry, the source one entry flagged 1 | 2
        let x1 = e.mem.alloc(0x80);
        let x2 = e.mem.alloc(0x80);
        let entry1 = e.mem.alloc(8);
        e.mem.set_u32(entry1, x1);
        let node1 = e.mem.alloc(8);
        e.mem.set_u32(node1, entry1);
        let entry2 = e.mem.alloc(8);
        e.mem.set_u32(entry2, x2);
        e.mem.set_u32(entry2 + 4, 3);
        let node2 = e.mem.alloc(8);
        e.mem.set_u32(node2, entry2);
        e.register_double(0x0041_f170, move |_, a| {
            (if a[0] == me + 0x44 { node1 } else { node2 }).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        // this list: both directions cleared for the entry's reference; the source's list: set
        assert_eq!(
            calls_to(&log, 0x0041_f4c0),
            vec![vec![x1 + 0x44, me, 0], vec![x2 + 0x44, me, 1]]
        );
        assert_eq!(
            calls_to(&log, 0x0041_f650),
            vec![vec![x1 + 0x44, me, 0], vec![x2 + 0x44, me, 1]]
        );
        // an entry without the flags is not set
        e.mem.set_u32(entry2 + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0055_c3e0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0041_f4c0), vec![vec![x1 + 0x44, me, 0]]);
    }

    // ---- 0055d230: Compare ---------------------------------------------------

    fn compare_engine() -> (Engine, Ptr<TESObjectREFR>, Ptr<TESObjectREFR>) {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        stub(&mut e, &[0x0048_5270, 0x0041_27e0]);
        e.register(MEMCMP, |e, a| {
            let (x, y) = (e.mem.bytes(a[0], a[2]), e.mem.bytes(a[1], a[2]));
            (x.cmp(&y) as i32).into_ret()
        });
        e.register(0x0059_8040, |e, a| e.mem.f32(a[0] + 0x3c).into_ret());
        let this = standard_refr(&mut e);
        let other = standard_refr(&mut e);
        e.set(this, TESObjectREFR::fRefScale, 1.0f32);
        e.set(other, TESObjectREFR::fRefScale, 1.0f32);
        (e, this, other)
    }

    #[test]
    fn compare_finds_references_that_match() {
        let (mut e, this, other) = compare_engine();
        assert!(!e.call(0x0055_d230, &args![this, other]).bool());
    }

    #[test]
    fn compare_finds_every_kind_of_difference() {
        // not a reference
        let (mut e, this, _) = compare_engine();
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0055_d230, &args![this, 0x1234u32]).bool());

        // the form components differ
        let (mut e, this, other) = compare_engine();
        returns(&mut e, 0x0048_5270, 1);
        assert!(e.call(0x0055_d230, &args![this, other]).bool());

        // the OBJ_REFR data differs (one byte of the rotation)
        let (mut e, this, other) = compare_engine();
        e.mem.set_u8(other.addr() + 0x2a, 9);
        assert!(e.call(0x0055_d230, &args![this, other]).bool());

        // the parent cells differ
        let (mut e, this, other) = compare_engine();
        e.set(other, TESObjectREFR::pParentCell, Ptr::new(0x4000));
        assert!(e.call(0x0055_d230, &args![this, other]).bool());

        // the extra lists differ
        let (mut e, this, other) = compare_engine();
        returns(&mut e, 0x0041_27e0, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0055_d230, &args![this, other]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0041_27e0),
            vec![vec![this.addr() + 0x44, other.addr() + 0x44]]
        );

        // the scales differ, or are NaN
        let (mut e, this, other) = compare_engine();
        e.set(other, TESObjectREFR::fRefScale, 1.5f32);
        assert!(e.call(0x0055_d230, &args![this, other]).bool());
        let (mut e, this, other) = compare_engine();
        e.set(this, TESObjectREFR::fRefScale, f32::NAN);
        e.set(other, TESObjectREFR::fRefScale, f32::NAN);
        assert!(e.call(0x0055_d230, &args![this, other]).bool());
    }

    // ---- 0055d310: HasContainer ------------------------------------------------

    #[test]
    fn has_container_depends_on_the_kind_of_base_object() {
        let mut e = engine();
        let refr = new_refr(&mut e);
        let data = refr.at(TESObjectREFR::data);
        // no base object
        assert_eq!(e.call(0x0055_d310, &args![refr]).u32(), 0);
        for (kind, expected_offset) in [
            (0x1bu8, Some(0x30u32)),
            (0x29, None),
            (0x2a, Some(0x64)),
            (0x2b, Some(0x64)),
            (0x2c, None),
            (0x1a, None),
        ] {
            let base = form(&mut e, kind, 1, 0);
            e.set(data, OBJ_REFR::pObjectReference, Ptr::new(base));
            let found = e.call(0x0055_d310, &args![refr]).u32();
            assert_eq!(
                found,
                expected_offset.map_or(0, |o| base + o),
                "kind {kind:#x}"
            );
        }
    }

    // ---- 0055d3b0 ------------------------------------------------------------------

    #[test]
    fn base_check_accepts_the_base_or_the_two_remembered_bases() {
        let mut e = engine();
        let refr = new_refr(&mut e);
        let data = refr.at(TESObjectREFR::data);
        e.set(data, OBJ_REFR::pObjectReference, Ptr::new(0x1001));
        returns(&mut e, 0x0042_16f0, 0x1002);
        returns(&mut e, 0x0042_1720, 0x1003);
        for (form, expected) in [
            (0x1001u32, true),
            (0x1002, true),
            (0x1003, true),
            (0x1004, false),
        ] {
            assert_eq!(e.call(0x0055_d3b0, &args![refr, form]).bool(), expected);
        }
    }

    // ---- 0055d420 --------------------------------------------------------------------

    #[test]
    fn altered_flag_reaches_the_parent_cell_only_for_a_set_flag() {
        let mut e = engine();
        stub(&mut e, &[0x0048_4730]);
        e.register(0x00ff_0040, |_, _| Ret::default());
        let cell = object_with_vtable(&mut e, 0x20, &[(0xc8, 0x00ff_0040)]);
        let refr = standard_refr(&mut e);
        e.set(refr, TESObjectREFR::pParentCell, Ptr::new(cell));
        e.call_log = Some(vec![]);
        e.call(0x0055_d420, &args![refr, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00ff_0040), vec![vec![cell, 1]]);
        assert_eq!(calls_to(&log, 0x0048_4730), vec![vec![refr.addr(), 1]]);

        // flag clear: only TESForm's handler
        e.call_log = Some(vec![]);
        e.call(0x0055_d420, &args![refr, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x00ff_0040).is_empty());
        assert_eq!(calls_to(&log, 0x0048_4730), vec![vec![refr.addr(), 0]]);

        // a flagged form (iFormFlags & 0x4000) skips the cell
        e.mem.set_u32(refr.addr() + 8, 0x4000);
        e.call_log = Some(vec![]);
        e.call(0x0055_d420, &args![refr, 1u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x00ff_0040).is_empty());
    }

    // ---- 0055d480 --------------------------------------------------------------------

    fn name_engine() -> (Engine, Ptr<TESObjectREFR>) {
        let mut e = engine();
        stub(&mut e, &[0x0047_4cb0, 0x0042_26e0]);
        returns(&mut e, 0x0040_1280, 0x0aa1);
        returns(&mut e, 0x0042_16f0, 0);
        let refr = standard_refr(&mut e);
        (e, refr)
    }

    #[test]
    fn reference_name_prefers_the_special_case_then_the_base_objects_name() {
        let (mut e, refr) = name_engine();
        // 00474cb0 true: the special string
        returns(&mut e, 0x0047_4cb0, 1);
        assert_eq!(e.call(0x0055_d480, &args![refr]).u32(), 0x0aa1);
        returns(&mut e, 0x0047_4cb0, 0);
        // no base object: the empty string
        assert_eq!(e.call(0x0055_d480, &args![refr]).u32(), EMPTY_STRING);
        // a base object with a name
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"Sunny");
        e.register_double(0x00ff_0041, move |_, _| name.into_ret());
        let base = object_with_vtable(&mut e, 0x40, &[(0x130, 0x00ff_0041)]);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(base),
        );
        assert_eq!(e.call(0x0055_d480, &args![refr]).u32(), name);
        // the data handler says no names: empty again
        returns(&mut e, 0x0042_26e0, 1);
        let handler = e.mem.alloc(8);
        e.set_global(GLOBAL_DATA_HANDLER, handler);
        assert_eq!(e.call(0x0055_d480, &args![refr]).u32(), EMPTY_STRING);
    }

    #[test]
    fn reference_name_falls_back_to_the_leveled_creatures_original_base() {
        let (mut e, refr) = name_engine();
        let empty = e.mem.alloc(8);
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"Gecko");
        e.register_double(0x00ff_0041, move |_, _| empty.into_ret());
        e.register_double(0x00ff_0042, move |_, _| name.into_ret());
        let base = object_with_vtable(&mut e, 0x40, &[(0x130, 0x00ff_0041)]);
        let original = object_with_vtable(&mut e, 0x40, &[(0x130, 0x00ff_0042)]);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(base),
        );
        returns(&mut e, 0x0042_16f0, original);
        // the base's own name is empty: the original base's name
        assert_eq!(e.call(0x0055_d480, &args![refr]).u32(), name);
        // no original base either: the (empty) name stays
        returns(&mut e, 0x0042_16f0, 0);
        assert_eq!(e.call(0x0055_d480, &args![refr]).u32(), empty);
    }

    // ---- 0055d520 --------------------------------------------------------------------

    #[test]
    fn full_name_is_the_base_objects() {
        let mut e = engine();
        returns(&mut e, 0x0048_2720, 0x0f11);
        let refr = new_refr(&mut e);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(0x5555),
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_d520, &args![refr]).u32(), 0x0f11);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0048_2720), vec![vec![0x5555]]);
    }

    // ---- 0055d540: GetFormDetailedString -------------------------------------------

    #[test]
    fn detailed_string_names_the_base_world_and_cell() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                SPRINTF,
                STRING_CONSTRUCT,
                STRING_FORMAT,
                STRING_DESTRUCT,
                0x0057_5d70,
            ],
        );
        returns(&mut e, 0x0044_0e30, 0x0b01);
        returns(&mut e, 0x0040_1280, 0x0b02);
        e.register(READ_FIRST_DWORD, |_, _| 0x0b03u32.into_ret());
        let refr = standard_refr(&mut e);
        // base object and parent cell with a name slot (0x130) and a string slot (0x90)
        e.register(0x00ff_0050, |_, _| 0x0c01u32.into_ret());
        e.register(0x00ff_0051, |_, _| Ret::default());
        let base = object_with_vtable(&mut e, 0x40, &[(0x130, 0x00ff_0050)]);
        e.mem.set_u32(base + 0xc, 0x0001_00aa);
        let cell = object_with_vtable(&mut e, 0x40, &[(0x90, 0x00ff_0051)]);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(base),
        );
        e.set(refr, TESObjectREFR::pParentCell, Ptr::new(cell));
        e.mem.set_u32(refr.addr() + 0xc, 0x0002_00bb);
        let out = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0055_d540, &args![refr, out]);
        let log = e.call_log.take().unwrap();
        // the base fragment, formatted with the kind, the name and the ID
        let fragments = calls_to(&log, SPRINTF);
        assert_eq!(fragments.len(), 1);
        assert_eq!(
            fragments[0][1..],
            [DETAILED_BASE_FORMAT, 0x0b01, 0x0c01, 0x0001_00aa]
        );
        let first_buffer = fragments[0][0];
        // the cell is asked for its string into the temporary BSStringT
        let cell_call = calls_to(&log, 0x00ff_0051);
        assert_eq!(cell_call.len(), 1);
        assert_eq!(cell_call[0][0], cell);
        // the final text
        assert_eq!(
            calls_to(&log, STRING_FORMAT)
                .into_iter()
                .filter(|c| c[0] == out)
                .collect::<Vec<_>>(),
            vec![vec![
                out,
                DETAILED_FORMAT,
                0x0b01,
                0x0b02,
                0x0002_00bb,
                first_buffer,
                0x0b03
            ]]
        );
        assert_eq!(calls_to(&log, STRING_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&log, STRING_DESTRUCT).len(), 1);
    }

    #[test]
    fn detailed_string_adds_the_world_space_fragment_when_there_is_one() {
        let mut e = engine();
        stub(
            &mut e,
            &[SPRINTF, STRING_CONSTRUCT, STRING_FORMAT, STRING_DESTRUCT],
        );
        returns(&mut e, 0x0044_0e30, 1);
        returns(&mut e, 0x0040_1280, 2);
        e.register(READ_FIRST_DWORD, |_, _| 3u32.into_ret());
        let refr = standard_refr(&mut e);
        e.register(0x00ff_0050, |_, _| 0x0c01u32.into_ret());
        let world = object_with_vtable(&mut e, 0x40, &[(0x130, 0x00ff_0050)]);
        e.mem.set_u32(world + 0xc, 0x0003_00cc);
        returns(&mut e, 0x0057_5d70, world);
        let out = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0055_d540, &args![refr, out]);
        let log = e.call_log.take().unwrap();
        let fragments = calls_to(&log, SPRINTF);
        assert_eq!(fragments.len(), 1);
        assert_eq!(
            fragments[0][1..],
            [DETAILED_WORLD_FORMAT, 0x0c01, 0x0003_00cc]
        );
    }

    // ---- 0055d6d0 --------------------------------------------------------------------

    #[test]
    fn lod_flag_follows_the_references_test() {
        let mut e = engine();
        stub(&mut e, &[0x0070_5b10, 0x0054_b800, 0x0047_6ab0]);
        returns(&mut e, 0x0045_c6b0, 0x0100);
        let refr = new_refr(&mut e);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(0x5555),
        );
        let target = 0x0222u32;
        e.register_double(0x00ff_0060, move |_, _| target.into_ret());
        let object = object_with_vtable(&mut e, 0x20, &[(0x10, 0x00ff_0060)]);

        // the test holds, the target's flag is not 9: set it and run 0054b800
        returns(&mut e, 0x0056_5090, 1);
        returns(&mut e, 0x0070_58c0, 3);
        e.call_log = Some(vec![]);
        e.call(0x0055_d6d0, &args![refr, object]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0070_5b10), vec![vec![target, 9]]);
        assert_eq!(calls_to(&log, 0x0054_b800), vec![vec![target]]);

        // the test fails and the flag is 9: it gets the base's LOD multiplier
        returns(&mut e, 0x0056_5090, 0);
        returns(&mut e, 0x0070_58c0, 9);
        e.call_log = Some(vec![]);
        e.call(0x0055_d6d0, &args![refr, object]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0045_c6b0), vec![vec![0x5555]]);
        assert_eq!(calls_to(&log, 0x0070_5b10), vec![vec![target, 0x0100]]);
        assert_eq!(calls_to(&log, 0x0047_6ab0), vec![vec![target]]);

        // they agree: nothing is changed
        returns(&mut e, 0x0056_5090, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_d6d0, &args![refr, object]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x0070_5b10).is_empty());

        // no object, or an object without a target: nothing
        e.call_log = Some(vec![]);
        e.call(0x0055_d6d0, &args![refr, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    // ---- 0055d760: InitItem ----------------------------------------------------

    const INIT_CALLEES: &[u32] = &[
        0x0040_1280,
        0x0041_6be0,
        0x0041_81e0,
        0x0041_9970,
        0x0041_9ad0,
        0x0041_aeb0,
        0x0042_1f30,
        0x0042_5fd0,
        0x0042_cde0,
        0x0043_8f50,
        0x0043_9180,
        0x0043_9f90,
        0x0043_b2b0,
        0x0045_34f0,
        0x0046_23f0,
        0x0046_a010,
        0x0047_53d0,
        0x0047_c850,
        0x0047_cdb0,
        0x0047_ce10,
        0x0048_46a0,
        0x0048_4ab0,
        0x0048_4af0,
        0x004b_1480,
        0x004b_f220,
        0x004d_1440,
        0x004d_1610,
        0x004d_1960,
        0x004f_ede0,
        0x004f_f150,
        0x0054_4c30,
        0x0054_4c60,
        0x0054_9580,
        0x0055_e230,
        0x0056_4e60,
        0x0056_5730,
        0x0056_7490,
        0x0056_9840,
        0x0056_a9f0,
        0x0056_aa70,
        0x0056_ac00,
        0x0056_afc0,
        0x0057_22c0,
        0x0057_2c80,
        0x0057_5d70,
        0x0058_6170,
        0x0058_78d0,
        0x005f_6e60,
        0x006f_cfa0,
        0x0088_4350,
        0x008a_1800,
        0x0096_d470,
        0x0098_ddd0,
        0x0040_1000,
        MESSAGE,
        EXTRA_REMOVE_TYPE,
        0x0041_8770,
        0x0044_0e30,
    ];

    /// A reference ready for `InitItem`: every callee stubbed, a base object
    /// of type 0x28 and the CRT's float tests given real bodies. The
    /// scale getter reads the float at +0x3c.
    fn init_engine() -> (Engine, Ptr<TESObjectREFR>) {
        let mut e = engine();
        stub(&mut e, INIT_CALLEES);
        e.register(CRT_FINITE, |_, a| {
            u32::from(f64::take(a, &mut 0).is_finite()).into_ret()
        });
        e.register(CRT_ISNAN, |_, a| {
            u32::from(f64::take(a, &mut 0).is_nan()).into_ret()
        });
        e.register(0x0056_7400, |e, a| e.mem.f32(a[0] + 0x3c).into_ret());
        returns(&mut e, EXTRA_GET_DATA, 0);
        // ClampAngle: the identity unless a test changes it
        e.register(0x004b_1480, |_, a| f32::from_bits(a[0]).into_ret());
        // health percent 1.0 unless a test changes it
        e.register(0x0041_b550, |_, _| 1.0f32.into_ret());
        // TLS warning counter: the getter returns the stored value
        let counter = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let c = counter.clone();
        e.register_double(0x0046_e8a0, move |_, _| c.get().into_ret());
        let c = counter;
        e.register_double(0x004f_ffe0, move |_, a| {
            c.set(a[0]);
            Ret::default()
        });
        let refr = standard_refr(&mut e);
        let base = form(&mut e, 0x28, 0x0001_0000, 0);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(base),
        );
        e.set(refr, TESObjectREFR::fRefScale, 1.0f32);
        // the save/load object for the enable-parent part
        let save_load = e.mem.alloc(8);
        e.set_global(GLOBAL_SAVE_LOAD, save_load);
        (e, refr)
    }

    #[test]
    fn init_item_does_nothing_when_the_form_says_to_skip_it() {
        let (mut e, refr) = init_engine();
        e.mem.set_u32(refr.addr() + 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn init_item_replaces_a_corrupt_position_and_rotation_with_the_zero_vector() {
        let (mut e, refr) = init_engine();
        let me = refr.addr();
        for (i, v) in [1.0f32, f32::INFINITY, 3.0].iter().enumerate() {
            e.mem.set_f32(me + 0x30 + 4 * i as u32, *v);
        }
        for (i, v) in [4.0f32, 5.0, f32::NAN].iter().enumerate() {
            e.mem.set_f32(me + 0x24 + 4 * i as u32, *v);
        }
        for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.set_global(ZERO_VECTOR + 4 * i as u32, *v);
        }
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![CORRUPT_LOCATION_MESSAGE], vec![CORRUPT_ANGLE_MESSAGE]]
        );
        for i in 0..3 {
            assert_eq!(e.mem.f32(me + 0x30 + 4 * i), 10.0 * (i + 1) as f32);
            assert_eq!(e.mem.f32(me + 0x24 + 4 * i), 10.0 * (i + 1) as f32);
        }
    }

    #[test]
    fn init_item_leaves_a_sound_position_alone() {
        let (mut e, refr) = init_engine();
        let me = refr.addr();
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(me + 0x30 + 4 * i as u32, *v);
            e.mem.set_f32(me + 0x24 + 4 * i as u32, *v);
        }
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MESSAGE).is_empty());
        assert_eq!(e.mem.f32(me + 0x34), 2.0);
        // the extra list is initialized with the reference
        assert_eq!(calls_to(&log, 0x0041_6be0), vec![vec![me + 0x44, me]]);
    }

    #[test]
    fn init_item_scales_the_health_by_the_bases_health() {
        let (mut e, refr) = init_engine();
        let me = refr.addr();
        let base = e.mem.u32(me + 0x20);
        e.register(0x0041_b550, |_, _| 0.5f32.into_ret());
        returns(&mut e, 0x0048_73d0, 100);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0048_73d0), vec![vec![base]]);
        assert_eq!(
            calls_to(&log, 0x0041_9970),
            vec![vec![me + 0x44, 50.0f32.to_bits()]]
        );

        // full health: not touched
        e.register(0x0041_b550, |_, _| 1.0f32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x0041_9970).is_empty());
    }

    #[test]
    fn init_item_counts_a_single_ammo_stack_from_the_base() {
        let (mut e, refr) = init_engine();
        let me = refr.addr();
        let ammo = form(&mut e, 0x29, 0x0001_0001, 0);
        e.mem.set_u8(ammo + 0x84, 7); // the byte `00401170` reads at +0x80
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(ammo),
        );
        returns(&mut e, 0x0041_8770, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0041_9ad0), vec![vec![me + 0x44, 7]]);
        // a stack of two keeps its count
        returns(&mut e, 0x0041_8770, 2);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x0041_9ad0).is_empty());
    }

    #[test]
    fn init_item_marks_a_deleted_base_object_deleted() {
        let (mut e, refr) = init_engine();
        let base = e.mem.u32(refr.addr() + 0x20);
        e.mem.set_u32(base + 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00ff_0013), vec![vec![refr.addr(), 1]]);
        // a leveled base is reported
        e.mem.set_u8(base + 4, 0x2c);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MESSAGE), vec![vec![LEVELED_BASE_MESSAGE]]);
    }

    #[test]
    fn init_item_keeps_a_multi_bound_marker_for_a_scaled_reference() {
        let (mut e, refr) = init_engine();
        let me = refr.addr();
        e.set(refr, TESObjectREFR::fRefScale, 2.0f32);
        returns(&mut e, 0x0043_9f90, 1);
        let block = e.mem.alloc(12);
        returns(&mut e, 0x0040_1000, block);
        e.register(0x0043_8f50, |_, a| a[0].into_ret());
        // the marker's position is the three floats at the start of the block
        for (i, v) in [5.0f32, 6.0, 7.0].iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen2 = seen.clone();
        e.register_double(0x0098_ddd0, move |e, a| {
            seen2
                .borrow_mut()
                .push((a[0], e.mem.u32(a[1]), e.mem.u32(a[1] + 8)));
            Ret::default()
        });
        let world = e.mem.alloc(0x10);
        returns(&mut e, 0x0057_5d70, world);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        // a new marker is attached to the extra list; the position scaled in place
        assert_eq!(calls_to(&log, 0x0042_1f30), vec![vec![me + 0x44, block]]);
        assert_eq!(calls_to(&log, 0x0043_9180)[0][1], 2.0f32.to_bits());
        assert_eq!(
            *seen.borrow(),
            vec![(block, 5.0f32.to_bits(), 7.0f32.to_bits())]
        );
        // the scale goes back to 1.0, the world space learns of the bound ref
        assert_eq!(
            calls_to(&log, 0x0056_7490),
            vec![vec![me, 1.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, 0x0058_78d0), vec![vec![world, me]]);
    }

    #[test]
    fn init_item_puts_a_non_actor_at_its_starting_position() {
        let (mut e, refr) = init_engine();
        returns(&mut e, 0x0057_2c80, 1);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.set_global(ZERO_VECTOR + 4 * i as u32, *v);
        }
        e.register(0x00ff_0070, |_, _| Ret::default());
        e.mem.set_u32(REFR_VTABLE + 0x174, 0x00ff_0070);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x00ff_0070),
            vec![vec![
                refr.addr(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]]
        );
    }

    fn actor_refr(e: &mut Engine) -> Ptr<TESObjectREFR> {
        let refr = standard_refr(e);
        let base = form(e, 0x28, 0x0001_0000, 0);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(base),
        );
        e.set(refr, TESObjectREFR::fRefScale, 1.0f32);
        for (slot, target) in [
            (0x100u32, 0x00ff_0080u32),
            (0x290, 0x00ff_0081),
            (0x46c, 0x00ff_0082),
            (0x360, 0x00ff_0083),
            (0x218, 0x00ff_0084),
            (0x308, 0x00ff_0085),
            (0x1f8, 0x00ff_0086),
        ] {
            e.mem.set_u32(REFR_VTABLE + slot, target);
        }
        e.register(0x00ff_0080, |_, _| true.into_ret());
        stub(
            e,
            &[
                0x00ff_0081,
                0x00ff_0082,
                0x00ff_0083,
                0x00ff_0084,
                0x00ff_0085,
                0x00ff_0086,
            ],
        );
        refr
    }

    #[test]
    fn init_item_finishes_an_actor_loaded_dead_without_the_basics() {
        let (mut e, _) = init_engine();
        let actor = actor_refr(&mut e);
        let me = actor.addr();
        // an actor that is not alive-and-initialized: virtual +0x290 false
        // gets +0x46c; not dead per +0x360 but dead per 005722c0
        returns(&mut e, 0x0057_22c0, 1);
        let data = e.mem.alloc(0x80);
        returns(&mut e, 0x0041_81e0, data);
        returns(&mut e, 0x0047_cdb0, 1);
        returns(&mut e, 0x0056_afc0, 0);
        returns(&mut e, 0x0046_1580, 0);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![actor]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00ff_0082), vec![vec![me]]);
        assert_eq!(calls_to(&log, 0x008a_1800), vec![vec![me, 2]]);
        assert_eq!(
            calls_to(&log, 0x0096_d470),
            vec![vec![OBJECT_PROCESS_LISTS, me, 3]]
        );
        // the base data at +0x30 of the actor's data block
        assert_eq!(calls_to(&log, 0x0047_ce10), vec![vec![data + 0x30, me]]);
        // 0055e200 was the translation: it asks 00461580 with 0x200
        assert_eq!(calls_to(&log, 0x0046_1580), vec![vec![data + 0x30, 0x200]]);
        assert_eq!(calls_to(&log, 0x00ff_0086), vec![vec![me, 1]]);
        // not a creature: the guard flag is not set
        assert!(calls_to(&log, 0x00ff_0085).is_empty());
    }

    #[test]
    fn init_item_flags_a_creature_guard_class() {
        let (mut e, _) = init_engine();
        let actor = actor_refr(&mut e);
        let me = actor.addr();
        e.register(0x00ff_0084, |_, _| true.into_ret());
        returns(&mut e, 0x0088_4350, 0x0c1a);
        returns(&mut e, 0x005f_6e60, 1);
        let data = e.mem.alloc(0x80);
        returns(&mut e, 0x0041_81e0, data);
        returns(&mut e, 0x0046_1580, 0);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![actor]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x005f_6e60), vec![vec![0x0c1a]]);
        assert_eq!(calls_to(&log, 0x00ff_0085), vec![vec![me, 1]]);
    }

    #[test]
    fn init_item_updates_the_players_speed_hook() {
        let (mut e, _) = init_engine();
        let player = actor_refr(&mut e);
        e.set_global(GLOBAL_PLAYER, player.addr());
        returns(&mut e, 0x0041_81e0, 0x4000);
        returns(&mut e, 0x0046_1580, 0);
        e.register(0x0064_7f00, |_, _| 77.5f32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![player]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0064_7f00),
            vec![vec![player.addr() + 0xa4, 0, 0, 0, 1, 0, 0]]
        );
        assert_eq!(calls_to(&log, 0x0055_e230), vec![vec![77.5f32.to_bits()]]);
    }

    #[test]
    fn init_item_follows_the_enable_state_parent() {
        let (mut e, refr) = init_engine();
        let me = refr.addr();
        let parent = e.mem.alloc(0x80);
        e.mem.set_u32(parent + 8, 0x800); // the parent is disabled
        returns(&mut e, 0x0056_a9f0, parent);
        // the opposite flag: this reference is disabled when the parent is not
        returns(&mut e, 0x0056_aa70, 1);
        returns(&mut e, 0x0047_c850, 0);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0048_4af0), vec![vec![me, 0]]);
        // the save/load object's flag was set to 0 and then restored
        let flags = calls_to(&log, 0x0045_34f0);
        assert_eq!(flags.len(), 2);
        assert_eq!(flags[0][1], 0);

        // same sense: this reference takes the parent's state
        returns(&mut e, 0x0056_aa70, 0);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0048_4af0), vec![vec![me, 1]]);
    }

    #[test]
    fn init_item_reports_errors_raised_while_it_ran_in_three_shapes() {
        // no cell
        let (mut e, refr) = init_engine();
        let counter_bump = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let c = counter_bump.clone();
        // the second read of the warning counter sees a new warning
        e.register_double(0x0046_e8a0, move |_, _| {
            c.set(c.get() + 1);
            (if c.get() == 1 { 0u32 } else { 2 }).into_ret()
        });
        let base = e.mem.u32(refr.addr() + 0x20);
        e.register(0x00ff_0090, |_, _| 0x0d01u32.into_ret());
        let named_base = object_with_vtable(&mut e, 0x100, &[(0x130, 0x00ff_0090)]);
        e.mem.set_u8(named_base + 4, 0x28);
        e.mem.set_u32(named_base + 0xc, 0x0001_0000);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(named_base),
        );
        let _ = base;
        returns(&mut e, 0x0040_1280, 0x0d02);
        e.mem.set_u32(refr.addr() + 0xc, 0x00ab_cdef);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![
                INIT_ITEM_ERROR_NO_CELL,
                0x0d01,
                0x0001_0000,
                0x0d02,
                0x00ab_cdef
            ]]
        );

        // a plain cell
        let cell = object_with_vtable(&mut e, 0x100, &[(0x130, 0x00ff_0090)]);
        e.mem.set_u32(cell + 0xc, 0x0007_0001);
        e.set(refr, TESObjectREFR::pParentCell, Ptr::new(cell));
        returns(&mut e, 0x0042_5fd0, 1);
        counter_bump.set(0);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![
                INIT_ITEM_ERROR_CELL,
                0x0d01,
                0x0001_0000,
                0x0d02,
                0x00ab_cdef,
                0x0d01,
                0x0007_0001
            ]]
        );

        // a cell with coordinates in a world
        returns(&mut e, 0x0042_5fd0, 0);
        let world = object_with_vtable(&mut e, 0x100, &[(0x130, 0x00ff_0090)]);
        e.mem.set_u32(world + 0xc, 0x0009_0001);
        returns(&mut e, 0x0057_5d70, world);
        returns(&mut e, 0x0054_4c30, 12);
        returns(&mut e, 0x0054_4c60, 34);
        counter_bump.set(0);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![
                INIT_ITEM_ERROR_WORLD,
                0x0d01,
                0x0001_0000,
                0x0d02,
                0x00ab_cdef,
                0x0d01,
                0x0007_0001,
                12,
                34,
                0x0d01,
                0x0009_0001
            ]]
        );
    }

    #[test]
    fn init_item_initializes_a_containers_inventory_and_the_script() {
        let (mut e, refr) = init_engine();
        let me = refr.addr();
        let container = form(&mut e, 0x1b, 5, 0);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(container),
        );
        returns(&mut e, 0x004b_f220, 0x1c1c);
        returns(&mut e, 0x0042_cde0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        for step in [0x004d_1440, 0x004d_1610, 0x004d_1960, 0x0042_cde0] {
            assert_eq!(calls_to(&log, step), vec![vec![0x1c1c]], "{step:08x}");
        }
        assert_eq!(calls_to(&log, 0x0041_aeb0), vec![vec![me + 0x44]]);
        assert_eq!(calls_to(&log, 0x0056_5730), vec![vec![me]]);
        assert_eq!(calls_to(&log, 0x0046_a010), vec![vec![me, 0]]);
    }

    #[test]
    fn init_item_hides_the_terrain_tree_of_a_disabled_exterior_object() {
        let (mut e, refr) = init_engine();
        let me = refr.addr();
        e.mem.set_u32(me + 8, 0x800); // disabled
        returns(&mut e, 0x0054_9580, 1);
        let world = e.mem.alloc(0x10);
        returns(&mut e, 0x0057_5d70, world);
        returns(&mut e, 0x0058_6170, 0x7e7e);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x006f_cfa0), vec![vec![0x7e7e, me, 1]]);
    }

    #[test]
    fn init_item_clamps_the_rotation_and_drops_the_extra_of_type_0x6e() {
        let (mut e, refr) = init_engine();
        let me = refr.addr();
        let base = e.mem.u32(me + 0x20);
        e.mem.set_u8(base + 4, 0x2a);
        e.mem.set_f32(me + 0x2c, 7.0);
        e.register(0x004b_1480, |_, a| (f32::from_bits(a[0]) - 6.0).into_ret());
        returns(&mut e, EXTRA_GET_DATA, 0x0e6e);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(me + 0x2c), 1.0);
        assert_eq!(calls_to(&log, EXTRA_GET_DATA), vec![vec![me + 0x44, 0x6e]]);
        assert_eq!(
            calls_to(&log, EXTRA_REMOVE_TYPE),
            vec![vec![me + 0x44, 0x6e]]
        );
        // a base of type 0x28 keeps it
        let base = e.mem.u32(me + 0x20);
        e.mem.set_u8(base + 4, 0x28);
        e.call_log = Some(vec![]);
        e.call(0x0055_d760, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, EXTRA_REMOVE_TYPE).len(), 0);
    }

    // ---- 0055e1d0 .. 0055e730 -----------------------------------------------------

    #[test]
    fn counters_and_flags_in_the_tail_of_the_range() {
        let mut e = engine();
        let cell = e.mem.alloc(0x100);
        e.mem.set_u16(cell + 0xa8, 5);
        e.call(0x0055_e1d0, &args![cell]);
        assert_eq!(e.mem.u16(cell + 0xa8), 4);
        e.mem.set_u16(cell + 0xa8, 0);
        e.call(0x0055_e1d0, &args![cell]);
        assert_eq!(e.mem.u16(cell + 0xa8), 0xffff);

        returns(&mut e, 0x0046_1580, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0055_e200, &args![cell]).bool());
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), 0x0046_1580),
            vec![vec![cell, 0x200]]
        );
        returns(&mut e, 0x0046_1580, 0);
        assert!(e.call(0x0055_e200, &args![cell]).bool());
    }

    // ---- 0055e250 / 0055e3f0 / 0055e4d0 / 0055e5e0: group ordering --------------

    /// A cell object whose virtuals at +0x38, +0x3c and +0x110 are recorders
    /// returning 1, with `id` as its form ID.
    fn recording_cell(e: &mut Engine, id: u32) -> u32 {
        for addr in [0x00ff_00a0, 0x00ff_00a1, 0x00ff_00a2] {
            e.register(addr, |_, _| true.into_ret());
        }
        let cell = object_with_vtable(
            e,
            0x100,
            &[
                (0x38, 0x00ff_00a0),
                (0x3c, 0x00ff_00a1),
                (0x110, 0x00ff_00a2),
            ],
        );
        e.mem.set_u32(cell + 0xc, id);
        cell
    }

    fn group_engine() -> (Engine, Ptr<TESObjectREFR>, u32) {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        stub(&mut e, &[GET_REF_PERSISTS]);
        e.set_global(GROUP_DATA_VTABLE_PTR, 0x0101_c654u32);
        let refr = standard_refr(&mut e);
        let cell = recording_cell(&mut e, 0x0007_0001);
        e.set(refr, TESObjectREFR::pParentCell, Ptr::new(cell));
        (e, refr, cell)
    }

    /// A group-data record (the class `fn_0055e5e0` produces).
    fn group_record(e: &mut Engine, kind: u32, form_id: u32) -> u32 {
        let record = e.mem.alloc(0x14);
        e.mem
            .set_u32(record, e.global::<u32>(GROUP_DATA_VTABLE_PTR));
        e.mem.set_u32(record + 8, form_id);
        e.mem.set_u32(record + 0xc, kind);
        record
    }

    /// A form object with a type, whose slot-0 virtual reports `cell`.
    fn form_in_cell(e: &mut Engine, kind: u8, cell: u32) -> u32 {
        e.register_double(0x00ff_00b0, move |_, _| cell.into_ret());
        let form = object_with_vtable(e, 0x100, &[(0, 0x00ff_00b0)]);
        e.mem.set_u8(form + 4, kind);
        form
    }

    #[test]
    fn saves_before_defers_to_the_cell_for_forms_that_are_not_references() {
        let (mut e, refr, cell) = group_engine();
        returns(&mut e, 0x0055_48a0, 0);
        let other = form_in_cell(&mut e, 0x10, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0055_e250, &args![refr, other]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00ff_00a1), vec![vec![cell, other]]);
    }

    #[test]
    fn saves_before_compares_cells_then_persistence() {
        let (mut e, refr, cell) = group_engine();
        returns(&mut e, 0x0055_48a0, 1);
        // another cell: the cell decides, given that cell
        let elsewhere = e.mem.alloc(0x10);
        let other = form_in_cell(&mut e, 0x3a, elsewhere);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0055_e250, &args![refr, other]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00ff_00a1), vec![vec![cell, elsewhere]]);

        // the same cell, a reference form: before only when this persists and it does not
        let kinds_and_persistence = [
            (0x3au8, true, false, true),
            (0x3a, true, true, false),
            (0x3a, false, false, false),
            (0x69, true, false, true),
            (0x42, true, false, true),
            (0x43, false, false, false),
            (0x41, true, false, false),
        ];
        for (kind, mine, theirs, expected) in kinds_and_persistence {
            let other = form_in_cell(&mut e, kind, cell);
            let me = refr.addr();
            let other_addr = other;
            e.register_double(GET_REF_PERSISTS, move |_, a| {
                (if a[0] == me { mine } else { theirs }).into_ret()
            });
            let _ = other_addr;
            assert_eq!(
                e.call(0x0055_e250, &args![refr, other]).bool(),
                expected,
                "kind {kind:#x} mine {mine} theirs {theirs}"
            );
        }
    }

    #[test]
    fn saves_before_for_a_group_record_only_orders_this_references_cell_group() {
        let (mut e, refr, cell) = group_engine();
        let me = refr.addr();
        let persists = std::rc::Rc::new(std::cell::Cell::new(true));
        let p = persists.clone();
        e.register_double(GET_REF_PERSISTS, move |_, a| {
            (a[0] == me && p.get()).into_ret()
        });
        e.register_double(0x0048_39c0, move |_, a| {
            (if a[0] == 0x0007_0001 { cell } else { 0 }).into_ret()
        });
        // null or a record of another class
        assert!(!e.call(0x0055_e3f0, &args![refr, 0u32]).bool());
        let alien = e.mem.alloc(0x14);
        assert!(!e.call(0x0055_e3f0, &args![refr, alien]).bool());
        // kind 9 for its own cell while persisting
        let record = group_record(&mut e, 9, 0x0007_0001);
        assert!(e.call(0x0055_e3f0, &args![refr, record]).bool());
        // kind 8, a different cell, or not persisting: no
        let kind8 = group_record(&mut e, 8, 0x0007_0001);
        assert!(!e.call(0x0055_e3f0, &args![refr, kind8]).bool());
        let other_cell = group_record(&mut e, 9, 0x0009_0009);
        assert!(!e.call(0x0055_e3f0, &args![refr, other_cell]).bool());
        persists.set(false);
        assert!(!e.call(0x0055_e3f0, &args![refr, record]).bool());
        // another kind goes to the cell's virtual at +0x38
        let other_kind = group_record(&mut e, 3, 5);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0055_e3f0, &args![refr, other_kind]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00ff_00a0), vec![vec![cell, other_kind]]);
    }

    #[test]
    fn belongs_in_group_applies_the_kind_rules() {
        let (mut e, refr, cell) = group_engine();
        let me = refr.addr();
        let persists = std::rc::Rc::new(std::cell::Cell::new(true));
        let p = persists.clone();
        e.register_double(GET_REF_PERSISTS, move |_, a| {
            (a[0] == me && p.get()).into_ret()
        });
        // 00485be0(cell, id): the group's form is in this cell
        e.register(0x0048_5be0, |_, a| (a[1] == 0x0007_0001).into_ret());

        assert!(!e.call(0x0055_e4d0, &args![refr, 0u32, 1u32, 1u32]).bool());
        let alien = e.mem.alloc(0x14);
        assert!(!e.call(0x0055_e4d0, &args![refr, alien, 1u32, 1u32]).bool());

        // kind 6 needs the first flag; then the cell test; with the second flag clear or kind 6: true
        let kind6 = group_record(&mut e, 6, 0x0007_0001);
        assert!(!e.call(0x0055_e4d0, &args![refr, kind6, 0u32, 0u32]).bool());
        assert!(e.call(0x0055_e4d0, &args![refr, kind6, 1u32, 0u32]).bool());
        assert!(e.call(0x0055_e4d0, &args![refr, kind6, 1u32, 1u32]).bool());
        let elsewhere6 = group_record(&mut e, 6, 0x0009_0009);
        assert!(!e
            .call(0x0055_e4d0, &args![refr, elsewhere6, 1u32, 1u32])
            .bool());

        // kinds 8 and 9 with the second flag set depend on persistence
        let kind8 = group_record(&mut e, 8, 0x0007_0001);
        let kind9 = group_record(&mut e, 9, 0x0007_0001);
        assert!(e.call(0x0055_e4d0, &args![refr, kind8, 0u32, 0u32]).bool());
        assert!(e.call(0x0055_e4d0, &args![refr, kind8, 0u32, 1u32]).bool());
        assert!(!e.call(0x0055_e4d0, &args![refr, kind9, 0u32, 1u32]).bool());
        persists.set(false);
        assert!(!e.call(0x0055_e4d0, &args![refr, kind8, 0u32, 1u32]).bool());
        assert!(e.call(0x0055_e4d0, &args![refr, kind9, 0u32, 1u32]).bool());

        // any other kind goes to the cell's virtual +0x110 when the first flag is set
        let other = group_record(&mut e, 3, 5);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0055_e4d0, &args![refr, other, 1u32, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00ff_00a2), vec![vec![cell, other, 1, 0]]);
        assert!(!e.call(0x0055_e4d0, &args![refr, other, 0u32, 1u32]).bool());
    }

    #[test]
    fn create_group_data_builds_the_record_for_the_parent_group() {
        let (mut e, refr, cell) = group_engine();
        let me = refr.addr();
        let persists = std::rc::Rc::new(std::cell::Cell::new(true));
        let p = persists.clone();
        e.register_double(GET_REF_PERSISTS, move |_, a| {
            (a[0] == me && p.get()).into_ret()
        });
        returns(&mut e, 0x0055_16c0, 0);
        returns(&mut e, 0x0054_4210, 0x00ab_0001);
        let cell_id = e.mem.u32(cell + 0xc);

        // null out record: nothing
        e.call(0x0055_e5e0, &args![refr, 0u32, 0u32]);

        // a parent of kind 3 whose form is the cell's 00544210: becomes kind 6
        let out = e.mem.alloc(0x14);
        for i in 0..5 {
            e.mem.set_u32(out + 4 * i, 0xffff_ffff);
        }
        let parent = group_record(&mut e, 3, 0x00ab_0001);
        e.call(0x0055_e5e0, &args![refr, out, parent]);
        assert_eq!(e.mem.u32(out), 0x0101_c654);
        assert_eq!(e.mem.u32(out + 4), 0);
        assert_eq!(e.mem.u32(out + 8), cell_id);
        assert_eq!(e.mem.u32(out + 0xc), 6);
        assert_eq!(e.mem.u32(out + 0x10), 0);

        // a parent of kind 1 matches through 005516c0 too
        let out = e.mem.alloc(0x14);
        let parent = group_record(&mut e, 1, 0x0099_9999);
        e.call(0x0055_e5e0, &args![refr, out, parent]);
        assert_eq!(e.mem.u32(out), 0);
        returns(&mut e, 0x0055_16c0, 1);
        e.call(0x0055_e5e0, &args![refr, out, parent]);
        assert_eq!(e.mem.u32(out + 0xc), 6);

        // a parent of kind 6 for this cell: kind 8 when persisting, 9 when not
        let parent = group_record(&mut e, 6, cell_id);
        let out = e.mem.alloc(0x14);
        e.call(0x0055_e5e0, &args![refr, out, parent]);
        assert_eq!(e.mem.u32(out + 0xc), 8);
        persists.set(false);
        let out = e.mem.alloc(0x14);
        e.call(0x0055_e5e0, &args![refr, out, parent]);
        assert_eq!(e.mem.u32(out + 0xc), 9);

        // other parents (kind 2, or kind 6 for another cell) leave out[0] cleared
        for (kind, id) in [(2u32, 0x00ab_0001u32), (6, 0x0099_9999)] {
            let out = e.mem.alloc(0x14);
            e.mem.set_u32(out, 0x1234);
            let parent = group_record(&mut e, kind, id);
            e.call(0x0055_e5e0, &args![refr, out, parent]);
            assert_eq!(e.mem.u32(out), 0);
        }
    }

    // ---- 0055e730: GetSaveSize -------------------------------------------------

    fn save_size_engine() -> (Engine, Ptr<TESObjectREFR>) {
        let mut e = engine();
        returns(&mut e, 0x0048_4bf0, 10);
        returns(&mut e, 0x0086_2110, 0);
        returns(&mut e, 0x0041_8520, 0x1c1c);
        returns(&mut e, 0x004d_3960, 20);
        returns(&mut e, 0x0042_2c40, 30);
        returns(&mut e, 0x0055_f880, 40);
        returns(&mut e, 0x0056_0350, 50);
        returns(&mut e, 0x008d_f040, 0x42);
        let switches = e.mem.alloc(8);
        returns(&mut e, 0x0040_8d60, switches);
        stub(&mut e, &[ERROR_LOG, 0x004f_d3e0]);
        let save_load = e.mem.alloc(8);
        e.set_global(GLOBAL_SAVE_LOAD, save_load);
        let refr = standard_refr(&mut e);
        (e, refr)
    }

    #[test]
    fn save_size_adds_up_the_parts_for_the_change_flags() {
        let (mut e, refr) = save_size_engine();
        let size = |e: &mut Engine, flags: u32| e.call(0x0055_e730, &args![refr, flags]).u16();
        assert_eq!(size(&mut e, 0), 10);
        // flag 0x20: the container changes
        assert_eq!(size(&mut e, 0x20), 30);
        // flag 4: 2 + 0560350
        assert_eq!(size(&mut e, 4), 10 + 2 + 50);
        // flag 0x10000000 on a non-actor: 2 + 0055f880
        assert_eq!(size(&mut e, 0x1000_0000), 10 + 2 + 40);
        // flag 0x10 only counts from version 0x43
        assert_eq!(size(&mut e, 0x10), 10);
        returns(&mut e, 0x008d_f040, 0x43);
        assert_eq!(size(&mut e, 0x10), 14);
        // save blocks: +6
        returns(&mut e, 0x0086_2110, 1);
        assert_eq!(size(&mut e, 0), 16);
    }

    #[test]
    fn save_size_of_an_actor_adds_its_extras_and_skips_the_non_actor_part() {
        let (mut e, refr) = save_size_engine();
        e.register(0x00ff_0012, |_, _| true.into_ret());
        e.call_log = Some(vec![]);
        let total = e.call(0x0055_e730, &args![refr, 0x1000_0000u32]).u16();
        let log = e.call_log.take().unwrap();
        assert_eq!(total, 10 + 30);
        assert_eq!(
            calls_to(&log, 0x0042_2c40),
            vec![vec![refr.addr() + 0x44, 0x1000_0000, refr.addr()]]
        );
        assert!(calls_to(&log, 0x0055_f880).is_empty());
    }

    #[test]
    fn save_size_wraps_at_16_bits() {
        let (mut e, refr) = save_size_engine();
        returns(&mut e, 0x0048_4bf0, 0xfffe);
        assert_eq!(
            e.call(0x0055_e730, &args![refr, 4u32]).u16(),
            0xfffeu16.wrapping_add(2 + 50)
        );
    }

    #[test]
    fn save_size_logs_what_it_computed_when_the_debug_switch_is_set() {
        let (mut e, refr) = save_size_engine();
        let switches = e.mem.alloc(8);
        e.mem.set_u8(switches, 1);
        returns(&mut e, 0x0040_8d60, switches);
        // without a world space
        e.call_log = Some(vec![]);
        e.call(0x0055_e730, &args![refr, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ERROR_LOG),
            vec![vec![SAVE_SIZE_SHORT_FORMAT, 52, SOURCE_LINE, SOURCE_FILE]]
        );
        // with one: its form ID, name, and the dword at +5
        let world = e.mem.alloc(0x20);
        e.mem.set_u32(world, 0x0006_0001);
        e.mem.write(world + 5, &0xcafe_f00du32.to_le_bytes());
        returns(&mut e, 0x004f_d3e0, world);
        e.register(0x00ff_00c0, |_, _| 0x0e01u32.into_ret());
        let form = object_with_vtable(&mut e, 0x40, &[(0x130, 0x00ff_00c0)]);
        returns(&mut e, 0x0048_39c0, form);
        e.call_log = Some(vec![]);
        e.call(0x0055_e730, &args![refr, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ERROR_LOG),
            vec![vec![
                SAVE_SIZE_FORMAT,
                52,
                0x0006_0001,
                0x0e01,
                0xcafe_f00d,
                SOURCE_LINE,
                SOURCE_FILE
            ]]
        );
    }

    // ===== 0055e230 to 005612a0 ==============================================

    /// An engine with the pages the batch's globals live on.
    fn batch_engine() -> Engine {
        let mut e = engine();
        for page in [
            0x0126_7000,
            0x0101_a000,
            0x0119_7000,
            0x0101_6000,
            0x0101_1000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(MINUS_ONE_DOUBLE, -1.0f64);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(STEP_DIVISOR, 20.0f64);
        e.set_global(MIN_STEP_DOUBLE, 0.0166666f64);
        e.set_global(MIN_STEP_FLOAT, 0.0166666f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(THREE, 3.0f32);
        e
    }

    /// Registers the double of a virtual slot of the shared reference
    /// vtable at address `0x00fd0000 + offset`, so call logs name it.
    fn slot(e: &mut Engine, offset: u32, f: AbiFn) {
        let target = 0x00fd_0000 + offset;
        e.register(target, f);
        e.mem.set_u32(REFR_VTABLE + offset, target);
    }

    /// A reference whose `IsActor` slot answers `actor`, with harmless
    /// doubles in the other virtual slots the batch calls (unless a test
    /// installed one first): `+0x1cc`, `+0x208`, `+0xfc`, `+0x1c4` do
    /// nothing; `+0x21c`, `+0x22c`, `+0x234` answer false; `+0x1e4` null;
    /// `+0x1f4` is `this + 0x30`; `+0x130` is a fixed name.
    fn batch_refr(e: &mut Engine, actor: bool) -> Ptr<TESObjectREFR> {
        slot(
            e,
            0x100,
            if actor {
                |_, _| true.into_ret()
            } else {
                |_, _| false.into_ret()
            },
        );
        let defaults: [(u32, AbiFn); 10] = [
            (0x1cc, |_, _| Ret::default()),
            (0x208, |_, _| Ret::default()),
            (0xfc, |_, _| Ret::default()),
            (0x1c4, |_, _| Ret::default()),
            (0x21c, |_, _| false.into_ret()),
            (0x22c, |_, _| false.into_ret()),
            (0x234, |_, _| false.into_ret()),
            (0x1e4, |_, _| 0u32.into_ret()),
            (0x1f4, |_, a| (a[0] + 0x30).into_ret()),
            (0x130, |_, _| 0x0e01u32.into_ret()),
        ];
        for (offset, f) in defaults {
            if e.mem.u32(REFR_VTABLE + offset) == 0 {
                slot(e, offset, f);
            }
        }
        new_refr(e)
    }

    /// Gives the reference a base object of the given form type.
    fn with_base(e: &mut Engine, refr: Ptr<TESObjectREFR>, kind: u8) -> u32 {
        let base = form(e, kind, 0x0001_0001, 0);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(base),
        );
        base
    }

    fn stream_write(e: &mut Engine, game: u32, source: u32, size: u32) {
        let bytes = e.mem.bytes(source, size);
        let position = e.mem.u32(game + 0x14);
        e.mem.write(position, &bytes);
        e.mem.set_u32(game + 0x14, position + size);
    }

    fn stream_read(e: &mut Engine, game: u32, dest: u32, size: u32) {
        let position = e.mem.u32(game + 0x14);
        let bytes = e.mem.bytes(position, size);
        e.mem.write(dest, &bytes);
        e.mem.set_u32(game + 0x14, position + size);
    }

    /// Installs a `TESSaveLoadGame` double at the global (position pointer
    /// at +0x14, version byte at +0x80, "use save game blocks" byte at
    /// +0x1f0) with the stream functions working on a 0x2000-byte buffer.
    /// Returns the buffer.
    fn set_up_stream(e: &mut Engine, version: u8, blocks: bool) -> u32 {
        let game = e.mem.alloc(0x200);
        let buffer = e.mem.alloc(0x2000);
        e.mem.set_u32(game + 0x14, buffer);
        e.mem.set_u8(game + 0x80, version);
        e.mem.set_u8(game + 0x1f0, blocks as u8);
        e.set_global(GLOBAL_SAVE_LOAD, game);
        e.register(SAVE_POSITION, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(SAVE_VERSION, |e, a| e.mem.u8(a[0] + 0x80).into_ret());
        e.register(USE_SAVE_GAME_BLOCKS, |e, a| {
            (e.mem.u8(a[0] + 0x1f0) != 0).into_ret()
        });
        e.register(SAVE_WRITE, |e, a| {
            stream_write(e, a[0], a[1], a[2]);
            Ret::default()
        });
        e.register(SAVE_READ, |e, a| {
            stream_read(e, a[0], a[1], a[2]);
            Ret::default()
        });
        e.register(SAVE_SKIP, |e, a| {
            let position = e.mem.u32(a[0] + 0x14);
            e.mem.set_u32(a[0] + 0x14, position + a[1]);
            Ret::default()
        });
        e.register(FORM_SAVE_DATA, |e, a| {
            let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
            stream_write(e, game, a[1], a[2]);
            Ret::default()
        });
        e.register(FORM_LOAD_DATA, |e, a| {
            let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
            stream_read(e, game, a[1], a[2]);
            Ret::default()
        });
        buffer
    }

    fn save_setup(version: u8, blocks: bool) -> (Engine, u32, u32) {
        let mut e = batch_engine();
        let buffer = set_up_stream(&mut e, version, blocks);
        let switches = e.mem.alloc(8);
        returns(&mut e, 0x0040_8d60, switches);
        stub(
            &mut e,
            &[
                FORM_SAVE_GAME,
                CONTAINER_CHANGES_SAVE,
                EXTRA_SAVE_GAME,
                ERROR_LOG,
                MESSAGE,
                SAVING_FORM_HEADER,
                GET_LOADED_3D,
            ],
        );
        returns(&mut e, EXTRA_GET_CONTAINER_CHANGES, 0x1c1c);
        (e, buffer, switches)
    }

    // ---- 0055e230 --------------------------------------------------------

    #[test]
    fn running_speed_is_kept_only_when_above_zero() {
        let mut e = batch_engine();
        e.call(0x0055_e230, &args![4.5f32]);
        assert_eq!(e.global::<f32>(RUNNING_SPEED), 4.5);
        for rejected in [0.0f32, -1.0, f32::NAN] {
            e.call(0x0055_e230, &args![rejected]);
        }
        assert_eq!(e.global::<f32>(RUNNING_SPEED), 4.5);
        e.call(0x0055_e230, &args![0.25f32]);
        assert_eq!(e.global::<f32>(RUNNING_SPEED), 0.25);
    }

    // ---- 0055e940: SaveGame ------------------------------------------------

    #[test]
    fn save_game_frames_its_data_in_a_sized_block() {
        let (mut e, buffer, _) = save_setup(0x43, true);
        let refr = batch_refr(&mut e, false);
        e.set(refr, TESObjectREFR::fRefScale, 1.5);
        e.call_log = Some(vec![]);
        e.call(0x0055_e940, &args![refr, 0x10u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FORM_SAVE_GAME),
            vec![vec![refr.addr(), 0x10]]
        );
        // the tag, the size (the placeholder plus the scale), the scale
        assert_eq!(e.mem.bytes(buffer, 4), b"KOLB");
        assert_eq!(e.mem.u16(buffer + 4), 6);
        assert_eq!(e.mem.f32(buffer + 6), 1.5);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 10);
        // an older save version does not write the scale
        let (mut e, buffer, _) = save_setup(0x42, false);
        let refr = batch_refr(&mut e, false);
        e.call(0x0055_e940, &args![refr, 0x10u32]);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer);
    }

    #[test]
    fn save_game_writes_each_part_for_its_flag() {
        let (mut e, buffer, _) = save_setup(0x43, false);
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_e940, &args![refr, 0x1000_0024u32]);
        let log = e.call_log.take().unwrap();
        // flag 0x20: the container changes of the extra list
        assert_eq!(
            calls_to(&log, EXTRA_GET_CONTAINER_CHANGES),
            vec![vec![refr.addr() + 0x44]]
        );
        assert_eq!(calls_to(&log, CONTAINER_CHANGES_SAVE), vec![vec![0x1c1c]]);
        // no extra data for a non-actor
        assert!(calls_to(&log, EXTRA_SAVE_GAME).is_empty());
        // flag 0x10000000: the controller data's size (2: no 3D) and its
        // empty list (a zero count); flag 4: a zero Havok size
        assert_eq!(e.mem.bytes(buffer, 6), [2, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn save_game_of_an_actor_saves_its_extras_and_not_the_controller_data() {
        let (mut e, buffer, _) = save_setup(0x43, false);
        let refr = batch_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0055_e940, &args![refr, 0x1000_0000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, EXTRA_SAVE_GAME),
            vec![vec![refr.addr() + 0x44, 0x1000_0000, refr.addr()]]
        );
        assert!(calls_to(&log, GET_LOADED_3D).is_empty());
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer);
    }

    #[test]
    fn save_game_reports_a_block_larger_than_a_short() {
        let (mut e, buffer, _) = save_setup(0x43, true);
        let refr = batch_refr(&mut e, false);
        // entry, after the tag, at the end
        let positions = [buffer + 0x10, buffer + 4, buffer + 4 + 0x1_0006];
        let mut next = 0;
        e.register_double(SAVE_POSITION, move |_, _| {
            next += 1;
            positions[next - 1].into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0055_e940, &args![refr, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![SAVE_BLOCK_TOO_LARGE_MESSAGE, SOURCE_FILE, 0xb01]]
        );
        // the size is the low 16 bits of the real one
        assert_eq!(e.mem.u16(buffer + 4), 6);
    }

    #[test]
    fn save_game_logs_what_it_wrote_when_the_debug_switch_is_set() {
        let (mut e, buffer, switches) = save_setup(0x43, false);
        e.mem.set_u8(switches, 1);
        let refr = batch_refr(&mut e, false);
        // without a form header
        e.call_log = Some(vec![]);
        e.call(0x0055_e940, &args![refr, 0x10u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ERROR_LOG),
            vec![vec![SAVE_GAME_SHORT_FORMAT, 4, 0xb01, SOURCE_FILE]]
        );
        // with one: form ID, name and the dword at +5
        let header = e.mem.alloc(0x20);
        e.mem.set_u32(header, 0x0006_0001);
        e.mem.write(header + 5, &0xcafe_f00du32.to_le_bytes());
        returns(&mut e, SAVING_FORM_HEADER, header);
        e.register(0x00fd_0130, |_, _| 0x0e01u32.into_ret());
        let named = object_with_vtable(&mut e, 0x40, &[(0x130, 0x00fd_0130)]);
        returns(&mut e, 0x0048_39c0, named);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        e.mem.set_u32(game + 0x14, buffer);
        e.call_log = Some(vec![]);
        e.call(0x0055_e940, &args![refr, 0x10u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ERROR_LOG),
            vec![vec![
                SAVE_GAME_FORMAT,
                4,
                0x0006_0001,
                0x0e01,
                0xcafe_f00d,
                0xb01,
                SOURCE_FILE
            ]]
        );
    }

    // ---- 0055ebf0 --------------------------------------------------------

    #[test]
    fn havok_save_data_constructor_clears_the_record() {
        let mut e = batch_engine();
        let record: Ptr<HavokSaveData> = e.new_object();
        e.mem.write(record.addr(), &[0xff; 0x14]);
        let result = e.call(0x0055_ebf0, &args![record]).ptr::<HavokSaveData>();
        assert_eq!(result, record);
        // the fields are cleared; the padding bytes (1, 6, 7) are not touched
        assert_eq!(
            e.mem.bytes(record.addr(), 0x14),
            [0, 255, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    // ---- 0055ec40: LoadGame ------------------------------------------------

    /// An engine with a stream holding `stream`, and the callees `LoadGame`
    /// leaves to others stubbed.
    fn load_setup(version: u8, blocks: bool, stream: &[u8]) -> (Engine, u32) {
        let mut e = batch_engine();
        let buffer = set_up_stream(&mut e, version, blocks);
        e.mem.write(buffer, stream);
        stub(
            &mut e,
            &[
                FORM_LOAD_GAME,
                FORM_SET_EMPTY,
                SAVE_LOAD_UNAVAILABLE,
                FORM_SET_DISABLED,
                ENABLE_PARENT,
                LOADING_FORM_HEADER,
                MESSAGE,
                EXTRA_LOAD_GAME,
                EXTRA_GET_PERSISTENT_CELL,
                REMOVE_WEAPON,
                FORM_PREPARE,
                FORM_STEP,
                0x0041_aeb0,
                0x008a_dc50,
                GET_INVENTORY_CHANGES,
                INVENTORY_CHANGES_LOAD,
                LOAD_OLD_RECORD_A,
                LOAD_OLD_RECORD_B,
                SET_SCALE,
                EXTRA_REMOVE_LAST_SEQUENCE,
            ],
        );
        (e, buffer)
    }

    fn block_stream(tag: &[u8; 4], size: u16) -> Vec<u8> {
        let mut bytes = tag.to_vec();
        bytes.extend_from_slice(&size.to_le_bytes());
        bytes
    }

    /// A form header (form ID, flags dword at +5, version byte at +9) for
    /// `LOADING_FORM_HEADER`, and the form with the name getter its ID
    /// resolves to.
    fn form_header(e: &mut Engine) -> u32 {
        let header = e.mem.alloc(0x20);
        e.mem.set_u32(header, 0x0006_0001);
        e.mem.write(header + 5, &0x00ab_cdefu32.to_le_bytes());
        e.mem.set_u8(header + 9, 0x2b);
        returns(e, LOADING_FORM_HEADER, header);
        e.register(0x00fd_0130, |_, _| 0x0e01u32.into_ret());
        let named = object_with_vtable(e, 0x40, &[(0x130, 0x00fd_0130)]);
        returns(e, 0x0048_39c0, named);
        header
    }

    #[test]
    fn load_game_checks_the_block_size_against_where_the_read_ended() {
        let refr_flags = (0u32, 0u32);
        // exact: no message
        let (mut e, _) = load_setup(0x43, true, &block_stream(b"KOLB", 2));
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, refr_flags.0, refr_flags.1]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MESSAGE).is_empty());
        assert_eq!(
            calls_to(&log, FORM_LOAD_GAME),
            vec![vec![refr.addr(), 0, 0]]
        );
        // the block claims 3 bytes after its tag: the read ended 1 short
        let (mut e, _) = load_setup(0x43, true, &block_stream(b"KOLB", 3));
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![LOAD_UNDERRUN_FORMAT, 1, SOURCE_FILE, 0xb8f, 0x43]]
        );
        // it claims 1: the read went 1 over
        let (mut e, _) = load_setup(0x43, true, &block_stream(b"KOLB", 1));
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![LOAD_OVERRUN_FORMAT, 1, SOURCE_FILE, 0xb8f, 0x43]]
        );
        // with a form being loaded, the message names it
        let (mut e, _) = load_setup(0x43, true, &block_stream(b"KOLB", 1));
        form_header(&mut e);
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![
                LOAD_OVERRUN_FORM_FORMAT,
                1,
                SOURCE_FILE,
                0xb8f,
                0x0006_0001,
                0x0e01,
                0x2b,
                0x00ab_cdef
            ]]
        );
        let (mut e, _) = load_setup(0x43, true, &block_stream(b"KOLB", 3));
        form_header(&mut e);
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![
                LOAD_UNDERRUN_FORM_FORMAT,
                1,
                SOURCE_FILE,
                0xb8f,
                0x0006_0001,
                0x0e01,
                0x2b,
                0x00ab_cdef
            ]]
        );
    }

    #[test]
    fn load_game_reports_a_wrong_block_tag() {
        let (mut e, _) = load_setup(0x43, true, &block_stream(b"ABCD", 2));
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![BLOCK_HEADER_FORMAT, SOURCE_FILE, 0xb10, 0x43]]
        );
        let (mut e, _) = load_setup(0x43, true, &block_stream(b"ABCD", 2));
        form_header(&mut e);
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![
                BLOCK_HEADER_FORM_FORMAT,
                SOURCE_FILE,
                0xb10,
                0x0006_0001,
                0x0e01,
                0x2b,
                0x00ab_cdef
            ]]
        );
        // without save blocks nothing is read or checked
        let (mut e, buffer) = load_setup(0x43, false, b"ABCDEF");
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MESSAGE).is_empty());
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer);
    }

    #[test]
    fn load_game_takes_the_disabled_state_from_the_enable_parent() {
        let cases = [
            // (parent disabled, follows inverted, expected argument)
            (true, false, 1u32),
            (true, true, 0),
            (false, false, 0),
            (false, true, 1),
        ];
        for (parent_disabled, inverted, expected) in cases {
            let (mut e, _) = load_setup(0x43, false, &[]);
            returns(&mut e, SAVE_LOAD_UNAVAILABLE, 1);
            let parent = form(&mut e, 0x3a, 7, if parent_disabled { 0x800 } else { 0 });
            returns(&mut e, ENABLE_PARENT, parent);
            returns(&mut e, FOLLOWS_ENABLE_PARENT, inverted as u32);
            let refr = batch_refr(&mut e, false);
            e.call_log = Some(vec![]);
            e.call(0x0055_ec40, &args![refr, 0u32, 0u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, FORM_SET_DISABLED),
                vec![vec![refr.addr(), expected]],
                "parent disabled {parent_disabled}, inverted {inverted}"
            );
        }
        // no parent: the state is left alone
        let (mut e, _) = load_setup(0x43, false, &[]);
        returns(&mut e, SAVE_LOAD_UNAVAILABLE, 1);
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FORM_SET_DISABLED).is_empty());
    }

    #[test]
    fn load_game_flag_1_clears_the_state_of_a_disabled_or_deleted_reference() {
        for (form_flags, flags, expected) in [
            (0x800u32, 1u32, true),
            (0x20, 1, true),
            (0, 1, false),
            (0x800, 0, false),
        ] {
            let (mut e, _) = load_setup(0x43, false, &[]);
            slot(&mut e, 0x1cc, |_, _| Ret::default());
            let refr = batch_refr(&mut e, false);
            e.mem.set_u32(refr.addr() + 8, form_flags);
            e.call_log = Some(vec![]);
            e.call(0x0055_ec40, &args![refr, flags, 0u32]);
            let log = e.call_log.take().unwrap();
            let expected_calls = if expected {
                vec![vec![refr.addr(), 0, 1]]
            } else {
                vec![]
            };
            assert_eq!(calls_to(&log, 0x00fd_01cc), expected_calls);
        }
        // a non-actor that is not empty gets SetEmpty(true) on flag 0x200000
        let (mut e, _) = load_setup(0x43, false, &[]);
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0x20_0000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FORM_SET_EMPTY), vec![vec![refr.addr(), 1]]);
        let (mut e, _) = load_setup(0x43, false, &[]);
        let refr = batch_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0x20_0000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FORM_SET_EMPTY).is_empty());
    }

    #[test]
    fn load_game_flag_20_reloads_the_inventory_of_a_container() {
        // an actor with a container: drops the drawn weapon, clears the
        // inventory extra, tells the actor and loads the changes
        let (mut e, _) = load_setup(0x43, false, &[]);
        slot(&mut e, 0x21c, |_, _| true.into_ret());
        let refr = batch_refr(&mut e, true);
        with_base(&mut e, refr, 0x2a);
        returns(&mut e, GET_INVENTORY_CHANGES, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0x20u32, 0u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = addresses(&log)
            .into_iter()
            .filter(|a| {
                [
                    REMOVE_WEAPON,
                    FORM_PREPARE,
                    FORM_STEP,
                    0x0041_aeb0,
                    0x008a_dc50,
                    GET_INVENTORY_CHANGES,
                    INVENTORY_CHANGES_LOAD,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                REMOVE_WEAPON,
                FORM_PREPARE,
                FORM_STEP,
                FORM_STEP,
                0x0041_aeb0,
                0x008a_dc50,
                GET_INVENTORY_CHANGES,
                INVENTORY_CHANGES_LOAD
            ]
        );
        assert_eq!(calls_to(&log, FORM_PREPARE), vec![vec![refr.addr(), 1]]);
        assert_eq!(calls_to(&log, 0x008a_dc50), vec![vec![refr.addr()]]);
        assert_eq!(calls_to(&log, INVENTORY_CHANGES_LOAD), vec![vec![0x4444]]);
        // a plain container: no weapon, no actor call
        let (mut e, _) = load_setup(0x43, false, &[]);
        let refr = batch_refr(&mut e, false);
        with_base(&mut e, refr, 0x1b);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0x20u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, REMOVE_WEAPON).is_empty());
        assert!(calls_to(&log, 0x008a_dc50).is_empty());
        assert_eq!(calls_to(&log, 0x0041_aeb0), vec![vec![refr.addr() + 0x44]]);
        assert_eq!(calls_to(&log, INVENTORY_CHANGES_LOAD).len(), 1);
        // not a container at all: nothing
        let (mut e, _) = load_setup(0x43, false, &[]);
        let refr = batch_refr(&mut e, false);
        with_base(&mut e, refr, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0x20u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, INVENTORY_CHANGES_LOAD).is_empty());
    }

    #[test]
    fn load_game_reads_the_older_records_and_the_scale_by_flag() {
        let mut stream = vec![];
        stream.extend_from_slice(&7u16.to_le_bytes()); // 0x10000000 record size
        stream.extend_from_slice(&9u16.to_le_bytes()); // flag 4 record size
        stream.extend_from_slice(&2.5f32.to_le_bytes()); // the scale
        let (mut e, _) = load_setup(0x43, false, &stream);
        let refr = batch_refr(&mut e, false);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0x1000_0014u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOAD_OLD_RECORD_A),
            vec![vec![game, refr.addr(), 7]]
        );
        assert_eq!(
            calls_to(&log, LOAD_OLD_RECORD_B),
            vec![vec![game, refr.addr(), 9]]
        );
        assert_eq!(e.get(refr, TESObjectREFR::fRefScale), 2.5);
        assert_eq!(
            calls_to(&log, SET_SCALE),
            vec![vec![refr.addr(), 2.5f32.to_bits()]]
        );
        // an actor has no 0x10000000 record; zero sizes load nothing
        let (mut e, _) = load_setup(0x43, false, &[0, 0, 0, 0]);
        let refr = batch_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0x1000_0004u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, LOAD_OLD_RECORD_A).is_empty());
        assert!(calls_to(&log, LOAD_OLD_RECORD_B).is_empty());
        // before version 0x43 the scale is part of the extra data instead
        let (mut e, buffer) = load_setup(0x42, false, &[]);
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0x10u32, 0x3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, EXTRA_LOAD_GAME),
            vec![vec![refr.addr() + 0x44, 0x10, 3, refr.addr()]]
        );
        assert!(calls_to(&log, SET_SCALE).is_empty());
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer);
        // an actor always loads its extra data
        let (mut e, _) = load_setup(0x43, false, &[]);
        let refr = batch_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, EXTRA_LOAD_GAME).len(), 1);
    }

    #[test]
    fn load_game_updates_the_open_state_the_sequence_and_the_loaded_data() {
        let (mut e, _) = load_setup(0x43, false, &[]);
        returns(&mut e, EXTRA_FLAG_TEST, 1);
        stub(&mut e, &[EXTRA_FLAG_SET, EXTRA_FLAG_CLEAR]);
        let refr = batch_refr(&mut e, false);
        let loaded: Ptr<LOADED_REF_DATA> = e.new_object();
        e.mem.set_u32(loaded.addr(), 0x1234);
        e.mem.set_u32(loaded.addr() + 4, 3);
        e.mem.set_f32(loaded.addr() + 8, 99.0);
        e.set(refr, TESObjectREFR::pLoadedData, loaded.cast());
        e.set_global(LOADED_DATA_DEFAULT_HEIGHT, -2.5f32);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0xc0_0000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        let list = refr.addr() + 0x44;
        assert_eq!(calls_to(&log, EXTRA_FLAG_TEST), vec![vec![list, 8]]);
        assert_eq!(calls_to(&log, EXTRA_FLAG_SET), vec![vec![list, 8]]);
        assert!(calls_to(&log, EXTRA_FLAG_CLEAR).is_empty());
        assert_eq!(calls_to(&log, EXTRA_REMOVE_LAST_SEQUENCE), vec![vec![list]]);
        assert_eq!(
            e.get(loaded, LOADED_REF_DATA::pCurrentWaterObject),
            Ptr::NULL
        );
        assert_eq!(e.get(loaded, LOADED_REF_DATA::iUnderwaterCount), 0);
        assert_eq!(e.get(loaded, LOADED_REF_DATA::fRelevantWaterHeight), -2.5);
        // a clear bit clears it
        returns(&mut e, EXTRA_FLAG_TEST, 0);
        e.call_log = Some(vec![]);
        e.call(0x0055_ec40, &args![refr, 0x40_0000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, EXTRA_FLAG_CLEAR), vec![vec![list, 8]]);
    }

    // ---- 0055f240 --------------------------------------------------------

    /// The callees `fn_0055f240` and its friends leave to others.
    fn finish_setup() -> Engine {
        let mut e = batch_engine();
        set_up_stream(&mut e, 0x43, false);
        stub(
            &mut e,
            &[
                FORM_PREPARE,
                FORM_SET_EMPTY,
                SAVE_LOAD_UNAVAILABLE,
                REMOVE_WEAPON,
                FORM_STEP,
                RESET_STATE,
                IS_OPEN_CLOSE_FORM,
                SAVE_LOAD_TEST_632CE0,
                GET_OPEN_STATE,
                SET_OPEN_STATE,
                EXTRA_FLAG_TEST,
                EXTRA_FLAG_SET,
                EXTRA_FLAG_CLEAR,
                RESTORE_RAGDOLL_DATA,
                REFR_DISABLE_FIX,
                FORM_IS_KIND_477BA0,
                GET_LOADED_3D,
                EXTRA_GET_PERSISTENT_CELL,
                LOAD_FIRST_STEP,
                CHECKED_CAST,
                FOLLOWS_ENABLE_PARENT,
                FORM_SET_DISABLED,
                ENABLE_PARENT,
                0x0041_8460,
                0x0041_ae90,
                0x0042_2920,
                0x0041_cc60,
                0x0041_d930,
                0x0041_b060,
                0x0042_2720,
                0x0042_2670,
                EXTRA_REMOVE_TYPE,
                0x0041_9dc0,
                0x0041_aff0,
                0x0042_e040,
                0x0042_16b0,
                0x0041_de00,
            ],
        );
        e
    }

    #[test]
    fn fn_0055f240_starts_with_the_form_step_and_the_controller_data() {
        let mut e = finish_setup();
        let refr = batch_refr(&mut e, false);
        with_base(&mut e, refr, 0x10);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        e.call_log = Some(vec![]);
        e.call(0x0055_f240, &args![refr, 0x1020_0000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FORM_PREPARE),
            vec![vec![refr.addr(), 0x1020_0000]]
        );
        assert_eq!(calls_to(&log, FORM_SET_EMPTY), vec![vec![refr.addr(), 0]]);
        // the result of 0047c850 is asked for and ignored
        assert_eq!(calls_to(&log, SAVE_LOAD_UNAVAILABLE)[0], vec![game]);
        // flag 0x10000000 on a non-actor: the controller data (fn_0055f970)
        assert_eq!(calls_to(&log, GET_LOADED_3D)[0], vec![refr.addr()]);
        // an actor skips both
        let mut e = finish_setup();
        let refr = batch_refr(&mut e, true);
        with_base(&mut e, refr, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0055_f240, &args![refr, 0x1020_0000u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FORM_SET_EMPTY).is_empty());
        assert!(calls_to(&log, GET_LOADED_3D).is_empty());
    }

    #[test]
    fn fn_0055f240_flag_20_resets_after_a_container_change() {
        let mut e = finish_setup();
        slot(&mut e, 0x21c, |_, _| true.into_ret());
        slot(&mut e, 0x208, |_, _| Ret::default());
        let refr = batch_refr(&mut e, true);
        with_base(&mut e, refr, 0x10);
        // actor with a drawn weapon, and the stubbed 0047c850 says false:
        // the reset goes through 00574920
        e.call_log = Some(vec![]);
        e.call(0x0055_f240, &args![refr, 0x20u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, REMOVE_WEAPON), vec![vec![refr.addr()]]);
        assert_eq!(
            calls_to(&log, FORM_PREPARE),
            vec![vec![refr.addr(), 0x20], vec![refr.addr(), 1]]
        );
        assert_eq!(calls_to(&log, RESET_STATE), vec![vec![refr.addr(), 0]]);
        assert!(calls_to(&log, 0x00fd_0208).is_empty());
        // when it says true the virtual at +0x208 does it
        returns(&mut e, SAVE_LOAD_UNAVAILABLE, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_f240, &args![refr, 0x20u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, RESET_STATE).is_empty());
        assert_eq!(calls_to(&log, 0x00fd_0208), vec![vec![refr.addr(), 0]]);
    }

    #[test]
    fn fn_0055f240_sets_the_open_state_of_open_close_forms() {
        let mut e = finish_setup();
        returns(&mut e, IS_OPEN_CLOSE_FORM, 1);
        returns(&mut e, GET_OPEN_STATE, 3);
        returns(&mut e, EXTRA_FLAG_TEST, 1);
        let refr = batch_refr(&mut e, false);
        let base = with_base(&mut e, refr, 0x10);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        e.call_log = Some(vec![]);
        e.call(0x0055_f240, &args![refr, 0x40_0000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, IS_OPEN_CLOSE_FORM), vec![vec![base]]);
        assert_eq!(calls_to(&log, SAVE_LOAD_TEST_632CE0), vec![vec![game]]);
        assert_eq!(calls_to(&log, EXTRA_FLAG_SET).len(), 1);
        assert_eq!(calls_to(&log, GET_OPEN_STATE), vec![vec![refr.addr(), 8]]);
        assert_eq!(
            calls_to(&log, SET_OPEN_STATE),
            vec![vec![refr.addr(), 3, 1]]
        );
        // when 00632ce0 says true the flag is left alone; a revert in
        // progress (+0x48) skips the open state
        returns(&mut e, SAVE_LOAD_TEST_632CE0, 1);
        e.mem.set_u32(game + 0x48, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_f240, &args![refr, 0x40_0000u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, EXTRA_FLAG_SET).is_empty());
        assert!(calls_to(&log, SET_OPEN_STATE).is_empty());
    }

    #[test]
    fn fn_0055f240_ends_with_the_ragdoll_and_the_enable_fixes() {
        let mut e = finish_setup();
        slot(&mut e, 0x22c, |_, _| true.into_ret());
        let refr = batch_refr(&mut e, false);
        with_base(&mut e, refr, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0055_f240, &args![refr, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00fd_022c), vec![vec![refr.addr(), 0]]);
        assert_eq!(
            calls_to(&log, RESTORE_RAGDOLL_DATA),
            vec![vec![refr.addr(), 0]]
        );
        // form types 0x15, 0xd and 0x1c with a disabled reference
        for (kind, form_flags, kind_test, expected) in [
            (0x15u8, 0x800u32, false, true),
            (0xd, 0x20, false, true),
            (0x1c, 0, true, true),
            (0x1c, 0, false, false),
            (0x15, 0, true, false),
            (0x10, 0x800, true, false),
        ] {
            let mut e = finish_setup();
            returns(&mut e, FORM_IS_KIND_477BA0, kind_test as u32);
            let refr = batch_refr(&mut e, false);
            with_base(&mut e, refr, kind);
            e.mem.set_u32(refr.addr() + 8, form_flags);
            e.call_log = Some(vec![]);
            e.call(0x0055_f240, &args![refr, 0u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, REFR_DISABLE_FIX).len(),
                expected as usize,
                "kind {kind:#x} flags {form_flags:#x}"
            );
        }
        // a revert in progress skips all of it
        let mut e = finish_setup();
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        e.mem.set_u32(game + 0x48, 1);
        let refr = batch_refr(&mut e, false);
        with_base(&mut e, refr, 0x15);
        e.mem.set_u32(refr.addr() + 8, 0x800);
        e.call_log = Some(vec![]);
        e.call(0x0055_f240, &args![refr, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, REFR_DISABLE_FIX).is_empty());
    }

    // ---- 0055f5b0 ---------------------------------------------------------

    #[test]
    fn revert_state_zero_means_not_reverting() {
        let mut e = batch_engine();
        let game = e.mem.alloc(0x200);
        assert!(e.call(0x0055_f5b0, &args![game]).bool());
        e.mem.set_u32(game + 0x48, 2);
        assert!(!e.call(0x0055_f5b0, &args![game]).bool());
    }

    // ---- 0055f5f0 ---------------------------------------------------------

    #[test]
    fn fn_0055f5f0_follows_the_parent_and_handles_the_container_and_extras() {
        let mut e = finish_setup();
        stub(&mut e, &[FORM_SET_DISABLED, INVENTORY_CHANGES_STEP_A]);
        slot(&mut e, 0x1cc, |_, _| Ret::default());
        slot(&mut e, 0xfc, |_, _| Ret::default());
        stub(&mut e, &[EXTRA_AFTER_LOAD]);
        returns(&mut e, GET_INVENTORY_CHANGES, 0x4444);
        let parent = form(&mut e, 0x3a, 7, 0x800);
        returns(&mut e, ENABLE_PARENT, parent);
        let refr = batch_refr(&mut e, true);
        let base = with_base(&mut e, refr, 0x2a);
        e.call_log = Some(vec![]);
        e.call(0x0055_f5f0, &args![refr, 0x20u32, 0x5u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOAD_FIRST_STEP),
            vec![vec![refr.addr(), 0x20, 5]]
        );
        // the parent is disabled and the reference does not invert it
        assert_eq!(
            calls_to(&log, FORM_SET_DISABLED),
            vec![vec![refr.addr(), 1]]
        );
        // a parent makes the clear step run even without flag 1, but this
        // reference is neither disabled nor deleted
        assert!(calls_to(&log, 0x00fd_01cc).is_empty());
        assert_eq!(calls_to(&log, INVENTORY_CHANGES_STEP_A), vec![vec![0x4444]]);
        assert_eq!(calls_to(&log, 0x00fd_00fc), vec![vec![refr.addr()]]);
        assert_eq!(
            calls_to(&log, EXTRA_AFTER_LOAD),
            vec![vec![refr.addr() + 0x44, 0x20, 5, refr.addr(), base]]
        );
        // disabled and flag 1: the virtual at +0x1cc clears the state
        e.mem.set_u32(refr.addr() + 8, 0x800);
        e.call_log = Some(vec![]);
        e.call(0x0055_f5f0, &args![refr, 1u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00fd_01cc), vec![vec![refr.addr(), 0, 1]]);
        // no parent, no flag 1: nothing is cleared; a non-actor has no
        // extra-data step
        returns(&mut e, ENABLE_PARENT, 0);
        let plain = batch_refr(&mut e, false);
        e.mem.set_u32(plain.addr() + 8, 0x800);
        e.call_log = Some(vec![]);
        e.call(0x0055_f5f0, &args![plain, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x00fd_01cc).is_empty());
        assert!(calls_to(&log, EXTRA_AFTER_LOAD).is_empty());
    }

    // ---- 0055f780: FinishInitLoadGame --------------------------------------

    #[test]
    fn finish_init_load_game_moves_the_3d_to_the_reference() {
        let mut e = finish_setup();
        stub(
            &mut e,
            &[
                SET_3D_LOCATION,
                SET_3D_ORIENTATION,
                UPDATE_POSITION,
                MAKE_VELOCITY,
                SET_3D_VELOCITY,
                FINISH_INIT_LAST,
                INVENTORY_CHANGES_STEP_B,
            ],
        );
        returns(&mut e, GET_LOADED_3D, 0x3d3d);
        returns(&mut e, GET_ORIENTATION, 0x0a0a);
        returns(&mut e, GET_INVENTORY_CHANGES, 0x4444);
        let refr = batch_refr(&mut e, false);
        with_base(&mut e, refr, 0x1b);
        e.call_log = Some(vec![]);
        e.call(0x0055_f780, &args![refr, 0x20u32, 0x2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOAD_FIRST_STEP),
            vec![vec![refr.addr(), 0x20, 2]]
        );
        assert_eq!(calls_to(&log, INVENTORY_CHANGES_STEP_B), vec![vec![0x4444]]);
        // the location (virtual +0x1f4: this + 0x30), then the orientation
        assert_eq!(
            calls_to(&log, SET_3D_LOCATION),
            vec![vec![0x3d3d, refr.addr() + 0x30]]
        );
        assert_eq!(calls_to(&log, GET_ORIENTATION).len(), 1);
        assert_eq!(calls_to(&log, GET_ORIENTATION)[0][0], refr.addr());
        assert_eq!(
            calls_to(&log, SET_3D_ORIENTATION),
            vec![vec![0x3d3d, 0x0a0a]]
        );
        assert_eq!(calls_to(&log, UPDATE_POSITION), vec![vec![0x3d3d, 1, 0]]);
        // a zero velocity record: (record, 0.0, 0, 0)
        let made = calls_to(&log, MAKE_VELOCITY);
        assert_eq!(made.len(), 1);
        assert_eq!(&made[0][1..], &[0.0f32.to_bits(), 0, 0]);
        assert_eq!(
            calls_to(&log, SET_3D_VELOCITY),
            vec![vec![0x3d3d, made[0][0]]]
        );
        assert_eq!(calls_to(&log, FINISH_INIT_LAST), vec![vec![refr.addr(), 0]]);
        // without a 3D only the ends run
        returns(&mut e, GET_LOADED_3D, 0);
        e.call_log = Some(vec![]);
        e.call(0x0055_f780, &args![refr, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SET_3D_LOCATION).is_empty());
        assert_eq!(calls_to(&log, FINISH_INIT_LAST).len(), 1);
    }

    #[test]
    fn finish_init_load_game_gates_the_extra_data_handler_on_actors() {
        let mut e = finish_setup();
        stub(&mut e, &[FINISH_INIT_LAST, 0x0041_8460, 0x0048_3710]);
        let refr = batch_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0055_f780, &args![refr, 0x2_0000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        // the handler (fn_00426720) does nothing for an actor target
        assert!(calls_to(&log, 0x0041_8460).is_empty());
        // and is not reached at all for a non-actor
        let plain = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0055_f780, &args![plain, 0x2_0000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0041_8460).is_empty());
        assert_eq!(
            calls_to(&log, FINISH_INIT_LAST),
            vec![vec![plain.addr(), 0]]
        );
    }

    // ---- sequences ----------------------------------------------------------

    /// Doubles for the accessors of sequences and their manager: a fake
    /// sequence keeps its name pointer at +8, the "generic" word at +0x20,
    /// the save size at +0x24, the offset time at +0x28 and the duration at
    /// +0x2c; a fake manager keeps the count at +0xc and the sequences from
    /// +0x10.
    fn sequence_api(e: &mut Engine) {
        e.register(MANAGER_HAS_SEQUENCES, |_, _| true.into_ret());
        e.register(MANAGER_SEQUENCE_COUNT, |e, a| {
            e.mem.u32(a[0] + 0xc).into_ret()
        });
        e.register(MANAGER_SEQUENCE_AT, |e, a| {
            e.mem.u32(a[0] + 0x10 + 4 * a[1]).into_ret()
        });
        e.register(SEQUENCE_IS_GENERIC, |e, a| {
            e.mem.u32(a[0] + 0x20).into_ret()
        });
        e.register(SEQUENCE_NAME_HOLDER, |_, a| (a[0] + 8).into_ret());
        e.register(NAME_TEXT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STRING_LENGTH, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(SEQUENCE_SAVE_SIZE, |e, a| {
            (e.mem.u32(a[0] + 0x24) as u16).into_ret()
        });
        e.register(SEQUENCE_OFFSET_TIME, |e, a| {
            e.mem.f32(a[0] + 0x28).into_ret()
        });
        e.register(SEQUENCE_DURATION, |e, a| e.mem.f32(a[0] + 0x2c).into_ret());
        e.register(STRING_COMPARE, |e, a| {
            u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])).into_ret()
        });
    }

    fn fake_sequence(e: &mut Engine, name: &str, generic: u32, save_size: u32) -> u32 {
        let sequence = e.mem.alloc(0x40);
        let text = e.mem.alloc(name.len() as u32 + 1);
        e.mem.set_cstr(text, name.as_bytes());
        e.mem.set_u32(sequence + 8, text);
        e.mem.set_u32(sequence + 0x20, generic);
        e.mem.set_u32(sequence + 0x24, save_size);
        sequence
    }

    fn fake_manager(e: &mut Engine, sequences: &[u32]) -> u32 {
        let manager = e.mem.alloc(0x40);
        e.mem.set_u32(manager + 0xc, sequences.len() as u32);
        for (i, sequence) in sequences.iter().enumerate() {
            e.mem.set_u32(manager + 0x10 + 4 * i as u32, *sequence);
        }
        manager
    }

    /// A reference whose loaded 3D is `node`, whose first controller is
    /// `0xc0c0` and which casts to `manager`.
    fn animated_refr(e: &mut Engine, manager: u32) -> (Ptr<TESObjectREFR>, u32) {
        let node = e.mem.alloc(0x20);
        returns(e, GET_LOADED_3D, node);
        returns(e, GET_CONTROLLER, 0xc0c0);
        e.register_double(CHECKED_CAST, move |_, a| {
            if a[1] == 0 {
                0u32.into_ret()
            } else {
                manager.into_ret()
            }
        });
        (batch_refr(e, false), node)
    }

    // ---- 0055f880 / 0055f900 ----------------------------------------------

    #[test]
    fn fn_0055f880_is_the_size_of_the_managers_sequences() {
        let mut e = batch_engine();
        set_up_stream(&mut e, 0x43, false);
        sequence_api(&mut e);
        let a = fake_sequence(&mut e, "ab", 1, 5);
        let skipped = fake_sequence(&mut e, "no", 0, 99);
        let b = fake_sequence(&mut e, "xyz", 2, 7);
        let manager = fake_manager(&mut e, &[a, skipped, b]);
        let (refr, node) = animated_refr(&mut e, manager);
        e.call_log = Some(vec![]);
        // 2, then (1 + 2 + 5) and (1 + 3 + 7)
        assert_eq!(e.call(0x0055_f880, &args![refr]).u16(), 21);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CHECKED_CAST),
            vec![vec![MANAGER_TYPE, 0xc0c0]]
        );
        assert_eq!(calls_to(&log, GET_CONTROLLER)[0], vec![node]);
        // no controller, no manager: only the count
        returns(&mut e, GET_CONTROLLER, 0);
        assert_eq!(e.call(0x0055_f880, &args![refr]).u16(), 2);
        returns(&mut e, GET_LOADED_3D, 0);
        assert_eq!(e.call(0x0055_f880, &args![refr]).u16(), 2);
    }

    #[test]
    fn fn_0055f900_saves_the_sequences_with_the_default_blend_time() {
        let mut e = batch_engine();
        let buffer = set_up_stream(&mut e, 0x43, false);
        sequence_api(&mut e);
        e.register(SEQUENCE_SAVE, |_, _| Ret::default());
        e.set_global(DEFAULT_BLEND_TIME, 0.75f32);
        let a = fake_sequence(&mut e, "ab", 1, 5);
        let manager = fake_manager(&mut e, &[a]);
        let (refr, _) = animated_refr(&mut e, manager);
        e.call_log = Some(vec![]);
        e.call(0x0055_f900, &args![refr]);
        let log = e.call_log.take().unwrap();
        // a count of 1, then the name with its length byte
        assert_eq!(e.mem.bytes(buffer, 5), [1, 0, 2, b'a', b'b']);
        assert_eq!(
            calls_to(&log, SEQUENCE_SAVE),
            vec![vec![a, 0.75f32.to_bits()]]
        );
    }

    // ---- 0055fdb0 / 0055fe70 ----------------------------------------------

    #[test]
    fn fn_0055fdb0_adds_up_the_named_sequences() {
        let mut e = batch_engine();
        sequence_api(&mut e);
        let a = fake_sequence(&mut e, "ab", 1, 5);
        let skipped = fake_sequence(&mut e, "no", 0, 99);
        let b = fake_sequence(&mut e, "xyz", 2, 7);
        let manager = fake_manager(&mut e, &[a, skipped, b, 0]);
        assert_eq!(e.call(0x0055_fdb0, &args![manager]).u16(), 21);
        assert_eq!(e.call(0x0055_fdb0, &args![0u32]).u16(), 2);
        returns(&mut e, MANAGER_HAS_SEQUENCES, 0);
        assert_eq!(e.call(0x0055_fdb0, &args![manager]).u16(), 2);
        // the sum wraps at 16 bits
        returns(&mut e, MANAGER_HAS_SEQUENCES, 1);
        e.mem.set_u32(a + 0x24, 0xffff);
        assert_eq!(e.call(0x0055_fdb0, &args![manager]).u16(), 15);
    }

    #[test]
    fn fn_0055fe70_writes_count_names_and_sequence_data() {
        let mut e = batch_engine();
        let buffer = set_up_stream(&mut e, 0x43, false);
        sequence_api(&mut e);
        e.register(SEQUENCE_SAVE, |_, _| Ret::default());
        e.set_global(DEFAULT_BLEND_TIME, 0.75f32);
        let a = fake_sequence(&mut e, "ab", 1, 5);
        let skipped = fake_sequence(&mut e, "no", 0, 99);
        let b = fake_sequence(&mut e, "xyz", 2, 7);
        let manager = fake_manager(&mut e, &[a, skipped, b]);
        e.call_log = Some(vec![]);
        e.call(0x0055_fe70, &args![manager, 0.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            e.mem.bytes(buffer, 11),
            [2, 0, 2, b'a', b'b', 3, b'x', b'y', b'z', 0, 0]
        );
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 9);
        assert_eq!(
            calls_to(&log, SEQUENCE_SAVE),
            vec![vec![a, 0.5f32.to_bits()], vec![b, 0.5f32.to_bits()]]
        );
        // -1.0 stands for the default blend time
        e.mem.set_u32(game + 0x14, buffer);
        e.call_log = Some(vec![]);
        e.call(0x0055_fe70, &args![manager, -1.0f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SEQUENCE_SAVE)[0], vec![a, 0.75f32.to_bits()]);
        // no manager: just a zero count
        e.mem.set_u32(game + 0x14, buffer);
        e.mem.write(buffer, &[9, 9, 9]);
        e.call(0x0055_fe70, &args![0u32, 1.0f32]);
        assert_eq!(e.mem.bytes(buffer, 3), [0, 0, 9]);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 2);
    }

    // ---- 0055ffa0 ---------------------------------------------------------

    /// A load setup for `fn_0055ffa0`: `stream` is the saved data.
    fn sequences_load_setup(version: u8, stream: &[u8]) -> (Engine, u32, u32, u32) {
        let mut e = batch_engine();
        let buffer = set_up_stream(&mut e, version, false);
        e.mem.write(buffer, stream);
        sequence_api(&mut e);
        stub(
            &mut e,
            &[
                MANAGER_SET_FLAG,
                MANAGER_ACTIVATE,
                SEQUENCE_LOAD,
                ADD_3D_VELOCITY,
                SET_3D_VELOCITY,
            ],
        );
        e.register(MAKE_VELOCITY, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u8(a[0] + 4, a[2] as u8);
            e.mem.set_u8(a[0] + 5, a[3] as u8);
            a[0].into_ret()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(0x0040_6d30, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        returns(&mut e, EMPTY_SEQUENCE_SIZE, 5);
        let first = fake_sequence(&mut e, "ab", 0, 0);
        e.mem.set_f32(first + 0x2c, 2.0);
        let second = fake_sequence(&mut e, "xyz", 1, 0);
        let manager = fake_manager(&mut e, &[first, second]);
        (e, buffer, manager, first)
    }

    fn entry_stream(names: &[&str]) -> Vec<u8> {
        let mut stream = (names.len() as u16).to_le_bytes().to_vec();
        for name in names {
            stream.push(name.len() as u8);
            stream.extend_from_slice(name.as_bytes());
        }
        stream
    }

    #[test]
    fn fn_0055ffa0_loads_the_named_sequences_and_skips_the_unknown_ones() {
        // three entries; the unknown one is followed by 5 bytes of data that
        // the load skips (the known ones are read by the sequences themselves)
        let mut stream = 3u16.to_le_bytes().to_vec();
        stream.extend_from_slice(&[2, b'a', b'b']);
        stream.extend_from_slice(&[4, b'z', b'z', b'z', b'z', 1, 2, 3, 4, 5]);
        stream.extend_from_slice(&[3, b'x', b'y', b'z']);
        let (mut e, buffer, manager, first) = sequences_load_setup(0x43, &stream);
        let second = e.mem.u32(manager + 0x14);

        let total = stream.len() as u32;
        e.call_log = Some(vec![]);
        e.call(0x0055_ffa0, &args![manager, 0u32, 1.0f32]);
        let log = e.call_log.take().unwrap();
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + total);
        // the manager is switched on for entries; a non-generic sequence is
        // activated first; both are loaded with the blend time
        assert_eq!(calls_to(&log, MANAGER_SET_FLAG), vec![vec![manager, 1]]);
        assert_eq!(
            calls_to(&log, MANAGER_ACTIVATE),
            vec![vec![
                manager,
                first,
                0,
                0,
                1.0f32.to_bits(),
                0.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(
            calls_to(&log, SEQUENCE_LOAD),
            vec![
                vec![first, 1.0f32.to_bits()],
                vec![second, 1.0f32.to_bits()]
            ]
        );
        assert_eq!(calls_to(&log, EMPTY_SEQUENCE_SIZE).len(), 1);
        // no object: no velocity records
        assert!(calls_to(&log, ADD_3D_VELOCITY).is_empty());
        assert!(calls_to(&log, SET_3D_VELOCITY).is_empty());
    }

    #[test]
    fn fn_0055ffa0_gives_the_object_velocity_records_over_the_duration() {
        let stream = entry_stream(&["ab"]);
        let (mut e, buffer, manager, _) = sequences_load_setup(0x43, &stream);
        let object = e.mem.alloc(0x20);
        returns(&mut e, IS_ACTOR_3D, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_ffa0, &args![manager, object, 1.0f32]);
        let log = e.call_log.take().unwrap();
        // total = 2 + 1; start = max(1 - 3, 0) = 0; step = 3 / 20 = 0.15:
        // records at 0, 0.15, ... 0.9
        let made = calls_to(&log, MAKE_VELOCITY);
        let times: Vec<f32> = made.iter().map(|m| f32::from_bits(m[1])).collect();
        assert_eq!(made.len(), 8);
        assert_eq!(times[0], 0.0);
        assert!((times[6] - 0.9).abs() < 1e-5);
        assert_eq!(calls_to(&log, ADD_3D_VELOCITY).len(), 7);
        for record in &made[..7] {
            assert_eq!(&record[2..], &[0, 0]);
        }
        // and a last record with the blend time, set on the object
        assert_eq!(&made[7][1..], &[1.0f32.to_bits(), 1, 0]);
        assert_eq!(
            calls_to(&log, SET_3D_VELOCITY),
            vec![vec![object, made[7][0]]]
        );
        // a short sequence uses the smallest step
        let second = e.mem.u32(manager + 0x10);
        e.mem.set_f32(second + 0x2c, 0.0);
        e.mem
            .set_u32(e.global::<u32>(GLOBAL_SAVE_LOAD) + 0x14, buffer);
        e.call_log = Some(vec![]);
        e.call(0x0055_ffa0, &args![manager, object, 0.09f32]);
        let log = e.call_log.take().unwrap();
        let made = calls_to(&log, MAKE_VELOCITY);
        assert_eq!(made.len(), 7);
        assert_eq!(f32::from_bits(made[1][1]), 0.0166666f32);
    }

    #[test]
    fn fn_0055ffa0_reads_old_saves_names_from_the_table() {
        // versions 0x15 and 0x16 save an index into the table at 011977d8
        // (stride 0x24, a name pointer first)
        let mut stream = 1u16.to_le_bytes().to_vec();
        stream.extend_from_slice(&1i32.to_le_bytes());
        let (mut e, buffer, manager, first) = sequences_load_setup(0x16, &stream);
        let name = e.mem.u32(first + 8);
        e.mem.set_u32(NAME_TABLE + 0x24, name);
        e.call_log = Some(vec![]);
        e.call(0x0055_ffa0, &args![manager, 0u32, 0.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SEQUENCE_LOAD),
            vec![vec![first, 0.5f32.to_bits()]]
        );
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 6);
        // an index past the table gives an empty name: no sequence matches
        let mut stream = 1u16.to_le_bytes().to_vec();
        stream.extend_from_slice(&0xf5i32.to_le_bytes());
        let (mut e, buffer, manager, _) = sequences_load_setup(0x15, &stream);
        e.call_log = Some(vec![]);
        e.call(0x0055_ffa0, &args![manager, 0u32, 0.5f32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SEQUENCE_LOAD).is_empty());
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 6 + 5);
    }

    #[test]
    fn fn_0055ffa0_ignores_an_absurd_count_and_uses_the_default_blend() {
        let mut stream = 65001u16.to_le_bytes().to_vec();
        stream.extend_from_slice(&[0; 8]);
        let (mut e, buffer, manager, _) = sequences_load_setup(0x43, &stream);
        e.call_log = Some(vec![]);
        e.call(0x0055_ffa0, &args![manager, 0u32, 0.5f32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MANAGER_SET_FLAG).is_empty());
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 2);
        // -1.0 is the default blend time
        e.set_global(DEFAULT_BLEND_TIME, 0.25f32);
        let stream = entry_stream(&["ab"]);
        let (mut e, _, manager, first) = sequences_load_setup(0x43, &stream);
        e.set_global(DEFAULT_BLEND_TIME, 0.25f32);
        e.call_log = Some(vec![]);
        e.call(0x0055_ffa0, &args![manager, 0u32, -1.0f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SEQUENCE_LOAD),
            vec![vec![first, 0.25f32.to_bits()]]
        );
    }

    /// The default constructors the compiler calls on stack vectors and
    /// quaternions (`006815c0`, `006240d0`) just return their `this`.
    fn identity_constructors(e: &mut Engine) {
        e.register(VECTOR_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(QUATERNION_CONSTRUCT, |_, a| a[0].into_ret());
    }
    // ---- 0055f970 ---------------------------------------------------------

    /// The doubles `fn_0055f970` needs on top of the sequence ones; the
    /// names the exe keeps for the sequences it looks up are "Unequip",
    /// "IdleA" and "IdleB".
    fn rest_setup(
        last_finished: Option<&str>,
        sequences: &[(&str, u32)],
    ) -> (Engine, Ptr<TESObjectREFR>, u32, u32, Vec<u32>) {
        let mut e = batch_engine();
        set_up_stream(&mut e, 0x43, false);
        sequence_api(&mut e);
        stub(
            &mut e,
            &[
                MANAGER_DEACTIVATE_ALL,
                MANAGER_SET_FLAG,
                MANAGER_ACTIVATE,
                SEQUENCE_SET_OFFSET,
                SET_3D_VELOCITY,
                FIXED_STRING_DESTRUCT,
                HAS_MORPHER_CONTROLLER,
            ],
        );
        e.register(MAKE_VELOCITY, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(MANAGER_FIND_SEQUENCE, |e, a| {
            let name = e.mem.cstr(e.mem.u32(a[1]));
            for i in 0..e.mem.u32(a[0] + 0xc) {
                let sequence = e.mem.u32(a[0] + 0x10 + 4 * i);
                if e.mem.cstr(e.mem.u32(sequence + 8)) == name {
                    return sequence.into_ret();
                }
            }
            0u32.into_ret()
        });
        e.register(SEQUENCE_NAME_COMPARE, |e, a| {
            u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])).into_ret()
        });
        for (global, text) in [
            (NAME_UNEQUIP, "Unequip"),
            (NAME_SEQUENCE_A, "IdleA"),
            (NAME_SEQUENCE_B, "IdleB"),
        ] {
            let pointer = e.mem.alloc(16);
            e.mem.set_cstr(pointer, text.as_bytes());
            e.set_global(global, pointer);
        }
        let last = last_finished.map(|name| {
            let pointer = e.mem.alloc(16);
            e.mem.set_cstr(pointer, name.as_bytes());
            pointer
        });
        returns(&mut e, EXTRA_GET_LAST_SEQUENCE, last.unwrap_or(0));
        let made: Vec<u32> = sequences
            .iter()
            .map(|(name, generic)| fake_sequence(&mut e, name, *generic, 0))
            .collect();
        for (i, sequence) in made.iter().enumerate() {
            e.mem.set_f32(sequence + 0x28, 0.5 + i as f32);
        }
        let manager = fake_manager(&mut e, &made);
        let (refr, node) = animated_refr(&mut e, manager);
        // the base object's virtual at +0x178 gives the new location
        e.register(0x00fc_0178, |_, _| 0x1234u32.into_ret());
        let base = object_with_vtable(&mut e, 0x40, &[(0x178, 0x00fc_0178)]);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(base),
        );
        (e, refr, manager, node, made)
    }

    #[test]
    fn fn_0055f970_resets_the_location_after_an_unequip_sequence() {
        let (mut e, refr, _, _, _) = rest_setup(Some("Unequip"), &[]);
        e.call_log = Some(vec![]);
        e.call(0x0055_f970, &args![refr]);
        let log = e.call_log.take().unwrap();
        let base = e.get(refr.at(TESObjectREFR::data), OBJ_REFR::pObjectReference);
        assert_eq!(
            calls_to(&log, 0x00fc_0178),
            vec![vec![base.addr(), refr.addr()]]
        );
        assert_eq!(
            calls_to(&log, 0x00fd_01cc),
            vec![vec![refr.addr(), 0x1234, 1]]
        );
        assert_eq!(calls_to(&log, 0x00fd_01c4), vec![vec![refr.addr()]]);
        // another last sequence does not
        let (mut e, refr, _, _, _) = rest_setup(Some("Walk"), &[]);
        e.call_log = Some(vec![]);
        e.call(0x0055_f970, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x00fd_01c4).is_empty());
    }

    #[test]
    fn fn_0055f970_resets_the_location_for_a_generic_unequip_or_a_morpher() {
        // a sequence named "Unequip" that is a generic-location one
        let (mut e, refr, manager, _, _) = rest_setup(None, &[("Unequip", 1)]);
        e.call_log = Some(vec![]);
        e.call(0x0055_f970, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00fd_01c4).len(), 1);
        assert_eq!(calls_to(&log, MANAGER_FIND_SEQUENCE)[0][0], manager);
        // one that is not: no reset
        let (mut e, refr, _, _, _) = rest_setup(None, &[("Unequip", 0)]);
        e.call_log = Some(vec![]);
        e.call(0x0055_f970, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x00fd_01c4).is_empty());
        // a morpher controller on the 3D
        let (mut e, refr, _, node, _) = rest_setup(None, &[]);
        returns(&mut e, HAS_MORPHER_CONTROLLER, 1);
        e.call_log = Some(vec![]);
        e.call(0x0055_f970, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, HAS_MORPHER_CONTROLLER), vec![vec![node]]);
        assert_eq!(calls_to(&log, 0x00fd_01c4).len(), 1);
    }

    #[test]
    fn fn_0055f970_restarts_the_two_idle_sequences_it_finds() {
        let (mut e, refr, manager, node, made) = rest_setup(None, &[("IdleA", 0), ("IdleB", 1)]);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.call_log = Some(vec![]);
        e.call(0x0055_f970, &args![refr]);
        let log = e.call_log.take().unwrap();
        // the manager is deactivated once found and once at the end; the
        // flag is set once
        assert_eq!(
            calls_to(&log, MANAGER_DEACTIVATE_ALL),
            vec![vec![manager, 0], vec![manager, 0]]
        );
        assert_eq!(calls_to(&log, MANAGER_SET_FLAG), vec![vec![manager, 1]]);
        // the non-generic sequence is activated; both get -FLT_MAX as offset
        assert_eq!(
            calls_to(&log, MANAGER_ACTIVATE),
            vec![vec![
                manager,
                made[0],
                0,
                0,
                1.0f32.to_bits(),
                0.0f32.to_bits(),
                0
            ]]
        );
        let minimum = (-f32::MAX).to_bits();
        assert_eq!(
            calls_to(&log, SEQUENCE_SET_OFFSET),
            vec![vec![made[0], minimum], vec![made[1], minimum]]
        );
        // each gives the 3D a velocity record from its offset time
        let made_records = calls_to(&log, MAKE_VELOCITY);
        assert_eq!(made_records.len(), 2);
        assert_eq!(&made_records[0][1..], &[0.5f32.to_bits(), 1, 0]);
        assert_eq!(&made_records[1][1..], &[1.5f32.to_bits(), 1, 0]);
        assert_eq!(
            calls_to(&log, SET_3D_VELOCITY),
            vec![
                vec![node, made_records[0][0]],
                vec![node, made_records[1][0]]
            ]
        );
    }

    #[test]
    fn fn_0055f970_falls_back_to_the_first_sequence_and_ends_by_deactivating() {
        let (mut e, refr, manager, node, made) = rest_setup(None, &[("Other", 1)]);
        e.call_log = Some(vec![]);
        e.call(0x0055_f970, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MANAGER_SET_FLAG),
            vec![vec![manager, 1], vec![manager, 0]]
        );
        assert_eq!(calls_to(&log, MANAGER_SEQUENCE_AT), vec![vec![manager, 0]]);
        // the fallback activates the sequence whatever its kind
        assert_eq!(calls_to(&log, MANAGER_ACTIVATE).len(), 1);
        assert_eq!(calls_to(&log, MANAGER_ACTIVATE)[0][1], made[0]);
        assert_eq!(calls_to(&log, SET_3D_VELOCITY).len(), 1);
        assert_eq!(calls_to(&log, SET_3D_VELOCITY)[0][0], node);
        // deactivated after the lookup, in the fallback, and at the end
        assert_eq!(calls_to(&log, MANAGER_DEACTIVATE_ALL).len(), 3);
        // without a 3D: only the final cast of the missing controller
        let (mut e, refr, _, _, _) = rest_setup(None, &[]);
        returns(&mut e, GET_LOADED_3D, 0);
        e.call_log = Some(vec![]);
        e.call(0x0055_f970, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MANAGER_DEACTIVATE_ALL).is_empty());
        assert_eq!(calls_to(&log, CHECKED_CAST), vec![vec![MANAGER_TYPE, 0]]);
    }

    // ---- the Havok record ---------------------------------------------------

    /// A stand-in for a collision node: its object at +8, its body
    /// reference at +0x10 (a rigid body whose entity, at +4, holds the
    /// "active" byte), and the flag bytes the type tests read at +0x1e
    /// (skipped type) and +0x1f (weapon type).
    fn havok_node(e: &mut Engine, object: u32, body: Option<bool>, weapon: bool) -> u32 {
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node + 8, object);
        e.mem.set_u8(node + 0x1f, weapon as u8);
        if let Some(active) = body {
            let entity = e.mem.alloc(0x40);
            e.mem.set_u8(entity, active as u8);
            let rigid_body = e.mem.alloc(0x40);
            e.mem.set_u32(rigid_body + 4, entity);
            e.mem.set_u32(node + 0x10, rigid_body);
        }
        node
    }

    fn havok_engine() -> Engine {
        let mut e = batch_engine();
        set_up_stream(&mut e, 0x51, false);
        sequence_api(&mut e);
        e.map(0x0102_0000, 0x1000);
        e.mem.set_cstr(ARROW, b"Arrow");
        e.register(NODE_OBJECT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(NODE_BODY_REF, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(IS_OF_TYPE, |e, a| {
            let offset = match a[0] {
                WEAPON_NODE_TYPE => 0x1f,
                SKIPPED_NODE_TYPE => 0x1e,
                _ => return false.into_ret(),
            };
            (a[1] != 0 && e.mem.u8(a[1] + offset) != 0).into_ret()
        });
        e.register(CHECKED_CAST, |_, a| a[1].into_ret());
        e.register(BODY_ENTITY, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ENTITY_ACTIVE_FLAG, |e, a| {
            let flag = e.mem.u8(a[0]);
            e.mem.set_u8(a[1], flag);
            a[1].into_ret()
        });
        e
    }

    /// The collision walker double: calls the callback (its address is the
    /// third argument) for each node, with the walk context.
    fn install_walk(e: &mut Engine, nodes: Vec<u32>) {
        e.register_double(WALK_COLLISION, move |e, a| {
            for node in &nodes {
                e.call(a[2], &[*node, a[1]]);
            }
            Ret::default()
        });
    }

    #[test]
    fn fn_00560350_counts_the_bodies_and_sizes_the_record() {
        let mut e = havok_engine();
        let root = havok_node(&mut e, 0, Some(true), false);
        let first = havok_node(&mut e, 0, Some(true), false);
        let second = havok_node(&mut e, 0, Some(false), false);
        let weapon = havok_node(&mut e, 0, Some(false), true);
        let no_body = havok_node(&mut e, 0, None, false);
        let arrow_object = fake_sequence(&mut e, "Arrow", 0, 0);
        let arrow = havok_node(&mut e, arrow_object, Some(true), false);
        install_walk(&mut e, vec![root, first, second, weapon, no_body, arrow]);
        returns(&mut e, COLLISION_ROOT, root);
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        let refr = batch_refr(&mut e, false);
        let record: Ptr<HavokSaveData> = e.new_object();
        e.call_log = Some(vec![]);
        let total = e.call(0x0056_0350, &args![refr, record]).u16();
        let log = e.call_log.take().unwrap();
        // active: root and first; inactive: second and the weapon node
        assert_eq!(e.get(record, HavokSaveData::sActiveBoneCount), 2);
        assert_eq!(e.get(record, HavokSaveData::sInactiveBoneCount), 2);
        // 8 weapon, 2 root, 4 both kinds
        assert_eq!(e.get(record, HavokSaveData::cFlags), 0xe);
        assert_eq!(e.get(record, HavokSaveData::pRef), refr.cast());
        assert_eq!(e.get(record, HavokSaveData::pObj3D), Ptr::new(obj3d));
        assert_eq!(e.get(record, HavokSaveData::pCollisionRoot), Ptr::new(root));
        // 3 + 4 bodies + 2 active * 0x18 + (4 - 1 for the root) * 0x1c
        assert_eq!(total, 3 + 4 + 2 * 0x18 + 3 * 0x1c);
        let walks = calls_to(&log, WALK_COLLISION);
        assert_eq!(walks.len(), 1);
        assert_eq!(walks[0][0], obj3d);
        assert_eq!(walks[0][2], 0x0056_0870);
        let walk = walks[0][1];
        assert_eq!(e.mem.u8(walk + 4), 1);
        assert_eq!(e.mem.u32(walk + 8), 0x12);
        assert_eq!(e.mem.u32(walk + 0xc), record.addr());
        // without an output record the size is the same
        assert_eq!(e.call(0x0056_0350, &args![refr, 0u32]).u16(), total);
    }

    #[test]
    fn fn_00560350_with_only_active_bodies_sets_flag_1() {
        let mut e = havok_engine();
        let root = havok_node(&mut e, 0, Some(true), false);
        let first = havok_node(&mut e, 0, Some(true), false);
        install_walk(&mut e, vec![root, first]);
        returns(&mut e, COLLISION_ROOT, root);
        returns(&mut e, GET_LOADED_3D, 0x3d3d);
        let refr = batch_refr(&mut e, false);
        let record: Ptr<HavokSaveData> = e.new_object();
        let total = e.call(0x0056_0350, &args![refr, record]).u16();
        assert_eq!(e.get(record, HavokSaveData::cFlags), 3);
        assert_eq!(total, 3 + 2 * 0x18 + 0x1c);
        // no bodies at all: just the 3
        install_walk(&mut e, vec![]);
        let record: Ptr<HavokSaveData> = e.new_object();
        assert_eq!(e.call(0x0056_0350, &args![refr, record]).u16(), 3);
        assert_eq!(e.get(record, HavokSaveData::cFlags), 0);
        // no 3D: 0, and the record is left alone
        returns(&mut e, GET_LOADED_3D, 0);
        let record: Ptr<HavokSaveData> = e.new_object();
        e.mem.set_u8(record.addr(), 0x77);
        assert_eq!(e.call(0x0056_0350, &args![refr, record]).u16(), 0);
        assert_eq!(e.mem.u8(record.addr()), 0x77);
    }

    #[test]
    fn fn_00560870_skips_arrows_and_bodies_of_the_skipped_type_for_actors() {
        let mut e = havok_engine();
        let root = havok_node(&mut e, 0, Some(true), false);
        returns(&mut e, COLLISION_ROOT, root);
        let refr = batch_refr(&mut e, true);
        let record: Ptr<HavokSaveData> = e.new_object();
        e.set(record, HavokSaveData::pRef, refr.cast());
        e.set(record, HavokSaveData::pCollisionRoot, Ptr::new(root));
        let walk: Ptr<CollisionWalkContext> = e.new_object();
        e.set(walk, CollisionWalkContext::pUserData, record.cast());
        // an arrow object: skipped even though it has a body
        let arrow_object = fake_sequence(&mut e, "Arrow", 0, 0);
        let arrow = havok_node(&mut e, arrow_object, Some(true), false);
        e.call(0x0056_0870, &args![arrow, walk]);
        assert_eq!(e.get(record, HavokSaveData::sActiveBoneCount), 0);
        // the collision root is never skipped for being an arrow
        e.mem.set_u32(root + 8, arrow_object);
        e.call(0x0056_0870, &args![root, walk]);
        assert_eq!(e.get(record, HavokSaveData::sActiveBoneCount), 1);
        assert_eq!(e.get(record, HavokSaveData::cFlags), 2);
        // an object of the skipped type, for an actor reference
        let object = e.mem.alloc(0x40);
        e.mem.set_u8(object + 0x1e, 1);
        let typed = havok_node(&mut e, object, Some(false), false);
        e.call(0x0056_0870, &args![typed, walk]);
        assert_eq!(e.get(record, HavokSaveData::sInactiveBoneCount), 0);
        // but not for a non-actor
        batch_refr(&mut e, false);
        e.call(0x0056_0870, &args![typed, walk]);
        assert_eq!(e.get(record, HavokSaveData::sInactiveBoneCount), 1);
        // a node without a body reference adds nothing
        let bare = havok_node(&mut e, 0, None, false);
        e.call(0x0056_0870, &args![bare, walk]);
        assert_eq!(e.get(record, HavokSaveData::sInactiveBoneCount), 1);
    }

    #[test]
    fn fn_005604b0_saves_the_flags_the_count_and_walks_the_nodes() {
        let mut e = havok_engine();
        install_walk(&mut e, vec![]);
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        let refr = batch_refr(&mut e, false);
        let record: Ptr<HavokSaveData> = e.new_object();
        e.set(record, HavokSaveData::cFlags, 5);
        e.set(record, HavokSaveData::sActiveBoneCount, 2);
        e.set(record, HavokSaveData::sInactiveBoneCount, 3);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        let buffer = e.mem.u32(game + 0x14);
        e.call_log = Some(vec![]);
        e.call(0x0056_04b0, &args![refr, record]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.bytes(buffer, 3), [5, 5, 0]);
        let walks = calls_to(&log, WALK_COLLISION);
        assert_eq!(walks.len(), 1);
        assert_eq!((walks[0][0], walks[0][2]), (obj3d, 0x0056_0a10));
        assert_eq!(e.mem.u32(walks[0][1] + 0xc), record.addr());
        // no 3D, nothing
        returns(&mut e, GET_LOADED_3D, 0);
        e.mem.set_u32(game + 0x14, buffer);
        e.call(0x0056_04b0, &args![refr, record]);
        assert_eq!(e.mem.u32(game + 0x14), buffer);
    }

    // ---- small accessors of the Havok glue ---------------------------------

    #[test]
    fn rigid_body_activity_comes_from_the_entity() {
        let mut e = havok_engine();
        let no_entity = e.mem.alloc(0x40);
        assert!(!e.call(0x0056_09b0, &args![no_entity]).bool());
        let body = e.mem.alloc(0x40);
        let entity = e.mem.alloc(0x40);
        e.mem.set_u32(body + 4, entity);
        assert!(!e.call(0x0056_09b0, &args![body]).bool());
        e.mem.set_u8(entity, 1);
        assert!(e.call(0x0056_09b0, &args![body]).bool());
        // the byte test
        let byte = e.mem.alloc(8);
        assert!(!e.call(0x0056_09f0, &args![byte]).bool());
        e.mem.set_u8(byte, 0x80);
        assert!(e.call(0x0056_09f0, &args![byte]).bool());
    }

    #[test]
    fn vector_element_helpers_index_floats() {
        let mut e = batch_engine();
        assert_eq!(e.call(0x0056_0d30, &args![0x1000u32, 3u32]).u32(), 0x100c);
        assert_eq!(e.call(0x0056_0d10, &args![0x1000u32, 2u32]).u32(), 0x1008);
        let quaternion = e.mem.alloc(0x10);
        e.call(0x0056_0cf0, &args![quaternion, 2.5f32]);
        assert_eq!(e.mem.f32(quaternion + 8), 2.5);
    }

    /// Doubles for the velocity getters: an entity at +4 of the body whose
    /// chain `+0x10` leads to the first velocity vector's holder (the
    /// vector is 0x20 bytes into it) and on to the second vector.
    fn velocity_body(e: &mut Engine) -> (u32, u32, u32) {
        e.register(BODY_ENTITY, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(BODY_VECTOR_PART, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(BODY_VECTOR_FINISH, |_, a| (a[0] + 0x20).into_ret());
        e.register(VECTOR_TO_NI, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(VECTOR_TO_NI_VELOCITY, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(ZERO_VECTOR_GETTER, |_, _| ZERO_VECTOR.into_ret());
        let body = e.mem.alloc(0x40);
        let entity = e.mem.alloc(0x40);
        let first = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x40);
        e.mem.set_u32(body + 4, entity);
        e.mem.set_u32(entity + 0x10, first);
        e.mem.set_u32(first + 0x10, second);
        for i in 0..3 {
            e.mem.set_f32(first + 0x20 + 4 * i, 1.0 + i as f32);
            e.mem.set_f32(second + 4 * i, 10.0 + i as f32);
        }
        (body, first, second)
    }

    #[test]
    fn velocity_getters_walk_from_the_entity_and_default_to_the_zero_vector() {
        let mut e = batch_engine();
        let (body, first, second) = velocity_body(&mut e);
        let entity = e.mem.u32(body + 4);
        assert_eq!(e.call(0x0056_0dc0, &args![entity]).u32(), first + 0x20);
        assert_eq!(e.call(0x0056_0e50, &args![entity]).u32(), second);
        assert_eq!(e.call(0x0056_0d80, &args![body]).u32(), first + 0x20);
        assert_eq!(e.call(0x0056_0e10, &args![body]).u32(), second);
        let bare = e.mem.alloc(0x40);
        assert_eq!(e.call(0x0056_0d80, &args![bare]).u32(), ZERO_VECTOR);
        assert_eq!(e.call(0x0056_0e10, &args![bare]).u32(), ZERO_VECTOR);
        // the converting ones copy the vector to the destination
        let dest = e.mem.alloc(0x10);
        e.call(0x0056_0d50, &args![body, dest]);
        assert_eq!(e.mem.f32(dest + 8), 3.0);
        e.call(0x0056_0de0, &args![body, dest]);
        assert_eq!(e.mem.f32(dest + 8), 12.0);
    }

    #[test]
    fn rigid_body_position_and_rotation_convert_what_the_virtuals_fill() {
        let mut e = batch_engine();
        // virtual +0xd4 / +0xd8 return a pointer to the filled vector
        e.register(0x00fb_00d4, |e, a| {
            for i in 0..3 {
                e.mem.set_f32(a[1] + 4 * i, 1.0 + i as f32);
            }
            a[1].into_ret()
        });
        e.register(0x00fb_00d8, |e, a| {
            for i in 0..4 {
                e.mem.set_f32(a[1] + 4 * i, 5.0 + i as f32);
            }
            a[1].into_ret()
        });
        let body = object_with_vtable(&mut e, 0x40, &[(0xd4, 0x00fb_00d4), (0xd8, 0x00fb_00d8)]);
        identity_constructors(&mut e);
        e.register(VECTOR_TO_NI, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        // the four setters of the quaternion copy store the component at
        // different places of the destination (+8 is `fn_00560cf0`'s)
        e.register(QUATERNION_SET_0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(QUATERNION_SET_2, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(QUATERNION_SET_3, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret::default()
        });
        let position = e.mem.alloc(0x10);
        e.call(0x0056_0bc0, &args![body, position]);
        assert_eq!(
            [
                e.mem.f32(position),
                e.mem.f32(position + 4),
                e.mem.f32(position + 8)
            ],
            [1.0, 2.0, 3.0]
        );
        let rotation = e.mem.alloc(0x10);
        e.call(0x0056_0c20, &args![body, rotation]);
        assert_eq!(
            [
                e.mem.f32(rotation),
                e.mem.f32(rotation + 4),
                e.mem.f32(rotation + 8),
                e.mem.f32(rotation + 0xc)
            ],
            [5.0, 7.0, 6.0, 8.0]
        );
        // the converter returns its destination
        let source = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_f32(source + 4 * i, 0.5 * i as f32);
        }
        let dest = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0056_0c80, &args![dest, source]).u32(), dest);
    }

    #[test]
    fn body_position_and_rotation_setters_convert_and_call_the_virtuals() {
        let mut e = batch_engine();
        identity_constructors(&mut e);
        e.register(NI_TO_VECTOR, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(NI_QUATERNION_TO_HAVOK, |e, a| {
            let bytes = e.mem.bytes(a[1], 16);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f32>::new()));
        let log = seen.clone();
        e.register_double(0x00fb_00dc, move |e, a| {
            log.borrow_mut().push(e.mem.f32(a[1] + 8));
            Ret::default()
        });
        let log = seen.clone();
        e.register_double(0x00fb_00e0, move |e, a| {
            log.borrow_mut().push(e.mem.f32(a[1] + 0xc));
            Ret::default()
        });
        // the normalization (fn_005611c0) is checked below; here it is a
        // plain function of the vector, so give it a unit quaternion
        e.register(0x0056_1440, |e, a| {
            let mut dot = 0.0f32;
            for i in 0..4 {
                dot += e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[2] + 4 * i);
            }
            for i in 0..4 {
                e.mem.set_f32(a[1] + 4 * i, dot);
            }
            a[1].into_ret()
        });
        e.register(COPY_16_BYTES, |e, a| {
            let bytes = e.mem.bytes(a[1], 16);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let body = object_with_vtable(&mut e, 0x40, &[(0xdc, 0x00fb_00dc), (0xe0, 0x00fb_00e0)]);
        let point = e.mem.alloc(0x10);
        e.mem.set_f32(point + 8, 7.0);
        e.call(0x0056_10f0, &args![body, point]);
        let quaternion = e.mem.alloc(0x10);
        for (i, value) in [0.0f32, 0.0, 0.0, 2.0].into_iter().enumerate() {
            e.mem.set_f32(quaternion + 4 * i as u32, value);
        }
        e.call(0x0056_1150, &args![body, quaternion]);
        let seen = seen.borrow();
        assert_eq!(seen[0], 7.0);
        // normalized from (0, 0, 0, 2) to about (0, 0, 0, 1)
        assert!((seen[1] - 1.0).abs() < 1e-3, "{}", seen[1]);
    }

    #[test]
    fn normalization_scales_by_the_newton_step_of_the_rsqrt_estimate() {
        let mut e = batch_engine();
        identity_constructors(&mut e);
        e.register(0x0056_1440, |e, a| {
            let mut dot = 0.0f32;
            for i in 0..4 {
                dot += e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[2] + 4 * i);
            }
            for i in 0..4 {
                e.mem.set_f32(a[1] + 4 * i, dot);
            }
            a[1].into_ret()
        });
        e.register(COPY_16_BYTES, |e, a| {
            let bytes = e.mem.bytes(a[1], 16);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let vector = e.mem.alloc(0x10);
        for (i, value) in [1.0f32, 2.0, 2.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(vector + 4 * i as u32, value);
        }
        // fn_005612a0: the factor is 1 / sqrt(25) = 0.2, in lane 0 only
        let factor = e.mem.alloc(0x10);
        let returned = e.call(0x0056_12a0, &args![vector, factor]).u32();
        assert_eq!(returned, factor);
        assert!(
            (e.mem.f32(factor) - 0.2).abs() < 1e-5,
            "{}",
            e.mem.f32(factor)
        );
        assert_eq!(e.mem.bytes(factor + 4, 12), [0; 12]);
        // fn_00561240 splats the first lane
        let splat = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0056_1240, &args![factor, splat]).u32(), splat);
        for i in 0..4 {
            assert_eq!(e.mem.u32(splat + 4 * i), e.mem.u32(factor));
        }
        // fn_005611e0 / fn_005611c0 scale the vector by it
        e.call(0x0056_11c0, &args![vector]);
        let expected = [0.2f32, 0.4, 0.4, 0.8];
        for i in 0..4 {
            assert!(
                (e.mem.f32(vector + 4 * i) - expected[i as usize]).abs() < 1e-4,
                "lane {i}: {}",
                e.mem.f32(vector + 4 * i)
            );
        }
        // a zero vector has a zero factor (the estimate is masked)
        let zero = e.mem.alloc(0x10);
        let factor = e.mem.alloc(0x10);
        e.mem.set_u32(factor, 0xdead_beef);
        e.call(0x0056_12a0, &args![zero, factor]);
        assert_eq!(e.mem.u32(factor), 0);
        e.call(0x0056_11e0, &args![zero]);
        assert_eq!(e.mem.bytes(zero, 16), [0; 16]);
    }

    // ---- the save and load callbacks of the collision walk ---------------------

    fn floats(values: &[f32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    /// Doubles for everything the Havok callbacks convert, for a body with
    /// a vtable: position (1, 2, 3) and rotation (5, 6, 7, 8) from the
    /// virtuals +0xd4 / +0xd8, velocities (1, 2, 3) and (10, 11, 12), and
    /// the quaternion setters storing component 0, 2 and 3 at +0, +4, +0xc
    /// (`fn_00560cf0` stores component 1 at +8), so a rotation is saved as
    /// 5, 7, 6, 8. The record of what the position/rotation setter
    /// virtuals (+0xdc, +0xe0) and the body setters were given is returned.
    struct BodyLog {
        position: std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>>,
        rotation: std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>>,
        first_vector: std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>>,
        second_vector: std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>>,
    }

    fn rich_body(e: &mut Engine, active: bool, was_set: bool) -> (u32, BodyLog) {
        use std::cell::RefCell;
        use std::rc::Rc;
        let (body, _, _) = velocity_body(e);
        let entity = e.mem.u32(body + 4);
        e.mem.set_u8(entity, active as u8);
        identity_constructors(e);
        e.register(0x00fb_00d4, |e, a| {
            for i in 0..3 {
                e.mem.set_f32(a[1] + 4 * i, 1.0 + i as f32);
            }
            a[1].into_ret()
        });
        e.register(0x00fb_00d8, |e, a| {
            for i in 0..4 {
                e.mem.set_f32(a[1] + 4 * i, 5.0 + i as f32);
            }
            a[1].into_ret()
        });
        e.register(QUATERNION_SET_0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(QUATERNION_SET_2, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(QUATERNION_SET_3, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret::default()
        });
        e.register(NI_TO_VECTOR, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(NI_QUATERNION_TO_HAVOK, |e, a| {
            let bytes = e.mem.bytes(a[1], 16);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(0x0056_1440, |e, a| {
            let mut dot = 0.0f32;
            for i in 0..4 {
                dot += e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[2] + 4 * i);
            }
            for i in 0..4 {
                e.mem.set_f32(a[1] + 4 * i, dot);
            }
            a[1].into_ret()
        });
        e.register(COPY_16_BYTES, |e, a| {
            let bytes = e.mem.bytes(a[1], 16);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let log = BodyLog {
            position: Rc::new(RefCell::new(vec![])),
            rotation: Rc::new(RefCell::new(vec![])),
            first_vector: Rc::new(RefCell::new(vec![])),
            second_vector: Rc::new(RefCell::new(vec![])),
        };
        let sink = log.position.clone();
        e.register_double(0x00fb_00dc, move |e, a| {
            sink.borrow_mut().push(e.mem.bytes(a[1], 12));
            Ret::default()
        });
        let sink = log.rotation.clone();
        e.register_double(0x00fb_00e0, move |e, a| {
            sink.borrow_mut().push(e.mem.bytes(a[1], 16));
            Ret::default()
        });
        let sink = log.first_vector.clone();
        e.register_double(BODY_SET_FIRST_VECTOR, move |e, a| {
            sink.borrow_mut().push(e.mem.bytes(a[1], 12));
            Ret::default()
        });
        let sink = log.second_vector.clone();
        e.register_double(BODY_SET_SECOND_VECTOR, move |e, a| {
            sink.borrow_mut().push(e.mem.bytes(a[1], 12));
            Ret::default()
        });
        stub(e, &[BODY_FLAG_SETTER]);
        e.register(
            0x00fb_0094,
            if was_set {
                |_, _| 1u32.into_ret()
            } else {
                |_, _| 0u32.into_ret()
            },
        );
        let vtable = e.mem.alloc(0x500);
        for (offset, target) in [
            (0xd4, 0x00fb_00d4),
            (0xd8, 0x00fb_00d8),
            (0xdc, 0x00fb_00dc),
            (0xe0, 0x00fb_00e0),
            (0x94, 0x00fb_0094),
        ] {
            e.mem.set_u32(vtable + offset, target);
        }
        e.mem.set_u32(body, vtable);
        (body, log)
    }

    /// A node whose body reference is `body`.
    fn node_of(e: &mut Engine, body: u32) -> u32 {
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node + 0x10, body);
        node
    }

    fn walk_state(
        e: &mut Engine,
        refr: Ptr<TESObjectREFR>,
        root: u32,
        flags: u8,
    ) -> (Ptr<HavokSaveData>, Ptr<CollisionWalkContext>) {
        let record: Ptr<HavokSaveData> = e.new_object();
        e.set(record, HavokSaveData::pRef, refr.cast());
        e.set(record, HavokSaveData::pCollisionRoot, Ptr::new(root));
        e.set(record, HavokSaveData::cFlags, flags);
        let walk: Ptr<CollisionWalkContext> = e.new_object();
        e.set(walk, CollisionWalkContext::pUserData, record.cast());
        (record, walk)
    }

    #[test]
    fn fn_00560a10_saves_position_rotation_activity_and_velocities() {
        let mut e = havok_engine();
        let (body, _) = rich_body(&mut e, true, false);
        let root = havok_node(&mut e, 0, None, false);
        let node = node_of(&mut e, body);
        let refr = batch_refr(&mut e, false);
        let (_, walk) = walk_state(&mut e, refr, root, 4);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        let buffer = e.mem.u32(game + 0x14);
        e.call(0x0056_0a10, &args![node, walk]);
        let mut expected = floats(&[1.0, 2.0, 3.0]);
        expected.extend(floats(&[5.0, 7.0, 6.0, 8.0]));
        expected.push(1);
        expected.extend(floats(&[1.0, 2.0, 3.0]));
        expected.extend(floats(&[10.0, 11.0, 12.0]));
        assert_eq!(e.mem.bytes(buffer, expected.len() as u32), expected);
        assert_eq!(e.mem.u32(game + 0x14), buffer + expected.len() as u32);
        // the root has no position or rotation of its own; without flag 4
        // there is no activity byte; an inactive body has no velocities
        let root_body = rich_body(&mut e, true, false).0;
        e.mem.set_u32(root + 0x10, root_body);
        e.mem.set_u32(game + 0x14, buffer);
        let (_, walk) = walk_state(&mut e, refr, root, 0);
        e.call(0x0056_0a10, &args![root, walk]);
        let mut expected = floats(&[1.0, 2.0, 3.0]);
        expected.extend(floats(&[10.0, 11.0, 12.0]));
        assert_eq!(e.mem.bytes(buffer, expected.len() as u32), expected);
        let (inactive, _) = rich_body(&mut e, false, false);
        let node = node_of(&mut e, inactive);
        e.mem.set_u32(game + 0x14, buffer);
        let (_, walk) = walk_state(&mut e, refr, root, 4);
        e.call(0x0056_0a10, &args![node, walk]);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 12 + 16 + 1);
        assert_eq!(e.mem.u8(buffer + 28), 0);
        // a node without a body writes nothing
        let bare = havok_node(&mut e, 0, None, false);
        e.mem.set_u32(game + 0x14, buffer);
        e.call(0x0056_0a10, &args![bare, walk]);
        assert_eq!(e.mem.u32(game + 0x14), buffer);
    }

    #[test]
    fn fn_00560e70_loads_position_rotation_and_velocities_into_the_body() {
        let mut e = havok_engine();
        let (body, log) = rich_body(&mut e, false, true);
        let root = havok_node(&mut e, 0, None, false);
        let node = node_of(&mut e, body);
        stub(&mut e, &[0x004d_9fa0]);
        let refr = batch_refr(&mut e, false);
        let (record, walk) = walk_state(&mut e, refr, root, 4);
        let mut stream = floats(&[1.0, 2.0, 3.0]);
        stream.extend(floats(&[0.0, 0.0, 0.0, 1.0]));
        stream.push(1);
        stream.extend(floats(&[7.0, 8.0, 9.0]));
        stream.extend(floats(&[10.0, 11.0, 12.0]));
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        let buffer = e.mem.u32(game + 0x14);
        e.mem.write(buffer, &stream);
        e.call_log = Some(vec![]);
        e.call(0x0056_0e70, &args![node, walk]);
        let calls = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(game + 0x14), buffer + stream.len() as u32);
        assert_eq!(log.position.borrow()[0], floats(&[1.0, 2.0, 3.0]));
        let rotation = log.rotation.borrow()[0].clone();
        let lanes: Vec<f32> = rotation
            .chunks(4)
            .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
            .collect();
        assert!(
            (lanes[3] - 1.0).abs() < 1e-3 && lanes[0] == 0.0,
            "{lanes:?}"
        );
        assert_eq!(log.first_vector.borrow()[0], floats(&[7.0, 8.0, 9.0]));
        assert_eq!(log.second_vector.borrow()[0], floats(&[10.0, 11.0, 12.0]));
        // the body's flag was set before and active after
        assert_eq!(
            calls_to(&calls, BODY_FLAG_SETTER),
            vec![vec![body, 0], vec![body, 1]]
        );
        // an inactive body gets zero velocities and the flag cleared again
        let (body, log) = rich_body(&mut e, false, true);
        let node = node_of(&mut e, body);
        e.set(record, HavokSaveData::cFlags, 0);
        let mut stream = floats(&[1.0, 2.0, 3.0]);
        stream.extend(floats(&[0.0, 0.0, 0.0, 1.0]));
        e.mem.write(buffer, &stream);
        e.mem.set_u32(game + 0x14, buffer);
        e.call_log = Some(vec![]);
        e.call(0x0056_0e70, &args![node, walk]);
        let calls = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(game + 0x14), buffer + stream.len() as u32);
        assert_eq!(
            calls_to(&calls, BODY_SET_FIRST_VECTOR),
            vec![vec![body, ZERO_VECTOR]]
        );
        assert_eq!(
            calls_to(&calls, BODY_SET_SECOND_VECTOR),
            vec![vec![body, ZERO_VECTOR]]
        );
        assert_eq!(
            calls_to(&calls, BODY_FLAG_SETTER),
            vec![vec![body, 0], vec![body, 0]]
        );
        assert_eq!(log.first_vector.borrow().clone(), vec![vec![0u8; 12]]);
        // flag 1 means active without a byte in the stream
        e.set(record, HavokSaveData::cFlags, 1);
        let mut stream = floats(&[1.0, 2.0, 3.0]);
        stream.extend(floats(&[0.0, 0.0, 0.0, 1.0]));
        stream.extend(floats(&[4.0, 5.0, 6.0, 7.0, 8.0, 9.0]));
        e.mem.write(buffer, &stream);
        e.mem.set_u32(game + 0x14, buffer);
        e.call(0x0056_0e70, &args![node, walk]);
        assert_eq!(e.mem.u32(game + 0x14), buffer + stream.len() as u32);
        assert_eq!(log.first_vector.borrow()[1], floats(&[4.0, 5.0, 6.0]));
    }

    #[test]
    fn fn_00560e70_sets_the_collision_roots_pose_from_the_reference() {
        let mut e = havok_engine();
        let (body, log) = rich_body(&mut e, false, false);
        let root = node_of(&mut e, body);
        stub(&mut e, &[0x004d_9fa0]);
        e.register(GET_ORIENTATION, |_, a| a[1].into_ret());
        e.register(QUATERNION_FROM_ROTATION, |e, a| {
            e.mem.write(a[0], &floats(&[0.0, 2.0, 0.0, 0.0]));
            a[0].into_ret()
        });
        let refr = batch_refr(&mut e, false);
        let (record, walk) = walk_state(&mut e, refr, root, 3);
        e.mem.set_f32(refr.addr() + 0x30 + 8, 4.0);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        let buffer = e.mem.u32(game + 0x14);
        e.call(0x0056_0e70, &args![root, walk]);
        // flag 2 is cleared, nothing about the pose is read from the stream
        // (flag 1 here: active, so the two velocity vectors are)
        assert_eq!(e.get(record, HavokSaveData::cFlags), 1);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 24);
        // the location is `this + 0x30` (the double of the virtual +0x1f4)
        assert_eq!(
            log.position.borrow()[0],
            e.mem.bytes(refr.addr() + 0x30, 12)
        );
        // the orientation's quaternion is normalized to (0, 1, 0, 0)
        let rotation = log.rotation.borrow()[0].clone();
        let y = f32::from_le_bytes(rotation[4..8].try_into().unwrap());
        assert!((y - 1.0).abs() < 1e-3, "{y}");
    }

    // ---- 00560530: loading the Havok record -----------------------------------

    /// A reference big enough for an actor's ragdoll pointer at +0xac.
    fn big_refr(e: &mut Engine, actor: bool) -> Ptr<TESObjectREFR> {
        batch_refr(e, actor);
        let refr = e.mem.alloc(0x200);
        e.mem.set_u32(refr, REFR_VTABLE);
        e.mem.set_u32(refr + 0x18, CHILD_CELL_VTABLE);
        Ptr::new(refr)
    }

    fn havok_load_setup(version: u8, stream: &[u8]) -> (Engine, u32) {
        let mut e = havok_engine();
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        e.mem.set_u8(game + 0x80, version);
        let buffer = e.mem.u32(game + 0x14);
        e.mem.write(buffer, stream);
        stub(
            &mut e,
            &[
                LOAD_OLD_HAVOK,
                MESSAGE,
                SET_HAVOK_WEAPON,
                DISABLE_RAGDOLL_ANIM,
                KNOCK_DOWN,
                SET_MOTION,
                SET_FIXED,
                CLEAR_ANIM_GROUP,
                ANIM_FINISH,
                MAKE_VELOCITY,
                SET_3D_VELOCITY,
            ],
        );
        (e, buffer)
    }

    #[test]
    fn fn_00560530_sends_old_saves_elsewhere_and_skips_a_reference_without_3d() {
        let (mut e, _) = havok_load_setup(0x50, &[]);
        let refr = big_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0056_0530, &args![refr, 12u16]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOAD_OLD_HAVOK), vec![vec![refr.addr(), 12]]);
        assert!(calls_to(&log, GET_LOADED_3D).is_empty());
        // 0x51 and later, but no 3D: a message and the record is skipped
        let (mut e, buffer) = havok_load_setup(0x51, &[]);
        returns(&mut e, GET_LOADED_3D, 0);
        let refr = big_refr(&mut e, false);
        e.mem.set_u32(refr.addr() + 0xc, 0x0006_0001);
        e.call_log = Some(vec![]);
        e.call(0x0056_0530, &args![refr, 12u16]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![HAVOK_NO_3D_FORMAT, 0x0e01, 0x0006_0001]]
        );
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 12);
    }

    #[test]
    fn fn_00560530_with_a_different_bone_count_skips_the_record_and_knocks_an_actor_down() {
        // saved: flags 8 (weapon), 5 bones
        let (mut e, buffer) = havok_load_setup(0x51, &[8, 5, 0]);
        let root = havok_node(&mut e, 0, Some(true), false);
        let first = havok_node(&mut e, 0, Some(true), false);
        install_walk(&mut e, vec![root, first]);
        returns(&mut e, COLLISION_ROOT, root);
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        let refr = big_refr(&mut e, true);
        e.mem.set_u32(refr.addr() + 0xc, 0x0006_0001);
        e.mem.set_u32(refr.addr() + 0xac, 0x5a5a);
        e.call_log = Some(vec![]);
        e.call(0x0056_0530, &args![refr, 20u16]);
        let log = e.call_log.take().unwrap();
        // the actor learns it had a weapon bone
        assert_eq!(calls_to(&log, SET_HAVOK_WEAPON), vec![vec![refr.addr(), 1]]);
        // 5 saved, 2 now; and the weapon bone differs
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![
                vec![HAVOK_BONE_COUNT_FORMAT, 0x0e01, 0x0006_0001, 5, 2],
                vec![HAVOK_WEAPON_BONE_FORMAT, TRUE_TEXT, FALSE_TEXT]
            ]
        );
        // the rest of the record (size - 3) is skipped after the 3 bytes read
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 3 + 17);
        assert_eq!(calls_to(&log, DISABLE_RAGDOLL_ANIM), vec![vec![0x5a5a, 1]]);
        assert_eq!(
            calls_to(&log, KNOCK_DOWN),
            vec![vec![obj3d, ZERO_VECTOR, 1, 0.0f32.to_bits(), 0]]
        );
        // no weapon difference and no ragdoll: neither message nor disable
        let (mut e, _) = havok_load_setup(0x51, &[0, 5, 0]);
        install_walk(&mut e, vec![]);
        returns(&mut e, COLLISION_ROOT, 0);
        returns(&mut e, GET_LOADED_3D, obj3d);
        let refr = big_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0056_0530, &args![refr, 20u16]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MESSAGE).len(), 1);
        assert!(calls_to(&log, DISABLE_RAGDOLL_ANIM).is_empty());
        assert_eq!(calls_to(&log, SET_HAVOK_WEAPON), vec![vec![refr.addr(), 0]]);
        assert_eq!(calls_to(&log, KNOCK_DOWN).len(), 1);
    }

    #[test]
    fn fn_00560530_with_the_same_bone_count_loads_the_bodies() {
        // saved: no flags, no bones
        let (mut e, buffer) = havok_load_setup(0x51, &[0, 0, 0]);
        install_walk(&mut e, vec![]);
        returns(&mut e, COLLISION_ROOT, 0x7007);
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        slot(&mut e, 0x234, |_, _| true.into_ret());
        slot(&mut e, 0x1e4, |_, _| 0x9009u32.into_ret());
        let refr = big_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0056_0530, &args![refr, 20u16]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MESSAGE).is_empty());
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 3);
        // the load walk gets the saved record as its context, with the
        // reference, the 3D and the collision root filled in
        let walks = calls_to(&log, WALK_COLLISION);
        assert_eq!(walks.len(), 2);
        assert_eq!(walks[0][2], 0x0056_0870);
        assert_eq!(walks[1][2], 0x0056_0e70);
        let saved = e.mem.u32(walks[1][1] + 0xc);
        assert_eq!(e.mem.u32(saved + 8), refr.addr());
        assert_eq!(e.mem.u32(saved + 0xc), obj3d);
        assert_eq!(e.mem.u32(saved + 0x10), 0x7007);
        // an actor: motion set before and after the walk, fixed when the
        // virtual +0x234 says so, its animation group cleared
        let motion = vec![obj3d, 1, 1, 0, 1];
        assert_eq!(calls_to(&log, SET_MOTION), vec![motion.clone(), motion]);
        assert_eq!(calls_to(&log, SET_FIXED), vec![vec![obj3d, 1, 1]]);
        assert_eq!(
            calls_to(&log, CLEAR_ANIM_GROUP),
            vec![vec![0x9009, 0x14, 0.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, ANIM_FINISH), vec![vec![0x9009]]);
        // one zero velocity record (before the walk), none after
        let made = calls_to(&log, MAKE_VELOCITY);
        assert_eq!(made.len(), 1);
        assert_eq!(&made[0][1..], &[0.0f32.to_bits(), 0, 0]);
        assert_eq!(calls_to(&log, SET_3D_VELOCITY).len(), 1);
        // a non-actor: only the zero velocity after the walk
        let (mut e, _) = havok_load_setup(0x51, &[0, 0, 0]);
        install_walk(&mut e, vec![]);
        returns(&mut e, COLLISION_ROOT, 0x7007);
        returns(&mut e, GET_LOADED_3D, obj3d);
        let refr = big_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0056_0530, &args![refr, 20u16]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SET_MOTION).is_empty());
        assert!(calls_to(&log, SET_FIXED).is_empty());
        assert_eq!(calls_to(&log, MAKE_VELOCITY).len(), 1);
        assert_eq!(
            calls_to(&log, SET_3D_VELOCITY),
            vec![vec![obj3d, calls_to(&log, MAKE_VELOCITY)[0][0]]]
        );
    }

    // ===== 00561440 to 00563890 ==========================================

    type Sink = std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>>;
    /// The walks of `load_havok_setup`: (callback, flags byte of the record).
    type WalkFlags = std::rc::Rc<std::cell::RefCell<Vec<(u32, u8)>>>;
    /// The sequence loads of `load_animation_setup`: (this, manager, name, time).
    type SequenceLoads = std::rc::Rc<std::cell::RefCell<Vec<(u32, u32, Vec<u8>, u32)>>>;

    /// Registers a double at `addr` that records the `len` bytes its argument
    /// number `arg` points to, at each call.
    fn record_bytes(e: &mut Engine, addr: u32, arg: usize, len: u32) -> Sink {
        let sink: Sink = Sink::default();
        let seen = sink.clone();
        e.register_double(addr, move |e, a| {
            seen.borrow_mut().push(e.mem.bytes(a[arg], len));
            Ret::default()
        });
        sink
    }

    /// Registers a double at `addr` that records `a[2]` bytes at `a[1]`
    /// (a buffer write: destination, source, size).
    fn record_writes(e: &mut Engine, addr: u32) -> Sink {
        let sink: Sink = Sink::default();
        let seen = sink.clone();
        e.register_double(addr, move |e, a| {
            seen.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            Ret::default()
        });
        sink
    }

    /// A stream the buffer load double reads from.
    fn load_stream(e: &mut Engine, bytes: Vec<u8>) -> std::rc::Rc<std::cell::Cell<usize>> {
        let position = std::rc::Rc::new(std::cell::Cell::new(0usize));
        let seen = position.clone();
        e.register_double(BUFFER_LOAD_DATA, move |e, a| {
            let at = seen.get();
            let size = a[2] as usize;
            e.mem.write(a[1], &bytes[at..at + size]);
            seen.set(at + size);
            Ret::default()
        });
        position
    }

    fn lanes(bytes: &[u8]) -> Vec<f32> {
        bytes
            .chunks(4)
            .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
            .collect()
    }

    /// The addresses of `log` that are in `only`, in call order.
    fn ordered(log: &[(u32, Vec<u32>)], only: &[u32]) -> Vec<u32> {
        log.iter()
            .map(|(a, _)| *a)
            .filter(|a| only.contains(a))
            .collect()
    }

    /// A 16-byte vector with the given lanes.
    fn vector4(e: &mut Engine, x: f32, y: f32, z: f32) -> u32 {
        let vector = e.mem.alloc(16);
        e.mem.write(vector, &floats(&[x, y, z, 0.0]));
        vector
    }

    /// Doubles with the meaning of the SSE helpers `fn_00561730` chains.
    fn vector_doubles(e: &mut Engine) {
        e.register(VECTOR_SUBTRACT, |e, a| {
            for i in 0..4 {
                let value = e.mem.f32(a[1] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, value);
            }
            a[0].into_ret()
        });
        e.register(VECTOR_ABSOLUTE, |e, a| {
            for i in 0..4 {
                let value = e.mem.f32(a[1] + 4 * i).abs();
                e.mem.set_f32(a[0] + 4 * i, value);
            }
            a[0].into_ret()
        });
        e.register(VECTOR_SPLAT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1]);
            }
            a[0].into_ret()
        });
        e.register(VECTOR_COMPARE, |e, a| {
            for i in 0..4 {
                let within = e.mem.f32(a[0] + 4 * i) <= e.mem.f32(a[2] + 4 * i);
                e.mem
                    .set_u32(a[1] + 4 * i, if within { u32::MAX } else { 0 });
            }
            a[1].into_ret()
        });
        e.register(VECTOR_ALL_SET, |e, a| {
            let mut bits = 0;
            for i in 0..4 {
                bits |= (e.mem.u32(a[0] + 4 * i) >> 31) << i;
            }
            u32::from(bits & a[1] == a[1]).into_ret()
        });
    }

    /// What the motion's two velocity setters were given.
    struct MotionLog {
        linear: Sink,
        angular: Sink,
    }

    /// Doubles for the velocity setters of the body `body` made by
    /// `velocity_body`: its current velocities are (1, 2, 3) and
    /// (10, 11, 12), its motion (the first velocity holder) gets a vtable
    /// whose slots +0x40 and +0x44 record the 12 bytes of the vector.
    fn motion_doubles(e: &mut Engine, body: u32) -> MotionLog {
        vector_doubles(e);
        e.register(NI_TO_VECTOR, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(NI_POINT_TO_VECTOR_ZERO_W, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            e.mem.write(a[0], &bytes);
            e.mem.set_u32(a[0] + 12, 0);
            a[0].into_ret()
        });
        stub(e, &[BODY_LOCK_MARKER, ENTITY_ACTIVATE, ENTITY_DEACTIVATE]);
        let entity = e.mem.u32(body + 4);
        let motion = e.mem.u32(entity + 0x10);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x40, 0x00fb_0140);
        e.mem.set_u32(vtable + 0x44, 0x00fb_0144);
        e.mem.set_u32(motion, vtable);
        MotionLog {
            linear: record_bytes(e, 0x00fb_0140, 1, 12),
            angular: record_bytes(e, 0x00fb_0144, 1, 12),
        }
    }

    /// An engine with a body whose entity has the velocities (1, 2, 3) and
    /// (10, 11, 12): (engine, body, entity, motion log).
    fn glue_world() -> (Engine, u32, u32, MotionLog) {
        let mut e = batch_engine();
        e.set_global(VELOCITY_EPSILON, 0.001f32);
        let (body, _, _) = velocity_body(&mut e);
        let motion = motion_doubles(&mut e, body);
        let entity = e.mem.u32(body + 4);
        (e, body, entity, motion)
    }

    // ---- 00561440, 00561500 ---------------------------------------------

    #[test]
    fn fn_00561440_is_the_dot_product_added_in_the_order_of_the_shuffles() {
        let mut e = batch_engine();
        e.register(COPY_16_BYTES, |e, a| {
            let bytes = e.mem.bytes(a[1], 16);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let this = e.mem.alloc(16);
        let other = e.mem.alloc(16);
        let out = e.mem.alloc(16);
        e.mem.write(this, &floats(&[1.0, 2.0, 3.0, 4.0]));
        e.mem.write(other, &floats(&[5.0, 6.0, 7.0, 8.0]));
        e.call_log = Some(vec![]);
        let result = e.call(0x0056_1440, &args![this, out, other]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, out);
        assert_eq!(lanes(&e.mem.bytes(out, 16)), vec![70.0; 4]);
        assert_eq!(calls_to(&log, COPY_16_BYTES).len(), 1);
        // the sum is (p3 + p1) + (p2 + p0), not left to right
        e.mem.write(this, &floats(&[1.0e8, 1.0, -1.0e8, 1.0]));
        e.mem.write(other, &floats(&[1.0; 4]));
        e.call(0x0056_1440, &args![this, out, other]);
        assert_eq!(lanes(&e.mem.bytes(out, 16)), vec![2.0; 4]);
    }

    #[test]
    fn fn_00561500_stores_the_quaternion_components_in_havok_order() {
        let mut e = batch_engine();
        // the four getters return the floats at +4, +8, +0xc and +0 in ST0
        for (getter, offset) in [
            (0x006b_9130, 4),
            (0x0048_8d50, 8),
            (0x0084_d030, 0xc),
            (0x006a_7f50, 0),
        ] {
            e.register_double(getter, move |e, a| Ret {
                st0: f64::from(e.mem.f32(a[0] + offset)),
                ..Ret::default()
            });
        }
        let source = e.mem.alloc(16);
        e.mem.write(source, &floats(&[0.5, 0.1, 0.2, 0.3]));
        let dest = e.mem.alloc(16);
        assert_eq!(e.call(0x0056_1500, &args![dest, source]).u32(), dest);
        assert_eq!(lanes(&e.mem.bytes(dest, 16)), vec![0.1, 0.2, 0.3, 0.5]);
    }

    // ---- 00561580 and the velocity setters -------------------------------

    #[test]
    fn fn_00561580_activates_or_deactivates_the_entity_between_lock_markers() {
        let mut e = batch_engine();
        returns(&mut e, BODY_ENTITY, 0x4040);
        stub(
            &mut e,
            &[BODY_LOCK_MARKER, ENTITY_ACTIVATE, ENTITY_DEACTIVATE],
        );
        e.call_log = Some(vec![]);
        e.call(0x0056_1580, &args![0x3000u32, 1u8]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log),
            vec![
                0x0056_1580,
                BODY_ENTITY,
                BODY_LOCK_MARKER,
                ENTITY_ACTIVATE,
                BODY_LOCK_MARKER
            ]
        );
        assert_eq!(calls_to(&log, ENTITY_ACTIVATE), vec![vec![0x4040]]);
        assert_eq!(calls_to(&log, BODY_LOCK_MARKER), vec![vec![0x3000]; 2]);
        e.call_log = Some(vec![]);
        e.call(0x0056_1580, &args![0x3000u32, 0u8]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ENTITY_DEACTIVATE), vec![vec![0x4040]]);
        assert!(calls_to(&log, ENTITY_ACTIVATE).is_empty());
        // a body without an entity does nothing
        returns(&mut e, BODY_ENTITY, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_1580, &args![0x3000u32, 1u8]);
        assert_eq!(addresses(&e.call_log.take().unwrap()).len(), 2);
    }

    #[test]
    fn fn_00561730_tells_whether_all_lanes_are_within_the_tolerance() {
        let mut e = batch_engine();
        vector_doubles(&mut e);
        let this = vector4(&mut e, 1.0, 2.0, 3.0);
        let near = vector4(&mut e, 1.0005, 2.0, 3.0);
        let far = vector4(&mut e, 1.0, 2.0, 3.01);
        assert_eq!(e.call(0x0056_1730, &args![this, near, 0.001f32]).u32(), 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_1730, &args![this, far, 0.001f32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            ordered(
                &log,
                &[
                    VECTOR_SUBTRACT,
                    VECTOR_ABSOLUTE,
                    VECTOR_SPLAT,
                    VECTOR_COMPARE,
                    VECTOR_ALL_SET
                ]
            ),
            vec![
                VECTOR_SUBTRACT,
                VECTOR_ABSOLUTE,
                VECTOR_SPLAT,
                VECTOR_COMPARE,
                VECTOR_ALL_SET
            ]
        );
        // the subtraction is this - other, the splat the tolerance, and all
        // four bits of the comparison are required
        let sub = calls_to(&log, VECTOR_SUBTRACT);
        assert_eq!(&sub[0][1..], &[this, far]);
        assert_eq!(calls_to(&log, VECTOR_SPLAT)[0][1], 0.001f32.to_bits());
        assert_eq!(calls_to(&log, VECTOR_ALL_SET)[0][1], 0xf);
        // a lane past the tolerance in any position fails
        let off_in_w = e.mem.alloc(16);
        e.mem.write(off_in_w, &floats(&[1.0, 2.0, 3.0, 0.5]));
        assert_eq!(
            e.call(0x0056_1730, &args![this, off_in_w, 0.001f32]).u32(),
            0
        );
    }

    #[test]
    fn the_rigid_body_velocity_setters_only_act_on_a_changed_velocity() {
        let (mut e, _, entity, motion) = glue_world();
        let same = vector4(&mut e, 1.0, 2.0, 3.0);
        let near = vector4(&mut e, 1.0005, 2.0, 3.0);
        let far = vector4(&mut e, 4.0, 5.0, 6.0);
        e.call_log = Some(vec![]);
        e.call(0x0056_16d0, &args![entity, same]);
        e.call(0x0056_16d0, &args![entity, near]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ENTITY_ACTIVATE).is_empty());
        assert!(motion.linear.borrow().is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0056_16d0, &args![entity, far]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ENTITY_ACTIVATE), vec![vec![entity]]);
        assert_eq!(
            motion.linear.borrow().clone(),
            vec![floats(&[4.0, 5.0, 6.0])]
        );
        // the angular velocity is compared with (10, 11, 12) and set through
        // the other slot
        let angular_same = vector4(&mut e, 10.0, 11.0, 12.0);
        e.call(0x0056_1800, &args![entity, angular_same]);
        assert!(motion.angular.borrow().is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0056_1800, &args![entity, far]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ENTITY_ACTIVATE), vec![vec![entity]]);
        assert_eq!(
            motion.angular.borrow().clone(),
            vec![floats(&[4.0, 5.0, 6.0])]
        );
        assert_eq!(motion.linear.borrow().len(), 1);
    }

    #[test]
    fn the_body_level_velocity_setters_lock_around_the_entity_change() {
        let (mut e, body, entity, motion) = glue_world();
        let far = vector4(&mut e, 4.0, 5.0, 6.0);
        e.call_log = Some(vec![]);
        e.call(0x0056_1690, &args![body, far]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            ordered(&log, &[BODY_LOCK_MARKER, ENTITY_ACTIVATE]),
            vec![BODY_LOCK_MARKER, ENTITY_ACTIVATE, BODY_LOCK_MARKER]
        );
        assert_eq!(calls_to(&log, BODY_LOCK_MARKER), vec![vec![body]; 2]);
        assert_eq!(calls_to(&log, ENTITY_ACTIVATE), vec![vec![entity]]);
        assert_eq!(motion.linear.borrow().len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0056_17c0, &args![body, far]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, BODY_LOCK_MARKER), vec![vec![body]; 2]);
        assert_eq!(motion.angular.borrow().len(), 1);
        // a body without an entity is left alone
        let bare = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x0056_1690, &args![bare, far]);
        e.call(0x0056_17c0, &args![bare, far]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, BODY_LOCK_MARKER).is_empty());
    }

    #[test]
    fn the_point_setters_convert_the_vector_and_set_the_velocity() {
        let (mut e, body, _, motion) = glue_world();
        let point = e.mem.alloc(16);
        e.mem.write(point, &floats(&[7.0, 8.0, 9.0]));
        e.call(0x0056_15d0, &args![body, point]);
        assert_eq!(
            motion.linear.borrow().clone(),
            vec![floats(&[7.0, 8.0, 9.0])]
        );
        assert!(motion.angular.borrow().is_empty());
        e.call(0x0056_1630, &args![body, point]);
        assert_eq!(
            motion.angular.borrow().clone(),
            vec![floats(&[7.0, 8.0, 9.0])]
        );
    }

    // ---- 00561860: the older Havok record --------------------------------

    /// `havok_load_setup` without the stand-in for `00561860`: the same
    /// engine with the callees of the Havok record loaders stubbed.
    fn old_havok_setup(version: u8, stream: &[u8]) -> (Engine, u32) {
        let mut e = havok_engine();
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        e.mem.set_u8(game + 0x80, version);
        let buffer = e.mem.u32(game + 0x14);
        e.mem.write(buffer, stream);
        stub(
            &mut e,
            &[
                MESSAGE,
                SET_HAVOK_WEAPON,
                DISABLE_RAGDOLL_ANIM,
                KNOCK_DOWN,
                SET_MOTION,
                SET_FIXED,
                CLEAR_ANIM_GROUP,
                ANIM_FINISH,
                MAKE_VELOCITY,
                SET_3D_VELOCITY,
            ],
        );
        (e, buffer)
    }

    type Walks = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u8>)>>>;

    /// A collision walker double that records the callback and the context
    /// bytes of each walk and calls no callback.
    fn capture_walks(e: &mut Engine) -> Walks {
        let walks: Walks = Walks::default();
        let seen = walks.clone();
        e.register_double(WALK_COLLISION, move |e, a| {
            seen.borrow_mut().push((a[2], e.mem.bytes(a[1], 0x10)));
            Ret::default()
        });
        walks
    }

    #[test]
    fn fn_00561860_skips_a_record_without_3d_after_the_legacy_byte() {
        let (mut e, buffer) = old_havok_setup(0x10, &[1, 0xff]);
        returns(&mut e, GET_LOADED_3D, 0);
        let refr = big_refr(&mut e, false);
        e.mem.set_u32(refr.addr() + 0xc, 0x0006_0001);
        e.call_log = Some(vec![]);
        e.call(0x0056_1860, &args![refr, 12u16]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![HAVOK_NO_3D_FORMAT, 0x0e01, 0x0006_0001]]
        );
        // the legacy byte is read before the 3D is looked up (before 0x16),
        // then the record's size is skipped
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 1 + 12);
        // a later version reads nothing first
        let (mut e, buffer) = old_havok_setup(0x2b, &[]);
        returns(&mut e, GET_LOADED_3D, 0);
        let refr = big_refr(&mut e, false);
        e.call(0x0056_1860, &args![refr, 12u16]);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 12);
    }

    #[test]
    fn fn_00561860_with_a_different_bone_count_skips_the_record_and_knocks_an_actor_down() {
        // version 0x2b: flags byte 8, saved count 5; two bodies now
        let (mut e, buffer) = old_havok_setup(0x2b, &[8, 5]);
        let root = havok_node(&mut e, 0, Some(true), false);
        let first = havok_node(&mut e, 0, Some(true), false);
        install_walk(&mut e, vec![root, first]);
        returns(&mut e, COLLISION_ROOT, root);
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        let refr = big_refr(&mut e, true);
        e.mem.set_u32(refr.addr() + 0xc, 0x0006_0001);
        e.mem.set_u32(refr.addr() + 0xac, 0x5a5a);
        e.call_log = Some(vec![]);
        e.call(0x0056_1860, &args![refr, 20u16]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![HAVOK_BONE_COUNT_FORMAT, 0x0e01, 0x0006_0001, 5, 2]]
        );
        // the two bytes read, then size - 2 skipped
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 2 + 18);
        assert_eq!(calls_to(&log, DISABLE_RAGDOLL_ANIM), vec![vec![0x5a5a, 1]]);
        assert_eq!(
            calls_to(&log, KNOCK_DOWN),
            vec![vec![obj3d, ZERO_VECTOR, 1, 0.0f32.to_bits(), 0]]
        );
        // the bodies are not walked a second time
        assert!(calls_to(&log, MAKE_VELOCITY).is_empty());
        // a non-actor is only reported
        let (mut e, _) = old_havok_setup(0x2b, &[8, 5]);
        install_walk(&mut e, vec![root]);
        returns(&mut e, COLLISION_ROOT, root);
        returns(&mut e, GET_LOADED_3D, obj3d);
        let refr = big_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0056_1860, &args![refr, 20u16]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MESSAGE).len(), 1);
        assert!(calls_to(&log, KNOCK_DOWN).is_empty());
    }

    #[test]
    fn fn_00561860_with_the_same_count_walks_the_bodies_with_the_saved_flags() {
        let (mut e, buffer) = old_havok_setup(0x2b, &[1, 0]);
        let walks = capture_walks(&mut e);
        returns(&mut e, COLLISION_ROOT, 0x7007);
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        slot(&mut e, 0x234, |_, _| true.into_ret());
        slot(&mut e, 0x1e4, |_, _| 0x9009u32.into_ret());
        let refr = big_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0056_1860, &args![refr, 20u16]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MESSAGE).is_empty());
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 2);
        // the first walk counts (fn_00560870), the second loads (fn_00561b40)
        // with the context: recurse, mode 0x12, root, reference, flags, 0
        let walks = walks.borrow();
        assert_eq!(walks.len(), 2);
        assert_eq!(walks[0].0, 0x0056_0870);
        assert_eq!(walks[1].0, 0x0056_1b40);
        let context = calls_to(&log, WALK_COLLISION)[1][1];
        assert_eq!(e.mem.u8(context + 4), 1);
        assert_eq!(e.mem.u32(context + 8), 0x12);
        assert_eq!(e.mem.u32(context + 0xc), 0x7007);
        assert_eq!(e.mem.u32(context + 0x10), refr.addr());
        assert_eq!(e.mem.u32(context + 0x14), 1);
        assert_eq!(e.mem.u32(context + 0x18), 0);
        // a fixed actor has its bodies frozen and its animation group cleared
        assert_eq!(calls_to(&log, SET_FIXED), vec![vec![obj3d, 1, 1]]);
        assert_eq!(calls_to(&log, SET_MOTION), vec![vec![obj3d, 1, 1, 0, 1]]);
        assert_eq!(
            calls_to(&log, CLEAR_ANIM_GROUP),
            vec![vec![0x9009, 0x14, 0.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, ANIM_FINISH), vec![vec![0x9009]]);
        // and every reference gets the zero velocity record
        let made = calls_to(&log, MAKE_VELOCITY);
        assert_eq!(made.len(), 1);
        assert_eq!(&made[0][1..], &[0.0f32.to_bits(), 0, 0]);
        assert_eq!(
            calls_to(&log, SET_3D_VELOCITY),
            vec![vec![obj3d, made[0][0]]]
        );
        // a non-actor skips the freezing
        let (mut e, _) = old_havok_setup(0x2b, &[0, 0]);
        install_walk(&mut e, vec![]);
        returns(&mut e, COLLISION_ROOT, 0x7007);
        returns(&mut e, GET_LOADED_3D, obj3d);
        let refr = big_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0056_1860, &args![refr, 20u16]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SET_FIXED).is_empty());
        assert_eq!(calls_to(&log, MAKE_VELOCITY).len(), 1);
    }

    #[test]
    fn fn_00561860_reads_the_flags_the_way_each_version_saved_them() {
        // before 0x16: a byte before the 3D lookup that sets bit 0; no count
        // before 0x18, so a different count does not matter
        let (mut e, buffer) = old_havok_setup(0x10, &[1]);
        let walks = capture_walks(&mut e);
        returns(&mut e, COLLISION_ROOT, 0x7007);
        returns(&mut e, GET_LOADED_3D, 0x3d3d);
        let refr = big_refr(&mut e, false);
        e.call(0x0056_1860, &args![refr, 5u16]);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 1);
        assert_eq!(walks.borrow().len(), 1);
        // 0x16 to 0x2a: the same byte, read after the lookup
        let (mut e, buffer) = old_havok_setup(0x17, &[7]);
        let walks = capture_walks(&mut e);
        returns(&mut e, COLLISION_ROOT, 0x7007);
        returns(&mut e, GET_LOADED_3D, 0x3d3d);
        let refr = big_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0056_1860, &args![refr, 5u16]);
        let log = e.call_log.take().unwrap();
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + 1);
        let context = calls_to(&log, WALK_COLLISION)[0][1];
        assert_eq!(e.mem.u32(context + 0x14), 1);
        assert_eq!(walks.borrow().len(), 1);
        // 0x2b and later: the flags byte itself
        let (mut e, _) = old_havok_setup(0x2b, &[6, 0]);
        let _walks = capture_walks(&mut e);
        returns(&mut e, COLLISION_ROOT, 0x7007);
        returns(&mut e, GET_LOADED_3D, 0x3d3d);
        let refr = big_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0056_1860, &args![refr, 5u16]);
        let log = e.call_log.take().unwrap();
        let context = calls_to(&log, WALK_COLLISION)[1][1];
        assert_eq!(e.mem.u32(context + 0x14), 6);
    }

    // ---- 00561b40: the callback of the older record ------------------------

    /// An engine with a rich body for the older loading callback, the
    /// doubles for the velocities, and a walk context of `iFlags` `flags`
    /// whose reference is `refr` and root `root`.
    fn old_load_setup(
        version: u8,
        stream: &[u8],
        flags: u32,
        actor: bool,
    ) -> (Engine, u32, BodyLog, MotionLog, Ptr<OldHavokLoadWalk>, u32) {
        let (mut e, _) = old_havok_setup(version, &[]);
        let (body, log) = rich_body(&mut e, true, false);
        let motion = motion_doubles(&mut e, body);
        stub(&mut e, &[RAGDOLL_NODE_STEP]);
        let refr = batch_refr(&mut e, actor);
        let walk: Ptr<OldHavokLoadWalk> = e.new_object();
        e.set(walk, OldHavokLoadWalk::pRef, refr.cast());
        e.set(walk, OldHavokLoadWalk::iFlags, flags);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        let buffer = e.mem.u32(game + 0x14);
        e.mem.write(buffer, stream);
        (e, body, log, motion, walk, buffer)
    }

    #[test]
    fn fn_00561b40_reads_the_pose_and_the_velocities_from_the_stream() {
        let mut stream = floats(&[1.0, 2.0, 3.0]);
        stream.extend(floats(&[0.0, 0.0, 0.0, 1.0]));
        stream.extend(floats(&[4.0, 5.0, 6.0, 7.0, 8.0, 9.0]));
        let (mut e, body, log, motion, walk, buffer) = old_load_setup(0x51, &stream, 1, false);
        let node = node_of(&mut e, body);
        e.call_log = Some(vec![]);
        e.call(0x0056_1b40, &args![node, walk]);
        let calls = e.call_log.take().unwrap();
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + stream.len() as u32);
        assert_eq!(calls_to(&calls, RAGDOLL_NODE_STEP), vec![vec![node, 0]]);
        assert_eq!(log.position.borrow()[0], floats(&[1.0, 2.0, 3.0]));
        let rotation = lanes(&log.rotation.borrow()[0]);
        assert!(
            (rotation[3] - 1.0).abs() < 1e-3 && rotation[0] == 0.0,
            "{rotation:?}"
        );
        assert_eq!(
            motion.linear.borrow().clone(),
            vec![floats(&[4.0, 5.0, 6.0])]
        );
        assert_eq!(
            motion.angular.borrow().clone(),
            vec![floats(&[7.0, 8.0, 9.0])]
        );
        // the flag setter and the two changed velocities activate the entity
        assert_eq!(calls_to(&calls, ENTITY_ACTIVATE).len(), 3);
        // without bit 0 the velocities are the zero vector and nothing is read
        let mut stream = floats(&[1.0, 2.0, 3.0]);
        stream.extend(floats(&[0.0, 0.0, 0.0, 1.0]));
        let (mut e, body, _, motion, walk, buffer) = old_load_setup(0x51, &stream, 0, false);
        let node = node_of(&mut e, body);
        e.call_log = Some(vec![]);
        e.call(0x0056_1b40, &args![node, walk]);
        let calls = e.call_log.take().unwrap();
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer + stream.len() as u32);
        assert_eq!(motion.linear.borrow().clone(), vec![vec![0u8; 12]]);
        assert_eq!(motion.angular.borrow().clone(), vec![vec![0u8; 12]]);
        assert_eq!(calls_to(&calls, ENTITY_ACTIVATE).len(), 2);
    }

    #[test]
    fn fn_00561b40_takes_the_collision_roots_pose_from_the_reference() {
        // saves before 0x2b: the node that is the collision root
        let (mut e, body, log, motion, walk, buffer) = old_load_setup(0x20, &[], 0, false);
        let node = node_of(&mut e, body);
        e.set(walk, OldHavokLoadWalk::pCollisionRoot, Ptr::new(node));
        e.register(GET_ORIENTATION, |_, a| a[1].into_ret());
        e.register(QUATERNION_FROM_ROTATION, |e, a| {
            e.mem.write(a[0], &floats(&[0.0, 2.0, 0.0, 0.0]));
            a[0].into_ret()
        });
        let refr = e.get(walk, OldHavokLoadWalk::pRef);
        e.mem.write(refr.addr() + 0x30, &floats(&[9.0, 8.0, 7.0]));
        e.call(0x0056_1b40, &args![node, walk]);
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        // nothing is read from the stream for the pose or the velocities
        assert_eq!(e.mem.u32(game + 0x14), buffer);
        assert_eq!(log.position.borrow()[0], floats(&[9.0, 8.0, 7.0]));
        let rotation = lanes(&log.rotation.borrow()[0]);
        assert!((rotation[1] - 1.0).abs() < 1e-3, "{rotation:?}");
        assert_eq!(motion.linear.borrow().len(), 1);
        // a newer save asks flag 2 instead, and clears it
        let (mut e, body, log, _, walk, _) = old_load_setup(0x51, &[], 2, false);
        let node = node_of(&mut e, body);
        e.register(GET_ORIENTATION, |_, a| a[1].into_ret());
        e.register(QUATERNION_FROM_ROTATION, |e, a| {
            e.mem.write(a[0], &floats(&[0.0, 2.0, 0.0, 0.0]));
            a[0].into_ret()
        });
        e.call(0x0056_1b40, &args![node, walk]);
        assert_eq!(e.get(walk, OldHavokLoadWalk::iFlags), 0);
        assert_eq!(log.position.borrow().len(), 1);
        // an older save's other nodes read their pose
        let mut stream = floats(&[1.0, 2.0, 3.0]);
        stream.extend(floats(&[0.0, 0.0, 0.0, 1.0]));
        let (mut e, body, log, _, walk, _) = old_load_setup(0x20, &stream, 0, false);
        let node = node_of(&mut e, body);
        e.set(walk, OldHavokLoadWalk::pCollisionRoot, Ptr::new(0x1234));
        e.call(0x0056_1b40, &args![node, walk]);
        assert_eq!(log.position.borrow()[0], floats(&[1.0, 2.0, 3.0]));
    }

    #[test]
    fn fn_00561b40_skips_arrows_and_gives_an_actors_node_its_virtual() {
        let (mut e, body, log, _, walk, buffer) = old_load_setup(0x51, &[], 0, false);
        // an arrow: skipped even with a body
        let arrow_object = fake_sequence(&mut e, "Arrow", 0, 0);
        let arrow = node_of(&mut e, body);
        e.mem.set_u32(arrow + 8, arrow_object);
        e.call(0x0056_1b40, &args![arrow, walk]);
        assert!(log.position.borrow().is_empty());
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD);
        assert_eq!(e.mem.u32(game + 0x14), buffer);
        // a node without a body reads nothing
        let bare = havok_node(&mut e, 0, None, false);
        e.call(0x0056_1b40, &args![bare, walk]);
        assert!(log.position.borrow().is_empty());
        // an actor: an object of the skipped type ends it, another one makes
        // the node's virtual +0xb0 run with (1, 0, 0) first
        let (mut e, body, log, _, walk, _) = old_load_setup(0x51, &[0; 40], 0, true);
        let skipped = e.mem.alloc(0x40);
        e.mem.set_u8(skipped + 0x1e, 1);
        let node = node_of(&mut e, body);
        e.mem.set_u32(node + 8, skipped);
        e.call(0x0056_1b40, &args![node, walk]);
        assert!(log.position.borrow().is_empty());
        let plain = e.mem.alloc(0x40);
        e.mem.set_u32(node + 8, plain);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xb0, 0x00fb_00b0);
        e.mem.set_u32(node, vtable);
        e.register_double(0x00fb_00b0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0056_1b40, &args![node, walk]);
        let calls = e.call_log.take().unwrap();
        assert_eq!(calls_to(&calls, 0x00fb_00b0), vec![vec![node, 1, 0, 0]]);
        assert_eq!(log.position.borrow().len(), 1);
    }

    // ---- 00561d90 ---------------------------------------------------------

    #[test]
    fn original_open_state_is_the_extra_flag_inverted_when_the_change_is_recorded() {
        for (recorded, flag, expected) in [
            (false, false, false),
            (false, true, true),
            (true, false, true),
            (true, true, false),
        ] {
            let mut e = batch_engine();
            e.register(CHANGE_FLAGS_CONSTRUCT, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                a[0].into_ret()
            });
            e.set_global(GLOBAL_SAVE_LOAD_GAME, 0x7777u32);
            returns(&mut e, SAVE_LOAD_GET_CHANGE, u32::from(recorded));
            returns(&mut e, EXTRA_FLAG_TEST, u32::from(flag));
            let refr = new_refr(&mut e);
            e.call_log = Some(vec![]);
            let state = e.call(0x0056_1d90, &args![refr]).bool();
            let log = e.call_log.take().unwrap();
            assert_eq!(state, expected, "recorded {recorded}, flag {flag}");
            // the change asked about is the open state, 0x400000
            assert_eq!(
                calls_to(&log, SAVE_LOAD_GET_CHANGE),
                vec![vec![0x7777, refr.addr(), 0x0040_0000]]
            );
            assert_eq!(
                calls_to(&log, EXTRA_FLAG_TEST),
                vec![vec![refr.addr() + 0x44, 8]]
            );
        }
    }

    // ---- 00561df0 / 00561ef0: the editor location --------------------------

    /// A reference with an editor-location extra (position 1, 2, 3 at +0xc
    /// and rotation 10, 11, 12 at +0x18) and doubles that give it no
    /// starting space, no world space and no parent cell; returns the
    /// engine, the reference and three output slots (position, rotation,
    /// space).
    fn editor_location_setup() -> (Engine, Ptr<TESObjectREFR>, u32, u32, u32) {
        let mut e = batch_engine();
        let record = e.mem.alloc(0x40);
        for i in 0..3 {
            e.mem.set_f32(record + 0xc + 4 * i, 1.0 + i as f32);
            e.mem.set_f32(record + 0x18 + 4 * i, 10.0 + i as f32);
        }
        e.register_double(EXTRA_GET_DATA, move |_, a| {
            if a[1] == 0xf {
                record.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        returns(&mut e, EXTRA_GET_STARTING_SPACE, 0);
        returns(&mut e, GET_WORLD_SPACE, 0);
        let refr = new_refr(&mut e);
        let position = e.mem.alloc(16);
        let rotation = e.mem.alloc(16);
        let space = e.mem.alloc(16);
        (e, refr, position, rotation, space)
    }

    #[test]
    fn editor_location_needs_the_extra_and_copies_its_position_and_rotation() {
        let (mut e, refr, position, rotation, space) = editor_location_setup();
        returns(&mut e, EXTRA_GET_STARTING_SPACE, 0x5001);
        assert!(e
            .call(0x0056_1df0, &args![refr, position, rotation, space, 0u32])
            .bool());
        assert_eq!(lanes(&e.mem.bytes(position, 12)), vec![1.0, 2.0, 3.0]);
        assert_eq!(lanes(&e.mem.bytes(rotation, 12)), vec![10.0, 11.0, 12.0]);
        assert_eq!(e.mem.u32(space), 0x5001);
        // no editor-location extra: false, and nothing written
        e.register(EXTRA_GET_DATA, |_, _| 0u32.into_ret());
        let other = e.mem.alloc(16);
        assert!(!e
            .call(0x0056_1df0, &args![refr, other, other, other, 0u32])
            .bool());
        assert_eq!(e.mem.u32(other), 0);
    }

    #[test]
    fn editor_location_space_comes_from_the_extra_the_world_the_parent_cell_or_the_default() {
        // the reference's world space when the extra has none
        let (mut e, refr, position, rotation, space) = editor_location_setup();
        returns(&mut e, GET_WORLD_SPACE, 0x6006);
        assert!(e
            .call(0x0056_1df0, &args![refr, position, rotation, space, 0u32])
            .bool());
        assert_eq!(e.mem.u32(space), 0x6006);
        // else the parent cell
        let (mut e, refr, position, rotation, space) = editor_location_setup();
        e.set(refr, TESObjectREFR::pParentCell, Ptr::new(0x7007));
        assert!(e
            .call(0x0056_1df0, &args![refr, position, rotation, space, 0u32])
            .bool());
        assert_eq!(e.mem.u32(space), 0x7007);
        // else the default cell: none is false
        let (mut e, refr, position, rotation, space) = editor_location_setup();
        assert!(!e
            .call(0x0056_1df0, &args![refr, position, rotation, space, 0u32])
            .bool());
        // a cell (form type 0x39) that passes the cell test stands for itself
        let cell = form(&mut e, 0x39, 0x1111, 0);
        returns(&mut e, CELL_TEST, 1);
        returns(&mut e, CELL_GET_WORLD_SPACE, 0x8008);
        assert!(e
            .call(0x0056_1df0, &args![refr, position, rotation, space, cell])
            .bool());
        assert_eq!(e.mem.u32(space), cell);
        // one that fails it stands for its world space
        returns(&mut e, CELL_TEST, 0);
        assert!(e
            .call(0x0056_1df0, &args![refr, position, rotation, space, cell])
            .bool());
        assert_eq!(e.mem.u32(space), 0x8008);
        // any other form stands for itself
        let other = form(&mut e, 0x3a, 0x2222, 0);
        assert!(e
            .call(0x0056_1df0, &args![refr, position, rotation, space, other])
            .bool());
        assert_eq!(e.mem.u32(space), other);
    }

    /// An engine for `MoveToEditorLocation`: the reference's virtual +0x138
    /// reports position (4, 5, 6) and rotation (7, 8, 9) in the space
    /// 0x6006.
    fn move_to_editor_setup() -> (Engine, Ptr<TESObjectREFR>) {
        let mut e = batch_engine();
        slot(&mut e, 0x138, |e, a| {
            e.mem.write(a[1], &floats(&[4.0, 5.0, 6.0]));
            e.mem.write(a[2], &floats(&[7.0, 8.0, 9.0]));
            e.mem.set_u32(a[3], 0x6006);
            true.into_ret()
        });
        let refr = batch_refr(&mut e, false);
        let tes = e.mem.alloc(8);
        e.set_global(GLOBAL_TES, tes);
        // the space resolves to the world 0x7007 and no cell
        e.register(GET_LOCATION_CELL_OR_WORLD, |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[2], 0x7007);
            Ret::default()
        });
        returns(&mut e, WORLD_CELL_FROM_COORD, 0x8008);
        returns(&mut e, GET_REF_PERSISTS, 0);
        returns(&mut e, TES_IS_CELL_LOADED, 1);
        returns(&mut e, CELL_TEST, 0);
        returns(&mut e, GET_LOADED_3D, 0);
        stub(
            &mut e,
            &[
                SET_LOCATION_ON_REFERENCE,
                SET_ROTATION,
                MOVE_REF_TO_NEW_SPACE,
            ],
        );
        (e, refr)
    }

    #[test]
    fn move_to_editor_location_moves_the_reference_to_its_exterior_cell() {
        let (mut e, refr) = move_to_editor_setup();
        e.call_log = Some(vec![]);
        assert!(e.call(0x0056_1ef0, &args![refr, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        // the cell comes from the world and the position the editor gave
        let from_coord = calls_to(&log, WORLD_CELL_FROM_COORD);
        assert_eq!(from_coord.len(), 1);
        assert_eq!(from_coord[0][0], 0x7007);
        assert_eq!(calls_to(&log, GET_LOCATION_CELL_OR_WORLD)[0][0], 0x6006);
        assert_eq!(
            calls_to(&log, SET_LOCATION_ON_REFERENCE),
            vec![vec![refr.addr(), from_coord[0][1]]]
        );
        assert_eq!(
            calls_to(&log, SET_ROTATION),
            vec![vec![
                refr.addr(),
                7.0f32.to_bits(),
                8.0f32.to_bits(),
                9.0f32.to_bits()
            ]]
        );
        // an exterior cell: moved by world space, no cell
        assert_eq!(
            calls_to(&log, MOVE_REF_TO_NEW_SPACE),
            vec![vec![refr.addr(), 0, 0x7007]]
        );
    }

    #[test]
    fn move_to_editor_location_moves_the_reference_into_an_interior_cell() {
        let (mut e, refr) = move_to_editor_setup();
        e.register(GET_LOCATION_CELL_OR_WORLD, |e, a| {
            e.mem.set_u32(a[1], 0x9009);
            e.mem.set_u32(a[2], 0);
            Ret::default()
        });
        returns(&mut e, CELL_TEST, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0056_1ef0, &args![refr, 0x1234u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, WORLD_CELL_FROM_COORD).is_empty());
        assert_eq!(
            calls_to(&log, MOVE_REF_TO_NEW_SPACE),
            vec![vec![refr.addr(), 0x9009, 0]]
        );
        // the default cell was handed to the virtual
        assert_eq!(calls_to(&log, 0x00fd_0138)[0][4], 0x1234);
    }

    #[test]
    fn move_to_editor_location_needs_a_loaded_cell_unless_the_reference_persists() {
        // not persistent and the cell is not loaded: nothing is moved
        let (mut e, refr) = move_to_editor_setup();
        returns(&mut e, TES_IS_CELL_LOADED, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0056_1ef0, &args![refr, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, TES_IS_CELL_LOADED).len(), 1);
        assert!(calls_to(&log, SET_LOCATION_ON_REFERENCE).is_empty());
        // no cell at all
        returns(&mut e, WORLD_CELL_FROM_COORD, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0056_1ef0, &args![refr, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, TES_IS_CELL_LOADED).is_empty());
        // a persistent reference does not need one
        returns(&mut e, GET_REF_PERSISTS, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0056_1ef0, &args![refr, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MOVE_REF_TO_NEW_SPACE).len(), 1);
        // no editor location: false before anything else
        slot(&mut e, 0x138, |_, _| false.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0056_1ef0, &args![refr, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, GET_LOCATION_CELL_OR_WORLD).is_empty());
    }

    #[test]
    fn move_to_editor_location_ends_with_the_3d_position_update() {
        let (mut e, refr) = move_to_editor_setup();
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        stub(
            &mut e,
            &[
                SET_3D_LOCATION,
                SET_3D_ORIENTATION,
                UPDATE_POSITION,
                SET_3D_VELOCITY,
                MAKE_VELOCITY,
            ],
        );
        e.register(GET_ORIENTATION, |_, a| a[1].into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0056_1ef0, &args![refr, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SET_3D_LOCATION).len(), 1);
        assert_eq!(calls_to(&log, UPDATE_POSITION), vec![vec![obj3d, 1, 0]]);
    }

    // ---- 00562020, 005620e0 ------------------------------------------------

    #[test]
    fn update_3d_position_moves_the_controller_and_the_3d() {
        let mut e = batch_engine();
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        slot(&mut e, 0xfc, |_, _| true.into_ret());
        returns(&mut e, GET_CHARACTER_CONTROLLER, 0xc7c7);
        e.register(NI_TO_VECTOR, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        let positions = record_bytes(&mut e, CONTROLLER_SET_POSITION, 1, 12);
        stub(
            &mut e,
            &[
                SET_3D_LOCATION,
                SET_3D_ORIENTATION,
                UPDATE_POSITION,
                SET_3D_VELOCITY,
                MAKE_VELOCITY,
            ],
        );
        e.register(GET_ORIENTATION, |_, a| a[1].into_ret());
        let refr = batch_refr(&mut e, false);
        e.mem.write(refr.addr() + 0x30, &floats(&[1.0, 2.0, 3.0]));
        e.call_log = Some(vec![]);
        e.call(0x0056_2020, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            ordered(
                &log,
                &[
                    GET_CHARACTER_CONTROLLER,
                    CONTROLLER_SET_POSITION,
                    SET_3D_LOCATION,
                    GET_ORIENTATION,
                    SET_3D_ORIENTATION,
                    UPDATE_POSITION,
                    MAKE_VELOCITY,
                    SET_3D_VELOCITY
                ]
            ),
            vec![
                GET_CHARACTER_CONTROLLER,
                CONTROLLER_SET_POSITION,
                SET_3D_LOCATION,
                GET_ORIENTATION,
                SET_3D_ORIENTATION,
                UPDATE_POSITION,
                MAKE_VELOCITY,
                SET_3D_VELOCITY
            ]
        );
        assert_eq!(positions.borrow().clone(), vec![floats(&[1.0, 2.0, 3.0])]);
        assert_eq!(
            calls_to(&log, SET_3D_LOCATION),
            vec![vec![obj3d, refr.addr() + 0x30]]
        );
        let matrix = calls_to(&log, GET_ORIENTATION)[0][1];
        assert_eq!(
            calls_to(&log, SET_3D_ORIENTATION),
            vec![vec![obj3d, matrix]]
        );
        assert_eq!(calls_to(&log, UPDATE_POSITION), vec![vec![obj3d, 1, 0]]);
        let made = calls_to(&log, MAKE_VELOCITY);
        assert_eq!(&made[0][1..], &[0.0f32.to_bits(), 0, 0]);
        assert_eq!(
            calls_to(&log, SET_3D_VELOCITY),
            vec![vec![obj3d, made[0][0]]]
        );
    }

    #[test]
    fn update_3d_position_leaves_the_controller_alone_unless_mobile_with_one() {
        let mut e = batch_engine();
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        slot(&mut e, 0xfc, |_, _| false.into_ret());
        returns(&mut e, GET_CHARACTER_CONTROLLER, 0xc7c7);
        stub(
            &mut e,
            &[
                CONTROLLER_SET_POSITION,
                SET_3D_LOCATION,
                SET_3D_ORIENTATION,
                UPDATE_POSITION,
                SET_3D_VELOCITY,
                MAKE_VELOCITY,
            ],
        );
        e.register(GET_ORIENTATION, |_, a| a[1].into_ret());
        let refr = batch_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0056_2020, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, GET_CHARACTER_CONTROLLER).is_empty());
        assert_eq!(calls_to(&log, UPDATE_POSITION).len(), 1);
        // mobile but without a controller
        slot(&mut e, 0xfc, |_, _| true.into_ret());
        returns(&mut e, GET_CHARACTER_CONTROLLER, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2020, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, CONTROLLER_SET_POSITION).is_empty());
        assert_eq!(calls_to(&log, UPDATE_POSITION).len(), 1);
        // no 3D: nothing at all
        returns(&mut e, GET_LOADED_3D, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2020, &args![refr]);
        assert_eq!(addresses(&e.call_log.take().unwrap()).len(), 2);
    }

    #[test]
    fn character_controller_position_is_converted_to_a_havok_vector() {
        let mut e = batch_engine();
        e.register(NI_TO_VECTOR, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        let vectors = record_bytes(&mut e, CONTROLLER_SET_POSITION, 1, 12);
        let point = e.mem.alloc(16);
        e.mem.write(point, &floats(&[3.0, 2.0, 1.0]));
        e.call_log = Some(vec![]);
        e.call(0x0056_20e0, &args![0xc7c7u32, point]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CONTROLLER_SET_POSITION)[0][0], 0xc7c7);
        assert_eq!(vectors.borrow().clone(), vec![floats(&[3.0, 2.0, 1.0])]);
    }

    // ---- 00562140 and the change-flag helpers -----------------------------

    /// An engine with the readers of the buffer's change flags (header at
    /// +0x17, old at +0x2c) and the flag test working on memory, and a
    /// save-load game object at its global; see `buffer_with`.
    fn buffer_engine() -> Engine {
        let mut e = batch_engine();
        e.register(BUFFER_CHANGE_FLAGS, |e, a| {
            let flags = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], flags);
            a[1].into_ret()
        });
        e.register(BUFFER_OLD_CHANGE_FLAGS, |e, a| {
            let flags = e.mem.u32(a[0] + 0x2c);
            e.mem.set_u32(a[1], flags);
            a[1].into_ret()
        });
        e.register(CHANGE_FLAGS_TEST, |e, a| {
            (e.mem.u32(a[0]) & a[1] != 0).into_ret()
        });
        let game = e.mem.alloc(0x300);
        e.set_global(GLOBAL_SAVE_LOAD_GAME, game);
        e
    }

    fn buffer_with(e: &mut Engine, header: u32, old: u32, own: u32) -> u32 {
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(buffer + 0x17, header);
        e.mem.set_u32(buffer + 0x2c, old);
        e.mem.set_u32(buffer + 0x28, own);
        buffer
    }

    #[test]
    fn fn_00562140_clears_the_animation_change_unless_something_keeps_it() {
        let mut e = buffer_engine();
        stub(&mut e, &[EMPTY_BUFFER_STEP]);
        returns(&mut e, GET_LOADED_3D, 0x3d3d);
        returns(&mut e, EXTRA_GET_LAST_SEQUENCE, 0);
        let refr = batch_refr(&mut e, false);
        let buffer = buffer_with(&mut e, 0x1000_0040, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2140, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, EMPTY_BUFFER_STEP),
            vec![vec![refr.addr(), buffer]]
        );
        // only the 0x10000000 bit goes
        assert_eq!(e.mem.u32(buffer + 0x17), 0x40);
        // without the bit, nothing happens
        let buffer = buffer_with(&mut e, 0x40, 0, 0);
        e.call(0x0056_2140, &args![refr, buffer]);
        assert_eq!(e.mem.u32(buffer + 0x17), 0x40);
        // a sequence left to save keeps it
        returns(&mut e, EXTRA_GET_LAST_SEQUENCE, 0x5151);
        let buffer = buffer_with(&mut e, 0x1000_0040, 0, 0);
        e.call(0x0056_2140, &args![refr, buffer]);
        assert_eq!(e.mem.u32(buffer + 0x17), 0x1000_0040);
        returns(&mut e, EXTRA_GET_LAST_SEQUENCE, 0);
        // so does the save-load game's global flag 4
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD_GAME);
        e.mem.set_u32(game + 0x244, 4);
        let buffer = buffer_with(&mut e, 0x1000_0040, 0, 0);
        e.call(0x0056_2140, &args![refr, buffer]);
        assert_eq!(e.mem.u32(buffer + 0x17), 0x1000_0040);
        e.mem.set_u32(game + 0x244, 0);
        // a reference without a 3D
        returns(&mut e, GET_LOADED_3D, 0);
        let buffer = buffer_with(&mut e, 0x1000_0040, 0, 0);
        e.call(0x0056_2140, &args![refr, buffer]);
        assert_eq!(e.mem.u32(buffer + 0x17), 0x1000_0040);
        returns(&mut e, GET_LOADED_3D, 0x3d3d);
        // and an actor
        let actor = batch_refr(&mut e, true);
        let buffer = buffer_with(&mut e, 0x1000_0040, 0, 0);
        e.call(0x0056_2140, &args![actor, buffer]);
        assert_eq!(e.mem.u32(buffer + 0x17), 0x1000_0040);
    }

    #[test]
    fn the_change_flag_helpers_test_and_clear_bits() {
        let mut e = batch_engine();
        let game = e.mem.alloc(0x300);
        e.mem.set_u32(game + 0x244, 0xb);
        assert!(!e.call(0x0056_21d0, &args![game]).bool());
        e.mem.set_u32(game + 0x244, 0x5);
        assert!(e.call(0x0056_21d0, &args![game]).bool());
        let flags = e.mem.alloc(8);
        e.mem.set_u32(flags, 0xff);
        e.call(0x0056_2210, &args![flags, 0x0fu32]);
        assert_eq!(e.mem.u32(flags), 0xf0);
        // the buffer's change flags are at +0x17
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(buffer + 0x17, 0x1000_0003);
        e.call(0x0056_21f0, &args![buffer, 0x1000_0000u32]);
        assert_eq!(e.mem.u32(buffer + 0x17), 3);
        // the buffer's own flags: bit 4 at +0x28
        e.mem.set_u32(buffer + 0x28, 4);
        assert!(e.call(0x0056_2d00, &args![buffer]).bool());
        e.mem.set_u32(buffer + 0x28, 0xb);
        assert!(!e.call(0x0056_2d00, &args![buffer]).bool());
    }

    // ---- the buffer-based save and load handlers ---------------------------

    /// `buffer_engine` with stand-ins for everything the buffer handlers
    /// call: stubs for the writers and loaders, the inventory changes at
    /// 0x1c1c, no loaded 3D, the shadow-scene and model-loader globals.
    fn buffer_handler_engine() -> Engine {
        let mut e = buffer_engine();
        stub(
            &mut e,
            &[
                FORM_SAVE_GAME_BUFFER,
                FORM_LOAD_GAME_BUFFER,
                EMPTY_BUFFER_STEP,
                BUFFER_SAVE_DATA,
                BUFFER_LOAD_DATA,
                EXTRA_SAVE_GAME_OV2,
                EXTRA_LOAD_GAME_OV2,
                EXTRA_LOAD_GAME_WITH_BASE,
                EXTRA_FINISH_LOAD_GAME,
                EXTRA_LOAD_STEP,
                EXTRA_LOAD_LAST_STEP,
                EXTRA_STEP_BEFORE_CHANGES,
                INVENTORY_CHANGES_SAVE_GAME,
                INVENTORY_CHANGES_LOAD_GAME,
                INVENTORY_CHANGES_FINISH,
                QUEUE_SUB_BUFFER,
                FORM_SET_EMPTY,
                FORM_SET_DISABLED,
                SET_SCALE,
                UPDATE_ADDON_NODE_SOUNDS,
                TERRAIN_HIDE_TREE,
                CELL_ADD_ACTIVATING_REF,
                CELL_REFERENCE_STEP,
                RESET_STATE,
                SET_OPEN_STATE,
                RESTORE_RAGDOLL_DATA,
                EXTRA_FLAG_SET,
                EXTRA_FLAG_CLEAR,
                REFR_CLEAR_ACTION,
                REFR_SET_ACTION,
                SNAP_HAVOK_TO_3D,
                OBSTACLE_ON_DOOR_CLOSE,
                OBSTACLE_ON_DOOR_OPEN,
                MODEL_LOADER_QUEUE_REFERENCE,
                WORLD_GET_TERRAIN_MANAGER,
            ],
        );
        returns(&mut e, GET_INVENTORY_CHANGES, 0x1c1c);
        returns(&mut e, GET_OPEN_STATE, 0);
        returns(&mut e, BASE_FORM_TEST, 0);
        returns(&mut e, GET_WORLD_SPACE, 0);
        returns(&mut e, EXTRA_FLAG_TEST, 0);
        returns(&mut e, REFR_HEALTH_GATE, 0);
        returns(&mut e, GET_LOADED_3D, 0);
        returns(&mut e, GET_OBSTACLE_MANAGER, 0x0b0b);
        returns(&mut e, SAVE_LOAD_GAME_TEST, 0);
        returns(&mut e, REFR_3D_TEST, 0);
        e.register(CHANGE_FLAGS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(CHECKED_CAST, |_, a| a[1].into_ret());
        let tes = e.mem.alloc(8);
        e.set_global(GLOBAL_TES, tes);
        let loader = e.mem.alloc(8);
        e.set_global(GLOBAL_MODEL_LOADER, loader);
        e
    }

    /// A reference whose virtual +0x1d0 (the loaded 3D) gives null.
    fn handler_refr(e: &mut Engine, actor: bool) -> Ptr<TESObjectREFR> {
        slot(e, 0x1d0, |_, _| 0u32.into_ret());
        batch_refr(e, actor)
    }

    // ---- 00562230 -----------------------------------------------------------

    #[test]
    fn save_game_ov2_writes_each_part_for_its_change_flag() {
        let mut e = buffer_handler_engine();
        let writes = record_writes(&mut e, BUFFER_SAVE_DATA);
        let refr = handler_refr(&mut e, false);
        e.mem.write(refr.addr() + 0x3c, &floats(&[2.5]));
        let buffer = buffer_with(&mut e, 0x10 | 0x40 | 0x20, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2230, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FORM_SAVE_GAME_BUFFER),
            vec![vec![refr.addr(), buffer]]
        );
        // the scale: four bytes at +0x3c
        assert_eq!(writes.borrow().clone(), vec![floats(&[2.5])]);
        assert_eq!(
            calls_to(&log, BUFFER_SAVE_DATA),
            vec![vec![buffer, refr.addr() + 0x3c, 4, 0]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_SAVE_GAME_OV2),
            vec![vec![refr.addr() + 0x44, buffer]]
        );
        assert_eq!(
            calls_to(&log, INVENTORY_CHANGES_SAVE_GAME),
            vec![vec![0x1c1c, buffer]]
        );
        // no flags, no parts
        let buffer = buffer_with(&mut e, 0x2, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2230, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log)
                .into_iter()
                .filter(|a| *a == BUFFER_SAVE_DATA || *a == EXTRA_SAVE_GAME_OV2)
                .count(),
            0
        );
        // 0x40000 is an extra-data flag only for an actor
        let buffer = buffer_with(&mut e, 0x4_0000, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2230, &args![refr, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), EXTRA_SAVE_GAME_OV2).is_empty());
        let actor = handler_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0056_2230, &args![actor, buffer]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), EXTRA_SAVE_GAME_OV2).len(),
            1
        );
    }

    /// The doubles the animation part of `SaveGame` needs: the saved
    /// animation `animation`, a loaded 3D `obj3d`, a buffer position at
    /// +0xc that `SaveControllerManager` advances by 10, a start token.
    fn animation_save_engine(animation: u32, obj3d: u32) -> Engine {
        let mut e = buffer_handler_engine();
        returns(&mut e, EXTRA_GET_SAVED_ANIMATION, animation);
        returns(&mut e, GET_LOADED_3D, obj3d);
        returns(&mut e, BUFFER_START_VARIABLE, 0x7070);
        returns(&mut e, GET_CONTROLLER, 0);
        e.register(CHECKED_CAST, |_, a| a[1].into_ret());
        returns(&mut e, EXTRA_GET_LAST_SEQUENCE, 0x5151);
        stub(
            &mut e,
            &[
                BUFFER_SAVE_STRING,
                BUFFER_SAVE_VARIABLE_OV2,
                SUB_BUFFER_SAVE_GAME,
            ],
        );
        e.register(SAVE_CONTROLLER_MANAGER, |e, a| {
            let position = e.mem.u32(a[1] + 0xc);
            e.mem.set_u32(a[1] + 0xc, position + 10);
            Ret::default()
        });
        e
    }

    #[test]
    fn save_game_ov2_saves_the_animation_as_a_sized_value() {
        let mut e = animation_save_engine(0, 0x3d3d);
        let refr = handler_refr(&mut e, false);
        let buffer = buffer_with(&mut e, 0x1000_0000, 0, 0);
        e.mem.set_u32(buffer + 0xc, 100);
        e.call_log = Some(vec![]);
        e.call(0x0056_2230, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            ordered(
                &log,
                &[
                    BUFFER_START_VARIABLE,
                    BUFFER_SAVE_STRING,
                    SAVE_CONTROLLER_MANAGER,
                    BUFFER_SAVE_VARIABLE_OV2
                ]
            ),
            vec![
                BUFFER_START_VARIABLE,
                BUFFER_SAVE_STRING,
                SAVE_CONTROLLER_MANAGER,
                BUFFER_SAVE_VARIABLE_OV2
            ]
        );
        // the last finished sequence name, then the manager with the -1.0 blend
        assert_eq!(
            calls_to(&log, BUFFER_SAVE_STRING),
            vec![vec![buffer, 0x5151, 0]]
        );
        assert_eq!(
            calls_to(&log, SAVE_CONTROLLER_MANAGER),
            vec![vec![0, buffer, 0xbf80_0000]]
        );
        // the size of what the animation wrote, and the token
        assert_eq!(
            calls_to(&log, BUFFER_SAVE_VARIABLE_OV2),
            vec![vec![buffer, 10, 0x7070]]
        );
    }

    #[test]
    fn save_game_ov2_hands_a_3d_less_animation_to_a_sub_buffer() {
        // no 3D but a saved animation: the sub buffer saves it
        let mut e = animation_save_engine(0x5252, 0);
        let refr = handler_refr(&mut e, false);
        let buffer = buffer_with(&mut e, 0x1000_0000, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2230, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        let sub = calls_to(&log, SUB_BUFFER_SAVE_GAME);
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0][1], buffer);
        // the cell it is called on holds the saved animation
        assert_eq!(calls_to(&log, CHANGE_FLAGS_CONSTRUCT)[0][1], 0x5252);
        assert!(calls_to(&log, BUFFER_START_VARIABLE).is_empty());
        // no 3D and no animation: the sized value is written anyway
        let mut e = animation_save_engine(0, 0);
        let refr = handler_refr(&mut e, false);
        let buffer = buffer_with(&mut e, 0x1000_0000, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2230, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, BUFFER_SAVE_VARIABLE_OV2).len(), 1);
        // a 3D with an animation: also the sized value
        let mut e = animation_save_engine(0x5252, 0x3d3d);
        let refr = handler_refr(&mut e, false);
        let buffer = buffer_with(&mut e, 0x1000_0000, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2230, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SUB_BUFFER_SAVE_GAME).is_empty());
        assert_eq!(calls_to(&log, BUFFER_SAVE_VARIABLE_OV2).len(), 1);
        // an actor does not save the animation
        let actor = handler_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0056_2230, &args![actor, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), BUFFER_START_VARIABLE).is_empty());
    }

    // ---- 005623d0 -----------------------------------------------------------

    #[test]
    fn fn_005623d0_loads_each_part_for_its_change_flag() {
        let mut e = buffer_handler_engine();
        let refr = handler_refr(&mut e, false);
        e.mem.write(refr.addr() + 0x3c, &floats(&[2.5]));
        let buffer = buffer_with(
            &mut e,
            0x0020_0000 | 0x10 | 0x40 | 0x20 | 0x1000_0000 | 0x0040_0000,
            0,
            0,
        );
        e.call_log = Some(vec![]);
        e.call(0x0056_23d0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FORM_LOAD_GAME_BUFFER),
            vec![vec![refr.addr(), buffer]]
        );
        assert_eq!(calls_to(&log, FORM_SET_EMPTY), vec![vec![refr.addr(), 1]]);
        assert_eq!(
            calls_to(&log, BUFFER_LOAD_DATA),
            vec![vec![buffer, refr.addr() + 0x3c, 4]]
        );
        assert_eq!(
            calls_to(&log, SET_SCALE),
            vec![vec![refr.addr(), 2.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_LOAD_GAME_OV2),
            vec![vec![refr.addr() + 0x44, buffer]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_STEP_BEFORE_CHANGES),
            vec![vec![refr.addr() + 0x44]]
        );
        assert_eq!(
            calls_to(&log, INVENTORY_CHANGES_LOAD_GAME),
            vec![vec![0x1c1c, buffer]]
        );
        let game = e.global::<u32>(GLOBAL_SAVE_LOAD_GAME);
        assert_eq!(
            calls_to(&log, QUEUE_SUB_BUFFER),
            vec![vec![game, 0, buffer, 0]]
        );
        // the open-state flag: the list does not have it, so it is cleared
        assert_eq!(
            calls_to(&log, EXTRA_FLAG_TEST),
            vec![vec![refr.addr() + 0x44, 8]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_FLAG_CLEAR),
            vec![vec![refr.addr() + 0x44, 8]]
        );
    }

    #[test]
    fn fn_005623d0_leaves_the_non_actor_parts_to_other_references() {
        let mut e = buffer_handler_engine();
        let actor = handler_refr(&mut e, true);
        let buffer = buffer_with(
            &mut e,
            0x0020_0000 | 0x1000_0000 | 0x0040_0000 | 0x4_0000,
            0,
            0,
        );
        e.call_log = Some(vec![]);
        e.call(0x0056_23d0, &args![actor, buffer]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FORM_SET_EMPTY).is_empty());
        assert!(calls_to(&log, QUEUE_SUB_BUFFER).is_empty());
        assert!(calls_to(&log, EXTRA_FLAG_TEST).is_empty());
        // 0x40000 is an extra-data flag for an actor
        assert_eq!(calls_to(&log, EXTRA_LOAD_GAME_OV2).len(), 1);
    }

    #[test]
    fn fn_005623d0_applies_the_destruction_stage_when_there_is_a_health() {
        let mut e = buffer_handler_engine();
        returns(&mut e, REFR_HEALTH_GATE, 1);
        e.register(EXTRA_GET_OBJECT_HEALTH, |_, _| Ret {
            st0: 50.0,
            ..Ret::default()
        });
        returns(&mut e, GET_DESTRUCTION_FORM, 0xd0d0);
        returns(&mut e, UPDATE_DAMAGE_STAGE, 1);
        let refr = handler_refr(&mut e, false);
        let loaded = e.mem.alloc(0x20);
        e.mem.set_u32(loaded + 0x14, 0x3d3d);
        e.set(refr, TESObjectREFR::pLoadedData, Ptr::new(loaded));
        let base = e.mem.alloc(0x20);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(base),
        );
        let buffer = buffer_with(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_23d0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, GET_DESTRUCTION_FORM), vec![vec![base]]);
        assert_eq!(
            calls_to(&log, UPDATE_DAMAGE_STAGE),
            vec![vec![0xd0d0, refr.addr(), 0]]
        );
        assert_eq!(
            calls_to(&log, UPDATE_ADDON_NODE_SOUNDS),
            vec![vec![refr.addr(), 0]]
        );
        // a stage that did not change leaves the sounds alone
        returns(&mut e, UPDATE_DAMAGE_STAGE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_23d0, &args![refr, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), UPDATE_ADDON_NODE_SOUNDS).is_empty());
        // a health of -1.0 or no base object skips the step
        returns(&mut e, UPDATE_DAMAGE_STAGE, 1);
        e.register(EXTRA_GET_OBJECT_HEALTH, |_, _| Ret {
            st0: -1.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0056_23d0, &args![refr, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), UPDATE_DAMAGE_STAGE).is_empty());
        e.register(EXTRA_GET_OBJECT_HEALTH, |_, _| Ret {
            st0: 50.0,
            ..Ret::default()
        });
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(0),
        );
        e.call_log = Some(vec![]);
        e.call(0x0056_23d0, &args![refr, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), UPDATE_DAMAGE_STAGE).is_empty());
    }

    // ---- 00562660 -----------------------------------------------------------

    #[test]
    fn fn_00562660_follows_the_enable_parent_and_loads_extras_and_inventory() {
        let mut e = buffer_handler_engine();
        let parent = form(&mut e, 0x3a, 0x2222, 0x800);
        returns(&mut e, ENABLE_PARENT, parent);
        returns(&mut e, FOLLOWS_ENABLE_PARENT, 0);
        let refr = handler_refr(&mut e, false);
        let base = form(&mut e, 0x3b, 0x3333, 0);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(base),
        );
        let buffer = buffer_with(&mut e, 0x40 | 0x20, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2660, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, EMPTY_BUFFER_STEP),
            vec![vec![refr.addr(), buffer]]
        );
        // a disabled parent that is followed as it is disables the reference
        assert_eq!(
            calls_to(&log, FORM_SET_DISABLED),
            vec![vec![refr.addr(), 1]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_LOAD_GAME_WITH_BASE),
            vec![vec![refr.addr() + 0x44, buffer, base]]
        );
        assert_eq!(
            calls_to(&log, INVENTORY_CHANGES_FINISH),
            vec![vec![0x1c1c, buffer]]
        );
        // followed inverted: the opposite
        returns(&mut e, FOLLOWS_ENABLE_PARENT, 1);
        e.call_log = Some(vec![]);
        e.call(0x0056_2660, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FORM_SET_DISABLED),
            vec![vec![refr.addr(), 0]]
        );
        // no parent: no change; no flags: no loading
        returns(&mut e, ENABLE_PARENT, 0);
        let buffer = buffer_with(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2660, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FORM_SET_DISABLED).is_empty());
        assert!(calls_to(&log, EXTRA_LOAD_GAME_WITH_BASE).is_empty());
        assert!(calls_to(&log, INVENTORY_CHANGES_FINISH).is_empty());
    }

    #[test]
    fn fn_00562660_hides_the_terrain_tree_of_a_disabled_object() {
        let mut e = buffer_handler_engine();
        returns(&mut e, ENABLE_PARENT, 0);
        returns(&mut e, BASE_FORM_TEST, 1);
        returns(&mut e, GET_WORLD_SPACE, 0x6006);
        returns(&mut e, WORLD_GET_TERRAIN_MANAGER, 0x7a7a);
        let refr = handler_refr(&mut e, false);
        let base = form(&mut e, 0x3b, 0x3333, 0);
        e.set(
            refr.at(TESObjectREFR::data),
            OBJ_REFR::pObjectReference,
            Ptr::new(base),
        );
        let buffer = buffer_with(&mut e, 0, 0, 0);
        // the reference is not disabled
        e.call_log = Some(vec![]);
        e.call(0x0056_2660, &args![refr, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), TERRAIN_HIDE_TREE).is_empty());
        // disabled (flag 0x800 at +8)
        e.mem.set_u32(refr.addr() + 8, 0x800);
        e.call_log = Some(vec![]);
        e.call(0x0056_2660, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, WORLD_GET_TERRAIN_MANAGER),
            vec![vec![0x6006]]
        );
        assert_eq!(
            calls_to(&log, TERRAIN_HIDE_TREE),
            vec![vec![0x7a7a, refr.addr(), 1]]
        );
        // the base form fails its test, or there is no world space
        returns(&mut e, BASE_FORM_TEST, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2660, &args![refr, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), TERRAIN_HIDE_TREE).is_empty());
        returns(&mut e, BASE_FORM_TEST, 1);
        returns(&mut e, GET_WORLD_SPACE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_2660, &args![refr, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), TERRAIN_HIDE_TREE).is_empty());
    }

    // ---- 005627c0 -----------------------------------------------------------

    #[test]
    fn finish_load_game_puts_the_open_state_back_for_a_non_actor() {
        // recorded change absent and the extra flag set: originally open
        let mut e = buffer_handler_engine();
        returns(&mut e, SAVE_LOAD_GET_CHANGE, 0);
        returns(&mut e, EXTRA_FLAG_TEST, 1);
        let refr = handler_refr(&mut e, false);
        let buffer = buffer_with(&mut e, 0x40 | 0x0080_0000, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_27c0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, EMPTY_BUFFER_STEP),
            vec![vec![refr.addr(), buffer]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_FINISH_LOAD_GAME),
            vec![vec![refr.addr() + 0x44, buffer]]
        );
        assert_eq!(
            calls_to(&log, REFR_CLEAR_ACTION),
            vec![vec![refr.addr(), 4]]
        );
        assert_eq!(calls_to(&log, SNAP_HAVOK_TO_3D), vec![vec![refr.addr()]]);
        assert_eq!(
            calls_to(&log, OBSTACLE_ON_DOOR_CLOSE),
            vec![vec![0x0b0b, refr.addr()]]
        );
        assert!(calls_to(&log, REFR_SET_ACTION).is_empty());
        // the other way round
        returns(&mut e, EXTRA_FLAG_TEST, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_27c0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, REFR_SET_ACTION), vec![vec![refr.addr(), 4]]);
        assert_eq!(
            calls_to(&log, OBSTACLE_ON_DOOR_OPEN),
            vec![vec![0x0b0b, refr.addr()]]
        );
        assert!(calls_to(&log, REFR_CLEAR_ACTION).is_empty());
        // an actor does not
        let actor = handler_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0056_27c0, &args![actor, buffer]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SNAP_HAVOK_TO_3D).is_empty());
    }

    #[test]
    fn finish_load_game_adds_the_activating_reference_and_queues_the_3d() {
        let mut e = buffer_handler_engine();
        returns(&mut e, CELL_PRIORITY_TEST, 1);
        returns(&mut e, TES_GET_CELL_PRIORITY, 5);
        let refr = handler_refr(&mut e, false);
        e.set(refr, TESObjectREFR::pParentCell, Ptr::new(0x6006));
        let buffer = buffer_with(&mut e, 0x0400_0000, 0, 0);
        let tes = e.global::<u32>(GLOBAL_TES);
        let loader = e.global::<u32>(GLOBAL_MODEL_LOADER);
        e.call_log = Some(vec![]);
        e.call(0x0056_27c0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CELL_ADD_ACTIVATING_REF),
            vec![vec![0x6006, refr.addr()]]
        );
        // no 3D and a cell that passes the test: queued with its priority
        assert_eq!(
            calls_to(&log, TES_GET_CELL_PRIORITY),
            vec![vec![tes, 0x6006, 0]]
        );
        assert_eq!(
            calls_to(&log, MODEL_LOADER_QUEUE_REFERENCE),
            vec![vec![loader, refr.addr(), 5, 1]]
        );
        // a cell that fails the test, or a reference that has a 3D, is not
        returns(&mut e, CELL_PRIORITY_TEST, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_27c0, &args![refr, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), MODEL_LOADER_QUEUE_REFERENCE).is_empty());
        returns(&mut e, CELL_PRIORITY_TEST, 1);
        slot(&mut e, 0x1d0, |_, _| 0x3d3du32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0056_27c0, &args![refr, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), MODEL_LOADER_QUEUE_REFERENCE).is_empty());
        slot(&mut e, 0x1d0, |_, _| 0u32.into_ret());
        // the virtual +0x1cc instead when 00437b90 holds
        returns(&mut e, REFR_3D_TEST, 1);
        e.call_log = Some(vec![]);
        e.call(0x0056_27c0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00fd_01cc), vec![vec![refr.addr(), 0, 0]]);
        assert!(calls_to(&log, MODEL_LOADER_QUEUE_REFERENCE).is_empty());
        // the save-load game can say to do none of it
        returns(&mut e, SAVE_LOAD_GAME_TEST, 1);
        e.call_log = Some(vec![]);
        e.call(0x0056_27c0, &args![refr, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), REFR_3D_TEST).is_empty());
    }

    // ---- 005629a0 -----------------------------------------------------------

    #[test]
    fn fn_005629a0_resets_the_radius_and_loads_with_the_old_change_flags() {
        let mut e = buffer_handler_engine();
        let refr = handler_refr(&mut e, false);
        let loaded = e.mem.alloc(0x20);
        e.set(refr, TESObjectREFR::pLoadedData, Ptr::new(loaded));
        e.register(EXTRA_GET_OBJECT_HEALTH, |_, _| Ret {
            st0: -1.0,
            ..Ret::default()
        });
        // header flags are ignored: only the old change flags count
        let buffer = buffer_with(&mut e, 0xffff_ffff, 0x0020_0000 | 0x40, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(loaded + 0xc), -1.0);
        assert_eq!(calls_to(&log, FORM_SET_EMPTY), vec![vec![refr.addr(), 0]]);
        assert_eq!(
            calls_to(&log, EXTRA_LOAD_STEP),
            vec![vec![refr.addr() + 0x44, buffer]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_LOAD_LAST_STEP),
            vec![vec![refr.addr() + 0x44, buffer]]
        );
        assert!(calls_to(&log, UPDATE_DAMAGE_STAGE).is_empty());
        // an actor is not emptied and its extra-data mask is wider
        let actor = handler_refr(&mut e, true);
        let buffer = buffer_with(&mut e, 0, 0x0020_0000 | 0x4_0000, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![actor, buffer]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FORM_SET_EMPTY).is_empty());
        assert_eq!(calls_to(&log, EXTRA_LOAD_STEP).len(), 1);
    }

    #[test]
    fn fn_005629a0_updates_the_damage_stage_for_an_object_with_a_health() {
        let mut e = buffer_handler_engine();
        returns(&mut e, REFR_HEALTH_GATE, 1);
        e.register(EXTRA_GET_OBJECT_HEALTH, |_, _| Ret {
            st0: 50.0,
            ..Ret::default()
        });
        returns(&mut e, GET_DESTRUCTION_FORM, 0xd0d0);
        returns(&mut e, UPDATE_DAMAGE_STAGE, 1);
        let refr = handler_refr(&mut e, false);
        let loaded = e.mem.alloc(0x20);
        e.mem.set_u32(loaded + 0x14, 0x3d3d);
        e.set(refr, TESObjectREFR::pLoadedData, Ptr::new(loaded));
        let buffer = buffer_with(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        // this version makes the call without a base-form check and with 1
        assert_eq!(
            calls_to(&log, UPDATE_DAMAGE_STAGE),
            vec![vec![0xd0d0, refr.addr(), 1]]
        );
        assert_eq!(
            calls_to(&log, UPDATE_ADDON_NODE_SOUNDS),
            vec![vec![refr.addr(), 0]]
        );
        // a health of -1.0 skips it
        e.register(EXTRA_GET_OBJECT_HEALTH, |_, _| Ret {
            st0: -1.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![refr, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), UPDATE_DAMAGE_STAGE).is_empty());
    }

    #[test]
    fn fn_005629a0_resets_the_state_and_the_open_state() {
        let mut e = buffer_handler_engine();
        e.register(EXTRA_GET_OBJECT_HEALTH, |_, _| Ret {
            st0: -1.0,
            ..Ret::default()
        });
        returns(&mut e, GET_OPEN_STATE, 3);
        let refr = handler_refr(&mut e, false);
        // flag 0x20 present: the state is reset with 0
        let buffer = buffer_with(&mut e, 0, 0x20 | 0x0040_0000, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RESET_STATE), vec![vec![refr.addr(), 0]]);
        // the open-state flag is copied, then the open state set
        assert_eq!(calls_to(&log, EXTRA_FLAG_CLEAR).len(), 1);
        assert_eq!(calls_to(&log, GET_OPEN_STATE), vec![vec![refr.addr(), 8]]);
        assert_eq!(
            calls_to(&log, SET_OPEN_STATE),
            vec![vec![refr.addr(), 3, 1]]
        );
        // only the 0x8000000 bit: reset with 1
        let buffer = buffer_with(&mut e, 0, 0x0800_0000, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RESET_STATE), vec![vec![refr.addr(), 1]]);
        assert!(calls_to(&log, EXTRA_FLAG_TEST).is_empty());
        // the buffer's own flag 4 calls the virtual +0x208 instead
        let buffer = buffer_with(&mut e, 0, 0x0800_0000, 4);
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, RESET_STATE).is_empty());
        assert_eq!(calls_to(&log, 0x00fd_0208), vec![vec![refr.addr(), 0]]);
        // an actor sets no open state
        let actor = handler_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![actor, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), SET_OPEN_STATE).is_empty());
    }

    #[test]
    fn fn_005629a0_restores_the_ragdoll_and_the_activating_reference() {
        let mut e = buffer_handler_engine();
        e.register(EXTRA_GET_OBJECT_HEALTH, |_, _| Ret {
            st0: -1.0,
            ..Ret::default()
        });
        returns(&mut e, SHOULD_3D_RAGDOLL, 1);
        // a 3D whose virtual +0x10 gives no target ends fn_0055d6d0 at once
        let obj3d = object_with_vtable(&mut e, 0x40, &[(0x10, 0x00fb_0010)]);
        e.register(0x00fb_0010, |_, _| 0u32.into_ret());
        e.register_double(0x00fd_01d0, move |_, _| obj3d.into_ret());
        e.mem.set_u32(REFR_VTABLE + 0x1d0, 0x00fd_01d0);
        let refr = batch_refr(&mut e, false);
        e.set(refr, TESObjectREFR::pParentCell, Ptr::new(0x6006));
        let buffer = buffer_with(&mut e, 0, 0x0400_0000, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SHOULD_3D_RAGDOLL),
            vec![vec![refr.addr(), obj3d]]
        );
        assert_eq!(
            calls_to(&log, RESTORE_RAGDOLL_DATA),
            vec![vec![refr.addr(), 0]]
        );
        assert_eq!(
            calls_to(&log, CELL_REFERENCE_STEP),
            vec![vec![0x6006, refr.addr()]]
        );
        // an actor keeps its rag doll data; a 3D that should not be one too
        let actor = batch_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![actor, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), RESTORE_RAGDOLL_DATA).is_empty());
        let refr = batch_refr(&mut e, false);
        returns(&mut e, SHOULD_3D_RAGDOLL, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![refr, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), RESTORE_RAGDOLL_DATA).is_empty());
    }

    #[test]
    fn fn_005629a0_runs_the_rest_state_for_a_non_actor_with_the_animation_flag() {
        let mut e = buffer_handler_engine();
        e.register(EXTRA_GET_OBJECT_HEALTH, |_, _| Ret {
            st0: -1.0,
            ..Ret::default()
        });
        let refr = handler_refr(&mut e, false);
        let buffer = buffer_with(&mut e, 0, 0x1000_0000, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![refr, buffer]);
        let log = e.call_log.take().unwrap();
        // fn_00563890 looks the 3D up twice
        assert_eq!(calls_to(&log, GET_LOADED_3D).len(), 2);
        // an actor does not
        let actor = handler_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0056_29a0, &args![actor, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), GET_LOADED_3D).is_empty());
    }

    #[test]
    fn the_extra_data_change_mask_adds_the_actor_bits() {
        assert_eq!(extra_change_mask(false), 0xa402_1c40);
        assert_eq!(extra_change_mask(true), 0xa406_1840);
    }

    // ---- the Havok record of the buffer-based save --------------------------

    /// The double of `00517630`: the body's motion kind is at +0x30.
    fn motion_kind_double(e: &mut Engine) {
        e.register(BODY_MOTION_KIND, |e, a| e.mem.u32(a[0] + 0x30).into_ret());
    }

    /// A collision node with a rigid body made by `rich_body` (position
    /// (1, 2, 3), rotation (5, 6, 7, 8), velocities (1, 2, 3) and (10, 11,
    /// 12)), motion kind `kind` and flag word `flag_word` (+0x10). Returns
    /// the node and the body.
    fn glue_node(e: &mut Engine, active: bool, kind: u32, flag_word: u32) -> (u32, u32) {
        let (body, _) = rich_body(e, active, false);
        e.mem.set_u32(body + 0x30, kind);
        e.mem.set_u32(body + 0x10, flag_word);
        (node_of(e, body), body)
    }

    fn save_state(
        e: &mut Engine,
        refr: Ptr<TESObjectREFR>,
        root: u32,
        flags: u8,
        buffer: u32,
    ) -> (Ptr<BGSHavokSaveData>, Ptr<CollisionWalkContext>) {
        let record: Ptr<BGSHavokSaveData> = e.new_object();
        e.set(record, BGSHavokSaveData::pRef, refr.cast());
        e.set(record, BGSHavokSaveData::pCollisionRoot, Ptr::new(root));
        e.set(record, BGSHavokSaveData::cFlags, flags);
        e.set(record, BGSHavokSaveData::pSaveLoadBuffer, Ptr::new(buffer));
        let walk: Ptr<CollisionWalkContext> = e.new_object();
        e.set(walk, CollisionWalkContext::pUserData, record.cast());
        (record, walk)
    }

    #[test]
    fn havok_record_constructor_and_the_flag_word_tests() {
        let mut e = batch_engine();
        let record: Ptr<BGSHavokSaveData> = e.new_object();
        e.mem.write(record.addr(), &[0xff; 0x20]);
        assert_eq!(e.call(0x0056_2e70, &args![record]).u32(), record.addr());
        // the flags byte and the seven words are cleared; the padding after
        // the byte is not touched
        let cleared = e.mem.bytes(record.addr(), 0x20);
        assert_eq!(cleared[0], 0);
        assert_eq!(cleared[1..4], [0xff; 3]);
        assert_eq!(cleared[4..], [0u8; 0x1c]);
        // the flag word is at +0x10
        let body = e.mem.alloc(0x40);
        e.mem.set_u32(body + 0x10, 0x203);
        assert_eq!(e.call(0x0056_3200, &args![body, 0x200u32]).u32(), 0x200);
        assert_eq!(e.call(0x0056_3200, &args![body, 0x400u32]).u32(), 0);
        assert!(e.call(0x0056_31e0, &args![body, 0x200u32]).bool());
        assert!(e.call(0x0056_31e0, &args![body, 0x3u32]).bool());
        assert!(!e.call(0x0056_31e0, &args![body, 0x400u32]).bool());
    }

    #[test]
    fn fn_00563120_counts_active_inactive_and_keyframed_bodies() {
        let mut e = havok_engine();
        motion_kind_double(&mut e);
        let (root, _) = glue_node(&mut e, true, 0, 0);
        let (keyed, _) = glue_node(&mut e, true, 4, 0);
        let (flagged, _) = glue_node(&mut e, false, 4, 0x200);
        let (idle, _) = glue_node(&mut e, false, 0, 0);
        let bare = havok_node(&mut e, 0, None, false);
        let refr = batch_refr(&mut e, false);
        let (record, walk) = save_state(&mut e, refr, root, 0, 0);
        for node in [root, keyed, flagged, idle, bare] {
            e.call(0x0056_3120, &args![node, walk]);
        }
        assert_eq!(e.get(record, BGSHavokSaveData::iActiveBoneCount), 2);
        assert_eq!(e.get(record, BGSHavokSaveData::iInactiveBoneCount), 2);
        // the body of kind 4 whose flag word has 0x200 is not keyframed
        assert_eq!(e.get(record, BGSHavokSaveData::iKeyFramedBoneCount), 1);
        // only the collision root sets flag 2
        assert_eq!(e.get(record, BGSHavokSaveData::cFlags), 2);
    }

    #[test]
    fn fn_00562d20_sets_the_record_flags_from_the_counted_bodies() {
        let mut e = havok_engine();
        motion_kind_double(&mut e);
        let (root, _) = glue_node(&mut e, true, 0, 0);
        let (keyed, _) = glue_node(&mut e, false, 4, 0);
        let (idle, _) = glue_node(&mut e, false, 0, 0);
        install_walk(&mut e, vec![root, keyed, idle]);
        returns(&mut e, COLLISION_ROOT, root);
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        let refr = big_refr(&mut e, false);
        let record: Ptr<BGSHavokSaveData> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0056_2d20, &args![refr, record]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(record, BGSHavokSaveData::pRef), refr.cast());
        assert_eq!(e.get(record, BGSHavokSaveData::pObj3D), Ptr::new(obj3d));
        assert_eq!(
            e.get(record, BGSHavokSaveData::pCollisionRoot),
            Ptr::new(root)
        );
        assert_eq!(e.get(record, BGSHavokSaveData::iActiveBoneCount), 1);
        assert_eq!(e.get(record, BGSHavokSaveData::iInactiveBoneCount), 2);
        assert_eq!(e.get(record, BGSHavokSaveData::iKeyFramedBoneCount), 1);
        // root seen (2), active and inactive (4)
        assert_eq!(e.get(record, BGSHavokSaveData::cFlags), 6);
        let walks = calls_to(&log, WALK_COLLISION);
        assert_eq!(walks.len(), 1);
        assert_eq!(walks[0][0], obj3d);
        assert_eq!(walks[0][2], 0x0056_3120);
        assert_eq!(e.mem.u8(walks[0][1] + 4), 1);
        assert_eq!(e.mem.u32(walks[0][1] + 8), 0x12);
        assert_eq!(e.mem.u32(walks[0][1] + 0xc), record.addr());
        // only active bodies: flag 1
        let (mut e, record, _) = {
            let mut e = havok_engine();
            motion_kind_double(&mut e);
            let (root, _) = glue_node(&mut e, true, 0, 0);
            let (second, _) = glue_node(&mut e, true, 0, 0);
            install_walk(&mut e, vec![root, second]);
            returns(&mut e, COLLISION_ROOT, root);
            returns(&mut e, GET_LOADED_3D, obj3d);
            let record: Ptr<BGSHavokSaveData> = e.new_object();
            (e, record, root)
        };
        let refr = big_refr(&mut e, false);
        e.call(0x0056_2d20, &args![refr, record]);
        assert_eq!(e.get(record, BGSHavokSaveData::cFlags), 3);
        // a keyframed body alone: flag 4 (and the root's 2)
        let (mut e, record) = {
            let mut e = havok_engine();
            motion_kind_double(&mut e);
            let (root, _) = glue_node(&mut e, false, 4, 0);
            install_walk(&mut e, vec![root]);
            returns(&mut e, COLLISION_ROOT, root);
            returns(&mut e, GET_LOADED_3D, obj3d);
            let record: Ptr<BGSHavokSaveData> = e.new_object();
            (e, record)
        };
        let refr = big_refr(&mut e, false);
        e.call(0x0056_2d20, &args![refr, record]);
        assert_eq!(e.get(record, BGSHavokSaveData::cFlags), 6);
        // no 3D: the record is left alone
        returns(&mut e, GET_LOADED_3D, 0);
        let untouched: Ptr<BGSHavokSaveData> = e.new_object();
        e.mem.set_u8(untouched.addr(), 0x77);
        e.call(0x0056_2d20, &args![refr, untouched]);
        assert_eq!(e.mem.u8(untouched.addr()), 0x77);
        assert_eq!(e.get(untouched, BGSHavokSaveData::pRef), Ptr::new(0));
    }

    #[test]
    fn fn_00563220_writes_pose_kind_and_velocities_of_a_body() {
        let mut e = havok_engine();
        motion_kind_double(&mut e);
        let writes = record_writes(&mut e, BUFFER_SAVE_DATA);
        let (node, _) = glue_node(&mut e, true, 4, 0);
        let (root, _) = glue_node(&mut e, true, 0, 0);
        let refr = batch_refr(&mut e, false);
        let (_, walk) = save_state(&mut e, refr, root, 4, 0x6600);
        e.call_log = Some(vec![]);
        e.call(0x0056_3220, &args![node, walk]);
        let log = e.call_log.take().unwrap();
        // position, rotation, kind (active, keyframed), the two velocities
        let mut rotation = floats(&[5.0, 7.0, 6.0, 8.0]);
        rotation.truncate(16);
        assert_eq!(
            writes.borrow().clone(),
            vec![
                floats(&[1.0, 2.0, 3.0]),
                rotation,
                vec![3],
                floats(&[1.0, 2.0, 3.0]),
                floats(&[10.0, 11.0, 12.0]),
            ]
        );
        assert!(calls_to(&log, BUFFER_SAVE_DATA)
            .iter()
            .all(|c| c[0] == 0x6600));
        // the collision root has no pose of its own
        writes.borrow_mut().clear();
        e.call(0x0056_3220, &args![root, walk]);
        assert_eq!(writes.borrow().len(), 3);
        assert_eq!(writes.borrow()[0], vec![1]);
        // a flag word with 0x200 is not keyframed; without flag 4 there is
        // no kind byte
        let (flagged, _) = glue_node(&mut e, true, 4, 0x200);
        writes.borrow_mut().clear();
        e.call(0x0056_3220, &args![flagged, walk]);
        assert_eq!(writes.borrow()[2], vec![1]);
        let (_, walk) = save_state(&mut e, refr, root, 0, 0x6600);
        writes.borrow_mut().clear();
        e.call(0x0056_3220, &args![flagged, walk]);
        assert_eq!(writes.borrow().len(), 4);
        // an inactive body has no velocities
        let (idle, _) = glue_node(&mut e, false, 0, 0);
        let (_, walk) = save_state(&mut e, refr, root, 4, 0x6600);
        writes.borrow_mut().clear();
        e.call(0x0056_3220, &args![idle, walk]);
        assert_eq!(writes.borrow().len(), 3);
        assert_eq!(writes.borrow()[2], vec![0]);
        // a node without a body writes nothing
        let bare = havok_node(&mut e, 0, None, false);
        writes.borrow_mut().clear();
        e.call(0x0056_3220, &args![bare, walk]);
        assert!(writes.borrow().is_empty());
    }

    #[test]
    fn save_havok_data_writes_the_flags_the_count_and_each_body() {
        let mut e = havok_engine();
        motion_kind_double(&mut e);
        let writes = record_writes(&mut e, BUFFER_SAVE_DATA);
        let (root, _) = glue_node(&mut e, true, 0, 0);
        let (second, _) = glue_node(&mut e, true, 0, 0);
        let (idle, _) = glue_node(&mut e, false, 0, 0);
        install_walk(&mut e, vec![root, second, idle]);
        returns(&mut e, COLLISION_ROOT, root);
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        stub(&mut e, &[BUFFER_SAVE_VARIABLE]);
        let refr = big_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0056_2de0, &args![refr, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        let written = writes.borrow().clone();
        // flags: root seen (2), active and inactive (4)
        assert_eq!(written[0], vec![6]);
        // then the count of the active and inactive bodies
        assert_eq!(calls_to(&log, BUFFER_SAVE_VARIABLE), vec![vec![0x6600, 3]]);
        // the root: kind and two velocities; the others also a pose
        assert_eq!(written[1], vec![1]);
        assert_eq!(written.len(), 1 + 3 + 5 + 3);
        assert_eq!(written[4], floats(&[1.0, 2.0, 3.0]));
        assert_eq!(written[6], vec![1]);
        assert_eq!(written[11], vec![0]);
        // two walks: counting, then writing
        let walks = calls_to(&log, WALK_COLLISION);
        assert_eq!(
            walks.iter().map(|w| w[2]).collect::<Vec<_>>(),
            vec![0x0056_3120, 0x0056_3220]
        );
        // without a loaded 3D nothing is written
        returns(&mut e, GET_LOADED_3D, 0);
        writes.borrow_mut().clear();
        e.call(0x0056_2de0, &args![refr, 0x6600u32]);
        assert!(writes.borrow().is_empty());
    }

    // ---- the Havok record of the buffer-based load -------------------------

    #[test]
    fn fn_00563380_reads_pose_kind_and_velocities_into_a_body() {
        let mut e = havok_engine();
        let (body, log) = rich_body(&mut e, true, false);
        let motion = motion_doubles(&mut e, body);
        stub(&mut e, &[RAGDOLL_NODE_STEP]);
        // the body's virtual +0xe4 records its argument
        let vtable = e.mem.u32(body);
        e.mem.set_u32(vtable + 0xe4, 0x00fb_00e4);
        e.register_double(0x00fb_00e4, |_, _| Ret::default());
        let node = node_of(&mut e, body);
        let refr = batch_refr(&mut e, false);
        let root = havok_node(&mut e, 0, None, false);
        let (record, walk) = save_state(&mut e, refr, root, 4, 0x6600);
        let mut stream = floats(&[1.0, 2.0, 3.0]);
        stream.extend(floats(&[0.0, 0.0, 0.0, 1.0]));
        stream.push(3);
        stream.extend(floats(&[4.0, 5.0, 6.0]));
        stream.extend(floats(&[7.0, 8.0, 9.0]));
        let position = load_stream(&mut e, stream.clone());
        e.call_log = Some(vec![]);
        e.call(0x0056_3380, &args![node, walk]);
        let calls = e.call_log.take().unwrap();
        assert_eq!(position.get(), stream.len());
        assert_eq!(calls_to(&calls, RAGDOLL_NODE_STEP), vec![vec![node, 0]]);
        assert_eq!(log.position.borrow()[0], floats(&[1.0, 2.0, 3.0]));
        let rotation = lanes(&log.rotation.borrow()[0]);
        assert!((rotation[3] - 1.0).abs() < 1e-3, "{rotation:?}");
        // bit 1 of the kind byte calls the body's virtual with 4
        assert_eq!(calls_to(&calls, 0x00fb_00e4), vec![vec![body, 4]]);
        // bit 0 makes it active: the velocities and the flag setter
        assert_eq!(
            motion.linear.borrow().clone(),
            vec![floats(&[4.0, 5.0, 6.0])]
        );
        assert_eq!(
            motion.angular.borrow().clone(),
            vec![floats(&[7.0, 8.0, 9.0])]
        );
        assert_eq!(calls_to(&calls, ENTITY_ACTIVATE).len(), 3);
        // flag 1 alone: active without a kind byte
        e.set(record, BGSHavokSaveData::cFlags, 1);
        let mut stream = floats(&[1.0, 2.0, 3.0]);
        stream.extend(floats(&[0.0, 0.0, 0.0, 1.0]));
        stream.extend(floats(&[4.0, 5.0, 6.0, 7.0, 8.0, 9.0]));
        let position = load_stream(&mut e, stream.clone());
        e.call(0x0056_3380, &args![node, walk]);
        assert_eq!(position.get(), stream.len());
        // flags 0: inactive, nothing but the pose
        e.set(record, BGSHavokSaveData::cFlags, 0);
        let mut stream = floats(&[1.0, 2.0, 3.0]);
        stream.extend(floats(&[0.0, 0.0, 0.0, 1.0]));
        let position = load_stream(&mut e, stream.clone());
        motion.linear.borrow_mut().clear();
        e.call(0x0056_3380, &args![node, walk]);
        assert_eq!(position.get(), stream.len());
        assert!(motion.linear.borrow().is_empty());
        // a node without a body: only the step
        let bare = havok_node(&mut e, 0, None, false);
        e.call_log = Some(vec![]);
        e.call(0x0056_3380, &args![bare, walk]);
        let calls = e.call_log.take().unwrap();
        assert_eq!(calls_to(&calls, RAGDOLL_NODE_STEP), vec![vec![bare, 0]]);
        assert_eq!(addresses(&calls).len(), 3);
    }

    #[test]
    fn fn_00563380_takes_the_collision_roots_pose_from_the_reference() {
        let mut e = havok_engine();
        let (body, log) = rich_body(&mut e, true, false);
        stub(&mut e, &[RAGDOLL_NODE_STEP]);
        let node = node_of(&mut e, body);
        let refr = batch_refr(&mut e, false);
        e.mem.write(refr.addr() + 0x30, &floats(&[9.0, 8.0, 7.0]));
        e.register(GET_ORIENTATION, |_, a| a[1].into_ret());
        e.register(QUATERNION_FROM_ROTATION, |e, a| {
            e.mem.write(a[0], &floats(&[0.0, 2.0, 0.0, 0.0]));
            a[0].into_ret()
        });
        let (record, walk) = save_state(&mut e, refr, node, 2, 0x6600);
        let position = load_stream(&mut e, vec![]);
        e.call(0x0056_3380, &args![node, walk]);
        // nothing is read, and the flag is cleared
        assert_eq!(position.get(), 0);
        assert_eq!(e.get(record, BGSHavokSaveData::cFlags), 0);
        assert_eq!(log.position.borrow()[0], floats(&[9.0, 8.0, 7.0]));
        let rotation = lanes(&log.rotation.borrow()[0]);
        assert!((rotation[1] - 1.0).abs() < 1e-3, "{rotation:?}");
    }

    #[test]
    fn load_havok_data_warns_without_a_3d_or_when_the_count_differs() {
        // no 3D
        let mut e = havok_engine();
        returns(&mut e, GET_LOADED_3D, 0);
        stub(&mut e, &[SAVE_GAME_WARNING]);
        let refr = big_refr(&mut e, false);
        e.mem.set_u32(refr.addr() + 0xc, 0x0006_0001);
        e.call_log = Some(vec![]);
        e.call(0x0056_2ed0, &args![refr, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SAVE_GAME_WARNING),
            vec![vec![HAVOK_NO_3D_FORMAT, 0x0e01, 0x0006_0001]]
        );
        assert!(calls_to(&log, BUFFER_LOAD_DATA).is_empty());
        // a different count: warned, and an actor is knocked down
        let mut e = havok_engine();
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        returns(&mut e, COLLISION_ROOT, 0x7007);
        install_walk(&mut e, vec![]);
        returns(&mut e, BUFFER_LOAD_VARIABLE, 7);
        stub(
            &mut e,
            &[
                SAVE_GAME_WARNING,
                BUFFER_LOAD_DATA,
                DISABLE_RAGDOLL_ANIM,
                KNOCK_DOWN,
            ],
        );
        let refr = big_refr(&mut e, true);
        e.mem.set_u32(refr.addr() + 0xc, 0x0006_0001);
        e.mem.set_u32(refr.addr() + 0xac, 0x5a5a);
        e.call_log = Some(vec![]);
        e.call(0x0056_2ed0, &args![refr, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SAVE_GAME_WARNING),
            vec![vec![HAVOK_BONE_COUNT_FORMAT, 0x0e01, 0x0006_0001, 7, 0]]
        );
        assert_eq!(calls_to(&log, DISABLE_RAGDOLL_ANIM), vec![vec![0x5a5a, 1]]);
        assert_eq!(
            calls_to(&log, KNOCK_DOWN),
            vec![vec![obj3d, ZERO_VECTOR, 1, 0.0f32.to_bits(), 0]]
        );
        assert!(calls_to(&log, SET_MOTION).is_empty());
        // a non-actor is only warned
        let refr = big_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0056_2ed0, &args![refr, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SAVE_GAME_WARNING).len(), 1);
        assert!(calls_to(&log, KNOCK_DOWN).is_empty());
    }

    /// An engine for a Havok load whose counts agree (none saved, none
    /// now); the flags byte read is 5. The walks are recorded as (callback,
    /// the flags byte of the record in the user data).
    fn load_havok_setup() -> (Engine, u32, WalkFlags) {
        let mut e = havok_engine();
        let obj3d = e.mem.alloc(0x20);
        returns(&mut e, GET_LOADED_3D, obj3d);
        returns(&mut e, COLLISION_ROOT, 0x7007);
        returns(&mut e, BUFFER_LOAD_VARIABLE, 0);
        e.register(BUFFER_LOAD_DATA, |e, a| {
            e.mem.set_u8(a[1], 5);
            Ret::default()
        });
        stub(
            &mut e,
            &[
                SET_MOTION,
                MAKE_VELOCITY,
                SET_3D_VELOCITY,
                SAVE_GAME_WARNING,
            ],
        );
        let walks = WalkFlags::default();
        let seen = walks.clone();
        e.register_double(WALK_COLLISION, move |e, a| {
            let record = e.mem.u32(a[1] + 0xc);
            seen.borrow_mut().push((a[2], e.mem.u8(record)));
            Ret::default()
        });
        slot(&mut e, 0x2e8, |_, _| false.into_ret());
        slot(&mut e, 0x230, |_, _| false.into_ret());
        returns(&mut e, REFR_STATE_TEST, 0);
        (e, obj3d, walks)
    }

    #[test]
    fn load_havok_data_sets_the_motion_and_walks_the_bodies() {
        let (mut e, obj3d, walks) = load_havok_setup();
        let refr = big_refr(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0056_2ed0, &args![refr, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        // the first walk counts, the second loads with the flags byte read
        assert_eq!(
            walks.borrow().clone(),
            vec![(0x0056_3120, 0), (0x0056_3380, 5)]
        );
        assert_eq!(calls_to(&log, SET_MOTION), vec![vec![obj3d, 1, 1, 0, 0]]);
        let made = calls_to(&log, MAKE_VELOCITY);
        assert_eq!(&made[0][1..], &[0.0f32.to_bits(), 0, 0]);
        // the zero velocity record is given before and after the walk
        assert_eq!(
            calls_to(&log, SET_3D_VELOCITY),
            vec![vec![obj3d, made[0][0]]; 2]
        );
        // an actor gets motion 4 unless one of its four tests holds
        let actor = big_refr(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0056_2ed0, &args![actor, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SET_MOTION), vec![vec![obj3d, 4, 1, 0, 0]]);
        assert_eq!(calls_to(&log, 0x00fd_022c), vec![vec![actor.addr(), 0]]);
        returns(&mut e, REFR_STATE_TEST, 1);
        e.call_log = Some(vec![]);
        e.call(0x0056_2ed0, &args![actor, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SET_MOTION)[0][1], 1);
        // the tests are made in order; the first that holds ends them
        assert!(calls_to(&log, 0x00fd_022c).is_empty());
        returns(&mut e, REFR_STATE_TEST, 0);
        slot(&mut e, 0x22c, |_, _| true.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0056_2ed0, &args![actor, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SET_MOTION)[0][1], 1);
        assert!(calls_to(&log, 0x00fd_02e8).is_empty());
        slot(&mut e, 0x22c, |_, _| false.into_ret());
        slot(&mut e, 0x2e8, |_, _| true.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0056_2ed0, &args![actor, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SET_MOTION)[0][1], 1);
        assert!(calls_to(&log, 0x00fd_0230).is_empty());
        slot(&mut e, 0x2e8, |_, _| false.into_ret());
        slot(&mut e, 0x230, |_, _| true.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0056_2ed0, &args![actor, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SET_MOTION)[0][1], 1);
    }

    // ---- 00563530, 00563650, 005636e0: the animation state ----------------

    /// A reference with loaded data whose 3D (+0x14) is `rest_setup`'s node.
    fn loaded_refr(e: &mut Engine, refr: Ptr<TESObjectREFR>, node: u32) {
        let loaded = e.mem.alloc(0x20);
        e.mem.set_u32(loaded + 0x14, node);
        e.set(refr, TESObjectREFR::pLoadedData, Ptr::new(loaded));
    }

    #[test]
    fn fn_00563530_is_true_for_a_finished_sequence_or_a_generic_sequence_of_another_name() {
        // a last finished sequence
        let (mut e, refr, _, _, _) = rest_setup(Some("Walk"), &[]);
        assert!(e.call(0x0056_3530, &args![refr]).bool());
        // no loaded data
        let (mut e, refr, _, node, _) = rest_setup(None, &[("Walk", 1)]);
        assert!(!e.call(0x0056_3530, &args![refr]).bool());
        // loaded data without a 3D
        loaded_refr(&mut e, refr, 0);
        assert!(!e.call(0x0056_3530, &args![refr]).bool());
        // a generic sequence named neither "IdleA" nor "IdleB"
        loaded_refr(&mut e, refr, node);
        assert!(e.call(0x0056_3530, &args![refr]).bool());
        // not generic, or one of the two names: false
        for sequences in [
            vec![("Walk", 0)],
            vec![("IdleA", 1)],
            vec![("IdleB", 1), ("Walk", 0)],
            vec![],
        ] {
            let (mut e, refr, _, node, _) = rest_setup(None, &sequences);
            loaded_refr(&mut e, refr, node);
            assert!(!e.call(0x0056_3530, &args![refr]).bool(), "{sequences:?}");
        }
        // the second of several decides
        let (mut e, refr, _, node, _) = rest_setup(None, &[("IdleA", 1), ("Other", 1)]);
        loaded_refr(&mut e, refr, node);
        assert!(e.call(0x0056_3530, &args![refr]).bool());
    }

    #[test]
    fn save_animation_writes_the_last_sequence_and_the_controller_manager() {
        let (mut e, refr, manager, _, _) = rest_setup(Some("Walk"), &[]);
        stub(&mut e, &[BUFFER_SAVE_STRING, SAVE_CONTROLLER_MANAGER]);
        e.call_log = Some(vec![]);
        e.call(0x0056_3650, &args![refr, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        let strings = calls_to(&log, BUFFER_SAVE_STRING);
        assert_eq!(strings.len(), 1);
        assert_eq!(strings[0][0], 0x6600);
        assert_eq!(e.mem.cstr(strings[0][1]), b"Walk".to_vec());
        assert_eq!(strings[0][2], 0);
        assert_eq!(
            calls_to(&log, SAVE_CONTROLLER_MANAGER),
            vec![vec![manager, 0x6600, 0xbf80_0000]]
        );
        // without a 3D the manager is null
        returns(&mut e, GET_LOADED_3D, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_3650, &args![refr, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SAVE_CONTROLLER_MANAGER),
            vec![vec![0, 0x6600, 0xbf80_0000]]
        );
    }

    /// `rest_setup` plus the doubles `fn_005636e0` needs; the name the
    /// buffer yields is `name`, and a base object that `open_close` says is
    /// an open/close form. Records (this, manager, name, time) of the
    /// sequence load and the name stored in the extra data.
    fn load_animation_setup(
        name: &'static str,
        open_close: bool,
    ) -> (Engine, Ptr<TESObjectREFR>, u32, SequenceLoads, Sink) {
        let (mut e, refr, manager, _, _) = rest_setup(None, &[]);
        e.mem.set_cstr(TEXT_OPEN_SIDE, b"Open");
        e.mem.set_cstr(TEXT_CLOSE_SIDE, b"Close");
        e.register_double(BUFFER_LOAD_STRING, move |e, a| {
            e.mem.set_cstr(a[1], name.as_bytes());
            Ret::default()
        });
        let stored = Sink::default();
        let seen = stored.clone();
        e.register_double(EXTRA_SET_LAST_SEQUENCE, move |e, a| {
            seen.borrow_mut().push(e.mem.cstr(a[1]));
            Ret::default()
        });
        returns(&mut e, IS_OPEN_CLOSE_FORM, u32::from(open_close));
        e.register(STRING_COPY, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        let loads = SequenceLoads::default();
        let seen = loads.clone();
        e.register_double(ANIMATION_LOAD_STEP, move |e, a| {
            seen.borrow_mut().push((a[0], a[1], e.mem.cstr(a[2]), a[3]));
            Ret::default()
        });
        stub(&mut e, &[CONTROLLER_MANAGER_LOADER]);
        (e, refr, manager, loads, stored)
    }

    #[test]
    fn fn_005636e0_swaps_the_two_sides_of_an_open_close_form() {
        for (name, open_close, expected, swapped) in [
            ("Open", true, "Close", true),
            ("Close", true, "Open", true),
            ("Walk", true, "Walk", false),
            ("Open", false, "Open", false),
        ] {
            let (mut e, refr, manager, loads, stored) = load_animation_setup(name, open_close);
            e.call_log = Some(vec![]);
            e.call(0x0056_36e0, &args![refr, 0x6600u32]);
            let log = e.call_log.take().unwrap();
            // the name read is stored first, as read
            assert_eq!(stored.borrow().clone(), vec![name.as_bytes().to_vec()]);
            let time = if swapped { -f32::MAX } else { f32::MAX };
            assert_eq!(
                loads.borrow().clone(),
                vec![(
                    refr.addr(),
                    manager,
                    expected.as_bytes().to_vec(),
                    time.to_bits()
                )],
                "{name} {open_close}"
            );
            assert_eq!(
                calls_to(&log, CONTROLLER_MANAGER_LOADER),
                vec![vec![manager, 0x6600, 0xbf80_0000]]
            );
        }
        // an empty name stores nothing and loads no sequence
        let (mut e, refr, manager, loads, stored) = load_animation_setup("", true);
        e.call_log = Some(vec![]);
        e.call(0x0056_36e0, &args![refr, 0x6600u32]);
        let log = e.call_log.take().unwrap();
        assert!(stored.borrow().is_empty());
        assert!(loads.borrow().is_empty());
        assert_eq!(
            calls_to(&log, CONTROLLER_MANAGER_LOADER),
            vec![vec![manager, 0x6600, 0xbf80_0000]]
        );
    }

    // ---- 00563890 -----------------------------------------------------------

    /// `rest_setup` plus the doubles `fn_00563890` needs on top.
    fn rest_state_setup(
        last_finished: Option<&str>,
        sequences: &[(&str, u32)],
    ) -> (Engine, Ptr<TESObjectREFR>, u32, u32, Vec<u32>) {
        let (mut e, refr, manager, node, made) = rest_setup(last_finished, sequences);
        stub(
            &mut e,
            &[
                GENERIC_SEQUENCE_STEP,
                CELL_ATTACH_REFERENCE_3D,
                SCENE_ADD_OBJECT,
            ],
        );
        returns(&mut e, TES_IS_CELL_LOADED, 1);
        returns(&mut e, SCENE_GETTER, 0x5c5c);
        let tes = e.mem.alloc(8);
        e.set_global(GLOBAL_TES, tes);
        e.set(refr, TESObjectREFR::pParentCell, Ptr::new(0x6006));
        (e, refr, manager, node, made)
    }

    #[test]
    fn fn_00563890_resets_the_reference_and_restarts_its_sequences() {
        let (mut e, refr, manager, node, made) =
            rest_state_setup(Some("Unequip"), &[("Walk", 1), ("IdleA", 0)]);
        e.call_log = Some(vec![]);
        e.call(0x0056_3890, &args![refr]);
        let log = e.call_log.take().unwrap();
        // the reset: location from the base object, the two virtuals, and
        // the loaded parent cell attaches the 3D again
        assert_eq!(
            calls_to(&log, 0x00fd_01cc),
            vec![vec![refr.addr(), 0x1234, 1]]
        );
        assert_eq!(calls_to(&log, 0x00fd_01c4).len(), 1);
        assert_eq!(
            calls_to(&log, CELL_ATTACH_REFERENCE_3D),
            vec![vec![0x6006, refr.addr(), 0]]
        );
        // only the generic sequence is stepped
        assert_eq!(
            calls_to(&log, GENERIC_SEQUENCE_STEP),
            vec![vec![refr.addr(), made[0]]]
        );
        // deactivated after the loop, after the first sequence, and at the end
        assert_eq!(
            calls_to(&log, MANAGER_DEACTIVATE_ALL),
            vec![vec![manager, 0]; 3]
        );
        assert_eq!(
            calls_to(&log, MANAGER_SET_FLAG),
            vec![vec![manager, 1], vec![manager, 0], vec![manager, 1]]
        );
        // the first sequence and the found idle sequence are activated
        let activate = |sequence: u32| {
            vec![
                manager,
                sequence,
                0,
                0,
                1.0f32.to_bits(),
                0.0f32.to_bits(),
                0,
            ]
        };
        assert_eq!(
            calls_to(&log, MANAGER_ACTIVATE),
            vec![activate(made[0]), activate(made[1])]
        );
        let minimum = (-f32::MAX).to_bits();
        assert_eq!(
            calls_to(&log, SEQUENCE_SET_OFFSET),
            vec![vec![made[0], minimum], vec![made[1], minimum]]
        );
        let records = calls_to(&log, MAKE_VELOCITY);
        assert_eq!(records.len(), 2);
        assert_eq!(&records[0][1..], &[0.5f32.to_bits(), 1, 0]);
        assert_eq!(&records[1][1..], &[1.5f32.to_bits(), 1, 0]);
        assert_eq!(
            calls_to(&log, SET_3D_VELOCITY),
            vec![vec![node, records[0][0]], vec![node, records[1][0]]]
        );
        // a reset reference's 3D goes back into the shadow scene
        assert_eq!(calls_to(&log, SCENE_GETTER), vec![vec![0]]);
        assert_eq!(calls_to(&log, SCENE_ADD_OBJECT), vec![vec![0x5c5c, node]]);
    }

    #[test]
    fn fn_00563890_without_a_reset_only_restarts_the_sequences() {
        let (mut e, refr, manager, _, made) = rest_state_setup(Some("Walk"), &[("IdleB", 1)]);
        e.call_log = Some(vec![]);
        e.call(0x0056_3890, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x00fd_01c4).is_empty());
        assert!(calls_to(&log, CELL_ATTACH_REFERENCE_3D).is_empty());
        assert!(calls_to(&log, SCENE_ADD_OBJECT).is_empty());
        // the generic IdleB is stepped and restarted but not activated again
        assert_eq!(
            calls_to(&log, GENERIC_SEQUENCE_STEP),
            vec![vec![refr.addr(), made[0]]]
        );
        // sequence 0 is the same one: activated once, offset twice
        assert_eq!(calls_to(&log, MANAGER_ACTIVATE).len(), 1);
        assert_eq!(calls_to(&log, SEQUENCE_SET_OFFSET).len(), 2);
        assert_eq!(
            calls_to(&log, MANAGER_SET_FLAG),
            vec![vec![manager, 1], vec![manager, 0], vec![manager, 1]]
        );
        // no sequences at all: the flag goes up and down, nothing restarts
        let (mut e, refr, manager, _, _) = rest_state_setup(None, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0056_3890, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MANAGER_SET_FLAG),
            vec![vec![manager, 1], vec![manager, 0]]
        );
        assert!(calls_to(&log, MANAGER_ACTIVATE).is_empty());
        assert_eq!(calls_to(&log, MANAGER_DEACTIVATE_ALL).len(), 3);
    }

    #[test]
    fn fn_00563890_resets_for_a_generic_unequip_or_a_morpher_and_needs_a_loaded_cell() {
        // a generic sequence named "Unequip"
        let (mut e, refr, _, _, _) = rest_state_setup(None, &[("Unequip", 1)]);
        e.call_log = Some(vec![]);
        e.call(0x0056_3890, &args![refr]);
        assert_eq!(calls_to(&e.call_log.take().unwrap(), 0x00fd_01c4).len(), 1);
        // a morpher controller, and a cell that is not loaded
        let (mut e, refr, _, node, _) = rest_state_setup(None, &[]);
        returns(&mut e, HAS_MORPHER_CONTROLLER, 1);
        returns(&mut e, TES_IS_CELL_LOADED, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_3890, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, HAS_MORPHER_CONTROLLER), vec![vec![node]]);
        assert_eq!(calls_to(&log, 0x00fd_01c4).len(), 1);
        assert!(calls_to(&log, CELL_ATTACH_REFERENCE_3D).is_empty());
        // a reference without a parent cell is not re-attached either
        e.set(refr, TESObjectREFR::pParentCell, Ptr::new(0));
        e.call_log = Some(vec![]);
        e.call(0x0056_3890, &args![refr]);
        assert!(calls_to(&e.call_log.take().unwrap(), TES_IS_CELL_LOADED).is_empty());
        // no 3D: only the final cast of the missing controller
        returns(&mut e, GET_LOADED_3D, 0);
        e.call_log = Some(vec![]);
        e.call(0x0056_3890, &args![refr]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MANAGER_DEACTIVATE_ALL).is_empty());
        assert!(calls_to(&log, SCENE_ADD_OBJECT).is_empty());
        assert_eq!(calls_to(&log, CHECKED_CAST), vec![vec![MANAGER_TYPE, 0]]);
    }
}

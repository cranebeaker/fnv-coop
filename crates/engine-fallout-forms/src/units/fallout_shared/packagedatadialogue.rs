//! `fallout shared/packagedatadialogue.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESDialoguePackageData` (Xbox PDB) is the dialogue variant of a package's
//! data block. Its vtable is at `010688ac`; the base class `TESPackageData`
//! only has a vtable (`01068884`). The PC build of this unit is unoptimized
//! code; exception-unwinding frames and security-cookie checks are not
//! translated (they hide no ordinary behaviour).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (cdecl, size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (cdecl, block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESPackageData` base-class constructor (stores the base vtable).
const PACKAGE_DATA_CONSTRUCT: u32 = 0x0067_ad20;
/// `TESPackageData` base-class destructor body (stores the base vtable).
const PACKAGE_DATA_DESTRUCT: u32 = 0x0067_ad70;
/// `PackageLocation::PackageLocation` (Xbox PDB), thiscall.
const PACKAGE_LOCATION_CONSTRUCT: u32 = 0x0067_f030;
/// `PackageLocation::~PackageLocation` (Xbox PDB), scalar deleting form
/// (one stack argument: the delete flag).
const PACKAGE_LOCATION_DESTRUCT: u32 = 0x0067_0b30;
/// `PackageLocation::Copy` (Xbox PDB): `this`, source, 0.
const PACKAGE_LOCATION_COPY: u32 = 0x0067_f4d0;
/// `PackageLocation::Compare` (Xbox PDB): `this`, other; nonzero = differ.
const PACKAGE_LOCATION_COMPARE: u32 = 0x0067_f600;
/// `PackageLocation::Save` (Xbox PDB): `this`, chunk type.
const PACKAGE_LOCATION_SAVE: u32 = 0x0067_f6c0;
/// `PackageLocation::SaveGame` (Xbox PDB): `this`, save buffer.
const PACKAGE_LOCATION_SAVE_GAME: u32 = 0x0067_fb60;
/// `PackageLocation::LoadGame` (Xbox PDB): `this`, load buffer.
const PACKAGE_LOCATION_LOAD_GAME: u32 = 0x0067_fbe0;
/// `PackageLocation::InitLoadGame` (Xbox PDB): `this`, argument.
const PACKAGE_LOCATION_INIT_LOAD_GAME: u32 = 0x0067_fc50;
/// `TESForm::GetFile` (Xbox PDB): `this`, index (-1 = the last source file).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB), cdecl: form ID pointer, file.
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The form lookup by ID used before `dynamic_cast` (cdecl, form ID).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `TESForm::__AddChunkData` (Xbox PDB), cdecl: chunk type, data, size.
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// Returns the form ID of the form it is called on (`ECX`).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// Returns a byte flag from a global (`011c54ba`); when it is set the
/// chunk is byte-swapped before and after it is written.
const CHUNK_SWAP_FLAG: u32 = 0x0040_1500;
/// Byte-swaps the 0x18-byte chunk in `ECX` in place.
const SWAP_CHUNK: u32 = 0x0067_b100;
/// `BGSSaveGameBuffer` raw write: `this`, data, size, 0.
const SAVE_RAW: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID` (Xbox PDB): `this`, form ID, 0.
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSLoadGameBuffer` raw read: `this`, destination, size.
const LOAD_RAW: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadFormID` (Xbox PDB): the form ID read.
const LOAD_FORM_ID: u32 = 0x0086_48a0;
/// The log function that takes a format and arguments (cdecl).
const LOG_FORMAT: u32 = 0x005b_5e40;
/// `memset` wrapper (cdecl): destination, value, size.
const MEMSET: u32 = 0x0040_3d30;

/// RTTI type descriptors the `dynamic_cast` of a package data goes between
/// (source `TESPackageData`, target `TESDialoguePackageData`).
const TYPE_PACKAGE_DATA: u32 = 0x0119_bdec;
const TYPE_DIALOGUE_PACKAGE_DATA: u32 = 0x0119_be10;
/// The type descriptors for the topic lookup (source `TESForm`, target the
/// topic class).
const TYPE_FORM: u32 = 0x0118_3028;
const TYPE_TOPIC: u32 = 0x0118_4720;

/// The vtable of `TESDialoguePackageData`.
const DIALOGUE_PACKAGE_DATA_VTABLE: u32 = 0x0106_88ac;
/// The format string "FORMS: Failed to find topic (%08X) in init for
/// dialogue package data on package '%s' (%08X)."
const TOPIC_NOT_FOUND_FORMAT: u32 = 0x0106_88d0;
/// Chunk type of the saved data (the bytes "PKDD") and of its location
/// (the bytes "PLD2").
const CHUNK_DIALOGUE_DATA: u32 = 0x4444_4b50;
const CHUNK_TARGET_LOCATION: u32 = 0x3244_4c50;
/// The size of the saved chunk.
const CHUNK_SIZE: u32 = 0x18;
/// Size of a `PackageLocation`.
const PACKAGE_LOCATION_SIZE: u32 = 0x0c;

layout! {
    /// `TESDialoguePackageData` (Xbox PDB), 0x20 bytes; the PC offsets are
    /// the same.
    pub struct TESDialoguePackageData: 0x20 {
        /// `fFov` (Xbox PDB).
        0x04 fFov: f32,
        /// `ptopic` (Xbox PDB): `TESTopic *`.
        0x08 ptopic: Ptr,
        /// `pTargetLocation` (Xbox PDB): `PackageLocation *`.
        0x0C pTargetLocation: Ptr,
        /// `bNOHeadtracking` (Xbox PDB).
        0x10 bNOHeadtracking: u8,
        /// `bDoNotControlTarget` (Xbox PDB).
        0x11 bDoNotControlTarget: u8,
        /// `bSpeakerMoveTalk` (Xbox PDB).
        0x12 bSpeakerMoveTalk: u8,
        /// `fDistanceStartTalking` (Xbox PDB).
        0x14 fDistanceStartTalking: f32,
        /// `bSayTo` (Xbox PDB).
        0x18 bSayTo: u8,
        /// `eTriggerType` (Xbox PDB): `PDIAL_TRIGGER_TYPE`.
        0x1C eTriggerType: u32,
    }
}

/// `dynamic_cast<TESDialoguePackageData *>` of a `TESPackageData *`.
fn cast_to_dialogue_data(e: &mut Engine, data: Ptr) -> Ptr<TESDialoguePackageData> {
    e.call(
        RT_DYNAMIC_CAST,
        &args![data, 0, TYPE_PACKAGE_DATA, TYPE_DIALOGUE_PACKAGE_DATA, 0],
    )
    .ptr()
}

/// A topic pointer from a form ID: the form lookup, then `dynamic_cast` from
/// `TESForm` to the topic class.
fn topic_from_form_id(e: &mut Engine, form_id: u32) -> Ptr {
    let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
    e.call(RT_DYNAMIC_CAST, &args![form, 0, TYPE_FORM, TYPE_TOPIC, 0])
        .ptr()
}

/// A new `PackageLocation` (allocation, then the constructor when the
/// allocation succeeded).
fn new_package_location(e: &mut Engine) -> Ptr {
    let block: Ptr = e.call(OPERATOR_NEW, &args![PACKAGE_LOCATION_SIZE]).ptr();
    if block.is_null() {
        Ptr::NULL
    } else {
        e.call(PACKAGE_LOCATION_CONSTRUCT, &args![block]).ptr()
    }
}

// Translated from 0067b1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDialoguePackageData::_scalar_deleting_destructor_` (Xbox PDB): runs
/// the destructor body and frees the object when bit 0 of `flags` is set.
pub fn tes_dialogue_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TESDialoguePackageData>,
    flags: u32,
) -> Ptr<TESDialoguePackageData> {
    fn_0067b280(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0067b210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESDialoguePackageData`: the base constructor, then
/// the class's vtable `010688ac` and the fields (its only caller, `00673a40`,
/// passes a 0x18-byte block by value). The argument words are: the field of
/// view, the topic, a word whose low byte is `bNOHeadtracking` and next byte
/// `bDoNotControlTarget`, `fDistanceStartTalking`, `bSayTo` (low byte) and
/// the trigger type. `bSpeakerMoveTalk` is not set.
#[allow(clippy::too_many_arguments)]
pub fn fn_0067b210(
    e: &mut Engine,
    this: Ptr<TESDialoguePackageData>,
    fov: f32,
    topic: Ptr,
    flag_bytes: u32,
    distance_start_talking: f32,
    say_to: u32,
    trigger_type: u32,
) -> Ptr<TESDialoguePackageData> {
    e.call(PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), DIALOGUE_PACKAGE_DATA_VTABLE);
    e.set(this, TESDialoguePackageData::fFov, fov);
    e.set(
        this,
        TESDialoguePackageData::bNOHeadtracking,
        flag_bytes as u8,
    );
    e.set(this, TESDialoguePackageData::ptopic, topic);
    e.set(
        this,
        TESDialoguePackageData::fDistanceStartTalking,
        distance_start_talking,
    );
    e.set(this, TESDialoguePackageData::pTargetLocation, Ptr::NULL);
    e.set(
        this,
        TESDialoguePackageData::bDoNotControlTarget,
        (flag_bytes >> 8) as u8,
    );
    e.set(this, TESDialoguePackageData::bSayTo, say_to as u8);
    e.set(this, TESDialoguePackageData::eTriggerType, trigger_type);
    this
}

// Translated from 0067b280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067b280 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// The destructor body (`fastcall`): stores this class's vtable, deletes
/// the target location (through its scalar deleting destructor) when there
/// is one, and runs the base destructor. The field is not cleared.
pub fn fn_0067b280(e: &mut Engine, this: Ptr<TESDialoguePackageData>) {
    e.mem.set_u32(this.addr(), DIALOGUE_PACKAGE_DATA_VTABLE);
    let location = e.get(this, TESDialoguePackageData::pTargetLocation);
    if !location.is_null() {
        e.call(PACKAGE_LOCATION_DESTRUCT, &args![location, 1]);
    }
    e.call(PACKAGE_DATA_DESTRUCT, &args![this]);
}

// Translated from 0067b310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067b310 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// `TESDialoguePackageData::Copy` (Xbox PDB): copies the fields of another
/// package data (cast to this class; nothing happens if it is not one).
/// The target location is deleted, then copied into a new one when the
/// source has one. `bSpeakerMoveTalk` is not copied.
pub fn tes_dialogue_package_data_copy(
    e: &mut Engine,
    this: Ptr<TESDialoguePackageData>,
    source: Ptr,
) {
    let other = cast_to_dialogue_data(e, source);
    if other.is_null() {
        return;
    }
    let fov = e.get(other, TESDialoguePackageData::fFov);
    e.set(this, TESDialoguePackageData::fFov, fov);
    let distance = e.get(other, TESDialoguePackageData::fDistanceStartTalking);
    e.set(
        this,
        TESDialoguePackageData::fDistanceStartTalking,
        distance,
    );
    let no_headtracking = e.get(other, TESDialoguePackageData::bNOHeadtracking);
    e.set(
        this,
        TESDialoguePackageData::bNOHeadtracking,
        no_headtracking,
    );

    let old_location = e.get(this, TESDialoguePackageData::pTargetLocation);
    if !old_location.is_null() {
        e.call(PACKAGE_LOCATION_DESTRUCT, &args![old_location, 1]);
        e.set(this, TESDialoguePackageData::pTargetLocation, Ptr::NULL);
    }
    let source_location = e.get(other, TESDialoguePackageData::pTargetLocation);
    if !source_location.is_null() {
        let location = new_package_location(e);
        e.set(this, TESDialoguePackageData::pTargetLocation, location);
        e.call(PACKAGE_LOCATION_COPY, &args![location, source_location, 0]);
    }
    let topic = e.get(other, TESDialoguePackageData::ptopic);
    e.set(this, TESDialoguePackageData::ptopic, topic);
    let do_not_control = e.get(other, TESDialoguePackageData::bDoNotControlTarget);
    e.set(
        this,
        TESDialoguePackageData::bDoNotControlTarget,
        do_not_control,
    );
    let say_to = e.get(other, TESDialoguePackageData::bSayTo);
    e.set(this, TESDialoguePackageData::bSayTo, say_to);
    let trigger = e.get(other, TESDialoguePackageData::eTriggerType);
    e.set(this, TESDialoguePackageData::eTriggerType, trigger);
}

// Translated from 0067b470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDialoguePackageData::Compare` (Xbox PDB): true when the other
/// package data differs from this one (or is not a dialogue package data).
/// Compares the field of view, the start distance, `bNOHeadtracking`, the
/// topic, the target locations (both absent, or equal by
/// `PackageLocation::Compare`), `bDoNotControlTarget`, `bSayTo` and the
/// trigger type. A NaN never compares equal.
pub fn tes_dialogue_package_data_compare(
    e: &mut Engine,
    this: Ptr<TESDialoguePackageData>,
    other: Ptr,
) -> bool {
    let other = cast_to_dialogue_data(e, other);
    if other.is_null() {
        return true;
    }
    if e.get(this, TESDialoguePackageData::fFov) != e.get(other, TESDialoguePackageData::fFov) {
        return true;
    }
    if e.get(this, TESDialoguePackageData::fDistanceStartTalking)
        != e.get(other, TESDialoguePackageData::fDistanceStartTalking)
    {
        return true;
    }
    if e.get(this, TESDialoguePackageData::bNOHeadtracking)
        != e.get(other, TESDialoguePackageData::bNOHeadtracking)
    {
        return true;
    }
    if e.get(this, TESDialoguePackageData::ptopic).addr()
        != e.get(other, TESDialoguePackageData::ptopic).addr()
    {
        return true;
    }
    let location = e.get(this, TESDialoguePackageData::pTargetLocation);
    let other_location = e.get(other, TESDialoguePackageData::pTargetLocation);
    if !location.is_null() {
        if e.call(PACKAGE_LOCATION_COMPARE, &args![location, other_location])
            .bool()
        {
            return true;
        }
    } else if !other_location.is_null() {
        return true;
    }
    if e.get(this, TESDialoguePackageData::bDoNotControlTarget)
        != e.get(other, TESDialoguePackageData::bDoNotControlTarget)
    {
        return true;
    }
    if e.get(this, TESDialoguePackageData::bSayTo) != e.get(other, TESDialoguePackageData::bSayTo) {
        return true;
    }
    e.get(this, TESDialoguePackageData::eTriggerType)
        != e.get(other, TESDialoguePackageData::eTriggerType)
}

// Translated from 0067b590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDialoguePackageData::InitItem` (Xbox PDB): resolves the topic form ID
/// (held in the topic field after loading) for the plugin file of the
/// owning package `owner`: `AddCompileIndex` on the ID, the form lookup and a
/// `dynamic_cast` to the topic class. When no topic is found for a nonzero
/// ID, logs the ID, the owner's name (virtual slot `0x130`) and the owner's
/// form ID.
pub fn tes_dialogue_package_data_init_item(
    e: &mut Engine,
    this: Ptr<TESDialoguePackageData>,
    owner: Ptr,
) {
    let form_id = e.get(this, TESDialoguePackageData::ptopic).addr();
    e.with_stack(4, |e, id_slot| {
        e.mem.set_u32(id_slot.addr(), form_id);
        let file = e.call(TES_FORM_GET_FILE, &args![owner, -1i32]).u32();
        e.call(TES_FORM_ADD_COMPILE_INDEX, &args![id_slot, file]);
        let resolved_id = e.mem.u32(id_slot.addr());
        let topic = topic_from_form_id(e, resolved_id);
        e.set(this, TESDialoguePackageData::ptopic, topic);
        if topic.is_null() && resolved_id != 0 {
            let owner_id = e.call(GET_FORM_ID, &args![owner]).u32();
            let name = e.vcall(owner.addr(), 0x130, &args![]).u32();
            e.call(
                LOG_FORMAT,
                &args![TOPIC_NOT_FOUND_FORMAT, resolved_id, name, owner_id],
            );
        }
    });
}

// Translated from 0067b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDialoguePackageData::Save` (Xbox PDB): writes the 0x18-byte "PKDD"
/// chunk (field of view, topic form ID, `bNOHeadtracking`,
/// `bDoNotControlTarget`, start distance, `bSayTo`, trigger type), byte
/// swapping it before and after the write when the global flag says so, then
/// the target location as a "PLD2" chunk when there is one.
pub fn tes_dialogue_package_data_save(e: &mut Engine, this: Ptr<TESDialoguePackageData>) {
    e.with_stack(CHUNK_SIZE, |e, chunk| {
        e.call(MEMSET, &args![chunk, 0, CHUNK_SIZE]);
        let base = chunk.addr();
        let no_headtracking = e.get(this, TESDialoguePackageData::bNOHeadtracking);
        e.mem.set_u8(base + 8, no_headtracking);
        let fov = e.get(this, TESDialoguePackageData::fFov);
        e.mem.set_f32(base, fov);
        let do_not_control = e.get(this, TESDialoguePackageData::bDoNotControlTarget);
        e.mem.set_u8(base + 9, do_not_control);
        let say_to = e.get(this, TESDialoguePackageData::bSayTo);
        e.mem.set_u8(base + 0x10, say_to);
        let trigger = e.get(this, TESDialoguePackageData::eTriggerType);
        e.mem.set_u32(base + 0x14, trigger);
        let distance = e.get(this, TESDialoguePackageData::fDistanceStartTalking);
        e.mem.set_f32(base + 0x0c, distance);
        let topic = e.get(this, TESDialoguePackageData::ptopic);
        if !topic.is_null() {
            let form_id = e.call(GET_FORM_ID, &args![topic]).u32();
            e.mem.set_u32(base + 4, form_id);
        }
        if e.call(CHUNK_SWAP_FLAG, &args![]).bool() {
            e.call(SWAP_CHUNK, &args![chunk]);
        }
        e.call(
            ADD_CHUNK_DATA,
            &args![CHUNK_DIALOGUE_DATA, chunk, CHUNK_SIZE],
        );
        if e.call(CHUNK_SWAP_FLAG, &args![]).bool() {
            e.call(SWAP_CHUNK, &args![chunk]);
        }
    });
    let location = e.get(this, TESDialoguePackageData::pTargetLocation);
    if !location.is_null() {
        e.call(
            PACKAGE_LOCATION_SAVE,
            &args![location, CHUNK_TARGET_LOCATION],
        );
    }
}

/// The (offset, size) pairs `SaveGame` and `LoadGame` transfer raw, in order.
const GAME_FIELDS: [(u32, u32); 7] = [
    (0x04, 4),
    (0x10, 1),
    (0x11, 1),
    (0x12, 1),
    (0x14, 4),
    (0x18, 1),
    (0x1c, 4),
];

// Translated from 0067b6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDialoguePackageData::SaveGame` (Xbox PDB): writes to the save buffer
/// the field of view, the three flag bytes (`+0x10`, `+0x11`, `+0x12`), the
/// start distance, `bSayTo`, the trigger type, the topic's form ID, a byte
/// saying whether there is a target location, and that location's save game
/// data.
pub fn tes_dialogue_package_data_save_game(
    e: &mut Engine,
    this: Ptr<TESDialoguePackageData>,
    buffer: Ptr,
) {
    let base = this.addr();
    for (offset, size) in GAME_FIELDS {
        e.call(SAVE_RAW, &args![buffer, base + offset, size, 0]);
    }
    let topic = e.get(this, TESDialoguePackageData::ptopic);
    e.call(SAVE_FORM_ID, &args![buffer, topic, 0]);
    let location = e.get(this, TESDialoguePackageData::pTargetLocation);
    let has_location = !location.is_null();
    e.with_stack(1, |e, flag| {
        e.mem.set_u8(flag.addr(), has_location as u8);
        e.call(SAVE_RAW, &args![buffer, flag, 1, 0]);
    });
    if has_location {
        e.call(PACKAGE_LOCATION_SAVE_GAME, &args![location, buffer]);
    }
}

// Translated from 0067b7d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067b7d0 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// `TESDialoguePackageData::LoadGame` (Xbox PDB): reads what `SaveGame`
/// wrote. The topic is the form lookup of the loaded ID cast to the topic
/// class; a new `PackageLocation` is built and loaded when the flag byte is
/// set (an old one is not freed).
pub fn tes_dialogue_package_data_load_game(
    e: &mut Engine,
    this: Ptr<TESDialoguePackageData>,
    buffer: Ptr,
) {
    let base = this.addr();
    for (offset, size) in GAME_FIELDS {
        e.call(LOAD_RAW, &args![buffer, base + offset, size]);
    }
    let form_id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
    let topic = topic_from_form_id(e, form_id);
    e.set(this, TESDialoguePackageData::ptopic, topic);
    let has_location = e.with_stack(1, |e, flag| {
        e.mem.set_u8(flag.addr(), 0);
        e.call(LOAD_RAW, &args![buffer, flag, 1]);
        e.mem.u8(flag.addr())
    });
    if has_location != 0 {
        let location = new_package_location(e);
        e.set(this, TESDialoguePackageData::pTargetLocation, location);
        e.call(PACKAGE_LOCATION_LOAD_GAME, &args![location, buffer]);
    }
}

// Translated from 0067b920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDialoguePackageData::InitLoadGame` (Xbox PDB): forwards its argument
/// to the target location's `InitLoadGame` when there is one.
pub fn tes_dialogue_package_data_init_load_game(
    e: &mut Engine,
    this: Ptr<TESDialoguePackageData>,
    argument: u32,
) {
    let location = e.get(this, TESDialoguePackageData::pTargetLocation);
    if !location.is_null() {
        e.call(PACKAGE_LOCATION_INIT_LOAD_GAME, &args![location, argument]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0067b1e0,
            tes_dialogue_package_data_scalar_deleting_destructor(
                Ptr<TESDialoguePackageData>,
                u32,
            )
                -> Ptr<TESDialoguePackageData>
        ),
        entry!(
            0x0067b210,
            fn_0067b210(
                Ptr<TESDialoguePackageData>,
                f32,
                Ptr,
                u32,
                f32,
                u32,
                u32,
            ) -> Ptr<TESDialoguePackageData>
        ),
        entry!(0x0067b280, fn_0067b280(Ptr<TESDialoguePackageData>)),
        entry!(
            0x0067b310,
            tes_dialogue_package_data_copy(Ptr<TESDialoguePackageData>, Ptr)
        ),
        entry!(
            0x0067b470,
            tes_dialogue_package_data_compare(Ptr<TESDialoguePackageData>, Ptr) -> bool
        ),
        entry!(
            0x0067b590,
            tes_dialogue_package_data_init_item(Ptr<TESDialoguePackageData>, Ptr)
        ),
        entry!(
            0x0067b630,
            tes_dialogue_package_data_save(Ptr<TESDialoguePackageData>)
        ),
        entry!(
            0x0067b6f0,
            tes_dialogue_package_data_save_game(Ptr<TESDialoguePackageData>, Ptr)
        ),
        entry!(
            0x0067b7d0,
            tes_dialogue_package_data_load_game(Ptr<TESDialoguePackageData>, Ptr)
        ),
        entry!(
            0x0067b920,
            tes_dialogue_package_data_init_load_game(Ptr<TESDialoguePackageData>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Stand-ins for the casts: the object passes straight through the
    /// `dynamic_cast` and the form lookup; allocation and the base class are
    /// plain.
    fn engine_with_casts() -> Engine {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(PACKAGE_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(PACKAGE_DATA_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.map(0x0106_8000, 0x1000);
        e
    }

    fn dialogue_data(e: &mut Engine) -> Ptr<TESDialoguePackageData> {
        e.new_object()
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_with_the_flag() {
        let mut e = engine_with_casts();
        let data = dialogue_data(&mut e);
        let location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(data, TESDialoguePackageData::pTargetLocation, location);
        e.register(PACKAGE_LOCATION_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let back = e.call(0x0067_b1e0, &args![data, 0]).ptr::<()>();
        assert_eq!(back.addr(), data.addr());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != OPERATOR_DELETE));
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == PACKAGE_LOCATION_DESTRUCT && a[..] == [location.addr(), 1]));
        assert_eq!(e.mem.u32(data.addr()), DIALOGUE_PACKAGE_DATA_VTABLE);

        e.call_log = Some(vec![]);
        e.call(0x0067_b1e0, &args![data, 1]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == OPERATOR_DELETE && a[..] == [data.addr()]));
    }

    #[test]
    fn constructor_stores_the_vtable_and_the_arguments() {
        let mut e = engine_with_casts();
        let data = dialogue_data(&mut e);
        e.mem.set_u8(data.addr() + 0x12, 0x5a);
        let topic = Ptr::<()>::new(0x1234);
        e.call(
            0x0067_b210,
            &args![data, 1.5f32, topic, 0x0201u32, 90.0f32, 1u32, 7u32],
        );
        assert_eq!(e.mem.u32(data.addr()), DIALOGUE_PACKAGE_DATA_VTABLE);
        assert_eq!(e.get(data, TESDialoguePackageData::fFov), 1.5);
        assert_eq!(e.get(data, TESDialoguePackageData::ptopic).addr(), 0x1234);
        assert_eq!(e.get(data, TESDialoguePackageData::bNOHeadtracking), 1);
        assert_eq!(e.get(data, TESDialoguePackageData::bDoNotControlTarget), 2);
        assert_eq!(
            e.get(data, TESDialoguePackageData::fDistanceStartTalking),
            90.0
        );
        assert_eq!(e.get(data, TESDialoguePackageData::bSayTo), 1);
        assert_eq!(e.get(data, TESDialoguePackageData::eTriggerType), 7);
        assert!(e
            .get(data, TESDialoguePackageData::pTargetLocation)
            .is_null());
        // bSpeakerMoveTalk is not touched.
        assert_eq!(e.get(data, TESDialoguePackageData::bSpeakerMoveTalk), 0x5a);
    }

    #[test]
    fn destructor_body_without_a_location_only_runs_the_base() {
        let mut e = engine_with_casts();
        let data = dialogue_data(&mut e);
        e.register(PACKAGE_LOCATION_DESTRUCT, |_, _| panic!("no location"));
        e.call(0x0067_b280, &args![data]);
        assert_eq!(e.mem.u32(data.addr()), DIALOGUE_PACKAGE_DATA_VTABLE);
    }

    #[test]
    fn copy_replaces_the_location_and_copies_fields() {
        let mut e = engine_with_casts();
        let this = dialogue_data(&mut e);
        let source = dialogue_data(&mut e);
        e.set(source, TESDialoguePackageData::fFov, 2.0);
        e.set(source, TESDialoguePackageData::ptopic, Ptr::new(0x77));
        e.set(source, TESDialoguePackageData::bNOHeadtracking, 1);
        e.set(source, TESDialoguePackageData::bDoNotControlTarget, 1);
        e.set(source, TESDialoguePackageData::bSayTo, 1);
        e.set(source, TESDialoguePackageData::eTriggerType, 3);
        e.set(source, TESDialoguePackageData::fDistanceStartTalking, 4.0);
        let source_location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(
            source,
            TESDialoguePackageData::pTargetLocation,
            source_location,
        );
        let old_location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(this, TESDialoguePackageData::pTargetLocation, old_location);
        let new_block = e.mem.alloc(0x0c);
        e.register_double(OPERATOR_NEW, move |_, _| Ret {
            eax: new_block,
            ..Ret::default()
        });
        e.register(PACKAGE_LOCATION_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(PACKAGE_LOCATION_DESTRUCT, |_, _| Ret::default());
        e.register(PACKAGE_LOCATION_COPY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_b310, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(
            |(addr, a)| *addr == PACKAGE_LOCATION_DESTRUCT && a[..] == [old_location.addr(), 1]
        ));
        assert!(log.iter().any(|(addr, a)| *addr == PACKAGE_LOCATION_COPY
            && a[..] == [new_block, source_location.addr(), 0]));
        assert_eq!(
            e.get(this, TESDialoguePackageData::pTargetLocation).addr(),
            new_block
        );
        assert_eq!(e.get(this, TESDialoguePackageData::fFov), 2.0);
        assert_eq!(e.get(this, TESDialoguePackageData::ptopic).addr(), 0x77);
        assert_eq!(e.get(this, TESDialoguePackageData::bNOHeadtracking), 1);
        assert_eq!(e.get(this, TESDialoguePackageData::bDoNotControlTarget), 1);
        assert_eq!(e.get(this, TESDialoguePackageData::bSayTo), 1);
        assert_eq!(e.get(this, TESDialoguePackageData::eTriggerType), 3);
        assert_eq!(
            e.get(this, TESDialoguePackageData::fDistanceStartTalking),
            4.0
        );
    }

    #[test]
    fn copy_of_something_that_is_not_dialogue_data_does_nothing() {
        let mut e = engine_with_casts();
        let this = dialogue_data(&mut e);
        e.set(this, TESDialoguePackageData::fFov, 9.0);
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        e.call(0x0067_b310, &args![this, 0x1000u32]);
        assert_eq!(e.get(this, TESDialoguePackageData::fFov), 9.0);
    }

    #[test]
    fn compare_finds_each_difference() {
        let mut e = engine_with_casts();
        let a = dialogue_data(&mut e);
        let b = dialogue_data(&mut e);
        let compare = |e: &mut Engine| e.call(0x0067_b470, &args![a, b]).bool();
        assert!(!compare(&mut e));
        e.set(b, TESDialoguePackageData::fFov, 1.0);
        assert!(compare(&mut e));
        e.set(b, TESDialoguePackageData::fFov, 0.0);
        e.set(b, TESDialoguePackageData::fDistanceStartTalking, 1.0);
        assert!(compare(&mut e));
        e.set(b, TESDialoguePackageData::fDistanceStartTalking, 0.0);
        e.set(b, TESDialoguePackageData::bNOHeadtracking, 1);
        assert!(compare(&mut e));
        e.set(b, TESDialoguePackageData::bNOHeadtracking, 0);
        e.set(b, TESDialoguePackageData::ptopic, Ptr::new(4));
        assert!(compare(&mut e));
        e.set(b, TESDialoguePackageData::ptopic, Ptr::NULL);
        e.set(b, TESDialoguePackageData::bDoNotControlTarget, 1);
        assert!(compare(&mut e));
        e.set(b, TESDialoguePackageData::bDoNotControlTarget, 0);
        e.set(b, TESDialoguePackageData::bSayTo, 1);
        assert!(compare(&mut e));
        e.set(b, TESDialoguePackageData::bSayTo, 0);
        e.set(b, TESDialoguePackageData::eTriggerType, 2);
        assert!(compare(&mut e));
        e.set(b, TESDialoguePackageData::eTriggerType, 0);
        assert!(!compare(&mut e));
        // Not a dialogue package data.
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        assert!(compare(&mut e));
    }

    #[test]
    fn compare_locations_by_presence_and_by_their_compare() {
        let mut e = engine_with_casts();
        let a = dialogue_data(&mut e);
        let b = dialogue_data(&mut e);
        let location_a = Ptr::<()>::new(e.mem.alloc(0x0c));
        let location_b = Ptr::<()>::new(e.mem.alloc(0x0c));
        // Only the other has a location.
        e.set(b, TESDialoguePackageData::pTargetLocation, location_b);
        assert!(e.call(0x0067_b470, &args![a, b]).bool());
        // Both have one: PackageLocation::Compare decides.
        e.set(a, TESDialoguePackageData::pTargetLocation, location_a);
        e.register(PACKAGE_LOCATION_COMPARE, |_, _| Ret::default());
        assert!(!e.call(0x0067_b470, &args![a, b]).bool());
        e.register(PACKAGE_LOCATION_COMPARE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert!(e.call(0x0067_b470, &args![a, b]).bool());
    }

    #[test]
    fn init_item_resolves_the_topic() {
        let mut e = engine_with_casts();
        let data = dialogue_data(&mut e);
        let owner = Ptr::<()>::new(e.mem.alloc(8));
        e.set(data, TESDialoguePackageData::ptopic, Ptr::new(0x0100_0abc));
        e.register(TES_FORM_GET_FILE, |_, _| Ret {
            eax: 0x4000,
            ..Ret::default()
        });
        // AddCompileIndex rewrites the ID in place.
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            e.mem.set_u32(a[0], 0x0500_0abc);
            Ret::default()
        });
        e.register(LOG_FORMAT, |_, _| panic!("topic found"));
        e.call_log = Some(vec![]);
        e.call(0x0067_b590, &args![data, owner]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == TES_FORM_GET_FILE && a[..] == [owner.addr(), u32::MAX]));
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == LOOKUP_FORM && a[..] == [0x0500_0abc]));
        assert_eq!(
            e.get(data, TESDialoguePackageData::ptopic).addr(),
            0x0500_0abc
        );
    }

    #[test]
    fn init_item_logs_a_missing_topic_only_for_a_nonzero_id() {
        let mut e = engine_with_casts();
        let data = dialogue_data(&mut e);
        let owner = Ptr::<()>::new(e.mem.alloc(8));
        e.put_vtable(0x0300_0000, &[0; 0x4d]);
        e.mem.set_u32(0x0300_0000 + 0x130, 0x0abc_0001);
        e.mem.set_u32(owner.addr(), 0x0300_0000);
        e.register(0x0abc_0001, |_, _| Ret {
            eax: 0x6666,
            ..Ret::default()
        });
        e.register(TES_FORM_GET_FILE, |_, _| Ret::default());
        e.register(TES_FORM_ADD_COMPILE_INDEX, |_, _| Ret::default());
        e.register(GET_FORM_ID, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        e.register(LOG_FORMAT, |_, _| Ret::default());
        e.set(data, TESDialoguePackageData::ptopic, Ptr::new(0x42));
        e.call_log = Some(vec![]);
        e.call(0x0067_b590, &args![data, owner]);
        let log = e.call_log.take().unwrap();
        let line = log.iter().find(|(addr, _)| *addr == LOG_FORMAT).unwrap();
        assert_eq!(
            line.1[..],
            [TOPIC_NOT_FOUND_FORMAT, 0x42, 0x6666, owner.addr() + 1]
        );
        // A zero ID is not reported.
        e.set(data, TESDialoguePackageData::ptopic, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0067_b590, &args![data, owner]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != LOG_FORMAT));
        assert!(e.get(data, TESDialoguePackageData::ptopic).is_null());
    }

    #[test]
    fn save_writes_the_chunk_and_swaps_around_it_when_flagged() {
        let mut e = engine_with_casts();
        let data = dialogue_data(&mut e);
        e.set(data, TESDialoguePackageData::fFov, 1.5);
        e.set(data, TESDialoguePackageData::bNOHeadtracking, 1);
        e.set(data, TESDialoguePackageData::bDoNotControlTarget, 1);
        e.set(data, TESDialoguePackageData::fDistanceStartTalking, 8.0);
        e.set(data, TESDialoguePackageData::bSayTo, 1);
        e.set(data, TESDialoguePackageData::eTriggerType, 5);
        e.set(data, TESDialoguePackageData::ptopic, Ptr::new(0x9000));
        let location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(data, TESDialoguePackageData::pTargetLocation, location);
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(GET_FORM_ID, |_, _| Ret {
            eax: 0x0100_0001,
            ..Ret::default()
        });
        e.register(CHUNK_SWAP_FLAG, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(SWAP_CHUNK, |_, _| Ret::default());
        let written = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = written.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            *sink.borrow_mut() = e.mem.bytes(a[1], a[2]);
            assert_eq!(a[0], CHUNK_DIALOGUE_DATA);
            Ret::default()
        });
        e.register(PACKAGE_LOCATION_SAVE, |_, a| {
            assert_eq!(a[1], CHUNK_TARGET_LOCATION);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0067_b630, &args![data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().filter(|(addr, _)| *addr == SWAP_CHUNK).count(),
            2
        );
        assert!(log.iter().any(|(addr, _)| *addr == PACKAGE_LOCATION_SAVE));
        let mut expected = vec![0u8; 0x18];
        expected[0..4].copy_from_slice(&1.5f32.to_le_bytes());
        expected[4..8].copy_from_slice(&0x0100_0001u32.to_le_bytes());
        expected[8] = 1;
        expected[9] = 1;
        expected[0x0c..0x10].copy_from_slice(&8.0f32.to_le_bytes());
        expected[0x10] = 1;
        expected[0x14..0x18].copy_from_slice(&5u32.to_le_bytes());
        assert_eq!(*written.borrow(), expected);
    }

    #[test]
    fn save_without_topic_location_or_swap_writes_a_zero_id_only() {
        let mut e = engine_with_casts();
        let data = dialogue_data(&mut e);
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(CHUNK_SWAP_FLAG, |_, _| Ret::default());
        e.register(ADD_CHUNK_DATA, |e, a| {
            assert_eq!(e.mem.u32(a[1] + 4), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0067_b630, &args![data]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != SWAP_CHUNK
            && *addr != PACKAGE_LOCATION_SAVE
            && *addr != GET_FORM_ID));
    }

    #[test]
    fn save_game_writes_fields_topic_and_location() {
        let mut e = engine_with_casts();
        let data = dialogue_data(&mut e);
        let buffer = Ptr::<()>::new(e.mem.alloc(8));
        let location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(data, TESDialoguePackageData::pTargetLocation, location);
        e.set(data, TESDialoguePackageData::ptopic, Ptr::new(0x55));
        e.register(SAVE_RAW, |_, _| Ret::default());
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        e.register(PACKAGE_LOCATION_SAVE_GAME, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_b6f0, &args![data, buffer]);
        let log = e.call_log.take().unwrap();
        let raw: Vec<_> = log.iter().filter(|(addr, _)| *addr == SAVE_RAW).collect();
        let base = data.addr();
        let sizes: Vec<_> = raw.iter().map(|(_, a)| (a[1], a[2])).collect();
        assert_eq!(
            &sizes[..7],
            &[
                (base + 4, 4),
                (base + 0x10, 1),
                (base + 0x11, 1),
                (base + 0x12, 1),
                (base + 0x14, 4),
                (base + 0x18, 1),
                (base + 0x1c, 4)
            ]
        );
        assert_eq!(raw.len(), 8);
        let topic = log.iter().find(|(addr, _)| *addr == SAVE_FORM_ID).unwrap();
        assert_eq!(topic.1[..], [buffer.addr(), 0x55, 0]);
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == PACKAGE_LOCATION_SAVE_GAME
                && a[..] == [location.addr(), buffer.addr()]));
    }

    #[test]
    fn save_game_without_a_location_skips_the_location() {
        let mut e = engine_with_casts();
        let data = dialogue_data(&mut e);
        let buffer = Ptr::<()>::new(e.mem.alloc(8));
        let flag = std::rc::Rc::new(std::cell::Cell::new(9u8));
        let sink = flag.clone();
        e.register_double(SAVE_RAW, move |e, a| {
            if a[2] == 1 {
                sink.set(e.mem.u8(a[1]));
            }
            Ret::default()
        });
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        e.register(PACKAGE_LOCATION_SAVE_GAME, |_, _| panic!("no location"));
        e.call(0x0067_b6f0, &args![data, buffer]);
        assert_eq!(flag.get(), 0);
    }

    #[test]
    fn load_game_reads_fields_and_builds_the_location() {
        let mut e = engine_with_casts();
        let data = dialogue_data(&mut e);
        let buffer = Ptr::<()>::new(e.mem.alloc(8));
        let block = e.mem.alloc(0x0c);
        // Each raw read stores a recognizable value; the flag byte reads 1.
        e.register(LOAD_RAW, |e, a| {
            match a[2] {
                4 => e.mem.set_u32(a[1], 0x1111),
                _ => e.mem.set_u8(a[1], 1),
            }
            Ret::default()
        });
        e.register(LOAD_FORM_ID, |_, _| Ret {
            eax: 0x0100_0077,
            ..Ret::default()
        });
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0x0c);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(PACKAGE_LOCATION_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(PACKAGE_LOCATION_LOAD_GAME, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_b7d0, &args![data, buffer]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(
            |(addr, a)| *addr == PACKAGE_LOCATION_LOAD_GAME && a[..] == [block, buffer.addr()]
        ));
        assert_eq!(e.mem.u32(data.addr() + 4), 0x1111);
        assert_eq!(e.mem.u8(data.addr() + 0x12), 1);
        assert_eq!(
            e.get(data, TESDialoguePackageData::ptopic).addr(),
            0x0100_0077
        );
        assert_eq!(
            e.get(data, TESDialoguePackageData::pTargetLocation).addr(),
            block
        );
    }

    #[test]
    fn load_game_without_the_location_flag_leaves_it_alone() {
        let mut e = engine_with_casts();
        let data = dialogue_data(&mut e);
        let buffer = Ptr::<()>::new(e.mem.alloc(8));
        e.register(LOAD_RAW, |_, _| Ret::default());
        e.register(LOAD_FORM_ID, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |_, _| panic!("no location"));
        e.call(0x0067_b7d0, &args![data, buffer]);
        assert!(e
            .get(data, TESDialoguePackageData::pTargetLocation)
            .is_null());
    }

    #[test]
    fn init_load_game_forwards_only_with_a_location() {
        let mut e = engine_with_casts();
        let data = dialogue_data(&mut e);
        e.register(PACKAGE_LOCATION_INIT_LOAD_GAME, |_, _| {
            panic!("no location")
        });
        e.call(0x0067_b920, &args![data, 7u32]);
        let location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(data, TESDialoguePackageData::pTargetLocation, location);
        e.register(PACKAGE_LOCATION_INIT_LOAD_GAME, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_b920, &args![data, 7u32]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == PACKAGE_LOCATION_INIT_LOAD_GAME
                && a[..] == [location.addr(), 7]));
    }
}

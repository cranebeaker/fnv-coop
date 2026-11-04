//! `fallout shared/packagelocation.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `PackageLocation` is the 12-byte place a package refers to: a location
//! type byte, a radius, and one word whose meaning depends on the type (a
//! reference, a cell, an object form, an object type). The type numbers the
//! code distinguishes are 0 (reference), 1 (cell), 2 and 3 (resolved from the
//! acting actor), 4 (object form), 5 (object type), 6 (resolved from the
//! actor's reference) and 255 (cleared); the exe's own names for them were not
//! recovered, so they are only described here.

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast` (cdecl: pointer, vfptr delta, source type descriptor,
/// target type descriptor, is-reference flag).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Delete wrapper (`00401030`), called with the object.
const DELETE_OBJECT: u32 = 0x0040_1030;
/// Form type byte getter (`00401170`, the byte at +4 of its `this`).
const FORM_TYPE_BYTE: u32 = 0x0040_1170;
/// Returns a global flag byte (`00401500`, reads `0x011c54ba`): when set,
/// the 12-byte save record is byte-swapped around the write.
const SWAP_BYTES_FLAG: u32 = 0x0040_1500;
/// `memset` (cdecl: destination, fill value, size).
const MEMSET: u32 = 0x0040_3d30;
/// Byte-swaps the two dwords at +4 and +8 of a save record (`0067eff0`).
const SWAP_RECORD: u32 = 0x0067_eff0;
/// `TESForm::__AddChunkData` (Xbox PDB), cdecl: form, data, size.
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `TESForm::GetFile` (Xbox PDB), thiscall with the file index (-1: last).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB), cdecl: pointer to the form ID,
/// file.
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Form lookup by form ID (`004839c0`, cdecl), 0 if there is no such form.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// Form ID getter (`0084e3a0`, the dword at +0xC of its `this`).
const FORM_ID: u32 = 0x0084_e3a0;
/// Test on a form (`00474cb0`, thiscall): non-zero when its virtual slot
/// `0x130` (used here to fetch a name) returns something.
const FORM_HAS_NAME: u32 = 0x0047_4cb0;
/// Form virtual slot `0x130`: the text the master-file warnings print.
const FORM_NAME_SLOT: u32 = 0x130;
/// Log call (`005b5e40`, cdecl: format, form ID, owner name or ID).
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// `007af430` (the engine map calls it `BGSSaveFormBuffer::GetForm`, a
/// folded-code name): the body returns the dword at +0x20 of its `this`.
const GET_FORM_FIELD: u32 = 0x007a_f430;
/// First helper of `fn_0067f220` (`0062ddb0`, cdecl): from a reference to
/// the object the test needs, 0 if there is none.
const FIND_TEST_OBJECT: u32 = 0x0062_ddb0;
/// Second helper of `fn_0067f220` (`0062df20`, thiscall: the second
/// argument, then the reference) producing the byte result.
const TEST_OBJECT_RESULT: u32 = 0x0062_df20;
/// Reference accessor (`00569b80`, thiscall).
const REFERENCE_RESOLVE: u32 = 0x0056_9b80;
/// `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB, `008d8520`): the
/// dword at +0x68 of its `this`.
const SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// Virtual slot `0x514` of that object, called with no arguments.
const ACQUIRE_OBJECT_SLOT: u32 = 0x514;
/// Pointer to the save/load stream object (`0x011de45c`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// Stream write (`008579b0`, thiscall: data, size).
const STREAM_WRITE: u32 = 0x0085_79b0;
/// `TESSaveLoadGame::SaveNumericID` (Xbox PDB, `00857a10`): data, size.
const STREAM_SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
/// Stream read (`008579e0`, thiscall: data, size).
const STREAM_READ: u32 = 0x0085_79e0;
/// `TESSaveLoadGame::LoadNumericID` (Xbox PDB, `00857aa0`): data, size.
const STREAM_LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
/// Save-game buffer write (`00865e50`, thiscall: data, size, 0).
const SAVE_BUFFER_WRITE: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (Xbox PDB, `00865df0`): form
/// pointer, 0.
const SAVE_BUFFER_FORM_ID: u32 = 0x0086_5df0;
/// Load-game buffer read (`00864980`, thiscall: destination, size).
const LOAD_BUFFER_READ: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (Xbox PDB, `008648e0`): address of
/// the word to fill.
const LOAD_BUFFER_FORM_ID: u32 = 0x0086_48e0;

/// RTTI type descriptors the casts name: `TESForm`, `TESObjectREFR` and
/// `TESObjectCELL` (decorated names read from the exe).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_TES_OBJECT_CELL: u32 = 0x0118_3fb4;

/// Location type: cleared.
const LOC_TYPE_NONE: i32 = -1;
/// Location type 0: a reference.
const LOC_TYPE_REFERENCE: i32 = 0;
/// Location type 1: a cell.
const LOC_TYPE_CELL: i32 = 1;
/// Location type 4: an object form.
const LOC_TYPE_OBJECT: i32 = 4;
/// Location type 5: an object type.
const LOC_TYPE_OBJECT_TYPE: i32 = 5;

layout! {
    /// `PackageLocation` (Xbox PDB), 0xC bytes on both builds.
    pub struct PackageLocation: 0x0c {
        /// `eLocType` (Xbox PDB): `char`.
        0x00 eLocType: i8,
        /// `iRad` (Xbox PDB).
        0x04 iRad: u32,
        /// `pObject` (Xbox PDB): `TESForm*`.
        0x08 pObject: Ptr,
        /// `eObjType` (Xbox PDB): `PACKAGE_OBJECT_TYPE`.
        0x08 eObjType: u32,
        /// `pRef` (Xbox PDB): `TESObjectREFR*`.
        0x08 pRef: Ptr,
        /// `pCell` (Xbox PDB): `TESObjectCELL*`.
        0x08 pCell: Ptr,
    }
}

/// Types 0 to 4 keep a form (or its form ID in a save) in the shared word.
fn holds_form(location_type: i32) -> bool {
    (0..=4).contains(&location_type)
}

// Translated from 00670b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::~PackageLocation` (Xbox PDB): clears the location and,
/// when bit 0 of `flags` is set, deletes the block. Returns `this`.
pub fn package_location_destructor(
    e: &mut Engine,
    this: Ptr<PackageLocation>,
    flags: u32,
) -> Ptr<PackageLocation> {
    fn_0067f110(e, this);
    if flags & 1 != 0 {
        e.call(DELETE_OBJECT, &args![this]);
    }
    this
}

// Translated from 00678ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::GetLocType` (Xbox PDB): the location type, sign
/// extended.
pub fn package_location_get_loc_type(e: &mut Engine, this: Ptr<PackageLocation>) -> i32 {
    e.get(this, PackageLocation::eLocType) as i32
}

// Translated from 0067f030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::PackageLocation` (Xbox PDB): type 3, radius 0, no
/// object. Returns `this`.
pub fn package_location_package_location(
    e: &mut Engine,
    this: Ptr<PackageLocation>,
) -> Ptr<PackageLocation> {
    e.set(this, PackageLocation::eLocType, 3);
    e.set(this, PackageLocation::iRad, 0);
    e.set(this, PackageLocation::pObject, Ptr::NULL);
    this
}

// Translated from 0067f060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the location from a 12-byte save record (type byte, the stored
/// word at +4, the radius at +8; the layout `PackageLocation::Save` writes).
/// The type is copied and the radius is taken from the record's +8. The
/// shared word is taken from +4 for types 0, 1, 4 and 5 and cleared for types
/// 2 and 3; for type 1 the radius is cleared again. Other types leave the
/// shared word as it was. Returns `this`.
pub fn fn_0067f060(
    e: &mut Engine,
    this: Ptr<PackageLocation>,
    record: Ptr<PackageLocation>,
) -> Ptr<PackageLocation> {
    let record_type = e.mem.u8(record.addr());
    e.mem.set_u8(this.addr(), record_type);
    let radius = e.mem.u32(record.addr() + 8);
    e.set(this, PackageLocation::iRad, radius);
    let stored = Ptr::new(e.mem.u32(record.addr() + 4));
    match record_type as i8 {
        1 => {
            e.set(this, PackageLocation::pObject, stored);
            e.set(this, PackageLocation::iRad, 0);
        }
        0 | 4 | 5 => e.set(this, PackageLocation::pObject, stored),
        2 | 3 => e.set(this, PackageLocation::pObject, Ptr::NULL),
        _ => {}
    }
    this
}

// Translated from 0067f110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the location: type 255, radius 0, no object.
pub fn fn_0067f110(e: &mut Engine, this: Ptr<PackageLocation>) {
    e.set(this, PackageLocation::eLocType, -1);
    e.set(this, PackageLocation::iRad, 0);
    e.set(this, PackageLocation::pObject, Ptr::NULL);
}

// Translated from 0067f140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::SetLocType` (Xbox PDB): when the type changes, stores
/// it and clears what belonged to the old one: the shared word for types 0
/// to 6, and the radius too for type 1.
pub fn package_location_set_loc_type(
    e: &mut Engine,
    this: Ptr<PackageLocation>,
    location_type: i32,
) {
    if location_type == package_location_get_loc_type(e, this) {
        return;
    }
    e.set(this, PackageLocation::eLocType, location_type as i8);
    match package_location_get_loc_type(e, this) {
        1 => {
            e.set(this, PackageLocation::pObject, Ptr::NULL);
            e.set(this, PackageLocation::iRad, 0);
        }
        0..=6 => e.set(this, PackageLocation::pObject, Ptr::NULL),
        _ => {}
    }
}

// Translated from 0067f1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The radius, 0 for the cleared type and for type 1.
pub fn fn_0067f1c0(e: &mut Engine, this: Ptr<PackageLocation>) -> u32 {
    let location_type = e.get(this, PackageLocation::eLocType);
    if location_type == -1 || location_type == 1 {
        0
    } else {
        e.get(this, PackageLocation::iRad)
    }
}

// Translated from 0067f1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the radius, except for the cleared type and type 1.
pub fn fn_0067f1f0(e: &mut Engine, this: Ptr<PackageLocation>, radius: u32) {
    let location_type = e.get(this, PackageLocation::eLocType);
    if location_type != -1 && location_type != 1 {
        e.set(this, PackageLocation::iRad, radius);
    }
}

// Translated from 0067f220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves the location's reference for `actor` (`fn_0067f2a0`) and tests
/// it. Always clears `*result` first. Returns `false` when there is no
/// reference or the byte `00401170` reads from the dword at +0x20 of the
/// reference is not `0x15`; returns `true` with `*result` left 0 when
/// `0062ddb0` finds no object for the reference; otherwise returns `true`
/// with `*result` set to the low byte of
/// `0062df20(object, second, reference)`.
pub fn fn_0067f220(
    e: &mut Engine,
    this: Ptr<PackageLocation>,
    actor: Ptr,
    second: u32,
    result: Ptr,
) -> bool {
    e.mem.set_u8(result.addr(), 0);
    let reference = fn_0067f2a0(e, this, actor);
    if reference.is_null() {
        return false;
    }
    let form = e.call(GET_FORM_FIELD, &args![reference]).u32();
    let form_type = e.call(FORM_TYPE_BYTE, &args![form]).u32();
    if form_type != 0x15 {
        return false;
    }
    let object = e.call(FIND_TEST_OBJECT, &args![reference]).u32();
    if object == 0 {
        return true;
    }
    let value = e
        .call(TEST_OBJECT_RESULT, &args![object, second, reference])
        .u8();
    e.mem.set_u8(result.addr(), value);
    true
}

// Translated from 0067f2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves the reference the location stands for, given the acting
/// `actor` (or 0):
/// - type 0: the stored reference, if any;
/// - types 2 and 3: the actor;
/// - types 4 and 5: what slot `0x514` of the object `008d8520(actor)` returns,
///   or the actor if that is 0;
/// - type 6: `00569b80(actor)` when the actor and that result are non-zero;
/// - other types: 0.
pub fn fn_0067f2a0(e: &mut Engine, this: Ptr<PackageLocation>, actor: Ptr) -> Ptr {
    let mut resolved = Ptr::NULL;
    match package_location_get_loc_type(e, this) {
        0 => {
            if !package_location_get_loc_reference(e, this).is_null() {
                resolved = package_location_get_loc_reference(e, this);
            }
        }
        2 | 3 => {
            if !actor.is_null() {
                resolved = actor;
            }
        }
        4 | 5 => {
            let object = e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
            resolved = e.vcall(object, ACQUIRE_OBJECT_SLOT, &args![]).ptr();
            if resolved.is_null() {
                resolved = actor;
            }
        }
        6 if !actor.is_null() && e.call(REFERENCE_RESOLVE, &args![actor]).u32() != 0 => {
            resolved = e.call(REFERENCE_RESOLVE, &args![actor]).ptr();
        }
        _ => {}
    }
    resolved
}

// Translated from 0067f390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::GetLocReference` (Xbox PDB): the reference for type 0,
/// otherwise 0.
pub fn package_location_get_loc_reference(e: &mut Engine, this: Ptr<PackageLocation>) -> Ptr {
    if e.get(this, PackageLocation::eLocType) as i32 == LOC_TYPE_REFERENCE {
        e.get(this, PackageLocation::pRef)
    } else {
        Ptr::NULL
    }
}

// Translated from 0067f3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::SetLocReference` (Xbox PDB): becomes type 0 holding
/// `reference`.
pub fn package_location_set_loc_reference(
    e: &mut Engine,
    this: Ptr<PackageLocation>,
    reference: Ptr,
) {
    e.set(this, PackageLocation::eLocType, LOC_TYPE_REFERENCE as i8);
    e.set(this, PackageLocation::pRef, reference);
}

// Translated from 0067f3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::GetLocCell` (Xbox PDB): the cell for type 1, otherwise
/// 0.
pub fn package_location_get_loc_cell(e: &mut Engine, this: Ptr<PackageLocation>) -> Ptr {
    if e.get(this, PackageLocation::eLocType) as i32 == LOC_TYPE_CELL {
        e.get(this, PackageLocation::pCell)
    } else {
        Ptr::NULL
    }
}

// Translated from 0067f410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::SetLocCell` (Xbox PDB): becomes type 1 holding `cell`.
pub fn package_location_set_loc_cell(e: &mut Engine, this: Ptr<PackageLocation>, cell: Ptr) {
    e.set(this, PackageLocation::eLocType, LOC_TYPE_CELL as i8);
    e.set(this, PackageLocation::pCell, cell);
}

// Translated from 0067f430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::GetLocObject` (Xbox PDB): the object form for type 4,
/// otherwise 0.
pub fn package_location_get_loc_object(e: &mut Engine, this: Ptr<PackageLocation>) -> Ptr {
    if e.get(this, PackageLocation::eLocType) as i32 == LOC_TYPE_OBJECT {
        e.get(this, PackageLocation::pObject)
    } else {
        Ptr::NULL
    }
}

// Translated from 0067f460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Becomes type 4 holding the object form `object` (the setter paired with
/// `PackageLocation::GetLocObject`; the map gives it no name).
pub fn fn_0067f460(e: &mut Engine, this: Ptr<PackageLocation>, object: Ptr) {
    e.set(this, PackageLocation::eLocType, LOC_TYPE_OBJECT as i8);
    e.set(this, PackageLocation::pObject, object);
}

// Translated from 0067f480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::GetLocObjectType` (Xbox PDB): the object type for
/// type 5, otherwise 0.
pub fn package_location_get_loc_object_type(e: &mut Engine, this: Ptr<PackageLocation>) -> u32 {
    if e.get(this, PackageLocation::eLocType) as i32 == LOC_TYPE_OBJECT_TYPE {
        e.get(this, PackageLocation::eObjType)
    } else {
        0
    }
}

// Translated from 0067f4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Becomes type 5 holding the object type `object_type` (the setter paired
/// with `PackageLocation::GetLocObjectType`; the map gives it no name).
pub fn fn_0067f4b0(e: &mut Engine, this: Ptr<PackageLocation>, object_type: u32) {
    e.set(this, PackageLocation::eLocType, LOC_TYPE_OBJECT_TYPE as i8);
    e.set(this, PackageLocation::eObjType, object_type);
}

// Translated from 0067f4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::Copy` (Xbox PDB): copies `source` into this location.
/// Does nothing for a null source, or for a cleared source unless
/// `copy_cleared` is set. Takes the type, then the radius and the shared
/// word that belong to it (no radius for type 1; only the radius for types
/// 2, 3, 6 and 7).
pub fn package_location_copy(
    e: &mut Engine,
    this: Ptr<PackageLocation>,
    source: Ptr<PackageLocation>,
    copy_cleared: bool,
) {
    if source.is_null() {
        return;
    }
    if !copy_cleared && package_location_get_loc_type(e, source) == LOC_TYPE_NONE {
        return;
    }
    let source_type = package_location_get_loc_type(e, source);
    package_location_set_loc_type(e, this, source_type);
    match package_location_get_loc_type(e, this) {
        0 => {
            let radius = fn_0067f1c0(e, source);
            fn_0067f1f0(e, this, radius);
            let reference = package_location_get_loc_reference(e, source);
            package_location_set_loc_reference(e, this, reference);
        }
        1 => {
            let cell = package_location_get_loc_cell(e, source);
            package_location_set_loc_cell(e, this, cell);
        }
        4 => {
            let radius = fn_0067f1c0(e, source);
            fn_0067f1f0(e, this, radius);
            let object = package_location_get_loc_object(e, source);
            fn_0067f460(e, this, object);
        }
        5 => {
            let radius = fn_0067f1c0(e, source);
            fn_0067f1f0(e, this, radius);
            let object_type = package_location_get_loc_object_type(e, source);
            fn_0067f4b0(e, this, object_type);
        }
        2 | 3 | 6 | 7 => {
            let radius = fn_0067f1c0(e, source);
            fn_0067f1f0(e, this, radius);
        }
        _ => {}
    }
}

// Translated from 0067f600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::Compare` (Xbox PDB): `true` when `other` is null or
/// differs from this location in type, radius, reference, cell, object or
/// object type; `false` when they are equal.
pub fn package_location_compare(
    e: &mut Engine,
    this: Ptr<PackageLocation>,
    other: Ptr<PackageLocation>,
) -> bool {
    if other.is_null() {
        return true;
    }
    package_location_get_loc_type(e, this) != package_location_get_loc_type(e, other)
        || fn_0067f1c0(e, this) != fn_0067f1c0(e, other)
        || package_location_get_loc_reference(e, this)
            != package_location_get_loc_reference(e, other)
        || package_location_get_loc_cell(e, this) != package_location_get_loc_cell(e, other)
        || package_location_get_loc_object(e, this) != package_location_get_loc_object(e, other)
        || package_location_get_loc_object_type(e, this)
            != package_location_get_loc_object_type(e, other)
}

// Translated from 0067f6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::Save` (Xbox PDB): writes the 12-byte record (type byte;
/// at +4 the object type for type 5, otherwise the form ID of the stored
/// form if there is one; at +8 the radius) to `form` with
/// `TESForm::__AddChunkData`, byte-swapping +4 and +8 around the write when
/// the swap flag is set.
pub fn package_location_save(e: &mut Engine, this: Ptr<PackageLocation>, form: Ptr) {
    e.with_stack(0xc, |e, record| {
        e.call(MEMSET, &args![record, 0u32, 0xcu32]);
        let location_type = package_location_get_loc_type(e, this);
        e.mem.set_u8(record.addr(), location_type as u8);
        if package_location_get_loc_type(e, this) == LOC_TYPE_OBJECT_TYPE {
            let object_type = package_location_get_loc_object_type(e, this);
            e.mem.set_u32(record.addr() + 4, object_type);
        } else {
            let stored = e.get(this, PackageLocation::pObject);
            if !stored.is_null() {
                let id = e.call(FORM_ID, &args![stored]).u32();
                e.mem.set_u32(record.addr() + 4, id);
            }
        }
        let radius = fn_0067f1c0(e, this);
        e.mem.set_u32(record.addr() + 8, radius);
        if e.call(SWAP_BYTES_FLAG, &args![]).u8() != 0 {
            e.call(SWAP_RECORD, &args![record]);
        }
        e.call(ADD_CHUNK_DATA, &args![form, record, 0xcu32]);
        if e.call(SWAP_BYTES_FLAG, &args![]).u8() != 0 {
            e.call(SWAP_RECORD, &args![record]);
        }
    });
}

/// The "unable to find Package Location ..." warning of `InitItem`: with the
/// owner's name when `00474cb0` says it has one, otherwise with its form ID.
fn report_missing(e: &mut Engine, owner: Ptr, id: u32, named_format: u32, id_format: u32) {
    if e.call(FORM_HAS_NAME, &args![owner]).u32() != 0 {
        let name = e.vcall(owner.addr(), FORM_NAME_SLOT, &args![]).u32();
        e.call(LOG_MESSAGE, &args![named_format, id, name]);
    } else {
        let owner_id = e.call(FORM_ID, &args![owner]).u32();
        e.call(LOG_MESSAGE, &args![id_format, id, owner_id]);
    }
}

/// Looks up the form with the given ID and casts it from `TESForm` to
/// `target` (`__RTDynamicCast`).
fn lookup_and_cast(e: &mut Engine, id: u32, target: u32) -> Ptr {
    let found = e.call(LOOKUP_FORM, &args![id]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![found, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .ptr()
}

// Translated from 0067f760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::InitItem` (Xbox PDB): turns the stored form ID into the
/// form, after resolving it against the owner `form`'s file
/// (`TESForm::AddCompileIndex`). Type 0 looks up a `TESObjectREFR`, type 1 a
/// `TESObjectCELL`, type 4 any form; a form that is not found is reported
/// (with the owner's name or ID) and the location keeps its type with an
/// empty word. Type 5 and the other types are left alone.
pub fn package_location_init_item(e: &mut Engine, this: Ptr<PackageLocation>, owner: Ptr) {
    if package_location_get_loc_type(e, this) == LOC_TYPE_OBJECT_TYPE {
        return;
    }
    let stored = e.get(this, PackageLocation::pObject).addr();
    let id = e.with_stack(4, |e, id_slot| {
        e.mem.set_u32(id_slot.addr(), stored);
        let file = e.call(FORM_GET_FILE, &args![owner, -1i32]).u32();
        e.call(ADD_COMPILE_INDEX, &args![id_slot, file]);
        e.mem.u32(id_slot.addr())
    });
    match e.get(this, PackageLocation::eLocType) {
        0 => {
            let reference = lookup_and_cast(e, id, TYPE_TES_OBJECT_REFR);
            if !reference.is_null() {
                package_location_set_loc_reference(e, this, reference);
            } else {
                report_missing(e, owner, id, 0x0106_8d90, 0x0106_8d38);
                package_location_set_loc_reference(e, this, Ptr::NULL);
            }
        }
        1 => {
            let cell = lookup_and_cast(e, id, TYPE_TES_OBJECT_CELL);
            if !cell.is_null() {
                package_location_set_loc_cell(e, this, cell);
            } else {
                report_missing(e, owner, id, 0x0106_8ee0, 0x0106_8e90);
                package_location_set_loc_cell(e, this, Ptr::NULL);
            }
        }
        4 => {
            let object = e.call(LOOKUP_FORM, &args![id]).ptr::<()>();
            if !object.is_null() {
                fn_0067f460(e, this, object);
            } else {
                report_missing(e, owner, id, 0x0106_8e40, 0x0106_8de8);
                fn_0067f460(e, this, Ptr::NULL);
            }
        }
        _ => {}
    }
}

// Translated from 0067f970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::IsLocationGeneric` (Xbox PDB): the type is above 3.
pub fn package_location_is_location_generic(e: &mut Engine, this: Ptr<PackageLocation>) -> bool {
    package_location_get_loc_type(e, this) > 3
}

// Translated from 0067f990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Size in bytes of the location in a save game: 1 for the type, 4 for the
/// radius, and 4 more for the shared word of types 0 to 5; 5 for any other
/// type.
pub fn fn_0067f990(e: &mut Engine, this: Ptr<PackageLocation>) -> u16 {
    let mut size: u16 = 1 + 4;
    let location_type = e.get(this, PackageLocation::eLocType) as i32;
    if holds_form(location_type) || location_type == LOC_TYPE_OBJECT_TYPE {
        size += 4;
    }
    size
}

// Translated from 0067fa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the location to the save-game stream (`0x011de45c`): the type
/// byte, the radius, then for types 0 to 4 the form ID of the stored form
/// (0 if none) and for type 5 the object type.
pub fn fn_0067fa00(e: &mut Engine, this: Ptr<PackageLocation>) {
    let stream = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(STREAM_WRITE, &args![stream, this, 1u32]);
    e.call(STREAM_WRITE, &args![stream, this.addr() + 4, 4u32]);
    let location_type = e.get(this, PackageLocation::eLocType) as i32;
    if holds_form(location_type) {
        e.with_stack(4, |e, id_slot| {
            let stored = e.get(this, PackageLocation::pObject);
            if !stored.is_null() {
                let id = e.call(FORM_ID, &args![stored]).u32();
                e.mem.set_u32(id_slot.addr(), id);
            }
            e.call(STREAM_SAVE_NUMERIC_ID, &args![stream, id_slot, 4u32]);
        });
    } else if location_type == LOC_TYPE_OBJECT_TYPE {
        e.call(STREAM_WRITE, &args![stream, this.addr() + 8, 4u32]);
    }
}

// Translated from 0067faa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the location from the save-game stream (`0x011de45c`), the inverse
/// of `fn_0067fa00`: type byte, radius, then the numeric ID (types 0 to 4,
/// kept in the shared word as the form ID) or the object type (type 5).
pub fn fn_0067faa0(e: &mut Engine, this: Ptr<PackageLocation>) {
    let stream = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(STREAM_READ, &args![stream, this, 1u32]);
    e.call(STREAM_READ, &args![stream, this.addr() + 4, 4u32]);
    let location_type = e.get(this, PackageLocation::eLocType) as i32;
    if holds_form(location_type) {
        let id = e.with_stack(4, |e, id_slot| {
            e.call(STREAM_LOAD_NUMERIC_ID, &args![stream, id_slot, 4u32]);
            e.mem.u32(id_slot.addr())
        });
        e.set(this, PackageLocation::pObject, Ptr::new(id));
    } else if location_type == LOC_TYPE_OBJECT_TYPE {
        e.call(STREAM_READ, &args![stream, this.addr() + 8, 4u32]);
    }
}

// Translated from 0067fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// For types 0 to 4, replaces the stored form ID with the form `004839c0`
/// looks up (0 if there is none).
pub fn fn_0067fb20(e: &mut Engine, this: Ptr<PackageLocation>) {
    if holds_form(e.get(this, PackageLocation::eLocType) as i32) {
        let id = e.get(this, PackageLocation::pObject).addr();
        let form = e.call(LOOKUP_FORM, &args![id]).ptr::<()>();
        e.set(this, PackageLocation::pObject, form);
    }
}

// Translated from 0067fb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::SaveGame` (Xbox PDB): writes the type byte and radius to
/// the save buffer, then for types 0 to 4 the stored form (as a form
/// reference, `BGSSaveGameBuffer::SaveFormID_ov2`) and for type 5 the
/// object type.
pub fn package_location_save_game(e: &mut Engine, this: Ptr<PackageLocation>, buffer: Ptr) {
    e.call(SAVE_BUFFER_WRITE, &args![buffer, this, 1u32, 0u32]);
    e.call(
        SAVE_BUFFER_WRITE,
        &args![buffer, this.addr() + 4, 4u32, 0u32],
    );
    let location_type = e.get(this, PackageLocation::eLocType) as i32;
    if holds_form(location_type) {
        let stored = e.get(this, PackageLocation::pObject);
        e.call(SAVE_BUFFER_FORM_ID, &args![buffer, stored, 0u32]);
    } else if location_type == LOC_TYPE_OBJECT_TYPE {
        e.call(
            SAVE_BUFFER_WRITE,
            &args![buffer, this.addr() + 8, 4u32, 0u32],
        );
    }
}

// Translated from 0067fbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::LoadGame` (Xbox PDB): reads what `SaveGame` wrote: the
/// type byte and radius, then for types 0 to 4 the form reference (loaded
/// into the shared word by `BGSLoadGameBuffer::LoadFormID_ov2`) and for type
/// 5 the object type.
pub fn package_location_load_game(e: &mut Engine, this: Ptr<PackageLocation>, buffer: Ptr) {
    e.call(LOAD_BUFFER_READ, &args![buffer, this, 1u32]);
    e.call(LOAD_BUFFER_READ, &args![buffer, this.addr() + 4, 4u32]);
    let location_type = e.get(this, PackageLocation::eLocType) as i32;
    if holds_form(location_type) {
        e.call(LOAD_BUFFER_FORM_ID, &args![buffer, this.addr() + 8]);
    } else if location_type == LOC_TYPE_OBJECT_TYPE {
        e.call(LOAD_BUFFER_READ, &args![buffer, this.addr() + 8, 4u32]);
    }
}

// Translated from 0067fc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageLocation::InitLoadGame` (Xbox PDB): for types 0 to 4, replaces
/// the stored form ID by the form `004839c0` looks up, leaving 0 as 0.
pub fn package_location_init_load_game(e: &mut Engine, this: Ptr<PackageLocation>) {
    if holds_form(e.get(this, PackageLocation::eLocType) as i32) {
        let id = e.get(this, PackageLocation::pObject);
        let form = if id.is_null() {
            Ptr::NULL
        } else {
            e.call(LOOKUP_FORM, &args![id]).ptr::<()>()
        };
        e.set(this, PackageLocation::pObject, form);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00670b30,
            package_location_destructor(Ptr<PackageLocation>, u32) -> Ptr<PackageLocation>
        ),
        entry!(
            0x00678ca0,
            package_location_get_loc_type(Ptr<PackageLocation>) -> i32
        ),
        entry!(
            0x0067f030,
            package_location_package_location(Ptr<PackageLocation>) -> Ptr<PackageLocation>
        ),
        entry!(
            0x0067f060,
            fn_0067f060(Ptr<PackageLocation>, Ptr<PackageLocation>) -> Ptr<PackageLocation>
        ),
        entry!(0x0067f110, fn_0067f110(Ptr<PackageLocation>)),
        entry!(
            0x0067f140,
            package_location_set_loc_type(Ptr<PackageLocation>, i32)
        ),
        entry!(0x0067f1c0, fn_0067f1c0(Ptr<PackageLocation>) -> u32),
        entry!(0x0067f1f0, fn_0067f1f0(Ptr<PackageLocation>, u32)),
        entry!(
            0x0067f220,
            fn_0067f220(Ptr<PackageLocation>, Ptr, u32, Ptr) -> bool
        ),
        entry!(0x0067f2a0, fn_0067f2a0(Ptr<PackageLocation>, Ptr) -> Ptr),
        entry!(
            0x0067f390,
            package_location_get_loc_reference(Ptr<PackageLocation>) -> Ptr
        ),
        entry!(
            0x0067f3c0,
            package_location_set_loc_reference(Ptr<PackageLocation>, Ptr)
        ),
        entry!(
            0x0067f3e0,
            package_location_get_loc_cell(Ptr<PackageLocation>) -> Ptr
        ),
        entry!(
            0x0067f410,
            package_location_set_loc_cell(Ptr<PackageLocation>, Ptr)
        ),
        entry!(
            0x0067f430,
            package_location_get_loc_object(Ptr<PackageLocation>) -> Ptr
        ),
        entry!(0x0067f460, fn_0067f460(Ptr<PackageLocation>, Ptr)),
        entry!(
            0x0067f480,
            package_location_get_loc_object_type(Ptr<PackageLocation>) -> u32
        ),
        entry!(0x0067f4b0, fn_0067f4b0(Ptr<PackageLocation>, u32)),
        entry!(
            0x0067f4d0,
            package_location_copy(Ptr<PackageLocation>, Ptr<PackageLocation>, bool)
        ),
        entry!(
            0x0067f600,
            package_location_compare(Ptr<PackageLocation>, Ptr<PackageLocation>) -> bool
        ),
        entry!(0x0067f6c0, package_location_save(Ptr<PackageLocation>, Ptr)),
        entry!(
            0x0067f760,
            package_location_init_item(Ptr<PackageLocation>, Ptr)
        ),
        entry!(
            0x0067f970,
            package_location_is_location_generic(Ptr<PackageLocation>) -> bool
        ),
        entry!(0x0067f990, fn_0067f990(Ptr<PackageLocation>) -> u16),
        entry!(0x0067fa00, fn_0067fa00(Ptr<PackageLocation>)),
        entry!(0x0067faa0, fn_0067faa0(Ptr<PackageLocation>)),
        entry!(0x0067fb20, fn_0067fb20(Ptr<PackageLocation>)),
        entry!(
            0x0067fb60,
            package_location_save_game(Ptr<PackageLocation>, Ptr)
        ),
        entry!(
            0x0067fbe0,
            package_location_load_game(Ptr<PackageLocation>, Ptr)
        ),
        entry!(
            0x0067fc50,
            package_location_init_load_game(Ptr<PackageLocation>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;
    type Written = Rc<RefCell<Vec<u8>>>;
    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A location with the given type byte, radius and shared word.
    fn location(e: &mut Engine, location_type: i8, radius: u32, word: u32) -> Ptr<PackageLocation> {
        let p: Ptr<PackageLocation> = e.new_object();
        e.set(p, PackageLocation::eLocType, location_type);
        e.set(p, PackageLocation::iRad, radius);
        e.mem.set_u32(p.addr() + 8, word);
        p
    }

    fn kind(e: &Engine, p: Ptr<PackageLocation>) -> i8 {
        e.get(p, PackageLocation::eLocType)
    }

    fn radius(e: &Engine, p: Ptr<PackageLocation>) -> u32 {
        e.get(p, PackageLocation::iRad)
    }

    fn word(e: &Engine, p: Ptr<PackageLocation>) -> u32 {
        e.mem.u32(p.addr() + 8)
    }

    /// Registers a double for `addr` that records its argument words and
    /// returns `result` in EAX.
    fn recorder(e: &mut Engine, addr: u32, result: u32) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(vec![]));
        let seen = calls.clone();
        e.register_double(addr, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
        calls
    }

    #[test]
    fn destructor_clears_and_deletes_only_with_the_flag() {
        let mut e = Engine::new();
        let calls = recorder(&mut e, DELETE_OBJECT, 0);
        let p = location(&mut e, 4, 9, 0x1234);
        assert_eq!(package_location_destructor(&mut e, p, 0), p);
        assert_eq!((kind(&e, p), radius(&e, p), word(&e, p)), (-1, 0, 0));
        assert!(calls.borrow().is_empty());
        let q = location(&mut e, 0, 9, 0x1234);
        e.call(0x0067_0b30, &args![q, 1u32]);
        assert_eq!(*calls.borrow(), vec![vec![q.addr()]]);
        assert_eq!(kind(&e, q), -1);
    }

    #[test]
    fn get_loc_type_sign_extends() {
        let mut e = Engine::new();
        let p = location(&mut e, -1, 0, 0);
        assert_eq!(e.call(0x0067_8ca0, &args![p]).i32(), -1);
        let q = location(&mut e, 5, 0, 0);
        assert_eq!(e.call(0x0067_8ca0, &args![q]).i32(), 5);
    }

    #[test]
    fn constructor_makes_an_empty_type_3_location() {
        let mut e = Engine::new();
        let p = location(&mut e, 0, 7, 0x55);
        assert_eq!(e.call(0x0067_f030, &args![p]).ptr::<()>().addr(), p.addr());
        assert_eq!((kind(&e, p), radius(&e, p), word(&e, p)), (3, 0, 0));
    }

    /// A 12-byte save record: type, stored word at +4, radius at +8.
    fn record(e: &mut Engine, record_type: u8, stored: u32, radius: u32) -> Ptr<PackageLocation> {
        let r: Ptr<PackageLocation> = e.new_object();
        e.mem.set_u8(r.addr(), record_type);
        e.mem.set_u32(r.addr() + 4, stored);
        e.mem.set_u32(r.addr() + 8, radius);
        r
    }

    #[test]
    fn construct_from_record_takes_the_word_by_type() {
        let mut e = Engine::new();
        // (record type, resulting radius, resulting word)
        let cases = [
            (0u8, 40u32, 0x500u32),
            (1, 0, 0x500),
            (2, 40, 0),
            (3, 40, 0),
            (4, 40, 0x500),
            (5, 40, 0x500),
        ];
        for (record_type, expected_radius, expected_word) in cases {
            let this = location(&mut e, 9, 3, 0x77);
            let source = record(&mut e, record_type, 0x500, 40);
            let back = e.call(0x0067_f060, &args![this, source]).ptr::<()>();
            assert_eq!(back.addr(), this.addr());
            assert_eq!(kind(&e, this), record_type as i8);
            assert_eq!(radius(&e, this), expected_radius, "type {record_type}");
            assert_eq!(word(&e, this), expected_word, "type {record_type}");
        }
        // Another type copies the radius but keeps the word.
        let this = location(&mut e, 9, 3, 0x77);
        let source = record(&mut e, 6, 0x500, 40);
        e.call(0x0067_f060, &args![this, source]);
        assert_eq!(
            (kind(&e, this), radius(&e, this), word(&e, this)),
            (6, 40, 0x77)
        );
    }

    #[test]
    fn reset_clears_everything() {
        let mut e = Engine::new();
        let p = location(&mut e, 5, 9, 0x1234);
        e.call(0x0067_f110, &args![p]);
        assert_eq!((kind(&e, p), radius(&e, p), word(&e, p)), (-1, 0, 0));
    }

    #[test]
    fn set_loc_type_clears_by_new_type_only_when_it_changes() {
        let mut e = Engine::new();
        let p = location(&mut e, 0, 9, 0x1234);
        e.call(0x0067_f140, &args![p, 0i32]);
        assert_eq!((radius(&e, p), word(&e, p)), (9, 0x1234));
        e.call(0x0067_f140, &args![p, 2i32]);
        assert_eq!((kind(&e, p), radius(&e, p), word(&e, p)), (2, 9, 0));
        e.mem.set_u32(p.addr() + 8, 0x99);
        e.call(0x0067_f140, &args![p, 1i32]);
        assert_eq!((kind(&e, p), radius(&e, p), word(&e, p)), (1, 0, 0));
        // Out of the 0..=6 range: the type is stored, nothing is cleared.
        e.mem.set_u32(p.addr() + 4, 5);
        e.mem.set_u32(p.addr() + 8, 0x99);
        e.call(0x0067_f140, &args![p, 9i32]);
        assert_eq!((kind(&e, p), radius(&e, p), word(&e, p)), (9, 5, 0x99));
        e.call(0x0067_f140, &args![p, -1i32]);
        assert_eq!((kind(&e, p), radius(&e, p), word(&e, p)), (-1, 5, 0x99));
    }

    #[test]
    fn radius_getter_hides_it_for_cleared_and_cell() {
        let mut e = Engine::new();
        for (location_type, expected) in [(-1i8, 0u32), (1, 0), (0, 25), (4, 25), (5, 25), (3, 25)]
        {
            let p = location(&mut e, location_type, 25, 0);
            assert_eq!(
                e.call(0x0067_f1c0, &args![p]).u32(),
                expected,
                "{location_type}"
            );
        }
    }

    #[test]
    fn radius_setter_ignores_cleared_and_cell() {
        let mut e = Engine::new();
        for (location_type, expected) in [(-1i8, 25u32), (1, 25), (0, 99), (4, 99), (2, 99)] {
            let p = location(&mut e, location_type, 25, 0);
            e.call(0x0067_f1f0, &args![p, 99u32]);
            assert_eq!(radius(&e, p), expected, "{location_type}");
        }
    }

    /// Doubles for the callees of the actor resolution.
    fn resolution_engine() -> (Engine, Calls, Calls) {
        let mut e = Engine::new();
        let acquire = recorder(&mut e, SAVED_ACQUIRE_OBJECT, 0);
        let resolve = recorder(&mut e, REFERENCE_RESOLVE, 0x6000);
        (e, acquire, resolve)
    }

    /// An object whose virtual slot `0x514` returns `result`.
    fn object_with_slot(e: &mut Engine, result: u32) -> u32 {
        let object = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x520);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + ACQUIRE_OBJECT_SLOT, 0x00a0_0000);
        e.register_double(0x00a0_0000, move |_, _| Ret {
            eax: result,
            ..Ret::default()
        });
        object
    }

    #[test]
    fn resolve_reference_by_type() {
        let (mut e, _, resolve) = resolution_engine();
        let actor = Ptr::<()>::new(0x5000);
        // Type 0: the stored reference, whatever the actor.
        let p = location(&mut e, 0, 0, 0x4000);
        assert_eq!(e.call(0x0067_f2a0, &args![p, actor]).u32(), 0x4000);
        assert_eq!(
            e.call(0x0067_f2a0, &args![p, Ptr::<()>::NULL]).u32(),
            0x4000
        );
        let empty = location(&mut e, 0, 0, 0);
        assert_eq!(e.call(0x0067_f2a0, &args![empty, actor]).u32(), 0);
        // Types 2 and 3: the actor.
        for location_type in [2i8, 3] {
            let p = location(&mut e, location_type, 0, 0x4000);
            assert_eq!(e.call(0x0067_f2a0, &args![p, actor]).u32(), 0x5000);
            assert_eq!(e.call(0x0067_f2a0, &args![p, Ptr::<()>::NULL]).u32(), 0);
        }
        // Types 1 and 255: nothing.
        for location_type in [1i8, -1, 7] {
            let p = location(&mut e, location_type, 0, 0x4000);
            assert_eq!(e.call(0x0067_f2a0, &args![p, actor]).u32(), 0);
        }
        // Type 6: through 00569b80, only with an actor.
        let p = location(&mut e, 6, 0, 0);
        assert_eq!(e.call(0x0067_f2a0, &args![p, actor]).u32(), 0x6000);
        assert_eq!(e.call(0x0067_f2a0, &args![p, Ptr::<()>::NULL]).u32(), 0);
        assert_eq!(resolve.borrow().len(), 2);
        e.register(REFERENCE_RESOLVE, |_, _| Ret::default());
        assert_eq!(e.call(0x0067_f2a0, &args![p, actor]).u32(), 0);
    }

    #[test]
    fn resolve_reference_for_object_types_asks_the_acquire_object() {
        for location_type in [4i8, 5] {
            let (mut e, _, _) = resolution_engine();
            let object = object_with_slot(&mut e, 0x7000);
            e.register(SAVED_ACQUIRE_OBJECT, |_, _| Ret::default());
            let acquire = recorder(&mut e, SAVED_ACQUIRE_OBJECT, object);
            let actor = Ptr::<()>::new(0x5000);
            let p = location(&mut e, location_type, 0, 0);
            assert_eq!(e.call(0x0067_f2a0, &args![p, actor]).u32(), 0x7000);
            assert_eq!(*acquire.borrow(), vec![vec![0x5000]]);
            // The slot returning 0 falls back to the actor.
            let none = object_with_slot(&mut e, 0);
            recorder(&mut e, SAVED_ACQUIRE_OBJECT, none);
            assert_eq!(e.call(0x0067_f2a0, &args![p, actor]).u32(), 0x5000);
        }
    }

    #[test]
    fn test_reference_covers_each_exit() {
        let (mut e, _, _) = resolution_engine();
        let out = Ptr::<()>::new(e.mem.alloc(4));
        let actor = Ptr::<()>::new(0x5000);
        let form_field = recorder(&mut e, GET_FORM_FIELD, 0x8000);
        let form_type = recorder(&mut e, FORM_TYPE_BYTE, 0x15);
        let find = recorder(&mut e, FIND_TEST_OBJECT, 0x9000);
        let result = recorder(&mut e, TEST_OBJECT_RESULT, 0x1234_5601);
        // Type 3 resolves to the actor.
        let p = location(&mut e, 3, 0, 0);
        e.mem.set_u8(out.addr(), 0x77);
        assert!(e.call(0x0067_f220, &args![p, actor, 0xabu32, out]).bool());
        assert_eq!(e.mem.u8(out.addr()), 1);
        assert_eq!(*form_field.borrow(), vec![vec![0x5000]]);
        assert_eq!(*form_type.borrow(), vec![vec![0x8000]]);
        assert_eq!(*find.borrow(), vec![vec![0x5000]]);
        assert_eq!(*result.borrow(), vec![vec![0x9000, 0xab, 0x5000]]);
        // No object found: success, the byte stays 0.
        recorder(&mut e, FIND_TEST_OBJECT, 0);
        e.mem.set_u8(out.addr(), 0x77);
        assert!(e.call(0x0067_f220, &args![p, actor, 0xabu32, out]).bool());
        assert_eq!(e.mem.u8(out.addr()), 0);
        // The form is not a reference: failure.
        recorder(&mut e, FORM_TYPE_BYTE, 0x14);
        e.mem.set_u8(out.addr(), 0x77);
        assert!(!e.call(0x0067_f220, &args![p, actor, 0xabu32, out]).bool());
        assert_eq!(e.mem.u8(out.addr()), 0);
        // No reference at all (type 1): failure.
        let cell = location(&mut e, 1, 0, 0x4000);
        assert!(!e
            .call(0x0067_f220, &args![cell, actor, 0xabu32, out])
            .bool());
    }

    #[test]
    fn typed_getters_return_the_word_only_for_their_type() {
        let mut e = Engine::new();
        for (location_type, getter, setter_type) in [
            (0i8, 0x0067_f390u32, 0),
            (1, 0x0067_f3e0, 1),
            (4, 0x0067_f430, 4),
            (5, 0x0067_f480, 5),
        ] {
            for other in [-1i8, 0, 1, 2, 3, 4, 5, 6] {
                let p = location(&mut e, other, 0, 0xabc);
                let expected = if other == location_type { 0xabc } else { 0 };
                assert_eq!(
                    e.call(getter, &args![p]).u32(),
                    expected,
                    "{setter_type} {other}"
                );
            }
        }
    }

    #[test]
    fn typed_setters_set_type_and_word() {
        let mut e = Engine::new();
        for (setter, expected_type) in [
            (0x0067_f3c0u32, 0i8),
            (0x0067_f410, 1),
            (0x0067_f460, 4),
            (0x0067_f4b0, 5),
        ] {
            let p = location(&mut e, 2, 17, 0);
            e.call(setter, &args![p, 0x1357u32]);
            assert_eq!(kind(&e, p), expected_type);
            assert_eq!(word(&e, p), 0x1357);
            assert_eq!(radius(&e, p), 17);
        }
    }

    #[test]
    fn copy_follows_the_source_type() {
        let mut e = Engine::new();
        // Null source and cleared source (without the flag): nothing happens.
        let this = location(&mut e, 4, 3, 0x77);
        e.call(0x0067_f4d0, &args![this, Ptr::<()>::NULL, 1u32]);
        assert_eq!(
            (kind(&e, this), radius(&e, this), word(&e, this)),
            (4, 3, 0x77)
        );
        let cleared = location(&mut e, -1, 0, 0);
        e.call(0x0067_f4d0, &args![this, cleared, 0u32]);
        assert_eq!(
            (kind(&e, this), radius(&e, this), word(&e, this)),
            (4, 3, 0x77)
        );
        // With the flag the cleared type is copied too.
        e.call(0x0067_f4d0, &args![this, cleared, 1u32]);
        assert_eq!(kind(&e, this), -1);
        // (source type, source radius, source word) -> (radius, word) copied.
        let cases = [
            (0i8, 40u32, 0x500u32, 40u32, 0x500u32),
            (1, 40, 0x500, 0, 0x500),
            (4, 40, 0x500, 40, 0x500),
            (5, 40, 0x500, 40, 0x500),
            (2, 40, 0x500, 40, 0),
            (3, 40, 0x500, 40, 0),
            (6, 40, 0x500, 40, 0),
        ];
        for (source_type, source_radius, source_word, want_radius, want_word) in cases {
            let this = location(&mut e, 9, 3, 0x77);
            let source = location(&mut e, source_type, source_radius, source_word);
            e.call(0x0067_f4d0, &args![this, source, 0u32]);
            assert_eq!(kind(&e, this), source_type);
            assert_eq!(radius(&e, this), want_radius, "type {source_type}");
            assert_eq!(word(&e, this), want_word, "type {source_type}");
        }
    }

    #[test]
    fn compare_reports_any_difference() {
        let mut e = Engine::new();
        let a = location(&mut e, 0, 5, 0x500);
        let same = location(&mut e, 0, 5, 0x500);
        assert!(!e.call(0x0067_f600, &args![a, same]).bool());
        assert!(e.call(0x0067_f600, &args![a, Ptr::<()>::NULL]).bool());
        let other_type = location(&mut e, 4, 5, 0x500);
        assert!(e.call(0x0067_f600, &args![a, other_type]).bool());
        let other_radius = location(&mut e, 0, 6, 0x500);
        assert!(e.call(0x0067_f600, &args![a, other_radius]).bool());
        let other_word = location(&mut e, 0, 5, 0x501);
        assert!(e.call(0x0067_f600, &args![a, other_word]).bool());
        // Cells compare by cell and ignore the radius.
        let cell_a = location(&mut e, 1, 5, 0x500);
        let cell_b = location(&mut e, 1, 9, 0x500);
        assert!(!e.call(0x0067_f600, &args![cell_a, cell_b]).bool());
        let cell_c = location(&mut e, 1, 5, 0x501);
        assert!(e.call(0x0067_f600, &args![cell_a, cell_c]).bool());
        // Object types compare by the type word.
        let kind_a = location(&mut e, 5, 5, 3);
        let kind_b = location(&mut e, 5, 5, 4);
        assert!(e.call(0x0067_f600, &args![kind_a, kind_b]).bool());
    }

    /// Doubles for `Save`: memset, the form ID getter (`+0x1000`), the swap
    /// flag, the record swap and the chunk write. Returns the swap count and
    /// the record bytes the chunk write saw.
    fn save_engine(swap: bool) -> (Engine, Rc<RefCell<u32>>, Written) {
        let mut e = Engine::new();
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(FORM_ID, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        e.register_double(SWAP_BYTES_FLAG, move |_, _| Ret {
            eax: swap as u32,
            ..Ret::default()
        });
        let swaps = Rc::new(RefCell::new(0u32));
        let counted = swaps.clone();
        e.register_double(SWAP_RECORD, move |e, a| {
            *counted.borrow_mut() += 1;
            for offset in [4, 8] {
                let value = e.mem.u32(a[0] + offset);
                e.mem.set_u32(a[0] + offset, value.swap_bytes());
            }
            Ret::default()
        });
        let written = Rc::new(RefCell::new(vec![]));
        let saw = written.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            assert_eq!(a[2], 0xc);
            *saw.borrow_mut() = (0..0xc).map(|i| e.mem.u8(a[1] + i)).collect();
            Ret::default()
        });
        (e, swaps, written)
    }

    #[test]
    fn save_writes_the_twelve_byte_record() {
        let (mut e, swaps, written) = save_engine(false);
        let owner = Ptr::<()>::new(0x3000);
        // A reference: the form ID of the stored form and the radius.
        let p = location(&mut e, 0, 25, 0x4000);
        e.call(0x0067_f6c0, &args![p, owner]);
        assert_eq!(
            *written.borrow(),
            [0, 0, 0, 0, 0x00, 0x50, 0, 0, 25, 0, 0, 0]
        );
        // A type with no stored form leaves +4 at 0.
        let empty = location(&mut e, 2, 3, 0);
        e.call(0x0067_f6c0, &args![empty, owner]);
        assert_eq!(*written.borrow(), [2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0]);
        // An object type: the type number itself.
        let kind = location(&mut e, 5, 8, 7);
        e.call(0x0067_f6c0, &args![kind, owner]);
        assert_eq!(*written.borrow(), [5, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0]);
        // A cell has no radius in the record.
        let cell = location(&mut e, 1, 8, 0x4000);
        e.call(0x0067_f6c0, &args![cell, owner]);
        assert_eq!(
            *written.borrow(),
            [1, 0, 0, 0, 0x00, 0x50, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(*swaps.borrow(), 0);
    }

    #[test]
    fn save_swaps_around_the_write_when_flagged() {
        let (mut e, swaps, written) = save_engine(true);
        let p = location(&mut e, 5, 0x0102_0304, 0x0a0b_0c0d);
        e.call(0x0067_f6c0, &args![p, Ptr::<()>::new(0x3000)]);
        assert_eq!(*swaps.borrow(), 2);
        assert_eq!(
            *written.borrow(),
            [5, 0, 0, 0, 0x0a, 0x0b, 0x0c, 0x0d, 0x01, 0x02, 0x03, 0x04]
        );
    }

    /// Doubles for `InitItem`. The compile-index double adds `0x01000000` to
    /// the ID; the lookup knows the IDs in `known`; the cast passes the
    /// pointer through.
    fn init_item_engine(known: &'static [u32]) -> (Engine, Calls, Ptr) {
        let mut e = Engine::new();
        recorder(&mut e, FORM_GET_FILE, 0x777);
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register_double(LOOKUP_FORM, move |_, a| Ret {
            eax: if known.contains(&a[0]) {
                a[0] + 0x10
            } else {
                0
            },
            ..Ret::default()
        });
        e.register(DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(FORM_ID, |_, _| Ret {
            eax: 0xf00d,
            ..Ret::default()
        });
        let log = recorder(&mut e, LOG_MESSAGE, 0);
        let owner = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(owner, vtable);
        e.mem.set_u32(vtable + FORM_NAME_SLOT, 0x00a0_0100);
        e.register(0x00a0_0100, |_, _| Ret {
            eax: 0x999,
            ..Ret::default()
        });
        (e, log, Ptr::new(owner))
    }

    #[test]
    fn init_item_resolves_the_stored_id_by_type() {
        let (mut e, log, owner) = init_item_engine(&[0x0100_0042]);
        e.register(FORM_HAS_NAME, |_, _| Ret::default());
        // Reference.
        let p = location(&mut e, 0, 4, 0x42);
        e.call(0x0067_f760, &args![p, owner]);
        assert_eq!(
            (kind(&e, p), radius(&e, p), word(&e, p)),
            (0, 4, 0x0100_0052)
        );
        // Cell.
        let cell = location(&mut e, 1, 0, 0x42);
        e.call(0x0067_f760, &args![cell, owner]);
        assert_eq!((kind(&e, cell), word(&e, cell)), (1, 0x0100_0052));
        // Object form.
        let object = location(&mut e, 4, 4, 0x42);
        e.call(0x0067_f760, &args![object, owner]);
        assert_eq!((kind(&e, object), word(&e, object)), (4, 0x0100_0052));
        assert!(log.borrow().is_empty());
        // Object types and the other types are left alone.
        for location_type in [5i8, 2, 3, 6, -1] {
            let p = location(&mut e, location_type, 4, 0x42);
            e.call(0x0067_f760, &args![p, owner]);
            assert_eq!((kind(&e, p), word(&e, p)), (location_type, 0x42));
        }
    }

    #[test]
    fn init_item_reports_a_missing_form_with_the_owner() {
        // Each type: (location type, format with name, format with ID).
        let cases = [
            (0i8, 0x0106_8d90u32, 0x0106_8d38u32),
            (1, 0x0106_8ee0, 0x0106_8e90),
            (4, 0x0106_8e40, 0x0106_8de8),
        ];
        for (location_type, named, by_id) in cases {
            let (mut e, log, owner) = init_item_engine(&[]);
            e.register(FORM_HAS_NAME, |_, _| Ret {
                eax: 1,
                ..Ret::default()
            });
            let p = location(&mut e, location_type, 4, 0x42);
            e.call(0x0067_f760, &args![p, owner]);
            assert_eq!(
                (kind(&e, p), radius(&e, p), word(&e, p)),
                (location_type, 4, 0)
            );
            assert_eq!(*log.borrow(), vec![vec![named, 0x0100_0042, 0x999]]);
            // Without a name the owner's form ID is printed.
            let (mut e, log, owner) = init_item_engine(&[]);
            e.register(FORM_HAS_NAME, |_, _| Ret::default());
            let p = location(&mut e, location_type, 4, 0x42);
            e.call(0x0067_f760, &args![p, owner]);
            assert_eq!(*log.borrow(), vec![vec![by_id, 0x0100_0042, 0xf00d]]);
        }
    }

    #[test]
    fn generic_means_type_above_three() {
        let mut e = Engine::new();
        for (location_type, expected) in
            [(-1i8, false), (0, false), (3, false), (4, true), (5, true)]
        {
            let p = location(&mut e, location_type, 0, 0);
            assert_eq!(
                e.call(0x0067_f970, &args![p]).bool(),
                expected,
                "{location_type}"
            );
        }
    }

    #[test]
    fn save_size_depends_on_the_type() {
        let mut e = Engine::new();
        for (location_type, expected) in
            [(-1i8, 5u32), (0, 9), (1, 9), (4, 9), (5, 9), (6, 5), (9, 5)]
        {
            let p = location(&mut e, location_type, 0, 0);
            assert_eq!(
                e.call(0x0067_f990, &args![p]).u32() & 0xffff,
                expected,
                "{location_type}"
            );
        }
    }

    /// The stream object pointer (`0x011de45c`) and doubles for its calls;
    /// returns the call log in order as (callee, words after the stream).
    fn stream_engine() -> (Engine, CallLog) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x0033_0000u32);
        e.register(FORM_ID, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        for callee in [
            STREAM_WRITE,
            STREAM_SAVE_NUMERIC_ID,
            STREAM_READ,
            STREAM_LOAD_NUMERIC_ID,
        ] {
            let seen = log.clone();
            e.register_double(callee, move |e, a| {
                seen.borrow_mut().push((callee, a.to_vec()));
                if callee == STREAM_LOAD_NUMERIC_ID {
                    e.mem.set_u32(a[1], 0x8123);
                }
                if callee == STREAM_SAVE_NUMERIC_ID {
                    // Remember the ID the caller put in the slot.
                    seen.borrow_mut().push((0, vec![e.mem.u32(a[1])]));
                }
                Ret::default()
            });
        }
        (e, log)
    }

    #[test]
    fn stream_save_writes_by_type() {
        let (mut e, log) = stream_engine();
        let stream = 0x0033_0000;
        let p = location(&mut e, 0, 5, 0x4000);
        let base = p.addr();
        e.call(0x0067_fa00, &args![p]);
        assert_eq!(
            *log.borrow(),
            vec![
                (STREAM_WRITE, vec![stream, base, 1]),
                (STREAM_WRITE, vec![stream, base + 4, 4]),
                (
                    STREAM_SAVE_NUMERIC_ID,
                    vec![stream, log.borrow()[2].1[1], 4]
                ),
                (0, vec![0x5000]),
            ]
        );
        // No stored form: ID 0.
        log.borrow_mut().clear();
        let empty = location(&mut e, 4, 5, 0);
        e.call(0x0067_fa00, &args![empty]);
        assert_eq!(log.borrow()[3], (0, vec![0]));
        // Object type: the word itself, written from the object.
        log.borrow_mut().clear();
        let kind = location(&mut e, 5, 5, 7);
        e.call(0x0067_fa00, &args![kind]);
        assert_eq!(
            log.borrow()[2],
            (STREAM_WRITE, vec![stream, kind.addr() + 8, 4])
        );
        // Other types: the type and radius only.
        log.borrow_mut().clear();
        let other = location(&mut e, 6, 5, 7);
        e.call(0x0067_fa00, &args![other]);
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn stream_load_reads_by_type() {
        let (mut e, log) = stream_engine();
        let stream = 0x0033_0000;
        let p = location(&mut e, 1, 0, 0);
        e.call(0x0067_faa0, &args![p]);
        assert_eq!(word(&e, p), 0x8123);
        assert_eq!(log.borrow()[0], (STREAM_READ, vec![stream, p.addr(), 1]));
        assert_eq!(
            log.borrow()[1],
            (STREAM_READ, vec![stream, p.addr() + 4, 4])
        );
        assert_eq!(log.borrow()[2].0, STREAM_LOAD_NUMERIC_ID);
        // Object type: read straight into the word.
        log.borrow_mut().clear();
        let kind = location(&mut e, 5, 0, 0);
        e.call(0x0067_faa0, &args![kind]);
        assert_eq!(
            log.borrow()[2],
            (STREAM_READ, vec![stream, kind.addr() + 8, 4])
        );
        assert_eq!(word(&e, kind), 0);
        // Other types: no third read.
        log.borrow_mut().clear();
        let other = location(&mut e, 6, 0, 0x55);
        e.call(0x0067_faa0, &args![other]);
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(word(&e, other), 0x55);
    }

    #[test]
    fn lookups_after_load_replace_ids_with_forms() {
        let mut e = Engine::new();
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        // 0067fb20 looks up even an ID of 0.
        let p = location(&mut e, 0, 0, 0x42);
        e.call(0x0067_fb20, &args![p]);
        assert_eq!(word(&e, p), 0x52);
        let zero = location(&mut e, 4, 0, 0);
        e.call(0x0067_fb20, &args![zero]);
        assert_eq!(word(&e, zero), 0x10);
        // 0067fc50 keeps a 0 as 0.
        let q = location(&mut e, 1, 0, 0x42);
        e.call(0x0067_fc50, &args![q]);
        assert_eq!(word(&e, q), 0x52);
        let zero = location(&mut e, 4, 0, 0);
        e.call(0x0067_fc50, &args![zero]);
        assert_eq!(word(&e, zero), 0);
        // Object types and others are untouched.
        for location_type in [5i8, 6, -1] {
            let r = location(&mut e, location_type, 0, 0x42);
            e.call(0x0067_fb20, &args![r]);
            e.call(0x0067_fc50, &args![r]);
            assert_eq!(word(&e, r), 0x42);
        }
    }

    #[test]
    fn save_game_buffer_writes_by_type() {
        let mut e = Engine::new();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        for callee in [SAVE_BUFFER_WRITE, SAVE_BUFFER_FORM_ID] {
            let seen = log.clone();
            e.register_double(callee, move |_, a| {
                seen.borrow_mut().push((callee, a.to_vec()));
                Ret::default()
            });
        }
        let buffer = Ptr::<()>::new(0x3000);
        let p = location(&mut e, 4, 5, 0x4000);
        e.call(0x0067_fb60, &args![p, buffer]);
        assert_eq!(
            *log.borrow(),
            vec![
                (SAVE_BUFFER_WRITE, vec![0x3000, p.addr(), 1, 0]),
                (SAVE_BUFFER_WRITE, vec![0x3000, p.addr() + 4, 4, 0]),
                (SAVE_BUFFER_FORM_ID, vec![0x3000, 0x4000, 0]),
            ]
        );
        log.borrow_mut().clear();
        let kind = location(&mut e, 5, 5, 7);
        e.call(0x0067_fb60, &args![kind, buffer]);
        assert_eq!(
            log.borrow()[2],
            (SAVE_BUFFER_WRITE, vec![0x3000, kind.addr() + 8, 4, 0])
        );
        log.borrow_mut().clear();
        let other = location(&mut e, 6, 5, 7);
        e.call(0x0067_fb60, &args![other, buffer]);
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn load_game_buffer_reads_by_type() {
        let mut e = Engine::new();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        for callee in [LOAD_BUFFER_READ, LOAD_BUFFER_FORM_ID] {
            let seen = log.clone();
            e.register_double(callee, move |_, a| {
                seen.borrow_mut().push((callee, a.to_vec()));
                Ret::default()
            });
        }
        let buffer = Ptr::<()>::new(0x3000);
        let p = location(&mut e, 0, 5, 0);
        e.call(0x0067_fbe0, &args![p, buffer]);
        assert_eq!(
            *log.borrow(),
            vec![
                (LOAD_BUFFER_READ, vec![0x3000, p.addr(), 1]),
                (LOAD_BUFFER_READ, vec![0x3000, p.addr() + 4, 4]),
                (LOAD_BUFFER_FORM_ID, vec![0x3000, p.addr() + 8]),
            ]
        );
        log.borrow_mut().clear();
        let kind = location(&mut e, 5, 5, 0);
        e.call(0x0067_fbe0, &args![kind, buffer]);
        assert_eq!(
            log.borrow()[2],
            (LOAD_BUFFER_READ, vec![0x3000, kind.addr() + 8, 4])
        );
        log.borrow_mut().clear();
        let other = location(&mut e, 6, 5, 0);
        e.call(0x0067_fbe0, &args![other, buffer]);
        assert_eq!(log.borrow().len(), 2);
    }
}

//! `fallout shared/packagetarget.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `PackageTarget` (0x10 bytes) says what an AI package acts on: a type byte
//! at +0 (0 = a reference, 1 = a base object, 2 = an object type; the Xbox PDB
//! calls the field `eTargType`), a word at +4 whose meaning follows the type
//! (`pReference`, `pObject` or `eObjType`), an integer value at +8 and the
//! acquire radius (a `float`) at +0xC. Types above 2 keep the +4 word
//! untouched everywhere.
//!
//! Nothing is left out: none of the functions of this unit has an
//! exception-unwinding frame.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::GetFile` (Xbox PDB; `this` = form, argument -1 = the first
/// file that has the form).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB; cdecl: pointer to the form id, file).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Looks a form up by id (cdecl, one argument): the form or null.
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (cdecl: pointer, vfptr delta, source type descriptor,
/// target type descriptor, is-reference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The two type descriptors `InitItem` passes to `__RTDynamicCast` for a
/// reference lookup (source, target). Which classes they describe is not
/// confirmed here.
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_41cc;
/// Form id getter (`this` = form; returns `iFormID`, +0x0C).
const FORM_GET_FORM_ID: u32 = 0x0084_e3a0;
/// Getter of the word at +8 of the object it is called on.
const GET_VALUE: u32 = 0x0044_ddc0;
/// The matching setter (one argument).
const SET_VALUE: u32 = 0x0040_3550;
/// Getter of the `float` at +0xC (result in ST0).
const GET_ACQUIRE_RADIUS: u32 = 0x0084_d030;
/// The matching setter (a `float` argument).
const SET_ACQUIRE_RADIUS: u32 = 0x0063_f790;
/// `memset(destination, value, count)`.
const MEMSET: u32 = 0x0040_3d30;
/// `TESForm::__AddChunkData` (Xbox PDB), cdecl: (record, data, size).
const TES_FORM_ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// Reads the byte-swap flag (byte at `0x011c54ba`; the Xbox build is
/// big-endian).
const NEEDS_BYTE_SWAP: u32 = 0x0040_1500;
/// Byte-swaps the three words at +4, +8 and +0xC of the block it is called on.
const BYTE_SWAP_FIELDS: u32 = 0x0067_ff10;
/// Debug printing (cdecl, format string then arguments).
const PRINT_LINE: u32 = 0x005b_5e40;
/// Whether an owner has a name to print (`this` = owner): it calls the
/// owner's virtual name getter under a re-entrancy flag and returns a
/// non-zero value when the name is non-null.
const OWNER_HAS_NAME: u32 = 0x0047_4cb0;
/// Slot of the owner's virtual name getter (returns the name string).
const OWNER_NAME_SLOT: u32 = 0x130;
/// The global pointer to the save-game helper object that the writers
/// (`008579b0`, `00857a10`) and readers (`008579e0`, `00857aa0`) are called on.
const SAVE_LOAD_OBJECT: u32 = 0x011d_e45c;
/// Raw byte writer/reader on that object (`this`; data pointer, size).
const SAVE_BYTES: u32 = 0x0085_79b0;
const LOAD_BYTES: u32 = 0x0085_79e0;
/// `TESSaveLoadGame::SaveNumericID` (Xbox PDB; `this`, id pointer, size) and
/// `TESSaveLoadGame::LoadNumericID` (Xbox PDB; `this`, out pointer, size).
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
/// Raw writer of the save-game buffer (`this` = buffer; data, size, 0).
const SAVE_GAME_WRITE: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (Xbox PDB; `this` = buffer; form, 0).
const SAVE_GAME_FORM_ID: u32 = 0x0086_5df0;
/// Raw reader of the load-game buffer (`this` = buffer; destination, size).
const LOAD_GAME_READ: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (Xbox PDB; `this` = buffer; out pointer).
const LOAD_GAME_FORM_ID: u32 = 0x0086_48e0;

/// Messages of `InitItem` when a form is not found, as (owner has a name,
/// owner without a name, no owner): the first two take the form id and the
/// owner's name or form id, the last only the form id. For a reference
/// (type 0) and for a base object (type 1).
const MESSAGES_REFERENCE: [u32; 3] = [0x0106_9198, 0x0106_9140, 0x0106_90f0];
const MESSAGES_OBJECT: [u32; 3] = [0x0106_90a0, 0x0106_9050, 0x0106_9000];

layout! {
    /// `PackageTarget` (Xbox PDB), 0x10 bytes on both builds.
    pub struct PackageTarget: 0x10 {
        /// `eTargType` (Xbox PDB): 0 reference, 1 object, 2 object type.
        0x00 eTargType: u8,
        /// `pReference` / `pObject` / `eObjType` (Xbox PDB), one word.
        0x04 pRefOrObj: u32,
        /// `iValue` (Xbox PDB).
        0x08 iValue: u32,
        /// `fAcquireRadius` (Xbox PDB).
        0x0C fAcquireRadius: f32,
    }
}

// Translated from 00519b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::GetTargType` (Xbox PDB): the type byte.
pub fn package_target_get_targ_type(e: &mut Engine, this: Ptr<PackageTarget>) -> u8 {
    e.get(this, PackageTarget::eTargType)
}

// Translated from 0067ff70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::PackageTarget` (Xbox PDB): type 2 (object type), the
/// word, the value and the radius cleared. Returns `this`.
pub fn package_target_package_target(
    e: &mut Engine,
    this: Ptr<PackageTarget>,
) -> Ptr<PackageTarget> {
    e.set(this, PackageTarget::eTargType, 2);
    e.set(this, PackageTarget::pRefOrObj, 0);
    e.set(this, PackageTarget::iValue, 0);
    e.set(this, PackageTarget::fAcquireRadius, 0.0);
    this
}

// Translated from 0067ffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assignment (`PackageTarget::operator=`; unnamed in the map): takes the
/// other target's type (which clears the +4 word when it changes), value
/// and radius through the setters, then copies the +4 word when the
/// resulting type is 0, 1 or 2. Returns `this`.
pub fn fn_0067ffa0(
    e: &mut Engine,
    this: Ptr<PackageTarget>,
    other: Ptr<PackageTarget>,
) -> Ptr<PackageTarget> {
    let other_type = e.get(other, PackageTarget::eTargType);
    package_target_set_targ_type(e, this, other_type as u32);
    let value = e.get(other, PackageTarget::iValue);
    e.call(SET_VALUE, &args![this, value]);
    let radius = e.get(other, PackageTarget::fAcquireRadius);
    e.call(SET_ACQUIRE_RADIUS, &args![this, radius]);
    if e.get(this, PackageTarget::eTargType) <= 2 {
        let word = e.get(other, PackageTarget::pRefOrObj);
        e.set(this, PackageTarget::pRefOrObj, word);
    }
    this
}

// Translated from 00680020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::GetTargReference` (Xbox PDB): the +4 word when the type
/// is 0, else 0.
pub fn package_target_get_targ_reference(e: &mut Engine, this: Ptr<PackageTarget>) -> u32 {
    if package_target_get_targ_type(e, this) == 0 {
        e.get(this, PackageTarget::pRefOrObj)
    } else {
        0
    }
}

// Translated from 00680050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::GetTargObject` (Xbox PDB): the +4 word when the type is
/// 1, else 0.
pub fn package_target_get_targ_object(e: &mut Engine, this: Ptr<PackageTarget>) -> u32 {
    if package_target_get_targ_type(e, this) == 1 {
        e.get(this, PackageTarget::pRefOrObj)
    } else {
        0
    }
}

// Translated from 00680080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::GetTargObjectType` (Xbox PDB): the +4 word when the type
/// is 2, else 0.
pub fn package_target_get_targ_object_type(e: &mut Engine, this: Ptr<PackageTarget>) -> u32 {
    if package_target_get_targ_type(e, this) == 2 {
        e.get(this, PackageTarget::pRefOrObj)
    } else {
        0
    }
}

// Translated from 006800b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::SetTargType` (Xbox PDB): when the type (compared as a
/// full word) differs, stores its low byte and, for types 0, 1 and 2,
/// clears the +4 word.
pub fn package_target_set_targ_type(e: &mut Engine, this: Ptr<PackageTarget>, targ_type: u32) {
    if e.get(this, PackageTarget::eTargType) as u32 != targ_type {
        e.set(this, PackageTarget::eTargType, targ_type as u8);
        if e.get(this, PackageTarget::eTargType) <= 2 {
            e.set(this, PackageTarget::pRefOrObj, 0);
        }
    }
}

// Translated from 00680110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::SetTargReference` (Xbox PDB): stores the +4 word when the
/// type is 0.
pub fn package_target_set_targ_reference(e: &mut Engine, this: Ptr<PackageTarget>, reference: u32) {
    if package_target_get_targ_type(e, this) == 0 {
        e.set(this, PackageTarget::pRefOrObj, reference);
    }
}

// Translated from 00680140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the +4 word when the type is 1 (the object setter; unnamed in the
/// map).
pub fn fn_00680140(e: &mut Engine, this: Ptr<PackageTarget>, object: u32) {
    if package_target_get_targ_type(e, this) == 1 {
        e.set(this, PackageTarget::pRefOrObj, object);
    }
}

// Translated from 00680170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the +4 word when the type is 2 (the object type setter; unnamed in
/// the map).
pub fn fn_00680170(e: &mut Engine, this: Ptr<PackageTarget>, object_type: u32) {
    if package_target_get_targ_type(e, this) == 2 {
        e.set(this, PackageTarget::pRefOrObj, object_type);
    }
}

// Translated from 006801a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::Copy` (Xbox PDB): nothing for a null source; otherwise
/// copies the type, the reference, the object, the object type, the value
/// (through the +8 getter and setter) and the radius (through the +0xC
/// getter and setter).
pub fn package_target_copy(e: &mut Engine, this: Ptr<PackageTarget>, source: Ptr<PackageTarget>) {
    if source.is_null() {
        return;
    }
    let targ_type = package_target_get_targ_type(e, source);
    package_target_set_targ_type(e, this, targ_type as u32);
    let reference = package_target_get_targ_reference(e, source);
    package_target_set_targ_reference(e, this, reference);
    let object = package_target_get_targ_object(e, source);
    fn_00680140(e, this, object);
    let object_type = package_target_get_targ_object_type(e, source);
    fn_00680170(e, this, object_type);
    let value = e.call(GET_VALUE, &args![source]).u32();
    e.call(SET_VALUE, &args![this, value]);
    let radius = e.call(GET_ACQUIRE_RADIUS, &args![source]).f32();
    e.call(SET_ACQUIRE_RADIUS, &args![this, radius]);
}

// Translated from 00680220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::Compare` (Xbox PDB): 0 when `other` is non-null and
/// equal in type, reference, object, object type, value and radius, else 1
/// (a null `other` and a NaN radius count as different).
pub fn package_target_compare(
    e: &mut Engine,
    this: Ptr<PackageTarget>,
    other: Ptr<PackageTarget>,
) -> u8 {
    if other.is_null() {
        return 1;
    }
    if package_target_get_targ_type(e, this) != package_target_get_targ_type(e, other) {
        return 1;
    }
    if package_target_get_targ_reference(e, this) != package_target_get_targ_reference(e, other) {
        return 1;
    }
    if package_target_get_targ_object(e, this) != package_target_get_targ_object(e, other) {
        return 1;
    }
    if package_target_get_targ_object_type(e, this) != package_target_get_targ_object_type(e, other)
    {
        return 1;
    }
    let own_value = e.call(GET_VALUE, &args![this]).u32();
    let other_value = e.call(GET_VALUE, &args![other]).u32();
    if own_value != other_value {
        return 1;
    }
    let own_radius = e.call(GET_ACQUIRE_RADIUS, &args![this]).f32();
    let other_radius = e.call(GET_ACQUIRE_RADIUS, &args![other]).f32();
    if own_radius as f64 == other_radius as f64 {
        0
    } else {
        1
    }
}

// Translated from 006802f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::Save` (Xbox PDB): writes the target as a 0x10-byte chunk
/// of the record `record`. The chunk holds the type byte, the value and the
/// radius; its +4 word is the form id of the referenced form (types 0 and
/// 1, when set) or the object type (type 2). When the byte-swap flag is set
/// (the Xbox build) the words are swapped before and after the write.
pub fn package_target_save(e: &mut Engine, this: Ptr<PackageTarget>, record: u32) {
    e.with_stack(0x10, |e, chunk| {
        e.call(MEMSET, &args![chunk, 0u32, 0x10u32]);
        let value = e.get(this, PackageTarget::iValue);
        e.mem.set_u32(chunk.addr() + 8, value);
        let radius = e.get(this, PackageTarget::fAcquireRadius);
        e.mem.set_f32(chunk.addr() + 0xC, radius);
        let targ_type = e.get(this, PackageTarget::eTargType);
        e.mem.set_u8(chunk.addr(), targ_type);
        if targ_type < 2 {
            let form = e.get(this, PackageTarget::pRefOrObj);
            if form != 0 {
                let form_id = e.call(FORM_GET_FORM_ID, &args![form]).u32();
                e.mem.set_u32(chunk.addr() + 4, form_id);
            }
        } else if targ_type == 2 {
            let object_type = package_target_get_targ_object_type(e, this);
            e.mem.set_u32(chunk.addr() + 4, object_type);
        }
        if e.call(NEEDS_BYTE_SWAP, &args![]).u8() != 0 {
            e.call(BYTE_SWAP_FIELDS, &args![chunk]);
        }
        e.call(TES_FORM_ADD_CHUNK_DATA, &args![record, chunk, 0x10u32]);
        if e.call(NEEDS_BYTE_SWAP, &args![]).u8() != 0 {
            e.call(BYTE_SWAP_FIELDS, &args![chunk]);
        }
    });
}

/// Prints the "unable to find" message of `InitItem` (`messages` as
/// `MESSAGES_REFERENCE`): with no owner the form id only; with an owner that
/// has a name the form id and the name (virtual getter); with an owner
/// without a name the form id and the owner's form id.
fn report_missing(e: &mut Engine, owner: u32, form_id: u32, messages: [u32; 3]) {
    if owner == 0 {
        e.call(PRINT_LINE, &args![messages[2], form_id]);
    } else if e.call(OWNER_HAS_NAME, &args![owner]).u32() != 0 {
        let name = e.vcall(owner, OWNER_NAME_SLOT, &args![]).u32();
        e.call(PRINT_LINE, &args![messages[0], form_id, name]);
    } else {
        let owner_id = e.call(FORM_GET_FORM_ID, &args![owner]).u32();
        e.call(PRINT_LINE, &args![messages[1], form_id, owner_id]);
    }
}

// Translated from 006803b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::InitItem` (Xbox PDB): resolves the stored form id of a
/// loaded plugin record. The id is first fixed up with the compile index of
/// `owner`'s file (`TESForm::AddCompileIndex`); then a type 0 target looks
/// the form up and casts it (`__RTDynamicCast`), a type 1 target takes the
/// form as is. A form that is not found stores null and prints a
/// "MASTERFILE: Unable to find Package Target ..." message that names the
/// owner when it can. Other types do nothing more.
pub fn package_target_init_item(e: &mut Engine, this: Ptr<PackageTarget>, owner: u32) {
    let targ_type = e.get(this, PackageTarget::eTargType);
    let stored_id = e.get(this, PackageTarget::pRefOrObj);
    let file = e
        .call(TES_FORM_GET_FILE, &args![owner, 0xffff_ffffu32])
        .u32();
    let form_id = e.with_stack(4, |e, id| {
        e.mem.set_u32(id.addr(), stored_id);
        e.call(TES_FORM_ADD_COMPILE_INDEX, &args![id, file]);
        e.mem.u32(id.addr())
    });
    if targ_type == 0 {
        let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
        let reference = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
            )
            .u32();
        e.set(this, PackageTarget::pRefOrObj, reference);
        if reference == 0 {
            report_missing(e, owner, form_id, MESSAGES_REFERENCE);
        }
    } else if targ_type == 1 {
        let object = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
        e.set(this, PackageTarget::pRefOrObj, object);
        if object == 0 {
            report_missing(e, owner, form_id, MESSAGES_OBJECT);
        }
    }
}

// Translated from 00680520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Size in bytes of the target's save data (unnamed in the map): 13 for
/// types 0, 1 and 2, else 9 (that is 1 + 4 + 4, plus 4 for the +4 word).
pub fn fn_00680520(e: &mut Engine, this: Ptr<PackageTarget>) -> u16 {
    if e.get(this, PackageTarget::eTargType) <= 2 {
        13
    } else {
        9
    }
}

/// The object the helpers `008579b0` / `008579e0` / `00857a10` /
/// `00857aa0` are called on (the pointer at `0x011de45c`, read at every use
/// as the game does).
fn save_load_object(e: &Engine) -> u32 {
    e.global::<u32>(SAVE_LOAD_OBJECT)
}

// Translated from 00680590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the target to the save-game helper object (unnamed in the map):
/// the type byte, the value, the radius, then for types 0 and 1 the form id
/// of the +4 form (0 when null) as a numeric id, and for type 2 the raw
/// word.
pub fn fn_00680590(e: &mut Engine, this: Ptr<PackageTarget>) {
    let target = this.addr();
    let object = save_load_object(e);
    e.call(SAVE_BYTES, &args![object, target, 1u32]);
    let object = save_load_object(e);
    e.call(SAVE_BYTES, &args![object, target + 8, 4u32]);
    let object = save_load_object(e);
    e.call(SAVE_BYTES, &args![object, target + 0xC, 4u32]);
    let targ_type = e.get(this, PackageTarget::eTargType);
    if targ_type < 2 {
        let mut form_id = 0u32;
        let form = e.get(this, PackageTarget::pRefOrObj);
        if form != 0 {
            form_id = e.call(FORM_GET_FORM_ID, &args![form]).u32();
        }
        e.with_stack(4, |e, id| {
            e.mem.set_u32(id.addr(), form_id);
            let object = save_load_object(e);
            e.call(SAVE_NUMERIC_ID, &args![object, id, 4u32]);
        });
    } else if targ_type == 2 {
        let object = save_load_object(e);
        e.call(SAVE_BYTES, &args![object, target + 4, 4u32]);
    }
}

// Translated from 00680640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the target back from the save-game helper object (unnamed in the
/// map; the counterpart of `00680590`): the type byte, the value, the
/// radius, then for types 0 and 1 a numeric id stored into the +4 word, and
/// for type 2 the raw word.
pub fn fn_00680640(e: &mut Engine, this: Ptr<PackageTarget>) {
    let target = this.addr();
    let object = save_load_object(e);
    e.call(LOAD_BYTES, &args![object, target, 1u32]);
    let object = save_load_object(e);
    e.call(LOAD_BYTES, &args![object, target + 8, 4u32]);
    let object = save_load_object(e);
    e.call(LOAD_BYTES, &args![object, target + 0xC, 4u32]);
    let targ_type = e.get(this, PackageTarget::eTargType);
    if targ_type < 2 {
        let id = e.with_stack(4, |e, id| {
            let object = save_load_object(e);
            e.call(LOAD_NUMERIC_ID, &args![object, id, 4u32]);
            e.mem.u32(id.addr())
        });
        e.set(this, PackageTarget::pRefOrObj, id);
    } else if targ_type == 2 {
        let object = save_load_object(e);
        e.call(LOAD_BYTES, &args![object, target + 4, 4u32]);
    }
}

// Translated from 006806e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For types 0 and 1, replaces the +4 word (a form id) by the form it
/// names, looked up by id (null when none; unnamed in the map).
pub fn fn_006806e0(e: &mut Engine, this: Ptr<PackageTarget>) {
    if e.get(this, PackageTarget::eTargType) < 2 {
        let form_id = e.get(this, PackageTarget::pRefOrObj);
        let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
        e.set(this, PackageTarget::pRefOrObj, form);
    }
}

// Translated from 00680720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::SaveGame` (Xbox PDB): writes the type byte, the value
/// and the radius into the save-game buffer `buffer`, then the form id of
/// the +4 form (types 0 and 1) or the raw word (type 2).
pub fn package_target_save_game(e: &mut Engine, this: Ptr<PackageTarget>, buffer: u32) {
    let target = this.addr();
    e.call(SAVE_GAME_WRITE, &args![buffer, target, 1u32, 0u32]);
    e.call(SAVE_GAME_WRITE, &args![buffer, target + 8, 4u32, 0u32]);
    e.call(SAVE_GAME_WRITE, &args![buffer, target + 0xC, 4u32, 0u32]);
    let targ_type = e.get(this, PackageTarget::eTargType);
    if targ_type < 2 {
        let form = e.get(this, PackageTarget::pRefOrObj);
        e.call(SAVE_GAME_FORM_ID, &args![buffer, form, 0u32]);
    } else if targ_type == 2 {
        e.call(SAVE_GAME_WRITE, &args![buffer, target + 4, 4u32, 0u32]);
    }
}

// Translated from 006807b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::LoadGame` (Xbox PDB): reads the type byte, the value and
/// the radius from the load-game buffer `buffer`, then the form id (types 0
/// and 1, through `LoadFormID`, into the +4 word) or the raw word (type 2).
pub fn package_target_load_game(e: &mut Engine, this: Ptr<PackageTarget>, buffer: u32) {
    let target = this.addr();
    e.call(LOAD_GAME_READ, &args![buffer, target, 1u32]);
    e.call(LOAD_GAME_READ, &args![buffer, target + 8, 4u32]);
    e.call(LOAD_GAME_READ, &args![buffer, target + 0xC, 4u32]);
    let targ_type = e.get(this, PackageTarget::eTargType);
    if targ_type < 2 {
        e.call(LOAD_GAME_FORM_ID, &args![buffer, target + 4]);
    } else if targ_type == 2 {
        e.call(LOAD_GAME_READ, &args![buffer, target + 4, 4u32]);
    }
}

// Translated from 00680830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PackageTarget::InitLoadGame` (Xbox PDB): for types 0 and 1, resolves the
/// loaded +4 form id to the form (0 stays 0). The one stack word is never
/// read.
pub fn package_target_init_load_game(e: &mut Engine, this: Ptr<PackageTarget>, _unused_1: u32) {
    if e.get(this, PackageTarget::eTargType) < 2 {
        let form_id = e.get(this, PackageTarget::pRefOrObj);
        let form = if form_id != 0 {
            e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32()
        } else {
            0
        };
        e.set(this, PackageTarget::pRefOrObj, form);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00519b00,
            package_target_get_targ_type(Ptr<PackageTarget>) -> u8
        ),
        entry!(
            0x0067ff70,
            package_target_package_target(Ptr<PackageTarget>) -> Ptr<PackageTarget>
        ),
        entry!(
            0x0067ffa0,
            fn_0067ffa0(Ptr<PackageTarget>, Ptr<PackageTarget>) -> Ptr<PackageTarget>
        ),
        entry!(
            0x00680020,
            package_target_get_targ_reference(Ptr<PackageTarget>) -> u32
        ),
        entry!(
            0x00680050,
            package_target_get_targ_object(Ptr<PackageTarget>) -> u32
        ),
        entry!(
            0x00680080,
            package_target_get_targ_object_type(Ptr<PackageTarget>) -> u32
        ),
        entry!(
            0x006800b0,
            package_target_set_targ_type(Ptr<PackageTarget>, u32)
        ),
        entry!(
            0x00680110,
            package_target_set_targ_reference(Ptr<PackageTarget>, u32)
        ),
        entry!(0x00680140, fn_00680140(Ptr<PackageTarget>, u32)),
        entry!(0x00680170, fn_00680170(Ptr<PackageTarget>, u32)),
        entry!(
            0x006801a0,
            package_target_copy(Ptr<PackageTarget>, Ptr<PackageTarget>)
        ),
        entry!(
            0x00680220,
            package_target_compare(Ptr<PackageTarget>, Ptr<PackageTarget>) -> u8
        ),
        entry!(0x006802f0, package_target_save(Ptr<PackageTarget>, u32)),
        entry!(
            0x006803b0,
            package_target_init_item(Ptr<PackageTarget>, u32)
        ),
        entry!(0x00680520, fn_00680520(Ptr<PackageTarget>) -> u16),
        entry!(0x00680590, fn_00680590(Ptr<PackageTarget>)),
        entry!(0x00680640, fn_00680640(Ptr<PackageTarget>)),
        entry!(0x006806e0, fn_006806e0(Ptr<PackageTarget>)),
        entry!(
            0x00680720,
            package_target_save_game(Ptr<PackageTarget>, u32)
        ),
        entry!(
            0x006807b0,
            package_target_load_game(Ptr<PackageTarget>, u32)
        ),
        entry!(
            0x00680830,
            package_target_init_load_game(Ptr<PackageTarget>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with doubles for the value and radius accessors of the
    /// +8 and +0xC words.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(GET_VALUE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(SET_VALUE, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(GET_ACQUIRE_RADIUS, |e, a| e.mem.f32(a[0] + 0xC).into_ret());
        e.register(SET_ACQUIRE_RADIUS, |e, a| {
            e.mem.set_f32(a[0] + 0xC, f32::from_bits(a[1]));
            Ret::default()
        });
        e.register(FORM_GET_FORM_ID, |e, a| e.mem.u32(a[0] + 0xC).into_ret());
        e
    }

    fn target(
        e: &mut Engine,
        targ_type: u8,
        word: u32,
        value: u32,
        radius: f32,
    ) -> Ptr<PackageTarget> {
        let t: Ptr<PackageTarget> = e.new_object();
        e.set(t, PackageTarget::eTargType, targ_type);
        e.set(t, PackageTarget::pRefOrObj, word);
        e.set(t, PackageTarget::iValue, value);
        e.set(t, PackageTarget::fAcquireRadius, radius);
        t
    }

    fn word(e: &Engine, t: Ptr<PackageTarget>) -> u32 {
        e.get(t, PackageTarget::pRefOrObj)
    }

    /// A form whose id (+0xC) is `id`.
    fn form(e: &mut Engine, id: u32) -> u32 {
        let f = e.mem.alloc(0x20);
        e.mem.set_u32(f + 0xC, id);
        f
    }

    #[test]
    fn get_targ_type_returns_the_byte() {
        let mut e = engine();
        let t = target(&mut e, 1, 0x1234, 0, 0.0);
        assert_eq!(e.call(0x00519b00, &args![t]).u8(), 1);
    }

    #[test]
    fn constructor_sets_object_type_and_clears_the_rest() {
        let mut e = engine();
        let t = target(&mut e, 0, 5, 6, 7.0);
        assert_eq!(e.call(0x0067ff70, &args![t]).ptr::<PackageTarget>(), t);
        assert_eq!(e.get(t, PackageTarget::eTargType), 2);
        assert_eq!(word(&e, t), 0);
        assert_eq!(e.get(t, PackageTarget::iValue), 0);
        assert_eq!(e.get(t, PackageTarget::fAcquireRadius), 0.0);
    }

    #[test]
    fn assignment_copies_type_value_radius_and_word() {
        let mut e = engine();
        let other = target(&mut e, 1, 0x4444, 9, 2.5);
        let this = target(&mut e, 0, 0x1111, 0, 0.0);
        let back = e
            .call(0x0067ffa0, &args![this, other])
            .ptr::<PackageTarget>();
        assert_eq!(back, this);
        assert_eq!(e.get(this, PackageTarget::eTargType), 1);
        assert_eq!(word(&e, this), 0x4444);
        assert_eq!(e.get(this, PackageTarget::iValue), 9);
        assert_eq!(e.get(this, PackageTarget::fAcquireRadius), 2.5);
    }

    #[test]
    fn assignment_of_an_unknown_type_leaves_the_word() {
        let mut e = engine();
        let other = target(&mut e, 7, 0x4444, 0, 0.0);
        let this = target(&mut e, 2, 0x1111, 0, 0.0);
        e.call(0x0067ffa0, &args![this, other]);
        assert_eq!(e.get(this, PackageTarget::eTargType), 7);
        assert_eq!(word(&e, this), 0x1111);
    }

    #[test]
    fn getters_answer_only_for_their_type() {
        let mut e = engine();
        for (targ_type, expected) in [
            (0u8, [8u32, 0, 0]),
            (1, [0, 8, 0]),
            (2, [0, 0, 8]),
            (3, [0, 0, 0]),
        ] {
            let t = target(&mut e, targ_type, 8, 0, 0.0);
            assert_eq!(e.call(0x00680020, &args![t]).u32(), expected[0]);
            assert_eq!(e.call(0x00680050, &args![t]).u32(), expected[1]);
            assert_eq!(e.call(0x00680080, &args![t]).u32(), expected[2]);
        }
    }

    #[test]
    fn set_targ_type_clears_the_word_only_on_change() {
        let mut e = engine();
        let t = target(&mut e, 0, 0x55, 0, 0.0);
        e.call(0x006800b0, &args![t, 0u32]);
        assert_eq!(word(&e, t), 0x55);
        e.call(0x006800b0, &args![t, 1u32]);
        assert_eq!(e.get(t, PackageTarget::eTargType), 1);
        assert_eq!(word(&e, t), 0);
        // A type above 2 is stored but the word is kept.
        e.set(t, PackageTarget::pRefOrObj, 0x66);
        e.call(0x006800b0, &args![t, 9u32]);
        assert_eq!(e.get(t, PackageTarget::eTargType), 9);
        assert_eq!(word(&e, t), 0x66);
        // The comparison is on the whole word: 0x109 differs from 9.
        e.call(0x006800b0, &args![t, 0x109u32]);
        assert_eq!(e.get(t, PackageTarget::eTargType), 9);
        assert_eq!(word(&e, t), 0x66);
        e.call(0x006800b0, &args![t, 0x100u32]);
        assert_eq!(e.get(t, PackageTarget::eTargType), 0);
        assert_eq!(word(&e, t), 0);
    }

    #[test]
    fn setters_store_only_for_their_type() {
        let mut e = engine();
        for (setter, wanted) in [(0x00680110u32, 0u8), (0x00680140, 1), (0x00680170, 2)] {
            for targ_type in 0..4u8 {
                let t = target(&mut e, targ_type, 1, 0, 0.0);
                e.call(setter, &args![t, 0x99u32]);
                let expected = if targ_type == wanted { 0x99 } else { 1 };
                assert_eq!(
                    word(&e, t),
                    expected,
                    "setter {setter:08x} type {targ_type}"
                );
            }
        }
    }

    #[test]
    fn copy_of_null_does_nothing() {
        let mut e = engine();
        let this = target(&mut e, 1, 3, 4, 5.0);
        e.call(0x006801a0, &args![this, Ptr::<PackageTarget>::NULL]);
        assert_eq!(word(&e, this), 3);
        assert_eq!(e.get(this, PackageTarget::iValue), 4);
    }

    #[test]
    fn copy_takes_everything_of_the_source() {
        let mut e = engine();
        let source = target(&mut e, 2, 0x77, 12, 1.5);
        let this = target(&mut e, 0, 0x11, 0, 0.0);
        e.call(0x006801a0, &args![this, source]);
        assert_eq!(e.get(this, PackageTarget::eTargType), 2);
        assert_eq!(word(&e, this), 0x77);
        assert_eq!(e.get(this, PackageTarget::iValue), 12);
        assert_eq!(e.get(this, PackageTarget::fAcquireRadius), 1.5);
    }

    #[test]
    fn compare_is_zero_only_when_everything_matches() {
        let mut e = engine();
        let a = target(&mut e, 1, 0x40, 3, 2.0);
        let b = target(&mut e, 1, 0x40, 3, 2.0);
        assert_eq!(e.call(0x00680220, &args![a, b]).u8(), 0);
        assert_eq!(
            e.call(0x00680220, &args![a, Ptr::<PackageTarget>::NULL])
                .u8(),
            1
        );
        // Each field in turn.
        let different_type = target(&mut e, 2, 0x40, 3, 2.0);
        assert_eq!(e.call(0x00680220, &args![a, different_type]).u8(), 1);
        let different_word = target(&mut e, 1, 0x41, 3, 2.0);
        assert_eq!(e.call(0x00680220, &args![a, different_word]).u8(), 1);
        let different_value = target(&mut e, 1, 0x40, 4, 2.0);
        assert_eq!(e.call(0x00680220, &args![a, different_value]).u8(), 1);
        let different_radius = target(&mut e, 1, 0x40, 3, 2.5);
        assert_eq!(e.call(0x00680220, &args![a, different_radius]).u8(), 1);
        let not_a_number = target(&mut e, 1, 0x40, 3, f32::NAN);
        assert_eq!(
            e.call(0x00680220, &args![not_a_number, not_a_number]).u8(),
            1
        );
        // Types 0 and 2 compare their word too.
        for targ_type in [0u8, 2] {
            let x = target(&mut e, targ_type, 5, 0, 0.0);
            let y = target(&mut e, targ_type, 6, 0, 0.0);
            assert_eq!(e.call(0x00680220, &args![x, y]).u8(), 1);
        }
    }

    /// The chunks `Save` handed to the record.
    type Chunks = Rc<RefCell<Vec<Vec<u8>>>>;

    /// Doubles for `Save`: memset, the chunk sink, the swap flag and swap.
    fn save_engine(swap_flag: u8) -> (Engine, Chunks, Rc<RefCell<u32>>) {
        let mut e = engine();
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.map(0x011c_5000, 0x1000);
        e.set_global(0x011c_54ba, swap_flag);
        e.register(NEEDS_BYTE_SWAP, |e, _| {
            e.global::<u8>(0x011c_54ba).into_ret()
        });
        let swaps = Rc::new(RefCell::new(0u32));
        let counter = swaps.clone();
        e.register_double(BYTE_SWAP_FIELDS, move |_, _| {
            *counter.borrow_mut() += 1;
            Ret::default()
        });
        let chunks = Rc::new(RefCell::new(Vec::new()));
        let sink = chunks.clone();
        e.register_double(TES_FORM_ADD_CHUNK_DATA, move |e, a| {
            assert_eq!(a[2], 0x10);
            let bytes = (0..0x10).map(|i| e.mem.u8(a[1] + i)).collect();
            sink.borrow_mut().push(bytes);
            Ret::default()
        });
        (e, chunks, swaps)
    }

    #[test]
    fn save_writes_the_form_id_for_a_reference() {
        let (mut e, chunks, swaps) = save_engine(0);
        let f = form(&mut e, 0x0100_0abc);
        let t = target(&mut e, 0, f, 7, 1.5);
        e.call(0x006802f0, &args![t, 0x1234u32]);
        let chunk = chunks.borrow()[0].clone();
        assert_eq!(chunk[0], 0);
        assert_eq!(&chunk[1..4], &[0, 0, 0]);
        assert_eq!(
            u32::from_le_bytes(chunk[4..8].try_into().unwrap()),
            0x0100_0abc
        );
        assert_eq!(u32::from_le_bytes(chunk[8..12].try_into().unwrap()), 7);
        assert_eq!(f32::from_le_bytes(chunk[12..16].try_into().unwrap()), 1.5);
        assert_eq!(*swaps.borrow(), 0);
    }

    #[test]
    fn save_writes_zero_for_a_missing_form() {
        let (mut e, chunks, _) = save_engine(0);
        let t = target(&mut e, 1, 0, 0, 0.0);
        e.call(0x006802f0, &args![t, 1u32]);
        let chunk = chunks.borrow()[0].clone();
        assert_eq!(chunk[0], 1);
        assert_eq!(&chunk[4..8], &[0, 0, 0, 0]);
    }

    #[test]
    fn save_writes_the_object_type_and_swaps_on_big_endian() {
        let (mut e, chunks, swaps) = save_engine(1);
        let t = target(&mut e, 2, 0x31, 0, 0.0);
        e.call(0x006802f0, &args![t, 1u32]);
        let chunk = chunks.borrow()[0].clone();
        assert_eq!(u32::from_le_bytes(chunk[4..8].try_into().unwrap()), 0x31);
        // Before the write and after it.
        assert_eq!(*swaps.borrow(), 2);
    }

    #[test]
    fn save_of_an_unknown_type_writes_a_zero_word() {
        let (mut e, chunks, _) = save_engine(0);
        let t = target(&mut e, 5, 0x31, 0, 0.0);
        e.call(0x006802f0, &args![t, 1u32]);
        let chunk = chunks.borrow()[0].clone();
        assert_eq!(chunk[0], 5);
        assert_eq!(&chunk[4..8], &[0, 0, 0, 0]);
    }

    /// Doubles for `InitItem`: the file lookup, the compile-index fixup
    /// (adds 0x0100_0000), the form lookup (only form id `known` exists), an
    /// identity cast, the owner's name test and name getter.
    fn init_engine(known: u32, owner_named: bool) -> (Engine, u32) {
        let mut e = engine();
        e.register(TES_FORM_GET_FILE, |_, a| {
            assert_eq!(a[1], 0xffff_ffff);
            0x55u32.into_ret()
        });
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            assert_eq!(a[1], 0x55);
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x0100_0000);
            Ret::default()
        });
        let found = e.mem.alloc(0x20);
        e.register_double(LOOKUP_FORM_BY_ID, move |_, a| {
            if a[0] == known { found } else { 0 }.into_ret()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| {
            assert_eq!(&a[1..], &[0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]);
            a[0].into_ret()
        });
        e.register_double(OWNER_HAS_NAME, move |_, _| {
            u32::from(owner_named).into_ret()
        });
        e.register(PRINT_LINE, |_, _| Ret::default());
        e.put_vtable(0x0120_0000, &[0; 0x4d]);
        e.mem.set_u32(0x0120_0000 + OWNER_NAME_SLOT, 0x0050_0000);
        e.register(0x0050_0000, |_, _| 0x0abc_0000u32.into_ret());
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner, 0x0120_0000);
        e.mem.set_u32(owner + 0xC, 0x0700_0001);
        (e, owner)
    }

    fn prints(log: &[(u32, Vec<u32>)]) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(addr, _)| *addr == PRINT_LINE)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn init_item_resolves_a_reference() {
        let (mut e, owner) = init_engine(0x0100_0042, true);
        let t = target(&mut e, 0, 0x42, 0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x006803b0, &args![t, owner]);
        assert_ne!(word(&e, t), 0);
        assert!(prints(&e.call_log.take().unwrap()).is_empty());
    }

    #[test]
    fn init_item_reports_a_missing_reference_by_owner_name() {
        let (mut e, owner) = init_engine(0, true);
        let t = target(&mut e, 0, 0x42, 0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x006803b0, &args![t, owner]);
        assert_eq!(word(&e, t), 0);
        assert_eq!(
            prints(&e.call_log.take().unwrap()),
            vec![vec![MESSAGES_REFERENCE[0], 0x0100_0042, 0x0abc_0000]]
        );
    }

    #[test]
    fn init_item_reports_a_missing_reference_by_owner_id_or_unknown() {
        let (mut e, owner) = init_engine(0, false);
        let t = target(&mut e, 0, 0x42, 0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x006803b0, &args![t, owner]);
        assert_eq!(
            prints(&e.call_log.take().unwrap()),
            vec![vec![MESSAGES_REFERENCE[1], 0x0100_0042, 0x0700_0001]]
        );
        let t = target(&mut e, 0, 0x42, 0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x006803b0, &args![t, 0u32]);
        assert_eq!(
            prints(&e.call_log.take().unwrap()),
            vec![vec![MESSAGES_REFERENCE[2], 0x0100_0042]]
        );
    }

    #[test]
    fn init_item_resolves_and_reports_an_object() {
        let (mut e, owner) = init_engine(0x0100_0042, true);
        let t = target(&mut e, 1, 0x42, 0, 0.0);
        e.call(0x006803b0, &args![t, owner]);
        assert_ne!(word(&e, t), 0);
        let t = target(&mut e, 1, 0x43, 0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x006803b0, &args![t, owner]);
        assert_eq!(word(&e, t), 0);
        assert_eq!(
            prints(&e.call_log.take().unwrap()),
            vec![vec![MESSAGES_OBJECT[0], 0x0100_0043, 0x0abc_0000]]
        );
    }

    #[test]
    fn init_item_object_messages_without_a_name_or_owner() {
        let (mut e, owner) = init_engine(0, false);
        let t = target(&mut e, 1, 0x43, 0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x006803b0, &args![t, owner]);
        e.set(t, PackageTarget::pRefOrObj, 0x43);
        e.call(0x006803b0, &args![t, 0u32]);
        let printed = prints(&e.call_log.take().unwrap());
        assert_eq!(
            printed[0],
            vec![MESSAGES_OBJECT[1], 0x0100_0043, 0x0700_0001]
        );
        assert_eq!(printed[1], vec![MESSAGES_OBJECT[2], 0x0100_0043]);
    }

    #[test]
    fn init_item_leaves_other_types_alone() {
        let (mut e, owner) = init_engine(0, true);
        let t = target(&mut e, 2, 0x42, 0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x006803b0, &args![t, owner]);
        assert_eq!(word(&e, t), 0x42);
        assert!(prints(&e.call_log.take().unwrap()).is_empty());
    }

    #[test]
    fn save_size_is_13_for_known_types_and_9_otherwise() {
        let mut e = engine();
        for (targ_type, size) in [(0u8, 13u16), (1, 13), (2, 13), (3, 9), (255, 9)] {
            let t = target(&mut e, targ_type, 0, 0, 0.0);
            assert_eq!(e.call(0x00680520, &args![t]).u32() as u16, size);
        }
    }

    /// The calls the helpers of a `PackageTarget` make on the save-game
    /// helper object and buffers, as (address, argument words).
    type Recorded = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles for the save-game helper object at `0x011de45c` (which
    /// points to 0x7000) and for the save and load buffers' readers and
    /// writers: every call is recorded; the numeric-id and form-id loaders
    /// store 0x0100_0ddd through their out pointer.
    fn helper_engine() -> (Engine, Recorded) {
        let mut e = engine();
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_LOAD_OBJECT, 0x7000u32);
        let calls: Recorded = Rc::new(RefCell::new(Vec::new()));
        for addr in [
            SAVE_BYTES,
            LOAD_BYTES,
            SAVE_NUMERIC_ID,
            LOAD_NUMERIC_ID,
            SAVE_GAME_WRITE,
            SAVE_GAME_FORM_ID,
            LOAD_GAME_READ,
            LOAD_GAME_FORM_ID,
        ] {
            let sink = calls.clone();
            e.register_double(addr, move |e, a| {
                sink.borrow_mut().push((addr, a.to_vec()));
                if addr == LOAD_NUMERIC_ID || addr == LOAD_GAME_FORM_ID {
                    e.mem.set_u32(a[1], 0x0100_0ddd);
                }
                Ret::default()
            });
        }
        (e, calls)
    }

    #[test]
    fn helper_save_writes_the_numeric_id_of_the_form() {
        let (mut e, calls) = helper_engine();
        let f = form(&mut e, 0x0100_0abc);
        let t = target(&mut e, 1, f, 0, 0.0);
        let t_addr = t.addr();
        e.call(0x0068_0590, &args![t]);
        let calls = calls.borrow();
        assert_eq!(calls[0], (SAVE_BYTES, vec![0x7000, t_addr, 1]));
        assert_eq!(calls[1], (SAVE_BYTES, vec![0x7000, t_addr + 8, 4]));
        assert_eq!(calls[2], (SAVE_BYTES, vec![0x7000, t_addr + 0xC, 4]));
        // The id is written through a pointer to a local holding the form id.
        assert_eq!(calls[3].0, SAVE_NUMERIC_ID);
        assert_eq!((calls[3].1[0], calls[3].1[2]), (0x7000, 4));
        assert_eq!(calls.len(), 4);
    }

    #[test]
    fn helper_save_writes_the_id_zero_for_no_form_and_the_raw_word_for_a_type() {
        let (mut e, calls) = helper_engine();
        let id_seen = Rc::new(RefCell::new(1u32));
        let seen = id_seen.clone();
        e.register_double(SAVE_NUMERIC_ID, move |e, a| {
            *seen.borrow_mut() = e.mem.u32(a[1]);
            Ret::default()
        });
        let t = target(&mut e, 0, 0, 0, 0.0);
        e.call(0x0068_0590, &args![t]);
        assert_eq!(*id_seen.borrow(), 0);
        calls.borrow_mut().clear();
        let t = target(&mut e, 2, 0x31, 0, 0.0);
        e.call(0x0068_0590, &args![t]);
        assert_eq!(
            calls.borrow()[3],
            (SAVE_BYTES, vec![0x7000, t.addr() + 4, 4])
        );
        calls.borrow_mut().clear();
        let t = target(&mut e, 4, 0x31, 0, 0.0);
        e.call(0x0068_0590, &args![t]);
        assert_eq!(calls.borrow().len(), 3);
    }

    #[test]
    fn helper_load_reads_the_numeric_id_into_the_word() {
        let (mut e, calls) = helper_engine();
        let t = target(&mut e, 0, 0, 0, 0.0);
        e.call(0x0068_0640, &args![t]);
        let t_addr = t.addr();
        assert_eq!(calls.borrow()[0], (LOAD_BYTES, vec![0x7000, t_addr, 1]));
        assert_eq!(calls.borrow()[1], (LOAD_BYTES, vec![0x7000, t_addr + 8, 4]));
        assert_eq!(
            calls.borrow()[2],
            (LOAD_BYTES, vec![0x7000, t_addr + 0xC, 4])
        );
        assert_eq!(calls.borrow()[3].0, LOAD_NUMERIC_ID);
        assert_eq!(word(&e, t), 0x0100_0ddd);
        // Type 2 reads the raw word; other types read nothing more.
        calls.borrow_mut().clear();
        let t = target(&mut e, 2, 0, 0, 0.0);
        e.call(0x0068_0640, &args![t]);
        assert_eq!(
            calls.borrow()[3],
            (LOAD_BYTES, vec![0x7000, t.addr() + 4, 4])
        );
        calls.borrow_mut().clear();
        let t = target(&mut e, 6, 9, 0, 0.0);
        e.call(0x0068_0640, &args![t]);
        assert_eq!(calls.borrow().len(), 3);
        assert_eq!(word(&e, t), 9);
    }

    #[test]
    fn resolve_replaces_the_form_id_by_the_form() {
        let mut e = engine();
        e.register(LOOKUP_FORM_BY_ID, |_, a| (a[0] + 0x1000).into_ret());
        let t = target(&mut e, 1, 0x42, 0, 0.0);
        e.call(0x0068_06e0, &args![t]);
        assert_eq!(word(&e, t), 0x1042);
        let t = target(&mut e, 2, 0x42, 0, 0.0);
        e.call(0x0068_06e0, &args![t]);
        assert_eq!(word(&e, t), 0x42);
    }

    #[test]
    fn save_game_writes_the_fields_and_the_form_or_raw_word() {
        let (mut e, calls) = helper_engine();
        let t = target(&mut e, 0, 0x1234, 0, 0.0);
        let t_addr = t.addr();
        e.call(0x0068_0720, &args![t, 0x9000u32]);
        assert_eq!(
            *calls.borrow(),
            vec![
                (SAVE_GAME_WRITE, vec![0x9000, t_addr, 1, 0]),
                (SAVE_GAME_WRITE, vec![0x9000, t_addr + 8, 4, 0]),
                (SAVE_GAME_WRITE, vec![0x9000, t_addr + 0xC, 4, 0]),
                (SAVE_GAME_FORM_ID, vec![0x9000, 0x1234, 0]),
            ]
        );
        calls.borrow_mut().clear();
        let t = target(&mut e, 2, 0x31, 0, 0.0);
        e.call(0x0068_0720, &args![t, 0x9000u32]);
        assert_eq!(
            calls.borrow()[3],
            (SAVE_GAME_WRITE, vec![0x9000, t.addr() + 4, 4, 0])
        );
        calls.borrow_mut().clear();
        let t = target(&mut e, 3, 0x31, 0, 0.0);
        e.call(0x0068_0720, &args![t, 0x9000u32]);
        assert_eq!(calls.borrow().len(), 3);
    }

    #[test]
    fn load_game_reads_the_fields_and_the_form_or_raw_word() {
        let (mut e, calls) = helper_engine();
        let t = target(&mut e, 1, 0, 0, 0.0);
        let t_addr = t.addr();
        e.call(0x0068_07b0, &args![t, 0x9000u32]);
        assert_eq!(
            *calls.borrow(),
            vec![
                (LOAD_GAME_READ, vec![0x9000, t_addr, 1]),
                (LOAD_GAME_READ, vec![0x9000, t_addr + 8, 4]),
                (LOAD_GAME_READ, vec![0x9000, t_addr + 0xC, 4]),
                (LOAD_GAME_FORM_ID, vec![0x9000, t_addr + 4]),
            ]
        );
        assert_eq!(word(&e, t), 0x0100_0ddd);
        calls.borrow_mut().clear();
        let t = target(&mut e, 2, 0, 0, 0.0);
        e.call(0x0068_07b0, &args![t, 0x9000u32]);
        assert_eq!(
            calls.borrow()[3],
            (LOAD_GAME_READ, vec![0x9000, t.addr() + 4, 4])
        );
        calls.borrow_mut().clear();
        let t = target(&mut e, 3, 0, 0, 0.0);
        e.call(0x0068_07b0, &args![t, 0x9000u32]);
        assert_eq!(calls.borrow().len(), 3);
    }

    #[test]
    fn init_load_game_resolves_a_non_zero_form_id_only() {
        let mut e = engine();
        e.register(LOOKUP_FORM_BY_ID, |_, a| (a[0] + 0x1000).into_ret());
        let t = target(&mut e, 0, 0x42, 0, 0.0);
        e.call(0x0068_0830, &args![t, 0u32]);
        assert_eq!(word(&e, t), 0x1042);
        let t = target(&mut e, 1, 0, 0, 0.0);
        e.call(0x0068_0830, &args![t, 0u32]);
        assert_eq!(word(&e, t), 0);
        let t = target(&mut e, 2, 0x42, 0, 0.0);
        e.call(0x0068_0830, &args![t, 0u32]);
        assert_eq!(word(&e, t), 0x42);
    }
}

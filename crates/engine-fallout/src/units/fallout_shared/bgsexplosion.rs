//! `fallout shared/bgsexplosion.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSExplosion` (form type `0x51`, record `EXPL`) is 0xA8 bytes on the PC
//! (0xB8 on the Xbox: its `TESForm` is 0x10 bytes larger). Its vtable is at
//! `01024a94` (RTTI `BGSExplosion`); the components sit at `+0x30`
//! (`TESFullName`, vtable `01024a80`), `+0x3C` (`TESModel`, `01024a5c`),
//! `+0x54` (`TESEnchantableForm`, `01024a44`), `+0x64` (`BGSPreloadable`,
//! `01024a2c`) and `+0x68` (`TESImageSpaceModifiableForm`, `01024a18`); the
//! impact placed object is at `+0x70` and the record's `DATA` chunk
//! (`BGSExplosionData`, 0x34 bytes) at `+0x74`.
//!
//! Vtable slots checked in the exe: `0x10` deleting destructor
//! (`004facf0`), `0x14` `InitializeData` (`004fade0`), `0x20` `Load`
//! (`004fb0d0`), `0x2C` `Save` (`004fb3b0`), `0x88` `InitItem`
//! (`004fae10`), `0x108` `Copy` (`004fb5a0`), `0x10C` `Compare`
//! (`004fb610`), `0x14C` `Clone3D` (`004fb880`); slot `0x10` of the
//! `BGSPreloadable` vtable (`QueueFiles` in the Xbox PDB) is `004fb6a0`.
//!
//! Several small accessors here are shared by the linker with other classes
//! (identical code); they are described by the offset they touch and what
//! that offset is in an explosion. Functions that run under a compiler
//! exception frame or the stack cookie carry a `Translation incomplete`
//! marker (the unwinding and the cookie check are not translated).

#[allow(unused_imports)]
use crate::prelude::*;

// ---- Layout --------------------------------------------------------------------------

layout! {
    /// `BGSExplosionData` (Xbox PDB), 0x34 bytes: the record's `DATA` chunk.
    /// Offsets are relative to the block; in the explosion it starts at
    /// `+0x74`. Only the fields this unit uses by name are declared.
    pub struct BGSExplosionData: 0x34 {
        /// `pLight` (Xbox PDB, `TESObjectLIGH*`; a form id in the record).
        0x0C pLight: u32,
        /// `pSound1` (Xbox PDB, `TESSound*`).
        0x10 pSound1: u32,
        /// `iFlags` (Xbox PDB).
        0x14 iFlags: u32,
        /// `pImpactDataSet` (Xbox PDB, `BGSImpactDataSet*`).
        0x1C pImpactDataSet: u32,
        /// `pSound2` (Xbox PDB, `TESSound*`).
        0x20 pSound2: u32,
        /// `eSoundLevel` (Xbox PDB `SOUND_LEVEL`).
        0x30 eSoundLevel: u32,
    }
}

layout! {
    /// `BGSExplosion` (Xbox PDB), 0xA8 bytes on the PC.
    pub struct BGSExplosion: 0xA8 {
        /// `pImpactPlacedObject` (Xbox PDB `+0x80`, `TESBoundObject*`).
        0x70 pImpactPlacedObject: u32,
        /// `BGSExplosionData Data` (Xbox PDB `+0x84`).
        0x74 Data: Inline<BGSExplosionData>,
    }
}

/// Offsets of the component bases and of the data block in the explosion.
const FULL_NAME: u32 = 0x30;
const MODEL: u32 = 0x3C;
const ENCHANTABLE: u32 = 0x54;
const PRELOADABLE: u32 = 0x64;
const IMAGE_SPACE_MODIFIABLE: u32 = 0x68;
const DATA: u32 = 0x74;
/// Size of the `DATA` chunk.
const DATA_SIZE: u32 = 0x34;

/// The six vtables the destructor puts back: the form's and the five
/// component bases'.
const VTABLE: u32 = 0x0102_4a94;
const FULL_NAME_VTABLE: u32 = 0x0102_4a80;
const MODEL_VTABLE: u32 = 0x0102_4a5c;
const ENCHANTABLE_VTABLE: u32 = 0x0102_4a44;
const PRELOADABLE_VTABLE: u32 = 0x0102_4a2c;
const IMAGE_SPACE_MODIFIABLE_VTABLE: u32 = 0x0102_4a18;

/// The form type byte of an explosion record (`0x51`).
const FORM_TYPE_EXPLOSION: u32 = 0x51;

/// The four-character code of a chunk, as the file stores it.
const fn tag(name: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*name)
}

const CHUNK_DATA: u32 = tag(b"DATA");
const CHUNK_EDID: u32 = tag(b"EDID");
const CHUNK_OBND: u32 = tag(b"OBND");
const CHUNK_FULL: u32 = tag(b"FULL");
const CHUNK_MODL: u32 = tag(b"MODL");
const CHUNK_MODT: u32 = tag(b"MODT");
const CHUNK_ENAM: u32 = tag(b"ENAM");
const CHUNK_EITM: u32 = tag(b"EITM");
const CHUNK_ANAM: u32 = tag(b"ANAM");
const CHUNK_EAMT: u32 = tag(b"EAMT");
const CHUNK_MNAM: u32 = tag(b"MNAM");
const CHUNK_INAM: u32 = tag(b"INAM");

// ---- Globals and strings ----------------------------------------------------------------

/// The object models are queued with and loaded through (`00443d30`,
/// `ModelLoader::LoadFile`; identity of the global not confirmed).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// A form whose model (`+0x3C`) every explosion queues and that `Clone3D`
/// clones for references flagged by `004fb970` (identity not confirmed).
const SHARED_MODEL_FORM: u32 = 0x011c_a284;
/// `"MASTERFILE: Could not find impact data set (%08X) for explosion '%s'
/// (%08X)."`
const MESSAGE_NO_IMPACT_DATA_SET: u32 = 0x0102_4c70;
/// `"MASTERFILE: Could not find impact placed object (%08X) for explosion
/// '%s' (%08X)."`
const MESSAGE_NO_IMPACT_PLACED_OBJECT: u32 = 0x0102_4c18;
/// The three settings `GetSoundLevelValue` reads, for sound levels 0, 1 and
/// 2 (their names are not confirmed here).
const SOUND_LEVEL_0_SETTING: u32 = 0x011c_dde0;
const SOUND_LEVEL_1_SETTING: u32 = 0x011c_d538;
const SOUND_LEVEL_2_SETTING: u32 = 0x011c_d3b4;

/// Type descriptors (names read from the RTTI): `TESForm`, `TESObjectLIGH`,
/// `TESSound`, `BGSImpactDataSet`, `TESBoundObject` and `BGSExplosion`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_LIGH: u32 = 0x0118_3938;
const TYPE_TES_SOUND: u32 = 0x0118_32fc;
const TYPE_BGS_IMPACT_DATA_SET: u32 = 0x0118_62dc;
const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_BGS_EXPLOSION: u32 = 0x0118_620c;

// ---- Calls outside this file ---------------------------------------------------------

/// The release the deleting destructor calls (`cdecl`, pointer; `00401030`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `TESModel` destructor body (`thiscall`, `00489070`), `TESFullName`
/// destructor (`00591300`) and `TESBoundObject` destructor body
/// (`0050e9f0`).
const MODEL_DESTRUCTOR: u32 = 0x0048_9070;
const FULL_NAME_DESTRUCTOR: u32 = 0x0059_1300;
const BOUND_OBJECT_DESTRUCTOR: u32 = 0x0050_e9f0;
/// Writes the `BGSExplosionData` defaults (`thiscall` on the data block,
/// `004faa20`).
const DATA_DEFAULTS: u32 = 0x004f_aa20;
/// Swaps the byte order of the data block (`CASINO_DATA::Endian` in the
/// engine map, folded code; `thiscall` on the block, `004faaa0`).
const DATA_ENDIAN: u32 = 0x004f_aaa0;
/// Shared accessors (identical code under other names in the engine map):
/// the word at `+0x84` (`pSound1` of an explosion, `004fd3c0`), at `+0x94`
/// (`pSound2`, `009185e0`), at `+0x90` (`pImpactDataSet`, `00516bf0`) and at
/// `+0x70` (`pImpactPlacedObject`, `004fd380`); setters of `+0x80`
/// (`pLight`, `008ac890`), `+0x94` (`004febf0`) and `+0x70` (`0057bd60`).
const GET_SOUND1: u32 = 0x004f_d3c0;
const GET_SOUND2: u32 = 0x0091_85e0;
const GET_IMPACT_DATA_SET: u32 = 0x0051_6bf0;
const GET_IMPACT_PLACED_OBJECT: u32 = 0x004f_d380;
const SET_LIGHT: u32 = 0x008a_c890;
const SET_SOUND2: u32 = 0x004f_ebf0;
const SET_IMPACT_PLACED_OBJECT: u32 = 0x0057_bd60;
/// Bit 3 of the form flags (`AL`), `thiscall` (`004013e0`), and its setter
/// (`thiscall` (form, value), `00484ab0`).
const FORM_FLAG_8_TEST: u32 = 0x0040_13e0;
const FORM_SET_FLAG_8: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (Xbox PDB), `thiscall` (form, `-1`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB), `cdecl` (pointer to the id, file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Form lookup by id (`cdecl`, one word; `004839c0`).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `iFormID` getter (`thiscall`; folded body `0084e3a0`).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// The message function the loaders call (`cdecl`, format and words; its
/// body returns 0 in this build; `005b5e40`).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// `TESImageSpaceModifiableForm::InitItem` and `TESEnchantableForm::InitItem`
/// (Xbox PDB), `thiscall` (component, form).
const IMAGE_SPACE_MODIFIABLE_INIT_ITEM: u32 = 0x0048_7490;
const ENCHANTABLE_INIT_ITEM: u32 = 0x0048_3210;
/// Stores a word at `+4` of the object (`thiscall`, `RET 4`; named
/// `CombatProcedureAttackMelee::Initialize` in the engine map because the
/// code is shared): the enchantment of `TESEnchantableForm`, the image space
/// modifier of `TESImageSpaceModifiableForm`.
const SET_WORD_4: u32 = 0x006e_cd40;
/// Stores a 16-bit value at `+8` of the `TESEnchantableForm` (the amount of
/// enchantment; `thiscall`, `00483170`).
const ENCHANTABLE_SET_AMOUNT: u32 = 0x0048_3170;
/// `TESForm::LoadForm` (Xbox PDB), `thiscall` (form, file).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESForm::LoadData` (Xbox PDB), `thiscall` (form, file, buffer, size).
const FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `TESForm::__SaveData` (Xbox PDB), `thiscall` (form, buffer, size).
const FORM_SAVE_DATA: u32 = 0x0048_5e20;
/// `TESForm::StartForm` and `TESForm::CloseForm` (Xbox PDB), `thiscall`.
const FORM_START_FORM: u32 = 0x0048_55a0;
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// A chunk holding one word (`cdecl`, chunk, value; `00485910`).
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// Save-side byte-swap switch (`cdecl`, no arguments, `AL`; `00401500`).
const SWAP_ON_SAVE: u32 = 0x0040_1500;
/// `TESFullName::Save`, `TESModel::Save` (model, model chunk, texture chunk)
/// and `TESEnchantableForm::Save` (Xbox PDB), `thiscall`; the image space
/// modifier's save (`MNAM`, `thiscall`, `00487450`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const MODEL_SAVE: u32 = 0x0048_91d0;
const ENCHANTABLE_SAVE: u32 = 0x0047_bd70;
const IMAGE_SPACE_MODIFIABLE_SAVE: u32 = 0x0048_7450;
/// The `TESFullName` loader (`cdecl`, component, file; `00487050`).
const FULL_NAME_LOAD: u32 = 0x0048_7050;
/// `TESModel::LoadModelChunk` (Xbox PDB), `cdecl` (model, file).
const MODEL_LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESFile::GetTESForm` (Xbox PDB): the form type of the record being read.
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (Xbox PDB): the current chunk's code, 0 at the end.
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
/// Moves to the next chunk (`AL`: true while there is one), `thiscall`.
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData_ov3` (Xbox PDB): `thiscall` (file, buffer, size).
const FILE_GET_CHUNK_BYTES: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB): `thiscall` (file, pointer to a word).
const FILE_GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov2` (Xbox PDB): `thiscall` (file, pointer to a
/// 16-bit value).
const FILE_GET_CHUNK_SHORT: u32 = 0x0047_2840;
/// The size of the current chunk, `thiscall` (file).
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// Whether the file's data are byte-swapped (`AL`, `thiscall`).
const FILE_SWAPS: u32 = 0x0040_1680;
/// `TESForm::CopyAllComponents` and `TESForm::CompareAllComponents` (Xbox
/// PDB), `thiscall` (form, other form).
const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `memcpy` wrapper (`cdecl`: destination, source, count; `00401460`) and
/// `memcmp` (`00ec4835`).
const MEMCPY: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
/// The length of a model's path (`thiscall` on the `TESModel`; `0048cee0`).
const MODEL_PATH_LENGTH: u32 = 0x0048_cee0;
/// `TES::GetLODMult` (Xbox PDB name; `cdecl`, one word: the form; `0045c6b0`).
const FORM_LOD_MULT: u32 = 0x0045_c6b0;
/// Queues a model for loading (`thiscall` on `MODEL_LOADER`: model, two
/// words, a multiplier and three flags; `RET 0x1C`; `00443d30`).
const QUEUE_MODEL: u32 = 0x0044_3d30;
/// `ModelLoader::LoadFile` (Xbox PDB), `thiscall` on `MODEL_LOADER` (path
/// and five words).
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
/// The impact data set's model for a material (`cdecl` (material) maps it to
/// an index, `0058e8f0`; `thiscall` (set, index) returns the entry,
/// `0058e9d0`).
const IMPACT_INDEX_FOR_MATERIAL: u32 = 0x0058_e8f0;
const IMPACT_DATA_SET_GET: u32 = 0x0058_e9d0;
/// Returns the setting's value address (`thiscall` on the setting,
/// `0043d4d0`).
const SETTING_VALUE: u32 = 0x0043_d4d0;
/// The byte at `+0xD9` of a reference (`thiscall`, `004fb970`): when set,
/// `Clone3D` clones the shared model instead.
const REFERENCE_BYTE_D9: u32 = 0x004f_b970;
/// `TESBoundObject::Clone3D` (Xbox PDB), `thiscall` (object, reference,
/// flag).
const BOUND_OBJECT_CLONE3D: u32 = 0x0050_ee50;
/// `NiCloningProcess` constructor (`thiscall`, process, scale; `004ad050`),
/// `NiCloningProcess::~NiCloningProcess` (Xbox PDB) and `NiObject::Clone`
/// (Xbox PDB, `thiscall`, object, process).
const CLONING_PROCESS_CONSTRUCTOR: u32 = 0x004a_d050;
const CLONING_PROCESS_DESTRUCTOR: u32 = 0x004a_d270;
const NI_OBJECT_CLONE: u32 = 0x00a5_d2c0;
/// `NiCloningProcess` is 0x1C bytes on the stack of `Clone3D`.
const CLONING_PROCESS_SIZE: u32 = 0x1c;

/// Virtual slots of a form: `ClearData` (`0x18`, Xbox PDB), `LoadObjectBound`
/// (`0xE0`, Xbox PDB), the name string the messages print (`0x130`) and the
/// editor id setter `Load` calls for `EDID` (`0x134`).
const FORM_CLEAR_DATA_SLOT: u32 = 0x18;
const FORM_LOAD_OBJECT_BOUND_SLOT: u32 = 0xe0;
const FORM_GET_EDITOR_ID_SLOT: u32 = 0x130;
const FORM_SET_EDITOR_ID_SLOT: u32 = 0x134;
/// `TESModel` virtual slot `0x14`: the model path.
const MODEL_GET_PATH_SLOT: u32 = 0x14;
/// Slot `0x10` of a `BGSPreloadable` component (`QueueFiles`, Xbox PDB).
const PRELOADABLE_QUEUE_FILES_SLOT: u32 = 0x10;

/// A component pointer, or null when the object itself is null.
fn component(object: u32, offset: u32) -> u32 {
    if object == 0 {
        0
    } else {
        object + offset
    }
}

/// The editor id and form id of the form, as the messages print them.
fn name_and_id(e: &mut Engine, this: Ptr<BGSExplosion>) -> (u32, u32) {
    let id = e.call(GET_FORM_ID, &args![this]).u32();
    let name = e.vcall(this.addr(), FORM_GET_EDITOR_ID_SLOT, &[]).u32();
    (name, id)
}

/// The resolution `InitItem` does for a form id: move the id to the running
/// load order (the form's file first), look it up and cast it to `target`.
/// Returns the moved id and the cast form.
fn resolve_form(e: &mut Engine, this: Ptr<BGSExplosion>, id: u32, target: u32) -> (u32, u32) {
    let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
    let id = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), id);
        e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
        e.mem.u32(slot.addr())
    });
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
        )
        .u32();
    (id, cast)
}

/// The form id of a form pointer, 0 for null (the game calls the getter a
/// second time for the id).
fn id_of(e: &mut Engine, this: Ptr<BGSExplosion>, getter: u32) -> u32 {
    if e.call(getter, &args![this]).u32() != 0 {
        let form = e.call(getter, &args![this]).u32();
        e.call(GET_FORM_ID, &args![form]).u32()
    } else {
        0
    }
}

// ---- Accessors ---------------------------------------------------------------------------

// Translated from 00477950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSExplosion::GetData` (Xbox PDB; the body tests the data block's
/// `iFlags`): `iFlags & mask != 0` in `AL`. The rest of `EAX` keeps the upper
/// bytes of `iFlags & mask`, as the game's `SETNZ AL` leaves them.
pub fn bgs_explosion_get_data(e: &mut Engine, this: Ptr<BGSExplosion>, mask: u32) -> u32 {
    let data = fn_004fadc0(e, this);
    let masked = e.get(data, BGSExplosionData::iFlags) & mask;
    (masked & 0xffff_ff00) | (masked != 0) as u32
}

// Translated from 004fadc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the data block (`this + 0x74`).
pub fn fn_004fadc0(_e: &mut Engine, this: Ptr<BGSExplosion>) -> Ptr<BGSExplosionData> {
    this.at(BGSExplosion::Data)
}

// Translated from 004fb070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x80` (`Data.pLight` of an explosion; other classes share
/// the code).
pub fn fn_004fb070(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x80)
}

// Translated from 004fb090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `+0x84` (`Data.pSound1` of an explosion; other classes
/// share the code).
pub fn fn_004fb090(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x84, value);
}

// Translated from 004fb0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `+0x90` (`Data.pImpactDataSet` of an explosion; other
/// classes share the code).
pub fn fn_004fb0b0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x90, value);
}

// Translated from 004fb810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSExplosion::GetSoundLevelValue` (Xbox PDB): the value of the setting
/// for `Data.eSoundLevel` 0, 1 or 2 (`011cdde0`, `011cd538`, `011cd3b4`,
/// read through `0043d4d0`); 0 for any other level.
pub fn bgs_explosion_get_sound_level_value(e: &mut Engine, this: Ptr<BGSExplosion>) -> u32 {
    let data = this.at(BGSExplosion::Data);
    let setting = match e.get(data, BGSExplosionData::eSoundLevel) {
        0 => SOUND_LEVEL_0_SETTING,
        1 => SOUND_LEVEL_1_SETTING,
        2 => SOUND_LEVEL_2_SETTING,
        _ => return 0,
    };
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(value)
}

// Translated from 004fb7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// On an impact data set (`this`): the entry for `material` (`0058e8f0` maps
/// the material to an index, `0058e9d0` reads the set's entry).
pub fn fn_004fb7a0(e: &mut Engine, this: Ptr, material: u32) -> u32 {
    let index = e.call(IMPACT_INDEX_FOR_MATERIAL, &args![material]).u32();
    e.call(IMPACT_DATA_SET_GET, &args![this, index]).u32()
}

// Translated from 004fb7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The impact data set's entry for `material` (`004fb7a0`), or 0 when the
/// explosion has no impact data set.
pub fn fn_004fb7d0(e: &mut Engine, this: Ptr<BGSExplosion>, material: u32) -> u32 {
    if e.call(GET_IMPACT_DATA_SET, &args![this]).u32() == 0 {
        return 0;
    }
    let set = e.call(GET_IMPACT_DATA_SET, &args![this]).u32();
    fn_004fb7a0(e, Ptr::new(set), material)
}

// ---- Destructor, data ----------------------------------------------------------------------

// Translated from 004fabf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fabf0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x51 (0xa8 bytes); `ECX` is the object.
/// The engine map names it `BGSExplosion::BGSExplosion` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0102_4a94`) is `BGSExplosion`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004fabf0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_8f50, &args![base + 0x3c]);
    e.call(0x0048_30a0, &args![base + 0x54]);
    e.call(0x004f_9d90, &args![base + 0x64]);
    e.call(0x0048_7420, &args![base + 0x68]);
    e.mem.set_u32(base, 0x0102_4a94);
    e.mem.set_u32(base + 0x30, 0x0102_4a80);
    e.mem.set_u32(base + 0x3c, 0x0102_4a5c);
    e.mem.set_u32(base + 0x54, 0x0102_4a44);
    e.mem.set_u32(base + 0x64, 0x0102_4a2c);
    e.mem.set_u32(base + 0x68, 0x0102_4a18);
    e.call(0x004f_aa00, &args![base + 0x74]);
    e.call(0x004f_15a0, &args![this, 0x51u32]);
    e.call(0x004f_ade0, &args![this]);
    e.call(0x0041_fd00, &args![base + 0x54, 0x2u32]);
    this
}

// Translated from 004facf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (vtable slot `0x10`): runs the destructor body
/// (`004fad20`) and, when bit 0 of `flags` is set, frees the object. Returns
/// `this`.
pub fn fn_004facf0(e: &mut Engine, this: Ptr<BGSExplosion>, flags: u32) -> Ptr<BGSExplosion> {
    fn_004fad20(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004fad20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fad20 - the C++ exception unwinding frame (FS:[0] state table that tracks which members are destroyed) is not translated.
/// The destructor body: puts the six vtables back, destroys the model and
/// the full name components and runs the `TESBoundObject` destructor.
pub fn fn_004fad20(e: &mut Engine, this: Ptr<BGSExplosion>) {
    let object = this.addr();
    e.mem.set_u32(object, VTABLE);
    e.mem.set_u32(object + FULL_NAME, FULL_NAME_VTABLE);
    e.mem.set_u32(object + MODEL, MODEL_VTABLE);
    e.mem.set_u32(object + ENCHANTABLE, ENCHANTABLE_VTABLE);
    e.mem.set_u32(object + PRELOADABLE, PRELOADABLE_VTABLE);
    e.mem.set_u32(
        object + IMAGE_SPACE_MODIFIABLE,
        IMAGE_SPACE_MODIFIABLE_VTABLE,
    );
    e.call(MODEL_DESTRUCTOR, &args![object + MODEL]);
    e.call(FULL_NAME_DESTRUCTOR, &args![object + FULL_NAME]);
    e.call(BOUND_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 004fade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x14` (`InitializeData` in the Xbox PDB): clears the impact
/// placed object and writes the data block's defaults (`004faa20`).
pub fn fn_004fade0(e: &mut Engine, this: Ptr<BGSExplosion>) {
    e.set(this, BGSExplosion::pImpactPlacedObject, 0);
    let data = fn_004fadc0(e, this);
    e.call(DATA_DEFAULTS, &args![data]);
}

// ---- InitItem, Load, Save --------------------------------------------------------------

// Translated from 004fae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSExplosion::InitItem` (Xbox PDB), vtable slot `0x88`: unless bit 3 of
/// the form flags is set, turns the form ids into forms (the id moved to the
/// running load order with the explosion's file first, then cast): the
/// light (`TESObjectLIGH`), the two sounds (`TESSound`), the impact data set
/// (`BGSImpactDataSet`; a miss is logged) and the impact placed object
/// (`TESBoundObject`; a miss is logged). Zero ids are left alone. Then
/// initializes the image space modifiable and enchantable components and
/// marks the form initialized.
pub fn bgs_explosion_init_item(e: &mut Engine, this: Ptr<BGSExplosion>) {
    if e.call(FORM_FLAG_8_TEST, &args![this]).bool() {
        return;
    }
    let untyped = this.cast::<()>();
    let light = fn_004fb070(e, untyped);
    if light != 0 {
        let (_, form) = resolve_form(e, this, light, TYPE_TES_OBJECT_LIGH);
        e.call(SET_LIGHT, &args![this, form]);
    }
    let sound = e.call(GET_SOUND1, &args![this]).u32();
    if sound != 0 {
        let (_, form) = resolve_form(e, this, sound, TYPE_TES_SOUND);
        fn_004fb090(e, untyped, form);
    }
    let sound = e.call(GET_SOUND2, &args![this]).u32();
    if sound != 0 {
        let (_, form) = resolve_form(e, this, sound, TYPE_TES_SOUND);
        e.call(SET_SOUND2, &args![this, form]);
    }
    let set = e.call(GET_IMPACT_DATA_SET, &args![this]).u32();
    if set != 0 {
        let (id, form) = resolve_form(e, this, set, TYPE_BGS_IMPACT_DATA_SET);
        fn_004fb0b0(e, untyped, form);
        if e.call(GET_IMPACT_DATA_SET, &args![this]).u32() == 0 {
            let (name, form_id) = name_and_id(e, this);
            e.call(
                PRINT_MESSAGE,
                &args![MESSAGE_NO_IMPACT_DATA_SET, id, name, form_id],
            );
        }
    }
    let placed = e.get(this, BGSExplosion::pImpactPlacedObject);
    if placed != 0 {
        let (id, form) = resolve_form(e, this, placed, TYPE_TES_BOUND_OBJECT);
        e.set(this, BGSExplosion::pImpactPlacedObject, form);
        if e.get(this, BGSExplosion::pImpactPlacedObject) == 0 {
            let (name, form_id) = name_and_id(e, this);
            e.call(
                PRINT_MESSAGE,
                &args![MESSAGE_NO_IMPACT_PLACED_OBJECT, id, name, form_id],
            );
        }
    }
    e.call(
        IMAGE_SPACE_MODIFIABLE_INIT_ITEM,
        &args![this.addr() + IMAGE_SPACE_MODIFIABLE, this],
    );
    e.call(
        ENCHANTABLE_INIT_ITEM,
        &args![this.addr() + ENCHANTABLE, this],
    );
    e.call(FORM_SET_FLAG_8, &args![this, 1u32]);
}

/// A word chunk read into a zeroed local (`TESFile::GetChunkData`).
fn read_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, slot| {
        e.call(FILE_GET_CHUNK_WORD, &args![file, slot]);
        e.mem.u32(slot.addr())
    })
}

// Translated from 004fb0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSExplosion::Load` (vtable slot `0x20`): reads an explosion record (form
/// type `0x51`, compared as a byte) from `file` and returns whether it did.
/// Clears the data (virtual slot `0x18`), reads the form header, clears bit 3
/// of the form flags and loops over the chunks: `EDID` (editor id, slot
/// `0x134`), `OBND` (slot `0xE0`), `FULL` (name), `MODL`/`MODT` (model),
/// `ENAM`/`EITM` (the enchantment form id, stored at `+4` of the enchantable
/// component), `ANAM`/`EAMT` (its amount, 16 bits), `MNAM` (the image space
/// modifier's form id), `INAM` (the impact placed object's form id) and
/// `DATA` (the 0x34-byte data block, byte-swapped when the file is). Unknown
/// chunks are skipped.
///
/// Not translated: the stack-protector cookie. The `alloca` buffer for `EDID`
/// is a heap block here.
// Translation incomplete: 004fb0d0 - the compiler's security cookie check (`__security_check_cookie`, 00ec408c) is not translated.
pub fn bgs_explosion_load(e: &mut Engine, this: Ptr<BGSExplosion>, file: Ptr) -> bool {
    let object = this.addr();
    if e.call(FILE_GET_FORM_TYPE, &args![file]).u8() as u32 != FORM_TYPE_EXPLOSION {
        return false;
    }
    e.vcall(object, FORM_CLEAR_DATA_SLOT, &[]);
    e.call(FORM_LOAD_FORM, &args![this, file]);
    e.call(FORM_SET_FLAG_8, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_ENAM | CHUNK_EITM => {
                let id = read_word(e, file);
                e.call(SET_WORD_4, &args![object + ENCHANTABLE, id]);
            }
            CHUNK_MODL | CHUNK_MODT => {
                let model = component(object, MODEL);
                e.call(MODEL_LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_DATA => {
                e.call(FORM_LOAD_DATA, &args![this, file, object + DATA, DATA_SIZE]);
            }
            CHUNK_EDID => {
                // The game takes a stack buffer as large as the chunk and reads
                // at most 0x200 bytes into it.
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                let buffer = e.mem.alloc(size.max(1));
                e.call(FILE_GET_CHUNK_BYTES, &args![file, buffer, 0x200u32]);
                e.vcall(object, FORM_SET_EDITOR_ID_SLOT, &args![buffer]);
                e.mem.free(buffer);
            }
            CHUNK_OBND => {
                e.vcall(object, FORM_LOAD_OBJECT_BOUND_SLOT, &args![file]);
            }
            CHUNK_FULL => {
                let name = component(object, FULL_NAME);
                e.call(FULL_NAME_LOAD, &args![name, file]);
            }
            CHUNK_ANAM | CHUNK_EAMT => {
                let amount = e.with_stack(4, |e, slot| {
                    e.mem.set_u16(slot.addr(), 0);
                    e.call(FILE_GET_CHUNK_SHORT, &args![file, slot]);
                    e.mem.u16(slot.addr())
                });
                e.call(
                    ENCHANTABLE_SET_AMOUNT,
                    &args![object + ENCHANTABLE, amount as u32],
                );
            }
            CHUNK_MNAM => {
                let id = read_word(e, file);
                e.call(SET_WORD_4, &args![object + IMAGE_SPACE_MODIFIABLE, id]);
            }
            CHUNK_INAM => {
                let id = read_word(e, file);
                e.set(this, BGSExplosion::pImpactPlacedObject, id);
            }
            _ => {}
        }
        if chunk == CHUNK_DATA && e.call(FILE_SWAPS, &args![file]).bool() {
            e.call(DATA_ENDIAN, &args![object + DATA]);
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 004fb3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSExplosion::Save` (vtable slot `0x2C`): writes the record: the name,
/// the model (`MODL`/`MODT`), the enchantment, the image space modifier, the
/// 0x34-byte data block (with the light, the two sounds and the impact data
/// set replaced by their form ids while it is written, and byte-swapped
/// around the write when the save swaps), then the impact placed object's
/// form id (`INAM`) when there is one.
pub fn bgs_explosion_save(e: &mut Engine, this: Ptr<BGSExplosion>) {
    let object = this.addr();
    let untyped = this.cast::<()>();
    e.call(FORM_START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![object + FULL_NAME]);
    e.call(MODEL_SAVE, &args![object + MODEL, CHUNK_MODL, CHUNK_MODT]);
    e.call(ENCHANTABLE_SAVE, &args![object + ENCHANTABLE]);
    e.call(
        IMAGE_SPACE_MODIFIABLE_SAVE,
        &args![object + IMAGE_SPACE_MODIFIABLE],
    );

    let light = fn_004fb070(e, untyped);
    let light_id = if fn_004fb070(e, untyped) != 0 {
        let form = fn_004fb070(e, untyped);
        e.call(GET_FORM_ID, &args![form]).u32()
    } else {
        0
    };
    e.call(SET_LIGHT, &args![this, light_id]);
    let sound1 = e.call(GET_SOUND1, &args![this]).u32();
    let id = id_of(e, this, GET_SOUND1);
    fn_004fb090(e, untyped, id);
    let sound2 = e.call(GET_SOUND2, &args![this]).u32();
    let id = id_of(e, this, GET_SOUND2);
    e.call(SET_SOUND2, &args![this, id]);
    let impact = e.call(GET_IMPACT_DATA_SET, &args![this]).u32();
    let id = id_of(e, this, GET_IMPACT_DATA_SET);
    fn_004fb0b0(e, untyped, id);

    if e.call(SWAP_ON_SAVE, &[]).bool() {
        e.call(DATA_ENDIAN, &args![object + DATA]);
    }
    e.call(FORM_SAVE_DATA, &args![this, object + DATA, DATA_SIZE]);
    if e.call(SWAP_ON_SAVE, &[]).bool() {
        e.call(DATA_ENDIAN, &args![object + DATA]);
    }

    e.call(SET_LIGHT, &args![this, light]);
    fn_004fb090(e, untyped, sound1);
    e.call(SET_SOUND2, &args![this, sound2]);
    fn_004fb0b0(e, untyped, impact);

    if e.call(GET_IMPACT_PLACED_OBJECT, &args![this]).u32() != 0 {
        let placed = e.call(GET_IMPACT_PLACED_OBJECT, &args![this]).u32();
        let id = e.call(GET_FORM_ID, &args![placed]).u32();
        e.call(ADD_CHUNK_WORD, &args![CHUNK_INAM, id]);
    }
    e.call(FORM_CLOSE_FORM, &args![this]);
}

// ---- Copy, Compare -----------------------------------------------------------------------

/// `__RTDynamicCast` of `other` to `BGSExplosion`.
fn cast_to_explosion(e: &mut Engine, other: u32) -> Ptr<BGSExplosion> {
    e.call(
        RT_DYNAMIC_CAST,
        &args![other, 0u32, TYPE_TES_FORM, TYPE_BGS_EXPLOSION, 0u32],
    )
    .ptr()
}

// Translated from 004fb5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x108` (`Copy` in the Xbox PDB): when `other` is an
/// explosion, copies its components (`TESForm::CopyAllComponents`), its
/// 0x34-byte data block and its impact placed object.
pub fn fn_004fb5a0(e: &mut Engine, this: Ptr<BGSExplosion>, other: u32) {
    let source = cast_to_explosion(e, other);
    if source.is_null() {
        return;
    }
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, source]);
    let source_data = fn_004fadc0(e, source);
    let data = fn_004fadc0(e, this);
    e.call(MEMCPY, &args![data, source_data, DATA_SIZE]);
    let placed = e.call(GET_IMPACT_PLACED_OBJECT, &args![source]).u32();
    e.call(SET_IMPACT_PLACED_OBJECT, &args![this, placed]);
}

// Translated from 004fb610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x10C` (`Compare` in the Xbox PDB): true when `other` is not
/// an explosion, or its components, data block or impact placed object
/// differ.
pub fn fn_004fb610(e: &mut Engine, this: Ptr<BGSExplosion>, other: u32) -> bool {
    let target = cast_to_explosion(e, other);
    if target.is_null() {
        return true;
    }
    if e.call(FORM_COMPARE_ALL_COMPONENTS, &args![this, target])
        .bool()
    {
        return true;
    }
    let target_data = fn_004fadc0(e, target);
    if e.call(MEMCMP, &args![this.addr() + DATA, target_data, DATA_SIZE])
        .i32()
        != 0
    {
        return true;
    }
    let mine = e.call(GET_IMPACT_PLACED_OBJECT, &args![this]).u32();
    let theirs = e.call(GET_IMPACT_PLACED_OBJECT, &args![target]).u32();
    mine != theirs
}

// ---- Preloading, Clone3D -----------------------------------------------------------------

// Translated from 004fb6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSPreloadable` component's virtual slot `0x10` (`QueueFiles` in the
/// Xbox PDB) as the explosion installs it; `this` is the component, at
/// `+0x64` of the explosion. Queues the explosion's model when it has a path
/// (with the multiplier `0045c6b0` gives for the explosion), calls the same
/// slot of the impact data set's component at `+0x18` of the set (when there
/// is a set), then queues the model at `+0x3C` of the form in `011ca284`
/// when there is that form and its model has a path (the explosion's
/// multiplier again); each with the two arguments.
pub fn fn_004fb6a0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let explosion = this.addr().wrapping_sub(PRELOADABLE);
    let model = this.addr().wrapping_sub(PRELOADABLE - MODEL);
    if e.call(MODEL_PATH_LENGTH, &args![model]).u32() != 0 {
        let model = if explosion == 0 { 0 } else { model };
        let multiplier = e.call(FORM_LOD_MULT, &args![explosion]).u32();
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(
            QUEUE_MODEL,
            &args![loader, model, first, second, multiplier, 1u32, 0u32, 0u32],
        );
    }
    // `Data.pImpactDataSet`, at `+0x2C` of the component.
    let set = e.mem.u32(this.addr() + 0x2c);
    if set != 0 {
        e.vcall(
            set + 0x18,
            PRELOADABLE_QUEUE_FILES_SLOT,
            &args![first, second],
        );
    }
    if e.global::<u32>(SHARED_MODEL_FORM) != 0 {
        let shared = e.global::<u32>(SHARED_MODEL_FORM);
        if e.call(MODEL_PATH_LENGTH, &args![shared + MODEL]).u32() != 0 {
            let shared = e.global::<u32>(SHARED_MODEL_FORM);
            let shared_model = component(shared, MODEL);
            let multiplier = e.call(FORM_LOD_MULT, &args![explosion]).u32();
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(
                QUEUE_MODEL,
                &args![
                    loader,
                    shared_model,
                    first,
                    second,
                    multiplier,
                    1u32,
                    0u32,
                    0u32
                ],
            );
        }
    }
}

// Translated from 004fb880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSExplosion::Clone3D` (Xbox PDB), vtable slot `0x14C`: makes the 3D
/// model for `reference`. Unless the reference's byte at `+0xD9` is set
/// (`004fb970`), this is `TESBoundObject::Clone3D` (with `flag`). Otherwise
/// the model of the form in `011ca284` (its `TESModel` path, virtual slot
/// `0x14`) is loaded through `ModelLoader::LoadFile` (path, 0, 1, 0, 0, 0)
/// and, when that gives an object, cloned with a fresh `NiCloningProcess`
/// (scale 1.0); null when it does not.
// Translation incomplete: 004fb880 - the C++ exception unwinding frame (FS:[0] handler chain that destroys the stack NiCloningProcess if a call throws) is not translated.
pub fn bgs_explosion_clone3d(
    e: &mut Engine,
    this: Ptr<BGSExplosion>,
    reference: u32,
    flag: u8,
) -> u32 {
    if !e.call(REFERENCE_BYTE_D9, &args![reference]).bool() {
        return e
            .call(BOUND_OBJECT_CLONE3D, &args![this, reference, flag as u32])
            .u32();
    }
    let shared = e.global::<u32>(SHARED_MODEL_FORM);
    let path = e.vcall(shared + MODEL, MODEL_GET_PATH_SLOT, &[]).u32();
    let loader = e.global::<u32>(MODEL_LOADER);
    let model = e
        .call(
            MODEL_LOADER_LOAD_FILE,
            &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
        )
        .u32();
    e.with_stack(CLONING_PROCESS_SIZE, |e, process| {
        e.call(
            CLONING_PROCESS_CONSTRUCTOR,
            &args![process, 1.0f32.to_bits()],
        );
        let clone = if model != 0 {
            e.call(NI_OBJECT_CLONE, &args![model, process]).u32()
        } else {
            0
        };
        e.call(CLONING_PROCESS_DESTRUCTOR, &args![process]);
        clone
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0047_7950,
            bgs_explosion_get_data(Ptr<BGSExplosion>, u32) -> u32
        ),
        entry!(0x004f_abf0, fn_004fabf0(Ptr) -> Ptr),
        entry!(
            0x004f_acf0,
            fn_004facf0(Ptr<BGSExplosion>, u32) -> Ptr<BGSExplosion>
        ),
        entry!(0x004f_ad20, fn_004fad20(Ptr<BGSExplosion>)),
        entry!(
            0x004f_adc0,
            fn_004fadc0(Ptr<BGSExplosion>) -> Ptr<BGSExplosionData>
        ),
        entry!(0x004f_ade0, fn_004fade0(Ptr<BGSExplosion>)),
        entry!(0x004f_ae10, bgs_explosion_init_item(Ptr<BGSExplosion>)),
        entry!(0x004f_b070, fn_004fb070(Ptr) -> u32),
        entry!(0x004f_b090, fn_004fb090(Ptr, u32)),
        entry!(0x004f_b0b0, fn_004fb0b0(Ptr, u32)),
        entry!(
            0x004f_b0d0,
            bgs_explosion_load(Ptr<BGSExplosion>, Ptr) -> bool
        ),
        entry!(0x004f_b3b0, bgs_explosion_save(Ptr<BGSExplosion>)),
        entry!(0x004f_b5a0, fn_004fb5a0(Ptr<BGSExplosion>, u32)),
        entry!(0x004f_b610, fn_004fb610(Ptr<BGSExplosion>, u32) -> bool),
        entry!(0x004f_b6a0, fn_004fb6a0(Ptr, u32, u32)),
        entry!(0x004f_b7a0, fn_004fb7a0(Ptr, u32) -> u32),
        entry!(0x004f_b7d0, fn_004fb7d0(Ptr<BGSExplosion>, u32) -> u32),
        entry!(
            0x004f_b810,
            bgs_explosion_get_sound_level_value(Ptr<BGSExplosion>) -> u32
        ),
        entry!(
            0x004f_b880,
            bgs_explosion_clone3d(Ptr<BGSExplosion>, u32, u8) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Vec<(u32, Vec<u32>)>;

    fn explosion(e: &mut Engine) -> Ptr<BGSExplosion> {
        e.new_object()
    }

    fn returning(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            returning(e, addr, 0);
        }
    }

    fn give_vtable(e: &mut Engine, object: u32, slots: &[(u32, u32)]) {
        let vtable = e.mem.alloc(0x200);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        e.mem.set_u32(object, vtable);
    }

    /// The logged calls to `addr`, without the address.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The logged call addresses that are in `addrs`, in order.
    fn order_of(e: &Engine, addrs: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| addrs.contains(a))
            .map(|(a, _)| *a)
            .collect()
    }

    /// Doubles for the shared accessors, reading and writing the explosion
    /// like the game.
    fn accessor_doubles(e: &mut Engine) {
        for (addr, offset) in [
            (GET_SOUND1, 0x84u32),
            (GET_SOUND2, 0x94),
            (GET_IMPACT_DATA_SET, 0x90),
            (GET_IMPACT_PLACED_OBJECT, 0x70),
        ] {
            e.register_double(addr, move |e, a| Ret {
                eax: e.mem.u32(a[0] + offset),
                ..Ret::default()
            });
        }
        for (addr, offset) in [
            (SET_LIGHT, 0x80u32),
            (SET_SOUND2, 0x94),
            (SET_IMPACT_PLACED_OBJECT, 0x70),
        ] {
            e.register_double(addr, move |e, a| {
                e.mem.set_u32(a[0] + offset, a[1]);
                Ret::default()
            });
        }
        e.register(GET_FORM_ID, |_, a| Ret {
            eax: a[0] | 0x0100_0000,
            ..Ret::default()
        });
    }

    #[test]
    fn fn_004fabf0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_abf0,
            size: 0xa8,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_8f50, &[A::T(0x3c)]),
                (0x0048_30a0, &[A::T(0x54)]),
                (0x004f_9d90, &[A::T(0x64)]),
                (0x0048_7420, &[A::T(0x68)]),
                (0x004f_aa00, &[A::T(0x74)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x51)]),
                (0x004f_ade0, &[A::T(0x0)]),
                (0x0041_fd00, &[A::T(0x54), A::V(0x2)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_4a94),
                (0x30, 4, 0x0102_4a80),
                (0x3c, 4, 0x0102_4a5c),
                (0x54, 4, 0x0102_4a44),
                (0x64, 4, 0x0102_4a2c),
                (0x68, 4, 0x0102_4a18),
            ],
        });
    }

    #[test]
    fn get_data_tests_the_flags_and_keeps_the_upper_bytes() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        e.mem.set_u32(x.addr() + 0x74 + 0x14, 0x0000_0301);
        assert_eq!(e.call(0x0047_7950, &args![x, 0x100u32]).u32(), 0x101);
        assert_eq!(e.call(0x0047_7950, &args![x, 0x1u32]).u32(), 1);
        assert_eq!(e.call(0x0047_7950, &args![x, 0x4u32]).u32(), 0);
    }

    #[test]
    fn data_block_is_at_0x74() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        assert_eq!(e.call(0x004f_adc0, &args![x]).u32(), x.addr() + 0x74);
    }

    #[test]
    fn word_80_getter() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        e.mem.set_u32(x.addr() + 0x80, 0x1234);
        assert_eq!(e.call(0x004f_b070, &args![x]).u32(), 0x1234);
    }

    #[test]
    fn word_84_setter() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        e.call(0x004f_b090, &args![x, 0x55u32]);
        assert_eq!(e.mem.u32(x.addr() + 0x84), 0x55);
    }

    #[test]
    fn word_90_setter() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        e.call(0x004f_b0b0, &args![x, 0x66u32]);
        assert_eq!(e.mem.u32(x.addr() + 0x90), 0x66);
    }

    #[test]
    fn destructor_body_puts_the_vtables_back_and_destroys_the_components() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        quiet(
            &mut e,
            &[
                MODEL_DESTRUCTOR,
                FULL_NAME_DESTRUCTOR,
                BOUND_OBJECT_DESTRUCTOR,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x004f_ad20, &args![x]);
        let a = x.addr();
        for (offset, vtable) in [
            (0, VTABLE),
            (0x30, 0x0102_4a80),
            (0x3c, 0x0102_4a5c),
            (0x54, 0x0102_4a44),
            (0x64, 0x0102_4a2c),
            (0x68, 0x0102_4a18),
        ] {
            assert_eq!(e.mem.u32(a + offset), vtable);
        }
        let log: Calls = e.call_log.take().unwrap()[1..].to_vec();
        assert_eq!(
            log,
            vec![
                (MODEL_DESTRUCTOR, vec![a + 0x3c]),
                (FULL_NAME_DESTRUCTOR, vec![a + 0x30]),
                (BOUND_OBJECT_DESTRUCTOR, vec![a]),
            ]
        );
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        quiet(
            &mut e,
            &[
                MODEL_DESTRUCTOR,
                FULL_NAME_DESTRUCTOR,
                BOUND_OBJECT_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_acf0, &args![x, 2u32]).u32(), x.addr());
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, BOUND_OBJECT_DESTRUCTOR).len(), 1);
        e.call(0x004f_acf0, &args![x, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![x.addr()]]);
    }

    #[test]
    fn initialize_data_clears_the_placed_object_and_writes_the_defaults() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        quiet(&mut e, &[DATA_DEFAULTS]);
        e.set(x, BGSExplosion::pImpactPlacedObject, 9);
        e.call_log = Some(vec![]);
        e.call(0x004f_ade0, &args![x]);
        assert_eq!(e.get(x, BGSExplosion::pImpactPlacedObject), 0);
        assert_eq!(calls_to(&e, DATA_DEFAULTS), vec![vec![x.addr() + 0x74]]);
    }

    /// The double standing in for the form's virtual slot `0x130`.
    const EDITOR_ID_TARGET: u32 = 0x00dd_0130;

    /// Doubles for `InitItem`: ids resolve to `id + 0x1000` unless their low
    /// byte is `0xFF`.
    fn init_item_doubles(e: &mut Engine, initialized: bool) {
        accessor_doubles(e);
        returning(e, FORM_FLAG_8_TEST, initialized as u32);
        returning(e, FORM_GET_FILE, 0x7000);
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0200_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: if a[0] & 0xff == 0xff {
                0
            } else {
                a[0] + 0x1000
            },
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        returning(e, EDITOR_ID_TARGET, 0x00ab_cdef);
        quiet(
            e,
            &[
                SET_SOUND2,
                PRINT_MESSAGE,
                IMAGE_SPACE_MODIFIABLE_INIT_ITEM,
                ENCHANTABLE_INIT_ITEM,
                FORM_SET_FLAG_8,
            ],
        );
    }

    #[test]
    fn init_item_resolves_the_forms_and_logs_misses() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        let a = x.addr();
        give_vtable(&mut e, a, &[(0x130, EDITOR_ID_TARGET)]);
        init_item_doubles(&mut e, false);
        e.mem.set_u32(a + 0x80, 0x10);
        e.mem.set_u32(a + 0x84, 0x20);
        e.mem.set_u32(a + 0x94, 0);
        e.mem.set_u32(a + 0x90, 0x3ff);
        e.mem.set_u32(a + 0x70, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x004f_ae10, &args![x]);
        assert_eq!(e.mem.u32(a + 0x80), 0x0200_1010);
        assert_eq!(e.mem.u32(a + 0x84), 0x0200_1020);
        assert!(calls_to(&e, SET_SOUND2).is_empty());
        assert_eq!(e.mem.u32(a + 0x90), 0);
        assert_eq!(e.mem.u32(a + 0x70), 0x0200_1040);
        let casts: Vec<u32> = calls_to(&e, RT_DYNAMIC_CAST).iter().map(|c| c[3]).collect();
        assert_eq!(
            casts,
            vec![
                TYPE_TES_OBJECT_LIGH,
                TYPE_TES_SOUND,
                TYPE_BGS_IMPACT_DATA_SET,
                TYPE_TES_BOUND_OBJECT
            ]
        );
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![
                MESSAGE_NO_IMPACT_DATA_SET,
                0x0200_03ff,
                0x00ab_cdef,
                a | 0x0100_0000
            ]]
        );
        assert_eq!(
            calls_to(&e, IMAGE_SPACE_MODIFIABLE_INIT_ITEM),
            vec![vec![a + 0x68, a]]
        );
        assert_eq!(calls_to(&e, ENCHANTABLE_INIT_ITEM), vec![vec![a + 0x54, a]]);
        assert_eq!(calls_to(&e, FORM_SET_FLAG_8), vec![vec![a, 1]]);
    }

    #[test]
    fn init_item_logs_a_missing_placed_object_and_skips_initialized_forms() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        let a = x.addr();
        give_vtable(&mut e, a, &[(0x130, EDITOR_ID_TARGET)]);
        init_item_doubles(&mut e, false);
        e.mem.set_u32(a + 0x70, 0x4ff);
        e.call_log = Some(vec![]);
        e.call(0x004f_ae10, &args![x]);
        assert_eq!(e.mem.u32(a + 0x70), 0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![
                MESSAGE_NO_IMPACT_PLACED_OBJECT,
                0x0200_04ff,
                0x00ab_cdef,
                a | 0x0100_0000
            ]]
        );

        init_item_doubles(&mut e, true);
        e.mem.set_u32(a + 0x80, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x004f_ae10, &args![x]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(e.mem.u32(a + 0x80), 0x10);
    }

    /// A file double that hands out `chunks` one by one; word chunks read
    /// `0x500 + index`, 16-bit chunks `0x77`.
    fn file_doubles(e: &mut Engine, form_type: u32, chunks: Vec<u32>, swaps: bool) {
        let position = Rc::new(RefCell::new(0usize));
        let count_of = chunks.len();
        returning(e, FILE_GET_FORM_TYPE, form_type);
        let (p, list) = (position.clone(), chunks);
        e.register_double(FILE_GET_CHUNK, move |_, _| Ret {
            eax: list.get(*p.borrow()).copied().unwrap_or(0),
            ..Ret::default()
        });
        let (p, count) = (position.clone(), count_of);
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            *p.borrow_mut() += 1;
            Ret {
                eax: (*p.borrow() < count) as u32,
                ..Ret::default()
            }
        });
        let p = position;
        e.register_double(FILE_GET_CHUNK_WORD, move |e, a| {
            e.mem.set_u32(a[1], 0x500 + *p.borrow() as u32);
            Ret::default()
        });
        e.register(FILE_GET_CHUNK_SHORT, |e, a| {
            e.mem.set_u16(a[1], 0x77);
            Ret::default()
        });
        returning(e, FILE_CHUNK_SIZE, 8);
        returning(e, FILE_SWAPS, swaps as u32);
    }

    #[test]
    fn load_reads_every_chunk() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        let a = x.addr();
        give_vtable(
            &mut e,
            a,
            &[
                (0x18, 0x00dd_0018),
                (0xe0, 0x00dd_00e0),
                (0x134, 0x00dd_0134),
            ],
        );
        file_doubles(
            &mut e,
            0x251,
            vec![
                CHUNK_EDID,
                CHUNK_OBND,
                CHUNK_FULL,
                CHUNK_MODL,
                CHUNK_DATA,
                CHUNK_ENAM,
                CHUNK_EAMT,
                CHUNK_MNAM,
                CHUNK_INAM,
                CHUNK_MODT,
                tag(b"XXXX"),
            ],
            true,
        );
        quiet(
            &mut e,
            &[
                0x00dd_0018,
                0x00dd_00e0,
                0x00dd_0134,
                FORM_LOAD_FORM,
                FORM_SET_FLAG_8,
                FILE_GET_CHUNK_BYTES,
                FULL_NAME_LOAD,
                MODEL_LOAD_MODEL_CHUNK,
                FORM_LOAD_DATA,
                SET_WORD_4,
                ENCHANTABLE_SET_AMOUNT,
                DATA_ENDIAN,
            ],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_b0d0, &args![x, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, 0x00dd_0018), vec![vec![a]]);
        assert_eq!(calls_to(&e, FORM_SET_FLAG_8), vec![vec![a, 0]]);
        assert_eq!(calls_to(&e, 0x00dd_00e0), vec![vec![a, 0x9000]]);
        assert_eq!(calls_to(&e, 0x00dd_0134).len(), 1);
        assert_eq!(calls_to(&e, FILE_GET_CHUNK_BYTES)[0][2], 0x200);
        assert_eq!(calls_to(&e, FULL_NAME_LOAD), vec![vec![a + 0x30, 0x9000]]);
        assert_eq!(
            calls_to(&e, MODEL_LOAD_MODEL_CHUNK),
            vec![vec![a + 0x3c, 0x9000], vec![a + 0x3c, 0x9000]]
        );
        assert_eq!(
            calls_to(&e, FORM_LOAD_DATA),
            vec![vec![a, 0x9000, a + 0x74, 0x34]]
        );
        assert_eq!(calls_to(&e, DATA_ENDIAN), vec![vec![a + 0x74]]);
        assert_eq!(
            calls_to(&e, SET_WORD_4),
            vec![vec![a + 0x54, 0x505], vec![a + 0x68, 0x507]]
        );
        assert_eq!(
            calls_to(&e, ENCHANTABLE_SET_AMOUNT),
            vec![vec![a + 0x54, 0x77]]
        );
        assert_eq!(e.get(x, BGSExplosion::pImpactPlacedObject), 0x508);
    }

    #[test]
    fn load_refuses_another_form_type() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        file_doubles(&mut e, 0x52, vec![], false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_b0d0, &args![x, 0x9000u32]).bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn save_writes_ids_in_the_data_and_restores_the_pointers() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        let a = x.addr();
        accessor_doubles(&mut e);
        returning(&mut e, SWAP_ON_SAVE, 1);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(FORM_SAVE_DATA, move |e, a| {
            let data = a[1];
            sink.borrow_mut().extend([
                e.mem.u32(data + 0xc),
                e.mem.u32(data + 0x10),
                e.mem.u32(data + 0x20),
                e.mem.u32(data + 0x1c),
            ]);
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                FORM_START_FORM,
                FULL_NAME_SAVE,
                MODEL_SAVE,
                ENCHANTABLE_SAVE,
                IMAGE_SPACE_MODIFIABLE_SAVE,
                DATA_ENDIAN,
                ADD_CHUNK_WORD,
                FORM_CLOSE_FORM,
            ],
        );
        e.mem.set_u32(a + 0x80, 0x10);
        e.mem.set_u32(a + 0x84, 0x20);
        e.mem.set_u32(a + 0x94, 0);
        e.mem.set_u32(a + 0x90, 0x40);
        e.mem.set_u32(a + 0x70, 0x50);
        e.call_log = Some(vec![]);
        e.call(0x004f_b3b0, &args![x]);
        assert_eq!(
            seen.borrow().as_slice(),
            &[0x0100_0010, 0x0100_0020, 0, 0x0100_0040]
        );
        assert_eq!(e.mem.u32(a + 0x80), 0x10);
        assert_eq!(e.mem.u32(a + 0x84), 0x20);
        assert_eq!(e.mem.u32(a + 0x94), 0);
        assert_eq!(e.mem.u32(a + 0x90), 0x40);
        assert_eq!(calls_to(&e, DATA_ENDIAN).len(), 2);
        assert_eq!(
            calls_to(&e, MODEL_SAVE),
            vec![vec![a + 0x3c, CHUNK_MODL, CHUNK_MODT]]
        );
        assert_eq!(
            calls_to(&e, ADD_CHUNK_WORD),
            vec![vec![CHUNK_INAM, 0x0100_0050]]
        );
        assert_eq!(
            order_of(&e, &[FORM_START_FORM, FORM_SAVE_DATA, FORM_CLOSE_FORM]),
            vec![FORM_START_FORM, FORM_SAVE_DATA, FORM_CLOSE_FORM]
        );

        // No impact placed object: no `INAM`.
        e.mem.set_u32(a + 0x70, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_b3b0, &args![x]);
        assert!(calls_to(&e, ADD_CHUNK_WORD).is_empty());
    }

    /// `__RTDynamicCast` double: `0x1234` is not an explosion.
    fn cast_double(e: &mut Engine) {
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: if a[0] == 0x1234 { 0 } else { a[0] },
            ..Ret::default()
        });
    }

    #[test]
    fn copy_takes_the_components_data_and_placed_object() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        let other = explosion(&mut e);
        accessor_doubles(&mut e);
        cast_double(&mut e);
        quiet(&mut e, &[FORM_COPY_ALL_COMPONENTS, MEMCPY]);
        e.set(other, BGSExplosion::pImpactPlacedObject, 0x88);
        e.call_log = Some(vec![]);
        e.call(0x004f_b5a0, &args![x, 0x1234u32]);
        assert!(calls_to(&e, FORM_COPY_ALL_COMPONENTS).is_empty());
        e.call(0x004f_b5a0, &args![x, other]);
        assert_eq!(
            calls_to(&e, FORM_COPY_ALL_COMPONENTS),
            vec![vec![x.addr(), other.addr()]]
        );
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![x.addr() + 0x74, other.addr() + 0x74, 0x34]]
        );
        assert_eq!(e.get(x, BGSExplosion::pImpactPlacedObject), 0x88);
    }

    #[test]
    fn compare_answers_true_on_any_difference() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        let other = explosion(&mut e);
        accessor_doubles(&mut e);
        cast_double(&mut e);
        returning(&mut e, FORM_COMPARE_ALL_COMPONENTS, 0);
        returning(&mut e, MEMCMP, 0);
        assert!(e.call(0x004f_b610, &args![x, 0x1234u32]).bool());
        assert!(!e.call(0x004f_b610, &args![x, other]).bool());
        e.set(other, BGSExplosion::pImpactPlacedObject, 1);
        assert!(e.call(0x004f_b610, &args![x, other]).bool());
        e.set(other, BGSExplosion::pImpactPlacedObject, 0);
        returning(&mut e, MEMCMP, 0xffff_ffff);
        assert!(e.call(0x004f_b610, &args![x, other]).bool());
        returning(&mut e, MEMCMP, 0);
        returning(&mut e, FORM_COMPARE_ALL_COMPONENTS, 1);
        assert!(e.call(0x004f_b610, &args![x, other]).bool());
    }

    /// Maps the model loader and shared form globals and sets them.
    fn model_globals(e: &mut Engine, shared: u32) {
        e.map(MODEL_LOADER & !0xfff, 0x1000);
        e.map(SHARED_MODEL_FORM & !0xfff, 0x1000);
        e.set_global(MODEL_LOADER, 0x4444u32);
        e.set_global(SHARED_MODEL_FORM, shared);
    }

    #[test]
    fn queue_files_queues_the_models_and_the_impact_data_set() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        let a = x.addr();
        let shared = e.mem.alloc(0x100);
        let set = e.mem.alloc(0x100);
        model_globals(&mut e, shared);
        e.mem.set_u32(a + 0x90, set);
        give_vtable(&mut e, set + 0x18, &[(0x10, 0x00dd_0010)]);
        quiet(&mut e, &[0x00dd_0010, QUEUE_MODEL]);
        returning(&mut e, MODEL_PATH_LENGTH, 3);
        returning(&mut e, FORM_LOD_MULT, 2);
        e.call_log = Some(vec![]);
        e.call(0x004f_b6a0, &args![a + 0x64, 7u32, 8u32]);
        assert_eq!(
            calls_to(&e, QUEUE_MODEL),
            vec![
                vec![0x4444, a + 0x3c, 7, 8, 2, 1, 0, 0],
                vec![0x4444, shared + 0x3c, 7, 8, 2, 1, 0, 0]
            ]
        );
        assert_eq!(calls_to(&e, FORM_LOD_MULT), vec![vec![a], vec![a]]);
        assert_eq!(calls_to(&e, 0x00dd_0010), vec![vec![set + 0x18, 7, 8]]);

        // No paths, no set, no shared form: nothing is queued.
        returning(&mut e, MODEL_PATH_LENGTH, 0);
        e.mem.set_u32(a + 0x90, 0);
        e.set_global(SHARED_MODEL_FORM, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x004f_b6a0, &args![a + 0x64, 7u32, 8u32]);
        assert!(calls_to(&e, QUEUE_MODEL).is_empty());
        assert!(calls_to(&e, 0x00dd_0010).is_empty());
    }

    #[test]
    fn impact_entry_for_material() {
        let mut e = Engine::new();
        returning(&mut e, IMPACT_INDEX_FOR_MATERIAL, 5);
        returning(&mut e, IMPACT_DATA_SET_GET, 0x99);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_b7a0, &args![0x3000u32, 0x13u32]).u32(), 0x99);
        assert_eq!(calls_to(&e, IMPACT_INDEX_FOR_MATERIAL), vec![vec![0x13]]);
        assert_eq!(calls_to(&e, IMPACT_DATA_SET_GET), vec![vec![0x3000, 5]]);
    }

    #[test]
    fn impact_entry_of_the_explosion_needs_a_set() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        accessor_doubles(&mut e);
        returning(&mut e, IMPACT_INDEX_FOR_MATERIAL, 5);
        returning(&mut e, IMPACT_DATA_SET_GET, 0x99);
        assert_eq!(e.call(0x004f_b7d0, &args![x, 1u32]).u32(), 0);
        e.mem.set_u32(x.addr() + 0x90, 0x3000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_b7d0, &args![x, 1u32]).u32(), 0x99);
        assert_eq!(calls_to(&e, IMPACT_DATA_SET_GET), vec![vec![0x3000, 5]]);
    }

    #[test]
    fn sound_level_value_reads_the_setting_for_the_level() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        let values = e.mem.alloc(0x10);
        e.mem.set_u32(values, 10);
        e.mem.set_u32(values + 4, 11);
        e.mem.set_u32(values + 8, 12);
        e.register_double(SETTING_VALUE, move |_, a| Ret {
            eax: match a[0] {
                SOUND_LEVEL_0_SETTING => values,
                SOUND_LEVEL_1_SETTING => values + 4,
                SOUND_LEVEL_2_SETTING => values + 8,
                _ => 0,
            },
            ..Ret::default()
        });
        for (level, value) in [(0u32, 10u32), (1, 11), (2, 12), (3, 0)] {
            e.mem.set_u32(x.addr() + 0xa4, level);
            assert_eq!(e.call(0x004f_b810, &args![x]).u32(), value);
        }
    }

    #[test]
    fn clone3d_uses_the_bound_object_or_the_shared_model() {
        let mut e = Engine::new();
        let x = explosion(&mut e);
        let shared = e.mem.alloc(0x100);
        model_globals(&mut e, shared);
        give_vtable(&mut e, shared + 0x3c, &[(0x14, 0x00dd_0014)]);
        returning(&mut e, 0x00dd_0014, 0x6000);
        returning(&mut e, BOUND_OBJECT_CLONE3D, 0x51);
        returning(&mut e, NI_OBJECT_CLONE, 0x52);
        quiet(
            &mut e,
            &[CLONING_PROCESS_CONSTRUCTOR, CLONING_PROCESS_DESTRUCTOR],
        );
        returning(&mut e, REFERENCE_BYTE_D9, 0x100);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004f_b880, &args![x, 0x7000u32, 0x1ffu32]).u32(),
            0x51
        );
        assert_eq!(
            calls_to(&e, BOUND_OBJECT_CLONE3D),
            vec![vec![x.addr(), 0x7000, 0xff]]
        );

        returning(&mut e, REFERENCE_BYTE_D9, 1);
        returning(&mut e, MODEL_LOADER_LOAD_FILE, 0x8000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_b880, &args![x, 0x7000u32, 0u32]).u32(), 0x52);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_LOAD_FILE),
            vec![vec![0x4444, 0x6000, 0, 1, 0, 0, 0]]
        );
        let process = calls_to(&e, CLONING_PROCESS_CONSTRUCTOR)[0].clone();
        assert_eq!(process[1], 1.0f32.to_bits());
        assert_eq!(
            calls_to(&e, NI_OBJECT_CLONE),
            vec![vec![0x8000, process[0]]]
        );
        assert_eq!(
            calls_to(&e, CLONING_PROCESS_DESTRUCTOR),
            vec![vec![process[0]]]
        );

        returning(&mut e, MODEL_LOADER_LOAD_FILE, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_b880, &args![x, 0x7000u32, 0u32]).u32(), 0);
        assert!(calls_to(&e, NI_OBJECT_CLONE).is_empty());
        assert_eq!(calls_to(&e, CLONING_PROCESS_DESTRUCTOR).len(), 1);
    }
}

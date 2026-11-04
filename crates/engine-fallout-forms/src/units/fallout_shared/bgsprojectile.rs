//! `fallout shared/bgsprojectile.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSProjectile` (form type `0x33`, record `PROJ`) is 0xD0 bytes on the PC.
//! Its vtable is at `010251ac`; four more vtables for its component bases are
//! installed by the constructor (`TESFullName` at `+0x30`: `01025198`,
//! `TESModel` at `+0x3C`: `01025174`, `BGSPreloadable` at `+0x54`:
//! `0102515c`, `BGSDestructibleObjectForm` at `+0x58`: `01025148`; the RTTI
//! locator before each names `BGSProjectile`). The record's `DATA` chunk
//! (`BGSProjectileData`, 0x54 bytes) sits at `+0x60`, the muzzle flash
//! `TESModel` at `+0xB4` and the sound level at `+0xCC`.
//!
//! This unit holds the constructor and destructor, `InitializeData`/
//! `ClearData`, `InitItem`, `Clone3D`, `Load`, `Save`, `Copy` and `Compare`
//! (vtable slots `0x10`, `0x14`, `0x18`, `0x88`, `0x14C`, `0x20`, `0x2C`,
//! `0x108`, `0x10C`), the `BGSPreloadable` override at slot `0x10` of the
//! component vtable, the byte swap of the data block and small helpers.
//! Functions that run under a compiler exception frame or the stack cookie
//! carry a `Translation incomplete` marker (the unwinding and the cookie
//! check are not translated).

#[allow(unused_imports)]
use crate::prelude::*;

// ---- Layout --------------------------------------------------------------------------

layout! {
    /// `BGSProjectileData` (Xbox PDB), 0x54 bytes: the record's `DATA` chunk.
    /// Offsets are relative to the block; in the projectile it starts at
    /// `+0x60`.
    pub struct BGSProjectileData: 0x54 {
        /// `iFlags` (Xbox PDB). Bits `0x1F0000` are the projectile type
        /// (`0x20000`, `0x40000`, `0x80000` and `0x100000` are tested by the
        /// code here); `0x20`, `0x40`, `0x100` and `0x2` are tested too.
        0x00 iFlags: u32,
        /// `fGravity` (Xbox PDB).
        0x04 fGravity: f32,
        /// `fSpeed` (Xbox PDB).
        0x08 fSpeed: f32,
        /// `fRange` (Xbox PDB).
        0x0C fRange: f32,
        /// `pLight` (Xbox PDB, `TESObjectLIGH*`; a form id in the record).
        0x10 pLight: u32,
        /// `pMuzzleFlashLight` (Xbox PDB, `TESObjectLIGH*`).
        0x14 pMuzzleFlashLight: u32,
        /// `fTracerChance` (Xbox PDB).
        0x18 fTracerChance: f32,
        /// `fExplosionProximity` (Xbox PDB).
        0x1C fExplosionProximity: f32,
        /// `fExplosionTimer` (Xbox PDB).
        0x20 fExplosionTimer: f32,
        /// `pExplosionType` (Xbox PDB, `BGSExplosion*`).
        0x24 pExplosionType: u32,
        /// `pActiveSoundLoop` (Xbox PDB, `TESSound*`).
        0x28 pActiveSoundLoop: u32,
        /// `fMuzzleFlashDuration` (Xbox PDB).
        0x2C fMuzzleFlashDuration: f32,
        /// `fFadeOutTime` (Xbox PDB).
        0x30 fFadeOutTime: f32,
        /// `fForce` (Xbox PDB).
        0x34 fForce: f32,
        /// `pCountdownSound` (Xbox PDB, `TESSound*`).
        0x38 pCountdownSound: u32,
        /// `pDeactivateSound` (Xbox PDB, `TESSound*`).
        0x3C pDeactivateSound: u32,
        /// `pDefaultWeaponSource` (Xbox PDB, `TESObjectWEAP*`).
        0x40 pDefaultWeaponSource: u32,
        /// `fRotationX` (Xbox PDB).
        0x44 fRotationX: f32,
        /// `fRotationY` (Xbox PDB).
        0x48 fRotationY: f32,
        /// `fRotationZ` (Xbox PDB).
        0x4C fRotationZ: f32,
        /// `fBounceMultiplier` (Xbox PDB).
        0x50 fBounceMultiplier: f32,
    }
}

layout! {
    /// `BGSProjectile` (Xbox PDB), 0xD0 bytes on the PC (0xE0 on the Xbox:
    /// its `TESForm` is 0x10 bytes larger, so every Xbox offset past the form
    /// is 0x10 higher). Only the parts this unit uses are named.
    pub struct BGSProjectile: 0xD0 {
        /// `BGSProjectileData Data` (Xbox PDB `+0x70`).
        0x60 Data: Inline<BGSProjectileData>,
        /// `eSoundLevel` (Xbox PDB `SOUND_LEVEL`).
        0xCC eSoundLevel: u32,
    }
}

/// Offsets of the component bases and of the data block in the projectile.
const FULL_NAME: u32 = 0x30;
const MODEL: u32 = 0x3C;
const PRELOADABLE: u32 = 0x54;
const DESTRUCTIBLE: u32 = 0x58;
const DATA: u32 = 0x60;
const MUZZLE_FLASH_MODEL: u32 = 0xB4;
const SOUND_LEVEL: u32 = 0xCC;
/// Size of the `DATA` chunk.
const DATA_SIZE: u32 = 0x54;

/// The five vtables a projectile installs: its own and the four component
/// bases'.
const VTABLE: u32 = 0x0102_51ac;
const FULL_NAME_VTABLE: u32 = 0x0102_5198;
const MODEL_VTABLE: u32 = 0x0102_5174;
const PRELOADABLE_VTABLE: u32 = 0x0102_515c;
const DESTRUCTIBLE_VTABLE: u32 = 0x0102_5148;

/// The form type byte of a projectile record (`0x33`).
const FORM_TYPE_PROJECTILE: u32 = 0x33;

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
const CHUNK_DEST: u32 = tag(b"DEST");
const CHUNK_DSTD: u32 = tag(b"DSTD");
const CHUNK_NAM1: u32 = tag(b"NAM1");
const CHUNK_NAM2: u32 = tag(b"NAM2");
const CHUNK_VNAM: u32 = tag(b"VNAM");

/// Type bits of `Data.iFlags` (`0x1F0000`) the code here tests.
const TYPE_0X20000: u32 = 0x2_0000;
const TYPE_0X40000: u32 = 0x4_0000;
const TYPE_0X80000: u32 = 0x8_0000;
const TYPE_0X100000: u32 = 0x10_0000;
/// Other tested bits of `Data.iFlags`.
const FLAG_0X20: u32 = 0x20;
const FLAG_0X40: u32 = 0x40;
const FLAG_0X100: u32 = 0x100;

// ---- Globals --------------------------------------------------------------------------

/// The data handler (`TESDataHandler*`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// An object whose flag word at `+0x244` bit 1 is tested by `0042ce10`
/// during `Clone3D` (its identity is not confirmed).
const CLONE_GATE_OBJECT: u32 = 0x011d_df38;
/// The object `BGSPreloadable::QueueFiles` hands the models to (called with
/// `00443d30`; `modelloader.cpp` range, identity not confirmed).
const MODEL_QUEUE_OBJECT: u32 = 0x011c_3b3c;
/// `100.0` (`double`, `01017a40`): the tracer roll multiplies the chance by it.
const TRACER_CHANCE_SCALE: u32 = 0x0101_7a40;

// ---- Calls outside this file ---------------------------------------------------------

/// `TESBoundObject::TESBoundObject` (Xbox PDB), `thiscall`.
const BOUND_OBJECT_CONSTRUCTOR: u32 = 0x0050_e820;
/// `TESBoundObject` destructor body (`thiscall`, `0050e9f0`).
const BOUND_OBJECT_DESTRUCTOR: u32 = 0x0050_e9f0;
/// Constructor of the `TESFullName` component (`thiscall`, `00402d00`) and
/// its destructor (`00591300`).
const FULL_NAME_CONSTRUCTOR: u32 = 0x0040_2d00;
const FULL_NAME_DESTRUCTOR: u32 = 0x0059_1300;
/// Constructor of `TESModel` (`thiscall`, `00488f50`) and its destructor
/// (`00489070`).
const MODEL_CONSTRUCTOR: u32 = 0x0048_8f50;
const MODEL_DESTRUCTOR: u32 = 0x0048_9070;
/// Constructor of `BGSPreloadable` (`thiscall`, `004f9d90`).
const PRELOADABLE_CONSTRUCTOR: u32 = 0x004f_9d90;
/// Constructor (`00475350`) and destructor (`00475380`) of
/// `BGSDestructibleObjectForm` (`thiscall`).
const DESTRUCTIBLE_CONSTRUCTOR: u32 = 0x0047_5350;
const DESTRUCTIBLE_DESTRUCTOR: u32 = 0x0047_5380;
/// Stores the byte argument at `+0x04` of the form (`TESForm::cFormType` in
/// the Xbox PDB), `thiscall`, `RET 4` (no name in the engine map).
const FORM_SET_FORM_TYPE: u32 = 0x004f_15a0;
/// Writes the `BGSProjectileData` defaults (flags `0x10000`, zeroes, the
/// speed and range constants, bounce multiplier 1.0), `thiscall` on the
/// data block (`004fcc20`; no name in the engine map).
const DATA_DEFAULTS: u32 = 0x004f_cc20;
/// Returns the address of the data block (`this + 0x60`), `thiscall`
/// (`004ea950`; the linker folded this body under another class's name).
const DATA_POINTER: u32 = 0x004e_a950;
/// `Data.iFlags & 0x1F0000`, `thiscall` (`004fd4f0`).
const PROJECTILE_TYPE: u32 = 0x004f_d4f0;
/// `Data.iFlags & argument != 0` (`AL`), `thiscall`, `RET 4` (`004fd420`).
const FLAGS_TEST: u32 = 0x004f_d420;
/// `Data.iFlags & 2 != 0` (`AL`), `thiscall` (`004fd360`).
const FLAG_0X2_TEST: u32 = 0x004f_d360;
/// Getters of the form pointers in the data block, `thiscall`: `pLight`
/// (`+0x70`), `pMuzzleFlashLight` (`+0x74`), `pExplosionType` (`+0x84`),
/// `pActiveSoundLoop` (`+0x88`), `pCountdownSound` (`+0x98`),
/// `pDeactivateSound` (`+0x9C`), `pDefaultWeaponSource` (`+0xA0`).
const GET_LIGHT: u32 = 0x004f_d380;
const GET_MUZZLE_FLASH_LIGHT: u32 = 0x004f_d3a0;
const GET_EXPLOSION: u32 = 0x004f_d3c0;
const GET_ACTIVE_SOUND_LOOP: u32 = 0x004f_d3e0;
const GET_COUNTDOWN_SOUND: u32 = 0x0081_0fd0;
const GET_DEACTIVATE_SOUND: u32 = 0x0058_6150;
const GET_DEFAULT_WEAPON_SOURCE: u32 = 0x004f_d400;
/// The matching setters, `thiscall` (projectile, form), `RET 4`.
const SET_LIGHT: u32 = 0x0057_bd60;
const SET_MUZZLE_FLASH_LIGHT: u32 = 0x004f_ebb0;
const SET_EXPLOSION: u32 = 0x004f_b090;
const SET_ACTIVE_SOUND_LOOP: u32 = 0x0085_7230;
const SET_COUNTDOWN_SOUND: u32 = 0x0040_78a0;
const SET_DEACTIVATE_SOUND: u32 = 0x0081_0570;
const SET_DEFAULT_WEAPON_SOURCE: u32 = 0x0050_0a20;
/// `BGSDestructibleObjectForm::InitItem` (Xbox PDB), `thiscall` (component,
/// form).
const DESTRUCTIBLE_INIT_ITEM: u32 = 0x0047_8be0;
/// `BGSDestructibleObjectForm::Save` (Xbox PDB), `thiscall` (component).
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
/// `BGSDestructibleObjectForm::LoadChunk` (Xbox PDB), `cdecl` (component,
/// file).
const DESTRUCTIBLE_LOAD_CHUNK: u32 = 0x0047_81e0;
/// `TESFullName::Save` (Xbox PDB), `thiscall` (component).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
/// The `TESFullName` loader (`cdecl`, component, file; `00487050`).
const FULL_NAME_LOAD: u32 = 0x0048_7050;
/// `TESModel::Save` (Xbox PDB), `thiscall` (model, model chunk, texture
/// chunk).
const MODEL_SAVE: u32 = 0x0048_91d0;
/// `TESModel::LoadModelChunk` (Xbox PDB), `cdecl` (model, file).
const MODEL_LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESModel::LoadModelTextureChunk` (Xbox PDB), `cdecl` (model, file).
const MODEL_LOAD_TEXTURE_CHUNK: u32 = 0x0048_93e0;
/// The length of a model's path (`thiscall` on the `TESModel`; the string at
/// `+4`).
const MODEL_PATH_LENGTH: u32 = 0x0048_cee0;
/// `TESModel` virtual slots: `InitializeDataComponent`, `ClearDataComponent`,
/// `CopyComponent` and `CompareComponent` (Xbox PDB), then the path getter
/// (`0x14`) and setter (`0x18`) used by `Save` and `Load`.
const MODEL_INITIALIZE_SLOT: u32 = 0x00;
const MODEL_CLEAR_SLOT: u32 = 0x04;
const MODEL_COPY_SLOT: u32 = 0x08;
const MODEL_COMPARE_SLOT: u32 = 0x0c;
const MODEL_GET_PATH_SLOT: u32 = 0x14;
const MODEL_SET_PATH_SLOT: u32 = 0x18;
/// `TESForm::LoadForm` (Xbox PDB), `thiscall` (form, file).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESForm::LoadData` (Xbox PDB), `thiscall` (form, file, buffer, size).
const FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `TESForm::__SaveData` (Xbox PDB), `thiscall` (form, buffer, size).
const FORM_SAVE_DATA: u32 = 0x0048_5e20;
/// `TESForm::StartForm` and `TESForm::CloseForm` (Xbox PDB), `thiscall`.
const FORM_START_FORM: u32 = 0x0048_55a0;
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// A chunk holding one word (`cdecl`, chunk, value), and
/// `TESForm::AddChunkArray` (`cdecl`, chunk, data, size).
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// Save-side byte-swap switch (`cdecl`, no arguments, `AL`).
const SWAP_ON_SAVE: u32 = 0x0040_1500;
/// Swaps the byte order of the word at the pointer (`cdecl`, pointer and an
/// unused second word; `00401080`).
const SWAP_WORD: u32 = 0x0040_1080;
/// `iFormID` getter (`thiscall`; the engine map names the folded body
/// `0084e3a0`).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// `TESForm` "initialized" bit setter (bit 3 of `iFormFlags`), `thiscall`
/// (form, value), and its test (`AL`).
const FORM_SET_FLAG_8: u32 = 0x0048_4ab0;
const FORM_FLAG_8_TEST: u32 = 0x0040_13e0;
/// `TESForm::GetFile` (Xbox PDB), `thiscall` (form, `-1`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB), `cdecl` (pointer to the form id,
/// file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Form lookup by id (`cdecl`, one word).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESForm::CopyAllComponents` and `TESForm::CompareAllComponents` (Xbox
/// PDB), `thiscall` (form, other form).
const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `memcpy` wrapper (`cdecl`: destination, source, count; `00401460`) and
/// `memcmp` (`_memcmp`, `00ec4835`).
const MEMCPY: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
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
/// The size of the current chunk, `thiscall` (file; `+0x25C`).
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// Whether the file's data are byte-swapped (`AL`, `thiscall`; `+0x299`).
const FILE_SWAPS: u32 = 0x0040_1680;
/// `TESBoundObject::Clone3D` (Xbox PDB), `thiscall` (object, reference,
/// flag).
const BOUND_OBJECT_CLONE3D: u32 = 0x0050_ee50;
/// A reference's word at `+0x144` (`009d9f00`) and the word at `+8` of an
/// object (`0044ddc0`), both `thiscall`.
const GET_WORD_144: u32 = 0x009d_9f00;
const GET_WORD_8: u32 = 0x0044_ddc0;
/// `NiCloningProcess` constructor (`thiscall`, process, scale; `004ad050`)
/// and `NiCloningProcess::~NiCloningProcess` (Xbox PDB), and
/// `NiObject::Clone` (Xbox PDB, `thiscall`, object, process).
const CLONING_PROCESS_CONSTRUCTOR: u32 = 0x004a_d050;
const CLONING_PROCESS_DESTRUCTOR: u32 = 0x004a_d270;
const NI_OBJECT_CLONE: u32 = 0x00a5_d2c0;
/// `NiCloningProcess` is 0x1C bytes on the stack of `Clone3D`.
const CLONING_PROCESS_SIZE: u32 = 0x1c;
/// `thiscall` on the data handler (`00469860`): true when the form id is
/// `0xFF000000` or above.
const ID_AT_OR_ABOVE_DYNAMIC: u32 = 0x0046_9860;
/// `thiscall` on `CLONE_GATE_OBJECT`: its flag word `+0x244` bit 1.
const CLONE_GATE_TEST: u32 = 0x0042_ce10;
/// Returns the singleton at `011c5f40`, creating it first when the byte
/// argument is set (`cdecl`, `004b0e20`); and the lookup in it (`thiscall`
/// on the singleton, `RET 4`: the object; `004b0ba0`).
const CLONE_POOL_GET: u32 = 0x004b_0e20;
const CLONE_POOL_LOOKUP: u32 = 0x004b_0ba0;
/// `thiscall` (object, reference): undoes what `Clone3D` set up on the
/// object when the pooled copy was not used (`0050fbf0`).
const CLONE_FALLBACK_CLEANUP: u32 = 0x0050_fbf0;
/// `thiscall`, `RET 4` (node, flag; `0049ee60`).
const NODE_SET_FLAG: u32 = 0x0049_ee60;
/// A reference's base form (`004181e0`, reads `+0x20`).
const GET_BASE_FORM: u32 = 0x0041_81e0;
/// `thiscall` (reference, mask): `[reference + 0xC8] & mask != 0`
/// (`00444d00`, `RET 4`).
const REFERENCE_FLAGS_TEST: u32 = 0x0044_4d00;
/// `thiscall` (projectile base, reference, byte, word, word), `RET 0x14`
/// (`0050f670`).
const BOUND_OBJECT_APPLY: u32 = 0x0050_f670;
/// `TES::GetLODMult` (Xbox PDB, but the body takes a form and answers
/// 1 to 3 from its form type): `cdecl`, one word (`0045c6b0`).
const FORM_LOD_MULT: u32 = 0x0045_c6b0;
/// Queues a model for loading (`thiscall` on `MODEL_QUEUE_OBJECT`: model,
/// two words, a multiplier and three flags; `RET 0x1C`; `00443d30`).
const QUEUE_MODEL: u32 = 0x0044_3d30;
/// The float a weapon holds at `+0x134` (`ST0`, `thiscall`; `00821640`).
const WEAPON_FLOAT_134: u32 = 0x0082_1640;
/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 to an integer in
/// EAX; the value is passed as an `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// `TESObjectWEAP::GetNumProjectiles` (Xbox PDB), `thiscall` (weapon,
/// with-mod, skip-ammo, actor).
const WEAPON_GET_NUM_PROJECTILES: u32 = 0x0052_5b20;
/// `min(a, b)` of two signed integers (`cdecl`; `004a8f20`).
const MIN_INT: u32 = 0x004a_8f20;
/// `TESObjectWEAP::GetModEffectValue` (Xbox PDB), `thiscall` (mod effect, 0),
/// result in `ST0`.
const WEAPON_GET_MOD_EFFECT_VALUE: u32 = 0x004b_cf60;
/// `BGSClipRoundsForm::cClipRounds` getter (`thiscall`, the byte at `+4`).
const CLIP_ROUNDS_GET: u32 = 0x0040_1170;
/// The explosion's float at `+0x78` (`ST0`, `thiscall`; `006a78f0`; the
/// engine map names a navmesh function because the code is shared); for a
/// projectile it is `fTracerChance` (data `+0x18`).
const GET_FLOAT_78: u32 = 0x006a_78f0;
/// Returns a random number (`cdecl`, no arguments; `00487f50`).
const RANDOM: u32 = 0x0048_7f50;
/// The explosion's `sound level`-like word at `+0xCC` (`009ad610`) of a
/// projectile: `eSoundLevel`.
const GET_WORD_CC: u32 = 0x009a_d610;

/// Virtual slot `0x134` of a form: the editor id setter `Load` calls for the
/// `EDID` chunk.
const FORM_SET_EDITOR_ID_SLOT: u32 = 0x134;
/// Virtual slot `0xE0` of a form (`LoadObjectBound` in the Xbox PDB).
const FORM_LOAD_OBJECT_BOUND_SLOT: u32 = 0xe0;
/// Virtual slot `0x18` of a form (`ClearData` in the Xbox PDB).
const FORM_CLEAR_DATA_SLOT: u32 = 0x18;
/// Slot `0x174` of the projectile's vtable (`0050fe50`, the model loader for
/// a reference), `thiscall`.
const LOAD_3D_SLOT: u32 = 0x174;
/// Slot `0x320` of a `Projectile` reference's vtable, `thiscall` (actor,
/// `0`), `AL`.
const PROJECTILE_HIT_SLOT: u32 = 0x320;
/// Slot `0x10` of an `NiObject` (returns a pointer, none of the arguments are
/// read).
const NI_OBJECT_SLOT_10: u32 = 0x10;
/// Slot `0x10` of the `BGSPreloadable` component (`QueueFiles` in the Xbox
/// PDB), `thiscall` (two words).
const PRELOADABLE_QUEUE_FILES_SLOT: u32 = 0x10;

/// Type descriptors: `TESForm`, `TESObjectLIGH`, `BGSExplosion`, `TESSound`,
/// `TESObjectWEAP`, `BGSProjectile`, `TESObjectREFR`, `Actor` and
/// `Projectile`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_LIGH: u32 = 0x0118_3938;
const TYPE_BGS_EXPLOSION: u32 = 0x0118_620c;
const TYPE_TES_SOUND: u32 = 0x0118_32fc;
const TYPE_TES_OBJECT_WEAP: u32 = 0x0118_3998;
const TYPE_BGS_PROJECTILE: u32 = 0x0118_6228;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_ACTOR: u32 = 0x0118_46d4;
const TYPE_PROJECTILE: u32 = 0x0118_9dbc;

// ---- Data block helpers ---------------------------------------------------------------

/// The data block of a projectile.
fn data_block(this: Ptr<BGSProjectile>) -> Ptr<BGSProjectileData> {
    this.at(BGSProjectile::Data)
}

// Translated from 004fcc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the `BGSProjectileData` block (`this`): writes the defaults
/// (`004fcc20`) and returns `this`.
pub fn fn_004fcc00(e: &mut Engine, this: Ptr<BGSProjectileData>) -> Ptr<BGSProjectileData> {
    e.call(DATA_DEFAULTS, &args![this]);
    this
}

// Translated from 004fccf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSProjectileData::Endian` (Xbox PDB): swaps the byte order of the 14
/// 4-byte numbers of the data block (offsets 0, 4, 8, 0xC, 0x18, 0x1C, 0x20,
/// 0x2C, 0x30, 0x34, 0x44, 0x48, 0x4C and 0x50; the form ids and `iFlags`
/// ... are the others) in place.
pub fn bgs_projectile_data_endian(e: &mut Engine, this: Ptr<BGSProjectileData>) {
    for offset in [
        0x00u32, 0x04, 0x08, 0x0c, 0x18, 0x1c, 0x20, 0x2c, 0x30, 0x34, 0x44, 0x48, 0x4c, 0x50,
    ] {
        e.call(SWAP_WORD, &args![this.addr() + offset, 0u32]);
    }
}

// ---- Constructor, destructor ----------------------------------------------------------

// Translated from 004fce60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSProjectile` constructor (the vtables it installs are
/// `BGSProjectile`'s by their RTTI; no name in the engine map): constructs
/// the `TESBoundObject` base and the components (`TESFullName`, `TESModel`,
/// `BGSPreloadable`, `BGSDestructibleObjectForm`), installs the five
/// vtables, constructs the data block and the muzzle flash model, sets the
/// form type to `0x33` and runs `InitializeData`.
// Translation incomplete: 004fce60 - the C++ exception unwinding frame (FS:[0] handler chain that destroys the constructed bases if a later step throws) is not translated.
pub fn fn_004fce60(e: &mut Engine, this: Ptr<BGSProjectile>) -> Ptr<BGSProjectile> {
    let object = this.addr();
    e.call(BOUND_OBJECT_CONSTRUCTOR, &args![this]);
    e.call(FULL_NAME_CONSTRUCTOR, &args![object + FULL_NAME]);
    e.call(MODEL_CONSTRUCTOR, &args![object + MODEL]);
    e.call(PRELOADABLE_CONSTRUCTOR, &args![object + PRELOADABLE]);
    e.call(DESTRUCTIBLE_CONSTRUCTOR, &args![object + DESTRUCTIBLE]);
    e.mem.set_u32(object, VTABLE);
    e.mem.set_u32(object + FULL_NAME, FULL_NAME_VTABLE);
    e.mem.set_u32(object + MODEL, MODEL_VTABLE);
    e.mem.set_u32(object + PRELOADABLE, PRELOADABLE_VTABLE);
    e.mem.set_u32(object + DESTRUCTIBLE, DESTRUCTIBLE_VTABLE);
    fn_004fcc00(e, data_block(this));
    e.call(MODEL_CONSTRUCTOR, &args![object + MUZZLE_FLASH_MODEL]);
    e.call(FORM_SET_FORM_TYPE, &args![this, FORM_TYPE_PROJECTILE]);
    fn_004fd050(e, this);
    this
}

// Translated from 004fcf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (vtable slot `0x10`, `__vecDelDtor` in the
/// Xbox PDB): runs the destructor body and, when bit 0 of `flags` is set,
/// frees the object. Returns `this`.
pub fn fn_004fcf50(e: &mut Engine, this: Ptr<BGSProjectile>, flags: u32) -> Ptr<BGSProjectile> {
    fn_004fcf80(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// Release called by the deleting destructor (`cdecl`, pointer; `00401030`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 004fcf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body: puts the five vtables back, clears the data
/// (`ClearData`), destroys the muzzle flash model, the destructible
/// component, the model and the full name component, and runs the
/// `TESBoundObject` destructor.
// Translation incomplete: 004fcf80 - the C++ exception unwinding frame (FS:[0] state table that tracks which members are destroyed) is not translated.
pub fn fn_004fcf80(e: &mut Engine, this: Ptr<BGSProjectile>) {
    let object = this.addr();
    e.mem.set_u32(object, VTABLE);
    e.mem.set_u32(object + FULL_NAME, FULL_NAME_VTABLE);
    e.mem.set_u32(object + MODEL, MODEL_VTABLE);
    e.mem.set_u32(object + PRELOADABLE, PRELOADABLE_VTABLE);
    e.mem.set_u32(object + DESTRUCTIBLE, DESTRUCTIBLE_VTABLE);
    fn_004fd090(e, this);
    e.call(MODEL_DESTRUCTOR, &args![object + MUZZLE_FLASH_MODEL]);
    e.call(DESTRUCTIBLE_DESTRUCTOR, &args![object + DESTRUCTIBLE]);
    e.call(MODEL_DESTRUCTOR, &args![object + MODEL]);
    e.call(FULL_NAME_DESTRUCTOR, &args![object + FULL_NAME]);
    e.call(BOUND_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 004fd050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x14` (`InitializeData` in the Xbox PDB): writes the data
/// defaults, initializes the muzzle flash model (its virtual slot 0,
/// `InitializeDataComponent`) and sets the sound level to 1.
pub fn fn_004fd050(e: &mut Engine, this: Ptr<BGSProjectile>) {
    let data = e.call(DATA_POINTER, &args![this]).u32();
    e.call(DATA_DEFAULTS, &args![data]);
    e.vcall(this.addr() + MUZZLE_FLASH_MODEL, MODEL_INITIALIZE_SLOT, &[]);
    e.set(this, BGSProjectile::eSoundLevel, 1);
}

// Translated from 004fd090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x18` (`ClearData` in the Xbox PDB): clears the muzzle flash
/// model (its virtual slot 1, `ClearDataComponent`).
pub fn fn_004fd090(e: &mut Engine, this: Ptr<BGSProjectile>) {
    e.vcall(this.addr() + MUZZLE_FLASH_MODEL, MODEL_CLEAR_SLOT, &[]);
}

// ---- InitItem ---------------------------------------------------------------------------

/// The form pointers `InitItem` resolves, in order: getter, setter and the
/// type the form is cast to.
const RESOLVED_FORMS: [(u32, u32, u32); 7] = [
    (GET_LIGHT, SET_LIGHT, TYPE_TES_OBJECT_LIGH),
    (
        GET_MUZZLE_FLASH_LIGHT,
        SET_MUZZLE_FLASH_LIGHT,
        TYPE_TES_OBJECT_LIGH,
    ),
    (GET_EXPLOSION, SET_EXPLOSION, TYPE_BGS_EXPLOSION),
    (GET_ACTIVE_SOUND_LOOP, SET_ACTIVE_SOUND_LOOP, TYPE_TES_SOUND),
    (GET_COUNTDOWN_SOUND, SET_COUNTDOWN_SOUND, TYPE_TES_SOUND),
    (GET_DEACTIVATE_SOUND, SET_DEACTIVATE_SOUND, TYPE_TES_SOUND),
    (
        GET_DEFAULT_WEAPON_SOURCE,
        SET_DEFAULT_WEAPON_SOURCE,
        TYPE_TES_OBJECT_WEAP,
    ),
];

// Translated from 004fd0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSProjectile::InitItem` (Xbox PDB), vtable slot `0x88`: unless bit 3 of
/// the form flags is set, initializes the destructible component, then turns
/// the form ids in the data block (light, muzzle flash light, explosion,
/// active sound loop, countdown sound, deactivate sound, default weapon
/// source) into forms of the expected class (the id is moved to the running
/// game's load order with the projectile's file first; one that does not
/// resolve becomes null, nothing is logged). A projectile without data flag
/// `2` loses its explosion. Marks the form initialized.
pub fn bgs_projectile_init_item(e: &mut Engine, this: Ptr<BGSProjectile>) {
    if e.call(FORM_FLAG_8_TEST, &args![this]).bool() {
        return;
    }
    e.call(
        DESTRUCTIBLE_INIT_ITEM,
        &args![this.addr() + DESTRUCTIBLE, this],
    );
    for (getter, setter, target_type) in RESOLVED_FORMS {
        let id = e.call(getter, &args![this]).u32();
        if id != 0 {
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
                    &args![form, 0u32, TYPE_TES_FORM, target_type, 0u32],
                )
                .u32();
            e.call(setter, &args![this, cast]);
        }
    }
    if !e.call(FLAG_0X2_TEST, &args![this]).bool() {
        e.call(SET_EXPLOSION, &args![this, 0u32]);
    }
    e.call(FORM_SET_FLAG_8, &args![this, 1u32]);
}

// ---- Clone3D ----------------------------------------------------------------------------

// Translated from 004fd510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSProjectile::Clone3D` (Xbox PDB), vtable slot `0x14C`: makes the 3D
/// model of `reference` (zero for no reference).
///
/// Projectile types `0x40000` and `0x100000` use `TESBoundObject::Clone3D`
/// directly. Otherwise, when the reference has the two linked objects
/// (`+0x144`, then `+8`), the model comes from clone of the object that
/// virtual slot `0x174` of that second object loads (`NiObject::Clone` with
/// a fresh `NiCloningProcess`); else the projectile's own slot `0x174` is
/// tried, looked up in the clone pool when the reference's id is
/// `0xFF000000` or above and the gate object allows it; and failing that
/// `TESBoundObject::Clone3D` with `flag` is used (with its fallback
/// cleanup when slot `0x174` produced something). The result then gets
/// its node flag set (`0049ee60`, 1) when its virtual slot `0x10` answers.
///
/// Not translated: the exception unwinding frame (destroys the cloning
/// process if a call throws).
// Translation incomplete: 004fd510 - the C++ exception unwinding frame (FS:[0] handler chain that destroys the stack NiCloningProcess if a call throws) is not translated.
pub fn bgs_projectile_clone3d(
    e: &mut Engine,
    this: Ptr<BGSProjectile>,
    reference: u32,
    flag: u8,
) -> u32 {
    let mut result = 0u32;
    'clone: {
        if reference == 0 {
            break 'clone;
        }
        let kind = e.call(PROJECTILE_TYPE, &args![this]).u32();
        if kind == TYPE_0X40000 || kind == TYPE_0X100000 {
            result = e
                .call(BOUND_OBJECT_CLONE3D, &args![this, reference, 1u32])
                .u32();
            break 'clone;
        }
        let first = e.call(GET_WORD_144, &args![reference]).u32();
        if first != 0 {
            let second = e.call(GET_WORD_144, &args![reference]).u32();
            if e.call(GET_WORD_8, &args![second]).u32() != 0 {
                let third = e.call(GET_WORD_144, &args![reference]).u32();
                let loader = e.call(GET_WORD_8, &args![third]).u32();
                let model = e.vcall(loader, LOAD_3D_SLOT, &args![0u32]).u32();
                e.with_stack(CLONING_PROCESS_SIZE, |e, process| {
                    e.call(CLONING_PROCESS_CONSTRUCTOR, &args![process, 1.0f32]);
                    if model != 0 {
                        result = e.call(NI_OBJECT_CLONE, &args![model, process]).u32();
                    }
                    e.call(CLONING_PROCESS_DESTRUCTOR, &args![process]);
                });
                break 'clone;
            }
        }
        let model = e.vcall(this.addr(), LOAD_3D_SLOT, &args![reference]).u32();
        if model != 0 {
            let id = e.call(GET_FORM_ID, &args![reference]).u32();
            let handler = e.global::<u32>(DATA_HANDLER);
            if e.call(ID_AT_OR_ABOVE_DYNAMIC, &args![handler, id]).bool() {
                let gate = e.global::<u32>(CLONE_GATE_OBJECT);
                if !e.call(CLONE_GATE_TEST, &args![gate]).bool() {
                    let pool = e.call(CLONE_POOL_GET, &args![1u32]).u32();
                    result = e.call(CLONE_POOL_LOOKUP, &args![pool, model]).u32();
                }
            }
        }
        if result == 0 {
            result = e
                .call(BOUND_OBJECT_CLONE3D, &args![this, reference, flag as u32])
                .u32();
            if model != 0 {
                e.call(CLONE_FALLBACK_CLEANUP, &args![this, reference]);
            }
        }
    }
    if result != 0 && e.vcall(result, NI_OBJECT_SLOT_10, &[]).u32() != 0 {
        let node = e.vcall(result, NI_OBJECT_SLOT_10, &[]).u32();
        e.call(NODE_SET_FLAG, &args![node, 1u32]);
    }
    result
}

// ---- Load, Save ---------------------------------------------------------------------------

/// A component pointer, or null when the object itself is null.
fn component(object: u32, offset: u32) -> u32 {
    if object == 0 {
        0
    } else {
        object + offset
    }
}

// Translated from 004fd6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSProjectile::Load` (Xbox PDB), vtable slot `0x20`: reads a projectile
/// record (form type `0x33`) from `file` and returns whether it did. Clears
/// the data (virtual slot `0x18`), reads the form header
/// (`TESForm::LoadForm`), clears bit 3 of the form flags and loops over the
/// chunks: `EDID` (editor id, virtual slot `0x134`), `OBND` (object bound,
/// slot `0xE0`), `FULL` (name), `MODL`/`MODT` (model), `DEST`/`DSTD`
/// (destruction data), `DATA` (the 0x54-byte data block, byte-swapped when
/// the file is), `NAM1` (the muzzle flash model path, when not empty), `NAM2`
/// (its texture chunk, when not empty) and `VNAM` (the sound level: the
/// word when it is 0 to 3, else 1). Unknown chunks are skipped.
///
/// Not translated: the stack-protector cookie of the compiled function. The
/// `alloca` buffers are heap blocks here.
// Translation incomplete: 004fd6f0 - the compiler's security cookie check (`__security_check_cookie`, 00ec408c) is not translated.
pub fn bgs_projectile_load(e: &mut Engine, this: Ptr<BGSProjectile>, file: Ptr) -> bool {
    let object = this.addr();
    if e.call(FILE_GET_FORM_TYPE, &args![file]).u8() as u32 != FORM_TYPE_PROJECTILE {
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
            CHUNK_DSTD | CHUNK_DEST => {
                let destructible = component(object, DESTRUCTIBLE);
                e.call(DESTRUCTIBLE_LOAD_CHUNK, &args![destructible, file]);
            }
            CHUNK_DATA => {
                e.call(FORM_LOAD_DATA, &args![this, file, object + DATA, DATA_SIZE]);
            }
            CHUNK_NAM1 => {
                if e.call(FILE_CHUNK_SIZE, &args![file]).u32() != 0 {
                    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                    let buffer = e.mem.alloc(size.max(1));
                    e.call(FILE_GET_CHUNK_BYTES, &args![file, buffer, 0u32]);
                    e.vcall(
                        object + MUZZLE_FLASH_MODEL,
                        MODEL_SET_PATH_SLOT,
                        &args![buffer],
                    );
                    e.mem.free(buffer);
                }
            }
            CHUNK_NAM2 => {
                if e.call(FILE_CHUNK_SIZE, &args![file]).u32() != 0 {
                    e.call(
                        MODEL_LOAD_TEXTURE_CHUNK,
                        &args![object + MUZZLE_FLASH_MODEL, file],
                    );
                }
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
            CHUNK_MODL | CHUNK_MODT => {
                let model = component(object, MODEL);
                e.call(MODEL_LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_VNAM => {
                let level = e.with_stack(4, |e, slot| {
                    e.call(FILE_GET_CHUNK_WORD, &args![file, slot]);
                    e.mem.u32(slot.addr())
                });
                e.mem
                    .set_u32(object + SOUND_LEVEL, if level <= 3 { level } else { 1 });
            }
            _ => {}
        }
        if chunk == CHUNK_DATA && e.call(FILE_SWAPS, &args![file]).bool() {
            bgs_projectile_data_endian(e, data_block(this));
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 004fda00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSProjectile::Save` (Xbox PDB), vtable slot `0x2C`: writes the record:
/// the name (`FULL`), the model (`MODL`/`MODT`), the destruction data, the
/// 0x54-byte data block (with the seven form pointers replaced by their form
/// ids while it is written and the numbers byte-swapped around the write when
/// the save swaps), then the muzzle flash model (`NAM1`/`NAM2`; an empty path
/// is written as a one-byte `NAM1`) and the sound level (`VNAM`).
pub fn bgs_projectile_save(e: &mut Engine, this: Ptr<BGSProjectile>) {
    let object = this.addr();
    e.call(FORM_START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![object + FULL_NAME]);
    e.call(MODEL_SAVE, &args![object + MODEL, CHUNK_MODL, CHUNK_MODT]);
    e.call(DESTRUCTIBLE_SAVE, &args![object + DESTRUCTIBLE]);

    // The pointers become form ids for the write and are restored after.
    const POINTERS: [u32; 7] = [0x70, 0x74, 0x84, 0x88, 0x98, 0x9c, 0xa0];
    let mut saved = [0u32; 7];
    for (slot, offset) in POINTERS.into_iter().enumerate() {
        let pointer = e.mem.u32(object + offset);
        saved[slot] = pointer;
        let id = if pointer != 0 {
            e.call(GET_FORM_ID, &args![pointer]).u32()
        } else {
            0
        };
        e.mem.set_u32(object + offset, id);
    }
    if e.call(SWAP_ON_SAVE, &[]).bool() {
        bgs_projectile_data_endian(e, data_block(this));
    }
    e.call(FORM_SAVE_DATA, &args![this, object + DATA, DATA_SIZE]);
    if e.call(SWAP_ON_SAVE, &[]).bool() {
        bgs_projectile_data_endian(e, data_block(this));
    }
    for (slot, offset) in POINTERS.into_iter().enumerate() {
        e.mem.set_u32(object + offset, saved[slot]);
    }

    let muzzle = object + MUZZLE_FLASH_MODEL;
    if e.call(MODEL_PATH_LENGTH, &args![muzzle]).u32() == 0 {
        let length = e.call(MODEL_PATH_LENGTH, &args![muzzle]).u32();
        let path = e.vcall(muzzle, MODEL_GET_PATH_SLOT, &[]).u32();
        e.call(
            ADD_CHUNK_ARRAY,
            &args![CHUNK_NAM1, path, length.wrapping_add(1)],
        );
    } else {
        e.call(MODEL_SAVE, &args![muzzle, CHUNK_NAM1, CHUNK_NAM2]);
    }
    let sound_level = e.mem.u32(object + SOUND_LEVEL);
    e.call(ADD_CHUNK_WORD, &args![CHUNK_VNAM, sound_level]);
    e.call(FORM_CLOSE_FORM, &args![this]);
}

// ---- Copy, Compare ----------------------------------------------------------------------

/// `__RTDynamicCast` of `other` to `BGSProjectile`.
fn cast_to_projectile(e: &mut Engine, other: u32) -> Ptr<BGSProjectile> {
    e.call(
        RT_DYNAMIC_CAST,
        &args![other, 0u32, TYPE_TES_FORM, TYPE_BGS_PROJECTILE, 0u32],
    )
    .ptr()
}

// Translated from 004fdcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x108` (`Copy` in the Xbox PDB): when `other` is a
/// projectile, copies its components (`TESForm::CopyAllComponents`), its
/// muzzle flash model (virtual slot 2), its sound level and its 0x54-byte
/// data block.
pub fn fn_004fdcf0(e: &mut Engine, this: Ptr<BGSProjectile>, other: u32) {
    let source = cast_to_projectile(e, other);
    if source.is_null() {
        return;
    }
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, source]);
    e.vcall(
        this.addr() + MUZZLE_FLASH_MODEL,
        MODEL_COPY_SLOT,
        &args![source.addr() + MUZZLE_FLASH_MODEL],
    );
    let level = e.mem.u32(source.addr() + SOUND_LEVEL);
    e.mem.set_u32(this.addr() + SOUND_LEVEL, level);
    let source_data = e.call(DATA_POINTER, &args![source]).u32();
    let data = e.call(DATA_POINTER, &args![this]).u32();
    e.call(MEMCPY, &args![data, source_data, DATA_SIZE]);
}

// Translated from 004fdd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x10C` (`Compare` in the Xbox PDB): true when `other` is not
/// a projectile, or its components, data block, muzzle flash model or sound
/// level differ.
pub fn fn_004fdd80(e: &mut Engine, this: Ptr<BGSProjectile>, other: u32) -> bool {
    let target = cast_to_projectile(e, other);
    if target.is_null() {
        return true;
    }
    if e.call(FORM_COMPARE_ALL_COMPONENTS, &args![this, target])
        .bool()
    {
        return true;
    }
    let target_data = e.call(DATA_POINTER, &args![target]).u32();
    if e.call(MEMCMP, &args![this.addr() + DATA, target_data, DATA_SIZE])
        .i32()
        != 0
    {
        return true;
    }
    if e.vcall(
        this.addr() + MUZZLE_FLASH_MODEL,
        MODEL_COMPARE_SLOT,
        &args![target.addr() + MUZZLE_FLASH_MODEL],
    )
    .bool()
    {
        return true;
    }
    let target_level = e.call(GET_WORD_CC, &args![target]).u32();
    e.mem.u32(this.addr() + SOUND_LEVEL) != target_level
}

// ---- Other virtual functions and helpers --------------------------------------------------

// Translated from 004fde40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x124` of the projectile (not identified): `projectile` is
/// a reference cast to `Projectile` and `actor` one cast to `Actor`; both
/// must be non-null. When this form has data flag `0x20`, the projectile
/// reference does not have its own flag `0x200`, and the projectile's
/// virtual slot `0x320` accepts the actor, the answer is true. Otherwise,
/// when this form has data flag `0x40`, `0050f670` (this, the four other
/// arguments) decides; else false. Returns the low byte as a `bool`
/// (the game leaves the rest of `EAX` unspecified).
pub fn fn_004fde40(
    e: &mut Engine,
    this: Ptr<BGSProjectile>,
    projectile_reference: u32,
    actor_reference: u32,
    byte_argument: u8,
    fourth: u32,
    fifth: u32,
) -> bool {
    let actor = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                actor_reference,
                0u32,
                TYPE_TES_OBJECT_REFR,
                TYPE_ACTOR,
                0u32
            ],
        )
        .u32();
    let projectile = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                projectile_reference,
                0u32,
                TYPE_TES_OBJECT_REFR,
                TYPE_PROJECTILE,
                0u32
            ],
        )
        .u32();
    if actor == 0 || projectile == 0 {
        return false;
    }
    let base = e.call(GET_BASE_FORM, &args![projectile]).u32();
    if fn_004fdf20(e, Ptr::new(base))
        && !e
            .call(REFERENCE_FLAGS_TEST, &args![projectile, 0x200u32])
            .bool()
        && e.vcall(projectile, PROJECTILE_HIT_SLOT, &args![actor, 0u32])
            .bool()
    {
        return true;
    }
    let base = e.call(GET_BASE_FORM, &args![projectile]).u32();
    if fn_004fdf40(e, Ptr::new(base)) {
        return e
            .call(
                BOUND_OBJECT_APPLY,
                &args![
                    this,
                    projectile_reference,
                    actor_reference,
                    byte_argument as u32,
                    fourth,
                    fifth
                ],
            )
            .bool();
    }
    false
}

// Translated from 004fdf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Data flag `0x20` of the projectile (`Data.iFlags & 0x20`).
pub fn fn_004fdf20(e: &mut Engine, this: Ptr<BGSProjectile>) -> bool {
    e.call(FLAGS_TEST, &args![this, FLAG_0X20]).bool()
}

// Translated from 004fdf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Data flag `0x40` of the projectile (`Data.iFlags & 0x40`).
pub fn fn_004fdf40(e: &mut Engine, this: Ptr<BGSProjectile>) -> bool {
    e.call(FLAGS_TEST, &args![this, FLAG_0X40]).bool()
}

// Translated from 004fdf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tracer roll: true when a random number modulo 100 is below
/// `fTracerChance * 100` (the product truncated to an integer, compared as
/// unsigned).
pub fn fn_004fdf60(e: &mut Engine, this: Ptr<BGSProjectile>) -> bool {
    let chance = e.call(GET_FLOAT_78, &args![this]).f64();
    let scale: f64 = e.global(TRACER_CHANCE_SCALE);
    let threshold = e.call(FTOL, &args![chance * scale]).u32();
    let random = e.call(RANDOM, &[]).u32();
    random % 100 < threshold
}

// Translated from 004fdfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSPreloadable` component's virtual slot `0x10` (`QueueFiles` in the
/// Xbox PDB) as the projectile installs it; `this` is the component, at
/// `+0x54` of the projectile. Queues the projectile's model (with the
/// multiplier `0045c6b0` gives for the form), then the muzzle flash model
/// (multiplier 3), then calls the same slot of the explosion's own component
/// (`+0x64` of the explosion), each with the two arguments.
pub fn fn_004fdfb0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let projectile = this.addr().wrapping_sub(PRELOADABLE);
    let queue = e.global::<u32>(MODEL_QUEUE_OBJECT);
    if e.call(MODEL_PATH_LENGTH, &args![this.addr().wrapping_sub(0x18)])
        .u32()
        != 0
    {
        let model = if projectile == 0 {
            0
        } else {
            this.addr().wrapping_sub(0x18)
        };
        let multiplier = e.call(FORM_LOD_MULT, &args![projectile]).u32();
        e.call(
            QUEUE_MODEL,
            &args![queue, model, first, second, multiplier, 1u32, 0u32, 0u32],
        );
    }
    if e.call(MODEL_PATH_LENGTH, &args![this.addr() + 0x60]).u32() != 0 {
        e.call(
            QUEUE_MODEL,
            &args![
                queue,
                this.addr() + 0x60,
                first,
                second,
                3u32,
                1u32,
                0u32,
                0u32
            ],
        );
    }
    if e.call(GET_EXPLOSION, &args![projectile]).u32() != 0 {
        let explosion = e.call(GET_EXPLOSION, &args![projectile]).u32();
        let component = explosion + 0x64;
        e.vcall(
            component,
            PRELOADABLE_QUEUE_FILES_SLOT,
            &args![first, second],
        );
    }
}

// Translated from 004fe080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSProjectile::GetPreCloneCount` (Xbox PDB): how many clones of the
/// projectile's model to prepare for `weapon`. 0 for projectile types
/// `0x40000`, `0x100000` and `0x80000`; 5 without a weapon. Otherwise the
/// weapon's float at `+0x134` truncated, times its projectile count, times
/// 3; limited to the weapon's clip rounds unless the type is `0x20000`;
/// times 4 when data flag `0x100` is set.
pub fn bgs_projectile_get_pre_clone_count(
    e: &mut Engine,
    this: Ptr<BGSProjectile>,
    weapon: u32,
) -> i32 {
    if e.call(PROJECTILE_TYPE, &args![this]).u32() == TYPE_0X40000
        || e.call(PROJECTILE_TYPE, &args![this]).u32() == TYPE_0X100000
        || e.call(PROJECTILE_TYPE, &args![this]).u32() == TYPE_0X80000
    {
        return 0;
    }
    if weapon == 0 {
        return 5;
    }
    let value = e.call(WEAPON_FLOAT_134, &args![weapon]).f64();
    let truncated = e.call(FTOL, &args![value]).i32();
    let projectiles = e
        .call(WEAPON_GET_NUM_PROJECTILES, &args![weapon, 0u32, 0u32, 0u32])
        .u8();
    let mut count = truncated.wrapping_mul(projectiles as i32).wrapping_mul(3);
    if e.call(PROJECTILE_TYPE, &args![this]).u32() != TYPE_0X20000 {
        let clip = tes_object_weap_get_form_clip_rounds(e, Ptr::new(weapon), 0);
        count = e.call(MIN_INT, &args![count, clip]).i32();
    }
    if e.call(FLAGS_TEST, &args![this, FLAG_0X100]).bool() {
        count = count.wrapping_shl(2);
    }
    count
}

// Translated from 004fe160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::GetFormClipRounds` (Xbox PDB; the body reads the
/// `BGSClipRoundsForm` at `+0xAC` of the weapon, `cClipRounds`): the clip
/// rounds, plus when `with_mod` the weapon's mod effect value `2` truncated
/// to an integer and to its low byte.
pub fn tes_object_weap_get_form_clip_rounds(e: &mut Engine, this: Ptr, with_mod: u8) -> i32 {
    let clip_rounds_form = this.addr() + 0xac;
    if with_mod != 0 {
        let effect = e
            .call(WEAPON_GET_MOD_EFFECT_VALUE, &args![this, 2u32, 0u32])
            .f64();
        // `FISTP` with the rounding mode forced to truncation.
        let bonus = effect as i32 as u8 as i32;
        let rounds = e.call(CLIP_ROUNDS_GET, &args![clip_rounds_form]).u32() as i32;
        rounds.wrapping_add(bonus)
    } else {
        e.call(CLIP_ROUNDS_GET, &args![clip_rounds_form]).u32() as i32
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004fcc00,
            fn_004fcc00(Ptr<BGSProjectileData>) -> Ptr<BGSProjectileData>
        ),
        entry!(
            0x004fccf0,
            bgs_projectile_data_endian(Ptr<BGSProjectileData>)
        ),
        entry!(
            0x004fce60,
            fn_004fce60(Ptr<BGSProjectile>) -> Ptr<BGSProjectile>
        ),
        entry!(
            0x004fcf50,
            fn_004fcf50(Ptr<BGSProjectile>, u32) -> Ptr<BGSProjectile>
        ),
        entry!(0x004fcf80, fn_004fcf80(Ptr<BGSProjectile>)),
        entry!(0x004fd050, fn_004fd050(Ptr<BGSProjectile>)),
        entry!(0x004fd090, fn_004fd090(Ptr<BGSProjectile>)),
        entry!(0x004fd0c0, bgs_projectile_init_item(Ptr<BGSProjectile>)),
        entry!(
            0x004fd510,
            bgs_projectile_clone3d(Ptr<BGSProjectile>, u32, u8) -> u32
        ),
        entry!(
            0x004fd6f0,
            bgs_projectile_load(Ptr<BGSProjectile>, Ptr) -> bool
        ),
        entry!(0x004fda00, bgs_projectile_save(Ptr<BGSProjectile>)),
        entry!(0x004fdcf0, fn_004fdcf0(Ptr<BGSProjectile>, u32)),
        entry!(0x004fdd80, fn_004fdd80(Ptr<BGSProjectile>, u32) -> bool),
        entry!(
            0x004fde40,
            fn_004fde40(Ptr<BGSProjectile>, u32, u32, u8, u32, u32) -> bool
        ),
        entry!(0x004fdf20, fn_004fdf20(Ptr<BGSProjectile>) -> bool),
        entry!(0x004fdf40, fn_004fdf40(Ptr<BGSProjectile>) -> bool),
        entry!(0x004fdf60, fn_004fdf60(Ptr<BGSProjectile>) -> bool),
        entry!(0x004fdfb0, fn_004fdfb0(Ptr, u32, u32)),
        entry!(
            0x004fe080,
            bgs_projectile_get_pre_clone_count(Ptr<BGSProjectile>, u32) -> i32
        ),
        entry!(
            0x004fe160,
            tes_object_weap_get_form_clip_rounds(Ptr, u8) -> i32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn projectile(e: &mut Engine) -> Ptr<BGSProjectile> {
        e.new_object()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn returning(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    fn returning_float(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    fn recording(e: &mut Engine, addr: u32) -> Log {
        recording_returning(e, addr, 0)
    }

    fn recording_returning(e: &mut Engine, addr: u32, value: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        log
    }

    fn give_vtable(e: &mut Engine, object: u32, slots: &[(u32, u32)]) {
        let vtable = e.mem.alloc(0x800);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        e.mem.set_u32(object, vtable);
    }

    fn map_pages(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.map(addr & !0xfff, 0x1000);
        }
    }

    /// `_ftol2_sse` double: truncates the `f64` argument.
    fn ftol_double(e: &mut Engine) {
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            Ret {
                eax: value as i32 as u32,
                ..Ret::default()
            }
        });
    }

    /// Doubles that read the data flags of a projectile like the game.
    fn flag_doubles(e: &mut Engine) {
        e.register(DATA_POINTER, |_, a| Ret {
            eax: a[0] + 0x60,
            ..Ret::default()
        });
        e.register(PROJECTILE_TYPE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x60) & 0x1f_0000,
            ..Ret::default()
        });
        e.register(FLAGS_TEST, |e, a| Ret {
            eax: (e.mem.u32(a[0] + 0x60) & a[1] != 0) as u32,
            ..Ret::default()
        });
        e.register(FLAG_0X2_TEST, |e, a| Ret {
            eax: (e.mem.u32(a[0] + 0x60) & 2 != 0) as u32,
            ..Ret::default()
        });
    }

    #[test]
    fn data_constructor_writes_the_defaults() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x54);
        let calls = recording(&mut e, DATA_DEFAULTS);
        assert_eq!(e.call(0x004f_cc00, &args![data]).u32(), data);
        assert_eq!(calls.borrow().as_slice(), &[vec![data]]);
    }

    #[test]
    fn endian_swaps_the_fourteen_numbers() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x54);
        let calls = recording(&mut e, SWAP_WORD);
        e.call(0x004f_ccf0, &args![data]);
        let offsets: Vec<u32> = calls.borrow().iter().map(|c| c[0] - data).collect();
        assert_eq!(
            offsets,
            vec![0, 4, 8, 0xc, 0x18, 0x1c, 0x20, 0x2c, 0x30, 0x34, 0x44, 0x48, 0x4c, 0x50]
        );
        assert!(calls.borrow().iter().all(|c| c[1] == 0));
    }

    #[test]
    fn constructor_builds_the_components_and_vtables() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        let order = Rc::new(RefCell::new(Vec::<(u32, u32)>::new()));
        for addr in [
            BOUND_OBJECT_CONSTRUCTOR,
            FULL_NAME_CONSTRUCTOR,
            MODEL_CONSTRUCTOR,
            PRELOADABLE_CONSTRUCTOR,
            DESTRUCTIBLE_CONSTRUCTOR,
            DATA_DEFAULTS,
            DATA_POINTER,
        ] {
            let sink = order.clone();
            e.register_double(addr, move |_, a| {
                sink.borrow_mut().push((addr, a[0]));
                Ret {
                    eax: a[0] + 0x60,
                    ..Ret::default()
                }
            });
        }
        let types = recording(&mut e, FORM_SET_FORM_TYPE);
        give_vtable(&mut e, p.addr() + 0xb4, &[(0, 0x00dd_0000)]);
        let init = recording(&mut e, 0x00dd_0000);
        assert_eq!(e.call(0x004f_ce60, &args![p]).u32(), p.addr());
        let a = p.addr();
        assert_eq!(
            order.borrow().iter().map(|x| x.0).collect::<Vec<_>>(),
            vec![
                BOUND_OBJECT_CONSTRUCTOR,
                FULL_NAME_CONSTRUCTOR,
                MODEL_CONSTRUCTOR,
                PRELOADABLE_CONSTRUCTOR,
                DESTRUCTIBLE_CONSTRUCTOR,
                DATA_DEFAULTS,
                MODEL_CONSTRUCTOR,
                DATA_POINTER,
                DATA_DEFAULTS,
            ]
        );
        assert_eq!(order.borrow()[1].1, a + 0x30);
        assert_eq!(order.borrow()[6].1, a + 0xb4);
        assert_eq!(e.mem.u32(a), VTABLE);
        assert_eq!(e.mem.u32(a + 0x30), 0x0102_5198);
        assert_eq!(e.mem.u32(a + 0x3c), 0x0102_5174);
        assert_eq!(e.mem.u32(a + 0x54), 0x0102_515c);
        assert_eq!(e.mem.u32(a + 0x58), 0x0102_5148);
        assert_eq!(types.borrow().as_slice(), &[vec![a, 0x33]]);
        assert_eq!(init.borrow().len(), 1);
        assert_eq!(e.mem.u32(a + 0xcc), 1);
    }

    #[test]
    fn destructor_body_and_deleting_destructor() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        let order = Rc::new(RefCell::new(Vec::<(u32, u32)>::new()));
        for addr in [
            MODEL_DESTRUCTOR,
            DESTRUCTIBLE_DESTRUCTOR,
            FULL_NAME_DESTRUCTOR,
            BOUND_OBJECT_DESTRUCTOR,
        ] {
            let sink = order.clone();
            e.register_double(addr, move |_, a| {
                sink.borrow_mut().push((addr, a[0]));
                Ret::default()
            });
        }
        give_vtable(&mut e, p.addr() + 0xb4, &[(4, 0x00dd_0004)]);
        let clear = recording(&mut e, 0x00dd_0004);
        let free = recording(&mut e, OPERATOR_DELETE);
        let a = p.addr();
        assert_eq!(e.call(0x004f_cf50, &args![p, 0u32]).u32(), a);
        assert!(free.borrow().is_empty());
        assert_eq!(e.mem.u32(a), VTABLE);
        assert_eq!(clear.borrow().len(), 1);
        assert_eq!(
            order.borrow().as_slice(),
            &[
                (MODEL_DESTRUCTOR, a + 0xb4),
                (DESTRUCTIBLE_DESTRUCTOR, a + 0x58),
                (MODEL_DESTRUCTOR, a + 0x3c),
                (FULL_NAME_DESTRUCTOR, a + 0x30),
                (BOUND_OBJECT_DESTRUCTOR, a),
            ]
        );
        e.call(0x004f_cf50, &args![p, 1u32]);
        assert_eq!(free.borrow().as_slice(), &[vec![a]]);
    }

    #[test]
    fn initialize_and_clear_data_use_the_muzzle_flash_model_slots() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        flag_doubles(&mut e);
        let defaults = recording(&mut e, DATA_DEFAULTS);
        give_vtable(
            &mut e,
            p.addr() + 0xb4,
            &[(0, 0x00dd_0000), (4, 0x00dd_0004)],
        );
        let init = recording(&mut e, 0x00dd_0000);
        let clear = recording(&mut e, 0x00dd_0004);
        e.mem.set_u32(p.addr() + 0xcc, 9);
        e.call(0x004f_d050, &args![p]);
        assert_eq!(defaults.borrow().as_slice(), &[vec![p.addr() + 0x60]]);
        assert_eq!(init.borrow().as_slice(), &[vec![p.addr() + 0xb4]]);
        assert_eq!(e.mem.u32(p.addr() + 0xcc), 1);
        e.call(0x004f_d090, &args![p]);
        assert_eq!(clear.borrow().as_slice(), &[vec![p.addr() + 0xb4]]);
    }

    /// Doubles for `InitItem`: the getters return the ids in `ids`.
    fn init_item_doubles(e: &mut Engine, ids: [u32; 7]) -> Rc<RefCell<Vec<(u32, u32)>>> {
        flag_doubles(e);
        returning(e, FORM_FLAG_8_TEST, 0);
        quiet(e, &[DESTRUCTIBLE_INIT_ITEM]);
        returning(e, FORM_GET_FILE, 0x7000);
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: if a[0] & 0xff == 0xff { 0 } else { a[0] },
            ..Ret::default()
        });
        for (slot, (getter, _, _)) in RESOLVED_FORMS.iter().enumerate() {
            returning(e, *getter, ids[slot]);
        }
        let sets = Rc::new(RefCell::new(Vec::new()));
        for (_, setter, _) in RESOLVED_FORMS {
            let sink = sets.clone();
            e.register_double(setter, move |_, a| {
                sink.borrow_mut().push((setter, a[1]));
                Ret::default()
            });
        }
        sets
    }

    #[test]
    fn init_item_does_nothing_when_flag_eight_is_set() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        returning(&mut e, FORM_FLAG_8_TEST, 1);
        let flags = recording(&mut e, FORM_SET_FLAG_8);
        e.call(0x004f_d0c0, &args![p]);
        assert!(flags.borrow().is_empty());
    }

    #[test]
    fn init_item_resolves_every_form_pointer() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        let sets = init_item_doubles(&mut e, [1, 2, 3, 0xff, 0, 6, 7]);
        // Flag 2 is set, so the explosion is kept.
        e.mem.set_u32(p.addr() + 0x60, 2);
        let flags = recording(&mut e, FORM_SET_FLAG_8);
        e.call(0x004f_d0c0, &args![p]);
        let expect = |i: usize, value: u32| (RESOLVED_FORMS[i].1, value);
        assert_eq!(
            sets.borrow().as_slice(),
            &[
                expect(0, 0x0100_0001 + 0x1000),
                expect(1, 0x0100_0002 + 0x1000),
                expect(2, 0x0100_0003 + 0x1000),
                // The unknown form becomes null; the zero id is skipped.
                expect(3, 0),
                expect(5, 0x0100_0006 + 0x1000),
                expect(6, 0x0100_0007 + 0x1000),
            ]
        );
        assert_eq!(flags.borrow().as_slice(), &[vec![p.addr(), 1]]);
    }

    #[test]
    fn init_item_clears_the_explosion_without_flag_two() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        let sets = init_item_doubles(&mut e, [0; 7]);
        e.call(0x004f_d0c0, &args![p]);
        assert_eq!(sets.borrow().as_slice(), &[(SET_EXPLOSION, 0)]);
    }

    struct CloneLog {
        bound: Log,
        cleanup: Log,
    }

    fn clone_doubles(e: &mut Engine, kind: u32) -> CloneLog {
        returning(e, PROJECTILE_TYPE, kind);
        let bound = recording_returning(e, BOUND_OBJECT_CLONE3D, 0x5000);
        let cleanup = recording(e, CLONE_FALLBACK_CLEANUP);
        map_pages(e, &[DATA_HANDLER, CLONE_GATE_OBJECT]);
        e.set_global(DATA_HANDLER, 0x1111u32);
        e.set_global(CLONE_GATE_OBJECT, 0x2222u32);
        returning(e, GET_WORD_144, 0);
        returning(e, GET_FORM_ID, 0xff00_0001);
        returning(e, ID_AT_OR_ABOVE_DYNAMIC, 1);
        returning(e, CLONE_GATE_TEST, 0);
        returning(e, CLONE_POOL_GET, 0x3000);
        returning(e, CLONE_POOL_LOOKUP, 0);
        quiet(e, &[NODE_SET_FLAG]);
        // The results answer nothing on slot 0x10.
        map_pages(e, &[0x5000, 0x8000, 0xa000]);
        quiet(e, &[0x00dd_0010]);
        for object in [0x5000, 0x8000, 0xa000] {
            give_vtable(e, object, &[(NI_OBJECT_SLOT_10, 0x00dd_0010)]);
        }
        CloneLog { bound, cleanup }
    }

    #[test]
    fn clone3d_without_reference_returns_null() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        let log = clone_doubles(&mut e, 0);
        assert_eq!(e.call(0x004f_d510, &args![p, 0u32, 0u32]).u32(), 0);
        assert!(log.bound.borrow().is_empty());
    }

    #[test]
    fn clone3d_of_the_direct_types_uses_the_bound_object_clone() {
        let mut e = Engine::new();
        map_pages(&mut e, &[0, 0x4000, 0x5000, 0x8000]);
        let p = projectile(&mut e);
        let log = clone_doubles(&mut e, TYPE_0X40000);
        // The result answers on slot 0x10 with a node.
        give_vtable(&mut e, 0x5000, &[(0x10, 0x00dd_0010)]);
        returning(&mut e, 0x00dd_0010, 0x6000);
        let node = recording(&mut e, NODE_SET_FLAG);
        assert_eq!(
            e.call(0x004f_d510, &args![p, 0x4000u32, 0u32]).u32(),
            0x5000
        );
        assert_eq!(log.bound.borrow().as_slice(), &[vec![p.addr(), 0x4000, 1]]);
        assert_eq!(node.borrow().as_slice(), &[vec![0x6000, 1]]);
    }

    #[test]
    fn clone3d_falls_back_to_the_bound_object_clone() {
        let mut e = Engine::new();
        map_pages(&mut e, &[0, 0x4000, 0x5000, 0x8000]);
        let p = projectile(&mut e);
        let log = clone_doubles(&mut e, 0);
        give_vtable(&mut e, p.addr(), &[(LOAD_3D_SLOT, 0x00dd_0174)]);
        returning(&mut e, 0x00dd_0174, 0x7000);
        let lookups = recording(&mut e, CLONE_POOL_LOOKUP);
        e.call(0x004f_d510, &args![p, 0x4000u32, 5u32]);
        assert_eq!(lookups.borrow().as_slice(), &[vec![0x3000, 0x7000]]);
        assert_eq!(log.bound.borrow().as_slice(), &[vec![p.addr(), 0x4000, 5]]);
        assert_eq!(log.cleanup.borrow().as_slice(), &[vec![p.addr(), 0x4000]]);
    }

    #[test]
    fn clone3d_uses_the_pooled_copy_when_it_exists() {
        let mut e = Engine::new();
        map_pages(&mut e, &[0, 0x4000, 0x5000, 0x8000]);
        let p = projectile(&mut e);
        let log = clone_doubles(&mut e, 0);
        give_vtable(&mut e, p.addr(), &[(LOAD_3D_SLOT, 0x00dd_0174)]);
        returning(&mut e, 0x00dd_0174, 0x7000);
        returning(&mut e, CLONE_POOL_LOOKUP, 0x8000);
        assert_eq!(
            e.call(0x004f_d510, &args![p, 0x4000u32, 5u32]).u32(),
            0x8000
        );
        assert!(log.bound.borrow().is_empty());
        assert!(log.cleanup.borrow().is_empty());
    }

    #[test]
    fn clone3d_skips_the_pool_for_ordinary_ids_and_a_closed_gate() {
        let mut e = Engine::new();
        map_pages(&mut e, &[0, 0x4000, 0x5000, 0x8000]);
        let p = projectile(&mut e);
        let log = clone_doubles(&mut e, 0);
        give_vtable(&mut e, p.addr(), &[(LOAD_3D_SLOT, 0x00dd_0174)]);
        returning(&mut e, 0x00dd_0174, 0x7000);
        let lookups = recording(&mut e, CLONE_POOL_LOOKUP);
        returning(&mut e, ID_AT_OR_ABOVE_DYNAMIC, 0);
        e.call(0x004f_d510, &args![p, 0x4000u32, 0u32]);
        returning(&mut e, ID_AT_OR_ABOVE_DYNAMIC, 1);
        returning(&mut e, CLONE_GATE_TEST, 1);
        e.call(0x004f_d510, &args![p, 0x4000u32, 0u32]);
        assert!(lookups.borrow().is_empty());
        assert_eq!(log.bound.borrow().len(), 2);
    }

    #[test]
    fn clone3d_clones_the_linked_objects_model() {
        let mut e = Engine::new();
        map_pages(&mut e, &[0, 0x4000, 0x5000, 0x8000]);
        let p = projectile(&mut e);
        let log = clone_doubles(&mut e, 0);
        returning(&mut e, GET_WORD_144, 0x100);
        returning(&mut e, GET_WORD_8, 0x200);
        give_vtable(&mut e, 0x200, &[(LOAD_3D_SLOT, 0x00dd_0174)]);
        let loads = recording_returning(&mut e, 0x00dd_0174, 0x9000);
        let made = recording(&mut e, CLONING_PROCESS_CONSTRUCTOR);
        let ended = recording(&mut e, CLONING_PROCESS_DESTRUCTOR);
        let clone = recording_returning(&mut e, NI_OBJECT_CLONE, 0xA000);
        assert_eq!(
            e.call(0x004f_d510, &args![p, 0x4000u32, 0u32]).u32(),
            0xA000
        );
        assert_eq!(loads.borrow().as_slice(), &[vec![0x200, 0]]);
        let process = made.borrow()[0][0];
        assert_eq!(made.borrow()[0][1], 1.0f32.to_bits());
        assert_eq!(clone.borrow().as_slice(), &[vec![0x9000, process]]);
        assert_eq!(ended.borrow().as_slice(), &[vec![process]]);
        assert!(log.bound.borrow().is_empty());
    }

    /// A file with the given chunks (code, bytes) for `Load`.
    fn load_doubles(e: &mut Engine, form_type: u32, swaps: bool, chunks: Vec<(u32, Vec<u8>)>) {
        returning(e, FILE_GET_FORM_TYPE, form_type);
        quiet(e, &[FORM_LOAD_FORM, FORM_SET_FLAG_8]);
        returning(e, FILE_SWAPS, swaps as u32);
        let position = Rc::new(RefCell::new(0usize));
        let chunks = Rc::new(chunks);
        let (p, c) = (position.clone(), chunks.clone());
        e.register_double(FILE_GET_CHUNK, move |_, _| Ret {
            eax: c.get(*p.borrow()).map(|chunk| chunk.0).unwrap_or(0),
            ..Ret::default()
        });
        let (p, c) = (position.clone(), chunks.clone());
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            *p.borrow_mut() += 1;
            Ret {
                eax: (*p.borrow() < c.len()) as u32,
                ..Ret::default()
            }
        });
        let (p, c) = (position.clone(), chunks.clone());
        e.register_double(FILE_CHUNK_SIZE, move |_, _| Ret {
            eax: c[*p.borrow()].1.len() as u32,
            ..Ret::default()
        });
        let (p, c) = (position.clone(), chunks.clone());
        e.register_double(FILE_GET_CHUNK_BYTES, move |e, a| {
            let bytes = &c[*p.borrow()].1;
            let count = if a[2] == 0 {
                bytes.len()
            } else {
                (a[2] as usize).min(bytes.len())
            };
            for (i, byte) in bytes.iter().take(count).enumerate() {
                e.mem.set_u8(a[1] + i as u32, *byte);
            }
            Ret::default()
        });
        let (p, c) = (position, chunks);
        e.register_double(FILE_GET_CHUNK_WORD, move |e, a| {
            let bytes = &c[*p.borrow()].1;
            let mut word = [0u8; 4];
            word.copy_from_slice(&bytes[..4]);
            e.mem.set_u32(a[1], u32::from_le_bytes(word));
            Ret::default()
        });
    }

    /// A double that records the NUL-terminated string in its second word.
    fn recording_string(e: &mut Engine, addr: u32) -> Rc<RefCell<Vec<u8>>> {
        let text = Rc::new(RefCell::new(Vec::<u8>::new()));
        let sink = text.clone();
        e.register_double(addr, move |e, a| {
            let mut at = a[1];
            while e.mem.u8(at) != 0 {
                sink.borrow_mut().push(e.mem.u8(at));
                at += 1;
            }
            Ret::default()
        });
        text
    }

    #[test]
    fn load_rejects_other_record_types() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        load_doubles(&mut e, 0x32, false, vec![]);
        let load_form = recording(&mut e, FORM_LOAD_FORM);
        assert!(!e.call(0x004f_d6f0, &args![p, 0x5000u32]).bool());
        assert!(load_form.borrow().is_empty());
    }

    #[test]
    fn load_dispatches_every_chunk() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        load_doubles(
            &mut e,
            0x33,
            true,
            vec![
                (CHUNK_EDID, b"Proj\0".to_vec()),
                (CHUNK_OBND, vec![]),
                (CHUNK_FULL, vec![1]),
                (CHUNK_MODL, vec![1]),
                (CHUNK_MODT, vec![1]),
                (CHUNK_DEST, vec![1]),
                (CHUNK_DSTD, vec![1]),
                (CHUNK_DATA, vec![0; 4]),
                (CHUNK_NAM1, b"flash.nif\0".to_vec()),
                (CHUNK_NAM2, vec![1]),
                (CHUNK_VNAM, vec![2, 0, 0, 0]),
                (tag(b"XXXX"), vec![1]),
            ],
        );
        let a = p.addr();
        give_vtable(
            &mut e,
            a,
            &[
                (FORM_CLEAR_DATA_SLOT, 0x00dd_0018),
                (FORM_SET_EDITOR_ID_SLOT, 0x00dd_0134),
                (FORM_LOAD_OBJECT_BOUND_SLOT, 0x00dd_00e0),
            ],
        );
        give_vtable(&mut e, a + 0xb4, &[(MODEL_SET_PATH_SLOT, 0x00dd_0118)]);
        let clear = recording(&mut e, 0x00dd_0018);
        let bound = recording(&mut e, 0x00dd_00e0);
        let editor = recording_string(&mut e, 0x00dd_0134);
        let path = recording_string(&mut e, 0x00dd_0118);
        let name = recording(&mut e, FULL_NAME_LOAD);
        let model = recording(&mut e, MODEL_LOAD_MODEL_CHUNK);
        let texture = recording(&mut e, MODEL_LOAD_TEXTURE_CHUNK);
        let destruction = recording(&mut e, DESTRUCTIBLE_LOAD_CHUNK);
        let data = recording(&mut e, FORM_LOAD_DATA);
        let swaps = recording(&mut e, SWAP_WORD);
        assert!(e.call(0x004f_d6f0, &args![p, 0x5000u32]).bool());
        assert_eq!(clear.borrow().as_slice(), &[vec![a]]);
        assert_eq!(editor.borrow().as_slice(), b"Proj");
        assert_eq!(bound.borrow().as_slice(), &[vec![a, 0x5000]]);
        assert_eq!(name.borrow().as_slice(), &[vec![a + 0x30, 0x5000]]);
        assert_eq!(
            model.borrow().as_slice(),
            &[vec![a + 0x3c, 0x5000], vec![a + 0x3c, 0x5000]]
        );
        assert_eq!(
            destruction.borrow().as_slice(),
            &[vec![a + 0x58, 0x5000], vec![a + 0x58, 0x5000]]
        );
        assert_eq!(data.borrow().as_slice(), &[vec![a, 0x5000, a + 0x60, 0x54]]);
        // The data chunk is swapped (14 numbers) because the file swaps.
        assert_eq!(swaps.borrow().len(), 14);
        assert_eq!(path.borrow().as_slice(), b"flash.nif");
        assert_eq!(texture.borrow().as_slice(), &[vec![a + 0xb4, 0x5000]]);
        assert_eq!(e.mem.u32(a + 0xcc), 2);
    }

    #[test]
    fn load_clamps_the_sound_level_and_skips_empty_model_chunks() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        load_doubles(
            &mut e,
            0x33,
            false,
            vec![
                (CHUNK_NAM1, vec![]),
                (CHUNK_NAM2, vec![]),
                (CHUNK_VNAM, vec![4, 0, 0, 0]),
                (CHUNK_DATA, vec![0; 4]),
            ],
        );
        give_vtable(&mut e, p.addr(), &[(FORM_CLEAR_DATA_SLOT, 0x00dd_0018)]);
        quiet(&mut e, &[0x00dd_0018, FORM_LOAD_DATA]);
        let texture = recording(&mut e, MODEL_LOAD_TEXTURE_CHUNK);
        let swaps = recording(&mut e, SWAP_WORD);
        assert!(e.call(0x004f_d6f0, &args![p, 0x5000u32]).bool());
        assert_eq!(e.mem.u32(p.addr() + 0xcc), 1);
        assert!(texture.borrow().is_empty());
        assert!(swaps.borrow().is_empty());
    }

    const POINTER_OFFSETS: [u32; 7] = [0x70, 0x74, 0x84, 0x88, 0x98, 0x9c, 0xa0];

    #[test]
    fn save_writes_ids_and_restores_the_pointers() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        let a = p.addr();
        quiet(
            &mut e,
            &[
                FORM_START_FORM,
                FORM_CLOSE_FORM,
                FULL_NAME_SAVE,
                DESTRUCTIBLE_SAVE,
            ],
        );
        e.register(GET_FORM_ID, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        returning(&mut e, SWAP_ON_SAVE, 1);
        let swaps = recording(&mut e, SWAP_WORD);
        returning(&mut e, MODEL_PATH_LENGTH, 4);
        let models = recording(&mut e, MODEL_SAVE);
        let words = recording(&mut e, ADD_CHUNK_WORD);
        let seen = Rc::new(RefCell::new(Vec::<(Vec<u32>, Vec<u32>)>::new()));
        let sink = seen.clone();
        e.register_double(FORM_SAVE_DATA, move |e, args| {
            let ids = POINTER_OFFSETS
                .iter()
                .map(|o| e.mem.u32(args[0] + o))
                .collect();
            sink.borrow_mut().push((args.to_vec(), ids));
            Ret::default()
        });
        for (i, offset) in POINTER_OFFSETS.iter().enumerate() {
            e.mem
                .set_u32(a + offset, if i == 1 { 0 } else { 0x4000 + i as u32 });
        }
        e.mem.set_u32(a + 0xcc, 3);
        e.call(0x004f_da00, &args![p]);
        let (call, ids) = seen.borrow()[0].clone();
        assert_eq!(call, vec![a, a + 0x60, 0x54]);
        let expected: Vec<u32> = (0..7u32)
            .map(|i| if i == 1 { 0 } else { 0x4000 + i + 0x100 })
            .collect();
        assert_eq!(ids, expected);
        for (i, offset) in POINTER_OFFSETS.iter().enumerate() {
            let want = if i == 1 { 0 } else { 0x4000 + i as u32 };
            assert_eq!(e.mem.u32(a + offset), want);
        }
        assert_eq!(swaps.borrow().len(), 28);
        assert_eq!(
            models.borrow().as_slice(),
            &[
                vec![a + 0x3c, CHUNK_MODL, CHUNK_MODT],
                vec![a + 0xb4, CHUNK_NAM1, CHUNK_NAM2]
            ]
        );
        assert_eq!(words.borrow().as_slice(), &[vec![CHUNK_VNAM, 3]]);
    }

    #[test]
    fn save_writes_an_empty_muzzle_flash_path_as_one_byte() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        let a = p.addr();
        quiet(
            &mut e,
            &[
                FORM_START_FORM,
                FORM_CLOSE_FORM,
                FULL_NAME_SAVE,
                DESTRUCTIBLE_SAVE,
                MODEL_SAVE,
                FORM_SAVE_DATA,
                ADD_CHUNK_WORD,
            ],
        );
        returning(&mut e, SWAP_ON_SAVE, 0);
        returning(&mut e, MODEL_PATH_LENGTH, 0);
        give_vtable(&mut e, a + 0xb4, &[(MODEL_GET_PATH_SLOT, 0x00dd_0114)]);
        returning(&mut e, 0x00dd_0114, 0x6666);
        let arrays = recording(&mut e, ADD_CHUNK_ARRAY);
        let swaps = recording(&mut e, SWAP_WORD);
        e.call(0x004f_da00, &args![p]);
        assert_eq!(arrays.borrow().as_slice(), &[vec![CHUNK_NAM1, 0x6666, 1]]);
        assert!(swaps.borrow().is_empty());
    }

    #[test]
    fn copy_copies_components_model_level_and_data() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        let other = projectile(&mut e);
        flag_doubles(&mut e);
        let cast = recording_returning(&mut e, RT_DYNAMIC_CAST, other.addr());
        let components = recording(&mut e, FORM_COPY_ALL_COMPONENTS);
        let copy = recording(&mut e, MEMCPY);
        give_vtable(&mut e, p.addr() + 0xb4, &[(MODEL_COPY_SLOT, 0x00dd_0008)]);
        let model = recording(&mut e, 0x00dd_0008);
        e.mem.set_u32(other.addr() + 0xcc, 3);
        e.call(0x004f_dcf0, &args![p, 0x9000u32]);
        assert_eq!(
            cast.borrow().as_slice(),
            &[vec![0x9000, 0, TYPE_TES_FORM, TYPE_BGS_PROJECTILE, 0]]
        );
        assert_eq!(
            components.borrow().as_slice(),
            &[vec![p.addr(), other.addr()]]
        );
        assert_eq!(
            model.borrow().as_slice(),
            &[vec![p.addr() + 0xb4, other.addr() + 0xb4]]
        );
        assert_eq!(e.mem.u32(p.addr() + 0xcc), 3);
        assert_eq!(
            copy.borrow().as_slice(),
            &[vec![p.addr() + 0x60, other.addr() + 0x60, 0x54]]
        );
    }

    #[test]
    fn copy_ignores_other_forms() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        returning(&mut e, RT_DYNAMIC_CAST, 0);
        let components = recording(&mut e, FORM_COPY_ALL_COMPONENTS);
        e.call(0x004f_dcf0, &args![p, 0x9000u32]);
        assert!(components.borrow().is_empty());
    }

    #[test]
    fn compare_checks_each_part_in_turn() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        let other = projectile(&mut e);
        flag_doubles(&mut e);
        returning(&mut e, RT_DYNAMIC_CAST, 0);
        assert!(e.call(0x004f_dd80, &args![p, 0x9000u32]).bool());
        returning(&mut e, RT_DYNAMIC_CAST, other.addr());
        returning(&mut e, FORM_COMPARE_ALL_COMPONENTS, 1);
        assert!(e.call(0x004f_dd80, &args![p, 0x9000u32]).bool());
        returning(&mut e, FORM_COMPARE_ALL_COMPONENTS, 0);
        returning(&mut e, MEMCMP, 1);
        assert!(e.call(0x004f_dd80, &args![p, 0x9000u32]).bool());
        returning(&mut e, MEMCMP, 0);
        give_vtable(
            &mut e,
            p.addr() + 0xb4,
            &[(MODEL_COMPARE_SLOT, 0x00dd_000c)],
        );
        returning(&mut e, 0x00dd_000c, 1);
        assert!(e.call(0x004f_dd80, &args![p, 0x9000u32]).bool());
        returning(&mut e, 0x00dd_000c, 0);
        e.register(GET_WORD_CC, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xcc),
            ..Ret::default()
        });
        e.mem.set_u32(p.addr() + 0xcc, 1);
        e.mem.set_u32(other.addr() + 0xcc, 2);
        assert!(e.call(0x004f_dd80, &args![p, 0x9000u32]).bool());
        e.mem.set_u32(other.addr() + 0xcc, 1);
        assert!(!e.call(0x004f_dd80, &args![p, 0x9000u32]).bool());
    }

    #[test]
    fn flag_tests_ask_for_0x20_and_0x40() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        flag_doubles(&mut e);
        e.mem.set_u32(p.addr() + 0x60, 0x20);
        assert!(e.call(0x004f_df20, &args![p]).bool());
        assert!(!e.call(0x004f_df40, &args![p]).bool());
        e.mem.set_u32(p.addr() + 0x60, 0x40);
        assert!(!e.call(0x004f_df20, &args![p]).bool());
        assert!(e.call(0x004f_df40, &args![p]).bool());
    }

    /// Doubles for the hit check: every cast succeeds and the base form is
    /// `base`.
    fn hit_doubles(e: &mut Engine, base: Ptr<BGSProjectile>) {
        flag_doubles(e);
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        returning(e, GET_BASE_FORM, base.addr());
        returning(e, REFERENCE_FLAGS_TEST, 0);
    }

    #[test]
    fn hit_check_accepts_through_the_projectile_slot() {
        let mut e = Engine::new();
        map_pages(&mut e, &[0, 0x4000, 0x5000, 0x8000]);
        let p = projectile(&mut e);
        let base = projectile(&mut e);
        hit_doubles(&mut e, base);
        e.mem.set_u32(base.addr() + 0x60, 0x20);
        give_vtable(&mut e, 0x4000, &[(PROJECTILE_HIT_SLOT, 0x00dd_0320)]);
        let hits = recording_returning(&mut e, 0x00dd_0320, 1);
        let apply = recording(&mut e, BOUND_OBJECT_APPLY);
        let call = args![p, 0x4000u32, 0x5000u32, 1u32, 2u32, 3u32];
        assert!(e.call(0x004f_de40, &call).bool());
        assert_eq!(hits.borrow().as_slice(), &[vec![0x4000, 0x5000, 0]]);
        assert!(apply.borrow().is_empty());
    }

    #[test]
    fn hit_check_falls_through_to_the_bound_object_apply() {
        let mut e = Engine::new();
        map_pages(&mut e, &[0, 0x4000, 0x5000, 0x8000]);
        let p = projectile(&mut e);
        let base = projectile(&mut e);
        hit_doubles(&mut e, base);
        e.mem.set_u32(base.addr() + 0x60, 0x60);
        give_vtable(&mut e, 0x4000, &[(PROJECTILE_HIT_SLOT, 0x00dd_0320)]);
        recording_returning(&mut e, 0x00dd_0320, 0);
        let apply = recording_returning(&mut e, BOUND_OBJECT_APPLY, 1);
        let call = args![p, 0x4000u32, 0x5000u32, 1u32, 2u32, 3u32];
        assert!(e.call(0x004f_de40, &call).bool());
        assert_eq!(
            apply.borrow().as_slice(),
            &[vec![p.addr(), 0x4000, 0x5000, 1, 2, 3]]
        );
        // Neither flag: false.
        e.mem.set_u32(base.addr() + 0x60, 0);
        assert!(!e.call(0x004f_de40, &call).bool());
        // A reference that does not cast: false.
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: if a[0] == 0x5000 { 0 } else { a[0] },
            ..Ret::default()
        });
        e.mem.set_u32(base.addr() + 0x60, 0x40);
        assert!(!e.call(0x004f_de40, &call).bool());
    }

    #[test]
    fn tracer_roll_compares_a_random_number_modulo_100() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        map_pages(&mut e, &[TRACER_CHANCE_SCALE]);
        e.set_global(TRACER_CHANCE_SCALE, 100.0f64);
        returning_float(&mut e, GET_FLOAT_78, 0.25);
        ftol_double(&mut e);
        returning(&mut e, RANDOM, 124);
        assert!(e.call(0x004f_df60, &args![p]).bool());
        returning(&mut e, RANDOM, 125);
        assert!(!e.call(0x004f_df60, &args![p]).bool());
        returning_float(&mut e, GET_FLOAT_78, 0.0);
        returning(&mut e, RANDOM, 0);
        assert!(!e.call(0x004f_df60, &args![p]).bool());
    }

    #[test]
    fn queue_files_queues_both_models_and_the_explosion() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        let a = p.addr();
        map_pages(&mut e, &[MODEL_QUEUE_OBJECT]);
        e.set_global(MODEL_QUEUE_OBJECT, 0x1234u32);
        returning(&mut e, MODEL_PATH_LENGTH, 5);
        returning(&mut e, FORM_LOD_MULT, 2);
        let queue = recording(&mut e, QUEUE_MODEL);
        let explosion = e.mem.alloc(0x100);
        give_vtable(
            &mut e,
            explosion + 0x64,
            &[(PRELOADABLE_QUEUE_FILES_SLOT, 0x00dd_0010)],
        );
        returning(&mut e, GET_EXPLOSION, explosion);
        let files = recording(&mut e, 0x00dd_0010);
        e.call(0x004f_dfb0, &args![a + 0x54, 7u32, 8u32]);
        assert_eq!(
            queue.borrow().as_slice(),
            &[
                vec![0x1234, a + 0x3c, 7, 8, 2, 1, 0, 0],
                vec![0x1234, a + 0xb4, 7, 8, 3, 1, 0, 0]
            ]
        );
        assert_eq!(files.borrow().as_slice(), &[vec![explosion + 0x64, 7, 8]]);
    }

    #[test]
    fn queue_files_skips_empty_models_and_a_missing_explosion() {
        let mut e = Engine::new();
        let p = projectile(&mut e);
        map_pages(&mut e, &[MODEL_QUEUE_OBJECT]);
        returning(&mut e, MODEL_PATH_LENGTH, 0);
        returning(&mut e, GET_EXPLOSION, 0);
        let queue = recording(&mut e, QUEUE_MODEL);
        e.call(0x004f_dfb0, &args![p.addr() + 0x54, 7u32, 8u32]);
        assert!(queue.borrow().is_empty());
    }

    fn count_doubles(e: &mut Engine, kind: u32, flags: u32, clip: u32) -> Ptr<BGSProjectile> {
        let p = projectile(e);
        flag_doubles(e);
        e.mem.set_u32(p.addr() + 0x60, kind | flags);
        returning_float(e, WEAPON_FLOAT_134, 2.5);
        ftol_double(e);
        returning(e, WEAPON_GET_NUM_PROJECTILES, 2);
        e.register(MIN_INT, |_, a| Ret {
            eax: (a[0] as i32).min(a[1] as i32) as u32,
            ..Ret::default()
        });
        returning(e, CLIP_ROUNDS_GET, clip);
        returning_float(e, WEAPON_GET_MOD_EFFECT_VALUE, 0.0);
        p
    }

    #[test]
    fn pre_clone_count_special_cases() {
        let mut e = Engine::new();
        for kind in [TYPE_0X40000, TYPE_0X100000, TYPE_0X80000] {
            let p = count_doubles(&mut e, kind, 0, 100);
            assert_eq!(e.call(0x004f_e080, &args![p, 0x5000u32]).i32(), 0);
        }
        let p = count_doubles(&mut e, 0, 0, 100);
        assert_eq!(e.call(0x004f_e080, &args![p, 0u32]).i32(), 5);
    }

    #[test]
    fn pre_clone_count_scales_limits_and_multiplies() {
        let mut e = Engine::new();
        // 2 (2.5 truncated) * 2 projectiles * 3 = 12, under the clip of 100.
        let p = count_doubles(&mut e, 0, 0, 100);
        let weapon = e.mem.alloc(0x200);
        assert_eq!(e.call(0x004f_e080, &args![p, weapon]).i32(), 12);
        // Flag 0x100 multiplies by four.
        let p = count_doubles(&mut e, 0, FLAG_0X100, 100);
        assert_eq!(e.call(0x004f_e080, &args![p, weapon]).i32(), 48);
        // A small clip limits the count; type 0x20000 ignores the clip.
        let p = count_doubles(&mut e, 0, 0, 5);
        assert_eq!(e.call(0x004f_e080, &args![p, weapon]).i32(), 5);
        let p = count_doubles(&mut e, TYPE_0X20000, 0, 5);
        assert_eq!(e.call(0x004f_e080, &args![p, weapon]).i32(), 12);
    }

    #[test]
    fn clip_rounds_adds_the_truncated_mod_effect_byte() {
        let mut e = Engine::new();
        let weapon = e.mem.alloc(0x200);
        let getter = recording_returning(&mut e, CLIP_ROUNDS_GET, 30);
        returning_float(&mut e, WEAPON_GET_MOD_EFFECT_VALUE, 4.9);
        assert_eq!(e.call(0x004f_e160, &args![weapon, 0u32]).i32(), 30);
        assert_eq!(e.call(0x004f_e160, &args![weapon, 1u32]).i32(), 34);
        assert_eq!(getter.borrow()[0], vec![weapon + 0xac]);
        // The bonus is the low byte of the integer.
        returning_float(&mut e, WEAPON_GET_MOD_EFFECT_VALUE, 256.0);
        assert_eq!(e.call(0x004f_e160, &args![weapon, 1u32]).i32(), 30);
    }
}

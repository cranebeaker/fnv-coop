//! `fallout shared/tesobjectweap.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESObjectWEAP` is the weapon form. On PC `TESForm` is 0x18 bytes, so every
//! field of the class sits 0x10 lower than the Xbox PDB says (`data` is at
//! +0x104 there, +0xF4 here); each offset below was checked against the PC
//! code that uses it. The embedded components keep their PDB order:
//! `TESFullName` +0x30, the main `TESModelTextureSwap` +0x3C, `TESIcon` +0x5C,
//! `TESScriptableForm` +0x68, `TESEnchantableForm` +0x74, `TESValueForm`
//! +0x84, `BGSAmmoForm` +0xA4, `BGSDestructibleObjectForm` +0xB4,
//! `BGSRepairItemList` +0xBC, `BGSEquipType` +0xC4, `BGSPreloadable` +0xCC,
//! `BGSMessageIcon` +0xD0, `BGSBipedModelList` +0xE0 and
//! `BGSPickupPutdownSounds` +0xE8.
//!
//! The ten `TESModel` objects the weapon carries are at +0x1D0 (shell
//! casing), +0x1E8 (scope), +0x200 (world model) and, as
//! `TESModelTextureSwap`, +0x270 (mod 1), +0x290 (mod 2), +0x2B0 (mod 3),
//! +0x2D0 (mods 1+2), +0x2F0 (mods 1+3), +0x310 (mods 2+3) and +0x330 (all
//! three). The mod number of a model is a bit mask of the weapon's three mod
//! slots (1, 2 and 4).
//!
//! Compiler artifacts that are not translated: the `__alloca_probe_16`
//! stack buffers of `Load` are heap buffers here, and the exception frames and
//! `__security_check_cookie` of `Load`, `ExpelShellCasing` and the firing
//! function are left out (each such function says so).

#[allow(unused_imports)]
use crate::prelude::*;

// ---- Layouts ---------------------------------------------------------------

layout! {
    /// `OBJ_WEAP` (Xbox PDB): the weapon's DATA block, 0xCC bytes, embedded at
    /// +0xF4 of `TESObjectWEAP` on PC. Only the fields the translated code
    /// reads (offsets as the PDB, confirmed against the PC code).
    pub struct ObjWeap: 0xCC {
        /// `eType` (Xbox PDB): the weapon type, a signed `char`.
        0x00 eType: i8,
        /// `fSpeed` (Xbox PDB).
        0x04 fSpeed: f32,
        /// `cFlags` (Xbox PDB).
        0x0C cFlags: u8,
        /// `cHandGripAnim` (Xbox PDB).
        0x0D cHandGripAnim: u8,
        /// `cAmmoPerShot` (Xbox PDB).
        0x0E cAmmoPerShot: u8,
        /// `cReloadAnim` (Xbox PDB).
        0x0F cReloadAnim: u8,
        /// `fMinSpread` (Xbox PDB).
        0x10 fMinSpread: f32,
        /// `fSpread` (Xbox PDB).
        0x14 fSpread: f32,
        /// `pProjectile` (Xbox PDB): a form id until `InitItem` resolves it.
        0x24 pProjectile: Ptr,
        /// `cAttackAnim` (Xbox PDB).
        0x29 cAttackAnim: u8,
        /// `cNumProjectiles` (Xbox PDB).
        0x2A cNumProjectiles: u8,
        /// `iFlagsEx` (Xbox PDB); bit 1 (value 2) is set by `0051f590`.
        0x38 iFlagsEx: u32,
        /// `fAttackMult` (Xbox PDB).
        0x3C fAttackMult: f32,
        /// `eSkill` (Xbox PDB): an `ActorValue::Index`.
        0x68 eSkill: u32,
        /// `fLimbDamageMult` (Xbox PDB).
        0x74 fLimbDamageMult: f32,
        /// `eResistance` (Xbox PDB): an `ActorValue::Index`.
        0x78 eResistance: u32,
        /// `fSemiAutomaticFireDelayMin` (Xbox PDB).
        0x80 fSemiAutomaticFireDelayMin: f32,
        /// `fSemiAutomaticFireDelayMax` (Xbox PDB).
        0x84 fSemiAutomaticFireDelayMax: f32,
    }

    /// `OBJ_WEAP_CRITICAL` (Xbox PDB): 0x10 bytes, embedded at +0x1C0.
    pub struct ObjWeapCritical: 0x10 {
        /// `pEffect` (Xbox PDB): a form id until `InitItem` resolves it.
        0x0C pEffect: Ptr,
    }

    /// `OBJ_WEAP_VATS_SPECIAL` (Xbox PDB): 0x14 bytes, embedded at +0x370.
    pub struct ObjWeapVatsSpecial: 0x14 {
        /// `pVATSSpecialEffect` (Xbox PDB): a form id until `InitItem`
        /// resolves it.
        0x00 pVATSSpecialEffect: Ptr,
    }

    /// `TESObjectWEAP` (Xbox PDB) on PC: 0x388 bytes (0x398 on the Xbox).
    pub struct TesObjectWeap: 0x388 {
        /// `TESForm::iFormFlags` (Xbox PDB).
        0x08 iFormFlags: u32,
        /// `TESForm::iFormID` (Xbox PDB).
        0x0C iFormID: u32,
        /// `BGSAmmoForm::pFormAmmo` (Xbox PDB).
        0xA8 pFormAmmo: Ptr,
        /// `data` (Xbox PDB).
        0xF4 data: Inline<ObjWeap>,
        /// `criticalData` (Xbox PDB).
        0x1C0 criticalData: Inline<ObjWeapCritical>,
        /// `pScopeEffect` (Xbox PDB): a `TESEffectShader`.
        0x218 pScopeEffect: Ptr,
        /// `pAttackSound` (Xbox PDB): a `TESSound`.
        0x21C pAttackSound: Ptr,
        /// `pAttackSound2` (Xbox PDB).
        0x220 pAttackSound2: Ptr,
        /// `pAttackSound2D` (Xbox PDB).
        0x224 pAttackSound2D: Ptr,
        /// `pAttackLoop` (Xbox PDB).
        0x228 pAttackLoop: Ptr,
        /// `pAttackFailSound` (Xbox PDB).
        0x22C pAttackFailSound: Ptr,
        /// `pMeleeBlockSound` (Xbox PDB).
        0x230 pMeleeBlockSound: Ptr,
        /// `pIdleSound` (Xbox PDB).
        0x234 pIdleSound: Ptr,
        /// `pEquipSound` (Xbox PDB).
        0x238 pEquipSound: Ptr,
        /// `pUnEquipSound` (Xbox PDB).
        0x23C pUnEquipSound: Ptr,
        /// `pModSilencedSound` (Xbox PDB).
        0x240 pModSilencedSound: Ptr,
        /// `pModSilencedSound2` (Xbox PDB).
        0x244 pModSilencedSound2: Ptr,
        /// `pModSilencedSound2D` (Xbox PDB).
        0x248 pModSilencedSound2D: Ptr,
        /// `pImpactDataSet` (Xbox PDB): a `BGSImpactDataSet`.
        0x24C pImpactDataSet: Ptr,
        /// `p1stPersonObject` (Xbox PDB): a `TESObjectSTAT`.
        0x250 p1stPersonObject: Ptr,
        /// `p1stPersonMod1Object` (Xbox PDB).
        0x254 p1stPersonMod1Object: Ptr,
        /// `p1stPersonMod2Object` (Xbox PDB).
        0x258 p1stPersonMod2Object: Ptr,
        /// `p1stPersonMod3Object` (Xbox PDB).
        0x25C p1stPersonMod3Object: Ptr,
        /// `p1stPersonMod12Object` (Xbox PDB).
        0x260 p1stPersonMod12Object: Ptr,
        /// `p1stPersonMod23Object` (Xbox PDB).
        0x264 p1stPersonMod23Object: Ptr,
        /// `p1stPersonMod13Object` (Xbox PDB).
        0x268 p1stPersonMod13Object: Ptr,
        /// `p1stPersonMod123Object` (Xbox PDB).
        0x26C p1stPersonMod123Object: Ptr,
        /// `pModObjectOne` (Xbox PDB): a `TESObjectIMOD`.
        0x350 pModObjectOne: Ptr,
        /// `pModObjectTwo` (Xbox PDB).
        0x354 pModObjectTwo: Ptr,
        /// `pModObjectThree` (Xbox PDB).
        0x358 pModObjectThree: Ptr,
        /// `eSoundLevel` (Xbox PDB).
        0x364 eSoundLevel: u32,
        /// `specialVATSData` (Xbox PDB).
        0x370 specialVATSData: Inline<ObjWeapVatsSpecial>,
        /// `bIsLoopingReload` (Xbox PDB).
        0x384 bIsLoopingReload: u8,
    }
}

/// Offsets of the embedded components inside `TESObjectWEAP` (see the module
/// documentation).
pub(crate) const FULL_NAME: u32 = 0x30;
pub(crate) const MODEL: u32 = 0x3C;
pub(crate) const ICON: u32 = 0x5C;
pub(crate) const SCRIPTABLE_FORM: u32 = 0x68;
pub(crate) const ENCHANTABLE_FORM: u32 = 0x74;
pub(crate) const VALUE_FORM: u32 = 0x84;
pub(crate) const AMMO_FORM: u32 = 0xA4;
pub(crate) const DESTRUCTIBLE_FORM: u32 = 0xB4;
pub(crate) const REPAIR_ITEM_LIST: u32 = 0xBC;
pub(crate) const EQUIP_TYPE: u32 = 0xC4;
pub(crate) const PRELOADABLE: u32 = 0xCC;
pub(crate) const MESSAGE_ICON: u32 = 0xD0;
pub(crate) const BIPED_MODEL_LIST: u32 = 0xE0;
pub(crate) const PICKUP_PUTDOWN_SOUNDS: u32 = 0xE8;
/// `data` (`OBJ_WEAP`), `criticalData` and `specialVATSData`.
pub(crate) const DATA: u32 = 0xF4;
pub(crate) const CRITICAL_DATA: u32 = 0x1C0;
pub(crate) const VATS_SPECIAL_DATA: u32 = 0x370;
/// The shell casing, scope and world `TESModel` objects.
pub(crate) const SHELL_CASING_MODEL: u32 = 0x1D0;
pub(crate) const SCOPE_MODEL: u32 = 0x1E8;
pub(crate) const WORLD_MODEL: u32 = 0x200;
/// World models for mod slots 1, 2, 3, 1+2, 1+3, 2+3 and 1+2+3.
pub(crate) const WORLD_MODEL_MOD_1: u32 = 0x270;
pub(crate) const WORLD_MODEL_MOD_2: u32 = 0x290;
pub(crate) const WORLD_MODEL_MOD_3: u32 = 0x2B0;
pub(crate) const WORLD_MODEL_MOD_12: u32 = 0x2D0;
pub(crate) const WORLD_MODEL_MOD_13: u32 = 0x2F0;
pub(crate) const WORLD_MODEL_MOD_23: u32 = 0x310;
pub(crate) const WORLD_MODEL_MOD_123: u32 = 0x330;
/// `EmbeddedWeaponNode` and `VATSAttackName` (`BSStringT<char>`, 8 bytes).
pub(crate) const EMBEDDED_WEAPON_NODE: u32 = 0x35C;
pub(crate) const VATS_ATTACK_NAME: u32 = 0x368;

/// The `OBJ_WEAP` DATA block of a weapon.
pub(crate) fn weapon_data(this: Ptr<TesObjectWeap>) -> Ptr<ObjWeap> {
    this.at(TesObjectWeap::data)
}

// ---- Calls outside this file --------------------------------------------------

/// `TESForm` flag test (`iFormFlags & 8`, `AL`); `InitItem` stops when set.
const FORM_FLAG_8_TEST: u32 = 0x004013e0;
/// `BGSBipedModelList::InitItem` (Xbox PDB), `thiscall` (weapon).
const BIPED_MODEL_LIST_INIT_ITEM: u32 = 0x00474da0;
/// `TESScriptableForm::InitItem` (Xbox PDB), `thiscall` (weapon).
const SCRIPTABLE_FORM_INIT_ITEM: u32 = 0x0048cd90;
/// `TESEnchantableForm::InitItem` (Xbox PDB), `thiscall` (weapon).
const ENCHANTABLE_FORM_INIT_ITEM: u32 = 0x00483210;
/// `BGSAmmoForm::InitItem` (Xbox PDB), `thiscall` (weapon).
const AMMO_FORM_INIT_ITEM: u32 = 0x00474bc0;
/// `BGSDestructibleObjectForm::InitItem` (Xbox PDB), `thiscall` (weapon).
const DESTRUCTIBLE_FORM_INIT_ITEM: u32 = 0x00478be0;
/// `TESModelTextureSwap::InitItem` (Xbox PDB), `thiscall` (weapon).
const MODEL_TEXTURE_SWAP_INIT_ITEM: u32 = 0x0048aa80;
/// `BGSRepairItemList::InitItem` (Xbox PDB), `thiscall` (weapon).
const REPAIR_ITEM_LIST_INIT_ITEM: u32 = 0x0047b8e0;
/// `BGSPickupPutdownSounds::InitItem` (Xbox PDB), `thiscall` (weapon).
const PICKUP_PUTDOWN_SOUNDS_INIT_ITEM: u32 = 0x0047b6a0;
/// The weapon's projectile (`thiscall`, `this` and an actor, `RET 4`; no name
/// in the engine map): `GetCurrentAmmo(actor)`'s projectile (`008d80e0` on the
/// ammo) when there is one, else `data.pProjectile` (`this + 0x118`).
const GET_PROJECTILE: u32 = 0x00525a90;
/// `TESForm::GetFile` (Xbox PDB), `thiscall` (form, `-1`).
const FORM_GET_FILE: u32 = 0x00484e60;
/// `TESForm::AddCompileIndex` (Xbox PDB), `cdecl` (pointer to the form id,
/// file): rewrites the id to the load order of the running game.
const FORM_ADD_COMPILE_INDEX: u32 = 0x00485d50;
/// Form lookup by id (`cdecl`, one word).
const LOOKUP_FORM: u32 = 0x004839c0;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec43fb;
/// `iFormID` getter (`thiscall`; the engine map names the folded body
/// `0084e3a0`).
const GET_FORM_ID: u32 = 0x0084e3a0;
/// Virtual slot `0x130` of a form: its object type name (`GetObjectTypeName`
/// in the Xbox PDB).
const FORM_GET_OBJECT_TYPE_NAME_SLOT: u32 = 0x130;
/// Logging function (`cdecl`, format and arguments).
const LOG_MESSAGE: u32 = 0x005b5e40;
/// Getter of the weapon's scope effect (`this`, +0x218) and its setter
/// (`this`, value).
const GET_SCOPE_EFFECT: u32 = 0x0051f550;
const SET_SCOPE_EFFECT: u32 = 0x0051f570;
/// Getter and setter of the critical-hit effect (`this`, +0x1CC) and of the
/// VATS special effect (+0x370).
const GET_CRITICAL_EFFECT: u32 = 0x0051f4d0;
const GET_VATS_SPECIAL_EFFECT: u32 = 0x0051f510;
const SET_VATS_SPECIAL_EFFECT: u32 = 0x0051f530;
/// `thiscall` on a setting object: the pointer to its value
/// (`00408d60`).
const SETTING_VALUE_POINTER: u32 = 0x00408d60;
/// `TESForm` setter of the "initialized" bit (bit 3 of `iFormFlags`),
/// `thiscall` (weapon, value).
const FORM_SET_FLAG_8: u32 = 0x00484ab0;

/// `TESForm` type `AL` of a weapon reference test: virtual slot `0x100`
/// (`IsActor` in the Xbox PDB) of a reference.
const REFERENCE_IS_ACTOR_SLOT: u32 = 0x100;
/// The player character pointer.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;

/// `TESObjectWEAP::GetModEffectValue` (Xbox PDB), `thiscall` (mod effect,
/// `0`), result in `ST0`.
const GET_MOD_EFFECT_VALUE: u32 = 0x004bcf60;
/// `max(a, b)` of two `float`s (`cdecl`, result in `ST0`; `effectitem.cpp`,
/// no name in the engine map).
const MAX_FLOAT: u32 = 0x00404010;
/// `thiscall` on the weapon: its type (`data.eType`, sign-extended).
const GET_WEAPON_TYPE: u32 = 0x00446390;
/// The model-getter slot of a `TESModel` (`GetModel` in the Xbox PDB): the
/// path as a `char*`.
const MODEL_GET_MODEL_SLOT: u32 = 0x14;
/// The model-setter slot of a `TESModel` (`SetModel` in the Xbox PDB).
const MODEL_SET_MODEL_SLOT: u32 = 0x18;
/// The `TESModel` the weapon uses when no mod model applies: a form id
/// (`0x001735d4`) the model choice treats as "no mod model".
const NO_MOD_MODEL_FORM_ID: u32 = 0x0017_35d4;

/// `BGSExplosion::GetRadiusBSUnits` (Xbox PDB), `thiscall`, result in `ST0`.
const EXPLOSION_GET_RADIUS: u32 = 0x00477900;
/// `thiscall` on a projectile-like object: its explosion (`004fd3c0`).
const PROJECTILE_GET_EXPLOSION: u32 = 0x004fd3c0;
/// `thiscall` on an explosion: the `float` at `+0x78`, in `ST0` (the engine
/// map files `006a78f0` under `navmeshsearch.cpp`; that it is the damage is
/// inferred from the setting it is compared with, unconfirmed).
const EXPLOSION_GET_DAMAGE: u32 = 0x006a78f0;
/// `thiscall` on a setting object: a pointer to its `float` value.
const SETTING_GET_FLOAT_POINTER: u32 = 0x00403e20;
/// The game settings `fDangerousProjectileExplosionDamage` and
/// `fDangerousProjectileExplosionRadius` the combat weapon type compares
/// against (names from the exe's static initializers of the settings).
const SETTING_DANGEROUS_PROJECTILE_EXPLOSION_DAMAGE: u32 = 0x011c_f3f0;
const SETTING_DANGEROUS_PROJECTILE_EXPLOSION_RADIUS: u32 = 0x011c_ea74;
/// `"COMBAT: Unknown weapon type %i encountered in function
/// TESObjectWEAP::GetCombatWeaponType()."`
const UNKNOWN_WEAPON_TYPE_MESSAGE: u32 = 0x0102_ca98;

// ---- Small accessors -----------------------------------------------------------

// Translated from 0051f4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `data.pProjectile` (`this + 0x118`).
pub fn fn_0051f4b0(e: &mut Engine, this: Ptr<TesObjectWeap>, value: Ptr) {
    e.set(weapon_data(this), ObjWeap::pProjectile, value);
}

// Translated from 0051f4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `criticalData.pEffect` (`this + 0x1CC`).
pub fn fn_0051f4f0(e: &mut Engine, this: Ptr<TesObjectWeap>, value: Ptr) {
    e.set(
        this.at(TesObjectWeap::criticalData),
        ObjWeapCritical::pEffect,
        value,
    );
}

// Translated from 0051f590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`enable`) or clears bit 1 (value 2) of `data.iFlagsEx`
/// (`this + 0x12C`).
pub fn fn_0051f590(e: &mut Engine, this: Ptr<TesObjectWeap>, enable: bool) {
    let data = weapon_data(this);
    let flags = e.get(data, ObjWeap::iFlagsEx);
    let flags = if enable { flags | 2 } else { flags & !2 };
    e.set(data, ObjWeap::iFlagsEx, flags);
}

// Translated from 00521580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `data.cNumProjectiles` (`this + 0x11E`).
pub fn fn_00521580(e: &mut Engine, this: Ptr<TesObjectWeap>, value: u8) {
    e.set(weapon_data(this), ObjWeap::cNumProjectiles, value);
}

// ---- InitItem ---------------------------------------------------------------------

/// The reference resolution `InitItem` repeats for every form the weapon
/// points at: the id is moved to the running game's load order with the
/// weapon's file (`TESForm::GetFile(-1)`, `AddCompileIndex`), looked up by
/// id, and cast to the expected class. Returns the cast pointer and the
/// rewritten id.
fn resolve_form_reference(
    e: &mut Engine,
    this: Ptr<TesObjectWeap>,
    id: u32,
    target_type: u32,
) -> (Ptr, u32) {
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
        .ptr();
    (cast, id)
}

/// The log line `InitItem` writes when a reference did not resolve: the
/// message, the rewritten id, the weapon's object type name (virtual slot
/// `0x130`) and the weapon's own form id.
fn report_missing_reference(e: &mut Engine, this: Ptr<TesObjectWeap>, id: u32, message: u32) {
    let form_id = e.call(GET_FORM_ID, &args![this]).u32();
    let name = e
        .vcall(this.addr(), FORM_GET_OBJECT_TYPE_NAME_SLOT, &[])
        .u32();
    e.call(LOG_MESSAGE, &args![message, id, name, form_id]);
}

/// `TypeDescriptor`s `InitItem` casts to (`.?AV...@@` names read from the
/// exe): the source is always `TESForm`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_BGS_PROJECTILE: u32 = 0x0118_6228;
const TYPE_TES_EFFECT_SHADER: u32 = 0x0118_3918;
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;
const TYPE_TES_SOUND: u32 = 0x0118_32fc;
const TYPE_BGS_IMPACT_DATA_SET: u32 = 0x0118_62dc;
const TYPE_TES_OBJECT_STAT: u32 = 0x0118_6568;
const TYPE_TES_OBJECT_IMOD: u32 = 0x0118_a754;

/// The "MASTERFILE: Could not find ..." messages (`%08X`, `'%s'`, `(%08X)`).
const MESSAGE_PROJECTILE: u32 = 0x0102_ca18;
const MESSAGE_SCOPE_EFFECT: u32 = 0x0102_c9d0;
const MESSAGE_SPELL_ITEM: u32 = 0x0102_c988;
const MESSAGE_VATS_SPELL_ITEM: u32 = 0x0102_c938;
const MESSAGE_OPEN_SOUND: u32 = 0x0102_c8f0;
const MESSAGE_OPEN_SOUND_2: u32 = 0x0102_c8a8;
const MESSAGE_SOUND: u32 = 0x0102_c868;
const MESSAGE_IDLE_SOUND: u32 = 0x0102_c820;
const MESSAGE_EQUIP_SOUND: u32 = 0x0102_c7d8;
const MESSAGE_UNEQUIP_SOUND: u32 = 0x0102_c790;
const MESSAGE_IMPACT_DATA_SET: u32 = 0x0102_c740;
const MESSAGE_FIRST_PERSON_MODEL: u32 = 0x0102_c6e8;
const MESSAGE_MISC_OBJECT: u32 = 0x0102_c6a0;

/// The weapon's plain pointer fields `InitItem` resolves in turn, in the
/// order the code does: field offset, expected class, message.
const RESOLVED_FIELDS: [(u32, u32, u32); 24] = [
    (0x21C, TYPE_TES_SOUND, MESSAGE_OPEN_SOUND),
    (0x220, TYPE_TES_SOUND, MESSAGE_OPEN_SOUND_2),
    (0x224, TYPE_TES_SOUND, MESSAGE_OPEN_SOUND),
    (0x228, TYPE_TES_SOUND, MESSAGE_SOUND),
    (0x22C, TYPE_TES_SOUND, MESSAGE_OPEN_SOUND),
    (0x230, TYPE_TES_SOUND, MESSAGE_OPEN_SOUND),
    (0x234, TYPE_TES_SOUND, MESSAGE_IDLE_SOUND),
    (0x238, TYPE_TES_SOUND, MESSAGE_EQUIP_SOUND),
    (0x23C, TYPE_TES_SOUND, MESSAGE_UNEQUIP_SOUND),
    (0x240, TYPE_TES_SOUND, MESSAGE_UNEQUIP_SOUND),
    (0x244, TYPE_TES_SOUND, MESSAGE_UNEQUIP_SOUND),
    (0x248, TYPE_TES_SOUND, MESSAGE_UNEQUIP_SOUND),
    (0x24C, TYPE_BGS_IMPACT_DATA_SET, MESSAGE_IMPACT_DATA_SET),
    (0x250, TYPE_TES_OBJECT_STAT, MESSAGE_FIRST_PERSON_MODEL),
    (0x254, TYPE_TES_OBJECT_STAT, MESSAGE_FIRST_PERSON_MODEL),
    (0x258, TYPE_TES_OBJECT_STAT, MESSAGE_FIRST_PERSON_MODEL),
    (0x25C, TYPE_TES_OBJECT_STAT, MESSAGE_FIRST_PERSON_MODEL),
    (0x260, TYPE_TES_OBJECT_STAT, MESSAGE_FIRST_PERSON_MODEL),
    (0x268, TYPE_TES_OBJECT_STAT, MESSAGE_FIRST_PERSON_MODEL),
    (0x264, TYPE_TES_OBJECT_STAT, MESSAGE_FIRST_PERSON_MODEL),
    (0x26C, TYPE_TES_OBJECT_STAT, MESSAGE_FIRST_PERSON_MODEL),
    (0x350, TYPE_TES_OBJECT_IMOD, MESSAGE_MISC_OBJECT),
    (0x354, TYPE_TES_OBJECT_IMOD, MESSAGE_MISC_OBJECT),
    (0x358, TYPE_TES_OBJECT_IMOD, MESSAGE_MISC_OBJECT),
];

// Translated from 0051e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::InitItem` (Xbox PDB): after load, unless bit 3 of the
/// form flags is set, initializes the embedded components and turns the form
/// ids the weapon holds (projectile, scope effect, critical and VATS
/// effects, sounds, impact data set, first-person and mod objects) into
/// pointers; an id that does not resolve is logged and left as null. Then it
/// fixes a non-positive speed and attack multiplier to 1.0, copies the
/// "bit 2 of `iFlagsEx`" setting, sets `bIsLoopingReload` for reload
/// animations 19 and above, and marks the form initialized.
pub fn tes_object_weap_init_item(e: &mut Engine, this: Ptr<TesObjectWeap>) {
    if e.call(FORM_FLAG_8_TEST, &args![this]).bool() {
        return;
    }
    let weapon = this.addr();
    for (function, component) in [
        (BIPED_MODEL_LIST_INIT_ITEM, BIPED_MODEL_LIST),
        (SCRIPTABLE_FORM_INIT_ITEM, SCRIPTABLE_FORM),
        (ENCHANTABLE_FORM_INIT_ITEM, ENCHANTABLE_FORM),
        (AMMO_FORM_INIT_ITEM, AMMO_FORM),
        (DESTRUCTIBLE_FORM_INIT_ITEM, DESTRUCTIBLE_FORM),
        (MODEL_TEXTURE_SWAP_INIT_ITEM, MODEL),
        (REPAIR_ITEM_LIST_INIT_ITEM, REPAIR_ITEM_LIST),
        (PICKUP_PUTDOWN_SOUNDS_INIT_ITEM, PICKUP_PUTDOWN_SOUNDS),
    ] {
        e.call(function, &args![weapon + component, weapon]);
    }

    // The projectile: read through `00525a90`, stored with `0051f4b0`.
    let id = e.call(GET_PROJECTILE, &args![this, 0u32]).u32();
    if id != 0 {
        let (form, id) = resolve_form_reference(e, this, id, TYPE_BGS_PROJECTILE);
        fn_0051f4b0(e, this, form);
        if e.call(GET_PROJECTILE, &args![this, 0u32]).u32() == 0 {
            report_missing_reference(e, this, id, MESSAGE_PROJECTILE);
        }
    }

    // The scope effect, the critical effect and the VATS special effect:
    // each has a getter and a setter.
    let id = e.call(GET_SCOPE_EFFECT, &args![this]).u32();
    if id != 0 {
        let (form, id) = resolve_form_reference(e, this, id, TYPE_TES_EFFECT_SHADER);
        e.call(SET_SCOPE_EFFECT, &args![this, form]);
        if e.call(GET_SCOPE_EFFECT, &args![this]).u32() == 0 {
            report_missing_reference(e, this, id, MESSAGE_SCOPE_EFFECT);
        }
    }
    let id = e.call(GET_CRITICAL_EFFECT, &args![this]).u32();
    if id != 0 {
        let (form, id) = resolve_form_reference(e, this, id, TYPE_SPELL_ITEM);
        fn_0051f4f0(e, this, form);
        if e.call(GET_CRITICAL_EFFECT, &args![this]).u32() == 0 {
            report_missing_reference(e, this, id, MESSAGE_SPELL_ITEM);
        }
    }
    let id = e.call(GET_VATS_SPECIAL_EFFECT, &args![this]).u32();
    if id != 0 {
        let (form, id) = resolve_form_reference(e, this, id, TYPE_SPELL_ITEM);
        e.call(SET_VATS_SPECIAL_EFFECT, &args![this, form]);
        if e.call(GET_VATS_SPECIAL_EFFECT, &args![this]).u32() == 0 {
            report_missing_reference(e, this, id, MESSAGE_VATS_SPELL_ITEM);
        }
    }

    for (offset, target_type, message) in RESOLVED_FIELDS {
        let id = e.mem.u32(weapon + offset);
        if id != 0 {
            let (form, id) = resolve_form_reference(e, this, id, target_type);
            e.mem.set_u32(weapon + offset, form.addr());
            if form.is_null() {
                report_missing_reference(e, this, id, message);
            }
        }
    }

    // A speed or attack multiplier that is not positive becomes 1.0.
    let zero: f64 = e.global(ZERO);
    let data = weapon_data(this);
    if e.get(data, ObjWeap::fSpeed) as f64 <= zero {
        e.set(data, ObjWeap::fSpeed, 1.0f32);
    }
    if e.get(data, ObjWeap::fAttackMult) as f64 <= zero {
        e.set(data, ObjWeap::fAttackMult, 1.0f32);
    }
    let setting = e
        .call(SETTING_VALUE_POINTER, &args![SETTING_FORCE_NPCS_USE_AMMO])
        .u32();
    if e.mem.u8(setting) != 0 {
        fn_0051f590(e, this, true);
    }
    e.set(this, TesObjectWeap::bIsLoopingReload, 0);
    if e.get(data, ObjWeap::cReloadAnim) >= 0x13 {
        e.set(this, TesObjectWeap::bIsLoopingReload, 1);
    }
    e.call(FORM_SET_FLAG_8, &args![this, 1u32]);
}

/// `0.0` as a `double` (`01012060`).
const ZERO: u32 = 0x0101_2060;
/// The game setting `bForceNPCsUseAmmo:Combat` (name from the exe's static
/// initializer of the setting); when set, `InitItem` turns on bit 1 of
/// `iFlagsEx`.
const SETTING_FORCE_NPCS_USE_AMMO: u32 = 0x011c_94d0;

// ---- Load / save game and the DATA block ---------------------------------------------

/// Byte-swaps the `u32` at the pointer in place (`cdecl`, pointer and one
/// unused word); the callers of `OBJ_WEAP::Endian` push a zero byte as the
/// second word.
const SWAP_WORD_IN_PLACE: u32 = 0x0040_1080;

/// The `u32` and `float` fields of `OBJ_WEAP` that `Endian` swaps, in the
/// order the code does (byte offsets in the DATA block).
const ENDIAN_OFFSETS: [u32; 44] = [
    0x04, 0x08, 0x10, 0x14, 0x18, 0x1C, 0x2C, 0x30, 0x34, 0x38, 0x3C, 0x40, 0x44, 0x48, 0x4C, 0x50,
    0x54, 0x58, 0x5C, 0x60, 0x64, 0x68, 0x6C, 0x70, 0x74, 0x78, 0x7C, 0x80, 0x84, 0x88, 0x8C, 0x90,
    0x94, 0x98, 0x9C, 0xA0, 0xA8, 0xC8, 0xB0, 0xB4, 0xB8, 0xBC, 0xC0, 0xC4,
];

// Translated from 00520ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `OBJ_WEAP::Endian` (Xbox PDB): byte-swaps the 44 four-byte fields of the
/// DATA block (the bytes and the one-word fields stay).
pub fn obj_weap_endian(e: &mut Engine, this: Ptr<ObjWeap>) {
    for offset in ENDIAN_OFFSETS {
        e.call(SWAP_WORD_IN_PLACE, &args![this.addr() + offset, 0u32]);
    }
}

/// `TESForm::GetSaveSize` (the form's own part), `thiscall` (one word).
const FORM_GET_SAVE_SIZE: u32 = 0x0048_4bf0;
/// `TESValueForm::GetSaveSize` (Xbox PDB), `thiscall` (one word).
const VALUE_FORM_GET_SAVE_SIZE: u32 = 0x0048_e9b0;
/// The form's own part of `SaveGame`, `thiscall` (one word).
const FORM_SAVE_GAME: u32 = 0x0048_4c20;
/// `TESValueForm::SaveGame` (Xbox PDB), `thiscall` (one word).
const VALUE_FORM_SAVE_GAME: u32 = 0x0048_e9e0;
/// `TESForm::LoadGame` (Xbox PDB), `thiscall` (two words).
const FORM_LOAD_GAME: u32 = 0x0048_4c50;
/// `TESValueForm::LoadGame` (Xbox PDB), `thiscall` (two words).
const VALUE_FORM_LOAD_GAME: u32 = 0x0048_ea10;
/// The form's own part of the weapon vtable entry at `0x0102c570`.
const FORM_VTABLE_ENTRY_570: u32 = 0x0048_4d60;
/// The `TESValueForm` part of the same entry.
const VALUE_FORM_ENTRY_570: u32 = 0x0048_ea40;
/// The form's own part of the weapon vtable entry at `0x0102c578`.
const FORM_VTABLE_ENTRY_578: u32 = 0x0048_4da0;
/// The `TESValueForm` part of the same entry.
const VALUE_FORM_ENTRY_578: u32 = 0x0048_ea80;

// Translated from 00523020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weapon's save-game size (weapon vtable entry at `0x0102c56c`): the
/// `TESForm` part plus the `TESValueForm` part (at +0x84), as a `u16` sum.
pub fn fn_00523020(e: &mut Engine, this: Ptr<TesObjectWeap>, argument: u32) -> u16 {
    let form_part = e.call(FORM_GET_SAVE_SIZE, &args![this, argument]).u16();
    let value_part = e
        .call(
            VALUE_FORM_GET_SAVE_SIZE,
            &args![this.addr() + VALUE_FORM, argument],
        )
        .u16();
    form_part.wrapping_add(value_part)
}

// Translated from 00523080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Weapon vtable entry at `0x0102c574`: saves the `TESForm` part and the
/// `TESValueForm` part (at +0x84) into the save game.
pub fn fn_00523080(e: &mut Engine, this: Ptr<TesObjectWeap>, argument: u32) {
    e.call(FORM_SAVE_GAME, &args![this, argument]);
    e.call(
        VALUE_FORM_SAVE_GAME,
        &args![this.addr() + VALUE_FORM, argument],
    );
}

// Translated from 005230b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Weapon vtable entry at `0x0102c57c`: loads the `TESForm` part and the
/// `TESValueForm` part (at +0x84) from the save game (two words).
pub fn fn_005230b0(e: &mut Engine, this: Ptr<TesObjectWeap>, first: u32, second: u32) {
    e.call(FORM_LOAD_GAME, &args![this, first, second]);
    e.call(
        VALUE_FORM_LOAD_GAME,
        &args![this.addr() + VALUE_FORM, first, second],
    );
}

// Translated from 005230f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Weapon vtable entry at `0x0102c570`: the `TESForm` part, then the
/// `TESValueForm` part (at +0x84), each given the one argument.
pub fn fn_005230f0(e: &mut Engine, this: Ptr<TesObjectWeap>, argument: u32) {
    e.call(FORM_VTABLE_ENTRY_570, &args![this, argument]);
    e.call(
        VALUE_FORM_ENTRY_570,
        &args![this.addr() + VALUE_FORM, argument],
    );
}

// Translated from 00523120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Weapon vtable entry at `0x0102c578`: the `TESForm` part, then the
/// `TESValueForm` part (at +0x84), each given the one argument.
pub fn fn_00523120(e: &mut Engine, this: Ptr<TesObjectWeap>, argument: u32) {
    e.call(FORM_VTABLE_ENTRY_578, &args![this, argument]);
    e.call(
        VALUE_FORM_ENTRY_578,
        &args![this.addr() + VALUE_FORM, argument],
    );
}

// ---- Save ----------------------------------------------------------------------------

/// The four-character code of a chunk, as the file stores it.
const fn tag(name: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*name)
}

/// `TESForm::StartForm` (Xbox PDB), `thiscall` (weapon).
const FORM_START_FORM: u32 = 0x0048_55a0;
/// `TESForm::CloseForm` (Xbox PDB), `thiscall` (weapon).
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// `TESForm::SaveData` (Xbox PDB), `thiscall` (weapon).
const FORM_SAVE_DATA: u32 = 0x0048_5e00;
/// `TESFullName::Save` (Xbox PDB), `thiscall` (component).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
/// `TESModelTextureSwap::Save` (Xbox PDB), `thiscall` (model chunk, texture
/// chunk, swap chunk).
const MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
/// `TESModel::SaveFlags` (Xbox PDB), `thiscall` (chunk).
const MODEL_SAVE_FLAGS: u32 = 0x0048_9240;
/// `TESTexture::Save` (Xbox PDB), `thiscall` (chunk).
const TEXTURE_SAVE: u32 = 0x0048_e370;
/// `BGSMessageIcon::Save` (Xbox PDB), `thiscall` (chunk).
const MESSAGE_ICON_SAVE: u32 = 0x0047_a240;
/// `TESScriptableForm::Save` (Xbox PDB), `thiscall`.
const SCRIPTABLE_FORM_SAVE: u32 = 0x0048_cca0;
/// `TESEnchantableForm::Save` (Xbox PDB), `thiscall`.
const ENCHANTABLE_FORM_SAVE: u32 = 0x0047_bd70;
/// `BGSAmmoForm::Save` (Xbox PDB), `thiscall`.
const AMMO_FORM_SAVE: u32 = 0x0047_4af0;
/// `BGSDestructibleObjectForm::Save` (Xbox PDB), `thiscall`.
const DESTRUCTIBLE_FORM_SAVE: u32 = 0x0047_7f20;
/// `BGSRepairItemList::Save` (Xbox PDB), `thiscall`.
const REPAIR_ITEM_LIST_SAVE: u32 = 0x0047_b8a0;
/// `BGSEquipType::Save` (Xbox PDB), `thiscall`.
const EQUIP_TYPE_SAVE: u32 = 0x0047_9350;
/// `BGSBipedModelList::Save` (Xbox PDB), `thiscall`.
const BIPED_MODEL_LIST_SAVE: u32 = 0x0047_4d60;
/// `BGSPickupPutdownSounds::Save` (Xbox PDB), `thiscall`.
const PICKUP_PUTDOWN_SOUNDS_SAVE: u32 = 0x0047_b560;
/// `TESModel::Save` (Xbox PDB), `thiscall` (model chunk, texture chunk).
const MODEL_SAVE: u32 = 0x0048_91d0;
/// Weapon flag test (`data.cFlags & 4`, `AL`), `thiscall`.
const WEAPON_FLAG_4_TEST: u32 = 0x004a_d010;
/// Chunk writers: a chunk holding one word (`cdecl`, chunk, value); one
/// holding an array (`TESForm::AddChunkArray`, `cdecl`, chunk, data, size);
/// and `TESForm::__AddChunkData` (`cdecl`, chunk, data, size).
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// Save-side byte-swap switch (`cdecl`, no arguments, `AL`).
const SWAP_ON_SAVE: u32 = 0x0040_1500;
/// `BSStringT<char>` length (`thiscall`) and c-string getter (`thiscall`).
const STRING_LENGTH: u32 = 0x0040_48e0;
const STRING_C_STR: u32 = 0x0055_9450;
/// Endian swap of the critical-hit block (`thiscall`, +0x1C0) and of the
/// VATS special block (`thiscall`, +0x370).
const CRITICAL_DATA_ENDIAN: u32 = 0x0053_5aa0;
const VATS_SPECIAL_ENDIAN: u32 = 0x0067_ff10;

/// The texture-swap models after the world model, in save order: model
/// offset, model chunk, texture chunk.
const SAVED_MOD_MODELS: [(u32, [u8; 4], [u8; 4]); 7] = [
    (WORLD_MODEL_MOD_1, *b"MWD1", *b"MWT1"),
    (WORLD_MODEL_MOD_2, *b"MWD2", *b"MWT2"),
    (WORLD_MODEL_MOD_3, *b"MWD4", *b"MWT4"),
    (WORLD_MODEL_MOD_12, *b"MWD3", *b"MWT3"),
    (WORLD_MODEL_MOD_13, *b"MWD5", *b"MWT5"),
    (WORLD_MODEL_MOD_23, *b"MWD6", *b"MWT6"),
    (WORLD_MODEL_MOD_123, *b"MWD7", *b"MWT7"),
];

/// The weapon fields saved as a form id when set, in save order: field
/// offset and chunk.
const SAVED_FORM_FIELDS: [(u32, [u8; 4]); 24] = [
    (0x24C, *b"INAM"),
    (0x250, *b"WNAM"),
    (0x254, *b"WNM1"),
    (0x258, *b"WNM2"),
    (0x25C, *b"WNM4"),
    (0x260, *b"WNM3"),
    (0x264, *b"WNM6"),
    (0x268, *b"WNM5"),
    (0x26C, *b"WNM7"),
    (0x350, *b"WMI1"),
    (0x354, *b"WMI2"),
    (0x358, *b"WMI3"),
    (0x21C, *b"SNAM"),
    (0x220, *b"SNAM"),
    (0x224, *b"XNAM"),
    (0x228, *b"NAM7"),
    (0x22C, *b"TNAM"),
    (0x230, *b"NAM6"),
    (0x234, *b"UNAM"),
    (0x238, *b"NAM9"),
    (0x23C, *b"NAM8"),
    (0x240, *b"WMS1"),
    (0x244, *b"WMS1"),
    (0x248, *b"WMS2"),
];

/// A `BSStringT<char>` saved as a chunk of its characters and the
/// terminator, when it is not empty.
fn save_string_chunk(e: &mut Engine, string: u32, chunk: [u8; 4]) {
    let length = e.call(STRING_LENGTH, &args![string]).u32();
    if length != 0 {
        let length = e.call(STRING_LENGTH, &args![string]).u32();
        let text = e.call(STRING_C_STR, &args![string]).u32();
        e.call(ADD_CHUNK_ARRAY, &args![tag(&chunk), text, length + 1]);
    }
}

/// Writes one block of the weapon as a chunk with its form pointer replaced
/// by the pointed-at form's id: the pointer at `pointer_at` is swapped for
/// the id, the block is byte-swapped (`swap`) when the save is big-endian,
/// written as one chunk (`block`, `size` bytes) and swapped back. Returns
/// the pointer, which the caller restores after the last chunk.
fn save_block_with_form_id(
    e: &mut Engine,
    this: Ptr<TesObjectWeap>,
    pointer_at: u32,
    (block, size): (u32, u32),
    chunk: [u8; 4],
    swap: impl Fn(&mut Engine),
) -> u32 {
    let weapon = this.addr();
    let saved = e.mem.u32(weapon + pointer_at);
    let id = if saved != 0 {
        e.call(GET_FORM_ID, &args![saved]).u32()
    } else {
        0
    };
    e.mem.set_u32(weapon + pointer_at, id);
    if e.call(SWAP_ON_SAVE, &[]).bool() {
        swap(e);
    }
    e.call(ADD_CHUNK_DATA, &args![tag(&chunk), weapon + block, size]);
    if e.call(SWAP_ON_SAVE, &[]).bool() {
        swap(e);
    }
    saved
}

// Translated from 00521730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::Save` (Xbox PDB): writes the form record. Starts the
/// form, saves each component in turn (name, models with their texture
/// swaps, icon, script, enchantment, ammo, destruction data, repair list,
/// equip type, biped model list, pickup and putdown sounds), the shell
/// casing, scope and world models, the seven mod models, the embedded
/// weapon node and VATS attack name strings, the ids of the objects and
/// sounds the weapon points at, and then the DATA, critical-hit and VATS
/// special blocks with their pointers replaced by form ids (restored after
/// writing) and the sound level; then closes the form.
pub fn tes_object_weap_save(e: &mut Engine, this: Ptr<TesObjectWeap>) {
    let weapon = this.addr();
    e.call(FORM_START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![weapon + FULL_NAME]);
    e.call(
        MODEL_TEXTURE_SWAP_SAVE,
        &args![weapon + MODEL, tag(b"MODL"), tag(b"MODT"), tag(b"MODS")],
    );
    e.call(MODEL_SAVE_FLAGS, &args![weapon + MODEL, tag(b"MODD")]);
    e.call(TEXTURE_SAVE, &args![weapon + ICON, tag(b"ICON")]);
    e.call(
        MESSAGE_ICON_SAVE,
        &args![weapon + MESSAGE_ICON, tag(b"MICO")],
    );
    e.call(SCRIPTABLE_FORM_SAVE, &args![weapon + SCRIPTABLE_FORM]);
    e.call(ENCHANTABLE_FORM_SAVE, &args![weapon + ENCHANTABLE_FORM]);
    e.call(AMMO_FORM_SAVE, &args![weapon + AMMO_FORM]);
    e.call(DESTRUCTIBLE_FORM_SAVE, &args![weapon + DESTRUCTIBLE_FORM]);
    e.call(REPAIR_ITEM_LIST_SAVE, &args![weapon + REPAIR_ITEM_LIST]);
    e.call(EQUIP_TYPE_SAVE, &args![weapon + EQUIP_TYPE]);
    e.call(BIPED_MODEL_LIST_SAVE, &args![weapon + BIPED_MODEL_LIST]);
    e.call(
        PICKUP_PUTDOWN_SOUNDS_SAVE,
        &args![weapon + PICKUP_PUTDOWN_SOUNDS],
    );
    e.call(
        MODEL_SAVE,
        &args![weapon + SHELL_CASING_MODEL, tag(b"MOD2"), tag(b"MO2T")],
    );
    if e.call(WEAPON_FLAG_4_TEST, &args![this]).bool() {
        e.call(
            MODEL_SAVE,
            &args![weapon + SCOPE_MODEL, tag(b"MOD3"), tag(b"MO3T")],
        );
        let scope_effect = e.get(this, TesObjectWeap::pScopeEffect);
        if !scope_effect.is_null() {
            let id = e.call(GET_FORM_ID, &args![scope_effect]).u32();
            e.call(ADD_CHUNK_WORD, &args![tag(b"EFSD"), id]);
        }
    }
    e.call(
        MODEL_SAVE,
        &args![weapon + WORLD_MODEL, tag(b"MOD4"), tag(b"MO4T")],
    );
    for (model, model_chunk, texture_chunk) in SAVED_MOD_MODELS {
        e.call(
            MODEL_TEXTURE_SWAP_SAVE,
            &args![
                weapon + model,
                tag(&model_chunk),
                tag(&texture_chunk),
                tag(b"MODS")
            ],
        );
    }
    save_string_chunk(e, weapon + EMBEDDED_WEAPON_NODE, *b"NNAM");
    save_string_chunk(e, weapon + VATS_ATTACK_NAME, *b"VANM");
    for (offset, chunk) in SAVED_FORM_FIELDS {
        let field = e.mem.u32(weapon + offset);
        if field != 0 {
            let id = e.call(GET_FORM_ID, &args![field]).u32();
            e.call(ADD_CHUNK_WORD, &args![tag(&chunk), id]);
        }
    }
    e.call(FORM_SAVE_DATA, &args![this]);

    let data = weapon + DATA;
    let projectile = save_block_with_form_id(e, this, 0x118, (DATA, 0xCC), *b"DNAM", |e| {
        obj_weap_endian(e, Ptr::new(data))
    });
    let critical = weapon + CRITICAL_DATA;
    let critical_effect =
        save_block_with_form_id(e, this, 0x1CC, (CRITICAL_DATA, 0x10), *b"CRDT", |e| {
            e.call(CRITICAL_DATA_ENDIAN, &args![critical]);
        });
    let vats = weapon + VATS_SPECIAL_DATA;
    let vats_effect =
        save_block_with_form_id(e, this, 0x370, (VATS_SPECIAL_DATA, 0x14), *b"VATS", |e| {
            e.call(VATS_SPECIAL_ENDIAN, &args![vats]);
        });
    let sound_level = e.get(this, TesObjectWeap::eSoundLevel);
    e.call(ADD_CHUNK_WORD, &args![tag(b"VNAM"), sound_level]);
    e.mem.set_u32(weapon + 0x118, projectile);
    e.mem.set_u32(weapon + 0x1CC, critical_effect);
    e.mem.set_u32(weapon + 0x370, vats_effect);
    e.call(FORM_CLOSE_FORM, &args![this]);
}

// ---- Weapon type, models and ammo ----------------------------------------------------

// Translated from 00522c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::GetCombatWeaponType` (Xbox PDB): the weapon's combat
/// class. Types 0 to 2 give 2; types 3 to 9 and 13 give 1, or 0 when the
/// weapon's projectile has an explosion whose distance and radius both
/// exceed the two combat settings; type 10 gives 3; types 11 and 12 give 4.
/// Any other type is logged and gives 2.
pub fn tes_object_weap_get_combat_weapon_type(e: &mut Engine, this: Ptr<TesObjectWeap>) -> u32 {
    let weapon_type = e.call(GET_WEAPON_TYPE, &args![this]).u32();
    match weapon_type {
        0..=2 => 2,
        3..=9 | 13 => {
            let projectile = e.call(GET_PROJECTILE, &args![this, 0u32]).u32();
            if projectile != 0 && e.call(PROJECTILE_GET_EXPLOSION, &args![projectile]).u32() != 0 {
                let explosion = e.call(PROJECTILE_GET_EXPLOSION, &args![projectile]).u32();
                let distance = e.call(EXPLOSION_GET_DAMAGE, &args![explosion]).f64();
                let setting = e
                    .call(
                        SETTING_GET_FLOAT_POINTER,
                        &args![SETTING_DANGEROUS_PROJECTILE_EXPLOSION_DAMAGE],
                    )
                    .u32();
                if (e.mem.f32(setting) as f64) < distance {
                    let explosion = e.call(PROJECTILE_GET_EXPLOSION, &args![projectile]).u32();
                    let radius = e.call(EXPLOSION_GET_RADIUS, &args![explosion]).f64();
                    let setting = e
                        .call(
                            SETTING_GET_FLOAT_POINTER,
                            &args![SETTING_DANGEROUS_PROJECTILE_EXPLOSION_RADIUS],
                        )
                        .u32();
                    if (e.mem.f32(setting) as f64) < radius {
                        return 0;
                    }
                }
            }
            1
        }
        10 => 3,
        11 | 12 => 4,
        _ => {
            let weapon_type = e.call(GET_WEAPON_TYPE, &args![this]).u32();
            e.call(
                LOG_MESSAGE,
                &args![UNKNOWN_WEAPON_TYPE_MESSAGE, weapon_type],
            );
            2
        }
    }
}

/// Whether the `TESModel` at `model` has a non-empty path: its virtual
/// `GetModel` returns the path, and the first character is tested.
fn model_has_path(e: &mut Engine, model: u32) -> bool {
    let path = e.vcall(model, MODEL_GET_MODEL_SLOT, &[]).u32();
    e.mem.u8(path) != 0
}

// Translated from 00522d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weapon's model: the world model (+0x200) when it has a path, else
/// the weapon's main model (+0x3C), null for a null weapon.
pub fn fn_00522d80(e: &mut Engine, this: Ptr<TesObjectWeap>) -> Ptr {
    let weapon = this.addr();
    if model_has_path(e, weapon + WORLD_MODEL) {
        return Ptr::new(weapon + WORLD_MODEL);
    }
    if weapon == 0 {
        Ptr::NULL
    } else {
        Ptr::new(weapon + MODEL)
    }
}

// Translated from 00522df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The model for a set of installed mods: `mods` is a bit mask of the three
/// mod slots (1, 2, 4) and `force_mods` forces the mod models even for a weapon
/// of type 0. For a type 0 weapon without `force_mods` this is
/// [`fn_00522d80`]'s world-model rule; otherwise it is the world model for
/// that mod combination when it has a path and the weapon's form id is not
/// `0x001735d4`, else the weapon's main model (+0x3C).
pub fn fn_00522df0(e: &mut Engine, this: Ptr<TesObjectWeap>, mods: u8, force_mods: bool) -> Ptr {
    let weapon = this.addr();
    let main_model = if weapon == 0 {
        Ptr::NULL
    } else {
        Ptr::new(weapon + MODEL)
    };
    if e.call(GET_WEAPON_TYPE, &args![this]).u32() == 0 && !force_mods {
        if model_has_path(e, weapon + WORLD_MODEL) {
            return Ptr::new(weapon + WORLD_MODEL);
        }
        return main_model;
    }
    let candidate = match (mods & 1 != 0, mods & 2 != 0, mods & 4 != 0) {
        (true, true, true) => WORLD_MODEL_MOD_123,
        (true, true, false) => WORLD_MODEL_MOD_12,
        (true, false, true) => WORLD_MODEL_MOD_13,
        (true, false, false) => WORLD_MODEL_MOD_1,
        (false, true, true) => WORLD_MODEL_MOD_23,
        (false, true, false) => WORLD_MODEL_MOD_2,
        (false, false, true) => WORLD_MODEL_MOD_3,
        (false, false, false) => 0,
    };
    let model = if candidate != 0 && model_has_path(e, weapon + candidate) {
        Ptr::new(weapon + candidate)
    } else {
        Ptr::NULL
    };
    if model.is_null() || e.call(GET_FORM_ID, &args![this]).u32() == NO_MOD_MODEL_FORM_ID {
        main_model
    } else {
        model
    }
}

// ---- Small getters and setters -----------------------------------------------------------

/// `NiMatrix3`-style sine/cosine helper: writes two floats (`cdecl`: angle,
/// pointer, pointer; the matrix shape of `MakeXRotation` shows they are
/// the sine and the cosine).
const SINE_COSINE: u32 = 0x0041_69a0;

// Translated from 00524ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::MakeXRotation` (Xbox PDB): the rotation about the X axis by
/// `angle` as nine floats (row-major): `1 0 0 / 0 c s / 0 -s c`.
pub fn nimatrix3_make_x_rotation(e: &mut Engine, this: Ptr, angle: f32) {
    let (sine, cosine) = e.with_stack(8, |e, slots| {
        e.call(SINE_COSINE, &args![angle, slots.addr() + 4, slots.addr()]);
        (e.mem.f32(slots.addr() + 4), e.mem.f32(slots.addr()))
    });
    let matrix = this.addr();
    for (index, value) in [1.0, 0.0, 0.0, 0.0, cosine, sine, 0.0, -sine, cosine]
        .into_iter()
        .enumerate()
    {
        e.mem.set_f32(matrix + 4 * index as u32, value);
    }
}

// Translated from 00524b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 1 (value 2) of `data.cFlags` (`this + 0x100`).
pub fn fn_00524b40(e: &mut Engine, this: Ptr<TesObjectWeap>) -> bool {
    e.get(weapon_data(this), ObjWeap::cFlags) & 2 != 0
}

// Translated from 00524b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `data.cAmmoPerShot` (`this + 0x102`).
pub fn fn_00524b60(e: &mut Engine, this: Ptr<TesObjectWeap>) -> u8 {
    e.get(weapon_data(this), ObjWeap::cAmmoPerShot)
}

/// A weapon float reduced by the mod effect `3` when `with_mod`: the value
/// minus `GetModEffectValue(3, 0)`, not below zero.
fn float_less_mod_effect(
    e: &mut Engine,
    this: Ptr<TesObjectWeap>,
    value: f32,
    with_mod: bool,
) -> f32 {
    if !with_mod {
        return value;
    }
    let effect = e.call(GET_MOD_EFFECT_VALUE, &args![this, 3u32, 0u32]).f32();
    // x87: `FSUBR` leaves `value - effect` in extended precision and the
    // result is stored as a float for the call.
    let reduced = (value as f64 - effect as f64) as f32;
    e.call(MAX_FLOAT, &args![0.0f32, reduced]).f32()
}

// Translated from 00524b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `data.fMinSpread` (`this + 0x104`), less the mod effect `3` when
/// `with_mod`, not below zero.
pub fn fn_00524b80(e: &mut Engine, this: Ptr<TesObjectWeap>, with_mod: bool) -> f32 {
    let value = e.get(weapon_data(this), ObjWeap::fMinSpread);
    float_less_mod_effect(e, this, value, with_mod)
}

// Translated from 00524be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `data.fSpread` (`this + 0x108`), less the mod effect `3` when
/// `with_mod`, not below zero.
pub fn fn_00524be0(e: &mut Engine, this: Ptr<TesObjectWeap>, with_mod: bool) -> f32 {
    let value = e.get(weapon_data(this), ObjWeap::fSpread);
    float_less_mod_effect(e, this, value, with_mod)
}

/// `thiscall` on a 3x3 matrix (`this`, `result`, vector): `result = this *
/// vector`, returned (`globalfunc.cpp`).
const MATRIX_MULTIPLY_VECTOR: u32 = 0x004b_4500;
/// `thiscall` on a vector (`this`, `result`, scale): `result = this * scale`,
/// returned.
const VECTOR_SCALED: u32 = 0x0045_bb20;
/// `thiscall` on a vector (`this`, `result`, other): `result = this +
/// other`, returned.
const VECTOR_ADD: u32 = 0x0043_9e90;

// Translated from 00524c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies a transform to a point and stores the result at `result`
/// (returned): the transform is laid out as a 3x3 rotation, a translation
/// at +0x24 and a scale at +0x30 (a `NiTransform`), so the result is
/// `rotation * point * scale + translation`, computed by the three vector
/// helpers through two 12-byte temporaries.
pub fn fn_00524c40(e: &mut Engine, transform: Ptr, result: Ptr, point: u32) -> Ptr {
    let scale = e.mem.f32(transform.addr() + 0x30);
    e.with_stack(24, |e, temps| {
        let rotated = e
            .call(
                MATRIX_MULTIPLY_VECTOR,
                &args![transform, temps.addr(), point],
            )
            .u32();
        let scaled = e
            .call(VECTOR_SCALED, &args![rotated, temps.addr() + 12, scale])
            .u32();
        e.call(VECTOR_ADD, &args![scaled, result, transform.addr() + 0x24]);
    });
    result
}

/// The singleton object at `011deb7c` that `00559450` reads.
const SINGLETON_011DEB7C: u32 = 0x011d_eb7c;
/// `BSFaceGenNiNode::GetAnimationData` (the engine map's name for the
/// folded body), `thiscall` on the word the singleton getter returns.
const FOLDED_GETTER_006629F0: u32 = 0x0066_29f0;

// Translated from 00524c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the word at the start of the object at `011deb7c` (`00559450`) and
/// passes it to `006629f0`, returning that result.
pub fn fn_00524c90(e: &mut Engine) -> u32 {
    let word = e.call(STRING_C_STR, &args![SINGLETON_011DEB7C]).u32();
    e.call(FOLDED_GETTER_006629F0, &args![word]).u32()
}

// Translated from 00524cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c621c`.
pub fn fn_00524cb0(e: &mut Engine) -> u32 {
    e.global(0x011c_621c)
}

// Translated from 00524cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c6220`.
pub fn fn_00524cc0(e: &mut Engine) -> u32 {
    e.global(0x011c_6220)
}

// Translated from 00524cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0xd` (`OBJ_WEAP::cHandGripAnim` if `this` is a
/// weapon's DATA block; the callers are the firing code and `CombatHit`).
pub fn fn_00524cd0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xD)
}

/// `thiscall` on an object (one word): the target of `00524cf0`
/// (`456fc0`).
const FN_00456FC0: u32 = 0x0045_6fc0;

// Translated from 00524cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00456fc0(this, 1)` and returns its result.
pub fn fn_00524cf0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FN_00456FC0, &args![this, 1u32]).u32()
}

// Translated from 00524d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x64b` (the callers are the menu-mode and iron
/// sights code on the player).
pub fn fn_00524d10(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x64B)
}

// Translated from 00524d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `this + 0x1f8`.
pub fn fn_00524d30(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x1F8, value);
}

// Translated from 00524d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x1f8`.
pub fn fn_00524d50(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x1F8)
}

/// The three words copied by `00524d70`.
const WORDS_011F426C: u32 = 0x011f_426c;

// Translated from 00524d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `this + 0xd2c` and copies the three words at
/// `011f426c` to `this + 0xd30`.
pub fn fn_00524d70(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0xD2C, 0);
    for index in 0..3 {
        let word = e.mem.u32(WORDS_011F426C + 4 * index);
        e.mem.set_u32(this.addr() + 0xD30 + 4 * index, word);
    }
}

/// The table of the fast reciprocal square root (a pointer to 256 words).
const INVERSE_SQRT_TABLE: u32 = 0x011f_4aa8;

// Translated from 00525340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Normalizes the 3-float vector at `vector` in place with the game's table
/// based reciprocal square root: the squared length is split into exponent
/// and top mantissa bits, the table gives the mantissa of the root, and the
/// vector is multiplied by its reciprocal. A vector of squared length zero
/// is multiplied by zero.
pub fn fn_00525340(e: &mut Engine, vector: Ptr) {
    let base = vector.addr();
    let (x, y, z) = (e.mem.f32(base), e.mem.f32(base + 4), e.mem.f32(base + 8));
    // x87: the three squares are summed in extended precision and stored as
    // a float once.
    let squared = (x as f64 * x as f64 + y as f64 * y as f64 + z as f64 * z as f64) as f32;
    let scale = if squared.to_bits() == 0 {
        0.0f32
    } else {
        let bits = squared.to_bits();
        let mut exponent = (bits >> 23).wrapping_sub(0x7f) as u16 as i16;
        let mut mantissa = bits & 0x7f_ffff;
        if exponent & 1 != 0 {
            mantissa |= 0x80_0000;
        }
        exponent >>= 1;
        let table: u32 = e.global(INVERSE_SQRT_TABLE);
        let entry = e.mem.u32(table + (mantissa >> 16) * 4);
        let root = f32::from_bits((((exponent as i32) + 0x7f) << 23) as u32 | entry);
        (1.0f64 / root as f64) as f32
    };
    e.mem.set_f32(base, (x as f64 * scale as f64) as f32);
    e.mem.set_f32(base + 4, (y as f64 * scale as f64) as f32);
    e.mem.set_f32(base + 8, (z as f64 * scale as f64) as f32);
}

// Translated from 00525420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `011dea2a`.
pub fn fn_00525420(e: &mut Engine) -> u8 {
    e.global(0x011d_ea2a)
}

// Translated from 00525430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at `this + 8` is non-zero.
pub fn fn_00525430(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 8) != 0
}

/// `thiscall` on a `TESModel` (`0048cee0`): non-zero when it has a model
/// to queue.
const MODEL_HAS_FILE: u32 = 0x0048_cee0;
/// The word at `011c3b3c` is the model loader object; `00443d30` is its queueing function
/// (`00443d30`, `thiscall`: model, two words, a level of detail, `1`, `0`,
/// `0`; `RET 0x1C`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
const MODEL_LOADER_QUEUE: u32 = 0x0044_3d30;
/// Virtual slot `0x10` of the `BGSPreloadable` part of a form: its
/// `QueueFiles`.
const PRELOADABLE_QUEUE_FILES_SLOT: u32 = 0x10;
/// Offset of the `BGSPreloadable` part inside the ammo form that `00525a90`
/// returns.
const AMMO_PRELOADABLE: u32 = 0x54;
/// Offset of the `BGSPreloadable` part inside the impact data set.
const IMPACT_DATA_SET_PRELOADABLE: u32 = 0x18;
/// The level of detail multiplier (`TES::GetLODMult`, `cdecl`, one word).
const GET_LOD_MULT: u32 = 0x0045_c6b0;

/// Queues the preload files of the weapon's ammo and impact data set: the
/// `QueueFiles` of the `BGSPreloadable` parts, called with the two words.
fn queue_ammo_and_impact_files(e: &mut Engine, this: Ptr<TesObjectWeap>, first: u32, second: u32) {
    if e.call(GET_PROJECTILE, &args![this, 0u32]).u32() != 0 {
        let preloadable = e.call(GET_PROJECTILE, &args![this, 0u32]).u32() + AMMO_PRELOADABLE;
        e.vcall(
            preloadable,
            PRELOADABLE_QUEUE_FILES_SLOT,
            &args![first, second],
        );
    }
    let impact_data_set = e.get(this, TesObjectWeap::pImpactDataSet);
    if !impact_data_set.is_null() {
        e.vcall(
            impact_data_set.addr() + IMPACT_DATA_SET_PRELOADABLE,
            PRELOADABLE_QUEUE_FILES_SLOT,
            &args![first, second],
        );
    }
}

// Translated from 00525450 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weapon's `BGSPreloadable::QueueFiles` (Xbox PDB), entered with
/// `this` at the `BGSPreloadable` part (weapon + 0xCC): queues the shell
/// casing model (+0x1D0), then the preload files of the projectile that
/// `00525a90` returns (+0x54) and of the impact data set (+0x18 of the
/// form at weapon + 0x24C).
pub fn fn_00525450(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let weapon = Ptr::<TesObjectWeap>::new(this.addr().wrapping_sub(PRELOADABLE));
    let model = weapon.addr() + SHELL_CASING_MODEL;
    if e.call(MODEL_HAS_FILE, &args![model]).u32() != 0 {
        let loader: u32 = e.global(MODEL_LOADER);
        e.call(
            MODEL_LOADER_QUEUE,
            &args![loader, model, first, second, 0u32, 1u32, 0u32, 0u32],
        );
    }
    queue_ammo_and_impact_files(e, weapon, first, second);
}

// Translated from 00525510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::QueueFiles` (Xbox PDB): queues the shell casing model,
/// the scope model when `include_scope` and the weapon has bit 2 of its
/// flags (with the level of detail from `TES::GetLODMult`), and the preload
/// files of the projectile and the impact data set.
pub fn tes_object_weap_queue_files(
    e: &mut Engine,
    this: Ptr<TesObjectWeap>,
    first: u32,
    second: u32,
    include_scope: bool,
) {
    let weapon = this.addr();
    let loader: u32 = e.global(MODEL_LOADER);
    let model = weapon + SHELL_CASING_MODEL;
    if e.call(MODEL_HAS_FILE, &args![model]).u32() != 0 {
        e.call(
            MODEL_LOADER_QUEUE,
            &args![loader, model, first, second, 0u32, 1u32, 0u32, 0u32],
        );
    }
    if include_scope && e.call(WEAPON_FLAG_4_TEST, &args![this]).bool() {
        let scope = weapon + SCOPE_MODEL;
        if e.call(MODEL_HAS_FILE, &args![scope]).u32() != 0 {
            let detail = e.call(GET_LOD_MULT, &args![this]).u32();
            e.call(
                MODEL_LOADER_QUEUE,
                &args![loader, scope, first, second, detail, 1u32, 0u32, 0u32],
            );
        }
    }
    queue_ammo_and_impact_files(e, this, first, second);
}

/// The game settings `iSoundLevelLoud`, `iSoundLevelNormal` and
/// `iSoundLevelSilent` that `00525620` reads for the sound levels 0, 1 and 2
/// (`0043d4d0`, `thiscall`, a pointer to the word; names from the exe's
/// static initializers of the settings).
const SETTING_SOUND_LEVEL_LOUD: u32 = 0x011c_dde0;
const SETTING_SOUND_LEVEL_NORMAL: u32 = 0x011c_d538;
const SETTING_SOUND_LEVEL_SILENT: u32 = 0x011c_d3b4;
const SETTING_GET_WORD_POINTER: u32 = 0x0043_d4d0;

// Translated from 00525620 (decompiled, FalloutNV.exe 1.4.0.525)
/// A setting word chosen by the weapon's sound level (`eSoundLevel`):
/// `force_two` makes the level 2 and `force_one` (applied after) makes it
/// 1; level 0, 1 and 2 read the words of three setting objects; any other
/// level gives 0.
pub fn fn_00525620(
    e: &mut Engine,
    this: Ptr<TesObjectWeap>,
    force_two: bool,
    force_one: bool,
) -> u32 {
    let mut level = e.get(this, TesObjectWeap::eSoundLevel);
    if force_two {
        level = 2;
    }
    if force_one {
        level = 1;
    }
    let object = match level {
        0 => SETTING_SOUND_LEVEL_LOUD,
        1 => SETTING_SOUND_LEVEL_NORMAL,
        2 => SETTING_SOUND_LEVEL_SILENT,
        _ => return 0,
    };
    let word = e.call(SETTING_GET_WORD_POINTER, &args![object]).u32();
    e.mem.u32(word)
}

/// Weapon flag test (`data.cFlags & 0x20`, `AL`), `thiscall`.
const WEAPON_FLAG_20_TEST: u32 = 0x0046_e8c0;
/// The embedded weapon node name as a c-string, or null (`thiscall`).
const GET_EMBEDDED_WEAPON_NODE: u32 = 0x0052_2c00;
/// `"ProjectileNode"` and `"##ProjectileNode"`.
const PROJECTILE_NODE_NAME: u32 = 0x0102_cb10;
const HASHED_PROJECTILE_NODE_NAME: u32 = 0x0102_cb20;
/// Child lookup by name (`cdecl`, node, name).
const FIND_NODE_BY_NAME: u32 = 0x004a_ae30;

// Translated from 005256b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The name of the node projectiles leave from: the embedded weapon node
/// name when the weapon has flag `0x20` and a non-empty name, else
/// `"ProjectileNode"`.
pub fn fn_005256b0(e: &mut Engine, this: Ptr<TesObjectWeap>) -> u32 {
    if e.call(WEAPON_FLAG_20_TEST, &args![this]).bool()
        && e.call(GET_EMBEDDED_WEAPON_NODE, &args![this]).u32() != 0
    {
        let name = e.call(GET_EMBEDDED_WEAPON_NODE, &args![this]).u32();
        if e.mem.i8(name) != 0 {
            return e.call(GET_EMBEDDED_WEAPON_NODE, &args![this]).u32();
        }
    }
    PROJECTILE_NODE_NAME
}

// Translated from 00525700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::GetFireNode` (Xbox PDB): the child of `root` named by
/// [`fn_005256b0`], else the one named `"##ProjectileNode"`.
pub fn tes_object_weap_get_fire_node(e: &mut Engine, this: Ptr<TesObjectWeap>, root: Ptr) -> Ptr {
    let name = fn_005256b0(e, this);
    let mut node = e.call(FIND_NODE_BY_NAME, &args![root, name]).ptr();
    if node.is_null() {
        node = e
            .call(FIND_NODE_BY_NAME, &args![root, HASHED_PROJECTILE_NODE_NAME])
            .ptr();
    }
    node
}

/// `TypeDescriptor`s of `TESObjectREFR` and `Actor`.
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_ACTOR: u32 = 0x0118_46d4;
/// `thiscall` on `this` (the target of `00525750`): five words.
const FN_0050F670: u32 = 0x0050_f670;

// Translated from 00525750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its five words to `0050f670(this, ...)` after a
/// `dynamic_cast` of the second to `Actor` whose result is not used (kept
/// because the call is part of the code); returns that function's result.
pub fn fn_00525750(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    object: u32,
    flag: bool,
    third: u32,
    fourth: u32,
) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![object, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
    );
    e.call(
        FN_0050F670,
        &args![this, first, object, flag, third, fourth],
    )
    .u32()
}

/// The actor's process pointer: `+0x68` of the actor (the engine map names
/// the folded body `MiddleHighProcess::GetSavedAcquireObject`).
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// A reference's position: `+0x30` of the reference (`thiscall`).
const REFERENCE_POSITION: u32 = 0x0043_6aa0;
/// A scene node's world translation: `+0x8C` of the node (`thiscall`).
const NODE_WORLD_TRANSLATION: u32 = 0x0045_bb80;
/// `Actor::GetHeight` (Xbox PDB), `thiscall`, result in `ST0`.
const ACTOR_GET_HEIGHT: u32 = 0x0088_53a0;
/// `AL` of the player's third-person test (`thiscall`).
const PLAYER_BIPED_FLAG: u32 = 0x004e_af60;
/// `PlayerCharacter::GetBiped` (Xbox PDB), `thiscall` (flag).
const PLAYER_GET_BIPED: u32 = 0x0095_0b00;
/// `PlayerCharacter::GetCurrent3D` (Xbox PDB), `thiscall`.
const PLAYER_GET_CURRENT_3D: u32 = 0x0095_0be0;
/// Virtual slot `0x1ec` of an actor: its biped.
const ACTOR_GET_BIPED_SLOT: u32 = 0x1ec;
/// Virtual slot `0x190` of the actor's process (`biped`): the weapon node.
const PROCESS_GET_WEAPON_NODE_SLOT: u32 = 0x190;
/// Virtual slot `0x20c` of a reference: a node.
const REFERENCE_NODE_SLOT: u32 = 0x20c;
/// Virtual slot `0x1d0` of a reference: its 3D root.
const REFERENCE_GET_3D_SLOT: u32 = 0x1d0;

// Translated from 005257a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Where the weapon fires from for `reference`: starts from the
/// reference's position (raised by the actor's height for an actor), and
/// replaces it with the world translation of the weapon node of a
/// ranged/thrown weapon (types 10 to 13) held in the actor's process, or of
/// the fire node found by [`tes_object_weap_get_fire_node`] under the
/// reference's 3D (the player's own 3D for the player), or of the
/// reference's own node when the process has a held object. Writes the
/// three floats to `result` and returns it.
pub fn fn_005257a0(e: &mut Engine, this: Ptr<TesObjectWeap>, result: Ptr, reference: Ptr) -> Ptr {
    let is_actor = e
        .vcall(reference.addr(), REFERENCE_IS_ACTOR_SLOT, &[])
        .bool();
    let actor = if is_actor { reference } else { Ptr::NULL };
    let process = if actor.is_null() {
        0
    } else {
        e.call(ACTOR_PROCESS, &args![actor]).u32()
    };
    let player: u32 = e.global(PLAYER_CHARACTER);
    let is_player = actor.addr() == player;
    let position = e.call(REFERENCE_POSITION, &args![reference]).u32();
    let mut point = [
        e.mem.u32(position),
        e.mem.u32(position + 4),
        e.mem.u32(position + 8),
    ];
    if !actor.is_null() {
        let height = e.call(ACTOR_GET_HEIGHT, &args![actor]).f32();
        let raised = (height as f64 + f32::from_bits(point[2]) as f64) as f32;
        point[2] = raised.to_bits();
    }
    let mut done = false;
    if !actor.is_null() && process != 0 {
        let weapon_type = e.call(GET_WEAPON_TYPE, &args![this]).u32();
        if (10..=13).contains(&weapon_type) {
            let biped = if is_player {
                let flag = e.call(PLAYER_BIPED_FLAG, &args![player]).u8();
                e.call(PLAYER_GET_BIPED, &args![player, flag]).u32()
            } else {
                e.vcall(actor.addr(), ACTOR_GET_BIPED_SLOT, &[]).u32()
            };
            let weapon_node = e
                .vcall(process, PROCESS_GET_WEAPON_NODE_SLOT, &args![biped])
                .u32();
            if weapon_node != 0 {
                let translation = e.call(NODE_WORLD_TRANSLATION, &args![weapon_node]).u32();
                point = [
                    e.mem.u32(translation),
                    e.mem.u32(translation + 4),
                    e.mem.u32(translation + 8),
                ];
            }
            done = true;
        }
    }
    if !done {
        let node = if process != 0 {
            e.vcall(reference.addr(), REFERENCE_NODE_SLOT, &[]).u32()
        } else {
            let root = if is_player {
                e.call(PLAYER_GET_CURRENT_3D, &args![player]).u32()
            } else {
                e.vcall(reference.addr(), REFERENCE_GET_3D_SLOT, &[]).u32()
            };
            tes_object_weap_get_fire_node(e, this, Ptr::new(root)).addr()
        };
        if node != 0 {
            let translation = e.call(NODE_WORLD_TRANSLATION, &args![node]).u32();
            point = [
                e.mem.u32(translation),
                e.mem.u32(translation + 4),
                e.mem.u32(translation + 8),
            ];
        }
    }
    for (index, word) in point.into_iter().enumerate() {
        e.mem.set_u32(result.addr() + 4 * index as u32, word);
    }
    result
}

/// `thiscall` on the held-item object the process returns (`0044ddc0`).
const HELD_ITEM_GET_FORM: u32 = 0x0044_ddc0;
/// The weapon's default ammo (`thiscall` on `BGSAmmoForm`, +0xA4).
const AMMO_FORM_GET_AMMO: u32 = 0x0047_4920;
/// `TypeDescriptor`s of `TESBoundObject` and `TESAmmo`.
const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_TES_AMMO: u32 = 0x0118_40c4;

// Translated from 00525980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::GetCurrentAmmo` (Xbox PDB): the form of the actor's ammo
/// item (process virtual slot `0x14c`) when the actor has one, it is not the
/// same form as its weapon item (slot `0x148`) and it casts to `TESAmmo`;
/// otherwise the weapon's default ammo (`BGSAmmoForm` at +0xA4).
pub fn tes_object_weap_get_current_ammo(
    e: &mut Engine,
    this: Ptr<TesObjectWeap>,
    actor: Ptr,
) -> u32 {
    if !actor.is_null() && e.call(ACTOR_PROCESS, &args![actor]).u32() != 0 {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        if e.vcall(process, PROCESS_AMMO_ITEM_SLOT, &[]).u32() != 0 {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            let held = e.vcall(process, PROCESS_AMMO_ITEM_SLOT, &[]).u32();
            let ammo = e.call(HELD_ITEM_GET_FORM, &args![held]).u32();
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            let other = if e.vcall(process, PROCESS_WEAPON_ITEM_SLOT, &[]).u32() != 0 {
                let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
                let item = e.vcall(process, PROCESS_WEAPON_ITEM_SLOT, &[]).u32();
                e.call(HELD_ITEM_GET_FORM, &args![item]).u32()
            } else {
                0
            };
            if ammo != other {
                let cast = e.call(
                    RT_DYNAMIC_CAST,
                    &args![ammo, 0u32, TYPE_TES_BOUND_OBJECT, TYPE_TES_AMMO, 0u32],
                );
                if cast.u32() != 0 {
                    return ammo;
                }
            }
        }
    }
    e.call(AMMO_FORM_GET_AMMO, &args![this.addr() + AMMO_FORM])
        .u32()
}

// Translated from 00525b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::GetNumProjectiles` (Xbox PDB): `data.cNumProjectiles`;
/// unless `skip_ammo`, when the actor's current ammo has a non-zero
/// projectile count (byte at +0xB0 of the ammo) that count times the
/// weapon's ammo per shot replaces it (as a byte); when `with_mod`, the
/// mod effect `0xC` is added and the sum truncated to a byte.
pub fn tes_object_weap_get_num_projectiles(
    e: &mut Engine,
    this: Ptr<TesObjectWeap>,
    with_mod: bool,
    skip_ammo: bool,
    actor: Ptr,
) -> u8 {
    let mut count = e.get(weapon_data(this), ObjWeap::cNumProjectiles);
    if !skip_ammo && tes_object_weap_get_current_ammo(e, this, actor) != 0 {
        let ammo = tes_object_weap_get_current_ammo(e, this, actor);
        if fn_00525be0(e, Ptr::new(ammo)) > 0 {
            let ammo = tes_object_weap_get_current_ammo(e, this, actor);
            let per_ammo = fn_00525be0(e, Ptr::new(ammo));
            let per_shot = fn_00524b60(e, this);
            count = (per_ammo as u32 * per_shot as u32) as u8;
        }
    }
    if with_mod {
        let effect = e
            .call(GET_MOD_EFFECT_VALUE, &args![this, 0xCu32, 0u32])
            .f64();
        // `FIADD` then `FISTP` with the rounding mode forced to truncation.
        count = (effect + count as f64) as i32 as u8;
    }
    count
}

// Translated from 00525be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0xb0` (of an ammo form).
pub fn fn_00525be0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xB0)
}

// ---- Load ------------------------------------------------------------------------------

/// `TESFile::GetTESForm` (Xbox PDB), `thiscall` on the file: the form type
/// of the record being read (`AL`).
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (Xbox PDB), `thiscall`: the next chunk's
/// four-character code, 0 at the end of the record.
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
/// Advances to the next chunk (`thiscall`, no name in the engine map);
/// `AL` is false at the end of the record.
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// Size of the current chunk (`thiscall`).
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// Whether the file's chunks are big-endian (`thiscall`, `AL`).
const FILE_SWAPS: u32 = 0x0040_1680;
/// `TESFile::GetChunkData` (Xbox PDB), `thiscall` (pointer to a `u32`).
const FILE_GET_CHUNK_U32: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov2` (Xbox PDB), `thiscall` (pointer to a `u16`).
const FILE_GET_CHUNK_U16: u32 = 0x0047_2840;
/// `TESFile::GetChunkData_ov3` (Xbox PDB), `thiscall` (buffer, size).
const FILE_GET_CHUNK_BYTES: u32 = 0x0047_2890;
/// The file's chunk version (`thiscall`, `u16` in `AX`).
const FILE_CHUNK_VERSION: u32 = 0x0040_3570;
/// `TESForm::LoadForm` (Xbox PDB), `thiscall` (file).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESForm::LoadData` (Xbox PDB), `thiscall` (file, `0`, `0`).
const FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `TESFullName`'s chunk loader (`cdecl`, component, file; no name in the
/// engine map).
const FULL_NAME_LOAD: u32 = 0x0048_7050;
/// `TESModel::LoadModelChunk` (Xbox PDB), `cdecl` (model, file).
const MODEL_LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESModel::LoadModelTextureChunk` (Xbox PDB), `cdecl` (model, file).
const MODEL_LOAD_TEXTURE_CHUNK: u32 = 0x0048_93e0;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (Xbox PDB), `thiscall`
/// (data, size, version).
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;
/// `TESTexture::LoadTextureChunk` (Xbox PDB), `cdecl` (texture, file).
const TEXTURE_LOAD_CHUNK: u32 = 0x0048_e3d0;
/// `BGSMessageIcon`'s chunk loader (`cdecl`, icon, file).
const MESSAGE_ICON_LOAD: u32 = 0x0047_a290;
/// `BGSDestructibleObjectForm::LoadChunk` (Xbox PDB), `cdecl` (component,
/// file).
const DESTRUCTIBLE_LOAD_CHUNK: u32 = 0x0047_81e0;
/// Stores a form id in a component (`thiscall`, one word, at `this + 4`;
/// the engine map names the folded body
/// `CombatProcedureAttackMelee::Initialize`).
const STORE_FORM_ID: u32 = 0x006e_cd40;
/// Stores the word at `this + 8` (`thiscall`, one word).
const STORE_WORD_AT_8: u32 = 0x0040_3550;
/// `TESEnchantableForm`'s amount setter (`thiscall`, a `u16` in a word).
const ENCHANTABLE_SET_AMOUNT: u32 = 0x0048_3170;
/// Virtual slot `0xe0` of a form (`LoadObjectBound` in the Xbox PDB).
const FORM_LOAD_OBJECT_BOUND_SLOT: u32 = 0xE0;
/// Virtual slot `0x134` of the weapon: the editor id setter (not named in
/// the Xbox PDB layout).
const FORM_SET_EDITOR_ID_SLOT: u32 = 0x134;
/// `memset` (`cdecl`, destination, value, size).
const MEMSET: u32 = 0x0040_3d30;
/// `BSStringT<char>::Set` (`thiscall`, text, length).
const STRING_ASSIGN: u32 = 0x0040_37f0;
/// Validity tests of an actor value (`cdecl`, one word, `AL`) used for the
/// weapon skill and resistance.
const SKILL_IS_VALID: u32 = 0x0047_f060;
const RESISTANCE_IS_VALID: u32 = 0x0066_eba0;
/// Setters on the weapon for the process-related chunks (`thiscall`, one
/// word; the engine map names the folded bodies `MiddleHighProcess::*`).
const LOAD_NAM_FACE_SKINNED: u32 = 0x008d_8ae0;
const LOAD_NAM_SOUND_CHECK: u32 = 0x0052_15a0;
const LOAD_NAM_SOUND_A: u32 = 0x0052_15d0;
const LOAD_NAM_SOUND_B: u32 = 0x0052_15f0;
const LOAD_NAM_BOUND: u32 = 0x005c_8a30;
const LOAD_NAM_TNAM: u32 = 0x0052_1670;
const LOAD_NAM_UNAM: u32 = 0x0052_16b0;
const LOAD_NAM_NAM6: u32 = 0x0052_1690;
const LOAD_NAM_NAM7: u32 = 0x0052_1650;
const LOAD_NAM_NAM8: u32 = 0x0052_16f0;
const LOAD_NAM_NAM9: u32 = 0x0052_16d0;
const LOAD_NAM_WNAM: u32 = 0x0052_1710;
const LOAD_WMS_CHECK: u32 = 0x0052_1630;
const LOAD_WMS_A: u32 = 0x0052_1610;
const LOAD_WMS_B: u32 = 0x008d_8a80;
const LOAD_WMS2: u32 = 0x008d_8aa0;
/// `(this, number, id)` setters for the `WNM1..7` and `WMI1..3` chunks.
const LOAD_WNM: u32 = 0x0052_14d0;
const LOAD_WMI: u32 = 0x0052_1480;
/// `"MASTERFILE: Clearing invalid type on weapon '%s' (%08X)."`
const CLEARING_INVALID_TYPE_MESSAGE: u32 = 0x0102_ca5c;
/// The two default fire delays (`float` constants `0.1` and `0.5`).
const DEFAULT_FIRE_DELAY_MIN: u32 = 0x0101_e2bc;
const DEFAULT_FIRE_DELAY_MAX: u32 = 0x0101_6248;

/// Chunk codes `Load` handles.
const CHUNK_EDID: u32 = tag(b"EDID");
const CHUNK_OBND: u32 = tag(b"OBND");
const CHUNK_FULL: u32 = tag(b"FULL");
const CHUNK_MODL: u32 = tag(b"MODL");
const CHUNK_MODD: u32 = tag(b"MODD");
const CHUNK_MOSD: u32 = tag(b"MOSD");
const CHUNK_MODT: u32 = tag(b"MODT");
const CHUNK_MODS: u32 = tag(b"MODS");
const CHUNK_ICON: u32 = tag(b"ICON");
const CHUNK_MICO: u32 = tag(b"MICO");
const CHUNK_SCRI: u32 = tag(b"SCRI");
const CHUNK_EITM: u32 = tag(b"EITM");
const CHUNK_ENAM: u32 = tag(b"ENAM");
const CHUNK_ANAM: u32 = tag(b"ANAM");
const CHUNK_EAMT: u32 = tag(b"EAMT");
const CHUNK_NAM0: u32 = tag(b"NAM0");
const CHUNK_DSTD: u32 = tag(b"DSTD");
const CHUNK_DEST: u32 = tag(b"DEST");
const CHUNK_REPL: u32 = tag(b"REPL");
const CHUNK_ETYP: u32 = tag(b"ETYP");
const CHUNK_BIPL: u32 = tag(b"BIPL");
const CHUNK_YNAM: u32 = tag(b"YNAM");
const CHUNK_ZNAM: u32 = tag(b"ZNAM");
const CHUNK_DATA: u32 = tag(b"DATA");
const CHUNK_DNAM: u32 = tag(b"DNAM");
const CHUNK_INAM: u32 = tag(b"INAM");
const CHUNK_NNAM: u32 = tag(b"NNAM");
const CHUNK_SNAM: u32 = tag(b"SNAM");
const CHUNK_TNAM: u32 = tag(b"TNAM");
const CHUNK_UNAM: u32 = tag(b"UNAM");
const CHUNK_VNAM: u32 = tag(b"VNAM");
const CHUNK_WNAM: u32 = tag(b"WNAM");
const CHUNK_XNAM: u32 = tag(b"XNAM");
const CHUNK_NAM1: u32 = tag(b"NAM1");
const CHUNK_NAM6: u32 = tag(b"NAM6");
const CHUNK_NAM7: u32 = tag(b"NAM7");
const CHUNK_NAM8: u32 = tag(b"NAM8");
const CHUNK_NAM9: u32 = tag(b"NAM9");
const CHUNK_VANM: u32 = tag(b"VANM");
const CHUNK_EFSD: u32 = tag(b"EFSD");
const CHUNK_CRDT: u32 = tag(b"CRDT");
const CHUNK_VATS: u32 = tag(b"VATS");
const CHUNK_WMS1: u32 = tag(b"WMS1");
const CHUNK_WMS2: u32 = tag(b"WMS2");
const CHUNK_MOD2: u32 = tag(b"MOD2");
const CHUNK_MOD3: u32 = tag(b"MOD3");
const CHUNK_MOD4: u32 = tag(b"MOD4");
const CHUNK_MO2T: u32 = tag(b"MO2T");
const CHUNK_MO3T: u32 = tag(b"MO3T");
const CHUNK_MO4T: u32 = tag(b"MO4T");

/// A chunk that holds the form id of a mod model slot or a mod object, and
/// the setter slot it goes to: `(chunk, number)`.
const CHUNK_WNM: [(u32, u32); 7] = [
    (tag(b"WNM1"), 1),
    (tag(b"WNM2"), 2),
    (tag(b"WNM3"), 3),
    (tag(b"WNM4"), 4),
    (tag(b"WNM5"), 5),
    (tag(b"WNM6"), 6),
    (tag(b"WNM7"), 7),
];
const CHUNK_WMI: [(u32, u32); 3] = [(tag(b"WMI1"), 1), (tag(b"WMI2"), 2), (tag(b"WMI3"), 4)];
/// `TESModel` path chunks and the model each one fills: the shell casing
/// (`MOD2`, and the older `NAM1`), scope (`MOD3`), world model (`MOD4`) and the
/// seven mod models (`MWD1` to `MWD7`).
const MODEL_PATH_CHUNKS: [(u32, u32); 11] = [
    (CHUNK_MOD2, SHELL_CASING_MODEL),
    (CHUNK_NAM1, SHELL_CASING_MODEL),
    (CHUNK_MOD3, SCOPE_MODEL),
    (CHUNK_MOD4, WORLD_MODEL),
    (tag(b"MWD1"), WORLD_MODEL_MOD_1),
    (tag(b"MWD2"), WORLD_MODEL_MOD_2),
    (tag(b"MWD4"), WORLD_MODEL_MOD_3),
    (tag(b"MWD3"), WORLD_MODEL_MOD_12),
    (tag(b"MWD5"), WORLD_MODEL_MOD_13),
    (tag(b"MWD6"), WORLD_MODEL_MOD_23),
    (tag(b"MWD7"), WORLD_MODEL_MOD_123),
];
/// `TESModel` texture chunks and the model each one fills.
const MODEL_TEXTURE_CHUNKS: [(u32, u32); 10] = [
    (CHUNK_MO2T, SHELL_CASING_MODEL),
    (CHUNK_MO3T, SCOPE_MODEL),
    (CHUNK_MO4T, WORLD_MODEL),
    (tag(b"MWT1"), WORLD_MODEL_MOD_1),
    (tag(b"MWT2"), WORLD_MODEL_MOD_2),
    (tag(b"MWT4"), WORLD_MODEL_MOD_3),
    (tag(b"MWT3"), WORLD_MODEL_MOD_12),
    (tag(b"MWT5"), WORLD_MODEL_MOD_13),
    (tag(b"MWT6"), WORLD_MODEL_MOD_23),
    (tag(b"MWT7"), WORLD_MODEL_MOD_123),
];

/// A `u32` read with `TESFile::GetChunkData` into a fresh word that starts
/// as `initial`.
fn read_chunk_u32(e: &mut Engine, file: Ptr, initial: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), initial);
        e.call(FILE_GET_CHUNK_U32, &args![file, slot]);
        e.mem.u32(slot.addr())
    })
}

/// Reads the whole current chunk into a buffer (the game's stack buffer
/// from `__alloca_probe_16` is a heap block here) and calls `f` with the
/// buffer and the chunk size.
fn with_chunk_bytes<R>(
    e: &mut Engine,
    file: Ptr,
    requested: Option<u32>,
    f: impl FnOnce(&mut Engine, u32, u32) -> R,
) -> R {
    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
    let buffer = e.mem.alloc(size.max(1));
    e.call(
        FILE_GET_CHUNK_BYTES,
        &args![file, buffer, requested.unwrap_or(size)],
    );
    let result = f(e, buffer, size);
    e.mem.free(buffer);
    result
}

/// A `TESModel` path chunk: when the chunk is not empty its bytes go to
/// the model's virtual `SetModel`, else `SetModel(0)`.
fn load_model_path(e: &mut Engine, file: Ptr, model: u32) {
    if e.call(FILE_CHUNK_SIZE, &args![file]).u32() != 0 {
        with_chunk_bytes(e, file, Some(0), |e, buffer, _| {
            e.vcall(model, MODEL_SET_MODEL_SLOT, &args![buffer]);
        });
    } else {
        e.vcall(model, MODEL_SET_MODEL_SLOT, &args![0u32]);
    }
}

/// The weapon's `u32` field store used by the setter chunks: the word read
/// from the chunk goes to `function(this, ...)`.
fn load_word_into(e: &mut Engine, file: Ptr, function: u32, leading: &[u32]) {
    let word = read_chunk_u32(e, file, 0);
    let mut words = leading.to_vec();
    words.push(word);
    e.call(function, &words);
}

/// Defaults the DATA block before it is read: zeroes all 0xCC bytes and
/// sets the two semi-automatic fire delays.
fn reset_data_block(e: &mut Engine, this: Ptr<TesObjectWeap>) {
    let data = weapon_data(this);
    e.call(MEMSET, &args![data, 0u32, 0xCCu32]);
    let minimum: f32 = e.global(DEFAULT_FIRE_DELAY_MIN);
    e.set(data, ObjWeap::fSemiAutomaticFireDelayMin, minimum);
    let maximum: f32 = e.global(DEFAULT_FIRE_DELAY_MAX);
    e.set(data, ObjWeap::fSemiAutomaticFireDelayMax, maximum);
}

/// The DATA chunk (`DNAM`): older records (version below 12 and 14) have a
/// shorter block and attack animation numbers from an older numbering,
/// which are shifted up; the newest records are validated field by field.
fn load_data_chunk(e: &mut Engine, this: Ptr<TesObjectWeap>, file: Ptr) {
    let data = weapon_data(this);
    let version = e.call(FILE_CHUNK_VERSION, &args![file]).u16();
    if version < 12 {
        reset_data_block(e, this);
        let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
        e.call(FILE_GET_CHUNK_BYTES, &args![file, data, size]);
        if e.call(FILE_SWAPS, &args![file]).bool() {
            obj_weap_endian(e, data);
        }
        let animation = e.get(data, ObjWeap::cAttackAnim);
        if (0x38..0xA8).contains(&animation) {
            let animation = animation.wrapping_add(0x12);
            e.set(data, ObjWeap::cAttackAnim, animation);
            if (0x56..0xA8).contains(&animation) {
                e.set(data, ObjWeap::cAttackAnim, animation.wrapping_add(6));
            }
        }
    } else if version < 14 {
        reset_data_block(e, this);
        let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
        e.call(FILE_GET_CHUNK_BYTES, &args![file, data, size]);
        if e.call(FILE_SWAPS, &args![file]).bool() {
            obj_weap_endian(e, data);
        }
        let animation = e.get(data, ObjWeap::cAttackAnim);
        if (0x56..0xA8).contains(&animation) {
            e.set(data, ObjWeap::cAttackAnim, animation.wrapping_add(6));
        }
    } else {
        reset_data_block(e, this);
        e.call(FILE_GET_CHUNK_BYTES, &args![file, data, 0xCCu32]);
        if e.call(FILE_SWAPS, &args![file]).bool() {
            obj_weap_endian(e, data);
        }
        let zero: f64 = e.global(ZERO);
        if e.get(data, ObjWeap::fLimbDamageMult) as f64 <= zero {
            e.set(data, ObjWeap::fLimbDamageMult, 1.0f32);
        }
        let hand_grip = e.get(data, ObjWeap::cHandGripAnim);
        if !((0xE6..=0xEB).contains(&hand_grip) || hand_grip == 0xFF) {
            e.set(data, ObjWeap::cHandGripAnim, 0xFF);
        }
        let skill = e.get(data, ObjWeap::eSkill);
        if !e.call(SKILL_IS_VALID, &args![skill]).bool() {
            e.set(data, ObjWeap::eSkill, 0x2D);
        }
        let resistance = e.get(data, ObjWeap::eResistance);
        if !e.call(RESISTANCE_IS_VALID, &args![resistance]).bool() && resistance != 0xFFFF_FFFF {
            e.set(data, ObjWeap::eResistance, 0xFFFF_FFFF);
        }
    }
}

// Translated from 0051f630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::Load` (Xbox PDB): reads a weapon record (form type 0x28)
/// from `file` and returns whether it did. Starts with the form header
/// (`TESForm::LoadForm`) and clears bit 3 of the form flags, then loops over
/// the record's chunks and handles each (editor id, name, models and their
/// texture swaps, icon, script, enchantment, ammo, destruction data, repair
/// list, equip type, biped model list, pickup and putdown sounds, the DATA
/// block with the fix-ups for older versions, the critical-hit and VATS
/// blocks, embedded weapon node and VATS attack name, sound level and the
/// form ids of the objects, sounds and mods); unknown chunks are skipped.
/// After the record, a weapon without projectiles but with a projectile
/// gets one, and a weapon type of 14 or more is logged and cleared.
///
/// Not translated: the stack-protector cookie of the compiled function and
/// the `alloca` buffers (heap blocks here).
// Translation incomplete: 0051f630 - the compiler's security cookie check (`__security_check_cookie`) is not translated.
pub fn tes_object_weap_load(e: &mut Engine, this: Ptr<TesObjectWeap>, file: Ptr) -> bool {
    let weapon = this.addr();
    if e.call(FILE_GET_FORM_TYPE, &args![file]).u8() != 0x28 {
        return false;
    }
    e.call(FORM_LOAD_FORM, &args![this, file]);
    e.call(FORM_SET_FLAG_8, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        load_chunk(e, this, file, chunk);
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    let data = weapon_data(this);
    if tes_object_weap_get_num_projectiles(e, this, false, true, Ptr::NULL) == 0
        && e.call(GET_PROJECTILE, &args![this, 0u32]).u32() != 0
    {
        fn_00521580(e, this, 1);
    }
    if e.get(data, ObjWeap::eType) as i32 >= 0xE {
        let form_id = e.call(GET_FORM_ID, &args![this]).u32();
        let name = e.vcall(weapon, FORM_GET_OBJECT_TYPE_NAME_SLOT, &[]).u32();
        e.call(
            LOG_MESSAGE,
            &args![CLEARING_INVALID_TYPE_MESSAGE, name, form_id],
        );
        e.set(data, ObjWeap::eType, 0);
    }
    true
}

/// Handles one chunk of a weapon record.
fn load_chunk(e: &mut Engine, this: Ptr<TesObjectWeap>, file: Ptr, chunk: u32) {
    let weapon = this.addr();
    match chunk {
        CHUNK_EDID => {
            // The editor id goes to the weapon's virtual setter.
            with_chunk_bytes(e, file, Some(0x200), |e, buffer, _| {
                e.vcall(weapon, FORM_SET_EDITOR_ID_SLOT, &args![buffer]);
            });
        }
        CHUNK_OBND => {
            e.vcall(weapon, FORM_LOAD_OBJECT_BOUND_SLOT, &args![file]);
        }
        CHUNK_FULL => {
            let component = if weapon == 0 { 0 } else { weapon + FULL_NAME };
            e.call(FULL_NAME_LOAD, &args![component, file]);
        }
        CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
            let model = if weapon == 0 { 0 } else { weapon + MODEL };
            e.call(MODEL_LOAD_MODEL_CHUNK, &args![model, file]);
        }
        CHUNK_MODS => {
            let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
            let buffer = e.mem.alloc(size.max(1));
            e.call(FILE_GET_CHUNK_BYTES, &args![file, buffer, size]);
            let version = e.call(FILE_CHUNK_VERSION, &args![file]).u16();
            e.call(
                UNPACK_TEXTURE_SWAP,
                &args![weapon + MODEL, buffer, size, version],
            );
            e.mem.free(buffer);
        }
        CHUNK_ICON => {
            let component = if weapon == 0 { 0 } else { weapon + ICON };
            e.call(TEXTURE_LOAD_CHUNK, &args![component, file]);
        }
        CHUNK_MICO => {
            let component = if weapon == 0 {
                0
            } else {
                weapon + MESSAGE_ICON
            };
            e.call(MESSAGE_ICON_LOAD, &args![component, file]);
        }
        CHUNK_SCRI => {
            let id = read_chunk_u32(e, file, 0);
            e.call(STORE_FORM_ID, &args![weapon + SCRIPTABLE_FORM, id]);
            e.call(
                SCRIPTABLE_FORM_INIT_ITEM,
                &args![weapon + SCRIPTABLE_FORM, weapon],
            );
        }
        CHUNK_EITM | CHUNK_ENAM => {
            load_word_into(e, file, STORE_FORM_ID, &[weapon + ENCHANTABLE_FORM]);
        }
        CHUNK_ANAM | CHUNK_EAMT => {
            let amount = e.with_stack(2, |e, slot| {
                e.mem.set_u16(slot.addr(), 0);
                e.call(FILE_GET_CHUNK_U16, &args![file, slot]);
                e.mem.u16(slot.addr())
            });
            e.call(
                ENCHANTABLE_SET_AMOUNT,
                &args![weapon + ENCHANTABLE_FORM, amount as u32],
            );
        }
        CHUNK_NAM0 => load_word_into(e, file, STORE_FORM_ID, &[weapon + AMMO_FORM]),
        CHUNK_DSTD | CHUNK_DEST => {
            let component = if weapon == 0 {
                0
            } else {
                weapon + DESTRUCTIBLE_FORM
            };
            e.call(DESTRUCTIBLE_LOAD_CHUNK, &args![component, file]);
        }
        CHUNK_REPL => load_word_into(e, file, STORE_FORM_ID, &[weapon + REPAIR_ITEM_LIST]),
        CHUNK_ETYP => load_word_into(e, file, STORE_FORM_ID, &[weapon + EQUIP_TYPE]),
        CHUNK_BIPL => load_word_into(e, file, STORE_FORM_ID, &[weapon + BIPED_MODEL_LIST]),
        CHUNK_YNAM => load_word_into(e, file, STORE_FORM_ID, &[weapon + PICKUP_PUTDOWN_SOUNDS]),
        CHUNK_ZNAM => load_word_into(e, file, STORE_WORD_AT_8, &[weapon + PICKUP_PUTDOWN_SOUNDS]),
        CHUNK_DATA => {
            e.call(FORM_LOAD_DATA, &args![this, file, 0u32, 0u32]);
        }
        CHUNK_DNAM => load_data_chunk(e, this, file),
        CHUNK_INAM => load_word_into(e, file, LOAD_NAM_FACE_SKINNED, &[weapon]),
        CHUNK_SNAM => {
            let id = read_chunk_u32(e, file, 0);
            if e.call(LOAD_NAM_SOUND_CHECK, &args![this, 0u32]).u32() != 0 {
                e.call(LOAD_NAM_SOUND_B, &args![this, id]);
            } else {
                e.call(LOAD_NAM_SOUND_A, &args![this, id]);
            }
        }
        CHUNK_XNAM => load_word_into(e, file, LOAD_NAM_BOUND, &[weapon]),
        CHUNK_TNAM => load_word_into(e, file, LOAD_NAM_TNAM, &[weapon]),
        CHUNK_UNAM => load_word_into(e, file, LOAD_NAM_UNAM, &[weapon]),
        CHUNK_NAM6 => load_word_into(e, file, LOAD_NAM_NAM6, &[weapon]),
        CHUNK_NAM7 => load_word_into(e, file, LOAD_NAM_NAM7, &[weapon]),
        CHUNK_NAM8 => load_word_into(e, file, LOAD_NAM_NAM8, &[weapon]),
        CHUNK_NAM9 => load_word_into(e, file, LOAD_NAM_NAM9, &[weapon]),
        CHUNK_WNAM => load_word_into(e, file, LOAD_NAM_WNAM, &[weapon]),
        CHUNK_VNAM => {
            // The word is not cleared first; it is read straight from the
            // chunk.
            let level = read_chunk_u32(e, file, 0);
            let level = if level > 3 { 1 } else { level };
            e.set(this, TesObjectWeap::eSoundLevel, level);
        }
        CHUNK_NNAM => {
            with_chunk_bytes(e, file, Some(0), |e, buffer, _| {
                e.call(
                    STRING_ASSIGN,
                    &args![weapon + EMBEDDED_WEAPON_NODE, buffer, 0u32],
                );
            });
        }
        CHUNK_VANM => {
            with_chunk_bytes(e, file, Some(0), |e, buffer, _| {
                e.call(
                    STRING_ASSIGN,
                    &args![weapon + VATS_ATTACK_NAME, buffer, 0u32],
                );
            });
        }
        CHUNK_EFSD => {
            let id = read_chunk_u32(e, file, 0);
            e.call(SET_SCOPE_EFFECT, &args![this, id]);
        }
        CHUNK_CRDT => {
            let block = weapon + CRITICAL_DATA;
            e.call(FILE_GET_CHUNK_BYTES, &args![file, block, 0x10u32]);
            if e.call(FILE_SWAPS, &args![file]).bool() {
                e.call(CRITICAL_DATA_ENDIAN, &args![block]);
            }
        }
        CHUNK_VATS => {
            let block = weapon + VATS_SPECIAL_DATA;
            e.call(FILE_GET_CHUNK_BYTES, &args![file, block, 0x14u32]);
            if e.call(FILE_SWAPS, &args![file]).bool() {
                e.call(VATS_SPECIAL_ENDIAN, &args![block]);
            }
        }
        CHUNK_WMS1 => {
            let id = read_chunk_u32(e, file, 0);
            if e.call(LOAD_WMS_CHECK, &args![this]).u32() != 0 {
                e.call(LOAD_WMS_B, &args![this, id]);
            } else {
                e.call(LOAD_WMS_A, &args![this, id]);
            }
        }
        CHUNK_WMS2 => load_word_into(e, file, LOAD_WMS2, &[weapon]),
        _ => {
            for (id, number) in CHUNK_WNM {
                if chunk == id {
                    load_word_into(e, file, LOAD_WNM, &[weapon, number]);
                    return;
                }
            }
            for (id, number) in CHUNK_WMI {
                if chunk == id {
                    load_word_into(e, file, LOAD_WMI, &[weapon, number]);
                    return;
                }
            }
            for (path_chunk, model) in MODEL_PATH_CHUNKS {
                if chunk == path_chunk {
                    load_model_path(e, file, weapon + model);
                    return;
                }
            }
            for (texture_chunk, model) in MODEL_TEXTURE_CHUNKS {
                if chunk == texture_chunk {
                    e.call(MODEL_LOAD_TEXTURE_CHUNK, &args![weapon + model, file]);
                    return;
                }
            }
        }
    }
}

// ---- ExpelShellCasing ------------------------------------------------------------------

/// The shell casing model getter (`thiscall`: `this + 0x1D0`).
const GET_SHELL_CASING_MODEL: u32 = 0x0046_00d0;
/// `"ShellCasingNode"`.
const SHELL_CASING_NODE_NAME: u32 = 0x0102_cb00;
/// `cdecl` (x, y, z, scene node, range): whether the point is within
/// `range` of the node (`AL`; false for a null node).
const POINT_NEAR_NODE: u32 = 0x004b_61d0;
/// Distance, in game units, beyond which the casing is not shown.
const CASING_RANGE: u32 = 0x0101_e704;
/// `thiscall` on a vector (`this`, result, other): `result = this - other`,
/// returned.
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
/// Length of the vector (`thiscall`, result in `ST0`).
const VECTOR_LENGTH: u32 = 0x0045_7990;
/// A node's world rotation (`thiscall`: `this + 0x68`).
const NODE_WORLD_ROTATION: u32 = 0x0046_1130;
/// `thiscall` on a 3x3 matrix (`this`, column, result): the column
/// `column` as a vector.
const MATRIX_GET_COLUMN: u32 = 0x0043_9f50;
/// `RandomFloat` (Xbox PDB), `cdecl` (minimum, maximum), result in `ST0`.
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// Vector constructor (`thiscall`, `this`, x, y, z), returns `this`.
const VECTOR_CONSTRUCT: u32 = 0x0041_6870;
/// `thiscall` on a vector (`this`, other): `this += other`, returned.
const VECTOR_ADD_ASSIGN: u32 = 0x0063_c8a0;
/// `thiscall` on a vector (`this`, scale): `this *= scale`, returned.
const VECTOR_SCALE_ASSIGN: u32 = 0x0043_9180;
/// The character controller of a mobile object (`thiscall`; virtual slot
/// `0x28c` of its process).
const MOBILE_GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `cdecl` (result vector, matrix): three angles taken from the matrix.
const MATRIX_ANGLES: u32 = 0x0045_8620;
/// Memory manager allocation (`cdecl`, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The temporary effect that is the shell casing (constructor `thiscall`,
/// 11 words: the actor's `+0x40` word, the lifetime, the model path, the
/// actor, the position, the rotation, the eject velocity, the spin, `1.0`,
/// `0`, `1`; the engine map files it under `bstempeffectdebris.cpp`).
const DEBRIS_CONSTRUCT: u32 = 0x0068_2c40;
/// `thiscall` on that object (`1`).
const DEBRIS_FINISH: u32 = 0x007e_6580;
/// `min(a, b)` of two `float`s (`cdecl`, result in `ST0`).
const MIN_FLOAT: u32 = 0x0040_ebd0;
/// `thiscall` on an actor: the word at `+0x40`.
const ACTOR_WORD_40: u32 = 0x008d_6f30;
/// The game settings the casing code reads (`thiscall` `00403e20`: pointer
/// to the `float`): `fGunShellCameraDistance`, `fGunShellDirectionRandomize`,
/// `fGunShellEjectSpeed`, `fGunShellRotateRandomize`, `fGunShellRotateSpeed`
/// and `fGunShellLifetime` (names from the exe's static initializers).
const SETTING_GUN_SHELL_CAMERA_DISTANCE: u32 = 0x011c_f57c;
const SETTING_GUN_SHELL_DIRECTION_RANDOMIZE: u32 = 0x011c_f4b0;
const SETTING_GUN_SHELL_EJECT_SPEED: u32 = 0x011c_f498;
const SETTING_GUN_SHELL_ROTATE_RANDOMIZE: u32 = 0x011c_e0e4;
const SETTING_GUN_SHELL_ROTATE_SPEED: u32 = 0x011c_f1d4;
const SETTING_GUN_SHELL_LIFETIME: u32 = 0x011c_e7c0;
/// The default direction (three `float`s at `011a946c`).
const DEFAULT_DIRECTION: u32 = 0x011a_946c;
/// The object whose word at `+8` `00525430` tests.
const OBJECT_011F2250: u32 = 0x011f_2250;
/// `2.0` as a `float` (`010162c0`).
const TWO: u32 = 0x0101_62c0;

/// The float a setting object holds (`thiscall` `00403e20`).
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let pointer = e.call(SETTING_GET_FLOAT_POINTER, &args![setting]).u32();
    e.mem.f32(pointer)
}

/// The three words at the vector `pointer`.
fn vector_words(e: &Engine, pointer: u32) -> [u32; 3] {
    [
        e.mem.u32(pointer),
        e.mem.u32(pointer + 4),
        e.mem.u32(pointer + 8),
    ]
}

/// Scratch space of [`tes_object_weap_expel_shell_casing`]: the offsets of
/// its local vectors and matrices inside one block.
const LOCAL_DELTA: u32 = 0x00;
const LOCAL_EJECT_VELOCITY: u32 = 0x10;
const LOCAL_POSITION: u32 = 0x20;
const LOCAL_ROTATION: u32 = 0x30;
const LOCAL_EJECT_JITTER: u32 = 0x60;
const LOCAL_CONTROLLER_ANGLES: u32 = 0x70;
const LOCAL_SPIN_DIRECTION: u32 = 0x80;
const LOCAL_SPIN_LOCAL: u32 = 0x90;
const LOCAL_SPIN_ROTATED: u32 = 0xA0;
const LOCAL_SPIN: u32 = 0xB0;
const LOCAL_BLOCK_SIZE: u32 = 0xC0;

// Translated from 00524db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::ExpelShellCasing` (Xbox PDB): spawns a shell casing for
/// an actor who has just fired. Does nothing unless `actor` is an actor,
/// the weapon has a shell casing model, casings are not disabled
/// (`011dea2a`), and the actor's `"ShellCasingNode"` is within 128 units of
/// the camera object and within `fGunShellCameraDistance`. The casing leaves
/// the node along the third column of the node's rotation, randomized by
/// `fGunShellDirectionRandomize` and scaled by `fGunShellEjectSpeed` (plus
/// the angles of the actor's character controller), and spins about a
/// direction randomized by `fGunShellRotateRandomize`, normalized, scaled by
/// `fGunShellRotateSpeed` and rotated by the node's rotation; it lives
/// `fGunShellLifetime` (at most 2 for the player in first person, who also
/// has the casing finished with `007e6580(1)` unless `011f2250` is set).
///
/// Not translated: the C++ exception frame of the compiled function.
// Translation incomplete: 00524db0 - the C++ exception unwinding frame (state-based cleanup) is not translated.
pub fn tes_object_weap_expel_shell_casing(e: &mut Engine, this: Ptr<TesObjectWeap>, actor: Ptr) {
    if actor.is_null() || !e.vcall(actor.addr(), REFERENCE_IS_ACTOR_SLOT, &[]).bool() {
        return;
    }
    let model = e.call(GET_SHELL_CASING_MODEL, &args![this]).u32();
    if e.call(MODEL_HAS_FILE, &args![model]).u32() == 0 {
        return;
    }
    if fn_00525420(e) != 0 {
        return;
    }
    let player: u32 = e.global(PLAYER_CHARACTER);
    let root = if actor.addr() == player {
        e.call(PLAYER_GET_CURRENT_3D, &args![player]).u32()
    } else {
        e.vcall(actor.addr(), REFERENCE_GET_3D_SLOT, &[]).u32()
    };
    let node = e
        .call(FIND_NODE_BY_NAME, &args![root, SHELL_CASING_NODE_NAME])
        .u32();
    if node == 0 {
        return;
    }
    e.with_stack(LOCAL_BLOCK_SIZE, |e, block| {
        expel_from_node(e, this, actor, node, block.addr());
    });
}

/// The part of `ExpelShellCasing` after the node is found; `block` holds
/// its local vectors.
fn expel_from_node(e: &mut Engine, this: Ptr<TesObjectWeap>, actor: Ptr, node: u32, block: u32) {
    let player: u32 = e.global(PLAYER_CHARACTER);
    let mut near = true;
    let range: f32 = e.global(CASING_RANGE);
    let camera = fn_00524c90(e);
    let position = e.call(NODE_WORLD_TRANSLATION, &args![node]).u32();
    let [x, y, z] = vector_words(e, position);
    if !e
        .call(POINT_NEAR_NODE, &args![x, y, z, camera, range])
        .bool()
    {
        near = false;
    }
    let camera = fn_00524c90(e);
    let camera_position = e.call(NODE_WORLD_TRANSLATION, &args![camera]).u32();
    let node_position = e.call(NODE_WORLD_TRANSLATION, &args![node]).u32();
    e.call(
        VECTOR_SUBTRACT,
        &args![node_position, block + LOCAL_DELTA, camera_position],
    );
    let length = e.call(VECTOR_LENGTH, &args![block + LOCAL_DELTA]).f64();
    if (setting_float(e, SETTING_GUN_SHELL_CAMERA_DISTANCE) as f64) < length {
        near = false;
    }
    if !near {
        return;
    }

    let model = e.call(GET_SHELL_CASING_MODEL, &args![this]).u32();
    let path = e.vcall(model, MODEL_GET_MODEL_SLOT, &[]).u32();
    let node_position = e.call(NODE_WORLD_TRANSLATION, &args![node]).u32();
    for (index, word) in vector_words(e, node_position).into_iter().enumerate() {
        e.mem
            .set_u32(block + LOCAL_POSITION + 4 * index as u32, word);
    }
    let rotation = e.call(NODE_WORLD_ROTATION, &args![node]).u32();
    for index in 0..9 {
        let word = e.mem.u32(rotation + 4 * index);
        e.mem.set_u32(block + LOCAL_ROTATION + 4 * index, word);
    }

    // The eject velocity: the node's third rotation column with a random
    // spread and the character controller.s angles added, times the eject
    // speed.
    let rotation = e.call(NODE_WORLD_ROTATION, &args![node]).u32();
    e.call(
        MATRIX_GET_COLUMN,
        &args![rotation, 2u32, block + LOCAL_EJECT_VELOCITY],
    );
    let limit_a = setting_float(e, SETTING_GUN_SHELL_DIRECTION_RANDOMIZE);
    let limit_b = setting_float(e, SETTING_GUN_SHELL_DIRECTION_RANDOMIZE);
    let first = e.call(RANDOM_FLOAT, &args![-limit_a, limit_b]).f32();
    let second = e.call(RANDOM_FLOAT, &args![-limit_a, limit_b]).f32();
    let third = e.call(RANDOM_FLOAT, &args![-limit_a, limit_b]).f32();
    let random = e
        .call(
            VECTOR_CONSTRUCT,
            &args![block + LOCAL_EJECT_JITTER, third, second, first],
        )
        .u32();
    e.call(
        VECTOR_ADD_ASSIGN,
        &args![block + LOCAL_EJECT_VELOCITY, random],
    );
    let eject_speed = setting_float(e, SETTING_GUN_SHELL_EJECT_SPEED);
    e.call(
        VECTOR_SCALE_ASSIGN,
        &args![block + LOCAL_EJECT_VELOCITY, eject_speed],
    );
    if e.call(MOBILE_GET_CHAR_CONTROLLER, &args![actor]).u32() != 0 {
        let controller = e.call(MOBILE_GET_CHAR_CONTROLLER, &args![actor]).u32();
        e.call(
            MATRIX_ANGLES,
            &args![block + LOCAL_CONTROLLER_ANGLES, controller + 0x4F0],
        );
        e.call(
            VECTOR_ADD_ASSIGN,
            &args![
                block + LOCAL_EJECT_VELOCITY,
                block + LOCAL_CONTROLLER_ANGLES
            ],
        );
    }

    // The spin: the default direction with a random length, normalized,
    // scaled by the rotate speed on the x and z axes and rotated by the
    // node's rotation.
    for index in 0..3 {
        let word = e.mem.u32(DEFAULT_DIRECTION + 4 * index);
        e.mem
            .set_u32(block + LOCAL_SPIN_DIRECTION + 4 * index, word);
    }
    let spread_a = setting_float(e, SETTING_GUN_SHELL_ROTATE_RANDOMIZE);
    let spread_b = setting_float(e, SETTING_GUN_SHELL_ROTATE_RANDOMIZE);
    let random = e.call(RANDOM_FLOAT, &args![-spread_a, spread_b]).f32();
    // x87: `1.0 + random` is stored as a float.
    let factor = (1.0f64 + random as f64) as f32;
    e.call(
        VECTOR_SCALE_ASSIGN,
        &args![block + LOCAL_SPIN_DIRECTION, factor],
    );
    fn_00525340(e, Ptr::new(block + LOCAL_SPIN_DIRECTION));
    let direction_z = e.mem.f32(block + LOCAL_SPIN_DIRECTION + 8);
    let speed_z =
        (setting_float(e, SETTING_GUN_SHELL_ROTATE_SPEED) as f64 * direction_z as f64) as f32;
    let direction_x = e.mem.f32(block + LOCAL_SPIN_DIRECTION);
    let speed_x =
        (setting_float(e, SETTING_GUN_SHELL_ROTATE_SPEED) as f64 * direction_x as f64) as f32;
    e.call(
        VECTOR_CONSTRUCT,
        &args![block + LOCAL_SPIN_LOCAL, speed_x, 0.0f32, speed_z],
    );
    let rotation = e.call(NODE_WORLD_ROTATION, &args![node]).u32();
    let rotated = e
        .call(
            MATRIX_MULTIPLY_VECTOR,
            &args![
                rotation,
                block + LOCAL_SPIN_ROTATED,
                block + LOCAL_SPIN_LOCAL
            ],
        )
        .u32();
    for (index, word) in vector_words(e, rotated).into_iter().enumerate() {
        e.mem.set_u32(block + LOCAL_SPIN + 4 * index as u32, word);
    }

    let first_person_player = actor.addr() == player
        && e.call(PLAYER_BIPED_FLAG, &args![player]).u8() == 0
        && !fn_00525430(e, Ptr::new(OBJECT_011F2250));
    let object = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
    let created = if object == 0 {
        0
    } else {
        let mut lifetime = setting_float(e, SETTING_GUN_SHELL_LIFETIME);
        if first_person_player {
            let two: f32 = e.global(TWO);
            lifetime = e.call(MIN_FLOAT, &args![lifetime, two]).f32();
        }
        let actor_word = e.call(ACTOR_WORD_40, &args![actor]).u32();
        e.call(
            DEBRIS_CONSTRUCT,
            &args![
                object,
                actor_word,
                lifetime,
                path,
                actor,
                block + LOCAL_POSITION,
                block + LOCAL_ROTATION,
                block + LOCAL_EJECT_VELOCITY,
                block + LOCAL_SPIN,
                1.0f32,
                0u32,
                1u32
            ],
        )
        .u32()
    };
    if first_person_player {
        e.call(DEBRIS_FINISH, &args![created, 1u32]);
    }
}

// ---- The firing function --------------------------------------------------------------

/// `thiscall` on the ammo form: the ammo's effect (a `TESForm`).
const AMMO_GET_EFFECT: u32 = 0x0097_ae90;
/// `thiscall` on the ammo form: the chance of the effect, a `float` in
/// `ST0`.
const AMMO_GET_EFFECT_CHANCE: u32 = 0x006d_6d90;
/// `BGSEntryPoint::HandleEntryPoint` (Xbox PDB), `cdecl` (entry point,
/// actor, item, pointer to the value).
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// The entry point the ammo effect chance goes through.
const ENTRY_POINT_AMMO_EFFECT_CHANCE: u32 = 0x39;
/// A random 32-bit number (`cdecl`, no arguments).
const RANDOM_INTEGER: u32 = 0x0048_7f50;
/// Virtual slot `0xe4` of the ammo's effect: whether it can be applied.
const EFFECT_CAN_APPLY_SLOT: u32 = 0xE4;
/// Virtual slot `0x190` of the player: applies a form (form, `0`, `1`).
const PLAYER_APPLY_FORM_SLOT: u32 = 0x190;
/// The `TypeDescriptor` of `TESFullName` (`.?AVTESFullName@@`) the code
/// casts the ammo effect to (the result is not used).
const TYPE_TES_FULL_NAME: u32 = 0x0118_3158;
/// A non-zero word at this offset of an actor stops the firing function.
const ACTOR_BLOCKS_FIRING_OFFSET: u32 = 0x1A0;
/// The global vector (`011a9484`) scaled by `96.0` (`float` at `0102cafc`)
/// and added to an actor's position to get the firing position.
const FIRING_OFFSET_VECTOR: u32 = 0x011a_9484;
const FIRING_OFFSET_SCALE: u32 = 0x0102_cafc;
/// `MobileObject::GetLooking` (Xbox PDB), `thiscall`, result in `ST0`.
const MOBILE_GET_LOOKING: u32 = 0x0093_1d70;
/// Virtual slot `0x2bc` of an actor (one word): a `float` angle in `ST0`.
const ACTOR_GET_ANGLE_SLOT: u32 = 0x2BC;
/// `thiscall` on a reference: a pointer to its three rotation angles.
const REFERENCE_ROTATION_ANGLES: u32 = 0x0043_0830;
/// Virtual slot `0x148` of an actor's process: the weapon item; slot
/// `0x14c`: the ammo item.
const PROCESS_WEAPON_ITEM_SLOT: u32 = 0x148;
const PROCESS_AMMO_ITEM_SLOT: u32 = 0x14c;
/// `ItemChange::HasModEffectActive_ov2` (Xbox PDB), `thiscall` (item,
/// effect).
const ITEM_HAS_MOD_EFFECT: u32 = 0x004b_da70;
/// `ItemChange::HasModEffectActive` (Xbox PDB), `thiscall` (item, effect,
/// pointer to a `float`).
const ITEM_HAS_MOD_EFFECT_VALUE: u32 = 0x004b_d8d0;
/// `thiscall` on an item or a handle: a count or a reference (`00726070`;
/// the engine map has no name).
const HANDLE_RESOLVE: u32 = 0x0072_6070;
/// `thiscall` on a scene node (`0045bc00`, `0043b4a0`, `00450f90`): the
/// three steps the code takes on the node of a held weapon that has run
/// out of ammo.
const NODE_STEP_A: u32 = 0x0045_bc00;
const NODE_STEP_B: u32 = 0x0043_b4a0;
const NODE_STEP_C: u32 = 0x0045_0f90;
/// Virtual slot `0xc` of a scene node: its `NiNode` view (`AsNode`).
const NODE_AS_NODE_SLOT: u32 = 0xC;
/// `Actor::GetIronSights` (Xbox PDB), `thiscall`.
const ACTOR_GET_IRON_SIGHTS: u32 = 0x008b_bc10;
/// `thiscall` on the weapon: the sight model (`00504e60`).
const WEAPON_SIGHT_MODEL: u32 = 0x0050_4e60;
/// `thiscall` on the weapon (`004ad030`, `AL`).
const WEAPON_FLAG_TEST_4AD030: u32 = 0x004a_d030;
/// The VATS object (`011f2250`) and its getters: `00441110` is named
/// `PathingLocation::GetWorldspace` by the engine map (identical-code
/// folding), `0044ddc0` returns its mode, `009c71c0` its current action.
const VATS_OBJECT: u32 = 0x011f_2250;
const VATS_GET_MODE: u32 = 0x0044_ddc0;
const VATS_GET_CURRENT_ACTION: u32 = 0x009c_71c0;
const VATS_GET_WORLDSPACE: u32 = 0x0044_1110;
/// The two actors that fire for the VATS object (`011f21e8`, `011f21ec`)
/// and the actor it acts for (`011f21f0`).
const VATS_ACTOR_A: u32 = 0x011f_21e8;
const VATS_ACTOR_B: u32 = 0x011f_21ec;
const VATS_ACTOR: u32 = 0x011f_21f0;
/// `thiscall` on the VATS object (one word): records a shot (`009c8b90`)
/// and finishes the attack (`009c8c40`).
const VATS_RECORD_SHOT: u32 = 0x009c_8b90;
const VATS_FINISH_ATTACK: u32 = 0x009c_8c40;
/// A function-local static vector `(0, -5, 0)` (`011c950c`), its guard word
/// (`011c9518`, bit 0) and the `float` `-5.0` (`0102caf8`).
const SIGHT_OFFSET_VECTOR: u32 = 0x011c_950c;
const SIGHT_OFFSET_GUARD: u32 = 0x011c_9518;
const SIGHT_OFFSET_Y: u32 = 0x0102_caf8;
/// The game settings read by the firing code (names from the exe's static
/// initializers): the boolean `bLaserSights:Combat` and `bAimSights:Combat`
/// (`00408d60`, a byte) and the `float` `fGunReferenceSkill`,
/// `fGrenadeHighArcSpeedPercentage`, `fNPCMaxGunWobbleAngle`,
/// `fVATSShotBurstTime` and `fVATSShotLongBurstTime` (`00403e20`).
const SETTING_LASER_SIGHTS: u32 = 0x011c_94a0;
const SETTING_AIM_SIGHTS: u32 = 0x011c_94b0;
const SETTING_GUN_REFERENCE_SKILL: u32 = 0x011c_e23c;
const SETTING_GRENADE_HIGH_ARC_SPEED_PERCENTAGE: u32 = 0x011c_fb78;
const SETTING_NPC_MAX_GUN_WOBBLE_ANGLE: u32 = 0x011c_e034;
const SETTING_VATS_SHOT_BURST_TIME: u32 = 0x011c_f63c;
const SETTING_VATS_SHOT_LONG_BURST_TIME: u32 = 0x011c_f108;
/// The VATS fire-rate fraction accumulated across shots (`float`).
const VATS_FIRE_ACCUMULATOR: u32 = 0x011c_9508;
/// `float` and `double` constants: `1.0` (double), `2*pi`, `pi/2`,
/// `4096.0`, `1000.0` (double), `pi/180` (double) and `512.0`.
const DOUBLE_ONE: u32 = 0x0101_2070;
const TWO_PI: u32 = 0x0101_ff50;
const HALF_PI: u32 = 0x0101_ff38;
const CYLINDER_LENGTH: u32 = 0x0101_7a3c;
const DOUBLE_THOUSAND: u32 = 0x0101_7b70;
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
const AIM_DISTANCE: u32 = 0x0102_31a0;
const HALF: u32 = 0x0101_6248;
/// The controls manager (the word at `011dea0c` is its pointer), its
/// `Rumble` (`thiscall`: two `float`s, a duration, four zeros) and
/// `IsInMenuMode`.
const CONTROLS_MANAGER_POINTER: u32 = 0x011d_ea0c;
const CONTROLS_RUMBLE: u32 = 0x00a2_55b0;
const IS_IN_MENU_MODE: u32 = 0x0070_2360;
const CONTROLS_MANAGER_GET: u32 = 0x0087_7720;
/// `thiscall` on the weapon: the rumble strength, and the two rumble
/// motors' values (`008133b0`, `00821680`, `00567470`, results in `ST0`).
const WEAPON_RUMBLE_STRENGTH: u32 = 0x0081_33b0;
const WEAPON_RUMBLE_FLOAT_A: u32 = 0x0082_1680;
const WEAPON_RUMBLE_FLOAT_B: u32 = 0x0056_7470;
/// `_ftol2_sse` (`00ec62c0`): the value in `ST0` truncated to an integer;
/// the uniform form takes it as a leading `f64`.
const FTOL: u32 = 0x00ec_62c0;
/// `BGSAmmoForm::IsRockItLauncher` (Xbox PDB), `thiscall` (ammo form part).
const AMMO_IS_ROCKIT_LAUNCHER: u32 = 0x0047_4a80;
/// Virtual slots of the actor's process: `0x440` (shots per trigger pull,
/// `AL`), `0x448` (one word), `0x42c` (recoil, `ST0`), `0x274` (the target
/// handle), `0x27c` (the package location).
const PROCESS_SHOT_COUNT_SLOT: u32 = 0x440;
const PROCESS_SET_SHOT_COUNT_SLOT: u32 = 0x448;
const PROCESS_RECOIL_SLOT: u32 = 0x42c;
const PROCESS_TARGET_HANDLE_SLOT: u32 = 0x274;
const PROCESS_PACKAGE_LOCATION_SLOT: u32 = 0x27c;
/// Virtual slots of the actor: `0x428` (its combat target), `0x3f0` (called
/// with the number of shots fired), `0x360` (`AL`), `0x3c8` (called with
/// the held item, a `float` and `0`), `0x1e4` (its current animation).
const ACTOR_COMBAT_TARGET_SLOT: u32 = 0x428;
const ACTOR_SHOTS_FIRED_SLOT: u32 = 0x3f0;
const ACTOR_IS_READY_TEST_SLOT: u32 = 0x360;
const ACTOR_SET_HELD_SLOT: u32 = 0x3c8;
const ACTOR_CURRENT_ANIMATION_SLOT: u32 = 0x1e4;
/// `Animation::BlendOut` (Xbox PDB), `thiscall` (animation, `4`, `0`).
const ANIMATION_BLEND_OUT: u32 = 0x0049_94f0;
/// `thiscall` on the weapon (`004c0bf0`): whether the shot should blend
/// out the animation.
const WEAPON_SHOULD_BLEND_OUT: u32 = 0x004c_0bf0;
/// `thiscall` on the combat target (`009807f0`: out point, firing
/// position): whether the target has a firing point.
const TARGET_GET_FIRING_POINT: u32 = 0x0098_07f0;
/// `GetUnclampedZAngleFromVector` (Xbox PDB), `cdecl` (vector), `ST0`.
const Z_ANGLE_FROM_VECTOR: u32 = 0x004b_1550;
/// `NiASin` (Xbox PDB), `cdecl` (value), result in `ST0`.
const NI_ASIN: u32 = 0x004b_54c0;
/// `TESPackage::GetLocationReference` (Xbox PDB), `thiscall` (location,
/// actor) and the location's type (`0041ca90`).
const PACKAGE_GET_LOCATION_REFERENCE: u32 = 0x0067_6140;
const LOCATION_GET_TYPE: u32 = 0x0041_ca90;
/// `CombatUtilities::GetTargetFiringLocation` and
/// `CombatUtilities::GetProjectedPointForTarget` (Xbox PDB), `cdecl`.
const GET_TARGET_FIRING_LOCATION: u32 = 0x009a_8600;
const GET_PROJECTED_POINT_FOR_TARGET: u32 = 0x009a_8bf0;
/// `thiscall` on a projectile: its speed (`ST0`).
const PROJECTILE_GET_SPEED: u32 = 0x0069_ef80;
/// `thiscall` on an actor (one word): a stat (`ST0`).
const ACTOR_GET_STAT: u32 = 0x008b_0dd0;
/// `thiscall` on an ammo form (`00923000`) and the spread function
/// (`0059a030`, `cdecl`: `3`, that word, the spread, result in `ST0`).
const AMMO_GET_SPREAD_WORD: u32 = 0x0092_3000;
const SPREAD_FOR_AMMO: u32 = 0x0059_a030;
/// The projectile creation (`009bca60`, `cdecl`, 16 words).
const CREATE_PROJECTILE: u32 = 0x009b_ca60;
/// Two `cdecl` functions of one `float` (`004e44b0`, `004e4470`): the
/// sine and cosine of the random angle.
const RANDOM_CONE_X: u32 = 0x004e_44b0;
const RANDOM_CONE_Z: u32 = 0x004e_4470;
/// `thiscall` on an actor: its current package; on the package: its
/// use-weapon data (`006758f0`); on the data: a flag (`00424940`).
const MOBILE_GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
const PACKAGE_GET_USE_WEAPON_DATA: u32 = 0x0067_58f0;
const USE_WEAPON_DATA_FLAG: u32 = 0x0042_4940;
/// `ActionPoints::IsAttackAction` (Xbox PDB), `cdecl` (action).
const IS_ATTACK_ACTION: u32 = 0x0066_dde0;
/// `thiscall` on a projectile (one word) (`008126e0`).
const PROJECTILE_SET_FLAG: u32 = 0x0081_26e0;
/// `TESObjectREFR::RemoveWeapon` (Xbox PDB), `thiscall` on the actor.
const ACTOR_REMOVE_WEAPON: u32 = 0x0057_1b50;
/// `cdecl` (weapon, shooter, node, is player): muzzle effects (`0083ac30`).
const FIRE_EFFECTS: u32 = 0x0083_ac30;
/// `cdecl` (actor): a `float` of the actor in `ST0` (`00646260`).
const ACTOR_GET_FIRE_FLOAT: u32 = 0x0064_6260;
/// A node's parent: `+0x18` (`thiscall`).
const NODE_PARENT: u32 = 0x0096_11e0;
/// `NiPointer` operations: construct from a pointer (`thiscall`, one word),
/// get the pointer, assign (`thiscall`, one word) and destruct.
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_GET: u32 = 0x0055_9450;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// Helpers of the debug lines: `NiColorA` constructor (`thiscall`: r, g, b,
/// a), `BSTestObjects::CreateCylinder` and `CreateBox_ov2` (Xbox PDB,
/// `cdecl`), the `NiObjectNET` name setter, the local rotation setter,
/// `AddNoLightingPropertyRecurse`, `BSShaderManager::PrepareObject`,
/// `NiAVObject::UpdateProperties`, the `NiUpdateData` constructor and the
/// node update.
const NI_COLOR_CONSTRUCT: u32 = 0x0041_4430;
const CREATE_CYLINDER: u32 = 0x00c5_84a0;
const CREATE_BOX: u32 = 0x00c5_5d50;
const OBJECT_SET_NAME: u32 = 0x00a5_b950;
const SET_LOCAL_ROTATION: u32 = 0x0043_fa80;
const ADD_NO_LIGHTING: u32 = 0x004b_6e90;
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// Virtual slots of a scene node for the debug lines: find a child by name
/// (`0x9c`), attach (`0xdc`, child and a flag) and detach (`0xe8`).
const NODE_FIND_CHILD_SLOT: u32 = 0x9C;
const NODE_ATTACH_CHILD_SLOT: u32 = 0xDC;
const NODE_DETACH_CHILD_SLOT: u32 = 0xE8;
/// `NiMatrix3::ToEulerAnglesZXY` (Xbox PDB), `thiscall` on the rotation
/// (pointers to the Z, X and Y angles).
const MATRIX_TO_EULER_ANGLES: u32 = 0x00a5_9400;

/// Where the firing function keeps the locals whose address it passes: the
/// muzzle position and aim angles, temporaries of the vector helpers, the
/// target point and aim deltas, the shot's `NiPointer`, the debug lines'
/// pointers, colour, matrix and update data.
const FIRE_MUZZLE: u32 = 0x00;
const FIRE_ANGLE_Z: u32 = 0x10;
const FIRE_ANGLE_X: u32 = 0x14;
const FIRE_ANGLE_Y: u32 = 0x18;
const FIRE_OFFSET_TEMP: u32 = 0x20;
const FIRE_SUM_TEMP: u32 = 0x30;
const FIRE_SIGHT_POINT: u32 = 0x40;
const FIRE_AMMO_CHANCE: u32 = 0x50;
const FIRE_TARGET_POINT: u32 = 0x60;
const FIRE_DELTA: u32 = 0x70;
const FIRE_PROJECTED_TEMP: u32 = 0x80;
const FIRE_AIM_DELTA: u32 = 0xA0;
const FIRE_AIM_SUM_TEMP: u32 = 0xB0;
const FIRE_SHOT_POINTER: u32 = 0xC0;
const FIRE_DEBUG_POINTER: u32 = 0xC4;
const FIRE_COLOR: u32 = 0xD0;
const FIRE_MATRIX: u32 = 0xE0;
const FIRE_UPDATE_DATA: u32 = 0x110;
const FIRE_MOD_VALUE: u32 = 0x120;
const FIRE_FRAME_SIZE: u32 = 0x130;

/// The state of one call of the firing function.
struct Fire {
    this: Ptr<TesObjectWeap>,
    reference: Ptr,
    /// The reference when it is an actor, else null.
    actor: Ptr,
    player: u32,
    is_player: bool,
    /// The actor's process (`+0x68`), or 0.
    process: u32,
    /// The process's weapon item, or 0.
    held: u32,
    /// The scene node the shot leaves from, or 0.
    node: u32,
    /// The frame holding the address-taken locals.
    frame: u32,
}

/// Stores three words at `vector`.
fn store_vector(e: &mut Engine, vector: u32, words: [u32; 3]) {
    for (index, word) in words.into_iter().enumerate() {
        e.mem.set_u32(vector + 4 * index as u32, word);
    }
}

/// The value of a boolean setting object (`00408d60`).
fn setting_flag(e: &mut Engine, setting: u32) -> bool {
    let pointer = e.call(SETTING_VALUE_POINTER, &args![setting]).u32();
    e.mem.u8(pointer) != 0
}

/// `NiPointer::Get` of the pointer object at `pointer`.
fn ni_pointer_get(e: &mut Engine, pointer: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![pointer]).u32()
}

// Translated from 00523150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The weapon's fire function (`thiscall`, `reference` is the shooter).
/// For the player it first applies the ammo's chance-based effect. It then
/// finds where the shot leaves from (the held weapon's node for hand-held
/// weapons of types 10 to 13, else the weapon's fire node; the iron sights
/// move the player's shot to the camera), the aim angles (from the
/// reference, or towards the combat target or package location for other
/// actors), the spread (from the weapon, the actor's stat and the ammo),
/// rumbles the controller for the player, creates the projectiles
/// (`009bca60`) with the VATS bookkeeping, hands the number of shots to the
/// actor, drops a thrown weapon that has no ammo left and expels a shell
/// casing.
///
/// Not translated: the C++ exception frame of the compiled function
/// (cleanup of its `NiPointer` locals on unwinding).
// Translation incomplete: 00523150 - the C++ exception unwinding frame (state-based cleanup of the NiPointer locals) is not translated.
pub fn fn_00523150(e: &mut Engine, this: Ptr<TesObjectWeap>, reference: Ptr) {
    if reference.is_null() {
        return;
    }
    e.with_stack(FIRE_FRAME_SIZE, |e, frame| {
        fire(e, this, reference, frame.addr());
    });
}

/// The ammo effect step: the chance goes through the ammo-effect entry
/// point, a random number from 0 to 99 is compared with it, and the effect
/// is applied to the player when it can be.
fn apply_ammo_effect(e: &mut Engine, this: Ptr<TesObjectWeap>, ammo: u32, player: u32, frame: u32) {
    let effect = e.call(AMMO_GET_EFFECT, &args![ammo]).u32();
    let chance = e.call(AMMO_GET_EFFECT_CHANCE, &args![ammo]).f32();
    e.mem.set_f32(frame + FIRE_AMMO_CHANCE, chance);
    e.call(
        HANDLE_ENTRY_POINT,
        &args![
            ENTRY_POINT_AMMO_EFFECT_CHANCE,
            player,
            this,
            frame + FIRE_AMMO_CHANCE
        ],
    );
    let roll = (e.call(RANDOM_INTEGER, &[]).u32() % 100) as f32;
    if effect == 0 {
        return;
    }
    let chance = e.mem.f32(frame + FIRE_AMMO_CHANCE);
    if (chance as f64) < roll as f64 || chance.is_nan() {
        return;
    }
    if e.vcall(effect, EFFECT_CAN_APPLY_SLOT, &[]).bool() {
        e.call(
            RT_DYNAMIC_CAST,
            &args![effect, 0u32, TYPE_TES_FORM, TYPE_TES_FULL_NAME, 0u32],
        );
        e.vcall(player, PLAYER_APPLY_FORM_SLOT, &args![effect, 0u32, 1u32]);
    }
}

/// The body of the firing function.
fn fire(e: &mut Engine, this: Ptr<TesObjectWeap>, reference: Ptr, frame: u32) {
    let player: u32 = e.global(PLAYER_CHARACTER);
    let is_actor = e
        .vcall(reference.addr(), REFERENCE_IS_ACTOR_SLOT, &[])
        .bool();
    let actor = if is_actor { reference } else { Ptr::NULL };
    if actor.addr() == player {
        let ammo = tes_object_weap_get_current_ammo(e, this, Ptr::new(player));
        if ammo != 0 {
            apply_ammo_effect(e, this, ammo, player, frame);
        }
    }
    if !actor.is_null() && e.mem.u32(actor.addr() + ACTOR_BLOCKS_FIRING_OFFSET) != 0 {
        return;
    }
    let process = if actor.is_null() {
        0
    } else {
        e.call(ACTOR_PROCESS, &args![actor]).u32()
    };
    let held = if process != 0 {
        e.vcall(process, PROCESS_WEAPON_ITEM_SLOT, &[]).u32()
    } else {
        0
    };
    let mut shot = Fire {
        this,
        reference,
        actor,
        player,
        is_player: actor.addr() == player,
        process,
        held,
        node: 0,
        frame,
    };
    fire_locate(e, &mut shot);
    let mut spread = fire_base_spread(e, &shot);
    let mut target = 0u32;
    if !shot.is_player && !shot.actor.is_null() {
        match fire_aim_at_target(e, &mut shot, &mut spread) {
            Some(found) => target = found,
            None => return,
        }
    }
    fire_rumble(e, &shot);
    fire_shots(e, &mut shot, spread, target);
}

/// Finds the shot's origin node, muzzle position and aim angles.
fn fire_locate(e: &mut Engine, shot: &mut Fire) {
    let frame = shot.frame;
    let origin = if shot.actor.is_null() {
        e.call(REFERENCE_POSITION, &args![shot.reference]).u32()
    } else {
        let scale: f32 = e.global(FIRING_OFFSET_SCALE);
        let offset = e
            .call(
                VECTOR_SCALED,
                &args![FIRING_OFFSET_VECTOR, frame + FIRE_OFFSET_TEMP, scale],
            )
            .u32();
        let position = e.call(REFERENCE_POSITION, &args![shot.reference]).u32();
        e.call(VECTOR_ADD, &args![position, frame + FIRE_SUM_TEMP, offset])
            .u32()
    };
    let words = vector_words(e, origin);
    store_vector(e, frame + FIRE_MUZZLE, words);
    let angle_x = if shot.actor.is_null() {
        let angles = e
            .call(REFERENCE_ROTATION_ANGLES, &args![shot.reference])
            .u32();
        e.mem.f32(angles)
    } else {
        e.call(MOBILE_GET_LOOKING, &args![shot.actor]).f32()
    };
    e.mem.set_f32(frame + FIRE_ANGLE_X, angle_x);
    let angle_z = if shot.actor.is_null() {
        let angles = e
            .call(REFERENCE_ROTATION_ANGLES, &args![shot.reference])
            .u32();
        e.mem.f32(angles + 8)
    } else {
        e.vcall(shot.actor.addr(), ACTOR_GET_ANGLE_SLOT, &args![0u32])
            .f32()
    };
    e.mem.set_f32(frame + FIRE_ANGLE_Z, angle_z);

    let weapon_type = e.call(GET_WEAPON_TYPE, &args![shot.this]).u32();
    if shot.process != 0 && (10..=13).contains(&weapon_type) {
        fire_locate_held_weapon(e, shot);
        return;
    }
    shot.node = if shot.process != 0 {
        e.vcall(shot.reference.addr(), REFERENCE_NODE_SLOT, &[])
            .u32()
    } else {
        let root = if shot.is_player {
            e.call(PLAYER_GET_CURRENT_3D, &args![shot.player]).u32()
        } else {
            e.vcall(shot.reference.addr(), REFERENCE_GET_3D_SLOT, &[])
                .u32()
        };
        tes_object_weap_get_fire_node(e, shot.this, Ptr::new(root)).addr()
    };
    if shot.node != 0 {
        fire_from_node(e, shot);
    }
}

/// A hand-held weapon (types 10 to 13): the node comes from the actor's
/// process.
fn fire_locate_held_weapon(e: &mut Engine, shot: &mut Fire) {
    let frame = shot.frame;
    if shot.is_player {
        let ammo_item = e.vcall(shot.process, PROCESS_AMMO_ITEM_SLOT, &[]).u32();
        if ammo_item != 0 {
            let item = e.vcall(shot.process, PROCESS_AMMO_ITEM_SLOT, &[]).u32();
            if e.call(HANDLE_RESOLVE, &args![item]).i32() > 1 {
                let flag = e.call(PLAYER_BIPED_FLAG, &args![shot.player]).u8();
                let biped = e.call(PLAYER_GET_BIPED, &args![shot.player, flag]).u32();
                shot.node = e
                    .vcall(shot.process, PROCESS_GET_WEAPON_NODE_SLOT, &args![biped])
                    .u32();
                if shot.node != 0 && e.vcall(shot.node, NODE_AS_NODE_SLOT, &[]).u32() != 0 {
                    let as_node = e.vcall(shot.node, NODE_AS_NODE_SLOT, &[]).u32();
                    if e.call(NODE_STEP_A, &args![as_node, 0u32]).u32() != 0 {
                        let as_node = e.vcall(shot.node, NODE_AS_NODE_SLOT, &[]).u32();
                        let next = e.call(NODE_STEP_B, &args![as_node, 0u32]).u32();
                        e.call(NODE_STEP_C, &args![next, 1u32]);
                    }
                }
            }
        }
    }
    let biped = e.vcall(shot.actor.addr(), ACTOR_GET_BIPED_SLOT, &[]).u32();
    shot.node = e
        .vcall(shot.process, PROCESS_GET_WEAPON_NODE_SLOT, &args![biped])
        .u32();
    if shot.node != 0 {
        let translation = e.call(NODE_WORLD_TRANSLATION, &args![shot.node]).u32();
        let words = vector_words(e, translation);
        store_vector(e, frame + FIRE_MUZZLE, words);
    }
    if shot.is_player {
        fn_00524d70(e, Ptr::new(shot.player));
    }
}

/// With the fire node found: the muzzle position (moved to the camera when
/// the player aims down the sights), the aim angles from the node's
/// rotation, the debug fire-node cylinder, and the muzzle effects.
fn fire_from_node(e: &mut Engine, shot: &mut Fire) {
    let frame = shot.frame;
    let translation = e.call(NODE_WORLD_TRANSLATION, &args![shot.node]).u32();
    let words = vector_words(e, translation);
    store_vector(e, frame + FIRE_MUZZLE, words);
    let mode = e.call(VATS_GET_MODE, &args![VATS_OBJECT]).i32();
    if mode != 4
        && shot.actor.addr() == shot.player
        && e.call(ACTOR_GET_IRON_SIGHTS, &args![shot.player]).bool()
    {
        let sight_model = e.call(WEAPON_SIGHT_MODEL, &args![shot.this]).u32();
        if e.call(MODEL_HAS_FILE, &args![sight_model]).u32() != 0 {
            let mut aiming = false;
            if !e.call(WEAPON_FLAG_TEST_4AD030, &args![shot.this]).bool() {
                aiming = true;
            } else {
                let process = e.call(ACTOR_PROCESS, &args![shot.player]).u32();
                if process != 0 && e.vcall(process, PROCESS_WEAPON_ITEM_SLOT, &[]).u32() != 0 {
                    let item = e.vcall(process, PROCESS_WEAPON_ITEM_SLOT, &[]).u32();
                    if e.call(ITEM_HAS_MOD_EFFECT, &args![item, 0xEu32]).bool() {
                        aiming = true;
                    }
                }
            }
            let camera = fn_00524c90(e);
            if aiming && camera != 0 {
                if e.global::<u32>(SIGHT_OFFSET_GUARD) & 1 == 0 {
                    let guard: u32 = e.global(SIGHT_OFFSET_GUARD);
                    e.set_global(SIGHT_OFFSET_GUARD, guard | 1);
                    let y: f32 = e.global(SIGHT_OFFSET_Y);
                    e.call(
                        VECTOR_CONSTRUCT,
                        &args![SIGHT_OFFSET_VECTOR, 0.0f32, y, 0.0f32],
                    );
                }
                let transform = e.call(NODE_WORLD_ROTATION, &args![camera]).u32();
                let moved = fn_00524c40(
                    e,
                    Ptr::new(transform),
                    Ptr::new(frame + FIRE_SIGHT_POINT),
                    SIGHT_OFFSET_VECTOR,
                );
                let words = vector_words(e, moved.addr());
                store_vector(e, frame + FIRE_MUZZLE, words);
            }
        }
    }
    let reference = shot.reference.addr();
    if reference != 0 && reference == shot.player && fn_00524d10(e, Ptr::new(shot.player)) == 0 {
        let angle = e
            .vcall(shot.player, ACTOR_GET_ANGLE_SLOT, &args![0u32])
            .f32();
        e.mem.set_f32(frame + FIRE_ANGLE_Z, angle);
        let looking = e.call(MOBILE_GET_LOOKING, &args![shot.player]).f32();
        e.mem.set_f32(frame + FIRE_ANGLE_X, looking);
    } else {
        let rotation = e.call(NODE_WORLD_ROTATION, &args![shot.node]).u32();
        e.call(
            MATRIX_TO_EULER_ANGLES,
            &args![
                rotation,
                frame + FIRE_ANGLE_Z,
                frame + FIRE_ANGLE_X,
                frame + FIRE_ANGLE_Y
            ],
        );
    }
    if setting_flag(e, SETTING_LASER_SIGHTS) {
        fire_debug_cylinder(e, shot);
    }
    e.call(
        FIRE_EFFECTS,
        &args![shot.this, shot.reference, shot.node, shot.is_player as u32],
    );
}

/// The debug line along the fire node: finds or creates a red cylinder
/// named by `011c621c` under the node's parent and attaches it.
fn fire_debug_cylinder(e: &mut Engine, shot: &Fire) {
    let frame = shot.frame;
    let pointer = frame + FIRE_DEBUG_POINTER;
    let mut parent = e.vcall(shot.node, NODE_AS_NODE_SLOT, &[]).u32();
    if parent == 0 {
        let node_parent = e.call(NODE_PARENT, &args![shot.node]).u32();
        parent = e.vcall(node_parent, NODE_AS_NODE_SLOT, &[]).u32();
    }
    if parent == 0 {
        return;
    }
    let name = fn_00524cb0(e);
    let existing = e.vcall(parent, NODE_FIND_CHILD_SLOT, &args![name]).u32();
    e.call(NI_POINTER_CONSTRUCT, &args![pointer, existing]);
    if ni_pointer_get(e, pointer) == 0 {
        e.call(
            NI_COLOR_CONSTRUCT,
            &args![frame + FIRE_COLOR, 1.0f32, 0.0f32, 0.0f32, 1.0f32],
        );
        let length: f32 = e.global(CYLINDER_LENGTH);
        let cylinder = e
            .call(
                CREATE_CYLINDER,
                &args![1.0f32, 1.0f32, length, 10u32, 10u32, frame + FIRE_COLOR],
            )
            .u32();
        e.call(NI_POINTER_ASSIGN, &args![pointer, cylinder]);
        let name = fn_00524cb0(e);
        let object = ni_pointer_get(e, pointer);
        e.call(OBJECT_SET_NAME, &args![object, name]);
        let angle: f32 = e.global(HALF_PI);
        nimatrix3_make_x_rotation(e, Ptr::new(frame + FIRE_MATRIX), angle);
        let object = ni_pointer_get(e, pointer);
        e.call(SET_LOCAL_ROTATION, &args![object, frame + FIRE_MATRIX]);
        let object = ni_pointer_get(e, pointer);
        e.call(ADD_NO_LIGHTING, &args![object]);
        let object = ni_pointer_get(e, pointer);
        e.call(PREPARE_OBJECT, &args![object, 0u32, 0u32]);
        let object = ni_pointer_get(e, pointer);
        e.vcall(parent, NODE_ATTACH_CHILD_SLOT, &args![object, 1u32]);
        let object = ni_pointer_get(e, pointer);
        e.call(UPDATE_PROPERTIES, &args![object]);
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![frame + FIRE_UPDATE_DATA, 0.0f32, 0u32, 0u32],
        );
        let object = ni_pointer_get(e, pointer);
        e.call(NODE_UPDATE, &args![object, frame + FIRE_UPDATE_DATA]);
    }
    e.call(NI_POINTER_DESTRUCT, &args![pointer]);
}

/// The spread from the weapon: the actor's stat (a setting without an
/// actor) and the weapon's two spread values (reduced by mod effect 3 when
/// the weapon item has it), converted to radians, then adjusted by the
/// ammo.
fn fire_base_spread(e: &mut Engine, shot: &Fire) -> f32 {
    let stat = if shot.actor.is_null() {
        let pointer = e
            .call(
                SETTING_GET_FLOAT_POINTER,
                &args![SETTING_GUN_REFERENCE_SKILL],
            )
            .u32();
        e.mem.f32(pointer)
    } else {
        e.call(ACTOR_GET_STAT, &args![shot.actor, 1u32]).f32()
    };
    let mut use_mod = false;
    if !shot.actor.is_null()
        && e.call(ACTOR_PROCESS, &args![shot.actor]).u32() != 0
        && shot.held != 0
        && e.call(ITEM_HAS_MOD_EFFECT, &args![shot.held, 3u32]).bool()
    {
        use_mod = true;
    }
    let minimum = fn_00524b80(e, shot.this, use_mod);
    let range = fn_00524be0(e, shot.this, use_mod);
    let radians: f64 = e.global(DEGREES_TO_RADIANS);
    let mut spread = ((range as f64 * stat as f64 + minimum as f64) * radians) as f32;
    if !shot.actor.is_null() && e.call(ACTOR_PROCESS, &args![shot.actor]).u32() != 0 {
        let ammo = tes_object_weap_get_current_ammo(e, shot.this, shot.actor);
        let word = if ammo != 0 {
            let ammo = tes_object_weap_get_current_ammo(e, shot.this, shot.actor);
            e.call(AMMO_GET_SPREAD_WORD, &args![ammo]).u32()
        } else {
            0
        };
        spread = e.call(SPREAD_FOR_AMMO, &args![3u32, word, spread]).f32();
    }
    spread
}

/// For an actor other than the player: aims at the combat target (or the
/// package location when it has none) and adds the actor's stat and recoil
/// to the spread. Returns the combat target, or `None` when the function
/// ends here (a target without a firing point).
fn fire_aim_at_target(e: &mut Engine, shot: &mut Fire, spread: &mut f32) -> Option<u32> {
    let frame = shot.frame;
    let target = e
        .vcall(shot.actor.addr(), ACTOR_COMBAT_TARGET_SLOT, &[])
        .u32();
    if target != 0 {
        let has_point = e
            .call(
                TARGET_GET_FIRING_POINT,
                &args![target, frame + FIRE_TARGET_POINT, frame + FIRE_MUZZLE],
            )
            .bool();
        if !has_point {
            if e.call(WEAPON_SHOULD_BLEND_OUT, &args![shot.this]).bool() {
                let animation = e
                    .vcall(shot.actor.addr(), ACTOR_CURRENT_ANIMATION_SLOT, &[])
                    .u32();
                e.call(ANIMATION_BLEND_OUT, &args![animation, 4u32, 0u32]);
            }
            return None;
        }
        e.call(
            VECTOR_SUBTRACT,
            &args![
                frame + FIRE_TARGET_POINT,
                frame + FIRE_DELTA,
                frame + FIRE_MUZZLE
            ],
        );
        fire_angles_from_delta(e, frame, FIRE_DELTA);
        if setting_flag(e, SETTING_AIM_SIGHTS) {
            fire_debug_box(e, shot);
        }
    } else if shot.process != 0 {
        fire_aim_at_package(e, shot)?;
    }
    // The actor's second stat widens the spread; the process adds recoil.
    let stat = e.call(ACTOR_GET_STAT, &args![shot.actor, 2u32]).f32();
    let setting = e
        .call(
            SETTING_GET_FLOAT_POINTER,
            &args![SETTING_NPC_MAX_GUN_WOBBLE_ANGLE],
        )
        .u32();
    let setting = e.mem.f32(setting);
    let radians: f64 = e.global(DEGREES_TO_RADIANS);
    *spread = ((stat as f64 * setting as f64) * radians + *spread as f64) as f32;
    let process = e.call(ACTOR_PROCESS, &args![shot.actor]).u32();
    let recoil = e.vcall(process, PROCESS_RECOIL_SLOT, &[]).f32();
    *spread = (*spread as f64 + recoil as f64) as f32;
    Some(target)
}

/// Aim angles from the vector at `frame + vector`: the Z angle, and minus
/// the arc sine of its `z` over its length for the X angle.
fn fire_angles_from_delta(e: &mut Engine, frame: u32, vector: u32) {
    let angle_z = e.call(Z_ANGLE_FROM_VECTOR, &args![frame + vector]).f32();
    e.mem.set_f32(frame + FIRE_ANGLE_Z, angle_z);
    let length = e.call(VECTOR_LENGTH, &args![frame + vector]).f64();
    let z = e.mem.f32(frame + vector + 8);
    let ratio = (z as f64 / length) as f32;
    let arc = e.call(NI_ASIN, &args![ratio]).f64();
    e.mem.set_f32(frame + FIRE_ANGLE_X, (-arc) as f32);
}

/// The debug box from the muzzle to the target point (a green box with the
/// name of `011c6220`, replacing the previous one) under the actor's
/// holder.
fn fire_debug_box(e: &mut Engine, shot: &Fire) {
    let frame = shot.frame;
    let pointer = frame + FIRE_DEBUG_POINTER;
    let word = e.call(ACTOR_WORD_40, &args![shot.actor]).u32();
    let holder = fn_00524cf0(e, Ptr::new(word));
    if holder == 0 {
        return;
    }
    let name = fn_00524cc0(e);
    let existing = e.vcall(holder, NODE_FIND_CHILD_SLOT, &args![name]).u32();
    e.call(NI_POINTER_CONSTRUCT, &args![pointer, existing]);
    if ni_pointer_get(e, pointer) != 0 {
        let object = ni_pointer_get(e, pointer);
        let parent = e.call(NODE_PARENT, &args![object]).u32();
        let object = ni_pointer_get(e, pointer);
        e.vcall(parent, NODE_DETACH_CHILD_SLOT, &args![object]);
    }
    e.call(
        NI_COLOR_CONSTRUCT,
        &args![frame + FIRE_COLOR, 0.0f32, 1.0f32, 0.0f32, 1.0f32],
    );
    let half: f32 = e.global(HALF);
    let line = e
        .call(
            CREATE_BOX,
            &args![
                frame + FIRE_MUZZLE,
                frame + FIRE_TARGET_POINT,
                half,
                half,
                half,
                half,
                frame + FIRE_COLOR,
                0u32
            ],
        )
        .u32();
    e.call(NI_POINTER_ASSIGN, &args![pointer, line]);
    let name = fn_00524cc0(e);
    let object = ni_pointer_get(e, pointer);
    e.call(OBJECT_SET_NAME, &args![object, name]);
    let object = ni_pointer_get(e, pointer);
    e.vcall(holder, NODE_ATTACH_CHILD_SLOT, &args![object, 1u32]);
    let object = ni_pointer_get(e, pointer);
    e.call(UPDATE_PROPERTIES, &args![object]);
    e.call(
        UPDATE_DATA_CONSTRUCT,
        &args![frame + FIRE_UPDATE_DATA, 0.0f32, 0u32, 0u32],
    );
    let object = ni_pointer_get(e, pointer);
    e.call(NODE_UPDATE, &args![object, frame + FIRE_UPDATE_DATA]);
    e.call(NI_POINTER_DESTRUCT, &args![pointer]);
}

/// Aiming without a combat target: the actor's package location, when its
/// type is `0x10` and the target handle resolves, gives the point to aim at
/// (the projected point for the weapon's projectile, else the target's
/// firing location); a location that is the package's own reference aims a
/// fixed distance along the actor's heading. Returns `None` when the handle
/// does not resolve (the function ends).
fn fire_aim_at_package(e: &mut Engine, shot: &mut Fire) -> Option<()> {
    let frame = shot.frame;
    let process = shot.process;
    if e.vcall(process, PROCESS_PACKAGE_LOCATION_SLOT, &[]).u32() == 0 {
        return Some(());
    }
    let location = e.vcall(process, PROCESS_PACKAGE_LOCATION_SLOT, &[]).u32();
    if e.call(LOCATION_GET_TYPE, &args![location]).u32() != 0x10 {
        return Some(());
    }
    let handle = e.vcall(process, PROCESS_TARGET_HANDLE_SLOT, &[]).u32();
    if handle == 0 {
        return Some(());
    }
    let resolved = e.call(HANDLE_RESOLVE, &args![handle]).u32();
    if resolved == 0 {
        return None;
    }
    let projectile = e.call(GET_PROJECTILE, &args![shot.this, 0u32]).u32();
    let point = if projectile == 0 {
        e.call(
            GET_TARGET_FIRING_LOCATION,
            &args![frame + FIRE_PROJECTED_TEMP, resolved, 1u32],
        )
        .u32()
    } else {
        let flag = false;
        let scale = if flag {
            let pointer = e
                .call(
                    SETTING_GET_FLOAT_POINTER,
                    &args![SETTING_GRENADE_HIGH_ARC_SPEED_PERCENTAGE],
                )
                .u32();
            e.mem.f32(pointer)
        } else {
            1.0f32
        };
        let speed = e.call(PROJECTILE_GET_SPEED, &args![projectile]).f64();
        let speed = (speed * scale as f64) as f32;
        e.call(
            GET_PROJECTED_POINT_FOR_TARGET,
            &args![
                frame + FIRE_PROJECTED_TEMP,
                shot.actor,
                resolved,
                0u32,
                projectile,
                speed,
                flag as u32
            ],
        )
        .u32()
    };
    let words = vector_words(e, point);
    store_vector(e, frame + FIRE_TARGET_POINT, words);
    e.call(
        VECTOR_SUBTRACT,
        &args![
            frame + FIRE_TARGET_POINT,
            frame + FIRE_AIM_DELTA,
            frame + FIRE_MUZZLE
        ],
    );
    let location = e.vcall(process, PROCESS_PACKAGE_LOCATION_SLOT, &[]).u32();
    let reference = e
        .call(PACKAGE_GET_LOCATION_REFERENCE, &args![location, shot.actor])
        .u32();
    if reference == resolved {
        e.mem.set_f32(frame + FIRE_AIM_DELTA + 8, 0.0);
        let heading = e
            .vcall(shot.actor.addr(), ACTOR_GET_ANGLE_SLOT, &args![0u32])
            .f32();
        e.call(
            SINE_COSINE,
            &args![heading, frame + FIRE_AIM_DELTA, frame + FIRE_AIM_DELTA + 4],
        );
        let distance: f32 = e.global(AIM_DISTANCE);
        e.call(
            VECTOR_SCALE_ASSIGN,
            &args![frame + FIRE_AIM_DELTA, distance],
        );
        let sum = e
            .call(
                VECTOR_ADD,
                &args![
                    frame + FIRE_MUZZLE,
                    frame + FIRE_AIM_SUM_TEMP,
                    frame + FIRE_AIM_DELTA
                ],
            )
            .u32();
        let words = vector_words(e, sum);
        store_vector(e, frame + FIRE_TARGET_POINT, words);
    }
    fire_angles_from_delta(e, frame, FIRE_AIM_DELTA);
    Some(())
}

/// Rumbles the controller for the player's shot (two strengths) when the
/// shooter is the player holding a weapon item or the ammo is a Rock-It
/// launcher's, outside menu mode.
fn fire_rumble(e: &mut Engine, shot: &Fire) {
    let by_player = shot.reference.addr() == shot.player
        && shot.process != 0
        && e.vcall(shot.process, PROCESS_AMMO_ITEM_SLOT, &[]).u32() != 0;
    let launcher = by_player
        || e.call(
            AMMO_IS_ROCKIT_LAUNCHER,
            &args![shot.this.addr() + AMMO_FORM],
        )
        .bool();
    if !(by_player || launcher) || e.call(IS_IN_MENU_MODE, &[]).bool() {
        return;
    }
    let manager_word: u32 = e.global(CONTROLS_MANAGER_POINTER);
    let controls = e.call(CONTROLS_MANAGER_GET, &args![manager_word]).u32();
    let zero: f64 = e.global(ZERO);
    if e.call(WEAPON_RUMBLE_STRENGTH, &args![shot.this]).f64() > zero {
        let strength = e.call(WEAPON_RUMBLE_STRENGTH, &args![shot.this]).f64();
        let scale: f64 = e.global(DOUBLE_THOUSAND);
        let duration = e.call(FTOL, &args![strength * scale]).u32();
        let first = e.call(WEAPON_RUMBLE_FLOAT_A, &args![shot.this]).f32();
        let second = e.call(WEAPON_RUMBLE_FLOAT_B, &args![shot.this]).f32();
        e.call(
            CONTROLS_RUMBLE,
            &args![controls, second, first, duration, 0u32, 0u32, 0u32, 0u32],
        );
    }
    if e.call(WEAPON_RUMBLE_STRENGTH, &args![shot.this]).f64() > zero {
        let half: f32 = e.global(HALF);
        e.call(
            CONTROLS_RUMBLE,
            &args![controls, half, 1.0f32, 0x14Au32, 0u32, 0u32, 0u32, 0u32],
        );
    }
}

/// The VATS action record the firing code updates: byte `+8` is the number
/// of normal shots left, byte `+9` the number of action-point shots left,
/// the word at `+0` the action and the word at `+0xC` a target handle.
const VATS_ACTION_SHOTS: u32 = 8;
const VATS_ACTION_AP_SHOTS: u32 = 9;
const VATS_ACTION_TARGET: u32 = 0xC;

/// Whether the action word of the VATS action is an attack action.
fn vats_action_is_attack(e: &mut Engine, vats_action: u32) -> bool {
    let action = e.mem.u32(vats_action);
    e.call(IS_ATTACK_ACTION, &args![action]).bool()
}

/// Whether the VATS action has shots left and its action is an attack.
fn vats_attack_pending(e: &mut Engine, vats_action: u32) -> bool {
    if vats_action == 0 {
        return false;
    }
    if e.mem.u8(vats_action + VATS_ACTION_SHOTS) == 0
        && e.mem.u8(vats_action + VATS_ACTION_AP_SHOTS) == 0
    {
        return false;
    }
    vats_action_is_attack(e, vats_action)
}

/// What the firing function keeps between the shots of one pull of the
/// trigger.
struct Shots {
    /// The VATS action being fired, or 0.
    vats_action: u32,
    /// Whether the actor's package uses the weapon with the package data's
    /// flag (`[ebp-0x15]`), and the data's own flag (`[ebp-0x4d]`).
    package_flag: bool,
    use_weapon_flag: bool,
    /// The target handle (`[ebp-0x28]`) handed to the projectile.
    target_handle: u32,
    /// Whether the weapon item has mod effect `0xC`.
    has_spread_mod: bool,
}

/// Creates the projectiles of one shot and the VATS bookkeeping after each;
/// returns the last projectile.
fn fire_one_shot(
    e: &mut Engine,
    shot: &Fire,
    state: &Shots,
    spread: f32,
    target: u32,
    ap_shot: bool,
) -> u32 {
    let frame = shot.frame;
    let mut first_projectile = true;
    let mut projectile = 0u32;
    let mut count =
        tes_object_weap_get_num_projectiles(e, shot.this, state.has_spread_mod, false, shot.actor);
    while count != 0 {
        let mut angle_x = e.mem.f32(frame + FIRE_ANGLE_X);
        let mut angle_z = e.mem.f32(frame + FIRE_ANGLE_Z);
        let mut offset_x = 0.0f32;
        let mut offset_z = 0.0f32;
        if !state.package_flag {
            let radius = e.call(RANDOM_FLOAT, &args![0.0f32, spread]).f32();
            let two_pi: f32 = e.global(TWO_PI);
            let around = e.call(RANDOM_FLOAT, &args![0.0f32, two_pi]).f32();
            offset_x = (e.call(RANDOM_CONE_X, &args![around]).f64() * radius as f64) as f32;
            offset_z = (e.call(RANDOM_CONE_Z, &args![around]).f64() * radius as f64) as f32;
            angle_x = (angle_x as f64 + offset_x as f64) as f32;
            angle_z = (angle_z as f64 + offset_z as f64) as f32;
        }
        let shooter = if e
            .vcall(shot.reference.addr(), REFERENCE_IS_ACTOR_SLOT, &[])
            .bool()
        {
            shot.reference
        } else {
            Ptr::NULL
        };
        let skips_ap = state.use_weapon_flag || ap_shot;
        let pointer = ni_pointer_get(e, frame + FIRE_SHOT_POINTER);
        let muzzle = vector_words(e, frame + FIRE_MUZZLE);
        let kind = e.call(GET_PROJECTILE, &args![shot.this, shooter]).u32();
        projectile = e
            .call(
                CREATE_PROJECTILE,
                &args![
                    kind,
                    shot.reference,
                    target,
                    shot.this,
                    muzzle[0],
                    muzzle[1],
                    muzzle[2],
                    angle_z,
                    angle_x,
                    pointer,
                    state.target_handle,
                    state.package_flag as u32,
                    skips_ap as u32,
                    offset_z,
                    offset_x,
                    0u32
                ],
            )
            .u32();
        let vats_action = state.vats_action;
        if vats_action != 0
            && (e.mem.u8(vats_action + VATS_ACTION_SHOTS) != 0
                || e.mem.u8(vats_action + VATS_ACTION_AP_SHOTS) != 0)
            && vats_action_is_attack(e, vats_action)
            && !ap_shot
            && (!fn_00524b40(e, shot.this)
                || (e.call(VATS_GET_WORLDSPACE, &args![VATS_OBJECT]).u32() == 0
                    && fn_00524b40(e, shot.this)))
        {
            e.call(VATS_RECORD_SHOT, &args![VATS_OBJECT, projectile]);
        }
        if first_projectile {
            first_projectile = false;
        } else {
            e.call(PROJECTILE_SET_FLAG, &args![projectile, 0u32]);
        }
        count = count.wrapping_sub(1);
    }
    projectile
}

/// The shots of one pull of the trigger and what follows them: the
/// projectiles with their VATS bookkeeping, the player's shot counter, the
/// actor's shots-fired call, dropping a hand-held weapon that has run out,
/// the aim of a held ranged weapon, and the shell casing.
fn fire_shots(e: &mut Engine, shot: &mut Fire, mut spread: f32, target: u32) {
    let frame = shot.frame;
    let mut shots: u8 = if shot.process != 0 {
        e.vcall(shot.process, PROCESS_SHOT_COUNT_SLOT, &[]).u8()
    } else {
        1
    };
    let mut state = Shots {
        vats_action: 0,
        package_flag: false,
        use_weapon_flag: false,
        target_handle: 0,
        has_spread_mod: false,
    };
    if shot.held != 0 {
        state.has_spread_mod = e
            .call(ITEM_HAS_MOD_EFFECT, &args![shot.held, 0xCu32])
            .bool();
    }
    if state.has_spread_mod {
        let effect = e
            .call(GET_MOD_EFFECT_VALUE, &args![shot.this, 0xCu32, 1u32])
            .f64();
        spread = (effect * spread as f64) as f32;
    }
    let per_pull = tes_object_weap_get_num_projectiles(e, shot.this, false, false, shot.actor);
    let total = per_pull as i32 * (shots as i8 as i32);
    let mut fired = 0i32;
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![frame + FIRE_SHOT_POINTER, 0u32],
    );
    let vats_actor: u32 = e.global(VATS_ACTOR);
    if e.call(VATS_GET_MODE, &args![VATS_OBJECT]).i32() == 4 && shot.actor.addr() == vats_actor {
        state.vats_action = e.call(VATS_GET_CURRENT_ACTION, &args![VATS_OBJECT]).u32();
    }
    let package = if shot.actor.is_null() {
        0
    } else {
        e.call(MOBILE_GET_CURRENT_PACKAGE, &args![shot.actor]).u32()
    };
    if package != 0 {
        let data = e.call(PACKAGE_GET_USE_WEAPON_DATA, &args![package]).u32();
        if data != 0 {
            state.package_flag = e.call(USE_WEAPON_DATA_FLAG, &args![data]).bool();
            state.use_weapon_flag = fn_00524cd0(e, Ptr::new(data)) != 0;
        }
    }
    let vats_action = state.vats_action;
    let mut attack_ap = false;
    if vats_action != 0 && e.mem.u8(vats_action + VATS_ACTION_AP_SHOTS) != 0 {
        let action = e.mem.u32(vats_action);
        if e.call(IS_ATTACK_ACTION, &args![action]).bool() {
            attack_ap = true;
        }
    }
    if !shot.actor.is_null() && shot.process != 0 {
        e.vcall(
            shot.process,
            PROCESS_SET_SHOT_COUNT_SLOT,
            &args![total as u32],
        );
    }
    let mut vats_actor_shot = false;
    if vats_action != 0 && !shot.actor.is_null() {
        let a: u32 = e.global(VATS_ACTOR_A);
        let b: u32 = e.global(VATS_ACTOR_B);
        if shot.actor.addr() == a || shot.actor.addr() == b {
            vats_actor_shot = true;
            state.package_flag = true;
            state.target_handle = e.mem.u32(vats_action + VATS_ACTION_TARGET);
        }
    }
    if state.package_flag && !vats_actor_shot {
        let handle = e.vcall(shot.process, PROCESS_TARGET_HANDLE_SLOT, &[]).u32();
        if handle != 0 && e.vcall(handle, 8, &[]).u32() == 0x10 {
            state.target_handle = e.call(HANDLE_RESOLVE, &args![handle]).u32();
        }
    }

    while shots != 0 {
        let ap_shot = attack_ap && {
            let accumulator: f32 = e.global(VATS_FIRE_ACCUMULATOR);
            let zero: f64 = e.global(ZERO);
            ((accumulator as f64) < zero && e.mem.u8(vats_action + VATS_ACTION_AP_SHOTS) > 0)
                || e.mem.u8(vats_action + VATS_ACTION_SHOTS) == 0
        };
        let projectile = fire_one_shot(e, shot, &state, spread, target, ap_shot);
        if !ap_shot {
            fired += 1;
        }
        if vats_attack_pending(e, vats_action) {
            fire_vats_cost(e, shot, vats_action, ap_shot, projectile);
        }
        shots = shots.wrapping_sub(1);
    }

    fire_after_shots(e, shot, fired);
}

/// After a VATS shot: takes the shot from the action's counters, adds the
/// fire-rate fraction of the settings to the accumulator (dropping a whole
/// one), and finishes the attack when the last normal shot is gone.
fn fire_vats_cost(e: &mut Engine, shot: &Fire, vats_action: u32, ap_shot: bool, projectile: u32) {
    let ap_shots = e.mem.u8(vats_action + VATS_ACTION_AP_SHOTS);
    if ap_shot && ap_shots != 0 {
        e.mem
            .set_u8(vats_action + VATS_ACTION_AP_SHOTS, ap_shots - 1);
    } else {
        let cost = (fn_00524b60(e, shot.this) as i32).max(1);
        let left = e.mem.u8(vats_action + VATS_ACTION_SHOTS) as i32;
        e.mem
            .set_u8(vats_action + VATS_ACTION_SHOTS, (left - cost) as u8);
    }
    let numerator = e
        .call(
            SETTING_GET_FLOAT_POINTER,
            &args![SETTING_VATS_SHOT_BURST_TIME],
        )
        .u32();
    let rate_b = e
        .call(
            SETTING_GET_FLOAT_POINTER,
            &args![SETTING_VATS_SHOT_LONG_BURST_TIME],
        )
        .u32();
    let rate_b = e.mem.f32(rate_b);
    let rate_a = e
        .call(
            SETTING_GET_FLOAT_POINTER,
            &args![SETTING_VATS_SHOT_BURST_TIME],
        )
        .u32();
    let rate_a = e.mem.f32(rate_a);
    let divisor = e.call(MAX_FLOAT, &args![rate_a, rate_b]).f64();
    let step = (e.mem.f32(numerator) as f64 / divisor) as f32;
    let accumulator: f32 = e.global(VATS_FIRE_ACCUMULATOR);
    let accumulator = (accumulator as f64 + step as f64) as f32;
    e.set_global(VATS_FIRE_ACCUMULATOR, accumulator);
    if step as f64 <= accumulator as f64 {
        let one: f64 = e.global(DOUBLE_ONE);
        e.set_global(VATS_FIRE_ACCUMULATOR, (accumulator as f64 - one) as f32);
    }
    if !ap_shot && e.mem.u8(vats_action + VATS_ACTION_SHOTS) == 0 {
        e.call(VATS_FINISH_ATTACK, &args![VATS_OBJECT, projectile]);
    }
}

/// The end of the firing function, after the shots: the player's shot
/// counter (when the weapon item has mod effect 5), the actor's
/// shots-fired call, the hand-held weapon that has run out of ammo, the
/// aim of a held ranged weapon and the shell casing.
fn fire_after_shots(e: &mut Engine, shot: &mut Fire, fired: i32) {
    let frame = shot.frame;
    if !shot.actor.is_null() {
        let mut adjust = 0i32;
        if shot.actor.addr() == shot.player {
            let process = e.call(ACTOR_PROCESS, &args![shot.player]).u32();
            if process != 0 && e.vcall(process, PROCESS_WEAPON_ITEM_SLOT, &[]).u32() != 0 {
                e.mem.set_f32(frame + FIRE_MOD_VALUE, 0.0);
                let item = e.vcall(process, PROCESS_WEAPON_ITEM_SLOT, &[]).u32();
                if e.call(
                    ITEM_HAS_MOD_EFFECT_VALUE,
                    &args![item, 5u32, frame + FIRE_MOD_VALUE],
                )
                .bool()
                {
                    let counter = fn_00524d50(e, Ptr::new(shot.player)) as i8 as i32;
                    fn_00524d30(e, Ptr::new(shot.player), (counter + fired) as u8);
                    if fn_00524d50(e, Ptr::new(shot.player)) as i8 as i32 >= 4 {
                        let counter = fn_00524d50(e, Ptr::new(shot.player)) as i8 as i32;
                        fn_00524d30(e, Ptr::new(shot.player), (counter - 4) as u8);
                        adjust = 1;
                    }
                }
            }
        }
        e.vcall(
            shot.actor.addr(),
            ACTOR_SHOTS_FIRED_SLOT,
            &args![(fired - adjust) as u32],
        );
    }
    let weapon_type = e.call(GET_WEAPON_TYPE, &args![shot.this]).u32();
    let hand_held = (shot.process != 0 && weapon_type == 10) || (11..=13).contains(&weapon_type);
    if hand_held {
        let mut ammo_left = false;
        if e.vcall(shot.process, PROCESS_AMMO_ITEM_SLOT, &[]).u32() != 0 {
            let item = e.vcall(shot.process, PROCESS_AMMO_ITEM_SLOT, &[]).u32();
            ammo_left = e.call(HANDLE_RESOLVE, &args![item]).i32() > 0;
        }
        if !ammo_left {
            e.call(ACTOR_REMOVE_WEAPON, &args![shot.actor]);
        } else if shot.node != 0 {
            let as_node = e.vcall(shot.node, NODE_AS_NODE_SLOT, &[]).u32();
            let step = e.call(NODE_STEP_A, &args![as_node, 0u32]).u32();
            if step != 0 {
                e.call(NODE_STEP_C, &args![step, 1u32]);
            }
        }
    }
    e.call(NI_POINTER_ASSIGN, &args![frame + FIRE_SHOT_POINTER, 0u32]);
    let weapon_type = e.call(GET_WEAPON_TYPE, &args![shot.this]).i32();
    if (3..=9).contains(&weapon_type)
        && !shot.actor.is_null()
        && e.vcall(shot.actor.addr(), ACTOR_IS_READY_TEST_SLOT, &[])
            .bool()
        && shot.process != 0
        && e.vcall(shot.process, PROCESS_WEAPON_ITEM_SLOT, &[]).u32() != 0
        && !e.call(WEAPON_FLAG_20_TEST, &args![shot.this]).bool()
    {
        let value = e.call(ACTOR_GET_FIRE_FLOAT, &args![shot.actor]).f32();
        let process = e.call(ACTOR_PROCESS, &args![shot.actor]).u32();
        let item = e.vcall(process, PROCESS_WEAPON_ITEM_SLOT, &[]).u32();
        e.vcall(
            shot.actor.addr(),
            ACTOR_SET_HELD_SLOT,
            &args![item, value, 0u32],
        );
    }
    if fn_00524b40(e, shot.this) {
        tes_object_weap_expel_shell_casing(e, shot.this, shot.reference);
    }
    e.call(NI_POINTER_DESTRUCT, &args![frame + FIRE_SHOT_POINTER]);
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051e420, tes_object_weap_init_item(Ptr<TesObjectWeap>)),
        entry!(0x0051f4b0, fn_0051f4b0(Ptr<TesObjectWeap>, Ptr)),
        entry!(0x0051f4f0, fn_0051f4f0(Ptr<TesObjectWeap>, Ptr)),
        entry!(0x0051f590, fn_0051f590(Ptr<TesObjectWeap>, bool)),
        entry!(
            0x0051f630,
            tes_object_weap_load(Ptr<TesObjectWeap>, Ptr) -> bool
        ),
        entry!(0x00520ff0, obj_weap_endian(Ptr<ObjWeap>)),
        entry!(0x00521580, fn_00521580(Ptr<TesObjectWeap>, u8)),
        entry!(0x00521730, tes_object_weap_save(Ptr<TesObjectWeap>)),
        entry!(
            0x00522c80,
            tes_object_weap_get_combat_weapon_type(Ptr<TesObjectWeap>) -> u32
        ),
        entry!(0x00522d80, fn_00522d80(Ptr<TesObjectWeap>) -> Ptr),
        entry!(0x00522df0, fn_00522df0(Ptr<TesObjectWeap>, u8, bool) -> Ptr),
        entry!(0x00523020, fn_00523020(Ptr<TesObjectWeap>, u32) -> u16),
        entry!(0x00523080, fn_00523080(Ptr<TesObjectWeap>, u32)),
        entry!(0x005230b0, fn_005230b0(Ptr<TesObjectWeap>, u32, u32)),
        entry!(0x005230f0, fn_005230f0(Ptr<TesObjectWeap>, u32)),
        entry!(0x00523120, fn_00523120(Ptr<TesObjectWeap>, u32)),
        entry!(0x00523150, fn_00523150(Ptr<TesObjectWeap>, Ptr)),
        entry!(0x00524ac0, nimatrix3_make_x_rotation(Ptr, f32)),
        entry!(0x00524b40, fn_00524b40(Ptr<TesObjectWeap>) -> bool),
        entry!(0x00524b60, fn_00524b60(Ptr<TesObjectWeap>) -> u8),
        entry!(0x00524b80, fn_00524b80(Ptr<TesObjectWeap>, bool) -> f32),
        entry!(0x00524be0, fn_00524be0(Ptr<TesObjectWeap>, bool) -> f32),
        entry!(0x00524c40, fn_00524c40(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00524c90, fn_00524c90() -> u32),
        entry!(0x00524cb0, fn_00524cb0() -> u32),
        entry!(0x00524cc0, fn_00524cc0() -> u32),
        entry!(0x00524cd0, fn_00524cd0(Ptr) -> u8),
        entry!(0x00524cf0, fn_00524cf0(Ptr) -> u32),
        entry!(0x00524d10, fn_00524d10(Ptr) -> u8),
        entry!(0x00524d30, fn_00524d30(Ptr, u8)),
        entry!(0x00524d50, fn_00524d50(Ptr) -> u8),
        entry!(0x00524d70, fn_00524d70(Ptr)),
        entry!(
            0x00524db0,
            tes_object_weap_expel_shell_casing(Ptr<TesObjectWeap>, Ptr)
        ),
        entry!(0x00525340, fn_00525340(Ptr)),
        entry!(0x00525420, fn_00525420() -> u8),
        entry!(0x00525430, fn_00525430(Ptr) -> bool),
        entry!(0x00525450, fn_00525450(Ptr, u32, u32)),
        entry!(
            0x00525510,
            tes_object_weap_queue_files(Ptr<TesObjectWeap>, u32, u32, bool)
        ),
        entry!(
            0x00525620,
            fn_00525620(Ptr<TesObjectWeap>, bool, bool) -> u32
        ),
        entry!(0x005256b0, fn_005256b0(Ptr<TesObjectWeap>) -> u32),
        entry!(
            0x00525700,
            tes_object_weap_get_fire_node(Ptr<TesObjectWeap>, Ptr) -> Ptr
        ),
        entry!(0x00525750, fn_00525750(Ptr, u32, u32, bool, u32, u32) -> u32),
        entry!(0x005257a0, fn_005257a0(Ptr<TesObjectWeap>, Ptr, Ptr) -> Ptr),
        entry!(
            0x00525980,
            tes_object_weap_get_current_ammo(Ptr<TesObjectWeap>, Ptr) -> u32
        ),
        entry!(
            0x00525b20,
            tes_object_weap_get_num_projectiles(Ptr<TesObjectWeap>, bool, bool, Ptr) -> u8
        ),
        entry!(0x00525be0, fn_00525be0(Ptr) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// The arguments of every call a recording double saw.
    type Log = Rc<RefCell<Vec<Vec<u32>>>>;
    /// The model paths (as bytes) a recording `SetModel` double saw, by model.
    type PathLog = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    fn weapon(e: &mut Engine) -> Ptr<TesObjectWeap> {
        e.new_object()
    }

    /// Maps the page of every address.
    fn map_pages(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.map(addr & !0xfff, 0x1000);
        }
    }

    /// Doubles that do nothing and return zero.
    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// A double returning `value` in `EAX`.
    fn returning(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// A double returning `value` in `ST0`.
    fn returning_float(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    /// A double that records the arguments of every call.
    fn recording(e: &mut Engine, addr: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        log
    }

    /// The calls logged so far, in order.
    fn logged(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap_or_default()
    }

    /// The addresses of the logged calls.
    fn addresses(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter().map(|(addr, _)| *addr).collect()
    }

    /// Gives `object` a vtable whose listed slots (byte offsets) hold the
    /// targets.
    fn give_vtable(e: &mut Engine, object: u32, slots: &[(u32, u32)]) {
        let vtable = e.mem.alloc(0x800);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        e.mem.set_u32(object, vtable);
    }

    /// `iFormID` getter double: the id at `+0xC` of the form.
    fn form_id_getter(e: &mut Engine) {
        e.register(GET_FORM_ID, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xC),
            ..Ret::default()
        });
    }

    #[test]
    fn projectile_setter_stores_data_projectile() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        e.call(0x0051_f4b0, &args![w, 0x1234u32]);
        assert_eq!(e.mem.u32(w.addr() + 0x118), 0x1234);
    }

    #[test]
    fn critical_effect_setter_stores_the_effect() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        e.call(0x0051_f4f0, &args![w, 0x5678u32]);
        assert_eq!(e.mem.u32(w.addr() + 0x1CC), 0x5678);
    }

    #[test]
    fn flags_ex_bit_two_is_set_and_cleared() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        e.mem.set_u32(w.addr() + 0x12C, 0x10);
        e.call(0x0051_f590, &args![w, 1u32]);
        assert_eq!(e.mem.u32(w.addr() + 0x12C), 0x12);
        e.call(0x0051_f590, &args![w, 0u32]);
        assert_eq!(e.mem.u32(w.addr() + 0x12C), 0x10);
    }

    #[test]
    fn num_projectiles_setter_stores_a_byte() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        e.call(0x0052_1580, &args![w, 3u32]);
        assert_eq!(e.mem.u8(w.addr() + 0x11E), 3);
    }

    #[test]
    fn endian_swaps_the_forty_four_words_in_order() {
        let mut e = Engine::new();
        quiet(&mut e, &[SWAP_WORD_IN_PLACE]);
        let data = Ptr::<ObjWeap>::new(0x1000);
        e.call_log = Some(vec![]);
        e.call(0x0052_0ff0, &args![data]);
        let log = logged(&mut e);
        assert_eq!(log.len(), 45);
        assert_eq!(log[1], (SWAP_WORD_IN_PLACE, vec![0x1004, 0]));
        assert_eq!(log[2].1[0], 0x1008);
        assert_eq!(log[37].1[0], 0x1000 + 0xA8);
        assert_eq!(log[38].1[0], 0x1000 + 0xC8);
        assert_eq!(log[44].1[0], 0x1000 + 0xC4);
    }

    #[test]
    fn combat_weapon_type_follows_the_weapon_type() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        e.register(GET_WEAPON_TYPE, |e, a| Ret {
            eax: e.mem.i8(a[0] + 0xF4) as i32 as u32,
            ..Ret::default()
        });
        returning(&mut e, GET_PROJECTILE, 0);
        quiet(&mut e, &[LOG_MESSAGE]);
        for (weapon_type, expected) in [
            (0i8, 2),
            (2, 2),
            (3, 1),
            (9, 1),
            (10, 3),
            (11, 4),
            (12, 4),
            (13, 1),
        ] {
            e.mem.set_i8(w.addr() + 0xF4, weapon_type);
            assert_eq!(
                e.call(0x0052_2c80, &args![w]).u32(),
                expected,
                "type {weapon_type}"
            );
        }
    }

    #[test]
    fn combat_weapon_type_is_zero_for_an_explosive_projectile_in_range() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        e.mem.set_i8(w.addr() + 0xF4, 5);
        e.register(GET_WEAPON_TYPE, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        returning(&mut e, GET_PROJECTILE, 0x4000);
        returning(&mut e, PROJECTILE_GET_EXPLOSION, 0x4100);
        returning_float(&mut e, EXPLOSION_GET_DAMAGE, 300.0);
        returning_float(&mut e, EXPLOSION_GET_RADIUS, 50.0);
        // The two settings hold 100.0 and 10.0: both are below the values.
        e.map(0x4000, 0x1000);
        e.mem.set_f32(0x4200, 100.0);
        e.mem.set_f32(0x4204, 10.0);
        e.register(SETTING_GET_FLOAT_POINTER, |_, a| Ret {
            eax: if a[0] == SETTING_DANGEROUS_PROJECTILE_EXPLOSION_DAMAGE {
                0x4200
            } else {
                0x4204
            },
            ..Ret::default()
        });
        assert_eq!(e.call(0x0052_2c80, &args![w]).u32(), 0);
        // A radius at or below the second setting falls back to 1.
        returning_float(&mut e, EXPLOSION_GET_RADIUS, 5.0);
        assert_eq!(e.call(0x0052_2c80, &args![w]).u32(), 1);
    }

    #[test]
    fn combat_weapon_type_logs_an_unknown_type() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        e.register(GET_WEAPON_TYPE, |_, _| Ret {
            eax: 0xFFFF_FFFF,
            ..Ret::default()
        });
        let log = recording(&mut e, LOG_MESSAGE);
        assert_eq!(e.call(0x0052_2c80, &args![w]).u32(), 2);
        assert_eq!(
            log.borrow().as_slice(),
            &[vec![UNKNOWN_WEAPON_TYPE_MESSAGE, 0xFFFF_FFFF]]
        );
    }

    /// A model double whose path is the text at the object's `+0x40`
    /// pointer; the weapon's models are set up by the tests.
    fn path_getter(e: &mut Engine, model_vtable: u32) {
        e.register(model_vtable, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x10),
            ..Ret::default()
        });
    }

    /// Gives the model at `weapon + offset` a path (empty when `text` is
    /// empty) through a vtable with `GetModel` at slot `0x14`.
    fn give_model_path(e: &mut Engine, weapon: u32, offset: u32, text: &[u8]) {
        let model = weapon + offset;
        give_vtable(e, model, &[(MODEL_GET_MODEL_SLOT, 0x00dd_0014)]);
        let buffer = e.mem.alloc(16);
        e.mem.set_cstr(buffer, text);
        e.mem.set_u32(model + 0x10, buffer);
        // The weapon is large enough for the offsets used here.
    }

    #[test]
    fn world_model_is_used_when_it_has_a_path() {
        let mut e = Engine::new();
        let w = e.mem.alloc(0x400);
        path_getter(&mut e, 0x00dd_0014);
        give_model_path(&mut e, w, WORLD_MODEL, b"world.nif");
        assert_eq!(e.call(0x0052_2d80, &args![w]).u32(), w + WORLD_MODEL);
        give_model_path(&mut e, w, WORLD_MODEL, b"");
        assert_eq!(e.call(0x0052_2d80, &args![w]).u32(), w + MODEL);
    }

    #[test]
    fn mod_model_is_chosen_by_the_mod_mask() {
        let mut e = Engine::new();
        let w = e.mem.alloc(0x400);
        path_getter(&mut e, 0x00dd_0014);
        e.register(GET_WEAPON_TYPE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        form_id_getter(&mut e);
        for offset in [
            WORLD_MODEL_MOD_1,
            WORLD_MODEL_MOD_2,
            WORLD_MODEL_MOD_3,
            WORLD_MODEL_MOD_12,
            WORLD_MODEL_MOD_13,
            WORLD_MODEL_MOD_23,
            WORLD_MODEL_MOD_123,
        ] {
            give_model_path(&mut e, w, offset, b"mod.nif");
        }
        for (mask, offset) in [
            (1u8, WORLD_MODEL_MOD_1),
            (2, WORLD_MODEL_MOD_2),
            (4, WORLD_MODEL_MOD_3),
            (3, WORLD_MODEL_MOD_12),
            (5, WORLD_MODEL_MOD_13),
            (6, WORLD_MODEL_MOD_23),
            (7, WORLD_MODEL_MOD_123),
        ] {
            assert_eq!(
                e.call(0x0052_2df0, &args![w, mask as u32, 0u32]).u32(),
                w + offset,
                "mask {mask}"
            );
        }
        // No mask, or a mod model without a path: the main model.
        assert_eq!(e.call(0x0052_2df0, &args![w, 0u32, 0u32]).u32(), w + MODEL);
        give_model_path(&mut e, w, WORLD_MODEL_MOD_1, b"");
        assert_eq!(e.call(0x0052_2df0, &args![w, 1u32, 0u32]).u32(), w + MODEL);
        // The special form id also falls back to the main model.
        e.mem.set_u32(w + 0xC, NO_MOD_MODEL_FORM_ID);
        assert_eq!(e.call(0x0052_2df0, &args![w, 2u32, 0u32]).u32(), w + MODEL);
    }

    #[test]
    fn type_zero_weapon_uses_the_world_model_rule() {
        let mut e = Engine::new();
        let w = e.mem.alloc(0x400);
        path_getter(&mut e, 0x00dd_0014);
        e.register(GET_WEAPON_TYPE, |_, _| Ret::default());
        give_model_path(&mut e, w, WORLD_MODEL, b"world.nif");
        give_model_path(&mut e, w, WORLD_MODEL_MOD_1, b"mod.nif");
        assert_eq!(
            e.call(0x0052_2df0, &args![w, 1u32, 0u32]).u32(),
            w + WORLD_MODEL
        );
        // The third argument forces the mod models.
        assert_eq!(
            e.call(0x0052_2df0, &args![w, 1u32, 1u32]).u32(),
            w + WORLD_MODEL_MOD_1
        );
    }

    /// Doubles for everything `InitItem` calls outside this file.
    fn init_item_doubles(e: &mut Engine) {
        quiet(
            e,
            &[
                BIPED_MODEL_LIST_INIT_ITEM,
                SCRIPTABLE_FORM_INIT_ITEM,
                ENCHANTABLE_FORM_INIT_ITEM,
                AMMO_FORM_INIT_ITEM,
                DESTRUCTIBLE_FORM_INIT_ITEM,
                MODEL_TEXTURE_SWAP_INIT_ITEM,
                REPAIR_ITEM_LIST_INIT_ITEM,
                PICKUP_PUTDOWN_SOUNDS_INIT_ITEM,
                FORM_SET_FLAG_8,
                SET_SCOPE_EFFECT,
                SET_VATS_SPECIAL_EFFECT,
            ],
        );
        returning(e, FORM_FLAG_8_TEST, 0);
        returning(e, GET_PROJECTILE, 0);
        returning(e, GET_SCOPE_EFFECT, 0);
        returning(e, GET_CRITICAL_EFFECT, 0);
        returning(e, GET_VATS_SPECIAL_EFFECT, 0);
        returning(e, FORM_GET_FILE, 0x7000);
        // The id gets the load order 1 in its top byte.
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        // The "form" of an id is the id plus 0x1000; ids ending in 0xFF are
        // unknown forms and do not cast.
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: if a[0] & 0xFF == 0xFF { 0 } else { a[0] },
            ..Ret::default()
        });
        form_id_getter(e);
        quiet(e, &[LOG_MESSAGE]);
        // The setting is off and the zero constant is mapped.
        map_pages(e, &[ZERO, 0x6000]);
        returning(e, SETTING_VALUE_POINTER, 0x6000);
    }

    #[test]
    fn init_item_does_nothing_when_form_flag_eight_is_set() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        returning(&mut e, FORM_FLAG_8_TEST, 1);
        e.call_log = Some(vec![]);
        e.call(0x0051_e420, &args![w]);
        assert_eq!(
            addresses(&logged(&mut e)),
            vec![0x0051_e420, FORM_FLAG_8_TEST]
        );
    }

    #[test]
    fn init_item_resolves_form_ids_and_logs_the_ones_that_do_not() {
        let mut e = Engine::new();
        init_item_doubles(&mut e);
        let w = weapon(&mut e);
        give_vtable(
            &mut e,
            w.addr(),
            &[(FORM_GET_OBJECT_TYPE_NAME_SLOT, 0x00dd_0130)],
        );
        returning(&mut e, 0x00dd_0130, 0xBEEF);
        e.mem.set_u32(w.addr() + 0xC, 0x1234);
        e.mem.set_u32(w.addr() + 0x21C, 5);
        e.mem.set_u32(w.addr() + 0x220, 0xFF);
        e.mem.set_u32(w.addr() + 0x250, 6);
        let log = recording(&mut e, LOG_MESSAGE);
        let adds = recording(&mut e, FORM_SET_FLAG_8);
        e.call(0x0051_e420, &args![w]);
        // 5 -> load order 1 -> form id + 0x1000.
        assert_eq!(e.mem.u32(w.addr() + 0x21C), 0x0100_0005 + 0x1000);
        assert_eq!(e.mem.u32(w.addr() + 0x250), 0x0100_0006 + 0x1000);
        // The unknown one is null and logged with its rewritten id, the
        // weapon's type name and its own id.
        assert_eq!(e.mem.u32(w.addr() + 0x220), 0);
        assert_eq!(
            log.borrow().as_slice(),
            &[vec![MESSAGE_OPEN_SOUND_2, 0x0100_00FF, 0xBEEF, 0x1234]]
        );
        // The form is marked initialized at the end.
        assert_eq!(adds.borrow().as_slice(), &[vec![w.addr(), 1]]);
    }

    #[test]
    fn init_item_resolves_the_projectile_scope_and_spell_effects() {
        let mut e = Engine::new();
        init_item_doubles(&mut e);
        let w = weapon(&mut e);
        form_id_getter(&mut e);
        // The projectile id is read through 00525a90 and stored in the data.
        returning(&mut e, GET_PROJECTILE, 0x30);
        returning(&mut e, GET_SCOPE_EFFECT, 0x31);
        returning(&mut e, GET_CRITICAL_EFFECT, 0x32);
        returning(&mut e, GET_VATS_SPECIAL_EFFECT, 0x33);
        let scope = recording(&mut e, SET_SCOPE_EFFECT);
        let vats = recording(&mut e, SET_VATS_SPECIAL_EFFECT);
        let log = recording(&mut e, LOG_MESSAGE);
        e.call(0x0051_e420, &args![w]);
        assert_eq!(e.mem.u32(w.addr() + 0x118), 0x0100_0030 + 0x1000);
        assert_eq!(e.mem.u32(w.addr() + 0x1CC), 0x0100_0032 + 0x1000);
        assert_eq!(
            scope.borrow().as_slice(),
            &[vec![w.addr(), 0x0100_0031 + 0x1000]]
        );
        assert_eq!(
            vats.borrow().as_slice(),
            &[vec![w.addr(), 0x0100_0033 + 0x1000]]
        );
        // The getters still return ids, so nothing was "missing".
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn init_item_clamps_speed_and_multiplier_and_sets_the_flags() {
        let mut e = Engine::new();
        init_item_doubles(&mut e);
        let w = weapon(&mut e);
        let data = weapon_data(w);
        e.set(data, ObjWeap::fSpeed, -2.0);
        e.set(data, ObjWeap::fAttackMult, 0.0);
        e.set(data, ObjWeap::cReloadAnim, 0x13);
        e.mem.set_u8(0x6000, 1);
        e.call(0x0051_e420, &args![w]);
        assert_eq!(e.get(data, ObjWeap::fSpeed), 1.0);
        assert_eq!(e.get(data, ObjWeap::fAttackMult), 1.0);
        assert_eq!(e.get(data, ObjWeap::iFlagsEx), 2);
        assert_eq!(e.get(w, TesObjectWeap::bIsLoopingReload), 1);
        // Positive values stay, and a short reload clears the flag.
        e.set(data, ObjWeap::fSpeed, 0.5);
        e.set(data, ObjWeap::cReloadAnim, 0x12);
        e.call(0x0051_e420, &args![w]);
        assert_eq!(e.get(data, ObjWeap::fSpeed), 0.5);
        assert_eq!(e.get(w, TesObjectWeap::bIsLoopingReload), 0);
    }

    /// Doubles for everything `Save` calls outside this file, recording the
    /// chunk words and data (with the pointer field the block replaces).
    struct SaveLog {
        words: Log,
        arrays: Log,
        data: Rc<RefCell<Vec<(u32, u32, u32)>>>,
    }

    fn save_doubles(e: &mut Engine) -> SaveLog {
        quiet(
            e,
            &[
                FORM_START_FORM,
                FORM_CLOSE_FORM,
                FORM_SAVE_DATA,
                FULL_NAME_SAVE,
                MODEL_TEXTURE_SWAP_SAVE,
                MODEL_SAVE_FLAGS,
                TEXTURE_SAVE,
                MESSAGE_ICON_SAVE,
                SCRIPTABLE_FORM_SAVE,
                ENCHANTABLE_FORM_SAVE,
                AMMO_FORM_SAVE,
                DESTRUCTIBLE_FORM_SAVE,
                REPAIR_ITEM_LIST_SAVE,
                EQUIP_TYPE_SAVE,
                BIPED_MODEL_LIST_SAVE,
                PICKUP_PUTDOWN_SOUNDS_SAVE,
                MODEL_SAVE,
                CRITICAL_DATA_ENDIAN,
                VATS_SPECIAL_ENDIAN,
                SWAP_WORD_IN_PLACE,
            ],
        );
        returning(e, WEAPON_FLAG_4_TEST, 0);
        returning(e, SWAP_ON_SAVE, 0);
        returning(e, STRING_LENGTH, 0);
        form_id_getter(e);
        let data = Rc::new(RefCell::new(Vec::new()));
        let sink = data.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            // The pointer field of the block as the chunk sees it.
            let field = match a[0] {
                x if x == tag(b"DNAM") => e.mem.u32(a[1] + 0x24),
                x if x == tag(b"CRDT") => e.mem.u32(a[1] + 0x0C),
                _ => e.mem.u32(a[1]),
            };
            sink.borrow_mut().push((a[0], field, a[2]));
            Ret::default()
        });
        SaveLog {
            words: recording(e, ADD_CHUNK_WORD),
            arrays: recording(e, ADD_CHUNK_ARRAY),
            data,
        }
    }

    /// A form whose id (at `+0xC`) is `id`.
    fn form_with_id(e: &mut Engine, id: u32) -> u32 {
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form + 0xC, id);
        form
    }

    #[test]
    fn save_writes_ids_for_the_pointers_and_restores_them() {
        let mut e = Engine::new();
        let log = save_doubles(&mut e);
        let w = weapon(&mut e);
        let sound = form_with_id(&mut e, 0x111);
        let impact = form_with_id(&mut e, 0x222);
        let projectile = form_with_id(&mut e, 0x333);
        let critical = form_with_id(&mut e, 0x444);
        let vats = form_with_id(&mut e, 0x555);
        e.mem.set_u32(w.addr() + 0x21C, sound);
        e.mem.set_u32(w.addr() + 0x24C, impact);
        e.mem.set_u32(w.addr() + 0x118, projectile);
        e.mem.set_u32(w.addr() + 0x1CC, critical);
        e.mem.set_u32(w.addr() + 0x370, vats);
        e.mem.set_u32(w.addr() + 0x364, 2);
        e.call(0x0052_1730, &args![w]);
        let words = log.words.borrow();
        assert!(words.contains(&vec![tag(b"SNAM"), 0x111]));
        assert!(words.contains(&vec![tag(b"INAM"), 0x222]));
        assert_eq!(words.last().unwrap(), &vec![tag(b"VNAM"), 2]);
        // Only the fields that are set are written.
        assert_eq!(words.len(), 3);
        assert_eq!(
            log.data.borrow().as_slice(),
            &[
                (tag(b"DNAM"), 0x333, 0xCC),
                (tag(b"CRDT"), 0x444, 0x10),
                (tag(b"VATS"), 0x555, 0x14),
            ]
        );
        // The pointers are back.
        assert_eq!(e.mem.u32(w.addr() + 0x118), projectile);
        assert_eq!(e.mem.u32(w.addr() + 0x1CC), critical);
        assert_eq!(e.mem.u32(w.addr() + 0x370), vats);
    }

    #[test]
    fn save_writes_the_scope_model_and_effect_only_for_a_scoped_weapon() {
        let mut e = Engine::new();
        let log = save_doubles(&mut e);
        let models = recording(&mut e, MODEL_SAVE);
        let w = weapon(&mut e);
        let scope_effect = form_with_id(&mut e, 0x666);
        e.mem.set_u32(w.addr() + 0x218, scope_effect);
        e.call(0x0052_1730, &args![w]);
        assert!(!log.words.borrow().contains(&vec![tag(b"EFSD"), 0x666]));
        assert_eq!(models.borrow().len(), 2);
        returning(&mut e, WEAPON_FLAG_4_TEST, 1);
        models.borrow_mut().clear();
        e.call(0x0052_1730, &args![w]);
        assert!(log.words.borrow().contains(&vec![tag(b"EFSD"), 0x666]));
        let models = models.borrow();
        assert_eq!(models.len(), 3);
        assert_eq!(
            models[1],
            vec![w.addr() + SCOPE_MODEL, tag(b"MOD3"), tag(b"MO3T")]
        );
    }

    #[test]
    fn save_writes_the_strings_with_their_terminator() {
        let mut e = Engine::new();
        let log = save_doubles(&mut e);
        let w = weapon(&mut e);
        // The string at +0x35C is "abc"; its c-string pointer is 0x9000.
        let node_string = w.addr() + EMBEDDED_WEAPON_NODE;
        e.register_double(STRING_LENGTH, move |_, a| Ret {
            eax: if a[0] == node_string { 3 } else { 0 },
            ..Ret::default()
        });
        returning(&mut e, STRING_C_STR, 0x9000);
        e.call(0x0052_1730, &args![w]);
        assert_eq!(
            log.arrays.borrow().as_slice(),
            &[vec![tag(b"NNAM"), 0x9000, 4]]
        );
    }

    #[test]
    fn save_swaps_the_blocks_around_the_chunk_on_a_big_endian_save() {
        let mut e = Engine::new();
        let _log = save_doubles(&mut e);
        returning(&mut e, SWAP_ON_SAVE, 1);
        let critical = recording(&mut e, CRITICAL_DATA_ENDIAN);
        let vats = recording(&mut e, VATS_SPECIAL_ENDIAN);
        let words = recording(&mut e, SWAP_WORD_IN_PLACE);
        let w = weapon(&mut e);
        e.call(0x0052_1730, &args![w]);
        assert_eq!(critical.borrow().len(), 2);
        assert_eq!(vats.borrow().len(), 2);
        assert_eq!(critical.borrow()[0], vec![w.addr() + CRITICAL_DATA]);
        assert_eq!(vats.borrow()[0], vec![w.addr() + VATS_SPECIAL_DATA]);
        // The DATA block is swapped before and after: two times 44 words.
        assert_eq!(words.borrow().len(), 88);
    }

    #[test]
    fn save_size_adds_the_form_and_value_parts_as_a_word() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        returning(&mut e, FORM_GET_SAVE_SIZE, 0x10);
        returning(&mut e, VALUE_FORM_GET_SAVE_SIZE, 0xFFFF);
        e.call_log = Some(vec![]);
        let size = e.call(0x0052_3020, &args![w, 9u32]).u16();
        assert_eq!(size, 0x000F);
        let log = logged(&mut e);
        assert_eq!(log[1], (FORM_GET_SAVE_SIZE, vec![w.addr(), 9]));
        assert_eq!(
            log[2],
            (VALUE_FORM_GET_SAVE_SIZE, vec![w.addr() + VALUE_FORM, 9])
        );
    }

    #[test]
    fn save_game_entry_saves_both_parts() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        quiet(&mut e, &[FORM_SAVE_GAME, VALUE_FORM_SAVE_GAME]);
        e.call_log = Some(vec![]);
        e.call(0x0052_3080, &args![w, 4u32]);
        let log = logged(&mut e);
        assert_eq!(log[1], (FORM_SAVE_GAME, vec![w.addr(), 4]));
        assert_eq!(
            log[2],
            (VALUE_FORM_SAVE_GAME, vec![w.addr() + VALUE_FORM, 4])
        );
    }

    #[test]
    fn load_game_entry_loads_both_parts() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        quiet(&mut e, &[FORM_LOAD_GAME, VALUE_FORM_LOAD_GAME]);
        e.call_log = Some(vec![]);
        e.call(0x0052_30b0, &args![w, 4u32, 5u32]);
        let log = logged(&mut e);
        assert_eq!(log[1], (FORM_LOAD_GAME, vec![w.addr(), 4, 5]));
        assert_eq!(
            log[2],
            (VALUE_FORM_LOAD_GAME, vec![w.addr() + VALUE_FORM, 4, 5])
        );
    }

    #[test]
    fn the_two_other_vtable_entries_forward_to_both_parts() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        quiet(
            &mut e,
            &[
                FORM_VTABLE_ENTRY_570,
                VALUE_FORM_ENTRY_570,
                FORM_VTABLE_ENTRY_578,
                VALUE_FORM_ENTRY_578,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0052_30f0, &args![w, 6u32]);
        e.call(0x0052_3120, &args![w, 7u32]);
        let log = logged(&mut e);
        assert_eq!(log[1], (FORM_VTABLE_ENTRY_570, vec![w.addr(), 6]));
        assert_eq!(
            log[2],
            (VALUE_FORM_ENTRY_570, vec![w.addr() + VALUE_FORM, 6])
        );
        assert_eq!(log[4], (FORM_VTABLE_ENTRY_578, vec![w.addr(), 7]));
        assert_eq!(
            log[5],
            (VALUE_FORM_ENTRY_578, vec![w.addr() + VALUE_FORM, 7])
        );
    }

    #[test]
    fn x_rotation_fills_the_matrix_from_the_sine_and_cosine() {
        let mut e = Engine::new();
        e.register(SINE_COSINE, |e, a| {
            e.mem.set_f32(a[1], 0.5);
            e.mem.set_f32(a[2], 0.25);
            Ret::default()
        });
        let matrix = e.mem.alloc(36);
        e.mem.write(matrix, &[0xFF; 36]);
        e.call(0x0052_4ac0, &args![matrix, 1.0f32]);
        let values: Vec<f32> = (0..9).map(|i| e.mem.f32(matrix + 4 * i)).collect();
        assert_eq!(values, vec![1.0, 0.0, 0.0, 0.0, 0.25, 0.5, 0.0, -0.5, 0.25]);
    }

    #[test]
    fn flag_and_byte_getters_read_the_data_block() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let data = weapon_data(w);
        assert!(!e.call(0x0052_4b40, &args![w]).bool());
        e.set(data, ObjWeap::cFlags, 2);
        assert!(e.call(0x0052_4b40, &args![w]).bool());
        e.set(data, ObjWeap::cAmmoPerShot, 7);
        assert_eq!(e.call(0x0052_4b60, &args![w]).u8(), 7);
    }

    /// `max` double for the clamp helper and a mod effect value.
    fn spread_doubles(e: &mut Engine, effect: f64) {
        e.register(MAX_FLOAT, |_, a| Ret {
            st0: f32::from_bits(a[0]).max(f32::from_bits(a[1])) as f64,
            ..Ret::default()
        });
        returning_float(e, GET_MOD_EFFECT_VALUE, effect);
    }

    #[test]
    fn spread_getters_subtract_the_mod_effect_down_to_zero() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let data = weapon_data(w);
        e.set(data, ObjWeap::fMinSpread, 1.0);
        e.set(data, ObjWeap::fSpread, 2.0);
        spread_doubles(&mut e, 0.25);
        assert_eq!(e.call(0x0052_4b80, &args![w, 0u32]).f32(), 1.0);
        assert_eq!(e.call(0x0052_4b80, &args![w, 1u32]).f32(), 0.75);
        assert_eq!(e.call(0x0052_4be0, &args![w, 0u32]).f32(), 2.0);
        assert_eq!(e.call(0x0052_4be0, &args![w, 1u32]).f32(), 1.75);
        spread_doubles(&mut e, 5.0);
        assert_eq!(e.call(0x0052_4b80, &args![w, 1u32]).f32(), 0.0);
        assert_eq!(e.call(0x0052_4be0, &args![w, 1u32]).f32(), 0.0);
    }

    #[test]
    fn transform_point_chains_the_three_vector_helpers() {
        let mut e = Engine::new();
        let transform = e.mem.alloc(0x40);
        e.mem.set_f32(transform + 0x30, 2.0);
        let result = e.mem.alloc(16);
        returning(&mut e, MATRIX_MULTIPLY_VECTOR, 0xAAA0);
        returning(&mut e, VECTOR_SCALED, 0xBBB0);
        quiet(&mut e, &[VECTOR_ADD]);
        e.call_log = Some(vec![]);
        let out = e
            .call(0x0052_4c40, &args![transform, result, 0x1234u32])
            .u32();
        assert_eq!(out, result);
        let log = logged(&mut e);
        let multiply = &log[1];
        assert_eq!(multiply.0, MATRIX_MULTIPLY_VECTOR);
        assert_eq!(multiply.1[0], transform);
        assert_eq!(multiply.1[2], 0x1234);
        let temp = multiply.1[1];
        // The second step scales the first result into the other temporary,
        // by the float at +0x30.
        assert_eq!(
            log[2],
            (VECTOR_SCALED, vec![0xAAA0, temp + 12, 2.0f32.to_bits()])
        );
        // The last adds the translation at +0x24 into the result.
        assert_eq!(log[3], (VECTOR_ADD, vec![0xBBB0, result, transform + 0x24]));
    }

    #[test]
    fn camera_getter_chains_two_calls() {
        let mut e = Engine::new();
        e.register(STRING_C_STR, |_, a| Ret {
            eax: if a[0] == SINGLETON_011DEB7C {
                0x1111
            } else {
                0
            },
            ..Ret::default()
        });
        e.register(FOLDED_GETTER_006629F0, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0052_4c90, &[]).u32(), 0x1112);
    }

    #[test]
    fn global_getters_read_their_words() {
        let mut e = Engine::new();
        map_pages(&mut e, &[0x011c_621c, 0x011d_ea2a]);
        e.set_global(0x011c_621c, 0x1111u32);
        e.set_global(0x011c_6220, 0x2222u32);
        e.set_global(0x011d_ea2a, 5u8);
        assert_eq!(e.call(0x0052_4cb0, &[]).u32(), 0x1111);
        assert_eq!(e.call(0x0052_4cc0, &[]).u32(), 0x2222);
        assert_eq!(e.call(0x0052_5420, &[]).u8(), 5);
    }

    #[test]
    fn byte_accessors_read_and_write_their_offsets() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x700);
        e.mem.set_u8(object + 0xD, 9);
        e.mem.set_u8(object + 0x64B, 4);
        assert_eq!(e.call(0x0052_4cd0, &args![object]).u8(), 9);
        assert_eq!(e.call(0x0052_4d10, &args![object]).u8(), 4);
        e.call(0x0052_4d30, &args![object, 0x7Fu32]);
        assert_eq!(e.mem.u8(object + 0x1F8), 0x7F);
        assert_eq!(e.call(0x0052_4d50, &args![object]).u8(), 0x7F);
        e.mem.set_u32(object + 8, 1);
        assert!(e.call(0x0052_5430, &args![object]).bool());
        e.mem.set_u32(object + 8, 0);
        assert!(!e.call(0x0052_5430, &args![object]).bool());
    }

    #[test]
    fn call_through_forwards_the_word_and_one() {
        let mut e = Engine::new();
        let log = recording(&mut e, FN_00456FC0);
        e.call(0x0052_4cf0, &args![0x4444u32]);
        assert_eq!(log.borrow().as_slice(), &[vec![0x4444, 1]]);
        returning(&mut e, FN_00456FC0, 0x99);
        assert_eq!(e.call(0x0052_4cf0, &args![0x4444u32]).u32(), 0x99);
    }

    #[test]
    fn clears_a_word_and_copies_three_words() {
        let mut e = Engine::new();
        map_pages(&mut e, &[WORDS_011F426C]);
        e.set_global(WORDS_011F426C, 1.5f32);
        e.set_global(WORDS_011F426C + 4, 2.5f32);
        e.set_global(WORDS_011F426C + 8, 3.5f32);
        let object = e.mem.alloc(0xD40);
        e.mem.set_u32(object + 0xD2C, 0xFFFF);
        e.call(0x0052_4d70, &args![object]);
        assert_eq!(e.mem.u32(object + 0xD2C), 0);
        assert_eq!(e.mem.f32(object + 0xD30), 1.5);
        assert_eq!(e.mem.f32(object + 0xD34), 2.5);
        assert_eq!(e.mem.f32(object + 0xD38), 3.5);
    }

    /// A table for the reciprocal square root: the mantissa bits of
    /// `sqrt(1 + i / 128)` for the even exponents and of
    /// `sqrt(2 * (1 + (i - 128) / 128))` for the odd ones.
    fn inverse_sqrt_table(e: &mut Engine) {
        let table = e.mem.alloc(256 * 4);
        for i in 0..256u32 {
            let m = if i < 128 {
                1.0 + i as f64 / 128.0
            } else {
                2.0 * (1.0 + (i - 128) as f64 / 128.0)
            };
            let root = (m.sqrt() as f32).to_bits();
            // The mantissa bits of the root.
            e.mem.set_u32(table + 4 * i, root & 0x7F_FFFF);
        }
        map_pages(e, &[INVERSE_SQRT_TABLE]);
        e.set_global(INVERSE_SQRT_TABLE, table);
    }

    #[test]
    fn normalize_uses_the_reciprocal_root_table() {
        let mut e = Engine::new();
        inverse_sqrt_table(&mut e);
        let vector = e.mem.alloc(12);
        e.mem.set_f32(vector, 3.0);
        e.mem.set_f32(vector + 4, 4.0);
        e.mem.set_f32(vector + 8, 0.0);
        e.call(0x0052_5340, &args![vector]);
        // 3-4-5 triangle: the table lookup is exact for a squared length of
        // 25.
        assert_eq!(e.mem.f32(vector), 0.6);
        assert_eq!(e.mem.f32(vector + 4), 0.8);
        assert_eq!(e.mem.f32(vector + 8), 0.0);
    }

    #[test]
    fn normalize_leaves_a_zero_vector_zero() {
        let mut e = Engine::new();
        inverse_sqrt_table(&mut e);
        let vector = e.mem.alloc(12);
        e.call(0x0052_5340, &args![vector]);
        assert_eq!(e.mem.f32(vector), 0.0);
        // A squared length of 2 has an odd exponent and uses the upper half
        // of the table.
        e.mem.set_f32(vector, 1.0);
        e.mem.set_f32(vector + 4, 1.0);
        e.call(0x0052_5340, &args![vector]);
        let value = e.mem.f32(vector);
        assert!(
            (value - std::f32::consts::FRAC_1_SQRT_2).abs() < 0.01,
            "{value}"
        );
    }

    #[test]
    fn preload_entry_queues_the_casing_projectile_and_impact_files() {
        let mut e = Engine::new();
        let w = e.mem.alloc(0x400);
        let casing = w + SHELL_CASING_MODEL;
        returning(&mut e, MODEL_HAS_FILE, 1);
        map_pages(&mut e, &[MODEL_LOADER]);
        e.set_global(MODEL_LOADER, 0x7777u32);
        let queued = recording(&mut e, MODEL_LOADER_QUEUE);
        // The projectile and the impact data set each have a preloadable
        // part with `QueueFiles` in slot 0x10.
        let projectile = e.mem.alloc(0x100);
        give_vtable(
            &mut e,
            projectile + AMMO_PRELOADABLE,
            &[(PRELOADABLE_QUEUE_FILES_SLOT, 0x00dd_0101)],
        );
        returning(&mut e, GET_PROJECTILE, projectile);
        let impact = e.mem.alloc(0x100);
        give_vtable(
            &mut e,
            impact + IMPACT_DATA_SET_PRELOADABLE,
            &[(PRELOADABLE_QUEUE_FILES_SLOT, 0x00dd_0102)],
        );
        e.mem.set_u32(w + 0x24C, impact);
        let projectile_queue = recording(&mut e, 0x00dd_0101);
        let impact_queue = recording(&mut e, 0x00dd_0102);
        // Entered at the BGSPreloadable part, 0xCC into the weapon.
        e.call(0x0052_5450, &args![w + PRELOADABLE, 11u32, 22u32]);
        assert_eq!(
            queued.borrow().as_slice(),
            &[vec![0x7777, casing, 11, 22, 0, 1, 0, 0]]
        );
        assert_eq!(
            projectile_queue.borrow().as_slice(),
            &[vec![projectile + AMMO_PRELOADABLE, 11, 22]]
        );
        assert_eq!(
            impact_queue.borrow().as_slice(),
            &[vec![impact + IMPACT_DATA_SET_PRELOADABLE, 11, 22]]
        );
    }

    #[test]
    fn queue_files_adds_the_scope_model_with_its_detail_level() {
        let mut e = Engine::new();
        let w = e.mem.alloc(0x400);
        returning(&mut e, MODEL_HAS_FILE, 1);
        map_pages(&mut e, &[MODEL_LOADER]);
        e.set_global(MODEL_LOADER, 0x7777u32);
        let queued = recording(&mut e, MODEL_LOADER_QUEUE);
        returning(&mut e, GET_PROJECTILE, 0);
        returning(&mut e, GET_LOD_MULT, 2);
        // Without the scope flag only the casing is queued.
        returning(&mut e, WEAPON_FLAG_4_TEST, 0);
        e.call(0x0052_5510, &args![w, 1u32, 2u32, 1u32]);
        assert_eq!(queued.borrow().len(), 1);
        queued.borrow_mut().clear();
        returning(&mut e, WEAPON_FLAG_4_TEST, 1);
        e.call(0x0052_5510, &args![w, 1u32, 2u32, 1u32]);
        assert_eq!(
            queued.borrow()[1],
            vec![0x7777, w + SCOPE_MODEL, 1, 2, 2, 1, 0, 0]
        );
        // The caller can leave the scope out.
        queued.borrow_mut().clear();
        e.call(0x0052_5510, &args![w, 1u32, 2u32, 0u32]);
        assert_eq!(queued.borrow().len(), 1);
    }

    #[test]
    fn setting_word_follows_the_sound_level() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let values: Vec<u32> = (0..3).map(|_| e.mem.alloc(8)).collect();
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(*v, 100 + i as u32);
        }
        let table = values.clone();
        e.register_double(SETTING_GET_WORD_POINTER, move |_, a| Ret {
            eax: match a[0] {
                SETTING_SOUND_LEVEL_LOUD => table[0],
                SETTING_SOUND_LEVEL_NORMAL => table[1],
                _ => table[2],
            },
            ..Ret::default()
        });
        for (level, expected) in [(0u32, 100u32), (1, 101), (2, 102), (5, 0)] {
            e.set(w, TesObjectWeap::eSoundLevel, level);
            assert_eq!(e.call(0x0052_5620, &args![w, 0u32, 0u32]).u32(), expected);
        }
        e.set(w, TesObjectWeap::eSoundLevel, 0);
        assert_eq!(e.call(0x0052_5620, &args![w, 1u32, 0u32]).u32(), 102);
        // The second flag wins.
        assert_eq!(e.call(0x0052_5620, &args![w, 1u32, 1u32]).u32(), 101);
    }

    #[test]
    fn projectile_node_name_prefers_the_embedded_node() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        returning(&mut e, WEAPON_FLAG_20_TEST, 1);
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"Muzzle");
        returning(&mut e, GET_EMBEDDED_WEAPON_NODE, name);
        assert_eq!(e.call(0x0052_56b0, &args![w]).u32(), name);
        // An empty name, no name or no flag gives the default.
        e.mem.set_cstr(name, b"");
        assert_eq!(e.call(0x0052_56b0, &args![w]).u32(), PROJECTILE_NODE_NAME);
        returning(&mut e, GET_EMBEDDED_WEAPON_NODE, 0);
        assert_eq!(e.call(0x0052_56b0, &args![w]).u32(), PROJECTILE_NODE_NAME);
        returning(&mut e, WEAPON_FLAG_20_TEST, 0);
        assert_eq!(e.call(0x0052_56b0, &args![w]).u32(), PROJECTILE_NODE_NAME);
    }

    #[test]
    fn fire_node_falls_back_to_the_hashed_name() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        returning(&mut e, WEAPON_FLAG_20_TEST, 0);
        e.register(FIND_NODE_BY_NAME, |_, a| Ret {
            eax: if a[1] == HASHED_PROJECTILE_NODE_NAME {
                0x7700
            } else {
                0
            },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0052_5700, &args![w, 0x5000u32]).u32(), 0x7700);
        let log: Vec<_> = logged(&mut e)
            .into_iter()
            .filter(|(addr, _)| *addr == FIND_NODE_BY_NAME)
            .collect();
        assert_eq!(
            log,
            vec![
                (FIND_NODE_BY_NAME, vec![0x5000, PROJECTILE_NODE_NAME]),
                (FIND_NODE_BY_NAME, vec![0x5000, HASHED_PROJECTILE_NODE_NAME]),
            ]
        );
        // A hit on the first name stops there.
        returning(&mut e, FIND_NODE_BY_NAME, 0x7800);
        assert_eq!(e.call(0x0052_5700, &args![w, 0x5000u32]).u32(), 0x7800);
    }

    #[test]
    fn forwarding_entry_casts_and_passes_five_words() {
        let mut e = Engine::new();
        let cast = recording(&mut e, RT_DYNAMIC_CAST);
        let forwarded: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = forwarded.clone();
        e.register_double(FN_0050F670, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: 0x42,
                ..Ret::default()
            }
        });
        let r = e
            .call(
                0x0052_5750,
                &args![0x10u32, 0x20u32, 0x30u32, 1u32, 0x50u32, 0x60u32],
            )
            .u32();
        assert_eq!(r, 0x42);
        assert_eq!(
            cast.borrow().as_slice(),
            &[vec![0x30, 0, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0]]
        );
        assert_eq!(
            forwarded.borrow().as_slice(),
            &[vec![0x10, 0x20, 0x30, 1, 0x50, 0x60]]
        );
    }

    /// An actor whose process is a heap object with the item getters in
    /// slots `0x14c` (ammo item) and `0x148` (weapon item).
    fn actor_with_items(e: &mut Engine, ammo_item: u32, weapon_item: u32) -> (u32, u32) {
        let actor = e.mem.alloc(0x800);
        let process = e.mem.alloc(0x10);
        give_vtable(
            e,
            process,
            &[
                (PROCESS_AMMO_ITEM_SLOT, 0x00dd_014c),
                (PROCESS_WEAPON_ITEM_SLOT, 0x00dd_0148),
            ],
        );
        returning(e, 0x00dd_014c, ammo_item);
        returning(e, 0x00dd_0148, weapon_item);
        e.register_double(ACTOR_PROCESS, move |_, _| Ret {
            eax: process,
            ..Ret::default()
        });
        (actor, process)
    }

    /// Item to form doubles: item `0xA1` is form `0xF1`, `0xB1` is `0xF2`.
    fn item_forms(e: &mut Engine) {
        e.register(HELD_ITEM_GET_FORM, |_, a| Ret {
            eax: match a[0] {
                0xA1 => 0xF1,
                0xB1 => 0xF2,
                _ => 0,
            },
            ..Ret::default()
        });
    }

    #[test]
    fn current_ammo_is_the_default_without_an_actor() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        returning(&mut e, AMMO_FORM_GET_AMMO, 0xDEF);
        assert_eq!(e.call(0x0052_5980, &args![w, 0u32]).u32(), 0xDEF);
        e.call_log = Some(vec![]);
        e.call(0x0052_5980, &args![w, 0u32]);
        let calls = logged(&mut e);
        assert_eq!(calls[1], (AMMO_FORM_GET_AMMO, vec![w.addr() + AMMO_FORM]));
    }

    #[test]
    fn current_ammo_is_the_actors_ammo_form_when_it_casts() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        item_forms(&mut e);
        let (actor, _) = actor_with_items(&mut e, 0xA1, 0xB1);
        returning(&mut e, AMMO_FORM_GET_AMMO, 0xDEF);
        returning(&mut e, RT_DYNAMIC_CAST, 0xF1);
        assert_eq!(e.call(0x0052_5980, &args![w, actor]).u32(), 0xF1);
        // The cast failed: back to the default.
        returning(&mut e, RT_DYNAMIC_CAST, 0);
        assert_eq!(e.call(0x0052_5980, &args![w, actor]).u32(), 0xDEF);
        // The weapon item has the same form as the ammo item: default.
        returning(&mut e, RT_DYNAMIC_CAST, 0xF1);
        e.register(HELD_ITEM_GET_FORM, |_, _| Ret {
            eax: 0xF1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0052_5980, &args![w, actor]).u32(), 0xDEF);
    }

    #[test]
    fn current_ammo_is_the_default_without_an_ammo_item() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        item_forms(&mut e);
        let (actor, _) = actor_with_items(&mut e, 0, 0xB1);
        returning(&mut e, AMMO_FORM_GET_AMMO, 0xDEF);
        assert_eq!(e.call(0x0052_5980, &args![w, actor]).u32(), 0xDEF);
    }

    #[test]
    fn projectile_count_multiplies_ammo_count_and_ammo_per_shot() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let data = weapon_data(w);
        e.set(data, ObjWeap::cNumProjectiles, 1);
        e.set(data, ObjWeap::cAmmoPerShot, 2);
        let ammo = e.mem.alloc(0x100);
        e.mem.set_u8(ammo + 0xB0, 3);
        returning(&mut e, AMMO_FORM_GET_AMMO, ammo);
        returning_float(&mut e, GET_MOD_EFFECT_VALUE, 2.9);
        // (with mod, skip ammo, actor)
        assert_eq!(e.call(0x0052_5b20, &args![w, 0u32, 0u32, 0u32]).u8(), 6);
        // Skipping the ammo keeps the weapon's own count.
        assert_eq!(e.call(0x0052_5b20, &args![w, 0u32, 1u32, 0u32]).u8(), 1);
        // The mod effect is added and the sum truncated.
        assert_eq!(e.call(0x0052_5b20, &args![w, 1u32, 1u32, 0u32]).u8(), 3);
        // An ammo without a count leaves the weapon's own.
        e.mem.set_u8(ammo + 0xB0, 0);
        assert_eq!(e.call(0x0052_5b20, &args![w, 0u32, 0u32, 0u32]).u8(), 1);
        assert_eq!(e.call(0x0052_5be0, &args![ammo]).u8(), 0);
    }

    /// Doubles for `fn_005257a0`: a position `(1, 2, 3)` for the reference
    /// and a node whose world translation is `(7, 8, 9)`.
    fn muzzle_doubles(e: &mut Engine) -> (u32, u32, u32) {
        let position = e.mem.alloc(12);
        for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, v);
        }
        let translation = e.mem.alloc(12);
        for (i, v) in [7.0f32, 8.0, 9.0].into_iter().enumerate() {
            e.mem.set_f32(translation + 4 * i as u32, v);
        }
        returning(e, REFERENCE_POSITION, position);
        returning(e, NODE_WORLD_TRANSLATION, translation);
        map_pages(e, &[PLAYER_CHARACTER]);
        e.set_global(PLAYER_CHARACTER, 0x9999u32);
        let result = e.mem.alloc(12);
        (position, translation, result)
    }

    #[test]
    fn muzzle_of_a_non_actor_is_the_fire_node_or_the_position() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let (_, _, result) = muzzle_doubles(&mut e);
        let reference = e.mem.alloc(0x40);
        give_vtable(
            &mut e,
            reference,
            &[
                (REFERENCE_IS_ACTOR_SLOT, 0x00dd_0100),
                (REFERENCE_GET_3D_SLOT, 0x00dd_01d0),
            ],
        );
        returning(&mut e, 0x00dd_0100, 0);
        returning(&mut e, 0x00dd_01d0, 0x5000);
        returning(&mut e, WEAPON_FLAG_20_TEST, 0);
        returning(&mut e, FIND_NODE_BY_NAME, 0x6000);
        let out = e.call(0x0052_57a0, &args![w, result, reference]).u32();
        assert_eq!(out, result);
        assert_eq!(
            [
                e.mem.f32(result),
                e.mem.f32(result + 4),
                e.mem.f32(result + 8)
            ],
            [7.0, 8.0, 9.0]
        );
        // No node: the reference's position.
        returning(&mut e, FIND_NODE_BY_NAME, 0);
        e.call(0x0052_57a0, &args![w, result, reference]);
        assert_eq!(
            [
                e.mem.f32(result),
                e.mem.f32(result + 4),
                e.mem.f32(result + 8)
            ],
            [1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn muzzle_of_an_actor_is_raised_by_its_height() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let (_, _, result) = muzzle_doubles(&mut e);
        let (actor, process) = actor_with_items(&mut e, 0, 0);
        give_vtable(
            &mut e,
            actor,
            &[
                (REFERENCE_IS_ACTOR_SLOT, 0x00dd_0100),
                (ACTOR_GET_BIPED_SLOT, 0x00dd_01ec),
            ],
        );
        returning(&mut e, 0x00dd_0100, 1);
        returning(&mut e, 0x00dd_01ec, 0x321);
        // Slot 0x190 of the process gives the weapon node of a biped.
        let vtable = e.mem.u32(process);
        e.mem
            .set_u32(vtable + PROCESS_GET_WEAPON_NODE_SLOT, 0x00dd_0190);
        e.register(0x00dd_0190, |_, a| Ret {
            eax: if a[1] == 0x321 { 0x6100 } else { 0 },
            ..Ret::default()
        });
        returning_float(&mut e, ACTOR_GET_HEIGHT, 1.5);
        e.register(GET_WEAPON_TYPE, |_, _| Ret {
            eax: 10,
            ..Ret::default()
        });
        // A held weapon of type 10: the node of the process.
        e.call(0x0052_57a0, &args![w, result, actor]);
        assert_eq!(
            [
                e.mem.f32(result),
                e.mem.f32(result + 4),
                e.mem.f32(result + 8)
            ],
            [7.0, 8.0, 9.0]
        );
        // Without a node: the position, raised by the height.
        e.register(0x00dd_0190, |_, _| Ret::default());
        e.call(0x0052_57a0, &args![w, result, actor]);
        assert_eq!(
            [
                e.mem.f32(result),
                e.mem.f32(result + 4),
                e.mem.f32(result + 8)
            ],
            [1.0, 2.0, 4.5]
        );
    }

    /// What the doubled `TESFile` hands out while a record is loaded.
    #[derive(Default)]
    struct FileState {
        chunks: VecDeque<u32>,
        words: VecDeque<u32>,
        size: u32,
        bytes: Vec<u8>,
        version: u16,
        swaps: bool,
        form_type: u8,
    }

    type File = Rc<RefCell<FileState>>;

    /// Doubles for the file and every callee `Load` has outside this file;
    /// the chunk list and the data the file returns are in the state.
    fn load_doubles(e: &mut Engine, chunks: &[u32], words: &[u32]) -> File {
        let state: File = Rc::new(RefCell::new(FileState {
            chunks: chunks.iter().copied().collect(),
            words: words.iter().copied().collect(),
            form_type: 0x28,
            version: 15,
            ..FileState::default()
        }));
        quiet(
            e,
            &[
                FORM_LOAD_FORM,
                FORM_SET_FLAG_8,
                SWAP_WORD_IN_PLACE,
                FORM_LOAD_DATA,
                FULL_NAME_LOAD,
                MODEL_LOAD_MODEL_CHUNK,
                MODEL_LOAD_TEXTURE_CHUNK,
                UNPACK_TEXTURE_SWAP,
                TEXTURE_LOAD_CHUNK,
                MESSAGE_ICON_LOAD,
                DESTRUCTIBLE_LOAD_CHUNK,
                STORE_FORM_ID,
                STORE_WORD_AT_8,
                SCRIPTABLE_FORM_INIT_ITEM,
                ENCHANTABLE_SET_AMOUNT,
                MEMSET,
                LOG_MESSAGE,
                STRING_ASSIGN,
                CRITICAL_DATA_ENDIAN,
                VATS_SPECIAL_ENDIAN,
                LOAD_NAM_FACE_SKINNED,
                LOAD_NAM_SOUND_A,
                LOAD_NAM_SOUND_B,
                LOAD_NAM_BOUND,
                LOAD_NAM_TNAM,
                LOAD_NAM_UNAM,
                LOAD_NAM_NAM6,
                LOAD_NAM_NAM7,
                LOAD_NAM_NAM8,
                LOAD_NAM_NAM9,
                LOAD_NAM_WNAM,
                LOAD_WMS_A,
                LOAD_WMS_B,
                LOAD_WMS2,
                LOAD_WNM,
                LOAD_WMI,
                SET_SCOPE_EFFECT,
            ],
        );
        returning(e, LOAD_NAM_SOUND_CHECK, 0);
        returning(e, LOAD_WMS_CHECK, 0);
        returning(e, GET_PROJECTILE, 0);
        returning(e, SKILL_IS_VALID, 1);
        returning(e, RESISTANCE_IS_VALID, 1);
        form_id_getter(e);
        let file = state.clone();
        e.register_double(FILE_GET_FORM_TYPE, move |_, _| Ret {
            eax: file.borrow().form_type as u32,
            ..Ret::default()
        });
        let file = state.clone();
        e.register_double(FILE_GET_CHUNK, move |_, _| Ret {
            eax: file.borrow_mut().chunks.pop_front().unwrap_or(0),
            ..Ret::default()
        });
        let file = state.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| Ret {
            eax: !file.borrow().chunks.is_empty() as u32,
            ..Ret::default()
        });
        let file = state.clone();
        e.register_double(FILE_CHUNK_SIZE, move |_, _| Ret {
            eax: file.borrow().size,
            ..Ret::default()
        });
        let file = state.clone();
        e.register_double(FILE_CHUNK_VERSION, move |_, _| Ret {
            eax: file.borrow().version as u32,
            ..Ret::default()
        });
        let file = state.clone();
        e.register_double(FILE_SWAPS, move |_, _| Ret {
            eax: file.borrow().swaps as u32,
            ..Ret::default()
        });
        let file = state.clone();
        e.register_double(FILE_GET_CHUNK_U32, move |e, a| {
            let word = file.borrow_mut().words.pop_front().unwrap_or(0);
            e.mem.set_u32(a[1], word);
            Ret::default()
        });
        let file = state.clone();
        e.register_double(FILE_GET_CHUNK_U16, move |e, a| {
            let word = file.borrow_mut().words.pop_front().unwrap_or(0);
            e.mem.set_u16(a[1], word as u16);
            Ret::default()
        });
        let file = state.clone();
        e.register_double(FILE_GET_CHUNK_BYTES, move |e, a| {
            let state = file.borrow();
            let count = state
                .bytes
                .len()
                .min(if a[2] == 0 { usize::MAX } else { a[2] as usize });
            e.mem.write(a[1], &state.bytes[..count]);
            Ret::default()
        });
        state
    }

    #[test]
    fn load_refuses_a_record_that_is_not_a_weapon() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let file = load_doubles(&mut e, &[tag(b"NAM0")], &[1]);
        file.borrow_mut().form_type = 0x27;
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0051_f630, &args![w, 0x7000u32]).bool());
        assert_eq!(
            addresses(&logged(&mut e)),
            vec![0x0051_f630, FILE_GET_FORM_TYPE]
        );
    }

    #[test]
    fn load_stores_form_ids_into_their_components() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let _file = load_doubles(
            &mut e,
            &[
                tag(b"NAM0"),
                tag(b"REPL"),
                tag(b"SCRI"),
                tag(b"YNAM"),
                tag(b"ZNAM"),
                tag(b"WNM3"),
                tag(b"WMI3"),
                tag(b"SNAM"),
                tag(b"EFSD"),
            ],
            &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99],
        );
        let ids = recording(&mut e, STORE_FORM_ID);
        let word8 = recording(&mut e, STORE_WORD_AT_8);
        let scripts = recording(&mut e, SCRIPTABLE_FORM_INIT_ITEM);
        let wnm = recording(&mut e, LOAD_WNM);
        let wmi = recording(&mut e, LOAD_WMI);
        let sounds = recording(&mut e, LOAD_NAM_SOUND_A);
        let scope = recording(&mut e, SET_SCOPE_EFFECT);
        let flags = recording(&mut e, FORM_SET_FLAG_8);
        assert!(e.call(0x0051_f630, &args![w, 0x7000u32]).bool());
        let a = w.addr();
        assert_eq!(
            ids.borrow().as_slice(),
            &[
                vec![a + AMMO_FORM, 0x11],
                vec![a + REPAIR_ITEM_LIST, 0x22],
                vec![a + SCRIPTABLE_FORM, 0x33],
                vec![a + PICKUP_PUTDOWN_SOUNDS, 0x44],
            ]
        );
        assert_eq!(
            word8.borrow().as_slice(),
            &[vec![a + PICKUP_PUTDOWN_SOUNDS, 0x55]]
        );
        assert_eq!(scripts.borrow().as_slice(), &[vec![a + SCRIPTABLE_FORM, a]]);
        assert_eq!(wnm.borrow().as_slice(), &[vec![a, 3, 0x66]]);
        // The third mod object chunk is slot 4.
        assert_eq!(wmi.borrow().as_slice(), &[vec![a, 4, 0x77]]);
        assert_eq!(sounds.borrow().as_slice(), &[vec![a, 0x88]]);
        assert_eq!(scope.borrow().as_slice(), &[vec![a, 0x99]]);
        // The form flag is cleared at the start.
        assert_eq!(flags.borrow().as_slice(), &[vec![a, 0]]);
    }

    #[test]
    fn load_picks_the_sound_setter_by_the_check_result() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let _file = load_doubles(&mut e, &[tag(b"SNAM"), tag(b"WMS1")], &[1, 2]);
        returning(&mut e, LOAD_NAM_SOUND_CHECK, 1);
        returning(&mut e, LOAD_WMS_CHECK, 1);
        let b = recording(&mut e, LOAD_NAM_SOUND_B);
        let wms = recording(&mut e, LOAD_WMS_B);
        e.call(0x0051_f630, &args![w, 0x7000u32]);
        assert_eq!(b.borrow().as_slice(), &[vec![w.addr(), 1]]);
        assert_eq!(wms.borrow().as_slice(), &[vec![w.addr(), 2]]);
    }

    #[test]
    fn load_clamps_the_sound_level() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let _file = load_doubles(&mut e, &[tag(b"VNAM")], &[2]);
        e.call(0x0051_f630, &args![w, 0x7000u32]);
        assert_eq!(e.get(w, TesObjectWeap::eSoundLevel), 2);
        let _file = load_doubles(&mut e, &[tag(b"VNAM")], &[9]);
        e.call(0x0051_f630, &args![w, 0x7000u32]);
        assert_eq!(e.get(w, TesObjectWeap::eSoundLevel), 1);
    }

    #[test]
    fn load_sets_model_paths_through_the_models_virtual_setter() {
        let mut e = Engine::new();
        let w = e.mem.alloc(0x400);
        let file = load_doubles(&mut e, &[tag(b"MOD4"), tag(b"MWD7"), tag(b"MO4T")], &[]);
        {
            let mut state = file.borrow_mut();
            state.size = 6;
            state.bytes = b"a.nif\0".to_vec();
        }
        let paths: PathLog = Rc::new(RefCell::new(Vec::new()));
        let sink = paths.clone();
        e.register_double(0x00dd_0018, move |e, a| {
            let text = if a[1] == 0 {
                Vec::new()
            } else {
                e.mem.cstr(a[1])
            };
            sink.borrow_mut().push((a[0], text));
            Ret::default()
        });
        give_vtable(
            &mut e,
            w + WORLD_MODEL,
            &[(MODEL_SET_MODEL_SLOT, 0x00dd_0018)],
        );
        give_vtable(
            &mut e,
            w + WORLD_MODEL_MOD_123,
            &[(MODEL_SET_MODEL_SLOT, 0x00dd_0018)],
        );
        let textures = recording(&mut e, MODEL_LOAD_TEXTURE_CHUNK);
        e.call(0x0051_f630, &args![w, 0x7000u32]);
        assert_eq!(
            paths.borrow().as_slice(),
            &[
                (w + WORLD_MODEL, b"a.nif".to_vec()),
                (w + WORLD_MODEL_MOD_123, b"a.nif".to_vec())
            ]
        );
        assert_eq!(
            textures.borrow().as_slice(),
            &[vec![w + WORLD_MODEL, 0x7000]]
        );
        // An empty chunk gives the setter a null path.
        file.borrow_mut().size = 0;
        file.borrow_mut().chunks.push_back(tag(b"MOD4"));
        paths.borrow_mut().clear();
        e.call(0x0051_f630, &args![w, 0x7000u32]);
        assert_eq!(paths.borrow().as_slice(), &[(w + WORLD_MODEL, Vec::new())]);
    }

    #[test]
    fn load_reads_the_critical_and_vats_blocks_and_swaps_them() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let file = load_doubles(&mut e, &[tag(b"CRDT"), tag(b"VATS")], &[]);
        file.borrow_mut().bytes = vec![0xAB; 0x14];
        file.borrow_mut().swaps = true;
        let critical = recording(&mut e, CRITICAL_DATA_ENDIAN);
        let vats = recording(&mut e, VATS_SPECIAL_ENDIAN);
        e.call(0x0051_f630, &args![w, 0x7000u32]);
        assert_eq!(e.mem.u8(w.addr() + CRITICAL_DATA), 0xAB);
        assert_eq!(e.mem.u8(w.addr() + CRITICAL_DATA + 0xF), 0xAB);
        assert_eq!(e.mem.u8(w.addr() + CRITICAL_DATA + 0x10), 0);
        assert_eq!(e.mem.u8(w.addr() + VATS_SPECIAL_DATA + 0x13), 0xAB);
        assert_eq!(
            critical.borrow().as_slice(),
            &[vec![w.addr() + CRITICAL_DATA]]
        );
        assert_eq!(
            vats.borrow().as_slice(),
            &[vec![w.addr() + VATS_SPECIAL_DATA]]
        );
    }

    #[test]
    fn load_reads_the_enchantment_amount_as_a_short() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let _file = load_doubles(&mut e, &[tag(b"EAMT")], &[0x1234_5678]);
        let amount = recording(&mut e, ENCHANTABLE_SET_AMOUNT);
        e.call(0x0051_f630, &args![w, 0x7000u32]);
        assert_eq!(
            amount.borrow().as_slice(),
            &[vec![w.addr() + ENCHANTABLE_FORM, 0x5678]]
        );
    }

    /// 0xCC bytes of DATA with the given weapon type, attack animation,
    /// hand grip animation and limb multiplier.
    fn data_bytes(
        attack_anim: u8,
        hand_grip: u8,
        limb: f32,
        skill: u32,
        resistance: u32,
    ) -> Vec<u8> {
        let mut bytes = vec![0u8; 0xCC];
        bytes[0x29] = attack_anim;
        bytes[0xD] = hand_grip;
        bytes[0x74..0x78].copy_from_slice(&limb.to_le_bytes());
        bytes[0x68..0x6C].copy_from_slice(&skill.to_le_bytes());
        bytes[0x78..0x7C].copy_from_slice(&resistance.to_le_bytes());
        bytes
    }

    #[test]
    fn load_validates_a_current_data_block() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        map_pages(
            &mut e,
            &[ZERO, DEFAULT_FIRE_DELAY_MIN, DEFAULT_FIRE_DELAY_MAX],
        );
        e.set_global(DEFAULT_FIRE_DELAY_MIN, 0.1f32);
        e.set_global(DEFAULT_FIRE_DELAY_MAX, 0.5f32);
        let file = load_doubles(&mut e, &[tag(b"DNAM")], &[]);
        file.borrow_mut().bytes = data_bytes(0x50, 0x10, 0.0, 7, 9);
        returning(&mut e, SKILL_IS_VALID, 0);
        returning(&mut e, RESISTANCE_IS_VALID, 0);
        let memset = recording(&mut e, MEMSET);
        e.call(0x0051_f630, &args![w, 0x7000u32]);
        let data = weapon_data(w);
        assert_eq!(memset.borrow().as_slice(), &[vec![data.addr(), 0, 0xCC]]);
        // Reset values were overwritten by the block: only the checks show.
        assert_eq!(e.get(data, ObjWeap::fLimbDamageMult), 1.0);
        assert_eq!(e.get(data, ObjWeap::cHandGripAnim), 0xFF);
        assert_eq!(e.get(data, ObjWeap::eSkill), 0x2D);
        assert_eq!(e.get(data, ObjWeap::eResistance), 0xFFFF_FFFF);
        assert_eq!(e.get(data, ObjWeap::cAttackAnim), 0x50);
        // Valid values and a hand grip in range stay.
        returning(&mut e, SKILL_IS_VALID, 1);
        returning(&mut e, RESISTANCE_IS_VALID, 1);
        file.borrow_mut().bytes = data_bytes(0x50, 0xE8, 2.0, 7, 9);
        file.borrow_mut().chunks.push_back(tag(b"DNAM"));
        e.call(0x0051_f630, &args![w, 0x7000u32]);
        assert_eq!(e.get(data, ObjWeap::fLimbDamageMult), 2.0);
        assert_eq!(e.get(data, ObjWeap::cHandGripAnim), 0xE8);
        assert_eq!(e.get(data, ObjWeap::eSkill), 7);
        assert_eq!(e.get(data, ObjWeap::eResistance), 9);
    }

    #[test]
    fn load_shifts_the_attack_animation_of_older_data_blocks() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        map_pages(
            &mut e,
            &[ZERO, DEFAULT_FIRE_DELAY_MIN, DEFAULT_FIRE_DELAY_MAX],
        );
        e.set_global(DEFAULT_FIRE_DELAY_MIN, 0.1f32);
        e.set_global(DEFAULT_FIRE_DELAY_MAX, 0.5f32);
        let data = weapon_data(w);
        // Version 11: from 0x38 up, +0x12, then +6 from 0x56 up.
        for (version, from, to) in [
            (11u16, 0x37u8, 0x37u8),
            (11, 0x38, 0x4A),
            (11, 0x45, 0x57 + 6),
            (11, 0xA8, 0xA8),
            (13, 0x55, 0x55),
            (13, 0x56, 0x5C),
            (13, 0xA8, 0xA8),
        ] {
            let file = load_doubles(&mut e, &[tag(b"DNAM")], &[]);
            {
                let mut state = file.borrow_mut();
                state.version = version;
                state.size = 0xCC;
                state.bytes = data_bytes(from, 0, 1.0, 0, 0)[..0x80].to_vec();
            }
            e.call(0x0051_f630, &args![w, 0x7000u32]);
            assert_eq!(
                e.get(data, ObjWeap::cAttackAnim),
                to,
                "version {version} animation {from:#x}"
            );
            // The reset values of the part the chunk does not cover stay.
            assert_eq!(e.get(data, ObjWeap::fSemiAutomaticFireDelayMin), 0.1);
            assert_eq!(e.get(data, ObjWeap::fSemiAutomaticFireDelayMax), 0.5);
        }
    }

    #[test]
    fn load_swaps_a_big_endian_data_block() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        map_pages(
            &mut e,
            &[ZERO, DEFAULT_FIRE_DELAY_MIN, DEFAULT_FIRE_DELAY_MAX],
        );
        let file = load_doubles(&mut e, &[tag(b"DNAM")], &[]);
        file.borrow_mut().swaps = true;
        file.borrow_mut().bytes = data_bytes(0, 0, 1.0, 0, 0);
        let swaps = recording(&mut e, SWAP_WORD_IN_PLACE);
        e.call(0x0051_f630, &args![w, 0x7000u32]);
        assert_eq!(swaps.borrow().len(), 44);
    }

    #[test]
    fn load_finishes_by_fixing_projectiles_and_type() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let _file = load_doubles(&mut e, &[], &[]);
        let log = recording(&mut e, LOG_MESSAGE);
        give_vtable(
            &mut e,
            w.addr(),
            &[(FORM_GET_OBJECT_TYPE_NAME_SLOT, 0x00dd_0130)],
        );
        returning(&mut e, 0x00dd_0130, 0xBEEF);
        e.mem.set_u32(w.addr() + 0xC, 0x1234);
        // A projectile and no projectile count: the count becomes 1.
        returning(&mut e, GET_PROJECTILE, 0x4000);
        let data = weapon_data(w);
        e.set(data, ObjWeap::eType, 14);
        e.call(0x0051_f630, &args![w, 0x7000u32]);
        assert_eq!(e.get(data, ObjWeap::cNumProjectiles), 1);
        assert_eq!(e.get(data, ObjWeap::eType), 0);
        assert_eq!(
            log.borrow().as_slice(),
            &[vec![CLEARING_INVALID_TYPE_MESSAGE, 0xBEEF, 0x1234]]
        );
        // A valid type (13) or an existing count stays.
        e.set(data, ObjWeap::eType, 13);
        e.set(data, ObjWeap::cNumProjectiles, 3);
        e.call(0x0051_f630, &args![w, 0x7000u32]);
        assert_eq!(e.get(data, ObjWeap::eType), 13);
        assert_eq!(e.get(data, ObjWeap::cNumProjectiles), 3);
    }

    /// A `float` setting double: the pointer for each setting address.
    fn float_settings(e: &mut Engine, settings: &[(u32, f32)]) {
        let table: Vec<(u32, u32)> = settings
            .iter()
            .map(|(setting, value)| {
                let cell = e.mem.alloc(8);
                e.mem.set_f32(cell, *value);
                (*setting, cell)
            })
            .collect();
        e.register_double(SETTING_GET_FLOAT_POINTER, move |_, a| Ret {
            eax: table
                .iter()
                .find(|(setting, _)| *setting == a[0])
                .map(|(_, cell)| *cell)
                .unwrap_or_else(|| panic!("unexpected setting {:08x}", a[0])),
            ..Ret::default()
        });
    }

    /// Writes `values` as consecutive floats into a fresh block.
    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        block
    }

    /// The three floats at `vector`.
    fn vec3(e: &Engine, vector: u32) -> [f32; 3] {
        [
            e.mem.f32(vector),
            e.mem.f32(vector + 4),
            e.mem.f32(vector + 8),
        ]
    }

    /// Doubles for the vector helpers `ExpelShellCasing` uses: subtract,
    /// add-assign, scale-assign, construct and matrix-times-vector (the
    /// matrix is the identity).
    fn vector_helpers(e: &mut Engine) {
        e.register(VECTOR_SUBTRACT, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(VECTOR_ADD_ASSIGN, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, value);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(VECTOR_SCALE_ASSIGN, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) * f32::from_bits(a[1]);
                e.mem.set_f32(a[0] + 4 * i, value);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(VECTOR_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(MATRIX_MULTIPLY_VECTOR, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
    }

    #[test]
    fn casing_needs_an_actor_a_model_and_the_node() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        // Not an actor: nothing else is called.
        let actor = e.mem.alloc(0x100);
        give_vtable(&mut e, actor, &[(REFERENCE_IS_ACTOR_SLOT, 0x00dd_0100)]);
        returning(&mut e, 0x00dd_0100, 0);
        e.call_log = Some(vec![]);
        e.call(0x0052_4db0, &args![w, actor]);
        assert_eq!(addresses(&logged(&mut e)), vec![0x0052_4db0, 0x00dd_0100]);
        e.call(0x0052_4db0, &args![w, 0u32]);
        // An actor but no casing model, then casings off, then no node.
        returning(&mut e, 0x00dd_0100, 1);
        returning(&mut e, GET_SHELL_CASING_MODEL, 0x1234);
        returning(&mut e, MODEL_HAS_FILE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0052_4db0, &args![w, actor]);
        assert_eq!(logged(&mut e).len(), 4);
        returning(&mut e, MODEL_HAS_FILE, 1);
        map_pages(&mut e, &[0x011d_ea2a, PLAYER_CHARACTER]);
        e.set_global(0x011d_ea2a, 1u8);
        e.set_global(PLAYER_CHARACTER, 0x9999u32);
        e.call_log = Some(vec![]);
        e.call(0x0052_4db0, &args![w, actor]);
        assert!(!addresses(&logged(&mut e)).contains(&FIND_NODE_BY_NAME));
        e.set_global(0x011d_ea2a, 0u8);
        give_vtable(
            &mut e,
            actor,
            &[
                (REFERENCE_IS_ACTOR_SLOT, 0x00dd_0100),
                (REFERENCE_GET_3D_SLOT, 0x00dd_01d0),
            ],
        );
        returning(&mut e, 0x00dd_01d0, 0x5000);
        returning(&mut e, FIND_NODE_BY_NAME, 0);
        e.call_log = Some(vec![]);
        e.call(0x0052_4db0, &args![w, actor]);
        let log = logged(&mut e);
        assert_eq!(
            log.iter()
                .find(|(addr, _)| *addr == FIND_NODE_BY_NAME)
                .unwrap()
                .1,
            vec![0x5000, SHELL_CASING_NODE_NAME]
        );
        assert!(!addresses(&log).contains(&NODE_WORLD_TRANSLATION));
    }

    /// Everything a casing needs to be spawned for a non-player actor;
    /// returns the actor, the constructor log and the model path.
    fn casing_scene(e: &mut Engine, within_range: bool) -> (u32, Log, u32) {
        vector_helpers(e);
        inverse_sqrt_table(e);
        let actor = e.mem.alloc(0x100);
        give_vtable(
            e,
            actor,
            &[
                (REFERENCE_IS_ACTOR_SLOT, 0x00dd_0100),
                (REFERENCE_GET_3D_SLOT, 0x00dd_01d0),
            ],
        );
        returning(e, 0x00dd_0100, 1);
        returning(e, 0x00dd_01d0, 0x5000);
        // The model and its path (slot 0x14 of the model).
        let model = e.mem.alloc(0x20);
        let path = e.mem.alloc(16);
        e.mem.set_cstr(path, b"casing.nif");
        give_vtable(e, model, &[(MODEL_GET_MODEL_SLOT, 0x00dd_0014)]);
        returning(e, 0x00dd_0014, path);
        returning(e, GET_SHELL_CASING_MODEL, model);
        returning(e, MODEL_HAS_FILE, 1);
        map_pages(
            e,
            &[
                0x011d_ea2a,
                PLAYER_CHARACTER,
                CASING_RANGE,
                DEFAULT_DIRECTION,
                TWO,
            ],
        );
        e.set_global(PLAYER_CHARACTER, 0x9999u32);
        e.set_global(CASING_RANGE, 128.0f32);
        e.set_global(DEFAULT_DIRECTION, 0.0f32);
        e.set_global(DEFAULT_DIRECTION + 4, 0.0f32);
        e.set_global(DEFAULT_DIRECTION + 8, 1.0f32);
        returning(e, FIND_NODE_BY_NAME, 0x6000);
        // The camera object and the node positions.
        returning(e, STRING_C_STR, 0xC0);
        e.register(FOLDED_GETTER_006629F0, |_, _| Ret {
            eax: 0xC1,
            ..Ret::default()
        });
        let node_position = floats(e, &[10.0, 0.0, 0.0]);
        let camera_position = floats(e, &[9.0, 0.0, 0.0]);
        e.register_double(NODE_WORLD_TRANSLATION, move |_, a| Ret {
            eax: if a[0] == 0xC1 {
                camera_position
            } else {
                node_position
            },
            ..Ret::default()
        });
        returning(e, POINT_NEAR_NODE, within_range as u32);
        returning_float(e, VECTOR_LENGTH, 1.0);
        let rotation = floats(e, &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        returning(e, NODE_WORLD_ROTATION, rotation);
        e.register(MATRIX_GET_COLUMN, |e, a| {
            e.mem.set_f32(a[2], 0.0);
            e.mem.set_f32(a[2] + 4, 0.0);
            e.mem.set_f32(a[2] + 8, 1.0);
            Ret::default()
        });
        returning_float(e, RANDOM_FLOAT, 0.5);
        returning(e, MOBILE_GET_CHAR_CONTROLLER, 0);
        float_settings(
            e,
            &[
                (SETTING_GUN_SHELL_CAMERA_DISTANCE, 500.0),
                (SETTING_GUN_SHELL_DIRECTION_RANDOMIZE, 2.0),
                (SETTING_GUN_SHELL_EJECT_SPEED, 3.0),
                (SETTING_GUN_SHELL_ROTATE_RANDOMIZE, 0.5),
                (SETTING_GUN_SHELL_ROTATE_SPEED, 10.0),
                (SETTING_GUN_SHELL_LIFETIME, 4.0),
            ],
        );
        returning(e, OPERATOR_NEW, 0x8888);
        returning(e, ACTOR_WORD_40, 0x40);
        let debris: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = debris.clone();
        e.register_double(DEBRIS_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: 0x9000,
                ..Ret::default()
            }
        });
        (actor, debris, path)
    }

    #[test]
    fn casing_is_not_spawned_out_of_range() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let (actor, debris, _) = casing_scene(&mut e, false);
        e.call(0x0052_4db0, &args![w, actor]);
        assert!(debris.borrow().is_empty());
        // Nor beyond the range setting.
        let (actor, debris, _) = casing_scene(&mut e, true);
        returning_float(&mut e, VECTOR_LENGTH, 600.0);
        e.call(0x0052_4db0, &args![w, actor]);
        assert!(debris.borrow().is_empty());
    }

    #[test]
    fn casing_flies_along_the_nodes_rotation_with_a_spread() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let (actor, debris, path) = casing_scene(&mut e, true);
        e.call(0x0052_4db0, &args![w, actor]);
        let debris = debris.borrow();
        assert_eq!(debris.len(), 1);
        let call = &debris[0];
        assert_eq!(call.len(), 12);
        let (object, actor_word, lifetime, model_path, shooter) =
            (call[0], call[1], call[2], call[3], call[4]);
        assert_eq!(object, 0x8888);
        assert_eq!(actor_word, 0x40);
        // Not the player in first person: the setting as it is.
        assert_eq!(f32::from_bits(lifetime), 4.0);
        assert_eq!(model_path, path);
        assert_eq!(shooter, actor);
        // The node's position and its rotation (identity).
        assert_eq!(vec3(&e, call[5]), [10.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(call[6]), 1.0);
        // Eject velocity: the rotation column (0, 0, 1) plus the random
        // spread 0.5 each, times the eject speed 3.
        assert_eq!(vec3(&e, call[7]), [1.5, 1.5, 4.5]);
        // Spin: the default direction (0, 0, 1) scaled by 1.5 and normalized,
        // times the rotate speed 10, on the x and z axes.
        let spin = vec3(&e, call[8]);
        assert_eq!(spin[1], 0.0);
        assert!(spin[0].abs() < 1e-6);
        assert!((spin[2] - 10.0).abs() < 0.1, "{spin:?}");
        assert_eq!(f32::from_bits(call[9]), 1.0);
        assert_eq!(&call[10..], &[0, 1]);
    }

    #[test]
    fn casing_for_the_first_person_player_is_finished_and_has_a_capped_lifetime() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let (actor, debris, _) = casing_scene(&mut e, true);
        // The actor is the player in first person.
        e.set_global(PLAYER_CHARACTER, actor);
        returning(&mut e, PLAYER_GET_CURRENT_3D, 0x5000);
        returning(&mut e, PLAYER_BIPED_FLAG, 0);
        map_pages(&mut e, &[OBJECT_011F2250, TWO]);
        e.set_global(TWO, 2.0f32);
        e.register(MIN_FLOAT, |_, a| Ret {
            st0: f32::from_bits(a[0]).min(f32::from_bits(a[1])) as f64,
            ..Ret::default()
        });
        let finish = recording(&mut e, DEBRIS_FINISH);
        e.call(0x0052_4db0, &args![w, actor]);
        // The lifetime 4 is cut down to 2, and the casing is finished.
        assert_eq!(f32::from_bits(debris.borrow()[0][2]), 2.0);
        assert_eq!(finish.borrow().as_slice(), &[vec![0x9000, 1]]);
    }

    /// A `Fire` for the helper tests: a weapon, an actor with an items
    /// process, the frame, and the player global set to something else.
    fn fire_state(e: &mut Engine) -> (Fire, u32) {
        let this = weapon(e);
        let (actor, process) = actor_with_items(e, 0, 0);
        map_pages(e, &[PLAYER_CHARACTER]);
        e.set_global(PLAYER_CHARACTER, 0x9999u32);
        let frame = e.mem.alloc(FIRE_FRAME_SIZE);
        (
            Fire {
                this,
                reference: Ptr::new(actor),
                actor: Ptr::new(actor),
                player: 0x9999,
                is_player: false,
                process,
                held: 0,
                node: 0,
                frame,
            },
            frame,
        )
    }

    #[test]
    fn fire_ignores_a_null_reference() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0052_3150, &args![w, 0u32]);
        assert_eq!(logged(&mut e).len(), 1);
    }

    #[test]
    fn fire_stops_for_an_actor_with_the_blocking_word() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        map_pages(&mut e, &[PLAYER_CHARACTER]);
        e.set_global(PLAYER_CHARACTER, 0x9999u32);
        let actor = e.mem.alloc(0x200);
        give_vtable(&mut e, actor, &[(REFERENCE_IS_ACTOR_SLOT, 0x00dd_0100)]);
        returning(&mut e, 0x00dd_0100, 1);
        e.mem.set_u32(actor + 0x1A0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0052_3150, &args![w, actor]);
        assert_eq!(addresses(&logged(&mut e)), vec![0x0052_3150, 0x00dd_0100]);
    }

    #[test]
    fn ammo_effect_is_applied_when_the_roll_is_within_the_chance() {
        let mut e = Engine::new();
        let w = weapon(&mut e);
        let frame = e.mem.alloc(FIRE_FRAME_SIZE);
        let player = e.mem.alloc(0x100);
        give_vtable(&mut e, player, &[(PLAYER_APPLY_FORM_SLOT, 0x00dd_0190)]);
        let applied = recording(&mut e, 0x00dd_0190);
        let effect = e.mem.alloc(0x100);
        give_vtable(&mut e, effect, &[(EFFECT_CAN_APPLY_SLOT, 0x00dd_00e4)]);
        returning(&mut e, 0x00dd_00e4, 1);
        returning(&mut e, AMMO_GET_EFFECT, effect);
        returning_float(&mut e, AMMO_GET_EFFECT_CHANCE, 50.0);
        // The entry point doubles the chance.
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            let value = e.mem.f32(a[3]);
            e.mem.set_f32(a[3], value * 2.0);
            Ret::default()
        });
        quiet(&mut e, &[RT_DYNAMIC_CAST]);
        // 99 % 100 is 99, within the doubled chance of 100.
        returning(&mut e, RANDOM_INTEGER, 199);
        apply_ammo_effect(&mut e, w, 0x4000, player, frame);
        assert_eq!(applied.borrow().as_slice(), &[vec![player, effect, 0, 1]]);
        // A roll above the chance (50) does nothing.
        applied.borrow_mut().clear();
        returning(&mut e, RANDOM_INTEGER, 199);
        e.register(HANDLE_ENTRY_POINT, |_, _| Ret::default());
        apply_ammo_effect(&mut e, w, 0x4000, player, frame);
        assert!(applied.borrow().is_empty());
        // An effect that cannot be applied does nothing either.
        returning(&mut e, 0x00dd_00e4, 0);
        returning(&mut e, RANDOM_INTEGER, 0);
        apply_ammo_effect(&mut e, w, 0x4000, player, frame);
        assert!(applied.borrow().is_empty());
    }

    /// The doubles of the whole firing function for a shooter that is not
    /// an actor: a position, a fire node and no process.
    fn fire_scene(e: &mut Engine) -> (Ptr<TesObjectWeap>, u32, Log, Log) {
        let w = weapon(e);
        let data = weapon_data(w);
        e.set(data, ObjWeap::fMinSpread, 1.0);
        e.set(data, ObjWeap::fSpread, 2.0);
        e.set(data, ObjWeap::cNumProjectiles, 2);
        let reference = e.mem.alloc(0x40);
        give_vtable(
            e,
            reference,
            &[
                (REFERENCE_IS_ACTOR_SLOT, 0x00dd_0100),
                (REFERENCE_GET_3D_SLOT, 0x00dd_01d0),
            ],
        );
        returning(e, 0x00dd_0100, 0);
        returning(e, 0x00dd_01d0, 0x5000);
        let position = floats(e, &[1.0, 2.0, 3.0]);
        returning(e, REFERENCE_POSITION, position);
        let angles = floats(e, &[0.25, 0.0, 0.75]);
        returning(e, REFERENCE_ROTATION_ANGLES, angles);
        returning(e, GET_WEAPON_TYPE, 0);
        returning(e, WEAPON_FLAG_20_TEST, 0);
        returning(e, FIND_NODE_BY_NAME, 0x6000);
        let translation = floats(e, &[7.0, 8.0, 9.0]);
        returning(e, NODE_WORLD_TRANSLATION, translation);
        returning(e, VATS_GET_MODE, 0);
        returning(e, NODE_WORLD_ROTATION, translation);
        e.register(MATRIX_TO_EULER_ANGLES, |e, a| {
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[2], 2.0);
            e.mem.set_f32(a[3], 0.0);
            Ret::default()
        });
        let flag = e.mem.alloc(8);
        returning(e, SETTING_VALUE_POINTER, flag);
        let effects = recording(e, FIRE_EFFECTS);
        float_settings(e, &[(SETTING_GUN_REFERENCE_SKILL, 3.0)]);
        map_pages(
            e,
            &[
                PLAYER_CHARACTER,
                ZERO,
                DEGREES_TO_RADIANS,
                TWO_PI,
                VATS_ACTOR,
            ],
        );
        e.set_global(PLAYER_CHARACTER, 0x9999u32);
        e.set_global(DEGREES_TO_RADIANS, 0.5f64);
        e.set_global(TWO_PI, 6.0f32);
        returning(e, AMMO_IS_ROCKIT_LAUNCHER, 0);
        quiet(
            e,
            &[
                NI_POINTER_CONSTRUCT,
                NI_POINTER_ASSIGN,
                NI_POINTER_DESTRUCT,
                PROJECTILE_SET_FLAG,
            ],
        );
        returning(e, NI_POINTER_GET, 0x55);
        returning(e, AMMO_FORM_GET_AMMO, 0);
        returning(e, GET_PROJECTILE, 0x4444);
        returning_float(e, RANDOM_FLOAT, 0.5);
        returning_float(e, RANDOM_CONE_X, 0.5);
        returning_float(e, RANDOM_CONE_Z, 0.25);
        let created: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = created.clone();
        e.register_double(CREATE_PROJECTILE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: 0x7777,
                ..Ret::default()
            }
        });
        (w, reference, created, effects)
    }

    #[test]
    fn fire_creates_every_projectile_at_the_fire_node() {
        let mut e = Engine::new();
        let (w, reference, created, effects) = fire_scene(&mut e);
        let sets = recording(&mut e, PROJECTILE_SET_FLAG);
        e.call(0x0052_3150, &args![w, reference]);
        let created = created.borrow();
        assert_eq!(created.len(), 2);
        let expected = vec![
            0x4444,
            reference,
            0,
            w.addr(),
            7.0f32.to_bits(),
            8.0f32.to_bits(),
            9.0f32.to_bits(),
            // Z angle 1.0 and X angle 2.0 from the node's rotation, each
            // moved by the random cone (0.5 * 0.5 and 0.25 * 0.5).
            1.125f32.to_bits(),
            2.25f32.to_bits(),
            0x55,
            0,
            0,
            0,
            0.125f32.to_bits(),
            0.25f32.to_bits(),
            0,
        ];
        assert_eq!(created[0], expected);
        assert_eq!(created[1], expected);
        // The second projectile gets its flag cleared; the first does not.
        assert_eq!(sets.borrow().as_slice(), &[vec![0x7777, 0]]);
        assert_eq!(
            effects.borrow().as_slice(),
            &[vec![w.addr(), reference, 0x6000, 0]]
        );
    }

    #[test]
    fn fire_spread_is_the_weapon_spread_times_the_stat_in_radians() {
        let mut e = Engine::new();
        let (w, reference, _created, _) = fire_scene(&mut e);
        let randoms = Rc::new(RefCell::new(Vec::new()));
        let sink = randoms.clone();
        e.register_double(RANDOM_FLOAT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                st0: 0.5,
                ..Ret::default()
            }
        });
        e.call(0x0052_3150, &args![w, reference]);
        // (2.0 * 3.0 + 1.0) * 0.5 = 3.5: the first random is 0..spread.
        let randoms: std::cell::Ref<Vec<Vec<u32>>> = randoms.borrow();
        assert_eq!(randoms[0], vec![0.0f32.to_bits(), 3.5f32.to_bits()]);
        assert_eq!(randoms[1], vec![0.0f32.to_bits(), 6.0f32.to_bits()]);
    }

    #[test]
    fn fire_without_a_node_still_fires_from_the_position() {
        let mut e = Engine::new();
        let (w, reference, created, effects) = fire_scene(&mut e);
        returning(&mut e, FIND_NODE_BY_NAME, 0);
        e.call(0x0052_3150, &args![w, reference]);
        let created = created.borrow();
        assert_eq!(created.len(), 2);
        // The reference's position and angles; no muzzle effects.
        assert_eq!(
            &created[0][4..7],
            &[1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert!(effects.borrow().is_empty());
    }

    #[test]
    fn vats_shot_cost_counts_down_and_finishes_the_attack() {
        let mut e = Engine::new();
        let (shot, _) = fire_state(&mut e);
        e.set(weapon_data(shot.this), ObjWeap::cAmmoPerShot, 2);
        map_pages(&mut e, &[VATS_FIRE_ACCUMULATOR, DOUBLE_ONE]);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.set_global(VATS_FIRE_ACCUMULATOR, 0.25f32);
        float_settings(
            &mut e,
            &[
                (SETTING_VATS_SHOT_BURST_TIME, 10.0),
                (SETTING_VATS_SHOT_LONG_BURST_TIME, 4.0),
            ],
        );
        e.register(MAX_FLOAT, |_, a| Ret {
            st0: f32::from_bits(a[0]).max(f32::from_bits(a[1])) as f64,
            ..Ret::default()
        });
        let finished = recording(&mut e, VATS_FINISH_ATTACK);
        let action = e.mem.alloc(16);
        e.mem.set_u8(action + 8, 3);
        e.mem.set_u8(action + 9, 0);
        fire_vats_cost(&mut e, &shot, action, false, 0x7777);
        // 3 shots less 2 left; 10 / max(10, 4) = 1.0 added to 0.25, then
        // one whole shot dropped.
        assert_eq!(e.mem.u8(action + 8), 1);
        assert_eq!(e.global::<f32>(VATS_FIRE_ACCUMULATOR), 0.25);
        assert!(finished.borrow().is_empty());
        // The last shots: the attack is finished.
        e.mem.set_u8(action + 8, 2);
        fire_vats_cost(&mut e, &shot, action, false, 0x7778);
        assert_eq!(e.mem.u8(action + 8), 0);
        assert_eq!(finished.borrow().as_slice(), &[vec![VATS_OBJECT, 0x7778]]);
        // An action-point shot uses the other counter and does not finish.
        e.mem.set_u8(action + 9, 2);
        fire_vats_cost(&mut e, &shot, action, true, 0x7779);
        assert_eq!(e.mem.u8(action + 9), 1);
        assert_eq!(finished.borrow().len(), 1);
    }

    /// Doubles for aiming at a combat target from the muzzle at the origin.
    fn aim_doubles(e: &mut Engine) {
        e.register(VECTOR_SUBTRACT, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            Ret::default()
        });
        returning_float(e, Z_ANGLE_FROM_VECTOR, 0.5);
        returning_float(e, VECTOR_LENGTH, 10.0);
        e.register(NI_ASIN, |_, a| Ret {
            st0: (f32::from_bits(a[0]) as f64).asin(),
            ..Ret::default()
        });
        map_pages(e, &[DEGREES_TO_RADIANS, ZERO]);
        e.set_global(DEGREES_TO_RADIANS, 0.5f64);
        float_settings(e, &[(SETTING_NPC_MAX_GUN_WOBBLE_ANGLE, 0.5)]);
        returning(e, SETTING_VALUE_POINTER, 0x6000);
        map_pages(e, &[0x6000]);
    }

    #[test]
    fn aiming_at_a_combat_target_sets_the_angles_and_widens_the_spread() {
        let mut e = Engine::new();
        let (mut shot, frame) = fire_state(&mut e);
        aim_doubles(&mut e);
        give_vtable(
            &mut e,
            shot.actor.addr(),
            &[(ACTOR_COMBAT_TARGET_SLOT, 0x00dd_0428)],
        );
        returning(&mut e, 0x00dd_0428, 0x1111);
        // The target's firing point is (3, 4, 5) from the muzzle (0, 0, 0).
        e.register(TARGET_GET_FIRING_POINT, |e, a| {
            e.mem.set_f32(a[1], 3.0);
            e.mem.set_f32(a[1] + 4, 4.0);
            e.mem.set_f32(a[1] + 8, 5.0);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        returning_float(&mut e, ACTOR_GET_STAT, 2.0);
        let vtable = e.mem.u32(shot.process);
        e.mem.set_u32(vtable + PROCESS_RECOIL_SLOT, 0x00dd_042c);
        returning_float(&mut e, 0x00dd_042c, 0.25);
        let mut spread = 1.0f32;
        let target = fire_aim_at_target(&mut e, &mut shot, &mut spread);
        assert_eq!(target, Some(0x1111));
        // Z angle from the helper, X angle -asin(5 / 10).
        assert_eq!(e.mem.f32(frame + FIRE_ANGLE_Z), 0.5);
        assert_eq!(e.mem.f32(frame + FIRE_ANGLE_X), -(0.5f64.asin() as f32));
        // 1.0 + (2.0 * 0.5) * 0.5 + 0.25 recoil.
        assert_eq!(spread, 1.75);
    }

    #[test]
    fn aiming_at_a_target_without_a_firing_point_ends_the_shot() {
        let mut e = Engine::new();
        let (mut shot, _) = fire_state(&mut e);
        aim_doubles(&mut e);
        give_vtable(
            &mut e,
            shot.actor.addr(),
            &[
                (ACTOR_COMBAT_TARGET_SLOT, 0x00dd_0428),
                (ACTOR_CURRENT_ANIMATION_SLOT, 0x00dd_01e4),
            ],
        );
        returning(&mut e, 0x00dd_0428, 0x1111);
        returning(&mut e, 0x00dd_01e4, 0xA11);
        returning(&mut e, TARGET_GET_FIRING_POINT, 0);
        returning(&mut e, WEAPON_SHOULD_BLEND_OUT, 1);
        let blend = recording(&mut e, ANIMATION_BLEND_OUT);
        let mut spread = 1.0f32;
        assert_eq!(fire_aim_at_target(&mut e, &mut shot, &mut spread), None);
        assert_eq!(blend.borrow().as_slice(), &[vec![0xA11, 4, 0]]);
        // Without the blend-out the shot just ends.
        returning(&mut e, WEAPON_SHOULD_BLEND_OUT, 0);
        blend.borrow_mut().clear();
        assert_eq!(fire_aim_at_target(&mut e, &mut shot, &mut spread), None);
        assert!(blend.borrow().is_empty());
    }

    #[test]
    fn aiming_at_a_package_location_uses_the_projected_point() {
        let mut e = Engine::new();
        let (mut shot, frame) = fire_state(&mut e);
        aim_doubles(&mut e);
        give_vtable(
            &mut e,
            shot.actor.addr(),
            &[
                (ACTOR_COMBAT_TARGET_SLOT, 0x00dd_0428),
                (ACTOR_GET_ANGLE_SLOT, 0x00dd_02bc),
            ],
        );
        returning(&mut e, 0x00dd_0428, 0);
        let vtable = e.mem.u32(shot.process);
        for (slot, target) in [
            (PROCESS_PACKAGE_LOCATION_SLOT, 0x00dd_027c),
            (PROCESS_TARGET_HANDLE_SLOT, 0x00dd_0274),
            (PROCESS_RECOIL_SLOT, 0x00dd_042c),
        ] {
            e.mem.set_u32(vtable + slot, target);
        }
        returning(&mut e, 0x00dd_027c, 0x2222);
        returning(&mut e, 0x00dd_0274, 0x3333);
        returning_float(&mut e, 0x00dd_042c, 0.0);
        returning(&mut e, LOCATION_GET_TYPE, 0x10);
        returning(&mut e, HANDLE_RESOLVE, 0x4444);
        returning(&mut e, GET_PROJECTILE, 0);
        returning_float(&mut e, ACTOR_GET_STAT, 0.0);
        // The firing location for the resolved handle is (0, 6, 8).
        let location = floats(&mut e, &[0.0, 6.0, 8.0]);
        e.register_double(GET_TARGET_FIRING_LOCATION, move |_, _| Ret {
            eax: location,
            ..Ret::default()
        });
        // The package location is not the target: no override of the delta.
        returning(&mut e, PACKAGE_GET_LOCATION_REFERENCE, 0x5555);
        let mut spread = 1.0f32;
        let found = fire_aim_at_target(&mut e, &mut shot, &mut spread);
        assert_eq!(found, Some(0));
        assert_eq!(vec3(&e, frame + FIRE_TARGET_POINT), [0.0, 6.0, 8.0]);
        assert_eq!(
            e.mem.f32(frame + FIRE_ANGLE_X),
            (-(0.8f32 as f64).asin()) as f32
        );
        // A handle that does not resolve ends the shot.
        returning(&mut e, HANDLE_RESOLVE, 0);
        assert_eq!(fire_aim_at_target(&mut e, &mut shot, &mut spread), None);
    }

    #[test]
    fn aiming_at_the_package_reference_aims_along_the_heading() {
        let mut e = Engine::new();
        let (mut shot, frame) = fire_state(&mut e);
        aim_doubles(&mut e);
        vector_helpers(&mut e);
        e.register(VECTOR_ADD, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(VECTOR_SUBTRACT, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            Ret::default()
        });
        e.register(SINE_COSINE, |e, a| {
            e.mem.set_f32(a[1], 0.5);
            e.mem.set_f32(a[2], 0.25);
            Ret::default()
        });
        map_pages(&mut e, &[AIM_DISTANCE]);
        e.set_global(AIM_DISTANCE, 2.0f32);
        give_vtable(
            &mut e,
            shot.actor.addr(),
            &[
                (ACTOR_COMBAT_TARGET_SLOT, 0x00dd_0428),
                (ACTOR_GET_ANGLE_SLOT, 0x00dd_02bc),
            ],
        );
        returning(&mut e, 0x00dd_0428, 0);
        returning_float(&mut e, 0x00dd_02bc, 0.3);
        let vtable = e.mem.u32(shot.process);
        for (slot, target) in [
            (PROCESS_PACKAGE_LOCATION_SLOT, 0x00dd_027c),
            (PROCESS_TARGET_HANDLE_SLOT, 0x00dd_0274),
            (PROCESS_RECOIL_SLOT, 0x00dd_042c),
        ] {
            e.mem.set_u32(vtable + slot, target);
        }
        returning(&mut e, 0x00dd_027c, 0x2222);
        returning(&mut e, 0x00dd_0274, 0x3333);
        returning_float(&mut e, 0x00dd_042c, 0.0);
        returning(&mut e, LOCATION_GET_TYPE, 0x10);
        returning(&mut e, HANDLE_RESOLVE, 0x4444);
        returning(&mut e, GET_PROJECTILE, 0);
        returning_float(&mut e, ACTOR_GET_STAT, 0.0);
        let location = floats(&mut e, &[100.0, 100.0, 100.0]);
        returning(&mut e, GET_TARGET_FIRING_LOCATION, location);
        // The package location is the target itself: aim along the heading,
        // (sin, cos, 0) scaled by the aim distance, from the muzzle.
        returning(&mut e, PACKAGE_GET_LOCATION_REFERENCE, 0x4444);
        let mut spread = 1.0f32;
        fire_aim_at_target(&mut e, &mut shot, &mut spread);
        assert_eq!(vec3(&e, frame + FIRE_TARGET_POINT), [1.0, 0.5, 0.0]);
    }

    /// Doubles for the rumble: the strength and the two motor values.
    fn rumble_doubles(e: &mut Engine, strength: f64) -> Log {
        map_pages(
            e,
            &[
                CONTROLS_MANAGER_POINTER,
                DOUBLE_THOUSAND,
                HALF,
                ZERO,
                PLAYER_CHARACTER,
            ],
        );
        e.set_global(CONTROLS_MANAGER_POINTER, 0x1234u32);
        e.set_global(DOUBLE_THOUSAND, 1000.0f64);
        e.set_global(HALF, 0.5f32);
        returning(e, CONTROLS_MANAGER_GET, 0xC0DE);
        returning_float(e, WEAPON_RUMBLE_STRENGTH, strength);
        e.register(FTOL, |_, a| Ret {
            eax: f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32,
            ..Ret::default()
        });
        returning_float(e, WEAPON_RUMBLE_FLOAT_A, 0.25);
        returning_float(e, WEAPON_RUMBLE_FLOAT_B, 0.75);
        returning(e, IS_IN_MENU_MODE, 0);
        returning(e, AMMO_IS_ROCKIT_LAUNCHER, 0);
        recording(e, CONTROLS_RUMBLE)
    }

    #[test]
    fn rumble_runs_twice_for_the_player_with_a_weapon_item() {
        let mut e = Engine::new();
        let (mut shot, _) = fire_state(&mut e);
        let log = rumble_doubles(&mut e, 0.5);
        shot.reference = Ptr::new(0x9999);
        let (_, process) = actor_with_items(&mut e, 0xA1, 0xB1);
        shot.process = process;
        fire_rumble(&mut e, &shot);
        let half = 0.5f32.to_bits();
        assert_eq!(
            log.borrow().as_slice(),
            &[
                vec![
                    0xC0DE,
                    0.75f32.to_bits(),
                    0.25f32.to_bits(),
                    500,
                    0,
                    0,
                    0,
                    0
                ],
                vec![0xC0DE, half, 1.0f32.to_bits(), 0x14A, 0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn rumble_needs_strength_and_a_player_or_launcher_outside_menus() {
        let mut e = Engine::new();
        let (mut shot, _) = fire_state(&mut e);
        let log = rumble_doubles(&mut e, 0.0);
        shot.reference = Ptr::new(0x9999);
        let (_, process) = actor_with_items(&mut e, 0xA1, 0xB1);
        shot.process = process;
        fire_rumble(&mut e, &shot);
        assert!(log.borrow().is_empty(), "no strength, no rumble");
        returning_float(&mut e, WEAPON_RUMBLE_STRENGTH, 0.5);
        returning(&mut e, IS_IN_MENU_MODE, 1);
        fire_rumble(&mut e, &shot);
        assert!(log.borrow().is_empty(), "no rumble in menus");
        returning(&mut e, IS_IN_MENU_MODE, 0);
        // Another shooter, but a Rock-It launcher's ammo.
        shot.reference = Ptr::new(0x8888);
        fire_rumble(&mut e, &shot);
        assert!(log.borrow().is_empty());
        returning(&mut e, AMMO_IS_ROCKIT_LAUNCHER, 1);
        fire_rumble(&mut e, &shot);
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn after_the_shots_the_player_counter_wraps_and_the_actor_is_told() {
        let mut e = Engine::new();
        let (mut shot, _) = fire_state(&mut e);
        // The shooter is the player.
        e.set_global(PLAYER_CHARACTER, shot.actor.addr());
        shot.player = shot.actor.addr();
        let (_, process) = actor_with_items(&mut e, 0xA1, 0xB1);
        shot.process = process;
        let actor = shot.actor.addr();
        give_vtable(
            &mut e,
            actor,
            &[
                (ACTOR_SHOTS_FIRED_SLOT, 0x00dd_03f0),
                (ACTOR_IS_READY_TEST_SLOT, 0x00dd_0360),
            ],
        );
        let told = recording(&mut e, 0x00dd_03f0);
        returning(&mut e, 0x00dd_0360, 0);
        returning(&mut e, GET_WEAPON_TYPE, 0);
        returning(&mut e, ITEM_HAS_MOD_EFFECT_VALUE, 1);
        quiet(&mut e, &[NI_POINTER_ASSIGN, NI_POINTER_DESTRUCT]);
        // The counter lives in the player at +0x1F8: 2 + 3 shots = 5, over
        // 4, so it wraps to 1 and the actor is told 3 - 1 shots.
        e.mem.set_u8(actor + 0x1F8, 2);
        fire_after_shots(&mut e, &mut shot, 3);
        assert_eq!(e.mem.u8(actor + 0x1F8), 1);
        assert_eq!(told.borrow().as_slice(), &[vec![actor, 2]]);
    }

    #[test]
    fn after_the_shots_a_hand_held_weapon_without_ammo_is_removed() {
        let mut e = Engine::new();
        let (mut shot, _) = fire_state(&mut e);
        let actor = shot.actor.addr();
        give_vtable(
            &mut e,
            actor,
            &[
                (ACTOR_SHOTS_FIRED_SLOT, 0x00dd_03f0),
                (ACTOR_IS_READY_TEST_SLOT, 0x00dd_0360),
            ],
        );
        quiet(
            &mut e,
            &[0x00dd_03f0, NI_POINTER_ASSIGN, NI_POINTER_DESTRUCT],
        );
        returning(&mut e, 0x00dd_0360, 0);
        returning(&mut e, GET_WEAPON_TYPE, 11);
        let removed = recording(&mut e, ACTOR_REMOVE_WEAPON);
        // No ammo item: the weapon is removed.
        fire_after_shots(&mut e, &mut shot, 1);
        assert_eq!(removed.borrow().as_slice(), &[vec![actor]]);
        // With ammo items left (count 2) it is kept.
        let (_, process) = actor_with_items(&mut e, 0xA1, 0xB1);
        shot.process = process;
        returning(&mut e, HANDLE_RESOLVE, 2);
        removed.borrow_mut().clear();
        fire_after_shots(&mut e, &mut shot, 1);
        assert!(removed.borrow().is_empty());
    }

    /// `NiPointer` doubles that keep the pointer in the object they are
    /// given: construct and assign store it, get reads it, destruct clears.
    fn stateful_pointers(e: &mut Engine) {
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(NI_POINTER_DESTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
    }

    #[test]
    fn fire_node_of_a_held_weapon_comes_from_the_process() {
        let mut e = Engine::new();
        let (mut shot, frame) = fire_state(&mut e);
        let actor = shot.actor.addr();
        give_vtable(&mut e, actor, &[(ACTOR_GET_BIPED_SLOT, 0x00dd_01ec)]);
        returning(&mut e, 0x00dd_01ec, 0x321);
        let vtable = e.mem.u32(shot.process);
        e.mem
            .set_u32(vtable + PROCESS_GET_WEAPON_NODE_SLOT, 0x00dd_0190);
        e.register(0x00dd_0190, |_, a| Ret {
            eax: if a[1] == 0x321 { 0x6100 } else { 0 },
            ..Ret::default()
        });
        let translation = floats(&mut e, &[4.0, 5.0, 6.0]);
        returning(&mut e, NODE_WORLD_TRANSLATION, translation);
        fire_locate_held_weapon(&mut e, &mut shot);
        assert_eq!(shot.node, 0x6100);
        assert_eq!(vec3(&e, frame + FIRE_MUZZLE), [4.0, 5.0, 6.0]);
        // No node: the muzzle is left as it was.
        e.mem.set_f32(frame + FIRE_MUZZLE, 1.0);
        e.register(0x00dd_0190, |_, _| Ret::default());
        fire_locate_held_weapon(&mut e, &mut shot);
        assert_eq!(shot.node, 0);
        assert_eq!(e.mem.f32(frame + FIRE_MUZZLE), 1.0);
    }

    #[test]
    fn fire_in_iron_sights_starts_from_the_camera() {
        let mut e = Engine::new();
        let (mut shot, frame) = fire_state(&mut e);
        // The shooter is the player.
        let player = shot.actor.addr();
        e.set_global(PLAYER_CHARACTER, player);
        shot.player = player;
        shot.is_player = true;
        shot.node = 0x6000;
        give_vtable(&mut e, player, &[(ACTOR_GET_ANGLE_SLOT, 0x00dd_02bc)]);
        returning_float(&mut e, 0x00dd_02bc, 0.3);
        returning_float(&mut e, MOBILE_GET_LOOKING, 0.4);
        // The node position, the aim-down-sights state and a sight model.
        let node_position = floats(&mut e, &[7.0, 8.0, 9.0]);
        returning(&mut e, NODE_WORLD_TRANSLATION, node_position);
        returning(&mut e, VATS_GET_MODE, 0);
        returning(&mut e, ACTOR_GET_IRON_SIGHTS, 1);
        returning(&mut e, WEAPON_SIGHT_MODEL, 0x1234);
        returning(&mut e, MODEL_HAS_FILE, 1);
        returning(&mut e, WEAPON_FLAG_TEST_4AD030, 0);
        // The camera object and its transform: scale 1, translation 100 on x.
        returning(&mut e, STRING_C_STR, 0xC0);
        returning(&mut e, FOLDED_GETTER_006629F0, 0xC1);
        let transform = e.mem.alloc(0x40);
        e.mem.set_f32(transform + 0x30, 1.0);
        e.mem.set_f32(transform + 0x24, 100.0);
        returning(&mut e, NODE_WORLD_ROTATION, transform);
        // The static offset vector is built once, (0, -5, 0).
        map_pages(&mut e, &[SIGHT_OFFSET_GUARD, SIGHT_OFFSET_Y]);
        e.set_global(SIGHT_OFFSET_Y, -5.0f32);
        let constructed: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = constructed.clone();
        e.register_double(VECTOR_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret::default()
        });
        // The transform applied to the point: rotation, scale, translation.
        e.register(MATRIX_MULTIPLY_VECTOR, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(VECTOR_SCALED, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) * f32::from_bits(a[2]);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(VECTOR_ADD, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        returning(&mut e, SETTING_VALUE_POINTER, 0x6000);
        map_pages(&mut e, &[0x6000]);
        let effects = recording(&mut e, FIRE_EFFECTS);
        fire_from_node(&mut e, &mut shot);
        assert_eq!(vec3(&e, frame + FIRE_MUZZLE), [100.0, -5.0, 0.0]);
        assert_eq!(e.global::<u32>(SIGHT_OFFSET_GUARD) & 1, 1);
        assert_eq!(constructed.borrow().len(), 1);
        // The player's own view: the angles come from the player, not from
        // the rotation of the node.
        assert_eq!(e.mem.f32(frame + FIRE_ANGLE_Z), 0.3);
        assert_eq!(e.mem.f32(frame + FIRE_ANGLE_X), 0.4);
        assert_eq!(effects.borrow().len(), 1);
        // The offset is built only once.
        fire_from_node(&mut e, &mut shot);
        assert_eq!(constructed.borrow().len(), 1);
    }

    #[test]
    fn laser_sight_cylinder_is_created_once_under_the_parent_of_the_node() {
        let mut e = Engine::new();
        let (mut shot, _) = fire_state(&mut e);
        stateful_pointers(&mut e);
        // The parent of the node, found through the node's virtual slot 0xc.
        let parent = e.mem.alloc(0x40);
        let attached: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = attached.clone();
        e.register_double(0x00dd_00dc, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        give_vtable(
            &mut e,
            parent,
            &[
                (NODE_FIND_CHILD_SLOT, 0x00dd_009c),
                (NODE_ATTACH_CHILD_SLOT, 0x00dd_00dc),
            ],
        );
        returning(&mut e, 0x00dd_009c, 0);
        let node = e.mem.alloc(0x40);
        give_vtable(&mut e, node, &[(NODE_AS_NODE_SLOT, 0x00dd_000c)]);
        returning(&mut e, 0x00dd_000c, parent);
        shot.node = node;
        map_pages(&mut e, &[0x011c_621c, CYLINDER_LENGTH, HALF_PI]);
        e.set_global(0x011c_621c, 0x5151u32);
        e.set_global(CYLINDER_LENGTH, 4096.0f32);
        e.set_global(HALF_PI, 1.5f32);
        quiet(
            &mut e,
            &[
                NI_COLOR_CONSTRUCT,
                OBJECT_SET_NAME,
                SET_LOCAL_ROTATION,
                ADD_NO_LIGHTING,
                PREPARE_OBJECT,
                UPDATE_PROPERTIES,
                UPDATE_DATA_CONSTRUCT,
                NODE_UPDATE,
                SINE_COSINE,
            ],
        );
        e.register_double(CREATE_CYLINDER, |_, _| Ret {
            eax: 0x7000,
            ..Ret::default()
        });
        fire_debug_cylinder(&mut e, &shot);
        // The cylinder is attached to the parent with the flag 1.
        assert_eq!(attached.borrow().as_slice(), &[vec![parent, 0x7000, 1]]);
        // An existing child is not replaced.
        returning(&mut e, 0x00dd_009c, 0x7100);
        attached.borrow_mut().clear();
        fire_debug_cylinder(&mut e, &shot);
        assert!(attached.borrow().is_empty());
    }
}

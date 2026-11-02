//! `fallout shared/tescreature.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The float `fFootWeight` starts from (`01017718`, read with `FLD dword`).
const FOOT_WEIGHT_DEFAULT: u32 = 0x0101_7718;

layout! {
    /// `TESCreature` (Xbox PDB), 0x160 bytes on the PC (the PDB's 0x170 minus the
    /// 0x10 the PC `TESForm` loses). Only the fields used so far; names and
    /// types are the PDB's.
    pub struct TESCreature: 0x160 {
        /// `pSoundCreature` (Xbox PDB; shares its offset with `pCreatureSounds`).
        0x128 pSoundCreature: u32,
        /// `cHandReach` (Xbox PDB).
        0x130 cHandReach: u8,
        /// `fTurnSpeed` (Xbox PDB).
        0x134 fTurnSpeed: f32,
        /// `fBaseScale` (Xbox PDB).
        0x13c fBaseScale: f32,
        /// `pCombatStyle` (Xbox PDB).
        0x140 pCombatStyle: u32,
        /// `pBodyPartData` (Xbox PDB).
        0x144 pBodyPartData: u32,
        /// `eBloodImpactMaterial` (Xbox PDB).
        0x148 eBloodImpactMaterial: u32,
        /// `pImpactDataSet` (Xbox PDB).
        0x14c pImpactDataSet: u32,
        /// `bEmbeddedWeaponChecked` (Xbox PDB).
        0x150 bEmbeddedWeaponChecked: bool,
        /// `eSoundLevel` (Xbox PDB).
        0x154 eSoundLevel: u32,
        /// `pWeaponList` (Xbox PDB).
        0x158 pWeaponList: u32,
        /// `cCanUseHandToHand` (Xbox PDB).
        0x15c cCanUseHandToHand: u8,
    }
}

// Translated from 005f7230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f7230 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESCreature::TESCreature` (Xbox PDB): the form constructor
/// `CreateFormOfType` (`00465110`) runs for form type 0x2b (0x160 bytes).
/// `ECX` is the object. It constructs the `TESActorBase` base (`005f75e0`)
/// and the two components at `+0x10c` and `+0x114`, stores the vtable
/// pointers of the object and its components, registers the form type
/// (`004f15a0`), runs `005f7a40`, seeds the AI form at `+0x90` (`0x32`,
/// `1`, `2`, `2` through `0047ef50`, `0047ee00`, `0047eed0`, `0047ef00`),
/// and, when `004516b0` on the object at `011c3f2c` answers false, sets the
/// faction rank of the actor base data at `+0x30` to the object at
/// `011cb550` and `0` (`0047d800`).
pub fn tescreature_tescreature(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x005f_75e0, &args![this]);
    e.call(0x0047_fdc0, &args![base + 0x10c]);
    e.call(0x0048_9a60, &args![base + 0x114]);
    for (offset, vtable) in [
        (0x00, 0x0104_8f5c),
        (0x30, 0x0104_8eec),
        (0x64, 0x0104_8ed8),
        (0x70, 0x0104_8ec4),
        (0x7c, 0x0104_8ea4),
        (0x90, 0x0104_8e84),
        (0xb0, 0x0104_8e6c),
        (0xb8, 0x0104_8e58),
        (0xc4, 0x0104_8e44),
        (0xd0, 0x0104_8e30),
        (0xdc, 0x0104_8e0c),
        (0xf4, 0x0104_8df8),
        (0x100, 0x0104_8dc8),
        (0x104, 0x0104_8db4),
        (0x10c, 0x0104_8d9c),
        (0x114, 0x0104_8d88),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    e.call(0x004f_15a0, &args![this, 0x2bu32]);
    e.call(0x005f_7a40, &args![this]);
    e.call(0x0047_ef50, &args![base + 0x90, 0x32u32, 0u32]);
    e.call(0x0047_ee00, &args![base + 0x90, 1u32, 0u32]);
    e.call(0x0047_eed0, &args![base + 0x90, 2u32, 0u32]);
    e.call(0x0047_ef00, &args![base + 0x90, 2u32, 0u32]);
    let checked = e.global::<u32>(0x011c_3f2c);
    if !e.call(0x0045_16b0, &args![checked]).bool() {
        let faction = e.global::<u32>(0x011c_b550);
        e.call(0x0047_d800, &args![base + 0x30, faction, 0u32]);
    }
    this
}

/// `009dace0` (named `CombatProcedureUseCombatItem::InitLoadGame` in the engine map): the
/// base body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const INIT_LOAD_GAME_TARGET_009DACE0: u32 = 0x009d_ace0;

// Translated from 005fb430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable word 01048fcc): forwards `this` and one pointer to
/// `009dace0` (named `CombatProcedureUseCombatItem::InitLoadGame`; Xbox PDB
/// `?InitLoadGame@CombatProcedureUseCombatItem@@UAAXPAVBGSLoadFormBuffer@@@Z`, void), whose
/// body only calls the empty stub `004534f0` on the same two words. Returns nothing.
pub fn fn_005fb430(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(INIT_LOAD_GAME_TARGET_009DACE0, &args![this, buffer]);
}

// Translated from 005f75e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f75e0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESActorBase::TESActorBase` (Xbox PDB): the constructor `TESCreature`'s (`005f7230`) and
/// `TESNPC`'s constructors run first; `ECX` is the object. It constructs the base
/// (`0050f6f0`) and the components at `+0x30`, `+0x64`, `+0x70`, `+0x7c`, `+0x90`, `+0xb0`,
/// `+0xb8`, `+0xc4`, `+0xd0`, `+0xdc`, `+0xf4`, `+0x100` (`005f78e0`) and `+0x104`, then
/// stores the vtable pointers. The exception-unwinding frame is not translated.
pub fn tesactorbase_tesactorbase(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0047_c700, &args![base + 0x30]);
    e.call(0x0048_1610, &args![base + 0x64]);
    e.call(0x0047_bd10, &args![base + 0x70]);
    e.call(0x0048_cfe0, &args![base + 0x7c]);
    e.call(0x0047_ec40, &args![base + 0x90]);
    e.call(0x0048_71d0, &args![base + 0xb0]);
    e.call(0x0047_ff70, &args![base + 0xb8]);
    e.call(0x0047_f9c0, &args![base + 0xc4]);
    e.call(0x0040_2d00, &args![base + 0xd0]);
    e.call(0x0048_8f50, &args![base + 0xdc]);
    e.call(0x0048_cc50, &args![base + 0xf4]);
    fn_005f78e0(e, Ptr::new(base + 0x100));
    e.call(0x0047_5350, &args![base + 0x104]);
    for (offset, vtable) in [
        (0x00, 0x0104_92ac),
        (0x30, 0x0104_923c),
        (0x64, 0x0104_9224),
        (0x70, 0x0104_9210),
        (0x7c, 0x0104_91f0),
        (0x90, 0x0104_91d0),
        (0xb0, 0x0104_91b8),
        (0xb8, 0x0104_91a4),
        (0xc4, 0x0104_9190),
        (0xd0, 0x0104_917c),
        (0xdc, 0x0104_9158),
        (0xf4, 0x0104_9144),
        (0x100, 0x0104_9114),
        (0x104, 0x0104_9100),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    this
}

// Translated from 005f78e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the component at `+0x100` of `TESActorBase`: stores its base vtable
/// (`0x0104_9450`) and returns the object. The engine map gives it no name, so the class is
/// unconfirmed.
pub fn fn_005f78e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0104_9450);
    this
}

// Translated from 005f7b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit `0x200` of the actor base data's flags: calls
/// `TESActorBaseData::SetFlagBit` (`0047dd50`, `ECX` = `this`) with the bit `0x200`, the value
/// `clear == 0` and the second flag byte passed through. `ECX` is the actor base data
/// subobject. The engine map gives the function no name.
pub fn fn_005f7b50(e: &mut Engine, this: Ptr, clear: u8, notify: u8) {
    e.call(
        0x0047_dd50,
        &args![this, 0x200u32, u32::from(clear == 0), u32::from(notify)],
    );
}

// Translated from 005f7a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): the step of the
/// `TESCreature` constructor (`005f7230`) that gives the creature fields their
/// start values. `ECX` is the object. Field names are the Xbox PDB's, at the PC
/// offset (the PDB offset minus 0x10, which every field set here agrees with):
/// the four bytes at `+0x12c` (`Data`, `CREATURE_DATA`: 0, 0x32, 0x32, 0x32),
/// `cHandReach` 0x20, `fTurnSpeed` 0.0, `fFootWeight` the float at `01017718`
/// (copied through the x87 stack, so a signalling NaN would be quieted),
/// `fBaseScale` 1.0, `eBloodImpactMaterial` 6, `eSoundLevel` 1,
/// `cCanUseHandToHand` 0xff, the other pointers and flags 0. Then it sets bit
/// `0x40` of the actor base data at `+0x30` (`TESActorBaseData::SetFlagBit`,
/// `0047dd50`, with 0x40, 1, 0), runs `005f7b50` on it with 0 and 0, and runs
/// `006ecd40` on the component at `+0xb0` with 0x32.
pub fn fn_005f7a40(e: &mut Engine, this: Ptr<TESCreature>) {
    let base = this.addr();
    e.mem.set_u8(base + 0x12c, 0);
    e.mem.set_u8(base + 0x12d, 0x32);
    e.mem.set_u8(base + 0x12e, 0x32);
    e.mem.set_u8(base + 0x12f, 0x32);
    e.set(this, TESCreature::cHandReach, 0x20);
    e.set(this, TESCreature::pImpactDataSet, 0);
    e.set(this, TESCreature::pWeaponList, 0);
    e.set(this, TESCreature::fTurnSpeed, 0.0);
    let foot_weight = x87_quiet_bits(e.global::<u32>(FOOT_WEIGHT_DEFAULT));
    e.mem.set_u32(base + 0x138, foot_weight);
    e.set(this, TESCreature::fBaseScale, 1.0);
    e.set(this, TESCreature::pSoundCreature, 0);
    e.set(this, TESCreature::pCombatStyle, 0);
    e.set(this, TESCreature::pBodyPartData, 0);
    e.set(this, TESCreature::bEmbeddedWeaponChecked, false);
    e.set(this, TESCreature::eSoundLevel, 1);
    e.set(this, TESCreature::eBloodImpactMaterial, 6);
    e.set(this, TESCreature::cCanUseHandToHand, 0xff);
    e.call(0x0047_dd50, &args![base + 0x30, 0x40u32, 1u32, 0u32]);
    fn_005f7b50(e, Ptr::new(base + 0x30), 0, 0);
    e.call(0x006e_cd40, &args![base + 0xb0, 0x32u32]);
}

/// `00461580` (unnamed in the engine map): `thiscall`, `RET 4`; whether `this[4] & mask` is
/// non-zero (`SETNZ AL`).
const FORM_FLAGS_TEST: u32 = 0x0046_1580;
/// The offset, from `this`, of the object whose flags word (+4) is tested.
const FLAGS_OBJECT_OFFSET_30: u32 = 0x30;

// Translated from 005f90c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `__fastcall`: whether bit `0x1000000` is set in the flags word of
/// the object at `this + 0x30` (`00461580`); the result is the callee's `AL`, which this function
/// leaves in `EAX` unchanged (the decompiler shows it as `void`).
pub fn fn_005f90c0(e: &mut Engine, this: Ptr) -> bool {
    let flags_object = this.addr().wrapping_add(FLAGS_OBJECT_OFFSET_30);
    e.call(FORM_FLAGS_TEST, &args![flags_object, 0x0100_0000u32])
        .bool()
}

// Translated from 005f90e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `__fastcall`: whether bit `0x2000000` is set in the flags word of
/// the object at `this + 0x30` (`00461580`); like `005f90c0`, the callee's `AL` stays the result.
pub fn fn_005f90e0(e: &mut Engine, this: Ptr) -> bool {
    let flags_object = this.addr().wrapping_add(FLAGS_OBJECT_OFFSET_30);
    e.call(FORM_FLAGS_TEST, &args![flags_object, 0x0200_0000u32])
        .bool()
}

/// `0047dd50` (`TESActorBaseData::SetFlagBit`, Xbox PDB; thiscall, `RET 0xC`:
/// mask, value byte, flag byte).
const DATA_SET_FLAG_BIT: u32 = 0x0047_dd50;
/// `00461560` (thiscall on the actor base data): whether its level is a
/// multiple of the player's (the flag the level-multiplier code tests).
const DATA_PC_LEVEL_MULT: u32 = 0x0046_1560;
/// `0047ded0` (thiscall on the actor base data): its level (16-bit).
const DATA_LEVEL: u32 = 0x0047_ded0;
/// `00403e20` (thiscall on a `Setting`): the address of its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `00401000` (cdecl: size): allocates.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030` (cdecl: block): frees.
const FREE: u32 = 0x0040_1030;
/// The player singleton pointer (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// The `TES` singleton pointer (`011c3f2c`).
const TES: u32 = 0x011c_3f2c;
/// The save/load game singleton pointer (`011ddf38`).
const SAVE_LOAD_GAME: u32 = 0x011d_df38;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors used here.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_ACTOR: u32 = 0x0118_46d4;
const TYPE_MOBILE_OBJECT: u32 = 0x0118_4920;
const TYPE_BASE_PROCESS: u32 = 0x0118_a6b8;
const TYPE_MIDDLE_HIGH_PROCESS: u32 = 0x0118_c64c;
const TYPE_TES_CREATURE: u32 = 0x0118_3a00;
const TYPE_TES_COMBAT_STYLE: u32 = 0x0118_627c;
const TYPE_BGS_BODY_PART_DATA: u32 = 0x0118_61bc;
const TYPE_BGS_IMPACT_DATA_SET: u32 = 0x0118_62dc;
const TYPE_BGS_LIST_FORM: u32 = 0x0118_63d4;
/// `005b5e40` (cdecl, variadic): prints a formatted warning.
const PRINT_WARNING: u32 = 0x005b_5e40;
/// `0084e3a0` (thiscall on a form): its form id.
const FORM_ID: u32 = 0x0084_e3a0;
/// The vtable slot giving a form's name (byte offset 0x130).
const SLOT_NAME: u32 = 0x130;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`) and `TESForm::AddCompileIndex`
/// (`00485d50`), `004839c0` (the form with an id).
const FORM_GET_FILE: u32 = 0x0048_4e60;
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `00428110`/`004280f0`: the save buffer's flag test (`00428110` copies the
/// buffer's word at +0x17 into a local, `004280f0` tests bits of it).
const BUFFER_FLAGS: u32 = 0x0042_8110;
const FLAGS_TEST: u32 = 0x0042_80f0;
/// `_ftol2` (`00ec62c0`): ST0 to an integer.
const FTOL2: u32 = 0x00ec_62c0;

// Translated from 005f9040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flag 0x20000 of the actor base data `this` (`00461580`).
pub fn fn_005f9040(e: &mut Engine, this: Ptr) -> bool {
    e.call(FORM_FLAGS_TEST, &args![this, 0x0002_0000u32]).bool()
}

// Translated from 005f9060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flag 0x10000 of the actor base data `this` (`00461580`).
pub fn fn_005f9060(e: &mut Engine, this: Ptr) -> bool {
    e.call(FORM_FLAGS_TEST, &args![this, 0x0001_0000u32]).bool()
}

// Translated from 005f9080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flag 0x8000 of the actor base data `this` (`00461580`).
pub fn fn_005f9080(e: &mut Engine, this: Ptr) -> bool {
    e.call(FORM_FLAGS_TEST, &args![this, 0x8000u32]).bool()
}

// Translated from 005f90a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flag 0x80000 of the actor base data `this` (`00461580`).
pub fn fn_005f90a0(e: &mut Engine, this: Ptr) -> bool {
    e.call(FORM_FLAGS_TEST, &args![this, 0x0008_0000u32]).bool()
}

// Translated from 005fa2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in `Data.iCombat` (`CREATURE_DATA`, Xbox PDB; +0x12D; `RET 4`).
pub fn fn_005fa2c0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x12d, value);
}

// Translated from 005fbdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in `Data.eType` (+0x12C; `RET 4`).
pub fn fn_005fbdb0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x12c, value);
}

// Translated from 005fa310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `pBodyPartData` (+0x144; `RET 4`).
pub fn fn_005fa310(e: &mut Engine, this: Ptr<TESCreature>, value: u32) {
    e.set(this, TESCreature::pBodyPartData, value);
}

// Translated from 005fa2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pBodyPartData`, or the default body part data at `011cafec` when it is
/// null.
pub fn fn_005fa2e0(e: &mut Engine, this: Ptr<TESCreature>) -> u32 {
    if e.get(this, TESCreature::pBodyPartData) != 0 {
        e.get(this, TESCreature::pBodyPartData)
    } else {
        e.global::<u32>(0x011c_afec)
    }
}

// Translated from 005f7bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the 0x58-byte object at `pSoundCreature`
/// (made by `005f9210`): runs `005f0360`'s counterpart `005f0380` on `this`,
/// then frees it (`00401030`) when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005f7bf0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005f_0380, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 005f9100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x128 read as the sound creature: 0 when flag 0x100 of the
/// actor base data is set (the word then holds creature sounds), the word
/// otherwise.
pub fn fn_005f9100(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(
        FORM_FLAGS_TEST,
        &args![this.addr() + FLAGS_OBJECT_OFFSET_30, 0x100u32],
    )
    .bool()
    {
        0
    } else {
        e.mem.u32(this.addr() + 0x128)
    }
}

// Translated from 005f91d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x128 read as the creature sounds: the word when flag 0x100
/// of the actor base data is set, 0 otherwise.
pub fn fn_005f91d0(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(
        FORM_FLAGS_TEST,
        &args![this.addr() + FLAGS_OBJECT_OFFSET_30, 0x100u32],
    )
    .bool()
    {
        e.mem.u32(this.addr() + 0x128)
    } else {
        0
    }
}

// Translated from 005fbf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCreature::IsHumanoidCreature` (Xbox PDB): whether the creature type
/// (`0059f3a0`, a signed byte) is 3, 4, 5 or 7.
pub fn tes_creature_is_humanoid_creature(e: &mut Engine, this: Ptr) -> bool {
    let kind = e.call(0x0059_f3a0, &args![this]).u8() as i8;
    matches!(kind, 3 | 4 | 5 | 7)
}

// Translated from 005f7420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorValueOwner::GetPermanentActorFloatValue` (Xbox PDB name of this body,
/// `RET 4`: the actor value): vtable slot 0x04 (the base value) plus slot
/// 0x18 (the permanent modifier), the first stored as a double, the sum
/// `FADD`ed and stored as a float.
pub fn actor_value_owner_get_permanent_actor_float_value(
    e: &mut Engine,
    this: Ptr,
    value: u32,
) -> f32 {
    let base = e.vcall(this.addr(), 0x4, &args![value]).f64();
    let modifier = e.vcall(this.addr(), 0x18, &args![value]).f64();
    f32::from_bits(x87_store32(x87_add(modifier, base)))
}

/// The level multiplier (`0047ded0` on the actor base data `data`, at least
/// 1) when `00461560` says the level scales, else `None`.
fn level_multiplier(e: &mut Engine, data: u32) -> Option<i16> {
    if !e.call(DATA_PC_LEVEL_MULT, &args![data]).bool() {
        return None;
    }
    let mut level = e.call(DATA_LEVEL, &args![data]).u16() as i16;
    if (level as i32) < 1 {
        level = 1;
    }
    Some(level)
}

// Translated from 005f8f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fatigue of the actor base data `this` (`TESActorBaseData::GetFatigue`,
/// `004a8ae0`), times the level multiplier when the level scales (16-bit).
pub fn fn_005f8f10(e: &mut Engine, this: Ptr) -> u16 {
    let mut fatigue = e.call(0x004a_8ae0, &args![this]).u16();
    if let Some(level) = level_multiplier(e, this.addr()) {
        fatigue = (i32::from(level).wrapping_mul(i32::from(fatigue))) as u16;
    }
    fatigue
}

// Translated from 005f8e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The health of the `TESHealthForm` part `this` (`00726070`: its word at
/// +4), times the level multiplier of the actor base data at `this - 0x80`
/// when the level scales (16-bit, zero-extended).
pub fn fn_005f8e90(e: &mut Engine, this: Ptr) -> u32 {
    let data = this.addr().wrapping_sub(0x80);
    let health = if let Some(level) = level_multiplier(e, data) {
        let base = e.call(0x0072_6070, &args![this]).u32();
        (i32::from(level).wrapping_mul(base as i32)) as u16
    } else {
        e.call(0x0072_6070, &args![this]).u16()
    };
    u32::from(health)
}

// Translated from 005f8f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The creature's combat skill: `Data.iCombat` (+0x12D), plus, when the
/// level scales, the low byte of the level times the setting at `011cb5b4`
/// (`FILD`, `FMUL`, truncated by `FISTP`).
pub fn fn_005f8f70(e: &mut Engine, this: Ptr) -> u8 {
    let mut skill = e.mem.u8(this.addr() + 0x12d);
    if let Some(level) = level_multiplier(e, this.addr() + 0x30) {
        let level = f64::from(i32::from(level));
        let factor = e.call(SETTING_VALUE, &args![0x011c_b5b4u32]).u32();
        let product = x87_mul(x87_load32(e.mem.u32(factor)), level);
        let bonus = crate::x87::fistp_chop(product) as u8;
        skill = skill.wrapping_add(bonus);
    }
    skill
}

// Translated from 005f8d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCreature::GetAttackDamage` (Xbox PDB name; `this` is the
/// `TESAttackDamageForm` part at +0x10C): 0 when `cCanUseHandToHand` (this
/// + 0x50) is 0. While it is still unknown (-1), and the reconstruct map at
/// `011cb6a0` is ready (`00843150`) and the `TES` flag `004516b0` is clear,
/// it becomes whether the idle manager has a root file list
/// (`TESIdleManager::GetRootFilenameList`) for the model (slot 0x14 of the
/// `TESModel` part at this - 0x30, with 1) holding an item. Otherwise the
/// part's damage (`TESAttackDamageForm::GetAttackDamage`), plus, when the
/// level scales, the level times the setting at `011cb58c` (`_ftol2`).
pub fn tes_creature_get_attack_damage(e: &mut Engine, this: Ptr) -> u16 {
    let base = this.addr();
    let map = e.global::<u32>(0x011c_b6a0);
    if e.call(0x0084_3150, &args![map]).bool() {
        let tes = e.global::<u32>(TES);
        if !e.call(0x0045_16b0, &args![tes]).bool() && e.mem.u8(base + 0x50) as i8 == -1 {
            // 005f8dc0: the model's +0x14 (`00408da0`, plain RET) takes no stack word; the
            // pushed 1 is the last word of `00600700` (RET 8).
            let model = e.vcall(base.wrapping_sub(0x30), 0x14, &args![]).u32();
            let map = e.global::<u32>(0x011c_b6a0);
            let list = e.call(0x0060_0700, &args![map, model, 1u32]).u32();
            let empty = list == 0 || {
                let item = e.call(0x0068_15c0, &args![list]).u32();
                e.mem.u32(item) == 0
            };
            e.mem.set_u8(base + 0x50, u8::from(!empty));
        }
    }
    if e.mem.u8(base + 0x50) as i8 == 0 {
        return 0;
    }
    let data = base.wrapping_sub(0xdc);
    if let Some(level) = level_multiplier(e, data) {
        let damage = e.call(0x0047_ff30, &args![this]).u16();
        let level = f64::from(i32::from(level));
        let factor = e.call(SETTING_VALUE, &args![0x011c_b58cu32]).u32();
        let product = x87_mul(x87_load32(e.mem.u32(factor)), level);
        let bonus = e.call(FTOL2, &args![product]).u32();
        (u32::from(damage).wrapping_add(bonus)) as u16
    } else {
        e.call(0x0047_ff30, &args![this]).u16()
    }
}

// Translated from 005fa120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the creature may wear `form` (`RET 4`): form types 0x1E, 0x28 and
/// 0x29, and (checked again) a type-0x28 form `0046e8c0` accepts.
pub fn fn_005fa120(e: &mut Engine, _this: Ptr, form: u32) -> bool {
    let kind = e.call(0x0040_1170, &args![form]).i32();
    if kind == 0x1e || (kind > 0x27 && kind <= 0x29) {
        return true;
    }
    e.call(0x0040_1170, &args![form]).u32() == 0x28 && e.call(0x0046_e8c0, &args![form]).bool()
}

// Translated from 005f7b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the creature sounds at +0x128 when flag 0x100 of the actor base data
/// says it holds them (the deleting destructor `005f7bf0` with 1) and clears
/// the word; then `cCanUseHandToHand` goes back to -1.
pub fn fn_005f7b80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(
        FORM_FLAGS_TEST,
        &args![base + FLAGS_OBJECT_OFFSET_30, 0x100u32],
    )
    .bool()
    {
        let sounds = e.mem.u32(base + 0x128);
        if sounds != 0 {
            fn_005f7bf0(e, Ptr::new(sounds), 1);
        }
        e.mem.set_u32(base + 0x128, 0);
    }
    e.mem.set_u8(base + 0x15c, 0xff);
}

// Translated from 005f92d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCreature::PickCreatureSound` (Xbox PDB, `RET 4`: the sound type): the
/// creature's own sounds (`005f91d0`) pick one (`005f0a30`); a creature that
/// borrows another's sounds (`005f9100`) asks that one; 0 without either.
pub fn tes_creature_pick_creature_sound(e: &mut Engine, this: Ptr, kind: u32) -> u32 {
    if e.call(
        FORM_FLAGS_TEST,
        &args![this.addr() + FLAGS_OBJECT_OFFSET_30, 0x100u32],
    )
    .bool()
    {
        let sounds = fn_005f91d0(e, this);
        if sounds != 0 {
            return e.call(0x005f_0a30, &args![sounds, kind]).u32();
        }
    } else {
        let other = fn_005f9100(e, this);
        if other != 0 {
            return tes_creature_pick_creature_sound(e, Ptr::new(other), kind);
        }
    }
    0
}

// Translated from 005fb330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the creature (`RET 4`: the save buffer): the actor base
/// (`005f1f30`), then, when the buffer's flag 0x200 is set, the bytes
/// `Data.iCombat`, `Data.iOther` and `Data.iStealth` (+0x12D..+0x12F) one by
/// one (`00865e50` with size 1 and 0).
pub fn fn_005fb330(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    e.call(0x005f_1f30, &args![this, buffer]);
    let saves = e.with_stack(4, |e, local| {
        let flags = e.call(BUFFER_FLAGS, &args![buffer, local]).u32();
        e.call(FLAGS_TEST, &args![flags, 0x200u32]).bool()
    });
    if saves {
        for offset in [0x12d, 0x12e, 0x12f] {
            e.call(0x0086_5e50, &args![buffer, base + offset, 1u32, 0u32]);
        }
    }
}

// Translated from 005fa0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCreature::InitWorn` (Xbox PDB, `RET 4`: the reference): when the
/// reference is a `MobileObject` with a process whose cast to
/// `MiddleHighProcess` succeeds, that process's vtable slot 0x544 runs with
/// the reference.
pub fn tes_creature_init_worn(e: &mut Engine, _this: Ptr, reference: u32) {
    let mobile = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                reference,
                0u32,
                TYPE_TES_OBJECT_REFR,
                TYPE_MOBILE_OBJECT,
                0u32
            ],
        )
        .u32();
    if mobile == 0 || e.call(0x008d_8520, &args![mobile]).u32() == 0 {
        return;
    }
    let process = e.call(0x008d_8520, &args![mobile]).u32();
    let high = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                process,
                0u32,
                TYPE_BASE_PROCESS,
                TYPE_MIDDLE_HIGH_PROCESS,
                0u32
            ],
        )
        .u32();
    if high != 0 {
        e.vcall(high, 0x544, &args![mobile]);
    }
}

// Translated from 005fa180 (decompiled, FalloutNV.exe 1.4.0.525)
/// The creature's actor value `index` (`RET 4`; `this` is the
/// `ActorValueOwner` part at +0x100): a skill (`00406d70` with 0x400 says
/// so) is the combat skill (`005f8f70` on the creature); 0x38 the attack
/// damage (slot 0x10 of the `TESAttackDamageForm` part at this + 0xC,
/// 16-bit); anything else the actor base's (`005f0fb0`).
pub fn fn_005fa180(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if e.call(0x0040_6d70, &args![index, 0x400u32]).bool() {
        return u32::from(fn_005f8f70(e, Ptr::new(this.addr().wrapping_sub(0x100))));
    }
    if index == 0x38 {
        return u32::from(e.vcall(this.addr() + 0xc, 0x10, &[]).u16());
    }
    e.call(0x005f_0fb0, &args![this, index]).u32()
}

// Translated from 005f9140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the sound creature (`RET 4`). When the word at +0x128 holds the
/// creature's own sounds (flag 0x100) and `sound` is not null, the sounds are
/// freed (`005f7bf0` with 1), flag 0x100 is cleared
/// (`TESActorBaseData::SetFlagBit` with 0x100, 0, 1) and the word set; a null
/// `sound` changes nothing then. Otherwise the word is set.
pub fn fn_005f9140(e: &mut Engine, this: Ptr, sound: u32) {
    let base = this.addr();
    if e.call(
        FORM_FLAGS_TEST,
        &args![base + FLAGS_OBJECT_OFFSET_30, 0x100u32],
    )
    .bool()
    {
        if sound != 0 {
            let sounds = e.mem.u32(base + 0x128);
            if sounds != 0 {
                fn_005f7bf0(e, Ptr::new(sounds), 1);
            }
            e.call(DATA_SET_FLAG_BIT, &args![base + 0x30, 0x100u32, 0u32, 1u32]);
            e.mem.set_u32(base + 0x128, sound);
        }
    } else {
        e.mem.set_u32(base + 0x128, sound);
    }
}

// Translated from 005f9210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f9210 - the exception-unwinding frame (FS:[0], frees the block if its constructor throws) is not translated.
/// The creature's own sounds at +0x128, made when missing: unless flag 0x100
/// is set and the word is not null, sets flag 0x100
/// (`TESActorBaseData::SetFlagBit` with 0x100, 1, 1) and stores a new
/// 0x58-byte block constructed by `005f0360` (null if the allocation fails).
/// Returns the word.
pub fn fn_005f9210(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let has = e
        .call(
            FORM_FLAGS_TEST,
            &args![base + FLAGS_OBJECT_OFFSET_30, 0x100u32],
        )
        .bool();
    if !has || e.mem.u32(base + 0x128) == 0 {
        e.call(DATA_SET_FLAG_BIT, &args![base + 0x30, 0x100u32, 1u32, 1u32]);
        let block = e.call(ALLOCATE, &args![0x58u32]).u32();
        let sounds = if block != 0 {
            e.call(0x005f_0360, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(base + 0x128, sounds);
    }
    e.mem.u32(base + 0x128)
}

// Translated from 005f77b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f77b0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `TESActorBase` destructor body: destroys the components in reverse
/// order (+0x104 `00475380`, +0xDC `00489070`, +0xD0 `00591300`, +0xC4
/// `0047f9f0`, +0xB8 `0047ffa0`, +0xB0 `00487200`, +0x90 `0047ecb0`, +0x7C
/// `0048d060`, +0x64 `00481680`, +0x30 `0047c880`), then the base
/// (`0050f740`).
pub fn fn_005f77b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, destructor) in [
        (0x104u32, 0x0047_5380u32),
        (0xdc, 0x0048_9070),
        (0xd0, 0x0059_1300),
        (0xc4, 0x0047_f9f0),
        (0xb8, 0x0047_ffa0),
        (0xb0, 0x0048_7200),
        (0x90, 0x0047_ecb0),
        (0x7c, 0x0048_d060),
        (0x64, 0x0048_1680),
        (0x30, 0x0047_c880),
    ] {
        e.call(destructor, &args![base + offset]);
    }
    e.call(0x0050_f740, &args![this]);
}

// Translated from 005fbdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Once per creature (`bEmbeddedWeaponChecked`, +0x150; `RET 4`: the biped
/// data or null, nothing when null): removes from the container (+0x64) each
/// type-0x28 item `0046e8c0` accepts whose `00522c00` value `004aae30`
/// answers 0 for (`00481790`, then the walk starts over), then sets the flag.
pub fn fn_005fbdd0(e: &mut Engine, this: Ptr, biped: u32) {
    let base = this.addr();
    if e.mem.u8(base + 0x150) != 0 || biped == 0 {
        return;
    }
    let mut node = e.call(0x0071_7e50, &args![base + 0x64]).u32();
    while node != 0 {
        let item = e.call(0x0068_15c0, &args![node]).u32();
        let entry = e.mem.u32(item);
        let mut removed = false;
        if entry != 0 {
            let form = e.mem.u32(entry + 4);
            if e.call(0x0040_1170, &args![form]).u32() == 0x28 {
                let weapon = e.mem.u32(entry + 4);
                if e.call(0x0046_e8c0, &args![weapon]).bool() {
                    let value = e.call(0x0052_2c00, &args![weapon]).u32();
                    if e.call(0x004a_ae30, &args![biped, value]).u32() == 0 {
                        e.call(0x0048_1790, &args![base + 0x64, entry]);
                        node = e.call(0x0071_7e50, &args![base + 0x64]).u32();
                        removed = true;
                    }
                }
            }
        }
        if !removed {
            node = e.call(0x0072_6070, &args![node]).u32();
        }
    }
    e.mem.set_u8(base + 0x150, 1);
}

// Translated from 005f7900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f7900 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `TESCreature` destructor body: stores the creature's vtables (as its
/// constructor `005f7230` does), frees the creature sounds (`005f7b80`),
/// destroys the `TESModelList` (+0x114, `00489ae0`) and
/// `TESAttackDamageForm` (+0x10C, `0047fdf0`) parts, then runs the
/// `TESActorBase` destructor body (`005f77b0`).
pub fn fn_005f7900(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, vtable) in [
        (0x00, 0x0104_8f5c),
        (0x30, 0x0104_8eec),
        (0x64, 0x0104_8ed8),
        (0x70, 0x0104_8ec4),
        (0x7c, 0x0104_8ea4),
        (0x90, 0x0104_8e84),
        (0xb0, 0x0104_8e6c),
        (0xb8, 0x0104_8e58),
        (0xc4, 0x0104_8e44),
        (0xd0, 0x0104_8e30),
        (0xdc, 0x0104_8e0c),
        (0xf4, 0x0104_8df8),
        (0x100, 0x0104_8dc8),
        (0x104, 0x0104_8db4),
        (0x10c, 0x0104_8d9c),
        (0x114, 0x0104_8d88),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    fn_005f7b80(e, this);
    e.call(0x0048_9ae0, &args![base + 0x114]);
    e.call(0x0047_fdf0, &args![base + 0x10c]);
    fn_005f77b0(e, this);
}

// Translated from 005f9e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCreature::InitDefaultWorn` (Xbox PDB, `RET 0x10`: the reference, a word
/// it does not read, an "equip" byte and a "keep the save flag" byte).
/// Without the keep byte the save/load game's flag (`004623f0`) is cleared
/// first and its old value put back at the end. A reference whose slot 0x100
/// says so drops its weapon (`TESObjectREFR::RemoveWeapon`); its inventory
/// changes (`InventoryChanges::GetInventoryChanges`) unequip everything
/// (`RemoveAllObjectsWorn` with 1); an actor's process runs slots 0x168,
/// 0x16C and 0x164 (with 0) and 0x160 (0, 0, 0). With the equip byte the
/// best item (`004c7400` with the creature, a float, 6, 1) is equipped
/// through the reference's slot 0x184 (form, 1, 0, 0) when the creature may
/// wear it (`005fa120`) and the reference does not already (`00575400`);
/// the second item of that code is never set here (its branch stays
/// translated). The item entries are freed (`004459e0` with 1). At the end
/// an actor with a process casts its permanent magic
/// (`Actor::CastPermanentMagic` with 1).
pub fn tes_creature_init_default_worn(
    e: &mut Engine,
    this: Ptr,
    reference: u32,
    _unused_2: u32,
    equip: u8,
    keep: u8,
) {
    let mut saved = 1u8;
    if keep == 0 {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        saved = e.call(0x0046_23f0, &args![game, 0u32]).u8();
    }
    if e.vcall(reference, 0x100, &[]).bool() {
        e.call(0x0057_1b50, &args![reference]);
    }
    let changes = e.call(0x004b_f220, &args![reference]).u32();
    e.call(0x004b_fe50, &args![changes, 1u32, reference]);
    let actor = e
        .call(
            RT_DYNAMIC_CAST,
            &args![reference, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
        )
        .u32();
    if actor != 0 && e.call(0x008d_8520, &args![actor]).u32() != 0 {
        let process = e.call(0x008d_8520, &args![actor]).u32();
        e.vcall(process, 0x168, &args![0u32]);
        let process = e.call(0x008d_8520, &args![actor]).u32();
        e.vcall(process, 0x16c, &args![0u32]);
        let process = e.call(0x008d_8520, &args![actor]).u32();
        e.vcall(process, 0x164, &args![0u32]);
        let process = e.call(0x008d_8520, &args![actor]).u32();
        e.vcall(process, 0x160, &args![0u32, 0u32, 0u32]);
    }
    let mut item = 0u32;
    // The second entry is initialised to null and never set again.
    let other = 0u32;
    e.with_stack(4, |e, score| {
        e.mem.set_u32(score.addr(), 0.0f32.to_bits());
        if equip != 0 && changes != 0 {
            item = e
                .call(0x004c_7400, &args![changes, this, score, 6u32, 1u32])
                .u32();
        }
    });
    if item != 0 {
        let form = e.call(0x0044_ddc0, &args![item]).u32();
        if fn_005fa120(e, this, form) {
            let form = e.call(0x0044_ddc0, &args![item]).u32();
            if !e.call(0x0057_5400, &args![reference, form]).bool() {
                let form = e.call(0x0044_ddc0, &args![item]).u32();
                e.vcall(reference, 0x184, &args![form, 1u32, 0u32, 0u32]);
                if other != 0 {
                    let form = e.call(0x0044_ddc0, &args![other]).u32();
                    if !e.call(0x0057_5400, &args![reference, form]).bool() {
                        let mut extra = 0u32;
                        if e.call(0x0055_9450, &args![other]).u32() != 0 {
                            let list = e.call(0x0055_9450, &args![other]).u32();
                            let first = e.call(0x0068_15c0, &args![list]).u32();
                            extra = e.mem.u32(first);
                        }
                        let count = e.call(0x0072_6070, &args![other]).u32();
                        let form = e.call(0x0044_ddc0, &args![other]).u32();
                        e.vcall(reference, 0x184, &args![form, count, extra, 0u32]);
                    }
                }
            }
        }
    }
    if item != 0 {
        e.call(0x0044_59e0, &args![item, 1u32]);
    }
    if other != 0 {
        e.call(0x0044_59e0, &args![other, 1u32]);
    }
    if keep == 0 {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(0x0046_23f0, &args![game, u32::from(saved)]);
    }
    if actor != 0 && e.call(0x008d_8520, &args![actor]).u32() != 0 {
        e.call(0x008c_26e0, &args![actor, 1u32]);
    }
}

/// Resolves the form id in the word at `field` (the creature's own file's
/// compile index, `004839c0`, cast to `target`) and stores the result there;
/// returns `(id, result)` with the id after the compile index.
fn resolve_field(e: &mut Engine, this: Ptr, field: u32, id: u32, target: u32) -> (u32, u32) {
    let id = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), id);
        let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
        e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
        e.mem.u32(slot.addr())
    });
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    let resolved = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
        )
        .u32();
    e.mem.set_u32(this.addr() + field, resolved);
    (id, resolved)
}

// Translated from 005f9340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCreature::InitItem` (Xbox PDB): nothing when form flag 0x8 is set.
/// Initialises the components (`TESScriptableForm`, `TESContainer`,
/// `TESSpellList`, `BGSTouchSpellForm`, `TESAIForm`, `TESActorBaseData`,
/// `BGSDestructibleObjectForm` `InitItem`s with the creature); when bit 0 of
/// the bound data word at +0x34 is set clears its bits 0x40, 0x10, 0x20,
/// 0x40000 and 0x800000. Then resolves the form ids, each with a
/// `MASTERFILE` warning when missing: the sound creature at +0x128 (when it
/// does not hold the creature's own sounds; a sound creature that is the
/// creature itself or has its own sound creature (`0059bb30` on its actor
/// base data) is dropped with a warning), the combat style (slot 0x188, kept
/// at +0x140; slot 0x18C gets `00505000` when there is none), the body part
/// data (+0x144), the impact data set (+0x14C) and the weapon list (+0x158).
/// A creature without health starts dead (`TESActorBase::SetStartsDead`).
/// Finally form flag bit 1 is set (`00484ab0` with 1).
pub fn tes_creature_init_item(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    e.call(0x0048_cd90, &args![base + 0xf4, this]);
    e.call(0x0048_1bb0, &args![base + 0x64, this]);
    e.call(0x0048_d400, &args![base + 0x7c, this]);
    e.call(0x0047_beb0, &args![base + 0x70, this]);
    e.call(0x0047_f4c0, &args![base + 0x90, this]);
    e.call(0x0047_d8b0, &args![base + 0x30, this]);
    e.call(0x0047_8be0, &args![base + 0x104, this]);
    if e.mem.u32(base + 0x34) & 1 != 0 {
        for mask in [
            0xffff_ffbfu32,
            0xffff_ffef,
            0xffff_ffdf,
            0xfffb_ffff,
            0xff7f_ffff,
        ] {
            let word = e.mem.u32(base + 0x34) & mask;
            e.mem.set_u32(base + 0x34, word);
        }
    }
    if !e
        .call(
            FORM_FLAGS_TEST,
            &args![base + FLAGS_OBJECT_OFFSET_30, 0x100u32],
        )
        .bool()
    {
        let id = e.mem.u32(base + 0x128);
        if id != 0 {
            let (id, sound) = resolve_field(e, this, 0x128, id, TYPE_TES_CREATURE);
            if sound == 0 {
                let name = e.vcall(base, SLOT_NAME, &[]).u32();
                let own_id = e.call(FORM_ID, &args![this]).u32();
                e.call(PRINT_WARNING, &args![0x0104_95e0u32, id, own_id, name]);
            } else if sound == base || e.call(0x0059_bb30, &args![sound + 0x30]).u32() != 0 {
                let own_id = e.call(FORM_ID, &args![this]).u32();
                let name = e.vcall(base, SLOT_NAME, &[]).u32();
                let sound = e.mem.u32(base + 0x128);
                let sound_id = e.call(FORM_ID, &args![sound]).u32();
                let sound = e.mem.u32(base + 0x128);
                let sound_name = e.vcall(sound, SLOT_NAME, &[]).u32();
                e.call(
                    PRINT_WARNING,
                    &args![0x0104_9598u32, sound_name, sound_id, name, own_id],
                );
                e.mem.set_u32(base + 0x128, 0);
            }
        }
    }
    if e.mem.u32(base + 0x140) != 0 {
        let id = e.vcall(base, 0x188, &[]).u32();
        if id != 0 {
            let (id, style) = resolve_field(e, this, 0x140, id, TYPE_TES_COMBAT_STYLE);
            if style == 0 {
                let name = e.vcall(base, SLOT_NAME, &[]).u32();
                let own_id = e.call(FORM_ID, &args![this]).u32();
                e.call(PRINT_WARNING, &args![0x0104_9548u32, id, own_id, name]);
            }
        }
    }
    if e.vcall(base, 0x188, &[]).u32() == 0 {
        let style = e.call(0x0050_5000, &[]).u32();
        e.vcall(base, 0x18c, &args![style]);
    }
    if e.mem.u32(base + 0x144) != 0 {
        let id = e.mem.u32(base + 0x144);
        if id != 0 {
            let (id, parts) = resolve_field(e, this, 0x144, id, TYPE_BGS_BODY_PART_DATA);
            if parts == 0 {
                let name = e.vcall(base, SLOT_NAME, &[]).u32();
                let own_id = e.call(FORM_ID, &args![this]).u32();
                e.call(PRINT_WARNING, &args![0x0104_94f8u32, id, own_id, name]);
            }
        }
    }
    let id = e.mem.u32(base + 0x14c);
    if id != 0 {
        let (id, impacts) = resolve_field(e, this, 0x14c, id, TYPE_BGS_IMPACT_DATA_SET);
        if impacts == 0 {
            let own_id = e.call(FORM_ID, &args![this]).u32();
            let name = e.vcall(base, SLOT_NAME, &[]).u32();
            e.call(PRINT_WARNING, &args![0x0102_c740u32, id, name, own_id]);
        }
    }
    let id = e.mem.u32(base + 0x158);
    if id != 0 {
        let (id, list) = resolve_field(e, this, 0x158, id, TYPE_BGS_LIST_FORM);
        if list == 0 {
            let own_id = e.call(FORM_ID, &args![this]).u32();
            let name = e.vcall(base, SLOT_NAME, &[]).u32();
            e.call(PRINT_WARNING, &args![0x0104_94bcu32, id, name, own_id]);
        }
    }
    if e.vcall(base + 0xb0, 0x10, &[]).u32() == 0 {
        e.call(0x0056_5210, &args![this, 1u32]);
    }
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

/// The container entry `CNTO` leaves for a following `COED` chunk (`011cb5f8`).
const PENDING_CONTAINER_ENTRY: u32 = 0x011c_b5f8;
/// `TESFile` calls of the load: `GetTESForm` (`00472660`), `GetTESChunk`
/// (`004726b0`), the next-chunk step (`004726f0`), the chunk size
/// (`00401660`), `GetChunkData_ov3` (`00472890`: buffer, size),
/// `GetChunkData` (`004727f0`: one word), `GetChunkData_ov2` (`00472840`:
/// 16 bits), the swap flag (`00401680`) and the record version (`00403570`).
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
const FILE_GET_CHUNK_WORD: u32 = 0x0047_27f0;
const FILE_GET_CHUNK_SHORT: u32 = 0x0047_2840;
const FILE_SWAPS: u32 = 0x0040_1680;
const FILE_VERSION: u32 = 0x0040_3570;
/// `00403d30` (cdecl: pointer, byte, count): `memset`.
const MEMSET: u32 = 0x0040_3d30;

/// A word read with `TESFile::GetChunkData` into a zeroed local.
fn read_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, local| {
        e.call(FILE_GET_CHUNK_WORD, &args![file, local]);
        e.mem.u32(local.addr())
    })
}

/// A component address the code passes: `this + offset`, or 0 for a null
/// `this`.
fn part(this: Ptr, offset: u32) -> u32 {
    if this.addr() == 0 {
        0
    } else {
        this.addr() + offset
    }
}

/// The `AIDT` chunk's fix-ups for older record versions on the 0x14-byte
/// block at `block` (byte 0 aggression, byte 1, byte 0xE).
fn fix_ai_data(e: &mut Engine, file: Ptr, block: u32) {
    if e.call(FILE_VERSION, &args![file]).u16() < 4 && e.mem.u8(block) > 1 {
        let value = e.mem.u8(block).wrapping_add(1);
        e.mem.set_u8(block, value);
    }
    if e.call(FILE_VERSION, &args![file]).u16() < 6 {
        // Byte 0 and byte 0xE from the old byte 0 (a value above 5 keeps both).
        let pair = match e.mem.u8(block) {
            0 => Some((None, 0u8)),
            1 => Some((Some(0u8), 1)),
            2 => Some((Some(1), 1)),
            3 => Some((Some(1), 2)),
            4 => Some((Some(2), 2)),
            5 => Some((Some(3), 0)),
            _ => None,
        };
        if let Some((first, other)) = pair {
            if let Some(first) = first {
                e.mem.set_u8(block, first);
            }
            e.mem.set_u8(block + 0xe, other);
        }
    }
    if e.call(FILE_VERSION, &args![file]).u16() < 7 {
        let value = 4u8.wrapping_sub(e.mem.u8(block + 1));
        e.mem.set_u8(block + 1, value);
    }
}

// Translated from 005f7ec0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f7ec0 - the exception-unwinding frame (FS:[0], frees the `COED` block if its constructor throws) and the security cookie check (`__security_check_cookie`, 00ec408c) are not translated; the alloca chunk buffers are heap blocks.
/// `TESCreature::Load` (Xbox PDB, `RET 4`: the file): false unless the record
/// is form type 0x2B. `TESForm::LoadForm`, form flag bit 0 cleared, then each
/// chunk until the next-chunk step fails (a sound chunk that
/// `CreatureSounds::LoadCreatureSound` refuses reads the same chunk tag
/// again without stepping): `EDID`, `OBND`, `FULL`, `MODL`/`MODT`, `SCRI`,
/// `CNTO`/`COED` (container entries, `004817f0`, with the item extra data
/// of `COED`), `SPLO`, `EITM`, `EAMT`, `AIDT` (with its version fix-ups),
/// `PKID`, `ACBS`, `SNAM`, `INAM`, `VTCK`, `TPLT`, `DSTD`/`DEST`, `KFFZ`,
/// `NIFZ`/`NIFT`, `DATA` (4 bytes at +0x12C), `RNAM` (the hand reach),
/// `TNAM`, `WNAM`, `BNAM` (the floats at +0x134, +0x138, +0x13C), `ZNAM`
/// (slot 0x18C), `PNAM` (+0x144), `CNAM` (`005f8bb0`), `LNAM` (`005f8bd0`),
/// `CSCR` (`005f9140`), `NAM4` (+0x148), `NAM5` (+0x154, 1 above 3),
/// `NAM0`/`NAM1` (read and dropped), `CSDT` (the sound type for the next
/// `CSDI`/`CSDF`). Afterwards a `MASTERFILE` warning when the speed
/// multiplier (`008f21d0` on the actor base data) is 0. Returns true.
pub fn tes_creature_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if u32::from(e.call(FILE_GET_FORM_TYPE, &args![file]).u8()) != 0x2b {
        return false;
    }
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    let mut sound_type = 0xffff_ffffu32;
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        let mut stay = false;
        let keep_going = true;
        match chunk {
            // BNAM
            0x4d41_4e42 => {
                e.call(FILE_GET_CHUNK_WORD, &args![file, base + 0x13c]);
            }
            // OBND
            0x444e_424f => {
                e.vcall(base, 0xe0, &args![file]);
            }
            // DATA
            0x4154_4144 => {
                e.call(0x0048_61f0, &args![this, file, base + 0x12c, 4u32]);
            }
            // NAM4
            0x344d_414e => {
                let value = read_word(e, file);
                e.mem.set_u32(base + 0x148, value);
            }
            // NAM0, NAM1: read into a buffer the chunk's size and dropped.
            0x304d_414e | 0x314d_414e => {
                if e.call(FILE_CHUNK_SIZE, &args![file]).u32() != 0 {
                    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                    let buffer = e.mem.alloc(size.max(1));
                    e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                    e.mem.free(buffer);
                }
            }
            // NAM5
            0x354d_414e => {
                let value = read_word(e, file);
                let level = if value > 3 { 1 } else { value };
                e.mem.set_u32(base + 0x154, level);
            }
            // COED, CNTO
            0x4445_4f43 | 0x4f54_4e43 => {
                let tag = e.call(FILE_GET_CHUNK, &args![file]).u32();
                if tag == 0x4445_4f43 {
                    if e.global::<u32>(PENDING_CONTAINER_ENTRY) != 0 {
                        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
                        let extra = if block != 0 {
                            e.call(0x0040_e690, &args![block]).u32()
                        } else {
                            0
                        };
                        let entry = e.global::<u32>(PENDING_CONTAINER_ENTRY);
                        e.mem.set_u32(entry + 8, extra);
                        let entry = e.global::<u32>(PENDING_CONTAINER_ENTRY);
                        let extra = e.mem.u32(entry + 8);
                        e.call(0x0040_e780, &args![extra, file]);
                        e.set_global::<u32>(PENDING_CONTAINER_ENTRY, 0);
                    }
                } else if tag == 0x4f54_4e43 {
                    let entry = e.with_stack(8, |e, local| {
                        e.call(MEMSET, &args![local, 0u32, 8u32]);
                        e.call(FILE_GET_CHUNK_DATA, &args![file, local, 8u32]);
                        if e.call(FILE_SWAPS, &args![file]).bool() {
                            e.call(0x0046_2230, &args![local]);
                        }
                        e.call(0x0048_17f0, &args![base + 0x64, local]).u32()
                    });
                    e.set_global::<u32>(PENDING_CONTAINER_ENTRY, entry);
                }
            }
            // EDID
            0x4449_4445 => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                let buffer = e.mem.alloc(size.max(1));
                e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                e.vcall(base, 0x134, &args![buffer]);
                e.mem.free(buffer);
            }
            // PKID
            0x4449_4b50 => {
                let package = read_word(e, file);
                e.call(0x0047_f500, &args![base + 0x90, package]);
            }
            // SCRI
            0x4952_4353 => {
                let script = read_word(e, file);
                e.call(0x006e_cd40, &args![base + 0xf4, script]);
                e.call(0x0048_cd90, &args![base + 0xf4, this]);
            }
            // DSTD, DEST
            0x4454_5344 | 0x5453_4544 => {
                e.call(0x0047_81e0, &args![part(this, 0x104), file]);
            }
            // CSDF, CSDI
            0x4644_5343 | 0x4944_5343 => {
                let sounds = fn_005f9210(e, this);
                if !e
                    .call(0x005f_0480, &args![sounds, file, sound_type, this])
                    .bool()
                {
                    stay = true;
                }
            }
            // VTCK
            0x4b43_5456 => {
                let value = read_word(e, file);
                e.mem.set_u32(base + 0x50, value);
            }
            // MODL, MODT
            0x4c44_4f4d | 0x5444_4f4d => {
                e.call(0x0048_92d0, &args![part(this, 0xdc), file]);
            }
            // FULL
            0x4c4c_5546 => {
                e.call(0x0048_7050, &args![part(this, 0xd0), file]);
            }
            // PNAM
            0x4d41_4e50 => {
                let value = read_word(e, file);
                e.mem.set_u32(base + 0x144, value);
            }
            // CNAM
            0x4d41_4e43 => {
                let value = read_word(e, file);
                e.call(0x005f_8bb0, &args![this, value]);
            }
            // INAM
            0x4d41_4e49 => {
                let value = read_word(e, file);
                e.mem.set_u32(base + 0x4c, value);
            }
            // LNAM
            0x4d41_4e4c => {
                let value = read_word(e, file);
                e.call(0x005f_8bd0, &args![this, value]);
            }
            // EITM
            0x4d54_4945 => {
                let value = read_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x70, value]);
            }
            // RNAM: the hand reach, a word or a byte.
            0x4d41_4e52 => {
                if e.call(FILE_CHUNK_SIZE, &args![file]).u32() == 4 {
                    let value = read_word(e, file);
                    e.mem.set_u8(base + 0x130, value as u8);
                } else {
                    e.call(FILE_GET_CHUNK_DATA, &args![file, base + 0x130, 1u32]);
                }
            }
            // SNAM: a faction and its rank.
            0x4d41_4e53 => {
                e.with_stack(8, |e, local| {
                    e.call(MEMSET, &args![local, 0u32, 8u32]);
                    e.call(FILE_GET_CHUNK_DATA, &args![file, local, 8u32]);
                    if e.call(FILE_SWAPS, &args![file]).bool() {
                        e.call(0x0050_3210, &args![local]);
                    }
                    let faction = e.mem.u32(local.addr());
                    let rank = u32::from(e.mem.u8(local.addr() + 4));
                    e.call(0x0047_d800, &args![base + 0x30, faction, rank]);
                });
            }
            // TNAM
            0x4d41_4e54 => {
                e.call(FILE_GET_CHUNK_WORD, &args![file, base + 0x134]);
            }
            // WNAM
            0x4d41_4e57 => {
                e.call(FILE_GET_CHUNK_WORD, &args![file, base + 0x138]);
            }
            // ZNAM
            0x4d41_4e5a => {
                let value = read_word(e, file);
                e.vcall(base, 0x18c, &args![value]);
            }
            // SPLO
            0x4f4c_5053 => {
                let spell = read_word(e, file);
                e.call(0x0048_d290, &args![base + 0x7c, spell]);
            }
            // NIFT, NIFZ
            0x5446_494e | 0x5a46_494e => {
                e.call(0x0048_9ce0, &args![base + 0x114, part(this, 0x114), file]);
            }
            // AIDT
            0x5444_4941 => {
                e.with_stack(0x14, |e, local| {
                    e.call(MEMSET, &args![local, 0u32, 0x14u32]);
                    e.call(FILE_GET_CHUNK_DATA, &args![file, local, 0x14u32]);
                    if e.call(FILE_SWAPS, &args![file]).bool() {
                        e.call(0x0050_3240, &args![local]);
                    }
                    fix_ai_data(e, file, local.addr());
                    e.call(0x0047_f0b0, &args![base + 0x90, local]);
                });
            }
            // CSCR
            0x5243_5343 => {
                let value = read_word(e, file);
                fn_005f9140(e, this, value);
            }
            // ACBS
            0x5342_4341 => {
                e.call(FILE_GET_CHUNK_DATA, &args![file, base + 0x34, 0x18u32]);
                if e.call(FILE_SWAPS, &args![file]).bool() {
                    e.call(0x0047_cb90, &args![base + 0x34]);
                }
                if e.call(FILE_VERSION, &args![file]).u16() < 8 {
                    e.mem.set_u16(base + 0x4a, 0);
                }
                e.mem.set_u16(base + 0x3a, 0);
            }
            // CSDT
            0x5444_5343 => {
                sound_type = read_word(e, file);
            }
            // TPLT
            0x544c_5054 => {
                let value = read_word(e, file);
                e.mem.set_u32(base + 0x54, value);
            }
            // EAMT
            0x544d_4145 => {
                let amount = e.with_stack(4, |e, local| {
                    e.call(FILE_GET_CHUNK_SHORT, &args![file, local]);
                    e.mem.u16(local.addr())
                });
                e.call(0x0048_3170, &args![base + 0x70, u32::from(amount)]);
            }
            // KFFZ
            0x5a46_464b => {
                e.call(0x0047_faa0, &args![base + 0xc4, part(this, 0xc4), file]);
            }
            _ => {}
        }
        if !keep_going {
            break;
        }
        if !stay && !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    if e.call(0x008f_21d0, &args![base + 0x30]).u16() == 0 {
        let own_id = e.call(FORM_ID, &args![this]).u32();
        let name = e.vcall(base, SLOT_NAME, &[]).u32();
        e.call(PRINT_WARNING, &args![0x0104_947cu32, name, own_id]);
    }
    true
}

/// The sound block both halves of the template copy share: the creature's
/// own sounds copied from the template's (`005f0640` on `005f9210`), or the
/// template's sound creature taken over (`005f9140`).
fn copy_sounds(e: &mut Engine, creature: Ptr, template: Ptr) {
    if fn_005f91d0(e, template) != 0 {
        let sounds = fn_005f9210(e, creature);
        let from = fn_005f91d0(e, template);
        e.call(0x005f_0640, &args![sounds, from]);
    } else {
        let other = fn_005f9100(e, template);
        fn_005f9140(e, creature, other);
    }
}

// Translated from 005fb450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fb450 - the exception-unwinding frame (FS:[0], destroys the local creature if a step throws) and the security cookie check (`__security_check_cookie`, 00ec408c) are not translated.
/// The creature's `TESActorBaseData::CopyFromTemplateForm` (vtable slot 0x10
/// of the actor base data `this`, at +0x30 of the creature; `RET 4`: a
/// template or null). Nothing without a template of its own (`0059bb30`) and
/// without one given. A default creature is built on the stack
/// (`TESCreature::TESCreature`, `008126e0` with `00505000`); the template is
/// the data's own unless `0047cdd0` says not, the given one when it is a
/// creature (form type 0x2B), else the default creature; a template with a
/// template of its own copies from it first (its slot 0x10 with 0) unless
/// `0047cca0` says it has. Then, for each template flag (`0047cd40`): 9 the
/// script, 8 the container, 7 the name and flag bits 0x1070220A, 6 the
/// model, model list, animations, slot 0x180/0x184, `0050eaa0`, `005f9b30`/
/// `005f8bb0`, slots 0x58/0x5C, the destructible data, flag bits 0x4B8F8071,
/// the sounds and +0x124 (`009c0070`); a missing sound creature (+0x128) is
/// copied in any case; 0 the AI data words and float, the creature type,
/// `005f9b50`/`005f8bd0` and the combat style; 1 the stats (level words,
/// attributes, health, attack damage, speed, combat skill, `005f9ad0`/
/// `005f9af0` and three floats); 2 the factions; 3 the spells and the touch
/// spell; 4 and 5 the AI form parts. Then the template's packing
/// (`0047d1a0`, `BGSMenuPacker::RecomputePacking`, `0047cd90`), `0047ccc0`
/// with 1, and the default creature destroyed (`005f7900`).
pub fn fn_005fb450(e: &mut Engine, this: Ptr, given: u32) {
    let data = this.addr();
    let creature = Ptr::new(data.wrapping_sub(0x30));
    if e.call(0x0059_bb30, &args![this]).u32() == 0 && given == 0 {
        return;
    }
    e.with_stack(0x160, |e, local| {
        tescreature_tescreature(e, local);
        let combat_style = e.call(0x0050_5000, &[]).u32();
        e.call(0x0081_26e0, &args![local, combat_style]);
        let mut template = 0u32;
        if !e.call(0x0047_cdd0, &args![this]).bool() {
            template = e.call(0x0059_bb30, &args![this]).u32();
        }
        if given != 0 && e.call(0x0040_1170, &args![given]).u32() == 0x2b {
            template = given;
        }
        if template == 0 {
            template = local.addr();
        } else if e.call(0x0059_bb30, &args![template + 0x30]).u32() != 0
            && !e.call(0x0047_cca0, &args![template + 0x30]).bool()
        {
            e.vcall(template + 0x30, 0x10, &args![0u32]);
        }
        let t = Ptr::new(template);
        let flag = |e: &mut Engine, bit: u32| e.call(0x0047_cd40, &args![this, bit]).bool();
        if flag(e, 9) {
            e.call(0x0048_cce0, &args![data + 0xc4, part(t, 0xf4)]);
        }
        if flag(e, 8) {
            e.call(0x0048_1c80, &args![data + 0x34, part(t, 0x64)]);
        }
        if flag(e, 7) {
            e.call(0x0048_70f0, &args![data + 0xa0, part(t, 0xd0)]);
            let own = e.call(0x0071_7e50, &args![this]).u32();
            let own = e.mem.u32(own) & 0xef8f_ddf5;
            let theirs = e.call(0x0071_7e50, &args![template + 0x30]).u32();
            let theirs = e.mem.u32(theirs) & 0x1070_220a;
            e.call(0x0047_dd30, &args![this, theirs | own]);
        }
        if flag(e, 6) {
            e.call(0x0048_9430, &args![data + 0xac, part(t, 0xdc)]);
            e.call(0x0048_9db0, &args![data + 0xe4, part(t, 0x114)]);
            e.call(0x0047_fb50, &args![data + 0x94, part(t, 0xc4)]);
            let value = e.vcall(template, 0x180, &[]).u32();
            e.vcall(creature.addr(), 0x184, &args![value]);
            e.call(0x0050_eaa0, &args![creature, template]);
            let value = e.call(0x005f_9b30, &args![template]).u32();
            e.call(0x005f_8bb0, &args![creature, value]);
            let value = e.vcall(template + 0x30, 0x58, &[]).u32();
            e.vcall(data, 0x5c, &args![value]);
            e.call(0x0047_86e0, &args![data + 0xd4, part(t, 0x104)]);
            let own = e.call(0x0071_7e50, &args![this]).u32();
            let own = !0x4b8f_8071u32 & e.mem.u32(own);
            let theirs = e.call(0x0071_7e50, &args![template + 0x30]).u32();
            let theirs = e.mem.u32(theirs) & 0x4b8f_8071;
            e.call(0x0047_dd30, &args![this, theirs | own]);
            copy_sounds(e, creature, t);
            let value = e.call(0x009c_0070, &args![template]).u32();
            e.mem.set_u32(data + 0x124, value);
        }
        if e.mem.u32(data + 0xf8) == 0 {
            copy_sounds(e, creature, t);
        }
        if flag(e, 0) {
            let value = e.call(0x0044_1110, &args![template + 0x30]).u32();
            e.call(0x0050_f9a0, &args![this, value]);
            let value = e.vcall(template + 0x30, 0x68, &[]).u32();
            e.call(0x0050_f9c0, &args![this, value]);
            let value = e.vcall(template + 0x30, 0x64, &[]).f64();
            e.call(0x0047_e1c0, &args![this, x87_store32(value)]);
            let kind = e.call(0x0059_f3a0, &args![template]).u8();
            fn_005fbdb0(e, creature, kind);
            let value = e.call(0x005f_9b50, &args![template]).u32();
            e.call(0x005f_8bd0, &args![creature, value]);
            let value = e.vcall(template, 0x188, &[]).u32();
            e.vcall(creature.addr(), 0x18c, &args![value]);
        }
        if flag(e, 1) {
            let value = u32::from(e.call(0x0047_d370, &args![template + 0x30]).u16());
            e.call(0x0047_dfe0, &args![this, value]);
            let value = u32::from(e.call(0x0046_1560, &args![template + 0x30]).u8());
            e.call(0x005d_1070, &args![this, value]);
            let value = u32::from(e.call(0x0047_d390, &args![template + 0x30]).u16());
            e.call(0x0047_de40, &args![this, value]);
            let value = u32::from(e.call(0x0047_d3b0, &args![template + 0x30]).u16());
            e.call(0x0047_de70, &args![this, value]);
            e.call(0x0048_0000, &args![data + 0x88, part(t, 0xb8)]);
            e.call(0x0048_7240, &args![data + 0x80, part(t, 0xb0)]);
            e.call(0x0047_fe30, &args![data + 0xdc, part(t, 0x10c)]);
            let value = u32::from(e.call(0x008f_21d0, &args![template + 0x30]).u16());
            e.call(0x0047_e270, &args![this, value]);
            let value = u32::from(e.vcall(template + 0x30, 0x60, &[]).u16());
            e.call(0x0047_e010, &args![this, value]);
            let skill = fn_005f8f70(e, t);
            fn_005fa2c0(e, creature, skill);
            let value = u32::from(e.call(0x005f_9ad0, &args![template]).u8());
            e.call(0x005f_9af0, &args![creature, value]);
            for (getter, setter) in [
                (0x0082_1640u32, 0x0050_7780u32),
                (0x0056_7470, 0x0080_3a90),
                (0x0082_1660, 0x005f_9b10),
            ] {
                let value = e.call(getter, &args![template]).f64();
                e.call(setter, &args![creature, x87_store32(value)]);
            }
        }
        if flag(e, 2) {
            e.call(0x0047_ca20, &args![this, part(t, 0x30)]);
        }
        if flag(e, 3) {
            e.call(0x0048_d630, &args![data + 0x4c, part(t, 0x7c)]);
            e.call(0x0047_bdd0, &args![data + 0x40, part(t, 0x70)]);
        }
        if flag(e, 4) {
            e.call(0x0047_f1e0, &args![data + 0x60, part(t, 0x90)]);
        }
        if flag(e, 5) {
            e.call(0x0047_f2d0, &args![data + 0x60, part(t, 0x90)]);
        }
        e.with_stack(4, |e, packing| {
            e.mem.set_u16(packing.addr(), 0);
            let packer = e.call(0x0047_d1a0, &args![creature, packing]).u32();
            if packer != 0 {
                e.call(0x0070_37c0, &args![this, packer]);
                let value = u32::from(e.mem.u16(packing.addr()));
                e.call(0x0047_cd90, &args![this, value]);
            }
        });
        e.call(0x0047_ccc0, &args![this, 1u32]);
        fn_005f7900(e, local);
    });
}

/// `Interface::QueueMenuCreate` (Xbox PDB, `00709470`, cdecl, six words).
const QUEUE_MENU_CREATE: u32 = 0x0070_9470;
/// `008d8520` (thiscall on a reference): its process.
const REF_PROCESS: u32 = 0x008d_8520;
/// `MiddleHighProcess::GetForceNextUpdate` (Xbox PDB name of `00566950`).
const ACTOR_00566950: u32 = 0x0056_6950;
/// `00493bb0` (thiscall on the actor, `animation.cpp`).
const ACTOR_00493BB0: u32 = 0x0049_3bb0;
/// `005d43c0` (thiscall on a reference): its extra data list.
const REF_EXTRA_LIST: u32 = 0x005d_43c0;
/// `0041d8a0` (thiscall on a package): its schedule; `00726070` the word at +4.
const PACKAGE_SCHEDULE: u32 = 0x0041_d8a0;
const WORD_4: u32 = 0x0072_6070;
/// `0041ca90` (thiscall on a package): its type.
const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// The VATS object (`011f2250`) and `VATS::QuitVATSPlayback` (Xbox PDB,
/// `009c8950`, `RET 8`).
const VATS: u32 = 0x011f_2250;
const VATS_QUIT_PLAYBACK: u32 = 0x009c_8950;

/// The "%s %s" message of `005fa330` with an icon (`007052f0`): the
/// reference's name (`0055d520`) and the text of the setting `setting`
/// (`00403df0`), with the float at `010162c0` as the display time.
fn activate_message(e: &mut Engine, actor: u32, setting: u32, icon: u32) {
    let text = e.call(0x0040_3df0, &args![setting]).u32();
    let name = e.call(0x0055_d520, &args![actor]).u32();
    e.with_stack(0x134, |e, line| {
        e.call(0x00ec_623a, &args![line, 0x0101_2058u32, name, text]);
        let time = x87_quiet_bits(e.global::<u32>(0x0101_62c0));
        e.call(0x0070_52f0, &args![line, 0u32, icon, 0u32, time, 0u32]);
    });
}

/// Where the talk-state call of `005fa330` finds its process: one fetched
/// before, or the process of an actor (`008d8520`), fetched right before
/// the call.
enum TalkProcess {
    Known(u32),
    Of(u32),
}

/// The talk-state choice of `005fa330`: when the package has a schedule
/// whose word at +4 is 0, state 2 for `actor` (process vtable slot 0x288),
/// otherwise state 1 for `other`.
fn set_talk_state(e: &mut Engine, process: TalkProcess, package: u32, actor: u32, other: u32) {
    let mut state = (other, 1u32);
    if e.call(PACKAGE_SCHEDULE, &args![package]).u32() != 0 {
        let schedule = e.call(PACKAGE_SCHEDULE, &args![package]).u32();
        if e.call(WORD_4, &args![schedule]).u32() == 0 {
            state = (actor, 2);
        }
    }
    let process = match process {
        TalkProcess::Known(process) => process,
        TalkProcess::Of(owner) => e.call(REF_PROCESS, &args![owner]).u32(),
    };
    e.vcall(process, 0x288, &args![state.0, state.1]);
}

// Translated from 005fa330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fa330 - the compiler's security cookie check (`__security_check_cookie`, 00ec408c) is not translated.
/// The creature's activation (`RET 0x14`: the activated reference, the
/// activator, a word it does not read, an item and a count). Both references
/// must be actors (`__RTDynamicCast` to `Actor`); false when the creature
/// actor's slot 0x2E8 says so, it has no process, its process is busy (slot
/// 0x52C with the actor's slot 0x22C(0) false, or slot 0x610). An activator
/// other than the player with a type-0xF package extra hands that package to
/// the creature (`ExtraDataList::SetPackageExtra`, process slot 0x5A0, the
/// activator's slot 0x28C with the day for once-a-day packages, the
/// creature's slot 0x48 with 0x8000_0000). Then, in order: a dead or
/// unconscious creature (`00437bd0`, slot 0x230, `004f8960` = 6) shows the
/// "surprised" message unless its type is 6 (false); a fleeing one
/// (`Actor::IsFleeing`) shows the "sad" message unless it fights the player
/// unblocked (false), else true; a creature whose slot 0x22C(0) holds opens
/// the container menu for the player (`QueueMenuCreate` 1) or takes the item
/// (slot 0x17C) from another activator (true); an attacker
/// (`Actor::GetShouldAttackActor`) starts combat (process slot 0x33C, true);
/// an essential dead creature activated by the player gives false; the
/// player's companion-style activation (the player base's slot 0x1C and the
/// creature's state) opens menu 1 with 2, or shows the "sad" message when
/// its process's slot 0x110 refuses (false); the companion wheel
/// (`CompanionWheelMenu::GetShouldCompanionWheelOpen`, menu 8, false); then
/// the dialogue: the player's greeting topic (`TESTopic::GetTopic`,
/// `CreateDialogueItem`), said directly (`0057b7c0`) when it has a single
/// response, otherwise the dialogue menu (4) with the talk states and sounds
/// (`BSAudio::EndAllSoundsOfType` 4, `Actor::StopAttack`,
/// `MagicCaster::InterruptCast`, `VATS::QuitVATSPlayback`), or the
/// conversation between two actors (slot 0x280). The dialogue item is freed
/// at the end (`005c90d0` with 1). True unless a step above says false.
pub fn fn_005fa330(
    e: &mut Engine,
    this: Ptr,
    target: u32,
    activator: u32,
    _unused_3: u32,
    item: u32,
    count: u32,
) -> bool {
    let player = |e: &Engine| e.global::<u32>(PLAYER);
    let who = e
        .call(
            RT_DYNAMIC_CAST,
            &args![activator, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
        )
        .u32();
    let me = e
        .call(
            RT_DYNAMIC_CAST,
            &args![target, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
        )
        .u32();
    if me == 0 || who == 0 || e.vcall(me, 0x2e8, &[]).bool() {
        return false;
    }
    let process = e.call(REF_PROCESS, &args![me]).u32();
    if process == 0 {
        return false;
    }
    if process != 0
        && e.vcall(process, 0x52c, &[]).u32() != 0
        && !e.vcall(me, 0x22c, &args![0u32]).bool()
    {
        return false;
    }
    if process != 0 && e.vcall(process, 0x610, &[]).u32() != 0 {
        return false;
    }
    let who_process = e.call(REF_PROCESS, &args![who]).u32();
    let who_package = e.vcall(who_process, 0x27c, &[]).u32();
    let my_package = e.vcall(process, 0x27c, &[]).u32();
    if who != 0 && who != player(e) {
        let list = e.call(REF_EXTRA_LIST, &args![who]).u32();
        let extra = e.call(0x0041_cb10, &args![list]).u32();
        if extra != 0 && e.call(PACKAGE_TYPE, &args![extra]).u32() == 0xf {
            let my_list = e.call(REF_EXTRA_LIST, &args![me]).u32();
            let target_data = e.call(0x0041_cb70, &args![my_list]).u32();
            let list = e.call(REF_EXTRA_LIST, &args![who]).u32();
            e.call(
                0x0041_c930,
                &args![list, extra, 4u32, target_data, 0u32, 0u32, 0u32],
            );
            let my_process = e.call(REF_PROCESS, &args![me]).u32();
            e.vcall(my_process, 0x5a0, &args![who, extra]);
            if e.call(0x0067_0f90, &args![extra]).bool() {
                let day = u32::from(e.call(0x0086_7d60, &args![0x011d_e7b8u32]).u8());
                e.vcall(who, 0x28c, &args![extra, day]);
            }
            e.vcall(me, 0x48, &args![0x8000_0000u32]);
        }
    }
    if e.call(0x0043_7bd0, &args![me]).bool()
        || e.vcall(me, 0x230, &[]).bool()
        || e.call(0x004f_8960, &args![me]).u32() == 6
    {
        if e.call(0x0059_f3a0, &args![this]).u8() as i8 != 6 {
            activate_message(e, me, 0x011d_210c, 0x0104_9638);
        }
        return false;
    }
    if e.call(0x008a_6650, &args![me, 0u32]).bool() {
        let combat = e.vcall(me, 0x428, &[]).u32();
        let fights = combat != 0
            && {
                let player_ref = player(e);
                e.call(0x0097_fa10, &args![combat, player_ref]).bool()
            }
            && {
                let player_ref = player(e);
                !e.call(0x0089_4d60, &args![player_ref]).bool()
            };
        if fights {
            return true;
        }
        activate_message(e, me, 0x011d_2538, 0x0102_08a0);
        return false;
    }
    if e.vcall(me, 0x22c, &args![0u32]).bool() {
        if who == player(e) {
            e.call(
                QUEUE_MENU_CREATE,
                &args![1u32, target, 0u32, 0u32, 1u32, 0u32],
            );
        } else if item != 0
            && e.call(
                RT_DYNAMIC_CAST,
                &args![item, 0u32, 0x0118_3108u32, 0x0118_3a1cu32, 0u32],
            )
            .u32()
                == 0
        {
            e.vcall(
                target,
                0x17c,
                &args![item, 0u32, count, 0u32, 0u32, who, 0u32, 0u32, 1u32, 0u32],
            );
        }
        return true;
    }
    if who != player(e) {
        let attacks = e.with_stack(4, |e, local| {
            e.call(0x008b_06d0, &args![who, me, 0u32, local, 0u32])
                .bool()
        });
        if attacks {
            let who_process = e.call(REF_PROCESS, &args![who]).u32();
            e.vcall(
                who_process,
                0x33c,
                &args![who, me, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32],
            );
            return true;
        }
    }
    if who == player(e) && e.vcall(me, 0x230, &[]).bool() && e.call(0x0087_f3d0, &args![me]).bool()
    {
        return false;
    }
    let base = e.call(0x0041_81e0, &args![me]).u32().wrapping_add(0x30);
    if e.vcall(base, 0x1c, &[]).bool()
        && !e.call(ACTOR_00493BB0, &args![me]).bool()
        && who == player(e)
    {
        let dead_not_essential =
            e.vcall(me, 0x230, &[]).bool() && !e.call(0x0087_f3d0, &args![me]).bool();
        let other = dead_not_essential || {
            let player_ref = player(e);
            e.call(0x0049_97b0, &args![player_ref]).bool()
                && !e.call(ACTOR_00566950, &args![me]).bool()
        };
        if other && !e.call(0x008a_ce90, &args![me]).bool() {
            if !e.vcall(me, 0x230, &[]).bool() {
                let my_process = e.call(REF_PROCESS, &args![me]).u32();
                if e.vcall(my_process, 0x110, &[]).bool() {
                    activate_message(e, me, 0x011d_2394, 0x0102_08a0);
                    return false;
                }
            }
            e.call(
                QUEUE_MENU_CREATE,
                &args![1u32, target, 0u32, 0u32, 2u32, 0u32],
            );
            return true;
        }
    }
    let companion_wheel = |e: &mut Engine| {
        if e.call(ACTOR_00566950, &args![me]).bool() && activator == e.global::<u32>(PLAYER) {
            if e.call(0x0075_4d90, &args![me]).bool() {
                e.call(
                    QUEUE_MENU_CREATE,
                    &args![8u32, target, 0u32, 0u32, 0u32, 0u32],
                );
            }
            return Some(false);
        }
        None
    };
    if !e.call(0x0089_8ff0, &args![me]).bool() && who == player(e) {
        return companion_wheel(e).unwrap_or(true);
    }
    if let Some(result) = companion_wheel(e) {
        return result;
    }
    let topic = if who == player(e) {
        e.call(0x0061_a2d0, &args![0u32, 0u32]).u32()
    } else {
        0
    };
    let dialogue = if topic != 0 {
        let player_ref = player(e);
        e.call(0x0061_b320, &args![topic, me, player_ref, 0u32, 0u32, 0u32])
            .u32()
    } else {
        0
    };
    'talk: {
        if dialogue != 0 {
            let id = e.call(FORM_ID, &args![dialogue]).u32();
            if e.call(0x0061_9df0, &args![id]).bool()
                && e.call(0x0083_c7b0, &args![dialogue]).bool()
                && !e.call(0x0083_c7e0, &args![dialogue]).bool()
            {
                e.call(0x0057_b7c0, &args![me, topic, 0u32, 0u32]);
                break 'talk;
            }
        }
        if dialogue == 0 {
            return false;
        }
        if e.call(ACTOR_00493BB0, &args![me]).bool()
            && who == player(e)
            && e.call(ACTOR_00566950, &args![me]).bool()
        {
            e.call(VATS_QUIT_PLAYBACK, &args![VATS, 0u32, 0u32]);
            e.call(QUEUE_MENU_CREATE, &args![4u32, me, 0u32, 0u32, 1u32, 0u32]);
            break 'talk;
        }
        if me == 0 {
            break 'talk;
        }
        if e.call(ACTOR_00493BB0, &args![me]).bool() && !e.vcall(me, 0x230, &[]).bool() {
            break 'talk;
        }
        if who == player(e)
            && e.vcall(me, 0x214, &[]).u32() != 0
            && e.vcall(me, 0x214, &[]).u32() != 9
            && e.vcall(me, 0x214, &[]).u32() != 4
        {
            let my_process = e.call(REF_PROCESS, &args![me]).u32();
            if e.vcall(my_process, 0x4d4, &[]).u32() == 0 {
                break 'talk;
            }
            let my_process = e.call(REF_PROCESS, &args![me]).u32();
            let data = e.vcall(my_process, 0x4d4, &[]).u32();
            if (e.mem.u8(data + 0xe) as i32) <= 0x13 {
                break 'talk;
            }
        }
        let mut menu_argument = 0u32;
        if who == player(e) {
            if !e.call(0x008b_3bb0, &args![me]).bool() {
                let my_process = e.call(REF_PROCESS, &args![me]).u32();
                e.vcall(my_process, 0x294, &args![me]);
            }
            e.call(0x008a_8e50, &args![who]);
            e.call(0x008a_8e50, &args![me]);
            if e.vcall(me, 0x2c8, &[]).u32() == player(e)
                && my_package != 0
                && e.call(PACKAGE_TYPE, &args![my_package]).u32() == 0
            {
                set_talk_state(e, TalkProcess::Known(process), my_package, who, me);
            }
            let audio = e.call(0x0045_3a70, &[]).u32();
            e.call(0x00ad_8780, &args![audio, 4u32]);
            e.call(0x0081_5b00, &args![me + 0x88]);
            e.call(0x008a_7a90, &args![me]);
            e.call(0x008a_7a90, &args![who]);
            if e.vcall(process, 0x4bc, &[]).u32() != 9 {
                e.call(VATS_QUIT_PLAYBACK, &args![VATS, 0u32, 0u32]);
                let mut speaker = who;
                if speaker == player(e) {
                    speaker = me;
                }
                e.call(
                    QUEUE_MENU_CREATE,
                    &args![4u32, speaker, menu_argument, 0u32, 1u32, 0u32],
                );
            } else {
                let player_ref = player(e);
                e.vcall(
                    me,
                    0x280,
                    &args![player_ref, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
                );
            }
            break 'talk;
        }
        if e.call(0x0070_2640, &[]).u32() != 0x3f1 && me == player(e) {
            let my_process = e.call(REF_PROCESS, &args![me]).u32();
            if !e.vcall(my_process, 0x3fc, &args![who]).bool() {
                return false;
            }
            let audio = e.call(0x0045_3a70, &[]).u32();
            e.call(0x00ad_8780, &args![audio, 4u32]);
            e.call(0x008a_8e50, &args![who]);
            e.call(0x008a_8e50, &args![me]);
            let package = e.call(0x0093_44a0, &args![who]).u32();
            let dialogue_package = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![package, 0u32, 0x0118_46a0u32, 0x0119_9c3cu32, 0u32],
                )
                .u32();
            if dialogue_package != 0 {
                menu_argument = e.mem.u32(dialogue_package + 0x8c);
            }
            set_talk_state(e, TalkProcess::Of(who), who_package, who, who);
            e.call(0x0081_5b00, &args![me + 0x88]);
            e.call(0x008a_7a90, &args![me]);
            e.call(0x008a_7a90, &args![who]);
            e.call(VATS_QUIT_PLAYBACK, &args![VATS, 0u32, 0u32]);
            let mut speaker = who;
            if speaker == player(e) {
                speaker = me;
            }
            e.call(
                QUEUE_MENU_CREATE,
                &args![4u32, speaker, menu_argument, 0u32, 1u32, 0u32],
            );
            break 'talk;
        }
        if me != who && e.call(0x0093_44a0, &args![me]).u32() != 0 {
            let package = e.call(0x0093_44a0, &args![me]).u32();
            if e.call(PACKAGE_TYPE, &args![package]).u32() != 1 {
                let package = e.call(0x0093_44a0, &args![me]).u32();
                if e.call(PACKAGE_TYPE, &args![package]).u32() != 2 {
                    e.vcall(process, 0x288, &args![me, 1u32]);
                }
            }
        }
        if me != who && e.vcall(me, 0x214, &[]).u32() == 0 {
            let position = e.vcall(who, 0x1f4, &[]).u32();
            let (x, y, z) = (
                e.mem.u32(position),
                e.mem.u32(position + 4),
                e.mem.u32(position + 8),
            );
            e.call(0x008b_b520, &args![me, x, y, z, 0u32]);
        }
        if !e.call(0x0093_36c0, &args![who]).bool() {
            set_talk_state(e, TalkProcess::Of(who), who_package, who, who);
            if e.vcall(
                who,
                0x280,
                &args![me, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .bool()
            {
                let who_process = e.call(REF_PROCESS, &args![who]).u32();
                e.vcall(who_process, 0x288, &args![who, 2u32]);
                e.vcall(process, 0x288, &args![me, 2u32]);
            }
        } else {
            let who_process = e.call(REF_PROCESS, &args![who]).u32();
            e.vcall(who_process, 0x288, &args![who, 1u32]);
        }
    }
    if dialogue != 0 {
        e.call(0x005c_90d0, &args![dialogue, 1u32]);
    }
    true
}

/// The destructor body `005f78b0` runs (`005f77b0`, `thiscall`).
const CREATURE_DESTRUCT_005F77B0: u32 = 0x005f_77b0;
/// The savers `TESCreature::Save` calls: `TESForm::StartForm`
/// (`004855a0`), `TESFullName::Save` (`00487010`), `TESModel::Save`
/// (`004891d0`, two chunk words), `TESSpellList::Save` (`0048d330`),
/// `TESEnchantableForm::Save` (`0047bd70`), `TESModelList::Save`
/// (`00489ba0`, two chunk words), `TESActorBaseData::Save` (`0047cac0`),
/// `BGSDestructibleObjectForm::Save` (`00477f20`), the empty `00483710`,
/// `TESForm::__SaveData` (`00485e20`, `thiscall (form, data, size)`),
/// `TESForm::AddChunk` with a byte (`004858f0`) and with a word
/// (`00485910`, both `cdecl`) and `TESForm::CloseForm` (`00485680`); the
/// byte `00401500` answers (whether saving swaps bytes).
const SAVE_FORM_START: u32 = 0x0048_55a0;
const SAVE_FULL_NAME: u32 = 0x0048_7010;
const SAVE_MODEL: u32 = 0x0048_91d0;
const SAVE_SPELL_LIST: u32 = 0x0048_d330;
const SAVE_ENCHANTABLE: u32 = 0x0047_bd70;
const SAVE_MODEL_LIST: u32 = 0x0048_9ba0;
const SAVE_ACTOR_BASE_DATA: u32 = 0x0047_cac0;
const SAVE_DESTRUCTIBLE: u32 = 0x0047_7f20;
const SAVE_EMPTY_00483710: u32 = 0x0048_3710;
const SAVE_FORM_DATA: u32 = 0x0048_5e20;
const SAVE_CHUNK_BYTE: u32 = 0x0048_58f0;
const SAVE_CHUNK_WORD: u32 = 0x0048_5910;
const SAVE_FORM_CLOSE: u32 = 0x0048_5680;
const SAVE_SWAPS: u32 = 0x0040_1500;
/// The creature getters the save asks for the template (`005f91d0`) and the
/// combat style (`005f9100`), and the template's own save step (`005f03a0`);
/// all `thiscall` without words.
const CREATURE_TEMPLATE_005F91D0: u32 = 0x005f_91d0;
const CREATURE_COMBAT_STYLE_005F9100: u32 = 0x005f_9100;
const TEMPLATE_005F03A0: u32 = 0x005f_03a0;
/// The actor base data setter `005f9010` forwards to (`0047dd50`,
/// `thiscall`, three words: the flag, the value, 1).
const ACTOR_BASE_DATA_SET_FLAG: u32 = 0x0047_dd50;
/// The calls of the creature actor value setter `005fa200`: whether the
/// index is one of a group (`00406d70`, `cdecl (index, 0x400)`), the
/// setter `005fa2c0` (`thiscall`, one byte), the 16-bit setter of the
/// actor base data at +0x10C (`0047ff50`, one word) and the base setter
/// `005f12d0` (`thiscall`, two words).
const INDEX_IN_GROUP: u32 = 0x0040_6d70;
const CREATURE_SETTER_005FA2C0: u32 = 0x005f_a2c0;
const DATA_SET_SHORT: u32 = 0x0047_ff50;
const BASE_SET_ACTOR_VALUE: u32 = 0x005f_12d0;
/// The change-flag size, save, load and load-game steps of the base
/// (`005f16f0`, `005f18c0`, `005f1b30`, `005f1fd0`), `TESForm::SaveGameDataOLD`
/// (`00484ce0`) and `TESForm::LoadGameDataOLD` (`00484d00`), both
/// `thiscall (form, data, size)`, and the load-game buffer reads
/// (`00428110`, `004280f0`, `00864980` as in `TESLeveledList::LoadGame`).
const BASE_CHANGED_SIZE: u32 = 0x005f_16f0;
const BASE_SAVE_GAME: u32 = 0x005f_18c0;
const BASE_LOAD_GAME: u32 = 0x005f_1b30;
const BASE_LOAD_GAME_BUFFER: u32 = 0x005f_1fd0;
const FORM_SAVE_GAME_DATA: u32 = 0x0048_4ce0;
const FORM_LOAD_GAME_DATA: u32 = 0x0048_4d00;
const BUFFER_READ: u32 = 0x0086_4980;
/// The setting value getter (`0043d4d0`, `thiscall`: the address of the
/// value) and the three settings the sound level picks
/// (`011cdde0`, `011cd538`, `011cd3b4`).
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
const SETTING_SOUND_LEVEL_0: u32 = 0x011c_dde0;
const SETTING_SOUND_LEVEL_1: u32 = 0x011c_d538;
const SETTING_SOUND_LEVEL_2: u32 = 0x011c_d3b4;

// Translated from 005f78b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the scalar deleting
/// destructor of a `TESCreature`): the destructor body `005f77b0`, then
/// `operator delete` (`00401030`) when bit 0 of `flags` is set; answers
/// `this`.
pub fn fn_005f78b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(CREATURE_DESTRUCT_005F77B0, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 005f7c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCreature::Save` (Xbox PDB): starts the form, saves the full name
/// (+0xD0), the model (+0xDC, `MODL`, `MODT`), the spell list (+0x7C), the
/// enchantable part (+0x70), the model list (+0x114, `NIFZ`, `NIFT`), the
/// actor base data (+0x30) and the destructible data (+0x104), the four
/// bytes at +0x12C (`__SaveData`; the empty `00483710` run on them before
/// and after when saving swaps bytes), the hand reach (`RNAM`), the forms
/// of virtual slots +0x188 (`ZNAM`) and +0x180 (`PNAM`) when there are
/// any, the turn speed (`TNAM`), base scale (`BNAM`) and foot weight
/// (+0x138, `WNAM`) floats, the blood impact material (`NAM4`) and sound
/// level (`NAM5`); with actor base flag 0x100 set the template's
/// `005f03a0`, otherwise the combat style's form id (`CSCR`); the impact
/// data set (`CNAM`) and weapon list (`LNAM`) when set; closes the form.
pub fn tes_creature_save(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.call(SAVE_FORM_START, &args![this]);
    e.call(SAVE_FULL_NAME, &args![at + 0xd0]);
    e.call(
        SAVE_MODEL,
        &args![at + 0xdc, 0x4c44_4f4du32, 0x5444_4f4du32],
    );
    e.call(SAVE_SPELL_LIST, &args![at + 0x7c]);
    e.call(SAVE_ENCHANTABLE, &args![at + 0x70]);
    e.call(
        SAVE_MODEL_LIST,
        &args![at + 0x114, 0x5a46_494eu32, 0x5446_494eu32],
    );
    e.call(SAVE_ACTOR_BASE_DATA, &args![at + 0x30]);
    e.call(SAVE_DESTRUCTIBLE, &args![at + 0x104]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        e.call(SAVE_EMPTY_00483710, &args![at + 0x12c]);
    }
    e.call(SAVE_FORM_DATA, &args![this, at + 0x12c, 4u32]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        e.call(SAVE_EMPTY_00483710, &args![at + 0x12c]);
    }
    let reach = u32::from(e.mem.u8(at + 0x130));
    e.call(SAVE_CHUNK_BYTE, &args![0x4d41_4e52u32, reach]);
    for (slot, chunk) in [(0x188u32, 0x4d41_4e5au32), (0x180, 0x4d41_4e50)] {
        if e.vcall(at, slot, &args![]).u32() != 0 {
            let form = e.vcall(at, slot, &args![]).u32();
            let id = e.call(FORM_ID, &args![form]).u32();
            e.call(SAVE_CHUNK_WORD, &args![chunk, id]);
        }
    }
    for (offset, chunk) in [
        (0x134u32, 0x4d41_4e54u32),
        (0x13c, 0x4d41_4e42),
        (0x138, 0x4d41_4e57),
    ] {
        let value = x87_quiet_bits(e.mem.u32(at + offset));
        e.call(SAVE_CHUNK_WORD, &args![chunk, value]);
    }
    let value = e.mem.u32(at + 0x148);
    e.call(SAVE_CHUNK_WORD, &args![0x344d_414eu32, value]);
    let value = e.mem.u32(at + 0x154);
    e.call(SAVE_CHUNK_WORD, &args![0x354d_414eu32, value]);
    if e.call(FORM_FLAGS_TEST, &args![at + 0x30, 0x100u32]).u8() != 0 {
        let template = e.call(CREATURE_TEMPLATE_005F91D0, &args![this]).u32();
        if template != 0 {
            e.call(TEMPLATE_005F03A0, &args![template]);
        }
    } else {
        let style = e.call(CREATURE_COMBAT_STYLE_005F9100, &args![this]).u32();
        if style != 0 {
            let id = e.call(FORM_ID, &args![style]).u32();
            e.call(SAVE_CHUNK_WORD, &args![0x5243_5343u32, id]);
        }
    }
    for (offset, chunk) in [(0x14cu32, 0x4d41_4e43u32), (0x158, 0x4d41_4e4c)] {
        let form = e.mem.u32(at + offset);
        if form != 0 {
            let id = e.call(FORM_ID, &args![form]).u32();
            e.call(SAVE_CHUNK_WORD, &args![chunk, id]);
        }
    }
    e.call(SAVE_FORM_CLOSE, &args![this]);
}

// Translated from 005f9010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one byte word): sets actor base
/// flag 0x800 to `value` (`0047dd50(0x800, value, 1)`).
pub fn fn_005f9010(e: &mut Engine, this: Ptr, value: u32) {
    let value = value & 0xff;
    e.call(
        ACTOR_BASE_DATA_SET_FLAG,
        &args![this, 0x800u32, value, 1u32],
    );
}

// Translated from 005fa200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: an actor value index
/// and a value): for an index of the 0x400 group (`00406d70`) the low byte
/// of the value goes to `005fa2c0` and virtual slot +0x48 is told 0x200;
/// index 0x38 stores the value clamped to 0..0xFFFF (signed) through
/// `0047ff50` on +0x10C; any other index goes to the base setter
/// `005f12d0`.
pub fn fn_005fa200(e: &mut Engine, this: Ptr, index: u32, value: u32) {
    if e.call(INDEX_IN_GROUP, &args![index, 0x400u32]).u8() != 0 {
        e.call(CREATURE_SETTER_005FA2C0, &args![this, value & 0xff]);
        e.vcall(this.addr(), 0x48, &args![0x200u32]);
    } else if index == 0x38 {
        let signed = value as i32;
        let stored = if signed < 0 {
            0
        } else if signed > 0xffff {
            0xffff
        } else {
            value & 0xffff
        };
        e.call(DATA_SET_SHORT, &args![this.addr() + 0x10c, stored]);
    } else {
        e.call(BASE_SET_ACTOR_VALUE, &args![this, index, value]);
    }
}

// Translated from 005fb180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: change flags): the
/// base size of the changed data (`005f16f0`) plus 3 with flag 0x200. A
/// block guarded by `flags & 0` (4 more) never runs and is left out.
pub fn fn_005fb180(e: &mut Engine, this: Ptr, flags: u32) -> u16 {
    let mut size = e.call(BASE_CHANGED_SIZE, &args![this, flags]).u16();
    if flags & 0x200 != 0 {
        size = size.wrapping_add(3);
    }
    size
}

// Translated from 005fb1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: change flags): the
/// base save (`005f18c0`), then with flag 0x200 the bytes at +0x12D,
/// +0x12E and +0x12F (`TESForm::SaveGameDataOLD`, one byte each). A block
/// guarded by `flags & 0` (the combat style form id) never runs and is
/// left out.
pub fn fn_005fb1e0(e: &mut Engine, this: Ptr, flags: u32) {
    e.call(BASE_SAVE_GAME, &args![this, flags]);
    if flags & 0x200 != 0 {
        for offset in [0x12du32, 0x12e, 0x12f] {
            e.call(
                FORM_SAVE_GAME_DATA,
                &args![this, this.addr() + offset, 1u32],
            );
        }
    }
}

// Translated from 005fb270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: change flags and a
/// second word): the base load (`005f1b30`), then with flag 0x200 the
/// bytes at +0x12D, +0x12E and +0x12F (`TESForm::LoadGameDataOLD`, one
/// byte each). A block guarded by `flags & 0` (the combat style) never
/// runs and is left out.
pub fn fn_005fb270(e: &mut Engine, this: Ptr, flags: u32, second: u32) {
    e.call(BASE_LOAD_GAME, &args![this, flags, second]);
    if flags & 0x200 != 0 {
        for offset in [0x12du32, 0x12e, 0x12f] {
            e.call(
                FORM_LOAD_GAME_DATA,
                &args![this, this.addr() + offset, 1u32],
            );
        }
    }
}

// Translated from 005fb3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: the load-game buffer):
/// the base load (`005f1fd0`), then, when the buffer's change flags have
/// 0x200 (`00428110` with a stack word, `004280f0`), one byte each into
/// +0x12D, +0x12E and +0x12F (`00864980`).
pub fn fn_005fb3b0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(BASE_LOAD_GAME_BUFFER, &args![this, buffer]);
    let changed = e.with_stack(4, |e, scratch| {
        let flags = e.call(BUFFER_FLAGS, &args![buffer, scratch]).u32();
        e.call(FLAGS_TEST, &args![flags, 0x200u32]).u8()
    });
    if changed != 0 {
        for offset in [0x12du32, 0x12e, 0x12f] {
            e.call(BUFFER_READ, &args![buffer, this.addr() + offset, 1u32]);
        }
    }
}

// Translated from 005fbeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the value of the setting for
/// the sound level (+0x154): 0 `011cdde0`, 1 `011cd538`, 2 `011cd3b4`
/// (the word at the address `0043d4d0` answers); 0 for any other level.
pub fn fn_005fbeb0(e: &mut Engine, this: Ptr) -> u32 {
    let setting = match e.mem.u32(this.addr() + 0x154) {
        0 => SETTING_SOUND_LEVEL_0,
        1 => SETTING_SOUND_LEVEL_1,
        2 => SETTING_SOUND_LEVEL_2,
        _ => return 0,
    };
    let value = e.call(SETTING_VALUE_ADDRESS, &args![setting]).u32();
    e.mem.u32(value)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_90c0, fn_005f90c0(Ptr) -> bool),
        entry!(0x005f_90e0, fn_005f90e0(Ptr) -> bool),
        entry!(0x005f_7230, tescreature_tescreature(Ptr) -> Ptr),
        entry!(0x005f_b430, fn_005fb430(Ptr, Ptr)),
        entry!(0x005f_75e0, tesactorbase_tesactorbase(Ptr) -> Ptr),
        entry!(0x005f_78e0, fn_005f78e0(Ptr) -> Ptr),
        entry!(0x005f_7b50, fn_005f7b50(Ptr, u8, u8)),
        entry!(0x005f_7a40, fn_005f7a40(Ptr<TESCreature>)),
        entry!(0x005f_9040, fn_005f9040(Ptr) -> bool),
        entry!(0x005f_9060, fn_005f9060(Ptr) -> bool),
        entry!(0x005f_9080, fn_005f9080(Ptr) -> bool),
        entry!(0x005f_90a0, fn_005f90a0(Ptr) -> bool),
        entry!(0x005f_a2c0, fn_005fa2c0(Ptr, u8)),
        entry!(0x005f_bdb0, fn_005fbdb0(Ptr, u8)),
        entry!(0x005f_a310, fn_005fa310(Ptr<TESCreature>, u32)),
        entry!(0x005f_a2e0, fn_005fa2e0(Ptr<TESCreature>) -> u32),
        entry!(0x005f_7bf0, fn_005f7bf0(Ptr, u32) -> Ptr),
        entry!(0x005f_9100, fn_005f9100(Ptr) -> u32),
        entry!(0x005f_91d0, fn_005f91d0(Ptr) -> u32),
        entry!(0x005f_bf20, tes_creature_is_humanoid_creature(Ptr) -> bool),
        entry!(0x005f_7420, actor_value_owner_get_permanent_actor_float_value(Ptr, u32) -> f32),
        entry!(0x005f_8f10, fn_005f8f10(Ptr) -> u16),
        entry!(0x005f_8e90, fn_005f8e90(Ptr) -> u32),
        entry!(0x005f_8f70, fn_005f8f70(Ptr) -> u8),
        entry!(0x005f_8d70, tes_creature_get_attack_damage(Ptr) -> u16),
        entry!(0x005f_a120, fn_005fa120(Ptr, u32) -> bool),
        entry!(0x005f_7b80, fn_005f7b80(Ptr)),
        entry!(0x005f_92d0, tes_creature_pick_creature_sound(Ptr, u32) -> u32),
        entry!(0x005f_b330, fn_005fb330(Ptr, u32)),
        entry!(0x005f_a0a0, tes_creature_init_worn(Ptr, u32)),
        entry!(0x005f_a180, fn_005fa180(Ptr, u32) -> u32),
        entry!(0x005f_9140, fn_005f9140(Ptr, u32)),
        entry!(0x005f_9210, fn_005f9210(Ptr) -> u32),
        entry!(0x005f_77b0, fn_005f77b0(Ptr)),
        entry!(0x005f_bdd0, fn_005fbdd0(Ptr, u32)),
        entry!(0x005f_7900, fn_005f7900(Ptr)),
        entry!(
            0x005f_9e00,
            tes_creature_init_default_worn(Ptr, u32, u32, u8, u8)
        ),
        entry!(0x005f_9340, tes_creature_init_item(Ptr)),
        entry!(0x005f_7ec0, tes_creature_load(Ptr, Ptr) -> bool),
        entry!(0x005f_b450, fn_005fb450(Ptr, u32)),
        entry!(0x005f_a330, fn_005fa330(Ptr, u32, u32, u32, u32, u32) -> bool),
        entry!(0x005f_78b0, fn_005f78b0(Ptr, u32) -> Ptr),
        entry!(0x005f_7c20, tes_creature_save(Ptr)),
        entry!(0x005f_9010, fn_005f9010(Ptr, u32)),
        entry!(0x005f_a200, fn_005fa200(Ptr, u32, u32)),
        entry!(0x005f_b180, fn_005fb180(Ptr, u32) -> u16),
        entry!(0x005f_b1e0, fn_005fb1e0(Ptr, u32)),
        entry!(0x005f_b270, fn_005fb270(Ptr, u32, u32)),
        entry!(0x005f_b3b0, fn_005fb3b0(Ptr, Ptr)),
        entry!(0x005f_beb0, fn_005fbeb0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tescreature_tescreature_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_7230,
            size: 0x160,
            globals: &[(0x011c_3f2c, 0x1000000), (0x011c_b550, 0x1000001)],
            returns: &[],
            calls: &[
                (0x005f_75e0, &[A::T(0x0)]),
                (0x0047_fdc0, &[A::T(0x10c)]),
                (0x0048_9a60, &[A::T(0x114)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2b)]),
                (0x005f_7a40, &[A::T(0x0)]),
                (0x0047_ef50, &[A::T(0x90), A::V(0x32), A::V(0x0)]),
                (0x0047_ee00, &[A::T(0x90), A::V(0x1), A::V(0x0)]),
                (0x0047_eed0, &[A::T(0x90), A::V(0x2), A::V(0x0)]),
                (0x0047_ef00, &[A::T(0x90), A::V(0x2), A::V(0x0)]),
                (0x0045_16b0, &[A::V(0x1000000)]),
                (0x0047_d800, &[A::T(0x30), A::V(0x1000001), A::V(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_8f5c),
                (0x30, 4, 0x0104_8eec),
                (0x64, 4, 0x0104_8ed8),
                (0x70, 4, 0x0104_8ec4),
                (0x7c, 4, 0x0104_8ea4),
                (0x90, 4, 0x0104_8e84),
                (0xb0, 4, 0x0104_8e6c),
                (0xb8, 4, 0x0104_8e58),
                (0xc4, 4, 0x0104_8e44),
                (0xd0, 4, 0x0104_8e30),
                (0xdc, 4, 0x0104_8e0c),
                (0xf4, 4, 0x0104_8df8),
                (0x100, 4, 0x0104_8dc8),
                (0x104, 4, 0x0104_8db4),
                (0x10c, 4, 0x0104_8d9c),
                (0x114, 4, 0x0104_8d88),
            ],
        });
    }

    #[test]
    fn tescreature_tescreature_skips_the_faction_rank_when_the_check_holds() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_7230,
            size: 0x160,
            globals: &[(0x011c_3f2c, 0x1000000), (0x011c_b550, 0x1000001)],
            returns: &[(0x0045_16b0, 1)],
            calls: &[
                (0x005f_75e0, &[A::T(0)]),
                (0x0047_fdc0, &[A::T(0x10c)]),
                (0x0048_9a60, &[A::T(0x114)]),
                (0x004f_15a0, &[A::T(0), A::V(0x2b)]),
                (0x005f_7a40, &[A::T(0)]),
                (0x0047_ef50, &[A::T(0x90), A::V(0x32), A::V(0)]),
                (0x0047_ee00, &[A::T(0x90), A::V(1), A::V(0)]),
                (0x0047_eed0, &[A::T(0x90), A::V(2), A::V(0)]),
                (0x0047_ef00, &[A::T(0x90), A::V(2), A::V(0)]),
                (0x0045_16b0, &[A::V(0x1000000)]),
            ],
            stores: &[
                (0x00, 4, 0x0104_8f5c),
                (0x30, 4, 0x0104_8eec),
                (0x64, 4, 0x0104_8ed8),
                (0x70, 4, 0x0104_8ec4),
                (0x7c, 4, 0x0104_8ea4),
                (0x90, 4, 0x0104_8e84),
                (0xb0, 4, 0x0104_8e6c),
                (0xb8, 4, 0x0104_8e58),
                (0xc4, 4, 0x0104_8e44),
                (0xd0, 4, 0x0104_8e30),
                (0xdc, 4, 0x0104_8e0c),
                (0xf4, 4, 0x0104_8df8),
                (0x100, 4, 0x0104_8dc8),
                (0x104, 4, 0x0104_8db4),
                (0x10c, 4, 0x0104_8d9c),
                (0x114, 4, 0x0104_8d88),
            ],
        });
    }

    #[test]
    fn forwarder_fn_005fb430_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(INIT_LOAD_GAME_TARGET_009DACE0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_b430, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == INIT_LOAD_GAME_TARGET_009DACE0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tesactorbase_tesactorbase_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_75e0,
            size: 0x110,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0047_c700, &[A::T(0x30)]),
                (0x0048_1610, &[A::T(0x64)]),
                (0x0047_bd10, &[A::T(0x70)]),
                (0x0048_cfe0, &[A::T(0x7c)]),
                (0x0047_ec40, &[A::T(0x90)]),
                (0x0048_71d0, &[A::T(0xb0)]),
                (0x0047_ff70, &[A::T(0xb8)]),
                (0x0047_f9c0, &[A::T(0xc4)]),
                (0x0040_2d00, &[A::T(0xd0)]),
                (0x0048_8f50, &[A::T(0xdc)]),
                (0x0048_cc50, &[A::T(0xf4)]),
                (0x0047_5350, &[A::T(0x104)]),
            ],
            stores: &[
                (0x00, 4, 0x0104_92ac),
                (0x30, 4, 0x0104_923c),
                (0x64, 4, 0x0104_9224),
                (0x70, 4, 0x0104_9210),
                (0x7c, 4, 0x0104_91f0),
                (0x90, 4, 0x0104_91d0),
                (0xb0, 4, 0x0104_91b8),
                (0xb8, 4, 0x0104_91a4),
                (0xc4, 4, 0x0104_9190),
                (0xd0, 4, 0x0104_917c),
                (0xdc, 4, 0x0104_9158),
                (0xf4, 4, 0x0104_9144),
                // `005f78e0` stores 0x0104_9450 first; the constructor overwrites it.
                (0x100, 4, 0x0104_9114),
                (0x104, 4, 0x0104_9100),
            ],
        });
    }

    #[test]
    fn fn_005f78e0_stores_the_vtable() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_78e0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[],
            stores: &[(0x0, 4, 0x0104_9450)],
        });
    }

    #[test]
    fn fn_005f7b50_sets_the_flag_bit_from_the_negated_argument() {
        for (clear, notify, want) in [
            (0u32, 1u32, 1u32),
            (1, 0, 0),
            (0x100, 1, 1),
            (0x1_0001, 0, 0),
        ] {
            let mut e = Engine::new();
            e.register(0x0047_dd50, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x005f_7b50, &[0x5000, clear, notify]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log[1..],
                [(0x0047_dd50, vec![0x5000, 0x200, want, notify & 0xff])],
                "clear {clear:#x}"
            );
        }
    }
}

#[cfg(test)]
mod start_value_tests {
    use super::*;

    fn run(float_bits: u32) -> (Engine, u32, Vec<(u32, Vec<u32>)>) {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(FOOT_WEIGHT_DEFAULT, float_bits);
        for address in [0x0047_dd50, 0x006e_cd40] {
            e.register(address, |_, _| Ret::default());
        }
        let creature = e.mem.alloc(0x160);
        e.mem.write(creature, &[0xcd; 0x160]);
        e.call_log = Some(vec![]);
        e.call(0x005f_7a40, &args![creature]);
        let log = e.call_log.take().unwrap();
        (e, creature, log.into_iter().skip(1).collect())
    }

    #[test]
    fn fn_005f7a40_sets_the_start_values_and_calls_the_components() {
        let (e, c, log) = run(0x4040_0000);
        let bytes = |offset: u32| e.mem.u8(c + offset);
        let word = |offset: u32| e.mem.u32(c + offset);
        assert_eq!(
            [bytes(0x12c), bytes(0x12d), bytes(0x12e), bytes(0x12f)],
            [0, 0x32, 0x32, 0x32]
        );
        assert_eq!(bytes(0x130), 0x20);
        assert_eq!(bytes(0x150), 0);
        assert_eq!(bytes(0x15c), 0xff);
        assert_eq!(word(0x134), 0);
        assert_eq!(word(0x138), 0x4040_0000);
        assert_eq!(word(0x13c), 0x3f80_0000);
        for offset in [0x128, 0x140, 0x144, 0x14c, 0x158] {
            assert_eq!(word(offset), 0, "{offset:#x}");
        }
        assert_eq!(word(0x148), 6);
        assert_eq!(word(0x154), 1);
        // Bytes the function never names keep their value.
        assert_eq!(bytes(0x131), 0xcd);
        assert_eq!(bytes(0x15d), 0xcd);
        assert_eq!(log[0], (0x0047_dd50, vec![c + 0x30, 0x40, 1, 0]));
        assert_eq!(log[1], (0x0047_dd50, vec![c + 0x30, 0x200, 1, 0]));
        assert_eq!(log[2], (0x006e_cd40, vec![c + 0xb0, 0x32]));
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn fn_005f7a40_quiets_a_signalling_nan_foot_weight() {
        let (e, c, _) = run(0x7f80_0001);
        assert_eq!(e.mem.u32(c + 0x138), 0x7fc0_0001);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn check_flag_test(address: u32, mask: u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x40);
        e.register_double(0x0046_1580, move |_, a| u32::from(a[1] == mask).into_ret());
        assert!(e.call(address, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        let tests: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x0046_1580)
            .map(|c| &c.1)
            .collect();
        assert_eq!(tests, vec![&vec![this + 0x30, mask]]);
    }

    #[test]
    fn at_005f90c0_tests_the_flag_0x1000000_of_the_object_at_offset_0x30() {
        check_flag_test(0x005f_90c0, 0x0100_0000);
    }

    #[test]
    fn at_005f90e0_tests_the_flag_0x2000000_of_the_object_at_offset_0x30() {
        check_flag_test(0x005f_90e0, 0x0200_0000);
    }

    #[test]
    fn the_flag_tests_answer_false_when_the_callee_does() {
        let mut e = Engine::new();
        e.register(0x0046_1580, |_, _| Ret::default());
        assert!(!e.call(0x005f_90c0, &args![0x1000u32]).bool());
        assert!(!e.call(0x005f_90e0, &args![0x1000u32]).bool());
    }
}

#[cfg(test)]
mod b024_tests {
    use crate::prelude::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn logged(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    /// `00461580` answers `flags & mask` for the flags word given.
    fn flags_engine(flags: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(0x0046_1580, move |_, a| ret(u32::from(a[1] & flags != 0)));
        e
    }

    /// A vtable at `table` whose slots all lead to doubles at `0x00a0_0000 +
    /// slot` (registered by the test as needed).
    fn vtable(e: &mut Engine, table: u32, object: u32) {
        let slots: Vec<u32> = (0..0x200u32).map(|i| 0x00a0_0000 + 4 * i).collect();
        e.put_vtable(table, &slots);
        e.mem.set_u32(object, table);
    }

    #[test]
    fn flag_getters_pass_their_masks() {
        for (addr, mask) in [
            (0x005f_9040u32, 0x0002_0000u32),
            (0x005f_9060, 0x0001_0000),
            (0x005f_9080, 0x8000),
            (0x005f_90a0, 0x0008_0000),
        ] {
            let mut e = flags_engine(mask);
            let (r, log) = logged(&mut e, addr, &[0x3000]);
            assert!(r.bool());
            assert_eq!(only(&log, 0x0046_1580), vec![vec![0x3000, mask]]);
            let mut e = flags_engine(!mask);
            assert!(!e.call(addr, &[0x3000]).bool());
        }
    }

    #[test]
    fn at_005fa2c0_and_005fbdb0_store_the_data_bytes() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x160);
        e.call(0x005f_a2c0, &[this, 0x1ab]);
        e.call(0x005f_bdb0, &[this, 0x2cd]);
        assert_eq!(e.mem.u8(this + 0x12d), 0xab);
        assert_eq!(e.mem.u8(this + 0x12c), 0xcd);
        assert_eq!(e.mem.u8(this + 0x12e), 0);
    }

    #[test]
    fn at_005fa310_and_005fa2e0_body_part_data() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x160);
        e.map(0x011c_a000, 0x1000);
        e.set_global::<u32>(0x011c_afec, 0x7777);
        assert_eq!(e.call(0x005f_a2e0, &[this]).u32(), 0x7777);
        e.call(0x005f_a310, &[this, 0x1234]);
        assert_eq!(e.mem.u32(this + 0x144), 0x1234);
        assert_eq!(e.call(0x005f_a2e0, &[this]).u32(), 0x1234);
    }

    #[test]
    fn at_005f7bf0_destroys_and_frees_on_bit_0() {
        for (flags, frees) in [(1u32, true), (2, false), (0xffff_ffff, true)] {
            let mut e = Engine::new();
            e.register(0x005f_0380, ok);
            e.register(0x0040_1030, ok);
            let (r, log) = logged(&mut e, 0x005f_7bf0, &[0x3000, flags]);
            assert_eq!(r.u32(), 0x3000);
            assert_eq!(only(&log, 0x005f_0380), vec![vec![0x3000]]);
            assert_eq!(only(&log, 0x0040_1030).len(), usize::from(frees));
        }
    }

    #[test]
    fn at_005f9100_and_005f91d0_read_the_sound_word_by_flag_100() {
        let mut e = flags_engine(0x100);
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + 0x128, 0x55);
        assert_eq!(e.call(0x005f_9100, &[this]).u32(), 0);
        assert_eq!(e.call(0x005f_91d0, &[this]).u32(), 0x55);
        let (_, log) = logged(&mut e, 0x005f_91d0, &[this]);
        assert_eq!(only(&log, 0x0046_1580), vec![vec![this + 0x30, 0x100]]);
        let mut e = flags_engine(0);
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + 0x128, 0x55);
        assert_eq!(e.call(0x005f_9100, &[this]).u32(), 0x55);
        assert_eq!(e.call(0x005f_91d0, &[this]).u32(), 0);
    }

    #[test]
    fn at_005fbf20_humanoid_types() {
        for kind in 0u32..=9 {
            let mut e = Engine::new();
            e.register_double(0x0059_f3a0, move |_, _| ret(0x100 | kind));
            let humanoid = e.call(0x005f_bf20, &[0x3000]).bool();
            assert_eq!(humanoid, matches!(kind, 3 | 4 | 5 | 7), "type {kind}");
        }
    }

    #[test]
    fn at_005f7420_adds_base_and_modifier() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        vtable(&mut e, 0x0090_0000, this);
        e.register(0x00a0_0004, |_, a| st0(f64::from(a[1]) + 0.5));
        e.register(0x00a0_0018, |_, _| st0(1.0 / 3.0));
        let (r, log) = logged(&mut e, 0x005f_7420, &[this, 7]);
        assert_eq!(r.f64(), f64::from((7.5f64 + 1.0 / 3.0) as f32));
        assert_eq!(only(&log, 0x00a0_0004), vec![vec![this, 7]]);
        assert_eq!(only(&log, 0x00a0_0018), vec![vec![this, 7]]);
    }

    fn level_engine(scales: bool, level: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(0x0046_1560, move |_, _| ret(u32::from(scales)));
        e.register_double(0x0047_ded0, move |_, _| ret(level));
        e
    }

    #[test]
    fn at_005f8f10_multiplies_fatigue_by_level() {
        let mut e = level_engine(true, 3);
        e.register(0x004a_8ae0, |_, _| ret(0x1_0050));
        assert_eq!(e.call(0x005f_8f10, &[0x3000]).u32() as u16, 0xf0);
        let mut e = level_engine(true, 0xffff);
        e.register(0x004a_8ae0, |_, _| ret(0x50));
        assert_eq!(e.call(0x005f_8f10, &[0x3000]).u32() as u16, 0x50);
        let mut e = level_engine(false, 3);
        e.register(0x004a_8ae0, |_, _| ret(0x50));
        let (r, log) = logged(&mut e, 0x005f_8f10, &[0x3000]);
        assert_eq!(r.u32() as u16, 0x50);
        assert!(only(&log, 0x0047_ded0).is_empty());
    }

    #[test]
    fn at_005f8e90_multiplies_health_by_level() {
        let mut e = level_engine(true, 4);
        e.register(0x0072_6070, |_, _| ret(0x4100));
        let (r, log) = logged(&mut e, 0x005f_8e90, &[0x30b0]);
        assert_eq!(r.u32(), 0x0400);
        assert_eq!(only(&log, 0x0046_1560), vec![vec![0x3030]]);
        let mut e = level_engine(false, 4);
        e.register(0x0072_6070, |_, _| ret(0x1_4100));
        assert_eq!(e.call(0x005f_8e90, &[0x30b0]).u32(), 0x4100);
    }

    fn setting_engine(e: &mut Engine, setting: u32, value: f32) {
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, value.to_bits());
        e.register_double(0x0040_3e20, move |_, a| {
            ret(if a[0] == setting { cell } else { 0 })
        });
    }

    #[test]
    fn at_005f8f70_adds_the_level_bonus_to_the_combat_skill() {
        let mut e = level_engine(true, 3);
        let this = e.mem.alloc(0x160);
        e.mem.set_u8(this + 0x12d, 0xfe);
        setting_engine(&mut e, 0x011c_b5b4, 1.9);
        // 3 * 1.9 = 5.7, truncated 5; 0xFE + 5 wraps to 3.
        assert_eq!(e.call(0x005f_8f70, &[this]).u8(), 3);
        let mut e = level_engine(false, 3);
        let this = e.mem.alloc(0x160);
        e.mem.set_u8(this + 0x12d, 0x40);
        assert_eq!(e.call(0x005f_8f70, &[this]).u8(), 0x40);
    }

    fn damage_engine(ready: u32, flag: u32, list: u32, scales: bool) -> (Engine, u32) {
        let mut e = level_engine(scales, 2);
        let creature = e.mem.alloc(0x160);
        vtable(&mut e, 0x0090_0000, creature + 0xdc);
        e.register(0x00a0_0014, |_, _| ret(0x6000));
        e.map(0x011c_b000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.set_global::<u32>(0x011c_b6a0, 0x0066_0000);
        e.register_double(0x0084_3150, move |_, _| ret(ready));
        e.register_double(0x0045_16b0, move |_, _| ret(flag));
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, list);
        e.register_double(0x0060_0700, move |_, _| {
            ret(if list == 0xffff { 0 } else { cell })
        });
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0047_ff30, |_, _| ret(0x0001_0020));
        setting_engine(&mut e, 0x011c_b58c, 2.5);
        e.register(0x00ec_62c0, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            ret(v.trunc() as i32 as u32)
        });
        (e, creature + 0x10c)
    }

    #[test]
    fn at_005f8d70_resolves_hand_to_hand_and_adds_the_level_bonus() {
        // Unknown (-1): the root file list holds an item, so 1.
        let (mut e, this) = damage_engine(1, 0, 0x77, true);
        e.mem.set_u8(this + 0x50, 0xff);
        let (r, log) = logged(&mut e, 0x005f_8d70, &[this]);
        assert_eq!(e.mem.u8(this + 0x50), 1);
        assert_eq!(only(&log, 0x00a0_0014), vec![vec![this - 0x30]]);
        assert_eq!(only(&log, 0x0060_0700), vec![vec![0x0066_0000, 0x6000, 1]]);
        // 0x20 + trunc(2 * 2.5).
        assert_eq!(r.u32() as u16, 0x25);
        // An empty list: 0, and the damage is 0.
        let (mut e, this) = damage_engine(1, 0, 0, true);
        e.mem.set_u8(this + 0x50, 0xff);
        assert_eq!(e.call(0x005f_8d70, &[this]).u32() as u16, 0);
        assert_eq!(e.mem.u8(this + 0x50), 0);
        // No list at all: 0.
        let (mut e, this) = damage_engine(1, 0, 0xffff, true);
        e.mem.set_u8(this + 0x50, 0xff);
        e.call(0x005f_8d70, &[this]);
        assert_eq!(e.mem.u8(this + 0x50), 0);
        // Not ready, or the TES flag: the byte stays -1 (nonzero), no level bonus.
        let (mut e, this) = damage_engine(0, 0, 0x77, false);
        e.mem.set_u8(this + 0x50, 0xff);
        assert_eq!(e.call(0x005f_8d70, &[this]).u32() as u16, 0x20);
        assert_eq!(e.mem.u8(this + 0x50), 0xff);
        let (mut e, this) = damage_engine(1, 1, 0x77, false);
        e.mem.set_u8(this + 0x50, 0xff);
        let (_, log) = logged(&mut e, 0x005f_8d70, &[this]);
        assert!(only(&log, 0x0060_0700).is_empty());
        // Known 0: 0 without asking anything else.
        let (mut e, this) = damage_engine(1, 0, 0x77, true);
        e.mem.set_u8(this + 0x50, 0);
        let (r, log) = logged(&mut e, 0x005f_8d70, &[this]);
        assert_eq!(r.u32(), 0);
        assert!(only(&log, 0x0047_ff30).is_empty());
    }

    #[test]
    fn at_005fa120_wearable_types() {
        for (kind, weapon_ok, expected) in [
            (0x1eu32, false, true),
            (0x28, false, true),
            (0x29, false, true),
            (0x27, true, false),
            (0x2a, true, false),
            (0x1a, true, false),
        ] {
            let mut e = Engine::new();
            e.register_double(0x0040_1170, move |_, _| ret(kind));
            e.register_double(0x0046_e8c0, move |_, _| ret(u32::from(weapon_ok)));
            assert_eq!(
                e.call(0x005f_a120, &[0x3000, 0x5000]).bool(),
                expected,
                "type {kind:#x}"
            );
        }
    }

    #[test]
    fn at_005f7b80_frees_own_sounds_and_resets_hand_to_hand() {
        let mut e = flags_engine(0x100);
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + 0x128, 0x5550);
        e.register(0x005f_0380, ok);
        e.register(0x0040_1030, ok);
        let (_, log) = logged(&mut e, 0x005f_7b80, &[this]);
        assert_eq!(only(&log, 0x005f_0380), vec![vec![0x5550]]);
        assert_eq!(only(&log, 0x0040_1030), vec![vec![0x5550]]);
        assert_eq!(e.mem.u32(this + 0x128), 0);
        assert_eq!(e.mem.u8(this + 0x15c), 0xff);
        // Borrowed sounds: the word stays.
        let mut e = flags_engine(0);
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + 0x128, 0x5550);
        let (_, log) = logged(&mut e, 0x005f_7b80, &[this]);
        assert_eq!(log.len(), 2);
        assert_eq!(e.mem.u32(this + 0x128), 0x5550);
        assert_eq!(e.mem.u8(this + 0x15c), 0xff);
    }

    #[test]
    fn at_005f92d0_follows_the_sound_creature_chain() {
        let mut e = Engine::new();
        let first = e.mem.alloc(0x160);
        let second = e.mem.alloc(0x160);
        // First borrows from second; second has its own sounds.
        e.mem.set_u32(first + 0x34, 0);
        e.mem.set_u32(second + 0x34, 0x100);
        e.mem.set_u32(first + 0x128, second);
        e.mem.set_u32(second + 0x128, 0x7000);
        e.register(0x0046_1580, |e, a| {
            ret(u32::from(e.mem.u32(a[0] + 4) & a[1] != 0))
        });
        e.register(0x005f_0a30, |_, a| ret(a[0] + a[1]));
        assert_eq!(e.call(0x005f_92d0, &[first, 3]).u32(), 0x7003);
        e.mem.set_u32(second + 0x128, 0);
        assert_eq!(e.call(0x005f_92d0, &[first, 3]).u32(), 0);
        e.mem.set_u32(first + 0x128, 0);
        assert_eq!(e.call(0x005f_92d0, &[first, 3]).u32(), 0);
    }

    #[test]
    fn at_005fb330_saves_the_data_bytes_with_flag_200() {
        for (flag, saves) in [(0x200u32, true), (0xfdff, false)] {
            let mut e = Engine::new();
            e.register(0x005f_1f30, ok);
            e.register(0x0042_8110, |_, a| ret(a[1]));
            e.register_double(0x0042_80f0, move |_, a| ret(u32::from(a[1] & flag != 0)));
            e.register(0x0086_5e50, ok);
            let (_, log) = logged(&mut e, 0x005f_b330, &[0x3000, 0x8000]);
            assert_eq!(only(&log, 0x005f_1f30), vec![vec![0x3000, 0x8000]]);
            let saved = only(&log, 0x0086_5e50);
            if saves {
                assert_eq!(
                    saved,
                    vec![
                        vec![0x8000, 0x312d, 1, 0],
                        vec![0x8000, 0x312e, 1, 0],
                        vec![0x8000, 0x312f, 1, 0]
                    ]
                );
            } else {
                assert!(saved.is_empty());
            }
        }
    }

    #[test]
    fn at_005fa0a0_hands_the_reference_to_a_high_process() {
        let mut e = Engine::new();
        e.register(0x00ec_43fb, |_, a| {
            ret(if a[0] == 0 { 0 } else { a[0] + 1 })
        });
        e.register(0x008d_8520, |_, a| {
            ret(if a[0] == 0x5001 { 0x0098_0000 } else { 0 })
        });
        e.map(0x0098_0000, 0x1000);
        vtable(&mut e, 0x0090_0000, 0x0098_0001);
        e.register(0x00a0_0544, ok);
        let (_, log) = logged(&mut e, 0x005f_a0a0, &[0x3000, 0x5000]);
        assert_eq!(
            only(&log, 0x00ec_43fb),
            vec![
                vec![0x5000, 0, 0x0118_41cc, 0x0118_4920, 0],
                vec![0x0098_0000, 0, 0x0118_a6b8, 0x0118_c64c, 0]
            ]
        );
        assert_eq!(only(&log, 0x00a0_0544), vec![vec![0x0098_0001, 0x5001]]);
        // No process: nothing more.
        let (_, log) = logged(&mut e, 0x005f_a0a0, &[0x3000, 0x6000]);
        assert_eq!(only(&log, 0x00ec_43fb).len(), 1);
    }

    #[test]
    fn at_005fa180_reads_skills_damage_or_the_base_values() {
        let mut e = level_engine(false, 1);
        let creature = e.mem.alloc(0x160);
        e.mem.set_u8(creature + 0x12d, 0x33);
        let owner = creature + 0x100;
        e.register(0x0040_6d70, |_, a| {
            ret(u32::from(a[0] >= 0x20 && a[0] < 0x2e))
        });
        vtable(&mut e, 0x0090_0000, owner + 0xc);
        e.register(0x00a0_0010, |_, _| ret(0x0001_0044));
        e.register(0x005f_0fb0, |_, a| ret(a[1] + 0x100));
        assert_eq!(e.call(0x005f_a180, &[owner, 0x21]).u32(), 0x33);
        assert_eq!(e.call(0x005f_a180, &[owner, 0x38]).u32(), 0x44);
        let (r, log) = logged(&mut e, 0x005f_a180, &[owner, 5]);
        assert_eq!(r.u32(), 0x105);
        assert_eq!(only(&log, 0x0040_6d70), vec![vec![5, 0x400]]);
    }

    #[test]
    fn at_005f9140_sets_the_sound_creature() {
        let mut e = flags_engine(0x100);
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + 0x128, 0x5550);
        e.register(0x005f_0380, ok);
        e.register(0x0040_1030, ok);
        e.register(0x0047_dd50, ok);
        // Null: nothing changes while the word holds own sounds.
        let (_, log) = logged(&mut e, 0x005f_9140, &[this, 0]);
        assert_eq!(log.len(), 2);
        assert_eq!(e.mem.u32(this + 0x128), 0x5550);
        let (_, log) = logged(&mut e, 0x005f_9140, &[this, 0x6660]);
        assert_eq!(only(&log, 0x005f_0380), vec![vec![0x5550]]);
        assert_eq!(
            only(&log, 0x0047_dd50),
            vec![vec![this + 0x30, 0x100, 0, 1]]
        );
        assert_eq!(e.mem.u32(this + 0x128), 0x6660);
        let mut e = flags_engine(0);
        let this = e.mem.alloc(0x160);
        e.call(0x005f_9140, &[this, 0]);
        assert_eq!(e.mem.u32(this + 0x128), 0);
        e.call(0x005f_9140, &[this, 0x7770]);
        assert_eq!(e.mem.u32(this + 0x128), 0x7770);
    }

    #[test]
    fn at_005f9210_makes_own_sounds_when_missing() {
        let mut e = flags_engine(0);
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + 0x128, 0x4440);
        e.register(0x0047_dd50, ok);
        e.register(0x0040_1000, |_, _| ret(0x5000));
        e.register(0x005f_0360, |_, a| ret(a[0] + 8));
        let (r, log) = logged(&mut e, 0x005f_9210, &[this]);
        assert_eq!(r.u32(), 0x5008);
        assert_eq!(
            only(&log, 0x0047_dd50),
            vec![vec![this + 0x30, 0x100, 1, 1]]
        );
        assert_eq!(only(&log, 0x0040_1000), vec![vec![0x58]]);
        // Own sounds present: returned as they are.
        let mut e = flags_engine(0x100);
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + 0x128, 0x4440);
        let (r, log) = logged(&mut e, 0x005f_9210, &[this]);
        assert_eq!(r.u32(), 0x4440);
        assert_eq!(log.len(), 2);
        // The allocation fails: null.
        let mut e = flags_engine(0x100);
        let this = e.mem.alloc(0x160);
        e.register(0x0047_dd50, ok);
        e.register(0x0040_1000, |_, _| ret(0));
        assert_eq!(e.call(0x005f_9210, &[this]).u32(), 0);
    }

    const ACTOR_BASE_DESTRUCTORS: [(u32, u32); 10] = [
        (0x104, 0x0047_5380),
        (0xdc, 0x0048_9070),
        (0xd0, 0x0059_1300),
        (0xc4, 0x0047_f9f0),
        (0xb8, 0x0047_ffa0),
        (0xb0, 0x0048_7200),
        (0x90, 0x0047_ecb0),
        (0x7c, 0x0048_d060),
        (0x64, 0x0048_1680),
        (0x30, 0x0047_c880),
    ];

    fn destructor_doubles(e: &mut Engine) {
        for (_, addr) in ACTOR_BASE_DESTRUCTORS {
            e.register(addr, ok);
        }
        for addr in [
            0x0050_f740u32,
            0x0048_9ae0,
            0x0047_fdf0,
            0x005f_0380,
            0x0040_1030,
        ] {
            e.register(addr, ok);
        }
    }

    #[test]
    fn at_005f77b0_destroys_the_components_in_reverse() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        let (_, log) = logged(&mut e, 0x005f_77b0, &[0x3000]);
        let mut expected: Log = ACTOR_BASE_DESTRUCTORS
            .iter()
            .map(|(offset, addr)| (*addr, vec![0x3000 + offset]))
            .collect();
        expected.push((0x0050_f740, vec![0x3000]));
        assert_eq!(log.into_iter().skip(1).collect::<Log>(), expected);
    }

    #[test]
    fn at_005f7900_restores_vtables_and_destroys() {
        let mut e = flags_engine(0x100);
        destructor_doubles(&mut e);
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + 0x128, 0x5550);
        let (_, log) = logged(&mut e, 0x005f_7900, &[this]);
        assert_eq!(e.mem.u32(this), 0x0104_8f5c);
        assert_eq!(e.mem.u32(this + 0x114), 0x0104_8d88);
        assert_eq!(e.mem.u8(this + 0x15c), 0xff);
        assert_eq!(only(&log, 0x005f_0380), vec![vec![0x5550]]);
        assert_eq!(only(&log, 0x0048_9ae0), vec![vec![this + 0x114]]);
        assert_eq!(only(&log, 0x0047_fdf0), vec![vec![this + 0x10c]]);
        assert_eq!(only(&log, 0x0050_f740), vec![vec![this]]);
    }

    #[test]
    fn at_005fbdd0_drops_unusable_weapons_once() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x160);
        // Container list at +0x68 (00717e50 gives +4): entries (form at +4).
        let forms = [0x6100u32, 0x6200, 0x6300];
        let entries: Vec<u32> = forms
            .iter()
            .map(|f| {
                let entry = e.mem.alloc(8);
                e.mem.set_u32(entry + 4, *f);
                entry
            })
            .collect();
        let mut node = this + 0x68;
        for (i, entry) in entries.iter().enumerate() {
            e.mem.set_u32(node, *entry);
            if i + 1 < entries.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        e.register(0x0071_7e50, |_, a| ret(a[0] + 4));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0040_1170, |_, a| {
            ret(if a[0] == 0x6300 { 0x1e } else { 0x28 })
        });
        e.register(0x0046_e8c0, |_, _| ret(1));
        e.register(0x0052_2c00, |_, a| ret(a[0]));
        // The weapon 0x6200 is unusable.
        e.register(0x004a_ae30, |_, a| ret(u32::from(a[1] != 0x6200)));
        e.register(0x0048_1790, |e, a| {
            // Remove the entry: unlink its node.
            let mut node = a[0] + 4;
            let mut previous = 0;
            while e.mem.u32(node) != a[1] {
                previous = node;
                node = e.mem.u32(node + 4);
            }
            let next = e.mem.u32(node + 4);
            e.mem.set_u32(previous + 4, next);
            Ret::default()
        });
        let (_, log) = logged(&mut e, 0x005f_bdd0, &[this, 0x9000]);
        assert_eq!(only(&log, 0x0048_1790), vec![vec![this + 0x64, entries[1]]]);
        assert_eq!(only(&log, 0x004a_ae30).len(), 3);
        assert_eq!(e.mem.u8(this + 0x150), 1);
        // Already checked, or no biped data: nothing.
        let (_, log) = logged(&mut e, 0x005f_bdd0, &[this, 0x9000]);
        assert_eq!(log.len(), 1);
        e.mem.set_u8(this + 0x150, 0);
        let (_, log) = logged(&mut e, 0x005f_bdd0, &[this, 0]);
        assert_eq!(log.len(), 1);
    }

    fn worn_engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x160);
        let reference = e.mem.alloc(0x10);
        vtable(&mut e, 0x0090_0000, reference);
        e.map(0x011d_d000, 0x1000);
        e.set_global::<u32>(0x011d_df38, 0x0077_0000);
        e.register(0x0046_23f0, |_, a| ret(if a[1] == 0 { 0x101 } else { 0 }));
        e.register(0x00a0_0100, |_, _| ret(1));
        e.register(0x0057_1b50, ok);
        e.register(0x004b_f220, |_, _| ret(0x4400));
        e.register(0x004b_fe50, ok);
        e.register(0x00ec_43fb, |_, a| ret(a[0] + 1));
        e.register(0x008d_8520, |_, _| ret(0x0098_0000));
        e.map(0x0098_0000, 0x1000);
        e.mem.set_u32(0x0098_0000, 0x0090_0000);
        for slot in [0x160u32, 0x164, 0x168, 0x16c, 0x184] {
            e.register(0x00a0_0000 + slot, ok);
        }
        e.register(0x004c_7400, |_, _| ret(0x4500));
        e.register(0x0044_ddc0, |_, _| ret(0x6100));
        e.register(0x0040_1170, |_, _| ret(0x28));
        e.register(0x0057_5400, |_, _| ret(0));
        e.register(0x0044_59e0, ok);
        e.register(0x008c_26e0, ok);
        (e, this, reference)
    }

    #[test]
    fn at_005f9e00_unequips_and_equips_the_best_item() {
        let (mut e, this, reference) = worn_engine();
        let (_, log) = logged(&mut e, 0x005f_9e00, &[this, reference, 0xdead, 1, 0]);
        assert_eq!(
            only(&log, 0x0046_23f0),
            vec![vec![0x0077_0000, 0], vec![0x0077_0000, 1]]
        );
        assert_eq!(only(&log, 0x0057_1b50), vec![vec![reference]]);
        assert_eq!(only(&log, 0x004b_fe50), vec![vec![0x4400, 1, reference]]);
        assert_eq!(only(&log, 0x00a0_0168), vec![vec![0x0098_0000, 0]]);
        assert_eq!(only(&log, 0x00a0_0160), vec![vec![0x0098_0000, 0, 0, 0]]);
        let best = only(&log, 0x004c_7400);
        assert_eq!(best[0][..2], [0x4400, this]);
        assert_eq!(best[0][3..], [6, 1]);
        assert_eq!(
            only(&log, 0x00a0_0184),
            vec![vec![reference, 0x6100, 1, 0, 0]]
        );
        assert_eq!(only(&log, 0x0044_59e0), vec![vec![0x4500, 1]]);
        assert_eq!(only(&log, 0x008c_26e0), vec![vec![reference + 1, 1]]);

        // Keep the flag, no equip: no flag calls, nothing equipped.
        let (mut e, this, reference) = worn_engine();
        let (_, log) = logged(&mut e, 0x005f_9e00, &[this, reference, 0, 0, 1]);
        assert!(only(&log, 0x0046_23f0).is_empty());
        assert!(only(&log, 0x004c_7400).is_empty());
        assert!(only(&log, 0x0044_59e0).is_empty());

        // Already worn: not equipped again, still freed.
        let (mut e, this, reference) = worn_engine();
        e.register(0x0057_5400, |_, _| ret(1));
        let (_, log) = logged(&mut e, 0x005f_9e00, &[this, reference, 0, 1, 1]);
        assert!(only(&log, 0x00a0_0184).is_empty());
        assert_eq!(only(&log, 0x0044_59e0).len(), 1);

        // Not an actor: no process calls.
        let (mut e, this, reference) = worn_engine();
        e.register(0x00ec_43fb, |_, _| ret(0));
        let (_, log) = logged(&mut e, 0x005f_9e00, &[this, reference, 0, 0, 1]);
        assert!(only(&log, 0x008d_8520).is_empty());
        assert!(only(&log, 0x008c_26e0).is_empty());
    }

    fn init_engine() -> (Engine, u32) {
        let mut e = flags_engine(0);
        let this = e.mem.alloc(0x160);
        vtable(&mut e, 0x0090_0000, this);
        vtable(&mut e, 0x0090_1000, this + 0xb0);
        e.register(0x0040_13e0, |_, _| ret(0));
        for addr in [
            0x0048_cd90u32,
            0x0048_1bb0,
            0x0048_d400,
            0x0047_beb0,
            0x0047_f4c0,
            0x0047_d8b0,
            0x0047_8be0,
            0x0056_5210,
            0x0048_4ab0,
            0x005b_5e40,
            0x00a0_018c,
        ] {
            e.register(addr, ok);
        }
        e.register(0x0048_4e60, |_, _| ret(0x5000));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| ret(a[0] & 0xffff));
        // Ids below 0x8000 resolve to themselves plus 1; others are missing.
        e.register(0x00ec_43fb, |_, a| {
            ret(if a[0] < 0x8000 { a[0] + 1 } else { 0 })
        });
        e.register(0x00a0_0130, |_, a| ret(a[0] + 0x30));
        e.register(0x0084_e3a0, |_, a| ret(a[0] + 0xe3));
        e.register(0x00a0_0188, |e, a| ret(e.mem.u32(a[0] + 0x140)));
        e.register(0x0050_5000, |_, _| ret(0x0bb0));
        e.register(0x0059_bb30, |_, _| ret(0));
        (e, this)
    }

    #[test]
    fn at_005f9340_resolves_the_creature_forms() {
        let (mut e, this) = init_engine();
        e.register(0x00a0_0010, |_, _| ret(10));
        e.mem.set_u32(this + 0x34, 0xffff_ffff);
        e.mem.set_u32(this + 0x128, 0x1100);
        e.mem.set_u32(this + 0x140, 0x1200);
        e.mem.set_u32(this + 0x144, 0x9300);
        e.mem.set_u32(this + 0x14c, 0x1400);
        e.mem.set_u32(this + 0x158, 0x9500);
        let (_, log) = logged(&mut e, 0x005f_9340, &[this]);
        assert_eq!(e.mem.u32(this + 0x34), 0xff7b_ff8f);
        assert_eq!(e.mem.u32(this + 0x128), 0x1101);
        assert_eq!(e.mem.u32(this + 0x140), 0x1201);
        assert_eq!(e.mem.u32(this + 0x144), 0);
        assert_eq!(e.mem.u32(this + 0x14c), 0x1401);
        assert_eq!(e.mem.u32(this + 0x158), 0);
        assert_eq!(
            only(&log, 0x005b_5e40),
            vec![
                vec![0x0104_94f8, 0x0100_9300, this + 0xe3, this + 0x30],
                vec![0x0104_94bc, 0x0100_9500, this + 0x30, this + 0xe3]
            ]
        );
        assert_eq!(only(&log, 0x0048_d400), vec![vec![this + 0x7c, this]]);
        assert!(only(&log, 0x0056_5210).is_empty());
        assert!(only(&log, 0x00a0_018c).is_empty());
        assert_eq!(only(&log, 0x0048_4ab0), vec![vec![this, 1]]);
    }

    #[test]
    fn at_005f9340_drops_a_bad_sound_creature_and_starts_dead() {
        let (mut e, this) = init_engine();
        e.register(0x00a0_0010, |_, _| ret(0));
        // The sound creature resolves to the creature itself.
        let id = this - 1;
        e.register_double(0x0048_39c0, move |_, _| ret(id));
        e.register(0x00ec_43fb, |_, a| ret(a[0] + 1));
        e.mem.set_u32(this + 0x128, 0x42);
        let (_, log) = logged(&mut e, 0x005f_9340, &[this]);
        assert_eq!(e.mem.u32(this + 0x128), 0);
        assert_eq!(
            only(&log, 0x005b_5e40),
            vec![vec![
                0x0104_9598,
                this + 0x30,
                this + 0xe3,
                this + 0x30,
                this + 0xe3
            ]]
        );
        // No combat style: slot 0x18C gets the default.
        assert_eq!(only(&log, 0x00a0_018c), vec![vec![this, 0x0bb0]]);
        assert_eq!(only(&log, 0x0056_5210), vec![vec![this, 1]]);

        // A missing sound creature: the warning names the id.
        let (mut e, this) = init_engine();
        e.register(0x00a0_0010, |_, _| ret(1));
        e.mem.set_u32(this + 0x128, 0x9900);
        let (_, log) = logged(&mut e, 0x005f_9340, &[this]);
        assert_eq!(
            only(&log, 0x005b_5e40),
            vec![vec![0x0104_95e0, 0x0100_9900, this + 0xe3, this + 0x30]]
        );

        // Flag 8: nothing.
        let (mut e, this) = init_engine();
        e.register(0x0040_13e0, |_, _| ret(1));
        let (_, log) = logged(&mut e, 0x005f_9340, &[this]);
        assert_eq!(log.len(), 2);
    }
}

#[cfg(test)]
mod b024_load_tests {
    use crate::prelude::*;
    use std::cell::Cell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn logged(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    fn vtable(e: &mut Engine, table: u32, object: u32) {
        let slots: Vec<u32> = (0..0x200u32).map(|i| 0x00a0_0000 + 4 * i).collect();
        e.put_vtable(table, &slots);
        e.mem.set_u32(object, table);
    }

    /// A file whose chunks are `chunks`; each chunk's word data is `0x100 +
    /// index`, its size `size`. `refuse` makes `CreatureSounds::LoadCreatureSound`
    /// refuse its first call.
    fn load_engine(chunks: Vec<u32>, size: u32, version: u32, refuse: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x160);
        let file = e.mem.alloc(0x300);
        vtable(&mut e, 0x0090_0000, this);
        for slot in [0xe0u32, 0x130, 0x134, 0x18c] {
            e.register(0x00a0_0000 + slot, ok);
        }
        e.register(0x0047_2660, |_, _| ret(0x2b));
        let index = Rc::new(Cell::new(0usize));
        let at = index.clone();
        let list = chunks.clone();
        e.register_double(0x0047_26b0, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let step = index.clone();
        let n = chunks.len();
        e.register_double(0x0047_26f0, move |_, _| {
            step.set(step.get() + 1);
            ret(u32::from(step.get() < n))
        });
        let word = index.clone();
        e.register_double(0x0047_27f0, move |e, a| {
            e.mem.set_u32(a[1], 0x100 + word.get() as u32);
            Ret::default()
        });
        e.register(0x0047_2840, |e, a| {
            e.mem.set_u16(a[1], 0x77);
            Ret::default()
        });
        e.register_double(0x0040_1660, move |_, _| ret(size));
        e.register(0x0047_2890, |e, a| {
            if a[2] == 8 {
                e.mem.set_u32(a[1], 0x5555);
                e.mem.set_u8(a[1] + 4, 3);
            }
            if a[2] == 0x14 {
                e.mem.set_u8(a[1], 2);
                e.mem.set_u8(a[1] + 1, 1);
            }
            Ret::default()
        });
        e.register(0x0040_1680, |_, _| ret(1));
        e.register_double(0x0040_3570, move |_, _| ret(version));
        let mut refused = !refuse;
        e.register_double(0x005f_0480, move |_, _| {
            let r = u32::from(refused);
            refused = true;
            ret(r)
        });
        for addr in [
            0x0048_5110u32,
            0x0048_4ab0,
            0x0048_61f0,
            0x0040_e780,
            0x0046_2230,
            0x0047_f500,
            0x006e_cd40,
            0x0048_cd90,
            0x0047_81e0,
            0x0048_92d0,
            0x0048_7050,
            0x005f_8bb0,
            0x005f_8bd0,
            0x0050_3210,
            0x0047_d800,
            0x0048_d290,
            0x0048_9ce0,
            0x0050_3240,
            0x0047_f0b0,
            0x0047_cb90,
            0x0048_3170,
            0x0047_faa0,
            0x0040_3d30,
            0x005b_5e40,
            0x0047_dd50,
        ] {
            e.register(addr, ok);
        }
        e.register(0x0048_17f0, |_, _| ret(0x0077_0000));
        e.register(0x0040_1000, |_, _| ret(0x0078_0000));
        e.register(0x0040_e690, |_, a| ret(a[0] + 1));
        e.register(0x0046_1580, |_, _| ret(0));
        e.register(0x005f_0360, |_, a| ret(a[0]));
        e.register(0x008f_21d0, |_, _| ret(1));
        e.register(0x0084_e3a0, |_, _| ret(0x42));
        e.map(0x011c_b000, 0x1000);
        e.map(0x0077_0000, 0x1000);
        (e, this, file)
    }

    #[test]
    fn at_005f7ec0_loads_the_creature_chunks() {
        let (mut e, this, file) = load_engine(vec![], 4, 10, false);
        e.register(0x0047_2660, |_, _| ret(0x2a));
        let (r, log) = logged(&mut e, 0x005f_7ec0, &[this, file]);
        assert!(!r.bool());
        assert_eq!(log.len(), 2);

        let chunks = vec![
            0x4d41_4e42, // 0 BNAM
            0x4154_4144, // 1 DATA
            0x344d_414e, // 2 NAM4
            0x354d_414e, // 3 NAM5 (0x103 > 3: 1)
            0x4f54_4e43, // 4 CNTO
            0x4445_4f43, // 5 COED
            0x4b43_5456, // 6 VTCK
            0x4d41_4e50, // 7 PNAM
            0x4d41_4e49, // 8 INAM
            0x4d41_4e52, // 9 RNAM (size 4: a word)
            0x4d41_4e53, // 10 SNAM
            0x5444_4941, // 11 AIDT
            0x5342_4341, // 12 ACBS
            0x544c_5054, // 13 TPLT
            0x5444_5343, // 14 CSDT
            0x4944_5343, // 15 CSDI
            0x544d_4145, // 16 EAMT
            0x5a46_494e, // 17 NIFZ
            0x4f4c_5053, // 18 SPLO
        ];
        let (mut e, this, file) = load_engine(chunks, 4, 5, false);
        e.mem.set_u16(this + 0x4a, 0x1234);
        let (r, log) = logged(&mut e, 0x005f_7ec0, &[this, file]);
        assert!(r.bool());
        assert_eq!(only(&log, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(only(&log, 0x0048_4ab0), vec![vec![this, 0]]);
        let words = only(&log, 0x0047_27f0);
        assert_eq!(words[0][1], this + 0x13c);
        assert_eq!(
            only(&log, 0x0048_61f0),
            vec![vec![this, file, this + 0x12c, 4]]
        );
        assert_eq!(e.mem.u32(this + 0x148), 0x102);
        assert_eq!(e.mem.u32(this + 0x154), 1);
        assert_eq!(only(&log, 0x0048_17f0)[0][0], this + 0x64);
        // COED: the block for the pending entry, loaded, stored at its +8.
        assert_eq!(e.mem.u32(0x0077_0008), 0x0078_0001);
        assert_eq!(only(&log, 0x0040_e780), vec![vec![0x0078_0001, file]]);
        assert_eq!(e.global::<u32>(0x011c_b5f8), 0);
        assert_eq!(e.mem.u32(this + 0x50), 0x106);
        assert_eq!(e.mem.u32(this + 0x144), 0x107);
        assert_eq!(e.mem.u32(this + 0x4c), 0x108);
        assert_eq!(e.mem.u8(this + 0x130), 0x09);
        assert_eq!(only(&log, 0x0047_d800), vec![vec![this + 0x30, 0x5555, 3]]);
        // AIDT at version 5: byte 0 = 2 -> 1 with byte 0xE 1, byte 1 = 4 - 1.
        let ai = only(&log, 0x0047_f0b0);
        assert_eq!(ai[0][0], this + 0x90);
        assert_eq!(only(&log, 0x0047_cb90), vec![vec![this + 0x34]]);
        assert_eq!(e.mem.u16(this + 0x4a), 0);
        assert_eq!(e.mem.u32(this + 0x54), 0x10d);
        // CSDI gets the CSDT word.
        let sound = only(&log, 0x005f_0480);
        assert_eq!(sound[0][1..], [file, 0x10e, this]);
        assert_eq!(only(&log, 0x0048_3170), vec![vec![this + 0x70, 0x77]]);
        assert_eq!(
            only(&log, 0x0048_9ce0),
            vec![vec![this + 0x114, this + 0x114, file]]
        );
        assert_eq!(only(&log, 0x0048_d290), vec![vec![this + 0x7c, 0x112]]);
        assert!(only(&log, 0x005b_5e40).is_empty());
    }

    #[test]
    fn at_005f7ec0_ai_data_fix_ups_and_the_speed_warning() {
        let (mut e, this, file) = load_engine(vec![0x5444_4941], 4, 3, false);
        e.register(0x008f_21d0, |_, _| ret(0x1_0000));
        e.register(0x0047_f0b0, |e, a| {
            // Check the fixed block: byte 0 was 2, +1 below version 4 = 3,
            // then case 3 below version 6: 1 with byte 0xE 2; byte 1: 4 - 1.
            assert_eq!(e.mem.u8(a[1]), 1);
            assert_eq!(e.mem.u8(a[1] + 0xe), 2);
            assert_eq!(e.mem.u8(a[1] + 1), 3);
            Ret::default()
        });
        let (_, log) = logged(&mut e, 0x005f_7ec0, &[this, file]);
        assert_eq!(only(&log, 0x0047_f0b0).len(), 1);
        let warning = only(&log, 0x005b_5e40);
        assert_eq!(warning.len(), 1);
        assert_eq!(warning[0][0], 0x0104_947c);
        assert_eq!(warning[0][2], 0x42);
    }

    #[test]
    fn at_005f7ec0_a_refused_sound_reads_the_same_chunk_again() {
        let (mut e, this, file) = load_engine(vec![0x4644_5343, 0x4d41_4e52], 1, 10, true);
        let (_, log) = logged(&mut e, 0x005f_7ec0, &[this, file]);
        // CSDF twice (refused, then accepted), then RNAM with size 1.
        assert_eq!(only(&log, 0x005f_0480).len(), 2);
        assert_eq!(only(&log, 0x0047_26f0).len(), 2);
        assert_eq!(only(&log, 0x0047_2890), vec![vec![file, this + 0x130, 1]]);
    }

    /// Doubles for the creature constructor and destructor the template copy
    /// runs on its stack creature.
    fn template_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let creature = e.mem.alloc(0x160);
        vtable(&mut e, 0x0090_0000, creature);
        vtable(&mut e, 0x0090_0000, creature + 0x30);
        e.map(0x011c_3000, 0x1000);
        e.map(0x011c_b000, 0x1000);
        e.register(0x0045_16b0, |_, _| ret(1));
        for addr in [
            0x005f_75e0u32,
            0x0047_fdc0,
            0x0048_9a60,
            0x004f_15a0,
            0x005f_7a40,
            0x0047_ef50,
            0x0047_ee00,
            0x0047_eed0,
            0x0047_ef00,
            0x0050_5000,
            0x0081_26e0,
            0x0048_9ae0,
            0x0047_fdf0,
            0x0047_5380,
            0x0048_9070,
            0x0059_1300,
            0x0047_f9f0,
            0x0047_ffa0,
            0x0048_7200,
            0x0047_ecb0,
            0x0048_d060,
            0x0048_1680,
            0x0047_c880,
            0x0050_f740,
            0x0048_cce0,
            0x0048_1c80,
            0x0048_70f0,
            0x0047_dd30,
            0x0048_9430,
            0x0048_9db0,
            0x0047_fb50,
            0x0050_eaa0,
            0x005f_8bb0,
            0x0047_86e0,
            0x0050_f9a0,
            0x0050_f9c0,
            0x0047_e1c0,
            0x005f_8bd0,
            0x0047_dfe0,
            0x005d_1070,
            0x0047_de40,
            0x0047_de70,
            0x0048_0000,
            0x0048_7240,
            0x0047_fe30,
            0x0047_e270,
            0x0047_e010,
            0x005f_9af0,
            0x0050_7780,
            0x0080_3a90,
            0x005f_9b10,
            0x0047_ca20,
            0x0048_d630,
            0x0047_bdd0,
            0x0047_f1e0,
            0x0047_f2d0,
            0x0070_37c0,
            0x0047_cd90,
            0x0047_ccc0,
            0x005f_0640,
            0x0047_dd50,
        ] {
            e.register(addr, ok);
        }
        for slot in [
            0x10u32, 0x58, 0x5c, 0x60, 0x64, 0x68, 0x180, 0x184, 0x188, 0x18c,
        ] {
            e.register(0x00a0_0000 + slot, |_, a| ret(a[0] & 0xfff));
        }
        e.register(0x00a0_0064, |_, _| st0(0.75));
        e.register(0x0071_7e50, |_, a| ret(a[0] + 4));
        e.register(0x0046_1580, |_, _| ret(0));
        e.register(0x0046_1560, |_, _| ret(0));
        e.register(0x0040_3e20, |_, _| ret(0));
        e.register(0x0059_f3a0, |_, _| ret(0x105));
        e.register(0x0044_1110, |_, _| ret(0x11));
        e.register(0x005f_9b30, |_, _| ret(0x9b30));
        e.register(0x005f_9b50, |_, _| ret(0x9b50));
        e.register(0x009c_0070, |_, _| ret(0x0070));
        e.register(0x0047_d370, |_, _| ret(0x1_0370));
        e.register(0x0047_d390, |_, _| ret(0x1_0390));
        e.register(0x0047_d3b0, |_, _| ret(0x1_03b0));
        e.register(0x008f_21d0, |_, _| ret(0x1_21d0));
        e.register(0x005f_9ad0, |_, _| ret(0x1ad));
        e.register(0x0082_1640, |_, _| st0(1.5));
        e.register(0x0056_7470, |_, _| st0(2.5));
        e.register(0x0082_1660, |_, _| st0(3.5));
        e.register(0x0047_d1a0, |e, a| {
            e.mem.set_u16(a[1], 0x99);
            ret(0x0070_0000)
        });
        (e, creature)
    }

    #[test]
    fn at_005fb450_copies_from_the_template_by_flag() {
        let (mut e, creature) = template_engine();
        let data = creature + 0x30;
        let template = e.mem.alloc(0x160);
        vtable(&mut e, 0x0090_0000, template);
        vtable(&mut e, 0x0090_0000, template + 0x30);
        e.mem.set_u32(data + 4, 0xffff_ffff);
        e.mem.set_u32(template + 0x34, 0xffff_ffff);
        e.register_double(0x0059_bb30, move |_, a| {
            ret(if a[0] == data { template } else { 0 })
        });
        e.register(0x0047_cdd0, |_, _| ret(0));
        e.register(0x0047_cca0, |_, _| ret(0));
        // Every template flag set.
        e.register(0x0047_cd40, |_, _| ret(1));
        let (_, log) = logged(&mut e, 0x005f_b450, &[data, 0]);
        assert_eq!(
            only(&log, 0x0048_cce0),
            vec![vec![data + 0xc4, template + 0xf4]]
        );
        assert_eq!(
            only(&log, 0x0048_1c80),
            vec![vec![data + 0x34, template + 0x64]]
        );
        assert_eq!(
            only(&log, 0x0047_dd30),
            vec![vec![data, 0xffff_ffff], vec![data, 0xffff_ffff]]
        );
        assert_eq!(
            only(&log, 0x0048_9db0),
            vec![vec![data + 0xe4, template + 0x114]]
        );
        assert_eq!(
            only(&log, 0x00a0_0184),
            vec![vec![creature, template & 0xfff]]
        );
        assert_eq!(only(&log, 0x005f_8bb0), vec![vec![creature, 0x9b30]]);
        assert_eq!(e.mem.u32(data + 0x124), 0x70);
        assert_eq!(only(&log, 0x0047_e1c0), vec![vec![data, 0.75f32.to_bits()]]);
        assert_eq!(e.mem.u8(creature + 0x12c), 0x05);
        assert_eq!(only(&log, 0x0047_dfe0), vec![vec![data, 0x0370]]);
        assert_eq!(only(&log, 0x008f_21d0)[0], vec![template + 0x30]);
        assert_eq!(only(&log, 0x0047_e270), vec![vec![data, 0x21d0]]);
        assert_eq!(only(&log, 0x005f_9af0), vec![vec![creature, 0xad]]);
        assert_eq!(
            only(&log, 0x0050_7780),
            vec![vec![creature, 1.5f32.to_bits()]]
        );
        assert_eq!(
            only(&log, 0x0048_d630),
            vec![vec![data + 0x4c, template + 0x7c]]
        );
        assert_eq!(
            only(&log, 0x0047_f2d0),
            vec![vec![data + 0x60, template + 0x90]]
        );
        assert_eq!(only(&log, 0x0070_37c0), vec![vec![data, 0x0070_0000]]);
        assert_eq!(only(&log, 0x0047_cd90), vec![vec![data, 0x99]]);
        assert_eq!(only(&log, 0x0047_ccc0), vec![vec![data, 1]]);
        // The stack creature was built and destroyed.
        assert_eq!(only(&log, 0x005f_75e0).len(), 1);
        assert_eq!(only(&log, 0x0050_f740).len(), 1);
        // The sounds were copied twice (flag 6 and the missing sound creature).
        assert!(only(&log, 0x0046_1580).len() >= 2);
    }

    #[test]
    fn at_005fb450_without_templates_does_nothing_and_falls_back_to_the_default() {
        let (mut e, creature) = template_engine();
        let data = creature + 0x30;
        e.register(0x0059_bb30, |_, _| ret(0));
        let (_, log) = logged(&mut e, 0x005f_b450, &[data, 0]);
        assert_eq!(log.len(), 2);

        // A given creature template.
        let (mut e, creature) = template_engine();
        let data = creature + 0x30;
        let given = e.mem.alloc(0x160);
        vtable(&mut e, 0x0090_0000, given);
        vtable(&mut e, 0x0090_0000, given + 0x30);
        e.register(0x0059_bb30, |_, _| ret(0));
        e.register(0x0047_cdd0, |_, _| ret(1));
        e.register(0x0040_1170, |_, _| ret(0x2b));
        e.register(0x0047_cd40, |_, a| ret(u32::from(a[1] == 2)));
        let (_, log) = logged(&mut e, 0x005f_b450, &[data, given]);
        assert_eq!(only(&log, 0x0047_ca20), vec![vec![data, given + 0x30]]);
        assert!(only(&log, 0x0048_cce0).is_empty());

        // A given non-creature: the default (stack) creature is the template.
        let (mut e, creature) = template_engine();
        let data = creature + 0x30;
        e.register(0x0059_bb30, |_, _| ret(0));
        e.register(0x0047_cdd0, |_, _| ret(1));
        e.register(0x0040_1170, |_, _| ret(0x2a));
        e.register(0x0047_cd40, |_, a| ret(u32::from(a[1] == 2)));
        let (_, log) = logged(&mut e, 0x005f_b450, &[data, 0x5000]);
        let copied = only(&log, 0x0047_ca20);
        assert_eq!(copied.len(), 1);
        assert_ne!(copied[0][1], 0x5030);
    }

    #[test]
    fn at_005fb450_lets_a_templated_template_copy_first() {
        let (mut e, creature) = template_engine();
        let data = creature + 0x30;
        let template = e.mem.alloc(0x160);
        vtable(&mut e, 0x0090_0000, template);
        vtable(&mut e, 0x0090_0000, template + 0x30);
        e.register_double(0x0059_bb30, move |_, _| ret(template));
        e.register(0x0047_cdd0, |_, _| ret(0));
        e.register(0x0047_cca0, |_, _| ret(0));
        e.register(0x0047_cd40, |_, _| ret(0));
        let (_, log) = logged(&mut e, 0x005f_b450, &[data, 0]);
        assert_eq!(only(&log, 0x00a0_0010), vec![vec![template + 0x30, 0]]);
    }
}

#[cfg(test)]
mod b024_activate_tests {
    use crate::prelude::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn yes(_: &mut Engine, _: &[u32]) -> Ret {
        ret(1)
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The creature actor `ME`, the activator `WHO`, their processes and the
    /// player's base form; each object's vtable slot `s` leads to a double at
    /// `slots + s` (all answer 0 unless a test says otherwise).
    const ME: u32 = 0x0080_0000;
    const WHO: u32 = 0x0081_0000;
    const MY_PROCESS: u32 = 0x0082_0000;
    const WHO_PROCESS: u32 = 0x0083_0000;
    const BASE: u32 = 0x0084_0000;
    const ME_SLOTS: u32 = 0x00a0_0000;
    const WHO_SLOTS: u32 = 0x00a1_0000;
    const MY_PROCESS_SLOTS: u32 = 0x00a2_0000;
    const WHO_PROCESS_SLOTS: u32 = 0x00a3_0000;
    const BASE_SLOTS: u32 = 0x00a4_0000;

    fn object(e: &mut Engine, at: u32, table: u32, slots: u32) {
        e.map(at, 0x1000);
        let targets: Vec<u32> = (0..0x200u32).map(|i| slots + 4 * i).collect();
        e.put_vtable(table, &targets);
        e.mem.set_u32(at, table);
        for target in targets {
            e.register(target, ok);
        }
    }

    fn engine(player_is_who: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x160);
        object(&mut e, ME, 0x0090_0000, ME_SLOTS);
        object(&mut e, WHO, 0x0091_0000, WHO_SLOTS);
        object(&mut e, MY_PROCESS, 0x0092_0000, MY_PROCESS_SLOTS);
        object(&mut e, WHO_PROCESS, 0x0093_0000, WHO_PROCESS_SLOTS);
        object(&mut e, BASE, 0x0095_0000, BASE_SLOTS);
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(0x011d_ea3c, if player_is_who { WHO } else { 0x0099_0000 });
        e.map(0x0101_6000, 0x1000);
        e.set_global::<u32>(0x0101_62c0, 3.0f32.to_bits());
        e.register(0x00ec_43fb, |_, a| ret(a[0]));
        e.register(0x008d_8520, |_, a| {
            ret(match a[0] {
                ME => MY_PROCESS,
                WHO => WHO_PROCESS,
                _ => 0,
            })
        });
        e.register(0x0041_81e0, |_, _| ret(BASE - 0x30));
        for addr in [
            0x005d_43c0u32,
            0x0041_cb10,
            0x0041_ca90,
            0x0041_cb70,
            0x0041_c930,
            0x0067_0f90,
            0x0086_7d60,
            0x0043_7bd0,
            0x004f_8960,
            0x0059_f3a0,
            0x0040_3df0,
            0x0055_d520,
            0x00ec_623a,
            0x0070_52f0,
            0x008a_6650,
            0x0097_fa10,
            0x0089_4d60,
            0x0070_9470,
            0x008b_06d0,
            0x0087_f3d0,
            0x0049_3bb0,
            0x0049_97b0,
            0x0056_6950,
            0x008a_ce90,
            0x0089_8ff0,
            0x0075_4d90,
            0x0061_a2d0,
            0x0061_b320,
            0x0084_e3a0,
            0x0061_9df0,
            0x0083_c7b0,
            0x0083_c7e0,
            0x0057_b7c0,
            0x009c_8950,
            0x008b_3bb0,
            0x008a_8e50,
            0x0041_d8a0,
            0x0072_6070,
            0x0045_3a70,
            0x00ad_8780,
            0x0081_5b00,
            0x008a_7a90,
            0x0070_2640,
            0x0093_44a0,
            0x008b_b520,
            0x0093_36c0,
            0x005c_90d0,
        ] {
            e.register(addr, ok);
        }
        (e, this)
    }

    fn activate(e: &mut Engine, this: u32, item: u32, count: u32) -> (bool, Log) {
        e.call_log = Some(vec![]);
        let r = e
            .call(0x005f_a330, &[this, ME, WHO, 0xdead, item, count])
            .bool();
        (r, e.call_log.take().unwrap())
    }

    #[test]
    fn at_005fa330_needs_two_actors_and_a_free_process() {
        let (mut e, this) = engine(false);
        e.register(0x00ec_43fb, |_, a| ret(if a[0] == WHO { 0 } else { a[0] }));
        assert!(!activate(&mut e, this, 0, 0).0);
        let (mut e, this) = engine(false);
        e.register(ME_SLOTS + 0x2e8, yes);
        assert!(!activate(&mut e, this, 0, 0).0);
        let (mut e, this) = engine(false);
        e.register(0x008d_8520, |_, _| ret(0));
        assert!(!activate(&mut e, this, 0, 0).0);
        let (mut e, this) = engine(false);
        e.register(MY_PROCESS_SLOTS + 0x52c, yes);
        assert!(!activate(&mut e, this, 0, 0).0);
        let (mut e, this) = engine(false);
        e.register(MY_PROCESS_SLOTS + 0x610, yes);
        assert!(!activate(&mut e, this, 0, 0).0);
    }

    #[test]
    fn at_005fa330_a_dead_creature_shows_the_message() {
        let (mut e, this) = engine(true);
        e.register(ME_SLOTS + 0x230, yes);
        e.register(0x0040_3df0, |_, a| ret(a[0] + 1));
        e.register(0x0055_d520, |_, _| ret(0x7700));
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(!r);
        assert_eq!(only(&log, 0x0040_3df0), vec![vec![0x011d_210c]]);
        let line = only(&log, 0x00ec_623a)[0].clone();
        assert_eq!(line[1..], [0x0101_2058, 0x7700, 0x011d_210d]);
        assert_eq!(
            only(&log, 0x0070_52f0),
            vec![vec![line[0], 0, 0x0104_9638, 0, 3.0f32.to_bits(), 0]]
        );
        // Creature type 6: no message.
        let (mut e, this) = engine(true);
        e.register(0x0043_7bd0, yes);
        e.register(0x0059_f3a0, |_, _| ret(6));
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(!r);
        assert!(only(&log, 0x0070_52f0).is_empty());
    }

    #[test]
    fn at_005fa330_a_fleeing_creature() {
        // Fighting the player, not blocked: true, no message.
        let (mut e, this) = engine(true);
        e.register(0x008a_6650, yes);
        e.register(ME_SLOTS + 0x428, |_, _| ret(0x55));
        e.register(0x0097_fa10, yes);
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(r);
        assert!(only(&log, 0x0070_52f0).is_empty());
        assert_eq!(only(&log, 0x0097_fa10), vec![vec![0x55, WHO]]);
        // Blocked: the "sad" message, false.
        e.register(0x0089_4d60, yes);
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(!r);
        assert_eq!(only(&log, 0x0070_52f0)[0][2], 0x0102_08a0);
    }

    #[test]
    fn at_005fa330_container_creatures_and_attacks() {
        // The player opens the container menu.
        let (mut e, this) = engine(true);
        e.register(ME_SLOTS + 0x22c, yes);
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(r);
        assert_eq!(only(&log, 0x0070_9470), vec![vec![1, ME, 0, 0, 1, 0]]);
        // Another activator gives an item.
        let (mut e, this) = engine(false);
        e.register(ME_SLOTS + 0x22c, yes);
        e.register(0x00ec_43fb, |_, a| {
            ret(if a[3] == 0x0118_3a1c { 0 } else { a[0] })
        });
        let (r, log) = activate(&mut e, this, 0x6000, 3);
        assert!(r);
        assert_eq!(
            only(&log, ME_SLOTS + 0x17c),
            vec![vec![ME, 0x6000, 0, 3, 0, 0, WHO, 0, 0, 1, 0]]
        );
        // An attacker starts combat.
        let (mut e, this) = engine(false);
        e.register(0x008b_06d0, yes);
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(r);
        assert_eq!(only(&log, 0x008b_06d0)[0][..3], [WHO, ME, 0]);
        assert_eq!(
            only(&log, WHO_PROCESS_SLOTS + 0x33c),
            vec![vec![WHO_PROCESS, WHO, ME, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0]]
        );
    }

    #[test]
    fn at_005fa330_hands_over_a_follow_package() {
        let (mut e, this) = engine(false);
        e.register(0x005d_43c0, |_, a| ret(a[0] + 0x44));
        e.register(0x0041_cb10, |_, _| ret(0x6600));
        e.register(0x0041_ca90, |_, _| ret(0xf));
        e.register(0x0041_cb70, |_, _| ret(0x6700));
        e.register(0x0067_0f90, yes);
        e.register(0x0086_7d60, |_, _| ret(0x112));
        let (_, log) = activate(&mut e, this, 0, 0);
        assert_eq!(
            only(&log, 0x0041_c930),
            vec![vec![WHO + 0x44, 0x6600, 4, 0x6700, 0, 0, 0]]
        );
        assert_eq!(
            only(&log, MY_PROCESS_SLOTS + 0x5a0),
            vec![vec![MY_PROCESS, WHO, 0x6600]]
        );
        assert_eq!(only(&log, WHO_SLOTS + 0x28c), vec![vec![WHO, 0x6600, 0x12]]);
        assert_eq!(only(&log, ME_SLOTS + 0x48), vec![vec![ME, 0x8000_0000]]);
    }

    fn dialogue_engine() -> (Engine, u32) {
        let (mut e, this) = engine(true);
        e.register(0x0089_8ff0, yes);
        e.register(0x0061_a2d0, |_, _| ret(0x7100));
        e.register(0x0061_b320, |_, _| ret(0x7200));
        (e, this)
    }

    #[test]
    fn at_005fa330_says_a_single_response_directly() {
        let (mut e, this) = dialogue_engine();
        e.register(0x0061_9df0, yes);
        e.register(0x0083_c7b0, yes);
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(r);
        assert_eq!(
            only(&log, 0x0061_b320),
            vec![vec![0x7100, ME, WHO, 0, 0, 0]]
        );
        assert_eq!(only(&log, 0x0057_b7c0), vec![vec![ME, 0x7100, 0, 0]]);
        assert_eq!(only(&log, 0x005c_90d0), vec![vec![0x7200, 1]]);
        assert!(only(&log, 0x0070_9470).is_empty());
    }

    #[test]
    fn at_005fa330_opens_the_dialogue_menu_for_the_player() {
        let (mut e, this) = dialogue_engine();
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(r);
        assert_eq!(
            only(&log, MY_PROCESS_SLOTS + 0x294),
            vec![vec![MY_PROCESS, ME]]
        );
        assert_eq!(only(&log, 0x008a_8e50), vec![vec![WHO], vec![ME]]);
        assert_eq!(only(&log, 0x00ad_8780)[0][1], 4);
        assert_eq!(only(&log, 0x0081_5b00), vec![vec![ME + 0x88]]);
        assert_eq!(only(&log, 0x009c_8950), vec![vec![0x011f_2250, 0, 0]]);
        // The speaker is the creature (the activator is the player).
        assert_eq!(only(&log, 0x0070_9470), vec![vec![4, ME, 0, 0, 1, 0]]);
        assert_eq!(only(&log, 0x005c_90d0), vec![vec![0x7200, 1]]);
        // With the process's slot 0x4BC at 9: the creature's slot 0x280.
        let (mut e, this) = dialogue_engine();
        e.register(MY_PROCESS_SLOTS + 0x4bc, |_, _| ret(9));
        let (_, log) = activate(&mut e, this, 0, 0);
        assert!(only(&log, 0x0070_9470).is_empty());
        assert_eq!(
            only(&log, ME_SLOTS + 0x280),
            vec![vec![ME, WHO, 0, 0, 1, 0, 0, 0, 0, 0]]
        );
    }

    #[test]
    fn at_005fa330_sets_the_talk_state_from_the_package_schedule() {
        let (mut e, this) = dialogue_engine();
        e.register(ME_SLOTS + 0x2c8, |_, _| ret(WHO));
        e.register(MY_PROCESS_SLOTS + 0x27c, |_, _| ret(0x6800));
        e.register(0x0041_d8a0, |_, _| ret(0x6900));
        let (_, log) = activate(&mut e, this, 0, 0);
        assert_eq!(
            only(&log, MY_PROCESS_SLOTS + 0x288),
            vec![vec![MY_PROCESS, WHO, 2]]
        );
        e.register(0x0072_6070, yes);
        let (_, log) = activate(&mut e, this, 0, 0);
        assert_eq!(
            only(&log, MY_PROCESS_SLOTS + 0x288),
            vec![vec![MY_PROCESS, ME, 1]]
        );
    }

    #[test]
    fn at_005fa330_without_dialogue_is_false() {
        let (mut e, this) = engine(true);
        e.register(0x0089_8ff0, yes);
        let (r, _) = activate(&mut e, this, 0, 0);
        assert!(!r);
        let (mut e, this) = engine(false);
        e.register(0x0089_8ff0, yes);
        // A topic for a non-player activator is never asked: false.
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(!r);
        assert!(only(&log, 0x0061_a2d0).is_empty());
    }

    #[test]
    fn at_005fa330_the_companion_wheel() {
        let (mut e, this) = engine(true);
        e.register(0x0056_6950, yes);
        e.register(0x0075_4d90, yes);
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(!r);
        assert_eq!(only(&log, 0x0070_9470), vec![vec![8, ME, 0, 0, 0, 0]]);
        // 00898ff0 false with the player: true when the wheel does not apply.
        let (mut e, this) = engine(true);
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(r);
        assert!(only(&log, 0x0061_a2d0).is_empty());
    }

    /// The player singleton changes while the dialogue item is made (the
    /// double moves it to `now`), which reaches the branches for an
    /// activator that is not the player.
    fn moved_player_engine(now: u32) -> (Engine, u32) {
        let (mut e, this) = dialogue_engine();
        e.register_double(0x0061_b320, move |e, _| {
            e.set_global::<u32>(0x011d_ea3c, now);
            ret(0x7200)
        });
        let position = e.mem.alloc(12);
        e.mem.set_u32(position, 1);
        e.mem.set_u32(position + 4, 2);
        e.mem.set_u32(position + 8, 3);
        e.register_double(WHO_SLOTS + 0x1f4, move |_, _| ret(position));
        (e, this)
    }

    #[test]
    fn at_005fa330_two_actors_converse() {
        let (mut e, this) = moved_player_engine(0x0099_0000);
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(r);
        assert_eq!(only(&log, 0x008b_b520), vec![vec![ME, 1, 2, 3, 0]]);
        assert_eq!(
            only(&log, WHO_PROCESS_SLOTS + 0x288),
            vec![vec![WHO_PROCESS, WHO, 1]]
        );
        assert_eq!(
            only(&log, WHO_SLOTS + 0x280),
            vec![vec![WHO, ME, 0, 0, 1, 0, 0, 0, 0, 0]]
        );
        assert_eq!(only(&log, 0x005c_90d0), vec![vec![0x7200, 1]]);
        // The conversation starts: both sides get state 2.
        let (mut e, this) = moved_player_engine(0x0099_0000);
        e.register(WHO_SLOTS + 0x280, yes);
        let (_, log) = activate(&mut e, this, 0, 0);
        assert_eq!(
            only(&log, WHO_PROCESS_SLOTS + 0x288),
            vec![vec![WHO_PROCESS, WHO, 1], vec![WHO_PROCESS, WHO, 2]]
        );
        assert_eq!(
            only(&log, MY_PROCESS_SLOTS + 0x288),
            vec![vec![MY_PROCESS, ME, 2]]
        );
        // Already in dialogue: state 1 only.
        let (mut e, this) = moved_player_engine(0x0099_0000);
        e.register(0x0093_36c0, yes);
        let (_, log) = activate(&mut e, this, 0, 0);
        assert_eq!(
            only(&log, WHO_PROCESS_SLOTS + 0x288),
            vec![vec![WHO_PROCESS, WHO, 1]]
        );
        assert!(only(&log, WHO_SLOTS + 0x280).is_empty());
    }

    #[test]
    fn at_005fa330_the_creature_as_the_player_talks_to_the_activator() {
        let (mut e, this) = moved_player_engine(ME);
        let (r, _) = activate(&mut e, this, 0, 0);
        assert!(!r);
        let (mut e, this) = moved_player_engine(ME);
        e.register(MY_PROCESS_SLOTS + 0x3fc, yes);
        e.register(0x0093_44a0, |_, _| ret(0x6a00));
        e.register(0x00ec_43fb, |e, a| {
            if a[3] == 0x0119_9c3c {
                let package = e.mem.alloc(0x90);
                e.mem.set_u32(package + 0x8c, 0x5150);
                ret(package)
            } else {
                ret(a[0])
            }
        });
        let (r, log) = activate(&mut e, this, 0, 0);
        assert!(r);
        assert_eq!(
            only(&log, WHO_PROCESS_SLOTS + 0x288),
            vec![vec![WHO_PROCESS, WHO, 1]]
        );
        assert_eq!(only(&log, 0x0070_9470), vec![vec![4, WHO, 0x5150, 0, 1, 0]]);
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const STATE: u32 = 0x7300_0000;
    const SLOT_180: u32 = 0x7100_0180;
    const SLOT_188: u32 = 0x7100_0188;
    const SLOT_48: u32 = 0x7100_0048;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn creature(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x190);
        e.mem.set_u32(vtable + 0x180, SLOT_180);
        e.mem.set_u32(vtable + 0x188, SLOT_188);
        e.mem.set_u32(vtable + 0x48, SLOT_48);
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this, vtable);
        this
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(STATE, 0x40);
        e
    }

    #[test]
    fn scalar_deleting_destructor() {
        let mut e = engine();
        quiet(&mut e, &[CREATURE_DESTRUCT_005F77B0, FREE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_78b0, &args![0x1000u32, 1u32]).u32(), 0x1000);
        assert_eq!(calls(&e, FREE), vec![vec![0x1000]]);
        e.call_log = Some(vec![]);
        e.call(0x005f_78b0, &args![0x1000u32, 2u32]);
        assert_eq!(calls(&e, CREATURE_DESTRUCT_005F77B0), vec![vec![0x1000]]);
        assert!(calls(&e, FREE).is_empty());
    }

    #[test]
    fn save_writes_every_part() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                SAVE_FORM_START,
                SAVE_FULL_NAME,
                SAVE_MODEL,
                SAVE_SPELL_LIST,
                SAVE_ENCHANTABLE,
                SAVE_MODEL_LIST,
                SAVE_ACTOR_BASE_DATA,
                SAVE_DESTRUCTIBLE,
                SAVE_EMPTY_00483710,
                SAVE_FORM_DATA,
                SAVE_CHUNK_BYTE,
                SAVE_CHUNK_WORD,
                SAVE_FORM_CLOSE,
                TEMPLATE_005F03A0,
            ],
        );
        e.register(SAVE_SWAPS, |e, _| e.mem.u32(STATE).into_ret());
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e.register(SLOT_188, |e, _| e.mem.u32(STATE + 4).into_ret());
        e.register(SLOT_180, |_, _| 0x6000u32.into_ret());
        e.register(FORM_FLAGS_TEST, |e, _| e.mem.u32(STATE + 8).into_ret());
        e.register(CREATURE_TEMPLATE_005F91D0, |_, _| 0x7000u32.into_ret());
        e.register(CREATURE_COMBAT_STYLE_005F9100, |_, _| 0x8000u32.into_ret());
        let this = creature(&mut e);
        e.mem.set_u8(this + 0x130, 0x22);
        e.mem.set_u32(this + 0x134, 0x7f80_0001);
        e.mem.set_f32(this + 0x13c, 1.5);
        e.mem.set_f32(this + 0x138, 2.5);
        e.mem.set_u32(this + 0x148, 4);
        e.mem.set_u32(this + 0x154, 2);
        e.mem.set_u32(this + 0x158, 0x9000);
        e.mem.set_u32(STATE, 1);
        e.call_log = Some(vec![]);
        e.call(0x005f_7c20, &args![this]);
        assert_eq!(
            calls(&e, SAVE_MODEL),
            vec![vec![this + 0xdc, 0x4c44_4f4d, 0x5444_4f4d]]
        );
        assert_eq!(
            calls(&e, SAVE_MODEL_LIST),
            vec![vec![this + 0x114, 0x5a46_494e, 0x5446_494e]]
        );
        assert_eq!(calls(&e, SAVE_ENCHANTABLE), vec![vec![this + 0x70]]);
        assert_eq!(calls(&e, SAVE_EMPTY_00483710).len(), 2);
        assert_eq!(calls(&e, SAVE_FORM_DATA), vec![vec![this, this + 0x12c, 4]]);
        assert_eq!(calls(&e, SAVE_CHUNK_BYTE), vec![vec![0x4d41_4e52, 0x22]]);
        assert_eq!(
            calls(&e, SAVE_CHUNK_WORD),
            vec![
                vec![0x4d41_4e50, 0x6001],
                vec![0x4d41_4e54, 0x7fc0_0001],
                vec![0x4d41_4e42, 1.5f32.to_bits()],
                vec![0x4d41_4e57, 2.5f32.to_bits()],
                vec![0x344d_414e, 4],
                vec![0x354d_414e, 2],
                vec![0x5243_5343, 0x8001],
                vec![0x4d41_4e4c, 0x9001]
            ]
        );
        assert!(calls(&e, TEMPLATE_005F03A0).is_empty());
        // With a ZNAM form, no swapping and actor base flag 0x100.
        e.mem.set_u32(STATE, 0);
        e.mem.set_u32(STATE + 4, 0x5000);
        e.mem.set_u32(STATE + 8, 1);
        e.call_log = Some(vec![]);
        e.call(0x005f_7c20, &args![this]);
        assert!(calls(&e, SAVE_EMPTY_00483710).is_empty());
        assert_eq!(calls(&e, SAVE_CHUNK_WORD)[0], vec![0x4d41_4e5a, 0x5001]);
        assert_eq!(calls(&e, TEMPLATE_005F03A0), vec![vec![0x7000]]);
        assert!(calls(&e, SAVE_CHUNK_WORD)
            .iter()
            .all(|c| c[0] != 0x5243_5343));
    }

    #[test]
    fn flag_setter_forwards() {
        let mut e = engine();
        quiet(&mut e, &[ACTOR_BASE_DATA_SET_FLAG]);
        e.call_log = Some(vec![]);
        e.call(0x005f_9010, &args![0x1000u32, 0x1234_5601u32]);
        assert_eq!(
            calls(&e, ACTOR_BASE_DATA_SET_FLAG),
            vec![vec![0x1000, 0x800, 1, 1]]
        );
    }

    #[test]
    fn actor_value_setter_branches() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                CREATURE_SETTER_005FA2C0,
                SLOT_48,
                DATA_SET_SHORT,
                BASE_SET_ACTOR_VALUE,
            ],
        );
        e.register(INDEX_IN_GROUP, |_, a| u32::from(a[0] == 0x20).into_ret());
        let this = creature(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005f_a200, &args![this, 0x20u32, 0x1ffu32]);
        assert_eq!(calls(&e, INDEX_IN_GROUP)[0], vec![0x20, 0x400]);
        assert_eq!(calls(&e, CREATURE_SETTER_005FA2C0), vec![vec![this, 0xff]]);
        assert_eq!(calls(&e, SLOT_48), vec![vec![this, 0x200]]);
        e.call_log = Some(vec![]);
        for value in [0xffff_fff0u32, 0x1_0000, 0x1234] {
            e.call(0x005f_a200, &args![this, 0x38u32, value]);
        }
        assert_eq!(
            calls(&e, DATA_SET_SHORT),
            vec![
                vec![this + 0x10c, 0],
                vec![this + 0x10c, 0xffff],
                vec![this + 0x10c, 0x1234]
            ]
        );
        e.call(0x005f_a200, &args![this, 0x10u32, 5u32]);
        assert_eq!(calls(&e, BASE_SET_ACTOR_VALUE), vec![vec![this, 0x10, 5]]);
    }

    #[test]
    fn change_size_save_and_loads() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                BASE_SAVE_GAME,
                BASE_LOAD_GAME,
                BASE_LOAD_GAME_BUFFER,
                FORM_SAVE_GAME_DATA,
                FORM_LOAD_GAME_DATA,
                BUFFER_READ,
            ],
        );
        e.register(BASE_CHANGED_SIZE, |_, _| 0x1_0010u32.into_ret());
        assert_eq!(e.call(0x005f_b180, &args![0x1000u32, 0x200u32]).u16(), 0x13);
        assert_eq!(e.call(0x005f_b180, &args![0x1000u32, 0x100u32]).u16(), 0x10);
        e.call_log = Some(vec![]);
        e.call(0x005f_b1e0, &args![0x1000u32, 0x200u32]);
        e.call(0x005f_b270, &args![0x1000u32, 0x200u32, 7u32]);
        assert_eq!(calls(&e, BASE_SAVE_GAME), vec![vec![0x1000, 0x200]]);
        assert_eq!(calls(&e, BASE_LOAD_GAME), vec![vec![0x1000, 0x200, 7]]);
        assert_eq!(
            calls(&e, FORM_SAVE_GAME_DATA),
            vec![
                vec![0x1000, 0x112d, 1],
                vec![0x1000, 0x112e, 1],
                vec![0x1000, 0x112f, 1]
            ]
        );
        assert_eq!(calls(&e, FORM_LOAD_GAME_DATA).len(), 3);
        e.call_log = Some(vec![]);
        e.call(0x005f_b1e0, &args![0x1000u32, 0x100u32]);
        e.call(0x005f_b270, &args![0x1000u32, 0x100u32, 7u32]);
        assert!(calls(&e, FORM_SAVE_GAME_DATA).is_empty());
        assert!(calls(&e, FORM_LOAD_GAME_DATA).is_empty());
        // The buffer load: the flags test passes, then fails.
        e.register(BUFFER_FLAGS, |_, a| (a[0] + 0x40).into_ret());
        e.register(FLAGS_TEST, |e, _| e.mem.u32(STATE).into_ret());
        e.mem.set_u32(STATE, 1);
        e.call_log = Some(vec![]);
        e.call(0x005f_b3b0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls(&e, BASE_LOAD_GAME_BUFFER), vec![vec![0x1000, 0x2000]]);
        assert_eq!(calls(&e, FLAGS_TEST), vec![vec![0x2040, 0x200]]);
        assert_eq!(
            calls(&e, BUFFER_READ),
            vec![
                vec![0x2000, 0x112d, 1],
                vec![0x2000, 0x112e, 1],
                vec![0x2000, 0x112f, 1]
            ]
        );
        e.mem.set_u32(STATE, 0);
        e.call_log = Some(vec![]);
        e.call(0x005f_b3b0, &args![0x1000u32, 0x2000u32]);
        assert!(calls(&e, BUFFER_READ).is_empty());
    }

    #[test]
    fn sound_level_setting() {
        let mut e = engine();
        e.register(SETTING_VALUE_ADDRESS, |_, a| {
            (0x7300_0000 + (a[0] & 0xf)).into_ret()
        });
        e.mem.set_u32(STATE, 11);
        e.mem.set_u32(STATE + 4, 22);
        e.mem.set_u32(STATE + 8, 33);
        let this = e.mem.alloc(0x160);
        let mut answers = vec![];
        for level in 0..4u32 {
            e.mem.set_u32(this + 0x154, level);
            answers.push(e.call(0x005f_beb0, &args![this]).u32());
        }
        // 011cdde0 & 0xf = 0, 011cd538 & 0xf = 8, 011cd3b4 & 0xf = 4.
        assert_eq!(answers, vec![11, 33, 22, 0]);
    }
}

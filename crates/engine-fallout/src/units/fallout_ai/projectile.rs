//! `fallout/ai/combat/projectile.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `Projectile` (Xbox PDB) is a `MobileObject` (0x88 bytes on PC); its own
//! fields follow at the PC offsets the constructors (`009bbc70`, `009bbef0`)
//! write. The PC `NiTransform` is 0x34 bytes (the Xbox build pads it to 16-byte
//! alignment), so every field after `kFollowOffset` sits 0x28 lower than the
//! Xbox PDB says. `Projectile::ImpactData` (Xbox PDB) is 0x30 bytes on both.
//!
//! The functions are translated in address order across several sessions;
//! the first session stopped before `009c3d90` (the next function to
//! translate).
//!
//! Functions with a C++ exception frame (`FS:[0]`) are translated in full
//! except for the unwinding, which the engine does not model; each carries a
//! `Translation incomplete` line saying so.

// `!(a < b)` mirrors the exe's x87 flag tests, which are true for NaN.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `Projectile` (Xbox PDB). The PC size is not confirmed by an allocation
    /// of the class itself; its last field (`fSoundDist`) ends at +0x150 and
    /// a subclass allocates 0x154 bytes (`009bd5c0`).
    pub struct Projectile: 0x150 {
        /// `MobileObject::pCurrentProcess` (Xbox PDB), cleared by the constructors.
        0x68 pCurrentProcess: u32,
        /// `bImpactProcessed` (Xbox PDB).
        0x90 bImpactProcessed: bool,
        /// `iFlags` (Xbox PDB), tested by `00444d00`.
        0xC8 iFlags: u32,
        /// `fPower` (Xbox PDB).
        0xCC fPower: f32,
        /// `fSpeedMult` (Xbox PDB).
        0xD0 fSpeedMult: f32,
        /// `fRange` (Xbox PDB).
        0xD4 fRange: f32,
        /// `fAge` (Xbox PDB).
        0xD8 fAge: f32,
        /// `fDamage` (Xbox PDB).
        0xDC fDamage: f32,
        /// `fAlpha` (Xbox PDB).
        0xE0 fAlpha: f32,
        /// `fExplosionTimer` (Xbox PDB).
        0xE4 fExplosionTimer: f32,
        /// `fBlinkTimer` (Xbox PDB).
        0xE8 fBlinkTimer: f32,
        /// `fSpreadHeading` (Xbox PDB).
        0xEC fSpreadHeading: f32,
        /// `fSpreadLooking` (Xbox PDB).
        0xF0 fSpreadLooking: f32,
        /// `fWeaponCondition` (Xbox PDB).
        0xF4 fWeaponCondition: f32,
        /// `pWeaponSource` (Xbox PDB): `TESObjectWEAP*` (read by `00476c90`).
        0xF8 pWeaponSource: u32,
        /// `pShooter` (Xbox PDB): `TESObjectREFR*` (read by `00874480`).
        0xFC pShooter: u32,
        /// `pDesiredTarget` (Xbox PDB): `TESObjectREFR*` (read by `009b4440`).
        0x100 pDesiredTarget: u32,
        /// `fDistanceMoved` (Xbox PDB).
        0x110 fDistanceMoved: f32,
        /// `bSendAlarm` (Xbox PDB).
        0x118 bSendAlarm: bool,
        /// `fDecalSize` (Xbox PDB).
        0x124 fDecalSize: f32,
        /// `pTarget` (Xbox PDB): `ProjectileTarget*`.
        0x140 pTarget: u32,
        /// `pSecondaryItemChange` (Xbox PDB): `ItemChange*`.
        0x144 pSecondaryItemChange: u32,
        /// `bSoundFired` (Xbox PDB).
        0x148 bSoundFired: bool,
        /// `fSoundDist` (Xbox PDB).
        0x14C fSoundDist: f32,
    }

    /// `Projectile::ImpactData` (Xbox PDB), 0x30 bytes.
    pub struct ImpactData: 0x30 {
        /// `pCollidee` (Xbox PDB): `TESObjectREFR*`.
        0x00 pCollidee: u32,
        /// `kLocation` (Xbox PDB): `NiPoint3`, x.
        0x04 kLocationX: f32,
        /// `kNormal` (Xbox PDB): `NiPoint3`, x.
        0x10 kNormalX: f32,
        /// `pTargetWorldObject` (Xbox PDB): `hkpWorldObject*`.
        0x1C pTargetWorldObject: u32,
        /// `eMaterial` (Xbox PDB).
        0x20 eMaterial: u32,
        /// `eDamageLimb` (Xbox PDB).
        0x24 eDamageLimb: u32,
        /// `bProcessed` (Xbox PDB).
        0x28 bProcessed: bool,
        /// `bDoDamage` (Xbox PDB).
        0x29 bDoDamage: bool,
        /// `sTargetWorldObjectCount` (Xbox PDB).
        0x2A sTargetWorldObjectCount: i16,
        /// `sTargetWorldObjectIndex` (Xbox PDB).
        0x2C sTargetWorldObjectIndex: i16,
    }
}

/// `ImpactList` (Xbox PDB): `BSSimpleList<Projectile::ImpactData*>` at +0x88.
pub(crate) const IMPACT_LIST: u32 = 0x88;
/// `kFollowOffset` (Xbox PDB): `NiTransform` (0x34 bytes on PC) at +0x94.
pub(crate) const FOLLOW_OFFSET: u32 = 0x94;
/// `MovementDirection` (Xbox PDB): `NiPoint3` at +0x104.
pub(crate) const MOVEMENT_DIRECTION: u32 = 0x104;
/// `spLight` (Xbox PDB): `NiPointer<NiLight>` at +0x114.
pub(crate) const LIGHT: u32 = 0x114;
/// `spPickCache` (Xbox PDB): `NiPointer<ProjectilePickCache>` at +0x11C.
pub(crate) const PICK_CACHE: u32 = 0x11C;
/// `spActorCause` (Xbox PDB): `NiPointer<ActorCause>` at +0x120.
pub(crate) const ACTOR_CAUSE: u32 = 0x120;
/// `sndHandle` (Xbox PDB): `BSSoundHandle` (0xC bytes) at +0x128.
pub(crate) const SOUND_HANDLE: u32 = 0x128;
/// `sndCountdown` (Xbox PDB): `BSSoundHandle` at +0x134.
pub(crate) const COUNTDOWN_SOUND: u32 = 0x134;

/// `TESObjectREFR` rotation (`data` +4, PC +0x24): x, y (+0x28), z (+0x2C).
pub(crate) const REFR_ANGLE_X: u32 = 0x24;
pub(crate) const REFR_ANGLE_Z: u32 = 0x2C;
/// `TESObjectREFR` position (`data` +0x10, PC +0x30).
pub(crate) const REFR_POSITION: u32 = 0x30;

/// The projectile's primary vtable and the `TESChildCell` base's vtable.
const VTABLE: u32 = 0x0109_00dc;
const CHILD_CELL_VTABLE: u32 = 0x0109_00d0;

/// `MemoryManager` allocation (`00401000`, cdecl size) and release
/// (`00401030`, cdecl pointer).
const ALLOCATE: u32 = 0x0040_1000;
const DEALLOCATE: u32 = 0x0040_1030;
/// Tests `iFlags & mask` (`00444d00`).
const TEST_FLAGS: u32 = 0x0044_4d00;
/// Sets or clears flag bits (`005de0a0`: mask, set).
const SET_FLAGS: u32 = 0x005d_e0a0;
/// The projectile's source weapon (`00476c90`, reads +0xF8).
const GET_WEAPON: u32 = 0x0047_6c90;
/// The projectile's shooter (`00874480`, reads +0xFC).
const GET_SHOOTER: u32 = 0x0087_4480;
/// The projectile's desired target (`009b4440`, reads +0x100).
const GET_DESIRED_TARGET: u32 = 0x009b_4440;
/// The reference's base object (`004181e0`).
const GET_BASE_OBJECT: u32 = 0x0041_81e0;
/// `TESForm` form id getter (`0084e3a0`, reads +0xC).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// The value pointer of a `Setting` (`00403e20`).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// Pointer to a `bool` setting's value (`00408d60`).
const BOOL_SETTING_VALUE: u32 = 0x0040_8d60;
/// `NiPointer<T>::operator=` on `spLight` / `spPickCache` (`0066b0d0`).
const ASSIGN_NI_POINTER: u32 = 0x0066_b0d0;
/// `NiPointer<ActorCause>::operator=` (Xbox PDB) on `spActorCause` (`0042f780`).
const ASSIGN_ACTOR_CAUSE: u32 = 0x0042_f780;
/// `MiddleHighProcess::GetSavedAcquireObject` per the engine map (`008d8520`);
/// called on an actor here (name unconfirmed for this use).
const ACTOR_PROCESS_OBJECT: u32 = 0x008d_8520;
/// The thread-unsafe "copy of a `NiPoint3`" pattern: three words.
const NI_POINT3_SIZE: u32 = 0xC;

/// `PlayerCharacter*` singleton.
const PLAYER: u32 = 0x011d_ea3c;
/// `TES*` singleton.
const TES: u32 = 0x011d_ea10;
/// The `VATS` singleton (object at this address).
const VATS: u32 = 0x011f_2250;
/// `ProcessLists` singleton (object at this address).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// A global `NiPoint3` the constructors copy into `MovementDirection`.
const ZERO_POINT: u32 = 0x011f_426c;
/// `FLT_MAX` (`float`): the initial `fExplosionTimer`.
const FLOAT_MAX: u32 = 0x0101_6970;
/// `10000.0` (`float`): the initial `fSoundDist`.
const INITIAL_SOUND_DISTANCE: u32 = 0x0102_2958;
/// `double` multiplier of the damage when the weapon's mod effect 0xC is active.
const MOD_DAMAGE_SCALE: u32 = 0x0105_1678;

/// Settings read through [`SETTING_VALUE`].
const SETTING_ROCKIT_DAMAGE: u32 = 0x011c_ed70;
const SETTING_ROCKIT_WEIGHT: u32 = 0x011c_f6b4;
const SETTING_EXPLOSION_TIMER: u32 = 0x011c_e6cc;
const SETTING_ALARM_DISTANCE: u32 = 0x011c_ed88;
/// `bool` settings read through [`BOOL_SETTING_VALUE`].
const SETTING_DEBUG_VATS_TARGETING: u32 = 0x011f_20b8;
const SETTING_DEBUG_IMPACTS: u32 = 0x011f_20ec;
/// `float` setting: lifetime of the debug objects (`TES::AddTempDebugObject`).
const SETTING_DEBUG_OBJECT_TIME: u32 = 0x011f_210c;

fn test_flags(e: &mut Engine, this: Ptr<Projectile>, mask: u32) -> bool {
    e.call(TEST_FLAGS, &args![this, mask]).bool()
}

fn set_flags(e: &mut Engine, this: Ptr<Projectile>, mask: u32, set: bool) {
    e.call(SET_FLAGS, &args![this, mask, set]);
}

/// `IsActor` (virtual +0x100).
fn is_actor(e: &mut Engine, object: u32) -> bool {
    e.vcall(object, 0x100, &args![]).bool()
}

/// The `float` a setting holds, through `FLD`/`FSTP` (quieted).
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_quiet(e.mem.f32(value))
}

fn bool_setting(e: &mut Engine, setting: u32) -> bool {
    let value = e.call(BOOL_SETTING_VALUE, &args![setting]).u32();
    e.mem.u8(value) != 0
}

fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(ALLOCATE, &args![size]).u32()
}

fn copy_point(e: &mut Engine, destination: u32, source: u32) {
    for offset in [0, 4, 8] {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(destination + offset, word);
    }
}

fn point_words(e: &Engine, point: u32) -> [u32; 3] {
    [e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8)]
}

/// `TESWorldSpace::GetParentWorldSimple` (`004fd380`, `thiscall`; here on
/// the base object), `TESObjectLIGH::GenDynamic` (`0050d810`, `thiscall
/// (light, reference, base, 0)`) and the light test `00440d80`
/// (`thiscall`, `bool`).
const BASE_LIGHT: u32 = 0x004f_d380;
const GEN_DYNAMIC: u32 = 0x0050_d810;
const LIGHT_TEST: u32 = 0x0044_0d80;
/// The pointer getter (`00559450`, `thiscall`, answers the word at `this`).
const POINTER_GET: u32 = 0x0055_9450;

// Translated from 009c3d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the projectile's light (the engine map has no name): when its
/// light base (vtable +0x1D0) is set, the base object (`004181e0`) has a
/// light (`004fd380`) and `00440d80` answers false, `spLight` is assigned
/// the dynamic light that light makes (`0050d810` with the projectile, the
/// light base and 0).
pub fn fn_009c3d90(e: &mut Engine, this: Ptr<Projectile>) {
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() == 0 {
        return;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if e.call(BASE_LIGHT, &args![base]).u32() == 0 {
        return;
    }
    if e.call(LIGHT_TEST, &args![this]).bool() {
        return;
    }
    let light_base = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let light = e.call(BASE_LIGHT, &args![base]).u32();
    let dynamic = e
        .call(GEN_DYNAMIC, &args![light, this, light_base, 0u32])
        .u32();
    e.call(ASSIGN_NI_POINTER, &args![this.addr() + LIGHT, dynamic]);
}

// Translated from 009c3e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the projectile's light (the engine map has no name): with
/// `spLight` set, the scene node `00450b80(0)` loses it
/// (`ShadowSceneNode::RemoveLight`, `00b5eed0`) and `spLight` is cleared.
pub fn fn_009c3e10(e: &mut Engine, this: Ptr<Projectile>) {
    let slot = this.addr() + LIGHT;
    if e.call(POINTER_GET, &args![slot]).u32() == 0 {
        return;
    }
    let light = e.call(POINTER_GET, &args![slot]).u32();
    let scene = e.call(0x0045_0b80, &args![0u32]).u32();
    e.call(0x00b5_eed0, &args![scene, light]);
    e.call(ASSIGN_NI_POINTER, &args![slot, 0u32]);
}

// Translated from 009c3e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fades the projectile's light with its alpha (the engine map has no
/// name): with `spLight` set and flag 0x20, the light gets (`0050dd20`)
/// the alpha clamped to at most 1.0 (`0040ebd0`) and at least 0.0
/// (`00404010`), each result stored as a `float`.
pub fn fn_009c3e60(e: &mut Engine, this: Ptr<Projectile>) {
    let slot = this.addr() + LIGHT;
    if e.call(POINTER_GET, &args![slot]).u32() == 0 || !test_flags(e, this, 0x20) {
        return;
    }
    let alpha = x87_quiet_bits(e.mem.u32(this.addr() + 0xe0));
    let capped = e.call(0x0040_ebd0, &args![1.0f32, alpha]).f64();
    let capped = x87_store32(capped);
    let clamped = e.call(0x0040_4010, &args![0.0f32, capped]).f64();
    let clamped = x87_store32(clamped);
    let light = e.call(POINTER_GET, &args![slot]).u32();
    e.call(0x0050_dd20, &args![light, clamped]);
}

/// The setting `009c3f50` scales by (`011ce1d0`).
const SETTING_009C3F50: u32 = 0x011c_e1d0;

// Translated from 009c3f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A blend of a value (the engine map has no name; `cdecl (value, mix,
/// scale)`, the result in ST0): `(setting * (1 - mix) + value * mix) *
/// scale` with the setting at `011ce1d0`, each partial kept in the x87
/// order (the two products kept as `double`s), the result stored as a
/// `float`.
pub fn fn_009c3f50(e: &mut Engine, value: f32, mix: f32, scale: f32) -> f32 {
    let mixed = x87_mul(x87_load32(value.to_bits()), x87_m32(mix.to_bits()));
    let rest = x87_sub(1.0, x87_load32(mix.to_bits()));
    let setting = e.call(SETTING_VALUE, &args![SETTING_009C3F50]).u32();
    let setting = x87_load32(e.mem.u32(setting));
    let blended = x87_add(x87_mul(setting, rest), mixed);
    let result = x87_store32(x87_mul(blended, x87_m32(scale.to_bits())));
    f32::from_bits(x87_quiet_bits(result))
}

// Translated from 009c4e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `MovementDirection` (the engine map has no name; `thiscall (x, y,
/// z, add distance)`, `RET 0x10`): the three words stored; with `add
/// distance` it is unitized and its length (`NiPoint3::UnitizeGetLength`,
/// ST0) added to `fDistanceMoved`; otherwise only unitized (`004a0c10`).
pub fn fn_009c4e60(
    e: &mut Engine,
    this: Ptr<Projectile>,
    x: u32,
    y: u32,
    z: u32,
    add_distance: bool,
) {
    let direction = this.addr() + MOVEMENT_DIRECTION;
    e.mem.set_u32(direction, x);
    e.mem.set_u32(direction + 4, y);
    e.mem.set_u32(direction + 8, z);
    if add_distance {
        let length = e.call(0x0045_7910, &args![direction]).f64();
        let moved = this.addr() + 0x110;
        let total = x87_add(length, x87_m32(e.mem.u32(moved)));
        e.mem.set_u32(moved, x87_store32(total));
    } else {
        e.call(0x004a_0c10, &args![direction]);
    }
}

// Translated from 009c4f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::StopCountdownSound` (Xbox PDB): a valid countdown sound
/// (`BSSoundHandle::IsValid`) is stopped and, still valid, released.
pub fn projectile_stop_countdown_sound(e: &mut Engine, this: Ptr<Projectile>) {
    let handle = this.addr() + COUNTDOWN_SOUND;
    if !e.call(0x00ad_8ce0, &args![handle]).bool() {
        return;
    }
    e.call(0x00ad_88f0, &args![handle]);
    if e.call(0x00ad_8ce0, &args![handle]).bool() {
        e.call(0x00ad_8d10, &args![handle]);
    }
}

// Translated from 009c4da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the projectile's impacts and owners (the engine map has no
/// name): every impact on `ImpactList` is deleted (`009b3490` with 1) and
/// taken off (`0063f7b0`) until the list is empty (`008256d0`);
/// `spActorCause` is cleared; the object `008d8b80` answers, if any, is
/// deleted (`009bc890` with 1), and `008126e0` runs with 0.
pub fn fn_009c4da0(e: &mut Engine, this: Ptr<Projectile>) {
    let list = this.addr() + IMPACT_LIST;
    while !e.call(0x0082_56d0, &args![list]).bool() {
        let head = e.call(0x0068_15c0, &args![list]).u32();
        let impact = e.mem.u32(head);
        if impact != 0 {
            e.call(0x009b_3490, &args![impact, 1u32]);
        }
        e.call(0x0063_f7b0, &args![list]);
    }
    e.call(ASSIGN_ACTOR_CAUSE, &args![this.addr() + ACTOR_CAUSE, 0u32]);
    if e.call(0x008d_8b80, &args![this]).u32() != 0 {
        let target = e.call(0x008d_8b80, &args![this]).u32();
        if target != 0 {
            fn_009bc890(e, Ptr::new(target), 1);
        }
        e.call(0x0081_26e0, &args![this, 0u32]);
    }
}

// Translated from 009c5af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::PickAutoWater` (Xbox PDB; `thiscall (pick, &fraction)`,
/// `RET 8`): in a cell with water (`004518e0`), when the pick's start
/// (`004585f0`) is above the water height (`TESObjectCELL::GetWaterHeight`,
/// stored as a `float`) and its end (`004587f0`) at or below it, the
/// fraction of the ray to the water, `(height - start z) / (end - start)
/// z` (`00439ef0`), goes to `*fraction` and true is answered.
pub fn projectile_pick_auto_water(
    e: &mut Engine,
    this: Ptr<Projectile>,
    pick: Ptr,
    fraction: Ptr,
) -> bool {
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    if cell == 0 || !e.call(0x0045_18e0, &args![cell]).bool() {
        return false;
    }
    let height = x87_store32(e.call(0x0054_71e0, &args![cell]).f64());
    e.with_stack(0x24, |e, frame| {
        let start = frame.addr();
        let end = start + 0xc;
        let difference = start + 0x18;
        e.call(0x0068_15c0, &args![start]);
        e.call(0x0045_85f0, &args![pick, start]);
        e.call(0x0068_15c0, &args![end]);
        e.call(0x0045_87f0, &args![pick, end]);
        let level = x87_load32(height);
        if !(level < x87_load32(e.mem.u32(start + 8))) {
            return false;
        }
        let end_z = x87_load32(e.mem.u32(end + 8));
        if !(level >= end_z) {
            return false;
        }
        e.call(0x0043_9ef0, &args![end, difference, start]);
        let drop = x87_store32(x87_sub(level, x87_m32(e.mem.u32(start + 8))));
        let ratio = x87_div(x87_load32(drop), x87_m32(e.mem.u32(difference + 8)));
        e.mem.set_u32(fraction.addr(), x87_store32(ratio));
        true
    })
}

// Translated from 009c40c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c40c0 - its C++ exception frame (FS:[0], the hit collector's unwind) and the /GS cookie check (00ec408c) are not translated
/// `Projectile::CheckShotBehindWall` (Xbox PDB): only for the player's
/// shots. A pick (`004a3c20`) from the player's eye (position, vtable
/// +0x1F4, raised by `0070c3b0` times the scale, both through the x87) to
/// the projectile (the ray `00439ef0` of the two), with the collision
/// filter of the player's character controller (`0070c440`) or of the
/// player (`00931ed0`) given layer 6 (`004a39f0`), is cast into the
/// player's cell's world (vtable +0xC8, `004a3fb0` attaching an
/// `hkpAllRayHitCollector`); water on the way (`009c5af0`, its fraction
/// starting at `FLT_MAX`) counts as a hit too. A hit is checked against
/// the projectile (`009c0d80` with its base object, 0, 1 and the
/// fraction); a confirmed hit stops the projectile (`009795c0` with 0.0)
/// and is answered.
pub fn projectile_check_shot_behind_wall(e: &mut Engine, this: Ptr<Projectile>) -> bool {
    let player = e.global::<u32>(PLAYER);
    if e.get(this, Projectile::pShooter) != player {
        return false;
    }
    // The exe's locals from its aligned `EBP`: at(k) is `EBP - k`.
    e.with_stack(0x450, |e, frame| {
        let at = |k: u32| frame.addr() + 0x450 - k;
        let pick = at(0xd0);
        let eye = at(0xdc);
        let position = at(0xe8);
        let ray = at(0xf4);
        let filter = at(0x114);
        let collector = at(0x440);
        let distance = at(0x44c);
        e.call(0x004a_3c20, &args![pick]);
        e.call(0x0068_15c0, &args![eye]);
        e.call(0x0068_15c0, &args![position]);
        e.call(0x0068_15c0, &args![ray]);
        let own = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        copy_point(e, position, own);
        let player = e.global::<u32>(PLAYER);
        let theirs = e.vcall(player, 0x1f4, &args![]).u32();
        copy_point(e, eye, theirs);
        let player = e.global::<u32>(PLAYER);
        let height = e.call(0x0070_c3b0, &args![player]).f64();
        let player = e.global::<u32>(PLAYER);
        let scale = e.call(0x0056_7400, &args![player]).f64();
        let raised = x87_add(x87_mul(scale, height), x87_m32(e.mem.u32(eye + 8)));
        e.mem.set_u32(eye + 8, x87_store32(raised));
        let difference = e.call(0x0043_9ef0, &args![position, at(0x10c), eye]).u32();
        copy_point(e, ray, difference);
        let controller = e.call(0x0093_06d0, &args![this]).u32();
        e.call(0x008c_71b0, &args![filter, 0u32]);
        let source = if controller != 0 {
            e.call(0x0070_c440, &args![controller, at(0x118)]).u32()
        } else {
            let player = e.global::<u32>(PLAYER);
            e.call(0x0093_1ed0, &args![player, at(0x11c)]).u32()
        };
        let value = e.mem.u32(source);
        e.mem.set_u32(filter, value);
        e.call(0x004a_39f0, &args![filter, 6u32]);
        let filter_word = e.mem.u32(filter);
        e.call(0x004a_3f70, &args![pick, filter_word]);
        e.call(0x004a_3da0, &args![pick, eye]);
        e.call(0x007f_68a0, &args![pick, ray]);
        e.call(0x004a_3a70, &args![collector]);
        e.call(0x004a_3fb0, &args![pick, collector]);
        let player = e.global::<u32>(PLAYER);
        let cell = e.call(0x008d_6f30, &args![player]).u32();
        let world = e.call(0x0045_43c0, &args![cell]).u32();
        let mut hit = e.vcall(world, 0xc8, &args![pick]).u8();
        let far = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
        e.mem.set_u32(distance, far);
        let water = projectile_pick_auto_water(e, this, Ptr::new(pick), Ptr::new(distance));
        hit |= u8::from(water);
        if hit != 0 {
            let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
            let fraction = x87_quiet_bits(e.mem.u32(distance));
            let confirmed = fn_009c0d80(
                e,
                Ptr::new(pick),
                this,
                base,
                0,
                1,
                f32::from_bits(fraction),
            );
            hit = u8::from(confirmed != 0);
        }
        if hit != 0 {
            e.call(0x0097_95c0, &args![this, 0.0f32]);
        }
        e.call(0x004a_3bc0, &args![collector]);
        hit != 0
    })
}

// Translated from 009c45d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::ChangeProcessLevel` (Xbox PDB): the wanted level is high
/// (0) when the projectile has a light base (vtable +0x1D0) or `00445750`
/// on `011c3b3c` accepts it, and its cell is loaded (`TES::IsCellLoaded`
/// with 1); middle-high (1) when vtable +0x324 says so and the cell is
/// loaded with 0; else 3 (none). The current level is its process's
/// (`0045cd60`), 3 without one. On a change: to 0 vtable +0x240, to 1
/// vtable +0x24C; to none, a form the data handler (`00469860` on
/// `011c3f2c`) marks is unloaded (with vtable +0x324, `UnloadForm` on
/// `011ddf38`), deleted (vtable +0xC4 with 1) and false answered; any other
/// loses its process under the lock at `011f11a0`
/// (`ProcessLists::RemoveReference`, the process deleted, vtable 0 with
/// 1), then vtable +0x2A0 and +0x2FC run. True otherwise.
pub fn projectile_change_process_level(e: &mut Engine, this: Ptr<Projectile>) -> bool {
    let object = this.addr();
    let process = e.get(this, Projectile::pCurrentProcess);
    let current = if process != 0 {
        e.call(0x0045_cd60, &args![process]).u32()
    } else {
        3
    };
    let mut wanted = 3u32;
    let middle = e.vcall(object, 0x324, &args![]).bool();
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let wants_processing = e.vcall(object, 0x1d0, &args![]).u32() != 0 || {
        let list = e.global::<u32>(0x011c_3b3c);
        e.call(0x0044_5750, &args![list, this]).bool()
    };
    if wants_processing {
        let tes = e.global::<u32>(TES);
        if e.call(0x0045_11e0, &args![tes, cell, 1u32]).bool() {
            wanted = 0;
        } else if middle {
            let tes = e.global::<u32>(TES);
            if e.call(0x0045_11e0, &args![tes, cell, 0u32]).bool() {
                wanted = 1;
            }
        }
    }
    if current == wanted {
        return true;
    }
    match wanted {
        0 => {
            e.vcall(object, 0x240, &args![]);
            true
        }
        1 => {
            e.vcall(object, 0x24c, &args![]);
            true
        }
        _ => {
            let form_id = e.call(GET_FORM_ID, &args![this]).u32();
            let handler = e.global::<u32>(0x011c_3f2c);
            if e.call(0x0046_9860, &args![handler, form_id]).bool() {
                if middle {
                    let save_load = e.global::<u32>(0x011d_df38);
                    e.call(0x0084_9730, &args![save_load, this, 0u32]);
                }
                e.vcall(object, 0xc4, &args![1u32]);
                return false;
            }
            let process = e.get(this, Projectile::pCurrentProcess);
            let level = e.call(0x0045_cd60, &args![process]).u32();
            e.call(0x0040_fbf0, &args![0x011f_11a0u32, 0u32]);
            e.call(0x0096_d470, &args![PROCESS_LISTS, this, level]);
            let process = e.get(this, Projectile::pCurrentProcess);
            if process != 0 {
                e.vcall(process, 0, &args![1u32]);
            }
            e.set(this, Projectile::pCurrentProcess, 0);
            e.call(0x0040_fba0, &args![0x011f_11a0u32]);
            e.vcall(object, 0x2a0, &args![]);
            e.vcall(object, 0x2fc, &args![]);
            true
        }
    }
}

/// The list node test `008256d0` (`thiscall`, `bool`: no item), the item
/// getter `006815c0` (the node's own address, its first word the item) and
/// the next node `00726070` of a `BSSimpleList`.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_ITEM: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `004839c0` (cdecl(form)): what the cast is given for a loaded form.
const CAST_SOURCE_004839C0: u32 = 0x0048_39c0;
/// RTTI type descriptors `.?AVTESForm@@`, `.?AVTESObjectREFR@@` and
/// `.?AVTESObjectWEAP@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_TES_OBJECT_WEAP: u32 = 0x0118_3998;
/// `BGSSaveGameBuffer`'s byte save (`00865e50`, thiscall(source, size, 0))
/// and `BGSLoadGameBuffer`'s byte load (`00864980`, thiscall(destination,
/// size)).
const SAVE_BYTES: u32 = 0x0086_5e50;
const LOAD_BYTES: u32 = 0x0086_4980;
/// `004e44b0` and `004e4470` (cdecl(`float`), ST0): the cosine and the sine
/// the circle of [`fn_009c3f90`] is built from (names not in the exe).
const CIRCLE_X: u32 = 0x004e_44b0;
const CIRCLE_Y: u32 = 0x004e_4470;
/// The process-lists lock (`011f11a0`): `0040fbf0(0)` takes it, `0040fba0`
/// leaves it.
const PROCESS_LOCK: u32 = 0x011f_11a0;

/// The impact data of the projectile's impact list, in order (each node's
/// item; the walk stops at an empty node or a null next).
fn impact_items(e: &mut Engine, this: Ptr<Projectile>) -> Vec<u32> {
    let mut items = Vec::new();
    let mut node = this.addr() + IMPACT_LIST;
    while node != 0 {
        let item = e.call(LIST_ITEM, &args![node]).u32();
        items.push(e.mem.u32(item));
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    items
}

// Translated from 009c3ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The projectile's impact on `collidee` (thiscall(`collidee`), `RET 4`):
/// walks the impact list (+0x88) while nothing is found, the node is not
/// null and not empty (`008256d0`), and answers the first impact data whose
/// `pCollidee` is `collidee` (0 for none).
pub fn fn_009c3ee0(e: &mut Engine, this: Ptr<Projectile>, collidee: u32) -> u32 {
    let mut found = 0;
    let mut node = this.addr() + IMPACT_LIST;
    while found == 0 && node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let item = e.call(LIST_ITEM, &args![node]).u32();
        let data = e.mem.u32(item);
        node = e.call(LIST_NEXT, &args![node]).u32();
        if e.mem.u32(data) == collidee {
            found = data;
        }
    }
    found
}

// Translated from 009c3f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills `out` with `count` points on a ring (cdecl, ten words: `out`,
/// `count`, the `center` point by value, `a`, `b`, `radius`, `length`,
/// `phase`). For point `i`: the angle `i / count * 2pi + phase` (the
/// `double` at `0101ff48`), the two angles `b + cos(angle) * radius` and `a
/// + sin(angle) * radius` (`004e44b0`, `004e4470`), the direction
/// `(cos q sin p, sin q sin p, -cos p)` (`00416870`) scaled by `length`
/// (`0045bb20`) and added to the center (`00439e90`); x87 throughout, each
/// local stored as a `float`.
pub fn fn_009c3f90(
    e: &mut Engine,
    out: u32,
    count: i32,
    center_x: f32,
    center_y: f32,
    center_z: f32,
    a: f32,
    b: f32,
    radius: f32,
    length: f32,
    phase: f32,
) {
    let (a, b, radius, length, phase) = (
        a.to_bits(),
        b.to_bits(),
        radius.to_bits(),
        length.to_bits(),
        phase.to_bits(),
    );
    e.with_stack(0x58, |e, frame| {
        let ebp = frame.addr() + 0x58;
        let center = ebp - 0x50;
        e.mem.set_u32(center, center_x.to_bits());
        e.mem.set_u32(center + 4, center_y.to_bits());
        e.mem.set_u32(center + 8, center_z.to_bits());
        let mut index = 0i32;
        while index < count {
            let turn = x87_div(f64::from(index), f64::from(count));
            let turn = x87_mul(turn, e.global::<f64>(0x0101_ff48));
            let angle = x87_store32(x87_add(turn, x87_m32(phase)));
            let cosine = e.call(CIRCLE_X, &args![x87_quiet_bits(angle)]).f64();
            let p = x87_store32(x87_add(x87_mul(cosine, x87_m32(radius)), x87_m32(b)));
            let sine = e.call(CIRCLE_Y, &args![x87_quiet_bits(angle)]).f64();
            let q = x87_store32(x87_add(x87_mul(sine, x87_m32(radius)), x87_m32(a)));
            let sin_p = x87_store32(e.call(CIRCLE_Y, &args![x87_quiet_bits(p)]).f64());
            let cos_p = x87_store32(e.call(CIRCLE_X, &args![x87_quiet_bits(p)]).f64());
            let sin_q = x87_store32(e.call(CIRCLE_Y, &args![x87_quiet_bits(q)]).f64());
            let cos_q = x87_store32(e.call(CIRCLE_X, &args![x87_quiet_bits(q)]).f64());
            let z = x87_store32(-x87_load32(cos_p));
            let y = x87_store32(x87_mul(x87_load32(sin_q), x87_m32(sin_p)));
            let x = x87_store32(x87_mul(x87_load32(cos_q), x87_m32(sin_p)));
            let direction = ebp - 0x20;
            e.call(
                0x0041_6870,
                &args![direction, x87_quiet_bits(x), x87_quiet_bits(y), z],
            );
            let scaled = e
                .call(
                    0x0045_bb20,
                    &args![direction, ebp - 0x38, x87_quiet_bits(length)],
                )
                .u32();
            let point = e
                .call(0x0043_9e90, &args![center, ebp - 0x44, scaled])
                .u32();
            let slot = out.wrapping_add((index as u32).wrapping_mul(0xc));
            copy_point(e, slot, point);
            index += 1;
        }
    });
}

// Translated from 009c43e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c43e0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Arms a mine (thiscall(`target`, `silent`), `RET 8`, a byte; no Xbox PDB
/// name): false unless the base object's `004fdf20` answers true and flag
/// 0x200 is clear. Then the countdown sound stops, and unless `silent` the
/// base object's sound (`00586150`) plays at the projectile (`BSAudio`
/// `00ad7480` with flags 0x4102 and `00511840`'s file, `0068a7d0` at
/// virtual +0x1F4, `BSSoundHandle::Play(0)`, the handle released by
/// `00483710`). Flag 0x200 is set. The shooter counts when it is a
/// character (virtual +0x100), else the owner (`00567790`) of form type 8.
/// Unless `silent`, a mine without a timer (`fExplosionTimer` exactly
/// `FLT_MAX`) that does not react to `target` (`GetMineReactsToTarget`)
/// stops there; otherwise the mines-armed statistic (`004d5c60(0xb)`) and,
/// for a positive integer setting at `011cecdc`, the player's virtual
/// +0x488 with it. Answers true.
pub fn fn_009c43e0(e: &mut Engine, this: Ptr<Projectile>, target: u32, silent: u32) -> bool {
    let silent = silent as u8;
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if !e.call(0x004f_df20, &args![base]).bool() {
        return false;
    }
    if test_flags(e, this, 0x200) {
        return false;
    }
    projectile_stop_countdown_sound(e, this);
    if silent == 0 {
        let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
        let sound = e.call(0x0058_6150, &args![base]).u32();
        if sound != 0 {
            e.with_stack(0xc, |e, handle| {
                let file = e.call(0x0051_1840, &args![sound]).u32();
                let audio = e.call(0x0045_3a70, &[]).u32();
                e.call(0x00ad_7480, &args![audio, handle, file, 0x4102u32, sound]);
                let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
                e.call(0x0068_a7d0, &args![handle, position]);
                e.call(0x00ad_8830, &args![handle, 0u32]);
                e.call(0x0048_3710, &args![handle]);
            });
        }
    }
    set_flags(e, this, 0x200, true);
    let mut actor = 0;
    let mut owner = 0;
    let shooter = e.get(this, Projectile::pShooter);
    if shooter != 0 && is_actor(e, shooter) {
        actor = e.get(this, Projectile::pShooter);
    } else {
        let form = e.call(0x0056_7790, &args![this]).u32();
        if form != 0 && e.call(0x0040_1170, &args![form]).u32() == 8 {
            owner = form;
        }
    }
    if silent == 0 {
        let timer = e.mem.u32(this.addr() + 0xe4);
        // `FCOMP` with the `double` `FLT_MAX`, `TEST AH,0x44`/`JP`.
        let reacts = if x87_load32(timer) == e.global::<f64>(0x0102_31b0) {
            projectile_get_mine_reacts_to_target(e, this, target, actor, owner)
        } else {
            true
        };
        if reacts {
            e.call(0x004d_5c60, &args![0xbu32]);
            let value = e.call(0x0043_d4d0, &args![0x011c_ecdcu32]).u32();
            if e.mem.i32(value) > 0 {
                let value = e.call(0x0043_d4d0, &args![0x011c_ecdcu32]).u32();
                let amount = e.mem.u32(value);
                let player = e.global::<u32>(PLAYER);
                e.vcall(player, 0x488, &args![amount]);
            }
        }
    }
    true
}

/// The process change [`fn_009c47d0`] .. [`fn_009c4b60`] share: under the
/// process-lists lock, the old process's level change is reported
/// (`0096d470(this, 0045cd60(old))`), a new process of `size` bytes is
/// built by `constructor` (null when the allocation failed), copies the old
/// one (its virtual +4), the old one is deleted (its virtual +0(1)), and the
/// new one (`+0x68`) is registered at `level` (`0096d450(this, level, 0, 0,
/// 0)`).
fn change_process(e: &mut Engine, this: Ptr<Projectile>, size: u32, constructor: u32, level: u32) {
    let base = this.addr();
    e.call(0x0040_fbf0, &args![PROCESS_LOCK, 0u32]);
    let old = e.mem.u32(base + 0x68);
    if old != 0 {
        let old_level = e.call(0x0045_cd60, &args![old]).u32();
        e.call(0x0096_d470, &args![PROCESS_LISTS, this, old_level]);
    }
    let block = allocate(e, size);
    let process = if block != 0 {
        e.call(constructor, &args![block]).u32()
    } else {
        0
    };
    let old = e.mem.u32(base + 0x68);
    e.vcall(process, 4, &args![old]);
    let old = e.mem.u32(base + 0x68);
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    e.mem.set_u32(base + 0x68, process);
    e.call(
        0x0096_d450,
        &args![PROCESS_LISTS, this, level, 0u32, 0u32, 0u32],
    );
    e.call(0x0040_fba0, &args![PROCESS_LOCK]);
}

// Translated from 009c47d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c47d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Gives the projectile a high process (0x46C bytes, `008d7510`, level 0;
/// see [`change_process`]), then its virtual +0x1FC(0) and the new
/// process's virtual +0x48. Answers true.
pub fn fn_009c47d0(e: &mut Engine, this: Ptr<Projectile>) -> bool {
    change_process(e, this, 0x46c, 0x008d_7510, 0);
    e.vcall(this.addr(), 0x1fc, &args![0u32]);
    let process = e.mem.u32(this.addr() + 0x68);
    e.vcall(process, 0x48, &args![]);
    true
}

// Translated from 009c4920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c4920 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Gives the projectile a process of 0x25C bytes (`00913fe0`, level 1; see
/// [`change_process`]). Answers true.
pub fn fn_009c4920(e: &mut Engine, this: Ptr<Projectile>) -> bool {
    change_process(e, this, 0x25c, 0x0091_3fe0, 1);
    true
}

// Translated from 009c4a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c4a40 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Gives the projectile a process of 0xC8 bytes (`0092c950`, level 2; see
/// [`change_process`]). Answers true.
pub fn fn_009c4a40(e: &mut Engine, this: Ptr<Projectile>) -> bool {
    change_process(e, this, 0xc8, 0x0092_c950, 2);
    true
}

// Translated from 009c4b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c4b60 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Gives the projectile a process of 0xB4 bytes (`00906dc0`, level 3; see
/// [`change_process`]). Answers true.
pub fn fn_009c4b60(e: &mut Engine, this: Ptr<Projectile>) -> bool {
    change_process(e, this, 0xb4, 0x0090_6dc0, 3);
    true
}

// Translated from 009c4c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forgets a reference that goes away (thiscall(`reference`), `RET 4`; no
/// Xbox PDB name): as the desired target it is cleared (`00420ba0(0)`) and
/// a projectile with flag 0x400 ends ([`fn_009bc8f0`]); an impact on it
/// (walking the impact list until the first) ends the projectile; as the
/// shooter it ends the projectile; as the reference of the projectile's
/// `008d8b80` object, that object is deleted (`009bc890(1)`) and
/// `008126e0(0)` runs.
pub fn fn_009c4c80(e: &mut Engine, this: Ptr<Projectile>, reference: u32) {
    if e.call(GET_DESIRED_TARGET, &args![this]).u32() == reference {
        e.call(0x0042_0ba0, &args![this, 0u32]);
        if test_flags(e, this, 0x400) {
            fn_009bc8f0(e, this);
        }
    }
    let list = this.addr() + IMPACT_LIST;
    if !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let mut found = false;
        let mut node = list;
        while !found && node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let item = e.call(LIST_ITEM, &args![node]).u32();
            let data = e.mem.u32(item);
            node = e.call(LIST_NEXT, &args![node]).u32();
            if e.mem.u32(data) == reference {
                fn_009bc8f0(e, this);
                found = true;
            }
        }
    }
    if e.call(GET_SHOOTER, &args![this]).u32() == reference {
        fn_009bc8f0(e, this);
    }
    if e.call(0x008d_8b80, &args![this]).u32() != 0 {
        let object = e.call(0x008d_8b80, &args![this]).u32();
        if e.mem.u32(object) == reference {
            let object = e.call(0x008d_8b80, &args![this]).u32();
            if object != 0 {
                fn_009bc890(e, Ptr::new(object), 1);
            }
            e.call(0x0081_26e0, &args![this, 0u32]);
        }
    }
}

// Translated from 009c4ec0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c4ec0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `Projectile::StartCountdownSound` (Xbox PDB): unless the countdown
/// handle (+0x134) is playing (`00ad8ce0`), and when the base object has a
/// countdown sound (`00810fd0`), plays its form id (`0084e3a0`) at the
/// projectile (`00933150(handle out, id, 0, 0x40000102, 1)`), stores the
/// handle (`00418900`) and releases the temporary (`00483710`).
pub fn projectile_start_countdown_sound(e: &mut Engine, this: Ptr<Projectile>) {
    let handle = this.addr() + COUNTDOWN_SOUND;
    if e.call(0x00ad_8ce0, &args![handle]).bool() {
        return;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if e.call(0x0081_0fd0, &args![base]).u32() == 0 {
        return;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let sound = e.call(0x0081_0fd0, &args![base]).u32();
    let id = e.call(GET_FORM_ID, &args![sound]).u32();
    e.with_stack(0xc, |e, temporary| {
        let played = e
            .call(
                0x0093_3150,
                &args![this, temporary, id, 0u32, 0x4000_0102u32, 1u32],
            )
            .u32();
        e.call(0x0041_8900, &args![handle, played]);
        e.call(0x0048_3710, &args![temporary]);
    });
}

// Translated from 009c4ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::SaveGame` (Xbox PDB; thiscall(`buffer`), `RET 4`): the
/// `MobileObject` part (`00932880`), then `iFlags`, `fPower`, `fRange`,
/// `fSpeedMult`, `fAge`, `fDamage`, `fAlpha`, `fExplosionTimer`,
/// `fBlinkTimer`, the weapon, shooter and desired target form ids
/// (`00865df0`), the movement direction, `fDistanceMoved`, `bSendAlarm`,
/// `fDecalSize`, the follow offset (`00865ee0`, 0x24 bytes), the words at
/// +0xB8 (0xC bytes) and +0xC4, `fSpreadHeading`, `fSpreadLooking`,
/// `fWeaponCondition`, the actor cause's id (`00825c00`, -1 without one),
/// whether there is a secondary item change (then its `004bed60`), the
/// impacts as a counted block (`00865f20`, each
/// `ImpactData::SaveGame`, `00865ff0`) and `bImpactProcessed`.
pub fn projectile_save_game(e: &mut Engine, this: Ptr<Projectile>, buffer: Ptr) {
    let base = this.addr();
    e.call(0x0093_2880, &args![this, buffer]);
    for offset in [0xc8u32, 0xcc, 0xd4, 0xd0, 0xd8, 0xdc, 0xe0, 0xe4, 0xe8] {
        e.call(SAVE_BYTES, &args![buffer, base + offset, 4u32, 0u32]);
    }
    for offset in [0xf8u32, 0xfc, 0x100] {
        let form = e.mem.u32(base + offset);
        e.call(0x0086_5df0, &args![buffer, form, 0u32]);
    }
    for (offset, size) in [(0x104u32, 0xcu32), (0x110, 4), (0x118, 1), (0x124, 4)] {
        e.call(SAVE_BYTES, &args![buffer, base + offset, size, 0u32]);
    }
    e.call(
        0x0086_5ee0,
        &args![buffer, base + FOLLOW_OFFSET, 0x24u32, 0u32],
    );
    for (offset, size) in [
        (0xb8u32, 0xcu32),
        (0xc4, 4),
        (0xec, 4),
        (0xf0, 4),
        (0xf4, 4),
    ] {
        e.call(SAVE_BYTES, &args![buffer, base + offset, size, 0u32]);
    }
    e.with_stack(8, |e, locals| {
        let cause = if e.call(POINTER_GET, &args![base + ACTOR_CAUSE]).u32() != 0 {
            let cause = e.call(POINTER_GET, &args![base + ACTOR_CAUSE]).u32();
            e.call(0x0082_5c00, &args![cause]).u32()
        } else {
            u32::MAX
        };
        e.mem.set_u32(locals.addr() + 4, cause);
        e.call(SAVE_BYTES, &args![buffer, locals.addr() + 4, 4u32, 0u32]);
        let has_item = e.get(this, Projectile::pSecondaryItemChange) != 0;
        e.mem.set_u8(locals.addr() + 3, u8::from(has_item));
        e.call(SAVE_BYTES, &args![buffer, locals.addr() + 3, 1u32, 0u32]);
        if e.mem.u8(locals.addr() + 3) != 0 {
            let item = e.get(this, Projectile::pSecondaryItemChange);
            e.call(0x004b_ed60, &args![item, buffer]);
        }
    });
    let mut count = 0u32;
    let block = e.call(0x0086_5f20, &args![buffer]).u32();
    for data in impact_items(e, this) {
        if data != 0 {
            projectile_impact_data_save_game(e, Ptr::new(data), buffer);
            count = count.wrapping_add(1);
        }
    }
    e.call(0x0086_5ff0, &args![buffer, count, block]);
    e.call(SAVE_BYTES, &args![buffer, base + 0x90, 1u32, 0u32]);
}

// Translated from 009c52f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c52f0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `Projectile::LoadGame` (Xbox PDB; thiscall(`buffer`), `RET 4`): the
/// `MobileObject` part (`00932a00`), the fields [`projectile_save_game`]
/// writes in the same order (the weapon through `008648a0`, `004839c0` and
/// a `TESForm` to `TESObjectWEAP` cast; shooter and target through
/// `008648e0`; the follow offset through `00864a20`), the +0xB8/+0xC4 words
/// from version 0xD (the buffer's virtual +0), the spread angles from 8 and
/// the weapon condition from 0xE; the actor cause from its id (`0066e330`,
/// `0042f780`); from version 3 the secondary item change (0xC bytes,
/// `0076b630`, `004bee00`; dropped (`004bc780`, deleted) when it has no
/// object, `0044ddc0`); from 0xE the impacts (each 0x30 bytes, `009bffb0`,
/// `ImpactData::LoadGame`, `bProcessed` cleared without 3D (virtual
/// +0x1D0), appended by `00905820`); from 0x10 `bImpactProcessed`.
pub fn projectile_load_game(e: &mut Engine, this: Ptr<Projectile>, buffer: Ptr) {
    let base = this.addr();
    e.call(0x0093_2a00, &args![this, buffer]);
    for offset in [0xc8u32, 0xcc, 0xd4, 0xd0, 0xd8, 0xdc, 0xe0, 0xe4, 0xe8] {
        e.call(LOAD_BYTES, &args![buffer, base + offset, 4u32]);
    }
    let form = e.call(0x0086_48a0, &args![buffer]).u32();
    let source = e.call(CAST_SOURCE_004839C0, &args![form]).u32();
    let weapon = e
        .call(
            RT_DYNAMIC_CAST,
            &args![source, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_WEAP, 0u32],
        )
        .u32();
    e.set(this, Projectile::pWeaponSource, weapon);
    e.call(0x0086_48e0, &args![buffer, base + 0xfc]);
    e.call(0x0086_48e0, &args![buffer, base + 0x100]);
    for (offset, size) in [(0x104u32, 0xcu32), (0x110, 4), (0x118, 1), (0x124, 4)] {
        e.call(LOAD_BYTES, &args![buffer, base + offset, size]);
    }
    e.call(0x0086_4a20, &args![buffer, base + FOLLOW_OFFSET, 0x24u32]);
    if i32::from(e.vcall(buffer.addr(), 0, &args![]).u8()) >= 0xd {
        e.call(LOAD_BYTES, &args![buffer, base + 0xb8, 0xcu32]);
        e.call(LOAD_BYTES, &args![buffer, base + 0xc4, 4u32]);
    }
    if i32::from(e.vcall(buffer.addr(), 0, &args![]).u8()) >= 8 {
        e.call(LOAD_BYTES, &args![buffer, base + 0xec, 4u32]);
        e.call(LOAD_BYTES, &args![buffer, base + 0xf0, 4u32]);
    }
    if i32::from(e.vcall(buffer.addr(), 0, &args![]).u8()) >= 0xe {
        e.call(LOAD_BYTES, &args![buffer, base + 0xf4, 4u32]);
    }
    e.with_stack(8, |e, locals| {
        let id = locals.addr() + 4;
        e.mem.set_u32(id, u32::MAX);
        e.call(LOAD_BYTES, &args![buffer, id, 4u32]);
        let cause = e.call(0x0066_e330, &args![e.mem.u32(id)]).u32();
        e.call(ASSIGN_ACTOR_CAUSE, &args![base + ACTOR_CAUSE, cause]);
        if i32::from(e.vcall(buffer.addr(), 0, &args![]).u8()) >= 3 {
            let flag = locals.addr() + 3;
            e.mem.set_u8(flag, 0);
            e.call(LOAD_BYTES, &args![buffer, flag, 1u32]);
            if e.mem.u8(flag) != 0 {
                let block = allocate(e, 0xc);
                let item = if block != 0 {
                    e.call(0x0076_b630, &args![block]).u32()
                } else {
                    0
                };
                e.set(this, Projectile::pSecondaryItemChange, item);
                let item = e.get(this, Projectile::pSecondaryItemChange);
                e.call(0x004b_ee00, &args![item, buffer]);
                let item = e.get(this, Projectile::pSecondaryItemChange);
                if e.call(0x0044_ddc0, &args![item]).u32() == 0 {
                    let item = e.get(this, Projectile::pSecondaryItemChange);
                    e.call(0x004b_c780, &args![item]);
                    let item = e.get(this, Projectile::pSecondaryItemChange);
                    if item != 0 {
                        e.call(0x0044_59e0, &args![item, 1u32]);
                    }
                    e.set(this, Projectile::pSecondaryItemChange, 0);
                }
            }
        }
    });
    if i32::from(e.vcall(buffer.addr(), 0, &args![]).u8()) >= 0xe {
        let count = e.call(0x0086_4a60, &args![buffer]).u32();
        let mut index = 0u32;
        while index < count {
            let block = allocate(e, 0x30);
            let data = if block != 0 {
                fn_009bffb0(e, Ptr::new(block)).addr()
            } else {
                0
            };
            projectile_impact_data_load_game(e, Ptr::new(data), buffer);
            if e.vcall(base, 0x1d0, &args![]).u32() == 0 {
                e.mem.set_u8(data.wrapping_add(0x28), 0);
            }
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), data);
                e.call(0x0090_5820, &args![base + IMPACT_LIST, slot]);
            });
            index += 1;
        }
    }
    if i32::from(e.vcall(buffer.addr(), 0, &args![]).u8()) >= 0x10 {
        e.call(LOAD_BYTES, &args![buffer, base + 0x90, 1u32]);
    }
}

/// The form of a loaded form id word, cast to a reference: `__RTDynamicCast(
/// 004839c0(form), 0, TESForm, TESObjectREFR, 0)`, or 0 for none.
fn as_reference(e: &mut Engine, form: u32) -> u32 {
    if form == 0 {
        return 0;
    }
    let source = e.call(CAST_SOURCE_004839C0, &args![form]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![source, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
    )
    .u32()
}

// Translated from 009c5740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The projectile's finish-load step (thiscall(`buffer`), `RET 4`; no Xbox
/// PDB name): the `MobileObject` part (`00932b70`), the shooter and desired
/// target resolved to references (see [`as_reference`]), the secondary
/// item change's `004bef60`, and from version 0xE (the buffer's virtual +0)
/// each impact's `009a4f40`.
pub fn fn_009c5740(e: &mut Engine, this: Ptr<Projectile>, buffer: Ptr) {
    e.call(0x0093_2b70, &args![this, buffer]);
    let shooter = e.get(this, Projectile::pShooter);
    let shooter = as_reference(e, shooter);
    e.set(this, Projectile::pShooter, shooter);
    let target = e.get(this, Projectile::pDesiredTarget);
    let target = as_reference(e, target);
    e.set(this, Projectile::pDesiredTarget, target);
    let item = e.get(this, Projectile::pSecondaryItemChange);
    if item != 0 {
        e.call(0x004b_ef60, &args![item, buffer]);
    }
    if i32::from(e.vcall(buffer.addr(), 0, &args![]).u8()) >= 0xe {
        for data in impact_items(e, this) {
            if data != 0 {
                e.call(0x009a_4f40, &args![data, buffer]);
            }
        }
    }
}

// Translated from 009c5870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The projectile's form-id fix-up step (thiscall(`buffer`), `RET 4`; no
/// Xbox PDB name): the `MobileObject` part (`00932d60`), then from version
/// 0xE (the buffer's virtual +0) each impact's `009bbb40`.
pub fn fn_009c5870(e: &mut Engine, this: Ptr<Projectile>, buffer: Ptr) {
    e.call(0x0093_2d60, &args![this, buffer]);
    if i32::from(e.vcall(buffer.addr(), 0, &args![]).u8()) >= 0xe {
        for data in impact_items(e, this) {
            if data != 0 {
                fn_009bbb40(e, Ptr::new(data), buffer.addr());
            }
        }
    }
}

// Translated from 009c58e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the projectile for a load (thiscall(`buffer`), `RET 4`; no Xbox
/// PDB name): the `MobileObject` part (`00932f60`), every impact deleted
/// (`009b3490(1)`) and its node removed (`0063f7b0`), the fields back to
/// their starting values (flags 0, power and speed 1, range, age and damage
/// 0, alpha 1, distance 0, the movement direction from `011f426c`, no
/// weapon, shooter or target, no light, pick cache or actor cause, no
/// alarm, decal 0, explosion timer `FLT_MAX`, blink 0, condition 1, impact
/// not processed, the follow offset identity (`00a86bf0`), no target), the
/// secondary item change deleted (`004459e0(1)`), and virtual +0x308 when
/// `00562d00(buffer)` answers true.
pub fn fn_009c58e0(e: &mut Engine, this: Ptr<Projectile>, buffer: Ptr) {
    let base = this.addr();
    e.call(0x0093_2f60, &args![this, buffer]);
    while !e.call(LIST_IS_EMPTY, &args![base + IMPACT_LIST]).bool() {
        let item = e.call(LIST_ITEM, &args![base + IMPACT_LIST]).u32();
        let data = e.mem.u32(item);
        if data != 0 {
            e.call(0x009b_3490, &args![data, 1u32]);
        }
        e.call(0x0063_f7b0, &args![base + IMPACT_LIST]);
    }
    let one = 1.0f32.to_bits();
    e.set(this, Projectile::iFlags, 0);
    for (offset, bits) in [
        (0xccu32, one),
        (0xd0, one),
        (0xd4, 0),
        (0xd8, 0),
        (0xdc, 0),
        (0xe0, one),
        (0x110, 0),
    ] {
        e.mem.set_u32(base + offset, bits);
    }
    copy_point(e, base + MOVEMENT_DIRECTION, ZERO_POINT);
    e.set(this, Projectile::pWeaponSource, 0);
    e.set(this, Projectile::pShooter, 0);
    e.set(this, Projectile::pDesiredTarget, 0);
    e.call(ASSIGN_NI_POINTER, &args![base + LIGHT, 0u32]);
    e.call(ASSIGN_NI_POINTER, &args![base + PICK_CACHE, 0u32]);
    e.call(ASSIGN_ACTOR_CAUSE, &args![base + ACTOR_CAUSE, 0u32]);
    e.mem.set_u8(base + 0x118, 0);
    e.mem.set_u32(base + 0x124, 0);
    let max = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
    e.mem.set_u32(base + 0xe4, max);
    e.mem.set_u32(base + 0xe8, 0);
    e.mem.set_u32(base + 0xf4, one);
    e.mem.set_u8(base + 0x90, 0);
    e.call(0x00a8_6bf0, &args![base + FOLLOW_OFFSET]);
    e.set(this, Projectile::pTarget, 0);
    let item = e.get(this, Projectile::pSecondaryItemChange);
    if item != 0 {
        e.call(0x0044_59e0, &args![item, 1u32]);
    }
    e.set(this, Projectile::pSecondaryItemChange, 0);
    if e.call(0x0056_2d00, &args![buffer]).bool() {
        e.vcall(base, 0x308, &args![]);
    }
}

// Translated from 009c5ba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c5ba0 - the /GS security-cookie check (00ec408c) is not translated.
/// Casts a pick against a node's collision and its children (cdecl(`pick`,
/// `node`, `phantoms_only`, `flush`), a byte; no Xbox PDB name). For a
/// node with a collision object (`0043b610`) whose world object (`006fa820`)
/// has a shape (`004ae6a0`) and is in a world (`0043b4f0`, `00624070`;
/// with `phantoms_only`, not of `006240b0` kind 1): the shape's transform
/// (`0056ded0`; for an `0043b300(01267e64)` collision object whose world
/// object is not `008cffa0` or whose `004a7bd0` is 1.0, the node's
/// transform `0056df80` and, for an `0043b300(012683c8)` world object, its
/// motion transform (`00632b40`, `008cd6a0(00891170, 00436aa0)`,
/// `00cb2240`); otherwise the world object's virtual +0xF4), the pick's
/// ray in it (`00632ca0`, `004a3d20`, `00c7f690` on the pick's start and
/// end `00458820`), the shape's cast (`004ae750`, `00632c30`) and on a hit
/// (`00632c60`, `005609f0`) the pick's collector (`008cdd90`) gets it
/// (virtual +0(`00460140(004ae750(world object))`, output)). Then each
/// child of the node's virtual +0xC list (`0043b480`, `0043b4a0`) is cast
/// the same way (no flush), and with `flush` the collector's `00c855f0`.
/// Answers whether anything was hit.
pub fn fn_009c5ba0(e: &mut Engine, pick: u32, node: u32, phantoms_only: u32, flush: u32) -> bool {
    let phantoms_only = phantoms_only as u8;
    let flush = flush as u8;
    let mut hit = false;
    if node != 0 {
        e.with_stack(0x1a0, |e, frame| {
            let ebp = frame.addr() + 0x1a0;
            hit = cast_node(e, pick, node, phantoms_only, ebp);
        });
        let children = e.vcall(node, 0xc, &args![]).u32();
        if children != 0 {
            let mut index = 0u32;
            while index < e.call(0x0043_b480, &args![children]).u32() {
                let child = e.call(0x0043_b4a0, &args![children, index]).u32();
                let child_hit = fn_009c5ba0(e, pick, child, u32::from(phantoms_only), 0);
                hit |= child_hit;
                index += 1;
            }
        }
    }
    if flush != 0 {
        let collector = e.call(0x008c_dd90, &args![pick]).u32();
        e.call(0x00c8_55f0, &args![collector]);
    }
    hit
}

/// The cast of [`fn_009c5ba0`] on one node, on its frame (`ebp`).
fn cast_node(e: &mut Engine, pick: u32, node: u32, phantoms_only: u8, ebp: u32) -> bool {
    let collision = e.call(0x0043_b610, &args![node]).u32();
    if collision == 0 {
        return false;
    }
    let world_object = e.call(0x006f_a820, &args![collision]).u32();
    let shape = if world_object != 0 {
        e.call(0x004a_e6a0, &args![world_object]).u32()
    } else {
        0
    };
    if shape == 0 {
        return false;
    }
    let world = e.call(0x0043_b4f0, &args![world_object, ebp - 0x1c]).u32();
    if !e.call(0x0062_4070, &args![world]).bool() {
        return false;
    }
    if phantoms_only != 0 {
        let world = e.call(0x0043_b4f0, &args![world_object, ebp - 0x20]).u32();
        if e.call(0x0062_40b0, &args![world]).u32() == 1 {
            return false;
        }
    }
    let transform = ebp - 0x60;
    e.call(0x0056_ded0, &args![transform]);
    if e.call(0x0043_b300, &args![0x0126_7e64u32, collision])
        .bool()
    {
        let follows_node = !e.call(0x008c_ffa0, &args![world_object]).bool() || {
            // `FCOMP` with the `double` 1.0, `TEST AH,0x44`/`JP`: only an
            // equal value passes.
            let value = e.call(0x004a_7bd0, &args![collision]).f64();
            value == e.global::<f64>(0x0101_2070)
        };
        if follows_node {
            let node_transform = e.call(0x0046_1130, &args![node]).u32();
            e.call(0x0056_df80, &args![transform, node_transform]);
            if e.call(0x0043_b300, &args![0x0126_83c8u32, world_object])
                .bool()
            {
                let motion = ebp - 0xb0;
                let copy = ebp - 0xf0;
                e.call(0x0056_ded0, &args![motion]);
                e.call(0x0063_2b40, &args![copy, transform]);
                let position = e.call(0x0043_6aa0, &args![world_object]).u32();
                let rotation = e.call(0x0089_1170, &args![world_object]).u32();
                e.call(0x008c_d6a0, &args![motion, rotation, position]);
                e.call(0x00cb_2240, &args![transform, copy, motion]);
            }
        } else {
            e.vcall(world_object, 0xf4, &args![transform]);
        }
    } else {
        e.vcall(world_object, 0xf4, &args![transform]);
    }
    let ray = ebp - 0x120;
    let output = ebp - 0x170;
    e.call(0x0063_2ca0, &args![ray]);
    e.call(0x004a_3d20, &args![output]);
    let start = e.call(0x0068_15c0, &args![pick]).u32();
    e.call(0x00c7_f690, &args![ray, transform, start]);
    let end = e.call(0x0045_8820, &args![pick]).u32();
    e.call(0x00c7_f690, &args![ebp - 0x110, transform, end]);
    let cast = e.call(0x004a_e750, &args![shape]).u32();
    e.call(0x0063_2c30, &args![cast, ebp - 0x171, ray, output]);
    let result = e.call(0x0063_2c60, &args![output, ebp - 0x172]).u32();
    if !e.call(0x0056_09f0, &args![result]).bool() {
        return false;
    }
    let collector = e.call(0x008c_dd90, &args![pick]).u32();
    let object = e.call(0x004a_e750, &args![world_object]).u32();
    let collidable = e.call(0x0046_0140, &args![object]).u32();
    e.vcall(collector, 0, &args![collidable, output]);
    true
}

// Translated from 009c5ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the projectile's 0x14-byte target (thiscall; no
/// Xbox PDB name): the reference cleared, the `NiPointer` at +4 built empty
/// (`00633c90(0)`), the point at +8 from `011f426c`. Returns `this`.
pub fn fn_009c5ee0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.call(0x0063_3c90, &args![this.addr() + 4, 0u32]);
    copy_point(e, this.addr() + 8, ZERO_POINT);
    this
}

// Translated from 009c5f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The world point of the projectile's target (thiscall(`out`), `RET 4`;
/// returns `out`; no Xbox PDB name): with a node (`+4`), the point +8 in
/// its transform (`00461130`, `00524c40`); else with a reference whose 3D
/// (virtual +0x1D0) exists, its bound's center (`0043d450`); else the
/// reference's virtual +0x1F4 point; with neither, `011f426c`.
pub fn fn_009c5f30(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let base = this.addr();
    e.with_stack(0x1c, |e, frame| {
        let ebp = frame.addr() + 0x1c;
        let point = ebp - 0xc;
        copy_point(e, point, ZERO_POINT);
        if e.call(POINTER_GET, &args![base + 4]).u32() != 0 {
            let node = e.call(POINTER_GET, &args![base + 4]).u32();
            let transform = e.call(0x0046_1130, &args![node]).u32();
            let world = e
                .call(0x0052_4c40, &args![transform, ebp - 0x18, base + 8])
                .u32();
            copy_point(e, point, world);
        } else {
            let reference = e.mem.u32(base);
            if reference != 0 {
                if e.vcall(reference, 0x1d0, &args![]).u32() != 0 {
                    let root = e.vcall(reference, 0x1d0, &args![]).u32();
                    let bound = e.call(0x0043_d450, &args![root]).u32();
                    let center = e.call(LIST_ITEM, &args![bound]).u32();
                    copy_point(e, point, center);
                } else {
                    let position = e.vcall(reference, 0x1f4, &args![]).u32();
                    copy_point(e, point, position);
                }
            }
        }
        copy_point(e, out.addr(), point);
    });
    out
}

// Translated from 009c6030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::AimAtPoint` (Xbox PDB; thiscall, the point by value, `RET
/// 0xC`): the normalized direction from the projectile (virtual +0x1F4) to
/// the point (`00439ef0`, `004a0c10`). The looking angle stays (`00931d70`)
/// unless the base object's `00966980` is exactly 0, then it is the
/// negated `004b1460(z / (1 - z * z))`; the heading becomes the direction's
/// z angle (`004b13c0`), set (virtual +0x2C4) always when `00966980` is 0,
/// else only when it differs from the current heading (virtual +0x2BC(0))
/// by at most 2.5 (`00408840`). The looking angle is set last (`00931d90`).
pub fn projectile_aim_at_point(e: &mut Engine, this: Ptr<Projectile>, x: u32, y: u32, z: u32) {
    let base = this.addr();
    // The frame (locals at `ebp - x`) and, above it at `ebp + 8`, the
    // point the caller pushed.
    e.with_stack(0x50, |e, frame| {
        let ebp = frame.addr() + 0x38;
        let target = ebp + 8;
        e.mem.set_u32(target, x);
        e.mem.set_u32(target + 4, y);
        e.mem.set_u32(target + 8, z);
        let position = e.vcall(base, 0x1f4, &args![]).u32();
        let offset = e
            .call(0x0043_9ef0, &args![target, ebp - 0x20, position])
            .u32();
        let direction = ebp - 0x10;
        copy_point(e, direction, offset);
        e.call(0x004a_0c10, &args![direction]);
        // The current heading is stored and then overwritten below.
        let _ = x87_store32(e.vcall(base, 0x2bc, &args![0u32]).f64());
        let mut looking = x87_store32(e.call(0x0093_1d70, &args![this]).f64());
        let object = e.call(GET_BASE_OBJECT, &args![this]).u32();
        let flat = x87_store32(e.call(0x0096_6980, &args![object]).f64());
        let zero = e.global::<f64>(0x0101_2060);
        let height = e.mem.u32(direction + 8);
        if x87_load32(flat) == zero {
            let square = x87_mul(x87_load32(height), x87_m32(height));
            let rest = x87_sub(1.0, square);
            let ratio = x87_store32(x87_div(x87_m32(height), rest));
            let angle = e.call(0x004b_1460, &args![x87_quiet_bits(ratio)]).f64();
            looking = x87_store32(-angle);
        }
        let heading = x87_store32(e.call(0x004b_13c0, &args![direction]).f64());
        let set_heading = if x87_load32(flat) == zero {
            true
        } else {
            let current = e.vcall(base, 0x2bc, &args![0u32]).f64();
            let change = x87_store32(x87_sub(x87_m32(heading), current));
            let size = e.call(0x0040_8840, &args![x87_quiet_bits(change)]).f64();
            // `FCOMP` with the `double` 2.5, `TEST AH,0x41`/`JP`: an
            // ordered "at most" sets it.
            size <= e.global::<f64>(0x0101_8c00)
        };
        if set_heading {
            e.vcall(base, 0x2c4, &args![x87_quiet_bits(heading)]);
        }
        e.call(0x0093_1d90, &args![this, x87_quiet_bits(looking)]);
    });
}

// Translated from 009c6160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the projectile's collision into the current world (thiscall, one
/// word it does not read, `RET 4`; no Xbox PDB name). With a character
/// controller (`009306d0`) or flag 4: the node (`0043fcd0`) and the world
/// of the projectile's cell (`008d6f30`, `004543c0`). A controller in
/// another world (`00621ad0`) is reset there (`0088b0a0(0)`, the cell's
/// water height `005471e0` (0 without a cell) through `0057b1d0`, the
/// position `00436aa0` through `005620e0`), and given the node when it
/// has another (`00810660`, `00c6d7a0`). Without a controller the world is
/// the 3D's first collision object's (`004b5260`, `006fa820`, virtual
/// +0x94, `00621480`). A node in another world is removed from it
/// (`00c69ee0(node, 1, 0)`, when it is in one) and added to the cell's
/// world (its virtual +0xD0(node, 1, 1, 0, 0)); one already there is
/// updated (`00c69f50(node, 1, 0)`).
pub fn fn_009c6160(e: &mut Engine, this: Ptr<Projectile>, _unused_0: u32) {
    let base = this.addr();
    let controller = e.call(0x0093_06d0, &args![this]).u32();
    if controller == 0 && !test_flags(e, this, 4) {
        return;
    }
    let node = e.call(0x0043_fcd0, &args![this]).u32();
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let cell_world = if cell != 0 {
        e.call(0x0045_43c0, &args![cell]).u32()
    } else {
        0
    };
    let current;
    if controller != 0 {
        current = e.call(0x0062_1ad0, &args![controller]).u32();
        if cell_world != current {
            e.call(0x0088_b0a0, &args![controller, 0u32]);
            let height = if cell != 0 {
                x87_store32(e.call(0x0054_71e0, &args![cell]).f64())
            } else {
                x87_store32(0.0)
            };
            e.call(0x0057_b1d0, &args![controller, x87_quiet_bits(height)]);
            let position = e.call(0x0043_6aa0, &args![this]).u32();
            e.call(0x0056_20e0, &args![controller, position]);
        }
        if e.call(0x0081_0660, &args![controller]).u32() != node {
            e.call(0x00c6_d7a0, &args![controller, node]);
        }
    } else {
        let root = e.vcall(base, 0x1d0, &args![]).u32();
        let first = e.call(0x004b_5260, &args![root]).u32();
        let object = if first != 0 {
            e.call(0x006f_a820, &args![first]).u32()
        } else {
            0
        };
        current = if object != 0 {
            let world = e.vcall(object, 0x94, &args![]).u32();
            e.call(0x0062_1480, &args![world]).u32()
        } else {
            0
        };
    }
    let mut moved = false;
    if current != cell_world {
        if current != 0 {
            e.call(0x00c6_9ee0, &args![node, 1u32, 0u32]);
        }
        moved = true;
    }
    if cell_world != 0 {
        if moved {
            e.vcall(cell_world, 0xd0, &args![node, 1u32, 1u32, 0u32, 0u32]);
        } else {
            e.call(0x00c6_9f50, &args![node, 1u32, 0u32]);
        }
    }
}

// Translated from 009c6330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::SetProjectileOrientation` (Xbox PDB; thiscall(`matrix`),
/// `RET 4`): the matrix's Euler angles (`NiMatrix3::ToEulerAnglesZXY`),
/// the first as the heading (virtual +0x2C4) and the second as the looking
/// angle (`00931d90`).
pub fn projectile_set_projectile_orientation(e: &mut Engine, this: Ptr<Projectile>, matrix: u32) {
    e.with_stack(0x10, |e, frame| {
        let ebp = frame.addr() + 0x10;
        e.call(0x00a5_9400, &args![matrix, ebp - 8, ebp - 4, ebp - 0xc]);
        let heading = x87_quiet_bits(e.mem.u32(ebp - 8));
        e.vcall(this.addr(), 0x2c4, &args![heading]);
        let looking = x87_quiet_bits(e.mem.u32(ebp - 4));
        e.call(0x0093_1d90, &args![this, looking]);
    });
}

// Translated from 009c6440 (decompiled, FalloutNV.exe 1.4.0.525)
/// A character-controller constructor (thiscall(`info`, `projectile`),
/// `RET 8`; no Xbox PDB name): the base `00c741e0(info)`, the vtables at
/// +0, +0x3E0 and +0x410, and the projectile at +0x650. Returns `this`.
pub fn fn_009c6440(e: &mut Engine, this: Ptr, info: u32, projectile: u32) -> Ptr {
    let base = this.addr();
    e.call(0x00c7_41e0, &args![this, info]);
    e.mem.set_u32(base, 0x0109_0594);
    e.mem.set_u32(base + 0x3e0, 0x0109_0584);
    e.mem.set_u32(base + 0x410, 0x0109_0568);
    e.mem.set_u32(base + 0x650, projectile);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009bb970,
            projectile_impact_data_save_game(Ptr<ImpactData>, Ptr)
        ),
        entry!(
            0x009bba90,
            projectile_impact_data_load_game(Ptr<ImpactData>, Ptr)
        ),
        entry!(0x009bbb40, fn_009bbb40(Ptr<ImpactData>, u32)),
        entry!(
            0x009bbc40,
            fn_009bbc40(Ptr<Projectile>, u32) -> Ptr<Projectile>
        ),
        entry!(
            0x009bbc70,
            projectile_ctor(Ptr<Projectile>, u32) -> Ptr<Projectile>
        ),
        entry!(
            0x009bbef0,
            fn_009bbef0(Ptr<Projectile>, Ptr, u32, u32, u32, u32, u32, f32, f32) -> Ptr<Projectile>
        ),
        entry!(0x009bc470, fn_009bc470(Ptr<Projectile>)),
        entry!(0x009bc870, fn_009bc870(Ptr) -> bool),
        entry!(0x009bc890, fn_009bc890(Ptr, u32) -> Ptr),
        entry!(0x009bc8c0, fn_009bc8c0(Ptr) -> bool),
        entry!(0x009bd540, fn_009bd540(Ptr) -> f32),
        entry!(0x009bd560, fn_009bd560(Ptr<Projectile>, u32)),
        entry!(0x009bd580, fn_009bd580(Ptr<Projectile>, f32)),
        entry!(0x009bd5a0, fn_009bd5a0(Ptr<Projectile>, f32)),
        entry!(0x009bd5c0, fn_009bd5c0(Ptr, u32, u32, u32) -> Ptr<Projectile>),
        entry!(0x009bd760, fn_009bd760(Ptr<Projectile>, f32)),
        entry!(0x009bd780, fn_009bd780(u32, u32) -> Ptr<Projectile>),
        entry!(0x009bd830, fn_009bd830(Ptr, u32) -> bool),
        entry!(0x009bd860, projectile_aim_projectile(Ptr<Projectile>)),
        entry!(0x009bdff0, projectile_init_3d(Ptr<Projectile>)),
        entry!(0x009beae0, fn_009beae0(Ptr<Projectile>, u8)),
        entry!(0x009beb00, fn_009beb00(Ptr<Projectile>)),
        entry!(0x009bf190, fn_009bf190(Ptr<Projectile>)),
        entry!(0x009bf370, fn_009bf370(Ptr<Projectile>, f32, u32, u32, u32)),
        entry!(
            0x009bf5a0,
            projectile_add_impact(Ptr<Projectile>, u32, Ptr, Ptr, u32, u32, u32) -> bool
        ),
        entry!(0x009bc8f0, fn_009bc8f0(Ptr<Projectile>)),
        entry!(
            0x009bca60,
            fn_009bca60(Ptr, u32, u32, u32, u32, u32, u32, f32, f32, u32, u32, u8, u8, f32, f32, u32) -> Ptr<Projectile>
        ),
        entry!(0x009bda10, fn_009bda10(Ptr<Projectile>)),
        entry!(0x009bdf60, fn_009bdf60(Ptr<Projectile>, u32)),
        entry!(0x009bdf80, fn_009bdf80(Ptr<Projectile>) -> bool),
        entry!(0x009be0a0, fn_009be0a0(Ptr<Projectile>)),
        entry!(0x009bec10, fn_009bec10(Ptr<Projectile>, f32)),
        entry!(0x009bec50, fn_009bec50(Ptr<Projectile>, f32) -> bool),
        entry!(0x009bec90, fn_009bec90(Ptr<Projectile>)),
        entry!(0x009becc0, projectile_update(Ptr<Projectile>, u32)),
        entry!(0x009bf300, fn_009bf300(Ptr<Projectile>, f32)),
        entry!(
            0x009bf470,
            projectile_sync_3d_with_reference(Ptr<Projectile>, u8, u8)
        ),
        entry!(0x009bffb0, fn_009bffb0(Ptr<ImpactData>) -> Ptr<ImpactData>),
        entry!(0x009c0050, fn_009c0050(Ptr, u32)),
        entry!(0x009c0070, fn_009c0070(Ptr) -> u32),
        entry!(0x009c0090, fn_009c0090(Ptr<Projectile>) -> bool),
        entry!(
            0x009c03f0,
            projectile_run_target_pick(Ptr<Projectile>) -> bool
        ),
        entry!(
            0x009c0d80,
            fn_009c0d80(Ptr, Ptr<Projectile>, u32, u32, u8, f32) -> u32
        ),
        entry!(0x009c19e0, fn_009c19e0(Ptr) -> bool),
        entry!(0x009c1a00, fn_009c1a00(Ptr, u32) -> Ptr),
        entry!(0x009c1a20, fn_009c1a20(Ptr, u32)),
        entry!(0x009c1a50, fn_009c1a50(Ptr) -> u32),
        entry!(0x009c1a70, fn_009c1a70(Ptr, u32) -> Ptr),
        entry!(0x009c1aa0, fn_009c1aa0(Ptr, u32) -> Ptr),
        entry!(0x009c1b00, fn_009c1b00(Ptr, u32) -> Ptr),
        entry!(
            0x009c1b70,
            projectile_process_impacts(Ptr<Projectile>) -> bool
        ),
        entry!(
            0x009c20e0,
            fn_009c20e0(Ptr<Projectile>, u32, Ptr, Ptr, u32, u32)
        ),
        entry!(0x009c2c80, fn_009c2c80(u32, u32, u32, u32) -> bool),
        entry!(0x009c2e60, fn_009c2e60(Ptr) -> bool),
        entry!(
            0x009c2e80,
            fn_009c2e80(Ptr<Projectile>, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x009c2ff0, fn_009c2ff0(Ptr<Projectile>)),
        entry!(0x009c30d0, projectile_combat_hit(Ptr<Projectile>, u32, u8)),
        entry!(0x009c3190, projectile_check_explosion(Ptr<Projectile>, f32)),
        entry!(
            0x009c3930,
            projectile_get_mine_reacts_to_target(Ptr<Projectile>, u32, u32, u32) -> bool
        ),
        entry!(0x009c39e0, fn_009c39e0(Ptr<Projectile>) -> bool),
        entry!(0x009c_3d90, fn_009c3d90(Ptr<Projectile>)),
        entry!(0x009c_3e10, fn_009c3e10(Ptr<Projectile>)),
        entry!(0x009c_3e60, fn_009c3e60(Ptr<Projectile>)),
        entry!(0x009c_3f50, fn_009c3f50(f32, f32, f32) -> f32),
        entry!(
            0x009c_40c0,
            projectile_check_shot_behind_wall(Ptr<Projectile>) -> bool
        ),
        entry!(
            0x009c_45d0,
            projectile_change_process_level(Ptr<Projectile>) -> bool
        ),
        entry!(0x009c_4da0, fn_009c4da0(Ptr<Projectile>)),
        entry!(
            0x009c_4e60,
            fn_009c4e60(Ptr<Projectile>, u32, u32, u32, bool)
        ),
        entry!(
            0x009c_4f90,
            projectile_stop_countdown_sound(Ptr<Projectile>)
        ),
        entry!(
            0x009c_5af0,
            projectile_pick_auto_water(Ptr<Projectile>, Ptr, Ptr) -> bool
        ),
        entry!(0x009c_3ee0, fn_009c3ee0(Ptr<Projectile>, u32) -> u32),
        entry!(
            0x009c_3f90,
            fn_009c3f90(u32, i32, f32, f32, f32, f32, f32, f32, f32, f32)
        ),
        entry!(0x009c_43e0, fn_009c43e0(Ptr<Projectile>, u32, u32) -> bool),
        entry!(0x009c_47d0, fn_009c47d0(Ptr<Projectile>) -> bool),
        entry!(0x009c_4920, fn_009c4920(Ptr<Projectile>) -> bool),
        entry!(0x009c_4a40, fn_009c4a40(Ptr<Projectile>) -> bool),
        entry!(0x009c_4b60, fn_009c4b60(Ptr<Projectile>) -> bool),
        entry!(0x009c_4c80, fn_009c4c80(Ptr<Projectile>, u32)),
        entry!(
            0x009c_4ec0,
            projectile_start_countdown_sound(Ptr<Projectile>)
        ),
        entry!(0x009c_4ff0, projectile_save_game(Ptr<Projectile>, Ptr)),
        entry!(0x009c_52f0, projectile_load_game(Ptr<Projectile>, Ptr)),
        entry!(0x009c_5740, fn_009c5740(Ptr<Projectile>, Ptr)),
        entry!(0x009c_5870, fn_009c5870(Ptr<Projectile>, Ptr)),
        entry!(0x009c_58e0, fn_009c58e0(Ptr<Projectile>, Ptr)),
        entry!(0x009c_5ba0, fn_009c5ba0(u32, u32, u32, u32) -> bool),
        entry!(0x009c_5ee0, fn_009c5ee0(Ptr) -> Ptr),
        entry!(0x009c_5f30, fn_009c5f30(Ptr, Ptr) -> Ptr),
        entry!(
            0x009c_6030,
            projectile_aim_at_point(Ptr<Projectile>, u32, u32, u32)
        ),
        entry!(0x009c_6160, fn_009c6160(Ptr<Projectile>, u32)),
        entry!(
            0x009c_6330,
            projectile_set_projectile_orientation(Ptr<Projectile>, u32)
        ),
        entry!(0x009c_6440, fn_009c6440(Ptr, u32, u32) -> Ptr),
    ]
}

// Translated from 009bb970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::ImpactData::SaveGame` (Xbox PDB): writes location, normal,
/// material, limb and `bProcessed` (`00865e50`: pointer, size, 0), then the
/// collidee's world-object count (`GetHavokWorldObjectCountInSceneGraph`,
/// `004b6ae0`) and the target world object's index in it (`004b6b60`), both
/// zero without a collidee or target, and finally the collidee's form id
/// (`BGSSaveGameBuffer::SaveFormID_ov2`, `00865df0`).
pub fn projectile_impact_data_save_game(e: &mut Engine, this: Ptr<ImpactData>, buffer: Ptr) {
    const SAVE_DATA: u32 = 0x0086_5e50;
    for (offset, size) in [(0x04, 0xC), (0x10, 0xC), (0x20, 4), (0x24, 4), (0x28, 1)] {
        e.call(SAVE_DATA, &args![buffer, this.byte_add(offset), size, 0u32]);
    }
    e.set(this, ImpactData::sTargetWorldObjectCount, 0);
    e.set(this, ImpactData::sTargetWorldObjectIndex, 0);
    let collidee = e.get(this, ImpactData::pCollidee);
    if collidee != 0 && e.get(this, ImpactData::pTargetWorldObject) != 0 {
        let root = e.vcall(collidee, 0x1D0, &args![]).u32();
        let count = e.call(0x004b_6ae0, &args![root]).u16();
        e.set(this, ImpactData::sTargetWorldObjectCount, count as i16);
        let target = e.get(this, ImpactData::pTargetWorldObject);
        let collidee = e.get(this, ImpactData::pCollidee);
        let root = e.vcall(collidee, 0x1D0, &args![]).u32();
        let index = e.call(0x004b_6b60, &args![root, target]).u16();
        e.set(this, ImpactData::sTargetWorldObjectIndex, index as i16);
    }
    e.call(SAVE_DATA, &args![buffer, this.byte_add(0x2A), 2u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, this.byte_add(0x2C), 2u32, 0u32]);
    let collidee = e.get(this, ImpactData::pCollidee);
    e.call(0x0086_5df0, &args![buffer, collidee, 0u32]);
}

// Translated from 009bba90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::ImpactData::LoadGame` (Xbox PDB): reads back what
/// `SaveGame` wrote (`00864980`: pointer, size); the two world-object
/// words only when the buffer's version (its virtual slot 0) is above 0x10;
/// the collidee through `BGSLoadGameBuffer::LoadFormID_ov2` (`008648e0`).
pub fn projectile_impact_data_load_game(e: &mut Engine, this: Ptr<ImpactData>, buffer: Ptr) {
    const LOAD_DATA: u32 = 0x0086_4980;
    for (offset, size) in [(0x04, 0xC), (0x10, 0xC), (0x20, 4), (0x24, 4), (0x28, 1)] {
        e.call(LOAD_DATA, &args![buffer, this.byte_add(offset), size]);
    }
    let version = e.vcall(buffer.addr(), 0, &args![]).u8();
    if version >= 0x11 {
        e.call(LOAD_DATA, &args![buffer, this.byte_add(0x2A), 2u32]);
        e.call(LOAD_DATA, &args![buffer, this.byte_add(0x2C), 2u32]);
    }
    e.call(0x0086_48e0, &args![buffer, this]);
}

// Translated from 009bbb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `Projectile::FinishLoadGame` by the engine map; the body works on a
/// `Projectile::ImpactData`: when the saved world-object count is nonzero,
/// the index is not -1 and there is a collidee, it compares the count with
/// the collidee's current one (`004b6ae0`). Equal: the target world object
/// is looked up by index (`004b6c20`) and, when found, referenced
/// (`00c90510`). Different: `SaveGameWarning` (`00845cc0`) with the
/// collidee's name (virtual +0x130), form id, old and new count.
pub fn fn_009bbb40(e: &mut Engine, this: Ptr<ImpactData>, _unused_1: u32) {
    let count = e.get(this, ImpactData::sTargetWorldObjectCount);
    if count == 0
        || e.get(this, ImpactData::sTargetWorldObjectIndex) == -1
        || e.get(this, ImpactData::pCollidee) == 0
    {
        return;
    }
    let collidee = e.get(this, ImpactData::pCollidee);
    let root = e.vcall(collidee, 0x1D0, &args![]).u32();
    let current = e.call(0x004b_6ae0, &args![root]).u16() as i16;
    if count != current {
        let form_id = e.call(GET_FORM_ID, &args![collidee]).u32();
        let name = e.vcall(collidee, 0x130, &args![]).u32();
        e.call(
            0x0084_5cc0,
            &args![0x0109_0088u32, name, form_id, count as i32, current as i32],
        );
        return;
    }
    let index = e.get(this, ImpactData::sTargetWorldObjectIndex);
    if index == -1 {
        return;
    }
    let root = e.vcall(collidee, 0x1D0, &args![]).u32();
    let target = e.call(0x004b_6c20, &args![root, index as i32]).u32();
    e.set(this, ImpactData::pTargetWorldObject, target);
    if target != 0 {
        e.call(0x00c9_0510, &args![target]);
    }
}

// Translated from 009bbc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The projectile's deleting destructor: runs the destructor (`009bc470`)
/// and frees the object when bit 0 of `flags` is set.
pub fn fn_009bbc40(e: &mut Engine, this: Ptr<Projectile>, flags: u32) -> Ptr<Projectile> {
    fn_009bc470(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// The member constructors both projectile constructors run after
/// `MobileObject::MobileObject` (`0092eb50`).
fn construct_members(e: &mut Engine, this: Ptr<Projectile>) {
    e.call(0x0092_eb50, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.mem.set_u32(this.addr() + 0x18, CHILD_CELL_VTABLE);
    e.call(0x0096_a2d0, &args![this.byte_add(IMPACT_LIST)]);
    e.call(0x0047_6a80, &args![this.byte_add(FOLLOW_OFFSET)]);
    e.call(0x0068_15c0, &args![this.byte_add(MOVEMENT_DIRECTION)]);
    e.call(0x0063_3c90, &args![this.byte_add(LIGHT), 0u32]);
    e.call(0x0063_3c90, &args![this.byte_add(PICK_CACHE), 0u32]);
    e.call(0x0042_f730, &args![this.byte_add(ACTOR_CAUSE), 0u32]);
    e.call(0x0041_a250, &args![this.byte_add(SOUND_HANDLE)]);
    e.call(0x0041_a250, &args![this.byte_add(COUNTDOWN_SOUND)]);
    e.set(this, Projectile::pCurrentProcess, 0);
}

fn float_global(e: &Engine, address: u32) -> f32 {
    x87_quiet(e.global::<f32>(address))
}

// Translation incomplete: 009bbc70 - C++ exception unwinding (FS:[0] frame) is not modelled.
// Translated from 009bbc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::Projectile` (Xbox PDB): constructs the members and sets the
/// defaults (power, speed and alpha 1, explosion timer `FLT_MAX`, sound
/// distance 10000, the global zero point as movement direction, identity
/// follow offset), then `00484ab0(0)`. The stack word is never read.
pub fn projectile_ctor(e: &mut Engine, this: Ptr<Projectile>, _unused_1: u32) -> Ptr<Projectile> {
    construct_members(e, this);
    e.set(this, Projectile::iFlags, 0);
    e.set(this, Projectile::fPower, 1.0);
    e.set(this, Projectile::fSpeedMult, 1.0);
    e.set(this, Projectile::fRange, 0.0);
    e.set(this, Projectile::fAge, 0.0);
    e.set(this, Projectile::fDamage, 0.0);
    e.set(this, Projectile::fAlpha, 1.0);
    e.set(this, Projectile::fDistanceMoved, 0.0);
    e.set(this, Projectile::fSpreadHeading, 0.0);
    e.set(this, Projectile::fSpreadLooking, 0.0);
    copy_point(e, this.addr() + MOVEMENT_DIRECTION, ZERO_POINT);
    e.set(this, Projectile::pWeaponSource, 0);
    e.set(this, Projectile::pShooter, 0);
    e.set(this, Projectile::pDesiredTarget, 0);
    e.call(ASSIGN_NI_POINTER, &args![this.byte_add(LIGHT), 0u32]);
    e.call(ASSIGN_NI_POINTER, &args![this.byte_add(PICK_CACHE), 0u32]);
    e.call(ASSIGN_ACTOR_CAUSE, &args![this.byte_add(ACTOR_CAUSE), 0u32]);
    e.set(this, Projectile::bSendAlarm, false);
    e.set(this, Projectile::fDecalSize, 0.0);
    let timer = float_global(e, FLOAT_MAX);
    e.set(this, Projectile::fExplosionTimer, timer);
    e.set(this, Projectile::fBlinkTimer, 0.0);
    e.set(this, Projectile::bImpactProcessed, false);
    e.call(0x00a8_6bf0, &args![this.byte_add(FOLLOW_OFFSET)]);
    e.set(this, Projectile::pTarget, 0);
    e.set(this, Projectile::pSecondaryItemChange, 0);
    e.set(this, Projectile::fWeaponCondition, 1.0);
    e.set(this, Projectile::bSoundFired, false);
    let distance = float_global(e, INITIAL_SOUND_DISTANCE);
    e.set(this, Projectile::fSoundDist, distance);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    this
}

// Translation incomplete: 009bbef0 - C++ exception unwinding (FS:[0] frame) is not modelled.
// Translated from 009bbef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Projectile constructor from a base object, shooter, weapon, position and
/// angles (z through `005757d0`, x through `00575770`; y is set to 0).
///
/// With a weapon, the damage starts as the weapon's base damage (the `u16`
/// of the virtual +0x10 of its component at +0x9C). An actor shooter whose
/// object at `008d8520` returns an item (virtual +0x148) takes the item's
/// condition ratio (`ItemChange::GetItemHealth` over
/// `TESHealthForm::GetFormHealth`), its modified attack damage and whether
/// mod effect 0xC is active. The damage is divided by the weapon's projectile
/// count (`TESObjectWEAP::GetNumProjectiles`) and scaled by the `double` at
/// `01051678` when the mod effect is active. For the player with a Rock-It
/// launcher ammo the item's weight adds damage through two `00404010` calls
/// on the settings at `011ced70` and `011cf6b4`. An actor shooter for which
/// `00493bb0` holds sets flag 0x80000.
#[allow(clippy::too_many_arguments)]
pub fn fn_009bbef0(
    e: &mut Engine,
    this: Ptr<Projectile>,
    base_object: Ptr,
    shooter: u32,
    weapon: u32,
    x: u32,
    y: u32,
    z: u32,
    angle_z: f32,
    angle_x: f32,
) -> Ptr<Projectile> {
    construct_members(e, this);
    e.with_stack(NI_POINT3_SIZE, |e, location| {
        e.mem.set_u32(location.addr(), x);
        e.mem.set_u32(location.addr() + 4, y);
        e.mem.set_u32(location.addr() + 8, z);
        e.call(0x0057_5830, &args![this, location]);
    });
    e.call(0x0057_57d0, &args![this, x87_quiet(angle_z)]);
    e.call(0x0057_5770, &args![this, x87_quiet(angle_x)]);
    e.call(0x0057_57a0, &args![this, 0.0f32]);
    e.set(this, Projectile::pShooter, shooter);
    e.set(this, Projectile::pDesiredTarget, 0);
    e.set(this, Projectile::pWeaponSource, 0);
    e.set(this, Projectile::fPower, 1.0);
    e.set(this, Projectile::fSpeedMult, 1.0);
    e.set(this, Projectile::fDamage, 0.0);
    e.set(this, Projectile::fRange, 0.0);
    e.set(this, Projectile::fAlpha, 1.0);
    let timer = float_global(e, FLOAT_MAX);
    e.set(this, Projectile::fExplosionTimer, timer);
    e.set(this, Projectile::fBlinkTimer, 0.0);
    e.call(ASSIGN_NI_POINTER, &args![this.byte_add(PICK_CACHE), 0u32]);
    e.call(ASSIGN_ACTOR_CAUSE, &args![this.byte_add(ACTOR_CAUSE), 0u32]);
    e.set(this, Projectile::bImpactProcessed, false);
    e.set(this, Projectile::pTarget, 0);
    e.set(this, Projectile::pSecondaryItemChange, 0);
    e.set(this, Projectile::fWeaponCondition, 1.0);
    e.set(this, Projectile::bSoundFired, false);
    let distance = float_global(e, INITIAL_SOUND_DISTANCE);
    e.set(this, Projectile::fSoundDist, distance);
    e.call(0x0057_5690, &args![this, base_object]);

    if weapon != 0 {
        let mut mod_active = false;
        e.set(this, Projectile::pWeaponSource, weapon);
        let weapon_form = e.call(GET_WEAPON, &args![this]).u32();
        let damage_form = weapon_form + 0x9C;
        let base_damage = e.vcall(damage_form, 0x10, &args![]).u16();
        e.set(this, Projectile::fDamage, base_damage as f32);
        let mut actor = 0u32;
        if e.call(GET_SHOOTER, &args![this]).u32() != 0 {
            let shooter = e.call(GET_SHOOTER, &args![this]).u32();
            if is_actor(e, shooter) {
                actor = e.call(GET_SHOOTER, &args![this]).u32();
                if e.call(ACTOR_PROCESS_OBJECT, &args![actor]).u32() != 0 {
                    let process = e.call(ACTOR_PROCESS_OBJECT, &args![actor]).u32();
                    let item = e.vcall(process, 0x148, &args![]).u32();
                    if item != 0 {
                        let health = e.call(0x004b_cdb0, &args![item, 0u32]).f64();
                        let form = e.call(0x0044_ddc0, &args![item]).u32();
                        let form_health = e.call(0x0048_73d0, &args![form]).u32();
                        e.set(
                            this,
                            Projectile::fWeaponCondition,
                            (health / form_health as f64) as f32,
                        );
                        let owner = if actor != 0 { actor + 0xA4 } else { 0 };
                        let damage = e.call(0x004b_df00, &args![item, owner, 1.0f32, 1u32]).f64();
                        e.set(this, Projectile::fDamage, damage as f32);
                        mod_active = e.call(0x004b_da70, &args![item, 0xCu32]).bool();
                    }
                }
            }
        }
        let weapon_form = e.call(GET_WEAPON, &args![this]).u32();
        let projectiles = e
            .call(0x0052_5b20, &args![weapon_form, mod_active, 0u32, actor])
            .u8();
        let damage = e.get(this, Projectile::fDamage);
        e.set(
            this,
            Projectile::fDamage,
            (damage as f64 / projectiles as f64) as f32,
        );
        if mod_active {
            let scale: f64 = e.global(MOD_DAMAGE_SCALE);
            let damage = e.get(this, Projectile::fDamage);
            e.set(this, Projectile::fDamage, (damage as f64 * scale) as f32);
        }
    }

    let player: u32 = e.global(PLAYER);
    if e.call(GET_SHOOTER, &args![this]).u32() == player
        && e.call(GET_WEAPON, &args![this]).u32() != 0
    {
        let weapon_form = e.call(GET_WEAPON, &args![this]).u32();
        if e.call(0x0047_4a80, &args![weapon_form + 0xA4]).bool() {
            let player: u32 = e.global(PLAYER);
            let item = e.call(0x0096_8f90, &args![player]).u32();
            if item != 0 {
                e.call(0x005f_a310, &args![this, item]);
                if e.call(0x0044_ddc0, &args![item]).u32() != 0 {
                    let scale = e.call(SETTING_VALUE, &args![SETTING_ROCKIT_DAMAGE]).u32();
                    let player: u32 = e.global(PLAYER);
                    // 009bc318 / 009bc32a: the byte `004d1360` answers is pushed first; it
                    // is the second word of the cdecl `0048ebc0` (ADD ESP,4 pops one, the
                    // slot is reused for the next call's argument).
                    let flag = e.call(0x004d_1360, &args![player]).u8();
                    let form = e.call(0x0044_ddc0, &args![item]).u32();
                    let weight = e.call(0x0048_ebc0, &args![form, u32::from(flag)]).f64() as f32;
                    let limit = setting_float(e, SETTING_ROCKIT_WEIGHT);
                    let clamped = e.call(0x0040_4010, &args![limit, weight]).f64();
                    let bonus = (clamped * e.mem.f32(scale) as f64) as f32;
                    let bonus = e.call(0x0040_4010, &args![0.0f32, bonus]).f64();
                    let damage = e.get(this, Projectile::fDamage);
                    e.set(this, Projectile::fDamage, (bonus + damage as f64) as f32);
                }
            }
        }
    }

    e.set(this, Projectile::iFlags, 0);
    e.set(this, Projectile::fAge, 0.0);
    e.set(this, Projectile::fDistanceMoved, 0.0);
    copy_point(e, this.addr() + MOVEMENT_DIRECTION, ZERO_POINT);
    e.call(ASSIGN_NI_POINTER, &args![this.byte_add(LIGHT), 0u32]);
    e.set(this, Projectile::fDecalSize, 0.0);
    e.call(0x00a8_6bf0, &args![this.byte_add(FOLLOW_OFFSET)]);
    let shooter = e.get(this, Projectile::pShooter);
    if shooter != 0 && is_actor(e, shooter) {
        let shooter = e.get(this, Projectile::pShooter);
        if e.call(0x0049_3bb0, &args![shooter]).bool() {
            set_flags(e, this, 0x80000, true);
        }
    }
    e.call(0x0048_4ab0, &args![this, 0u32]);
    this
}

// Translation incomplete: 009bc470 - C++ exception unwinding (FS:[0] frame) is not modelled.
// Translated from 009bc470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The projectile's destructor: releases the secondary item change
/// (`004459e0(1)`), removes the 3D's master particle add-on nodes, clears
/// the 3D (`TESObjectREFR::Set3D(0, 1)`), stops the countdown sound
/// (`009c4f90`), fades out the sound unless the base object has flag 0x80
/// (`009bc870`), clears it from the VATS slots (`009c8b90`, `009c8c40`),
/// unregisters it with `ProcessLists` (flag 0x80, `00905330`), lets the
/// desired target forget it (flag 0x400), runs `009c3e10`, frees every
/// impact (`009b3490(1)`) while popping the list, removes it from the
/// manager at `011f1958` unless `004226e0` holds on `011c3f2c`, frees its
/// `ProjectileTarget` (`009bc890(1)`), releases its pointers and destroys
/// its members and the `MobileObject` base.
pub fn fn_009bc470(e: &mut Engine, this: Ptr<Projectile>) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.mem.set_u32(this.addr() + 0x18, CHILD_CELL_VTABLE);
    let item = e.get(this, Projectile::pSecondaryItemChange);
    if item != 0 {
        e.call(0x0044_59e0, &args![item, 1u32]);
    }
    e.set(this, Projectile::pSecondaryItemChange, 0);
    if e.call(0x0043_fcd0, &args![this]).u32() != 0 {
        let root = e.call(0x0043_fcd0, &args![this]).u32();
        if e.vcall(root, 0xC, &args![]).u32() != 0 {
            let root = e.call(0x0043_fcd0, &args![this]).u32();
            let node = e.vcall(root, 0xC, &args![]).u32();
            if e.call(0x0057_84b0, &args![node]).bool() {
                let root = e.call(0x0043_fcd0, &args![this]).u32();
                let node = e.vcall(root, 0xC, &args![]).u32();
                e.call(0x0057_8170, &args![node]);
            }
        }
    }
    e.call(0x0057_02e0, &args![this, 0u32, 1u32]);
    e.call(0x009c_4f90, &args![this]);
    if e.call(GET_BASE_OBJECT, &args![this]).u32() != 0 {
        let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
        if !fn_009bc870(e, Ptr::new(base)) {
            e.call(0x00ad_8da0, &args![this.byte_add(SOUND_HANDLE), 1000u32]);
        }
    }
    if e.call(0x0044_1110, &args![VATS]).u32() == this.addr() {
        e.call(0x009c_8b90, &args![VATS, 0u32]);
    }
    if e.call(0x007a_f430, &args![VATS]).u32() == this.addr() {
        e.call(0x009c_8c40, &args![VATS, 0u32]);
    }
    if test_flags(e, this, 0x80) {
        let list = e.call(0x0046_10f0, &args![PROCESS_LISTS]).u32();
        if list != 0 {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), this.addr());
                e.call(0x0090_5330, &args![list, slot]);
            });
        }
    }
    if test_flags(e, this, 0x400) && e.call(GET_DESIRED_TARGET, &args![this]).u32() != 0 {
        let target = e.call(GET_DESIRED_TARGET, &args![this]).u32();
        if is_actor(e, target) {
            let actor = e.call(GET_DESIRED_TARGET, &args![this]).u32();
            if e.call(ACTOR_PROCESS_OBJECT, &args![actor]).u32() != 0 {
                let process = e.call(ACTOR_PROCESS_OBJECT, &args![actor]).u32();
                e.vcall(process, 0x384, &args![0u32]);
            }
        }
        let target = e.call(GET_DESIRED_TARGET, &args![this]).u32();
        let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
        e.vcall(
            target,
            0x17C,
            &args![base, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
        );
    }
    e.call(0x009c_3e10, &args![this]);
    let list = this.byte_add(IMPACT_LIST);
    while !e.call(0x0082_56d0, &args![list]).bool() {
        let head = e.call(0x0068_15c0, &args![list]).u32();
        let impact = e.mem.u32(head);
        if impact != 0 {
            e.call(0x009b_3490, &args![impact, 1u32]);
        }
        e.call(0x0063_f7b0, &args![list]);
    }
    let game: u32 = e.global(0x011c_3f2c);
    if !e.call(0x0042_26e0, &args![game]).bool() {
        e.call(0x0092_ed60, &args![this, 0u32]);
        let manager: u32 = e.global(0x011f_1958);
        let manager = e.call(0x0082_5c00, &args![manager]).u32();
        e.call(0x009a_51e0, &args![manager, this]);
    }
    let target = e.get(this, Projectile::pTarget);
    if target != 0 {
        fn_009bc890(e, Ptr::new(target), 1);
        e.set(this, Projectile::pTarget, 0);
    }
    e.call(ASSIGN_NI_POINTER, &args![this.byte_add(PICK_CACHE), 0u32]);
    e.call(ASSIGN_ACTOR_CAUSE, &args![this.byte_add(ACTOR_CAUSE), 0u32]);
    e.call(0x0048_3710, &args![this.byte_add(COUNTDOWN_SOUND)]);
    e.call(0x0048_3710, &args![this.byte_add(SOUND_HANDLE)]);
    e.call(0x0042_f760, &args![this.byte_add(ACTOR_CAUSE)]);
    e.call(0x0045_cec0, &args![this.byte_add(PICK_CACHE)]);
    e.call(0x0045_cec0, &args![this.byte_add(LIGHT)]);
    e.call(0x0046_ffb0, &args![this.byte_add(IMPACT_LIST)]);
    e.call(0x0092_ec40, &args![this]);
}

// Translated from 009bc870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the base object has flag 0x80 (`004fd420`).
pub fn fn_009bc870(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x004f_d420, &args![this, 0x80u32]).bool()
}

// Translated from 009bc890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of the projectile's target (`pTarget`): runs the
/// destructor `00800190` and frees the object when bit 0 of `flags` is set.
pub fn fn_009bc890(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0080_0190, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 009bc8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the object's form id is known to the object at `011c3f2c`
/// (`00469860`).
pub fn fn_009bc8c0(e: &mut Engine, this: Ptr) -> bool {
    let form_id = e.call(GET_FORM_ID, &args![this]).u32();
    let game: u32 = e.global(0x011c_3f2c);
    e.call(0x0046_9860, &args![game, form_id]).bool()
}

// Translated from 009bd540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0x730 of the object (not a `Projectile` field; the
/// owning class is unconfirmed), loaded onto the x87 stack.
pub fn fn_009bd540(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x730))
}

// Translated from 009bd560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `spPickCache` (`0066b0d0`).
pub fn fn_009bd560(e: &mut Engine, this: Ptr<Projectile>, cache: u32) {
    e.call(ASSIGN_NI_POINTER, &args![this.byte_add(PICK_CACHE), cache]);
}

// Translated from 009bd580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fSpreadHeading`.
pub fn fn_009bd580(e: &mut Engine, this: Ptr<Projectile>, value: f32) {
    e.set(this, Projectile::fSpreadHeading, x87_quiet(value));
}

// Translated from 009bd5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fSpreadLooking`.
pub fn fn_009bd5a0(e: &mut Engine, this: Ptr<Projectile>, value: f32) {
    e.set(this, Projectile::fSpreadLooking, x87_quiet(value));
}

// Translation incomplete: 009bd5c0 - C++ exception unwinding (FS:[0] frame) is not modelled.
// Translated from 009bd5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates a projectile of the 0x154-byte subclass (`009b4020`) when
/// `base_object`'s form type (`004fd4f0`) is 0x20000, at the global zero
/// point with zero angles. An actor `shooter` gets an `ActorCause` (0x1C
/// bytes, `0066e010`) through the projectile's virtual +0x164. Then sets
/// flag 0x400, `00420ba0(target)`, runs its virtual +0x308 and sets the
/// explosion timer from the setting at `011ce6cc`. Null when no projectile
/// was made.
pub fn fn_009bd5c0(
    e: &mut Engine,
    base_object: Ptr,
    shooter: u32,
    target: u32,
    weapon: u32,
) -> Ptr<Projectile> {
    let mut projectile = 0u32;
    if !base_object.is_null() && e.call(0x004f_d4f0, &args![base_object]).u32() == 0x20000 {
        let block = allocate(e, 0x154);
        projectile = if block != 0 {
            let [x, y, z] = point_words(e, ZERO_POINT);
            e.call(
                0x009b_4020,
                &args![block, base_object, shooter, weapon, x, y, z, 0.0f32, 0.0f32],
            )
            .u32()
        } else {
            0
        };
    }
    if projectile != 0 {
        if shooter != 0 && is_actor(e, shooter) {
            let block = allocate(e, 0x1C);
            let cause = if block != 0 {
                e.call(0x0066_e010, &args![block, shooter]).u32()
            } else {
                0
            };
            e.vcall(projectile, 0x164, &args![cause]);
        }
        set_flags(e, Ptr::new(projectile), 0x400, true);
        e.call(0x0042_0ba0, &args![projectile, target]);
        e.vcall(projectile, 0x308, &args![]);
        let timer = setting_float(e, SETTING_EXPLOSION_TIMER);
        fn_009bd760(e, Ptr::new(projectile), timer);
    }
    Ptr::new(projectile)
}

// Translated from 009bd760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fExplosionTimer`.
pub fn fn_009bd760(e: &mut Engine, this: Ptr<Projectile>, value: f32) {
    e.set(this, Projectile::fExplosionTimer, x87_quiet(value));
}

// Translated from 009bd780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds a projectile in the `ProcessLists` list `00717e50` returns (count
/// `005be5c0(0)`, element `00968670(i)`) whose `IsProjectile` (virtual
/// +0x224) holds, whose base object is `base_object` and whose shooter is
/// `shooter`; null when none.
pub fn fn_009bd780(e: &mut Engine, shooter: u32, base_object: u32) -> Ptr<Projectile> {
    let mut found = 0u32;
    let mut index = 0u32;
    while found == 0 {
        let list = e.call(0x0071_7e50, &args![PROCESS_LISTS]).u32();
        if index >= e.call(0x005b_e5c0, &args![list, 0u32]).u32() {
            break;
        }
        let list = e.call(0x0071_7e50, &args![PROCESS_LISTS]).u32();
        let object = e.call(0x0096_8670, &args![list, index]).u32();
        if object != 0
            && e.vcall(object, 0x224, &args![]).bool()
            && e.call(GET_BASE_OBJECT, &args![object]).u32() == base_object
            && e.call(GET_SHOOTER, &args![object]).u32() == shooter
        {
            found = object;
        }
        index += 1;
    }
    Ptr::new(found)
}

// Translated from 009bd830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `TESObjectREFR::Load` per the engine map (`0055af90`) with
/// the one argument and returns its `bool`.
pub fn fn_009bd830(e: &mut Engine, this: Ptr, value: u32) -> bool {
    e.call(0x0055_af90, &args![this, value]).bool()
}

// Translation incomplete: 009bd860 - C++ exception unwinding (FS:[0] frame) is not modelled.
// Translated from 009bd860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::AimProjectile` (Xbox PDB). Skipped for a projectile with flag
/// 4 but not 8 while VATS (`0044ddc0` on `011f2250`) is not in mode 4.
/// Otherwise the fire node is the player's (virtual +0x20C) or the
/// projectile's own 3D (virtual +0x1D0). In VATS mode 4 with a current
/// action whose byte +4 is set, and either flag 0x40000 or no flag 4, a
/// 0x14-byte object (`009c5ee0`) is passed to `008126e0`. Without a pick
/// cache (`00559450`) the player aims it (`00965620` with the angles and
/// position and the `float` at `0102efc4` twice); then
/// `Projectile::Sync3DWithReference(1, 0)`.
pub fn projectile_aim_projectile(e: &mut Engine, this: Ptr<Projectile>) {
    let mut aim = true;
    if test_flags(e, this, 4)
        && !test_flags(e, this, 8)
        && e.call(0x0044_ddc0, &args![VATS]).u32() != 4
    {
        aim = false;
    }
    if !aim {
        return;
    }
    let player: u32 = e.global(PLAYER);
    let mut node = e.vcall(player, 0x20C, &args![]).u32();
    if node == 0 {
        node = e.vcall(this.addr(), 0x1D0, &args![]).u32();
    }
    let check_vats = test_flags(e, this, 0x40000) || !test_flags(e, this, 4);
    if check_vats
        && e.call(0x0044_ddc0, &args![VATS]).u32() == 4
        && e.call(0x009c_71c0, &args![VATS]).u32() != 0
    {
        let action = e.call(0x009c_71c0, &args![VATS]).u32();
        if e.mem.u8(action + 4) != 0 {
            let block = allocate(e, 0x14);
            let object = if block != 0 {
                e.call(0x009c_5ee0, &args![block]).u32()
            } else {
                0
            };
            e.call(0x0081_26e0, &args![this, object]);
        }
    }
    if e.call(0x0055_9450, &args![this.byte_add(PICK_CACHE)]).u32() == 0 {
        let spread = float_global(e, 0x0102_efc4);
        let player: u32 = e.global(PLAYER);
        e.call(
            0x0096_5620,
            &args![
                player,
                this,
                node,
                this.byte_add(REFR_ANGLE_Z),
                this.byte_add(REFR_ANGLE_X),
                this.byte_add(REFR_POSITION),
                spread,
                spread
            ],
        );
    }
    projectile_sync_3d_with_reference(e, this, 1, 0);
}

// Translated from 009bdff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::Init3D` (Xbox PDB): sets the 3D (virtual +0x1C8 with 0) and,
/// when there is one (virtual +0x1D0), sets its scale to 1 (`00440490`),
/// syncs it with the reference, adds the master particle add-on nodes when
/// its object (virtual +0xC) has add-on flags, and starts its animations.
pub fn projectile_init_3d(e: &mut Engine, this: Ptr<Projectile>) {
    e.vcall(this.addr(), 0x1C8, &args![0u32]);
    let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
    if root == 0 {
        return;
    }
    e.call(0x0044_0490, &args![root, 1.0f32]);
    projectile_sync_3d_with_reference(e, this, 1, 0);
    if e.vcall(root, 0xC, &args![]).u32() != 0 {
        let node = e.vcall(root, 0xC, &args![]).u32();
        if e.call(0x0057_84b0, &args![node]).bool() {
            let node = e.vcall(root, 0xC, &args![]).u32();
            e.call(0x0057_8060, &args![node]);
        }
    }
    e.call(0x00a6_d2d0, &args![root]);
}

// Translated from 009beae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at +0x95 (inside the `kFollowOffset` transform's first
/// word on PC; the field's meaning is unconfirmed).
pub fn fn_009beae0(e: &mut Engine, this: Ptr<Projectile>, value: u8) {
    e.mem.set_u8(this.addr() + 0x95, value);
}

// Translated from 009beb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a projectile with flag 4 but not 8 whose form id `00469860` knows on
/// `011c3f2c`: re-arms the 3D's physics (`00c6a040(root, 0, 1, 1)`, a zero
/// point to its virtual +0xB8, `00c6a040(root, 1, 1, 1)`), takes the
/// direction `00439f50` gives for the root's `00461130`, scales it by
/// `009669c0` and adds it as velocity (`TESHavokUtilities::AddVelocity`),
/// then marks change 4 (virtual +0x48).
pub fn fn_009beb00(e: &mut Engine, this: Ptr<Projectile>) {
    if test_flags(e, this, 8) || !test_flags(e, this, 4) {
        return;
    }
    let form_id = e.call(GET_FORM_ID, &args![this]).u32();
    let game: u32 = e.global(0x011c_3f2c);
    if !e.call(0x0046_9860, &args![game, form_id]).bool() {
        return;
    }
    let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
    if root == 0 {
        return;
    }
    e.with_stack(2 * NI_POINT3_SIZE, |e, locals| {
        let zero = locals.addr();
        let velocity = locals.addr() + NI_POINT3_SIZE;
        e.call(0x00c6_a040, &args![root, 0u32, 1u32, 1u32]);
        e.call(0x0043_d410, &args![zero, 0.0f32, 0u32, 0u32]);
        e.vcall(root, 0xB8, &args![zero]);
        e.call(0x00c6_a040, &args![root, 1u32, 1u32, 1u32]);
        e.call(0x0068_15c0, &args![velocity]);
        let body = e.call(0x0046_1130, &args![root]).u32();
        e.call(0x0043_9f50, &args![body, 1u32, velocity]);
        let speed = e.call(0x0096_69c0, &args![this]).f64() as f32;
        e.call(0x0043_9180, &args![velocity, speed]);
        e.call(0x0062_b8d0, &args![root, velocity, 1u32]);
    });
    e.vcall(this.addr(), 0x48, &args![4u32]);
}

// Translated from 009bf190 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a projectile with flag 0x400 whose base object lacks flag 0x400
/// (`004fd420`), once its `00885bd0` value passes the setting at `011ced88`
/// (strictly, unordered fails) and flag 0x800 is not set: sets flag 0x800,
/// hands it to the manager at `011f1958` (`009a52f0`) and, when the desired
/// target is an actor with an object at `008d8520`, alerts it (virtual
/// +0x384 with 1, +0x2A4 with the topic `TESTopic::GetTopic(2, 5)`, +0x44).
pub fn fn_009bf190(e: &mut Engine, this: Ptr<Projectile>) {
    if !test_flags(e, this, 0x400) {
        return;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if e.call(0x004f_d420, &args![base, 0x400u32]).bool() {
        return;
    }
    let value = e.call(0x0088_5bd0, &args![this]).f64();
    let threshold = e.call(SETTING_VALUE, &args![SETTING_ALARM_DISTANCE]).u32();
    let threshold = e.mem.f32(threshold) as f64;
    if !(threshold < value) {
        return;
    }
    if test_flags(e, this, 0x800) {
        return;
    }
    set_flags(e, this, 0x800, true);
    let manager: u32 = e.global(0x011f_1958);
    let manager = e.call(0x0082_5c00, &args![manager]).u32();
    e.call(0x009a_52f0, &args![manager, this]);
    if e.call(GET_DESIRED_TARGET, &args![this]).u32() == 0 {
        return;
    }
    let target = e.call(GET_DESIRED_TARGET, &args![this]).u32();
    if !is_actor(e, target) {
        return;
    }
    let actor = e.call(GET_DESIRED_TARGET, &args![this]).u32();
    if e.call(ACTOR_PROCESS_OBJECT, &args![actor]).u32() == 0 {
        return;
    }
    let process = e.call(ACTOR_PROCESS_OBJECT, &args![actor]).u32();
    e.vcall(process, 0x384, &args![1u32]);
    let topic = e.call(0x0061_a2d0, &args![2u32, 5u32]).u32();
    let process = e.call(ACTOR_PROCESS_OBJECT, &args![actor]).u32();
    e.vcall(process, 0x2A4, &args![actor, topic, 0u32, 0u32, 0u32, 0u32]);
    let process = e.call(ACTOR_PROCESS_OBJECT, &args![actor]).u32();
    e.vcall(process, 0x44, &args![actor, 0u32, 2u32, 1u32, 1u32, 1u32]);
}

// Translated from 009bf370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the projectile by `delta` (scaled by `1 / 00716440()` when the
/// shooter is an actor whose actor value 0x33 is positive, virtual +8 of
/// its +0xA4 component) towards the point `(x, y, z)` with
/// `MobileObject::Move(.., 0xF)`, then passes the difference of the
/// position (virtual +0x1F4) after and before (`00439ef0`) to `009c4e60`
/// with 1.
pub fn fn_009bf370(e: &mut Engine, this: Ptr<Projectile>, delta: f32, x: u32, y: u32, z: u32) {
    let position = e.vcall(this.addr(), 0x1F4, &args![]).u32();
    let before = point_words(e, position);
    let mut delta = delta;
    let shooter = e.get(this, Projectile::pShooter);
    if shooter != 0 && is_actor(e, shooter) {
        let shooter = e.get(this, Projectile::pShooter);
        let value = e.vcall(shooter + 0xA4, 8, &args![0x33u32]).i32();
        if value > 0 {
            let scale = e.call(0x0071_6440, &args![]).f64();
            delta = ((1.0 / scale) * delta as f64) as f32;
        }
    }
    e.with_stack(3 * NI_POINT3_SIZE + NI_POINT3_SIZE, |e, locals| {
        let target = locals.addr();
        let old = target + NI_POINT3_SIZE;
        let new = old + NI_POINT3_SIZE;
        let difference = new + NI_POINT3_SIZE;
        e.mem.set_u32(target, x);
        e.mem.set_u32(target + 4, y);
        e.mem.set_u32(target + 8, z);
        e.call(0x0092_f260, &args![this, x87_quiet(delta), target, 0xFu32]);
        for (i, word) in before.iter().enumerate() {
            e.mem.set_u32(old + 4 * i as u32, *word);
        }
        let position = e.vcall(this.addr(), 0x1F4, &args![]).u32();
        copy_point(e, new, position);
        let result = e.call(0x0043_9ef0, &args![new, difference, old]).u32();
        let [dx, dy, dz] = point_words(e, result);
        e.call(0x009c_4e60, &args![this, dx, dy, dz, 1u32]);
    });
}

/// Sets a debug colour (`NiColorA` constructor `00414430`: r, g, b, a) in a
/// temporary and copies it into `color`.
fn debug_color(e: &mut Engine, color: u32, rgba: [f32; 4]) {
    e.with_stack(0x10, |e, temporary| {
        let made = e
            .call(
                0x0041_4430,
                &args![temporary, rgba[0], rgba[1], rgba[2], rgba[3]],
            )
            .u32();
        for offset in [0, 4, 8, 12] {
            let word = e.mem.u32(made + offset);
            e.mem.set_u32(color + offset, word);
        }
    });
}

/// The name a reference reports for the VATS debug lines: virtual +0x130
/// when `00474cb0` returns nonzero, else `0055d520`.
fn debug_reference_name(e: &mut Engine, reference: u32) -> u32 {
    if e.call(0x0047_4cb0, &args![reference]).u32() != 0 {
        e.vcall(reference, 0x130, &args![]).u32()
    } else {
        e.call(0x0055_d520, &args![reference]).u32()
    }
}

/// The name of a scene-graph object for the VATS debug lines (`00413f40`,
/// then `0043b1b0`), or the empty string at `01011584` for null.
fn debug_part_name(e: &mut Engine, object: u32) -> u32 {
    if object == 0 {
        return 0x0101_1584;
    }
    let name = e.call(0x0041_3f40, &args![object]).u32();
    e.call(0x0043_b1b0, &args![name]).u32()
}

// Translation incomplete: 009bf5a0 - C++ exception unwinding (FS:[0] frame) is not modelled.
// Translated from 009bf5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::AddImpact` (Xbox PDB): makes an `ImpactData` (0x30 bytes,
/// `009bffb0`) and appends it to `ImpactList` (`005ae3d0`); returns true.
///
/// With flag 0x1000 and a desired target the impact did not hit, the impact
/// is redirected to the desired target: its position, the movement
/// direction's `004a0bd0` normalised (`004a0c10`), and for an actor
/// material 6, the body part `008b3d70` finds along the path (with its limb
/// `008b3ef0` and world object through `006838b0`, `006fa820`,
/// `00620b80`, referenced by `00c90510`). Otherwise the impact takes the
/// collidee, location and normal, the collidable's world object
/// (`004b59d0`, referenced), the limb an actor collidee's hit node gives,
/// and the material: material 0x24 on a collidable is resolved from the
/// shape (`004a3a20`/`004a3a40`/`00c84f10`, or the TES at `00457880` for a
/// type-1 shape) and becomes 8 for a type 0xB shape; material 8 adds a
/// water ripple (directly, or queued when `008c7aa0` holds).
///
/// With the VATS targeting debug setting and a VATS target (`008d8b80`)
/// it logs whether the hit matched. The desired collidee is marked
/// targeted. For a type-5 projectile (virtual +0x304) whose hit count
/// (`009c0070`) over the weapon's `00821640` is below `00885bd0`, the
/// shooter is notified (virtual +0x3F0), the count goes up (`009c0050`),
/// `bDoDamage` says whether the collidee is an actor, and the beam is
/// killed (`ContinuousBeamProjectile::SetKill`) when the shooter's
/// `008a88b0` is below 1; otherwise `bDoDamage` is cleared. With either
/// impact debug setting a coloured marker and normal line are drawn.
#[allow(clippy::too_many_arguments)]
pub fn projectile_add_impact(
    e: &mut Engine,
    this: Ptr<Projectile>,
    collidee: u32,
    location: Ptr,
    normal: Ptr,
    collidable: u32,
    shape_key: u32,
    material: u32,
) -> bool {
    let block = allocate(e, 0x30);
    let impact = if block != 0 {
        fn_009bffb0(e, Ptr::new(block)).addr()
    } else {
        0
    };
    let data: Ptr<ImpactData> = Ptr::new(impact);
    e.with_stack(4, |e, impact_slot| {
        e.mem.set_u32(impact_slot.addr(), impact);
        let desired = e.get(this, Projectile::pDesiredTarget);
        if test_flags(e, this, 0x1000)
            && desired != 0
            && e.get(data, ImpactData::pCollidee) != desired
        {
            redirect_impact_to_desired_target(e, this, data);
        } else {
            fill_impact(
                e, this, data, collidee, location, normal, collidable, shape_key, material,
            );
        }

        let mut vats_hit = true;
        if bool_setting(e, SETTING_DEBUG_VATS_TARGETING)
            && e.call(0x008d_8b80, &args![this]).u32() != 0
        {
            vats_hit = log_vats_hit(e, this, collidee, collidable);
        }
        let hit = e.get(data, ImpactData::pCollidee);
        if hit != 0 {
            e.call(0x0056_4db0, &args![hit, 1u32]);
        }
        if e.vcall(this.addr(), 0x304, &args![]).u32() == 5 && !this.is_null() {
            count_beam_hit(e, this, data);
        }
        if bool_setting(e, SETTING_DEBUG_IMPACTS) || bool_setting(e, SETTING_DEBUG_VATS_TARGETING) {
            draw_impact_debug(e, data, vats_hit);
        }
        e.call(0x005a_e3d0, &args![this.byte_add(IMPACT_LIST), impact_slot]);
    });
    true
}

fn redirect_impact_to_desired_target(e: &mut Engine, this: Ptr<Projectile>, data: Ptr<ImpactData>) {
    let target = e.get(this, Projectile::pDesiredTarget);
    e.set(data, ImpactData::pCollidee, target);
    e.set(data, ImpactData::pTargetWorldObject, 0);
    let position = e.vcall(target, 0x1F4, &args![]).u32();
    copy_point(e, data.addr() + 4, position);
    e.with_stack(NI_POINT3_SIZE, |e, local| {
        let direction = e
            .call(
                0x004a_0bd0,
                &args![this.byte_add(MOVEMENT_DIRECTION), local],
            )
            .u32();
        copy_point(e, data.addr() + 0x10, direction);
    });
    e.call(0x004a_0c10, &args![data.byte_add(0x10)]);
    e.set(data, ImpactData::eDamageLimb, 0);
    e.set(data, ImpactData::eMaterial, 0);
    let target = e.get(this, Projectile::pDesiredTarget);
    if !is_actor(e, target) {
        return;
    }
    e.set(data, ImpactData::eMaterial, 6);
    let actor = e.get(this, Projectile::pDesiredTarget);
    let [lx, ly, lz] = point_words(e, data.addr() + 4);
    let part = e.with_stack(NI_POINT3_SIZE, |e, local| {
        let start = e
            .call(
                0x0043_9ef0,
                &args![data.byte_add(4), local, this.byte_add(MOVEMENT_DIRECTION)],
            )
            .u32();
        let [sx, sy, sz] = point_words(e, start);
        e.call(0x008b_3d70, &args![actor, sx, sy, sz, lx, ly, lz])
            .u32()
    });
    if part != 0 {
        let limb = e.call(0x008b_3ef0, &args![actor, part]).u32();
        e.set(data, ImpactData::eDamageLimb, limb);
        let collision = e.call(0x0068_38b0, &args![part]).u32();
        let world_object = if e.call(0x006f_a820, &args![collision]).u32() != 0 {
            let body = e.call(0x006f_a820, &args![collision]).u32();
            e.call(0x0062_0b80, &args![body]).u32()
        } else {
            0
        };
        e.set(data, ImpactData::pTargetWorldObject, world_object);
    }
    let world_object = e.get(data, ImpactData::pTargetWorldObject);
    if world_object != 0 {
        e.call(0x00c9_0510, &args![world_object]);
    }
}

#[allow(clippy::too_many_arguments)]
fn fill_impact(
    e: &mut Engine,
    _this: Ptr<Projectile>,
    data: Ptr<ImpactData>,
    collidee: u32,
    location: Ptr,
    normal: Ptr,
    collidable: u32,
    shape_key: u32,
    material: u32,
) {
    e.set(data, ImpactData::pCollidee, collidee);
    copy_point(e, data.addr() + 4, location.addr());
    copy_point(e, data.addr() + 0x10, normal.addr());
    let world_object = if collidable != 0 {
        e.call(0x004b_59d0, &args![collidable]).u32()
    } else {
        0
    };
    e.set(data, ImpactData::pTargetWorldObject, world_object);
    if world_object != 0 {
        e.call(0x00c9_0510, &args![world_object]);
        if collidee != 0
            && is_actor(e, collidee)
            && e.call(0x004b_5980, &args![collidable]).u32() == 1
        {
            let shape = e.call(0x0043_b540, &args![collidable]).u32();
            let shape_holds = e.with_stack(4, |e, local| {
                let wrapped = e.call(0x008c_71b0, &args![local, shape]).u32();
                e.call(0x0062_4070, &args![wrapped]).bool()
            });
            if shape_holds {
                let node = e.call(0x004b_5820, &args![collidable]).u32();
                let hit = e.vcall(node, 0xC, &args![]).u32();
                let limb = e.call(0x008b_3ef0, &args![collidee, hit]).u32();
                e.set(data, ImpactData::eDamageLimb, limb);
            }
        }
    }
    let mut resolved = material;
    if collidable != 0 && material == 0x24 {
        let shape = e.call(0x0043_b540, &args![collidable]).u32();
        let type_one = e.with_stack(4, |e, local| {
            let wrapped = e.call(0x008c_71b0, &args![local, shape]).u32();
            e.call(0x004a_3a20, &args![wrapped]).u32() == 1
        });
        if type_one {
            let tes: u32 = e.global(TES);
            resolved = e.call(0x0045_7880, &args![tes, location]).u32();
        } else {
            let node = e.call(0x0055_9450, &args![collidable]).u32();
            let shape = e.call(0x004a_3a40, &args![node]).u32();
            if shape != 0 {
                resolved = e.call(0x00c8_4f10, &args![shape, shape_key]).u32();
            }
        }
        let shape = e.call(0x0043_b540, &args![collidable]).u32();
        let kind = e.with_stack(4, |e, local| {
            let wrapped = e.call(0x008c_71b0, &args![local, shape]).u32();
            e.call(0x0043_b4d0, &args![wrapped]).u32()
        });
        if kind == 0xB {
            resolved = 8;
        }
    }
    e.set(data, ImpactData::eMaterial, resolved);
    if resolved == 8 {
        let size = float_global(e, 0x0101_3ea4);
        if !e.call(0x008c_7aa0, &args![]).bool() {
            let [x, y, z] = point_words(e, location.addr());
            let tes: u32 = e.global(TES);
            let water = e.call(0x0070_ec90, &args![tes]).u32();
            e.call(0x004e_8510, &args![water, x, y, z, size]);
        } else {
            let queue = e.call(0x0045_37b0, &args![]).u32();
            e.call(0x0087_b760, &args![queue, size, location]);
        }
    }
}

/// The VATS targeting debug log; returns whether the hit reference and part
/// are the VATS target's.
fn log_vats_hit(e: &mut Engine, this: Ptr<Projectile>, collidee: u32, collidable: u32) -> bool {
    const ERROR: u32 = 0x0040_fbe0;
    let hit_part = e.call(0x004b_5820, &args![collidable]).u32();
    let vats_target = e.call(0x008d_8b80, &args![this]).u32();
    let node = e.call(0x0055_9450, &args![vats_target + 4]).u32();
    let first = e.call(0x004b_5260, &args![node]).u32();
    let target_part = if first != 0 {
        e.call(0x0044_ddc0, &args![first]).u32()
    } else {
        0
    };
    let vats_target = e.call(0x008d_8b80, &args![this]).u32();
    let target = e.mem.u32(vats_target);
    let success = collidee == target && hit_part == target_part;
    let result = if success { 0x0109_048cu32 } else { 0x0109_045c };
    e.call(ERROR, &args![0x0109_0434u32, result]);
    let name = debug_reference_name(e, target);
    let form_id = e.call(GET_FORM_ID, &args![target]).u32();
    e.call(ERROR, &args![0x0109_041cu32, name, form_id]);
    let part = debug_part_name(e, target_part);
    e.call(ERROR, &args![0x0109_040cu32, part]);
    let hit_id = if collidee != 0 {
        e.call(GET_FORM_ID, &args![collidee]).u32()
    } else {
        0
    };
    let hit_name = if collidee != 0 {
        debug_reference_name(e, collidee)
    } else {
        0x0103_b168
    };
    e.call(ERROR, &args![0x0109_0474u32, hit_name, hit_id]);
    let part = debug_part_name(e, hit_part);
    e.call(ERROR, &args![0x0109_0464u32, part]);
    success
}

fn count_beam_hit(e: &mut Engine, this: Ptr<Projectile>, data: Ptr<ImpactData>) {
    let hits = fn_009c0070(e, this.cast()) as i32;
    let limit = e.call(0x0088_5bd0, &args![this]).f64();
    let weapon = e.call(GET_WEAPON, &args![this]).u32();
    let rate = e.call(0x0082_1640, &args![weapon]).f64();
    if hits as f64 / rate < limit {
        let shooter = e.call(GET_SHOOTER, &args![this]).u32();
        if shooter != 0 {
            e.vcall(shooter, 0x3F0, &args![1u32]);
        }
        fn_009c0050(e, this.cast(), hits.wrapping_add(1) as u32);
        let collidee = e.get(data, ImpactData::pCollidee);
        let do_damage = collidee != 0 && is_actor(e, collidee);
        e.set(data, ImpactData::bDoDamage, do_damage);
        if e.call(0x008a_88b0, &args![shooter]).i32() < 1 {
            e.call(0x009a_b9a0, &args![this, shooter]);
        }
    } else {
        e.set(data, ImpactData::bDoDamage, false);
    }
}

fn draw_impact_debug(e: &mut Engine, data: Ptr<ImpactData>, vats_hit: bool) {
    let alpha = float_global(e, 0x0101_e2bc);
    let strong_alpha = float_global(e, 0x0102_3934);
    let half = float_global(e, 0x0101_6248);
    let marker_size = float_global(e, 0x0101_712c);
    let normal_length = float_global(e, 0x0104_e0e8);
    e.with_stack(0x10 + 2 * NI_POINT3_SIZE, |e, locals| {
        let color = locals.addr();
        e.call(0x0041_4430, &args![color, 0.0f32, 0.0f32, 1.0f32, alpha]);
        let collidee = e.get(data, ImpactData::pCollidee);
        if collidee != 0 && is_actor(e, collidee) {
            debug_color(e, color, [0.0, 1.0, 0.0, alpha]);
        }
        if !vats_hit {
            debug_color(e, color, [1.0, 0.0, 0.0, strong_alpha]);
        }
        if e.get(data, ImpactData::bDoDamage) {
            debug_color(e, color, [half, 0.0, 0.0, strong_alpha]);
        }
        let marker = e.call(0x004b_29b0, &args![marker_size, color, 1u32]).u32();
        e.call(0x00a5_a040, &args![marker]);
        e.call(0x0044_0460, &args![marker, data.byte_add(4)]);
        let time = setting_float(e, SETTING_DEBUG_OBJECT_TIME);
        let tes: u32 = e.global(TES);
        e.call(0x0045_8e20, &args![tes, marker, time]);
        let scaled = color + 0x10;
        let end = scaled + NI_POINT3_SIZE;
        let scaled = e
            .call(
                0x0045_bb20,
                &args![data.byte_add(0x10), scaled, normal_length],
            )
            .u32();
        let end = e
            .call(0x0043_9e90, &args![data.byte_add(4), end, scaled])
            .u32();
        let line = e
            .call(
                0x004b_3890,
                &args![data.byte_add(4), color, end, color, 1u32],
            )
            .u32();
        let time = setting_float(e, SETTING_DEBUG_OBJECT_TIME);
        let tes: u32 = e.global(TES);
        e.call(0x0045_8e20, &args![tes, line, time]);
    });
}

// Translated from 009bc8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With the VATS targeting debug setting, an empty impact list and a VATS
/// target (`008d8b80`), logs a missed VATS shot (target and part). Then
/// calls `007f6a20` for a reference whose `00401170` is 0x3E, and turns the
/// projectile off: virtual +0xC4 with 1 when `00469860` on `011c3f2c` knows
/// its form id, `TESObjectREFR::Disable` (`00574400`) otherwise.
pub fn fn_009bc8f0(e: &mut Engine, this: Ptr<Projectile>) {
    const ERROR: u32 = 0x0040_fbe0;
    if bool_setting(e, SETTING_DEBUG_VATS_TARGETING)
        && e.call(0x0082_56d0, &args![this.byte_add(IMPACT_LIST)])
            .bool()
        && e.call(0x008d_8b80, &args![this]).u32() != 0
    {
        let vats_target = e.call(0x008d_8b80, &args![this]).u32();
        let node = e.call(0x0055_9450, &args![vats_target + 4]).u32();
        let first = e.call(0x004b_5260, &args![node]).u32();
        let target_part = if first != 0 {
            e.call(0x0044_ddc0, &args![first]).u32()
        } else {
            0
        };
        let vats_target = e.call(0x008d_8b80, &args![this]).u32();
        let target = e.mem.u32(vats_target);
        e.call(ERROR, &args![0x0109_0434u32, 0x0109_045cu32]);
        let name = debug_reference_name(e, target);
        let form_id = e.call(GET_FORM_ID, &args![target]).u32();
        e.call(ERROR, &args![0x0109_041cu32, name, form_id]);
        let part = debug_part_name(e, target_part);
        e.call(ERROR, &args![0x0109_040cu32, part]);
    }
    if e.call(0x0040_1170, &args![this]).u32() == 0x3E {
        e.call(0x007f_6a20, &args![this]);
    }
    let form_id = e.call(GET_FORM_ID, &args![this]).u32();
    let game: u32 = e.global(0x011c_3f2c);
    if e.call(0x0046_9860, &args![game, form_id]).bool() {
        e.vcall(this.addr(), 0xC4, &args![1u32]);
    } else {
        e.call(0x0057_4400, &args![this]);
    }
}

/// Allocates `size` bytes and runs the projectile subclass `constructor` on
/// them with the eight launch words; null when the allocation fails.
fn make_projectile(e: &mut Engine, size: u32, constructor: u32, launch: &[u32; 8]) -> u32 {
    let block = allocate(e, size);
    if block == 0 {
        return 0;
    }
    let mut words = vec![block];
    words.extend_from_slice(launch);
    e.call(constructor, &words).u32()
}

/// `00966a00(projectile, value)` with the product rounded to `float`.
fn set_speed(e: &mut Engine, projectile: u32, value: f64) {
    e.call(0x0096_6a00, &args![projectile, value as f32]);
}

// Translation incomplete: 009bca60 - C++ exception unwinding (FS:[0] frame) is not modelled.
// Translated from 009bca60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Launches a projectile (cdecl): creates the subclass the base object's
/// type (`004fd4f0`) names (0x10000: `009b7af0`, 0x160 bytes; 0x20000:
/// `009b4020`, 0x154; 0x40000: `00979070`, 0x154; 0x100000: `009ab090`,
/// 0x158; 0x80000: reuses the shooter's existing one, `009bd780`, restarting
/// it with `009b3400`, else `009b31f0`, 0x158), each with base, shooter,
/// weapon, position and angles.
///
/// The player's weapon scales the speed (`00453700` then `00966a00`) by its mod
/// effect 9 value when the item has it and, for weapon type 0xD
/// (`00446390`), by `1 + setting(011cf9c8) * (actor value 5 - 00663b60)`.
/// An actor shooter scales it by entry point 0x3B
/// (`BGSEntryPoint::HandleEntryPoint`) and by `1 / 00716440()` with a
/// positive actor value 0x33. The spreads, flag 0x20000 and, for a new
/// projectile, the actor cause, pick cache, the player's default object 6
/// (`00567ad0`) and `0040ebd0(1, ..)` of `009bd540` for a weapon with
/// `004c0bf0`, the owner's `0097f720`, the desired target, flag 0x1000,
/// virtual +0x228 and virtual +0x308 are set. A base object with a sound
/// (`004fd3e0`) gets its launch sound unless it has flag 0x80, or
/// `00975300` holds and `0045cd80` is not at most 0; with flag 1 the sound
/// plays (`bSoundFired`). With the debug byte at `011f20a4` a marker and a
/// direction line are drawn. The projectile is handed to the manager at
/// `011f1958` (`009a52f0`).
#[allow(clippy::too_many_arguments)]
pub fn fn_009bca60(
    e: &mut Engine,
    base_object: Ptr,
    shooter: u32,
    owner: u32,
    weapon: u32,
    x: u32,
    y: u32,
    z: u32,
    angle_z: f32,
    angle_x: f32,
    pick_cache: u32,
    desired_target: u32,
    flag_1000: u8,
    flag_20000: u8,
    spread_heading: f32,
    spread_looking: f32,
    slot_228_value: u32,
) -> Ptr<Projectile> {
    let mut projectile = 0u32;
    let mut fresh = true;
    let launch = [
        base_object.addr(),
        shooter,
        weapon,
        x,
        y,
        z,
        x87_quiet(angle_z).to_bits(),
        x87_quiet(angle_x).to_bits(),
    ];
    if !base_object.is_null() {
        match e.call(0x004f_d4f0, &args![base_object]).i32() {
            0x10000 => projectile = make_projectile(e, 0x160, 0x009b_7af0, &launch),
            0x20000 => projectile = make_projectile(e, 0x154, 0x009b_4020, &launch),
            0x40000 => projectile = make_projectile(e, 0x154, 0x0097_9070, &launch),
            0x100000 => projectile = make_projectile(e, 0x158, 0x009a_b090, &launch),
            0x80000 => {
                projectile = fn_009bd780(e, shooter, base_object.addr()).addr();
                if projectile != 0 {
                    e.call(0x009b_3400, &args![projectile]);
                    fresh = false;
                } else {
                    projectile = make_projectile(e, 0x158, 0x009b_31f0, &launch);
                }
            }
            _ => {}
        }
    }

    let player: u32 = e.global(PLAYER);
    if shooter == player && weapon != 0 {
        let process = e.call(ACTOR_PROCESS_OBJECT, &args![player]).u32();
        let item = e.vcall(process, 0x148, &args![]).u32();
        if item != 0 && e.call(0x004b_da70, &args![item, 9u32]).bool() {
            let speed = e.call(0x0045_3700, &args![projectile]).f64();
            let value = e.call(0x004b_cf60, &args![weapon, 9u32, 0u32]).f64();
            set_speed(e, projectile, value * speed);
        }
        if e.call(0x0044_6390, &args![weapon]).u32() == 0xD {
            let required = e.call(0x0066_3b60, &args![weapon]).i32();
            let player: u32 = e.global(PLAYER);
            let skill = e.vcall(player + 0xA4, 8, &args![5u32]).i32();
            let difference = skill.wrapping_sub(required) as f64;
            let per_point = setting_float(e, 0x011c_f9c8);
            let factor = (per_point as f64 * difference + 1.0) as f32;
            let speed = e.call(0x0045_3700, &args![projectile]).f64();
            set_speed(e, projectile, speed * factor as f64);
        }
    }

    if projectile != 0 && shooter != 0 && is_actor(e, shooter) {
        let factor = e.with_stack(4, |e, value| {
            e.mem.set_f32(value.addr(), 1.0);
            e.call(0x005e_58f0, &args![0x3Bu32, shooter, weapon, value]);
            e.mem.f32(value.addr())
        });
        if factor as f64 != 1.0 {
            let speed = e.call(0x0045_3700, &args![projectile]).f64();
            set_speed(e, projectile, speed * factor as f64);
        }
        if e.vcall(shooter + 0xA4, 8, &args![0x33u32]).i32() > 0 {
            let speed = e.call(0x0045_3700, &args![projectile]).f64();
            let divisor = e.call(0x0071_6440, &args![]).f64();
            set_speed(e, projectile, (1.0 / divisor) * speed);
        }
    }
    let this: Ptr<Projectile> = Ptr::new(projectile);
    if projectile != 0 {
        fn_009bd580(e, this, x87_quiet(spread_heading));
        fn_009bd5a0(e, this, x87_quiet(spread_looking));
    }
    if projectile != 0 {
        e.call(SET_FLAGS, &args![this, 0x20000u32, flag_20000 as u32]);
    }
    if projectile != 0 && fresh {
        if shooter != 0 && is_actor(e, shooter) {
            let block = allocate(e, 0x1C);
            let cause = if block != 0 {
                e.call(0x0066_e010, &args![block, shooter]).u32()
            } else {
                0
            };
            e.vcall(projectile, 0x164, &args![cause]);
        }
        fn_009bd560(e, this, pick_cache);
        let player: u32 = e.global(PLAYER);
        if shooter == player {
            let object = e.call(0x0058_db10, &args![6u32]).u32();
            if object != 0 {
                e.call(0x0056_7ad0, &args![this, object]);
            }
            if e.call(0x004c_0bf0, &args![weapon]).bool() {
                let offset = e.call(SETTING_VALUE, &args![0x011c_f32cu32]).u32();
                let scale = e.call(SETTING_VALUE, &args![0x011c_ee00u32]).u32();
                let player: u32 = e.global(PLAYER);
                let value = fn_009bd540(e, Ptr::new(player));
                let value =
                    (value as f64 * e.mem.f32(scale) as f64 + e.mem.f32(offset) as f64) as f32;
                let clamped = e.call(0x0040_ebd0, &args![1.0f32, value]).f64() as f32;
                e.call(0x0058_6130, &args![this, clamped]);
            }
        }
        if owner != 0 {
            e.call(0x0097_f720, &args![owner, this]);
        }
        if desired_target != 0 {
            e.set(this, Projectile::pDesiredTarget, desired_target);
        }
        if flag_1000 != 0 {
            set_flags(e, this, 0x1000, true);
        }
        if slot_228_value != 0 {
            e.vcall(projectile, 0x228, &args![slot_228_value]);
        }
        e.vcall(projectile, 0x308, &args![]);
    }

    if projectile != 0
        && !base_object.is_null()
        && e.call(0x004f_d3e0, &args![base_object]).u32() != 0
    {
        let silent = fn_009bc870(e, base_object)
            || (e.call(0x0097_5300, &args![base_object]).bool()
                && !(e.call(0x0045_cd80, &args![base_object]).f64() <= 0.0));
        if !silent {
            start_launch_sound(e, this, base_object, shooter);
        }
        if test_flags(e, this, 1) {
            e.call(0x00ad_8830, &args![this.byte_add(SOUND_HANDLE), 1u32]);
            e.set(this, Projectile::bSoundFired, true);
        }
    } else if projectile != 0 && test_flags(e, this, 1) {
        e.set(this, Projectile::bSoundFired, true);
    }

    if e.global::<u8>(0x011f_20a4) != 0
        && projectile != 0
        && e.vcall(projectile, 0x1D0, &args![]).u32() != 0
    {
        draw_launch_debug(e, this);
    }
    if projectile != 0 {
        let manager: u32 = e.global(0x011f_1958);
        let manager = e.call(0x0082_5c00, &args![manager]).u32();
        e.call(0x009a_52f0, &args![manager, this]);
    }
    this
}

/// The launch sound of `009bca60`: the base object's sound (`004fd3e0`)
/// through `BSAudio::GetSoundHandleByFilename` (`00ad7480` on the
/// instance `00453a70` returns, flags 0x40000012) into a temporary handle,
/// assigned to `sndHandle` (`00418900`), positioned at the shooter
/// (`0068a7d0`), following the 3D (`BSSoundHandle::SetObjectToFollow`),
/// with `fSoundDist` 100 times the sound's `00553b90` byte.
fn start_launch_sound(e: &mut Engine, this: Ptr<Projectile>, base_object: Ptr, shooter: u32) {
    let sound = e.call(0x004f_d3e0, &args![base_object]).u32();
    let sound_again = e.call(0x004f_d3e0, &args![base_object]).u32();
    let file = e.call(0x0051_1840, &args![sound_again]).u32();
    e.with_stack(0xC, |e, temporary| {
        let audio = e.call(0x0045_3a70, &args![]).u32();
        let handle = e
            .call(
                0x00ad_7480,
                &args![audio, temporary, file, 0x4000_0012u32, sound],
            )
            .u32();
        e.call(0x0041_8900, &args![this.byte_add(SOUND_HANDLE), handle]);
        e.call(0x0048_3710, &args![temporary]);
    });
    let position = e.vcall(shooter, 0x1F4, &args![]).u32();
    e.call(0x0068_a7d0, &args![this.byte_add(SOUND_HANDLE), position]);
    let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
    e.call(0x00ad_8f20, &args![this.byte_add(SOUND_HANDLE), root]);
    let sound = e.call(0x004f_d3e0, &args![base_object]).u32();
    let range = e.call(0x0055_3b90, &args![sound]).u8();
    e.set(this, Projectile::fSoundDist, (range as i32 * 100) as f32);
}

/// The launch debug drawing of `009bca60`: a red marker at the 3D's
/// translation (`0045bb80`) and a yellow line from it along the rotation
/// `004a0c90` builds from virtual +0x2BC, length `01013974`.
fn draw_launch_debug(e: &mut Engine, this: Ptr<Projectile>) {
    let alpha = float_global(e, 0x0101_8180);
    let marker_size = float_global(e, 0x0101_e570);
    let length = float_global(e, 0x0101_3974);
    e.with_stack(0x10 + 0x24 + 3 * NI_POINT3_SIZE, |e, locals| {
        let color = locals.addr();
        let rotation = color + 0x10;
        let forward = rotation + 0x24;
        let direction = forward + NI_POINT3_SIZE;
        let end = direction + NI_POINT3_SIZE;
        e.call(0x0041_4430, &args![color, 1.0f32, 0.0f32, 0.0f32, alpha]);
        let marker = e.call(0x004b_29b0, &args![marker_size, color, 1u32]).u32();
        e.call(0x00a5_a040, &args![marker]);
        let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
        let position = e.call(0x0045_bb80, &args![root]).u32();
        e.call(0x0044_0460, &args![marker, position]);
        let time = setting_float(e, SETTING_DEBUG_OBJECT_TIME);
        let tes: u32 = e.global(TES);
        e.call(0x0045_8e20, &args![tes, marker, time]);
        e.call(0x0068_15c0, &args![rotation]);
        let heading = e.vcall(this.addr(), 0x2BC, &args![0u32]).f64() as f32;
        e.call(0x004a_0c90, &args![rotation, heading]);
        debug_color(e, color, [1.0, 1.0, 0.0, alpha]);
        let forward = e
            .call(0x0041_6870, &args![forward, 0.0f32, length, 0.0f32])
            .u32();
        let direction = e
            .call(0x004b_4500, &args![rotation, direction, forward])
            .u32();
        let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
        let position = e.call(0x0045_bb80, &args![root]).u32();
        let end = e.call(0x0043_9e90, &args![position, end, direction]).u32();
        let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
        let position = e.call(0x0045_bb80, &args![root]).u32();
        let line = e
            .call(0x004b_3890, &args![position, color, end, color, 1u32])
            .u32();
        let time = setting_float(e, SETTING_DEBUG_OBJECT_TIME);
        let tes: u32 = e.global(TES);
        e.call(0x0045_8e20, &args![tes, line, time]);
    });
}
// Translation incomplete: 009bda10 - C++ exception unwinding (FS:[0] frame) is not modelled.
// Translated from 009bda10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts a projectile in the world. It is "loaded" (no aiming, firing or
/// alarm) when `0042ce10` on `011ddf38` holds, when `00576d30` holds or with
/// flag 0x10000. A projectile without a weapon takes its base object's
/// (`004fd400`). Sets the destructible flag when the base is destructible
/// (`004753d0` then `004846a0(1)`), `00484ab0(1)` and flag 0x10000; a fresh
/// one with a shooter takes the shooter's `008d6f30` through virtual
/// +0x228. Without a process it gets a `HighProcess` (0x46C bytes,
/// `008d7510`). Without flag 2 and with a process level (`0045cd60`) of at
/// most 1 it loads its 3D (`Projectile::Init3D` when none, virtual +0x1C4)
/// and sets its fade (`00476ab0` on the 3D's virtual +0x10 object, or
/// `00b6bb30(.., 1)` on its virtual +0xC object). It is added to the
/// shooter's cell (`TESObjectCELL::AddReference`), `009c3d90` runs, a fresh
/// one runs `009c2ff0`, a new process is registered with `ProcessLists`
/// and flag 0x80 adds it to the list `004610f0` returns. The range is the
/// base's `009a7c40`, scaled by the setting at `011ce5f4` for an actor
/// shooter in iron sights (virtual +0x360, `008bbc10`) whose weapon has
/// `004ad010`. With flag 0x100 a fresh one checks for a wall in front
/// (`Projectile::CheckShotBehindWall`); unless `009bdf80` the explosion
/// timer is the base's `00644790`. Unless blocked or loaded it runs
/// `009c0090`, aims (player shooter), `009beb00` and with flag 1
/// `009bec90`. Types 0x40000 and 0x100000 run `009abe40` with the 3D;
/// then virtual +0x30C, the 3D's virtual +0xA8 with a `0043d410` local of
/// the `float` at `011c3c08`, and the pick cache is released.
pub fn fn_009bda10(e: &mut Engine, this: Ptr<Projectile>) {
    let game: u32 = e.global(0x011d_df38);
    let loaded = e.call(0x0042_ce10, &args![game]).bool()
        || e.call(0x0057_6d30, &args![this]).bool()
        || test_flags(e, this, 0x10000);
    if e.call(GET_WEAPON, &args![this]).u32() == 0 {
        let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
        let weapon = e.call(0x004f_d400, &args![base]).u32();
        fn_009bdf60(e, this, weapon);
    }
    let base = e.call(0x007a_f430, &args![this]).u32();
    if e.call(0x0047_53d0, &args![base]).bool() {
        e.call(0x0048_46a0, &args![this, 1u32]);
    }
    e.call(0x0048_4ab0, &args![this, 1u32]);
    set_flags(e, this, 0x10000, true);
    if e.call(GET_SHOOTER, &args![this]).u32() != 0 && !loaded {
        let shooter = e.call(GET_SHOOTER, &args![this]).u32();
        let value = e.call(0x008d_6f30, &args![shooter]).u32();
        e.vcall(this.addr(), 0x228, &args![value]);
    }
    e.set(this, Projectile::bSendAlarm, false);
    let new_process = e.get(this, Projectile::pCurrentProcess) == 0;
    if new_process {
        let block = allocate(e, 0x46C);
        let process = if block != 0 {
            e.call(0x008d_7510, &args![block]).u32()
        } else {
            0
        };
        e.set(this, Projectile::pCurrentProcess, process);
    }
    if !test_flags(e, this, 2) && e.get(this, Projectile::pCurrentProcess) != 0 {
        let process = e.get(this, Projectile::pCurrentProcess);
        if e.call(0x0045_cd60, &args![process]).i32() <= 1 {
            if e.vcall(this.addr(), 0x1D0, &args![]).u32() == 0 {
                projectile_init_3d(e, this);
            }
            e.vcall(this.addr(), 0x1C4, &args![]);
            if e.vcall(this.addr(), 0x1D0, &args![]).u32() != 0 {
                let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
                if e.vcall(root, 0x10, &args![]).u32() != 0 {
                    let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
                    let object = e.vcall(root, 0x10, &args![]).u32();
                    e.call(0x0047_6ab0, &args![object]);
                } else {
                    let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
                    let object = e.vcall(root, 0xC, &args![]).u32();
                    e.call(0x00b6_bb30, &args![object, 1.0f32]);
                }
            }
        }
    }
    if e.call(0x008d_6f30, &args![this]).u32() != 0 {
        let cell = e.call(0x008d_6f30, &args![this]).u32();
        e.call(0x0054_8230, &args![cell, this, 0u32]);
    }
    e.call(0x009c_3d90, &args![this]);
    if !loaded {
        fn_009c2ff0(e, this);
    }
    if new_process {
        e.call(
            0x0096_d450,
            &args![PROCESS_LISTS, this, 0u32, 0u32, 0u32, 0u32],
        );
    }
    if test_flags(e, this, 0x80) {
        let list = e.call(0x0046_10f0, &args![PROCESS_LISTS]).u32();
        if list != 0 {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), this.addr());
                e.call(0x005a_e3d0, &args![list, slot]);
            });
        }
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let range = e.call(0x009a_7c40, &args![base]).f64() as f32;
    e.set(this, Projectile::fRange, range);
    let shooter = e.get(this, Projectile::pShooter);
    if !loaded && shooter != 0 && is_actor(e, shooter) {
        let actor = e.get(this, Projectile::pShooter);
        if e.vcall(actor, 0x360, &args![]).bool()
            && e.call(0x008b_bc10, &args![actor]).bool()
            && e.get(this, Projectile::pWeaponSource) != 0
        {
            let weapon = e.get(this, Projectile::pWeaponSource);
            if e.call(0x004a_d010, &args![weapon]).bool() {
                let scale = e.call(SETTING_VALUE, &args![0x011c_e5f4u32]).u32();
                let range = e.get(this, Projectile::fRange);
                e.set(
                    this,
                    Projectile::fRange,
                    (range as f64 * e.mem.f32(scale) as f64) as f32,
                );
            }
        }
    }
    let mut blocked = false;
    if !loaded && test_flags(e, this, 0x100) {
        blocked = e.call(0x009c_40c0, &args![this]).bool();
    }
    if !fn_009bdf80(e, this) {
        let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
        let timer = e.call(0x0064_4790, &args![base]).f64() as f32;
        e.set(this, Projectile::fExplosionTimer, timer);
    }
    if !blocked && !loaded {
        fn_009c0090(e, this);
        let player: u32 = e.global(PLAYER);
        if e.call(GET_SHOOTER, &args![this]).u32() == player {
            projectile_aim_projectile(e, this);
        }
        fn_009beb00(e, this);
        if test_flags(e, this, 1) {
            fn_009bec90(e, this);
        }
    }
    let base = e.call(0x007a_f430, &args![this]).u32();
    if e.call(0x004f_d4f0, &args![base]).i32() == 0x40000
        || e.call(0x004f_d4f0, &args![base]).i32() == 0x100000
    {
        let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
        e.call(0x009a_be40, &args![this, root]);
    }
    e.vcall(this.addr(), 0x30C, &args![]);
    if e.vcall(this.addr(), 0x1D0, &args![]).u32() != 0 {
        let value = float_global(e, 0x011c_3c08);
        e.with_stack(NI_POINT3_SIZE, |e, local| {
            e.call(0x0043_d410, &args![local, value, 1u32, 0u32]);
            let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
            e.vcall(root, 0xA8, &args![local, 0u32]);
        });
    }
    e.call(ASSIGN_NI_POINTER, &args![this.byte_add(PICK_CACHE), 0u32]);
}

// Translated from 009bdf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `pWeaponSource`.
pub fn fn_009bdf60(e: &mut Engine, this: Ptr<Projectile>, weapon: u32) {
    e.set(this, Projectile::pWeaponSource, weapon);
}

// Translated from 009bdf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the base object's `005de080` holds, or `00975300` holds and its
/// `0045cd80` is above 0 (ordered).
pub fn fn_009bdf80(e: &mut Engine, this: Ptr<Projectile>) -> bool {
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if e.call(0x005d_e080, &args![base]).bool() {
        return true;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if !e.call(0x0097_5300, &args![base]).bool() {
        return false;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    e.call(0x0045_cd80, &args![base]).f64() > 0.0
}
// Translation incomplete: 009be0a0 - C++ exception unwinding (FS:[0] frame) and the /GS cookie check (00ec408c) are not modelled.
// Translated from 009be0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts the projectile's 3D into the Havok world of its cell (`008d6f30`,
/// `004543c0`), for a projectile with 3D and a process at level 0
/// (`0045cd60`).
///
/// Flag 8: keyframes it (`00c6a0b0(root, 0, 1, 0)`, `bhkWorld::SetMotion(root,
/// 4, 1, 0, 1)`). Flag 4 (a physical projectile): dynamic motion
/// (`SetMotion(.., 1, 1, 0, 1)`), the settings at `011ce308` (scaled by the
/// base's `006d2c20` when positive, `00c6a120`) and `011ce5e8` (`00c6a190`)
/// when nonzero, `00c6a4c0(root, 6, 0, 1, 0)`, the shooter's collision
/// filter (`00c6a3d0`, from `00931ed0` for an actor or the first collision
/// object of its 3D through `0043b4f0`), added to the world (virtual +0xD0)
/// and, when the base's `009b88e0` holds, an angular velocity from the
/// base's three `float`s transformed by the 3D's rotation
/// (`NiMatrix3::TransformVertices`, `004b4d50`,
/// `hkpRigidBody::setAngularVelocity`) on the rigid body found on the 3D
/// (`004a8b00`, or its first child that has one).
///
/// Otherwise without a character controller (`MobileObject::
/// GetCharController`) one is built (0x660 bytes, `009c6440`) from a
/// construction-info local (`00c6da50` .. `00c6de10`): world, scale 1, the
/// 3D, position, a sphere shape (`bhkSphereShape`, radius 1 or the `float`
/// at `0101e2bc`), speed (`00966980`, scaled for actor value 0x33), the
/// shooter's collision filter, and handed to the process (virtual +0x290
/// with a `NiPointer` argument). The controller then gets the water height
/// (`TESObjectCELL::GetWaterHeight`, or the `float` at `01015f5c`), the
/// world, the 3D (with wireframe when `004bae00`), and its position (raised
/// by the `double` at `010240b0` when it changes worlds), and is added to
/// the world (virtual +0xD0 with the `0070c440` filter).
pub fn fn_009be0a0(e: &mut Engine, this: Ptr<Projectile>) {
    let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
    if root == 0 || e.get(this, Projectile::pCurrentProcess) == 0 {
        return;
    }
    let process = e.get(this, Projectile::pCurrentProcess);
    if e.call(0x0045_cd60, &args![process]).u32() != 0 {
        return;
    }
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let world = if cell != 0 {
        e.call(0x0045_43c0, &args![cell]).u32()
    } else {
        0
    };
    if world == 0 {
        return;
    }
    if test_flags(e, this, 8) {
        e.call(0x00c6_a0b0, &args![root, 0u32, 1u32, 0u32]);
        e.call(0x00c6_a350, &args![root, 4u32, 1u32, 0u32, 1u32]);
        return;
    }
    let body = e.call(0x004a_8b00, &args![root]).u32();
    if test_flags(e, this, 4) {
        add_physical_projectile(e, this, root, world, body);
    } else {
        add_character_controller(e, this, root, world, cell);
    }
}

/// The flag-4 branch of `009be0a0`.
fn add_physical_projectile(
    e: &mut Engine,
    this: Ptr<Projectile>,
    root: u32,
    world: u32,
    body: u32,
) {
    let mut body = body;
    let mut rigid_body = 0u32;
    let object = e.vcall(this.addr(), 0x1D0, &args![]).u32();
    e.call(0x00c6_a350, &args![object, 1u32, 1u32, 0u32, 1u32]);
    let setting = e.call(SETTING_VALUE, &args![0x011c_e308u32]).u32();
    if e.mem.f32(setting) as f64 != 0.0 {
        let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
        let factor = e.call(0x006d_2c20, &args![base]).f64();
        if factor > 0.0 {
            let setting = e.call(SETTING_VALUE, &args![0x011c_e308u32]).u32();
            let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
            let factor = e.call(0x006d_2c20, &args![base]).f64();
            let value = (factor * e.mem.f32(setting) as f64) as f32;
            e.call(0x00c6_a120, &args![root, value, 1u32, 0u32]);
        } else {
            let value = setting_float(e, 0x011c_e308);
            e.call(0x00c6_a120, &args![root, value, 1u32, 0u32]);
        }
    }
    let setting = e.call(SETTING_VALUE, &args![0x011c_e5e8u32]).u32();
    if e.mem.f32(setting) as f64 != 0.0 {
        let value = setting_float(e, 0x011c_e5e8);
        e.call(0x00c6_a190, &args![root, value, 1u32, 0u32]);
    }
    e.call(0x00c6_a4c0, &args![root, 6u32, 0u32, 1u32, 0u32]);
    if e.call(GET_SHOOTER, &args![this]).u32() != 0 {
        let shooter = e.call(GET_SHOOTER, &args![this]).u32();
        if is_actor(e, shooter) {
            let actor = e.call(GET_SHOOTER, &args![this]).u32();
            let filter = e.with_stack(4, |e, local| {
                let info = e.call(0x0093_1ed0, &args![actor, local]).u32();
                e.call(0x004a_3a20, &args![info]).u32()
            });
            e.call(0x00c6_a3d0, &args![root, filter, 1u32, 0u32]);
        } else {
            let shooter = e.call(GET_SHOOTER, &args![this]).u32();
            let shooter_root = e.vcall(shooter, 0x1D0, &args![]).u32();
            let first = e.call(0x004b_5260, &args![shooter_root]).u32();
            let collision = if first != 0 {
                e.call(0x006f_a820, &args![first]).u32()
            } else {
                0
            };
            if collision != 0 {
                let filter = e.with_stack(4, |e, local| {
                    let info = e.call(0x0043_b4f0, &args![collision, local]).u32();
                    e.call(0x004a_3a20, &args![info]).u32()
                });
                e.call(0x00c6_a3d0, &args![root, filter, 1u32, 0u32]);
            }
        }
    }
    e.vcall(world, 0xD0, &args![root, 1u32, 1u32, 0u32, 0u32]);
    if body != 0 {
        let collision = e.call(0x006f_a820, &args![body]).u32();
        let entity = e.call(0x0062_0b80, &args![collision]).u32();
        rigid_body = e.call(0x004b_5a80, &args![entity]).u32();
    } else if e.vcall(root, 0x10, &args![]).u32() != 0 {
        let node = root;
        if e.call(0x0045_3470, &args![node]).u32() != 0 {
            let mut index = 0u32;
            while index < e.call(0x0045_3470, &args![node]).u32() {
                let child = e.call(0x0043_b4a0, &args![node, index]).u32();
                body = e.call(0x004a_8b00, &args![child]).u32();
                if body != 0 {
                    let collision = e.call(0x006f_a820, &args![body]).u32();
                    let entity = e.call(0x0062_0b80, &args![collision]).u32();
                    rigid_body = e.call(0x004b_5a80, &args![entity]).u32();
                    break;
                }
                index += 1;
            }
        }
    }
    if rigid_body == 0 {
        return;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if !e.call(0x009b_88e0, &args![base]).bool() {
        return;
    }
    let havok_body = e.call(0x0062_0b80, &args![rigid_body]).u32();
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let z = e.call(0x009b_88c0, &args![base]).f64() as f32;
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let y = e.call(0x009b_88a0, &args![base]).f64() as f32;
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let x = e.call(0x0081_2870, &args![base]).f64() as f32;
    e.with_stack(
        3 * NI_POINT3_SIZE + 0x10 + 0x24 + NI_POINT3_SIZE,
        |e, locals| {
            let spin = locals.addr();
            let rotated = spin + NI_POINT3_SIZE;
            let velocity = rotated + NI_POINT3_SIZE;
            let matrix = velocity + 0x10;
            let zero = matrix + 0x24;
            e.call(0x0041_6870, &args![spin, x, y, z]);
            e.call(0x0068_15c0, &args![rotated]);
            e.call(0x0068_15c0, &args![velocity]);
            let object = e.vcall(this.addr(), 0x1D0, &args![]).u32();
            let rotation = e.call(0x0046_1130, &args![object]).u32();
            for offset in (0..0x24).step_by(4) {
                let word = e.mem.u32(rotation + offset);
                e.mem.set_u32(matrix + offset, word);
            }
            let zero = e
                .call(0x0041_6870, &args![zero, 0.0f32, 0.0f32, 0.0f32])
                .u32();
            e.call(0x00a5_82f0, &args![matrix, zero, 1u32, spin, rotated]);
            let rx = x87_quiet(e.mem.f32(rotated));
            let ry = x87_quiet(e.mem.f32(rotated + 4));
            let rz = x87_quiet(e.mem.f32(rotated + 8));
            e.call(0x004b_4d50, &args![velocity, rx, ry, rz, 0.0f32]);
            e.call(0x0056_1800, &args![havok_body, velocity]);
        },
    );
}

/// Size of the character-controller construction info `009be0a0` keeps on
/// its stack (from `EBP-0x190` to the controller pointer at `EBP-0xC4`).
const CONTROLLER_INFO_SIZE: u32 = 0xCC;

/// The character-controller branch of `009be0a0`.
fn add_character_controller(
    e: &mut Engine,
    this: Ptr<Projectile>,
    root: u32,
    world: u32,
    cell: u32,
) {
    let mut controller = e.call(0x0093_06d0, &args![this]).u32();
    if controller == 0 {
        controller = build_character_controller(e, this, root, world);
    }
    if controller == 0 {
        return;
    }
    if cell != 0 {
        let height = e.call(0x0054_71e0, &args![cell]).f64() as f32;
        e.call(0x0057_b1d0, &args![controller, height]);
    } else {
        let height = float_global(e, 0x0101_5f5c);
        e.call(0x0057_b1d0, &args![controller, height]);
    }
    let old_world = e.call(0x0062_1ad0, &args![controller]).u32();
    e.call(0x00c7_0c90, &args![controller, world]);
    let object = e.call(0x0081_0660, &args![controller]).u32();
    let mut reposition = true;
    if object != root {
        e.call(0x00c6_d7a0, &args![controller, root]);
        if world != 0 && e.call(0x004b_ae00, &args![world]).bool() {
            e.vcall(controller, 0xD0, &args![0u32]);
            let object = e.call(0x0081_0660, &args![controller]).u32();
            e.call(0x00b6_b910, &args![object, 1u32]);
        }
        e.call(0x00c6_a350, &args![root, 4u32, 1u32, 0u32, 1u32]);
        reposition = true;
    }
    let position = e.vcall(this.addr(), 0x1F4, &args![]).u32();
    let [x, y, z] = point_words(e, position);
    let mut z = f32::from_bits(z);
    if old_world != world {
        if old_world != 0 {
            e.call(0x00c6_9ee0, &args![root, 1u32, 0u32]);
        }
        let lift: f64 = e.global(0x0102_40b0);
        z = (z as f64 + lift) as f32;
        reposition = true;
    }
    if world == 0 {
        return;
    }
    e.with_stack(NI_POINT3_SIZE + 4, |e, locals| {
        let point = locals.addr();
        let filter_info = point + NI_POINT3_SIZE;
        e.mem.set_u32(point, x);
        e.mem.set_u32(point + 4, y);
        e.mem.set_f32(point + 8, z);
        e.call(0x0056_20e0, &args![controller, point]);
        if reposition {
            e.call(0x0070_c440, &args![controller, filter_info]);
            let filter = e.call(0x004a_3a20, &args![filter_info]).u32();
            e.vcall(world, 0xD0, &args![root, 1u32, 0u32, filter, 0u32]);
        } else {
            e.call(0x00c6_9f50, &args![root, 1u32, 0u32]);
        }
    });
}

/// Builds the projectile's character controller (the part of `009be0a0`
/// between the construction-info constructor and destructor); returns it.
fn build_character_controller(e: &mut Engine, this: Ptr<Projectile>, root: u32, world: u32) -> u32 {
    e.with_stack(CONTROLLER_INFO_SIZE + 0x10 + 4, |e, locals| {
        let info = locals.addr();
        let box_local = info + CONTROLLER_INFO_SIZE;
        let filter_local = box_local + 0x10;
        e.call(0x00c6_da50, &args![info, 1u32]);
        e.call(0x0085_7230, &args![info, world]);
        e.call(0x0081_0550, &args![info, 1.0f32]);
        e.call(0x008a_c890, &args![info, root]);
        let position = e.vcall(this.addr(), 0x1F4, &args![]).u32();
        e.call(0x0081_0510, &args![info, position]);
        let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
        let radius = if e.call(0x009b_88e0, &args![base]).bool() {
            1.0
        } else {
            float_global(e, 0x0101_e2bc)
        };
        let block = e.call(0x00aa_13e0, &args![0x14u32]).u32();
        let shape = if block != 0 {
            e.call(0x0056_e950, &args![block, radius, 1u32]).u32()
        } else {
            0
        };
        e.call(0x0081_05c0, &args![info, shape]);
        fn_009beae0(e, Ptr::new(info), 0);
        e.call(0x0081_0530, &args![info, 0u32]);
        let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
        let mut speed = e.call(0x0096_6980, &args![base]).f64() as f32;
        let shooter = e.get(this, Projectile::pShooter);
        if shooter != 0 && is_actor(e, shooter) {
            let actor = e.get(this, Projectile::pShooter);
            let filter_info = e.call(0x0093_1ed0, &args![actor, filter_local]).u32();
            let filter = e.call(0x004a_3a20, &args![filter_info]).u32();
            e.call(0x0081_0590, &args![info, 6u32, filter]);
            if e.vcall(actor + 0xA4, 8, &args![0x33u32]).i32() > 0 {
                let divisor = e.call(0x0071_6440, &args![]).f64();
                speed = ((1.0 / divisor) * speed as f64) as f32;
            }
        }
        e.call(0x0040_7860, &args![info, x87_quiet(speed)]);
        e.call(0x0081_0570, &args![info, 6u32]);
        let block = e.call(0x0055_3300, &args![0x660u32]).u32();
        let controller = if block != 0 {
            e.call(0x009c_6440, &args![block, info, this]).u32()
        } else {
            0
        };
        let layer = e.call(0x0052_5420, &args![]).u8();
        e.call(0x0081_0610, &args![controller, layer as u32]);
        e.call(0x0081_05e0, &args![controller, 1u32]);
        let extent = -float_global(e, 0x0109_0050);
        e.call(
            0x0055_32a0,
            &args![box_local, extent, 0.0f32, 0.0f32, 0.0f32],
        );
        e.call(0x0080_fb00, &args![controller, box_local]);
        let argument = e.with_stack(4, |e, slot| {
            e.call(0x0063_3c90, &args![slot, controller]);
            e.mem.u32(slot.addr())
        });
        let process = e.get(this, Projectile::pCurrentProcess);
        e.vcall(process, 0x290, &args![argument]);
        e.call(0x0081_0640, &args![controller, this]);
        e.call(0x00c6_a0b0, &args![root, 0u32, 1u32, 0u32]);
        e.call(0x00c6_de10, &args![info]);
        controller
    })
}
// Translated from 009bec10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Without flag 0x80, passes `value` to the projectile's virtual +0x2F8.
pub fn fn_009bec10(e: &mut Engine, this: Ptr<Projectile>, value: f32) {
    if !test_flags(e, this, 0x80) {
        e.vcall(this.addr(), 0x2F8, &args![x87_quiet(value)]);
    }
}

// Translated from 009bec50 (decompiled, FalloutNV.exe 1.4.0.525)
/// With flag 0x80, passes `value` to the projectile's virtual +0x2F8;
/// returns false.
pub fn fn_009bec50(e: &mut Engine, this: Ptr<Projectile>, value: f32) -> bool {
    if test_flags(e, this, 0x80) {
        e.vcall(this.addr(), 0x2F8, &args![x87_quiet(value)]);
    }
    false
}

// Translated from 009bec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// With flag 1, runs the projectile's virtual +0x328.
pub fn fn_009bec90(e: &mut Engine, this: Ptr<Projectile>) {
    if test_flags(e, this, 1) {
        e.vcall(this.addr(), 0x328, &args![]);
    }
}

// Translation incomplete: 009becc0 - the /GS cookie check (00ec408c) is not modelled.
// Translated from 009becc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::Update` (Xbox PDB). The frame time (`0084d030` on
/// `011f6394`) is scaled in VATS mode 4 for a projectile without flag 4 by
/// the player's or the VATS target's (`011f21cc`) update multiplier. Flag
/// 0x20 fades the alpha by `time / 006ca4e0(base)` (clamped at 0) and sets
/// it on the 3D (`00b6bb30`). A flag-4 projectile of an actor shooter with a
/// positive actor value 0x33 gets a force on its rigid body
/// (`hkpRigidBody::applyForce`, from a `005532a0` vector with the `float` at
/// `0104f754`, `004a3f10` with the world's `009a6080`, `005dbf20` with the
/// `float` at `010162c0`, `00627920`). Then virtual +0x310 with the time
/// unless the type (virtual +0x304) is 5, `fAge` grows, `009c3e60` runs, the
/// alarm is sent once (`00977770` on `ProcessLists` with an actor shooter),
/// and the sound (unfired, without flag 1) starts once the player is within
/// `fSoundDist` (`00457990`, `00408840`), with a debug marker when the byte
/// at `011f20a4` is set. The stack word is never read.
pub fn projectile_update(e: &mut Engine, this: Ptr<Projectile>, _unused_1: u32) {
    let mut time = e.call(0x0084_d030, &args![0x011f_6394u32]).f64() as f32;
    if e.call(0x0044_ddc0, &args![VATS]).u32() == 4 && !test_flags(e, this, 4) {
        let player: u32 = e.global(PLAYER);
        if e.call(GET_SHOOTER, &args![this]).u32() == player {
            let scale = e.call(0x009c_8cc0, &args![VATS]).f64();
            time = (scale * time as f64) as f32;
        } else {
            let target: u32 = e.global(0x011f_21cc);
            if e.call(GET_SHOOTER, &args![this]).u32() == target {
                let scale = e.call(0x009c_8d60, &args![VATS]).f64();
                time = (scale * time as f64) as f32;
            }
        }
    }
    if test_flags(e, this, 0x20) {
        let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
        let duration = e.call(0x006c_a4e0, &args![base]).f64() as f32;
        let alpha = e.get(this, Projectile::fAlpha);
        let alpha = (alpha as f64 - time as f64 / duration as f64) as f32;
        e.set(this, Projectile::fAlpha, alpha);
        if e.get(this, Projectile::fAlpha) as f64 <= 0.0 {
            e.set(this, Projectile::fAlpha, 0.0);
        }
        if e.vcall(this.addr(), 0x1D0, &args![]).u32() != 0 {
            let alpha = x87_quiet(e.get(this, Projectile::fAlpha));
            let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
            e.call(0x00b6_bb30, &args![root, alpha]);
        }
    }
    if e.call(GET_SHOOTER, &args![this]).u32() != 0 {
        let shooter = e.call(GET_SHOOTER, &args![this]).u32();
        if is_actor(e, shooter) && test_flags(e, this, 4) {
            let actor = e.call(GET_SHOOTER, &args![this]).u32();
            if e.vcall(actor + 0xA4, 8, &args![0x33u32]).i32() > 0
                && e.vcall(this.addr(), 0x1D0, &args![]).u32() != 0
            {
                apply_update_force(e, this, time);
            }
        }
    }
    if e.vcall(this.addr(), 0x304, &args![]).u32() != 5 {
        e.vcall(this.addr(), 0x310, &args![x87_quiet(time)]);
    }
    let age = e.get(this, Projectile::fAge);
    e.set(this, Projectile::fAge, (age as f64 + time as f64) as f32);
    e.call(0x009c_3e60, &args![this]);
    if !e.get(this, Projectile::bSendAlarm) {
        let mut actor = 0u32;
        if e.call(GET_SHOOTER, &args![this]).u32() != 0 {
            let shooter = e.call(GET_SHOOTER, &args![this]).u32();
            if is_actor(e, shooter) {
                actor = e.call(GET_SHOOTER, &args![this]).u32();
            }
        }
        e.call(0x0097_7770, &args![PROCESS_LISTS, actor, this]);
        e.set(this, Projectile::bSendAlarm, true);
    }
    let player: u32 = e.global(PLAYER);
    if e.get(this, Projectile::bSoundFired)
        || test_flags(e, this, 1)
        || e.vcall(this.addr(), 0x1D0, &args![]).u32() == 0
        || player == 0
    {
        return;
    }
    let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
    let position = e.call(0x0045_bb80, &args![root]).u32();
    let distance = e.with_stack(NI_POINT3_SIZE, |e, offset| {
        let player: u32 = e.global(PLAYER);
        let player_root = e.vcall(player, 0x1D0, &args![]).u32();
        let player_position = e.call(0x0045_bb80, &args![player_root]).u32();
        e.call(0x0043_9ef0, &args![player_position, offset, position]);
        let length = e.call(0x0045_7990, &args![offset]).f64() as f32;
        e.call(0x0040_8840, &args![length]).f64()
    });
    let range = e.get(this, Projectile::fSoundDist) as f64;
    if !(range >= distance) {
        return;
    }
    e.call(0x00ad_8830, &args![this.byte_add(SOUND_HANDLE), 1u32]);
    e.set(this, Projectile::bSoundFired, true);
    if e.global::<u8>(0x011f_20a4) == 0 {
        return;
    }
    let alpha = float_global(e, 0x0101_8180);
    let marker_size = float_global(e, 0x0101_712c);
    e.with_stack(0x10, |e, color| {
        e.call(0x0041_4430, &args![color, 0.0f32, 0.0f32, 1.0f32, alpha]);
        let marker = e.call(0x004b_29b0, &args![marker_size, color, 1u32]).u32();
        e.call(0x00a5_a040, &args![marker]);
        let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
        let position = e.call(0x0045_bb80, &args![root]).u32();
        e.call(0x0044_0460, &args![marker, position]);
        let time = setting_float(e, SETTING_DEBUG_OBJECT_TIME);
        let tes: u32 = e.global(TES);
        e.call(0x0045_8e20, &args![tes, marker, time]);
    });
}

/// The force `Projectile::Update` applies to a flag-4 projectile's rigid
/// body.
fn apply_update_force(e: &mut Engine, this: Ptr<Projectile>, time: f32) {
    let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
    let body = e.call(0x004a_8b00, &args![root]).u32();
    let rigid_body = if body != 0 {
        let collision = e.call(0x006f_a820, &args![body]).u32();
        let entity = e.call(0x0062_0b80, &args![collision]).u32();
        e.call(0x004b_5a80, &args![entity]).u32()
    } else {
        0
    };
    if rigid_body == 0 {
        return;
    }
    let havok_body = e.call(0x0062_0b80, &args![rigid_body]).u32();
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let world = e.call(0x0045_43c0, &args![cell]).u32();
    let strength = float_global(e, 0x0104_f754);
    let scale = float_global(e, 0x0101_62c0);
    e.with_stack(0x20, |e, locals| {
        let force = locals.addr();
        let factor = force + 0x10;
        e.call(0x0055_32a0, &args![force, 0.0f32, 0.0f32, strength, 0.0f32]);
        let gravity = e.call(0x009a_6080, &args![world]).u32();
        e.call(0x004a_3f10, &args![force, gravity]);
        e.call(0x005d_bf20, &args![factor, scale]);
        e.call(0x0062_7920, &args![force, factor]);
        e.call(0x0063_1100, &args![havok_body, x87_quiet(time), force]);
    });
}
// Translated from 009bf300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the projectile by `delta` along its local forward axis: the point
/// `(0, 009669c0() * delta, 0)` (`00416870`) passed to `009bf370`.
pub fn fn_009bf300(e: &mut Engine, this: Ptr<Projectile>, delta: f32) {
    let delta = x87_quiet(delta);
    let speed = e.call(0x0096_69c0, &args![this]).f64();
    let distance = (speed * delta as f64) as f32;
    let [x, y, z] = e.with_stack(NI_POINT3_SIZE, |e, point| {
        e.call(0x0041_6870, &args![point, 0.0f32, distance, 0.0f32]);
        point_words(e, point.addr())
    });
    fn_009bf370(e, this, delta, x, y, z);
}

// Translated from 009bf470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::Sync3DWithReference` (Xbox PDB): moves the 3D to the
/// reference's position (`00440460`); with `rotate`, sets its rotation from
/// the reference's angles (`00430830`, `00a59660` with z, x, y,
/// `0043fa80`); with `align` and flag 0x40, re-tilts the 3D's own rotation
/// (`006a9540`, `NiMatrix3::ToEulerAnglesZXY`) to the negated `NiASin` of
/// the z of the projectile's `009b7010` vector.
pub fn projectile_sync_3d_with_reference(
    e: &mut Engine,
    this: Ptr<Projectile>,
    rotate: u8,
    align: u8,
) {
    let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
    if root == 0 {
        return;
    }
    let position = e.vcall(this.addr(), 0x1F4, &args![]).u32();
    e.call(0x0044_0460, &args![root, position]);
    if rotate != 0 {
        let angles = e.call(0x0043_0830, &args![this]).u32();
        let [x, y, z] = point_words(e, angles);
        e.with_stack(0x24, |e, matrix| {
            e.call(0x0068_15c0, &args![matrix]);
            e.call(
                0x00a5_9660,
                &args![
                    matrix,
                    x87_quiet_bits(z),
                    x87_quiet_bits(x),
                    x87_quiet_bits(y)
                ],
            );
            e.call(0x0043_fa80, &args![root, matrix]);
        });
    }
    if align != 0 && test_flags(e, this, 0x40) {
        e.with_stack(0x24 + 3 * 4 + NI_POINT3_SIZE, |e, locals| {
            let matrix = locals.addr();
            let tilt = matrix + 0x24;
            let first = tilt + 4;
            let last = first + 4;
            let vector = last + 4;
            let rotation = e.call(0x006a_9540, &args![root]).u32();
            for offset in (0..0x24).step_by(4) {
                let word = e.mem.u32(rotation + offset);
                e.mem.set_u32(matrix + offset, word);
            }
            e.call(0x00a5_9400, &args![matrix, first, tilt, last]);
            let vector = e.call(0x009b_7010, &args![this, vector]).u32();
            let sine = x87_quiet(e.mem.f32(vector + 8));
            let angle = -e.call(0x004b_54c0, &args![sine]).f64();
            e.mem.set_f32(tilt, angle as f32);
            let first = x87_quiet(e.mem.f32(first));
            let tilt = x87_quiet(e.mem.f32(tilt));
            let last = x87_quiet(e.mem.f32(last));
            e.call(0x00a5_9660, &args![matrix, first, tilt, last]);
            e.call(0x0043_fa80, &args![root, matrix]);
        });
    }
}
// Translated from 009bffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::ImpactData` constructor: constructs both points
/// (`006815c0`), sets `bDoDamage`, no collidee, the global zero point as
/// location and normal (`004390c0`), no world object, material 0x24, limb
/// -1, not processed, zero world-object count and index.
pub fn fn_009bffb0(e: &mut Engine, this: Ptr<ImpactData>) -> Ptr<ImpactData> {
    e.call(0x0068_15c0, &args![this.byte_add(4)]);
    e.call(0x0068_15c0, &args![this.byte_add(0x10)]);
    e.set(this, ImpactData::bDoDamage, true);
    e.set(this, ImpactData::pCollidee, 0);
    copy_point(e, this.addr() + 4, ZERO_POINT);
    e.call(0x0043_90c0, &args![this.byte_add(0x10), ZERO_POINT]);
    e.set(this, ImpactData::pTargetWorldObject, 0);
    e.set(this, ImpactData::eMaterial, 0x24);
    e.set(this, ImpactData::eDamageLimb, 0xFFFF_FFFF);
    e.set(this, ImpactData::bProcessed, false);
    e.set(this, ImpactData::sTargetWorldObjectCount, 0);
    e.set(this, ImpactData::sTargetWorldObjectIndex, 0);
    this
}

/// A word at +0x154, past the `Projectile` fields: a subclass's field (the
/// type-5 projectile's hit count in `Projectile::AddImpact`; the class is
/// unconfirmed).
const SUBCLASS_HIT_COUNT: u32 = 0x154;

// Translated from 009c0050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the word at +0x154 (see [`SUBCLASS_HIT_COUNT`]).
pub fn fn_009c0050(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + SUBCLASS_HIT_COUNT, value);
}

// Translated from 009c0070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x154 (see [`SUBCLASS_HIT_COUNT`]).
pub fn fn_009c0070(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + SUBCLASS_HIT_COUNT)
}

/// Size of the hit collector `009c0090` keeps on its stack (`EBP-0x490` to
/// `EBP-0x80`).
const HIT_COLLECTOR_SIZE: u32 = 0x410;

// Translation incomplete: 009c0090 - C++ exception unwinding (FS:[0] frame) and the /GS cookie check (00ec408c) are not modelled.
// Translated from 009c0090 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a base object with `009baf30` whose `006ab360` object has
/// `0048cee0`, casts a segment through the projectile's position (minus
/// and plus the `004a3760` vector of 1 and `011a9490`) against the
/// phantom of its cell's Havok world (`00541c80`, `006286d0`, virtual
/// +0x44 into a collector `009b2e30` .. `009b2db0`). The first hit whose
/// reference (`TESObjectREFR::FindReferenceFor3D`) and entity
/// (`004b5950`, `004b5a80`) exist and whose shape type (`0043b4f0`,
/// `0043b4d0`) is 0x17 or 0x18 sets action flag 0x80 on the projectile
/// and 0x100 on the script at `011ca288` (`Script::SetActionFlag` with the
/// reference's extra data, `005d43c0`). Returns whether one was found.
pub fn fn_009c0090(e: &mut Engine, this: Ptr<Projectile>) -> bool {
    let mut found = false;
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if !e.call(0x009b_af30, &args![base]).bool() {
        return found;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let object = e.call(0x006a_b360, &args![base]).u32();
    if e.call(0x0048_cee0, &args![object]).u32() == 0 {
        return found;
    }
    let world = if e.call(0x008d_6f30, &args![this]).u32() != 0 {
        let cell = e.call(0x008d_6f30, &args![this]).u32();
        e.call(0x0045_43c0, &args![cell]).u32()
    } else {
        0
    };
    if world == 0 {
        return found;
    }
    let holder = e.call(0x0054_1c80, &args![world]).u32();
    let phantom = if holder != 0 {
        e.call(0x0062_86d0, &args![holder]).u32()
    } else {
        0
    };
    let scratch = 4 * NI_POINT3_SIZE + 0x20 + HIT_COLLECTOR_SIZE + 4;
    e.with_stack(scratch, |e, locals| {
        let half_a = locals.addr();
        let start = half_a + NI_POINT3_SIZE;
        let half_b = start + NI_POINT3_SIZE;
        let end = half_b + NI_POINT3_SIZE;
        let segment = end + NI_POINT3_SIZE;
        let collector = segment + 0x20;
        let shape_info = collector + HIT_COLLECTOR_SIZE;
        let half = e
            .call(0x004a_3760, &args![half_a, 1.0f32, 0x011a_9490u32])
            .u32();
        let position = e.vcall(this.addr(), 0x1F4, &args![]).u32();
        e.call(0x0043_9ef0, &args![position, start, half]);
        let half = e
            .call(0x004a_3760, &args![half_b, 1.0f32, 0x011a_9490u32])
            .u32();
        let position = e.vcall(this.addr(), 0x1F4, &args![]).u32();
        e.call(0x0043_9e90, &args![position, end, half]);
        e.call(0x0043_7c10, &args![segment]);
        e.call(0x004a_3e00, &args![segment, start]);
        e.call(0x004a_3e00, &args![segment + 0x10, end]);
        e.call(0x009b_2e30, &args![collector, 0u32]);
        e.vcall(phantom, 0x44, &args![segment, collector]);
        let mut index = 0i32;
        while !found && index < e.call(0x0072_6070, &args![collector]).i32() {
            let hit = e.call(0x0062_6660, &args![collector, index]).u32();
            let object = e.mem.u32(hit + 4);
            let collidable = e.call(0x009b_2c90, &args![object]).u32();
            let node = e.call(0x004b_5820, &args![collidable]).u32();
            let reference = e.call(0x0056_f930, &args![node]).u32();
            let world_object = e.call(0x004b_5950, &args![collidable]).u32();
            let entity = if world_object != 0 {
                e.call(0x004b_5a80, &args![world_object]).u32()
            } else {
                0
            };
            if entity != 0 && reference != 0 {
                let info = e.call(0x0043_b4f0, &args![entity, shape_info]).u32();
                let mut matches = e.call(0x0043_b4d0, &args![info]).u32() == 0x17;
                if !matches {
                    let info = e.call(0x0043_b4f0, &args![entity, shape_info]).u32();
                    matches = e.call(0x0043_b4d0, &args![info]).u32() == 0x18;
                }
                if matches {
                    found = true;
                    let extra = e.call(0x005d_43c0, &args![reference]).u32();
                    e.call(0x005a_c750, &args![this, extra, 0x80u32]);
                    let extra = e.call(0x005d_43c0, &args![reference]).u32();
                    let script: u32 = e.global(0x011c_a288);
                    e.call(0x005a_c750, &args![script, extra, 0x100u32]);
                }
            }
            index += 1;
        }
        e.call(0x009b_2db0, &args![collector]);
    });
    found
}
/// `00404010(00408840(a), 00408840(b))` in the order `RunTargetPick` makes
/// the calls: `b` first.
fn pick_min_of_two(e: &mut Engine, a: f32, b: f32) -> f32 {
    let rb = e.call(0x0040_8840, &args![b]).f64() as f32;
    let ra = e.call(0x0040_8840, &args![a]).f64() as f32;
    e.call(0x0040_4010, &args![ra, rb]).f64() as f32
}

/// The largest-axis ratio `RunTargetPick` tests: each axis of `offset`
/// divided by the matching axis of `extent` (z, y, then x), combined through
/// `00408840` and `00404010`.
fn pick_axis_ratio(e: &mut Engine, offset: u32, extent: u32) -> f32 {
    let ratio = |e: &mut Engine, axis: u32| {
        (e.mem.f32(offset + axis) as f64 / e.mem.f32(extent + axis) as f64) as f32
    };
    let z = ratio(e, 8);
    let y = ratio(e, 4);
    let yz = pick_min_of_two(e, y, z);
    let x = ratio(e, 0);
    let rx = e.call(0x0040_8840, &args![x]).f64() as f32;
    e.call(0x0040_4010, &args![rx, yz]).f64() as f32
}

/// Shortens `ray` (`00439180`) so that `start + ray` stays on the inner
/// side of `bound` on each axis: `greater` selects the upper bound test
/// (`bound < start + ray`) or the lower one (`bound > start + ray`).
fn pick_clamp_axis(e: &mut Engine, start: u32, ray: u32, bound: u32, axis: u32, upper: bool) {
    let reach = e.mem.f32(start + axis) as f64 + e.mem.f32(ray + axis) as f64;
    let limit = e.mem.f32(bound + axis) as f64;
    let outside = if upper { limit < reach } else { limit > reach };
    if outside {
        let scale =
            ((limit - e.mem.f32(start + axis) as f64) / e.mem.f32(ray + axis) as f64) as f32;
        e.call(0x0043_9180, &args![ray, scale]);
    }
}

/// Bytes of the `RunTargetPick` frame below `EBP` that the translation
/// models (its locals sit at their `EBP` offsets).
const PICK_FRAME: u32 = 0x5A0;

// Translation incomplete: 009c03f0 - C++ exception unwinding (FS:[0] frame) and the /GS cookie check (00ec408c) are not modelled.
// Translated from 009c03f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::RunTargetPick` (Xbox PDB): casts the projectile's ray for
/// this frame in its cell's Havok world; false without one.
///
/// The ray starts at the position and points along the heading (virtual
/// +0x2BC, `004a0c90`) tilted by `MobileObject::GetLooking`
/// (`NiMatrix3::MakeXRotation`, product `0043f8d0`, column 1, normalised),
/// with the length `00979260() - 00508070()`. The pick (`004a3c20`) uses
/// the character controller's collision filter (`0070c440`), or layer 6
/// with the actor shooter's filter (`00931ed0`) or a new one (`005dbe10`),
/// and the pick cache's object (`00500a20`). Against the world's bounds
/// (`00541cb0` +0x310 and +0x320 through `00458620`, origin `00c66be0`,
/// scaled by the `float` at `010162c0`): a start outside logs
/// `005b5e40("...", name)`; an end outside shortens the ray to the bounds.
/// The ray is cast with the pick count override (`bhkWorld::
/// SetPickCountOverride`) through the player's world (virtual +0xC8) and
/// `Projectile::PickAutoWater` with a distance of `FLT_MAX`; a hit is
/// processed by `009c0d80`. With the impact debug setting the ray is drawn
/// (red when `00632ce0`, else yellow); when `00632ce0` holds, `009bc8f0`
/// runs. Returns whether it hit.
pub fn projectile_run_target_pick(e: &mut Engine, this: Ptr<Projectile>) -> bool {
    if e.call(0x008d_6f30, &args![this]).u32() == 0 {
        return false;
    }
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    if e.call(0x0045_43c0, &args![cell]).u32() == 0 {
        return false;
    }
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let world = e.call(0x0045_43c0, &args![cell]).u32();
    e.with_stack(PICK_FRAME, |e, frame| {
        let at = |offset: u32| frame.addr() + PICK_FRAME - offset;
        let pick = at(0xD0);
        let start = at(0xDC);
        let direction = at(0xE8);
        let heading = at(0x10C);
        let tilt = at(0x130);
        let product = at(0x154);
        let filter = at(0x158);
        let filter_out = at(0x15C);
        let filter_info = at(0x164);
        let ray = at(0x180);
        let origin = at(0x18C);
        let upper = at(0x19C);
        let lower = at(0x1A8);
        let extent = at(0x1B4);
        let start_offset = at(0x1C0);
        let end = at(0x1E4);
        let end_offset = at(0x1F0);
        let results = at(0x540);
        let distance = at(0x550);
        let color = at(0x58C);
        let line_end = at(0x598);

        e.call(0x004a_3c20, &args![pick]);
        e.call(0x0068_15c0, &args![start]);
        e.call(0x0068_15c0, &args![direction]);
        let position = e.vcall(this.addr(), 0x1F4, &args![]).u32();
        copy_point(e, start, position);
        e.call(0x0068_15c0, &args![heading]);
        let angle = e.vcall(this.addr(), 0x2BC, &args![0u32]).f64() as f32;
        e.call(0x004a_0c90, &args![heading, angle]);
        e.call(0x0068_15c0, &args![tilt]);
        let looking = e.call(0x0093_1d70, &args![this]).f64() as f32;
        e.call(0x0052_4ac0, &args![tilt, looking]);
        let rotation = e.call(0x0043_f8d0, &args![heading, product, tilt]).u32();
        for offset in (0..0x24).step_by(4) {
            let word = e.mem.u32(rotation + offset);
            e.mem.set_u32(heading + offset, word);
        }
        e.call(0x0043_9f50, &args![heading, 1u32, direction]);
        e.call(0x004a_0c10, &args![direction]);
        e.call(0x008c_71b0, &args![filter, 0u32]);
        if e.call(0x0093_06d0, &args![this]).u32() != 0 {
            let controller = e.call(0x0093_06d0, &args![this]).u32();
            let info = e.call(0x0070_c440, &args![controller, filter_out]).u32();
            let word = e.mem.u32(info);
            e.mem.set_u32(filter, word);
        } else {
            e.call(0x004a_39f0, &args![filter, 6u32]);
            let mut actor_filter = false;
            if e.call(GET_SHOOTER, &args![this]).u32() != 0 {
                let shooter = e.call(GET_SHOOTER, &args![this]).u32();
                if is_actor(e, shooter) {
                    let shooter = e.call(GET_SHOOTER, &args![this]).u32();
                    let info = e.call(0x0093_1ed0, &args![shooter, filter_info]).u32();
                    let value = e.call(0x004a_3a20, &args![info]).u32();
                    e.call(0x0059_ce80, &args![filter, value]);
                    actor_filter = true;
                }
            }
            if !actor_filter {
                let value = e.call(0x005d_be10, &args![]).u32();
                e.call(0x0059_ce80, &args![filter, value]);
            }
        }
        let cache = this.byte_add(PICK_CACHE);
        if e.call(0x0055_9450, &args![cache]).u32() != 0 {
            let object = e.call(0x0055_9450, &args![cache]).u32();
            if e.call(0x0044_ddc0, &args![object]).u32() != 0 {
                let object = e.call(0x0055_9450, &args![cache]).u32();
                let target = e.call(0x0044_ddc0, &args![object]).u32();
                e.call(0x0050_0a20, &args![pick, target]);
            }
        }
        let far = e.call(0x0097_9260, &args![this]).f64();
        let near = e.call(0x0050_8070, &args![this]).f64();
        let length = (far - near) as f32;
        e.call(0x0045_bb20, &args![direction, ray, length]);
        e.call(0x0068_15c0, &args![origin]);
        e.call(0x00c6_6be0, &args![world, origin]);
        let havok_world = e.call(0x0054_1cb0, &args![world]).u32();
        e.call(0x0068_15c0, &args![upper]);
        e.call(0x0045_8620, &args![upper, havok_world + 0x320]);
        e.call(0x0068_15c0, &args![lower]);
        e.call(0x0045_8620, &args![lower, havok_world + 0x310]);
        e.call(0x0043_9ef0, &args![upper, extent, lower]);
        let scale = float_global(e, 0x0101_62c0);
        e.call(0x0049_41c0, &args![extent, scale]);
        e.call(0x0043_9ef0, &args![start, start_offset, origin]);
        let one: f64 = e.global(0x0101_2070);
        let start_ratio = pick_axis_ratio(e, start_offset, extent);
        if start_ratio as f64 > one {
            let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
            let name = if e.call(0x0047_4cb0, &args![base]).u32() != 0 {
                let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
                e.vcall(base, 0x130, &args![]).u32()
            } else {
                let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
                e.call(0x0040_8da0, &args![base + 0x30]).u32()
            };
            e.call(0x005b_5e40, &args![0x0109_0494u32, name]);
        } else {
            e.call(0x0043_9e90, &args![start, end, ray]);
            e.call(0x0043_9ef0, &args![end, end_offset, origin]);
            let end_ratio = pick_axis_ratio(e, end_offset, extent);
            if end_ratio as f64 > one {
                for axis in [0, 4, 8] {
                    pick_clamp_axis(e, start, ray, upper, axis, true);
                }
                for axis in [0, 4, 8] {
                    pick_clamp_axis(e, start, ray, lower, axis, false);
                }
            }
        }
        let word = e.mem.u32(filter);
        e.call(0x004a_3f70, &args![pick, word]);
        e.call(0x004a_3da0, &args![pick, start]);
        e.call(0x007f_68a0, &args![pick, ray]);
        e.call(0x004a_3a70, &args![results]);
        e.call(0x004a_3fb0, &args![pick, results]);
        let previous = e.call(0x0088_8960, &args![]).u8();
        e.call(0x00c6_6230, &args![1u32]);
        let player: u32 = e.global(PLAYER);
        let player_cell = e.call(0x008d_6f30, &args![player]).u32();
        let player_world = e.call(0x0045_43c0, &args![player_cell]).u32();
        let mut hit = e.vcall(player_world, 0xC8, &args![pick]).u8();
        e.call(0x00c6_6230, &args![previous as u32]);
        let max = float_global(e, FLOAT_MAX);
        e.mem.set_f32(distance, max);
        let water = e.call(0x009c_5af0, &args![this, pick, distance]).u8();
        hit |= water;
        if hit != 0 {
            let reach = x87_quiet(e.mem.f32(distance));
            let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
            let processed = fn_009c0d80(e, Ptr::new(pick), this, base, 0, 1, reach);
            hit = (processed != 0) as u8;
        }
        if bool_setting(e, SETTING_DEBUG_IMPACTS) {
            let rgba = if e.call(0x0063_2ce0, &args![pick]).bool() {
                [1.0, 0.0, 0.0, 1.0]
            } else {
                [1.0, 1.0, 0.0, 1.0]
            };
            debug_color(e, color, rgba);
            let end = e.call(0x0043_9e90, &args![start, line_end, ray]).u32();
            let line = e
                .call(0x004b_3890, &args![start, color, end, color, 1u32])
                .u32();
            let time = setting_float(e, SETTING_DEBUG_OBJECT_TIME);
            let tes: u32 = e.global(TES);
            e.call(0x0045_8e20, &args![tes, line, time]);
        }
        if e.call(0x0063_2ce0, &args![pick]).bool() {
            fn_009bc8f0(e, this);
        }
        e.call(0x004a_3bc0, &args![results]);
        hit != 0
    })
}
/// Bytes of the `009c0d80` frame below `EBP` that the translation models
/// (its locals sit at their `EBP` offsets).
const IMPACT_PICK_FRAME: u32 = 0x600;

/// The `i`-th hit of a pick (`008cdd90`, `00460140`, `004a46b0`).
fn pick_hit(e: &mut Engine, pick: u32, index: i32) -> u32 {
    let collector = e.call(0x008c_dd90, &args![pick]).u32();
    let hits = e.call(0x0046_0140, &args![collector]).u32();
    e.call(0x004a_46b0, &args![hits, index]).u32()
}

/// The number of hits of a pick (`008cdd90`, `00460140`, `00726070`).
fn pick_hit_count(e: &mut Engine, pick: u32) -> i32 {
    let collector = e.call(0x008c_dd90, &args![pick]).u32();
    let hits = e.call(0x0046_0140, &args![collector]).u32();
    e.call(0x0072_6070, &args![hits]).i32()
}

/// The shape type of an entity (`0043b4f0` into `info`, `0043b4d0`).
fn shape_type(e: &mut Engine, entity: u32, info: u32) -> u32 {
    let shape = e.call(0x0043_b4f0, &args![entity, info]).u32();
    e.call(0x0043_b4d0, &args![shape]).u32()
}

/// A sound at a point: `BSAudio::GetSoundHandleByFilename` for the base
/// object's sound (flags 0x102) into `handle`, positioned
/// (`BSSoundHandle::SetPosition`) and played (`BSSoundHandle::Play(0)`).
fn play_impact_sound(e: &mut Engine, projectile: Ptr<Projectile>, handle: u32, point: u32) {
    let base = e.call(GET_BASE_OBJECT, &args![projectile]).u32();
    let sound = e.call(0x004f_d3e0, &args![base]).u32();
    let base = e.call(GET_BASE_OBJECT, &args![projectile]).u32();
    let sound_again = e.call(0x004f_d3e0, &args![base]).u32();
    let file = e.call(0x0051_1840, &args![sound_again]).u32();
    let audio = e.call(0x0045_3a70, &args![]).u32();
    e.call(0x00ad_7480, &args![audio, handle, file, 0x102u32, sound]);
    let x = x87_quiet(e.mem.f32(point));
    let y = x87_quiet(e.mem.f32(point + 4));
    let z = x87_quiet(e.mem.f32(point + 8));
    e.call(0x00ad_8b60, &args![handle, x, y, z]);
    e.call(0x00ad_8830, &args![handle, 0u32]);
}

/// The cyan debug marker `009c0d80` draws at `position`.
fn impact_sound_marker(
    e: &mut Engine,
    color: u32,
    position: Option<u32>,
    projectile: Ptr<Projectile>,
) {
    let alpha = float_global(e, 0x0101_7870);
    let size = float_global(e, 0x0101_712c);
    e.call(0x0041_4430, &args![color, 0.0f32, 1.0f32, 1.0f32, alpha]);
    let marker = e.call(0x004b_29b0, &args![size, color, 1u32]).u32();
    e.call(0x00a5_a040, &args![marker]);
    let position = match position {
        Some(point) => point,
        None => {
            let root = e.vcall(projectile.addr(), 0x1D0, &args![]).u32();
            e.call(0x0045_bb80, &args![root]).u32()
        }
    };
    e.call(0x0044_0460, &args![marker, position]);
    let time = setting_float(e, SETTING_DEBUG_OBJECT_TIME);
    let tes: u32 = e.global(TES);
    e.call(0x0045_8e20, &args![tes, marker, time]);
}

// Translation incomplete: 009c0d80 - C++ exception unwinding (FS:[0] frame) and the /GS cookie check (00ec408c) are not modelled.
// Translated from 009c0d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Processes the hits of a cast pick (cdecl): returns the scene-graph node of
/// the hit that was taken, or 0. `base` defaults to the projectile's base
/// object.
///
/// Hits are taken in order while none is taken and their distance is not
/// beyond `max_distance`. A hit on an entity (`004b5950`, `004b5a80`) first
/// sets the action flags 0x80 on the projectile and 0x100 on its weapon for
/// shape types 0x17/0x18 (`Script::SetActionFlag`), then counts only when
/// it is in the set at `01268444` (`0043b300`) or, with
/// `include_type_1e`, of shape type 0x1E; such a hit marks a target and,
/// when its first collision object is flagged (`009c19e0`) or has no
/// virtual +0x94, re-casts a short pick (`009c5ba0`) whose first hit
/// replaces this one; both directions are scaled by the `float` at
/// `010162c0`. `009c2c80` can veto the hit (it gets the hit's collidable
/// as the re-cast left it). A taken hit on a projectile
/// moves it to the hit (`009795c0`, `009c4e60` with flag 1) and adds the
/// impact (virtual +0x32C, material 0x24, last valid shape key); without a
/// projectile any hit is taken. The distance goes to `distance_out`.
///
/// For a projectile without a taken hit and a finite `max_distance`
/// (`double` at `010231b0`), an impact with nothing at the ray's end is
/// added (normal at `011a9484`, material 8). A missed target that is the
/// player plays the base's sound at the pick's `005dbe60` point; otherwise,
/// for a non-player shooter and a listener (`0044edb0`), the sound plays at
/// the point of the ray nearest the listener when it is closer than the
/// `double` at `01088210`.
pub fn fn_009c0d80(
    e: &mut Engine,
    pick: Ptr,
    projectile: Ptr<Projectile>,
    base: u32,
    distance_out: u32,
    include_type_1e: u8,
    max_distance: f32,
) -> u32 {
    let mut base = base;
    if base == 0 && !projectile.is_null() {
        base = e.call(GET_BASE_OBJECT, &args![projectile]).u32();
    }
    let mut taken = false;
    let mut result = 0u32;
    let pick = pick.addr();
    if pick == 0 || e.call(0x008c_dd90, &args![pick]).u32() == 0 {
        return result;
    }
    e.with_stack(IMPACT_PICK_FRAME, |e, frame| {
        let at = |offset: u32| frame.addr() + IMPACT_PICK_FRAME - offset;
        let start = at(0x28);
        let direction = at(0x34);
        let hit = at(0xA0);
        let mut had_target = false;
        let mut target_reference = 0u32;
        e.call(0x0068_15c0, &args![start]);
        e.call(0x0068_15c0, &args![direction]);
        e.call(0x0045_85f0, &args![pick, start]);
        fn_009c1a20(e, Ptr::new(pick), direction);
        let mut index = 0i32;
        while !taken && index < pick_hit_count(e, pick) {
            let record = pick_hit(e, pick, index);
            e.call(0x0069_6c30, &args![hit, record]);
            let distance = e.mem.f32(hit + 0x10) as f64;
            if !((max_distance as f64) < distance) {
                let collidable = e.mem.u32(hit + 0x50);
                let node = e.call(0x004b_5820, &args![collidable]).u32();
                let reference = e.call(0x0056_f930, &args![node]).u32();
                if node != 0 {
                    let mut take = true;
                    let world_object = e.call(0x004b_5950, &args![collidable]).u32();
                    let entity = if world_object != 0 {
                        e.call(0x004b_5a80, &args![world_object]).u32()
                    } else {
                        0
                    };
                    if entity != 0 {
                        take = false;
                        if !projectile.is_null() && reference != 0 {
                            let mut flagged = shape_type(e, entity, at(0xBC)) == 0x17;
                            if !flagged {
                                flagged = shape_type(e, entity, at(0xC0)) == 0x18;
                            }
                            if flagged {
                                let extra = e.call(0x005d_43c0, &args![reference]).u32();
                                e.call(0x005a_c750, &args![projectile, extra, 0x80u32]);
                                if e.call(GET_WEAPON, &args![projectile]).u32() != 0 {
                                    let extra = e.call(0x005d_43c0, &args![reference]).u32();
                                    let weapon = e.call(GET_WEAPON, &args![projectile]).u32();
                                    e.call(0x005a_c750, &args![weapon, extra, 0x100u32]);
                                }
                            }
                        }
                        let counts = e.call(0x0043_b300, &args![0x0126_8444u32, entity]).bool()
                            || (include_type_1e != 0 && shape_type(e, entity, at(0xC4)) == 0x1E);
                        if counts {
                            had_target = true;
                            target_reference = reference;
                            take = refine_hit(e, frame.addr(), pick, node, take);
                        }
                    }
                    // 009c121d: the collidable is read again from the hit (`[EBP-0x50]`, its
                    // +0x50) after the short re-cast copied its first hit over it (009c11f7).
                    let collidable = e.mem.u32(hit + 0x50);
                    if !fn_009c2c80(e, reference, collidable, projectile.addr(), base) {
                        take = false;
                    }
                    if take {
                        if !projectile.is_null() {
                            taken = add_hit_impact(e, frame.addr(), projectile, reference);
                            if taken {
                                result = node;
                            }
                        } else {
                            taken = true;
                            result = node;
                        }
                        if taken && distance_out != 0 {
                            let distance = x87_quiet(e.mem.f32(hit + 0x10));
                            e.mem.set_f32(distance_out, distance);
                        }
                    }
                }
            }
            index += 1;
        }
        if projectile.is_null() {
            return;
        }
        let unlimited: f64 = e.global(0x0102_31b0);
        if !taken && max_distance as f64 != unlimited {
            let scaled = at(0x50C);
            let end = at(0x518);
            e.call(
                0x0045_bb20,
                &args![direction, scaled, x87_quiet(max_distance)],
            );
            e.call(0x0043_9e90, &args![start, end, scaled]);
            if test_flags(e, projectile, 1) {
                let length = e.call(0x0045_7990, &args![scaled]).f64() as f32;
                e.call(0x0097_95c0, &args![projectile, length]);
                let [x, y, z] = point_words(e, scaled);
                e.call(0x009c_4e60, &args![projectile, x, y, z, 0u32]);
            }
            e.vcall(
                projectile.addr(),
                0x32C,
                &args![0u32, end, 0x011a_9484u32, 0u32, 0xFFFF_FFFFu32, 8u32],
            );
        }
        if !taken && had_target {
            let player: u32 = e.global(PLAYER);
            if target_reference == 0 || target_reference != player {
                return;
            }
            let point = at(0x524);
            let handle = at(0x534);
            e.call(0x0068_15c0, &args![point]);
            e.call(0x005d_be60, &args![pick, point]);
            if e.call(GET_BASE_OBJECT, &args![projectile]).u32() == 0 {
                return;
            }
            let base = e.call(GET_BASE_OBJECT, &args![projectile]).u32();
            if e.call(0x004f_d3e0, &args![base]).u32() == 0 {
                return;
            }
            play_impact_sound(e, projectile, handle, point);
            if e.global::<u8>(0x011f_20a4) != 0
                && e.vcall(projectile.addr(), 0x1D0, &args![]).u32() != 0
            {
                impact_sound_marker(e, at(0x544), None, projectile);
            }
            e.call(0x0048_3710, &args![handle]);
            return;
        }
        let base = e.call(GET_BASE_OBJECT, &args![projectile]).u32();
        if e.call(0x004f_d3e0, &args![base]).u32() == 0 {
            return;
        }
        let player: u32 = e.global(PLAYER);
        if taken && target_reference == player {
            return;
        }
        if e.call(GET_SHOOTER, &args![projectile]).u32() == player {
            return;
        }
        let audio = e.call(0x0045_3a70, &args![]).u32();
        if e.call(0x0044_edb0, &args![audio]).u32() == 0 {
            return;
        }
        play_nearest_sound(e, frame.addr(), pick, projectile, target_reference);
    });
    result
}

/// The short re-cast of `009c0d80` for a target hit: returns whether the
/// hit is taken (`take` unchanged when the first collision object does not
/// qualify or nothing is hit).
fn refine_hit(e: &mut Engine, frame: u32, pick: u32, node: u32, take: bool) -> bool {
    let at = |offset: u32| frame + IMPACT_PICK_FRAME - offset;
    let first = e.call(0x004b_5260, &args![node]).u32();
    let collision = if first != 0 {
        e.call(0x006f_a820, &args![first]).u32()
    } else {
        0
    };
    if collision == 0 {
        return take;
    }
    let info = e.call(0x0043_b4f0, &args![collision, at(0xD4)]).u32();
    if !fn_009c19e0(e, Ptr::new(info)) && e.vcall(collision, 0x94, &args![]).u32() != 0 {
        return take;
    }
    let mut take = take;
    let short_pick = at(0x190);
    let start = at(0x19C);
    let direction = at(0x1A8);
    let filter = at(0x1AC);
    let collector = at(0x4D0);
    e.call(0x004a_3c20, &args![short_pick]);
    e.call(0x0068_15c0, &args![start]);
    e.call(0x0045_85f0, &args![pick, start]);
    e.call(0x0068_15c0, &args![direction]);
    fn_009c1a20(e, Ptr::new(pick), direction);
    let scale = float_global(e, 0x0101_62c0);
    e.call(0x0043_9180, &args![at(0x34), scale]);
    let scale = float_global(e, 0x0101_62c0);
    e.call(0x0043_9180, &args![direction, scale]);
    let info = fn_009c1a00(e, Ptr::new(pick), filter);
    let word = e.mem.u32(info.addr());
    e.call(0x004a_3f70, &args![short_pick, word]);
    e.call(0x004a_3da0, &args![short_pick, start]);
    e.call(0x007f_68a0, &args![short_pick, direction]);
    e.call(0x004a_3a70, &args![collector]);
    e.call(0x004a_3fb0, &args![short_pick, collector]);
    let found = e
        .call(0x009c_5ba0, &args![short_pick, node, 0u32, 1u32])
        .bool();
    if found && pick_hit_count(e, short_pick) > 0 {
        let record = pick_hit(e, short_pick, 0);
        fn_009c1a70(e, Ptr::new(at(0xA0)), record);
        take = true;
    }
    e.call(0x004a_3bc0, &args![collector]);
    take
}

/// Adds the impact for a taken hit of `009c0d80` (virtual +0x32C); returns
/// its result.
fn add_hit_impact(e: &mut Engine, frame: u32, projectile: Ptr<Projectile>, reference: u32) -> bool {
    let at = |offset: u32| frame + IMPACT_PICK_FRAME - offset;
    let hit = at(0xA0);
    let scaled = at(0x4E0);
    let point = at(0x4EC);
    let normal = at(0x4F8);
    let distance = x87_quiet(e.mem.f32(hit + 0x10));
    e.call(0x0045_bb20, &args![at(0x34), scaled, distance]);
    e.call(0x0043_9e90, &args![at(0x28), point, scaled]);
    e.call(0x0068_15c0, &args![normal]);
    e.call(0x004a_3970, &args![normal, hit]);
    if test_flags(e, projectile, 1) {
        let length = e.call(0x0045_7990, &args![scaled]).f64() as f32;
        e.call(0x0097_95c0, &args![projectile, length]);
        let [x, y, z] = point_words(e, scaled);
        e.call(0x009c_4e60, &args![projectile, x, y, z, 0u32]);
    }
    let mut key = 0xFFFF_FFFFu32;
    for slot in 0..8 {
        let value = e.mem.u32(hit + 0x20 + 4 * slot);
        if value == 0xFFFF_FFFF {
            break;
        }
        key = value;
    }
    let collidable = e.mem.u32(hit + 0x50);
    e.vcall(
        projectile.addr(),
        0x32C,
        &args![reference, point, normal, collidable, key, 0x24u32],
    )
    .bool()
}

/// The near-miss sound of `009c0d80`: the point of the ray (pick start to
/// the hit reference's position, or the pick's `004587f0` end) nearest the
/// listener's position (its virtual +0xC), played when closer than the
/// `double` at `01088210`.
fn play_nearest_sound(
    e: &mut Engine,
    frame: u32,
    pick: u32,
    projectile: Ptr<Projectile>,
    target_reference: u32,
) {
    let at = |offset: u32| frame + IMPACT_PICK_FRAME - offset;
    let start = at(0x554);
    let end = at(0x560);
    let ray = at(0x56C);
    let to_listener = at(0x578);
    let listener_position = at(0x584);
    for local in [start, end, ray, to_listener, listener_position] {
        e.call(0x0068_15c0, &args![local]);
    }
    let audio = e.call(0x0045_3a70, &args![]).u32();
    let listener = e.call(0x0044_edb0, &args![audio]).u32();
    let position = e.vcall(listener, 0xC, &args![]).u32();
    copy_point(e, listener_position, position);
    if target_reference != 0 {
        let position = e.vcall(target_reference, 0x1F4, &args![]).u32();
        copy_point(e, end, position);
    } else {
        e.call(0x0045_87f0, &args![pick, end]);
    }
    e.call(0x0045_85f0, &args![pick, start]);
    let difference = e.call(0x0043_9ef0, &args![end, at(0x594), start]).u32();
    copy_point(e, ray, difference);
    let difference = e
        .call(0x0043_9ef0, &args![listener_position, at(0x5A0), start])
        .u32();
    copy_point(e, to_listener, difference);
    let along = e.call(0x004b_6190, &args![ray, to_listener]).f64() as f32;
    let length = e.call(0x004b_6190, &args![ray, ray]).f64() as f32;
    let fraction = (along as f64 / length as f64) as f32;
    let nearest = at(0x5C4);
    let step = e.call(0x004a_3760, &args![at(0x5B8), fraction, ray]).u32();
    e.call(0x0043_9e90, &args![start, nearest, step]);
    let offset = e
        .call(0x0043_9ef0, &args![listener_position, at(0x5D0), nearest])
        .u32();
    let distance = e.call(0x0045_7990, &args![offset]).f64() as f32;
    let limit: f64 = e.global(0x0108_8210);
    if !((distance as f64) < limit) {
        return;
    }
    let handle = at(0x5E4);
    play_impact_sound(e, projectile, handle, nearest);
    if e.global::<u8>(0x011f_20a4) != 0 {
        impact_sound_marker(e, at(0x5F4), Some(nearest), projectile);
    }
    e.call(0x0048_3710, &args![handle]);
}

// Translated from 009c19e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x4000 of the first word is set.
pub fn fn_009c19e0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr()) & 0x4000 != 0
}

// Translated from 009c1a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the collision-filter info of the word at +0x24 (`008c71b0`) in
/// `result` and returns it.
pub fn fn_009c1a00(e: &mut Engine, this: Ptr, result: u32) -> Ptr {
    let word = e.mem.u32(this.addr() + 0x24);
    e.call(0x008c_71b0, &args![result, word]);
    Ptr::new(result)
}

// Translated from 009c1a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the vector at +0x90 (`009c1a50`) into `result` (`00458620`).
pub fn fn_009c1a20(e: &mut Engine, this: Ptr, result: u32) {
    let vector = fn_009c1a50(e, this);
    e.call(0x0045_8620, &args![result, vector]);
}

// Translated from 009c1a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the vector at +0x90.
pub fn fn_009c1a50(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr() + 0x90
}

// Translated from 009c1a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a hit record (`009c1aa0`) and its word at +0x50; returns `this`.
pub fn fn_009c1a70(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    fn_009c1aa0(e, this, source);
    let word = e.mem.u32(source + 0x50);
    e.mem.set_u32(this.addr() + 0x50, word);
    this
}

// Translated from 009c1aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the base of a hit record (`009c1b00`), its eight words at +0x20
/// and the word at +0x40; returns `this`.
pub fn fn_009c1aa0(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    fn_009c1b00(e, this, source);
    for index in 0..8 {
        let word = e.mem.u32(source + 0x20 + 4 * index);
        e.mem.set_u32(this.addr() + 0x20 + 4 * index, word);
    }
    let word = e.mem.u32(source + 0x40);
    e.mem.set_u32(this.addr() + 0x40, word);
    this
}

// Translated from 009c1b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the first part of a hit record: the vector (`004a3f10`), the
/// `float` at +0x10 (through the x87 stack), the word at +0x14 and the two
/// words at +0x18; returns `this`.
pub fn fn_009c1b00(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    e.call(0x004a_3f10, &args![this, source]);
    let fraction = x87_quiet(e.mem.f32(source + 0x10));
    e.mem.set_f32(this.addr() + 0x10, fraction);
    let word = e.mem.u32(source + 0x14);
    e.mem.set_u32(this.addr() + 0x14, word);
    for index in 0..2 {
        let word = e.mem.u32(source + 0x18 + 4 * index);
        e.mem.set_u32(this.addr() + 0x18 + 4 * index, word);
    }
    this
}
// Translation incomplete: 009c1b70 - C++ exception unwinding (FS:[0] frame) is not modelled.
// Translated from 009c1b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::ProcessImpacts` (Xbox PDB): once (`bImpactProcessed`), walks
/// the impact list (`008256d0` empty test, `006815c0` item, `00726070`
/// next) until the list ends or `00437b90` holds; returns whether any
/// impact was walked.
///
/// A damaging impact (`bDoDamage`, damage `00885d70` above 0) on a
/// collidee: between actors where the shooter's `00437b90` does not hold it
/// is a `Projectile::CombatHit`; otherwise the collidee gets the actor cause
/// (virtual +0x168 to +0x164) when its body's `00517630` is below 4, and
/// the damage plus the weapon's `00645c20` when an aiming actor shooter
/// (virtual +0x360) is in VATS (`00525430`): an actor through a `HitData`
/// (0x64 bytes, `HitData::InitializeHitData_ov4`, `Actor::HitMe`), anything
/// else through its virtual +0x144; the player's shot at a `00477ba0`
/// collidee runs `00963eb0(10, float at 0102918c, 0)`. The impact counts as
/// changed when the collidee's 3D (`0043fcd0`) changed. Unless the
/// collidee has form flag 0x20, or the base's `004fd360` holds without
/// `00975300` or `005de080`, an unchanged impact notifies the VATS
/// shooters at `011f21e8`/`011f21ec` (`0082ee10`) and runs `009c20e0` and
/// `009c2e80` with the projectile's `009b7010` vector.
pub fn projectile_process_impacts(e: &mut Engine, this: Ptr<Projectile>) -> bool {
    let mut processed = false;
    let mut target_alive = false;
    if e.get(this, Projectile::bImpactProcessed)
        || e.call(0x0082_56d0, &args![this.byte_add(IMPACT_LIST)])
            .bool()
    {
        return processed;
    }
    let mut node = this.addr() + IMPACT_LIST;
    loop {
        if node == 0 || e.call(0x0082_56d0, &args![node]).bool() {
            break;
        }
        let item = e.call(0x0068_15c0, &args![node]).u32();
        let impact: Ptr<ImpactData> = Ptr::new(e.mem.u32(item));
        node = e.call(0x0072_6070, &args![node]).u32();
        let mut changed = false;
        let collidee = e.get(impact, ImpactData::pCollidee);
        if collidee != 0
            && e.call(0x0088_5d70, &args![this]).f64() > 0.0
            && e.get(impact, ImpactData::bDoDamage)
        {
            let root_before = e.call(0x0043_fcd0, &args![collidee]).u32();
            let mut combat_hit = false;
            if e.call(GET_SHOOTER, &args![this]).u32() != 0 {
                let shooter = e.call(GET_SHOOTER, &args![this]).u32();
                if is_actor(e, shooter) && is_actor(e, collidee) {
                    let shooter = e.call(GET_SHOOTER, &args![this]).u32();
                    combat_hit = !e.call(0x0043_7b90, &args![shooter]).bool();
                }
            }
            if combat_hit {
                target_alive = !e.vcall(collidee, 0x22C, &args![0u32]).bool();
                projectile_combat_hit(e, this, collidee, 0);
            } else {
                target_alive = damage_collidee(e, this, impact, collidee, target_alive);
            }
            if root_before != e.call(0x0043_fcd0, &args![collidee]).u32() {
                changed = true;
            }
        }
        let protected = collidee != 0 && e.call(0x0044_ddc0, &args![collidee]).u32() & 0x20 != 0;
        if !protected && !impact_skipped_by_base(e, this) && !changed {
            notify_vats_shooters(e, this, collidee, target_alive);
            let world_object = e.get(impact, ImpactData::pTargetWorldObject);
            let world_object = if world_object != 0 {
                e.call(0x0046_0140, &args![world_object]).u32()
            } else {
                0
            };
            let material = e.get(impact, ImpactData::eMaterial);
            fn_009c20e0(
                e,
                this,
                collidee,
                impact.byte_add(4),
                impact.byte_add(0x10),
                world_object,
                material,
            );
            let [vx, vy, vz] = e.with_stack(NI_POINT3_SIZE, |e, local| {
                let vector = e.call(0x009b_7010, &args![this, local]).u32();
                point_words(e, vector)
            });
            let [lx, ly, lz] = point_words(e, impact.addr() + 4);
            fn_009c2e80(e, this, lx, ly, lz, vx, vy, vz, world_object);
        }
        processed = true;
        if e.call(0x0043_7b90, &args![this]).bool() {
            break;
        }
    }
    e.set(this, Projectile::bImpactProcessed, true);
    processed
}

/// The non-combat damage of `Projectile::ProcessImpacts`; returns the new
/// "target alive" flag (unchanged when the collidee is not an actor).
fn damage_collidee(
    e: &mut Engine,
    this: Ptr<Projectile>,
    impact: Ptr<ImpactData>,
    collidee: u32,
    target_alive: bool,
) -> bool {
    let mut target_alive = target_alive;
    let world_object = e.get(impact, ImpactData::pTargetWorldObject);
    let body = e.call(0x004b_5a80, &args![world_object]).u32();
    if body != 0 && e.call(0x0051_7630, &args![body]).i32() < 4 {
        let cause = e.vcall(this.addr(), 0x168, &args![]).u32();
        e.vcall(collidee, 0x164, &args![cause]);
    }
    let mut actor = 0u32;
    if e.call(GET_SHOOTER, &args![this]).u32() != 0 {
        let shooter = e.call(GET_SHOOTER, &args![this]).u32();
        if is_actor(e, shooter) {
            actor = e.call(GET_SHOOTER, &args![this]).u32();
        }
    }
    let mut bonus = 0.0f32;
    if actor != 0
        && e.vcall(actor, 0x360, &args![]).bool()
        && e.call(0x0052_5430, &args![VATS]).bool()
        && e.call(GET_WEAPON, &args![this]).u32() != 0
    {
        let weapon = e.call(GET_WEAPON, &args![this]).u32();
        bonus = e.call(0x0064_5c20, &args![weapon]).u16() as f32;
    }
    let damage = (e.call(0x0088_5d70, &args![this]).f64() + bonus as f64) as f32;
    if is_actor(e, collidee) {
        e.with_stack(4, |e, pointer| {
            let block = allocate(e, 0x64);
            let hit = if block != 0 {
                e.call(0x009b_4d90, &args![block]).u32()
            } else {
                0
            };
            e.call(0x008c_1c30, &args![pointer, hit]);
            let hit = e.call(0x0055_9450, &args![pointer]).u32();
            e.call(0x009b_5650, &args![hit, 0u32, collidee, 0u32, this]);
            target_alive = !e.vcall(collidee, 0x22C, &args![0u32]).bool();
            let hit = e.call(0x0055_9450, &args![pointer]).u32();
            e.call(0x0089_a760, &args![collidee, hit, 0u32]);
            e.call(0x008c_1c60, &args![pointer]);
        });
    } else {
        let collidee = e.get(impact, ImpactData::pCollidee);
        e.vcall(collidee, 0x144, &args![x87_quiet(damage), 0u32]);
    }
    if e.call(0x0047_7ba0, &args![collidee]).bool() {
        let player: u32 = e.global(PLAYER);
        if e.call(GET_SHOOTER, &args![this]).u32() == player {
            let strength = float_global(e, 0x0102_918c);
            let player: u32 = e.global(PLAYER);
            e.call(0x0096_3eb0, &args![player, 10u32, strength, 0u32]);
        }
    }
    target_alive
}

/// Whether the base object makes `Projectile::ProcessImpacts` skip the
/// impact's effects: `004fd360` holds while neither `00975300` nor
/// `005de080` does.
fn impact_skipped_by_base(e: &mut Engine, this: Ptr<Projectile>) -> bool {
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if !e.call(0x004f_d360, &args![base]).bool() {
        return false;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if e.call(0x0097_5300, &args![base]).bool() {
        return false;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    !e.call(0x005d_e080, &args![base]).bool()
}

/// Notifies VATS (`0082ee10`) when the shooter is the VATS shooter at
/// `011f21e8` hitting an actor, or the one at `011f21ec` hitting an actor
/// whose virtual +0x22C holds while the target was alive.
fn notify_vats_shooters(e: &mut Engine, this: Ptr<Projectile>, collidee: u32, target_alive: bool) {
    let first: u32 = e.global(0x011f_21e8);
    if first != 0
        && first == e.call(GET_SHOOTER, &args![this]).u32()
        && collidee != 0
        && is_actor(e, collidee)
    {
        e.call(0x0082_ee10, &args![]);
    }
    let second: u32 = e.global(0x011f_21ec);
    if second != 0
        && second == e.call(GET_SHOOTER, &args![this]).u32()
        && collidee != 0
        && is_actor(e, collidee)
        && e.vcall(collidee, 0x22C, &args![0u32]).bool()
        && target_alive
    {
        e.call(0x0082_ee10, &args![]);
    }
}
/// Bytes of the `009c20e0` frame below `EBP` that the translation models.
const IMPACT_EFFECTS_FRAME: u32 = 0x2F0;

/// A `float` the game stores and then pushes through the x87 stack.
fn rounded(value: f64) -> f32 {
    x87_quiet(value as f32)
}

/// Plays a sound form at a point (`009c20e0`): handle from
/// `BSAudio::GetSoundHandleByFilename` (flags 0x4102), positioned, played,
/// released.
fn play_sound_at(e: &mut Engine, handle: u32, sound: u32, location: Ptr) {
    let file = e.call(0x0051_1840, &args![sound]).u32();
    let audio = e.call(0x0045_3a70, &args![]).u32();
    e.call(0x00ad_7480, &args![audio, handle, file, 0x4102u32, sound]);
    let x = x87_quiet(e.mem.f32(location.addr()));
    let y = x87_quiet(e.mem.f32(location.addr() + 4));
    let z = x87_quiet(e.mem.f32(location.addr() + 8));
    e.call(0x00ad_8b60, &args![handle, x, y, z]);
    e.call(0x00ad_8830, &args![handle, 0u32]);
    e.call(0x0048_3710, &args![handle]);
}

// Translation incomplete: 009c20e0 - C++ exception unwinding (FS:[0] frame) is not modelled.
// Translated from 009c20e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The impact effects of an impact: nothing for an actor collidee. A
/// collidee gets the action flags 0x80 (projectile) and 0x100 (weapon). The
/// weapon's impact data for the material (`00522ba0`) drives it:
///
/// - Decals (when the data has a texture set, `006733e0`, and the cell has
///   a Havok world): a decal caster (`00622a70` with `fDecalSize`, first
///   made from the data's `00476b70(004a40a0, 004a40c0)` when 0) collects
///   the bodies around the location (`00623090`); each body of the impact's
///   material (by shape key through virtual +0xCC/+0xE0, else `006205a0`)
///   gets a decal (`004a3fe0` on `004519d0`'s decal manager) built from
///   location, normal, size, the data's parameters and colour (`004a4220`
///   bytes over the `double` at `0101e568`), on the body's node; bodies of
///   other kinds need a reference with `004a1060(ref, 1)`. Afterwards the
///   decal size is cleared.
/// - The impact effect (`006890b0`) when the location is within the
///   setting at `011ce26c` of the camera and visible (`004b61d0`) and the
///   data names a model: oriented along the normal, the movement direction
///   or its reflection (`005f36f0` mode 1 or 2).
/// - The actor cause's actor's process (virtual +0xFC) is told of the
///   impact, and the data's two sounds (`0068a810`, `0068a830`) play at the
///   location.
pub fn fn_009c20e0(
    e: &mut Engine,
    this: Ptr<Projectile>,
    collidee: u32,
    location: Ptr,
    normal: Ptr,
    _world_object: u32,
    material: u32,
) {
    let mut visible = true;
    let camera = e.call(0x0052_4c90, &args![]).u32();
    let camera_position = e.call(0x0045_bb80, &args![camera]).u32();
    e.with_stack(IMPACT_EFFECTS_FRAME, |e, frame| {
        let at = |offset: u32| frame.addr() + IMPACT_EFFECTS_FRAME - offset;
        e.call(0x0043_9ef0, &args![location, at(0x1C), camera_position]);
        if e.call(0x0052_5420, &args![]).bool() {
            visible = false;
        } else {
            let distance = e.call(0x0045_7990, &args![at(0x1C)]).f64();
            let limit = e.call(SETTING_VALUE, &args![0x011c_e26cu32]).u32();
            if (e.mem.f32(limit) as f64) < distance {
                visible = false;
            } else {
                let size = float_global(e, 0x0101_e340);
                let camera = e.call(0x0052_4c90, &args![]).u32();
                let [x, y, z] = point_words(e, location.addr());
                if !e.call(0x004b_61d0, &args![x, y, z, camera, size]).bool() {
                    visible = false;
                }
            }
        }
        if collidee != 0 && is_actor(e, collidee) {
            return;
        }
        if collidee != 0 {
            let extra = e.call(0x005d_43c0, &args![collidee]).u32();
            e.call(0x005a_c750, &args![this, extra, 0x80u32]);
            if e.get(this, Projectile::pWeaponSource) != 0 {
                let extra = e.call(0x005d_43c0, &args![collidee]).u32();
                let weapon = e.get(this, Projectile::pWeaponSource);
                e.call(0x005a_c750, &args![weapon, extra, 0x100u32]);
            }
        }
        let weapon = e.get(this, Projectile::pWeaponSource);
        let data = if weapon != 0 {
            e.call(0x0052_2ba0, &args![weapon, material]).u32()
        } else {
            0
        };
        if data == 0 {
            return;
        }
        let texture = e.call(0x0067_33e0, &args![data]).u32();
        if texture != 0 {
            add_impact_decals(
                e,
                frame.addr(),
                this,
                location,
                normal,
                material,
                data,
                texture,
            );
        }
        if visible {
            add_impact_effect(e, frame.addr(), this, location, normal, data);
        }
        let cause = e.vcall(this.addr(), 0x168, &args![]).u32();
        if cause != 0 && e.call(0x0055_9450, &args![cause]).u32() != 0 {
            let actor = e.call(0x0055_9450, &args![cause]).u32();
            if e.call(ACTOR_PROCESS_OBJECT, &args![actor]).u32() != 0 {
                let process = e.call(ACTOR_PROCESS_OBJECT, &args![actor]).u32();
                let value = e.call(0x0058_e790, &args![data]).u32();
                let [x, y, z] = point_words(e, location.addr());
                e.vcall(process, 0xFC, &args![actor, x, y, z, value, 0u32, 0u32]);
            }
        }
        let sound = e.call(0x0068_a810, &args![data]).u32();
        if sound != 0 {
            play_sound_at(e, at(0x26C), sound, location);
        }
        let sound = e.call(0x0068_a830, &args![data]).u32();
        if sound != 0 {
            play_sound_at(e, at(0x278), sound, location);
        }
    });
}

/// The decal part of `009c20e0`.
#[allow(clippy::too_many_arguments)]
fn add_impact_decals(
    e: &mut Engine,
    frame: u32,
    this: Ptr<Projectile>,
    location: Ptr,
    normal: Ptr,
    material: u32,
    data: u32,
    texture: u32,
) {
    let at = |offset: u32| frame + IMPACT_EFFECTS_FRAME - offset;
    let unit_normal = at(0x50);
    let decal_location = at(0x40);
    copy_point(e, unit_normal, normal.addr());
    e.call(0x004a_0c10, &args![unit_normal]);
    copy_point(e, decal_location, location.addr());
    if e.get(this, Projectile::fDecalSize) as f64 == 0.0 {
        let high = e.call(0x004a_40c0, &args![data]).f64() as f32;
        let low = e.call(0x004a_40a0, &args![data]).f64() as f32;
        let size = e.call(0x0047_6b70, &args![low, high]).f64() as f32;
        e.set(this, Projectile::fDecalSize, size);
    }
    let size = x87_quiet(e.get(this, Projectile::fDecalSize));
    let caster = e.call(0x0062_2a70, &args![size, 1u32]).u32();
    e.call(0x0062_33c0, &args![caster, 0u32]);
    e.call(0x004a_1a10, &args![caster, 8u32, 0u32]);
    e.call(0x004a_1a10, &args![caster, 0x1Du32, 0u32]);
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let world = if cell != 0 {
        e.call(0x0045_43c0, &args![cell]).u32()
    } else {
        0
    };
    if world == 0 {
        return;
    }
    e.call(0x0062_31a0, &args![caster, world]);
    let collector = at(0x184);
    e.call(0x0062_d350, &args![collector]);
    e.call(0x0062_3090, &args![caster, location, collector]);
    let bodies = e.call(0x0041_3f40, &args![collector]).u32();
    let count = e.call(0x0072_6070, &args![bodies]).i32();
    let random_scale = e.call(0x004a_4240, &args![]).f64() as f32;
    let range = float_global(e, 0x0101_e570);
    let random = e.call(0x0047_6b70, &args![0.0f32, range]).f64() as f32;
    let random = e.call(0x0040_6cc0, &args![random]).f64();
    let variant = e.call(0x00ec_62c0, &args![random]).u8();
    let mut previous = 0u32;
    for index in 0..count {
        let bodies = e.call(0x0041_3f40, &args![collector]).u32();
        let pair = e.call(0x0062_e150, &args![bodies, index]).u32();
        let body = e.mem.u32(pair + 8);
        if body == 0 {
            continue;
        }
        let mut body_material = material;
        let bodies = e.call(0x0041_3f40, &args![collector]).u32();
        let pair = e.call(0x0062_e150, &args![bodies, index]).u32();
        let key = e.mem.u32(pair + 0xC);
        let node = e.call(0x0055_9450, &args![body]).u32();
        let shape = e.call(0x004a_3a40, &args![node]).u32();
        if shape == 0 {
            continue;
        }
        if key != 0xFFFF_FFFF {
            let container = e.vcall(shape, 0xCC, &args![]).u32();
            if container != 0 {
                body_material = e.vcall(container, 0xE0, &args![key]).u32();
            }
        } else {
            body_material = e.call(0x0062_05a0, &args![shape]).u32();
        }
        if body_material != material || body == previous {
            continue;
        }
        previous = body;
        let shape_kind = e.call(0x0043_b540, &args![body]).u32();
        let wrapped = e.call(0x008c_71b0, &args![at(0x27C), shape_kind]).u32();
        let single = e.call(0x004a_3a20, &args![wrapped]).u32() == 1;
        let mut object = 0u32;
        if single {
            let entity = e.call(0x004b_59f0, &args![body]).u32();
            if entity != 0 {
                let property = e
                    .call(0x00c8_5ef0, &args![at(0x284), entity, 0x011f_4a20u32])
                    .u32();
                object = e.call(0x0055_9450, &args![property]).u32();
                if object != 0 && e.call(0x0096_11e0, &args![object]).u32() != 0 {
                    object = e.call(0x0096_11e0, &args![object]).u32();
                }
            }
        } else {
            object = e.call(0x00c7_fa90, &args![body]).u32();
        }
        let reference = e.call(0x0056_f930, &args![object]).u32();
        if !single && !(reference != 0 && e.call(0x004a_1060, &args![reference, 1u32]).bool()) {
            continue;
        }
        let [x, y, z] = point_words(e, location.addr());
        let tes: u32 = e.global(TES);
        let manager = e.call(0x0045_19d0, &args![tes, x, y, z]).u32();
        let decal = at(0x22C);
        e.call(0x004a_37b0, &args![decal]);
        copy_point(e, decal, decal_location);
        copy_point(e, decal + 0xC, unit_normal);
        let size = x87_quiet(e.get(this, Projectile::fDecalSize));
        e.mem.set_f32(at(0x1F4), size);
        let size = x87_quiet(e.get(this, Projectile::fDecalSize));
        e.mem.set_f32(at(0x1F0), size);
        let depth = e.call(0x008a_ff10, &args![data]).f64() as f32;
        e.mem.set_f32(at(0x1EC), depth);
        e.mem.set_u32(at(0x204), object);
        e.mem.set_u32(at(0x1FC), texture);
        e.mem.set_f32(at(0x1E8), x87_quiet(random_scale));
        e.mem.set_u8(at(0x1BC), variant);
        let value = e.call(0x004a_4140, &args![data]).u8();
        e.mem.set_u8(at(0x1B7), value);
        let value = e.call(0x004a_4180, &args![data]).u8();
        e.mem.set_u8(at(0x1B8), value);
        let value = e.call(0x004a_4100, &args![data]).u8();
        e.mem.set_u8(at(0x1B9), value);
        let value = e.call(0x004a_41c0, &args![data]).f64() as f32;
        e.mem.set_f32(at(0x1E0), value);
        let value = e.call(0x004a_41e0, &args![data]).u8();
        e.mem.set_u8(at(0x1B6), value);
        let value = e.call(0x004a_40e0, &args![data]).f64() as f32;
        e.mem.set_f32(at(0x1D8), value);
        let value = e.call(0x009a_9350, &args![data]).f64() as f32;
        e.mem.set_f32(at(0x1D4), value);
        let scale: f64 = e.global(0x0101_e568);
        let color = e.call(0x004a_4220, &args![data]).u32();
        let red = rounded(((color >> 16) & 0xFF) as f64 / scale);
        let color = e.call(0x004a_4220, &args![data]).u32();
        let green = rounded(((color as u16 as i32 >> 8) & 0xFF) as f64 / scale);
        let color = e.call(0x004a_4220, &args![data]).u32();
        let blue = rounded((color & 0xFF) as f64 / scale);
        let rgb = e
            .call(0x0041_6870, &args![at(0x290), blue, green, red])
            .u32();
        copy_point(e, at(0x1CC), rgb);
        if body_material == 3 {
            e.mem.set_u8(at(0x1B3), 1);
        }
        e.call(0x004a_3fe0, &args![manager, decal, 1u32, 0u32]);
    }
    if count > 0 {
        e.set(this, Projectile::fDecalSize, 0.0);
    }
    e.call(0x0062_d4b0, &args![collector]);
}

/// The impact-effect part of `009c20e0`.
fn add_impact_effect(
    e: &mut Engine,
    frame: u32,
    this: Ptr<Projectile>,
    location: Ptr,
    normal: Ptr,
    data: u32,
) {
    let at = |offset: u32| frame + IMPACT_EFFECTS_FRAME - offset;
    let model = e.vcall(data + 0x18, 0x14, &args![]).u32();
    if model == 0 || e.mem.i8(model) == 0 {
        return;
    }
    let direction = at(0x244);
    e.call(0x0068_15c0, &args![direction]);
    match e.call(0x005f_36f0, &args![data]).u32() {
        1 => {
            let moving = e
                .call(
                    0x004a_0bd0,
                    &args![this.byte_add(MOVEMENT_DIRECTION), at(0x29C)],
                )
                .u32();
            copy_point(e, direction, moving);
            e.call(0x004a_0c10, &args![direction]);
        }
        2 => {
            let incoming = at(0x25C);
            let surface = at(0x250);
            e.call(
                0x004a_0bd0,
                &args![this.byte_add(MOVEMENT_DIRECTION), incoming],
            );
            e.call(0x004a_0c10, &args![incoming]);
            copy_point(e, surface, normal.addr());
            e.call(0x004a_0c10, &args![surface]);
            let dot = e.call(0x004b_6190, &args![surface, incoming]).f64();
            let twice: f64 = e.global(0x0101_1590);
            let factor = rounded(dot * twice);
            let scaled = e
                .call(0x004a_3760, &args![at(0x2A8), factor, surface])
                .u32();
            let reflected = e
                .call(0x0043_9ef0, &args![scaled, at(0x2B4), incoming])
                .u32();
            copy_point(e, direction, reflected);
        }
        _ => {
            copy_point(e, direction, normal.addr());
            e.call(0x004a_0c10, &args![direction]);
        }
    }
    let [lx, ly, lz] = point_words(e, location.addr());
    let [dx, dy, dz] = point_words(e, direction);
    let scale = e.call(0x0050_8100, &args![data]).f64() as f32;
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    e.call(
        0x0068_90b0,
        &args![cell, scale, model, dx, dy, dz, lx, ly, lz, 1.0f32, 7u32, 0u32],
    );
}
// Translated from 009c2c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a pick hit may be taken (cdecl; the projectile word is never
/// read). A hit on an entity is refused when the entity is in the set at
/// `01268444` (`0043b300`) or not of shape type 0x1E, or when the first
/// collision object of the reference's 3D (the player's `00950bb0(0)` for
/// the player) is neither flagged (`009c19e0`) nor without a virtual +0x94.
/// A hit on a shape of type 0x1A or 0x1C is then refused unless the base
/// object's type (`004fd4f0`) is 0x20000, or 0x10000 without `009a7f80`
/// and without flag 0x200 (`009c2e60`).
pub fn fn_009c2c80(
    e: &mut Engine,
    reference: u32,
    collidable: u32,
    _unused_2: u32,
    base_object: u32,
) -> bool {
    let world_object = if collidable != 0 {
        e.call(0x004b_5950, &args![collidable]).u32()
    } else {
        0
    };
    let entity = if world_object != 0 {
        e.call(0x004b_5a80, &args![world_object]).u32()
    } else {
        0
    };
    if entity != 0 {
        if e.call(0x0043_b300, &args![0x0126_8444u32, entity]).bool() {
            return false;
        }
        let kind = e.with_stack(4, |e, info| shape_type(e, entity, info.addr()));
        if kind != 0x1E {
            return false;
        }
        let mut root = if reference != 0 {
            e.vcall(reference, 0x1D0, &args![]).u32()
        } else {
            0
        };
        let player: u32 = e.global(PLAYER);
        if reference == player {
            root = e.call(0x0095_0bb0, &args![player, 0u32]).u32();
        }
        let first = e.call(0x004b_5260, &args![root]).u32();
        let collision = if first != 0 {
            e.call(0x006f_a820, &args![first]).u32()
        } else {
            0
        };
        if collision != 0 {
            let flagged = e.with_stack(4, |e, local| {
                let info = e.call(0x0043_b4f0, &args![collision, local]).u32();
                fn_009c19e0(e, Ptr::new(info))
            });
            if !flagged && e.vcall(collision, 0x94, &args![]).u32() != 0 {
                return false;
            }
        }
    }
    if base_object != 0 && collidable != 0 {
        let shape = e.call(0x0043_b540, &args![collidable]).u32();
        let mut kind = e.with_stack(4, |e, local| {
            let wrapped = e.call(0x008c_71b0, &args![local, shape]).u32();
            e.call(0x0043_b4d0, &args![wrapped]).u32()
        });
        if kind != 0x1A {
            let shape = e.call(0x0043_b540, &args![collidable]).u32();
            kind = e.with_stack(4, |e, local| {
                let wrapped = e.call(0x008c_71b0, &args![local, shape]).u32();
                e.call(0x0043_b4d0, &args![wrapped]).u32()
            });
            if kind != 0x1C {
                return true;
            }
        }
        let form_type = e.call(0x004f_d4f0, &args![base_object]).u32();
        if form_type == 0x20000 {
            return true;
        }
        if form_type != 0x10000 {
            return false;
        }
        if e.call(0x009a_7f80, &args![base_object]).bool() {
            return false;
        }
        return !fn_009c2e60(e, Ptr::new(base_object));
    }
    true
}

// Translated from 009c2e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the base object has flag 0x200 (`004fd420`).
pub fn fn_009c2e60(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x004f_d420, &args![this, 0x200u32]).bool()
}

// Translation incomplete: 009c2e80 - the /GS cookie check (00ec408c) is not modelled.
// Translated from 009c2e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes the body hit at `(x, y, z)` (the impact location) along
/// `(dx, dy, dz)`: for a world object whose entity (`004b59f0`,
/// `004b5a80`) has an object (virtual +0x98) and a motion type
/// (`00517630`) below 4, the base's impulse (`00644930`, through
/// `0062b520(.., entity, 1)`) when nonzero scales the direction (made a
/// Havok vector, `004a3e00`; normalised by `0062aea0` when its `00630b40`
/// length is positive; scaled through `005dbf20`/`00627920`), applied at
/// the location through `0081f1e0` on the entity's `004ae750`, between two
/// `00483710` calls on the object.
#[allow(clippy::too_many_arguments)]
pub fn fn_009c2e80(
    e: &mut Engine,
    this: Ptr<Projectile>,
    x: u32,
    y: u32,
    z: u32,
    dx: u32,
    dy: u32,
    dz: u32,
    world_object: u32,
) {
    if world_object == 0 {
        return;
    }
    let inner = e.call(0x004b_59f0, &args![world_object]).u32();
    let entity = e.call(0x004b_5a80, &args![inner]).u32();
    let object = if entity != 0 {
        e.vcall(entity, 0x98, &args![]).u32()
    } else {
        0
    };
    if object == 0 || e.call(0x0051_7630, &args![entity]).i32() >= 4 {
        return;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let impulse = e.call(0x0064_4930, &args![base]).f64() as f32;
    let impulse = e
        .call(0x0062_b520, &args![x87_quiet(impulse), entity, 1u32])
        .f64() as f32;
    if impulse as f64 == 0.0 {
        return;
    }
    e.with_stack(0x40 + 2 * NI_POINT3_SIZE, |e, locals| {
        let push = locals.addr();
        let point = push + 0x10;
        let length = point + 0x10;
        let factor = length + 0x10;
        let direction = factor + 0x10;
        let location = direction + NI_POINT3_SIZE;
        for (address, words) in [(direction, [dx, dy, dz]), (location, [x, y, z])] {
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(address + 4 * i as u32, *word);
            }
        }
        e.call(0x0068_15c0, &args![push]);
        e.call(0x004a_3e00, &args![push, direction]);
        e.call(0x0068_15c0, &args![point]);
        e.call(0x004a_3e00, &args![point, location]);
        let squared = e.call(0x0063_0b40, &args![push, length]).u32();
        if e.call(0x0045_86d0, &args![squared]).f64() > 0.0 {
            e.call(0x0062_aea0, &args![push]);
        }
        e.call(0x005d_bf20, &args![factor, x87_quiet(impulse)]);
        e.call(0x0062_7920, &args![push, factor]);
        e.call(0x0048_3710, &args![object]);
        let target = e.call(0x004a_e750, &args![entity]).u32();
        e.call(0x0081_f1e0, &args![target, push, point]);
        e.call(0x0048_3710, &args![object]);
    });
}

// Translated from 009c2ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a base object with `009baf30` whose `006ab360` object has
/// `0048cee0`: a shooter that is not dead (virtual +0x22C with 0) nor
/// virtual +0x230, and not an actor with `008a8870`, gets `009bb6d0`
/// (cdecl, shooter and projectile).
pub fn fn_009c2ff0(e: &mut Engine, this: Ptr<Projectile>) {
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if !e.call(0x009b_af30, &args![base]).bool() {
        return;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let object = e.call(0x006a_b360, &args![base]).u32();
    if e.call(0x0048_cee0, &args![object]).u32() == 0 {
        return;
    }
    let shooter = e.call(GET_SHOOTER, &args![this]).u32();
    let actor = if shooter != 0 && is_actor(e, shooter) {
        shooter
    } else {
        0
    };
    if shooter == 0
        || e.vcall(shooter, 0x22C, &args![0u32]).bool()
        || e.vcall(shooter, 0x230, &args![]).bool()
    {
        return;
    }
    if actor != 0 && e.call(0x008a_8870, &args![actor]).bool() {
        return;
    }
    e.call(0x009b_b6d0, &args![shooter, this]);
}

// Translated from 009c30d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::CombatHit` (Xbox PDB): an actor shooter reports the hit on
/// `target` (`Actor::CombatHit(target, flag, projectile, 0)`), unless it is
/// not aiming (virtual +0x360), the projectile has flag 0x80000 and the
/// shooter is not in combat with the target (`Actor::IsInCombatWithActor`).
pub fn projectile_combat_hit(e: &mut Engine, this: Ptr<Projectile>, target: u32, flag: u8) {
    let shooter = e.get(this, Projectile::pShooter);
    if shooter == 0 || !is_actor(e, shooter) {
        return;
    }
    let mut report = true;
    let shooter = e.get(this, Projectile::pShooter);
    if !e.vcall(shooter, 0x360, &args![]).bool()
        && test_flags(e, this, 0x80000)
        && !e.call(0x008b_c700, &args![shooter, target]).bool()
    {
        report = false;
    }
    if report {
        e.call(
            0x0089_9cb0,
            &args![shooter, target, flag as u32, this, 0u32],
        );
    }
}
/// `FISTP qword` with the rounding control set to truncate: the integer part,
/// or the x87 "integer indefinite" (`i64::MIN`) for NaN and values outside
/// the 64-bit range.
fn truncate_to_i64(value: f64) -> i64 {
    if value.is_nan()
        || value >= 9_223_372_036_854_775_808.0
        || value < -9_223_372_036_854_775_808.0
    {
        i64::MIN
    } else {
        value.trunc() as i64
    }
}

// Translated from 009c3190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::CheckExplosion` (Xbox PDB): for a base object that explodes
/// (`004fd360`) with an explosion (`004fd3c0`) and a projectile without
/// flag 0x200.
///
/// Timed projectiles (`00975300`): an untriggered mine (timer `FLT_MAX`,
/// nonzero `0045cd80`) checks its trigger (`009c39e0`) and stops its
/// countdown sound when not triggered; otherwise the timer runs down by
/// `delta` and explodes at 0, while the blink timer runs down and, at 0,
/// sets the light (`004aae30` on the 3D with `01030e48`,
/// `TESObjectREFR::SetLight`) and restarts from the setting at `011cee78`,
/// or interpolated towards `011ce2cc` below the setting at `011ce9e4`, with
/// `Projectile::StartCountdownSound`. Others explode with flag 0x100000, or
/// when the base has no `005de080` and the impact list is not empty.
///
/// The explosion is placed at the first impact (rotation from its normal,
/// `BSUtilities::GetRotationZToVectorTranspose`, after a `004a4240` angle)
/// when the setting at `011d0fe4` is positive or flag 0x2000 is set, else
/// at the projectile (`009c6380`) with its 3D's rotation (or the global
/// matrix at `011a9448`); it is made by `009ac9c0` with the shooter (none
/// with flag 0x400) and the cell, oriented by the impact normal for type
/// 0x10000 (`0089d640`) and given the weapon (`00476c70`). Flag 0x8000
/// runs virtual +0x314 in VATS mode 4 and `009bc8f0`. An explosion with a
/// positive `006a78f0` and flag 0x400 kills an actor desired target
/// (`Actor::Kill` by the actor shooter, a `MiscStatManager` stat 0xE and
/// the victim's process virtual +0x72C when the shooter aims) and lets the
/// player's process hear it (virtual +0xFC).
pub fn projectile_check_explosion(e: &mut Engine, this: Ptr<Projectile>, delta: f32) {
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if !e.call(0x004f_d360, &args![base]).bool() {
        return;
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if e.call(0x004f_d3c0, &args![base]).u32() == 0 || test_flags(e, this, 0x200) {
        return;
    }
    let mut explode = false;
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    if e.call(0x0097_5300, &args![base]).bool() {
        let unlimited: f64 = e.global(0x0102_31b0);
        let mut waiting = false;
        if e.get(this, Projectile::fExplosionTimer) as f64 == unlimited {
            let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
            if e.call(0x0045_cd80, &args![base]).f64() != 0.0 && !fn_009c39e0(e, this) {
                waiting = true;
            }
        }
        if waiting {
            e.call(0x009c_4f90, &args![this]);
        } else {
            let timer = e.get(this, Projectile::fExplosionTimer);
            let timer = (timer as f64 - delta as f64) as f32;
            e.set(this, Projectile::fExplosionTimer, timer);
            if e.get(this, Projectile::fExplosionTimer) as f64 <= 0.0 {
                explode = true;
            } else {
                let blink = e.get(this, Projectile::fBlinkTimer);
                let blink = (blink as f64 - delta as f64) as f32;
                e.set(this, Projectile::fBlinkTimer, blink);
                if e.get(this, Projectile::fBlinkTimer) as f64 <= 0.0 {
                    blink_countdown(e, this);
                }
            }
        }
    } else if test_flags(e, this, 0x100000) {
        explode = true;
    } else {
        let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
        if !e.call(0x005d_e080, &args![base]).bool()
            && !e
                .call(0x0082_56d0, &args![this.byte_add(IMPACT_LIST)])
                .bool()
        {
            explode = true;
        }
    }
    if explode {
        explode_projectile(e, this);
    }
}

/// The blink step of `Projectile::CheckExplosion`.
fn blink_countdown(e: &mut Engine, this: Ptr<Projectile>) {
    let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
    let light = e.call(0x004a_ae30, &args![root, 0x0103_0e48u32]).u32();
    if light != 0 {
        let lit = e.call(0x0045_6610, &args![light]).u8();
        let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
        e.call(0x0057_26e0, &args![this, root, lit as u32]);
    }
    let threshold = e.call(SETTING_VALUE, &args![0x011c_e9e4u32]).u32();
    let timer = e.get(this, Projectile::fExplosionTimer) as f64;
    if (e.mem.f32(threshold) as f64) < timer {
        let value = setting_float(e, 0x011c_ee78);
        e.set(this, Projectile::fBlinkTimer, value);
    } else {
        let fastest = e.call(SETTING_VALUE, &args![0x011c_e2ccu32]).u32();
        let threshold = e.call(SETTING_VALUE, &args![0x011c_e9e4u32]).u32();
        let ratio = e.get(this, Projectile::fExplosionTimer) as f64 / e.mem.f32(threshold) as f64;
        let slowest = e.call(SETTING_VALUE, &args![0x011c_ee78u32]).u32();
        let fastest_again = e.call(SETTING_VALUE, &args![0x011c_e2ccu32]).u32();
        let value = (e.mem.f32(slowest) as f64 - e.mem.f32(fastest_again) as f64) * ratio
            + e.mem.f32(fastest) as f64;
        e.set(this, Projectile::fBlinkTimer, value as f32);
    }
    e.call(0x009c_4ec0, &args![this]);
}

/// The explosion of `Projectile::CheckExplosion`.
fn explode_projectile(e: &mut Engine, this: Ptr<Projectile>) {
    e.with_stack(NI_POINT3_SIZE + 3 * 0x24, |e, locals| {
        let position = locals.addr();
        let rotation = position + NI_POINT3_SIZE;
        let heading = rotation + 0x24;
        let normal_rotation = heading + 0x24;
        e.call(0x0068_15c0, &args![position]);
        e.call(0x0068_15c0, &args![rotation]);
        let placed_at_impact = {
            let count = e.call(0x0043_d4d0, &args![0x011d_0fe4u32]).u32();
            let use_impact = e.mem.i32(count) > 0 || test_flags(e, this, 0x2000);
            use_impact
                && !e
                    .call(0x0082_56d0, &args![this.byte_add(IMPACT_LIST)])
                    .bool()
        };
        if placed_at_impact {
            let item = e
                .call(0x0068_15c0, &args![this.byte_add(IMPACT_LIST)])
                .u32();
            let impact = e.mem.u32(item);
            let angle = e.call(0x004a_4240, &args![]).f64() as f32;
            e.call(0x0068_15c0, &args![heading]);
            e.call(0x004a_0c90, &args![heading, x87_quiet(angle)]);
            let [nx, ny, nz] = point_words(e, impact + 0x10);
            let matrix = e
                .call(0x00c4_b8a0, &args![normal_rotation, nx, ny, nz])
                .u32();
            for offset in (0..0x24).step_by(4) {
                let word = e.mem.u32(matrix + offset);
                e.mem.set_u32(rotation + offset, word);
            }
            copy_point(e, position, impact + 4);
        } else {
            let current = e.vcall(this.addr(), 0x1F4, &args![]).u32();
            copy_point(e, position, current);
            let source = if e.vcall(this.addr(), 0x1D0, &args![]).u32() != 0 {
                let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
                e.call(0x0046_1130, &args![root]).u32()
            } else {
                0x011a_9448
            };
            for offset in (0..0x24).step_by(4) {
                let word = e.mem.u32(source + offset);
                e.mem.set_u32(rotation + offset, word);
            }
            let root = e.vcall(this.addr(), 0x1D0, &args![]).u32();
            e.call(0x009c_6380, &args![this, root, position]);
        }
        let cell = e.call(0x008d_6f30, &args![this]).u32();
        let owner = if test_flags(e, this, 0x400) {
            0
        } else {
            e.call(GET_SHOOTER, &args![this]).u32()
        };
        let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
        let form = e.call(0x004f_d3c0, &args![base]).u32();
        let mut words = vec![form, owner, 0, cell];
        words.extend_from_slice(&point_words(e, position));
        for offset in (0..0x24).step_by(4) {
            words.push(e.mem.u32(rotation + offset));
        }
        let explosion = e.call(0x009a_c9c0, &words).u32();
        if explosion != 0
            && !e
                .call(0x0082_56d0, &args![this.byte_add(IMPACT_LIST)])
                .bool()
        {
            let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
            if e.call(0x004f_d4f0, &args![base]).u32() == 0x10000 {
                let item = e
                    .call(0x0068_15c0, &args![this.byte_add(IMPACT_LIST)])
                    .u32();
                let impact = e.mem.u32(item);
                let [nx, ny, nz] = point_words(e, impact + 0x10);
                e.call(0x0089_d640, &args![explosion, nx, ny, nz]);
            }
        }
        if explosion != 0 && e.call(GET_WEAPON, &args![this]).u32() != 0 {
            let weapon = e.call(GET_WEAPON, &args![this]).u32();
            e.call(0x0047_6c70, &args![explosion, weapon]);
        }
    });
    if test_flags(e, this, 0x8000) {
        if e.call(0x0044_ddc0, &args![VATS]).u32() == 4 {
            e.vcall(this.addr(), 0x314, &args![]);
        }
        fn_009bc8f0(e, this);
    }
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let form = e.call(0x004f_d3c0, &args![base]).u32();
    if !(e.call(0x006a_78f0, &args![form]).f64() > 0.0) || !test_flags(e, this, 0x400) {
        return;
    }
    if e.call(GET_DESIRED_TARGET, &args![this]).u32() == 0 {
        return;
    }
    let target = e.call(GET_DESIRED_TARGET, &args![this]).u32();
    if !is_actor(e, target) {
        return;
    }
    let victim = e.call(GET_DESIRED_TARGET, &args![this]).u32();
    let mut killer = 0u32;
    if e.call(GET_SHOOTER, &args![this]).u32() != 0 {
        let shooter = e.call(GET_SHOOTER, &args![this]).u32();
        if is_actor(e, shooter) {
            killer = e.call(GET_SHOOTER, &args![this]).u32();
        }
    }
    if e.vcall(killer, 0x360, &args![]).bool() {
        e.call(0x004d_5c60, &args![0xEu32]);
        if e.call(ACTOR_PROCESS_OBJECT, &args![victim]).u32() != 0 {
            let process = e.call(ACTOR_PROCESS_OBJECT, &args![victim]).u32();
            let distance = float_global(e, INITIAL_SOUND_DISTANCE);
            let player: u32 = e.global(PLAYER);
            e.vcall(process, 0x72C, &args![player, distance]);
        }
    }
    e.call(0x0089_d900, &args![victim, killer, 0.0f32]);
    let player: u32 = e.global(PLAYER);
    let process = e.call(ACTOR_PROCESS_OBJECT, &args![player]).u32();
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let form = e.call(0x004f_d3c0, &args![base]).u32();
    let level = e.call(0x004f_b810, &args![form]).u32();
    let position = e.vcall(victim, 0x1F4, &args![]).u32();
    let [x, y, z] = point_words(e, position);
    let player: u32 = e.global(PLAYER);
    e.vcall(process, 0xFC, &args![player, x, y, z, level, 2u32, victim]);
}

// Translated from 009c3930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::GetMineReactsToTarget` (Xbox PDB): whether a mine reacts to
/// `target`: never to its own shooter unless the setting at `011cee48` is
/// positive, never to a dead target (virtual +0x22C with 0) or a member of
/// `faction` (`Actor::GetFactionRank`), and otherwise when `owner_actor`
/// is absent or its fight reaction to the target
/// (`Actor::GetFactionFightReaction`) is 0 or 1.
pub fn projectile_get_mine_reacts_to_target(
    e: &mut Engine,
    this: Ptr<Projectile>,
    target: u32,
    owner_actor: u32,
    faction: u32,
) -> bool {
    let count = e.call(0x0043_d4d0, &args![0x011c_ee48u32]).u32();
    let friendly_fire = e.mem.i32(count) > 0;
    if !friendly_fire && target == e.call(GET_SHOOTER, &args![this]).u32() {
        return false;
    }
    if e.vcall(target, 0x22C, &args![0u32]).bool() {
        return false;
    }
    if faction != 0 && e.call(0x008b_8290, &args![target, faction, 1u32]).i32() != -1 {
        return false;
    }
    let reaction = if owner_actor != 0 {
        e.with_stack(4, |e, flag| {
            e.call(0x008b_87a0, &args![owner_actor, target, flag]).u32()
        })
    } else {
        0
    };
    reaction == 0 || reaction == 1
}

/// The squared distance between the projectile and `other` (`00439ef0` of
/// their positions, `004a7290`), rounded to `float`.
fn squared_distance_to(e: &mut Engine, this: Ptr<Projectile>, other: u32) -> f32 {
    e.with_stack(NI_POINT3_SIZE, |e, difference| {
        let own = e.vcall(this.addr(), 0x1F4, &args![]).u32();
        let theirs = e.vcall(other, 0x1F4, &args![]).u32();
        let offset = e.call(0x0043_9ef0, &args![theirs, difference, own]).u32();
        e.call(0x004a_7290, &args![offset]).f64() as f32
    })
}

// Translated from 009c39e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a mine is triggered: an actor of the high process list
/// (`00717e50`, `005be5c0`, `00968670`) that is alive and active (no
/// `0087f390`, virtual +0x1A0 with 0, `008ace90`) within the base's
/// trigger radius (`0045cd80`, scaled by the setting at `011cf7e0` outside
/// cells with `00425fd0`) to which the mine reacts
/// (`Projectile::GetMineReactsToTarget` with the actor shooter, or the
/// owning faction, form type 8, of `TESObjectREFR::GetOwner`). Without one
/// and without flag 0x4000, the player in range rolls entry point 4 against
/// `00487f50() % 100` (chance from the `float` at `01016410`); a failed
/// roll sets flag 0x4000. A trigger sets the explosion timer: the setting
/// at `011cf7ac`, plus `actor value 0x23 * 00644790(base) / double at
/// 01017a40` for a creature (virtual +0x218) or the player.
pub fn fn_009c39e0(e: &mut Engine, this: Ptr<Projectile>) -> bool {
    let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
    let mut radius = e.call(0x0045_cd80, &args![base]).f64() as f32;
    if e.call(0x008d_6f30, &args![this]).u32() != 0 {
        let cell = e.call(0x008d_6f30, &args![this]).u32();
        if !e.call(0x0042_5fd0, &args![cell]).bool() {
            let scale = e.call(SETTING_VALUE, &args![0x011c_f7e0u32]).u32();
            radius = (radius as f64 * e.mem.f32(scale) as f64) as f32;
        }
    }
    let radius_squared = (radius as f64 * radius as f64) as f32;
    let mut candidate = 0u32;
    let mut triggered = false;
    let mut owner_actor = 0u32;
    let mut faction = 0u32;
    let shooter = e.get(this, Projectile::pShooter);
    if shooter != 0 && is_actor(e, shooter) {
        owner_actor = e.get(this, Projectile::pShooter);
    } else {
        let owner = e.call(0x0056_7790, &args![this]).u32();
        if owner != 0 && e.call(0x0040_1170, &args![owner]).u32() == 8 {
            faction = owner;
        }
    }
    let mut index = 0u32;
    loop {
        let list = e.call(0x0071_7e50, &args![PROCESS_LISTS]).u32();
        if index >= e.call(0x005b_e5c0, &args![list, 0u32]).u32() {
            break;
        }
        let list = e.call(0x0071_7e50, &args![PROCESS_LISTS]).u32();
        candidate = e.call(0x0096_8670, &args![list, index]).u32();
        index += 1;
        let actor = if candidate != 0 && is_actor(e, candidate) {
            candidate
        } else {
            0
        };
        if actor == 0
            || e.call(0x0087_f390, &args![actor]).bool()
            || e.vcall(actor, 0x1A0, &args![0u32]).bool()
            || e.call(0x008a_ce90, &args![actor]).bool()
        {
            continue;
        }
        let distance = squared_distance_to(e, this, candidate);
        if (radius_squared as f64) > distance as f64
            && projectile_get_mine_reacts_to_target(e, this, actor, owner_actor, faction)
        {
            triggered = true;
            break;
        }
    }
    if !test_flags(e, this, 0x4000) && !triggered {
        candidate = e.global(PLAYER);
        let distance = squared_distance_to(e, this, candidate);
        let player: u32 = e.global(PLAYER);
        if (radius_squared as f64) > distance as f64
            && projectile_get_mine_reacts_to_target(e, this, player, owner_actor, faction)
        {
            let chance = float_global(e, 0x0101_6410);
            let chance = e.with_stack(4, |e, value| {
                e.mem.set_f32(value.addr(), chance);
                let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
                let player: u32 = e.global(PLAYER);
                e.call(0x005e_58f0, &args![4u32, player, base, value]);
                e.mem.f32(value.addr())
            });
            let roll = e.call(0x0048_7f50, &args![]).u32() % 100;
            if roll < truncate_to_i64(chance as f64) as u32 {
                triggered = true;
            } else {
                set_flags(e, this, 0x4000, true);
            }
        }
    }
    if triggered && candidate != 0 {
        let player: u32 = e.global(PLAYER);
        if e.vcall(candidate, 0x218, &args![]).bool() || candidate == player {
            let offset = e.call(SETTING_VALUE, &args![0x011c_f7acu32]).u32();
            let base = e.call(GET_BASE_OBJECT, &args![this]).u32();
            let per_point = e.call(0x0064_4790, &args![base]).f64();
            let skill = e.call(0x0066_ef20, &args![candidate + 0xA4, 0x23u32]).i32();
            let divisor: f64 = e.global(0x0101_7a40);
            let timer = (skill as f64 * per_point) / divisor + e.mem.f32(offset) as f64;
            e.set(this, Projectile::fExplosionTimer, timer as f32);
        } else {
            let timer = setting_float(e, 0x011c_f7ac);
            e.set(this, Projectile::fExplosionTimer, timer);
        }
    }
    triggered
}

#[cfg(test)]
mod tests {
    use super::*;

    const CALLEES: &[u32] = &[
        0x0040_1030,
        0x0040_1170,
        0x0040_3e20,
        0x0040_4010,
        0x0040_6cc0,
        0x0040_7860,
        0x0040_8840,
        0x0040_8d60,
        0x0040_8da0,
        0x0040_ebd0,
        0x0040_fbe0,
        0x0041_3f40,
        0x0041_4430,
        0x0041_6870,
        0x0041_81e0,
        0x0041_8900,
        0x0041_a250,
        0x0042_0ba0,
        0x0042_26e0,
        0x0042_5fd0,
        0x0042_ce10,
        0x0042_f730,
        0x0042_f760,
        0x0042_f780,
        0x0043_0830,
        0x0043_7b90,
        0x0043_7c10,
        0x0043_90c0,
        0x0043_9180,
        0x0043_9e90,
        0x0043_9ef0,
        0x0043_9f50,
        0x0043_b1b0,
        0x0043_b300,
        0x0043_b4a0,
        0x0043_b4d0,
        0x0043_b4f0,
        0x0043_b540,
        0x0043_d410,
        0x0043_d4d0,
        0x0043_f8d0,
        0x0043_fa80,
        0x0043_fcd0,
        0x0044_0460,
        0x0044_0490,
        0x0044_1110,
        0x0044_4d00,
        0x0044_59e0,
        0x0044_6390,
        0x0044_ddc0,
        0x0044_edb0,
        0x0045_19d0,
        0x0045_3470,
        0x0045_3700,
        0x0045_37b0,
        0x0045_3a70,
        0x0045_43c0,
        0x0045_6610,
        0x0045_7880,
        0x0045_7990,
        0x0045_85f0,
        0x0045_8620,
        0x0045_86d0,
        0x0045_87f0,
        0x0045_8e20,
        0x0045_bb20,
        0x0045_bb80,
        0x0045_cd60,
        0x0045_cd80,
        0x0045_cec0,
        0x0046_0140,
        0x0046_10f0,
        0x0046_1130,
        0x0046_9860,
        0x0046_ffb0,
        0x0047_4a80,
        0x0047_4cb0,
        0x0047_53d0,
        0x0047_6a80,
        0x0047_6ab0,
        0x0047_6b70,
        0x0047_6c70,
        0x0047_6c90,
        0x0047_7ba0,
        0x0048_3710,
        0x0048_46a0,
        0x0048_4ab0,
        0x0048_73d0,
        0x0048_7f50,
        0x0048_cee0,
        0x0048_ebc0,
        0x0049_3bb0,
        0x0049_41c0,
        0x004a_0bd0,
        0x004a_0c10,
        0x004a_0c90,
        0x004a_1060,
        0x004a_1a10,
        0x004a_3760,
        0x004a_37b0,
        0x004a_3970,
        0x004a_39f0,
        0x004a_3a20,
        0x004a_3a40,
        0x004a_3a70,
        0x004a_3bc0,
        0x004a_3c20,
        0x004a_3da0,
        0x004a_3e00,
        0x004a_3f10,
        0x004a_3f70,
        0x004a_3fb0,
        0x004a_3fe0,
        0x004a_40a0,
        0x004a_40c0,
        0x004a_40e0,
        0x004a_4100,
        0x004a_4140,
        0x004a_4180,
        0x004a_41c0,
        0x004a_41e0,
        0x004a_4220,
        0x004a_4240,
        0x004a_46b0,
        0x004a_7290,
        0x004a_8b00,
        0x004a_ae30,
        0x004a_d010,
        0x004a_e750,
        0x004b_29b0,
        0x004b_3890,
        0x004b_4500,
        0x004b_4d50,
        0x004b_5260,
        0x004b_54c0,
        0x004b_5820,
        0x004b_5950,
        0x004b_5980,
        0x004b_59d0,
        0x004b_59f0,
        0x004b_5a80,
        0x004b_6190,
        0x004b_61d0,
        0x004b_6ae0,
        0x004b_6b60,
        0x004b_6c20,
        0x004b_ae00,
        0x004b_cdb0,
        0x004b_cf60,
        0x004b_da70,
        0x004b_df00,
        0x004c_0bf0,
        0x004d_1360,
        0x004d_5c60,
        0x004e_8510,
        0x004f_b810,
        0x004f_d360,
        0x004f_d3c0,
        0x004f_d3e0,
        0x004f_d400,
        0x004f_d420,
        0x004f_d4f0,
        0x0050_0a20,
        0x0050_8070,
        0x0050_8100,
        0x0051_1840,
        0x0051_7630,
        0x0052_2ba0,
        0x0052_4ac0,
        0x0052_4c90,
        0x0052_5420,
        0x0052_5430,
        0x0052_5b20,
        0x0054_1c80,
        0x0054_1cb0,
        0x0054_71e0,
        0x0054_8230,
        0x0055_32a0,
        0x0055_3300,
        0x0055_3b90,
        0x0055_9450,
        0x0055_af90,
        0x0055_d520,
        0x0056_1800,
        0x0056_20e0,
        0x0056_4db0,
        0x0056_7790,
        0x0056_7ad0,
        0x0056_e950,
        0x0056_f930,
        0x0057_02e0,
        0x0057_26e0,
        0x0057_4400,
        0x0057_5690,
        0x0057_5770,
        0x0057_57a0,
        0x0057_57d0,
        0x0057_5830,
        0x0057_6d30,
        0x0057_8060,
        0x0057_8170,
        0x0057_84b0,
        0x0057_b1d0,
        0x0058_6130,
        0x0058_db10,
        0x0058_e790,
        0x0059_ce80,
        0x005a_c750,
        0x005a_e3d0,
        0x005b_5e40,
        0x005b_e5c0,
        0x005d_43c0,
        0x005d_be10,
        0x005d_be60,
        0x005d_bf20,
        0x005d_e080,
        0x005d_e0a0,
        0x005e_58f0,
        0x005f_36f0,
        0x005f_a310,
        0x0061_a2d0,
        0x0062_05a0,
        0x0062_0b80,
        0x0062_1ad0,
        0x0062_2a70,
        0x0062_3090,
        0x0062_31a0,
        0x0062_33c0,
        0x0062_4070,
        0x0062_6660,
        0x0062_7920,
        0x0062_86d0,
        0x0062_aea0,
        0x0062_b520,
        0x0062_b8d0,
        0x0062_d350,
        0x0062_d4b0,
        0x0062_e150,
        0x0063_0b40,
        0x0063_1100,
        0x0063_2ce0,
        0x0063_3c90,
        0x0063_f7b0,
        0x0064_4790,
        0x0064_4930,
        0x0064_5c20,
        0x0066_3b60,
        0x0066_b0d0,
        0x0066_e010,
        0x0066_ef20,
        0x0067_33e0,
        0x0068_15c0,
        0x0068_38b0,
        0x0068_90b0,
        0x0068_a7d0,
        0x0068_a810,
        0x0068_a830,
        0x0069_6c30,
        0x006a_78f0,
        0x006a_9540,
        0x006a_b360,
        0x006c_a4e0,
        0x006d_2c20,
        0x006f_a820,
        0x0070_c440,
        0x0070_ec90,
        0x0071_6440,
        0x0071_7e50,
        0x0072_6070,
        0x007a_f430,
        0x007f_68a0,
        0x007f_6a20,
        0x0080_0190,
        0x0080_fb00,
        0x0081_0510,
        0x0081_0530,
        0x0081_0550,
        0x0081_0570,
        0x0081_0590,
        0x0081_05c0,
        0x0081_05e0,
        0x0081_0610,
        0x0081_0640,
        0x0081_0660,
        0x0081_26e0,
        0x0081_2870,
        0x0081_f1e0,
        0x0082_1640,
        0x0082_56d0,
        0x0082_5c00,
        0x0082_ee10,
        0x0084_5cc0,
        0x0084_d030,
        0x0084_e3a0,
        0x0085_7230,
        0x0086_48e0,
        0x0086_4980,
        0x0086_5df0,
        0x0086_5e50,
        0x0087_4480,
        0x0087_b760,
        0x0087_f390,
        0x0088_5bd0,
        0x0088_5d70,
        0x0088_8960,
        0x0089_9cb0,
        0x0089_a760,
        0x0089_d640,
        0x0089_d900,
        0x008a_8870,
        0x008a_88b0,
        0x008a_c890,
        0x008a_ce90,
        0x008a_ff10,
        0x008b_3d70,
        0x008b_3ef0,
        0x008b_8290,
        0x008b_87a0,
        0x008b_bc10,
        0x008b_c700,
        0x008c_1c30,
        0x008c_1c60,
        0x008c_71b0,
        0x008c_7aa0,
        0x008c_dd90,
        0x008d_6f30,
        0x008d_7510,
        0x008d_8520,
        0x008d_8b80,
        0x0090_5330,
        0x0092_eb50,
        0x0092_ec40,
        0x0092_ed60,
        0x0092_f260,
        0x0093_06d0,
        0x0093_1d70,
        0x0093_1ed0,
        0x0095_0bb0,
        0x0096_11e0,
        0x0096_3eb0,
        0x0096_5620,
        0x0096_6980,
        0x0096_69c0,
        0x0096_6a00,
        0x0096_8670,
        0x0096_8f90,
        0x0096_a2d0,
        0x0096_d450,
        0x0097_5300,
        0x0097_7770,
        0x0097_9070,
        0x0097_9260,
        0x0097_95c0,
        0x0097_f720,
        0x009a_51e0,
        0x009a_52f0,
        0x009a_6080,
        0x009a_7c40,
        0x009a_7f80,
        0x009a_9350,
        0x009a_b090,
        0x009a_b9a0,
        0x009a_be40,
        0x009a_c9c0,
        0x009b_2c90,
        0x009b_2db0,
        0x009b_2e30,
        0x009b_31f0,
        0x009b_3400,
        0x009b_3490,
        0x009b_4020,
        0x009b_4440,
        0x009b_4d90,
        0x009b_5650,
        0x009b_7010,
        0x009b_7af0,
        0x009b_88a0,
        0x009b_88c0,
        0x009b_88e0,
        0x009b_af30,
        0x009b_b6d0,
        0x009c_3d90,
        0x009c_3e10,
        0x009c_3e60,
        0x009c_40c0,
        0x009c_4e60,
        0x009c_4ec0,
        0x009c_4f90,
        0x009c_5af0,
        0x009c_5ba0,
        0x009c_5ee0,
        0x009c_6380,
        0x009c_6440,
        0x009c_71c0,
        0x009c_8b90,
        0x009c_8c40,
        0x009c_8cc0,
        0x009c_8d60,
        0x00a5_82f0,
        0x00a5_9400,
        0x00a5_9660,
        0x00a5_a040,
        0x00a6_d2d0,
        0x00a8_6bf0,
        0x00aa_13e0,
        0x00ad_7480,
        0x00ad_8830,
        0x00ad_8b60,
        0x00ad_8da0,
        0x00ad_8f20,
        0x00b6_b910,
        0x00b6_bb30,
        0x00c4_b8a0,
        0x00c6_6230,
        0x00c6_6be0,
        0x00c6_9ee0,
        0x00c6_9f50,
        0x00c6_a040,
        0x00c6_a0b0,
        0x00c6_a120,
        0x00c6_a190,
        0x00c6_a350,
        0x00c6_a3d0,
        0x00c6_a4c0,
        0x00c6_d7a0,
        0x00c6_da50,
        0x00c6_de10,
        0x00c7_0c90,
        0x00c7_fa90,
        0x00c8_4f10,
        0x00c8_5ef0,
        0x00c9_0510,
        0x00ec_62c0,
    ];
    /// Fake addresses every vtable slot of a test object points at
    /// (`SLOT_BASE + slot`), each a double returning 0.
    const SLOT_BASE: u32 = 0x7F00_0000;
    /// A double returning 1 (`IsActor` and other `bool` slots).
    const RETURNS_TRUE: u32 = 0x7E00_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x0010_0000);
        e.map(0x011A_0000, 0x0010_0000);
        e.map(0x0126_0000, 0x0001_0000);
        for &address in CALLEES {
            e.register(address, |_, _| Ret::default());
        }
        for slot in (0..0x800).step_by(4) {
            e.register(SLOT_BASE + slot, |_, _| Ret::default());
        }
        e.register(RETURNS_TRUE, |_, _| 1u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(BOOL_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(0x0043_d4d0, |_, a| (a[0] + 4).into_ret());
        e.register(TEST_FLAGS, |e, a| {
            (e.mem.u32(a[0] + 0xC8) & a[1] != 0).into_ret()
        });
        e.register(GET_WEAPON, |e, a| e.mem.u32(a[0] + 0xF8).into_ret());
        e.register(GET_SHOOTER, |e, a| e.mem.u32(a[0] + 0xFC).into_ret());
        e.register(GET_DESIRED_TARGET, |e, a| {
            e.mem.u32(a[0] + 0x100).into_ret()
        });
        e.register(GET_FORM_ID, |e, a| e.mem.u32(a[0] + 0xC).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn vtable(e: &mut Engine) -> u32 {
        let table = e.mem.alloc(0x800);
        for slot in (0..0x800).step_by(4) {
            e.mem.set_u32(table + slot, SLOT_BASE + slot);
        }
        table
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let table = vtable(e);
        let object = e.mem.alloc(size.max(4));
        e.mem.set_u32(object, table);
        object
    }

    fn actor(e: &mut Engine) -> u32 {
        let actor = object(e, 0x200);
        set_slot(e, actor, 0x100, RETURNS_TRUE);
        actor
    }

    fn projectile(e: &mut Engine) -> Ptr<Projectile> {
        Ptr::new(object(e, 0x160))
    }

    fn set_slot(e: &mut Engine, object: u32, slot: u32, target: u32) {
        let table = e.mem.u32(object);
        e.mem.set_u32(table + slot, target);
    }

    fn stub(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    fn stub_float(e: &mut Engine, address: u32, value: f64) {
        e.register_double(address, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    fn sequence(e: &mut Engine, address: u32, values: Vec<u32>) {
        let mut index = 0;
        e.register_double(address, move |_, _| {
            let value = values[index.min(values.len() - 1)];
            index += 1;
            value.into_ret()
        });
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn point_subtract(e: &mut Engine) {
        e.register(0x0043_9ef0, |e, a| {
            for axis in [0, 4, 8] {
                let value = e.mem.f32(a[0] + axis) as f64 - e.mem.f32(a[2] + axis) as f64;
                e.mem.set_f32(a[1] + axis, value as f32);
            }
            a[1].into_ret()
        });
    }

    fn set_point(e: &mut Engine, address: u32, point: [f32; 3]) {
        for (i, value) in point.iter().enumerate() {
            e.mem.set_f32(address + 4 * i as u32, *value);
        }
    }

    #[test]
    fn impact_data_save_game_writes_counts_and_collidee() {
        let mut e = engine();
        let impact: Ptr<ImpactData> = Ptr::new(e.mem.alloc(0x30));
        let collidee = object(&mut e, 0x70);
        e.set(impact, ImpactData::pCollidee, collidee);
        e.set(impact, ImpactData::pTargetWorldObject, 0x1234);
        stub(&mut e, 0x004b_6ae0, 5);
        stub(&mut e, 0x004b_6b60, 2);
        projectile_impact_data_save_game(&mut e, impact, Ptr::new(0x9000));
        assert_eq!(e.get(impact, ImpactData::sTargetWorldObjectCount), 5);
        assert_eq!(e.get(impact, ImpactData::sTargetWorldObjectIndex), 2);
        assert_eq!(calls(&e, 0x0086_5e50).len(), 7);
        assert_eq!(calls(&e, 0x0086_5df0), vec![vec![0x9000, collidee, 0]]);

        let empty: Ptr<ImpactData> = Ptr::new(e.mem.alloc(0x30));
        projectile_impact_data_save_game(&mut e, empty, Ptr::new(0x9000));
        assert_eq!(e.get(empty, ImpactData::sTargetWorldObjectCount), 0);
    }

    #[test]
    fn impact_data_load_game_reads_world_object_words_from_version_0x11() {
        let mut e = engine();
        let impact: Ptr<ImpactData> = Ptr::new(e.mem.alloc(0x30));
        let buffer = object(&mut e, 0x10);
        stub(&mut e, 0x7E00_0010, 0x10);
        set_slot(&mut e, buffer, 0, 0x7E00_0010);
        projectile_impact_data_load_game(&mut e, impact, Ptr::new(buffer));
        assert_eq!(calls(&e, 0x0086_4980).len(), 5);
        stub(&mut e, 0x7E00_0010, 0x11);
        projectile_impact_data_load_game(&mut e, impact, Ptr::new(buffer));
        assert_eq!(calls(&e, 0x0086_4980).len(), 12);
        assert_eq!(calls(&e, 0x0086_48e0).len(), 2);
    }

    #[test]
    fn impact_data_finish_load_game_resolves_or_warns() {
        let mut e = engine();
        let impact: Ptr<ImpactData> = Ptr::new(e.mem.alloc(0x30));
        let collidee = object(&mut e, 0x70);
        e.set(impact, ImpactData::pCollidee, collidee);
        e.set(impact, ImpactData::sTargetWorldObjectCount, 3);
        e.set(impact, ImpactData::sTargetWorldObjectIndex, 1);
        stub(&mut e, 0x004b_6ae0, 3);
        stub(&mut e, 0x004b_6c20, 0x4444);
        fn_009bbb40(&mut e, impact, 0);
        assert_eq!(e.get(impact, ImpactData::pTargetWorldObject), 0x4444);
        assert_eq!(calls(&e, 0x00c9_0510), vec![vec![0x4444]]);

        stub(&mut e, 0x004b_6ae0, 4);
        fn_009bbb40(&mut e, impact, 0);
        let warnings = calls(&e, 0x0084_5cc0);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0][0], 0x0109_0088);
        assert_eq!(&warnings[0][3..], &[3, 4]);
    }

    fn quiet_destructor_callees(e: &mut Engine) {
        stub(e, 0x0082_56d0, 1);
        stub(e, 0x0042_26e0, 1);
    }

    #[test]
    fn deleting_destructor_frees_with_bit_0() {
        let mut e = engine();
        quiet_destructor_callees(&mut e);
        let this = projectile(&mut e);
        fn_009bbc40(&mut e, this, 0);
        assert!(calls(&e, DEALLOCATE).is_empty());
        assert_eq!(calls(&e, 0x0092_ec40).len(), 1);
        fn_009bbc40(&mut e, this, 1);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![this.addr()]]);
    }

    #[test]
    fn constructor_sets_defaults() {
        let mut e = engine();
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(INITIAL_SOUND_DISTANCE, 10000.0f32);
        e.set_global(ZERO_POINT, 1.5f32);
        let this = projectile(&mut e);
        assert_eq!(projectile_ctor(&mut e, this, 0), this);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x18), CHILD_CELL_VTABLE);
        assert_eq!(e.get(this, Projectile::fPower), 1.0);
        assert_eq!(e.get(this, Projectile::fExplosionTimer), f32::MAX);
        assert_eq!(e.get(this, Projectile::fSoundDist), 10000.0);
        assert_eq!(e.mem.f32(this.addr() + MOVEMENT_DIRECTION), 1.5);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this.addr(), 0]]);
        assert_eq!(
            calls(&e, 0x00a8_6bf0),
            vec![vec![this.addr() + FOLLOW_OFFSET]]
        );
    }

    #[test]
    fn launch_constructor_divides_weapon_damage_by_projectile_count() {
        let mut e = engine();
        let this = projectile(&mut e);
        let weapon = e.mem.alloc(0x200);
        let component = object(&mut e, 0x10);
        e.mem.set_u32(weapon + 0x9C, e.mem.u32(component));
        stub(&mut e, 0x7E00_0020, 30);
        set_slot(&mut e, weapon + 0x9C, 0x10, 0x7E00_0020);
        stub(&mut e, 0x0052_5b20, 3);
        fn_009bbef0(
            &mut e,
            this,
            Ptr::new(0x5555),
            0,
            weapon,
            1.0f32.to_bits(),
            2.0f32.to_bits(),
            3.0f32.to_bits(),
            0.5,
            0.25,
        );
        assert_eq!(e.get(this, Projectile::fDamage), 10.0);
        assert_eq!(e.get(this, Projectile::pWeaponSource), weapon);
        assert_eq!(
            calls(&e, 0x0057_57d0),
            vec![vec![this.addr(), 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, 0x0057_5770),
            vec![vec![this.addr(), 0.25f32.to_bits()]]
        );
        assert_eq!(calls(&e, 0x0057_5690), vec![vec![this.addr(), 0x5555]]);
        assert_eq!(calls(&e, 0x0052_5b20), vec![vec![weapon, 0, 0, 0]]);
    }

    #[test]
    fn destructor_fades_sound_and_frees_target() {
        let mut e = engine();
        quiet_destructor_callees(&mut e);
        let this = projectile(&mut e);
        e.set(this, Projectile::pTarget, 0x7777);
        stub(&mut e, GET_BASE_OBJECT, 0x6000);
        fn_009bc470(&mut e, this);
        assert_eq!(
            calls(&e, 0x00ad_8da0),
            vec![vec![this.addr() + SOUND_HANDLE, 1000]]
        );
        assert_eq!(calls(&e, 0x0080_0190), vec![vec![0x7777]]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![0x7777]]);
        assert_eq!(e.get(this, Projectile::pTarget), 0);
        assert_eq!(calls(&e, 0x0092_ec40), vec![vec![this.addr()]]);
    }

    #[test]
    fn base_flag_0x80_check() {
        let mut e = engine();
        stub(&mut e, 0x004f_d420, 1);
        assert!(fn_009bc870(&mut e, Ptr::new(0x6000)));
        assert_eq!(calls(&e, 0x004f_d420), vec![vec![0x6000, 0x80]]);
    }

    #[test]
    fn target_deleting_destructor() {
        let mut e = engine();
        assert_eq!(fn_009bc890(&mut e, Ptr::new(0x6000), 0).addr(), 0x6000);
        assert!(calls(&e, DEALLOCATE).is_empty());
        fn_009bc890(&mut e, Ptr::new(0x6000), 1);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![0x6000]]);
        assert_eq!(calls(&e, 0x0080_0190).len(), 2);
    }

    #[test]
    fn form_id_known_check() {
        let mut e = engine();
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form + 0xC, 0x00AB_CDEF);
        e.set_global(0x011c_3f2cu32, 0x3000u32);
        stub(&mut e, 0x0046_9860, 1);
        assert!(fn_009bc8c0(&mut e, Ptr::new(form)));
        assert_eq!(calls(&e, 0x0046_9860), vec![vec![0x3000, 0x00AB_CDEF]]);
    }

    #[test]
    fn float_at_0x730() {
        let mut e = engine();
        let object = e.mem.alloc(0x740);
        e.mem.set_f32(object + 0x730, 2.5);
        assert_eq!(fn_009bd540(&mut e, Ptr::new(object)), 2.5);
        e.mem.set_u32(object + 0x730, 0x7F80_0001);
        assert_eq!(fn_009bd540(&mut e, Ptr::new(object)).to_bits(), 0x7FC0_0001);
    }

    #[test]
    fn pick_cache_assignment() {
        let mut e = engine();
        let this = projectile(&mut e);
        fn_009bd560(&mut e, this, 0x4242);
        assert_eq!(
            calls(&e, ASSIGN_NI_POINTER),
            vec![vec![this.addr() + PICK_CACHE, 0x4242]]
        );
    }

    #[test]
    fn spread_heading_setter() {
        let mut e = engine();
        let this = projectile(&mut e);
        fn_009bd580(&mut e, this, 0.75);
        assert_eq!(e.get(this, Projectile::fSpreadHeading), 0.75);
    }

    #[test]
    fn spread_looking_setter() {
        let mut e = engine();
        let this = projectile(&mut e);
        fn_009bd5a0(&mut e, this, -0.5);
        assert_eq!(e.get(this, Projectile::fSpreadLooking), -0.5);
    }

    #[test]
    fn creates_type_0x20000_projectile_with_explosion_timer() {
        let mut e = engine();
        let table = vtable(&mut e);
        stub(&mut e, 0x004f_d4f0, 0x20000);
        e.register_double(0x009b_4020, move |e, a| {
            e.mem.set_u32(a[0], table);
            a[0].into_ret()
        });
        e.mem.set_f32(SETTING_EXPLOSION_TIMER + 4, 4.0);
        let made = fn_009bd5c0(&mut e, Ptr::new(0x6000), 0, 0x8888, 0x9999);
        assert!(!made.is_null());
        let created = calls(&e, 0x009b_4020);
        assert_eq!(created[0][1..4], [0x6000, 0, 0x9999]);
        assert_eq!(calls(&e, SET_FLAGS), vec![vec![made.addr(), 0x400, 1]]);
        assert_eq!(calls(&e, 0x0042_0ba0), vec![vec![made.addr(), 0x8888]]);
        assert_eq!(e.get(made, Projectile::fExplosionTimer), 4.0);

        stub(&mut e, 0x004f_d4f0, 0x10000);
        assert!(fn_009bd5c0(&mut e, Ptr::new(0x6000), 0, 0, 0).is_null());
    }

    #[test]
    fn explosion_timer_setter() {
        let mut e = engine();
        let this = projectile(&mut e);
        fn_009bd760(&mut e, this, 3.0);
        assert_eq!(e.get(this, Projectile::fExplosionTimer), 3.0);
    }

    #[test]
    fn finds_shooters_projectile_of_base() {
        let mut e = engine();
        let first = projectile(&mut e);
        let second = projectile(&mut e);
        for p in [first, second] {
            set_slot(&mut e, p.addr(), 0x224, RETURNS_TRUE);
        }
        e.set(second, Projectile::pShooter, 0x1111);
        stub(&mut e, 0x005b_e5c0, 2);
        sequence(&mut e, 0x0096_8670, vec![first.addr(), second.addr()]);
        stub(&mut e, GET_BASE_OBJECT, 0x6000);
        assert_eq!(fn_009bd780(&mut e, 0x1111, 0x6000), second);
        stub(&mut e, 0x005b_e5c0, 0);
        assert!(fn_009bd780(&mut e, 0x1111, 0x6000).is_null());
    }

    #[test]
    fn forwards_to_0055af90() {
        let mut e = engine();
        stub(&mut e, 0x0055_af90, 1);
        assert!(fn_009bd830(&mut e, Ptr::new(0x6000), 7));
        assert_eq!(calls(&e, 0x0055_af90), vec![vec![0x6000, 7]]);
    }

    #[test]
    fn aim_projectile_aims_through_the_player() {
        let mut e = engine();
        let player = object(&mut e, 0x100);
        e.set_global(PLAYER, player);
        stub(&mut e, 0x7E00_0030, 0x2222);
        set_slot(&mut e, player, 0x20C, 0x7E00_0030);
        e.set_global(0x0102_efc4u32, 0.5f32);
        let this = projectile(&mut e);
        projectile_aim_projectile(&mut e, this);
        let aimed = calls(&e, 0x0096_5620);
        assert_eq!(
            aimed,
            vec![vec![
                player,
                this.addr(),
                0x2222,
                this.addr() + REFR_ANGLE_Z,
                this.addr() + REFR_ANGLE_X,
                this.addr() + REFR_POSITION,
                0.5f32.to_bits(),
                0.5f32.to_bits()
            ]]
        );

        e.set(this, Projectile::iFlags, 4);
        projectile_aim_projectile(&mut e, this);
        assert_eq!(calls(&e, 0x0096_5620).len(), 1);
    }

    #[test]
    fn init_3d_scales_and_starts_animations() {
        let mut e = engine();
        let this = projectile(&mut e);
        let root = object(&mut e, 0x100);
        stub(&mut e, 0x7E00_0040, root);
        set_slot(&mut e, this.addr(), 0x1D0, 0x7E00_0040);
        let angles = e.mem.alloc(0xC);
        stub(&mut e, 0x0043_0830, angles);
        projectile_init_3d(&mut e, this);
        assert_eq!(calls(&e, 0x0044_0490), vec![vec![root, 1.0f32.to_bits()]]);
        assert_eq!(calls(&e, 0x00a6_d2d0), vec![vec![root]]);
        assert_eq!(calls(&e, SLOT_BASE + 0x1C8), vec![vec![this.addr(), 0]]);
    }

    #[test]
    fn stores_byte_at_0x95() {
        let mut e = engine();
        let this = projectile(&mut e);
        fn_009beae0(&mut e, this, 9);
        assert_eq!(e.mem.u8(this.addr() + 0x95), 9);
    }

    #[test]
    fn rearms_physics_and_adds_velocity() {
        let mut e = engine();
        let this = projectile(&mut e);
        e.set(this, Projectile::iFlags, 4);
        stub(&mut e, 0x0046_9860, 1);
        let root = object(&mut e, 0x100);
        stub(&mut e, 0x7E00_0040, root);
        set_slot(&mut e, this.addr(), 0x1D0, 0x7E00_0040);
        stub_float(&mut e, 0x0096_69c0, 3.0);
        fn_009beb00(&mut e, this);
        assert_eq!(calls(&e, 0x00c6_a040).len(), 2);
        let scaled = calls(&e, 0x0043_9180);
        assert_eq!(scaled[0][1], 3.0f32.to_bits());
        assert_eq!(calls(&e, 0x0062_b8d0)[0][2], 1);
        assert_eq!(calls(&e, SLOT_BASE + 0x48), vec![vec![this.addr(), 4]]);

        e.set(this, Projectile::iFlags, 0xC);
        fn_009beb00(&mut e, this);
        assert_eq!(calls(&e, 0x0062_b8d0).len(), 1);
    }

    #[test]
    fn raises_alarm_once_past_the_distance() {
        let mut e = engine();
        let this = projectile(&mut e);
        e.set(this, Projectile::iFlags, 0x400);
        e.mem.set_f32(SETTING_ALARM_DISTANCE + 4, 2.0);
        stub_float(&mut e, 0x0088_5bd0, 3.0);
        fn_009bf190(&mut e, this);
        assert_eq!(calls(&e, SET_FLAGS), vec![vec![this.addr(), 0x800, 1]]);
        assert_eq!(calls(&e, 0x009a_52f0).len(), 1);

        stub_float(&mut e, 0x0088_5bd0, 2.0);
        fn_009bf190(&mut e, this);
        assert_eq!(calls(&e, 0x009a_52f0).len(), 1);
    }

    #[test]
    fn moves_and_reports_the_step() {
        let mut e = engine();
        point_subtract(&mut e);
        let this = projectile(&mut e);
        let before = e.mem.alloc(0xC);
        let after = e.mem.alloc(0xC);
        set_point(&mut e, before, [1.0, 1.0, 1.0]);
        set_point(&mut e, after, [2.0, 3.0, 4.0]);
        sequence(&mut e, 0x7E00_0050, vec![before, after]);
        set_slot(&mut e, this.addr(), 0x1F4, 0x7E00_0050);
        fn_009bf370(&mut e, this, 0.5, 7, 8, 9);
        let moved = calls(&e, 0x0092_f260);
        assert_eq!(moved[0][..2], [this.addr(), 0.5f32.to_bits()]);
        assert_eq!(moved[0][3], 0xF);
        assert_eq!(
            calls(&e, 0x009c_4e60),
            vec![vec![
                this.addr(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                1
            ]]
        );
    }

    #[test]
    fn add_impact_records_the_hit_and_adds_a_water_ripple() {
        let mut e = engine();
        let this = projectile(&mut e);
        let location = e.mem.alloc(0xC);
        let normal = e.mem.alloc(0xC);
        set_point(&mut e, location, [1.0, 2.0, 3.0]);
        set_point(&mut e, normal, [0.0, 0.0, 1.0]);
        let collidee = object(&mut e, 0x70);
        let tes = e.mem.alloc(0x10);
        e.set_global(TES, tes);
        stub(&mut e, 0x0070_ec90, 0x5150);
        e.set_global(0x0101_3ea4u32, 0.25f32);
        assert!(projectile_add_impact(
            &mut e,
            this,
            collidee,
            Ptr::new(location),
            Ptr::new(normal),
            0,
            0,
            8
        ));
        let added = calls(&e, 0x005a_e3d0);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], this.addr() + IMPACT_LIST);
        assert_eq!(
            calls(&e, 0x004e_8510),
            vec![vec![
                0x5150,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                0.25f32.to_bits()
            ]]
        );
        assert_eq!(calls(&e, 0x0056_4db0), vec![vec![collidee, 1]]);
    }

    /// Points virtual +0x1F4 (position) of `object` at a fresh zero point.
    fn give_position(e: &mut Engine, object: u32, stub_address: u32) -> u32 {
        let point = e.mem.alloc(0xC);
        stub(e, stub_address, point);
        set_slot(e, object, 0x1F4, stub_address);
        point
    }

    #[test]
    fn turns_off_and_runs_0x3e_hook() {
        let mut e = engine();
        let this = projectile(&mut e);
        stub(&mut e, 0x0040_1170, 0x3E);
        fn_009bc8f0(&mut e, this);
        assert_eq!(calls(&e, 0x007f_6a20), vec![vec![this.addr()]]);
        assert_eq!(calls(&e, 0x0057_4400), vec![vec![this.addr()]]);
        stub(&mut e, 0x0046_9860, 1);
        fn_009bc8f0(&mut e, this);
        assert_eq!(calls(&e, SLOT_BASE + 0xC4), vec![vec![this.addr(), 1]]);
        assert_eq!(calls(&e, 0x0057_4400).len(), 1);
    }

    #[test]
    fn launch_creates_and_registers_a_projectile() {
        let mut e = engine();
        let table = vtable(&mut e);
        stub(&mut e, 0x004f_d4f0, 0x20000);
        e.register_double(0x009b_4020, move |e, a| {
            e.mem.set_u32(a[0], table);
            a[0].into_ret()
        });
        let made = fn_009bca60(
            &mut e,
            Ptr::new(0x6000),
            0,
            0x3333,
            0,
            0,
            0,
            0,
            0.0,
            0.0,
            0x5151,
            0x4444,
            1,
            1,
            0.25,
            0.5,
            0,
        );
        assert!(!made.is_null());
        assert_eq!(e.get(made, Projectile::fSpreadHeading), 0.25);
        assert_eq!(e.get(made, Projectile::fSpreadLooking), 0.5);
        assert_eq!(e.get(made, Projectile::pDesiredTarget), 0x4444);
        assert_eq!(calls(&e, 0x0097_f720), vec![vec![0x3333, made.addr()]]);
        assert_eq!(
            calls(&e, SET_FLAGS),
            vec![vec![made.addr(), 0x20000, 1], vec![made.addr(), 0x1000, 1]]
        );
        assert_eq!(
            calls(&e, ASSIGN_NI_POINTER),
            vec![vec![made.addr() + PICK_CACHE, 0x5151]]
        );
        assert_eq!(calls(&e, 0x009a_52f0), vec![vec![0, made.addr()]]);
    }

    #[test]
    fn start_loaded_projectile_sets_process_range_and_timer() {
        let mut e = engine();
        let this = projectile(&mut e);
        stub(&mut e, 0x0042_ce10, 1);
        stub(&mut e, 0x004f_d400, 0x9999);
        stub(&mut e, 0x008d_7510, 0x1234);
        stub_float(&mut e, 0x009a_7c40, 500.0);
        stub_float(&mut e, 0x0064_4790, 7.0);
        fn_009bda10(&mut e, this);
        assert_eq!(e.get(this, Projectile::pWeaponSource), 0x9999);
        assert_eq!(e.get(this, Projectile::pCurrentProcess), 0x1234);
        assert_eq!(e.get(this, Projectile::fRange), 500.0);
        assert_eq!(e.get(this, Projectile::fExplosionTimer), 7.0);
        assert_eq!(
            calls(&e, 0x0096_d450),
            vec![vec![PROCESS_LISTS, this.addr(), 0, 0, 0, 0]]
        );
        assert_eq!(calls(&e, SLOT_BASE + 0x30C).len(), 1);
    }

    #[test]
    fn weapon_source_setter() {
        let mut e = engine();
        let this = projectile(&mut e);
        fn_009bdf60(&mut e, this, 0x77);
        assert_eq!(e.get(this, Projectile::pWeaponSource), 0x77);
    }

    #[test]
    fn explosion_timer_kept_by_base() {
        let mut e = engine();
        let this = projectile(&mut e);
        assert!(!fn_009bdf80(&mut e, this));
        stub(&mut e, 0x0097_5300, 1);
        stub_float(&mut e, 0x0045_cd80, 2.0);
        assert!(fn_009bdf80(&mut e, this));
        stub_float(&mut e, 0x0045_cd80, f64::NAN);
        assert!(!fn_009bdf80(&mut e, this));
        stub(&mut e, 0x005d_e080, 1);
        assert!(fn_009bdf80(&mut e, this));
    }

    #[test]
    fn havok_setup_keyframes_flag_8_projectiles() {
        let mut e = engine();
        let this = projectile(&mut e);
        let root = object(&mut e, 0x100);
        stub(&mut e, 0x7E00_0040, root);
        set_slot(&mut e, this.addr(), 0x1D0, 0x7E00_0040);
        e.set(this, Projectile::pCurrentProcess, 0x1234);
        e.set(this, Projectile::iFlags, 8);
        stub(&mut e, 0x008d_6f30, 0x5000);
        stub(&mut e, 0x0045_43c0, 0x6000);
        fn_009be0a0(&mut e, this);
        assert_eq!(calls(&e, 0x00c6_a0b0), vec![vec![root, 0, 1, 0]]);
        assert_eq!(calls(&e, 0x00c6_a350), vec![vec![root, 4, 1, 0, 1]]);

        stub(&mut e, 0x0045_cd60, 1);
        fn_009be0a0(&mut e, this);
        assert_eq!(calls(&e, 0x00c6_a0b0).len(), 1);
    }

    #[test]
    fn slot_0x2f8_without_flag_0x80() {
        let mut e = engine();
        let this = projectile(&mut e);
        fn_009bec10(&mut e, this, 1.5);
        assert_eq!(
            calls(&e, SLOT_BASE + 0x2F8),
            vec![vec![this.addr(), 1.5f32.to_bits()]]
        );
        e.set(this, Projectile::iFlags, 0x80);
        fn_009bec10(&mut e, this, 1.5);
        assert_eq!(calls(&e, SLOT_BASE + 0x2F8).len(), 1);
    }

    #[test]
    fn slot_0x2f8_with_flag_0x80() {
        let mut e = engine();
        let this = projectile(&mut e);
        assert!(!fn_009bec50(&mut e, this, 1.5));
        assert!(calls(&e, SLOT_BASE + 0x2F8).is_empty());
        e.set(this, Projectile::iFlags, 0x80);
        assert!(!fn_009bec50(&mut e, this, 2.5));
        assert_eq!(
            calls(&e, SLOT_BASE + 0x2F8),
            vec![vec![this.addr(), 2.5f32.to_bits()]]
        );
    }

    #[test]
    fn slot_0x328_with_flag_1() {
        let mut e = engine();
        let this = projectile(&mut e);
        fn_009bec90(&mut e, this);
        assert!(calls(&e, SLOT_BASE + 0x328).is_empty());
        e.set(this, Projectile::iFlags, 1);
        fn_009bec90(&mut e, this);
        assert_eq!(calls(&e, SLOT_BASE + 0x328), vec![vec![this.addr()]]);
    }

    #[test]
    fn update_fades_ages_and_alarms() {
        let mut e = engine();
        let this = projectile(&mut e);
        e.set(this, Projectile::iFlags, 0x20);
        e.set(this, Projectile::fAlpha, 1.0);
        e.set(this, Projectile::fAge, 1.0);
        stub_float(&mut e, 0x0084_d030, 0.5);
        stub_float(&mut e, 0x006c_a4e0, 2.0);
        projectile_update(&mut e, this, 0);
        assert_eq!(e.get(this, Projectile::fAlpha), 0.75);
        assert_eq!(e.get(this, Projectile::fAge), 1.5);
        assert_eq!(
            calls(&e, SLOT_BASE + 0x310),
            vec![vec![this.addr(), 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, 0x0097_7770),
            vec![vec![PROCESS_LISTS, 0, this.addr()]]
        );
        assert!(e.get(this, Projectile::bSendAlarm));

        e.set(this, Projectile::fAlpha, 0.1);
        projectile_update(&mut e, this, 0);
        assert_eq!(e.get(this, Projectile::fAlpha), 0.0);
        assert_eq!(calls(&e, 0x0097_7770).len(), 1);
    }

    #[test]
    fn forward_step_moves_by_speed_times_delta() {
        let mut e = engine();
        let this = projectile(&mut e);
        give_position(&mut e, this.addr(), 0x7E00_0050);
        point_subtract(&mut e);
        stub_float(&mut e, 0x0096_69c0, 2.0);
        fn_009bf300(&mut e, this, 3.0);
        let made = calls(&e, 0x0041_6870);
        assert_eq!(made[0][1..], [0, 6.0f32.to_bits(), 0]);
        assert_eq!(calls(&e, 0x0092_f260)[0][1], 3.0f32.to_bits());
    }

    #[test]
    fn sync_3d_moves_rotates_and_tilts() {
        let mut e = engine();
        let this = projectile(&mut e);
        let root = object(&mut e, 0x100);
        stub(&mut e, 0x7E00_0040, root);
        set_slot(&mut e, this.addr(), 0x1D0, 0x7E00_0040);
        let position = give_position(&mut e, this.addr(), 0x7E00_0050);
        let angles = e.mem.alloc(0xC);
        set_point(&mut e, angles, [1.0, 2.0, 3.0]);
        stub(&mut e, 0x0043_0830, angles);
        let matrix = e.mem.alloc(0x24);
        stub(&mut e, 0x006a_9540, matrix);
        let vector = e.mem.alloc(0xC);
        stub(&mut e, 0x009b_7010, vector);
        stub_float(&mut e, 0x004b_54c0, 0.5);
        e.set(this, Projectile::iFlags, 0x40);
        projectile_sync_3d_with_reference(&mut e, this, 1, 1);
        assert_eq!(calls(&e, 0x0044_0460), vec![vec![root, position]]);
        let rotations = calls(&e, 0x00a5_9660);
        assert_eq!(
            rotations[0][1..],
            [3.0f32.to_bits(), 1.0f32.to_bits(), 2.0f32.to_bits()]
        );
        assert_eq!(rotations[1][2], (-0.5f32).to_bits());
        assert_eq!(calls(&e, 0x0043_fa80).len(), 2);
    }

    #[test]
    fn impact_data_constructor_defaults() {
        let mut e = engine();
        e.set_global(ZERO_POINT, 2.0f32);
        let impact: Ptr<ImpactData> = Ptr::new(e.mem.alloc(0x30));
        e.set(impact, ImpactData::pCollidee, 0x55);
        assert_eq!(fn_009bffb0(&mut e, impact), impact);
        assert_eq!(e.get(impact, ImpactData::pCollidee), 0);
        assert_eq!(e.get(impact, ImpactData::kLocationX), 2.0);
        assert_eq!(e.get(impact, ImpactData::eMaterial), 0x24);
        assert_eq!(e.get(impact, ImpactData::eDamageLimb), 0xFFFF_FFFF);
        assert!(e.get(impact, ImpactData::bDoDamage));
        assert_eq!(
            calls(&e, 0x0043_90c0),
            vec![vec![impact.addr() + 0x10, ZERO_POINT]]
        );
    }

    #[test]
    fn subclass_hit_count_setter() {
        let mut e = engine();
        let this = e.mem.alloc(0x160);
        fn_009c0050(&mut e, Ptr::new(this), 9);
        assert_eq!(e.mem.u32(this + 0x154), 9);
    }

    #[test]
    fn subclass_hit_count_getter() {
        let mut e = engine();
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + 0x154, 4);
        assert_eq!(fn_009c0070(&mut e, Ptr::new(this)), 4);
    }

    #[test]
    fn phantom_check_needs_base_and_world() {
        let mut e = engine();
        let this = projectile(&mut e);
        assert!(!fn_009c0090(&mut e, this));
        assert!(calls(&e, 0x006a_b360).is_empty());
        stub(&mut e, 0x009b_af30, 1);
        stub(&mut e, 0x0048_cee0, 1);
        assert!(!fn_009c0090(&mut e, this));
        assert!(calls(&e, 0x0054_1c80).is_empty());
    }

    #[test]
    fn run_target_pick_casts_and_reports_miss() {
        let mut e = engine();
        let this = projectile(&mut e);
        assert!(!projectile_run_target_pick(&mut e, this));
        let world = object(&mut e, 0x100);
        stub(&mut e, 0x008d_6f30, 0x5000);
        stub(&mut e, 0x0045_43c0, world);
        give_position(&mut e, this.addr(), 0x7E00_0050);
        let product = e.mem.alloc(0x24);
        stub(&mut e, 0x0043_f8d0, product);
        e.set_global(0x0101_2070u32, 1.0f64);
        assert!(!projectile_run_target_pick(&mut e, this));
        assert_eq!(calls(&e, 0x004a_3c20).len(), 1);
        assert_eq!(calls(&e, 0x00c6_6230), vec![vec![1], vec![0]]);
        assert_eq!(calls(&e, SLOT_BASE + 0xC8).len(), 1);
        assert_eq!(calls(&e, 0x004a_3bc0).len(), 1);
        assert!(calls(&e, 0x005b_5e40).is_empty());
    }

    #[test]
    fn hit_processing_takes_the_first_hit_without_projectile() {
        let mut e = engine();
        assert_eq!(fn_009c0d80(&mut e, Ptr::NULL, Ptr::NULL, 0, 0, 0, 1.0), 0);
        stub(&mut e, 0x008c_dd90, 0x9100);
        stub(&mut e, 0x0072_6070, 1);
        stub(&mut e, 0x004b_5820, 0x5000);
        let out = e.mem.alloc(4);
        e.mem.set_f32(out, 9.0);
        let node = fn_009c0d80(&mut e, Ptr::new(0x9000), Ptr::NULL, 0, out, 0, f32::MAX);
        assert_eq!(node, 0x5000);
        assert_eq!(e.mem.f32(out), 0.0);
        assert_eq!(
            calls(&e, 0x0045_8620),
            vec![vec![calls(&e, 0x0045_8620)[0][0], 0x9090]]
        );
    }

    /// A target hit (a body in the set at `01268444`) re-cast onto the target's node: the
    /// re-cast's first hit replaces the record, and `009c2c80` is asked about the collidable
    /// the record holds then (009c121d reads `[EBP-0x50]` again), not the one the pick hit.
    #[test]
    fn hit_processing_vetoes_with_the_collidable_the_re_cast_left() {
        let mut e = engine();
        let pick = e.mem.alloc(0xB0);
        let (first_collidable, refined_collidable) = (0x7A00u32, 0x7B00u32);
        let records = [first_collidable, refined_collidable].map(|collidable| {
            let record = e.mem.alloc(0x60);
            e.mem.set_f32(record + 0x10, 0.5);
            e.mem.set_u32(record + 0x50, collidable);
            record
        });
        stub(&mut e, 0x008c_dd90, 0x9100);
        stub(&mut e, 0x0072_6070, 1);
        sequence(&mut e, 0x004a_46b0, records.to_vec());
        e.register_double(0x0069_6c30, |e, a| {
            let words = e.mem.bytes(a[1], 0x60);
            e.mem.write(a[0], &words);
            a[0].into_ret()
        });
        stub(&mut e, 0x004b_5820, 0x5000);
        stub(&mut e, 0x0056_f930, 0x6000);
        // Only the collidable the pick hit belongs to an entity of the target set.
        e.register_double(0x004b_5950, move |_, a| {
            u32::from(a[0] == first_collidable)
                .wrapping_mul(0x7000)
                .into_ret()
        });
        stub(&mut e, 0x004b_5a80, 0x7100);
        stub(&mut e, 0x0043_b300, 1);
        // The target's first collision object is flagged (0x4000): the short re-cast runs
        // and hits.
        stub(&mut e, 0x004b_5260, 0x7200);
        stub(&mut e, 0x006f_a820, 0x7300);
        let info = e.mem.alloc(4);
        e.mem.set_u32(info, 0x4000);
        stub(&mut e, 0x0043_b4f0, info);
        stub(&mut e, 0x009c_5ba0, 1);
        let node = fn_009c0d80(&mut e, Ptr::new(pick), Ptr::NULL, 0, 0, 1, f32::MAX);
        // With the re-cast's collidable `009c2c80` finds no entity and lets the hit through.
        assert_eq!(node, 0x5000);
        let asked = calls(&e, 0x004b_5950);
        assert_eq!(asked.last(), Some(&vec![refined_collidable]));
    }

    #[test]
    fn shape_flag_0x4000() {
        let mut e = engine();
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 0x4000);
        assert!(fn_009c19e0(&mut e, Ptr::new(word)));
        e.mem.set_u32(word, 0x3FFF);
        assert!(!fn_009c19e0(&mut e, Ptr::new(word)));
    }

    #[test]
    fn filter_info_of_word_0x24() {
        let mut e = engine();
        let pick = e.mem.alloc(0x30);
        e.mem.set_u32(pick + 0x24, 0x77);
        assert_eq!(fn_009c1a00(&mut e, Ptr::new(pick), 0x8000).addr(), 0x8000);
        assert_eq!(calls(&e, 0x008c_71b0), vec![vec![0x8000, 0x77]]);
    }

    #[test]
    fn writes_vector_0x90() {
        let mut e = engine();
        fn_009c1a20(&mut e, Ptr::new(0x9000), 0x8000);
        assert_eq!(calls(&e, 0x0045_8620), vec![vec![0x8000, 0x9090]]);
    }

    #[test]
    fn vector_0x90_address() {
        let mut e = engine();
        assert_eq!(fn_009c1a50(&mut e, Ptr::new(0x9000)), 0x9090);
    }

    fn hit_record(e: &mut Engine) -> u32 {
        let record = e.mem.alloc(0x60);
        for offset in (0..0x60).step_by(4) {
            e.mem.set_u32(record + offset, 0x100 + offset);
        }
        e.mem.set_u32(record + 0x10, 0x7F80_0001);
        record
    }

    #[test]
    fn hit_record_copy_with_collidable() {
        let mut e = engine();
        let source = hit_record(&mut e);
        let target = e.mem.alloc(0x60);
        fn_009c1a70(&mut e, Ptr::new(target), source);
        assert_eq!(e.mem.u32(target + 0x50), 0x150);
        assert_eq!(e.mem.u32(target + 0x24), 0x124);
        assert_eq!(e.mem.u32(target + 0x10), 0x7FC0_0001);
        assert_eq!(calls(&e, 0x004a_3f10), vec![vec![target, source]]);
    }

    #[test]
    fn hit_record_copy_with_keys() {
        let mut e = engine();
        let source = hit_record(&mut e);
        let target = e.mem.alloc(0x60);
        fn_009c1aa0(&mut e, Ptr::new(target), source);
        assert_eq!(e.mem.u32(target + 0x3C), 0x13C);
        assert_eq!(e.mem.u32(target + 0x40), 0x140);
        assert_eq!(e.mem.u32(target + 0x50), 0);
    }

    #[test]
    fn hit_record_copy_base() {
        let mut e = engine();
        let source = hit_record(&mut e);
        let target = e.mem.alloc(0x60);
        fn_009c1b00(&mut e, Ptr::new(target), source);
        assert_eq!(e.mem.u32(target + 0x14), 0x114);
        assert_eq!(e.mem.u32(target + 0x1C), 0x11C);
        assert_eq!(e.mem.u32(target + 0x20), 0);
    }

    #[test]
    fn process_impacts_walks_the_list_once() {
        let mut e = engine();
        let this = projectile(&mut e);
        let impact = e.mem.alloc(0x30);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, impact);
        stub(&mut e, 0x0068_15c0, item);
        stub(&mut e, 0x0052_5420, 1);
        let vector = e.mem.alloc(0xC);
        stub(&mut e, 0x009b_7010, vector);
        assert!(projectile_process_impacts(&mut e, this));
        assert!(e.get(this, Projectile::bImpactProcessed));
        assert!(calls(&e, 0x0088_5d70).is_empty());
        assert_eq!(
            calls(&e, 0x0072_6070),
            vec![vec![this.addr() + IMPACT_LIST]]
        );
        assert!(!projectile_process_impacts(&mut e, this));
    }

    #[test]
    fn impact_effects_play_the_impact_sound() {
        let mut e = engine();
        let this = projectile(&mut e);
        let location = e.mem.alloc(0xC);
        set_point(&mut e, location, [1.0, 2.0, 3.0]);
        let normal = e.mem.alloc(0xC);
        let target = actor(&mut e);
        fn_009c20e0(
            &mut e,
            this,
            target,
            Ptr::new(location),
            Ptr::new(normal),
            0,
            1,
        );
        assert!(calls(&e, 0x0052_2ba0).is_empty());

        e.set(this, Projectile::pWeaponSource, 0x4000);
        stub(&mut e, 0x0052_2ba0, 0x7000);
        stub(&mut e, 0x0052_5420, 1);
        stub(&mut e, 0x0068_a810, 0x7100);
        fn_009c20e0(&mut e, this, 0, Ptr::new(location), Ptr::new(normal), 0, 1);
        assert_eq!(calls(&e, 0x0052_2ba0), vec![vec![0x4000, 1]]);
        let sound = calls(&e, 0x00ad_7480);
        assert_eq!(sound.len(), 1);
        assert_eq!(sound[0][3..], [0x4102, 0x7100]);
        assert_eq!(
            calls(&e, 0x00ad_8b60)[0][1..],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
    }

    #[test]
    fn hit_veto_for_listed_entities_and_shapes() {
        let mut e = engine();
        assert!(fn_009c2c80(&mut e, 0, 0, 0, 0));
        stub(&mut e, 0x004b_5950, 0x20);
        stub(&mut e, 0x004b_5a80, 0x30);
        stub(&mut e, 0x0043_b300, 1);
        assert!(!fn_009c2c80(&mut e, 0, 0x10, 0, 0));
        stub(&mut e, 0x0043_b300, 0);
        assert!(!fn_009c2c80(&mut e, 0, 0x10, 0, 0));
        stub(&mut e, 0x0043_b4d0, 0x1E);
        assert!(fn_009c2c80(&mut e, 0, 0x10, 0, 0));
        stub(&mut e, 0x004b_5950, 0);
        stub(&mut e, 0x0043_b4d0, 0x1A);
        stub(&mut e, 0x004f_d4f0, 0x40000);
        assert!(!fn_009c2c80(&mut e, 0, 0x10, 0, 0x6000));
        stub(&mut e, 0x004f_d4f0, 0x20000);
        assert!(fn_009c2c80(&mut e, 0, 0x10, 0, 0x6000));
    }

    #[test]
    fn base_flag_0x200_check() {
        let mut e = engine();
        stub(&mut e, 0x004f_d420, 1);
        assert!(fn_009c2e60(&mut e, Ptr::new(0x6000)));
        assert_eq!(calls(&e, 0x004f_d420), vec![vec![0x6000, 0x200]]);
    }

    #[test]
    fn pushes_the_hit_body() {
        let mut e = engine();
        let this = projectile(&mut e);
        fn_009c2e80(&mut e, this, 0, 0, 0, 0, 0, 0, 0);
        assert!(calls(&e, 0x004b_59f0).is_empty());
        let entity = object(&mut e, 0x20);
        stub(&mut e, 0x004b_5a80, entity);
        stub(&mut e, 0x7E00_0060, 0x7700);
        set_slot(&mut e, entity, 0x98, 0x7E00_0060);
        stub_float(&mut e, 0x0064_4930, 2.0);
        stub_float(&mut e, 0x0062_b520, 3.0);
        stub_float(&mut e, 0x0045_86d0, 1.0);
        stub(&mut e, 0x004a_e750, 0x7800);
        fn_009c2e80(&mut e, this, 0, 0, 0, 0, 0, 0, 0x10);
        assert_eq!(calls(&e, 0x0062_b520)[0][1..], [entity, 1]);
        assert_eq!(calls(&e, 0x005d_bf20)[0][1], 3.0f32.to_bits());
        assert_eq!(calls(&e, 0x0062_aea0).len(), 1);
        assert_eq!(calls(&e, 0x0048_3710), vec![vec![0x7700], vec![0x7700]]);
        assert_eq!(calls(&e, 0x0081_f1e0)[0][0], 0x7800);
    }

    #[test]
    fn notifies_living_shooter() {
        let mut e = engine();
        let this = projectile(&mut e);
        let shooter = object(&mut e, 0x100);
        e.set(this, Projectile::pShooter, shooter);
        stub(&mut e, 0x009b_af30, 1);
        stub(&mut e, 0x0048_cee0, 1);
        fn_009c2ff0(&mut e, this);
        assert_eq!(calls(&e, 0x009b_b6d0), vec![vec![shooter, this.addr()]]);
        set_slot(&mut e, shooter, 0x22C, RETURNS_TRUE);
        fn_009c2ff0(&mut e, this);
        assert_eq!(calls(&e, 0x009b_b6d0).len(), 1);
    }

    #[test]
    fn combat_hit_reports_for_actor_shooters() {
        let mut e = engine();
        let this = projectile(&mut e);
        let shooter = actor(&mut e);
        e.set(this, Projectile::pShooter, shooter);
        e.set(this, Projectile::iFlags, 0x80000);
        projectile_combat_hit(&mut e, this, 0x2222, 1);
        assert!(calls(&e, 0x0089_9cb0).is_empty());
        set_slot(&mut e, shooter, 0x360, RETURNS_TRUE);
        projectile_combat_hit(&mut e, this, 0x2222, 1);
        assert_eq!(
            calls(&e, 0x0089_9cb0),
            vec![vec![shooter, 0x2222, 1, this.addr(), 0]]
        );
    }

    #[test]
    fn check_explosion_explodes_and_blinks() {
        let mut e = engine();
        let this = projectile(&mut e);
        stub(&mut e, 0x004f_d360, 1);
        stub(&mut e, 0x004f_d3c0, 0x8800);
        stub(&mut e, 0x008d_6f30, 0x5000);
        give_position(&mut e, this.addr(), 0x7E00_0050);
        e.set(this, Projectile::iFlags, 0x100000);
        projectile_check_explosion(&mut e, this, 0.1);
        let made = calls(&e, 0x009a_c9c0);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0].len(), 16);
        assert_eq!(made[0][..4], [0x8800, 0, 0, 0x5000]);
        assert_eq!(calls(&e, 0x009c_6380).len(), 1);

        stub(&mut e, 0x0097_5300, 1);
        e.set(this, Projectile::fExplosionTimer, 5.0);
        e.set(this, Projectile::fBlinkTimer, 0.5);
        e.mem.set_f32(0x011c_e9e4 + 4, 2.0);
        e.mem.set_f32(0x011c_ee78 + 4, 0.75);
        projectile_check_explosion(&mut e, this, 1.0);
        assert_eq!(e.get(this, Projectile::fExplosionTimer), 4.0);
        assert_eq!(e.get(this, Projectile::fBlinkTimer), 0.75);
        assert_eq!(calls(&e, 0x009c_4ec0), vec![vec![this.addr()]]);
        assert_eq!(calls(&e, 0x009a_c9c0).len(), 1);
    }

    #[test]
    fn mine_reacts_to_hostile_targets_only() {
        let mut e = engine();
        let this = projectile(&mut e);
        let shooter = actor(&mut e);
        e.set(this, Projectile::pShooter, shooter);
        assert!(!projectile_get_mine_reacts_to_target(
            &mut e, this, shooter, 0, 0
        ));
        let target = actor(&mut e);
        assert!(projectile_get_mine_reacts_to_target(
            &mut e, this, target, 0, 0
        ));
        stub(&mut e, 0x008b_87a0, 2);
        assert!(!projectile_get_mine_reacts_to_target(
            &mut e, this, target, shooter, 0
        ));
        assert_eq!(calls(&e, 0x008b_87a0)[0][..2], [shooter, target]);
        stub(&mut e, 0x008b_8290, 0);
        assert!(!projectile_get_mine_reacts_to_target(
            &mut e, this, target, 0, 0x99
        ));
    }

    #[test]
    fn mine_triggers_on_actor_in_range() {
        let mut e = engine();
        let this = projectile(&mut e);
        give_position(&mut e, this.addr(), 0x7E00_0050);
        let target = actor(&mut e);
        give_position(&mut e, target, 0x7E00_0051);
        stub_float(&mut e, 0x0045_cd80, 10.0);
        stub(&mut e, 0x005b_e5c0, 1);
        stub(&mut e, 0x0096_8670, target);
        stub_float(&mut e, 0x004a_7290, 25.0);
        e.mem.set_f32(0x011c_f7ac + 4, 6.0);
        assert!(fn_009c39e0(&mut e, this));
        assert_eq!(e.get(this, Projectile::fExplosionTimer), 6.0);

        stub_float(&mut e, 0x004a_7290, 200.0);
        e.set(this, Projectile::iFlags, 0x4000);
        assert!(!fn_009c39e0(&mut e, this));
    }

    // The functions below are doubled in `CALLEES` for the earlier tests, so
    // these tests call the translations directly.

    #[test]
    fn light_is_made_from_the_base_light_unless_refused() {
        let mut e = engine();
        let this = projectile(&mut e);
        // No light base: nothing.
        fn_009c3d90(&mut e, this);
        assert!(calls(&e, GEN_DYNAMIC).is_empty());
        set_slot(&mut e, this.addr(), 0x1d0, RETURNS_TRUE);
        stub(&mut e, BASE_LIGHT, 0x4400);
        stub(&mut e, LIGHT_TEST, 0);
        stub(&mut e, GEN_DYNAMIC, 0x5500);
        e.register(ASSIGN_NI_POINTER, |_, _| Ret::default());
        fn_009c3d90(&mut e, this);
        assert_eq!(
            calls(&e, GEN_DYNAMIC),
            vec![vec![0x4400, this.addr(), 1, 0]]
        );
        assert_eq!(
            calls(&e, ASSIGN_NI_POINTER),
            vec![vec![this.addr() + LIGHT, 0x5500]]
        );
        // Refused by 00440d80.
        stub(&mut e, LIGHT_TEST, 1);
        e.call_log = Some(vec![]);
        fn_009c3d90(&mut e, this);
        assert!(calls(&e, GEN_DYNAMIC).is_empty());
    }

    #[test]
    fn light_is_removed_from_the_scene_and_cleared() {
        let mut e = engine();
        let this = projectile(&mut e);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ASSIGN_NI_POINTER, |_, _| Ret::default());
        stub(&mut e, 0x0045_0b80, 0x6600);
        e.register(0x00b5_eed0, |_, _| Ret::default());
        fn_009c3e10(&mut e, this);
        assert!(calls(&e, 0x00b5_eed0).is_empty());
        e.mem.set_u32(this.addr() + LIGHT, 0x7700);
        fn_009c3e10(&mut e, this);
        assert_eq!(calls(&e, 0x0045_0b80), vec![vec![0]]);
        assert_eq!(calls(&e, 0x00b5_eed0), vec![vec![0x6600, 0x7700]]);
        assert_eq!(
            calls(&e, ASSIGN_NI_POINTER),
            vec![vec![this.addr() + LIGHT, 0]]
        );
    }

    #[test]
    fn light_fade_clamps_the_alpha() {
        let mut e = engine();
        let this = projectile(&mut e);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0040_ebd0, |_, a| {
            f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))).into_ret()
        });
        e.register(0x0040_4010, |_, a| {
            f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))).into_ret()
        });
        e.register(0x0050_dd20, |_, _| Ret::default());
        e.mem.set_u32(this.addr() + LIGHT, 0x7700);
        e.mem.set_f32(this.addr() + 0xe0, 1.5);
        // Without flag 0x20: nothing.
        fn_009c3e60(&mut e, this);
        assert!(calls(&e, 0x0050_dd20).is_empty());
        e.mem.set_u32(this.addr() + 0xC8, 0x20);
        fn_009c3e60(&mut e, this);
        assert_eq!(
            calls(&e, 0x0040_ebd0),
            vec![vec![1.0f32.to_bits(), 1.5f32.to_bits()]]
        );
        assert_eq!(calls(&e, 0x0050_dd20), vec![vec![0x7700, 1.0f32.to_bits()]]);
    }

    #[test]
    fn blend_mixes_the_setting_and_the_value() {
        let mut e = engine();
        e.map(SETTING_009C3F50, 8);
        e.mem.set_f32(SETTING_009C3F50 + 4, 10.0);
        // (10 * 0.75 + 2 * 0.25) * 2.
        assert_eq!(fn_009c3f50(&mut e, 2.0, 0.25, 2.0), 16.0);
    }

    #[test]
    fn movement_direction_is_set_and_unitized() {
        let mut e = engine();
        let this = projectile(&mut e);
        stub_float(&mut e, 0x0045_7910, 3.0);
        e.mem.set_f32(this.addr() + 0x110, 1.5);
        fn_009c4e60(&mut e, this, 1, 2, 3, true);
        assert_eq!(e.mem.u32(this.addr() + MOVEMENT_DIRECTION + 8), 3);
        assert_eq!(e.mem.f32(this.addr() + 0x110), 4.5);
        e.register(0x004a_0c10, |_, _| Ret::default());
        fn_009c4e60(&mut e, this, 4, 5, 6, false);
        assert_eq!(e.mem.f32(this.addr() + 0x110), 4.5);
        assert_eq!(
            calls(&e, 0x004a_0c10),
            vec![vec![this.addr() + MOVEMENT_DIRECTION]]
        );
    }

    #[test]
    fn countdown_sound_is_stopped_and_released() {
        let mut e = engine();
        let this = projectile(&mut e);
        for address in [0x00ad_88f0, 0x00ad_8d10] {
            e.register(address, |_, _| Ret::default());
        }
        stub(&mut e, 0x00ad_8ce0, 0);
        projectile_stop_countdown_sound(&mut e, this);
        assert!(calls(&e, 0x00ad_88f0).is_empty());
        sequence(&mut e, 0x00ad_8ce0, vec![1, 1]);
        projectile_stop_countdown_sound(&mut e, this);
        let handle = this.addr() + COUNTDOWN_SOUND;
        assert_eq!(calls(&e, 0x00ad_88f0), vec![vec![handle]]);
        assert_eq!(calls(&e, 0x00ad_8d10), vec![vec![handle]]);
    }

    #[test]
    fn impacts_owners_and_target_are_cleared() {
        let mut e = engine();
        let this = projectile(&mut e);
        // Two impacts, then empty.
        sequence(&mut e, 0x0082_56d0, vec![0, 0, 1]);
        let head = e.mem.alloc(4);
        e.mem.set_u32(head, 0x6100);
        stub(&mut e, 0x0068_15c0, head);
        for address in [0x009b_3490, 0x0063_f7b0, 0x0081_26e0, 0x0080_0190] {
            e.register(address, |_, _| Ret::default());
        }
        let target = e.mem.alloc(0x10);
        stub(&mut e, 0x008d_8b80, target);
        fn_009c4da0(&mut e, this);
        assert_eq!(calls(&e, 0x009b_3490), vec![vec![0x6100, 1]; 2]);
        assert_eq!(calls(&e, 0x0063_f7b0).len(), 2);
        assert_eq!(
            calls(&e, ASSIGN_ACTOR_CAUSE),
            vec![vec![this.addr() + ACTOR_CAUSE, 0]]
        );
        assert_eq!(calls(&e, 0x0080_0190), vec![vec![target]]);
        assert_eq!(calls(&e, 0x0081_26e0), vec![vec![this.addr(), 0]]);
    }

    /// A pick from (0, 0, 10) to (0, 0, -10) over water at `height`.
    fn water_engine(height: f64) -> (Engine, Ptr<Projectile>, u32) {
        let mut e = engine();
        let this = projectile(&mut e);
        stub(&mut e, 0x008d_6f30, 0x4000);
        stub(&mut e, 0x0045_18e0, 1);
        stub_float(&mut e, 0x0054_71e0, height);
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0045_85f0, |e, a| {
            e.mem.set_f32(a[1] + 8, 10.0);
            Ret::default()
        });
        e.register(0x0045_87f0, |e, a| {
            e.mem.set_f32(a[1] + 8, -10.0);
            Ret::default()
        });
        point_subtract(&mut e);
        let fraction = e.mem.alloc(4);
        (e, this, fraction)
    }

    #[test]
    fn auto_water_answers_the_fraction_to_the_surface() {
        let (mut e, this, fraction) = water_engine(5.0);
        assert!(projectile_pick_auto_water(
            &mut e,
            this,
            Ptr::new(0x9000),
            Ptr::new(fraction)
        ));
        // (5 - 10) / (-20).
        assert_eq!(e.mem.f32(fraction), 0.25);
        // Water above the start or below the end: no hit.
        let (mut e, this, fraction) = water_engine(12.0);
        assert!(!projectile_pick_auto_water(
            &mut e,
            this,
            Ptr::new(0x9000),
            Ptr::new(fraction)
        ));
        let (mut e, this, fraction) = water_engine(-12.0);
        assert!(!projectile_pick_auto_water(
            &mut e,
            this,
            Ptr::new(0x9000),
            Ptr::new(fraction)
        ));
        // No water in the cell.
        let (mut e, this, fraction) = water_engine(5.0);
        stub(&mut e, 0x0045_18e0, 0);
        assert!(!projectile_pick_auto_water(
            &mut e,
            this,
            Ptr::new(0x9000),
            Ptr::new(fraction)
        ));
    }

    #[test]
    fn shot_behind_wall_checks_player_shots_only() {
        let (mut e, this, _) = water_engine(5.0);
        let player = actor(&mut e);
        e.map(PLAYER, 4);
        e.set_global(PLAYER, player);
        // Not the player's: false at once.
        assert!(!projectile_check_shot_behind_wall(&mut e, this));
        assert!(calls(&e, 0x004a_3c20).is_empty());
        e.set(this, Projectile::pShooter, player);
        for address in [
            0x004a_3c20,
            0x008c_71b0,
            0x004a_39f0,
            0x004a_3f70,
            0x004a_3da0,
            0x007f_68a0,
            0x004a_3a70,
            0x004a_3fb0,
            0x004a_3bc0,
            0x0097_95c0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let point = e.mem.alloc(0xc);
        set_slot(&mut e, this.addr(), 0x1f4, 0x7D00_0000);
        set_slot(&mut e, player, 0x1f4, 0x7D00_0000);
        stub(&mut e, 0x7D00_0000, point);
        stub_float(&mut e, 0x0070_c3b0, 0.0);
        stub_float(&mut e, 0x0056_7400, 1.0);
        let filter = e.mem.alloc(4);
        stub(&mut e, 0x0093_06d0, 0);
        stub(&mut e, 0x0093_1ed0, filter);
        let world = object(&mut e, 0x10);
        stub(&mut e, 0x0045_43c0, world);
        // The world's cast misses, the water hits (fraction 0.25); 009c0d80
        // (this file's, run for real) finds no hits in the pick (008cdd90
        // answers 0), so the hit is not confirmed and nothing stops.
        stub(&mut e, 0x008c_dd90, 0);
        assert!(!projectile_check_shot_behind_wall(&mut e, this));
        assert_eq!(calls(&e, 0x004a_39f0)[0][1], 6);
        assert_eq!(calls(&e, 0x0093_1ed0)[0][0], player);
        assert_eq!(calls(&e, 0x008c_dd90).len(), 1);
        assert!(calls(&e, 0x0097_95c0).is_empty());
        assert_eq!(calls(&e, 0x004a_3bc0).len(), 1);
    }

    #[test]
    fn process_level_changes_by_cell_state() {
        let mut e = engine();
        let this = projectile(&mut e);
        e.map(TES, 4);
        e.set_global(TES, 0x4100u32);
        e.map(0x011c_3b3c, 4);
        e.map(0x011c_3f2c, 4);
        e.map(0x011d_df38, 4);
        stub(&mut e, 0x008d_6f30, 0x4000);
        stub(&mut e, 0x0045_cd60, 3);
        for address in [0x0040_fbf0, 0x0040_fba0, 0x0096_d470, 0x0084_9730] {
            e.register(address, |_, _| Ret::default());
        }
        // A light base and a loaded cell: high level (vtable +0x240).
        set_slot(&mut e, this.addr(), 0x1d0, RETURNS_TRUE);
        stub(&mut e, 0x0045_11e0, 1);
        assert!(projectile_change_process_level(&mut e, this));
        assert_eq!(calls(&e, SLOT_BASE + 0x240), vec![vec![this.addr()]]);
        // Not loaded but middle-high possible: vtable +0x24C.
        set_slot(&mut e, this.addr(), 0x324, RETURNS_TRUE);
        sequence(&mut e, 0x0045_11e0, vec![0, 1]);
        assert!(projectile_change_process_level(&mut e, this));
        assert_eq!(calls(&e, SLOT_BASE + 0x24C), vec![vec![this.addr()]]);
        // Already at none: nothing more.
        stub(&mut e, 0x0045_11e0, 0);
        e.call_log = Some(vec![]);
        assert!(projectile_change_process_level(&mut e, this));
        assert!(calls(&e, 0x0096_d470).is_empty());
        // From high to none: the process removed and deleted.
        let process = object(&mut e, 0x10);
        e.set(this, Projectile::pCurrentProcess, process);
        stub(&mut e, 0x0045_cd60, 0);
        stub(&mut e, 0x0046_9860, 0);
        assert!(projectile_change_process_level(&mut e, this));
        assert_eq!(
            calls(&e, 0x0096_d470),
            vec![vec![PROCESS_LISTS, this.addr(), 0]]
        );
        assert_eq!(calls(&e, SLOT_BASE), vec![vec![process, 1]]);
        assert_eq!(e.get(this, Projectile::pCurrentProcess), 0);
        assert_eq!(calls(&e, SLOT_BASE + 0x2FC), vec![vec![this.addr()]]);
        // A form the data handler marks is unloaded and deleted instead.
        e.set(this, Projectile::pCurrentProcess, process);
        stub(&mut e, 0x0046_9860, 1);
        e.call_log = Some(vec![]);
        assert!(!projectile_change_process_level(&mut e, this));
        assert_eq!(calls(&e, 0x0084_9730).len(), 1);
        assert_eq!(calls(&e, SLOT_BASE + 0xC4), vec![vec![this.addr(), 1]]);
    }
}

#[cfg(test)]
mod n026_tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];
    type Logged = Vec<(u32, Vec<u32>)>;

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn word(value: u32) -> Ret {
        value.into_ret()
    }

    /// An object whose vtable slots answer the given results (each slot a
    /// double at `vtable + 0x10_0000 + slot`).
    fn object(e: &mut Engine, vtable: u32, slots: &[(u32, Ret)]) -> u32 {
        let size = slots.iter().map(|(s, _)| *s).max().unwrap_or(0) / 4 + 1;
        let mut words = vec![0u32; size as usize];
        for (slot, ret) in slots {
            let target = vtable + 0x10_0000 + slot;
            words[*slot as usize / 4] = target;
            let ret = *ret;
            e.register_double(target, move |_, _| ret);
        }
        e.put_vtable(vtable, &words);
        let object = e.mem.alloc(0x700);
        e.mem.set_u32(object, vtable);
        object
    }

    /// `BSSimpleList` doubles: a node is `{item, next}`; empty when both
    /// are 0.
    fn lists(e: &mut Engine) {
        e.register(LIST_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_ITEM, |_, a| a[0].into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
    }

    /// Puts `items` (impact data with `pCollidee` from the list) into the
    /// projectile's impact list.
    fn impacts(e: &mut Engine, this: u32, collidees: &[u32]) -> Vec<u32> {
        let mut data = Vec::new();
        let mut node = this + IMPACT_LIST;
        for (i, collidee) in collidees.iter().enumerate() {
            let item = e.mem.alloc(0x30);
            e.mem.set_u32(item, *collidee);
            data.push(item);
            e.mem.set_u32(node, item);
            if i + 1 < collidees.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        data
    }

    fn projectile(e: &mut Engine, slots: &[(u32, Ret)]) -> Ptr<Projectile> {
        Ptr::new(object(e, 0x0a00_0000, slots))
    }

    #[test]
    fn impact_lookup_walks_the_list() {
        let mut e = Engine::new();
        lists(&mut e);
        let this = projectile(&mut e, &[]);
        assert_eq!(e.call(0x009c_3ee0, &args![this, 0x11u32]).u32(), 0);
        let data = impacts(&mut e, this.addr(), &[0x11, 0x22, 0x22]);
        assert_eq!(e.call(0x009c_3ee0, &args![this, 0x11u32]).u32(), data[0]);
        assert_eq!(e.call(0x009c_3ee0, &args![this, 0x22u32]).u32(), data[1]);
        assert_eq!(e.call(0x009c_3ee0, &args![this, 0x33u32]).u32(), 0);
    }

    #[test]
    fn ring_points() {
        let mut e = Engine::new();
        e.map(0x0101_f000, 0x1000);
        e.set_global(0x0101_ff48, std::f64::consts::TAU);
        e.register(CIRCLE_X, |_, a| st0(f64::from(f32::from_bits(a[0])) + 1.0));
        e.register(CIRCLE_Y, |_, _| st0(0.5));
        e.register(0x0041_6870, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register(0x0045_bb20, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) * f32::from_bits(a[2]);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            a[1].into_ret()
        });
        e.register(0x0043_9e90, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            a[1].into_ret()
        });
        let out = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(
            0x009c_3f90,
            &args![out, 2i32, 10.0f32, 20.0f32, 30.0f32, 1.0f32, 2.0f32, 3.0f32, 2.0f32, 0.0f32],
        );
        let log = e.call_log.take().unwrap();
        // Point 1: the angle is 1/2 of tau.
        let first_cosines = calls(&log, CIRCLE_X);
        assert_eq!(first_cosines.len(), 6);
        assert_eq!(
            first_cosines[3][0],
            x87_store32(x87_mul(0.5, std::f64::consts::TAU))
        );
        // Point 0: angle 0, p = (0 + 1) * 3 + 2 = 5, q = 0.5 * 3 + 1 = 2.5;
        // the direction (cos q sin p, sin q sin p, -cos p) = (3.5 * 0.5,
        // 0.5 * 0.5, -6), times 2, plus the center.
        let directions = calls(&log, 0x0041_6870);
        assert_eq!(f32::from_bits(directions[0][1]), 1.75);
        assert_eq!(f32::from_bits(directions[0][2]), 0.25);
        assert_eq!(f32::from_bits(directions[0][3]), -6.0);
        assert_eq!(
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
            [13.5, 20.5, 18.0]
        );
        assert_ne!(e.mem.u32(out + 0xc), 0);
    }

    struct Mine {
        base_ok: u32,
        flags: u32,
        silent: u32,
        timer: f32,
        target_is_shooter: bool,
        actor_shooter: u32,
    }

    fn mine(world: Mine) -> (bool, Ptr<Projectile>, Logged) {
        let mut e = Engine::new();
        e.map(0x0102_3000, 0x1000);
        e.set_global(0x0102_31b0, f64::from(f32::MAX));
        e.map(0x011c_e000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        let player = object(&mut e, 0x0a10_0000, &[(0x488, word(0))]);
        e.set_global(PLAYER, player);
        let shooter = object(&mut e, 0x0a20_0000, &[(0x100, word(world.actor_shooter))]);
        let this = projectile(&mut e, &[(0x1f4, word(0x0b00_0000))]);
        e.set(this, Projectile::pShooter, shooter);
        e.set(this, Projectile::iFlags, world.flags);
        e.mem.set_f32(this.addr() + 0xe4, world.timer);
        e.register(GET_BASE_OBJECT, |_, _| word(0x0c00_0000));
        let base_ok = world.base_ok;
        e.register_double(0x004f_df20, move |_, _| word(base_ok));
        e.register(TEST_FLAGS, |e, a| {
            u32::from(e.mem.u32(a[0] + 0xc8) & a[1] != 0).into_ret()
        });
        e.register(SET_FLAGS, |e, a| {
            let flags = e.mem.u32(a[0] + 0xc8);
            e.mem.set_u32(
                a[0] + 0xc8,
                if a[2] != 0 {
                    flags | a[1]
                } else {
                    flags & !a[1]
                },
            );
            Ret::default()
        });
        e.register(0x00ad_8ce0, |_, _| Ret::default());
        e.register(0x0058_6150, |_, _| word(0x0d00_0000));
        e.register(0x0051_1840, |_, _| word(0x0d10_0000));
        e.register(0x0045_3a70, |_, _| word(0x0d20_0000));
        quiet(
            &mut e,
            &[
                0x00ad_7480,
                0x0068_a7d0,
                0x00ad_8830,
                0x0048_3710,
                0x004d_5c60,
            ],
        );
        e.register(0x0056_7790, |_, _| word(0x0e00_0000));
        e.register(0x0040_1170, |_, _| word(8));
        e.register(0x0043_d4d0, |e, a| {
            let value = if a[0] == 0x011c_ecdc { 3 } else { 0 };
            e.mem.set_u32(a[0] + 4, value);
            (a[0] + 4).into_ret()
        });
        e.register_double(GET_SHOOTER, move |_, _| word(shooter));
        let target = if world.target_is_shooter {
            shooter
        } else {
            0x0f00_0000
        };
        e.call_log = Some(vec![]);
        let armed = e
            .call(0x009c_43e0, &args![this, target, world.silent])
            .bool();
        let log = e.call_log.take().unwrap();
        (armed, this, log)
    }

    #[test]
    fn mines_arm_once() {
        let (armed, _, log) = mine(Mine {
            base_ok: 0,
            flags: 0,
            silent: 0,
            timer: 1.0,
            target_is_shooter: false,
            actor_shooter: 1,
        });
        assert!(!armed);
        assert!(calls(&log, 0x00ad_8ce0).is_empty());
        let (armed, _, log) = mine(Mine {
            base_ok: 1,
            flags: 0x200,
            silent: 0,
            timer: 1.0,
            target_is_shooter: false,
            actor_shooter: 1,
        });
        assert!(!armed);
        assert!(calls(&log, 0x00ad_8ce0).is_empty());
    }

    #[test]
    fn an_armed_mine_plays_its_sound_and_counts() {
        let (armed, _, log) = mine(Mine {
            base_ok: 1,
            flags: 0,
            silent: 0x100,
            timer: 1.0,
            target_is_shooter: false,
            actor_shooter: 1,
        });
        assert!(armed);
        let played = calls(&log, 0x00ad_7480);
        assert_eq!(played[0][0], 0x0d20_0000);
        assert_eq!(played[0][2..], [0x0d10_0000, 0x4102, 0x0d00_0000]);
        assert_eq!(calls(&log, 0x0068_a7d0)[0][1], 0x0b00_0000);
        assert_eq!(calls(&log, 0x00ad_8830)[0][1], 0);
        assert_eq!(calls(&log, 0x004d_5c60), vec![vec![0xb]]);
        assert_eq!(calls(&log, 0x0a20_0488)[0][1], 3);
        assert!(calls(&log, 0x0056_7790).is_empty());

        // Silent: no sound, no statistic.
        let (armed, _, log) = mine(Mine {
            base_ok: 1,
            flags: 0,
            silent: 1,
            timer: 1.0,
            target_is_shooter: false,
            actor_shooter: 0,
        });
        assert!(armed);
        assert!(calls(&log, 0x00ad_7480).is_empty());
        assert!(calls(&log, 0x004d_5c60).is_empty());
        assert_eq!(calls(&log, 0x0056_7790).len(), 1);

        // No timer and no reaction to the shooter itself: no statistic.
        let (armed, _, log) = mine(Mine {
            base_ok: 1,
            flags: 0,
            silent: 0,
            timer: f32::MAX,
            target_is_shooter: true,
            actor_shooter: 1,
        });
        assert!(armed);
        assert!(calls(&log, 0x004d_5c60).is_empty());
    }

    #[test]
    fn process_changes() {
        for (address, size, constructor, level) in [
            (0x009c_47d0u32, 0x46cu32, 0x008d_7510u32, 0u32),
            (0x009c_4920, 0x25c, 0x0091_3fe0, 1),
            (0x009c_4a40, 0xc8, 0x0092_c950, 2),
            (0x009c_4b60, 0xb4, 0x0090_6dc0, 3),
        ] {
            let mut e = Engine::new();
            let this = projectile(&mut e, &[(0x1fc, word(0))]);
            let old = object(&mut e, 0x0a40_0000, &[(0, word(0)), (4, word(0))]);
            let new = object(&mut e, 0x0a50_0000, &[(4, word(0)), (0x48, word(0))]);
            e.mem.set_u32(this.addr() + 0x68, old);
            quiet(
                &mut e,
                &[0x0040_fbf0, 0x0040_fba0, 0x0096_d470, 0x0096_d450],
            );
            e.register(0x0045_cd60, |_, _| word(5));
            e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register_double(constructor, move |_, _| word(new));
            e.call_log = Some(vec![]);
            assert!(e.call(address, &args![this]).bool());
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, ALLOCATE), vec![vec![size]]);
            assert_eq!(e.mem.u32(this.addr() + 0x68), new);
            assert_eq!(
                calls(&log, 0x0096_d470),
                vec![vec![PROCESS_LISTS, this.addr(), 5]]
            );
            assert_eq!(calls(&log, 0x0a60_0004), vec![vec![new, old]]);
            assert_eq!(calls(&log, 0x0a50_0000), vec![vec![old, 1]]);
            assert_eq!(
                calls(&log, 0x0096_d450),
                vec![vec![PROCESS_LISTS, this.addr(), level, 0, 0, 0]]
            );
            assert_eq!(calls(&log, 0x0040_fbf0), vec![vec![PROCESS_LOCK, 0]]);
            assert_eq!(calls(&log, 0x0040_fba0), vec![vec![PROCESS_LOCK]]);
            let high = usize::from(address == 0x009c_47d0);
            assert_eq!(calls(&log, 0x0a10_01fc).len(), high);
            assert_eq!(calls(&log, 0x0a60_0048).len(), high);
        }
    }

    /// Doubles that make `fn_009bc8f0` disable the projectile through
    /// `00574400` (counted).
    fn kills(e: &mut Engine) {
        e.register(BOOL_SETTING_VALUE, |_, _| word(0x0101_0000));
        e.map(0x0101_0000, 0x1000);
        e.register(0x0040_1170, |_, _| word(0));
        e.register(GET_FORM_ID, |_, _| word(0x1234));
        e.map(0x011c_3000, 0x1000);
        e.register(0x0046_9860, |_, _| word(0));
        quiet(e, &[0x0057_4400]);
    }

    #[test]
    fn forgetting_a_reference() {
        let mut e = Engine::new();
        lists(&mut e);
        kills(&mut e);
        let this = projectile(&mut e, &[]);
        impacts(&mut e, this.addr(), &[0x11, 0x22, 0x22]);
        e.register(GET_DESIRED_TARGET, |_, _| word(0x22));
        e.register(TEST_FLAGS, |_, a| u32::from(a[1] == 0x400).into_ret());
        quiet(&mut e, &[0x0042_0ba0, 0x0081_26e0, 0x0080_0190, DEALLOCATE]);
        e.register(GET_SHOOTER, |_, _| word(0x22));
        let vats = e.mem.alloc(8);
        e.mem.set_u32(vats, 0x22);
        e.register_double(0x008d_8b80, move |_, _| word(vats));
        e.call_log = Some(vec![]);
        e.call(0x009c_4c80, &args![this, 0x22u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0042_0ba0), vec![vec![this.addr(), 0]]);
        // The target (flag 0x400), one impact and the shooter.
        assert_eq!(calls(&log, 0x0057_4400).len(), 3);
        assert_eq!(calls(&log, 0x0080_0190), vec![vec![vats]]);
        assert_eq!(calls(&log, DEALLOCATE), vec![vec![vats]]);
        assert_eq!(calls(&log, 0x0081_26e0), vec![vec![this.addr(), 0]]);

        e.call_log = Some(vec![]);
        e.call(0x009c_4c80, &args![this, 0x33u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, 0x0057_4400).is_empty());
        assert!(calls(&log, 0x0081_26e0).is_empty());
    }

    #[test]
    fn countdown_sound_and_target_object() {
        let mut e = Engine::new();
        let this = projectile(&mut e, &[]);
        let playing = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let flag = playing.clone();
        e.register_double(0x00ad_8ce0, move |_, _| word(flag.get()));
        e.register(GET_BASE_OBJECT, |_, _| word(0x0c00_0000));
        e.register(0x0081_0fd0, |_, _| word(0x0c10_0000));
        e.register(GET_FORM_ID, |_, _| word(0x777));
        e.register(0x0093_3150, |_, a| a[1].into_ret());
        quiet(&mut e, &[0x0041_8900, 0x0048_3710]);
        e.call_log = Some(vec![]);
        e.call(0x009c_4ec0, &args![this]);
        playing.set(0);
        e.call(0x009c_4ec0, &args![this]);
        let log = e.call_log.take().unwrap();
        let played = calls(&log, 0x0093_3150);
        assert_eq!(played.len(), 1);
        assert_eq!(played[0][2..], [0x777, 0, 0x4000_0102, 1]);
        assert_eq!(
            calls(&log, 0x0041_8900),
            vec![vec![this.addr() + COUNTDOWN_SOUND, played[0][1]]]
        );

        let target = e.mem.alloc(0x14);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(ZERO_POINT, 0x0abc);
        quiet(&mut e, &[0x0063_3c90]);
        e.mem.set_u32(target, 0xffff);
        assert_eq!(e.call(0x009c_5ee0, &args![target]).u32(), target);
        assert_eq!(e.mem.u32(target), 0);
        assert_eq!(e.mem.u32(target + 8), 0x0abc);
    }

    #[test]
    fn target_points() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(ZERO_POINT, 0x0abc);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0046_1130, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0052_4c40, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            a[1].into_ret()
        });
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(bound, 0x0bbb);
        e.register_double(0x0043_d450, move |_, _| word(bound));
        e.register(LIST_ITEM, |_, a| a[0].into_ret());
        let position = e.mem.alloc(0x10);
        e.mem.set_u32(position, 0x0ccc);
        let rooted = object(&mut e, 0x0a70_0000, &[(0x1d0, word(1))]);
        let bare = object(
            &mut e,
            0x0a80_0000,
            &[(0x1d0, word(0)), (0x1f4, word(position))],
        );
        let target = e.mem.alloc(0x14);
        let out = e.mem.alloc(0xc);
        assert_eq!(e.call(0x009c_5f30, &args![target, out]).u32(), out);
        assert_eq!(e.mem.u32(out), 0x0abc);
        e.mem.set_u32(target, bare);
        e.call(0x009c_5f30, &args![target, out]);
        assert_eq!(e.mem.u32(out), 0x0ccc);
        e.mem.set_u32(target, rooted);
        e.call(0x009c_5f30, &args![target, out]);
        assert_eq!(e.mem.u32(out), 0x0bbb);
        e.mem.set_u32(target + 4, 0x5000);
        e.call(0x009c_5f30, &args![target, out]);
        assert_eq!(e.mem.u32(out), 0x5010);
    }

    fn buffer(e: &mut Engine, version: u32) -> u32 {
        object(e, 0x0a90_0000, &[(0, word(version))])
    }

    #[test]
    fn save_writes_the_fields_in_order() {
        let mut e = Engine::new();
        lists(&mut e);
        let this = projectile(&mut e, &[]);
        impacts(&mut e, this.addr(), &[0x11, 0x22]);
        let buffer = buffer(&mut e, 0x10);
        e.set(this, Projectile::pWeaponSource, 0x41);
        e.set(this, Projectile::pShooter, 0x42);
        e.set(this, Projectile::pDesiredTarget, 0x43);
        quiet(
            &mut e,
            &[
                0x0093_2880,
                SAVE_BYTES,
                0x0086_5df0,
                0x0086_5ee0,
                0x0086_5ff0,
                0x004b_ed60,
            ],
        );
        e.register(0x0086_5f20, |_, _| word(0x99));
        e.register(POINTER_GET, |_, _| word(0));
        // Impact data saves: the doubles of `ImpactData::SaveGame`'s callees.
        e.register(0x0086_5df0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009c_4ff0, &args![this, buffer]);
        let log = e.call_log.take().unwrap();
        let saved: Vec<_> = calls(&log, SAVE_BYTES)
            .iter()
            .map(|w| (w[1].wrapping_sub(this.addr()), w[2]))
            .collect();
        let base = [
            (0xc8, 4),
            (0xcc, 4),
            (0xd4, 4),
            (0xd0, 4),
            (0xd8, 4),
            (0xdc, 4),
            (0xe0, 4),
            (0xe4, 4),
            (0xe8, 4),
            (0x104, 0xc),
            (0x110, 4),
            (0x118, 1),
            (0x124, 4),
            (0xb8, 0xc),
            (0xc4, 4),
            (0xec, 4),
            (0xf0, 4),
            (0xf4, 4),
        ];
        assert_eq!(saved[..base.len()], base);
        assert_eq!(saved.last().unwrap(), &(0x90, 1));
        let forms: Vec<_> = calls(&log, 0x0086_5df0)
            .iter()
            .take(3)
            .map(|w| w[1])
            .collect();
        assert_eq!(forms, [0x41, 0x42, 0x43]);
        assert_eq!(calls(&log, 0x0086_5ff0), vec![vec![buffer, 2, 0x99]]);
        assert!(calls(&log, 0x004b_ed60).is_empty());
    }

    #[test]
    fn load_reads_by_version() {
        for version in [2u32, 0xd, 0x10] {
            let mut e = Engine::new();
            lists(&mut e);
            let this = projectile(&mut e, &[(0x1d0, word(0))]);
            let buffer = buffer(&mut e, version);
            quiet(
                &mut e,
                &[
                    0x0093_2a00,
                    LOAD_BYTES,
                    0x0086_48e0,
                    0x0086_4a20,
                    ASSIGN_ACTOR_CAUSE,
                ],
            );
            e.register(0x0086_48a0, |_, _| word(0x51));
            e.register(CAST_SOURCE_004839C0, |_, a| (a[0] + 1).into_ret());
            e.register(RT_DYNAMIC_CAST, |_, a| {
                assert_eq!(a[2..], [TYPE_TES_FORM, TYPE_TES_OBJECT_WEAP, 0]);
                (a[0] + 1).into_ret()
            });
            e.register(0x0066_e330, |_, a| (a[0] ^ 0x1000).into_ret());
            e.register(0x0086_4a60, |_, _| word(0));
            e.call_log = Some(vec![]);
            e.call(0x009c_52f0, &args![this, buffer]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.get(this, Projectile::pWeaponSource), 0x53);
            let loaded: Vec<_> = calls(&log, LOAD_BYTES)
                .iter()
                .map(|w| w[1].wrapping_sub(this.addr()))
                .collect();
            assert_eq!(loaded.contains(&0xb8), version >= 0xd);
            assert_eq!(loaded.contains(&0x90), version >= 0x10);
            assert_eq!(calls(&log, 0x0086_4a60).len(), usize::from(version >= 0xe));
            assert_eq!(
                calls(&log, ASSIGN_ACTOR_CAUSE),
                vec![vec![this.addr() + ACTOR_CAUSE, u32::MAX ^ 0x1000]]
            );
        }
    }

    #[test]
    fn load_restores_the_item_change_and_impacts() {
        let mut e = Engine::new();
        lists(&mut e);
        let this = projectile(&mut e, &[(0x1d0, word(0))]);
        let buffer = buffer(&mut e, 0xe);
        quiet(
            &mut e,
            &[
                0x0093_2a00,
                0x0086_48e0,
                0x0086_4a20,
                ASSIGN_ACTOR_CAUSE,
                0x004b_ee00,
            ],
        );
        e.register(LOAD_BYTES, |e, a| {
            if a[2] == 1 {
                e.mem.set_u8(a[1], 1);
            }
            Ret::default()
        });
        e.register(0x0086_48a0, |_, _| word(0));
        e.register(CAST_SOURCE_004839C0, |_, _| word(0));
        e.register(RT_DYNAMIC_CAST, |_, _| word(0));
        e.register(0x0066_e330, |_, _| word(0));
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0076_b630, |_, a| a[0].into_ret());
        e.register(0x0044_ddc0, |_, _| word(0));
        quiet(&mut e, &[0x004b_c780, 0x0044_59e0]);
        e.register(0x0086_4a60, |_, _| word(2));
        // The impact constructor and load are this file's own; their callees:
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0086_4980, |e, a| {
            if a[2] == 1 {
                e.mem.set_u8(a[1], 1);
            }
            Ret::default()
        });
        quiet(&mut e, &[0x0086_48e0, 0x0090_5820, 0x0043_90c0]);
        e.map(0x011f_4000, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x009c_52f0, &args![this, buffer]);
        let log = e.call_log.take().unwrap();
        // The item change has no object: dropped and deleted.
        let item = calls(&log, 0x004b_ee00)[0][0];
        assert_eq!(calls(&log, 0x0044_59e0), vec![vec![item, 1]]);
        assert_eq!(e.get(this, Projectile::pSecondaryItemChange), 0);
        let appended = calls(&log, 0x0090_5820);
        assert_eq!(appended.len(), 2);
        assert_eq!(appended[0][0], this.addr() + IMPACT_LIST);
        let sizes: Vec<_> = calls(&log, ALLOCATE).iter().map(|w| w[0]).collect();
        assert_eq!(sizes, [0xc, 0x30, 0x30]);
    }

    #[test]
    fn finish_load_and_fix_up_steps() {
        let mut e = Engine::new();
        lists(&mut e);
        let this = projectile(&mut e, &[]);
        impacts(&mut e, this.addr(), &[0x11, 0x22]);
        let buffer = buffer(&mut e, 0xe);
        e.set(this, Projectile::pShooter, 0x61);
        e.set(this, Projectile::pSecondaryItemChange, 0x71);
        quiet(
            &mut e,
            &[0x0093_2b70, 0x004b_ef60, 0x009a_4f40, 0x0093_2d60],
        );
        e.register(CAST_SOURCE_004839C0, |_, a| (a[0] + 1).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| {
            assert_eq!(a[2..], [TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0]);
            (a[0] + 1).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x009c_5740, &args![this, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(this, Projectile::pShooter), 0x63);
        assert_eq!(e.get(this, Projectile::pDesiredTarget), 0);
        assert_eq!(calls(&log, CAST_SOURCE_004839C0).len(), 1);
        assert_eq!(calls(&log, 0x004b_ef60), vec![vec![0x71, buffer]]);
        assert_eq!(calls(&log, 0x009a_4f40).len(), 2);

        // 009c5870: each impact's form-id fix-up (`009bbb40`, this file's,
        // with no target world objects it does nothing).
        e.call_log = Some(vec![]);
        e.call(0x009c_5870, &args![this, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0093_2d60), vec![vec![this.addr(), buffer]]);
    }

    #[test]
    fn reset_for_load() {
        let mut e = Engine::new();
        lists(&mut e);
        let this = projectile(&mut e, &[(0x308, word(0))]);
        impacts(&mut e, this.addr(), &[0x11]);
        e.map(0x0101_6000, 0x1000);
        e.mem.set_f32(FLOAT_MAX, f32::MAX);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(ZERO_POINT, 0x0abc);
        e.set(this, Projectile::pSecondaryItemChange, 0x71);
        e.set(this, Projectile::iFlags, 0xff);
        quiet(
            &mut e,
            &[
                0x0093_2f60,
                0x009b_3490,
                ASSIGN_NI_POINTER,
                ASSIGN_ACTOR_CAUSE,
                0x00a8_6bf0,
                0x0044_59e0,
            ],
        );
        e.register(0x0063_f7b0, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(0x0056_2d00, |_, _| word(1));
        e.call_log = Some(vec![]);
        e.call(0x009c_58e0, &args![this, 0x0b00_0000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x009b_3490).len(), 1);
        assert_eq!(e.get(this, Projectile::iFlags), 0);
        assert_eq!(e.get(this, Projectile::fPower), 1.0);
        assert_eq!(e.get(this, Projectile::fExplosionTimer), f32::MAX);
        assert_eq!(e.mem.u32(this.addr() + MOVEMENT_DIRECTION), 0x0abc);
        assert_eq!(calls(&log, 0x0044_59e0), vec![vec![0x71, 1]]);
        assert_eq!(e.get(this, Projectile::pSecondaryItemChange), 0);
        assert_eq!(calls(&log, 0x0a10_0308).len(), 1);
    }

    #[test]
    fn pick_casts_the_node_and_its_children() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(0x0101_2070, 1.0f64);
        let collector = object(&mut e, 0x0aa0_0000, &[(0, word(0))]);
        e.register_double(0x008c_dd90, move |_, _| word(collector));
        let child = object(&mut e, 0x0ab0_0000, &[(0xc, word(0))]);
        let node = object(&mut e, 0x0ac0_0000, &[(0xc, word(0x0d00_0000))]);
        e.register(0x0043_b480, |_, _| word(1));
        e.register_double(0x0043_b4a0, move |_, _| word(child));
        e.register(0x0043_b610, |_, a| a[0].into_ret());
        e.register(0x006f_a820, |_, a| (a[0] + 1).into_ret());
        let world_object = object(&mut e, 0x0ad0_0000, &[(0xf4, word(0))]);
        e.register_double(0x006f_a820, move |_, _| word(world_object));
        e.register(0x004a_e6a0, |_, _| word(0x0e00_0000));
        e.register(0x0043_b4f0, |_, _| word(0x0e10_0000));
        e.register(0x0062_4070, |_, _| word(1));
        e.register(0x0043_b300, |_, _| word(0));
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0045_8820, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x004a_e750, |_, a| (a[0] + 2).into_ret());
        e.register(0x0063_2c60, |_, _| word(0x0e20_0000));
        e.register(0x0056_09f0, |_, _| word(1));
        e.register(0x0046_0140, |_, a| (a[0] + 3).into_ret());
        quiet(
            &mut e,
            &[
                0x0056_ded0,
                0x0063_2ca0,
                0x004a_3d20,
                0x00c7_f690,
                0x0063_2c30,
                0x00c8_55f0,
            ],
        );
        e.call_log = Some(vec![]);
        let hit = e
            .call(0x009c_5ba0, &args![0x0f00_0000u32, node, 0u32, 1u32])
            .bool();
        let log = e.call_log.take().unwrap();
        assert!(hit);
        // The node and its child each give the collector a hit.
        assert_eq!(calls(&log, 0x0ab0_0000 + 0x10_0000 + 0xc).len(), 1);
        assert_eq!(calls(&log, 0x0aa0_0000 + 0x10_0000).len(), 2);
        assert_eq!(
            calls(&log, 0x0aa0_0000 + 0x10_0000)[0][1],
            world_object + 2 + 3
        );
        // Flush once, at the top.
        assert_eq!(calls(&log, 0x00c8_55f0), vec![vec![collector]]);
        // The world object's transform (not a `01267e64` collision object).
        assert_eq!(calls(&log, 0x0ad0_0000 + 0x10_0000 + 0xf4).len(), 2);
        // A null node casts nothing.
        let mut e2 = Engine::new();
        assert!(!e2.call(0x009c_5ba0, &args![0u32, 0u32, 0u32, 0u32]).bool());
    }

    #[test]
    fn aim_at_point_and_orientation() {
        for flat in [0.0f64, 1.0] {
            let mut e = Engine::new();
            e.map(0x0101_2000, 0x1000);
            e.set_global(0x0101_2060, 0.0f64);
            e.map(0x0101_8000, 0x1000);
            e.set_global(0x0101_8c00, 2.5f64);
            let position = e.mem.alloc(0xc);
            let this = projectile(
                &mut e,
                &[
                    (0x1f4, word(position)),
                    (0x2bc, st0(0.25)),
                    (0x2c4, word(0)),
                ],
            );
            e.register(0x0043_9ef0, |e, a| {
                for i in 0..3 {
                    let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                    e.mem.set_f32(a[1] + 4 * i, value);
                }
                a[1].into_ret()
            });
            quiet(&mut e, &[0x004a_0c10, 0x0093_1d90]);
            e.register(0x0093_1d70, |_, _| st0(0.125));
            e.register(GET_BASE_OBJECT, |_, _| word(0x0c00_0000));
            e.register_double(0x0096_6980, move |_, _| st0(flat));
            e.register(0x004b_1460, |_, a| {
                st0(f64::from(f32::from_bits(a[0])) * 2.0)
            });
            e.register(0x004b_13c0, |_, _| st0(1.0));
            e.register(0x0040_8840, |_, a| {
                st0(f64::from(f32::from_bits(a[0])).abs())
            });
            e.call_log = Some(vec![]);
            e.call(0x009c_6030, &args![this, 0.0f32, 0.0f32, 0.5f32]);
            let log = e.call_log.take().unwrap();
            let heading = calls(&log, 0x0a10_02c4);
            assert_eq!(heading.len(), 1);
            assert_eq!(f32::from_bits(heading[0][1]), 1.0);
            let looking = f32::from_bits(calls(&log, 0x0093_1d90)[0][1]);
            if flat == 0.0 {
                // -(0.5 / (1 - 0.25)) * 2.
                let ratio = x87_store32(x87_div(0.5, x87_sub(1.0, 0.25)));
                assert_eq!(looking, -(f32::from_bits(ratio) * 2.0));
                assert!(calls(&log, 0x0040_8840).is_empty());
            } else {
                assert_eq!(looking, 0.125);
                assert_eq!(f32::from_bits(calls(&log, 0x0040_8840)[0][0]), 0.75);
            }
        }

        let mut e = Engine::new();
        let this = projectile(&mut e, &[(0x2c4, word(0))]);
        e.register(0x00a5_9400, |e, a| {
            e.mem.set_f32(a[1], 1.5);
            e.mem.set_f32(a[2], -0.5);
            Ret::default()
        });
        quiet(&mut e, &[0x0093_1d90]);
        e.call_log = Some(vec![]);
        e.call(0x009c_6330, &args![this, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0a10_02c4)[0][1], 1.5f32.to_bits());
        assert_eq!(calls(&log, 0x0093_1d90)[0][1], (-0.5f32).to_bits());
    }

    #[test]
    fn collision_moves_between_worlds() {
        for (controller, cell_world, current) in [
            (0x0b00_0000u32, 0x0c00_0000u32, 0x0d00_0000u32),
            (0x0b00_0000, 0x0c00_0000, 0x0c00_0000),
            (0, 0x0c00_0000, 0),
        ] {
            let mut e = Engine::new();
            let this = projectile(&mut e, &[(0x1d0, word(0x0e00_0000))]);
            e.register_double(0x0093_06d0, move |_, _| word(controller));
            e.register(TEST_FLAGS, |_, _| word(1));
            e.register(0x0043_fcd0, |_, _| word(0x0f00_0000));
            e.register(0x008d_6f30, |_, _| word(0x0f10_0000));
            let world = object(&mut e, 0x0ae0_0000, &[(0xd0, word(0))]);
            let cell_world = if cell_world != 0 { world } else { 0 };
            e.register_double(0x0045_43c0, move |_, _| word(cell_world));
            let current = if current == 0x0c00_0000 {
                cell_world
            } else {
                current
            };
            e.register_double(0x0062_1ad0, move |_, _| word(current));
            quiet(
                &mut e,
                &[
                    0x0088_b0a0,
                    0x0057_b1d0,
                    0x0056_20e0,
                    0x00c6_d7a0,
                    0x00c6_9ee0,
                    0x00c6_9f50,
                ],
            );
            e.register(0x0054_71e0, |_, _| st0(3.0));
            e.register(0x0043_6aa0, |_, _| word(0x4444));
            e.register(0x0081_0660, |_, _| word(0));
            e.register(0x004b_5260, |_, _| word(0));
            e.call_log = Some(vec![]);
            e.call(0x009c_6160, &args![this, 0u32]);
            let log = e.call_log.take().unwrap();
            if controller != 0 && current != cell_world {
                assert_eq!(calls(&log, 0x0057_b1d0)[0][1], 3.0f32.to_bits());
                assert_eq!(calls(&log, 0x0056_20e0)[0][1], 0x4444);
                assert_eq!(calls(&log, 0x00c6_9ee0), vec![vec![0x0f00_0000, 1, 0]]);
                assert_eq!(calls(&log, 0x0ae0_00d0 + 0x10_0000).len(), 1);
            } else if controller != 0 {
                assert!(calls(&log, 0x0088_b0a0).is_empty());
                assert_eq!(calls(&log, 0x00c6_9f50), vec![vec![0x0f00_0000, 1, 0]]);
            } else {
                // No controller: the 3D has no collision object, so world 0.
                assert!(calls(&log, 0x00c6_9ee0).is_empty());
                assert_eq!(
                    calls(&log, 0x0ae0_00d0 + 0x10_0000),
                    vec![vec![world, 0x0f00_0000, 1, 1, 0, 0]]
                );
            }
            assert_eq!(calls(&log, 0x00c6_d7a0).len(), usize::from(controller != 0));
        }
        // Neither a controller nor flag 4: nothing.
        let mut e = Engine::new();
        let this = projectile(&mut e, &[]);
        e.register(0x0093_06d0, |_, _| word(0));
        e.register(TEST_FLAGS, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x009c_6160, &args![this, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
    }

    #[test]
    fn controller_constructor() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x00c7_41e0]);
        let this = e.mem.alloc(0x660);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x009c_6440, &args![this, 0x11u32, 0x22u32]).u32(),
            this
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x00c7_41e0), vec![vec![this, 0x11]]);
        assert_eq!(e.mem.u32(this), 0x0109_0594);
        assert_eq!(e.mem.u32(this + 0x3e0), 0x0109_0584);
        assert_eq!(e.mem.u32(this + 0x410), 0x0109_0568);
        assert_eq!(e.mem.u32(this + 0x650), 0x22);
    }
}

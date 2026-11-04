//! `fallout/ai/combat/combatutilities.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CombatUtilities` is a set of free (cdecl) functions the combat AI uses
//! for distances, firing locations, projected target points and line-of-sight
//! tests. They work on references (`TESObjectREFR`) and actors through their
//! virtual functions, which are called by vtable offset here.
//!
//! x87 note: the code is x87. Values are computed in `f64` and rounded to
//! `f32` at each `float` store; comparisons follow the exact status-word test
//! the code makes (`fcom` / `parity_even`), so NaN takes the branch the game
//! takes.

#[allow(unused_imports)]
use crate::prelude::*;

// ---- callees outside this unit ----------------------------------------------------------------

/// `00403e20` (thiscall on a `Setting`): the address of the setting's value
/// (`this + 4`), or of a shared zero float when `this` is null.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `00404010` (cdecl, two floats): the larger of the two (the second when
/// they compare unordered).
const FLOAT_MAX: u32 = 0x0040_4010;
/// `00408840` (cdecl, one float): `00408860` on the same value.
const FLOAT_ABS_2: u32 = 0x0040_8840;
/// `00408860` (cdecl, one float): `fabs` (`00ec6cde`) rounded to a float.
const FLOAT_ABS: u32 = 0x0040_8860;
/// `004169a0` (cdecl: angle, `*sin`, `*cos`): `FSINCOS` of the angle.
const SIN_COS: u32 = 0x0041_69a0;
/// `00439180` (thiscall on an `NiPoint3`, one float): scales the point in
/// place, returns `this`.
const POINT_SCALE: u32 = 0x0043_9180;
/// `00439e90` (thiscall on an `NiPoint3`: result, other): `result = this +
/// other`, returns `result`.
const POINT_ADD: u32 = 0x0043_9e90;
/// `00439ef0` (thiscall on an `NiPoint3`: result, other): `result = this -
/// other`, returns `result`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00457990` (thiscall on an `NiPoint3`): its length.
const POINT_LENGTH: u32 = 0x0045_7990;
/// `004a0c10` (thiscall on an `NiPoint3`): makes it unit length in place
/// (zero when its length is at most the constant at `01017cf8`).
const POINT_UNITIZE: u32 = 0x004a_0c10;
/// `004aae30` (cdecl: node, name).
const FIND_NODE: u32 = 0x004a_ae30;
/// `GetUnclampedZAngleFromVector` (map, Xbox PDB; cdecl, one `NiPoint3*`).
const GET_UNCLAMPED_Z_ANGLE_FROM_VECTOR: u32 = 0x004b_1550;
/// `TESObjectREFR::GetScale` (map, Xbox PDB; thiscall).
const REFR_GET_SCALE: u32 = 0x0056_7400;
/// `TESObjectREFR::GetDistanceFromReference` (map, Xbox PDB; thiscall,
/// three words: the other reference and two flags).
const REFR_GET_DISTANCE_FROM_REFERENCE: u32 = 0x0057_23b0;
/// `005a2030` (thiscall): the byte at `this + 0x14d`.
const REFR_BYTE_14D: u32 = 0x005a_2030;
/// `005f2420` (thiscall).
const ANIM_GROUP_005F2420: u32 = 0x005f_2420;
/// `006447f0` (thiscall on a weapon): a float.
const WEAPON_VALUE_6447F0: u32 = 0x0064_47f0;
/// `006450c0` (thiscall on a weapon): a flag.
const WEAPON_FLAG_6450C0: u32 = 0x0064_50c0;
/// `CombatFormulas::CalcWeaponReach` (map, Xbox PDB; cdecl, one float).
const CALC_WEAPON_REACH: u32 = 0x0064_63e0;
/// `Animation::ZeroGlobalTransform` (map name; thiscall). The map name may
/// come from folded code; the body is not used here.
const ANIMATION_0048F7F0: u32 = 0x0048_f7f0;
/// `MiddleHighProcess::GetSavedAcquireObject` (map name; thiscall on an
/// actor, as this unit calls it).
const ACTOR_8D8520: u32 = 0x008d_8520;
/// `Actor::GetCurrentWeapon` (map, Xbox PDB; thiscall).
const ACTOR_GET_CURRENT_WEAPON: u32 = 0x008a_1710;
/// `Actor::GetForwardLength` (map, Xbox PDB; thiscall).
const ACTOR_GET_FORWARD_LENGTH: u32 = 0x008b_e420;
/// `008e26a0` (thiscall on what vtable slot `0x428` returns): a float.
const PROCESS_ANGLE_8E26A0: u32 = 0x008e_26a0;
/// `00950bb0` (thiscall on the player, one word).
const PLAYER_950BB0: u32 = 0x0095_0bb0;
/// `00963730` (thiscall on the player).
const PLAYER_963730: u32 = 0x0096_3730;
/// `00965620` (thiscall on the player, seven words).
const PLAYER_965620: u32 = 0x0096_5620;

// ---- globals and constants of the exe ---------------------------------------------------------

/// The player character pointer (`PlayerCharacter*`; the functions this
/// unit calls on it are `playercharacter.cpp`'s in the engine map).
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// A zero `NiPoint3` in `.bss` (three floats).
const ZERO_POINT: u32 = 0x011f_426c;
/// `"ProjectileNode"`.
const PROJECTILE_NODE_NAME: u32 = 0x0102_cb10;

/// `double` `3.4028234663852886e38` (`FLT_MAX`).
const DOUBLE_FLT_MAX: u32 = 0x0102_31b0;
/// `double` `-1.0`.
const DOUBLE_MINUS_ONE: u32 = 0x0101_a6b0;
/// `double` `0.5`.
const DOUBLE_HALF: u32 = 0x0101_1588;
/// `double` `0.01745329...` (pi / 180, as a float).
const DOUBLE_DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `double` `57.2957763671875` (180 / pi, as a float).
const DOUBLE_RADIANS_TO_DEGREES: u32 = 0x0102_f248;
/// `double` `128.0`.
const DOUBLE_128: u32 = 0x0102_e430;
/// `double` `180.0`.
const DOUBLE_180: u32 = 0x0104_ed58;
/// `double` `360.0`.
const DOUBLE_360: u32 = 0x0103_c348;
/// `double` `3.0`.
const DOUBLE_3: u32 = 0x0102_1928;
/// `float` `1.1754944e-38` (`FLT_MIN`).
const FLOAT_FLT_MIN: u32 = 0x0102_efc4;

/// `fAICombatSlopeDifference` (`Setting`; name from its initializer
/// `00f63ab0`).
const SETTING_AI_COMBAT_SLOPE_DIFFERENCE: u32 = 0x011c_e4a0;
/// `fCombatOverheadHitConeAngle` (initializer `00f61890`).
const SETTING_COMBAT_OVERHEAD_HIT_CONE_ANGLE: u32 = 0x011c_eb80;
/// `fCombatSweepHitConeAngle` (initializer `00f61860`).
const SETTING_COMBAT_SWEEP_HIT_CONE_ANGLE: u32 = 0x011c_fa04;
/// `fCombatUppercutHitConeAngle` (initializer `00f618c0`).
const SETTING_COMBAT_UPPERCUT_HIT_CONE_ANGLE: u32 = 0x011c_fe44;
/// `fCombatHitConeAngle` (initializer `00f61680`).
const SETTING_COMBAT_HIT_CONE_ANGLE: u32 = 0x011c_fa58;
/// `fCombatDeadActorHitConeMult` (initializer `00f616b0`).
const SETTING_COMBAT_DEAD_ACTOR_HIT_CONE_MULT: u32 = 0x011c_fda4;

// ---- x87 helpers ------------------------------------------------------------------------------

/// The high byte of the x87 status word (`FNSTSW AX`, then `AH`) after
/// `FCOM st0, operand`: C0 (`0x01`) when `st0 < operand`, C3 (`0x40`) when
/// equal, nothing when greater, C0 | C2 | C3 (`0x45`) when unordered.
fn fcom(st0: f64, operand: f64) -> u8 {
    if st0.is_nan() || operand.is_nan() {
        0x45
    } else if st0 < operand {
        0x01
    } else if st0 == operand {
        0x40
    } else {
        0x00
    }
}

/// The parity flag after `TEST AH, mask` (set when the masked byte has an
/// even number of 1 bits), which `JP` / `JNP` test.
fn parity_even(status: u8, mask: u8) -> bool {
    (status & mask).count_ones() % 2 == 0
}

/// `FLD dword [address]`.
fn load_f32(e: &Engine, address: u32) -> f64 {
    e.mem.f32(address) as f64
}

/// `FSTP dword [address]` of an x87 value.
fn store_f32(e: &mut Engine, address: u32, value: f64) {
    e.mem.set_f32(address, value as f32);
}

/// Copies a float with `FLD` / `FSTP`.
fn copy_f32(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// Copies three words with integer moves (an `NiPoint3` copied by value).
fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// The value of a float `Setting` (`[SETTING_VALUE(setting)]`).
fn setting_f32(e: &mut Engine, setting: u32) -> f64 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    load_f32(e, value)
}

/// A float the game pushes as an argument after `FSTP dword [ESP]`.
fn float_arg(value: f64) -> f32 {
    value as f32
}

fn is_actor(e: &mut Engine, reference: Ptr) -> bool {
    // TESObjectREFR vtable +0x100: true for actors (the code casts to the
    // actor after it).
    e.vcall(reference.addr(), 0x100, &[]).bool()
}

// ---- more callees outside this unit -----------------------------------------------------------

/// `00403d30` (cdecl: destination, byte, count): fills memory.
const MEMORY_FILL: u32 = 0x0040_3d30;
/// `00401050` (stdcall: array, element size, count, constructor): constructs
/// each element of an array.
const CONSTRUCT_ARRAY: u32 = 0x0040_1050;
/// `00408d60` (thiscall on a `Setting`): the address of a bool setting's
/// value (`this + 4`), or of a shared zero byte when `this` is null.
const SETTING_BOOL_VALUE: u32 = 0x0040_8d60;
/// `0040ebd0` (cdecl, two floats): the smaller of the two, as the code uses
/// it next to `FLOAT_MAX`.
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// `00414430` (thiscall on an `NiColorA`, four floats): sets r, g, b, a and
/// returns `this`.
const COLOR_SET: u32 = 0x0041_4430;
/// `00416870` (thiscall on an `NiPoint3`, three floats): sets x, y, z.
const POINT_SET: u32 = 0x0041_6870;
/// `0042f5a0` (cdecl, two words): the smaller (as `min`).
const WORD_MIN: u32 = 0x0042_f5a0;
/// `00430830` (thiscall on a reference): `this + 0x24` (its rotation, as
/// `GetAngleToProjectedTarget` subtracts it from angles).
const REFR_ANGLE_430830: u32 = 0x0043_0830;
/// `00436aa0` (thiscall on a reference): `this + 0x30` (its position).
const REFR_POSITION_436AA0: u32 = 0x0043_6aa0;
/// `0043d4d0` (thiscall on a `Setting`): the address of an integer
/// setting's value (`this + 4`), or of a shared zero when `this` is null.
const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
/// `00440460` (thiscall, one `NiPoint3*`): sets the object's translation.
const NODE_SET_TRANSLATE: u32 = 0x0044_0460;
/// `00441110` (thiscall): the target map of the object at `011f1958`.
const TARGET_MAP_441110: u32 = 0x0044_1110;
/// `0044ddc0` (thiscall on an array): the word at +8 (its element count).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// `NiPoint3::UnitizeGetLength` (map, Xbox PDB; thiscall).
const POINT_UNITIZE_GET_LENGTH: u32 = 0x0045_7910;
/// `TES::AddTempDebugObject` (map, Xbox PDB; thiscall: object, seconds).
const TES_ADD_TEMP_DEBUG_OBJECT: u32 = 0x0045_8e20;
/// `0045bb20` (thiscall on an `NiPoint3`: result, factor): `result = this *
/// factor`, returns `result`.
const POINT_TIMES: u32 = 0x0045_bb20;
/// `0045bb80` (thiscall on a node): `this + 0x8c` (its world translation).
const NODE_WORLD_TRANSLATE: u32 = 0x0045_bb80;
/// `00452dc0` (thiscall on an `NiPoint2`, two floats): sets x and y.
const POINT2_SET: u32 = 0x0045_2dc0;
/// `BGSExplosion::GetRadiusBSUnits` (map, Xbox PDB; thiscall).
const EXPLOSION_GET_RADIUS: u32 = 0x0047_7900;
/// `004181e0` (thiscall on a projectile): its base form.
const PROJECTILE_BASE_4181E0: u32 = 0x0041_81e0;
/// `004579e0` (cdecl, one float; wraps `004019b0`). Unconfirmed: the code
/// uses it as a square root.
const MATH_4579E0: u32 = 0x0045_79e0;
/// `004997b0` (thiscall on an actor): a flag (sneaking, as the code then
/// asks `Actor::GetSneakHeight`).
const ACTOR_FLAG_4997B0: u32 = 0x0049_97b0;
/// `004a0db0` (thiscall on a weapon): a float (its minimum range, as stored).
const WEAPON_VALUE_4A0DB0: u32 = 0x004a_0db0;
/// `004a39f0` (thiscall, one word): stores the low 7 bits into the word.
const FILTER_SET_LAYER: u32 = 0x004a_39f0;
/// `004a3a20` (thiscall on a word): its high 16 bits.
const FILTER_HIGH_HALF: u32 = 0x004a_3a20;
/// `hkpAllRayHitCollector::hkpAllRayHitCollector` (map, Xbox PDB).
const RAY_COLLECTOR_CONSTRUCT: u32 = 0x004a_3a70;
/// `hkpAllRayHitCollector::~hkpAllRayHitCollector` (map, Xbox PDB).
const RAY_COLLECTOR_DESTRUCT: u32 = 0x004a_3bc0;
/// `004a3c20` (thiscall): constructs the 0xb0-byte pick data.
const PICK_DATA_CONSTRUCT: u32 = 0x004a_3c20;
/// `004a3da0` (thiscall on the pick data, `NiPoint3*`): sets the start.
const PICK_DATA_SET_START: u32 = 0x004a_3da0;
/// `004a3eb0` (thiscall on the pick data, `NiPoint3*`): sets the end.
const PICK_DATA_SET_END: u32 = 0x004a_3eb0;
/// `004a3f70` (thiscall on the pick data, one word): sets the filter.
const PICK_DATA_SET_FILTER: u32 = 0x004a_3f70;
/// `004a3fb0` (thiscall on the pick data, collector): sets the collector.
const PICK_DATA_SET_COLLECTOR: u32 = 0x004a_3fb0;
/// `MakeTriPoint` (map, Xbox PDB; cdecl: size, color, flag).
const MAKE_TRI_POINT: u32 = 0x004b_29b0;
/// `004b3800` (thiscall on an `NiPoint3`: result, other).
const POINT_CROSS: u32 = 0x004b_3800;
/// `004b3890` (cdecl, five words): a debug line object.
const MAKE_LINE: u32 = 0x004b_3890;
/// `GetZAngleFromVector` (map, Xbox PDB; cdecl, one `NiPoint3*`).
const GET_Z_ANGLE_FROM_VECTOR: u32 = 0x004b_13c0;
/// `ClampAngle` (map, Xbox PDB; cdecl, one float).
const CLAMP_ANGLE: u32 = 0x004b_1480;
/// `004b15e0` (cdecl: angle, angle, `float*`): an angle difference.
const ANGLE_DIFFERENCE: u32 = 0x004b_15e0;
/// `NiASin` (map, Xbox PDB; cdecl, one float).
const ASIN: u32 = 0x004b_54c0;
/// `004c0c30` (thiscall on a weapon): a flag (ranged, as
/// `IsActorUsingRanged` returns it).
const WEAPON_FLAG_4C0C30: u32 = 0x004c_0c30;
/// `004e4470` (cdecl, one float; wraps the unidentified `00ec9f30`).
/// Unconfirmed: the code uses it as a cosine.
const TRIG_4E4470: u32 = 0x004e_4470;
/// `004e44b0` (cdecl, one float; wraps the unidentified `00eca060`).
/// Unconfirmed: the code uses it as a sine.
const TRIG_4E44B0: u32 = 0x004e_44b0;
/// `004fd3c0` (thiscall on a projectile): the word at +0x84, which the code
/// uses as an explosion (`BGSExplosion::GetRadiusBSUnits` is called on it).
const PROJECTILE_EXPLOSION: u32 = 0x004f_d3c0;
/// `004fd3e0` (thiscall on what vtable slot `0x428` returns).
const CONTROLLER_4FD3E0: u32 = 0x004f_d3e0;
/// `004fd420` (thiscall, one word): a flag test.
const FLAG_TEST_4FD420: u32 = 0x004f_d420;
/// `00508050` (thiscall): a float.
const VALUE_508050: u32 = 0x0050_8050;
/// `00525a90` (thiscall on a weapon, one word): its projectile.
const WEAPON_PROJECTILE: u32 = 0x0052_5a90;
/// `BGSWorldLocation::IsLocationLoaded` (map, Xbox PDB; thiscall).
const WORLD_LOCATION_IS_LOADED: u32 = 0x0052_7a00;
/// `TESObjectCELL::Pick` (map, Xbox PDB; thiscall, pick data).
const CELL_PICK: u32 = 0x0055_3ee0;
/// `TESObjectREFR::GetRefPersists` (map, Xbox PDB; thiscall).
const REFR_GET_REF_PERSISTS: u32 = 0x0056_53d0;
/// `TESObjectREFR::FindReferenceFor3D` (map, Xbox PDB; cdecl).
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `00579670` (thiscall on an actor): a flag.
const ACTOR_FLAG_579670: u32 = 0x0057_9670;
/// `0057dd70` (cdecl, one float; wraps the unidentified `00eca590`).
/// Unconfirmed: the code uses it as a tangent (a height from an angle).
const TRIG_57DD70: u32 = 0x0057_dd70;
/// `00589850` (thiscall on an `NiPoint3`): the length of its x and y.
const POINT_LENGTH_XY: u32 = 0x0058_9850;
/// `0059ceb0` (thiscall, one pointer): stores it into the pick data.
const PICK_DATA_SET_RAY_COLLECTOR: u32 = 0x0059_ceb0;
/// `0059ce80` (thiscall on a word, one word): stores the value into the
/// high 16 bits.
const FILTER_SET_HIGH_HALF: u32 = 0x0059_ce80;
/// `0059cee0` (thiscall): destroys the collector `fn_009a7bc0` built.
const LOS_COLLECTOR_DESTRUCT: u32 = 0x0059_cee0;
/// `0062a220` (thiscall): the collector's base constructor.
const COLLECTOR_BASE_CONSTRUCT: u32 = 0x0062_a220;
/// `0062a3b0` (thiscall, two words).
const COLLECTOR_INIT: u32 = 0x0062_a3b0;
/// `00646f50` (thiscall on a weapon): a float.
const WEAPON_VALUE_646F50: u32 = 0x0064_6f50;
/// `00647790` (thiscall on a weapon): a flag.
const WEAPON_FLAG_647790: u32 = 0x0064_7790;
/// `006479a0` (cdecl, three floats).
const EXPLOSION_FORMULA_6479A0: u32 = 0x0064_79a0;
/// `bhkCharacterProxy::GetLinearVelocity` (map, Xbox PDB; thiscall,
/// `NiPoint3*`).
const CHARACTER_PROXY_GET_LINEAR_VELOCITY: u32 = 0x0066_ca00;
/// `0069ef80` (thiscall on a projectile): the float at +0x68, used as its
/// speed where the speed argument is 0.
const PROJECTILE_SPEED: u32 = 0x0069_ef80;
/// `006815c0` (thiscall on an `NiPoint3`): returns `this` (its default
/// constructor).
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
/// `006a78f0` (thiscall on an explosion): the float at +0x78 (its damage, as the
/// settings it is compared with say).
const EXPLOSION_DAMAGE: u32 = 0x006a_78f0;
/// `006a7ad0` (thiscall on an array, index): the address of an element.
const ARRAY_ELEMENT: u32 = 0x006a_7ad0;
/// `006b99e0` (thiscall on an `NiPoint2`, other): the dot product.
const POINT2_DOT: u32 = 0x006b_99e0;
/// `006d5320`: the constructor `fn_009a9a20` passes for its 0x18-byte
/// entries.
const ENTRY_CONSTRUCT: u32 = 0x006d_5320;
/// `006e4820` (thiscall on a cover edge): a flag.
const EDGE_FLAG_6E4820: u32 = 0x006e_4820;
/// `007031c0` (thiscall on what vtable slot `0x428` returns): a weapon.
const CONTROLLER_WEAPON: u32 = 0x0070_31c0;
/// `00755660` (cdecl, one float; wraps `004b1460`). Unconfirmed: the code
/// uses it as an arctangent (an angle from a slope).
const MATH_755660: u32 = 0x0075_5660;
/// `Actor::GetHeight` (map, Xbox PDB; thiscall).
const ACTOR_GET_HEIGHT: u32 = 0x0088_53a0;
/// `Actor::GetSneakHeight` (map, Xbox PDB; thiscall, one word).
const ACTOR_GET_SNEAK_HEIGHT: u32 = 0x0088_54d0;
/// `Actor::GetRadius` (map, Xbox PDB; thiscall).
const ACTOR_GET_RADIUS: u32 = 0x008b_e1b0;
/// `008d6f30` (thiscall on a reference): the word at +0x40 (its cell).
const REFR_PARENT_CELL: u32 = 0x008d_6f30;
/// `MobileObject::GetCharController` (map, Xbox PDB; thiscall).
const MOBILE_GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `MobileObject::GetLooking` (map, Xbox PDB; thiscall).
const MOBILE_GET_LOOKING: u32 = 0x0093_1d70;
/// `00931ed0` (thiscall on a reference, word out): its collision filter.
const REFR_COLLISION_FILTER: u32 = 0x0093_1ed0;
/// `MobileObject::GetActualHeading` (map, Xbox PDB; thiscall).
const MOBILE_GET_ACTUAL_HEADING: u32 = 0x0093_6da0;
/// `PlayerCharacter::GetBiped` (map, Xbox PDB; thiscall, one word).
const PLAYER_GET_BIPED: u32 = 0x0095_0b00;
/// `00966980` (thiscall on a projectile): 1.0 when its flag 0x20000 is set
/// (`004fd420`), otherwise the float at +0x64 (its gravity).
const PROJECTILE_GRAVITY: u32 = 0x0096_6980;
/// `009669c0` (thiscall on a projectile): a float.
const PROJECTILE_VALUE_9669C0: u32 = 0x0096_69c0;
/// `0097ae90` (thiscall on a combat controller): its actor.
const CONTROLLER_ACTOR: u32 = 0x0097_ae90;
/// `00980070` (thiscall on a combat controller).
const CONTROLLER_980070: u32 = 0x0098_0070;
/// `CombatController::GetTargetSegmentInView` (map, Xbox PDB; thiscall).
const CONTROLLER_GET_TARGET_SEGMENT_IN_VIEW: u32 = 0x0098_1920;
/// `CombatPlan::ContainsAction` (map, Xbox PDB; thiscall, one word).
const CONTAINS_ACTION: u32 = 0x0098_1940;
/// `00982be0` (thiscall on a cover edge): a flag.
const EDGE_FLAG_982BE0: u32 = 0x0098_2be0;
/// `CombatTargetMap::GetTargetAttackerArray` (map, Xbox PDB; thiscall, one
/// word).
const GET_TARGET_ATTACKER_ARRAY: u32 = 0x009a_4250;
/// `NiAVObject::UpdateProperties` (map, Xbox PDB; thiscall).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `009c0d80` (cdecl, six words): runs a projectile pick and returns what
/// it hit.
const PROJECTILE_PICK: u32 = 0x009c_0d80;
/// `00483710` (thiscall): returns at once (the code calls it on `011f1c7c`
/// at the start and end of the line-of-sight tests).
const NO_OP_483710: u32 = 0x0048_3710;
/// `_ftol2_sse` (`00ec62c0`): truncates the value in ST0 (passed as an
/// `f64`).
const FTOL: u32 = 0x00ec_62c0;

// ---- more globals and constants ---------------------------------------------------------------

/// The object `00483710` is called on.
const NO_OP_OBJECT: u32 = 0x011f_1c7c;
/// The `TES` object (`TES::AddTempDebugObject`'s `this`).
const TES_OBJECT: u32 = 0x011d_ea10;
/// A reference pointer the debug geometry is limited to (0: every actor).
const DEBUG_REFERENCE: u32 = 0x011d_ea24;
/// The object whose target map `fn_009a9a20` reads (`00441110`'s `this`).
const TARGET_MAP_OWNER: u32 = 0x011f_1958;
/// A float factor the projectile's gravity is multiplied by.
const GRAVITY_FACTOR: u32 = 0x011f_1c00;
/// The number of sectors around a target (`int`, 30 in the exe).
const SECTOR_COUNT: u32 = 0x011a_558c;
/// How many neighbouring sectors on each side a taken sector also blocks
/// (`int`, 2 in the exe).
const SECTOR_SPREAD: u32 = 0x011a_5590;
/// `float` 1024.0: the distance a projected point is at least moved away.
const MIN_PROJECTION_DISTANCE: u32 = 0x011a_5594;
/// The up vector `(0, 0, 1)`.
const UP_VECTOR: u32 = 0x011a_9484;
/// `vftable` of the line-of-sight ray collector `fn_009a7bc0` builds.
const LOS_COLLECTOR_VTABLE: u32 = 0x0104_e1dc;

/// `double` `2pi` (as a float).
const DOUBLE_TWO_PI: u32 = 0x0101_ff48;
/// `double` `pi` (as a float).
const DOUBLE_PI: u32 = 0x0101_ff40;
/// `double` `-pi` (as a float).
const DOUBLE_MINUS_PI: u32 = 0x0101_ff58;
/// `double` `pi / 2` (as a float).
const DOUBLE_HALF_PI: u32 = 0x0103_0f38;
/// `double` `0.0`.
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// `double` `1.0`.
const DOUBLE_ONE: u32 = 0x0101_2070;
/// `double` `2.0`.
const DOUBLE_TWO: u32 = 0x0101_1590;
/// `double` `0.25`.
const DOUBLE_QUARTER: u32 = 0x0102_90b0;
/// `double` `0.75`.
const DOUBLE_THREE_QUARTERS: u32 = 0x0101_de30;
/// `double` `0.9` (as a float).
const DOUBLE_NINE_TENTHS: u32 = 0x0106_b9e8;
/// `double` `0.1` (as a float).
const DOUBLE_TENTH: u32 = 0x0101_ffa0;
/// `double` `16.0`.
const DOUBLE_16: u32 = 0x0102_40b8;
/// `double` `256.0`.
const DOUBLE_256: u32 = 0x0102_31d8;
/// `double` `8192.0`.
const DOUBLE_8192: u32 = 0x0108_4d28;
/// `float` `pi`.
const FLOAT_PI: u32 = 0x0108_e374;
/// `float` `3.4028235e38` (`FLT_MAX`).
const FLOAT_FLT_MAX: u32 = 0x0101_6970;
/// `float` `512.0`.
const FLOAT_512: u32 = 0x0102_31a0;
/// `float` `1024.0`.
const FLOAT_1024: u32 = 0x0102_36e0;
/// `float` `64.0`.
const FLOAT_64: u32 = 0x0101_e6ec;
/// `float` `128.0`.
const FLOAT_128: u32 = 0x0101_e704;
/// `float` `5.0`.
const FLOAT_5: u32 = 0x0101_712c;

/// `fGrenadeThrowHitFractionThreshold` (initializer `00f5e440`).
const SETTING_GRENADE_THROW_HIT_FRACTION_THRESHOLD: u32 = 0x011c_f95c;
/// `fGrenadeHighArcSpeedPercentage` (initializer `00f5e0e0`).
const SETTING_GRENADE_HIGH_ARC_SPEED_PERCENTAGE: u32 = 0x011c_fb78;
/// `iBallisticProjectilePathPickSegments` (initializer `00f5e410`).
const SETTING_BALLISTIC_PROJECTILE_PATH_PICK_SEGMENTS: u32 = 0x011c_fd7c;
/// `iAimingNumIterations` (initializer `00f62a90`).
const SETTING_AIMING_NUM_ITERATIONS: u32 = 0x011c_fc08;
/// `bDebugCombatProjectileLOS:Combat` (initializer `00fac580`).
const SETTING_DEBUG_COMBAT_PROJECTILE_LOS: u32 = 0x011f_1c4c;
/// `fDebugCombatProjectileLOSTime:Combat` (initializer `00fac5b0`).
const SETTING_DEBUG_COMBAT_PROJECTILE_LOS_TIME: u32 = 0x011f_1c24;
/// `fCombatSplashDamageMaxSpeed` (initializer `00f5dc30`).
const SETTING_COMBAT_SPLASH_DAMAGE_MAX_SPEED: u32 = 0x011c_ebd4;
/// `fCombatSplashDamageMinRadius` (initializer `00f5dbd0`).
const SETTING_COMBAT_SPLASH_DAMAGE_MIN_RADIUS: u32 = 0x011c_f270;
/// `fCombatSplashDamageMinDamage` (initializer `00f5dc00`).
const SETTING_COMBAT_SPLASH_DAMAGE_MIN_DAMAGE: u32 = 0x011c_eb5c;
/// `fCombatAbsoluteMaxRangeMult` (initializer `00f5e530`).
const SETTING_COMBAT_ABSOLUTE_MAX_RANGE_MULT: u32 = 0x011c_e87c;
/// `fCombatProjectileMaxRangeOptimalMult` (initializer `00f5e590`).
const SETTING_COMBAT_PROJECTILE_MAX_RANGE_OPTIMAL_MULT: u32 = 0x011c_eab0;
/// `fDangerousProjectileExplosionDamage` (initializer `00f5dc90`).
const SETTING_DANGEROUS_PROJECTILE_EXPLOSION_DAMAGE: u32 = 0x011c_f3f0;
/// `fDangerousProjectileExplosionRadius` (initializer `00f5dcc0`).
const SETTING_DANGEROUS_PROJECTILE_EXPLOSION_RADIUS: u32 = 0x011c_ea74;

/// An `NiPoint3` passed by value: three words (x, y, z), copied with
/// integer moves, so the bits are kept.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PointArg(pub [u32; 3]);

impl PointArg {
    /// The three words at `address`.
    pub fn load(e: &Engine, address: u32) -> Self {
        PointArg([
            e.mem.u32(address),
            e.mem.u32(address.wrapping_add(4)),
            e.mem.u32(address.wrapping_add(8)),
        ])
    }

    /// Writes the three words to `address`.
    pub fn store(self, e: &mut Engine, address: u32) {
        for (i, word) in self.0.iter().enumerate() {
            e.mem.set_u32(address.wrapping_add(4 * i as u32), *word);
        }
    }

    /// A point from three floats.
    pub fn from_floats(x: f32, y: f32, z: f32) -> Self {
        PointArg([x.to_bits(), y.to_bits(), z.to_bits()])
    }
}

impl Arg for PointArg {
    const WORDS: usize = 3;
    fn take(words: &[u32], i: &mut usize) -> Self {
        let x = u32::take(words, i);
        let y = u32::take(words, i);
        let z = u32::take(words, i);
        PointArg([x, y, z])
    }
    fn put(self, out: &mut Vec<u32>) {
        out.extend_from_slice(&self.0);
    }
}

/// Folds the float at `address` into -pi..pi the way the code does: below
/// or at `-pi` it gains `2pi`, above `pi` it loses `2pi` (NaN is left).
fn wrap_angle(e: &mut Engine, address: u32) {
    let minus_pi: f64 = e.global(DOUBLE_MINUS_PI);
    let pi: f64 = e.global(DOUBLE_PI);
    let two_pi: f64 = e.global(DOUBLE_TWO_PI);
    let angle = load_f32(e, address);
    if !parity_even(fcom(angle, minus_pi), 0x41) {
        store_f32(e, address, angle + two_pi);
    } else if fcom(angle, pi) & 0x41 == 0 {
        store_f32(e, address, angle - two_pi);
    }
}

/// The value of an integer `Setting` (`[SETTING_INT_VALUE(setting)]`).
fn setting_u32(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_INT_VALUE, &args![setting]).u32();
    e.mem.u32(value)
}

/// A projectile's gravity times the factor at `011f1c00`, as the code
/// computes it before storing it as a float.
fn projectile_gravity(e: &mut Engine, projectile: Ptr) -> f64 {
    let gravity = e.call(PROJECTILE_GRAVITY, &args![projectile]).f64();
    gravity * load_f32(e, GRAVITY_FACTOR)
}

/// `result = base + direction * factor` through the game's point helpers
/// (`0045bb20` into `scaled`, then `00439e90` into `sum`); returns the
/// address `00439e90` returns.
fn add_scaled(
    e: &mut Engine,
    base: u32,
    direction: u32,
    factor: f32,
    scaled: u32,
    sum: u32,
) -> u32 {
    let scaled = e.call(POINT_TIMES, &args![direction, scaled, factor]).u32();
    e.call(POINT_ADD, &args![base, sum, scaled]).u32()
}

/// `00446390` (thiscall on a weapon): the signed byte at +0xf4 (its type,
/// as `GetAttackerFiringLocation` compares it with 10 to 13).
const WEAPON_TYPE_446390: u32 = 0x0044_6390;
/// `004eaf60` (thiscall on the player): the byte at +0x64a.
const PLAYER_4EAF60: u32 = 0x004e_af60;
/// `004a7290` (thiscall on an `NiPoint3`): `x*x + y*y + z*z`.
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;

/// The ray cast `CalculateProjectileLOS` repeats per segment: sets the
/// segment on the pick data, picks in the attacker's cell, runs the
/// projectile pick and draws the debug line when `bDebugCombatProjectileLOS`
/// is set (for the reference at `011d_ea24`, the player or every actor when
/// that is 0). Returns the hit object.
#[allow(clippy::too_many_arguments)]
fn projectile_los_segment(
    e: &mut Engine,
    attacker: Ptr,
    projectile: Ptr,
    pick: u32,
    start: u32,
    end: u32,
    fraction: u32,
    picked: u32,
) -> u32 {
    e.call(PICK_DATA_SET_START, &args![pick, start]);
    e.call(PICK_DATA_SET_END, &args![pick, end]);
    let cell = e.call(REFR_PARENT_CELL, &args![attacker]).u32();
    let cell_hit = e.call(CELL_PICK, &args![cell, pick]).u32();
    e.mem.set_u32(picked, cell_hit);
    let far = x87_quiet_bits(e.mem.u32(FLOAT_FLT_MAX));
    let hit = e
        .call(
            PROJECTILE_PICK,
            &args![pick, 0u32, projectile, fraction, 0u32, far],
        )
        .u32();
    let debug = e
        .call(
            SETTING_BOOL_VALUE,
            &args![SETTING_DEBUG_COMBAT_PROJECTILE_LOS],
        )
        .u32();
    if e.mem.u8(debug) != 0 {
        let selected: u32 = e.global(DEBUG_REFERENCE);
        let player: u32 = e.global(PLAYER_CHARACTER);
        if selected == 0 || selected == player || selected == attacker.addr() {
            let fraction_bits = x87_quiet_bits(e.mem.u32(fraction));
            combat_utilities_add_los_debug_geomtry(
                e,
                Ptr::new(start),
                Ptr::new(end),
                hit,
                f32::from_bits(fraction_bits),
                Ptr::NULL,
                Ptr::NULL,
            );
        }
    }
    hit
}

/// The tail `GetAttackerFiringLocation` and `fn_009a8250` share: the
/// reference's position (vtable +0x1f4) raised by its firing z offset.
fn raised_position(e: &mut Engine, result: Ptr, reference: Ptr) -> Ptr {
    let position = e.vcall(reference.addr(), 0x1f4, &[]).u32();
    let point = PointArg::load(e, position);
    let raise = combat_utilities_get_attacker_firing_z_offset(e, reference, false);
    let z = (raise as f64 + f32::from_bits(point.0[2]) as f64) as f32;
    PointArg([point.0[0], point.0[1], z.to_bits()]).store(e, result.addr());
    result
}

/// Bit `sector` (the shift count masked to 5 bits, as `SHL` does).
fn sector_bit(sector: i8) -> u32 {
    1u32.wrapping_shl(sector as i32 as u32)
}

/// Gives each recorded attacker whose byte +0x15 equals `flagged` a free
/// sector (`fn_009a98a0`), marks it taken and stores the angle through
/// `fn_009a9f00` on its controller (the attacker's own angle when it got
/// its own sector).
fn assign_sectors(e: &mut Engine, entries: u32, count: u32, mask: &mut u32, flagged: bool) {
    let mut index: u32 = 0;
    while index < count {
        let entry = entries + index * 0x18;
        if (e.mem.u8(entry + 0x15) != 0) == flagged {
            let own = e.mem.i8(entry + 0x14);
            let sector = fn_009a98a0(e, *mask, own);
            if sector != -1 {
                *mask = fn_009a9940(e, *mask, sector);
                let mut angle = f32::from_bits(x87_quiet_bits(e.mem.u32(entry + 0x10)));
                if sector != own {
                    angle = fn_009a99f0(e, sector);
                }
                let controller = e.mem.u32(entry);
                fn_009a9f00(e, Ptr::new(controller), angle);
            }
        }
        index += 1;
    }
}

// ---- translations -----------------------------------------------------------------------------

// Translated from 009a64d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetDistanceBetweenFacingActors` (Xbox PDB): the
/// distance between the two references (`GetDistanceFromReference`), less
/// their extents (`get_attacker_target_extent_sum`). Unless the distance is
/// `FLT_MAX`: when both are actors that either both have the byte at +0x14d
/// set, or (`level_check` set) differ in height by at least
/// `fAICombatSlopeDifference`, and their vertical spans (position z plus the
/// z of vtable +0x1dc and +0x1d8) overlap, the distance is the horizontal
/// one between their positions instead. Returns 0 without a target.
pub fn combat_utilities_get_distance_between_facing_actors(
    e: &mut Engine,
    attacker: Ptr,
    target: Ptr,
    level_check: bool,
) -> f32 {
    if target.is_null() {
        return 0.0;
    }
    e.with_stack(0xa8, |e, frame| {
        // EBP-relative locals inside the 0xa8-byte frame.
        let local = |offset: u32| frame.addr() + 0xa8 - offset;
        let distance = e
            .call(
                REFR_GET_DISTANCE_FROM_REFERENCE,
                &args![attacker, target, 0u32, 0u32],
            )
            .f32();
        e.mem.set_f32(local(0x8), distance);
        let flt_max: f64 = e.global(DOUBLE_FLT_MAX);
        if !parity_even(fcom(load_f32(e, local(0x8)), flt_max), 0x44) {
            return e.mem.f32(local(0x8));
        }
        'facing: {
            if !is_actor(e, attacker) || !is_actor(e, target) {
                break 'facing;
            }
            if attacker.is_null() || target.is_null() {
                break 'facing;
            }
            let both_flagged = e.call(REFR_BYTE_14D, &args![attacker]).bool()
                && e.call(REFR_BYTE_14D, &args![target]).bool();
            let attacker_position = e.vcall(attacker.addr(), 0x1f4, &[]).u32();
            copy_point(e, attacker_position, local(0x2c));
            let target_position = e.vcall(target.addr(), 0x1f4, &[]).u32();
            copy_point(e, target_position, local(0x44));
            copy_point(e, local(0x2c), local(0x38));
            copy_point(e, local(0x44), local(0x20));
            if !both_flagged {
                if !level_check {
                    break 'facing;
                }
                let height_difference = load_f32(e, local(0x30)) - load_f32(e, local(0x18));
                let height_difference = float_arg(height_difference);
                let height_difference = e.call(FLOAT_ABS, &args![height_difference]).f64();
                let slope = setting_f32(e, SETTING_AI_COMBAT_SLOPE_DIFFERENCE);
                if parity_even(fcom(slope, height_difference), 0x41) {
                    break 'facing;
                }
            }
            e.mem.set_f32(local(0x30), 0.0);
            e.mem.set_f32(local(0x18), 0.0);
            let attacker_z = load_f32(e, local(0x24));
            let target_z = load_f32(e, local(0x3c));
            let top = e.vcall(attacker.addr(), 0x1dc, &args![local(0x70)]).u32();
            let attacker_top = load_f32(e, top + 8) + attacker_z;
            store_f32(e, local(0x54), attacker_top);
            let bottom = e.vcall(attacker.addr(), 0x1d8, &args![local(0x7c)]).u32();
            let attacker_bottom = load_f32(e, bottom + 8) + attacker_z;
            store_f32(e, local(0x4c), attacker_bottom);
            let top = e.vcall(target.addr(), 0x1dc, &args![local(0x88)]).u32();
            let target_top = load_f32(e, top + 8) + target_z;
            store_f32(e, local(0x50), target_top);
            let bottom = e.vcall(target.addr(), 0x1d8, &args![local(0x94)]).u32();
            let target_bottom = load_f32(e, bottom + 8) + target_z;
            store_f32(e, local(0x48), target_bottom);
            let attacker_top = load_f32(e, local(0x54));
            let attacker_bottom = load_f32(e, local(0x4c));
            let target_top = load_f32(e, local(0x50));
            let target_bottom = load_f32(e, local(0x48));
            let overlaps = (fcom(target_top, attacker_top) & 0x01 == 0
                && !parity_even(fcom(target_bottom, attacker_top), 0x41))
                || (fcom(target_top, attacker_bottom) & 0x01 == 0
                    && !parity_even(fcom(target_bottom, attacker_bottom), 0x41));
            if !overlaps {
                break 'facing;
            }
            e.call(
                POINT_SUBTRACT,
                &args![local(0x38), local(0x60), local(0x20)],
            );
            let flat = e.call(POINT_LENGTH, &args![local(0x60)]).f32();
            e.mem.set_f32(local(0x64), flat);
            copy_f32(e, local(0x64), local(0x8));
        }
        let extents = combat_utilities_get_attacker_target_extent_sum(e, attacker, target);
        e.mem.set_f32(local(0x4), extents);
        (load_f32(e, local(0x8)) - load_f32(e, local(0x4))) as f32
    })
}

// Translated from 009a6770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetAttackerTargetExtentSum` (Xbox PDB): the extent of
/// the attacker plus that of the target (when there is one). An actor's
/// extent is `Actor::GetForwardLength`; another reference's is its scale
/// times the y of what vtable +0x1dc returns.
pub fn combat_utilities_get_attacker_target_extent_sum(
    e: &mut Engine,
    attacker: Ptr,
    target: Ptr,
) -> f32 {
    e.with_stack(0x18, |e, buffers| {
        let mut sum = if is_actor(e, attacker) {
            e.call(ACTOR_GET_FORWARD_LENGTH, &args![attacker]).f32()
        } else {
            let bound = e
                .vcall(attacker.addr(), 0x1dc, &args![buffers.addr()])
                .u32();
            let scale = e.call(REFR_GET_SCALE, &args![attacker]).f64();
            (scale * load_f32(e, bound + 4)) as f32
        };
        if !target.is_null() {
            sum = if is_actor(e, target) {
                let length = e.call(ACTOR_GET_FORWARD_LENGTH, &args![target]).f64();
                (length + sum as f64) as f32
            } else {
                let bound = e
                    .vcall(target.addr(), 0x1dc, &args![buffers.addr() + 0xc])
                    .u32();
                let scale = e.call(REFR_GET_SCALE, &args![target]).f64();
                (scale * load_f32(e, bound + 4) + sum as f64) as f32
            };
        }
        sum
    })
}

// Translated from 009a6830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::ComputeFaceTargetDistance` (Xbox PDB): half the reach
/// plus the two extents, at least 0. A reach of -1 means the attacker's
/// current weapon's reach (`get_weapon_reach`).
pub fn combat_utilities_compute_face_target_distance(
    e: &mut Engine,
    attacker: Ptr,
    target: Ptr,
    mut reach: f32,
) -> f32 {
    let extents = combat_utilities_get_attacker_target_extent_sum(e, attacker, target);
    let minus_one: f64 = e.global(DOUBLE_MINUS_ONE);
    if !parity_even(fcom(reach as f64, minus_one), 0x44) {
        let weapon = e.call(ACTOR_GET_CURRENT_WEAPON, &args![attacker]).ptr();
        reach = combat_utilities_get_weapon_reach(e, attacker, weapon);
    }
    let half: f64 = e.global(DOUBLE_HALF);
    let distance = (reach as f64 * half) as f32;
    let distance = (distance as f64 + extents as f64) as f32;
    e.call(FLOAT_MAX, &args![distance, 0.0f32]).f32()
}

// Translated from 009a68b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::ComputeFaceTargetLocation` (Xbox PDB): writes to
/// `result` the point `from` plus a direction scaled by
/// `compute_face_target_distance(actor, target, reach)`, and returns
/// `result`. The direction is (sin, cos) of the angle `008e26a0` gives for
/// what the actor's vtable +0x428 returns, when that is set, the target is
/// an actor and `fn_009a9a20(target, actor)` holds; otherwise it stays the
/// zero point (`011f426c`): the code then computes the unit vector from
/// `from` to the actor into a local it never reads.
pub fn combat_utilities_compute_face_target_location(
    e: &mut Engine,
    result: Ptr,
    actor: Ptr,
    target: Ptr,
    from: Ptr,
    reach: f32,
) -> Ptr {
    e.with_stack(0x18, |e, locals| {
        let direction = locals.addr();
        let unused = locals.addr() + 0xc;
        copy_point(e, ZERO_POINT, direction);
        let target_actor = if is_actor(e, target) {
            target
        } else {
            Ptr::NULL
        };
        let process = e.vcall(actor.addr(), 0x428, &[]).ptr::<()>();
        let mut turned = false;
        if !process.is_null() && !target_actor.is_null() && fn_009a9a20(e, target_actor, actor) {
            let angle = e.call(PROCESS_ANGLE_8E26A0, &args![process]).f32();
            let to_radians: f64 = e.global(DOUBLE_DEGREES_TO_RADIANS);
            let radians = float_arg(angle as f64 * to_radians);
            e.call(SIN_COS, &args![radians, direction, direction + 4]);
            turned = true;
        }
        if !turned {
            let position = e.vcall(actor.addr(), 0x1f4, &[]).u32();
            e.call(POINT_SUBTRACT, &args![position, unused, from]);
            e.call(POINT_UNITIZE, &args![unused]);
        }
        let distance = combat_utilities_compute_face_target_distance(e, actor, target, reach);
        e.call(POINT_SCALE, &args![direction, distance]);
        e.call(POINT_ADD, &args![from, result, direction]);
        result
    })
}

// Translated from 009a69c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetWeaponReach` (Xbox PDB): the reference's scale times
/// a reach: without a weapon, what the reference's vtable +0x380 returns;
/// with a weapon whose flag `006450c0` is clear, 128; otherwise
/// `CombatFormulas::CalcWeaponReach` of the weapon's float `006447f0`.
pub fn combat_utilities_get_weapon_reach(e: &mut Engine, actor: Ptr, weapon: Ptr) -> f32 {
    let scale = e.call(REFR_GET_SCALE, &args![actor]).f32() as f64;
    if weapon.is_null() {
        let reach = e.vcall(actor.addr(), 0x380, &[]).f64();
        return (reach * scale) as f32;
    }
    if !e.call(WEAPON_FLAG_6450C0, &args![weapon]).bool() {
        let default_reach: f64 = e.global(DOUBLE_128);
        return (scale * default_reach) as f32;
    }
    let value = e.call(WEAPON_VALUE_6447F0, &args![weapon]).f64();
    let reach = e.call(CALC_WEAPON_REACH, &args![float_arg(value)]).f64();
    (reach * scale) as f32
}

// Translated from 009a6a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The hit cone angle setting for an attack animation group: 0x5c and 0x5d
/// use `fCombatOverheadHitConeAngle`, 0x5e to 0x60
/// `fCombatSweepHitConeAngle`, 0x61 `fCombatUppercutHitConeAngle`, anything
/// else `fCombatHitConeAngle`.
pub fn fn_009a6a40(e: &mut Engine, group: i32) -> f32 {
    // The compiler's switch: a jump table at `009a6ac0` for 0x5c..=0x61.
    let setting = match group {
        0x5c | 0x5d => SETTING_COMBAT_OVERHEAD_HIT_CONE_ANGLE,
        0x5e..=0x60 => SETTING_COMBAT_SWEEP_HIT_CONE_ANGLE,
        0x61 => SETTING_COMBAT_UPPERCUT_HIT_CONE_ANGLE,
        _ => SETTING_COMBAT_HIT_CONE_ANGLE,
    };
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.f32(value)
}

// Translated from 009a6ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `target` is inside `actor`'s hit cone: the absolute angle between
/// the actor's heading (vtable +0x2bc, plus for the player the angle the
/// player's `00965620` gives for its `"ProjectileNode"` when `00963730`
/// says so) and the direction to the target, in degrees (folded into
/// -180..180 when above 180), is at most the cone. The cone is the attack
/// group's (`fn_009a6a40`, the player only; the group comes from the
/// player's acquire-object chain, 0x1a without one) or
/// `fCombatHitConeAngle`; three times that within 128 units, and for an
/// actor target for which vtable +0x22c(0) holds, the cone times
/// `fCombatDeadActorHitConeMult` is tried again. Writes the angle to
/// `angle_out` when it is not null.
pub fn fn_009a6ae0(e: &mut Engine, actor: Ptr, target: Ptr, angle_out: Ptr) -> bool {
    e.with_stack(0x48, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x48 - offset;
        let mut inside = false;
        let mut group: u32 = 0x1a;
        let player: u32 = e.global(PLAYER_CHARACTER);
        if actor.addr() == player && e.call(ACTOR_8D8520, &args![actor]).u32() != 0 {
            let object = e.call(ACTOR_8D8520, &args![actor]).u32();
            if e.vcall(object, 0x3e8, &[]).u32() != 0 {
                let object = e.call(ACTOR_8D8520, &args![actor]).u32();
                let animation = e.vcall(object, 0x3e8, &[]).u32();
                if e.call(ANIMATION_0048F7F0, &args![animation]).u32() != 0 {
                    let object = e.call(ACTOR_8D8520, &args![actor]).u32();
                    let animation = e.vcall(object, 0x3e8, &[]).u32();
                    let entry = e.call(ANIMATION_0048F7F0, &args![animation]).u32();
                    group = e.call(ANIM_GROUP_005F2420, &args![entry]).u32();
                }
            }
        }
        let actor_position = e.vcall(actor.addr(), 0x1f4, &[]).u32();
        let target_position = e.vcall(target.addr(), 0x1f4, &[]).u32();
        let offset = local(0x1c);
        e.call(
            POINT_SUBTRACT,
            &args![target_position, offset, actor_position],
        );
        let direction = e
            .call(GET_UNCLAMPED_Z_ANGLE_FROM_VECTOR, &args![offset])
            .f32();
        e.mem.set_f32(local(0x24), direction);
        e.mem.set_f32(local(0x8), 0.0);
        let player: u32 = e.global(PLAYER_CHARACTER);
        if actor.addr() == player {
            e.mem.set_f32(local(0x30), 0.0);
            e.mem.set_f32(local(0x28), 0.0);
            let player: u32 = e.global(PLAYER_CHARACTER);
            let root = e.call(PLAYER_950BB0, &args![player, 0u32]).u32();
            let node = e.call(FIND_NODE, &args![root, PROJECTILE_NODE_NAME]).u32();
            e.mem.set_u32(local(0x2c), node);
            let flt_min = x87_quiet_bits(e.mem.u32(FLOAT_FLT_MIN));
            let player: u32 = e.global(PLAYER_CHARACTER);
            e.call(
                PLAYER_965620,
                &args![
                    player,
                    0u32,
                    node,
                    local(0x30),
                    local(0x28),
                    0u32,
                    flt_min,
                    flt_min
                ],
            );
            let player: u32 = e.global(PLAYER_CHARACTER);
            if e.call(PLAYER_963730, &args![player]).u32() != 0 {
                copy_f32(e, local(0x30), local(0x8));
            }
        }
        let heading = e.vcall(actor.addr(), 0x2bc, &args![0u32]).f64();
        let difference = float_arg(heading + load_f32(e, local(0x8)) - load_f32(e, local(0x24)));
        let difference = e.call(FLOAT_ABS, &args![difference]).f64();
        let to_degrees: f64 = e.global(DOUBLE_RADIANS_TO_DEGREES);
        store_f32(e, local(0x20), difference * to_degrees);
        let half_turn: f64 = e.global(DOUBLE_180);
        if fcom(load_f32(e, local(0x20)), half_turn) & 0x41 == 0 {
            let full_turn: f64 = e.global(DOUBLE_360);
            let folded = float_arg(load_f32(e, local(0x20)) - full_turn);
            let folded = e.call(FLOAT_ABS_2, &args![folded]).f32();
            e.mem.set_f32(local(0x20), folded);
        }
        e.mem.set_f32(local(0x4), 0.0);
        let player: u32 = e.global(PLAYER_CHARACTER);
        if actor.addr() == player {
            let cone = fn_009a6a40(e, group as i32);
            e.mem.set_f32(local(0x4), cone);
        } else {
            let value = e
                .call(SETTING_VALUE, &args![SETTING_COMBAT_HIT_CONE_ANGLE])
                .u32();
            copy_f32(e, value, local(0x4));
        }
        let distance = e.call(POINT_LENGTH, &args![offset]).f64();
        let near: f64 = e.global(DOUBLE_128);
        if !parity_even(fcom(distance, near), 0x05) {
            let three: f64 = e.global(DOUBLE_3);
            store_f32(e, local(0x4), load_f32(e, local(0x4)) * three);
        }
        let angle = load_f32(e, local(0x20));
        if fcom(load_f32(e, local(0x4)), angle) & 0x01 == 0 {
            inside = true;
        } else if is_actor(e, target) && e.vcall(target.addr(), 0x22c, &args![0u32]).bool() {
            let value = e
                .call(
                    SETTING_VALUE,
                    &args![SETTING_COMBAT_DEAD_ACTOR_HIT_CONE_MULT],
                )
                .u32();
            store_f32(e, local(0x4), load_f32(e, local(0x4)) * load_f32(e, value));
            if fcom(load_f32(e, local(0x4)), angle) & 0x01 == 0 {
                inside = true;
            }
        }
        if !angle_out.is_null() {
            copy_f32(e, local(0x20), angle_out.addr());
        }
        inside
    })
}

// Translated from 009a6d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::IsTargetWithinFiringArc` (Xbox PDB): whether the angles
/// to `point` (`GetAngleToProjectedPoint`) are within half of each arc of
/// the actor's heading (`MobileObject::GetActualHeading`, z angle) and
/// looking angle (`MobileObject::GetLooking`, x angle). An arc that is not
/// below `2pi` is not checked.
pub fn combat_utilities_is_target_within_firing_arc(
    e: &mut Engine,
    actor: Ptr,
    point: Ptr,
    horizontal_arc: f32,
    vertical_arc: f32,
) -> bool {
    e.with_stack(0x24, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x24 - offset;
        let angles = local(0xc);
        let target = PointArg::load(e, point.addr());
        combat_utilities_get_angle_to_projected_point(e, Ptr::new(angles), actor, target);
        let two_pi: f64 = e.global(DOUBLE_TWO_PI);
        let half: f64 = e.global(DOUBLE_HALF);
        if !parity_even(fcom(horizontal_arc as f64, two_pi), 0x05) {
            let heading = e.call(MOBILE_GET_ACTUAL_HEADING, &args![actor]).f32();
            e.mem.set_f32(local(0x18), heading);
            let z = e.mem.f32(local(0x4));
            let z = e.call(CLAMP_ANGLE, &args![z]).f32();
            e.mem.set_f32(local(0x4), z);
            e.mem.set_f32(local(0x10), 0.0);
            let heading = e.mem.f32(local(0x18));
            let difference = e
                .call(ANGLE_DIFFERENCE, &args![heading, z, local(0x10)])
                .f64();
            let difference = e.call(FLOAT_ABS_2, &args![float_arg(difference)]).f32();
            e.mem.set_f32(local(0x14), difference);
            if fcom(difference as f64, horizontal_arc as f64 * half) & 0x41 == 0 {
                return false;
            }
        }
        if !parity_even(fcom(vertical_arc as f64, two_pi), 0x05) {
            let looking = e.call(MOBILE_GET_LOOKING, &args![actor]).f32();
            e.mem.set_f32(local(0x24), looking);
            let x = e.mem.f32(local(0xc));
            let x = e.call(CLAMP_ANGLE, &args![x]).f32();
            e.mem.set_f32(local(0xc), x);
            e.mem.set_f32(local(0x1c), 0.0);
            let looking = e.mem.f32(local(0x24));
            let difference = e
                .call(ANGLE_DIFFERENCE, &args![looking, x, local(0x1c)])
                .f64();
            let difference = e.call(FLOAT_ABS_2, &args![float_arg(difference)]).f32();
            e.mem.set_f32(local(0x20), difference);
            if fcom(difference as f64, vertical_arc as f64 * half) & 0x41 == 0 {
                return false;
            }
        }
        true
    })
}

// Translated from 009a6e90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a6e90 - C++ exception frame (unwinding of the ray collector on a throw) and the /GS cookie check (00ec408c) not translated
/// `CombatUtilities::CalculateProjectileLOS` (Xbox PDB): casts the
/// projectile's path from `from` to `to` and returns whether something was
/// hit. Without gravity (the projectile's `00966980` times `011f1c00`) it is
/// one ray; otherwise the arc is cut into `iBallisticProjectilePathPickSegments`
/// straight segments, launched at the pitch of the line from `from` to
/// `to` with `speed` (or the projectile's `0069ef80` when 0), stopping at
/// the first hit. On a hit, the hit reference (`FindReferenceFor3D`), the
/// hit point and the fraction of the whole path are written to the
/// out pointers that are not null.
#[allow(clippy::too_many_arguments)]
pub fn combat_utilities_calculate_projectile_los(
    e: &mut Engine,
    attacker: Ptr,
    projectile: Ptr,
    speed: f32,
    from: PointArg,
    to: PointArg,
    hit_point_out: Ptr,
    hit_reference_out: Ptr,
    fraction_out: Ptr,
) -> bool {
    const FRAME: u32 = 0x4e0;
    e.with_stack(FRAME + 0x18, |e, frame| {
        let local = |offset: u32| frame.addr() + FRAME - offset;
        // The by-value arguments, where the code reads them.
        let from_arg = frame.addr() + FRAME;
        let to_arg = from_arg + 0xc;
        from.store(e, from_arg);
        to.store(e, to_arg);
        e.call(NO_OP_483710, &args![NO_OP_OBJECT]);
        let filter = local(0x18);
        e.call(0x008c_71b0, &args![filter, 0u32]);
        e.call(FILTER_SET_LAYER, &args![filter, 6u32]);
        let attacker_filter = e
            .call(REFR_COLLISION_FILTER, &args![attacker, local(0x1c)])
            .u32();
        let group = e.call(FILTER_HIGH_HALF, &args![attacker_filter]).u32();
        e.call(FILTER_SET_HIGH_HALF, &args![filter, group]);
        let gravity = if projectile.is_null() {
            0.0
        } else {
            projectile_gravity(e, projectile)
        };
        store_f32(e, local(0x2c), gravity);
        e.mem.set_u32(local(0x30), 0);
        e.mem.set_u32(local(0x34), 0);
        let collector = local(0x360);
        let pick = local(0x410);
        e.call(RAY_COLLECTOR_CONSTRUCT, &args![collector]);
        e.call(PICK_DATA_CONSTRUCT, &args![pick]);
        let filter_word = e.mem.u32(filter);
        e.call(PICK_DATA_SET_FILTER, &args![pick, filter_word]);
        e.call(PICK_DATA_SET_COLLECTOR, &args![pick, collector]);
        let start = local(0x41c);
        let end = local(0x428);
        let fraction = local(0x42c);
        let reached = local(0x430);
        let segments = local(0x434);
        from.store(e, start);
        to.store(e, end);
        e.mem.set_f32(fraction, 0.0);
        e.mem.set_u32(reached, 1);
        e.mem.set_u32(segments, 1);
        let zero: f64 = e.global(DOUBLE_ZERO);
        let hit = if !parity_even(fcom(load_f32(e, local(0x2c)), zero), 0x44) {
            let hit = projectile_los_segment(
                e,
                attacker,
                projectile,
                pick,
                start,
                end,
                fraction,
                local(0x30),
            );
            e.mem.set_u32(local(0x34), hit);
            hit
        } else {
            let offset = local(0x440);
            e.call(POINT_SUBTRACT, &args![to_arg, offset, from_arg]);
            let length = e.call(POINT_LENGTH, &args![offset]).f64();
            store_f32(e, local(0x444), load_f32(e, local(0x438)) / length);
            let sine = e.mem.f32(local(0x444));
            let pitch = e.call(ASIN, &args![sine]).f32();
            e.mem.set_f32(local(0x448), pitch);
            e.mem.set_f32(local(0x438), 0.0);
            let flat = e.call(POINT_LENGTH, &args![offset]).f32();
            e.mem.set_f32(local(0x44c), flat);
            let yaw = e
                .call(GET_UNCLAMPED_Z_ANGLE_FROM_VECTOR, &args![offset])
                .f32();
            e.mem.set_f32(local(0x450), yaw);
            e.mem.set_u32(local(0x454), x87_quiet_bits(speed.to_bits()));
            if !parity_even(fcom(load_f32(e, local(0x454)), zero), 0x44) {
                let default_speed = e.call(PROJECTILE_SPEED, &args![projectile]).f32();
                e.mem.set_f32(local(0x454), default_speed);
            }
            let count = setting_u32(e, SETTING_BALLISTIC_PROJECTILE_PATH_PICK_SEGMENTS);
            e.mem.set_u32(segments, count);
            let step = load_f32(e, local(0x44c)) / count as f64;
            store_f32(e, local(0x464), step);
            let step = load_f32(e, local(0x464));
            let yaw = e.mem.f32(local(0x450));
            let sine = e.call(TRIG_4E44B0, &args![yaw]).f64();
            store_f32(e, local(0x468), sine * step);
            let cosine = e.call(TRIG_4E4470, &args![yaw]).f64();
            store_f32(e, local(0x46c), cosine * step);
            let pitch = e.mem.f32(local(0x448));
            let tangent = e.call(TRIG_57DD70, &args![pitch]).f32();
            e.mem.set_f32(local(0x470), tangent);
            let cosine = e.call(TRIG_4E4470, &args![pitch]).f32();
            e.mem.set_f32(local(0x474), cosine);
            let two: f64 = e.global(DOUBLE_TWO);
            let mut index: u32 = 0;
            let mut hit = 0;
            while index < e.mem.u32(segments) {
                let n = index.wrapping_add(1);
                e.mem.set_u32(reached, n);
                from.store(e, end);
                let distance = n as f64 * load_f32(e, local(0x464));
                store_f32(e, local(0x484), distance);
                let t = load_f32(e, local(0x484));
                let g = load_f32(e, local(0x2c));
                let v = load_f32(e, local(0x454));
                let c = load_f32(e, local(0x474));
                let rise = t * load_f32(e, local(0x470)) - g * t * t / (v * v * c * c * two);
                store_f32(e, local(0x488), rise);
                let x = n as f64 * load_f32(e, local(0x468)) + load_f32(e, end);
                store_f32(e, end, x);
                let y = n as f64 * load_f32(e, local(0x46c)) + load_f32(e, end + 4);
                store_f32(e, end + 4, y);
                let z = load_f32(e, end + 8) + load_f32(e, local(0x488));
                store_f32(e, end + 8, z);
                hit = projectile_los_segment(
                    e,
                    attacker,
                    projectile,
                    pick,
                    start,
                    end,
                    fraction,
                    local(0x30),
                );
                e.mem.set_u32(local(0x34), hit);
                if hit != 0 {
                    break;
                }
                PointArg::load(e, end).store(e, start);
                index = index.wrapping_add(1);
            }
            hit
        };
        let result = if hit == 0 {
            false
        } else {
            if !hit_reference_out.is_null() {
                let reference = e.call(FIND_REFERENCE_FOR_3D, &args![hit]).u32();
                e.mem.set_u32(hit_reference_out.addr(), reference);
            }
            if !hit_point_out.is_null() {
                let along = e
                    .call(POINT_SUBTRACT, &args![end, local(0x4bc), start])
                    .u32();
                let fraction_value = e.mem.f32(fraction);
                let scaled = e
                    .call(POINT_TIMES, &args![along, local(0x4b0), fraction_value])
                    .u32();
                let point = e.call(POINT_ADD, &args![scaled, local(0x4a4), start]).u32();
                PointArg::load(e, point).store(e, hit_point_out.addr());
            }
            if !fraction_out.is_null() {
                let count = e.mem.u32(segments);
                if count == 1 {
                    copy_f32(e, fraction, fraction_out.addr());
                } else {
                    let before = e.mem.u32(reached).wrapping_sub(1);
                    let total = load_f32(e, fraction) / count as f64 + before as f64 / count as f64;
                    store_f32(e, fraction_out.addr(), total);
                }
            }
            true
        };
        e.call(NO_OP_483710, &args![NO_OP_OBJECT]);
        e.call(RAY_COLLECTOR_DESTRUCT, &args![collector]);
        result
    })
}

// Translated from 009a7580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::CheckProjectileArc` (Xbox PDB): `CheckProjectileArc_ov2`
/// from `from` (or the attacker's firing location when null) to the
/// target's firing location (mode 2) with the target's velocity. False
/// without attacker or target.
#[allow(clippy::too_many_arguments)]
pub fn combat_utilities_check_projectile_arc(
    e: &mut Engine,
    attacker: Ptr,
    target: Ptr,
    projectile: Ptr,
    from: Ptr,
    high_arc: Ptr,
    out: Ptr,
    threshold: f32,
) -> bool {
    if attacker.is_null() || target.is_null() {
        return false;
    }
    e.with_stack(0x30, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x30 - offset;
        e.call(POINT_CONSTRUCT, &args![local(0x18)]);
        if from.is_null() {
            let location =
                combat_utilities_get_attacker_firing_location(e, Ptr::new(local(0x30)), attacker);
            PointArg::load(e, location.addr()).store(e, local(0x18));
        } else {
            PointArg::load(e, from.addr()).store(e, local(0x18));
        }
        combat_utilities_get_target_firing_location(e, Ptr::new(local(0x24)), target, 2);
        combat_utilities_get_target_velocity(e, Ptr::new(local(0xc)), target);
        let start = PointArg::load(e, local(0x18));
        let target_location = PointArg::load(e, local(0x24));
        let velocity = PointArg::load(e, local(0xc));
        combat_utilities_check_projectile_arc_ov2(
            e,
            attacker,
            target,
            projectile,
            start,
            target_location,
            velocity,
            high_arc,
            out,
            f32::from_bits(x87_quiet_bits(threshold.to_bits())),
        )
    })
}

// Translated from 009a7670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::CheckProjectileArc_ov2` (Xbox PDB): whether a
/// projectile fired from `from` can reach the projected target point (for
/// the target's location and velocity). Without gravity the straight line
/// is tested (line of sight with speed 0); with gravity the current arc
/// (`*high_arc`, high arcs at `fGrenadeHighArcSpeedPercentage` of the
/// projectile's `0069ef80` speed) and then the other one are tried, and
/// `*high_arc` keeps the arc that worked (it is restored when neither
/// does). A hit on something else still counts when the threshold is at
/// most the hit fraction (straight, NaN included) or below it (arcs); a
/// threshold of -1 means `fGrenadeThrowHitFractionThreshold`. The aimed
/// point goes to `out`.
#[allow(clippy::too_many_arguments)]
pub fn combat_utilities_check_projectile_arc_ov2(
    e: &mut Engine,
    attacker: Ptr,
    target: Ptr,
    projectile: Ptr,
    from: PointArg,
    target_location: PointArg,
    velocity: PointArg,
    high_arc: Ptr,
    out: Ptr,
    threshold: f32,
) -> bool {
    if attacker.is_null() || target.is_null() {
        return false;
    }
    e.with_stack(0x38, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x38 - offset;
        let mut threshold = threshold;
        let minus_one: f64 = e.global(DOUBLE_MINUS_ONE);
        if !parity_even(fcom(threshold as f64, minus_one), 0x44) {
            let value = e
                .call(
                    SETTING_VALUE,
                    &args![SETTING_GRENADE_THROW_HIT_FRACTION_THRESHOLD],
                )
                .u32();
            threshold = f32::from_bits(x87_quiet_bits(e.mem.u32(value)));
        }
        let fraction = local(0x8);
        let hit_reference = local(0x4);
        e.mem.set_f32(fraction, 0.0);
        e.mem.set_u32(hit_reference, 0);
        let gravity = projectile_gravity(e, projectile);
        store_f32(e, local(0xc), gravity);
        let gravity = e.mem.f32(local(0xc));
        let zero: f64 = e.global(DOUBLE_ZERO);
        if !parity_even(fcom(gravity as f64, zero), 0x44) {
            let high = e.mem.u8(high_arc.addr()) != 0;
            let speed = e.call(PROJECTILE_SPEED, &args![projectile]).f32();
            let projected = local(0x1c);
            combat_utilities_get_projected_point_for_target_ov4(
                e,
                Ptr::new(projected),
                from,
                target_location,
                velocity,
                speed,
                gravity,
                high,
            );
            let aim = PointArg::load(e, projected);
            let blocked = combat_utilities_calculate_projectile_los(
                e,
                attacker,
                projectile,
                0.0,
                from,
                aim,
                Ptr::NULL,
                Ptr::new(hit_reference),
                Ptr::new(fraction),
            );
            if blocked
                && e.mem.u32(hit_reference) != target.addr()
                && fcom(threshold as f64, load_f32(e, fraction)) & 0x41 == 0
            {
                return false;
            }
            aim.store(e, out.addr());
            return true;
        }
        let saved = e.mem.u8(high_arc.addr());
        let mut attempt: u32 = 0;
        while attempt < 2 {
            if attempt == 1 {
                let current = e.mem.u8(high_arc.addr());
                e.mem.set_u8(high_arc.addr(), u8::from(current == 0));
            }
            let high = e.mem.u8(high_arc.addr()) != 0;
            if high {
                let value = e
                    .call(
                        SETTING_VALUE,
                        &args![SETTING_GRENADE_HIGH_ARC_SPEED_PERCENTAGE],
                    )
                    .u32();
                copy_f32(e, value, local(0x38));
            } else {
                e.mem.set_f32(local(0x38), 1.0);
            }
            let speed = e.call(PROJECTILE_SPEED, &args![projectile]).f64();
            store_f32(e, local(0x34), speed * load_f32(e, local(0x38)));
            let speed = e.mem.f32(local(0x34));
            let projected = local(0x2c);
            combat_utilities_get_projected_point_for_target_ov4(
                e,
                Ptr::new(projected),
                from,
                target_location,
                velocity,
                speed,
                gravity,
                high,
            );
            let aim = PointArg::load(e, projected);
            let blocked = combat_utilities_calculate_projectile_los(
                e,
                attacker,
                projectile,
                speed,
                from,
                aim,
                Ptr::NULL,
                Ptr::new(hit_reference),
                Ptr::new(fraction),
            );
            if !blocked
                || e.mem.u32(hit_reference) == target.addr()
                || !parity_even(fcom(threshold as f64, load_f32(e, fraction)), 0x05)
            {
                aim.store(e, out.addr());
                return true;
            }
            attempt += 1;
        }
        e.mem.set_u8(high_arc.addr(), saved);
        false
    })
}

// Translated from 009a7940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a7940 - C++ exception frame (unwinding of the collector on a throw) and the /GS cookie check (00ec408c) not translated
/// `CombatUtilities::CalculateLOS` (Xbox PDB): whether the line from `from`
/// (raised by the attacker's firing z offset) to `to` (raised by the
/// target's mode-2 firing z offset when there is a target) is clear: true
/// when the attacker's cell pick (`TESObjectCELL::Pick`) hits nothing or
/// hits the target. The pick uses collision layer `layer` (0x25 when -1)
/// with the attacker's filter; the collector (`fn_009a7bc0`) gets the
/// target's filter when the target's vtable +0xfc holds.
pub fn combat_utilities_calculate_los(
    e: &mut Engine,
    attacker: Ptr,
    target: Ptr,
    from: Ptr,
    to: Ptr,
    _unused_5: u32,
    layer: u32,
) -> bool {
    const FRAME: u32 = 0x190;
    e.with_stack(FRAME, |e, frame| {
        let local = |offset: u32| frame.addr() + FRAME - offset;
        e.call(NO_OP_483710, &args![NO_OP_OBJECT]);
        let layer = if layer == u32::MAX { 0x25 } else { layer };
        e.mem.set_u32(local(0x18), layer);
        let start = local(0x24);
        let end = local(0x30);
        PointArg::load(e, from.addr()).store(e, start);
        let raise = combat_utilities_get_attacker_firing_z_offset(e, attacker, true);
        store_f32(e, start + 8, raise as f64 + load_f32(e, start + 8));
        PointArg::load(e, to.addr()).store(e, end);
        if !target.is_null() {
            let raise = combat_utilities_get_target_firing_z_offset(e, target, 2, true);
            store_f32(e, end + 8, raise as f64 + load_f32(e, end + 8));
        }
        let pick = local(0xe0);
        let filter = local(0xe4);
        e.call(PICK_DATA_CONSTRUCT, &args![pick]);
        e.call(REFR_COLLISION_FILTER, &args![attacker, filter]);
        if !target.is_null() && e.vcall(target.addr(), 0xfc, &[]).bool() {
            let word = e
                .call(REFR_COLLISION_FILTER, &args![target, local(0xec)])
                .u32();
            let value = e.mem.u32(word);
            e.mem.set_u32(filter, value);
        }
        let collector = local(0x170);
        fn_009a7bc0(e, Ptr::new(collector), layer, Ptr::new(filter));
        e.call(PICK_DATA_SET_RAY_COLLECTOR, &args![pick, collector]);
        let word = e
            .call(REFR_COLLISION_FILTER, &args![attacker, local(0x174)])
            .u32();
        let value = e.mem.u32(word);
        e.mem.set_u32(filter, value);
        e.call(FILTER_SET_LAYER, &args![filter, layer]);
        let filter_word = e.mem.u32(filter);
        e.call(PICK_DATA_SET_FILTER, &args![pick, filter_word]);
        e.call(PICK_DATA_SET_START, &args![pick, start]);
        e.call(PICK_DATA_SET_END, &args![pick, end]);
        let cell = e.call(REFR_PARENT_CELL, &args![attacker]).u32();
        let hit = e.call(CELL_PICK, &args![cell, pick]).u32();
        let clear = if hit == 0 {
            true
        } else {
            let reference = e.call(FIND_REFERENCE_FOR_3D, &args![hit]).u32();
            !target.is_null() && reference == target.addr()
        };
        e.call(NO_OP_483710, &args![NO_OP_OBJECT]);
        e.call(LOS_COLLECTOR_DESTRUCT, &args![collector]);
        clear
    })
}

// Translated from 009a7bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a7bc0 - C++ exception frame (unwinding of the base on a throw) not translated
/// Constructs `CalculateLOS`'s ray collector at `this` (thiscall): the base
/// constructor (`0062a220`), the vtable `0104e1dc`, a zero word at +0x70
/// (`008c71b0`) and `0062a3b0(layer, filter)`. Returns `this`.
pub fn fn_009a7bc0(e: &mut Engine, this: Ptr, layer: u32, filter: Ptr) -> Ptr {
    e.call(COLLECTOR_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), LOS_COLLECTOR_VTABLE);
    e.call(0x008c_71b0, &args![this.addr() + 0x70, 0u32]);
    e.call(COLLECTOR_INIT, &args![this, layer, filter]);
    this
}

// Translated from 009a7c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at +0x6c (thiscall; `fn_009a7f00` reads it from a projectile
/// as the range it returns for one without an arc).
pub fn fn_009a7c40(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x6c)
}

// Translated from 009a7c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The launch angle to reach a point `distance` away horizontally and
/// `height` up at `speed` under `gravity`: with `s = speed^2` (rounded to a
/// float) and the discriminant `s^2 - (gravity * distance^2 + 2 * height *
/// s) * gravity`, `pi / 4` when that is not positive (or NaN), otherwise
/// `00755660` of the higher (`high`, `00404010`) or lower (`0040ebd0`) of
/// `(s -+ 004579e0(discriminant)) / (distance * gravity)`.
pub fn fn_009a7c60(
    e: &mut Engine,
    distance: f32,
    height: f32,
    speed: f32,
    gravity: f32,
    high: bool,
) -> f32 {
    let (distance, height, speed, gravity) =
        (distance as f64, height as f64, speed as f64, gravity as f64);
    let two: f64 = e.global(DOUBLE_TWO);
    let speed_squared = (speed * speed) as f32 as f64;
    let discriminant = (speed_squared * speed_squared
        - (gravity * distance * distance + height * two * speed_squared) * gravity)
        as f32;
    let zero: f64 = e.global(DOUBLE_ZERO);
    if fcom(discriminant as f64, zero) & 0x41 != 0 {
        let quarter: f64 = e.global(DOUBLE_QUARTER);
        return (load_f32(e, FLOAT_PI) * quarter) as f32;
    }
    let root = e.call(MATH_4579E0, &args![discriminant]).f32() as f64;
    let low = ((speed_squared - root) / (distance * gravity)) as f32;
    let high_root = ((speed_squared + root) / (distance * gravity)) as f32;
    let slope = if high {
        e.call(FLOAT_MAX, &args![low, high_root]).f32()
    } else {
        e.call(FLOAT_MIN, &args![low, high_root]).f32()
    };
    e.call(MATH_755660, &args![slope]).f32()
}

// Translated from 009a7d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The launch angle (`fn_009a7c60`) for `offset` (its length without z,
/// and its z), `speed` (the projectile's `009669c0` when 0) and the gravity
/// of the projectile's `004181e0` object (`00966980` times `011f1c00`).
pub fn fn_009a7d50(
    e: &mut Engine,
    projectile: Ptr,
    offset: PointArg,
    speed: f32,
    high: bool,
) -> f32 {
    e.with_stack(0xc, |e, point| {
        offset.store(e, point.addr());
        let height = f32::from_bits(x87_quiet_bits(offset.0[2]));
        e.mem.set_f32(point.addr() + 8, 0.0);
        let distance = e.call(POINT_LENGTH, &args![point]).f32();
        let zero: f64 = e.global(DOUBLE_ZERO);
        let mut speed = speed;
        if !parity_even(fcom(speed as f64, zero), 0x44) {
            speed = e.call(PROJECTILE_VALUE_9669C0, &args![projectile]).f32();
        }
        let base = e.call(PROJECTILE_BASE_4181E0, &args![projectile]).ptr();
        let gravity = projectile_gravity(e, base) as f32;
        fn_009a7c60(e, distance, height, speed, gravity, high)
    })
}

// Translated from 009a7dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `distance / (005b9e80(angle) * speed)` (`005b9e80` wraps `004e4470`; as
/// used, the horizontal part of the launch speed).
pub fn fn_009a7dd0(e: &mut Engine, distance: f32, angle: f32, speed: f32) -> f32 {
    let factor = e.call(0x005b_9e80, &args![angle]).f64();
    (distance as f64 / (factor * speed as f64)) as f32
}

// Translated from 009a7e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::ComputeProjectileTime` (Xbox PDB): the flight time
/// between the two points. Without gravity (`00966980` = 0) the distance
/// over `speed` (the argument, even when 0); otherwise `fn_009a7dd0` of the
/// horizontal distance and the low-arc launch angle (`fn_009a7c60` with the
/// speed argument, times `fGrenadeHighArcSpeedPercentage` for a high arc
/// when it is 0), with the projectile's speed (`0069ef80`) when the speed
/// argument is 0.
pub fn combat_utilities_compute_projectile_time(
    e: &mut Engine,
    projectile: Ptr,
    from: PointArg,
    to: PointArg,
    high: bool,
    speed: f32,
) -> f32 {
    e.with_stack(0x24 + 0x18, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x24 - offset;
        let from_arg = frame.addr() + 0x24;
        let to_arg = from_arg + 0xc;
        from.store(e, from_arg);
        to.store(e, to_arg);
        let offset = local(0xc);
        e.call(POINT_SUBTRACT, &args![from_arg, offset, to_arg]);
        let mut speed = speed;
        let mut time_speed = f32::from_bits(x87_quiet_bits(speed.to_bits()));
        let zero: f64 = e.global(DOUBLE_ZERO);
        if !parity_even(fcom(speed as f64, zero), 0x44) {
            time_speed = e.call(PROJECTILE_SPEED, &args![projectile]).f32();
        }
        let gravity = e.call(PROJECTILE_GRAVITY, &args![projectile]).f64();
        if !parity_even(fcom(gravity, zero), 0x44) {
            let distance = e.call(POINT_LENGTH, &args![offset]).f64();
            return (distance / speed as f64) as f32;
        }
        if high && !parity_even(fcom(speed as f64, zero), 0x44) {
            let value = e
                .call(
                    SETTING_VALUE,
                    &args![SETTING_GRENADE_HIGH_ARC_SPEED_PERCENTAGE],
                )
                .u32();
            speed = (speed as f64 * load_f32(e, value)) as f32;
        }
        let height = f32::from_bits(x87_quiet_bits(e.mem.u32(local(0x4))));
        let distance = e.call(POINT_LENGTH_XY, &args![offset]).f32();
        let gravity = projectile_gravity(e, projectile) as f32;
        let angle = fn_009a7c60(e, distance, height, speed, gravity, false);
        fn_009a7dd0(e, distance, angle, time_speed)
    })
}

// Translated from 009a7f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The range of a projectile: its float at +0x6c (`fn_009a7c40`) when
/// `fn_009a7f80` holds or its gravity (`00966980`) is not above 0;
/// otherwise `speed^2 / (gravity * 011f1c00)`, with the projectile's speed
/// (`0069ef80`) when `speed` is 0.
pub fn fn_009a7f00(e: &mut Engine, projectile: Ptr, speed: f32) -> f32 {
    let range = fn_009a7c40(e, projectile);
    if fn_009a7f80(e, projectile) {
        return range;
    }
    let gravity = e.call(PROJECTILE_GRAVITY, &args![projectile]).f32();
    let zero: f64 = e.global(DOUBLE_ZERO);
    if !parity_even(fcom(gravity as f64, zero), 0x41) {
        return range;
    }
    let gravity = (gravity as f64 * load_f32(e, GRAVITY_FACTOR)) as f32;
    let mut speed = speed;
    if !parity_even(fcom(speed as f64, zero), 0x44) {
        speed = e.call(PROJECTILE_SPEED, &args![projectile]).f32();
    }
    (speed as f64 * speed as f64 / gravity as f64) as f32
}

// Translated from 009a7f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flag test 1 of the object (`004fd420(this, 1)`, thiscall).
pub fn fn_009a7f80(e: &mut Engine, this: Ptr) -> bool {
    e.call(FLAG_TEST_4FD420, &args![this, 1u32]).bool()
}

// Translated from 009a7fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetAttackerFiringZOffset` (Xbox PDB): for an actor,
/// three quarters of its height (`Actor::GetSneakHeight(0)` when
/// `004997b0` holds and `standing` is not set, else `Actor::GetHeight`);
/// for another reference, half the z distance between what vtable +0x1dc
/// and +0x1d8 return.
pub fn combat_utilities_get_attacker_firing_z_offset(
    e: &mut Engine,
    reference: Ptr,
    standing: bool,
) -> f32 {
    if is_actor(e, reference) {
        let height = if e.call(ACTOR_FLAG_4997B0, &args![reference]).bool() && !standing {
            e.call(ACTOR_GET_SNEAK_HEIGHT, &args![reference, 0u32])
                .f32()
        } else {
            e.call(ACTOR_GET_HEIGHT, &args![reference]).f32()
        };
        let height = f32::from_bits(x87_quiet_bits(height.to_bits()));
        let three_quarters: f64 = e.global(DOUBLE_THREE_QUARTERS);
        return (height as f64 * three_quarters) as f32;
    }
    e.with_stack(0x18, |e, buffers| {
        let top = e
            .vcall(reference.addr(), 0x1dc, &args![buffers.addr()])
            .u32();
        let bottom = e
            .vcall(reference.addr(), 0x1d8, &args![buffers.addr() + 0xc])
            .u32();
        let half: f64 = e.global(DOUBLE_HALF);
        ((load_f32(e, top + 8) - load_f32(e, bottom + 8)) * half) as f32
    })
}

// Translated from 009a8050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetAttackerFiringLocation` (Xbox PDB): where the
/// attacker fires from. For an actor without a weapon (the weapon of what
/// vtable +0x428 returns, `007031c0`, or `Actor::GetCurrentWeapon`) or with
/// one for which `004c0c30` holds: with an acquire object (`008d8520`) and
/// no weapon or a weapon whose `00446390` type is 10 to 13, the world
/// translation of the node the object's vtable +0x190 returns for the
/// biped (the player's `PlayerCharacter::GetBiped`, others' vtable
/// +0x1ec); otherwise that of the node the actor's vtable +0x20c returns.
/// Without such a node: the position raised by `GetAttackerFiringZOffset`.
pub fn combat_utilities_get_attacker_firing_location(
    e: &mut Engine,
    result: Ptr,
    attacker: Ptr,
) -> Ptr {
    if is_actor(e, attacker) {
        let player: u32 = e.global(PLAYER_CHARACTER);
        let is_player = attacker.addr() == player;
        let controller = e.vcall(attacker.addr(), 0x428, &[]).u32();
        let weapon = if controller != 0 {
            e.call(CONTROLLER_WEAPON, &args![controller]).u32()
        } else {
            e.call(ACTOR_GET_CURRENT_WEAPON, &args![attacker]).u32()
        };
        if weapon == 0 || e.call(WEAPON_FLAG_4C0C30, &args![weapon]).bool() {
            let object = e.call(ACTOR_8D8520, &args![attacker]).u32();
            let mut use_biped = !attacker.is_null() && object != 0;
            if use_biped && weapon != 0 {
                use_biped = false;
                for kind in [10u32, 11, 13, 12] {
                    if e.call(WEAPON_TYPE_446390, &args![weapon]).u32() == kind {
                        use_biped = true;
                        break;
                    }
                }
            }
            let node = if use_biped {
                let biped = if is_player {
                    let player: u32 = e.global(PLAYER_CHARACTER);
                    let flag = e.call(PLAYER_4EAF60, &args![player]).u8();
                    let player: u32 = e.global(PLAYER_CHARACTER);
                    e.call(PLAYER_GET_BIPED, &args![player, flag as u32]).u32()
                } else {
                    e.vcall(attacker.addr(), 0x1ec, &[]).u32()
                };
                e.vcall(object, 0x190, &args![biped]).u32()
            } else {
                e.vcall(attacker.addr(), 0x20c, &[]).u32()
            };
            if node != 0 {
                let translation = e.call(NODE_WORLD_TRANSLATE, &args![node]).u32();
                PointArg::load(e, translation).store(e, result.addr());
                return result;
            }
        }
    }
    raised_position(e, result, attacker)
}

// Translated from 009a8250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reference's position (vtable +0x1f4) raised by
/// `GetAttackerFiringZOffset(reference, false)`, written to `result`.
pub fn fn_009a8250(e: &mut Engine, result: Ptr, reference: Ptr) -> Ptr {
    raised_position(e, result, reference)
}

// Translated from 009a82b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetBackProjectedAttackerFiringLocation` (Xbox PDB):
/// the attacker's firing location moved onto the line through the actor's
/// position along (sin, cos) of its heading (vtable +0x2bc) plus pi/2, with
/// z raised by the horizontal distance moved times sin / cos of
/// `MobileObject::GetLooking`. Unchanged for a reference that is not an
/// actor.
pub fn combat_utilities_get_back_projected_attacker_firing_location(
    e: &mut Engine,
    result: Ptr,
    attacker: Ptr,
) -> Ptr {
    e.with_stack(0x74, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x74 - offset;
        let location = local(0xc);
        combat_utilities_get_attacker_firing_location(e, Ptr::new(location), attacker);
        if is_actor(e, attacker) {
            let position = e.vcall(attacker.addr(), 0x1f4, &[]).u32();
            let origin = local(0x4c);
            PointArg::load(e, position).store(e, origin);
            let heading = e.vcall(attacker.addr(), 0x2bc, &args![0u32]).f32();
            e.mem.set_f32(local(0x2c), 0.0);
            e.mem.set_f32(local(0x58), 0.0);
            let half_pi: f64 = e.global(DOUBLE_HALF_PI);
            let side = float_arg(heading as f64 + half_pi);
            e.call(SIN_COS, &args![side, local(0x2c), local(0x58)]);
            let axis = local(0x54);
            let axis_x = e.mem.f32(local(0x2c));
            let axis_y = e.mem.f32(local(0x58));
            e.call(POINT2_SET, &args![axis, axis_x, axis_y]);
            let offset = local(0x38);
            let offset_x = float_arg(load_f32(e, location) - load_f32(e, origin));
            let offset_y = float_arg(load_f32(e, location + 4) - load_f32(e, origin + 4));
            e.call(POINT2_SET, &args![offset, offset_x, offset_y]);
            let along = e.call(POINT2_DOT, &args![offset, axis]).f32();
            e.mem.set_f32(local(0x14), along);
            let back = local(0x28);
            e.call(POINT_CONSTRUCT, &args![back]);
            let along = load_f32(e, local(0x14));
            store_f32(e, back, load_f32(e, axis) * along + load_f32(e, origin));
            store_f32(
                e,
                back + 4,
                load_f32(e, axis + 4) * along + load_f32(e, origin + 4),
            );
            let looking = e.call(MOBILE_GET_LOOKING, &args![attacker]).f32();
            e.mem.set_f32(local(0x30), looking);
            e.mem.set_f32(local(0x40), 0.0);
            e.mem.set_f32(local(0x18), 0.0);
            e.call(SIN_COS, &args![looking, local(0x40), local(0x18)]);
            let moved = local(0x20);
            let moved_x = float_arg(load_f32(e, location) - load_f32(e, back));
            let moved_y = float_arg(load_f32(e, location + 4) - load_f32(e, back + 4));
            e.call(POINT2_SET, &args![moved, moved_x, moved_y]);
            let distance = e.call(POINT_LENGTH_XY, &args![moved]).f32();
            e.mem.set_f32(local(0x5c), distance);
            let rise =
                load_f32(e, local(0x5c)) * load_f32(e, local(0x40)) / load_f32(e, local(0x18));
            store_f32(e, local(0x3c), rise);
            copy_f32(e, back, location);
            copy_f32(e, back + 4, location + 4);
            store_f32(
                e,
                location + 8,
                load_f32(e, location + 8) + load_f32(e, local(0x3c)),
            );
        }
        PointArg::load(e, location).store(e, result.addr());
        result
    })
}

// Translated from 009a8460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetTargetFiringZOffset` (Xbox PDB): the height on the
/// target to aim at. For an actor (height as in `GetAttackerFiringZOffset`):
/// a tenth of it when vtable +0x230, `00579670` or vtable +0x22c(0) holds;
/// otherwise by `mode`: 0 a quarter, 1 and 3 half, 2 three quarters (0.9
/// when sneaking), anything else 0. For another reference: its scale times
/// half the z distance between what vtable +0x1dc and +0x1d8 return.
pub fn combat_utilities_get_target_firing_z_offset(
    e: &mut Engine,
    reference: Ptr,
    mode: u32,
    standing: bool,
) -> f32 {
    let half: f64 = e.global(DOUBLE_HALF);
    if !is_actor(e, reference) {
        return e.with_stack(0x18, |e, buffers| {
            let top = e
                .vcall(reference.addr(), 0x1dc, &args![buffers.addr()])
                .u32();
            let bottom = e
                .vcall(reference.addr(), 0x1d8, &args![buffers.addr() + 0xc])
                .u32();
            let span = load_f32(e, top + 8) - load_f32(e, bottom + 8);
            let scale = e.call(REFR_GET_SCALE, &args![reference]).f64();
            (scale * span * half) as f32
        });
    }
    let sneaking = e.call(ACTOR_FLAG_4997B0, &args![reference]).bool() && !standing;
    let height = if sneaking {
        e.call(ACTOR_GET_SNEAK_HEIGHT, &args![reference, 0u32])
            .f32()
    } else {
        e.call(ACTOR_GET_HEIGHT, &args![reference]).f32()
    };
    let height = f32::from_bits(x87_quiet_bits(height.to_bits())) as f64;
    if e.vcall(reference.addr(), 0x230, &[]).bool()
        || e.call(ACTOR_FLAG_579670, &args![reference]).bool()
        || e.vcall(reference.addr(), 0x22c, &args![0u32]).bool()
    {
        let tenth: f64 = e.global(DOUBLE_TENTH);
        return (height * tenth) as f32;
    }
    // The compiler's switch: a jump table at `009a85f0` for 0..=3.
    match mode {
        0 => {
            let quarter: f64 = e.global(DOUBLE_QUARTER);
            (height * quarter) as f32
        }
        1 | 3 => (height * half) as f32,
        2 => {
            let factor: f64 = if sneaking {
                e.global(DOUBLE_NINE_TENTHS)
            } else {
                e.global(DOUBLE_THREE_QUARTERS)
            };
            (height * factor) as f32
        }
        _ => 0.0,
    }
}

// Translated from 009a8600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetTargetFiringLocation` (Xbox PDB): the target's
/// position (`00436aa0`) raised by `GetTargetFiringZOffset(target, mode,
/// false)`, written to `result`.
pub fn combat_utilities_get_target_firing_location(
    e: &mut Engine,
    result: Ptr,
    target: Ptr,
    mode: u32,
) -> Ptr {
    let position = e.call(REFR_POSITION_436AA0, &args![target]).u32();
    let point = PointArg::load(e, position);
    let raise = combat_utilities_get_target_firing_z_offset(e, target, mode, false);
    let z = (raise as f64 + f32::from_bits(point.0[2]) as f64) as f32;
    PointArg([point.0[0], point.0[1], z.to_bits()]).store(e, result.addr());
    result
}

// Translated from 009a8660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetTargetVelocity` (Xbox PDB): the linear velocity of
/// an actor's character controller (`MobileObject::GetCharController`,
/// `bhkCharacterProxy::GetLinearVelocity`); the zero point otherwise.
pub fn combat_utilities_get_target_velocity(e: &mut Engine, result: Ptr, target: Ptr) -> Ptr {
    e.with_stack(0xc, |e, velocity| {
        copy_point(e, ZERO_POINT, velocity.addr());
        if is_actor(e, target) {
            let controller = e.call(MOBILE_GET_CHAR_CONTROLLER, &args![target]).u32();
            if controller != 0 {
                e.call(
                    CHARACTER_PROXY_GET_LINEAR_VELOCITY,
                    &args![controller, velocity],
                );
            }
        }
        PointArg::load(e, velocity.addr()).store(e, result.addr());
        result
    })
}

// Translated from 009a86e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetAngleToProjectedTarget` (Xbox PDB): the angles from
/// the attacker's firing location to the projected target point
/// (`GetAngleToProjectedTarget_ov2`), less the attacker's own angles
/// (`00430830`), with y zeroed and x and z folded into -pi..pi.
pub fn combat_utilities_get_angle_to_projected_target(
    e: &mut Engine,
    result: Ptr,
    attacker: Ptr,
    target: Ptr,
    speed: f32,
    gravity: f32,
    mode: u32,
) -> Ptr {
    e.with_stack(0x24, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x24 - offset;
        let location = local(0xc);
        combat_utilities_get_attacker_firing_location(e, Ptr::new(location), attacker);
        let angles = local(0x18);
        let from = PointArg::load(e, location);
        let speed = f32::from_bits(x87_quiet_bits(speed.to_bits()));
        let gravity = f32::from_bits(x87_quiet_bits(gravity.to_bits()));
        combat_utilities_get_angle_to_projected_target_ov2(
            e,
            Ptr::new(angles),
            from,
            target,
            speed,
            gravity,
            mode,
        );
        let own = e.call(REFR_ANGLE_430830, &args![attacker]).u32();
        let relative = e
            .call(POINT_SUBTRACT, &args![angles, local(0x24), own])
            .u32();
        PointArg::load(e, relative).store(e, angles);
        e.mem.set_f32(angles + 4, 0.0);
        wrap_angle(e, angles);
        wrap_angle(e, angles + 8);
        PointArg::load(e, angles).store(e, result.addr());
        result
    })
}

// Translated from 009a87f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetAngleToProjectedTarget_ov2` (Xbox PDB): the angles
/// (`GetAngleToProjectedPoint_ov2`) from `from` to the target's projected
/// point (`GetProjectedPointForTarget_ov4` with its firing location for
/// `mode` and its velocity, low arc); the zero point without a target.
pub fn combat_utilities_get_angle_to_projected_target_ov2(
    e: &mut Engine,
    result: Ptr,
    from: PointArg,
    target: Ptr,
    speed: f32,
    gravity: f32,
    mode: u32,
) -> Ptr {
    if target.is_null() {
        copy_point(e, ZERO_POINT, result.addr());
        return result;
    }
    e.with_stack(0x24, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x24 - offset;
        combat_utilities_get_target_velocity(e, Ptr::new(local(0x24)), target);
        combat_utilities_get_target_firing_location(e, Ptr::new(local(0xc)), target, mode);
        let velocity = PointArg::load(e, local(0x24));
        let location = PointArg::load(e, local(0xc));
        let speed = f32::from_bits(x87_quiet_bits(speed.to_bits()));
        let gravity = f32::from_bits(x87_quiet_bits(gravity.to_bits()));
        combat_utilities_get_projected_point_for_target_ov4(
            e,
            Ptr::new(local(0x18)),
            from,
            location,
            velocity,
            speed,
            gravity,
            false,
        );
        let projected = PointArg::load(e, local(0x18));
        combat_utilities_get_angle_to_projected_point_ov2(e, result, from, projected);
        result
    })
}

// Translated from 009a88e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetAngleToProjectedPoint` (Xbox PDB): `fn_009a8970`
/// from the attacker's firing location, with the attacker's raised
/// position (`fn_009a8250`) as the origin, to `point`.
pub fn combat_utilities_get_angle_to_projected_point(
    e: &mut Engine,
    result: Ptr,
    attacker: Ptr,
    point: PointArg,
) -> Ptr {
    e.with_stack(0x1c, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x1c - offset;
        combat_utilities_get_attacker_firing_location(e, Ptr::new(local(0xc)), attacker);
        e.mem.set_f32(local(0x10), 0.0);
        let raised = fn_009a8250(e, Ptr::new(local(0x1c)), attacker);
        let raised = PointArg::load(e, raised.addr());
        let from = PointArg::load(e, local(0xc));
        fn_009a8970(e, result, from, raised, 0, point);
        result
    })
}

// Translated from 009a8970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetAngleToProjectedPoint_ov2` from `from` to `point`, with `point`
/// first pushed out along the direction from `origin` to 1024 units
/// (`011a5594`) from `origin` when `004a7290` of that offset (as used, the
/// squared length) is below 1024 squared and not 0. When `origin` and
/// `from` have the same x and y, `point` is used as it is.
pub fn fn_009a8970(
    e: &mut Engine,
    result: Ptr,
    from: PointArg,
    origin: PointArg,
    _unused_8: u32,
    point: PointArg,
) -> Ptr {
    let same_x = !parity_even(
        fcom(
            f32::from_bits(origin.0[0]) as f64,
            f32::from_bits(from.0[0]) as f64,
        ),
        0x44,
    );
    let same_y = !parity_even(
        fcom(
            f32::from_bits(origin.0[1]) as f64,
            f32::from_bits(from.0[1]) as f64,
        ),
        0x44,
    );
    if same_x && same_y {
        return combat_utilities_get_angle_to_projected_point_ov2(e, result, from, point);
    }
    e.with_stack(0x30 + 0x18, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x30 - offset;
        let origin_arg = frame.addr() + 0x30;
        let point_arg = origin_arg + 0xc;
        origin.store(e, origin_arg);
        point.store(e, point_arg);
        let offset = local(0xc);
        e.call(POINT_SUBTRACT, &args![point_arg, offset, origin_arg]);
        let squared = e.call(POINT_LENGTH_SQUARED, &args![offset]).f32();
        e.mem.set_f32(local(0x10), squared);
        let zero: f64 = e.global(DOUBLE_ZERO);
        if parity_even(fcom(squared as f64, zero), 0x44) {
            let minimum = load_f32(e, MIN_PROJECTION_DISTANCE);
            if !parity_even(fcom(squared as f64, minimum * minimum), 0x05) {
                let length = e.call(MATH_4579E0, &args![squared]).f32();
                e.mem.set_f32(local(0x14), length);
                let factor = float_arg(minimum / length as f64);
                let pushed = add_scaled(e, origin_arg, offset, factor, local(0x20), local(0x2c));
                PointArg::load(e, pushed).store(e, point_arg);
            }
        }
        let point = PointArg::load(e, point_arg);
        combat_utilities_get_angle_to_projected_point_ov2(e, result, from, point)
    })
}

// Translated from 009a8af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetAngleToProjectedPoint_ov2` (Xbox PDB): the angles
/// of the direction from `from` to `to`: x the negated `NiASin` of z over
/// the length (`NiPoint3::UnitizeGetLength`), y 0, z
/// `GetUnclampedZAngleFromVector`, x and z folded into -pi..pi.
pub fn combat_utilities_get_angle_to_projected_point_ov2(
    e: &mut Engine,
    result: Ptr,
    from: PointArg,
    to: PointArg,
) -> Ptr {
    e.with_stack(0x28 + 0x18, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x28 - offset;
        let from_arg = frame.addr() + 0x28;
        let to_arg = from_arg + 0xc;
        from.store(e, from_arg);
        to.store(e, to_arg);
        let direction = local(0x14);
        e.call(POINT_SUBTRACT, &args![to_arg, direction, from_arg]);
        copy_f32(e, direction + 8, local(0x1c));
        let length = e.call(POINT_UNITIZE_GET_LENGTH, &args![direction]).f32();
        e.mem.set_f32(local(0x20), length);
        store_f32(
            e,
            local(0x28),
            load_f32(e, local(0x1c)) / load_f32(e, local(0x20)),
        );
        let sine = e.mem.f32(local(0x28));
        let pitch = e.call(ASIN, &args![sine]).f32();
        e.mem.set_f32(local(0x24), pitch);
        store_f32(e, local(0x8), -load_f32(e, local(0x24)));
        wrap_angle(e, local(0x8));
        let yaw = e
            .call(GET_UNCLAMPED_Z_ANGLE_FROM_VECTOR, &args![direction])
            .f32();
        e.mem.set_f32(local(0x4), yaw);
        copy_f32(e, local(0x4), local(0x18));
        wrap_angle(e, local(0x18));
        let pitch = f32::from_bits(x87_quiet_bits(e.mem.u32(local(0x8))));
        let yaw = f32::from_bits(x87_quiet_bits(e.mem.u32(local(0x18))));
        e.call(POINT_SET, &args![result, pitch, 0.0f32, yaw]);
        result
    })
}

// Translated from 009a8bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetProjectedPointForTarget` (Xbox PDB):
/// `GetProjectedPointForTarget_ov3` for the target's firing location and
/// velocity. The firing-location mode is 2 for a high arc of a projectile
/// with gravity; otherwise 4 when the projectile's `004fd3c0` explosion
/// has a radius of at least `fCombatSplashDamageMinRadius` and more than
/// `fCombatSplashDamageMinDamage` damage, while `fn_009a7f80` does not
/// hold and the projectile is not faster than `fCombatSplashDamageMaxSpeed`;
/// else the attacker's combat controller's target segment in view (1
/// without one).
#[allow(clippy::too_many_arguments)]
pub fn combat_utilities_get_projected_point_for_target(
    e: &mut Engine,
    result: Ptr,
    attacker: Ptr,
    target: Ptr,
    from: Ptr,
    projectile: Ptr,
    speed: f32,
    high: bool,
) -> Ptr {
    let mut mode: u32 = 1;
    if is_actor(e, attacker) {
        let controller = e.vcall(attacker.addr(), 0x428, &[]).u32();
        if controller != 0 {
            mode = e
                .call(CONTROLLER_GET_TARGET_SEGMENT_IN_VIEW, &args![controller])
                .u32();
        }
    }
    let zero: f64 = e.global(DOUBLE_ZERO);
    let arcing = high && {
        let gravity = e.call(PROJECTILE_GRAVITY, &args![projectile]).f64();
        parity_even(fcom(gravity, zero), 0x44)
    };
    if arcing {
        mode = 2;
    } else {
        let explosion = e.call(PROJECTILE_EXPLOSION, &args![projectile]).u32();
        let mut fast = true;
        if !fn_009a7f80(e, projectile) {
            let projectile_speed = e.call(PROJECTILE_SPEED, &args![projectile]).f64();
            let limit = setting_f32(e, SETTING_COMBAT_SPLASH_DAMAGE_MAX_SPEED);
            fast = !parity_even(fcom(limit, projectile_speed), 0x05);
        }
        if explosion != 0 {
            let radius = e.call(EXPLOSION_GET_RADIUS, &args![explosion]).f64();
            let minimum = setting_f32(e, SETTING_COMBAT_SPLASH_DAMAGE_MIN_RADIUS);
            if !parity_even(fcom(minimum, radius), 0x41) && !fast {
                let damage = e.call(EXPLOSION_DAMAGE, &args![explosion]).f64();
                let minimum = setting_f32(e, SETTING_COMBAT_SPLASH_DAMAGE_MIN_DAMAGE);
                if !parity_even(fcom(minimum, damage), 0x05) {
                    mode = 4;
                }
            }
        }
    }
    e.with_stack(0x18, |e, frame| {
        let location = frame.addr();
        let velocity = frame.addr() + 0xc;
        combat_utilities_get_target_firing_location(e, Ptr::new(location), target, mode);
        combat_utilities_get_target_velocity(e, Ptr::new(velocity), target);
        let location = PointArg::load(e, location);
        let velocity = PointArg::load(e, velocity);
        let speed = f32::from_bits(x87_quiet_bits(speed.to_bits()));
        combat_utilities_get_projected_point_for_target_ov3(
            e, result, attacker, location, velocity, from, projectile, speed, high,
        )
    })
}

// Translated from 009a8d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetProjectedPointForTarget_ov2` (Xbox PDB):
/// `GetProjectedPointForTarget_ov3` for the point at `point` with the zero
/// velocity.
#[allow(clippy::too_many_arguments)]
pub fn combat_utilities_get_projected_point_for_target_ov2(
    e: &mut Engine,
    result: Ptr,
    attacker: Ptr,
    point: Ptr,
    from: Ptr,
    projectile: Ptr,
    speed: f32,
    high: bool,
) -> Ptr {
    let still = PointArg::load(e, ZERO_POINT);
    let location = PointArg::load(e, point.addr());
    let speed = f32::from_bits(x87_quiet_bits(speed.to_bits()));
    combat_utilities_get_projected_point_for_target_ov3(
        e, result, attacker, location, still, from, projectile, speed, high,
    )
}

// Translated from 009a8df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetProjectedPointForTarget_ov3` (Xbox PDB): the target
/// location itself when `fn_009a7f80` holds for the projectile; otherwise
/// `GetProjectedPointForTarget_ov4` from `from` (the attacker's firing
/// location when null) with the projectile's gravity and `speed` (its
/// `0069ef80` speed when 0).
#[allow(clippy::too_many_arguments)]
pub fn combat_utilities_get_projected_point_for_target_ov3(
    e: &mut Engine,
    result: Ptr,
    attacker: Ptr,
    target_location: PointArg,
    velocity: PointArg,
    from: Ptr,
    projectile: Ptr,
    speed: f32,
    high: bool,
) -> Ptr {
    if fn_009a7f80(e, projectile) {
        target_location.store(e, result.addr());
        return result;
    }
    e.with_stack(0x1c, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x1c - offset;
        let start = local(0x10);
        e.call(POINT_CONSTRUCT, &args![start]);
        if from.is_null() {
            let location =
                combat_utilities_get_attacker_firing_location(e, Ptr::new(local(0x1c)), attacker);
            PointArg::load(e, location.addr()).store(e, start);
        } else {
            PointArg::load(e, from.addr()).store(e, start);
        }
        let gravity = projectile_gravity(e, projectile) as f32;
        let zero: f64 = e.global(DOUBLE_ZERO);
        let mut speed = f32::from_bits(x87_quiet_bits(speed.to_bits()));
        if !parity_even(fcom(speed as f64, zero), 0x44) {
            speed = e.call(PROJECTILE_SPEED, &args![projectile]).f32();
        }
        let from = PointArg::load(e, start);
        combat_utilities_get_projected_point_for_target_ov4(
            e,
            result,
            from,
            target_location,
            velocity,
            speed,
            gravity,
            high,
        )
    })
}

// Translated from 009a8f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::GetProjectedPointForTarget_ov4` (Xbox PDB): where the
/// target will be when the projectile gets there, refined
/// `iAimingNumIterations` times (once for a still target): the flight time
/// to the current guess (distance over speed without gravity, the
/// `fn_009a7dd0` time of the `fn_009a7c60` launch angle with gravity) moves
/// the target location by its velocity. A guess closer than 1 or farther
/// than 8192 (or higher than 8192, with gravity) ends the refinement. With
/// gravity the guess's z is then `from`'s z plus the horizontal distance
/// times `0057dd70` of the last launch angle.
#[allow(clippy::too_many_arguments)]
pub fn combat_utilities_get_projected_point_for_target_ov4(
    e: &mut Engine,
    result: Ptr,
    from: PointArg,
    target_location: PointArg,
    velocity: PointArg,
    speed: f32,
    gravity: f32,
    high: bool,
) -> Ptr {
    e.with_stack(0x80 + 0x24, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x80 - offset;
        let from_arg = frame.addr() + 0x80;
        let target_arg = from_arg + 0xc;
        let velocity_arg = target_arg + 0xc;
        from.store(e, from_arg);
        target_location.store(e, target_arg);
        velocity.store(e, velocity_arg);
        let mut iterations = setting_u32(e, SETTING_AIMING_NUM_ITERATIONS) as i32;
        let zero: f64 = e.global(DOUBLE_ZERO);
        let still = velocity.0.iter().all(|word| {
            let component = f32::from_bits(*word) as f64;
            !parity_even(fcom(component, zero), 0x44)
        });
        if still {
            iterations = 1;
        }
        e.mem.set_f32(local(0x8), 0.0);
        let guess = local(0x14);
        target_location.store(e, guess);
        let one: f64 = e.global(DOUBLE_ONE);
        let far: f64 = e.global(DOUBLE_8192);
        if !parity_even(fcom(gravity as f64, zero), 0x44) {
            // No gravity: straight flight.
            e.mem.set_f32(local(0x18), 0.0);
            let mut index: i32 = 0;
            while index < iterations {
                let offset = local(0x28);
                e.call(POINT_SUBTRACT, &args![guess, offset, from_arg]);
                let distance = e.call(POINT_LENGTH, &args![offset]).f32();
                e.mem.set_f32(local(0x18), distance);
                let distance = distance as f64;
                if !parity_even(fcom(distance, one), 0x05) || fcom(distance, far) & 0x41 == 0 {
                    break;
                }
                store_f32(e, local(0x8), distance / speed as f64);
                let time = e.mem.f32(local(0x8));
                let moved = add_scaled(e, target_arg, velocity_arg, time, local(0x5c), local(0x68));
                PointArg::load(e, moved).store(e, guess);
                index += 1;
            }
        } else {
            e.mem.set_f32(local(0x38), 0.0);
            e.mem.set_f32(local(0x40), 0.0);
            e.mem.set_f32(local(0x3c), 0.0);
            let mut index: i32 = 0;
            while index < iterations {
                let offset = local(0x50);
                e.call(POINT_SUBTRACT, &args![guess, offset, from_arg]);
                copy_f32(e, offset + 8, local(0x40));
                e.mem.set_f32(offset + 8, 0.0);
                let distance = e.call(POINT_LENGTH, &args![offset]).f32();
                e.mem.set_f32(local(0x38), distance);
                let height = load_f32(e, local(0x40));
                let distance = distance as f64;
                if fcom(height, far) & 0x41 == 0
                    || !parity_even(fcom(distance, one), 0x05)
                    || fcom(distance, far) & 0x41 == 0
                {
                    break;
                }
                let distance = e.mem.f32(local(0x38));
                let height = e.mem.f32(local(0x40));
                let angle = fn_009a7c60(e, distance, height, speed, gravity, high);
                e.mem.set_f32(local(0x3c), angle);
                let time = fn_009a7dd0(e, distance, angle, speed);
                e.mem.set_f32(local(0x8), time);
                let moved = add_scaled(e, target_arg, velocity_arg, time, local(0x74), local(0x80));
                PointArg::load(e, moved).store(e, guess);
                index += 1;
            }
            let offset = local(0x34);
            e.call(POINT_SUBTRACT, &args![guess, offset, from_arg]);
            e.mem.set_f32(offset + 8, 0.0);
            let distance = e.call(POINT_LENGTH, &args![offset]).f32();
            e.mem.set_f32(local(0x38), distance);
            let angle = e.mem.f32(local(0x3c));
            let tangent = e.call(TRIG_57DD70, &args![angle]).f64();
            let z = tangent * load_f32(e, local(0x38)) + load_f32(e, from_arg + 8);
            store_f32(e, guess + 8, z);
        }
        PointArg::load(e, guess).store(e, result.addr());
        result
    })
}

// Translated from 009a9180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::ComputeOptimalWeaponRanges` (Xbox PDB): writes the
/// minimum, optimal and maximum ranges. Without a weapon: 512 and 1024
/// times the factors (`fn_009a9350`, `00508050`) of the controller's
/// `00980070` object; with one, the weapon's own (`004a0db0`, `00646f50`),
/// scaled the same way unless `00647790` holds. The maximum is the optimal
/// times `fCombatAbsoluteMaxRangeMult`. With a projectile (`00525a90` for
/// the controller's actor), the optimal is at most its range
/// (`fn_009a7f00`) times `fCombatProjectileMaxRangeOptimalMult`, the
/// maximum at most its range, the minimum at least its explosion's radius,
/// at most the optimal less 256 and at least 0.
pub fn combat_utilities_compute_optimal_weapon_ranges(
    e: &mut Engine,
    controller: Ptr,
    weapon: Ptr,
    minimum: Ptr,
    optimal: Ptr,
    maximum: Ptr,
) {
    let (minimum, optimal, maximum) = (minimum.addr(), optimal.addr(), maximum.addr());
    copy_f32(e, FLOAT_512, minimum);
    copy_f32(e, FLOAT_1024, optimal);
    let object: Ptr = e.call(CONTROLLER_980070, &args![controller]).ptr();
    let scale_by_object = |e: &mut Engine| {
        let factor = fn_009a9350(e, object) as f64;
        store_f32(e, minimum, factor * load_f32(e, minimum));
        let factor = e.call(VALUE_508050, &args![object]).f64();
        store_f32(e, optimal, factor * load_f32(e, optimal));
    };
    if weapon.is_null() {
        scale_by_object(e);
        let mult = setting_f32(e, SETTING_COMBAT_ABSOLUTE_MAX_RANGE_MULT);
        store_f32(e, maximum, load_f32(e, optimal) * mult);
        return;
    }
    let value = e.call(WEAPON_VALUE_4A0DB0, &args![weapon]).f32();
    e.mem.set_f32(minimum, value);
    let value = e.call(WEAPON_VALUE_646F50, &args![weapon]).f32();
    e.mem.set_f32(optimal, value);
    if !e.call(WEAPON_FLAG_647790, &args![weapon]).bool() {
        scale_by_object(e);
    }
    let mult = setting_f32(e, SETTING_COMBAT_ABSOLUTE_MAX_RANGE_MULT);
    store_f32(e, maximum, load_f32(e, optimal) * mult);
    let actor = e.call(CONTROLLER_ACTOR, &args![controller]).u32();
    let projectile: Ptr = e.call(WEAPON_PROJECTILE, &args![weapon, actor]).ptr();
    if projectile.is_null() {
        return;
    }
    let range = fn_009a7f00(e, projectile, 0.0);
    let mult = setting_f32(e, SETTING_COMBAT_PROJECTILE_MAX_RANGE_OPTIMAL_MULT);
    let limit = float_arg(range as f64 * mult);
    let current = e.mem.f32(optimal);
    let value = e.call(FLOAT_MIN, &args![current, limit]).f32();
    e.mem.set_f32(optimal, value);
    let current = e.mem.f32(maximum);
    let value = e.call(FLOAT_MIN, &args![current, range]).f32();
    e.mem.set_f32(maximum, value);
    let explosion = e.call(PROJECTILE_EXPLOSION, &args![projectile]).u32();
    if explosion != 0 {
        let radius = e.call(EXPLOSION_GET_RADIUS, &args![explosion]).f32();
        let current = e.mem.f32(minimum);
        let value = e.call(FLOAT_MAX, &args![current, radius]).f32();
        e.mem.set_f32(minimum, value);
    }
    let gap: f64 = e.global(DOUBLE_256);
    let limit = float_arg(load_f32(e, optimal) - gap);
    let current = e.mem.f32(minimum);
    let value = e.call(FLOAT_MIN, &args![current, limit]).f32();
    e.mem.set_f32(minimum, value);
    let current = e.mem.f32(minimum);
    let value = e.call(FLOAT_MAX, &args![current, 0.0f32]).f32();
    e.mem.set_f32(minimum, value);
}

// Translated from 009a9350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at +0x38 (thiscall).
pub fn fn_009a9350(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x38)
}

// Translated from 009a9370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::CalculateCoverPositions` (Xbox PDB): `fn_009a93a0`
/// with the parts of `cover` at +0x14 and +0x70.
pub fn combat_utilities_calculate_cover_positions(
    e: &mut Engine,
    controller: Ptr,
    cover: Ptr,
    left: Ptr,
    right: Ptr,
) {
    fn_009a93a0(
        e,
        controller,
        Ptr::new(cover.addr().wrapping_add(0x14)),
        Ptr::new(cover.addr().wrapping_add(0x70)),
        left,
        right,
    );
}

// Translated from 009a93a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_009a93e0` with the radius (`Actor::GetRadius`) of the controller's
/// actor (`0097ae90`).
pub fn fn_009a93a0(e: &mut Engine, controller: Ptr, edge: Ptr, info: Ptr, left: Ptr, right: Ptr) {
    let actor = e.call(CONTROLLER_ACTOR, &args![controller]).u32();
    let radius = e.call(ACTOR_GET_RADIUS, &args![actor]).f32();
    fn_009a93e0(e, radius, edge, info, left, right);
}

// Translated from 009a93e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The two positions at a cover edge. The base point is the edge's point
/// at +0x28 moved along `004b3800` of the edge direction (+0x40) and the up
/// vector by `fn_009a95f0(info)`, then back along the edge direction by
/// `radius + 16`. When `006e4820` does not hold, the left position is the
/// base point and so is the right one, unless `00982be0` holds or `info`'s
/// byte +4 is set: then it is the point at +0x28 less (byte +5 set) or the
/// point at +0x34 plus (clear) 64 along the same side vector, moved back the
/// same way. When `006e4820` holds, the right position is the base point and
/// the left one the base point moved back another 128 along the edge
/// direction. Nothing is done when both out pointers are null.
pub fn fn_009a93e0(e: &mut Engine, radius: f32, edge: Ptr, info: Ptr, left: Ptr, right: Ptr) {
    if left.is_null() && right.is_null() {
        return;
    }
    e.with_stack(0xac, |e, frame| {
        let local = |offset: u32| frame.addr() + 0xac - offset;
        let edge = edge.addr();
        let sixteen: f64 = e.global(DOUBLE_16);
        store_f32(e, local(0x14), radius as f64 + sixteen);
        copy_f32(e, FLOAT_64, local(0x10));
        copy_f32(e, FLOAT_128, local(0x34));
        let back = local(0x20);
        let back_factor = -e.mem.f32(local(0x14));
        e.call(POINT_TIMES, &args![edge + 0x40, back, back_factor]);
        let side = local(0xc);
        e.call(POINT_CROSS, &args![edge + 0x40, side, UP_VECTOR]);
        let width = fn_009a95f0(e, info);
        e.mem.set_f32(local(0x30), width);
        let width = e.mem.f32(local(0x30));
        let base = local(0x2c);
        let moved = add_scaled(e, edge + 0x28, side, width, local(0x40), local(0x4c));
        e.call(POINT_ADD, &args![moved, base, back]);
        if !e.call(EDGE_FLAG_6E4820, &args![edge]).bool() {
            if !left.is_null() {
                PointArg::load(e, base).store(e, left.addr());
            }
            if !right.is_null() {
                let wide =
                    e.call(EDGE_FLAG_982BE0, &args![edge]).bool() || e.mem.u8(info.addr() + 4) != 0;
                if !wide {
                    PointArg::load(e, base).store(e, right.addr());
                } else if e.mem.u8(info.addr() + 5) != 0 {
                    let step = e.mem.f32(local(0x10));
                    let scaled = e.call(POINT_TIMES, &args![side, local(0x58), step]).u32();
                    let inner = e
                        .call(POINT_SUBTRACT, &args![edge + 0x28, local(0x64), scaled])
                        .u32();
                    let point = e.call(POINT_ADD, &args![inner, local(0x70), back]).u32();
                    PointArg::load(e, point).store(e, right.addr());
                } else {
                    let step = e.mem.f32(local(0x10));
                    let outer = add_scaled(e, edge + 0x34, side, step, local(0x7c), local(0x88));
                    let point = e.call(POINT_ADD, &args![outer, local(0x94), back]).u32();
                    PointArg::load(e, point).store(e, right.addr());
                }
            }
        } else {
            if !right.is_null() {
                PointArg::load(e, base).store(e, right.addr());
            }
            if !left.is_null() {
                let step = -e.mem.f32(local(0x34));
                let point = add_scaled(e, base, edge + 0x40, step, local(0xa0), local(0xac));
                PointArg::load(e, point).store(e, left.addr());
            }
        }
    });
}

// Translated from 009a95f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `(byte 2 / 2 + byte 3) * byte 0` of the structure (thiscall).
pub fn fn_009a95f0(e: &mut Engine, this: Ptr) -> f32 {
    let two: f64 = e.global(DOUBLE_TWO);
    let base = this.addr();
    let value =
        (e.mem.u8(base + 2) as f64 / two + e.mem.u8(base + 3) as f64) * e.mem.u8(base) as f64;
    value as f32
}

// Translated from 009a9630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::IsActorUsingMelee` (Xbox PDB): true without a weapon
/// (that of what vtable +0x428 returns, `007031c0`, or
/// `Actor::GetCurrentWeapon`), for a weapon whose `006450c0` flag is set,
/// or when that object's plan contains action 7 or 8
/// (`CombatPlan::ContainsAction`).
pub fn combat_utilities_is_actor_using_melee(e: &mut Engine, actor: Ptr) -> bool {
    let controller = e.vcall(actor.addr(), 0x428, &[]).u32();
    let weapon = if controller != 0 {
        e.call(CONTROLLER_WEAPON, &args![controller]).u32()
    } else {
        e.call(ACTOR_GET_CURRENT_WEAPON, &args![actor]).u32()
    };
    if weapon == 0 || e.call(WEAPON_FLAG_6450C0, &args![weapon]).bool() {
        return true;
    }
    if controller == 0 {
        return false;
    }
    e.call(CONTAINS_ACTION, &args![controller, 7u32]).bool()
        || e.call(CONTAINS_ACTION, &args![controller, 8u32]).bool()
}

// Translated from 009a96c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::IsActorUsingRanged` (Xbox PDB): the actor has a
/// current weapon for which `004c0c30` holds.
pub fn combat_utilities_is_actor_using_ranged(e: &mut Engine, actor: Ptr) -> bool {
    let weapon = e.call(ACTOR_GET_CURRENT_WEAPON, &args![actor]).u32();
    weapon != 0 && e.call(WEAPON_FLAG_4C0C30, &args![weapon]).bool()
}

// Translated from 009a96f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::CanActorExistInSpace` (Xbox PDB):
/// `TESObjectREFR::GetRefPersists(reference)` or
/// `BGSWorldLocation::IsLocationLoaded(location)`.
pub fn combat_utilities_can_actor_exist_in_space(
    e: &mut Engine,
    reference: Ptr,
    location: Ptr,
) -> bool {
    e.call(REFR_GET_REF_PERSISTS, &args![reference]).bool()
        || e.call(WORLD_LOCATION_IS_LOADED, &args![location]).bool()
}

// Translated from 009a9720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::IsDangerousExplosion` (Xbox PDB): the explosion's
/// damage (`006a78f0`) is at least `fDangerousProjectileExplosionDamage`
/// and `006479a0(radius, damage, that setting)` is at least
/// `fDangerousProjectileExplosionRadius` (unordered values pass both).
pub fn combat_utilities_is_dangerous_explosion(e: &mut Engine, explosion: Ptr) -> bool {
    if explosion.is_null() {
        return false;
    }
    let damage = e.call(EXPLOSION_DAMAGE, &args![explosion]).f32();
    let minimum = setting_f32(e, SETTING_DANGEROUS_PROJECTILE_EXPLOSION_DAMAGE);
    if fcom(minimum, damage as f64) & 0x41 == 0 {
        return false;
    }
    let radius = e.call(EXPLOSION_GET_RADIUS, &args![explosion]).f32();
    let value = e
        .call(
            SETTING_VALUE,
            &args![SETTING_DANGEROUS_PROJECTILE_EXPLOSION_DAMAGE],
        )
        .u32();
    let setting = f32::from_bits(x87_quiet_bits(e.mem.u32(value)));
    let reach = e
        .call(EXPLOSION_FORMULA_6479A0, &args![radius, damage, setting])
        .f32();
    let minimum = setting_f32(e, SETTING_DANGEROUS_PROJECTILE_EXPLOSION_RADIUS);
    fcom(minimum, reach as f64) & 0x41 != 0
}

// Translated from 009a97b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sector `offset` steps from `sector`, wrapped into
/// `0..SECTOR_COUNT` (byte arithmetic, as the code stores it).
pub fn fn_009a97b0(e: &mut Engine, sector: i8, offset: i8) -> i8 {
    let count: i32 = e.global(SECTOR_COUNT);
    let mut value = (sector as i32 + offset as i32) as i8;
    if (value as i32) < 0 {
        value = (value as i32).wrapping_add(count) as i8;
    }
    if value as i32 >= count {
        value = (value as i32).wrapping_sub(count) as i8;
    }
    value
}

// Translated from 009a9800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `sector` or one of its `SECTOR_SPREAD` neighbours on either side
/// is taken in `mask`.
pub fn fn_009a9800(e: &mut Engine, mask: u32, sector: i8) -> bool {
    if sector_bit(sector) & mask != 0 {
        return true;
    }
    let mut step: i32 = 1;
    while step <= e.global::<i32>(SECTOR_SPREAD) {
        let before = fn_009a97b0(e, sector, step.wrapping_neg() as i8);
        if sector_bit(before) & mask != 0 {
            return true;
        }
        let after = fn_009a97b0(e, sector, step as i8);
        if sector_bit(after) & mask != 0 {
            return true;
        }
        step += 1;
    }
    false
}

// Translated from 009a98a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The free sector nearest to `sector` (`fn_009a9800` false), trying
/// `sector + i` then `sector - i` for `i` below half of `SECTOR_COUNT`; -1
/// when there is none.
pub fn fn_009a98a0(e: &mut Engine, mask: u32, sector: i8) -> i8 {
    let mut step: i32 = 0;
    while step < e.global::<i32>(SECTOR_COUNT) / 2 {
        let after = fn_009a97b0(e, sector, step as i8);
        if !fn_009a9800(e, mask, after) {
            return after;
        }
        let before = fn_009a97b0(e, sector, step.wrapping_neg() as i8);
        if !fn_009a9800(e, mask, before) {
            return before;
        }
        step += 1;
    }
    -1
}

// Translated from 009a9940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `mask` with `sector` and its `SECTOR_SPREAD` neighbours on either side
/// marked taken.
pub fn fn_009a9940(e: &mut Engine, mask: u32, sector: i8) -> u32 {
    let mut mask = mask | sector_bit(sector);
    let mut step: i32 = 1;
    while step <= e.global::<i32>(SECTOR_SPREAD) {
        let before = fn_009a97b0(e, sector, step.wrapping_neg() as i8);
        mask |= sector_bit(before);
        let after = fn_009a97b0(e, sector, step as i8);
        mask |= sector_bit(after);
        step += 1;
    }
    mask
}

// Translated from 009a99d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sector of an angle in degrees: `degrees / 360 * SECTOR_COUNT`
/// truncated (`_ftol2_sse`).
pub fn fn_009a99d0(e: &mut Engine, degrees: f32) -> i32 {
    let full_turn: f64 = e.global(DOUBLE_360);
    let count: i32 = e.global(SECTOR_COUNT);
    let value = degrees as f64 / full_turn * count as f64;
    e.call(FTOL, &args![value]).i32()
}

// Translated from 009a99f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The angle in degrees of a sector: `sector * 360 / SECTOR_COUNT`.
pub fn fn_009a99f0(e: &mut Engine, sector: i8) -> f32 {
    let full_turn: f64 = e.global(DOUBLE_360);
    let count: i32 = e.global(SECTOR_COUNT);
    (sector as f64 * full_turn / count as f64) as f32
}

// Translated from 009a9a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Spreads the attackers of `target` around it: for each of up to 64
/// entries of the target's attacker array
/// (`CombatTargetMap::GetTargetAttackerArray` on `00441110` of the object
/// at `011f1958`) whose vtable +0x428 object's plan contains action 7 or 8,
/// records that object, the attacker's position, its angle from the target
/// in degrees, the angle's sector and whether the object's `004fd3e0`
/// result returns 1 from its vtable +0x34. Those flagged get free sectors
/// first, then the others (`assign_sectors`). Returns whether `actor` was
/// among the recorded attackers.
pub fn fn_009a9a20(e: &mut Engine, target: Ptr, actor: Ptr) -> bool {
    if target.is_null() {
        return false;
    }
    const FRAME: u32 = 0x660;
    e.with_stack(FRAME, |e, frame| {
        let local = |offset: u32| frame.addr() + FRAME - offset;
        let mut found = false;
        let owner: u32 = e.global(TARGET_MAP_OWNER);
        let map = e.call(TARGET_MAP_441110, &args![owner]).u32();
        let array = e.call(GET_TARGET_ATTACKER_ARRAY, &args![map, target]).u32();
        if array == 0 {
            return false;
        }
        let target_position = local(0x618);
        let position = e.call(REFR_POSITION_436AA0, &args![target]).u32();
        PointArg::load(e, position).store(e, target_position);
        let entries = local(0x608);
        e.call(
            CONSTRUCT_ARRAY,
            &args![entries, 0x18u32, 0x40u32, ENTRY_CONSTRUCT],
        );
        e.call(MEMORY_FILL, &args![entries, 0u32, 0x600u32]);
        let mut recorded: u32 = 0;
        let mut flagged: u32 = 0;
        let mut mask: u32 = 0;
        let size = e.call(ARRAY_COUNT, &args![array]).u32();
        let size = e.call(WORD_MIN, &args![size, 0x40u32]).u32();
        let mut index: u32 = 0;
        while index < size {
            let slot = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
            let attacker = e.mem.u32(slot);
            let controller = if attacker != 0 {
                e.vcall(attacker, 0x428, &[]).u32()
            } else {
                0
            };
            if controller != 0
                && (e.call(CONTAINS_ACTION, &args![controller, 7u32]).bool()
                    || e.call(CONTAINS_ACTION, &args![controller, 8u32]).bool())
            {
                let entry = entries + recorded * 0x18;
                e.mem.set_u32(entry, controller);
                let position = e.call(REFR_POSITION_436AA0, &args![attacker]).u32();
                PointArg::load(e, position).store(e, entry + 4);
                let offset = e
                    .call(
                        POINT_SUBTRACT,
                        &args![entry + 4, local(0x654), target_position],
                    )
                    .u32();
                let angle = e.call(GET_Z_ANGLE_FROM_VECTOR, &args![offset]).f64();
                let to_degrees: f64 = e.global(DOUBLE_RADIANS_TO_DEGREES);
                store_f32(e, entry + 0x10, angle * to_degrees);
                let degrees = e.mem.f32(entry + 0x10);
                let sector = fn_009a99d0(e, degrees);
                e.mem.set_u8(entry + 0x14, sector as u8);
                if e.call(CONTROLLER_4FD3E0, &args![controller]).u32() != 0 {
                    let object = e.call(CONTROLLER_4FD3E0, &args![controller]).u32();
                    if e.vcall(object, 0x34, &[]).u32() == 1 {
                        e.mem.set_u8(entry + 0x15, 1);
                        flagged += 1;
                    }
                }
                recorded += 1;
                if attacker == actor.addr() {
                    found = true;
                }
            }
            index += 1;
        }
        if recorded != 0 {
            if flagged != 0 {
                assign_sectors(e, entries, recorded, &mut mask, true);
            }
            assign_sectors(e, entries, recorded, &mut mask, false);
        }
        found
    })
}

// Translated from 009a9f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` into the float at +0xe0 (thiscall; on what vtable +0x428
/// of an attacker returns, in `fn_009a9a20`).
pub fn fn_009a9f00(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0xe0, x87_quiet_bits(value.to_bits()));
}

// Translated from 009a9f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatUtilities::AddLOSDebugGeomtry` (Xbox PDB): adds a temporary debug
/// line (`004b3890`) from `from` to the end point (`from + (to - from) *
/// fraction` when `hit` is set, else `to`) in `color` (1, 1, 0, 1 by
/// default), and with a hit also a triangle marker (`MakeTriPoint`, size
/// 5) moved to the end point and a line on to `to`, both in `hit_color`
/// (1, 0, 0, 1 by default), each for `fDebugCombatProjectileLOSTime`
/// seconds (`TES::AddTempDebugObject`).
pub fn combat_utilities_add_los_debug_geomtry(
    e: &mut Engine,
    from: Ptr,
    to: Ptr,
    hit: u32,
    fraction: f32,
    color: Ptr,
    hit_color: Ptr,
) {
    e.with_stack(0x94, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x94 - offset;
        let hit_color_source = if hit_color.is_null() {
            e.call(
                COLOR_SET,
                &args![local(0x4c), 1.0f32, 0.0f32, 0.0f32, 1.0f32],
            )
            .u32()
        } else {
            hit_color.addr()
        };
        let hit_color = local(0x10);
        for i in 0..4 {
            let word = e.mem.u32(hit_color_source + 4 * i);
            e.mem.set_u32(hit_color + 4 * i, word);
        }
        let color_source = if color.is_null() {
            e.call(
                COLOR_SET,
                &args![local(0x60), 1.0f32, 1.0f32, 0.0f32, 1.0f32],
            )
            .u32()
        } else {
            color.addr()
        };
        let color = local(0x2c);
        for i in 0..4 {
            let word = e.mem.u32(color_source + 4 * i);
            e.mem.set_u32(color + 4 * i, word);
        }
        let end_source = if hit != 0 {
            let along = e.call(POINT_SUBTRACT, &args![to, local(0x70), from]).u32();
            let scaled = e
                .call(POINT_TIMES, &args![along, local(0x7c), fraction])
                .u32();
            e.call(POINT_ADD, &args![scaled, local(0x88), from]).u32()
        } else {
            to.addr()
        };
        let end = local(0x1c);
        PointArg::load(e, end_source).store(e, end);
        let tes_add = |e: &mut Engine, object: u32| {
            let seconds = e
                .call(
                    SETTING_VALUE,
                    &args![SETTING_DEBUG_COMBAT_PROJECTILE_LOS_TIME],
                )
                .u32();
            let seconds = f32::from_bits(x87_quiet_bits(e.mem.u32(seconds)));
            let tes: u32 = e.global(TES_OBJECT);
            e.call(TES_ADD_TEMP_DEBUG_OBJECT, &args![tes, object, seconds]);
        };
        let line = e
            .call(MAKE_LINE, &args![from, color, end, color, 1u32])
            .u32();
        tes_add(e, line);
        if hit != 0 {
            let size = f32::from_bits(x87_quiet_bits(e.mem.u32(FLOAT_5)));
            let marker = e.call(MAKE_TRI_POINT, &args![size, hit_color, 1u32]).u32();
            e.call(UPDATE_PROPERTIES, &args![marker]);
            e.call(NODE_SET_TRANSLATE, &args![marker, end]);
            tes_add(e, marker);
            let line = e
                .call(MAKE_LINE, &args![end, hit_color, to, hit_color, 1u32])
                .u32();
            tes_add(e, line);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a64d0, combat_utilities_get_distance_between_facing_actors(Ptr, Ptr, bool) -> f32),
        entry!(0x009a6770, combat_utilities_get_attacker_target_extent_sum(Ptr, Ptr) -> f32),
        entry!(0x009a6830, combat_utilities_compute_face_target_distance(Ptr, Ptr, f32) -> f32),
        entry!(0x009a68b0, combat_utilities_compute_face_target_location(Ptr, Ptr, Ptr, Ptr, f32) -> Ptr),
        entry!(0x009a69c0, combat_utilities_get_weapon_reach(Ptr, Ptr) -> f32),
        entry!(0x009a6a40, fn_009a6a40(i32) -> f32),
        entry!(0x009a6ae0, fn_009a6ae0(Ptr, Ptr, Ptr) -> bool),
        entry!(0x009a6d70, combat_utilities_is_target_within_firing_arc(Ptr, Ptr, f32, f32) -> bool),
        entry!(0x009a6e90, combat_utilities_calculate_projectile_los(Ptr, Ptr, f32, PointArg, PointArg, Ptr, Ptr, Ptr) -> bool),
        entry!(0x009a7580, combat_utilities_check_projectile_arc(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, f32) -> bool),
        entry!(0x009a7670, combat_utilities_check_projectile_arc_ov2(Ptr, Ptr, Ptr, PointArg, PointArg, PointArg, Ptr, Ptr, f32) -> bool),
        entry!(0x009a7940, combat_utilities_calculate_los(Ptr, Ptr, Ptr, Ptr, u32, u32) -> bool),
        entry!(0x009a7bc0, fn_009a7bc0(Ptr, u32, Ptr) -> Ptr),
        entry!(0x009a7c40, fn_009a7c40(Ptr) -> f32),
        entry!(0x009a7c60, fn_009a7c60(f32, f32, f32, f32, bool) -> f32),
        entry!(0x009a7d50, fn_009a7d50(Ptr, PointArg, f32, bool) -> f32),
        entry!(0x009a7dd0, fn_009a7dd0(f32, f32, f32) -> f32),
        entry!(0x009a7e00, combat_utilities_compute_projectile_time(Ptr, PointArg, PointArg, bool, f32) -> f32),
        entry!(0x009a7f00, fn_009a7f00(Ptr, f32) -> f32),
        entry!(0x009a7f80, fn_009a7f80(Ptr) -> bool),
        entry!(0x009a7fa0, combat_utilities_get_attacker_firing_z_offset(Ptr, bool) -> f32),
        entry!(0x009a8050, combat_utilities_get_attacker_firing_location(Ptr, Ptr) -> Ptr),
        entry!(0x009a8250, fn_009a8250(Ptr, Ptr) -> Ptr),
        entry!(0x009a82b0, combat_utilities_get_back_projected_attacker_firing_location(Ptr, Ptr) -> Ptr),
        entry!(0x009a8460, combat_utilities_get_target_firing_z_offset(Ptr, u32, bool) -> f32),
        entry!(0x009a8600, combat_utilities_get_target_firing_location(Ptr, Ptr, u32) -> Ptr),
        entry!(0x009a8660, combat_utilities_get_target_velocity(Ptr, Ptr) -> Ptr),
        entry!(0x009a86e0, combat_utilities_get_angle_to_projected_target(Ptr, Ptr, Ptr, f32, f32, u32) -> Ptr),
        entry!(0x009a87f0, combat_utilities_get_angle_to_projected_target_ov2(Ptr, PointArg, Ptr, f32, f32, u32) -> Ptr),
        entry!(0x009a88e0, combat_utilities_get_angle_to_projected_point(Ptr, Ptr, PointArg) -> Ptr),
        entry!(0x009a8970, fn_009a8970(Ptr, PointArg, PointArg, u32, PointArg) -> Ptr),
        entry!(0x009a8af0, combat_utilities_get_angle_to_projected_point_ov2(Ptr, PointArg, PointArg) -> Ptr),
        entry!(0x009a8bf0, combat_utilities_get_projected_point_for_target(Ptr, Ptr, Ptr, Ptr, Ptr, f32, bool) -> Ptr),
        entry!(0x009a8d80, combat_utilities_get_projected_point_for_target_ov2(Ptr, Ptr, Ptr, Ptr, Ptr, f32, bool) -> Ptr),
        entry!(0x009a8df0, combat_utilities_get_projected_point_for_target_ov3(Ptr, Ptr, PointArg, PointArg, Ptr, Ptr, f32, bool) -> Ptr),
        entry!(0x009a8f00, combat_utilities_get_projected_point_for_target_ov4(Ptr, PointArg, PointArg, PointArg, f32, f32, bool) -> Ptr),
        entry!(
            0x009a9180,
            combat_utilities_compute_optimal_weapon_ranges(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x009a9350, fn_009a9350(Ptr) -> f32),
        entry!(
            0x009a9370,
            combat_utilities_calculate_cover_positions(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x009a93a0, fn_009a93a0(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x009a93e0, fn_009a93e0(f32, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x009a95f0, fn_009a95f0(Ptr) -> f32),
        entry!(0x009a9630, combat_utilities_is_actor_using_melee(Ptr) -> bool),
        entry!(0x009a96c0, combat_utilities_is_actor_using_ranged(Ptr) -> bool),
        entry!(0x009a96f0, combat_utilities_can_actor_exist_in_space(Ptr, Ptr) -> bool),
        entry!(0x009a9720, combat_utilities_is_dangerous_explosion(Ptr) -> bool),
        entry!(0x009a97b0, fn_009a97b0(i8, i8) -> i8),
        entry!(0x009a9800, fn_009a9800(u32, i8) -> bool),
        entry!(0x009a98a0, fn_009a98a0(u32, i8) -> i8),
        entry!(0x009a9940, fn_009a9940(u32, i8) -> u32),
        entry!(0x009a99d0, fn_009a99d0(f32) -> i32),
        entry!(0x009a99f0, fn_009a99f0(i8) -> f32),
        entry!(0x009a9a20, fn_009a9a20(Ptr, Ptr) -> bool),
        entry!(0x009a9f00, fn_009a9f00(Ptr, f32)),
        entry!(
            0x009a9f20,
            combat_utilities_add_los_debug_geomtry(Ptr, Ptr, u32, f32, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret_u(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn ret_f(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn arg_f(words: &[u32], index: usize) -> f64 {
        f32::from_bits(words[index]) as f64
    }

    fn get3(e: &Engine, address: u32) -> [f32; 3] {
        [
            e.mem.f32(address),
            e.mem.f32(address + 4),
            e.mem.f32(address + 8),
        ]
    }

    fn set3(e: &mut Engine, address: u32, value: [f32; 3]) {
        for (i, v) in value.iter().enumerate() {
            e.mem.set_f32(address + 4 * i as u32, *v);
        }
    }

    fn point(e: &mut Engine, value: [f32; 3]) -> u32 {
        let address = e.mem.alloc(12);
        set3(e, address, value);
        address
    }

    fn arg_point(value: [f32; 3]) -> PointArg {
        PointArg::from_floats(value[0], value[1], value[2])
    }

    /// An object whose vtable slot `s` calls the address of the slot itself,
    /// so `on` can stand in for any slot of any object.
    fn object(e: &mut Engine) -> Ptr {
        let this = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x500);
        for slot in (0..0x500).step_by(4) {
            e.mem.set_u32(vtable + slot, vtable + slot);
        }
        e.mem.set_u32(this, vtable);
        Ptr::new(this)
    }

    fn on(
        e: &mut Engine,
        this: Ptr,
        slot: u32,
        f: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        let vtable = e.mem.u32(this.addr());
        e.register_double(vtable + slot, f);
    }

    fn returns(e: &mut Engine, address: u32, value: Ret) {
        e.register_double(address, move |_, _| value);
    }

    fn actor(e: &mut Engine) -> Ptr {
        let this = object(e);
        on(e, this, 0x100, |_, _| ret_u(1));
        this
    }

    fn reference(e: &mut Engine) -> Ptr {
        let this = object(e);
        on(e, this, 0x100, |_, _| ret_u(0));
        this
    }

    /// Slot 0x1f4 of `this` returns a point holding `position`.
    fn place(e: &mut Engine, this: Ptr, position: [f32; 3]) {
        let address = point(e, position);
        on(e, this, 0x1f4, move |_, _| ret_u(address));
    }

    /// Slots 0x1dc / 0x1d8 return points whose y and z are the given values.
    fn bounds(e: &mut Engine, this: Ptr, top: [f32; 3], bottom: [f32; 3]) {
        let top = point(e, top);
        let bottom = point(e, bottom);
        on(e, this, 0x1dc, move |_, _| ret_u(top));
        on(e, this, 0x1d8, move |_, _| ret_u(bottom));
    }

    fn log_of(e: &mut Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn set_setting(e: &mut Engine, setting: u32, value: f32) {
        e.mem.set_f32(setting + 4, value);
    }

    fn sub3(e: &Engine, a: u32, b: u32) -> [f32; 3] {
        let (a, b) = (get3(e, a), get3(e, b));
        [
            (a[0] as f64 - b[0] as f64) as f32,
            (a[1] as f64 - b[1] as f64) as f32,
            (a[2] as f64 - b[2] as f64) as f32,
        ]
    }

    /// The engine with the exe's constants, the settings pages and plain
    /// doubles for the math and point helpers this unit calls.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_a000,
            0x0101_d000,
            0x0101_e000,
            0x0101_f000,
            0x0102_1000,
            0x0102_3000,
            0x0102_4000,
            0x0102_9000,
            0x0102_c000,
            0x0102_e000,
            0x0102_f000,
            0x0103_0000,
            0x0103_c000,
            0x0104_e000,
            0x0106_b000,
            0x0108_4000,
            0x0108_e000,
            0x011a_5000,
            0x011a_9000,
            0x011c_e000,
            0x011c_f000,
            0x011d_e000,
            0x011f_1000,
            0x011f_4000,
            0x0120_2000,
        ] {
            e.map(page, 0x1000);
        }
        let pi = std::f32::consts::PI;
        e.set_global(DOUBLE_FLT_MAX, f32::MAX as f64);
        e.set_global(DOUBLE_MINUS_ONE, -1.0f64);
        e.set_global(DOUBLE_HALF, 0.5f64);
        e.set_global(DOUBLE_DEGREES_TO_RADIANS, (pi / 180.0) as f64);
        e.set_global(DOUBLE_RADIANS_TO_DEGREES, 57.295_776_367_187_5f64);
        e.set_global(DOUBLE_128, 128.0f64);
        e.set_global(DOUBLE_180, 180.0f64);
        e.set_global(DOUBLE_360, 360.0f64);
        e.set_global(DOUBLE_3, 3.0f64);
        e.set_global(FLOAT_FLT_MIN, f32::MIN_POSITIVE);
        e.set_global(DOUBLE_TWO_PI, (2.0 * pi) as f64);
        e.set_global(DOUBLE_PI, pi as f64);
        e.set_global(DOUBLE_MINUS_PI, -pi as f64);
        e.set_global(DOUBLE_HALF_PI, (pi / 2.0) as f64);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.set_global(DOUBLE_TWO, 2.0f64);
        e.set_global(DOUBLE_QUARTER, 0.25f64);
        e.set_global(DOUBLE_THREE_QUARTERS, 0.75f64);
        e.set_global(DOUBLE_NINE_TENTHS, 0.9f32 as f64);
        e.set_global(DOUBLE_TENTH, 0.1f32 as f64);
        e.set_global(DOUBLE_16, 16.0f64);
        e.set_global(DOUBLE_256, 256.0f64);
        e.set_global(DOUBLE_8192, 8192.0f64);
        e.set_global(FLOAT_PI, pi);
        e.set_global(FLOAT_FLT_MAX, f32::MAX);
        e.set_global(FLOAT_512, 512.0f32);
        e.set_global(FLOAT_1024, 1024.0f32);
        e.set_global(FLOAT_64, 64.0f32);
        e.set_global(FLOAT_128, 128.0f32);
        e.set_global(FLOAT_5, 5.0f32);
        e.set_global(SECTOR_COUNT, 30i32);
        e.set_global(SECTOR_SPREAD, 2i32);
        e.set_global(MIN_PROJECTION_DISTANCE, 1024.0f32);
        set3(&mut e, UP_VECTOR, [0.0, 0.0, 1.0]);
        e.set_global(GRAVITY_FACTOR, 1.0f32);

        for setting in [SETTING_VALUE, SETTING_BOOL_VALUE, SETTING_INT_VALUE] {
            e.register(setting, |_, a| ret_u(a[0] + 4));
        }
        e.register(POINT_SUBTRACT, |e, a| {
            let value = sub3(e, a[0], a[2]);
            set3(e, a[1], value);
            ret_u(a[1])
        });
        e.register(POINT_ADD, |e, a| {
            let (x, y) = (get3(e, a[0]), get3(e, a[2]));
            let sum = [
                (x[0] as f64 + y[0] as f64) as f32,
                (x[1] as f64 + y[1] as f64) as f32,
                (x[2] as f64 + y[2] as f64) as f32,
            ];
            set3(e, a[1], sum);
            ret_u(a[1])
        });
        e.register(POINT_TIMES, |e, a| {
            let (x, f) = (get3(e, a[0]), arg_f(a, 2));
            let product = [
                (f * x[0] as f64) as f32,
                (f * x[1] as f64) as f32,
                (f * x[2] as f64) as f32,
            ];
            set3(e, a[1], product);
            ret_u(a[1])
        });
        e.register(POINT_SCALE, |e, a| {
            let (x, f) = (get3(e, a[0]), arg_f(a, 1));
            let product = [
                (x[0] as f64 * f) as f32,
                (x[1] as f64 * f) as f32,
                (x[2] as f64 * f) as f32,
            ];
            set3(e, a[0], product);
            ret_u(a[0])
        });
        e.register(POINT_LENGTH, |e, a| {
            let x = get3(e, a[0]).map(|v| v as f64);
            ret_f((x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt() as f32 as f64)
        });
        e.register(POINT_LENGTH_XY, |e, a| {
            let x = get3(e, a[0]).map(|v| v as f64);
            ret_f((x[0] * x[0] + x[1] * x[1]).sqrt() as f32 as f64)
        });
        e.register(POINT_LENGTH_SQUARED, |e, a| {
            let x = get3(e, a[0]).map(|v| v as f64);
            ret_f((x[0] * x[0] + x[1] * x[1] + x[2] * x[2]) as f32 as f64)
        });
        e.register(POINT_UNITIZE, |e, a| {
            let x = get3(e, a[0]).map(|v| v as f64);
            let length = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
            let unit = x.map(|v| (v / length) as f32);
            set3(e, a[0], unit);
            Ret::default()
        });
        e.register(POINT_UNITIZE_GET_LENGTH, |e, a| {
            let x = get3(e, a[0]).map(|v| v as f64);
            let length = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
            let unit = x.map(|v| (v / length) as f32);
            set3(e, a[0], unit);
            ret_f(length as f32 as f64)
        });
        e.register(POINT_SET, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret_u(a[0])
        });
        e.register(POINT2_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret_u(a[0])
        });
        e.register(POINT2_DOT, |e, a| {
            let (x, y) = (get3(e, a[0]), get3(e, a[1]));
            ret_f((x[0] as f64 * y[0] as f64 + x[1] as f64 * y[1] as f64) as f32 as f64)
        });
        e.register(POINT_CROSS, |e, a| {
            let (x, y) = (get3(e, a[0]), get3(e, a[2]));
            let cross = [
                x[1] * y[2] - x[2] * y[1],
                x[2] * y[0] - x[0] * y[2],
                x[0] * y[1] - x[1] * y[0],
            ];
            set3(e, a[1], cross);
            ret_u(a[1])
        });
        e.register(POINT_CONSTRUCT, |_, a| ret_u(a[0]));
        e.register(NO_OP_483710, |_, _| Ret::default());
        e.register(FLOAT_MAX, |_, a| {
            let (first, second) = (arg_f(a, 0), arg_f(a, 1));
            ret_f(if first > second { first } else { second })
        });
        e.register(FLOAT_MIN, |_, a| {
            let (first, second) = (arg_f(a, 0), arg_f(a, 1));
            ret_f(if first < second { first } else { second })
        });
        e.register(FLOAT_ABS, |_, a| ret_f(arg_f(a, 0).abs()));
        e.register(FLOAT_ABS_2, |_, a| ret_f(arg_f(a, 0).abs()));
        e.register(MATH_4579E0, |_, a| ret_f(arg_f(a, 0).sqrt() as f32 as f64));
        e.register(TRIG_4E44B0, |_, a| ret_f(arg_f(a, 0).sin() as f32 as f64));
        e.register(TRIG_4E4470, |_, a| ret_f(arg_f(a, 0).cos() as f32 as f64));
        e.register(0x005b_9e80, |_, a| ret_f(arg_f(a, 0).cos() as f32 as f64));
        e.register(TRIG_57DD70, |_, a| ret_f(arg_f(a, 0).tan() as f32 as f64));
        e.register(MATH_755660, |_, a| ret_f(arg_f(a, 0).atan() as f32 as f64));
        e.register(ASIN, |_, a| ret_f(arg_f(a, 0).asin() as f32 as f64));
        e.register(SIN_COS, |e, a| {
            let angle = arg_f(a, 0);
            e.mem.set_f32(a[1], angle.sin() as f32);
            e.mem.set_f32(a[2], angle.cos() as f32);
            Ret::default()
        });
        e.register(GET_UNCLAMPED_Z_ANGLE_FROM_VECTOR, |e, a| {
            let x = get3(e, a[0]);
            ret_f((x[0] as f64).atan2(x[1] as f64) as f32 as f64)
        });
        e.register(GET_Z_ANGLE_FROM_VECTOR, |e, a| {
            let x = get3(e, a[0]);
            ret_f((x[0] as f64).atan2(x[1] as f64) as f32 as f64)
        });
        e.register(CLAMP_ANGLE, |_, a| ret_f(arg_f(a, 0)));
        e.register(ANGLE_DIFFERENCE, |_, a| ret_f(arg_f(a, 1) - arg_f(a, 0)));
        e.register(FTOL, |_, a| ret_u(f64::take(a, &mut 0) as i32 as u32));
        e
    }

    // ---- 009a64d0 ----------------------------------------------------------------------------

    fn facing_pair(e: &mut Engine, distance: f32) -> (Ptr, Ptr) {
        let attacker = actor(e);
        let target = actor(e);
        returns(e, REFR_GET_DISTANCE_FROM_REFERENCE, ret_f(distance as f64));
        returns(e, ACTOR_GET_FORWARD_LENGTH, ret_f(10.0));
        place(e, attacker, [0.0, 0.0, 0.0]);
        place(e, target, [30.0, 40.0, 5.0]);
        bounds(e, attacker, [0.0, 0.0, 100.0], [0.0, 0.0, 0.0]);
        bounds(e, target, [0.0, 0.0, 100.0], [0.0, 0.0, 0.0]);
        (attacker, target)
    }

    #[test]
    fn distance_between_facing_actors_branches() {
        let mut e = engine();
        let (attacker, target) = facing_pair(&mut e, 500.0);
        // No target: 0.
        let none = e.call(0x009a_64d0, &args![attacker, 0u32, false]).f32();
        assert_eq!(none, 0.0);
        // FLT_MAX is returned as it is.
        returns(
            &mut e,
            REFR_GET_DISTANCE_FROM_REFERENCE,
            ret_f(f32::MAX as f64),
        );
        let far = e.call(0x009a_64d0, &args![attacker, target, false]).f32();
        assert_eq!(far, f32::MAX);
        // Both flagged, spans overlap: the flat distance 50 less 2 * 10.
        returns(&mut e, REFR_GET_DISTANCE_FROM_REFERENCE, ret_f(500.0));
        returns(&mut e, REFR_BYTE_14D, ret_u(1));
        let flat = e.call(0x009a_64d0, &args![attacker, target, false]).f32();
        assert_eq!(flat, 30.0);
        // Not flagged and no level check: the reference distance.
        returns(&mut e, REFR_BYTE_14D, ret_u(0));
        let plain = e.call(0x009a_64d0, &args![attacker, target, false]).f32();
        assert_eq!(plain, 480.0);
        // Level check with a height difference of 5 below the slope setting.
        set_setting(&mut e, SETTING_AI_COMBAT_SLOPE_DIFFERENCE, 10.0);
        let level = e.call(0x009a_64d0, &args![attacker, target, true]).f32();
        assert_eq!(level, 480.0);
        // At or above it: the flat distance.
        set_setting(&mut e, SETTING_AI_COMBAT_SLOPE_DIFFERENCE, 5.0);
        let slope = e.call(0x009a_64d0, &args![attacker, target, true]).f32();
        assert_eq!(slope, 30.0);
        // Spans that do not overlap: the reference distance.
        bounds(&mut e, target, [0.0, 0.0, 300.0], [0.0, 0.0, 200.0]);
        let apart = e.call(0x009a_64d0, &args![attacker, target, true]).f32();
        assert_eq!(apart, 480.0);
    }

    // ---- 009a6770 ----------------------------------------------------------------------------

    #[test]
    fn extent_sum_adds_actor_length_and_scaled_bound() {
        let mut e = engine();
        let attacker = actor(&mut e);
        let target = reference(&mut e);
        returns(&mut e, ACTOR_GET_FORWARD_LENGTH, ret_f(10.0));
        returns(&mut e, REFR_GET_SCALE, ret_f(2.0));
        bounds(&mut e, target, [0.0, 3.0, 0.0], [0.0, 0.0, 0.0]);
        let sum = e.call(0x009a_6770, &args![attacker, target]).f32();
        assert_eq!(sum, 16.0);
        let alone = e.call(0x009a_6770, &args![attacker, 0u32]).f32();
        assert_eq!(alone, 10.0);
        let swapped = e.call(0x009a_6770, &args![target, attacker]).f32();
        assert_eq!(swapped, 16.0);
    }

    // ---- 009a6830 ----------------------------------------------------------------------------

    #[test]
    fn face_target_distance_uses_half_the_reach() {
        let mut e = engine();
        let attacker = actor(&mut e);
        let target = actor(&mut e);
        returns(&mut e, ACTOR_GET_FORWARD_LENGTH, ret_f(10.0));
        let given = e.call(0x009a_6830, &args![attacker, target, 40.0f32]).f32();
        assert_eq!(given, 40.0);
        // -1: the reach of the current weapon (none: vtable +0x380).
        returns(&mut e, ACTOR_GET_CURRENT_WEAPON, ret_u(0));
        returns(&mut e, REFR_GET_SCALE, ret_f(1.0));
        on(&mut e, attacker, 0x380, |_, _| ret_f(60.0));
        let weapon = e.call(0x009a_6830, &args![attacker, target, -1.0f32]).f32();
        assert_eq!(weapon, 50.0);
        // Never below 0.
        let negative = e
            .call(0x009a_6830, &args![attacker, target, -100.0f32])
            .f32();
        assert_eq!(negative, 0.0);
    }

    // ---- 009a68b0 ----------------------------------------------------------------------------

    #[test]
    fn face_target_location_without_facing_stays_at_the_point() {
        let mut e = engine();
        let actor_ref = actor(&mut e);
        let target = reference(&mut e);
        returns(&mut e, ACTOR_GET_FORWARD_LENGTH, ret_f(10.0));
        returns(&mut e, REFR_GET_SCALE, ret_f(1.0));
        bounds(&mut e, target, [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]);
        on(&mut e, actor_ref, 0x428, |_, _| ret_u(0));
        place(&mut e, actor_ref, [0.0, 10.0, 0.0]);
        let from = point(&mut e, [5.0, 6.0, 7.0]);
        let result = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        let back = e
            .call(
                0x009a_68b0,
                &args![result, actor_ref, target, from, 20.0f32],
            )
            .u32();
        assert_eq!(back, result);
        assert_eq!(get3(&e, result), [5.0, 6.0, 7.0]);
        assert_eq!(log_of(&mut e, POINT_UNITIZE).len(), 1);
        let scale = log_of(&mut e, POINT_SCALE);
        assert_eq!(f32::from_bits(scale[0][1]), 21.0);
    }

    #[test]
    fn face_target_location_turns_by_the_process_angle() {
        let mut e = engine();
        let (target, actor_ref, _) = attackers_setup(&mut e);
        returns(&mut e, ACTOR_GET_FORWARD_LENGTH, ret_f(10.0));
        let process = e.mem.alloc(0x100);
        on(&mut e, actor_ref, 0x428, move |_, _| ret_u(process));
        returns(&mut e, PROCESS_ANGLE_8E26A0, ret_f(90.0));
        let from = point(&mut e, [0.0, 0.0, 0.0]);
        let result = e.mem.alloc(12);
        e.call(
            0x009a_68b0,
            &args![result, actor_ref, target, from, 20.0f32],
        );
        // sin(90 degrees) = 1 along x, cos along y; distance 20 / 2 + 20.
        let value = get3(&e, result);
        assert_eq!(value[0], 30.0);
        assert!(value[1].abs() < 1e-5);
    }

    // ---- 009a69c0 ----------------------------------------------------------------------------

    #[test]
    fn weapon_reach_branches() {
        let mut e = engine();
        let actor_ref = actor(&mut e);
        returns(&mut e, REFR_GET_SCALE, ret_f(2.0));
        on(&mut e, actor_ref, 0x380, |_, _| ret_f(30.0));
        assert_eq!(e.call(0x009a_69c0, &args![actor_ref, 0u32]).f32(), 60.0);
        let weapon = e.mem.alloc(4);
        returns(&mut e, WEAPON_FLAG_6450C0, ret_u(0));
        assert_eq!(e.call(0x009a_69c0, &args![actor_ref, weapon]).f32(), 256.0);
        returns(&mut e, WEAPON_FLAG_6450C0, ret_u(1));
        returns(&mut e, WEAPON_VALUE_6447F0, ret_f(1.5));
        e.register(CALC_WEAPON_REACH, |_, a| ret_f(arg_f(a, 0) * 10.0));
        assert_eq!(e.call(0x009a_69c0, &args![actor_ref, weapon]).f32(), 30.0);
    }

    // ---- 009a6a40 ----------------------------------------------------------------------------

    #[test]
    fn hit_cone_setting_by_group() {
        let mut e = engine();
        set_setting(&mut e, SETTING_COMBAT_OVERHEAD_HIT_CONE_ANGLE, 1.0);
        set_setting(&mut e, SETTING_COMBAT_SWEEP_HIT_CONE_ANGLE, 2.0);
        set_setting(&mut e, SETTING_COMBAT_UPPERCUT_HIT_CONE_ANGLE, 3.0);
        set_setting(&mut e, SETTING_COMBAT_HIT_CONE_ANGLE, 4.0);
        let cases = [
            (0x5bi32, 4.0f32),
            (0x5c, 1.0),
            (0x5d, 1.0),
            (0x5e, 2.0),
            (0x60, 2.0),
            (0x61, 3.0),
            (0x62, 4.0),
            (0x66, 4.0),
            (0x67, 4.0),
        ];
        for (group, cone) in cases {
            assert_eq!(e.call(0x009a_6a40, &args![group]).f32(), cone, "{group:#x}");
        }
    }

    // ---- 009a6ae0 ----------------------------------------------------------------------------

    #[test]
    fn hit_cone_test_compares_the_heading_offset() {
        let mut e = engine();
        let attacker = actor(&mut e);
        let target = actor(&mut e);
        place(&mut e, attacker, [0.0, 0.0, 0.0]);
        place(&mut e, target, [0.0, 200.0, 0.0]);
        on(&mut e, attacker, 0x2bc, |_, _| ret_f(0.5));
        on(&mut e, target, 0x22c, |_, _| ret_u(0));
        set_setting(&mut e, SETTING_COMBAT_HIT_CONE_ANGLE, 30.0);
        let angle = e.mem.alloc(4);
        let degrees = (0.5f64 * 57.295_776_367_187_5) as f32;
        assert!(e.call(0x009a_6ae0, &args![attacker, target, angle]).bool());
        assert_eq!(e.mem.f32(angle), degrees);
        set_setting(&mut e, SETTING_COMBAT_HIT_CONE_ANGLE, 20.0);
        assert!(!e.call(0x009a_6ae0, &args![attacker, target, 0u32]).bool());
        // A dead target widens the cone.
        on(&mut e, target, 0x22c, |_, _| ret_u(1));
        set_setting(&mut e, SETTING_COMBAT_DEAD_ACTOR_HIT_CONE_MULT, 2.0);
        assert!(e.call(0x009a_6ae0, &args![attacker, target, 0u32]).bool());
        // Within 128 units the cone is tripled.
        on(&mut e, target, 0x22c, |_, _| ret_u(0));
        place(&mut e, target, [0.0, 100.0, 0.0]);
        assert!(e.call(0x009a_6ae0, &args![attacker, target, 0u32]).bool());
        // Above 180 degrees the angle is folded.
        on(&mut e, attacker, 0x2bc, |_, _| ret_f(6.0));
        place(&mut e, target, [0.0, 200.0, 0.0]);
        e.call(0x009a_6ae0, &args![attacker, target, angle]);
        let raw = (6.0f64 * 57.295_776_367_187_5) as f32;
        assert_eq!(e.mem.f32(angle), (raw as f64 - 360.0).abs() as f32);
    }

    // ---- 009a6d70 ----------------------------------------------------------------------------

    /// A reference (not an actor) at `position` with flat bounds, so its
    /// firing location is its position.
    fn flat_reference(e: &mut Engine, position: [f32; 3]) -> Ptr {
        let this = reference(e);
        place(e, this, position);
        bounds(e, this, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        this
    }

    #[test]
    fn firing_arc_checks_half_of_each_arc() {
        let mut e = engine();
        let shooter = flat_reference(&mut e, [0.0, 0.0, 0.0]);
        returns(&mut e, MOBILE_GET_ACTUAL_HEADING, ret_f(0.2f32 as f64));
        returns(&mut e, MOBILE_GET_LOOKING, ret_f(0.0));
        let target = point(&mut e, [0.0, 100.0, 0.0]);
        assert!(e
            .call(0x009a_6d70, &args![shooter, target, 0.5f32, 7.0f32])
            .bool());
        assert!(!e
            .call(0x009a_6d70, &args![shooter, target, 0.3f32, 7.0f32])
            .bool());
        // Arcs of 2pi and more are not checked.
        assert!(e
            .call(0x009a_6d70, &args![shooter, target, 7.0f32, 7.0f32])
            .bool());
        // The vertical arc against the looking angle.
        returns(&mut e, MOBILE_GET_LOOKING, ret_f(0.4f32 as f64));
        assert!(!e
            .call(0x009a_6d70, &args![shooter, target, 7.0f32, 0.5f32])
            .bool());
        assert!(e
            .call(0x009a_6d70, &args![shooter, target, 7.0f32, 1.0f32])
            .bool());
    }

    // ---- 009a6e90 ----------------------------------------------------------------------------

    /// Doubles for the pick machinery; `hits` lists what each projectile
    /// pick returns (hit object, fraction).
    fn pick_setup(e: &mut Engine, hits: Vec<(u32, f32)>) {
        for address in [
            RAY_COLLECTOR_CONSTRUCT,
            RAY_COLLECTOR_DESTRUCT,
            PICK_DATA_CONSTRUCT,
            PICK_DATA_SET_FILTER,
            PICK_DATA_SET_COLLECTOR,
            PICK_DATA_SET_RAY_COLLECTOR,
            PICK_DATA_SET_START,
            PICK_DATA_SET_END,
            FILTER_SET_LAYER,
            FILTER_SET_HIGH_HALF,
            LOS_COLLECTOR_DESTRUCT,
            0x008c_71b0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let filter = e.mem.alloc(4);
        returns(e, REFR_COLLISION_FILTER, ret_u(filter));
        returns(e, FILTER_HIGH_HALF, ret_u(0));
        returns(e, REFR_PARENT_CELL, ret_u(0x1234));
        returns(e, CELL_PICK, ret_u(0));
        let mut next = 0;
        e.register_double(PROJECTILE_PICK, move |e, a| {
            let (hit, fraction) = hits.get(next).copied().unwrap_or((0, 0.0));
            next += 1;
            e.mem.set_f32(a[3], fraction);
            ret_u(hit)
        });
        e.register(FIND_REFERENCE_FOR_3D, |_, a| ret_u(a[0] + 1));
    }

    #[test]
    fn projectile_los_straight_reports_the_hit() {
        let mut e = engine();
        let shooter = actor(&mut e);
        pick_setup(&mut e, vec![(0x500, 0.25)]);
        let hit_point = e.mem.alloc(12);
        let hit_reference = e.mem.alloc(4);
        let fraction = e.mem.alloc(4);
        let from = arg_point([0.0, 0.0, 0.0]);
        let to = arg_point([0.0, 100.0, 40.0]);
        let hit = e
            .call(
                0x009a_6e90,
                &args![
                    shooter,
                    0u32,
                    0.0f32,
                    from,
                    to,
                    hit_point,
                    hit_reference,
                    fraction
                ],
            )
            .bool();
        assert!(hit);
        assert_eq!(get3(&e, hit_point), [0.0, 25.0, 10.0]);
        assert_eq!(e.mem.u32(hit_reference), 0x501);
        assert_eq!(e.mem.f32(fraction), 0.25);
        // Nothing hit.
        pick_setup(&mut e, vec![]);
        let hit = e
            .call(
                0x009a_6e90,
                &args![shooter, 0u32, 0.0f32, from, to, 0u32, 0u32, 0u32],
            )
            .bool();
        assert!(!hit);
    }

    #[test]
    fn projectile_los_arc_walks_the_segments() {
        let mut e = engine();
        let shooter = actor(&mut e);
        let projectile = e.mem.alloc(4);
        returns(&mut e, PROJECTILE_GRAVITY, ret_f(10.0));
        e.mem
            .set_u32(SETTING_BALLISTIC_PROJECTILE_PATH_PICK_SEGMENTS + 4, 4);
        pick_setup(&mut e, vec![(0, 0.0), (0x700, 0.5)]);
        let fraction = e.mem.alloc(4);
        let from = arg_point([0.0, 0.0, 0.0]);
        let to = arg_point([0.0, 100.0, 0.0]);
        e.call_log = Some(vec![]);
        let hit = e
            .call(
                0x009a_6e90,
                &args![shooter, projectile, 1000.0f32, from, to, 0u32, 0u32, fraction],
            )
            .bool();
        assert!(hit);
        // Two picks, the second hit half way: (1 + 0.5) / 4.
        assert_eq!(log_of(&mut e, PROJECTILE_PICK).len(), 2);
        assert_eq!(e.mem.f32(fraction), 0.375);
        // The segments start where the previous one ended.
        let ends = log_of(&mut e, PICK_DATA_SET_END);
        assert_eq!(ends.len(), 2);
    }

    // ---- 009a7580 / 009a7670 -----------------------------------------------------------------

    /// A projectile without gravity, speed 1000, one aiming iteration.
    fn straight_projectile(e: &mut Engine) -> u32 {
        let projectile = e.mem.alloc(4);
        returns(e, PROJECTILE_GRAVITY, ret_f(0.0));
        returns(e, PROJECTILE_SPEED, ret_f(1000.0));
        e.mem.set_u32(SETTING_AIMING_NUM_ITERATIONS + 4, 1);
        projectile
    }

    #[test]
    fn check_projectile_arc_aims_at_the_target() {
        let mut e = engine();
        let shooter = flat_reference(&mut e, [0.0, 0.0, 0.0]);
        let target = flat_reference(&mut e, [0.0, 0.0, 0.0]);
        let position = point(&mut e, [0.0, 300.0, 0.0]);
        returns(&mut e, REFR_POSITION_436AA0, ret_u(position));
        returns(&mut e, REFR_GET_SCALE, ret_f(1.0));
        let projectile = straight_projectile(&mut e);
        pick_setup(&mut e, vec![]);
        let high = e.mem.alloc(4);
        let out = e.mem.alloc(12);
        let missing = e
            .call(
                0x009a_7580,
                &args![0u32, target, projectile, 0u32, high, out, 0.5f32],
            )
            .bool();
        assert!(!missing);
        let clear = e
            .call(
                0x009a_7580,
                &args![shooter, target, projectile, 0u32, high, out, 0.5f32],
            )
            .bool();
        assert!(clear);
        assert_eq!(get3(&e, out), [0.0, 300.0, 0.0]);
    }

    #[test]
    fn check_projectile_arc_ov2_threshold_and_arcs() {
        let mut e = engine();
        let shooter = actor(&mut e);
        let target = actor(&mut e);
        let projectile = straight_projectile(&mut e);
        let high = e.mem.alloc(4);
        let out = e.mem.alloc(12);
        let from = arg_point([0.0, 0.0, 0.0]);
        let at = arg_point([0.0, 300.0, 0.0]);
        let still = arg_point([0.0, 0.0, 0.0]);
        // Straight: blocked by something else short of the threshold.
        pick_setup(&mut e, vec![(0x900, 0.25)]);
        let words = args![shooter, target, projectile, from, at, still, high, out, 0.5f32];
        assert!(!e.call(0x009a_7670, &words).bool());
        // At the threshold it counts (-1: the setting).
        set_setting(&mut e, SETTING_GRENADE_THROW_HIT_FRACTION_THRESHOLD, 0.25);
        pick_setup(&mut e, vec![(0x900, 0.25)]);
        let words = args![shooter, target, projectile, from, at, still, high, out, -1.0f32];
        assert!(e.call(0x009a_7670, &words).bool());
        // Arcs: the first is blocked, the toggled one is clear.
        returns(&mut e, PROJECTILE_GRAVITY, ret_f(10.0));
        set_setting(&mut e, SETTING_GRENADE_HIGH_ARC_SPEED_PERCENTAGE, 0.5);
        e.mem
            .set_u32(SETTING_BALLISTIC_PROJECTILE_PATH_PICK_SEGMENTS + 4, 1);
        pick_setup(&mut e, vec![(0x900, 0.25), (0, 0.0)]);
        let words = args![shooter, target, projectile, from, at, still, high, out, 0.5f32];
        assert!(e.call(0x009a_7670, &words).bool());
        assert_eq!(e.mem.u8(high), 1);
        // Both blocked: the arc flag is restored.
        pick_setup(&mut e, vec![(0x900, 0.25), (0x900, 0.25)]);
        assert!(!e.call(0x009a_7670, &words).bool());
        assert_eq!(e.mem.u8(high), 1);
        // A hit beyond the threshold counts for an arc.
        pick_setup(&mut e, vec![(0x900, 0.75)]);
        assert!(e.call(0x009a_7670, &words).bool());
    }

    // ---- 009a7940 / 009a7bc0 -----------------------------------------------------------------

    #[test]
    fn calculate_los_is_clear_without_a_hit_or_on_the_target() {
        let mut e = engine();
        let shooter = flat_reference(&mut e, [0.0, 0.0, 0.0]);
        let target = flat_reference(&mut e, [0.0, 0.0, 0.0]);
        on(&mut e, target, 0xfc, |_, _| ret_u(0));
        returns(&mut e, REFR_GET_SCALE, ret_f(1.0));
        returns(&mut e, COLLECTOR_BASE_CONSTRUCT, Ret::default());
        returns(&mut e, COLLECTOR_INIT, Ret::default());
        pick_setup(&mut e, vec![]);
        let from = point(&mut e, [0.0, 0.0, 0.0]);
        let to = point(&mut e, [0.0, 100.0, 0.0]);
        e.call_log = Some(vec![]);
        let words = args![shooter, target, from, to, 0u32, u32::MAX];
        assert!(e.call(0x009a_7940, &words).bool());
        let layers = log_of(&mut e, FILTER_SET_LAYER);
        assert_eq!(layers[0][1], 0x25);
        returns(&mut e, CELL_PICK, ret_u(0x800));
        // The hit's reference is 0x801.
        assert!(!e.call(0x009a_7940, &words).bool());
        let target_addr = target.addr();
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, _| ret_u(target_addr));
        assert!(e.call(0x009a_7940, &words).bool());
    }

    #[test]
    fn los_collector_constructor_sets_the_vtable() {
        let mut e = engine();
        returns(&mut e, COLLECTOR_BASE_CONSTRUCT, Ret::default());
        returns(&mut e, COLLECTOR_INIT, Ret::default());
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x70, 7);
        e.call_log = Some(vec![]);
        let back = e.call(0x009a_7bc0, &args![this, 5u32, 0x1234u32]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this), LOS_COLLECTOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0x70), 0);
        assert_eq!(log_of(&mut e, COLLECTOR_INIT)[0], vec![this, 5, 0x1234]);
    }

    // ---- 009a7c40 .. 009a7f80 ----------------------------------------------------------------

    #[test]
    fn float_getters_read_their_offsets() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.mem.set_f32(this + 0x6c, 12.5);
        e.mem.set_f32(this + 0x38, 3.25);
        assert_eq!(e.call(0x009a_7c40, &args![this]).f32(), 12.5);
        assert_eq!(e.call(0x009a_9350, &args![this]).f32(), 3.25);
    }

    #[test]
    fn launch_angle_picks_a_root() {
        let mut e = engine();
        // No solution: pi / 4.
        let none = e
            .call(
                0x009a_7c60,
                &args![1000.0f32, 0.0f32, 10.0f32, 10.0f32, false],
            )
            .f32();
        assert_eq!(none, (std::f32::consts::PI as f64 * 0.25) as f32);
        let (d, h, v, g) = (100.0f64, 0.0f64, 100.0f64, 10.0f64);
        let s = (v * v) as f32 as f64;
        let disc = (s * s - (g * d * d + h * 2.0 * s) * g) as f32;
        let root = (disc as f64).sqrt() as f32 as f64;
        let low = ((s - root) / (d * g)) as f32;
        let high = ((s + root) / (d * g)) as f32;
        let words = args![100.0f32, 0.0f32, 100.0f32, 10.0f32, false];
        assert_eq!(
            e.call(0x009a_7c60, &words).f32(),
            (low as f64).atan() as f32
        );
        let words = args![100.0f32, 0.0f32, 100.0f32, 10.0f32, true];
        assert_eq!(
            e.call(0x009a_7c60, &words).f32(),
            (high as f64).atan() as f32
        );
    }

    #[test]
    fn launch_angle_for_an_offset() {
        let mut e = engine();
        let projectile = e.mem.alloc(4);
        let base = e.mem.alloc(4);
        returns(&mut e, PROJECTILE_BASE_4181E0, ret_u(base));
        returns(&mut e, PROJECTILE_GRAVITY, ret_f(10.0));
        returns(&mut e, PROJECTILE_VALUE_9669C0, ret_f(100.0));
        e.call_log = Some(vec![]);
        let offset = arg_point([60.0, 80.0, 5.0]);
        let angle = e
            .call(0x009a_7d50, &args![projectile, offset, 0.0f32, false])
            .f32();
        let expected = fn_009a7c60(&mut e, 100.0, 5.0, 100.0, 10.0, false);
        assert_eq!(angle, expected);
        assert_eq!(log_of(&mut e, PROJECTILE_GRAVITY)[0], vec![base]);
    }

    #[test]
    fn horizontal_time_divides_by_speed() {
        let mut e = engine();
        let time = e.call(0x009a_7dd0, &args![100.0f32, 0.0f32, 50.0f32]).f32();
        assert_eq!(time, 2.0);
    }

    #[test]
    fn projectile_time_with_and_without_gravity() {
        let mut e = engine();
        let projectile = e.mem.alloc(4);
        returns(&mut e, PROJECTILE_GRAVITY, ret_f(0.0));
        returns(&mut e, PROJECTILE_SPEED, ret_f(500.0));
        let from = arg_point([0.0, 100.0, 0.0]);
        let to = arg_point([0.0, 0.0, 0.0]);
        let words = args![projectile, from, to, false, 50.0f32];
        assert_eq!(e.call(0x009a_7e00, &words).f32(), 2.0);
        // Gravity: the low-arc angle and the horizontal time.
        returns(&mut e, PROJECTILE_GRAVITY, ret_f(10.0));
        let words = args![projectile, from, to, false, 0.0f32];
        let time = e.call(0x009a_7e00, &words).f32();
        let angle = fn_009a7c60(&mut e, 100.0, 0.0, 0.0, 10.0, false);
        assert_eq!(time, fn_009a7dd0(&mut e, 100.0, angle, 500.0));
    }

    #[test]
    fn projectile_range_from_speed_and_gravity() {
        let mut e = engine();
        let projectile = e.mem.alloc(0x80);
        e.mem.set_f32(projectile + 0x6c, 900.0);
        returns(&mut e, FLAG_TEST_4FD420, ret_u(1));
        assert_eq!(e.call(0x009a_7f00, &args![projectile, 0.0f32]).f32(), 900.0);
        returns(&mut e, FLAG_TEST_4FD420, ret_u(0));
        returns(&mut e, PROJECTILE_GRAVITY, ret_f(0.0));
        assert_eq!(e.call(0x009a_7f00, &args![projectile, 0.0f32]).f32(), 900.0);
        returns(&mut e, PROJECTILE_GRAVITY, ret_f(10.0));
        e.set_global(GRAVITY_FACTOR, 2.0f32);
        returns(&mut e, PROJECTILE_SPEED, ret_f(100.0));
        assert_eq!(e.call(0x009a_7f00, &args![projectile, 0.0f32]).f32(), 500.0);
        assert_eq!(e.call(0x009a_7f00, &args![projectile, 20.0f32]).f32(), 20.0);
    }

    #[test]
    fn flag_test_asks_for_flag_one() {
        let mut e = engine();
        returns(&mut e, FLAG_TEST_4FD420, ret_u(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x009a_7f80, &args![0x40u32]).bool());
        assert_eq!(log_of(&mut e, FLAG_TEST_4FD420)[0], vec![0x40, 1]);
    }

    // ---- 009a7fa0 .. 009a8660 ----------------------------------------------------------------

    #[test]
    fn attacker_firing_z_offset() {
        let mut e = engine();
        let shooter = actor(&mut e);
        returns(&mut e, ACTOR_GET_HEIGHT, ret_f(100.0));
        returns(&mut e, ACTOR_GET_SNEAK_HEIGHT, ret_f(60.0));
        returns(&mut e, ACTOR_FLAG_4997B0, ret_u(1));
        assert_eq!(e.call(0x009a_7fa0, &args![shooter, false]).f32(), 45.0);
        assert_eq!(e.call(0x009a_7fa0, &args![shooter, true]).f32(), 75.0);
        let thing = reference(&mut e);
        bounds(&mut e, thing, [0.0, 0.0, 30.0], [0.0, 0.0, 10.0]);
        assert_eq!(e.call(0x009a_7fa0, &args![thing, false]).f32(), 10.0);
    }

    #[test]
    fn attacker_firing_location_branches() {
        let mut e = engine();
        let thing = flat_reference(&mut e, [1.0, 2.0, 3.0]);
        let result = e.mem.alloc(12);
        e.call(0x009a_8050, &args![result, thing]);
        assert_eq!(get3(&e, result), [1.0, 2.0, 3.0]);
        e.call(0x009a_8250, &args![result, thing]);
        assert_eq!(get3(&e, result), [1.0, 2.0, 3.0]);
        // An actor with a weapon that is not ranged: the raised position.
        let shooter = actor(&mut e);
        place(&mut e, shooter, [0.0, 0.0, 10.0]);
        returns(&mut e, ACTOR_FLAG_4997B0, ret_u(0));
        returns(&mut e, ACTOR_GET_HEIGHT, ret_f(100.0));
        on(&mut e, shooter, 0x428, |_, _| ret_u(0));
        let weapon = e.mem.alloc(4);
        returns(&mut e, ACTOR_GET_CURRENT_WEAPON, ret_u(weapon));
        returns(&mut e, WEAPON_FLAG_4C0C30, ret_u(0));
        e.call(0x009a_8050, &args![result, shooter]);
        assert_eq!(get3(&e, result), [0.0, 0.0, 85.0]);
        // Ranged, no acquire object: the node of vtable +0x20c.
        returns(&mut e, WEAPON_FLAG_4C0C30, ret_u(1));
        returns(&mut e, ACTOR_8D8520, ret_u(0));
        let node = e.mem.alloc(0x100);
        set3(&mut e, node + 0x8c, [7.0, 8.0, 9.0]);
        e.register(NODE_WORLD_TRANSLATE, |_, a| ret_u(a[0] + 0x8c));
        on(&mut e, shooter, 0x20c, move |_, _| ret_u(node));
        e.call(0x009a_8050, &args![result, shooter]);
        assert_eq!(get3(&e, result), [7.0, 8.0, 9.0]);
        // An acquire object and a type 12 weapon: the biped node.
        let object = object(&mut e);
        returns(&mut e, ACTOR_8D8520, ret_u(object.addr()));
        returns(&mut e, WEAPON_TYPE_446390, ret_u(12));
        on(&mut e, shooter, 0x1ec, |_, _| ret_u(0x4444));
        let other = e.mem.alloc(0x100);
        set3(&mut e, other + 0x8c, [4.0, 5.0, 6.0]);
        on(&mut e, object, 0x190, move |_, a| {
            assert_eq!(a[1], 0x4444);
            ret_u(other)
        });
        e.call(0x009a_8050, &args![result, shooter]);
        assert_eq!(get3(&e, result), [4.0, 5.0, 6.0]);
        // Another type: back to vtable +0x20c.
        returns(&mut e, WEAPON_TYPE_446390, ret_u(3));
        e.call(0x009a_8050, &args![result, shooter]);
        assert_eq!(get3(&e, result), [7.0, 8.0, 9.0]);
    }

    #[test]
    fn back_projected_firing_location() {
        let mut e = engine();
        let shooter = actor(&mut e);
        on(&mut e, shooter, 0x428, |_, _| ret_u(0));
        returns(&mut e, ACTOR_GET_CURRENT_WEAPON, ret_u(0x10));
        returns(&mut e, WEAPON_FLAG_4C0C30, ret_u(0));
        returns(&mut e, ACTOR_FLAG_4997B0, ret_u(0));
        returns(&mut e, ACTOR_GET_HEIGHT, ret_f(100.0));
        let first = point(&mut e, [10.0, 20.0, 0.0]);
        let second = point(&mut e, [0.0, 0.0, 0.0]);
        let mut calls = 0;
        on(&mut e, shooter, 0x1f4, move |_, _| {
            calls += 1;
            ret_u(if calls == 1 { first } else { second })
        });
        on(&mut e, shooter, 0x2bc, |_, _| ret_f(0.0));
        let mut turns = 0;
        e.register_double(SIN_COS, move |e, a| {
            turns += 1;
            let (sine, cosine) = if turns == 1 { (1.0, 0.0) } else { (0.5, 1.0) };
            e.mem.set_f32(a[1], sine);
            e.mem.set_f32(a[2], cosine);
            Ret::default()
        });
        let result = e.mem.alloc(12);
        e.call(0x009a_82b0, &args![result, shooter]);
        assert_eq!(get3(&e, result), [10.0, 0.0, 85.0]);
        // Not an actor: the firing location.
        let thing = flat_reference(&mut e, [1.0, 2.0, 3.0]);
        e.call(0x009a_82b0, &args![result, thing]);
        assert_eq!(get3(&e, result), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn target_firing_z_offset_by_mode() {
        let mut e = engine();
        let target = actor(&mut e);
        returns(&mut e, ACTOR_GET_HEIGHT, ret_f(100.0));
        returns(&mut e, ACTOR_GET_SNEAK_HEIGHT, ret_f(50.0));
        returns(&mut e, ACTOR_FLAG_4997B0, ret_u(0));
        returns(&mut e, ACTOR_FLAG_579670, ret_u(0));
        on(&mut e, target, 0x230, |_, _| ret_u(0));
        on(&mut e, target, 0x22c, |_, _| ret_u(0));
        let expected = [25.0f32, 50.0, 75.0, 50.0, 0.0];
        for (mode, value) in expected.iter().enumerate() {
            let words = args![target, mode as u32, false];
            assert_eq!(e.call(0x009a_8460, &words).f32(), *value, "mode {mode}");
        }
        // Sneaking: mode 2 is 0.9 of the sneak height.
        returns(&mut e, ACTOR_FLAG_4997B0, ret_u(1));
        let sneaking = e.call(0x009a_8460, &args![target, 2u32, false]).f32();
        assert_eq!(sneaking, (50.0f64 * 0.9f32 as f64) as f32);
        let standing = e.call(0x009a_8460, &args![target, 2u32, true]).f32();
        assert_eq!(standing, 75.0);
        // Knocked down (vtable +0x230): a tenth.
        on(&mut e, target, 0x230, |_, _| ret_u(1));
        let down = e.call(0x009a_8460, &args![target, 0u32, true]).f32();
        assert_eq!(down, (100.0f64 * 0.1f32 as f64) as f32);
        // Not an actor: scale times half the span.
        let thing = reference(&mut e);
        bounds(&mut e, thing, [0.0, 0.0, 30.0], [0.0, 0.0, 10.0]);
        returns(&mut e, REFR_GET_SCALE, ret_f(3.0));
        assert_eq!(e.call(0x009a_8460, &args![thing, 1u32, false]).f32(), 30.0);
    }

    #[test]
    fn target_firing_location_raises_the_position() {
        let mut e = engine();
        let target = actor(&mut e);
        let position = point(&mut e, [1.0, 2.0, 3.0]);
        returns(&mut e, REFR_POSITION_436AA0, ret_u(position));
        returns(&mut e, ACTOR_GET_HEIGHT, ret_f(100.0));
        returns(&mut e, ACTOR_FLAG_4997B0, ret_u(0));
        returns(&mut e, ACTOR_FLAG_579670, ret_u(0));
        on(&mut e, target, 0x230, |_, _| ret_u(0));
        on(&mut e, target, 0x22c, |_, _| ret_u(0));
        let result = e.mem.alloc(12);
        let back = e.call(0x009a_8600, &args![result, target, 1u32]).u32();
        assert_eq!(back, result);
        assert_eq!(get3(&e, result), [1.0, 2.0, 53.0]);
    }

    #[test]
    fn target_velocity_from_the_character_controller() {
        let mut e = engine();
        let target = actor(&mut e);
        e.set_global(ZERO_POINT, 0.0f32);
        returns(&mut e, MOBILE_GET_CHAR_CONTROLLER, ret_u(0x900));
        e.register(CHARACTER_PROXY_GET_LINEAR_VELOCITY, |e, a| {
            set3(e, a[1], [1.0, 2.0, 3.0]);
            Ret::default()
        });
        let result = e.mem.alloc(12);
        e.call(0x009a_8660, &args![result, target]);
        assert_eq!(get3(&e, result), [1.0, 2.0, 3.0]);
        returns(&mut e, MOBILE_GET_CHAR_CONTROLLER, ret_u(0));
        e.call(0x009a_8660, &args![result, target]);
        assert_eq!(get3(&e, result), [0.0, 0.0, 0.0]);
        let thing = reference(&mut e);
        returns(&mut e, MOBILE_GET_CHAR_CONTROLLER, ret_u(0x900));
        e.call(0x009a_8660, &args![result, thing]);
        assert_eq!(get3(&e, result), [0.0, 0.0, 0.0]);
    }

    // ---- 009a86e0 .. 009a8af0 ----------------------------------------------------------------

    #[test]
    fn angle_to_projected_target_folds_relative_angles() {
        let mut e = engine();
        let shooter = flat_reference(&mut e, [0.0, 0.0, 0.0]);
        let own = point(&mut e, [4.0, 1.0, -4.0]);
        returns(&mut e, REFR_ANGLE_430830, ret_u(own));
        let result = e.mem.alloc(12);
        let words = args![result, shooter, 0u32, 100.0f32, 0.0f32, 1u32];
        e.call(0x009a_86e0, &words);
        let two_pi = (2.0 * std::f32::consts::PI) as f64;
        assert_eq!(
            get3(&e, result),
            [(-4.0 + two_pi) as f32, 0.0, (4.0 - two_pi) as f32]
        );
    }

    #[test]
    fn angle_to_projected_target_ov2_aims_at_the_projection() {
        let mut e = engine();
        let result = e.mem.alloc(12);
        let from = arg_point([0.0, 0.0, 0.0]);
        set3(&mut e, result, [9.0, 9.0, 9.0]);
        e.call(
            0x009a_87f0,
            &args![result, from, 0u32, 1.0f32, 0.0f32, 1u32],
        );
        assert_eq!(get3(&e, result), [0.0, 0.0, 0.0]);
        let target = flat_reference(&mut e, [0.0, 0.0, 0.0]);
        let position = point(&mut e, [100.0, 0.0, 0.0]);
        returns(&mut e, REFR_POSITION_436AA0, ret_u(position));
        returns(&mut e, REFR_GET_SCALE, ret_f(1.0));
        e.mem.set_u32(SETTING_AIMING_NUM_ITERATIONS + 4, 1);
        e.call(
            0x009a_87f0,
            &args![result, from, target, 1.0f32, 0.0f32, 1u32],
        );
        let value = get3(&e, result);
        assert_eq!(value[2], (std::f64::consts::FRAC_PI_2) as f32);
    }

    #[test]
    fn angle_to_projected_point_ov2_pitch_and_yaw() {
        let mut e = engine();
        let result = e.mem.alloc(12);
        let from = arg_point([0.0, 0.0, 0.0]);
        let to = arg_point([0.0, 10.0, 10.0]);
        e.call(0x009a_8af0, &args![result, from, to]);
        let length = 200.0f64.sqrt() as f32;
        let sine = (10.0f64 / length as f64) as f32;
        let pitch = -((sine as f64).asin() as f32);
        assert_eq!(get3(&e, result), [pitch, 0.0, 0.0]);
        // Yaw folded above pi: -x gives -pi/2.
        let to = arg_point([-10.0, 0.0, 0.0]);
        e.call(0x009a_8af0, &args![result, from, to]);
        assert_eq!(get3(&e, result)[2], (-std::f64::consts::FRAC_PI_2) as f32);
    }

    #[test]
    fn angle_to_projected_point_pushes_near_points_out() {
        let mut e = engine();
        let result = e.mem.alloc(12);
        let expected = e.mem.alloc(12);
        let from = arg_point([5.0, 0.0, 0.0]);
        let origin = arg_point([0.0, 0.0, 0.0]);
        let near = arg_point([0.0, 10.0, 0.0]);
        e.call(0x009a_8970, &args![result, from, origin, 0u32, near]);
        let factor = (1024.0f64 / 10.0) as f32;
        let pushed = arg_point([0.0, (factor as f64 * 10.0) as f32, 0.0]);
        e.call(0x009a_8af0, &args![expected, from, pushed]);
        assert_eq!(get3(&e, result), get3(&e, expected));
        // Same x and y: the point as it is.
        e.call(0x009a_8970, &args![result, origin, origin, 0u32, near]);
        e.call(0x009a_8af0, &args![expected, origin, near]);
        assert_eq!(get3(&e, result), get3(&e, expected));
        // From the attacker (a flat reference): the same as from its position.
        let shooter = flat_reference(&mut e, [0.0, 0.0, 0.0]);
        e.call(0x009a_88e0, &args![result, shooter, near]);
        assert_eq!(get3(&e, result), get3(&e, expected));
    }

    // ---- 009a8bf0 .. 009a8f00 ----------------------------------------------------------------

    #[test]
    fn projected_point_mode_selection() {
        let mut e = engine();
        let shooter = actor(&mut e);
        let target = actor(&mut e);
        let position = point(&mut e, [0.0, 0.0, 0.0]);
        returns(&mut e, REFR_POSITION_436AA0, ret_u(position));
        returns(&mut e, ACTOR_GET_HEIGHT, ret_f(100.0));
        returns(&mut e, ACTOR_FLAG_4997B0, ret_u(0));
        returns(&mut e, ACTOR_FLAG_579670, ret_u(0));
        on(&mut e, target, 0x230, |_, _| ret_u(0));
        on(&mut e, target, 0x22c, |_, _| ret_u(0));
        returns(&mut e, MOBILE_GET_CHAR_CONTROLLER, ret_u(0));
        let controller = e.mem.alloc(4);
        on(&mut e, shooter, 0x428, move |_, _| ret_u(controller));
        returns(&mut e, CONTROLLER_GET_TARGET_SEGMENT_IN_VIEW, ret_u(1));
        // The shooter fires from its raised position (no ranged weapon).
        returns(&mut e, CONTROLLER_WEAPON, ret_u(0x10));
        returns(&mut e, WEAPON_FLAG_4C0C30, ret_u(0));
        place(&mut e, shooter, [0.0, -300.0, 0.0]);
        let projectile = e.mem.alloc(4);
        // The projected point is the target location (flag test holds).
        returns(&mut e, FLAG_TEST_4FD420, ret_u(1));
        returns(&mut e, PROJECTILE_EXPLOSION, ret_u(0));
        let result = e.mem.alloc(12);
        let words = args![result, shooter, target, 0u32, projectile, 0.0f32, false];
        e.call(0x009a_8bf0, &words);
        assert_eq!(get3(&e, result)[2], 50.0);
        // A high arc with gravity: mode 2.
        returns(&mut e, PROJECTILE_GRAVITY, ret_f(10.0));
        let words = args![result, shooter, target, 0u32, projectile, 0.0f32, true];
        e.call(0x009a_8bf0, &words);
        assert_eq!(get3(&e, result)[2], 75.0);
        // A slow splash projectile: mode 4 (no offset).
        returns(&mut e, FLAG_TEST_4FD420, ret_u(0));
        returns(&mut e, PROJECTILE_GRAVITY, ret_f(0.0));
        let explosion = e.mem.alloc(4);
        returns(&mut e, PROJECTILE_EXPLOSION, ret_u(explosion));
        returns(&mut e, EXPLOSION_GET_RADIUS, ret_f(200.0));
        returns(&mut e, EXPLOSION_DAMAGE, ret_f(50.0));
        returns(&mut e, PROJECTILE_SPEED, ret_f(100.0));
        set_setting(&mut e, SETTING_COMBAT_SPLASH_DAMAGE_MAX_SPEED, 500.0);
        set_setting(&mut e, SETTING_COMBAT_SPLASH_DAMAGE_MIN_RADIUS, 100.0);
        set_setting(&mut e, SETTING_COMBAT_SPLASH_DAMAGE_MIN_DAMAGE, 10.0);
        e.mem.set_u32(SETTING_AIMING_NUM_ITERATIONS + 4, 1);
        let words = args![result, shooter, target, 0u32, projectile, 0.0f32, false];
        e.call(0x009a_8bf0, &words);
        assert_eq!(get3(&e, result)[2], 0.0);
        // Too fast for splash: the controller's segment again.
        returns(&mut e, PROJECTILE_SPEED, ret_f(1000.0));
        e.call(0x009a_8bf0, &words);
        assert_eq!(get3(&e, result)[2], 50.0);
    }

    #[test]
    fn projected_point_ov2_and_ov3() {
        let mut e = engine();
        let shooter = flat_reference(&mut e, [0.0, 0.0, 0.0]);
        let projectile = e.mem.alloc(4);
        returns(&mut e, FLAG_TEST_4FD420, ret_u(1));
        let at = point(&mut e, [1.0, 2.0, 3.0]);
        let result = e.mem.alloc(12);
        let words = args![result, shooter, at, 0u32, projectile, 0.0f32, false];
        e.call(0x009a_8d80, &words);
        assert_eq!(get3(&e, result), [1.0, 2.0, 3.0]);
        // Not held: projected from the given start for a moving target.
        returns(&mut e, FLAG_TEST_4FD420, ret_u(0));
        returns(&mut e, PROJECTILE_GRAVITY, ret_f(0.0));
        returns(&mut e, PROJECTILE_SPEED, ret_f(100.0));
        e.mem.set_u32(SETTING_AIMING_NUM_ITERATIONS + 4, 1);
        let start = point(&mut e, [0.0, 0.0, 0.0]);
        let location = arg_point([0.0, 100.0, 0.0]);
        let velocity = arg_point([10.0, 0.0, 0.0]);
        let words = args![result, shooter, location, velocity, start, projectile, 0.0f32, false];
        e.call(0x009a_8df0, &words);
        assert_eq!(get3(&e, result), [10.0, 100.0, 0.0]);
    }

    #[test]
    fn projected_point_ov4_straight_and_ballistic() {
        let mut e = engine();
        e.mem.set_u32(SETTING_AIMING_NUM_ITERATIONS + 4, 3);
        let result = e.mem.alloc(12);
        let from = arg_point([0.0, 0.0, 0.0]);
        let target = arg_point([0.0, 100.0, 0.0]);
        let velocity = arg_point([10.0, 0.0, 0.0]);
        let words = args![result, from, target, velocity, 100.0f32, 0.0f32, false];
        e.call(0x009a_8f00, &words);
        // Three refinements: t = 1, then |(10,100)| / 100, then again.
        let mut guess = [0.0f32, 100.0, 0.0];
        for _ in 0..3 {
            let distance = ((guess[0] as f64).powi(2) + (guess[1] as f64).powi(2)).sqrt() as f32;
            let time = (distance as f64 / 100.0) as f32;
            guess = [(time as f64 * 10.0) as f32, 100.0, 0.0];
        }
        assert_eq!(get3(&e, result), guess);
        // Too close: the target as it is.
        let target_near = arg_point([0.0, 0.5, 0.0]);
        let words = args![result, from, target_near, velocity, 100.0f32, 0.0f32, false];
        e.call(0x009a_8f00, &words);
        assert_eq!(get3(&e, result), [0.0, 0.5, 0.0]);
        // Ballistic, still target: z from the launch angle.
        let still = arg_point([0.0, 0.0, 0.0]);
        let ahead = arg_point([100.0, 0.0, 0.0]);
        let words = args![result, from, ahead, still, 1000.0f32, 10.0f32, false];
        e.call(0x009a_8f00, &words);
        let angle = fn_009a7c60(&mut e, 100.0, 0.0, 1000.0, 10.0, false);
        let z = ((angle as f64).tan() as f32 as f64 * 100.0) as f32;
        assert_eq!(get3(&e, result), [100.0, 0.0, z]);
    }

    // ---- 009a9180 .. 009a95f0 ----------------------------------------------------------------

    #[test]
    fn optimal_weapon_ranges() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        e.mem.set_f32(object + 0x38, 0.5);
        returns(&mut e, CONTROLLER_980070, ret_u(object));
        returns(&mut e, VALUE_508050, ret_f(2.0));
        set_setting(&mut e, SETTING_COMBAT_ABSOLUTE_MAX_RANGE_MULT, 3.0);
        let ranges = e.mem.alloc(12);
        let words = args![0x10u32, 0u32, ranges, ranges + 4, ranges + 8];
        e.call(0x009a_9180, &words);
        assert_eq!(get3(&e, ranges), [256.0, 2048.0, 6144.0]);
        // A weapon with its own ranges, a projectile with a range of 1500
        // and an explosion of radius 100.
        let weapon = e.mem.alloc(4);
        returns(&mut e, WEAPON_VALUE_4A0DB0, ret_f(50.0));
        returns(&mut e, WEAPON_VALUE_646F50, ret_f(2000.0));
        returns(&mut e, WEAPON_FLAG_647790, ret_u(1));
        returns(&mut e, CONTROLLER_ACTOR, ret_u(0x20));
        let projectile = e.mem.alloc(0x80);
        e.mem.set_f32(projectile + 0x6c, 1500.0);
        returns(&mut e, WEAPON_PROJECTILE, ret_u(projectile));
        returns(&mut e, FLAG_TEST_4FD420, ret_u(1));
        set_setting(
            &mut e,
            SETTING_COMBAT_PROJECTILE_MAX_RANGE_OPTIMAL_MULT,
            0.5,
        );
        let explosion = e.mem.alloc(4);
        returns(&mut e, PROJECTILE_EXPLOSION, ret_u(explosion));
        returns(&mut e, EXPLOSION_GET_RADIUS, ret_f(100.0));
        let words = args![0x10u32, weapon, ranges, ranges + 4, ranges + 8];
        e.call(0x009a_9180, &words);
        assert_eq!(get3(&e, ranges), [100.0, 750.0, 1500.0]);
        // The minimum stays below the optimal less 256.
        returns(&mut e, EXPLOSION_GET_RADIUS, ret_f(700.0));
        e.call(0x009a_9180, &words);
        assert_eq!(get3(&e, ranges), [494.0, 750.0, 1500.0]);
    }

    /// A cover edge: direction (1, 0, 0) at +0x40, points (0, 0, 0) at
    /// +0x28 and (100, 0, 0) at +0x34; the info gives a width of 6.
    fn cover_edge(e: &mut Engine) -> (u32, u32) {
        let edge = e.mem.alloc(0x50);
        set3(e, edge + 0x28, [0.0, 0.0, 0.0]);
        set3(e, edge + 0x34, [100.0, 0.0, 0.0]);
        set3(e, edge + 0x40, [1.0, 0.0, 0.0]);
        let info = e.mem.alloc(8);
        e.mem.set_u8(info, 2);
        e.mem.set_u8(info + 2, 4);
        e.mem.set_u8(info + 3, 1);
        (edge, info)
    }

    #[test]
    fn cover_positions() {
        let mut e = engine();
        let (edge, info) = cover_edge(&mut e);
        returns(&mut e, EDGE_FLAG_6E4820, ret_u(0));
        returns(&mut e, EDGE_FLAG_982BE0, ret_u(0));
        let left = e.mem.alloc(12);
        let right = e.mem.alloc(12);
        let words = args![4.0f32, edge, info, left, right];
        e.call(0x009a_93e0, &words);
        assert_eq!(get3(&e, left), [-20.0, -6.0, 0.0]);
        assert_eq!(get3(&e, right), [-20.0, -6.0, 0.0]);
        e.mem.set_u8(info + 4, 1);
        e.mem.set_u8(info + 5, 1);
        e.call(0x009a_93e0, &words);
        assert_eq!(get3(&e, right), [-20.0, 64.0, 0.0]);
        e.mem.set_u8(info + 5, 0);
        e.call(0x009a_93e0, &words);
        assert_eq!(get3(&e, right), [80.0, -64.0, 0.0]);
        returns(&mut e, EDGE_FLAG_6E4820, ret_u(1));
        e.call(0x009a_93e0, &words);
        assert_eq!(get3(&e, right), [-20.0, -6.0, 0.0]);
        assert_eq!(get3(&e, left), [-148.0, -6.0, 0.0]);
        // Nothing to write: no calls.
        e.call_log = Some(vec![]);
        e.call(0x009a_93e0, &args![4.0f32, edge, info, 0u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn cover_positions_from_the_controller() {
        let mut e = engine();
        let cover = e.mem.alloc(0xc0);
        let edge = cover + 0x14;
        set3(&mut e, edge + 0x40, [1.0, 0.0, 0.0]);
        let info = cover + 0x70;
        e.mem.set_u8(info, 2);
        e.mem.set_u8(info + 2, 4);
        e.mem.set_u8(info + 3, 1);
        returns(&mut e, CONTROLLER_ACTOR, ret_u(0x30));
        returns(&mut e, ACTOR_GET_RADIUS, ret_f(4.0));
        returns(&mut e, EDGE_FLAG_6E4820, ret_u(0));
        let left = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        e.call(0x009a_9370, &args![0x10u32, cover, left, 0u32]);
        assert_eq!(get3(&e, left), [-20.0, -6.0, 0.0]);
        assert_eq!(log_of(&mut e, ACTOR_GET_RADIUS)[0], vec![0x30]);
        e.call(0x009a_93a0, &args![0x10u32, edge, info, left, 0u32]);
        assert_eq!(get3(&e, left), [-20.0, -6.0, 0.0]);
    }

    #[test]
    fn cover_width_from_bytes() {
        let mut e = engine();
        let info = e.mem.alloc(4);
        e.mem.set_u8(info, 3);
        e.mem.set_u8(info + 2, 5);
        e.mem.set_u8(info + 3, 2);
        assert_eq!(e.call(0x009a_95f0, &args![info]).f32(), 13.5);
    }

    // ---- 009a9630 .. 009a9720 ----------------------------------------------------------------

    #[test]
    fn melee_and_ranged_checks() {
        let mut e = engine();
        let fighter = actor(&mut e);
        on(&mut e, fighter, 0x428, |_, _| ret_u(0));
        returns(&mut e, ACTOR_GET_CURRENT_WEAPON, ret_u(0));
        assert!(e.call(0x009a_9630, &args![fighter]).bool());
        assert!(!e.call(0x009a_96c0, &args![fighter]).bool());
        returns(&mut e, ACTOR_GET_CURRENT_WEAPON, ret_u(0x50));
        returns(&mut e, WEAPON_FLAG_6450C0, ret_u(0));
        returns(&mut e, WEAPON_FLAG_4C0C30, ret_u(1));
        assert!(!e.call(0x009a_9630, &args![fighter]).bool());
        assert!(e.call(0x009a_96c0, &args![fighter]).bool());
        let controller = e.mem.alloc(4);
        on(&mut e, fighter, 0x428, move |_, _| ret_u(controller));
        returns(&mut e, CONTROLLER_WEAPON, ret_u(0x50));
        e.register(CONTAINS_ACTION, |_, a| ret_u(u32::from(a[1] == 8)));
        assert!(e.call(0x009a_9630, &args![fighter]).bool());
        e.register(CONTAINS_ACTION, |_, _| ret_u(0));
        assert!(!e.call(0x009a_9630, &args![fighter]).bool());
        returns(&mut e, WEAPON_FLAG_6450C0, ret_u(1));
        assert!(e.call(0x009a_9630, &args![fighter]).bool());
    }

    #[test]
    fn can_exist_in_space() {
        let mut e = engine();
        returns(&mut e, REFR_GET_REF_PERSISTS, ret_u(0));
        returns(&mut e, WORLD_LOCATION_IS_LOADED, ret_u(0));
        assert!(!e.call(0x009a_96f0, &args![1u32, 2u32]).bool());
        returns(&mut e, WORLD_LOCATION_IS_LOADED, ret_u(1));
        assert!(e.call(0x009a_96f0, &args![1u32, 2u32]).bool());
        returns(&mut e, WORLD_LOCATION_IS_LOADED, ret_u(0));
        returns(&mut e, REFR_GET_REF_PERSISTS, ret_u(1));
        assert!(e.call(0x009a_96f0, &args![1u32, 2u32]).bool());
    }

    #[test]
    fn dangerous_explosion_thresholds() {
        let mut e = engine();
        assert!(!e.call(0x009a_9720, &args![0u32]).bool());
        let explosion = e.mem.alloc(4);
        set_setting(&mut e, SETTING_DANGEROUS_PROJECTILE_EXPLOSION_DAMAGE, 20.0);
        set_setting(&mut e, SETTING_DANGEROUS_PROJECTILE_EXPLOSION_RADIUS, 100.0);
        returns(&mut e, EXPLOSION_DAMAGE, ret_f(10.0));
        returns(&mut e, EXPLOSION_GET_RADIUS, ret_f(300.0));
        e.register(EXPLOSION_FORMULA_6479A0, |_, a| ret_f(arg_f(a, 0) / 2.0));
        assert!(!e.call(0x009a_9720, &args![explosion]).bool());
        returns(&mut e, EXPLOSION_DAMAGE, ret_f(20.0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x009a_9720, &args![explosion]).bool());
        let formula = log_of(&mut e, EXPLOSION_FORMULA_6479A0);
        assert_eq!(
            formula[0],
            vec![300.0f32.to_bits(), 20.0f32.to_bits(), 20.0f32.to_bits()]
        );
        returns(&mut e, EXPLOSION_GET_RADIUS, ret_f(150.0));
        assert!(!e.call(0x009a_9720, &args![explosion]).bool());
    }

    // ---- sectors ------------------------------------------------------------------------------

    #[test]
    fn sector_wraps() {
        let mut e = engine();
        assert_eq!(e.call(0x009a_97b0, &args![5i8, 3i8]).i32() as i8, 8);
        assert_eq!(e.call(0x009a_97b0, &args![1i8, -3i8]).i32() as i8, 28);
        assert_eq!(e.call(0x009a_97b0, &args![28i8, 3i8]).i32() as i8, 1);
        // Only the low bytes count (the caller pushes `-i` as a word).
        assert_eq!(
            e.call(0x009a_97b0, &args![0x105u32, 0xffff_fffeu32]).i32() as i8,
            3
        );
    }

    #[test]
    fn sector_masks() {
        let mut e = engine();
        let mask = e.call(0x009a_9940, &args![0u32, 0i8]).u32();
        assert_eq!(mask, 0b111 | (1 << 29) | (1 << 28));
        assert!(e.call(0x009a_9800, &args![mask, 2i8]).bool());
        assert!(e.call(0x009a_9800, &args![mask, 4i8]).bool());
        assert!(!e.call(0x009a_9800, &args![mask, 5i8]).bool());
        assert!(e.call(0x009a_9800, &args![mask, 26i8]).bool());
        assert!(!e.call(0x009a_9800, &args![mask, 25i8]).bool());
        // Nearest free sector: 0 is taken, 5 above, 25 below; 5 first.
        assert_eq!(e.call(0x009a_98a0, &args![mask, 0i8]).i32() as i8, 5);
        assert_eq!(e.call(0x009a_98a0, &args![0u32, 7i8]).i32() as i8, 7);
        assert_eq!(e.call(0x009a_98a0, &args![u32::MAX, 7i8]).i32() as i8, -1);
    }

    #[test]
    fn sector_angles() {
        let mut e = engine();
        assert_eq!(e.call(0x009a_99d0, &args![95.0f32]).i32(), 7);
        assert_eq!(e.call(0x009a_99d0, &args![-13.0f32]).i32(), -1);
        assert_eq!(e.call(0x009a_99f0, &args![7i8]).f32(), 84.0);
        assert_eq!(e.call(0x009a_99f0, &args![-1i8]).f32(), -12.0);
    }

    /// A target with two melee attackers (the first flagged by its
    /// `004fd3e0` object, the second the returned `actor`); both
    /// controllers record their angle at +0xe0.
    fn attackers_setup(e: &mut Engine) -> (Ptr, Ptr, [u32; 2]) {
        let target = actor(e);
        let target_position = point(e, [0.0, 0.0, 0.0]);
        let first = actor(e);
        let second = actor(e);
        let first_position = point(e, [0.0, 100.0, 0.0]);
        let second_position = point(e, [0.0, 50.0, 0.0]);
        let (t, f, s) = (target.addr(), first.addr(), second.addr());
        e.register_double(REFR_POSITION_436AA0, move |_, a| {
            ret_u(if a[0] == t {
                target_position
            } else if a[0] == f {
                first_position
            } else {
                assert_eq!(a[0], s);
                second_position
            })
        });
        let first_controller = e.mem.alloc(0x100);
        let second_controller = e.mem.alloc(0x100);
        on(e, first, 0x428, move |_, _| ret_u(first_controller));
        on(e, second, 0x428, move |_, _| ret_u(second_controller));
        e.register(CONTAINS_ACTION, |_, a| ret_u(u32::from(a[1] == 7)));
        let flagged = object(e);
        on(e, flagged, 0x34, |_, _| ret_u(1));
        let flagged_addr = flagged.addr();
        e.register_double(CONTROLLER_4FD3E0, move |_, a| {
            ret_u(if a[0] == first_controller {
                flagged_addr
            } else {
                0
            })
        });
        let slots = e.mem.alloc(8);
        e.mem.set_u32(slots, f);
        e.mem.set_u32(slots + 4, s);
        returns(e, TARGET_MAP_441110, ret_u(0x60));
        let array = e.mem.alloc(4);
        returns(e, GET_TARGET_ATTACKER_ARRAY, ret_u(array));
        returns(e, ARRAY_COUNT, ret_u(2));
        e.register(WORD_MIN, |_, a| ret_u(a[0].min(a[1])));
        e.register_double(ARRAY_ELEMENT, move |_, a| ret_u(slots + 4 * a[1]));
        returns(e, CONSTRUCT_ARRAY, Ret::default());
        e.register(MEMORY_FILL, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            ret_u(a[0])
        });
        (target, second, [first_controller, second_controller])
    }

    #[test]
    fn attackers_get_separate_sectors() {
        let mut e = engine();
        let (target, second, controllers) = attackers_setup(&mut e);
        assert!(!e.call(0x009a_9a20, &args![0u32, second]).bool());
        assert!(e.call(0x009a_9a20, &args![target, second]).bool());
        // Both stand north of the target (angle 0, sector 0): the flagged
        // first keeps sector 0, the second moves to sector 5 (60 degrees).
        assert_eq!(e.mem.f32(controllers[0] + 0xe0), 0.0);
        assert_eq!(e.mem.f32(controllers[1] + 0xe0), 60.0);
    }

    // ---- 009a9f00 / 009a9f20 -----------------------------------------------------------------

    #[test]
    fn store_angle_quiets_signalling_nan() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.call(0x009a_9f00, &args![this, 12.5f32]);
        assert_eq!(e.mem.f32(this + 0xe0), 12.5);
        e.call(0x009a_9f00, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0xe0), 0x7fc0_0001);
    }

    #[test]
    fn los_debug_geometry() {
        let mut e = engine();
        e.register(COLOR_SET, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret_u(a[0])
        });
        let mut lines = 0;
        e.register_double(MAKE_LINE, move |_, _| {
            lines += 1;
            ret_u(0x100 + lines)
        });
        returns(&mut e, MAKE_TRI_POINT, ret_u(0x200));
        returns(&mut e, UPDATE_PROPERTIES, Ret::default());
        returns(&mut e, NODE_SET_TRANSLATE, Ret::default());
        returns(&mut e, TES_ADD_TEMP_DEBUG_OBJECT, Ret::default());
        e.set_global(TES_OBJECT, 0x77u32);
        set_setting(&mut e, SETTING_DEBUG_COMBAT_PROJECTILE_LOS_TIME, 2.0);
        let from = point(&mut e, [0.0, 0.0, 0.0]);
        let to = point(&mut e, [0.0, 100.0, 0.0]);
        e.call_log = Some(vec![]);
        e.call(0x009a_9f20, &args![from, to, 0u32, 0.0f32, 0u32, 0u32]);
        let added = log_of(&mut e, TES_ADD_TEMP_DEBUG_OBJECT);
        assert_eq!(added, vec![vec![0x77, 0x101, 2.0f32.to_bits()]]);
        e.call_log = Some(vec![]);
        e.call(0x009a_9f20, &args![from, to, 0x5u32, 0.25f32, 0u32, 0u32]);
        let added: Vec<u32> = log_of(&mut e, TES_ADD_TEMP_DEBUG_OBJECT)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(added, vec![0x102, 0x200, 0x103]);
        let marker = log_of(&mut e, MAKE_TRI_POINT);
        assert_eq!(marker[0][0], 5.0f32.to_bits());
        let line = log_of(&mut e, MAKE_LINE);
        // The first line ends a quarter of the way, in yellow; the second
        // goes on from there to `to`.
        let end = line[0][2];
        assert_eq!(get3(&e, end), [0.0, 25.0, 0.0]);
        assert_eq!(e.mem.f32(line[0][1] + 4), 1.0);
        assert_eq!(line[1][0], end);
        assert_eq!(line[1][2], to);
    }
}

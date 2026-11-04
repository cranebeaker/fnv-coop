//! `fallout/ai/run_009b36f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `009c52f0` (named `Projectile::LoadGame` in the engine map): the base body the wrapper
/// forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LOAD_GAME_TARGET_009C52F0: u32 = 0x009c_52f0;

// Translated from 009b4cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable words 0108c420, 0108eac0 and 0108f6d0): forwards `this`
/// and one pointer to `009c52f0` (named `Projectile::LoadGame`; Xbox PDB
/// `?LoadGame@Projectile@@UAAXPAVBGSLoadFormBuffer@@@Z`, void). Returns nothing.
pub fn fn_009b4cf0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(LOAD_GAME_TARGET_009C52F0, &args![this, buffer]);
}

/// `009bc470` (`ECX` = projectile; in `fallout/ai/combat/projectile.cpp` by
/// the engine map).
const PROJECTILE_009BC470: u32 = 0x009b_c470;
/// `Projectile::Projectile` (engine map name of `009bbc70`; `ECX` = object,
/// one byte word; `RET 4`).
const PROJECTILE_CONSTRUCT: u32 = 0x009b_bc70;
/// `004f15a0` (`ECX` = form, the form type; `RET 4`).
const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// `GrenadeProjectile`'s two vtables (RTTI names read through their
/// locators): the main one at +0 and the one at +0x18.
const GRENADE_VTABLE: u32 = 0x0108_f674;
const GRENADE_VTABLE_18: u32 = 0x0108_f668;
/// `00978f10` (`ECX` = object): a byte.
const OBJECT_00978F10: u32 = 0x0097_8f10;

// Translated from 009b3f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `009bc470` on `this`. The engine map gives it no name.
pub fn fn_009b3f80(e: &mut Engine, this: Ptr) {
    e.call(PROJECTILE_009BC470, &args![this]);
}

// Translated from 009b3fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b3fa0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs a `GrenadeProjectile` (the vtables it stores are that class's
/// by RTTI; one byte word, `RET 4`): `Projectile::Projectile` with the
/// byte, the two vtables (+0, +0x18), the byte at +0x150 cleared and the
/// form type 0x3e (`004f15a0`). Returns `this`. The engine map gives it no
/// name.
pub fn fn_009b3fa0(e: &mut Engine, this: Ptr, flag: u8) -> Ptr {
    let base = this.addr();
    e.call(PROJECTILE_CONSTRUCT, &args![this, u32::from(flag)]);
    e.mem.set_u32(base, GRENADE_VTABLE);
    e.mem.set_u32(base + 0x18, GRENADE_VTABLE_18);
    e.mem.set_u8(base + 0x150, 0);
    e.call(SET_FORM_TYPE, &args![this, 0x3eu32]);
    this
}

// Translated from 009b4d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the object's virtual slot `0x318` or else `00978f10` reports
/// true (each a byte). The engine map gives it no name.
pub fn fn_009b4d40(e: &mut Engine, this: Ptr) -> bool {
    if e.vcall(this.addr(), 0x318, &[]).bool() {
        return true;
    }
    e.call(OBJECT_00978F10, &args![this]).bool()
}

/// `00559450` (unnamed): `thiscall`, returns the word at `+0`.
const READ_FIRST_WORD: u32 = 0x0055_9450;
/// `004181e0` (unnamed, `extradatalist.cpp` per the engine map): `thiscall`; a word (the
/// projectile's shooter, as `009c2c80` and the other callers here use it).
const SHOOTER: u32 = 0x0041_81e0;
/// `009c2c80` (unnamed, `projectile.cpp`): `cdecl`, four words; a `bool`.
const IMPACT_ALLOWED: u32 = 0x009c_2c80;
/// `008256d0` (unnamed): `thiscall`; whether the list node is empty.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (unnamed): `thiscall`, returns `this` (a list node's item slot; also the
/// no-op constructor the code runs on point and matrix locals).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (unnamed): `thiscall`; the next list node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `Projectile::AddImpact` (`009bf5a0`, Xbox PDB): `thiscall`, six words (`RET 0x18`).
const ADD_IMPACT: u32 = 0x009b_f5a0;
/// `0043b300` (unnamed, `modelloader.cpp`): `cdecl` (an `NiRTTI`, an object); whether the
/// object is of that class.
const IS_KIND_OF: u32 = 0x0043_b300;
/// The `NiRTTI` objects of `BSStripParticleSystem` (`0x01202560`) and `NiPSysEmitter`
/// (`0x01202540`), by the names their static initializers give them.
const RTTI_BS_STRIP_PARTICLE_SYSTEM: u32 = 0x0120_2560;
const RTTI_NI_PSYS_EMITTER: u32 = 0x0120_2540;
/// `004d7d50` / `004d7d70` (unnamed): `thiscall`; the particle system's modifier count,
/// and modifier `index` (`RET 4`).
const MODIFIER_COUNT: u32 = 0x004d_7d50;
const MODIFIER_AT: u32 = 0x004d_7d70;
/// `00526ac0` (named `BSMultiBoundCapsule::QMultiBoundRadius` in the engine map; a
/// folded getter): `thiscall`; a float in `ST0`.
const EMITTER_SIZE: u32 = 0x0052_6ac0;
/// `0043b480` / `0043b4a0` (unnamed, `modelloader.cpp`): `thiscall`; a node's child count
/// and child `index` (`RET 4`).
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
/// `0.0` and `1.0` (`double`, `0x01012060` and `0x01012070`).
const ZERO: u32 = 0x0101_2060;
const ONE: u32 = 0x0101_2070;
/// `Projectile::SaveGame` (`009c4ff0`) and `Projectile::LoadGame` (`009c52f0`), Xbox PDB:
/// `thiscall`, one word (`RET 4`).
const PROJECTILE_SAVE_GAME: u32 = 0x009c_4ff0;
/// `00865e50` / `00864980` (unnamed, `bgssavegamebuffer.cpp` / `bgsloadgamebuffer.cpp`):
/// `thiscall` on the buffer: save (data, size, 0; `RET 0xc`) and load (data, size;
/// `RET 8`).
const SAVE_DATA: u32 = 0x0086_5e50;
const LOAD_DATA: u32 = 0x0086_4980;
/// `009c58e0` (unnamed, `projectile.cpp`): `thiscall`, one word (`RET 4`).
const PROJECTILE_INIT_LOAD: u32 = 0x009c_58e0;
/// `009bbef0` (unnamed, `projectile.cpp`): the other `Projectile` constructor
/// (`thiscall`, eight words, `RET 0x20`).
const PROJECTILE_CONSTRUCT_WITH: u32 = 0x009b_bef0;
/// `005de0a0` (unnamed, `tesscriptfunctions.cpp` per the engine map): `thiscall`, two
/// words (`RET 8`): a flag mask and whether to set it.
const SET_FLAG: u32 = 0x005d_e0a0;
/// `00444d00` (unnamed, `modelloader.cpp`): `thiscall`, one word (`RET 4`); a flag test.
const HAS_FLAG: u32 = 0x0044_4d00;
/// `004fd360`, `00975300`, `005de080` (unnamed): `thiscall` on the shooter; `bool`s.
const SHOOTER_TEST_004FD360: u32 = 0x004f_d360;
const SHOOTER_TEST_00975300: u32 = 0x0097_5300;
const SHOOTER_TEST_005DE080: u32 = 0x005d_e080;
/// `009bda10` (unnamed, `projectile.cpp`): `thiscall`, no stack words.
const PROJECTILE_INITIALIZE: u32 = 0x009b_da10;
/// `00440d80` (unnamed, `modelloader.cpp`): `thiscall`; a `bool`.
const IS_DISABLED: u32 = 0x0044_0d80;
/// `00885bd0` (unnamed, `actor.cpp`): `thiscall`; a float in `ST0` (the projectile's age,
/// as the comparisons here use it).
const AGE: u32 = 0x0088_5bd0;
/// `005dbe10` (unnamed): no words; the next key of a counter at `0x011b0808`.
const NEXT_KEY: u32 = 0x005d_be10;
/// `00c6a3d0` (unnamed, `bhkworld.obj`): `cdecl`, four words.
const SET_COLLISION_KEY: u32 = 0x00c6_a3d0;
/// `TES::GetWorldSpace` (`004fd3e0`, engine map; a folded getter): `thiscall`; a word.
const SHOOTER_WORLD: u32 = 0x004f_d3e0;
/// `004e75d0` (unnamed, `teswater.cpp`): `thiscall`, one word (`RET 4`); returns a pointer.
const WATER_AT: u32 = 0x004e_75d0;
/// `005e39b0` (unnamed): `cdecl`, one word.
const WATER_TEST: u32 = 0x005e_39b0;
/// `00511840` (unnamed): `thiscall`; a sound name.
const SOUND_NAME: u32 = 0x0051_1840;
/// `BSAudio::QInstance` (`00453a70`), `BSAudio::GetSoundHandleByFilename` (`00ad7480`,
/// `RET 0x10`), `BSSoundHandle::Play` (`00ad8830`, `RET 4`), Xbox PDB.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE: u32 = 0x00ad_7480;
const SOUND_PLAY: u32 = 0x00ad_8830;
/// `0068a7d0` (unnamed): `thiscall`, one word (`RET 4`); sets the handle's position.
const SOUND_SET_POSITION: u32 = 0x0068_a7d0;
/// `00483710` (unnamed, `tesform.cpp`): `thiscall`; the sound handle's destructor.
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// `0045cd80` (unnamed): `thiscall`; a float in `ST0`.
const SHOOTER_FLOAT: u32 = 0x0045_cd80;
/// `00403e20` (unnamed, `effectitem.cpp`): `thiscall` on a `Setting`; the address of its
/// float.
const SETTING_FLOAT: u32 = 0x0040_3e20;
/// The `Setting`s `fn_009b41b0` reads (`0x011cee0c` and `0x011cf534`).
const SETTING_011CEE0C: u32 = 0x011c_ee0c;
const SETTING_011CF534: u32 = 0x011c_f534;
/// `0084e3a0` (unnamed): `thiscall`; a word.
const PROJECTILE_FORM: u32 = 0x0084_e3a0;
/// The object (`0x011c3f2c`) whose `00469860` (`thiscall`, `RET 4`) is asked.
const DATA_HANDLER: u32 = 0x011c_3f2c;
const DATA_HANDLER_TEST: u32 = 0x0046_9860;
/// `009bc8f0`, `009bf190` (unnamed, `projectile.cpp`): `thiscall`, no stack words.
const PROJECTILE_EXPIRE: u32 = 0x009b_c8f0;
const PROJECTILE_DETACH: u32 = 0x009b_f190;
/// `TESObjectREFR::SetLocationOnReference` (`00575830`, Xbox PDB): `thiscall`, one word
/// (`RET 4`).
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
/// `Projectile::CheckExplosion` (`009c3190`, Xbox PDB): `thiscall`, one float (`RET 4`).
const CHECK_EXPLOSION: u32 = 0x009c_3190;
/// The player character pointer (`0x011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// The array (`0x011f2250`) whose count (`0044ddc0`) is compared with 4.
const COUNTED_ARRAY: u32 = 0x011f_2250;
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// `bhkCollisionObject::GetbhkCollisionObject` (`0043b610`, Xbox PDB): `cdecl`, one word.
const GET_COLLISION_OBJECT: u32 = 0x0043_b610;
/// `006fa820`, `00517630`, `00560d80` (unnamed): `thiscall`; the rigid body, a word, and
/// the address of its 16-byte vector.
const RIGID_BODY: u32 = 0x006f_a820;
const BODY_WORD: u32 = 0x0051_7630;
const BODY_VECTOR: u32 = 0x0056_0d80;
/// `00c8cce0` (unnamed, `bhkrigidbody.obj`): `cdecl`, one word (an address); a `bool`.
const BODY_TEST: u32 = 0x00c8_cce0;
/// The float (`0x010177e8`, 0.9) and doubles (`0x01017a40`, 100.0; `0x0101ffa0`, 0.1)
/// `fn_009b4460` scales the vector with.
const SCALE_FLOAT: u32 = 0x0101_77e8;
const HUNDRED: u32 = 0x0101_7a40;
const TENTH: u32 = 0x0101_ffa0;
/// `004b4d50` (unnamed): `thiscall`, four floats (`RET 0x10`); sets a 16-byte vector.
const VECTOR_SET: u32 = 0x004b_4d50;
/// `00561690` (unnamed, `tesobjectrefr.cpp`): `thiscall`, one word (`RET 4`).
const BODY_SET_VECTOR: u32 = 0x0056_1690;
/// `TESHavokUtilities::AddVelocity` (`0062b8d0`, Xbox PDB): `cdecl`, three words.
const ADD_VELOCITY: u32 = 0x0062_b8d0;
/// The point `0x011f426c` (three words) the code copies as a zero point.
const ZERO_POINT: u32 = 0x011f_426c;
/// `TESObjectREFR::SetLight` (`005726e0`, engine map; `RET 8`).
const SET_LIGHT: u32 = 0x0057_26e0;
/// `009c43e0` (unnamed, `projectile.cpp`): `thiscall`, two words (`RET 8`); a `bool`.
const PROJECTILE_009C43E0: u32 = 0x009c_43e0;
/// `0047c850` (unnamed): `thiscall` on the object at `0x011de45c`; a `bool`.
const PAUSE_OWNER: u32 = 0x011d_e45c;
const IS_PAUSED: u32 = 0x0047_c850;
/// `bhkWorld::UpdatePosition` (`00c69f50`, Xbox PDB): `cdecl`, three words.
const UPDATE_POSITION: u32 = 0x00c6_9f50;
/// `0087ce50` (unnamed, `taskqueueinterface.cpp`): `thiscall`; a float in `ST0`.
const NODE_FLOAT: u32 = 0x0087_ce50;
/// `0045bb20` (unnamed, `tes.cpp`): `thiscall`, two words (`RET 8`): an out point and a
/// float; the point scaled.
const POINT_SCALED: u32 = 0x0045_bb20;
/// `00439e90` / `00439ef0` (unnamed, `multiboundmarkerdata.cpp`): `thiscall`, two words
/// (`RET 8`): an out point and another point; the sum and the difference.
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00440460` (unnamed, `modelloader.cpp`): `thiscall`, one word (`RET 4`); sets the
/// node's local translation.
const SET_LOCAL_TRANSLATE: u32 = 0x0044_0460;
/// `0043d410` (unnamed, `modelloader.cpp`): `thiscall`, three words (`RET 0xc`);
/// constructs a point.
const POINT_CONSTRUCT: u32 = 0x0043_d410;
/// `00a59c60` (unnamed): `thiscall`, one word (`RET 4`).
const NODE_SET_POINT: u32 = 0x00a5_9c60;
/// `00461130` (unnamed): `thiscall`, returns `this + 0x68` (the node's world rotation).
const WORLD_ROTATE: u32 = 0x0046_1130;
/// `00439f50` (unnamed, `multiboundmarkerdata.cpp`): `thiscall`, two words (`RET 8`): a
/// column index and an out point.
const MATRIX_COLUMN: u32 = 0x0043_9f50;
/// `00439180` (unnamed, `multiboundmarkerdata.cpp`): `thiscall`, one float (`RET 4`);
/// scales the point.
const SCALE_BY: u32 = 0x0043_9180;
/// `-1.0` (`float`, `0x01012054`) and `2.0` (`float`, `0x010162c0`).
const MINUS_ONE: u32 = 0x0101_2054;
const TWO_FLOAT: u32 = 0x0101_62c0;
/// `00525340` (unnamed, `tesobjectweap.cpp`): `cdecl`, one word; normalizes the point.
const NORMALIZE: u32 = 0x0052_5340;
/// `NiPoint3::Dot` (`004b6190`, Xbox PDB): `thiscall`, one word (`RET 4`); `ST0`.
const DOT: u32 = 0x004b_6190;
/// `004a3760` (unnamed, `bgsdecalmanager.cpp`): `cdecl` (out point, float, point): the
/// point times the float.
const FLOAT_TIMES_POINT: u32 = 0x004a_3760;
/// `0069ef80`, `006d2c20` (unnamed): `thiscall` on the shooter; floats in `ST0`.
const SHOOTER_VALUE_0069EF80: u32 = 0x0069_ef80;
const SHOOTER_VALUE_006D2C20: u32 = 0x006d_2c20;
/// `00810660` (unnamed, `magicballprojectile.cpp`): `thiscall`; a word.
const CONTACT_NODE: u32 = 0x0081_0660;
/// `TESObjectREFR::FindReferenceFor3D` (`0056f930`, Xbox PDB): `cdecl`, one word.
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `00621a90` (unnamed) then `00460140` (`thiscall`, returns `this + 0x10`).
const CONTACT_SHAPE: u32 = 0x0062_1a90;
const MEMBER_AT_10: u32 = 0x0046_0140;
/// `bhkWorld::SetMotion` (`00c6a350`, Xbox PDB): `cdecl`, five words.
const SET_MOTION: u32 = 0x00c6_a350;
/// `00624020` (unnamed): `thiscall`; a collidable's word.
const COLLIDABLE_WORD: u32 = 0x0062_4020;
/// `004b59f0` / `004b5a20` (unnamed, `globalfunc.cpp`): `cdecl`, one word each.
const COLLIDABLE_OBJECT: u32 = 0x004b_59f0;
const OBJECT_OWNER: u32 = 0x004b_5a20;
/// `00458620` (unnamed, `tes.cpp`): `cdecl` (destination, source); copies a point.
const COPY_POINT: u32 = 0x0045_8620;

/// `008d6f30` / `004543c0` (unnamed): `thiscall`; an actor's cell and the cell's world
/// (havok) object.
const PARENT_CELL: u32 = 0x008d_6f30;
const CELL_WORLD: u32 = 0x0045_43c0;
/// `004a3c20` (unnamed): constructs the ray-cast input local (`ECX` only).
const RAY_CONSTRUCT: u32 = 0x004a_3c20;
/// `004a0c90` (unnamed, `bgsdecalmanager.cpp`): `thiscall`, one float (`RET 4`); makes
/// a rotation matrix.
const MATRIX_ROTATION: u32 = 0x004a_0c90;
/// `MobileObject::GetLooking` (`00931d70`, Xbox PDB): `thiscall`; a float in `ST0`.
const GET_LOOKING: u32 = 0x0093_1d70;
/// `NiMatrix3::MakeXRotation` (`00524ac0`, Xbox PDB): `thiscall`, one float (`RET 4`).
const MAKE_X_ROTATION: u32 = 0x0052_4ac0;
/// `NiMatrix3::operator*` (`0043f8d0`, engine map `NiMatrix3::operatorP`): `thiscall`,
/// two words (`RET 8`).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `004a0c10` (unnamed, `bgsdecalmanager.cpp`): `thiscall`; normalizes the point.
const POINT_NORMALIZE: u32 = 0x004a_0c10;
/// `008c71b0` (named `BGSChangeFlags::IsForcedChange`; a folded body): `thiscall`, one
/// word (`RET 4`); constructs the filter word.
const FILTER_CONSTRUCT: u32 = 0x008c_71b0;
/// `MobileObject::GetCharController` (`009306d0`, Xbox PDB): `thiscall`; a word.
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `0070c440` (unnamed): `thiscall`, one word (`RET 4`); the controller's filter word
/// (returns its address).
const CONTROLLER_FILTER: u32 = 0x0070_c440;
/// `004a39f0` (unnamed, `bgsdecalmanager.cpp`): `thiscall`, one word (`RET 4`).
const FILTER_SET_LAYER: u32 = 0x004a_39f0;
/// `00874480` (unnamed, `main.cpp` per the engine map): `thiscall`; the source object.
const SOURCE_OBJECT: u32 = 0x0087_4480;
/// `00931ed0` (unnamed, `mobileobject.cpp`): `thiscall`, one word (`RET 4`); then
/// `004a3a20` (`thiscall`) gives the filter group word.
const SOURCE_FILTER: u32 = 0x0093_1ed0;
const FILTER_GROUP: u32 = 0x004a_3a20;
/// `0059ce80` (unnamed, `tesconditionfunctions.cpp`): `thiscall`, one word (`RET 4`); sets
/// the filter's group.
const FILTER_SET_GROUP: u32 = 0x0059_ce80;
/// `00500a20` (unnamed, `bgsterminal.cpp`): `thiscall`, one word (`RET 4`).
const RAY_SET_IGNORED: u32 = 0x0050_0a20;
/// `eh vector constructor iterator` (`00401050`, `stdcall`: array, size, count,
/// constructor).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// `004a4240`, `009a7c40`, `00645ce0` (unnamed): floats in `ST0` (no stack words; the
/// second on the shooter, the third on `this`).
const RAY_VALUE_004A4240: u32 = 0x004a_4240;
const SHOOTER_VALUE_009A7C40: u32 = 0x009a_7c40;
const OWN_VALUE_00645CE0: u32 = 0x0064_5ce0;
/// `009c3f90` (unnamed, `projectile.cpp`): `cdecl`, ten words; fills the four ray ends.
const RAY_ENDS: u32 = 0x009c_3f90;
/// `00979260`, `00508070` (unnamed): `thiscall`; floats in `ST0`.
const LENGTH_00979260: u32 = 0x0097_9260;
const LENGTH_00508070: u32 = 0x0050_8070;
/// `004a3f70`, `004a3da0`, `007f68a0`, `004a3fb0` (unnamed): `thiscall`, one word each
/// (`RET 4`); set the ray's filter, start, end and collector.
const RAY_SET_FILTER: u32 = 0x004a_3f70;
const RAY_SET_START: u32 = 0x004a_3da0;
const RAY_SET_END: u32 = 0x007f_68a0;
const RAY_SET_COLLECTOR: u32 = 0x004a_3fb0;
/// `hkpAllRayHitCollector`'s constructor (`004a3a70`) and destructor (`004a3bc0`), Xbox
/// PDB (`ECX` only).
const COLLECTOR_CONSTRUCT: u32 = 0x004a_3a70;
const COLLECTOR_DESTRUCT: u32 = 0x004a_3bc0;
/// `009c0d80` (unnamed, `projectile.cpp`): `cdecl`, six words; a word.
const RAY_HIT_TEST: u32 = 0x009c_0d80;
/// The float `FLT_MAX` (`0x01016970`).
const FLOAT_MAX: u32 = 0x0101_6970;
/// The `Setting` at `0x011f20ec` whose byte (`00408d60`) turns the debug lines on.
const DEBUG_SETTING: u32 = 0x011f_20ec;
const SETTING_BYTE: u32 = 0x0040_8d60;
/// `00632ce0` (unnamed): `thiscall`; whether the ray hit.
const RAY_HIT: u32 = 0x0063_2ce0;
/// `00414430` (unnamed, `extradatalist.cpp`): `thiscall`, four floats (`RET 0x10`);
/// constructs a colour.
const COLOR_CONSTRUCT: u32 = 0x0041_4430;
/// `004b3890` (unnamed, `globalfunc.cpp`): `cdecl`, five words; a debug line object.
const DEBUG_LINE: u32 = 0x004b_3890;
/// The `Setting` at `0x011f210c` (the line's time) and `TES::AddTempDebugObject`
/// (`00458e20`, Xbox PDB, `RET 8`) on the object at `0x011dea10`.
const DEBUG_TIME_SETTING: u32 = 0x011f_210c;
const TES: u32 = 0x011d_ea10;
const ADD_TEMP_DEBUG_OBJECT: u32 = 0x0045_8e20;

// Translated from 009b36f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b36f0 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
/// Unnamed in the engine map (`ECX` only; a projectile's ray test): false without a cell
/// world (`008d6f30`, `004543c0`). Otherwise casts four rays from the projectile's
/// position (slot `+0x1f4`): the ray input (`004a3c20`) gets a filter (the character
/// controller's, `0070c440`, or layer 6 with the source's group, `00931ed0`/`004a3a20`,
/// or a new key `005dbe10`) and an ignored object (`009b3cb0`'s count, `00500a20`); the
/// four ends come from `009c3f90` (with the position, slot `+0x2bc`'s, `00931d70`'s,
/// `00645ce0`'s, the shooter's `009a7c40` and `004a4240`'s floats). Each ray, from the
/// position to its end, goes to the player's cell world (slot `+0xc8`) with an
/// `hkpAllRayHitCollector`; a hit that `009c0d80` confirms makes the result true. With
/// the debug setting (`0x011f20ec`) each ray is also drawn (`004b3890`, red when it hit,
/// yellow when not, for the time `0x011f210c` says; `TES::AddTempDebugObject`). The
/// rotation the facing and looking angles make (`004a0c90`, `00524ac0`, `0043f8d0`) and
/// its column 1 (`00439f50`, `004a0c10`) give the point the length (`00979260` minus
/// `00508070`) away, which is computed but not used further.
pub fn fn_009b36f0(e: &mut Engine, this: Ptr) -> bool {
    let p = this.addr();
    if e.call(PARENT_CELL, &args![this]).u32() == 0 {
        return false;
    }
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    if e.call(CELL_WORLD, &args![cell]).u32() == 0 {
        return false;
    }
    e.with_stack(0x540, |e, frame| {
        let base = frame.addr() + 0x540;
        let at = |offset: u32| base - offset;
        let mut result = false;
        let cell = e.call(PARENT_CELL, &args![this]).u32();
        let _world = e.call(CELL_WORLD, &args![cell]).u32();
        let ray = at(0xd0);
        let position = at(0xdc);
        let direction = at(0xe8);
        let rotation = at(0x10c);
        let looking = at(0x130);
        let filter = at(0x158);
        let ends = at(0x198);
        e.call(RAY_CONSTRUCT, &args![ray]);
        e.call(NODE_ITEM, &args![position]);
        e.call(NODE_ITEM, &args![direction]);
        let source_position = e.vcall(p, 0x1f4, &[]).u32();
        let words = e.mem.bytes(source_position, 0xc);
        e.mem.write(position, &words);
        e.call(NODE_ITEM, &args![rotation]);
        let facing = e.vcall(p, 0x2bc, &args![0u32]).f64();
        e.call(MATRIX_ROTATION, &args![rotation, x87_store32(facing)]);
        e.call(NODE_ITEM, &args![looking]);
        let look = e.call(GET_LOOKING, &args![this]).f64();
        e.call(MAKE_X_ROTATION, &args![looking, x87_store32(look)]);
        let product = e
            .call(MATRIX_MULTIPLY, &args![rotation, at(0x154), looking])
            .u32();
        let words = e.mem.bytes(product, 0x24);
        e.mem.write(rotation, &words);
        e.call(MATRIX_COLUMN, &args![rotation, 1u32, direction]);
        e.call(POINT_NORMALIZE, &args![direction]);
        e.call(FILTER_CONSTRUCT, &args![filter, 0u32]);
        if e.call(GET_CHAR_CONTROLLER, &args![this]).u32() != 0 {
            let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
            let word = e
                .call(CONTROLLER_FILTER, &args![controller, at(0x15c)])
                .u32();
            let value = e.mem.u32(word);
            e.mem.set_u32(filter, value);
        } else {
            e.call(FILTER_SET_LAYER, &args![filter, 6u32]);
            let source_counts = if e.call(SOURCE_OBJECT, &args![this]).u32() != 0 {
                let source = e.call(SOURCE_OBJECT, &args![this]).u32();
                e.vcall(source, 0x100, &[]).u8() != 0
            } else {
                false
            };
            if source_counts {
                let source = e.call(SOURCE_OBJECT, &args![this]).u32();
                let object = e.call(SOURCE_FILTER, &args![source, at(0x164)]).u32();
                let group = e.call(FILTER_GROUP, &args![object]).u32();
                e.call(FILTER_SET_GROUP, &args![filter, group]);
            } else {
                let key = e.call(NEXT_KEY, &[]).u32();
                e.call(FILTER_SET_GROUP, &args![filter, key]);
            }
        }
        if fn_009b3cb0(e, this) != 0 {
            let ignored = fn_009b3cb0(e, this);
            if e.call(ARRAY_COUNT, &args![ignored]).u32() != 0 {
                let ignored = fn_009b3cb0(e, this);
                let count = e.call(ARRAY_COUNT, &args![ignored]).u32();
                e.call(RAY_SET_IGNORED, &args![ray, count]);
            }
        }
        e.mem.set_u32(at(0x168), 4);
        e.call(VECTOR_CONSTRUCT, &args![ends, 0xcu32, 4u32, NODE_ITEM]);
        let last = x87_store32(e.call(RAY_VALUE_004A4240, &[]).f64());
        let shooter = e.call(SHOOTER, &args![this]).u32();
        let shooter_float = x87_store32(e.call(SHOOTER_VALUE_009A7C40, &args![shooter]).f64());
        let own_float = x87_store32(e.call(OWN_VALUE_00645CE0, &args![this]).f64());
        let look = x87_store32(e.call(GET_LOOKING, &args![this]).f64());
        let facing = x87_store32(e.vcall(p, 0x2bc, &args![0u32]).f64());
        let x = e.mem.u32(position);
        let y = e.mem.u32(position + 4);
        let z = e.mem.u32(position + 8);
        e.call(
            RAY_ENDS,
            &args![
                ends,
                3u32,
                x,
                y,
                z,
                facing,
                look,
                own_float,
                shooter_float,
                last
            ],
        );
        let full = e.call(LENGTH_00979260, &args![this]).f64();
        let used = e.call(LENGTH_00508070, &args![this]).f64();
        let length = x87_store32(x87_sub(full, used));
        let scaled = e
            .call(
                POINT_SCALED,
                &args![direction, at(0x1b0), x87_quiet_bits(length)],
            )
            .u32();
        let end = e.call(POINT_ADD, &args![position, at(0x1bc), scaled]).u32();
        let words = e.mem.bytes(end, 0xc);
        e.mem.write(at(0x174), &words);
        let filter_word = e.mem.u32(filter);
        e.call(RAY_SET_FILTER, &args![ray, filter_word]);
        e.call(RAY_SET_START, &args![ray, position]);
        let mut index = 0i32;
        while index < 4 {
            let ray_end = ends + (index as u32) * 0xc;
            let target = e
                .call(POINT_SUBTRACT, &args![ray_end, at(0x1cc), position])
                .u32();
            e.call(RAY_SET_END, &args![ray, target]);
            let collector = at(0x4f0);
            e.call(COLLECTOR_CONSTRUCT, &args![collector]);
            e.call(RAY_SET_COLLECTOR, &args![ray, collector]);
            let player = e.mem.u32(PLAYER);
            let player_cell = e.call(PARENT_CELL, &args![player]).u32();
            let player_world = e.call(CELL_WORLD, &args![player_cell]).u32();
            let mut hit = e.vcall(player_world, 0xc8, &args![ray]).u8() != 0;
            if hit {
                let limit = x87_quiet_bits(e.mem.u32(FLOAT_MAX));
                hit = e
                    .call(RAY_HIT_TEST, &args![ray, this, 0u32, 0u32, 0u32, limit])
                    .u32()
                    != 0;
            }
            if hit {
                result = true;
            }
            let enabled = e.call(SETTING_BYTE, &args![DEBUG_SETTING]).u32();
            if e.mem.u8(enabled) != 0 {
                let one = 1.0f32.to_bits();
                let zero = 0.0f32.to_bits();
                let color = if e.call(RAY_HIT, &args![ray]).u8() != 0 {
                    e.call(COLOR_CONSTRUCT, &args![at(0x508), one, zero, zero, one])
                        .u32()
                } else {
                    e.call(COLOR_CONSTRUCT, &args![at(0x51c), one, one, zero, one])
                        .u32()
                };
                let words = e.mem.bytes(color, 0x10);
                let kept = at(0x530);
                e.mem.write(kept, &words);
                let line = e
                    .call(DEBUG_LINE, &args![position, kept, ray_end, kept, 1u32])
                    .u32();
                let time = e.call(SETTING_FLOAT, &args![DEBUG_TIME_SETTING]).u32();
                let time = x87_quiet_bits(e.mem.u32(time));
                let tes = e.mem.u32(TES);
                e.call(ADD_TEMP_DEBUG_OBJECT, &args![tes, line, time]);
            }
            e.call(COLLECTOR_DESTRUCT, &args![collector]);
            index += 1;
        }
        result
    })
}

// Translated from 009b3cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the word at `+0x11c` (`00559450`).
pub fn fn_009b3cb0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(READ_FIRST_WORD, &args![this.addr() + 0x11c]).u32()
}

// Translated from 009b3cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x18`): an impact on `target` (with the
/// other five words). False unless `009c2c80` allows it (`target`, `fourth`, `this`, the
/// shooter `004181e0`). When the impact list at `+0x88` (`008256d0`, `006815c0`,
/// `00726070`) already has an entry for `target`, true unless `target` is set and its slot
/// `+0x100` is true; otherwise `Projectile::AddImpact`'s answer.
pub fn fn_009b3cd0(
    e: &mut Engine,
    this: Ptr,
    target: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) -> bool {
    let p = this.addr();
    let shooter = e.call(SHOOTER, &args![this]).u32();
    if e.call(IMPACT_ALLOWED, &args![target, fourth, this, shooter])
        .u8()
        == 0
    {
        return false;
    }
    let mut node = p + 0x88;
    while node != 0 && e.call(LIST_IS_EMPTY, &args![node]).u8() == 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if target == e.mem.u32(entry) {
            if target != 0 && e.vcall(target, 0x100, &[]).u8() != 0 {
                return false;
            }
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(
        ADD_IMPACT,
        &args![this, target, second, third, fourth, fifth, sixth],
    )
    .u8()
        != 0
}

// Translated from 009b3da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): the size of the particles under `node`:
/// for a `BSStripParticleSystem` (`0043b300`), the first non-zero size of its
/// `NiPSysEmitter` modifiers (`00526ac0`); then the largest of that and its children's
/// (slot `+0xc` node, `0043b480`, `0043b4a0`), recursively. 0.0 for no node.
pub fn fn_009b3da0(e: &mut Engine, node: u32) -> f32 {
    let mut size = 0.0f32.to_bits();
    if node == 0 {
        return f32::from_bits(x87_quiet_bits(size));
    }
    let zero = f64::from_bits(e.mem.u64(ZERO));
    if e.call(IS_KIND_OF, &args![RTTI_BS_STRIP_PARTICLE_SYSTEM, node])
        .u8()
        != 0
    {
        let mut index = 0u32;
        // `TEST AH,0x44; JP`: the loop goes on only while C3 alone is set (equal to 0.0).
        while x87_load32(size) == zero && index < e.call(MODIFIER_COUNT, &args![node]).u32() {
            let modifier = e.call(MODIFIER_AT, &args![node, index]).u32();
            if modifier != 0
                && e.call(IS_KIND_OF, &args![RTTI_NI_PSYS_EMITTER, modifier])
                    .u8()
                    != 0
            {
                size = x87_store32(e.call(EMITTER_SIZE, &args![modifier]).f64());
            }
            index = index.wrapping_add(1);
        }
    }
    let children = e.vcall(node, 0xc, &[]).u32();
    if children == 0 {
        return f32::from_bits(x87_quiet_bits(size));
    }
    let mut index = 0u32;
    while index < e.call(CHILD_COUNT, &args![children]).u32() {
        let child = e.call(CHILD_AT, &args![children, index]).u32();
        let child_size = x87_store32(f64::from(fn_009b3da0(e, child)));
        // `FCOMPP` of the size against the child's, `TEST AH,0x5; JP`: taken when C0
        // alone is set (the size is below).
        if x87_load32(size) < x87_load32(child_size) {
            size = x87_quiet_bits(child_size);
        }
        index = index.wrapping_add(1);
    }
    f32::from_bits(x87_quiet_bits(size))
}

// Translated from 009b3eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `Projectile::SaveGame`, then the floats
/// at `+0x150` and `+0x154` into `buffer` (`00865e50`).
pub fn fn_009b3eb0(e: &mut Engine, this: Ptr, buffer: u32) {
    let p = this.addr();
    e.call(PROJECTILE_SAVE_GAME, &args![this, buffer]);
    e.call(SAVE_DATA, &args![buffer, p + 0x150, 4u32, 0u32]);
    e.call(SAVE_DATA, &args![buffer, p + 0x154, 4u32, 0u32]);
}

// Translated from 009b3f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `Projectile::LoadGame`, then the
/// floats at `+0x150` and `+0x154` from `buffer` (`00864980`).
pub fn fn_009b3f00(e: &mut Engine, this: Ptr, buffer: u32) {
    let p = this.addr();
    e.call(LOAD_GAME_TARGET_009C52F0, &args![this, buffer]);
    e.call(LOAD_DATA, &args![buffer, p + 0x150, 4u32]);
    e.call(LOAD_DATA, &args![buffer, p + 0x154, 4u32]);
}

// Translated from 009b3f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `009c58e0` with `buffer`, then 0.0 in
/// the floats at `+0x150` and `+0x154`.
pub fn fn_009b3f50(e: &mut Engine, this: Ptr, buffer: u32) {
    let p = this.addr();
    e.call(PROJECTILE_INIT_LOAD, &args![this, buffer]);
    e.mem.set_u32(p + 0x150, 0.0f32.to_bits());
    e.mem.set_u32(p + 0x154, 0.0f32.to_bits());
}

// Translated from 009b4020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b4020 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `GrenadeProjectile`'s other constructor (unnamed in the engine map; `thiscall`,
/// `RET 0x20`): the `Projectile` constructor `009bbef0` with the three words, the point
/// (three words, by value) and the two floats (copied through `FLD`/`FSTP`), the two
/// vtables, the byte at `+0x150` cleared and the form type 0x3e (`004f15a0`). Returns
/// `this`.
pub fn fn_009b4020(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    point_x: u32,
    point_y: u32,
    point_z: u32,
    float_a: f32,
    float_b: f32,
) -> Ptr {
    let base = this.addr();
    e.call(
        PROJECTILE_CONSTRUCT_WITH,
        &args![
            this,
            first,
            second,
            third,
            point_x,
            point_y,
            point_z,
            x87_quiet_bits(float_a.to_bits()),
            x87_quiet_bits(float_b.to_bits())
        ],
    );
    e.mem.set_u32(base, GRENADE_VTABLE);
    e.mem.set_u32(base + 0x18, GRENADE_VTABLE_18);
    e.mem.set_u8(base + 0x150, 0);
    e.call(SET_FORM_TYPE, &args![this, 0x3eu32]);
    this
}

// Translated from 009b40d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only; a `GrenadeProjectile` function): sets its flags
/// (`005de0a0`): 4; with flag 0x400 (`00444d00`) 2 set and 0x100 cleared, otherwise 0x100
/// set; 0x2000 set when the shooter (`004181e0`) passes `004fd360` and fails both
/// `00975300` and `005de080`, cleared otherwise; 0x8000 and 0x40000 set; then `009bda10`.
pub fn fn_009b40d0(e: &mut Engine, this: Ptr) {
    e.call(SET_FLAG, &args![this, 4u32, 1u32]);
    if e.call(HAS_FLAG, &args![this, 0x400u32]).u8() != 0 {
        e.call(SET_FLAG, &args![this, 2u32, 1u32]);
        e.call(SET_FLAG, &args![this, 0x100u32, 0u32]);
    } else {
        e.call(SET_FLAG, &args![this, 0x100u32, 1u32]);
    }
    let shooter = e.call(SHOOTER, &args![this]).u32();
    let mut bounce = false;
    if e.call(SHOOTER_TEST_004FD360, &args![shooter]).u8() != 0 {
        let shooter = e.call(SHOOTER, &args![this]).u32();
        if e.call(SHOOTER_TEST_00975300, &args![shooter]).u8() == 0 {
            let shooter = e.call(SHOOTER, &args![this]).u32();
            if e.call(SHOOTER_TEST_005DE080, &args![shooter]).u8() == 0 {
                bounce = true;
            }
        }
    }
    e.call(SET_FLAG, &args![this, 0x2000u32, u32::from(bounce)]);
    e.call(SET_FLAG, &args![this, 0x8000u32, 1u32]);
    e.call(SET_FLAG, &args![this, 0x4_0000u32, 1u32]);
    e.call(PROJECTILE_INITIALIZE, &args![this]);
}

// Translated from 009b41b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b41b0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 4`; a `GrenadeProjectile` update with the
/// frame `time`): nothing when `00440d80` says so. Once older than 1.0 (`00885bd0`; the
/// byte at `+0x150` remembers it), its node gets a new collision key (`00c6a3d0` with
/// `005dbe10`'s key, 1, 0), and, when the shooter has a world space (`004fd3e0`) without
/// water there (`004e75d0`, `005e39b0`), the world space's sound (`00511840`, flags 0x102)
/// plays at its position (slot `+0x1f4`; `BSAudio::GetSoundHandleByFilename`, `0068a7d0`,
/// `BSSoundHandle::Play`). A time limit (the float of `Setting` `0x011cee0c` when the
/// shooter passes `00975300` with `0045cd80` above 0.0 or passes `005de080`, of
/// `0x011cf534` otherwise) that is not 0.0 and is below the age expires it (`009bc8f0`)
/// when the data handler (`0x011c3f2c`, `00469860`) accepts its form (`0084e3a0`). With
/// flag 0x400 and an attached reference (`fn_009b4440`), it detaches (`009bf190`) and
/// moves to that reference's position (`00575830`). Then `Projectile::CheckExplosion`.
pub fn fn_009b41b0(e: &mut Engine, this: Ptr, time: f32) {
    let p = this.addr();
    if e.call(IS_DISABLED, &args![this]).u8() != 0 {
        return;
    }
    let one = f64::from_bits(e.mem.u64(ONE));
    let zero = f64::from_bits(e.mem.u64(ZERO));
    if e.mem.u8(p + 0x150) == 0 && e.call(AGE, &args![this]).f64() > one {
        let key = e.call(NEXT_KEY, &[]).u32();
        let node = e.vcall(p, 0x1d0, &[]).u32();
        e.call(SET_COLLISION_KEY, &args![node, key, 1u32, 0u32]);
        e.mem.set_u8(p + 0x150, 1);
        let has_world = if e.call(SHOOTER, &args![this]).u32() != 0 {
            let shooter = e.call(SHOOTER, &args![this]).u32();
            e.call(SHOOTER_WORLD, &args![shooter]).u32() != 0
        } else {
            false
        };
        if has_world {
            let dry = e.with_stack(0x40, |e, local| {
                let shooter = e.call(SHOOTER, &args![this]).u32();
                let world = e.call(SHOOTER_WORLD, &args![shooter]).u32();
                let water = e.call(WATER_AT, &args![world, local]).u32();
                let water_word = e.mem.u32(water + 0x4);
                e.call(WATER_TEST, &args![water_word]).u32() == 0
            });
            if dry {
                e.with_stack(0xc, |e, handle| {
                    let shooter = e.call(SHOOTER, &args![this]).u32();
                    let world = e.call(SHOOTER_WORLD, &args![shooter]).u32();
                    let shooter = e.call(SHOOTER, &args![this]).u32();
                    let world_again = e.call(SHOOTER_WORLD, &args![shooter]).u32();
                    let name = e.call(SOUND_NAME, &args![world_again]).u32();
                    let audio = e.call(AUDIO_INSTANCE, &[]).u32();
                    e.call(
                        GET_SOUND_HANDLE,
                        &args![audio, handle, name, 0x102u32, world],
                    );
                    let position = e.vcall(p, 0x1f4, &[]).u32();
                    e.call(SOUND_SET_POSITION, &args![handle, position]);
                    e.call(SOUND_PLAY, &args![handle, 0u32]);
                    e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
                });
            }
        }
    }

    let shooter = e.call(SHOOTER, &args![this]).u32();
    let mut limit_setting = None;
    if e.call(SHOOTER_TEST_00975300, &args![shooter]).u8() != 0 {
        let shooter = e.call(SHOOTER, &args![this]).u32();
        if e.call(SHOOTER_FLOAT, &args![shooter]).f64() > zero {
            limit_setting = Some(SETTING_011CEE0C);
        }
    }
    let limit_setting = match limit_setting {
        Some(setting) => setting,
        None => {
            let shooter = e.call(SHOOTER, &args![this]).u32();
            if e.call(SHOOTER_TEST_005DE080, &args![shooter]).u8() != 0 {
                SETTING_011CEE0C
            } else {
                SETTING_011CF534
            }
        }
    };
    let limit = e.call(SETTING_FLOAT, &args![limit_setting]).u32();
    let limit = x87_quiet_bits(e.mem.u32(limit));
    // `TEST AH,0x44; JNP`: skipped only when equal to 0.0 (C3 alone).
    if x87_load32(limit) != zero {
        let age = e.call(AGE, &args![this]).f64();
        // `FCOMPP` of the limit against the age, `TEST AH,0x5; JP`: on only when below.
        if x87_load32(limit) < age {
            let form = e.call(PROJECTILE_FORM, &args![this]).u32();
            let handler = e.mem.u32(DATA_HANDLER);
            if e.call(DATA_HANDLER_TEST, &args![handler, form]).u8() != 0 {
                e.call(PROJECTILE_EXPIRE, &args![this]);
            }
        }
    }
    if e.call(HAS_FLAG, &args![this, 0x400u32]).u8() != 0 && fn_009b4440(e, this) != 0 {
        e.call(PROJECTILE_DETACH, &args![this]);
        let attached = fn_009b4440(e, this);
        let position = e.vcall(attached, 0x1f4, &[]).u32();
        e.call(SET_LOCATION_ON_REFERENCE, &args![this, position]);
    }
    e.call(
        CHECK_EXPLOSION,
        &args![this, x87_quiet_bits(time.to_bits())],
    );
}

// Translated from 009b4440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the word at `+0x100`.
pub fn fn_009b4440(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x100)
}

// Translated from 009b4460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b4460 - the /GS cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 0x18`; an impact of a `GrenadeProjectile`):
/// while the array at `0x011f2250` holds 4 items, the node's rigid body (`0043b610`,
/// `006fa820`) that passes `00c8cce0` gets its vector (`00560d80`) multiplied (`fn_009b4670`)
/// by `(f, f, f, 0)`, `f = (100 - the player's value 0x23) / 100.0 * 0.9 + 0.1`, and set
/// back (`00561690`). Then, with an empty impact list (`008256d0` on `+0x88`), when
/// `009c2c80` allows it and `Projectile::AddImpact` takes it: a set `target` whose slot
/// `+0x100` is true pushes the node (`TESHavokUtilities::AddVelocity` with the zero point
/// and 1); otherwise, at age 0.0 (`00885bd0`), `fn_009b4740` and the byte at `+0x90` set.
/// True then, false otherwise.
pub fn fn_009b4460(
    e: &mut Engine,
    this: Ptr,
    target: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) -> bool {
    let p = this.addr();
    if e.call(ARRAY_COUNT, &args![COUNTED_ARRAY]).u32() == 4 {
        let node = e.vcall(p, 0x1d0, &[]).u32();
        let collision = e.call(GET_COLLISION_OBJECT, &args![node]).u32();
        if collision != 0 {
            let body = e.call(RIGID_BODY, &args![collision]).u32();
            if body != 0 {
                e.with_stack(0x40, |e, frame| {
                    let word_slot = frame.addr();
                    let vector = frame.addr() + 0x10;
                    let word = e.call(BODY_WORD, &args![body]).u32();
                    e.mem.set_u32(word_slot, word);
                    if e.call(BODY_TEST, &args![word_slot]).u8() != 0 {
                        e.call(NODE_ITEM, &args![vector]);
                        let mut factor = x87_quiet_bits(e.mem.u32(SCALE_FLOAT));
                        let player = e.mem.u32(PLAYER);
                        let owner = player + 0xa4;
                        let value = e.vcall(owner, 0x8, &args![0x23u32]).i32();
                        let rest = 0x64i32.wrapping_sub(value);
                        let hundred = f64::from_bits(e.mem.u64(HUNDRED));
                        let ratio = x87_div(f64::from(rest), hundred);
                        factor = x87_store32(x87_mul(ratio, x87_m32(factor)));
                        let tenth = f64::from_bits(e.mem.u64(TENTH));
                        factor = x87_store32(x87_add(x87_load32(factor), tenth));
                        let f = x87_quiet_bits(factor);
                        e.call(VECTOR_SET, &args![vector, f, f, f, 0.0f32.to_bits()]);
                        let other = e.call(BODY_VECTOR, &args![body]).u32();
                        fn_009b4670(e, Ptr::new(vector), other);
                        e.call(BODY_SET_VECTOR, &args![body, vector]);
                    }
                });
            }
        }
    }
    if e.call(LIST_IS_EMPTY, &args![p + 0x88]).u8() == 0 {
        return false;
    }
    let shooter = e.call(SHOOTER, &args![this]).u32();
    if e.call(IMPACT_ALLOWED, &args![target, fourth, this, shooter])
        .u8()
        == 0
    {
        return false;
    }
    if e.call(
        ADD_IMPACT,
        &args![this, target, second, third, fourth, fifth, sixth],
    )
    .u8()
        == 0
    {
        return false;
    }
    if target != 0 && e.vcall(target, 0x100, &[]).u8() != 0 {
        let node = e.vcall(p, 0x1d0, &[]).u32();
        e.call(ADD_VELOCITY, &args![node, ZERO_POINT, 1u32]);
    } else {
        let zero = f64::from_bits(e.mem.u64(ZERO));
        // `TEST AH,0x44; JP`: on only when C3 alone is set (equal to 0.0).
        if e.call(AGE, &args![this]).f64() == zero {
            fn_009b4740(e, this);
            e.mem.set_u8(p + 0x90, 1);
        }
    }
    true
}

// Translated from 009b4670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b4670 - the /GS cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 4`): multiplies the 16-byte vector at
/// `this` by the one at `other`, lane by lane (`MULPS`).
pub fn fn_009b4670(e: &mut Engine, this: Ptr, other: u32) {
    let load = |e: &Engine, at: u32| -> crate::sse::Xmm {
        [
            f32::from_bits(e.mem.u32(at)),
            f32::from_bits(e.mem.u32(at + 4)),
            f32::from_bits(e.mem.u32(at + 8)),
            f32::from_bits(e.mem.u32(at + 12)),
        ]
    };
    let factor = load(e, other);
    let value = load(e, this.addr());
    let product = crate::sse::mulps(value, factor);
    for (lane, v) in product.iter().enumerate() {
        e.mem.set_u32(this.addr() + 4 * lane as u32, v.to_bits());
    }
}

// Translated from 009b46c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): `TESObjectREFR::SetLight` (`005726e0`) with the
/// node (slot `+0x1d0`) and whether flag 0x200 (`00444d00`) is clear.
pub fn fn_009b46c0(e: &mut Engine, this: Ptr) {
    let clear = e.call(HAS_FLAG, &args![this, 0x200u32]).u8() == 0;
    let node = e.vcall(this.addr(), 0x1d0, &[]).u32();
    e.call(SET_LIGHT, &args![this, node, u32::from(clear)]);
}

// Translated from 009b4700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): `009c43e0` with `first` and the byte
/// `flag`, then slot `+0x30c`; returns `009c43e0`'s byte.
pub fn fn_009b4700(e: &mut Engine, this: Ptr, first: u32, flag: u8) -> bool {
    let result = e
        .call(PROJECTILE_009C43E0, &args![this, first, u32::from(flag)])
        .u8();
    e.vcall(this.addr(), 0x30c, &[]);
    result != 0
}

// Translated from 009b4740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only; a `GrenadeProjectile` bounce): with an impact in
/// the list at `+0x88` and the game not paused (`0047c850` on `0x011de45c`): moves the
/// node (`00c69f50`; then its translation to the impact's point `+4` plus its normal
/// `+0x10` scaled by `0087ce50`'s float, `00440460`) and clears its rotation point
/// (`0043d410`, `00a59c60`); the reflected direction (the node's column 1 times -1.0,
/// normalized, `00439f50`/`00439180`/`00525340`, minus the normal times twice its dot
/// with the normal, `004b6190`/`004a3760`/`00439ef0`/`00439e90`) scaled by the shooter's
/// `0069ef80` and, when above 0.0, its `006d2c20`, is added to the node's velocity
/// (`TESHavokUtilities::AddVelocity` with 1). Returns true.
pub fn fn_009b4740(e: &mut Engine, this: Ptr) -> bool {
    let p = this.addr();
    if e.call(LIST_IS_EMPTY, &args![p + 0x88]).u8() != 0 {
        return true;
    }
    let node = e.vcall(p, 0x1d0, &[]).u32();
    let slot = e.call(NODE_ITEM, &args![p + 0x88]).u32();
    let impact = e.mem.u32(slot);
    let pause_owner = e.mem.u32(PAUSE_OWNER);
    if e.call(IS_PAUSED, &args![pause_owner]).u8() != 0 {
        return true;
    }
    e.call(UPDATE_POSITION, &args![node, 1u32, 0u32]);
    e.with_stack(0x80, |e, frame| {
        let base = frame.addr() + 0x80;
        let at = |offset: u32| base - offset;
        let velocity = at(0x14);
        let rotation_point = at(0x20);
        let reflected = at(0x3c);
        let float = x87_store32(e.call(NODE_FLOAT, &args![node]).f64());
        let scaled = e
            .call(POINT_SCALED, &args![impact + 0x10, at(0x48), float])
            .u32();
        let moved = e
            .call(POINT_ADD, &args![impact + 0x4, at(0x54), scaled])
            .u32();
        e.call(SET_LOCAL_TRANSLATE, &args![node, moved]);
        e.call(
            POINT_CONSTRUCT,
            &args![rotation_point, 0.0f32.to_bits(), 0u32, 0u32],
        );
        e.call(NODE_SET_POINT, &args![node, rotation_point]);
        for word in 0..3 {
            let value = e.mem.u32(ZERO_POINT + 4 * word);
            e.mem.set_u32(velocity + 4 * word, value);
        }
        e.call(NODE_ITEM, &args![reflected]);
        let rotate = e.call(WORLD_ROTATE, &args![node]).u32();
        e.call(MATRIX_COLUMN, &args![rotate, 1u32, reflected]);
        let minus_one = x87_quiet_bits(e.mem.u32(MINUS_ONE));
        e.call(SCALE_BY, &args![reflected, minus_one]);
        e.call(NORMALIZE, &args![reflected]);
        e.call(NORMALIZE, &args![impact + 0x10]);
        let dot = x87_store32(e.call(DOT, &args![impact + 0x10, reflected]).f64());
        let along = e
            .call(
                FLOAT_TIMES_POINT,
                &args![at(0x60), x87_quiet_bits(dot), impact + 0x10],
            )
            .u32();
        let difference = e
            .call(POINT_SUBTRACT, &args![along, at(0x6c), reflected])
            .u32();
        let two = x87_quiet_bits(e.mem.u32(TWO_FLOAT));
        e.call(FLOAT_TIMES_POINT, &args![at(0x30), two, difference]);
        let sum = e
            .call(POINT_ADD, &args![reflected, at(0x78), at(0x30)])
            .u32();
        let words = e.mem.bytes(sum, 0xc);
        e.mem.write(velocity, &words);
        let shooter = e.call(SHOOTER, &args![this]).u32();
        let speed = x87_store32(e.call(SHOOTER_VALUE_0069EF80, &args![shooter]).f64());
        e.call(SCALE_BY, &args![velocity, speed]);
        let zero = f64::from_bits(e.mem.u64(ZERO));
        let shooter = e.call(SHOOTER, &args![this]).u32();
        if e.call(SHOOTER_VALUE_006D2C20, &args![shooter]).f64() > zero {
            let shooter = e.call(SHOOTER, &args![this]).u32();
            let extra = x87_store32(e.call(SHOOTER_VALUE_006D2C20, &args![shooter]).f64());
            e.call(SCALE_BY, &args![velocity, extra]);
        }
        e.call(ADD_VELOCITY, &args![node, velocity, 1u32]);
    });
    true
}

/// Two points of three words each, kept in a stack block as the code's by-value
/// arguments: their addresses.
fn by_value_points(e: &mut Engine, words: [u32; 6], f: impl FnOnce(&mut Engine, u32, u32)) {
    e.with_stack(0x18, |e, block| {
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(block.addr() + 4 * i as u32, *w);
        }
        f(e, block.addr(), block.addr() + 0xc);
    });
}

// Translated from 009b4950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; a contact callback): the references of
/// the contact's two collidables (`+0`, `+4`: `00624020`, `004b59f0`, `004b5a20`,
/// `0044ddc0`, `TESObjectREFR::FindReferenceFor3D`). When the first reference's slot
/// `+0x224` is true the hit is the second (points `006815c0` and `00460140` of `+0x10`, the
/// second times -1.0; the second collidable's words), otherwise when the second's is the
/// hit is the first (both points `00460140` of `+0x10`; the first collidable's words). The
/// projectile's slot `+0x32c` gets the hit, both points, the two words and 0x24. A hit
/// whose slot `+0x100` is true with both objects set marks the contact (`+0x20` = 1).
pub fn fn_009b4950(e: &mut Engine, this: Ptr, contact: u32) {
    let reference_of = |e: &mut Engine, collidable: u32| -> (u32, u32) {
        let word = e.call(COLLIDABLE_WORD, &args![collidable]).u32();
        let object = e.call(COLLIDABLE_OBJECT, &args![word]).u32();
        let owner = if object != 0 {
            e.call(OBJECT_OWNER, &args![object]).u32()
        } else {
            0
        };
        let node = if owner != 0 {
            e.call(ARRAY_COUNT, &args![owner]).u32()
        } else {
            0
        };
        let reference = if node != 0 {
            e.call(FIND_REFERENCE_FOR_3D, &args![node]).u32()
        } else {
            0
        };
        (object, reference)
    };
    let first = e.mem.u32(contact);
    let (first_object, first_reference) = reference_of(e, first);
    let second = e.mem.u32(contact + 0x4);
    let (second_object, second_reference) = reference_of(e, second);
    e.with_stack(0x18, |e, points| {
        let one = points.addr();
        let two = points.addr() + 0xc;
        for word in 0..3 {
            let value = e.mem.u32(ZERO_POINT + 4 * word);
            e.mem.set_u32(one + 4 * word, value);
            e.mem.set_u32(two + 4 * word, value);
        }
        let mut hit = 0u32;
        let mut word_a = 0u32;
        let mut word_b = 0xffff_ffffu32;
        if first_reference != 0 && e.vcall(first_reference, 0x224, &[]).u8() != 0 {
            hit = second_reference;
            let shape = e.mem.u32(contact + 0x10);
            let from = e.call(NODE_ITEM, &args![shape]).u32();
            e.call(COPY_POINT, &args![one, from]);
            let shape = e.mem.u32(contact + 0x10);
            let from = e.call(MEMBER_AT_10, &args![shape]).u32();
            e.call(COPY_POINT, &args![two, from]);
            let minus_one = x87_quiet_bits(e.mem.u32(MINUS_ONE));
            e.call(SCALE_BY, &args![two, minus_one]);
            let collidable = e.mem.u32(contact + 0x4);
            word_a = e.call(COLLIDABLE_WORD, &args![collidable]).u32();
            let collidable = e.mem.u32(contact + 0x4);
            word_b = e.call(NODE_NEXT, &args![collidable]).u32();
        } else if second_reference != 0 && e.vcall(second_reference, 0x224, &[]).u8() != 0 {
            hit = first_reference;
            let shape = e.mem.u32(contact + 0x10);
            let from = e.call(MEMBER_AT_10, &args![shape]).u32();
            e.call(COPY_POINT, &args![one, from]);
            let shape = e.mem.u32(contact + 0x10);
            let from = e.call(MEMBER_AT_10, &args![shape]).u32();
            e.call(COPY_POINT, &args![two, from]);
            let collidable = e.mem.u32(contact);
            word_a = e.call(COLLIDABLE_WORD, &args![collidable]).u32();
            let collidable = e.mem.u32(contact);
            word_b = e.call(NODE_NEXT, &args![collidable]).u32();
        }
        e.vcall(
            this.addr(),
            0x32c,
            &args![hit, one, two, word_a, word_b, 0x24u32],
        );
        if hit != 0 && e.vcall(hit, 0x100, &[]).u8() != 0 && first_object != 0 && second_object != 0
        {
            e.mem.set_u32(contact + 0x20, 1);
        }
    });
}

// Translated from 009b4bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x1c`): with a contact node (`00810660`) on
/// `object`, the projectile's slot `+0x32c` gets the node's reference
/// (`TESObjectREFR::FindReferenceFor3D`), the addresses of the two points passed by value
/// (`first_*`, `second_*`), the shape word (`00621a90`, `00460140`), -1 and 0x24.
pub fn fn_009b4bf0(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    first_x: u32,
    first_y: u32,
    first_z: u32,
    second_x: u32,
    second_y: u32,
    second_z: u32,
) {
    if object == 0 || e.call(CONTACT_NODE, &args![object]).u32() == 0 {
        return;
    }
    let node = e.call(CONTACT_NODE, &args![object]).u32();
    let reference = e.call(FIND_REFERENCE_FOR_3D, &args![node]).u32();
    let shape = e.call(CONTACT_SHAPE, &args![object]).u32();
    let word = e.call(MEMBER_AT_10, &args![shape]).u32();
    by_value_points(
        e,
        [first_x, first_y, first_z, second_x, second_y, second_z],
        |e, first, second| {
            e.vcall(
                this.addr(),
                0x32c,
                &args![reference, first, second, word, 0xffff_ffffu32, 0x24u32],
            );
        },
    );
}

// Translated from 009b4c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): when the shooter (`004181e0`) passes
/// `005de080`, or passes `00975300` with `0045cd80` above 0.0, sets the node's motion
/// (`bhkWorld::SetMotion` with the node, 5, 1, 0, 1).
pub fn fn_009b4c70(e: &mut Engine, this: Ptr) {
    let shooter = e.call(SHOOTER, &args![this]).u32();
    let mut set = e.call(SHOOTER_TEST_005DE080, &args![shooter]).u8() != 0;
    if !set {
        let shooter = e.call(SHOOTER, &args![this]).u32();
        if e.call(SHOOTER_TEST_00975300, &args![shooter]).u8() != 0 {
            let zero = f64::from_bits(e.mem.u64(ZERO));
            let shooter = e.call(SHOOTER, &args![this]).u32();
            set = e.call(SHOOTER_FLOAT, &args![shooter]).f64() > zero;
        }
    }
    if set {
        let node = e.vcall(this.addr(), 0x1d0, &[]).u32();
        e.call(SET_MOTION, &args![node, 5u32, 1u32, 0u32, 1u32]);
    }
}

// Translated from 009b4d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `009c58e0` with `buffer`, then the
/// byte at `+0x150` cleared.
pub fn fn_009b4d10(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(PROJECTILE_INIT_LOAD, &args![this, buffer]);
    e.mem.set_u8(this.addr() + 0x150, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009b_4cf0, fn_009b4cf0(Ptr, Ptr)),
        entry!(0x009b_3f80, fn_009b3f80(Ptr)),
        entry!(0x009b_3fa0, fn_009b3fa0(Ptr, u8) -> Ptr),
        entry!(0x009b_4d40, fn_009b4d40(Ptr) -> bool),
        entry!(0x009b_36f0, fn_009b36f0(Ptr) -> bool),
        entry!(0x009b_3cb0, fn_009b3cb0(Ptr) -> u32),
        entry!(
            0x009b_3cd0,
            fn_009b3cd0(Ptr, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(0x009b_3da0, fn_009b3da0(u32) -> f32),
        entry!(0x009b_3eb0, fn_009b3eb0(Ptr, u32)),
        entry!(0x009b_3f00, fn_009b3f00(Ptr, u32)),
        entry!(0x009b_3f50, fn_009b3f50(Ptr, u32)),
        entry!(
            0x009b_4020,
            fn_009b4020(Ptr, u32, u32, u32, u32, u32, u32, f32, f32) -> Ptr
        ),
        entry!(0x009b_40d0, fn_009b40d0(Ptr)),
        entry!(0x009b_41b0, fn_009b41b0(Ptr, f32)),
        entry!(0x009b_4440, fn_009b4440(Ptr) -> u32),
        entry!(
            0x009b_4460,
            fn_009b4460(Ptr, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(0x009b_4670, fn_009b4670(Ptr, u32)),
        entry!(0x009b_46c0, fn_009b46c0(Ptr)),
        entry!(0x009b_4700, fn_009b4700(Ptr, u32, u8) -> bool),
        entry!(0x009b_4740, fn_009b4740(Ptr) -> bool),
        entry!(0x009b_4950, fn_009b4950(Ptr, u32)),
        entry!(
            0x009b_4bf0,
            fn_009b4bf0(Ptr, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x009b_4c70, fn_009b4c70(Ptr)),
        entry!(0x009b_4d10, fn_009b4d10(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_009b4cf0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LOAD_GAME_TARGET_009C52F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009b_4cf0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LOAD_GAME_TARGET_009C52F0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod b027_tests {
    use super::*;

    fn log_of(e: &mut Engine, addr: u32, words: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log = Some(vec![]);
        e.call(addr, words);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect()
    }

    #[test]
    fn at_009b3f80_forwards() {
        let mut e = Engine::new();
        e.register(PROJECTILE_009BC470, |_, _| Ret::default());
        assert_eq!(
            log_of(&mut e, 0x009b_3f80, &[0x4000]),
            vec![(PROJECTILE_009BC470, vec![0x4000])]
        );
    }

    #[test]
    fn at_009b3fa0_constructs_a_grenade_projectile() {
        let mut e = Engine::new();
        e.register(PROJECTILE_CONSTRUCT, |_, _| Ret::default());
        e.register(SET_FORM_TYPE, |_, _| Ret::default());
        let this = e.mem.alloc(0x154);
        e.mem.set_u8(this + 0x150, 0xff);
        assert_eq!(
            log_of(&mut e, 0x009b_3fa0, &[this, 0x301]),
            vec![
                (PROJECTILE_CONSTRUCT, vec![this, 1]),
                (SET_FORM_TYPE, vec![this, 0x3e]),
            ]
        );
        assert_eq!(e.mem.u32(this), GRENADE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), GRENADE_VTABLE_18);
        assert_eq!(e.mem.u8(this + 0x150), 0);
        assert_eq!(e.call(0x009b_3fa0, &args![this, 0u32]).u32(), this);
    }

    #[test]
    fn at_009b4d40_asks_the_slot_then_00978f10() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x31c / 4];
        slots[0x318 / 4] = 0x0f00_1318;
        e.put_vtable(0x0f00_0000, &slots);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x0f00_0000);
        e.register(0x0f00_1318, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        e.register(OBJECT_00978F10, |_, _| Ret::default());
        assert!(!e.call(0x009b_4d40, &args![this]).bool());
        e.register(OBJECT_00978F10, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert!(e.call(0x009b_4d40, &args![this]).bool());
        let log = log_of(&mut e, 0x009b_4d40, &[this]);
        assert_eq!(log.len(), 2);
        e.register(0x0f00_1318, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let log = log_of(&mut e, 0x009b_4d40, &[this]);
        assert_eq!(log, vec![(0x0f00_1318, vec![this])]);
    }
}

#[cfg(test)]
mod projectile_tests {
    use super::*;

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

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// An object (0x200 bytes) whose vtable has `targets` at the given slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x400);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        let p = e.mem.alloc(0x200);
        e.mem.set_u32(p, table);
        p
    }

    fn constants(e: &mut Engine) {
        e.map(ZERO & !0xfff, 0x1000);
        e.mem.set_u64(ZERO, 0);
        e.mem.set_u64(ONE, 1.0f64.to_bits());
        e.map(ZERO_POINT & !0xfff, 0x1000);
        e.map(PLAYER & !0xfff, 0x1000);
        e.map(FLOAT_MAX & !0xfff, 0x1000);
        e.mem.set_f32(FLOAT_MAX, f32::MAX);
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.mem.set_f32(TWO_FLOAT, 2.0);
        e.map(COUNTED_ARRAY & !0xfff, 0x1000);
    }

    #[test]
    fn fn_009b3cb0_reads_the_word_at_0x11c() {
        let mut e = Engine::new();
        e.register(READ_FIRST_WORD, |e, a| ret(e.mem.u32(a[0])));
        let this = e.mem.alloc(0x120);
        e.mem.set_u32(this + 0x11c, 0x77);
        assert_eq!(e.call(0x009b_3cb0, &args![this]).u32(), 0x77);
    }

    #[test]
    fn fn_009b3cd0_checks_the_impact_list() {
        let mut e = Engine::new();
        e.register(SHOOTER, |_, _| ret(0x55));
        e.register(IMPACT_ALLOWED, |_, _| ret(1));
        // The list: nodes `[entry, next]`; empty when the entry is 0.
        e.register(LIST_IS_EMPTY, |e, a| ret((e.mem.u32(a[0]) == 0) as u32));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(ADD_IMPACT, |_, _| ret(1));
        e.register(0x7100, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        let this = e.mem.alloc(0x100);
        let target = object(&mut e, &[(0x100, 0x7100)]);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, target);
        e.mem.set_u32(this + 0x88, entry);
        let impact = |e: &mut Engine, target: u32| {
            e.call(
                0x009b_3cd0,
                &args![this, target, 2u32, 3u32, 4u32, 5u32, 6u32],
            )
            .bool()
        };
        e.call_log = Some(vec![]);
        assert!(impact(&mut e, target));
        assert!(calls(&e, ADD_IMPACT).is_empty());
        assert_eq!(calls(&e, IMPACT_ALLOWED), vec![vec![target, 4, this, 0x55]]);
        e.mem.set_u32(target + 0x10, 1);
        assert!(!impact(&mut e, target));
        // Not in the list: AddImpact decides.
        let other = object(&mut e, &[(0x100, 0x7100)]);
        e.call_log = Some(vec![]);
        assert!(impact(&mut e, other));
        assert_eq!(
            calls(&e, ADD_IMPACT),
            vec![vec![this, other, 2, 3, 4, 5, 6]]
        );
        e.register(IMPACT_ALLOWED, |_, _| ret(0));
        assert!(!impact(&mut e, other));
    }

    #[test]
    fn fn_009b3da0_finds_the_largest_emitter_size() {
        let mut e = Engine::new();
        constants(&mut e);
        // Kinds from `+0x40`; modifiers at `+0x44` (count) and `+0x48`; a size at `+0x60`;
        // children from slot +0xc (the list at `+0x50`, 0 for none).
        e.register(IS_KIND_OF, |e, a| {
            ret((e.mem.u32(a[1] + 0x40) == a[0]) as u32)
        });
        e.register(MODIFIER_COUNT, |e, a| ret(e.mem.u32(a[0] + 0x44)));
        e.register(MODIFIER_AT, |e, a| ret(e.mem.u32(a[0] + 0x48 + 4 * a[1])));
        e.register(EMITTER_SIZE, |e, a| st0(f64::from(e.mem.f32(a[0] + 0x60))));
        e.register(0x700c, |e, a| ret(e.mem.u32(a[0] + 0x50)));
        e.register(CHILD_COUNT, |e, a| ret(e.mem.u32(a[0])));
        e.register(CHILD_AT, |e, a| ret(e.mem.u32(a[0] + 4 + 4 * a[1])));
        let emitter = |e: &mut Engine, size: f32| {
            let p = object(e, &[]);
            e.mem.set_u32(p + 0x40, RTTI_NI_PSYS_EMITTER);
            e.mem.set_f32(p + 0x60, size);
            p
        };
        let other = object(&mut e, &[]);
        let small = emitter(&mut e, 2.0);
        let large = emitter(&mut e, 9.0);
        let root = object(&mut e, &[(0xc, 0x700c)]);
        e.mem.set_u32(root + 0x40, RTTI_BS_STRIP_PARTICLE_SYSTEM);
        e.mem.set_u32(root + 0x44, 3);
        for (i, m) in [other, small, large].iter().enumerate() {
            e.mem.set_u32(root + 0x48 + 4 * i as u32, *m);
        }
        let child = object(&mut e, &[(0xc, 0x700c)]);
        e.mem.set_u32(child + 0x40, RTTI_BS_STRIP_PARTICLE_SYSTEM);
        e.mem.set_u32(child + 0x44, 1);
        let middle = emitter(&mut e, 3.0);
        e.mem.set_u32(child + 0x48, middle);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, 1);
        e.mem.set_u32(list + 4, child);
        e.mem.set_u32(root + 0x50, list);
        // The root stops at the first non-zero size (2.0); its child gives 3.0.
        assert_eq!(e.call(0x009b_3da0, &args![root]).f32(), 3.0);
        assert_eq!(e.call(0x009b_3da0, &args![0u32]).f32(), 0.0);
        e.mem.set_f32(middle + 0x60, 1.0);
        assert_eq!(e.call(0x009b_3da0, &args![root]).f32(), 2.0);
    }

    #[test]
    fn save_load_and_init_load_handle_the_two_floats() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                PROJECTILE_SAVE_GAME,
                LOAD_GAME_TARGET_009C52F0,
                SAVE_DATA,
                LOAD_DATA,
                PROJECTILE_INIT_LOAD,
            ],
        );
        let this = e.mem.alloc(0x160);
        e.call_log = Some(vec![]);
        e.call(0x009b_3eb0, &args![this, 0x6000u32]);
        assert_eq!(calls(&e, PROJECTILE_SAVE_GAME), vec![vec![this, 0x6000]]);
        assert_eq!(
            calls(&e, SAVE_DATA),
            vec![
                vec![0x6000, this + 0x150, 4, 0],
                vec![0x6000, this + 0x154, 4, 0]
            ]
        );
        e.call(0x009b_3f00, &args![this, 0x6000u32]);
        assert_eq!(
            calls(&e, LOAD_GAME_TARGET_009C52F0),
            vec![vec![this, 0x6000]]
        );
        assert_eq!(
            calls(&e, LOAD_DATA),
            vec![vec![0x6000, this + 0x150, 4], vec![0x6000, this + 0x154, 4]]
        );
        e.mem.write(this + 0x150, &[0xcd; 8]);
        e.call(0x009b_3f50, &args![this, 0x6000u32]);
        assert_eq!(e.mem.u64(this + 0x150), 0);
        e.mem.set_u8(this + 0x150, 0xcd);
        e.call(0x009b_4d10, &args![this, 0x6000u32]);
        assert_eq!(e.mem.u8(this + 0x150), 0);
        assert_eq!(calls(&e, PROJECTILE_INIT_LOAD).len(), 2);
    }

    #[test]
    fn fn_009b4020_constructs_the_grenade_with_a_point() {
        let mut e = Engine::new();
        stub(&mut e, &[PROJECTILE_CONSTRUCT_WITH, SET_FORM_TYPE]);
        let this = e.mem.alloc(0x154);
        e.mem.set_u8(this + 0x150, 0xff);
        e.call_log = Some(vec![]);
        let words = args![
            this,
            1u32,
            2u32,
            3u32,
            4u32,
            5u32,
            6u32,
            0.5f32,
            0x7f80_0003u32
        ];
        assert_eq!(e.call(0x009b_4020, &words).u32(), this);
        assert_eq!(
            calls(&e, PROJECTILE_CONSTRUCT_WITH),
            vec![vec![this, 1, 2, 3, 4, 5, 6, 0.5f32.to_bits(), 0x7fc0_0003]]
        );
        assert_eq!(e.mem.u32(this), GRENADE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), GRENADE_VTABLE_18);
        assert_eq!(e.mem.u8(this + 0x150), 0);
        assert_eq!(calls(&e, SET_FORM_TYPE), vec![vec![this, 0x3e]]);
    }

    #[test]
    fn fn_009b40d0_sets_the_grenade_flags() {
        let mut e = Engine::new();
        stub(&mut e, &[SET_FLAG, PROJECTILE_INITIALIZE]);
        e.register(SHOOTER, |_, _| ret(0x55));
        e.register(HAS_FLAG, |_, _| ret(1));
        e.register(SHOOTER_TEST_004FD360, |_, _| ret(1));
        e.register(SHOOTER_TEST_00975300, |_, _| ret(0));
        e.register(SHOOTER_TEST_005DE080, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x009b_40d0, &args![0x4000u32]);
        let flags: Vec<_> = calls(&e, SET_FLAG).iter().map(|c| (c[1], c[2])).collect();
        assert_eq!(
            flags,
            vec![
                (4, 1),
                (2, 1),
                (0x100, 0),
                (0x2000, 1),
                (0x8000, 1),
                (0x4_0000, 1)
            ]
        );
        assert_eq!(calls(&e, PROJECTILE_INITIALIZE), vec![vec![0x4000]]);
        e.register(HAS_FLAG, |_, _| ret(0));
        e.register(SHOOTER_TEST_005DE080, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x009b_40d0, &args![0x4000u32]);
        let flags: Vec<_> = calls(&e, SET_FLAG).iter().map(|c| (c[1], c[2])).collect();
        assert_eq!(
            flags,
            vec![(4, 1), (0x100, 1), (0x2000, 0), (0x8000, 1), (0x4_0000, 1)]
        );
    }

    fn update_doubles(e: &mut Engine) {
        constants(e);
        e.register(IS_DISABLED, |_, _| ret(0));
        e.register(AGE, |_, _| st0(2.0));
        e.register(NEXT_KEY, |_, _| ret(0x0b));
        e.register(SHOOTER, |_, _| ret(0x55));
        e.register(SHOOTER_WORLD, |_, _| ret(0x66));
        e.register(WATER_AT, |_, a| ret(a[1]));
        e.register(WATER_TEST, |_, _| ret(0));
        e.register(SOUND_NAME, |_, _| ret(0x0a00));
        e.register(AUDIO_INSTANCE, |_, _| ret(0x0b00));
        e.register(SHOOTER_TEST_00975300, |_, _| ret(1));
        e.register(SHOOTER_FLOAT, |_, _| st0(1.0));
        e.register(SHOOTER_TEST_005DE080, |_, _| ret(0));
        e.map(SETTING_011CEE0C & !0xfff, 0x1000);
        e.map(SETTING_011CF534 & !0xfff, 0x1000);
        e.register(SETTING_FLOAT, |_, a| ret(a[0] + 4));
        e.mem.set_f32(SETTING_011CEE0C + 4, 1.5);
        e.mem.set_f32(SETTING_011CF534 + 4, 0.0);
        e.register(PROJECTILE_FORM, |_, _| ret(0x0c00));
        e.map(DATA_HANDLER & !0xfff, 0x1000);
        e.register(DATA_HANDLER_TEST, |_, _| ret(1));
        e.register(HAS_FLAG, |_, _| ret(1));
        stub(
            e,
            &[
                SET_COLLISION_KEY,
                GET_SOUND_HANDLE,
                SOUND_SET_POSITION,
                SOUND_PLAY,
                SOUND_HANDLE_DESTRUCT,
                PROJECTILE_EXPIRE,
                PROJECTILE_DETACH,
                SET_LOCATION_ON_REFERENCE,
                CHECK_EXPLOSION,
            ],
        );
        e.register(0x71d0, |_, _| ret(0x0d00));
        e.register(0x71f4, |_, a| ret(a[0] + 0x40));
    }

    #[test]
    fn fn_009b41b0_lands_plays_expires_and_explodes() {
        let mut e = Engine::new();
        update_doubles(&mut e);
        let this = object(&mut e, &[(0x1d0, 0x71d0), (0x1f4, 0x71f4)]);
        let attached = object(&mut e, &[(0x1f4, 0x71f4)]);
        e.mem.set_u32(this + 0x100, attached);
        e.call_log = Some(vec![]);
        e.call(0x009b_41b0, &args![this, 0.25f32]);
        assert_eq!(calls(&e, SET_COLLISION_KEY), vec![vec![0x0d00, 0x0b, 1, 0]]);
        assert_eq!(e.mem.u8(this + 0x150), 1);
        let handle = calls(&e, GET_SOUND_HANDLE);
        assert_eq!(handle[0][0], 0x0b00);
        assert_eq!(handle[0][2..], [0x0a00, 0x102, 0x66]);
        assert_eq!(calls(&e, SOUND_SET_POSITION)[0][1], this + 0x40);
        assert_eq!(calls(&e, SOUND_PLAY)[0][1], 0);
        // The limit 1.5 is below the age 2.0: expired.
        assert_eq!(calls(&e, PROJECTILE_EXPIRE), vec![vec![this]]);
        assert_eq!(calls(&e, PROJECTILE_DETACH), vec![vec![this]]);
        assert_eq!(
            calls(&e, SET_LOCATION_ON_REFERENCE),
            vec![vec![this, attached + 0x40]]
        );
        assert_eq!(
            calls(&e, CHECK_EXPLOSION),
            vec![vec![this, 0.25f32.to_bits()]]
        );
        // Landed already: no new key; the other setting (0.0) does not expire it.
        e.register(SHOOTER_FLOAT, |_, _| st0(0.0));
        e.mem.set_u32(this + 0x100, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_41b0, &args![this, 0.25f32]);
        assert!(calls(&e, SET_COLLISION_KEY).is_empty());
        assert!(calls(&e, PROJECTILE_EXPIRE).is_empty());
        assert!(calls(&e, PROJECTILE_DETACH).is_empty());
        assert_eq!(calls(&e, CHECK_EXPLOSION).len(), 1);
        // Disabled: nothing at all.
        e.register(IS_DISABLED, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x009b_41b0, &args![this, 0.25f32]);
        assert!(calls(&e, CHECK_EXPLOSION).is_empty());
    }

    #[test]
    fn fn_009b4440_reads_the_attached_reference() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x104);
        e.mem.set_u32(this + 0x100, 0x99);
        assert_eq!(e.call(0x009b_4440, &args![this]).u32(), 0x99);
    }

    fn impact_doubles(e: &mut Engine) {
        constants(e);
        e.mem.set_u32(COUNTED_ARRAY + 8, 4);
        e.register(ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(GET_COLLISION_OBJECT, |_, _| ret(0x0e00));
        e.register(RIGID_BODY, |_, _| ret(0x0f00));
        e.register(BODY_WORD, |_, _| ret(0x31));
        e.register(BODY_TEST, |e, a| ret((e.mem.u32(a[0]) == 0x31) as u32));
        e.map(SCALE_FLOAT & !0xfff, 0x2000);
        e.mem.set_f32(SCALE_FLOAT, 0.9);
        e.mem.set_u64(HUNDRED, 100.0f64.to_bits());
        e.map(TENTH & !0xfff, 0x1000);
        e.mem.set_u64(TENTH, f64::from(0.1f32).to_bits());
        e.register(0x7008, |_, a| ret(if a[1] == 0x23 { 30 } else { 0 }));
        e.register(VECTOR_SET, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret::default()
        });
        e.register(SHOOTER, |_, _| ret(0x55));
        e.register(IMPACT_ALLOWED, |_, _| ret(1));
        e.register(ADD_IMPACT, |_, _| ret(1));
        e.register(LIST_IS_EMPTY, |_, _| ret(1));
        e.register(AGE, |_, _| st0(0.0));
        e.register(0x71d0, |_, _| ret(0x0d00));
        e.register(0x7100, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        stub(e, &[BODY_SET_VECTOR, ADD_VELOCITY]);
    }

    #[test]
    fn fn_009b4460_slows_the_body_and_takes_the_impact() {
        let mut e = Engine::new();
        impact_doubles(&mut e);
        let player = e.mem.alloc(0xa8);
        let owner_table = e.mem.alloc(0x10);
        e.mem.set_u32(owner_table + 8, 0x7008);
        e.mem.set_u32(player + 0xa4, owner_table);
        e.mem.set_u32(PLAYER, player);
        let vector = e.mem.alloc(0x10);
        for (i, v) in [1.0f32, 2.0, 4.0, 8.0].iter().enumerate() {
            e.mem.set_f32(vector + 4 * i as u32, *v);
        }
        e.register_double(BODY_VECTOR, move |_, _| ret(vector));
        let this = object(&mut e, &[(0x1d0, 0x71d0)]);
        let target = object(&mut e, &[(0x100, 0x7100)]);
        e.mem.set_u32(target + 0x10, 1);
        e.call_log = Some(vec![]);
        let words = args![this, target, 2u32, 3u32, 4u32, 5u32, 6u32];
        assert!(e.call(0x009b_4460, &words).bool());
        let set = calls(&e, VECTOR_SET);
        let f = f32::from_bits(set[0][1]);
        assert!((f - 0.73).abs() < 1e-6, "{f}");
        assert_eq!(set[0][2..], [set[0][1], set[0][1], 0]);
        let stored = calls(&e, BODY_SET_VECTOR);
        let multiplied = stored[0][1];
        assert_eq!(e.mem.f32(multiplied + 4), 2.0 * f);
        assert_eq!(e.mem.f32(multiplied + 12), 0.0);
        assert_eq!(calls(&e, ADD_VELOCITY), vec![vec![0x0d00, ZERO_POINT, 1]]);
        // No target: at age 0.0 the bounce runs (its list is empty) and +0x90 is set.
        e.call_log = Some(vec![]);
        let words = args![this, 0u32, 2u32, 3u32, 4u32, 5u32, 6u32];
        assert!(e.call(0x009b_4460, &words).bool());
        assert_eq!(e.mem.u8(this + 0x90), 1);
        assert!(calls(&e, ADD_VELOCITY).is_empty());
        // A non-empty list refuses the impact.
        e.register(LIST_IS_EMPTY, |_, _| ret(0));
        assert!(!e.call(0x009b_4460, &words).bool());
    }

    #[test]
    fn fn_009b4670_multiplies_lane_by_lane() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        for i in 0..4u32 {
            e.mem.set_f32(a + 4 * i, 1.0 + i as f32);
            e.mem.set_f32(b + 4 * i, 0.5);
        }
        e.call(0x009b_4670, &args![a, b]);
        let lanes: Vec<f32> = (0..4).map(|i| e.mem.f32(a + 4 * i)).collect();
        assert_eq!(lanes, vec![0.5, 1.0, 1.5, 2.0]);
    }

    #[test]
    fn fn_009b46c0_and_fn_009b4700_forward() {
        let mut e = Engine::new();
        stub(&mut e, &[SET_LIGHT, 0x730c]);
        e.register(HAS_FLAG, |_, _| ret(0));
        e.register(0x71d0, |_, _| ret(0x0d00));
        e.register(PROJECTILE_009C43E0, |_, _| ret(0x101));
        let this = object(&mut e, &[(0x1d0, 0x71d0), (0x30c, 0x730c)]);
        e.call_log = Some(vec![]);
        e.call(0x009b_46c0, &args![this]);
        assert_eq!(calls(&e, SET_LIGHT), vec![vec![this, 0x0d00, 1]]);
        assert!(e.call(0x009b_4700, &args![this, 7u32, 0x201u32]).bool());
        assert_eq!(calls(&e, PROJECTILE_009C43E0), vec![vec![this, 7, 1]]);
        assert_eq!(calls(&e, 0x730c), vec![vec![this]]);
    }

    fn bounce_doubles(e: &mut Engine) {
        constants(e);
        e.register(LIST_IS_EMPTY, |_, _| ret(0));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.map(PAUSE_OWNER & !0xfff, 0x1000);
        e.register(IS_PAUSED, |_, _| ret(0));
        e.register(NODE_FLOAT, |_, _| st0(0.5));
        e.register(POINT_SCALED, |_, a| ret(a[1]));
        e.register(POINT_ADD, |_, a| ret(a[1]));
        e.register(POINT_SUBTRACT, |_, a| ret(a[1]));
        e.register(FLOAT_TIMES_POINT, |_, a| ret(a[0]));
        e.register(WORLD_ROTATE, |_, a| ret(a[0] + 0x68));
        e.register(DOT, |_, _| st0(0.25));
        e.register(SHOOTER, |_, _| ret(0x55));
        e.register(SHOOTER_VALUE_0069EF80, |_, _| st0(3.0));
        e.register(SHOOTER_VALUE_006D2C20, |_, _| st0(2.0));
        e.register(0x71d0, |_, _| ret(0x0d00));
        stub(
            e,
            &[
                UPDATE_POSITION,
                SET_LOCAL_TRANSLATE,
                POINT_CONSTRUCT,
                NODE_SET_POINT,
                MATRIX_COLUMN,
                SCALE_BY,
                NORMALIZE,
                ADD_VELOCITY,
            ],
        );
    }

    #[test]
    fn fn_009b4740_reflects_and_pushes_the_node() {
        let mut e = Engine::new();
        bounce_doubles(&mut e);
        let this = object(&mut e, &[(0x1d0, 0x71d0)]);
        let impact = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x88, impact);
        e.call_log = Some(vec![]);
        assert!(e.call(0x009b_4740, &args![this]).bool());
        assert_eq!(calls(&e, UPDATE_POSITION), vec![vec![0x0d00, 1, 0]]);
        assert_eq!(calls(&e, POINT_SCALED)[0][..1], [impact + 0x10]);
        assert_eq!(calls(&e, POINT_SCALED)[0][2], 0.5f32.to_bits());
        assert_eq!(calls(&e, POINT_ADD)[0][0], impact + 4);
        let scales: Vec<u32> = calls(&e, SCALE_BY).iter().map(|c| c[1]).collect();
        assert_eq!(
            scales,
            vec![(-1.0f32).to_bits(), 3.0f32.to_bits(), 2.0f32.to_bits()]
        );
        let normalized = calls(&e, NORMALIZE);
        assert_eq!(normalized[1], vec![impact + 0x10]);
        let times = calls(&e, FLOAT_TIMES_POINT);
        assert_eq!(times[0][1], 0.25f32.to_bits());
        assert_eq!(times[0][2], impact + 0x10);
        assert_eq!(times[1][1], 2.0f32.to_bits());
        let pushed = calls(&e, ADD_VELOCITY);
        assert_eq!(pushed[0][0], 0x0d00);
        assert_eq!(pushed[0][2], 1);
        // Paused, or no impact: nothing moves.
        e.register(IS_PAUSED, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x009b_4740, &args![this]).bool());
        assert!(calls(&e, UPDATE_POSITION).is_empty());
        e.register(LIST_IS_EMPTY, |_, _| ret(1));
        assert!(e.call(0x009b_4740, &args![this]).bool());
    }

    #[test]
    fn fn_009b4950_reports_the_hit_reference() {
        let mut e = Engine::new();
        constants(&mut e);
        // A collidable's word is its own `+0`; the object of a word is `word + 0x10`;
        // its owner `+0x20`; the node is the owner's `+8` word; the reference the node's
        // `+0`.
        e.register(COLLIDABLE_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(COLLIDABLE_OBJECT, |_, a| ret(a[0] + 0x10));
        e.register(OBJECT_OWNER, |_, a| ret(a[0] + 0x20));
        e.register(ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(FIND_REFERENCE_FOR_3D, |e, a| ret(e.mem.u32(a[0])));
        e.register(NODE_ITEM, |_, a| ret(a[0] + 0x4));
        e.register(MEMBER_AT_10, |_, a| ret(a[0] + 0x10));
        e.register(NODE_NEXT, |_, a| ret(a[0] + 1));
        stub(&mut e, &[COPY_POINT, SCALE_BY, 0x732c]);
        e.register(0x7224, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(0x7100, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        let reference = |e: &mut Engine| {
            let r = object(e, &[(0x224, 0x7224), (0x100, 0x7100)]);
            let node = e.mem.alloc(4);
            e.mem.set_u32(node, r);
            let word = e.mem.alloc(0x40);
            e.mem.set_u32(word + 0x30 + 8, node);
            let collidable = e.mem.alloc(4);
            e.mem.set_u32(collidable, word);
            (r, word, collidable)
        };
        let (first, _, first_collidable) = reference(&mut e);
        let (second, second_word, second_collidable) = reference(&mut e);
        let contact = e.mem.alloc(0x24);
        e.mem.set_u32(contact, first_collidable);
        e.mem.set_u32(contact + 4, second_collidable);
        e.mem.set_u32(contact + 0x10, 0x3000);
        e.mem.set_u32(first + 0x20, 1);
        e.mem.set_u32(second + 0x24, 1);
        let this = object(&mut e, &[(0x32c, 0x732c)]);
        e.call_log = Some(vec![]);
        e.call(0x009b_4950, &args![this, contact]);
        let reported = calls(&e, 0x732c);
        assert_eq!(reported[0][1], second);
        assert_eq!(reported[0][4], second_word);
        assert_eq!(reported[0][5], second_collidable + 1);
        assert_eq!(reported[0][6], 0x24);
        let copies = calls(&e, COPY_POINT);
        assert_eq!(copies[0], vec![reported[0][2], 0x3004]);
        assert_eq!(copies[1], vec![reported[0][3], 0x3010]);
        assert_eq!(calls(&e, SCALE_BY)[0][1], (-1.0f32).to_bits());
        assert_eq!(e.mem.u32(contact + 0x20), 1);
        // Neither reference asks for it: no hit, the default words.
        e.mem.set_u32(first + 0x20, 0);
        e.mem.set_u32(contact + 0x20, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_4950, &args![this, contact]);
        let reported = calls(&e, 0x732c);
        assert_eq!(reported[0][1], 0);
        assert_eq!(reported[0][4..], [0, 0xffff_ffff, 0x24]);
        assert_eq!(e.mem.u32(contact + 0x20), 0);
    }

    #[test]
    fn fn_009b4bf0_reports_the_contact() {
        let mut e = Engine::new();
        e.register(CONTACT_NODE, |_, a| ret(a[0] + 1));
        e.register(FIND_REFERENCE_FOR_3D, |_, a| ret(a[0] + 0x100));
        e.register(CONTACT_SHAPE, |_, _| ret(0x2000));
        e.register(MEMBER_AT_10, |_, a| ret(a[0] + 0x10));
        e.map(0x7e00, 0x100);
        e.register(0x732c, |e, a| {
            let first = e.mem.bytes(a[2], 0xc);
            let second = e.mem.bytes(a[3], 0xc);
            e.mem.write(0x7e00, &first);
            e.mem.write(0x7e0c, &second);
            Ret::default()
        });
        let this = object(&mut e, &[(0x32c, 0x732c)]);
        e.call_log = Some(vec![]);
        e.call(
            0x009b_4bf0,
            &args![this, 0x4000u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32],
        );
        let reported = calls(&e, 0x732c);
        assert_eq!(reported[0][1], 0x4101);
        assert_eq!(reported[0][4..], [0x2010, 0xffff_ffff, 0x24]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(0x7e00 + 4 * i)).collect();
        assert_eq!(words, vec![1, 2, 3, 4, 5, 6]);
        e.call_log = Some(vec![]);
        e.call(
            0x009b_4bf0,
            &args![this, 0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32],
        );
        assert!(calls(&e, 0x732c).is_empty());
    }

    #[test]
    fn fn_009b4c70_sets_the_motion_when_asked() {
        let mut e = Engine::new();
        constants(&mut e);
        stub(&mut e, &[SET_MOTION]);
        e.register(SHOOTER, |_, _| ret(0x55));
        e.register(SHOOTER_TEST_005DE080, |_, _| ret(0));
        e.register(SHOOTER_TEST_00975300, |_, _| ret(1));
        e.register(SHOOTER_FLOAT, |_, _| st0(0.5));
        e.register(0x71d0, |_, _| ret(0x0d00));
        let this = object(&mut e, &[(0x1d0, 0x71d0)]);
        e.call_log = Some(vec![]);
        e.call(0x009b_4c70, &args![this]);
        assert_eq!(calls(&e, SET_MOTION), vec![vec![0x0d00, 5, 1, 0, 1]]);
        e.register(SHOOTER_FLOAT, |_, _| st0(0.0));
        e.call_log = Some(vec![]);
        e.call(0x009b_4c70, &args![this]);
        assert!(calls(&e, SET_MOTION).is_empty());
        e.register(SHOOTER_TEST_005DE080, |_, _| ret(1));
        e.call(0x009b_4c70, &args![this]);
        assert_eq!(calls(&e, SET_MOTION).len(), 1);
    }

    fn ray_doubles(e: &mut Engine) {
        constants(e);
        e.register(PARENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x1f0)));
        e.register(CELL_WORLD, |e, a| ret(e.mem.u32(a[0])));
        e.register(MATRIX_MULTIPLY, |_, a| ret(a[1]));
        e.register(GET_LOOKING, |_, _| st0(0.5));
        e.register(GET_CHAR_CONTROLLER, |_, _| ret(0));
        e.register(SOURCE_OBJECT, |_, _| ret(0));
        e.register(NEXT_KEY, |_, _| ret(0x0b));
        e.register(READ_FIRST_WORD, |_, _| ret(0));
        e.register(RAY_VALUE_004A4240, |_, _| st0(4.0));
        e.register(SHOOTER, |_, _| ret(0x55));
        e.register(SHOOTER_VALUE_009A7C40, |_, _| st0(3.0));
        e.register(OWN_VALUE_00645CE0, |_, _| st0(2.0));
        e.register(LENGTH_00979260, |_, _| st0(10.0));
        e.register(LENGTH_00508070, |_, _| st0(4.0));
        e.register(POINT_SCALED, |_, a| ret(a[1]));
        e.register(POINT_ADD, |_, a| ret(a[1]));
        e.register(POINT_SUBTRACT, |_, a| ret(a[1]));
        // The hit test confirms only the third ray.
        e.register(RAY_HIT_TEST, |e, _| {
            let n = e.mem.u32(0x7f00);
            e.mem.set_u32(0x7f00, n + 1);
            ret((n == 2) as u32)
        });
        e.map(DEBUG_SETTING & !0xfff, 0x1000);
        e.register(SETTING_BYTE, |_, a| ret(a[0] + 0x10));
        e.register(RAY_HIT, |_, _| ret(1));
        e.register(COLOR_CONSTRUCT, |_, a| ret(a[0]));
        e.register(DEBUG_LINE, |_, _| ret(0x0c00));
        e.map(DEBUG_TIME_SETTING & !0xfff, 0x1000);
        e.register(SETTING_FLOAT, |_, a| ret(a[0] + 4));
        e.map(TES & !0xfff, 0x1000);
        e.map(0x7f00, 0x100);
        e.register(0x7100, |_, _| st0(1.5));
        e.register(0x71f4, |_, a| ret(a[0] + 0x40));
        e.register(0x70c8, |_, _| ret(1));
        stub(
            e,
            &[
                RAY_CONSTRUCT,
                NODE_ITEM,
                MATRIX_ROTATION,
                MAKE_X_ROTATION,
                MATRIX_COLUMN,
                POINT_NORMALIZE,
                FILTER_CONSTRUCT,
                FILTER_SET_LAYER,
                FILTER_SET_GROUP,
                VECTOR_CONSTRUCT,
                RAY_ENDS,
                RAY_SET_FILTER,
                RAY_SET_START,
                RAY_SET_END,
                COLLECTOR_CONSTRUCT,
                RAY_SET_COLLECTOR,
                COLLECTOR_DESTRUCT,
                ADD_TEMP_DEBUG_OBJECT,
            ],
        );
    }

    #[test]
    fn fn_009b36f0_casts_four_rays() {
        let mut e = Engine::new();
        ray_doubles(&mut e);
        let this = object(&mut e, &[(0x2bc, 0x7100), (0x1f4, 0x71f4)]);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(this + 0x40 + 4 * i as u32, *v);
        }
        let world = object(&mut e, &[(0xc8, 0x70c8)]);
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, world);
        e.mem.set_u32(this + 0x1f0, cell);
        let player = e.mem.alloc(0x200);
        e.mem.set_u32(player + 0x1f0, cell);
        e.mem.set_u32(PLAYER, player);
        e.call_log = Some(vec![]);
        assert!(e.call(0x009b_36f0, &args![this]).bool());
        let ends = calls(&e, RAY_ENDS);
        assert_eq!(ends.len(), 1);
        assert_eq!(
            ends[0][1..],
            [
                3,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                1.5f32.to_bits(),
                0.5f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                4.0f32.to_bits()
            ]
        );
        assert_eq!(calls(&e, FILTER_SET_LAYER)[0][1], 6);
        assert_eq!(calls(&e, FILTER_SET_GROUP)[0][1], 0x0b);
        assert_eq!(calls(&e, RAY_SET_END).len(), 4);
        assert_eq!(calls(&e, COLLECTOR_CONSTRUCT).len(), 4);
        assert_eq!(calls(&e, COLLECTOR_DESTRUCT).len(), 4);
        assert_eq!(calls(&e, RAY_HIT_TEST).len(), 4);
        assert_eq!(calls(&e, POINT_SCALED)[0][2], 6.0f32.to_bits());
        assert!(calls(&e, DEBUG_LINE).is_empty());
        // With the debug setting on, each ray is drawn (red: the double says it hit).
        e.mem.set_u8(DEBUG_SETTING + 0x10, 1);
        e.mem.set_u32(0x7f00, 0);
        e.register(RAY_HIT_TEST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x009b_36f0, &args![this]).bool());
        assert_eq!(calls(&e, DEBUG_LINE).len(), 4);
        let color = calls(&e, COLOR_CONSTRUCT);
        assert_eq!(color[0][1..], [1.0f32.to_bits(), 0, 0, 1.0f32.to_bits()]);
        let added = calls(&e, ADD_TEMP_DEBUG_OBJECT);
        assert_eq!(added.len(), 4);
        assert_eq!(added[0][1], 0x0c00);
        // No cell world: false at once.
        e.mem.set_u32(cell, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x009b_36f0, &args![this]).bool());
        assert!(calls(&e, RAY_CONSTRUCT).is_empty());
    }
}

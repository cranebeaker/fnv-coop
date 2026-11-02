//! `fallout/magic/summoncreatureeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `SummonCreatureEffect` (Xbox PDB, 0x74 bytes): an `AssociatedItemEffect` (the
//! `ActiveEffect` base, 0x48 bytes, and `pAssociatedItem` +0x48) plus `pSummonedCreature`
//! +0x4C, `bTargetSet` +0x50, `fAppearTime` +0x54, `AppearLocation` +0x58 and
//! `AppearRotation` +0x64 (`NiPoint3`s), `bPlacement` +0x70 and `bcached` +0x71 (the PC
//! code uses the same offsets). The `ActiveEffect` fields used: `fElapsedSeconds` +4,
//! `pSpell` +8, `pEffect` +0xC, `pTarget` +0x24, `pCaster` +0x28. The compiler's exception
//! frames (`FS:[0]`) and stack-cookie checks are not translated; the functions that have
//! them carry a `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// `SummonCreatureEffect`'s vtable (`0107c2ac`, RTTI `.?AVSummonCreatureEffect@@`).
const SUMMON_CREATURE_EFFECT_VTABLE: u32 = 0x0107_c2ac;
/// `ActiveEffect` fields (Xbox PDB).
const ELAPSED_SECONDS: u32 = 0x4;
const SPELL: u32 = 0x8;
const EFFECT: u32 = 0xc;
const CASTER: u32 = 0x28;
/// `SummonCreatureEffect` fields (Xbox PDB).
const SUMMONED_CREATURE: u32 = 0x4c;
const TARGET_SET: u32 = 0x50;
const APPEAR_TIME: u32 = 0x54;
const APPEAR_LOCATION: u32 = 0x58;
const APPEAR_ROTATION: u32 = 0x64;
const PLACEMENT: u32 = 0x70;
const CACHED: u32 = 0x71;

/// The allocators `00401000` and `00aa13e0` (`cdecl`, one word: the size) and the free
/// `00401030` (`cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
const ALLOCATE_AA13E0: u32 = 0x00aa_13e0;
const FREE: u32 = 0x0040_1030;
/// The `AssociatedItemEffect` constructor `00806c20` (`thiscall`, three words) and
/// `AssociatedItemEffect::CopyData` (engine map, `00806d50`; one word).
const ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR: u32 = 0x0080_6c20;
const ASSOCIATED_ITEM_EFFECT_COPY_DATA: u32 = 0x0080_6d50;
/// The `ActiveEffect` destructor body `0080d930`.
const ACTIVE_EFFECT_DESTRUCTOR: u32 = 0x0080_d930;
/// `006815c0` (`thiscall`): answers `this` (a point's default constructor; also a list
/// node's item slot).
const POINT_DEFAULT: u32 = 0x0068_15c0;
/// Three words at `011f426c` (a point).
const POINT_011F426C: u32 = 0x011f_426c;
/// `0044ddc0` (`thiscall`): the effect's word at +8 (`pSpell`).
const EFFECT_SPELL: u32 = 0x0044_ddc0;
/// `MagicItem::Preload` and `MagicItem::Unload` (engine map, `0040a420`, `0040b800`;
/// `thiscall`, one word).
const MAGIC_ITEM_PRELOAD: u32 = 0x0040_a420;
const MAGIC_ITEM_UNLOAD: u32 = 0x0040_b800;
/// `0059bb30` (`thiscall`): the effect's `pTarget` (+0x24).
const ACTIVE_EFFECT_TARGET: u32 = 0x0059_bb30;
/// `MagicTarget::GetTargetAsActor` (engine map, `00822b40`; `thiscall`).
const TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `00430830` (`thiscall`): the address of the reference's word at +0x24 (its rotation).
const REFERENCE_ROTATION: u32 = 0x0043_0830;
/// The reference's vtable slot +0x1F4 (no stack words): the address of its position.
const SLOT_POSITION: u32 = 0x1f4;
/// `"SpecialIdle_SummonEffect"` (`0107c308`) and `"Hit"` (`0103275c`).
const SPECIAL_IDLE_SUMMON_EFFECT: u32 = 0x0107_c308;
const HIT: u32 = 0x0103_275c;
/// `005b9e80` and `005c53d0` (`cdecl`, one float each; answers in `ST0`).
const ANGLE_005B9E80: u32 = 0x005b_9e80;
const ANGLE_005C53D0: u32 = 0x005c_53d0;
/// `00416870` (`thiscall`, three floats): constructs a point.
const POINT_CONSTRUCT: u32 = 0x0041_6870;
/// `004a3760` (`cdecl`, three words: out point, float, point): the point scaled.
const POINT_SCALED: u32 = 0x004a_3760;
/// The float the direction is scaled by (`0101e6ec`; also the smallest bound size).
const SCALE_0101E6EC: u32 = 0x0101_e6ec;
/// `0084e3a0` (`thiscall`): the word at +0xC; `00825c00` (`thiscall`): the word at +0x14.
const WORD_0C: u32 = 0x0084_e3a0;
const WORD_14: u32 = 0x0082_5c00;
/// `008d6f30` (`thiscall`): the reference's word at +0x40 (its cell).
const REFERENCE_CELL: u32 = 0x008d_6f30;
/// `006890b0` (`bstempeffectparticle.cpp`; `cdecl`, twelve words): makes a temporary
/// particle effect; `00689920` (`thiscall`, one word: a name) and `00490e40` (`thiscall`)
/// on it.
const MAKE_PARTICLE_EFFECT: u32 = 0x0068_90b0;
const PARTICLE_00689920: u32 = 0x0068_9920;
const PARTICLE_00490E40: u32 = 0x0049_0e40;
/// `004b6220` (`cdecl`, three words): a float in `ST0` (the time the creature appears).
const ANIMATION_TIME: u32 = 0x004b_6220;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map name of `008d8520`; the body
/// answers the reference's word at +0x68: its process).
const REFERENCE_PROCESS: u32 = 0x008d_8520;
/// `0045cd60` (`thiscall`): the process's word at +0x28.
const PROCESS_WORD_28: u32 = 0x0045_cd60;
/// The creature's vtable slots: +0x22C (one word, here 0: a byte), +0xC4 (one word, here
/// 1), +0x100 (no stack words: whether it is an actor), +0x1FC (one word, here 0), +0x260
/// (no stack words).
const SLOT_22C: u32 = 0x22c;
const SLOT_C4: u32 = 0xc4;
const SLOT_IS_ACTOR: u32 = 0x100;
const SLOT_1FC: u32 = 0x1fc;
const SLOT_260: u32 = 0x260;
/// The process's vtable slot +0x694 (one word, here 1).
const PROCESS_SLOT_694: u32 = 0x694;
/// `Actor::Kill` (engine map, `0089d900`; `thiscall`, two words).
const ACTOR_KILL: u32 = 0x0089_d900;
/// `HighProcess::FadeAndDelete` (engine map, `008feab0`; `thiscall`, one word).
const FADE_AND_DELETE: u32 = 0x008f_eab0;
/// `CommandEffect::CleanupCommandedActor` (engine map, `0080b9e0`; `cdecl`, three words),
/// `0080b8f0` (`commandeffect.cpp`; `cdecl`, two words) and
/// `CommandEffect::InitCommandedActor` (engine map, `0080b850`; `cdecl`, two words).
const CLEANUP_COMMANDED_ACTOR: u32 = 0x0080_b9e0;
const COMMAND_EFFECT_0080B8F0: u32 = 0x0080_b8f0;
const INIT_COMMANDED_ACTOR: u32 = 0x0080_b850;
/// `ActiveEffect::Dispel` (engine map, `00804210`; `thiscall`, one word).
const ACTIVE_EFFECT_DISPEL: u32 = 0x0080_4210;
/// The player (`PlayerCharacter*` global at `011dea3c`); its `MagicTarget` base is at
/// +0x94.
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_MAGIC_TARGET: u32 = 0x94;
/// The reference `00828b20` measures from (`011dcc58`, set by `00828cb0` before sorting).
const SORT_REFERENCE: u32 = 0x011d_cc58;
/// `qsort` (engine map, `00ec6f20`; `cdecl`, four words).
const QSORT: u32 = 0x00ec_6f20;
/// The floats `00828dc0` starts from (`0101b2d0`, `01013d84`).
const NEAR_0101B2D0: u32 = 0x0101_b2d0;
const FAR_01013D84: u32 = 0x0101_3d84;
/// `00439ef0` (`thiscall` on a point, two words: out point, other point): the difference;
/// `00439e90` (`thiscall` on a point, two words: out point, other point): another
/// combination of the two.
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const POINT_COMBINE: u32 = 0x0043_9e90;
/// `00457990` (`thiscall` on a point): its length in `ST0`.
const POINT_LENGTH: u32 = 0x0045_7990;
/// Nine words at `011a9448` (a 3x3 matrix) that `00828dc0` copies.
const MATRIX_011A9448: u32 = 0x011a_9448;
/// `004a0c90` (`thiscall` on a matrix, one float), `NiMatrix3::Inverse` (engine map,
/// `004b45b0`; one word: the out matrix), `004b3ae0` (`cdecl`, three words: out point,
/// point, matrix), `004a0c10` (`thiscall` on a point).
const MATRIX_FROM_ANGLE: u32 = 0x004a_0c90;
const MATRIX_INVERSE: u32 = 0x004b_45b0;
const MATRIX_TIMES_POINT: u32 = 0x004b_3ae0;
const POINT_004A0C10: u32 = 0x004a_0c10;
/// `00408820` (`cdecl`, one float): a float in `ST0`.
const SIDE_00408820: u32 = 0x0040_8820;
/// The `double` 0.5-like constant `00828dc0` and `00828f00` scale by (`01011588`).
const FACTOR_01011588: u32 = 0x0101_1588;
/// `__RTDynamicCast` (engine map, `00ec43fb`; `cdecl`, five words) and the type
/// descriptors used with it (`01183028` the source type, `01183108` and `011846d4` the
/// targets).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_01183028: u32 = 0x0118_3028;
const TYPE_01183108: u32 = 0x0118_3108;
const TYPE_011846D4: u32 = 0x0118_46d4;
/// `006733e0` (`thiscall`): the effect's word at +0x48 (`pAssociatedItem`).
const ASSOCIATED_ITEM: u32 = 0x0067_33e0;
/// `TESBoundObject::GetBoundSize` (engine map, `0050ebf0`; `thiscall`): a float in `ST0`.
const BOUND_SIZE: u32 = 0x0050_ebf0;
/// The `double`s `00828f00` uses (`010240c0`, `01020758`).
const MINIMUM_010240C0: u32 = 0x0102_40c0;
const LIFT_01020758: u32 = 0x0102_0758;
/// The Havok helpers of `00828f00` (`thiscall` unless noted): `bhkSphereShape` constructor
/// (`0056e950`, two words), filter info (`00622290`, three words), `0056e090`, `00559450`
/// (the first word of a struct), `bhkCharacterProxy::operatorP` (`004ae750`), `0056e120`,
/// `bhkSimpleShapePhantom` constructor (`0056e2d0`, one word), the cell's world
/// (`004543c0`), `bhkWorldObject::Add` (`00c85810`, one word), collector constructors
/// (`00620ff0`, `00621290`), `004a3e00` (`cdecl`, two words), `hkpAllCdPointCollector`
/// query (`00c9eba0`, four words), `00621210` (one word), `005609f0`,
/// `bhkPhantom::Remove` (`00c9f420`), the destructors (`006211b0`, `00c9eec0`,
/// `0056ea90`).
const SPHERE_SHAPE_CONSTRUCT: u32 = 0x0056_e950;
const FILTER_INFO: u32 = 0x0062_2290;
const PHANTOM_INFO_CONSTRUCT: u32 = 0x0056_e090;
const FIRST_WORD: u32 = 0x0055_9450;
const SHAPE_HAVOK_OBJECT: u32 = 0x004a_e750;
const STRUCT_0056E120: u32 = 0x0056_e120;
const PHANTOM_CONSTRUCT: u32 = 0x0056_e2d0;
const CELL_WORLD: u32 = 0x0045_43c0;
const WORLD_OBJECT_ADD: u32 = 0x00c8_5810;
const COLLECTOR_CONSTRUCT: u32 = 0x0062_0ff0;
const VECTOR_CONSTRUCT: u32 = 0x0062_1290;
const VECTOR_FROM_POINT: u32 = 0x004a_3e00;
const PHANTOM_QUERY: u32 = 0x00c9_eba0;
const COLLECTOR_RESULT: u32 = 0x0062_1210;
const RESULT_HAS_HIT: u32 = 0x0056_09f0;
const PHANTOM_REMOVE: u32 = 0x00c9_f420;
const COLLECTOR_DESTROY: u32 = 0x0062_11b0;
const PHANTOM_DESTROY: u32 = 0x00c9_eec0;
const PHANTOM_INFO_DESTROY: u32 = 0x0056_ea90;
/// `TESObjectREFR::GetRelevantWaterHeight` (engine map, `0057b0a0`): a float in `ST0`.
const REFERENCE_WATER_HEIGHT: u32 = 0x0057_b0a0;
/// The pick helpers (`thiscall`): constructor (`004a3c20`), `008c71b0` (one word),
/// `004a39f0` (one word), `004a3f70` (one word), the two ends (`004a3da0`, `004a3eb0`, one
/// word each), `TES::Pick` (engine map, `00458420`, one word, on the object at
/// `011dea10`), `005c53f0` (one word).
const PICK_CONSTRUCT: u32 = 0x004a_3c20;
const PICK_008C71B0: u32 = 0x008c_71b0;
const PICK_004A39F0: u32 = 0x004a_39f0;
const PICK_004A3F70: u32 = 0x004a_3f70;
const PICK_FROM: u32 = 0x004a_3da0;
const PICK_TO: u32 = 0x004a_3eb0;
const TES_PICK: u32 = 0x0045_8420;
const TES: u32 = 0x011d_ea10;
const PICK_RESULT: u32 = 0x005c_53f0;
/// The data handler (`011c3f2c`) and `004698a0` (`tesdatahandler.cpp`; `thiscall`, eight
/// words): places a new reference.
const DATA_HANDLER: u32 = 0x011c_3f2c;
const CREATE_REFERENCE: u32 = 0x0046_98a0;
/// `TESObjectREFR::GetWorldSpace` (engine map, `00575d70`), `::SetLocationOnReference`
/// (`00575830`, one word), `::SetDelete` (`00564930`, one word), `0055d520` (its name).
const REFERENCE_WORLD_SPACE: u32 = 0x0057_5d70;
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
const SET_DELETE: u32 = 0x0056_4930;
const REFERENCE_NAME: u32 = 0x0055_d520;
/// The effect's `EffectSetting` (`007df1d0`), the effect setting's name into a stack
/// string (`004046b0`, one word), the string's destructor (`004037d0`) and the print
/// `005b5e40` (`cdecl`, format and one word).
const EFFECT_SETTING: u32 = 0x007d_f1d0;
const SETTING_NAME: u32 = 0x0040_46b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const PRINT: u32 = 0x005b_5e40;
/// `"GAME EFFECTS: %s effect has no associated creature"` (`0107c37c`),
/// `"GAME EFFECTS: %s summoned a non-actor."` (`0107c354`) and
/// `"DEFAULT: %s effect unable to create reference."` (`0107c324`).
const NO_ASSOCIATED_CREATURE: u32 = 0x0107_c37c;
const SUMMONED_A_NON_ACTOR: u32 = 0x0107_c354;
const UNABLE_TO_CREATE_REFERENCE: u32 = 0x0107_c324;
/// `ValueAndConditionsEffect::GetSaveSize` (engine map, `00806da0`),
/// `AssociatedItemEffect::SaveGame`, `::LoadGame`, `::InitLoadGame` (`00806de0`,
/// `00806e30`, `00806e70`) (`thiscall`, one word each).
const BASE_GET_SAVE_SIZE: u32 = 0x0080_6da0;
const BASE_SAVE_GAME: u32 = 0x0080_6de0;
const BASE_LOAD_GAME: u32 = 0x0080_6e30;
const BASE_INIT_LOAD_GAME: u32 = 0x0080_6e70;
/// The save game object (`011de45c`), `TESSaveLoadGame::SaveNumericID` and
/// `::LoadNumericID` (engine map, `00857a10`, `00857aa0`), its raw write (`008579b0`) and
/// read (`008579e0`) (`thiscall`, two words: address, size).
const SAVE_GAME: u32 = 0x011d_e45c;
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
const SAVE_WRITE: u32 = 0x0085_79b0;
const SAVE_READ: u32 = 0x0085_79e0;
/// `004839c0` (`cdecl`, one word): resolves a loaded form id.
const RESOLVE_FORM: u32 = 0x0048_39c0;
/// The magic target's vtable slot +8 (no stack words): its effect list.
const SLOT_ACTIVE_EFFECT_LIST: u32 = 0x8;
/// List helpers (`thiscall`): empty test (`008256d0`), item slot (`006815c0`, the node
/// itself) and next node (`00726070`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// `004064b0` (`effectitemlist.cpp`; `thiscall` on an `EffectSetting`): a byte;
/// `00403e60` (`thiscall`, one word: a flag): whether the flag is set.
const SETTING_TEST_004064B0: u32 = 0x0040_64b0;
const SETTING_HAS_FLAG: u32 = 0x0040_3e60;
/// `006b9130` (`thiscall`): the effect's float at +4 (`fElapsedSeconds`) in `ST0`.
const EFFECT_ELAPSED: u32 = 0x006b_9130;
/// The float the oldest search starts from (`01015f5c`).
const START_01015F5C: u32 = 0x0101_5f5c;
/// `0043d4d0` (`thiscall`): the address of a setting's value (+4); the setting at
/// `011d171c` is the limit on summoned creatures.
const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
const SUMMON_LIMIT_SETTING: u32 = 0x011d_171c;

/// The effect's target as an actor (`0059bb30`, then `00822b40` when there is a target),
/// or 0.
fn target_actor(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(ACTIVE_EFFECT_TARGET, &args![this]).u32() == 0 {
        return 0;
    }
    let target = e.call(ACTIVE_EFFECT_TARGET, &args![this]).u32();
    e.call(TARGET_AS_ACTOR, &args![target]).u32()
}

/// Copies three words.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// The save game object.
fn save_game(e: &Engine) -> u32 {
    e.global(SAVE_GAME)
}

// Translated from 0080d4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SummonCreatureEffect::CheckTarget` (Xbox PDB; `thiscall`, `RET 4`): true (`EAX` = 1)
/// when the target is not null and is an actor (`00822b40`).
pub fn summon_creature_effect_check_target(e: &mut Engine, _this: Ptr, target: u32) -> bool {
    let actor = if target == 0 {
        0
    } else {
        e.call(TARGET_AS_ACTOR, &args![target]).u32()
    };
    actor != 0
}

// Translated from 00828470 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00828470 - the C++ exception-unwinding frame (FS:[0]) that destroys the base when a member constructor throws is not translated
/// The `SummonCreatureEffect` constructor (by the vtable it stores; unnamed in the engine
/// map; `thiscall`, `RET 0xc`): the `AssociatedItemEffect` constructor `00806c20` with the
/// three words, the vtable, the two points (`006815c0`), `pSummonedCreature`,
/// `bTargetSet` and `fAppearTime` cleared, both points set to the three words at
/// `011f426c`, `bPlacement` and `bcached` cleared. Returns `this`.
pub fn fn_00828470(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    let base = this.addr();
    e.call(
        ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(base, SUMMON_CREATURE_EFFECT_VTABLE);
    e.call(POINT_DEFAULT, &args![base + APPEAR_LOCATION]);
    e.call(POINT_DEFAULT, &args![base + APPEAR_ROTATION]);
    e.mem.set_u32(base + SUMMONED_CREATURE, 0);
    e.mem.set_u8(base + TARGET_SET, 0);
    e.mem.set_f32(base + APPEAR_TIME, 0.0);
    copy_point(e, base + APPEAR_LOCATION, POINT_011F426C);
    copy_point(e, base + APPEAR_ROTATION, POINT_011F426C);
    e.mem.set_u8(base + PLACEMENT, 0);
    e.mem.set_u8(base + CACHED, 0);
    this
}

// Translated from 00828560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SummonCreatureEffect::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, `RET 4`):
/// the destructor `00828590`, then the free `00401030` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn summon_creature_effect_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00828590(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00828590 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00828590 - the C++ exception-unwinding frame (FS:[0]) that destroys the base when the unload throws is not translated
/// The `SummonCreatureEffect` destructor (unnamed in the engine map): the vtable; when
/// `bcached` is set and there is a spell, unloads it (`0040b800` with 1); then the
/// `ActiveEffect` destructor body `0080d930`.
pub fn fn_00828590(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SUMMON_CREATURE_EFFECT_VTABLE);
    if e.mem.u8(base + CACHED) != 0 && e.call(EFFECT_SPELL, &args![this]).u32() != 0 {
        let spell = e.call(EFFECT_SPELL, &args![this]).u32();
        e.call(MAGIC_ITEM_UNLOAD, &args![spell, 1u32]);
    }
    e.call(ACTIVE_EFFECT_DESTRUCTOR, &args![this]);
}

// Translated from 00828610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00828610 - the C++ exception-unwinding frame (FS:[0]) that frees the allocation when the constructor throws is not translated
/// `SummonCreatureEffect::Copy` (Xbox PDB): allocates a `SummonCreatureEffect` (0x74
/// bytes), constructs it with this effect's `pCaster`, `pSpell` and `pEffect` (0 when the
/// allocation failed), copies the `AssociatedItemEffect` data (`00806d50`), then
/// `pSummonedCreature`, `bTargetSet`, `fAppearTime` (`FLD`/`FSTP`), both points and
/// `bPlacement`; when this effect is cached and the copy is not, and there is a spell, the
/// spell is preloaded (`0040a420` with 0); last `bcached`. Returns the copy (the stores go
/// to address 0 when the allocation failed, as in the exe).
pub fn summon_creature_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let block = e.call(ALLOCATE, &args![0x74u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let caster = e.mem.u32(base + CASTER);
        let spell = e.mem.u32(base + SPELL);
        let effect = e.mem.u32(base + EFFECT);
        fn_00828470(e, Ptr::new(block), caster, spell, effect).addr()
    };
    e.call(ASSOCIATED_ITEM_EFFECT_COPY_DATA, &args![this, copy]);
    let creature = e.mem.u32(base + SUMMONED_CREATURE);
    e.mem.set_u32(copy + SUMMONED_CREATURE, creature);
    let target_set = e.mem.u8(base + TARGET_SET);
    e.mem.set_u8(copy + TARGET_SET, target_set);
    let appear_time = x87_quiet_bits(e.mem.u32(base + APPEAR_TIME));
    e.mem.set_u32(copy + APPEAR_TIME, appear_time);
    copy_point(e, copy + APPEAR_LOCATION, base + APPEAR_LOCATION);
    copy_point(e, copy + APPEAR_ROTATION, base + APPEAR_ROTATION);
    let placement = e.mem.u8(base + PLACEMENT);
    e.mem.set_u8(copy + PLACEMENT, placement);
    if e.mem.u8(base + CACHED) != 0
        && e.mem.u8(copy + CACHED) == 0
        && e.call(EFFECT_SPELL, &args![this]).u32() != 0
    {
        let spell = e.call(EFFECT_SPELL, &args![this]).u32();
        e.call(MAGIC_ITEM_PRELOAD, &args![spell, 0u32]);
    }
    let cached = e.mem.u8(base + CACHED);
    e.mem.set_u8(copy + CACHED, cached);
    copy
}

// Translated from 00828760 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00828760 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map (`cdecl`, three words): allocates a 0x74-byte
/// `SummonCreatureEffect` (`00401000`) and constructs it with the three words
/// (`00828470`); returns it, or null when the allocation fails.
pub fn fn_00828760(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x74u32]).u32();
    if block == 0 {
        return 0;
    }
    fn_00828470(e, Ptr::new(block), caster, spell, effect).addr()
}

// Translated from 008287e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SummonCreatureEffect::Start` (Xbox PDB): with a target actor, takes its rotation
/// (`00430830`) and position (slot +0x1F4) as the appear rotation and location, moves the
/// location to the spot `00828cb0` finds (with `pSummonedCreature`) when it finds one,
/// builds the summon particle effect (`006890b0`: the actor's cell, 1.0, the slot +0x14
/// answer of the effect setting's object at +0x18, the direction `(-r2, r1, 0)` scaled by
/// the float at `0101e6ec` (`004a3760`) where `r1 = 005b9e80(-zrot)` and
/// `r2 = 005c53d0(-zrot)`, the location, 1.0, 7, 0), names it
/// `"SpecialIdle_SummonEffect"` (`00689920`), sets `fAppearTime` from its `"Hit"` time
/// (`00490e40`, `004b6220`), and preloads the spell (`0040a420` with 0) and sets `bcached`
/// unless cached already.
pub fn summon_creature_effect_start(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let actor = target_actor(e, this);
    if actor == 0 {
        return;
    }
    let rotation = e.call(REFERENCE_ROTATION, &args![actor]).u32();
    copy_point(e, base + APPEAR_ROTATION, rotation);
    let position = e.vcall(actor, SLOT_POSITION, &args![]).u32();
    copy_point(e, base + APPEAR_LOCATION, position);
    e.with_stack(0x30, |e, frame| {
        let spot = frame.addr();
        let direction = frame.addr() + 0x10;
        let scaled = frame.addr() + 0x20;
        e.call(POINT_DEFAULT, &args![spot]);
        let creature = e.mem.u32(base + SUMMONED_CREATURE);
        if fn_00828cb0(e, this, actor, creature, spot) {
            copy_point(e, base + APPEAR_LOCATION, spot);
        }
        let angle = -x87_load32(e.mem.u32(base + APPEAR_ROTATION + 8));
        let first = e.call(ANGLE_005B9E80, &args![x87_store32(angle)]).f64();
        let first = x87_store32(first);
        let angle = -x87_load32(e.mem.u32(base + APPEAR_ROTATION + 8));
        let second = e.call(ANGLE_005C53D0, &args![x87_store32(angle)]).f64();
        let second = x87_store32(-second);
        let point = e
            .call(POINT_CONSTRUCT, &args![direction, second, first, 0.0f32])
            .u32();
        let scale = x87_quiet_bits(e.global::<u32>(SCALE_0101E6EC));
        e.call(POINT_SCALED, &args![scaled, scale, point]);
        let effect_item = e.call(WORD_0C, &args![this]).u32();
        let setting = e.call(WORD_14, &args![effect_item]).u32();
        let object = setting.wrapping_add(0x18);
        // The exe pushes the two points (copied onto its stack) before the two calls
        // that give the first words.
        let mut points = vec![];
        for i in 0..3 {
            points.push(e.mem.u32(scaled + 4 * i));
        }
        for i in 0..3 {
            points.push(e.mem.u32(base + APPEAR_LOCATION + 4 * i));
        }
        let model = e.vcall(object, 0x14, &args![]).u32();
        let cell = e.call(REFERENCE_CELL, &args![actor]).u32();
        let mut words = vec![cell, 1.0f32.to_bits(), model];
        words.extend(points);
        words.extend([1.0f32.to_bits(), 7, 0]);
        let particle = e.call(MAKE_PARTICLE_EFFECT, &words).u32();
        e.call(
            PARTICLE_00689920,
            &args![particle, SPECIAL_IDLE_SUMMON_EFFECT],
        );
        let animation = e.call(PARTICLE_00490E40, &args![particle]).u32();
        let time = e
            .call(
                ANIMATION_TIME,
                &args![animation, SPECIAL_IDLE_SUMMON_EFFECT, HIT],
            )
            .f64();
        e.mem.set_u32(base + APPEAR_TIME, x87_store32(time));
    });
    if e.mem.u8(base + CACHED) == 0 && e.call(EFFECT_SPELL, &args![this]).u32() != 0 {
        let spell = e.call(EFFECT_SPELL, &args![this]).u32();
        e.call(MAGIC_ITEM_PRELOAD, &args![spell, 0u32]);
        e.mem.set_u8(base + CACHED, 1);
    }
}

// Translated from 008289e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SummonCreatureEffect::Finish` (Xbox PDB): with a summoned creature and a target actor:
/// when the creature has a process (`008d8520`) whose word at +0x28 is null, kills it
/// unless its slot +0x22C (with 0) says otherwise (`0089d900` with 0, 0.0), fades and
/// deletes it (`008feab0`) and cleans up the commanded actor (`0080b9e0` with the target
/// and 1); otherwise cleans up the commanded actor first and then calls the creature's
/// slot +0xC4 with 1. `pSummonedCreature` is cleared. Then, when cached and there is a
/// spell, unloads it (`0040b800` with 0) and clears `bcached`.
pub fn summon_creature_effect_finish(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let actor = target_actor(e, this);
    if e.mem.u32(base + SUMMONED_CREATURE) != 0 && actor != 0 {
        let creature = e.mem.u32(base + SUMMONED_CREATURE);
        let has_free_process = e.call(REFERENCE_PROCESS, &args![creature]).u32() != 0 && {
            let process = e.call(REFERENCE_PROCESS, &args![creature]).u32();
            e.call(PROCESS_WORD_28, &args![process]).u32() == 0
        };
        if has_free_process {
            if !e.vcall(creature, SLOT_22C, &args![0u32]).bool() {
                e.call(ACTOR_KILL, &args![creature, 0u32, 0.0f32]);
            }
            let process = e.call(REFERENCE_PROCESS, &args![creature]).u32();
            e.call(FADE_AND_DELETE, &args![process, creature]);
            e.call(CLEANUP_COMMANDED_ACTOR, &args![creature, actor, 1u32]);
        } else {
            e.call(CLEANUP_COMMANDED_ACTOR, &args![creature, actor, 1u32]);
            e.vcall(creature, SLOT_C4, &args![1u32]);
        }
        e.mem.set_u32(base + SUMMONED_CREATURE, 0);
    }
    if e.mem.u8(base + CACHED) != 0 && e.call(EFFECT_SPELL, &args![this]).u32() != 0 {
        let spell = e.call(EFFECT_SPELL, &args![this]).u32();
        e.call(MAGIC_ITEM_UNLOAD, &args![spell, 0u32]);
        e.mem.set_u8(base + CACHED, 0);
    }
}

// Translated from 00828b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words: two candidate spots): the `qsort`
/// comparison of `00828cb0`. Scores both against the reference at `011dcc58`
/// (`00828dc0`, rounded to `float`): 0 when equal (`FUCOMPP`), 1 when the second scores
/// higher, -1 otherwise (also when unordered): the best spots come first.
pub fn fn_00828b20(e: &mut Engine, first: u32, second: u32) -> i32 {
    let reference: u32 = e.global(SORT_REFERENCE);
    let first_score = fn_00828dc0(e, reference, first).to_bits();
    let reference: u32 = e.global(SORT_REFERENCE);
    let second_score = fn_00828dc0(e, reference, second).to_bits();
    let first_score = x87_load32(first_score);
    let second_score = x87_load32(second_score);
    if second_score == first_score {
        return 0;
    }
    // FLD first; FLD second; FCOMPP; TEST AH,0x41; JNZ: -1 unless the second is greater.
    if second_score > first_score {
        1
    } else {
        -1
    }
}

// Translated from 00828ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SummonCreatureEffect::Update` (Xbox PDB; `thiscall`, `RET 4`, the seconds unread
/// here): dispels the effect (`00804210` with 0) without a target actor, or once placed
/// when the creature is gone or its slot +0x22C (with 0) answers true. Otherwise, once
/// `fAppearTime` is not above `fElapsedSeconds` (ordered) and not yet placed, summons
/// (`008294e0`), limits the player's summons (`00829970`) when the target is the player,
/// and sets `bPlacement`; then, while the creature's slot +0x22C answers false, runs
/// `0080b8f0` with the creature and the target.
pub fn summon_creature_effect_update(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let base = this.addr();
    let actor = target_actor(e, this);
    let dispel = actor == 0
        || (e.mem.u8(base + PLACEMENT) != 0
            && (e.mem.u32(base + SUMMONED_CREATURE) == 0 || {
                let creature = e.mem.u32(base + SUMMONED_CREATURE);
                e.vcall(creature, SLOT_22C, &args![0u32]).bool()
            }));
    if dispel {
        e.call(ACTIVE_EFFECT_DISPEL, &args![this, 0u32]);
        return;
    }
    let elapsed = x87_load32(e.mem.u32(base + ELAPSED_SECONDS));
    let appear = x87_load32(e.mem.u32(base + APPEAR_TIME));
    // FLD elapsed; FLD appear; FCOMPP; TEST AH,0x41; JP: on when below or equal.
    if appear <= elapsed && e.mem.u8(base + PLACEMENT) == 0 {
        summon_creature_effect_do_summon(e, this, actor);
        let player: u32 = e.global(PLAYER);
        if actor == player {
            summon_creature_effect_limit_player_summoned_creatures(e, this);
        }
        e.mem.set_u8(base + PLACEMENT, 1);
    }
    if e.mem.u32(base + SUMMONED_CREATURE) != 0 {
        let creature = e.mem.u32(base + SUMMONED_CREATURE);
        if !e.vcall(creature, SLOT_22C, &args![0u32]).bool() {
            let creature = e.mem.u32(base + SUMMONED_CREATURE);
            e.call(COMMAND_EFFECT_0080B8F0, &args![creature, actor]);
        }
    }
}

// Translated from 00828cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: the actor, the creature, an out
/// point): asks `00828da0` for candidate spots (a count and a block of 0xC-byte points);
/// with none, false. Otherwise sorts them (`qsort` with `00828b20`, the actor at
/// `011dcc58`), takes the first one `00828f00` accepts for the creature into `out`, frees
/// the block (`00401030`) and answers whether one was found.
pub fn fn_00828cb0(e: &mut Engine, this: Ptr, actor: u32, creature: u32, out: u32) -> bool {
    e.with_stack(4, |e, spots| {
        e.mem.set_u32(spots.addr(), 0);
        let count = fn_00828da0(e, this, spots.addr(), actor, creature);
        let block = e.mem.u32(spots.addr());
        if count <= 0 || block == 0 {
            return false;
        }
        e.set_global::<u32>(SORT_REFERENCE, actor);
        e.call(QSORT, &args![block, count, 0xcu32, 0x0082_8b20u32]);
        let mut index = 0;
        while index < count {
            let spot = block.wrapping_add((index as u32).wrapping_mul(0xc));
            if fn_00828f00(e, this, creature, spot) {
                break;
            }
            index += 1;
        }
        if index < count {
            let spot = block.wrapping_add((index as u32).wrapping_mul(0xc));
            copy_point(e, out, spot);
            e.call(FREE, &args![block]);
            return true;
        }
        e.call(FREE, &args![block]);
        false
    })
}

// Translated from 00828da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): the candidate spots; this build
/// answers 0 and writes nothing (its two locals are set to 0 and the first returned).
pub fn fn_00828da0(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> i32 {
    0
}

// Translated from 00828dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words: a reference and a spot): the spot's
/// score in `ST0`. With `near` the float at `0101b2d0` and `far` the one at `01013d84`
/// and `d` the distance from the reference's position (slot +0x1F4, `00439ef0`,
/// `00457990`): a distance factor `d / near` below `near`, `(far - d) / (far - near)` below
/// `far`, 0.0 beyond; times the square of `1 - 00408820(x)` and times
/// `(y + 1) * double at 01011588`, where `(x, y)` is the offset rotated into the
/// reference's frame (the matrix at `011a9448` turned by the reference's z rotation,
/// `004a0c90`, inverted, `004b45b0`, applied, `004b3ae0`, `004a0c10`). x87, rounded to
/// `float` at each store.
pub fn fn_00828dc0(e: &mut Engine, reference: u32, spot: u32) -> f32 {
    let near = x87_quiet_bits(e.global::<u32>(NEAR_0101B2D0));
    let far = x87_quiet_bits(e.global::<u32>(FAR_01013D84));
    e.with_stack(0x90, |e, frame| {
        // Locals at their offsets below the exe's EBP.
        let top = frame.addr() + 0x90;
        let local = |offset: u32| top - offset;
        let position = e.vcall(reference, SLOT_POSITION, &args![]).u32();
        e.call(POINT_DIFFERENCE, &args![spot, local(0x38), position]);
        let distance = e.call(POINT_LENGTH, &args![local(0x38)]).f64();
        let distance = x87_store32(distance);
        // The exe stores 1.0 first; every path below overwrites it.
        let factor = if x87_load32(near) > x87_load32(distance) {
            x87_store32(x87_div(x87_load32(distance), x87_m32(near)))
        } else if x87_load32(far) > x87_load32(distance) {
            let above = x87_sub(x87_load32(far), x87_m32(distance));
            let span = x87_sub(x87_load32(far), x87_m32(near));
            x87_store32(x87_div(above, span))
        } else {
            x87_store32(0.0)
        };
        let rotation = e.call(REFERENCE_ROTATION, &args![reference]).u32();
        copy_point(e, local(0x24), rotation);
        for i in 0..9 {
            let word = e.global::<u32>(MATRIX_011A9448 + 4 * i);
            e.mem.set_u32(local(0x60) + 4 * i, word);
        }
        let z_rotation = x87_quiet_bits(e.mem.u32(local(0x1c)));
        e.call(MATRIX_FROM_ANGLE, &args![local(0x60), z_rotation]);
        e.call(MATRIX_INVERSE, &args![local(0x60), local(0x84)]);
        e.call(
            MATRIX_TIMES_POINT,
            &args![local(0x18), local(0x38), local(0x60)],
        );
        e.call(POINT_004A0C10, &args![local(0x18)]);
        let x = x87_quiet_bits(e.mem.u32(local(0x18)));
        let r = e.call(SIDE_00408820, &args![x]).f64();
        let side = x87_store32(x87_sub(1.0, r));
        let side = x87_store32(x87_mul(x87_load32(side), x87_m32(side)));
        let y = x87_load32(e.mem.u32(local(0x14)));
        let ahead = x87_mul(x87_add(y, 1.0), e.global::<f64>(FACTOR_01011588));
        let ahead = x87_store32(ahead);
        let score = x87_mul(x87_load32(factor), x87_m32(side));
        let score = x87_store32(x87_mul(score, x87_m32(ahead)));
        f32::from_bits(x87_quiet_bits(score))
    })
}

// Translated from 00828f00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00828f00 - the C++ exception-unwinding frame (FS:[0]) and the /GS stack-cookie check (00ec408c) are not translated
/// Unnamed in the engine map (`thiscall`, `RET 8`: the creature and a spot; the frame is
/// 16-byte aligned): whether the creature fits at the spot. Its bound size (`0050ebf0` of
/// `pAssociatedItem` cast through `__RTDynamicCast`; below the `double` at `010240c0` it
/// is the float at `0101e6ec`) makes a sphere shape (`0056e950` with `size * double at
/// 01011588` and 1) in a phantom (`0056e2d0`) added to the target actor's cell's world
/// when there is a cell; a Havok query between two points `size * double at 01011588 +
/// double at 01020758` above the spot (`00439e90`, `004a3e00`, `00c9eba0`) decides: a hit
/// (`00621210`, `005609f0`) means false. Otherwise false also when the spot is below the
/// target actor's water height (`0057b0a0` against its position's z) or when a pick
/// (`00458420` on `011dea10`) from the actor's position to the spot, both raised by the
/// `double` at `010240c0`, hits something (`005c53f0`, `005609f0`); else true. The phantom
/// is removed and the shape deleted (slot 0 with 1) on every path past the query, and the
/// collectors, the phantom and its info are destroyed before returning.
pub fn fn_00828f00(e: &mut Engine, this: Ptr, _unused_1: u32, spot: u32) -> bool {
    let actor = target_actor(e, this);
    let item = e.call(ASSOCIATED_ITEM, &args![this]).u32();
    let bound = e
        .call(
            DYNAMIC_CAST,
            &args![item, 0u32, TYPE_01183028, TYPE_01183108, 0u32],
        )
        .u32();
    let mut size = x87_store32(e.call(BOUND_SIZE, &args![bound]).f64());
    // TEST AH,0x05; JP: replaced only when below (ordered).
    if x87_load32(size) < e.global::<f64>(MINIMUM_010240C0) {
        size = x87_quiet_bits(e.global::<u32>(SCALE_0101E6EC));
    }
    let block = e.call(ALLOCATE_AA13E0, &args![0x14u32]).u32();
    let shape = if block == 0 {
        0
    } else {
        let radius = x87_mul(x87_load32(size), e.global::<f64>(FACTOR_01011588));
        let radius = x87_quiet_bits(x87_store32(radius));
        e.call(SPHERE_SHAPE_CONSTRUCT, &args![block, radius, 1u32])
            .u32()
    };
    e.with_stack(0x9a0, |e, frame| {
        // The exe's EBP is 16-byte aligned; locals sit below it.
        let top = (frame.addr() + 0x990) & !0xf;
        let local = |offset: u32| top - offset;
        let phantom = local(0xb8);
        let info = local(0xa0);
        let first_collector = local(0x470);
        let second_collector = local(0x810);
        e.call(FILTER_INFO, &args![local(0x40), 0x26u32, 0u32, 0u32]);
        e.call(PHANTOM_INFO_CONSTRUCT, &args![info]);
        let filter = e.call(FIRST_WORD, &args![local(0x40)]).u32();
        e.mem.set_u32(info, filter);
        let havok_shape = e.call(SHAPE_HAVOK_OBJECT, &args![shape]).u32();
        e.mem.set_u32(local(0x9c), havok_shape);
        e.call(STRUCT_0056E120, &args![local(0x80)]);
        e.call(PHANTOM_CONSTRUCT, &args![phantom, info]);
        if e.call(REFERENCE_CELL, &args![actor]).u32() != 0 {
            let cell = e.call(REFERENCE_CELL, &args![actor]).u32();
            let world = e.call(CELL_WORLD, &args![cell]).u32();
            e.call(WORLD_OBJECT_ADD, &args![phantom, world]);
        }
        e.call(POINT_DEFAULT, &args![local(0xd0)]);
        e.call(COLLECTOR_CONSTRUCT, &args![first_collector]);
        e.call(COLLECTOR_CONSTRUCT, &args![second_collector]);
        e.call(VECTOR_CONSTRUCT, &args![local(0x830)]);
        for (vector, point, combined, height) in [
            (local(0xd0), local(0x840), local(0x84c), local(0x834)),
            (local(0x830), local(0x85c), local(0x868), local(0x850)),
        ] {
            let lift = x87_add(
                x87_mul(x87_load32(size), e.global::<f64>(FACTOR_01011588)),
                e.global::<f64>(LIFT_01020758),
            );
            e.mem.set_u32(height, x87_store32(lift));
            let lift = x87_quiet_bits(e.mem.u32(height));
            let offset = e
                .call(POINT_CONSTRUCT, &args![point, 0.0f32, 0.0f32, lift])
                .u32();
            let raised = e.call(POINT_COMBINE, &args![spot, combined, offset]).u32();
            e.call(VECTOR_FROM_POINT, &args![vector, raised]);
        }
        e.call(
            PHANTOM_QUERY,
            &args![
                phantom,
                local(0xd0),
                local(0x830),
                second_collector,
                first_collector
            ],
        );
        let result = e
            .call(COLLECTOR_RESULT, &args![first_collector, local(0x869)])
            .u32();
        let fits = if e.call(RESULT_HAS_HIT, &args![result]).bool() {
            e.call(PHANTOM_REMOVE, &args![phantom]);
            if shape != 0 {
                e.vcall(shape, 0, &args![1u32]);
            }
            false
        } else {
            e.call(PHANTOM_REMOVE, &args![phantom]);
            if shape != 0 {
                e.vcall(shape, 0, &args![1u32]);
            }
            let water = e.call(REFERENCE_WATER_HEIGHT, &args![actor]).f64();
            let position = e.vcall(actor, SLOT_POSITION, &args![]).u32();
            let z = x87_load32(e.mem.u32(position + 8));
            // FCOMP; TEST AH,0x05; JP: fails only when below (ordered).
            if z < water {
                false
            } else {
                let pick = local(0x950);
                let pick_word = local(0x954);
                let from = local(0x960);
                let to = local(0x96c);
                e.call(PICK_CONSTRUCT, &args![pick]);
                e.call(PICK_008C71B0, &args![pick_word, 0u32]);
                e.call(PICK_004A39F0, &args![pick_word, 0x26u32]);
                let word = e.mem.u32(pick_word);
                e.call(PICK_004A3F70, &args![pick, word]);
                let position = e.vcall(actor, SLOT_POSITION, &args![]).u32();
                copy_point(e, from, position);
                let raised = x87_add(
                    x87_load32(e.mem.u32(from + 8)),
                    e.global::<f64>(MINIMUM_010240C0),
                );
                e.mem.set_u32(from + 8, x87_store32(raised));
                copy_point(e, to, spot);
                let raised = x87_add(
                    x87_load32(e.mem.u32(to + 8)),
                    e.global::<f64>(MINIMUM_010240C0),
                );
                e.mem.set_u32(to + 8, x87_store32(raised));
                e.call(PICK_FROM, &args![pick, from]);
                e.call(PICK_TO, &args![pick, to]);
                let tes: u32 = e.global(TES);
                e.call(TES_PICK, &args![tes, pick]);
                let result = e
                    .call(PICK_RESULT, &args![local(0x920), local(0x96d)])
                    .u32();
                !e.call(RESULT_HAS_HIT, &args![result]).bool()
            }
        };
        e.call(COLLECTOR_DESTROY, &args![second_collector]);
        e.call(COLLECTOR_DESTROY, &args![first_collector]);
        e.call(PHANTOM_DESTROY, &args![phantom]);
        e.call(PHANTOM_INFO_DESTROY, &args![info]);
        fits
    })
}

// Translated from 008294e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008294e0 - the C++ exception-unwinding frame (FS:[0]) that destroys the stack string when the print throws is not translated
/// `SummonCreatureEffect::DoSummon` (Xbox PDB; `thiscall`, `RET 4`; the word is not read:
/// the target actor is asked again). With a target actor: without a creature form
/// (`pAssociatedItem` cast through `__RTDynamicCast`) it prints the effect setting's name
/// with `"GAME EFFECTS: %s effect has no associated creature"`. Otherwise it places a
/// reference of the form at the appear location and rotation in the actor's cell and
/// world space (`004698a0` on `011c3f2c`). An actor reference becomes
/// `pSummonedCreature`: placed (`00575830`), its slots +0x1FC (with 0) and +0x260 run, it
/// becomes the target's commanded actor (`0080b850`) and its process's slot +0x694 gets 1.
/// Any other reference is reported (`"GAME EFFECTS: %s summoned a non-actor."` with the
/// target's name) and deleted (`00564930` with 1). The exe's last branch (no reference
/// and no form: `"DEFAULT: %s effect unable to create reference."`) cannot run, since a
/// null form returned earlier; it is translated all the same.
pub fn summon_creature_effect_do_summon(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let base = this.addr();
    let actor = target_actor(e, this);
    if actor == 0 {
        return;
    }
    let item = e.call(ASSOCIATED_ITEM, &args![this]).u32();
    let form = e
        .call(
            DYNAMIC_CAST,
            &args![item, 0u32, TYPE_01183028, TYPE_01183108, 0u32],
        )
        .u32();
    if form == 0 {
        print_setting_name(e, this, NO_ASSOCIATED_CREATURE);
        return;
    }
    let world_space = e.call(REFERENCE_WORLD_SPACE, &args![actor]).u32();
    let cell = e.call(REFERENCE_CELL, &args![actor]).u32();
    let handler: u32 = e.global(DATA_HANDLER);
    let reference = e
        .call(
            CREATE_REFERENCE,
            &args![
                handler,
                form,
                base + APPEAR_LOCATION,
                base + APPEAR_ROTATION,
                cell,
                world_space,
                0u32,
                0u32,
                0u32
            ],
        )
        .u32();
    if reference != 0 {
        if e.vcall(reference, SLOT_IS_ACTOR, &args![]).bool() {
            e.mem.set_u32(base + SUMMONED_CREATURE, reference);
            let creature = e.mem.u32(base + SUMMONED_CREATURE);
            e.call(
                SET_LOCATION_ON_REFERENCE,
                &args![creature, base + APPEAR_LOCATION],
            );
            let creature = e.mem.u32(base + SUMMONED_CREATURE);
            e.vcall(creature, SLOT_1FC, &args![0u32]);
            let creature = e.mem.u32(base + SUMMONED_CREATURE);
            e.vcall(creature, SLOT_260, &args![]);
            let creature = e.mem.u32(base + SUMMONED_CREATURE);
            e.call(INIT_COMMANDED_ACTOR, &args![creature, actor]);
            let creature = e.mem.u32(base + SUMMONED_CREATURE);
            let process = e.call(REFERENCE_PROCESS, &args![creature]).u32();
            e.vcall(process, PROCESS_SLOT_694, &args![1u32]);
        } else {
            let name = e.call(REFERENCE_NAME, &args![actor]).u32();
            e.call(PRINT, &args![SUMMONED_A_NON_ACTOR, name]);
            e.call(SET_DELETE, &args![reference, 1u32]);
        }
    } else if form == 0 {
        // Unreachable: a null form returned above.
        print_setting_name(e, this, UNABLE_TO_CREATE_REFERENCE);
    }
}

/// Prints the effect setting's name (`007df1d0`, `004046b0` into a stack string,
/// `00559450`) with `format` (`005b5e40`) and destroys the string (`004037d0`).
fn print_setting_name(e: &mut Engine, this: Ptr, format: u32) {
    e.with_stack(8, |e, text| {
        let setting = e.call(EFFECT_SETTING, &args![this]).u32();
        let string = e.call(SETTING_NAME, &args![setting, text]).u32();
        let name = e.call(FIRST_WORD, &args![string]).u32();
        e.call(PRINT, &args![format, name]);
        e.call(STRING_DESTROY, &args![text]);
    });
}

// Translated from 00829720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SummonCreatureEffect::GetSaveSize` (Xbox PDB; `thiscall`, `RET 4`; 16-bit sums): the
/// base size (`00806da0` with the same word) plus 4 + 1, plus 4 + 0xC + 0xC without a
/// summoned creature.
pub fn summon_creature_effect_get_save_size(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    let mut size = e.call(BASE_GET_SAVE_SIZE, &args![this, argument]).u16();
    size = size.wrapping_add(4);
    size = size.wrapping_add(1);
    if e.mem.u32(this.addr() + SUMMONED_CREATURE) == 0 {
        size = size.wrapping_add(4);
        size = size.wrapping_add(0xc);
        size = size.wrapping_add(0xc);
    }
    size
}

// Translated from 008297a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SummonCreatureEffect::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): the base data
/// (`00806de0`), the creature's form id (its word at +0xC, `0084e3a0`; 0 without one) as a
/// numeric id (`00857a10`), `bTargetSet` (1 byte), and without a creature `fAppearTime`
/// and both points (`008579b0`).
pub fn summon_creature_effect_save_game(e: &mut Engine, this: Ptr, argument: u32) {
    let base = this.addr();
    e.call(BASE_SAVE_GAME, &args![this, argument]);
    e.with_stack(4, |e, id| {
        e.mem.set_u32(id.addr(), 0);
        if e.mem.u32(base + SUMMONED_CREATURE) != 0 {
            let creature = e.mem.u32(base + SUMMONED_CREATURE);
            let form_id = e.call(WORD_0C, &args![creature]).u32();
            e.mem.set_u32(id.addr(), form_id);
        }
        let save = save_game(e);
        e.call(SAVE_NUMERIC_ID, &args![save, id, 4u32]);
    });
    let save = save_game(e);
    e.call(SAVE_WRITE, &args![save, base + TARGET_SET, 1u32]);
    if e.mem.u32(base + SUMMONED_CREATURE) == 0 {
        let save = save_game(e);
        e.call(SAVE_WRITE, &args![save, base + APPEAR_TIME, 4u32]);
        let save = save_game(e);
        e.call(SAVE_WRITE, &args![save, base + APPEAR_LOCATION, 0xcu32]);
        let save = save_game(e);
        e.call(SAVE_WRITE, &args![save, base + APPEAR_ROTATION, 0xcu32]);
    }
}

// Translated from 00829850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SummonCreatureEffect::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): the base data
/// (`00806e30`), the creature's numeric id into `pSummonedCreature` (`00857aa0`, whose
/// byte answer is kept) and `bTargetSet`; with a zero id and a zero answer it also reads
/// `fAppearTime` and both points and clears `bPlacement`, otherwise sets `bPlacement`.
pub fn summon_creature_effect_load_game(e: &mut Engine, this: Ptr, argument: u32) {
    let base = this.addr();
    e.call(BASE_LOAD_GAME, &args![this, argument]);
    e.with_stack(4, |e, id| {
        let save = save_game(e);
        let answer = e.call(LOAD_NUMERIC_ID, &args![save, id, 4u32]).u8();
        let creature = e.mem.u32(id.addr());
        e.mem.set_u32(base + SUMMONED_CREATURE, creature);
        let save = save_game(e);
        e.call(SAVE_READ, &args![save, base + TARGET_SET, 1u32]);
        if creature == 0 && answer == 0 {
            let save = save_game(e);
            e.call(SAVE_READ, &args![save, base + APPEAR_TIME, 4u32]);
            let save = save_game(e);
            e.call(SAVE_READ, &args![save, base + APPEAR_LOCATION, 0xcu32]);
            let save = save_game(e);
            e.call(SAVE_READ, &args![save, base + APPEAR_ROTATION, 0xcu32]);
            e.mem.set_u8(base + PLACEMENT, 0);
        } else {
            e.mem.set_u8(base + PLACEMENT, 1);
        }
    });
}

// Translated from 00829900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SummonCreatureEffect::InitLoadGame` (Xbox PDB; `thiscall`, `RET 4`): the base step
/// (`00806e70`); with a loaded creature id, resolves it (`004839c0`) and casts it through
/// `__RTDynamicCast` (`01183028` to `011846d4`) into `pSummonedCreature`, dispelling the
/// effect (`00804210` with 1) when that gives null.
pub fn summon_creature_effect_init_load_game(e: &mut Engine, this: Ptr, argument: u32) {
    let base = this.addr();
    e.call(BASE_INIT_LOAD_GAME, &args![this, argument]);
    let id = e.mem.u32(base + SUMMONED_CREATURE);
    if id == 0 {
        return;
    }
    let form = e.call(RESOLVE_FORM, &args![id]).u32();
    let creature = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_01183028, TYPE_011846D4, 0u32],
        )
        .u32();
    e.mem.set_u32(base + SUMMONED_CREATURE, creature);
    if e.mem.u32(base + SUMMONED_CREATURE) == 0 {
        e.call(ACTIVE_EFFECT_DISPEL, &args![this, 1u32]);
    }
}

// Translated from 00829970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SummonCreatureEffect::LimitPlayerSummonedCreatures` (Xbox PDB): counts the player's
/// effects (its magic target's list, slot +8, until an empty node) whose effect setting
/// has the byte `004064b0` and flag 0x40000, keeping the one with the largest
/// `fElapsedSeconds` (`006b9130`, starting from the float at `01015f5c`, ordered `<`);
/// when the count is above the integer setting at `011d171c` (`0043d4d0`), dispels that
/// oldest one (`00804210` with 0).
pub fn summon_creature_effect_limit_player_summoned_creatures(e: &mut Engine, _this: Ptr) {
    let player: u32 = e.global(PLAYER);
    let target = player.wrapping_add(PLAYER_MAGIC_TARGET);
    let mut node = e.vcall(target, SLOT_ACTIVE_EFFECT_LIST, &args![]).u32();
    let mut oldest = 0u32;
    let mut best = x87_quiet_bits(e.global::<u32>(START_01015F5C));
    let mut count = 0i32;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        let effect = e.mem.u32(slot);
        let setting = e.call(EFFECT_SETTING, &args![effect]).u32();
        if e.call(SETTING_TEST_004064B0, &args![setting]).bool()
            && e.call(SETTING_HAS_FLAG, &args![setting, 0x40000u32]).bool()
        {
            count = count.wrapping_add(1);
            let elapsed = e.call(EFFECT_ELAPSED, &args![effect]).f64();
            // FLD best; FCOMPP; TEST AH,0x05; JP: on only when best is below (ordered).
            if x87_load32(best) < elapsed {
                oldest = effect;
                best = x87_store32(e.call(EFFECT_ELAPSED, &args![effect]).f64());
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if oldest != 0 {
        let limit = e
            .call(SETTING_INT_VALUE, &args![SUMMON_LIMIT_SETTING])
            .u32();
        if count > e.mem.u32(limit) as i32 {
            e.call(ACTIVE_EFFECT_DISPEL, &args![oldest, 0u32]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0080_d4b0,
            summon_creature_effect_check_target(Ptr, u32) -> bool
        ),
        entry!(0x0082_8470, fn_00828470(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x0082_8560,
            summon_creature_effect_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0082_8590, fn_00828590(Ptr)),
        entry!(0x0082_8610, summon_creature_effect_copy(Ptr) -> u32),
        entry!(0x0082_8760, fn_00828760(u32, u32, u32) -> u32),
        entry!(0x0082_87e0, summon_creature_effect_start(Ptr)),
        entry!(0x0082_89e0, summon_creature_effect_finish(Ptr)),
        entry!(0x0082_8b20, fn_00828b20(u32, u32) -> i32),
        entry!(0x0082_8ba0, summon_creature_effect_update(Ptr, u32)),
        entry!(0x0082_8cb0, fn_00828cb0(Ptr, u32, u32, u32) -> bool),
        entry!(0x0082_8da0, fn_00828da0(Ptr, u32, u32, u32) -> i32),
        entry!(0x0082_8dc0, fn_00828dc0(u32, u32) -> f32),
        entry!(0x0082_8f00, fn_00828f00(Ptr, u32, u32) -> bool),
        entry!(0x0082_94e0, summon_creature_effect_do_summon(Ptr, u32)),
        entry!(
            0x0082_9720,
            summon_creature_effect_get_save_size(Ptr, u32) -> u16
        ),
        entry!(0x0082_97a0, summon_creature_effect_save_game(Ptr, u32)),
        entry!(0x0082_9850, summon_creature_effect_load_game(Ptr, u32)),
        entry!(0x0082_9900, summon_creature_effect_init_load_game(Ptr, u32)),
        entry!(
            0x0082_9970,
            summon_creature_effect_limit_player_summoned_creatures(Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const POSITION_DOUBLE: u32 = 0x0ea0_0000;
    const SLOT_22C_DOUBLE: u32 = 0x0ea0_0100;
    const SLOT_C4_DOUBLE: u32 = 0x0ea0_0200;
    const IS_ACTOR_DOUBLE: u32 = 0x0ea0_0300;
    const SLOT_1FC_DOUBLE: u32 = 0x0ea0_0400;
    const SLOT_260_DOUBLE: u32 = 0x0ea0_0500;
    const PROCESS_694_DOUBLE: u32 = 0x0ea0_0600;
    const MODEL_DOUBLE: u32 = 0x0ea0_0700;
    const DELETE_DOUBLE: u32 = 0x0ea0_0800;
    const LIST_DOUBLE: u32 = 0x0ea0_0900;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
    }

    struct World {
        e: Engine,
        this: u32,
        actor: u32,
        creature: u32,
        position: u32,
    }

    /// A summon effect whose target is an actor (one vtable for every reference and
    /// process: position, +0x22C answering false, +0xC4, +0x100 answering true, +0x1FC,
    /// +0x260, +0x694), a summoned creature with a process, the player elsewhere, globals
    /// mapped and the common callees doubled.
    fn world() -> World {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_3000,
            0x0101_5000,
            0x0101_b000,
            0x0101_e000,
            0x0102_0000,
            0x0102_4000,
            0x011a_9000,
            0x011c_3000,
            0x011d_c000,
            0x011d_e000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global::<f32>(POINT_011F426C, 1.0);
        e.set_global::<f32>(POINT_011F426C + 4, 2.0);
        e.set_global::<f32>(POINT_011F426C + 8, 3.0);
        e.set_global::<f32>(SCALE_0101E6EC, 64.0);
        e.set_global::<u32>(SAVE_GAME, 0x3000);
        e.set_global::<u32>(DATA_HANDLER, 0x3100);
        e.set_global::<u32>(TES, 0x3200);
        let vtable = e.mem.alloc(0x700);
        for (slot, double) in [
            (SLOT_POSITION, POSITION_DOUBLE),
            (SLOT_22C, SLOT_22C_DOUBLE),
            (SLOT_C4, SLOT_C4_DOUBLE),
            (SLOT_IS_ACTOR, IS_ACTOR_DOUBLE),
            (SLOT_1FC, SLOT_1FC_DOUBLE),
            (SLOT_260, SLOT_260_DOUBLE),
            (PROCESS_SLOT_694, PROCESS_694_DOUBLE),
            (0x14, MODEL_DOUBLE),
            (0, DELETE_DOUBLE),
            (SLOT_ACTIVE_EFFECT_LIST, LIST_DOUBLE),
        ] {
            e.mem.set_u32(vtable + slot, double);
        }
        let actor = e.mem.alloc(0x100);
        e.mem.set_u32(actor, vtable);
        let creature = e.mem.alloc(0x100);
        e.mem.set_u32(creature, vtable);
        // The creature's process (`008d8520` answers the creature + 0x80 here).
        e.mem.set_u32(creature + 0x80, vtable);
        let player = e.mem.alloc(0x100);
        e.mem.set_u32(player, vtable);
        e.mem.set_u32(player + PLAYER_MAGIC_TARGET, vtable);
        e.set_global::<u32>(PLAYER, player);
        let this = e.mem.alloc(0x74);
        e.mem.set_u32(this + SPELL, 0x5100);
        e.mem.set_u32(this + EFFECT, 0x5200);
        e.mem.set_u32(this + 0x24, 0x5300);
        e.mem.set_u32(this + CASTER, 0x5400);
        let position = e.mem.alloc(0x10);
        e.mem.set_f32(position, 10.0);
        e.mem.set_f32(position + 4, 20.0);
        e.mem.set_f32(position + 8, 30.0);
        let setting_object = e.mem.alloc(0x20);
        e.mem.set_u32(setting_object + 0x18, vtable);
        e.register(ACTIVE_EFFECT_TARGET, |e, a| {
            e.mem.u32(a[0] + 0x24).into_ret()
        });
        e.register_double(TARGET_AS_ACTOR, move |_, _| actor.into_ret());
        e.register_double(POSITION_DOUBLE, move |_, _| position.into_ret());
        e.register_double(REFERENCE_ROTATION, move |_, _| position.into_ret());
        e.register(SLOT_22C_DOUBLE, |_, _| 0u32.into_ret());
        e.register(IS_ACTOR_DOUBLE, |_, _| 1u32.into_ret());
        e.register(POINT_DEFAULT, |_, a| a[0].into_ret());
        e.register(EFFECT_SPELL, |e, a| e.mem.u32(a[0] + SPELL).into_ret());
        e.register(REFERENCE_PROCESS, |_, a| (a[0] + 0x80).into_ret());
        e.register(PROCESS_WORD_28, |_, _| 0u32.into_ret());
        e.register(WORD_0C, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register_double(WORD_14, move |_, _| setting_object.into_ret());
        e.register(REFERENCE_CELL, |_, _| 0x6600u32.into_ret());
        e.register(ANGLE_005B9E80, |_, _| 0.5f64.into_ret());
        e.register(ANGLE_005C53D0, |_, _| 0.25f64.into_ret());
        e.register(POINT_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register(POINT_SCALED, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[2] + 4 * i) * f32::from_bits(a[1]);
                e.mem.set_f32(a[0] + 4 * i, value);
            }
            a[0].into_ret()
        });
        e.register(MODEL_DOUBLE, |_, _| 0x6700u32.into_ret());
        e.register(MAKE_PARTICLE_EFFECT, |_, _| 0x6800u32.into_ret());
        e.register(PARTICLE_00490E40, |_, _| 0x6900u32.into_ret());
        e.register(ANIMATION_TIME, |_, _| 1.5f64.into_ret());
        e.register(ASSOCIATED_ITEM, |_, _| 0x6a00u32.into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register_double(CREATE_REFERENCE, move |_, _| creature.into_ret());
        e.register(EFFECT_SETTING, |_, _| 0x6b00u32.into_ret());
        e.register(SETTING_NAME, |_, a| a[1].into_ret());
        e.register(FIRST_WORD, |_, _| 0x6c00u32.into_ret());
        e.register(REFERENCE_NAME, |_, _| 0x6d00u32.into_ret());
        e.register(REFERENCE_WORLD_SPACE, |_, _| 0x6e00u32.into_ret());
        quiet(
            &mut e,
            &[
                SLOT_C4_DOUBLE,
                SLOT_1FC_DOUBLE,
                SLOT_260_DOUBLE,
                PROCESS_694_DOUBLE,
                DELETE_DOUBLE,
                ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR,
                ASSOCIATED_ITEM_EFFECT_COPY_DATA,
                ACTIVE_EFFECT_DESTRUCTOR,
                MAGIC_ITEM_PRELOAD,
                MAGIC_ITEM_UNLOAD,
                FREE,
                PARTICLE_00689920,
                ACTOR_KILL,
                FADE_AND_DELETE,
                CLEANUP_COMMANDED_ACTOR,
                COMMAND_EFFECT_0080B8F0,
                INIT_COMMANDED_ACTOR,
                ACTIVE_EFFECT_DISPEL,
                SET_LOCATION_ON_REFERENCE,
                SET_DELETE,
                PRINT,
                STRING_DESTROY,
            ],
        );
        e.call_log = Some(vec![]);
        World {
            e,
            this,
            actor,
            creature,
            position,
        }
    }

    #[test]
    fn check_target_needs_an_actor() {
        let mut w = world();
        assert!(!w.e.call(0x0080_d4b0, &[w.this, 0]).bool());
        assert!(calls(&w.e, TARGET_AS_ACTOR).is_empty());
        assert_eq!(w.e.call(0x0080_d4b0, &[w.this, 0x5300]).u32(), 1);
        w.e.register(TARGET_AS_ACTOR, |_, _| 0u32.into_ret());
        assert_eq!(w.e.call(0x0080_d4b0, &[w.this, 0x5300]).u32(), 0);
    }

    #[test]
    fn constructor_sets_the_fields() {
        let mut w = world();
        let block = w.e.mem.alloc(0x74);
        for i in 0..29 {
            w.e.mem.set_u32(block + 4 * i, 0xffff_ffff);
        }
        assert_eq!(w.e.call(0x0082_8470, &[block, 1, 2, 3]).u32(), block);
        assert_eq!(
            calls(&w.e, ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR),
            vec![vec![block, 1, 2, 3]]
        );
        assert_eq!(w.e.mem.u32(block), SUMMON_CREATURE_EFFECT_VTABLE);
        assert_eq!(w.e.mem.u32(block + SUMMONED_CREATURE), 0);
        assert_eq!(w.e.mem.u8(block + TARGET_SET), 0);
        assert_eq!(w.e.mem.u32(block + APPEAR_TIME), 0);
        assert_eq!(w.e.mem.f32(block + APPEAR_LOCATION + 8), 3.0);
        assert_eq!(w.e.mem.f32(block + APPEAR_ROTATION), 1.0);
        assert_eq!(w.e.mem.u32(block + PLACEMENT), 0xffff_0000);
    }

    #[test]
    fn destructors_unload_when_cached() {
        let mut w = world();
        w.e.mem.set_u8(w.this + CACHED, 1);
        assert_eq!(w.e.call(0x0082_8560, &[w.this, 1]).u32(), w.this);
        assert_eq!(calls(&w.e, MAGIC_ITEM_UNLOAD), vec![vec![0x5100, 1]]);
        assert_eq!(calls(&w.e, ACTIVE_EFFECT_DESTRUCTOR), vec![vec![w.this]]);
        assert_eq!(calls(&w.e, FREE), vec![vec![w.this]]);
        assert_eq!(w.e.mem.u32(w.this), SUMMON_CREATURE_EFFECT_VTABLE);
        w.e.mem.set_u8(w.this + CACHED, 0);
        w.e.call(0x0082_8590, &[w.this]);
        assert_eq!(calls(&w.e, MAGIC_ITEM_UNLOAD).len(), 1);
        assert_eq!(calls(&w.e, ACTIVE_EFFECT_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn copy_and_factory() {
        let mut w = world();
        let block = w.e.mem.alloc(0x74);
        w.e.register_double(ALLOCATE, move |_, _| block.into_ret());
        w.e.mem.set_u32(w.this + SUMMONED_CREATURE, w.creature);
        w.e.mem.set_u8(w.this + TARGET_SET, 1);
        w.e.mem.set_f32(w.this + APPEAR_TIME, 2.5);
        w.e.mem.set_f32(w.this + APPEAR_LOCATION + 4, 7.0);
        w.e.mem.set_f32(w.this + APPEAR_ROTATION + 8, 9.0);
        w.e.mem.set_u8(w.this + PLACEMENT, 1);
        w.e.mem.set_u8(w.this + CACHED, 1);
        assert_eq!(w.e.call(0x0082_8610, &[w.this]).u32(), block);
        assert_eq!(
            calls(&w.e, ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR),
            vec![vec![block, 0x5400, 0x5100, 0x5200]]
        );
        assert_eq!(
            calls(&w.e, ASSOCIATED_ITEM_EFFECT_COPY_DATA),
            vec![vec![w.this, block]]
        );
        assert_eq!(w.e.mem.u32(block + SUMMONED_CREATURE), w.creature);
        assert_eq!(w.e.mem.u8(block + TARGET_SET), 1);
        assert_eq!(w.e.mem.f32(block + APPEAR_TIME), 2.5);
        assert_eq!(w.e.mem.f32(block + APPEAR_LOCATION + 4), 7.0);
        assert_eq!(w.e.mem.f32(block + APPEAR_ROTATION + 8), 9.0);
        assert_eq!(w.e.mem.u8(block + PLACEMENT), 1);
        assert_eq!(w.e.mem.u8(block + CACHED), 1);
        assert_eq!(calls(&w.e, MAGIC_ITEM_PRELOAD), vec![vec![0x5100, 0]]);

        assert_eq!(w.e.call(0x0082_8760, &[4, 5, 6]).u32(), block);
        assert_eq!(calls(&w.e, ALLOCATE), vec![vec![0x74], vec![0x74]]);
        w.e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(w.e.call(0x0082_8760, &[4, 5, 6]).u32(), 0);
        assert_eq!(calls(&w.e, ASSOCIATED_ITEM_EFFECT_CONSTRUCTOR).len(), 2);
    }

    #[test]
    fn start_places_the_summon_effect() {
        let mut w = world();
        w.e.mem.set_u32(w.this + SUMMONED_CREATURE, w.creature);
        w.e.call(0x0082_87e0, &[w.this]);
        // Rotation and location from the actor (both the position double here).
        assert_eq!(w.e.mem.f32(w.this + APPEAR_ROTATION + 8), 30.0);
        assert_eq!(w.e.mem.f32(w.this + APPEAR_LOCATION), 10.0);
        // -zrot goes to both helpers.
        assert_eq!(
            calls(&w.e, ANGLE_005B9E80),
            vec![vec![(-30.0f32).to_bits()]]
        );
        assert_eq!(
            calls(&w.e, ANGLE_005C53D0),
            vec![vec![(-30.0f32).to_bits()]]
        );
        // The direction (-0.25, 0.5, 0) scaled by 64.
        let made = calls(&w.e, MAKE_PARTICLE_EFFECT);
        assert_eq!(made.len(), 1);
        assert_eq!(
            made[0],
            vec![
                0x6600,
                1.0f32.to_bits(),
                0x6700,
                (-16.0f32).to_bits(),
                32.0f32.to_bits(),
                0,
                10.0f32.to_bits(),
                20.0f32.to_bits(),
                30.0f32.to_bits(),
                1.0f32.to_bits(),
                7,
                0
            ]
        );
        assert_eq!(
            calls(&w.e, PARTICLE_00689920),
            vec![vec![0x6800, SPECIAL_IDLE_SUMMON_EFFECT]]
        );
        assert_eq!(
            calls(&w.e, ANIMATION_TIME),
            vec![vec![0x6900, SPECIAL_IDLE_SUMMON_EFFECT, HIT]]
        );
        assert_eq!(w.e.mem.f32(w.this + APPEAR_TIME), 1.5);
        assert_eq!(calls(&w.e, MAGIC_ITEM_PRELOAD), vec![vec![0x5100, 0]]);
        assert_eq!(w.e.mem.u8(w.this + CACHED), 1);
        // Cached already: no second preload.
        w.e.call(0x0082_87e0, &[w.this]);
        assert_eq!(calls(&w.e, MAGIC_ITEM_PRELOAD).len(), 1);
    }

    #[test]
    fn start_without_target_actor_does_nothing() {
        let mut w = world();
        w.e.register(TARGET_AS_ACTOR, |_, _| 0u32.into_ret());
        w.e.call(0x0082_87e0, &[w.this]);
        assert!(calls(&w.e, MAKE_PARTICLE_EFFECT).is_empty());
        assert!(calls(&w.e, MAGIC_ITEM_PRELOAD).is_empty());
        let _ = w.position;
    }

    #[test]
    fn finish_kills_or_cleans_up_the_creature() {
        let mut w = world();
        w.e.mem.set_u32(w.this + SUMMONED_CREATURE, w.creature);
        w.e.mem.set_u8(w.this + CACHED, 1);
        w.e.call(0x0082_89e0, &[w.this]);
        assert_eq!(calls(&w.e, ACTOR_KILL), vec![vec![w.creature, 0, 0]]);
        assert_eq!(
            calls(&w.e, FADE_AND_DELETE),
            vec![vec![w.creature + 0x80, w.creature]]
        );
        assert_eq!(
            calls(&w.e, CLEANUP_COMMANDED_ACTOR),
            vec![vec![w.creature, w.actor, 1]]
        );
        assert_eq!(w.e.mem.u32(w.this + SUMMONED_CREATURE), 0);
        assert_eq!(calls(&w.e, MAGIC_ITEM_UNLOAD), vec![vec![0x5100, 0]]);
        assert_eq!(w.e.mem.u8(w.this + CACHED), 0);

        // A busy process (+0x28): cleanup first, then slot +0xC4 with 1.
        let mut w = world();
        w.e.mem.set_u32(w.this + SUMMONED_CREATURE, w.creature);
        w.e.register(PROCESS_WORD_28, |_, _| 1u32.into_ret());
        w.e.call(0x0082_89e0, &[w.this]);
        assert!(calls(&w.e, ACTOR_KILL).is_empty());
        assert_eq!(calls(&w.e, SLOT_C4_DOUBLE), vec![vec![w.creature, 1]]);
        assert_eq!(calls(&w.e, CLEANUP_COMMANDED_ACTOR).len(), 1);

        // A creature that slot +0x22C says is gone is not killed.
        let mut w = world();
        w.e.mem.set_u32(w.this + SUMMONED_CREATURE, w.creature);
        w.e.register(SLOT_22C_DOUBLE, |_, _| 1u32.into_ret());
        w.e.call(0x0082_89e0, &[w.this]);
        assert!(calls(&w.e, ACTOR_KILL).is_empty());
        assert_eq!(calls(&w.e, FADE_AND_DELETE).len(), 1);
    }

    /// Doubles for `00828dc0`: the distance is the spot's first float, the rotated point
    /// is (0, 1, 0) and `00408820` answers 0.
    fn score_world() -> World {
        let mut w = world();
        w.e.set_global::<f32>(NEAR_0101B2D0, 100.0);
        w.e.set_global::<f32>(FAR_01013D84, 1000.0);
        w.e.set_global::<f64>(FACTOR_01011588, 0.5);
        w.e.register(POINT_DIFFERENCE, |e, a| {
            let x = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], x);
            a[1].into_ret()
        });
        w.e.register(POINT_LENGTH, |e, a| f64::from(e.mem.f32(a[0])).into_ret());
        w.e.register(MATRIX_TIMES_POINT, |e, a| {
            e.mem.set_f32(a[0], 0.0);
            e.mem.set_f32(a[0] + 4, 1.0);
            e.mem.set_f32(a[0] + 8, 0.0);
            a[0].into_ret()
        });
        w.e.register(SIDE_00408820, |_, _| 0.0f64.into_ret());
        quiet(
            &mut w.e,
            &[MATRIX_FROM_ANGLE, MATRIX_INVERSE, POINT_004A0C10],
        );
        w
    }

    #[test]
    fn score_by_distance() {
        let mut w = score_world();
        let spot = w.e.mem.alloc(0xc);
        // Near: 50 / 100; side (1 - 0)^2; ahead (1 + 1) * 0.5.
        w.e.mem.set_f32(spot, 50.0);
        assert_eq!(w.e.call(0x0082_8dc0, &[w.actor, spot]).f64(), 0.5);
        // Between: (1000 - 550) / 900.
        w.e.mem.set_f32(spot, 550.0);
        assert_eq!(w.e.call(0x0082_8dc0, &[w.actor, spot]).f64(), 0.5);
        // Beyond: 0.
        w.e.mem.set_f32(spot, 2000.0);
        assert_eq!(w.e.call(0x0082_8dc0, &[w.actor, spot]).f64(), 0.0);
        // The matrix at 011a9448 and the z rotation reach the helpers.
        let angles = calls(&w.e, MATRIX_FROM_ANGLE);
        assert_eq!(angles[0][1], 30.0f32.to_bits());
    }

    #[test]
    fn comparison_orders_best_first() {
        let mut w = score_world();
        w.e.set_global::<u32>(SORT_REFERENCE, w.actor);
        let low = w.e.mem.alloc(0xc);
        w.e.mem.set_f32(low, 10.0);
        let high = w.e.mem.alloc(0xc);
        w.e.mem.set_f32(high, 90.0);
        assert_eq!(w.e.call(0x0082_8b20, &[low, high]).i32(), 1);
        assert_eq!(w.e.call(0x0082_8b20, &[high, low]).i32(), -1);
        assert_eq!(w.e.call(0x0082_8b20, &[low, low]).i32(), 0);
    }

    #[test]
    fn spots_are_never_found_in_this_build() {
        let mut w = world();
        let out = w.e.mem.alloc(0xc);
        w.e.mem.set_u32(out, 0x1234);
        assert!(!w
            .e
            .call(0x0082_8cb0, &[w.this, w.actor, w.creature, out])
            .bool());
        assert_eq!(w.e.mem.u32(out), 0x1234);
        assert!(calls(&w.e, QSORT).is_empty());
        assert_eq!(w.e.call(0x0082_8da0, &[w.this, out, 1, 2]).i32(), 0);
    }

    /// Doubles for `00828f00`'s Havok and pick helpers; `hit` and `pick_hit` choose the
    /// two `005609f0` answers.
    fn fit_world(hit: bool, pick_hit: bool, water: f64) -> World {
        let mut w = world();
        w.e.set_global::<f64>(MINIMUM_010240C0, 0.0);
        w.e.set_global::<f64>(FACTOR_01011588, 0.5);
        w.e.set_global::<f64>(LIFT_01020758, 4.0);
        w.e.register(BOUND_SIZE, |_, _| 20.0f64.into_ret());
        let shape = w.e.mem.alloc(0x14);
        let shape_vtable = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(shape_vtable, DELETE_DOUBLE);
        w.e.mem.set_u32(shape, shape_vtable);
        w.e.register_double(ALLOCATE_AA13E0, move |_, _| shape.into_ret());
        w.e.register(SPHERE_SHAPE_CONSTRUCT, |_, a| a[0].into_ret());
        w.e.register(FIRST_WORD, |_, _| 0x7100u32.into_ret());
        w.e.register(SHAPE_HAVOK_OBJECT, |_, _| 0x7200u32.into_ret());
        w.e.register(CELL_WORLD, |_, _| 0x7300u32.into_ret());
        w.e.register(POINT_COMBINE, |_, a| a[1].into_ret());
        w.e.register(COLLECTOR_RESULT, |_, _| 0x111u32.into_ret());
        w.e.register(PICK_RESULT, |_, _| 0x222u32.into_ret());
        w.e.register_double(RESULT_HAS_HIT, move |_, a| {
            let answer = if a[0] == 0x111 { hit } else { pick_hit };
            u32::from(answer).into_ret()
        });
        w.e.register_double(REFERENCE_WATER_HEIGHT, move |_, _| water.into_ret());
        quiet(
            &mut w.e,
            &[
                FILTER_INFO,
                PHANTOM_INFO_CONSTRUCT,
                STRUCT_0056E120,
                PHANTOM_CONSTRUCT,
                WORLD_OBJECT_ADD,
                COLLECTOR_CONSTRUCT,
                VECTOR_CONSTRUCT,
                VECTOR_FROM_POINT,
                PHANTOM_QUERY,
                PHANTOM_REMOVE,
                COLLECTOR_DESTROY,
                PHANTOM_DESTROY,
                PHANTOM_INFO_DESTROY,
                PICK_CONSTRUCT,
                PICK_008C71B0,
                PICK_004A39F0,
                PICK_004A3F70,
                PICK_FROM,
                PICK_TO,
                TES_PICK,
            ],
        );
        w
    }

    fn assert_cleaned_up(w: &World) {
        assert_eq!(calls(&w.e, PHANTOM_REMOVE).len(), 1);
        assert_eq!(calls(&w.e, DELETE_DOUBLE).len(), 1);
        assert_eq!(calls(&w.e, COLLECTOR_DESTROY).len(), 2);
        assert_eq!(calls(&w.e, PHANTOM_DESTROY).len(), 1);
        assert_eq!(calls(&w.e, PHANTOM_INFO_DESTROY).len(), 1);
    }

    #[test]
    fn fits_when_nothing_is_in_the_way() {
        let mut w = fit_world(false, false, 0.0);
        let spot = w.e.mem.alloc(0xc);
        w.e.mem.set_f32(spot + 8, 5.0);
        assert!(w.e.call(0x0082_8f00, &[w.this, w.creature, spot]).bool());
        // Radius 20 * 0.5 = 10; lift 10 + 4 = 14 above the spot, twice.
        let radius = calls(&w.e, SPHERE_SHAPE_CONSTRUCT);
        assert_eq!(radius[0][1..], [10.0f32.to_bits(), 1]);
        let lifts = calls(&w.e, POINT_CONSTRUCT);
        assert_eq!(lifts.len(), 2);
        assert!(lifts.iter().all(|l| l[1..] == [0, 0, 14.0f32.to_bits()]));
        assert_eq!(calls(&w.e, WORLD_OBJECT_ADD)[0][1], 0x7300);
        // The pick runs from the actor (z 30) to the spot (z 5).
        let from = calls(&w.e, PICK_FROM)[0][1];
        let to = calls(&w.e, PICK_TO)[0][1];
        assert_eq!(w.e.mem.f32(from + 8), 30.0);
        assert_eq!(w.e.mem.f32(to + 8), 5.0);
        assert_eq!(calls(&w.e, TES_PICK)[0][0], 0x3200);
        assert_cleaned_up(&w);
    }

    #[test]
    fn does_not_fit_on_a_hit_below_water_or_behind_something() {
        let mut w = fit_world(true, false, 0.0);
        let spot = w.e.mem.alloc(0xc);
        assert!(!w.e.call(0x0082_8f00, &[w.this, w.creature, spot]).bool());
        assert!(calls(&w.e, REFERENCE_WATER_HEIGHT).is_empty());
        assert_cleaned_up(&w);

        let mut w = fit_world(false, false, 31.0);
        assert!(!w.e.call(0x0082_8f00, &[w.this, w.creature, spot]).bool());
        assert!(calls(&w.e, TES_PICK).is_empty());
        assert_cleaned_up(&w);

        let mut w = fit_world(false, true, 0.0);
        assert!(!w.e.call(0x0082_8f00, &[w.this, w.creature, spot]).bool());
        assert_cleaned_up(&w);
    }

    #[test]
    fn small_bounds_use_the_minimum() {
        let mut w = fit_world(false, false, 0.0);
        w.e.set_global::<f64>(MINIMUM_010240C0, 30.0);
        let spot = w.e.mem.alloc(0xc);
        w.e.call(0x0082_8f00, &[w.this, w.creature, spot]);
        let radius = calls(&w.e, SPHERE_SHAPE_CONSTRUCT);
        assert_eq!(radius[0][1], 32.0f32.to_bits());
    }

    #[test]
    fn update_dispels_or_summons() {
        // No target actor: dispel.
        let mut w = world();
        w.e.register(TARGET_AS_ACTOR, |_, _| 0u32.into_ret());
        w.e.call(0x0082_8ba0, &[w.this, 0]);
        assert_eq!(calls(&w.e, ACTIVE_EFFECT_DISPEL), vec![vec![w.this, 0]]);

        // Placed but the creature is gone: dispel.
        let mut w = world();
        w.e.mem.set_u8(w.this + PLACEMENT, 1);
        w.e.call(0x0082_8ba0, &[w.this, 0]);
        assert_eq!(calls(&w.e, ACTIVE_EFFECT_DISPEL).len(), 1);

        // Not yet time to appear: only the commanded actor step with a creature.
        let mut w = world();
        w.e.mem.set_f32(w.this + APPEAR_TIME, 2.0);
        w.e.mem.set_f32(w.this + ELAPSED_SECONDS, 1.0);
        w.e.mem.set_u32(w.this + SUMMONED_CREATURE, w.creature);
        w.e.call(0x0082_8ba0, &[w.this, 0]);
        assert!(calls(&w.e, CREATE_REFERENCE).is_empty());
        assert_eq!(
            calls(&w.e, COMMAND_EFFECT_0080B8F0),
            vec![vec![w.creature, w.actor]]
        );

        // Time to appear: summon, then the commanded actor step.
        let mut w = world();
        w.e.mem.set_f32(w.this + APPEAR_TIME, 1.0);
        w.e.mem.set_f32(w.this + ELAPSED_SECONDS, 1.0);
        w.e.call(0x0082_8ba0, &[w.this, 0]);
        assert_eq!(calls(&w.e, CREATE_REFERENCE).len(), 1);
        assert_eq!(w.e.mem.u8(w.this + PLACEMENT), 1);
        assert_eq!(w.e.mem.u32(w.this + SUMMONED_CREATURE), w.creature);
        assert_eq!(calls(&w.e, COMMAND_EFFECT_0080B8F0).len(), 1);
        assert!(calls(&w.e, LIST_DOUBLE).is_empty());
    }

    #[test]
    fn do_summon_places_an_actor() {
        let mut w = world();
        w.e.call(0x0082_94e0, &[w.this, 0]);
        assert_eq!(
            calls(&w.e, CREATE_REFERENCE),
            vec![vec![
                0x3100,
                0x6a00,
                w.this + APPEAR_LOCATION,
                w.this + APPEAR_ROTATION,
                0x6600,
                0x6e00,
                0,
                0,
                0
            ]]
        );
        assert_eq!(w.e.mem.u32(w.this + SUMMONED_CREATURE), w.creature);
        assert_eq!(
            calls(&w.e, SET_LOCATION_ON_REFERENCE),
            vec![vec![w.creature, w.this + APPEAR_LOCATION]]
        );
        assert_eq!(calls(&w.e, SLOT_1FC_DOUBLE), vec![vec![w.creature, 0]]);
        assert_eq!(calls(&w.e, SLOT_260_DOUBLE), vec![vec![w.creature]]);
        assert_eq!(
            calls(&w.e, INIT_COMMANDED_ACTOR),
            vec![vec![w.creature, w.actor]]
        );
        assert_eq!(
            calls(&w.e, PROCESS_694_DOUBLE),
            vec![vec![w.creature + 0x80, 1]]
        );
    }

    #[test]
    fn do_summon_reports_problems() {
        // No creature form.
        let mut w = world();
        w.e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        w.e.call(0x0082_94e0, &[w.this, 0]);
        assert_eq!(
            calls(&w.e, PRINT),
            vec![vec![NO_ASSOCIATED_CREATURE, 0x6c00]]
        );
        assert_eq!(calls(&w.e, STRING_DESTROY).len(), 1);
        assert!(calls(&w.e, CREATE_REFERENCE).is_empty());

        // A non-actor reference is deleted.
        let mut w = world();
        w.e.register(IS_ACTOR_DOUBLE, |_, _| 0u32.into_ret());
        w.e.call(0x0082_94e0, &[w.this, 0]);
        assert_eq!(calls(&w.e, PRINT), vec![vec![SUMMONED_A_NON_ACTOR, 0x6d00]]);
        assert_eq!(calls(&w.e, SET_DELETE), vec![vec![w.creature, 1]]);
        assert_eq!(w.e.mem.u32(w.this + SUMMONED_CREATURE), 0);

        // No reference made: nothing more.
        let mut w = world();
        w.e.register(CREATE_REFERENCE, |_, _| 0u32.into_ret());
        w.e.call(0x0082_94e0, &[w.this, 0]);
        assert!(calls(&w.e, PRINT).is_empty());
    }

    #[test]
    fn save_size_save_load_and_init() {
        let mut w = world();
        w.e.register(BASE_GET_SAVE_SIZE, |_, _| 0x10u32.into_ret());
        assert_eq!(w.e.call(0x0082_9720, &[w.this, 9]).u32(), 0x10 + 5 + 0x1c);
        w.e.mem.set_u32(w.this + SUMMONED_CREATURE, w.creature);
        assert_eq!(w.e.call(0x0082_9720, &[w.this, 9]).u32(), 0x15);

        let probe = w.e.mem.alloc(4);
        w.e.mem.set_u32(w.creature + 0xc, 0x00ab_cdef);
        w.e.register_double(SAVE_NUMERIC_ID, move |e, a| {
            let id = e.mem.u32(a[1]);
            e.mem.set_u32(probe, id);
            Ret::default()
        });
        quiet(&mut w.e, &[BASE_SAVE_GAME, SAVE_WRITE]);
        w.e.call(0x0082_97a0, &[w.this, 9]);
        assert_eq!(w.e.mem.u32(probe), 0x00ab_cdef);
        assert_eq!(
            calls(&w.e, SAVE_WRITE),
            vec![vec![0x3000, w.this + TARGET_SET, 1]]
        );
        w.e.mem.set_u32(w.this + SUMMONED_CREATURE, 0);
        w.e.call(0x0082_97a0, &[w.this, 9]);
        assert_eq!(w.e.mem.u32(probe), 0);
        assert_eq!(calls(&w.e, SAVE_WRITE).len(), 5);
    }

    #[test]
    fn load_and_init_load() {
        let mut w = world();
        quiet(&mut w.e, &[BASE_LOAD_GAME, SAVE_READ, BASE_INIT_LOAD_GAME]);
        w.e.register(LOAD_NUMERIC_ID, |e, a| {
            e.mem.set_u32(a[1], 0x0012_3456);
            1u32.into_ret()
        });
        w.e.call(0x0082_9850, &[w.this, 9]);
        assert_eq!(w.e.mem.u32(w.this + SUMMONED_CREATURE), 0x0012_3456);
        assert_eq!(w.e.mem.u8(w.this + PLACEMENT), 1);
        assert_eq!(calls(&w.e, SAVE_READ).len(), 1);

        w.e.register(LOAD_NUMERIC_ID, |e, a| {
            e.mem.set_u32(a[1], 0);
            0u32.into_ret()
        });
        w.e.call(0x0082_9850, &[w.this, 9]);
        assert_eq!(w.e.mem.u8(w.this + PLACEMENT), 0);
        assert_eq!(calls(&w.e, SAVE_READ).len(), 5);

        // Init: no id, nothing; an id that does not resolve dispels.
        w.e.call(0x0082_9900, &[w.this, 9]);
        assert!(calls(&w.e, RESOLVE_FORM).is_empty());
        w.e.mem.set_u32(w.this + SUMMONED_CREATURE, 0x0012_3456);
        w.e.register(RESOLVE_FORM, |_, _| 0u32.into_ret());
        w.e.call(0x0082_9900, &[w.this, 9]);
        assert_eq!(
            calls(&w.e, DYNAMIC_CAST),
            vec![vec![0, 0, TYPE_01183028, TYPE_011846D4, 0]]
        );
        assert_eq!(calls(&w.e, ACTIVE_EFFECT_DISPEL), vec![vec![w.this, 1]]);
        w.e.mem.set_u32(w.this + SUMMONED_CREATURE, 0x0012_3456);
        w.e.register(RESOLVE_FORM, |_, _| 0x7777u32.into_ret());
        w.e.call(0x0082_9900, &[w.this, 9]);
        assert_eq!(w.e.mem.u32(w.this + SUMMONED_CREATURE), 0x7777);
        assert_eq!(calls(&w.e, ACTIVE_EFFECT_DISPEL).len(), 1);
    }

    #[test]
    fn limit_dispels_the_oldest_over_the_limit() {
        let mut w = world();
        w.e.set_global::<f32>(START_01015F5C, 0.0);
        // Three summon effects with elapsed 1, 3, 2, then an empty node.
        let effects: Vec<u32> = (0..3).map(|_| w.e.mem.alloc(8)).collect();
        for (effect, elapsed) in effects.iter().zip([1.0f32, 3.0, 2.0]) {
            w.e.mem.set_f32(effect + 4, elapsed);
        }
        let mut next = 0;
        let mut first = 0;
        for &effect in effects.iter().rev() {
            let node = w.e.mem.alloc(8);
            w.e.mem.set_u32(node, effect);
            w.e.mem.set_u32(node + 4, next);
            next = node;
            first = node;
        }
        w.e.register_double(LIST_DOUBLE, move |_, _| first.into_ret());
        w.e.register(LIST_IS_EMPTY, |_, _| 0u32.into_ret());
        w.e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        w.e.register(SETTING_TEST_004064B0, |_, _| 1u32.into_ret());
        w.e.register(SETTING_HAS_FLAG, |_, _| 1u32.into_ret());
        w.e.register(EFFECT_ELAPSED, |e, a| {
            f64::from(e.mem.f32(a[0] + 4)).into_ret()
        });
        let limit = w.e.mem.alloc(4);
        w.e.mem.set_u32(limit, 2);
        w.e.register_double(SETTING_INT_VALUE, move |_, _| limit.into_ret());
        w.e.call(0x0082_9970, &[w.this]);
        assert_eq!(calls(&w.e, ACTIVE_EFFECT_DISPEL), vec![vec![effects[1], 0]]);
        assert_eq!(
            calls(&w.e, SETTING_HAS_FLAG),
            vec![vec![0x6b00, 0x40000]; 3]
        );
        // At the limit nothing is dispelled.
        w.e.mem.set_u32(limit, 3);
        w.e.call(0x0082_9970, &[w.this]);
        assert_eq!(calls(&w.e, ACTIVE_EFFECT_DISPEL).len(), 1);
    }
}

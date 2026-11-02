//! `fallout/magic/magicprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `MagicProjectile` (Xbox PDB), 0xA8 bytes on PC (a `MobileObject`; the
    /// fields sit 0x10 lower than the PDB's, as for every `TESForm`, and
    /// `MagicSprayProjectile` allocates 0xA8).
    pub struct MagicProjectile: 0xa8 {
        /// `MobileObject::pCurrentProcess` (Xbox PDB +0x78).
        0x68 pCurrentProcess: u32,
        /// `fSpeed` (Xbox PDB).
        0x88 fSpeed: u32,
        /// `fDistanceMoved` (Xbox PDB).
        0x8C fDistanceMoved: u32,
        /// `fAge` (Xbox PDB).
        0x90 fAge: u32,
        /// `pCaster` (Xbox PDB): `MagicCaster*`.
        0x94 pCaster: u32,
        /// `pSpell` (Xbox PDB): `MagicItem*`.
        0x98 pSpell: u32,
        /// `pDominantEffect` (Xbox PDB): `EffectItem*`.
        0x9C pDominantEffect: u32,
        /// `pAVEffect` (Xbox PDB): `EffectSetting*`.
        0xA0 pAVEffect: u32,
        /// `fLastUpdate` (Xbox PDB).
        0xA4 fLastUpdate: u32,
    }
}

/// `MagicProjectile`'s two vtables (`0107b394` at +0, `0107b388` at +0x18).
const MAGIC_PROJECTILE_VTABLE: u32 = 0x0107_b394;
const MAGIC_PROJECTILE_VTABLE_18: u32 = 0x0107_b388;
/// `MobileObject::MobileObject` (engine map, `0092eb50`) and
/// `MobileObject::~MobileObject` (`0092ec40`), `thiscall`.
const MOBILE_OBJECT_CONSTRUCT: u32 = 0x0092_eb50;
const MOBILE_OBJECT_DESTRUCT: u32 = 0x0092_ec40;
/// `ProcessLists` (`011e0e80`): `GetSystemTimeClock` (engine map,
/// `0096d490`, the time in `ST0`), `AddReference` (`0096d450`, five words)
/// and `RemoveReference` (`0096d470`, two words).
const PROCESS_LISTS: u32 = 0x011e_0e80;
const GET_SYSTEM_TIME_CLOCK: u32 = 0x0096_d490;
const ADD_REFERENCE: u32 = 0x0096_d450;
const REMOVE_REFERENCE: u32 = 0x0096_d470;
/// `operator new` (`00401000`) and `HighProcess::HighProcess` (engine map,
/// `008d7510`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const HIGH_PROCESS_CONSTRUCT: u32 = 0x008d_7510;
/// `TESObjectREFR::SetLocationOnReference` (engine map, `00575830`, one
/// word: the position's address), `00575700` (`thiscall`, three words: the
/// rotation), `TESObjectREFR::SetObjectReference` (`00575690`, one word),
/// `00930700` (`thiscall`, one word: the cell) and `008d6f30` (`thiscall`:
/// a reference's cell).
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
const SET_ROTATION_00575700: u32 = 0x0057_5700;
const SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
const SET_CELL_00930700: u32 = 0x0093_0700;
const REFERENCE_CELL_008D6F30: u32 = 0x008d_6f30;
/// `0045cd60` (`thiscall` on the process: the word passed to
/// `RemoveReference`), `0048cee0` (`thiscall`: a model's path length) and
/// `ModelLoader::ReleaseModel` (engine map, `00445300`, three words) on the
/// model loader (`011c3b3c`).
const PROCESS_KEY_0045CD60: u32 = 0x0045_cd60;
const PATH_LENGTH_0048CEE0: u32 = 0x0048_cee0;
const RELEASE_MODEL: u32 = 0x0044_5300;
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// `004226e0` (`thiscall`, returns a byte) on `011c3f2c`, and
/// `MobileObject::CleanUpOtherMobileObjects` (engine map, `0092ed60`, one
/// word).
const GAME_TEST_004226E0: u32 = 0x0042_26e0;
const GAME_011C3F2C: u32 = 0x011c_3f2c;
const CLEAN_UP_OTHER_MOBILE_OBJECTS: u32 = 0x0092_ed60;
/// `FaderManager::GetFaderAlpha` (engine map, `007014e0`, one word, `ST0`)
/// on `011d8804`, and the `double`s 0.0 (`01012060`) and 0.3 (`010290d0`).
const GET_FADER_ALPHA: u32 = 0x0070_14e0;
const FADER_MANAGER: u32 = 0x011d_8804;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const LONGEST_STEP: u32 = 0x0102_90d0;
/// `00825c00` (`thiscall`: the effect item's effect setting) and `004086a0`
/// (`thiscall` on the effect setting: its projectile kind).
const EFFECT_SETTING_00825C00: u32 = 0x0082_5c00;
const PROJECTILE_KIND_004086A0: u32 = 0x0040_86a0;
/// The four projectile constructors and their sizes:
/// `MagicBallProjectile::MagicBallProjectile` (`0080f810`, 0xC4 bytes, 12
/// words), `MagicBoltProjectile` (`00811980`, 0xD8, 10 words),
/// `MagicSprayProjectile` (`00822920`, 0xA8, 10 words) and
/// `MagicFogProjectile` (`00819360`, 0xD0, 12 words).
const BALL_PROJECTILE_CONSTRUCT: u32 = 0x0080_f810;
const BOLT_PROJECTILE_CONSTRUCT: u32 = 0x0081_1980;
const SPRAY_PROJECTILE_CONSTRUCT: u32 = 0x0082_2920;
const FOG_PROJECTILE_CONSTRUCT: u32 = 0x0081_9360;
/// The dummy reference (`MagicProjectile::pDummyObjectRef`, `011dc774`),
/// `TESAmmo::TESAmmo` (engine map, `005026b0`), the model path
/// `"marker_error.nif"` (`0107b6b4`) and
/// `TESDataHandler::AddFormToDataHandler` (`004603b0`, one word) on the
/// data handler (`011c3f2c`).
const DUMMY_OBJECT_REFERENCE: u32 = 0x011d_c774;
const AMMO_CONSTRUCT: u32 = 0x0050_26b0;
const MARKER_ERROR_NIF: u32 = 0x0107_b6b4;
const ADD_FORM_TO_DATA_HANDLER: u32 = 0x0046_03b0;
/// `TES::IsCellLoaded` (engine map, `004511e0`, two words) on `TES`
/// (`011dea10`).
const IS_CELL_LOADED: u32 = 0x0045_11e0;
const TES: u32 = 0x011d_ea10;
/// `ModelLoader::LoadFile` (engine map, `00447080`, six words), the
/// cloning process constructor `004ad050` (`thiscall`, one `float`) and
/// destructor `004ad270`, `NiObject::Clone` (`00a5d2c0`, one word),
/// `00440490` (one `float`: set the scale), `00450f90` (one word),
/// `0043fa80` / `00440460` (one word: set the rotation / translation) with
/// `011a9448` / `011f426c`, `009611e0` (the parent), `0050f5a0` and
/// `NiAVObject::UpdateProperties` (`00a5a040`).
const LOAD_FILE: u32 = 0x0044_7080;
const CLONING_PROCESS_CONSTRUCT: u32 = 0x004a_d050;
const CLONING_PROCESS_DESTRUCT: u32 = 0x004a_d270;
const NI_OBJECT_CLONE: u32 = 0x00a5_d2c0;
const SET_SCALE_00440490: u32 = 0x0044_0490;
const SET_00450F90: u32 = 0x0045_0f90;
const SET_ROTATE_0043FA80: u32 = 0x0043_fa80;
const SET_TRANSLATE_00440460: u32 = 0x0044_0460;
const IDENTITY_ROTATION: u32 = 0x011a_9448;
const ZERO_POINT: u32 = 0x011f_426c;
const PARENT_009611E0: u32 = 0x0096_11e0;
const OBJECT_0050F5A0: u32 = 0x0050_f5a0;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// The 3D load (`0081e570`): `00440d80` / `00440da0` (`thiscall`, bytes),
/// `004fd400` (the effect setting), `BGSSaveFormBuffer::GetForm` (the
/// engine map's name for `007af430`: the base form), `0050d4a0` (a byte),
/// `00693ef0` (one word), `00570f70` (one word: set the 3D),
/// `TESObjectREFR::GetOrientation` (`0056fa00`, one word),
/// `bhkNiCollisionObject::ResetSim` (`00c6bd00`, `cdecl`, two words),
/// `00448a80` (no arguments: an extra data name),
/// `NiObjectNET::GetExtraData` (`00a5bdd0`, one word), `00448a40` /
/// `00448a20` (bytes), `TESObjectREFR::AddAddonNodes` (`00577e20`,
/// `cdecl`), `0056c7d0` (one word), `TESObjectREFR::InitAnimation`
/// (`005659f0`) and `TESObjectREFR::GetScale` (`00567400`, `ST0`).
const TEST_00440D80: u32 = 0x0044_0d80;
const TEST_00440DA0: u32 = 0x0044_0da0;
const EFFECT_SETTING_004FD400: u32 = 0x004f_d400;
const BASE_FORM_007AF430: u32 = 0x007a_f430;
const FORM_TEST_0050D4A0: u32 = 0x0050_d4a0;
const SET_00693EF0: u32 = 0x0069_3ef0;
const SET_3D_00570F70: u32 = 0x0057_0f70;
const GET_ORIENTATION: u32 = 0x0056_fa00;
const RESET_SIM: u32 = 0x00c6_bd00;
const EXTRA_DATA_NAME_00448A80: u32 = 0x0044_8a80;
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const EXTRA_TEST_00448A40: u32 = 0x0044_8a40;
const REFERENCE_TEST_00448A20: u32 = 0x0044_8a20;
const ADD_ADDON_NODES: u32 = 0x0057_7e20;
const SET_0056C7D0: u32 = 0x0056_c7d0;
const INIT_ANIMATION: u32 = 0x0056_59f0;
const GET_SCALE: u32 = 0x0056_7400;
/// `MobileObject::GetCharController` (engine map, `009306d0`),
/// `00810660` (`thiscall`: the controller's display geometry),
/// `BSShaderUtil::RecursiveSetPropertyWireframe` (`00b6b910`, `cdecl`, two
/// words) and `bhkCharacterController::KillDisplayGeometry` (`00c6ffc0`).
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
const DISPLAY_GEOMETRY_00810660: u32 = 0x0081_0660;
const RECURSIVE_SET_PROPERTY_WIREFRAME: u32 = 0x00b6_b910;
const KILL_DISPLAY_GEOMETRY: u32 = 0x00c6_ffc0;
/// The fixed string constructor `00438170` and destructor `004381b0`, and
/// `"MagicAreaDisplay"` (`0107a898`).
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const MAGIC_AREA_DISPLAY: u32 = 0x0107_a898;
/// The save/load game object (`011de45c`) and its version (`008df040`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const SAVE_VERSION: u32 = 0x008d_f040;
/// `00931ff0` (`thiscall`, one word: `MobileObject`'s save size),
/// `MobileObject::SaveGame` / `LoadGame` / `InitLoadGame` (engine map,
/// `00932070`, `009320e0`, `00932490`), `MagicCaster::GetMagicCasterFormID`
/// (`00818be0`), `MagicItem::GetMagicItemFormID` (`0040a1e0`),
/// `TESForm::SaveNumericID` / `LoadNumericID` / `SaveGameDataOLD` /
/// `LoadGameDataOLD` (`00484d20`, `00484d40`, `00484ce0`, `00484d00`, two
/// words each).
const MOBILE_OBJECT_SAVE_SIZE: u32 = 0x0093_1ff0;
const MOBILE_OBJECT_SAVE_GAME: u32 = 0x0093_2070;
const MOBILE_OBJECT_LOAD_GAME: u32 = 0x0093_20e0;
const MOBILE_OBJECT_INIT_LOAD_GAME: u32 = 0x0093_2490;
const GET_MAGIC_CASTER_FORM_ID: u32 = 0x0081_8be0;
const GET_MAGIC_ITEM_FORM_ID: u32 = 0x0040_a1e0;
const SAVE_NUMERIC_ID: u32 = 0x0048_4d20;
const LOAD_NUMERIC_ID: u32 = 0x0048_4d40;
const SAVE_GAME_DATA_OLD: u32 = 0x0048_4ce0;
const LOAD_GAME_DATA_OLD: u32 = 0x0048_4d00;
/// `MagicCaster::GetMagicCasterByNumericID` (engine map, `00818c60`,
/// `cdecl`), `MagicItem::GetMagicItemByNumericID` (`0040a250`, `cdecl`),
/// `EffectItemList::GetCostliestEffect` (`004064d0`, two words),
/// `MagicItem::GetAVEffect` (`0040a300`, one word),
/// `MobileObject::GetCurrentProcessType` (`00931850`) and
/// `TESSaveLoadGame::AddFormToDeferredDeletionsList` (`0085a3d0`, one
/// word).
const GET_MAGIC_CASTER_BY_NUMERIC_ID: u32 = 0x0081_8c60;
const GET_MAGIC_ITEM_BY_NUMERIC_ID: u32 = 0x0040_a250;
const GET_COSTLIEST_EFFECT: u32 = 0x0040_64d0;
const GET_AV_EFFECT: u32 = 0x0040_a300;
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
const ADD_FORM_TO_DEFERRED_DELETIONS_LIST: u32 = 0x0085_a3d0;
/// `CheckShotBehindWall` (`0081eba0`): `009185e0` (the caster's word +0x94),
/// the player (`011dea3c`), `0070c3b0` (a height, `ST0`), `00439ef0`
/// (`thiscall`, two words: a difference), `00439e90` (`thiscall`, two
/// words: a sum), `0045bb20` (`thiscall`, two words: a scaled point),
/// `0070c440` / `00931ed0` (`thiscall`, one word: a collision filter),
/// `008c71b0` (`thiscall`, one word), the ray query's constructor
/// `004a3c20`, `004a3f70` (filter), `004a3da0` (from), `004a3eb0` (to),
/// `004a3fb0` (collector), `hkpAllRayHitCollector` (`004a3a70`,
/// `004a3bc0`), `00460140` (its hits), `00726070` (their count),
/// `004a46b0` (one hit), `00696c30` (copy a hit), the cell's world
/// (`004543c0`), `GetAVObjectForCollidable` (`004b5820`, `cdecl`),
/// `004b59f0` / `004b5a80` (`cdecl`) and
/// `TESObjectREFR::FindReferenceFor3D` (`0056f930`, `cdecl`).
const CASTER_KEY_009185E0: u32 = 0x0091_85e0;
const PLAYER: u32 = 0x011d_ea3c;
const HEIGHT_0070C3B0: u32 = 0x0070_c3b0;
const POINT_SUBTRACT_00439EF0: u32 = 0x0043_9ef0;
const POINT_ADD_00439E90: u32 = 0x0043_9e90;
const POINT_SCALE_0045BB20: u32 = 0x0045_bb20;
const CONTROLLER_FILTER_0070C440: u32 = 0x0070_c440;
const PLAYER_FILTER_00931ED0: u32 = 0x0093_1ed0;
const WORD_CONSTRUCT_008C71B0: u32 = 0x008c_71b0;
const RAY_QUERY_CONSTRUCT: u32 = 0x004a_3c20;
const RAY_SET_FILTER: u32 = 0x004a_3f70;
const RAY_SET_FROM: u32 = 0x004a_3da0;
const RAY_SET_TO: u32 = 0x004a_3eb0;
const RAY_SET_COLLECTOR: u32 = 0x004a_3fb0;
const ALL_HIT_COLLECTOR_CONSTRUCT: u32 = 0x004a_3a70;
const ALL_HIT_COLLECTOR_DESTRUCT: u32 = 0x004a_3bc0;
const COLLECTOR_HITS_00460140: u32 = 0x0046_0140;
const HIT_COUNT_00726070: u32 = 0x0072_6070;
const HIT_AT_004A46B0: u32 = 0x004a_46b0;
const COPY_HIT_00696C30: u32 = 0x0069_6c30;
const CELL_WORLD_004543C0: u32 = 0x0045_43c0;
const GET_AV_OBJECT_FOR_COLLIDABLE: u32 = 0x004b_5820;
const CONTACT_OBJECT_004B59F0: u32 = 0x004b_59f0;
const CONTACT_OBJECT_004B5A80: u32 = 0x004b_5a80;
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `006815c0` (`thiscall`: answers `this`).
const SELF_006815C0: u32 = 0x0068_15c0;

/// Copies three words.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

// Translated from 0081dd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicProjectile::MagicProjectile` (engine map; `thiscall`, `RET 0x28`:
/// caster, magic item, dominant effect, effect setting, the position and
/// the rotation as three words each): the `MobileObject` base, both
/// vtables, `fAge` 0.0, the four pointers, `fLastUpdate` from
/// `ProcessLists::GetSystemTimeClock`, speed and distance 0.0, a new
/// `HighProcess` (0x46C bytes, 0 when the allocation fails) as the process,
/// the location (`SetLocationOnReference`, given the address of the
/// position words; here a stack block holding them), the rotation
/// (`00575700`), the caster's reference's cell (`00930700`) when the
/// caster's slot +0x2C answers one, and the dummy object
/// (`GetDummyObjectReference`, `SetObjectReference`). Returns `this`.
// Translation incomplete: 0081dd00 - the FS:[0] exception frame (unwinding the base and freeing the process on a throw) is not translated
#[allow(clippy::too_many_arguments)]
pub fn magic_projectile_magic_projectile(
    e: &mut Engine,
    this: Ptr<MagicProjectile>,
    caster: u32,
    spell: u32,
    dominant_effect: u32,
    setting: u32,
    position_x: u32,
    position_y: u32,
    position_z: u32,
    rotation_x: u32,
    rotation_y: u32,
    rotation_z: u32,
) -> Ptr<MagicProjectile> {
    let base = this.addr();
    e.call(MOBILE_OBJECT_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, MAGIC_PROJECTILE_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x18), MAGIC_PROJECTILE_VTABLE_18);
    e.set(this, MagicProjectile::fAge, 0);
    e.set(this, MagicProjectile::pCaster, caster);
    e.set(this, MagicProjectile::pSpell, spell);
    e.set(this, MagicProjectile::pDominantEffect, dominant_effect);
    e.set(this, MagicProjectile::pAVEffect, setting);
    let now = e.call(GET_SYSTEM_TIME_CLOCK, &args![PROCESS_LISTS]).st0;
    e.set(this, MagicProjectile::fLastUpdate, x87_store32(now));
    e.set(this, MagicProjectile::fSpeed, 0);
    e.set(this, MagicProjectile::fDistanceMoved, 0);
    let block = e.call(OPERATOR_NEW, &args![0x46cu32]).u32();
    let process = if block != 0 {
        e.call(HIGH_PROCESS_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.set(this, MagicProjectile::pCurrentProcess, process);
    e.with_stack(0xc, |e, position| {
        let position = position.addr();
        e.mem.set_u32(position, position_x);
        e.mem.set_u32(position.wrapping_add(4), position_y);
        e.mem.set_u32(position.wrapping_add(8), position_z);
        e.call(SET_LOCATION_ON_REFERENCE, &args![base, position]);
    });
    e.call(
        SET_ROTATION_00575700,
        &args![base, rotation_x, rotation_y, rotation_z],
    );
    let reference = e.vcall(caster, 0x2c, &[]).u32();
    if reference != 0 {
        let cell = e.call(REFERENCE_CELL_008D6F30, &args![reference]).u32();
        e.call(SET_CELL_00930700, &args![base, cell]);
    }
    let dummy = magic_projectile_get_dummy_object_reference(e);
    e.call(SET_OBJECT_REFERENCE, &args![base, dummy]);
    this
}

// Translated from 0081de70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicProjectile::~MagicProjectile` (engine map; `thiscall`): both
/// vtables; the projectile leaves the process lists (`RemoveReference`
/// with the process's `0045cd60` word) when it has a process; the effect
/// setting's model is released when its path is not empty
/// (`ModelLoader::ReleaseModel(path, 0, 1)`); unless `004226e0` on
/// `011c3f2c` answers true, `CleanUpOtherMobileObjects(0)`; then the
/// `MobileObject` destructor.
// Translation incomplete: 0081de70 - the FS:[0] exception frame (unwinding the base on a throw) is not translated
pub fn magic_projectile_destructor(e: &mut Engine, this: Ptr<MagicProjectile>) {
    let base = this.addr();
    e.mem.set_u32(base, MAGIC_PROJECTILE_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x18), MAGIC_PROJECTILE_VTABLE_18);
    if e.get(this, MagicProjectile::pCurrentProcess) != 0 {
        let process = e.get(this, MagicProjectile::pCurrentProcess);
        let word = e.call(PROCESS_KEY_0045CD60, &args![process]).u32();
        e.call(REMOVE_REFERENCE, &args![PROCESS_LISTS, base, word]);
    }
    if e.get(this, MagicProjectile::pAVEffect) != 0 {
        let model = e.get(this, MagicProjectile::pAVEffect).wrapping_add(0x18);
        if e.call(PATH_LENGTH_0048CEE0, &args![model]).u32() != 0 {
            let model = e.get(this, MagicProjectile::pAVEffect).wrapping_add(0x18);
            let path = e.vcall(model, 0x14, &[]).u32();
            let loader = e.mem.u32(MODEL_LOADER);
            e.call(RELEASE_MODEL, &args![loader, path, 0u32, 1u32]);
        }
    }
    let game = e.mem.u32(GAME_011C3F2C);
    if !e.call(GAME_TEST_004226E0, &args![game]).bool() {
        e.call(CLEAN_UP_OTHER_MOBILE_OBJECTS, &args![base, 0u32]);
    }
    e.call(MOBILE_OBJECT_DESTRUCT, &args![base]);
}

// Translated from 0081df60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicProjectile::Process` (engine map; `thiscall`, `RET 4`; the word is
/// not read): the step since `fLastUpdate` by the system clock; a clock
/// gone backwards or a negative last update gives a step of 0, or the
/// clock itself when it is above 0 and below 0.3; `fLastUpdate` takes the
/// clock. While the fader's alpha (`GetFaderAlpha(1)`) is at most 0 (or
/// NaN), `fAge` grows by the step and this projectile's slot +0x2F8 runs
/// with it.
pub fn magic_projectile_process(e: &mut Engine, this: Ptr<MagicProjectile>, _unused_0: u32) {
    let base = this.addr();
    let now = x87_store32(e.call(GET_SYSTEM_TIME_CLOCK, &args![PROCESS_LISTS]).st0);
    let last = e.get(this, MagicProjectile::fLastUpdate);
    let mut step = x87_store32(x87_sub(x87_load32(now), x87_m32(last)));
    let zero = e.global::<f64>(ZERO_DOUBLE);
    // FCOMPP / FCOMP with TEST AH,5: the reset when now < last, or else
    // when last < 0.0 (both ordered).
    let backwards = x87_load32(now) < x87_load32(last);
    let reset = backwards || x87_load32(e.get(this, MagicProjectile::fLastUpdate)) < zero;
    if reset {
        step = 0;
        if x87_load32(now) > zero && x87_load32(now) < e.global::<f64>(LONGEST_STEP) {
            step = x87_quiet_bits(now);
        }
    }
    e.set(this, MagicProjectile::fLastUpdate, x87_quiet_bits(now));
    let fader = e.mem.u32(FADER_MANAGER);
    let alpha = e.call(GET_FADER_ALPHA, &args![fader, 1u32]).st0;
    // FCOMP with 0.0; TEST AH,0x41; JNZ: on unless alpha > 0.0.
    if alpha > zero {
        return;
    }
    let age = x87_store32(x87_add(
        x87_load32(e.get(this, MagicProjectile::fAge)),
        x87_m32(step),
    ));
    e.set(this, MagicProjectile::fAge, age);
    e.vcall(base, 0x2f8, &args![x87_quiet_bits(step)]);
}

// Translated from 0081e040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicProjectile::Launch` (engine map; `cdecl`, twelve words: caster,
/// two words for the ball and fog projectiles, magic item, effect item,
/// effect setting (0: the effect item's), the position and the rotation):
/// by the kind `004086a0` gives for the effect-setting word (read even when
/// it is 0) a ball (0), bolt (1), spray (2) or fog (3) projectile is made;
/// the new projectile joins the process lists (`AddReference(p, 0, 0, 0,
/// 0)`). Another kind makes nothing.
// Translation incomplete: 0081e040 - the FS:[0] exception frame (freeing the block when a constructor throws) is not translated
#[allow(clippy::too_many_arguments)]
pub fn magic_projectile_launch(
    e: &mut Engine,
    caster: u32,
    first: u32,
    second: u32,
    spell: u32,
    effect: u32,
    setting: u32,
    position_x: u32,
    position_y: u32,
    position_z: u32,
    rotation_x: u32,
    rotation_y: u32,
    rotation_z: u32,
) {
    let chosen = if setting != 0 {
        setting
    } else {
        e.call(EFFECT_SETTING_00825C00, &args![effect]).u32()
    };
    let kind = e.call(PROJECTILE_KIND_004086A0, &args![setting]).u32();
    let common = [
        caster, spell, effect, chosen, position_x, position_y, position_z, rotation_x, rotation_y,
        rotation_z,
    ];
    let make = |e: &mut Engine, size: u32, constructor: u32, extra: &[u32]| {
        let block = e.call(OPERATOR_NEW, &args![size]).u32();
        if block == 0 {
            return 0;
        }
        let mut words = vec![block];
        words.extend_from_slice(&common);
        words.extend_from_slice(extra);
        e.call(constructor, &words).u32()
    };
    let projectile = match kind {
        0 => make(e, 0xc4, BALL_PROJECTILE_CONSTRUCT, &[first, second]),
        1 => make(e, 0xd8, BOLT_PROJECTILE_CONSTRUCT, &[]),
        2 => make(e, 0xa8, SPRAY_PROJECTILE_CONSTRUCT, &[]),
        3 => make(e, 0xd0, FOG_PROJECTILE_CONSTRUCT, &[first, second]),
        _ => 0,
    };
    if projectile != 0 {
        e.call(
            ADD_REFERENCE,
            &args![PROCESS_LISTS, projectile, 0u32, 0u32, 0u32, 0u32],
        );
    }
}

// Translated from 0081e310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicProjectile::GetDummyObjectReference` (engine map; no arguments):
/// the first call makes the dummy object, a new 0xDC-byte `TESAmmo` whose
/// model (+0x3C, slot +0x18) is `"marker_error.nif"`, keeps it at
/// `011dc774` and adds it to the data handler. Returns the dummy object.
// Translation incomplete: 0081e310 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
pub fn magic_projectile_get_dummy_object_reference(e: &mut Engine) -> u32 {
    if e.mem.u32(DUMMY_OBJECT_REFERENCE) == 0 {
        let block = e.call(OPERATOR_NEW, &args![0xdcu32]).u32();
        let ammo = if block != 0 {
            e.call(AMMO_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.vcall(ammo.wrapping_add(0x3c), 0x18, &args![MARKER_ERROR_NIF]);
        e.mem.set_u32(DUMMY_OBJECT_REFERENCE, ammo);
        let handler = e.mem.u32(GAME_011C3F2C);
        let dummy = e.mem.u32(DUMMY_OBJECT_REFERENCE);
        e.call(ADD_FORM_TO_DATA_HANDLER, &args![handler, dummy]);
    }
    e.mem.u32(DUMMY_OBJECT_REFERENCE)
}

// Translated from 0081e3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`; answers a byte): true when `TES::IsCellLoaded`
/// answers true for the projectile's cell (with 1); otherwise this
/// projectile's slot +0x10 runs with 1 (a null `this` skips it) and the
/// answer is false.
pub fn fn_0081e3d0(e: &mut Engine, this: Ptr<MagicProjectile>) -> u8 {
    let cell = e.call(REFERENCE_CELL_008D6F30, &args![this]).u32();
    let tes = e.mem.u32(TES);
    if e.call(IS_CELL_LOADED, &args![tes, cell, 1u32]).bool() {
        return 1;
    }
    if this.addr() != 0 {
        e.vcall(this.addr(), 0x10, &args![1u32]);
    }
    0
}

// Translated from 0081e440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicProjectile::Clone3D` (engine map; `cdecl`: an effect setting):
/// `Clone3D_ov2` with the setting's model path (its +0x18's slot +0x14);
/// its answer stays in `EAX`.
pub fn magic_projectile_clone3_d(e: &mut Engine, setting: u32) -> u32 {
    let path = e.vcall(setting.wrapping_add(0x18), 0x14, &[]).u32();
    magic_projectile_clone3_d_ov2(e, path)
}

// Translated from 0081e460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicProjectile::Clone3D_ov2` (engine map; `cdecl`: a model path): for
/// a non-empty path the model (`ModelLoader::LoadFile(path, 0, 1, 0, 0,
/// 0)`) is cloned through a cloning process (scale 1.0, on the stack); the
/// clone gets scale 1.0, `00450f90(0)`, the identity rotation and the zero
/// translation, `0050f5a0` when it has a parent, and its properties
/// updated. Returns the clone or 0.
// Translation incomplete: 0081e460 - the FS:[0] exception frame (destroying the cloning process on a throw) is not translated
pub fn magic_projectile_clone3_d_ov2(e: &mut Engine, path: u32) -> u32 {
    let mut model = 0;
    if path != 0 && e.mem.i8(path) != 0 {
        let loader = e.mem.u32(MODEL_LOADER);
        model = e
            .call(
                LOAD_FILE,
                &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
    }
    if model == 0 {
        return 0;
    }
    e.with_stack(0x1c, |e, process| {
        let process = process.addr();
        let one = 1.0f32.to_bits();
        e.call(CLONING_PROCESS_CONSTRUCT, &args![process, one]);
        let clone = e.call(NI_OBJECT_CLONE, &args![model, process]).u32();
        if clone != 0 {
            e.call(SET_SCALE_00440490, &args![clone, one]);
            e.call(SET_00450F90, &args![clone, 0u32]);
            e.call(SET_ROTATE_0043FA80, &args![clone, IDENTITY_ROTATION]);
            e.call(SET_TRANSLATE_00440460, &args![clone, ZERO_POINT]);
            if e.call(PARENT_009611E0, &args![clone]).u32() != 0 {
                e.call(OBJECT_0050F5A0, &args![clone]);
            }
            e.call(UPDATE_PROPERTIES, &args![clone]);
        }
        e.call(CLONING_PROCESS_DESTRUCT, &args![process]);
        clone
    })
}

// Translated from 0081e570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`; the word is not read): the projectile's
/// 3D. Nothing (0) when `00440d80` or `00440da0` answers true; the present
/// 3D's answer is 0 when there is one. Otherwise the effect setting's
/// model is cloned (`Clone3D`) and, when it is, set up: `00693ef0(1)` when
/// the base form passes `0050d4a0`, set as the 3D (`00570f70`), placed at
/// slot +0x1F4's position and the orientation (into a stack block), its
/// simulation reset (`ResetSim(clone, 1)`), add-on nodes added when its
/// extra data (named by `00448a80`) passes `00448a40` and the projectile
/// fails `00448a20`, `0056c7d0`, the animation initialised and the scale
/// set. Returns the clone.
pub fn fn_0081e570(e: &mut Engine, this: Ptr<MagicProjectile>, _unused_0: u32) -> u32 {
    let base = this.addr();
    if e.call(TEST_00440D80, &args![base]).bool() || e.call(TEST_00440DA0, &args![base]).bool() {
        return 0;
    }
    if e.vcall(base, 0x1d0, &[]).u32() != 0 {
        return 0;
    }
    let setting = e.call(EFFECT_SETTING_004FD400, &args![base]).u32();
    let clone = magic_projectile_clone3_d(e, setting);
    if clone == 0 {
        return 0;
    }
    let form = e.call(BASE_FORM_007AF430, &args![base]).u32();
    if e.call(FORM_TEST_0050D4A0, &args![form]).bool() {
        e.call(SET_00693EF0, &args![base, 1u32]);
    }
    e.call(SET_3D_00570F70, &args![base, clone]);
    let position = e.vcall(base, 0x1f4, &[]).u32();
    e.call(SET_TRANSLATE_00440460, &args![clone, position]);
    e.with_stack(0x24, |e, rows| {
        let rotation = e.call(GET_ORIENTATION, &args![base, rows]).u32();
        e.call(SET_ROTATE_0043FA80, &args![clone, rotation]);
    });
    e.call(RESET_SIM, &args![clone, 1u32]);
    let name = e.call(EXTRA_DATA_NAME_00448A80, &[]).u32();
    let extra = e.call(GET_EXTRA_DATA, &args![clone, name]).u32();
    if extra != 0
        && e.call(EXTRA_TEST_00448A40, &args![extra]).bool()
        && !e.call(REFERENCE_TEST_00448A20, &args![base]).bool()
    {
        e.call(ADD_ADDON_NODES, &args![clone]);
    }
    e.call(SET_0056C7D0, &args![clone, base]);
    e.call(INIT_ANIMATION, &args![base]);
    let scale = x87_store32(e.call(GET_SCALE, &args![base]).st0);
    e.call(SET_SCALE_00440490, &args![clone, scale]);
    clone
}

// Translated from 0081e6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a byte): with a character controller, on
/// shows its display geometry (its slot +0xD0 with 0, the geometry set to
/// wireframe with `RecursiveSetPropertyWireframe(geometry, 1)`, this
/// projectile's slot +0x310); off kills the display geometry and removes
/// the area display (`0081e730`).
pub fn fn_0081e6c0(e: &mut Engine, this: Ptr<MagicProjectile>, show: u8) {
    let base = this.addr();
    let controller = e.call(GET_CHAR_CONTROLLER, &args![base]).u32();
    if controller == 0 {
        return;
    }
    if show != 0 {
        e.vcall(controller, 0xd0, &args![0u32]);
        let geometry = e.call(DISPLAY_GEOMETRY_00810660, &args![controller]).u32();
        e.call(RECURSIVE_SET_PROPERTY_WIREFRAME, &args![geometry, 1u32]);
        e.vcall(base, 0x310, &[]);
    } else {
        e.call(KILL_DISPLAY_GEOMETRY, &args![controller]);
        fn_0081e730(e, this);
    }
}

// Translated from 0081e730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): finds the `"MagicAreaDisplay"` node in the 3D
/// (its slot +0x9C, the name as a fixed string on the stack) and detaches
/// it from its parent (the parent's slot +0xE8).
// Translation incomplete: 0081e730 - the FS:[0] exception frame (destroying the fixed string on a throw) is not translated
pub fn fn_0081e730(e: &mut Engine, this: Ptr<MagicProjectile>) {
    let base = this.addr();
    let display = e.with_stack(4, |e, name| {
        if e.vcall(base, 0x1d0, &[]).u32() == 0 {
            return 0;
        }
        let node = e.vcall(base, 0x1d0, &[]).u32();
        let made = e
            .call(FIXED_STRING_CONSTRUCT, &args![name, MAGIC_AREA_DISPLAY])
            .u32();
        let found = e.vcall(node, 0x9c, &args![made]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![name]);
        found
    });
    if display != 0 && e.call(PARENT_009611E0, &args![display]).u32() != 0 {
        let parent = e.call(PARENT_009611E0, &args![display]).u32();
        e.vcall(parent, 0xe8, &args![display]);
    }
}

// Translated from 0081e830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: the save size of `MagicProjectile`):
/// `MobileObject`'s (`00931ff0`) plus 12, plus 4 from save version 0x48 and
/// 4 more from version 100 (16-bit sums).
pub fn fn_0081e830(e: &mut Engine, this: Ptr<MagicProjectile>, argument: u32) -> u16 {
    let mut size = e
        .call(MOBILE_OBJECT_SAVE_SIZE, &args![this, argument])
        .u16();
    size = size.wrapping_add(4).wrapping_add(4).wrapping_add(4);
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(SAVE_VERSION, &args![save_load]).u8() >= 0x48 {
        size = size.wrapping_add(4);
    }
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(SAVE_VERSION, &args![save_load]).u8() >= 100 {
        size = size.wrapping_add(4);
    }
    size
}

// Translated from 0081e8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicProjectile::SaveGame` (engine map; `thiscall`, `RET 4`):
/// `MobileObject::SaveGame`, the caster's and the magic item's form ids,
/// `fLastUpdate`, then `fAge` from save version 0x48 and `fDistanceMoved`
/// from version 100.
pub fn magic_projectile_save_game(e: &mut Engine, this: Ptr<MagicProjectile>, argument: u32) {
    let base = this.addr();
    e.call(MOBILE_OBJECT_SAVE_GAME, &args![base, argument]);
    e.with_stack(8, |e, frame| {
        // `[EBP-4]` and `[EBP-8]`.
        let caster_id = frame.addr().wrapping_add(4);
        let item_id = frame.addr();
        let caster = e.get(this, MagicProjectile::pCaster);
        let id = e.call(GET_MAGIC_CASTER_FORM_ID, &args![caster]).u32();
        e.mem.set_u32(caster_id, id);
        e.call(SAVE_NUMERIC_ID, &args![base, caster_id, 4u32]);
        let spell = e.get(this, MagicProjectile::pSpell);
        let id = e.call(GET_MAGIC_ITEM_FORM_ID, &args![spell]).u32();
        e.mem.set_u32(item_id, id);
        e.call(SAVE_NUMERIC_ID, &args![base, item_id, 4u32]);
    });
    e.call(
        SAVE_GAME_DATA_OLD,
        &args![base, base.wrapping_add(0xa4), 4u32],
    );
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(SAVE_VERSION, &args![save_load]).u8() >= 0x48 {
        e.call(
            SAVE_GAME_DATA_OLD,
            &args![base, base.wrapping_add(0x90), 4u32],
        );
    }
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(SAVE_VERSION, &args![save_load]).u8() >= 100 {
        e.call(
            SAVE_GAME_DATA_OLD,
            &args![base, base.wrapping_add(0x8c), 4u32],
        );
    }
}

// Translated from 0081e980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`: `MagicProjectile`'s load):
/// `MobileObject::LoadGame`, the caster's and the magic item's ids (kept as
/// ids in `pCaster` and `pSpell`), `fLastUpdate`, then `fAge` from save
/// version 0x48 and `fDistanceMoved` from version 100.
pub fn fn_0081e980(e: &mut Engine, this: Ptr<MagicProjectile>, first: u32, second: u32) {
    let base = this.addr();
    e.call(MOBILE_OBJECT_LOAD_GAME, &args![base, first, second]);
    e.with_stack(8, |e, frame| {
        let caster_id = frame.addr().wrapping_add(4);
        let item_id = frame.addr();
        e.call(LOAD_NUMERIC_ID, &args![base, caster_id, 4u32]);
        let id = e.mem.u32(caster_id);
        e.set(this, MagicProjectile::pCaster, id);
        e.call(LOAD_NUMERIC_ID, &args![base, item_id, 4u32]);
        let id = e.mem.u32(item_id);
        e.set(this, MagicProjectile::pSpell, id);
    });
    e.call(
        LOAD_GAME_DATA_OLD,
        &args![base, base.wrapping_add(0xa4), 4u32],
    );
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(SAVE_VERSION, &args![save_load]).u8() >= 0x48 {
        e.call(
            LOAD_GAME_DATA_OLD,
            &args![base, base.wrapping_add(0x90), 4u32],
        );
    }
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(SAVE_VERSION, &args![save_load]).u8() >= 100 {
        e.call(
            LOAD_GAME_DATA_OLD,
            &args![base, base.wrapping_add(0x8c), 4u32],
        );
    }
}

// Translated from 0081ea40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicProjectile::InitLoadGame` (engine map; `thiscall`, `RET 8`): the
/// caster and magic item from their ids, the dominant effect
/// (`GetCostliestEffect(2, 0)` of the item's effect list at +0xC) and the
/// effect setting (`GetAVEffect(0)`), slot +0x1C8 with 0,
/// `MobileObject::InitLoadGame`, the projectile moved within the process
/// lists (removed and added with its process type); without a caster or a
/// magic item it goes on the deferred deletions list.
pub fn magic_projectile_init_load_game(
    e: &mut Engine,
    this: Ptr<MagicProjectile>,
    first: u32,
    second: u32,
) {
    let base = this.addr();
    let id = e.get(this, MagicProjectile::pCaster);
    let caster = e.call(GET_MAGIC_CASTER_BY_NUMERIC_ID, &args![id]).u32();
    e.set(this, MagicProjectile::pCaster, caster);
    let id = e.get(this, MagicProjectile::pSpell);
    let spell = e.call(GET_MAGIC_ITEM_BY_NUMERIC_ID, &args![id]).u32();
    e.set(this, MagicProjectile::pSpell, spell);
    let list = e.get(this, MagicProjectile::pSpell).wrapping_add(0xc);
    let dominant = e.call(GET_COSTLIEST_EFFECT, &args![list, 2u32, 0u32]).u32();
    e.set(this, MagicProjectile::pDominantEffect, dominant);
    let spell = e.get(this, MagicProjectile::pSpell);
    let setting = e.call(GET_AV_EFFECT, &args![spell, 0u32]).u32();
    e.set(this, MagicProjectile::pAVEffect, setting);
    e.vcall(base, 0x1c8, &args![0u32]);
    e.call(MOBILE_OBJECT_INIT_LOAD_GAME, &args![base, first, second]);
    let kind = e.call(GET_CURRENT_PROCESS_TYPE, &args![base]).u32();
    e.call(REMOVE_REFERENCE, &args![PROCESS_LISTS, base, kind]);
    let kind = e.call(GET_CURRENT_PROCESS_TYPE, &args![base]).u32();
    e.call(
        ADD_REFERENCE,
        &args![PROCESS_LISTS, base, kind, 0u32, 0u32, 0u32],
    );
    if e.get(this, MagicProjectile::pCaster) == 0 || e.get(this, MagicProjectile::pSpell) == 0 {
        let save_load = e.mem.u32(SAVE_LOAD_GAME);
        e.call(ADD_FORM_TO_DEFERRED_DELETIONS_LIST, &args![save_load, base]);
    }
}

// Translated from 0081eb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a caster; answers a byte): when the word
/// is the projectile's caster, clears `pCaster`, calls this projectile's
/// slot +0xC4 with 1 and answers true.
pub fn fn_0081eb50(e: &mut Engine, this: Ptr<MagicProjectile>, caster: u32) -> u8 {
    if caster != e.get(this, MagicProjectile::pCaster) {
        return 0;
    }
    e.set(this, MagicProjectile::pCaster, 0);
    e.vcall(this.addr(), 0xc4, &args![1u32]);
    1
}

// Translated from 0081eba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicProjectile::CheckShotBehindWall` (engine map; `thiscall`). Only
/// for a projectile with 3D whose caster is the player (or which has no
/// caster word +0x94): a ray from the projectile's position (slot +0x1F4)
/// to the player's eye (the player's position raised by `0070c3b0` times
/// the scale) with the player's collision filter (the character
/// controller's `0070c440`, or the player's `00931ed0`) is cast in the
/// player's cell's world (its slot +0xC8 with the query); the first hit
/// whose reference is neither the player, the projectile nor an actor, and
/// whose collidable gives an object (`004b59f0`, `004b5a80`), reports the
/// hit point (start plus the hit fraction times the ray) through the
/// projectile's slot +0x304 with that object, the reference and 0.
///
/// The stack frame is modelled as a block of the frame's size, the locals
/// at their `EBP` offsets.
// Translation incomplete: 0081eba0 - the FS:[0] exception frame and the /GS stack cookie check (00ec408c) are not translated
pub fn magic_projectile_check_shot_behind_wall(e: &mut Engine, this: Ptr<MagicProjectile>) {
    let base = this.addr();
    let node = e.vcall(base, 0x1d0, &[]).u32();
    if e.call(CASTER_KEY_009185E0, &args![base]).u32() != 0 {
        let caster = e.call(CASTER_KEY_009185E0, &args![base]).u32();
        let reference = e.vcall(caster, 0x2c, &[]).u32();
        if reference != e.mem.u32(PLAYER) {
            return;
        }
    }
    if node == 0 {
        return;
    }
    e.with_stack(0x4f0, |e, frame| {
        let ebp = frame.addr().wrapping_add(0x4f0);
        let local = |offset: u32| ebp.wrapping_sub(offset);
        let query = local(0xd0);
        let eye = local(0xdc);
        let start = local(0xe8);
        let ray = local(0xf4);
        e.call(RAY_QUERY_CONSTRUCT, &args![query]);
        e.call(SELF_006815C0, &args![eye]);
        e.call(SELF_006815C0, &args![start]);
        e.call(SELF_006815C0, &args![ray]);
        let position = e.vcall(base, 0x1f4, &[]).u32();
        copy_point(e, start, position);
        let player = e.mem.u32(PLAYER);
        let position = e.vcall(player, 0x1f4, &[]).u32();
        copy_point(e, eye, position);
        let player = e.mem.u32(PLAYER);
        let height = e.call(HEIGHT_0070C3B0, &args![player]).st0;
        let player = e.mem.u32(PLAYER);
        let scale = e.call(GET_SCALE, &args![player]).st0;
        let z = x87_add(x87_mul(scale, height), x87_m32(e.mem.u32(local(0xd4))));
        e.mem.set_u32(local(0xd4), x87_store32(z));
        let difference = e
            .call(POINT_SUBTRACT_00439EF0, &args![start, local(0x10c), eye])
            .u32();
        copy_point(e, ray, difference);
        let controller = e.call(GET_CHAR_CONTROLLER, &args![base]).u32();
        e.mem.set_u32(local(0x110), controller);
        e.call(WORD_CONSTRUCT_008C71B0, &args![local(0x114), 0u32]);
        let filter = if e.mem.u32(local(0x110)) != 0 {
            let controller = e.mem.u32(local(0x110));
            e.call(CONTROLLER_FILTER_0070C440, &args![controller, local(0x118)])
                .u32()
        } else {
            let player = e.mem.u32(PLAYER);
            e.call(PLAYER_FILTER_00931ED0, &args![player, local(0x11c)])
                .u32()
        };
        let word = e.mem.u32(filter);
        e.mem.set_u32(local(0x114), word);
        let word = e.mem.u32(local(0x114));
        e.call(RAY_SET_FILTER, &args![query, word]);
        e.call(RAY_SET_FROM, &args![query, eye]);
        let to = e
            .call(POINT_ADD_00439E90, &args![eye, local(0x128), ray])
            .u32();
        e.call(RAY_SET_TO, &args![query, to]);
        let collector = local(0x450);
        e.call(ALL_HIT_COLLECTOR_CONSTRUCT, &args![collector]);
        e.call(RAY_SET_COLLECTOR, &args![query, collector]);
        e.mem.set_u8(local(0x451), 0);
        let player = e.mem.u32(PLAYER);
        if e.call(REFERENCE_CELL_008D6F30, &args![player]).u32() != 0 {
            let player = e.mem.u32(PLAYER);
            let cell = e.call(REFERENCE_CELL_008D6F30, &args![player]).u32();
            let world = e.call(CELL_WORLD_004543C0, &args![cell]).u32();
            if e.vcall(world, 0xc8, &args![query]).bool() {
                let mut searching = true;
                let mut index: i32 = 0;
                while searching {
                    let hits = e.call(COLLECTOR_HITS_00460140, &args![collector]).u32();
                    let count = e.call(HIT_COUNT_00726070, &args![hits]).i32();
                    if index >= count || e.mem.u8(local(0x451)) != 0 {
                        break;
                    }
                    let hits = e.call(COLLECTOR_HITS_00460140, &args![collector]).u32();
                    let hit = e.call(HIT_AT_004A46B0, &args![hits, index as u32]).u32();
                    e.call(COPY_HIT_00696C30, &args![local(0x4c0), hit]);
                    let collidable = e.mem.u32(local(0x470));
                    let object = e
                        .call(GET_AV_OBJECT_FOR_COLLIDABLE, &args![collidable])
                        .u32();
                    if object != 0 {
                        let collidable = e.mem.u32(local(0x470));
                        let found = e.call(CONTACT_OBJECT_004B59F0, &args![collidable]).u32();
                        let source = e.call(CONTACT_OBJECT_004B5A80, &args![found]).u32();
                        let reference = e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32();
                        if reference != e.mem.u32(PLAYER)
                            && reference != base
                            && !e.vcall(reference, 0x100, &[]).bool()
                            && source != 0
                        {
                            let fraction = x87_quiet_bits(e.mem.u32(local(0x4b0)));
                            let scaled = e
                                .call(POINT_SCALE_0045BB20, &args![ray, local(0x4d8), fraction])
                                .u32();
                            e.call(POINT_ADD_00439E90, &args![eye, local(0x4e4), scaled]);
                            let x = e.mem.u32(local(0x4e4));
                            let y = e.mem.u32(local(0x4e0));
                            let z = e.mem.u32(local(0x4dc));
                            e.vcall(base, 0x304, &args![x, y, z, source, reference, 0u32]);
                            searching = false;
                        }
                    }
                    index = index.wrapping_add(1);
                }
            }
        }
        e.call(ALL_HIT_COLLECTOR_DESTRUCT, &args![collector]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0081_dd00,
            magic_projectile_magic_projectile(
                Ptr<MagicProjectile>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<MagicProjectile>
        ),
        entry!(
            0x0081_de70,
            magic_projectile_destructor(Ptr<MagicProjectile>)
        ),
        entry!(
            0x0081_df60,
            magic_projectile_process(Ptr<MagicProjectile>, u32)
        ),
        entry!(
            0x0081_e040,
            magic_projectile_launch(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x0081_e310, magic_projectile_get_dummy_object_reference() -> u32),
        entry!(0x0081_e3d0, fn_0081e3d0(Ptr<MagicProjectile>) -> u8),
        entry!(0x0081_e440, magic_projectile_clone3_d(u32) -> u32),
        entry!(0x0081_e460, magic_projectile_clone3_d_ov2(u32) -> u32),
        entry!(0x0081_e570, fn_0081e570(Ptr<MagicProjectile>, u32) -> u32),
        entry!(0x0081_e6c0, fn_0081e6c0(Ptr<MagicProjectile>, u8)),
        entry!(0x0081_e730, fn_0081e730(Ptr<MagicProjectile>)),
        entry!(0x0081_e830, fn_0081e830(Ptr<MagicProjectile>, u32) -> u16),
        entry!(
            0x0081_e8c0,
            magic_projectile_save_game(Ptr<MagicProjectile>, u32)
        ),
        entry!(0x0081_e980, fn_0081e980(Ptr<MagicProjectile>, u32, u32)),
        entry!(
            0x0081_ea40,
            magic_projectile_init_load_game(Ptr<MagicProjectile>, u32, u32)
        ),
        entry!(0x0081_eb50, fn_0081eb50(Ptr<MagicProjectile>, u32) -> u8),
        entry!(
            0x0081_eba0,
            magic_projectile_check_shot_behind_wall(Ptr<MagicProjectile>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

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

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn zero(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    /// An object whose vtable sends each listed slot to `code + slot`, a
    /// double answering the paired word.
    fn object(e: &mut Engine, code: u32, slots: &[(u32, u32)]) -> u32 {
        let size = slots.iter().map(|s| s.0).max().unwrap_or(0) + 4;
        let vtable = e.mem.alloc(size);
        for &(slot, answer) in slots {
            e.mem.set_u32(vtable + slot, code + slot);
            e.register_double(code + slot, move |_, _| ret(answer));
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    const PROJECTILE: u32 = 0x0300_0000;
    const CASTER: u32 = 0x0310_0000;
    const SETTING: u32 = 0x0320_0000;
    const NODE: u32 = 0x0330_0000;
    const PLAYER_CODE: u32 = 0x0340_0000;
    const WORLD: u32 = 0x0350_0000;
    const REFERENCE: u32 = 0x0360_0000;

    fn map_globals(e: &mut Engine) {
        for addr in [
            DUMMY_OBJECT_REFERENCE,
            GAME_011C3F2C,
            MODEL_LOADER,
            FADER_MANAGER,
            SAVE_LOAD_GAME,
            PLAYER,
            TES,
        ] {
            e.map(addr, 4);
        }
        for (addr, value) in [(ZERO_DOUBLE, 0.0f64), (LONGEST_STEP, 0.3)] {
            e.map(addr, 8);
            e.set_global::<f64>(addr, value);
        }
    }

    #[test]
    fn constructor_fills_the_projectile() {
        let mut e = Engine::new();
        map_globals(&mut e);
        e.mem.set_u32(DUMMY_OBJECT_REFERENCE, 0xd0d0);
        zero(
            &mut e,
            &[
                MOBILE_OBJECT_CONSTRUCT,
                SET_ROTATION_00575700,
                SET_CELL_00930700,
                SET_OBJECT_REFERENCE,
            ],
        );
        e.register(GET_SYSTEM_TIME_CLOCK, |_, _| st0(12.5));
        e.register(OPERATOR_NEW, |_, _| ret(0x4000));
        e.register(HIGH_PROCESS_CONSTRUCT, |_, a| ret(a[0] + 4));
        e.register(SET_LOCATION_ON_REFERENCE, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 7);
            assert_eq!(e.mem.u32(a[1] + 8), 9);
            Ret::default()
        });
        e.register(REFERENCE_CELL_008D6F30, |_, a| ret(a[0] + 0x10));
        let caster = object(&mut e, CASTER, &[(0x2c, 0x5e5e)]);
        let this = e.new_object::<MagicProjectile>();
        let (r, log) = run(
            &mut e,
            0x0081_dd00,
            &args![this, caster, 2u32, 3u32, 4u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32],
        );
        assert_eq!(r.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), MAGIC_PROJECTILE_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x18), MAGIC_PROJECTILE_VTABLE_18);
        assert_eq!(e.get(this, MagicProjectile::pCaster), caster);
        assert_eq!(e.get(this, MagicProjectile::pSpell), 2);
        assert_eq!(e.get(this, MagicProjectile::pDominantEffect), 3);
        assert_eq!(e.get(this, MagicProjectile::pAVEffect), 4);
        assert_eq!(e.get(this, MagicProjectile::fLastUpdate), 12.5f32.to_bits());
        assert_eq!(e.get(this, MagicProjectile::pCurrentProcess), 0x4004);
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![0x46c]]);
        assert_eq!(
            calls(&log, SET_ROTATION_00575700),
            vec![vec![this.addr(), 10, 11, 12]]
        );
        assert_eq!(
            calls(&log, SET_CELL_00930700),
            vec![vec![this.addr(), 0x5e6e]]
        );
        assert_eq!(
            calls(&log, SET_OBJECT_REFERENCE),
            vec![vec![this.addr(), 0xd0d0]]
        );
        // No caster reference: no cell; a failed allocation: no process.
        e.register(CASTER + 0x2c, |_, _| ret(0));
        e.register(OPERATOR_NEW, |_, _| ret(0));
        let (_, log) = run(
            &mut e,
            0x0081_dd00,
            &args![this, caster, 2u32, 3u32, 4u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32],
        );
        assert!(calls(&log, SET_CELL_00930700).is_empty());
        assert_eq!(e.get(this, MagicProjectile::pCurrentProcess), 0);
    }

    #[test]
    fn destructor_releases_the_process_and_model() {
        let mut e = Engine::new();
        map_globals(&mut e);
        e.mem.set_u32(MODEL_LOADER, 0x10ad);
        zero(
            &mut e,
            &[
                REMOVE_REFERENCE,
                RELEASE_MODEL,
                CLEAN_UP_OTHER_MOBILE_OBJECTS,
                MOBILE_OBJECT_DESTRUCT,
            ],
        );
        e.register(PROCESS_KEY_0045CD60, |_, _| ret(3));
        e.register(PATH_LENGTH_0048CEE0, |_, _| ret(5));
        e.register(GAME_TEST_004226E0, |_, _| ret(0));
        let setting = e.mem.alloc(0x20);
        let model = object(&mut e, SETTING, &[(0x14, 0x9a7)]);
        let vtable = e.mem.u32(model);
        e.mem.set_u32(setting + 0x18, vtable);
        let this = e.new_object::<MagicProjectile>();
        e.set(this, MagicProjectile::pCurrentProcess, 0x4004);
        e.set(this, MagicProjectile::pAVEffect, setting);
        let (_, log) = run(&mut e, 0x0081_de70, &args![this]);
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            vec![vec![PROCESS_LISTS, this.addr(), 3]]
        );
        assert_eq!(calls(&log, RELEASE_MODEL), vec![vec![0x10ad, 0x9a7, 0, 1]]);
        assert_eq!(
            calls(&log, CLEAN_UP_OTHER_MOBILE_OBJECTS),
            vec![vec![this.addr(), 0]]
        );
        assert_eq!(calls(&log, MOBILE_OBJECT_DESTRUCT), vec![vec![this.addr()]]);
        e.register(PATH_LENGTH_0048CEE0, |_, _| ret(0));
        e.register(GAME_TEST_004226E0, |_, _| ret(1));
        e.set(this, MagicProjectile::pCurrentProcess, 0);
        let (_, log) = run(&mut e, 0x0081_de70, &args![this]);
        assert!(calls(&log, REMOVE_REFERENCE).is_empty());
        assert!(calls(&log, RELEASE_MODEL).is_empty());
        assert!(calls(&log, CLEAN_UP_OTHER_MOBILE_OBJECTS).is_empty());
    }

    fn process_world(e: &mut Engine, now: f64, alpha: f64) -> Ptr<MagicProjectile> {
        map_globals(e);
        e.register_double(GET_SYSTEM_TIME_CLOCK, move |_, _| st0(now));
        e.register_double(GET_FADER_ALPHA, move |_, _| st0(alpha));
        let this = object(e, PROJECTILE, &[(0x2f8, 0)]);
        Ptr::new(this)
    }

    #[test]
    fn process_ages_by_the_clock_step() {
        let mut e = Engine::new();
        let this = process_world(&mut e, 10.25, 0.0);
        e.set(this, MagicProjectile::fLastUpdate, 10.0f32.to_bits());
        e.set(this, MagicProjectile::fAge, 1.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0081_df60, &args![this, 0u32]);
        assert_eq!(
            e.get(this, MagicProjectile::fLastUpdate),
            10.25f32.to_bits()
        );
        assert_eq!(e.get(this, MagicProjectile::fAge), 1.25f32.to_bits());
        assert_eq!(
            calls(&log, PROJECTILE + 0x2f8),
            vec![vec![this.addr(), 0.25f32.to_bits()]]
        );
        // A clock gone backwards: the clock itself when small.
        let this = process_world(&mut e, 0.125, 0.0);
        e.set(this, MagicProjectile::fLastUpdate, 10.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0081_df60, &args![this, 0u32]);
        assert_eq!(calls(&log, PROJECTILE + 0x2f8)[0][1], 0.125f32.to_bits());
        // Backwards to 0.5: no step.
        let this = process_world(&mut e, 0.5, 0.0);
        e.set(this, MagicProjectile::fLastUpdate, 10.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0081_df60, &args![this, 0u32]);
        assert_eq!(calls(&log, PROJECTILE + 0x2f8)[0][1], 0);
        // A negative last update resets too.
        let this = process_world(&mut e, 2.0, 0.0);
        e.set(this, MagicProjectile::fLastUpdate, (-1.0f32).to_bits());
        let (_, log) = run(&mut e, 0x0081_df60, &args![this, 0u32]);
        assert_eq!(calls(&log, PROJECTILE + 0x2f8)[0][1], 0);
        // A visible fader: no aging.
        let this = process_world(&mut e, 2.0, 0.5);
        e.set(this, MagicProjectile::fLastUpdate, 1.0f32.to_bits());
        let (_, log) = run(&mut e, 0x0081_df60, &args![this, 0u32]);
        assert!(calls(&log, PROJECTILE + 0x2f8).is_empty());
        assert_eq!(e.get(this, MagicProjectile::fLastUpdate), 2.0f32.to_bits());
        assert_eq!(calls(&log, GET_FADER_ALPHA)[0][1], 1);
    }

    #[test]
    fn launch_makes_the_kind_of_projectile() {
        let mut e = Engine::new();
        map_globals(&mut e);
        e.register(OPERATOR_NEW, |_, a| ret(0x7000 + a[0]));
        for constructor in [
            BALL_PROJECTILE_CONSTRUCT,
            BOLT_PROJECTILE_CONSTRUCT,
            SPRAY_PROJECTILE_CONSTRUCT,
            FOG_PROJECTILE_CONSTRUCT,
        ] {
            e.register(constructor, |_, a| ret(a[0]));
        }
        zero(&mut e, &[ADD_REFERENCE]);
        e.register(EFFECT_SETTING_00825C00, |_, a| ret(a[0] + 0x100));
        let words =
            |kind: u32, setting: u32| vec![1, 2, 3, 4, 5 + kind, setting, 7, 8, 9, 10, 11, 12];
        let expectations = [
            (0u32, BALL_PROJECTILE_CONSTRUCT, 0xc4u32, true),
            (1, BOLT_PROJECTILE_CONSTRUCT, 0xd8, false),
            (2, SPRAY_PROJECTILE_CONSTRUCT, 0xa8, false),
            (3, FOG_PROJECTILE_CONSTRUCT, 0xd0, true),
        ];
        for (kind, constructor, size, extra) in expectations {
            e.register_double(PROJECTILE_KIND_004086A0, move |_, _| ret(kind));
            let (_, log) = run(&mut e, 0x0081_e040, &words(kind, 0x55));
            let made = calls(&log, constructor);
            let block = 0x7000 + size;
            let mut expected = vec![block, 1, 4, 5 + kind, 0x55, 7, 8, 9, 10, 11, 12];
            if extra {
                expected.extend_from_slice(&[2, 3]);
            }
            assert_eq!(made, vec![expected]);
            assert_eq!(calls(&log, PROJECTILE_KIND_004086A0), vec![vec![0x55]]);
            assert_eq!(
                calls(&log, ADD_REFERENCE),
                vec![vec![PROCESS_LISTS, block, 0, 0, 0, 0]]
            );
        }
        // No setting: the effect item's, while the kind reads the 0 word.
        e.register(PROJECTILE_KIND_004086A0, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0081_e040, &words(1, 0));
        assert_eq!(calls(&log, PROJECTILE_KIND_004086A0), vec![vec![0]]);
        assert_eq!(calls(&log, BOLT_PROJECTILE_CONSTRUCT)[0][4], 0x106);
        // Another kind: nothing.
        e.register(PROJECTILE_KIND_004086A0, |_, _| ret(4));
        let (_, log) = run(&mut e, 0x0081_e040, &words(4, 0x55));
        assert!(calls(&log, OPERATOR_NEW).is_empty());
        assert!(calls(&log, ADD_REFERENCE).is_empty());
        // A failed allocation: nothing added.
        e.register(PROJECTILE_KIND_004086A0, |_, _| ret(2));
        e.register(OPERATOR_NEW, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0081_e040, &words(2, 0x55));
        assert!(calls(&log, SPRAY_PROJECTILE_CONSTRUCT).is_empty());
        assert!(calls(&log, ADD_REFERENCE).is_empty());
    }

    #[test]
    fn dummy_object_is_made_once() {
        let mut e = Engine::new();
        map_globals(&mut e);
        e.mem.set_u32(GAME_011C3F2C, 0xda7a);
        let ammo = object(&mut e, 0x0370_0000, &[]);
        let model = object(&mut e, SETTING, &[(0x18, 0)]);
        let vtable = e.mem.u32(model);
        e.mem.set_u32(ammo + 0x3c, vtable);
        e.register(OPERATOR_NEW, |_, _| ret(0x6000));
        e.register_double(AMMO_CONSTRUCT, move |_, _| ret(ammo));
        zero(&mut e, &[ADD_FORM_TO_DATA_HANDLER]);
        let (r, log) = run(&mut e, 0x0081_e310, &[]);
        assert_eq!(r.u32(), ammo);
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![0xdc]]);
        assert_eq!(calls(&log, AMMO_CONSTRUCT), vec![vec![0x6000]]);
        assert_eq!(
            calls(&log, SETTING + 0x18),
            vec![vec![ammo + 0x3c, MARKER_ERROR_NIF]]
        );
        assert_eq!(
            calls(&log, ADD_FORM_TO_DATA_HANDLER),
            vec![vec![0xda7a, ammo]]
        );
        let (r, log) = run(&mut e, 0x0081_e310, &[]);
        assert_eq!(r.u32(), ammo);
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn cell_check_and_caster_clear() {
        let mut e = Engine::new();
        map_globals(&mut e);
        e.mem.set_u32(TES, 0x7e5);
        let this = object(&mut e, PROJECTILE, &[(0x10, 0), (0xc4, 0)]);
        e.register(REFERENCE_CELL_008D6F30, |_, _| ret(0xce11));
        e.register(IS_CELL_LOADED, |_, _| ret(1));
        let (r, log) = run(&mut e, 0x0081_e3d0, &args![this]);
        assert_eq!(r.u32(), 1);
        assert_eq!(calls(&log, IS_CELL_LOADED), vec![vec![0x7e5, 0xce11, 1]]);
        e.register(IS_CELL_LOADED, |_, _| ret(0));
        let (r, log) = run(&mut e, 0x0081_e3d0, &args![this]);
        assert_eq!(r.u32(), 0);
        assert_eq!(calls(&log, PROJECTILE + 0x10), vec![vec![this, 1]]);
        // The caster check.
        let this = Ptr::<MagicProjectile>::new(this);
        e.set(this, MagicProjectile::pCaster, 0x5c);
        let (r, _) = run(&mut e, 0x0081_eb50, &args![this, 0x5du32]);
        assert_eq!(r.u32(), 0);
        assert_eq!(e.get(this, MagicProjectile::pCaster), 0x5c);
        let (r, log) = run(&mut e, 0x0081_eb50, &args![this, 0x5cu32]);
        assert_eq!(r.u32(), 1);
        assert_eq!(e.get(this, MagicProjectile::pCaster), 0);
        assert_eq!(calls(&log, PROJECTILE + 0xc4), vec![vec![this.addr(), 1]]);
    }

    fn clone_world(e: &mut Engine) {
        map_globals(e);
        e.mem.set_u32(MODEL_LOADER, 0x10ad);
        e.register(LOAD_FILE, |_, _| ret(0x0de1));
        e.register(NI_OBJECT_CLONE, |_, _| ret(0xc10e));
        zero(
            e,
            &[
                CLONING_PROCESS_CONSTRUCT,
                CLONING_PROCESS_DESTRUCT,
                SET_SCALE_00440490,
                SET_00450F90,
                SET_ROTATE_0043FA80,
                SET_TRANSLATE_00440460,
                OBJECT_0050F5A0,
                UPDATE_PROPERTIES,
            ],
        );
        e.register(PARENT_009611E0, |_, _| ret(1));
    }

    #[test]
    fn clone_3d_loads_and_clones_the_model() {
        let mut e = Engine::new();
        clone_world(&mut e);
        let path = e.mem.alloc(4);
        e.mem.set_u8(path, b'a');
        let setting = e.mem.alloc(0x20);
        let model = object(&mut e, SETTING, &[(0x14, path)]);
        let vtable = e.mem.u32(model);
        e.mem.set_u32(setting + 0x18, vtable);
        let (r, log) = run(&mut e, 0x0081_e440, &args![setting]);
        assert_eq!(r.u32(), 0xc10e);
        assert_eq!(
            calls(&log, LOAD_FILE),
            vec![vec![0x10ad, path, 0, 1, 0, 0, 0]]
        );
        let process = calls(&log, CLONING_PROCESS_CONSTRUCT)[0].clone();
        assert_eq!(process[1], 1.0f32.to_bits());
        assert_eq!(calls(&log, NI_OBJECT_CLONE), vec![vec![0x0de1, process[0]]]);
        assert_eq!(
            calls(&log, SET_ROTATE_0043FA80),
            vec![vec![0xc10e, IDENTITY_ROTATION]]
        );
        assert_eq!(
            calls(&log, SET_TRANSLATE_00440460),
            vec![vec![0xc10e, ZERO_POINT]]
        );
        assert_eq!(calls(&log, OBJECT_0050F5A0), vec![vec![0xc10e]]);
        assert_eq!(
            calls(&log, CLONING_PROCESS_DESTRUCT),
            vec![vec![process[0]]]
        );
        // An empty path: nothing loaded.
        e.mem.set_u8(path, 0);
        let (r, log) = run(&mut e, 0x0081_e460, &args![path]);
        assert_eq!(r.u32(), 0);
        assert!(calls(&log, LOAD_FILE).is_empty());
        // No clone: only the cloning process.
        e.mem.set_u8(path, b'a');
        e.register(NI_OBJECT_CLONE, |_, _| ret(0));
        let (r, log) = run(&mut e, 0x0081_e460, &args![path]);
        assert_eq!(r.u32(), 0);
        assert!(calls(&log, SET_SCALE_00440490).is_empty());
        assert_eq!(calls(&log, CLONING_PROCESS_DESTRUCT).len(), 1);
    }

    #[test]
    fn load_3d_sets_up_the_clone() {
        let mut e = Engine::new();
        clone_world(&mut e);
        let path = e.mem.alloc(4);
        e.mem.set_u8(path, b'a');
        let setting = e.mem.alloc(0x20);
        let model = object(&mut e, SETTING, &[(0x14, path)]);
        let vtable = e.mem.u32(model);
        e.mem.set_u32(setting + 0x18, vtable);
        let this = object(&mut e, PROJECTILE, &[(0x1d0, 0), (0x1f4, 0x9050)]);
        zero(
            &mut e,
            &[
                TEST_00440D80,
                TEST_00440DA0,
                SET_00693EF0,
                SET_3D_00570F70,
                RESET_SIM,
                ADD_ADDON_NODES,
                SET_0056C7D0,
                INIT_ANIMATION,
            ],
        );
        e.register_double(EFFECT_SETTING_004FD400, move |_, _| ret(setting));
        e.register(BASE_FORM_007AF430, |_, _| ret(0xf0));
        e.register(FORM_TEST_0050D4A0, |_, _| ret(1));
        e.register(GET_ORIENTATION, |_, a| ret(a[1]));
        e.register(EXTRA_DATA_NAME_00448A80, |_, _| ret(0x4a4e));
        e.register(GET_EXTRA_DATA, |_, _| ret(0xe7));
        e.register(EXTRA_TEST_00448A40, |_, _| ret(1));
        e.register(REFERENCE_TEST_00448A20, |_, _| ret(0));
        e.register(GET_SCALE, |_, _| st0(1.5));
        let (r, log) = run(&mut e, 0x0081_e570, &args![this, 0u32]);
        assert_eq!(r.u32(), 0xc10e);
        assert_eq!(calls(&log, SET_00693EF0), vec![vec![this, 1]]);
        assert_eq!(calls(&log, SET_3D_00570F70), vec![vec![this, 0xc10e]]);
        assert_eq!(calls(&log, SET_TRANSLATE_00440460)[1], vec![0xc10e, 0x9050]);
        let orientation = calls(&log, GET_ORIENTATION)[0][1];
        assert_eq!(
            calls(&log, SET_ROTATE_0043FA80)[1],
            vec![0xc10e, orientation]
        );
        assert_eq!(calls(&log, RESET_SIM), vec![vec![0xc10e, 1]]);
        assert_eq!(calls(&log, GET_EXTRA_DATA), vec![vec![0xc10e, 0x4a4e]]);
        assert_eq!(calls(&log, ADD_ADDON_NODES), vec![vec![0xc10e]]);
        assert_eq!(calls(&log, SET_0056C7D0), vec![vec![0xc10e, this]]);
        assert_eq!(
            calls(&log, SET_SCALE_00440490)[1],
            vec![0xc10e, 1.5f32.to_bits()]
        );
        // Already with 3D: 0.
        e.register(PROJECTILE + 0x1d0, |_, _| ret(1));
        let (r, log) = run(&mut e, 0x0081_e570, &args![this, 0u32]);
        assert_eq!(r.u32(), 0);
        assert!(calls(&log, LOAD_FILE).is_empty());
        e.register(TEST_00440DA0, |_, _| ret(1));
        let (r, log) = run(&mut e, 0x0081_e570, &args![this, 0u32]);
        assert_eq!(r.u32(), 0);
        assert!(calls(&log, PROJECTILE + 0x1d0).is_empty());
    }

    #[test]
    fn area_display_on_and_off() {
        let mut e = Engine::new();
        let parent = object(&mut e, 0x0380_0000, &[(0xe8, 0)]);
        let node = object(&mut e, NODE, &[(0x9c, 0xa7ea)]);
        let this = object(&mut e, PROJECTILE, &[(0x1d0, node), (0x310, 0)]);
        let controller = object(&mut e, CASTER, &[(0xd0, 0)]);
        e.register_double(GET_CHAR_CONTROLLER, move |_, _| ret(controller));
        e.register(DISPLAY_GEOMETRY_00810660, |_, _| ret(0x6e0));
        zero(
            &mut e,
            &[
                RECURSIVE_SET_PROPERTY_WIREFRAME,
                KILL_DISPLAY_GEOMETRY,
                FIXED_STRING_DESTRUCT,
            ],
        );
        e.register(FIXED_STRING_CONSTRUCT, |_, a| ret(a[0]));
        e.register_double(PARENT_009611E0, move |_, _| ret(parent));
        let (_, log) = run(&mut e, 0x0081_e6c0, &args![this, 0x101u32]);
        assert_eq!(calls(&log, CASTER + 0xd0), vec![vec![controller, 0]]);
        assert_eq!(
            calls(&log, RECURSIVE_SET_PROPERTY_WIREFRAME),
            vec![vec![0x6e0, 1]]
        );
        assert_eq!(calls(&log, PROJECTILE + 0x310), vec![vec![this]]);
        let (_, log) = run(&mut e, 0x0081_e6c0, &args![this, 0u32]);
        assert_eq!(calls(&log, KILL_DISPLAY_GEOMETRY), vec![vec![controller]]);
        let name = calls(&log, FIXED_STRING_CONSTRUCT)[0].clone();
        assert_eq!(name[1], MAGIC_AREA_DISPLAY);
        assert_eq!(calls(&log, NODE + 0x9c), vec![vec![node, name[0]]]);
        assert_eq!(calls(&log, 0x0380_00e8), vec![vec![parent, 0xa7ea]]);
        // Without 3D nothing is looked up.
        e.register(PROJECTILE + 0x1d0, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0081_e730, &args![this]);
        assert!(calls(&log, FIXED_STRING_CONSTRUCT).is_empty());
        assert!(calls(&log, 0x0380_00e8).is_empty());
        e.register(GET_CHAR_CONTROLLER, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0081_e6c0, &args![this, 1u32]);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn save_size_save_load_and_init() {
        let mut e = Engine::new();
        map_globals(&mut e);
        e.mem.set_u32(SAVE_LOAD_GAME, 0x5a1d);
        e.register(MOBILE_OBJECT_SAVE_SIZE, |_, _| ret(0x20));
        for (version, size) in [(0x47u32, 0x2c), (0x48, 0x30), (100, 0x34)] {
            e.register_double(SAVE_VERSION, move |_, _| ret(version));
            assert_eq!(
                e.call(0x0081_e830, &args![0x40u32, 1u32]).u32() & 0xffff,
                size
            );
        }
        let this = e.new_object::<MagicProjectile>();
        e.set(this, MagicProjectile::pCaster, 0xca);
        e.set(this, MagicProjectile::pSpell, 0x5b);
        zero(
            &mut e,
            &[
                MOBILE_OBJECT_SAVE_GAME,
                SAVE_GAME_DATA_OLD,
                MOBILE_OBJECT_LOAD_GAME,
                LOAD_GAME_DATA_OLD,
            ],
        );
        e.register(GET_MAGIC_CASTER_FORM_ID, |_, a| ret(a[0] + 0x100));
        e.register(GET_MAGIC_ITEM_FORM_ID, |_, a| ret(a[0] + 0x200));
        e.register(SAVE_NUMERIC_ID, |e, a| ret(e.mem.u32(a[1])));
        let base = this.addr();
        let (_, log) = run(&mut e, 0x0081_e8c0, &args![this, 9u32]);
        assert_eq!(calls(&log, MOBILE_OBJECT_SAVE_GAME), vec![vec![base, 9]]);
        let ids: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == SAVE_NUMERIC_ID)
            .map(|(_, w)| w[2])
            .collect();
        assert_eq!(ids, vec![4, 4]);
        assert_eq!(
            calls(&log, SAVE_GAME_DATA_OLD),
            vec![
                vec![base, base + 0xa4, 4],
                vec![base, base + 0x90, 4],
                vec![base, base + 0x8c, 4]
            ]
        );
        e.register(SAVE_VERSION, |_, _| ret(0x47));
        let (_, log) = run(&mut e, 0x0081_e8c0, &args![this, 9u32]);
        assert_eq!(calls(&log, SAVE_GAME_DATA_OLD).len(), 1);
        // Load: the ids land in the pointers.
        let mut next = 0x30;
        e.register_double(LOAD_NUMERIC_ID, move |e, a| {
            next += 1;
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        e.register(SAVE_VERSION, |_, _| ret(0x48));
        let (_, log) = run(&mut e, 0x0081_e980, &args![this, 1u32, 2u32]);
        assert_eq!(calls(&log, MOBILE_OBJECT_LOAD_GAME), vec![vec![base, 1, 2]]);
        assert_eq!(e.get(this, MagicProjectile::pCaster), 0x31);
        assert_eq!(e.get(this, MagicProjectile::pSpell), 0x32);
        assert_eq!(
            calls(&log, LOAD_GAME_DATA_OLD),
            vec![vec![base, base + 0xa4, 4], vec![base, base + 0x90, 4]]
        );
        // Init: the pointers from the ids.
        let this = object(&mut e, PROJECTILE, &[(0x1c8, 0)]);
        let this = Ptr::<MagicProjectile>::new(this);
        e.set(this, MagicProjectile::pCaster, 0x31);
        e.set(this, MagicProjectile::pSpell, 0x32);
        e.register(GET_MAGIC_CASTER_BY_NUMERIC_ID, |_, a| ret(a[0] + 0x1000));
        e.register(GET_MAGIC_ITEM_BY_NUMERIC_ID, |_, a| ret(a[0] + 0x2000));
        e.register(GET_COSTLIEST_EFFECT, |_, a| ret(a[0] + 1));
        e.register(GET_AV_EFFECT, |_, a| ret(a[0] + 2));
        e.register(GET_CURRENT_PROCESS_TYPE, |_, _| ret(3));
        zero(
            &mut e,
            &[
                MOBILE_OBJECT_INIT_LOAD_GAME,
                REMOVE_REFERENCE,
                ADD_REFERENCE,
                ADD_FORM_TO_DEFERRED_DELETIONS_LIST,
            ],
        );
        let base = this.addr();
        let (_, log) = run(&mut e, 0x0081_ea40, &args![this, 1u32, 2u32]);
        assert_eq!(e.get(this, MagicProjectile::pCaster), 0x1031);
        assert_eq!(e.get(this, MagicProjectile::pSpell), 0x2032);
        assert_eq!(calls(&log, GET_COSTLIEST_EFFECT), vec![vec![0x203e, 2, 0]]);
        assert_eq!(e.get(this, MagicProjectile::pDominantEffect), 0x203f);
        assert_eq!(e.get(this, MagicProjectile::pAVEffect), 0x2034);
        assert_eq!(calls(&log, PROJECTILE + 0x1c8), vec![vec![base, 0]]);
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            vec![vec![PROCESS_LISTS, base, 3]]
        );
        assert_eq!(
            calls(&log, ADD_REFERENCE),
            vec![vec![PROCESS_LISTS, base, 3, 0, 0, 0]]
        );
        assert!(calls(&log, ADD_FORM_TO_DEFERRED_DELETIONS_LIST).is_empty());
        e.register(GET_MAGIC_CASTER_BY_NUMERIC_ID, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0081_ea40, &args![this, 1u32, 2u32]);
        assert_eq!(
            calls(&log, ADD_FORM_TO_DEFERRED_DELETIONS_LIST),
            vec![vec![0x5a1d, base]]
        );
    }

    /// A projectile with 3D and the player's caster, a world whose ray
    /// cast (slot +0xC8) answers `cast`, and two hits.
    fn wall_world(e: &mut Engine, cast: u32) -> (u32, u32) {
        map_globals(e);
        let player = object(e, PLAYER_CODE, &[(0x1f4, 0)]);
        e.mem.set_u32(PLAYER, player);
        let eye = e.mem.alloc(12);
        for (i, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_u32(eye + 4 * i as u32, value.to_bits());
        }
        e.register_double(PLAYER_CODE + 0x1f4, move |_, _| ret(eye));
        let caster = object(e, CASTER, &[(0x2c, player)]);
        let this = object(e, PROJECTILE, &[(0x1d0, 0x0d0d), (0x1f4, eye), (0x304, 0)]);
        e.register_double(CASTER_KEY_009185E0, move |_, _| ret(caster));
        e.register(HEIGHT_0070C3B0, |_, _| st0(100.0));
        e.register(GET_SCALE, |_, _| st0(0.5));
        e.register(POINT_SUBTRACT_00439EF0, |_, a| ret(a[1]));
        e.register(POINT_ADD_00439E90, |_, a| ret(a[1]));
        e.register(POINT_SCALE_0045BB20, |_, a| ret(a[1]));
        e.register(GET_CHAR_CONTROLLER, |_, _| ret(0));
        e.register(PLAYER_FILTER_00931ED0, |e, a| {
            e.mem.set_u32(a[1], 0xf117);
            ret(a[1])
        });
        zero(
            e,
            &[
                RAY_QUERY_CONSTRUCT,
                SELF_006815C0,
                WORD_CONSTRUCT_008C71B0,
                RAY_SET_FILTER,
                RAY_SET_FROM,
                RAY_SET_TO,
                ALL_HIT_COLLECTOR_CONSTRUCT,
                RAY_SET_COLLECTOR,
                ALL_HIT_COLLECTOR_DESTRUCT,
            ],
        );
        e.register(REFERENCE_CELL_008D6F30, |_, _| ret(0xce11));
        let world = object(e, WORLD, &[(0xc8, cast)]);
        e.register_double(CELL_WORLD_004543C0, move |_, _| ret(world));
        e.register(COLLECTOR_HITS_00460140, |_, _| ret(0x4175));
        e.register(HIT_COUNT_00726070, |_, _| ret(2));
        e.register(HIT_AT_004A46B0, |_, a| ret(0x100 + a[1]));
        // Each copied hit: its collidable (+0x50) names the hit.
        e.register(COPY_HIT_00696C30, |e, a| {
            e.mem.set_u32(a[0] + 0x50, a[1]);
            e.mem.set_u32(a[0] + 0x10, 0.5f32.to_bits());
            Ret::default()
        });
        e.register(GET_AV_OBJECT_FOR_COLLIDABLE, |_, a| ret(a[0] + 0x1000));
        e.register(CONTACT_OBJECT_004B59F0, |_, a| ret(a[0]));
        e.register(CONTACT_OBJECT_004B5A80, |_, a| ret(a[0] + 0x2000));
        (this, player)
    }

    #[test]
    fn shot_behind_wall_reports_the_first_wall_hit() {
        let mut e = Engine::new();
        let (this, _) = wall_world(&mut e, 1);
        let wall = object(&mut e, REFERENCE, &[(0x100, 0)]);
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, _| ret(wall));
        let (_, log) = run(&mut e, 0x0081_eba0, &args![this]);
        // Eye z: 3 + 0.5 * 100.
        let from = calls(&log, RAY_SET_FROM)[0][1];
        assert_eq!(e.mem.u32(from + 8), 53.0f32.to_bits());
        assert_eq!(calls(&log, RAY_SET_FILTER)[0][1], 0xf117);
        let reports = calls(&log, PROJECTILE + 0x304);
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0][0], this);
        assert_eq!(reports[0][4..], [0x2100, wall, 0]);
        assert_eq!(calls(&log, POINT_SCALE_0045BB20)[0][2], 0.5f32.to_bits());
        assert_eq!(calls(&log, ALL_HIT_COLLECTOR_DESTRUCT).len(), 1);
        // Hits on actors are skipped: no report after both hits.
        let actor = object(&mut e, REFERENCE + 0x1000, &[(0x100, 1)]);
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, _| ret(actor));
        let (_, log) = run(&mut e, 0x0081_eba0, &args![this]);
        assert!(calls(&log, PROJECTILE + 0x304).is_empty());
        assert_eq!(calls(&log, HIT_AT_004A46B0).len(), 2);
    }

    #[test]
    fn shot_behind_wall_needs_the_players_projectile_and_a_cast() {
        let mut e = Engine::new();
        let (this, _) = wall_world(&mut e, 0);
        let (_, log) = run(&mut e, 0x0081_eba0, &args![this]);
        assert!(calls(&log, HIT_AT_004A46B0).is_empty());
        assert_eq!(calls(&log, ALL_HIT_COLLECTOR_DESTRUCT).len(), 1);
        e.register(CASTER + 0x2c, |_, _| ret(0x0bad));
        let (_, log) = run(&mut e, 0x0081_eba0, &args![this]);
        assert!(calls(&log, RAY_QUERY_CONSTRUCT).is_empty());
        e.register(CASTER_KEY_009185E0, |_, _| ret(0));
        e.register(PROJECTILE + 0x1d0, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0081_eba0, &args![this]);
        assert!(calls(&log, RAY_QUERY_CONSTRUCT).is_empty());
    }
}

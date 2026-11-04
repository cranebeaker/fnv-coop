//! `fallout/magic/magicboltprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Constructor-like call on the local vector (`006815c0`, `this` = the local, returns `this`).
const INIT_LOCAL: u32 = 0x0068_15c0;
/// `bhkCharacterController::GetPosition_ov2` (Xbox PDB, `00c6e300`): writes the position
/// vector (16 bytes) through its one stack argument.
const GET_POSITION_OV2: u32 = 0x00c6_e300;
/// Point copy of `tes.cpp` (`00458620`, cdecl `(destination, source)`, three floats).
const COPY_POINT: u32 = 0x0045_8620;

// Translated from 00812b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00812b00 - the /GS cookie check (00ec408c) is not modeled.
/// `bhkCharacterController::GetPosition` (Xbox PDB; the engine map files this address under
/// `magicboltprojectile.cpp`): `this` is the controller, `out` the one stack argument. Runs
/// `006815c0` on a local, has `GetPosition_ov2` (`00c6e300`) fill it, and copies its first
/// three floats to `out` with `00458620` (a vector copy that leaves the fourth word alone).
pub fn bhk_character_controller_get_position(e: &mut Engine, this: Ptr, out: Ptr) {
    e.with_stack(0x20, |e, local| {
        e.call(INIT_LOCAL, &args![local]);
        e.call(GET_POSITION_OV2, &args![this, local]);
        e.call(COPY_POINT, &args![out, local]);
    });
}

// Translated from 008133b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float getter (unnamed in the engine map; its class is not confirmed,
/// several units call it): answers the float at `+0x144` of `this` in
/// `ST0` (`FLD dword`: a signalling NaN is quieted on the load).
pub fn fn_008133b0(e: &mut Engine, this: Ptr) -> f64 {
    x87_load32(e.mem.u32(this.addr() + 0x144))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00812b00, bhk_character_controller_get_position(Ptr, Ptr)),
        entry!(0x008133b0, fn_008133b0(Ptr) -> f64),
        entry!(
            0x0081_1980,
            magic_bolt_projectile_constructor(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(0x0081_1e00, fn_00811e00(u32)),
        entry!(0x0081_1e10, magic_bolt_projectile_destructor(Ptr)),
        entry!(0x0081_2170, fn_00812170(Ptr, u32) -> Ptr),
        entry!(0x0081_21a0, fn_008121a0(Ptr)),
        entry!(0x0081_2220, magic_bolt_projectile_init_3d(Ptr)),
        entry!(0x0081_2700, fn_00812700(Ptr, f32, u32)),
        entry!(0x0081_2890, magic_bolt_projectile_post_process(Ptr) -> bool),
        entry!(0x0081_2b60, fn_00812b60(Ptr, f32)),
        entry!(0x0081_3370, fn_00813370(Ptr) -> bool),
        entry!(0x0081_33d0, fn_008133d0(Ptr, u32)),
        entry!(0x0081_33f0, fn_008133f0(Ptr, u32, u32, u32, u32, u32, u8)),
        entry!(0x0081_39f0, fn_008139f0(Ptr, u32, u32)),
        entry!(0x0081_3ea0, fn_00813ea0(Ptr) -> Ptr),
        entry!(0x0081_3f30, fn_00813f30(Ptr)),
        entry!(0x0081_4590, fn_00814590(Ptr, u32) -> u16),
        entry!(0x0081_4670, fn_00814670(Ptr, u32)),
        entry!(0x0081_4870, fn_00814870(Ptr, u32, u32)),
        entry!(0x0081_4aa0, fn_00814aa0(Ptr) -> Ptr),
        entry!(
            0x0081_4ad0,
            magic_bolt_projectile_init_load_game(Ptr, u32, u32)
        ),
        entry!(
            0x0081_a760,
            magic_bolt_projectile_collide(Ptr, u32, u32, u32, u32, u32, u8)
        ),
    ]
}

#[cfg(test)]
mod getter_tests {
    use super::*;

    #[test]
    fn answers_the_float_at_0x144() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x148);
        e.mem.set_f32(object + 0x144, 3.25);
        assert_eq!(e.call(0x0081_33b0, &args![object]).f64(), 3.25);
        e.mem.set_u32(object + 0x144, 0x7f80_0001);
        let value = e.call(0x0081_33b0, &args![object]).f64();
        assert_eq!((value as f32).to_bits(), 0x7fc0_0001);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_position_is_copied_through_a_local() {
        let mut e = Engine::new();
        e.register(INIT_LOCAL, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(GET_POSITION_OV2, |e, a| {
            for (i, v) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, v);
            }
            Ret::default()
        });
        e.register(COPY_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
        let controller = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 12, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0081_2b00, &args![controller, out]);
        let log = e.call_log.take().unwrap();
        let of = |address: u32| log.iter().filter(|c| c.0 == address).count();
        assert_eq!(of(INIT_LOCAL), 1);
        assert_eq!(of(GET_POSITION_OV2), 1);
        assert_eq!(of(COPY_POINT), 1);
        let position_call = log.iter().find(|c| c.0 == GET_POSITION_OV2).unwrap();
        assert_eq!(position_call.1[0], controller);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.f32(out + 4), 2.0);
        assert_eq!(e.mem.f32(out + 8), 3.0);
        assert_eq!(e.mem.u32(out + 12), 0x1234);
    }
}

// ---------------------------------------------------------------------------
// MagicBoltProjectile (Xbox PDB): the bolt projectile's own functions.
// ---------------------------------------------------------------------------

use crate::units::fallout_magic::magicballprojectile as ball;

/// `MagicBoltProjectile`'s two vtables (`0107a8f4` at +0, `0107a8e8` at +0x18).
const MAGIC_BOLT_PROJECTILE_VTABLE: u32 = 0x0107_a8f4;
const MAGIC_BOLT_PROJECTILE_SECOND_VTABLE: u32 = 0x0107_a8e8;
/// `MagicProjectile::MagicProjectile` (engine map, `0081dd00`, `RET 0x28`) and
/// `~MagicProjectile` (`0081de70`).
const MAGIC_PROJECTILE_CONSTRUCTOR: u32 = 0x0081_dd00;
const MAGIC_PROJECTILE_DESTRUCTOR: u32 = 0x0081_de70;
/// `MagicProjectile::Clone3D` (engine map, `0081e440`, `cdecl`, one word),
/// `CheckShotBehindWall` (`0081eba0`), `SaveGame` (`0081e8c0`, `RET 4`), the load body
/// (`0081e980`, `RET 8`), `InitLoadGame` (`0081ea40`, `RET 8`) and the save size body
/// (`0081e830`, `RET 4`).
const MAGIC_PROJECTILE_CLONE_3D: u32 = 0x0081_e440;
const CHECK_SHOT_BEHIND_WALL: u32 = 0x0081_eba0;
const MAGIC_PROJECTILE_SAVE_GAME: u32 = 0x0081_e8c0;
const MAGIC_PROJECTILE_LOAD_GAME: u32 = 0x0081_e980;
const MAGIC_PROJECTILE_INIT_LOAD_GAME: u32 = 0x0081_ea40;
const MAGIC_PROJECTILE_SAVE_SIZE: u32 = 0x0081_e830;
/// `NiPointer` helpers (`ECX` = the slot): `00633c90` constructs from a pointer (`RET 4`),
/// `0066b0d0` assigns (`RET 4`), `0045cec0` destroys, `00559450` reads.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
const POINTER_GET: u32 = 0x0055_9450;
/// The sound handle at +0xC8: constructor `0041a250`, `IsValid` `00ad8ce0`, `Stop`
/// `00ad88f0`, `Release` `00ad8d10`, assignment `00418900`, destructor `00483710`,
/// `SetPosition` `00ad8b60` (three `float`s), `SetObjectToFollow` `00ad8f20` (`RET 4`),
/// `Play` `00ad8830` (`RET 4`).
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
const SOUND_HANDLE_SET_POSITION: u32 = 0x00ad_8b60;
const SOUND_HANDLE_FOLLOW: u32 = 0x00ad_8f20;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// `BSAudio::QInstance` (`00453a70`) and `BSAudio::GetSoundHandleByNumericID` (`00ad73b0`,
/// `RET 0xC`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_ID: u32 = 0x00ad_73b0;
/// `004fd400` the base form; `004fd3c0` and `008ce390` the projectile form's flight and
/// impact sounds (meanings not confirmed); `0084e3a0` a form's id.
const BASE_FORM: u32 = 0x004f_d400;
const PROJECTILE_FLIGHT_SOUND: u32 = 0x004f_d3c0;
const PROJECTILE_IMPACT_SOUND: u32 = 0x008c_e390;
const FORM_ID: u32 = 0x0084_e3a0;
/// `009b0bd0`: the projectile form's speed (`ST0`).
const PROJECTILE_SPEED: u32 = 0x009b_0bd0;
/// `00403e20` (a `float` setting's value address) and `0043d4d0` (an integer setting's).
const SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
/// Settings: the speed scale (`011d16f8`), the running-total part (`011d1758`), the
/// value given to [`fn_00811e00`] (`011d16ec`), the height added for the player (`011d1ae4`),
/// the distance limit (`011d1d00`), `fMagicAreaScale` / `Max` / `Min`, an integer limit
/// (`011d1afc`).
const SPEED_SETTING: u32 = 0x011d_16f8;
const COUNT_SETTING: u32 = 0x011d_1758;
const GLOBAL_SETTING: u32 = 0x011d_16ec;
const PLAYER_HEIGHT_SETTING: u32 = 0x011d_1ae4;
const DISTANCE_SETTING: u32 = 0x011d_1d00;
const AREA_SCALE: u32 = 0x011d_1bbc;
const AREA_SCALE_MAX: u32 = 0x011d_1acc;
const AREA_SCALE_MIN: u32 = 0x011d_15a4;
const SEGMENT_LIMIT_SETTING: u32 = 0x011d_1afc;
/// The word [`fn_00811e00`] stores (`011ae61c`) and a byte that stops the aiming
/// (`011dc55c`).
const BOLT_GLOBAL: u32 = 0x011a_e61c;
const AIM_DISABLED: u32 = 0x011d_c55c;
/// The player (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `009185e0` the projectile's caster; `MobileObject::GetCharController` (`009306d0`).
const PROJECTILE_CASTER: u32 = 0x0091_85e0;
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `008d6f30` the cell, `TESObjectCELL::AddReference` (`00548230`, `RET 8`), `004543c0` the
/// cell's world, `TESObjectCELL::GetWaterHeight` (`005471e0`, `ST0`), `005497a0` the cell's
/// root node.
const PARENT_CELL: u32 = 0x008d_6f30;
const CELL_ADD_REFERENCE: u32 = 0x0054_8230;
const CELL_HAVOK_WORLD: u32 = 0x0045_43c0;
const CELL_WATER_HEIGHT: u32 = 0x0054_71e0;
const CELL_ROOT_NODE: u32 = 0x0054_97a0;
/// `0045cd60`: the word of the object at +0x68 that stops the controller setup.
const OBJECT_BUSY: u32 = 0x0045_cd60;
/// `00570f70` (`RET 4`), `TESObjectREFR::SetLocationOnReference` (`00575830`, `RET 4`) and
/// `00436aa0` (the reference's position, +0x30).
const SET_3D: u32 = 0x0057_0f70;
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
const REFERENCE_POSITION: u32 = 0x0043_6aa0;
/// The player's `00950bb0` (`RET 4`), `00456610` (a `bool` of what it answers) and
/// `004eaf60` (a `bool` of the player).
const PLAYER_NODE: u32 = 0x0095_0bb0;
const NODE_FLAG: u32 = 0x0045_6610;
const PLAYER_FLAG: u32 = 0x004e_af60;
/// The process lists (`011e0e80`): `004610f0` its list at +0x70, `005ae3d0` adds (`RET 4`),
/// `00905330` removes (`RET 4`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
const PROCESS_LIST: u32 = 0x0046_10f0;
const LIST_ADD: u32 = 0x005a_e3d0;
const LIST_REMOVE: u32 = 0x0090_5330;
/// `0080fb00` (the controller's proxy velocity), `0080fae0` / `0080fc90` (the running
/// total, `magicballprojectile.rs`), `00810a80`, `00810f90`, `00810ff0`, `00810fd0`,
/// `00810550`, `00810510`, `008105c0`, `00810530`, `00810590`, `00810570`, `008106b0`,
/// `008105e0`, `00810610`, `00810640`, `00810660` (the same unit).
const SET_PROXY_VELOCITY: u32 = 0x0080_fb00;
const RUNNING_TOTAL_ADD: u32 = 0x0080_fae0;
const RUNNING_TOTAL_TAKE: u32 = 0x0080_fc90;
const CONTROLLER_FLOAT: u32 = 0x0081_0a80;
const FILTER_FLAG: u32 = 0x0081_0f90;
const SET_COLLISION_FILTER: u32 = 0x0081_0ff0;
const PROJECTILE_KIND: u32 = 0x0081_0fd0;
const INFO_SET_FLOAT_8C: u32 = 0x0081_0550;
const INFO_SET_POSITION: u32 = 0x0081_0510;
const INFO_SET_SHAPE: u32 = 0x0081_05c0;
const INFO_SET_BYTE: u32 = 0x0081_0530;
const INFO_SET_FILTER: u32 = 0x0081_0590;
const INFO_SET_WORD: u32 = 0x0081_0570;
const CONTROLLER_SET_FLOAT_548: u32 = 0x0081_06b0;
const CONTROLLER_FLAG_80000: u32 = 0x0081_05e0;
const CONTROLLER_FLAG_100000: u32 = 0x0081_0610;
const CONTROLLER_SET_OWNER: u32 = 0x0081_0640;
const CONTROLLER_NODE: u32 = 0x0081_0660;
/// `00621270` (`RET 4`): a flag mask of the controller's object (+0x410); `0070c440`
/// (`RET 4`): the controller's filter word.
const CONTROLLER_HAS_FLAG: u32 = 0x0062_1270;
const CONTROLLER_FILTER: u32 = 0x0070_c440;
/// The controller construction (see [`fn_00813f30`]).
const CONTROLLER_INFO_CONSTRUCT: u32 = 0x00c6_da50;
const CONTROLLER_INFO_DESTRUCT: u32 = 0x00c6_de10;
const CONTROLLER_INFO_SET_NODE: u32 = 0x008a_c890;
const CONTROLLER_INFO_SET_WORLD: u32 = 0x0085_7230;
const CONTROLLER_INFO_SET_FLOAT: u32 = 0x0040_7860;
const ALLOCATE_OBJECT: u32 = 0x00aa_13e0;
const SPHERE_SHAPE_CONSTRUCTOR: u32 = 0x0056_e950;
const SAVE_LOAD_FLAG: u32 = 0x0047_c850;
const ALLOCATE_CONTROLLER: u32 = 0x0055_3300;
const CHARACTER_CONTROLLER_CONSTRUCTOR: u32 = 0x0080_6eb0;
const CONTROLLER_FLAG_SOURCE: u32 = 0x0052_5420;
const CONTROLLER_SET_FLOAT: u32 = 0x0057_b1d0;
const CONTROLLER_WORLD: u32 = 0x0062_1ad0;
const CONTROLLER_SET_WORLD: u32 = 0x00c7_0c90;
const CONTROLLER_SET_NODE: u32 = 0x00c6_d7a0;
const CONTROLLER_SET_POSITION: u32 = 0x0056_20e0;
const WORLD_FLAG: u32 = 0x004b_ae00;
const SET_WIREFRAME: u32 = 0x00b6_b910;
const WORLD_SET_MOTION: u32 = 0x00c6_a350;
const WORLD_REMOVE_OBJECTS: u32 = 0x00c6_9ee0;
const WORLD_UPDATE_POSITION: u32 = 0x00c6_9f50;
const FILTER_LAYER: u32 = 0x004a_3a20;
/// `009611e0`: the word at +0x18 of a node (its parent).
const PARENT_NODE: u32 = 0x0096_11e0;
/// `00401000` / `00401030`: `operator new` / `operator delete`.
const ALLOCATE: u32 = 0x0040_1000;
const DEALLOCATE: u32 = 0x0040_1030;
/// `NiNode` (0xAC bytes through `00aa13e0`, constructor `00a5ecb0`, `RET 4`).
const NODE_CONSTRUCTOR: u32 = 0x00a5_ecb0;
/// `004f00e0` (`RET 4`): places a node at a point; node setters `00440460` (translation)
/// and `0043fa80` (rotation); `0043c490` (the node's world translation, +0x58); `0045bb80`
/// (a node's position); `00a59c60` (`RET 4`) updates a node with `0043d410`'s record.
const NODE_PLACE: u32 = 0x004f_00e0;
const NODE_SET_TRANSLATE: u32 = 0x0044_0460;
const NODE_SET_ROTATE: u32 = 0x0043_fa80;
const NODE_WORLD_TRANSLATE: u32 = 0x0043_c490;
const NODE_POSITION: u32 = 0x0045_bb80;
const UPDATE_DATA_INIT: u32 = 0x0043_d410;
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// The node's `BSXFlags` extra data: `00448a80` its key, `NiObjectNET::GetExtraData`
/// (`00a5bdd0`, `RET 4`), `AddExtraData` (`00a5bc40`, `RET 8`), `BSXFlags::BSXFlags`
/// (`00c42f70`, 0x10 bytes), `00519230` (`RET 8`).
const EXTRA_DATA_KEY: u32 = 0x0044_8a80;
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const ADD_EXTRA_DATA: u32 = 0x00a5_bc40;
const BSX_FLAGS_CONSTRUCTOR: u32 = 0x00c4_2f70;
const BSX_FLAGS_SET: u32 = 0x0051_9230;
/// `00430830` (the reference's angles), `006815c0` (answers `this`), `00a59660` (a matrix
/// from three angles), `NiMatrix3::FromEulerAnglesZYX` (`00a59780`), the `float` `atan2`
/// (`004b5550`, `cdecl`, two `float`s).
const ANGLES: u32 = 0x0043_0830;
const SELF: u32 = 0x0068_15c0;
const MATRIX_FROM_ANGLES: u32 = 0x00a5_9660;
const MATRIX_FROM_EULER_ZYX: u32 = 0x00a5_9780;
const ATAN2: u32 = 0x004b_5550;
/// The point helpers `00416870` (`RET 0xC`), `00439ef0` (a difference into an out point,
/// `RET 8`), `00457990` (the length, `ST0`), `00414430` (sets four `float`s, `RET 0x10`),
/// `005532a0` (a four-float vector, `RET 0x10`).
const POINT_SET: u32 = 0x0041_6870;
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const POINT_LENGTH: u32 = 0x0045_7990;
const COLOR_SET: u32 = 0x0041_4430;
const VECTOR4_SET: u32 = 0x0055_32a0;
/// `BoltShader::Start` (`00bc18d0`, `cdecl`: node, flag), `BoltShader::Stop` (`00bc1470`,
/// `cdecl`: two words) and the shader's setters (`thiscall`): `00803a70` (an integer),
/// `00821620`, `00507780`, `005f9b10`, `00803a90`, `00803b30`, `005077a0`, `00803b50` (a
/// `float` each), `00803ab0` / `00803ae0` (four `float`s), `008126e0` (an integer),
/// `00803a50` (a word), `00bc2360` (no word), `00bc2740` / `00bc28b0` (`RET 0x10`),
/// `0051b860` (`ST0`) and `00ee8990` (a `bool`).
const BOLT_SHADER_START: u32 = 0x00bc_18d0;
const BOLT_SHADER_STOP: u32 = 0x00bc_1470;
const SHADER_SET_INT_A70: u32 = 0x0080_3a70;
const SHADER_SET_821620: u32 = 0x0082_1620;
const SHADER_SET_507780: u32 = 0x0050_7780;
const SHADER_SET_5F9B10: u32 = 0x005f_9b10;
const SHADER_SET_A90: u32 = 0x0080_3a90;
const SHADER_SET_COLOR_AB0: u32 = 0x0080_3ab0;
const SHADER_SET_COLOR_AE0: u32 = 0x0080_3ae0;
const SHADER_SET_B30: u32 = 0x0080_3b30;
const SHADER_SET_INT_8126E0: u32 = 0x0081_26e0;
const SHADER_SET_5077A0: u32 = 0x0050_77a0;
const SHADER_SET_B50: u32 = 0x0080_3b50;
const SHADER_SET_WORD_A50: u32 = 0x0080_3a50;
const SHADER_FINISH: u32 = 0x00bc_2360;
const SHADER_UPDATE_SEGMENT: u32 = 0x00bc_2740;
const SHADER_UPDATE: u32 = 0x00bc_28b0;
const SHADER_FLOAT: u32 = 0x0051_b860;
const SHADER_DONE: u32 = 0x00ee_8990;
/// `00812870` (the `float` at +0xA4, `ST0`), `006ca4e0` (the `float` at +0x90, `ST0`),
/// `008d8b80` (the word at +0x140), `00440d80` (a `bool` that stops the update), `004587d0`
/// (`cdecl`, a `float` in `ST0`).
const FLOAT_A4: u32 = 0x0081_2870;
const FLOAT_90: u32 = 0x006c_a4e0;
const WORD_140: u32 = 0x008d_8b80;
const REFERENCE_FLAG: u32 = 0x0044_0d80;
const HEIGHT_OFFSET: u32 = 0x0045_87d0;
/// `008ace90`: a `bool` of the actor that stops the hit.
const ACTOR_STOPS_EXPLOSION: u32 = 0x008a_ce90;
/// `00586150` the effect item, `EffectItem::GetArea` (`00404420`).
const PROJECTILE_EFFECT_ITEM: u32 = 0x0058_6150;
const EFFECT_ITEM_GET_AREA: u32 = 0x0040_4420;
/// `00461130` (a node's transform at +0x68), `004b4500` (`RET 8`: the transform applied to
/// a point), `006a9540` (+0x34), the point at `011a9478`, `0081f000` (`RET 0x1C`).
const NODE_TRANSFORM: u32 = 0x0046_1130;
const TRANSFORM_APPLY: u32 = 0x004b_4500;
const NODE_TRANSFORM_34: u32 = 0x006a_9540;
const EXPLOSION_OFFSET: u32 = 0x011a_9478;
const PROJECTILE_EXPLODE_AT: u32 = 0x0081_f000;
/// `MagicCaster::FindTargets_ov2` (`00817160`, `RET 0x28`).
const FIND_TARGETS_OV2: u32 = 0x0081_7160;
/// `006890b0` (`cdecl`, twelve words: a temporary effect) and `00689920` (`RET 4`).
const MAKE_TEMP_EFFECT: u32 = 0x0068_90b0;
const TEMP_EFFECT_SET_IDLE: u32 = 0x0068_9920;
/// `"Bip01 Spine2"`, `"SpecialIdle_AreaEffect"`; fixed strings (`00438170` / `004381b0`).
const TEXT_SPINE: u32 = 0x0101_f618;
const TEXT_SPECIAL_IDLE_AREA_EFFECT: u32 = 0x0102_054c;
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// Save/load: `TESForm::SaveGameDataOLD` (`00484ce0`), `LoadGameDataOLD` (`00484d00`),
/// `SaveNumericID` (`00484d20`), `LoadNumericID` (`00484d40`) (`RET 8` each); the manager
/// (`011de45c`), its version (`008df040`), word save / load (`008579b0` / `008579e0`), a
/// reserved count slot (`00825c00`); `0056d230` (`RET 4`, the node's rotation as four
/// `float`s); `TESObjectREFR::FindReferenceFor3D` (`0056f930`, `cdecl`).
const SAVE_DATA: u32 = 0x0048_4ce0;
const LOAD_DATA: u32 = 0x0048_4d00;
const SAVE_ID: u32 = 0x0048_4d20;
const LOAD_ID: u32 = 0x0048_4d40;
const SAVE_LOAD_MANAGER: u32 = 0x011d_e45c;
const SAVE_VERSION: u32 = 0x008d_f040;
const SAVE_WORD: u32 = 0x0085_79b0;
const LOAD_WORD: u32 = 0x0085_79e0;
const RESERVE_COUNT: u32 = 0x0082_5c00;
const NODE_ROTATION: u32 = 0x0056_d230;
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `004839c0` (`cdecl`: a form id to a form) and the RTTI type descriptors `.?AVTESForm@@`
/// and `.?AVTESObjectREFR@@`.
const LOOKUP_FORM: u32 = 0x0048_39c0;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// Constants: `3.4028235e38` (`float`, `0107a8b0`), the default point (`011f426c`, three
/// words), `0.5` (`double`, `01011588`), `0.0` (`double`, `01012060`), `3.0` (`double`,
/// `01021928`), `8.0` (`double`, `010240b0`), `-3.4028235e38` (`float`, `01015f5c`), `0.01`
/// (`float`, `01013ea4`), `5.0` (`float`, `0101712c`), the four words at `011a9ea4`.
const FLOAT_MAX: u32 = 0x0107_a8b0;
const DEFAULT_POINT: u32 = 0x011f_426c;
const HALF: u32 = 0x0101_1588;
const ZERO: u32 = 0x0101_2060;
const THREE: u32 = 0x0102_1928;
const EIGHT: u32 = 0x0102_40b0;
const LOWEST_FLOAT: u32 = 0x0101_5f5c;
const HUNDREDTH: u32 = 0x0101_3ea4;
const FIVE: u32 = 0x0101_712c;
const LEGACY_ROTATION: u32 = 0x011a_9ea4;

/// A shader configuration: the settings the bolt shader's setters read.
struct ShaderSettings {
    int_a70: u32,
    value_821620: u32,
    value_507780: u32,
    value_5f9b10: u32,
    value_a90: u32,
    color_ab0: [u32; 3],
    color_ae0: [u32; 3],
    value_b30: u32,
    int_8126e0: u32,
    value_5077a0: u32,
    value_b50: u32,
    word_a50: u32,
}

/// The settings [`magic_bolt_projectile_init_3d`] uses.
const INIT_3D_SHADER: ShaderSettings = ShaderSettings {
    int_a70: 0x011d_186c,
    value_821620: 0x011d_1628,
    value_507780: 0x011d_1bec,
    value_5f9b10: 0x011d_1610,
    value_a90: 0x011d_1b2c,
    color_ab0: [0x011d_1be0, 0x011d_1908, 0x011d_18f0],
    color_ae0: [0x011d_1c4c, 0x011d_19d8, 0x011d_1cdc],
    value_b30: 0x011d_174c,
    int_8126e0: 0x011d_1800,
    value_5077a0: 0x011d_16c8,
    value_b50: 0x011d_17c4,
    word_a50: 1,
};

/// The settings [`fn_008139f0`] uses for a chained segment.
const SEGMENT_SHADER: ShaderSettings = ShaderSettings {
    int_a70: 0x011d_1a48,
    value_821620: 0x011d_168c,
    value_507780: 0x011d_17a0,
    value_5f9b10: 0x011d_1610,
    value_a90: 0x011d_1b2c,
    color_ab0: [0x011d_1be0, 0x011d_1908, 0x011d_18f0],
    color_ae0: [0x011d_1c4c, 0x011d_19d8, 0x011d_1cdc],
    value_b30: 0x011d_16b0,
    int_8126e0: 0x011d_1b50,
    value_5077a0: 0x011d_18c0,
    value_b50: 0x011d_1974,
    word_a50: 0,
};

fn setting_int(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_INT_VALUE, &args![setting]).u32();
    e.mem.u32(value)
}

/// A four-float colour (`00414430` on a local with the three settings, read last first,
/// and `1.0`), passed on by value.
fn shader_color(e: &mut Engine, settings: [u32; 3]) -> [u32; 4] {
    let third = ball::setting_bits(e, settings[2]);
    let second = ball::setting_bits(e, settings[1]);
    let first = ball::setting_bits(e, settings[0]);
    e.with_stack(0x10, |e, local| {
        let color = e
            .call(COLOR_SET, &args![local, first, second, third, 1.0f32])
            .u32();
        [
            e.mem.u32(color),
            e.mem.u32(color.wrapping_add(4)),
            e.mem.u32(color.wrapping_add(8)),
            e.mem.u32(color.wrapping_add(12)),
        ]
    })
}

/// Configures a bolt shader with the settings; `shader` answers the shader before each
/// setter (the exe reads it again each time in `Init3D`).
fn configure_shader(
    e: &mut Engine,
    settings: &ShaderSettings,
    shader: &dyn Fn(&mut Engine) -> u32,
) {
    let value = setting_int(e, settings.int_a70);
    let target = shader(e);
    e.call(SHADER_SET_INT_A70, &args![target, value]);
    for (setter, setting) in [
        (SHADER_SET_821620, settings.value_821620),
        (SHADER_SET_507780, settings.value_507780),
        (SHADER_SET_5F9B10, settings.value_5f9b10),
        (SHADER_SET_A90, settings.value_a90),
    ] {
        let value = ball::setting_bits(e, setting);
        let target = shader(e);
        e.call(setter, &args![target, value]);
    }
    let color = shader_color(e, settings.color_ab0);
    let target = shader(e);
    e.call(
        SHADER_SET_COLOR_AB0,
        &args![target, color[0], color[1], color[2], color[3]],
    );
    let color = shader_color(e, settings.color_ae0);
    let target = shader(e);
    e.call(
        SHADER_SET_COLOR_AE0,
        &args![target, color[0], color[1], color[2], color[3]],
    );
    let value = ball::setting_bits(e, settings.value_b30);
    let target = shader(e);
    e.call(SHADER_SET_B30, &args![target, value]);
    let value = setting_int(e, settings.int_8126e0);
    let target = shader(e);
    e.call(SHADER_SET_INT_8126E0, &args![target, value]);
    for (setter, setting) in [
        (SHADER_SET_5077A0, settings.value_5077a0),
        (SHADER_SET_B50, settings.value_b50),
    ] {
        let value = ball::setting_bits(e, setting);
        let target = shader(e);
        e.call(setter, &args![target, value]);
    }
    let target = shader(e);
    e.call(SHADER_SET_WORD_A50, &args![target, settings.word_a50]);
    let target = shader(e);
    e.call(SHADER_FINISH, &args![target]);
}

/// Points the matrix along the difference at `difference` (three `float`s; its z is
/// cleared): `FromEulerAnglesZYX(atan2(x, y), 0.0, -atan2(z, length of (x, y, 0)))`.
fn aim_matrix(e: &mut Engine, matrix: u32, difference: u32) {
    e.call(SELF, &args![matrix]);
    let z = x87_quiet_bits(e.mem.u32(difference.wrapping_add(8)));
    e.mem.set_u32(difference.wrapping_add(8), 0.0f32.to_bits());
    let length = x87_store32(e.call(POINT_LENGTH, &args![difference]).f64());
    let pitch = e.call(ATAN2, &args![z, x87_quiet_bits(length)]).f64();
    let pitch = x87_store32(-pitch);
    let x = x87_quiet_bits(e.mem.u32(difference));
    let y = x87_quiet_bits(e.mem.u32(difference.wrapping_add(4)));
    let yaw = x87_store32(e.call(ATAN2, &args![x, y]).f64());
    e.call(MATRIX_FROM_EULER_ZYX, &args![matrix, yaw, 0.0f32, pitch]);
}

fn read_point(e: &mut Engine, point: u32) -> [u32; 3] {
    [
        e.mem.u32(point),
        e.mem.u32(point.wrapping_add(4)),
        e.mem.u32(point.wrapping_add(8)),
    ]
}

fn write_point(e: &mut Engine, point: u32, words: [u32; 3]) {
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(point.wrapping_add(4 * i as u32), *word);
    }
}

/// The impact sound at the projectile's position (`008ce390` of the projectile form).
fn play_impact_sound(e: &mut Engine, this: Ptr) {
    let form = e.call(BASE_FORM, &args![this]).u32();
    let sound = e.call(PROJECTILE_IMPACT_SOUND, &args![form]).u32();
    let id = e.call(FORM_ID, &args![sound]).u32();
    e.with_stack(0x0c, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        e.call(GET_SOUND_HANDLE_BY_ID, &args![audio, handle, id, 0x102u32]);
        let [x, y, z] = ball::position_words(e, this);
        e.call(
            SOUND_HANDLE_SET_POSITION,
            &args![
                handle,
                x87_quiet_bits(x),
                x87_quiet_bits(y),
                x87_quiet_bits(z)
            ],
        );
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

/// Sets bit `0x4000` of the controller's filter word (`0070c440`, `00810f90`, `00810ff0`).
fn flag_controller_filter(e: &mut Engine, this: Ptr) {
    e.with_stack(4, |e, filter| {
        let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
        e.call(CONTROLLER_FILTER, &args![controller, filter]);
        e.call(FILTER_FLAG, &args![filter, 1u32]);
        let word = e.mem.u32(filter.addr());
        let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
        e.call(SET_COLLISION_FILTER, &args![controller, word]);
    });
}

/// The target reference's `"Bip01 Spine2"` node (slot +0x9C of its node), or null.
fn spine_node(e: &mut Engine, node: u32) -> u32 {
    e.with_stack(4, |e, name| {
        let name_pointer = e
            .call(FIXED_STRING_CONSTRUCT, &args![name, TEXT_SPINE])
            .u32();
        let child = e.vcall(node, 0x9c, &args![name_pointer]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![name]);
        child
    })
}

/// Makes the flight sound handle (+0xC8) for the sound id, places it at the reference's
/// position (`00436aa0`, or the slot +0x1F4 point when `from_slot`), has it follow the
/// node at +0xC0 and plays it (`Play(1)`).
fn start_flight_sound(e: &mut Engine, this: Ptr, id: u32, from_slot: bool) {
    let base = this.addr();
    e.with_stack(0x0c, |e, temporary| {
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        let made = e
            .call(
                GET_SOUND_HANDLE_BY_ID,
                &args![audio, temporary, id, 0x102u32],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![base.wrapping_add(0xc8), made]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![temporary]);
    });
    let [x, y, z] = if from_slot {
        ball::position_words(e, this)
    } else {
        let position = e.call(REFERENCE_POSITION, &args![this]).u32();
        read_point(e, position)
    };
    e.call(
        SOUND_HANDLE_SET_POSITION,
        &args![
            base.wrapping_add(0xc8),
            x87_quiet_bits(x),
            x87_quiet_bits(y),
            x87_quiet_bits(z)
        ],
    );
    let follow = e.call(POINTER_GET, &args![base.wrapping_add(0xc0)]).u32();
    e.call(SOUND_HANDLE_FOLLOW, &args![base.wrapping_add(0xc8), follow]);
    e.call(SOUND_HANDLE_PLAY, &args![base.wrapping_add(0xc8), 1u32]);
}

// Translated from 00811980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00811980 - the FS:[0] exception frame and the /GS cookie check (00ec408c) are not translated
/// `MagicBoltProjectile::MagicBoltProjectile` (Xbox PDB; `thiscall`, `RET 0x28`): the
/// `MagicProjectile` constructor with its ten words (passed on unchanged), the two vtables,
/// five null `NiPointer`s (+0xA8, +0xB4, +0xB8, +0xBC, +0xC0), the sound handle (+0xC8),
/// [`fn_00811e00`] with the integer setting at `011d16ec`, [`fn_008133d0`] with 0, +0x88 =
/// the projectile form's speed times the setting at `011d16f8`, +0xB0 = +0xC4 = 0, +0xD4 =
/// `0.0`. When the caster (the first word's slot +0x2C) is the player, the player's
/// `00950bb0(1)` exists and its `00456610` is false, the reference moves up by the setting at
/// `011d1ae4`. Then: a cloned 3D (`Clone3D`, held and released), [`magic_bolt_projectile_init_3d`],
/// `00570f70` with the node at +0xB4, [`fn_00813f30`], the controller's proxy velocity
/// `(-3.4028235e38, 0, 0, 0)`, the cell's `AddReference`, [`fn_008133d0`] with 0, the flight
/// sound (with a node at +0xC0), `CheckShotBehindWall`, and the running total
/// (`0080fae0` with the setting at `011d1758`) for other casters or, for the player's, the
/// projectile added to the process lists' list (`005ae3d0`). Returns `this`.
#[allow(clippy::too_many_arguments)]
pub fn magic_bolt_projectile_constructor(
    e: &mut Engine,
    this: Ptr,
    caster: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    first_x: u32,
    first_y: u32,
    first_z: u32,
    second_x: u32,
    second_y: u32,
    second_z: u32,
) -> Ptr {
    let base = this.addr();
    e.call(
        MAGIC_PROJECTILE_CONSTRUCTOR,
        &args![
            this, caster, word_2, word_3, word_4, first_x, first_y, first_z, second_x, second_y,
            second_z
        ],
    );
    e.mem.set_u32(base, MAGIC_BOLT_PROJECTILE_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x18), MAGIC_BOLT_PROJECTILE_SECOND_VTABLE);
    for offset in [0xa8u32, 0xb4, 0xb8, 0xbc, 0xc0] {
        e.call(POINTER_CONSTRUCT, &args![base.wrapping_add(offset), 0u32]);
    }
    e.call(SOUND_HANDLE_CONSTRUCT, &args![base.wrapping_add(0xc8)]);
    let value = setting_int(e, GLOBAL_SETTING);
    fn_00811e00(e, value);
    fn_008133d0(e, this, 0);
    let form = e.call(BASE_FORM, &args![this]).u32();
    let speed = e.call(PROJECTILE_SPEED, &args![form]).f64();
    let scale = e.call(SETTING_VALUE, &args![SPEED_SETTING]).u32();
    let value = x87_store32(x87_mul(x87_load32(e.mem.u32(scale)), speed));
    e.mem.set_u32(base.wrapping_add(0x88), value);
    e.mem.set_u32(base.wrapping_add(0xb0), 0);
    e.mem.set_u32(base.wrapping_add(0xc4), 0);
    e.mem.set_u32(base.wrapping_add(0xd4), 0.0f32.to_bits());
    let player = e.mem.u32(PLAYER);
    if e.vcall(caster, 0x2c, &[]).u32() == player
        && e.call(PLAYER_NODE, &args![player, 1u32]).u32() != 0
    {
        let player = e.mem.u32(PLAYER);
        let node = e.call(PLAYER_NODE, &args![player, 1u32]).u32();
        if !e.call(NODE_FLAG, &args![node]).bool() {
            let position = e.call(REFERENCE_POSITION, &args![this]).u32();
            let [x, y, z] = read_point(e, position);
            let height = e.call(SETTING_VALUE, &args![PLAYER_HEIGHT_SETTING]).u32();
            let z = x87_store32(x87_add(x87_load32(z), x87_m32(e.mem.u32(height))));
            e.with_stack(0x0c, |e, point| {
                write_point(e, point.addr(), [x, y, z]);
                e.call(SET_LOCATION_ON_REFERENCE, &args![this, point]);
            });
        }
    }
    e.with_stack(4, |e, clone| {
        let form = e.call(BASE_FORM, &args![this]).u32();
        let cloned = e.call(MAGIC_PROJECTILE_CLONE_3D, &args![form]).u32();
        e.call(POINTER_CONSTRUCT, &args![clone, cloned]);
        e.call(POINTER_ASSIGN, &args![clone, 0u32]);
        magic_bolt_projectile_init_3d(e, this);
        let node = e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32();
        e.call(SET_3D, &args![this, node]);
        fn_00813f30(e, this);
        e.with_stack(0x10, |e, velocity| {
            let lowest = x87_store32(-x87_load32(e.mem.u32(FLOAT_MAX)));
            e.call(
                VECTOR4_SET,
                &args![velocity, lowest, 0.0f32, 0.0f32, 0.0f32],
            );
            let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
            e.call(SET_PROXY_VELOCITY, &args![controller, velocity]);
        });
        let cell = e.call(PARENT_CELL, &args![this]).u32();
        e.call(CELL_ADD_REFERENCE, &args![cell, this, 0u32]);
        fn_008133d0(e, this, 0);
        let form = e.call(BASE_FORM, &args![this]).u32();
        if e.call(PROJECTILE_FLIGHT_SOUND, &args![form]).u32() != 0 {
            let form = e.call(BASE_FORM, &args![this]).u32();
            let sound = e.call(PROJECTILE_FLIGHT_SOUND, &args![form]).u32();
            let id = e.call(FORM_ID, &args![sound]).u32();
            let handle = base.wrapping_add(0xc8);
            if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
                e.call(SOUND_HANDLE_RELEASE, &args![handle]);
            }
            if e.call(POINTER_GET, &args![base.wrapping_add(0xc0)]).u32() != 0 {
                start_flight_sound(e, this, id, false);
            }
        }
        e.call(CHECK_SHOT_BEHIND_WALL, &args![this]);
        if ball::caster_reference(e, this) != e.mem.u32(PLAYER) {
            let value = ball::setting_bits(e, COUNT_SETTING);
            e.call(RUNNING_TOTAL_ADD, &args![value]);
        } else {
            let list = e.call(PROCESS_LIST, &args![PROCESS_LISTS]).u32();
            if list != 0 {
                e.with_stack(4, |e, item| {
                    e.mem.set_u32(item.addr(), base);
                    e.call(LIST_ADD, &args![list, item]);
                });
            }
        }
        e.call(POINTER_DESTRUCT, &args![clone]);
    });
    this
}

// Translated from 00811e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): stores it at `011ae61c`.
pub fn fn_00811e00(e: &mut Engine, value: u32) {
    e.mem.set_u32(BOLT_GLOBAL, value);
}

/// Releases a bolt segment's shader and node: `BoltShader::Stop(node, shader)` (the node at
/// `segment + node_offset`, the shader at `segment + shader_offset`), the shader pointer
/// assigned null, and the node detached from its parent (the parent's slot +0xE8).
fn release_segment(e: &mut Engine, shader_slot: u32, node_slot: u32) {
    let shader = e.call(POINTER_GET, &args![shader_slot]).u32();
    let node = e.call(POINTER_GET, &args![node_slot]).u32();
    e.call(BOLT_SHADER_STOP, &args![node, shader]);
    e.call(POINTER_ASSIGN, &args![shader_slot, 0u32]);
    if e.call(POINTER_GET, &args![node_slot]).u32() != 0 {
        let node = e.call(POINTER_GET, &args![node_slot]).u32();
        let parent = e.call(PARENT_NODE, &args![node]).u32();
        if parent != 0 {
            let node = e.call(POINTER_GET, &args![node_slot]).u32();
            e.vcall(parent, 0xe8, &args![node]);
        }
    }
}

// Translated from 00811e10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00811e10 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `MagicBoltProjectile::~MagicBoltProjectile` (Xbox PDB): the two vtables; the running
/// total (`0080fc90` with the setting at `011d1758`) unless the caster is the player's
/// `MagicCaster` part (`player + 0x88`); the projectile taken off the process lists' list
/// (`00905330`); the main shader and node released (`BoltShader::Stop`, the parent's slot
/// +0xE8), the pointers at +0xC0, +0xB4, +0xB8, +0xBC assigned null; each chained segment
/// (+0xB0, next at +0x1C) released the same way and deleted ([`fn_00812170`]); a valid sound
/// stopped and released; then the handle, the five `NiPointer`s and `~MagicProjectile`.
pub fn magic_bolt_projectile_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, MAGIC_BOLT_PROJECTILE_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x18), MAGIC_BOLT_PROJECTILE_SECOND_VTABLE);
    let player = e.mem.u32(PLAYER);
    let player_caster = if player != 0 {
        player.wrapping_add(0x88)
    } else {
        0
    };
    if e.call(PROJECTILE_CASTER, &args![this]).u32() != player_caster {
        let value = ball::setting_bits(e, COUNT_SETTING);
        e.call(RUNNING_TOTAL_TAKE, &args![value]);
    }
    let list = e.call(PROCESS_LIST, &args![PROCESS_LISTS]).u32();
    if list != 0 {
        e.with_stack(4, |e, item| {
            e.mem.set_u32(item.addr(), base);
            e.call(LIST_REMOVE, &args![list, item]);
        });
    }
    release_segment(e, base.wrapping_add(0xa8), base.wrapping_add(0xb4));
    for offset in [0xc0u32, 0xb4, 0xb8, 0xbc] {
        e.call(POINTER_ASSIGN, &args![base.wrapping_add(offset), 0u32]);
    }
    while e.mem.u32(base.wrapping_add(0xb0)) != 0 {
        let segment = e.mem.u32(base.wrapping_add(0xb0));
        let next = e.mem.u32(segment.wrapping_add(0x1c));
        release_segment(e, segment, segment.wrapping_add(4));
        for offset in [4u32, 0x14, 0x18] {
            e.call(POINTER_ASSIGN, &args![segment.wrapping_add(offset), 0u32]);
        }
        let segment = e.mem.u32(base.wrapping_add(0xb0));
        if segment != 0 {
            fn_00812170(e, Ptr::new(segment), 1);
        }
        e.mem.set_u32(base.wrapping_add(0xb0), next);
    }
    let handle = base.wrapping_add(0xc8);
    if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
        e.call(SOUND_HANDLE_STOP, &args![handle]);
        e.call(SOUND_HANDLE_RELEASE, &args![handle]);
    }
    e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    for offset in [0xc0u32, 0xbc, 0xb8, 0xb4, 0xa8] {
        e.call(POINTER_DESTRUCT, &args![base.wrapping_add(offset)]);
    }
    e.call(MAGIC_PROJECTILE_DESTRUCTOR, &args![this]);
}

// Translated from 00812170 (decompiled, FalloutNV.exe 1.4.0.525)
/// A bolt segment's scalar deleting destructor (unnamed in the engine map; `thiscall`,
/// `RET 4`): [`fn_008121a0`], then `operator delete` when bit 0 of the flags is set.
/// Returns `this`.
pub fn fn_00812170(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008121a0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 008121a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008121a0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// A bolt segment's destructor (unnamed in the engine map): destroys its `NiPointer`s at
/// +0x18, +0x14, +4 and +0.
pub fn fn_008121a0(e: &mut Engine, this: Ptr) {
    for offset in [0x18u32, 0x14, 4, 0] {
        e.call(POINTER_DESTRUCT, &args![this.addr().wrapping_add(offset)]);
    }
}

/// A new `NiNode` (0xAC bytes through `00aa13e0`, `00a5ecb0(0)`), or null.
fn new_node(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE_OBJECT, &args![0xacu32]).u32();
    if block != 0 {
        e.call(NODE_CONSTRUCTOR, &args![block, 0u32]).u32()
    } else {
        0
    }
}

// Translated from 00812220 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00812220 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `MagicBoltProjectile::Init3D` (Xbox PDB): a new node at +0xC0 placed at the slot +0x1F4
/// point (`004f00e0`); a new node at +0xB4 moved there, with `BSXFlags` extra data (made and
/// added when missing) given `00519230(1, 1)`; the shader pointer +0xA8 cleared; the +0xB4
/// node rotated to the reference's angles (`00a59660(z, x, y)`); a bolt shader started on it
/// (`BoltShader::Start(node, 1)`) into +0xA8 and, when there is one, configured from the
/// settings (integer `011d186c`, `011d1628`, `011d1bec`, `011d1610`, `011d1b2c`, two colours,
/// `011d174c`, integer `011d1800`, `011d16c8`, `011d17c4`, 1) and finished (`00bc2360`).
pub fn magic_bolt_projectile_init_3d(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let follow = new_node(e);
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(0xc0), follow]);
    let position = e.vcall(base, 0x1f4, &[]).u32();
    let follow = e.call(POINTER_GET, &args![base.wrapping_add(0xc0)]).u32();
    e.call(NODE_PLACE, &args![follow, position]);
    let node = new_node(e);
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(0xb4), node]);
    let position = e.vcall(base, 0x1f4, &[]).u32();
    let node = e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32();
    e.call(NODE_SET_TRANSLATE, &args![node, position]);
    let key = e.call(EXTRA_DATA_KEY, &[]).u32();
    let node = e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32();
    let mut flags = e.call(GET_EXTRA_DATA, &args![node, key]).u32();
    if flags == 0 {
        let block = e.call(ALLOCATE_OBJECT, &args![0x10u32]).u32();
        flags = if block != 0 {
            e.call(BSX_FLAGS_CONSTRUCTOR, &args![block]).u32()
        } else {
            0
        };
        let key = e.call(EXTRA_DATA_KEY, &[]).u32();
        let node = e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32();
        e.call(ADD_EXTRA_DATA, &args![node, key, flags]);
    }
    e.call(BSX_FLAGS_SET, &args![flags, 1u32, 1u32]);
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(0xa8), 0u32]);
    let angles = e.call(ANGLES, &args![this]).u32();
    let [x, y, z] = read_point(e, angles);
    e.with_stack(0x24, |e, matrix| {
        e.call(SELF, &args![matrix]);
        e.call(
            MATRIX_FROM_ANGLES,
            &args![
                matrix,
                x87_quiet_bits(z),
                x87_quiet_bits(x),
                x87_quiet_bits(y)
            ],
        );
        let node = e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32();
        e.call(NODE_SET_ROTATE, &args![node, matrix]);
    });
    let node = e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32();
    let shader = e.call(BOLT_SHADER_START, &args![node, 1u32]).u32();
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(0xa8), shader]);
    if e.call(POINTER_GET, &args![base.wrapping_add(0xa8)]).u32() == 0 {
        return;
    }
    let slot = base.wrapping_add(0xa8);
    configure_shader(e, &INIT_3D_SHADER, &|e: &mut Engine| {
        e.call(POINTER_GET, &args![slot]).u32()
    });
}

// Translated from 00812700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: the elapsed time, a bolt segment): moves
/// the segment's start (+8) to its source node's position (+0x14, when there is one) and
/// its node (+4) there; with the difference from the start to the target node's position
/// (+0x18), updates the segment's shader (+0) (`00bc2740`: time, the difference's length,
/// +0x88, and whether the setting at `011d1b14` is at most `00812870 - +0xD4`) and points
/// the node along the difference.
pub fn fn_00812700(e: &mut Engine, this: Ptr, time: f32, segment: u32) {
    let node = e.call(POINTER_GET, &args![segment.wrapping_add(4)]).u32();
    let start = segment.wrapping_add(8);
    if e.call(POINTER_GET, &args![segment.wrapping_add(0x14)])
        .u32()
        != 0
    {
        let source = e
            .call(POINTER_GET, &args![segment.wrapping_add(0x14)])
            .u32();
        let position = e.call(NODE_POSITION, &args![source]).u32();
        let words = read_point(e, position);
        write_point(e, start, words);
    }
    e.call(NODE_SET_TRANSLATE, &args![node, start]);
    e.with_stack(0x30, |e, locals| {
        let difference = locals.addr();
        let matrix = difference.wrapping_add(0x0c);
        let target = e
            .call(POINTER_GET, &args![segment.wrapping_add(0x18)])
            .u32();
        let target_position = e.call(NODE_POSITION, &args![target]).u32();
        e.call(POINT_DIFFERENCE, &args![target_position, difference, start]);
        let age = x87_sub(
            e.call(FLOAT_A4, &args![this]).f64(),
            x87_m32(e.mem.u32(this.addr().wrapping_add(0xd4))),
        );
        let limit = e.call(SETTING_VALUE, &args![0x011d_1b14u32]).u32();
        let limit = x87_load32(e.mem.u32(limit));
        // FCOMP / TEST AH,5 / JP: true when the setting is below or equal.
        let aged = limit <= age;
        let speed = x87_quiet_bits(e.mem.u32(this.addr().wrapping_add(0x88)));
        let length = x87_store32(e.call(POINT_LENGTH, &args![difference]).f64());
        let shader = e.call(POINTER_GET, &args![segment]).u32();
        e.call(
            SHADER_UPDATE_SEGMENT,
            &args![
                shader,
                x87_quiet_bits(time.to_bits()),
                length,
                speed,
                u32::from(aged)
            ],
        );
        aim_matrix(e, matrix, difference);
        e.call(NODE_SET_ROTATE, &args![node, matrix]);
    });
}

// Translated from 00812890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicBoltProjectile::PostProcess` (Xbox PDB): with the caster actor's magic node
/// (its `MagicCaster` part's slot +0x30), a controller and a node at +0xB4: when `006ca4e0`
/// is at most `0.5` and the caster is the player with `004eaf60` false, the node moves to
/// the magic node; when `006ca4e0` is above `0.0`, the node points at the controller's
/// position ([`bhk_character_controller_get_position`]); the node is updated (`0.0`).
/// Answers false then, true when any of the three is missing.
pub fn magic_bolt_projectile_post_process(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let caster = e.call(PROJECTILE_CASTER, &args![this]).u32();
    let reference = if caster != 0 {
        e.vcall(caster, 0x2c, &[]).u32()
    } else {
        0
    };
    let actor = if reference != 0 && e.vcall(reference, 0x100, &[]).bool() {
        reference
    } else {
        0
    };
    let magic_node = if actor != 0 {
        e.vcall(actor.wrapping_add(0x88), 0x30, &[]).u32()
    } else {
        0
    };
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    if magic_node == 0
        || controller == 0
        || e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32() == 0
    {
        return true;
    }
    let value = e.call(FLOAT_90, &args![this]).f64();
    // FCOMP / TEST AH,5 / JP: on when at most (ordered).
    if value <= e.mem.f64(HALF)
        && actor == e.mem.u32(PLAYER)
        && !{
            let player = e.mem.u32(PLAYER);
            e.call(PLAYER_FLAG, &args![player]).bool()
        }
    {
        let position = e.call(NODE_POSITION, &args![magic_node]).u32();
        let words = read_point(e, position);
        e.with_stack(0x0c, |e, point| {
            write_point(e, point.addr(), words);
            let node = e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32();
            e.call(NODE_SET_TRANSLATE, &args![node, point]);
        });
    }
    let value = e.call(FLOAT_90, &args![this]).f64();
    if value > e.mem.f64(ZERO) {
        e.with_stack(0x58, |e, locals| {
            let start = locals.addr();
            let controller_position = start.wrapping_add(0x0c);
            let difference = start.wrapping_add(0x1c);
            let matrix = start.wrapping_add(0x28);
            let node = e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32();
            let position = e.call(NODE_POSITION, &args![node]).u32();
            let words = read_point(e, position);
            write_point(e, start, words);
            e.call(SELF, &args![controller_position]);
            bhk_character_controller_get_position(
                e,
                Ptr::new(controller),
                Ptr::new(controller_position),
            );
            e.call(
                POINT_DIFFERENCE,
                &args![controller_position, difference, start],
            );
            aim_matrix(e, matrix, difference);
            let node = e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32();
            e.call(NODE_SET_ROTATE, &args![node, matrix]);
        });
    }
    e.with_stack(0x0c, |e, update| {
        e.call(UPDATE_DATA_INIT, &args![update, 0.0f32, 0u32, 0u32]);
        let node = e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32();
        e.call(NODE_UPDATE, &args![node, update]);
    });
    false
}

/// The default point's three words (`011f426c`), then the controller's position over them
/// when there is a controller ([`bhk_character_controller_get_position`]).
fn controller_position(e: &mut Engine, this: Ptr, out: u32) {
    let words = read_point(e, DEFAULT_POINT);
    write_point(e, out, words);
    if e.call(GET_CHAR_CONTROLLER, &args![this]).u32() != 0 {
        let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).ptr();
        bhk_character_controller_get_position(e, controller, Ptr::new(out));
    }
}

/// `00bc28b0` on the main shader, or (while moving) only when `+0x140 * the shader's
/// float` of the shader is at most `006ca4e0 * +0x88`.
fn update_main_shader(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    moving: bool,
    length: &dyn Fn(&mut Engine) -> u32,
) {
    let base = this.addr();
    let update = if moving {
        let reach = x87_mul(
            e.call(FLOAT_90, &args![this]).f64(),
            x87_m32(e.mem.u32(base.wrapping_add(0x88))),
        );
        let shader = e.call(POINTER_GET, &args![base.wrapping_add(0xa8)]).u32();
        let count = e.call(WORD_140, &args![shader]).i32();
        let shader = e.call(POINTER_GET, &args![base.wrapping_add(0xa8)]).u32();
        let per = fn_008133b0(e, Ptr::new(shader));
        // FCOMP / TEST AH,5 / JP: on when at most (ordered).
        x87_mul(per, f64::from(count)) <= reach
    } else {
        true
    };
    if update {
        let speed = x87_quiet_bits(e.mem.u32(base.wrapping_add(0x88)));
        let length = length(e);
        let shader = e.call(POINTER_GET, &args![base.wrapping_add(0xa8)]).u32();
        e.call(
            SHADER_UPDATE,
            &args![
                shader,
                x87_quiet_bits(time.to_bits()),
                length,
                speed,
                u32::from(moving)
            ],
        );
    }
}

// Translated from 00812b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: the elapsed time): the bolt's update.
/// Nothing at a time equal to `0.0` or when `00440d80` is true. With a shader and state 0
/// (flying): like the ball, slot +0x304 when low, slot +0x250 by `(0, +0x88 * time, 0)`, the
/// distance between the controller's positions before and after added to +0x8C (slot +0xC4
/// with 1 beyond the setting at `011d1d00`), and nothing more unless slot +0x30C answers
/// true. In state 1, the chained segments (+0xB0) are updated ([`fn_00812700`]) and those
/// whose shader is done ([`fn_00813370`]) removed and deleted. Then, unless the integer
/// setting at `011d1afc` is above 0 or the byte at `011dc55c` is set: toward the target node
/// (+0xBC) the main shader is updated and the node (+0xB4) aimed from the world translation
/// of the reference's node; without a target, from the controller's position (and when the
/// shader is done, [`fn_008133d0`] with 2 and the controller's filter flagged). Then the
/// follow node (+0xC0) moves to the position; when the shader is done, or in state 1 after
/// `00812870 - +0xD4` passed 3, a valid sound stops and, without segments, slot +0xC4 runs
/// with 1.
pub fn fn_00812b60(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    let time_value = x87_load32(time.to_bits());
    // FCOMP / TEST AH,0x44 / JP: stops only when equal.
    if time_value == e.mem.f64(ZERO) {
        return;
    }
    if e.call(REFERENCE_FLAG, &args![this]).bool() {
        return;
    }
    let node = ball::node(e, this);
    let translate = e.call(NODE_WORLD_TRANSLATE, &args![node]).u32();
    let head = read_point(e, translate);
    let state = e.mem.u32(base.wrapping_add(0xac));
    if e.call(POINTER_GET, &args![base.wrapping_add(0xa8)]).u32() != 0 && state == 0 {
        if !bolt_fly(e, this, time) {
            return;
        }
    } else if e.mem.u32(base.wrapping_add(0xac)) == 1 {
        update_segments(e, this, time);
    }
    let limit = setting_int(e, SEGMENT_LIMIT_SETTING) as i32;
    if !(limit > 0 || e.mem.u8(AIM_DISABLED) != 0) {
        aim_bolt(e, this, time, head);
    }
    if e.call(POINTER_GET, &args![base.wrapping_add(0xc0)]).u32() != 0 {
        let position = e.vcall(base, 0x1f4, &[]).u32();
        let follow = e.call(POINTER_GET, &args![base.wrapping_add(0xc0)]).u32();
        e.call(NODE_PLACE, &args![follow, position]);
    }
    let shader = e.call(POINTER_GET, &args![base.wrapping_add(0xa8)]).u32();
    let finished = if fn_00813370(e, Ptr::new(shader)) {
        true
    } else {
        if e.mem.u32(base.wrapping_add(0xac)) != 1 {
            return;
        }
        let age = x87_sub(
            e.call(FLOAT_A4, &args![this]).f64(),
            x87_m32(e.mem.u32(base.wrapping_add(0xd4))),
        );
        // FCOMP / TEST AH,0x41 / JNZ: on when above.
        age > e.mem.f64(THREE)
    };
    if !finished {
        return;
    }
    let handle = base.wrapping_add(0xc8);
    if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
        e.call(SOUND_HANDLE_STOP, &args![handle]);
        e.call(SOUND_HANDLE_RELEASE, &args![handle]);
    }
    if e.mem.u32(base.wrapping_add(0xb0)) == 0 {
        e.vcall(base, 0xc4, &args![1u32]);
    }
}

/// [`fn_00812b60`]'s flying state; answers whether the update goes on (slot +0x30C).
fn bolt_fly(e: &mut Engine, this: Ptr, time: f32) -> bool {
    let base = this.addr();
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    let height = x87_store32(e.call(CONTROLLER_FLOAT, &args![controller]).f64());
    let limit = e.call(HEIGHT_OFFSET, &args![height]).f64();
    let position = e.vcall(base, 0x1f4, &[]).u32();
    let z = x87_load32(e.mem.u32(position.wrapping_add(8)));
    // TEST AH,0x41 / JP: on when below or equal (ordered).
    if z <= limit {
        let [x, y, z] = ball::position_words(e, this);
        e.vcall(base, 0x304, &args![x, y, z, 0u32, 0u32, 1u32]);
    }
    e.with_stack(0x30, |e, locals| {
        let step = locals.addr();
        let before = step.wrapping_add(0x0c);
        let after = step.wrapping_add(0x18);
        let difference = step.wrapping_add(0x24);
        let speed = x87_store32(x87_mul(
            x87_load32(e.mem.u32(base.wrapping_add(0x88))),
            x87_m32(time.to_bits()),
        ));
        e.call(POINT_SET, &args![step, 0.0f32, speed, 0.0f32]);
        controller_position(e, this, before);
        e.vcall(
            base,
            0x250,
            &args![x87_quiet_bits(time.to_bits()), step, 0xfu32],
        );
        controller_position(e, this, after);
        let out = e
            .call(POINT_DIFFERENCE, &args![after, difference, before])
            .u32();
        let length = e.call(POINT_LENGTH, &args![out]).f64();
        let travelled = base.wrapping_add(0x8c);
        let sum = x87_add(length, x87_m32(e.mem.u32(travelled)));
        e.mem.set_u32(travelled, x87_store32(sum));
    });
    let limit = e.call(SETTING_VALUE, &args![DISTANCE_SETTING]).u32();
    let distance = x87_load32(e.mem.u32(base.wrapping_add(0x8c)));
    let limit = x87_load32(e.mem.u32(limit));
    // FCOMPP (the limit against the distance) / TEST AH,5 / JP: on when the limit is below.
    if limit < distance {
        e.vcall(base, 0xc4, &args![1u32]);
    }
    e.vcall(base, 0x30c, &[]).bool()
}

/// [`fn_00812b60`]'s segment walk (state 1).
fn update_segments(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    let mut previous = 0u32;
    let mut segment = e.mem.u32(base.wrapping_add(0xb0));
    while segment != 0 {
        fn_00812700(e, this, time, segment);
        let next = e.mem.u32(segment.wrapping_add(0x1c));
        let shader = e.call(POINTER_GET, &args![segment]).u32();
        if fn_00813370(e, Ptr::new(shader)) {
            let node = e.call(POINTER_GET, &args![segment.wrapping_add(4)]).u32();
            let parent = e.call(PARENT_NODE, &args![node]).u32();
            if parent != 0 {
                let node = e.call(POINTER_GET, &args![segment.wrapping_add(4)]).u32();
                e.vcall(parent, 0xe8, &args![node]);
            }
            let following = e.mem.u32(segment.wrapping_add(0x1c));
            if previous == 0 {
                e.mem.set_u32(base.wrapping_add(0xb0), following);
            } else {
                e.mem.set_u32(previous.wrapping_add(0x1c), following);
            }
            for offset in [4u32, 0, 0x14, 0x18] {
                e.call(POINTER_ASSIGN, &args![segment.wrapping_add(offset), 0u32]);
            }
            fn_00812170(e, Ptr::new(segment), 1);
        } else {
            previous = segment;
        }
        segment = next;
    }
}

/// [`fn_00812b60`]'s aiming of the main shader and node.
fn aim_bolt(e: &mut Engine, this: Ptr, time: f32, head: [u32; 3]) {
    let base = this.addr();
    let moving = e.mem.u32(base.wrapping_add(0xac)) != 0;
    e.with_stack(0x60, |e, locals| {
        let start = locals.addr();
        let difference = start.wrapping_add(0x0c);
        let matrix = start.wrapping_add(0x18);
        let other = start.wrapping_add(0x3c);
        write_point(e, start, head);
        if e.call(POINTER_GET, &args![base.wrapping_add(0xbc)]).u32() != 0 {
            let target = e.call(POINTER_GET, &args![base.wrapping_add(0xbc)]).u32();
            let target_position = e.call(NODE_POSITION, &args![target]).u32();
            e.call(POINT_DIFFERENCE, &args![target_position, difference, start]);
            update_main_shader(e, this, time, moving, &|e: &mut Engine| {
                x87_store32(e.call(POINT_LENGTH, &args![difference]).f64())
            });
            aim_matrix(e, matrix, difference);
            let node = e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32();
            e.call(NODE_SET_ROTATE, &args![node, matrix]);
        } else if e.call(GET_CHAR_CONTROLLER, &args![this]).u32() != 0 {
            e.call(SELF, &args![other]);
            let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).ptr();
            bhk_character_controller_get_position(e, controller, Ptr::new(other));
            let out = e
                .call(POINT_DIFFERENCE, &args![start, difference, other])
                .u32();
            let length = x87_store32(e.call(POINT_LENGTH, &args![out]).f64());
            update_main_shader(e, this, time, moving, &move |_: &mut Engine| length);
            let shader = e.call(POINTER_GET, &args![base.wrapping_add(0xa8)]).u32();
            if e.call(SHADER_DONE, &args![shader]).bool() {
                fn_008133d0(e, this, 2);
                flag_controller_filter(e, this);
            }
        }
    });
}

// Translated from 00813370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, on a bolt shader): true when the byte at +0x174 is
/// zero and the word at +0x78 is at least the word at +0x7C (signed).
pub fn fn_00813370(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    e.mem.u8(base.wrapping_add(0x174)) == 0
        && e.mem.i32(base.wrapping_add(0x78)) >= e.mem.i32(base.wrapping_add(0x7c))
}

// Translated from 008133d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores the word at +0xAC (the state).
pub fn fn_008133d0(e: &mut Engine, this: Ptr, state: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0xac), state);
}

// Translated from 008133f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008133f0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 0x18`: a point by value, a word passed on to
/// `0081f000`, the reference hit, a flag byte): the bolt hits. Nothing when the reference is
/// an actor whose `008ace90` is true. +0xD4 = `00812870`; the controller (slot +0x310 when
/// its object has flag `0x8000`, filter flagged); state 1 ([`fn_008133d0`]); with a node,
/// `0081f000` (the point, the offset at `011a9478` through the node's transform, the word);
/// a reference other than the player (or the player with `004eaf60` true) becomes the target
/// (+0xC4) and its `"Bip01 Spine2"` node the target node (+0xBC); the main shader
/// `00bc28b0(0.01, the distance from the node's world translation to the point, +0x88, 0)`;
/// without the flag the caster's `FindTargets_ov2`; +0xC4 cleared; when the projectile
/// form's model has a path, the area scale is computed (and left unused) and a temporary
/// effect is made (`006890b0`, `"SpecialIdle_AreaEffect"`); without the flag, the impact
/// sound.
#[allow(clippy::too_many_arguments)]
pub fn fn_008133f0(
    e: &mut Engine,
    this: Ptr,
    x: u32,
    y: u32,
    z: u32,
    passed: u32,
    reference: u32,
    flag: u8,
) {
    let base = this.addr();
    let actor = if reference != 0 && e.vcall(reference, 0x100, &[]).bool() {
        reference
    } else {
        0
    };
    if actor != 0 && e.call(ACTOR_STOPS_EXPLOSION, &args![actor]).bool() {
        return;
    }
    let time = x87_store32(e.call(FLOAT_A4, &args![this]).f64());
    e.mem.set_u32(base.wrapping_add(0xd4), time);
    if e.call(GET_CHAR_CONTROLLER, &args![this]).u32() != 0 {
        let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
        if e.call(
            CONTROLLER_HAS_FLAG,
            &args![controller.wrapping_add(0x410), 0x8000u32],
        )
        .u32()
            != 0
        {
            e.vcall(base, 0x310, &[]);
        }
        flag_controller_filter(e, this);
    }
    fn_008133d0(e, this, 1);
    let flag = flag != 0;
    // The point argument lives in the caller's stack; the difference below takes its
    // address.
    e.with_stack(0x0c, |e, point| {
        let point = point.addr();
        write_point(e, point, [x, y, z]);
        let node = ball::node(e, this);
        if node != 0 {
            let transform = e.call(NODE_TRANSFORM, &args![node]).u32();
            let offset = e.with_stack(0x0c, |e, out| {
                let moved = e
                    .call(TRANSFORM_APPLY, &args![transform, out, EXPLOSION_OFFSET])
                    .u32();
                read_point(e, moved)
            });
            let at = read_point(e, point);
            e.call(
                PROJECTILE_EXPLODE_AT,
                &args![this, at[0], at[1], at[2], offset[0], offset[1], offset[2], passed],
            );
        }
        if reference != 0 {
            let player = e.mem.u32(PLAYER);
            if reference != player || e.call(PLAYER_FLAG, &args![player]).bool() {
                e.mem.set_u32(base.wrapping_add(0xc4), reference);
                let target = e.mem.u32(base.wrapping_add(0xc4));
                let target_node = e.vcall(target, 0x1d0, &[]).u32();
                if target_node != 0 {
                    let spine = spine_node(e, target_node);
                    e.call(POINTER_ASSIGN, &args![base.wrapping_add(0xbc), spine]);
                }
            }
        }
        let node = ball::node(e, this);
        let translate = e.call(NODE_WORLD_TRANSLATE, &args![node]).u32();
        let head = read_point(e, translate);
        let length = e.with_stack(0x18, |e, locals| {
            let start = locals.addr();
            let difference = start.wrapping_add(0x0c);
            write_point(e, start, head);
            let out = e
                .call(POINT_DIFFERENCE, &args![start, difference, point])
                .u32();
            x87_store32(e.call(POINT_LENGTH, &args![out]).f64())
        });
        let speed = x87_quiet_bits(e.mem.u32(base.wrapping_add(0x88)));
        let hundredth = x87_quiet_bits(e.mem.u32(HUNDREDTH));
        let shader = e.call(POINTER_GET, &args![base.wrapping_add(0xa8)]).u32();
        e.call(
            SHADER_UPDATE,
            &args![shader, hundredth, x87_quiet_bits(length), speed, 0u32],
        );
        if !flag {
            let [px, py, pz] = ball::position_words(e, this);
            let cell = e.call(PARENT_CELL, &args![this]).u32();
            let kind = e.call(PROJECTILE_KIND, &args![this]).u32();
            let caster = e.call(PROJECTILE_CASTER, &args![this]).u32();
            e.call(
                FIND_TARGETS_OV2,
                &args![caster, kind, cell, px, py, pz, this, reference, 0u32, 1.0f32, 1.0f32],
            );
        }
        e.mem.set_u32(base.wrapping_add(0xc4), 0);
        let model = e.call(BASE_FORM, &args![this]).u32().wrapping_add(0x18);
        if e.vcall(model, 0x14, &[]).u32() == 0 {
            return;
        }
        let item = e.call(PROJECTILE_EFFECT_ITEM, &args![this]).u32();
        let area = e.call(EFFECT_ITEM_GET_AREA, &args![item]).i32();
        let scale_value = e.call(SETTING_VALUE, &args![AREA_SCALE]).u32();
        let mut scale = x87_store32(x87_mul(x87_load32(e.mem.u32(scale_value)), f64::from(area)));
        let max = e.call(SETTING_VALUE, &args![AREA_SCALE_MAX]).u32();
        // FCOMPP / TEST AH,5 / JP: lowered when the maximum is below.
        if x87_load32(e.mem.u32(max)) < x87_load32(scale) {
            scale = ball::setting_bits(e, AREA_SCALE_MAX);
        } else {
            let min = e.call(SETTING_VALUE, &args![AREA_SCALE_MIN]).u32();
            // FCOMPP / TEST AH,0x41 / JNZ: raised when the minimum is above.
            if x87_load32(e.mem.u32(min)) > x87_load32(scale) {
                scale = ball::setting_bits(e, AREA_SCALE_MIN);
            }
        }
        // The exe keeps the scale in a local nothing reads afterwards.
        let _unused_scale = scale;
        let model = e.call(BASE_FORM, &args![this]).u32().wrapping_add(0x18);
        let path = e.vcall(model, 0x14, &[]).u32();
        let offset = if ball::node(e, this) != 0 {
            let node = ball::node(e, this);
            let transform = e.call(NODE_TRANSFORM_34, &args![node]).u32();
            e.with_stack(0x0c, |e, out| {
                let moved = e
                    .call(TRANSFORM_APPLY, &args![transform, out, EXPLOSION_OFFSET])
                    .u32();
                read_point(e, moved)
            })
        } else {
            read_point(e, EXPLOSION_OFFSET)
        };
        let five = x87_quiet_bits(e.mem.u32(FIVE));
        let cell = e.call(PARENT_CELL, &args![this]).u32();
        let effect = e
            .call(
                MAKE_TEMP_EFFECT,
                &args![
                    cell, five, path, offset[0], offset[1], offset[2], x, y, z, 1.0f32, 7u32, 0u32
                ],
            )
            .u32();
        e.call(
            TEMP_EFFECT_SET_IDLE,
            &args![effect, TEXT_SPECIAL_IDLE_AREA_EFFECT],
        );
    });
    let form = e.call(BASE_FORM, &args![this]).u32();
    if e.call(PROJECTILE_IMPACT_SOUND, &args![form]).u32() != 0 && !flag {
        play_impact_sound(e, this);
    }
}

// Translated from 008139f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008139f0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 8`: a magic target, an effect item): chains
/// the bolt to the target. Nothing when the item is given and either is not this
/// projectile's effect item (by id) or the target's reference (its slot +4) is already the
/// target (+0xC4). A new node is attached under the cell's root node (its slot +0xDC with
/// 1) with a bolt shader (`BoltShader::Start(node, 0)`, configured from the segment
/// settings when there is one); the segment (0x20 bytes, [`fn_00813ea0`]) holds the shader,
/// the node, the controller's position, the current target's `"Bip01 Spine2"` node and the
/// new target's (or its node), is linked in front of +0xB0 after [`fn_00812700`] with `0.0`.
/// Nothing is made when the new target's reference has no node.
pub fn fn_008139f0(e: &mut Engine, this: Ptr, target: u32, item: u32) {
    let base = this.addr();
    if item != 0 {
        let id = e.call(FORM_ID, &args![item]).u32();
        if id != e.call(PROJECTILE_EFFECT_ITEM, &args![this]).u32() {
            return;
        }
        if e.vcall(target, 4, &[]).u32() == e.mem.u32(base.wrapping_add(0xc4)) {
            return;
        }
    }
    let node = new_node(e);
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    let root = e.call(CELL_ROOT_NODE, &args![cell]).u32();
    e.vcall(root, 0xdc, &args![node, 1u32]);
    let shader = e.call(BOLT_SHADER_START, &args![node, 0u32]).u32();
    if shader != 0 {
        configure_shader(e, &SEGMENT_SHADER, &move |_: &mut Engine| shader);
    }
    let position = e.with_stack(0x0c, |e, out| {
        e.call(SELF, &args![out]);
        let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).ptr();
        bhk_character_controller_get_position(e, controller, out);
        read_point(e, out.addr())
    });
    let current = e.mem.u32(base.wrapping_add(0xc4));
    let current_node = if current != 0 {
        e.vcall(current, 0x1d0, &[]).u32()
    } else {
        0
    };
    let source = if current_node != 0 {
        spine_node(e, current_node)
    } else {
        0
    };
    let reference = e.vcall(target, 4, &[]).u32();
    let target_node = e.vcall(reference, 0x1d0, &[]).u32();
    if target_node == 0 {
        return;
    }
    let mut destination = spine_node(e, target_node);
    if destination == 0 {
        destination = target_node;
    }
    let block = e.call(ALLOCATE, &args![0x20u32]).u32();
    let segment = if block != 0 {
        fn_00813ea0(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.call(POINTER_ASSIGN, &args![segment, shader]);
    e.call(POINTER_ASSIGN, &args![segment.wrapping_add(4), node]);
    e.call(POINTER_ASSIGN, &args![segment.wrapping_add(0x14), source]);
    e.call(
        POINTER_ASSIGN,
        &args![segment.wrapping_add(0x18), destination],
    );
    write_point(e, segment.wrapping_add(8), position);
    let first = e.mem.u32(base.wrapping_add(0xb0));
    e.mem.set_u32(segment.wrapping_add(0x1c), first);
    fn_00812700(e, this, 0.0, segment);
    e.mem.set_u32(base.wrapping_add(0xb0), segment);
}

// Translated from 00813ea0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00813ea0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// A bolt segment's constructor (unnamed in the engine map): null `NiPointer`s at +0, +4,
/// +0x14 and +0x18, and `006815c0` on the point at +8. Returns `this`.
pub fn fn_00813ea0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(POINTER_CONSTRUCT, &args![base, 0u32]);
    e.call(POINTER_CONSTRUCT, &args![base.wrapping_add(4), 0u32]);
    e.call(SELF, &args![base.wrapping_add(8)]);
    e.call(POINTER_CONSTRUCT, &args![base.wrapping_add(0x14), 0u32]);
    e.call(POINTER_CONSTRUCT, &args![base.wrapping_add(0x18), 0u32]);
    this
}

// Translated from 00813f30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00813f30 - the FS:[0] exception frame and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map: the bolt's character controller setup, the same steps as
/// the ball's (`0080feb0` in `magicballprojectile.rs`) except that the new controller gets
/// `008105e0(1)` before `00810610` with `00525420`'s flag.
pub fn fn_00813f30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let object = e.mem.u32(base.wrapping_add(0x68));
    if object == 0 || e.call(OBJECT_BUSY, &args![object]).u32() != 0 {
        return;
    }
    let mut controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    let world = if cell != 0 {
        e.call(CELL_HAVOK_WORLD, &args![cell]).u32()
    } else {
        0
    };
    if controller == 0 {
        controller = e.with_stack(0xc8, |e, info| build_controller(e, this, info, world));
    }
    if controller == 0 {
        return;
    }
    if cell != 0 && world != 0 {
        let height = x87_store32(e.call(CELL_WATER_HEIGHT, &args![cell]).f64());
        e.call(CONTROLLER_SET_FLOAT, &args![controller, height]);
    } else {
        let lowest = x87_quiet_bits(e.mem.u32(LOWEST_FLOAT));
        e.call(CONTROLLER_SET_FLOAT, &args![controller, lowest]);
    }
    let old_world = e.call(CONTROLLER_WORLD, &args![controller]).u32();
    e.call(CONTROLLER_SET_WORLD, &args![controller, world]);
    let node = ball::node(e, this);
    let current = e.call(CONTROLLER_NODE, &args![controller]).u32();
    let mut changed = true;
    if current != node {
        e.call(CONTROLLER_SET_NODE, &args![controller, node]);
        if world != 0 && e.call(WORLD_FLAG, &args![world]).bool() {
            e.vcall(controller, 0xd0, &args![0u32]);
            let object = e.call(CONTROLLER_NODE, &args![controller]).u32();
            e.call(SET_WIREFRAME, &args![object, 1u32]);
        }
        e.call(WORLD_SET_MOTION, &args![node, 4u32, 1u32, 0u32, 1u32]);
        changed = true;
    }
    let [x, y, mut z] = ball::position_words(e, this);
    if old_world != world {
        if old_world != 0 {
            e.call(WORLD_REMOVE_OBJECTS, &args![node, 1u32, 0u32]);
        }
        z = x87_store32(x87_add(x87_load32(z), e.mem.f64(EIGHT)));
        changed = true;
    }
    if world == 0 {
        return;
    }
    e.with_stack(0x0c, |e, position| {
        write_point(e, position.addr(), [x, y, z]);
        e.call(CONTROLLER_SET_POSITION, &args![controller, position]);
    });
    if changed {
        let layer = e.with_stack(4, |e, filter| {
            e.call(CONTROLLER_FILTER, &args![controller, filter]);
            e.call(FILTER_LAYER, &args![filter]).u32()
        });
        e.vcall(world, 0xd0, &args![node, 1u32, 0u32, layer, 0u32]);
    } else {
        // `changed` starts true and is only ever set true, so this is never reached; it is
        // translated as the exe has it.
        e.call(WORLD_UPDATE_POSITION, &args![node, 1u32, 0u32]);
    }
}

/// [`fn_00813f30`]'s controller construction on the local construction info.
fn build_controller(e: &mut Engine, this: Ptr, info: Ptr, world: u32) -> u32 {
    let base = this.addr();
    e.call(CONTROLLER_INFO_CONSTRUCT, &args![info, 1u32]);
    e.call(INFO_SET_FLOAT_8C, &args![info, 1.0f32]);
    let node = ball::node(e, this);
    e.call(CONTROLLER_INFO_SET_NODE, &args![info, node]);
    let position = e.vcall(base, 0x1f4, &[]).u32();
    e.call(INFO_SET_POSITION, &args![info, position]);
    e.call(CONTROLLER_INFO_SET_WORLD, &args![info, world]);
    e.call(CONTROLLER_INFO_SET_FLOAT, &args![info, 0.0f32]);
    let block = e.call(ALLOCATE_OBJECT, &args![0x14u32]).u32();
    let shape = if block != 0 {
        e.call(SPHERE_SHAPE_CONSTRUCTOR, &args![block, 1.0f32, 1u32])
            .u32()
    } else {
        0
    };
    e.call(INFO_SET_SHAPE, &args![info, shape]);
    e.call(INFO_SET_BYTE, &args![info, 0u32]);
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_LOAD_FLAG, &args![manager]).bool() {
        e.call(INFO_SET_FILTER, &args![info, 7u32, 0u32]);
    } else {
        let reference = ball::caster_reference(e, this);
        let layer = if reference != 0 {
            if e.vcall(reference, 0x100, &[]).bool() {
                ball::actor_layer(e, reference)
            } else {
                let reference_node = e.vcall(reference, 0x1d0, &[]).u32();
                ball::node_layer(e, reference_node)
            }
        } else {
            let own_node = ball::node(e, this);
            ball::node_layer(e, own_node)
        };
        e.call(INFO_SET_FILTER, &args![info, 7u32, layer]);
    }
    e.call(INFO_SET_WORD, &args![info, 6u32]);
    let block = e.call(ALLOCATE_CONTROLLER, &args![0x660u32]).u32();
    let made = if block != 0 {
        e.call(CHARACTER_CONTROLLER_CONSTRUCTOR, &args![block, info, this])
            .u32()
    } else {
        0
    };
    e.call(CONTROLLER_SET_FLOAT_548, &args![made, 1.0f32]);
    e.call(CONTROLLER_FLAG_80000, &args![made, 1u32]);
    let flag = e.call(CONTROLLER_FLAG_SOURCE, &[]).u8();
    e.call(CONTROLLER_FLAG_100000, &args![made, flag]);
    // The argument is an `NiPointer` built in its stack slot; the callee owns it.
    let holder = e.with_stack(4, |e, slot| {
        e.call(POINTER_CONSTRUCT, &args![slot, made]);
        e.mem.u32(slot.addr())
    });
    let object = e.mem.u32(base.wrapping_add(0x68));
    e.vcall(object, 0x290, &args![holder]);
    e.call(CONTROLLER_SET_OWNER, &args![made, base]);
    e.call(CONTROLLER_INFO_DESTRUCT, &args![info]);
    made
}

// Translated from 00814590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; the save size): the `MagicProjectile`
/// size plus 4 + 4, plus 0xC + 0x10 + 4 for a save version of 0x30 or more, plus 4 + 2,
/// plus 4 for each chained segment, plus 4 for a version of 0x71 or more (16-bit
/// arithmetic).
pub fn fn_00814590(e: &mut Engine, this: Ptr, buffer: u32) -> u16 {
    let mut size = 0u16;
    let base_size = e
        .call(MAGIC_PROJECTILE_SAVE_SIZE, &args![this, buffer])
        .u16();
    size = size.wrapping_add(base_size);
    size = size.wrapping_add(4);
    size = size.wrapping_add(4);
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x30 {
        size = size.wrapping_add(0xc);
        size = size.wrapping_add(0x10);
        size = size.wrapping_add(4);
    }
    size = size.wrapping_add(4);
    size = size.wrapping_add(2);
    let mut segment = e.mem.u32(this.addr().wrapping_add(0xb0));
    while segment != 0 {
        size = size.wrapping_add(4);
        segment = e.mem.u32(segment.wrapping_add(0x1c));
    }
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x71 {
        size = size.wrapping_add(4);
    }
    size
}

// Translated from 00814670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; the save): `MagicProjectile::SaveGame`,
/// the state (+0xAC, through the manager), +0x88; for a version of 0x30 or more the node's
/// world translation (0xC bytes), its rotation (`0056d230`, 0x10 bytes through the manager)
/// and the shader's `0051b860` (`0.0` without a shader); the target's id (+0xC4, 0
/// without); the segment count (2 bytes, written back into the slot `00825c00` reserves)
/// and, per segment, the id of the reference of its target node (`FindReferenceFor3D`, 0
/// without); +0xD4 for a version of 0x71 or more.
pub fn fn_00814670(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    e.call(MAGIC_PROJECTILE_SAVE_GAME, &args![this, buffer]);
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    e.call(SAVE_WORD, &args![manager, base.wrapping_add(0xac), 4u32]);
    e.call(SAVE_DATA, &args![this, base.wrapping_add(0x88), 4u32]);
    e.with_stack(0x30, |e, locals| {
        let translate = locals.addr();
        let rotation = translate.wrapping_add(0x0c);
        let value = translate.wrapping_add(0x1c);
        let id = translate.wrapping_add(0x20);
        let count = translate.wrapping_add(0x24);
        let segment_id = translate.wrapping_add(0x28);
        let manager = e.mem.u32(SAVE_LOAD_MANAGER);
        if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x30 {
            let node = ball::node(e, this);
            let world = e.call(NODE_WORLD_TRANSLATE, &args![node]).u32();
            let words = read_point(e, world);
            write_point(e, translate, words);
            e.call(SAVE_DATA, &args![this, translate, 0xcu32]);
            e.call(SELF, &args![rotation]);
            let node = ball::node(e, this);
            e.call(NODE_ROTATION, &args![node, rotation]);
            let manager = e.mem.u32(SAVE_LOAD_MANAGER);
            e.call(SAVE_WORD, &args![manager, rotation, 0x10u32]);
            e.mem.set_u32(value, 0.0f32.to_bits());
            if e.call(POINTER_GET, &args![base.wrapping_add(0xa8)]).u32() != 0 {
                let shader = e.call(POINTER_GET, &args![base.wrapping_add(0xa8)]).u32();
                let saved = x87_store32(e.call(SHADER_FLOAT, &args![shader]).f64());
                e.mem.set_u32(value, saved);
            }
            e.call(SAVE_DATA, &args![this, value, 4u32]);
        }
        e.mem.set_u32(id, 0);
        let target = e.mem.u32(base.wrapping_add(0xc4));
        if target != 0 {
            let form_id = e.call(FORM_ID, &args![target]).u32();
            e.mem.set_u32(id, form_id);
        }
        e.call(SAVE_ID, &args![this, id, 4u32]);
        e.mem.set_u16(count, 0);
        let manager = e.mem.u32(SAVE_LOAD_MANAGER);
        let slot = e.call(RESERVE_COUNT, &args![manager]).u32();
        e.call(SAVE_DATA, &args![this, count, 2u32]);
        let mut segment = e.mem.u32(base.wrapping_add(0xb0));
        while segment != 0 {
            e.mem.set_u32(segment_id, 0);
            if e.call(POINTER_GET, &args![segment.wrapping_add(0x18)])
                .u32()
                != 0
            {
                let node = e
                    .call(POINTER_GET, &args![segment.wrapping_add(0x18)])
                    .u32();
                let reference = e.call(FIND_REFERENCE_FOR_3D, &args![node]).u32();
                if reference != 0 {
                    let form_id = e.call(FORM_ID, &args![reference]).u32();
                    e.mem.set_u32(segment_id, form_id);
                }
            }
            e.call(SAVE_ID, &args![this, segment_id, 4u32]);
            let counted = e.mem.u16(count).wrapping_add(1);
            e.mem.set_u16(count, counted);
            segment = e.mem.u32(segment.wrapping_add(0x1c));
        }
        let counted = e.mem.u16(count);
        e.mem.set_u16(slot, counted);
    });
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x71 {
        e.call(SAVE_DATA, &args![this, base.wrapping_add(0xd4), 4u32]);
    }
}

// Translated from 00814870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00814870 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 8`; the load): slot +0x1C4, the
/// `MagicProjectile` load body, the state (+0xAC, through the manager), +0x88; a loaded
/// record (0x24 bytes, [`fn_00814aa0`]): for a version of 0x30 or more its point (0xC), its
/// four words (0x10, through the manager) and +0x1C; below 0x30 the player's position and the
/// four words at `011a9ea4`; the target's id into +0xC4 (`LoadNumericID`); the segment count
/// (2 bytes) and, when not zero, an array (`count + 1` words, the count first) of the
/// segments' ids at the record's +0x20; the record goes to +0xB0; +0xD4 for a version of
/// 0x71 or more.
pub fn fn_00814870(e: &mut Engine, this: Ptr, buffer: u32, version: u32) {
    let base = this.addr();
    e.vcall(base, 0x1c4, &[]);
    e.call(MAGIC_PROJECTILE_LOAD_GAME, &args![this, buffer, version]);
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    e.call(LOAD_WORD, &args![manager, base.wrapping_add(0xac), 4u32]);
    e.call(LOAD_DATA, &args![this, base.wrapping_add(0x88), 4u32]);
    let block = e.call(ALLOCATE, &args![0x24u32]).u32();
    let record = if block != 0 {
        fn_00814aa0(e, Ptr::new(block)).addr()
    } else {
        0
    };
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x30 {
        e.call(LOAD_DATA, &args![this, record, 0xcu32]);
        let manager = e.mem.u32(SAVE_LOAD_MANAGER);
        e.call(
            LOAD_WORD,
            &args![manager, record.wrapping_add(0xc), 0x10u32],
        );
        e.call(LOAD_DATA, &args![this, record.wrapping_add(0x1c), 4u32]);
    }
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_VERSION, &args![manager]).u8() < 0x30 {
        let player = e.mem.u32(PLAYER);
        let position = e.vcall(player, 0x1f4, &[]).u32();
        let words = read_point(e, position);
        write_point(e, record, words);
        for i in 0..4u32 {
            let word = e.mem.u32(LEGACY_ROTATION.wrapping_add(4 * i));
            e.mem
                .set_u32(record.wrapping_add(0xc).wrapping_add(4 * i), word);
        }
    }
    e.with_stack(8, |e, locals| {
        let id = locals.addr();
        let count = id.wrapping_add(4);
        e.call(LOAD_ID, &args![this, id, 4u32]);
        let target = e.mem.u32(id);
        e.mem.set_u32(base.wrapping_add(0xc4), target);
        e.call(LOAD_DATA, &args![this, count, 2u32]);
        let segments = u32::from(e.mem.u16(count));
        if segments != 0 {
            // `MUL` by 4 with the overflow flag turned into an all-ones size.
            let size = (segments + 1).checked_mul(4).unwrap_or(u32::MAX);
            let array = e.call(ALLOCATE, &args![size]).u32();
            e.mem.set_u32(record.wrapping_add(0x20), array);
            let array = e.mem.u32(record.wrapping_add(0x20));
            e.mem.set_u32(array, segments);
            let array = e.mem.u32(record.wrapping_add(0x20));
            e.call(LOAD_ID, &args![this, array.wrapping_add(4), segments << 2]);
        }
    });
    e.mem.set_u32(base.wrapping_add(0xb0), record);
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x71 {
        e.call(LOAD_DATA, &args![this, base.wrapping_add(0xd4), 4u32]);
    }
}

// Translated from 00814aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The loaded record's constructor (unnamed in the engine map): `006815c0` on +0 and +0xC,
/// +0x20 = null. Returns `this`.
pub fn fn_00814aa0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(SELF, &args![base]);
    e.call(SELF, &args![base.wrapping_add(0xc)]);
    e.mem.set_u32(base.wrapping_add(0x20), 0);
    this
}

// Translated from 00814ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00814ad0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `MagicBoltProjectile::InitLoadGame` (Xbox PDB; `thiscall`, `RET 8`):
/// `MagicProjectile::InitLoadGame`; the loaded target id (+0xC4) is looked up (`004839c0`)
/// and cast to `TESObjectREFR`, and its `"Bip01 Spine2"` node becomes the target node
/// (+0xBC); [`magic_bolt_projectile_init_3d`], `00570f70` with the node at +0xB4, the cell's
/// `AddReference`, and the flight sound (with a node).
pub fn magic_bolt_projectile_init_load_game(e: &mut Engine, this: Ptr, buffer: u32, version: u32) {
    let base = this.addr();
    e.call(
        MAGIC_PROJECTILE_INIT_LOAD_GAME,
        &args![this, buffer, version],
    );
    let id = e.mem.u32(base.wrapping_add(0xc4));
    if id != 0 {
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let reference = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
            )
            .u32();
        e.mem.set_u32(base.wrapping_add(0xc4), reference);
        let reference = e.mem.u32(base.wrapping_add(0xc4));
        if reference != 0 {
            let node = e.vcall(reference, 0x1d0, &[]).u32();
            if node != 0 {
                let spine = spine_node(e, node);
                e.call(POINTER_ASSIGN, &args![base.wrapping_add(0xbc), spine]);
            }
        }
    }
    magic_bolt_projectile_init_3d(e, this);
    let node = e.call(POINTER_GET, &args![base.wrapping_add(0xb4)]).u32();
    e.call(SET_3D, &args![this, node]);
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    e.call(CELL_ADD_REFERENCE, &args![cell, this, 0u32]);
    let form = e.call(BASE_FORM, &args![this]).u32();
    if e.call(PROJECTILE_FLIGHT_SOUND, &args![form]).u32() == 0 {
        return;
    }
    let form = e.call(BASE_FORM, &args![this]).u32();
    let sound = e.call(PROJECTILE_FLIGHT_SOUND, &args![form]).u32();
    let id = e.call(FORM_ID, &args![sound]).u32();
    let handle = base.wrapping_add(0xc8);
    if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
        e.call(SOUND_HANDLE_RELEASE, &args![handle]);
    }
    if ball::node(e, this) != 0 {
        start_flight_sound(e, this, id, true);
    }
}

// Translated from 0081a760 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0081a760 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `MagicBoltProjectile::Collide` (engine map; `thiscall`, `RET 0x18`: four words it never
/// reads, the reference hit and a flag byte): the controller's filter is flagged, +0xB4 =
/// 1, and `(+0xAC - +0xA8) / +0xAC` is the strength; without the flag and with a strength
/// above `0.0` the caster's `FindTargets_ov2` runs with it; without the flag the impact
/// sound plays.
#[allow(clippy::too_many_arguments)]
pub fn magic_bolt_projectile_collide(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    reference: u32,
    flag: u8,
) {
    let base = this.addr();
    flag_controller_filter(e, this);
    e.mem.set_u32(base.wrapping_add(0xb4), 1);
    let total = x87_load32(e.mem.u32(base.wrapping_add(0xac)));
    let left = x87_sub(total, x87_m32(e.mem.u32(base.wrapping_add(0xa8))));
    let strength = x87_store32(x87_div(left, x87_m32(e.mem.u32(base.wrapping_add(0xac)))));
    // FCOMP / TEST AH,0x41 / JNZ: on when above.
    if flag == 0 && x87_load32(strength) > e.mem.f64(ZERO) {
        let [px, py, pz] = ball::position_words(e, this);
        let cell = e.call(PARENT_CELL, &args![this]).u32();
        let kind = e.call(PROJECTILE_KIND, &args![this]).u32();
        let caster = e.call(PROJECTILE_CASTER, &args![this]).u32();
        e.call(
            FIND_TARGETS_OV2,
            &args![caster, kind, cell, px, py, pz, this, reference, 0u32, strength, 1.0f32],
        );
    }
    let form = e.call(BASE_FORM, &args![this]).u32();
    if e.call(PROJECTILE_IMPACT_SOUND, &args![form]).u32() != 0 && flag == 0 {
        play_impact_sound(e, this);
    }
}

#[cfg(test)]
mod magic_bolt_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &addr in addresses {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn slot(family: u32, slot: u32) -> u32 {
        0x0330_0000 + family * 0x1_0000 + slot
    }

    fn object(e: &mut Engine, family: u32, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for i in 0..0x100 {
            e.mem.set_u32(vtable + 4 * i, slot(family, 4 * i));
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Settings answer their own address's low byte as their value (`float` and integer).
    fn settings(e: &mut Engine) {
        e.register(SETTING_VALUE, |e, a| {
            let p = e.mem.alloc(4);
            e.mem.set_f32(p, (a[0] & 0xff) as f32);
            p.into_ret()
        });
        e.register(SETTING_INT_VALUE, |e, a| {
            let p = e.mem.alloc(4);
            e.mem.set_u32(p, a[0] & 0xff);
            p.into_ret()
        });
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_3000,
            0x0101_5000,
            0x0101_7000,
            0x0102_1000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(0x0107_a000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        e.map(0x011a_e000, 0x1000);
        e.map(0x011d_c000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_f32(FLOAT_MAX, f32::MAX);
        e.mem.set_f64(HALF, 0.5);
        e.mem.set_f64(ZERO, 0.0);
        e.mem.set_f64(THREE, 3.0);
        e.mem.set_f32(HUNDREDTH, 0.01);
        e.mem.set_f32(FIVE, 5.0);
        e.mem.set_u32(PLAYER, 0x0bad_0001);
        e.call_log = Some(vec![]);
        e
    }

    /// A bolt projectile whose node and position slots are doubled.
    fn bolt(e: &mut Engine) -> (u32, u32) {
        let this = object(e, 0, 0x100);
        let node = e.mem.alloc(0x100);
        e.register_double(slot(0, 0x1d0), move |_, _| node.into_ret());
        let position = e.mem.alloc(0x10);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        e.register_double(slot(0, 0x1f4), move |_, _| position.into_ret());
        e.register(PROJECTILE_CASTER, |_, _| 0u32.into_ret());
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        (this, node)
    }

    #[test]
    fn small_accessors() {
        let mut e = engine();
        e.call(0x0081_1e00, &args![0x1234u32]);
        assert_eq!(e.mem.u32(BOLT_GLOBAL), 0x1234);
        let this = e.mem.alloc(0x180);
        e.call(0x0081_33d0, &args![this, 2u32]);
        assert_eq!(e.mem.u32(this + 0xac), 2);
        e.mem.set_u32(this + 0x78, 5);
        e.mem.set_u32(this + 0x7c, 5);
        assert!(e.call(0x0081_3370, &args![this]).bool());
        e.mem.set_u32(this + 0x78, (-1i32) as u32);
        assert!(!e.call(0x0081_3370, &args![this]).bool());
        e.mem.set_u32(this + 0x78, 6);
        e.mem.set_u8(this + 0x174, 1);
        assert!(!e.call(0x0081_3370, &args![this]).bool());
        quiet(&mut e, &[SELF]);
        let record = e.mem.alloc(0x24);
        e.mem.set_u32(record + 0x20, 9);
        assert_eq!(e.call(0x0081_4aa0, &args![record]).u32(), record);
        assert_eq!(e.mem.u32(record + 0x20), 0);
        assert_eq!(calls(&e, SELF), vec![vec![record], vec![record + 0xc]]);
    }

    #[test]
    fn segment_constructor_destructor_and_delete() {
        let mut e = engine();
        quiet(
            &mut e,
            &[POINTER_CONSTRUCT, SELF, POINTER_DESTRUCT, DEALLOCATE],
        );
        let segment = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0081_3ea0, &args![segment]).u32(), segment);
        let made: Vec<u32> = calls(&e, POINTER_CONSTRUCT)
            .iter()
            .map(|c| c[0] - segment)
            .collect();
        assert_eq!(made, vec![0, 4, 0x14, 0x18]);
        assert_eq!(calls(&e, SELF), vec![vec![segment + 8]]);
        e.call(0x0081_21a0, &args![segment]);
        let gone: Vec<u32> = calls(&e, POINTER_DESTRUCT)
            .iter()
            .map(|c| c[0] - segment)
            .collect();
        assert_eq!(gone, vec![0x18, 0x14, 4, 0]);
        e.call(0x0081_2170, &args![segment, 0u32]);
        assert!(calls(&e, DEALLOCATE).is_empty());
        assert_eq!(e.call(0x0081_2170, &args![segment, 1u32]).u32(), segment);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![segment]]);
    }

    #[test]
    fn save_size_counts_segments() {
        let mut e = engine();
        e.register(MAGIC_PROJECTILE_SAVE_SIZE, |_, _| 0x10u32.into_ret());
        e.register(SAVE_VERSION, |_, _| 0x71u32.into_ret());
        let this = e.mem.alloc(0x100);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xb0, first);
        e.mem.set_u32(first + 0x1c, second);
        // 0x10 + 8 + 0x20 + 6 + 2 * 4 + 4.
        assert_eq!(e.call(0x0081_4590, &args![this, 0u32]).u16(), 0x4a);
        e.register(SAVE_VERSION, |_, _| 0x2fu32.into_ret());
        assert_eq!(e.call(0x0081_4590, &args![this, 0u32]).u16(), 0x26);
    }

    #[test]
    fn constructor_builds_the_bolt() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        settings(&mut e);
        let caster = object(&mut e, 1, 0x10);
        e.register(slot(1, 0x2c), |_, _| 0x7777u32.into_ret());
        e.register(PROJECTILE_SPEED, |_, _| st0(10.0));
        e.register(BASE_FORM, |_, _| 0x4000u32.into_ret());
        e.register(MAGIC_PROJECTILE_CLONE_3D, |_, _| 0x4100u32.into_ret());
        e.register(GET_CHAR_CONTROLLER, |_, _| 0x6600u32.into_ret());
        e.register(PARENT_CELL, |_, _| 0x5400u32.into_ret());
        e.register(PROJECTILE_FLIGHT_SOUND, |_, _| 0u32.into_ret());
        // Init3D and the controller setup stop early.
        e.register(ALLOCATE_OBJECT, |_, _| 0u32.into_ret());
        e.register(ANGLES, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(EXTRA_DATA_KEY, |_, _| 0x99u32.into_ret());
        e.register(GET_EXTRA_DATA, |_, _| 0x9900u32.into_ret());
        e.register(BOLT_SHADER_START, |_, _| 0u32.into_ret());
        e.register(RUNNING_TOTAL_ADD, |_, _| Ret::default());
        quiet(
            &mut e,
            &[
                MAGIC_PROJECTILE_CONSTRUCTOR,
                SOUND_HANDLE_CONSTRUCT,
                NODE_PLACE,
                NODE_SET_TRANSLATE,
                BSX_FLAGS_SET,
                SELF,
                MATRIX_FROM_ANGLES,
                NODE_SET_ROTATE,
                SET_3D,
                VECTOR4_SET,
                SET_PROXY_VELOCITY,
                CELL_ADD_REFERENCE,
                CHECK_SHOT_BEHIND_WALL,
                POINTER_DESTRUCT,
            ],
        );
        // The constructor puts the game's vtable in place; its position slot is doubled too.
        e.mem
            .set_u32(MAGIC_BOLT_PROJECTILE_VTABLE + 0x1f4, slot(0, 0x1f4));
        let returned = e
            .call(
                0x0081_1980,
                &args![this, caster, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32],
            )
            .u32();
        assert_eq!(returned, this);
        assert_eq!(
            calls(&e, MAGIC_PROJECTILE_CONSTRUCTOR),
            vec![vec![this, caster, 2, 3, 4, 5, 6, 7, 8, 9, 10]]
        );
        assert_eq!(e.mem.u32(this), MAGIC_BOLT_PROJECTILE_VTABLE);
        assert_eq!(e.mem.u32(BOLT_GLOBAL), 0xec);
        // 10 * the setting at 011d16f8 (0xf8).
        assert_eq!(e.mem.f32(this + 0x88), 2480.0);
        assert_eq!(e.mem.u32(this + 0xac), 0);
        assert_eq!(
            calls(&e, VECTOR4_SET)[0][1..],
            [(-f32::MAX).to_bits(), 0, 0, 0]
        );
        assert_eq!(calls(&e, SET_PROXY_VELOCITY)[0][0], 0x6600);
        assert_eq!(calls(&e, CELL_ADD_REFERENCE), vec![vec![0x5400, this, 0]]);
        assert_eq!(
            calls(&e, RUNNING_TOTAL_ADD),
            vec![vec![(0x58u32 as f32).to_bits()]]
        );
        assert!(calls(&e, SET_LOCATION_ON_REFERENCE).is_empty());
    }

    #[test]
    fn destructor_releases_everything() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        settings(&mut e);
        e.register(PROJECTILE_CASTER, |_, _| 0x3300u32.into_ret());
        e.register(PROCESS_LIST, |_, _| 0x7100u32.into_ret());
        e.register(PARENT_NODE, |_, _| 0u32.into_ret());
        e.register(SOUND_HANDLE_IS_VALID, |_, _| 1u32.into_ret());
        quiet(
            &mut e,
            &[
                RUNNING_TOTAL_TAKE,
                LIST_REMOVE,
                BOLT_SHADER_STOP,
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_RELEASE,
                SOUND_HANDLE_DESTRUCT,
                POINTER_DESTRUCT,
                MAGIC_PROJECTILE_DESTRUCTOR,
                DEALLOCATE,
            ],
        );
        e.mem.set_u32(this + 0xa8, 0xa800);
        e.mem.set_u32(this + 0xb4, 0xb400);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first, 0x1100);
        e.mem.set_u32(first + 4, 0x1104);
        e.mem.set_u32(first + 0x1c, second);
        e.mem.set_u32(this + 0xb0, first);
        e.call(0x0081_1e10, &args![this]);
        assert_eq!(calls(&e, RUNNING_TOTAL_TAKE).len(), 1);
        assert_eq!(calls(&e, LIST_REMOVE)[0][0], 0x7100);
        assert_eq!(
            calls(&e, BOLT_SHADER_STOP),
            vec![vec![0xb400, 0xa800], vec![0x1104, 0x1100], vec![0, 0]]
        );
        assert_eq!(e.mem.u32(this + 0xb0), 0);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![first], vec![second]]);
        assert_eq!(calls(&e, SOUND_HANDLE_STOP), vec![vec![this + 0xc8]]);
        assert_eq!(calls(&e, MAGIC_PROJECTILE_DESTRUCTOR), vec![vec![this]]);
        // The player's own bolt is not counted.
        let (other, _) = bolt(&mut e);
        e.mem.set_u32(PLAYER, 0x3300 - 0x88);
        e.register(PROJECTILE_CASTER, |_, _| 0x3300u32.into_ret());
        e.call(0x0081_1e10, &args![other]);
        assert_eq!(calls(&e, RUNNING_TOTAL_TAKE).len(), 1);
    }

    /// Doubles for the shader configuration and node making.
    fn shader_rig(e: &mut Engine) {
        settings(e);
        e.register(ALLOCATE_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NODE_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(COLOR_SET, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        quiet(
            e,
            &[
                SHADER_SET_INT_A70,
                SHADER_SET_821620,
                SHADER_SET_507780,
                SHADER_SET_5F9B10,
                SHADER_SET_A90,
                SHADER_SET_COLOR_AB0,
                SHADER_SET_COLOR_AE0,
                SHADER_SET_B30,
                SHADER_SET_INT_8126E0,
                SHADER_SET_5077A0,
                SHADER_SET_B50,
                SHADER_SET_WORD_A50,
                SHADER_FINISH,
            ],
        );
    }

    #[test]
    fn init_3d_makes_the_nodes_and_configures_the_shader() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        shader_rig(&mut e);
        e.register(EXTRA_DATA_KEY, |_, _| 0x99u32.into_ret());
        e.register(GET_EXTRA_DATA, |_, _| 0u32.into_ret());
        e.register(BSX_FLAGS_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(ANGLES, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(BOLT_SHADER_START, |_, _| 0x5a00u32.into_ret());
        quiet(
            &mut e,
            &[
                NODE_PLACE,
                NODE_SET_TRANSLATE,
                ADD_EXTRA_DATA,
                BSX_FLAGS_SET,
                SELF,
                MATRIX_FROM_ANGLES,
                NODE_SET_ROTATE,
            ],
        );
        e.call(0x0081_2220, &args![this]);
        let follow = e.mem.u32(this + 0xc0);
        let node = e.mem.u32(this + 0xb4);
        assert_ne!(follow, 0);
        assert_eq!(calls(&e, NODE_PLACE)[0][0], follow);
        assert_eq!(calls(&e, ADD_EXTRA_DATA)[0][..2], [node, 0x99]);
        let flags = calls(&e, ADD_EXTRA_DATA)[0][2];
        assert_eq!(calls(&e, BSX_FLAGS_SET), vec![vec![flags, 1, 1]]);
        assert_eq!(calls(&e, BOLT_SHADER_START), vec![vec![node, 1]]);
        assert_eq!(e.mem.u32(this + 0xa8), 0x5a00);
        assert_eq!(calls(&e, SHADER_SET_INT_A70), vec![vec![0x5a00, 0x6c]]);
        assert_eq!(
            calls(&e, SHADER_SET_COLOR_AB0),
            vec![vec![
                0x5a00,
                (0xe0u32 as f32).to_bits(),
                8.0f32.to_bits(),
                (0xf0u32 as f32).to_bits(),
                1.0f32.to_bits()
            ]]
        );
        assert_eq!(calls(&e, SHADER_SET_WORD_A50), vec![vec![0x5a00, 1]]);
        assert_eq!(calls(&e, SHADER_FINISH), vec![vec![0x5a00]]);
        // Without a shader nothing is configured.
        e.register(BOLT_SHADER_START, |_, _| 0u32.into_ret());
        e.call(0x0081_2220, &args![this]);
        assert_eq!(calls(&e, SHADER_FINISH).len(), 1);
    }

    #[test]
    fn segment_update_follows_its_nodes() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.mem.set_f32(this + 0x88, 7.0);
        e.mem.set_f32(this + 0xd4, 1.0);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        let source = e.mem.alloc(0x10);
        e.mem.set_f32(source, 4.0);
        let target = e.mem.alloc(0x10);
        e.mem.set_f32(target, 10.0);
        e.register(NODE_POSITION, |_, a| a[0].into_ret());
        e.register(POINT_DIFFERENCE, |e, a| {
            let value = e.mem.f32(a[0]) - e.mem.f32(a[2]);
            e.mem.set_f32(a[1], value);
            a[1].into_ret()
        });
        e.register(POINT_LENGTH, |e, a| st0(f64::from(e.mem.f32(a[0]))));
        e.register(FLOAT_A4, |_, _| st0(4.0));
        settings(&mut e);
        e.register(ATAN2, |_, _| st0(0.25));
        quiet(
            &mut e,
            &[
                NODE_SET_TRANSLATE,
                SHADER_UPDATE_SEGMENT,
                SELF,
                MATRIX_FROM_EULER_ZYX,
                NODE_SET_ROTATE,
            ],
        );
        let segment = e.mem.alloc(0x20);
        e.mem.set_u32(segment, 0x5a00);
        e.mem.set_u32(segment + 4, 0x4400);
        e.mem.set_u32(segment + 0x14, source);
        e.mem.set_u32(segment + 0x18, target);
        e.call(0x0081_2700, &args![this, 0.5f32, segment]);
        assert_eq!(e.mem.f32(segment + 8), 4.0);
        assert_eq!(
            calls(&e, NODE_SET_TRANSLATE),
            vec![vec![0x4400, segment + 8]]
        );
        // Age 4 - 1 = 3 is at least the setting at 011d1b14 (0x14 = 20)? No.
        assert_eq!(
            calls(&e, SHADER_UPDATE_SEGMENT),
            vec![vec![
                0x5a00,
                0.5f32.to_bits(),
                6.0f32.to_bits(),
                7.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(
            calls(&e, MATRIX_FROM_EULER_ZYX)[0][1..],
            [0.25f32.to_bits(), 0, (-0.25f32).to_bits()]
        );
        assert_eq!(calls(&e, NODE_SET_ROTATE)[0][0], 0x4400);
        e.register(FLOAT_A4, |_, _| st0(30.0));
        e.call(0x0081_2700, &args![this, 0.5f32, segment]);
        assert_eq!(calls(&e, SHADER_UPDATE_SEGMENT)[1][4], 1);
    }

    #[test]
    fn post_process_needs_magic_node_controller_and_node() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        let caster = object(&mut e, 1, 0x10);
        let actor = object(&mut e, 2, 0x100);
        let magic_caster_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(magic_caster_vtable + 0x30, slot(3, 0x30));
        e.mem.set_u32(actor + 0x88, magic_caster_vtable);
        e.register_double(PROJECTILE_CASTER, move |_, _| caster.into_ret());
        e.register_double(slot(1, 0x2c), move |_, _| actor.into_ret());
        e.register(slot(2, 0x100), |_, _| 1u32.into_ret());
        e.register(slot(3, 0x30), |_, _| 0x3030u32.into_ret());
        e.register(GET_CHAR_CONTROLLER, |_, _| 0x6600u32.into_ret());
        e.register(FLOAT_90, |_, _| st0(1.0));
        e.register(NODE_POSITION, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(POINT_LENGTH, |_, _| st0(1.0));
        e.register(ATAN2, |_, _| st0(0.0));
        quiet(
            &mut e,
            &[
                SELF,
                0x0068_15c0,
                GET_POSITION_OV2,
                COPY_POINT,
                POINT_DIFFERENCE,
                MATRIX_FROM_EULER_ZYX,
                NODE_SET_ROTATE,
                NODE_SET_TRANSLATE,
                UPDATE_DATA_INIT,
                NODE_UPDATE,
            ],
        );
        assert!(e.call(0x0081_2890, &args![this]).bool());
        e.mem.set_u32(this + 0xb4, 0x4400);
        assert!(!e.call(0x0081_2890, &args![this]).bool());
        assert_eq!(calls(&e, NODE_SET_ROTATE)[0][0], 0x4400);
        assert!(calls(&e, NODE_SET_TRANSLATE).is_empty());
        assert_eq!(calls(&e, NODE_UPDATE)[0][0], 0x4400);
        assert_eq!(calls(&e, GET_POSITION_OV2)[0][0], 0x6600);
        // The player's bolt, low: the node moves to the magic node.
        e.mem.set_u32(PLAYER, actor);
        e.register(FLOAT_90, |_, _| st0(0.25));
        e.register(PLAYER_FLAG, |_, _| 0u32.into_ret());
        assert!(!e.call(0x0081_2890, &args![this]).bool());
        assert_eq!(calls(&e, NODE_SET_TRANSLATE).len(), 1);
        assert_eq!(calls(&e, NODE_POSITION).last().unwrap()[0], 0x4400);
    }

    #[test]
    fn update_flies_and_finishes() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        settings(&mut e);
        e.mem.set_f32(this + 0x88, 10.0);
        let running = e.mem.alloc(0x180);
        e.mem.set_u8(running + 0x174, 1);
        e.mem.set_u32(this + 0xa8, running);
        e.register(REFERENCE_FLAG, |_, _| 0u32.into_ret());
        e.register(NODE_WORLD_TRANSLATE, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(GET_CHAR_CONTROLLER, |_, _| 0u32.into_ret());
        e.register(CONTROLLER_FLOAT, |_, _| st0(0.0));
        e.register(HEIGHT_OFFSET, |_, _| st0(0.0));
        e.register(POINT_DIFFERENCE, |_, a| a[1].into_ret());
        e.register(POINT_LENGTH, |_, _| st0(300.0));
        e.register(slot(0, 0x30c), |_, _| 1u32.into_ret());
        e.register(SOUND_HANDLE_IS_VALID, |_, _| 0u32.into_ret());
        quiet(
            &mut e,
            &[
                POINT_SET,
                slot(0, 0x250),
                slot(0, 0xc4),
                slot(0, 0x304),
                NODE_PLACE,
            ],
        );
        // The aiming is skipped: the integer setting at 011d1afc answers 0xfc.
        e.call(0x0081_2b60, &args![this, 0.5f32]);
        assert!(calls(&e, slot(0, 0x304)).is_empty());
        assert_eq!(calls(&e, POINT_SET)[0][1..], [0, 5.0f32.to_bits(), 0]);
        assert_eq!(e.mem.f32(this + 0x8c), 300.0);
        // 300 is beyond the distance setting (0x00 = 0): slot +0xC4 with 1.
        assert_eq!(calls(&e, slot(0, 0xc4)), vec![vec![this, 1]]);
        // The shader is not done and the state is 0: nothing more.
        assert_eq!(calls(&e, slot(0, 0xc4)).len(), 1);
        // A zero time does nothing.
        e.call(0x0081_2b60, &args![this, 0.0f32]);
        assert_eq!(calls(&e, POINT_SET).len(), 1);
        // A done shader with no segments ends the bolt.
        let shader = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0xa8, shader);
        e.mem.set_u32(this + 0xac, 2);
        e.call(0x0081_2b60, &args![this, 0.5f32]);
        assert_eq!(calls(&e, slot(0, 0xc4)).len(), 2);
    }

    #[test]
    fn update_removes_finished_segments() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        settings(&mut e);
        e.mem.set_u32(this + 0xac, 1);
        e.register(REFERENCE_FLAG, |_, _| 0u32.into_ret());
        e.register(NODE_WORLD_TRANSLATE, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(NODE_POSITION, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(POINT_LENGTH, |_, _| st0(1.0));
        e.register(FLOAT_A4, |_, _| st0(0.0));
        e.register(ATAN2, |_, _| st0(0.0));
        e.register(PARENT_NODE, |_, _| 0u32.into_ret());
        quiet(
            &mut e,
            &[
                NODE_SET_TRANSLATE,
                POINT_DIFFERENCE,
                SHADER_UPDATE_SEGMENT,
                SELF,
                MATRIX_FROM_EULER_ZYX,
                NODE_SET_ROTATE,
                POINTER_DESTRUCT,
                DEALLOCATE,
            ],
        );
        let done = e.mem.alloc(0x180);
        let running = e.mem.alloc(0x180);
        e.mem.set_u8(running + 0x174, 1);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        let third = e.mem.alloc(0x20);
        e.mem.set_u32(first, running);
        e.mem.set_u32(second, done);
        e.mem.set_u32(third, done);
        e.mem.set_u32(first + 0x1c, second);
        e.mem.set_u32(second + 0x1c, third);
        e.mem.set_u32(this + 0xb0, first);
        // The main shader is not done and the age (0) is not above 3: nothing more.
        let shader = e.mem.alloc(0x180);
        e.mem.set_u8(shader + 0x174, 1);
        e.mem.set_u32(this + 0xa8, shader);
        e.call(0x0081_2b60, &args![this, 0.5f32]);
        assert_eq!(e.mem.u32(this + 0xb0), first);
        assert_eq!(e.mem.u32(first + 0x1c), 0);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![second], vec![third]]);
        assert_eq!(calls(&e, SHADER_UPDATE_SEGMENT).len(), 3);
    }

    #[test]
    fn hit_targets_and_makes_the_effect() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        settings(&mut e);
        let reference = object(&mut e, 4, 0x10);
        e.register(slot(4, 0x100), |_, _| 0u32.into_ret());
        e.register(slot(4, 0x1d0), |_, _| 0u32.into_ret());
        e.register(FLOAT_A4, |_, _| st0(12.5));
        e.register(GET_CHAR_CONTROLLER, |_, _| 0u32.into_ret());
        e.register(NODE_TRANSFORM, |_, a| a[0].into_ret());
        e.register(NODE_TRANSFORM_34, |_, a| a[0].into_ret());
        e.register(TRANSFORM_APPLY, |_, a| a[1].into_ret());
        e.register(NODE_WORLD_TRANSLATE, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(POINT_DIFFERENCE, |_, a| a[1].into_ret());
        e.register(POINT_LENGTH, |_, _| st0(42.0));
        e.register(PARENT_CELL, |_, _| 0x5400u32.into_ret());
        e.register(PROJECTILE_KIND, |_, _| 0x1du32.into_ret());
        e.register(BASE_FORM, |e, _| {
            let form = e.mem.alloc(0x40);
            e.mem.set_u32(form + 0x18, 0x0300_0000);
            form.into_ret()
        });
        e.put_vtable(0x0300_0000, &[0, 0, 0, 0, 0, 0x0310_0000]);
        e.register(0x0310_0000, |_, _| 0x7a00u32.into_ret());
        e.register(PROJECTILE_EFFECT_ITEM, |_, _| 0u32.into_ret());
        e.register(EFFECT_ITEM_GET_AREA, |_, _| 3u32.into_ret());
        e.register(MAKE_TEMP_EFFECT, |_, _| 0x7e00u32.into_ret());
        e.register(PROJECTILE_IMPACT_SOUND, |_, _| 0u32.into_ret());
        quiet(
            &mut e,
            &[
                PROJECTILE_EXPLODE_AT,
                SHADER_UPDATE,
                FIND_TARGETS_OV2,
                TEMP_EFFECT_SET_IDLE,
            ],
        );
        e.call(
            0x0081_33f0,
            &args![this, 1.0f32, 2.0f32, 3.0f32, 0x66u32, reference, 0u8],
        );
        assert_eq!(e.mem.f32(this + 0xd4), 12.5);
        assert_eq!(e.mem.u32(this + 0xac), 1);
        assert_eq!(e.mem.u32(this + 0xc4), 0);
        assert_eq!(calls(&e, PROJECTILE_EXPLODE_AT)[0][7], 0x66);
        assert_eq!(
            calls(&e, SHADER_UPDATE),
            vec![vec![0, 0.01f32.to_bits(), 42.0f32.to_bits(), 0, 0]]
        );
        let find = &calls(&e, FIND_TARGETS_OV2)[0];
        assert_eq!(find[1..3], [0x1d, 0x5400]);
        assert_eq!(find[7], reference);
        let effect = &calls(&e, MAKE_TEMP_EFFECT)[0];
        assert_eq!(effect[..3], [0x5400, 5.0f32.to_bits(), 0x7a00]);
        assert_eq!(
            effect[6..],
            [
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                1.0f32.to_bits(),
                7,
                0
            ]
        );
        assert_eq!(
            calls(&e, TEMP_EFFECT_SET_IDLE),
            vec![vec![0x7e00, TEXT_SPECIAL_IDLE_AREA_EFFECT]]
        );
        // With the flag no targets are looked for.
        e.call(
            0x0081_33f0,
            &args![this, 1.0f32, 2.0f32, 3.0f32, 0x66u32, reference, 1u8],
        );
        assert_eq!(calls(&e, FIND_TARGETS_OV2).len(), 1);
    }

    #[test]
    fn chaining_makes_a_segment_in_front() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        shader_rig(&mut e);
        let target = object(&mut e, 5, 0x10);
        let reference = object(&mut e, 6, 0x10);
        let root = object(&mut e, 7, 0x10);
        let target_node = object(&mut e, 8, 0x10);
        e.register_double(slot(5, 4), move |_, _| reference.into_ret());
        e.register_double(slot(6, 0x1d0), move |_, _| target_node.into_ret());
        e.register(slot(8, 0x9c), |_, _| 0u32.into_ret());
        e.register_double(CELL_ROOT_NODE, move |_, _| root.into_ret());
        e.register(PARENT_CELL, |_, _| 0x5400u32.into_ret());
        e.register(BOLT_SHADER_START, |_, _| 0x5b00u32.into_ret());
        e.register(GET_CHAR_CONTROLLER, |_, _| 0x6600u32.into_ret());
        e.register(GET_POSITION_OV2, |_, _| Ret::default());
        e.register(COPY_POINT, |e, a| {
            e.mem.set_f32(a[0], 9.0);
            Ret::default()
        });
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NODE_POSITION, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(POINT_LENGTH, |_, _| st0(1.0));
        e.register(FLOAT_A4, |_, _| st0(0.0));
        e.register(ATAN2, |_, _| st0(0.0));
        quiet(
            &mut e,
            &[
                slot(7, 0xdc),
                SELF,
                FIXED_STRING_DESTRUCT,
                NODE_SET_TRANSLATE,
                POINT_DIFFERENCE,
                SHADER_UPDATE_SEGMENT,
                MATRIX_FROM_EULER_ZYX,
                NODE_SET_ROTATE,
            ],
        );
        let old = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xb0, old);
        e.call(0x0081_39f0, &args![this, target, 0u32]);
        let segment = e.mem.u32(this + 0xb0);
        assert_ne!(segment, old);
        assert_eq!(e.mem.u32(segment), 0x5b00);
        let node = e.mem.u32(segment + 4);
        assert_eq!(calls(&e, slot(7, 0xdc))[0][1..], [node, 1]);
        assert_eq!(e.mem.f32(segment + 8), 9.0);
        assert_eq!(e.mem.u32(segment + 0x14), 0);
        assert_eq!(e.mem.u32(segment + 0x18), target_node);
        assert_eq!(e.mem.u32(segment + 0x1c), old);
        assert_eq!(calls(&e, SHADER_SET_WORD_A50), vec![vec![0x5b00, 0]]);
        assert_eq!(calls(&e, SHADER_UPDATE_SEGMENT).len(), 1);
        // The same target again with the projectile's own item: nothing.
        e.mem.set_u32(this + 0xc4, reference);
        e.register(FORM_ID, |_, _| 0x1du32.into_ret());
        e.register(PROJECTILE_EFFECT_ITEM, |_, _| 0x1du32.into_ret());
        e.call(0x0081_39f0, &args![this, target, 0x2200u32]);
        assert_eq!(e.mem.u32(this + 0xb0), segment);
    }

    #[test]
    fn controller_setup_runs_the_steps_in_the_bolt_order() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        let process = object(&mut e, 9, 0x10);
        e.mem.set_u32(this + 0x68, process);
        e.register(OBJECT_BUSY, |_, _| 0u32.into_ret());
        e.register(GET_CHAR_CONTROLLER, |_, _| 0u32.into_ret());
        e.register(PARENT_CELL, |_, _| 0u32.into_ret());
        e.register(SAVE_LOAD_FLAG, |_, _| 1u32.into_ret());
        e.register(ALLOCATE_OBJECT, |_, _| 0u32.into_ret());
        e.register(ALLOCATE_CONTROLLER, |_, _| 0x6000u32.into_ret());
        e.register(CHARACTER_CONTROLLER_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(CONTROLLER_FLAG_SOURCE, |_, _| 1u32.into_ret());
        e.register(CONTROLLER_NODE, |_, _| 0u32.into_ret());
        e.register(CONTROLLER_WORLD, |_, _| 0u32.into_ret());
        quiet(
            &mut e,
            &[
                CONTROLLER_INFO_CONSTRUCT,
                INFO_SET_FLOAT_8C,
                CONTROLLER_INFO_SET_NODE,
                INFO_SET_POSITION,
                CONTROLLER_INFO_SET_WORLD,
                CONTROLLER_INFO_SET_FLOAT,
                INFO_SET_SHAPE,
                INFO_SET_BYTE,
                INFO_SET_FILTER,
                INFO_SET_WORD,
                CONTROLLER_SET_FLOAT_548,
                CONTROLLER_FLAG_80000,
                CONTROLLER_FLAG_100000,
                slot(9, 0x290),
                CONTROLLER_SET_OWNER,
                CONTROLLER_INFO_DESTRUCT,
                CONTROLLER_SET_FLOAT,
                CONTROLLER_SET_WORLD,
                CONTROLLER_SET_NODE,
                WORLD_SET_MOTION,
            ],
        );
        e.call(0x0081_3f30, &args![this]);
        let log = e.call_log.as_ref().unwrap();
        let position = |addr: u32| log.iter().position(|(a, _)| *a == addr).unwrap();
        assert!(position(CONTROLLER_FLAG_80000) < position(CONTROLLER_FLAG_100000));
        assert_eq!(calls(&e, INFO_SET_FILTER)[0][1..], [7, 0]);
        assert_eq!(calls(&e, CONTROLLER_FLAG_100000), vec![vec![0x6000, 1]]);
        assert_eq!(calls(&e, slot(9, 0x290))[0][1], 0x6000);
        // No world: the lowest float, and no position.
        assert_eq!(calls(&e, CONTROLLER_SET_FLOAT), vec![vec![0x6000, 0]]);
        assert!(calls(&e, CONTROLLER_SET_POSITION).is_empty());
    }

    #[test]
    fn save_writes_the_segment_ids() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        e.register(SAVE_VERSION, |_, _| 0x71u32.into_ret());
        e.register(NODE_WORLD_TRANSLATE, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(SHADER_FLOAT, |_, _| st0(0.5));
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e.register(FIND_REFERENCE_FOR_3D, |_, a| a[0].into_ret());
        let slot_word = e.mem.alloc(4);
        e.register_double(RESERVE_COUNT, move |_, _| slot_word.into_ret());
        e.register_double(SAVE_ID, |e, a| {
            let value = e.mem.u32(a[1]);
            (value).into_ret()
        });
        quiet(
            &mut e,
            &[
                MAGIC_PROJECTILE_SAVE_GAME,
                SAVE_WORD,
                SAVE_DATA,
                SELF,
                NODE_ROTATION,
            ],
        );
        e.mem.set_u32(this + 0xa8, 0x5a00);
        e.mem.set_u32(this + 0xc4, 0x7000);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0x18, 0x8000);
        e.mem.set_u32(first + 0x1c, second);
        e.mem.set_u32(this + 0xb0, first);
        e.call(0x0081_4670, &args![this, 0x77u32]);
        assert_eq!(e.mem.u16(slot_word), 2);
        let saved = calls(&e, SAVE_DATA);
        let sizes: Vec<u32> = saved.iter().map(|c| c[2]).collect();
        assert_eq!(sizes, vec![4, 0xc, 4, 2, 4]);
        assert_eq!(calls(&e, SAVE_WORD)[1][2], 0x10);
        assert_eq!(calls(&e, SAVE_ID).len(), 3);
        assert_eq!(saved.last().unwrap()[1], this + 0xd4);
    }

    #[test]
    fn load_reads_the_record_and_the_ids() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        e.register(SAVE_VERSION, |_, _| 0x2fu32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LOAD_ID, |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], 0x7001);
            }
            Ret::default()
        });
        e.register(LOAD_DATA, |e, a| {
            if a[2] == 2 {
                e.mem.set_u16(a[1], 3);
            }
            Ret::default()
        });
        let player = object(&mut e, 10, 0x10);
        e.mem.set_u32(PLAYER, player);
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position + 8, 6.0);
        e.register_double(slot(10, 0x1f4), move |_, _| position.into_ret());
        e.mem.set_f32(LEGACY_ROTATION, 1.0);
        quiet(
            &mut e,
            &[slot(0, 0x1c4), MAGIC_PROJECTILE_LOAD_GAME, LOAD_WORD, SELF],
        );
        e.call(0x0081_4870, &args![this, 0x77u32, 3u32]);
        let record = e.mem.u32(this + 0xb0);
        assert_eq!(e.mem.f32(record + 8), 6.0);
        assert_eq!(e.mem.f32(record + 0xc), 1.0);
        assert_eq!(e.mem.u32(this + 0xc4), 0x7001);
        let array = e.mem.u32(record + 0x20);
        assert_eq!(e.mem.u32(array), 3);
        assert_eq!(calls(&e, ALLOCATE)[1], vec![0x10]);
        assert_eq!(calls(&e, LOAD_ID)[1], vec![this, array + 4, 0xc]);
    }

    #[test]
    fn init_load_game_finds_the_target_and_restarts_the_sound() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        let reference = object(&mut e, 11, 0x10);
        e.mem.set_u32(this + 0xc4, 0x1234);
        e.register(LOOKUP_FORM, |_, a| (a[0] + 1).into_ret());
        e.register_double(RT_DYNAMIC_CAST, move |_, _| reference.into_ret());
        e.register(slot(11, 0x1d0), |_, _| 0u32.into_ret());
        // Init3D stops at its missing shader.
        e.register(ALLOCATE_OBJECT, |_, _| 0u32.into_ret());
        e.register(ANGLES, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(GET_EXTRA_DATA, |_, _| 0x9900u32.into_ret());
        e.register(BOLT_SHADER_START, |_, _| 0u32.into_ret());
        e.register(PROJECTILE_FLIGHT_SOUND, |_, _| 0x4500u32.into_ret());
        e.register(FORM_ID, |_, _| 0x45u32.into_ret());
        e.register(SOUND_HANDLE_IS_VALID, |_, _| 0u32.into_ret());
        e.register(GET_SOUND_HANDLE_BY_ID, |_, a| a[1].into_ret());
        quiet(
            &mut e,
            &[
                MAGIC_PROJECTILE_INIT_LOAD_GAME,
                NODE_PLACE,
                NODE_SET_TRANSLATE,
                EXTRA_DATA_KEY,
                BSX_FLAGS_SET,
                SELF,
                MATRIX_FROM_ANGLES,
                NODE_SET_ROTATE,
                SET_3D,
                PARENT_CELL,
                CELL_ADD_REFERENCE,
                BASE_FORM,
                AUDIO_INSTANCE,
                SOUND_HANDLE_ASSIGN,
                SOUND_HANDLE_DESTRUCT,
                SOUND_HANDLE_SET_POSITION,
                SOUND_HANDLE_FOLLOW,
                SOUND_HANDLE_PLAY,
            ],
        );
        e.call(0x0081_4ad0, &args![this, 0x77u32, 3u32]);
        assert_eq!(e.mem.u32(this + 0xc4), reference);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![0x1235, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0]]
        );
        assert_eq!(calls(&e, GET_SOUND_HANDLE_BY_ID)[0][2..], [0x45, 0x102]);
        assert_eq!(
            calls(&e, SOUND_HANDLE_SET_POSITION),
            vec![vec![
                this + 0xc8,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]]
        );
        assert_eq!(calls(&e, SOUND_HANDLE_PLAY), vec![vec![this + 0xc8, 1]]);
    }

    #[test]
    fn collide_finds_targets_with_the_remaining_strength() {
        let mut e = engine();
        let (this, _) = bolt(&mut e);
        e.mem.set_f32(this + 0xac, 4.0);
        e.mem.set_f32(this + 0xa8, 1.0);
        e.register(GET_CHAR_CONTROLLER, |_, _| 0x6600u32.into_ret());
        e.register(PARENT_CELL, |_, _| 0x5400u32.into_ret());
        e.register(PROJECTILE_KIND, |_, _| 0x1du32.into_ret());
        e.register(PROJECTILE_IMPACT_SOUND, |_, _| 0u32.into_ret());
        quiet(
            &mut e,
            &[
                CONTROLLER_FILTER,
                FILTER_FLAG,
                SET_COLLISION_FILTER,
                FIND_TARGETS_OV2,
                BASE_FORM,
            ],
        );
        e.call(
            0x0081_a760,
            &args![this, 0u32, 0u32, 0u32, 0u32, 0x8800u32, 0u8],
        );
        assert_eq!(e.mem.u32(this + 0xb4), 1);
        assert_eq!(calls(&e, FILTER_FLAG)[0][1], 1);
        let find = &calls(&e, FIND_TARGETS_OV2)[0];
        assert_eq!(find[7..], [0x8800, 0, 0.75f32.to_bits(), 1.0f32.to_bits()]);
        e.mem.set_f32(this + 0xa8, 4.0);
        e.call(
            0x0081_a760,
            &args![this, 0u32, 0u32, 0u32, 0u32, 0x8800u32, 0u8],
        );
        assert_eq!(calls(&e, FIND_TARGETS_OV2).len(), 1);
    }
}

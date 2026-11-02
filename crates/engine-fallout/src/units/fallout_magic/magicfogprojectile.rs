//! `fallout/magic/magicfogprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00819a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a float setter (`thiscall`, `RET 4`): stores the argument at
/// +0x18 through the x87 stack (`FLD`/`FSTP`, so a signalling NaN is stored quiet).
pub fn fn_00819a50(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x18, x87_quiet_bits(value.to_bits()));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_9a50, fn_00819a50(Ptr, f32)),
        entry!(
            0x0081_9360,
            magic_fog_projectile_constructor(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(0x0081_96e0, fn_008196e0(Ptr)),
        entry!(0x0081_9830, fn_00819830(Ptr)),
        entry!(0x0081_9890, fn_00819890(Ptr, u32)),
        entry!(0x0081_9a70, fn_00819a70(Ptr, f32)),
        entry!(0x0081_9a90, magic_fog_projectile_init_3d(Ptr)),
        entry!(0x0081_9ca0, fn_00819ca0(Ptr)),
        entry!(0x0081_a320, magic_fog_projectile_update(Ptr, f32)),
        entry!(0x0081_ade0, magic_fog_projectile_save_game(Ptr, u32)),
        entry!(0x0081_afa0, fn_0081afa0(Ptr, u32, u32)),
        entry!(
            0x0081_b180,
            magic_fog_projectile_init_load_game(Ptr, u32, u32)
        ),
        entry!(
            0x0081_b430,
            magic_fog_projectile_finish_init_load_game(Ptr, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_setter_stores_at_0x18_quieting_nans() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x20));
        fn_00819a50(&mut e, this, 2.5);
        assert_eq!(e.mem.f32(this.addr() + 0x18), 2.5);
        fn_00819a50(&mut e, this, f32::from_bits(0x7f80_0001));
        assert_eq!(e.mem.u32(this.addr() + 0x18), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this.addr() + 0x14), 0);
    }
}

// ---------------------------------------------------------------------------
// MagicFogProjectile (Xbox PDB): the fog projectile's own functions.
// ---------------------------------------------------------------------------

use crate::units::fallout_magic::magicballprojectile as ball;

/// `MagicFogProjectile`'s two vtables (`0107ad84` at +0, `0107ad78` at +0x18).
const MAGIC_FOG_PROJECTILE_VTABLE: u32 = 0x0107_ad84;
const MAGIC_FOG_PROJECTILE_SECOND_VTABLE: u32 = 0x0107_ad78;
/// `MagicProjectile::MagicProjectile` (`0081dd00`, `RET 0x28`), `~MagicProjectile`
/// (`0081de70`), `CheckShotBehindWall` (`0081eba0`), `SaveGame` (`0081e8c0`), the load body
/// (`0081e980`), `InitLoadGame` (`0081ea40`); `MobileObject::FinishInitLoadGame`
/// (`00932520`).
const MAGIC_PROJECTILE_CONSTRUCTOR: u32 = 0x0081_dd00;
const MAGIC_PROJECTILE_DESTRUCTOR: u32 = 0x0081_de70;
const CHECK_SHOT_BEHIND_WALL: u32 = 0x0081_eba0;
const MAGIC_PROJECTILE_SAVE_GAME: u32 = 0x0081_e8c0;
const MAGIC_PROJECTILE_LOAD_GAME: u32 = 0x0081_e980;
const MAGIC_PROJECTILE_INIT_LOAD_GAME: u32 = 0x0081_ea40;
const MOBILE_OBJECT_FINISH_INIT_LOAD_GAME: u32 = 0x0093_2520;
/// `0081e570` / `00570f70`: the constructor's two alternatives for its eleventh word.
const PROJECTILE_WITHOUT_LOCATION: u32 = 0x0081_e570;
const PROJECTILE_WITH_LOCATION: u32 = 0x0057_0f70;
/// The sound handle at +0xBC: constructor, `IsValid`, `Stop`, `Release`, assignment,
/// destructor, `SetVolume` (`00ad89e0`, `RET 4`); `00933150` makes one.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
const SOUND_HANDLE_SET_VOLUME: u32 = 0x00ad_89e0;
const MAKE_SOUND_HANDLE: u32 = 0x0093_3150;
/// `004fd400` the base form; `004fd3c0` / `004fd380` the projectile form's flight sound and
/// light (meanings not confirmed); `0084e3a0` a form's id; `009b0bd0` the speed (`ST0`).
const BASE_FORM: u32 = 0x004f_d400;
const PROJECTILE_FLIGHT_SOUND: u32 = 0x004f_d3c0;
const PROJECTILE_LIGHT: u32 = 0x004f_d380;
const FORM_ID: u32 = 0x0084_e3a0;
const PROJECTILE_SPEED: u32 = 0x009b_0bd0;
/// `00586150` the projectile's effect item, `EffectItem::GetDuration` (`004044d0`) and
/// `GetArea` (`00404420`); `00404010` (`cdecl`, two `float`s, `ST0`).
const PROJECTILE_EFFECT_ITEM: u32 = 0x0058_6150;
const EFFECT_ITEM_GET_DURATION: u32 = 0x0040_44d0;
const EFFECT_ITEM_GET_AREA: u32 = 0x0040_4420;
const SCALED_VALUE: u32 = 0x0040_4010;
/// `00403e20`: a `float` setting's value address.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// Settings: the duration factor (`011d1b74`), the area factor (`011d15c8`), the speed
/// scale (`011d16f8`), the running-total part (`011d15f8`), the damping (`011d1ad8`), the
/// distance limit (`011d1d00`), the pulse period (`011d17f4`), and the three particle
/// scales (`011d1890`, `011d1bd4`, `011d19f0`).
const DURATION_SETTING: u32 = 0x011d_1b74;
const AREA_SETTING: u32 = 0x011d_15c8;
const SPEED_SETTING: u32 = 0x011d_16f8;
const COUNT_SETTING: u32 = 0x011d_15f8;
const DAMPING_SETTING: u32 = 0x011d_1ad8;
const DISTANCE_SETTING: u32 = 0x011d_1d00;
const PULSE_SETTING: u32 = 0x011d_17f4;
const PARTICLE_SCALE_SETTING: u32 = 0x011d_1890;
const PARTICLE_SIZE_SETTING: u32 = 0x011d_1bd4;
const PARTICLE_SPREAD_SETTING: u32 = 0x011d_19f0;
/// The player (`011dea3c`) and the projectile's caster (`009185e0`).
const PLAYER: u32 = 0x011d_ea3c;
const PROJECTILE_CASTER: u32 = 0x0091_85e0;
/// `0080fae0` / `0080fc90` (the running total), `0080fb00` (the controller's proxy
/// velocity), `0080fcc0` (the `MagicLight` deleting destructor), `00810a80`, `008106b0`,
/// `00810fd0`, `00810ff0`, `00810550`, `00810510`, `008105c0`, `00810530`, `00810590`,
/// `00810570`, `00810610`, `008105e0`, `00810640`, `00810660` (`magicballprojectile.rs`).
const RUNNING_TOTAL_ADD: u32 = 0x0080_fae0;
const RUNNING_TOTAL_TAKE: u32 = 0x0080_fc90;
const SET_PROXY_VELOCITY: u32 = 0x0080_fb00;
const MAGIC_LIGHT_DELETE: u32 = 0x0080_fcc0;
const CONTROLLER_FLOAT: u32 = 0x0081_0a80;
const CONTROLLER_SET_FLOAT_548: u32 = 0x0081_06b0;
const PROJECTILE_KIND: u32 = 0x0081_0fd0;
const SET_COLLISION_FILTER: u32 = 0x0081_0ff0;
const INFO_SET_FLOAT_8C: u32 = 0x0081_0550;
const INFO_SET_POSITION: u32 = 0x0081_0510;
const INFO_SET_SHAPE: u32 = 0x0081_05c0;
const INFO_SET_BYTE: u32 = 0x0081_0530;
const INFO_SET_FILTER: u32 = 0x0081_0590;
const INFO_SET_WORD: u32 = 0x0081_0570;
const CONTROLLER_FLAG_100000: u32 = 0x0081_0610;
const CONTROLLER_FLAG_80000: u32 = 0x0081_05e0;
const CONTROLLER_SET_OWNER: u32 = 0x0081_0640;
const CONTROLLER_NODE: u32 = 0x0081_0660;
/// `MobileObject::GetCharController` (`009306d0`), the cell (`008d6f30`), its
/// `AddReference` (`00548230`), world (`004543c0`) and water height (`005471e0`).
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
const PARENT_CELL: u32 = 0x008d_6f30;
const CELL_ADD_REFERENCE: u32 = 0x0054_8230;
const CELL_HAVOK_WORLD: u32 = 0x0045_43c0;
const CELL_WATER_HEIGHT: u32 = 0x0054_71e0;
/// The controller construction and placement (as in `magicballprojectile.rs`).
const OBJECT_BUSY: u32 = 0x0045_cd60;
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
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const CONTROLLER_SET_FLOAT: u32 = 0x0057_b1d0;
const CONTROLLER_WORLD: u32 = 0x0062_1ad0;
const CONTROLLER_SET_WORLD: u32 = 0x00c7_0c90;
const CONTROLLER_SET_NODE: u32 = 0x00c6_d7a0;
const CONTROLLER_SET_POSITION: u32 = 0x0056_20e0;
const CONTROLLER_FILTER: u32 = 0x0070_c440;
const WORLD_FLAG: u32 = 0x004b_ae00;
const SET_WIREFRAME: u32 = 0x00b6_b910;
const WORLD_SET_MOTION: u32 = 0x00c6_a350;
const WORLD_REMOVE_OBJECTS: u32 = 0x00c6_9ee0;
const WORLD_UPDATE_POSITION: u32 = 0x00c6_9f50;
const FILTER_LAYER: u32 = 0x004a_3a20;
const FILTER_SET_LAYER: u32 = 0x004a_39f0;
const FILTER_SET_GROUP: u32 = 0x0059_ce80;
const ACTOR_COLLISION_INFO: u32 = 0x0093_1ed0;
/// The node work: `00440490` (scale), `00440460` (translation), `0043fa80` (rotation),
/// `00430830` (angles), `006815c0` (answers `this`), `00a59660`, `ProcessSelectiveUpdate`
/// (`00c4b500`), fixed strings, slot +0x9C, the controller manager and sequence helpers,
/// `0043d410` / `00a59c60` (an update).
const NODE_SET_SCALE: u32 = 0x0044_0490;
const NODE_SET_TRANSLATE: u32 = 0x0044_0460;
const NODE_SET_ROTATE: u32 = 0x0043_fa80;
const ANGLES: u32 = 0x0043_0830;
const SELF: u32 = 0x0068_15c0;
const MATRIX_FROM_ANGLES: u32 = 0x00a5_9660;
const PROCESS_SELECTIVE_UPDATE: u32 = 0x00c4_b500;
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const UPDATE_DATA_INIT: u32 = 0x0043_d410;
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `"SpecialIdle_Projectile"` and `"AreaEffect"`.
const TEXT_SPECIAL_IDLE_PROJECTILE: u32 = 0x0107_a874;
const TEXT_AREA_EFFECT: u32 = 0x0107_a88c;
/// The particle work: `0043b300` (`cdecl`: a kind test against `012024e0`), `004d7d50` /
/// `004d7d70` (the controllers), `00653270` with `01202550` / `01202540` (two controller
/// kinds), `00759450`, `00639aa0`, `004a7bd0`, `00792760` (`ST0` getters), `0098adb0`,
/// `00444020` (`RET 4` setters), and the children (`0043b480` count, `0043b4a0` one).
const IS_KIND_OF: u32 = 0x0043_b300;
const PARTICLE_SYSTEM_KIND: u32 = 0x0120_24e0;
const CONTROLLER_COUNT: u32 = 0x004d_7d50;
const CONTROLLER_AT: u32 = 0x004d_7d70;
const CONTROLLER_OF_KIND: u32 = 0x0065_3270;
const EMITTER_KIND: u32 = 0x0120_2550;
const SPAWNER_KIND: u32 = 0x0120_2540;
const EMITTER_RATE: u32 = 0x0075_9450;
const SPAWNER_RATE: u32 = 0x0063_9aa0;
const SPAWNER_SIZE: u32 = 0x004a_7bd0;
const SPAWNER_SPREAD: u32 = 0x0079_2760;
const SPAWNER_SET_RATE: u32 = 0x0098_adb0;
const SPAWNER_SET_SPREAD: u32 = 0x0044_4020;
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
/// Point helpers (`00416870`, `00439ef0`, `00457990`, `005532a0`), `004587d0` and the
/// reference flag `00440d80`.
const POINT_SET: u32 = 0x0041_6870;
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const POINT_LENGTH: u32 = 0x0045_7990;
const VECTOR4_SET: u32 = 0x0055_32a0;
const HEIGHT_OFFSET: u32 = 0x0045_87d0;
const REFERENCE_FLAG: u32 = 0x0044_0d80;
/// `MagicLight` functions: `MagicLight::MagicLight` (`0081bf80`), `Fade` (`0081c470`),
/// `Transfer` (`0081c4a0`), the update (`0081c320`), `00621b00` / `00621b20`.
const MAGIC_LIGHT_CONSTRUCTOR: u32 = 0x0081_bf80;
const MAGIC_LIGHT_FADE: u32 = 0x0081_c470;
const MAGIC_LIGHT_TRANSFER: u32 = 0x0081_c4a0;
const MAGIC_LIGHT_UPDATE: u32 = 0x0081_c320;
const MAGIC_LIGHT_GET_FLOAT: u32 = 0x0062_1b00;
const MAGIC_LIGHT_SET_FLOAT: u32 = 0x0062_1b20;
/// `00401000` / `00401030` (`operator new` / `delete`), `00401460` (a memory copy).
const ALLOCATE: u32 = 0x0040_1000;
const DEALLOCATE: u32 = 0x0040_1030;
const COPY_MEMORY: u32 = 0x0040_1460;
/// Save / load (as in `magicballprojectile.rs`), `00825c00` (a count slot), `00406e70`
/// (`RET 4`: an effect item's index) and `00406ee0` (`RET 4`: the effect item at an index),
/// `0081aa80` (`RET 0xC`: an affected-target record), `004839c0` (a form by id).
const SAVE_DATA: u32 = 0x0048_4ce0;
const LOAD_DATA: u32 = 0x0048_4d00;
const SAVE_ID: u32 = 0x0048_4d20;
const LOAD_ID: u32 = 0x0048_4d40;
const SAVE_LOAD_MANAGER: u32 = 0x011d_e45c;
const SAVE_VERSION: u32 = 0x008d_f040;
const SAVE_WORD: u32 = 0x0085_79b0;
const LOAD_WORD: u32 = 0x0085_79e0;
const RESERVE_COUNT: u32 = 0x0082_5c00;
const EFFECT_ITEM_INDEX: u32 = 0x0040_6e70;
const EFFECT_ITEM_AT: u32 = 0x0040_6ee0;
const TARGET_RECORD_CONSTRUCT: u32 = 0x0081_aa80;
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` and the type descriptors `.?AVTESForm@@`, `.?AVTESObjectREFR@@` and
/// `.?AVActor@@`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_ACTOR: u32 = 0x0118_46d4;
/// `00406cc0` (`cdecl`, a `float` in `ST0`) and `_ftol2` (`00ec62c0`).
const ROUND_DOWN: u32 = 0x0040_6cc0;
const FTOL: u32 = 0x00ec_62c0;
/// `MagicCaster::FindTargets_ov2` (`00817160`).
const FIND_TARGETS_OV2: u32 = 0x0081_7160;
/// Constants: `3.4028235e38` (`float`, `0107ad40`), the frame time (`011c3c08`), `0.1`
/// (`0101ffa0`), `60.0` (`01012638`), `0.0` (`01012060`), `5.0` (`01020998`), `2.0`
/// (`01011590`), `8.0` (`010240b0`) (`double`s), `-3.4028235e38` (`float`, `01015f5c`) and
/// the default point (`011f426c`).
const FLOAT_MAX: u32 = 0x0107_ad40;
const FRAME_TIME: u32 = 0x011c_3c08;
const TENTH: u32 = 0x0101_ffa0;
const SIXTY: u32 = 0x0101_2638;
const ZERO: u32 = 0x0101_2060;
const FIVE: u32 = 0x0102_0998;
const TWO: u32 = 0x0101_1590;
const EIGHT: u32 = 0x0102_40b0;
const LOWEST_FLOAT: u32 = 0x0101_5f5c;
const DEFAULT_POINT: u32 = 0x011f_426c;

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

fn setting_load(e: &mut Engine, setting: u32) -> f64 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_load32(e.mem.u32(value))
}

/// The running total: `0080fae0` (`add`) or `0080fc90` with the setting at `011d15f8`, for
/// casters other than the player.
fn count_projectile(e: &mut Engine, this: Ptr, add: bool) {
    if ball::caster_reference(e, this) != e.mem.u32(PLAYER) {
        let value = ball::setting_bits(e, COUNT_SETTING);
        let target = if add {
            RUNNING_TOTAL_ADD
        } else {
            RUNNING_TOTAL_TAKE
        };
        e.call(target, &args![value]);
    }
}

/// `00404010(value, the setting)` of an effect item word, as the constructor computes the
/// lifetime (+0xAC) and the size (+0xB0).
fn scaled_field(e: &mut Engine, this: Ptr, setting: u32, getter: u32) -> u32 {
    let factor = ball::setting_bits(e, setting);
    let item = e.call(PROJECTILE_EFFECT_ITEM, &args![this]).u32();
    let value = e.call(getter, &args![item]).i32();
    let value = x87_store32(f64::from(value));
    x87_store32(e.call(SCALED_VALUE, &args![value, factor]).f64())
}

/// Remakes the flight sound handle (+0xBC) from the projectile form's flight sound.
fn remake_flight_sound(e: &mut Engine, this: Ptr) {
    let handle = this.addr().wrapping_add(0xbc);
    let form = e.call(BASE_FORM, &args![this]).u32();
    let sound = e.call(PROJECTILE_FLIGHT_SOUND, &args![form]).u32();
    let id = e.call(FORM_ID, &args![sound]).u32();
    if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
        e.call(SOUND_HANDLE_RELEASE, &args![handle]);
    }
    e.with_stack(0x0c, |e, temporary| {
        let made = e
            .call(
                MAKE_SOUND_HANDLE,
                &args![this, temporary, id, 1u32, 0x4000_0102u32, 1u32],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![temporary]);
    });
}

fn has_flight_sound(e: &mut Engine, this: Ptr) -> bool {
    let form = e.call(BASE_FORM, &args![this]).u32();
    e.call(PROJECTILE_FLIGHT_SOUND, &args![form]).u32() != 0
}

// Translated from 00819360 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00819360 - the FS:[0] exception frame and the /GS cookie check (00ec408c) are not translated
/// `MagicFogProjectile::MagicFogProjectile` (Xbox PDB; `thiscall`, `RET 0x30`): the
/// `MagicProjectile` constructor (the ten words passed on), the vtables, the sound handle
/// (+0xBC), the lifetime +0xAC (`00404010` of the effect item's duration with the setting
/// at `011d1b74`), the size +0xB0 (of its area with `011d15c8`), the speed +0x88, +0xB4 = 0,
/// +0xA8 = `0.0`, +0xB8 = `1.0`, +0xCC = +0xC8 = null; `00570f70` / `0081e570`,
/// [`magic_fog_projectile_init_3d`], [`fn_00819ca0`], the controller's velocity
/// `(-3.4028235e38, 0, 0, 0)`, the cell's `AddReference`, the flight sound (volume `1.0`),
/// the light handed over and faded over the lifetime, `CheckShotBehindWall` and the running
/// total. Returns `this`.
#[allow(clippy::too_many_arguments)]
pub fn magic_fog_projectile_constructor(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    first_x: u32,
    first_y: u32,
    first_z: u32,
    second_x: u32,
    second_y: u32,
    second_z: u32,
    location: u32,
    light: u32,
) -> Ptr {
    let base = this.addr();
    e.call(
        MAGIC_PROJECTILE_CONSTRUCTOR,
        &args![
            this, word_1, word_2, word_3, word_4, first_x, first_y, first_z, second_x, second_y,
            second_z
        ],
    );
    e.mem.set_u32(base, MAGIC_FOG_PROJECTILE_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x18), MAGIC_FOG_PROJECTILE_SECOND_VTABLE);
    e.call(SOUND_HANDLE_CONSTRUCT, &args![base.wrapping_add(0xbc)]);
    let lifetime = scaled_field(e, this, DURATION_SETTING, EFFECT_ITEM_GET_DURATION);
    e.mem.set_u32(base.wrapping_add(0xac), lifetime);
    let size = scaled_field(e, this, AREA_SETTING, EFFECT_ITEM_GET_AREA);
    e.mem.set_u32(base.wrapping_add(0xb0), size);
    let form = e.call(BASE_FORM, &args![this]).u32();
    let speed = e.call(PROJECTILE_SPEED, &args![form]).f64();
    let speed = x87_store32(x87_mul(setting_load(e, SPEED_SETTING), speed));
    e.mem.set_u32(base.wrapping_add(0x88), speed);
    e.mem.set_u32(base.wrapping_add(0xb4), 0);
    e.mem.set_u32(base.wrapping_add(0xa8), 0.0f32.to_bits());
    e.mem.set_u32(base.wrapping_add(0xb8), 1.0f32.to_bits());
    e.mem.set_u32(base.wrapping_add(0xcc), 0);
    e.mem.set_u32(base.wrapping_add(0xc8), 0);
    if location != 0 {
        e.call(PROJECTILE_WITH_LOCATION, &args![this, location]);
    } else {
        e.call(PROJECTILE_WITHOUT_LOCATION, &args![this, 0u32]);
    }
    magic_fog_projectile_init_3d(e, this);
    fn_00819ca0(e, this);
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
    if has_flight_sound(e, this) {
        remake_flight_sound(e, this);
        e.call(
            SOUND_HANDLE_SET_VOLUME,
            &args![base.wrapping_add(0xbc), 1.0f32],
        );
    }
    if light != 0 {
        e.call(MAGIC_LIGHT_TRANSFER, &args![light, this]);
        e.mem.set_u32(base.wrapping_add(0xcc), light);
        let lifetime = x87_quiet_bits(e.mem.u32(base.wrapping_add(0xac)));
        let light = e.mem.u32(base.wrapping_add(0xcc));
        e.call(MAGIC_LIGHT_FADE, &args![light, 0u32, lifetime]);
    }
    e.call(CHECK_SHOT_BEHIND_WALL, &args![this]);
    count_projectile(e, this, true);
    this
}

// Translated from 008196e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008196e0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// The `MagicFogProjectile` destructor (unnamed in the engine map): the vtables, the
/// running total taken back for other casters, the affected-target list freed
/// ([`fn_00819830`]), a valid sound stopped and released, the light deleted, the handle and
/// `~MagicProjectile`.
pub fn fn_008196e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, MAGIC_FOG_PROJECTILE_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x18), MAGIC_FOG_PROJECTILE_SECOND_VTABLE);
    count_projectile(e, this, false);
    fn_00819830(e, this);
    let handle = base.wrapping_add(0xbc);
    if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
        e.call(SOUND_HANDLE_STOP, &args![handle]);
        e.call(SOUND_HANDLE_RELEASE, &args![handle]);
    }
    let light = e.mem.u32(base.wrapping_add(0xcc));
    // The exe tests the word, then its copy.
    let object = light;
    if light != 0 && object != 0 {
        e.call(MAGIC_LIGHT_DELETE, &args![object, 1u32]);
    }
    e.mem.set_u32(base.wrapping_add(0xcc), 0);
    e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    e.call(MAGIC_PROJECTILE_DESTRUCTOR, &args![this]);
}

// Translated from 00819830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): frees the affected-target list at +0xC8 (each
/// record's next at +8, `operator delete`) and clears it.
pub fn fn_00819830(e: &mut Engine, this: Ptr) {
    let head = this.addr().wrapping_add(0xc8);
    while e.mem.u32(head) != 0 {
        let record = e.mem.u32(head);
        let next = e.mem.u32(record.wrapping_add(8));
        e.mem.set_u32(head, next);
        e.call(DEALLOCATE, &args![record]);
    }
    e.mem.set_u32(head, 0);
}

// Translated from 00819890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a node): scales the fog's particles. For
/// a particle system (`0043b300` with `012024e0`), each controller of the emitter kind gets
/// [`fn_00819a70`] with its rate times `+0xAC * setting 011d1890`, and each of the spawner
/// kind its rate (`0098adb0`), size ([`fn_00819a50`], times `+0xB0 * setting 011d19f0`) and
/// spread (`00444020`, times `+0xB0 * setting 011d1bd4`); any other node with children (its
/// slot +0xC) passes this on to each child.
pub fn fn_00819890(e: &mut Engine, this: Ptr, node: u32) {
    let base = this.addr();
    let rate = scaled_by_setting(e, base.wrapping_add(0xac), PARTICLE_SCALE_SETTING);
    let spread = scaled_by_setting(e, base.wrapping_add(0xb0), PARTICLE_SIZE_SETTING);
    let size = scaled_by_setting(e, base.wrapping_add(0xb0), PARTICLE_SPREAD_SETTING);
    if e.call(IS_KIND_OF, &args![PARTICLE_SYSTEM_KIND, node])
        .bool()
    {
        let mut index = 0u32;
        while index < e.call(CONTROLLER_COUNT, &args![node]).u32() {
            let controller = e.call(CONTROLLER_AT, &args![node, index]).u32();
            let emitter = e
                .call(CONTROLLER_OF_KIND, &args![EMITTER_KIND, controller])
                .u32();
            if emitter != 0 {
                let value = e.call(EMITTER_RATE, &args![emitter]).f64();
                let value = x87_store32(x87_mul(value, x87_m32(rate)));
                fn_00819a70(e, Ptr::new(emitter), f32::from_bits(value));
            }
            let spawner = e
                .call(CONTROLLER_OF_KIND, &args![SPAWNER_KIND, controller])
                .u32();
            if spawner != 0 {
                let value = e.call(SPAWNER_RATE, &args![spawner]).f64();
                let value = x87_store32(x87_mul(value, x87_m32(rate)));
                e.call(SPAWNER_SET_RATE, &args![spawner, value]);
                let value = e.call(SPAWNER_SIZE, &args![spawner]).f64();
                let value = x87_store32(x87_mul(value, x87_m32(size)));
                fn_00819a50(e, Ptr::new(spawner), f32::from_bits(value));
                let value = e.call(SPAWNER_SPREAD, &args![spawner]).f64();
                let value = x87_store32(x87_mul(value, x87_m32(spread)));
                e.call(SPAWNER_SET_SPREAD, &args![spawner, value]);
            }
            index = index.wrapping_add(1);
        }
    } else if node != 0 && e.vcall(node, 0xc, &[]).u32() != 0 {
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![node]).u32() {
            let child = e.call(CHILD_AT, &args![node, index]).u32();
            fn_00819890(e, this, child);
            index = index.wrapping_add(1);
        }
    }
}

// Translated from 00819a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores the `float` at +0x2C (`FLD` /
/// `FSTP`: a signalling NaN is stored quiet).
pub fn fn_00819a70(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0x2c),
        x87_quiet_bits(value.to_bits()),
    );
}

// Translated from 00819a90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00819a90 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `MagicFogProjectile::Init3D` (Xbox PDB): with a node, its scale (+0xB8), translation,
/// rotation and selective update, as the ball's; the `"AreaEffect"` child's particles are
/// scaled ([`fn_00819890`]); and a `"SpecialIdle_Projectile"` sequence is started and the
/// node updated with the frame time minus `0.1`.
pub fn magic_fog_projectile_init_3d(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let node = ball::node(e, this);
    if node == 0 {
        return;
    }
    let scale = x87_quiet_bits(e.mem.u32(base.wrapping_add(0xb8)));
    e.call(NODE_SET_SCALE, &args![node, scale]);
    let position = e.vcall(base, 0x1f4, &[]).u32();
    e.call(NODE_SET_TRANSLATE, &args![node, position]);
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
        e.call(NODE_SET_ROTATE, &args![node, matrix]);
    });
    e.call(PROCESS_SELECTIVE_UPDATE, &args![node]);
    let child = e.with_stack(4, |e, name| {
        let name_pointer = e
            .call(FIXED_STRING_CONSTRUCT, &args![name, TEXT_AREA_EFFECT])
            .u32();
        let child = e.vcall(node, 0x9c, &args![name_pointer]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![name]);
        child
    });
    if child != 0 {
        fn_00819890(e, this, child);
    }
    let manager = ball::controller_manager(e, node);
    if manager == 0 {
        return;
    }
    let sequence = ball::find_sequence(e, manager, TEXT_SPECIAL_IDLE_PROJECTILE);
    if sequence == 0 {
        return;
    }
    start_sequence(e, manager, sequence);
    let time = x87_store32(x87_sub(x87_load32(e.mem.u32(FRAME_TIME)), e.mem.f64(TENTH)));
    e.with_stack(0x0c, |e, update| {
        e.call(UPDATE_DATA_INIT, &args![update, time, 1u32, 0u32]);
        e.call(NODE_UPDATE, &args![node, update]);
    });
}

/// The `float` at `field` times a setting's value (`FLD field`, `FMUL [value]`).
fn scaled_by_setting(e: &mut Engine, field: u32, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_store32(x87_mul(
        x87_load32(e.mem.u32(field)),
        x87_m32(e.mem.u32(value)),
    ))
}

/// Starts the sequence from its beginning, as the ball's helper, with the fog's
/// `-3.4028235e38` (`0107ad40`).
fn start_sequence(e: &mut Engine, manager: u32, sequence: u32) {
    e.call(DEACTIVATE_ALL, &args![manager, 0.0f32]);
    e.call(
        ACTIVATE_SEQUENCE,
        &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
    );
    e.call(MANAGER_FLAG, &args![manager, 1u32]);
    let lowest = x87_store32(-x87_load32(e.mem.u32(FLOAT_MAX)));
    e.call(SEQUENCE_SET_FLOAT, &args![sequence, lowest]);
}

/// `NiControllerManager::DeactivateAll` (`0048fef0`), the sequence activation (`0047aab0`),
/// `0047aa40` and the sequence's `0098adb0`.
const DEACTIVATE_ALL: u32 = 0x0048_fef0;
const ACTIVATE_SEQUENCE: u32 = 0x0047_aab0;
const MANAGER_FLAG: u32 = 0x0047_aa40;
const SEQUENCE_SET_FLOAT: u32 = 0x0098_adb0;

// Translated from 00819ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00819ca0 - the FS:[0] exception frame and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map: the fog's character controller setup, the ball's steps
/// (`0080feb0` in `magicballprojectile.rs`) except that the construction info gets the
/// reference's slot +0x2E4 (not `1.0`), the caster reference is asked before the save/load
/// manager's flag, and a node change with wireframe also runs slot +0x310.
pub fn fn_00819ca0(e: &mut Engine, this: Ptr) {
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
        controller = e.with_stack(0xc8, |e, info| build_fog_controller(e, this, info, world));
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
            e.vcall(base, 0x310, &[]);
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

/// [`fn_00819ca0`]'s controller construction on the local construction info.
fn build_fog_controller(e: &mut Engine, this: Ptr, info: Ptr, world: u32) -> u32 {
    let base = this.addr();
    e.call(CONTROLLER_INFO_CONSTRUCT, &args![info, 1u32]);
    let value = x87_store32(e.vcall(base, 0x2e4, &[]).f64());
    e.call(INFO_SET_FLOAT_8C, &args![info, value]);
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
    let reference = ball::caster_reference(e, this);
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_LOAD_FLAG, &args![manager]).bool() {
        e.call(INFO_SET_FILTER, &args![info, 7u32, 0u32]);
    } else {
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
    let flag = e.call(CONTROLLER_FLAG_SOURCE, &[]).u8();
    e.call(CONTROLLER_FLAG_100000, &args![made, flag]);
    e.call(CONTROLLER_FLAG_80000, &args![made, 1u32]);
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

// Translated from 0081a320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicFogProjectile::Update` (Xbox PDB; `thiscall`, `RET 4`: the elapsed time). At 60 or
/// more slot +0xC4 runs with 1; below `0.0` nothing more. While moving (+0xB4 zero, and
/// `00440d80` false): like the ball (slot +0x304 when low, slot +0x250, the distance), with
/// the controller's +0x548 set to `1.0 - time * setting 011d1ad8`, and a slow step (distance
/// over time at most 5.0) stops it (slot +0x304 with the default point and 0, 0, 0). Then the
/// age (+0xA8) grows by the time: past the lifetime (+0xAC) slot +0xC4 runs with 1;
/// otherwise the light is updated, the sound fades out over the second half of the
/// lifetime, and each time the age crosses a multiple of the pulse setting (`011d17f4`,
/// through `00406cc0` and `_ftol2`) the caster's `FindTargets_ov2` runs with the remaining
/// share of the lifetime.
pub fn magic_fog_projectile_update(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    let time_value = x87_load32(time.to_bits());
    if time_value >= e.mem.f64(SIXTY) {
        e.vcall(base, 0xc4, &args![1u32]);
    }
    if !(time_value >= e.mem.f64(ZERO)) {
        return;
    }
    if e.mem.u32(base.wrapping_add(0xb4)) == 0 {
        if e.call(REFERENCE_FLAG, &args![this]).bool() {
            return;
        }
        fog_move(e, this, time);
    }
    let old_age = x87_quiet_bits(e.mem.u32(base.wrapping_add(0xa8)));
    let age = x87_store32(x87_add(
        x87_load32(e.mem.u32(base.wrapping_add(0xa8))),
        x87_m32(time.to_bits()),
    ));
    e.mem.set_u32(base.wrapping_add(0xa8), age);
    let lifetime = x87_load32(e.mem.u32(base.wrapping_add(0xac)));
    // FCOMPP (the lifetime against the age) / TEST AH,5 / JP: over when the lifetime is
    // below.
    if lifetime < x87_load32(e.mem.u32(base.wrapping_add(0xa8))) {
        e.vcall(base, 0xc4, &args![1u32]);
        return;
    }
    let light = e.mem.u32(base.wrapping_add(0xcc));
    if light != 0 {
        e.call(
            MAGIC_LIGHT_UPDATE,
            &args![light, x87_quiet_bits(time.to_bits())],
        );
    }
    let handle = base.wrapping_add(0xbc);
    if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
        let half = x87_store32(x87_div(
            x87_load32(e.mem.u32(base.wrapping_add(0xac))),
            e.mem.f64(TWO),
        ));
        let age = x87_load32(e.mem.u32(base.wrapping_add(0xa8)));
        // FCOMPP (the half against the age) / TEST AH,5 / JP: on when the half is below.
        if x87_load32(half) < age {
            let past = x87_div(
                x87_sub(
                    x87_load32(e.mem.u32(base.wrapping_add(0xa8))),
                    x87_m32(half),
                ),
                x87_m32(half),
            );
            let volume = x87_store32(x87_mul(x87_sub(1.0, past), 1.0));
            e.call(SOUND_HANDLE_SET_VOLUME, &args![handle, volume]);
        }
    }
    let period = e.call(SETTING_VALUE, &args![PULSE_SETTING]).u32();
    let before = x87_store32(x87_div(x87_load32(old_age), x87_m32(e.mem.u32(period))));
    let before = e.call(ROUND_DOWN, &args![before]).f64();
    let before = e.call(FTOL, &args![before]).u32();
    let period = e.call(SETTING_VALUE, &args![PULSE_SETTING]).u32();
    let now = x87_store32(x87_div(
        x87_load32(e.mem.u32(base.wrapping_add(0xa8))),
        x87_m32(e.mem.u32(period)),
    ));
    let now = e.call(ROUND_DOWN, &args![now]).f64();
    let now = e.call(FTOL, &args![now]).u32();
    if before == now {
        return;
    }
    let share = x87_store32(x87_div(
        x87_sub(
            x87_load32(e.mem.u32(base.wrapping_add(0xac))),
            x87_m32(e.mem.u32(base.wrapping_add(0xa8))),
        ),
        x87_m32(e.mem.u32(base.wrapping_add(0xac))),
    ));
    // FCOMP / TEST AH,0x41 / JNZ: on when above.
    if !(x87_load32(share) > e.mem.f64(ZERO)) {
        return;
    }
    let [px, py, pz] = ball::position_words(e, this);
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    let kind = e.call(PROJECTILE_KIND, &args![this]).u32();
    let caster = e.call(PROJECTILE_CASTER, &args![this]).u32();
    e.call(
        FIND_TARGETS_OV2,
        &args![caster, kind, cell, px, py, pz, this, 0u32, 0u32, share, 1.0f32],
    );
}

/// [`magic_fog_projectile_update`]'s moving part.
fn fog_move(e: &mut Engine, this: Ptr, time: f32) {
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
        let now = step.wrapping_add(0x18);
        let difference = step.wrapping_add(0x24);
        let speed = x87_store32(x87_mul(
            x87_load32(e.mem.u32(base.wrapping_add(0x88))),
            x87_m32(time.to_bits()),
        ));
        e.call(POINT_SET, &args![step, 0.0f32, speed, 0.0f32]);
        let damping = e.call(SETTING_VALUE, &args![DAMPING_SETTING]).u32();
        let damping = x87_store32(x87_sub(
            1.0,
            x87_mul(x87_load32(time.to_bits()), x87_m32(e.mem.u32(damping))),
        ));
        let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
        e.call(
            CONTROLLER_SET_FLOAT_548,
            &args![controller, x87_quiet_bits(damping)],
        );
        let start = ball::position_words(e, this);
        write_point(e, before, start);
        e.vcall(
            base,
            0x250,
            &args![x87_quiet_bits(time.to_bits()), step, 0xfu32],
        );
        if !e.vcall(base, 0x30c, &[]).bool() {
            return;
        }
        let current = ball::position_words(e, this);
        write_point(e, now, current);
        let node = ball::node(e, this);
        e.call(NODE_SET_TRANSLATE, &args![node, now]);
        let out = e
            .call(POINT_DIFFERENCE, &args![now, difference, before])
            .u32();
        let length = x87_store32(e.call(POINT_LENGTH, &args![out]).f64());
        let travelled = base.wrapping_add(0x8c);
        let sum = x87_add(x87_load32(e.mem.u32(travelled)), x87_m32(length));
        e.mem.set_u32(travelled, x87_store32(sum));
        let limit = e.call(SETTING_VALUE, &args![DISTANCE_SETTING]).u32();
        let distance = x87_load32(e.mem.u32(travelled));
        // FCOMPP (the limit against the distance) / TEST AH,5 / JP: on when below.
        if x87_load32(e.mem.u32(limit)) < distance {
            e.vcall(base, 0xc4, &args![1u32]);
        }
        let pace = x87_div(x87_load32(length), x87_m32(time.to_bits()));
        // FCOMP / TEST AH,5 / JP: stopped when at most 5.0 (ordered).
        if pace <= e.mem.f64(FIVE) {
            let [x, y, z] = read_point(e, DEFAULT_POINT);
            e.vcall(base, 0x304, &args![x, y, z, 0u32, 0u32, 0u32]);
        }
    });
}

// Translated from 0081ade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicFogProjectile::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): `MagicProjectile::
/// SaveGame`, +0xA8, +0x88, +0xAC, the state +0xB4 (through the manager), +0xB8; in state 1
/// the light's `00621b00` (`0.0` without one); for a version of 0x71 or more +0xB0, the
/// affected-target count (2 bytes, written back into the slot `00825c00` reserves) and per
/// record the target's id and the index of its effect item in the projectile's spell
/// (`00406e70`, 0 without either).
pub fn magic_fog_projectile_save_game(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    e.call(MAGIC_PROJECTILE_SAVE_GAME, &args![this, buffer]);
    for offset in [0xa8u32, 0x88, 0xac] {
        e.call(SAVE_DATA, &args![this, base.wrapping_add(offset), 4u32]);
    }
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    e.call(SAVE_WORD, &args![manager, base.wrapping_add(0xb4), 4u32]);
    e.call(SAVE_DATA, &args![this, base.wrapping_add(0xb8), 4u32]);
    e.with_stack(0x10, |e, locals| {
        let value = locals.addr();
        let count = value.wrapping_add(4);
        let id = value.wrapping_add(8);
        let index = value.wrapping_add(0xc);
        if e.mem.u32(base.wrapping_add(0xb4)) == 1 {
            e.mem.set_u32(value, 0.0f32.to_bits());
            let light = e.mem.u32(base.wrapping_add(0xcc));
            if light != 0 {
                let saved = x87_store32(e.call(MAGIC_LIGHT_GET_FLOAT, &args![light]).f64());
                e.mem.set_u32(value, saved);
            }
            e.call(SAVE_DATA, &args![this, value, 4u32]);
        }
        let manager = e.mem.u32(SAVE_LOAD_MANAGER);
        if e.call(SAVE_VERSION, &args![manager]).u8() < 0x71 {
            return;
        }
        e.call(SAVE_DATA, &args![this, base.wrapping_add(0xb0), 4u32]);
        e.mem.set_u16(count, 0);
        let manager = e.mem.u32(SAVE_LOAD_MANAGER);
        let slot = e.call(RESERVE_COUNT, &args![manager]).u32();
        e.call(SAVE_DATA, &args![this, count, 2u32]);
        let mut record = e.mem.u32(base.wrapping_add(0xc8));
        while record != 0 {
            e.mem.set_u32(id, 0);
            let target = e.mem.u32(record);
            if target != 0 {
                let form_id = e.call(FORM_ID, &args![target]).u32();
                e.mem.set_u32(id, form_id);
            }
            e.call(SAVE_ID, &args![this, id, 4u32]);
            e.mem.set_u8(index, 0);
            if e.call(PROJECTILE_KIND, &args![this]).u32() != 0
                && e.mem.u32(record.wrapping_add(4)) != 0
            {
                let item = e.mem.u32(record.wrapping_add(4));
                let spell = e.call(PROJECTILE_KIND, &args![this]).u32();
                let found = e
                    .call(EFFECT_ITEM_INDEX, &args![spell.wrapping_add(0xc), item])
                    .u8();
                e.mem.set_u8(index, found);
            }
            e.call(SAVE_DATA, &args![this, index, 1u32]);
            let counted = e.mem.u16(count).wrapping_add(1);
            e.mem.set_u16(count, counted);
            record = e.mem.u32(record.wrapping_add(8));
        }
        let counted = e.mem.u16(count);
        e.mem.set_u16(slot, counted);
    });
}

// Translated from 0081afa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0081afa0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 8`; the load): frees the affected-target list
/// ([`fn_00819830`]), slot +0x1C4, the `MagicProjectile` load body, +0xA8, +0x88, +0xAC, the
/// state +0xB4 (through the manager), +0xB8; in state 1 a `float` copied into +0xCC (taken
/// back by [`magic_fog_projectile_init_load_game`]); for a version of 0x71 or more +0xB0 and
/// the records (an id and an index byte each, 0xC bytes through `0081aa80`, pushed in front
/// of +0xC8).
pub fn fn_0081afa0(e: &mut Engine, this: Ptr, buffer: u32, version: u32) {
    let base = this.addr();
    fn_00819830(e, this);
    e.vcall(base, 0x1c4, &[]);
    e.call(MAGIC_PROJECTILE_LOAD_GAME, &args![this, buffer, version]);
    for offset in [0xa8u32, 0x88, 0xac] {
        e.call(LOAD_DATA, &args![this, base.wrapping_add(offset), 4u32]);
    }
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    e.call(LOAD_WORD, &args![manager, base.wrapping_add(0xb4), 4u32]);
    e.call(LOAD_DATA, &args![this, base.wrapping_add(0xb8), 4u32]);
    e.with_stack(0x10, |e, locals| {
        let value = locals.addr();
        let count = value.wrapping_add(4);
        let id = value.wrapping_add(8);
        let index = value.wrapping_add(0xc);
        if e.mem.u32(base.wrapping_add(0xb4)) == 1 {
            e.call(LOAD_DATA, &args![this, value, 4u32]);
            e.call(COPY_MEMORY, &args![base.wrapping_add(0xcc), value, 4u32]);
        }
        let manager = e.mem.u32(SAVE_LOAD_MANAGER);
        if e.call(SAVE_VERSION, &args![manager]).u8() < 0x71 {
            return;
        }
        e.call(LOAD_DATA, &args![this, base.wrapping_add(0xb0), 4u32]);
        e.call(LOAD_DATA, &args![this, count, 2u32]);
        let mut i = 0u32;
        while i < u32::from(e.mem.u16(count)) {
            e.call(LOAD_ID, &args![this, id, 4u32]);
            e.call(LOAD_DATA, &args![this, index, 1u32]);
            let block = e.call(ALLOCATE, &args![0xcu32]).u32();
            let record = if block != 0 {
                let next = e.mem.u32(base.wrapping_add(0xc8));
                let target = e.mem.u32(id);
                let item = u32::from(e.mem.u8(index));
                e.call(TARGET_RECORD_CONSTRUCT, &args![block, target, item, next])
                    .u32()
            } else {
                0
            };
            e.mem.set_u32(base.wrapping_add(0xc8), record);
            i = i.wrapping_add(1);
        }
    });
}

// Translated from 0081b180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0081b180 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `MagicFogProjectile::InitLoadGame` (Xbox PDB; `thiscall`, `RET 8`):
/// `MagicProjectile::InitLoadGame`, [`magic_fog_projectile_init_3d`], the cell's
/// `AddReference`; outside state 1 the flight sound handle is made again; the word at +0xCC
/// (the saved `float`) is taken into a local and +0xCC cleared; with a node and a light form
/// a new `MagicLight` goes to +0xCC, fades over the lifetime and gets the saved `float`; for
/// a version of 0x71 or more each record's id is resolved to its reference (cast to
/// `TESObjectREFR`) and its index to the spell's effect item (`00406ee0`).
pub fn magic_fog_projectile_init_load_game(e: &mut Engine, this: Ptr, buffer: u32, version: u32) {
    let base = this.addr();
    e.call(
        MAGIC_PROJECTILE_INIT_LOAD_GAME,
        &args![this, buffer, version],
    );
    magic_fog_projectile_init_3d(e, this);
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    e.call(CELL_ADD_REFERENCE, &args![cell, this, 0u32]);
    if e.mem.u32(base.wrapping_add(0xb4)) != 1 && has_flight_sound(e, this) {
        remake_flight_sound(e, this);
    }
    e.with_stack(4, |e, saved| {
        e.call(COPY_MEMORY, &args![saved, base.wrapping_add(0xcc), 4u32]);
        e.mem.set_u32(base.wrapping_add(0xcc), 0);
        if ball::node(e, this) == 0 {
            return;
        }
        let form = e.call(BASE_FORM, &args![this]).u32();
        if e.call(PROJECTILE_LIGHT, &args![form]).u32() == 0 {
            return;
        }
        let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
        let light = if block != 0 {
            let node = ball::node(e, this);
            let form = e.call(BASE_FORM, &args![this]).u32();
            let light_form = e.call(PROJECTILE_LIGHT, &args![form]).u32();
            let id = e.call(FORM_ID, &args![light_form]).u32();
            e.call(MAGIC_LIGHT_CONSTRUCTOR, &args![block, id, node])
                .u32()
        } else {
            0
        };
        e.mem.set_u32(base.wrapping_add(0xcc), light);
        if e.mem.u32(base.wrapping_add(0xcc)) == 0 {
            return;
        }
        let lifetime = x87_quiet_bits(e.mem.u32(base.wrapping_add(0xac)));
        let light = e.mem.u32(base.wrapping_add(0xcc));
        e.call(MAGIC_LIGHT_FADE, &args![light, 0u32, lifetime]);
        let value = x87_quiet_bits(e.mem.u32(saved.addr()));
        let light = e.mem.u32(base.wrapping_add(0xcc));
        e.call(MAGIC_LIGHT_SET_FLOAT, &args![light, value]);
    });
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_VERSION, &args![manager]).u8() < 0x71 {
        return;
    }
    let mut record = e.mem.u32(base.wrapping_add(0xc8));
    while record != 0 {
        let id = e.mem.u32(record);
        let index = e.mem.u8(record.wrapping_add(4));
        if id != 0 {
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let reference = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
                )
                .u32();
            e.mem.set_u32(record, reference);
        }
        if e.call(PROJECTILE_KIND, &args![this]).u32() != 0 {
            let spell = e.call(PROJECTILE_KIND, &args![this]).u32();
            let item = e
                .call(
                    EFFECT_ITEM_AT,
                    &args![spell.wrapping_add(0xc), u32::from(index)],
                )
                .u32();
            e.mem.set_u32(record.wrapping_add(4), item);
        }
        record = e.mem.u32(record.wrapping_add(8));
    }
}

// Translated from 0081b430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicFogProjectile::FinishInitLoadGame` (Xbox PDB; `thiscall`, `RET 8`):
/// `MobileObject::FinishInitLoadGame`; with a controller, its filter word gets layer 7 and
/// the group of the caster actor (the caster reference cast to `Actor`), of the caster
/// reference's node, or of this node, and is set (`00810ff0`); then the running total for
/// other casters (`0080fae0` with the setting at `011d15f8`).
pub fn magic_fog_projectile_finish_init_load_game(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    version: u32,
) {
    e.call(
        MOBILE_OBJECT_FINISH_INIT_LOAD_GAME,
        &args![this, buffer, version],
    );
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
    if controller != 0 {
        let has_reference = e.call(PROJECTILE_CASTER, &args![this]).u32() != 0 && {
            let caster = e.call(PROJECTILE_CASTER, &args![this]).u32();
            e.vcall(caster, 0x2c, &[]).u32() != 0
        };
        let group = if has_reference {
            let caster = e.call(PROJECTILE_CASTER, &args![this]).u32();
            let reference = e.vcall(caster, 0x2c, &[]).u32();
            let actor = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![reference, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
                )
                .u32();
            if actor != 0 {
                e.with_stack(4, |e, out| {
                    let info = e.call(ACTOR_COLLISION_INFO, &args![actor, out]).u32();
                    e.call(FILTER_LAYER, &args![info]).u32()
                })
            } else {
                let reference_node = e.vcall(reference, 0x1d0, &[]).u32();
                ball::node_layer(e, reference_node)
            }
        } else {
            let own_node = ball::node(e, this);
            ball::node_layer(e, own_node)
        };
        e.with_stack(4, |e, filter| {
            e.call(CONTROLLER_FILTER, &args![controller, filter]);
            e.call(FILTER_SET_LAYER, &args![filter, 7u32]);
            e.call(FILTER_SET_GROUP, &args![filter, group]);
            let word = e.mem.u32(filter.addr());
            e.call(SET_COLLISION_FILTER, &args![controller, word]);
        });
    }
    count_projectile(e, this, true);
}

#[cfg(test)]
mod magic_fog_tests {
    use super::*;

    /// `0043b230` (the node's controller source) and `0047a520` (a sequence by name), as the
    /// ball's helpers call them.
    const CONTROLLER_MANAGER_SOURCE: u32 = 0x0043_b230;
    const FIND_SEQUENCE_BY_NAME: u32 = 0x0047_a520;

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
        0x0360_0000 + family * 0x1_0000 + slot
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

    /// Settings answer the low byte of their address as their value.
    fn settings(e: &mut Engine) {
        e.register(SETTING_VALUE, |e, a| {
            let p = e.mem.alloc(4);
            e.mem.set_f32(p, (a[0] & 0xff) as f32);
            p.into_ret()
        });
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_5000,
            0x0101_f000,
            0x0102_0000,
            0x0102_4000,
            0x0107_a000,
            0x011c_3000,
            0x011d_e000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(FLOAT_MAX, f32::MAX);
        e.mem.set_f64(SIXTY, 60.0);
        e.mem.set_f64(ZERO, 0.0);
        e.mem.set_f64(FIVE, 5.0);
        e.mem.set_f64(TWO, 2.0);
        e.mem.set_f64(TENTH, 0.1);
        e.mem.set_u32(PLAYER, 0x0bad_0001);
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32).into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// A fog projectile with a node and a position (1, 2, 3).
    fn fog(e: &mut Engine) -> (u32, u32) {
        let this = object(e, 0, 0x100);
        let node = object(e, 1, 0x100);
        e.register_double(slot(0, 0x1d0), move |_, _| node.into_ret());
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        e.register_double(slot(0, 0x1f4), move |_, _| position.into_ret());
        e.register(PROJECTILE_CASTER, |_, _| 0u32.into_ret());
        (this, node)
    }

    #[test]
    fn small_ones() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.call(0x0081_9a70, &args![this, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x7fc0_0001);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first + 8, second);
        e.mem.set_u32(this + 0xc8, first);
        quiet(&mut e, &[DEALLOCATE]);
        e.call(0x0081_9830, &args![this]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![first], vec![second]]);
        assert_eq!(e.mem.u32(this + 0xc8), 0);
    }

    #[test]
    fn particles_are_scaled_through_the_tree() {
        let mut e = engine();
        settings(&mut e);
        let this = e.mem.alloc(0x100);
        e.mem.set_f32(this + 0xac, 2.0);
        e.mem.set_f32(this + 0xb0, 3.0);
        let system = object(&mut e, 2, 0x10);
        let parent = object(&mut e, 3, 0x10);
        e.register_double(IS_KIND_OF, move |_, a| (a[1] == system).into_ret());
        e.register(CONTROLLER_COUNT, |_, _| 1u32.into_ret());
        e.register(CONTROLLER_AT, |_, _| 0x4400u32.into_ret());
        let emitter = e.mem.alloc(0x40);
        let spawner = e.mem.alloc(0x40);
        e.register_double(CONTROLLER_OF_KIND, move |_, a| {
            (if a[0] == EMITTER_KIND {
                emitter
            } else {
                spawner
            })
            .into_ret()
        });
        e.register(EMITTER_RATE, |_, _| st0(1.0));
        e.register(SPAWNER_RATE, |_, _| st0(1.0));
        e.register(SPAWNER_SIZE, |_, _| st0(1.0));
        e.register(SPAWNER_SPREAD, |_, _| st0(1.0));
        quiet(&mut e, &[SPAWNER_SET_RATE, SPAWNER_SET_SPREAD]);
        e.register(slot(3, 0xc), |_, _| 1u32.into_ret());
        e.register(CHILD_COUNT, |_, _| 1u32.into_ret());
        e.register_double(CHILD_AT, move |_, _| system.into_ret());
        e.call(0x0081_9890, &args![this, parent]);
        // Rate 2 * 0x90 = 288, spread 3 * 0xd4 = 636, size 3 * 0xf0 = 720.
        assert_eq!(e.mem.f32(emitter + 0x2c), 288.0);
        assert_eq!(
            calls(&e, SPAWNER_SET_RATE),
            vec![vec![spawner, 288.0f32.to_bits()]]
        );
        assert_eq!(e.mem.f32(spawner + 0x18), 720.0);
        assert_eq!(
            calls(&e, SPAWNER_SET_SPREAD),
            vec![vec![spawner, 636.0f32.to_bits()]]
        );
        // A leaf without children does nothing.
        e.register(slot(3, 0xc), |_, _| 0u32.into_ret());
        e.call(0x0081_9890, &args![this, parent]);
        assert_eq!(calls(&e, SPAWNER_SET_RATE).len(), 1);
    }

    #[test]
    fn init_3d_scales_the_area_effect_and_starts_the_idle() {
        let mut e = engine();
        let (this, node) = fog(&mut e);
        e.mem.set_f32(this + 0xb8, 4.0);
        e.register(ANGLES, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(slot(1, 0x9c), |_, _| 0u32.into_ret());
        e.register(CONTROLLER_MANAGER_SOURCE, |_, _| 0x7700u32.into_ret());
        e.register(CONTROLLER_OF_KIND, |_, _| 0x8800u32.into_ret());
        e.register(FIND_SEQUENCE_BY_NAME, |_, _| 0x8900u32.into_ret());
        e.mem.set_f32(FRAME_TIME, 0.6);
        quiet(
            &mut e,
            &[
                NODE_SET_SCALE,
                NODE_SET_TRANSLATE,
                SELF,
                MATRIX_FROM_ANGLES,
                NODE_SET_ROTATE,
                PROCESS_SELECTIVE_UPDATE,
                FIXED_STRING_DESTRUCT,
                DEACTIVATE_ALL,
                ACTIVATE_SEQUENCE,
                MANAGER_FLAG,
                SEQUENCE_SET_FLOAT,
                UPDATE_DATA_INIT,
                NODE_UPDATE,
            ],
        );
        e.call(0x0081_9a90, &args![this]);
        assert_eq!(
            calls(&e, NODE_SET_SCALE),
            vec![vec![node, 4.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, FIXED_STRING_CONSTRUCT)[0][1], TEXT_AREA_EFFECT);
        assert_eq!(
            calls(&e, SEQUENCE_SET_FLOAT),
            vec![vec![0x8900, (-f32::MAX).to_bits()]]
        );
        assert_eq!(calls(&e, NODE_UPDATE)[0][0], node);
    }

    #[test]
    fn constructor_initialises_the_fog() {
        let mut e = engine();
        let (this, _) = fog(&mut e);
        settings(&mut e);
        e.mem
            .set_u32(MAGIC_FOG_PROJECTILE_VTABLE + 0x1d0, slot(0, 0x1d0));
        e.register(slot(0, 0x1d0), |_, _| 0u32.into_ret());
        e.register(PROJECTILE_EFFECT_ITEM, |_, _| 0x2200u32.into_ret());
        e.register(EFFECT_ITEM_GET_DURATION, |_, _| 5u32.into_ret());
        e.register(EFFECT_ITEM_GET_AREA, |_, _| 7u32.into_ret());
        e.register(SCALED_VALUE, |_, a| {
            st0(f64::from(f32::from_bits(a[0])) * f64::from(f32::from_bits(a[1])))
        });
        e.register(PROJECTILE_SPEED, |_, _| st0(10.0));
        e.register(GET_CHAR_CONTROLLER, |_, _| 0x6600u32.into_ret());
        e.register(PARENT_CELL, |_, _| 0x5400u32.into_ret());
        e.register(BASE_FORM, |_, _| 0x4000u32.into_ret());
        e.register(PROJECTILE_FLIGHT_SOUND, |_, _| 0u32.into_ret());
        quiet(
            &mut e,
            &[
                MAGIC_PROJECTILE_CONSTRUCTOR,
                SOUND_HANDLE_CONSTRUCT,
                PROJECTILE_WITHOUT_LOCATION,
                VECTOR4_SET,
                SET_PROXY_VELOCITY,
                CELL_ADD_REFERENCE,
                MAGIC_LIGHT_TRANSFER,
                MAGIC_LIGHT_FADE,
                CHECK_SHOT_BEHIND_WALL,
                RUNNING_TOTAL_ADD,
            ],
        );
        let returned = e
            .call(
                0x0081_9360,
                &args![
                    this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 0u32,
                    0x4400u32
                ],
            )
            .u32();
        assert_eq!(returned, this);
        assert_eq!(e.mem.u32(this), MAGIC_FOG_PROJECTILE_VTABLE);
        // 5 * 0x74 = 580 and 7 * 0xc8 = 1400.
        assert_eq!(e.mem.f32(this + 0xac), 580.0);
        assert_eq!(e.mem.f32(this + 0xb0), 1400.0);
        assert_eq!(e.mem.f32(this + 0x88), 2480.0);
        assert_eq!(e.mem.f32(this + 0xb8), 1.0);
        assert_eq!(e.mem.u32(this + 0xcc), 0x4400);
        assert_eq!(
            calls(&e, MAGIC_LIGHT_FADE),
            vec![vec![0x4400, 0, 580.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, RUNNING_TOTAL_ADD),
            vec![vec![(0xf8u32 as f32).to_bits()]]
        );
        assert_eq!(calls(&e, SET_PROXY_VELOCITY)[0][0], 0x6600);
    }

    #[test]
    fn destructor_releases_everything() {
        let mut e = engine();
        let (this, _) = fog(&mut e);
        settings(&mut e);
        e.register(SOUND_HANDLE_IS_VALID, |_, _| 1u32.into_ret());
        quiet(
            &mut e,
            &[
                RUNNING_TOTAL_TAKE,
                DEALLOCATE,
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_RELEASE,
                MAGIC_LIGHT_DELETE,
                SOUND_HANDLE_DESTRUCT,
                MAGIC_PROJECTILE_DESTRUCTOR,
            ],
        );
        e.mem.set_u32(this + 0xcc, 0x4400);
        e.call(0x0081_96e0, &args![this]);
        assert_eq!(calls(&e, RUNNING_TOTAL_TAKE).len(), 1);
        assert_eq!(calls(&e, MAGIC_LIGHT_DELETE), vec![vec![0x4400, 1]]);
        assert_eq!(e.mem.u32(this + 0xcc), 0);
        assert_eq!(calls(&e, SOUND_HANDLE_STOP), vec![vec![this + 0xbc]]);
        assert_eq!(calls(&e, MAGIC_PROJECTILE_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn controller_setup_asks_the_reference_first() {
        let mut e = engine();
        let (this, _) = fog(&mut e);
        let process = object(&mut e, 4, 0x10);
        e.mem.set_u32(this + 0x68, process);
        e.register(OBJECT_BUSY, |_, _| 0u32.into_ret());
        e.register(GET_CHAR_CONTROLLER, |_, _| 0u32.into_ret());
        e.register(PARENT_CELL, |_, _| 0u32.into_ret());
        e.register(slot(0, 0x2e4), |_, _| st0(0.5));
        e.register(SAVE_LOAD_FLAG, |_, _| 1u32.into_ret());
        e.register(ALLOCATE_OBJECT, |_, _| 0u32.into_ret());
        e.register(ALLOCATE_CONTROLLER, |_, _| 0x6000u32.into_ret());
        e.register(CHARACTER_CONTROLLER_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(CONTROLLER_FLAG_SOURCE, |_, _| 0u32.into_ret());
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
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
                CONTROLLER_FLAG_100000,
                CONTROLLER_FLAG_80000,
                slot(4, 0x290),
                CONTROLLER_SET_OWNER,
                CONTROLLER_INFO_DESTRUCT,
                CONTROLLER_SET_FLOAT,
                CONTROLLER_SET_WORLD,
                CONTROLLER_SET_NODE,
                WORLD_SET_MOTION,
            ],
        );
        e.call(0x0081_9ca0, &args![this]);
        assert_eq!(calls(&e, INFO_SET_FLOAT_8C)[0][1], 0.5f32.to_bits());
        let log = e.call_log.as_ref().unwrap();
        let first = |addr: u32| log.iter().position(|(a, _)| *a == addr).unwrap();
        assert!(first(PROJECTILE_CASTER) < first(SAVE_LOAD_FLAG));
        assert!(first(CONTROLLER_FLAG_100000) < first(CONTROLLER_FLAG_80000));
        assert_eq!(calls(&e, slot(4, 0x290))[0][1], 0x6000);
        assert!(calls(&e, CONTROLLER_SET_POSITION).is_empty());
    }

    #[test]
    fn update_ages_and_pulses() {
        let mut e = engine();
        let (this, _) = fog(&mut e);
        settings(&mut e);
        e.mem.set_u32(this + 0xb4, 1);
        e.mem.set_f32(this + 0xac, 100.0);
        e.mem.set_f32(this + 0xa8, 60.0);
        e.register(SOUND_HANDLE_IS_VALID, |_, _| 1u32.into_ret());
        e.register(ROUND_DOWN, |_, a| {
            st0(f64::from(f32::from_bits(a[0])).floor())
        });
        e.register(PARENT_CELL, |_, _| 0x5400u32.into_ret());
        e.register(PROJECTILE_KIND, |_, _| 0x1du32.into_ret());
        quiet(
            &mut e,
            &[
                SOUND_HANDLE_SET_VOLUME,
                FIND_TARGETS_OV2,
                slot(0, 0xc4),
                MAGIC_LIGHT_UPDATE,
            ],
        );
        // Age 60 -> 70 crosses a multiple of the pulse (0xf4 = 244)? No.
        e.call(0x0081_a320, &args![this, 10.0f32]);
        assert_eq!(e.mem.f32(this + 0xa8), 70.0);
        // The volume: 1 - (70 - 50) / 50 = 0.6.
        assert_eq!(
            calls(&e, SOUND_HANDLE_SET_VOLUME),
            vec![vec![this + 0xbc, 0.6f32.to_bits()]]
        );
        assert!(calls(&e, FIND_TARGETS_OV2).is_empty());
        // A longer lifetime: 70 -> 250 crosses 244, the share is (1000 - 250) / 1000.
        e.mem.set_f32(this + 0xac, 1000.0);
        e.call(0x0081_a320, &args![this, 180.0f32]);
        let find = &calls(&e, FIND_TARGETS_OV2)[0];
        assert_eq!(find[..3], [0, 0x1d, 0x5400]);
        assert_eq!(find[9], 0.75f32.to_bits());
        // The 180 step ran slot +0xC4 (60 or more); past the lifetime it runs again.
        assert_eq!(calls(&e, slot(0, 0xc4)), vec![vec![this, 1]]);
        e.mem.set_f32(this + 0xac, 10.0);
        e.call(0x0081_a320, &args![this, 1.0f32]);
        assert_eq!(calls(&e, slot(0, 0xc4)), vec![vec![this, 1]; 2]);
        // Negative time: nothing.
        let age = e.mem.f32(this + 0xa8);
        e.call(0x0081_a320, &args![this, -1.0f32]);
        assert_eq!(e.mem.f32(this + 0xa8), age);
    }

    #[test]
    fn update_moves_while_flying() {
        let mut e = engine();
        let (this, _) = fog(&mut e);
        settings(&mut e);
        e.mem.set_f32(this + 0x88, 10.0);
        e.mem.set_f32(this + 0xac, 100.0);
        e.register(REFERENCE_FLAG, |_, _| 0u32.into_ret());
        e.register(GET_CHAR_CONTROLLER, |_, _| 0u32.into_ret());
        e.register(CONTROLLER_FLOAT, |_, _| st0(0.0));
        e.register(HEIGHT_OFFSET, |_, _| st0(0.0));
        e.register(slot(0, 0x30c), |_, _| 1u32.into_ret());
        e.register(POINT_DIFFERENCE, |_, a| a[1].into_ret());
        e.register(POINT_LENGTH, |_, _| st0(2.0));
        e.register(SOUND_HANDLE_IS_VALID, |_, _| 0u32.into_ret());
        e.register(ROUND_DOWN, |_, _| st0(0.0));
        quiet(
            &mut e,
            &[
                POINT_SET,
                CONTROLLER_SET_FLOAT_548,
                slot(0, 0x250),
                NODE_SET_TRANSLATE,
                slot(0, 0xc4),
                slot(0, 0x304),
            ],
        );
        e.call(0x0081_a320, &args![this, 0.5f32]);
        // Damping 1 - 0.5 * 0xd8 = -107.
        assert_eq!(
            calls(&e, CONTROLLER_SET_FLOAT_548),
            vec![vec![0, (-107.0f32).to_bits()]]
        );
        assert_eq!(e.mem.f32(this + 0x8c), 2.0);
        // Distance 2 beyond the limit (0): slot +0xC4; pace 2 / 0.5 = 4 <= 5: stopped.
        assert_eq!(calls(&e, slot(0, 0xc4)), vec![vec![this, 1]]);
        assert_eq!(
            calls(&e, slot(0, 0x304)),
            vec![vec![this, 0, 0, 0, 0, 0, 0]]
        );
    }

    #[test]
    fn save_and_load_the_records() {
        let mut e = engine();
        let (this, _) = fog(&mut e);
        e.register(SAVE_VERSION, |_, _| 0x71u32.into_ret());
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e.register(PROJECTILE_KIND, |_, _| 0x5500u32.into_ret());
        e.register(EFFECT_ITEM_INDEX, |_, _| 3u32.into_ret());
        let slot_word = e.mem.alloc(4);
        e.register_double(RESERVE_COUNT, move |_, _| slot_word.into_ret());
        quiet(
            &mut e,
            &[MAGIC_PROJECTILE_SAVE_GAME, SAVE_DATA, SAVE_WORD, SAVE_ID],
        );
        let record = e.mem.alloc(0xc);
        e.mem.set_u32(record, 0x7000);
        e.mem.set_u32(record + 4, 0x2200);
        e.mem.set_u32(this + 0xc8, record);
        e.call(0x0081_ade0, &args![this, 0x77u32]);
        assert_eq!(e.mem.u16(slot_word), 1);
        assert_eq!(calls(&e, SAVE_ID).len(), 1);
        assert_eq!(calls(&e, EFFECT_ITEM_INDEX), vec![vec![0x550c, 0x2200]]);
        let sizes: Vec<u32> = calls(&e, SAVE_DATA).iter().map(|c| c[2]).collect();
        assert_eq!(sizes, vec![4, 4, 4, 4, 4, 2, 1]);
        // The load: the records come back in front of the list.
        e.register(LOAD_DATA, |e, a| {
            if a[2] == 2 {
                e.mem.set_u16(a[1], 2);
            }
            Ret::default()
        });
        e.register(LOAD_ID, |e, a| {
            e.mem.set_u32(a[1], 0x7000);
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TARGET_RECORD_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 8, a[3]);
            a[0].into_ret()
        });
        quiet(
            &mut e,
            &[
                slot(0, 0x1c4),
                MAGIC_PROJECTILE_LOAD_GAME,
                LOAD_WORD,
                DEALLOCATE,
            ],
        );
        e.call(0x0081_afa0, &args![this, 0x77u32, 3u32]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![record]]);
        let first = e.mem.u32(this + 0xc8);
        let second = e.mem.u32(first + 8);
        assert_eq!(e.mem.u32(second + 8), 0);
        assert_eq!(calls(&e, TARGET_RECORD_CONSTRUCT).len(), 2);
    }

    #[test]
    fn init_load_game_resolves_the_records() {
        let mut e = engine();
        let (this, node) = fog(&mut e);
        e.mem.set_u32(this + 0xb4, 1);
        e.mem.set_f32(this + 0xcc, 0.25);
        e.mem.set_f32(this + 0xac, 9.0);
        e.register(ANGLES, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(slot(1, 0x9c), |_, _| 0u32.into_ret());
        e.register(CONTROLLER_MANAGER_SOURCE, |_, _| 0u32.into_ret());
        e.register(CONTROLLER_OF_KIND, |_, _| 0u32.into_ret());
        e.register(PARENT_CELL, |_, _| 0x5400u32.into_ret());
        e.register(COPY_MEMORY, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        e.register(BASE_FORM, |_, _| 0x4000u32.into_ret());
        e.register(PROJECTILE_LIGHT, |_, _| 0x4100u32.into_ret());
        e.register(FORM_ID, |_, _| 0x41u32.into_ret());
        e.register(ALLOCATE, |_, _| 0x4400u32.into_ret());
        e.register(MAGIC_LIGHT_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(SAVE_VERSION, |_, _| 0x71u32.into_ret());
        e.register(LOOKUP_FORM, |_, a| (a[0] + 1).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        e.register(PROJECTILE_KIND, |_, _| 0x5500u32.into_ret());
        e.register(EFFECT_ITEM_AT, |_, a| (a[1] + 0x2200).into_ret());
        quiet(
            &mut e,
            &[
                MAGIC_PROJECTILE_INIT_LOAD_GAME,
                NODE_SET_SCALE,
                NODE_SET_TRANSLATE,
                SELF,
                MATRIX_FROM_ANGLES,
                NODE_SET_ROTATE,
                PROCESS_SELECTIVE_UPDATE,
                FIXED_STRING_DESTRUCT,
                CELL_ADD_REFERENCE,
                MAGIC_LIGHT_FADE,
                MAGIC_LIGHT_SET_FLOAT,
            ],
        );
        let record = e.mem.alloc(0xc);
        e.mem.set_u32(record, 0x7000);
        e.mem.set_u8(record + 4, 2);
        e.mem.set_u32(this + 0xc8, record);
        e.call(0x0081_b180, &args![this, 0x77u32, 3u32]);
        assert_eq!(
            calls(&e, MAGIC_LIGHT_CONSTRUCTOR),
            vec![vec![0x4400, 0x41, node]]
        );
        assert_eq!(
            calls(&e, MAGIC_LIGHT_FADE),
            vec![vec![0x4400, 0, 9.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, MAGIC_LIGHT_SET_FLOAT),
            vec![vec![0x4400, 0.25f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(record), 0x7002);
        assert_eq!(e.mem.u32(record + 4), 0x2202);
        assert!(calls(&e, PROJECTILE_FLIGHT_SOUND).is_empty());
    }

    #[test]
    fn finish_init_load_game_sets_the_filter() {
        let mut e = engine();
        let (this, _) = fog(&mut e);
        settings(&mut e);
        let caster = object(&mut e, 5, 0x10);
        e.register_double(PROJECTILE_CASTER, move |_, _| caster.into_ret());
        e.register(slot(5, 0x2c), |_, _| 0x7000u32.into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(ACTOR_COLLISION_INFO, |_, a| a[1].into_ret());
        e.register(FILTER_LAYER, |_, _| 0x2bu32.into_ret());
        e.register(GET_CHAR_CONTROLLER, |_, _| 0x6600u32.into_ret());
        e.register(CONTROLLER_FILTER, |e, a| {
            e.mem.set_u32(a[1], 0x1200);
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                MOBILE_OBJECT_FINISH_INIT_LOAD_GAME,
                FILTER_SET_LAYER,
                FILTER_SET_GROUP,
                SET_COLLISION_FILTER,
                RUNNING_TOTAL_ADD,
            ],
        );
        e.call(0x0081_b430, &args![this, 0x77u32, 3u32]);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![0x7000, 0, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0]]
        );
        assert_eq!(calls(&e, FILTER_SET_GROUP)[0][1], 0x2b);
        assert_eq!(calls(&e, SET_COLLISION_FILTER), vec![vec![0x6600, 0x1200]]);
        assert_eq!(calls(&e, RUNNING_TOTAL_ADD).len(), 1);
    }
}

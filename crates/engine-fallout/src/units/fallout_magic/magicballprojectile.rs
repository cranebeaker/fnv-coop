//! `fallout/magic/magicballprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions here are on the player's movement path: they reach the Havok character
//! proxy of a `bhkCharacterController` (`0080fb00` is what `MoveAsProjectile` runs first).

#[allow(unused_imports)]
use crate::prelude::*;

/// The accessor `004ae750` forwards to for a non-null object (`00620b80`, `this` only; it
/// is a thin entry for `0044ddc0`, listed under `cameracaster.cpp` in the engine map).
const PROXY_OF_OBJECT: u32 = 0x0062_0b80;
/// `hkpCharacterProxy` linear velocity setter (`00cac960`, `this` = the proxy, one argument:
/// the address of a four-float vector).
const PROXY_SET_LINEAR_VELOCITY: u32 = 0x00ca_c960;

/// `004a3e00` (unnamed in the engine map): `cdecl`, two stack words (destination, source). Reads
/// three floats from the source, scales each by `004a3e90` and writes them to the destination's
/// first three slots (through `00560d30`), then writes `0.0` to the fourth.
const VECTOR_SCALE_COPY: u32 = 0x004a_3e00;
/// `00c6e050` (unnamed in the engine map, `bhkcharactercontroller.obj`): `thiscall`, `RET 8`.
const CHARACTER_CONTROLLER_CALL: u32 = 0x00c6_e050;
/// `00629670` (unnamed in the engine map): `thiscall`, `RET 8`; with a non-zero flag it ORs the
/// mask into the dword at `+4`, otherwise it hands the mask to `006296b0`.
const FLAG_SET_OR_CLEAR: u32 = 0x0062_9670;
/// `00559450` (unnamed in the engine map): `thiscall`, answers the dword at `this`.
const READ_FIRST_WORD: u32 = 0x0055_9450;
/// `bhkWorldObject::GetAVObject` (Xbox PDB, `00c85c80`, `RET 4`).
const WORLD_OBJECT_GET_AV_OBJECT: u32 = 0x00c8_5c80;

// Translated from 00810510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: hands `this` (as the destination) and the
/// pointer it was given (the source) to `004a3e00`, the scaled vector copy.
pub fn fn_00810510(e: &mut Engine, this: Ptr, source: Ptr) {
    e.call(VECTOR_SCALE_COPY, &args![this, source]);
}

// Translated from 00810550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: copies the float argument to `+0x8c` of `this`
/// with `FLD`/`FSTP` (a signalling NaN is stored quiet).
pub fn fn_00810550(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0x8c),
        x87_quiet_bits(value.to_bits()),
    );
}

// Translated from 00810610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: calls `00629670` on the object at `this + 0x410`
/// with the mask `0x100000` and the flag byte it was given (zero-extended).
pub fn fn_00810610(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(
        FLAG_SET_OR_CLEAR,
        &args![this.addr().wrapping_add(0x410), 0x0010_0000u32, flag],
    );
}

// Translated from 00810640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: calls `00c6e050` on `this` with `1000` and the
/// word it was given.
pub fn fn_00810640(e: &mut Engine, this: Ptr, value: u32) {
    e.call(CHARACTER_CONTROLLER_CALL, &args![this, 1000u32, value]);
}

// Translated from 00810660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `__fastcall`: the object at `*(this + 0x594)` (read through
/// `00559450`), or null when that is null; otherwise what `bhkWorldObject::GetAVObject`
/// (`00c85c80`) answers for it with a zero argument. The decompiler hangs the zero on the wrong
/// call; the disassembly pushes it for `GetAVObject`.
pub fn fn_00810660(e: &mut Engine, this: Ptr) -> u32 {
    let slot = this.addr().wrapping_add(0x594);
    if e.call(READ_FIRST_WORD, &args![slot]).u32() == 0 {
        return 0;
    }
    let object = e.call(READ_FIRST_WORD, &args![slot]).u32();
    e.call(WORLD_OBJECT_GET_AV_OBJECT, &args![object, 0u32])
        .u32()
}

// Translated from 00810530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores the byte argument at `+0x94` of
/// `this` (the class is unconfirmed; `00408e40` calls the neighbouring `00810570` on an
/// `EffectSetting`).
pub fn fn_00810530(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x94), value);
}

// Translated from 00810570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores the word argument at `+0x9c` of
/// `this` (`EffectSettingCollection` passes its sixth value through it).
pub fn fn_00810570(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x9c), value);
}

// Translated from 00810fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`__fastcall`): the word at `+0x98` of `this`.
pub fn fn_00810fd0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x98))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_0530, fn_00810530(Ptr, u8)),
        entry!(0x0081_0570, fn_00810570(Ptr, u32)),
        entry!(0x0081_0fd0, fn_00810fd0(Ptr) -> u32),
        entry!(0x0081_0510, fn_00810510(Ptr, Ptr)),
        entry!(0x0081_0550, fn_00810550(Ptr, f32)),
        entry!(0x0081_0610, fn_00810610(Ptr, u8)),
        entry!(0x0081_0640, fn_00810640(Ptr, u32)),
        entry!(0x0081_0660, fn_00810660(Ptr) -> u32),
        entry!(0x004ae750, bhk_character_proxy_operator_p(Ptr) -> u32),
        entry!(0x0080fb00, fn_0080fb00(Ptr, Ptr)),
        entry!(
            0x0080_f810,
            magic_ball_projectile_constructor(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(0x0080_fae0, fn_0080fae0(f32)),
        entry!(0x0080_fb30, fn_0080fb30(Ptr)),
        entry!(0x0080_fc90, fn_0080fc90(f32)),
        entry!(0x0080_fcc0, fn_0080fcc0(Ptr, u32) -> Ptr),
        entry!(0x0080_fcf0, magic_ball_projectile_init_3d(Ptr)),
        entry!(0x0080_feb0, fn_0080feb0(Ptr)),
        entry!(0x0081_0590, fn_00810590(Ptr, u32, u32)),
        entry!(0x0081_05c0, fn_008105c0(Ptr, u32)),
        entry!(0x0081_05e0, fn_008105e0(Ptr, u8)),
        entry!(0x0081_06b0, fn_008106b0(Ptr, f32)),
        entry!(0x0081_06d0, fn_008106d0(Ptr, f32)),
        entry!(0x0081_0a80, fn_00810a80(Ptr) -> f64),
        entry!(0x0081_0aa0, fn_00810aa0(Ptr, u32, u32, u32, u32, u32, u8)),
        entry!(0x0081_0f90, fn_00810f90(Ptr, u8)),
        entry!(
            0x0081_0ff0,
            bhk_character_controller_set_collision_filter(Ptr, u32)
        ),
        entry!(0x0081_1030, fn_00811030(Ptr) -> bool),
        entry!(0x0081_1070, fn_00811070(Ptr, u32) -> u16),
        entry!(0x0081_1110, fn_00811110(Ptr, u32)),
        entry!(0x0081_11e0, fn_008111e0(Ptr, u32, u32)),
        entry!(
            0x0081_12c0,
            magic_ball_projectile_init_load_game(Ptr, u32, u32)
        ),
        entry!(
            0x0081_15c0,
            magic_ball_projectile_finish_init_load_game(Ptr, u32, u32)
        ),
    ]
}

// Translated from 004ae750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::operatorP` (name from the engine map): null when `this` is null,
/// otherwise what `00620b80` answers for it.
pub fn bhk_character_proxy_operator_p(e: &mut Engine, this: Ptr) -> u32 {
    if this.is_null() {
        0
    } else {
        e.call(PROXY_OF_OBJECT, &args![this]).u32()
    }
}

// Translated from 0080fb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Looks up the proxy of `this` (`004ae750`) and, when there is
/// one, runs the proxy's linear velocity setter (`00cac960`) with `vector`.
pub fn fn_0080fb00(e: &mut Engine, this: Ptr, vector: Ptr) {
    let proxy = bhk_character_proxy_operator_p(e, this);
    if proxy != 0 {
        e.call(PROXY_SET_LINEAR_VELOCITY, &args![proxy, vector]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.register(PROXY_OF_OBJECT, |_, a| (a[0] + 0x100).into_ret());
        e.register(PROXY_SET_LINEAR_VELOCITY, |_, _| Ret::default());
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn proxy_operator_is_null_for_null_and_forwards_otherwise() {
        let mut e = rig();
        assert_eq!(e.call(0x004a_e750, &args![0u32]).u32(), 0);
        assert!(calls(&e, PROXY_OF_OBJECT).is_empty());
        assert_eq!(e.call(0x004a_e750, &args![0x4000u32]).u32(), 0x4100);
        assert_eq!(calls(&e, PROXY_OF_OBJECT), vec![vec![0x4000]]);
    }

    #[test]
    fn set_velocity_runs_only_when_a_proxy_exists() {
        let mut e = rig();
        e.call(0x0080_fb00, &args![0u32, 0x9000u32]);
        assert!(calls(&e, PROXY_SET_LINEAR_VELOCITY).is_empty());
        e.call(0x0080_fb00, &args![0x4000u32, 0x9000u32]);
        assert_eq!(
            calls(&e, PROXY_SET_LINEAR_VELOCITY),
            vec![vec![0x4100, 0x9000]]
        );
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
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

    #[test]
    fn at_00810510_passes_this_as_destination_and_the_argument_as_source() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x004a_3e00, |_, _| Ret::default());
        e.call(0x0081_0510, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls(&e, 0x004a_3e00), vec![vec![0x1000, 0x2000]]);
    }

    #[test]
    fn at_00810550_stores_the_float_quiet_at_offset_0x8c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        e.call(0x0081_0550, &args![this, 2.5f32]);
        assert_eq!(e.mem.u32(this + 0x8c), 2.5f32.to_bits());
        // A signalling NaN comes out quiet (FLD/FSTP).
        e.call(0x0081_0550, &args![this, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(this + 0x8c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x88), 0);
        assert_eq!(e.mem.u32(this + 0x90), 0);
    }

    #[test]
    fn at_00810610_flags_the_object_at_offset_0x410_with_the_mask() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x0062_9670, |_, _| Ret::default());
        e.call(0x0081_0610, &args![0x4000u32, 1u8]);
        e.call(0x0081_0610, &args![0x4000u32, 0u8]);
        assert_eq!(
            calls(&e, 0x0062_9670),
            vec![vec![0x4410, 0x0010_0000, 1], vec![0x4410, 0x0010_0000, 0]]
        );
    }

    #[test]
    fn at_00810640_calls_the_controller_with_1000_and_the_word() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c6_e050, |_, _| Ret::default());
        e.call(0x0081_0640, &args![0x4000u32, 0xfeed_beefu32]);
        assert_eq!(
            calls(&e, 0x00c6_e050),
            vec![vec![0x4000, 1000, 0xfeed_beef]]
        );
    }

    #[test]
    fn at_00810660_answers_null_without_an_object_and_the_av_object_with_one() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x600);
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x00c8_5c80, |_, a| (a[0] + 0x10).into_ret());
        assert_eq!(e.call(0x0081_0660, &args![this]).u32(), 0);
        assert!(calls(&e, 0x00c8_5c80).is_empty());
        e.mem.set_u32(this + 0x594, 0x7000);
        assert_eq!(e.call(0x0081_0660, &args![this]).u32(), 0x7010);
        assert_eq!(calls(&e, 0x00c8_5c80), vec![vec![0x7000, 0]]);
        assert_eq!(calls(&e, 0x0055_9450).last(), Some(&vec![this + 0x594]));
    }
}

#[cfg(test)]
mod field_accessor_tests {
    use super::*;

    #[test]
    fn byte_at_0x94_word_at_0x9c_and_word_at_0x98() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this + 0x94, 0x1111_1111);
        e.call(0x0081_0530, &args![this, 0x1ffu32]);
        // Only the low byte of the word is stored.
        assert_eq!(e.mem.u32(this + 0x94), 0x1111_11ff);
        e.call(0x0081_0570, &args![this, 0xdead_beefu32]);
        assert_eq!(e.mem.u32(this + 0x9c), 0xdead_beef);
        e.mem.set_u32(this + 0x98, 0x2468);
        assert_eq!(e.call(0x0081_0fd0, &args![this]).u32(), 0x2468);
    }
}

// ---------------------------------------------------------------------------
// MagicBallProjectile (Xbox PDB): the ball projectile's own functions.
// ---------------------------------------------------------------------------

/// `MagicBallProjectile`'s two vtables (`0107a554` at +0, `0107a548` at +0x18).
const MAGIC_BALL_PROJECTILE_VTABLE: u32 = 0x0107_a554;
const MAGIC_BALL_PROJECTILE_SECOND_VTABLE: u32 = 0x0107_a548;
/// `MagicProjectile::MagicProjectile` (engine map, `0081dd00`, `thiscall`, `RET 0x28`).
const MAGIC_PROJECTILE_CONSTRUCTOR: u32 = 0x0081_dd00;
/// `MagicProjectile::~MagicProjectile` (engine map, `0081de70`, `thiscall`).
const MAGIC_PROJECTILE_DESTRUCTOR: u32 = 0x0081_de70;
/// `MagicProjectile::CheckShotBehindWall` (engine map, `0081eba0`, `thiscall`).
const CHECK_SHOT_BEHIND_WALL: u32 = 0x0081_eba0;
/// `MagicProjectile::SaveGame` (engine map, `0081e8c0`, `RET 4`), the load body `0081e980`
/// (`RET 8`), `MagicProjectile::InitLoadGame` (`0081ea40`, `RET 8`) and the save size body
/// `0081e830` (`RET 4`, a 16-bit size).
const MAGIC_PROJECTILE_SAVE_GAME: u32 = 0x0081_e8c0;
const MAGIC_PROJECTILE_LOAD_GAME: u32 = 0x0081_e980;
const MAGIC_PROJECTILE_INIT_LOAD_GAME: u32 = 0x0081_ea40;
const MAGIC_PROJECTILE_SAVE_SIZE: u32 = 0x0081_e830;
/// `MobileObject::FinishInitLoadGame` (engine map, `00932520`, `RET 8`).
const MOBILE_OBJECT_FINISH_INIT_LOAD_GAME: u32 = 0x0093_2520;
/// `0081e570` (`thiscall`, `RET 4`) and `00570f70` (`thiscall`, `RET 4`): the constructor's
/// two alternatives for its eleventh word (zero or not).
const PROJECTILE_WITHOUT_LOCATION: u32 = 0x0081_e570;
const PROJECTILE_WITH_LOCATION: u32 = 0x0057_0f70;
/// The sound handle at +0xB4 (`BSSoundHandle`): its constructor `0041a250`, `IsValid`
/// (`00ad8ce0`), `Stop` (`00ad88f0`), `Release` (`00ad8d10`), assignment `00418900` (`RET 4`),
/// destructor `00483710`, `SetPosition` (`00ad8b60`, three `float`s) and `Play` (`00ad8830`,
/// one word).
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
const SOUND_HANDLE_SET_POSITION: u32 = 0x00ad_8b60;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// `BSAudio::QInstance` (`00453a70`) and `BSAudio::GetSoundHandleByNumericID` (`00ad73b0`,
/// `RET 0xC`: the handle to fill, the form id, flags).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_ID: u32 = 0x00ad_73b0;
/// `00933150` (`thiscall`, `RET 0x14`): makes a sound handle for the reference (the handle
/// to fill, the form id, 1, flags, 1); answers the handle.
const MAKE_SOUND_HANDLE: u32 = 0x0093_3150;
/// `004fd400` (`thiscall`): the reference's base form; `004fd3c0`, `004fd380` and
/// `008ce390` (`thiscall`): three form pointers of the projectile form (the sound it plays
/// while flying, its light, the sound of its impact; the meanings are not confirmed).
const BASE_FORM: u32 = 0x004f_d400;
const PROJECTILE_FLIGHT_SOUND: u32 = 0x004f_d3c0;
const PROJECTILE_LIGHT: u32 = 0x004f_d380;
const PROJECTILE_IMPACT_SOUND: u32 = 0x008c_e390;
/// `0084e3a0` (`thiscall`): the word at +0xC (a form's id).
const FORM_ID: u32 = 0x0084_e3a0;
/// `009b0bd0` (`thiscall`, `ECX` = the projectile form): a `float` in `ST0` (its speed).
const PROJECTILE_SPEED: u32 = 0x009b_0bd0;
/// `00403e20` (`thiscall`): the address of a `float` setting's value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The `float` setting scaling the speed (`011d16f8`), the one added to and taken from the
/// running total at `011dc778` (`011d1c28`), the distance limit (`011d1d00`) and the area
/// scale settings `fMagicAreaScale`, `fMagicAreaScaleMax`, `fMagicAreaScaleMin` (names as
/// `magiccaster.rs` gives them).
const SPEED_SETTING: u32 = 0x011d_16f8;
const COUNT_SETTING: u32 = 0x011d_1c28;
const DISTANCE_SETTING: u32 = 0x011d_1d00;
const AREA_SCALE: u32 = 0x011d_1bbc;
const AREA_SCALE_MAX: u32 = 0x011d_1acc;
const AREA_SCALE_MIN: u32 = 0x011d_15a4;
/// A running `float` total (`011dc778`) that the ball projectiles of casters other than the
/// player add to and take from.
const RUNNING_TOTAL: u32 = 0x011d_c778;
/// `00404010` (`cdecl`, two `float`s): a `float` in `ST0` (taken from the running total).
const TOTAL_PART: u32 = 0x0040_4010;
/// The player (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `009185e0` (`thiscall`): the projectile's magic caster; its slot +0x2C answers the
/// caster's reference.
const PROJECTILE_CASTER: u32 = 0x0091_85e0;
/// `MobileObject::GetCharController` (engine map, `009306d0`).
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `008d6f30` (`thiscall`): the reference's cell; `00548230` `TESObjectCELL::AddReference`
/// (`RET 8`); `004543c0` (`thiscall`): the cell's Havok world; `005471e0`
/// `TESObjectCELL::GetWaterHeight` (`ST0`).
const PARENT_CELL: u32 = 0x008d_6f30;
const CELL_ADD_REFERENCE: u32 = 0x0054_8230;
const CELL_HAVOK_WORLD: u32 = 0x0045_43c0;
const CELL_WATER_HEIGHT: u32 = 0x0054_71e0;
/// `0045cd60` (`thiscall`): a word of the object at +0x68; the controller is set up only
/// while it is zero.
const OBJECT_BUSY: u32 = 0x0045_cd60;
/// `00484b60` (`thiscall`, `RET 4`) and `TESForm::SetDisabled` (`00484af0`, `RET 4`).
const FORM_SET_FLAG: u32 = 0x0048_4b60;
const FORM_SET_DISABLED: u32 = 0x0048_4af0;
/// `TESForm::SaveGameDataOLD` (`00484ce0`) / `LoadGameDataOLD` (`00484d00`): `RET 8`, the
/// address and the size of a field.
const SAVE_DATA: u32 = 0x0048_4ce0;
const LOAD_DATA: u32 = 0x0048_4d00;
/// The save/load manager (`011de45c`), its version byte (`008df040`), its word save
/// (`008579b0`, `RET 8`) and load (`008579e0`, `RET 8`), and a `bool` (`0047c850`).
const SAVE_LOAD_MANAGER: u32 = 0x011d_e45c;
const SAVE_VERSION: u32 = 0x008d_f040;
const SAVE_WORD: u32 = 0x0085_79b0;
const LOAD_WORD: u32 = 0x0085_79e0;
const SAVE_LOAD_FLAG: u32 = 0x0047_c850;
/// `00401460` (`cdecl`, three words: destination, source, size): copies memory.
const COPY_MEMORY: u32 = 0x0040_1460;
/// `00440d80` (`thiscall`): a `bool` of the reference that stops the update.
const REFERENCE_FLAG: u32 = 0x0044_0d80;
/// The node setters `00440490` (scale, a `float`), `00440460` (translation, a point's
/// address) and `0043fa80` (rotation, a matrix's address) (`RET 4` each).
const NODE_SET_SCALE: u32 = 0x0044_0490;
const NODE_SET_TRANSLATE: u32 = 0x0044_0460;
const NODE_SET_ROTATE: u32 = 0x0043_fa80;
/// `00430830` (`thiscall`): the address of the reference's angles (three `float`s).
const ANGLES: u32 = 0x0043_0830;
/// `006815c0` (`thiscall`): answers `this` (run on the local matrix before it is filled).
const MATRIX_SELF: u32 = 0x0068_15c0;
/// The matrix from Euler angles (`00a59660`, `RET 0xC`).
const MATRIX_FROM_ANGLES: u32 = 0x00a5_9660;
/// `BSUtilities::ProcessSelectiveUpdate` (`00c4b500`, `cdecl`, one word).
const PROCESS_SELECTIVE_UPDATE: u32 = 0x00c4_b500;
/// `0043b230` (`thiscall`): the node's controller source; `00653270` (`cdecl`: the global at
/// `011f36ac`, that source): the node's `NiControllerManager`, or null.
const NODE_CONTROLLER_SOURCE: u32 = 0x0043_b230;
const CONTROLLER_MANAGER_OF: u32 = 0x0065_3270;
const CONTROLLER_MANAGERS: u32 = 0x011f_36ac;
/// `00438170` (`thiscall`, `RET 4`) constructs a fixed string from text, `004381b0` destroys
/// it; `0047a520` (`RET 4`) finds the manager's sequence by that name.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const FIND_SEQUENCE: u32 = 0x0047_a520;
/// `NiControllerManager::DeactivateAll` (`0048fef0`, a `float`), the sequence activation
/// `0047aab0` (`RET 0x18`), `0047aa40` (`RET 4`), the sequence's `0098adb0` (`RET 4`, a
/// `float`), its two `float`s `00508120` and `00508100` (`ST0`) and `008041a0` (a word).
const DEACTIVATE_ALL: u32 = 0x0048_fef0;
const ACTIVATE_SEQUENCE: u32 = 0x0047_aab0;
const MANAGER_FLAG: u32 = 0x0047_aa40;
const SEQUENCE_SET_FLOAT: u32 = 0x0098_adb0;
const SEQUENCE_FIRST_FLOAT: u32 = 0x0050_8120;
const SEQUENCE_SECOND_FLOAT: u32 = 0x0050_8100;
const SEQUENCE_WORD: u32 = 0x0080_41a0;
/// `"SpecialIdle_Projectile"`, `"SpecialIdle_AreaEffect"` and `"AreaEffect"`.
const TEXT_SPECIAL_IDLE_PROJECTILE: u32 = 0x0107_a874;
const TEXT_SPECIAL_IDLE_AREA_EFFECT: u32 = 0x0102_054c;
const TEXT_AREA_EFFECT: u32 = 0x0107_a88c;
/// `3.4028235e38` (`float`, `0107a510`; negated before use), the frame time `float`
/// (`011c3c08`), `0.1` (`double`, `0101ffa0`), `0.75` (`double`, `0101de30`), `60.0`
/// (`double`, `01012638`), `0.0` (`double`, `01012060`), `8.0` (`double`, `010240b0`) and
/// `-3.4028235e38` (`float`, `01015f5c`).
const FLOAT_MAX: u32 = 0x0107_a510;
const FRAME_TIME: u32 = 0x011c_3c08;
const TENTH: u32 = 0x0101_ffa0;
const THREE_QUARTERS: u32 = 0x0101_de30;
const SIXTY: u32 = 0x0101_2638;
const ZERO: u32 = 0x0101_2060;
const EIGHT: u32 = 0x0102_40b0;
const LOWEST_FLOAT: u32 = 0x0101_5f5c;
/// `0043d410` (`thiscall`, `RET 0xC`) fills an update record (a `float`, 1, 0) that
/// `00a59c60` (`RET 4`) updates the node with.
const UPDATE_DATA_INIT: u32 = 0x0043_d410;
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `MagicLight` functions: the destructor body `0081c240`, `MagicLight::MagicLight`
/// (`0081bf80`, `RET 8`), the update `0081c320` (`RET 4`), `MagicLight::Fade` (`0081c470`,
/// `RET 8`), `MagicLight::Transfer` (`0081c4a0`, `RET 4`), `00621b00` (`ST0`) and `00621b20`
/// (`RET 4`, a `float`).
const MAGIC_LIGHT_DESTRUCTOR: u32 = 0x0081_c240;
const MAGIC_LIGHT_CONSTRUCTOR: u32 = 0x0081_bf80;
const MAGIC_LIGHT_UPDATE: u32 = 0x0081_c320;
const MAGIC_LIGHT_FADE: u32 = 0x0081_c470;
const MAGIC_LIGHT_TRANSFER: u32 = 0x0081_c4a0;
const MAGIC_LIGHT_GET_FLOAT: u32 = 0x0062_1b00;
const MAGIC_LIGHT_SET_FLOAT: u32 = 0x0062_1b20;
/// `00401000` / `00401030`: `operator new` / `operator delete` (`cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
const DEALLOCATE: u32 = 0x0040_1030;
/// The character controller's construction: `00c6da50` (`RET 4`) builds the construction
/// info (0xC8 bytes), `00c6de10` destroys it; its setters `008ac890` (`RET 4`), `00857230`
/// (`RET 4`), `00407860` (`RET 4`, a `float`); the sphere shape (`00aa13e0` 0x14 bytes,
/// `bhkSphereShape::bhkSphereShape` `0056e950`, `RET 8`); the controller (`00553300` 0x660
/// bytes, `00806eb0`, `RET 8`).
const CONTROLLER_INFO_CONSTRUCT: u32 = 0x00c6_da50;
const CONTROLLER_INFO_DESTRUCT: u32 = 0x00c6_de10;
const CONTROLLER_INFO_SET_NODE: u32 = 0x008a_c890;
const CONTROLLER_INFO_SET_WORLD: u32 = 0x0085_7230;
const CONTROLLER_INFO_SET_FLOAT: u32 = 0x0040_7860;
const ALLOCATE_OBJECT: u32 = 0x00aa_13e0;
const SPHERE_SHAPE_CONSTRUCTOR: u32 = 0x0056_e950;
const ALLOCATE_CONTROLLER: u32 = 0x0055_3300;
const CHARACTER_CONTROLLER_CONSTRUCTOR: u32 = 0x0080_6eb0;
/// The collision layer sources: `00931ed0` (`RET 4`) of an actor, `004b5260`
/// (`FindFirstCollisionObject`, `cdecl`) of a node then `006fa820` and `0043b4f0` (`RET 4`),
/// `004a3a20` (the layer word) and `005dbe10` (`cdecl`, the default).
const ACTOR_COLLISION_INFO: u32 = 0x0093_1ed0;
const FIND_FIRST_COLLISION_OBJECT: u32 = 0x004b_5260;
const COLLISION_OBJECT_WORLD_OBJECT: u32 = 0x006f_a820;
const WORLD_OBJECT_FILTER: u32 = 0x0043_b4f0;
const FILTER_LAYER: u32 = 0x004a_3a20;
const DEFAULT_LAYER: u32 = 0x005d_be10;
/// `00525420` (`cdecl`): a `bool`.
const CONTROLLER_FLAG_SOURCE: u32 = 0x0052_5420;
/// `00633c90` (`RET 4`): an `NiPointer` constructed from a pointer.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// The controller's `0057b1d0` (`RET 4`, a `float`), `00621ad0` (its world),
/// `bhkCharacterController::SetWorld` (`00c70c90`, `RET 4`), `SetNiObject` (`00c6d7a0`,
/// `RET 4`), `SetPosition` (`005620e0`, `RET 4`), `0070c440` (`RET 4`, fills a word).
const CONTROLLER_SET_FLOAT: u32 = 0x0057_b1d0;
const CONTROLLER_WORLD: u32 = 0x0062_1ad0;
const CONTROLLER_SET_WORLD: u32 = 0x00c7_0c90;
const CONTROLLER_SET_NODE: u32 = 0x00c6_d7a0;
const CONTROLLER_SET_POSITION: u32 = 0x0056_20e0;
const CONTROLLER_FILTER: u32 = 0x0070_c440;
/// `004bae00` (`thiscall`): a `bool` of the world; `BSShaderUtil::
/// RecursiveSetPropertyWireframe` (`00b6b910`, `cdecl`, two words); `bhkWorld::SetMotion`
/// (`00c6a350`, `cdecl`, five words); `bhkWorld::RemoveObjects` (`00c69ee0`, `cdecl`,
/// three); `bhkWorld::UpdatePosition` (`00c69f50`, `cdecl`, three).
const WORLD_FLAG: u32 = 0x004b_ae00;
const SET_WIREFRAME: u32 = 0x00b6_b910;
const WORLD_SET_MOTION: u32 = 0x00c6_a350;
const WORLD_REMOVE_OBJECTS: u32 = 0x00c6_9ee0;
const WORLD_UPDATE_POSITION: u32 = 0x00c6_9f50;
/// `006222c0` (`thiscall`, `RET 0xC`) on the object at +0x84 of the construction info;
/// `0066b0d0` (`RET 4`) assigns an `NiPointer`.
const INFO_FILTER_SET: u32 = 0x0062_22c0;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `00626cb0` (`RET 4`): sets the collision filter word on the controller's object.
const OBJECT_SET_FILTER: u32 = 0x0062_6cb0;
/// `004a39f0` (`RET 4`) and `0059ce80` (`RET 4`): set the layer and the group of a filter
/// word.
const FILTER_SET_LAYER: u32 = 0x004a_39f0;
const FILTER_SET_GROUP: u32 = 0x0059_ce80;
/// `00621270` (`RET 4`): tests a flag mask of the object at +0x410 of the controller.
const CONTROLLER_HAS_FLAG: u32 = 0x0062_1270;
/// `004587d0` (`cdecl`, a `float`): a `float` in `ST0`.
const HEIGHT_OFFSET: u32 = 0x0045_87d0;
/// The point helpers `00416870` (`RET 0xC`), `00439ef0` (`RET 8`: a difference into an out
/// point) and `00457990` (the length, `ST0`).
const POINT_SET: u32 = 0x0041_6870;
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const POINT_LENGTH: u32 = 0x0045_7990;
/// `005532a0` (`thiscall`, `RET 0x10`): sets a four-float vector.
const VECTOR4_SET: u32 = 0x0055_32a0;
/// `008ace90` (`thiscall`): a `bool` of the actor that stops the explosion.
const ACTOR_STOPS_EXPLOSION: u32 = 0x008a_ce90;
/// `00586150` (`thiscall`): the projectile's effect item; `EffectItem::GetArea`
/// (`00404420`, an `int`).
const PROJECTILE_EFFECT_ITEM: u32 = 0x0058_6150;
const EFFECT_ITEM_GET_AREA: u32 = 0x0040_4420;
/// `TESObjectREFR::SetLocationOnReference` (`00575830`, `RET 4`, a point's address).
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
/// `00461130` (the node's transform at +0x68), `004b4500` (`RET 8`: the transform applied
/// to a point into an out point), the point at `011a9478`, and `0081f000` (`thiscall`,
/// `RET 0x1C`: two points by value and a word).
const NODE_TRANSFORM: u32 = 0x0046_1130;
const TRANSFORM_APPLY: u32 = 0x004b_4500;
const EXPLOSION_OFFSET: u32 = 0x011a_9478;
const PROJECTILE_EXPLODE_AT: u32 = 0x0081_f000;
/// `MagicCaster::FindTargets_ov2` (engine map, `00817160`, `RET 0x28`).
const FIND_TARGETS_OV2: u32 = 0x0081_7160;

/// The projectile caster's reference: null without a caster, otherwise its slot +0x2C.
pub(crate) fn caster_reference(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(PROJECTILE_CASTER, &args![this]).u32() == 0 {
        return 0;
    }
    let caster = e.call(PROJECTILE_CASTER, &args![this]).u32();
    e.vcall(caster, 0x2c, &[]).u32()
}

/// The `float` value of a setting, as its bits (`FLD`, quiet).
pub(crate) fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(value))
}

pub(crate) fn node(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr(), 0x1d0, &[]).u32()
}

/// Copies the three words of the point the reference's slot +0x1F4 answers.
pub(crate) fn position_words(e: &mut Engine, this: Ptr) -> [u32; 3] {
    let point = e.vcall(this.addr(), 0x1f4, &[]).u32();
    [
        e.mem.u32(point),
        e.mem.u32(point.wrapping_add(4)),
        e.mem.u32(point.wrapping_add(8)),
    ]
}

/// The node's `NiControllerManager` (`00653270` on `0043b230` of the node, or of null).
pub(crate) fn controller_manager(e: &mut Engine, node: u32) -> u32 {
    let source = if node != 0 {
        e.call(NODE_CONTROLLER_SOURCE, &args![node]).u32()
    } else {
        0
    };
    e.call(CONTROLLER_MANAGER_OF, &args![CONTROLLER_MANAGERS, source])
        .u32()
}

/// The manager's sequence named by the text (through a local fixed string).
pub(crate) fn find_sequence(e: &mut Engine, manager: u32, text: u32) -> u32 {
    e.with_stack(4, |e, name| {
        let name_pointer = e.call(FIXED_STRING_CONSTRUCT, &args![name, text]).u32();
        let sequence = e.call(FIND_SEQUENCE, &args![manager, name_pointer]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![name]);
        sequence
    })
}

/// Deactivates the manager's sequences and starts the sequence
/// (`0047aab0(sequence, 0, 0, 1.0, 0.0, 0)`, `0047aa40(1)`, the sequence's `0098adb0` with
/// `-3.4028235e38`).
pub(crate) fn start_sequence(e: &mut Engine, manager: u32, sequence: u32) {
    e.call(DEACTIVATE_ALL, &args![manager, 0.0f32]);
    e.call(
        ACTIVATE_SEQUENCE,
        &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
    );
    e.call(MANAGER_FLAG, &args![manager, 1u32]);
    let lowest = x87_store32(-x87_load32(e.mem.u32(FLOAT_MAX)));
    e.call(SEQUENCE_SET_FLOAT, &args![sequence, lowest]);
}

/// The collision layer word of a node: `004b5260`, `006fa820` and `0043b4f0` then
/// `004a3a20`, or `005dbe10` without a world object.
pub(crate) fn node_layer(e: &mut Engine, node: u32) -> u32 {
    let collision = e.call(FIND_FIRST_COLLISION_OBJECT, &args![node]).u32();
    let object = if collision != 0 {
        e.call(COLLISION_OBJECT_WORLD_OBJECT, &args![collision])
            .u32()
    } else {
        0
    };
    if object != 0 {
        e.with_stack(4, |e, out| {
            let filter = e.call(WORLD_OBJECT_FILTER, &args![object, out]).u32();
            e.call(FILTER_LAYER, &args![filter]).u32()
        })
    } else {
        e.call(DEFAULT_LAYER, &[]).u32()
    }
}

/// The collision layer word of an actor (`00931ed0`, `004a3a20`).
pub(crate) fn actor_layer(e: &mut Engine, actor: u32) -> u32 {
    e.with_stack(4, |e, out| {
        let info = e.call(ACTOR_COLLISION_INFO, &args![actor, out]).u32();
        e.call(FILTER_LAYER, &args![info]).u32()
    })
}

/// Makes the flight sound handle again (when the projectile form has a flight sound):
/// releases a valid one, then assigns a new one from `00933150`.
fn restart_flight_sound(e: &mut Engine, this: Ptr) {
    let form = e.call(BASE_FORM, &args![this]).u32();
    if e.call(PROJECTILE_FLIGHT_SOUND, &args![form]).u32() == 0 {
        return;
    }
    let form = e.call(BASE_FORM, &args![this]).u32();
    let sound = e.call(PROJECTILE_FLIGHT_SOUND, &args![form]).u32();
    let id = e.call(FORM_ID, &args![sound]).u32();
    let handle = this.addr().wrapping_add(0xb4);
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

/// `0081c320` on the light (+0xC0) with the time, when there is one.
fn update_light(e: &mut Engine, this: Ptr, time: f32) {
    let light = e.mem.u32(this.addr().wrapping_add(0xc0));
    if light != 0 {
        e.call(
            MAGIC_LIGHT_UPDATE,
            &args![light, x87_quiet_bits(time.to_bits())],
        );
    }
}

// Translated from 0080f810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080f810 - the FS:[0] exception frame and the /GS cookie check (00ec408c) are not translated
/// `MagicBallProjectile::MagicBallProjectile` (Xbox PDB; `thiscall`, `RET 0x30`): the
/// `MagicProjectile` constructor with the first four words and the two points (passed on
/// unchanged), the two vtables, the sound handle (+0xB4), +0x88 = the projectile form's
/// speed times the setting at `011d16f8`, +0xAC = 0, +0xA8 = `0.0`, +0xB0 = `1.0`, +0xC0 =
/// null; then `00570f70(location)` (or `0081e570(0)` without one),
/// [`magic_ball_projectile_init_3d`], [`fn_0080feb0`], the controller's velocity set to
/// `(-3.4028235e38, 0, 0, 0)` ([`fn_0080fb00`]), the cell's `AddReference`, the flight
/// sound, the light handed over (`MagicLight::Transfer`, kept at +0xC0),
/// `CheckShotBehindWall`, [`fn_0080fae0`] with the setting at `011d1c28` for casters other
/// than the player, and `00484b60(0x10000000)`. Returns `this`.
#[allow(clippy::too_many_arguments)]
pub fn magic_ball_projectile_constructor(
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
    e.call(
        MAGIC_PROJECTILE_CONSTRUCTOR,
        &args![
            this, word_1, word_2, word_3, word_4, first_x, first_y, first_z, second_x, second_y,
            second_z
        ],
    );
    let base = this.addr();
    e.mem.set_u32(base, MAGIC_BALL_PROJECTILE_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x18), MAGIC_BALL_PROJECTILE_SECOND_VTABLE);
    e.call(SOUND_HANDLE_CONSTRUCT, &args![base.wrapping_add(0xb4)]);
    let form = e.call(BASE_FORM, &args![this]).u32();
    let speed = e.call(PROJECTILE_SPEED, &args![form]).f64();
    let scale = e.call(SETTING_VALUE, &args![SPEED_SETTING]).u32();
    let value = x87_store32(x87_mul(x87_load32(e.mem.u32(scale)), speed));
    e.mem.set_u32(base.wrapping_add(0x88), value);
    e.mem.set_u32(base.wrapping_add(0xac), 0);
    e.mem.set_u32(base.wrapping_add(0xa8), 0.0f32.to_bits());
    e.mem.set_u32(base.wrapping_add(0xb0), 1.0f32.to_bits());
    e.mem.set_u32(base.wrapping_add(0xc0), 0);
    if location != 0 {
        e.call(PROJECTILE_WITH_LOCATION, &args![this, location]);
    } else {
        e.call(PROJECTILE_WITHOUT_LOCATION, &args![this, 0u32]);
    }
    magic_ball_projectile_init_3d(e, this);
    fn_0080feb0(e, this);
    e.with_stack(0x10, |e, velocity| {
        let lowest = x87_store32(-x87_load32(e.mem.u32(FLOAT_MAX)));
        e.call(
            VECTOR4_SET,
            &args![velocity, lowest, 0.0f32, 0.0f32, 0.0f32],
        );
        let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).ptr();
        fn_0080fb00(e, controller, velocity);
    });
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    e.call(CELL_ADD_REFERENCE, &args![cell, this, 0u32]);
    restart_flight_sound(e, this);
    if light != 0 {
        e.call(MAGIC_LIGHT_TRANSFER, &args![light, this]);
        e.mem.set_u32(base.wrapping_add(0xc0), light);
    }
    e.call(CHECK_SHOT_BEHIND_WALL, &args![this]);
    if caster_reference(e, this) != e.mem.u32(PLAYER) {
        let value = setting_bits(e, COUNT_SETTING);
        fn_0080fae0(e, f32::from_bits(value));
    }
    e.call(FORM_SET_FLAG, &args![this, 0x1000_0000u32]);
    this
}

// Translated from 0080fae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one `float`): adds it to the running total at
/// `011dc778` (x87 `FADD`, stored as `float`).
pub fn fn_0080fae0(e: &mut Engine, value: f32) {
    let total = x87_add(
        x87_load32(e.mem.u32(RUNNING_TOTAL)),
        x87_m32(value.to_bits()),
    );
    e.mem.set_u32(RUNNING_TOTAL, x87_store32(total));
}

// Translated from 0080fb30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080fb30 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// The `MagicBallProjectile` destructor (unnamed in the engine map): puts back the two
/// vtables, takes the setting at `011d1c28` back off the running total ([`fn_0080fc90`]) for
/// casters other than the player, stops and releases a valid sound handle (+0xB4), deletes
/// the light (+0xC0, [`fn_0080fcc0`] with 1) and clears it, destroys the handle and runs
/// `MagicProjectile::~MagicProjectile`.
pub fn fn_0080fb30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, MAGIC_BALL_PROJECTILE_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x18), MAGIC_BALL_PROJECTILE_SECOND_VTABLE);
    if caster_reference(e, this) != e.mem.u32(PLAYER) {
        let value = setting_bits(e, COUNT_SETTING);
        fn_0080fc90(e, f32::from_bits(value));
    }
    let handle = base.wrapping_add(0xb4);
    if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
        e.call(SOUND_HANDLE_STOP, &args![handle]);
        if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
            e.call(SOUND_HANDLE_RELEASE, &args![handle]);
        }
    }
    // The exe tests the word, then its copy.
    let light = e.mem.u32(base.wrapping_add(0xc0));
    let object = light;
    if light != 0 && object != 0 {
        fn_0080fcc0(e, Ptr::new(object), 1);
    }
    e.mem.set_u32(base.wrapping_add(0xc0), 0);
    e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    e.call(MAGIC_PROJECTILE_DESTRUCTOR, &args![this]);
}

// Translated from 0080fc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one `float`): takes `00404010(0.0, value)` off the
/// running total at `011dc778` (x87 `FSUBR`, stored as `float`).
pub fn fn_0080fc90(e: &mut Engine, value: f32) {
    let part = e
        .call(TOTAL_PART, &args![0.0f32, x87_quiet_bits(value.to_bits())])
        .f64();
    let total = x87_sub(x87_m32(e.mem.u32(RUNNING_TOTAL)), part);
    e.mem.set_u32(RUNNING_TOTAL, x87_store32(total));
}

// Translated from 0080fcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MagicLight` scalar deleting destructor (unnamed in the engine map; `thiscall`,
/// `RET 4`): the destructor body `0081c240`, then `operator delete` when bit 0 of the flags
/// is set. Returns `this`.
pub fn fn_0080fcc0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MAGIC_LIGHT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 0080fcf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080fcf0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `MagicBallProjectile::Init3D` (Xbox PDB). With a node (slot +0x1D0): its scale is +0xB0,
/// its translation the slot +0x1F4 point, its rotation the matrix of the reference's angles
/// (`00a59660(z, x, y)`), then `ProcessSelectiveUpdate`; when the node has a controller
/// manager with a `"SpecialIdle_Projectile"` sequence, the sequence is started and the node
/// updated with the frame time (`011c3c08`) minus `0.1`.
pub fn magic_ball_projectile_init_3d(e: &mut Engine, this: Ptr) {
    let node = node(e, this);
    if node == 0 {
        return;
    }
    let scale = x87_quiet_bits(e.mem.u32(this.addr().wrapping_add(0xb0)));
    e.call(NODE_SET_SCALE, &args![node, scale]);
    let position = e.vcall(this.addr(), 0x1f4, &[]).u32();
    e.call(NODE_SET_TRANSLATE, &args![node, position]);
    let angles = e.call(ANGLES, &args![this]).u32();
    let x = e.mem.u32(angles);
    let y = e.mem.u32(angles.wrapping_add(4));
    let z = e.mem.u32(angles.wrapping_add(8));
    e.with_stack(0x24, |e, matrix| {
        e.call(MATRIX_SELF, &args![matrix]);
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
    let source = e.call(NODE_CONTROLLER_SOURCE, &args![node]).u32();
    let manager = e
        .call(CONTROLLER_MANAGER_OF, &args![CONTROLLER_MANAGERS, source])
        .u32();
    if manager == 0 {
        return;
    }
    let sequence = find_sequence(e, manager, TEXT_SPECIAL_IDLE_PROJECTILE);
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

// Translated from 0080feb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080feb0 - the FS:[0] exception frame and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map: sets up the projectile's character controller, only while
/// the object at +0x68 exists and its `0045cd60` answers zero. Without a controller
/// (`GetCharController`) it builds one: a construction info (`00c6da50(1)`) given `1.0`
/// ([`fn_00810550`]), the node, the position ([`fn_00810510`]), the cell's world, `0.0`, a
/// sphere shape (`1.0`, 1), the flag 0 ([`fn_00810530`]), the collision layer 7 with the
/// group of the caster actor, of a caster reference's node or of this node (0 when the
/// save/load manager's `0047c850` is true) and 6 ([`fn_00810570`]); the controller (0x660
/// bytes, `00806eb0`) gets `1.0` ([`fn_008106b0`]), `00525420`'s flag ([`fn_00810610`]),
/// 1 ([`fn_008105e0`]), is handed (in an `NiPointer`) to the +0x68 object's slot +0x290 and
/// gets `this` ([`fn_00810640`]). Then, for the controller: `0057b1d0` with the cell's water
/// height (`-3.4028235e38` without cell or world), the world, the node (with wireframe and
/// motion set up when the world's `004bae00` is true), and, in a world, the position (8
/// higher when the world changed, the old world's objects removed) and the world's slot
/// +0xD0 add.
pub fn fn_0080feb0(e: &mut Engine, this: Ptr) {
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
    let node = node(e, this);
    let current = fn_00810660(e, Ptr::new(controller));
    let mut changed = true;
    if current != node {
        e.call(CONTROLLER_SET_NODE, &args![controller, node]);
        if world != 0 && e.call(WORLD_FLAG, &args![world]).bool() {
            e.vcall(controller, 0xd0, &args![0u32]);
            let object = fn_00810660(e, Ptr::new(controller));
            e.call(SET_WIREFRAME, &args![object, 1u32]);
        }
        e.call(WORLD_SET_MOTION, &args![node, 4u32, 1u32, 0u32, 1u32]);
        changed = true;
    }
    let [x, y, mut z] = position_words(e, this);
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
        e.mem.set_u32(position.addr(), x);
        e.mem.set_u32(position.addr().wrapping_add(4), y);
        e.mem.set_u32(position.addr().wrapping_add(8), z);
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

/// [`fn_0080feb0`]'s controller construction on the local construction info.
fn build_controller(e: &mut Engine, this: Ptr, info: Ptr, world: u32) -> u32 {
    let base = this.addr();
    e.call(CONTROLLER_INFO_CONSTRUCT, &args![info, 1u32]);
    fn_00810550(e, info, 1.0);
    let node_object = node(e, this);
    e.call(CONTROLLER_INFO_SET_NODE, &args![info, node_object]);
    let position = e.vcall(base, 0x1f4, &[]).ptr();
    fn_00810510(e, info, position);
    e.call(CONTROLLER_INFO_SET_WORLD, &args![info, world]);
    e.call(CONTROLLER_INFO_SET_FLOAT, &args![info, 0.0f32]);
    let block = e.call(ALLOCATE_OBJECT, &args![0x14u32]).u32();
    let shape = if block != 0 {
        e.call(SPHERE_SHAPE_CONSTRUCTOR, &args![block, 1.0f32, 1u32])
            .u32()
    } else {
        0
    };
    fn_008105c0(e, info, shape);
    fn_00810530(e, info, 0);
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_LOAD_FLAG, &args![manager]).bool() {
        fn_00810590(e, info, 7, 0);
    } else {
        let reference = caster_reference(e, this);
        let layer = if reference != 0 {
            if e.vcall(reference, 0x100, &[]).bool() {
                actor_layer(e, reference)
            } else {
                let reference_node = e.vcall(reference, 0x1d0, &[]).u32();
                node_layer(e, reference_node)
            }
        } else {
            let own_node = node(e, this);
            node_layer(e, own_node)
        };
        fn_00810590(e, info, 7, layer);
    }
    fn_00810570(e, info, 6);
    let block = e.call(ALLOCATE_CONTROLLER, &args![0x660u32]).u32();
    let made = if block != 0 {
        e.call(CHARACTER_CONTROLLER_CONSTRUCTOR, &args![block, info, this])
            .u32()
    } else {
        0
    };
    fn_008106b0(e, Ptr::new(made), 1.0);
    let flag = e.call(CONTROLLER_FLAG_SOURCE, &[]).u8();
    fn_00810610(e, Ptr::new(made), flag);
    fn_008105e0(e, Ptr::new(made), 1);
    // The argument is an `NiPointer` built in its stack slot; the callee owns it.
    let holder = e.with_stack(4, |e, slot| {
        e.call(POINTER_CONSTRUCT, &args![slot, made]);
        e.mem.u32(slot.addr())
    });
    let object = e.mem.u32(base.wrapping_add(0x68));
    e.vcall(object, 0x290, &args![holder]);
    fn_00810640(e, Ptr::new(made), base);
    e.call(CONTROLLER_INFO_DESTRUCT, &args![info]);
    made
}

// Translated from 00810590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): `006222c0` on the object at +0x84 with
/// the two words and 0.
pub fn fn_00810590(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        INFO_FILTER_SET,
        &args![this.addr().wrapping_add(0x84), first, second, 0u32],
    );
}

// Translated from 008105c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): assigns the pointer to the `NiPointer`
/// at +0xA0 (`0066b0d0`).
pub fn fn_008105c0(e: &mut Engine, this: Ptr, object: u32) {
    e.call(
        POINTER_ASSIGN,
        &args![this.addr().wrapping_add(0xa0), object],
    );
}

// Translated from 008105e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `00629670` on the object at +0x410 with
/// the mask `0x80000` and the flag byte (zero-extended).
pub fn fn_008105e0(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(
        FLAG_SET_OR_CLEAR,
        &args![this.addr().wrapping_add(0x410), 0x0008_0000u32, flag],
    );
}

// Translated from 008106b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores the `float` at +0x548 (`FLD` /
/// `FSTP`: a signalling NaN is stored quiet).
pub fn fn_008106b0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0x548),
        x87_quiet_bits(value.to_bits()),
    );
}

// Translated from 008106d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008106d0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 4`: the elapsed time): the ball's update.
/// At 60 or more (the `double` at `01012638`) slot +0xC4 runs with 1; below `0.0` (or
/// unordered) nothing more happens. By state (+0xAC; 0 becomes 1):
///
/// - 1 (flying): stops when `00440d80` is true; when the position's z is at or below
///   `004587d0` of the controller's [`fn_00810a80`], slot +0x304 (the position, 0, 0, 1);
///   slot +0x250 moves it by `(0, +0x88 * time, 0)` (time, the point, 0xF); when slot +0x30C
///   answers true, the node follows, the distance moved is added to +0x8C and, beyond the
///   setting at `011d1d00`, slot +0xC4 runs with 1; the light (+0xC0) is updated;
/// - 2 (exploding): when the node's controller manager has no `"SpecialIdle_AreaEffect"`
///   sequence, or it has run out (its `00508100` below its `00508120`, or `008041a0` zero),
///   deactivates the manager, disables the reference and runs slot +0xC4 with 1 (slot first,
///   then the disabling, without a manager); the light is updated.
pub fn fn_008106d0(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    let time_value = x87_load32(time.to_bits());
    // FCOMP / TEST AH,1 / JNZ: the slot runs when not below and ordered.
    let sixty = e.mem.f64(SIXTY);
    if time_value >= sixty {
        e.vcall(base, 0xc4, &args![1u32]);
    }
    let zero = e.mem.f64(ZERO);
    if !(time_value >= zero) {
        return;
    }
    let mut state = e.mem.u32(base.wrapping_add(0xac));
    if state == 0 {
        e.mem.set_u32(base.wrapping_add(0xac), 1);
        state = 1;
    }
    match state {
        1 => ball_fly(e, this, time),
        2 => {
            let node = node(e, this);
            let manager = controller_manager(e, node);
            if manager != 0 {
                let sequence = find_sequence(e, manager, TEXT_SPECIAL_IDLE_AREA_EFFECT);
                let finished = if sequence == 0 {
                    true
                } else {
                    let first = e.call(SEQUENCE_FIRST_FLOAT, &args![sequence]).f64();
                    let second = e.call(SEQUENCE_SECOND_FLOAT, &args![sequence]).f64();
                    // FCOMP / TEST AH,5 / JNP: finished when the second is below.
                    second < first || e.call(SEQUENCE_WORD, &args![sequence]).u32() == 0
                };
                if finished {
                    e.call(DEACTIVATE_ALL, &args![manager, 0.0f32]);
                    e.call(FORM_SET_DISABLED, &args![this, 1u32]);
                    e.vcall(base, 0xc4, &args![1u32]);
                }
            } else {
                e.vcall(base, 0xc4, &args![1u32]);
                e.call(FORM_SET_DISABLED, &args![this, 1u32]);
            }
            update_light(e, this, time);
        }
        _ => {}
    }
}

/// [`fn_008106d0`]'s flying state.
fn ball_fly(e: &mut Engine, this: Ptr, time: f32) {
    let base = this.addr();
    if e.call(REFERENCE_FLAG, &args![this]).bool() {
        return;
    }
    let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).ptr();
    let height = x87_store32(fn_00810a80(e, controller));
    let limit = e.call(HEIGHT_OFFSET, &args![height]).f64();
    let position = e.vcall(base, 0x1f4, &[]).u32();
    let z = x87_load32(e.mem.u32(position.wrapping_add(8)));
    // TEST AH,0x41 / JP: on when below or equal (ordered).
    if z <= limit {
        let [x, y, z] = position_words(e, this);
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
        let start = position_words(e, this);
        for (i, word) in start.iter().enumerate() {
            e.mem.set_u32(before.wrapping_add(4 * i as u32), *word);
        }
        e.vcall(
            base,
            0x250,
            &args![x87_quiet_bits(time.to_bits()), step, 0xfu32],
        );
        if !e.vcall(base, 0x30c, &[]).bool() {
            return;
        }
        let current = position_words(e, this);
        for (i, word) in current.iter().enumerate() {
            e.mem.set_u32(now.wrapping_add(4 * i as u32), *word);
        }
        if node(e, this) != 0 {
            let node = node(e, this);
            e.call(NODE_SET_TRANSLATE, &args![node, now]);
        }
        let out = e
            .call(POINT_DIFFERENCE, &args![now, difference, before])
            .u32();
        let length = e.call(POINT_LENGTH, &args![out]).f64();
        let travelled = base.wrapping_add(0x8c);
        let sum = x87_add(length, x87_m32(e.mem.u32(travelled)));
        e.mem.set_u32(travelled, x87_store32(sum));
        let limit = e.call(SETTING_VALUE, &args![DISTANCE_SETTING]).u32();
        let distance = x87_load32(e.mem.u32(travelled));
        let limit = x87_load32(e.mem.u32(limit));
        // FCOMPP (the limit against the distance) / TEST AH,5 / JP: on when the limit is
        // below.
        if limit < distance {
            e.vcall(base, 0xc4, &args![1u32]);
        }
        update_light(e, this, time);
    });
}

// Translated from 00810a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the `float` at +0x53C in `ST0` (`FLD`: a
/// signalling NaN is quieted).
pub fn fn_00810a80(e: &mut Engine, this: Ptr) -> f64 {
    x87_load32(e.mem.u32(this.addr().wrapping_add(0x53c)))
}

// Translated from 00810aa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00810aa0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 0x18`: a point by value, a word passed on to
/// `0081f000`, the reference hit, a flag byte): the ball explodes. Nothing happens when the
/// reference is an actor (slot +0x100) whose `008ace90` is true. +0xB0 = the effect item's
/// area times `fMagicAreaScale`, lowered to `fMagicAreaScaleMax` / raised to
/// `fMagicAreaScaleMin`; the reference moves to the point (`SetLocationOnReference`); with a
/// node, `0081f000` (the point, the offset at `011a9478` through the node's transform, the
/// word), and, without the flag, the node moves there and its `"AreaEffect"` child (slot
/// +0x9C) is scaled to +0xB0; a valid sound is stopped; without the flag the controller
/// runs slot +0x310 when its object has flag `0x8000` and its collision filter gets bit
/// `0x4000` ([`fn_00810f90`], [`bhk_character_controller_set_collision_filter`]); the state
/// (+0xAC) becomes 2; without the flag the caster's `FindTargets_ov2` runs; a
/// `"SpecialIdle_AreaEffect"` sequence is started (the light fading over `0.75` of its
/// `00508100`); without the flag, the impact sound plays at the position.
#[allow(clippy::too_many_arguments)]
pub fn fn_00810aa0(
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
    let item = e.call(PROJECTILE_EFFECT_ITEM, &args![this]).u32();
    let area = e.call(EFFECT_ITEM_GET_AREA, &args![item]).i32();
    let scale_value = e.call(SETTING_VALUE, &args![AREA_SCALE]).u32();
    let scale = x87_store32(x87_mul(x87_load32(e.mem.u32(scale_value)), f64::from(area)));
    let scale_field = base.wrapping_add(0xb0);
    e.mem.set_u32(scale_field, scale);
    let max = e.call(SETTING_VALUE, &args![AREA_SCALE_MAX]).u32();
    let current = x87_load32(e.mem.u32(scale_field));
    // FCOMPP (the maximum against the scale) / TEST AH,5 / JP: lowered when the maximum is
    // below.
    if x87_load32(e.mem.u32(max)) < current {
        let max = setting_bits(e, AREA_SCALE_MAX);
        e.mem.set_u32(scale_field, max);
    } else {
        let min = e.call(SETTING_VALUE, &args![AREA_SCALE_MIN]).u32();
        let current = x87_load32(e.mem.u32(scale_field));
        // FCOMPP / TEST AH,0x41 / JNZ: raised when the minimum is above.
        if x87_load32(e.mem.u32(min)) > current {
            let min = setting_bits(e, AREA_SCALE_MIN);
            e.mem.set_u32(scale_field, min);
        }
    }
    let flag = flag != 0;
    // The point argument lives in the caller's stack; `SetLocationOnReference` gets its
    // address.
    e.with_stack(0x0c, |e, point| {
        let point = point.addr();
        e.mem.set_u32(point, x);
        e.mem.set_u32(point.wrapping_add(4), y);
        e.mem.set_u32(point.wrapping_add(8), z);
        e.call(SET_LOCATION_ON_REFERENCE, &args![this, point]);
        let node = node(e, this);
        if node != 0 {
            let transform = e.call(NODE_TRANSFORM, &args![node]).u32();
            let offset = e.with_stack(0x0c, |e, out| {
                let moved = e
                    .call(TRANSFORM_APPLY, &args![transform, out, EXPLOSION_OFFSET])
                    .u32();
                [
                    e.mem.u32(moved),
                    e.mem.u32(moved.wrapping_add(4)),
                    e.mem.u32(moved.wrapping_add(8)),
                ]
            });
            let at = [
                e.mem.u32(point),
                e.mem.u32(point.wrapping_add(4)),
                e.mem.u32(point.wrapping_add(8)),
            ];
            e.call(
                PROJECTILE_EXPLODE_AT,
                &args![this, at[0], at[1], at[2], offset[0], offset[1], offset[2], passed],
            );
        }
        if node != 0 && !flag {
            e.call(NODE_SET_TRANSLATE, &args![node, point]);
            let child = e.with_stack(4, |e, name| {
                let name_pointer = e
                    .call(FIXED_STRING_CONSTRUCT, &args![name, TEXT_AREA_EFFECT])
                    .u32();
                let child = e.vcall(node, 0x9c, &args![name_pointer]).u32();
                e.call(FIXED_STRING_DESTRUCT, &args![name]);
                child
            });
            if child != 0 {
                let scale = x87_quiet_bits(e.mem.u32(scale_field));
                e.call(NODE_SET_SCALE, &args![child, scale]);
            }
        }
    });
    let handle = base.wrapping_add(0xb4);
    if e.call(SOUND_HANDLE_IS_VALID, &args![handle]).bool() {
        e.call(SOUND_HANDLE_STOP, &args![handle]);
    }
    if e.call(GET_CHAR_CONTROLLER, &args![this]).u32() != 0 && !flag {
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
        e.with_stack(4, |e, filter| {
            let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
            e.call(CONTROLLER_FILTER, &args![controller, filter]);
            fn_00810f90(e, filter, 1);
            let word = e.mem.u32(filter.addr());
            let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).ptr();
            bhk_character_controller_set_collision_filter(e, controller, word);
        });
    }
    e.mem.set_u32(base.wrapping_add(0xac), 2);
    if !flag {
        let [px, py, pz] = position_words(e, this);
        let cell = e.call(PARENT_CELL, &args![this]).u32();
        let kind = fn_00810fd0(e, this);
        let caster = e.call(PROJECTILE_CASTER, &args![this]).u32();
        e.call(
            FIND_TARGETS_OV2,
            &args![caster, kind, cell, px, py, pz, this, reference, 0u32, 1.0f32, 1.0f32],
        );
    }
    let node = node(e, this);
    let manager = controller_manager(e, node);
    if manager != 0 {
        let sequence = find_sequence(e, manager, TEXT_SPECIAL_IDLE_AREA_EFFECT);
        if sequence != 0 {
            start_sequence(e, manager, sequence);
            let light = e.mem.u32(base.wrapping_add(0xc0));
            if light != 0 {
                let length = e.call(SEQUENCE_SECOND_FLOAT, &args![sequence]).f64();
                let fade = x87_store32(x87_mul(length, e.mem.f64(THREE_QUARTERS)));
                e.call(MAGIC_LIGHT_FADE, &args![light, 0u32, fade]);
            }
        }
    }
    let form = e.call(BASE_FORM, &args![this]).u32();
    if e.call(PROJECTILE_IMPACT_SOUND, &args![form]).u32() != 0 && !flag {
        let form = e.call(BASE_FORM, &args![this]).u32();
        let sound = e.call(PROJECTILE_IMPACT_SOUND, &args![form]).u32();
        let id = e.call(FORM_ID, &args![sound]).u32();
        e.with_stack(0x0c, |e, handle| {
            let audio = e.call(AUDIO_INSTANCE, &[]).u32();
            e.call(GET_SOUND_HANDLE_BY_ID, &args![audio, handle, id, 0x102u32]);
            let [px, py, pz] = position_words(e, this);
            e.call(
                SOUND_HANDLE_SET_POSITION,
                &args![
                    handle,
                    x87_quiet_bits(px),
                    x87_quiet_bits(py),
                    x87_quiet_bits(pz)
                ],
            );
            e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
            e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
        });
    }
}

// Translated from 00810f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): sets bit `0x4000` of the word at `this`
/// when the flag byte is non-zero, clears it otherwise.
pub fn fn_00810f90(e: &mut Engine, this: Ptr, set: u8) {
    let word = e.mem.u32(this.addr());
    if set != 0 {
        e.mem.set_u32(this.addr(), word | 0x4000);
    } else {
        e.mem.set_u32(this.addr(), word & 0xffff_bfff);
    }
}

// Translated from 00810ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterController::SetCollisionFilter` (engine map; `thiscall`, `RET 4`): with an
/// object at +0x594 (`00559450`), hands it the filter word (`00626cb0`).
pub fn bhk_character_controller_set_collision_filter(e: &mut Engine, this: Ptr, filter: u32) {
    let slot = this.addr().wrapping_add(0x594);
    if e.call(READ_FIRST_WORD, &args![slot]).u32() != 0 {
        let object = e.call(READ_FIRST_WORD, &args![slot]).u32();
        e.call(OBJECT_SET_FILTER, &args![object, filter]);
    }
}

// Translated from 00811030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): true when the state (+0xAC) is not 2 and
/// `00440d80` answers false.
pub fn fn_00811030(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(0xac)) != 2 && !e.call(REFERENCE_FLAG, &args![this]).bool()
}

// Translated from 00811070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; the save size): the `MagicProjectile`
/// size (`0081e830`) plus 4 + 4, plus 4 for a save version (`008df040`) below 100, plus
/// 4 + 4, plus 4 in state 2 (16-bit arithmetic).
pub fn fn_00811070(e: &mut Engine, this: Ptr, buffer: u32) -> u16 {
    let mut size = 0u16;
    let base_size = e
        .call(MAGIC_PROJECTILE_SAVE_SIZE, &args![this, buffer])
        .u16();
    size = size.wrapping_add(base_size);
    size = size.wrapping_add(4);
    size = size.wrapping_add(4);
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_VERSION, &args![manager]).u8() < 100 {
        size = size.wrapping_add(4);
    }
    size = size.wrapping_add(4);
    size = size.wrapping_add(4);
    if e.mem.u32(this.addr().wrapping_add(0xac)) == 2 {
        size = size.wrapping_add(4);
    }
    size
}

// Translated from 00811110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; the save): `MagicProjectile::SaveGame`,
/// then +0xA8, +0x88, +0x8C (save version below 100 only), the state +0xAC (through the
/// save/load manager), +0xB0, and in state 2 the light's `00621b00` (`0.0` without a light),
/// 4 bytes each.
pub fn fn_00811110(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    e.call(MAGIC_PROJECTILE_SAVE_GAME, &args![this, buffer]);
    e.call(SAVE_DATA, &args![this, base.wrapping_add(0xa8), 4u32]);
    e.call(SAVE_DATA, &args![this, base.wrapping_add(0x88), 4u32]);
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_VERSION, &args![manager]).u8() < 100 {
        e.call(SAVE_DATA, &args![this, base.wrapping_add(0x8c), 4u32]);
    }
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    e.call(SAVE_WORD, &args![manager, base.wrapping_add(0xac), 4u32]);
    e.call(SAVE_DATA, &args![this, base.wrapping_add(0xb0), 4u32]);
    if e.mem.u32(base.wrapping_add(0xac)) == 2 {
        e.with_stack(4, |e, value| {
            e.mem.set_u32(value.addr(), 0.0f32.to_bits());
            let light = e.mem.u32(base.wrapping_add(0xc0));
            if light != 0 {
                let saved = e.call(MAGIC_LIGHT_GET_FLOAT, &args![light]).f64();
                e.mem.set_u32(value.addr(), x87_store32(saved));
            }
            e.call(SAVE_DATA, &args![this, value, 4u32]);
        });
    }
}

// Translated from 008111e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`; the load): slot +0x1C4, the
/// `MagicProjectile` load body `0081e980` with both words, then +0xA8, +0x88, +0x8C (save
/// version below 100 only), the state +0xAC (through the save/load manager), +0xB0, and in
/// state 2 a `float` loaded into a local and copied into +0xC0 (`00401460`), where
/// [`magic_ball_projectile_init_load_game`] takes it back.
pub fn fn_008111e0(e: &mut Engine, this: Ptr, buffer: u32, version: u32) {
    let base = this.addr();
    e.vcall(base, 0x1c4, &[]);
    e.call(MAGIC_PROJECTILE_LOAD_GAME, &args![this, buffer, version]);
    e.call(LOAD_DATA, &args![this, base.wrapping_add(0xa8), 4u32]);
    e.call(LOAD_DATA, &args![this, base.wrapping_add(0x88), 4u32]);
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    if e.call(SAVE_VERSION, &args![manager]).u8() < 100 {
        e.call(LOAD_DATA, &args![this, base.wrapping_add(0x8c), 4u32]);
    }
    let manager = e.mem.u32(SAVE_LOAD_MANAGER);
    e.call(LOAD_WORD, &args![manager, base.wrapping_add(0xac), 4u32]);
    e.call(LOAD_DATA, &args![this, base.wrapping_add(0xb0), 4u32]);
    if e.mem.u32(base.wrapping_add(0xac)) == 2 {
        e.with_stack(4, |e, value| {
            e.call(LOAD_DATA, &args![this, value, 4u32]);
            e.call(COPY_MEMORY, &args![base.wrapping_add(0xc0), value, 4u32]);
        });
    }
}

// Translated from 008112c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008112c0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `MagicBallProjectile::InitLoadGame` (Xbox PDB; `thiscall`, `RET 8`):
/// `MagicProjectile::InitLoadGame`, [`magic_ball_projectile_init_3d`], the cell's
/// `AddReference`; outside state 2 the flight sound handle is made again; the node is scaled
/// to +0xB0; the word at +0xC0 (the `float` the load left there) is copied into a local and
/// +0xC0 cleared; with a node and a light form, a new `MagicLight` (0x1C bytes, `0081bf80`
/// with the light form's id and the node) goes to +0xC0, and in state 2 it fades over `0.75`
/// of the `"SpecialIdle_AreaEffect"` sequence's `00508100` and gets the saved `float`
/// (`00621b20`).
pub fn magic_ball_projectile_init_load_game(e: &mut Engine, this: Ptr, buffer: u32, version: u32) {
    let base = this.addr();
    e.call(
        MAGIC_PROJECTILE_INIT_LOAD_GAME,
        &args![this, buffer, version],
    );
    magic_ball_projectile_init_3d(e, this);
    let cell = e.call(PARENT_CELL, &args![this]).u32();
    e.call(CELL_ADD_REFERENCE, &args![cell, this, 0u32]);
    if e.mem.u32(base.wrapping_add(0xac)) != 2 {
        restart_flight_sound(e, this);
    }
    if node(e, this) != 0 {
        let scale = x87_quiet_bits(e.mem.u32(base.wrapping_add(0xb0)));
        let node = node(e, this);
        e.call(NODE_SET_SCALE, &args![node, scale]);
    }
    e.with_stack(4, |e, saved| {
        e.call(COPY_MEMORY, &args![saved, base.wrapping_add(0xc0), 4u32]);
        e.mem.set_u32(base.wrapping_add(0xc0), 0);
        if node(e, this) == 0 {
            return;
        }
        let form = e.call(BASE_FORM, &args![this]).u32();
        if e.call(PROJECTILE_LIGHT, &args![form]).u32() == 0 {
            return;
        }
        let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
        let light = if block != 0 {
            let node = node(e, this);
            let form = e.call(BASE_FORM, &args![this]).u32();
            let light_form = e.call(PROJECTILE_LIGHT, &args![form]).u32();
            let id = e.call(FORM_ID, &args![light_form]).u32();
            e.call(MAGIC_LIGHT_CONSTRUCTOR, &args![block, id, node])
                .u32()
        } else {
            0
        };
        e.mem.set_u32(base.wrapping_add(0xc0), light);
        if e.mem.u32(base.wrapping_add(0xac)) != 2 || e.mem.u32(base.wrapping_add(0xc0)) == 0 {
            return;
        }
        let node = node(e, this);
        let manager = controller_manager(e, node);
        if manager == 0 {
            return;
        }
        let sequence = find_sequence(e, manager, TEXT_SPECIAL_IDLE_AREA_EFFECT);
        if sequence == 0 {
            return;
        }
        let length = e.call(SEQUENCE_SECOND_FLOAT, &args![sequence]).f64();
        let fade = x87_store32(x87_mul(length, e.mem.f64(THREE_QUARTERS)));
        let light = e.mem.u32(base.wrapping_add(0xc0));
        e.call(MAGIC_LIGHT_FADE, &args![light, 0u32, fade]);
        let value = x87_quiet_bits(e.mem.u32(saved.addr()));
        let light = e.mem.u32(base.wrapping_add(0xc0));
        e.call(MAGIC_LIGHT_SET_FLOAT, &args![light, value]);
    });
}

// Translated from 008115c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicBallProjectile::FinishInitLoadGame` (Xbox PDB; `thiscall`, `RET 8`):
/// `MobileObject::FinishInitLoadGame`; with a controller, its collision filter word gets
/// layer 7 (`004a39f0`) and the group (`0059ce80`) of the caster actor, of a caster
/// reference's node or of this node, and is set
/// ([`bhk_character_controller_set_collision_filter`]); then, for casters other than the
/// player, [`fn_0080fae0`] adds the setting at `011d1c28` to the running total.
pub fn magic_ball_projectile_finish_init_load_game(
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
            let actor = if e.vcall(reference, 0x100, &[]).bool() {
                reference
            } else {
                0
            };
            if actor != 0 {
                actor_layer(e, actor)
            } else {
                let reference_node = e.vcall(reference, 0x1d0, &[]).u32();
                node_layer(e, reference_node)
            }
        } else {
            let own_node = node(e, this);
            node_layer(e, own_node)
        };
        e.with_stack(4, |e, filter| {
            e.call(CONTROLLER_FILTER, &args![controller, filter]);
            e.call(FILTER_SET_LAYER, &args![filter, 7u32]);
            e.call(FILTER_SET_GROUP, &args![filter, group]);
            let word = e.mem.u32(filter.addr());
            bhk_character_controller_set_collision_filter(e, Ptr::new(controller), word);
        });
    }
    if caster_reference(e, this) != e.mem.u32(PLAYER) {
        let value = setting_bits(e, COUNT_SETTING);
        fn_0080fae0(e, f32::from_bits(value));
    }
}

#[cfg(test)]
mod magic_ball_tests {
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

    /// The double behind slot `slot` of the objects made with [`object`] for `family`.
    fn slot(family: u32, slot: u32) -> u32 {
        0x0320_0000 + family * 0x1_0000 + slot
    }

    /// An object of `size` bytes whose vtable sends each slot to [`slot`]`(family, slot)`.
    fn object(e: &mut Engine, family: u32, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for i in 0..0x100 {
            e.mem.set_u32(vtable + 4 * i, slot(family, 4 * i));
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A float setting whose value address `00403e20` answers.
    fn settings(e: &mut Engine, values: &'static [(u32, f32)]) {
        e.register_double(SETTING_VALUE, move |e, a| {
            let value = values
                .iter()
                .find(|(s, _)| *s == a[0])
                .map(|(_, v)| *v)
                .unwrap_or(0.0);
            let p = e.mem.alloc(4);
            e.mem.set_f32(p, value);
            p.into_ret()
        });
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_5000,
            0x0101_d000,
            0x0101_f000,
            0x0102_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(0x0107_a000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_c000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_f32(FLOAT_MAX, f32::MAX);
        e.mem.set_f32(LOWEST_FLOAT, -f32::MAX);
        e.mem.set_f64(TENTH, 0.1);
        e.mem.set_f64(THREE_QUARTERS, 0.75);
        e.mem.set_f64(SIXTY, 60.0);
        e.mem.set_f64(ZERO, 0.0);
        e.mem.set_f64(EIGHT, 8.0);
        e.mem.set_u32(PLAYER, 0x0bad_0001);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn running_total_adds_and_takes_away() {
        let mut e = engine();
        e.mem.set_f32(RUNNING_TOTAL, 1.5);
        e.call(0x0080_fae0, &args![2.25f32]);
        assert_eq!(e.mem.f32(RUNNING_TOTAL), 3.75);
        e.register(TOTAL_PART, |_, _| st0(0.75));
        e.call(0x0080_fc90, &args![2.0f32]);
        assert_eq!(e.mem.f32(RUNNING_TOTAL), 3.0);
        assert_eq!(calls(&e, TOTAL_PART), vec![vec![0, 2.0f32.to_bits()]]);
    }

    #[test]
    fn magic_light_deleting_destructor() {
        let mut e = engine();
        quiet(&mut e, &[MAGIC_LIGHT_DESTRUCTOR, DEALLOCATE]);
        assert_eq!(e.call(0x0080_fcc0, &args![0x4000u32, 0u32]).u32(), 0x4000);
        assert!(calls(&e, DEALLOCATE).is_empty());
        e.call(0x0080_fcc0, &args![0x4000u32, 1u32]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![0x4000]]);
        assert_eq!(calls(&e, MAGIC_LIGHT_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn small_setters_and_getters() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                INFO_FILTER_SET,
                POINTER_ASSIGN,
                FLAG_SET_OR_CLEAR,
                OBJECT_SET_FILTER,
            ],
        );
        let this = e.mem.alloc(0x600);
        e.call(0x0081_0590, &args![this, 7u32, 9u32]);
        assert_eq!(calls(&e, INFO_FILTER_SET), vec![vec![this + 0x84, 7, 9, 0]]);
        e.call(0x0081_05c0, &args![this, 0x55u32]);
        assert_eq!(calls(&e, POINTER_ASSIGN), vec![vec![this + 0xa0, 0x55]]);
        e.call(0x0081_05e0, &args![this, 1u8]);
        assert_eq!(
            calls(&e, FLAG_SET_OR_CLEAR),
            vec![vec![this + 0x410, 0x8_0000, 1]]
        );
        e.call(0x0081_06b0, &args![this, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(this + 0x548), 0x7fc0_0001);
        e.mem.set_f32(this + 0x53c, 4.5);
        assert_eq!(e.call(0x0081_0a80, &args![this]).f64(), 4.5);
        e.mem.set_u32(this, 0x1_0001);
        e.call(0x0081_0f90, &args![this, 1u8]);
        assert_eq!(e.mem.u32(this), 0x1_4001);
        e.call(0x0081_0f90, &args![this, 0u8]);
        assert_eq!(e.mem.u32(this), 0x1_0001);
        e.register(READ_FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.call(0x0081_0ff0, &args![this, 0x77u32]);
        assert!(calls(&e, OBJECT_SET_FILTER).is_empty());
        e.mem.set_u32(this + 0x594, 0x9000);
        e.call(0x0081_0ff0, &args![this, 0x77u32]);
        assert_eq!(calls(&e, OBJECT_SET_FILTER), vec![vec![0x9000, 0x77]]);
        e.register(REFERENCE_FLAG, |_, _| 0u32.into_ret());
        assert!(e.call(0x0081_1030, &args![this]).bool());
        e.mem.set_u32(this + 0xac, 2);
        assert!(!e.call(0x0081_1030, &args![this]).bool());
        e.mem.set_u32(this + 0xac, 1);
        e.register(REFERENCE_FLAG, |_, _| 1u32.into_ret());
        assert!(!e.call(0x0081_1030, &args![this]).bool());
    }

    #[test]
    fn save_size_counts_the_fields() {
        let mut e = engine();
        e.register(MAGIC_PROJECTILE_SAVE_SIZE, |_, _| 0x20u32.into_ret());
        e.register(SAVE_VERSION, |_, _| 99u32.into_ret());
        let this = e.mem.alloc(0x600);
        assert_eq!(e.call(0x0081_1070, &args![this, 0u32]).u16(), 0x34);
        e.mem.set_u32(this + 0xac, 2);
        assert_eq!(e.call(0x0081_1070, &args![this, 0u32]).u16(), 0x38);
        e.register(SAVE_VERSION, |_, _| 100u32.into_ret());
        assert_eq!(e.call(0x0081_1070, &args![this, 0u32]).u16(), 0x34);
        e.register(MAGIC_PROJECTILE_SAVE_SIZE, |_, _| 0xfff0u32.into_ret());
        assert_eq!(e.call(0x0081_1070, &args![this, 0u32]).u16(), 0x0004);
    }

    #[test]
    fn save_and_load_write_the_fields_in_order() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                MAGIC_PROJECTILE_SAVE_GAME,
                SAVE_DATA,
                SAVE_WORD,
                MAGIC_PROJECTILE_LOAD_GAME,
                LOAD_WORD,
                slot(0, 0x1c4),
            ],
        );
        e.register(SAVE_VERSION, |_, _| 50u32.into_ret());
        e.register(MAGIC_LIGHT_GET_FLOAT, |_, _| st0(0.5));
        e.register(LOAD_DATA, |e, a| {
            e.mem.set_f32(a[1], 6.5);
            Ret::default()
        });
        e.register(COPY_MEMORY, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        e.mem.set_u32(SAVE_LOAD_MANAGER, 0x5100);
        let this = object(&mut e, 0, 0x600);
        e.mem.set_u32(this + 0xac, 2);
        e.mem.set_u32(this + 0xc0, 0x4400);
        e.call(0x0081_1110, &args![this, 0x77u32]);
        let saved = calls(&e, SAVE_DATA);
        let fields: Vec<u32> = saved.iter().map(|c| c[1].wrapping_sub(this)).collect();
        assert_eq!(fields[..4], [0xa8, 0x88, 0x8c, 0xb0]);
        assert_eq!(saved.len(), 5);
        assert_eq!(calls(&e, SAVE_WORD), vec![vec![0x5100, this + 0xac, 4]]);
        assert_eq!(calls(&e, MAGIC_LIGHT_GET_FLOAT), vec![vec![0x4400]]);
        e.call(0x0081_11e0, &args![this, 0x77u32, 3u32]);
        assert_eq!(calls(&e, slot(0, 0x1c4)), vec![vec![this]]);
        assert_eq!(
            calls(&e, MAGIC_PROJECTILE_LOAD_GAME),
            vec![vec![this, 0x77, 3]]
        );
        let loaded = calls(&e, LOAD_DATA);
        assert_eq!(loaded.len(), 5);
        assert_eq!(e.mem.f32(this + 0xc0), 6.5);
        assert_eq!(calls(&e, LOAD_WORD), vec![vec![0x5100, this + 0xac, 4]]);
        // A newer save skips +0x8C; outside state 2 nothing more.
        e.register(SAVE_VERSION, |_, _| 100u32.into_ret());
        e.mem.set_u32(this + 0xac, 1);
        e.call(0x0081_1110, &args![this, 0x77u32]);
        assert_eq!(calls(&e, SAVE_DATA).len(), 5 + 3);
    }

    /// A projectile whose node, position and caster are doubled.
    fn projectile(e: &mut Engine, with_node: bool) -> (u32, u32) {
        let this = object(e, 0, 0x600);
        let node = if with_node { e.mem.alloc(0x100) } else { 0 };
        e.register_double(slot(0, 0x1d0), move |_, _| node.into_ret());
        let position = e.mem.alloc(0x10);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        e.register_double(slot(0, 0x1f4), move |_, _| position.into_ret());
        e.register(PROJECTILE_CASTER, |_, _| 0u32.into_ret());
        (this, node)
    }

    #[test]
    fn constructor_initialises_fields_and_counts_other_casters() {
        let mut e = engine();
        let (this, _) = projectile(&mut e, false);
        quiet(
            &mut e,
            &[
                MAGIC_PROJECTILE_CONSTRUCTOR,
                SOUND_HANDLE_CONSTRUCT,
                PROJECTILE_WITHOUT_LOCATION,
                PROJECTILE_WITH_LOCATION,
                VECTOR4_SET,
                CELL_ADD_REFERENCE,
                MAGIC_LIGHT_TRANSFER,
                CHECK_SHOT_BEHIND_WALL,
                FORM_SET_FLAG,
                GET_CHAR_CONTROLLER,
                BASE_FORM,
                PROJECTILE_FLIGHT_SOUND,
            ],
        );
        e.register(PROJECTILE_SPEED, |_, _| st0(100.0));
        e.register(PARENT_CELL, |_, _| 0x7700u32.into_ret());
        e.register(0x004a_e750, |_, _| 0u32.into_ret());
        settings(&mut e, &[(SPEED_SETTING, 2.0), (COUNT_SETTING, 1.25)]);
        // The constructor puts the game's vtable in place; its node slot is doubled too.
        e.mem
            .set_u32(MAGIC_BALL_PROJECTILE_VTABLE + 0x1d0, slot(0, 0x1d0));
        let returned = e
            .call(
                0x0080_f810,
                &args![
                    this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 0u32,
                    0x4400u32
                ],
            )
            .u32();
        assert_eq!(returned, this);
        assert_eq!(
            calls(&e, MAGIC_PROJECTILE_CONSTRUCTOR),
            vec![vec![this, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]
        );
        assert_eq!(e.mem.u32(this), MAGIC_BALL_PROJECTILE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), MAGIC_BALL_PROJECTILE_SECOND_VTABLE);
        assert_eq!(e.mem.f32(this + 0x88), 200.0);
        assert_eq!(e.mem.f32(this + 0xb0), 1.0);
        assert_eq!(e.mem.u32(this + 0xc0), 0x4400);
        assert_eq!(calls(&e, PROJECTILE_WITHOUT_LOCATION), vec![vec![this, 0]]);
        assert_eq!(
            calls(&e, VECTOR4_SET)[0][1..],
            [(-f32::MAX).to_bits(), 0, 0, 0]
        );
        assert_eq!(calls(&e, CELL_ADD_REFERENCE), vec![vec![0x7700, this, 0]]);
        assert_eq!(calls(&e, MAGIC_LIGHT_TRANSFER), vec![vec![0x4400, this]]);
        assert_eq!(e.mem.f32(RUNNING_TOTAL), 1.25);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 0x1000_0000]]);
        // With a location word the other path runs.
        e.call(
            0x0080_f810,
            &args![
                this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 0x99u32, 0u32
            ],
        );
        assert_eq!(calls(&e, PROJECTILE_WITH_LOCATION), vec![vec![this, 0x99]]);
        assert_eq!(e.mem.u32(this + 0xc0), 0);
    }

    #[test]
    fn destructor_releases_the_sound_and_the_light() {
        let mut e = engine();
        let (this, _) = projectile(&mut e, false);
        quiet(
            &mut e,
            &[
                SOUND_HANDLE_STOP,
                SOUND_HANDLE_RELEASE,
                SOUND_HANDLE_DESTRUCT,
                MAGIC_PROJECTILE_DESTRUCTOR,
                MAGIC_LIGHT_DESTRUCTOR,
                DEALLOCATE,
            ],
        );
        e.register(SOUND_HANDLE_IS_VALID, |_, _| 1u32.into_ret());
        e.register(TOTAL_PART, |_, _| st0(1.0));
        settings(&mut e, &[(COUNT_SETTING, 1.0)]);
        e.mem.set_f32(RUNNING_TOTAL, 3.0);
        e.mem.set_u32(this + 0xc0, 0x4400);
        e.call(0x0080_fb30, &args![this]);
        assert_eq!(e.mem.f32(RUNNING_TOTAL), 2.0);
        assert_eq!(calls(&e, SOUND_HANDLE_STOP), vec![vec![this + 0xb4]]);
        assert_eq!(calls(&e, SOUND_HANDLE_RELEASE), vec![vec![this + 0xb4]]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![0x4400]]);
        assert_eq!(e.mem.u32(this + 0xc0), 0);
        assert_eq!(calls(&e, MAGIC_PROJECTILE_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), MAGIC_BALL_PROJECTILE_VTABLE);
    }

    #[test]
    fn init_3d_places_the_node_and_starts_the_sequence() {
        let mut e = engine();
        let (this, node) = projectile(&mut e, true);
        e.mem.set_f32(this + 0xb0, 2.5);
        let angles = e.mem.alloc(0xc);
        e.mem.set_f32(angles, 0.1);
        e.mem.set_f32(angles + 4, 0.2);
        e.mem.set_f32(angles + 8, 0.3);
        e.register_double(ANGLES, move |_, _| angles.into_ret());
        e.register(MATRIX_SELF, |_, a| a[0].into_ret());
        e.register(CONTROLLER_MANAGER_OF, |_, _| 0x8800u32.into_ret());
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FIND_SEQUENCE, |_, _| 0x8900u32.into_ret());
        e.mem.set_f32(FRAME_TIME, 0.6);
        quiet(
            &mut e,
            &[
                NODE_SET_SCALE,
                NODE_SET_TRANSLATE,
                MATRIX_FROM_ANGLES,
                NODE_SET_ROTATE,
                PROCESS_SELECTIVE_UPDATE,
                NODE_CONTROLLER_SOURCE,
                FIXED_STRING_DESTRUCT,
                DEACTIVATE_ALL,
                ACTIVATE_SEQUENCE,
                MANAGER_FLAG,
                SEQUENCE_SET_FLOAT,
                UPDATE_DATA_INIT,
                NODE_UPDATE,
            ],
        );
        e.call(0x0080_fcf0, &args![this]);
        assert_eq!(
            calls(&e, NODE_SET_SCALE),
            vec![vec![node, 2.5f32.to_bits()]]
        );
        let matrix = &calls(&e, MATRIX_FROM_ANGLES)[0];
        assert_eq!(
            matrix[1..],
            [0.3f32.to_bits(), 0.1f32.to_bits(), 0.2f32.to_bits()]
        );
        assert_eq!(
            calls(&e, FIXED_STRING_CONSTRUCT)[0][1],
            TEXT_SPECIAL_IDLE_PROJECTILE
        );
        assert_eq!(
            calls(&e, ACTIVATE_SEQUENCE),
            vec![vec![0x8800, 0x8900, 0, 0, 1.0f32.to_bits(), 0, 0]]
        );
        assert_eq!(
            calls(&e, SEQUENCE_SET_FLOAT),
            vec![vec![0x8900, (-f32::MAX).to_bits()]]
        );
        let time = calls(&e, UPDATE_DATA_INIT)[0][1];
        assert_eq!(f32::from_bits(time), (0.6f32 as f64 - 0.1) as f32);
        assert_eq!(calls(&e, NODE_UPDATE)[0][0], node);
        // Without a manager, nothing after the selective update.
        e.register(CONTROLLER_MANAGER_OF, |_, _| 0u32.into_ret());
        e.call(0x0080_fcf0, &args![this]);
        assert_eq!(calls(&e, FIND_SEQUENCE).len(), 1);
        // Without a node, nothing at all.
        let (other, _) = projectile(&mut e, false);
        e.call(0x0080_fcf0, &args![other]);
        assert_eq!(calls(&e, NODE_SET_SCALE).len(), 2);
    }

    /// Doubles for the controller setup: the controller at 0x6600 with no object, in the
    /// world 0x5500 of the cell 0x5400.
    fn controller_rig(e: &mut Engine, existing: bool, old_world: u32) -> u32 {
        let (this, _) = projectile(e, true);
        let busy_object = object(e, 1, 0x10);
        e.mem.set_u32(this + 0x68, busy_object);
        e.register(OBJECT_BUSY, |_, _| 0u32.into_ret());
        let controller: AbiFn = if existing {
            |_, _| 0x6600u32.into_ret()
        } else {
            |_, _| 0u32.into_ret()
        };
        e.register(GET_CHAR_CONTROLLER, controller);
        e.register(PARENT_CELL, |_, _| 0x5400u32.into_ret());
        e.register(CELL_WATER_HEIGHT, |_, _| st0(-12.0));
        e.register_double(CONTROLLER_WORLD, move |_, _| old_world.into_ret());
        e.register(READ_FIRST_WORD, |_, _| 0u32.into_ret());
        e.register(WORLD_FLAG, |_, _| 0u32.into_ret());
        e.register(FILTER_LAYER, |_, _| 0x31u32.into_ret());
        let world = object(e, 2, 0x10);
        e.register_double(CELL_HAVOK_WORLD, move |_, _| world.into_ret());
        quiet(
            e,
            &[
                CONTROLLER_SET_FLOAT,
                CONTROLLER_SET_WORLD,
                CONTROLLER_SET_NODE,
                WORLD_SET_MOTION,
                WORLD_REMOVE_OBJECTS,
                CONTROLLER_SET_POSITION,
                CONTROLLER_FILTER,
                slot(2, 0xd0),
            ],
        );
        this
    }

    #[test]
    fn controller_setup_with_an_existing_controller() {
        let mut e = engine();
        let this = controller_rig(&mut e, true, 0x1111);
        e.register(CONTROLLER_SET_POSITION, |e, a| {
            assert_eq!(e.mem.f32(a[1] + 8), 11.0);
            Ret::default()
        });
        e.call(0x0080_feb0, &args![this]);
        assert_eq!(
            calls(&e, CONTROLLER_SET_FLOAT),
            vec![vec![0x6600, (-12.0f32).to_bits()]]
        );
        let node = calls(&e, slot(0, 0x1d0)).len();
        assert!(node > 0);
        assert_eq!(calls(&e, CONTROLLER_SET_NODE).len(), 1);
        assert_eq!(calls(&e, WORLD_SET_MOTION)[0][1..], [4, 1, 0, 1]);
        assert_eq!(calls(&e, WORLD_REMOVE_OBJECTS)[0][1..], [1, 0]);
        let add = &calls(&e, slot(2, 0xd0))[0];
        assert_eq!(add[2..], [1, 0, 0x31, 0]);
        assert!(calls(&e, CONTROLLER_INFO_CONSTRUCT).is_empty());
        // A busy object stops everything.
        e.register(OBJECT_BUSY, |_, _| 1u32.into_ret());
        e.call(0x0080_feb0, &args![this]);
        assert_eq!(calls(&e, CONTROLLER_SET_FLOAT).len(), 1);
    }

    #[test]
    fn controller_setup_builds_one_when_missing() {
        let mut e = engine();
        let this = controller_rig(&mut e, false, 0);
        e.register(SAVE_LOAD_FLAG, |_, _| 1u32.into_ret());
        e.register(ALLOCATE_OBJECT, |_, _| 0x0aa0u32.into_ret());
        e.register(SPHERE_SHAPE_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(ALLOCATE_CONTROLLER, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CHARACTER_CONTROLLER_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(CONTROLLER_FLAG_SOURCE, |_, _| 1u32.into_ret());
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                CONTROLLER_INFO_CONSTRUCT,
                CONTROLLER_INFO_SET_NODE,
                CONTROLLER_INFO_SET_WORLD,
                CONTROLLER_INFO_SET_FLOAT,
                CONTROLLER_INFO_DESTRUCT,
                VECTOR_SCALE_COPY,
                INFO_FILTER_SET,
                POINTER_ASSIGN,
                FLAG_SET_OR_CLEAR,
                CHARACTER_CONTROLLER_CALL,
                slot(1, 0x290),
            ],
        );
        e.call(0x0080_feb0, &args![this]);
        let info = calls(&e, CONTROLLER_INFO_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, CONTROLLER_INFO_CONSTRUCT)[0][1], 1);
        assert_eq!(e.mem.u32(info + 0x8c), 1.0f32.to_bits());
        assert_eq!(
            calls(&e, SPHERE_SHAPE_CONSTRUCTOR),
            vec![vec![0x0aa0, 1.0f32.to_bits(), 1]]
        );
        assert_eq!(calls(&e, INFO_FILTER_SET)[0][1..], [7, 0, 0]);
        assert_eq!(e.mem.u32(info + 0x9c), 6);
        let made = calls(&e, CHARACTER_CONTROLLER_CONSTRUCTOR)[0][0];
        assert_eq!(
            calls(&e, CHARACTER_CONTROLLER_CONSTRUCTOR)[0][1..],
            [info, this]
        );
        assert_eq!(e.mem.f32(made + 0x548), 1.0);
        assert_eq!(calls(&e, slot(1, 0x290))[0][1], made);
        assert_eq!(
            calls(&e, CHARACTER_CONTROLLER_CALL),
            vec![vec![made, 1000, this]]
        );
        assert_eq!(calls(&e, CONTROLLER_INFO_DESTRUCT), vec![vec![info]]);
        // The new controller is then placed in the world.
        assert_eq!(calls(&e, CONTROLLER_SET_WORLD)[0][0], made);
        assert!(calls(&e, WORLD_REMOVE_OBJECTS).is_empty());
    }

    /// A flying-state rig.
    fn update_rig(e: &mut Engine) -> u32 {
        let (this, _) = projectile(e, true);
        e.register(REFERENCE_FLAG, |_, _| 0u32.into_ret());
        e.register(GET_CHAR_CONTROLLER, |e, _| {
            let controller = e.mem.alloc(0x600);
            controller.into_ret()
        });
        e.register(HEIGHT_OFFSET, |_, _| st0(5.0));
        e.register(slot(0, 0x30c), |_, _| 1u32.into_ret());
        e.register(POINT_DIFFERENCE, |_, a| a[1].into_ret());
        e.register(POINT_LENGTH, |_, _| st0(30.0));
        settings(e, &[(DISTANCE_SETTING, 50.0)]);
        quiet(
            e,
            &[
                slot(0, 0xc4),
                slot(0, 0x304),
                slot(0, 0x250),
                POINT_SET,
                NODE_SET_TRANSLATE,
                MAGIC_LIGHT_UPDATE,
            ],
        );
        this
    }

    #[test]
    fn update_flies_and_stops_beyond_the_distance() {
        let mut e = engine();
        let this = update_rig(&mut e);
        e.mem.set_f32(this + 0x88, 10.0);
        e.mem.set_f32(this + 0x8c, 25.0);
        e.mem.set_u32(this + 0xc0, 0x4400);
        e.call(0x0081_06d0, &args![this, 0.5f32]);
        assert_eq!(e.mem.u32(this + 0xac), 1);
        // z = 3 is below the limit 5: slot +0x304 with the position.
        assert_eq!(
            calls(&e, slot(0, 0x304)),
            vec![vec![
                this,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                0,
                0,
                1
            ]]
        );
        assert_eq!(calls(&e, POINT_SET)[0][1..], [0, 5.0f32.to_bits(), 0]);
        assert_eq!(calls(&e, slot(0, 0x250))[0][1], 0.5f32.to_bits());
        assert_eq!(calls(&e, slot(0, 0x250))[0][3], 0xf);
        assert_eq!(e.mem.f32(this + 0x8c), 55.0);
        assert_eq!(calls(&e, slot(0, 0xc4)), vec![vec![this, 1]]);
        assert_eq!(
            calls(&e, MAGIC_LIGHT_UPDATE),
            vec![vec![0x4400, 0.5f32.to_bits()]]
        );
        // A negative time does nothing; 60 or more runs slot +0xC4 first.
        e.call(0x0081_06d0, &args![this, -1.0f32]);
        assert_eq!(calls(&e, slot(0, 0x250)).len(), 1);
        e.mem.set_f32(this + 0x8c, 0.0);
        e.call(0x0081_06d0, &args![this, 60.0f32]);
        assert_eq!(calls(&e, slot(0, 0xc4)).len(), 2);
    }

    #[test]
    fn update_in_the_exploding_state_ends_with_the_sequence() {
        let mut e = engine();
        let (this, _) = projectile(&mut e, true);
        e.mem.set_u32(this + 0xac, 2);
        quiet(
            &mut e,
            &[
                slot(0, 0xc4),
                NODE_CONTROLLER_SOURCE,
                FIXED_STRING_DESTRUCT,
                DEACTIVATE_ALL,
                FORM_SET_DISABLED,
            ],
        );
        e.register(CONTROLLER_MANAGER_OF, |_, _| 0x8800u32.into_ret());
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FIND_SEQUENCE, |_, _| 0x8900u32.into_ret());
        e.register(SEQUENCE_FIRST_FLOAT, |_, _| st0(2.0));
        e.register(SEQUENCE_SECOND_FLOAT, |_, _| st0(3.0));
        e.register(SEQUENCE_WORD, |_, _| 1u32.into_ret());
        e.call(0x0081_06d0, &args![this, 0.5f32]);
        assert!(calls(&e, FORM_SET_DISABLED).is_empty());
        e.register(SEQUENCE_WORD, |_, _| 0u32.into_ret());
        e.call(0x0081_06d0, &args![this, 0.5f32]);
        assert_eq!(calls(&e, FORM_SET_DISABLED), vec![vec![this, 1]]);
        assert_eq!(calls(&e, DEACTIVATE_ALL), vec![vec![0x8800, 0]]);
        assert_eq!(calls(&e, slot(0, 0xc4)), vec![vec![this, 1]]);
        // Without a manager: the slot, then the disabling.
        e.register(CONTROLLER_MANAGER_OF, |_, _| 0u32.into_ret());
        e.call(0x0081_06d0, &args![this, 0.5f32]);
        assert_eq!(calls(&e, slot(0, 0xc4)).len(), 2);
        assert_eq!(calls(&e, FORM_SET_DISABLED).len(), 2);
    }

    #[test]
    fn explode_scales_moves_and_finds_targets() {
        let mut e = engine();
        let (this, _) = projectile(&mut e, true);
        settings(
            &mut e,
            &[
                (AREA_SCALE, 0.5),
                (AREA_SCALE_MAX, 4.0),
                (AREA_SCALE_MIN, 1.0),
            ],
        );
        e.register(PROJECTILE_EFFECT_ITEM, |_, _| 0x2200u32.into_ret());
        e.register(EFFECT_ITEM_GET_AREA, |_, _| 20u32.into_ret());
        e.register(NODE_TRANSFORM, |_, a| (a[0] + 0x68).into_ret());
        e.register(TRANSFORM_APPLY, |e, a| {
            e.mem.set_f32(a[1] + 8, 9.0);
            a[1].into_ret()
        });
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[0].into_ret());
        let node_object = object(&mut e, 3, 0x10);
        e.register_double(slot(0, 0x1d0), move |_, _| node_object.into_ret());
        e.register(slot(3, 0x9c), |_, _| 0x9c00u32.into_ret());
        e.register(SOUND_HANDLE_IS_VALID, |_, _| 0u32.into_ret());
        e.register(GET_CHAR_CONTROLLER, |_, _| 0u32.into_ret());
        e.register(PARENT_CELL, |_, _| 0x5400u32.into_ret());
        e.register(PROJECTILE_CASTER, |_, _| 0x3300u32.into_ret());
        e.register(CONTROLLER_MANAGER_OF, |_, _| 0u32.into_ret());
        e.register(PROJECTILE_IMPACT_SOUND, |_, _| 0u32.into_ret());
        quiet(
            &mut e,
            &[
                SET_LOCATION_ON_REFERENCE,
                PROJECTILE_EXPLODE_AT,
                NODE_SET_TRANSLATE,
                FIXED_STRING_DESTRUCT,
                NODE_SET_SCALE,
                FIND_TARGETS_OV2,
                NODE_CONTROLLER_SOURCE,
                BASE_FORM,
            ],
        );
        e.mem.set_u32(this + 0x98, 0x1d);
        e.call(
            0x0081_0aa0,
            &args![this, 1.0f32, 2.0f32, 3.0f32, 0x66u32, 0u32, 0u8],
        );
        // 20 * 0.5 = 10, lowered to the maximum 4.
        assert_eq!(e.mem.f32(this + 0xb0), 4.0);
        let at = &calls(&e, PROJECTILE_EXPLODE_AT)[0];
        assert_eq!(
            at[1..4],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert_eq!(at[6], 9.0f32.to_bits());
        assert_eq!(at[7], 0x66);
        assert_eq!(calls(&e, TRANSFORM_APPLY)[0][2], EXPLOSION_OFFSET);
        assert_eq!(
            calls(&e, NODE_SET_SCALE),
            vec![vec![0x9c00, 4.0f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(this + 0xac), 2);
        let find = &calls(&e, FIND_TARGETS_OV2)[0];
        assert_eq!(find[..3], [0x3300, 0x1d, 0x5400]);
        assert_eq!(find[6..], [this, 0, 0, 1.0f32.to_bits(), 1.0f32.to_bits()]);
        // With the flag: no move, no targets; the area is raised to the minimum.
        e.register(EFFECT_ITEM_GET_AREA, |_, _| 1u32.into_ret());
        e.call(
            0x0081_0aa0,
            &args![this, 1.0f32, 2.0f32, 3.0f32, 0x66u32, 0u32, 1u8],
        );
        assert_eq!(e.mem.f32(this + 0xb0), 1.0);
        assert_eq!(calls(&e, FIND_TARGETS_OV2).len(), 1);
        assert_eq!(calls(&e, NODE_SET_TRANSLATE).len(), 1);
        // An actor whose 008ace90 is true stops it all.
        let actor = object(&mut e, 4, 0x10);
        e.register(slot(4, 0x100), |_, _| 1u32.into_ret());
        e.register(ACTOR_STOPS_EXPLOSION, |_, _| 1u32.into_ret());
        e.call(
            0x0081_0aa0,
            &args![this, 1.0f32, 2.0f32, 3.0f32, 0x66u32, actor, 0u8],
        );
        assert_eq!(calls(&e, PROJECTILE_EFFECT_ITEM).len(), 2);
    }

    #[test]
    fn init_load_game_remakes_the_light_from_the_saved_float() {
        let mut e = engine();
        let (this, node) = projectile(&mut e, true);
        e.mem.set_u32(this + 0xac, 2);
        e.mem.set_f32(this + 0xc0, 0.25);
        e.mem.set_f32(this + 0xb0, 3.0);
        quiet(
            &mut e,
            &[
                MAGIC_PROJECTILE_INIT_LOAD_GAME,
                CELL_ADD_REFERENCE,
                NODE_SET_SCALE,
                NODE_SET_TRANSLATE,
                MATRIX_FROM_ANGLES,
                NODE_SET_ROTATE,
                PROCESS_SELECTIVE_UPDATE,
                NODE_CONTROLLER_SOURCE,
                FIXED_STRING_DESTRUCT,
                MAGIC_LIGHT_FADE,
                MAGIC_LIGHT_SET_FLOAT,
                BASE_FORM,
                FORM_ID,
            ],
        );
        e.register(ANGLES, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(MATRIX_SELF, |_, a| a[0].into_ret());
        e.register(PARENT_CELL, |_, _| 0x5400u32.into_ret());
        e.register(COPY_MEMORY, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        e.register(PROJECTILE_LIGHT, |_, _| 0x4100u32.into_ret());
        e.register(ALLOCATE, |_, _| 0x4400u32.into_ret());
        e.register(MAGIC_LIGHT_CONSTRUCTOR, |_, a| a[0].into_ret());
        // Init3D finds no manager; the light's fade finds one.
        e.register_double(CONTROLLER_MANAGER_OF, {
            let mut n = 0;
            move |_, _| {
                n += 1;
                (if n == 1 { 0u32 } else { 0x8800 }).into_ret()
            }
        });
        e.register(FIXED_STRING_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FIND_SEQUENCE, |_, _| 0x8900u32.into_ret());
        e.register(SEQUENCE_SECOND_FLOAT, |_, _| st0(4.0));
        e.call(0x0081_12c0, &args![this, 0x77u32, 3u32]);
        assert_eq!(
            calls(&e, MAGIC_PROJECTILE_INIT_LOAD_GAME),
            vec![vec![this, 0x77, 3]]
        );
        assert_eq!(calls(&e, MAGIC_LIGHT_CONSTRUCTOR)[0][2], node);
        assert_eq!(e.mem.u32(this + 0xc0), 0x4400);
        assert_eq!(
            calls(&e, MAGIC_LIGHT_FADE),
            vec![vec![0x4400, 0, 3.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, MAGIC_LIGHT_SET_FLOAT),
            vec![vec![0x4400, 0.25f32.to_bits()]]
        );
        assert!(calls(&e, PROJECTILE_FLIGHT_SOUND).is_empty());
        assert_eq!(
            calls(&e, NODE_SET_SCALE).last(),
            Some(&vec![node, 3.0f32.to_bits()])
        );
    }

    #[test]
    fn finish_init_load_game_sets_the_collision_filter() {
        let mut e = engine();
        let (this, _) = projectile(&mut e, true);
        let caster = object(&mut e, 5, 0x10);
        let reference = object(&mut e, 6, 0x10);
        e.register_double(PROJECTILE_CASTER, move |_, _| caster.into_ret());
        e.register_double(slot(5, 0x2c), move |_, _| reference.into_ret());
        e.register(slot(6, 0x100), |_, _| 1u32.into_ret());
        e.register(ACTOR_COLLISION_INFO, |_, a| a[1].into_ret());
        e.register(FILTER_LAYER, |_, _| 0x2bu32.into_ret());
        e.register(GET_CHAR_CONTROLLER, |_, _| 0x6600u32.into_ret());
        e.register(CONTROLLER_FILTER, |e, a| {
            e.mem.set_u32(a[1], 0x1200);
            Ret::default()
        });
        e.register(READ_FIRST_WORD, |_, _| 0x9100u32.into_ret());
        settings(&mut e, &[(COUNT_SETTING, 2.0)]);
        quiet(
            &mut e,
            &[
                MOBILE_OBJECT_FINISH_INIT_LOAD_GAME,
                FILTER_SET_LAYER,
                FILTER_SET_GROUP,
                OBJECT_SET_FILTER,
            ],
        );
        e.call(0x0081_15c0, &args![this, 0x77u32, 3u32]);
        assert_eq!(calls(&e, FILTER_SET_LAYER)[0][1], 7);
        assert_eq!(calls(&e, FILTER_SET_GROUP)[0][1], 0x2b);
        assert_eq!(calls(&e, OBJECT_SET_FILTER), vec![vec![0x9100, 0x1200]]);
        assert_eq!(e.mem.f32(RUNNING_TOTAL), 2.0);
        // The player's projectiles are not counted; a node layer without a caster.
        e.mem.set_u32(PLAYER, reference);
        e.call(0x0081_15c0, &args![this, 0x77u32, 3u32]);
        assert_eq!(e.mem.f32(RUNNING_TOTAL), 2.0);
        e.register(PROJECTILE_CASTER, |_, _| 0u32.into_ret());
        e.register(FIND_FIRST_COLLISION_OBJECT, |_, _| 0u32.into_ret());
        e.register(DEFAULT_LAYER, |_, _| 0x3cu32.into_ret());
        e.call(0x0081_15c0, &args![this, 0x77u32, 3u32]);
        assert_eq!(calls(&e, FILTER_SET_GROUP).last().unwrap()[1], 0x3c);
    }
}

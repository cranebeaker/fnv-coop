//! `fallout/magic/nighteyeeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ValueModifierEffect::Update` (engine map, `0082c660`, `thiscall`, one
/// `float`), `ValueModifierEffect::Start` (`0082c000`) and
/// `ValueModifierEffect::Finish` (`0082c810`, both `thiscall`).
const VALUE_MODIFIER_UPDATE: u32 = 0x0082_c660;
const VALUE_MODIFIER_START: u32 = 0x0082_c000;
const VALUE_MODIFIER_FINISH: u32 = 0x0082_c810;
/// The `ValueModifierEffect` constructor body `0082ba60` (`thiscall`, three
/// words: caster, magic item, effect item).
const VALUE_MODIFIER_CONSTRUCT: u32 = 0x0082_ba60;
/// `NightEyeEffect`'s vtable (`0107bd7c`, RTTI `.?AVNightEyeEffect@@`).
const NIGHT_EYE_EFFECT_VTABLE: u32 = 0x0107_bd7c;
/// `operator new` (`00401000`, `cdecl`: the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00804cb0` (`thiscall`, one `float`; its body does nothing).
const EFFECT_SET_00804CB0: u32 = 0x0080_4cb0;
/// `0059bb30` (`thiscall`): the effect's magic target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// The player (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `ActiveEffect::InitLoadGame` (engine map, `00805670`, `thiscall`, one
/// word).
const ACTIVE_EFFECT_INIT_LOAD_GAME: u32 = 0x0080_5670;
/// `004534f0` (`thiscall`, one word; its body does nothing).
const REVERT_BASE_004534F0: u32 = 0x0045_34f0;
/// The save/load game object (`011de45c`) and `0055f5b0` on it (`thiscall`,
/// returns a byte).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const SAVE_LOAD_TEST_0055F5B0: u32 = 0x0055_f5b0;
/// `004e3270` (no arguments: the word at `011f91ac`) and `004ebbc0`
/// (`thiscall`, one word: an index) give the image-space modifier the
/// visual effect switches.
const MODIFIERS_004E3270: u32 = 0x004e_3270;
const MODIFIER_AT_004EBBC0: u32 = 0x004e_bbc0;
/// The player's +0xA4 (an object whose slot +8 the code calls with 0x32).
const PLAYER_VALUES: u32 = 0xa4;
/// `00403e20` (`thiscall`: the address of a setting's value) on the setting
/// `fMagicNightEyeAmbient` (`011d1668`).
const SETTING_VALUE: u32 = 0x0040_3e20;
const NIGHT_EYE_AMBIENT: u32 = 0x011d_1668;
/// The `float` global the visual effect stores (`011f947c`).
const NIGHT_EYE_LEVEL: u32 = 0x011f_947c;

// Translated from 0080cea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NightEyeEffect::Update` (engine map; `thiscall`, `RET 4`: the elapsed
/// seconds): `ValueModifierEffect::Update` (`0082c660`) with the seconds
/// (copied through `FLD`/`FSTP`).
pub fn night_eye_effect_update(e: &mut Engine, this: Ptr, seconds: f32) {
    let seconds = x87_quiet_bits(seconds.to_bits());
    e.call(VALUE_MODIFIER_UPDATE, &args![this, seconds]);
}

// Translated from 00825710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NightEyeEffect`'s constructor (`thiscall`, `RET 0xC`: caster, magic
/// item, effect item): the `ValueModifierEffect` constructor (`0082ba60`)
/// with the three words, then `NightEyeEffect`'s vtable. Returns `this`.
pub fn fn_00825710(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(
        VALUE_MODIFIER_CONSTRUCT,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), NIGHT_EYE_EFFECT_VTABLE);
    this
}

// Translated from 00825760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NightEyeEffect::Copy` (engine map; `thiscall`): a new 0x4C-byte
/// `NightEyeEffect` (`00401000`, then `00825710` with the caster +0x28, the
/// magic item +8 and the effect item +0xC, or 0 when the allocation fails),
/// handed to `this`'s slot +0x44 (`CopyData`). Returns the copy.
// Translation incomplete: 00825760 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
pub fn night_eye_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    let copy = if block != 0 {
        let base = this.addr();
        let caster = e.mem.u32(base.wrapping_add(0x28));
        let spell = e.mem.u32(base.wrapping_add(8));
        let effect = e.mem.u32(base.wrapping_add(0xc));
        fn_00825710(e, Ptr::new(block), caster, spell, effect).addr()
    } else {
        0
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

/// Whether the effect's magic target answers the player through its slot
/// +4 (`NightEyeEffect::Start` and `Finish`).
fn targets_the_player(e: &mut Engine, this: Ptr) -> bool {
    let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
    let reference = e.vcall(target, 4, &[]).u32();
    reference == e.mem.u32(PLAYER)
}

// Translated from 00825810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NightEyeEffect::Start` (engine map; `thiscall`): `00804cb0` with 1.0,
/// `ValueModifierEffect::Start`, then the visual effect (`00825990`) when
/// the magic target's slot +4 answers the player.
pub fn night_eye_effect_start(e: &mut Engine, this: Ptr) {
    e.call(EFFECT_SET_00804CB0, &args![this, 1.0f32.to_bits()]);
    e.call(VALUE_MODIFIER_START, &args![this]);
    if targets_the_player(e, this) {
        night_eye_effect_apply_visual_effect(e);
    }
}

// Translated from 00825860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NightEyeEffect::Finish` (engine map; `thiscall`):
/// `ValueModifierEffect::Finish`, then the visual effect (`00825990`) when
/// the magic target's slot +4 answers the player.
pub fn night_eye_effect_finish(e: &mut Engine, this: Ptr) {
    e.call(VALUE_MODIFIER_FINISH, &args![this]);
    if targets_the_player(e, this) {
        night_eye_effect_apply_visual_effect(e);
    }
}

// Translated from 008258a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, three words: caster, magic item, effect item): a new
/// 0x4C-byte `NightEyeEffect` (`00401000`, then `00825710`), or 0 when the
/// allocation fails.
// Translation incomplete: 008258a0 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
pub fn fn_008258a0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    if block != 0 {
        return fn_00825710(e, Ptr::new(block), caster, spell, effect).addr();
    }
    0
}

// Translated from 00825920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NightEyeEffect::InitLoadGame` (engine map; `thiscall`, `RET 4`: the
/// target reference): `ActiveEffect::InitLoadGame` (`00805670`), then the
/// visual effect (`00825990`) when the reference is the player.
pub fn night_eye_effect_init_load_game(e: &mut Engine, this: Ptr, reference: u32) {
    e.call(ACTIVE_EFFECT_INIT_LOAD_GAME, &args![this, reference]);
    if reference == e.mem.u32(PLAYER) {
        night_eye_effect_apply_visual_effect(e);
    }
}

// Translated from 00825950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NightEyeEffect::Revert` (engine map; `thiscall`, `RET 4`: the target
/// reference): `004534f0` with the reference, then the visual effect
/// (`00825990`) when `0055f5b0` on the save/load game object (`011de45c`)
/// answers true and the reference is the player.
pub fn night_eye_effect_revert(e: &mut Engine, this: Ptr, reference: u32) {
    e.call(REVERT_BASE_004534F0, &args![this, reference]);
    let save_load = e.mem.u32(SAVE_LOAD_GAME);
    if e.call(SAVE_LOAD_TEST_0055F5B0, &args![save_load]).bool() && reference == e.mem.u32(PLAYER) {
        night_eye_effect_apply_visual_effect(e);
    }
}

// Translated from 00825990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NightEyeEffect::ApplyVisualEffect` (engine map; no arguments): takes
/// the image-space modifier 3 (`004e3270`, `004ebbc0`); when the player's
/// +0xA4 object answers more than 0 through its slot +8 for 0x32, stores
/// `fMagicNightEyeAmbient` in `011f947c` and calls the modifier's slot
/// +0x2C with 1, otherwise stores 0.0 and calls it with 0.
pub fn night_eye_effect_apply_visual_effect(e: &mut Engine) {
    let modifiers = e.call(MODIFIERS_004E3270, &[]).u32();
    let modifier = e.call(MODIFIER_AT_004EBBC0, &args![modifiers, 3u32]).u32();
    let values = e.mem.u32(PLAYER).wrapping_add(PLAYER_VALUES);
    let level = e.vcall(values, 8, &args![0x32u32]).i32();
    if level > 0 {
        let ambient = e.call(SETTING_VALUE, &args![NIGHT_EYE_AMBIENT]).u32();
        let ambient = x87_quiet_bits(e.mem.u32(ambient));
        e.mem.set_u32(NIGHT_EYE_LEVEL, ambient);
        e.vcall(modifier, 0x2c, &args![1u32]);
    } else {
        e.mem.set_u32(NIGHT_EYE_LEVEL, 0);
        e.vcall(modifier, 0x2c, &args![0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_cea0, night_eye_effect_update(Ptr, f32)),
        entry!(0x0082_5710, fn_00825710(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0082_5760, night_eye_effect_copy(Ptr) -> u32),
        entry!(0x0082_5810, night_eye_effect_start(Ptr)),
        entry!(0x0082_5860, night_eye_effect_finish(Ptr)),
        entry!(0x0082_58a0, fn_008258a0(u32, u32, u32) -> u32),
        entry!(0x0082_5920, night_eye_effect_init_load_game(Ptr, u32)),
        entry!(0x0082_5950, night_eye_effect_revert(Ptr, u32)),
        entry!(0x0082_5990, night_eye_effect_apply_visual_effect()),
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

    /// The player at 0x7000 with a +0xA4 object answering `level` through
    /// its slot +8 (code `0x00b0_0008`), and a modifier whose slot +0x2C is
    /// `0x00b0_002c`.
    fn visual_world(e: &mut Engine, level: u32) -> u32 {
        e.map(PLAYER, 4);
        let player = e.mem.alloc(0xa8);
        e.mem.set_u32(PLAYER, player);
        e.put_vtable(0x00b1_0000, &[0, 0, 0x00b0_0008]);
        e.mem.set_u32(player + PLAYER_VALUES, 0x00b1_0000);
        e.register_double(0x00b0_0008, move |_, _| ret(level));
        let modifier = e.mem.alloc(4);
        e.put_vtable(0x00b2_0000, &[0; 12]);
        e.mem.set_u32(0x00b2_0000 + 0x2c, 0x00b0_002c);
        e.mem.set_u32(modifier, 0x00b2_0000);
        e.register(0x00b0_002c, |_, _| Ret::default());
        e.register(MODIFIERS_004E3270, |_, _| ret(0x5150));
        e.register_double(MODIFIER_AT_004EBBC0, move |_, _| ret(modifier));
        let ambient = e.mem.alloc(4);
        e.mem.set_u32(ambient, 0.25f32.to_bits());
        e.register_double(SETTING_VALUE, move |_, _| ret(ambient));
        e.map(NIGHT_EYE_LEVEL, 4);
        e.mem.set_u32(NIGHT_EYE_LEVEL, 0xdead);
        modifier
    }

    #[test]
    fn update_forwards_the_seconds() {
        let mut e = Engine::new();
        zero(&mut e, &[VALUE_MODIFIER_UPDATE]);
        let (_, log) = run(&mut e, 0x0080_cea0, &args![0x40u32, 0.5f32.to_bits()]);
        assert_eq!(
            calls(&log, VALUE_MODIFIER_UPDATE),
            vec![vec![0x40, 0.5f32.to_bits()]]
        );
        // A signalling NaN arrives quiet.
        let (_, log) = run(&mut e, 0x0080_cea0, &args![0x40u32, 0x7f80_0001u32]);
        assert_eq!(calls(&log, VALUE_MODIFIER_UPDATE)[0][1], 0x7fc0_0001);
    }

    #[test]
    fn constructor_and_copy() {
        let mut e = Engine::new();
        zero(&mut e, &[VALUE_MODIFIER_CONSTRUCT, 0x00b0_0044]);
        let block = e.mem.alloc(0x4c);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        let this = e.mem.alloc(0x4c);
        e.put_vtable(0x00b3_0000, &[0; 0x12]);
        e.mem.set_u32(0x00b3_0000 + 0x44, 0x00b0_0044);
        e.mem.set_u32(this, 0x00b3_0000);
        e.mem.set_u32(this + 0x28, 0xca);
        e.mem.set_u32(this + 8, 0x5b);
        e.mem.set_u32(this + 0xc, 0xef);
        let (r, log) = run(&mut e, 0x0082_5760, &args![this]);
        assert_eq!(r.u32(), block);
        assert_eq!(e.mem.u32(block), NIGHT_EYE_EFFECT_VTABLE);
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![0x4c]]);
        assert_eq!(
            calls(&log, VALUE_MODIFIER_CONSTRUCT),
            vec![vec![block, 0xca, 0x5b, 0xef]]
        );
        assert_eq!(calls(&log, 0x00b0_0044), vec![vec![this, block]]);
        // A failed allocation hands over 0.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        let (r, log) = run(&mut e, 0x0082_5760, &args![this]);
        assert_eq!(r.u32(), 0);
        assert!(calls(&log, VALUE_MODIFIER_CONSTRUCT).is_empty());
        assert_eq!(calls(&log, 0x00b0_0044), vec![vec![this, 0]]);
        // The cdecl maker.
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        let (r, log) = run(&mut e, 0x0082_58a0, &args![1u32, 2u32, 3u32]);
        assert_eq!(r.u32(), block);
        assert_eq!(
            calls(&log, VALUE_MODIFIER_CONSTRUCT),
            vec![vec![block, 1, 2, 3]]
        );
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x0082_58a0, &args![1u32, 2u32, 3u32]).u32(), 0);
    }

    #[test]
    fn visual_effect_switches_on_the_players_level() {
        let mut e = Engine::new();
        let modifier = visual_world(&mut e, 3);
        let (_, log) = run(&mut e, 0x0082_5990, &[]);
        assert_eq!(calls(&log, MODIFIER_AT_004EBBC0), vec![vec![0x5150, 3]]);
        let player = e.mem.u32(PLAYER);
        assert_eq!(calls(&log, 0x00b0_0008), vec![vec![player + 0xa4, 0x32]]);
        assert_eq!(calls(&log, SETTING_VALUE), vec![vec![NIGHT_EYE_AMBIENT]]);
        assert_eq!(e.mem.u32(NIGHT_EYE_LEVEL), 0.25f32.to_bits());
        assert_eq!(calls(&log, 0x00b0_002c), vec![vec![modifier, 1]]);
        let mut e = Engine::new();
        let modifier = visual_world(&mut e, 0xffff_ffff);
        let (_, log) = run(&mut e, 0x0082_5990, &[]);
        assert_eq!(e.mem.u32(NIGHT_EYE_LEVEL), 0);
        assert_eq!(calls(&log, 0x00b0_002c), vec![vec![modifier, 0]]);
    }

    /// An effect whose magic target answers `reference` through slot +4.
    fn effect_with_target(e: &mut Engine, reference: u32) -> u32 {
        let target = e.mem.alloc(4);
        e.put_vtable(0x00b4_0000, &[0, 0x00b0_0004]);
        e.mem.set_u32(target, 0x00b4_0000);
        e.register_double(0x00b0_0004, move |_, _| ret(reference));
        e.register_double(EFFECT_TARGET_0059BB30, move |_, _| ret(target));
        e.mem.alloc(0x4c)
    }

    #[test]
    fn start_and_finish_apply_for_the_player() {
        let mut e = Engine::new();
        visual_world(&mut e, 1);
        let player = e.mem.u32(PLAYER);
        zero(
            &mut e,
            &[
                EFFECT_SET_00804CB0,
                VALUE_MODIFIER_START,
                VALUE_MODIFIER_FINISH,
            ],
        );
        let this = effect_with_target(&mut e, player);
        let (_, log) = run(&mut e, 0x0082_5810, &args![this]);
        assert_eq!(
            calls(&log, EFFECT_SET_00804CB0),
            vec![vec![this, 1.0f32.to_bits()]]
        );
        assert_eq!(calls(&log, VALUE_MODIFIER_START), vec![vec![this]]);
        assert_eq!(calls(&log, MODIFIERS_004E3270).len(), 1);
        let (_, log) = run(&mut e, 0x0082_5860, &args![this]);
        assert_eq!(calls(&log, VALUE_MODIFIER_FINISH), vec![vec![this]]);
        assert_eq!(calls(&log, MODIFIERS_004E3270).len(), 1);
        let this = effect_with_target(&mut e, player + 4);
        let (_, log) = run(&mut e, 0x0082_5810, &args![this]);
        assert!(calls(&log, MODIFIERS_004E3270).is_empty());
        let (_, log) = run(&mut e, 0x0082_5860, &args![this]);
        assert!(calls(&log, MODIFIERS_004E3270).is_empty());
    }

    #[test]
    fn init_load_game_and_revert_apply_for_the_player() {
        let mut e = Engine::new();
        visual_world(&mut e, 1);
        let player = e.mem.u32(PLAYER);
        zero(
            &mut e,
            &[ACTIVE_EFFECT_INIT_LOAD_GAME, REVERT_BASE_004534F0],
        );
        let (_, log) = run(&mut e, 0x0082_5920, &args![0x10u32, player]);
        assert_eq!(
            calls(&log, ACTIVE_EFFECT_INIT_LOAD_GAME),
            vec![vec![0x10, player]]
        );
        assert_eq!(calls(&log, MODIFIERS_004E3270).len(), 1);
        let (_, log) = run(&mut e, 0x0082_5920, &args![0x10u32, 0x99u32]);
        assert!(calls(&log, MODIFIERS_004E3270).is_empty());
        e.map(SAVE_LOAD_GAME, 4);
        e.mem.set_u32(SAVE_LOAD_GAME, 0x5a1d);
        e.register(SAVE_LOAD_TEST_0055F5B0, |_, _| ret(0x100));
        let (_, log) = run(&mut e, 0x0082_5950, &args![0x10u32, player]);
        assert_eq!(calls(&log, REVERT_BASE_004534F0), vec![vec![0x10, player]]);
        assert_eq!(calls(&log, SAVE_LOAD_TEST_0055F5B0), vec![vec![0x5a1d]]);
        assert!(calls(&log, MODIFIERS_004E3270).is_empty());
        e.register(SAVE_LOAD_TEST_0055F5B0, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0082_5950, &args![0x10u32, player]);
        assert_eq!(calls(&log, MODIFIERS_004E3270).len(), 1);
        let (_, log) = run(&mut e, 0x0082_5950, &args![0x10u32, 0x99u32]);
        assert!(calls(&log, MODIFIERS_004E3270).is_empty());
    }
}

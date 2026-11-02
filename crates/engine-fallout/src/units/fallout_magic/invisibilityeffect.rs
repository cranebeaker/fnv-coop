//! `fallout/magic/invisibilityeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `InvisibilityEffect` (Xbox PDB, 0x4C bytes) is a `ValueModifierEffect` (its actor value
//! index at +0x48). Its functions fade the target actor out and back in: the player in
//! first person through two of its virtual slots (+0x41C with a `float`, +0x384 with a word
//! and a `float`), everyone else through the high process's `FadeOut` / `FadeIn`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_magic::activeeffect::ActiveEffect;

/// `InvisibilityEffect`'s vtable (`0107a21c`).
const INVISIBILITY_EFFECT_VTABLE: u32 = 0x0107_a21c;
/// The `ValueModifierEffect` constructor body `0082ba60` (`thiscall`, `RET 0xC`: caster,
/// spell, effect item).
const VALUE_MODIFIER_EFFECT_CONSTRUCTOR: u32 = 0x0082_ba60;
/// `00401000`: the game's `operator new` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `0059bb30` (`thiscall`): the effect's magic target (the engine map's name for the folded
/// body, `D3DTexture_LockRect`, does not fit).
const GET_MAGIC_TARGET: u32 = 0x0059_bb30;
/// `MagicTarget::GetTargetAsActor` (engine map, `00822b40`, `thiscall`).
const GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `ValueModifierEffect::Start` (engine map, `0082c000`, `thiscall`).
const VALUE_MODIFIER_EFFECT_START: u32 = 0x0082_c000;
/// `ValueModifierEffect::Finish` (engine map, `0082c810`, `thiscall`).
const VALUE_MODIFIER_EFFECT_FINISH: u32 = 0x0082_c810;
/// `ValueModifierEffect::FinishInitLoadGame` (engine map, `0082cd10`, `thiscall`, `RET 4`).
const VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME: u32 = 0x0082_cd10;
/// `004534f0` (`thiscall`, `RET 4`): the base `Revert` body the effect runs first.
const BASE_REVERT: u32 = 0x0045_34f0;
/// `005b9b00` (`cdecl`, no words; `tesscriptfunctions.cpp`): a `bool` in `AL` that, with the
/// actor being the player, selects the player's own fade slots. What it asks is not
/// confirmed.
const PLAYER_FADE_TEST: u32 = 0x005b_9b00;
/// The player (`PlayerCharacter *`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `008d8520` (`thiscall`; the engine map's name `MiddleHighProcess::GetSavedAcquireObject`
/// is a folded body): the actor's process, cast below from `BaseProcess` to `HighProcess`.
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// `__RTDynamicCast` (`cdecl`: object, 0, source type, target type, 0).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVBaseProcess@@` and `.?AVHighProcess@@`.
const TYPE_BASE_PROCESS: u32 = 0x0118_a6b8;
const TYPE_HIGH_PROCESS: u32 = 0x0118_a6d4;
/// `HighProcess::FadeOut` (engine map, `008fe960`, `thiscall`, `RET 0xC`).
const HIGH_PROCESS_FADE_OUT: u32 = 0x008f_e960;
/// `HighProcess::FadeIn` (engine map, `008fe8f0`, `thiscall`, `RET 8`).
const HIGH_PROCESS_FADE_IN: u32 = 0x008f_e8f0;
/// `Actor::UpdateAlpha` (engine map, `008c4640`, `thiscall`).
const ACTOR_UPDATE_ALPHA: u32 = 0x008c_4640;
/// `0.0` (`double`, `01012060`), the comparison constant.
const ZERO: u32 = 0x0101_2060;
/// The `float` the player's slot +0x384 receives when fading out (`0107a214`; the Xbox PDB
/// has a `const float InvisibilityEffect::REFRACTIONPOWER`, not confirmed to be this one).
const PLAYER_FADE_OUT_VALUE: u32 = 0x0107_a214;
/// `-1.0` (`float`, `01012054`), passed to the hit effect's constructor.
const MINUS_ONE: u32 = 0x0101_2054;
/// `007df1d0` (`thiscall`): the active effect's effect setting.
const EFFECT_SETTING: u32 = 0x007d_f1d0;
/// `0048cee0` (`thiscall`, `ECX` = the effect setting's `TESModel` at +0x18): the length of
/// the model's path (zero when it has none).
const MODEL_PATH_LENGTH: u32 = 0x0048_cee0;
/// `00aa13e0` (`cdecl`, one word: the size): the allocation the hit effect is made in.
const ALLOCATE_TEMP_EFFECT: u32 = 0x00aa_13e0;
/// `MagicModelHitEffect::MagicModelHitEffect_ov2` (engine map, `0081c720`, `thiscall`,
/// `RET 0xC`: actor, model path, `float`).
const MAGIC_MODEL_HIT_EFFECT_CONSTRUCTOR: u32 = 0x0081_c720;
/// `ProcessLists::AddTempEffect` (engine map, `00973fd0`, `thiscall`, `RET 4`) and the
/// process lists it runs on (`011e0e80`).
const ADD_TEMP_EFFECT: u32 = 0x0097_3fd0;
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// `0040df50` and `0040df70` (`cdecl`, no words): two `float` settings' values in `ST0`
/// (`011c36d8` and `011c3664` through `00403e20`).
const SETTING_VALUE_0040DF50: u32 = 0x0040_df50;
const SETTING_VALUE_0040DF70: u32 = 0x0040_df70;
/// `004b3ab0` (`cdecl`, five `float`s `a, b, c, d, x`): `(x - c) / (d - c) * (b - a) + a`
/// in `ST0`.
const REMAP: u32 = 0x004b_3ab0;

fn dynamic_cast(e: &mut Engine, object: u32, from: u32, to: u32) -> u32 {
    e.call(RT_DYNAMIC_CAST, &args![object, 0u32, from, to, 0u32])
        .u32()
}

/// The effect's target as an actor: null without a magic target, otherwise
/// `MagicTarget::GetTargetAsActor` of it (the exe asks `0059bb30` twice).
fn target_actor(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(GET_MAGIC_TARGET, &args![this]).u32() == 0 {
        return 0;
    }
    let target = e.call(GET_MAGIC_TARGET, &args![this]).u32();
    e.call(GET_TARGET_AS_ACTOR, &args![target]).u32()
}

/// The actor's value for the effect's actor value (`ValueModifierEffect` +0x48), through
/// the actor's `ActorValueOwner` (+0xA4) slot +8: an `int`.
fn actor_value(e: &mut Engine, this: Ptr, actor: u32) -> i32 {
    let index = e.mem.u32(this.addr().wrapping_add(0x48));
    e.vcall(actor.wrapping_add(0xa4), 8, &args![index]).i32()
}

/// The actor's high process: `008d8520`, cast from `BaseProcess` to `HighProcess`.
fn high_process(e: &mut Engine, actor: u32) -> u32 {
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    dynamic_cast(e, process, TYPE_BASE_PROCESS, TYPE_HIGH_PROCESS)
}

/// The player's fade-out slots: +0x41C with `1.0`, +0x384 with 1 and the `float` at
/// `0107a214`.
fn player_fade_out(e: &mut Engine, actor: u32) {
    e.vcall(actor, 0x41c, &args![1.0f32]);
    let value = x87_quiet_bits(e.mem.u32(PLAYER_FADE_OUT_VALUE));
    e.vcall(actor, 0x384, &args![1u32, value]);
}

fn is_player_fade(e: &mut Engine, actor: u32) -> bool {
    e.call(PLAYER_FADE_TEST, &[]).bool() && actor == e.mem.u32(PLAYER)
}

// Translated from 0080e030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `InvisibilityEffect` constructor (`thiscall`, `RET 0xC`): the `ValueModifierEffect`
/// body (`0082ba60`) with the caster, spell and effect item, then its own vtable
/// (`0107a21c`). Returns `this`.
pub fn fn_0080e030(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(
        VALUE_MODIFIER_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), INVISIBILITY_EFFECT_VTABLE);
    this
}

// Translated from 0080e060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080e060 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `InvisibilityEffect::Copy` (Xbox PDB): allocates 0x4C bytes (`00401000`), constructs an
/// `InvisibilityEffect` there with this effect's caster (+0x28), spell (+8) and effect item
/// (+0xC) (null when the allocation failed), has this effect's slot +0x44 (`CopyData`) copy
/// itself into it and returns it.
pub fn invisibility_effect_copy(e: &mut Engine, this: Ptr<ActiveEffect>) -> u32 {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    let copy = if block != 0 {
        let caster = e.get(this, ActiveEffect::pCaster);
        let spell = e.get(this, ActiveEffect::pSpell);
        let effect = e.get(this, ActiveEffect::pEffect);
        fn_0080e030(e, Ptr::new(block), caster, spell, effect).addr()
    } else {
        0
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 0080e110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InvisibilityEffect::Start` (Xbox PDB): runs `ValueModifierEffect::Start`; then, for a
/// target actor whose value of the effect's actor value is above `0.0` (the `double` at
/// `01012060`), fades it out: the player (when `005b9b00` answers true) through its slots
/// +0x41C (`1.0`) and +0x384 (1, the `float` at `0107a214`), anyone else through
/// `HighProcess::FadeOut(actor, 0, 0)` when its process is a `HighProcess` whose slot
/// +0x610 does not answer 4.
pub fn invisibility_effect_start(e: &mut Engine, this: Ptr) {
    let actor = target_actor(e, this);
    e.call(VALUE_MODIFIER_EFFECT_START, &args![this]);
    if actor == 0 {
        return;
    }
    let value = actor_value(e, this, actor);
    // FILD / FCOMP double / TEST AH,0x41 / JNZ: on only when greater (not when unordered).
    if f64::from(value) > e.mem.f64(ZERO) {
        if is_player_fade(e, actor) {
            player_fade_out(e, actor);
        } else {
            let high = high_process(e, actor);
            if high != 0 && e.vcall(high, 0x610, &[]).u32() != 4 {
                e.call(HIGH_PROCESS_FADE_OUT, &args![high, actor, 0u32, 0u32]);
            }
        }
    }
}

// Translated from 0080e230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080e230 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `InvisibilityEffect::Finish` (Xbox PDB): runs `ValueModifierEffect::Finish`; then, for a
/// target actor whose value of the effect's actor value is exactly `0.0` (the `double` at
/// `01012060`; unordered is not equal):
///
/// - fades it back in: the player (when `005b9b00` answers true) through its slot +0x384
///   (1, `0.0`), anyone else through `HighProcess::FadeIn(actor, 0)` (a `HighProcess`
///   whose slot +0x610 does not answer 4);
/// - with a local fade of `0.0` compared with the same constant: above it (and `005b9b00`
///   true) the player's slot +0x384 gets 1 and `004b3ab0`'s remap of `1.0 - fade` from
///   `[0.0, 1.0]` onto the two settings `0040df50` / `0040df70` answer; otherwise
///   `Actor::UpdateAlpha`. With the constant at `0.0` the first branch never runs; it is
///   translated as the exe has it;
/// - when the effect setting's model has a path, makes a `MagicModelHitEffect` (0x3C bytes,
///   `00aa13e0`; `0081c720` with the actor, the model's slot +0x14 and `-1.0`): handed to
///   `ProcessLists::AddTempEffect` when its slot +0xC4 answers true, otherwise deleted
///   through its slot 0 with 1.
pub fn invisibility_effect_finish(e: &mut Engine, this: Ptr) {
    e.call(VALUE_MODIFIER_EFFECT_FINISH, &args![this]);
    let actor = target_actor(e, this);
    if actor == 0 {
        return;
    }
    let value = actor_value(e, this, actor);
    // TEST AH,0x44 / JP: on only when equal.
    if f64::from(value) != e.mem.f64(ZERO) {
        return;
    }
    if is_player_fade(e, actor) {
        e.vcall(actor, 0x384, &args![1u32, 0.0f32]);
    } else {
        let high = high_process(e, actor);
        if high != 0 && e.vcall(high, 0x610, &[]).u32() != 4 {
            e.call(HIGH_PROCESS_FADE_IN, &args![high, actor, 0u32]);
        }
    }
    let fade = 0.0f32;
    // TEST AH,0x41 / JNZ: the block runs only when the fade is greater.
    if f64::from(fade) > e.mem.f64(ZERO) {
        if e.call(PLAYER_FADE_TEST, &[]).bool() {
            let remaining = x87_store32(x87_sub(1.0, x87_load32(fade.to_bits())));
            let low = x87_store32(e.call(SETTING_VALUE_0040DF50, &[]).f64());
            let high = x87_store32(e.call(SETTING_VALUE_0040DF70, &[]).f64());
            let alpha = x87_store32(
                e.call(REMAP, &args![high, low, 0.0f32, 1.0f32, remaining])
                    .f64(),
            );
            e.vcall(actor, 0x384, &args![1u32, alpha]);
        } else {
            e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
        }
    } else {
        e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
    }
    let mut hit_effect = 0u32;
    let setting = e.call(EFFECT_SETTING, &args![this]).u32();
    if e.call(MODEL_PATH_LENGTH, &args![setting.wrapping_add(0x18)])
        .u32()
        != 0
    {
        let block = e.call(ALLOCATE_TEMP_EFFECT, &args![0x3cu32]).u32();
        hit_effect = if block != 0 {
            let model = e
                .call(EFFECT_SETTING, &args![this])
                .u32()
                .wrapping_add(0x18);
            let minus_one = x87_quiet_bits(e.mem.u32(MINUS_ONE));
            let path = e.vcall(model, 0x14, &[]).u32();
            e.call(
                MAGIC_MODEL_HIT_EFFECT_CONSTRUCTOR,
                &args![block, actor, path, minus_one],
            )
            .u32()
        } else {
            0
        };
    }
    if hit_effect != 0 && e.vcall(hit_effect, 0xc4, &[]).bool() {
        e.call(ADD_TEMP_EFFECT, &args![PROCESS_LISTS, hit_effect]);
    } else if hit_effect != 0 {
        e.vcall(hit_effect, 0, &args![1u32]);
    }
}

// Translated from 0080e4c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080e4c0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map (`cdecl`, three words): allocates 0x4C bytes (`00401000`) and
/// constructs an `InvisibilityEffect` there with the caster, spell and effect item; returns
/// it, or null when the allocation failed.
pub fn fn_0080e4c0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    if block != 0 {
        fn_0080e030(e, Ptr::new(block), caster, spell, effect).addr()
    } else {
        0
    }
}

// Translated from 0080e540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InvisibilityEffect::FinishInitLoadGame` (engine map; `thiscall`, `RET 4`: the actor):
/// runs `ValueModifierEffect::FinishInitLoadGame` with the actor, then fades the player out
/// again (slots +0x41C and +0x384, when `005b9b00` answers true and the actor is the
/// player) or has `Actor::UpdateAlpha` run on the actor.
pub fn invisibility_effect_finish_init_load_game(e: &mut Engine, this: Ptr, actor: u32) {
    e.call(
        VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME,
        &args![this, actor],
    );
    if is_player_fade(e, actor) {
        player_fade_out(e, actor);
    } else {
        e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
    }
}

// Translated from 0080e5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InvisibilityEffect::Revert` (engine map; `thiscall`, `RET 4`: the actor): runs the base
/// body `004534f0` with the actor, then gives the player's slot +0x384 1 and `0.0` (when
/// `005b9b00` answers true and the actor is the player) or runs `Actor::UpdateAlpha`.
pub fn invisibility_effect_revert(e: &mut Engine, this: Ptr, actor: u32) {
    e.call(BASE_REVERT, &args![this, actor]);
    if is_player_fade(e, actor) {
        e.vcall(actor, 0x384, &args![1u32, 0.0f32]);
    } else {
        e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_e030, fn_0080e030(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x0080_e060,
            invisibility_effect_copy(Ptr<ActiveEffect>) -> u32
        ),
        entry!(0x0080_e110, invisibility_effect_start(Ptr)),
        entry!(0x0080_e230, invisibility_effect_finish(Ptr)),
        entry!(0x0080_e4c0, fn_0080e4c0(u32, u32, u32) -> u32),
        entry!(
            0x0080_e540,
            invisibility_effect_finish_init_load_game(Ptr, u32)
        ),
        entry!(0x0080_e5b0, invisibility_effect_revert(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
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

    const ACTOR_VTABLE: u32 = 0x0300_0000;
    const OWNER_VTABLE: u32 = 0x0300_1000;
    const HIGH_VTABLE: u32 = 0x0300_2000;
    const HIT_VTABLE: u32 = 0x0300_3000;
    const MODEL_VTABLE: u32 = 0x0300_4000;
    const ACTOR_SLOT_41C: u32 = 0x0310_0000;
    const ACTOR_SLOT_384: u32 = 0x0310_0010;
    const OWNER_VALUE: u32 = 0x0310_0020;
    const HIGH_KIND: u32 = 0x0310_0030;
    const HIT_KEEP: u32 = 0x0310_0040;
    const HIT_DELETE: u32 = 0x0310_0050;
    const MODEL_PATH: u32 = 0x0310_0060;

    /// An effect at 0x5000 whose target (0x6000) is the actor 0x7000, with doubles for
    /// every callee.
    fn rig(value: i32, player_test: bool, is_player: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(ZERO, 0.0);
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.map(0x0107_a000, 0x1000);
        e.mem.set_f32(PLAYER_FADE_OUT_VALUE, 0.25);
        e.map(0x011d_e000, 0x1000);
        let this = e.mem.alloc(0x50);
        let actor = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x48, 17);
        e.mem.set_u32(actor, ACTOR_VTABLE);
        e.mem.set_u32(actor + 0xa4, OWNER_VTABLE);
        e.mem.set_u32(PLAYER, if is_player { actor } else { 1 });
        let mut slots = vec![0u32; 0x420 / 4];
        slots[0x41c / 4] = ACTOR_SLOT_41C;
        slots[0x384 / 4] = ACTOR_SLOT_384;
        e.put_vtable(ACTOR_VTABLE, &slots);
        e.put_vtable(OWNER_VTABLE, &[0, 0, OWNER_VALUE]);
        let mut high = vec![0u32; 0x614 / 4];
        high[0x610 / 4] = HIGH_KIND;
        e.put_vtable(HIGH_VTABLE, &high);
        let mut hit = vec![0u32; 0xc8 / 4];
        hit[0] = HIT_DELETE;
        hit[0xc4 / 4] = HIT_KEEP;
        e.put_vtable(HIT_VTABLE, &hit);
        e.put_vtable(MODEL_VTABLE, &[0, 0, 0, 0, 0, MODEL_PATH]);
        e.register(GET_MAGIC_TARGET, |_, _| 0x6000u32.into_ret());
        e.register_double(GET_TARGET_AS_ACTOR, move |_, _| actor.into_ret());
        e.register_double(OWNER_VALUE, move |_, _| value.into_ret());
        e.register_double(PLAYER_FADE_TEST, move |_, _| player_test.into_ret());
        for addr in [
            VALUE_MODIFIER_EFFECT_START,
            VALUE_MODIFIER_EFFECT_FINISH,
            VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME,
            BASE_REVERT,
            ACTOR_SLOT_41C,
            ACTOR_SLOT_384,
            HIGH_PROCESS_FADE_OUT,
            HIGH_PROCESS_FADE_IN,
            ACTOR_UPDATE_ALPHA,
            ADD_TEMP_EFFECT,
            HIT_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ACTOR_PROCESS, |_, _| 0x8000u32.into_ret());
        e.register_double(RT_DYNAMIC_CAST, |e, _| {
            let high = e.mem.alloc(8);
            e.mem.set_u32(high, HIGH_VTABLE);
            high.into_ret()
        });
        e.register(HIGH_KIND, |_, _| 3u32.into_ret());
        e.register(EFFECT_SETTING, |_, _| 0x9000u32.into_ret());
        e.register(MODEL_PATH_LENGTH, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        (e, this, actor)
    }

    #[test]
    fn constructor_runs_the_base_and_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(VALUE_MODIFIER_EFFECT_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x4c);
        assert_eq!(
            e.call(0x0080_e030, &args![this, 1u32, 2u32, 3u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), INVISIBILITY_EFFECT_VTABLE);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_CONSTRUCTOR),
            vec![vec![this, 1, 2, 3]]
        );
    }

    #[test]
    fn copy_and_create_construct_in_a_new_block_or_answer_null() {
        let mut e = Engine::new();
        e.register(VALUE_MODIFIER_EFFECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0310_0000, |_, _| Ret::default());
        e.put_vtable(
            0x0300_0000,
            &[0u32; 0x11]
                .iter()
                .chain(&[0x0310_0000])
                .copied()
                .collect::<Vec<_>>(),
        );
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this, 0x0300_0000);
        e.mem.set_u32(this + 8, 0x22);
        e.mem.set_u32(this + 0xc, 0x33);
        e.mem.set_u32(this + 0x28, 0x11);
        let copy = e.call(0x0080_e060, &args![this]).u32();
        assert_eq!(e.mem.u32(copy), INVISIBILITY_EFFECT_VTABLE);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_CONSTRUCTOR),
            vec![vec![copy, 0x11, 0x22, 0x33]]
        );
        assert_eq!(calls(&e, 0x0310_0000), vec![vec![this, copy]]);
        let made = e.call(0x0080_e4c0, &args![4u32, 5u32, 6u32]).u32();
        assert_eq!(e.mem.u32(made), INVISIBILITY_EFFECT_VTABLE);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x4c], vec![0x4c]]);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0080_e060, &args![this]).u32(), 0);
        assert_eq!(calls(&e, 0x0310_0000).last(), Some(&vec![this, 0]));
        assert_eq!(e.call(0x0080_e4c0, &args![4u32, 5u32, 6u32]).u32(), 0);
    }

    #[test]
    fn start_fades_the_player_through_its_slots() {
        let (mut e, this, actor) = rig(1, true, true);
        e.call(0x0080_e110, &args![this]);
        assert_eq!(calls(&e, OWNER_VALUE), vec![vec![actor + 0xa4, 17]]);
        assert_eq!(
            calls(&e, ACTOR_SLOT_41C),
            vec![vec![actor, 1.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, ACTOR_SLOT_384),
            vec![vec![actor, 1, 0.25f32.to_bits()]]
        );
        assert!(calls(&e, HIGH_PROCESS_FADE_OUT).is_empty());
    }

    #[test]
    fn start_fades_others_through_the_high_process_and_skips_non_positive_values() {
        let (mut e, this, actor) = rig(1, true, false);
        e.call(0x0080_e110, &args![this]);
        let fade = calls(&e, HIGH_PROCESS_FADE_OUT);
        assert_eq!(fade.len(), 1);
        assert_eq!(fade[0][1..], [actor, 0, 0]);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![0x8000, 0, TYPE_BASE_PROCESS, TYPE_HIGH_PROCESS, 0]]
        );
        let (mut e, this, _) = rig(0, true, true);
        e.call(0x0080_e110, &args![this]);
        assert_eq!(calls(&e, VALUE_MODIFIER_EFFECT_START).len(), 1);
        assert!(calls(&e, ACTOR_SLOT_41C).is_empty());
        assert!(calls(&e, ACTOR_PROCESS).is_empty());
        // Kind 4 processes are not faded.
        let (mut e, this, _) = rig(5, false, true);
        e.register(HIGH_KIND, |_, _| 4u32.into_ret());
        e.call(0x0080_e110, &args![this]);
        assert!(calls(&e, HIGH_PROCESS_FADE_OUT).is_empty());
    }

    #[test]
    fn finish_fades_back_in_at_zero_and_makes_the_hit_effect() {
        let (mut e, this, actor) = rig(0, true, true);
        e.call(0x0080_e230, &args![this]);
        assert_eq!(calls(&e, ACTOR_SLOT_384), vec![vec![actor, 1, 0]]);
        assert_eq!(calls(&e, ACTOR_UPDATE_ALPHA), vec![vec![actor]]);
        assert_eq!(calls(&e, MODEL_PATH_LENGTH), vec![vec![0x9018]]);
        assert!(calls(&e, ALLOCATE_TEMP_EFFECT).is_empty());
        // Another actor, with a model: the hit effect is kept when its slot +0xC4 says so.
        let (mut e, this, actor) = rig(0, false, true);
        let setting = e.mem.alloc(0x40);
        e.mem.set_u32(setting + 0x18, MODEL_VTABLE);
        e.register_double(EFFECT_SETTING, move |_, _| setting.into_ret());
        e.register(MODEL_PATH_LENGTH, |_, _| 5u32.into_ret());
        e.register(MODEL_PATH, |_, _| 0x4444u32.into_ret());
        e.register(ALLOCATE_TEMP_EFFECT, |_, _| 0x0aa0u32.into_ret());
        e.register(MAGIC_MODEL_HIT_EFFECT_CONSTRUCTOR, |e, _| {
            let hit = e.mem.alloc(8);
            e.mem.set_u32(hit, HIT_VTABLE);
            hit.into_ret()
        });
        e.register(HIT_KEEP, |_, _| 1u32.into_ret());
        e.call(0x0080_e230, &args![this]);
        let fade = calls(&e, HIGH_PROCESS_FADE_IN);
        assert_eq!(fade[0][1..], [actor, 0]);
        let made = calls(&e, MAGIC_MODEL_HIT_EFFECT_CONSTRUCTOR);
        assert_eq!(made, vec![vec![0x0aa0, actor, 0x4444, (-1.0f32).to_bits()]]);
        assert_eq!(calls(&e, ADD_TEMP_EFFECT).len(), 1);
        assert_eq!(calls(&e, ADD_TEMP_EFFECT)[0][0], PROCESS_LISTS);
        assert!(calls(&e, HIT_DELETE).is_empty());
        // Not kept: deleted with 1.
        e.register(HIT_KEEP, |_, _| 0u32.into_ret());
        e.call(0x0080_e230, &args![this]);
        assert_eq!(calls(&e, HIT_DELETE).len(), 1);
        assert_eq!(calls(&e, HIT_DELETE)[0][1], 1);
        // A non-zero value does nothing after the base finish.
        let (mut e, this, _) = rig(3, true, true);
        e.call(0x0080_e230, &args![this]);
        assert_eq!(calls(&e, VALUE_MODIFIER_EFFECT_FINISH).len(), 1);
        assert!(calls(&e, ACTOR_UPDATE_ALPHA).is_empty());
    }

    #[test]
    fn finish_runs_the_fade_block_when_the_constant_is_below_the_fade() {
        let (mut e, this, actor) = rig(0, true, true);
        e.mem.set_f64(ZERO, -1.0);
        e.register(OWNER_VALUE, |_, _| (-1i32).into_ret());
        e.register(SETTING_VALUE_0040DF50, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e.register(SETTING_VALUE_0040DF70, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        e.register(REMAP, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.call(0x0080_e230, &args![this]);
        assert_eq!(
            calls(&e, REMAP),
            vec![vec![
                3.0f32.to_bits(),
                2.0f32.to_bits(),
                0,
                1.0f32.to_bits(),
                1.0f32.to_bits()
            ]]
        );
        assert_eq!(
            calls(&e, ACTOR_SLOT_384),
            vec![vec![actor, 1, 0], vec![actor, 1, 0.5f32.to_bits()]]
        );
        assert!(calls(&e, ACTOR_UPDATE_ALPHA).is_empty());
    }

    #[test]
    fn finish_init_load_game_and_revert_pick_the_player_or_update_alpha() {
        let (mut e, this, actor) = rig(0, true, true);
        e.call(0x0080_e540, &args![this, actor]);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME),
            vec![vec![this, actor]]
        );
        assert_eq!(calls(&e, ACTOR_SLOT_41C).len(), 1);
        e.call(0x0080_e5b0, &args![this, actor]);
        assert_eq!(calls(&e, BASE_REVERT), vec![vec![this, actor]]);
        assert_eq!(calls(&e, ACTOR_SLOT_384).last(), Some(&vec![actor, 1, 0]));
        assert!(calls(&e, ACTOR_UPDATE_ALPHA).is_empty());
        let (mut e, this, actor) = rig(0, false, true);
        e.call(0x0080_e540, &args![this, actor]);
        e.call(0x0080_e5b0, &args![this, actor]);
        assert_eq!(
            calls(&e, ACTOR_UPDATE_ALPHA),
            vec![vec![actor], vec![actor]]
        );
        assert!(calls(&e, ACTOR_SLOT_384).is_empty());
    }
}

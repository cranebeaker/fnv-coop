//! `fallout/magic/chameleoneffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `0082ba60` (thiscall: three words; unnamed): the `ValueModifierEffect` constructor.
const VALUE_MODIFIER_EFFECT_CONSTRUCTOR: u32 = 0x0082_ba60;
/// `ChameleonEffect`'s vtable (stored by its constructor).
const CHAMELEON_EFFECT_VTABLE: u32 = 0x0107_994c;
/// `MagicTarget::GetTargetAsActor` (`00822b40`).
const MAGIC_TARGET_GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `0059bb30` (thiscall): the effect's target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `ValueModifierEffect::Start` (`0082c000`).
const VALUE_MODIFIER_EFFECT_START: u32 = 0x0082_c000;
/// `ValueModifierEffect::Finish` (`0082c810`).
const VALUE_MODIFIER_EFFECT_FINISH: u32 = 0x0082_c810;
/// `ValueModifierEffect::FinishInitLoadGame` (`0082cd10`, thiscall: one word).
const VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME: u32 = 0x0082_cd10;
/// `ActiveEffect::FinishLoadGame` (`008067b0`, thiscall: one word).
const ACTIVE_EFFECT_FINISH_LOAD_GAME: u32 = 0x0080_67b0;
/// `004534f0` (thiscall: one word; unnamed, `tes.cpp`): the base `Revert`.
const BASE_REVERT_004534F0: u32 = 0x0045_34f0;
/// `005b9b00` (no arguments, `AL`; unnamed, `tesscriptfunctions.cpp`).
const TEST_005B9B00: u32 = 0x005b_9b00;
/// `Actor::UpdateAlpha` (`008c4640`).
const ACTOR_UPDATE_ALPHA: u32 = 0x008c_4640;
/// `00404010` (cdecl: two `float`s, returns `ST0`): the larger (`FCOMPP` order kept by the
/// callee).
const FLOAT_MAX_00404010: u32 = 0x0040_4010;
/// `0040ebd0` (cdecl: two `float`s, returns `ST0`): the smaller.
const FLOAT_MIN_0040EBD0: u32 = 0x0040_ebd0;
/// `0040df50` (no arguments, returns `ST0`): a setting's `float` (`011c36d8`).
const SETTING_0040DF50: u32 = 0x0040_df50;
/// `0040df70` (no arguments, returns `ST0`): a setting's `float` (`011c3664`).
const SETTING_0040DF70: u32 = 0x0040_df70;
/// `004b3ab0` (cdecl: five `float`s, returns `ST0`): maps a value between two ranges.
const REMAP_004B3AB0: u32 = 0x004b_3ab0;
/// `100.0` (`double`, `01017a40`).
const HUNDRED: u32 = 0x0101_7a40;
/// `0.0` (`double`, `01012060`).
const ZERO: u32 = 0x0101_2060;

/// The effect's target as an actor (`00822b40` of `0059bb30`), null without a target.
fn target_actor(e: &mut Engine, this: u32) -> u32 {
    if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
        0
    } else {
        let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        e.call(MAGIC_TARGET_GET_TARGET_AS_ACTOR, &args![target])
            .u32()
    }
}

/// The actor value the effect modifies, as a fraction: slot +0xc of the actor's object at
/// +0xa4 with the effect's +0x48 word, divided by 100 (`FDIV`, `FSTP float`), then at least
/// 0 (`00404010`) and at most 1 (`0040ebd0`), stored as a `float`.
fn clamped_fraction(e: &mut Engine, this: u32, actor: u32) -> u32 {
    let value_index = e.mem.u32(this + 0x48);
    let value = e.vcall(actor + 0xa4, 0xc, &args![value_index]).f64();
    let hundred: f64 = e.global(HUNDRED);
    let fraction = x87_store32(x87_div(value, hundred));
    let at_least = e
        .call(FLOAT_MAX_00404010, &args![0.0f32, x87_quiet_bits(fraction)])
        .f64();
    let at_least = x87_store32(at_least);
    let clamped = e.call(FLOAT_MIN_0040EBD0, &args![1.0f32, at_least]).f64();
    x87_store32(clamped)
}

/// Sets the actor's alpha (slot +0x384 with 1 and the alpha) from `visibility`: it maps
/// `visibility` from (0, 1) to the two settings' range (`0040df50`, `0040df70`;
/// `004b3ab0`), each stored as a `float`.
fn set_alpha_for(e: &mut Engine, actor: u32, visibility: u32) {
    let low = x87_store32(e.call(SETTING_0040DF50, &[]).f64());
    let high = x87_store32(e.call(SETTING_0040DF70, &[]).f64());
    let alpha = e
        .call(
            REMAP_004B3AB0,
            &args![high, low, 0.0f32, 1.0f32, x87_quiet_bits(visibility)],
        )
        .f64();
    let alpha = x87_store32(alpha);
    e.vcall(actor, 0x384, &args![1u32, x87_quiet_bits(alpha)]);
}

/// The fading the starts and loads do: slot +0x41c of the actor with 1.0, then the alpha
/// for `1 - clamped_fraction`.
fn fade(e: &mut Engine, this: u32, actor: u32) {
    e.vcall(actor, 0x41c, &args![1.0f32]);
    let fraction = clamped_fraction(e, this, actor);
    let visibility = x87_store32(x87_sub(1.0, x87_load32(fraction)));
    set_alpha_for(e, actor, visibility);
}

// Translated from 0080acf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`chameleoneffect.cpp`; thiscall, three words): the `ChameleonEffect`
/// constructor: the `ValueModifierEffect` part (`0082ba60` with the three words), then its
/// vtable; returns `this`.
pub fn fn_0080acf0(e: &mut Engine, this: Ptr, caster: u32, spell: u32, item: u32) -> u32 {
    e.call(
        VALUE_MODIFIER_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, item],
    );
    e.mem.set_u32(this.addr(), CHAMELEON_EFFECT_VTABLE);
    this.addr()
}

// Translated from 0080ad20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080ad20 - the exception-unwinding frame (FS:[0]) is not translated.
/// `ChameleonEffect::Copy` (Xbox PDB): a new 0x4c-byte effect (`operator new`, then
/// `fn_0080acf0` with the caster +0x28, spell +8 and effect item +0xc; null when the
/// allocation fails), given this effect's data through `CopyData` (slot +0x44); returns it.
pub fn chameleon_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let base = this.addr();
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        fn_0080acf0(e, Ptr::new(block), caster, spell, item)
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 0080add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChameleonEffect::Start` (Xbox PDB): `ValueModifierEffect::Start`; for a target actor,
/// when `005b9b00` is true it fades (slot +0x41c with 1.0, the alpha for one minus the
/// clamped fraction of its value), otherwise `Actor::UpdateAlpha`.
pub fn chameleon_effect_start(e: &mut Engine, this: Ptr) {
    let actor = target_actor(e, this.addr());
    e.call(VALUE_MODIFIER_EFFECT_START, &args![this]);
    if actor == 0 {
        return;
    }
    if e.call(TEST_005B9B00, &[]).bool() {
        fade(e, this.addr(), actor);
    } else {
        e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
    }
}

// Translated from 0080aef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChameleonEffect::Finish` (Xbox PDB): `ValueModifierEffect::Finish`; for a target actor,
/// when `005b9b00` is true: with a clamped fraction above 0 (`FCOMP`; NaN counts as not)
/// the alpha for one minus that fraction divided by 100 again, otherwise alpha 0 then
/// `Actor::UpdateAlpha`; when it is false, `Actor::UpdateAlpha`.
pub fn chameleon_effect_finish(e: &mut Engine, this: Ptr) {
    e.call(VALUE_MODIFIER_EFFECT_FINISH, &args![this]);
    let actor = target_actor(e, this.addr());
    if actor == 0 {
        return;
    }
    if !e.call(TEST_005B9B00, &[]).bool() {
        e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
        return;
    }
    let fraction = clamped_fraction(e, this.addr(), actor);
    let zero: f64 = e.global(ZERO);
    // FCOMP; TEST AH,0x41; JNZ: the fade only when above 0.
    if x87_load32(fraction) > zero {
        let hundred: f64 = e.global(HUNDRED);
        let scaled = x87_div(x87_load32(fraction), hundred);
        let visibility = x87_store32(x87_sub(1.0, scaled));
        set_alpha_for(e, actor, visibility);
    } else {
        e.vcall(actor, 0x384, &args![1u32, 0.0f32]);
        e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
    }
}

// Translated from 0080b040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080b040 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`chameleoneffect.cpp`; cdecl, three words): a new 0x4c-byte `ChameleonEffect`
/// (`fn_0080acf0` with the three words; null when the allocation fails).
pub fn fn_0080b040(e: &mut Engine, caster: u32, spell: u32, item: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    if block == 0 {
        0
    } else {
        fn_0080acf0(e, Ptr::new(block), caster, spell, item)
    }
}

// Translated from 0080b0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChameleonEffect::FinishInitLoadGame` (Xbox PDB; the word is the actor):
/// `ValueModifierEffect::FinishInitLoadGame`, then the actor fades as in `Start` when
/// `005b9b00` is true, or `Actor::UpdateAlpha`.
pub fn chameleon_effect_finish_init_load_game(e: &mut Engine, this: Ptr, actor: u32) {
    e.call(
        VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME,
        &args![this, actor],
    );
    if e.call(TEST_005B9B00, &[]).bool() {
        fade(e, this.addr(), actor);
    } else {
        e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
    }
}

// Translated from 0080b1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChameleonEffect::Revert` (Xbox PDB; the word is the actor): the base revert
/// (`004534f0`), then alpha 0 (slot +0x384 with 1, 0.0) when `005b9b00` is true, or
/// `Actor::UpdateAlpha`.
pub fn chameleon_effect_revert(e: &mut Engine, this: Ptr, actor: u32) {
    e.call(BASE_REVERT_004534F0, &args![this, actor]);
    if e.call(TEST_005B9B00, &[]).bool() {
        e.vcall(actor, 0x384, &args![1u32, 0.0f32]);
    } else {
        e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
    }
}

// Translated from 0080b200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChameleonEffect::FinishLoadGame` (Xbox PDB): the actor is the word's slot +0xc;
/// `ActiveEffect::FinishLoadGame`; then for an active effect the actor fades as in `Start`
/// when `005b9b00` is true, or `Actor::UpdateAlpha`.
pub fn chameleon_effect_finish_load_game(e: &mut Engine, this: Ptr, argument: u32) {
    let actor = e.vcall(argument, 0xc, &[]).u32();
    e.call(ACTIVE_EFFECT_FINISH_LOAD_GAME, &args![this, argument]);
    if e.mem.u8(this.addr() + 0x10) == 0 {
        return;
    }
    if e.call(TEST_005B9B00, &[]).bool() {
        fade(e, this.addr(), actor);
    } else {
        e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
    }
}

// Translated from 0080b310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ChameleonEffect::Revert_ov2` (Xbox PDB): the actor is the word's slot +0xc; the base
/// revert (`004534f0` with the word), then alpha 0 when `005b9b00` is true, or
/// `Actor::UpdateAlpha`.
pub fn chameleon_effect_revert_ov2(e: &mut Engine, this: Ptr, argument: u32) {
    let actor = e.vcall(argument, 0xc, &[]).u32();
    e.call(BASE_REVERT_004534F0, &args![this, argument]);
    if e.call(TEST_005B9B00, &[]).bool() {
        e.vcall(actor, 0x384, &args![1u32, 0.0f32]);
    } else {
        e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_acf0, fn_0080acf0(Ptr, u32, u32, u32) -> u32),
        entry!(0x0080_ad20, chameleon_effect_copy(Ptr) -> u32),
        entry!(0x0080_add0, chameleon_effect_start(Ptr)),
        entry!(0x0080_aef0, chameleon_effect_finish(Ptr)),
        entry!(0x0080_b040, fn_0080b040(u32, u32, u32) -> u32),
        entry!(
            0x0080_b0c0,
            chameleon_effect_finish_init_load_game(Ptr, u32)
        ),
        entry!(0x0080_b1b0, chameleon_effect_revert(Ptr, u32)),
        entry!(0x0080_b200, chameleon_effect_finish_load_game(Ptr, u32)),
        entry!(0x0080_b310, chameleon_effect_revert_ov2(Ptr, u32)),
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
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn answer_float(e: &mut Engine, addr: u32, st0: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0,
            ..Ret::default()
        });
    }

    /// An object whose vtable slots answer the given words; each slot's double sits at the
    /// slot's own address.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x600);
        for (offset, value) in slots {
            e.mem.set_u32(vtable + offset, vtable + offset);
            answer(e, vtable + offset, *value);
        }
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        this
    }

    fn slot(e: &Engine, object: u32, offset: u32) -> u32 {
        e.mem.u32(object) + offset
    }

    /// An actor whose value object (+0xa4, slot +0xc) answers `value`; the float helpers run
    /// as the exe's do (max, min, the settings 0.25 and 0.75, the linear remap).
    fn rig(value: f64) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x8000);
        e.set_global(HUNDRED, 100.0f64);
        e.set_global(ZERO, 0.0f64);
        let actor = object(&mut e, &[(0x41c, 0), (0x384, 0)]);
        let values = object(&mut e, &[]);
        let values_vtable = e.mem.u32(values);
        e.mem.set_u32(actor + 0xa4, values_vtable);
        e.mem.set_u32(values_vtable + 0xc, values_vtable + 0xc);
        answer_float(&mut e, values_vtable + 0xc, value);
        e.register(FLOAT_MAX_00404010, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: f64::from(if x < y { y } else { x }),
                ..Ret::default()
            }
        });
        e.register(FLOAT_MIN_0040EBD0, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: f64::from(if x > y { y } else { x }),
                ..Ret::default()
            }
        });
        answer_float(&mut e, SETTING_0040DF50, 0.25);
        answer_float(&mut e, SETTING_0040DF70, 0.75);
        e.register(REMAP_004B3AB0, |_, a| {
            let p: Vec<f64> = a.iter().map(|w| f64::from(f32::from_bits(*w))).collect();
            Ret {
                st0: (p[4] - p[2]) / (p[3] - p[2]) * (p[1] - p[0]) + p[0],
                ..Ret::default()
            }
        });
        for addr in [
            VALUE_MODIFIER_EFFECT_START,
            VALUE_MODIFIER_EFFECT_FINISH,
            VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME,
            ACTIVE_EFFECT_FINISH_LOAD_GAME,
            BASE_REVERT_004534F0,
            ACTOR_UPDATE_ALPHA,
        ] {
            answer(&mut e, addr, 0);
        }
        answer(&mut e, TEST_005B9B00, 1);
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        e.call_log = Some(vec![]);
        (e, actor)
    }

    fn effect(e: &mut Engine) -> u32 {
        let this = object(e, &[(0x44, 0)]);
        e.mem.set_u32(this + 0x48, 0x2a);
        this
    }

    #[test]
    fn constructor_and_copy_build_a_chameleon_effect() {
        let (mut e, _) = rig(0.0);
        answer(&mut e, VALUE_MODIFIER_EFFECT_CONSTRUCTOR, 0);
        let this = effect(&mut e);
        e.mem.set_u32(this + 8, 0x08);
        e.mem.set_u32(this + 0xc, 0x0c);
        e.mem.set_u32(this + 0x28, 0x28);
        let block = e.mem.alloc(0x4c);
        answer(&mut e, OPERATOR_NEW, block);
        assert_eq!(e.call(0x0080_ad20, &args![this]).u32(), block);
        assert_eq!(e.mem.u32(block), CHAMELEON_EFFECT_VTABLE);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_CONSTRUCTOR),
            vec![vec![block, 0x28, 0x08, 0x0c]]
        );
        assert_eq!(calls(&e, slot(&e, this, 0x44)), vec![vec![this, block]]);
        assert_eq!(
            e.call(0x0080_acf0, &args![block, 1u32, 2u32, 3u32]).u32(),
            block
        );
        assert_eq!(e.call(0x0080_b040, &args![1u32, 2u32, 3u32]).u32(), block);
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0080_b040, &args![1u32, 2u32, 3u32]).u32(), 0);
        assert_eq!(e.call(0x0080_ad20, &args![this]).u32(), 0);
    }

    #[test]
    fn start_fades_by_the_clamped_value() {
        // 50 of 100: visibility 0.5, alpha 0.25 + 0.5 * 0.5 = 0.5.
        let (mut e, actor) = rig(50.0);
        let this = effect(&mut e);
        e.call(0x0080_add0, &args![this]);
        assert_eq!(calls(&e, VALUE_MODIFIER_EFFECT_START), vec![vec![this]]);
        assert_eq!(
            calls(&e, slot(&e, actor, 0x41c)),
            vec![vec![actor, 1.0f32.to_bits()]]
        );
        let values_slot = e.mem.u32(actor + 0xa4) + 0xc;
        assert_eq!(calls(&e, values_slot), vec![vec![actor + 0xa4, 0x2a]]);
        assert_eq!(
            calls(&e, slot(&e, actor, 0x384)),
            vec![vec![actor, 1, 0.5f32.to_bits()]]
        );
        // Over 100 clamps to 1: visibility 0, alpha 0.75.
        let (mut e, actor) = rig(250.0);
        let this = effect(&mut e);
        e.call(0x0080_add0, &args![this]);
        assert_eq!(
            calls(&e, REMAP_004B3AB0),
            vec![vec![
                0.75f32.to_bits(),
                0.25f32.to_bits(),
                0,
                1.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(
            calls(&e, slot(&e, actor, 0x384)),
            vec![vec![actor, 1, 0.75f32.to_bits()]]
        );
        // Without 005b9b00 only the alpha update runs.
        let (mut e, actor) = rig(50.0);
        answer(&mut e, TEST_005B9B00, 0);
        let this = effect(&mut e);
        e.call(0x0080_add0, &args![this]);
        assert_eq!(calls(&e, ACTOR_UPDATE_ALPHA), vec![vec![actor]]);
        assert!(calls(&e, slot(&e, actor, 0x384)).is_empty());
    }

    #[test]
    fn finish_restores_the_alpha() {
        // 50: fraction 0.5 > 0: visibility 1 - 0.005.
        let (mut e, _actor) = rig(50.0);
        let this = effect(&mut e);
        e.call(0x0080_aef0, &args![this]);
        assert_eq!(calls(&e, VALUE_MODIFIER_EFFECT_FINISH), vec![vec![this]]);
        let visibility = f32::from_bits(x87_store32(x87_sub(1.0, 0.5 / 100.0)));
        assert_eq!(calls(&e, REMAP_004B3AB0)[0][4], visibility.to_bits());
        assert!(calls(&e, ACTOR_UPDATE_ALPHA).is_empty());
        // A fraction of 0 (or NaN): alpha 0, then the update.
        for value in [0.0, f64::NAN] {
            let (mut e, actor) = rig(value);
            let this = effect(&mut e);
            e.call(0x0080_aef0, &args![this]);
            assert_eq!(calls(&e, slot(&e, actor, 0x384)), vec![vec![actor, 1, 0]]);
            assert_eq!(calls(&e, ACTOR_UPDATE_ALPHA), vec![vec![actor]]);
        }
    }

    #[test]
    fn loads_and_reverts_reach_the_actor() {
        let (mut e, actor) = rig(50.0);
        let this = effect(&mut e);
        e.call(0x0080_b0c0, &args![this, actor]);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME),
            vec![vec![this, actor]]
        );
        assert_eq!(calls(&e, slot(&e, actor, 0x41c)).len(), 1);
        e.call(0x0080_b1b0, &args![this, actor]);
        assert_eq!(calls(&e, BASE_REVERT_004534F0), vec![vec![this, actor]]);
        assert_eq!(calls(&e, slot(&e, actor, 0x384))[1], vec![actor, 1, 0]);
        // The ov2 forms find the actor through the word's slot +0xc.
        let holder = object(&mut e, &[(0xc, actor)]);
        e.mem.set_u8(this + 0x10, 1);
        e.call(0x0080_b200, &args![this, holder]);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_FINISH_LOAD_GAME),
            vec![vec![this, holder]]
        );
        assert_eq!(calls(&e, slot(&e, actor, 0x41c)).len(), 2);
        e.call(0x0080_b310, &args![this, holder]);
        assert_eq!(calls(&e, BASE_REVERT_004534F0)[1], vec![this, holder]);
        // An inactive effect only finishes the load; without 005b9b00 the update runs.
        e.mem.set_u8(this + 0x10, 0);
        answer(&mut e, TEST_005B9B00, 0);
        e.call(0x0080_b200, &args![this, holder]);
        assert!(calls(&e, ACTOR_UPDATE_ALPHA).is_empty());
        e.call(0x0080_b310, &args![this, holder]);
        e.call(0x0080_b1b0, &args![this, actor]);
        e.call(0x0080_b0c0, &args![this, actor]);
        assert_eq!(calls(&e, ACTOR_UPDATE_ALPHA).len(), 3);
    }
}

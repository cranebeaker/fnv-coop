//! `fallout/magic/concussioneffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ConcussionEffect` (Xbox PDB), 0x4c bytes (the PC code allocates 0x4c): an
    /// `ActiveEffect` (0x48) and the timer.
    pub struct ConcussionEffect: 0x4c {
        /// `ActiveEffect::pSpell` (Xbox PDB): `MagicItem*`.
        0x08 pSpell: u32,
        /// `ActiveEffect::pEffect` (Xbox PDB): `EffectItem*`.
        0x0C pEffect: u32,
        /// `ActiveEffect::pCaster` (Xbox PDB): `MagicCaster*`.
        0x28 pCaster: u32,
        /// `fTimer` (Xbox PDB).
        0x48 fTimer: u32,
    }
}

/// The allocator `00401000` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `ConcussionEffect` constructor `0080bc70` (`thiscall`, three words).
const CONCUSSION_EFFECT_CONSTRUCTOR: u32 = 0x0080_bc70;
/// `MagicTarget::GetTargetAsActor` (engine map, `00822b40`, `thiscall`).
const GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `0059bb30` (`thiscall`): the effect's magic target (the engine map's name for the
/// folded body, `D3DTexture_LockRect`, does not fit).
const GET_MAGIC_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `004534f0` (`thiscall`, one word: the elapsed time as a `float`), the base update.
const BASE_UPDATE_004534F0: u32 = 0x0045_34f0;
/// `00403e20` (`thiscall`): the address of a `Setting`'s value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The `Setting` whose value the timer must exceed (`011d1ba4`).
const SETTING_011D1BA4: u32 = 0x011d_1ba4;
/// The `Setting` whose value is passed to the image-space trigger (`011cf834`).
const SETTING_011CF834: u32 = 0x011c_f834;
/// `008be870` (`thiscall`): a `float` of the actor in ST0, compared with 0.0.
const ACTOR_VALUE_008BE870: u32 = 0x008b_e870;
/// `0.0` (`double`, `01012060`).
const ZERO: u32 = 0x0101_2060;
/// A byte set to 1 while a concussion is applied and cleared after (`011dbe50`; the Xbox
/// PDB has a static `ConcussionEffect::bCheckingConcussions`, not confirmed to be this
/// byte).
const APPLYING_011DBE50: u32 = 0x011d_be50;
/// The player (`PlayerCharacter *`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `PlayerCharacter::IsSleepingorResting` (engine map, `0094df60`, `thiscall`).
const IS_SLEEPING_OR_RESTING: u32 = 0x0094_df60;
/// `TESImageSpaceModifier::GetConcussion` (engine map, `0080bfa0`, `cdecl`, no words).
const GET_CONCUSSION_MODIFIER: u32 = 0x0080_bfa0;
/// `ImageSpaceModifierInstanceForm::Trigger` (engine map, `005299a0`, `cdecl`, three
/// words: the modifier, a `float`, a word).
const TRIGGER_MODIFIER: u32 = 0x0052_99a0;
/// `BSAudio::QInstance` (engine map, `00453a70`): the audio manager.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByName` (engine map, `00ad7550`, `thiscall`, three words: the
/// handle to fill, the name, flags).
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
/// `BSSoundHandle::Play` (engine map, `00ad8830`, `thiscall`, one word).
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// The sound handle's destructor `00483710` (`thiscall`).
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
/// `"FXCrippleHead"`.
const CRIPPLE_HEAD_SOUND: u32 = 0x0107_9c38;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map name, `008d8520`, `thiscall`).
const ACTOR_OBJECT_008D8520: u32 = 0x008d_8520;

// Translated from 0080bcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ConcussionEffect::Copy` (engine map; `thiscall`, plain `RET`): allocates a 0x4c-byte
/// `ConcussionEffect` (`00401000`) and constructs it (`0080bc70`) with this effect's
/// `pCaster`, `pSpell` and `pEffect` (null when the allocation fails), copies `fTimer`
/// into it (`FLD`/`FSTP`: a signalling NaN is copied quiet; the copy is written even when
/// the allocation failed) and lets this effect copy the rest (its slot +0x44 with the
/// copy). Returns the copy.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0080bcb0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn concussion_effect_copy(e: &mut Engine, this: Ptr<ConcussionEffect>) -> u32 {
    let block = e.call(ALLOCATE, &args![0x4cu32]).u32();
    let copy = if block != 0 {
        let caster = e.get(this, ConcussionEffect::pCaster);
        let spell = e.get(this, ConcussionEffect::pSpell);
        let effect = e.get(this, ConcussionEffect::pEffect);
        e.call(
            CONCUSSION_EFFECT_CONSTRUCTOR,
            &args![block, caster, spell, effect],
        )
        .u32()
    } else {
        0
    };
    let timer = x87_quiet_bits(e.get(this, ConcussionEffect::fTimer));
    e.mem.set_u32(copy.wrapping_add(0x48), timer);
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 0080bd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ConcussionEffect::CheckTarget` (engine map; `thiscall`, `RET 4`: the magic target):
/// true when the target is an actor (`MagicTarget::GetTargetAsActor`; a null target is
/// none) whose slot +0x22c (with 0) and slot +0x2e8 both answer false (`AL`); false
/// otherwise. What those two slots ask is not confirmed.
pub fn concussion_effect_check_target(
    e: &mut Engine,
    _this: Ptr<ConcussionEffect>,
    target: u32,
) -> bool {
    let actor = if target != 0 {
        e.call(GET_TARGET_AS_ACTOR, &args![target]).u32()
    } else {
        0
    };
    if actor == 0 {
        return false;
    }
    if e.vcall(actor, 0x22c, &args![0u32]).u8() != 0 {
        return false;
    }
    e.vcall(actor, 0x2e8, &args![]).u8() == 0
}

// Translated from 0080bdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ConcussionEffect::Update` (engine map; `thiscall`, `RET 4`: the elapsed time): runs
/// the base update `004534f0` with the time, adds the time to `fTimer`, and when the
/// effect's magic target (`0059bb30`) is an actor (`MagicTarget::GetTargetAsActor`)
/// checks whether to apply the concussion to it
/// ([`concussion_effect_check_apply_concussion`]).
pub fn concussion_effect_update(e: &mut Engine, this: Ptr<ConcussionEffect>, seconds: u32) {
    e.call(BASE_UPDATE_004534F0, &args![this, x87_quiet_bits(seconds)]);
    let timer = x87_load32(e.get(this, ConcussionEffect::fTimer));
    let timer = x87_store32(x87_add(timer, x87_m32(seconds)));
    e.set(this, ConcussionEffect::fTimer, timer);
    let actor = if e.call(GET_MAGIC_TARGET_0059BB30, &args![this]).u32() != 0 {
        let target = e.call(GET_MAGIC_TARGET_0059BB30, &args![this]).u32();
        e.call(GET_TARGET_AS_ACTOR, &args![target]).u32()
    } else {
        0
    };
    if actor != 0 {
        concussion_effect_check_apply_concussion(e, this, actor);
    }
}

// Translated from 0080be40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ConcussionEffect::CheckApplyConcussion` (engine map; `thiscall`, `RET 4`: the actor).
/// Only when the value of the `Setting` at `011d1ba4` is below `fTimer` (not when either
/// is NaN): the timer is reset to 0.0, and when the actor's `008be870` value equals 0.0
/// (`01012060`) and the object at the actor's +0xa4 answers 0 from its slot +8 with 0x48,
/// the concussion is applied with the byte at `011dbe50` set to 1 around it:
///
/// - when the actor's slot +0x360 answers true (`AL`) and the player (`011dea3c`) is not
///   sleeping or resting, the concussion image-space modifier (`0080bfa0`) is triggered
///   (`005299a0`) with the value of the `Setting` at `011cf834` and 0, and the
///   `"FXCrippleHead"` sound is played (a 12-byte sound handle on the stack filled by
///   `BSAudio::GetSoundHandleByName` with flags 0x101, `Play` with 0, then destroyed);
/// - the object the actor's `008d8520` returns gets its slot +0x44 with the actor, 0, 2,
///   1, 0, 1.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0080be40 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn concussion_effect_check_apply_concussion(
    e: &mut Engine,
    this: Ptr<ConcussionEffect>,
    actor: u32,
) {
    let threshold = e.call(SETTING_VALUE, &args![SETTING_011D1BA4]).u32();
    let timer = f64::from(f32::from_bits(e.get(this, ConcussionEffect::fTimer)));
    let threshold = f64::from(e.mem.f32(threshold));
    // FLD fTimer; FLD setting; FCOMPP; TEST AH,5; JP: goes on only for setting < timer.
    if !(threshold < timer) {
        return;
    }
    e.set(this, ConcussionEffect::fTimer, 0);
    let value = e.call(ACTOR_VALUE_008BE870, &args![actor]).f64();
    let zero = e.mem.f64(ZERO);
    // FCOMP double; TEST AH,0x44; JP: goes on only when equal (not when unordered).
    if value != zero {
        return;
    }
    let embedded = actor.wrapping_add(0xa4);
    if e.vcall(embedded, 8, &args![0x48u32]).u32() != 0 {
        return;
    }
    e.mem.set_u8(APPLYING_011DBE50, 1);
    if e.vcall(actor, 0x360, &args![]).u8() != 0 {
        let player = e.mem.u32(PLAYER);
        if e.call(IS_SLEEPING_OR_RESTING, &args![player]).u8() == 0 {
            let value = e.call(SETTING_VALUE, &args![SETTING_011CF834]).u32();
            let strength = x87_quiet_bits(e.mem.u32(value));
            let modifier = e.call(GET_CONCUSSION_MODIFIER, &args![]).u32();
            e.call(TRIGGER_MODIFIER, &args![modifier, strength, 0u32]);
            e.with_stack(0xc, |e, handle| {
                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                let filled = e
                    .call(
                        GET_SOUND_HANDLE_BY_NAME,
                        &args![audio, handle, CRIPPLE_HEAD_SOUND, 0x101u32],
                    )
                    .u32();
                e.call(SOUND_HANDLE_PLAY, &args![filled, 0u32]);
                e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
            });
        }
    }
    let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
    e.vcall(object, 0x44, &args![actor, 0u32, 2u32, 1u32, 0u32, 1u32]);
    e.mem.set_u8(APPLYING_011DBE50, 0);
}

/// A byte that must be set for `GetConcussion` to answer (`0118abb1`).
const CONCUSSION_ENABLED_0118ABB1: u32 = 0x0118_abb1;
/// The concussion modifier `GetConcussion` keeps (`011c9704`).
const CONCUSSION_MODIFIER_011C9704: u32 = 0x011c_9704;
/// `004839c0` (cdecl, one word: a form id; returns the form).
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfDelta, source type, target type, isReference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESForm`'s RTTI type descriptor (`.?AVTESForm@@`).
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// `TESImageSpaceModifier`'s RTTI type descriptor (`.?AVTESImageSpaceModifier@@`).
const TES_IMAGE_SPACE_MODIFIER_TYPE: u32 = 0x0118_62b8;

// Translated from 0080bfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::GetConcussion` (Xbox PDB, cdecl, no words): null unless the
/// byte at `0118abb1` is set; otherwise the modifier kept at `011c9704`, found on first
/// use as form 0x164 (`004839c0`) cast from `TESForm` to `TESImageSpaceModifier`.
pub fn tes_image_space_modifier_get_concussion(e: &mut Engine) -> u32 {
    if e.mem.u8(CONCUSSION_ENABLED_0118ABB1) == 0 {
        return 0;
    }
    if e.global::<u32>(CONCUSSION_MODIFIER_011C9704) == 0 {
        let form = e.call(LOOKUP_FORM_004839C0, &args![0x164u32]).u32();
        let modifier = e
            .call(
                RT_DYNAMIC_CAST,
                &args![
                    form,
                    0u32,
                    TES_FORM_TYPE,
                    TES_IMAGE_SPACE_MODIFIER_TYPE,
                    0u32
                ],
            )
            .u32();
        e.set_global(CONCUSSION_MODIFIER_011C9704, modifier);
    }
    e.global::<u32>(CONCUSSION_MODIFIER_011C9704)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_bfa0, tes_image_space_modifier_get_concussion() -> u32),
        entry!(
            0x0080_bcb0,
            concussion_effect_copy(Ptr<ConcussionEffect>) -> u32
        ),
        entry!(
            0x0080_bd60,
            concussion_effect_check_target(Ptr<ConcussionEffect>, u32) -> bool
        ),
        entry!(
            0x0080_bdd0,
            concussion_effect_update(Ptr<ConcussionEffect>, u32)
        ),
        entry!(
            0x0080_be40,
            concussion_effect_check_apply_concussion(Ptr<ConcussionEffect>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An object whose vtable slots point at the addresses `base + slot`, each doubled to
    /// answer the given word.
    fn object_with_slots(e: &mut Engine, base: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for &(slot, answer) in slots {
            e.mem.set_u32(vtable + slot, base + slot);
            e.register_double(base + slot, move |_, _| answer.into_ret());
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn copy_constructs_with_caster_spell_effect_and_copies_the_timer() {
        let mut e = Engine::new();
        let copy = e.mem.alloc(0x4c);
        e.register_double(ALLOCATE, move |_, _| copy.into_ret());
        e.register(CONCUSSION_EFFECT_CONSTRUCTOR, |_, a| a[0].into_ret());
        let this = object_with_slots(&mut e, 0x7000, &[(0x44, 0)]);
        e.mem.set_u32(this + 0x28, 0x28);
        e.mem.set_u32(this + 8, 0x08);
        e.mem.set_u32(this + 0xc, 0x0c);
        e.mem.set_u32(this + 0x48, 0x7f80_0001);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_bcb0, &[this]).u32(), copy);
        let log = e.call_log.take().unwrap();
        let built = log
            .iter()
            .find(|(a, _)| *a == CONCUSSION_EFFECT_CONSTRUCTOR)
            .unwrap();
        assert_eq!(built.1, vec![copy, 0x28, 0x08, 0x0c]);
        assert_eq!(e.mem.u32(copy + 0x48), 0x7fc0_0001);
        let copied = log.iter().find(|(a, _)| *a == 0x7044).unwrap();
        assert_eq!(copied.1, vec![this, copy]);
    }

    #[test]
    fn check_target_needs_an_actor_answering_false_twice() {
        for (first, second, expected) in [
            (0u32, 0u32, true),
            (1, 0, false),
            (0, 0x100, true),
            (0, 1, false),
        ] {
            let mut e = Engine::new();
            let actor = object_with_slots(&mut e, 0x7000, &[(0x22c, first), (0x2e8, second)]);
            e.register_double(GET_TARGET_AS_ACTOR, move |_, _| actor.into_ret());
            assert_eq!(e.call(0x0080_bd60, &[0, 0x10]).bool(), expected);
        }
        // A null target, or a target that is no actor.
        let mut e = Engine::new();
        e.register(GET_TARGET_AS_ACTOR, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0080_bd60, &[0, 0]).bool());
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == GET_TARGET_AS_ACTOR));
        assert!(!e.call(0x0080_bd60, &[0, 0x10]).bool());
    }

    /// The engine for the apply check: settings, constants, the player, and doubles.
    fn apply_engine(
        threshold: f32,
        value: f64,
        embedded: u32,
        slot360: u32,
        resting: u32,
    ) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x011d_1000,
            0x011c_f000,
            0x011d_b000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(ZERO, 0.0);
        let values = e.mem.alloc(8);
        e.mem.set_f32(values, threshold);
        e.mem.set_u32(values + 4, 0x7f80_0002);
        e.register_double(SETTING_VALUE, move |_, a| {
            let address = if a[0] == SETTING_011D1BA4 {
                values
            } else {
                values + 4
            };
            address.into_ret()
        });
        e.register_double(ACTOR_VALUE_008BE870, move |_, _| value.into_ret());
        e.register_double(IS_SLEEPING_OR_RESTING, move |_, _| resting.into_ret());
        e.register(GET_CONCUSSION_MODIFIER, |_, _| 0x6666u32.into_ret());
        e.register(TRIGGER_MODIFIER, |e, _| {
            assert_eq!(e.mem.u8(APPLYING_011DBE50), 1);
            Ret::default()
        });
        e.register(AUDIO_INSTANCE, |_, _| 0x5555u32.into_ret());
        e.register(GET_SOUND_HANDLE_BY_NAME, |_, a| a[1].into_ret());
        e.register(SOUND_HANDLE_PLAY, |_, _| Ret::default());
        e.register(SOUND_HANDLE_DESTRUCTOR, |_, _| Ret::default());
        let process = object_with_slots(&mut e, 0x9000, &[(0x44, 0)]);
        e.register_double(ACTOR_OBJECT_008D8520, move |_, _| process.into_ret());
        let actor = object_with_slots(&mut e, 0x7000, &[(0x360, slot360)]);
        let inner_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(inner_vtable + 8, 0x8008);
        e.register_double(0x8008, move |_, _| embedded.into_ret());
        e.mem.set_u32(actor + 0xa4, inner_vtable);
        e.mem.set_u32(PLAYER, 0x4444);
        let this = e.mem.alloc(0x4c);
        e.mem.set_f32(this + 0x48, 2.0);
        e.call_log = Some(vec![]);
        (e, this, actor)
    }

    fn called(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn apply_triggers_sound_and_reaction() {
        let (mut e, this, actor) = apply_engine(1.0, 0.0, 0, 1, 0);
        e.call(0x0080_be40, &[this, actor]);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(
            called(&e, TRIGGER_MODIFIER),
            vec![vec![0x6666, 0x7fc0_0002, 0]]
        );
        let sound = called(&e, GET_SOUND_HANDLE_BY_NAME);
        assert_eq!(sound[0][0], 0x5555);
        assert_eq!(sound[0][2..], [CRIPPLE_HEAD_SOUND, 0x101]);
        assert_eq!(called(&e, SOUND_HANDLE_PLAY), vec![vec![sound[0][1], 0]]);
        assert_eq!(called(&e, SOUND_HANDLE_DESTRUCTOR), vec![vec![sound[0][1]]]);
        assert_eq!(called(&e, IS_SLEEPING_OR_RESTING), vec![vec![0x4444]]);
        assert_eq!(called(&e, 0x8008), vec![vec![actor + 0xa4, 0x48]]);
        let reaction = called(&e, 0x9044);
        assert_eq!(reaction[0][1..], [actor, 0, 2, 1, 0, 1]);
        assert_eq!(e.mem.u8(APPLYING_011DBE50), 0);
    }

    #[test]
    fn apply_skips_the_effects_for_non_qualifying_actors() {
        // Slot +0x360 false, or the player resting: only the reaction.
        for (slot360, resting) in [(0u32, 0u32), (1, 1)] {
            let (mut e, this, actor) = apply_engine(1.0, 0.0, 0, slot360, resting);
            e.call(0x0080_be40, &[this, actor]);
            assert!(called(&e, TRIGGER_MODIFIER).is_empty());
            assert_eq!(called(&e, 0x9044).len(), 1);
        }
    }

    #[test]
    fn apply_stops_early() {
        // Timer not above the setting (equal, below, or NaN): nothing, the timer kept.
        for threshold in [2.0f32, 3.0, f32::NAN] {
            let (mut e, this, actor) = apply_engine(threshold, 0.0, 0, 1, 0);
            e.call(0x0080_be40, &[this, actor]);
            assert_eq!(e.mem.f32(this + 0x48), 2.0);
            assert!(called(&e, ACTOR_VALUE_008BE870).is_empty());
        }
        // The actor's value not 0.0 (or NaN), or the embedded object answering non-zero:
        // the timer is reset but nothing is applied.
        for (value, embedded) in [(1.0f64, 0u32), (f64::NAN, 0), (0.0, 1)] {
            let (mut e, this, actor) = apply_engine(1.0, value, embedded, 1, 0);
            e.call(0x0080_be40, &[this, actor]);
            assert_eq!(e.mem.u32(this + 0x48), 0);
            assert!(called(&e, 0x9044).is_empty());
            assert_eq!(e.mem.u8(APPLYING_011DBE50), 0);
        }
    }

    #[test]
    fn update_adds_time_and_checks_an_actor_target() {
        let (mut e, this, actor) = apply_engine(5.0, 0.0, 0, 0, 0);
        e.register(BASE_UPDATE_004534F0, |_, _| Ret::default());
        e.register(GET_MAGIC_TARGET_0059BB30, |_, _| 0x3000u32.into_ret());
        e.register_double(GET_TARGET_AS_ACTOR, move |_, _| actor.into_ret());
        e.call(0x0080_bdd0, &[this, 0.5f32.to_bits()]);
        assert_eq!(e.mem.f32(this + 0x48), 2.5);
        assert_eq!(
            called(&e, BASE_UPDATE_004534F0),
            vec![vec![this, 0.5f32.to_bits()]]
        );
        assert_eq!(called(&e, GET_TARGET_AS_ACTOR), vec![vec![0x3000]]);
        // The apply check ran (it read the threshold setting).
        assert_eq!(called(&e, SETTING_VALUE), vec![vec![SETTING_011D1BA4]]);

        // No magic target: no check.
        let (mut e, this, _) = apply_engine(5.0, 0.0, 0, 0, 0);
        e.register(BASE_UPDATE_004534F0, |_, _| Ret::default());
        e.register(GET_MAGIC_TARGET_0059BB30, |_, _| 0u32.into_ret());
        e.call(0x0080_bdd0, &[this, 0.5f32.to_bits()]);
        assert!(called(&e, SETTING_VALUE).is_empty());
    }
}

#[cfg(test)]
mod get_concussion_tests {
    use super::*;

    #[test]
    fn get_concussion_finds_the_modifier_once_when_enabled() {
        let mut e = Engine::new();
        e.map(0x0118_a000, 0x1000);
        e.map(0x011c_9000, 0x1000);
        e.register(LOOKUP_FORM_004839C0, |_, a| (a[0] + 0x1000).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| (a[0] + 4).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_bfa0, &[]).u32(), 0);
        e.mem.set_u8(CONCUSSION_ENABLED_0118ABB1, 1);
        assert_eq!(e.call(0x0080_bfa0, &[]).u32(), 0x1168);
        assert_eq!(e.call(0x0080_bfa0, &[]).u32(), 0x1168);
        let log = e.call_log.take().unwrap();
        let lookups: Vec<_> = log.iter().filter(|c| c.0 == LOOKUP_FORM_004839C0).collect();
        assert_eq!(lookups.len(), 1);
        assert_eq!(lookups[0].1, vec![0x164]);
        let casts: Vec<_> = log.iter().filter(|c| c.0 == RT_DYNAMIC_CAST).collect();
        assert_eq!(
            casts[0].1,
            vec![0x1164, 0, TES_FORM_TYPE, TES_IMAGE_SPACE_MODIFIER_TYPE, 0]
        );
    }
}

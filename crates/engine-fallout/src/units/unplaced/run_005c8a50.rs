//! `(unplaced)/run_005c8a50` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Three script command handlers (their `CommandInfo` entries in the
//! exe's command table name them: `SayTo` at `01191130`, `EvaluatePackage`
//! at `011917c0`, `ResetAI` at `01195820`) and `DialogueItem`'s scalar
//! deleting destructor.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesscriptfunctions::ScriptArgs;

/// `__RTDynamicCast` (`object, 0, source type, target type, 0`), `cdecl`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor of `TESObjectREFR` (`.?AVTESObjectREFR@@`).
const RTTI_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// RTTI type descriptor of `Actor` (`.?AVActor@@`).
const RTTI_ACTOR: u32 = 0x0118_46d4;
/// `Actor::EvaluatePackage` (engine map), `thiscall`, two words.
const ACTOR_EVALUATE_PACKAGE: u32 = 0x008a_6ce0;
/// A `bool` getter on the actor (`thiscall`, no arguments), checked first.
const ACTOR_CHECK_00440DA0: u32 = 0x0044_0da0;
/// A `bool` getter on the actor (`thiscall`, no arguments), checked second.
const ACTOR_CHECK_00440D80: u32 = 0x0044_0d80;

/// `DialogueItem::~DialogueItem` (engine map), `thiscall`.
const DIALOGUE_ITEM_DESTRUCTOR: u32 = 0x0083_c670;
/// The game's `operator delete` (`cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

/// The object at `011cd378` that `SayTo` reads a word from (`0043d4d0`
/// returns its address) and sets through `0045ce80` around its work.
const SAY_TO_GUARD_OBJECT: u32 = 0x011c_d378;
/// `0043d4d0`: `thiscall` on [`SAY_TO_GUARD_OBJECT`], returns the address of
/// the word `SayTo` saves.
const GUARD_GET: u32 = 0x0043_d4d0;
/// `0045ce80`: `thiscall` on [`SAY_TO_GUARD_OBJECT`], one word.
const GUARD_SET: u32 = 0x0045_ce80;
/// `MobileObject::StopCurrentDialogue` (engine map), `thiscall`.
const STOP_CURRENT_DIALOGUE: u32 = 0x0093_4250;
/// `008d8520` (the engine map's name, `MiddleHighProcess::GetSavedAcquireObject`,
/// is not confirmed for this body): `thiscall` on an actor, the object `SayTo`
/// calls its virtual slots on.
const ACTOR_PROCESS_008D8520: u32 = 0x008d_8520;
/// `MobileObject::GetCurrentProcessType` (engine map), `thiscall`.
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `0057bd60`: `thiscall` on an actor, one word (the other actor).
const ACTOR_WITH_OTHER_0057BD60: u32 = 0x0057_bd60;
/// `PlayerCharacter::IsSleepingorResting` (engine map), `thiscall`.
const IS_SLEEPING_OR_RESTING: u32 = 0x0094_df60;
/// The player (`PlayerCharacter *`) global.
const PLAYER: u32 = 0x011d_ea3c;
/// `0045cd60`: `thiscall`, no arguments, a word result.
const PROCESS_CHECK_0045CD60: u32 = 0x0045_cd60;
/// `Actor::GetEssential` (engine map), `thiscall`, `bool`.
const ACTOR_GET_ESSENTIAL: u32 = 0x0087_f3d0;
/// `004f8960`: `thiscall` on an actor, a word result compared with 6.
const ACTOR_STATE_004F8960: u32 = 0x004f_8960;
/// `TESTopic::CreateDialogueItem` (engine map), `thiscall`, five words.
const CREATE_DIALOGUE_ITEM: u32 = 0x0061_b320;
/// `DialogueItem::FirstResponse` (engine map), `thiscall`.
const DIALOGUE_ITEM_FIRST_RESPONSE: u32 = 0x0083_c7b0;
/// `DialogueItem::GetCurrentResponse` (engine map), `thiscall`.
const DIALOGUE_ITEM_GET_CURRENT_RESPONSE: u32 = 0x0083_c820;
/// `DialogueItem::RunResult` (engine map), `thiscall`, one word.
const DIALOGUE_ITEM_RUN_RESULT: u32 = 0x0083_c850;
/// `005d43c0`: `thiscall`, returns `this + 0x44`.
const ACTOR_PLUS_44: u32 = 0x005d_43c0;
/// `Script::SetActionFlag` (engine map), `cdecl`, three words.
const SCRIPT_SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `005e3fa0`: `thiscall` on the reference, returns an object with a vtable.
const REFR_OBJECT_005E3FA0: u32 = 0x005e_3fa0;
/// `TESObjectREFR::SetSayToTopic` (engine map), `thiscall`, one word.
const SET_SAY_TO_TOPIC: u32 = 0x0057_ad20;
/// `TESObjectREFR::SetSayToTopicInfo` (engine map), `thiscall`, one word.
const SET_SAY_TO_TOPIC_INFO: u32 = 0x0057_ace0;
/// `0084e3a0`: `thiscall`, no arguments, a word result.
const GET_VALUE_0084E3A0: u32 = 0x0084_e3a0;
/// `BSSoundHandle::SetCompletionCallback` (engine map), `thiscall`, two words.
const SOUND_HANDLE_SET_COMPLETION_CALLBACK: u32 = 0x00ad_8e60;
/// The completion callback `SayTo` gives the sound (`005790f0`), passed as
/// its address.
const SAY_TO_COMPLETION_CALLBACK: u32 = 0x0057_90f0;
/// `00483710`: destroys the 12-byte sound handle on the stack (`thiscall`).
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
/// Size of the sound handle `SayTo` keeps on its stack (`EBP-0x3c..-0x30`).
const SOUND_HANDLE_SIZE: u32 = 0x0c;

/// The two handlers' shared body: the actor `thisObj` is (by
/// `__RTDynamicCast`), unless one of its checks (`00440da0`, `00440d80`,
/// virtual slot `0x360`) is true, evaluates its package with
/// `Actor::EvaluatePackage(0, reset)`. Always true.
fn evaluate_package_unless_busy(e: &mut Engine, a: ScriptArgs, reset: u32) -> bool {
    let actor = e
        .call(
            DYNAMIC_CAST,
            &args![a.this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
        )
        .u32();
    if actor != 0
        && !e.call(ACTOR_CHECK_00440DA0, &args![actor]).bool()
        && !e.call(ACTOR_CHECK_00440D80, &args![actor]).bool()
        && !e.vcall(actor, 0x360, &args![]).bool()
    {
        e.call(ACTOR_EVALUATE_PACKAGE, &args![actor, 0u32, reset]);
    }
    true
}

// Translated from 005c90d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem`'s scalar deleting destructor: `~DialogueItem` (`0083c670`),
/// then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn dialogue_item_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DIALOGUE_ITEM_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 005c9100 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005c9100 - the C++ exception frame (FS:[0]) and the /GS cookie of the handler are not translated
/// The `SayTo` script command (its `CommandInfo` at `01191130`): parses a
/// reference (`target`), a topic and two optional integers; sets
/// `*result` to 0 first. False when the parameters do not parse.
///
/// Around its work it saves the word `0043d4d0` returns for the object at
/// `011cd378`, sets it to `0x7fffffff` (`0045ce80`) and restores it after.
/// When `thisObj` is an actor (virtual slot `0x100`) and a target was given,
/// the speaker's dialogue stops and, with both actors' `008d8520` objects
/// present and the speaker's slot `0x22c(1)` false, the two are turned to
/// each other (when the speaker's process type is 0) and the line is either
/// said through the speaker's slot `0x2a4` or, when the player is sleeping or
/// resting, `0045cd60` is non-zero, or the speaker is essential with
/// `004f8960` = 6, run at once as a dialogue item (first response, both
/// result scripts, the topic's action flag `0x40000`, then deleted). Without
/// that, when `thisObj` (or its `005e3fa0` object) answers slot `0x1d0`, the
/// topic is said through `thisObj`'s slot `0x13c` with a sound handle whose
/// completion callback is `005790f0`.
pub fn say_to(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    // The parsed locals, in the order passed: target, topic, third, fourth.
    let Some([target, topic, third_param, fourth_param]) = a.parse_into(e, [0, 0, 0, 0]) else {
        return false;
    };
    let say_flag = u32::from(third_param as i32 > 0);

    let saved_word = e.call(GUARD_GET, &args![SAY_TO_GUARD_OBJECT]).u32();
    let saved = e.mem.u32(saved_word);
    e.call(GUARD_SET, &args![SAY_TO_GUARD_OBJECT, 0x7fff_ffffu32]);

    let this_obj = a.this_obj.addr();
    let mut speaker = 0u32;
    if e.vcall(this_obj, 0x100, &args![]).bool() {
        speaker = this_obj;
    }

    if speaker != 0 && target != 0 {
        say_to_actor(e, speaker, target, topic, fourth_param, say_flag);
    } else if this_obj != 0 {
        let mut speaks = e.vcall(this_obj, 0x1d0, &args![]).u32() != 0;
        if !speaks && e.vcall(this_obj, 0xfc, &args![]).bool() {
            let object = e.call(REFR_OBJECT_005E3FA0, &args![this_obj]).u32();
            speaks = e.vcall(object, 0x1d0, &args![]).u32() != 0;
        }
        if speaks {
            e.call(SET_SAY_TO_TOPIC, &args![this_obj, topic]);
            let item = e
                .call(
                    CREATE_DIALOGUE_ITEM,
                    &args![topic, this_obj, 0u32, 0u32, 0u32, 0u32],
                )
                .u32();
            if item != 0 {
                let info = e.call(GET_VALUE_0084E3A0, &args![item]).u32();
                e.call(SET_SAY_TO_TOPIC_INFO, &args![this_obj, info]);
            }
            e.with_stack(SOUND_HANDLE_SIZE, |e, handle| {
                e.vcall(
                    this_obj,
                    0x13c,
                    &args![handle, topic, target, 0u32, 0u32, 0u32, 0u32, say_flag],
                );
                let word = e.call(GET_VALUE_0084E3A0, &args![this_obj]).u32();
                e.call(
                    SOUND_HANDLE_SET_COMPLETION_CALLBACK,
                    &args![handle, SAY_TO_COMPLETION_CALLBACK, word],
                );
                e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
            });
        }
    }

    e.call(GUARD_SET, &args![SAY_TO_GUARD_OBJECT, saved]);
    true
}

/// [`say_to`]'s branch for an actor speaker with a target (`005c91ee` to
/// `005c9412`).
fn say_to_actor(
    e: &mut Engine,
    speaker: u32,
    target: u32,
    topic: u32,
    fourth_param: u32,
    say_flag: u32,
) {
    e.call(STOP_CURRENT_DIALOGUE, &args![speaker]);
    if e.call(ACTOR_PROCESS_008D8520, &args![speaker]).u32() == 0 {
        return;
    }
    if e.vcall(speaker, 0x22c, &args![1u32]).bool() {
        return;
    }
    if e.call(ACTOR_PROCESS_008D8520, &args![target]).u32() == 0 {
        return;
    }
    let process = e.call(ACTOR_PROCESS_008D8520, &args![speaker]).u32();
    e.vcall(process, 0x370, &args![0u32]);
    let process = e.call(ACTOR_PROCESS_008D8520, &args![speaker]).u32();
    e.vcall(process, 0x224, &args![]);
    if e.call(GET_CURRENT_PROCESS_TYPE, &args![speaker]).u32() == 0 {
        if fourth_param == 0 {
            let process = e.call(ACTOR_PROCESS_008D8520, &args![target]).u32();
            e.vcall(process, 0x648, &args![0u32]);
            let process = e.call(ACTOR_PROCESS_008D8520, &args![target]).u32();
            e.vcall(process, 0x628, &args![speaker]);
            e.call(ACTOR_WITH_OTHER_0057BD60, &args![target, speaker]);
        }
        let process = e.call(ACTOR_PROCESS_008D8520, &args![speaker]).u32();
        e.vcall(process, 0x648, &args![0u32]);
        let process = e.call(ACTOR_PROCESS_008D8520, &args![speaker]).u32();
        e.vcall(process, 0x628, &args![target]);
    }

    let player = e.global::<u32>(PLAYER);
    let mut run_at_once = e.call(IS_SLEEPING_OR_RESTING, &args![player]).bool();
    if !run_at_once {
        let process = e.call(ACTOR_PROCESS_008D8520, &args![speaker]).u32();
        run_at_once = e.call(PROCESS_CHECK_0045CD60, &args![process]).u32() != 0;
    }
    if !run_at_once && e.call(ACTOR_GET_ESSENTIAL, &args![speaker]).bool() {
        run_at_once = e.call(ACTOR_STATE_004F8960, &args![speaker]).u32() == 6;
    }

    if run_at_once {
        let item = e
            .call(
                CREATE_DIALOGUE_ITEM,
                &args![topic, speaker, target, 0u32, 0u32, 1u32],
            )
            .u32();
        if item != 0 {
            e.call(DIALOGUE_ITEM_FIRST_RESPONSE, &args![item]);
            e.call(DIALOGUE_ITEM_GET_CURRENT_RESPONSE, &args![item]);
            e.call(DIALOGUE_ITEM_RUN_RESULT, &args![item, 0u32]);
            e.call(DIALOGUE_ITEM_RUN_RESULT, &args![item, 1u32]);
            let flags = e.call(ACTOR_PLUS_44, &args![speaker]).u32();
            e.call(SCRIPT_SET_ACTION_FLAG, &args![topic, flags, 0x40000u32]);
            dialogue_item_scalar_deleting_destructor(e, Ptr::new(item), 1);
        }
    } else {
        e.call(ACTOR_WITH_OTHER_0057BD60, &args![speaker, target]);
        let process = e.call(ACTOR_PROCESS_008D8520, &args![speaker]).u32();
        e.vcall(
            process,
            0x2a4,
            &args![speaker, topic, say_flag, 0u32, 1u32, 1u32],
        );
    }
}

// Translated from 005c9530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ResetAI` script command (its `CommandInfo` at `01195820`): on an
/// actor `thisObj` that none of `00440da0`, `00440d80` and virtual slot
/// `0x360` holds back, `Actor::EvaluatePackage(0, 1)`. Always true.
pub fn reset_ai(e: &mut Engine, a: ScriptArgs) -> bool {
    evaluate_package_unless_busy(e, a, 1)
}

// Translated from 005c95a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `EvaluatePackage` (`evp`) script command (its `CommandInfo` at
/// `011917c0`): as [`reset_ai`] with `Actor::EvaluatePackage(0, 0)`.
pub fn evaluate_package(e: &mut Engine, a: ScriptArgs) -> bool {
    evaluate_package_unless_busy(e, a, 0)
}

/// `-1.0` (`float`), stored at +0x78 of the speaking actor.
const MINUS_ONE: u32 = 0x0101_2054;
/// `TESObjectREFR::CreateReference` (engine map, `00564480`, `cdecl`: form type, 0),
/// `TESForm::SetTemporary` (`00484490`), `TESObjectREFR::SetObjectReference`
/// (`00575690`) and `TESObjectREFR::SetLocationOnReference` (`00575830`), all `ECX` = the
/// new reference.
const CREATE_REFERENCE: u32 = 0x0056_4480;
const SET_TEMPORARY: u32 = 0x0048_4490;
const SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;

/// Says `topic` through `speaker`'s vtable slot +0x13c (with the sound handle `handle`,
/// the `listener` word and the flags), the topic and its info set on `this_obj` first
/// (`SetSayToTopic`, and `SetSayToTopicInfo` with the `0084e3a0` word of the dialogue item
/// `CreateDialogueItem` makes for `speaker`), then the handle's completion callback.
#[allow(clippy::too_many_arguments)]
fn say_through(
    e: &mut Engine,
    this_obj: u32,
    speaker: u32,
    listener: u32,
    topic: u32,
    subtitles: u32,
    flag_c: u32,
    flag_a: u32,
    handle: Ptr,
) {
    e.call(SET_SAY_TO_TOPIC, &args![this_obj, topic]);
    let item = e
        .call(
            CREATE_DIALOGUE_ITEM,
            &args![topic, speaker, 0u32, 0u32, 0u32, 0u32],
        )
        .u32();
    if item != 0 {
        let info = e.call(GET_VALUE_0084E3A0, &args![item]).u32();
        e.call(SET_SAY_TO_TOPIC_INFO, &args![this_obj, info]);
    }
    e.vcall(
        speaker,
        0x13c,
        &args![handle, topic, listener, subtitles, flag_c, 0u32, 0u32, flag_a],
    );
    let word = e.call(GET_VALUE_0084E3A0, &args![this_obj]).u32();
    e.call(
        SOUND_HANDLE_SET_COMPLETION_CALLBACK,
        &args![handle, SAY_TO_COMPLETION_CALLBACK, word],
    );
}

// Translated from 005c8a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005c8a50 - the C++ exception frame (FS:[0] chain, handler at 00ef95a8, the sound handles' unwind states) and the frame's cookie word are not translated
/// A script command handler (the standard eight-word `cdecl` signature; no code reference,
/// so reached through the command table; its `CommandInfo` entry was not looked up, so the
/// command's name is not confirmed here; the body is the `Say`-style counterpart of
/// [`say_to`]): sets `*result` to 0, parses a topic, an integer `a`, a reference, and the
/// integers `b` and `c`, false when they do not parse. With a topic, and the word
/// `0043d4d0` answers for `011cd378` saved and set to `0x7fffffff` (`0045ce80`) meanwhile:
///
/// - `thisObj` an actor (`__RTDynamicCast` to `Actor`) with a `008d8520` object whose slot
///   +0x22c(1) answers false: -1.0 at the actor's +0x78, the object's slots +0x370(0) and
///   +0xac; with process type 0 (`GetCurrentProcessType`) the object's slots +0x674(2),
///   +0x648(0), +0x628 (with +0x674's answer) and +0x224; otherwise, with `b` nonzero, the
///   topic is said through the actor's slot +0x408 (handle, topic, actor, `b != 0`,
///   `c > 0`, 0, 0) with its completion callback, and true is answered at once (the saved
///   word is not restored on this path). Then the line is run at once as a dialogue item
///   (first response, both result scripts, the action flag 0x40000, deleted) when the
///   player sleeps or rests, the object's `0045cd60` is nonzero, or the actor is essential
///   with `004f8960` = 6; else the object's slot +0x2a4 (actor, topic, `a > 0`, 0, 1, 1).
/// - Such an actor whose slot +0x22c(1) answers true, or without the object: when it has
///   the object, the topic's action flag 0x40000 and the object's slot +0x88(0).
/// - A non-actor `thisObj` with a reference given: a temporary reference of form type
///   0x3b (`CreateReference`) for that reference at `thisObj`'s slot +0x1f4 location says
///   the topic (slot +0x13c with itself as the listener), then is deleted (its slot +0x10
///   with 1).
/// - Otherwise, when `thisObj` (or its `005e3fa0` object) answers slot +0x1d0, `thisObj`
///   says it (slot +0x13c with no listener).
///
/// Then the saved word is restored; true.
pub fn fn_005c8a50(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([topic, int_a, reference, int_b, int_c]) = a.parse_into(e, [0, 0, 0, 0, 0]) else {
        return false;
    };
    let flag_a = u32::from(int_a as i32 > 0);
    let flag_b = u32::from(int_b != 0);
    let flag_c = u32::from(int_c as i32 > 0);
    let this_obj = a.this_obj.addr();
    let actor = e
        .call(
            DYNAMIC_CAST,
            &args![this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
        )
        .u32();
    let saved_word = e.call(GUARD_GET, &args![SAY_TO_GUARD_OBJECT]).u32();
    let saved = e.mem.u32(saved_word);
    e.call(GUARD_SET, &args![SAY_TO_GUARD_OBJECT, 0x7fff_ffffu32]);

    if topic != 0 {
        if actor != 0 {
            let has_process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32() != 0;
            if has_process && !e.vcall(actor, 0x22c, &args![1u32]).bool() {
                let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
                e.mem.set_u32(actor + 0x78, minus_one);
                let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
                e.vcall(process, 0x370, &args![0u32]);
                let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
                e.vcall(process, 0xac, &args![]);
                if e.call(GET_CURRENT_PROCESS_TYPE, &args![actor]).u32() == 0 {
                    let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
                    let value = e.vcall(process, 0x674, &args![2u32]).u32();
                    let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
                    e.vcall(process, 0x648, &args![0u32]);
                    let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
                    e.vcall(process, 0x628, &args![value]);
                    let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
                    e.vcall(process, 0x224, &args![]);
                } else if int_b != 0 {
                    e.call(SET_SAY_TO_TOPIC, &args![actor, topic]);
                    let item = e
                        .call(
                            CREATE_DIALOGUE_ITEM,
                            &args![topic, actor, 0u32, 0u32, 0u32, 0u32],
                        )
                        .u32();
                    if item != 0 {
                        let info = e.call(GET_VALUE_0084E3A0, &args![item]).u32();
                        e.call(SET_SAY_TO_TOPIC_INFO, &args![this_obj, info]);
                    }
                    e.with_stack(SOUND_HANDLE_SIZE, |e, handle| {
                        e.vcall(
                            actor,
                            0x408,
                            &args![handle, topic, actor, flag_b, flag_c, 0u32, 0u32],
                        );
                        let word = e.call(GET_VALUE_0084E3A0, &args![this_obj]).u32();
                        e.call(
                            SOUND_HANDLE_SET_COMPLETION_CALLBACK,
                            &args![handle, SAY_TO_COMPLETION_CALLBACK, word],
                        );
                        e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
                    });
                    return true;
                }
                let player = e.global::<u32>(PLAYER);
                let mut run_at_once = e.call(IS_SLEEPING_OR_RESTING, &args![player]).bool();
                if !run_at_once {
                    let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
                    run_at_once = e.call(PROCESS_CHECK_0045CD60, &args![process]).u32() != 0;
                }
                if !run_at_once && e.call(ACTOR_GET_ESSENTIAL, &args![actor]).bool() {
                    run_at_once = e.call(ACTOR_STATE_004F8960, &args![actor]).u32() == 6;
                }
                if run_at_once {
                    let item = e
                        .call(
                            CREATE_DIALOGUE_ITEM,
                            &args![topic, actor, 0u32, 0u32, 0u32, 1u32],
                        )
                        .u32();
                    if item != 0 {
                        e.call(DIALOGUE_ITEM_FIRST_RESPONSE, &args![item]);
                        e.call(DIALOGUE_ITEM_GET_CURRENT_RESPONSE, &args![item]);
                        e.call(DIALOGUE_ITEM_RUN_RESULT, &args![item, 0u32]);
                        e.call(DIALOGUE_ITEM_RUN_RESULT, &args![item, 1u32]);
                        let flags = e.call(ACTOR_PLUS_44, &args![actor]).u32();
                        e.call(SCRIPT_SET_ACTION_FLAG, &args![topic, flags, 0x40000u32]);
                        dialogue_item_scalar_deleting_destructor(e, Ptr::new(item), 1);
                    }
                } else {
                    let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
                    e.vcall(
                        process,
                        0x2a4,
                        &args![actor, topic, flag_a, 0u32, 1u32, 1u32],
                    );
                }
            } else if e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32() != 0 {
                let flags = e.call(ACTOR_PLUS_44, &args![actor]).u32();
                e.call(SCRIPT_SET_ACTION_FLAG, &args![topic, flags, 0x40000u32]);
                let process = e.call(ACTOR_PROCESS_008D8520, &args![actor]).u32();
                e.vcall(process, 0x88, &args![0u32]);
            }
        } else if this_obj != 0 && reference != 0 {
            let temporary = e.call(CREATE_REFERENCE, &args![0x3bu32, 0u32]).u32();
            e.call(SET_TEMPORARY, &args![temporary]);
            e.call(SET_OBJECT_REFERENCE, &args![temporary, reference]);
            let location = e.vcall(this_obj, 0x1f4, &args![]).u32();
            e.call(SET_LOCATION_ON_REFERENCE, &args![temporary, location]);
            e.with_stack(SOUND_HANDLE_SIZE, |e, handle| {
                say_through(
                    e, this_obj, temporary, temporary, topic, flag_b, flag_c, flag_a, handle,
                );
                if temporary != 0 {
                    e.vcall(temporary, 0x10, &args![1u32]);
                }
                e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
            });
        } else {
            let mut speaks = e.vcall(this_obj, 0x1d0, &args![]).u32() != 0;
            if !speaks && e.vcall(this_obj, 0xfc, &args![]).bool() {
                let object = e.call(REFR_OBJECT_005E3FA0, &args![this_obj]).u32();
                speaks = e.vcall(object, 0x1d0, &args![]).u32() != 0;
            }
            if speaks {
                e.with_stack(SOUND_HANDLE_SIZE, |e, handle| {
                    say_through(
                        e, this_obj, this_obj, 0, topic, flag_b, flag_c, flag_a, handle,
                    );
                    e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
                });
            }
        }
    }

    e.call(GUARD_SET, &args![SAY_TO_GUARD_OBJECT, saved]);
    true
}

/// `00952830` (`playercharacter.cpp`; `ECX` = the player, three stack words).
const PLAYER_00952830: u32 = 0x0095_2830;

// Translated from 005c9610 (decompiled, FalloutNV.exe 1.4.0.525)
/// A script command handler (the standard eight-word `cdecl` signature; no code reference;
/// its `CommandInfo` entry was not looked up): parses one word (0 before parsing) and,
/// when the parameters parse, hands it to `00952830` on the player with 1 and 0. Answers
/// whether they parsed (`*result` is not written).
pub fn fn_005c9610(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = a.parse_into(e, [0]) else {
        return false;
    };
    let player: u32 = e.global(PLAYER);
    e.call(PLAYER_00952830, &args![player, value, 1u32, 0u32]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005c_9610, fn_005c9610(ScriptArgs) -> bool),
        entry!(0x005c_8a50, fn_005c8a50(ScriptArgs) -> bool),
        entry!(0x005c_90d0, dialogue_item_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x005c_9100, say_to(ScriptArgs) -> bool),
        entry!(0x005c_9530, reset_ai(ScriptArgs) -> bool),
        entry!(0x005c_95a0, evaluate_package(ScriptArgs) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const PARSE_PARAMETERS: u32 = 0x005a_ccb0;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn script_args(this_obj: u32, result: u32) -> Vec<u32> {
        vec![0x10, 0x20, this_obj, 0x40, 0x50, 0x60, result, 0x80]
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        e.register(DIALOGUE_ITEM_DESTRUCTOR, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005c_90d0, &args![0x5000u32, 1u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, DIALOGUE_ITEM_DESTRUCTOR), vec![vec![0x5000]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x5000]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005c_90d0, &args![0x5000u32, 2u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, DIALOGUE_ITEM_DESTRUCTOR).len(), 1);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    /// An actor at a fresh block whose vtable slot 0x360 returns `busy`.
    fn evp_setup(e: &mut Engine, busy: bool) -> u32 {
        let actor = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(actor, vtable);
        e.mem.set_u32(vtable + 0x360, 0x7000_0360);
        if busy {
            e.register(0x7000_0360, |_, _| 1u32.into_ret());
        } else {
            e.register(0x7000_0360, |_, _| 0u32.into_ret());
        }
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(ACTOR_CHECK_00440DA0, |_, _| 0u32.into_ret());
        e.register(ACTOR_CHECK_00440D80, |_, _| 0u32.into_ret());
        e.register(ACTOR_EVALUATE_PACKAGE, |_, _| Ret::default());
        actor
    }

    #[test]
    fn reset_ai_and_evaluate_package_pass_their_flag() {
        let mut e = Engine::new();
        let actor = evp_setup(&mut e, false);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005c_9530, &script_args(actor, 0)).bool());
        assert_eq!(
            calls_to(&e, ACTOR_EVALUATE_PACKAGE),
            vec![vec![actor, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![actor, 0, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0]]
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x005c_95a0, &script_args(actor, 0)).bool());
        assert_eq!(
            calls_to(&e, ACTOR_EVALUATE_PACKAGE),
            vec![vec![actor, 0, 0]]
        );
    }

    #[test]
    fn evaluate_package_skips_busy_actors_and_non_actors() {
        let mut e = Engine::new();
        let actor = evp_setup(&mut e, true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005c_95a0, &script_args(actor, 0)).bool());
        assert!(calls_to(&e, ACTOR_EVALUATE_PACKAGE).is_empty());
        // The first check true: nothing else is asked.
        e.register(ACTOR_CHECK_00440DA0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x005c_9530, &script_args(actor, 0)).bool());
        assert!(calls_to(&e, ACTOR_CHECK_00440D80).is_empty());
        // Not an actor: no checks at all.
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x005c_9530, &script_args(actor, 0)).bool());
        assert!(calls_to(&e, ACTOR_CHECK_00440DA0).is_empty());
    }

    struct SayTo {
        speaker: u32,
        target: u32,
        process: u32,
        result: u32,
    }

    /// Doubles for every callee of `SayTo`; the parser writes `parsed`.
    fn say_to_setup(e: &mut Engine, parsed: [u32; 4], is_actor: bool) -> SayTo {
        let speaker = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x800);
        e.mem.set_u32(speaker, vtable);
        for slot in [0x100u32, 0x22c, 0x1d0, 0xfc, 0x13c] {
            e.mem.set_u32(vtable + slot, 0x7100_0000 + slot);
            e.register(0x7100_0000 + slot, |_, _| 0u32.into_ret());
        }
        if is_actor {
            e.register(0x7100_0100, |_, _| 1u32.into_ret());
        }
        let process = e.mem.alloc(0x10);
        let process_vtable = e.mem.alloc(0x800);
        e.mem.set_u32(process, process_vtable);
        for slot in [0x370u32, 0x224, 0x648, 0x628, 0x2a4, 0x1d0] {
            e.mem.set_u32(process_vtable + slot, 0x7200_0000 + slot);
            e.register(0x7200_0000 + slot, |_, _| Ret::default());
        }
        let result = e.mem.alloc(8);
        e.mem.set_f64(result, 5.0);
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            for (i, value) in parsed.iter().enumerate() {
                e.mem.set_u32(a[7 + i], *value);
            }
            1u32.into_ret()
        });
        let guard_word = e.mem.alloc(4);
        e.mem.set_u32(guard_word, 0x1234);
        e.register_double(GUARD_GET, move |_, _| guard_word.into_ret());
        e.register(GUARD_SET, |_, _| Ret::default());
        e.register(STOP_CURRENT_DIALOGUE, |_, _| Ret::default());
        e.register_double(ACTOR_PROCESS_008D8520, move |_, _| process.into_ret());
        e.register(GET_CURRENT_PROCESS_TYPE, |_, _| 0u32.into_ret());
        e.register(ACTOR_WITH_OTHER_0057BD60, |_, _| Ret::default());
        e.register(IS_SLEEPING_OR_RESTING, |_, _| 0u32.into_ret());
        e.map(PLAYER, 4);
        e.set_global(PLAYER, 0x9999u32);
        e.register(PROCESS_CHECK_0045CD60, |_, _| 0u32.into_ret());
        e.register(ACTOR_GET_ESSENTIAL, |_, _| 0u32.into_ret());
        e.register(ACTOR_STATE_004F8960, |_, _| 0u32.into_ret());
        e.register(CREATE_DIALOGUE_ITEM, |_, _| 0x6600u32.into_ret());
        e.register(DIALOGUE_ITEM_FIRST_RESPONSE, |_, _| Ret::default());
        e.register(DIALOGUE_ITEM_GET_CURRENT_RESPONSE, |_, _| Ret::default());
        e.register(DIALOGUE_ITEM_RUN_RESULT, |_, _| Ret::default());
        e.register(ACTOR_PLUS_44, |_, a| (a[0] + 0x44).into_ret());
        e.register(SCRIPT_SET_ACTION_FLAG, |_, _| Ret::default());
        e.register(DIALOGUE_ITEM_DESTRUCTOR, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(REFR_OBJECT_005E3FA0, |_, _| Ret::default());
        e.register(SET_SAY_TO_TOPIC, |_, _| Ret::default());
        e.register(SET_SAY_TO_TOPIC_INFO, |_, _| Ret::default());
        e.register(GET_VALUE_0084E3A0, |_, a| (a[0] ^ 1).into_ret());
        e.register(SOUND_HANDLE_SET_COMPLETION_CALLBACK, |_, _| Ret::default());
        e.register(SOUND_HANDLE_DESTRUCTOR, |_, _| Ret::default());
        SayTo {
            speaker,
            target: parsed[0],
            process,
            result,
        }
    }

    #[test]
    fn say_to_fails_when_parameters_do_not_parse() {
        let mut e = Engine::new();
        let s = say_to_setup(&mut e, [0; 4], true);
        e.register(PARSE_PARAMETERS, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x005c_9100, &script_args(s.speaker, s.result))
            .bool());
        assert_eq!(e.mem.f64(s.result), 0.0);
        assert!(calls_to(&e, GUARD_SET).is_empty());
        let parse = calls_to(&e, PARSE_PARAMETERS);
        assert_eq!(
            parse[0][..7],
            [0x10, 0x20, 0x80, s.speaker, 0x40, 0x50, 0x60]
        );
        assert_eq!(parse[0].len(), 11);
    }

    #[test]
    fn say_to_actor_with_target_says_through_the_process() {
        let mut e = Engine::new();
        let s = say_to_setup(&mut e, [0x7700, 0x8800, 3, 0], true);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x005c_9100, &script_args(s.speaker, s.result))
            .bool());
        assert_eq!(e.mem.f64(s.result), 0.0);
        assert_eq!(
            calls_to(&e, GUARD_SET),
            vec![
                vec![SAY_TO_GUARD_OBJECT, 0x7fff_ffff],
                vec![SAY_TO_GUARD_OBJECT, 0x1234]
            ]
        );
        assert_eq!(calls_to(&e, STOP_CURRENT_DIALOGUE), vec![vec![s.speaker]]);
        // Both turned to each other (fourth parameter 0).
        assert_eq!(
            calls_to(&e, 0x7200_0628),
            vec![vec![s.process, s.speaker], vec![s.process, s.target]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_WITH_OTHER_0057BD60),
            vec![vec![s.target, s.speaker], vec![s.speaker, s.target]]
        );
        assert_eq!(
            calls_to(&e, 0x7200_02a4),
            vec![vec![s.process, s.speaker, 0x8800, 1, 0, 1, 1]]
        );
        assert!(calls_to(&e, CREATE_DIALOGUE_ITEM).is_empty());
    }

    #[test]
    fn say_to_runs_the_item_at_once_while_the_player_sleeps() {
        let mut e = Engine::new();
        let s = say_to_setup(&mut e, [0x7700, 0x8800, 0, 1], true);
        e.register(IS_SLEEPING_OR_RESTING, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x005c_9100, &script_args(s.speaker, s.result))
            .bool());
        // Fourth parameter set: only the speaker turns.
        assert_eq!(calls_to(&e, 0x7200_0628), vec![vec![s.process, s.target]]);
        assert_eq!(
            calls_to(&e, CREATE_DIALOGUE_ITEM),
            vec![vec![0x8800, s.speaker, s.target, 0, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, DIALOGUE_ITEM_RUN_RESULT),
            vec![vec![0x6600, 0], vec![0x6600, 1]]
        );
        assert_eq!(
            calls_to(&e, SCRIPT_SET_ACTION_FLAG),
            vec![vec![0x8800, s.speaker + 0x44, 0x40000]]
        );
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x6600]]);
        assert!(calls_to(&e, 0x7200_02a4).is_empty());
    }

    #[test]
    fn say_to_essential_actor_state_6_runs_at_once() {
        let mut e = Engine::new();
        let s = say_to_setup(&mut e, [0x7700, 0x8800, 0, 0], true);
        e.register(ACTOR_GET_ESSENTIAL, |_, _| 1u32.into_ret());
        e.register(ACTOR_STATE_004F8960, |_, _| 6u32.into_ret());
        e.register(GET_CURRENT_PROCESS_TYPE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x005c_9100, &script_args(s.speaker, s.result))
            .bool());
        assert!(calls_to(&e, 0x7200_0628).is_empty());
        assert_eq!(calls_to(&e, CREATE_DIALOGUE_ITEM).len(), 1);
        // Essential but another state: said through the process.
        e.register(ACTOR_STATE_004F8960, |_, _| 5u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x005c_9100, &script_args(s.speaker, s.result))
            .bool());
        assert!(calls_to(&e, CREATE_DIALOGUE_ITEM).is_empty());
        assert_eq!(calls_to(&e, 0x7200_02a4).len(), 1);
    }

    #[test]
    fn say_to_actor_stops_when_slot_22c_is_true() {
        let mut e = Engine::new();
        let s = say_to_setup(&mut e, [0x7700, 0x8800, 0, 0], true);
        e.register(0x7100_022c, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x005c_9100, &script_args(s.speaker, s.result))
            .bool());
        assert_eq!(calls_to(&e, ACTOR_PROCESS_008D8520).len(), 1);
        assert!(calls_to(&e, 0x7200_0370).is_empty());
        assert_eq!(calls_to(&e, GUARD_SET).len(), 2);
    }

    #[test]
    fn say_to_reference_says_with_a_sound_handle() {
        let mut e = Engine::new();
        let s = say_to_setup(&mut e, [0, 0x8800, 2, 0], false);
        e.register(0x7100_01d0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x005c_9100, &script_args(s.speaker, s.result))
            .bool());
        assert_eq!(
            calls_to(&e, SET_SAY_TO_TOPIC),
            vec![vec![s.speaker, 0x8800]]
        );
        assert_eq!(
            calls_to(&e, CREATE_DIALOGUE_ITEM),
            vec![vec![0x8800, s.speaker, 0, 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, SET_SAY_TO_TOPIC_INFO),
            vec![vec![s.speaker, 0x6601]]
        );
        let say = calls_to(&e, 0x7100_013c);
        assert_eq!(say.len(), 1);
        let handle = say[0][1];
        assert_eq!(say[0][2..], [0x8800, 0, 0, 0, 0, 0, 1]);
        assert_eq!(
            calls_to(&e, SOUND_HANDLE_SET_COMPLETION_CALLBACK),
            vec![vec![handle, SAY_TO_COMPLETION_CALLBACK, s.speaker ^ 1]]
        );
        assert_eq!(calls_to(&e, SOUND_HANDLE_DESTRUCTOR), vec![vec![handle]]);
    }

    #[test]
    fn say_to_reference_asks_its_object_when_it_does_not_speak() {
        let mut e = Engine::new();
        let s = say_to_setup(&mut e, [0, 0x8800, 0, 0], false);
        e.register(0x7100_00fc, |_, _| 1u32.into_ret());
        let process = s.process;
        e.register_double(REFR_OBJECT_005E3FA0, move |_, _| process.into_ret());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x005c_9100, &script_args(s.speaker, s.result))
            .bool());
        assert_eq!(calls_to(&e, 0x7200_01d0), vec![vec![s.process]]);
        assert!(calls_to(&e, SET_SAY_TO_TOPIC).is_empty());
        e.register(0x7200_01d0, |_, _| 1u32.into_ret());
        e.register(CREATE_DIALOGUE_ITEM, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x005c_9100, &script_args(s.speaker, s.result))
            .bool());
        assert_eq!(calls_to(&e, SET_SAY_TO_TOPIC).len(), 1);
        assert!(calls_to(&e, SET_SAY_TO_TOPIC_INFO).is_empty());
        assert_eq!(calls_to(&e, 0x7100_013c).len(), 1);
    }
}

#[cfg(test)]
mod say_tests {
    use super::*;

    const PARSE_PARAMETERS: u32 = 0x005a_ccb0;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    struct Say {
        speaker: u32,
        process: u32,
        result: u32,
    }

    /// `thisObj` at a fresh block with a vtable of doubles (slot +0x22c answers
    /// `refuses`, slot +0x1d0 answers `speaks`); `actor` says whether the cast answers it.
    fn setup(e: &mut Engine, parsed: [u32; 5], actor: bool, refuses: bool, speaks: bool) -> Say {
        let speaker = e.mem.alloc(0x80);
        let vtable = e.mem.alloc(0x800);
        e.mem.set_u32(speaker, vtable);
        for slot in [0x10u32, 0x22c, 0x1d0, 0xfc, 0x13c, 0x408, 0x1f4] {
            e.mem.set_u32(vtable + slot, 0x7100_0000 + slot);
            e.register(0x7100_0000 + slot, |_, _| 0u32.into_ret());
        }
        if refuses {
            e.register(0x7100_022c, |_, _| 1u32.into_ret());
        }
        if speaks {
            e.register(0x7100_01d0, |_, _| 1u32.into_ret());
        }
        e.register(0x7100_01f4, |_, _| 0x4400u32.into_ret());
        let process = e.mem.alloc(0x10);
        let process_vtable = e.mem.alloc(0x800);
        e.mem.set_u32(process, process_vtable);
        for slot in [0x370u32, 0xac, 0x674, 0x648, 0x628, 0x224, 0x2a4, 0x88] {
            e.mem.set_u32(process_vtable + slot, 0x7200_0000 + slot);
            e.register(0x7200_0000 + slot, |_, _| 0x55u32.into_ret());
        }
        let result = e.mem.alloc(8);
        e.mem.set_f64(result, 5.0);
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            for (i, value) in parsed.iter().enumerate() {
                e.mem.set_u32(a[7 + i], *value);
            }
            1u32.into_ret()
        });
        e.register_double(DYNAMIC_CAST, move |_, a| {
            (if actor { a[0] } else { 0 }).into_ret()
        });
        let guard_word = e.mem.alloc(4);
        e.mem.set_u32(guard_word, 0x1234);
        e.register_double(GUARD_GET, move |_, _| guard_word.into_ret());
        e.register(GUARD_SET, |_, _| Ret::default());
        e.register_double(ACTOR_PROCESS_008D8520, move |_, _| process.into_ret());
        e.register(GET_CURRENT_PROCESS_TYPE, |_, _| 1u32.into_ret());
        e.register(IS_SLEEPING_OR_RESTING, |_, _| 0u32.into_ret());
        e.map(0x0101_2000, 0x1000);
        e.set_global(MINUS_ONE, (-1.0f32).to_bits());
        e.map(PLAYER, 4);
        e.set_global(PLAYER, 0x9999u32);
        e.register(PROCESS_CHECK_0045CD60, |_, _| 0u32.into_ret());
        e.register(ACTOR_GET_ESSENTIAL, |_, _| 0u32.into_ret());
        e.register(ACTOR_STATE_004F8960, |_, _| 0u32.into_ret());
        e.register(CREATE_DIALOGUE_ITEM, |_, _| 0x6600u32.into_ret());
        for callee in [
            DIALOGUE_ITEM_FIRST_RESPONSE,
            DIALOGUE_ITEM_GET_CURRENT_RESPONSE,
            DIALOGUE_ITEM_RUN_RESULT,
            SCRIPT_SET_ACTION_FLAG,
            DIALOGUE_ITEM_DESTRUCTOR,
            OPERATOR_DELETE,
            SET_SAY_TO_TOPIC,
            SET_SAY_TO_TOPIC_INFO,
            SOUND_HANDLE_SET_COMPLETION_CALLBACK,
            SOUND_HANDLE_DESTRUCTOR,
            SET_TEMPORARY,
            SET_OBJECT_REFERENCE,
            SET_LOCATION_ON_REFERENCE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(ACTOR_PLUS_44, |_, a| (a[0] + 0x44).into_ret());
        e.register(REFR_OBJECT_005E3FA0, |_, _| Ret::default());
        e.register(GET_VALUE_0084E3A0, |_, a| (a[0] ^ 1).into_ret());
        e.call_log = Some(vec![]);
        Say {
            speaker,
            process,
            result,
        }
    }

    fn run(e: &mut Engine, s: &Say) -> bool {
        e.call(
            0x005c_8a50,
            &[0x10, 0x20, s.speaker, 0x40, 0x50, 0x60, s.result, 0x80],
        )
        .bool()
    }

    #[test]
    fn an_actor_says_the_topic_through_its_process() {
        let mut e = Engine::new();
        let s = setup(&mut e, [0x3000, 1, 0, 0, 0], true, false, false);
        assert!(run(&mut e, &s));
        assert_eq!(e.mem.f64(s.result), 0.0);
        assert_eq!(e.mem.f32(s.speaker + 0x78), -1.0);
        assert_eq!(
            calls_to(&e, 0x7200_02a4),
            vec![vec![s.process, s.speaker, 0x3000, 1, 0, 1, 1]]
        );
        assert_eq!(
            calls_to(&e, GUARD_SET).last().unwrap(),
            &vec![SAY_TO_GUARD_OBJECT, 0x1234]
        );
    }

    #[test]
    fn an_actor_with_process_type_zero_turns_then_may_run_the_line_at_once() {
        let mut e = Engine::new();
        let s = setup(&mut e, [0x3000, 0, 0, 0, 0], true, false, false);
        e.register(GET_CURRENT_PROCESS_TYPE, |_, _| 0u32.into_ret());
        e.register(IS_SLEEPING_OR_RESTING, |_, _| 1u32.into_ret());
        assert!(run(&mut e, &s));
        assert_eq!(calls_to(&e, 0x7200_0628), vec![vec![s.process, 0x55]]);
        assert_eq!(calls_to(&e, DIALOGUE_ITEM_RUN_RESULT).len(), 2);
        assert_eq!(
            calls_to(&e, SCRIPT_SET_ACTION_FLAG),
            vec![vec![0x3000, s.speaker + 0x44, 0x40000]]
        );
        assert!(calls_to(&e, 0x7200_02a4).is_empty());
    }

    #[test]
    fn an_actor_with_b_says_through_slot_0x408_and_skips_the_restore() {
        let mut e = Engine::new();
        let s = setup(&mut e, [0x3000, 0, 0, 7, 2], true, false, false);
        assert!(run(&mut e, &s));
        let said = calls_to(&e, 0x7100_0408);
        assert_eq!(said.len(), 1);
        assert_eq!(said[0][0], s.speaker);
        assert_eq!(said[0][2..], [0x3000, s.speaker, 1, 1, 0, 0]);
        assert_eq!(calls_to(&e, GUARD_SET).len(), 1);
    }

    #[test]
    fn a_refusing_actor_only_gets_the_action_flag() {
        let mut e = Engine::new();
        let s = setup(&mut e, [0x3000, 0, 0, 0, 0], true, true, false);
        assert!(run(&mut e, &s));
        assert_eq!(calls_to(&e, 0x7200_0088), vec![vec![s.process, 0]]);
        assert!(calls_to(&e, 0x7200_0370).is_empty());
    }

    #[test]
    fn a_non_actor_with_a_reference_speaks_through_a_temporary_reference() {
        let mut e = Engine::new();
        let s = setup(&mut e, [0x3000, 1, 0x5000, 0, 0], false, false, false);
        let temporary = s.speaker;
        e.register_double(CREATE_REFERENCE, move |_, _| temporary.into_ret());
        assert!(run(&mut e, &s));
        assert_eq!(calls_to(&e, CREATE_REFERENCE), vec![vec![0x3b, 0]]);
        assert_eq!(
            calls_to(&e, SET_OBJECT_REFERENCE),
            vec![vec![temporary, 0x5000]]
        );
        assert_eq!(
            calls_to(&e, SET_LOCATION_ON_REFERENCE),
            vec![vec![temporary, 0x4400]]
        );
        let said = calls_to(&e, 0x7100_013c);
        assert_eq!(said[0][2..], [0x3000, temporary, 0, 0, 0, 0, 1]);
        assert_eq!(calls_to(&e, 0x7100_0010), vec![vec![temporary, 1]]);
    }

    #[test]
    fn a_speaking_non_actor_says_the_topic_itself() {
        let mut e = Engine::new();
        let s = setup(&mut e, [0x3000, 0, 0, 1, 1], false, false, true);
        assert!(run(&mut e, &s));
        let said = calls_to(&e, 0x7100_013c);
        assert_eq!(said[0][2..], [0x3000, 0, 1, 1, 0, 0, 0]);
        // Without a topic nothing but the guard happens.
        let mut e = Engine::new();
        let s = setup(&mut e, [0, 0, 0, 0, 0], false, false, true);
        assert!(run(&mut e, &s));
        assert!(calls_to(&e, 0x7100_013c).is_empty());
        assert_eq!(calls_to(&e, GUARD_SET).len(), 2);
    }

    #[test]
    fn unparsed_parameters_answer_false() {
        let mut e = Engine::new();
        let s = setup(&mut e, [0; 5], false, false, false);
        e.register(PARSE_PARAMETERS, |_, _| 0u32.into_ret());
        assert!(!run(&mut e, &s));
        assert!(calls_to(&e, GUARD_GET).is_empty());
    }
}

#[cfg(test)]
mod player_command_tests {
    use super::*;

    const PARSE_PARAMETERS: u32 = 0x005a_ccb0;

    #[test]
    fn fn_005c9610_hands_the_parsed_word_to_the_player() {
        let mut e = Engine::new();
        e.map(PLAYER, 4);
        e.set_global(PLAYER, 0x9999u32);
        e.register(PARSE_PARAMETERS, |e, a| {
            e.mem.set_u32(a[7], 0x77);
            1u32.into_ret()
        });
        e.register(PLAYER_00952830, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let words = [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80];
        assert!(e.call(0x005c_9610, &words).bool());
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == PLAYER_00952830).collect();
        assert_eq!(calls[0].1, vec![0x9999, 0x77, 1, 0]);
        e.register(PARSE_PARAMETERS, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005c_9610, &words).bool());
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == PLAYER_00952830));
    }
}

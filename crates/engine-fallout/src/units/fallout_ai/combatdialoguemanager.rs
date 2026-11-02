//! `fallout/ai/combat/combatdialoguemanager.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The setting at `011f1764` and its value address (`00408d60`,
/// `thiscall`): a byte that turns combat dialogue off.
const DIALOGUE_OFF_SETTING: u32 = 0x011f_1764;
const SETTING_VALUE: u32 = 0x0040_8d60;
/// The current time (`00457fe0`, `cdecl`, no words), the time of the last
/// combat line (`011f170c`) and the gap (1500) between two lines.
const CURRENT_TIME: u32 = 0x0045_7fe0;
const LAST_LINE_TIME: u32 = 0x011f_170c;
const LINE_GAP: u32 = 0x5dc;
/// The player (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// The controller's +0x9C object (`00586150`) and group (`004fb070`), both
/// `thiscall`.
const CONTROLLER_STATE: u32 = 0x0058_6150;
const CONTROLLER_GROUP: u32 = 0x004f_b070;
/// The holders of the timers the dialogue types use: on the controller's
/// object `00503650` (subtypes 0 and 1), `00984250` (3) and `00984230` (4),
/// on the group `00624700` (5); type 4 uses [`fn_00984210`].
const STATE_TIMER_LOW: u32 = 0x0050_3650;
const STATE_TIMER_3: u32 = 0x0098_4250;
const STATE_TIMER_4: u32 = 0x0098_4230;
const GROUP_TIMER_5: u32 = 0x0062_4700;
/// The timer's "elapsed" test (`008d7f80`, `thiscall`, `bool`) and restart
/// (`008d7f40`, `thiscall(float)`).
const TIMER_ELAPSED: u32 = 0x008d_7f80;
const TIMER_START: u32 = 0x008d_7f40;
/// The debug print (`005b5e40`, `cdecl(format, ...)`) and
/// `"COMBAT: Unknown dialogue type %i"` (`0108d250`).
const DEBUG_PRINT: u32 = 0x005b_5e40;
const UNKNOWN_TYPE_FORMAT: u32 = 0x0108_d250;
/// On the actor: `00437bd0` (`bool`) and `MobileObject::GetCurrentProcessType`
/// (engine map; `00931850`), either of which stops the line.
const ACTOR_TEST: u32 = 0x0043_7bd0;
const CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map; `008d8520`,
/// `thiscall`): the object whose slots 0x30C, 0x20C, 0x22C, 0x48C and 0x2A4
/// the code calls.
const SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// `0041ca90` (`thiscall`): compared with 0x1C and 0x1A.
const OBJECT_KIND: u32 = 0x0041_ca90;
/// The setting value address (`00403e20`, `thiscall` on the setting) and
/// `RandomFloat` (engine map; `00476b70`, `cdecl(float, float)`, ST0).
const SETTING_ADDRESS: u32 = 0x0040_3e20;
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `1048576.0` (`float`, `0108d24c`): the starting squared distance of the
/// nearest-speaker search.
const FAR_AWAY: u32 = 0x0108_d24c;
/// The group's member count (`00990890`, `thiscall`) and member
/// (`00903600`, `thiscall(index)`).
const GROUP_COUNT: u32 = 0x0099_0890;
const GROUP_MEMBER: u32 = 0x0090_3600;
/// `TESObjectREFR::GetWorldLocation` (engine map; `00575650`,
/// `thiscall(out)`, a 0x10-byte location) and
/// `BGSWorldLocation::GetDistanceSquared` (engine map; `00527a40`,
/// `thiscall(other)`, ST0).
const GET_WORLD_LOCATION: u32 = 0x0057_5650;
const DISTANCE_SQUARED: u32 = 0x0052_7a40;
/// `TESTopic::GetTopic` (engine map; `0061a2d0`, `cdecl(type, subtype)`)
/// and `TESTopic::CreateDialogueItem` (engine map; `0061b320`,
/// `thiscall(actor, word, topic, word, word)`).
const GET_TOPIC: u32 = 0x0061_a2d0;
const CREATE_DIALOGUE_ITEM: u32 = 0x0061_b320;
/// `DialogueItem::FirstResponse`, `GetCurrentResponse` and `RunResult`
/// (engine map; `0083c7b0`, `0083c820`, `0083c850`) and the item's scalar
/// deleting destructor (`005c90d0`, `thiscall(flags)`).
const FIRST_RESPONSE: u32 = 0x0083_c7b0;
const CURRENT_RESPONSE: u32 = 0x0083_c820;
const RUN_RESULT: u32 = 0x0083_c850;
const ITEM_DELETE: u32 = 0x005c_90d0;
/// `00967ac0` (`thiscall(word)`) on the player.
const PLAYER_CALL: u32 = 0x0096_7ac0;
/// The response's words for slot 0x284 of the actor: `006815c0` then
/// `004048e0`, `0084e3a0`, `0044ddc0`, and `00460140` then `00559450`.
const RESPONSE_ITEM: u32 = 0x0068_15c0;
const RESPONSE_TEXT: u32 = 0x0040_48e0;
const RESPONSE_WORD_A: u32 = 0x0084_e3a0;
const RESPONSE_WORD_B: u32 = 0x0044_ddc0;
const RESPONSE_SOUND: u32 = 0x0046_0140;
const POINTER_GET: u32 = 0x0055_9450;
/// `BSSoundHandle::SetCompletionCallback` (engine map; `00ad8e60`,
/// `thiscall(callback, word)`) with the callback `00967aa0`, and the
/// handle's destructor (`00483710`).
const SET_COMPLETION_CALLBACK: u32 = 0x00ad_8e60;
const COMPLETION_CALLBACK: u32 = 0x0096_7aa0;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// `004030b0` (`thiscall`) and `0097ae90` (`thiscall`: the word at +0xBC)
/// on the controller, the two words `StartDialogue` passes on.
const CONTROLLER_WORD: u32 = 0x0040_30b0;
const CONTROLLER_ACTOR: u32 = 0x0097_ae90;

/// The two settings whose values `RandomFloat` gets for a dialogue type and
/// subtype (the first is read first and passed second), when the type has
/// them. Type 2 is the compiler's switch (`009841ec`) as arms.
fn delay_settings(dialogue_type: u32, subtype: u32) -> Option<(u32, u32)> {
    match dialogue_type {
        2 => match subtype {
            0 | 1 => Some((0x011c_e4c8, 0x011c_fb20)),
            3 => Some((0x011c_e248, 0x011c_eba4)),
            4 => Some((0x011c_fcbc, 0x011c_ed40)),
            5 => Some((0x011c_f898, 0x011c_f858)),
            _ => None,
        },
        4 => {
            if (subtype as i32) < 0 || subtype as i32 > 1 {
                Some((0x011c_fa4c, 0x011c_f0b0))
            } else {
                Some((0x011c_e35c, 0x011c_f24c))
            }
        }
        _ => None,
    }
}

/// A setting's `float` as `FLD`/`FSTP` copy it.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_ADDRESS, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(value))
}

/// The object slot `slot` of `process` returns, when not null.
fn slot_object(e: &mut Engine, process: u32, slot: u32) -> Option<u32> {
    let object = e.vcall(process, slot, &[]).u32();
    if object == 0 {
        None
    } else {
        Some(object)
    }
}

// Translated from 00983970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatDialogueManager::StartDialogue` (Xbox PDB;
/// `thiscall(controller, type, subtype, flag)`): [`fn_009839b0`] with the
/// controller's actor (`0097ae90`), its `004030b0` word, the type, subtype
/// and flag, and the controller.
pub fn combat_dialogue_manager_start_dialogue(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    dialogue_type: u32,
    subtype: u32,
    flag: u8,
) -> bool {
    let word = e.call(CONTROLLER_WORD, &args![controller]).u32();
    let actor = e.call(CONTROLLER_ACTOR, &args![controller]).u32();
    fn_009839b0(
        e,
        this,
        actor,
        word,
        dialogue_type,
        subtype,
        flag,
        controller,
    )
}

// Translated from 009839b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009839b0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatDialogueManager::StartDialogue` (Xbox PDB, the second overload;
/// `thiscall(actor, word, type, subtype, flag, controller)`; `this` is not
/// read): has `actor` say the combat topic of `type`/`subtype`. False when
/// the setting at `011f1764` is set, when the actor's slot 0x100 is false,
/// or within 1500 of the last line (`00457fe0` against `011f170c`, which
/// then takes the time). A null `controller` becomes the actor's slot 0x428.
/// The type's timer (type 2 by subtype on the controller's object or group,
/// type 4 [`fn_00984210`]; any other type prints
/// `"COMBAT: Unknown dialogue type %i"`) must have elapsed (`008d7f80`), and
/// `00437bd0` and `GetCurrentProcessType` must be clear. Without `flag`,
/// the actor's process object (`008d8520`) must not be busy (slots 0x30C,
/// 0x20C, kinds 0x1C/0x1A of slots 0x22C/0x20C) and the player not have the
/// +0x608 byte set; with `flag`, the line is said directly (not through the
/// nearest speaker) when slot 0x214 is set or slot 0x20C holds a kind 0x1A.
/// The timer restarts with `RandomFloat` of the type's two settings. Then,
/// speaking through others, the nearest group member (squared distance
/// below 1048576, not the actor or the player, not busy) is the speaker
/// (else the actor). With the topic (`TESTopic::GetTopic`; none: false): the
/// player makes a dialogue item and, with a first response, plays it
/// (`00967ac0(1)`, slot 0x48C's sound handle, slot 0x284, the completion
/// callback `00967aa0`, `RunResult(0)`) and deletes the item, returning
/// false; anyone else says it through slot 0x280 with the speaker (its
/// result), or directly through the process object's slot 0x2A4 (true).
pub fn fn_009839b0(
    e: &mut Engine,
    _this: Ptr,
    actor: u32,
    word: u32,
    dialogue_type: u32,
    subtype: u32,
    flag: u8,
    controller: u32,
) -> bool {
    let off = e.call(SETTING_VALUE, &args![DIALOGUE_OFF_SETTING]).u32();
    if e.mem.u8(off) != 0 {
        return false;
    }
    if !e.vcall(actor, 0x100, &[]).bool() {
        return false;
    }
    let now = e.call(CURRENT_TIME, &[]).u32();
    let last: u32 = e.global(LAST_LINE_TIME);
    if now.wrapping_sub(last) < LINE_GAP {
        return false;
    }
    e.set_global(LAST_LINE_TIME, now);
    let mut controller = controller;
    if controller == 0 {
        controller = e.vcall(actor, 0x428, &[]).u32();
    }
    let mut timer = 0u32;
    if controller != 0 {
        let state = e.call(CONTROLLER_STATE, &args![controller]).u32();
        let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
        if dialogue_type == 2 {
            // The compiler's switch (`009841d4`) as arms; subtype 2 and
            // anything above 5 have no timer.
            match subtype {
                0 | 1 => timer = e.call(STATE_TIMER_LOW, &args![state]).u32(),
                3 => timer = e.call(STATE_TIMER_3, &args![state]).u32(),
                4 => timer = e.call(STATE_TIMER_4, &args![state]).u32(),
                5 => timer = e.call(GROUP_TIMER_5, &args![group]).u32(),
                _ => {}
            }
        } else if dialogue_type == 4 {
            timer = fn_00984210(e, Ptr::new(group), subtype);
        } else {
            e.call(DEBUG_PRINT, &args![UNKNOWN_TYPE_FORMAT, dialogue_type]);
        }
    }
    if timer != 0 && !e.call(TIMER_ELAPSED, &args![timer]).bool() {
        return false;
    }
    if e.call(ACTOR_TEST, &args![actor]).bool()
        || e.call(CURRENT_PROCESS_TYPE, &args![actor]).u32() != 0
    {
        return false;
    }
    let player: u32 = e.global(PLAYER);
    let mut through_speaker = true;
    if flag == 0 {
        let process = e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
        if e.vcall(process, 0x30c, &[]).bool() {
            return false;
        }
        let process = e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
        if e.vcall(process, 0x20c, &[]).u32() != 0 {
            return false;
        }
        let process = e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
        if let Some(object) = slot_object(e, process, 0x22c) {
            if e.call(OBJECT_KIND, &args![object]).u32() == 0x1c {
                return false;
            }
        }
        let process = e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
        if let Some(object) = slot_object(e, process, 0x20c) {
            if e.call(OBJECT_KIND, &args![object]).u32() == 0x1c
                || e.call(OBJECT_KIND, &args![object]).u32() == 0x1a
            {
                return false;
            }
        }
        if actor == player {
            let player: u32 = e.global(PLAYER);
            if fn_00984270(e, Ptr::new(player)) != 0 {
                return false;
            }
        }
    } else if e.vcall(actor, 0x214, &[]).u32() != 0 {
        through_speaker = false;
    } else {
        let process = e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
        if let Some(object) = slot_object(e, process, 0x20c) {
            if e.call(OBJECT_KIND, &args![object]).u32() == 0x1a {
                through_speaker = false;
            }
        }
    }
    if timer != 0 {
        let mut delay = 0.0f32.to_bits();
        if let Some((first, second)) = delay_settings(dialogue_type, subtype) {
            let low = setting_bits(e, first);
            let high = setting_bits(e, second);
            delay = x87_store32(e.call(RANDOM_FLOAT, &args![high, low]).f64());
        } else if dialogue_type != 2 && dialogue_type != 4 {
            e.call(DEBUG_PRINT, &args![UNKNOWN_TYPE_FORMAT, dialogue_type]);
        }
        e.call(TIMER_START, &args![timer, x87_quiet_bits(delay)]);
    }
    let mut speaker = 0u32;
    if through_speaker && controller != 0 {
        let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
        let mut nearest = x87_quiet_bits(e.global::<u32>(FAR_AWAY));
        let count = e.call(GROUP_COUNT, &args![group]).u32();
        e.with_stack(0x20, |e, locations| {
            let here = locations.addr();
            let there = here + 0x10;
            e.call(GET_WORLD_LOCATION, &args![actor, here]);
            for index in 0..count {
                let member = e.call(GROUP_MEMBER, &args![group, index]).u32();
                if member == actor || member == e.global::<u32>(PLAYER) {
                    continue;
                }
                let process = e.call(SAVED_ACQUIRE_OBJECT, &args![member]).u32();
                if e.vcall(process, 0x30c, &[]).bool() {
                    continue;
                }
                let process = e.call(SAVED_ACQUIRE_OBJECT, &args![member]).u32();
                if let Some(object) = slot_object(e, process, 0x22c) {
                    if e.call(OBJECT_KIND, &args![object]).u32() == 0x1c {
                        continue;
                    }
                }
                let process = e.call(SAVED_ACQUIRE_OBJECT, &args![member]).u32();
                if let Some(object) = slot_object(e, process, 0x20c) {
                    if e.call(OBJECT_KIND, &args![object]).u32() == 0x1c {
                        continue;
                    }
                }
                e.call(GET_WORLD_LOCATION, &args![member, there]);
                let distance = e.call(DISTANCE_SQUARED, &args![there, here]).f64();
                let distance = x87_store32(distance);
                if x87_load32(nearest) > x87_load32(distance) {
                    speaker = member;
                    nearest = x87_quiet_bits(distance);
                }
            }
        });
    }
    if speaker == 0 {
        speaker = actor;
    }
    let topic = e.call(GET_TOPIC, &args![dialogue_type, subtype]).u32();
    if topic == 0 {
        return false;
    }
    if actor == e.global::<u32>(PLAYER) {
        let item = e
            .call(
                CREATE_DIALOGUE_ITEM,
                &args![topic, actor, word, topic, 0u32, 1u32],
            )
            .u32();
        if item != 0 {
            if e.call(FIRST_RESPONSE, &args![item]).bool() {
                let response = e.call(CURRENT_RESPONSE, &args![item]).u32();
                if response != 0 {
                    let player: u32 = e.global(PLAYER);
                    e.call(PLAYER_CALL, &args![player, 1u32]);
                    let process = e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
                    e.with_stack(0xc, |e, handle| {
                        e.vcall(process, 0x48c, &args![handle, 0u32]);
                        let entry = e.call(RESPONSE_ITEM, &args![response]).u32();
                        let text = e.call(RESPONSE_TEXT, &args![entry]).u32();
                        let word_a = e.call(RESPONSE_WORD_A, &args![response]).u32();
                        let word_b = e.call(RESPONSE_WORD_B, &args![response]).u32();
                        let sound = e.call(RESPONSE_SOUND, &args![response]).u32();
                        let sound = e.call(POINTER_GET, &args![sound]).u32();
                        // The slot returns a `float` the exe discards (`FSTP ST0`).
                        e.vcall(
                            actor,
                            0x284,
                            &args![
                                sound, handle, word_b, word_a, text, 0u32, 0u32, 0u32, 0u32, 0u32,
                                0u32, 0u32, 1u32
                            ],
                        );
                        e.call(
                            SET_COMPLETION_CALLBACK,
                            &args![handle, COMPLETION_CALLBACK, 0u32],
                        );
                        e.call(RUN_RESULT, &args![item, 0u32]);
                        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
                    });
                }
            }
            e.call(ITEM_DELETE, &args![item, 1u32]);
        }
        return false;
    }
    if !through_speaker {
        let process = e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
        e.vcall(process, 0x2a4, &args![actor, topic, 0u32, 0u32, 0u32, 0u32]);
        return true;
    }
    e.vcall(
        actor,
        0x280,
        &args![speaker, 0u32, 0u32, 0u32, 1u32, 1u32, topic, 1u32, 0u32],
    )
    .bool()
}

// Translated from 00984210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(index)`): the address of the
/// 8-byte entry `index` at +0x58.
pub fn fn_00984210(_e: &mut Engine, this: Ptr, index: u32) -> u32 {
    this.addr()
        .wrapping_add(index.wrapping_mul(8))
        .wrapping_add(0x58)
}

// Translated from 00984270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the byte at +0x608.
pub fn fn_00984270(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x608)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0098_3970,
            combat_dialogue_manager_start_dialogue(Ptr, u32, u32, u32, u8) -> bool
        ),
        entry!(
            0x0098_39b0,
            fn_009839b0(Ptr, u32, u32, u32, u32, u8, u32) -> bool
        ),
        entry!(0x0098_4210, fn_00984210(Ptr, u32) -> u32),
        entry!(0x0098_4270, fn_00984270(Ptr) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0200_0000;
    const SLOTS: [u32; 10] = [
        0x100, 0x428, 0x214, 0x280, 0x284, 0x30c, 0x20c, 0x22c, 0x48c, 0x2a4,
    ];
    const CLOCK: u32 = 0x011f_1800;

    fn slot(offset: u32) -> u32 {
        0x0300_0000 + offset
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

    fn ret_f64(v: f64) -> Ret {
        Ret {
            st0: v,
            ..Ret::default()
        }
    }

    /// An object of the shared vtable: +4 slot 0x100's answer, +8 slot
    /// 0x428's controller, +0xC slot 0x214, +0x10 slot 0x30C, +0x14 slot
    /// 0x20C's object, +0x18 slot 0x22C's object, +0x1C its `0041ca90` kind,
    /// +0x20 its process object, +0x24 slot 0x280's answer, +0x28 its
    /// position (`float`).
    fn object(e: &mut Engine) -> u32 {
        let p = e.mem.alloc(0x700);
        e.mem.set_u32(p, VTABLE);
        e.mem.set_u32(p + 4, 1);
        let process = e.mem.alloc(0x40);
        e.mem.set_u32(process, VTABLE);
        e.mem.set_u32(p + 0x20, process);
        e.mem.set_u32(p + 0x24, 1);
        p
    }

    /// The engine, an actor, its controller (+0x9C its object, +0xA0 its
    /// group; the group's count at +0 and members from +4) and the object.
    fn world() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x490 / 4];
        for offset in SLOTS {
            slots[offset as usize / 4] = slot(offset);
        }
        e.put_vtable(VTABLE, &slots);
        e.map(0x011f_1000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0108_d000, 0x1000);
        e.map(0x011c_e000, 0x2000);
        e.set_global(FAR_AWAY, 1_048_576.0f32);
        e.set_global(CLOCK, 5000u32);
        e.register(SETTING_VALUE, |_, a| a[0].into_ret());
        e.register(SETTING_ADDRESS, |_, a| a[0].into_ret());
        e.register(CURRENT_TIME, |e, _| e.mem.u32(CLOCK).into_ret());
        e.register(slot(0x100), |e, a| e.mem.u8(a[0] + 4).into_ret());
        e.register(slot(0x428), |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(slot(0x214), |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(slot(0x30c), |e, a| e.mem.u8(a[0] + 0x10).into_ret());
        e.register(slot(0x20c), |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(slot(0x22c), |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(slot(0x280), |e, a| e.mem.u8(a[0] + 0x24).into_ret());
        for offset in [0x284, 0x48c, 0x2a4] {
            e.register(slot(offset), |_, _| Ret::default());
        }
        e.register(OBJECT_KIND, |e, a| e.mem.u32(a[0] + 0x1c).into_ret());
        e.register(SAVED_ACQUIRE_OBJECT, |e, a| {
            e.mem.u32(a[0] + 0x20).into_ret()
        });
        e.register(ACTOR_TEST, |_, _| false.into_ret());
        e.register(CURRENT_PROCESS_TYPE, |_, _| 0u32.into_ret());
        e.register(CONTROLLER_STATE, |e, a| e.mem.u32(a[0] + 0x9c).into_ret());
        e.register(CONTROLLER_GROUP, |e, a| e.mem.u32(a[0] + 0xa0).into_ret());
        // Timers: a byte that says it elapsed.
        e.register(STATE_TIMER_LOW, |_, a| (a[0] + 0x10).into_ret());
        e.register(STATE_TIMER_3, |_, a| (a[0] + 0x13).into_ret());
        e.register(STATE_TIMER_4, |_, a| (a[0] + 0x14).into_ret());
        e.register(GROUP_TIMER_5, |_, a| (a[0] + 0x15).into_ret());
        e.register(TIMER_ELAPSED, |e, a| (e.mem.u8(a[0]) == 0).into_ret());
        e.register(TIMER_START, |_, _| Ret::default());
        e.register(RANDOM_FLOAT, |_, a| {
            ret_f64(f64::from(f32::from_bits(a[0])) + f64::from(f32::from_bits(a[1])))
        });
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        e.register(GROUP_COUNT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GROUP_MEMBER, |e, a| {
            e.mem.u32(a[0] + 4 + 4 * a[1]).into_ret()
        });
        e.register(GET_WORLD_LOCATION, |e, a| {
            let x = e.mem.u32(a[0] + 0x28);
            e.mem.set_u32(a[1], x);
            Ret::default()
        });
        e.register(DISTANCE_SQUARED, |e, a| {
            let d = f64::from(e.mem.f32(a[0])) - f64::from(e.mem.f32(a[1]));
            ret_f64(d * d)
        });
        e.register(GET_TOPIC, |_, a| {
            if a[0] == 9 { 0u32 } else { 0x7700 }.into_ret()
        });
        let actor = object(&mut e);
        let controller = e.mem.alloc(0x100);
        let state = e.mem.alloc(0x40);
        let group = e.mem.alloc(0x200);
        e.mem.set_u32(controller + 0x9c, state);
        e.mem.set_u32(controller + 0xa0, group);
        e.call_log = Some(vec![]);
        (e, actor, controller, group)
    }

    fn say(e: &mut Engine, actor: u32, kind: (u32, u32), flag: u8, controller: u32) -> bool {
        e.call(
            0x0098_39b0,
            &args![0u32, actor, 0x44u32, kind.0, kind.1, flag, controller],
        )
        .bool()
    }

    #[test]
    fn the_setting_the_actor_and_the_clock_gate_every_line() {
        let (mut e, actor, controller, _) = world();
        e.mem.set_u8(DIALOGUE_OFF_SETTING, 1);
        assert!(!say(&mut e, actor, (2, 0), 0, controller));
        assert!(calls(&e, slot(0x100)).is_empty());
        e.mem.set_u8(DIALOGUE_OFF_SETTING, 0);
        e.mem.set_u8(actor + 4, 0);
        assert!(!say(&mut e, actor, (2, 0), 0, controller));
        assert!(calls(&e, CURRENT_TIME).is_empty());
        e.mem.set_u8(actor + 4, 1);
        e.set_global(LAST_LINE_TIME, 4000u32);
        assert!(!say(&mut e, actor, (2, 0), 0, controller));
        assert_eq!(e.global::<u32>(LAST_LINE_TIME), 4000);
        e.set_global(LAST_LINE_TIME, 3500u32);
        assert!(say(&mut e, actor, (2, 2), 0, controller));
        assert_eq!(e.global::<u32>(LAST_LINE_TIME), 5000);
    }

    #[test]
    fn each_type_restarts_its_timer_with_its_settings() {
        let (mut e, actor, controller, group) = world();
        let state = e.mem.u32(controller + 0x9c);
        let cases = [
            ((2, 1), state + 0x10, 0x011c_e4c8, 0x011c_fb20),
            ((2, 3), state + 0x13, 0x011c_e248, 0x011c_eba4),
            ((2, 4), state + 0x14, 0x011c_fcbc, 0x011c_ed40),
            ((2, 5), group + 0x15, 0x011c_f898, 0x011c_f858),
            ((4, 1), group + 0x58 + 8, 0x011c_e35c, 0x011c_f24c),
            ((4, 7), group + 0x58 + 56, 0x011c_fa4c, 0x011c_f0b0),
        ];
        for (kind, timer, low, high) in cases {
            e.set_global(LAST_LINE_TIME, 0u32);
            e.mem.set_f32(low, 1.0);
            e.mem.set_f32(high, 2.5);
            e.call_log = Some(vec![]);
            assert!(say(&mut e, actor, kind, 0, controller), "{kind:?}");
            assert_eq!(calls(&e, TIMER_ELAPSED), vec![vec![timer]], "{kind:?}");
            assert_eq!(
                calls(&e, RANDOM_FLOAT),
                vec![vec![2.5f32.to_bits(), 1.0f32.to_bits()]],
                "{kind:?}"
            );
            assert_eq!(calls(&e, TIMER_START), vec![vec![timer, 3.5f32.to_bits()]]);
            // A timer that has not elapsed stops the line.
            e.mem.set_u8(timer, 1);
            e.set_global(LAST_LINE_TIME, 0u32);
            assert!(!say(&mut e, actor, kind, 0, controller));
            e.mem.set_u8(timer, 0);
        }
        // An unknown type: printed once, no timer; no topic for type 9.
        e.set_global(LAST_LINE_TIME, 0u32);
        e.call_log = Some(vec![]);
        assert!(!say(&mut e, actor, (9, 0), 0, controller));
        assert_eq!(calls(&e, DEBUG_PRINT), vec![vec![UNKNOWN_TYPE_FORMAT, 9]]);
        assert!(calls(&e, TIMER_START).is_empty());
        assert_eq!(calls(&e, GET_TOPIC), vec![vec![9, 0]]);
        // Without a controller argument, slot 0x428 gives it.
        e.set_global(LAST_LINE_TIME, 0u32);
        e.mem.set_u32(actor + 8, controller);
        e.call_log = Some(vec![]);
        assert!(say(&mut e, actor, (2, 3), 0, 0));
        assert_eq!(calls(&e, TIMER_ELAPSED), vec![vec![state + 0x13]]);
    }

    #[test]
    fn a_busy_actor_does_not_speak() {
        let (mut e, actor, controller, _) = world();
        let process = e.mem.u32(actor + 0x20);
        let busy = |e: &mut Engine| {
            e.set_global(LAST_LINE_TIME, 0u32);
            !say(e, actor, (2, 2), 0, controller)
        };
        e.register(ACTOR_TEST, |_, _| true.into_ret());
        assert!(busy(&mut e));
        e.register(ACTOR_TEST, |_, _| false.into_ret());
        e.register(CURRENT_PROCESS_TYPE, |_, _| 1u32.into_ret());
        assert!(busy(&mut e));
        e.register(CURRENT_PROCESS_TYPE, |_, _| 0u32.into_ret());
        e.mem.set_u8(process + 0x10, 1);
        assert!(busy(&mut e));
        e.mem.set_u8(process + 0x10, 0);
        let other = object(&mut e);
        e.mem.set_u32(process + 0x14, other);
        assert!(busy(&mut e));
        e.mem.set_u32(process + 0x14, 0);
        e.mem.set_u32(process + 0x18, other);
        e.mem.set_u32(other + 0x1c, 0x1c);
        assert!(busy(&mut e));
        e.mem.set_u32(other + 0x1c, 0x1a);
        assert!(!busy(&mut e));
        // The player with the +0x608 byte set.
        e.set_global(PLAYER, actor);
        e.mem.set_u8(actor + 0x608, 1);
        assert!(busy(&mut e));
    }

    #[test]
    fn the_nearest_free_member_speaks() {
        let (mut e, actor, controller, group) = world();
        e.mem.set_f32(actor + 0x28, 0.0);
        let members: Vec<u32> = (0..4).map(|_| object(&mut e)).collect();
        for (member, x) in members.iter().zip([30.0f32, 10.0, 5.0, 20.0]) {
            e.mem.set_f32(member + 0x28, x);
        }
        // Member 2 is busy (slot 0x30C); the actor is in the list too.
        let busy_process = e.mem.u32(members[2] + 0x20);
        e.mem.set_u8(busy_process + 0x10, 1);
        e.mem.set_u32(group, 5);
        for (i, member) in [members[0], actor, members[1], members[2], members[3]]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(group + 4 + 4 * i as u32, *member);
        }
        assert!(say(&mut e, actor, (2, 2), 0, controller));
        assert_eq!(
            calls(&e, slot(0x280)),
            vec![vec![actor, members[1], 0, 0, 0, 1, 1, 0x7700, 1, 0]]
        );
        // Nobody within reach: the actor speaks.
        for member in &members {
            e.mem.set_f32(member + 0x28, 5000.0);
        }
        e.set_global(LAST_LINE_TIME, 0u32);
        e.call_log = Some(vec![]);
        e.mem.set_u8(actor + 0x24, 0);
        assert!(!say(&mut e, actor, (2, 2), 0, controller));
        assert_eq!(calls(&e, slot(0x280))[0][1], actor);
    }

    #[test]
    fn a_direct_line_goes_through_the_process_object() {
        let (mut e, actor, controller, _) = world();
        e.mem.set_u32(actor + 0xc, 1);
        assert!(say(&mut e, actor, (2, 2), 1, controller));
        let process = e.mem.u32(actor + 0x20);
        assert_eq!(
            calls(&e, slot(0x2a4)),
            vec![vec![process, actor, 0x7700, 0, 0, 0, 0]]
        );
        assert!(calls(&e, GROUP_COUNT).is_empty());
        // Slot 0x20C holding a kind 0x1A does the same.
        e.mem.set_u32(actor + 0xc, 0);
        let other = object(&mut e);
        e.mem.set_u32(other + 0x1c, 0x1a);
        e.mem.set_u32(process + 0x14, other);
        e.set_global(LAST_LINE_TIME, 0u32);
        assert!(say(&mut e, actor, (2, 2), 1, controller));
        assert_eq!(calls(&e, slot(0x2a4)).len(), 2);
    }

    #[test]
    fn the_player_plays_a_dialogue_item() {
        let (mut e, actor, controller, _) = world();
        e.set_global(PLAYER, actor);
        e.register(CREATE_DIALOGUE_ITEM, |_, _| 0x8800u32.into_ret());
        e.register(FIRST_RESPONSE, |_, _| true.into_ret());
        e.register(CURRENT_RESPONSE, |_, _| 0x8900u32.into_ret());
        e.register(RESPONSE_ITEM, |_, a| (a[0] + 1).into_ret());
        e.register(RESPONSE_TEXT, |_, a| (a[0] + 1).into_ret());
        e.register(RESPONSE_WORD_A, |_, _| 0xa1u32.into_ret());
        e.register(RESPONSE_WORD_B, |_, _| 0xb2u32.into_ret());
        e.register(RESPONSE_SOUND, |_, a| (a[0] + 0x10).into_ret());
        e.register(POINTER_GET, |_, a| (a[0] + 0x10).into_ret());
        for addr in [
            PLAYER_CALL,
            SET_COMPLETION_CALLBACK,
            RUN_RESULT,
            SOUND_HANDLE_DESTRUCT,
            ITEM_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        assert!(!say(&mut e, actor, (2, 2), 0, controller));
        assert_eq!(
            calls(&e, CREATE_DIALOGUE_ITEM),
            vec![vec![0x7700, actor, 0x44, 0x7700, 0, 1]]
        );
        assert_eq!(calls(&e, PLAYER_CALL), vec![vec![actor, 1]]);
        let handle = calls(&e, slot(0x48c))[0][1];
        assert_eq!(
            calls(&e, slot(0x284)),
            vec![vec![
                actor, 0x8920, handle, 0xb2, 0xa1, 0x8902, 0, 0, 0, 0, 0, 0, 0, 1
            ]]
        );
        assert_eq!(
            calls(&e, SET_COMPLETION_CALLBACK),
            vec![vec![handle, COMPLETION_CALLBACK, 0]]
        );
        assert_eq!(calls(&e, RUN_RESULT), vec![vec![0x8800, 0]]);
        assert_eq!(calls(&e, SOUND_HANDLE_DESTRUCT), vec![vec![handle]]);
        assert_eq!(calls(&e, ITEM_DELETE), vec![vec![0x8800, 1]]);
        // No response: the item is only deleted.
        e.register(CURRENT_RESPONSE, |_, _| 0u32.into_ret());
        e.set_global(LAST_LINE_TIME, 0u32);
        e.call_log = Some(vec![]);
        assert!(!say(&mut e, actor, (2, 2), 0, controller));
        assert!(calls(&e, PLAYER_CALL).is_empty());
        assert_eq!(calls(&e, ITEM_DELETE).len(), 1);
    }

    #[test]
    fn start_dialogue_passes_the_controller_words() {
        let (mut e, actor, controller, _) = world();
        e.register(CONTROLLER_WORD, |_, _| 0x44u32.into_ret());
        e.register_double(CONTROLLER_ACTOR, move |_, _| actor.into_ret());
        assert!(e
            .call(0x0098_3970, &args![0x100u32, controller, 2u32, 2u32, 0u32])
            .bool());
        assert_eq!(calls(&e, CONTROLLER_WORD), vec![vec![controller]]);
        assert_eq!(calls(&e, GET_TOPIC), vec![vec![2, 2]]);
        let group = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x0098_4210, &args![group, 3u32]).u32(),
            group + 0x58 + 24
        );
        let player = e.mem.alloc(0x610);
        e.mem.set_u8(player + 0x608, 5);
        assert_eq!(e.call(0x0098_4270, &args![player]).u32(), 5);
    }
}

//! `fallout/ai/followerbarks.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Followers (companions) call out short lines ("barks") when something
//! happens to them: knocked out, crippled, low health, out of ammo and so on.
//! The barks that are waiting to be said or are cooling down are the entries
//! of a global list (`BSSimpleList<FollowerBarks::FollowerBarkEntry *>` at
//! `0x011e0098`, whose first node is part of the global itself). An entry is
//! 0x10 bytes: the speaker, the bark type (the Xbox PDB's
//! `FollowerBarks::FollowerBarkType`, 0 to 13) and two expiration times in the
//! tick count of `00457fe0`.
//!
//! List helpers of the exe used by address: `006815c0` (the node itself, so
//! the entry pointer is its first word), `00726070` (the next node),
//! `0063f7b0` (removes the node: the next node's contents move into it, so the
//! walker does not advance after a removal) and `005ae3d0` (adds an item given
//! by address).

#[allow(unused_imports)]
use crate::prelude::*;

/// The global list of entries (the head node is inline in the global).
const ENTRY_LIST: u32 = 0x011e_0098;
/// The global pointer (`0x011dea3c`) to the player-side object that the
/// conversation code is run against.
const PLAYER: u32 = 0x011d_ea3c;
/// `float`: the value written to the speaker's `+0x78` field (`-1.0`).
const MINUS_ONE: u32 = 0x0101_2054;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `0.25` (`double`).
const QUARTER: u32 = 0x0102_90b0;
/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// The tick count (a wrapper of an imported function).
const TICK_COUNT: u32 = 0x0045_7fe0;
/// `(this = node) -> node`.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `(this = node) -> next node or 0`.
const NODE_NEXT: u32 = 0x0072_6070;
/// `(this = node)`: removes the node from the list.
const NODE_REMOVE: u32 = 0x0063_f7b0;
/// `(this = list, pointer to the item)`: adds an item to the list.
const LIST_ADD: u32 = 0x005a_e3d0;
/// `operator new` (`size`).
const ALLOCATE: u32 = 0x0040_1000;
/// `operator delete` (`block`).
const FREE: u32 = 0x0040_1030;
/// `memset` (`block, value, size`).
const MEMSET: u32 = 0x00ec_61c0;
/// `(this = object) -> byte at +0x18d` (the engine map names it
/// `MiddleHighProcess::GetForceNextUpdate`; the receiver here is the entry's
/// speaker, so the name is unconfirmed for this use).
const SPEAKER_FLAG: u32 = 0x0056_6950;
/// `TESForm::GetFormByEditorID` (one cdecl argument, the editor id string).
const GET_FORM_BY_EDITOR_ID: u32 = 0x0048_3a00;
/// `__RTDynamicCast` (cdecl: object, vfptr delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptors of the topic cast (source, target).
const TOPIC_SOURCE_TYPE: u32 = 0x0118_3028;
const TOPIC_TARGET_TYPE: u32 = 0x0118_4720;
/// The type descriptors of the cast of the player-side object.
const PLAYER_SOURCE_TYPE: u32 = 0x0118_a23c;
const PLAYER_TARGET_TYPE: u32 = 0x0118_3108;
/// Per-type number used with the cooldowns (`fn_008d5830`, called by address).
const BARK_COOLDOWN: u32 = 0x008d_5830;

/// `TESTopic::GetMatchingInfo` (Xbox PDB name): `(this = topic, speaker, other)`.
const GET_MATCHING_INFO: u32 = 0x0061_a720;
/// `(this = speaker) -> pointer at +0x68` (the engine map names it
/// `MiddleHighProcess::GetSavedAcquireObject`; the body returns the word at
/// `+0x68`, here the speaker's current process).
const GET_PROCESS: u32 = 0x008d_8520;
/// `(this = speaker) -> int`: the process type (-1 without a process; the
/// engine map calls it `MobileObject::GetCurrentProcessType`).
const GET_PROCESS_TYPE: u32 = 0x0093_1850;
/// `TESObjectREFR::SetSayToTopic` (Xbox PDB name): `(this, topic)`.
const SET_SAY_TO_TOPIC: u32 = 0x0057_ad20;
/// `TESObjectREFR::SetSayToTopicInfo` (Xbox PDB name): `(this, info)`.
const SET_SAY_TO_TOPIC_INFO: u32 = 0x0057_ace0;
/// `TESTopic::CreateDialogueItem` (Xbox PDB name): `(this = topic, speaker, 0, 0, 0, flag)`.
const CREATE_DIALOGUE_ITEM: u32 = 0x0061_b320;
/// `(this = dialogue item) -> word at +0xC` (a form id getter).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// `BSSoundHandle::SetCompletionCallback` (Xbox PDB name): `(this = handle, callback, data)`.
const SET_COMPLETION_CALLBACK: u32 = 0x00ad_8e60;
/// The completion callback handed to it.
const COMPLETION_CALLBACK: u32 = 0x0057_90f0;
/// The sound handle's destructor.
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// `PlayerCharacter::IsSleepingorResting` (Xbox PDB name): `(this = player) -> bool`.
const IS_SLEEPING_OR_RESTING: u32 = 0x0094_df60;
/// `(this = process) -> word at +0x28`.
const PROCESS_WORD: u32 = 0x0045_cd60;
/// `Actor::GetEssential` (Xbox PDB name): `(this = actor) -> bool`.
const GET_ESSENTIAL: u32 = 0x0087_f3d0;
/// `(this = actor) -> word at +0x108`.
const ACTOR_STATE: u32 = 0x004f_8960;
/// `DialogueItem::FirstResponse`, `GetCurrentResponse` and `RunResult` (Xbox PDB names).
const FIRST_RESPONSE: u32 = 0x0083_c7b0;
const GET_CURRENT_RESPONSE: u32 = 0x0083_c820;
const RUN_RESULT: u32 = 0x0083_c850;
/// `(this = actor) -> this + 0x44`.
const ACTION_FLAG_ADDRESS: u32 = 0x005d_43c0;
/// `Script::SetActionFlag` (Xbox PDB name), cdecl: `(topic, address, mask)`.
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `(this = dialogue item, 1)`.
const FINISH_DIALOGUE_ITEM: u32 = 0x005c_90d0;

layout! {
    /// `FollowerBarks::FollowerBarkEntry` (Xbox PDB), 0x10 bytes.
    pub struct FollowerBarkEntry: 0x10 {
        /// `m_pSpeaker` (Xbox PDB): `Actor*`.
        0x00 m_pSpeaker: Ptr,
        /// `m_eBarkType` (Xbox PDB): `FollowerBarks::FollowerBarkType`.
        0x04 m_eBarkType: u32,
        /// `m_tNoRepeatExpiration` (Xbox PDB).
        0x08 m_tNoRepeatExpiration: u32,
        /// `m_tNoOtherBarksExpiration` (Xbox PDB).
        0x0C m_tNoOtherBarksExpiration: u32,
    }
}

/// The ActorValueOwner-like sub-object of an actor, at `+0xA4`: its slot `0xC`
/// gives the current value and its slot `4` the base value of an actor value.
const VALUES_OFFSET: u32 = 0xa4;
/// Actor value 0x10 (read by the health tests).
const HEALTH_VALUE: u32 = 0x10;

// Translated from 008d5750 (decompiled, FalloutNV.exe 1.4.0.525)
/// A number per follower bark type (unnamed in the Xbox PDB), from a jump
/// table: 80, 80, 40, 100, 50, 60, 10, 70, 70, 30, 20, 90, 15 and 5 for types
/// 0 to 13; 0 for anything else. `TriggerFollowerBark` compares them.
pub fn fn_008d5750(_e: &mut Engine, kind: u32) -> i32 {
    match kind {
        0 => 0x50,
        1 => 0x50,
        2 => 0x28,
        3 => 100,
        4 => 0x32,
        5 => 0x3c,
        6 => 10,
        7 => 0x46,
        8 => 0x46,
        9 => 0x1e,
        10 => 0x14,
        11 => 0x5a,
        12 => 0xf,
        13 => 5,
        _ => 0,
    }
}

// Translated from 008d5910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FollowerBarks::GetBarkTopicByType` (Xbox PDB): the topic form for a bark
/// type, found by editor id (the exe's strings) and cast to the topic class;
/// 0 for type 3 and for types above 13. The editor ids by type: 0 "KOed",
/// 1 "KnockedBack", 2 and 6 "Crippled", 4 "HealthHalf", 5 "HealthQuarter",
/// 7 "NoAmmo", 8 "WeaponBroke", 9 "WeaponIneffective", 10 "UsedDoctorBag",
/// 11 "Wake", 12 "FollowersStealthing", 13 "Regenerating".
pub fn follower_barks_get_bark_topic_by_type(e: &mut Engine, kind: u32) -> u32 {
    let editor_id = match kind {
        0 => 0x0108_7688,
        1 => 0x0108_767c,
        2 => 0x0106_4cdc,
        4 => 0x0108_764c,
        5 => 0x0108_7658,
        6 => 0x0106_4cdc,
        7 => 0x0108_7674,
        8 => 0x0108_7668,
        9 => 0x0108_7638,
        10 => 0x0108_7628,
        11 => 0x0106_37c0,
        12 => 0x0108_7614,
        13 => 0x0108_7604,
        _ => return 0,
    };
    let form = e.call(GET_FORM_BY_EDITOR_ID, &args![editor_id]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![form, 0u32, TOPIC_SOURCE_TYPE, TOPIC_TARGET_TYPE, 0u32],
    )
    .u32()
}

/// The first word of a list node's item pointer: the entry the node holds.
fn node_entry(e: &mut Engine, node: u32) -> u32 {
    let item = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(item)
}

// Translated from 008d5bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The per-frame update of the bark list (unnamed in the Xbox PDB). For every
/// entry with a speaker whose no-repeat time has passed: when the entry's
/// condition holds (`fn_008d5e30`) and the speaker's flag byte (`00566950`) is
/// set, the bark is renewed (`fn_008d6000`) and said (`fn_008d6050`);
/// otherwise the entry is dropped once its other-barks time has passed or the
/// speaker's flag byte is clear. A dropped entry is freed and its node
/// removed; the walk stays on the same node then, as the node now holds the
/// next entry.
pub fn fn_008d5bc0(e: &mut Engine) {
    let now = e.call(TICK_COUNT, &args![]).u32();
    let mut node = ENTRY_LIST;
    while node != 0 {
        let entry = Ptr::<FollowerBarkEntry>::new(node_entry(e, node));
        let mut remove = false;
        if !entry.is_null()
            && !e.get(entry, FollowerBarkEntry::m_pSpeaker).is_null()
            && e.get(entry, FollowerBarkEntry::m_tNoRepeatExpiration) <= now
        {
            let speaker = e.get(entry, FollowerBarkEntry::m_pSpeaker).addr();
            let mut said = false;
            if fn_008d5e30(e, entry) && e.call(SPEAKER_FLAG, &args![speaker]).bool() {
                let speaker = e.get(entry, FollowerBarkEntry::m_pSpeaker).addr();
                let kind = e.get(entry, FollowerBarkEntry::m_eBarkType);
                fn_008d6000(e, entry, speaker, kind);
                fn_008d6050(e, entry);
                said = true;
            }
            if !said {
                if e.get(entry, FollowerBarkEntry::m_tNoOtherBarksExpiration) <= now {
                    remove = true;
                } else {
                    let speaker = e.get(entry, FollowerBarkEntry::m_pSpeaker).addr();
                    if !e.call(SPEAKER_FLAG, &args![speaker]).bool() {
                        remove = true;
                    }
                }
            }
        }
        if remove {
            e.call(FREE, &args![entry]);
            e.call(NODE_REMOVE, &args![node]);
        } else {
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
}

// Translated from 008d5cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FollowerBarks::TriggerFollowerBark` (Xbox PDB): asks for `speaker` to say
/// a bark of type `kind`. Walks the list: a bark of the speaker whose
/// `fn_008d5750` number is at least the new one's and whose other-barks time
/// has not passed blocks saying it; the same type with its no-repeat time not
/// passed blocks it and also stops the entry from being replaced. When the
/// repeat is not blocked, the speaker's entries of this type are removed
/// (without freeing them, as the exe does), a new zeroed 0x10-byte entry is
/// added and set for the bark (`fn_008d6000`), and said at once
/// (`fn_008d6050`) unless blocked.
pub fn follower_barks_trigger_follower_bark(e: &mut Engine, speaker: u32, kind: u32) {
    let now = e.call(TICK_COUNT, &args![]).u32();
    let mut may_say = true;
    let mut may_replace = true;
    let mut node = ENTRY_LIST;
    while node != 0 {
        let entry = Ptr::<FollowerBarkEntry>::new(node_entry(e, node));
        node = e.call(NODE_NEXT, &args![node]).u32();
        if !entry.is_null() && speaker == e.get(entry, FollowerBarkEntry::m_pSpeaker).addr() {
            let existing = e.get(entry, FollowerBarkEntry::m_eBarkType);
            let existing_number = fn_008d5750(e, existing);
            let new_number = fn_008d5750(e, kind);
            if existing_number >= new_number
                && now < e.get(entry, FollowerBarkEntry::m_tNoOtherBarksExpiration)
            {
                may_say = false;
            }
            if existing == kind && now < e.get(entry, FollowerBarkEntry::m_tNoRepeatExpiration) {
                may_say = false;
                may_replace = false;
            }
        }
    }
    if may_replace {
        let mut node = ENTRY_LIST;
        while node != 0 {
            let entry = Ptr::<FollowerBarkEntry>::new(node_entry(e, node));
            let matches = !entry.is_null()
                && speaker == e.get(entry, FollowerBarkEntry::m_pSpeaker).addr()
                && e.get(entry, FollowerBarkEntry::m_eBarkType) == kind;
            if matches {
                e.call(NODE_REMOVE, &args![node]);
            } else {
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        }
        let block = e.call(ALLOCATE, &args![0x10u32]).u32();
        let new_entry = if block == 0 {
            0
        } else {
            e.call(MEMSET, &args![block, 0u32, 0x10u32]);
            block
        };
        e.with_stack(4, |e, item| {
            e.mem.set_u32(item.addr(), new_entry);
            e.call(LIST_ADD, &args![ENTRY_LIST, item]);
        });
        fn_008d6000(e, Ptr::new(new_entry), speaker, kind);
        if may_say {
            fn_008d6050(e, Ptr::new(new_entry));
        }
    }
}

/// The ratio of the speaker's current to base health, `1.0` when the base is
/// zero (the exe's `FCOMP; TEST AH,0x44; JNP` skips the division only for an
/// ordered equal to zero), rounded to `float` as it is stored.
fn health_ratio(e: &mut Engine, speaker: u32) -> f32 {
    let values = speaker.wrapping_add(VALUES_OFFSET);
    let current = e.vcall(values, 0xc, &args![HEALTH_VALUE]).f32();
    let base = e.vcall(values, 4, &args![HEALTH_VALUE]).f32();
    let zero: f64 = e.global(ZERO);
    if base as f64 == zero {
        1.0
    } else {
        (current as f64 / base as f64) as f32
    }
}

// Translated from 008d5e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the entry's bark condition holds (unnamed in the Xbox PDB), by
/// type. False without a speaker, and for every type except 4, 5 and 6: type 4
/// (a health warning) when the speaker's health ratio is above 0.25 and at
/// most 0.5; type 5 when it is at most 0.25; type 6 when any of the actor
/// values 0x19 to 0x1E is exactly zero. All comparisons are the exe's flag
/// tests: false when a value is NaN.
pub fn fn_008d5e30(e: &mut Engine, entry: Ptr<FollowerBarkEntry>) -> bool {
    let speaker = e.get(entry, FollowerBarkEntry::m_pSpeaker).addr();
    if speaker == 0 {
        return false;
    }
    match e.get(entry, FollowerBarkEntry::m_eBarkType) {
        4 => {
            let ratio = health_ratio(e, speaker) as f64;
            let quarter: f64 = e.global(QUARTER);
            let half: f64 = e.global(HALF);
            ratio > quarter && ratio <= half
        }
        5 => {
            let ratio = health_ratio(e, speaker) as f64;
            let quarter: f64 = e.global(QUARTER);
            ratio <= quarter
        }
        6 => {
            let values = speaker.wrapping_add(VALUES_OFFSET);
            for value in 0x19u32..0x1f {
                let current = e.vcall(values, 0xc, &args![value]).f32() as f64;
                let zero: f64 = e.global(ZERO);
                if current == zero {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

// Translated from 008d6000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets an entry for a bark (unnamed in the Xbox PDB): the type, the speaker,
/// the no-repeat expiration (now plus `fn_008d5830`'s number for the type) and
/// the other-barks expiration (now plus 0x708, 1800).
pub fn fn_008d6000(e: &mut Engine, entry: Ptr<FollowerBarkEntry>, speaker: u32, kind: u32) {
    e.set(entry, FollowerBarkEntry::m_eBarkType, kind);
    e.set(entry, FollowerBarkEntry::m_pSpeaker, Ptr::new(speaker));
    let now = e.call(TICK_COUNT, &args![]).u32();
    let cooldown = e.call(BARK_COOLDOWN, &args![kind]).u32();
    e.set(
        entry,
        FollowerBarkEntry::m_tNoRepeatExpiration,
        now.wrapping_add(cooldown),
    );
    let now = e.call(TICK_COUNT, &args![]).u32();
    e.set(
        entry,
        FollowerBarkEntry::m_tNoOtherBarksExpiration,
        now.wrapping_add(0x708),
    );
}

// Translated from 008d6050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d6050 - the compiler's exception frame (FS:[0], states 0 to 2 around the sound handles) is not translated, and the branch for an entry without a speaker (unreachable here: the speaker is tested non-null at the start and nothing in between changes it) is left out.
/// Says the entry's bark (unnamed in the Xbox PDB). Nothing unless the type has
/// a topic (`GetBarkTopicByType`), the topic has a matching response for the
/// speaker and the player-side object (`GetMatchingInfo`) and the speaker has
/// a process. Then, by case:
///
/// - the speaker's slot `0x22C` (with 1) false: the speaker's `+0x78` is set to
///   `-1.0` and its process is told (slots `0x370`, `0xAC`). With no process
///   type (`00931850` zero) the process is also driven through slots
///   `0x674`, `0x648`, `0x628` and `0x224`; otherwise the speaker is told the
///   topic and a dialogue item is created, the speaker's slot `0x408` makes
///   the sound, a completion callback is attached to it, and that is all. After
///   the process-type-zero path, the speaker either (when the player is
///   sleeping or resting, the process word is set, or the speaker is essential
///   with state 6) runs the dialogue item's first response and both results
///   and sets the topic's action flag, or has its process (slot `0x2A4`) say
///   the topic;
/// - slot `0x22C` true or no process: only the topic's action flag is set and
///   the process told (slot `0x88` with 0).
pub fn fn_008d6050(e: &mut Engine, entry: Ptr<FollowerBarkEntry>) {
    let kind = e.get(entry, FollowerBarkEntry::m_eBarkType);
    let topic = follower_barks_get_bark_topic_by_type(e, kind);
    if topic == 0 {
        return;
    }
    let speaker = e.get(entry, FollowerBarkEntry::m_pSpeaker).addr();
    let player: u32 = e.global(PLAYER);
    let info = e
        .call(GET_MATCHING_INFO, &args![topic, speaker, player])
        .u32();
    if info == 0 || speaker == 0 {
        return;
    }
    if e.call(GET_PROCESS, &args![speaker]).u32() == 0 {
        return;
    }
    // The three flags are passed on to the sound (the first two) and the process
    // (the last); the fourth is passed to the process's slot 0x2A4.
    let flag_a = 1u32;
    let flag_b = 1u32;
    let flag_d = 0u32;
    // The cast result is only used by the branch that is left out.
    e.call(
        DYNAMIC_CAST,
        &args![player, 0u32, PLAYER_SOURCE_TYPE, PLAYER_TARGET_TYPE, 0u32],
    );
    let has_process = e.call(GET_PROCESS, &args![speaker]).u32() != 0;
    let in_conversation = has_process && e.vcall(speaker, 0x22c, &args![1u32]).bool();
    if !has_process || in_conversation {
        if e.call(GET_PROCESS, &args![speaker]).u32() != 0 {
            let address = e.call(ACTION_FLAG_ADDRESS, &args![speaker]).u32();
            e.call(SET_ACTION_FLAG, &args![topic, address, 0x40000u32]);
            let process = e.call(GET_PROCESS, &args![speaker]).u32();
            e.vcall(process, 0x88, &args![0u32]);
        }
        return;
    }
    let minus_one: f32 = e.global(MINUS_ONE);
    e.mem.set_f32(speaker + 0x78, minus_one);
    let process = e.call(GET_PROCESS, &args![speaker]).u32();
    e.vcall(process, 0x370, &args![0u32]);
    let process = e.call(GET_PROCESS, &args![speaker]).u32();
    e.vcall(process, 0xac, &args![]);
    if e.call(GET_PROCESS_TYPE, &args![speaker]).u32() == 0 {
        let process = e.call(GET_PROCESS, &args![speaker]).u32();
        let value = e.vcall(process, 0x674, &args![2u32]).u32();
        let process = e.call(GET_PROCESS, &args![speaker]).u32();
        e.vcall(process, 0x648, &args![0u32]);
        let process = e.call(GET_PROCESS, &args![speaker]).u32();
        e.vcall(process, 0x628, &args![value]);
        let process = e.call(GET_PROCESS, &args![speaker]).u32();
        e.vcall(process, 0x224, &args![]);
    } else {
        // (The exe also tests a flag byte that is always 1 here.)
        e.call(SET_SAY_TO_TOPIC, &args![speaker, topic]);
        let item = e
            .call(
                CREATE_DIALOGUE_ITEM,
                &args![topic, speaker, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        if item != 0 {
            let form_id = e.call(GET_FORM_ID, &args![item]).u32();
            e.call(SET_SAY_TO_TOPIC_INFO, &args![speaker, form_id]);
        }
        e.with_stack(12, |e, handle| {
            e.vcall(
                speaker,
                0x408,
                &args![handle, topic, speaker, flag_a, flag_b, 0u32, 0u32],
            );
            let data = e.call(GET_FORM_ID, &args![speaker]).u32();
            e.call(
                SET_COMPLETION_CALLBACK,
                &args![handle, COMPLETION_CALLBACK, data],
            );
            e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
        });
        return;
    }
    let said_through_dialogue = e.call(IS_SLEEPING_OR_RESTING, &args![player]).bool()
        || {
            let process = e.call(GET_PROCESS, &args![speaker]).u32();
            e.call(PROCESS_WORD, &args![process]).u32() != 0
        }
        || (e.call(GET_ESSENTIAL, &args![speaker]).bool()
            && e.call(ACTOR_STATE, &args![speaker]).u32() == 6);
    if said_through_dialogue {
        let item = e
            .call(
                CREATE_DIALOGUE_ITEM,
                &args![topic, speaker, 0u32, 0u32, 0u32, 1u32],
            )
            .u32();
        if item != 0 {
            e.call(FIRST_RESPONSE, &args![item]);
            e.call(GET_CURRENT_RESPONSE, &args![item]);
            e.call(RUN_RESULT, &args![item, 0u32]);
            e.call(RUN_RESULT, &args![item, 1u32]);
            let address = e.call(ACTION_FLAG_ADDRESS, &args![speaker]).u32();
            e.call(SET_ACTION_FLAG, &args![topic, address, 0x40000u32]);
            e.call(FINISH_DIALOGUE_ITEM, &args![item, 1u32]);
        }
    } else {
        let process = e.call(GET_PROCESS, &args![speaker]).u32();
        e.vcall(
            process,
            0x2a4,
            &args![speaker, topic, flag_d, 0u32, 1u32, 1u32],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d5750, fn_008d5750(u32) -> i32),
        entry!(
            0x008d5910,
            follower_barks_get_bark_topic_by_type(u32) -> u32
        ),
        entry!(0x008d5bc0, fn_008d5bc0()),
        entry!(0x008d5cb0, follower_barks_trigger_follower_bark(u32, u32)),
        entry!(0x008d5e30, fn_008d5e30(Ptr<FollowerBarkEntry>) -> bool),
        entry!(0x008d6000, fn_008d6000(Ptr<FollowerBarkEntry>, u32, u32)),
        entry!(0x008d6050, fn_008d6050(Ptr<FollowerBarkEntry>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn ret(v: u32) -> Ret {
        Ret {
            eax: v,
            ..Ret::default()
        }
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An object whose virtual slots are doubles that append `(slot, args)`
    /// to `log` and return the value given for the slot (default 0).
    fn object(e: &mut Engine, log: &Log, slots: &[(u32, Ret)]) -> u32 {
        static NEXT_TARGET: AtomicU32 = AtomicU32::new(0x7100_0000);
        let obj = e.mem.alloc(0x400);
        let table = e.mem.alloc(0x800);
        e.mem.set_u32(obj, table);
        for (slot, result) in slots {
            let target = NEXT_TARGET.fetch_add(1, Ordering::Relaxed);
            e.mem.set_u32(table + slot, target);
            let log = log.clone();
            let slot = *slot;
            let result = *result;
            e.register_double(target, move |_, a| {
                log.borrow_mut().push((slot, a[1..].to_vec()));
                result
            });
        }
        obj
    }

    #[test]
    fn numbers_per_bark_type() {
        let mut e = Engine::new();
        let expected = [80, 80, 40, 100, 50, 60, 10, 70, 70, 30, 20, 90, 15, 5, 0, 0];
        for (kind, want) in expected.iter().enumerate() {
            assert_eq!(
                e.call(0x008d5750, &args![kind as u32]).i32(),
                *want,
                "type {kind}"
            );
        }
        assert_eq!(e.call(0x008d5750, &args![u32::MAX]).i32(), 0);
    }

    #[test]
    fn topic_by_type_looks_up_the_editor_id_and_casts() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let s = seen.clone();
        e.register_double(GET_FORM_BY_EDITOR_ID, move |_, a| {
            s.borrow_mut().push(a[0]);
            ret(0x900 + a[0] % 0x100)
        });
        e.register(DYNAMIC_CAST, |_, a| {
            assert_eq!(a[1], 0);
            assert_eq!(a[2], 0x0118_3028);
            assert_eq!(a[3], 0x0118_4720);
            assert_eq!(a[4], 0);
            ret(a[0] + 1)
        });
        let ids = [
            (0u32, Some(0x0108_7688u32)),
            (1, Some(0x0108_767c)),
            (2, Some(0x0106_4cdc)),
            (3, None),
            (4, Some(0x0108_764c)),
            (5, Some(0x0108_7658)),
            (6, Some(0x0106_4cdc)),
            (7, Some(0x0108_7674)),
            (8, Some(0x0108_7668)),
            (9, Some(0x0108_7638)),
            (10, Some(0x0108_7628)),
            (11, Some(0x0106_37c0)),
            (12, Some(0x0108_7614)),
            (13, Some(0x0108_7604)),
            (14, None),
        ];
        for (kind, id) in ids {
            seen.borrow_mut().clear();
            let topic = e.call(0x008d5910, &args![kind]).u32();
            match id {
                Some(id) => {
                    assert_eq!(*seen.borrow(), vec![id], "type {kind}");
                    assert_eq!(topic, 0x900 + id % 0x100 + 1);
                }
                None => {
                    assert!(seen.borrow().is_empty(), "type {kind} makes no lookup");
                    assert_eq!(topic, 0);
                }
            }
        }
    }

    /// An actor-like object with the value owner at `+0xA4`: slot `0xC` gives
    /// `current(index)`, slot `4` gives `base`.
    fn speaker_with_values(e: &mut Engine, current: [f32; 32], base: f32) -> u32 {
        e.map(ZERO, 8);
        e.map(QUARTER, 8);
        e.map(HALF, 8);
        e.set_global(ZERO, 0.0f64);
        e.set_global(QUARTER, 0.25f64);
        e.set_global(HALF, 0.5f64);
        let speaker = e.mem.alloc(0x200);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(speaker + 0xa4, table);
        e.mem.set_u32(table + 0xc, 0x7200_000c);
        e.mem.set_u32(table + 4, 0x7200_0004);
        let slot_values = Rc::new(current);
        e.register_double(0x7200_000c, move |_, a| Ret {
            st0: slot_values[a[1] as usize] as f64,
            ..Ret::default()
        });
        e.register_double(0x7200_0004, move |_, _| Ret {
            st0: base as f64,
            ..Ret::default()
        });
        speaker
    }

    fn entry_of(e: &mut Engine, speaker: u32, kind: u32) -> Ptr<FollowerBarkEntry> {
        let entry = e.new_object::<FollowerBarkEntry>();
        e.set(entry, FollowerBarkEntry::m_pSpeaker, Ptr::new(speaker));
        e.set(entry, FollowerBarkEntry::m_eBarkType, kind);
        entry
    }

    fn health(current: f32) -> [f32; 32] {
        let mut v = [1.0; 32];
        v[0x10] = current;
        v
    }

    #[test]
    fn condition_false_without_a_speaker_and_for_other_types() {
        let mut e = Engine::new();
        let speaker = speaker_with_values(&mut e, health(1.0), 4.0);
        let none = entry_of(&mut e, 0, 5);
        assert!(!e.call(0x008d5e30, &args![none]).bool());
        for kind in [0u32, 1, 2, 3, 7, 8, 9, 10, 11, 12, 13, 14, 100] {
            let entry = entry_of(&mut e, speaker, kind);
            assert!(!e.call(0x008d5e30, &args![entry]).bool(), "type {kind}");
        }
    }

    #[test]
    fn quarter_health_warning_is_at_most_a_quarter() {
        let mut e = Engine::new();
        let speaker = speaker_with_values(&mut e, health(25.0), 100.0);
        let entry = entry_of(&mut e, speaker, 5);
        assert!(e.call(0x008d5e30, &args![entry]).bool(), "exactly 0.25");
        let speaker = speaker_with_values(&mut e, health(26.0), 100.0);
        let entry = entry_of(&mut e, speaker, 5);
        assert!(!e.call(0x008d5e30, &args![entry]).bool());
        let speaker = speaker_with_values(&mut e, health(0.0), 100.0);
        let entry = entry_of(&mut e, speaker, 5);
        assert!(e.call(0x008d5e30, &args![entry]).bool());
    }

    #[test]
    fn half_health_warning_is_above_a_quarter_and_at_most_half() {
        let mut e = Engine::new();
        for (current, want) in [(25.0f32, false), (26.0, true), (50.0, true), (51.0, false)] {
            let speaker = speaker_with_values(&mut e, health(current), 100.0);
            let entry = entry_of(&mut e, speaker, 4);
            assert_eq!(e.call(0x008d5e30, &args![entry]).bool(), want, "{current}");
        }
    }

    #[test]
    fn zero_base_health_counts_as_a_full_ratio_and_nan_never_warns() {
        let mut e = Engine::new();
        let speaker = speaker_with_values(&mut e, health(0.0), 0.0);
        let entry = entry_of(&mut e, speaker, 5);
        assert!(!e.call(0x008d5e30, &args![entry]).bool(), "ratio 1.0");
        let speaker = speaker_with_values(&mut e, health(10.0), f32::NAN);
        for kind in [4u32, 5] {
            let entry = entry_of(&mut e, speaker, kind);
            assert!(
                !e.call(0x008d5e30, &args![entry]).bool(),
                "NaN, type {kind}"
            );
        }
    }

    #[test]
    fn crippled_reminder_needs_a_zero_limb_value() {
        let mut e = Engine::new();
        let mut values = [1.0f32; 32];
        let speaker = speaker_with_values(&mut e, values, 1.0);
        let entry = entry_of(&mut e, speaker, 6);
        assert!(!e.call(0x008d5e30, &args![entry]).bool());
        values[0x1f] = 0.0;
        let speaker = speaker_with_values(&mut e, values, 1.0);
        let entry = entry_of(&mut e, speaker, 6);
        assert!(
            !e.call(0x008d5e30, &args![entry]).bool(),
            "0x1f is outside 0x19..=0x1e"
        );
        values[0x1d] = 0.0;
        let speaker = speaker_with_values(&mut e, values, 1.0);
        let entry = entry_of(&mut e, speaker, 6);
        assert!(e.call(0x008d5e30, &args![entry]).bool());
        values[0x1d] = -0.0;
        let speaker = speaker_with_values(&mut e, values, 1.0);
        let entry = entry_of(&mut e, speaker, 6);
        assert!(
            e.call(0x008d5e30, &args![entry]).bool(),
            "negative zero is equal to zero"
        );
        values[0x1d] = f32::NAN;
        let speaker = speaker_with_values(&mut e, values, 1.0);
        let entry = entry_of(&mut e, speaker, 6);
        assert!(!e.call(0x008d5e30, &args![entry]).bool());
    }

    fn tick_double(e: &mut Engine, now: u32) -> Rc<Cell<u32>> {
        let cell = Rc::new(Cell::new(now));
        let c = cell.clone();
        e.register_double(TICK_COUNT, move |_, _| ret(c.get()));
        cell
    }

    #[test]
    fn entry_setup_adds_the_cooldown_and_1800() {
        let mut e = Engine::new();
        tick_double(&mut e, 10_000);
        e.register(BARK_COOLDOWN, |_, a| ret(a[0] * 1000));
        let entry = e.new_object::<FollowerBarkEntry>();
        e.call(0x008d6000, &args![entry, 0x1234u32, 3u32]);
        assert_eq!(e.get(entry, FollowerBarkEntry::m_pSpeaker).addr(), 0x1234);
        assert_eq!(e.get(entry, FollowerBarkEntry::m_eBarkType), 3);
        assert_eq!(
            e.get(entry, FollowerBarkEntry::m_tNoRepeatExpiration),
            13_000
        );
        assert_eq!(
            e.get(entry, FollowerBarkEntry::m_tNoOtherBarksExpiration),
            11_800
        );
    }

    /// A list of nodes `[item, next]` starting with the global head node.
    fn list_rig(e: &mut Engine, entries: &[u32]) {
        e.map(ENTRY_LIST & !0xfff, 0x1000);
        let mut nodes = vec![ENTRY_LIST];
        for _ in 1..entries.len() {
            nodes.push(e.mem.alloc(8));
        }
        for (i, entry) in entries.iter().enumerate() {
            e.mem.set_u32(nodes[i], *entry);
            let next = nodes.get(i + 1).copied().unwrap_or(0);
            e.mem.set_u32(nodes[i] + 4, next);
        }
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        // The removal model: the next node's contents move into the node.
        e.register(NODE_REMOVE, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
    }

    fn list_items(e: &Engine) -> Vec<u32> {
        let mut items = vec![];
        let mut node = ENTRY_LIST;
        while node != 0 {
            let item = e.mem.u32(node);
            if item != 0 {
                items.push(item);
            }
            node = e.mem.u32(node + 4);
        }
        items
    }

    #[test]
    fn trigger_adds_a_fresh_entry_and_says_it() {
        let mut e = Engine::new();
        tick_double(&mut e, 100);
        list_rig(&mut e, &[0]);
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, 0xdead);
        e.register_double(ALLOCATE, move |_, a| {
            assert_eq!(a[0], 0x10);
            ret(block)
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(LIST_ADD, |e, a| {
            // The model list: store the entry into the (empty) head node.
            let entry = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], entry);
            Ret::default()
        });
        e.register(BARK_COOLDOWN, |_, _| ret(500));
        e.call(0x008d5cb0, &args![0x77u32, 3u32]);
        assert_eq!(list_items(&e), vec![block]);
        assert_eq!(e.mem.u32(block), 0x77, "the speaker");
        assert_eq!(e.mem.u32(block + 4), 3, "the type");
        assert_eq!(e.mem.u32(block + 8), 600, "no-repeat expiration");
        assert_eq!(e.mem.u32(block + 12), 1900, "other-barks expiration");
    }

    /// Pre-filled list with the entries `(speaker, kind, no_repeat, no_other)`.
    fn entries_rig(e: &mut Engine, now: u32, specs: &[(u32, u32, u32, u32)]) -> Vec<u32> {
        tick_double(e, now);
        let mut entries = vec![];
        for (speaker, kind, no_repeat, no_other) in specs {
            let entry = entry_of(e, *speaker, *kind);
            e.set(entry, FollowerBarkEntry::m_tNoRepeatExpiration, *no_repeat);
            e.set(
                entry,
                FollowerBarkEntry::m_tNoOtherBarksExpiration,
                *no_other,
            );
            entries.push(entry.addr());
        }
        list_rig(e, &entries);
        entries
    }

    fn trigger_doubles(e: &mut Engine) -> Rc<RefCell<Vec<u32>>> {
        let adds = Rc::new(RefCell::new(Vec::new()));
        let a = adds.clone();
        let block = e.mem.alloc(0x10);
        e.register_double(ALLOCATE, move |_, _| ret(block));
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register_double(LIST_ADD, move |e, args| {
            a.borrow_mut().push(e.mem.u32(args[1]));
            Ret::default()
        });
        adds
    }

    #[test]
    fn trigger_with_the_same_type_not_yet_expired_changes_nothing() {
        let mut e = Engine::new();
        let entries = entries_rig(&mut e, 100, &[(0x77, 3, 200, 50)]);
        let adds = trigger_doubles(&mut e);
        e.call(0x008d5cb0, &args![0x77u32, 3u32]);
        assert!(adds.borrow().is_empty());
        assert_eq!(list_items(&e), entries, "the entry stays");
    }

    #[test]
    fn trigger_replaces_an_expired_entry_of_the_same_type_without_freeing_it() {
        let mut e = Engine::new();
        entries_rig(&mut e, 100, &[(0x77, 3, 50, 50), (0x88, 3, 500, 500)]);
        let adds = trigger_doubles(&mut e);
        e.register(BARK_COOLDOWN, |_, _| ret(0));
        e.register(GET_FORM_BY_EDITOR_ID, |_, _| ret(0));
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x008d5cb0, &args![0x77u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert!(
            log.iter().all(|(a, _)| *a != FREE),
            "the old entry is not freed"
        );
        assert_eq!(adds.borrow().len(), 1);
        assert_eq!(
            list_items(&e).len(),
            1,
            "the speaker's old entry left; the other stays"
        );
    }

    #[test]
    fn trigger_with_a_stronger_active_bark_is_added_but_not_said() {
        let mut e = Engine::new();
        // Type 3 (100) is still within its other-barks time; the new type is 6 (10).
        entries_rig(&mut e, 100, &[(0x77, 3, 50, 500)]);
        trigger_doubles(&mut e);
        e.register(BARK_COOLDOWN, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x008d5cb0, &args![0x77u32, 6u32]);
        let log = e.call_log.take().unwrap();
        // No lookup of a topic: the bark is not said.
        assert!(log.iter().all(|(a, _)| *a != GET_FORM_BY_EDITOR_ID));
        assert!(log.iter().any(|(a, _)| *a == LIST_ADD));
    }

    #[test]
    fn trigger_for_another_speaker_is_not_blocked_by_the_list() {
        let mut e = Engine::new();
        entries_rig(&mut e, 100, &[(0x11, 3, 500, 500)]);
        trigger_doubles(&mut e);
        e.register(MEMSET, |_, _| Ret::default());
        e.register(BARK_COOLDOWN, |_, _| ret(0));
        e.register(GET_FORM_BY_EDITOR_ID, |_, _| ret(0));
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x008d5cb0, &args![0x77u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(a, _)| *a == GET_FORM_BY_EDITOR_ID), "said");
    }

    #[test]
    fn update_drops_expired_entries_and_frees_them() {
        let mut e = Engine::new();
        // Past both times: dropped.
        let entries = entries_rig(&mut e, 100, &[(0x77, 3, 50, 60), (0x88, 3, 500, 500)]);
        e.register(SPEAKER_FLAG, |_, _| ret(1));
        e.register(FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d5bc0, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|(a, _)| *a == FREE).count(), 1);
        assert!(log.contains(&(FREE, vec![entries[0]])));
        assert_eq!(list_items(&e), vec![entries[1]]);
    }

    #[test]
    fn update_drops_an_entry_whose_speaker_flag_is_clear() {
        let mut e = Engine::new();
        let entries = entries_rig(&mut e, 100, &[(0x77, 3, 50, 500)]);
        e.register(SPEAKER_FLAG, |_, _| ret(0));
        e.register(FREE, |_, _| Ret::default());
        e.call(0x008d5bc0, &args![]);
        assert!(list_items(&e).is_empty());
        let _ = entries;
    }

    #[test]
    fn update_keeps_an_entry_whose_speaker_flag_is_set_and_other_time_runs() {
        let mut e = Engine::new();
        let entries = entries_rig(&mut e, 100, &[(0x77, 3, 50, 500)]);
        e.register(SPEAKER_FLAG, |_, _| ret(1));
        e.call(0x008d5bc0, &args![]);
        assert_eq!(list_items(&e), entries);
    }

    #[test]
    fn update_leaves_entries_whose_no_repeat_time_has_not_passed() {
        let mut e = Engine::new();
        let entries = entries_rig(&mut e, 100, &[(0x77, 3, 200, 0)]);
        e.call_log = Some(vec![]);
        e.call(0x008d5bc0, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != SPEAKER_FLAG && *a != FREE));
        assert_eq!(list_items(&e), entries);
    }

    #[test]
    fn update_says_a_bark_whose_condition_holds() {
        let mut e = Engine::new();
        // Type 5 with health at 10%: the condition holds.
        let speaker = speaker_with_values(&mut e, health(10.0), 100.0);
        let entries = entries_rig(&mut e, 100, &[(speaker, 5, 50, 500)]);
        e.register(SPEAKER_FLAG, |_, _| ret(1));
        e.register(BARK_COOLDOWN, |_, _| ret(1000));
        e.register(GET_FORM_BY_EDITOR_ID, |_, _| ret(0));
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x008d5bc0, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(
            log.iter().any(|(a, _)| *a == GET_FORM_BY_EDITOR_ID),
            "the topic is looked up"
        );
        let entry = Ptr::<FollowerBarkEntry>::new(entries[0]);
        assert_eq!(e.get(entry, FollowerBarkEntry::m_tNoRepeatExpiration), 1100);
        assert_eq!(
            e.get(entry, FollowerBarkEntry::m_tNoOtherBarksExpiration),
            1900
        );
        assert_eq!(list_items(&e), entries, "the entry stays");
    }

    /// Everything the bark-saying function touches: a speaker object whose
    /// slots log, a process object whose slots log, the player and the
    /// callee doubles. Returns (entry, speaker, process, log).
    struct SayRig {
        e: Engine,
        entry: Ptr<FollowerBarkEntry>,
        speaker: u32,
        log: Log,
        calls: Log,
    }

    fn say_rig(in_conversation: bool, process_type: u32) -> SayRig {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let calls: Log = Rc::new(RefCell::new(Vec::new()));
        let speaker = object(
            &mut e,
            &log,
            &[
                (0x22c, ret(in_conversation as u32)),
                (0x408, Ret::default()),
            ],
        );
        let process = object(
            &mut e,
            &log,
            &[
                (0x370, Ret::default()),
                (0xac, Ret::default()),
                (0x674, ret(0x55)),
                (0x648, Ret::default()),
                (0x628, Ret::default()),
                (0x224, Ret::default()),
                (0x2a4, Ret::default()),
                (0x88, Ret::default()),
            ],
        );
        e.map(PLAYER, 4);
        e.set_global(PLAYER, 0x4000u32);
        e.map(MINUS_ONE, 4);
        e.set_global(MINUS_ONE, -1.0f32);
        e.register(GET_FORM_BY_EDITOR_ID, |_, _| ret(0x300));
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 0x300 { 0x301 } else { 0x4001 })
        });
        let sink = calls.clone();
        let record = |e: &mut Engine, sink: &Log, addr: u32, result: u32| {
            let sink = sink.clone();
            e.register_double(addr, move |_, a| {
                sink.borrow_mut().push((addr, a.to_vec()));
                ret(result)
            });
        };
        record(&mut e, &sink, GET_MATCHING_INFO, 0x777);
        record(&mut e, &sink, GET_PROCESS, process);
        record(&mut e, &sink, GET_PROCESS_TYPE, process_type);
        record(&mut e, &sink, SET_SAY_TO_TOPIC, 0);
        record(&mut e, &sink, SET_SAY_TO_TOPIC_INFO, 0);
        record(&mut e, &sink, CREATE_DIALOGUE_ITEM, 0x888);
        record(&mut e, &sink, GET_FORM_ID, 0x999);
        record(&mut e, &sink, SET_COMPLETION_CALLBACK, 0);
        record(&mut e, &sink, SOUND_HANDLE_DESTRUCT, 0);
        record(&mut e, &sink, IS_SLEEPING_OR_RESTING, 0);
        record(&mut e, &sink, PROCESS_WORD, 0);
        record(&mut e, &sink, GET_ESSENTIAL, 0);
        record(&mut e, &sink, ACTOR_STATE, 0);
        record(&mut e, &sink, FIRST_RESPONSE, 0);
        record(&mut e, &sink, GET_CURRENT_RESPONSE, 0);
        record(&mut e, &sink, RUN_RESULT, 0);
        record(&mut e, &sink, ACTION_FLAG_ADDRESS, 0xf1a6);
        record(&mut e, &sink, SET_ACTION_FLAG, 0);
        record(&mut e, &sink, FINISH_DIALOGUE_ITEM, 0);
        let entry = entry_of(&mut e, speaker, 1);
        SayRig {
            e,
            entry,
            speaker,
            log,
            calls,
        }
    }

    impl SayRig {
        fn run(&mut self) {
            let entry = self.entry;
            self.e.call(0x008d6050, &args![entry]);
        }
        fn called(&self, addr: u32) -> Vec<Vec<u32>> {
            self.calls
                .borrow()
                .iter()
                .filter(|(a, _)| *a == addr)
                .map(|(_, v)| v.clone())
                .collect()
        }
        fn slots(&self) -> Vec<u32> {
            self.log.borrow().iter().map(|(s, _)| *s).collect()
        }
    }

    #[test]
    fn say_does_nothing_without_a_topic() {
        let mut r = say_rig(false, 0);
        let entry = r.entry;
        r.e.set(entry, FollowerBarkEntry::m_eBarkType, 3);
        r.run();
        assert!(r.calls.borrow().is_empty());
    }

    #[test]
    fn say_does_nothing_without_matching_info() {
        let mut r = say_rig(false, 0);
        let sink = r.calls.clone();
        r.e.register_double(GET_MATCHING_INFO, move |_, a| {
            sink.borrow_mut().push((GET_MATCHING_INFO, a.to_vec()));
            Ret::default()
        });
        r.run();
        assert_eq!(
            r.called(GET_MATCHING_INFO),
            vec![vec![0x301, r.speaker, 0x4000]]
        );
        assert_eq!(r.calls.borrow().len(), 1);
    }

    #[test]
    fn say_does_nothing_without_a_process() {
        let mut r = say_rig(false, 0);
        r.e.register(GET_PROCESS, |_, _| Ret::default());
        r.run();
        assert!(r.slots().is_empty());
    }

    #[test]
    fn say_in_conversation_only_sets_the_action_flag_and_tells_the_process() {
        let mut r = say_rig(true, 0);
        r.run();
        assert_eq!(
            r.called(SET_ACTION_FLAG),
            vec![vec![0x301, 0xf1a6, 0x40000]]
        );
        assert_eq!(r.slots(), vec![0x22c, 0x88]);
        assert_eq!(r.log.borrow()[1], (0x88, vec![0]));
        assert!(r.called(CREATE_DIALOGUE_ITEM).is_empty());
    }

    #[test]
    fn say_with_process_type_zero_drives_the_process_then_says_through_it() {
        let mut r = say_rig(false, 0);
        r.run();
        assert_eq!(r.e.mem.f32(r.speaker + 0x78), -1.0);
        assert_eq!(
            r.slots(),
            vec![0x22c, 0x370, 0xac, 0x674, 0x648, 0x628, 0x224, 0x2a4]
        );
        let log = r.log.borrow();
        assert_eq!(log[0].1, vec![1]);
        assert_eq!(log[3].1, vec![2]);
        assert_eq!(log[5].1, vec![0x55], "the value from slot 0x674");
        // The final say: speaker, topic, 0, 0, 1, 1.
        assert_eq!(log[7].1, vec![r.speaker, 0x301, 0, 0, 1, 1]);
    }

    #[test]
    fn say_with_another_process_type_makes_the_sound_and_stops() {
        let mut r = say_rig(false, 5);
        r.run();
        assert_eq!(r.called(SET_SAY_TO_TOPIC), vec![vec![r.speaker, 0x301]]);
        assert_eq!(
            r.called(CREATE_DIALOGUE_ITEM),
            vec![vec![0x301, r.speaker, 0, 0, 0, 0]]
        );
        assert_eq!(
            r.called(SET_SAY_TO_TOPIC_INFO),
            vec![vec![r.speaker, 0x999]]
        );
        let sound = r
            .log
            .borrow()
            .iter()
            .find(|(s, _)| *s == 0x408)
            .cloned()
            .unwrap();
        assert_eq!(&sound.1[1..], &[0x301, r.speaker, 1, 1, 0, 0]);
        let handle = sound.1[0];
        assert_eq!(
            r.called(SET_COMPLETION_CALLBACK),
            vec![vec![handle, 0x0057_90f0, 0x999]]
        );
        assert_eq!(r.called(SOUND_HANDLE_DESTRUCT), vec![vec![handle]]);
        assert!(
            r.called(IS_SLEEPING_OR_RESTING).is_empty(),
            "returns before the sleep test"
        );
    }

    #[test]
    fn say_through_the_dialogue_item_when_the_player_is_sleeping() {
        let mut r = say_rig(false, 0);
        r.e.register(IS_SLEEPING_OR_RESTING, |_, _| ret(1));
        r.run();
        assert_eq!(
            r.called(CREATE_DIALOGUE_ITEM),
            vec![vec![0x301, r.speaker, 0, 0, 0, 1]]
        );
        assert_eq!(r.called(RUN_RESULT), vec![vec![0x888, 0], vec![0x888, 1]]);
        assert_eq!(
            r.called(SET_ACTION_FLAG),
            vec![vec![0x301, 0xf1a6, 0x40000]]
        );
        assert_eq!(r.called(FINISH_DIALOGUE_ITEM), vec![vec![0x888, 1]]);
        assert!(!r.slots().contains(&0x2a4));
    }

    #[test]
    fn say_through_the_dialogue_item_for_an_essential_actor_in_state_six() {
        let mut r = say_rig(false, 0);
        let sink = r.calls.clone();
        r.e.register_double(GET_ESSENTIAL, move |_, a| {
            sink.borrow_mut().push((GET_ESSENTIAL, a.to_vec()));
            ret(1)
        });
        r.e.register(ACTOR_STATE, |_, _| ret(6));
        r.run();
        assert_eq!(r.called(CREATE_DIALOGUE_ITEM).len(), 1);
        assert!(!r.slots().contains(&0x2a4));
    }

    #[test]
    fn essential_actor_in_another_state_says_through_the_process() {
        let mut r = say_rig(false, 0);
        r.e.register(GET_ESSENTIAL, |_, _| ret(1));
        r.e.register(ACTOR_STATE, |_, _| ret(5));
        r.run();
        assert!(r.slots().contains(&0x2a4));
        assert!(r.called(CREATE_DIALOGUE_ITEM).is_empty());
    }

    #[test]
    fn a_process_word_says_through_the_dialogue_item_and_a_null_item_does_nothing_more() {
        let mut r = say_rig(false, 0);
        r.e.register(PROCESS_WORD, |_, _| ret(1));
        r.e.register(CREATE_DIALOGUE_ITEM, |_, _| ret(0));
        r.run();
        assert!(r.called(RUN_RESULT).is_empty());
        assert!(!r.slots().contains(&0x2a4));
    }
}

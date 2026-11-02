//! `fallout/dialogue/menutopic.cpp` (Xbox PDB source unit), subsystem `fallout/dialogue`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `MenuTopicManager` keeps a list of topic lists (`listTopicLists`, a
//! `BSSimpleList<BSSimpleList<MenuTopic *> *>` at +4) whose first entry is the list shown,
//! and a node of that list as the current topic. The list helpers it calls: `006815c0`
//! (the address of a node's item), `00726070` (the next node), `008256d0` (whether a list
//! is empty), `0063f7b0` (remove the first item), `00905820` (add an item, given its
//! address), `005ae3d0` (add an item at the head, given its address), `005f65d0` (whether
//! the list holds the item, given its address).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `MenuTopic` (Xbox PDB), 0x2c bytes (the PC code allocates 0x2c); the fields this
    /// unit reads.
    pub struct MenuTopic: 0x2c {
        /// `pQuest` (Xbox PDB): `TESQuest*`.
        0x14 pQuest: u32,
        /// `pInfo` (Xbox PDB): `TESTopicInfo*`.
        0x18 pInfo: u32,
        /// `pCurrentResponse` (Xbox PDB): `BSSimpleList<DialogueResponse *>*`.
        0x20 pCurrentResponse: u32,
        /// `bGeneralTopic` (Xbox PDB).
        0x24 bGeneralTopic: u8,
    }

    /// `MenuTopicManager` (Xbox PDB), 0x18 bytes (the PC code allocates 0x18).
    pub struct MenuTopicManager: 0x18 {
        /// `pCurrentTopic` (Xbox PDB): `BSSimpleList<MenuTopic *>*`.
        0x00 pCurrentTopic: u32,
        /// `listTopicLists` (Xbox PDB): `BSSimpleList<BSSimpleList<MenuTopic *> *>`, inline.
        0x04 listTopicLists: u32,
        /// `pSpeaker` (Xbox PDB): `TESObjectREFR*`.
        0x0C pSpeaker: u32,
        /// `bSpeechChallengeLoss` (Xbox PDB).
        0x10 bSpeechChallengeLoss: u8,
        /// `iLastDialogXP` (Xbox PDB).
        0x14 iLastDialogXP: i32,
    }
}

/// List helpers (`thiscall`); see the module comment.
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
const LIST_ADD: u32 = 0x0090_5820;
const LIST_ADD_HEAD: u32 = 0x005a_e3d0;
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// The list constructor `0096a2d0`, destructor `0046ffb0`, the topic list's item destructor
/// `00470470` and scalar deleting destructor `004702f0` (`thiscall`, one word).
const LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
const LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
const TOPIC_LIST_CLEAR: u32 = 0x0047_0470;
const TOPIC_LIST_DELETE: u32 = 0x0047_02f0;
/// `0083fd60` (`thiscall`, one word: a compare function): sorts a topic list.
const TOPIC_LIST_SORT: u32 = 0x0083_fd60;
/// The allocator `00401000` and the free `00401030` (`cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// The `MenuTopic` destructor body `0083dec0` and constructor `0083db00` (`thiscall`, five
/// words: quest, topic, info, speaker, a flag).
const MENU_TOPIC_DESTRUCTOR: u32 = 0x0083_dec0;
const MENU_TOPIC_CONSTRUCTOR: u32 = 0x0083_db00;
/// `0083e0c0` (`thiscall`; engine map `BSSimpleList<MenuTopic_P>::AddTail`): a byte of the
/// topic, read as whether to keep it.
const MENU_TOPIC_KEEP_0083E0C0: u32 = 0x0083_e0c0;
/// `0083de60` (`thiscall`): a byte of a topic (`TESTopic`), compared with the fill flag.
const TOPIC_TEST_0083DE60: u32 = 0x0083_de60;
/// `009611e0` (`thiscall`): the word at +0x18 (a menu topic's `pInfo`).
const WORD_18: u32 = 0x0096_11e0;
/// `0045cd60` (`thiscall`): the word at +4 of a menu topic.
const WORD_4_0045CD60: u32 = 0x0045_cd60;
/// `TESTopicInfo::AddTopicList` (`0061f150`, `thiscall`) and `TESTopicInfo::RunResult`
/// (`0061f170`, `thiscall`, two words).
const ADD_TOPIC_LIST: u32 = 0x0061_f150;
const RUN_RESULT: u32 = 0x0061_f170;
/// `00619df0` and `00619b20` (`thiscall`): bytes of a topic or an info.
const TOPIC_FLAG_00619DF0: u32 = 0x0061_9df0;
const INFO_FLAG_00619B20: u32 = 0x0061_9b20;
/// `TESTopic::GetTopic` (`0061a2d0`, `cdecl`, two words).
const GET_TOPIC: u32 = 0x0061_a2d0;
/// `TESTopic::GetMatchingInfo_ov2` (`0061a760`, `thiscall`, three words).
const GET_MATCHING_INFO: u32 = 0x0061_a760;
/// `0061a790` (`thiscall`, six words): the topic's matching info.
const GET_MATCHING_INFO_0061A790: u32 = 0x0061_a790;
/// `00619d00` (`thiscall`, one word: the info): the info's quest.
const INFO_QUEST_00619D00: u32 = 0x0061_9d00;
/// `00965250` (`thiscall`, one `float`).
const SET_VALUE_00965250: u32 = 0x0096_5250;
/// The `float` `00965250` is given (`0107dd24`).
const VALUE_0107DD24: u32 = 0x0107_dd24;
/// `0055d520` (`thiscall`): the reference's name, for the message.
const REFERENCE_NAME_0055D520: u32 = 0x0055_d520;
/// `005b5e40` (`cdecl`, variable): reports a message; the format at `0107dce4`.
const REPORT_005B5E40: u32 = 0x005b_5e40;
const NO_GREETING_FORMAT: u32 = 0x0107_dce4;
/// `0062f2f0` (`thiscall`): a byte of a menu topic.
const MENU_TOPIC_FLAG_0062F2F0: u32 = 0x0062_f2f0;
/// `MenuTopicManager::PopTopicList` (`0083f1d0`, `thiscall`).
const POP_TOPIC_LIST: u32 = 0x0083_f1d0;
/// `009e32d0` then `00413f40` (`thiscall`): the info's topic list.
const INFO_OBJECT_009E32D0: u32 = 0x009e_32d0;
const OBJECT_LIST_00413F40: u32 = 0x0041_3f40;
/// `00464e30` (`thiscall`) on the player: the player's topic list.
const PLAYER_LIST_00464E30: u32 = 0x0046_4e30;
/// The player (`PlayerCharacter *`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) from `011841cc` to `011846d4`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_011841CC: u32 = 0x0118_41cc;
const CAST_TARGET_011846D4: u32 = 0x0118_46d4;
/// `0087f510` (`thiscall`): a byte of the cast speaker.
const SPEAKER_FLAG_0087F510: u32 = 0x0087_f510;
/// `0043d4d0` (`thiscall`) on `011d0dc8`: the address of a word compared with the
/// player's value 9.
const SETTING_0043D4D0: u32 = 0x0043_d4d0;
const SETTING_011D0DC8: u32 = 0x011d_0dc8;
/// `005d43c0` (`thiscall`): the speaker's extra data list; `0042df90` (`thiscall`, one
/// word) and `ExtraDataList::SetInfoGeneralTopic` (`0042dee0`, `thiscall`, one word) on it.
const EXTRA_LIST_005D43C0: u32 = 0x005d_43c0;
const GENERAL_TOPIC_0042DF90: u32 = 0x0042_df90;
const SET_INFO_GENERAL_TOPIC: u32 = 0x0042_dee0;
/// The `MenuTopicManager` singleton (`011ddde4`).
const MANAGER_SINGLETON: u32 = 0x011d_dde4;

/// The item of a list node (`*006815c0(node)`).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

// Translated from 00425ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MenuTopic` scalar deleting destructor (the engine map names the body
/// `MenuTopic::~MenuTopic`; `thiscall`, `RET 4`: the flags): runs the destructor body
/// `0083dec0` and frees the topic (`00401030`) when bit 0 of the flags is set. Returns
/// `this`.
pub fn fn_00425ff0(e: &mut Engine, this: Ptr<MenuTopic>, flags: u32) -> Ptr<MenuTopic> {
    e.call(MENU_TOPIC_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0083e170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopic::GetCurrentResponse` (engine map; `thiscall`): the response of
/// `pCurrentResponse`, or null when it is not set.
pub fn menu_topic_get_current_response(e: &mut Engine, this: Ptr<MenuTopic>) -> u32 {
    let current = e.get(this, MenuTopic::pCurrentResponse);
    if current == 0 {
        return 0;
    }
    node_item(e, current)
}

// Translated from 0083e1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MenuTopicManager` constructor (unnamed in the engine map; `thiscall`): constructs
/// the topic lists and clears the current topic and the speaker. Returns `this`.
pub fn menu_topic_manager_constructor(
    e: &mut Engine,
    this: Ptr<MenuTopicManager>,
) -> Ptr<MenuTopicManager> {
    e.call(LIST_CONSTRUCTOR, &args![this.addr() + 4]);
    e.set(this, MenuTopicManager::pCurrentTopic, 0);
    e.set(this, MenuTopicManager::pSpeaker, 0);
    this
}

// Translated from 0083e1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MenuTopicManager` destructor (unnamed in the engine map; `thiscall`): clears the
/// data ([`menu_topic_manager_clear_data`]) and destroys the topic lists (`0046ffb0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0083e1d0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn menu_topic_manager_destructor(e: &mut Engine, this: Ptr<MenuTopicManager>) {
    menu_topic_manager_clear_data(e, this);
    e.call(LIST_DESTRUCTOR, &args![this.addr() + 4]);
}

// Translated from 0083e230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopicManager::ClearData` (engine map; `thiscall`): clears the speaker, then while
/// there are topic lists takes the first off (`0063f7b0`), clears it
/// ([`menu_topic_manager_clear_topic_list`] with whether that was the last), destroys its
/// items (`00470470`) and deletes it (`004702f0` with 1, when not null).
pub fn menu_topic_manager_clear_data(e: &mut Engine, this: Ptr<MenuTopicManager>) {
    let lists = this.addr() + 4;
    e.set(this, MenuTopicManager::pSpeaker, 0);
    while e.call(LIST_IS_EMPTY, &args![lists]).u8() == 0 {
        let list = node_item(e, lists);
        e.call(LIST_REMOVE_HEAD, &args![lists]);
        let last = e.call(LIST_IS_EMPTY, &args![lists]).u8();
        menu_topic_manager_clear_topic_list(e, this, list, last);
        e.call(TOPIC_LIST_CLEAR, &args![list]);
        if list != 0 {
            e.call(TOPIC_LIST_DELETE, &args![list, 1u32]);
        }
    }
}

// Translated from 0083e2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopicManager::ClearTopicList` (engine map; `thiscall`, `RET 8`: a topic list and a
/// byte): clears the current topic; unless the byte is set it starts at the list's second
/// node; then while the node is not an empty list it deletes the node's topic unless it is
/// a general topic (`bGeneralTopic`; [`fn_00425ff0`] with 1, when not null) and removes it
/// (`0063f7b0`).
pub fn menu_topic_manager_clear_topic_list(
    e: &mut Engine,
    this: Ptr<MenuTopicManager>,
    list: u32,
    keep_first: u8,
) {
    e.set(this, MenuTopicManager::pCurrentTopic, 0);
    let mut node = list;
    if keep_first == 0 && node != 0 {
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    while node != 0 && e.call(LIST_IS_EMPTY, &args![node]).u8() == 0 {
        let topic = node_item(e, node);
        if e.mem.u8(topic + 0x24) == 0 {
            let topic = node_item(e, node);
            if topic != 0 {
                fn_00425ff0(e, Ptr::new(topic), 1);
            }
        }
        e.call(LIST_REMOVE_HEAD, &args![node]);
    }
}

/// Whether the current topic node is set and holds a topic.
fn current_holds_topic(e: &mut Engine, this: Ptr<MenuTopicManager>) -> bool {
    let current = e.get(this, MenuTopicManager::pCurrentTopic);
    current != 0 && node_item(e, current) != 0
}

// Translated from 0083e370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopicManager::FirstTopic` (engine map; `thiscall`, `RET 4`: a byte): makes the
/// first node of the shown topic list ([`fn_0083f260`]) current, its second when the byte
/// is set, and answers whether it holds a topic.
pub fn menu_topic_manager_first_topic(
    e: &mut Engine,
    this: Ptr<MenuTopicManager>,
    skip_first: u8,
) -> bool {
    let first = fn_0083f260(e, this);
    e.set(this, MenuTopicManager::pCurrentTopic, first);
    if skip_first != 0 && e.get(this, MenuTopicManager::pCurrentTopic) != 0 {
        let current = e.get(this, MenuTopicManager::pCurrentTopic);
        let next = e.call(NODE_NEXT, &args![current]).u32();
        e.set(this, MenuTopicManager::pCurrentTopic, next);
    }
    current_holds_topic(e, this)
}

// Translated from 0083e3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopicManager::NextTopic` (engine map; `thiscall`): moves the current topic to the
/// next node (when set) and answers whether it holds a topic.
pub fn menu_topic_manager_next_topic(e: &mut Engine, this: Ptr<MenuTopicManager>) -> bool {
    let current = e.get(this, MenuTopicManager::pCurrentTopic);
    if current != 0 {
        let next = e.call(NODE_NEXT, &args![current]).u32();
        e.set(this, MenuTopicManager::pCurrentTopic, next);
    }
    current_holds_topic(e, this)
}

// Translated from 0083e430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: an index): walks from the shown list's
/// second node `index` nodes on (signed count, stopping at the end), makes the node current
/// when there is one, and answers whether it holds a topic (false without a node).
pub fn fn_0083e430(e: &mut Engine, this: Ptr<MenuTopicManager>, index: i32) -> bool {
    let mut count = 0i32;
    let mut node = fn_0083f260(e, this);
    if node != 0 {
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    while node != 0 && count < index {
        node = e.call(NODE_NEXT, &args![node]).u32();
        count += 1;
    }
    if node != 0 {
        e.set(this, MenuTopicManager::pCurrentTopic, node);
    }
    node != 0 && {
        let current = e.get(this, MenuTopicManager::pCurrentTopic);
        node_item(e, current) != 0
    }
}

// Translated from 0083e4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopicManager::GetCurrentTopic` (engine map; `thiscall`): the current node's topic,
/// or null without a current node.
pub fn menu_topic_manager_get_current_topic(e: &mut Engine, this: Ptr<MenuTopicManager>) -> u32 {
    let current = e.get(this, MenuTopicManager::pCurrentTopic);
    if current == 0 {
        return 0;
    }
    node_item(e, current)
}

// Translated from 0083e4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopicManager::GetGreeting` (engine map; `thiscall`): the topic of the shown list's
/// first node.
pub fn menu_topic_manager_get_greeting(e: &mut Engine, this: Ptr<MenuTopicManager>) -> u32 {
    let first = fn_0083f260(e, this);
    node_item(e, first)
}

// Translated from 0083e510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words): deletes the `MenuTopicManager` singleton at
/// `011ddde4` ([`fn_0083e560`] with 1) and clears it, when it exists.
pub fn fn_0083e510(e: &mut Engine) {
    let manager = e.mem.u32(MANAGER_SINGLETON);
    if manager == 0 {
        return;
    }
    fn_0083e560(e, Ptr::new(manager), 1);
    e.mem.set_u32(MANAGER_SINGLETON, 0);
}

// Translated from 0083e560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MenuTopicManager` scalar deleting destructor (unnamed in the engine map;
/// `thiscall`, `RET 4`: the flags): runs [`menu_topic_manager_destructor`] and frees the
/// manager (`00401030`) when bit 0 of the flags is set. Returns `this`.
pub fn fn_0083e560(
    e: &mut Engine,
    this: Ptr<MenuTopicManager>,
    flags: u32,
) -> Ptr<MenuTopicManager> {
    menu_topic_manager_destructor(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0083e590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words): the `MenuTopicManager` singleton at `011ddde4`,
/// made on first use (0x18 bytes, [`menu_topic_manager_constructor`]; null when the
/// allocation fails).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0083e590 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_0083e590(e: &mut Engine) -> u32 {
    if e.mem.u32(MANAGER_SINGLETON) == 0 {
        let block = e.call(ALLOCATE, &args![0x18u32]).u32();
        let manager = if block != 0 {
            menu_topic_manager_constructor(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.mem.set_u32(MANAGER_SINGLETON, manager);
    }
    e.mem.u32(MANAGER_SINGLETON)
}

// Translated from 0083e620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopicManager::Initialize` (engine map; `thiscall`, `RET 8`: the speaker and a
/// topic). Clears the data, puts a new empty topic list (8 bytes, `0096a2d0`; null when the
/// allocation fails) at the head of the topic lists, keeps the speaker and clears
/// `bSpeechChallengeLoss` and `iLastDialogXP`. The topic is the given one or
/// `TESTopic::GetTopic(0, 0)`; its matching info for the speaker and the player
/// (`TESTopic::GetMatchingInfo_ov2`, with an out byte on the stack) makes the greeting
/// topic (a 0x2c-byte `MenuTopic`, `0083db00` with the info's quest, the topic, the info,
/// the speaker and 0), which gets the float at `0107dd24` (`00965250` on its word +4), is
/// asked `0083e0c0` and is added to the shown list. With a greeting its info's topic list
/// is added and its result script run (with 0 and the speaker); without one the format at
/// `0107dce4` is reported with the speaker's name (`005b5e40` with 6). The first topic is
/// then made current ([`menu_topic_manager_first_topic`] with 0), and the answer is whether
/// the greeting's info answers true from `00619df0`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0083e620 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn menu_topic_manager_initialize(
    e: &mut Engine,
    this: Ptr<MenuTopicManager>,
    speaker: u32,
    topic: u32,
) -> bool {
    menu_topic_manager_clear_data(e, this);
    e.with_stack(0xc, |e, locals| {
        let list_slot = locals.addr();
        let found = locals.addr() + 4;
        let greeting_slot = locals.addr() + 8;
        let block = e.call(ALLOCATE, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(LIST_CONSTRUCTOR, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(list_slot, list);
        e.call(LIST_ADD_HEAD, &args![this.addr() + 4, list_slot]);
        e.set(this, MenuTopicManager::pSpeaker, speaker);
        e.set(this, MenuTopicManager::bSpeechChallengeLoss, 0);
        e.set(this, MenuTopicManager::iLastDialogXP, 0);
        let mut topic = topic;
        if topic == 0 {
            topic = e.call(GET_TOPIC, &args![0u32, 0u32]).u32();
        }
        e.mem.set_u32(greeting_slot, 0);
        if topic != 0 {
            let player = e.mem.u32(PLAYER);
            let speaker = e.get(this, MenuTopicManager::pSpeaker);
            let info = e
                .call(GET_MATCHING_INFO, &args![topic, found, speaker, player])
                .u32();
            if info != 0 {
                let quest = e.call(INFO_QUEST_00619D00, &args![topic, info]).u32();
                let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
                let greeting = if block != 0 {
                    let speaker = e.get(this, MenuTopicManager::pSpeaker);
                    e.call(
                        MENU_TOPIC_CONSTRUCTOR,
                        &args![block, quest, topic, info, speaker, 0u32],
                    )
                    .u32()
                } else {
                    0
                };
                e.mem.set_u32(greeting_slot, greeting);
                let value = x87_quiet_bits(e.mem.u32(VALUE_0107DD24));
                let object = e.call(WORD_4_0045CD60, &args![greeting]).u32();
                e.call(SET_VALUE_00965250, &args![object, value]);
                e.call(MENU_TOPIC_KEEP_0083E0C0, &args![greeting]);
                let shown = fn_0083f260(e, this);
                e.call(LIST_ADD, &args![shown, greeting_slot]);
            }
        }
        let greeting = e.mem.u32(greeting_slot);
        if greeting != 0 {
            let info = e.call(WORD_18, &args![greeting]).u32();
            e.call(ADD_TOPIC_LIST, &args![info]);
            let speaker = e.get(this, MenuTopicManager::pSpeaker);
            let info = e.call(WORD_18, &args![greeting]).u32();
            e.call(RUN_RESULT, &args![info, 0u32, speaker]);
        } else {
            let speaker = e.get(this, MenuTopicManager::pSpeaker);
            let name = e.call(REFERENCE_NAME_0055D520, &args![speaker]).u32();
            e.call(REPORT_005B5E40, &args![6u32, NO_GREETING_FORMAT, name]);
        }
        menu_topic_manager_first_topic(e, this, 0);
        greeting != 0 && {
            let info = e.call(WORD_18, &args![greeting]).u32();
            info != 0 && {
                let info = e.call(WORD_18, &args![greeting]).u32();
                e.call(TOPIC_FLAG_00619DF0, &args![info]).u8() != 0
            }
        }
    })
}

// Translated from 0083ebb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopicManager::ProcessCurrentTopic` (engine map; `thiscall`): runs the result
/// script (`TESTopicInfo::RunResult` with 0 and the speaker) of the current topic's info,
/// unless that info has bit 0x40 of its byte +0x25 ([`fn_0083ec10`]).
pub fn menu_topic_manager_process_current_topic(e: &mut Engine, this: Ptr<MenuTopicManager>) {
    let topic = menu_topic_manager_get_current_topic(e, this);
    let mut info = 0;
    if topic != 0 {
        let candidate = e.call(WORD_18, &args![topic]).u32();
        if !fn_0083ec10(e, Ptr::new(candidate)) {
            info = e.call(WORD_18, &args![topic]).u32();
        }
    }
    if info != 0 {
        let speaker = e.get(this, MenuTopicManager::pSpeaker);
        e.call(RUN_RESULT, &args![info, 0u32, speaker]);
    }
}

// Translated from 0083ec10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): whether bit 0x40 of the byte at `this + 0x25`
/// (of a topic info) is set.
pub fn fn_0083ec10(e: &mut Engine, this: Ptr) -> bool {
    (e.mem.i8(this.addr() + 0x25) as i32) & 0x40 != 0
}

// Translated from 0083ec30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopicManager::LoadNextTopicList` (engine map; `thiscall`). Adds the current
/// topic's info topic list (`TESTopicInfo::AddTopicList`) when it has an info. After a
/// lost speech challenge (`bSpeechChallengeLoss`) with a current topic whose `0062f2f0`
/// answers false, the topic list is popped (`MenuTopicManager::PopTopicList`) and the flag
/// cleared. Otherwise, for a current topic that is not general: the next topics are its
/// info's list (`009e32d0`, `00413f40`) when `0062f2f0` answers true, else the player's
/// (`00464e30`); with a list the shown list is cleared
/// ([`menu_topic_manager_clear_topic_list`] with 0) and filled
/// ([`menu_topic_manager_fill_topic_list`] with 0, that answer and the topic's word +4),
/// and `bSpeechChallengeLoss` is cleared when set.
pub fn menu_topic_manager_load_next_topic_list(e: &mut Engine, this: Ptr<MenuTopicManager>) {
    let topic = menu_topic_manager_get_current_topic(e, this);
    if topic != 0 && e.call(WORD_18, &args![topic]).u32() != 0 {
        let info = e.call(WORD_18, &args![topic]).u32();
        e.call(ADD_TOPIC_LIST, &args![info]);
    }
    if e.get(this, MenuTopicManager::bSpeechChallengeLoss) != 0
        && topic != 0
        && e.call(MENU_TOPIC_FLAG_0062F2F0, &args![topic]).u8() == 0
    {
        e.call(POP_TOPIC_LIST, &args![this]);
        e.set(this, MenuTopicManager::bSpeechChallengeLoss, 0);
        return;
    }
    if topic == 0 || e.mem.u8(topic + 0x24) != 0 {
        return;
    }
    let from_info = e.call(MENU_TOPIC_FLAG_0062F2F0, &args![topic]).u8();
    let list = if from_info != 0 {
        let info = e.call(WORD_18, &args![topic]).u32();
        let object = e.call(INFO_OBJECT_009E32D0, &args![info]).u32();
        e.call(OBJECT_LIST_00413F40, &args![object]).u32()
    } else {
        let player = e.mem.u32(PLAYER);
        e.call(PLAYER_LIST_00464E30, &args![player]).u32()
    };
    let word = e.call(WORD_4_0045CD60, &args![topic]).u32();
    if list != 0 {
        let shown = fn_0083f260(e, this);
        menu_topic_manager_clear_topic_list(e, this, shown, 0);
        menu_topic_manager_fill_topic_list(e, this, list, 0, from_info, word);
    }
    if e.get(this, MenuTopicManager::bSpeechChallengeLoss) != 0 {
        e.set(this, MenuTopicManager::bSpeechChallengeLoss, 0);
    }
}

/// Adds the menu topic in the word at `slot` to the shown list unless it is there already
/// (`005f65d0`, `00905820`).
fn add_unless_listed(e: &mut Engine, this: Ptr<MenuTopicManager>, slot: u32) {
    let shown = fn_0083f260(e, this);
    if e.call(LIST_CONTAINS, &args![shown, slot]).u8() == 0 {
        let shown = fn_0083f260(e, this);
        e.call(LIST_ADD, &args![shown, slot]);
    }
}

// Translated from 0083ed50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopicManager::FillTopicList` (engine map; `thiscall`, `RET 0x10`: a list of
/// topics, a byte, a byte, a word). The speaker is cast (`__RTDynamicCast` from `011841cc`
/// to `011846d4`). For each topic of the list (until an empty node):
///
/// - the topic is taken only when `0083de60` answers what the first byte asks (true when
///   it is set, false when not);
/// - its kind ([`fn_0083f0d0`]) is compared with the player's value 9 (the object at the
///   player's +0xa4, its slot +8) against the word `0043d4d0` returns for `011d0dc8`
///   (signed): above it kind 1 is skipped, otherwise kind 3;
/// - when the topic answers true from `00619df0` and the cast speaker false from
///   `0087f510`, the speaker's general topic (`0042df90` on its extra data list) is added
///   once (when `0083e0c0` keeps it and it is not listed yet);
/// - its matching info (`0061a790` with an out byte, the speaker, the player, 0, the word,
///   0) makes a `MenuTopic` (0x2c bytes, `0083db00` with the info's quest, the topic, the
///   info, the speaker and the out byte) unless the topic answers true from `00619df0`
///   while the general topic was added or the cast speaker answers true; it is added when
///   kept and not listed yet, deleted otherwise. A topic answering true from `00619df0`,
///   added before the general topic, whose info answers false from `00619b20`, becomes the
///   speaker's general topic (`ExtraDataList::SetInfoGeneralTopic`) and the speaker's slot
///   +0x48 gets 0x80000000.
///
/// Unless the second byte is set, the shown list is then sorted (`0083fd60` with the
/// compare function `0083f110`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0083ed50 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn menu_topic_manager_fill_topic_list(
    e: &mut Engine,
    this: Ptr<MenuTopicManager>,
    list: u32,
    wanted: u8,
    keep_order: u8,
    word: u32,
) {
    let mut general_added = false;
    let speaker = e.get(this, MenuTopicManager::pSpeaker);
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![
                speaker,
                0u32,
                CAST_SOURCE_011841CC,
                CAST_TARGET_011846D4,
                0u32
            ],
        )
        .u32();
    e.with_stack(0xc, |e, locals| {
        let general_slot = locals.addr();
        let found = locals.addr() + 4;
        let made_slot = locals.addr() + 8;
        let mut node = list;
        while node != 0 && node_item(e, node) != 0 {
            let topic = node_item(e, node);
            node = e.call(NODE_NEXT, &args![node]).u32();
            if wanted != 0 {
                if e.call(TOPIC_TEST_0083DE60, &args![topic]).u8() == 0 {
                    continue;
                }
            } else if e.call(TOPIC_TEST_0083DE60, &args![topic]).u8() != 0 {
                continue;
            }
            let kind = fn_0083f0d0(e, Ptr::new(topic));
            let player = e.mem.u32(PLAYER);
            let value = e.vcall(player.wrapping_add(0xa4), 8, &args![9u32]).i32();
            let limit = e.call(SETTING_0043D4D0, &args![SETTING_011D0DC8]).u32();
            if value > e.mem.i32(limit) {
                if kind == 1 {
                    continue;
                }
            } else if kind == 3 {
                continue;
            }
            if e.call(TOPIC_FLAG_00619DF0, &args![topic]).u8() != 0
                && e.call(SPEAKER_FLAG_0087F510, &args![cast]).u8() == 0
            {
                e.mem.set_u32(general_slot, 0);
                let speaker = e.get(this, MenuTopicManager::pSpeaker);
                let extra = e.call(EXTRA_LIST_005D43C0, &args![speaker]).u32();
                let general = e.call(GENERAL_TOPIC_0042DF90, &args![extra, speaker]).u32();
                e.mem.set_u32(general_slot, general);
                if general != 0 && !general_added {
                    if e.call(MENU_TOPIC_KEEP_0083E0C0, &args![general]).u8() != 0 && !general_added
                    {
                        add_unless_listed(e, this, general_slot);
                    }
                    general_added = true;
                }
            }
            e.mem.set_u8(found, 0);
            let speaker = e.get(this, MenuTopicManager::pSpeaker);
            let player = e.mem.u32(PLAYER);
            let info = e
                .call(
                    GET_MATCHING_INFO_0061A790,
                    &args![topic, found, speaker, player, 0u32, word, 0u32],
                )
                .u32();
            if info == 0 {
                continue;
            }
            let quest = e.call(INFO_QUEST_00619D00, &args![topic, info]).u32();
            let mut make_general = false;
            let mut make = true;
            if e.call(TOPIC_FLAG_00619DF0, &args![topic]).u8() != 0 {
                if !general_added && e.call(SPEAKER_FLAG_0087F510, &args![cast]).u8() == 0 {
                    make_general = true;
                } else {
                    make = false;
                }
            }
            e.mem.set_u32(made_slot, 0);
            if make {
                let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
                let made = if block != 0 {
                    let flag = u32::from(e.mem.u8(found));
                    let speaker = e.get(this, MenuTopicManager::pSpeaker);
                    e.call(
                        MENU_TOPIC_CONSTRUCTOR,
                        &args![block, quest, topic, info, speaker, flag],
                    )
                    .u32()
                } else {
                    0
                };
                e.mem.set_u32(made_slot, made);
            }
            let added = make && {
                let made = e.mem.u32(made_slot);
                e.call(MENU_TOPIC_KEEP_0083E0C0, &args![made]).u8() != 0 && {
                    let shown = fn_0083f260(e, this);
                    e.call(LIST_CONTAINS, &args![shown, made_slot]).u8() == 0
                }
            };
            if added {
                let shown = fn_0083f260(e, this);
                e.call(LIST_ADD, &args![shown, made_slot]);
            } else if e.mem.u32(made_slot) != 0 {
                let made = e.mem.u32(made_slot);
                fn_00425ff0(e, Ptr::new(made), 1);
                e.mem.set_u32(made_slot, 0);
            }
            let made = e.mem.u32(made_slot);
            if make_general && made != 0 {
                let info = e.call(WORD_18, &args![made]).u32();
                if e.call(INFO_FLAG_00619B20, &args![info]).u8() == 0 {
                    let speaker = e.get(this, MenuTopicManager::pSpeaker);
                    let extra = e.call(EXTRA_LIST_005D43C0, &args![speaker]).u32();
                    e.call(SET_INFO_GENERAL_TOPIC, &args![extra, made]);
                    let speaker = e.get(this, MenuTopicManager::pSpeaker);
                    e.vcall(speaker, 0x48, &args![0x8000_0000u32]);
                }
            }
        }
    });
    if keep_order == 0 {
        let shown = fn_0083f260(e, this);
        e.call(TOPIC_LIST_SORT, &args![shown, 0x0083_f110u32]);
    }
}

// Translated from 0083f0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): a topic's kind from its byte +0x25: 1 when bit
/// 0x10 is set, else 3 when bit 0x20 is set, else 2.
pub fn fn_0083f0d0(e: &mut Engine, this: Ptr) -> u32 {
    let flags = e.mem.i8(this.addr() + 0x25) as i32;
    if flags & 0x10 != 0 {
        1
    } else {
        u32::from(flags & 0x20 != 0) + 2
    }
}

// Translated from 0083f110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the compare function [`menu_topic_manager_fill_topic_list`]
/// sorts with (`cdecl`, two menu topics). Each topic's value is `00526ae0` (in ST0) of its
/// word +4 (`0045cd60`), the first kept as a `double`: -1 when the second's is below the
/// first's, 1 when above, 0 when equal or unordered (each comparison fetches both values
/// again).
pub fn fn_0083f110(e: &mut Engine, first: u32, second: u32) -> i32 {
    let value = |e: &mut Engine, topic: u32| {
        let object = e.call(WORD_4_0045CD60, &args![topic]).u32();
        e.call(VALUE_00526AE0, &args![object]).f64()
    };
    let a = value(e, first);
    let b = value(e, second);
    // FCOMP double; TEST AH,5; JP: -1 only when b < a.
    if b < a {
        return -1;
    }
    let a = value(e, first);
    let b = value(e, second);
    // TEST AH,0x41; JNZ: 1 only when b > a.
    i32::from(b > a)
}

/// `00526ae0` (`thiscall`): a `float` in ST0 (the menu topic's sort value).
const VALUE_00526AE0: u32 = 0x0052_6ae0;

// Translated from 0083f1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a topic list): adds a non-null list at
/// the head of the topic lists (`005ae3d0` with the address of the stack word holding it).
pub fn fn_0083f1a0(e: &mut Engine, this: Ptr<MenuTopicManager>, list: u32) {
    if list == 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), list);
        e.call(LIST_ADD_HEAD, &args![this.addr() + 4, slot]);
    });
}

// Translated from 0083f260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The shown topic list: the item of the topic lists' first node (`006815c0` on
/// `this + 4`). The engine map's name for the body,
/// `BSSimpleList<BSSimpleList<MenuTopic_P>_P>::AddHead`, does not fit it (`thiscall`).
pub fn fn_0083f260(e: &mut Engine, this: Ptr<MenuTopicManager>) -> u32 {
    node_item(e, this.addr() + 4)
}

/// `007627c0` (`thiscall` on an info): whether it is a speech challenge (`AL`).
const INFO_IS_CHALLENGE_007627C0: u32 = 0x0076_27c0;
/// The speaker's virtual slots `0x100` (whether it is an actor, `AL`), `0x344` (with the player
/// and 0: an integer), `0xfc` (`AL`) and `0x460` (with the player and a `float`).
const SPEAKER_IS_ACTOR_SLOT: u32 = 0x100;
const SPEAKER_VALUE_SLOT: u32 = 0x344;
const SPEAKER_FLAG_SLOT: u32 = 0xfc;
const SPEAKER_SET_SLOT: u32 = 0x460;
/// The setting `011d0a24` (its value's address through `00403e20`): the challenge difficulty
/// used for a speaker that is not an actor.
const SETTING_011D0A24: u32 = 0x011d_0a24;
const SETTING_ADDRESS_00403E20: u32 = 0x0040_3e20;
/// `0061f2b0` (`thiscall` on an info): the challenge's word passed to `00648ea0`.
const INFO_CHALLENGE_0061F2B0: u32 = 0x0061_f2b0;
/// `ActorValueOwner::GetClampedActorFloatValue` (engine map; `0066ef50`, `thiscall` on the
/// player's `+0xa4`, one word: the actor value; `float` in ST0), here value `0x2b`.
const CLAMPED_ACTOR_VALUE_0066EF50: u32 = 0x0066_ef50;
/// `00648ea0` (`cdecl`: two `float`s and a word): the success chance; `00648f60` (`cdecl`, one
/// word: the chance; `float` in ST0).
const CHALLENGE_CHANCE_00648EA0: u32 = 0x0064_8ea0;
const CHALLENGE_VALUE_00648F60: u32 = 0x0064_8f60;
/// `00487f50` (no arguments): a random number.
const RANDOM_00487F50: u32 = 0x0048_7f50;
/// `008041a0` (`thiscall` on an info: its level) and
/// `ExperiencePoints::GetExperiencePoints` (engine map; `006705b0`, `cdecl`: 4 and the level
/// minus one).
const INFO_LEVEL_008041A0: u32 = 0x0080_41a0;
const EXPERIENCE_POINTS_006705B0: u32 = 0x0067_05b0;
/// `MiscStatManager::Increment` (engine map; `004d5c60`, `cdecl`, one word).
const MISC_STAT_INCREMENT_004D5C60: u32 = 0x004d_5c60;
/// `"SpeechChallengeFailure topic had no matching info ..."` (`0107dd28`).
const NO_FAILURE_INFO_MESSAGE: u32 = 0x0107_dd28;
/// `0061f220`, `0061aef0` (`thiscall` on an info; `0061aef0` answers in `AL`) and `00935920`
/// (`thiscall` on the speaker, one word: the info).
const INFO_0061F220: u32 = 0x0061_f220;
const INFO_FLAG_0061AEF0: u32 = 0x0061_aef0;
const SPEAKER_INFO_00935920: u32 = 0x0093_5920;
/// `00905330` (`thiscall` on a list, one word: the address of an item): removes the item.
const LIST_REMOVE_00905330: u32 = 0x0090_5330;

/// The save/load singleton holder (`011de45c`) and its `thiscall` helpers: `00825c00` (the
/// current position), `008579b0` (write: address, size), `00857a10` (`SaveNumericID`),
/// `008579e0` (read: address, size), `00857aa0` (`LoadNumericID`), `00862110`
/// (`UseSaveGameBlocks`, `AL`), `004fd3e0` / `004fd3c0` (the form being saved / loaded, or 0)
/// and `008df040` (the version, `AL`).
const SAVE_LOAD_GAME_011DE45C: u32 = 0x011d_e45c;
const SAVE_POSITION_00825C00: u32 = 0x0082_5c00;
const SAVE_WRITE_008579B0: u32 = 0x0085_79b0;
const SAVE_NUMERIC_ID_00857A10: u32 = 0x0085_7a10;
const LOAD_READ_008579E0: u32 = 0x0085_79e0;
const LOAD_NUMERIC_ID_00857AA0: u32 = 0x0085_7aa0;
const USE_SAVE_GAME_BLOCKS_00862110: u32 = 0x0086_2110;
const SAVING_FORM_004FD3E0: u32 = 0x004f_d3e0;
const LOADING_FORM_004FD3C0: u32 = 0x004f_d3c0;
const SAVE_VERSION_008DF040: u32 = 0x008d_f040;
/// The setting at `011de4e8` (its value's address through `00408d60`): whether save sizes are
/// reported (`Error`, `0040fbe0`, `cdecl`, variable).
const SETTING_011DE4E8: u32 = 0x011d_e4e8;
const SETTING_VALUE_00408D60: u32 = 0x0040_8d60;
const ERROR_0040FBE0: u32 = 0x0040_fbe0;
/// `"D:\_Fallout3\...\MenuTopic.cpp"` (`0107dd88`) and the messages: the save size
/// (`01012cb0` with a form, `01012c78` without), the save (`010153a0`, `0101536c`), the block
/// size (`01015318`), the block header (`01015718`, `010156a8`) and the overrun and underrun
/// (`01015588`, `01015500` with a form, `010154a0`, `01015440` without).
const SOURCE_FILE_0107DD88: u32 = 0x0107_dd88;
const SAVE_SIZE_FORM_MESSAGE: u32 = 0x0101_2cb0;
const SAVE_SIZE_MESSAGE: u32 = 0x0101_2c78;
const SAVE_FORM_MESSAGE: u32 = 0x0101_53a0;
const SAVE_MESSAGE: u32 = 0x0101_536c;
const BLOCK_TOO_LARGE_MESSAGE: u32 = 0x0101_5318;
const HEADER_FORM_MESSAGE: u32 = 0x0101_5718;
const HEADER_MESSAGE: u32 = 0x0101_56a8;
const OVERRUN_FORM_MESSAGE: u32 = 0x0101_5588;
const UNDERRUN_FORM_MESSAGE: u32 = 0x0101_5500;
const OVERRUN_MESSAGE: u32 = 0x0101_54a0;
const UNDERRUN_MESSAGE: u32 = 0x0101_5440;
/// The block tag `"KOLB"` read as a word (`0x424c4f4b`).
const BLOCK_TAG: u32 = 0x424c_4f4b;
/// `strlen` (`0044a670`, `cdecl`), `00559450` (`thiscall`: the string's characters), `memset`
/// (`00403d30`, `cdecl`), the string setters `004037f0` (two words) and `00438390` (one word).
const STRLEN_0044A670: u32 = 0x0044_a670;
const STRING_CHARS_00559450: u32 = 0x0055_9450;
const MEMSET_00403D30: u32 = 0x0040_3d30;
const STRING_SET_004037F0: u32 = 0x0040_37f0;
const STRING_SET_00438390: u32 = 0x0043_8390;
/// A form's save id (`0084e3a0`, `thiscall`) and `004839c0` (`cdecl`, one word).
const FORM_SAVE_ID_0084E3A0: u32 = 0x0084_e3a0;
const CAST_SOURCE_004839C0: u32 = 0x0048_39c0;
/// The type descriptors: the source `01183028` and the targets quest (`01186500`), topic
/// (`01184720`) and info (`01184738`).
const FORM_TYPE_01183028: u32 = 0x0118_3028;
const QUEST_TYPE_01186500: u32 = 0x0118_6500;
const TOPIC_TYPE_01184720: u32 = 0x0118_4720;
const INFO_TYPE_01184738: u32 = 0x0118_4738;
/// `BGSSaveGameBuffer` writers (`SaveString` `00865e70`, two words; the raw write `00865e50`,
/// three words; `SaveFormID_ov2` `00865df0`, two words) and `BGSLoadGameBuffer` readers
/// (`LoadString` `008649a0`, one word; the raw read `00864980`, two words; `LoadFormID`
/// `008648a0`), all `thiscall` on the buffer.
const SAVE_STRING_00865E70: u32 = 0x0086_5e70;
const SAVE_DATA_00865E50: u32 = 0x0086_5e50;
const SAVE_FORM_ID_00865DF0: u32 = 0x0086_5df0;
const LOAD_STRING_008649A0: u32 = 0x0086_49a0;
const LOAD_DATA_00864980: u32 = 0x0086_4980;
const LOAD_FORM_ID_008648A0: u32 = 0x0086_48a0;
/// The security cookie (`011c16bc`) and `__security_check_cookie` (`00ec408c`, fastcall).
const SECURITY_COOKIE_011C16BC: u32 = 0x011c_16bc;
const SECURITY_CHECK_COOKIE: u32 = 0x00ec_408c;

/// The save/load singleton.
fn save_load(e: &Engine) -> u32 {
    e.global::<u32>(SAVE_LOAD_GAME_011DE45C)
}

/// `__RTDynamicCast(004839c0(form), 0, 01183028, target, 0)`.
fn cast_form(e: &mut Engine, form: u32, target: u32) -> u32 {
    let object = e.call(CAST_SOURCE_004839C0, &args![form]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![object, 0u32, FORM_TYPE_01183028, target, 0u32],
    )
    .u32()
}

/// Whether the setting at `011de4e8` is set (the byte at the address `00408d60` gives).
fn reporting_sizes(e: &mut Engine) -> bool {
    let value = e
        .call(SETTING_VALUE_00408D60, &args![SETTING_011DE4E8])
        .u32();
    e.mem.u8(value) != 0
}

/// The size reports of [`fn_0083f280`] and [`fn_0083f3e0`]: with the form being saved
/// (`004fd3e0`), `Error(form_message, size, form id, its name (slot 0x130 of 004839c0(form)),
/// the word at +5, line, file)`; without, `Error(message, size, line, file)`.
fn report_size(e: &mut Engine, size: u32, line: u32, form_message: u32, message: u32) {
    let saver = save_load(e);
    let form = e.call(SAVING_FORM_004FD3E0, &args![saver]).u32();
    if form != 0 {
        let id = e.mem.u32(form);
        let object = e.call(CAST_SOURCE_004839C0, &args![id]).u32();
        let flags = e.mem.u32(form + 5);
        let name = e.vcall(object, 0x130, &args![]).u32();
        let id = e.mem.u32(form);
        e.call(
            ERROR_0040FBE0,
            &args![
                form_message,
                size,
                id,
                name,
                flags,
                line,
                SOURCE_FILE_0107DD88
            ],
        );
    } else {
        e.call(
            ERROR_0040FBE0,
            &args![message, size, line, SOURCE_FILE_0107DD88],
        );
    }
}

/// The form-load reports of [`menu_topic_load_game`]: with the form being loaded (`004fd3c0`),
/// `005b5e40(form_message, file, line, form id, its name, the byte at +9, the word at
/// +5)`; without, `005b5e40(message, file, line, version)`.
fn report_load(e: &mut Engine, line: u32, form_message: u32, message: u32) {
    let loader = save_load(e);
    let form = e.call(LOADING_FORM_004FD3C0, &args![loader]).u32();
    if form != 0 {
        let id = e.mem.u32(form);
        let object = e.call(CAST_SOURCE_004839C0, &args![id]).u32();
        let flags = e.mem.u32(form + 5);
        let version = u32::from(e.mem.u8(form + 9));
        let name = e.vcall(object, 0x130, &args![]).u32();
        let id = e.mem.u32(form);
        e.call(
            REPORT_005B5E40,
            &args![
                form_message,
                SOURCE_FILE_0107DD88,
                line,
                id,
                name,
                version,
                flags
            ],
        );
    } else {
        let loader = save_load(e);
        let version = u32::from(e.call(SAVE_VERSION_008DF040, &args![loader]).u8());
        e.call(
            REPORT_005B5E40,
            &args![message, SOURCE_FILE_0107DD88, line, version],
        );
    }
}

// Translated from 0083e850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083e850 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `MenuTopicManager::DoSpeechChallengeCheck` (engine map; `thiscall`, `RET 4`: a menu topic).
/// For a topic whose info (`009611e0`) is a speech challenge (`007627c0`): the difficulty is the
/// speaker's slot `0x344` (with the player and 0, an integer made a `float`) for an actor (slot
/// `0x100`), else the setting `011d0a24`'s `float`; the chance is `00648ea0(speech (the player's
/// clamped actor value 0x2b), difficulty, 0061f2b0(info))`. A random number (`00487f50`) mod 100
/// below the chance (signed) wins: `iLastDialogXP` = `GetExperiencePoints(4, level - 1)`.
/// Otherwise `bSpeechChallengeLoss` is set, the stat `0x1f` incremented, and the failure topic
/// (`TESTopic::GetTopic(0, 0xb)`) gives a matching info (`0061a790` with a result byte, the
/// speaker, the player, 0, the topic's word at +4, 0); without one the message `0107dd28` is
/// reported (`005b5e40` with 6). With one a new topic list (`operator new(8)`, `0096a2d0`) gets
/// the greeting and a new menu topic (`operator new(0x2c)`, `0083db00` with the info's quest,
/// the failure topic, the info, the speaker, 0; `0083e0c0` on it); the challenged info gets
/// `0061f220`; when `00619b20` or `0061aef0` answer for it, `00935920` runs (when `0061aef0` and
/// the speaker's slot `0xfc` do) and the topic is removed from the shown list (`00905330`). The
/// list is shown ([`fn_0083f1a0`], [`menu_topic_manager_first_topic`] with 1). An actor speaker
/// then gets its slot `0x460` with the player and `00648f60(chance)`. Returns
/// `bSpeechChallengeLoss`.
pub fn menu_topic_manager_do_speech_challenge_check(
    e: &mut Engine,
    this: Ptr<MenuTopicManager>,
    topic: u32,
) -> u8 {
    let base = this.addr();
    let speaker = move |e: &Engine| e.mem.u32(base + 0xc);
    if topic == 0 {
        return e.get(this, MenuTopicManager::bSpeechChallengeLoss);
    }
    let info = e.call(WORD_18, &args![topic]).u32();
    if e.call(INFO_IS_CHALLENGE_007627C0, &args![info]).u8() == 0 {
        return e.get(this, MenuTopicManager::bSpeechChallengeLoss);
    }
    let difficulty = if e.vcall(speaker(e), SPEAKER_IS_ACTOR_SLOT, &args![]).u8() != 0 {
        let player = e.global::<u32>(PLAYER);
        let value = e
            .vcall(speaker(e), SPEAKER_VALUE_SLOT, &args![player, 0u32])
            .i32();
        x87_store32(f64::from(value))
    } else {
        let value = e
            .call(SETTING_ADDRESS_00403E20, &args![SETTING_011D0A24])
            .u32();
        x87_quiet_bits(e.mem.u32(value))
    };
    let info = e.call(WORD_18, &args![topic]).u32();
    let challenge = e.call(INFO_CHALLENGE_0061F2B0, &args![info]).u32();
    let player = e.global::<u32>(PLAYER);
    let speech = e
        .call(
            CLAMPED_ACTOR_VALUE_0066EF50,
            &args![player.wrapping_add(0xa4), 0x2bu32],
        )
        .f64();
    let chance = e
        .call(
            CHALLENGE_CHANCE_00648EA0,
            &args![x87_store32(speech), difficulty, challenge],
        )
        .i32();
    let roll = e.call(RANDOM_00487F50, &args![]).u32() % 100;
    if (roll as i32) < chance {
        let info = e.call(WORD_18, &args![topic]).u32();
        let level = e.call(INFO_LEVEL_008041A0, &args![info]).u32();
        let points = e
            .call(
                EXPERIENCE_POINTS_006705B0,
                &args![4u32, level.wrapping_sub(1)],
            )
            .u32();
        e.mem.set_u32(base + 0x14, points);
        return e.get(this, MenuTopicManager::bSpeechChallengeLoss);
    }
    e.set(this, MenuTopicManager::bSpeechChallengeLoss, 1);
    e.call(MISC_STAT_INCREMENT_004D5C60, &args![0x1fu32]);
    let failure = e.call(GET_TOPIC, &args![0u32, 0xbu32]).u32();
    e.with_stack(0x10, |e, locals| {
        let found = locals.addr();
        let greeting_at = locals.addr() + 4;
        let topic_at = locals.addr() + 8;
        let argument_at = locals.addr() + 0xc;
        let word = e.call(WORD_4_0045CD60, &args![topic]).u32();
        let player = e.global::<u32>(PLAYER);
        let matching = e
            .call(
                GET_MATCHING_INFO_0061A790,
                &args![failure, found, speaker(e), player, 0u32, word, 0u32],
            )
            .u32();
        if matching == 0 {
            e.call(REPORT_005B5E40, &args![6u32, NO_FAILURE_INFO_MESSAGE]);
            return;
        }
        let block = e.call(ALLOCATE, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(LIST_CONSTRUCTOR, &args![block]).u32()
        } else {
            0
        };
        let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
        let made = if block != 0 {
            let quest = e.call(INFO_QUEST_00619D00, &args![failure, matching]).u32();
            e.call(
                MENU_TOPIC_CONSTRUCTOR,
                &args![block, quest, failure, matching, speaker(e), 0u32],
            )
            .u32()
        } else {
            0
        };
        e.mem.set_u32(topic_at, made);
        e.call(MENU_TOPIC_KEEP_0083E0C0, &args![made]);
        let info = e.call(WORD_18, &args![topic]).u32();
        e.call(INFO_0061F220, &args![info]);
        let info = e.call(WORD_18, &args![topic]).u32();
        let mut remove = e.call(INFO_FLAG_00619B20, &args![info]).u8() != 0;
        if !remove {
            let info = e.call(WORD_18, &args![topic]).u32();
            remove = e.call(INFO_FLAG_0061AEF0, &args![info]).u8() != 0;
        }
        if remove {
            let info = e.call(WORD_18, &args![topic]).u32();
            if e.call(INFO_FLAG_0061AEF0, &args![info]).u8() != 0
                && e.vcall(speaker(e), SPEAKER_FLAG_SLOT, &args![]).u8() != 0
            {
                let held = speaker(e);
                let info = e.call(WORD_18, &args![topic]).u32();
                e.call(SPEAKER_INFO_00935920, &args![held, info]);
            }
            e.mem.set_u32(argument_at, topic);
            let shown = fn_0083f260(e, this);
            e.call(LIST_REMOVE_00905330, &args![shown, argument_at]);
        }
        let greeting = menu_topic_manager_get_greeting(e, this);
        e.mem.set_u32(greeting_at, greeting);
        e.call(LIST_ADD, &args![list, greeting_at]);
        e.call(LIST_ADD, &args![list, topic_at]);
        fn_0083f1a0(e, this, list);
        menu_topic_manager_first_topic(e, this, 1);
    });
    if e.vcall(speaker(e), SPEAKER_IS_ACTOR_SLOT, &args![]).u8() != 0 {
        let value = e
            .call(CHALLENGE_VALUE_00648F60, &args![chance as u32])
            .f64();
        let player = e.global::<u32>(PLAYER);
        e.vcall(
            speaker(e),
            SPEAKER_SET_SLOT,
            &args![player, x87_store32(value)],
        );
    }
    e.get(this, MenuTopicManager::bSpeechChallengeLoss)
}

// Translated from 0083f1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopicManager::PopTopicList` (engine map; `thiscall`): when the topic lists have a
/// second node and the first holds a list, that list is removed (`0063f7b0`), cleared
/// ([`menu_topic_manager_clear_topic_list`] with 0), its items destroyed (`00470470`) and
/// deleted (`004702f0` with 1). Returns the list now shown (the first node's item).
pub fn menu_topic_manager_pop_topic_list(e: &mut Engine, this: Ptr<MenuTopicManager>) -> u32 {
    let lists = this.addr() + 4;
    if e.call(NODE_NEXT, &args![lists]).u32() != 0 {
        let list = node_item(e, lists);
        if list != 0 {
            e.call(LIST_REMOVE_HEAD, &args![lists]);
            menu_topic_manager_clear_topic_list(e, this, list, 0);
            e.call(TOPIC_LIST_CLEAR, &args![list]);
            if list != 0 {
                e.call(TOPIC_LIST_DELETE, &args![list, 1u32]);
            }
        }
    }
    node_item(e, lists)
}

// Translated from 0083f280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a `MenuTopic`): its size in the older save format as a 16-bit value:
/// 6 with save-game blocks (`00862110`), then the text's length plus one (`strlen` of
/// `00559450`), three bytes and three words. With the setting `011de4e8` set the size is
/// reported (line `0x2d9`, the messages `01012cb0` / `01012c78`).
pub fn fn_0083f280(e: &mut Engine, this: Ptr<MenuTopic>) -> u16 {
    let start = 0u16;
    let mut size = start;
    let saver = save_load(e);
    if e.call(USE_SAVE_GAME_BLOCKS_00862110, &args![saver]).u8() != 0 {
        size = size.wrapping_add(4).wrapping_add(2);
    }
    let text = e.call(STRING_CHARS_00559450, &args![this]).u32();
    size = size.wrapping_add(1);
    let length = e.call(STRLEN_0044A670, &args![text]).u32();
    size = size.wrapping_add(length as u16);
    for step in [1u16, 1, 1, 4, 4, 4] {
        size = size.wrapping_add(step);
    }
    if reporting_sizes(e) {
        let reported = u32::from(size).wrapping_sub(u32::from(start));
        report_size(
            e,
            reported,
            0x2d9,
            SAVE_SIZE_FORM_MESSAGE,
            SAVE_SIZE_MESSAGE,
        );
    }
    size
}

// Translated from 0083f3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a `MenuTopic`): saves it the older way through the `TESSaveLoadGame`
/// at `011de45c`. With save-game blocks: the tag `"KOLB"` and a 16-bit size placeholder whose
/// place is kept. Then the text (a length byte and, when not 0, the characters), the bytes at
/// `+8`, `+0x24`, `+0x25` and the save ids (`0084e3a0`, 0 for none) of `+0x14`, `+0x28` and
/// `pInfo`. With the setting `011de4e8` set the size written is reported (line `0x2ff`); with
/// blocks a block over `0xffff` bytes is reported (`01015318`) and the size goes into its place.
pub fn fn_0083f3e0(e: &mut Engine, this: Ptr<MenuTopic>) {
    let base = this.addr();
    e.with_stack(0x10, |e, locals| {
        let size_at = locals.addr();
        let tag_at = locals.addr() + 4;
        let length_at = locals.addr() + 8;
        let id_at = locals.addr() + 0xc;
        e.mem.set_u16(size_at, 0);
        let mut place = 0u32;
        let saver = save_load(e);
        let mut start = e.call(SAVE_POSITION_00825C00, &args![saver]).u32();
        if reporting_sizes(e) {
            let saver = save_load(e);
            start = e.call(SAVE_POSITION_00825C00, &args![saver]).u32();
        }
        let saver = save_load(e);
        if e.call(USE_SAVE_GAME_BLOCKS_00862110, &args![saver]).u8() != 0 {
            e.mem.set_u32(tag_at, BLOCK_TAG);
            let saver = save_load(e);
            e.call(SAVE_WRITE_008579B0, &args![saver, tag_at, 4u32]);
            let saver = save_load(e);
            place = e.call(SAVE_POSITION_00825C00, &args![saver]).u32();
            let saver = save_load(e);
            e.call(SAVE_WRITE_008579B0, &args![saver, size_at, 2u32]);
        }
        let text = e.call(STRING_CHARS_00559450, &args![base]).u32();
        let length = e.call(STRLEN_0044A670, &args![text]).u32() as u8;
        e.mem.set_u8(length_at, length);
        let saver = save_load(e);
        e.call(SAVE_WRITE_008579B0, &args![saver, length_at, 1u32]);
        let length = e.mem.u8(length_at);
        if length != 0 {
            let saver = save_load(e);
            e.call(SAVE_WRITE_008579B0, &args![saver, text, u32::from(length)]);
        }
        for offset in [8u32, 0x24, 0x25] {
            let saver = save_load(e);
            e.call(SAVE_WRITE_008579B0, &args![saver, base + offset, 1u32]);
        }
        for offset in [0x14u32, 0x28, 0x18] {
            e.mem.set_u32(id_at, 0);
            let form = e.mem.u32(base + offset);
            if form != 0 {
                let id = e.call(FORM_SAVE_ID_0084E3A0, &args![form]).u32();
                e.mem.set_u32(id_at, id);
            }
            let saver = save_load(e);
            e.call(SAVE_NUMERIC_ID_00857A10, &args![saver, id_at, 4u32]);
        }
        if reporting_sizes(e) {
            let saver = save_load(e);
            let now = e.call(SAVE_POSITION_00825C00, &args![saver]).u32();
            report_size(
                e,
                now.wrapping_sub(start),
                0x2ff,
                SAVE_FORM_MESSAGE,
                SAVE_MESSAGE,
            );
        }
        let saver = save_load(e);
        if e.call(USE_SAVE_GAME_BLOCKS_00862110, &args![saver]).u8() != 0 {
            let saver = save_load(e);
            let now = e.call(SAVE_POSITION_00825C00, &args![saver]).u32();
            if now > place.wrapping_add(0xffff) {
                e.call(
                    REPORT_005B5E40,
                    &args![BLOCK_TOO_LARGE_MESSAGE, SOURCE_FILE_0107DD88, 0x2ffu32],
                );
            }
            e.mem.set_u16(place, now.wrapping_sub(place) as u16);
        }
    });
}

// Translated from 0083f680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083f680 - ends in the /GS cookie check (00ec408c), translated as a call
/// `MenuTopic::LoadGame` (engine map; `thiscall`, `RET 4`: one word it does not read): loads the
/// topic the older way through the `TESSaveLoadGame` at `011de45c`. With save-game blocks: the
/// tag (a wrong one is reported, line `0x305`), the block start (`00825c00`) and its 16-bit
/// size. The text (a length byte and, when not 0, the characters into a zeroed 0x104-byte stack
/// buffer, `004037f0` with 0), the bytes at `+8`, `+0x24`, `+0x25`, and the numeric ids of
/// `+0x14` (quest), `+0x28` (topic) and `pInfo`, each cast to its type. With blocks the end is
/// checked against the block's size: an overrun or underrun is reported (line `0x329`).
pub fn menu_topic_load_game(e: &mut Engine, this: Ptr<MenuTopic>, _unused_1: u32) {
    let cookie = e.global::<u32>(SECURITY_COOKIE_011C16BC);
    let base = this.addr();
    e.with_stack(0x118, |e, locals| {
        let size_at = locals.addr();
        let tag_at = locals.addr() + 4;
        let length_at = locals.addr() + 8;
        let id_at = locals.addr() + 0xc;
        let text = locals.addr() + 0x10;
        e.mem.set_u16(size_at, 0);
        let mut start = 0u32;
        let loader = save_load(e);
        if e.call(USE_SAVE_GAME_BLOCKS_00862110, &args![loader]).u8() != 0 {
            let loader = save_load(e);
            e.call(LOAD_READ_008579E0, &args![loader, tag_at, 4u32]);
            if e.mem.u32(tag_at) != BLOCK_TAG {
                report_load(e, 0x305, HEADER_FORM_MESSAGE, HEADER_MESSAGE);
            }
            let loader = save_load(e);
            start = e.call(SAVE_POSITION_00825C00, &args![loader]).u32();
            let loader = save_load(e);
            e.call(LOAD_READ_008579E0, &args![loader, size_at, 2u32]);
        }
        e.call(MEMSET_00403D30, &args![text, 0u32, 0x104u32]);
        let loader = save_load(e);
        e.call(LOAD_READ_008579E0, &args![loader, length_at, 1u32]);
        let length = e.mem.u8(length_at);
        if length != 0 {
            let loader = save_load(e);
            e.call(LOAD_READ_008579E0, &args![loader, text, u32::from(length)]);
            e.call(STRING_SET_004037F0, &args![base, text, 0u32]);
        }
        for offset in [8u32, 0x24, 0x25] {
            let loader = save_load(e);
            e.call(LOAD_READ_008579E0, &args![loader, base + offset, 1u32]);
        }
        for (offset, target) in [
            (0x14u32, QUEST_TYPE_01186500),
            (0x28, TOPIC_TYPE_01184720),
            (0x18, INFO_TYPE_01184738),
        ] {
            let loader = save_load(e);
            e.call(LOAD_NUMERIC_ID_00857AA0, &args![loader, id_at, 4u32]);
            let form = e.mem.u32(id_at);
            let cast = cast_form(e, form, target);
            e.mem.set_u32(base + offset, cast);
        }
        let loader = save_load(e);
        if e.call(USE_SAVE_GAME_BLOCKS_00862110, &args![loader]).u8() != 0 {
            let loader = save_load(e);
            let now = e.call(SAVE_POSITION_00825C00, &args![loader]).u32();
            let loader = save_load(e);
            let form = e.call(LOADING_FORM_004FD3C0, &args![loader]).u32();
            let expected = |e: &Engine| u32::from(e.mem.u16(size_at)).wrapping_add(start);
            if form != 0 {
                let id = e.mem.u32(form);
                let object = e.call(CAST_SOURCE_004839C0, &args![id]).u32();
                let (message, difference) = if now > expected(e) {
                    (OVERRUN_FORM_MESSAGE, now.wrapping_sub(expected(e)))
                } else if now < expected(e) {
                    (UNDERRUN_FORM_MESSAGE, expected(e).wrapping_sub(now))
                } else {
                    return;
                };
                let flags = e.mem.u32(form + 5);
                let version = u32::from(e.mem.u8(form + 9));
                let name = e.vcall(object, 0x130, &args![]).u32();
                let id = e.mem.u32(form);
                e.call(
                    REPORT_005B5E40,
                    &args![
                        message,
                        difference,
                        SOURCE_FILE_0107DD88,
                        0x329u32,
                        id,
                        name,
                        version,
                        flags
                    ],
                );
            } else {
                let (message, difference) = if now > expected(e) {
                    (OVERRUN_MESSAGE, now.wrapping_sub(expected(e)))
                } else if now < expected(e) {
                    (UNDERRUN_MESSAGE, expected(e).wrapping_sub(now))
                } else {
                    return;
                };
                let loader = save_load(e);
                let version = u32::from(e.call(SAVE_VERSION_008DF040, &args![loader]).u8());
                e.call(
                    REPORT_005B5E40,
                    &args![message, difference, SOURCE_FILE_0107DD88, 0x329u32, version],
                );
            }
        }
    });
    e.call(SECURITY_CHECK_COOKIE, &args![cookie]);
}

// Translated from 0083fb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MenuTopic::SaveGame` (engine map; `thiscall`, `RET 4`: the save buffer): the text
/// (`SaveString` with its characters and 0), the bytes at `+8`, `+0x24`, `+0x25`, `+9` and `+0xa`
/// (one byte each), then the form ids of `+0x14`, `+0x28`, `+0x1c` and `pInfo`.
pub fn menu_topic_save_game(e: &mut Engine, this: Ptr<MenuTopic>, buffer: u32) {
    let base = this.addr();
    let text = e.call(STRING_CHARS_00559450, &args![base]).u32();
    e.call(SAVE_STRING_00865E70, &args![buffer, text, 0u32]);
    for offset in [8u32, 0x24, 0x25, 9, 0xa] {
        e.call(
            SAVE_DATA_00865E50,
            &args![buffer, base + offset, 1u32, 0u32],
        );
    }
    for offset in [0x14u32, 0x28, 0x1c, 0x18] {
        let form = e.mem.u32(base + offset);
        e.call(SAVE_FORM_ID_00865DF0, &args![buffer, form, 0u32]);
    }
}

// Translated from 0083fbe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083fbe0 - ends in the /GS cookie check (00ec408c), translated as a call
/// `MenuTopic::LoadGame_ov2` (engine map; `thiscall`, `RET 4`: the load buffer): the text
/// (`LoadString` into a 0x200-byte stack buffer, then `00438390`), the bytes at `+8`, `+0x24`,
/// `+0x25`, `+9` and `+0xa`, then from `LoadFormID` `+0x14` (cast to a quest), `+0x28` and `+0x1c`
/// (topics) and `pInfo` (an info).
pub fn menu_topic_load_game_ov2(e: &mut Engine, this: Ptr<MenuTopic>, buffer: u32) {
    let cookie = e.global::<u32>(SECURITY_COOKIE_011C16BC);
    let base = this.addr();
    e.with_stack(0x200, |e, text| {
        e.call(LOAD_STRING_008649A0, &args![buffer, text]);
        e.call(STRING_SET_00438390, &args![base, text]);
    });
    for offset in [8u32, 0x24, 0x25, 9, 0xa] {
        e.call(LOAD_DATA_00864980, &args![buffer, base + offset, 1u32]);
    }
    for (offset, target) in [
        (0x14u32, QUEST_TYPE_01186500),
        (0x28, TOPIC_TYPE_01184720),
        (0x1c, TOPIC_TYPE_01184720),
        (0x18, INFO_TYPE_01184738),
    ] {
        let form = e.call(LOAD_FORM_ID_008648A0, &args![buffer]).u32();
        let cast = cast_form(e, form, target);
        e.mem.set_u32(base + offset, cast);
    }
    e.call(SECURITY_CHECK_COOKIE, &args![cookie]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0042_5ff0,
            fn_00425ff0(Ptr<MenuTopic>, u32) -> Ptr<MenuTopic>
        ),
        entry!(
            0x0083_e170,
            menu_topic_get_current_response(Ptr<MenuTopic>) -> u32
        ),
        entry!(
            0x0083_e1a0,
            menu_topic_manager_constructor(Ptr<MenuTopicManager>) -> Ptr<MenuTopicManager>
        ),
        entry!(
            0x0083_e1d0,
            menu_topic_manager_destructor(Ptr<MenuTopicManager>)
        ),
        entry!(
            0x0083_e230,
            menu_topic_manager_clear_data(Ptr<MenuTopicManager>)
        ),
        entry!(
            0x0083_e2d0,
            menu_topic_manager_clear_topic_list(Ptr<MenuTopicManager>, u32, u8)
        ),
        entry!(
            0x0083_e370,
            menu_topic_manager_first_topic(Ptr<MenuTopicManager>, u8) -> bool
        ),
        entry!(
            0x0083_e3e0,
            menu_topic_manager_next_topic(Ptr<MenuTopicManager>) -> bool
        ),
        entry!(0x0083_e430, fn_0083e430(Ptr<MenuTopicManager>, i32) -> bool),
        entry!(
            0x0083_e4c0,
            menu_topic_manager_get_current_topic(Ptr<MenuTopicManager>) -> u32
        ),
        entry!(
            0x0083_e4f0,
            menu_topic_manager_get_greeting(Ptr<MenuTopicManager>) -> u32
        ),
        entry!(0x0083_e510, fn_0083e510()),
        entry!(
            0x0083_e560,
            fn_0083e560(Ptr<MenuTopicManager>, u32) -> Ptr<MenuTopicManager>
        ),
        entry!(0x0083_e590, fn_0083e590() -> u32),
        entry!(
            0x0083_e620,
            menu_topic_manager_initialize(Ptr<MenuTopicManager>, u32, u32) -> bool
        ),
        entry!(
            0x0083_ebb0,
            menu_topic_manager_process_current_topic(Ptr<MenuTopicManager>)
        ),
        entry!(0x0083_ec10, fn_0083ec10(Ptr) -> bool),
        entry!(
            0x0083_ec30,
            menu_topic_manager_load_next_topic_list(Ptr<MenuTopicManager>)
        ),
        entry!(
            0x0083_ed50,
            menu_topic_manager_fill_topic_list(Ptr<MenuTopicManager>, u32, u8, u8, u32)
        ),
        entry!(0x0083_f0d0, fn_0083f0d0(Ptr) -> u32),
        entry!(0x0083_f110, fn_0083f110(u32, u32) -> i32),
        entry!(0x0083_f1a0, fn_0083f1a0(Ptr<MenuTopicManager>, u32)),
        entry!(0x0083_f260, fn_0083f260(Ptr<MenuTopicManager>) -> u32),
        entry!(
            0x0083_e850,
            menu_topic_manager_do_speech_challenge_check(Ptr<MenuTopicManager>, u32) -> u8
        ),
        entry!(
            0x0083_f1d0,
            menu_topic_manager_pop_topic_list(Ptr<MenuTopicManager>) -> u32
        ),
        entry!(0x0083_f280, fn_0083f280(Ptr<MenuTopic>) -> u16),
        entry!(0x0083_f3e0, fn_0083f3e0(Ptr<MenuTopic>)),
        entry!(0x0083_f680, menu_topic_load_game(Ptr<MenuTopic>, u32)),
        entry!(0x0083_fb10, menu_topic_save_game(Ptr<MenuTopic>, u32)),
        entry!(0x0083_fbe0, menu_topic_load_game_ov2(Ptr<MenuTopic>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn called(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// List doubles over plain `BSSimpleList` nodes (item, next): the item's address is the
    /// node, the next node is +4, a list is empty when item and next are 0, removing the
    /// head copies the next node over it, adding appends a fresh node.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next != 0 {
                (e.mem.u32(next), e.mem.u32(next + 4))
            } else {
                (0, 0)
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(LIST_ADD, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0 {
                e.mem.set_u32(a[0], item);
                return Ret::default();
            }
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let fresh = e.mem.alloc(8);
            e.mem.set_u32(fresh, item);
            e.mem.set_u32(node + 4, fresh);
            Ret::default()
        });
        e.register(LIST_CONTAINS, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == item {
                    return true.into_ret();
                }
                node = e.mem.u32(node + 4);
            }
            false.into_ret()
        });
        e.register(LIST_ADD_HEAD, |e, a| {
            // Records the added item in the head (enough for these tests).
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.register(FREE, |_, _| Ret::default());
        e.register(MENU_TOPIC_DESTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    /// A list at `head` holding `items` (extra nodes are fresh blocks).
    fn fill(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    /// A manager whose shown list (a fresh list) holds `topics`.
    fn manager(e: &mut Engine, topics: &[u32]) -> (u32, u32) {
        let this = e.mem.alloc(0x18);
        let shown = e.mem.alloc(8);
        fill(e, shown, topics);
        e.mem.set_u32(this + 4, shown);
        (this, shown)
    }

    fn topic(e: &mut Engine, general: u8) -> u32 {
        let topic = e.mem.alloc(0x2c);
        e.mem.set_u8(topic + 0x24, general);
        topic
    }

    #[test]
    fn menu_topic_deleting_destructor_frees_with_bit_0() {
        let mut e = list_engine();
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(0x0042_5ff0, &[0x1000, flags]).u32(), 0x1000);
        }
        assert_eq!(called(&e, MENU_TOPIC_DESTRUCTOR).len(), 4);
        assert_eq!(called(&e, FREE), vec![vec![0x1000], vec![0x1000]]);
    }

    #[test]
    fn menu_topic_current_response() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x2c);
        assert_eq!(e.call(0x0083_e170, &[this]).u32(), 0);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0x77);
        e.mem.set_u32(this + 0x20, node);
        assert_eq!(e.call(0x0083_e170, &[this]).u32(), 0x77);
    }

    #[test]
    fn constructor_and_destructor() {
        let mut e = list_engine();
        e.register(LIST_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(LIST_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xee; 0x18]);
        assert_eq!(e.call(0x0083_e1a0, &[this]).u32(), this);
        assert_eq!(called(&e, LIST_CONSTRUCTOR), vec![vec![this + 4]]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // The destructor clears (an empty list: nothing to do) and destroys the lists.
        e.mem.set_u32(this + 4, 0);
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(this + 0xc, 0x99);
        e.call(0x0083_e1d0, &[this]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(called(&e, LIST_DESTRUCTOR), vec![vec![this + 4]]);
    }

    #[test]
    fn clear_data_clears_each_topic_list() {
        let mut e = list_engine();
        e.register(TOPIC_LIST_CLEAR, |_, _| Ret::default());
        e.register(TOPIC_LIST_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        let first = e.mem.alloc(8);
        let a = topic(&mut e, 0);
        let c = topic(&mut e, 0);
        fill(&mut e, first, &[a, c]);
        // Two topic lists: null, then `first`.
        fill(&mut e, this + 4, &[0, first]);
        e.mem.set_u32(this + 0xc, 5);
        e.call(0x0083_e230, &[this]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(called(&e, TOPIC_LIST_CLEAR), vec![vec![0], vec![first]]);
        assert_eq!(called(&e, TOPIC_LIST_DELETE), vec![vec![first, 1]]);
        // `first` was the last list: cleared from its first node.
        assert_eq!(called(&e, FREE), vec![vec![a], vec![c]]);
    }

    #[test]
    fn clear_topic_list_keeps_general_topics() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this, 0x1234);
        let a = topic(&mut e, 0);
        let b = topic(&mut e, 1);
        let c = topic(&mut e, 0);
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[a, b, c]);
        e.call(0x0083_e2d0, &[this, list, 1]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(called(&e, FREE), vec![vec![a], vec![c]]);
        // Without the byte the first node is kept.
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[a, c]);
        e.call_log = Some(vec![]);
        e.call(0x0083_e2d0, &[this, list, 0]);
        assert_eq!(called(&e, FREE), vec![vec![c]]);
    }

    #[test]
    fn first_next_and_indexed_topics() {
        let mut e = list_engine();
        let (this, shown) = manager(&mut e, &[0x11, 0x22, 0x33]);
        assert!(e.call(0x0083_e370, &[this, 0]).bool());
        assert_eq!(e.mem.u32(this), shown);
        assert_eq!(e.call(0x0083_e4c0, &[this]).u32(), 0x11);
        assert!(e.call(0x0083_e3e0, &[this]).bool());
        assert_eq!(e.call(0x0083_e4c0, &[this]).u32(), 0x22);
        assert!(e.call(0x0083_e370, &[this, 1]).bool());
        assert_eq!(e.call(0x0083_e4c0, &[this]).u32(), 0x22);
        assert!(e.call(0x0083_e3e0, &[this]).bool());
        assert!(!e.call(0x0083_e3e0, &[this]).bool());
        assert_eq!(e.call(0x0083_e4c0, &[this]).u32(), 0);
        assert!(!e.call(0x0083_e3e0, &[this]).bool());
        // Index 1 from the second node is the third; past the end leaves the current one.
        assert!(e.call(0x0083_e430, &[this, 1]).bool());
        assert_eq!(e.call(0x0083_e4c0, &[this]).u32(), 0x33);
        e.call(0x0083_e370, &[this, 0]);
        assert!(!e.call(0x0083_e430, &[this, 5]).bool());
        assert_eq!(e.call(0x0083_e4c0, &[this]).u32(), 0x11);
        assert!(e.call(0x0083_e430, &[this, -3i32 as u32]).bool());
        assert_eq!(e.call(0x0083_e4c0, &[this]).u32(), 0x22);
        // The greeting is the first topic; the shown list is the first topic list.
        assert_eq!(e.call(0x0083_e4f0, &[this]).u32(), 0x11);
        assert_eq!(e.call(0x0083_f260, &[this]).u32(), shown);
    }

    #[test]
    fn singleton_is_made_once_and_deleted() {
        let mut e = list_engine();
        e.map(0x011d_d000, 0x1000);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LIST_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(LIST_DESTRUCTOR, |_, _| Ret::default());
        let manager = e.call(0x0083_e590, &[]).u32();
        assert_ne!(manager, 0);
        assert_eq!(e.call(0x0083_e590, &[]).u32(), manager);
        assert_eq!(called(&e, ALLOCATE), vec![vec![0x18]]);
        e.call(0x0083_e510, &[]);
        assert_eq!(e.mem.u32(MANAGER_SINGLETON), 0);
        assert_eq!(called(&e, FREE), vec![vec![manager]]);
        // Nothing to delete the second time.
        e.call(0x0083_e510, &[]);
        assert_eq!(called(&e, FREE).len(), 1);
        // A failed allocation leaves it null.
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0083_e590, &[]).u32(), 0);
    }

    #[test]
    fn manager_deleting_destructor_frees_with_bit_0() {
        let mut e = list_engine();
        e.register(LIST_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        assert_eq!(e.call(0x0083_e560, &[this, 0]).u32(), this);
        assert!(called(&e, FREE).is_empty());
        assert_eq!(e.call(0x0083_e560, &[this, 1]).u32(), this);
        assert_eq!(called(&e, FREE), vec![vec![this]]);
    }

    #[test]
    fn process_current_topic_runs_an_unflagged_info() {
        for (flags, runs) in [(0u8, true), (0x40, false), (0xbf, true), (0xff, false)] {
            let mut e = list_engine();
            e.register(WORD_18, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
            e.register(RUN_RESULT, |_, _| Ret::default());
            let info = e.mem.alloc(0x28);
            e.mem.set_u8(info + 0x25, flags);
            let menu_topic = e.mem.alloc(0x2c);
            e.mem.set_u32(menu_topic + 0x18, info);
            let (this, shown) = manager(&mut e, &[menu_topic]);
            e.mem.set_u32(this, shown);
            e.mem.set_u32(this + 0xc, 0x5c);
            e.call(0x0083_ebb0, &[this]);
            let expected = if runs {
                vec![vec![info, 0, 0x5c]]
            } else {
                vec![]
            };
            assert_eq!(called(&e, RUN_RESULT), expected, "{flags:x}");
        }
        // No current topic: nothing.
        let mut e = list_engine();
        let this = e.mem.alloc(0x18);
        e.call(0x0083_ebb0, &[this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn info_flag_and_kind() {
        let mut e = list_engine();
        let info = e.mem.alloc(0x28);
        for (byte, flagged, kind) in [
            (0u8, false, 2u32),
            (0x40, true, 2),
            (0x10, false, 1),
            (0x30, false, 1),
            (0x20, false, 3),
            (0xe0, true, 3),
        ] {
            e.mem.set_u8(info + 0x25, byte);
            assert_eq!(e.call(0x0083_ec10, &[info]).bool(), flagged, "{byte:x}");
            assert_eq!(e.call(0x0083_f0d0, &[info]).u32(), kind, "{byte:x}");
        }
    }

    #[test]
    fn compare_orders_by_value() {
        let mut e = list_engine();
        e.register(WORD_4_0045CD60, |_, a| a[0].into_ret());
        e.register(VALUE_00526AE0, |e, a| f64::from(e.mem.f32(a[0])).into_ret());
        let a = e.mem.alloc(4);
        let b = e.mem.alloc(4);
        for (x, y, expected) in [
            (1.0f32, 0.5f32, -1i32),
            (0.5, 1.0, 1),
            (1.0, 1.0, 0),
            (f32::NAN, 1.0, 0),
        ] {
            e.mem.set_f32(a, x);
            e.mem.set_f32(b, y);
            assert_eq!(e.call(0x0083_f110, &[a, b]).i32(), expected, "{x} {y}");
        }
    }

    #[test]
    fn add_topic_list_skips_null() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x18);
        e.call(0x0083_f1a0, &[this, 0]);
        assert!(called(&e, LIST_ADD_HEAD).is_empty());
        e.call(0x0083_f1a0, &[this, 0x4444]);
        assert_eq!(called(&e, LIST_ADD_HEAD)[0][0], this + 4);
        assert_eq!(e.mem.u32(this + 4), 0x4444);
    }

    /// Doubles for initialize: a new topic list, matching info, the greeting constructor.
    fn initialize_engine(info: u32) -> Engine {
        let mut e = list_engine();
        e.map(0x0107_d000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(VALUE_0107DD24, 0x4f00_0000);
        e.mem.set_u32(PLAYER, 0x9999);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LIST_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(GET_TOPIC, |_, _| 0x7070u32.into_ret());
        e.register_double(GET_MATCHING_INFO, move |_, _| info.into_ret());
        e.register(INFO_QUEST_00619D00, |_, _| 0x9000u32.into_ret());
        e.register(MENU_TOPIC_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x18, a[3]);
            a[0].into_ret()
        });
        e.register(WORD_4_0045CD60, |_, a| (a[0] + 4).into_ret());
        e.register(SET_VALUE_00965250, |_, _| Ret::default());
        e.register(MENU_TOPIC_KEEP_0083E0C0, |_, _| 1u32.into_ret());
        e.register(WORD_18, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(ADD_TOPIC_LIST, |_, _| Ret::default());
        e.register(RUN_RESULT, |_, _| Ret::default());
        e.register(TOPIC_FLAG_00619DF0, |_, _| 1u32.into_ret());
        e.register(REFERENCE_NAME_0055D520, |_, _| 0x4e4eu32.into_ret());
        e.register(REPORT_005B5E40, |_, _| Ret::default());
        e
    }

    #[test]
    fn initialize_makes_the_greeting() {
        let mut e = initialize_engine(0x1f00);
        let this = e.mem.alloc(0x18);
        e.mem.set_u8(this + 0x10, 1);
        e.mem.set_u32(this + 0x14, 7);
        assert!(e.call(0x0083_e620, &[this, 0x5c, 0]).bool());
        assert_eq!(e.mem.u32(this + 0xc), 0x5c);
        assert_eq!(e.mem.u8(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(called(&e, GET_TOPIC), vec![vec![0, 0]]);
        let matching = called(&e, GET_MATCHING_INFO);
        assert_eq!(matching[0][0], 0x7070);
        assert_eq!(matching[0][2..], [0x5c, 0x9999]);
        let made = called(&e, MENU_TOPIC_CONSTRUCTOR);
        assert_eq!(made[0][1..], [0x9000, 0x7070, 0x1f00, 0x5c, 0]);
        let greeting = made[0][0];
        assert_eq!(
            called(&e, SET_VALUE_00965250),
            vec![vec![greeting + 4, 0x4f00_0000]]
        );
        assert_eq!(called(&e, RUN_RESULT), vec![vec![0x1f00, 0, 0x5c]]);
        // The greeting is the shown list's first topic and is current.
        let shown = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(shown), greeting);
        assert_eq!(e.mem.u32(this), shown);
    }

    #[test]
    fn initialize_reports_a_missing_greeting() {
        let mut e = initialize_engine(0);
        let this = e.mem.alloc(0x18);
        assert!(!e.call(0x0083_e620, &[this, 0x5c, 0x7171]).bool());
        assert!(called(&e, GET_TOPIC).is_empty());
        assert_eq!(called(&e, GET_MATCHING_INFO)[0][0], 0x7171);
        assert_eq!(
            called(&e, REPORT_005B5E40),
            vec![vec![6, NO_GREETING_FORMAT, 0x4e4e]]
        );
        assert!(called(&e, RUN_RESULT).is_empty());
    }

    /// Doubles for loading the next list: the current topic has an info and its own list.
    fn load_engine(from_info: u32) -> Engine {
        let mut e = list_engine();
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(PLAYER, 0x9999);
        e.register(WORD_18, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(ADD_TOPIC_LIST, |_, _| Ret::default());
        e.register_double(MENU_TOPIC_FLAG_0062F2F0, move |_, _| from_info.into_ret());
        e.register(POP_TOPIC_LIST, |_, _| Ret::default());
        e.register(INFO_OBJECT_009E32D0, |_, _| 0x3200u32.into_ret());
        e.register(OBJECT_LIST_00413F40, |_, _| 0x3f40u32.into_ret());
        e.register(PLAYER_LIST_00464E30, |_, _| 0x4e30u32.into_ret());
        e.register(WORD_4_0045CD60, |_, a| (a[0] + 4).into_ret());
        e
    }

    #[test]
    fn load_next_topic_list_pops_after_a_lost_challenge() {
        let mut e = load_engine(0);
        let menu_topic = topic(&mut e, 0);
        e.mem.set_u32(menu_topic + 0x18, 0x1f00);
        let (this, shown) = manager(&mut e, &[menu_topic]);
        e.mem.set_u32(this, shown);
        e.mem.set_u8(this + 0x10, 1);
        e.call(0x0083_ec30, &[this]);
        assert_eq!(called(&e, ADD_TOPIC_LIST), vec![vec![0x1f00]]);
        assert_eq!(called(&e, POP_TOPIC_LIST), vec![vec![this]]);
        assert_eq!(e.mem.u8(this + 0x10), 0);
    }

    #[test]
    fn load_next_topic_list_fills_from_the_info_or_the_player() {
        for (from_info, list) in [(1u32, 0x3f40u32), (0, 0x4e30)] {
            let mut e = load_engine(from_info);
            // The fill sees an empty list (its first node holds nothing) and sorts.
            e.map(0x0000_3000, 0x2000);
            e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
            e.register(TOPIC_LIST_SORT, |_, _| Ret::default());
            let menu_topic = topic(&mut e, 0);
            e.mem.set_u32(menu_topic + 0x18, 0x1f00);
            let (this, shown) = manager(&mut e, &[menu_topic]);
            e.mem.set_u32(this, shown);
            e.mem.set_u8(this + 0x10, u8::from(from_info != 0));
            e.call(0x0083_ec30, &[this]);
            let casts = called(&e, DYNAMIC_CAST);
            assert_eq!(casts.len(), 1);
            assert!(called(&e, POP_TOPIC_LIST).is_empty());
            // The shown list was cleared from its second node: the topic stays.
            assert_eq!(e.mem.u32(shown), menu_topic);
            assert_eq!(e.mem.u8(this + 0x10), 0);
            assert!(called(&e, NODE_ITEM).contains(&vec![list]));
            // The answer of `0062f2f0` is the fill's second byte: set, no sort.
            let sorted = called(&e, TOPIC_LIST_SORT);
            let expected = if from_info != 0 {
                vec![]
            } else {
                vec![vec![shown, 0x0083_f110]]
            };
            assert_eq!(sorted, expected);
        }
        // A general topic loads nothing.
        let mut e = load_engine(1);
        let menu_topic = topic(&mut e, 1);
        let (this, shown) = manager(&mut e, &[menu_topic]);
        e.mem.set_u32(this, shown);
        e.call(0x0083_ec30, &[this]);
        assert!(called(&e, OBJECT_LIST_00413F40).is_empty());
    }

    /// Doubles for filling: topics whose `0083de60` byte is +0x10, kind byte +0x25,
    /// `00619df0` byte +0x11; matching infos at topic +0x14; the player's value 9 is 5 and
    /// the limit 10.
    fn fill_engine(cast_byte: u32) -> Engine {
        let mut e = list_engine();
        e.map(0x011d_0000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        let player = e.mem.alloc(0xa8);
        let inner = e.mem.alloc(0x10);
        e.mem.set_u32(inner + 8, 0x6008);
        e.register(0x6008, |_, _| 5u32.into_ret());
        e.mem.set_u32(player + 0xa4, inner);
        e.mem.set_u32(PLAYER, player);
        let limit = e.mem.alloc(4);
        e.mem.set_u32(limit, 10);
        e.register_double(SETTING_0043D4D0, move |_, _| limit.into_ret());
        e.register(DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        e.register_double(SPEAKER_FLAG_0087F510, move |_, _| cast_byte.into_ret());
        e.register(TOPIC_TEST_0083DE60, |e, a| e.mem.u8(a[0] + 0x10).into_ret());
        e.register(TOPIC_FLAG_00619DF0, |e, a| e.mem.u8(a[0] + 0x11).into_ret());
        e.register(GET_MATCHING_INFO_0061A790, |e, a| {
            e.mem.set_u8(a[1], 1);
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(INFO_QUEST_00619D00, |_, _| 0x9000u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MENU_TOPIC_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x18, a[3]);
            a[0].into_ret()
        });
        e.register(MENU_TOPIC_KEEP_0083E0C0, |_, _| 1u32.into_ret());
        e.register(EXTRA_LIST_005D43C0, |_, a| (a[0] + 0x40).into_ret());
        e.register(GENERAL_TOPIC_0042DF90, |_, _| 0x6e6eu32.into_ret());
        e.register(WORD_18, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(INFO_FLAG_00619B20, |_, _| 0u32.into_ret());
        e.register(SET_INFO_GENERAL_TOPIC, |_, _| Ret::default());
        e.register(TOPIC_LIST_SORT, |_, _| Ret::default());
        e
    }

    /// A fill topic: test byte, kind byte, `00619df0` byte, matching info.
    fn fill_topic(e: &mut Engine, test: u8, kind: u8, special: u8, info: u32) -> u32 {
        let topic = e.mem.alloc(0x28);
        e.mem.set_u8(topic + 0x10, test);
        e.mem.set_u8(topic + 0x25, kind);
        e.mem.set_u8(topic + 0x11, special);
        e.mem.set_u32(topic + 0x14, info);
        topic
    }

    #[test]
    fn fill_makes_menu_topics_for_matching_topics() {
        let mut e = fill_engine(0);
        let speaker_vtable = e.mem.alloc(0x4c);
        e.mem.set_u32(speaker_vtable + 0x48, 0x6048);
        e.register(0x6048, |_, _| Ret::default());
        let speaker = e.mem.alloc(8);
        e.mem.set_u32(speaker, speaker_vtable);
        let (this, shown) = manager(&mut e, &[]);
        e.mem.set_u32(this + 0xc, speaker);
        let plain = fill_topic(&mut e, 0, 0, 0, 0x1001);
        let wrong_test = fill_topic(&mut e, 1, 0, 0, 0x1002);
        let kind3 = fill_topic(&mut e, 0, 0x20, 0, 0x1003);
        let no_info = fill_topic(&mut e, 0, 0, 0, 0);
        let special = fill_topic(&mut e, 0, 0x10, 1, 0x1005);
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[plain, wrong_test, kind3, no_info, special]);
        e.call(0x0083_ed50, &[this, list, 0, 0, 0x77]);
        let made = called(&e, MENU_TOPIC_CONSTRUCTOR);
        let topics: Vec<u32> = made.iter().map(|w| w[2]).collect();
        // The kind-3 topic is skipped (value 5 not above 10); the special one is made after
        // the general topic was added, so it is not made either.
        assert_eq!(topics, vec![plain]);
        assert_eq!(made[0][1..], [0x9000, plain, 0x1001, speaker, 1]);
        let matching = called(&e, GET_MATCHING_INFO_0061A790);
        assert_eq!(matching[0][2..], [speaker, e.mem.u32(PLAYER), 0, 0x77, 0]);
        // Shown list: the plain topic's menu topic, then the general topic.
        assert_eq!(e.mem.u32(shown), made[0][0]);
        let second = e.mem.u32(shown + 4);
        assert_eq!(e.mem.u32(second), 0x6e6e);
        assert_eq!(called(&e, TOPIC_LIST_SORT), vec![vec![shown, 0x0083_f110]]);
    }

    #[test]
    fn fill_makes_the_general_topic_when_the_speaker_answers_true() {
        // The cast speaker answering true: no general topic is added and the special topic
        // is not made.
        let mut e = fill_engine(1);
        let (this, shown) = manager(&mut e, &[]);
        e.mem.set_u32(this + 0xc, 0x5c00);
        let special = fill_topic(&mut e, 0, 0, 1, 0x1005);
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[special]);
        e.call(0x0083_ed50, &[this, list, 0, 1, 0]);
        assert!(called(&e, MENU_TOPIC_CONSTRUCTOR).is_empty());
        assert!(called(&e, GENERAL_TOPIC_0042DF90).is_empty());
        assert_eq!(e.mem.u32(shown), 0);
        // The second byte set: no sort.
        assert!(called(&e, TOPIC_LIST_SORT).is_empty());
    }

    #[test]
    fn fill_sets_the_general_topic_from_a_special_info() {
        // No general topic exists (0042df90 answers 0): the special topic is made and
        // becomes the speaker's general topic.
        let mut e = fill_engine(0);
        e.register(GENERAL_TOPIC_0042DF90, |_, _| 0u32.into_ret());
        let speaker_vtable = e.mem.alloc(0x4c);
        e.mem.set_u32(speaker_vtable + 0x48, 0x6048);
        e.register(0x6048, |_, _| Ret::default());
        let speaker = e.mem.alloc(8);
        e.mem.set_u32(speaker, speaker_vtable);
        let (this, _) = manager(&mut e, &[]);
        e.mem.set_u32(this + 0xc, speaker);
        let special = fill_topic(&mut e, 1, 0, 1, 0x1005);
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[special]);
        e.call(0x0083_ed50, &[this, list, 1, 0, 0]);
        let made = called(&e, MENU_TOPIC_CONSTRUCTOR);
        assert_eq!(made.len(), 1);
        assert_eq!(
            called(&e, SET_INFO_GENERAL_TOPIC),
            vec![vec![speaker + 0x40, made[0][0]]]
        );
        assert_eq!(called(&e, 0x6048), vec![vec![speaker, 0x8000_0000]]);
    }

    #[test]
    fn fill_deletes_topics_not_kept() {
        let mut e = fill_engine(0);
        e.register(MENU_TOPIC_KEEP_0083E0C0, |_, _| 0u32.into_ret());
        let (this, shown) = manager(&mut e, &[]);
        e.mem.set_u32(this + 0xc, 0x5c00);
        let plain = fill_topic(&mut e, 0, 0, 0, 0x1001);
        let list = e.mem.alloc(8);
        fill(&mut e, list, &[plain]);
        e.call(0x0083_ed50, &[this, list, 0, 1, 0]);
        let made = called(&e, MENU_TOPIC_CONSTRUCTOR);
        assert_eq!(called(&e, FREE), vec![vec![made[0][0]]]);
        assert_eq!(e.mem.u32(shown), 0);
    }
}

#[cfg(test)]
mod n089_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// List doubles: a node keeps its item at `+0` and its next node at `+4`; a node with
    /// neither is empty; adding appends a fresh node.
    fn list_doubles(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_ADD, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0 {
                e.mem.set_u32(a[0], item);
                return Ret::default();
            }
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let fresh = e.mem.alloc(8);
            e.mem.set_u32(fresh, item);
            e.mem.set_u32(node + 4, fresh);
            Ret::default()
        });
    }

    /// A speaker object whose vtable slots `0x100`, `0x344`, `0xfc` and `0x460` are the doubles
    /// at `0x0338_0000 + slot`.
    fn speaker(e: &mut Engine, actor: bool) -> u32 {
        let mut slots = vec![0u32; 0x119];
        for slot in [
            SPEAKER_IS_ACTOR_SLOT,
            SPEAKER_VALUE_SLOT,
            SPEAKER_FLAG_SLOT,
            SPEAKER_SET_SLOT,
        ] {
            slots[(slot / 4) as usize] = 0x0338_0000 + slot;
        }
        e.put_vtable(0x0339_0000, &slots);
        if actor {
            e.register(0x0338_0000 + SPEAKER_IS_ACTOR_SLOT, |_, _| 1u32.into_ret());
        } else {
            e.register(0x0338_0000 + SPEAKER_IS_ACTOR_SLOT, |_, _| 0u32.into_ret());
        }
        e.register(0x0338_0000 + SPEAKER_VALUE_SLOT, |_, _| 50u32.into_ret());
        e.register(0x0338_0000 + SPEAKER_FLAG_SLOT, |_, _| 1u32.into_ret());
        e.register(0x0338_0000 + SPEAKER_SET_SLOT, quiet);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0339_0000);
        object
    }

    /// The doubles of the challenge roll: a chance of 40 and the random number given.
    fn challenge_doubles(e: &mut Engine, random: u32) {
        e.map(PLAYER, 4);
        e.set_global(PLAYER, 0x0070_0000u32);
        e.register(WORD_18, |_, a| (a[0] + 0x18).into_ret());
        e.register(INFO_IS_CHALLENGE_007627C0, |_, _| 1u32.into_ret());
        e.register(INFO_CHALLENGE_0061F2B0, |_, _| 0xc4a1u32.into_ret());
        e.register(CLAMPED_ACTOR_VALUE_0066EF50, |_, _| 35.5f32.into_ret());
        e.register(CHALLENGE_CHANCE_00648EA0, |_, _| 40u32.into_ret());
        e.register_double(RANDOM_00487F50, move |_, _| random.into_ret());
        e.register(INFO_LEVEL_008041A0, |_, _| 3u32.into_ret());
        e.register(EXPERIENCE_POINTS_006705B0, |_, a| {
            (a[0] * 100 + a[1]).into_ret()
        });
        e.register(CHALLENGE_VALUE_00648F60, |_, a| {
            (a[0] as f32 / 2.0).into_ret()
        });
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 0x7f80_0001);
        e.register_double(SETTING_ADDRESS_00403E20, move |_, _| setting.into_ret());
    }

    #[test]
    fn n089_0083e850_wins_or_loses_the_speech_challenge() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        // 1039 % 100 = 39 < 40: won.
        challenge_doubles(&mut e, 1039);
        let this = e.mem.alloc(0x18);
        let talker = speaker(&mut e, true);
        e.mem.set_u32(this + 0xc, talker);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_e850, &args![this, 0x4000u32]).u8(), 0);
        assert_eq!(e.mem.u32(this + 0x14), 400 + 2);
        assert_eq!(
            calls_to(&e, 0x0338_0000 + SPEAKER_VALUE_SLOT),
            vec![vec![talker, 0x0070_0000, 0]]
        );
        assert_eq!(
            calls_to(&e, CHALLENGE_CHANCE_00648EA0),
            vec![vec![35.5f32.to_bits(), 50.0f32.to_bits(), 0xc4a1]]
        );
        assert_eq!(
            calls_to(&e, CLAMPED_ACTOR_VALUE_0066EF50),
            vec![vec![0x0070_00a4, 0x2b]]
        );
        assert!(calls_to(&e, 0x0338_0000 + SPEAKER_SET_SLOT).is_empty());
        // No topic, or not a challenge: nothing.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_e850, &args![this, 0u32]).u8(), 0);
        e.register(INFO_IS_CHALLENGE_007627C0, |_, _| 0x100u32.into_ret());
        assert_eq!(e.call(0x0083_e850, &args![this, 0x4000u32]).u8(), 0);
        assert!(calls_to(&e, RANDOM_00487F50).is_empty());
    }

    #[test]
    fn n089_0083e850_shows_the_failure_topic() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        // 40 % 100 = 40, not below 40: lost.
        challenge_doubles(&mut e, 40);
        for address in [
            MISC_STAT_INCREMENT_004D5C60,
            MENU_TOPIC_KEEP_0083E0C0,
            INFO_0061F220,
            SPEAKER_INFO_00935920,
            LIST_REMOVE_00905330,
            REPORT_005B5E40,
            LIST_ADD_HEAD,
        ] {
            e.register(address, quiet);
        }
        e.register(GET_TOPIC, |_, _| 0x7000u32.into_ret());
        e.register(WORD_4_0045CD60, |_, a| (a[0] + 4).into_ret());
        e.register(GET_MATCHING_INFO_0061A790, |_, _| 0x8000u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LIST_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(INFO_QUEST_00619D00, |_, a| (a[1] + 1).into_ret());
        e.register(MENU_TOPIC_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(INFO_FLAG_00619B20, |_, _| 0u32.into_ret());
        e.register(INFO_FLAG_0061AEF0, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x18);
        let talker = speaker(&mut e, true);
        e.mem.set_u32(this + 0xc, talker);
        let shown = e.mem.alloc(8);
        e.mem.set_u32(shown, 0x6000);
        e.mem.set_u32(this + 4, shown);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_e850, &args![this, 0x4000u32]).u8(), 1);
        assert_eq!(e.mem.u8(this + 0x10), 1);
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT_004D5C60), vec![vec![0x1f]]);
        assert_eq!(calls_to(&e, GET_TOPIC), vec![vec![0, 0xb]]);
        let matching = calls_to(&e, GET_MATCHING_INFO_0061A790);
        assert_eq!(matching[0][0], 0x7000);
        assert_eq!(matching[0][2..], [talker, 0x0070_0000, 0, 0x4004, 0]);
        let made = calls_to(&e, MENU_TOPIC_CONSTRUCTOR);
        assert_eq!(made[0][1..], [0x8001, 0x7000, 0x8000, talker, 0]);
        assert_eq!(
            calls_to(&e, SPEAKER_INFO_00935920),
            vec![vec![talker, 0x4018]]
        );
        let removed = calls_to(&e, LIST_REMOVE_00905330);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], shown);
        // The new list holds the greeting, then the failure topic, and is shown.
        let list = calls_to(&e, LIST_CONSTRUCTOR)[0][0];
        assert_eq!(e.mem.u32(list), 0x6000);
        let second = e.mem.u32(list + 4);
        assert_eq!(e.mem.u32(second), made[0][0]);
        assert_eq!(calls_to(&e, LIST_ADD_HEAD)[0][0], this + 4);
        // The actor gets the challenge's value: 00648f60(40) = 20.0.
        assert_eq!(
            calls_to(&e, 0x0338_0000 + SPEAKER_SET_SLOT),
            vec![vec![talker, 0x0070_0000, 20.0f32.to_bits()]]
        );
        // No matching info: reported, nothing shown; a non-actor speaker uses the setting
        // (its signalling NaN quieted) and gets nothing at the end.
        e.register(GET_MATCHING_INFO_0061A790, |_, _| 0u32.into_ret());
        let talker = speaker(&mut e, false);
        e.mem.set_u32(this + 0xc, talker);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_e850, &args![this, 0x4000u32]).u8(), 1);
        assert_eq!(
            calls_to(&e, REPORT_005B5E40),
            vec![vec![6, NO_FAILURE_INFO_MESSAGE]]
        );
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert_eq!(calls_to(&e, CHALLENGE_CHANCE_00648EA0)[0][1], 0x7fc0_0001);
        assert!(calls_to(&e, 0x0338_0000 + SPEAKER_SET_SLOT).is_empty());
    }

    #[test]
    fn n089_0083f1d0_pops_the_shown_list() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        for address in [TOPIC_LIST_CLEAR, TOPIC_LIST_DELETE] {
            e.register(address, quiet);
        }
        // Removing the head moves the second node's item into it.
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let item = e.mem.u32(next);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        let this = e.mem.alloc(0x18);
        let shown = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(this + 4, shown);
        e.mem.set_u32(this + 8, second);
        e.mem.set_u32(second, 0x5500);
        e.mem.set_u32(this, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_f1d0, &args![this]).u32(), 0x5500);
        assert_eq!(e.mem.u32(this), 0, "the current topic is cleared");
        assert_eq!(calls_to(&e, TOPIC_LIST_CLEAR), vec![vec![shown]]);
        assert_eq!(calls_to(&e, TOPIC_LIST_DELETE), vec![vec![shown, 1]]);
        // A single list stays.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_f1d0, &args![this]).u32(), 0x5500);
        assert!(calls_to(&e, LIST_REMOVE_HEAD).is_empty());
    }

    fn report_doubles(e: &mut Engine, report: bool, form: u32) {
        e.map(SAVE_LOAD_GAME_011DE45C, 4);
        e.set_global(SAVE_LOAD_GAME_011DE45C, 0x5a5a_0000u32);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, u8::from(report));
        e.register_double(SETTING_VALUE_00408D60, move |_, _| flag.into_ret());
        e.register_double(SAVING_FORM_004FD3E0, move |_, _| form.into_ret());
        e.register_double(LOADING_FORM_004FD3C0, move |_, _| form.into_ret());
        e.register(ERROR_0040FBE0, quiet);
        e.register(REPORT_005B5E40, quiet);
        e.register(CAST_SOURCE_004839C0, |_, a| (a[0] + 0x100).into_ret());
        e.put_vtable(0x033a_0000, &[0; 0x4d]);
        e.mem.set_u32(0x033a_0000 + 0x130, 0x033a_1130);
        e.register(0x033a_1130, |_, _| 0x4e4du32.into_ret());
        e.register(SAVE_VERSION_008DF040, |_, _| 0x0f0eu32.into_ret());
    }

    /// A form being saved or loaded: id `0x00ab_0001`, flags at `+5`, version byte at `+9`, its
    /// cast object (`id + 0x100`) with the name slot.
    fn current_form(e: &mut Engine) -> u32 {
        let form = e.mem.alloc(0x10);
        e.mem.set_u32(form, 0x00ab_0001);
        e.mem.set_u32(form + 5, 0xf1a6_0000);
        e.mem.set_u8(form + 9, 0x0f);
        e.map(0x00ab_0101, 4);
        e.mem.set_u32(0x00ab_0101, 0x033a_0000);
        form
    }

    #[test]
    fn n089_0083f280_sizes_the_topic() {
        let mut e = Engine::new();
        let form = current_form(&mut e);
        report_doubles(&mut e, true, form);
        e.register(USE_SAVE_GAME_BLOCKS_00862110, |_, _| 1u32.into_ret());
        e.register(STRING_CHARS_00559450, |_, _| 0x7e47u32.into_ret());
        e.register(STRLEN_0044A670, |_, _| 5u32.into_ret());
        let this = e.mem.alloc(0x2c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_f280, &args![this]).u16(), 6 + 1 + 5 + 3 + 12);
        assert_eq!(calls_to(&e, STRLEN_0044A670), vec![vec![0x7e47]]);
        assert_eq!(
            calls_to(&e, ERROR_0040FBE0),
            vec![vec![
                SAVE_SIZE_FORM_MESSAGE,
                27,
                0x00ab_0001,
                0x4e4d,
                0xf1a6_0000,
                0x2d9,
                SOURCE_FILE_0107DD88
            ]]
        );
        // No blocks, no form: the short report.
        report_doubles(&mut e, true, 0);
        e.register(USE_SAVE_GAME_BLOCKS_00862110, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_f280, &args![this]).u16(), 21);
        assert_eq!(
            calls_to(&e, ERROR_0040FBE0),
            vec![vec![SAVE_SIZE_MESSAGE, 21, 0x2d9, SOURCE_FILE_0107DD88]]
        );
        // Not reporting.
        report_doubles(&mut e, false, 0);
        e.call_log = Some(vec![]);
        e.call(0x0083_f280, &args![this]);
        assert!(calls_to(&e, ERROR_0040FBE0).is_empty());
    }

    #[test]
    fn n089_0083f3e0_saves_the_topic_the_old_way() {
        let mut e = Engine::new();
        report_doubles(&mut e, true, 0);
        e.register(USE_SAVE_GAME_BLOCKS_00862110, |_, _| 1u32.into_ret());
        // The position advances by each write's size; the block size place is a real word.
        let position = e.mem.alloc(0x20);
        e.mem.set_u32(position, position + 8);
        e.register_double(SAVE_POSITION_00825C00, move |e, _| {
            e.mem.u32(position).into_ret()
        });
        e.register_double(SAVE_WRITE_008579B0, move |e, a| {
            let now = e.mem.u32(position);
            e.mem.set_u32(position, now + a[2]);
            Ret::default()
        });
        e.register_double(SAVE_NUMERIC_ID_00857A10, move |e, a| {
            let now = e.mem.u32(position);
            e.mem.set_u32(position, now + a[2]);
            Ret::default()
        });
        e.register(STRING_CHARS_00559450, |_, _| 0x7e47u32.into_ret());
        e.register(STRLEN_0044A670, |_, _| 0x102u32.into_ret());
        e.register(FORM_SAVE_ID_0084E3A0, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 0x28, 0x2800);
        e.call_log = Some(vec![]);
        e.call(0x0083_f3e0, &args![this]);
        let writes = calls_to(&e, SAVE_WRITE_008579B0);
        assert_eq!(
            writes.iter().map(|w| w[2]).collect::<Vec<_>>(),
            vec![4, 2, 1, 2, 1, 1, 1],
            "tag, size, length (0x102 as a byte: 2), text, three bytes"
        );
        assert_eq!(writes[3][1], 0x7e47);
        assert_eq!(writes[4][1], this + 8);
        assert_eq!(calls_to(&e, FORM_SAVE_ID_0084E3A0), vec![vec![0x2800]]);
        // The block's size (from after the tag) goes into its place.
        let place = position + 8 + 4;
        assert_eq!(e.mem.u16(place), 2 + 1 + 2 + 3 + 12);
        assert_eq!(
            calls_to(&e, ERROR_0040FBE0),
            vec![vec![SAVE_MESSAGE, 24, 0x2ff, SOURCE_FILE_0107DD88]]
        );
        assert!(calls_to(&e, REPORT_005B5E40).is_empty());
    }

    #[test]
    fn n089_0083f680_loads_the_topic_the_old_way() {
        let mut e = Engine::new();
        let form = current_form(&mut e);
        report_doubles(&mut e, false, form);
        e.map(SECURITY_COOKIE_011C16BC, 4);
        e.register(USE_SAVE_GAME_BLOCKS_00862110, |_, _| 1u32.into_ret());
        e.register(SECURITY_CHECK_COOKIE, quiet);
        e.register(MEMSET_00403D30, quiet);
        e.register(STRING_SET_004037F0, quiet);
        e.register(DYNAMIC_CAST, |_, a| (a[0] + a[3]).into_ret());
        // Reads: a wrong tag, the size 9, the length 2 and its text, three bytes, three ids.
        let stream = e.mem.alloc(0x40);
        for (k, word) in [0x1234u32, 9, 2, 0x6968, 1, 2, 3, 0x11, 0x22, 0x33]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(stream + 4 + 4 * k as u32, *word);
        }
        let reader = move |e: &mut Engine, a: &[u32]| {
            let cursor = e.mem.u32(stream);
            let word = e.mem.u32(stream + 4 + cursor * 4);
            e.mem.set_u32(stream, cursor + 1);
            match a[2] {
                1 => e.mem.set_u8(a[1], word as u8),
                2 => e.mem.set_u16(a[1], word as u16),
                _ => e.mem.set_u32(a[1], word),
            }
            Ret::default()
        };
        e.register_double(LOAD_READ_008579E0, reader);
        e.register_double(LOAD_NUMERIC_ID_00857AA0, reader);
        // The position: the block starts at 100 and ends at 112 (3 past the size).
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, 100);
        e.register_double(SAVE_POSITION_00825C00, move |e, _| {
            let now = e.mem.u32(position);
            e.mem.set_u32(position, 112);
            now.into_ret()
        });
        let this = e.mem.alloc(0x2c);
        e.call_log = Some(vec![]);
        e.call(0x0083_f680, &args![this, 0u32]);
        let reports = calls_to(&e, REPORT_005B5E40);
        assert_eq!(reports.len(), 2);
        assert_eq!(
            reports[0],
            vec![
                HEADER_FORM_MESSAGE,
                SOURCE_FILE_0107DD88,
                0x305,
                0x00ab_0001,
                0x4e4d,
                0x0f,
                0xf1a6_0000
            ]
        );
        assert_eq!(
            reports[1],
            vec![
                OVERRUN_FORM_MESSAGE,
                3,
                SOURCE_FILE_0107DD88,
                0x329,
                0x00ab_0001,
                0x4e4d,
                0x0f,
                0xf1a6_0000
            ]
        );
        let text = calls_to(&e, STRING_SET_004037F0);
        assert_eq!(text.len(), 1);
        assert_eq!(e.mem.cstr(text[0][1]), b"hi".to_vec());
        assert_eq!(e.mem.u8(this + 8), 1);
        assert_eq!(e.mem.u8(this + 0x25), 3);
        assert_eq!(e.mem.u32(this + 0x14), 0x111 + QUEST_TYPE_01186500);
        assert_eq!(e.mem.u32(this + 0x28), 0x122 + TOPIC_TYPE_01184720);
        assert_eq!(e.mem.u32(this + 0x18), 0x133 + INFO_TYPE_01184738);
        assert_eq!(calls_to(&e, SECURITY_CHECK_COOKIE).len(), 1);
        // No form, the right tag, an underrun (the block ends 2 bytes short).
        report_doubles(&mut e, false, 0);
        e.mem.set_u32(stream, 0);
        e.mem.set_u32(stream + 4, BLOCK_TAG);
        e.mem.set_u32(stream + 8, 14);
        e.mem.set_u32(position, 100);
        e.call_log = Some(vec![]);
        e.call(0x0083_f680, &args![this, 0u32]);
        assert_eq!(
            calls_to(&e, REPORT_005B5E40),
            vec![vec![UNDERRUN_MESSAGE, 2, SOURCE_FILE_0107DD88, 0x329, 0x0e]]
        );
    }

    #[test]
    fn n089_0083fb10_saves_the_topic() {
        let mut e = Engine::new();
        for address in [
            SAVE_STRING_00865E70,
            SAVE_DATA_00865E50,
            SAVE_FORM_ID_00865DF0,
        ] {
            e.register(address, quiet);
        }
        e.register(STRING_CHARS_00559450, |_, _| 0x7e47u32.into_ret());
        let this = e.mem.alloc(0x2c);
        for (offset, form) in [(0x14u32, 0x14u32), (0x18, 0x18), (0x1c, 0x1c), (0x28, 0x28)] {
            e.mem.set_u32(this + offset, form);
        }
        e.call_log = Some(vec![]);
        e.call(0x0083_fb10, &args![this, 0xb0ffu32]);
        assert_eq!(
            calls_to(&e, SAVE_STRING_00865E70),
            vec![vec![0xb0ff, 0x7e47, 0]]
        );
        assert_eq!(
            calls_to(&e, SAVE_DATA_00865E50)
                .iter()
                .map(|c| c[1] - this)
                .collect::<Vec<_>>(),
            vec![8, 0x24, 0x25, 9, 0xa]
        );
        assert_eq!(
            calls_to(&e, SAVE_FORM_ID_00865DF0),
            vec![
                vec![0xb0ff, 0x14, 0],
                vec![0xb0ff, 0x28, 0],
                vec![0xb0ff, 0x1c, 0],
                vec![0xb0ff, 0x18, 0]
            ]
        );
    }

    #[test]
    fn n089_0083fbe0_loads_the_topic() {
        let mut e = Engine::new();
        e.map(SECURITY_COOKIE_011C16BC, 4);
        for address in [
            LOAD_STRING_008649A0,
            STRING_SET_00438390,
            LOAD_DATA_00864980,
            SECURITY_CHECK_COOKIE,
        ] {
            e.register(address, quiet);
        }
        e.register(LOAD_FORM_ID_008648A0, |_, _| 0x900u32.into_ret());
        e.register(CAST_SOURCE_004839C0, |_, a| (a[0] + 0x10).into_ret());
        e.register(DYNAMIC_CAST, |_, a| (a[0] + a[3]).into_ret());
        let this = e.mem.alloc(0x2c);
        e.call_log = Some(vec![]);
        e.call(0x0083_fbe0, &args![this, 0xb0ffu32]);
        let text = calls_to(&e, LOAD_STRING_008649A0)[0][1];
        assert_eq!(calls_to(&e, STRING_SET_00438390), vec![vec![this, text]]);
        assert_eq!(
            calls_to(&e, LOAD_DATA_00864980)
                .iter()
                .map(|c| (c[1] - this, c[2]))
                .collect::<Vec<_>>(),
            vec![(8, 1), (0x24, 1), (0x25, 1), (9, 1), (0xa, 1)]
        );
        assert_eq!(e.mem.u32(this + 0x14), 0x910 + QUEST_TYPE_01186500);
        assert_eq!(e.mem.u32(this + 0x28), 0x910 + TOPIC_TYPE_01184720);
        assert_eq!(e.mem.u32(this + 0x1c), 0x910 + TOPIC_TYPE_01184720);
        assert_eq!(e.mem.u32(this + 0x18), 0x910 + INFO_TYPE_01184738);
        assert_eq!(calls_to(&e, SECURITY_CHECK_COOKIE).len(), 1);
    }
}

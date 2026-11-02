//! `(unplaced)/run_0083d8b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0083e0a0` calls on the member (`004048e0`).
const CALLEE_004048E0: u32 = 0x0040_48e0;

// Translated from 0083e0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `004048e0` with `ECX` = `this + 0x18` and passes on its result.
pub fn fn_0083e0a0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_004048E0, &args![this.byte_add(0x18)]).u32()
}

layout! {
    /// The object `0083db00` constructs, 0x2C bytes: its fields are those of
    /// the Xbox PDB's `MenuTopic` at the same offsets (the class has no
    /// vtable to confirm the name by RTTI).
    pub struct MenuTopic: 0x2C {
        /// `cTopicText` (Xbox PDB): a `BSStringT<char>`.
        0x00 cTopicText: Inline<crate::types::BSStringT>,
        /// `bTopicIsChoice` (Xbox PDB).
        0x08 bTopicIsChoice: u8,
        /// `bInfoRefusal` (Xbox PDB).
        0x09 bInfoRefusal: u8,
        /// `bPersuasion` (Xbox PDB).
        0x0A bPersuasion: u8,
        /// `lResponses` (Xbox PDB): `BSSimpleList<DialogueResponse *>`.
        0x0C lResponses: Inline<crate::types::BSSimpleList>,
        /// `pQuest` (Xbox PDB).
        0x14 pQuest: u32,
        /// `pInfo` (Xbox PDB).
        0x18 pInfo: u32,
        /// `pOrigTopic` (Xbox PDB).
        0x1C pOrigTopic: u32,
        /// `pCurrentResponse` (Xbox PDB).
        0x20 pCurrentResponse: u32,
        /// `bGeneralTopic` (Xbox PDB).
        0x24 bGeneralTopic: u8,
        /// `bTopicIsNew` (Xbox PDB).
        0x25 bTopicIsNew: u8,
        /// `pTopic` (Xbox PDB).
        0x28 pTopic: u32,
    }
}

/// `BSStringT<char>` constructor (`004037b0`, `thiscall`).
const BS_STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `BSStringT<char>` destructor (`004037d0`, `thiscall`).
const BS_STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `004037f0`: sets a string from a C string (`thiscall`, two words:
/// text, 0).
const BS_STRING_SET_004037F0: u32 = 0x0040_37f0;
/// `BSStringT<char>::Set` (engine map; `00438470`, `thiscall`, one word).
const BS_STRING_SET: u32 = 0x0043_8470;
/// `00559450`: the word at `this` (a string's text, a list's head).
const FIRST_WORD: u32 = 0x0055_9450;
/// `004039c0`: stores a string's length (`thiscall`, one word; clamped to
/// `0xffff`).
const BS_STRING_SET_LENGTH: u32 = 0x0040_39c0;
/// `004047f0`: assigns one string to another (`thiscall`, one word).
const BS_STRING_ASSIGN: u32 = 0x0040_47f0;
/// `0096a2d0`: the response list's constructor (`thiscall`).
const RESPONSE_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `0046ffb0`: the response list's destructor (`thiscall`).
const RESPONSE_LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `008256d0`: a list's emptiness check (`thiscall`, `bool`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0`: the address of a list's first item (`thiscall`).
const LIST_FIRST_ITEM: u32 = 0x0068_15c0;
/// `0063f7b0`: removes a list's first node (`thiscall`).
const LIST_POP_FRONT: u32 = 0x0063_f7b0;
/// `0083c720`: a response's scalar deleting destructor (`thiscall`, flags).
const RESPONSE_DELETING_DESTRUCTOR: u32 = 0x0083_c720;
/// `00619df0`: a `bool` of the topic (`thiscall`).
const TOPIC_CHECK_00619DF0: u32 = 0x0061_9df0;
/// `00619b40`: a `bool` of the info (`thiscall`; the engine map's name for
/// the folded body is not this one's).
const INFO_CHECK_00619B40: u32 = 0x0061_9b40;
/// `0061f260`: a `bool` of the info (`thiscall`).
const INFO_CHECK_0061F260: u32 = 0x0061_f260;
/// `TESTopic::GetTopic` (engine map), `cdecl`, two words.
const TES_TOPIC_GET_TOPIC: u32 = 0x0061_a2d0;
/// `TESTopic::GetMatchingInfo` (engine map), `thiscall`, two words.
const TES_TOPIC_GET_MATCHING_INFO: u32 = 0x0061_a720;
/// `00619d00`: `thiscall` on a topic, one word (an info); a quest.
const TOPIC_QUEST_FOR_INFO: u32 = 0x0061_9d00;
/// The player (`PlayerCharacter *`) global.
const PLAYER: u32 = 0x011d_ea3c;
/// The object (a setting) `0043d4d0` gives the address of the integer of.
const SETTING_011D0DC8: u32 = 0x011d_0dc8;
/// `0043d4d0`: `thiscall`, the address of the setting's value.
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
/// `0083dea0`: whether the topic's string at `+0x34` is non-empty.
const TOPIC_HAS_NAME_0083DEA0: u32 = 0x0083_dea0;
/// `006a9540`: the topic's text (`thiscall`).
const TOPIC_NAME_006A9540: u32 = 0x006a_9540;
/// `004610d0`: the info's text (`thiscall`).
const INFO_NAME_004610D0: u32 = 0x0046_10d0;
/// `00408da0`: a text of the topic's `+0x18` component (`thiscall`; the
/// engine map's name, `MapMarkerData::GetLocationName`, is a folded body).
const COMPONENT_NAME_00408DA0: u32 = 0x0040_8da0;
/// `009e32d0`: `thiscall` on the info, an object or 0.
const INFO_OBJECT_009E32D0: u32 = 0x009e_32d0;
/// `00413f40`: `thiscall`, a list.
const OBJECT_LIST_00413F40: u32 = 0x0041_3f40;
/// `006733e0`: the info's quest (`thiscall`; the engine map's name is a
/// folded body's).
const INFO_QUEST_006733E0: u32 = 0x0067_33e0;
/// `TESTopic::GetOwnerTopic` (engine map), `cdecl`, one word (the info).
const TES_TOPIC_GET_OWNER_TOPIC: u32 = 0x0061_a310;
/// `0084e3a0`: `thiscall`, no arguments, a word (compared with form ids
/// `0xfd` and `0x118`).
const VALUE_0084E3A0: u32 = 0x0084_e3a0;
/// `0044dee0`: constructs the one-word response data list (`thiscall`).
const DATA_LIST_CONSTRUCT: u32 = 0x0044_dee0;
/// `0061f440`: destroys it (`thiscall`).
const DATA_LIST_DESTRUCT: u32 = 0x0061_f440;
/// `TESTopicInfo::CloneResponseList` (engine map), `thiscall`, one word.
const CLONE_RESPONSE_LIST: u32 = 0x0061_e760;
/// `0045cd60`: a response data node's next node (`+0x28`).
const DATA_NEXT: u32 = 0x0045_cd60;
/// `00401000`: the game's allocator (`cdecl`, size).
const ALLOCATE: u32 = 0x0040_1000;
/// `0083d280`: constructs a 0x2c-byte response (`thiscall`, five words).
const RESPONSE_CONSTRUCT: u32 = 0x0083_d280;
/// `00905820`: appends to the response list (`thiscall`, address of the
/// item).
const RESPONSE_LIST_ADD: u32 = 0x0090_5820;

// Translated from 0083d8b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083d8b0 - the C++ exception frame (FS:[0]) is not translated
/// No map name: copies the string at `this` into `result` without its
/// `{...}` parts (nesting counted; a `}` with no open brace counts as
/// none; `}` itself is never copied). The copy is built in a temporary
/// string set from the text, written over in place, ended with a 0 when
/// shorter, and given the copied length. Returns `result`.
pub fn fn_0083d8b0(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let this = this.addr();
    e.with_stack(8, |e, temp| {
        let temp = temp.addr();
        let mut count = 0u32;
        let mut depth = 0u32;
        e.call(BS_STRING_CONSTRUCT, &args![temp]);
        let text = e.call(FIRST_WORD, &args![this]).u32();
        e.call(BS_STRING_SET_004037F0, &args![temp, text, 0u32]);
        let mut index = 0u32;
        while index < e.call(CALLEE_004048E0, &args![this]).u32() {
            let text = e.call(FIRST_WORD, &args![this]).u32();
            if e.mem.u8(text.wrapping_add(index)) == 0x7b {
                depth = depth.wrapping_add(1);
            } else {
                let text = e.call(FIRST_WORD, &args![this]).u32();
                if e.mem.u8(text.wrapping_add(index)) == 0x7d {
                    depth = if depth == 0 { 0 } else { depth - 1 };
                }
            }
            if depth == 0 {
                let text = e.call(FIRST_WORD, &args![this]).u32();
                if e.mem.u8(text.wrapping_add(index)) != 0x7d {
                    let source = e.call(FIRST_WORD, &args![this]).u32();
                    let target = fn_0083da30(e, Ptr::new(temp), count);
                    let byte = e.mem.u8(source.wrapping_add(index));
                    e.mem.set_u8(target, byte);
                    count = count.wrapping_add(1);
                }
            }
            index = index.wrapping_add(1);
        }
        if count < e.call(CALLEE_004048E0, &args![temp]).u32() {
            let target = fn_0083da30(e, Ptr::new(temp), count);
            e.mem.set_u8(target, 0);
        }
        e.call(BS_STRING_SET_LENGTH, &args![temp, count]);
        e.call(BS_STRING_ASSIGN, &args![result, temp]);
        e.call(BS_STRING_DESTRUCT, &args![temp]);
    });
    result
}

// Translated from 0083da30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the address of character `index` of the string at `this`
/// (its text pointer plus `index`), or 0 when the text pointer is null.
pub fn fn_0083da30(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let text = e.mem.u32(this.addr());
    let offset = if text != 0 { index } else { 0 };
    offset.wrapping_add(text)
}

// Translated from 0083db00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083db00 - the C++ exception frame (FS:[0]) is not translated
/// No map name: constructs a [`MenuTopic`] for `quest`, `topic`, `info` and
/// `speaker`. With a topic and an info it picks the text (the topic's when
/// the player's slot-8 value for 9 at `+0xa4` is at most the setting at
/// `011d0dc8` and the topic has one, else the info's, else the topic's
/// `+0x18` component's), strips its `{...}` parts ([`fn_0083d8b0`]), sets
/// the flags and fills the responses ([`fn_0083df80`]); when `use_refusal`
/// is set and `0061f260` of the info is false, the info and topic come from
/// `TESTopic::GetTopic(6, 0xc)`'s matching info for the speaker.
/// Returns `this`.
pub fn fn_0083db00(
    e: &mut Engine,
    this: Ptr<MenuTopic>,
    quest: u32,
    topic: u32,
    info: u32,
    speaker: u32,
    use_refusal: bool,
) -> Ptr<MenuTopic> {
    let base = this.addr();
    e.call(BS_STRING_CONSTRUCT, &args![base]);
    e.call(RESPONSE_LIST_CONSTRUCT, &args![base + 0xc]);
    e.set(this, MenuTopic::pCurrentResponse, 0);
    e.set(this, MenuTopic::pQuest, 0);
    e.set(this, MenuTopic::pTopic, 0);
    e.set(this, MenuTopic::pInfo, 0);
    e.set(this, MenuTopic::pOrigTopic, topic);
    if e.call(TOPIC_CHECK_00619DF0, &args![topic]).bool() {
        e.set(this, MenuTopic::bGeneralTopic, 1);
        e.set(this, MenuTopic::bTopicIsChoice, 0);
    } else {
        e.set(this, MenuTopic::bGeneralTopic, 0);
    }
    e.set(this, MenuTopic::bInfoRefusal, 0);
    e.set(this, MenuTopic::bPersuasion, 0);
    let mut final_quest = quest;
    let mut final_topic = topic;

    if topic != 0 && info != 0 {
        let flag = e.call(INFO_CHECK_00619B40, &args![info]).bool();
        e.set(this, MenuTopic::bTopicIsNew, u8::from(!flag));
        e.set(this, MenuTopic::pInfo, info);
        if use_refusal && !e.call(INFO_CHECK_0061F260, &args![info]).bool() {
            let refusal_topic = e.call(TES_TOPIC_GET_TOPIC, &args![6u32, 0xcu32]).u32();
            let matching = if refusal_topic != 0 {
                let player = e.global::<u32>(PLAYER);
                e.call(
                    TES_TOPIC_GET_MATCHING_INFO,
                    &args![refusal_topic, speaker, player],
                )
                .u32()
            } else {
                0
            };
            if matching != 0 {
                e.set(this, MenuTopic::pInfo, matching);
                final_topic = refusal_topic;
                final_quest = e
                    .call(TOPIC_QUEST_FOR_INFO, &args![final_topic, matching])
                    .u32();
            }
            e.set(this, MenuTopic::bInfoRefusal, 1);
        } else {
            let refusal = e.call(INFO_CHECK_0061F260, &args![info]).u8();
            e.set(this, MenuTopic::bInfoRefusal, refusal);
        }

        let player = e.global::<u32>(PLAYER);
        let level = e.vcall(player + 0xa4, 8, &args![9u32]).i32();
        let setting = e
            .call(SETTING_VALUE_ADDRESS, &args![SETTING_011D0DC8])
            .u32();
        let topic_text =
            level <= e.mem.i32(setting) && e.call(TOPIC_HAS_NAME_0083DEA0, &args![topic]).bool();
        if topic_text {
            let text = e.call(TOPIC_NAME_006A9540, &args![topic]).u32();
            e.call(BS_STRING_SET, &args![base, text]);
        } else if fn_0083de40(e, Ptr::new(info)) {
            let text = e.call(INFO_NAME_004610D0, &args![info]).u32();
            e.call(BS_STRING_SET, &args![base, text]);
        } else {
            let text = e.call(COMPONENT_NAME_00408DA0, &args![topic + 0x18]).u32();
            e.call(BS_STRING_SET_004037F0, &args![base, text, 0u32]);
        }
        e.with_stack(8, |e, stripped| {
            let text = fn_0083d8b0(e, Ptr::new(base), stripped);
            e.call(BS_STRING_SET, &args![base, text]);
            e.call(BS_STRING_DESTRUCT, &args![stripped]);
        });

        if e.get(this, MenuTopic::bGeneralTopic) == 0 {
            let mut choice = 0u8;
            let current_info = e.get(this, MenuTopic::pInfo);
            if e.call(INFO_OBJECT_009E32D0, &args![current_info]).u32() != 0 {
                let current_info = e.get(this, MenuTopic::pInfo);
                let object = e.call(INFO_OBJECT_009E32D0, &args![current_info]).u32();
                let list = e.call(OBJECT_LIST_00413F40, &args![object]).u32();
                if !e.call(LIST_IS_EMPTY, &args![list]).bool() {
                    choice = 1;
                }
            }
            e.set(this, MenuTopic::bTopicIsChoice, choice);
        }
        if fn_0083de60(e, Ptr::new(topic)) {
            e.set(this, MenuTopic::bPersuasion, 1);
        }
        let current_info = e.get(this, MenuTopic::pInfo);
        let info_quest = e.call(INFO_QUEST_006733E0, &args![current_info]).u32();
        let current_info = e.get(this, MenuTopic::pInfo);
        let owner_topic = e
            .call(TES_TOPIC_GET_OWNER_TOPIC, &args![current_info])
            .u32();
        let current_info = e.get(this, MenuTopic::pInfo);
        if final_quest == info_quest && final_topic == owner_topic {
            fn_0083df80(e, this, final_quest, final_topic, current_info, speaker);
        } else {
            fn_0083df80(e, this, info_quest, owner_topic, current_info, speaker);
        }
    }

    e.set(this, MenuTopic::pTopic, final_topic);
    e.set(this, MenuTopic::pQuest, final_quest);
    this
}

// Translated from 0083de40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: whether the string at `this + 0x28` (of a topic info) has a
/// non-zero length (`004048e0`).
pub fn fn_0083de40(e: &mut Engine, this: Ptr) -> bool {
    e.call(CALLEE_004048E0, &args![this.addr() + 0x28]).u32() != 0
}

// Translated from 0083de60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: whether `0084e3a0` of `this` (a topic) is `0xfd` or
/// `0x118` (asked again for the second comparison).
pub fn fn_0083de60(e: &mut Engine, this: Ptr) -> bool {
    e.call(VALUE_0084E3A0, &args![this]).u32() == 0xfd
        || e.call(VALUE_0084E3A0, &args![this]).u32() == 0x118
}

// Translated from 0083dec0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083dec0 - the C++ exception frame (FS:[0]) is not translated
/// No map name: the [`MenuTopic`] destructor. Unless `bGeneralTopic` is
/// set, deletes every response of the list (`0083c720(1)` on each non-null
/// item, then the node removed) until it is empty; then destroys the list
/// and the text.
pub fn fn_0083dec0(e: &mut Engine, this: Ptr<MenuTopic>) {
    let base = this.addr();
    if e.get(this, MenuTopic::bGeneralTopic) == 0 {
        let list = base.wrapping_add(0xc);
        while list != 0 && !e.call(LIST_IS_EMPTY, &args![list]).bool() {
            let slot = e.call(LIST_FIRST_ITEM, &args![list]).u32();
            let item = e.mem.u32(slot);
            if item != 0 {
                e.call(RESPONSE_DELETING_DESTRUCTOR, &args![item, 1u32]);
            }
            e.call(LIST_POP_FRONT, &args![list]);
        }
    }
    e.call(RESPONSE_LIST_DESTRUCT, &args![base + 0xc]);
    e.call(BS_STRING_DESTRUCT, &args![base]);
}

// Translated from 0083df80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083df80 - the C++ exception frame (FS:[0]) is not translated
/// No map name: fills the [`MenuTopic`]'s responses from a clone of its
/// info's response list (`TESTopicInfo::CloneResponseList`): for each data
/// node with a non-empty text ([`fn_0083e0a0`]), only the first one for a
/// general topic, a new 0x2c-byte response (`0083d280(quest, topic, info,
/// speaker, node)`, or 0 when the allocation fails) is appended.
pub fn fn_0083df80(
    e: &mut Engine,
    this: Ptr<MenuTopic>,
    quest: u32,
    topic: u32,
    info: u32,
    speaker: u32,
) {
    let base = this.addr();
    e.with_stack(4, |e, data| {
        e.call(DATA_LIST_CONSTRUCT, &args![data]);
        let current_info = e.get(this, MenuTopic::pInfo);
        e.call(CLONE_RESPONSE_LIST, &args![current_info, data]);
        let mut node = e.call(FIRST_WORD, &args![data]).u32();
        let mut count = 0u32;
        while node != 0 {
            if fn_0083e0a0(e, Ptr::new(node)) != 0
                && (e.get(this, MenuTopic::bGeneralTopic) == 0 || count == 0)
            {
                let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
                let response = if block != 0 {
                    e.call(
                        RESPONSE_CONSTRUCT,
                        &args![block, quest, topic, info, speaker, node],
                    )
                    .u32()
                } else {
                    0
                };
                e.with_stack(4, |e, item| {
                    e.mem.set_u32(item.addr(), response);
                    e.call(RESPONSE_LIST_ADD, &args![base + 0xc, item]);
                });
                count = count.wrapping_add(1);
            }
            node = e.call(DATA_NEXT, &args![node]).u32();
        }
        e.call(DATA_LIST_DESTRUCT, &args![data]);
    });
}

/// The text `0083da50` sets (`01011584`; an empty string by its use, not confirmed).
const TEXT_01011584: u32 = 0x0101_1584;

// Translated from 0083da50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083da50 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f12493, state variable around the two members) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): a `MenuTopic` constructor without
/// a topic: constructs `cTopicText` and `lResponses`, clears `pCurrentResponse`, `pQuest`,
/// `pTopic` and `pInfo`, sets the text to the one at `01011584` (`004037f0` with 0) and
/// clears `bInfoRefusal` and `bPersuasion`. Returns `this`.
pub fn fn_0083da50(e: &mut Engine, this: Ptr<MenuTopic>) -> Ptr<MenuTopic> {
    e.call(BS_STRING_CONSTRUCT, &args![this]);
    e.call(RESPONSE_LIST_CONSTRUCT, &args![this.addr() + 0xc]);
    e.set(this, MenuTopic::pCurrentResponse, 0);
    e.set(this, MenuTopic::pQuest, 0);
    e.set(this, MenuTopic::pTopic, 0);
    e.set(this, MenuTopic::pInfo, 0);
    e.call(BS_STRING_SET_004037F0, &args![this, TEXT_01011584, 0u32]);
    e.set(this, MenuTopic::bInfoRefusal, 0);
    e.set(this, MenuTopic::bPersuasion, 0);
    this
}

// Translated from 0083dea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): whether `004048e0` on the member
/// at +0x34 answers non-zero (unsigned), as 0 or 1.
pub fn fn_0083dea0(e: &mut Engine, this: Ptr) -> u32 {
    let value = e.call(CALLEE_004048E0, &args![this.addr() + 0x34]).u32();
    u32::from(value != 0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_da50, fn_0083da50(Ptr<MenuTopic>) -> Ptr<MenuTopic>),
        entry!(0x0083_dea0, fn_0083dea0(Ptr) -> u32),
        entry!(0x0083_e0a0, fn_0083e0a0(Ptr) -> u32),
        entry!(0x0083_d8b0, fn_0083d8b0(Ptr, Ptr) -> Ptr),
        entry!(0x0083_da30, fn_0083da30(Ptr, u32) -> u32),
        entry!(
            0x0083_db00,
            fn_0083db00(Ptr<MenuTopic>, u32, u32, u32, u32, bool) -> Ptr<MenuTopic>
        ),
        entry!(0x0083_de40, fn_0083de40(Ptr) -> bool),
        entry!(0x0083_de60, fn_0083de60(Ptr) -> bool),
        entry!(0x0083_dec0, fn_0083dec0(Ptr<MenuTopic>)),
        entry!(0x0083_df80, fn_0083df80(Ptr<MenuTopic>, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0083e0a0_passes_this_plus_18_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_004048E0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_e0a0, &args![0x4100u32]).u32(), 0x144d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_004048E0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x18]);
    }
}

#[cfg(test)]
mod b040_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// String doubles over plain memory: a string is (text pointer, length
    /// word); the text is in its own block.
    fn string_doubles(e: &mut Engine) {
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CALLEE_004048E0, |e, a| {
            u32::from(e.mem.u16(a[0] + 4)).into_ret()
        });
        e.register(BS_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(BS_STRING_SET_004037F0, |e, a| {
            let text = e.mem.cstr(a[1]);
            let block = e.mem.alloc(text.len() as u32 + 1);
            e.mem.set_cstr(block, &text);
            e.mem.set_u32(a[0], block);
            e.mem.set_u16(a[0] + 4, text.len() as u16);
            Ret::default()
        });
        e.register(BS_STRING_SET_LENGTH, |e, a| {
            e.mem.set_u16(a[0] + 4, a[1] as u16);
            Ret::default()
        });
        e.register(BS_STRING_ASSIGN, |e, a| {
            let text = e.mem.u32(a[1]);
            let length = e.mem.u32(a[1] + 4);
            e.mem.set_u32(a[0], text);
            e.mem.set_u32(a[0] + 4, length);
            Ret::default()
        });
        e.register(BS_STRING_DESTRUCT, |_, _| Ret::default());
    }

    fn make_string(e: &mut Engine, text: &[u8]) -> u32 {
        let string = e.mem.alloc(8);
        let block = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(block, text);
        e.mem.set_u32(string, block);
        e.mem.set_u16(string + 4, text.len() as u16);
        string
    }

    #[test]
    fn fn_0083d8b0_strips_braced_parts() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        for (text, expected) in [
            (&b"Hello {smiles} there"[..], &b"Hello  there"[..]),
            (b"a{b{c}d}e", b"ae"),
            (b"x}y}", b"xy"),
            (b"plain", b"plain"),
        ] {
            let string = make_string(&mut e, text);
            let result = e.mem.alloc(8);
            let r = e.call(0x0083_d8b0, &args![string, result]).u32();
            assert_eq!(r, result);
            let length = e.mem.u16(result + 4) as usize;
            assert_eq!(length, expected.len());
            assert_eq!(e.mem.cstr(e.mem.u32(result)), expected);
        }
    }

    #[test]
    fn fn_0083da30_indexes_or_gives_null() {
        let mut e = Engine::new();
        let string = e.mem.alloc(8);
        e.mem.set_u32(string, 0x5000);
        assert_eq!(e.call(0x0083_da30, &args![string, 7u32]).u32(), 0x5007);
        e.mem.set_u32(string, 0);
        assert_eq!(e.call(0x0083_da30, &args![string, 7u32]).u32(), 0);
    }

    #[test]
    fn fn_0083de40_and_de60() {
        let mut e = Engine::new();
        e.register(CALLEE_004048E0, |e, a| e.mem.u32(a[0]).into_ret());
        let info = e.mem.alloc(0x30);
        assert!(!e.call(0x0083_de40, &args![info]).bool());
        e.mem.set_u32(info + 0x28, 3);
        assert!(e.call(0x0083_de40, &args![info]).bool());

        e.register(VALUE_0084E3A0, |e, a| e.mem.u32(a[0]).into_ret());
        let topic = e.mem.alloc(4);
        for (id, expected) in [(0xfdu32, true), (0x118, true), (0x117, false)] {
            e.mem.set_u32(topic, id);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0083_de60, &args![topic]).bool(), expected);
            let asked = calls_to(&e, VALUE_0084E3A0).len();
            assert_eq!(asked, if id == 0xfd { 1 } else { 2 });
        }
    }

    #[test]
    fn fn_0083dec0_deletes_responses_unless_general() {
        let mut e = Engine::new();
        let this = e.new_object::<MenuTopic>();
        let base = this.addr();
        // A list of three slots: two items and a null.
        let slots = e.mem.alloc(12);
        e.mem.set_u32(slots, 0x6100);
        e.mem.set_u32(slots + 8, 0x6300);
        e.mem.set_u32(base + 0xc, slots);
        e.mem.set_u32(base + 0x10, 3);
        e.register(LIST_IS_EMPTY, |e, a| (e.mem.u32(a[0] + 4) == 0).into_ret());
        e.register(LIST_FIRST_ITEM, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_POP_FRONT, |e, a| {
            let head = e.mem.u32(a[0]);
            let left = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0], head + 4);
            e.mem.set_u32(a[0] + 4, left - 1);
            Ret::default()
        });
        for addr in [
            RESPONSE_DELETING_DESTRUCTOR,
            RESPONSE_LIST_DESTRUCT,
            BS_STRING_DESTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0083_dec0, &args![this]);
        assert_eq!(
            calls_to(&e, RESPONSE_DELETING_DESTRUCTOR),
            vec![vec![0x6100, 1], vec![0x6300, 1]]
        );
        assert_eq!(calls_to(&e, LIST_POP_FRONT).len(), 3);
        assert_eq!(calls_to(&e, RESPONSE_LIST_DESTRUCT), vec![vec![base + 0xc]]);
        assert_eq!(calls_to(&e, BS_STRING_DESTRUCT), vec![vec![base]]);

        e.set(this, MenuTopic::bGeneralTopic, 1);
        e.mem.set_u32(base + 0x10, 3);
        e.call_log = Some(vec![]);
        e.call(0x0083_dec0, &args![this]);
        assert!(calls_to(&e, LIST_IS_EMPTY).is_empty());
        assert_eq!(calls_to(&e, RESPONSE_LIST_DESTRUCT).len(), 1);
    }

    /// Response data nodes: text length at +0x18+4 (through `004048e0`),
    /// next at +0x28.
    fn data_nodes(e: &mut Engine, lengths: &[u16]) -> u32 {
        let mut next = 0;
        for length in lengths.iter().rev() {
            let node = e.mem.alloc(0x2c);
            e.mem.set_u16(node + 0x18 + 4, *length);
            e.mem.set_u32(node + 0x28, next);
            next = node;
        }
        next
    }

    fn responses_engine(e: &mut Engine, first: u32) {
        e.register(CALLEE_004048E0, |e, a| {
            u32::from(e.mem.u16(a[0] + 4)).into_ret()
        });
        e.register(DATA_LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register_double(CLONE_RESPONSE_LIST, move |e, a| {
            e.mem.set_u32(a[1], first);
            Ret::default()
        });
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(DATA_NEXT, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(RESPONSE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(RESPONSE_LIST_ADD, |_, _| Ret::default());
        e.register(DATA_LIST_DESTRUCT, |_, _| Ret::default());
    }

    #[test]
    fn fn_0083df80_appends_responses_with_text() {
        let mut e = Engine::new();
        let first = data_nodes(&mut e, &[4, 0, 6]);
        responses_engine(&mut e, first);
        let this = e.new_object::<MenuTopic>();
        e.set(this, MenuTopic::pInfo, 0x4400);
        e.call_log = Some(vec![]);
        e.call(
            0x0083_df80,
            &args![this, 0x11u32, 0x22u32, 0x33u32, 0x44u32],
        );
        let clone = calls_to(&e, CLONE_RESPONSE_LIST);
        assert_eq!(clone[0][0], 0x4400);
        let made = calls_to(&e, RESPONSE_CONSTRUCT);
        assert_eq!(made.len(), 2);
        assert_eq!(made[0][1..], [0x11, 0x22, 0x33, 0x44, first]);
        let third = e.mem.u32(e.mem.u32(first + 0x28) + 0x28);
        assert_eq!(made[1][5], third);
        let added = calls_to(&e, RESPONSE_LIST_ADD);
        assert_eq!(added.len(), 2);
        assert_eq!(added[0][0], this.addr() + 0xc);
        assert_eq!(calls_to(&e, DATA_LIST_DESTRUCT).len(), 1);

        // A general topic keeps only the first; a failed allocation adds 0.
        e.set(this, MenuTopic::bGeneralTopic, 1);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.register_double(RESPONSE_LIST_ADD, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(
            0x0083_df80,
            &args![this, 0x11u32, 0x22u32, 0x33u32, 0x44u32],
        );
        assert!(calls_to(&e, RESPONSE_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&e, RESPONSE_LIST_ADD).len(), 1);
    }

    struct Topic {
        this: Ptr<MenuTopic>,
        topic: u32,
        info: u32,
        setting: u32,
    }

    fn topic_engine(e: &mut Engine) -> Topic {
        string_doubles(e);
        let first = data_nodes(e, &[]);
        responses_engine(e, first);
        e.register(RESPONSE_LIST_CONSTRUCT, |_, _| Ret::default());
        e.register(TOPIC_CHECK_00619DF0, |_, _| 0u32.into_ret());
        e.register(INFO_CHECK_00619B40, |_, _| 1u32.into_ret());
        e.register(INFO_CHECK_0061F260, |_, _| 0u32.into_ret());
        e.register(TES_TOPIC_GET_TOPIC, |_, _| 0x7a00u32.into_ret());
        e.register(TES_TOPIC_GET_MATCHING_INFO, |_, _| 0x7b00u32.into_ret());
        e.register(TOPIC_QUEST_FOR_INFO, |_, _| 0x7c00u32.into_ret());
        let player = e.mem.alloc(0xa8);
        let player_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(player + 0xa4, player_vtable);
        e.mem.set_u32(player_vtable + 8, 0x7000_0008);
        e.register(0x7000_0008, |_, _| 5u32.into_ret());
        e.map(PLAYER, 4);
        e.set_global(PLAYER, player);
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 10);
        e.register_double(SETTING_VALUE_ADDRESS, move |_, _| setting.into_ret());
        e.register(TOPIC_HAS_NAME_0083DEA0, |_, _| 1u32.into_ret());
        let topic_text = e.mem.alloc(16);
        e.mem.set_cstr(topic_text, b"Ask {x}");
        e.register_double(TOPIC_NAME_006A9540, move |_, _| topic_text.into_ret());
        e.register(INFO_NAME_004610D0, |_, _| Ret::default());
        e.register_double(COMPONENT_NAME_00408DA0, move |_, _| topic_text.into_ret());
        // Set from a C string: (text, 0); Set from a string: copy it.
        e.register(BS_STRING_SET, |e, a| {
            // A string object (text pointer, length) rather than C text: its first word points
            // at text as long as its second word says (stack locals are not heap blocks).
            let text = e.mem.u32(a[1]);
            let is_string =
                e.mem.is_mapped(text) && e.mem.u32(a[1] + 4) as usize == e.mem.cstr(text).len();
            if is_string {
                let text = e.mem.u32(a[1]);
                let length = e.mem.u32(a[1] + 4);
                e.mem.set_u32(a[0], text);
                e.mem.set_u32(a[0] + 4, length);
            } else {
                let length = e.mem.cstr(a[1]).len() as u32;
                e.mem.set_u32(a[0], a[1]);
                e.mem.set_u32(a[0] + 4, length);
            }
            Ret::default()
        });
        e.register(INFO_OBJECT_009E32D0, |_, _| 0x7d00u32.into_ret());
        e.register(OBJECT_LIST_00413F40, |_, _| 0x7e00u32.into_ret());
        e.register(LIST_IS_EMPTY, |_, _| 0u32.into_ret());
        e.register(VALUE_0084E3A0, |_, _| 0xfdu32.into_ret());
        e.register(INFO_QUEST_006733E0, |_, _| 0x1111u32.into_ret());
        e.register(TES_TOPIC_GET_OWNER_TOPIC, |_, _| 0x2222u32.into_ret());
        let info = e.mem.alloc(0x30);
        Topic {
            this: e.new_object::<MenuTopic>(),
            topic: 0x2222,
            info,
            setting,
        }
    }

    #[test]
    fn fn_0083db00_builds_a_topic_with_its_text() {
        let mut e = Engine::new();
        let t = topic_engine(&mut e);
        e.map(t.topic, 0x40);
        e.call_log = Some(vec![]);
        let r = e.call(
            0x0083_db00,
            &args![t.this, 0x1111u32, t.topic, t.info, 0x5555u32, 0u32],
        );
        assert_eq!(r.u32(), t.this.addr());
        assert_eq!(e.get(t.this, MenuTopic::pOrigTopic), t.topic);
        assert_eq!(e.get(t.this, MenuTopic::pInfo), t.info);
        assert_eq!(e.get(t.this, MenuTopic::pQuest), 0x1111);
        assert_eq!(e.get(t.this, MenuTopic::pTopic), t.topic);
        assert_eq!(e.get(t.this, MenuTopic::bTopicIsNew), 0);
        assert_eq!(e.get(t.this, MenuTopic::bInfoRefusal), 0);
        assert_eq!(e.get(t.this, MenuTopic::bTopicIsChoice), 1);
        assert_eq!(e.get(t.this, MenuTopic::bPersuasion), 1);
        assert_eq!(e.get(t.this, MenuTopic::bGeneralTopic), 0);
        // The topic's text, its braces stripped.
        let text = e.mem.u32(t.this.addr());
        assert_eq!(e.mem.cstr(text), b"Ask ");
        assert!(calls_to(&e, TES_TOPIC_GET_TOPIC).is_empty());
        assert_eq!(calls_to(&e, CLONE_RESPONSE_LIST)[0][0], t.info);
        // The text when the player's value is above the setting: the
        // component's (the info has no text of its own).
        e.mem.set_u32(t.setting, 1);
        e.call_log = Some(vec![]);
        e.call(
            0x0083_db00,
            &args![t.this, 0x1111u32, t.topic, t.info, 0x5555u32, 0u32],
        );
        assert!(calls_to(&e, TOPIC_NAME_006A9540).is_empty());
        assert_eq!(
            calls_to(&e, COMPONENT_NAME_00408DA0),
            vec![vec![t.topic + 0x18]]
        );
    }

    #[test]
    fn fn_0083db00_refusal_and_missing_parts() {
        let mut e = Engine::new();
        let t = topic_engine(&mut e);
        e.map(t.topic, 0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0083_db00,
            &args![t.this, 0x1111u32, t.topic, t.info, 0x5555u32, 1u32],
        );
        let player = e.global::<u32>(PLAYER);
        assert_eq!(calls_to(&e, TES_TOPIC_GET_TOPIC), vec![vec![6, 0xc]]);
        assert_eq!(
            calls_to(&e, TES_TOPIC_GET_MATCHING_INFO),
            vec![vec![0x7a00, 0x5555, player]]
        );
        assert_eq!(e.get(t.this, MenuTopic::pInfo), 0x7b00);
        assert_eq!(e.get(t.this, MenuTopic::bInfoRefusal), 1);
        // Quest and topic differ from the info's: the info's are used.
        let fill = calls_to(&e, CLONE_RESPONSE_LIST);
        assert_eq!(fill[0][0], 0x7b00);
        assert_eq!(e.get(t.this, MenuTopic::pQuest), 0x7c00);
        assert_eq!(e.get(t.this, MenuTopic::pTopic), 0x7a00);

        // No info: only the fields.
        e.register(TOPIC_CHECK_00619DF0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(
            0x0083_db00,
            &args![t.this, 0x1111u32, t.topic, 0u32, 0x5555u32, 1u32],
        );
        assert_eq!(e.get(t.this, MenuTopic::bGeneralTopic), 1);
        assert_eq!(e.get(t.this, MenuTopic::bTopicIsChoice), 0);
        assert_eq!(e.get(t.this, MenuTopic::pInfo), 0);
        assert_eq!(e.get(t.this, MenuTopic::pQuest), 0x1111);
        assert!(calls_to(&e, INFO_CHECK_00619B40).is_empty());
    }
}

#[cfg(test)]
mod tests_n077 {
    use super::*;

    #[test]
    fn fn_0083da50_constructs_an_empty_topic() {
        let mut e = Engine::new();
        for address in [
            BS_STRING_CONSTRUCT,
            RESPONSE_LIST_CONSTRUCT,
            BS_STRING_SET_004037F0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x2c);
        for offset in [0x14, 0x18, 0x20, 0x28] {
            e.mem.set_u32(this + offset, 0x1111);
        }
        e.mem.set_u8(this + 9, 1);
        e.mem.set_u8(this + 0xa, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_da50, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (BS_STRING_CONSTRUCT, vec![this]),
                (RESPONSE_LIST_CONSTRUCT, vec![this + 0xc]),
                (BS_STRING_SET_004037F0, vec![this, TEXT_01011584, 0]),
            ]
        );
        for offset in [0x14, 0x18, 0x20, 0x28] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!((e.mem.u8(this + 9), e.mem.u8(this + 0xa)), (0, 0));
    }

    #[test]
    fn fn_0083dea0_answers_whether_the_member_is_set() {
        let mut e = Engine::new();
        e.register(CALLEE_004048E0, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x0083_dea0, &args![this]).u32(), 0);
        e.mem.set_u32(this + 0x34, 0x8000_0000);
        assert_eq!(e.call(0x0083_dea0, &args![this]).u32(), 1);
    }
}

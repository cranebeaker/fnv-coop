//! `fallout/dialogue/dialogueitem.cpp` (Xbox PDB source unit), subsystem `fallout/dialogue`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `DialogueItem` keeps its responses in a `BSSimpleList<DialogueResponse *>` at +0; the
//! list helpers it calls: `006815c0` (the address of a node's item), `00726070` (the next
//! node), `008256d0` (whether a node is an empty list).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `DialogueItem` (Xbox PDB), 0x1c bytes.
    pub struct DialogueItem: 0x1c {
        /// `lResponses` (Xbox PDB): `BSSimpleList<DialogueResponse *>`, inline.
        0x00 lResponses: u32,
        /// `pCurrentResponse` (Xbox PDB): `BSSimpleList<DialogueResponse *>*`.
        0x08 pCurrentResponse: u32,
        /// `pInfo` (Xbox PDB): `TESTopicInfo*`.
        0x0C pInfo: u32,
        /// `pTopic` (Xbox PDB): `TESTopic*`.
        0x10 pTopic: u32,
        /// `pQuest` (Xbox PDB): `TESQuest*`.
        0x14 pQuest: u32,
        /// `pSpeaker` (Xbox PDB): `TESObjectREFR*`.
        0x18 pSpeaker: u32,
    }
}

/// `006815c0` (`thiscall`): the address of a list node's item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`): a list node's next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `008256d0` (`thiscall`): whether a list is empty (`AL`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `0063f7b0` (`thiscall`): removes a list's first item.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The list destructor `0046ffb0` (`thiscall`).
const LIST_DESTRUCTOR: u32 = 0x0046_ffb0;
/// The list constructor `0096a2d0` (`thiscall`).
const LIST_CONSTRUCTOR: u32 = 0x0096_a2d0;
/// `00905820` (`thiscall`, one word: the address of the item): adds an item to a list.
const LIST_ADD: u32 = 0x0090_5820;
/// The response-list iterator constructor `0044dee0`, its current node `00559450`, a
/// node's next node `0045cd60` and the iterator destructor `0061f440` (all `thiscall`).
const ITERATOR_CONSTRUCTOR: u32 = 0x0044_dee0;
const ITERATOR_NODE: u32 = 0x0055_9450;
const NEXT_NODE_0045CD60: u32 = 0x0045_cd60;
const ITERATOR_DESTRUCTOR: u32 = 0x0061_f440;
/// `TESTopicInfo::CloneResponseList` (engine map, `0061e760`, `thiscall`, one word: the
/// iterator to fill).
const CLONE_RESPONSE_LIST: u32 = 0x0061_e760;
/// The allocator `00401000` and the free `00401030` (`cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// The `DialogueResponse` constructor `0083d280` (`thiscall`, five words).
const RESPONSE_CONSTRUCTOR: u32 = 0x0083_d280;
/// The `BSStringT<char>` destructor `004037d0` (`thiscall`).
const STRING_DESTRUCTOR: u32 = 0x0040_37d0;
/// `00579200` (`thiscall`), `TESTopicInfo::AddTopicList` (`0061f150`, `thiscall`) and
/// `TESTopicInfo::RunResult` (`0061f170`, `thiscall`, two words).
const INFO_TEST_00579200: u32 = 0x0057_9200;
const ADD_TOPIC_LIST: u32 = 0x0061_f150;
const RUN_RESULT: u32 = 0x0061_f170;
/// `005b5e40` (`cdecl`, one word: a message) with the message at `0107dba8`.
const REPORT_005B5E40: u32 = 0x005b_5e40;
const MISSING_RESPONSE_MESSAGE: u32 = 0x0107_dba8;
/// `DialogueResponse::SaveGame` (`0083d690`, `thiscall`, one word).
const RESPONSE_SAVE_GAME: u32 = 0x0083_d690;
/// `BGSSaveGameBuffer` writers (`thiscall`): `StartVariableSizedValue` (`00865f20`),
/// `SaveVariableSizedValue_ov2` (`00865ff0`, two words), the raw write `00865e50` (three
/// words: address, size, 0) and `SaveFormID_ov2` (`00865df0`, two words).
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
const SAVE_DATA: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5df0;

/// The item of a list node (`*006815c0(node)`).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

// Translated from 0083c520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::DialogueItem_ov2` (engine map; `thiscall`, `RET 0x10`: quest, topic,
/// info, speaker): constructs the response list and clears `pCurrentResponse` and `pInfo`.
/// With a topic, an info and a quest it keeps all four and makes one `DialogueResponse`
/// (0x2c bytes, `0083d280` with the four and the info's response node; null when the
/// allocation fails) for each response of the info (an iterator on the stack filled by
/// `TESTopicInfo::CloneResponseList`), adding each to the list. Returns `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0083c520 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn dialogue_item_constructor(
    e: &mut Engine,
    this: Ptr<DialogueItem>,
    quest: u32,
    topic: u32,
    info: u32,
    speaker: u32,
) -> Ptr<DialogueItem> {
    e.call(LIST_CONSTRUCTOR, &args![this]);
    e.set(this, DialogueItem::pCurrentResponse, 0);
    e.set(this, DialogueItem::pInfo, 0);
    if topic == 0 || info == 0 || quest == 0 {
        return this;
    }
    e.set(this, DialogueItem::pInfo, info);
    e.set(this, DialogueItem::pTopic, topic);
    e.set(this, DialogueItem::pQuest, quest);
    e.set(this, DialogueItem::pSpeaker, speaker);
    e.with_stack(8, |e, locals| {
        let iterator = locals.addr();
        let item = iterator + 4;
        e.call(ITERATOR_CONSTRUCTOR, &args![iterator]);
        e.call(CLONE_RESPONSE_LIST, &args![info, iterator]);
        let mut node = e.call(ITERATOR_NODE, &args![iterator]).u32();
        while node != 0 {
            let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
            let response = if block != 0 {
                e.call(
                    RESPONSE_CONSTRUCTOR,
                    &args![block, quest, topic, info, speaker, node],
                )
                .u32()
            } else {
                0
            };
            e.mem.set_u32(item, response);
            e.call(LIST_ADD, &args![this, item]);
            node = e.call(NEXT_NODE_0045CD60, &args![node]).u32();
        }
        e.call(ITERATOR_DESTRUCTOR, &args![iterator]);
    });
    this
}

// Translated from 0083c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::~DialogueItem` (engine map; `thiscall`): while the response list is not
/// empty, deletes its first response ([`fn_0083c720`] with 1, when not null) and removes
/// it (`0063f7b0`); then destroys the list (`0046ffb0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0083c670 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn dialogue_item_destructor(e: &mut Engine, this: Ptr<DialogueItem>) {
    let list = this.addr();
    while list != 0 && e.call(LIST_IS_EMPTY, &args![list]).u8() == 0 {
        let response = node_item(e, list);
        if response != 0 {
            fn_0083c720(e, Ptr::new(response), 1);
        }
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
    e.call(LIST_DESTRUCTOR, &args![this]);
}

// Translated from 0083c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `DialogueResponse` scalar deleting destructor (unnamed in the engine map;
/// `thiscall`, `RET 4`: the flags): runs [`fn_0083c750`] and frees the response
/// (`00401030`) when bit 0 of the flags is set. Returns `this`.
pub fn fn_0083c720(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0083c750(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0083c750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `DialogueResponse` destructor body (unnamed in the engine map; `thiscall`):
/// destroys `cVoice` (+0x10) and then `cText` (+0) (`BSStringT<char>`, `004037d0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0083c750 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_0083c750(e: &mut Engine, this: Ptr) {
    e.call(STRING_DESTRUCTOR, &args![this.addr() + 0x10]);
    e.call(STRING_DESTRUCTOR, &args![this]);
}

// Translated from 0083c7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::FirstResponse` (engine map; `thiscall`): points `pCurrentResponse` at
/// the list's first node and answers whether it holds a response.
pub fn dialogue_item_first_response(e: &mut Engine, this: Ptr<DialogueItem>) -> bool {
    e.set(this, DialogueItem::pCurrentResponse, this.addr());
    let current = e.get(this, DialogueItem::pCurrentResponse);
    node_item(e, current) != 0
}

// Translated from 0083c7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::NextResponse` (engine map; `thiscall`): moves `pCurrentResponse` to the
/// next node (when it is set) and answers whether it is still set.
pub fn dialogue_item_next_response(e: &mut Engine, this: Ptr<DialogueItem>) -> bool {
    let current = e.get(this, DialogueItem::pCurrentResponse);
    if current != 0 {
        let next = e.call(NODE_NEXT, &args![current]).u32();
        e.set(this, DialogueItem::pCurrentResponse, next);
    }
    e.get(this, DialogueItem::pCurrentResponse) != 0
}

// Translated from 0083c820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::GetCurrentResponse` (engine map; `thiscall`): the response of
/// `pCurrentResponse`, or null when it is not set.
pub fn dialogue_item_get_current_response(e: &mut Engine, this: Ptr<DialogueItem>) -> u32 {
    let current = e.get(this, DialogueItem::pCurrentResponse);
    if current == 0 {
        return 0;
    }
    node_item(e, current)
}

// Translated from 0083c850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::RunResult` (engine map; `thiscall`, `RET 4`: a word passed on): when there
/// is an info and its `00579200` answers false, adds its topic list
/// (`TESTopicInfo::AddTopicList`) and runs its result script (`TESTopicInfo::RunResult`
/// with the word and `pSpeaker`).
pub fn dialogue_item_run_result(e: &mut Engine, this: Ptr<DialogueItem>, value: u32) {
    let info = e.get(this, DialogueItem::pInfo);
    if info == 0 || e.call(INFO_TEST_00579200, &args![info]).u8() != 0 {
        return;
    }
    e.call(ADD_TOPIC_LIST, &args![info]);
    let speaker = e.get(this, DialogueItem::pSpeaker);
    let info = e.get(this, DialogueItem::pInfo);
    e.call(RUN_RESULT, &args![info, value, speaker]);
}

// Translated from 0083c900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::GetDialogueResponseIndex` (engine map; `thiscall`, `RET 4`: a response):
/// the 16-bit index of the response in the list, counting from 0 until the list ends or
/// an empty node is reached. A response not found reports the message at `0107dba8`
/// (`005b5e40`) and answers 0.
pub fn dialogue_item_get_dialogue_response_index(
    e: &mut Engine,
    this: Ptr<DialogueItem>,
    response: u32,
) -> u16 {
    let mut index = 0u16;
    let mut node = this.addr();
    while node != 0 && e.call(LIST_IS_EMPTY, &args![node]).u8() == 0 {
        if node_item(e, node) == response {
            return index;
        }
        index = index.wrapping_add(1);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(REPORT_005B5E40, &args![MISSING_RESPONSE_MESSAGE]);
    0
}

// Translated from 0083ce40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::SaveGame_ov2` (engine map; `thiscall`, `RET 4`: the save buffer): saves
/// every non-null response (`DialogueResponse::SaveGame`) as one variable-sized value of
/// their count, then the 16-bit index of the current response (-1 without one; two bytes)
/// and the form ids of `pInfo`, `pTopic`, `pQuest` and `pSpeaker`.
pub fn dialogue_item_save_game(e: &mut Engine, this: Ptr<DialogueItem>, buffer: u32) {
    let mut count = 0u32;
    let mark = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut node = this.addr();
    while node != 0 {
        let response = node_item(e, node);
        if response != 0 {
            e.call(RESPONSE_SAVE_GAME, &args![response, buffer]);
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count, mark]);
    let current = e.get(this, DialogueItem::pCurrentResponse);
    let index = if current != 0 {
        let response = node_item(e, current);
        // CWDE: the index sign-extended.
        dialogue_item_get_dialogue_response_index(e, this, response) as i16 as i32
    } else {
        -1
    };
    e.with_stack(4, |e, word| {
        e.mem.set_u16(word.addr(), index as u16);
        e.call(SAVE_DATA, &args![buffer, word, 2u32, 0u32]);
    });
    for field in [
        DialogueItem::pInfo,
        DialogueItem::pTopic,
        DialogueItem::pQuest,
        DialogueItem::pSpeaker,
    ] {
        let form = e.get(this, field);
        e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
}

/// The message `0083c970` reports when the index is past the end of the list (`0107dc20`).
const INDEX_TOO_LARGE_MESSAGE: u32 = 0x0107_dc20;
/// `DialogueResponse` functions of `dialogueresponse.cpp` (all `thiscall` on a response):
/// the constructor `0083d1e0`, the save size `0083d420`, `SaveGame` `0083d490`, `LoadGame`
/// `0083d550` and `LoadGame_ov2` `0083d740` (one word: the load buffer).
const RESPONSE_DEFAULT_CONSTRUCTOR: u32 = 0x0083_d1e0;
const RESPONSE_SAVE_SIZE: u32 = 0x0083_d420;
const RESPONSE_SAVE_GAME_OLD: u32 = 0x0083_d490;
const RESPONSE_LOAD_GAME_OLD: u32 = 0x0083_d550;
const RESPONSE_LOAD_GAME: u32 = 0x0083_d740;
/// The save/load singleton holder (`011de45c`) and its `thiscall` helpers: `00825c00` (the
/// address the next byte goes to), `008579b0` (write: address, size), `00857a10`
/// (`TESSaveLoadGame::SaveNumericID`: address, size), `008579e0` (read: address, size) and
/// `00857aa0` (`TESSaveLoadGame::LoadNumericID`: address, size).
const SAVE_LOAD_GAME_011DE45C: u32 = 0x011d_e45c;
const SAVE_NEXT_BYTE_00825C00: u32 = 0x0082_5c00;
const SAVE_WRITE_008579B0: u32 = 0x0085_79b0;
const SAVE_NUMERIC_ID_00857A10: u32 = 0x0085_7a10;
const LOAD_READ_008579E0: u32 = 0x0085_79e0;
const LOAD_NUMERIC_ID_00857AA0: u32 = 0x0085_7aa0;
/// A form's save id (`0084e3a0`, `thiscall`).
const FORM_SAVE_ID_0084E3A0: u32 = 0x0084_e3a0;
/// `BGSLoadGameBuffer` readers (`thiscall` on the buffer): `LoadVariableSizedValue`
/// (`00864a60`), the raw read `00864980` (address, size), `LoadFormID` (`008648a0`) and
/// `LoadFormID_ov2` (`008648e0`, one word: where the form goes).
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0086_4a60;
const LOAD_DATA: u32 = 0x0086_4980;
const LOAD_FORM_ID: u32 = 0x0086_48a0;
const LOAD_FORM_ID_INTO: u32 = 0x0086_48e0;
/// `004839c0` (`cdecl`, one word: a form) and `__RTDynamicCast` (`00ec43fb`, `cdecl`: object,
/// 0, source type, target type, 0).
const CAST_SOURCE_004839C0: u32 = 0x0048_39c0;
const DYNAMIC_CAST_00EC43FB: u32 = 0x00ec_43fb;
/// The type descriptors the casts use: the source `01183028` and the targets for `pInfo`
/// (`01184738`), `pTopic` (`01184720`), `pQuest` (`01186500`) and `pSpeaker` (`011841cc`).
const FORM_TYPE_01183028: u32 = 0x0118_3028;
const INFO_TYPE_01184738: u32 = 0x0118_4738;
const TOPIC_TYPE_01184720: u32 = 0x0118_4720;
const QUEST_TYPE_01186500: u32 = 0x0118_6500;
const SPEAKER_TYPE_011841CC: u32 = 0x0118_41cc;

/// `__RTDynamicCast(004839c0(form), 0, 01183028, target, 0)`.
fn cast_form(e: &mut Engine, form: u32, target: u32) -> u32 {
    let object = e.call(CAST_SOURCE_004839C0, &args![form]).u32();
    e.call(
        DYNAMIC_CAST_00EC43FB,
        &args![object, 0u32, FORM_TYPE_01183028, target, 0u32],
    )
    .u32()
}

// Translated from 0083c4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::DialogueItem` (engine map; `thiscall`): constructs the response list
/// (`0096a2d0`) and clears `pInfo`, `pTopic`, `pQuest`, `pSpeaker` and `pCurrentResponse`.
/// Returns `this`.
pub fn dialogue_item_default_constructor(
    e: &mut Engine,
    this: Ptr<DialogueItem>,
) -> Ptr<DialogueItem> {
    e.call(LIST_CONSTRUCTOR, &args![this]);
    e.set(this, DialogueItem::pInfo, 0);
    e.set(this, DialogueItem::pTopic, 0);
    e.set(this, DialogueItem::pQuest, 0);
    e.set(this, DialogueItem::pSpeaker, 0);
    e.set(this, DialogueItem::pCurrentResponse, 0);
    this
}

// Translated from 0083c8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::SetCurrentResponse` (engine map; `thiscall`, `RET 4`: a response): points
/// `pCurrentResponse` at the first node holding the response, searching until the list ends or
/// an empty node is reached; unchanged when not found.
pub fn dialogue_item_set_current_response(e: &mut Engine, this: Ptr<DialogueItem>, response: u32) {
    let mut node = this.addr();
    while node != 0 && e.call(LIST_IS_EMPTY, &args![node]).u8() == 0 {
        if node_item(e, node) == response {
            e.set(this, DialogueItem::pCurrentResponse, node);
            return;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0083c970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::GetDialogueResponseByIndex` (engine map; `thiscall`, `RET 4`: a 16-bit index,
/// compared sign-extended): the response at the index, counting from 0 until the list ends or
/// an empty node is reached; past the end it reports the message at `0107dc20` (`005b5e40`) and
/// answers 0.
pub fn dialogue_item_get_dialogue_response_by_index(
    e: &mut Engine,
    this: Ptr<DialogueItem>,
    index: u32,
) -> u32 {
    let wanted = index as u16 as i16;
    let mut count = 0i16;
    let mut node = this.addr();
    while node != 0 && e.call(LIST_IS_EMPTY, &args![node]).u8() == 0 {
        if count == wanted {
            return node_item(e, node);
        }
        count = count.wrapping_add(1);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(REPORT_005B5E40, &args![INDEX_TOO_LARGE_MESSAGE]);
    0
}

// Translated from 0083c9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the item's save size as a 16-bit value: 1, plus each response's size
/// (`0083d420`, `AX`) until the list ends or an empty node is reached, plus 1 and four times 4,
/// all in 16 bits (`AX`).
pub fn fn_0083c9e0(e: &mut Engine, this: Ptr<DialogueItem>) -> u16 {
    let mut size = 1u16;
    let mut node = this.addr();
    while node != 0 && e.call(LIST_IS_EMPTY, &args![node]).u8() == 0 {
        let response = node_item(e, node);
        let part = e.call(RESPONSE_SAVE_SIZE, &args![response]).u16();
        size = size.wrapping_add(part);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    size.wrapping_add(1)
        .wrapping_add(4)
        .wrapping_add(4)
        .wrapping_add(4)
        .wrapping_add(4)
}

// Translated from 0083ca90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::SaveGame` (engine map; named `_ov1` here because [`dialogue_item_save_game`]
/// is the translation of `SaveGame_ov2`; `thiscall`; the older save path through the
/// `TESSaveLoadGame` at `011de45c`): a count byte is written (`008579b0`) and its place kept
/// (`00825c00`); every response until the list ends or an empty node is reached saves itself
/// (`0083d490`) and is counted; the count goes into its place. Then the index byte of the
/// current response ([`dialogue_item_get_dialogue_response_index`], `0xff` without one) and the
/// save ids (`0084e3a0`, 0 for none) of `pInfo`, `pTopic`, `pQuest` and `pSpeaker`
/// (`SaveNumericID`, 4 bytes each).
pub fn dialogue_item_save_game_ov1(e: &mut Engine, this: Ptr<DialogueItem>) {
    e.with_stack(0x10, |e, locals| {
        let count_at = locals.addr();
        let index_at = locals.addr() + 4;
        let id_at = locals.addr() + 8;
        e.mem.set_u8(count_at, 0);
        let saver = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
        let place = e.call(SAVE_NEXT_BYTE_00825C00, &args![saver]).u32();
        let saver = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
        e.call(SAVE_WRITE_008579B0, &args![saver, count_at, 1u32]);
        let mut node = this.addr();
        while node != 0 && e.call(LIST_IS_EMPTY, &args![node]).u8() == 0 {
            let response = node_item(e, node);
            e.call(RESPONSE_SAVE_GAME_OLD, &args![response]);
            let count = e.mem.u8(count_at).wrapping_add(1);
            e.mem.set_u8(count_at, count);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let count = e.mem.u8(count_at);
        e.mem.set_u8(place, count);
        e.mem.set_u8(index_at, 0xff);
        let current = e.get(this, DialogueItem::pCurrentResponse);
        if current != 0 {
            let response = node_item(e, current);
            let index = dialogue_item_get_dialogue_response_index(e, this, response);
            e.mem.set_u8(index_at, index as u8);
        }
        let saver = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
        e.call(SAVE_WRITE_008579B0, &args![saver, index_at, 1u32]);
        for field in [
            DialogueItem::pInfo,
            DialogueItem::pTopic,
            DialogueItem::pQuest,
            DialogueItem::pSpeaker,
        ] {
            e.mem.set_u32(id_at, 0);
            let form = e.get(this, field);
            if form != 0 {
                let id = e.call(FORM_SAVE_ID_0084E3A0, &args![form]).u32();
                e.mem.set_u32(id_at, id);
            }
            let saver = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
            e.call(SAVE_NUMERIC_ID_00857A10, &args![saver, id_at, 4u32]);
        }
    });
}

// Translated from 0083cc10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083cc10 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `DialogueItem::LoadGame` (engine map; `thiscall`; the older load path through the
/// `TESSaveLoadGame` at `011de45c`): reads a count byte and makes that many responses
/// (`operator new(0x2c)`, `0083d1e0`, null when the allocation fails), each loading itself
/// (`0083d550`) and added to the list (`00905820`); an index byte (-1: `pCurrentResponse`
/// cleared, otherwise sign-extended to [`dialogue_item_get_dialogue_response_by_index`] and
/// [`dialogue_item_set_current_response`]); then the numeric ids of `pInfo`, `pTopic` and
/// `pQuest`, each cast to its type when not 0 (a 0 leaves the field unchanged), and
/// `pSpeaker`'s word as it is read.
pub fn dialogue_item_load_game(e: &mut Engine, this: Ptr<DialogueItem>) {
    e.with_stack(0x10, |e, locals| {
        let count_at = locals.addr();
        let index_at = locals.addr() + 1;
        let response_at = locals.addr() + 4;
        let id_at = locals.addr() + 8;
        let loader = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
        e.call(LOAD_READ_008579E0, &args![loader, count_at, 1u32]);
        let mut index = 0u32;
        while index < u32::from(e.mem.u8(count_at)) {
            let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
            let response = if block != 0 {
                e.call(RESPONSE_DEFAULT_CONSTRUCTOR, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(response_at, response);
            e.call(RESPONSE_LOAD_GAME_OLD, &args![response]);
            e.call(LIST_ADD, &args![this, response_at]);
            index = index.wrapping_add(1);
        }
        let loader = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
        e.call(LOAD_READ_008579E0, &args![loader, index_at, 1u32]);
        let current = e.mem.u8(index_at) as i8;
        if current == -1 {
            e.set(this, DialogueItem::pCurrentResponse, 0);
        } else {
            let wide = u32::from(i16::from(current) as u16);
            let response = dialogue_item_get_dialogue_response_by_index(e, this, wide);
            dialogue_item_set_current_response(e, this, response);
        }
        for (field, target) in [
            (DialogueItem::pInfo, INFO_TYPE_01184738),
            (DialogueItem::pTopic, TOPIC_TYPE_01184720),
            (DialogueItem::pQuest, QUEST_TYPE_01186500),
        ] {
            let loader = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
            e.call(LOAD_NUMERIC_ID_00857AA0, &args![loader, id_at, 4u32]);
            let form = e.mem.u32(id_at);
            if form != 0 {
                let cast = cast_form(e, form, target);
                e.set(this, field, cast);
            }
        }
        let loader = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
        e.call(LOAD_NUMERIC_ID_00857AA0, &args![loader, id_at, 4u32]);
        let speaker = e.mem.u32(id_at);
        e.set(this, DialogueItem::pSpeaker, speaker);
    });
}

// Translated from 0083cdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): with a speaker, casts it (`004839c0`, `__RTDynamicCast` to the type
/// `011841cc`) and keeps the result in `pSpeaker`.
pub fn fn_0083cdf0(e: &mut Engine, this: Ptr<DialogueItem>) {
    let speaker = e.get(this, DialogueItem::pSpeaker);
    if speaker != 0 {
        let cast = cast_form(e, speaker, SPEAKER_TYPE_011841CC);
        e.set(this, DialogueItem::pSpeaker, cast);
    }
}

// Translated from 0083cf40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083cf40 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `DialogueItem::LoadGame_ov2` (engine map; `thiscall`, `RET 4`: the load buffer): a count
/// (`LoadVariableSizedValue`) of responses, each made (`operator new(0x2c)`, `0083d1e0`, null
/// when the allocation fails), loaded (`0083d740` with the buffer) and added to the list; a
/// 16-bit index (2 bytes into a local set to -1; when not -1,
/// [`dialogue_item_get_dialogue_response_by_index`] and [`dialogue_item_set_current_response`]);
/// `pInfo`, `pTopic` and `pQuest` from `LoadFormID`, each cast to its type; `pSpeaker` through
/// `LoadFormID_ov2`.
pub fn dialogue_item_load_game_ov2(e: &mut Engine, this: Ptr<DialogueItem>, buffer: u32) {
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    e.with_stack(8, |e, locals| {
        let response_at = locals.addr();
        let index_at = locals.addr() + 4;
        let mut index = 0u32;
        while index < count {
            let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
            let response = if block != 0 {
                e.call(RESPONSE_DEFAULT_CONSTRUCTOR, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(response_at, response);
            e.call(RESPONSE_LOAD_GAME, &args![response, buffer]);
            e.call(LIST_ADD, &args![this, response_at]);
            index = index.wrapping_add(1);
        }
        e.mem.set_u16(index_at, 0xffff);
        e.call(LOAD_DATA, &args![buffer, index_at, 2u32]);
        let current = e.mem.u16(index_at);
        if current as i16 != -1 {
            let response =
                dialogue_item_get_dialogue_response_by_index(e, this, u32::from(current));
            dialogue_item_set_current_response(e, this, response);
        }
    });
    for (field, target) in [
        (DialogueItem::pInfo, INFO_TYPE_01184738),
        (DialogueItem::pTopic, TOPIC_TYPE_01184720),
        (DialogueItem::pQuest, QUEST_TYPE_01186500),
    ] {
        let form = e.call(LOAD_FORM_ID, &args![buffer]).u32();
        let cast = cast_form(e, form, target);
        e.set(this, field, cast);
    }
    e.call(
        LOAD_FORM_ID_INTO,
        &args![buffer, this.addr() + DialogueItem::pSpeaker.off],
    );
}

// Translated from 0083d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueItem::InitLoadGame` (engine map; `thiscall`, `RET 4`: one word it does not read):
/// `pSpeaker` becomes its cast (`004839c0`, `__RTDynamicCast` to the type `011841cc`), or 0.
pub fn dialogue_item_init_load_game(e: &mut Engine, this: Ptr<DialogueItem>, _unused_1: u32) {
    let speaker = e.get(this, DialogueItem::pSpeaker);
    let cast = if speaker != 0 {
        cast_form(e, speaker, SPEAKER_TYPE_011841CC)
    } else {
        0
    };
    e.set(this, DialogueItem::pSpeaker, cast);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0083_c520,
            dialogue_item_constructor(Ptr<DialogueItem>, u32, u32, u32, u32) -> Ptr<DialogueItem>
        ),
        entry!(0x0083_c670, dialogue_item_destructor(Ptr<DialogueItem>)),
        entry!(0x0083_c720, fn_0083c720(Ptr, u32) -> Ptr),
        entry!(0x0083_c750, fn_0083c750(Ptr)),
        entry!(
            0x0083_c7b0,
            dialogue_item_first_response(Ptr<DialogueItem>) -> bool
        ),
        entry!(
            0x0083_c7e0,
            dialogue_item_next_response(Ptr<DialogueItem>) -> bool
        ),
        entry!(
            0x0083_c820,
            dialogue_item_get_current_response(Ptr<DialogueItem>) -> u32
        ),
        entry!(
            0x0083_c850,
            dialogue_item_run_result(Ptr<DialogueItem>, u32)
        ),
        entry!(
            0x0083_c900,
            dialogue_item_get_dialogue_response_index(Ptr<DialogueItem>, u32) -> u16
        ),
        entry!(0x0083_ce40, dialogue_item_save_game(Ptr<DialogueItem>, u32)),
        entry!(
            0x0083_c4d0,
            dialogue_item_default_constructor(Ptr<DialogueItem>) -> Ptr<DialogueItem>
        ),
        entry!(
            0x0083_c8a0,
            dialogue_item_set_current_response(Ptr<DialogueItem>, u32)
        ),
        entry!(
            0x0083_c970,
            dialogue_item_get_dialogue_response_by_index(Ptr<DialogueItem>, u32) -> u32
        ),
        entry!(0x0083_c9e0, fn_0083c9e0(Ptr<DialogueItem>) -> u16),
        entry!(0x0083_ca90, dialogue_item_save_game_ov1(Ptr<DialogueItem>)),
        entry!(0x0083_cc10, dialogue_item_load_game(Ptr<DialogueItem>)),
        entry!(0x0083_cdf0, fn_0083cdf0(Ptr<DialogueItem>)),
        entry!(
            0x0083_cf40,
            dialogue_item_load_game_ov2(Ptr<DialogueItem>, u32)
        ),
        entry!(
            0x0083_d0d0,
            dialogue_item_init_load_game(Ptr<DialogueItem>, u32)
        ),
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
    /// node, the next node is +4, a list is empty when its item and next are both 0.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
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

    #[test]
    fn constructor_makes_a_response_per_info_response() {
        let mut e = list_engine();
        e.register(LIST_CONSTRUCTOR, |_, _| Ret::default());
        e.register(ITERATOR_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(CLONE_RESPONSE_LIST, |e, a| {
            e.mem.set_u32(a[1], 0x0000_0100);
            Ret::default()
        });
        e.register(ITERATOR_NODE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NEXT_NODE_0045CD60, |_, a| {
            let next = if a[0] < 0x300 { a[0] + 0x100 } else { 0 };
            next.into_ret()
        });
        e.register(ITERATOR_DESTRUCTOR, |_, _| Ret::default());
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register(RESPONSE_CONSTRUCTOR, |_, a| (a[0] + a[5]).into_ret());
        e.register(LIST_ADD, |e, a| {
            assert_ne!(e.mem.u32(a[1]), 0);
            Ret::default()
        });
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 8, 0xee);
        assert_eq!(
            e.call(0x0083_c520, &[this, 0x14, 0x10, 0xc, 0x18]).u32(),
            this
        );
        for (offset, value) in [
            (8u32, 0u32),
            (0xc, 0xc),
            (0x10, 0x10),
            (0x14, 0x14),
            (0x18, 0x18),
        ] {
            assert_eq!(e.mem.u32(this + offset), value);
        }
        let made = called(&e, RESPONSE_CONSTRUCTOR);
        let nodes: Vec<u32> = made.iter().map(|w| w[5]).collect();
        assert_eq!(nodes, vec![0x100, 0x200, 0x300]);
        assert_eq!(made[0][1..5], [0x14, 0x10, 0xc, 0x18]);
        assert_eq!(called(&e, LIST_ADD).len(), 3);
        assert_eq!(called(&e, ITERATOR_DESTRUCTOR).len(), 1);

        // Without a quest only the list is made.
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x0083_c520, &[this, 0, 0x10, 0xc, 0x18]);
        assert!(called(&e, CLONE_RESPONSE_LIST).is_empty());
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn destructor_deletes_every_response() {
        let mut e = list_engine();
        e.register(LIST_REMOVE_HEAD, |e, a| {
            // Pop the head: copy the next node over it.
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
        e.register(LIST_DESTRUCTOR, |_, _| Ret::default());
        e.register(STRING_DESTRUCTOR, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        let a = e.mem.alloc(0x2c);
        let b = e.mem.alloc(0x2c);
        fill(&mut e, this, &[a, 0, b]);
        e.call(0x0083_c670, &[this]);
        let freed: Vec<u32> = called(&e, FREE).iter().map(|w| w[0]).collect();
        assert_eq!(freed, vec![a, b]);
        assert_eq!(called(&e, LIST_REMOVE_HEAD).len(), 3);
        assert_eq!(called(&e, LIST_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn response_destructors_destroy_both_strings() {
        let mut e = list_engine();
        e.register(STRING_DESTRUCTOR, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.call(0x0083_c750, &[0x1000]);
        assert_eq!(
            called(&e, STRING_DESTRUCTOR),
            vec![vec![0x1010], vec![0x1000]]
        );
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(0x0083_c720, &[0x1000, flags]).u32(), 0x1000);
        }
        assert_eq!(called(&e, STRING_DESTRUCTOR).len(), 8);
        assert_eq!(called(&e, FREE), vec![vec![0x1000], vec![0x1000]]);
    }

    #[test]
    fn first_next_and_current_walk_the_list() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x1c);
        fill(&mut e, this, &[0x11, 0x22]);
        assert!(e.call(0x0083_c7b0, &[this]).bool());
        assert_eq!(e.mem.u32(this + 8), this);
        assert_eq!(e.call(0x0083_c820, &[this]).u32(), 0x11);
        assert!(e.call(0x0083_c7e0, &[this]).bool());
        assert_eq!(e.call(0x0083_c820, &[this]).u32(), 0x22);
        assert!(!e.call(0x0083_c7e0, &[this]).bool());
        assert_eq!(e.call(0x0083_c820, &[this]).u32(), 0);
        assert!(!e.call(0x0083_c7e0, &[this]).bool());
        // An empty list's first node holds nothing.
        let empty = e.mem.alloc(0x1c);
        assert!(!e.call(0x0083_c7b0, &[empty]).bool());
    }

    #[test]
    fn run_result_needs_an_info_that_answers_false() {
        let mut e = list_engine();
        e.register(INFO_TEST_00579200, |_, a| {
            u32::from(a[0] == 0x2000).into_ret()
        });
        e.register(ADD_TOPIC_LIST, |_, _| Ret::default());
        e.register(RUN_RESULT, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, 0x18);
        e.call(0x0083_c850, &[this, 7]);
        assert!(called(&e, INFO_TEST_00579200).is_empty());
        e.mem.set_u32(this + 0xc, 0x2000);
        e.call(0x0083_c850, &[this, 7]);
        assert!(called(&e, RUN_RESULT).is_empty());
        e.mem.set_u32(this + 0xc, 0x1000);
        e.call(0x0083_c850, &[this, 7]);
        assert_eq!(called(&e, ADD_TOPIC_LIST), vec![vec![0x1000]]);
        assert_eq!(called(&e, RUN_RESULT), vec![vec![0x1000, 7, 0x18]]);
    }

    #[test]
    fn response_index_counts_or_reports() {
        let mut e = list_engine();
        e.register(REPORT_005B5E40, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        fill(&mut e, this, &[0x11, 0x22, 0x33]);
        assert_eq!(e.call(0x0083_c900, &[this, 0x33]).u16(), 2);
        assert_eq!(e.call(0x0083_c900, &[this, 0x11]).u16(), 0);
        assert!(called(&e, REPORT_005B5E40).is_empty());
        assert_eq!(e.call(0x0083_c900, &[this, 0x44]).u32(), 0);
        assert_eq!(
            called(&e, REPORT_005B5E40),
            vec![vec![MISSING_RESPONSE_MESSAGE]]
        );
    }

    #[test]
    fn save_game_writes_responses_index_and_forms() {
        let mut e = list_engine();
        e.register(START_VARIABLE_SIZED_VALUE, |_, _| 0x77u32.into_ret());
        e.register(RESPONSE_SAVE_GAME, |_, _| Ret::default());
        e.register(SAVE_VARIABLE_SIZED_VALUE, |_, _| Ret::default());
        e.register(SAVE_DATA, |e, a| {
            // Records the two bytes written.
            let written = u32::from(e.mem.u16(a[1]));
            e.mem.set_u32(0x0090_0000, written);
            Ret::default()
        });
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        e.map(0x0090_0000, 0x10);
        let this = e.mem.alloc(0x1c);
        fill(&mut e, this, &[0x11, 0, 0x33]);
        for (offset, value) in [(0xcu32, 0xc0u32), (0x10, 0x10), (0x14, 0x14), (0x18, 0x18)] {
            e.mem.set_u32(this + offset, value);
        }
        // The current response is the third node's.
        let second = e.mem.u32(this + 4);
        let third = e.mem.u32(second + 4);
        e.mem.set_u32(this + 8, third);
        e.call(0x0083_ce40, &[this, 0xb0]);
        let saved: Vec<u32> = called(&e, RESPONSE_SAVE_GAME)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(saved, vec![0x11, 0x33]);
        assert_eq!(
            called(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0xb0, 2, 0x77]]
        );
        assert_eq!(e.mem.u32(0x0090_0000), 2);
        let data = called(&e, SAVE_DATA);
        assert_eq!(data[0][2..], [2, 0]);
        let forms: Vec<u32> = called(&e, SAVE_FORM_ID).iter().map(|w| w[1]).collect();
        assert_eq!(forms, vec![0xc0, 0x10, 0x14, 0x18]);

        // No current response: -1.
        e.mem.set_u32(this + 8, 0);
        e.call(0x0083_ce40, &[this, 0xb0]);
        assert_eq!(e.mem.u32(0x0090_0000), 0xffff);
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
    /// neither is empty.
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
    }

    /// An item whose list holds `responses` (the first node inline at `+0`). Returns the item
    /// and the nodes.
    fn item_with(e: &mut Engine, responses: &[u32]) -> (u32, Vec<u32>) {
        let item = e.mem.alloc(0x1c);
        let mut nodes = vec![item];
        for _ in 1..responses.len() {
            nodes.push(e.mem.alloc(8));
        }
        for (k, response) in responses.iter().enumerate() {
            e.mem.set_u32(nodes[k], *response);
            let next = nodes.get(k + 1).copied().unwrap_or(0);
            e.mem.set_u32(nodes[k] + 4, next);
        }
        (item, nodes)
    }

    #[test]
    fn n089_0083c4d0_constructs_an_empty_item() {
        let mut e = Engine::new();
        e.register(LIST_CONSTRUCTOR, quiet);
        let this = e.mem.alloc(0x1c);
        e.mem.write(this, &[0xcc; 0x1c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_c4d0, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, LIST_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.bytes(this + 8, 0x14), vec![0; 0x14]);
        assert_eq!(e.mem.u32(this + 4), 0xcccc_cccc);
    }

    #[test]
    fn n089_0083c8a0_points_at_the_matching_node() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let (item, nodes) = item_with(&mut e, &[0x10, 0x20, 0x30]);
        e.call(0x0083_c8a0, &args![item, 0x30u32]);
        assert_eq!(e.mem.u32(item + 8), nodes[2]);
        e.mem.set_u32(item + 8, 0x77);
        e.call(0x0083_c8a0, &args![item, 0x40u32]);
        assert_eq!(e.mem.u32(item + 8), 0x77, "not found: unchanged");
    }

    #[test]
    fn n089_0083c970_finds_by_index_or_reports() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(REPORT_005B5E40, quiet);
        let (item, _) = item_with(&mut e, &[0x10, 0x20]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0083_c970, &args![item, 0x0001_0001u32]).u32(),
            0x20
        );
        assert_eq!(e.call(0x0083_c970, &args![item, 0u32]).u32(), 0x10);
        assert!(calls_to(&e, REPORT_005B5E40).is_empty());
        assert_eq!(e.call(0x0083_c970, &args![item, 2u32]).u32(), 0);
        assert_eq!(e.call(0x0083_c970, &args![item, 0xffffu32]).u32(), 0);
        assert_eq!(
            calls_to(&e, REPORT_005B5E40),
            vec![vec![INDEX_TOO_LARGE_MESSAGE]; 2]
        );
    }

    #[test]
    fn n089_0083c9e0_sums_the_save_size() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(RESPONSE_SAVE_SIZE, |_, a| (0xabcd_0000 + a[0]).into_ret());
        let (item, _) = item_with(&mut e, &[0x10, 0x20]);
        assert_eq!(
            e.call(0x0083_c9e0, &args![item]).u16(),
            1 + 0x10 + 0x20 + 1 + 16
        );
        // 16-bit wraparound.
        e.register(RESPONSE_SAVE_SIZE, |_, _| 0xfff0u32.into_ret());
        assert_eq!(
            e.call(0x0083_c9e0, &args![item]).u16(),
            1u16.wrapping_add(0xfff0)
                .wrapping_add(0xfff0)
                .wrapping_add(17)
        );
    }

    /// The save/load holder and a byte place for `00825c00`; the writes append the bytes and
    /// words they are given to a log at the returned address (`+0` the count, then the values).
    fn save_doubles(e: &mut Engine) -> (u32, u32) {
        e.map(SAVE_LOAD_GAME_011DE45C, 4);
        e.set_global(SAVE_LOAD_GAME_011DE45C, 0x5a5a_0000u32);
        let place = e.mem.alloc(4);
        e.register_double(SAVE_NEXT_BYTE_00825C00, move |_, _| place.into_ret());
        let written = e.mem.alloc(0x40);
        let record = move |e: &mut Engine, a: &[u32]| {
            let count = e.mem.u32(written);
            let value = if a[2] == 1 {
                u32::from(e.mem.u8(a[1]))
            } else {
                e.mem.u32(a[1])
            };
            e.mem.set_u32(written + 4 + count * 4, value);
            e.mem.set_u32(written, count + 1);
            Ret::default()
        };
        e.register_double(SAVE_WRITE_008579B0, record);
        e.register_double(SAVE_NUMERIC_ID_00857A10, record);
        (place, written)
    }

    fn written_values(e: &Engine, written: u32) -> Vec<u32> {
        (0..e.mem.u32(written))
            .map(|k| e.mem.u32(written + 4 + k * 4))
            .collect()
    }

    #[test]
    fn n089_0083ca90_saves_the_item_the_old_way() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let (place, written) = save_doubles(&mut e);
        e.register(RESPONSE_SAVE_GAME_OLD, quiet);
        e.register(FORM_SAVE_ID_0084E3A0, |_, a| (a[0] + 1).into_ret());
        let (item, nodes) = item_with(&mut e, &[0x10, 0x20, 0x30]);
        e.mem.set_u32(item + 8, nodes[1]);
        e.mem.set_u32(item + 0xc, 0x1100);
        e.mem.set_u32(item + 0x14, 0x3300);
        e.call_log = Some(vec![]);
        e.call(0x0083_ca90, &args![item]);
        assert_eq!(e.mem.u8(place), 3);
        assert_eq!(
            calls_to(&e, RESPONSE_SAVE_GAME_OLD),
            vec![vec![0x10], vec![0x20], vec![0x30]]
        );
        // The count placeholder 0, the index 1, the ids.
        assert_eq!(
            written_values(&e, written),
            vec![0, 1, 0x1101, 0, 0x3301, 0]
        );
        assert_eq!(calls_to(&e, SAVE_WRITE_008579B0)[0][0], 0x5a5a_0000);
        assert_eq!(
            calls_to(&e, FORM_SAVE_ID_0084E3A0),
            vec![vec![0x1100], vec![0x3300]]
        );
        // Without a current response the index is 0xff.
        e.mem.set_u32(written, 0);
        e.mem.set_u32(item + 8, 0);
        e.call(0x0083_ca90, &args![item]);
        assert_eq!(written_values(&e, written)[1], 0xff);
    }

    #[test]
    fn n089_0083cc10_loads_the_item_the_old_way() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.map(SAVE_LOAD_GAME_011DE45C, 4);
        // Reads: count 2, index 1, then the ids 0x11, 0, 0x33 and the speaker 0x44.
        let stream = e.mem.alloc(0x20);
        for (k, word) in [2u32, 1, 0x11, 0, 0x33, 0x44].iter().enumerate() {
            e.mem.set_u32(stream + 4 + 4 * k as u32, *word);
        }
        let reader = move |e: &mut Engine, a: &[u32]| {
            let cursor = e.mem.u32(stream);
            let word = e.mem.u32(stream + 4 + cursor * 4);
            e.mem.set_u32(stream, cursor + 1);
            if a[2] == 1 {
                e.mem.set_u8(a[1], word as u8);
            } else {
                e.mem.set_u32(a[1], word);
            }
            Ret::default()
        };
        e.register_double(LOAD_READ_008579E0, reader);
        e.register_double(LOAD_NUMERIC_ID_00857AA0, reader);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(RESPONSE_DEFAULT_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(RESPONSE_LOAD_GAME_OLD, quiet);
        // The list add appends the response to the inline list.
        e.register(LIST_ADD, |e, a| {
            let response = e.mem.u32(a[1]);
            let mut node = a[0];
            if e.mem.u32(node) == 0 {
                e.mem.set_u32(node, response);
                return Ret::default();
            }
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let fresh = e.mem.alloc(8);
            e.mem.set_u32(fresh, response);
            e.mem.set_u32(node + 4, fresh);
            Ret::default()
        });
        e.register(CAST_SOURCE_004839C0, |_, a| (a[0] + 0x100).into_ret());
        e.register(DYNAMIC_CAST_00EC43FB, |_, a| (a[0] + a[3]).into_ret());
        let item = e.mem.alloc(0x1c);
        e.mem.set_u32(item + 0x10, 0x7777);
        e.call_log = Some(vec![]);
        e.call(0x0083_cc10, &args![item]);
        let responses = calls_to(&e, RESPONSE_LOAD_GAME_OLD);
        assert_eq!(responses.len(), 2);
        let second_node = e.mem.u32(item + 4);
        assert_eq!(e.mem.u32(second_node), responses[1][0]);
        assert_eq!(
            e.mem.u32(item + 8),
            second_node,
            "the current response is index 1"
        );
        assert_eq!(e.mem.u32(item + 0xc), 0x111 + INFO_TYPE_01184738);
        assert_eq!(
            e.mem.u32(item + 0x10),
            0x7777,
            "an id of 0 leaves the field"
        );
        assert_eq!(e.mem.u32(item + 0x14), 0x133 + QUEST_TYPE_01186500);
        assert_eq!(e.mem.u32(item + 0x18), 0x44);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST_00EC43FB)[0],
            vec![0x111, 0, FORM_TYPE_01183028, INFO_TYPE_01184738, 0]
        );
        // No responses and index -1: the current response is cleared.
        e.mem.set_u32(stream, 0);
        e.mem.set_u32(stream + 4, 0);
        e.mem.set_u32(stream + 8, 0xff);
        e.call(0x0083_cc10, &args![item]);
        assert_eq!(e.mem.u32(item + 8), 0);
    }

    #[test]
    fn n089_0083cdf0_casts_the_speaker() {
        let mut e = Engine::new();
        e.register(CAST_SOURCE_004839C0, |_, a| (a[0] + 0x100).into_ret());
        e.register(DYNAMIC_CAST_00EC43FB, |_, a| (a[0] + 1).into_ret());
        let item = e.mem.alloc(0x1c);
        e.mem.set_u32(item + 0x18, 0x500);
        e.call_log = Some(vec![]);
        e.call(0x0083_cdf0, &args![item]);
        assert_eq!(e.mem.u32(item + 0x18), 0x601);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST_00EC43FB),
            vec![vec![0x600, 0, FORM_TYPE_01183028, SPEAKER_TYPE_011841CC, 0]]
        );
        e.mem.set_u32(item + 0x18, 0);
        e.call_log = Some(vec![]);
        e.call(0x0083_cdf0, &args![item]);
        assert!(calls_to(&e, CAST_SOURCE_004839C0).is_empty());
        assert_eq!(e.mem.u32(item + 0x18), 0);
    }

    #[test]
    fn n089_0083cf40_loads_the_item() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(LOAD_VARIABLE_SIZED_VALUE, |_, _| 2u32.into_ret());
        e.register(ALLOCATE, |_, a| (a[0] << 8).into_ret());
        e.register(RESPONSE_DEFAULT_CONSTRUCTOR, |_, a| (a[0] + 1).into_ret());
        e.register(RESPONSE_LOAD_GAME, quiet);
        e.register(LIST_ADD, quiet);
        e.register(LOAD_DATA, |e, a| {
            e.mem.set_u16(a[1], 1);
            Ret::default()
        });
        e.register(LOAD_FORM_ID, |_, _| 0x900u32.into_ret());
        e.register(LOAD_FORM_ID_INTO, quiet);
        e.register(CAST_SOURCE_004839C0, |_, a| (a[0] + 0x100).into_ret());
        e.register(DYNAMIC_CAST_00EC43FB, |_, a| (a[0] + a[3]).into_ret());
        let (item, nodes) = item_with(&mut e, &[0x10, 0x20]);
        e.call_log = Some(vec![]);
        e.call(0x0083_cf40, &args![item, 0xb0ffu32]);
        assert_eq!(
            calls_to(&e, RESPONSE_LOAD_GAME),
            vec![vec![0x2c01, 0xb0ff]; 2]
        );
        let adds = calls_to(&e, LIST_ADD);
        assert_eq!(adds.len(), 2);
        assert_eq!(adds[0][0], item);
        assert_eq!(e.mem.u32(item + 8), nodes[1]);
        assert_eq!(e.mem.u32(item + 0xc), 0xa00 + INFO_TYPE_01184738);
        assert_eq!(e.mem.u32(item + 0x10), 0xa00 + TOPIC_TYPE_01184720);
        assert_eq!(e.mem.u32(item + 0x14), 0xa00 + QUEST_TYPE_01186500);
        assert_eq!(
            calls_to(&e, LOAD_FORM_ID_INTO),
            vec![vec![0xb0ff, item + 0x18]]
        );
        assert_eq!(calls_to(&e, LOAD_DATA)[0][2], 2);
        // A failed allocation still loads (on null) and adds; index -1 keeps the current.
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.register(LOAD_DATA, quiet);
        e.call_log = Some(vec![]);
        e.call(0x0083_cf40, &args![item, 0xb0ffu32]);
        assert_eq!(calls_to(&e, RESPONSE_LOAD_GAME)[0], vec![0, 0xb0ff]);
        assert!(calls_to(&e, RESPONSE_DEFAULT_CONSTRUCTOR).is_empty());
        assert_eq!(e.mem.u32(item + 8), nodes[1]);
    }

    #[test]
    fn n089_0083d0d0_casts_or_clears_the_speaker() {
        let mut e = Engine::new();
        e.register(CAST_SOURCE_004839C0, |_, a| (a[0] + 0x100).into_ret());
        e.register(DYNAMIC_CAST_00EC43FB, |_, _| 0u32.into_ret());
        let item = e.mem.alloc(0x1c);
        e.mem.set_u32(item + 0x18, 0x500);
        e.call_log = Some(vec![]);
        e.call(0x0083_d0d0, &args![item, 0x1234u32]);
        assert_eq!(e.mem.u32(item + 0x18), 0);
        assert_eq!(calls_to(&e, CAST_SOURCE_004839C0), vec![vec![0x500]]);
        e.register(DYNAMIC_CAST_00EC43FB, |_, a| (a[0] + 2).into_ret());
        e.mem.set_u32(item + 0x18, 0x500);
        e.call(0x0083_d0d0, &args![item, 0u32]);
        assert_eq!(e.mem.u32(item + 0x18), 0x602);
    }
}

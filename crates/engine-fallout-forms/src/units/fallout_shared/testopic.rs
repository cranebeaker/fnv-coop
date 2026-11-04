//! `fallout shared/testopic.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Finding topics: a topic of a kind by index (`TESTopic::GetTopic`), a
//! topic info by form id in a topic's per-quest info lists (`00619e10`),
//! and the topic that holds an info (`TESTopic::GetOwnerTopic`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The topic counts per kind (`0119a41c`, one signed word per kind).
const TOPIC_COUNTS: u32 = 0x0119_a41c;
/// The topic arrays per kind (`0119a43c`, one word per kind: the address of
/// 0xc bytes per topic, whose first word `GetTopic` returns).
const TOPIC_ARRAYS: u32 = 0x0119_a43c;
const TOPIC_ENTRY_SIZE: u32 = 0xc;
/// The word at `011c3f2c`: the object whose list (`004612e0`) holds the
/// topics.
const TOPIC_LIST_OWNER: u32 = 0x011c_3f2c;
/// `004612e0` (`ECX` = object): `this + 0x108` (the topic list's first
/// node).
const LIST_AT_0X108: u32 = 0x0046_12e0;
/// `005d8a70` (`ECX` = topic): `this + 0x2c` (the list of the topic's
/// per-quest info lists).
const TOPIC_QUEST_LIST: u32 = 0x005d_8a70;
/// `006815c0` (`ECX` = node): the node itself (its first word is the item).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = node): the word at +4 (the next node).
const NODE_NEXT: u32 = 0x0072_6070;
/// `00717e50` (`ECX` = object): `this + 4` (in a per-quest entry, its info
/// array).
const PLUS_FOUR: u32 = 0x0071_7e50;
/// `0084e3a0` (`ECX` = object): the word at +0xc (an array's count; a
/// form's id).
const WORD_AT_0XC: u32 = 0x0084_e3a0;
/// `00619f20` (cdecl: per-quest entry, index): the info at that index.
const ENTRY_INFO: u32 = 0x0061_9f20;

// Translated from 0061a2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopic::GetTopic` (Xbox PDB; cdecl: kind, index): the first word of
/// entry `index` of the kind's topic array (`0119a43c`), or 0 for a
/// negative index or one not below the kind's count (`0119a41c`; both
/// compared signed).
pub fn tes_topic_get_topic(e: &mut Engine, kind: u32, index: i32) -> Ptr {
    if index < 0 {
        return Ptr::NULL;
    }
    let count = e.mem.i32(TOPIC_COUNTS.wrapping_add(kind.wrapping_mul(4)));
    if index >= count {
        return Ptr::NULL;
    }
    let array = e.mem.u32(TOPIC_ARRAYS.wrapping_add(kind.wrapping_mul(4)));
    Ptr::new(
        e.mem.u32(
            (index as u32)
                .wrapping_mul(TOPIC_ENTRY_SIZE)
                .wrapping_add(array),
        ),
    )
}

// Translated from 00619e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The topic info with form id `id` in the topic's (`ECX`) per-quest info
/// lists (two stack words, the second a byte; `RET 8`). The list
/// (`005d8a70`) is walked node by node until a node with no item; for each
/// entry with a nonzero info count (`0084e3a0` of `00717e50`), the infos
/// (`00619f20`) are searched from the last down to index 0 (signed) when
/// `last_first` is nonzero, else from the first up (unsigned); the first
/// non-null info whose id (`0084e3a0`) is `id` is returned. 0 if none. The
/// engine map gives it no name.
pub fn fn_00619e10(e: &mut Engine, this: Ptr, id: u32, last_first: u8) -> Ptr {
    let mut node = e.call(TOPIC_QUEST_LIST, &args![this]).u32();
    loop {
        if node == 0 {
            return Ptr::NULL;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            return Ptr::NULL;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(item);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let infos = e.call(PLUS_FOUR, &args![entry]).u32();
        let count = e.call(WORD_AT_0XC, &args![infos]).u32();
        if count == 0 {
            continue;
        }
        if last_first != 0 {
            let mut index = count.wrapping_sub(1) as i32;
            while index >= 0 {
                let info = e.call(ENTRY_INFO, &args![entry, index]).u32();
                if info != 0 && e.call(WORD_AT_0XC, &args![info]).u32() == id {
                    return Ptr::new(info);
                }
                index -= 1;
            }
        } else {
            let mut index = 0u32;
            while index < count {
                let info = e.call(ENTRY_INFO, &args![entry, index]).u32();
                if info != 0 && e.call(WORD_AT_0XC, &args![info]).u32() == id {
                    return Ptr::new(info);
                }
                index += 1;
            }
        }
    }
}

// Translated from 0061a310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopic::GetOwnerTopic` (Xbox PDB; cdecl: the topic info): walks the
/// topic list (`004612e0` of the object at `011c3f2c`) node by node until a
/// node with no item, and returns the first topic whose info lists hold an
/// info with this info's id (`0084e3a0`; `00619e10`, first to last) that is
/// this very info. 0 if none.
pub fn tes_topic_get_owner_topic(e: &mut Engine, info: Ptr) -> Ptr {
    let owner = e.mem.u32(TOPIC_LIST_OWNER);
    let mut node = e.call(LIST_AT_0X108, &args![owner]).u32();
    loop {
        if node == 0 {
            return Ptr::NULL;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            return Ptr::NULL;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let topic = e.mem.u32(item);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let id = e.call(WORD_AT_0XC, &args![info]).u32();
        if fn_00619e10(e, Ptr::new(topic), id, 0).addr() == info.addr() {
            return Ptr::new(topic);
        }
    }
}

/// `008256d0` (`ECX` = node): true when the node holds no item and has no
/// next node (an empty list).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `0063f7b0` (`ECX` = node): removes the node's item (the next node moves
/// into it).
const NODE_REMOVE: u32 = 0x0063_f7b0;
/// `005ae3d0` (`ECX` = node, the address of an item): inserts the item at
/// the node (before what it holds).
const NODE_INSERT: u32 = 0x005a_e3d0;
/// `00905820` (`ECX` = node, the address of an item): adds the item after
/// the node.
const NODE_APPEND: u32 = 0x0090_5820;
/// `00905330` (`ECX` = node, the address of an item): removes the item
/// from the nodes after this one.
const NODE_REMOVE_AFTER: u32 = 0x0090_5330;
/// `0096a2d0` (`ECX` = list): constructs an empty `BSSimpleList`.
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `0046ffb0` (`ECX` = list): destroys a `BSSimpleList`.
const LIST_DESTROY: u32 = 0x0046_ffb0;
/// `00877a30` (`ECX` = array, an index): the address of that element.
const ELEMENT_ADDRESS: u32 = 0x0087_7a30;
/// `0044ddc0` (`ECX` = array): its count (the `BSSimpleArray` at +0x1C of
/// a per-quest entry).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// `006a7af0` (`ECX` = array, an index): the address of that record (0xC
/// bytes: info form id, info, position).
const RECORD_ADDRESS: u32 = 0x006a_7af0;
/// `0061bf70` (`ECX` = array, an index, a flag): removes that record.
const RECORD_REMOVE: u32 = 0x0061_bf70;
/// `0061bc10` (`ECX` = info array, a position, an info): inserts the info.
const INFO_INSERT: u32 = 0x0061_bc10;
/// `0061f340` (`ECX` = info, a 16-bit index): stores the info's index.
const INFO_SET_INDEX: u32 = 0x0061_f340;
/// `00619a70` (`ECX` = info): the info's 16-bit index (0xFFFF for none).
const INFO_INDEX: u32 = 0x0061_9a70;
/// `__RTDynamicCast` (`00ec43fb`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptors `.?AVTESForm@@`, `.?AVTESTopicInfo@@`,
/// `.?AVTESQuest@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_TOPIC_INFO: u32 = 0x0118_4738;
const TYPE_TES_QUEST: u32 = 0x0118_6500;
/// `004839c0` (`cdecl`, a form id): the loaded form with that id, or 0.
const FORM_BY_ID: u32 = 0x0048_39c0;
/// `005b5e40` (`cdecl` format, ...): the log print.
const PRINT: u32 = 0x005b_5e40;
/// `00703c00` (`cdecl` format, ...): the console print.
const CONSOLE_PRINT: u32 = 0x0070_3c00;
/// The global `TESTopic::Load` leaves the last loaded topic in.
const LAST_LOADED_TOPIC: u32 = 0x011c_beb0;
/// The player pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// `"QUESTS: Could not find previous info (%08X) for TopicInfo (%08X) in
/// Topic \"%s\" (%08X)."`
const MISSING_PREVIOUS_INFO: u32 = 0x0104_d340;
/// `"MASTERFILE: No Quest Reference on Topic \"%s\" (%d non-deleted infos
/// attached to this quest)."`
const NO_QUEST_REFERENCE: u32 = 0x0104_d2d8;
/// `"------NEW CONVERSATION CREATED----------"`, `"Item %d: %s says '%s'"`,
/// `"Item %d: %s has no response!"`.
const CONVERSATION_HEADER: u32 = 0x0104_d410;
const CONVERSATION_LINE: u32 = 0x0104_d3f8;
const CONVERSATION_SILENT: u32 = 0x0104_d3d8;

/// `*006815c0(node)`.
fn item_of(e: &mut Engine, node: u32) -> u32 {
    let at = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(at)
}

/// `__RTDynamicCast(004839c0(id), 0, TESForm, target, 0)`.
fn form_as(e: &mut Engine, id: u32, target: u32) -> u32 {
    let form = e.call(FORM_BY_ID, &args![id]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .u32()
}

// Translated from 00619410 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 0x2 of the signed byte at +0x25 is set.
pub fn fn_00619410(e: &mut Engine, this: Ptr) -> bool {
    i32::from(e.mem.i8(this.addr() + 0x25)) & 2 != 0
}

// Translated from 0061aed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 0x20 of the signed byte at +0x25 is set.
pub fn fn_0061aed0(e: &mut Engine, this: Ptr) -> bool {
    i32::from(e.mem.i8(this.addr() + 0x25)) & 0x20 != 0
}

// Translated from 0061aef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 0x1 of the signed byte at +0x26 is set.
pub fn fn_0061aef0(e: &mut Engine, this: Ptr) -> bool {
    i32::from(e.mem.i8(this.addr() + 0x26)) & 1 != 0
}

// Translated from 0061b9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The topic's kind byte (+0x24), sign-extended.
pub fn fn_0061b9f0(e: &mut Engine, this: Ptr) -> i32 {
    i32::from(e.mem.i8(this.addr() + 0x24))
}

// Translated from 0061bef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the array the vtable `0104d450` belongs to (RTTI
/// `NiTLargePrimitiveArray<..>`): the base (`0061c120` with `size` and
/// `grow`), then the vtable. Returns `this`.
pub fn fn_0061bef0(e: &mut Engine, this: Ptr, size: u32, grow: u32) -> Ptr {
    e.call(0x0061_c120, &args![this, size, grow]);
    e.mem.set_u32(this.addr(), 0x0104_d450);
    this
}

// Translated from 006197c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TopicInfoArray`'s constructor (vtable `0104d338`, RTTI
/// `TopicInfoArray`): the base array (`0061bef0` with 0, 1), then the
/// vtable. Returns `this`.
pub fn fn_006197c0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0061bef0(e, this, 0, 1);
    e.mem.set_u32(this.addr(), 0x0104_d338);
    this
}

// Translated from 0061bf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray<INFO_LINK_ELEMENT>` (vtable
/// `0104d458`): the vtable, then `006b3f40` with 0, 0. Returns `this`.
pub fn fn_0061bf20(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0104_d458);
    e.call(0x006b_3f40, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00619740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00619740 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// A topic's per-quest entry constructor (0x34 bytes): the info array at +4
/// (`006197c0`), the link array at +0x1C (`0061bf20`), no quest (+0, the
/// quest or its form id), nothing at +0x2C and not resolved (+0x30).
/// Returns `this`.
pub fn fn_00619740(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    fn_006197c0(e, this.byte_add(4));
    fn_0061bf20(e, this.byte_add(0x1c));
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 0x2c, 0);
    e.mem.set_u8(base + 0x30, 0);
    this
}

// Translated from 006195d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006195d0 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// The topic's per-quest entry for the quest form id `quest` (two stack
/// words, `RET 8`; the second is never read), made when missing; 0 for a
/// zero `quest`. The entry list (`005d8a70`) is walked while the node holds
/// an entry and none was kept: a resolved entry (byte +0x30 set) is kept
/// when its quest (+0) or its +0x2C form has the id `quest` (`0084e3a0`,
/// each only when not null); an unresolved one when either word equals
/// `quest`. Without one, a new entry (0x34 bytes, `00619740`; null when the
/// allocation fails) gets `quest` at +0 and is appended to the list
/// (`00905820`).
pub fn fn_006195d0(e: &mut Engine, this: Ptr, quest: u32, _unused_2: u32) -> u32 {
    if quest == 0 {
        return 0;
    }
    let mut node = e.call(TOPIC_QUEST_LIST, &args![this]).u32();
    let mut found = 0u32;
    while node != 0 && item_of(e, node) != 0 && found == 0 {
        found = item_of(e, node);
        node = e.call(NODE_NEXT, &args![node]).u32();
        if e.mem.u8(found + 0x30) != 0 {
            let quest_form = e.mem.u32(found);
            if quest_form != 0 && e.call(WORD_AT_0XC, &args![quest_form]).u32() == quest {
                continue;
            }
            let other = e.mem.u32(found + 0x2c);
            if other != 0 && e.call(WORD_AT_0XC, &args![other]).u32() == quest {
                continue;
            }
            found = 0;
        } else if found != 0 && e.mem.u32(found) != quest && e.mem.u32(found + 0x2c) != quest {
            found = 0;
        }
    }
    if found != 0 {
        return found;
    }
    let block = e.call(0x0040_1000, &args![0x34u32]).u32();
    let entry = if block != 0 {
        fn_00619740(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.mem.set_u32(entry, quest);
    e.with_stack(4, |e, entry_at| {
        e.mem.set_u32(entry_at.addr(), entry);
        let list = e.call(TOPIC_QUEST_LIST, &args![this]).u32();
        e.call(NODE_APPEND, &args![list, entry_at]);
    });
    entry
}

// Translated from 00619820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopic::InsertInfo` (Xbox PDB): puts `info` into the info array of
/// the topic's entry for `quest` (`006195d0`) after the info with form id
/// `previous`; false (nothing done) for a zero `quest` or a null `info`.
/// - `previous` 0: at the start (`0061bc10` with 0).
/// - `previous` -1: when the info has no index yet (`00619a70` is 0xFFFF),
///   after the last non-null info (counting down from the count - 1 while
///   the index is above 0 and `00619f20` gives null), storing that
///   position as the info's index (`0061f340`) and putting it there
///   (`00864240` with the address of a stack copy of `info`).
/// - otherwise: an info that already has an index is first taken out
///   (`0061bd20` with its index and a zeroed stack word); then the previous
///   info is looked up (`00619e10`, last first). Found: after it (its
///   index + 1, or, without one, one past its position in the array, -1
///   when it is not there). Not found: a "QUESTS: Could not find previous
///   info ..." message (with the last loaded topic `011cbeb0`) and at the
///   start.
///
/// Then `0061bdc0` on the array and `00619fe0` (re-indexing). Returns true.
pub fn tes_topic_insert_info(
    e: &mut Engine,
    this: Ptr,
    quest: u32,
    info: Ptr,
    previous: u32,
) -> bool {
    if quest == 0 || info.is_null() {
        return false;
    }
    let entry = fn_006195d0(e, this, quest, 0);
    let array = |e: &mut Engine| e.call(PLUS_FOUR, &args![entry]).u32();
    if previous == 0 {
        let infos = array(e);
        e.call(INFO_INSERT, &args![infos, 0u32, info]);
    } else {
        if previous != u32::MAX && e.call(INFO_INDEX, &args![info]).u16() != 0xffff {
            e.with_stack(4, |e, out| {
                let index = e.call(INFO_INDEX, &args![info]).u16();
                let infos = array(e);
                e.call(0x0061_bd20, &args![infos, u32::from(index), out]);
            });
        }
        if previous != u32::MAX {
            let found = fn_00619e10(e, this, previous, 1).addr();
            if found != 0 && previous != 0 {
                let mut position = u32::MAX;
                if e.call(INFO_INDEX, &args![found]).u16() == 0xffff {
                    let mut index = 0u32;
                    loop {
                        let infos = array(e);
                        let count = e.call(WORD_AT_0XC, &args![infos]).u32();
                        if index >= count {
                            break;
                        }
                        let infos = array(e);
                        let at = e.call(ELEMENT_ADDRESS, &args![infos, index]).u32();
                        if e.mem.u32(at) == found {
                            position = index + 1;
                            break;
                        }
                        index += 1;
                    }
                } else {
                    let index = e.call(INFO_INDEX, &args![found]).u16();
                    position = u32::from(index) + 1;
                }
                let infos = array(e);
                e.call(INFO_INSERT, &args![infos, position, info]);
            } else {
                let topic: u32 = e.global(LAST_LOADED_TOPIC);
                let topic_id = e.call(WORD_AT_0XC, &args![topic]).u32();
                let topic: u32 = e.global(LAST_LOADED_TOPIC);
                let name = e.vcall(topic, 0x130, &[]).u32();
                let info_id = e.call(WORD_AT_0XC, &args![info]).u32();
                e.call(
                    PRINT,
                    &args![MISSING_PREVIOUS_INFO, previous, info_id, name, topic_id],
                );
                let infos = array(e);
                e.call(INFO_INSERT, &args![infos, 0u32, info]);
            }
        } else if e.call(INFO_INDEX, &args![info]).u16() == 0xffff {
            let infos = array(e);
            let mut last = e.call(WORD_AT_0XC, &args![infos]).u32().wrapping_sub(1) as i32;
            while last > 0 && e.call(ENTRY_INFO, &args![entry, last]).u32() == 0 {
                last -= 1;
            }
            last += 1;
            e.call(INFO_SET_INDEX, &args![info, (last as u32) & 0xffff]);
            e.with_stack(4, |e, info_at| {
                e.mem.set_u32(info_at.addr(), info.addr());
                let infos = array(e);
                e.call(0x0086_4240, &args![infos, last, info_at]);
            });
        }
    }
    let infos = array(e);
    e.call(0x0061_bdc0, &args![infos]);
    let infos = array(e);
    e.call(0x0061_9fe0, &args![infos]);
    true
}

// Translated from 00619030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopic::InitItem` (Xbox PDB): unless form flag 0x8 is set, for each
/// per-quest entry (the list walked while the node holds an item; null
/// entries skipped):
/// - an unresolved entry (+0x30 clear): each link record of its array at
///   +0x1C (`0044ddc0` count, `006a7af0` record, signed index) gets its
///   info (`TESTopicInfo` cast of the record's form id) at +4; a found info
///   already in the entry's info array, or a missing one (after a
///   `TESQuest` cast of the entry's quest id whose result is not used),
///   removes the record (`0061bf70` with 1). Then the quest: an id at +0
///   becomes the `TESQuest` (`004839c0` and the cast, no compile index); else
///   an id at +0x2C gets the compile index of the topic's file and becomes
///   the quest; else, when infos that `00440d80` says are not deleted are
///   attached, the "MASTERFILE: No Quest Reference on Topic ..." message.
///   The entry is marked resolved.
/// - its info array (`00717e50`, when not null): `0061bdc0`, `00864280`
///   with its count, and each non-null info gets the entry's quest
///   (`00673400`), its `InitItem` (vtable slot 0x88) and its index
///   (`0061f340`).
///
/// Finally, a topic with bit 0x2 of +0x25 (`00619410`) is given to the
/// player (`00952830` on `011dea3c` with the topic, 0, 0), and form flag
/// 0x8 is set.
pub fn tes_topic_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let mut node = e.call(TOPIC_QUEST_LIST, &args![this]).u32();
    while node != 0 && item_of(e, node) != 0 {
        let entry = item_of(e, node);
        if entry != 0 {
            if e.mem.u8(entry + 0x30) == 0 {
                resolve_entry(e, this, entry);
            }
            let infos = e.call(PLUS_FOUR, &args![entry]).u32();
            if infos != 0 {
                e.call(0x0061_bdc0, &args![infos]);
                let count = e.call(WORD_AT_0XC, &args![infos]).u32();
                e.call(0x0086_4280, &args![infos, count]);
                let count = e.call(WORD_AT_0XC, &args![infos]).u32();
                let mut index = 0u32;
                while index < count {
                    let at = e.call(ELEMENT_ADDRESS, &args![infos, index]).u32();
                    let info = e.mem.u32(at);
                    if info != 0 {
                        let quest = e.mem.u32(entry);
                        e.call(0x0067_3400, &args![info, quest]);
                        e.vcall(info, 0x88, &[]);
                        e.call(INFO_SET_INDEX, &args![info, index & 0xffff]);
                    }
                    index += 1;
                }
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if fn_00619410(e, this) {
        let player: u32 = e.global(PLAYER);
        e.call(0x0095_2830, &args![player, this, 0u32, 0u32]);
    }
    e.call(0x0048_4ab0, &args![this, true]);
}

/// The unresolved-entry part of `TESTopic::InitItem`.
fn resolve_entry(e: &mut Engine, this: Ptr, entry: u32) {
    let links = entry + 0x1c;
    let mut count = e.call(ARRAY_COUNT, &args![links]).u32() as i32;
    let mut index = 0i32;
    while index < count {
        let record = e.call(RECORD_ADDRESS, &args![links, index]).u32();
        let id = e.mem.u32(record);
        let info = form_as(e, id, TYPE_TES_TOPIC_INFO);
        let record = e.call(RECORD_ADDRESS, &args![links, index]).u32();
        e.mem.set_u32(record + 4, info);
        let remove = if info != 0 {
            let mut present = false;
            let mut k = 0u32;
            loop {
                let infos = e.call(PLUS_FOUR, &args![entry]).u32();
                let total = e.call(WORD_AT_0XC, &args![infos]).u32();
                if k >= total {
                    break;
                }
                let infos = e.call(PLUS_FOUR, &args![entry]).u32();
                let at = e.call(ELEMENT_ADDRESS, &args![infos, k]).u32();
                if e.mem.u32(at) == info {
                    present = true;
                    break;
                }
                k += 1;
            }
            present
        } else {
            // The cast's result is kept in a local the exe never reads.
            let quest_id = e.mem.u32(entry);
            form_as(e, quest_id, TYPE_TES_QUEST);
            true
        };
        if remove {
            e.call(RECORD_REMOVE, &args![links, index, 1u32]);
            count -= 1;
            index -= 1;
        }
        index += 1;
    }
    if e.mem.u32(entry) != 0 {
        let id = e.mem.u32(entry);
        let quest = form_as(e, id, TYPE_TES_QUEST);
        e.mem.set_u32(entry, quest);
    } else if e.mem.u32(entry + 0x2c) != 0 {
        let id = e.with_stack(4, |e, id_at| {
            let id = e.mem.u32(entry + 0x2c);
            e.mem.set_u32(id_at.addr(), id);
            let file = e.call(0x0048_4e60, &args![this, u32::MAX]).u32();
            e.call(0x0048_5d50, &args![id_at, file]);
            e.mem.u32(id_at.addr())
        });
        let quest = form_as(e, id, TYPE_TES_QUEST);
        e.mem.set_u32(entry + 0x2c, quest);
    } else {
        let mut total = 0u32;
        if e.call(PLUS_FOUR, &args![entry]).u32() != 0 {
            let infos = e.call(PLUS_FOUR, &args![entry]).u32();
            total = e.call(WORD_AT_0XC, &args![infos]).u32();
        }
        let mut live = 0i32;
        if total != 0 {
            let mut k = 0i32;
            while k < total as i32 {
                let infos = e.call(PLUS_FOUR, &args![entry]).u32();
                let at = e.call(ELEMENT_ADDRESS, &args![infos, k]).u32();
                let info = e.mem.u32(at);
                if !e.call(0x0044_0d80, &args![info]).bool() {
                    live += 1;
                }
                k += 1;
            }
        }
        if live > 0 {
            let name = e.vcall(this.addr(), 0x130, &[]).u32();
            e.call(PRINT, &args![NO_QUEST_REFERENCE, name, live]);
        }
    }
    e.mem.set_u8(entry + 0x30, 1);
}

// Translated from 0061a450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` (a, b, by_priority): whether topic `a` sorts before topic `b`;
/// false when either is null. With `by_priority`, the priority difference
/// `b - a` (`00526ae0`'s floats, x87, rounded to a `float`) above the
/// `double` at `01030f28` gives true; a difference that `0040eb10` (with
/// 0.0 and the `float` at `01017d00`) calls equal falls back to the names;
/// any other gives false. By name: true when `a`'s first character (vtable
/// slot 0x130, signed) is below `b`'s, else `00404dc0(name a, name b) > 0`
/// (signed; the names fetched again, `b`'s first).
pub fn fn_0061a450(e: &mut Engine, a: Ptr, b: Ptr, by_priority: u8) -> bool {
    if a.is_null() || b.is_null() {
        return false;
    }
    let name_a = e.vcall(a.addr(), 0x130, &[]).u32();
    let name_b = e.vcall(b.addr(), 0x130, &[]).u32();
    if by_priority != 0 {
        let priority_b = e.call(0x0052_6ae0, &args![b]).f64();
        let priority_a = e.call(0x0052_6ae0, &args![a]).f64();
        let difference = x87_store32(x87_sub(priority_b, priority_a));
        let limit: f64 = e.global(0x0103_0f28);
        let d = x87_load32(difference);
        // FCOMP, then `TEST AH,0x41` (C0 or C3): below, equal or unordered
        // go on.
        if d > limit {
            return true;
        }
        let epsilon = x87_quiet_bits(e.global::<u32>(0x0101_7d00));
        if !e
            .call(0x0040_eb10, &args![difference, 0u32, epsilon])
            .bool()
        {
            return false;
        }
    }
    let first_a = i32::from(e.mem.i8(name_a));
    let first_b = i32::from(e.mem.i8(name_b));
    if first_a < first_b {
        return true;
    }
    let name_b = e.vcall(b.addr(), 0x130, &[]).u32();
    let name_a = e.vcall(a.addr(), 0x130, &[]).u32();
    e.call(0x0040_4dc0, &args![name_a, name_b]).i32() > 0
}

// Translated from 0061a380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` (topic, list, by_priority): inserts `topic` into the sorted
/// topic `list` (nothing when either is null): into an empty list at its
/// start (`005ae3d0`); otherwise, walking the nodes while they are not
/// empty, nothing when the topic is already there, before the first topic
/// it sorts before (`0061a450`), else after the last node (`00905820`).
/// The item address passed is the address of the `topic` argument (a stack
/// copy here).
pub fn fn_0061a380(e: &mut Engine, topic: u32, list: u32, by_priority: u8) {
    if topic == 0 || list == 0 {
        return;
    }
    e.with_stack(4, |e, topic_at| {
        e.mem.set_u32(topic_at.addr(), topic);
        if e.call(LIST_IS_EMPTY, &args![list]).bool() {
            e.call(NODE_INSERT, &args![list, topic_at]);
            return;
        }
        let mut node = list;
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let current = item_of(e, node);
            let topic = e.mem.u32(topic_at.addr());
            if current == topic {
                return;
            }
            if fn_0061a450(e, Ptr::new(current), Ptr::new(topic), by_priority) {
                e.call(NODE_INSERT, &args![node, topic_at]);
                return;
            }
            if e.call(NODE_NEXT, &args![node]).u32() == 0 {
                e.call(NODE_APPEND, &args![node, topic_at]);
                return;
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
}

// Translated from 0061a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061a5a0 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `cdecl` (by_priority, list): sorts a topic list (the data handler's,
/// `004612e0` on `011c3f2c`, for a null `list`): every topic that sorts
/// before its successor (`0061a450`) is taken out (through the previous
/// node, `00905330`, or `0063f7b0` on its own node) and put into a
/// temporary list (`005ae3d0`); the walk goes on from the previous node's
/// next or the same node. Then each topic of the temporary list is taken
/// off it and inserted in order (`0061a380`). The temporary list is
/// constructed and destroyed by `0096a2d0` / `0046ffb0`.
pub fn fn_0061a5a0(e: &mut Engine, by_priority: u8, list: u32) {
    let head = if list == 0 {
        let handler = e.mem.u32(0x011c_3f2c);
        e.call(0x0046_12e0, &args![handler]).u32()
    } else {
        list
    };
    e.with_stack(0x10, |e, locals| {
        let temp = locals.addr();
        let moved_at = locals.addr() + 8;
        e.call(LIST_CONSTRUCT, &args![temp]);
        let mut node = head;
        let mut previous = 0u32;
        while node != 0 && item_of(e, node) != 0 {
            let mut moved = false;
            if e.call(NODE_NEXT, &args![node]).u32() != 0 {
                let current = item_of(e, node);
                e.mem.set_u32(moved_at, current);
                let next = e.call(NODE_NEXT, &args![node]).u32();
                let following = item_of(e, next);
                if fn_0061a450(e, Ptr::new(current), Ptr::new(following), by_priority) {
                    if previous != 0 {
                        e.call(NODE_REMOVE_AFTER, &args![previous, moved_at]);
                        node = e.call(NODE_NEXT, &args![previous]).u32();
                    } else {
                        e.call(NODE_REMOVE, &args![node]);
                    }
                    e.call(NODE_INSERT, &args![temp, moved_at]);
                    moved = true;
                }
            }
            if !moved {
                previous = node;
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        }
        while !e.call(LIST_IS_EMPTY, &args![temp]).bool() {
            let topic = item_of(e, temp);
            e.call(NODE_REMOVE, &args![temp]);
            fn_0061a380(e, topic, head, by_priority);
        }
        e.call(LIST_DESTROY, &args![temp]);
    });
}

/// The shared tail of the two info pickers: with no candidate gathered,
/// 0 when `random_only`, else the last info that said something; otherwise
/// a random candidate (`00487f50() % count + 1`, unsigned, counting the
/// non-null items), after `00470470` on the list. The temporary list is
/// destroyed (`0046ffb0`) either way.
fn pick_candidate(e: &mut Engine, temp: u32, count: u32, random_only: u8, last_said: u32) -> u32 {
    if e.call(LIST_IS_EMPTY, &args![temp]).bool() {
        let result = if random_only != 0 { 0 } else { last_said };
        e.call(LIST_DESTROY, &args![temp]);
        return result;
    }
    let pick = if count != 0 {
        e.call(0x0048_7f50, &[]).u32() % count + 1
    } else {
        0
    };
    let mut taken = 0i32;
    let mut node = temp;
    let mut chosen = 0u32;
    while taken < pick as i32 && node != 0 {
        let item = item_of(e, node);
        if item != 0 {
            chosen = item;
            taken += 1;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(0x0047_0470, &args![temp]);
    e.call(LIST_DESTROY, &args![temp]);
    chosen
}

/// Whether an info is excluded for `speaker`: info flag (`0061aef0`),
/// `speaker`'s vtable slot 0xFC and `00935a40(speaker, info)` all true.
fn info_excluded(e: &mut Engine, info: u32, speaker: u32) -> bool {
    info != 0
        && fn_0061aef0(e, Ptr::new(info))
        && e.vcall(speaker, 0xfc, &[]).bool()
        && e.call(0x0093_5a40, &args![speaker, info]).bool()
}

/// What the gathering loops do with a taken info: `None` to go on,
/// `Some(Some(info))` to return it at once, `Some(None)` to stop the walk.
type Taken = Option<Option<u32>>;

/// A taken info: stop after it when `0061aed0`; returned at once when it is
/// the first and has no bit 0x2 of +0x25; a later one without that bit
/// stops the walk; others are added to the temporary list (`005ae3d0`
/// with the address of the stack slot holding it).
fn take_info(e: &mut Engine, temp: u32, info_at: u32, stop: &mut bool, count: &mut u32) -> Taken {
    let info = e.mem.u32(info_at);
    if fn_0061aed0(e, Ptr::new(info)) {
        *stop = true;
    }
    if e.call(LIST_IS_EMPTY, &args![temp]).bool() && !fn_00619410(e, Ptr::new(info)) {
        return Some(Some(info));
    }
    if !fn_00619410(e, Ptr::new(info)) {
        *stop = true;
        return Some(None);
    }
    e.call(NODE_INSERT, &args![temp, info_at]);
    *count = count.wrapping_add(1);
    None
}

// Translated from 0061a7d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061a7d0 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `cdecl` (topic, said, speaker, target, random_only, about, exclude):
/// picks the info of `topic` for `speaker` and `target`. A topic for which
/// `00619df0` is true is replaced by `TESTopic::GetTopic(1, 5)`; `*said`
/// becomes 0; 0 for a null or `00440d80` topic. For each per-quest entry
/// (while the walk is not stopped) whose quest is not null, passes
/// `00455620` and whose condition (`005f5760`) is true for the pair
/// (`TESCondition::IsTrue`): a `TopicInfoArray` on the stack (`006197c0`)
/// gets the entry's infos (`00863d90`) and its links (records with an info
/// at their position, clamped to the count, `0061bc10`); then each info in
/// order: not excluded (see `info_excluded`) and accepted by `0061e600`
/// (with a "said" byte, 0, speaker, target) makes `*said` 0 and is taken
/// unless:
/// - with `random_only` or an `about` topic of kind 0 (`0061b9f0`), the
///   info's link list (`009e32d0`) rules it out: with `about`, only infos
///   that list `about` or a form with id 0xD3, or (without `random_only`)
///   whose walk ended on an empty node; without `about`, a non-empty list
///   is only for the player as `target`;
/// - in that case also, an info listed in `exclude` (by id, or a quest-less
///   `0061af10` check: same quest via `00825c00` and the topic argument as given
///   via `0044edb0`) is dropped.
///
/// Infos that said something (without `random_only`) are remembered as the
/// fallback and set `*said`. The array is destroyed with `00618920`. The
/// result: see `take_info` and `pick_candidate`.
#[allow(clippy::too_many_arguments)]
pub fn fn_0061a7d0(
    e: &mut Engine,
    topic: Ptr,
    said: Ptr,
    speaker: u32,
    target: u32,
    random_only: u8,
    about: u32,
    exclude: u32,
) -> u32 {
    let original = topic.addr();
    let mut topic = original;
    if topic != 0 && e.call(0x0061_9df0, &args![topic]).bool() {
        topic = tes_topic_get_topic(e, 1, 5).addr();
    }
    e.mem.set_u8(said.addr(), 0);
    if topic == 0 || e.call(0x0044_0d80, &args![topic]).bool() {
        return 0;
    }
    e.with_stack(0x10, |e, locals| {
        let temp = locals.addr();
        let info_at = locals.addr() + 8;
        let said_byte = locals.addr() + 0xc;
        e.call(LIST_CONSTRUCT, &args![temp]);
        let mut count = 0u32;
        let mut stop = false;
        let mut last_said = 0u32;
        let mut node = e.call(TOPIC_QUEST_LIST, &args![topic]).u32();
        while node != 0 && item_of(e, node) != 0 && !stop {
            let entry = item_of(e, node);
            let quest = e.mem.u32(entry);
            node = e.call(NODE_NEXT, &args![node]).u32();
            if quest == 0 || !e.call(0x0045_5620, &args![quest]).bool() {
                continue;
            }
            let condition = e.call(0x005f_5760, &args![quest]).u32();
            if !e
                .call(0x0068_0c30, &args![condition, speaker, target])
                .bool()
            {
                continue;
            }
            let outcome = e.with_stack(0x18, |e, infos| {
                fn_006197c0(e, infos);
                let source = e.call(PLUS_FOUR, &args![entry]).u32();
                let total = e.call(WORD_AT_0XC, &args![source]).u32();
                let mut i = 0u32;
                while i < total {
                    let at = e.call(ELEMENT_ADDRESS, &args![source, i]).u32();
                    e.call(0x0086_3d90, &args![infos, at]);
                    i += 1;
                }
                let links = entry + 0x1c;
                let records = e.call(ARRAY_COUNT, &args![links]).u32();
                let mut j = 0u32;
                while j < records {
                    let record = e.call(RECORD_ADDRESS, &args![links, j]).u32();
                    if e.mem.u32(record + 4) != 0 {
                        let record = e.call(RECORD_ADDRESS, &args![links, j]).u32();
                        let mut position = e.mem.u32(record + 8);
                        if (position as i32) < 0
                            || position > e.call(WORD_AT_0XC, &args![infos]).u32()
                        {
                            position = e.call(WORD_AT_0XC, &args![infos]).u32();
                        }
                        let record = e.call(RECORD_ADDRESS, &args![links, j]).u32();
                        let info = e.mem.u32(record + 4);
                        e.call(INFO_INSERT, &args![infos, position, info]);
                    }
                    j += 1;
                }
                let mut k = 0u32;
                let mut info = 0u32;
                if e.call(WORD_AT_0XC, &args![infos]).u32() > k {
                    let at = e.call(ELEMENT_ADDRESS, &args![infos, k]).u32();
                    info = e.mem.u32(at);
                }
                e.mem.set_u32(info_at, info);
                let mut outcome: Taken = None;
                while info != 0 {
                    e.mem.set_u8(said_byte, 0);
                    let allowed = !info_excluded(e, info, speaker);
                    if info != 0
                        && allowed
                        && e.call(0x0061_e600, &args![info, said_byte, 0u32, speaker, target])
                            .bool()
                    {
                        e.mem.set_u8(said.addr(), 0);
                        let mut take = true;
                        if random_only != 0 || (about != 0 && fn_0061b9f0(e, Ptr::new(about)) == 0)
                        {
                            take = links_allow(e, info, about, random_only, target);
                            if take {
                                if fn_0061aed0(e, Ptr::new(info)) {
                                    stop = true;
                                }
                                if listed_in(e, exclude, info, entry, original) {
                                    take = false;
                                }
                            }
                        }
                        if take {
                            if let Some(result) = take_info(e, temp, info_at, &mut stop, &mut count)
                            {
                                outcome = Some(result);
                                break;
                            }
                        }
                    }
                    if random_only == 0 && e.mem.u8(said_byte) != 0 {
                        last_said = info;
                        e.mem.set_u8(said.addr(), 1);
                    }
                    if stop {
                        break;
                    }
                    k += 1;
                    info = if k < e.call(WORD_AT_0XC, &args![infos]).u32() {
                        let at = e.call(ELEMENT_ADDRESS, &args![infos, k]).u32();
                        e.mem.u32(at)
                    } else {
                        0
                    };
                    e.mem.set_u32(info_at, info);
                }
                e.call(0x0061_8920, &args![infos]);
                outcome
            });
            match outcome {
                Some(Some(info)) => {
                    e.call(LIST_DESTROY, &args![temp]);
                    return info;
                }
                Some(None) => {
                    stop = true;
                }
                None => {}
            }
        }
        pick_candidate(e, temp, count, random_only, last_said)
    })
}

/// The link-list test of `0061a7d0` (see its doc).
fn links_allow(e: &mut Engine, info: u32, about: u32, random_only: u8, target: u32) -> bool {
    let links = e.call(0x009e_32d0, &args![info]).u32();
    if links == 0 {
        return true;
    }
    if about != 0 {
        let mut take = false;
        let mut node = e.call(NODE_ITEM, &args![links]).u32();
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            if item_of(e, node) == about {
                take = true;
                break;
            }
            let form = item_of(e, node);
            if e.call(WORD_AT_0XC, &args![form]).u32() == 0xd3 {
                take = true;
                break;
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        if random_only == 0 && node != 0 && e.call(LIST_IS_EMPTY, &args![node]).bool() {
            take = true;
        }
        take
    } else {
        let first = e.call(NODE_ITEM, &args![links]).u32();
        let player: u32 = e.global(PLAYER);
        e.call(LIST_IS_EMPTY, &args![first]).bool() || target == player
    }
}

/// Whether `info` is in the `exclude` list of `0061a7d0` (see its doc).
fn listed_in(e: &mut Engine, exclude: u32, info: u32, entry: u32, topic: u32) -> bool {
    let mut node = exclude;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let item = item_of(e, node);
        if e.call(WORD_AT_0XC, &args![item]).u32() == info {
            return true;
        }
        let quest = e.mem.u32(entry);
        if !e.call(0x0061_af10, &args![quest]).bool()
            && e.call(0x0082_5c00, &args![item]).u32() == e.mem.u32(entry)
            && e.call(0x0044_edb0, &args![item]).u32() == topic
        {
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 0061af30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061af30 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `cdecl` (info, said, speaker, target, random_only, exclude): the info
/// picker of `0061a7d0` over the infos linked from `info` (`00460140` on
/// `009e32d0(info)`), each with its quest (`006733e0`): `*said` becomes 0;
/// 0 for a null or `00440d80` info; a candidate whose quest passes
/// `00455620` and its condition, is not excluded and is accepted by
/// `0061e600` is taken unless listed in `exclude` (by id). Infos that said
/// something (without `random_only`) are the fallback and set `*said`.
#[allow(clippy::too_many_arguments)]
pub fn fn_0061af30(
    e: &mut Engine,
    info: Ptr,
    said: Ptr,
    speaker: u32,
    target: u32,
    random_only: u8,
    exclude: u32,
) -> u32 {
    let first = info.addr();
    e.mem.set_u8(said.addr(), 0);
    if first == 0 || e.call(0x0044_0d80, &args![first]).bool() {
        return 0;
    }
    e.with_stack(0x10, |e, locals| {
        let temp = locals.addr();
        let item_at = locals.addr() + 8;
        let said_byte = locals.addr() + 0xc;
        e.call(LIST_CONSTRUCT, &args![temp]);
        let mut count = 0u32;
        let mut stop = false;
        let mut node = 0u32;
        if e.call(0x009e_32d0, &args![first]).u32() != 0 {
            let links = e.call(0x009e_32d0, &args![first]).u32();
            node = e.call(0x0046_0140, &args![links]).u32();
        }
        let mut last_said = 0u32;
        while node != 0 && item_of(e, node) != 0 && !stop {
            let item = item_of(e, node);
            e.mem.set_u32(item_at, item);
            let quest = e.call(0x0067_33e0, &args![item]).u32();
            node = e.call(NODE_NEXT, &args![node]).u32();
            if quest == 0 || !e.call(0x0045_5620, &args![quest]).bool() {
                continue;
            }
            let condition = e.call(0x005f_5760, &args![quest]).u32();
            if !e
                .call(0x0068_0c30, &args![condition, speaker, target])
                .bool()
            {
                continue;
            }
            e.mem.set_u8(said_byte, 0);
            let allowed = !info_excluded(e, item, speaker);
            if item == 0 || !allowed {
                continue;
            }
            if !e
                .call(0x0061_e600, &args![item, said_byte, 0u32, speaker, target])
                .bool()
            {
                continue;
            }
            e.mem.set_u8(said.addr(), 0);
            let mut take = true;
            if fn_0061aed0(e, Ptr::new(item)) {
                stop = true;
            }
            let mut q = exclude;
            while q != 0 && !e.call(LIST_IS_EMPTY, &args![q]).bool() {
                let listed = item_of(e, q);
                if e.call(WORD_AT_0XC, &args![listed]).u32() == item {
                    take = false;
                    break;
                }
                q = e.call(NODE_NEXT, &args![q]).u32();
            }
            if take {
                match take_info(e, temp, item_at, &mut stop, &mut count) {
                    Some(Some(result)) => {
                        e.call(LIST_DESTROY, &args![temp]);
                        return result;
                    }
                    Some(None) => break,
                    None => {}
                }
            }
            if random_only == 0 && e.mem.u8(said_byte) != 0 {
                last_said = item;
                e.mem.set_u8(said.addr(), 1);
            }
            if stop {
                break;
            }
        }
        pick_candidate(e, temp, count, random_only, last_said)
    })
}

// Translated from 0061a790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopic::GetMatchingInfo` with six words (`RET 0x18`): `0061a7d0`
/// with the topic first and the fourth word as a byte.
#[allow(clippy::too_many_arguments)]
pub fn fn_0061a790(
    e: &mut Engine,
    this: Ptr,
    said: Ptr,
    speaker: u32,
    target: u32,
    random_only: u8,
    about: u32,
    exclude: u32,
) -> u32 {
    fn_0061a7d0(e, this, said, speaker, target, random_only, about, exclude)
}

// Translated from 0061a760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopic::GetMatchingInfo_ov2` (Xbox PDB): `0061a790` with the three
/// words and 0, 0, 0.
pub fn tes_topic_get_matching_info_ov2(
    e: &mut Engine,
    this: Ptr,
    said: Ptr,
    speaker: u32,
    target: u32,
) -> u32 {
    fn_0061a790(e, this, said, speaker, target, 0, 0, 0)
}

// Translated from 0061b320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061b320 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESTopic::CreateDialogueItem` (Xbox PDB): the info for `speaker` and
/// `target` (`0061a790` with a stack "said" byte, `random_only` 1, `about`,
/// `exclude`); 0 without one. Otherwise a new `DialogueItem` (0x1C bytes,
/// `DialogueItem::DialogueItem_ov2` `0083c520` with the info's +0x48 word
/// (`006733e0`), the topic, the info and `speaker`; null when the
/// allocation fails). With `run_result`, a dialogue item whose info (its
/// +0xC, `0084e3a0`) passes `00579200` runs the info's result script
/// (`TESTopicInfo::RunResult`, `0061f170`, with 0 and `speaker`) and
/// `TESTopicInfo::AddTopicList` (`0061f150`). Returns the item.
pub fn tes_topic_create_dialogue_item(
    e: &mut Engine,
    this: Ptr,
    speaker: u32,
    target: u32,
    about: u32,
    exclude: u32,
    run_result: u8,
) -> u32 {
    let info = e.with_stack(4, |e, said| {
        fn_0061a790(e, this, said, speaker, target, 1, about, exclude)
    });
    if info == 0 {
        return 0;
    }
    let block = e.call(0x0040_1000, &args![0x1cu32]).u32();
    let item = if block != 0 {
        let word = e.call(0x0067_33e0, &args![info]).u32();
        e.call(0x0083_c520, &args![block, word, this, info, speaker])
            .u32()
    } else {
        0
    };
    if run_result != 0 && item != 0 && e.call(WORD_AT_0XC, &args![item]).u32() != 0 {
        let current = e.call(WORD_AT_0XC, &args![item]).u32();
        if e.call(0x0057_9200, &args![current]).bool() {
            let current = e.call(WORD_AT_0XC, &args![item]).u32();
            e.call(0x0061_f170, &args![current, 0u32, speaker]);
            let current = e.call(WORD_AT_0XC, &args![item]).u32();
            e.call(0x0061_f150, &args![current]);
        }
    }
    item
}

/// The stack frame of `CreateConversation` (0x830 bytes below `EBP`, then
/// the saved `EBP`, the return address and the four arguments), kept in
/// game memory so that an index past its 500-entry array reaches what the
/// exe's index reaches.
struct Frame {
    ebp: u32,
}

impl Frame {
    fn get(&self, e: &Engine, offset: i32) -> u32 {
        e.mem.u32(self.ebp.wrapping_add(offset as u32))
    }
    fn set(&self, e: &mut Engine, offset: i32, value: u32) {
        e.mem.set_u32(self.ebp.wrapping_add(offset as u32), value);
    }
}

// `CreateConversation`'s frame offsets (from `EBP`).
const F_SPEAKER_ARG: i32 = 0x8;
const F_TARGET_ARG: i32 = 0xc;
const F_LIST_ARG: i32 = 0x10;
const F_TOPIC_ARG: i32 = 0x14;
const F_SPEAKER: i32 = -0x4;
const F_DIALOGUE: i32 = -0x8;
const F_PREVIOUS_TOPIC: i32 = -0xc;
const F_TARGET: i32 = -0x14;
const F_PICK: i32 = -0x18;
const F_TOPIC: i32 = -0x1c;
const F_DONE: i32 = -0x1d;
const F_NEXT: i32 = -0x24;
const F_USED: i32 = -0x7f8;
const F_CHOICES: i32 = -0x7fc;
const F_CHOICE_COUNT: i32 = -0x800;

/// `rand() % n + 1` with `CDQ; IDIV` (signed).
fn random_pick(e: &mut Engine, n: u32) -> u32 {
    let r = e.call(0x00ec_adb8, &[]).i32();
    (r.wrapping_rem(n as i32)).wrapping_add(1) as u32
}

// Translated from 0061b440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061b440 - the saved EBP and return-address words above the frame, which an index past the 500-entry array would reach, are zero in the model's frame
/// `TESTopic::CreateConversation` (Xbox PDB, `cdecl`: speaker, target, the
/// list of dialogue items, a topic): empties `list` (deleting each item,
/// `005c90d0` with 1); 0 for a null list, speaker or target, or when the
/// speaker's `007af430` form is of type 0x2B and its +0x30 part's vtable
/// slot 0x68 says 0. Seeds `srand` with `0061b9d0(0)`; the topic is the
/// given one or the data handler's topic 0xD2 (`TESDataHandler::GetTopic`).
/// Then, while there is a topic and the list holds fewer than 100 items
/// (`005ae380`): a dialogue item (`CreateDialogueItem`, with the previous
/// topic and the list); without one, when no topic was given or it is of
/// kind 1, the same with topic 0xD4; none ends the conversation. An item's
/// info that passes `00579200` runs its result script twice (`0061f170`
/// with 0 and 1) and `0061f150`. The item is appended (`00905820`), and
/// the next topic is chosen from the info's links (`00413f40` of
/// `009e32d0`): a random choice not yet used (`rand`, marked in a
/// 500-entry stack array that is cleared first; the walk takes the topic at
/// that position), speakers swapped for a topic of kind 0 or (half the
/// time) kind 2, until all choices were used or one gives an item. With
/// the byte at `011cbeac` set, the conversation is printed to the console
/// (`00703c00`). Returns the list's count.
pub fn tes_topic_create_conversation(
    e: &mut Engine,
    speaker: u32,
    target: u32,
    list: u32,
    topic: u32,
) -> u32 {
    if list == 0 {
        return 0;
    }
    while list != 0 && !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let item = item_of(e, list);
        if item != 0 {
            e.call(0x005c_90d0, &args![item, 1u32]);
        }
        e.call(NODE_REMOVE, &args![list]);
    }
    if speaker == 0 || target == 0 {
        return 0;
    }
    e.with_stack(0x830 + 0x18, |e, block| {
        let f = Frame {
            ebp: block.addr() + 0x830,
        };
        f.set(e, F_SPEAKER_ARG, speaker);
        f.set(e, F_TARGET_ARG, target);
        f.set(e, F_LIST_ARG, list);
        f.set(e, F_TOPIC_ARG, topic);
        conversation(e, &f)
    })
}

/// The body of `CreateConversation` after its first checks, on its frame.
fn conversation(e: &mut Engine, f: &Frame) -> u32 {
    let speaker = f.get(e, F_SPEAKER_ARG);
    f.set(e, F_SPEAKER, speaker);
    let target = f.get(e, F_TARGET_ARG);
    f.set(e, F_TARGET, target);
    f.set(e, F_TOPIC, 0);
    f.set(e, F_PREVIOUS_TOPIC, 0);
    let form = e.call(0x007a_f430, &args![f.get(e, F_SPEAKER)]).u32();
    if form != 0
        && e.call(0x0040_1170, &args![form]).u32() == 0x2b
        && e.vcall(form + 0x30, 0x68, &[]).u32() == 0
    {
        return 0;
    }
    f.set(e, F_TOPIC, f.get(e, F_TOPIC_ARG));
    if f.get(e, F_TOPIC) == 0 {
        let handler = e.mem.u32(0x011c_3f2c);
        let fallback = e.call(0x0046_1690, &args![handler, 0xd2u32]).u32();
        f.set(e, F_TOPIC, fallback);
    }
    f.set(e, F_DIALOGUE, 0);
    f.set(e, F_PICK, u32::MAX);
    let seed = e.call(0x0061_b9d0, &args![0u32]).u32();
    e.call(0x00ec_ada6, &args![seed]);
    let list = |e: &Engine| f.get(e, F_LIST_ARG);
    while f.get(e, F_TOPIC) != 0 {
        let l = list(e);
        if e.call(0x005a_e380, &args![l]).u32() >= 100 {
            break;
        }
        let item = create_item(e, f);
        f.set(e, F_DIALOGUE, item);
        if f.get(e, F_DIALOGUE) == 0 {
            let given = f.get(e, F_TOPIC_ARG);
            if given == 0 || fn_0061b9f0(e, Ptr::new(given)) == 1 {
                let handler = e.mem.u32(0x011c_3f2c);
                let fallback = e.call(0x0046_1690, &args![handler, 0xd4u32]).u32();
                f.set(e, F_TOPIC, fallback);
                let item = create_item(e, f);
                f.set(e, F_DIALOGUE, item);
            }
            if f.get(e, F_DIALOGUE) == 0 {
                break;
            }
        }
        let dialogue = f.get(e, F_DIALOGUE);
        if dialogue != 0 && e.call(WORD_AT_0XC, &args![dialogue]).u32() != 0 {
            let info = e.call(WORD_AT_0XC, &args![dialogue]).u32();
            if e.call(0x0057_9200, &args![info]).bool() {
                let speaker_arg = f.get(e, F_SPEAKER_ARG);
                let info = e.call(WORD_AT_0XC, &args![dialogue]).u32();
                e.call(0x0061_f170, &args![info, 0u32, speaker_arg]);
                let speaker_arg = f.get(e, F_SPEAKER_ARG);
                let info = e.call(WORD_AT_0XC, &args![dialogue]).u32();
                e.call(0x0061_f170, &args![info, 1u32, speaker_arg]);
                let info = e.call(WORD_AT_0XC, &args![dialogue]).u32();
                e.call(0x0061_f150, &args![info]);
            }
        }
        let l = list(e);
        e.call(
            NODE_APPEND,
            &args![l, f.ebp.wrapping_add(F_DIALOGUE as u32)],
        );
        f.set(e, F_PREVIOUS_TOPIC, f.get(e, F_TOPIC));
        f.set(e, F_TOPIC, 0);
        f.set(e, F_NEXT, 0);
        e.mem.set_u8(f.ebp.wrapping_add(F_DONE as u32), 0);
        e.call(
            0x0040_3d30,
            &args![f.ebp.wrapping_add(F_USED as u32), 0u32, 0x7d0u32],
        );
        f.set(e, F_PICK, u32::MAX);
        while f.get(e, F_NEXT) == 0 && e.mem.u8(f.ebp.wrapping_add(F_DONE as u32)) == 0 {
            choose_next(e, f);
        }
    }
    if e.global::<u8>(0x011c_beac) != 0 {
        print_conversation(e, list(e));
    }
    let l = list(e);
    e.call(0x005a_e380, &args![l]).u32()
}

/// `CreateDialogueItem` on the frame's topic with its speaker, target,
/// previous topic and list.
fn create_item(e: &mut Engine, f: &Frame) -> u32 {
    let topic = f.get(e, F_TOPIC);
    let speaker = f.get(e, F_SPEAKER);
    let target = f.get(e, F_TARGET);
    let previous = f.get(e, F_PREVIOUS_TOPIC);
    let list = f.get(e, F_LIST_ARG);
    tes_topic_create_dialogue_item(e, Ptr::new(topic), speaker, target, previous, list, 0)
}

/// One pass of `CreateConversation`'s inner loop: the next topic.
fn choose_next(e: &mut Engine, f: &Frame) {
    let done_at = f.ebp.wrapping_add(F_DONE as u32);
    let dialogue = f.get(e, F_DIALOGUE);
    let info = e.call(WORD_AT_0XC, &args![dialogue]).u32();
    if info == 0 {
        e.mem.set_u8(done_at, 1);
        return;
    }
    let info = e.call(WORD_AT_0XC, &args![dialogue]).u32();
    if e.call(0x009e_32d0, &args![info]).u32() == 0 {
        e.mem.set_u8(done_at, 1);
        return;
    }
    let info = e.call(WORD_AT_0XC, &args![dialogue]).u32();
    let links = e.call(0x009e_32d0, &args![info]).u32();
    let choices = e.call(0x0041_3f40, &args![links]).u32();
    f.set(e, F_CHOICES, choices);
    let n = e.call(0x005a_e380, &args![choices]).u32();
    f.set(e, F_CHOICE_COUNT, n);
    if n != 0 {
        if f.get(e, F_PICK) == u32::MAX {
            let pick = random_pick(e, f.get(e, F_CHOICE_COUNT));
            f.set(e, F_PICK, pick);
        } else {
            // The used flag of a pick sits at `EBP - 0x7fc + pick * 4`.
            loop {
                let pick = f.get(e, F_PICK);
                if f.get(e, F_CHOICES.wrapping_add((pick as i32).wrapping_mul(4))) == 0 {
                    break;
                }
                let pick = random_pick(e, f.get(e, F_CHOICE_COUNT));
                f.set(e, F_PICK, pick);
            }
        }
    }
    let pick = f.get(e, F_PICK) as i32;
    if pick > 0 {
        f.set(e, F_CHOICES.wrapping_add(pick.wrapping_mul(4)), 1);
    }
    let mut k = 0i32;
    loop {
        let choices = f.get(e, F_CHOICES);
        if k >= f.get(e, F_PICK) as i32
            || choices == 0
            || e.call(LIST_IS_EMPTY, &args![choices]).bool()
        {
            break;
        }
        let topic = item_of(e, choices);
        f.set(e, F_TOPIC, topic);
        let next = e.call(NODE_NEXT, &args![choices]).u32();
        f.set(e, F_CHOICES, next);
        k += 1;
    }
    let dialogue = f.get(e, F_DIALOGUE);
    let info = e.call(WORD_AT_0XC, &args![dialogue]).u32();
    let kind = fn_0061b9f0(e, Ptr::new(info));
    let swap = match kind {
        0 => true,
        2 => e.call(0x00ec_adb8, &[]).i32() % 2 == 0,
        _ => false,
    };
    if swap {
        let speaker = f.get(e, F_SPEAKER);
        let target = f.get(e, F_TARGET);
        f.set(e, F_SPEAKER, target);
        f.set(e, F_TARGET, speaker);
    }
    e.mem.set_u8(done_at, 1);
    let mut i = 0u32;
    while i < f.get(e, F_CHOICE_COUNT) && e.mem.u8(done_at) != 0 {
        if f.get(e, F_USED.wrapping_add((i as i32).wrapping_mul(4))) == 0 {
            e.mem.set_u8(done_at, 0);
        }
        i += 1;
    }
    let next = create_item(e, f);
    f.set(e, F_NEXT, next);
}

/// The console listing of `CreateConversation`.
fn print_conversation(e: &mut Engine, list: u32) {
    e.call(CONSOLE_PRINT, &args![CONVERSATION_HEADER]);
    let mut node = list;
    let mut index = 1u32;
    while node != 0 && item_of(e, node) != 0 {
        let item = item_of(e, node);
        node = e.call(NODE_NEXT, &args![node]).u32();
        if item != 0 {
            if e.call(0x0083_c7b0, &args![item]).bool() {
                let speaker = e.call(0x0096_11e0, &args![item]).u32();
                let response = e.call(0x0083_c820, &args![item]).u32();
                let text_at = e.call(NODE_ITEM, &args![response]).u32();
                let text = e.call(0x0055_9450, &args![text_at]).u32();
                let name = e.vcall(speaker, 0x130, &[]).u32();
                e.call(CONSOLE_PRINT, &args![CONVERSATION_LINE, index, name, text]);
            } else {
                let speaker = e.call(0x0096_11e0, &args![item]).u32();
                let name = e.vcall(speaker, 0x130, &[]).u32();
                e.call(CONSOLE_PRINT, &args![CONVERSATION_SILENT, index, name]);
            }
        }
        index = index.wrapping_add(1);
    }
}

/// RTTI type descriptor `.?AVTESTopic@@` (read from the exe).
const TYPE_TES_TOPIC: u32 = 0x0118_4720;
/// `00401170` (`thiscall`): a form's type byte.
const FORM_TYPE_OF: u32 = 0x0040_1170;
/// `00484020` and `00484150` (`thiscall`, `RET 4`; `tesform.cpp` in the engine map): the
/// form's own answers to the two questions below, for the other cases.
const FORM_ANSWER_484020: u32 = 0x0048_4020;
const FORM_ANSWER_484150: u32 = 0x0048_4150;
/// The word at `01187020` the object's first word (its vtable) must equal for `00619510`.
const OBJECT_VTABLE_WORD: u32 = 0x0118_7020;
/// `00618920` (`thiscall`): the destructor body `006197f0` runs; `0061bf50` (`thiscall`): the
/// one `0061c0f0` runs; `operator delete` (`00401030`, `cdecl`).
const DESTRUCT_618920: u32 = 0x0061_8920;
const DESTRUCT_61BF50: u32 = 0x0061_bf50;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `005a8080` (`thiscall`): the address of the byte at +0x3C.
const BYTE_3C_ADDRESS: u32 = 0x005a_8080;

// Translated from 00619430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a form): for a topic (type 0x45), whether this topic's form id
/// (`0084e3a0`) is below the other's; for an info (type 0x46), whether its owner topic
/// (`TESTopic::GetOwnerTopic`) is this one or vtable slot 0x3C says so for it; for anything
/// else, `00484020`. False when the casts or the owner fail.
pub fn fn_00619430(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let kind = e.call(FORM_TYPE_OF, &args![form]).u32();
    if kind == 0x45 {
        let topic = form_as_from(e, form, TYPE_TES_TOPIC);
        if topic != 0 {
            let mine = e.call(WORD_AT_0XC, &args![this]).u32();
            return mine < e.call(WORD_AT_0XC, &args![topic]).u32();
        }
        false
    } else if kind == 0x46 {
        let info = form_as_from(e, form, TYPE_TES_TOPIC_INFO);
        if info == 0 {
            return false;
        }
        let owner = tes_topic_get_owner_topic(e, Ptr::new(info)).addr();
        if owner == 0 {
            return false;
        }
        if owner == this.addr() {
            return true;
        }
        e.vcall(this.addr(), 0x3c, &args![owner]).bool()
    } else {
        e.call(FORM_ANSWER_484020, &args![this, form]).bool()
    }
}

/// The form cast from `TESForm` to `target` (`__RTDynamicCast`).
fn form_as_from(e: &mut Engine, form: u32, target: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .u32()
}

// Translated from 00619510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: an object): false unless the object's first word is the word at
/// `01187020`; then by its word at +0xC (the exe's jump table at `006195b0`): 0 asks
/// `00484150`, 1 to 6 answer false, 7 asks vtable slot 0x3C for the topic of the form id at +8
/// (`004839c0`, cast to `TESTopic`; false without one); above 7 false.
pub fn fn_00619510(e: &mut Engine, this: Ptr, object: u32) -> bool {
    if object == 0 || e.mem.u32(object) != e.global::<u32>(OBJECT_VTABLE_WORD) {
        return false;
    }
    match e.mem.u32(object.wrapping_add(0xc)) {
        0 => e.call(FORM_ANSWER_484150, &args![this, object]).bool(),
        1..=6 => false,
        7 => {
            let id = e.mem.u32(object.wrapping_add(8));
            let topic = form_as(e, id, TYPE_TES_TOPIC);
            if topic != 0 {
                e.vcall(this.addr(), 0x3c, &args![topic]).bool()
            } else {
                false
            }
        }
        _ => false,
    }
}

// Translated from 006197f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: flags; a scalar deleting destructor by its body): runs
/// `00618920` on `this` and, with flag 1, deletes it (`operator delete`). Returns `this`.
pub fn fn_006197f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT_618920, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0061a720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopic::GetMatchingInfo` (engine map, `RET 8`: two words passed on): the info
/// `TESTopic::GetMatchingInfo_ov2` answers, or 0 when it set its flag (a byte on the stack,
/// cleared first).
pub fn tes_topic_get_matching_info(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    e.with_stack(4, |e, flag| {
        e.mem.set_u8(flag.addr(), 0);
        let info = tes_topic_get_matching_info_ov2(e, this, flag, first, second);
        if e.mem.u8(flag.addr()) != 0 {
            0
        } else {
            info
        }
    })
}

// Translated from 0061af10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX`): whether bit 4 of the byte at +0x3C (`005a8080`) is set.
pub fn fn_0061af10(e: &mut Engine, this: Ptr) -> bool {
    let at = e.call(BYTE_3C_ADDRESS, &args![this]).u32();
    e.mem.u8(at) & 4 != 0
}

// Translated from 0061c0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<INFO_LINK_ELEMENT,1024>::scalar deleting destructor` (engine map, `RET 4`:
/// flags): runs `0061bf50` on `this` and, with flag 1, deletes it. Returns `this`.
pub fn fn_0061c0f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT_61BF50, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_9e10, fn_00619e10(Ptr, u32, u8) -> Ptr),
        entry!(0x0061_a2d0, tes_topic_get_topic(u32, i32) -> Ptr),
        entry!(0x0061_a310, tes_topic_get_owner_topic(Ptr) -> Ptr),
        entry!(0x0061_9410, fn_00619410(Ptr) -> bool),
        entry!(0x0061_aed0, fn_0061aed0(Ptr) -> bool),
        entry!(0x0061_aef0, fn_0061aef0(Ptr) -> bool),
        entry!(0x0061_b9f0, fn_0061b9f0(Ptr) -> i32),
        entry!(0x0061_bef0, fn_0061bef0(Ptr, u32, u32) -> Ptr),
        entry!(0x0061_97c0, fn_006197c0(Ptr) -> Ptr),
        entry!(0x0061_bf20, fn_0061bf20(Ptr) -> Ptr),
        entry!(0x0061_9740, fn_00619740(Ptr) -> Ptr),
        entry!(0x0061_95d0, fn_006195d0(Ptr, u32, u32) -> u32),
        entry!(0x0061_9820, tes_topic_insert_info(Ptr, u32, Ptr, u32) -> bool),
        entry!(0x0061_9030, tes_topic_init_item(Ptr)),
        entry!(0x0061_a450, fn_0061a450(Ptr, Ptr, u8) -> bool),
        entry!(0x0061_a380, fn_0061a380(u32, u32, u8)),
        entry!(0x0061_a5a0, fn_0061a5a0(u8, u32)),
        entry!(0x0061_a7d0, fn_0061a7d0(Ptr, Ptr, u32, u32, u8, u32, u32) -> u32),
        entry!(0x0061_af30, fn_0061af30(Ptr, Ptr, u32, u32, u8, u32) -> u32),
        entry!(0x0061_a790, fn_0061a790(Ptr, Ptr, u32, u32, u8, u32, u32) -> u32),
        entry!(0x0061_a760, tes_topic_get_matching_info_ov2(Ptr, Ptr, u32, u32) -> u32),
        entry!(0x0061_b320, tes_topic_create_dialogue_item(Ptr, u32, u32, u32, u32, u8) -> u32),
        entry!(0x0061_b440, tes_topic_create_conversation(u32, u32, u32, u32) -> u32),
        entry!(0x0061_9430, fn_00619430(Ptr, u32) -> bool),
        entry!(0x0061_9510, fn_00619510(Ptr, u32) -> bool),
        entry!(0x0061_97f0, fn_006197f0(Ptr, u32) -> Ptr),
        entry!(0x0061_a720, tes_topic_get_matching_info(Ptr, u32, u32) -> u32),
        entry!(0x0061_af10, fn_0061af10(Ptr) -> bool),
        entry!(0x0061_c0f0, fn_0061c0f0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// The doubles behave as the exe's bodies: a node's item is its first
    /// word, the next node its second; an entry's infos are at +4, with the
    /// count at +0xc of that and the infos in an array at +0x10 of the
    /// entry; a form's id is at +0xc.
    fn setup() -> Engine {
        let mut e = Engine::new();
        e.register(TOPIC_QUEST_LIST, |_, a| ret(a[0] + 0x2c));
        e.register(LIST_AT_0X108, |_, a| ret(a[0] + 0x108));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(PLUS_FOUR, |_, a| ret(a[0] + 4));
        e.register(WORD_AT_0XC, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(ENTRY_INFO, |e, a| {
            let array = e.mem.u32(a[0] + 0x20);
            ret(e.mem.u32(array + 4 * a[1]))
        });
        e
    }

    fn form(e: &mut Engine, id: u32) -> u32 {
        let p = e.mem.alloc(0x10);
        e.mem.set_u32(p + 0xc, id);
        p
    }

    /// A per-quest entry with these infos (count at +4+0xc, array at +0x20).
    fn entry(e: &mut Engine, infos: &[u32]) -> u32 {
        let p = e.mem.alloc(0x24);
        e.mem.set_u32(p + 0x10, infos.len() as u32);
        let array = e.mem.alloc(4 * infos.len().max(1) as u32);
        for (i, info) in infos.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *info);
        }
        e.mem.set_u32(p + 0x20, array);
        p
    }

    /// Writes a list at `head` (an embedded node) holding `items`.
    fn list(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            let next = if i + 1 < items.len() {
                e.mem.alloc(8)
            } else {
                0
            };
            e.mem.set_u32(node + 4, next);
            node = next;
        }
    }

    fn topic(e: &mut Engine, entries: &[u32]) -> u32 {
        let t = e.mem.alloc(0x40);
        list(e, t + 0x2c, entries);
        t
    }

    #[test]
    fn get_topic_checks_the_index() {
        let mut e = Engine::new();
        e.map(TOPIC_COUNTS, 0x40);
        e.mem.set_i32(TOPIC_COUNTS + 4, 2);
        let array = e.mem.alloc(0x18);
        e.mem.set_u32(array + 0xc, 0x4444);
        e.mem.set_u32(TOPIC_ARRAYS + 4, array);
        assert_eq!(e.call(0x0061_a2d0, &args![1u32, 1i32]).u32(), 0x4444);
        assert_eq!(e.call(0x0061_a2d0, &args![1u32, 2i32]).u32(), 0);
        assert_eq!(e.call(0x0061_a2d0, &args![1u32, -1i32]).u32(), 0);
    }

    #[test]
    fn finds_an_info_by_id_in_both_directions() {
        let mut e = setup();
        let first = form(&mut e, 0x77);
        let second = form(&mut e, 0x77);
        let other = form(&mut e, 0x55);
        let empty = entry(&mut e, &[]);
        let full = entry(&mut e, &[other, 0, first, second]);
        let t = topic(&mut e, &[empty, full]);
        assert_eq!(e.call(0x0061_9e10, &args![t, 0x77u32, 0u32]).u32(), first);
        assert_eq!(e.call(0x0061_9e10, &args![t, 0x77u32, 1u32]).u32(), second);
        // Only the low byte of the flag word counts.
        assert_eq!(
            e.call(0x0061_9e10, &args![t, 0x77u32, 0x100u32]).u32(),
            first
        );
        assert_eq!(e.call(0x0061_9e10, &args![t, 0x99u32, 1u32]).u32(), 0);
        // An item-less node ends the walk.
        let stop = topic(&mut e, &[0, full]);
        assert_eq!(e.call(0x0061_9e10, &args![stop, 0x77u32, 0u32]).u32(), 0);
    }

    #[test]
    fn owner_topic_is_the_one_holding_the_info() {
        let mut e = setup();
        let info = form(&mut e, 0x77);
        let twin = form(&mut e, 0x77);
        let holds_twin = entry(&mut e, &[twin]);
        let holds_info = entry(&mut e, &[info]);
        let a = topic(&mut e, &[holds_twin]);
        let b = topic(&mut e, &[holds_info]);
        let handler = e.mem.alloc(0x110);
        list(&mut e, handler + 0x108, &[a, b]);
        e.map(TOPIC_LIST_OWNER, 4);
        e.mem.set_u32(TOPIC_LIST_OWNER, handler);
        assert_eq!(e.call(0x0061_a310, &args![info]).u32(), b);
        let lost = form(&mut e, 0x55);
        assert_eq!(e.call(0x0061_a310, &args![lost]).u32(), 0);
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    /// Lists of (item, next) nodes; arrays with their elements' address at
    /// +4 and their count at +0xC (`0084e3a0` reads +0xC of anything).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e.register(NODE_REMOVE, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        e.register(NODE_INSERT, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) != 0 || e.mem.u32(a[0] + 4) != 0 {
                let moved = e.mem.alloc(8);
                let old_item = e.mem.u32(a[0]);
                let old_next = e.mem.u32(a[0] + 4);
                e.mem.set_u32(moved, old_item);
                e.mem.set_u32(moved + 4, old_next);
                e.mem.set_u32(a[0] + 4, moved);
            }
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.register(NODE_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            if e.mem.u32(node) == 0 {
                e.mem.set_u32(node, item);
            } else {
                let fresh = e.mem.alloc(8);
                e.mem.set_u32(fresh, item);
                e.mem.set_u32(node + 4, fresh);
            }
            Ret::default()
        });
        e.register(WORD_AT_0XC, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(PLUS_FOUR, |_, a| ret(a[0] + 4));
        e.register(ELEMENT_ADDRESS, |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(TOPIC_QUEST_LIST, |_, a| ret(a[0] + 0x2c));
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e
    }

    /// An array object with `elements` (its buffer at +4, count at +0xC).
    fn array_with(e: &mut Engine, at: u32, elements: &[u32]) {
        let buffer = e.mem.alloc(4 * elements.len().max(1) as u32 + 0x40);
        for (i, v) in elements.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *v);
        }
        e.mem.set_u32(at + 4, buffer);
        e.mem.set_u32(at + 0xc, elements.len() as u32);
    }

    #[test]
    fn b020_flag_tests() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x30);
        assert!(!e.call(0x0061_9410, &args![p]).bool());
        e.mem.set_u8(p + 0x25, 0x22);
        e.mem.set_u8(p + 0x26, 0x1);
        e.mem.set_u8(p + 0x24, 0xfe);
        assert!(e.call(0x0061_9410, &args![p]).bool());
        assert!(e.call(0x0061_aed0, &args![p]).bool());
        assert!(e.call(0x0061_aef0, &args![p]).bool());
        assert_eq!(e.call(0x0061_b9f0, &args![p]).i32(), -2);
        e.mem.set_u8(p + 0x25, 0x01);
        e.mem.set_u8(p + 0x26, 0x2);
        assert!(!e.call(0x0061_9410, &args![p]).bool());
        assert!(!e.call(0x0061_aed0, &args![p]).bool());
        assert!(!e.call(0x0061_aef0, &args![p]).bool());
    }

    #[test]
    fn b020_constructors() {
        let mut e = Engine::new();
        nop(&mut e, &[0x0061_c120, 0x006b_3f40]);
        let p = e.mem.alloc(0x34);
        e.mem.set_u32(p + 0x2c, 9);
        e.mem.set_u8(p + 0x30, 9);
        e.mem.set_u32(p, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_9740, &args![p]).u32(), p);
        assert_eq!(e.mem.u32(p), 0);
        assert_eq!(e.mem.u32(p + 4), 0x0104_d338);
        assert_eq!(e.mem.u32(p + 0x1c), 0x0104_d458);
        assert_eq!(e.mem.u32(p + 0x2c), 0);
        assert_eq!(e.mem.u8(p + 0x30), 0);
        assert_eq!(calls_to(&e, 0x0061_c120), vec![vec![p + 4, 0, 1]]);
        assert_eq!(calls_to(&e, 0x006b_3f40), vec![vec![p + 0x1c, 0, 0]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_bef0, &args![p, 3u32, 4u32]).u32(), p);
        assert_eq!(e.mem.u32(p), 0x0104_d450);
        assert_eq!(calls_to(&e, 0x0061_c120), vec![vec![p, 3, 4]]);
        assert_eq!(e.call(0x0061_97c0, &args![p]).u32(), p);
        assert_eq!(e.call(0x0061_bf20, &args![p]).u32(), p);
    }

    #[test]
    fn b020_006195d0_finds_or_makes_the_entry() {
        let mut e = engine();
        nop(&mut e, &[0x0061_c120, 0x006b_3f40]);
        e.register(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        let topic = e.mem.alloc(0x48);
        // An unresolved entry for quest id 0x10 and a resolved one whose
        // quest form has id 0x20.
        let unresolved = e.mem.alloc(0x34);
        e.mem.set_u32(unresolved, 0x10);
        let resolved = e.mem.alloc(0x34);
        let quest = e.mem.alloc(0x20);
        e.mem.set_u32(quest + 0xc, 0x20);
        e.mem.set_u32(resolved, quest);
        e.mem.set_u8(resolved + 0x30, 1);
        let second = e.mem.alloc(8);
        e.mem.set_u32(topic + 0x2c, unresolved);
        e.mem.set_u32(topic + 0x30, second);
        e.mem.set_u32(second, resolved);
        assert_eq!(e.call(0x0061_95d0, &args![topic, 0u32, 0u32]).u32(), 0);
        assert_eq!(
            e.call(0x0061_95d0, &args![topic, 0x10u32, 0u32]).u32(),
            unresolved
        );
        assert_eq!(
            e.call(0x0061_95d0, &args![topic, 0x20u32, 0u32]).u32(),
            resolved
        );
        let made = e.call(0x0061_95d0, &args![topic, 0x30u32, 0u32]).u32();
        assert!(made != 0 && made != unresolved && made != resolved);
        assert_eq!(e.mem.u32(made), 0x30);
        assert_eq!(e.mem.u32(made + 4), 0x0104_d338);
        let third = e.mem.u32(second + 4);
        assert_eq!(e.mem.u32(third), made);
    }

    /// A topic with one entry for quest id 0x10 whose info array holds
    /// `infos`; returns (topic, entry).
    fn topic_with_entry(e: &mut Engine, infos: &[u32]) -> (u32, u32) {
        let topic = e.mem.alloc(0x48);
        let entry = e.mem.alloc(0x34);
        e.mem.set_u32(entry, 0x10);
        array_with(e, entry + 4, infos);
        e.mem.set_u32(topic + 0x2c, entry);
        (topic, entry)
    }

    #[test]
    fn b020_00619820_inserts_infos() {
        let mut e = engine();
        nop(
            &mut e,
            &[
                INFO_INSERT,
                0x0061_bdc0,
                0x0061_9fe0,
                0x0061_bd20,
                INFO_SET_INDEX,
                0x0086_4240,
                PRINT,
            ],
        );
        e.register(INFO_INDEX, |e, a| ret(u32::from(e.mem.u16(a[0] + 0x40))));
        let info = e.mem.alloc(0x48);
        e.mem.set_u16(info + 0x40, 0xffff);
        e.mem.set_u32(info + 0xc, 0x55);
        let older = e.mem.alloc(0x48);
        e.mem.set_u32(older + 0xc, 0x44);
        e.mem.set_u16(older + 0x40, 0xffff);
        let (topic, entry) = topic_with_entry(&mut e, &[0, older]);
        assert!(!e.call(0x0061_9820, &args![topic, 0u32, info, 0u32]).bool());
        assert!(!e
            .call(0x0061_9820, &args![topic, 0x10u32, 0u32, 0u32])
            .bool());
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0061_9820, &args![topic, 0x10u32, info, 0u32])
            .bool());
        assert_eq!(calls_to(&e, INFO_INSERT), vec![vec![entry + 4, 0, info]]);
        assert_eq!(calls_to(&e, 0x0061_bdc0), vec![vec![entry + 4]]);
        assert_eq!(calls_to(&e, 0x0061_9fe0), vec![vec![entry + 4]]);
        // After the info with id 0x44 (found at position 1, no index).
        e.call_log = Some(vec![]);
        e.call(0x0061_9820, &args![topic, 0x10u32, info, 0x44u32]);
        assert_eq!(calls_to(&e, INFO_INSERT), vec![vec![entry + 4, 2, info]]);
        assert!(calls_to(&e, 0x0061_bd20).is_empty());
        // With an index, the info is first taken out; the older one has
        // index 3 now.
        e.mem.set_u16(info + 0x40, 7);
        e.mem.set_u16(older + 0x40, 3);
        e.call_log = Some(vec![]);
        e.call(0x0061_9820, &args![topic, 0x10u32, info, 0x44u32]);
        assert_eq!(calls_to(&e, 0x0061_bd20)[0][..2], [entry + 4, 7]);
        assert_eq!(calls_to(&e, INFO_INSERT), vec![vec![entry + 4, 4, info]]);
        // A previous info that is not there: the message and the start.
        e.map(0x011c_b000, 0x1000);
        let last = e.mem.alloc(0x20);
        e.mem.set_u32(last, 0x0150_0000);
        e.mem.set_u32(last + 0xc, 0x77);
        e.mem.set_u32(0x0150_0130, 0x0000_1130);
        e.register(0x0000_1130, |_, _| ret(0x6000));
        e.set_global::<u32>(LAST_LOADED_TOPIC, last);
        e.call_log = Some(vec![]);
        e.call(0x0061_9820, &args![topic, 0x10u32, info, 0x99u32]);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![MISSING_PREVIOUS_INFO, 0x99, 0x55, 0x6000, 0x77]]
        );
        assert_eq!(calls_to(&e, INFO_INSERT), vec![vec![entry + 4, 0, info]]);
        // -1: after the last non-null info, only without an index.
        e.register(ENTRY_INFO, |e, a| {
            let buffer = e.mem.u32(a[0] + 8);
            ret(e.mem.u32(buffer + 4 * a[1]))
        });
        e.call_log = Some(vec![]);
        e.call(0x0061_9820, &args![topic, 0x10u32, info, u32::MAX]);
        assert!(calls_to(&e, 0x0086_4240).is_empty());
        e.mem.set_u16(info + 0x40, 0xffff);
        e.call(0x0061_9820, &args![topic, 0x10u32, info, u32::MAX]);
        assert_eq!(calls_to(&e, INFO_SET_INDEX), vec![vec![info, 2]]);
        assert_eq!(calls_to(&e, 0x0086_4240)[0][..2], [entry + 4, 2]);
    }

    #[test]
    fn b020_00619030_resolves_the_entries() {
        let mut e = engine();
        e.register(0x0040_13e0, |_, _| ret(0));
        nop(
            &mut e,
            &[
                0x0061_bdc0,
                0x0086_4280,
                0x0067_3400,
                INFO_SET_INDEX,
                0x0048_4ab0,
                0x0000_1088,
                RECORD_REMOVE,
            ],
        );
        e.register(ARRAY_COUNT, |_, _| ret(0));
        e.register(FORM_BY_ID, |_, a| ret(a[0] + 0x0100_0000));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.mem.set_u32(0x0150_0088, 0x0000_1088);
        let info = e.mem.alloc(0x48);
        e.mem.set_u32(info, 0x0150_0000);
        let (topic, entry) = topic_with_entry(&mut e, &[info]);
        e.call_log = Some(vec![]);
        e.call(0x0061_9030, &args![topic]);
        assert_eq!(e.mem.u32(entry), 0x0100_0010);
        assert_eq!(e.mem.u8(entry + 0x30), 1);
        assert_eq!(calls_to(&e, 0x0067_3400), vec![vec![info, 0x0100_0010]]);
        assert_eq!(calls_to(&e, 0x0000_1088), vec![vec![info]]);
        assert_eq!(calls_to(&e, INFO_SET_INDEX), vec![vec![info, 0]]);
        assert_eq!(calls_to(&e, 0x0086_4280), vec![vec![entry + 4, 1]]);
        assert_eq!(calls_to(&e, 0x0048_4ab0), vec![vec![topic, 1]]);
        // Once flagged, nothing.
        e.register(0x0040_13e0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0061_9030, &args![topic]);
        assert!(calls_to(&e, 0x0048_4ab0).is_empty());
    }

    #[test]
    fn b020_00619030_reports_quest_less_entries_and_drops_links() {
        let mut e = engine();
        e.register(0x0040_13e0, |_, _| ret(0));
        nop(
            &mut e,
            &[
                0x0061_bdc0,
                0x0086_4280,
                0x0067_3400,
                INFO_SET_INDEX,
                0x0048_4ab0,
                0x0000_1088,
                PRINT,
                0x0000_1130,
            ],
        );
        e.mem.set_u32(0x0150_0088, 0x0000_1088);
        e.mem.set_u32(0x0150_0130, 0x0000_1130);
        e.register(0x0000_1130, |_, _| ret(0x6000));
        e.register(0x0044_0d80, |_, _| ret(0));
        // Two link records: one whose info is missing, one already listed.
        let records = e.mem.alloc(0x18);
        let info = e.mem.alloc(0x48);
        e.mem.set_u32(info, 0x0150_0000);
        e.mem.set_u32(records, 0x5);
        e.mem.set_u32(records + 0xc, info);
        let count = std::rc::Rc::new(std::cell::Cell::new(2u32));
        let c = count.clone();
        e.register_double(ARRAY_COUNT, move |_, _| ret(c.get()));
        let c = count;
        e.register_double(RECORD_REMOVE, move |e, a| {
            // Later records move down one place.
            for i in a[1]..1 {
                let bytes = e.mem.bytes(records + 0xc * (i + 1), 0xc);
                e.mem.write(records + 0xc * i, &bytes);
            }
            c.set(c.get() - 1);
            Ret::default()
        });
        e.register_double(RECORD_ADDRESS, move |_, a| ret(records + 0xc * a[1]));
        e.register(FORM_BY_ID, |_, a| ret(if a[0] == 5 { 0 } else { a[0] }));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        let topic = e.mem.alloc(0x48);
        e.mem.set_u32(topic, 0x0150_0000);
        let entry = e.mem.alloc(0x34);
        array_with(&mut e, entry + 4, &[info]);
        e.mem.set_u32(topic + 0x2c, entry);
        e.call_log = Some(vec![]);
        e.call(0x0061_9030, &args![topic]);
        assert_eq!(calls_to(&e, RECORD_REMOVE).len(), 2);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![NO_QUEST_REFERENCE, 0x6000, 1]]
        );
    }

    #[test]
    fn b020_0061a450_orders_topics() {
        let mut e = engine();
        e.mem.set_u32(0x0150_0130, 0x0000_1130);
        e.register(0x0000_1130, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        e.register(0x0040_4dc0, |e, a| {
            let x = e.mem.u8(a[0]) as i32;
            let y = e.mem.u8(a[1]) as i32;
            ret((x - y) as u32)
        });
        let topic = |e: &mut Engine, name: u8, priority: f32| {
            let t = e.mem.alloc(0x48);
            e.mem.set_u32(t, 0x0150_0000);
            let n = e.mem.alloc(4);
            e.mem.set_u8(n, name);
            e.mem.set_u32(t + 0x40, n);
            e.mem.set_f32(t + 0x28, priority);
            t
        };
        let a = topic(&mut e, b'a', 1.0);
        let b = topic(&mut e, b'b', 1.0);
        let a2 = topic(&mut e, b'a', 1.0);
        assert!(e.call(0x0061_a450, &args![a, b, 0u32]).bool());
        // Not a smaller first character: `00404dc0(name a, name b) > 0`.
        assert!(e.call(0x0061_a450, &args![b, a, 0u32]).bool());
        assert!(!e.call(0x0061_a450, &args![a, a2, 0u32]).bool());
        assert!(!e.call(0x0061_a450, &args![0u32, a, 0u32]).bool());
        // By priority: b - a above the limit gives true.
        e.map(0x0101_7000, 0x1000);
        e.map(0x0103_0000, 0x1000);
        e.set_global::<f64>(0x0103_0f28, 0.0);
        e.set_global::<f32>(0x0101_7d00, 0.001);
        e.register(0x0040_eb10, |_, a| {
            ret(u32::from(
                f32::from_bits(a[0]).abs() <= f32::from_bits(a[2]),
            ))
        });
        e.mem.set_f32(b + 0x28, 2.0);
        assert!(e.call(0x0061_a450, &args![a, b, 1u32]).bool());
        assert!(!e.call(0x0061_a450, &args![b, a, 1u32]).bool());
        e.mem.set_f32(b + 0x28, 1.0);
        assert!(e.call(0x0061_a450, &args![a, b, 1u32]).bool());
    }

    #[test]
    fn b020_0061a380_and_0061a5a0_keep_the_list_sorted() {
        let mut e = engine();
        e.mem.set_u32(0x0150_0130, 0x0000_1130);
        e.register(0x0000_1130, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        e.register(0x0040_4dc0, |_, _| ret(0));
        nop(&mut e, &[LIST_CONSTRUCT, LIST_DESTROY]);
        let topic = |e: &mut Engine, name: u8| {
            let t = e.mem.alloc(0x48);
            e.mem.set_u32(t, 0x0150_0000);
            let n = e.mem.alloc(4);
            e.mem.set_u8(n, name);
            e.mem.set_u32(t + 0x40, n);
            t
        };
        let a = topic(&mut e, b'a');
        let b = topic(&mut e, b'b');
        let c = topic(&mut e, b'c');
        let list = e.mem.alloc(8);
        let names = |e: &Engine, list: u32| {
            let mut out = vec![];
            let mut node = list;
            while node != 0 && e.mem.u32(node) != 0 {
                out.push(e.mem.u32(node));
                node = e.mem.u32(node + 4);
            }
            out
        };
        e.call(0x0061_a380, &args![b, list, 0u32]);
        e.call(0x0061_a380, &args![c, list, 0u32]);
        e.call(0x0061_a380, &args![a, list, 0u32]);
        e.call(0x0061_a380, &args![a, list, 0u32]);
        e.call(0x0061_a380, &args![0u32, list, 0u32]);
        // `0061a450(current, new)` puts the new topic before the first
        // topic that sorts before it.
        assert_eq!(names(&e, list), vec![c, b, a]);
        e.call(0x0061_a5a0, &args![0u32, list]);
        assert_eq!(names(&e, list), vec![c, b, a]);
        // Inserting b before a, and sorting [a, b], both give [b, a].
        let other = e.mem.alloc(8);
        e.call(0x0061_a380, &args![a, other, 0u32]);
        e.call(0x0061_a380, &args![b, other, 0u32]);
        assert_eq!(names(&e, other), vec![b, a]);
        let mixed = e.mem.alloc(8);
        let tail = e.mem.alloc(8);
        e.mem.set_u32(mixed, a);
        e.mem.set_u32(mixed + 4, tail);
        e.mem.set_u32(tail, b);
        e.call(0x0061_a5a0, &args![0u32, mixed]);
        assert_eq!(names(&e, mixed), vec![b, a]);
    }

    /// One topic with one entry (quest `0x0170_0000`) holding `info`, the
    /// quest passing its checks; `0061e600` accepts every info.
    fn picking_world(e: &mut Engine) -> (u32, u32) {
        nop(
            e,
            &[
                0x0061_c120,
                0x0061_8920,
                LIST_CONSTRUCT,
                LIST_DESTROY,
                0x0047_0470,
            ],
        );
        e.register(0x0061_9df0, |_, _| ret(0));
        e.register(0x009e_32d0, |_, _| ret(0));
        e.register(0x0044_0d80, |_, _| ret(0));
        e.register(0x0045_5620, |_, _| ret(1));
        e.register(0x005f_5760, |_, _| ret(0x0170_0100));
        e.register(0x0068_0c30, |_, _| ret(1));
        e.register(ARRAY_COUNT, |_, _| ret(0));
        e.register(0x0086_3d90, |e, a| {
            let mut buffer = e.mem.u32(a[0] + 4);
            if buffer == 0 {
                buffer = e.mem.alloc(0x40);
                e.mem.set_u32(a[0] + 4, buffer);
            }
            let count = e.mem.u32(a[0] + 0xc);
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(buffer + 4 * count, value);
            e.mem.set_u32(a[0] + 0xc, count + 1);
            Ret::default()
        });
        e.register(0x0061_e600, |e, a| {
            e.mem.set_u8(a[1], 0);
            ret(1)
        });
        let info = e.mem.alloc(0x50);
        let (topic, _) = topic_with_entry(e, &[info]);
        let entry = e.mem.u32(topic + 0x2c);
        e.mem.set_u32(entry, 0x0170_0000);
        (topic, info)
    }

    #[test]
    fn b020_0061a7d0_picks_the_first_accepted_info() {
        let mut e = engine();
        let (topic, info) = picking_world(&mut e);
        let said = e.mem.alloc(4);
        e.mem.set_u8(said, 9);
        assert_eq!(
            e.call(
                0x0061_a7d0,
                &args![topic, said, 0x10u32, 0x20u32, 0u32, 0u32, 0u32]
            )
            .u32(),
            info
        );
        assert_eq!(e.mem.u8(said), 0);
        assert_eq!(
            e.call(
                0x0061_a7d0,
                &args![0u32, said, 0x10u32, 0x20u32, 0u32, 0u32, 0u32]
            )
            .u32(),
            0
        );
        // A refused condition: no info.
        e.register(0x0068_0c30, |_, _| ret(0));
        assert_eq!(
            e.call(
                0x0061_a7d0,
                &args![topic, said, 0x10u32, 0x20u32, 0u32, 0u32, 0u32]
            )
            .u32(),
            0
        );
        // 0061a760 and 0061a790 forward to it.
        e.register(0x0068_0c30, |_, _| ret(1));
        assert_eq!(
            e.call(0x0061_a760, &args![topic, said, 0x10u32, 0x20u32])
                .u32(),
            info
        );
        assert_eq!(
            e.call(
                0x0061_a790,
                &args![topic, said, 0x10u32, 0x20u32, 0u32, 0u32, 0u32]
            )
            .u32(),
            info
        );
    }

    #[test]
    fn b020_0061a7d0_random_choice_among_flagged_infos() {
        let mut e = engine();
        let (topic, info) = picking_world(&mut e);
        // Bit 0x2 of +0x25: the info goes to the candidates.
        e.mem.set_u8(info + 0x25, 2);
        e.register(0x0048_7f50, |_, _| ret(7));
        let said = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x0061_a7d0,
                &args![topic, said, 0x10u32, 0x20u32, 0u32, 0u32, 0u32]
            )
            .u32(),
            info
        );
        assert_eq!(calls_to(&e, 0x0048_7f50).len(), 1);
        assert_eq!(calls_to(&e, 0x0047_0470).len(), 1);
    }

    #[test]
    fn b020_0061af30_picks_among_linked_infos() {
        let mut e = engine();
        let (_, info) = picking_world(&mut e);
        let first = e.mem.alloc(0x50);
        let links = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, info);
        e.register_double(0x009e_32d0, move |_, _| ret(links));
        e.register_double(0x0046_0140, move |_, _| ret(node));
        e.register(0x0067_33e0, |_, _| ret(0x0170_0000));
        let said = e.mem.alloc(4);
        assert_eq!(
            e.call(
                0x0061_af30,
                &args![first, said, 0x10u32, 0x20u32, 0u32, 0u32]
            )
            .u32(),
            info
        );
        // Listed in `exclude` (by id): nothing.
        let exclude = e.mem.alloc(8);
        let excluded = e.mem.alloc(0x10);
        e.mem.set_u32(excluded + 0xc, info);
        e.mem.set_u32(exclude, excluded);
        assert_eq!(
            e.call(
                0x0061_af30,
                &args![first, said, 0x10u32, 0x20u32, 0u32, exclude]
            )
            .u32(),
            0
        );
        assert_eq!(
            e.call(
                0x0061_af30,
                &args![0u32, said, 0x10u32, 0x20u32, 0u32, 0u32]
            )
            .u32(),
            0
        );
    }

    #[test]
    fn b020_0061b320_makes_a_dialogue_item() {
        let mut e = engine();
        let (topic, info) = picking_world(&mut e);
        e.register(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0067_33e0, |_, _| ret(0x4848));
        e.register(0x0083_c520, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[3]);
            ret(a[0])
        });
        e.register(0x0057_9200, |_, _| ret(1));
        nop(&mut e, &[0x0061_f170, 0x0061_f150]);
        e.call_log = Some(vec![]);
        let item = e
            .call(
                0x0061_b320,
                &args![topic, 0x10u32, 0x20u32, 0u32, 0u32, 1u32],
            )
            .u32();
        assert_ne!(item, 0);
        assert_eq!(
            calls_to(&e, 0x0083_c520),
            vec![vec![item, 0x4848, topic, info, 0x10]]
        );
        assert_eq!(calls_to(&e, 0x0061_f170), vec![vec![info, 0, 0x10]]);
        assert_eq!(calls_to(&e, 0x0061_f150), vec![vec![info]]);
        e.register(0x0068_0c30, |_, _| ret(0));
        assert_eq!(
            e.call(
                0x0061_b320,
                &args![topic, 0x10u32, 0x20u32, 0u32, 0u32, 1u32]
            )
            .u32(),
            0
        );
    }

    #[test]
    fn b020_0061b440_builds_a_conversation() {
        let mut e = engine();
        let (topic, info) = picking_world(&mut e);
        e.register(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0067_33e0, |_, _| ret(0));
        e.register(0x0083_c520, |_, a| ret(a[0]));
        e.register(0x007a_f430, |_, _| ret(0));
        e.register(0x0061_b9d0, |_, _| ret(7));
        e.register(0x005a_e380, |e, a| {
            let mut n = 0;
            let mut node = a[0];
            while node != 0 && e.mem.u32(node) != 0 {
                n += 1;
                node = e.mem.u32(node + 4);
            }
            ret(n)
        });
        nop(&mut e, &[0x00ec_ada6, 0x005c_90d0, 0x0040_3d30]);
        e.map(0x011c_b000, 0x1000);
        let list = e.mem.alloc(8);
        // An old item is deleted first.
        e.mem.set_u32(list, 0x0000_7777);
        assert_eq!(
            e.call(0x0061_b440, &args![0x10u32, 0x20u32, 0u32, topic])
                .u32(),
            0
        );
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0061_b440, &args![0u32, 0x20u32, list, topic])
                .u32(),
            0
        );
        assert_eq!(calls_to(&e, 0x005c_90d0), vec![vec![0x7777, 1]]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0061_b440, &args![0x10u32, 0x20u32, list, topic])
                .u32(),
            1
        );
        let item = e.mem.u32(list);
        assert_ne!(item, 0);
        assert_eq!(
            calls_to(&e, 0x0083_c520),
            vec![vec![item, 0, topic, info, 0x10]]
        );
        assert_eq!(calls_to(&e, 0x00ec_ada6), vec![vec![7]]);
        let cleared = calls_to(&e, 0x0040_3d30);
        assert_eq!(cleared.len(), 1);
        assert_eq!(&cleared[0][1..], &[0, 0x7d0]);
    }
}

#[cfg(test)]
mod n045_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A topic whose vtable slot 0x3C answers `answer`; its form id is `id`.
    fn topic(e: &mut Engine, id: u32) -> u32 {
        let t = e.mem.alloc(0x40);
        let vt = e.mem.alloc(0x40);
        e.mem.set_u32(t, vt);
        e.mem.set_u32(vt + 0x3c, 0x0050_0000);
        e.mem.set_u32(t + 0xc, id);
        t
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(WORD_AT_0XC, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(FORM_TYPE_OF, |e, a| ret(u32::from(e.mem.u8(a[0] + 4))));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(0x0050_0000, |_, a| ret(u32::from(a[1] == 0x5555)));
        e
    }

    #[test]
    fn fn_00619430_orders_topics_and_owns_infos() {
        let mut e = rig();
        let this = topic(&mut e, 10);
        let other = topic(&mut e, 20);
        e.mem.set_u8(other + 4, 0x45);
        assert!(e.call(0x0061_9430, &[this, other]).bool());
        e.mem.set_u32(other + 0xc, 5);
        assert!(!e.call(0x0061_9430, &[this, other]).bool());
        // An info without an owner topic.
        e.map(0x011c_3000, 0x1000);
        e.register(LIST_AT_0X108, |_, _| ret(0));
        let info = e.mem.alloc(0x20);
        e.mem.set_u8(info + 4, 0x46);
        assert!(!e.call(0x0061_9430, &[this, info]).bool());
        // Anything else asks 00484020.
        e.register(FORM_ANSWER_484020, |_, _| ret(1));
        let form = e.mem.alloc(0x20);
        e.mem.set_u8(form + 4, 0x10);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0061_9430, &[this, form]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, FORM_ANSWER_484020), vec![vec![this, form]]);
    }

    #[test]
    fn fn_00619510_answers_by_the_object_kind() {
        let mut e = rig();
        e.map(0x0118_7000, 0x1000);
        e.set_global::<u32>(OBJECT_VTABLE_WORD, 0x0300);
        let this = topic(&mut e, 1);
        let object = e.mem.alloc(0x10);
        assert!(!e.call(0x0061_9510, &[this, 0]).bool());
        assert!(!e.call(0x0061_9510, &[this, object]).bool());
        e.mem.set_u32(object, 0x0300);
        e.register(FORM_ANSWER_484150, |_, _| ret(1));
        assert!(e.call(0x0061_9510, &[this, object]).bool());
        e.mem.set_u32(object + 0xc, 3);
        assert!(!e.call(0x0061_9510, &[this, object]).bool());
        e.mem.set_u32(object + 0xc, 7);
        e.mem.set_u32(object + 8, 0x77);
        e.register(FORM_BY_ID, |_, a| {
            ret(if a[0] == 0x77 { 0x5555 } else { 0 })
        });
        assert!(e.call(0x0061_9510, &[this, object]).bool());
        e.mem.set_u32(object + 8, 0x78);
        assert!(!e.call(0x0061_9510, &[this, object]).bool());
        e.mem.set_u32(object + 0xc, 8);
        assert!(!e.call(0x0061_9510, &[this, object]).bool());
    }

    #[test]
    fn deleting_destructors_delete_with_flag_1() {
        let mut e = Engine::new();
        e.register(DESTRUCT_618920, ok);
        e.register(DESTRUCT_61BF50, ok);
        e.register(OPERATOR_DELETE, ok);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_97f0, &[0x5000, 0]).u32(), 0x5000);
        assert_eq!(e.call(0x0061_97f0, &[0x5000, 1]).u32(), 0x5000);
        assert_eq!(e.call(0x0061_c0f0, &[0x6000, 3]).u32(), 0x6000);
        assert_eq!(e.call(0x0061_c0f0, &[0x6000, 2]).u32(), 0x6000);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, DESTRUCT_618920).len(), 2);
        assert_eq!(only(&log, DESTRUCT_61BF50).len(), 2);
        assert_eq!(
            only(&log, OPERATOR_DELETE),
            vec![vec![0x5000], vec![0x6000]]
        );
    }

    #[test]
    fn get_matching_info_and_the_flag_byte() {
        let mut e = Engine::new();
        e.register(0x0061_9df0, |_, _| ret(0));
        e.register(0x0044_0d80, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_a720, &[0x5000, 1, 2]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0044_0d80), vec![vec![0x5000]]);
        e.register(BYTE_3C_ADDRESS, |_, a| ret(a[0] + 0x3c));
        let t = e.mem.alloc(0x40);
        assert!(!e.call(0x0061_af10, &[t]).bool());
        e.mem.set_u8(t + 0x3c, 4);
        assert!(e.call(0x0061_af10, &[t]).bool());
    }
}

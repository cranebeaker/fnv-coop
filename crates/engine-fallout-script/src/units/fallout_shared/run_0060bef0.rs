//! `fallout shared/run_0060bef0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The constructor of `TESQuest` (`0060c4b0`; the engine map gives it no name,
//! but `CreateFormOfType` allocates 0x6c bytes for form type 0x47 and calls it)
//! and the small members it calls that sit in this run.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::TESForm` (`00483370`).
const TES_FORM_CONSTRUCT: u32 = 0x0048_3370;
/// `TESScriptableForm` base constructor (`0048cc50`).
const SCRIPTABLE_FORM_CONSTRUCT: u32 = 0x0048_cc50;
/// `TESIcon` base constructor (`00402c20`).
const ICON_CONSTRUCT: u32 = 0x0040_2c20;
/// `TESFullName` base constructor (`00402d00`).
const FULL_NAME_CONSTRUCT: u32 = 0x0040_2d00;
/// The two-word list node constructor (`0096a2d0`: item and next, both 0).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The condition list constructor (`00680890`, a call of `0096a2d0`).
const CONDITION_LIST_CONSTRUCT: u32 = 0x0068_0890;
/// The `BSStringT<char>` constructor (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// Writes `TESForm::cFormType` (`004f15a0`, one stack word).
const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// `this + 0x3c`, the address of the quest data (`005a8080`).
const QUEST_DATA_ADDRESS: u32 = 0x005a_8080;

/// `TESQuest`'s vtable and those of its three other bases.
const TES_QUEST_VTABLE: u32 = 0x0104_ac44;
const SCRIPTABLE_FORM_VTABLE: u32 = 0x0104_ac2c;
const ICON_VTABLE: u32 = 0x0104_ac0c;
const FULL_NAME_VTABLE: u32 = 0x0104_abf8;
/// The form type of a quest (`QUST`).
const FORM_TYPE_QUEST: u8 = 0x47;
/// Byte offset of the vtable slot the enable setter calls to flag a change
/// (`AddChange` in the Xbox PDB's `TESForm` vtable).
const ADD_CHANGE_SLOT: u32 = 0x48;
/// The change flag the enable setter passes.
const CHANGE_QUEST_FLAGS: u32 = 2;

layout! {
    /// `QUEST_DATA` (Xbox PDB), 8 bytes.
    pub struct QuestData: 0x08 {
        /// `ucFlags` (Xbox PDB): bit 0 is "enabled".
        0x00 ucFlags: u8,
        /// `ucPriority` (Xbox PDB).
        0x01 ucPriority: u8,
        /// `fQuestDelayTime` (Xbox PDB).
        0x04 fQuestDelayTime: f32,
    }

    /// `TESQuest` (Xbox PDB), 0x6c bytes; the same offsets on PC.
    pub struct TESQuest: 0x6c {
        /// `pScriptLocals` (Xbox PDB): `ScriptLocals*`.
        0x5C pScriptLocals: Ptr,
        /// `cCurrentStage` (Xbox PDB).
        0x60 cCurrentStage: u8,
    }
}

/// Offsets in a `TESQuest` (Xbox PDB names): `data` (a [`QuestData`]),
/// `m_listStages`, `m_listObjectives`, `objConditions` and `cFormEditorID`.
const DATA: u32 = 0x3c;
const LIST_STAGES: u32 = 0x44;
const LIST_OBJECTIVES: u32 = 0x4c;
const CONDITIONS: u32 = 0x54;
const EDITOR_ID: u32 = 0x64;

// Translated from 0060c4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESQuest`: constructs the `TESForm` base and the
/// `TESScriptableForm` (+0x18), `TESIcon` (+0x24) and `TESFullName` (+0x30)
/// bases, sets the four vtables, constructs the quest data (+0x3c), the stage
/// and objective lists (+0x44, +0x4c), the condition list (+0x54) and the
/// editor ID string (+0x64), sets the form type to 0x47, clears the script
/// locals (+0x5c) and finishes with `0060c740` (quest enabled, current stage
/// 0). Returns `this`.
// Translation incomplete: 0060c4b0 - SEH exception unwinding and the stack security cookie are unsupported.
pub fn fn_0060c4b0(e: &mut Engine, this: Ptr<TESQuest>) -> Ptr<TESQuest> {
    let base = this.addr();
    e.call(TES_FORM_CONSTRUCT, &args![this]);
    e.call(SCRIPTABLE_FORM_CONSTRUCT, &args![base + 0x18]);
    e.call(ICON_CONSTRUCT, &args![base + 0x24]);
    e.call(FULL_NAME_CONSTRUCT, &args![base + 0x30]);
    e.mem.set_u32(base, TES_QUEST_VTABLE);
    e.mem.set_u32(base + 0x18, SCRIPTABLE_FORM_VTABLE);
    e.mem.set_u32(base + 0x24, ICON_VTABLE);
    e.mem.set_u32(base + 0x30, FULL_NAME_VTABLE);
    fn_0060c5c0(e, Ptr::new(base + DATA));
    e.call(LIST_CONSTRUCT, &args![base + LIST_STAGES]);
    e.call(LIST_CONSTRUCT, &args![base + LIST_OBJECTIVES]);
    e.call(CONDITION_LIST_CONSTRUCT, &args![base + CONDITIONS]);
    e.call(STRING_CONSTRUCT, &args![base + EDITOR_ID]);
    e.call(SET_FORM_TYPE, &args![this, FORM_TYPE_QUEST as u32]);
    e.set(this, TESQuest::pScriptLocals, Ptr::NULL);
    fn_0060c740(e, this);
    this
}

// Translated from 0060c5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `QUEST_DATA`: flags 0, priority 0, delay time 0.0.
pub fn fn_0060c5c0(e: &mut Engine, this: Ptr<QuestData>) -> Ptr<QuestData> {
    e.set(this, QuestData::ucFlags, 0);
    e.set(this, QuestData::ucPriority, 0);
    e.set(this, QuestData::fQuestDelayTime, 0.0);
    this
}

// Translated from 0060c740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the quest enabled (`0060c9c0` with 1, which also flags the change)
/// and sets the current stage (`cCurrentStage`) to 0.
pub fn fn_0060c740(e: &mut Engine, this: Ptr<TESQuest>) {
    fn_0060c9c0(e, this, 1);
    e.set(this, TESQuest::cCurrentStage, 0);
}

// Translated from 0060c9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`enable` non-zero) or clears bit 0 of the quest data's flags
/// (found through `005a8080`), then calls the vtable slot at `0x48` with 2
/// (the Xbox PDB's `TESForm::AddChange` with the quest-flags change; the
/// slot's PC body is not checked here). `RET 4`: one byte-sized argument.
pub fn fn_0060c9c0(e: &mut Engine, this: Ptr<TESQuest>, enable: u8) {
    let data: Ptr<QuestData> = e.call_as(QUEST_DATA_ADDRESS, &args![this]);
    let flags = e.get(data, QuestData::ucFlags);
    let flags = if enable != 0 { flags | 1 } else { flags & 0xfe };
    e.set(data, QuestData::ucFlags, flags);
    e.vcall(this.addr(), ADD_CHANGE_SLOT, &args![CHANGE_QUEST_FLAGS]);
}

// Translated from 0060c440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0xdc (`SUB ECX,0xdc`) and jumps to `00601370`. `00601370` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0060c440(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0060_1370, &args![this.addr().wrapping_sub(0xdc), flags])
        .u32()
}

// Translated from 0060c450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `00608da0`. `00608da0` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0060c450(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x0060_8da0,
        &args![this.addr().wrapping_sub(0x7c), argument],
    )
    .u16()
}

// Translated from 0060c460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x90 (`SUB ECX,0x90`) and jumps to `00608e80`. `00608e80` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_0060c460(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0060_8e80,
        &args![this.addr().wrapping_sub(0x90), first, second],
    );
}

// Translated from 0060c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x90 (`SUB ECX,0x90`) and jumps to `00608e00`. `00608e00` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0060c470(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0060_8e00,
        &args![this.addr().wrapping_sub(0x90), argument],
    );
}

// Translated from 0060c480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `00608e80`. `00608e80` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_0060c480(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0060_8e80,
        &args![this.addr().wrapping_sub(0x7c), first, second],
    );
}

// Translated from 0060c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x90 (`SUB ECX,0x90`) and jumps to `00608da0`. `00608da0` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0060c490(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x0060_8da0,
        &args![this.addr().wrapping_sub(0x90), argument],
    )
    .u16()
}

// Translated from 0060c4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `00608e00`. `00608e00` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0060c4a0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0060_8e00,
        &args![this.addr().wrapping_sub(0x7c), argument],
    );
}

/// The iterator helpers of the copy and fill templates: `0065e750` (`cdecl`,
/// two iterator addresses; a byte tag), `0065fe40` (`cdecl`, one iterator
/// address; the iterator's pointer) and `0065fd70` (`cdecl`, two words: a
/// byte's address and an iterator's; answers the byte's address).
const ITERATOR_CATEGORY_0065E750: u32 = 0x0065_e750;
const ITERATOR_BASE_0065FE40: u32 = 0x0065_fe40;
const ITERATOR_TAG_0065FD70: u32 = 0x0065_fd70;
/// CRT `memmove_s` (`00ec7ce3`, `cdecl`: destination, its size, source,
/// count).
const MEMMOVE_S: u32 = 0x00ec_7ce3;

// Translated from 0060c3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word-copy step of a copy template (`cdecl`,
/// six words as its caller `0060c320` pushes them: first, last, destination
/// and three tag words, not read). Moves `(last - first) >> 2` words
/// (arithmetic shift) with `memmove_s` when there are any, and answers the
/// destination past them.
#[allow(clippy::too_many_arguments)] // the caller's six stack words
pub fn fn_0060c3c0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let count = ((last.wrapping_sub(first) as i32) >> 2) as u32;
    let end = destination.wrapping_add(count.wrapping_mul(4));
    if count != 0 {
        let bytes = count.wrapping_shl(2);
        e.call(MEMMOVE_S, &args![destination, bytes, first, bytes]);
    }
    end
}

// Translated from 0060c320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a copy template (`cdecl`, four words: first,
/// last, destination, a fourth word). Asks the category of the first and
/// destination iterators (`0065e750` with their addresses), unwraps last and
/// first (`0065fe40` with their addresses, last first) and copies through
/// `0060c3c0(first, last, destination, fourth, category, 0)`, answering its
/// value (the destination past the copy: the exe ends on that call and leaves
/// its `EAX`).
pub fn fn_0060c320(e: &mut Engine, first: u32, last: u32, destination: u32, fourth: u32) -> u32 {
    e.with_stack(0x10, |e, frame| {
        // The argument slots the exe passes by address: first (+0), last
        // (+4), destination (+8).
        let slots = frame.addr();
        e.mem.set_u32(slots, first);
        e.mem.set_u32(slots + 4, last);
        e.mem.set_u32(slots + 8, destination);
        let category = e
            .call(ITERATOR_CATEGORY_0065E750, &args![slots, slots + 8])
            .u32() as u8;
        let last_base = e.call(ITERATOR_BASE_0065FE40, &args![slots + 4]).u32();
        let first_base = e.call(ITERATOR_BASE_0065FE40, &args![slots]).u32();
        let destination = e.mem.u32(slots + 8);
        fn_0060c3c0(
            e,
            first_base,
            last_base,
            destination,
            fourth,
            u32::from(category),
            0,
        )
    })
}

// Translated from 0060c270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: forwards its first four words to `0060c320`
/// (`cdecl`; its callers push six words, the last two not read) and answers
/// its value (the exe ends on that call and leaves its `EAX`).
#[allow(clippy::too_many_arguments)] // the callers' six stack words
pub fn fn_0060c270(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    fourth: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    fn_0060c320(e, first, last, destination, fourth)
}

// Translated from 0060c410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a float fill (`cdecl`, four words: the
/// destination, the count, the address of the float, a tag word not read):
/// `count` floats copied through `FLD`/`FSTP` (a signalling NaN is quieted).
pub fn fn_0060c410(e: &mut Engine, destination: u32, count: u32, value: u32, _unused_3: u32) {
    let mut at = destination;
    let mut left = count;
    while left != 0 {
        let bits = x87_quiet_bits(e.mem.u32(value));
        e.mem.set_u32(at, bits);
        left -= 1;
        at = at.wrapping_add(4);
    }
}

// Translated from 0060c380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a fill template (`cdecl`, five words: the
/// destination, the count, the value's address and two tag words not read):
/// unwraps the destination (`0065fe40` with its address) and fills through
/// `0060c410(destination, count, value, 0)`.
pub fn fn_0060c380(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
    let base = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), destination);
        e.call(ITERATOR_BASE_0065FE40, &args![slot]).u32()
    });
    fn_0060c410(e, base, count, value, 0);
}

// Translated from 0060c2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a fill entry (`cdecl`, three words: the
/// destination, the count and the value's address). Gets a tag byte
/// (`0065fd70` with a stack byte and the destination's address) and fills
/// through `0060c380(destination, count, value, tag, 0)`.
pub fn fn_0060c2e0(e: &mut Engine, destination: u32, count: u32, value: u32) {
    let (tag, destination) = e.with_stack(8, |e, frame| {
        let slot = frame.addr();
        e.mem.set_u32(slot, destination);
        // Three words go to 0065fd70: the tag byte's address, the destination's, and the
        // zero byte pushed first (0060c2ed; it stays on the stack for 0060c380 as its fifth).
        let at = e
            .call(ITERATOR_TAG_0065FD70, &args![slot + 4, slot, 0u32])
            .u32();
        (e.mem.u8(at), e.mem.u32(slot))
    });
    fn_0060c380(e, destination, count, value, u32::from(tag), 0);
}

/// The quest-objective list helpers: `005d43c0` and `005d43e0` (`thiscall` on
/// the quest; the address of a list), `008256d0` (`thiscall` on the list;
/// whether it is empty), `006815c0` (`thiscall`; the address of the head's
/// item), `0063f7b0` (`thiscall`; removes the head) and `00726070`
/// (`thiscall`; a node's or item's second word).
const QUEST_LIST_005D43C0: u32 = 0x005d_43c0;
const QUEST_LIST_005D43E0: u32 = 0x005d_43e0;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_HEAD_ITEM: u32 = 0x0068_15c0;
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
const LINK_00726070: u32 = 0x0072_6070;
/// `0060f260` (`thiscall`, no stack words): the stage's destructor body, and
/// the free `00401030` (`cdecl`, one word).
const STAGE_DESTRUCT: u32 = 0x0060_f260;
const FREE_00401030: u32 = 0x0040_1030;

// Translated from 0060c840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a scalar deleting destructor (`thiscall`,
/// `RET 4`: the flags): the body `0060f260`, then the free `00401030` with
/// bit 0. Answers `this`.
pub fn fn_0060c840(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(STAGE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_00401030, &args![this]);
    }
    this
}

// Translated from 0060c7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a quest): empties the list from
/// `005d43c0` when there is one: each head item (when not null) is deleted
/// through `0060c840` with 1 and the head removed (`0063f7b0`), until the list
/// is empty (`008256d0`).
pub fn fn_0060c7d0(e: &mut Engine, this: Ptr) {
    let list = e.call(QUEST_LIST_005D43C0, &args![this]).u32();
    if list != 0 {
        while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
            let at = e.call(LIST_HEAD_ITEM, &args![list]).u32();
            let item = e.mem.u32(at);
            if item != 0 {
                fn_0060c840(e, Ptr::new(item), 1);
            }
            e.call(LIST_REMOVE_HEAD, &args![list]);
        }
    }
}

// Translated from 0060c870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a quest): empties the list from
/// `005d43e0` when there is one: each head item (when not null) is deleted
/// through its vtable slot 0 with 1 and the head removed, until the list is
/// empty.
pub fn fn_0060c870(e: &mut Engine, this: Ptr) {
    let list = e.call(QUEST_LIST_005D43E0, &args![this]).u32();
    if list != 0 {
        while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
            let at = e.call(LIST_HEAD_ITEM, &args![list]).u32();
            let item = e.mem.u32(at);
            if item != 0 {
                e.vcall(item, 0, &args![1u32]);
            }
            e.call(LIST_REMOVE_HEAD, &args![list]);
        }
    }
}

// Translated from 0060c8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a quest, `RET 4`: a key): walks
/// the list from `005d43e0` (stopping at a null node or an empty list) and
/// answers the first item whose `00726070` word equals the key; the next node
/// is the node's `00726070` word. 0 when none.
pub fn fn_0060c8e0(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let mut node = e.call(QUEST_LIST_005D43E0, &args![this]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let at = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        let item = e.mem.u32(at);
        if item != 0 && e.call(LINK_00726070, &args![item]).u32() == key {
            return item;
        }
        node = e.call(LINK_00726070, &args![node]).u32();
    }
    0
}

/// `005f5760` (`thiscall` on the quest; the condition list's address),
/// `00680fe0` (`thiscall` on that list; empties it) and `0041af70`
/// (`thiscall`, `RET 4`: the script locals' scalar deleting destructor).
const QUEST_CONDITIONS: u32 = 0x005f_5760;
const CONDITIONS_CLEAR: u32 = 0x0068_0fe0;
const SCRIPT_LOCALS_DELETE: u32 = 0x0041_af70;

// Translated from 0060c760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a quest): empties the quest's
/// conditions (`005f5760`, `00680fe0`), its two lists (`0060c7d0`,
/// `0060c870`) and deletes its script locals (`pScriptLocals` +0x5c, through
/// `0041af70` with 1), clearing the pointer.
pub fn fn_0060c760(e: &mut Engine, this: Ptr) {
    let conditions = e.call(QUEST_CONDITIONS, &args![this]).u32();
    e.call(CONDITIONS_CLEAR, &args![conditions]);
    fn_0060c7d0(e, this);
    fn_0060c870(e, this);
    let locals = e.mem.u32(this.addr() + 0x5c);
    if locals != 0 {
        e.call(SCRIPT_LOCALS_DELETE, &args![locals, 1u32]);
        e.mem.set_u32(this.addr() + 0x5c, 0);
    }
}

/// The destructors `TESQuest`'s takes down: the editor ID string
/// (`004037d0`), the condition list (`006808b0`), the two lists (`0046ffb0`),
/// `TESFullName` (`00591300`), `TESIcon` (`00402c40`) and `TESForm`
/// (`00483630`), all `thiscall` with no stack words.
const STRING_DESTRUCT: u32 = 0x0040_37d0;
const CONDITION_LIST_DESTRUCT: u32 = 0x0068_08b0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
const FULL_NAME_DESTRUCT: u32 = 0x0059_1300;
const ICON_DESTRUCT: u32 = 0x0040_2c40;
const TES_FORM_DESTRUCT: u32 = 0x0048_3630;

// Translated from 0060c660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0060c660 - the C++ exception-unwinding frame (FS:[0] chain, handler 00efb0fa, state variable) and its stack cookie word are not translated
/// Unnamed in the engine map: `TESQuest`'s destructor body (`thiscall`, no
/// stack words): the four vtables, `0060c760`, then the members and bases
/// from the last: the editor ID (+0x64), the conditions (+0x54), the
/// objective and stage lists (+0x4c, +0x44), `TESFullName` (+0x30),
/// `TESIcon` (+0x24) and `TESForm`.
pub fn fn_0060c660(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, TES_QUEST_VTABLE);
    e.mem.set_u32(t + 0x18, SCRIPTABLE_FORM_VTABLE);
    e.mem.set_u32(t + 0x24, ICON_VTABLE);
    e.mem.set_u32(t + 0x30, FULL_NAME_VTABLE);
    fn_0060c760(e, this);
    e.call(STRING_DESTRUCT, &args![t + EDITOR_ID]);
    e.call(CONDITION_LIST_DESTRUCT, &args![t + CONDITIONS]);
    e.call(LIST_DESTRUCT, &args![t + LIST_OBJECTIVES]);
    e.call(LIST_DESTRUCT, &args![t + LIST_STAGES]);
    e.call(FULL_NAME_DESTRUCT, &args![t + 0x30]);
    e.call(ICON_DESTRUCT, &args![t + 0x24]);
    e.call(TES_FORM_DESTRUCT, &args![this]);
}

// Translated from 0060c630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `TESQuest`'s scalar deleting destructor
/// (`thiscall`, `RET 4`: the flags): `0060c660`, then the free `00401030`
/// with bit 0. Answers `this`.
pub fn fn_0060c630(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0060c660(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00401030, &args![this]);
    }
    this
}

/// `00559450` (`thiscall` on a `BSStringT`; its text or 0) and `""` in
/// `.rdata` (`01011584`).
const STRING_TEXT: u32 = 0x0055_9450;
const EMPTY_TEXT: u32 = 0x0101_1584;

// Translated from 0060c5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a quest): the editor ID's text
/// (+0x64, asked twice), or `""` when it has none.
pub fn fn_0060c5f0(e: &mut Engine, this: Ptr) -> u32 {
    let id = this.addr() + EDITOR_ID;
    if e.call(STRING_TEXT, &args![id]).u32() != 0 {
        e.call(STRING_TEXT, &args![id]).u32()
    } else {
        EMPTY_TEXT
    }
}

/// `MiscStatManager::Increment` (`004d5c60`, `cdecl`: the statistic),
/// `HUDMainMenu::ShowQuestName` (`0077a480`, `cdecl`: the quest) and
/// `0060cad0` (`cdecl`, one word).
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
const SHOW_QUEST_NAME: u32 = 0x0077_a480;
const QUEST_DONE_0060CAD0: u32 = 0x0060_cad0;

// Translated from 0060ca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a quest, `RET 4`: a byte). A set
/// byte on a quest without bit 1 of the flags (+0x3c) sets it, flags the
/// change (vtable slot `+0x48` with 2), counts statistic 0 and shows the
/// quest's name; a clear byte on a quest with it clears it, flags the change
/// and runs `0060cad0(0)`.
pub fn fn_0060ca30(e: &mut Engine, this: Ptr, on: u8) {
    let at = this.addr() + DATA;
    if on != 0 && e.mem.u8(at) & 2 == 0 {
        let flags = e.mem.u8(at) | 2;
        e.mem.set_u8(at, flags);
        e.vcall(this.addr(), ADD_CHANGE_SLOT, &args![CHANGE_QUEST_FLAGS]);
        e.call(MISC_STAT_INCREMENT, &args![0u32]);
        e.call(SHOW_QUEST_NAME, &args![this]);
    } else if on == 0 && e.mem.u8(at) & 2 != 0 {
        let flags = e.mem.u8(at) & 0xfd;
        e.mem.set_u8(at, flags);
        e.vcall(this.addr(), ADD_CHANGE_SLOT, &args![CHANGE_QUEST_FLAGS]);
        e.call(QUEST_DONE_0060CAD0, &args![0u32]);
    }
}

/// The function-static null `NiPointer` `0060c120` compares with
/// (`011cb7dc`), its guard word (`011cb7e0`, bit 0) and its `atexit`
/// destructor (`00fcb330`); CRT `_atexit` (`00ec658f`).
const STATIC_NULL_POINTER: u32 = 0x011c_b7dc;
const STATIC_GUARD: u32 = 0x011c_b7e0;
const STATIC_NULL_DESTRUCT: u32 = 0x00fc_b330;
const ATEXIT: u32 = 0x00ec_658f;
/// `004b0460` (`thiscall` on an `NiPointer`, `RET 4`: another's address; a
/// byte: whether they are equal).
const NI_POINTER_EQUAL: u32 = 0x004b_0460;

// Translated from 0060c120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0060c120 - the C++ exception-unwinding frame (FS:[0] chain, handler 00efb08e, state variable) and its stack cookie word are not translated
/// Unnamed in the engine map: an `NiTArray<NiPointer<..>>` store at an index
/// (`thiscall`, `RET 8`: the index and the address of the `NiPointer` to
/// store). On first use it makes the static null pointer (`011cb7dc`, guard
/// bit 0 of `011cb7e0`, `00633c90` with 0, its destructor given to
/// `_atexit`). Past the used size (+0xa) the size becomes `index + 1` and a
/// non-null new pointer counts one more (+0xc); within it a non-null new
/// pointer over a null old one counts one more, a null new one over a
/// non-null old one one less. Then the slot (`+4` block) is assigned
/// (`006e5cc0`).
pub fn fn_0060c120(e: &mut Engine, this: Ptr, index: u32, pointer: u32) {
    let t = this.addr();
    let guard = e.global::<u32>(STATIC_GUARD);
    if guard & 1 == 0 {
        e.set_global::<u32>(STATIC_GUARD, guard | 1);
        e.call(NI_POINTER_FROM_RAW, &args![STATIC_NULL_POINTER, 0u32]);
        e.call(ATEXIT, &args![STATIC_NULL_DESTRUCT]);
    }
    let slot = |e: &Engine| e.mem.u32(t + 4).wrapping_add(index.wrapping_mul(4));
    if index >= u32::from(e.mem.u16(t + 0xa)) {
        e.mem.set_u16(t + 0xa, index.wrapping_add(1) as u16);
        if e.call(NI_POINTER_NOT_EQUAL, &args![pointer, STATIC_NULL_POINTER])
            .bool()
        {
            let count = e.mem.u16(t + 0xc).wrapping_add(1);
            e.mem.set_u16(t + 0xc, count);
        }
    } else if e
        .call(NI_POINTER_NOT_EQUAL, &args![pointer, STATIC_NULL_POINTER])
        .bool()
    {
        let at = slot(e);
        if e.call(NI_POINTER_EQUAL, &args![at, STATIC_NULL_POINTER])
            .bool()
        {
            let count = e.mem.u16(t + 0xc).wrapping_add(1);
            e.mem.set_u16(t + 0xc, count);
        }
    } else {
        let at = slot(e);
        if e.call(NI_POINTER_NOT_EQUAL, &args![at, STATIC_NULL_POINTER])
            .bool()
        {
            let count = e.mem.u16(t + 0xc).wrapping_sub(1);
            e.mem.set_u16(t + 0xc, count);
        }
    }
    let at = slot(e);
    e.call(NI_POINTER_ASSIGN, &args![at, pointer]);
}

// Translated from 0060c290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the pointers `[first, last)` so that they end at `destination`
/// (cdecl; the engine map has no name): the count is `(last - first) >> 2`
/// (signed), the start `destination - 4 * count`; when the count is above
/// 0 the words move with `memmove_s(start, 4 * count, first, 4 * count)`
/// (`00ec7ce3`). Answers the start.
pub fn fn_0060c290(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    let count = (last.wrapping_sub(first) as i32) >> 2;
    let bytes = (count as u32).wrapping_shl(2);
    let start = destination.wrapping_sub(bytes);
    if count > 0 {
        e.call(0x00ec_7ce3, &args![start, bytes, first, bytes]);
    }
    start
}

// Translated from 0060c950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands each objective of the quest a state (`RET 4`: a word it never
/// reads; the engine map has no name): for each item of the list
/// `005d43e0` gives, up to an empty slot, `005ec5d0` on the item with 3
/// when `005a5e70` answers true for it and 2 otherwise.
pub fn fn_0060c950(e: &mut Engine, this: Ptr<TESQuest>, _unused_1: u32) {
    let mut node = e.call(0x005d_43e0, &args![this]).u32();
    while node != 0 {
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let objective = e.mem.u32(slot);
        let state = if e.call(0x005a_5e70, &args![objective]).bool() {
            3u32
        } else {
            2
        };
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let objective = e.mem.u32(slot);
        e.call(0x005e_c5d0, &args![objective, state]);
        node = e.call(0x0072_6070, &args![node]).u32();
    }
}

// Translated from 0060cad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lowers a miscellaneous stat by one (cdecl: the stat; the engine map
/// has no name): `MiscStatManager::ModVal(stat, -1)` (`004d5e10`, cdecl).
pub fn fn_0060cad0(e: &mut Engine, stat: u32) {
    e.call(0x004d_5e10, &args![stat, 0xffff_ffffu32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060_c290, fn_0060c290(u32, u32, u32) -> u32),
        entry!(0x0060_c950, fn_0060c950(Ptr<TESQuest>, u32)),
        entry!(0x0060_cad0, fn_0060cad0(u32)),
        entry!(0x0060c4b0, fn_0060c4b0(Ptr<TESQuest>) -> Ptr<TESQuest>),
        entry!(0x0060c5c0, fn_0060c5c0(Ptr<QuestData>) -> Ptr<QuestData>),
        entry!(0x0060c740, fn_0060c740(Ptr<TESQuest>)),
        entry!(0x0060c9c0, fn_0060c9c0(Ptr<TESQuest>, u8)),
        entry!(0x0060_c440, fn_0060c440(Ptr, u32) -> u32),
        entry!(0x0060_c450, fn_0060c450(Ptr, u32) -> u16),
        entry!(0x0060_c460, fn_0060c460(Ptr, u32, u32)),
        entry!(0x0060_c470, fn_0060c470(Ptr, u32)),
        entry!(0x0060_c480, fn_0060c480(Ptr, u32, u32)),
        entry!(0x0060_c490, fn_0060c490(Ptr, u32) -> u16),
        entry!(0x0060_c4a0, fn_0060c4a0(Ptr, u32)),
        entry!(0x0060_bef0, fn_0060bef0(Ptr<crate::types::NiTArray>, u32)),
        entry!(0x0060_c3c0, fn_0060c3c0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0060_c320, fn_0060c320(u32, u32, u32, u32) -> u32),
        entry!(0x0060_c270, fn_0060c270(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0060_c410, fn_0060c410(u32, u32, u32, u32)),
        entry!(0x0060_c380, fn_0060c380(u32, u32, u32, u32, u32)),
        entry!(0x0060_c2e0, fn_0060c2e0(u32, u32, u32)),
        entry!(0x0060_c840, fn_0060c840(Ptr, u32) -> Ptr),
        entry!(0x0060_c7d0, fn_0060c7d0(Ptr)),
        entry!(0x0060_c870, fn_0060c870(Ptr)),
        entry!(0x0060_c8e0, fn_0060c8e0(Ptr, u32) -> u32),
        entry!(0x0060_c760, fn_0060c760(Ptr)),
        entry!(0x0060_c660, fn_0060c660(Ptr)),
        entry!(0x0060_c630, fn_0060c630(Ptr, u32) -> Ptr),
        entry!(0x0060_c5f0, fn_0060c5f0(Ptr) -> u32),
        entry!(0x0060_ca30, fn_0060ca30(Ptr, u8)),
        entry!(0x0060_c120, fn_0060c120(Ptr, u32, u32)),
    ]
}

/// `00633c90`: an `NiPointer` constructed from a raw pointer (`thiscall`, `RET 4`;
/// stores it and, when non-null, takes a reference through `0040f6e0`); answers
/// `this`.
const NI_POINTER_FROM_RAW: u32 = 0x0063_3c90;
/// `00631820`: `NiPointer` "not equal" (`thiscall`, `RET 4`, the other
/// `NiPointer`'s address); answers a `bool` in `AL`.
const NI_POINTER_NOT_EQUAL: u32 = 0x0063_1820;
/// `0045cec0`: the `NiPointer` destructor (releases through `00401970` when
/// non-null).
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `006e5cc0`: `NiPointer` assignment from another `NiPointer`'s address
/// (`thiscall`, `RET 4`).
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// `005e0ba0`: allocates an array of `n` `NiPointer`s (`cdecl`, one word;
/// the engine map names it `NiTNewInterface<NiPointer<NiAVObject>>::Allocate`).
const NI_POINTER_ARRAY_ALLOCATE: u32 = 0x005e_0ba0;
/// `004b0220`: frees such an array when non-null (`cdecl`, one word).
const NI_POINTER_ARRAY_FREE: u32 = 0x004b_0220;

/// Runs `body` with a stack `NiPointer` temporary built from null
/// (`00633c90` with 0), passing the address `00633c90` answered, then
/// destroys the temporary (`0045cec0`), as the exe does for each of its three
/// temporaries.
fn with_null_ni_pointer<R>(e: &mut Engine, body: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(4, |e, temporary| {
        let null = e.call(NI_POINTER_FROM_RAW, &args![temporary, 0u32]).u32();
        let result = body(e, null);
        e.call(NI_POINTER_DESTRUCT, &args![temporary]);
        result
    })
}

// Translated from 0060bef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. On an `NiTArray` of `NiPointer`s (layout
/// [`crate::types::NiTArray`]) it changes the capacity to `max_size`
/// (`thiscall`, `RET 4`): nothing when it is already `m_usMaxSize`. When the
/// new capacity is below `m_usSize`, each slot from `max_size` up to
/// `m_usSize` that is not null (compared with a null `NiPointer`
/// temporary, `00631820`) is cleared (assigned a null temporary, `006e5cc0`)
/// and `m_usESize` drops by 1; then `m_usSize = max_size`. `m_usMaxSize`
/// takes the new capacity; for 0 the element pointer becomes null, otherwise
/// a new array of that many is allocated (`005e0ba0`), the old slots
/// `0..m_usSize` are assigned into it (`006e5cc0`) and the slots
/// `m_usSize..m_usMaxSize` are assigned a null temporary. The old array is
/// freed (`004b0220`).
// Translation incomplete: 0060bef0 - the SEH frame (FS:[0], unwind of the three NiPointer temporaries; the cookie it stores at [EBP-0x10] is never checked) is not translated.
pub fn fn_0060bef0(e: &mut Engine, this: Ptr<crate::types::NiTArray>, max_size: u32) {
    use crate::types::NiTArray;
    if max_size == u32::from(e.get(this, NiTArray::m_usMaxSize)) {
        return;
    }
    let slot = |e: &Engine, index: u16| {
        e.get(this, NiTArray::m_pBase)
            .wrapping_add(u32::from(index).wrapping_mul(4))
    };
    if max_size < u32::from(e.get(this, NiTArray::m_usSize)) {
        let mut index = max_size as u16;
        while index < e.get(this, NiTArray::m_usSize) {
            let occupied = with_null_ni_pointer(e, |e, null| {
                let at = slot(e, index);
                e.call(NI_POINTER_NOT_EQUAL, &args![at, null]).bool()
            });
            if occupied {
                with_null_ni_pointer(e, |e, null| {
                    let at = slot(e, index);
                    e.call(NI_POINTER_ASSIGN, &args![at, null]);
                });
                let filled = e.get(this, NiTArray::m_usESize);
                e.set(this, NiTArray::m_usESize, filled.wrapping_sub(1));
            }
            index = index.wrapping_add(1);
        }
        e.set(this, NiTArray::m_usSize, max_size as u16);
    }
    let old = e.get(this, NiTArray::m_pBase);
    e.set(this, NiTArray::m_usMaxSize, max_size as u16);
    if max_size == 0 {
        e.set(this, NiTArray::m_pBase, 0);
    } else {
        let count = u32::from(e.get(this, NiTArray::m_usMaxSize));
        let new = e.call(NI_POINTER_ARRAY_ALLOCATE, &args![count]).u32();
        e.set(this, NiTArray::m_pBase, new);
        let mut index: u16 = 0;
        while index < e.get(this, NiTArray::m_usSize) {
            let from = old.wrapping_add(u32::from(index).wrapping_mul(4));
            let at = slot(e, index);
            e.call(NI_POINTER_ASSIGN, &args![at, from]);
            index = index.wrapping_add(1);
        }
        let mut index = e.get(this, NiTArray::m_usSize);
        while index < e.get(this, NiTArray::m_usMaxSize) {
            with_null_ni_pointer(e, |e, null| {
                let at = slot(e, index);
                e.call(NI_POINTER_ASSIGN, &args![at, null]);
            });
            index = index.wrapping_add(1);
        }
    }
    e.call(NI_POINTER_ARRAY_FREE, &args![old]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TESForm;

    /// The test slot-0x48 function and a vtable made of it.
    const ADD_CHANGE: u32 = 0x00aa_0001;
    const TEST_VTABLE: u32 = 0x00ab_0000;
    /// Bytes after the quest's 0x6c where the slot double counts its calls
    /// and keeps the last argument.
    const CALLS: u32 = 0x70;
    const LAST_ARGUMENT: u32 = 0x74;

    /// An engine with test doubles for every callee outside this file and a
    /// vtable for the quest with a counting double in slot 0x48.
    fn quest_engine() -> Engine {
        let mut e = Engine::new();
        for callee in [
            TES_FORM_CONSTRUCT,
            SCRIPTABLE_FORM_CONSTRUCT,
            ICON_CONSTRUCT,
            FULL_NAME_CONSTRUCT,
            LIST_CONSTRUCT,
            CONDITION_LIST_CONSTRUCT,
            STRING_CONSTRUCT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        // The form-type setter writes the byte at +4 like the real one.
        e.register(SET_FORM_TYPE, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(QUEST_DATA_ADDRESS, |_, a| Ret {
            eax: a[0] + 0x3c,
            ..Ret::default()
        });
        e.register(ADD_CHANGE, |e, a| {
            let seen = e.mem.u32(a[0] + CALLS);
            e.mem.set_u32(a[0] + CALLS, seen + 1);
            e.mem.set_u32(a[0] + LAST_ARGUMENT, a[1]);
            Ret::default()
        });
        e.put_vtable(TEST_VTABLE, &[ADD_CHANGE; 0x20]);
        e.put_vtable(TES_QUEST_VTABLE, &[ADD_CHANGE; 0x20]);
        e
    }

    /// A zeroed quest block with room for the double's words, carrying the
    /// test vtable.
    fn quest_block(e: &mut Engine) -> Ptr<TESQuest> {
        let block = e.mem.alloc(0x80);
        e.mem.set_u32(block, TEST_VTABLE);
        Ptr::new(block)
    }

    #[test]
    fn quest_data_starts_cleared() {
        let mut e = quest_engine();
        let data: Ptr<QuestData> = e.new_object();
        e.set(data, QuestData::ucFlags, 0xff);
        e.set(data, QuestData::ucPriority, 0xff);
        e.set(data, QuestData::fQuestDelayTime, 3.0);
        let back = e.call(0x0060_c5c0, &args![data]).ptr::<QuestData>();
        assert_eq!(back, data);
        assert_eq!(e.get(data, QuestData::ucFlags), 0);
        assert_eq!(e.get(data, QuestData::ucPriority), 0);
        assert_eq!(e.get(data, QuestData::fQuestDelayTime), 0.0);
    }

    #[test]
    fn enable_sets_or_clears_bit_zero_and_flags_the_change() {
        let mut e = quest_engine();
        let quest = quest_block(&mut e);
        let data = quest.addr() + DATA;
        e.mem.set_u8(data, 0b1010);
        e.call(0x0060_c9c0, &args![quest, 1u32]);
        assert_eq!(e.mem.u8(data), 0b1011);
        assert_eq!(e.mem.u32(quest.addr() + CALLS), 1);
        assert_eq!(e.mem.u32(quest.addr() + LAST_ARGUMENT), 2);
        // Only the low byte of the stack word counts as the flag.
        e.call(0x0060_c9c0, &args![quest, 0x100u32]);
        assert_eq!(e.mem.u8(data), 0b1010);
        e.call(0x0060_c9c0, &args![quest, 3u32]);
        assert_eq!(e.mem.u8(data), 0b1011);
        assert_eq!(e.mem.u32(quest.addr() + CALLS), 3);
    }

    #[test]
    fn a_new_quest_is_enabled_at_stage_zero() {
        let mut e = quest_engine();
        let quest = quest_block(&mut e);
        e.set(quest, TESQuest::cCurrentStage, 9);
        e.call(0x0060_c740, &args![quest]);
        assert_eq!(e.mem.u8(quest.addr() + DATA) & 1, 1);
        assert_eq!(e.get(quest, TESQuest::cCurrentStage), 0);
    }

    #[test]
    fn constructor_builds_the_bases_in_the_games_order_and_sets_the_vtables() {
        let mut e = quest_engine();
        let quest = quest_block(&mut e);
        let base = quest.addr();
        e.set(quest, TESQuest::pScriptLocals, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        let back = e.call(0x0060_c4b0, &args![quest]).ptr::<TESQuest>();
        assert_eq!(back, quest);
        let log = e.call_log.take().unwrap();
        // The slot call is made through the vtable: leave it out of the list.
        let calls: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|(addr, _)| *addr != ADD_CHANGE)
            .collect();
        assert_eq!(
            calls[1..],
            [
                (TES_FORM_CONSTRUCT, vec![base]),
                (SCRIPTABLE_FORM_CONSTRUCT, vec![base + 0x18]),
                (ICON_CONSTRUCT, vec![base + 0x24]),
                (FULL_NAME_CONSTRUCT, vec![base + 0x30]),
                (LIST_CONSTRUCT, vec![base + 0x44]),
                (LIST_CONSTRUCT, vec![base + 0x4c]),
                (CONDITION_LIST_CONSTRUCT, vec![base + 0x54]),
                (STRING_CONSTRUCT, vec![base + 0x64]),
                (SET_FORM_TYPE, vec![base, 0x47]),
                (QUEST_DATA_ADDRESS, vec![base]),
            ]
        );
        assert_eq!(e.mem.u32(base), TES_QUEST_VTABLE);
        assert_eq!(e.mem.u32(base + 0x18), SCRIPTABLE_FORM_VTABLE);
        assert_eq!(e.mem.u32(base + 0x24), ICON_VTABLE);
        assert_eq!(e.mem.u32(base + 0x30), FULL_NAME_VTABLE);
        assert_eq!(e.get(quest.cast::<TESForm>(), TESForm::cFormType), 0x47);
        assert_eq!(e.get(quest, TESQuest::pScriptLocals), Ptr::NULL);
        assert_eq!(e.get(quest, TESQuest::cCurrentStage), 0);
        assert_eq!(e.mem.u8(base + DATA) & 1, 1);
        assert_eq!(e.mem.u32(base + CALLS), 1);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_0060c440() {
        assert_eq!(
            check(0x0060_c440, 0x0060_1370, 0xdc, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0060c450() {
        assert_eq!(check(0x0060_c450, 0x0060_8da0, 0x7c, &[0x77]).u16(), 0xbeef);
    }

    #[test]
    fn le81_thunk_0060c460() {
        check(0x0060_c460, 0x0060_8e80, 0x90, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_0060c470() {
        check(0x0060_c470, 0x0060_8e00, 0x90, &[0x77]);
    }

    #[test]
    fn le81_thunk_0060c480() {
        check(0x0060_c480, 0x0060_8e80, 0x7c, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_0060c490() {
        assert_eq!(check(0x0060_c490, 0x0060_8da0, 0x90, &[0x77]).u16(), 0xbeef);
    }

    #[test]
    fn le81_thunk_0060c4a0() {
        check(0x0060_c4a0, 0x0060_8e00, 0x7c, &[0x77]);
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;
    use crate::types::NiTArray;

    /// Doubles for the `NiPointer` helpers that act on memory as the exe's
    /// bodies do (without reference counts), and an allocator from the heap.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_POINTER_FROM_RAW, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(NI_POINTER_NOT_EQUAL, |e, a| {
            u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        e.register(NI_POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let v = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], v);
            a[0].into_ret()
        });
        e.register(NI_POINTER_ARRAY_ALLOCATE, |e, a| {
            let p = e.mem.alloc(a[0] * 4);
            for i in 0..a[0] {
                e.mem.set_u32(p + i * 4, 0xdead);
            }
            p.into_ret()
        });
        e.register(NI_POINTER_ARRAY_FREE, |_, _| Ret::default());
        e
    }

    fn array(e: &mut Engine, items: &[u32], max: u16) -> Ptr<NiTArray> {
        let a = e.new_object::<NiTArray>();
        let base = e.mem.alloc(u32::from(max) * 4);
        for (i, v) in items.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *v);
        }
        e.set(a, NiTArray::m_pBase, base);
        e.set(a, NiTArray::m_usMaxSize, max);
        e.set(a, NiTArray::m_usSize, items.len() as u16);
        e.set(
            a,
            NiTArray::m_usESize,
            items.iter().filter(|v| **v != 0).count() as u16,
        );
        a
    }

    fn frees(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == NI_POINTER_ARRAY_FREE)
            .map(|c| c.1[0])
            .collect()
    }

    #[test]
    fn fn_0060bef0_grows_shrinks_and_empties() {
        let mut e = engine();
        // Same capacity: nothing.
        let a = array(&mut e, &[1, 2], 4);
        e.call_log = Some(vec![]);
        e.call(0x0060_bef0, &args![a, 4u32]);
        assert!(frees(&e).is_empty());
        // Growing: old slots copied, new ones nulled, old array freed.
        let old = e.get(a, NiTArray::m_pBase);
        e.call(0x0060_bef0, &args![a, 6u32]);
        let base = e.get(a, NiTArray::m_pBase);
        assert_ne!(base, old);
        assert_eq!(e.get(a, NiTArray::m_usMaxSize), 6);
        let slots: Vec<u32> = (0..6).map(|i| e.mem.u32(base + 4 * i)).collect();
        assert_eq!(slots, [1, 2, 0, 0, 0, 0]);
        assert_eq!(frees(&e), [old]);
        // Shrinking below the size: the cut non-null slots are cleared and
        // uncounted, the size follows.
        let a = array(&mut e, &[1, 0, 3, 4], 4);
        e.call_log = Some(vec![]);
        let old = e.get(a, NiTArray::m_pBase);
        e.call(0x0060_bef0, &args![a, 1u32]);
        assert_eq!(e.mem.u32(old + 8), 0);
        assert_eq!(e.mem.u32(old + 12), 0);
        assert_eq!(e.get(a, NiTArray::m_usESize), 1);
        assert_eq!(e.get(a, NiTArray::m_usSize), 1);
        assert_eq!(e.get(a, NiTArray::m_usMaxSize), 1);
        assert_eq!(e.mem.u32(e.get(a, NiTArray::m_pBase)), 1);
        // Zero: no array.
        e.call(0x0060_bef0, &args![a, 0u32]);
        assert_eq!(e.get(a, NiTArray::m_pBase), 0);
        assert_eq!(e.get(a, NiTArray::m_usSize), 0);
        assert_eq!(frees(&e).len(), 2);
    }
}

#[cfg(test)]
mod b008_tests {
    use super::*;

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()[1..].to_vec()
    }

    #[test]
    fn fn_0060c3c0_moves_whole_words() {
        let mut e = Engine::new();
        e.register(MEMMOVE_S, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x0060_c3c0,
                &args![0x100u32, 0x10cu32, 0x200u32, 0u32, 0u32, 0u32]
            )
            .u32(),
            0x20c
        );
        assert_eq!(calls(&mut e), vec![(MEMMOVE_S, vec![0x200, 12, 0x100, 12])]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x0060_c3c0,
                &args![0x100u32, 0x100u32, 0x200u32, 0u32, 0u32, 0u32]
            )
            .u32(),
            0x200
        );
        assert!(calls(&mut e).is_empty());
    }

    #[test]
    fn fn_0060c270_copies_through_the_templates() {
        let mut e = Engine::new();
        e.register(MEMMOVE_S, |_, _| Ret::default());
        e.register(ITERATOR_CATEGORY_0065E750, |_, _| 7u32.into_ret());
        e.register(ITERATOR_BASE_0065FE40, |e, a| e.mem.u32(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e.call(
            0x0060_c270,
            &args![0x100u32, 0x108u32, 0x300u32, 9u32, 0u32, 0u32],
        );
        let log = calls(&mut e);
        assert_eq!(log.last().unwrap(), &(MEMMOVE_S, vec![0x300, 8, 0x100, 8]));
        assert_eq!(
            log.iter().filter(|c| c.0 == ITERATOR_BASE_0065FE40).count(),
            2
        );
    }

    #[test]
    fn fn_0060c2e0_fills_quieted_floats() {
        let mut e = Engine::new();
        e.register(ITERATOR_TAG_0065FD70, |_, a| a[0].into_ret());
        e.register(ITERATOR_BASE_0065FE40, |e, a| e.mem.u32(a[0]).into_ret());
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x7f80_0001);
        let out = e.mem.alloc(16);
        e.call(0x0060_c2e0, &args![out, 3u32, value]);
        for i in 0..3 {
            assert_eq!(e.mem.u32(out + 4 * i), 0x7fc0_0001);
        }
        assert_eq!(e.mem.u32(out + 12), 0);
    }

    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0137_0000, 0x100);
        // A list at 0x01370000 whose item count is the word there.
        e.register(QUEST_LIST_005D43C0, |_, _| 0x0137_0000u32.into_ret());
        e.register(QUEST_LIST_005D43E0, |_, _| 0x0137_0000u32.into_ret());
        e.register(LIST_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(LIST_HEAD_ITEM, |e, a| {
            let n = e.mem.u32(a[0]);
            (a[0] + 4 * n).into_ret()
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let n = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], n - 1);
            Ret::default()
        });
        e
    }

    #[test]
    fn fn_0060c7d0_and_0060c870_empty_the_lists() {
        let mut e = list_engine();
        e.register(STAGE_DESTRUCT, |_, _| Ret::default());
        e.register(FREE_00401030, |_, _| Ret::default());
        e.mem.set_u32(0x0137_0000, 2);
        e.mem.set_u32(0x0137_0004, 0);
        e.mem.set_u32(0x0137_0008, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0060_c7d0, &args![0x10u32]);
        let log = calls(&mut e);
        assert!(log.contains(&(STAGE_DESTRUCT, vec![0x5000])));
        assert!(log.contains(&(FREE_00401030, vec![0x5000])));
        assert_eq!(log.iter().filter(|c| c.0 == LIST_REMOVE_HEAD).count(), 2);

        const DELETE: u32 = 0x0137_1000;
        e.register(DELETE, |_, _| Ret::default());
        let table = e.mem.alloc(4);
        e.mem.set_u32(table, DELETE);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, table);
        e.mem.set_u32(0x0137_0000, 1);
        e.mem.set_u32(0x0137_0004, item);
        e.call_log = Some(vec![]);
        e.call(0x0060_c870, &args![0x10u32]);
        assert!(calls(&mut e).contains(&(DELETE, vec![item, 1])));
    }

    #[test]
    fn fn_0060c8e0_finds_by_key() {
        let mut e = Engine::new();
        let item = e.mem.alloc(8);
        e.mem.set_u32(item + 4, 0x33);
        let second = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, item);
        e.register_double(QUEST_LIST_005D43E0, move |_, _| first.into_ret());
        e.register(LIST_IS_EMPTY, |_, _| Ret::default());
        e.register(LIST_HEAD_ITEM, |_, a| a[0].into_ret());
        e.register(LINK_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        assert_eq!(e.call(0x0060_c8e0, &args![0x10u32, 0x33u32]).u32(), item);
        assert_eq!(e.call(0x0060_c8e0, &args![0x10u32, 0x34u32]).u32(), 0);
    }

    fn quest_engine() -> Engine {
        let mut e = list_engine();
        for f in [
            QUEST_CONDITIONS,
            CONDITIONS_CLEAR,
            SCRIPT_LOCALS_DELETE,
            STRING_DESTRUCT,
            CONDITION_LIST_DESTRUCT,
            LIST_DESTRUCT,
            FULL_NAME_DESTRUCT,
            ICON_DESTRUCT,
            TES_FORM_DESTRUCT,
            FREE_00401030,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn fn_0060c760_clears_the_quest() {
        let mut e = quest_engine();
        let quest = e.mem.alloc(0x6c);
        e.mem.set_u32(quest + 0x5c, 0x7700);
        e.call_log = Some(vec![]);
        e.call(0x0060_c760, &args![quest]);
        assert!(calls(&mut e).contains(&(SCRIPT_LOCALS_DELETE, vec![0x7700, 1])));
        assert_eq!(e.mem.u32(quest + 0x5c), 0);
    }

    #[test]
    fn fn_0060c630_destroys_and_frees() {
        let mut e = quest_engine();
        let quest = e.mem.alloc(0x6c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0060_c630, &args![quest, 1u32]).u32(), quest);
        let log = calls(&mut e);
        let tail: Vec<u32> = log.iter().rev().take(3).map(|c| c.0).collect();
        assert_eq!(tail, vec![FREE_00401030, TES_FORM_DESTRUCT, ICON_DESTRUCT]);
        assert_eq!(e.mem.u32(quest), TES_QUEST_VTABLE);
        assert!(log.contains(&(STRING_DESTRUCT, vec![quest + EDITOR_ID])));
        e.call_log = Some(vec![]);
        e.call(0x0060_c630, &args![quest, 0u32]);
        assert!(calls(&mut e).iter().all(|c| c.0 != FREE_00401030));
    }

    #[test]
    fn fn_0060c5f0_answers_the_editor_id() {
        let mut e = Engine::new();
        e.register(STRING_TEXT, |_, _| 0x4400u32.into_ret());
        assert_eq!(e.call(0x0060_c5f0, &args![0x1000u32]).u32(), 0x4400);
        e.register(STRING_TEXT, |_, _| Ret::default());
        assert_eq!(e.call(0x0060_c5f0, &args![0x1000u32]).u32(), EMPTY_TEXT);
    }

    #[test]
    fn fn_0060ca30_turns_bit_1_on_and_off() {
        const CHANGE: u32 = 0x0137_2000;
        let mut e = Engine::new();
        for f in [
            CHANGE,
            MISC_STAT_INCREMENT,
            SHOW_QUEST_NAME,
            QUEST_DONE_0060CAD0,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        let table = e.mem.alloc(0x50);
        e.mem.set_u32(table + ADD_CHANGE_SLOT, CHANGE);
        let quest = e.mem.alloc(0x6c);
        e.mem.set_u32(quest, table);
        e.call_log = Some(vec![]);
        e.call(0x0060_ca30, &args![quest, 1u32]);
        assert_eq!(e.mem.u8(quest + DATA), 2);
        assert_eq!(
            calls(&mut e),
            vec![
                (CHANGE, vec![quest, 2]),
                (MISC_STAT_INCREMENT, vec![0]),
                (SHOW_QUEST_NAME, vec![quest]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x0060_ca30, &args![quest, 1u32]);
        assert!(calls(&mut e).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0060_ca30, &args![quest, 0u32]);
        assert_eq!(e.mem.u8(quest + DATA), 0);
        assert_eq!(
            calls(&mut e),
            vec![(CHANGE, vec![quest, 2]), (QUEST_DONE_0060CAD0, vec![0])]
        );
    }

    #[test]
    fn fn_0060c120_counts_the_pointers() {
        let mut e = Engine::new();
        e.map(STATIC_GUARD, 4);
        e.map(STATIC_NULL_POINTER, 4);
        for f in [NI_POINTER_FROM_RAW, ATEXIT, NI_POINTER_ASSIGN] {
            e.register(f, |_, _| Ret::default());
        }
        // A pointer slot is "not null" when the word it holds is not 0.
        e.register(NI_POINTER_NOT_EQUAL, |e, a| {
            u32::from(e.mem.u32(a[0]) != 0).into_ret()
        });
        e.register(NI_POINTER_EQUAL, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        let array = e.mem.alloc(0x10);
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, block);
        let full = e.mem.alloc(4);
        e.mem.set_u32(full, 0x9999);
        let empty = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x0060_c120, &args![array, 2u32, full]);
        assert_eq!((e.mem.u16(array + 0xa), e.mem.u16(array + 0xc)), (3, 1));
        let log = calls(&mut e);
        assert_eq!(log[0], (NI_POINTER_FROM_RAW, vec![STATIC_NULL_POINTER, 0]));
        assert_eq!(
            log.last().unwrap(),
            &(NI_POINTER_ASSIGN, vec![block + 8, full])
        );
        // Within the size: a pointer over an empty slot counts up, an empty
        // one over a full slot counts down.
        e.call(0x0060_c120, &args![array, 1u32, full]);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        e.mem.set_u32(block + 4, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x0060_c120, &args![array, 1u32, empty]);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert!(calls(&mut e).iter().all(|c| c.0 != NI_POINTER_FROM_RAW));
    }

    #[test]
    fn fn_0060c270_answers_the_destination_past_the_copy() {
        let mut e = Engine::new();
        e.register(MEMMOVE_S, |_, _| Ret::default());
        e.register(ITERATOR_CATEGORY_0065E750, |_, _| 7u32.into_ret());
        e.register(ITERATOR_BASE_0065FE40, |e, a| e.mem.u32(a[0]).into_ret());
        let end = e.call(
            0x0060_c270,
            &args![0x100u32, 0x108u32, 0x300u32, 9u32, 0u32, 0u32],
        );
        assert_eq!(end.u32(), 0x308);
        let end = e.call(0x0060_c320, &args![0x100u32, 0x10cu32, 0x400u32, 9u32]);
        assert_eq!(end.u32(), 0x40c);
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
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

    #[test]
    fn fn_0060c290_moves_the_words_before_the_destination() {
        let mut e = Engine::new();
        e.register(0x00ec_7ce3, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0060_c290, &args![0x1000u32, 0x100cu32, 0x2010u32])
                .u32(),
            0x2004
        );
        assert_eq!(calls(&e, 0x00ec_7ce3), vec![vec![0x2004, 12, 0x1000, 12]]);
        // An empty or reversed range moves nothing.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0060_c290, &args![0x1000u32, 0x1000u32, 0x2010u32])
                .u32(),
            0x2010
        );
        assert_eq!(
            e.call(0x0060_c290, &args![0x1008u32, 0x1000u32, 0x2010u32])
                .u32(),
            0x2018
        );
        assert!(calls(&e, 0x00ec_7ce3).is_empty());
    }

    #[test]
    fn fn_0060c950_hands_each_objective_its_state() {
        let mut e = Engine::new();
        let quest = e.mem.alloc(0x6c);
        // The list: nodes {item, next}; the second item answers true.
        let second = e.mem.alloc(8);
        e.mem.set_u32(quest + 0x4c, 0x51);
        e.mem.set_u32(quest + 0x50, second);
        e.mem.set_u32(second, 0x52);
        e.register(0x005d_43e0, |_, a| ret(a[0] + 0x4c));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x005a_5e70, |_, a| ret(u32::from(a[0] == 0x52)));
        e.register(0x005e_c5d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0060_c950, &args![quest, 0xdeadu32]);
        assert_eq!(calls(&e, 0x005e_c5d0), vec![vec![0x51, 2], vec![0x52, 3]]);
        // An empty slot stops the walk.
        e.mem.set_u32(quest + 0x4c, 0);
        e.call_log = Some(vec![]);
        e.call(0x0060_c950, &args![quest, 0u32]);
        assert!(calls(&e, 0x005e_c5d0).is_empty());
    }

    #[test]
    fn fn_0060cad0_lowers_the_stat_by_one() {
        let mut e = Engine::new();
        e.register(0x004d_5e10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0060_cad0, &args![0x1bu32]);
        assert_eq!(calls(&e, 0x004d_5e10), vec![vec![0x1b, 0xffff_ffff]]);
    }
}

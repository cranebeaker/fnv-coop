//! `fallout shared/tesquest.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESQuest` (0x6c bytes on PC) keeps its quest data at +0x3c, the stage
//! list (`BSSimpleList<TESQuestStage *>`) at +0x44, the objective list at
//! +0x4c, the condition list at +0x54, the script locals at +0x5c and the
//! current stage at +0x60. Its constructor family is in
//! `run_0060bef0.rs`. This file holds the rest of the quest's own code
//! (set failed, init item, save, load, set stage done, ...) and the small
//! members of the stage classes that sit in the same address range
//! (`TESQuestStage` at `0060f210`/`0060f320`/`0060f440`, and the
//! `TESQuestStageItem` constructor and setters).
//!
//! The lists are walked as the exe does: `005d43c0` / `005d43e0` give the
//! stage / objective list (`this + 0x44` / `this + 0x4c`), a node is empty
//! when both of its words are zero (`008256d0`), `006815c0` returns the node
//! itself (identical-code folded; its first word is the item) and
//! `00726070` gives the next node.

#[allow(unused_imports)]
use crate::prelude::*;

use super::run_0060bef0::{QuestData, TESQuest};

/// `TESForm::StartForm` (`004855a0`) and `TESForm::CloseForm` (`00485680`).
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// `TESForm::LoadForm` (`00485110`, one stack word: the file).
const LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (one stack word): called with 0 by `Load` and with 1 by
/// `InitItem`.
const SET_FORM_FLAG_CHANGED: u32 = 0x0048_4ab0;
/// `TESForm::ForceChange` (`00484b90`, one stack word: the change mask).
const FORCE_CHANGE: u32 = 0x0048_4b90;
/// `TESForm::__SaveData` (`00485e20`: the data, its size) and
/// `TESForm::LoadSimpleStruct` (`004861f0`: file, data, size).
const SAVE_DATA: u32 = 0x0048_5e20;
const LOAD_DATA: u32 = 0x0048_61f0;
/// The `TESScriptableForm` component at `this + 0x18`: save (`0048cca0`),
/// `InitItem` (`0048cd90`, one stack word: the quest), the script setter
/// (`006ecd40`, one stack word).
const SCRIPTABLE_SAVE: u32 = 0x0048_cca0;
const SCRIPTABLE_INIT_ITEM: u32 = 0x0048_cd90;
const SCRIPTABLE_SET_SCRIPT: u32 = 0x006e_cd40;
/// `TESFullName` at `this + 0x30`: save (`00487010`), chunk loader
/// (`00487050`: component, file; cdecl).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const FULL_NAME_LOAD_CHUNK: u32 = 0x0048_7050;
/// `TESTexture` (the icon) at `this + 0x24`: save (`0048e370`, one stack
/// word: the chunk tag), chunk loader (`0048e3d0`: component, file; cdecl).
const ICON_SAVE: u32 = 0x0048_e370;
const ICON_LOAD_CHUNK: u32 = 0x0048_e3d0;
/// `TESCondition` at `this + 0x54`: save (`00680910`), `InitItem`
/// (`00680a20`, one stack word: the quest), chunk loader (`00680960`, one
/// stack word: the file).
const CONDITION_SAVE: u32 = 0x0068_0910;
const CONDITION_INIT_ITEM: u32 = 0x0068_0a20;
const CONDITION_LOAD_CHUNK: u32 = 0x0068_0960;
/// `Script::Run` (`005ac1e0`, four stack words), `ScriptLocals` creation
/// (`005abf60`, from the script) and its owner setter (`0087ce80`, one
/// stack word: the quest).
const SCRIPT_RUN: u32 = 0x005a_c1e0;
const SCRIPT_CREATE_LOCALS: u32 = 0x005a_bf60;
const SCRIPT_SET_OWNER: u32 = 0x0087_ce80;
/// `HUDMainMenu::ShowQuestName` (`0077a480`, cdecl, one stack word).
const SHOW_QUEST_NAME: u32 = 0x0077_a480;
/// `009373f0` reads a byte at `this`, `005dc960` reads the byte at
/// `this + 1`.
const READ_BYTE_0: u32 = 0x0093_73f0;
const READ_BYTE_1: u32 = 0x005d_c960;
/// `00483710`: an empty function, called for each stage while saving.
const STAGE_SAVE: u32 = 0x0048_3710;
/// `005d43c0`, `005d43e0`, `005f5760`: the stage list (`this + 0x44`), the
/// objective list (`this + 0x4c`) and the condition list (`this + 0x54`).
const STAGE_LIST: u32 = 0x005d_43c0;
const OBJECTIVE_LIST: u32 = 0x005d_43e0;
const CONDITION_LIST: u32 = 0x005f_5760;
/// `005a8080`: the address of the quest data (`this + 0x3c`).
const QUEST_DATA_ADDRESS: u32 = 0x005a_8080;
/// `0059e400`: bit 1 of the quest data flags; `00455620`: bit 0 (enabled).
const QUEST_FLAG_2: u32 = 0x0059_e400;
const QUEST_FLAG_ENABLED: u32 = 0x0045_5620;
/// `008256d0`: true for an empty list node. `00726070`: a node's next
/// pointer / a component's second word. `006815c0`: returns `this`.
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
const WORD_AT_4: u32 = 0x0072_6070;
const RETURN_THIS: u32 = 0x0068_15c0;
/// `00717e50`: `this + 4` (the item list of a stage).
const STAGE_ITEM_LIST: u32 = 0x0071_7e50;
/// `0048d150`: `this + 0xc` (the embedded `Script` of a stage item).
const ITEM_SCRIPT: u32 = 0x0048_d150;
/// `00500940`: `this + 0x18`.
const SCRIPT_FORM_DATA: u32 = 0x0050_0940;
/// `0060f240` (the two-byte stage data constructor) and `0096a2d0` (the
/// two-word list constructor).
const STAGE_DATA_CONSTRUCT: u32 = 0x0060_f240;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `TESCondition` constructor (`00680890`), `Script` constructor
/// (`005aa0f0`), `TESForm::SetTemporary` (`00484490`) and the
/// `memset` wrapper (`00403d30`: destination, value, count).
const CONDITION_CONSTRUCT: u32 = 0x0068_0890;
const SCRIPT_CONSTRUCT: u32 = 0x005a_a0f0;
const SET_TEMPORARY: u32 = 0x0048_4490;
const MEMSET: u32 = 0x0040_3d30;
/// `TESQuestStageItem::InitItem` (`0060fa10`: quest, stage; the engine map
/// names it `TESQuestStage::InitItem`).
const STAGE_ITEM_INIT: u32 = 0x0060_fa10;
/// `TESQuestStageItem::Load` (`0060f6a0`, one stack word: the file).
const STAGE_ITEM_LOAD: u32 = 0x0060_f6a0;
/// `0060f4a0` (this = the stage; quest, a flag): marks the stage done.
const STAGE_MARK_DONE: u32 = 0x0060_f4a0;
/// `BGSQuestObjective` constructor (`005ec090`), its `InitItem` (`005ec210`,
/// one stack word: the quest), save (`005ec280`), loader (`005ec370`, one
/// stack word: the file) and owner setter (`00437730`, one stack word).
const OBJECTIVE_CONSTRUCT: u32 = 0x005e_c090;
const OBJECTIVE_INIT_ITEM: u32 = 0x005e_c210;
const OBJECTIVE_SAVE: u32 = 0x005e_c280;
const OBJECTIVE_LOAD: u32 = 0x005e_c370;
const OBJECTIVE_SET_OWNER: u32 = 0x0043_7730;
/// `0060ff70`: the quest target constructor; `00610120`: its loader (one
/// stack word: the file); `007d6bb0`: the target list of an objective.
const TARGET_CONSTRUCT: u32 = 0x0060_ff70;
const TARGET_LOAD: u32 = 0x0061_0120;
const OBJECTIVE_TARGET_LIST: u32 = 0x007d_6bb0;
/// `BSSimpleList::AddHead`-style insertion (`00905820`, this = the list,
/// one stack word: the address of the item pointer).
const LIST_INSERT: u32 = 0x0090_5820;
/// `operator new` (`00401000`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `TESForm::iFormFlags` bit 3 test (`004013e0`).
const FORM_FLAG_TEST_BIT_3: u32 = 0x0040_13e0;
/// The file reader: record type (`00472660`), chunk type (`004726b0`),
/// chunk size (`00401660`), chunk data (`00472890`: buffer, size), a dword
/// (`004727f0`), next chunk (`004726f0`), byte-swap flag (`00401680`).
const FILE_RECORD_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
const FILE_GET_CHUNK_DWORD: u32 = 0x0047_27f0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_SWAP_FLAG: u32 = 0x0040_1680;
/// The byte-swap flag of the save writer (`00401500`) and the in-place
/// swap of a `float` (`00401080`: address, a flag byte).
const SAVE_SWAP_FLAG: u32 = 0x0040_1500;
const SWAP_FLOAT: u32 = 0x0040_1080;
/// The INI setting object (`011cb81c`) read through `00408d60` (its `+4`,
/// a `bool`): when set, `InitItem` enables the quest.
const SETTING_OBJECT: u32 = 0x011c_b81c;
const SETTING_BOOL: u32 = 0x0040_8d60;
/// `0060c9c0`: set/clear the enabled flag (translated in
/// `run_0060bef0.rs`).
const SET_ENABLED: u32 = 0x0060_c9c0;
/// `TESQuest` vtable slot `0x48` (`AddChange` in the Xbox PDB).
const ADD_CHANGE_SLOT: u32 = 0x48;
/// `TESQuest` vtable slots used by `Load` (byte offsets): `0x134` receives
/// the editor ID, `0xe0` the object bound chunk.
const SLOT_SET_EDITOR_ID: u32 = 0x134;
const SLOT_LOAD_OBJECT_BOUND: u32 = 0xe0;
/// The change flags: quest flags (2) and stage change (`0x80000000`); the
/// script locals change (`0x40000000`).
const CHANGE_QUEST_FLAGS: u32 = 2;
const CHANGE_QUEST_STAGES: u32 = 0x8000_0000;
const CHANGE_SCRIPT_LOCALS: u32 = 0x4000_0000;
/// Quest data flag bits used here: enabled (1), bit 1 (2), completed-
/// related bit 3 (8), the bit `0060cd00` sets (0x10) and failed (0x40).
const FLAG_BIT_1: u8 = 0x02;
const FLAG_BIT_3: u8 = 0x08;
const FLAG_BIT_4: u8 = 0x10;
const FLAG_FAILED: u8 = 0x40;
/// The form type of a quest record (`QUST`).
const FORM_TYPE_QUEST: u8 = 0x47;

/// Chunk tags, as the little-endian words the file reader returns.
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_SCDA: u32 = 0x4144_4353;
const CHUNK_CTDA: u32 = 0x4144_5443;
const CHUNK_QSTA: u32 = 0x4154_5351;
const CHUNK_SCRI: u32 = 0x4952_4353;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_SLSD: u32 = 0x4453_4c53;
const CHUNK_QOBJ: u32 = 0x4a42_4f51;
const CHUNK_SCVR: u32 = 0x5256_4353;
const CHUNK_SCRO: u32 = 0x4f52_4353;
const CHUNK_CNAM: u32 = 0x4d41_4e43;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_SCHR: u32 = 0x5248_4353;
const CHUNK_QSDT: u32 = 0x5444_5351;
const CHUNK_SCRV: u32 = 0x5652_4353;
const CHUNK_INDX: u32 = 0x5844_4e49;

/// Offsets in a `TESQuest` (Xbox PDB names, PC offsets): the
/// `TESScriptableForm` (+0x18), `TESIcon` (+0x24) and `TESFullName` (+0x30)
/// bases and the quest data (+0x3c).
const SCRIPTABLE: u32 = 0x18;
const ICON: u32 = 0x24;
const FULL_NAME: u32 = 0x30;
const DATA: u32 = 0x3c;

layout! {
    /// `TESQuestStage` (Xbox PDB), 0xc bytes; the same on PC. Its
    /// `QUEST_STAGE_DATA` is the first two bytes, the item list sits at +4.
    pub struct TESQuestStage: 0x0c {
        /// `m_Data.ucIndex` (Xbox PDB): the stage number.
        0x00 ucIndex: u8,
        /// `m_Data.bIsDone` (Xbox PDB), as the raw byte.
        0x01 bIsDone: u8,
    }

    /// `TESQuestStageItem` (Xbox PDB, 0x84 bytes), PC size 0x74: the
    /// embedded `Script` at +0xc is 0x10 bytes smaller on PC, so the fields
    /// after it sit 0x10 lower than in the PDB (the constructor `0060f520`
    /// stores to exactly these PC offsets).
    pub struct TESQuestStageItem: 0x74 {
        /// `m_Data.iFlags` (Xbox PDB).
        0x00 iFlags: u8,
        /// `m_fileOffset` (Xbox PDB).
        0x60 m_fileOffset: u32,
        /// `ucIndex` (Xbox PDB).
        0x64 ucIndex: u8,
        /// `m_bHasLogEntry` (Xbox PDB), as the raw byte.
        0x65 m_bHasLogEntry: u8,
        /// `m_pLogDate` (Xbox PDB).
        0x68 m_pLogDate: Ptr,
        /// `m_pOwner` (Xbox PDB).
        0x6C m_pOwner: Ptr,
        /// `m_pNextQuest` (Xbox PDB).
        0x70 m_pNextQuest: Ptr,
    }
}

/// The quest data of `this` (`005a8080` is `this + 0x3c`).
fn quest_data(this: Ptr<TESQuest>) -> Ptr<QuestData> {
    Ptr::new(this.addr() + DATA)
}

/// The item of a list node: `006815c0` returns the node, whose first word
/// is the item.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let node = e.call(RETURN_THIS, &args![node]).u32();
    e.mem.u32(node)
}

fn node_is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(NODE_IS_EMPTY, &args![node]).bool()
}

fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(WORD_AT_4, &args![node]).u32()
}

// Translated from 0060caf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESQuest::SetFailed` (Xbox PDB): with `failed` non-zero and the quest
/// not in the state `0059e400` tests (bit 1 of the quest data flags) and not
/// already failed, sets the failed bit (0x40) and bit 1, signals a quest-flags
/// change through vtable slot `0x48` and shows the quest name
/// (`HUDMainMenu::ShowQuestName`). With `failed` zero and the failed bit set,
/// clears it and signals the change.
pub fn tes_quest_set_failed(e: &mut Engine, this: Ptr<TESQuest>, failed: u8) {
    let data = quest_data(this);
    if failed != 0 && e.call(QUEST_FLAG_2, &args![this]).bool() {
        return;
    }
    if failed != 0 && e.get(data, QuestData::ucFlags) & FLAG_FAILED == 0 {
        let flags = e.get(data, QuestData::ucFlags) | FLAG_FAILED;
        e.set(data, QuestData::ucFlags, flags);
        let flags = e.get(data, QuestData::ucFlags) | FLAG_BIT_1;
        e.set(data, QuestData::ucFlags, flags);
        e.vcall(this.addr(), ADD_CHANGE_SLOT, &args![CHANGE_QUEST_FLAGS]);
        e.call(SHOW_QUEST_NAME, &args![this]);
        return;
    }
    if failed == 0 && e.get(data, QuestData::ucFlags) & FLAG_FAILED != 0 {
        let flags = e.get(data, QuestData::ucFlags) & !FLAG_FAILED;
        e.set(data, QuestData::ucFlags, flags);
        e.vcall(this.addr(), ADD_CHANGE_SLOT, &args![CHANGE_QUEST_FLAGS]);
    }
}

// Translated from 0060cba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The quest's `InitItem` (vtable slot `0x88`; the engine map names nothing
/// here). Unless the form is flagged (bit 3 of the form flags): runs the
/// script component's `InitItem`; when the form has a script, creates the
/// script locals, hands the script the quest, forces a script-locals change
/// when the locals exist, and enables the quest when the INI setting
/// (`011cb81c`) is on. Then initializes the conditions, every stage
/// (`0060f320`) and every objective, sets the bit `0060cd00` toggles from
/// the quest's enabled state and flags the form (`00484ab0` with 1).
pub fn fn_0060cba0(e: &mut Engine, this: Ptr<TESQuest>) {
    let base = this.addr();
    if e.call(FORM_FLAG_TEST_BIT_3, &args![this]).bool() {
        return;
    }
    e.call(SCRIPTABLE_INIT_ITEM, &args![base + SCRIPTABLE, this]);
    if e.call(WORD_AT_4, &args![base + SCRIPTABLE]).u32() != 0 {
        let script = e.call(WORD_AT_4, &args![base + SCRIPTABLE]).u32();
        let locals = e.call(SCRIPT_CREATE_LOCALS, &args![script]).u32();
        e.set(this, TESQuest::pScriptLocals, Ptr::new(locals));
        let script = e.call(WORD_AT_4, &args![base + SCRIPTABLE]).u32();
        e.call(SCRIPT_SET_OWNER, &args![script, this]);
        if e.get(this, TESQuest::pScriptLocals) != Ptr::NULL {
            e.call(FORCE_CHANGE, &args![this, CHANGE_SCRIPT_LOCALS]);
        }
        let setting = e.call(SETTING_BOOL, &args![SETTING_OBJECT]).u32();
        if e.mem.u8(setting) != 0 {
            e.call(SET_ENABLED, &args![this, 1u32]);
        }
    }
    let conditions = e.call(CONDITION_LIST, &args![this]).u32();
    if conditions != 0 {
        let conditions = e.call(CONDITION_LIST, &args![this]).u32();
        e.call(CONDITION_INIT_ITEM, &args![conditions, this]);
    }
    let mut node = e.call(STAGE_LIST, &args![this]).u32();
    while node != 0 && !node_is_empty(e, node) {
        let stage = node_item(e, node);
        fn_0060f320(e, Ptr::new(stage), this);
        node = node_next(e, node);
    }
    let mut node = e.call(OBJECTIVE_LIST, &args![this]).u32();
    while node != 0 && !node_is_empty(e, node) {
        let objective = node_item(e, node);
        e.call(OBJECTIVE_INIT_ITEM, &args![objective, this]);
        node = node_next(e, node);
    }
    let enabled = e.call(QUEST_FLAG_ENABLED, &args![this]).u8();
    fn_0060cd00(e, this, enabled);
    e.call(SET_FORM_FLAG_CHANGED, &args![this, 1u32]);
}

// Translated from 0060cd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 4 (0x10) of the quest data flags.
/// The exe names no quest accessor here; its caller `InitItem` passes the
/// quest's enabled state.
pub fn fn_0060cd00(e: &mut Engine, this: Ptr<TESQuest>, flag: u8) {
    let data: Ptr<QuestData> = e.call_as(QUEST_DATA_ADDRESS, &args![this]);
    let flags = e.get(data, QuestData::ucFlags);
    let flags = if flag != 0 {
        flags | FLAG_BIT_4
    } else {
        flags & !FLAG_BIT_4
    };
    e.set(data, QuestData::ucFlags, flags);
}

// Translated from 0060cd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESQuest::Save` (Xbox PDB): starts the form; saves the script component
/// (+0x18), the name (+0x30) and the icon with the `ICON` tag (+0x24);
/// writes the quest data (+0x3c, 8 bytes) with its delay time byte-swapped
/// before and after when the writer swaps (`00401500`); saves the conditions
/// when present, each stage (`00483710`, an empty function) and each
/// objective; closes the form.
pub fn tes_quest_save(e: &mut Engine, this: Ptr<TESQuest>) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(SCRIPTABLE_SAVE, &args![base + SCRIPTABLE]);
    e.call(FULL_NAME_SAVE, &args![base + FULL_NAME]);
    e.call(ICON_SAVE, &args![base + ICON, CHUNK_ICON]);
    if e.call(SAVE_SWAP_FLAG, &args![]).bool() {
        fn_0060ce80(e, quest_data(this));
    }
    e.call(SAVE_DATA, &args![this, base + DATA, 8u32]);
    if e.call(SAVE_SWAP_FLAG, &args![]).bool() {
        fn_0060ce80(e, quest_data(this));
    }
    let conditions = e.call(CONDITION_LIST, &args![this]).u32();
    if conditions != 0 {
        let conditions = e.call(CONDITION_LIST, &args![this]).u32();
        e.call(CONDITION_SAVE, &args![conditions]);
    }
    let mut node = e.call(STAGE_LIST, &args![this]).u32();
    while node != 0 && !node_is_empty(e, node) {
        let stage = node_item(e, node);
        e.call(STAGE_SAVE, &args![stage]);
        node = node_next(e, node);
    }
    let mut node = e.call(OBJECTIVE_LIST, &args![this]).u32();
    while node != 0 && !node_is_empty(e, node) {
        let objective = node_item(e, node);
        e.call(OBJECTIVE_SAVE, &args![objective]);
        node = node_next(e, node);
    }
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 0060ce80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the `float` at `data + 4` (the quest's delay time) in place
/// (`00401080` with a zero flag byte).
pub fn fn_0060ce80(e: &mut Engine, data: Ptr<QuestData>) {
    e.call(SWAP_FLOAT, &args![data.addr() + 4, 0u32]);
}

// Translated from 0060ceb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The quest's form loader (vtable slot `0x20`; the engine map names nothing
/// here). Returns false unless the file's record is a quest (type `0x47`).
/// Otherwise `TESForm::LoadForm`, then every chunk:
/// `EDID` to vtable slot `0x134` (the first 0x200 bytes), `OBND` to slot
/// `0xe0`, `SCRI` to the script component (+0x18), `FULL` and `ICON` to
/// their components, `DATA` into the quest data (+0x3c, 8 bytes), `QOBJ`
/// (a new `BGSQuestObjective`, 0x24 bytes, added to the objective list),
/// `QSTA` (a new `TESQuestTarget`, 0x48 bytes, added to the current
/// objective's target list, creating an objective when there is none),
/// `INDX` (a new `TESQuestStage`, 0xc bytes, added to the stage list),
/// `QSDT` (a new `TESQuestStageItem`, 0x74 bytes, loaded, given a running
/// index and added to the current stage's item list), the script chunks
/// (`SCDA`, `SLSD`, `SCVR`, `SCRO`, `SCHR`, `SCRV`) to the current stage
/// item's loader, `CNAM` (marks the item as having a log entry) and `CTDA`
/// (a condition for the current item, else the current target, else the
/// quest). After the last chunk the delay time is swapped back when the
/// reader swaps.
///
/// The item loader and the script chunks leave the file on the next
/// unhandled chunk, so those cases do not advance. A `QSDT` chunk with no
/// current stage does not advance either, as in the exe, and so never
/// ends.
// Translation incomplete: 0060ceb0 - SEH exception unwinding and the stack security cookie (/GS) check 00ec408c are not translated.
pub fn fn_0060ceb0(e: &mut Engine, this: Ptr<TESQuest>, file: Ptr) -> bool {
    if e.call(FILE_RECORD_TYPE, &args![file]).u8() != FORM_TYPE_QUEST {
        return false;
    }
    let base = this.addr();
    // The four locals the exe keeps on its stack and passes by address to
    // the list insertion: current stage, current item, current target and
    // current objective.
    let frame = e.mem.alloc(0x10);
    let (stage_cell, item_cell, target_cell, objective_cell) =
        (frame, frame + 4, frame + 8, frame + 12);
    for cell in [stage_cell, item_cell, target_cell, objective_cell] {
        e.mem.set_u32(cell, 0);
    }
    let mut buffers: Vec<u32> = Vec::new();
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_CHANGED, &args![this, 0u32]);
    let mut stage_item_index: u8 = 0;
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                buffers.push(buffer);
                e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                e.vcall(base, SLOT_SET_EDITOR_ID, &args![buffer]);
            }
            CHUNK_OBND => {
                e.vcall(base, SLOT_LOAD_OBJECT_BOUND, &args![file]);
            }
            CHUNK_SCRI => {
                let value = e.mem.alloc(4);
                buffers.push(value);
                e.mem.set_u32(value, 0);
                e.call(FILE_GET_CHUNK_DWORD, &args![file, value]);
                let script = e.mem.u32(value);
                e.call(SCRIPTABLE_SET_SCRIPT, &args![base + SCRIPTABLE, script]);
                e.call(SCRIPTABLE_INIT_ITEM, &args![base + SCRIPTABLE, this]);
            }
            CHUNK_FULL => {
                let component = if base == 0 { 0 } else { base + FULL_NAME };
                e.call(FULL_NAME_LOAD_CHUNK, &args![component, file]);
            }
            CHUNK_ICON => {
                let component = if base == 0 { 0 } else { base + ICON };
                e.call(ICON_LOAD_CHUNK, &args![component, file]);
            }
            CHUNK_DATA => {
                e.call(LOAD_DATA, &args![this, file, base + DATA, 8u32]);
            }
            CHUNK_QOBJ => {
                let objective = new_objective(e);
                e.mem.set_u32(objective_cell, objective);
                e.call(OBJECTIVE_LOAD, &args![objective, file]);
                let list = e.call(OBJECTIVE_LIST, &args![this]).u32();
                e.call(LIST_INSERT, &args![list, objective_cell]);
                e.call(OBJECTIVE_SET_OWNER, &args![objective, this]);
            }
            CHUNK_QSTA => {
                e.mem.set_u32(stage_cell, 0);
                e.mem.set_u32(item_cell, 0);
                let size = 0x48u32;
                let memory = e.call(OPERATOR_NEW, &args![size]).u32();
                let target = if memory != 0 {
                    e.call(TARGET_CONSTRUCT, &args![memory]).u32()
                } else {
                    0
                };
                e.mem.set_u32(target_cell, target);
                e.call(TARGET_LOAD, &args![target, file]);
                if e.mem.u32(objective_cell) == 0 {
                    let objective = new_objective(e);
                    e.mem.set_u32(objective_cell, objective);
                    let list = e.call(OBJECTIVE_LIST, &args![this]).u32();
                    e.call(LIST_INSERT, &args![list, objective_cell]);
                    e.call(OBJECTIVE_SET_OWNER, &args![objective, this]);
                }
                let objective = e.mem.u32(objective_cell);
                let targets = e.call(OBJECTIVE_TARGET_LIST, &args![objective]).u32();
                e.call(LIST_INSERT, &args![targets, target_cell]);
            }
            CHUNK_INDX => {
                let memory = e.call(OPERATOR_NEW, &args![0x0cu32]).u32();
                let stage = if memory != 0 {
                    fn_0060f210(e, Ptr::new(memory)).addr()
                } else {
                    0
                };
                e.mem.set_u32(stage_cell, stage);
                fn_0060f440(e, Ptr::new(stage), file);
                let list = e.call(STAGE_LIST, &args![this]).u32();
                e.call(LIST_INSERT, &args![list, stage_cell]);
            }
            CHUNK_QSDT => {
                if e.mem.u32(stage_cell) != 0 {
                    let memory = e.call(OPERATOR_NEW, &args![0x74u32]).u32();
                    let item = if memory != 0 {
                        fn_0060f520(e, Ptr::new(memory)).addr()
                    } else {
                        0
                    };
                    e.mem.set_u32(item_cell, item);
                    e.call(STAGE_ITEM_LOAD, &args![item, file]);
                    fn_0060d4b0(e, Ptr::new(item), stage_item_index);
                    stage_item_index = stage_item_index.wrapping_add(1);
                    let stage = e.mem.u32(stage_cell);
                    let items = e.call(STAGE_ITEM_LIST, &args![stage]).u32();
                    e.call(LIST_INSERT, &args![items, item_cell]);
                }
                // The item loader reads on to the next chunk it does not
                // handle itself, so the loop does not advance.
                continue;
            }
            CHUNK_SCDA | CHUNK_SLSD | CHUNK_SCVR | CHUNK_SCRO | CHUNK_SCHR | CHUNK_SCRV => {
                let item = e.mem.u32(item_cell);
                if item != 0 {
                    e.call(STAGE_ITEM_LOAD, &args![item, file]);
                    continue;
                }
            }
            CHUNK_CNAM => {
                let item = e.mem.u32(item_cell);
                if item != 0 {
                    fn_0060d490(e, Ptr::new(item), 1);
                }
            }
            CHUNK_CTDA => {
                let item = e.mem.u32(item_cell);
                let target = e.mem.u32(target_cell);
                let conditions = if item != 0 {
                    e.call(STAGE_ITEM_LIST, &args![item]).u32()
                } else if target != 0 {
                    e.call(STAGE_ITEM_LIST, &args![target]).u32()
                } else {
                    e.call(CONDITION_LIST, &args![this]).u32()
                };
                e.call(CONDITION_LOAD_CHUNK, &args![conditions, file]);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    if e.call(FILE_SWAP_FLAG, &args![file]).bool() {
        fn_0060ce80(e, quest_data(this));
    }
    for buffer in buffers {
        e.mem.free(buffer);
    }
    e.mem.free(frame);
    true
}

/// A new `BGSQuestObjective`: 0x24 bytes from `operator new`, constructed by
/// `005ec090` (0 when the allocation fails).
fn new_objective(e: &mut Engine) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
    if memory != 0 {
        e.call(OBJECTIVE_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    }
}

// Translated from 0060d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `has_log_entry` at +0x65 of a `TESQuestStageItem` (its
/// `m_bHasLogEntry`; `Load` calls it with 1 for a `CNAM` chunk).
pub fn fn_0060d490(e: &mut Engine, this: Ptr<TESQuestStageItem>, has_log_entry: u8) {
    e.set(this, TESQuestStageItem::m_bHasLogEntry, has_log_entry);
}

// Translated from 0060d4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `index` at +0x64 of a `TESQuestStageItem` (its
/// `ucIndex`; `Load` passes a running count).
pub fn fn_0060d4b0(e: &mut Engine, this: Ptr<TESQuestStageItem>, index: u8) {
    e.set(this, TESQuestStageItem::ucIndex, index);
}

// Translated from 0060d4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the quest has a script (the second word of the script component,
/// +0x1c), runs it with the quest's script locals (+0x5c): `Script::Run`
/// with 0, the locals, 0 and 0.
pub fn fn_0060d4d0(e: &mut Engine, this: Ptr<TESQuest>) {
    let base = this.addr();
    if e.call(WORD_AT_4, &args![base + SCRIPTABLE]).u32() == 0 {
        return;
    }
    let locals = e.get(this, TESQuest::pScriptLocals);
    let script = e.call(WORD_AT_4, &args![base + SCRIPTABLE]).u32();
    e.call(SCRIPT_RUN, &args![script, 0u32, locals, 0u32, 0u32]);
}

// Translated from 0060d510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESQuest::SetStageDone` (Xbox PDB): finds the stage numbered `stage` and,
/// when found, enables the quest (`0060c9c0` with 1) unless it is in the
/// state `0059e400` tests and `0060d5e0` is false, raises the current stage
/// to `stage` if that is larger, marks the stage done (`0060f4a0` with the
/// quest and 0) and flags a stage change (`0x80000000`) through vtable slot
/// `0x48`. Returns true when the stage was found.
pub fn tes_quest_set_stage_done(e: &mut Engine, this: Ptr<TESQuest>, stage: u8) -> bool {
    let mut node = e.call(STAGE_LIST, &args![this]).u32();
    while node != 0 && !node_is_empty(e, node) {
        let item = node_item(e, node);
        if e.call(READ_BYTE_0, &args![item]).u8() == stage {
            if !e.call(QUEST_FLAG_2, &args![this]).bool() || fn_0060d5e0(e, this) {
                e.call(SET_ENABLED, &args![this, 1u32]);
            }
            if e.get(this, TESQuest::cCurrentStage) < stage {
                e.set(this, TESQuest::cCurrentStage, stage);
            }
            e.call(STAGE_MARK_DONE, &args![item, this, 0u32]);
            e.vcall(this.addr(), ADD_CHANGE_SLOT, &args![CHANGE_QUEST_STAGES]);
            return true;
        }
        node = node_next(e, node);
    }
    false
}

// Translated from 0060d5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 3 (8) of the quest data flags.
pub fn fn_0060d5e0(e: &mut Engine, this: Ptr<TESQuest>) -> bool {
    let data: Ptr<QuestData> = e.call_as(QUEST_DATA_ADDRESS, &args![this]);
    e.get(data, QuestData::ucFlags) & FLAG_BIT_3 != 0
}

// Translated from 0060d600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The done byte (`+1`, `bIsDone`) of the stage numbered `stage`, 0 when
/// there is no such stage.
pub fn fn_0060d600(e: &mut Engine, this: Ptr<TESQuest>, stage: u8) -> u8 {
    let mut node = e.call(STAGE_LIST, &args![this]).u32();
    while node != 0 && !node_is_empty(e, node) {
        let item = node_item(e, node);
        if e.call(READ_BYTE_0, &args![item]).u8() == stage {
            let item = node_item(e, node);
            return e.call(READ_BYTE_1, &args![item]).u8();
        }
        node = node_next(e, node);
    }
    0
}

// Translated from 0060d670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Raises the current stage (+0x60) to the highest stage number among the
/// stages whose done byte (`+1`) is non-zero, when that is larger.
pub fn fn_0060d670(e: &mut Engine, this: Ptr<TESQuest>) {
    let mut highest: u8 = 0;
    let mut node = e.call(STAGE_LIST, &args![this]).u32();
    while node != 0 && !node_is_empty(e, node) {
        let stage = node_item(e, node);
        if e.call(READ_BYTE_1, &args![stage]).bool() {
            let number = e.call(READ_BYTE_0, &args![stage]).u8();
            if number > highest {
                highest = e.call(READ_BYTE_0, &args![stage]).u8();
            }
        }
        node = node_next(e, node);
    }
    if e.get(this, TESQuest::cCurrentStage) < highest {
        e.set(this, TESQuest::cCurrentStage, highest);
    }
}

// Translated from 0060d700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESQuest::GetCurrentStage` (Xbox PDB): the current stage byte (+0x60).
pub fn tes_quest_get_current_stage(e: &mut Engine, this: Ptr<TESQuest>) -> u8 {
    e.get(this, TESQuest::cCurrentStage)
}

// Translated from 0060f210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `TESQuestStage` (0xc bytes; `Load`'s `INDX` case
/// allocates 0xc and calls it, and the layout matches the Xbox PDB's
/// `TESQuestStage`: the two-byte stage data, then the item list at +4): the
/// stage data constructor `0060f240` (zeroes the two bytes) and the list
/// constructor `0096a2d0` on `this + 4`. Returns `this`. (The decompiler
/// labels it a library `SafeRWList` constructor, which it is not.)
pub fn fn_0060f210(e: &mut Engine, this: Ptr<TESQuestStage>) -> Ptr<TESQuestStage> {
    e.call(STAGE_DATA_CONSTRUCT, &args![this]);
    e.call(LIST_CONSTRUCT, &args![this.addr() + 4]);
    this
}

// Translated from 0060f320 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a stage (`this`) of `quest`: initializes every item of its item list
/// (`this + 4`) with `0060fa10` (quest, stage) and clears the byte at
/// `item + 0xc + 0x18 + 0x10` (the byte at +0x10 of the item's script
/// component data, as `0048d150` and `00500940` locate it). Null items are
/// skipped. The exe also counts the items in a local it never reads.
pub fn fn_0060f320(e: &mut Engine, this: Ptr<TESQuestStage>, quest: Ptr<TESQuest>) {
    let mut node = e.call(STAGE_ITEM_LIST, &args![this]).u32();
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            e.call(STAGE_ITEM_INIT, &args![item, quest, this]);
            let script = e.call(ITEM_SCRIPT, &args![item]).u32();
            let data = e.call(SCRIPT_FORM_DATA, &args![script]).u32();
            e.mem.set_u8(data + 0x10, 0);
        }
        node = node_next(e, node);
    }
}

// Translated from 0060f440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the stage data from the file's current chunk when it is `INDX`
/// (`file` is not null): an 8-byte chunk is read into a buffer and only its
/// first two bytes are kept; any other size reads two bytes straight into
/// `this` (stage number and done byte).
pub fn fn_0060f440(e: &mut Engine, this: Ptr<TESQuestStage>, file: Ptr) {
    if file == Ptr::NULL {
        return;
    }
    if e.call(FILE_CHUNK_TYPE, &args![file]).u32() != CHUNK_INDX {
        return;
    }
    if e.call(FILE_CHUNK_SIZE, &args![file]).u32() == 8 {
        let buffer = e.mem.alloc(8);
        e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 8u32]);
        let done = e.mem.u8(buffer + 1);
        let index = e.mem.u8(buffer);
        e.mem.free(buffer);
        e.set(this, TESQuestStage::bIsDone, done);
        e.set(this, TESQuestStage::ucIndex, index);
    } else {
        e.call(FILE_GET_CHUNK_DATA, &args![file, this, 2u32]);
    }
}

// Translated from 0060f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a `TESQuestStageItem` (0x74 bytes; `Load`'s `QSDT` case
/// allocates 0x74 and calls it): constructs the condition list (+4) and the
/// embedded `Script` (+0xc), zeroes the first byte (`memset`), the file
/// offset (+0x60), flags the script (`00484ab0` with 0), marks it temporary
/// and sets the byte at +0x10 of its data to 1, and clears the owner,
/// log date, next-quest pointers, index and has-log-entry byte. Returns
/// `this`.
// Translation incomplete: 0060f520 - SEH exception unwinding (the Script constructor's frame) is unsupported.
pub fn fn_0060f520(e: &mut Engine, this: Ptr<TESQuestStageItem>) -> Ptr<TESQuestStageItem> {
    let base = this.addr();
    e.call(CONDITION_CONSTRUCT, &args![base + 4]);
    e.call(SCRIPT_CONSTRUCT, &args![base + 0xc]);
    e.call(MEMSET, &args![this, 0u32, 1u32]);
    e.set(this, TESQuestStageItem::m_pNextQuest, Ptr::NULL);
    e.call(SET_FORM_FLAG_CHANGED, &args![base + 0xc, 0u32]);
    e.call(SET_TEMPORARY, &args![base + 0xc]);
    let data = e.call(SCRIPT_FORM_DATA, &args![base + 0xc]).u32();
    e.mem.set_u8(data + 0x10, 1);
    e.set(this, TESQuestStageItem::m_fileOffset, 0);
    e.set(this, TESQuestStageItem::m_pLogDate, Ptr::NULL);
    e.set(this, TESQuestStageItem::m_pOwner, Ptr::NULL);
    e.set(this, TESQuestStageItem::ucIndex, 0);
    e.set(this, TESQuestStageItem::m_bHasLogEntry, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060caf0, tes_quest_set_failed(Ptr<TESQuest>, u8)),
        entry!(0x0060cba0, fn_0060cba0(Ptr<TESQuest>)),
        entry!(0x0060cd00, fn_0060cd00(Ptr<TESQuest>, u8)),
        entry!(0x0060cd60, tes_quest_save(Ptr<TESQuest>)),
        entry!(0x0060ce80, fn_0060ce80(Ptr<QuestData>)),
        entry!(0x0060ceb0, fn_0060ceb0(Ptr<TESQuest>, Ptr) -> bool),
        entry!(0x0060d490, fn_0060d490(Ptr<TESQuestStageItem>, u8)),
        entry!(0x0060d4b0, fn_0060d4b0(Ptr<TESQuestStageItem>, u8)),
        entry!(0x0060d4d0, fn_0060d4d0(Ptr<TESQuest>)),
        entry!(
            0x0060d510,
            tes_quest_set_stage_done(Ptr<TESQuest>, u8) -> bool
        ),
        entry!(0x0060d5e0, fn_0060d5e0(Ptr<TESQuest>) -> bool),
        entry!(0x0060d600, fn_0060d600(Ptr<TESQuest>, u8) -> u8),
        entry!(0x0060d670, fn_0060d670(Ptr<TESQuest>)),
        entry!(0x0060d700, tes_quest_get_current_stage(Ptr<TESQuest>) -> u8),
        entry!(
            0x0060f210,
            fn_0060f210(Ptr<TESQuestStage>) -> Ptr<TESQuestStage>
        ),
        entry!(0x0060f320, fn_0060f320(Ptr<TESQuestStage>, Ptr<TESQuest>)),
        entry!(0x0060f440, fn_0060f440(Ptr<TESQuestStage>, Ptr)),
        entry!(
            0x0060f520,
            fn_0060f520(Ptr<TESQuestStageItem>) -> Ptr<TESQuestStageItem>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// A double that returns `eax` whatever it is called with.
    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Gives `object` a vtable of 0x500 bytes with `slots` filled in
    /// (byte offset, function address).
    fn with_vtable(e: &mut Engine, object: u32, slots: &[(u32, u32)]) {
        let table = e.mem.alloc(0x500);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        e.mem.set_u32(object, table);
    }

    /// The logged calls to `addr`, in order.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The addresses called, in order.
    fn addresses(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    /// The double standing for `TESForm::AddChange` (vtable slot 0x48).
    const ADD_CHANGE: u32 = 0x7400_0048;

    /// An engine whose small accessors behave as the exe's (as doubles) and
    /// a quest with a vtable that reaches [`ADD_CHANGE`].
    fn world() -> (Engine, Ptr<TESQuest>) {
        let mut e = Engine::new();
        e.register(STAGE_LIST, |_, a| eax(a[0] + 0x44));
        e.register(OBJECTIVE_LIST, |_, a| eax(a[0] + 0x4c));
        e.register(CONDITION_LIST, |_, a| eax(a[0] + 0x54));
        e.register(QUEST_DATA_ADDRESS, |_, a| eax(a[0] + 0x3c));
        e.register(NODE_IS_EMPTY, |e, a| {
            eax((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(WORD_AT_4, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(RETURN_THIS, |_, a| eax(a[0]));
        e.register(READ_BYTE_0, |e, a| eax(e.mem.u8(a[0]) as u32));
        e.register(READ_BYTE_1, |e, a| eax(e.mem.u8(a[0] + 1) as u32));
        e.register(QUEST_FLAG_2, |e, a| {
            eax((e.mem.u8(a[0] + 0x3c) & 2 != 0) as u32)
        });
        e.register(QUEST_FLAG_ENABLED, |e, a| {
            eax((e.mem.u8(a[0] + 0x3c) & 1 != 0) as u32)
        });
        e.register(FORM_FLAG_TEST_BIT_3, |e, a| {
            eax((e.mem.u32(a[0] + 8) & 8 != 0) as u32)
        });
        e.register(STAGE_ITEM_LIST, |_, a| eax(a[0] + 4));
        e.register(ITEM_SCRIPT, |_, a| eax(a[0] + 0xc));
        e.register(SCRIPT_FORM_DATA, |_, a| eax(a[0] + 0x18));
        stub(&mut e, ADD_CHANGE, 0);
        let quest: Ptr<TESQuest> = e.new_object();
        with_vtable(&mut e, quest.addr(), &[(ADD_CHANGE_SLOT, ADD_CHANGE)]);
        (e, quest)
    }

    /// Sets the quest data flags byte.
    fn set_flags(e: &mut Engine, quest: Ptr<TESQuest>, flags: u8) {
        e.mem.set_u8(quest.addr() + DATA, flags);
    }

    fn flags(e: &Engine, quest: Ptr<TESQuest>) -> u8 {
        e.mem.u8(quest.addr() + DATA)
    }

    /// Writes a list of `items` as nodes starting at the head node `head`
    /// (an embedded node, as in the quest); an empty list has a zero head.
    fn write_list(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            } else {
                e.mem.set_u32(node + 4, 0);
            }
        }
    }

    /// A stage with number `index` and done byte `done`.
    fn stage(e: &mut Engine, index: u8, done: u8) -> u32 {
        let stage = e.mem.alloc(0x0c);
        e.mem.set_u8(stage, index);
        e.mem.set_u8(stage + 1, done);
        stage
    }

    // ---- TESQuest::SetFailed ------------------------------------------

    #[test]
    fn set_failed_sets_the_failed_bits_and_shows_the_quest_name() {
        let (mut e, quest) = world();
        stub(&mut e, SHOW_QUEST_NAME, 0);
        e.call_log = Some(vec![]);
        e.call(0x0060_caf0, &args![quest, 1u32]);
        assert_eq!(flags(&e, quest), 0x42);
        assert_eq!(calls_to(&e, ADD_CHANGE), [[quest.addr(), 2]]);
        assert_eq!(calls_to(&e, SHOW_QUEST_NAME), [[quest.addr()]]);
    }

    #[test]
    fn set_failed_does_nothing_when_the_quest_is_already_failed_or_in_the_bit_1_state() {
        let (mut e, quest) = world();
        stub(&mut e, SHOW_QUEST_NAME, 0);
        // Already failed.
        set_flags(&mut e, quest, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x0060_caf0, &args![quest, 1u32]);
        assert_eq!(flags(&e, quest), 0x40);
        assert!(calls_to(&e, ADD_CHANGE).is_empty());
        // Bit 1 set: the quest-flag test makes the call a no-op, failed bit
        // or not.
        set_flags(&mut e, quest, 0x02);
        e.call(0x0060_caf0, &args![quest, 1u32]);
        assert_eq!(flags(&e, quest), 0x02);
        assert!(calls_to(&e, ADD_CHANGE).is_empty());
        assert!(calls_to(&e, SHOW_QUEST_NAME).is_empty());
    }

    #[test]
    fn set_failed_zero_clears_the_failed_bit_only_when_set() {
        let (mut e, quest) = world();
        set_flags(&mut e, quest, 0x43);
        e.call_log = Some(vec![]);
        e.call(0x0060_caf0, &args![quest, 0u32]);
        assert_eq!(flags(&e, quest), 0x03);
        assert_eq!(calls_to(&e, ADD_CHANGE), [[quest.addr(), 2]]);
        e.call(0x0060_caf0, &args![quest, 0u32]);
        assert_eq!(calls_to(&e, ADD_CHANGE).len(), 1);
        // With failed = 0 the bit-1 test is not made.
        assert!(calls_to(&e, QUEST_FLAG_2).is_empty());
    }

    // ---- small members -------------------------------------------------

    #[test]
    fn flag_members_read_and_write_the_quest_data_bits() {
        let (mut e, quest) = world();
        set_flags(&mut e, quest, 0x01);
        e.call(0x0060_cd00, &args![quest, 5u32]);
        assert_eq!(flags(&e, quest), 0x11);
        e.call(0x0060_cd00, &args![quest, 0u32]);
        assert_eq!(flags(&e, quest), 0x01);
        assert!(!e.call(0x0060_d5e0, &args![quest]).bool());
        set_flags(&mut e, quest, 0x08);
        assert!(e.call(0x0060_d5e0, &args![quest]).bool());
    }

    #[test]
    fn get_current_stage_and_the_item_setters_use_their_bytes() {
        let (mut e, quest) = world();
        e.set(quest, TESQuest::cCurrentStage, 17);
        assert_eq!(e.call(0x0060_d700, &args![quest]).u8(), 17);
        let item: Ptr<TESQuestStageItem> = e.new_object();
        e.call(0x0060_d490, &args![item, 1u32]);
        e.call(0x0060_d4b0, &args![item, 9u32]);
        assert_eq!(e.mem.u8(item.addr() + 0x65), 1);
        assert_eq!(e.mem.u8(item.addr() + 0x64), 9);
    }

    #[test]
    fn swap_float_swaps_the_delay_time_in_place() {
        let (mut e, quest) = world();
        stub(&mut e, SWAP_FLOAT, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x0060_ce80,
            &args![Ptr::<QuestData>::new(quest.addr() + DATA)],
        );
        assert_eq!(calls_to(&e, SWAP_FLOAT), [[quest.addr() + DATA + 4, 0]]);
    }

    #[test]
    fn run_script_needs_a_script_and_passes_the_locals() {
        let (mut e, quest) = world();
        stub(&mut e, SCRIPT_RUN, 0);
        e.call_log = Some(vec![]);
        e.call(0x0060_d4d0, &args![quest]);
        assert!(calls_to(&e, SCRIPT_RUN).is_empty());
        e.mem.set_u32(quest.addr() + 0x1c, 0xabc0);
        e.set(quest, TESQuest::pScriptLocals, Ptr::new(0x5555));
        e.call(0x0060_d4d0, &args![quest]);
        assert_eq!(calls_to(&e, SCRIPT_RUN), [[0xabc0, 0, 0x5555, 0, 0]]);
    }

    // ---- stage queries ---------------------------------------------------

    #[test]
    fn set_stage_done_marks_the_stage_raises_the_current_stage_and_flags_the_change() {
        let (mut e, quest) = world();
        let (first, second) = (stage(&mut e, 10, 0), stage(&mut e, 20, 0));
        write_list(&mut e, quest.addr() + 0x44, &[first, second]);
        stub(&mut e, SET_ENABLED, 0);
        stub(&mut e, STAGE_MARK_DONE, 0);
        e.set(quest, TESQuest::cCurrentStage, 5);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0060_d510, &args![quest, 20u32]).bool());
        assert_eq!(e.get(quest, TESQuest::cCurrentStage), 20);
        // Not in the bit-1 state: the quest is enabled.
        assert_eq!(calls_to(&e, SET_ENABLED), [[quest.addr(), 1]]);
        assert_eq!(calls_to(&e, STAGE_MARK_DONE), [[second, quest.addr(), 0]]);
        assert_eq!(
            calls_to(&e, ADD_CHANGE),
            [[quest.addr(), CHANGE_QUEST_STAGES]]
        );
    }

    #[test]
    fn set_stage_done_keeps_a_higher_current_stage_and_respects_the_bit_1_state() {
        let (mut e, quest) = world();
        let first = stage(&mut e, 10, 0);
        write_list(&mut e, quest.addr() + 0x44, &[first]);
        stub(&mut e, SET_ENABLED, 0);
        stub(&mut e, STAGE_MARK_DONE, 0);
        e.set(quest, TESQuest::cCurrentStage, 50);
        // Bit 1 set and bit 3 clear: no enabling.
        set_flags(&mut e, quest, 0x02);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0060_d510, &args![quest, 10u32]).bool());
        assert_eq!(e.get(quest, TESQuest::cCurrentStage), 50);
        assert!(calls_to(&e, SET_ENABLED).is_empty());
        // Bits 1 and 3: enabling again.
        set_flags(&mut e, quest, 0x0a);
        e.call(0x0060_d510, &args![quest, 10u32]);
        assert_eq!(calls_to(&e, SET_ENABLED).len(), 1);
    }

    #[test]
    fn set_stage_done_returns_false_for_an_unknown_stage_or_no_stages() {
        let (mut e, quest) = world();
        stub(&mut e, STAGE_MARK_DONE, 0);
        assert!(!e.call(0x0060_d510, &args![quest, 1u32]).bool());
        let first = stage(&mut e, 10, 0);
        write_list(&mut e, quest.addr() + 0x44, &[first]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0060_d510, &args![quest, 11u32]).bool());
        assert!(calls_to(&e, STAGE_MARK_DONE).is_empty());
        assert!(calls_to(&e, ADD_CHANGE).is_empty());
    }

    #[test]
    fn stage_done_byte_lookup() {
        let (mut e, quest) = world();
        let (first, second) = (stage(&mut e, 10, 0), stage(&mut e, 20, 1));
        write_list(&mut e, quest.addr() + 0x44, &[first, second]);
        assert_eq!(e.call(0x0060_d600, &args![quest, 20u32]).u8(), 1);
        assert_eq!(e.call(0x0060_d600, &args![quest, 10u32]).u8(), 0);
        assert_eq!(e.call(0x0060_d600, &args![quest, 30u32]).u8(), 0);
    }

    #[test]
    fn highest_done_stage_raises_the_current_stage() {
        let (mut e, quest) = world();
        let stages = [
            stage(&mut e, 10, 1),
            stage(&mut e, 40, 0),
            stage(&mut e, 30, 1),
            stage(&mut e, 20, 1),
        ];
        write_list(&mut e, quest.addr() + 0x44, &stages);
        e.set(quest, TESQuest::cCurrentStage, 5);
        e.call(0x0060_d670, &args![quest]);
        assert_eq!(e.get(quest, TESQuest::cCurrentStage), 30);
        // A larger current stage stays.
        e.set(quest, TESQuest::cCurrentStage, 99);
        e.call(0x0060_d670, &args![quest]);
        assert_eq!(e.get(quest, TESQuest::cCurrentStage), 99);
        // No stages at all.
        let (mut e, empty) = world();
        e.set(empty, TESQuest::cCurrentStage, 3);
        e.call(0x0060_d670, &args![empty]);
        assert_eq!(e.get(empty, TESQuest::cCurrentStage), 3);
    }

    // ---- InitItem --------------------------------------------------------

    /// Doubles for the callees of `InitItem`; returns the quest.
    fn init_world() -> (Engine, Ptr<TESQuest>) {
        let (mut e, quest) = world();
        for addr in [
            SCRIPTABLE_INIT_ITEM,
            SCRIPT_SET_OWNER,
            FORCE_CHANGE,
            SET_ENABLED,
            CONDITION_INIT_ITEM,
            OBJECTIVE_INIT_ITEM,
            SET_FORM_FLAG_CHANGED,
            STAGE_ITEM_INIT,
        ] {
            stub(&mut e, addr, 0);
        }
        e.register(SCRIPT_CREATE_LOCALS, |_, a| eax(a[0] + 0x1000));
        let setting = e.mem.alloc(8);
        e.register_double(SETTING_BOOL, move |_, a| {
            assert_eq!(a, [SETTING_OBJECT]);
            eax(setting + 4)
        });
        e.mem.set_u8(setting + 4, 0);
        (e, quest)
    }

    #[test]
    fn init_item_does_nothing_for_a_flagged_form() {
        let (mut e, quest) = init_world();
        e.mem.set_u32(quest.addr() + 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x0060_cba0, &args![quest]);
        assert_eq!(addresses(&e), [0x0060_cba0, FORM_FLAG_TEST_BIT_3]);
    }

    #[test]
    fn init_item_without_script_or_lists_only_sets_the_data_bit_and_flags_the_form() {
        let (mut e, quest) = init_world();
        set_flags(&mut e, quest, 1);
        e.call_log = Some(vec![]);
        e.call(0x0060_cba0, &args![quest]);
        assert!(calls_to(&e, SCRIPT_CREATE_LOCALS).is_empty());
        assert_eq!(flags(&e, quest), 0x11);
        assert_eq!(
            calls_to(&e, SCRIPTABLE_INIT_ITEM),
            [[quest.addr() + 0x18, quest.addr()]]
        );
        assert_eq!(calls_to(&e, SET_FORM_FLAG_CHANGED), [[quest.addr(), 1]]);
        // The condition list is initialized because its address is never null.
        assert_eq!(
            calls_to(&e, CONDITION_INIT_ITEM),
            [[quest.addr() + 0x54, quest.addr()]]
        );
    }

    #[test]
    fn init_item_with_a_script_creates_locals_and_enables_by_setting() {
        let (mut e, quest) = init_world();
        e.mem.set_u32(quest.addr() + 0x1c, 0x8000);
        e.call_log = Some(vec![]);
        e.call(0x0060_cba0, &args![quest]);
        assert_eq!(e.get(quest, TESQuest::pScriptLocals), Ptr::new(0x9000));
        assert_eq!(calls_to(&e, SCRIPT_SET_OWNER), [[0x8000, quest.addr()]]);
        assert_eq!(
            calls_to(&e, FORCE_CHANGE),
            [[quest.addr(), CHANGE_SCRIPT_LOCALS]]
        );
        // The setting is off: the quest is not enabled.
        assert!(calls_to(&e, SET_ENABLED).is_empty());
        // The setting on enables it.
        let (mut e, quest) = init_world();
        e.mem.set_u32(quest.addr() + 0x1c, 0x8000);
        let setting = e.mem.alloc(8);
        e.mem.set_u8(setting + 4, 1);
        e.register_double(SETTING_BOOL, move |_, _| eax(setting + 4));
        e.call_log = Some(vec![]);
        e.call(0x0060_cba0, &args![quest]);
        assert_eq!(calls_to(&e, SET_ENABLED), [[quest.addr(), 1]]);
    }

    #[test]
    fn init_item_initializes_every_stage_and_objective() {
        let (mut e, quest) = init_world();
        let (first, second) = (stage(&mut e, 10, 0), stage(&mut e, 20, 0));
        write_list(&mut e, quest.addr() + 0x44, &[first, second]);
        let item = e.mem.alloc(0x74);
        e.mem.set_u8(item + 0xc + 0x18 + 0x10, 0xff);
        write_list(&mut e, first + 4, &[item]);
        write_list(&mut e, second + 4, &[]);
        write_list(&mut e, quest.addr() + 0x4c, &[0x1111, 0x2222]);
        e.call_log = Some(vec![]);
        e.call(0x0060_cba0, &args![quest]);
        assert_eq!(calls_to(&e, STAGE_ITEM_INIT), [[item, quest.addr(), first]]);
        // The byte at +0x10 of the item's script data is cleared.
        assert_eq!(e.mem.u8(item + 0xc + 0x18 + 0x10), 0);
        assert_eq!(
            calls_to(&e, OBJECTIVE_INIT_ITEM),
            [[0x1111, quest.addr()], [0x2222, quest.addr()]]
        );
    }

    // ---- Save ------------------------------------------------------------

    #[test]
    fn save_writes_components_data_conditions_stages_and_objectives() {
        let (mut e, quest) = world();
        for addr in [
            START_FORM,
            SCRIPTABLE_SAVE,
            FULL_NAME_SAVE,
            ICON_SAVE,
            SAVE_DATA,
            CONDITION_SAVE,
            STAGE_SAVE,
            OBJECTIVE_SAVE,
            CLOSE_FORM,
            SWAP_FLOAT,
        ] {
            stub(&mut e, addr, 0);
        }
        stub(&mut e, SAVE_SWAP_FLAG, 1);
        write_list(&mut e, quest.addr() + 0x44, &[0xa1, 0xa2]);
        write_list(&mut e, quest.addr() + 0x4c, &[0xb1]);
        e.call_log = Some(vec![]);
        e.call(0x0060_cd60, &args![quest]);
        let base = quest.addr();
        assert_eq!(
            addresses(&e),
            [
                0x0060_cd60,
                START_FORM,
                SCRIPTABLE_SAVE,
                FULL_NAME_SAVE,
                ICON_SAVE,
                SAVE_SWAP_FLAG,
                SWAP_FLOAT,
                SAVE_DATA,
                SAVE_SWAP_FLAG,
                SWAP_FLOAT,
                CONDITION_LIST,
                CONDITION_LIST,
                CONDITION_SAVE,
                STAGE_LIST,
                NODE_IS_EMPTY,
                RETURN_THIS,
                STAGE_SAVE,
                WORD_AT_4,
                NODE_IS_EMPTY,
                RETURN_THIS,
                STAGE_SAVE,
                WORD_AT_4,
                OBJECTIVE_LIST,
                NODE_IS_EMPTY,
                RETURN_THIS,
                OBJECTIVE_SAVE,
                WORD_AT_4,
                CLOSE_FORM
            ]
        );
        assert_eq!(calls_to(&e, SCRIPTABLE_SAVE), [[base + 0x18]]);
        assert_eq!(calls_to(&e, FULL_NAME_SAVE), [[base + 0x30]]);
        assert_eq!(calls_to(&e, ICON_SAVE), [[base + 0x24, CHUNK_ICON]]);
        assert_eq!(calls_to(&e, SAVE_DATA), [[base, base + 0x3c, 8]]);
        assert_eq!(calls_to(&e, CONDITION_SAVE), [[base + 0x54]]);
        assert_eq!(calls_to(&e, STAGE_SAVE), [[0xa1], [0xa2]]);
        assert_eq!(calls_to(&e, OBJECTIVE_SAVE), [[0xb1]]);
    }

    #[test]
    fn save_does_not_swap_without_the_swap_flag() {
        let (mut e, quest) = world();
        for addr in [
            START_FORM,
            SCRIPTABLE_SAVE,
            FULL_NAME_SAVE,
            ICON_SAVE,
            SAVE_DATA,
            CONDITION_SAVE,
            CLOSE_FORM,
            SWAP_FLOAT,
        ] {
            stub(&mut e, addr, 0);
        }
        stub(&mut e, SAVE_SWAP_FLAG, 0);
        e.call_log = Some(vec![]);
        e.call(0x0060_cd60, &args![quest]);
        assert!(calls_to(&e, SWAP_FLOAT).is_empty());
    }

    // ---- Load ------------------------------------------------------------

    /// Everything `Load` records: list insertions (list, item), the sizes
    /// asked of `operator new`, and the file position.
    struct LoadLog {
        inserts: Rc<RefCell<Vec<(u32, u32)>>>,
        sizes: Rc<RefCell<Vec<u32>>>,
        position: Rc<Cell<usize>>,
    }

    /// Doubles for the file and every callee of `Load`, serving `chunks`
    /// (type, size). `swap` is the reader's byte-swap flag.
    fn load_world(chunks: Vec<(u32, u32)>, swap: bool) -> (Engine, Ptr<TESQuest>, LoadLog) {
        let (mut e, quest) = world();
        e.register(FILE_RECORD_TYPE, |_, _| eax(0x47));
        let position = Rc::new(Cell::new(0usize));
        let (p1, c1) = (position.clone(), chunks.clone());
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            eax(c1.get(p1.get()).map_or(0, |c| c.0))
        });
        let (p2, c2) = (position.clone(), chunks.clone());
        e.register_double(FILE_CHUNK_SIZE, move |_, _| {
            eax(c2.get(p2.get()).map_or(0, |c| c.1))
        });
        let (p3, c3) = (position.clone(), chunks);
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            p3.set(p3.get() + 1);
            eax((p3.get() < c3.len()) as u32)
        });
        // The item loader consumes the chunk it is called for.
        let p4 = position.clone();
        e.register_double(STAGE_ITEM_LOAD, move |_, _| {
            p4.set(p4.get() + 1);
            eax(0)
        });
        e.register(FILE_GET_CHUNK_DATA, |e, a| {
            // Stage data: number 7, done byte 1.
            e.mem.set_u8(a[1], 7);
            e.mem.set_u8(a[1] + 1, 1);
            Ret::default()
        });
        e.register(FILE_GET_CHUNK_DWORD, |e, a| {
            e.mem.set_u32(a[1], 0xcafe);
            Ret::default()
        });
        stub(&mut e, FILE_SWAP_FLAG, swap as u32);
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let s = sizes.clone();
        e.register_double(OPERATOR_NEW, move |e, a| {
            s.borrow_mut().push(a[0]);
            eax(e.mem.alloc(a[0]))
        });
        let inserts = Rc::new(RefCell::new(Vec::new()));
        let i = inserts.clone();
        e.register_double(LIST_INSERT, move |e, a| {
            i.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.register(OBJECTIVE_CONSTRUCT, |_, a| eax(a[0]));
        e.register(TARGET_CONSTRUCT, |_, a| eax(a[0]));
        e.register(OBJECTIVE_TARGET_LIST, |_, a| eax(a[0] + 0x14));
        for addr in [
            LOAD_FORM,
            SET_FORM_FLAG_CHANGED,
            SCRIPTABLE_SET_SCRIPT,
            SCRIPTABLE_INIT_ITEM,
            FULL_NAME_LOAD_CHUNK,
            ICON_LOAD_CHUNK,
            LOAD_DATA,
            OBJECTIVE_LOAD,
            OBJECTIVE_SET_OWNER,
            TARGET_LOAD,
            CONDITION_LOAD_CHUNK,
            STAGE_DATA_CONSTRUCT,
            LIST_CONSTRUCT,
            CONDITION_CONSTRUCT,
            SCRIPT_CONSTRUCT,
            MEMSET,
            SET_TEMPORARY,
            SWAP_FLOAT,
            0x7400_0134,
            0x7400_00e0,
        ] {
            stub(&mut e, addr, 0);
        }
        let table = e.mem.u32(quest.addr());
        e.mem.set_u32(table + SLOT_SET_EDITOR_ID, 0x7400_0134);
        e.mem.set_u32(table + SLOT_LOAD_OBJECT_BOUND, 0x7400_00e0);
        (
            e,
            quest,
            LoadLog {
                inserts,
                sizes,
                position,
            },
        )
    }

    #[test]
    fn load_rejects_a_record_that_is_not_a_quest() {
        let (mut e, quest, _) = load_world(vec![], false);
        e.register(FILE_RECORD_TYPE, |_, _| eax(0x46));
        let file = Ptr::<()>::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0060_ceb0, &args![quest, file]).bool());
        assert_eq!(addresses(&e), [0x0060_ceb0, FILE_RECORD_TYPE]);
    }

    #[test]
    fn load_hands_the_simple_chunks_to_their_components() {
        let (mut e, quest, log) = load_world(
            vec![
                (CHUNK_EDID, 6),
                (CHUNK_OBND, 12),
                (CHUNK_SCRI, 4),
                (CHUNK_FULL, 8),
                (CHUNK_ICON, 8),
                (CHUNK_DATA, 8),
                (CHUNK_CTDA, 28),
                (0x1234_5678, 0),
            ],
            false,
        );
        let file = Ptr::<()>::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0060_ceb0, &args![quest, file]).bool());
        let base = quest.addr();
        assert_eq!(log.position.get(), 8);
        assert_eq!(calls_to(&e, LOAD_FORM), [[base, file.addr()]]);
        assert_eq!(calls_to(&e, SET_FORM_FLAG_CHANGED), [[base, 0]]);
        let editor_id = calls_to(&e, 0x7400_0134);
        assert_eq!(editor_id.len(), 1);
        let read = calls_to(&e, FILE_GET_CHUNK_DATA);
        assert_eq!(read[0][1], editor_id[0][1]);
        assert_eq!(read[0][2], 0x200);
        assert_eq!(calls_to(&e, 0x7400_00e0), [[base, file.addr()]]);
        assert_eq!(calls_to(&e, SCRIPTABLE_SET_SCRIPT), [[base + 0x18, 0xcafe]]);
        assert_eq!(calls_to(&e, SCRIPTABLE_INIT_ITEM), [[base + 0x18, base]]);
        assert_eq!(
            calls_to(&e, FULL_NAME_LOAD_CHUNK),
            [[base + 0x30, file.addr()]]
        );
        assert_eq!(calls_to(&e, ICON_LOAD_CHUNK), [[base + 0x24, file.addr()]]);
        assert_eq!(
            calls_to(&e, LOAD_DATA),
            [[base, file.addr(), base + 0x3c, 8]]
        );
        // The quest's condition list takes a condition outside any item or
        // target.
        assert_eq!(
            calls_to(&e, CONDITION_LOAD_CHUNK),
            [[base + 0x54, file.addr()]]
        );
        // No swap flag: the delay time is left alone.
        assert!(calls_to(&e, SWAP_FLOAT).is_empty());
    }

    #[test]
    fn load_builds_objectives_targets_stages_and_items() {
        let (mut e, quest, log) = load_world(
            vec![
                (CHUNK_QOBJ, 4),
                (CHUNK_QSTA, 8),
                (CHUNK_CTDA, 28),
                (CHUNK_INDX, 8),
                (CHUNK_QSDT, 4),
                (CHUNK_SCDA, 4),
                (CHUNK_CNAM, 0),
                (CHUNK_CTDA, 28),
                (CHUNK_QSDT, 4),
                (CHUNK_SCRV, 4),
            ],
            true,
        );
        let file = Ptr::<()>::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0060_ceb0, &args![quest, file]).bool());
        let base = quest.addr();
        let sizes = log.sizes.borrow().clone();
        assert_eq!(sizes, [0x24, 0x48, 0x0c, 0x74, 0x74]);
        let inserts = log.inserts.borrow().clone();
        // Objective, its target, the stage, the two items.
        let objective = inserts[0].1;
        let target = inserts[1].1;
        let stage = inserts[2].1;
        let (item1, item2) = (inserts[3].1, inserts[4].1);
        assert_eq!(inserts[0], (base + 0x4c, objective));
        assert_eq!(inserts[1], (objective + 0x14, target));
        assert_eq!(inserts[2], (base + 0x44, stage));
        assert_eq!(inserts[3], (stage + 4, item1));
        assert_eq!(inserts[4], (stage + 4, item2));
        assert_eq!(inserts.len(), 5);
        assert_eq!(calls_to(&e, OBJECTIVE_LOAD), [[objective, file.addr()]]);
        assert_eq!(calls_to(&e, OBJECTIVE_SET_OWNER), [[objective, base]]);
        assert_eq!(calls_to(&e, TARGET_LOAD), [[target, file.addr()]]);
        // The stage data came from the INDX chunk (8 bytes: first two kept).
        assert_eq!(e.mem.u8(stage), 7);
        assert_eq!(e.mem.u8(stage + 1), 1);
        // Items: running index, the first has a log entry (CNAM).
        assert_eq!(e.mem.u8(item1 + 0x64), 0);
        assert_eq!(e.mem.u8(item2 + 0x64), 1);
        assert_eq!(e.mem.u8(item1 + 0x65), 1);
        assert_eq!(e.mem.u8(item2 + 0x65), 0);
        // Item loads: QSDT (first), SCDA (script chunk), QSDT (second); the
        // trailing SCRV goes to the second item as well.
        assert_eq!(
            calls_to(&e, STAGE_ITEM_LOAD),
            [
                [item1, file.addr()],
                [item1, file.addr()],
                [item2, file.addr()],
                [item2, file.addr()]
            ]
        );
        // The condition after the target (no item yet) went to the target.
        assert_eq!(
            calls_to(&e, CONDITION_LOAD_CHUNK),
            [[target + 4, file.addr()], [item1 + 4, file.addr()]]
        );
        // The reader swaps: the delay time is swapped back at the end.
        assert_eq!(calls_to(&e, SWAP_FLOAT), [[base + 0x3c + 4, 0]]);
    }

    #[test]
    fn load_creates_an_objective_for_a_target_without_one() {
        let (mut e, quest, log) = load_world(vec![(CHUNK_QSTA, 8)], false);
        let file = Ptr::<()>::new(e.mem.alloc(8));
        assert!(e.call(0x0060_ceb0, &args![quest, file]).bool());
        assert_eq!(*log.sizes.borrow(), [0x48, 0x24]);
        let inserts = log.inserts.borrow().clone();
        assert_eq!(inserts.len(), 2);
        assert_eq!(inserts[0].0, quest.addr() + 0x4c);
        let objective = inserts[0].1;
        assert_eq!(inserts[1].0, objective + 0x14);
    }

    #[test]
    fn load_leaves_script_chunks_and_names_without_an_item_to_the_advance() {
        let (mut e, quest, log) = load_world(
            vec![(CHUNK_SCDA, 4), (CHUNK_CNAM, 0), (CHUNK_SCHR, 4)],
            false,
        );
        let file = Ptr::<()>::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0060_ceb0, &args![quest, file]).bool());
        assert_eq!(log.position.get(), 3);
        assert!(calls_to(&e, STAGE_ITEM_LOAD).is_empty());
        assert!(log.sizes.borrow().is_empty());
    }

    // ---- stage classes ---------------------------------------------------

    #[test]
    fn stage_constructor_builds_data_and_list() {
        let (mut e, _) = world();
        stub(&mut e, STAGE_DATA_CONSTRUCT, 0);
        stub(&mut e, LIST_CONSTRUCT, 0);
        e.call_log = Some(vec![]);
        let stage: Ptr<TESQuestStage> = e.new_object();
        let back = e.call(0x0060_f210, &args![stage]).ptr::<TESQuestStage>();
        assert_eq!(back, stage);
        assert_eq!(calls_to(&e, STAGE_DATA_CONSTRUCT), [[stage.addr()]]);
        assert_eq!(calls_to(&e, LIST_CONSTRUCT), [[stage.addr() + 4]]);
    }

    #[test]
    fn stage_data_load_reads_the_indx_chunk_in_both_sizes() {
        let (mut e, _) = world();
        let stage: Ptr<TESQuestStage> = e.new_object();
        let file = Ptr::<()>::new(e.mem.alloc(8));
        e.register(FILE_CHUNK_TYPE, |_, _| eax(CHUNK_INDX));
        e.register(FILE_CHUNK_SIZE, |_, _| eax(8));
        e.register(FILE_GET_CHUNK_DATA, |e, a| {
            assert_eq!(a[2], 8);
            e.mem
                .write(a[1], &[3, 1, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee]);
            Ret::default()
        });
        e.call(0x0060_f440, &args![stage, file]);
        assert_eq!(e.get(stage, TESQuestStage::ucIndex), 3);
        assert_eq!(e.get(stage, TESQuestStage::bIsDone), 1);
        // Any other size reads two bytes straight into the stage.
        let other: Ptr<TESQuestStage> = e.new_object();
        e.register(FILE_CHUNK_SIZE, |_, _| eax(2));
        e.register(FILE_GET_CHUNK_DATA, |e, a| {
            assert_eq!(a[2], 2);
            e.mem.write(a[1], &[9, 1]);
            Ret::default()
        });
        e.call(0x0060_f440, &args![other, file]);
        assert_eq!(e.get(other, TESQuestStage::ucIndex), 9);
        assert_eq!(e.get(other, TESQuestStage::bIsDone), 1);
    }

    #[test]
    fn stage_data_load_ignores_a_null_file_and_other_chunks() {
        let (mut e, _) = world();
        let stage: Ptr<TESQuestStage> = e.new_object();
        let file = Ptr::<()>::new(e.mem.alloc(8));
        e.register(FILE_CHUNK_TYPE, |_, _| eax(CHUNK_DATA));
        e.call_log = Some(vec![]);
        e.call(0x0060_f440, &args![stage, Ptr::<()>::NULL]);
        assert_eq!(addresses(&e), [0x0060_f440]);
        e.call(0x0060_f440, &args![stage, file]);
        assert_eq!(addresses(&e), [0x0060_f440, 0x0060_f440, FILE_CHUNK_TYPE]);
    }

    #[test]
    fn stage_item_constructor_initializes_the_script_and_clears_the_fields() {
        let (mut e, _) = world();
        for addr in [
            CONDITION_CONSTRUCT,
            SCRIPT_CONSTRUCT,
            MEMSET,
            SET_FORM_FLAG_CHANGED,
            SET_TEMPORARY,
        ] {
            stub(&mut e, addr, 0);
        }
        let item: Ptr<TESQuestStageItem> = e.new_object();
        // Dirty memory: everything the constructor clears is non-zero.
        for offset in (0..0x74).step_by(4) {
            e.mem.set_u32(item.addr() + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        let back = e.call(0x0060_f520, &args![item]).ptr::<TESQuestStageItem>();
        assert_eq!(back, item);
        let base = item.addr();
        assert_eq!(
            addresses(&e)[1..],
            [
                CONDITION_CONSTRUCT,
                SCRIPT_CONSTRUCT,
                MEMSET,
                SET_FORM_FLAG_CHANGED,
                SET_TEMPORARY,
                SCRIPT_FORM_DATA
            ]
        );
        assert_eq!(calls_to(&e, CONDITION_CONSTRUCT), [[base + 4]]);
        assert_eq!(calls_to(&e, SCRIPT_CONSTRUCT), [[base + 0xc]]);
        assert_eq!(calls_to(&e, MEMSET), [[base, 0, 1]]);
        assert_eq!(calls_to(&e, SET_FORM_FLAG_CHANGED), [[base + 0xc, 0]]);
        assert_eq!(calls_to(&e, SET_TEMPORARY), [[base + 0xc]]);
        // The script data byte is set, the fields are cleared.
        assert_eq!(e.mem.u8(base + 0xc + 0x18 + 0x10), 1);
        for offset in [0x60, 0x68, 0x6c, 0x70] {
            assert_eq!(e.mem.u32(base + offset), 0, "{offset:x}");
        }
        assert_eq!(e.mem.u8(base + 0x64), 0);
        assert_eq!(e.mem.u8(base + 0x65), 0);
    }
}

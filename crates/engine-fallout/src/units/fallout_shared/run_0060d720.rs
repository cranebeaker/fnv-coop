//! `fallout shared/run_0060d720` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `strlen`-style length (`0044a670`, `cdecl`), `strcpy_s` (`00406d30`,
/// `cdecl` (dest, size, src)), `BSStringT<char>::StrCmp` (`00408a80`) and
/// the `BSStringT<char>` text assignment (`004037f0`).
const STRING_LENGTH: u32 = 0x0044_a670;
const STRCPY_S: u32 = 0x0040_6d30;
const STRING_COMPARE: u32 = 0x0040_8a80;
const STRING_ASSIGN: u32 = 0x0040_37f0;
/// The quest's editor id (`cFormEditorID`, `BSStringT<char>`, +0x64).
const EDITOR_ID: u32 = 0x64;

// Translated from 0060dab0 (PC disassembly, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0060dab0 - the /GS stack cookie check (00ec408c) is not translated.
/// The quest's `SetEditorID` (vtable slot; `thiscall`, `RET 4`): copies
/// `name` to a stack buffer of its length + 1 (the compiler's `alloca`,
/// `00ec5ec0`, here the engine's stack), and answers false when the editor
/// id (+0x64) already equals it (`StrCmp` with case ignored); else stores
/// it there and answers true.
pub fn fn_0060dab0(e: &mut Engine, this: Ptr, name: Ptr) -> bool {
    let size = e.call(STRING_LENGTH, &args![name]).u32().wrapping_add(1);
    e.with_stack(size, |e, buffer| {
        e.call(STRCPY_S, &args![buffer, size, name]);
        let editor_id = this.addr() + EDITOR_ID;
        if e.call(STRING_COMPARE, &args![editor_id, buffer, 1u32])
            .u32()
            == 0
        {
            return false;
        }
        e.call(STRING_ASSIGN, &args![editor_id, buffer, 0u32]);
        true
    })
}

/// Copies the save buffer's change flags (the word at +0x17 of the
/// buffer) into a local (`00428110`, thiscall on the buffer, the local's
/// address; returns that address).
const COPY_CHANGE_FLAGS: u32 = 0x0042_8110;
/// Tests a mask against the flags at its `this` (`004280f0`, thiscall, the
/// mask; a `bool` in AL).
const FLAGS_TEST: u32 = 0x0042_80f0;
/// `005621f0` (`tesobjectrefr.cpp`; thiscall on the buffer, one word).
const BUFFER_CALL_005621F0: u32 = 0x0056_21f0;
/// `00484d60` (`tesform.cpp`; thiscall on `this`, the buffer): the form's
/// own part of the save.
const FORM_SAVE_00484D60: u32 = 0x0048_4d60;
/// `BGSSaveGameBuffer`'s byte save (`00865e50`; source, size, a third word),
/// `StartVariableSizedValue` (`00865f20`) and `SaveVariableSizedValue_ov2`
/// (`00865ff0`; the count and the start position).
const SAVE_BYTES: u32 = 0x0086_5e50;
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
const FINISH_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
/// List helpers: the item pointer's address of a node (`006815c0`) and the
/// next node (`00726070`; the code also runs it on an item at `0060e810`).
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// Per-item getters (`009373f0`, `005dc960`, `005dc980`, each a byte in AL;
/// `00717e50`, a list head; `008d8520`, a pointer; `007af430`, a word).
const ITEM_FLAG_009373F0: u32 = 0x0093_73f0;
const ITEM_FLAG_005DC960: u32 = 0x005d_c960;
const ITEM_FLAG_005DC980: u32 = 0x005d_c980;
const ITEM_LIST_00717E50: u32 = 0x0071_7e50;
const ITEM_POINTER_008D8520: u32 = 0x008d_8520;
const ITEM_VALUE_007AF430: u32 = 0x007a_f430;
/// `ScriptLocals::SaveGame` (`005a9db0`, thiscall on the locals, the
/// buffer).
const SCRIPT_LOCALS_SAVE_GAME: u32 = 0x005a_9db0;
/// The fields `0060e810` saves (offsets as the code reads them): a byte at
/// +0x3c, a word at +0x40, the lists whose first nodes sit at +0x44 and
/// +0x4c, and the script locals pointer at +0x5c (also read by `0060e7c0`).
const BYTE_3C: u32 = 0x3c;
const WORD_40: u32 = 0x40;
const LIST_44: u32 = 0x44;
const LIST_4C: u32 = 0x4c;
const SCRIPT_LOCALS: u32 = 0x5c;
/// Other list helpers of `0060efd0` and `0060f110`: the first node of the
/// object's list (`005d43e0`), a test of an item (`005f65d0`, thiscall on
/// the second argument, the item's address), a check of the item's word
/// (`007af430`), the item's sub-list (`007d6bb0`), its reference
/// (`006101b0`, one word), its conditions (`00717e50`), their test
/// (`00680c30`, two words), the end check (`008256d0`), the start check
/// (`0059e400`), `00470470` on the first argument and `005ec500` on a
/// matched item (one word).
const FIRST_NODE_005D43E0: u32 = 0x005d_43e0;
const ITEM_TEST_005F65D0: u32 = 0x005f_65d0;
const SUB_LIST_007D6BB0: u32 = 0x007d_6bb0;
const ITEM_REFERENCE_006101B0: u32 = 0x0061_01b0;
const CONDITIONS_TRUE_00680C30: u32 = 0x0068_0c30;
const CURSOR_DONE_008256D0: u32 = 0x0082_56d0;
const SKIP_CHECK_0059E400: u32 = 0x0059_e400;
const ARGUMENT_CALL_00470470: u32 = 0x0047_0470;
const ITEM_CALL_005EC500: u32 = 0x005e_c500;

/// `004280f0(00428110(buffer, &local), mask)`: whether the save buffer's
/// change flags (copied to a stack local) have any bit of `mask`.
fn change_flags_have(e: &mut Engine, buffer: Ptr, mask: u32) -> bool {
    e.with_stack(4, |e, local| {
        let flags = e.call(COPY_CHANGE_FLAGS, &args![buffer, local]).u32();
        e.call(FLAGS_TEST, &args![flags, mask]).u32() & 0xff != 0
    })
}

/// `00865e50(buffer, source, size, 0)`.
fn save_bytes(e: &mut Engine, buffer: Ptr, source: u32, size: u32) {
    e.call(SAVE_BYTES, &args![buffer, source, size, 0u32]);
}

/// Saves a stack copy of `value`'s low `size` bytes.
fn save_local(e: &mut Engine, buffer: Ptr, value: u32, size: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        save_bytes(e, buffer, local.addr(), size);
    });
}

/// The item a list node holds (`*006815c0(node)`).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

// Translated from 0060e7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the buffer's change flags have bit 30 (`0x40000000`) and the word
/// at +0x5c of `this` is null, calls `005621f0(buffer, 0x40000000)`
/// (thiscall, `RET 4`; unnamed in the engine map).
pub fn fn_0060e7c0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    if change_flags_have(e, buffer, 0x4000_0000) && e.mem.u32(this.addr() + SCRIPT_LOCALS) == 0 {
        e.call(BUFFER_CALL_005621F0, &args![buffer, 0x4000_0000u32]);
    }
}

// Translated from 0060e810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the object into the save buffer (thiscall, `RET 4`; unnamed in
/// the engine map), as the buffer's change flags select:
/// the form's part (`00484d60`) always; flag 2: the byte at +0x3c; flag 4:
/// the word at +0x40; flag `0x80000000`: a counted list of the items of the
/// list at +0x44, each two bytes (`009373f0`, `005dc960`) and a counted
/// list of its sub-items (`00717e50`), each a byte (`005dc980`), a byte
/// saying whether `008d8520` gives a pointer and then the 4 bytes that
/// pointer points to; flag `0x40000000`: the script locals at +0x5c
/// (`ScriptLocals::SaveGame`); flag `0x20000000`: a counted list of the
/// items of the list at +0x4c whose word (`007af430`) is non-zero, each
/// `00726070(item)` and that word (4 bytes each). Counts are
/// variable-sized values patched in after their lists.
pub fn fn_0060e810(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    e.call(FORM_SAVE_00484D60, &args![this, buffer]);
    if change_flags_have(e, buffer, 2) {
        save_bytes(e, buffer, base + BYTE_3C, 1);
    }
    if change_flags_have(e, buffer, 4) {
        save_bytes(e, buffer, base + WORD_40, 4);
    }
    if change_flags_have(e, buffer, 0x8000_0000) {
        let mut count = 0u32;
        let start = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
        let mut node = base + LIST_44;
        while node != 0 {
            let item = node_item(e, node);
            if item != 0 {
                let first = e.call(ITEM_FLAG_009373F0, &args![item]).u32() & 0xff;
                let second = e.call(ITEM_FLAG_005DC960, &args![item]).u32() & 0xff;
                save_local(e, buffer, first, 1);
                save_local(e, buffer, second, 1);
                let mut inner_count = 0u32;
                let inner_start = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
                let mut sub = e.call(ITEM_LIST_00717E50, &args![item]).u32();
                while sub != 0 {
                    let entry = node_item(e, sub);
                    if entry != 0 {
                        let byte = e.call(ITEM_FLAG_005DC980, &args![entry]).u32() & 0xff;
                        let pointer = e.call(ITEM_POINTER_008D8520, &args![entry]).u32();
                        let has_pointer = u32::from(pointer != 0);
                        save_local(e, buffer, byte, 1);
                        save_local(e, buffer, has_pointer, 1);
                        if has_pointer != 0 {
                            save_bytes(e, buffer, pointer, 4);
                        }
                        inner_count = inner_count.wrapping_add(1);
                    }
                    sub = e.call(NODE_NEXT, &args![sub]).u32();
                }
                e.call(
                    FINISH_VARIABLE_SIZED_VALUE,
                    &args![buffer, inner_count, inner_start],
                );
                count = count.wrapping_add(1);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        e.call(FINISH_VARIABLE_SIZED_VALUE, &args![buffer, count, start]);
    }
    if change_flags_have(e, buffer, 0x4000_0000) {
        let locals = e.mem.u32(base + SCRIPT_LOCALS);
        e.call(SCRIPT_LOCALS_SAVE_GAME, &args![locals, buffer]);
    }
    if change_flags_have(e, buffer, 0x2000_0000) {
        let mut count = 0u32;
        let start = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
        let mut node = base + LIST_4C;
        while node != 0 {
            let item = node_item(e, node);
            if item != 0 {
                let word = e.call(ITEM_VALUE_007AF430, &args![item]).u32();
                if word != 0 {
                    let other = e.call(NODE_NEXT, &args![item]).u32();
                    save_local(e, buffer, other, 4);
                    save_local(e, buffer, word, 4);
                    count = count.wrapping_add(1);
                }
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        e.call(FINISH_VARIABLE_SIZED_VALUE, &args![buffer, count, start]);
    }
}

// Translated from 0060efd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checks the object's list against the list `cursor` (thiscall, `RET 8`;
/// unnamed in the engine map). Without a cursor or a first node
/// (`005d43e0`) only the end check runs. Otherwise for each item of the
/// object's list whose word (`007af430`) is 1 and that `005f65d0(other,
/// item)` accepts, each non-null entry of the item's sub-list
/// (`007d6bb0`) whose conditions (`00717e50`) hold for its reference
/// (`00680c30(conditions, 006101b0(entry, 0), 0)`) must be the cursor's
/// current item (false otherwise, also when the cursor or its item is
/// null), and the cursor moves on. At the end a cursor left non-null must
/// pass `008256d0`. Returns true when everything matched.
pub fn fn_0060efd0(e: &mut Engine, this: Ptr, cursor: u32, other: u32) -> bool {
    let mut cursor = cursor;
    let mut node = e.call(FIRST_NODE_005D43E0, &args![this]).u32();
    if cursor != 0 && node != 0 {
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            if e.mem.u32(slot) != 0 {
                let item = node_item(e, node);
                if e.call(ITEM_VALUE_007AF430, &args![item]).u32() == 1 {
                    let item_slot = e.call(NODE_ITEM, &args![node]).u32();
                    if e.call(ITEM_TEST_005F65D0, &args![other, item_slot]).u32() & 0xff != 0 {
                        let item = node_item(e, node);
                        let mut entry_node = e.call(SUB_LIST_007D6BB0, &args![item]).u32();
                        while entry_node != 0 {
                            let entry = node_item(e, entry_node);
                            if entry != 0 {
                                let reference =
                                    e.call(ITEM_REFERENCE_006101B0, &args![entry, 0u32]).u32();
                                let conditions = e.call(ITEM_LIST_00717E50, &args![entry]).u32();
                                let holds = e
                                    .call(
                                        CONDITIONS_TRUE_00680C30,
                                        &args![conditions, reference, 0u32],
                                    )
                                    .u32()
                                    & 0xff
                                    != 0;
                                if holds {
                                    if cursor == 0 {
                                        return false;
                                    }
                                    if node_item(e, cursor) == 0 {
                                        return false;
                                    }
                                    if node_item(e, cursor) != entry {
                                        return false;
                                    }
                                    if cursor != 0 {
                                        cursor = e.call(NODE_NEXT, &args![cursor]).u32();
                                    }
                                }
                            }
                            entry_node = e.call(NODE_NEXT, &args![entry_node]).u32();
                        }
                    }
                }
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
    if cursor != 0 && e.call(CURSOR_DONE_008256D0, &args![cursor]).u32() & 0xff == 0 {
        return false;
    }
    true
}

// Translated from 0060f110 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a non-null `target` (thiscall, `RET 8`; unnamed in the engine
/// map): `00470470(target)`; unless `0059e400(this)` answers true, every
/// item of the object's list (from `005d43e0`, stopping at the first null
/// item) that `005f65d0(other, item)` accepts gets
/// `005ec500(item, target)`.
pub fn fn_0060f110(e: &mut Engine, this: Ptr, target: u32, other: u32) {
    if target == 0 {
        return;
    }
    e.call(ARGUMENT_CALL_00470470, &args![target]);
    if e.call(SKIP_CHECK_0059E400, &args![this]).u32() & 0xff != 0 {
        return;
    }
    let mut node = e.call(FIRST_NODE_005D43E0, &args![this]).u32();
    while node != 0 {
        if node_item(e, node) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.call(ITEM_TEST_005F65D0, &args![other, slot]).u32() & 0xff != 0 {
            let item = node_item(e, node);
            e.call(ITEM_CALL_005EC500, &args![item, target]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// `005d43c0` (`ECX` = quest): the first node of its stage list.
const STAGE_LIST: u32 = 0x005d_43c0;
/// `0067fdb0` (`ECX` = stage, a byte; `RET 4`): stores the byte at the
/// stage's +1.
const STAGE_SET_BYTE_1: u32 = 0x0067_fdb0;
/// `00407800` (`ECX` = stage item, a word; `RET 4`): stores the item's
/// word (the engine map's name, `MiddleHighProcess::SetSavedAcquireObject`,
/// is a folded body).
const ITEM_SET_WORD: u32 = 0x0040_7800;
/// `007b3fa0` (`ECX` = the item's object, a word; `RET 4`): its scalar
/// deleting destructor as the code uses it (with 1).
const DELETE_OBJECT: u32 = 0x007b_3fa0;
/// `005ec5d0` (`ECX` = target, a word; `RET 4`).
const TARGET_RESET: u32 = 0x005e_c5d0;
/// `0041af70` (`ECX` = script locals, flags; `RET 4`): their scalar
/// deleting destructor.
const DELETE_LOCALS: u32 = 0x0041_af70;
/// `005abf60` (`ECX` = script): new script locals for it.
const NEW_LOCALS: u32 = 0x005a_bf60;
/// `005a8080` (`ECX` = quest): the address of its flags byte.
const QUEST_FLAGS: u32 = 0x005a_8080;
/// The quest's done/clear callees `0060d720` ends with: `00619b60`
/// (cdecl, the quest), `0060c9c0` (a byte), `0060ca30`, `0060caf0`
/// (`TESQuest::SetFailed`) and `005ec700` (each with 0).
const QUEST_RESET_LIST: u32 = 0x0061_9b60;
/// The player singleton pointer (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `005e3fa0` (`ECX` = objective): its quest.
const OBJECTIVE_QUEST: u32 = 0x005e_3fa0;
/// `0060fdb0` (`TESQuestStageItem::GetLogEntry`, Xbox PDB; `ECX` =
/// objective, the quest; `RET 4`).
const OBJECTIVE_LOG_ENTRY: u32 = 0x0060_fdb0;
/// `005ae3d0` (`ECX` = list, the address of the item; `RET 4`): adds the
/// item at the end; `00905820` adds it at the front.
const LIST_ADD_TAIL: u32 = 0x005a_e3d0;
const LIST_ADD_HEAD: u32 = 0x0090_5820;
/// `0096a2d0` (`ECX` = list): constructs an empty list; `0046ffb0`
/// destroys one.
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DESTROY: u32 = 0x0046_ffb0;
/// `005ae380` (`ECX` = list): its item count.
const LIST_COUNT: u32 = 0x005a_e380;
/// The save/load game object (`011de45c`, a pointer).
const SAVE_LOAD_GAME_OBJECT: u32 = 0x011d_e45c;
/// `TESSaveLoadGame::UseSaveGameBlocks` (`00862110`).
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// `008df040`: the save's version byte.
const SAVE_VERSION: u32 = 0x008d_f040;
/// `00825c00`: the current position in the save data.
const SAVE_POSITION: u32 = 0x0082_5c00;
/// `008579b0` / `008579e0` (`ECX` = save/load game; buffer, size): write
/// and read bytes.
const SAVE_WRITE: u32 = 0x0085_79b0;
const LOAD_READ: u32 = 0x0085_79e0;
/// `00408d60` on the setting at `011de4e8`: whether save sizes are
/// reported.
const SETTING_VALUE: u32 = 0x0040_8d60;
const REPORT_SAVE_SIZES: u32 = 0x011d_e4e8;
/// `004fd3e0` / `004fd3c0`: the record of the form being saved / loaded.
const SAVING_RECORD: u32 = 0x004f_d3e0;
const LOADING_RECORD: u32 = 0x004f_d3c0;
/// `004839c0` (cdecl: form id): the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `Error` (`0040fbe0`) and the warning line writer (`005b5e40`), both
/// cdecl with a format.
const ERROR: u32 = 0x0040_fbe0;
const LOG_WARNING: u32 = 0x005b_5e40;
/// `SaveGameWarning` (`00845cc0`, cdecl with a format).
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// The quest's source file name the reports pass (`0104ad80`).
const QUEST_SOURCE_FILE: u32 = 0x0104_ad80;
/// The warning for a saved stage the quest does not have (`0104adc0`).
const FORMAT_NO_STAGE: u32 = 0x0104_adc0;
/// The block tag written before the quest's save data (`KOLB`).
const BLOCK_TAG: u32 = 0x424c_4f4b;
/// `TESForm::SaveGameDataOLD` (`00484ce0`) and its read (`00484d00`):
/// `ECX` = form; buffer, size.
const SAVE_FORM_DATA: u32 = 0x0048_4ce0;
const LOAD_FORM_DATA: u32 = 0x0048_4d00;
/// `004b0fa0` / `00483710` (`ECX` = a 4-byte stack value): its constructor
/// and destructor; `004b1000` (`ECX` = value, the source; `RET 4`) copies
/// one; `004b1030` and `004b1050` read its two 16-bit halves.
const VALUE_CONSTRUCT: u32 = 0x004b_0fa0;
const VALUE_DESTROY: u32 = 0x0048_3710;
const VALUE_COPY: u32 = 0x004b_1000;
const VALUE_FIRST: u32 = 0x004b_1030;
const VALUE_SECOND: u32 = 0x004b_1050;
/// `00401000` (cdecl: size): allocates.
const ALLOCATE: u32 = 0x0040_1000;
/// `0060f2c0` (`ECX` = stage, a byte; `RET 4`): the stage's item with
/// that index.
const STAGE_ITEM: u32 = 0x0060_f2c0;
/// `0060d670` (`ECX` = quest): run after the stages are loaded.
const QUEST_STAGES_LOADED: u32 = 0x0060_d670;
/// `005a8b80` / `005a8bc0` (`ECX` = script locals): their constructor and
/// destructor.
const LOCALS_CONSTRUCT: u32 = 0x005a_8b80;
const LOCALS_DESTROY: u32 = 0x005a_8bc0;
/// `00864980` (`ECX` = load game buffer; data, size): reads bytes;
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (`00864a60`) a count.
const BUFFER_READ: u32 = 0x0086_4980;
const BUFFER_READ_COUNT: u32 = 0x0086_4a60;
/// `0042ce30` (`ECX` = buffer, the address of a local; `RET 4`): copies
/// the buffer's change flags into the local and returns its address.
const COPY_CHANGE_FLAGS_2: u32 = 0x0042_ce30;

/// Whether the save size reports are on (the setting at `011de4e8`).
fn reports_save_sizes(e: &mut Engine) -> bool {
    let value = e.call(SETTING_VALUE, &args![REPORT_SAVE_SIZES]).u32();
    e.mem.u8(value) != 0
}

fn save_load_game(e: &Engine) -> u32 {
    e.global::<u32>(SAVE_LOAD_GAME_OBJECT)
}

fn uses_save_blocks(e: &mut Engine) -> bool {
    let game = save_load_game(e);
    e.call(USE_SAVE_GAME_BLOCKS, &args![game]).u8() != 0
}

fn save_version(e: &mut Engine) -> u32 {
    let game = save_load_game(e);
    u32::from(e.call(SAVE_VERSION, &args![game]).u8())
}

fn save_position(e: &mut Engine) -> u32 {
    let game = save_load_game(e);
    e.call(SAVE_POSITION, &args![game]).u32()
}

/// The form of a save record (`004839c0` on its form id) and its name
/// (virtual slot 0x130).
fn record_form_name(e: &mut Engine, record: u32) -> u32 {
    let id = e.mem.u32(record);
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    e.vcall(form, 0x130, &[]).u32()
}

/// Reports the bytes written since `start` (`Error` at `line`) with the
/// record of the form being saved when there is one.
fn report_written(e: &mut Engine, written: u32, line: u32) {
    let game = save_load_game(e);
    let record = e.call(SAVING_RECORD, &args![game]).u32();
    if record != 0 {
        let name = record_form_name(e, record);
        let id = e.mem.u32(record);
        let flags = e.mem.u32(record + 5);
        e.call(
            ERROR,
            &args![
                0x0101_53a0u32,
                written,
                id,
                name,
                flags,
                line,
                QUEST_SOURCE_FILE
            ],
        );
    } else {
        e.call(
            ERROR,
            &args![0x0101_536cu32, written, line, QUEST_SOURCE_FILE],
        );
    }
}

/// The block-end check of a load (line `line`): a warning when the position
/// is past or before `end` (the block's start plus its size), with the
/// record of the form being loaded when there is one, else the save
/// version.
fn check_block_end(e: &mut Engine, end: u32, line: u32) {
    let position = save_position(e);
    let game = save_load_game(e);
    let record = e.call(LOADING_RECORD, &args![game]).u32();
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let (format, difference) = if position > end {
            (0x0101_5588u32, position.wrapping_sub(end))
        } else if position < end {
            (0x0101_5500u32, end.wrapping_sub(position))
        } else {
            return;
        };
        let flags = e.mem.u32(record + 5);
        let kind = u32::from(e.mem.u8(record + 9));
        let name = e.vcall(form, 0x130, &[]).u32();
        let id = e.mem.u32(record);
        e.call(
            LOG_WARNING,
            &args![
                format,
                difference,
                QUEST_SOURCE_FILE,
                line,
                id,
                name,
                kind,
                flags
            ],
        );
    } else {
        let (format, difference) = if position > end {
            (0x0101_54a0u32, position.wrapping_sub(end))
        } else if position < end {
            (0x0101_5440u32, end.wrapping_sub(position))
        } else {
            return;
        };
        let version = save_version(e);
        e.call(
            LOG_WARNING,
            &args![format, difference, QUEST_SOURCE_FILE, line, version],
        );
    }
}

/// The tag check of a load (line `line`): reads the 4-byte tag into `slot`
/// and warns when it is not `KOLB`.
fn read_block_tag(e: &mut Engine, slot: u32, line: u32) {
    let game = save_load_game(e);
    e.call(LOAD_READ, &args![game, slot, 4u32]);
    if e.mem.u32(slot) == BLOCK_TAG {
        return;
    }
    let game = save_load_game(e);
    let record = e.call(LOADING_RECORD, &args![game]).u32();
    if record != 0 {
        let name = record_form_name(e, record);
        let id = e.mem.u32(record);
        let kind = u32::from(e.mem.u8(record + 9));
        let flags = e.mem.u32(record + 5);
        e.call(
            LOG_WARNING,
            &args![
                0x0101_5718u32,
                QUEST_SOURCE_FILE,
                line,
                id,
                name,
                kind,
                flags
            ],
        );
    } else {
        let version = save_version(e);
        e.call(
            LOG_WARNING,
            &args![0x0101_56a8u32, QUEST_SOURCE_FILE, line, version],
        );
    }
}

/// The walk `0060d720` and `0060e6a0` make over the stages (from
/// `005d43c0`): each stage gets `0067fdb0` with 0, and each of its items
/// (the list `00717e50` gives) whose object (`008d8520`) is set gets that
/// word cleared (`00407800` with 0) and the object deleted (`007b3fa0`
/// with 1).
fn clear_stage_items(e: &mut Engine, this: Ptr) {
    let mut node = e.call(STAGE_LIST, &args![this]).u32();
    while node != 0 {
        let stage = node_item(e, node);
        if stage != 0 {
            e.call(STAGE_SET_BYTE_1, &args![stage, 0u32]);
            let mut entry_node = e.call(ITEM_LIST_00717E50, &args![stage]).u32();
            while entry_node != 0 {
                let entry = node_item(e, entry_node);
                let object = if entry != 0 {
                    e.call(ITEM_POINTER_008D8520, &args![entry]).u32()
                } else {
                    0
                };
                if object != 0 {
                    e.call(ITEM_SET_WORD, &args![entry, 0u32]);
                    e.call(DELETE_OBJECT, &args![object, 1u32]);
                }
                entry_node = e.call(NODE_NEXT, &args![entry_node]).u32();
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0060d720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the quest: clears every stage and its items' objects
/// (`clear_stage_items`), the byte at +0x60, gives each target of the
/// target list (`005d43e0`, up to the first empty node) `005ec5d0` with 0,
/// replaces the script locals at +0x5C (deleted with `0041af70`, new ones
/// from `005abf60` on the script at +0x1C through `00726070` on +0x18) when
/// there are some, then runs `00619b60` (cdecl), `0060c9c0` with the
/// flag of `0060d8d0`, and `0060ca30`, `TESQuest::SetFailed` and
/// `005ec700` with 0.
pub fn fn_0060d720(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    clear_stage_items(e, this);
    e.mem.set_u8(base + 0x60, 0);
    let mut node = e.call(FIRST_NODE_005D43E0, &args![this]).u32();
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let target = node_item(e, node);
        e.call(TARGET_RESET, &args![target, 0u32]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if e.mem.u32(base + SCRIPT_LOCALS) != 0 {
        let locals = e.mem.u32(base + SCRIPT_LOCALS);
        if locals != 0 {
            e.call(DELETE_LOCALS, &args![locals, 1u32]);
        }
        let script = e.call(NODE_NEXT, &args![base + 0x18]).u32();
        let locals = e.call(NEW_LOCALS, &args![script]).u32();
        e.mem.set_u32(base + SCRIPT_LOCALS, locals);
    }
    e.call(QUEST_RESET_LIST, &args![this]);
    let flag = u32::from(fn_0060d8d0(e, this));
    e.call(0x0060_c9c0, &args![this, flag]);
    e.call(0x0060_ca30, &args![this, 0u32]);
    e.call(0x0060_caf0, &args![this, 0u32]);
    e.call(0x005e_c700, &args![this, 0u32]);
}

// Translated from 0060d8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag 0x10 of the quest's flags byte (`005a8080`) is set.
pub fn fn_0060d8d0(e: &mut Engine, this: Ptr) -> bool {
    let flags = e.call(QUEST_FLAGS, &args![this]).u32();
    e.mem.u8(flags) & 0x10 != 0
}

// Translated from 0060d8f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0060d8f0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated
/// Collects quest objectives into `list` (cdecl: the list, a quest, three
/// bytes). Nothing for a null list. `00470470` runs on the list, then the
/// player's objectives (`0060da90` on the player at `011dea3c`, up to the
/// first empty node) are walked with a stack list of quests seen
/// (`0096a2d0`). For each objective with a quest (`005e3fa0`): with a
/// `quest` it must be that quest; without one, the objective's quest must
/// have `0059e400` equal to `match` (or `any` set) and not be in the seen
/// list (`005f65d0`), and is then added to it (`005ae3d0`). A matching
/// objective that has an object (`008d8520`) and a log entry
/// (`TESQuestStageItem::GetLogEntry` with its quest) goes into `list`,
/// at the front (`00905820`) with `front` set, else at the end
/// (`005ae3d0`). Then `00470470` and the destructor (`0046ffb0`) run on
/// the stack list.
pub fn fn_0060d8f0(e: &mut Engine, list: Ptr, quest: u32, matched: u8, front: u8, any: u8) {
    if list.is_null() {
        return;
    }
    e.call(ARGUMENT_CALL_00470470, &args![list]);
    e.with_stack(0x30, |e, frame| {
        // The exe's locals, at their offsets from `EBP`.
        let ebp = frame.addr() + 0x30;
        let seen = ebp - 0x14;
        let quest_slot = ebp - 0x1c;
        let objective_slot = ebp - 0x20;
        e.call(LIST_CONSTRUCT, &args![seen]);
        let player = e.global::<u32>(PLAYER);
        let mut node = fn_0060da90(e, Ptr::new(player)).addr();
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let objective = node_item(e, node);
            e.mem.set_u32(objective_slot, objective);
            let found = e.call(OBJECTIVE_QUEST, &args![objective]).u32();
            e.mem.set_u32(quest_slot, found);
            node = e.call(NODE_NEXT, &args![node]).u32();
            if found == 0 {
                continue;
            }
            let mut matches = quest == 0 || found == quest;
            if matches && quest == 0 {
                let state = e.call(SKIP_CHECK_0059E400, &args![found]).u8();
                matches = (matched == state || any != 0)
                    && e.call(ITEM_TEST_005F65D0, &args![seen, quest_slot]).u8() == 0;
                if matches {
                    e.call(LIST_ADD_TAIL, &args![seen, quest_slot]);
                }
            }
            if matches
                && e.call(ITEM_POINTER_008D8520, &args![objective]).u32() != 0
                && e.call(OBJECTIVE_LOG_ENTRY, &args![objective, found]).u32() != 0
            {
                if front != 0 {
                    e.call(LIST_ADD_HEAD, &args![list, objective_slot]);
                } else {
                    e.call(LIST_ADD_TAIL, &args![list, objective_slot]);
                }
            }
        }
        e.call(ARGUMENT_CALL_00470470, &args![seen]);
        e.call(LIST_DESTROY, &args![seen]);
    });
}

// Translated from 0060da90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0x6B0` (on the player: the list `0060d8f0` walks).
pub fn fn_0060da90(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x6b0)
}

// Translated from 0060db40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The quest's first stage (from `005d43c0`) whose index (`009373f0`) is
/// `index`, or 0 (`RET 4`).
pub fn fn_0060db40(e: &mut Engine, this: Ptr, index: u8) -> u32 {
    let mut node = e.call(STAGE_LIST, &args![this]).u32();
    while node != 0 {
        let stage = node_item(e, node);
        if stage != 0 && e.call(ITEM_FLAG_009373F0, &args![stage]).u8() == index {
            return stage;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    0
}

// Translated from 0060dba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The quest's save size for the change `flags` (`RET 4`; a `u16` sum):
/// the form's (`00484bf0`), 6 with save blocks, 1 with flag 2, with flag
/// 0x80000000 a count byte and per stage 2 + 1 + 5 per item (`005ae380` of
/// its item list), with flag 0x40000000 the script locals' (`005a94f0` on
/// +0x5C). With the report setting on, the size after the form's part is
/// reported (`Error`, line 0xF50).
pub fn fn_0060dba0(e: &mut Engine, this: Ptr, flags: u32) -> u16 {
    let mut size: u16 = 0;
    let form = e.call(0x0048_4bf0, &args![this, flags]).u16();
    size = size.wrapping_add(form);
    let start = size;
    if uses_save_blocks(e) {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    if flags & 2 != 0 {
        size = size.wrapping_add(1);
    }
    if flags & 0x8000_0000 != 0 {
        let mut node = e.call(STAGE_LIST, &args![this]).u32();
        size = size.wrapping_add(1);
        while node != 0 {
            let stage = node_item(e, node);
            if stage != 0 {
                size = size.wrapping_add(2);
                let items = e.call(ITEM_LIST_00717E50, &args![stage]).u32();
                let count = e.call(LIST_COUNT, &args![items]).u32();
                size = u32::from(size)
                    .wrapping_add(count.wrapping_mul(5))
                    .wrapping_add(1) as u16;
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
    if flags & 0x4000_0000 != 0 {
        let locals = e.mem.u32(this.addr() + SCRIPT_LOCALS);
        let part = e.call(0x005a_94f0, &args![locals]).u16();
        size = size.wrapping_add(part);
    }
    if reports_save_sizes(e) {
        let game = save_load_game(e);
        let record = e.call(SAVING_RECORD, &args![game]).u32();
        let reported = u32::from(size).wrapping_sub(u32::from(start));
        if record != 0 {
            let name = record_form_name(e, record);
            let id = e.mem.u32(record);
            let record_flags = e.mem.u32(record + 5);
            e.call(
                ERROR,
                &args![
                    0x0101_2cb0u32,
                    reported,
                    id,
                    name,
                    record_flags,
                    0xf50u32,
                    QUEST_SOURCE_FILE
                ],
            );
        } else {
            e.call(
                ERROR,
                &args![0x0101_2c78u32, reported, 0xf50u32, QUEST_SOURCE_FILE],
            );
        }
    }
    size
}

// Translated from 0060dd50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0060dd50 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated
/// The quest's save (`RET 4`: the change flags): the form's part
/// (`00484c20`); with save blocks the tag `KOLB` and a 16-bit size
/// placeholder (`008579b0`); with flag 2 the quest's flags byte
/// (`005a8080`); with flag 0x80000000 a count byte (patched in the save
/// data once known) and per stage its index (`009373f0`), its byte
/// (`005dc960`) and a count byte of its items (patched likewise), each
/// item as its byte (`005dc980`) and the 4 bytes of its object
/// (`008d8520`) or, without one, of a fresh stack value (`004b0fa0`,
/// destroyed with `00483710`); with flag 0x40000000 the script locals
/// (`005a9670` on +0x5C). Bytes go through `TESForm::SaveGameDataOLD`.
/// With the report setting on, the bytes written are reported (line
/// 0xFA3); with save blocks the block size is written into the
/// placeholder, with a warning when it is more than 0xFFFF.
pub fn fn_0060dd50(e: &mut Engine, this: Ptr, flags: u32) {
    let base = this.addr();
    e.call(0x0048_4c20, &args![this, flags]);
    e.with_stack(0x70, |e, frame| {
        // The exe's locals, at their offsets from `EBP`.
        let ebp = frame.addr() + 0x70;
        let size_slot = ebp - 0x14;
        let tag_slot = ebp - 0x1c;
        let flags_byte = ebp - 0x1d;
        let count_slot = ebp - 0x1e;
        let index_slot = ebp - 0x32;
        let byte_slot = ebp - 0x31;
        let item_count_slot = ebp - 0x39;
        let item_byte = ebp - 0x45;
        let value = ebp - 0x4c;
        e.mem.set_u16(size_slot, 0);
        let mut placeholder = 0u32;
        let mut start = save_position(e);
        if reports_save_sizes(e) {
            start = save_position(e);
        }
        if uses_save_blocks(e) {
            e.mem.set_u32(tag_slot, BLOCK_TAG);
            let game = save_load_game(e);
            e.call(SAVE_WRITE, &args![game, tag_slot, 4u32]);
            placeholder = save_position(e);
            let game = save_load_game(e);
            e.call(SAVE_WRITE, &args![game, size_slot, 2u32]);
        }
        if flags & 2 != 0 {
            let at = e.call(QUEST_FLAGS, &args![this]).u32();
            let quest_flags = e.mem.u8(at);
            e.mem.set_u8(flags_byte, quest_flags);
            e.call(SAVE_FORM_DATA, &args![this, flags_byte, 1u32]);
        }
        if flags & 0x8000_0000 != 0 {
            e.mem.set_u8(count_slot, 0);
            let count_at = save_position(e);
            e.call(SAVE_FORM_DATA, &args![this, count_slot, 1u32]);
            let mut node = e.call(STAGE_LIST, &args![this]).u32();
            while node != 0 {
                let stage = node_item(e, node);
                if stage != 0 {
                    let index = e.call(ITEM_FLAG_009373F0, &args![stage]).u8();
                    e.mem.set_u8(index_slot, index);
                    let byte = e.call(ITEM_FLAG_005DC960, &args![stage]).u8();
                    e.mem.set_u8(byte_slot, byte);
                    e.call(SAVE_FORM_DATA, &args![this, index_slot, 1u32]);
                    e.call(SAVE_FORM_DATA, &args![this, byte_slot, 1u32]);
                    e.mem.set_u8(item_count_slot, 0);
                    let item_count_at = save_position(e);
                    e.call(SAVE_FORM_DATA, &args![this, item_count_slot, 1u32]);
                    let mut entry_node = e.call(ITEM_LIST_00717E50, &args![stage]).u32();
                    while entry_node != 0 {
                        let entry = node_item(e, entry_node);
                        if entry != 0 {
                            let byte = e.call(ITEM_FLAG_005DC980, &args![entry]).u8();
                            e.mem.set_u8(item_byte, byte);
                            let object = e.call(ITEM_POINTER_008D8520, &args![entry]).u32();
                            e.call(SAVE_FORM_DATA, &args![this, item_byte, 1u32]);
                            if object != 0 {
                                let game = save_load_game(e);
                                e.call(SAVE_WRITE, &args![game, object, 4u32]);
                            } else {
                                e.call(VALUE_CONSTRUCT, &args![value]);
                                let game = save_load_game(e);
                                e.call(SAVE_WRITE, &args![game, value, 4u32]);
                                e.call(VALUE_DESTROY, &args![value]);
                            }
                            let count = e.mem.u8(item_count_slot).wrapping_add(1);
                            e.mem.set_u8(item_count_slot, count);
                        }
                        entry_node = e.call(NODE_NEXT, &args![entry_node]).u32();
                    }
                    let count = e.mem.u8(item_count_slot);
                    e.mem.set_u8(item_count_at, count);
                    let count = e.mem.u8(count_slot).wrapping_add(1);
                    e.mem.set_u8(count_slot, count);
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
            let count = e.mem.u8(count_slot);
            e.mem.set_u8(count_at, count);
        }
        if flags & 0x4000_0000 != 0 {
            let locals = e.mem.u32(base + SCRIPT_LOCALS);
            e.call(0x005a_9670, &args![locals]);
        }
        if reports_save_sizes(e) {
            let end = save_position(e);
            report_written(e, end.wrapping_sub(start), 0xfa3);
        }
        if uses_save_blocks(e) {
            let end = save_position(e);
            if end > placeholder.wrapping_add(0xffff) {
                e.call(
                    LOG_WARNING,
                    &args![0x0101_5318u32, QUEST_SOURCE_FILE, 0xfa3u32],
                );
            }
            e.mem
                .set_u16(placeholder, end.wrapping_sub(placeholder) as u16);
        }
    });
}

// Translated from 0060e0d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0060e0d0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated
/// The quest's load (`RET 8`: the change flags and a second word): the
/// form's part (`00484c50`); with save blocks the tag (warned about when
/// not `KOLB`, line 0xFAF) and the 16-bit block size; with flag 2 the
/// flags byte (`005a8080`); with flag 0x80000000 a count of stages, each an
/// index and a byte (given to the stage found by `0060db40` through
/// `0067fdb0`, or warned about with the quest's name, slot 0x138) and a
/// count of items, each a byte (its index) and a 4-byte value read into a
/// stack value (`004b0fa0`, `00483710`): for a found item (`0060f2c0`), a
/// value of two zero halves (`004b1030`, `004b1050`) clears the item's
/// object (deleted with `007b3fa0`, word cleared with `00407800`), any
/// other value is copied into the item's object (`004b1000`) or into a new
/// 4-byte one stored in the item; then the byte at +0x60 is cleared and
/// `0060d670` runs; with flag 0x40000000 the script locals at +0x5C load
/// (`005a9950`), or, without any, a temporary 0x14-byte one loads and is
/// deleted and `0060e680` gets 0x40000000 on the save/load game. With save
/// blocks the position is checked against the block's end (line 0x1010).
pub fn fn_0060e0d0(e: &mut Engine, this: Ptr, flags: u32, second: u32) {
    let base = this.addr();
    e.call(0x0048_4c50, &args![this, flags, second]);
    e.with_stack(0x90, |e, frame| {
        // The exe's locals, at their offsets from `EBP`.
        let ebp = frame.addr() + 0x90;
        let size_slot = ebp - 0x10;
        let tag_slot = ebp - 0x18;
        let flags_byte = ebp - 0x21;
        let count_slot = ebp - 0x22;
        let index_slot = ebp - 0x2a;
        let byte_slot = ebp - 0x29;
        let item_count_slot = ebp - 0x31;
        let value = ebp - 0x3c;
        let item_index = ebp - 0x3d;
        e.mem.set_u16(size_slot, 0);
        let mut block_start = 0u32;
        if uses_save_blocks(e) {
            read_block_tag(e, tag_slot, 0xfaf);
            block_start = save_position(e);
            let game = save_load_game(e);
            e.call(LOAD_READ, &args![game, size_slot, 2u32]);
        }
        if flags & 2 != 0 {
            e.call(LOAD_FORM_DATA, &args![this, flags_byte, 1u32]);
            let at = e.call(QUEST_FLAGS, &args![this]).u32();
            let byte = e.mem.u8(flags_byte);
            e.mem.set_u8(at, byte);
        }
        if flags & 0x8000_0000 != 0 {
            e.call(LOAD_FORM_DATA, &args![this, count_slot, 1u32]);
            let mut i = 0i32;
            while i < i32::from(e.mem.u8(count_slot)) {
                e.call(LOAD_FORM_DATA, &args![this, index_slot, 1u32]);
                e.call(LOAD_FORM_DATA, &args![this, byte_slot, 1u32]);
                let index = e.mem.u8(index_slot);
                let stage = fn_0060db40(e, this, index);
                if stage != 0 {
                    let byte = u32::from(e.mem.u8(byte_slot));
                    e.call(STAGE_SET_BYTE_1, &args![stage, byte]);
                } else {
                    let name = e.vcall(base, 0x138, &[]).u32();
                    let index = u32::from(e.mem.u8(index_slot));
                    e.call(LOG_WARNING, &args![FORMAT_NO_STAGE, index, name]);
                }
                e.call(LOAD_FORM_DATA, &args![this, item_count_slot, 1u32]);
                let mut j = 0i32;
                while j < i32::from(e.mem.u8(item_count_slot)) {
                    e.call(VALUE_CONSTRUCT, &args![value]);
                    e.call(LOAD_FORM_DATA, &args![this, item_index, 1u32]);
                    let game = save_load_game(e);
                    e.call(LOAD_READ, &args![game, value, 4u32]);
                    if stage != 0 {
                        let index = u32::from(e.mem.u8(item_index));
                        let item = e.call(STAGE_ITEM, &args![stage, index]).u32();
                        if item != 0 {
                            load_item_value(e, item, value);
                        }
                    }
                    e.call(VALUE_DESTROY, &args![value]);
                    j += 1;
                }
                i += 1;
            }
            e.mem.set_u8(base + 0x60, 0);
            e.call(QUEST_STAGES_LOADED, &args![this]);
        }
        if flags & 0x4000_0000 != 0 {
            let locals = e.mem.u32(base + SCRIPT_LOCALS);
            if locals == 0 {
                let block = e.call(ALLOCATE, &args![0x14u32]).u32();
                let temporary = if block != 0 {
                    e.call(LOCALS_CONSTRUCT, &args![block]).u32()
                } else {
                    0
                };
                e.call(0x005a_9950, &args![temporary]);
                if temporary != 0 {
                    e.call(DELETE_LOCALS, &args![temporary, 1u32]);
                }
                let game = save_load_game(e);
                fn_0060e680(e, Ptr::new(game), 0x4000_0000);
            } else {
                e.call(0x005a_9950, &args![locals]);
            }
        }
        if uses_save_blocks(e) {
            let end = u32::from(e.mem.u16(size_slot)).wrapping_add(block_start);
            check_block_end(e, end, 0x1010);
        }
    });
}

/// `0060e0d0`'s handling of a loaded item value: two zero halves clear the
/// item's object, anything else is copied into it or into a new one.
fn load_item_value(e: &mut Engine, item: u32, value: u32) {
    let cleared = e.call(VALUE_FIRST, &args![value]).u16() == 0
        && e.call(VALUE_SECOND, &args![value]).u16() == 0;
    if cleared {
        let object = e.call(ITEM_POINTER_008D8520, &args![item]).u32();
        if object != 0 {
            e.call(DELETE_OBJECT, &args![object, 1u32]);
            e.call(ITEM_SET_WORD, &args![item, 0u32]);
        }
        return;
    }
    let object = e.call(ITEM_POINTER_008D8520, &args![item]).u32();
    if object != 0 {
        e.call(VALUE_COPY, &args![object, value]);
        return;
    }
    let block = e.call(ALLOCATE, &args![4u32]).u32();
    let copy = if block != 0 {
        e.call(VALUE_COPY, &args![block, value]).u32()
    } else {
        0
    };
    e.call(ITEM_SET_WORD, &args![item, copy]);
}

// Translated from 0060e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bits` in the word at +0x54 (`RET 4`).
pub fn fn_0060e680(e: &mut Engine, this: Ptr, bits: u32) {
    let word = e.mem.u32(this.addr() + 0x54);
    e.mem.set_u32(this.addr() + 0x54, word | bits);
}

// Translated from 0060e6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reverts the quest for the change `flags` (`RET 4`): `004534f0` with
/// them; with flag 0x80000000 the stages and their items' objects are
/// cleared (`clear_stage_items`) and the byte at +0x60; with flag
/// 0x40000000 the script locals at +0x5C, when there are some, get
/// `005a9d00`.
pub fn fn_0060e6a0(e: &mut Engine, this: Ptr, flags: u32) {
    let base = this.addr();
    e.call(0x0045_34f0, &args![this, flags]);
    if flags & 0x8000_0000 != 0 {
        clear_stage_items(e, this);
        e.mem.set_u8(base + 0x60, 0);
    }
    if flags & 0x4000_0000 != 0 && e.mem.u32(base + SCRIPT_LOCALS) != 0 {
        let locals = e.mem.u32(base + SCRIPT_LOCALS);
        e.call(0x005a_9d00, &args![locals]);
    }
}

// Translated from 0060eaf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0060eaf0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated
/// Loads the quest from the load buffer (`RET 4`), as the buffer's change
/// flags select: the form's part (`00484da0`) always; flag 2: the byte at
/// +0x3C; flag 4: the word at +0x40; flag 0x80000000: a count
/// (`LoadVariableSizedValue`) of stages, each an index and a byte (given
/// to the stage `0060db40` finds through `0067fdb0`, or warned about with
/// `SaveGameWarning` and the quest's name, slot 0x138) and a count of
/// items, each an index byte, a byte saying whether a 4-byte value
/// follows, and that value (into a stack value, `004b0fa0`, `00483710`);
/// an item found (`0060f2c0`) for a value gets a new 4-byte copy
/// (`00407800`); then the byte at +0x60 is cleared and `0060d670` runs;
/// flag 0x40000000: the script locals at +0x5C load (`005a9f20`), or a
/// temporary stack one (`005a8b80`, `005a8bc0`) does; flag 0x20000000: a
/// count of word pairs, the second word given to `0050f9c0` of the object
/// `0060c8e0` finds for the first.
pub fn fn_0060eaf0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    e.call(0x0048_4da0, &args![this, buffer]);
    if change_flags_have(e, buffer, 2) {
        e.call(BUFFER_READ, &args![buffer, base + BYTE_3C, 1u32]);
    }
    if change_flags_have(e, buffer, 4) {
        e.call(BUFFER_READ, &args![buffer, base + WORD_40, 4u32]);
    }
    e.with_stack(0x80, |e, frame| {
        // The exe's locals, at their offsets from `EBP`.
        let ebp = frame.addr() + 0x80;
        let index_slot = ebp - 0x16;
        let byte_slot = ebp - 0x15;
        let item_index = ebp - 0x29;
        let has_value = ebp - 0x2a;
        let value = ebp - 0x28;
        let temporary = ebp - 0x44;
        let pair = ebp - 0x58;
        if change_flags_have(e, buffer, 0x8000_0000) {
            let count = e.call(BUFFER_READ_COUNT, &args![buffer]).u32();
            let mut i = 0u32;
            while i < count {
                e.mem.set_u8(index_slot, 0);
                e.mem.set_u8(byte_slot, 0);
                e.call(BUFFER_READ, &args![buffer, index_slot, 1u32]);
                e.call(BUFFER_READ, &args![buffer, byte_slot, 1u32]);
                let index = e.mem.u8(index_slot);
                let stage = fn_0060db40(e, this, index);
                if stage != 0 {
                    let byte = u32::from(e.mem.u8(byte_slot));
                    e.call(STAGE_SET_BYTE_1, &args![stage, byte]);
                } else {
                    let name = e.vcall(base, 0x138, &[]).u32();
                    let index = u32::from(e.mem.u8(index_slot));
                    e.call(SAVE_GAME_WARNING, &args![FORMAT_NO_STAGE, index, name]);
                }
                let items = e.call(BUFFER_READ_COUNT, &args![buffer]).u32();
                let mut j = 0u32;
                while j < items {
                    e.mem.set_u8(item_index, 0);
                    e.mem.set_u8(has_value, 0);
                    e.call(VALUE_CONSTRUCT, &args![value]);
                    e.call(BUFFER_READ, &args![buffer, item_index, 1u32]);
                    e.call(BUFFER_READ, &args![buffer, has_value, 1u32]);
                    if e.mem.u8(has_value) != 0 {
                        e.call(BUFFER_READ, &args![buffer, value, 4u32]);
                    }
                    if e.mem.u8(has_value) != 0 && stage != 0 {
                        let index = u32::from(e.mem.u8(item_index));
                        let item = e.call(STAGE_ITEM, &args![stage, index]).u32();
                        if item != 0 {
                            let block = e.call(ALLOCATE, &args![4u32]).u32();
                            let copy = if block != 0 {
                                e.call(VALUE_COPY, &args![block, value]).u32()
                            } else {
                                0
                            };
                            e.call(ITEM_SET_WORD, &args![item, copy]);
                        }
                    }
                    e.call(VALUE_DESTROY, &args![value]);
                    j += 1;
                }
                i += 1;
            }
            e.mem.set_u8(base + 0x60, 0);
            e.call(QUEST_STAGES_LOADED, &args![this]);
        }
        if change_flags_have(e, buffer, 0x4000_0000) {
            let locals = e.mem.u32(base + SCRIPT_LOCALS);
            if locals != 0 {
                e.call(0x005a_9f20, &args![locals, buffer]);
            } else {
                e.call(LOCALS_CONSTRUCT, &args![temporary]);
                e.call(0x005a_9f20, &args![temporary, buffer]);
                e.call(LOCALS_DESTROY, &args![temporary]);
            }
        }
        if change_flags_have(e, buffer, 0x2000_0000) {
            let count = e.call(BUFFER_READ_COUNT, &args![buffer]).u32();
            let mut i = 0u32;
            while i < count {
                e.mem.set_u32(pair, 0);
                e.mem.set_u32(pair + 4, 0);
                e.call(BUFFER_READ, &args![buffer, pair, 4u32]);
                e.call(BUFFER_READ, &args![buffer, pair + 4, 4u32]);
                let key = e.mem.u32(pair);
                let object = e.call(0x0060_c8e0, &args![this, key]).u32();
                if object != 0 {
                    let word = e.mem.u32(pair + 4);
                    e.call(0x0050_f9c0, &args![object, word]);
                }
                i += 1;
            }
        }
    });
}

/// `004280f0(0042ce30(buffer, &local), mask)`: `0060ee40`'s change flag
/// test.
fn change_flags_have_2(e: &mut Engine, buffer: Ptr, mask: u32) -> bool {
    e.with_stack(4, |e, local| {
        let flags = e.call(COPY_CHANGE_FLAGS_2, &args![buffer, local]).u32();
        e.call(FLAGS_TEST, &args![flags, mask]).u8() != 0
    })
}

// Translated from 0060ee40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reverts the quest for the buffer's change flags (`RET 4`): `004534f0`
/// with the buffer; flag 0x80000000 (`0042ce30`, `004280f0`): each stage of
/// the list at +0x44 gets `0067fdb0` with 0 and each of its items with an
/// object (`008d8520`) has it deleted (`007b3fa0`) and the word cleared
/// (`00407800`), then the byte at +0x60 is cleared; flag 0x40000000: the
/// script locals at +0x5C, when there are some, get `005aa090` with the
/// buffer; flag 0x20000000: each object of the list at +0x4C gets
/// `0050f9c0` with 0.
pub fn fn_0060ee40(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    e.call(0x0045_34f0, &args![this, buffer]);
    if change_flags_have_2(e, buffer, 0x8000_0000) {
        let mut node = base + LIST_44;
        while node != 0 {
            let stage = node_item(e, node);
            if stage != 0 {
                e.call(STAGE_SET_BYTE_1, &args![stage, 0u32]);
                let mut entry_node = e.call(ITEM_LIST_00717E50, &args![stage]).u32();
                while entry_node != 0 {
                    let entry = node_item(e, entry_node);
                    if entry != 0 && e.call(ITEM_POINTER_008D8520, &args![entry]).u32() != 0 {
                        let object = e.call(ITEM_POINTER_008D8520, &args![entry]).u32();
                        if object != 0 {
                            e.call(DELETE_OBJECT, &args![object, 1u32]);
                        }
                        e.call(ITEM_SET_WORD, &args![entry, 0u32]);
                    }
                    entry_node = e.call(NODE_NEXT, &args![entry_node]).u32();
                }
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        e.mem.set_u8(base + 0x60, 0);
    }
    if change_flags_have_2(e, buffer, 0x4000_0000) && e.mem.u32(base + SCRIPT_LOCALS) != 0 {
        let locals = e.mem.u32(base + SCRIPT_LOCALS);
        e.call(0x005a_a090, &args![locals, buffer]);
    }
    if change_flags_have_2(e, buffer, 0x2000_0000) {
        let mut node = base + LIST_4C;
        while node != 0 {
            let object = node_item(e, node);
            if object != 0 {
                e.call(0x0050_f9c0, &args![object, 0u32]);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
}

// Translated from 0060f1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The index (`009373f0`) of the stage (from `005d43c0`, up to the first
/// node `008256d0` calls empty) whose item for the item's own index
/// (`005dc980`, `0060f2c0`) is `item`; 0 when none (`RET 4`).
pub fn fn_0060f1a0(e: &mut Engine, this: Ptr, item: u32) -> u8 {
    let mut node = e.call(STAGE_LIST, &args![this]).u32();
    while node != 0 {
        if e.call(CURSOR_DONE_008256D0, &args![node]).u8() != 0 {
            break;
        }
        let stage = node_item(e, node);
        let index = u32::from(e.call(ITEM_FLAG_005DC980, &args![item]).u8());
        if e.call(STAGE_ITEM, &args![stage, index]).u32() == item {
            return e.call(ITEM_FLAG_009373F0, &args![stage]).u8();
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060dab0, fn_0060dab0(Ptr, Ptr) -> bool),
        entry!(0x0060_e7c0, fn_0060e7c0(Ptr, Ptr)),
        entry!(0x0060_e810, fn_0060e810(Ptr, Ptr)),
        entry!(0x0060_efd0, fn_0060efd0(Ptr, u32, u32) -> bool),
        entry!(0x0060_f110, fn_0060f110(Ptr, u32, u32)),
        entry!(0x0060_d720, fn_0060d720(Ptr)),
        entry!(0x0060_d8d0, fn_0060d8d0(Ptr) -> bool),
        entry!(0x0060_d8f0, fn_0060d8f0(Ptr, u32, u8, u8, u8)),
        entry!(0x0060_da90, fn_0060da90(Ptr) -> Ptr),
        entry!(0x0060_db40, fn_0060db40(Ptr, u8) -> u32),
        entry!(0x0060_dba0, fn_0060dba0(Ptr, u32) -> u16),
        entry!(0x0060_dd50, fn_0060dd50(Ptr, u32)),
        entry!(0x0060_e0d0, fn_0060e0d0(Ptr, u32, u32)),
        entry!(0x0060_e680, fn_0060e680(Ptr, u32)),
        entry!(0x0060_e6a0, fn_0060e6a0(Ptr, u32)),
        entry!(0x0060_eaf0, fn_0060eaf0(Ptr, Ptr)),
        entry!(0x0060_ee40, fn_0060ee40(Ptr, Ptr)),
        entry!(0x0060_f1a0, fn_0060f1a0(Ptr, u32) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_editor_id_unless_it_is_already_that() {
        let mut e = Engine::new();
        e.register(STRING_LENGTH, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(STRCPY_S, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.write(a[0], &s);
            e.mem.set_u8(a[0] + s.len() as u32, 0);
            Ret::default()
        });
        e.register_double(STRING_COMPARE, |_, _| 1u32.into_ret());
        e.register(STRING_ASSIGN, |_, _| Ret::default());
        let quest = e.mem.alloc(0x6c);
        let name = e.mem.alloc(8);
        e.mem.write(name, b"VMS49\0");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0060dab0, &[quest, name]).u32() & 0xff, 1);
        let log = e.call_log.take().unwrap();
        let assign = log.iter().find(|(a, _)| *a == STRING_ASSIGN).unwrap();
        assert_eq!(assign.1[0], quest + 0x64);
        assert_eq!(e.mem.cstr(assign.1[1]), b"VMS49");
        assert_eq!(assign.1[2], 0);
        // Equal already: nothing stored.
        e.register_double(STRING_COMPARE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0060dab0, &[quest, name]).u32() & 0xff, 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == STRING_ASSIGN));
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The test buffer: its change flags at +0x17, as `00428110` reads.
    fn buffer_with_flags(e: &mut Engine, flags: u32) -> u32 {
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(buffer + 0x17, flags);
        e.register(COPY_CHANGE_FLAGS, |e, a| {
            let flags = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], flags);
            a[1].into_ret()
        });
        e.register(FLAGS_TEST, |e, a| Ret {
            eax: 0xffff_ff00 | u32::from(e.mem.u32(a[0]) & a[1] != 0),
            ..Ret::default()
        });
        buffer
    }

    /// A list node (`{item, next}`); the doubles read it as the game's
    /// helpers do.
    fn node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    fn list_doubles(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
    }

    #[test]
    fn fn_0060e7c0_calls_out_only_with_the_flag_and_no_locals() {
        for (flags, locals, calls) in [
            (0x4000_0000u32, 0u32, true),
            (0xbfff_ffff, 0, false),
            (0x4000_0000, 0x1234, false),
        ] {
            let mut e = Engine::new();
            let buffer = buffer_with_flags(&mut e, flags);
            e.register(BUFFER_CALL_005621F0, |_, _| Ret::default());
            let this = e.mem.alloc(0x60);
            e.mem.set_u32(this + SCRIPT_LOCALS, locals);
            e.call_log = Some(vec![]);
            e.call(0x0060_e7c0, &args![this, buffer]);
            let seen = calls_to(&e, BUFFER_CALL_005621F0);
            if calls {
                assert_eq!(seen, vec![vec![buffer, 0x4000_0000]]);
            } else {
                assert!(seen.is_empty());
            }
            assert_eq!(calls_to(&e, FLAGS_TEST)[0][1], 0x4000_0000);
        }
    }

    /// Records saved bytes (`RECORD`: a length word, then the bytes) and
    /// the variable-sized counts (`COUNTS`: a length word, then count and
    /// start pairs).
    const RECORD: u32 = 0x0340_0000;
    const COUNTS: u32 = 0x0341_0000;

    fn record_saves(e: &mut Engine) {
        e.map(RECORD, 0x100);
        e.map(COUNTS, 0x100);
        e.register(SAVE_BYTES, |e, a| {
            assert_eq!(a[3], 0);
            let used = e.mem.u32(RECORD);
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(RECORD + 4 + used, &bytes);
            e.mem.set_u32(RECORD, used + a[2]);
            Ret::default()
        });
        e.register(START_VARIABLE_SIZED_VALUE, |e, _| {
            // Start positions 0x100, 0x101, ... in call order.
            let started = e.mem.u32(COUNTS + 0x80);
            e.mem.set_u32(COUNTS + 0x80, started + 1);
            (0x100 + started).into_ret()
        });
        e.register(FINISH_VARIABLE_SIZED_VALUE, |e, a| {
            let used = e.mem.u32(COUNTS);
            e.mem.set_u32(COUNTS + 4 + used * 8, a[1]);
            e.mem.set_u32(COUNTS + 8 + used * 8, a[2]);
            e.mem.set_u32(COUNTS, used + 1);
            Ret::default()
        });
    }

    fn recorded(e: &Engine) -> Vec<u8> {
        e.mem.bytes(RECORD + 4, e.mem.u32(RECORD))
    }

    fn counts(e: &Engine) -> Vec<(u32, u32)> {
        (0..e.mem.u32(COUNTS))
            .map(|i| (e.mem.u32(COUNTS + 4 + i * 8), e.mem.u32(COUNTS + 8 + i * 8)))
            .collect()
    }

    #[test]
    fn fn_0060e810_saves_only_the_form_part_without_flags() {
        let mut e = Engine::new();
        let buffer = buffer_with_flags(&mut e, 0x1fff_fff9);
        record_saves(&mut e);
        e.register(FORM_SAVE_00484D60, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x0060_e810, &args![this, buffer]);
        assert_eq!(calls_to(&e, FORM_SAVE_00484D60), vec![vec![this, buffer]]);
        assert!(recorded(&e).is_empty());
        assert!(counts(&e).is_empty());
    }

    #[test]
    fn fn_0060e810_saves_every_selected_part() {
        let mut e = Engine::new();
        let buffer = buffer_with_flags(&mut e, 0xe000_0006);
        record_saves(&mut e);
        list_doubles(&mut e);
        e.register(FORM_SAVE_00484D60, |_, _| Ret::default());
        e.register(SCRIPT_LOCALS_SAVE_GAME, |_, _| Ret::default());
        e.register(ITEM_FLAG_009373F0, |e, a| {
            (0xffff_ff00 | e.mem.u32(a[0])).into_ret()
        });
        e.register(ITEM_FLAG_005DC960, |e, a| {
            (0xffff_ff00 | e.mem.u32(a[0] + 4)).into_ret()
        });
        e.register(ITEM_LIST_00717E50, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ITEM_FLAG_005DC980, |e, a| {
            (0xffff_ff00 | e.mem.u32(a[0])).into_ret()
        });
        e.register(ITEM_POINTER_008D8520, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ITEM_VALUE_007AF430, |e, a| e.mem.u32(a[0]).into_ret());

        let this = e.mem.alloc(0x60);
        e.mem.set_u8(this + BYTE_3C, 0xab);
        e.mem.set_u32(this + WORD_40, 0x0403_0201);
        e.mem.set_u32(this + SCRIPT_LOCALS, 0x0777_0000);
        // List at +0x44: one item with two sub-entries (the second without
        // a pointer), then a null item.
        let pointed = e.mem.alloc(4);
        e.mem.set_u32(pointed, 0x0d0c_0b0a);
        let entry_a = e.mem.alloc(8);
        e.mem.set_u32(entry_a, 0x11);
        e.mem.set_u32(entry_a + 4, pointed);
        let entry_b = e.mem.alloc(8);
        e.mem.set_u32(entry_b, 0x22);
        let sub_second = node(&mut e, entry_b, 0);
        let sub_null = node(&mut e, 0, sub_second);
        let sub_first = node(&mut e, entry_a, sub_null);
        let item = e.mem.alloc(12);
        e.mem.set_u32(item, 0x05);
        e.mem.set_u32(item + 4, 0x06);
        e.mem.set_u32(item + 8, sub_first);
        let second = node(&mut e, 0, 0);
        e.mem.set_u32(this + LIST_44, item);
        e.mem.set_u32(this + LIST_44 + 4, second);
        // List at +0x4c: an item with a zero word (skipped), one with a
        // word; `00726070` on the item reads its +4.
        let skipped = e.mem.alloc(8);
        let kept = e.mem.alloc(8);
        e.mem.set_u32(kept, 0x0000_0033);
        e.mem.set_u32(kept + 4, 0x4444_4444);
        let kept_node = node(&mut e, kept, 0);
        e.mem.set_u32(this + LIST_4C, skipped);
        e.mem.set_u32(this + LIST_4C + 4, kept_node);

        e.call_log = Some(vec![]);
        e.call(0x0060_e810, &args![this, buffer]);
        let mut expected = vec![0xab, 1, 2, 3, 4];
        expected.extend([0x05, 0x06]);
        expected.extend([0x11, 1, 0x0a, 0x0b, 0x0c, 0x0d]);
        expected.extend([0x22, 0]);
        expected.extend([0x44, 0x44, 0x44, 0x44, 0x33, 0, 0, 0]);
        assert_eq!(recorded(&e), expected);
        // Inner count first (start 0x101), then the outer (0x100), then the
        // last list (0x102).
        assert_eq!(counts(&e), vec![(2, 0x101), (1, 0x100), (1, 0x102)]);
        assert_eq!(
            calls_to(&e, SCRIPT_LOCALS_SAVE_GAME),
            vec![vec![0x0777_0000, buffer]]
        );
    }

    fn efd0_setup(e: &mut Engine) {
        list_doubles(e);
        // Items: word at +0 (`007af430`), sub-list at +4 (`007d6bb0`).
        e.register(ITEM_VALUE_007AF430, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SUB_LIST_007D6BB0, |e, a| e.mem.u32(a[0] + 4).into_ret());
        // `005f65d0` accepts every item slot except when `other` is 0.
        e.register(ITEM_TEST_005F65D0, |_, a| u32::from(a[0] != 0).into_ret());
        // Entries: their reference is themselves, conditions hold when the
        // entry's word is odd.
        e.register(ITEM_REFERENCE_006101B0, |_, a| a[0].into_ret());
        e.register(ITEM_LIST_00717E50, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CONDITIONS_TRUE_00680C30, |_, a| (a[0] & 1).into_ret());
        e.register(CURSOR_DONE_008256D0, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) != 0).into_ret()
        });
        e.register(FIRST_NODE_005D43E0, |e, a| e.mem.u32(a[0]).into_ret());
    }

    #[test]
    fn fn_0060efd0_matches_holding_entries_against_the_cursor() {
        let mut e = Engine::new();
        efd0_setup(&mut e);
        let entry_on = e.mem.alloc(4);
        e.mem.set_u32(entry_on, 1);
        let entry_off = e.mem.alloc(4);
        e.mem.set_u32(entry_off, 2);
        let sub_b = node(&mut e, entry_on, 0);
        let sub_a = node(&mut e, entry_off, sub_b);
        let item = e.mem.alloc(8);
        e.mem.set_u32(item, 1);
        e.mem.set_u32(item + 4, sub_a);
        let first = node(&mut e, item, 0);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, first);
        // The cursor's item is the holding entry; its next is an end node
        // whose done flag (+8) decides.
        let end = e.mem.alloc(12);
        let cursor = e.mem.alloc(12);
        e.mem.set_u32(cursor, entry_on);
        e.mem.set_u32(cursor + 4, 0);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            1
        );
        // A cursor that does not match.
        e.mem.set_u32(cursor, entry_off);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            0
        );
        // A cursor left over that is not done fails, one that is passes.
        e.mem.set_u32(cursor, entry_on);
        e.mem.set_u32(cursor + 4, end);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            0
        );
        e.mem.set_u32(end + 8, 1);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            1
        );
        // Running out of cursor while an entry still holds fails.
        e.mem.set_u32(cursor + 4, 0);
        let sub_c = node(&mut e, entry_on, 0);
        e.mem.set_u32(sub_b + 4, sub_c);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            0
        );
        // Items the test refuses, or whose word is not 1, are skipped (the
        // untouched cursor then has to be done).
        e.mem.set_u32(cursor + 8, 1);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 0u32]).u32() & 0xff,
            1
        );
        e.mem.set_u32(item, 2);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            1
        );
        // No cursor: true without walking.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, 0u32, 1u32]).u32() & 0xff,
            1
        );
        assert!(calls_to(&e, NODE_ITEM).is_empty());
        assert!(calls_to(&e, CURSOR_DONE_008256D0).is_empty());
        // No first node: only the end check on the cursor.
        e.mem.set_u32(this, 0);
        e.mem.set_u32(cursor + 8, 0);
        assert_eq!(
            e.call(0x0060_efd0, &args![this, cursor, 1u32]).u32() & 0xff,
            0
        );
    }

    #[test]
    fn fn_0060f110_calls_out_for_accepted_items_until_a_null_item() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(FIRST_NODE_005D43E0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ARGUMENT_CALL_00470470, |_, _| Ret::default());
        e.register(SKIP_CHECK_0059E400, |e, a| e.mem.u32(a[0] + 4).into_ret());
        // Accepts item slots whose item is even.
        e.register(ITEM_TEST_005F65D0, |e, a| {
            assert_eq!(a[0], 0x0999_0000);
            u32::from(e.mem.u32(a[1]) % 2 == 0).into_ret()
        });
        e.register(ITEM_CALL_005EC500, |_, _| Ret::default());
        let after_null = node(&mut e, 0x40, 0);
        let null_node = node(&mut e, 0, after_null);
        let third = node(&mut e, 0x30, null_node);
        let second = node(&mut e, 0x21, third);
        let first = node(&mut e, 0x10, second);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, first);
        e.call_log = Some(vec![]);
        e.call(0x0060_f110, &args![this, 0x0555_0000u32, 0x0999_0000u32]);
        assert_eq!(
            calls_to(&e, ARGUMENT_CALL_00470470),
            vec![vec![0x0555_0000]]
        );
        assert_eq!(
            calls_to(&e, ITEM_CALL_005EC500),
            vec![vec![0x10, 0x0555_0000], vec![0x30, 0x0555_0000]]
        );
        // The skip check stops it after the first call.
        e.mem.set_u32(this + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0060_f110, &args![this, 0x0555_0000u32, 0x0999_0000u32]);
        assert_eq!(calls_to(&e, ARGUMENT_CALL_00470470).len(), 1);
        assert!(calls_to(&e, FIRST_NODE_005D43E0).is_empty());
        // A null target does nothing.
        e.call_log = Some(vec![]);
        e.call(0x0060_f110, &args![this, 0u32, 0x0999_0000u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn zero(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// A list of `items` (nodes `{item, next}`); an empty list is one node
    /// holding 0.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        if next == 0 {
            next = e.mem.alloc(8);
        }
        next
    }

    /// List doubles; a stage or entry is a block: +0 index byte (and
    /// `005dc980`'s byte), +1 `005dc960`'s byte, +4 its item list, +8 its
    /// object.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(CURSOR_DONE_008256D0, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(ITEM_FLAG_009373F0, |e, a| {
            u32::from(e.mem.u8(a[0])).into_ret()
        });
        e.register(ITEM_FLAG_005DC960, |e, a| {
            u32::from(e.mem.u8(a[0] + 1)).into_ret()
        });
        e.register(ITEM_FLAG_005DC980, |e, a| {
            u32::from(e.mem.u8(a[0])).into_ret()
        });
        e.register(ITEM_LIST_00717E50, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ITEM_POINTER_008D8520, |e, a| e.mem.u32(a[0] + 8).into_ret());
        // The quest's stage list is the word at +0x100 of the quest.
        e.register(STAGE_LIST, |e, a| e.mem.u32(a[0] + 0x100).into_ret());
        e.register(FIRST_NODE_005D43E0, |e, a| {
            e.mem.u32(a[0] + 0x104).into_ret()
        });
        e
    }

    fn block(e: &mut Engine, index: u8, second: u8, items: u32, object: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u8(block, index);
        e.mem.set_u8(block + 1, second);
        e.mem.set_u32(block + 4, items);
        e.mem.set_u32(block + 8, object);
        block
    }

    fn quest(e: &mut Engine) -> u32 {
        e.mem.alloc(0x110)
    }

    #[test]
    fn reset_clears_stages_targets_and_locals() {
        let mut e = engine();
        zero(
            &mut e,
            &[
                STAGE_SET_BYTE_1,
                ITEM_SET_WORD,
                DELETE_OBJECT,
                TARGET_RESET,
                DELETE_LOCALS,
                QUEST_RESET_LIST,
                0x0060_c9c0,
                0x0060_ca30,
                0x0060_caf0,
                0x005e_c700,
            ],
        );
        e.register(NEW_LOCALS, |_, a| (a[0] + 1).into_ret());
        let flags = e.mem.alloc(4);
        e.mem.set_u8(flags, 0x10);
        e.register_double(QUEST_FLAGS, move |_, _| flags.into_ret());
        let with = block(&mut e, 0, 0, 0, 0x7700);
        let without = block(&mut e, 0, 0, 0, 0);
        let entries = list(&mut e, &[with, 0, without]);
        let stage = block(&mut e, 1, 0, entries, 0);
        let stages = list(&mut e, &[stage, 0]);
        let targets = list(&mut e, &[0x8100, 0, 0x8200]);
        let this = quest(&mut e);
        e.mem.set_u32(this + 0x100, stages);
        e.mem.set_u32(this + 0x104, targets);
        e.mem.set_u8(this + 0x60, 9);
        e.mem.set_u32(this + 0x5c, 0x9900);
        let script_list = this + 0x18;
        e.mem.set_u32(script_list + 4, 0x4400);
        let (_, log) = run(&mut e, 0x0060_d720, &[this]);
        assert_eq!(calls(&log, STAGE_SET_BYTE_1), vec![vec![stage, 0]]);
        assert_eq!(calls(&log, ITEM_SET_WORD), vec![vec![with, 0]]);
        assert_eq!(calls(&log, DELETE_OBJECT), vec![vec![0x7700, 1]]);
        assert_eq!(calls(&log, TARGET_RESET), vec![vec![0x8100, 0]]);
        assert_eq!(calls(&log, DELETE_LOCALS), vec![vec![0x9900, 1]]);
        assert_eq!(calls(&log, NEW_LOCALS), vec![vec![0x4400]]);
        assert_eq!(e.mem.u32(this + 0x5c), 0x4401);
        assert_eq!(e.mem.u8(this + 0x60), 0);
        let rest: Log = log.into_iter().filter(|c| c.0 != QUEST_FLAGS).collect();
        let tail: Vec<_> = rest[rest.len() - 5..].to_vec();
        assert_eq!(
            tail,
            vec![
                (QUEST_RESET_LIST, vec![this]),
                (0x0060_c9c0, vec![this, 1]),
                (0x0060_ca30, vec![this, 0]),
                (0x0060_caf0, vec![this, 0]),
                (0x005e_c700, vec![this, 0]),
            ]
        );
        e.mem.set_u32(this + 0x5c, 0);
        e.mem.set_u8(flags, 0xef);
        let (_, log) = run(&mut e, 0x0060_d720, &[this]);
        assert!(calls(&log, NEW_LOCALS).is_empty());
        assert_eq!(calls(&log, 0x0060_c9c0), vec![vec![this, 0]]);
    }

    #[test]
    fn small_quest_helpers() {
        let mut e = engine();
        e.register(QUEST_FLAGS, |_, a| a[0].into_ret());
        let this = quest(&mut e);
        e.mem.set_u8(this, 0x10);
        assert!(e.call(0x0060_d8d0, &args![this]).bool());
        e.mem.set_u8(this, 0xef);
        assert!(!e.call(0x0060_d8d0, &args![this]).bool());
        assert_eq!(e.call(0x0060_da90, &args![0x1000u32]).u32(), 0x16b0);
        let a = block(&mut e, 3, 0, 0, 0);
        let b = block(&mut e, 7, 0, 0, 0);
        let stages = list(&mut e, &[a, 0, b]);
        e.mem.set_u32(this + 0x100, stages);
        assert_eq!(e.call(0x0060_db40, &args![this, 0x107u32]).u32(), b);
        assert_eq!(e.call(0x0060_db40, &args![this, 3u32]).u32(), a);
        assert_eq!(e.call(0x0060_db40, &args![this, 4u32]).u32(), 0);
        let target = e.mem.alloc(0x60);
        e.mem.set_u32(target + 0x54, 0x10);
        e.call(0x0060_e680, &args![target, 0x4000_0001u32]);
        assert_eq!(e.mem.u32(target + 0x54), 0x4000_0011);
    }

    #[test]
    fn objectives_are_collected_by_quest_and_state() {
        let mut e = engine();
        zero(
            &mut e,
            &[ARGUMENT_CALL_00470470, LIST_DESTROY, LIST_ADD_HEAD],
        );
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        // An objective: +0 its quest, +8 its object; a quest: +0 its state.
        e.register(OBJECTIVE_QUEST, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SKIP_CHECK_0059E400, |e, a| {
            u32::from(e.mem.u8(a[0])).into_ret()
        });
        e.register(ITEM_TEST_005F65D0, |e, a| {
            // Seen: the quest is listed at +0 of the seen list.
            let quest = e.mem.u32(a[1]);
            u32::from(e.mem.u32(a[0]) == quest).into_ret()
        });
        e.register(LIST_ADD_TAIL, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.register(OBJECTIVE_LOG_ENTRY, |_, _| 1u32.into_ret());
        let quest_a = e.mem.alloc(4);
        e.mem.set_u8(quest_a, 1);
        let quest_b = e.mem.alloc(4);
        e.mem.set_u8(quest_b, 0);
        let first = block(&mut e, 0, 0, 0, 0x5500);
        e.mem.set_u32(first, quest_a);
        let second = block(&mut e, 0, 0, 0, 0x5500);
        e.mem.set_u32(second, quest_a);
        let third = block(&mut e, 0, 0, 0, 0x5500);
        e.mem.set_u32(third, quest_b);
        let objectives = list(&mut e, &[first, second, third, 0]);
        let player = e.mem.alloc(0x6c0);
        e.mem.set_u32(player + 0x6b0, e.mem.u32(objectives));
        e.mem.set_u32(player + 0x6b4, e.mem.u32(objectives + 4));
        e.map(PLAYER, 4);
        e.mem.set_u32(PLAYER, player);
        let out = e.mem.alloc(8);
        let (_, log) = run(&mut e, 0x0060_d8f0, &[out, 0, 1, 0, 0]);
        // The first objective of quest A goes in, the second is a quest
        // already seen, the third's quest has another state.
        let added: Vec<Vec<u32>> = calls(&log, LIST_ADD_TAIL)
            .into_iter()
            .filter(|c| c[0] == out)
            .collect();
        assert_eq!(added.len(), 1);
        assert_eq!(e.mem.u32(out), first);
        assert_eq!(calls(&log, ARGUMENT_CALL_00470470).len(), 2);
        assert_eq!(calls(&log, LIST_DESTROY).len(), 1);
        // With `any` and the front flag, quest B's objective is taken too.
        let out = e.mem.alloc(8);
        let (_, log) = run(&mut e, 0x0060_d8f0, &[out, 0, 1, 1, 1]);
        assert_eq!(calls(&log, LIST_ADD_HEAD).len(), 2);
        // With a quest filter only its objectives count (no seen list).
        let (_, log) = run(&mut e, 0x0060_d8f0, &[out, quest_b, 1, 0, 0]);
        assert!(calls(&log, SKIP_CHECK_0059E400).is_empty());
        assert_eq!(calls(&log, OBJECTIVE_LOG_ENTRY), vec![vec![third, quest_b]]);
        let (_, log) = run(&mut e, 0x0060_d8f0, &[0, 0, 1, 0, 0]);
        assert!(log.is_empty());
    }

    fn save_engine(blocks: u32, report: u8) -> Engine {
        let mut e = engine();
        e.map(SAVE_LOAD_GAME_OBJECT, 4);
        e.mem.set_u32(SAVE_LOAD_GAME_OBJECT, 0x7700);
        e.register_double(USE_SAVE_GAME_BLOCKS, move |_, _| blocks.into_ret());
        e.register(SAVE_VERSION, |_, _| 0x30u32.into_ret());
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting, report);
        e.register_double(SETTING_VALUE, move |_, _| setting.into_ret());
        e.register(SAVING_RECORD, |_, _| Ret::default());
        e.register(LOADING_RECORD, |_, _| Ret::default());
        zero(&mut e, &[ERROR, LOG_WARNING, SAVE_GAME_WARNING]);
        e
    }

    #[test]
    fn save_size_counts_stages_items_and_locals() {
        let mut e = save_engine(1, 1);
        e.register(0x0048_4bf0, |_, _| 0x20u32.into_ret());
        e.register(LIST_COUNT, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(0x005a_94f0, |_, _| 0x9u32.into_ret());
        let items = e.mem.alloc(0x14);
        e.mem.set_u32(items + 0x10, 3);
        let stage = block(&mut e, 0, 0, items, 0);
        let stages = list(&mut e, &[stage, 0, stage]);
        let this = quest(&mut e);
        e.mem.set_u32(this + 0x100, stages);
        let (ret, log) = run(&mut e, 0x0060_dba0, &[this, 0xc000_0002]);
        let expected = 0x20 + 6 + 1 + 1 + 2 * (2 + 15 + 1) + 9;
        assert_eq!(ret.u32() & 0xffff, expected);
        assert_eq!(
            calls(&log, ERROR),
            vec![vec![0x0101_2c78, expected - 0x20, 0xf50, QUEST_SOURCE_FILE]]
        );
        let mut e = save_engine(0, 0);
        e.register(0x0048_4bf0, |_, _| 0x20u32.into_ret());
        let (ret, _) = run(&mut e, 0x0060_dba0, &[this, 0]);
        assert_eq!(ret.u32() & 0xffff, 0x20);
    }

    #[test]
    fn save_game_writes_stages_and_patches_the_counts() {
        let mut e = save_engine(1, 0);
        zero(&mut e, &[0x0048_4c20, VALUE_DESTROY, 0x005a_9670]);
        e.register(VALUE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        // The save data: bytes written go to a buffer, the position is its
        // end.
        let data = e.mem.alloc(0x100);
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, data);
        e.register_double(SAVE_POSITION, move |e, _| e.mem.u32(position).into_ret());
        let write = move |e: &mut Engine, from: u32, size: u32| {
            let at = e.mem.u32(position);
            for i in 0..size {
                let byte = e.mem.u8(from + i);
                e.mem.set_u8(at + i, byte);
            }
            e.mem.set_u32(position, at + size);
        };
        e.register_double(SAVE_WRITE, move |e, a| {
            write(e, a[1], a[2]);
            Ret::default()
        });
        e.register_double(SAVE_FORM_DATA, move |e, a| {
            write(e, a[1], a[2]);
            Ret::default()
        });
        let flags = e.mem.alloc(4);
        e.mem.set_u8(flags, 0x5a);
        e.register_double(QUEST_FLAGS, move |_, _| flags.into_ret());
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x4433_2211);
        let with = block(&mut e, 0x21, 0, 0, object);
        let without = block(&mut e, 0x22, 0, 0, 0);
        let entries = list(&mut e, &[with, 0, without]);
        let stage = block(&mut e, 10, 1, entries, 0);
        let stages = list(&mut e, &[stage, 0]);
        let this = quest(&mut e);
        e.mem.set_u32(this + 0x100, stages);
        let (_, log) = run(&mut e, 0x0060_dd50, &[this, 0xc000_0002]);
        assert_eq!(calls(&log, 0x0048_4c20), vec![vec![this, 0xc000_0002]]);
        let bytes: Vec<u8> = (0..0x15).map(|i| e.mem.u8(data + i)).collect();
        assert_eq!(
            bytes,
            vec![
                0x4b, 0x4f, 0x4c, 0x42, // KOLB
                0x11, 0x00, // the block size
                0x5a, // the flags byte
                1,    // one stage
                10, 1, 2, // its index, byte, two items
                0x21, 0x11, 0x22, 0x33, 0x44, // an item and its object
                0x22, 0, 0, 0, 0, // an item and the fresh value
            ]
        );
        assert_eq!(calls(&log, VALUE_CONSTRUCT).len(), 1);
        assert_eq!(calls(&log, 0x005a_9670), vec![vec![0]]);
    }

    #[test]
    fn load_game_reads_stages_and_items() {
        let mut e = save_engine(1, 0);
        zero(
            &mut e,
            &[
                0x0048_4c50,
                VALUE_CONSTRUCT,
                VALUE_DESTROY,
                STAGE_SET_BYTE_1,
                ITEM_SET_WORD,
                DELETE_OBJECT,
                VALUE_COPY,
                QUEST_STAGES_LOADED,
                0x005a_9950,
            ],
        );
        e.register(0x0f00_1000, |_, _| 0x6600u32.into_ret());
        let mut slots = vec![0u32; 0x13c / 4];
        slots[0x138 / 4] = 0x0f00_1000;
        e.put_vtable(0x0f00_0000, &slots);
        // The save data, read in order.
        let data = e.mem.alloc(0x40);
        let bytes: [u8; 25] = [
            0x4b, 0x4f, 0x4c, 0x42, 0x14, 0x00, // KOLB, the block size
            0x5a, // flags
            2,    // two stages
            10, 1, 2, // stage 10, byte 1, two items
            0, 0, 0, 0, 0, // item 0, value 0
            1, 5, 0, 0, 0, // item 1, value 5
            11, 3, 0, 0, // stage 11 (missing), byte 3, no items
        ];
        e.mem.write(data, &bytes);
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, data);
        e.register_double(SAVE_POSITION, move |e, _| e.mem.u32(position).into_ret());
        let read = move |e: &mut Engine, to: u32, size: u32| {
            let at = e.mem.u32(position);
            for i in 0..size {
                let byte = e.mem.u8(at + i);
                e.mem.set_u8(to + i, byte);
            }
            e.mem.set_u32(position, at + size);
        };
        e.register_double(LOAD_READ, move |e, a| {
            read(e, a[1], a[2]);
            Ret::default()
        });
        e.register_double(LOAD_FORM_DATA, move |e, a| {
            read(e, a[1], a[2]);
            Ret::default()
        });
        let flags = e.mem.alloc(4);
        e.register_double(QUEST_FLAGS, move |_, _| flags.into_ret());
        e.register(VALUE_FIRST, |e, a| (e.mem.u32(a[0]) & 0xffff).into_ret());
        e.register(VALUE_SECOND, |e, a| (e.mem.u32(a[0]) >> 16).into_ret());
        let item0 = block(&mut e, 0, 0, 0, 0x7100);
        let item1 = block(&mut e, 1, 0, 0, 0);
        e.register_double(STAGE_ITEM, move |_, a| {
            if a[1] == 0 { item0 } else { item1 }.into_ret()
        });
        e.register(ALLOCATE, |_, _| 0x9100u32.into_ret());
        e.register(VALUE_COPY, |_, a| a[0].into_ret());
        let stage = block(&mut e, 10, 0, 0, 0);
        let stages = list(&mut e, &[stage]);
        let this = quest(&mut e);
        e.mem.set_u32(this, 0x0f00_0000);
        e.mem.set_u32(this + 0x100, stages);
        e.mem.set_u8(this + 0x60, 1);
        e.mem.set_u32(this + 0x5c, 0x8800);
        let (_, log) = run(&mut e, 0x0060_e0d0, &[this, 0xc000_0002, 7]);
        assert_eq!(calls(&log, 0x0048_4c50), vec![vec![this, 0xc000_0002, 7]]);
        assert_eq!(e.mem.u8(flags), 0x5a);
        assert_eq!(calls(&log, STAGE_SET_BYTE_1), vec![vec![stage, 1]]);
        assert_eq!(calls(&log, DELETE_OBJECT), vec![vec![0x7100, 1]]);
        let value = calls(&log, VALUE_CONSTRUCT)[0][0];
        assert_eq!(calls(&log, VALUE_COPY), vec![vec![0x9100, value]]);
        assert_eq!(
            calls(&log, ITEM_SET_WORD),
            vec![vec![item0, 0], vec![item1, 0x9100]]
        );
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![vec![FORMAT_NO_STAGE, 11, 0x6600]]
        );
        assert_eq!(e.mem.u8(this + 0x60), 0);
        assert_eq!(calls(&log, QUEST_STAGES_LOADED), vec![vec![this]]);
        assert_eq!(calls(&log, 0x005a_9950), vec![vec![0x8800]]);
    }

    #[test]
    fn load_game_without_locals_skips_them_through_a_temporary() {
        let mut e = save_engine(1, 0);
        zero(&mut e, &[0x0048_4c50, 0x005a_9950, DELETE_LOCALS]);
        e.register(LOAD_READ, |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], 0x1234);
            } else {
                e.mem.set_u16(a[1], 0x10);
            }
            Ret::default()
        });
        e.register(SAVE_POSITION, |_, _| 0x1000u32.into_ret());
        e.register(ALLOCATE, |_, _| 0x9100u32.into_ret());
        e.register(LOCALS_CONSTRUCT, |_, a| a[0].into_ret());
        let game = e.mem.alloc(0x60);
        e.mem.set_u32(SAVE_LOAD_GAME_OBJECT, game);
        let this = quest(&mut e);
        let (_, log) = run(&mut e, 0x0060_e0d0, &[this, 0x4000_0000, 0]);
        assert_eq!(calls(&log, 0x005a_9950), vec![vec![0x9100]]);
        assert_eq!(calls(&log, DELETE_LOCALS), vec![vec![0x9100, 1]]);
        assert_eq!(e.mem.u32(game + 0x54), 0x4000_0000);
        // The bad tag and the short block are both reported.
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![
                vec![0x0101_56a8, QUEST_SOURCE_FILE, 0xfaf, 0x30],
                vec![0x0101_5440, 0x10, QUEST_SOURCE_FILE, 0x1010, 0x30]
            ]
        );
    }

    #[test]
    fn revert_clears_stages_and_reverts_locals() {
        let mut e = engine();
        zero(
            &mut e,
            &[
                0x0045_34f0,
                STAGE_SET_BYTE_1,
                ITEM_SET_WORD,
                DELETE_OBJECT,
                0x005a_9d00,
            ],
        );
        let with = block(&mut e, 0, 0, 0, 0x7700);
        let entries = list(&mut e, &[with]);
        let stage = block(&mut e, 1, 0, entries, 0);
        let stages = list(&mut e, &[stage]);
        let this = quest(&mut e);
        e.mem.set_u32(this + 0x100, stages);
        e.mem.set_u8(this + 0x60, 1);
        e.mem.set_u32(this + 0x5c, 0x8800);
        let (_, log) = run(&mut e, 0x0060_e6a0, &[this, 0xc000_0000]);
        assert_eq!(log[0], (0x0045_34f0, vec![this, 0xc000_0000]));
        assert_eq!(calls(&log, ITEM_SET_WORD), vec![vec![with, 0]]);
        assert_eq!(calls(&log, DELETE_OBJECT), vec![vec![0x7700, 1]]);
        assert_eq!(calls(&log, 0x005a_9d00), vec![vec![0x8800]]);
        assert_eq!(e.mem.u8(this + 0x60), 0);
        e.mem.set_u8(this + 0x60, 1);
        let (_, log) = run(&mut e, 0x0060_e6a0, &[this, 0x3fff_ffff]);
        assert_eq!(log.len(), 1);
        assert_eq!(e.mem.u8(this + 0x60), 1);
    }

    /// A buffer whose change flags are at +0; the two flag copiers read
    /// them.
    fn buffer(e: &mut Engine, flags: u32) -> u32 {
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer, flags);
        for copier in [COPY_CHANGE_FLAGS, COPY_CHANGE_FLAGS_2] {
            e.register(copier, |e, a| {
                let flags = e.mem.u32(a[0]);
                e.mem.set_u32(a[1], flags);
                a[1].into_ret()
            });
        }
        e.register(FLAGS_TEST, |e, a| {
            u32::from(e.mem.u32(a[0]) & a[1] != 0).into_ret()
        });
        buffer
    }

    #[test]
    fn buffer_load_reads_each_flagged_part() {
        let mut e = engine();
        zero(
            &mut e,
            &[
                0x0048_4da0,
                VALUE_CONSTRUCT,
                VALUE_DESTROY,
                STAGE_SET_BYTE_1,
                ITEM_SET_WORD,
                QUEST_STAGES_LOADED,
                LOCALS_CONSTRUCT,
                LOCALS_DESTROY,
                0x005a_9f20,
                0x0050_f9c0,
                SAVE_GAME_WARNING,
            ],
        );
        e.register(0x0f00_1000, |_, _| 0x6600u32.into_ret());
        let mut slots = vec![0u32; 0x13c / 4];
        slots[0x138 / 4] = 0x0f00_1000;
        e.put_vtable(0x0f00_0000, &slots);
        // The buffer's data: counts and bytes in reading order.
        let data = e.mem.alloc(0x40);
        let bytes: [u8; 23] = [
            0xaa, // +0x3c
            1, 2, 3, 4, // +0x40
            7, 1, // stage 7, byte 1
            2, 1, 9, 9, 9, 9, // item 2, a value
            9, 2, // stage 9 (missing), byte 2
            1, 2, 3, 4, 0, 5, 6, 7, // a pair
        ];
        e.mem.write(data, &bytes);
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, data);
        e.register_double(BUFFER_READ, move |e, a| {
            let at = e.mem.u32(position);
            for i in 0..a[2] {
                let byte = e.mem.u8(at + i);
                e.mem.set_u8(a[1] + i, byte);
            }
            e.mem.set_u32(position, at + a[2]);
            Ret::default()
        });
        // Counts: 2 stages, 1 item for the first, 0 for the second, 1 pair.
        let counts = e.mem.alloc(0x10);
        e.mem.write(counts, &[2, 1, 0, 0]);
        let next_count = e.mem.alloc(4);
        e.register_double(BUFFER_READ_COUNT, move |e, _| {
            let n = e.mem.u32(next_count);
            e.mem.set_u32(next_count, n + 1);
            let count = u32::from(e.mem.u8(counts + n));
            if n == 3 { 1 } else { count }.into_ret()
        });
        e.register(STAGE_ITEM, |_, a| (0x5500 + a[1]).into_ret());
        e.register(ALLOCATE, |_, _| 0x9100u32.into_ret());
        e.register(VALUE_COPY, |_, a| a[0].into_ret());
        e.register(0x0060_c8e0, |_, a| a[1].into_ret());
        let stage = block(&mut e, 7, 0, 0, 0);
        let stages = list(&mut e, &[stage]);
        let this = quest(&mut e);
        e.mem.set_u32(this, 0x0f00_0000);
        e.mem.set_u32(this + 0x100, stages);
        let buffer = buffer(&mut e, 0xe000_0006);
        let (_, log) = run(&mut e, 0x0060_eaf0, &[this, buffer]);
        assert_eq!(e.mem.u8(this + 0x3c), 0xaa);
        assert_eq!(e.mem.u32(this + 0x40), 0x0403_0201);
        assert_eq!(calls(&log, STAGE_SET_BYTE_1), vec![vec![stage, 1]]);
        assert_eq!(calls(&log, ITEM_SET_WORD), vec![vec![0x5502, 0x9100]]);
        assert_eq!(
            calls(&log, SAVE_GAME_WARNING),
            vec![vec![FORMAT_NO_STAGE, 9, 0x6600]]
        );
        let temporary = calls(&log, LOCALS_CONSTRUCT)[0][0];
        assert_eq!(calls(&log, 0x005a_9f20), vec![vec![temporary, buffer]]);
        assert_eq!(calls(&log, LOCALS_DESTROY), vec![vec![temporary]]);
        assert_eq!(
            calls(&log, 0x0050_f9c0),
            vec![vec![0x0403_0201, 0x0706_0500]]
        );
    }

    #[test]
    fn buffer_revert_clears_stage_items_and_objects() {
        let mut e = engine();
        zero(
            &mut e,
            &[
                0x0045_34f0,
                STAGE_SET_BYTE_1,
                ITEM_SET_WORD,
                DELETE_OBJECT,
                0x005a_a090,
                0x0050_f9c0,
            ],
        );
        let with = block(&mut e, 0, 0, 0, 0x7700);
        let without = block(&mut e, 0, 0, 0, 0);
        let entries = list(&mut e, &[without, with]);
        let stage = block(&mut e, 1, 0, entries, 0);
        let this = quest(&mut e);
        // The lists embedded at +0x44 and +0x4c.
        e.mem.set_u32(this + 0x44, stage);
        e.mem.set_u32(this + 0x4c, 0x6100);
        let tail = list(&mut e, &[0, 0x6200]);
        e.mem.set_u32(this + 0x50, tail);
        e.mem.set_u32(this + 0x5c, 0x8800);
        e.mem.set_u8(this + 0x60, 1);
        let buffer = buffer(&mut e, 0xe000_0000);
        let (_, log) = run(&mut e, 0x0060_ee40, &[this, buffer]);
        assert_eq!(calls(&log, STAGE_SET_BYTE_1), vec![vec![stage, 0]]);
        assert_eq!(calls(&log, DELETE_OBJECT), vec![vec![0x7700, 1]]);
        assert_eq!(calls(&log, ITEM_SET_WORD), vec![vec![with, 0]]);
        assert_eq!(e.mem.u8(this + 0x60), 0);
        assert_eq!(calls(&log, 0x005a_a090), vec![vec![0x8800, buffer]]);
        assert_eq!(
            calls(&log, 0x0050_f9c0),
            vec![vec![0x6100, 0], vec![0x6200, 0]]
        );
    }

    #[test]
    fn stage_index_of_an_item() {
        let mut e = engine();
        // Each stage's items: the stage's +4 word is the one item, any index.
        e.register(STAGE_ITEM, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let a = block(&mut e, 3, 0, 0x5500, 0);
        let b = block(&mut e, 8, 0, 0x5600, 0);
        let stages = list(&mut e, &[a, b, 0]);
        let this = quest(&mut e);
        e.mem.set_u32(this + 0x100, stages);
        e.map(0x5500, 0x400);
        assert_eq!(e.call(0x0060_f1a0, &args![this, 0x5600u32]).u32() & 0xff, 8);
        assert_eq!(e.call(0x0060_f1a0, &args![this, 0x5500u32]).u32() & 0xff, 3);
        assert_eq!(e.call(0x0060_f1a0, &args![this, 0x5700u32]).u32() & 0xff, 0);
    }
}

//! Program facts: addresses and layouts of FalloutNV.exe 1.4.0.525.
//!
//! Each constant names its source:
//! - `legacy`: the project's own plugin for the original program (branch `legacy-xnvse`, MIT),
//!   its `docs/engine-addresses.md` unless a file is named;
//! - `ghidra`: our analysis of the program, by function address.

/// `LookupFormByID`: cdecl (form id), the form or 0.
// legacy: section 2 (prologue verified).
pub const FORM_BY_ID: u32 = 0x0048_39c0;

/// `TESQuest::SetStage`: method (quest, stage), true when the quest has the stage. It marks
/// the stage done, starts the quest, raises the current stage and runs the result scripts.
// legacy: section 2. ghidra: 0060d510, 0060f4a0.
pub const QUEST_SET_STAGE: u32 = 0x0060_d510;

/// A quest's flags, one byte: [`quest_flag`].
// ghidra: 0060ca30 (`TESQuest::SetCompleted` tests and sets bit 1 of the byte at +0x3c),
// 0060c9c0 (`TESQuest::SetRunning` sets and clears bit 0 of the same byte).
pub const QUEST_FLAGS: u32 = 0x3c;

/// The bits of a quest's flags byte.
pub mod quest_flag {
    /// The quest runs.
    // ghidra: 0060c9c0.
    pub const RUNNING: u8 = 1;
    /// The quest is completed.
    // ghidra: 0060ca30.
    pub const COMPLETED: u8 = 2;
    /// The quest failed. The program sets [`COMPLETED`] with it.
    // ghidra: 0060caf0 (`TESQuest::SetFailed` sets 0x40 and 2 in the byte at +0x3c).
    // legacy: section 6 (the address of `TESQuest::SetFailed`).
    pub const FAILED: u8 = 0x40;
}

/// A quest's stage list: the head node. A node is {item pointer, next node pointer}. An empty
/// list has both zero.
// ghidra: 005d43c0 (the list is at +0x44), 008256d0 (the empty test), 00726070 (next at +4),
// 0060d510 (`SetStage` walks it).
pub const QUEST_STAGES: u32 = 0x44;

/// A stage item: the stage number, one byte.
// ghidra: 009373f0 (the byte at +0), 0060d510 (compared with the stage asked for).
pub const STAGE_NUMBER: u32 = 0;
/// A stage item: done, one byte (1 when the stage ran).
// ghidra: 0060f4a0 (`QuestStageInfo::Run` writes 1 at +1). legacy: section 2.
pub const STAGE_DONE: u32 = 1;

/// A quest's objective list: the head node, as [`QUEST_STAGES`].
// ghidra: 005d43e0 (the list is at +0x4c), 0060c8e0 (`TESQuest::GetObjective` walks it).
pub const QUEST_OBJECTIVES: u32 = 0x4c;

/// An objective's id, a 32-bit word.
// ghidra: 0060c8e0 (compared with the id asked for, read at +4). legacy: section 3.
pub const OBJECTIVE_ID: u32 = 4;
/// An objective's status, a 32-bit word: bit 0 displayed, bit 1 completed.
// legacy: section 3 (`BGSQuestObjective`, with its setter 0x5EC5D0).
pub const OBJECTIVE_STATUS: u32 = 0x20;

/// A quest's current stage, one byte. `SetStage` only raises it.
// legacy: section 2 (the note of `TESQuest::SetStage`). ghidra: 0060d510 (byte at +0x60).
pub const QUEST_STAGE: u32 = 0x60;

/// A global's value (`TESGlobal`), one float.
// legacy: section 3 (the getter 0x526AC0 reads the float at +0x24).
pub const GLOBAL_VALUE: u32 = 0x24;

/// A list's node: the item pointer.
pub const NODE_ITEM: u32 = 0;
/// A list's node: the next node pointer.
// ghidra: 00726070.
pub const NODE_NEXT: u32 = 4;

/// The player (`PlayerCharacter**`, reference 0x14).
// legacy: plugin/avatar/engine/engine_pins.h (`kThePlayerPtr`).
pub const PLAYER: u32 = 0x011d_ea3c;
/// Where a reference keeps its position (`TESObjectREFR` posX/Y/Z, three floats).
// legacy: section 3.
pub const REF_POSITION: u32 = 0x30;

/// The console manager: cdecl (create), the manager or 0. Its first word is the context that
/// the console's scripts compile in.
// ghidra: 0071b160.
pub const CONSOLE_MANAGER: u32 = 0x0071_b160;
// How the program runs a line typed in its console (ghidra: 0071b210, the console's key
// handler, at its two calls of `Script::Run`): it makes a `Script` on the stack, marks the
// form temporary, sets its text, runs it in the manager's context, and destroys it.
/// `Script`'s constructor: method (script).
// ghidra: 005aa0f0 (sets `Script::vftable`, then two members at +0x44 and +0x4c).
pub const SCRIPT_NEW: u32 = 0x005a_a0f0;
/// Bytes for a `Script`: the constructor's last member is 8 bytes at +0x4c.
pub const SCRIPT_BYTES: u32 = 0x54;
/// Marks a form temporary (flag 0x4000 at +8): method (form).
// ghidra: 00484490.
pub const FORM_SET_TEMPORARY: u32 = 0x0048_4490;
/// Sets a script's text (a copy, kept at +0x2c): method (script, text).
// ghidra: 005abe50.
pub const SCRIPT_SET_TEXT: u32 = 0x005a_be50;
/// Compiles the script's text and executes it: method (script, context, 1, calling reference).
// legacy: section 2 (`Script::Run`). ghidra: 005ac400.
pub const SCRIPT_RUN: u32 = 0x005a_c400;
/// `Script`'s destructor (it frees the text and the compiled data, not the object): method
/// (script).
// ghidra: 005aa1a0 (the console's key handler calls it on its stack object).
pub const SCRIPT_DROP: u32 = 0x005a_a1a0;

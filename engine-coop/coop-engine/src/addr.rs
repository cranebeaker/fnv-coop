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

/// `TESQuest`: the pointer to the quest's own script (0: the quest has none).
/// Source: measured with the probe on 5 quests (2026-11-10): the word at +0x1C is a `Script`
/// whose owner word ([`SCRIPT_QUEST`]) is the quest again. The legacy plugin's layout table
/// has the same member (`TESScriptableForm` at +0x18, its script at +4).
pub const QUEST_SCRIPT: u32 = 0x1c;

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

/// A quest's script event list (a pointer, 0 when the quest has no script variables yet).
// ghidra: 0059e300 (the quest's list is read at +0x5c for `GetScriptVariable`).
pub const QUEST_EVENT_LIST: u32 = 0x5c;
/// An event list's variables: a pointer to a list's head node ([`NODE_ITEM`], [`NODE_NEXT`]).
// ghidra: 0084e3a0 (the pointer at +0xc), 005a9140 (the walk that finds a variable by id).
pub const EVENT_LIST_VARS: u32 = 0xc;
/// A script variable's id, a 32-bit word (the variable's index in its script).
// ghidra: 005a9140 (compared with the id asked for, read at +0).
pub const VAR_ID: u32 = 0;
/// A script variable's value, a 64-bit float.
// ghidra: 005a9140 (the value is the double at +8).
pub const VAR_VALUE: u32 = 8;

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

/// A form's id, a 32-bit word (each form: a quest, a script, a global).
// legacy: plugin/avatar/engine/engine_pins.h (`kFormRefId`).
pub const FORM_ID: u32 = 0x0c;

/// The script runner that each script run passes: method (manager, script, calling reference,
/// event list, containing object, flag, three effect words). `Script::Execute` and the three
/// effect entries call it. The higher entries do not see each run.
// ghidra: 005ac1e0 (`Script::Execute` calls it with the script, its three arguments, the flag
// and three zeros), 005e2590. Runs on the real program: the nine argument words, and 26,450 of
// 26,450 runs in play, while an override of a higher entry missed effect and console runs.
pub const SCRIPT_RUNNER: u32 = 0x005e_2590;
/// The runner's argument words: the script and the calling reference.
pub const RUNNER_ARG_SCRIPT: usize = 1;
/// See [`RUNNER_ARG_SCRIPT`].
pub const RUNNER_ARG_REFERENCE: usize = 2;
/// A script's type, a 16-bit word: [`script_type`].
// legacy: section 6 (`Script::info.type`).
pub const SCRIPT_TYPE: u32 = 0x28;
/// The values of a script's type.
pub mod script_type {
    /// An object's script (a reference, an item, an actor).
    pub const OBJECT: u16 = 0;
    /// A quest's script.
    pub const QUEST: u16 = 1;
    /// A magic effect's script.
    pub const EFFECT: u16 = 0x100;
}
/// A script's owner quest (a pointer, 0 for a script that no quest owns).
// ghidra: 005ac1e0 (the pointer at +0x40 is tested and read for the quest's script delay).
pub const SCRIPT_QUEST: u32 = 0x40;

/// `TESQuest::SetRunning`: method (quest, bool). It sets or clears [`quest_flag::RUNNING`].
// legacy: section 2. ghidra: 0060c9c0.
pub const QUEST_SET_RUNNING: u32 = 0x0060_c9c0;
/// `TESQuest::SetCompleted`: method (quest, bool).
// legacy: section 2. ghidra: 0060ca30.
pub const QUEST_SET_COMPLETED: u32 = 0x0060_ca30;
/// `TESQuest::SetFailed`: method (quest, bool).
// legacy: section 6. ghidra: 0060caf0.
pub const QUEST_SET_FAILED: u32 = 0x0060_caf0;
/// `BGSQuestObjective::SetState`: method (objective, flags 0..3: bit 0 displayed, bit 1
/// completed). It shows the HUD's notices.
// legacy: section 2 (prologue verified).
pub const OBJECTIVE_SET_STATE: u32 = 0x005e_c5d0;
/// An objective's quest (a pointer).
// legacy: section 3 (`BGSQuestObjective`: `quest` +0x10).
pub const OBJECTIVE_QUEST: u32 = 0x10;

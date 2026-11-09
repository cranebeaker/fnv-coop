//! Every FalloutNV.exe 1.4.0.525 address and layout offset the bridge uses, in one place
//! (docs/coop/SPECS.md S2). Each names the translation that defines it, so an upstream change
//! to one is found by searching for the address.

/// `TESForm::GetFormByID(id)` (`004839c0`): the form with that id, or 0. The same address the
/// translations call as `LOOKUP_FORM` (`tesobjectarmo.rs`).
pub const LOOKUP_FORM: u32 = 0x0048_39c0;

/// `TESQuest::SetStageDone(quest, stage)` (`0060d510`, `tesquest.rs`): marks the stage done,
/// raises the current stage, enables the quest and flags the change (`0x8000_0000`); false for
/// a stage the quest does not have. The script command `SetStage` calls it
/// (`tesscriptfunctions_p3.rs`, `QUEST_SET_STAGE`).
pub const QUEST_SET_STAGE_DONE: u32 = 0x0060_d510;

/// `TESQuest::GetCurrentStage(quest)` (`0060d700`, `tesquest.rs`): the byte at +0x60.
pub const QUEST_GET_CURRENT_STAGE: u32 = 0x0060_d700;

/// `ScriptRunner::Run(this, script, reference, locals, container, keep_actions, effect_start,
/// effect_finish, elapsed)` (`005e0d20`, `tesscriptrunmanager.rs`, `runner_run`): every compiled
/// script's run, quest, object, effect and result scripts alike. Answers the runner's "references
/// changed" byte.
pub const SCRIPT_RUNNER_RUN: u32 = 0x005e_0d20;

/// `Script::m_header.bIsQuestScript`: the header at +0x18, its flag at +0x10 (`tesscript.rs`,
/// `Script` and `ScriptHeader` layouts).
pub const SCRIPT_IS_QUEST_OFFSET: u32 = 0x18 + 0x10;

/// `Script::pOwnerQuest`: the word at +0x40 (`tesscript.rs`, `Script` layout).
pub const SCRIPT_OWNER_QUEST_OFFSET: u32 = 0x40;

/// `BGSSaveLoadGame::AddChange(this, form, flags, force)` (`0084a690`,
/// `bgssaveloadgame.rs`): where every form's change for the save game ends
/// (`TESForm::AddChange` `00484b60`, `TESForm::ForceChange` `00484b90`).
pub const SAVE_LOAD_ADD_CHANGE: u32 = 0x0084_a690;

/// Whether `BGSSaveLoadGame` records a change of a form: `(this, form, force)` (`0084a620`,
/// `bgssaveloadgame.rs`), the test `AddChange` makes first. The game must be recording or the
/// change forced, the form must not have form flag 0x4000, and a form with flag 0x20 must not
/// have a dynamic id. It only reads, so asking it again after the change gives the game's answer.
pub const SAVE_LOAD_RECORDS_CHANGE: u32 = 0x0084_a620;

/// `TESForm::cFormType`: a byte at +0x04 (`tesform.rs`).
pub const FORM_TYPE_OFFSET: u32 = 0x04;

/// `TESForm::iFormID`: the word at +0x0c (`leaf::word_at_c`).
pub const FORM_ID_OFFSET: u32 = 0x0c;

/// `TESGlobal::fValue`: the float at +0x24 (`run_005a6150.rs`, `TESGlobal` layout).
pub const GLOBAL_VALUE_OFFSET: u32 = 0x24;

/// The form type of a `TESGlobal` (`GLOB`, `run_005a6150.rs`).
pub const FORM_TYPE_GLOBAL: u8 = 6;

/// The form type of a `TESQuest` (`QUST`, `run_0060bef0.rs`).
pub const FORM_TYPE_QUEST: u8 = 0x47;

/// The change flag `SetStageDone` adds to its quest (`tesquest.rs`, `CHANGE_QUEST_STAGES`). The
/// same bit means other things for other form types (a cell's seen data, a topic's said-once, a
/// package's never-run: `tessaveloadgame.rs`), so it names a quest's stages only with the type.
pub const CHANGE_QUEST_STAGES: u32 = 0x8000_0000;

/// `TESQuest::m_Data.ucFlags`: the quest data at +0x3c, its flags its first byte (`tesquest.rs`,
/// `QuestData`): bit 0 enabled, 0x02 completed, 0x40 failed.
pub const QUEST_FLAGS_OFFSET: u32 = 0x3c;

/// `TESQuest`'s stage list, a `BSSimpleList<TESQuestStage *>` embedded at +0x44 (`tesquest.rs`):
/// nodes of (item, next); a node with both words zero is an empty list.
pub const QUEST_STAGE_LIST_OFFSET: u32 = 0x44;

/// `TESQuest::cCurrentStage`: the byte at +0x60 (`tesquest.rs`).
pub const QUEST_CURRENT_STAGE_OFFSET: u32 = 0x60;

/// `TESQuestStage::ucIndex`: the stage number at +0x00 (`tesquest.rs`).
pub const STAGE_INDEX_OFFSET: u32 = 0x00;

/// `TESQuestStage::bIsDone`: the byte at +0x01 (`tesquest.rs`).
pub const STAGE_DONE_OFFSET: u32 = 0x01;

/// `TESQuest::m_listObjectives`: a `BSSimpleList<BGSQuestObjective *>` embedded at +0x4c
/// (`run_0060bef0.rs`, `LIST_OBJECTIVES`), walked as the stage list.
pub const QUEST_OBJECTIVE_LIST_OFFSET: u32 = 0x4c;

/// `BGSQuestObjective::iIndex`: the objective number at +0x04 (`bgsquestobjective.rs`).
pub const OBJECTIVE_INDEX_OFFSET: u32 = 0x04;

/// `BGSQuestObjective::eState`: `QUEST_OBJECTIVE_STATE` at +0x20, bit 0 displayed, bit 1
/// completed (`bgsquestobjective.rs`; the save's quest record keeps the same word, `crates/fos`).
pub const OBJECTIVE_STATE_OFFSET: u32 = 0x20;

/// `TESQuest::pScriptLocals`: the quest script's `ScriptLocals` at +0x5c (`run_0060bef0.rs`),
/// made when the quest is initialised at load (`0060cba0`), on every machine.
pub const QUEST_SCRIPT_LOCALS_OFFSET: u32 = 0x5c;

/// `ScriptLocals::m_pLocalList`: the pointer at +0x0c to the first node of its
/// `BSSimpleList<SCRIPT_LOCAL *>` (`tesscript.rs`).
pub const LOCALS_LIST_OFFSET: u32 = 0x0c;

/// `SCRIPT_LOCAL::uiID` at +0x00 and `fValue` (an `f64`) at +0x08 (`tesscript.rs`); the game's
/// `ScriptLocals::SetVariable` (`005a9290`) writes the value there.
pub const LOCAL_ID_OFFSET: u32 = 0x00;
/// See [`LOCAL_ID_OFFSET`].
pub const LOCAL_VALUE_OFFSET: u32 = 0x08;

/// The change flag of a quest's script variables (`CHANGE_SCRIPT_LOCALS`, `tesquest.rs`).
pub const CHANGE_SCRIPT_LOCALS: u32 = 0x4000_0000;

/// The change flag of a quest's objectives (the save writes them under it, `crates/fos`).
pub const CHANGE_QUEST_OBJECTIVES: u32 = 0x2000_0000;

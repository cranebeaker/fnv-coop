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
/// `BGSQuestObjective::SetState` (`thiscall`, objective, new state; `RET 4`;
/// `run_005ec4d0.rs`, `fn_005ec5d0`; the decomp's `bgsquestobjective.cpp`): what the
/// `SetObjectiveDisplayed` (`005d7d30`) and `SetObjectiveCompleted` (`005d7c20`) script commands
/// call. Besides storing the state it flags the quest's change, enables the quest and makes it the
/// active one when the player has none (state 1), queues the HUD's lines, and adds the objective
/// to the player's list (`00952a20`), which is the list the Pip-Boy's quest page walks
/// (`0079bc30`): an objective whose state is only written is not in the Pip-Boy.
pub const OBJECTIVE_SET_STATE: u32 = 0x005e_c5d0;
/// The player's objective list: a `BSSimpleList` of `BGSQuestObjective *` whose first node is in
/// the player at this offset (`0079bc30` answers `this + 0x6bc`; `mapmenu.rs` walks it for the
/// Pip-Boy's quest page).
pub const PLAYER_OBJECTIVE_LIST_OFFSET: u32 = 0x6bc;
/// Adds an objective to the player's objective list when it is not in it (`thiscall`, player,
/// objective; `playercharacter_p2.rs`, `fn_00952a20`): what `BGSQuestObjective::SetState` calls.
pub const PLAYER_UPDATE_OBJECTIVE: u32 = 0x0095_2a20;

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

/// The pointer to the game's one `PlayerCharacter` (`011dea3c`, `PLAYER_SINGLETON` in the
/// translations); 0 before the game has made it.
pub const PLAYER_SINGLETON: u32 = 0x011d_ea3c;

/// `TESForm::iFormFlags`: the word at +0x08; bit `0x800` is "disabled" (`0044_0da0` reads it).
pub const FORM_FLAGS_OFFSET: u32 = 0x08;
/// See [`FORM_FLAGS_OFFSET`].
pub const FORM_FLAG_DISABLED: u32 = 0x800;

/// `TESObjectREFR::data.Angle.z` (the heading, radians) at +0x2c and `data.Location` (three
/// floats) at +0x30 (`tesobjectrefr.rs`, `OBJ_REFR` at +0x20).
pub const REF_HEADING_OFFSET: u32 = 0x2c;
/// See [`REF_HEADING_OFFSET`].
pub const REF_POS_OFFSET: u32 = 0x30;
/// `TESObjectREFR::pParentCell` at +0x40 and `pLoadedData` (its 3D; 0 without) at +0x64.
pub const REF_CELL_OFFSET: u32 = 0x40;
/// See [`REF_CELL_OFFSET`].
pub const REF_LOADED_OFFSET: u32 = 0x64;

/// `Actor::bProcessMe`: the byte at +0xbc the per-actor `ToggleAI` (`005bbf70`) flips; the
/// process lists skip an actor whose byte is 0.
pub const ACTOR_PROCESS_ME_OFFSET: u32 = 0xbc;

/// The `PlaceAtMe` worker, `cdecl (reference, form, count, distance, direction, health scale)`
/// (`005c4b30`, `tesscriptfunctions_p3.rs`): the new reference, at the reference's own position
/// for a count of 1 and a distance of 0; 0 when it could not be made.
pub const PLACE_AT_ME: u32 = 0x005c_4b30;

/// `Script::SetPosBase(reference, axis 'X'/'Y'/'Z', value)` (`005c0740`) and
/// `Script::SetAngleBase(reference, axis, degrees)` (`005c0b10`), the bodies of the `SetPos` and
/// `SetAngle` commands (`tesscriptfunctions_p2.rs`): they move a loaded actor's controller and
/// node too.
pub const SET_POS_BASE: u32 = 0x005c_0740;
/// See [`SET_POS_BASE`].
pub const SET_ANGLE_BASE: u32 = 0x005c_0b10;

/// `PlayerCharacter::RequestPositionPlayer(this, request)` (`0093be30`, `playercharacter.rs`):
/// queues a move of the player that the game carries out in its own frame once the cell grid can
/// take it, as a door, a fast travel and the player's `MoveTo` do. The request is 0x34 bytes
/// (`PositionPlayerRequest`), made with `operator new` (`00401000`) and its constructor
/// (`00456540`, returns `this`): the world space at +0x00 (an exterior), or the interior cell at
/// +0x04; the position at +0x08; the rotation at +0x14 (its Z, the heading, at +0x1c); "reset the
/// weather" at +0x20. The game frees it.
pub const REQUEST_POSITION_PLAYER: u32 = 0x0093_be30;
/// See [`REQUEST_POSITION_PLAYER`].
pub const OPERATOR_NEW: u32 = 0x0040_1000;
/// See [`REQUEST_POSITION_PLAYER`].
pub const POSITION_REQUEST_CONSTRUCT: u32 = 0x0045_6540;
/// See [`REQUEST_POSITION_PLAYER`].
pub const POSITION_REQUEST_SIZE: u32 = 0x34;

/// The player's virtual at slot `0x22c` with 0: true while the player cannot be moved (the
/// player's `MoveTo` does nothing then, `tesscriptfunctions_p4.rs`).
pub const PLAYER_CANNOT_MOVE_SLOT: u32 = 0x22c;

/// `TESObjectCELL` "is interior" (`00425fd0`) and `TESObjectCELL::GetWorldSpace` (`0054ddd0`: the
/// world space of an exterior cell, 0 for an interior), `tesobjectcell.rs`.
pub const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
/// See [`CELL_IS_INTERIOR`].
pub const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;

/// The form type of a `TESObjectCELL` (`CELL`, `TYPE_CELL` in the translations).
pub const FORM_TYPE_CELL: u8 = 0x39;

/// `Actor::EndMovement(this)` (`0087faa0`) and `Actor::SetGhost(this, on)` (`008acf10`).
pub const ACTOR_END_MOVEMENT: u32 = 0x0087_faa0;
/// See [`ACTOR_END_MOVEMENT`].
pub const ACTOR_SET_GHOST: u32 = 0x008a_cf10;

/// `TESObjectREFR::Disable(this)` (`00574400`).
pub const REF_DISABLE: u32 = 0x0057_4400;

/// The form types of a placed NPC (`ACHR`) and a placed creature (`ACRE`); `REFR` is 0x3a, after
/// [`FORM_TYPE_CELL`].
pub const FORM_TYPE_ACHR: u8 = 0x3b;
/// See [`FORM_TYPE_ACHR`].
pub const FORM_TYPE_ACRE: u8 = 0x3c;
/// `Actor::eLifeState` at +0x108 (`actor.rs`): 0 alive, 1 dying, 2 dead, 6 an essential actor
/// down. `Actor::SetLifeState` (`008a1800`) flags the actor's change with
/// [`CHANGE_ACTOR_LIFESTATE`] whenever the state changes, so a death is in the change journal.
pub const ACTOR_LIFE_STATE_OFFSET: u32 = 0x108;
/// See [`ACTOR_LIFE_STATE_OFFSET`].
pub const CHANGE_ACTOR_LIFESTATE: u32 = 0x400;
/// `Actor::Kill` (`thiscall`, actor, attacker or 0, damage as `f32`; `actor_p3.rs`, `actor_kill`):
/// what the `KillActor` script command and a lethal hit call. An essential or protected actor
/// stays alive; with the task queue in use the kill is queued and done within the frame.
pub const ACTOR_KILL: u32 = 0x0089_d900;

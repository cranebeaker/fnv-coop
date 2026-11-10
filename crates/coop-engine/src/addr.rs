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

// social: conversations, party saves, sleep and wait (docs/coop/SPECS.md S12 to S15).

/// `TESTopicInfo::RunResult` (`thiscall`: info, index, reference; `RET 8`; `0061f170`,
/// `testopicinfo.rs`, `tes_topic_info_run_result`): runs the info's result script at the index
/// on the reference (index 0 when the line is said, `menutopic.rs`; index 1 when it ends,
/// `dialogmenu.rs`), nothing without a reference.
pub const INFO_RUN_RESULT: u32 = 0x0061_f170;
/// The form type of a `TESTopicInfo` (`INFO`; `testopicinfo.rs` reads an info's record "as a
/// record of form type 0x46").
pub const FORM_TYPE_INFO: u8 = 0x46;
/// `TESTopicInfo::pOwnerQuest`: the word at +0x48 (`testopicinfo.rs`, `TesTopicInfo` layout).
pub const INFO_OWNER_QUEST_OFFSET: u32 = 0x48;
/// The info's prompt, a string object at +0x28 whose first word is its characters
/// (`testopicinfo.rs`: "the prompt string (+0x28)"; `00559450` reads a string's characters at
/// +0).
pub const INFO_PROMPT_OFFSET: u32 = 0x28;
/// `TESTopic::GetOwnerTopic` (`cdecl`: info; `0061a310`, `testopic.rs`): the topic that holds
/// the info, or 0.
pub const TOPIC_GET_OWNER_TOPIC: u32 = 0x0061_a310;
/// The quest under which a topic holds an info (`thiscall`: topic, info; `RET 4`; `00619d00`,
/// `run_00619b60.rs`), or 0.
pub const TOPIC_INFO_QUEST: u32 = 0x0061_9d00;
/// `TESTopicInfo::GetInfoDisplayText` (`thiscall`: info, string, first only; `RET 8`;
/// `0061e2b0`, `testopicinfo.rs`): fills the string with the text of the info's responses,
/// `" | "` between them.
pub const INFO_GET_DISPLAY_TEXT: u32 = 0x0061_e2b0;
/// A string object's constructor and destructor (`thiscall`; `004037b0`, `004037d0`;
/// `testopicinfo.rs`: "construct and destroy"). The object is 8 bytes: its characters at +0.
pub const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// See [`STRING_CONSTRUCT`].
pub const STRING_DESTROY: u32 = 0x0040_37d0;
/// See [`STRING_CONSTRUCT`].
pub const STRING_SIZE: u32 = 8;
/// Frees a block of [`OPERATOR_NEW`] (`cdecl`: block; `00401030`, `testopicinfo.rs`).
pub const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Shows a HUD message (`cdecl`, six words: text, 0, icon path or 0, 0, time as `f32`, 0;
/// `007052f0`, `run_008c51f0.rs` and `actor_p3.rs`). The text is copied (at most 0x103
/// characters, `hudmainmenu.rs`, `fn_00775380`), so the caller's buffer may be freed.
pub const SHOW_HUD_MESSAGE: u32 = 0x0070_52f0;
/// The longest text [`SHOW_HUD_MESSAGE`] keeps.
pub const HUD_MESSAGE_MAX: usize = 0x103;
/// `DialogMenu::Create` (`cdecl`: speaker, a word for the topic manager, a target or 0;
/// `00761a20`, `dialogmenu.rs`): the menu's tile, or 0 when no menu was made (as for a null
/// speaker).
pub const DIALOG_MENU_CREATE: u32 = 0x0076_1a20;
/// `DialogMenu::Close` (no arguments; `00762160`, `dialogmenu.rs`): ends the open conversation
/// as the game's own goodbye does.
pub const DIALOG_MENU_CLOSE: u32 = 0x0076_2160;
/// The open dialogue menu (`011d9510`, `dialogmenu.rs`: `Create` stores it, the destructor
/// clears it) and `DialogMenu::pSpeaker` at +0x80 (`dialogmenu.rs` layout).
pub const DIALOG_MENU_OPEN: u32 = 0x011d_9510;
/// See [`DIALOG_MENU_OPEN`].
pub const DIALOG_MENU_SPEAKER_OFFSET: u32 = 0x80;

/// The seam in `BGSSaveLoadGame::SaveGame` where the co-save is written (`00847c45`, word: the
/// save's name) and the one in `LoadGame` after the `.fos` was read (`00848c3c`, word: the
/// save's name); `bgssaveloadgame.rs`, engine-core's seam table.
pub const SAVE_GAME_SITE: u32 = 0x0084_7c45;
/// See [`SAVE_GAME_SITE`].
pub const LOAD_GAME_SITE: u32 = 0x0084_8c3c;

/// `SleepWaitMenu::DoClick` (`thiscall`: menu, button id, tile; `RET 8`; `007c0220`,
/// `sleepwaitmenu.rs`): button 4 starts the rest, button 5 cancels and closes the menu.
pub const SLEEP_WAIT_DO_CLICK: u32 = 0x007c_0220;
/// See [`SLEEP_WAIT_DO_CLICK`].
pub const SLEEP_WAIT_REST_BUTTON: u32 = 4;
/// See [`SLEEP_WAIT_DO_CLICK`].
pub const SLEEP_WAIT_CANCEL_BUTTON: u32 = 5;
/// `SleepWaitMenu`: `pTiles[1]` at +0x2c (the hours slider) and `bSleepingInBed` at +0x45
/// (`sleepwaitmenu.rs`).
pub const SLEEP_WAIT_HOURS_TILE_OFFSET: u32 = 0x2c;
/// See [`SLEEP_WAIT_HOURS_TILE_OFFSET`].
pub const SLEEP_WAIT_IN_BED_OFFSET: u32 = 0x45;
/// The trait id of the slider's value (`iUT_CurrentValue`, `011da918`) and `Tile`'s float
/// getter (`thiscall`: tile, trait; the value in `ST0`; `00a011b0`); the hours asked for are
/// that value plus one (`sleepwaitmenu.rs`, `DoClick`).
pub const SLEEP_WAIT_VALUE_TRAIT: u32 = 0x011d_a918;
/// See [`SLEEP_WAIT_VALUE_TRAIT`].
pub const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// The player's wait: `(player, hours, in bed)` stores the hours left and the bed flag
/// (`thiscall`; `005c1a00`, `tesscriptfunctions_p2.rs`; the rest button calls it,
/// `sleepwaitmenu.rs`).
pub const PLAYER_SET_WAIT: u32 = 0x005c_1a00;
/// One hour of waiting or sleeping (`thiscall`: player; `0094df80`, `playercharacter_p2.rs`):
/// the clock and the calendar move an hour, health comes back, the hours left count down.
pub const PLAYER_WAIT_HOUR: u32 = 0x0094_df80;
/// What the menu runs before each hour (`sleepwaitmenu.rs`, `UpdateSleeping`): the scripts once
/// (`thiscall` on the object at `011dea10`; `00455490`, `tes.rs`).
pub const TES_SINGLETON: u32 = 0x011d_ea10;
/// See [`TES_SINGLETON`].
pub const TES_RUN_SCRIPTS: u32 = 0x0045_5490;

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

/// `BGSQuestObjective::pOwnerQuest` (Xbox PDB): the objective's quest.
pub const OBJECTIVE_QUEST_OFFSET: u32 = 0x10;

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
/// `PlayerCharacter::RewardExperience` (`thiscall`, player, amount; vtable +0x488;
/// `playercharacter_p2.rs`, `fn_0094fd10`): what the script command `RewardXP` (`005d5140`) calls
/// on the player. It hands the amount to `CharacterProgression::RewardExperience` (`008d5100`),
/// which adds it and shows the message.
pub const PLAYER_REWARD_EXPERIENCE: u32 = 0x0094_fd10;
/// `TESContainer`'s constructor (`thiscall`, container): an empty list of items. `AddItem`
/// (`005b4be0`) builds one of 0x10 bytes on its stack.
pub const CONTAINER_CONSTRUCT: u32 = 0x0048_1610;
/// `TESContainer`'s destructor (`thiscall`, container).
pub const CONTAINER_DESTRUCT: u32 = 0x0048_1680;
/// Bytes a temporary container takes (as `AddItem`'s).
pub const CONTAINER_SIZE: u32 = 0x10;
/// Adds items to a container (`thiscall`, container, object, count, extra; `004818e0`).
pub const CONTAINER_ADD: u32 = 0x0048_18e0;
/// Hands a container's items to a reference (`thiscall`, container, reference, quiet;
/// `004821a0`): what `AddItem` gives through, a leveled list already rolled into the container.
/// Its list of `ContainerObject`s (count, object, extra) is inline at +4, `{item, next}` nodes.
pub const CONTAINER_GIVE: u32 = 0x0048_21a0;
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
/// `TESObjectREFR::Enable(this)` (`00573f40`; `tesobjectrefr_p4.rs`, `fn_00573f40`). Both end in
/// `TESForm::SetDisabled` (`00484af0`), which flags the form's change with [`CHANGE_FORM_FLAGS`]:
/// a reference a script enables or disables is in the change journal.
pub const REF_ENABLE: u32 = 0x0057_3f40;
/// See [`REF_ENABLE`].
pub const CHANGE_FORM_FLAGS: u32 = 1;
/// The form type of a placed object (`REFR`).
pub const FORM_TYPE_REFR: u8 = 0x3a;

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

/// `Actor::EvaluatePackage` (`thiscall`, actor, now, reset AI; `actor_p4.rs`): what a script's
/// `evp` ends in (`run_005c8a50.rs`, with `(0, reset)`). With `now` zero the evaluation is
/// buffered and done by the actor's next AI update.
pub const ACTOR_EVALUATE_PACKAGE: u32 = 0x008a_6ce0;

// world: the changed-form stream, doors and locks (docs/coop/SPECS.md S12). Everything below is
// named by the translation of `fallout/misc/saveload` (`engine-fallout-misc`) or of the reference
// and open-close code (`engine-fallout-cell`, `engine-fallout-forms`), as each comment says.

/// The pointer to the game's one `BGSSaveLoadGame` (`011ddf38`, `bgssaveloadgame.rs`,
/// `SAVE_LOAD_GAME`); 0 before the game has made it.
pub const SAVE_LOAD_GAME: u32 = 0x011d_df38;
/// `BGSSaveLoadGame::pChangesMap` at +0x00, `pFormIDMap` at +0x08 and the embedded
/// `FormBufferArray` at +0x24 (`bgssaveloadgame.rs`, the `BGSSaveLoadGame` layout).
pub const GAME_CHANGES_MAP_OFFSET: u32 = 0x00;
/// See [`GAME_CHANGES_MAP_OFFSET`].
pub const GAME_FORM_ID_MAP_OFFSET: u32 = 0x08;
/// See [`GAME_CHANGES_MAP_OFFSET`].
pub const GAME_FORM_BUFFER_ARRAY_OFFSET: u32 = 0x24;
/// Whether a load is running: mask 2 of the game object's word at +0x244 (`0042ce10`,
/// `thiscall`; `bgssaveloadgame.rs`, `GAME_LOADING_FLAG_TEST`). `BGSSaveLoadGame::LoadForm` does
/// nothing while it is set.
pub const GAME_LOADING_FLAG_TEST: u32 = 0x0042_ce10;
/// `BGSSaveLoadChangesMap`'s "get flags" `(map, &out, id)` (`008456e0`, `bgssaveloadgame.rs`,
/// `CHANGES_MAP_GET_FLAGS`): the change flags the save holds for a form id, 0 for none.
pub const CHANGES_MAP_GET_FLAGS: u32 = 0x0084_56e0;
/// The pointer to the `BGSSaveLoadManager` (`011de134`, `MANAGER_INSTANCE`) and its "minor
/// version" getter (`00851110`, `thiscall`, `MANAGER_GET_MINOR_VERSION`), the version byte every
/// form record's header carries (`bgssaveloadgame.rs`).
pub const SAVE_LOAD_MANAGER: u32 = 0x011d_e134;
/// See [`SAVE_LOAD_MANAGER`].
pub const MANAGER_GET_MINOR_VERSION: u32 = 0x0085_1110;

/// `BGSSaveFormBuffer` (0x24 bytes, `bgssaveformbuffer.rs`): its constructor `(this)`
/// (`008659c0`), the header setter `(this, form id, change flags, form type, minor version)`
/// (`00865ad0`), the form setter `(this, form)` (`0050f9c0`) and its destructor `(this)`
/// (`00847dd0`), used as `BGSSaveLoadGame::SaveGame` and `UnloadForm` use them
/// (`bgssaveloadgame.rs`).
pub const SAVE_FORM_BUFFER_SIZE: u32 = 0x24;
/// See [`SAVE_FORM_BUFFER_SIZE`].
pub const SAVE_FORM_BUFFER_CONSTRUCT: u32 = 0x0086_59c0;
/// See [`SAVE_FORM_BUFFER_SIZE`].
pub const SAVE_FORM_BUFFER_SET_HEADER: u32 = 0x0086_5ad0;
/// See [`SAVE_FORM_BUFFER_SIZE`].
pub const SAVE_FORM_BUFFER_SET_FORM: u32 = 0x0050_f9c0;
/// See [`SAVE_FORM_BUFFER_SIZE`].
pub const SAVE_FORM_BUFFER_DESTRUCT: u32 = 0x0084_7dd0;
/// A save or load buffer's bytes: `pBuffer` at +0x04, `iBufferSize` at +0x08 and
/// `iBufferPosition` at +0x0c (`bgssavegamebuffer.rs` and `bgsloadgamebuffer.rs`, the
/// `BGSSaveGameBuffer` and `BGSLoadGameBuffer` layouts).
pub const BUFFER_DATA_OFFSET: u32 = 0x04;
/// See [`BUFFER_DATA_OFFSET`].
pub const BUFFER_SIZE_OFFSET: u32 = 0x08;
/// See [`BUFFER_DATA_OFFSET`].
pub const BUFFER_POSITION_OFFSET: u32 = 0x0c;
/// A form buffer's nine-byte `BGSSaveLoadFormHeader` at +0x14, in a save and a load form buffer
/// alike: the form's three-byte index, the change flags at +3, the form info (its save type) at
/// +7 and the version at +8 (`bgssaveformbuffer.rs`, `bgsloadformbuffer.rs`).
pub const FORM_BUFFER_HEADER_OFFSET: u32 = 0x14;
/// See [`FORM_BUFFER_HEADER_OFFSET`].
pub const FORM_HEADER_LEN: usize = 9;

/// `TESForm::SaveGame` (slot 0x54), `LoadGame` (0x5c) and `CheckSaveGame` (0x80), each called
/// with a form buffer (`bgssaveloadgame.rs`, `FORM_SLOT_SAVE`, `FORM_SLOT_LOAD`,
/// `FORM_SLOT_CHECK_SAVE_GAME`).
pub const FORM_SLOT_SAVE_GAME: u32 = 0x54;
/// See [`FORM_SLOT_SAVE_GAME`].
pub const FORM_SLOT_LOAD_GAME: u32 = 0x5c;
/// See [`FORM_SLOT_SAVE_GAME`].
pub const FORM_SLOT_CHECK_SAVE_GAME: u32 = 0x80;
/// The initial data written before a reference's or cell's own data: saved by `(save buffer,
/// form, change flags)` (`0084eb80`) and loaded by `(load buffer, form, change flags)`
/// (`0084f330`), both cdecl (`bgssaveloadgame.rs`, `SAVE_INITIAL_DATA`, `LOAD_INITIAL_DATA`).
pub const SAVE_INITIAL_DATA: u32 = 0x0084_eb80;
/// See [`SAVE_INITIAL_DATA`].
pub const LOAD_INITIAL_DATA: u32 = 0x0084_f330;

/// `BGSLoadFormBuffer` (0x30 bytes, `bgsloadformbuffer.rs`): its constructor `(this)`
/// (`008643b0`), `iFormID` at +0x10 and `iSize` at +0x20, the form setter `(this, form)`
/// (`007037c0`) and the header's change flags setter `(this, flags)` (`00428130`), used as
/// `BGSSaveLoadGame::LoadForm` uses them (`bgssaveloadgame.rs`).
pub const LOAD_FORM_BUFFER_SIZE: u32 = 0x30;
/// See [`LOAD_FORM_BUFFER_SIZE`].
pub const LOAD_FORM_BUFFER_CONSTRUCT: u32 = 0x0086_43b0;
/// See [`LOAD_FORM_BUFFER_SIZE`].
pub const LOAD_FORM_BUFFER_FORM_ID_OFFSET: u32 = 0x10;
/// See [`LOAD_FORM_BUFFER_SIZE`].
pub const LOAD_FORM_BUFFER_DATA_SIZE_OFFSET: u32 = 0x20;
/// See [`LOAD_FORM_BUFFER_SIZE`].
pub const LOAD_FORM_BUFFER_SET_FORM: u32 = 0x0070_37c0;
/// See [`LOAD_FORM_BUFFER_SIZE`].
pub const LOAD_FORM_BUFFER_SET_HEADER_FLAGS: u32 = 0x0042_8130;
/// `(form, on)`: called on a form around the application of its load data (`0046a010`,
/// `bgssaveloadgame.rs`, `FORM_MARK_LOADED`); `LoadForm` sets it, the unload pass clears it.
pub const FORM_MARK_LOADED: u32 = 0x0046_a010;
/// `operator delete(block)` (`00401030`, `bgssaveloadgame.rs`, `OPERATOR_DELETE`):
/// `BGSLoadGameBuffer::DeleteBuffer` frees a load buffer's data with it.
pub const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Adds a word to a `BSSimpleArray` `(array, &word)` (`007cb2e0`, `bgssaveloadgame.rs`,
/// `ARRAY_ADD`): `LoadForm` hands each load buffer to the game's form buffer array with it.
pub const ARRAY_ADD: u32 = 0x007c_b2e0;
/// The unload pass `(game, full)` (`008492b0`, `bgssaveloadgame.rs`, `fn_008492b0`): every
/// buffer in the form buffer array gets `InitLoadGame` and `FinishLoadGame` and is deleted. The
/// game runs it with `full` 0 after cells were attached (`tes.rs`).
pub const GAME_UNLOAD_PASS: u32 = 0x0084_92b0;

/// `BGSSaveLoadFormIDMap` (0x24 bytes; `run_00846b60.rs`, `bgssaveloadformidmap.rs`): the
/// numbering of form ids every record's references use. Its constructor `(this, buckets)`
/// (`00846b60`; the game passes 0x13af), "get or add the index of a form id" `(map, id)`
/// (`00846c90`: index 0 is the null id, a created form's id is answered as it is), "the form id
/// of an index" `(map, index)` (`00846d20`), and the next index at +0x20. A record's three-byte
/// indices are made and resolved through the map the game object points at
/// (`bgssaveloadstructs.rs`, `BGSNumericIDIndex`).
pub const FORM_ID_MAP_SIZE: u32 = 0x24;
/// See [`FORM_ID_MAP_SIZE`].
pub const FORM_ID_MAP_CONSTRUCT: u32 = 0x0084_6b60;
/// See [`FORM_ID_MAP_SIZE`].
pub const FORM_ID_MAP_BUCKETS: u32 = 0x13af;
/// See [`FORM_ID_MAP_SIZE`].
pub const FORM_ID_MAP_GET_OR_ADD: u32 = 0x0084_6c90;
/// See [`FORM_ID_MAP_SIZE`].
pub const FORM_ID_MAP_RESOLVE: u32 = 0x0084_6d20;
/// See [`FORM_ID_MAP_SIZE`].
pub const FORM_ID_MAP_NEXT_INDEX_OFFSET: u32 = 0x20;
/// Empties a `BGSSaveLoadFormIDMap` `(map)` and reserves index 0 for the null id again
/// (`00846c50`, `thiscall`; `bgssaveloadformidmap.rs`, `fn_00846c50`).
pub const FORM_ID_MAP_RESET: u32 = 0x0084_6c50;
/// `(game, on)` on the per-thread flag `LoadForm` lowers while it loads a form; answers the
/// flag as it was, which is passed back to restore it (`004623f0`, `bgssaveloadgame.rs`,
/// `SET_THREAD_FLAG`).
pub const SET_THREAD_FLAG: u32 = 0x0046_23f0;

/// See [`FORM_TYPE_REFR`].
pub const FORM_TYPE_FACTION: u8 = 8;
/// See [`FORM_TYPE_REFR`].
pub const FORM_TYPE_NOTE: u8 = 0x31;
/// See [`FORM_TYPE_REFR`].
pub const FORM_TYPE_INFO: u8 = 0x46;
/// See [`FORM_TYPE_REFR`].
pub const FORM_TYPE_REPUTATION: u8 = 0x68;

/// See [`CHANGE_FORM_FLAGS`].
pub const CHANGE_REFR_MOVE: u32 = 0x2;
/// See [`CHANGE_FORM_FLAGS`].
pub const CHANGE_REFR_HAVOK_MOVE: u32 = 0x4;
/// See [`CHANGE_FORM_FLAGS`].
pub const CHANGE_REFR_CELL_CHANGED: u32 = 0x8;
/// See [`CHANGE_FORM_FLAGS`].
pub const CHANGE_REFR_INVENTORY: u32 = 0x20;
/// See [`CHANGE_FORM_FLAGS`].
pub const CHANGE_OBJECT_EXTRA_LOCK: u32 = 0x1000;
/// See [`CHANGE_FORM_FLAGS`].
pub const CHANGE_OBJECT_EMPTY: u32 = 0x20_0000;
/// See [`CHANGE_FORM_FLAGS`].
pub const CHANGE_OBJECT_OPEN_STATE: u32 = 0x80_0000;
/// See [`CHANGE_FORM_FLAGS`].
pub const CHANGE_REFR_LEVELED_INVENTORY: u32 = 0x800_0000;

/// `BGSOpenCloseForm::GetOpenState(reference)` (`0047b250`, cdecl; `bgsopencloseform.rs`): 0 for
/// a reference that does not open, 1 open, 2 opening, 3 closed, 4 closing.
pub const GET_OPEN_STATE: u32 = 0x0047_b250;
/// `BGSOpenCloseForm::SetOpenState(reference, open, immediate)` (`0047aec0`, cdecl;
/// `bgsopencloseform.rs`): sets or clears the reference's "open" action, which adds or removes
/// its open state's change flag (`tesobjectrefr_p4.rs`, `fn_00572d50` and `ClearAction`), and
/// plays the open or close sequence. A door closed again loses its change flag through
/// `RemoveChange`, which the change journal does not see.
pub const SET_OPEN_STATE: u32 = 0x0047_aec0;

/// A reference's own lock `(reference)` (`00569140`, `thiscall`; `tesobjectrefr_p2.rs`,
/// `LOCK_OF_REFERENCE`): its `REFR_LOCK`, or 0 without one.
pub const REF_OWN_LOCK: u32 = 0x0056_9140;
/// `REFR_LOCK` (`extradataobjects.rs`, `RefrLock`): `cBaseLevel` at +0x00, `cFlags` at +0x08
/// (bit 0 locked) and `uiNumTries` at +0x0c (a lock is broken once the tries reach the game's
/// limit, `REFR_LOCK::IsBroken`).
pub const LOCK_LEVEL_OFFSET: u32 = 0x00;
/// See [`LOCK_LEVEL_OFFSET`].
pub const LOCK_FLAGS_OFFSET: u32 = 0x08;
/// See [`LOCK_LEVEL_OFFSET`].
pub const LOCK_TRIES_OFFSET: u32 = 0x0c;
/// See [`LOCK_LEVEL_OFFSET`].
pub const LOCK_FLAG_LOCKED: u8 = 1;
/// `TESObjectREFR::Lock(this)` (`00569250`) and `UnLock(this)` (`005692a0`): they lock or
/// unlock the reference's lock and record the change; nothing without a lock
/// (`tesobjectrefr_p2.rs`).
pub const REF_LOCK: u32 = 0x0056_9250;
/// See [`REF_LOCK`].
pub const REF_UNLOCK: u32 = 0x0056_92a0;
/// `TESObjectREFR::AddLockChange(this)` (`005789b0`, `tesobjectrefr_p2.rs`, `ADD_LOCK_CHANGE`):
/// records that the reference's lock changed, as `Lock` and `UnLock` do.
pub const REF_ADD_LOCK_CHANGE: u32 = 0x0057_89b0;
// world: end
// fights (docs/coop/SPECS.md S12): every constant below names the translation it is read from.

/// The game's one `ProcessLists` object (`011e0e80`, `processlists.rs`, `PROCESS_LISTS`), its
/// `pActorsCloseToPlayerA` (50 `Actor *` at +0x88, sorted by distance to the player) and
/// `iActorsCloseToPlayer` (the count of used entries, at +0x150): the actors the game runs at
/// full detail around its player (`processlists.rs`, `ProcessLists` layout).
pub const PROCESS_LISTS: u32 = 0x011e_0e80;
/// See [`PROCESS_LISTS`].
pub const CLOSE_ACTORS_OFFSET: u32 = 0x88;
/// See [`PROCESS_LISTS`].
pub const CLOSE_ACTORS_COUNT_OFFSET: u32 = 0x150;
/// See [`PROCESS_LISTS`] (`MAX_ACTORS_CLOSE_TO_PLAYER`).
pub const CLOSE_ACTORS_MAX: u32 = 50;
/// `ProcessArray` accessors (`processlists.rs`: `array_head`, `array_tail`, `array_object`),
/// `this` the process array at `PROCESS_LISTS + 4`: the first index of a process level (0 high to
/// 3 low), the end index of one, and the mobile object at an index. Every actor the game has
/// loaded is in one of the levels, near its player or not.
pub const PROCESS_ARRAY_HEAD: u32 = 0x005d_e0f0;
/// See [`PROCESS_ARRAY_HEAD`].
pub const PROCESS_ARRAY_TAIL: u32 = 0x005b_e5c0;
/// See [`PROCESS_ARRAY_HEAD`].
pub const PROCESS_ARRAY_OBJECT: u32 = 0x0096_8670;

/// The `ActorValueOwner` part of an actor, at +0xa4, and its virtual `+0x0c`, the current value
/// of an actor value as a float (`tesconditionfunctions.rs`, `fn_0059c4f0`, the `GetActorValue`
/// condition; `actor_p6.rs`, `ACTOR_VALUE_OWNER_OFFSET`).
pub const ACTOR_VALUE_OWNER_OFFSET: u32 = 0xa4;
/// See [`ACTOR_VALUE_OWNER_OFFSET`].
pub const OWNER_CURRENT_VALUE_SLOT: u32 = 0x0c;
/// The actor value that is health (`actor_p3.rs`, `actor_get_health_percentage`: value `0x10`).
pub const ACTOR_VALUE_HEALTH: u32 = 0x10;

/// `Actor` virtual slot `0x3ac`: changes an actor value by an amount, `(value, amount as f32,
/// source actor or 0)` (`actor_p3.rs`, `ACTOR_SLOT_MODIFY_ACTOR_VALUE`; `actor_p2.rs`,
/// `actor_restore_actor_value` raises a value through it, `fn_0089d6f0` lowers health through
/// it).
pub const ACTOR_MODIFY_VALUE_SLOT: u32 = 0x3ac;

/// `Actor` virtual slot `0x338`: applies a health and a fatigue loss from an attacker,
/// `(health loss as f32, fatigue loss as f32, attacker or 0)` (`actor_p3.rs`, `fn_0089d6f0`,
/// `ACTOR_SLOT_APPLY_DAMAGE`): the difficulty's adjustment, the attacker's and the victim's
/// combat controllers told, then health lowered. `Actor::HitMe` applies a hit's damage through
/// it. It does nothing for a protected actor and does not itself kill.
pub const ACTOR_APPLY_DAMAGE_SLOT: u32 = 0x338;

/// The actor's "in combat" byte test (`00493bb0`: the byte at +0x104; `actor_p3.rs`,
/// `ACTOR_IN_COMBAT`; the `IsInCombat` condition, `tesconditionfunctions_p2.rs`).
pub const ACTOR_IN_COMBAT: u32 = 0x0049_3bb0;

/// `Actor` virtual slot `0x42c`: the actor's combat target, or 0 (`tesscriptfunctions_p5.rs`,
/// `script_get_combat_target_function`).
pub const ACTOR_COMBAT_TARGET_SLOT: u32 = 0x42c;

/// `Actor::QueueAttack` (`008935f0`, `actor_p3.rs`, `actor_queue_attack`): `thiscall` on the
/// actor with the attack's animation group. A combat procedure's way to attack: it checks the
/// weapon, the ammunition and the animation, then queues the group in `Actor::eQueuedattack`
/// (true), which the actor's update starts (`Actor::StartAttack`, `00893a40`, every frame until
/// the attack is over). The player starts its attack at once.
pub const ACTOR_QUEUE_ATTACK: u32 = 0x0089_35f0;
/// `Actor::eQueuedattack`: the attack group queued, `0xff` for none (`actor.rs`).
pub const ACTOR_QUEUED_ATTACK_OFFSET: u32 = 0x110;
/// No attack queued, in [`ACTOR_QUEUED_ATTACK_OFFSET`].
pub const NO_QUEUED_ATTACK: u32 = 0xff;

/// `Actor` virtual slot `0x428`: the actor's `CombatController`, or 0 out of combat
/// (`actor_p3.rs`, `ACTOR_SLOT_428`; `combatgroup.rs`, the target choice).
pub const ACTOR_COMBAT_CONTROLLER_SLOT: u32 = 0x428;
/// `CombatController::pCombatGroup` (`combatcontroller.rs`, PC offsets).
pub const CONTROLLER_GROUP_OFFSET: u32 = 0x80;
/// `CombatController::pAttacker`: the actor it runs.
pub const CONTROLLER_ATTACKER_OFFSET: u32 = 0xbc;
/// `CombatController::pTarget`: the actor it fights now (what [`ACTOR_COMBAT_TARGET_SLOT`]
/// answers).
pub const CONTROLLER_TARGET_OFFSET: u32 = 0xc0;
/// `CombatController`'s change of target (`00980830`, `combatcontroller.rs`, `fn_00980830`):
/// `thiscall` on the controller with the new target (0 for none); every change of the
/// controller's target goes through it (the group's choice `00986c60`, `StopCombat`, the
/// update). The target counts of the group and of the target map follow the change.
pub const CONTROLLER_SET_TARGET: u32 = 0x0098_0830;
/// The `CombatTarget` a `CombatGroup` keeps for an actor, or 0 when the actor is not one of its
/// targets (`009865d0`, `combatgroup.rs`, `fn_009865d0`): `thiscall` on the group.
pub const GROUP_TARGET_OF: u32 = 0x0098_65d0;
/// `Actor` virtual slot `0x434`: `Actor::StopCombat(bool)` (`lowprocess.rs`,
/// `ACTOR_STOP_COMBAT_SLOT`, called with 0).
pub const ACTOR_STOP_COMBAT_SLOT: u32 = 0x434;

/// The actor's movement flags (`008846e0`, `actor.rs`, `fn_008846e0`): the mover's
/// `GetMoveMode` (virtual `+0x20`), 0 without a mover. Bits: `0x1` forward, `0x2` back, `0x4`
/// left, `0x8` right, `0x100` walking, `0x200` running, `0x400` sneaking (`Actor::IsRunning`
/// reads `0x200`).
pub const ACTOR_MOVE_FLAGS: u32 = 0x0088_46e0;
/// `Actor::ForceMoveMode` (`008b3a20`, `actor_p5.rs`): `thiscall` on the actor with the flags;
/// the mover's virtual `+0xc`, which sets `uiForcedMoveMode` and `bForceMoveMode`. While forced,
/// the mover plays those flags' movement animation and does not follow its own path
/// (`ActorMover::IsPathing` is false). Calls through the actor's mover: an actor without one
/// must not be asked.
pub const ACTOR_FORCE_MOVE_MODE: u32 = 0x008b_3a20;
/// `Actor::ClearForcedMoveMode` (`008b3a50`): the mover's virtual `+0x10`; its own path again.
pub const ACTOR_CLEAR_FORCED_MOVE_MODE: u32 = 0x008b_3a50;
/// `Actor::pActorMover` (`actor.rs`).
pub const ACTOR_MOVER_OFFSET: u32 = 0x190;

/// A reference's virtual slot `0x1d0`: its 3D (its scene node), 0 while it has none: a
/// reference without one is not drawn.
pub const REF_GET_3D_SLOT: u32 = 0x1d0;

/// `Actor::pCurrentProcess`: the word at +0x68 (`leaf::word_at_68`, typed from `008d8520`).
pub const ACTOR_PROCESS_OFFSET: u32 = 0x68;

/// How the `StartCombat` command starts an actor's combat with a target
/// (`tesscriptfunctions_p2.rs`, `script_start_combat_function`): the process's virtual `0x33c`
/// with the thirteen words `(actor, target, 1, 1, 0, 0, 0, not alarmed, 0, 1, 0, 1, 0)`, where
/// "not alarmed" is the negation of the actor's virtual `0x304`.
pub const PROCESS_START_COMBAT_SLOT: u32 = 0x33c;
/// See [`PROCESS_START_COMBAT_SLOT`].
pub const ACTOR_ALARMED_SLOT: u32 = 0x304;

/// `Actor::GetEssential` (`0087f3d0`, `tesconditionfunctions_p2.rs`, `ACTOR_GET_ESSENTIAL`).
pub const ACTOR_GET_ESSENTIAL: u32 = 0x0087_f3d0;

/// `Actor::HitMe` (`0089a760`, `actor_p3.rs`, `actor_hit_me`): `thiscall` on the actor that is
/// hit, with the `HitData` and a kind byte. Every weapon hit on an actor goes through it.
pub const ACTOR_HIT_ME: u32 = 0x0089_a760;
/// `HitData` (`actor_p3.rs`, `HitData` layout): `pAggressor` at +0x00, `eDamageLimb` at +0x10
/// (-1 for none), `fHealthDamage` at +0x14.
pub const HIT_AGGRESSOR_OFFSET: u32 = 0x00;
/// See [`HIT_AGGRESSOR_OFFSET`].
pub const HIT_LIMB_OFFSET: u32 = 0x10;
/// See [`HIT_AGGRESSOR_OFFSET`].
pub const HIT_HEALTH_DAMAGE_OFFSET: u32 = 0x14;

/// The byte of the player's disabled-controls flags, at +0x680, one bit for each of the seven
/// controls (`tesconditionfunctions.rs`, `fn_005a03d0`, `PLAYER_DISABLED_CONTROLS_OFFSET`; the
/// `GetPlayerControlsDisabled` condition reads it).
pub const PLAYER_DISABLED_CONTROLS_OFFSET: u32 = 0x680;
/// See [`PLAYER_DISABLED_CONTROLS_OFFSET`]: all seven controls.
pub const PLAYER_ALL_CONTROLS: u8 = 0x7f;
// social: conversations, party saves, sleep and wait (docs/coop/SPECS.md S12 to S15).

/// `TESTopicInfo::RunResult` (`thiscall`: info, index, reference; `RET 8`; `0061f170`,
/// `testopicinfo.rs`, `tes_topic_info_run_result`): runs the info's result script at the index
/// on the reference (index 0 when the line is said, `menutopic.rs`; index 1 when it ends,
/// `dialogmenu.rs`), nothing without a reference.
pub const INFO_RUN_RESULT: u32 = 0x0061_f170;
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
/// `TES::IsCellLoaded` (`thiscall`, the TES, a cell, 0; `004511e0`): whether the game has the cell
/// in its loaded grid (or is the interior it shows), so that what is in it is drawn.
pub const CELL_IS_LOADED: u32 = 0x0045_11e0;
/// See [`TES_SINGLETON`].
pub const TES_RUN_SCRIPTS: u32 = 0x0045_5490;

// Scenes (docs/coop/SPECS.md S18), from `run_005c8740.rs`.

/// The `StartConversation` script command's body (`cdecl`, the eight script-command words: the
/// parameter info, the script data, the reference the command runs on, the containing reference,
/// the script, the event list, the result, and a pointer to the offset into the script data).
pub const CMD_START_CONVERSATION: u32 = 0x005c_8740;
/// The `SayTo` script command (`005c9100`, `run_005c8a50.rs`, `say_to`): `thisObj` says the
/// topic to the target; parameters target, topic, two numbers.
pub const CMD_SAY_TO: u32 = 0x005c_9100;
/// `MobileObject::StopCurrentDialogue` (`00934250`, `thiscall`), as `SayTo` calls it first.
pub const MOBILE_STOP_CURRENT_DIALOGUE: u32 = 0x0093_4250;
/// `0057bd60` (`thiscall` on an actor, the other actor): `SayTo`'s pairing of speaker and target.
pub const ACTOR_SAY_PAIR: u32 = 0x0057_bd60;
/// `MobileObject::GetCurrentProcessType` (`00931850`, `thiscall`): 0 for high process.
pub const MOBILE_PROCESS_TYPE: u32 = 0x0093_1850;
/// `SayTo`'s virtual slots on a process: its talking state (`0x648`, a word), whom it talks to
/// (`0x628`, the other actor), and say the topic (`0x2a4`: speaker, topic, flag, 0, 1, 1).
pub const PROCESS_TALK_STATE_SLOT: u32 = 0x648;
/// See [`PROCESS_TALK_STATE_SLOT`].
pub const PROCESS_TALK_TO_SLOT: u32 = 0x628;
/// See [`PROCESS_TALK_STATE_SLOT`].
pub const PROCESS_SAY_TOPIC_SLOT: u32 = 0x2a4;
/// The actor's "busy" virtual slot `SayTo` asks before (`0x22c`, one word, 1).
pub const ACTOR_SAY_BUSY_SLOT: u32 = 0x22c;
/// `Script::ParseParameters` (`cdecl`, varargs: the parameter info, the script data, the offset
/// pointer, the reference, the container, the script, the event list, then one pointer per
/// parameter). It moves the offset on past the parameters it read.
pub const SCRIPT_PARSE_PARAMETERS: u32 = 0x005a_ccb0;
/// `PackageLocation`: its constructor, `SetLocReference` (one word) and destructor; 12 bytes.
pub const PACKAGE_LOCATION_CONSTRUCT: u32 = 0x0067_f030;
/// See [`PACKAGE_LOCATION_CONSTRUCT`].
pub const PACKAGE_LOCATION_SET_REFERENCE: u32 = 0x0067_f3c0;
/// See [`PACKAGE_LOCATION_CONSTRUCT`].
pub const PACKAGE_LOCATION_DESTRUCT: u32 = 0x0067_f110;
/// See [`PACKAGE_LOCATION_CONSTRUCT`].
pub const PACKAGE_LOCATION_SIZE: u32 = 12;
/// A reference's virtual slot that says whether it is a mobile object (an actor).
pub const REF_IS_MOBILE_SLOT: u32 = 0x100;
/// An actor's virtual slot that starts a conversation with another (nine words: the other, two
/// locations or 0, two flags, 0, the topic, 0, 0), as the command's body calls it.
pub const ACTOR_START_CONVERSATION_SLOT: u32 = 0x280;

/// `Script::MoveToFunctionBase` (`cdecl`: the reference, a target reference, and three offsets as
/// `f32`; `tesscriptfunctions_p4.rs`): what the `MoveTo` script command ends in. It takes the
/// reference to the target's cell or world space, at the target's place plus the offsets: the one
/// function of the game that moves a reference from one space to another.
pub const MOVE_TO_FUNCTION_BASE: u32 = 0x005c_cb20;
/// `MobileObject::IsinDialogue` (`thiscall`, the actor; `009336c0`): its current package is a
/// dialogue package (procedure type 0x1c), as a conversation a script started puts it in.
pub const MOBILE_IS_IN_DIALOGUE: u32 = 0x0093_36c0;

// Packages (docs/coop/SPECS.md S19).

/// `Actor::GetCurrentEditorPackage` (`thiscall`, actor; `actor.rs`): the package the actor runs,
/// the one of its extra data when an interrupt package (a conversation) is on top; 0 for none.
pub const ACTOR_GET_CURRENT_EDITOR_PACKAGE: u32 = 0x0088_14b0;
/// `PackageList::FindCurrentPackage` (`thiscall`, list, reference; `RET 4`; `packagelist.rs`): the
/// first package of the list whose schedule and conditions hold for the reference now, else its
/// current one. What an evaluation of an actor's package ends in.
pub const PACKAGE_LIST_FIND_CURRENT: u32 = 0x0067_e780;
/// What `FindCurrentPackage` does for the package it picked before returning it: when
/// `009611e0(package)` is -1 the procedure type is computed, `006777b0(package,
/// 00569b80(reference))`.
pub const PACKAGE_PROCEDURE_TYPE: u32 = 0x0096_11e0;
/// See [`PACKAGE_PROCEDURE_TYPE`].
pub const PACKAGE_CALCULATE_PROCEDURE: u32 = 0x0067_77b0;
/// See [`PACKAGE_PROCEDURE_TYPE`].
pub const REF_PROCEDURE_VALUE: u32 = 0x0056_9b80;
/// The form type of a package (`PACK`).
pub const FORM_TYPE_PACKAGE: u8 = 0x49;

// The checks the `StartConversation` body makes before the actor's slot (`run_005c8740.rs`), and
// what `PackageList::FindCurrentPackage` does for the package it picks (`packagelist.rs`,
// `picked`): both are repeated where co-op stands in for them (peer review, 2026-11-11).

/// An actor's virtual slot whose answer 9 refuses a conversation.
pub const ACTOR_SLOT_214: u32 = 0x214;
/// `cdecl`, a reference: whether it is fading out to be disabled.
pub const REF_IS_PENDING_FADE_DISABLED: u32 = 0x005a_a6d0;
/// `MobileObject::GetCurrentPackage` (`thiscall`, reference): the process's package, 0 for none.
pub const REF_GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
/// `cdecl`, a package: its type (1 follow, 2 escort, 7 accompany; 0x1c a dialogue made while
/// playing).
pub const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// [`PACKAGE_TYPE`] of a Dialogue package from the data files (`PKDT` type 15): an NPC running
/// one goes to its target and starts a conversation with it (a force greet).
pub const PACKAGE_TYPE_DIALOGUE: u32 = 0x0f;
/// `cdecl` (player, package): what the body does for a type-0x1c package after the slot.
pub const PLAYER_SET_CONVERSATION_PACKAGE: u32 = 0x005c_8a30;
/// `thiscall`, a reference: its extra-data list.
pub const REF_EXTRA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::GetPrimitive` (`thiscall`, list; `tesobjectrefr_p4.rs`): the reference's
/// primitive, the shape of a trigger volume; 0 for a reference without one.
pub const EXTRA_GET_PRIMITIVE: u32 = 0x0041_fbe0;
/// `ExtraDataList::RemoveFollower` (`thiscall`, list, follower).
pub const EXTRA_REMOVE_FOLLOWER: u32 = 0x0042_2690;
/// `ScriptEventList::SetActionFlag`-style call for a package (`cdecl`: package, list, flag): 0x800
/// the package ended, 0x200 it began.
pub const PACKAGE_SET_ACTION_FLAG: u32 = 0x005a_c750;
/// A process's virtual slots `PackageDone` (reference, package) and `SetPackageDoneOnce` (0), and
/// the one answering the object it follows.
pub const PROCESS_PACKAGE_DONE_SLOT: u32 = 0x59c;
/// See [`PROCESS_PACKAGE_DONE_SLOT`].
pub const PROCESS_SET_DONE_ONCE_SLOT: u32 = 0x5a8;
/// See [`PROCESS_PACKAGE_DONE_SLOT`].
pub const PROCESS_FOLLOWED_SLOT: u32 = 0x128;
/// `TESPackage::pPackTarg` (+0x30): a `PackageTarget` whose type (+0, 0 = a reference) and
/// reference (+4) say whom the package is about.
pub const PACKAGE_TARGET_OFFSET: u32 = 0x30;

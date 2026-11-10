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

/// Form types of the forms the changed-form stream carries: a placed object `REFR` (0x3a, after
/// [`FORM_TYPE_CELL`]), a faction `FACT` (8), a note `NOTE` (0x31), a dialogue response `INFO`
/// (0x46, one below [`FORM_TYPE_QUEST`]) and a reputation `REPU` (0x68): the form types of the
/// game's own save type table as `crates/fos` lists it (`SAVE_TYPES`, docs/FOS_SAVES.md).
pub const FORM_TYPE_REFR: u8 = 0x3a;
/// See [`FORM_TYPE_REFR`].
pub const FORM_TYPE_FACTION: u8 = 8;
/// See [`FORM_TYPE_REFR`].
pub const FORM_TYPE_NOTE: u8 = 0x31;
/// See [`FORM_TYPE_REFR`].
pub const FORM_TYPE_INFO: u8 = 0x46;
/// See [`FORM_TYPE_REFR`].
pub const FORM_TYPE_REPUTATION: u8 = 0x68;

/// Change flags of a reference (`crates/fos`, `change_flag_name`, docs/FOS_SAVES.md; the open
/// state's flag also in `tesobjectrefr_p4.rs`, `fn_00572d50`): the form flags (enabled or not
/// among them), moved, moved by physics, moved to another cell, the inventory, the lock, an
/// emptied container, and an open state other than the default.
pub const CHANGE_FORM_FLAGS: u32 = 0x1;
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

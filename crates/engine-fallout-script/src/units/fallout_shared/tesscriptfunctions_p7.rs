//! `fallout shared/tesscriptfunctions.cpp` (Xbox PDB source unit), part 7: its functions from `005d7d30` up to
//! (not including) `005db810` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesscriptfunctions`]; anything public there may be used here.
//!
//! Progress: the whole range is translated: the first 80 queue entries
//! (`005d7d30` to `005da630`) and the last 40 (`005da690` to `005db7b0`).
//!
//! The bodies follow the conventions of the main file: `cdecl`, the eight
//! stack words as [`ScriptArgs`], `AL` as the result; a body that never reads
//! a word takes no parameters here (the commands of the second batch, from
//! `005d9160`, keep the eight words as an unused `ScriptArgs`). Objects of
//! other classes (quests, objectives, the player, topics, ...) are read at
//! the PC offsets with a comment instead of through a `layout!`.
//!
//! Notes on the exe's code that the translations rely on:
//! - Several bodies read the out-parameters of `Script::ParseParameters`
//!   from locals whose type the call does not show (floats, bytes, a text
//!   buffer). The words are passed on as the bits the game stores; a byte
//!   local is read through its low byte.
//! - `005d8a90` is slot 0 of the vtable at `0103cf54`, the one
//!   [`fn_005d8af0`] installs in a small functor that `005d8930` hands to the
//!   process lists (`0096bbc0`).
//! - The compiler's exception-unwinding frames (`005d8bf0`, `005d8e80`,
//!   `005d9810`, `005d9bc0`) are not translated, nor are the stack-cookie
//!   checks.
//! - `005d9b20` and `005da570` call `004537b0` (the task queue getter, no
//!   argument) between pushing words and calling on: the pushed words belong
//!   to the later call (`0087a8b0`, five words; `0087aa40`, two), not to the
//!   getter. `005d9bc0` is the same on a larger scale: its calls to the
//!   face-gen routines (`00652af0`, `00652470`) reuse one stack slot for a
//!   float, and the float result of `00652440` comes in `ST0`.

#[allow(unused_imports)]
use super::tesscriptfunctions::*;
#[allow(unused_imports)]
use crate::prelude::*;

// ---- Callees outside this part (by exe address) ----------------------------

/// `Script::ParseParameters` (Xbox PDB), `cdecl`, varargs.
const PARSE_PARAMETERS: u32 = 0x005a_ccb0;
/// The console print: format address first, `double` arguments take two
/// words (`cdecl`).
const CONSOLE_PRINT: u32 = 0x0070_3c00;
/// The logging stub of the unit (`005b5e40`): takes a format and its
/// arguments, does nothing and returns 0 in this build.
const LOG_STUB: u32 = 0x005b_5e40;
/// `__RTDynamicCast` (`object, 0, source type, target type, 0`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `_memset` (`dest, value, count`).
const MEMSET: u32 = 0x00ec_61c0;
/// `_strlen`.
const STRLEN: u32 = 0x00ec_6130;
/// `__stricmp` through a 21-byte wrapper (`cdecl`, two strings): 0 when
/// equal ignoring case.
const STRING_COMPARE_NO_CASE: u32 = 0x0040_4dc0;

/// `thiscall`: `*(this + 0x0c)`, the form id of a form (also read as the
/// element count of the array `00717e50` returns).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// `Script::PutNumericIDInDouble` (Xbox PDB), `cdecl` (`address of the id,
/// double* result`).
const PUT_NUMERIC_ID_IN_DOUBLE: u32 = 0x005a_cc70;
/// `this + 0x44`: the extra data list of a reference (`thiscall`).
const EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// Form type byte (`this + 4`, `thiscall`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `BaseExtraList::RemoveExtra_ov2` (Xbox PDB), `thiscall` (`extra type`).
const REMOVE_EXTRA: u32 = 0x0041_0140;
/// `thiscall` on a reference: its `Process` (`MiddleHighProcess::
/// GetSavedAcquireObject` in the map: `*(this + 0x68)`).
const GET_PROCESS: u32 = 0x008d_8520;
/// `thiscall` on the player: the player's parent cell (`*(this + 0x40)`).
const PLAYER_GET_PARENT_CELL: u32 = 0x008d_6f30;
/// `TESForm::GetFile` (Xbox PDB), `thiscall` (`0`).
const GET_FILE: u32 = 0x0048_4e60;
/// `thiscall` on a form: bit `0x20` of its flags (`this + 8`).
const FORM_IS_DELETED_FLAG: u32 = 0x0044_0d80;
/// `TESObjectREFR::RunScript` (Xbox PDB), `thiscall`.
const RUN_SCRIPT: u32 = 0x0056_5870;
/// `TESObjectREFR::GetCalcLevel` (Xbox PDB), `thiscall` (`0`).
const GET_CALC_LEVEL: u32 = 0x0056_7e10;

/// `thiscall` on a quest (`index`): the objective with that index, or 0.
const QUEST_FIND_OBJECTIVE: u32 = 0x0060_c8e0;
/// `thiscall` on a quest: bit 1 of its flags (`tesconditionfunctions.cpp`).
const QUEST_FLAG_2: u32 = 0x0059_e400;
/// `thiscall` on an objective: `(this + 0x20) & 1`.
const OBJECTIVE_BIT_0: u32 = 0x005a_5e70;
/// `thiscall` on an objective: `1 < *(this + 0x20)`.
const OBJECTIVE_ABOVE_ONE: u32 = 0x005a_5dc0;
/// `thiscall` on an objective (`flag`).
const OBJECTIVE_SET: u32 = 0x005e_c5d0;
/// Virtual slot `0x138` of a quest: its name.
const QUEST_NAME_SLOT: u32 = 0x138;

/// `FalloutRadio::PipboyRadioEnable` (Xbox PDB), `cdecl` (`flag`).
const PIPBOY_RADIO_ENABLE: u32 = 0x0083_24e0;
/// `cdecl` (`value, flag`), `falloutradio.cpp`.
const RADIO_FN_00832240: u32 = 0x0083_2240;
/// `FalloutRadio::EnableNPCRadio` (Xbox PDB), `cdecl` (`reference, value`).
const ENABLE_NPC_RADIO: u32 = 0x0083_5810;
/// `FalloutRadio::DisableNPCRadio` (Xbox PDB), `cdecl` (`reference`).
const DISABLE_NPC_RADIO: u32 = 0x0083_5980;
/// `cdecl` (`reference`): the `AL` of `GetBroadcastState`.
const RADIO_BROADCAST_STATE: u32 = 0x0083_5b90;
/// `cdecl` (`reference, flag`), `falloutradio.cpp`.
const RADIO_FN_008359E0: u32 = 0x0083_59e0;
/// `FalloutRadio::Update` (Xbox PDB), `cdecl` (`1`).
const RADIO_UPDATE: u32 = 0x0083_2ad0;
/// `FalloutRadio::StartRadioConversation` (Xbox PDB), `cdecl`
/// (`reference, value`).
const START_RADIO_CONVERSATION: u32 = 0x0083_5be0;

/// A condition function of `tesconditionfunctions.cpp`, `cdecl`
/// (`thisObj, argument, 0, double* result`): returns its `AL`.
const CONDITION_FN_005A43B0: u32 = 0x005a_43b0;
/// `Script::GetIsObjectTypeConditionFunction` (Xbox PDB), same shape.
const GET_IS_OBJECT_TYPE_CONDITION: u32 = 0x005a_4410;
/// `Script::GetDialogueEmotionConditionFunction` (Xbox PDB), same shape.
const GET_DIALOGUE_EMOTION_CONDITION: u32 = 0x005a_4480;
/// `Script::GetDialogueEmotionValueConditionFunction` (Xbox PDB), same shape.
const GET_DIALOGUE_EMOTION_VALUE_CONDITION: u32 = 0x005a_4540;
/// `Script::IsWaterObjectConditionFunction` (Xbox PDB), same shape.
const IS_WATER_OBJECT_CONDITION: u32 = 0x005a_45c0;

/// `BSAudio::QInstance` (Xbox PDB): the audio singleton.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `thiscall` on the audio object: the byte at `+6`.
const AUDIO_BYTE_6: u32 = 0x005b_b4d0;
/// `BSAudio::SetMultiThreaded` (Xbox PDB), `thiscall` (`flag`).
const AUDIO_SET_MULTI_THREADED: u32 = 0x00ad_7230;

/// `thiscall` on a reference (`value`), `tesobjectrefr.cpp`: the target of
/// the commands `005d83b0` and `005d83d0`.
const REFERENCE_FN_0056A9B0: u32 = 0x0056_a9b0;

/// Constructor of a three-float point (`thiscall`: `x, y, z`).
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// `thiscall` on a cell (`point, text, index address`), `tesobjectcell.cpp`.
const CELL_FN_005578E0: u32 = 0x0055_78e0;
/// `thiscall` on a cell, `tesobjectcell.cpp`.
const CELL_FN_005578F0: u32 = 0x0055_78f0;

/// `thiscall` (`this + 0xf8` handed to [`DEREF`]) on the value
/// [`fn_005d8710`] returns.
const DEREF_AT_F8: u32 = 0x004e_6540;
/// `thiscall`: a flag (`AL`) computed from the object [`DEREF_AT_F8`] returns.
const FN_00456610: u32 = 0x0045_6610;
/// `thiscall` on the object [`DEREF_AT_F8`] returns (`flag`).
const FN_00450F90: u32 = 0x0045_0f90;
/// `cdecl` (`index`): `*(0x011f91c8 + index * 4)`.
const FN_00450B80: u32 = 0x0045_0b80;
/// `thiscall` (`value`) on what [`FN_00450B80`] returns.
const FN_00B5D270: u32 = 0x00b5_d270;
/// `thiscall`: `*this` (the word at the address it is given).
const DEREF: u32 = 0x0055_9450;

/// `cdecl` (`value, 0.0f, 0`).
const FN_004E0330: u32 = 0x004e_0330;
/// `cdecl`, no arguments.
const FN_004E0870: u32 = 0x004e_0870;
/// `cdecl` (`a, b, 0, c, d`), the four parsed words of `005d87a0`.
const FN_004DE8E0: u32 = 0x004d_e8e0;
/// `cdecl` (`float, flag`).
const FN_004DEF00: u32 = 0x004d_ef00;
/// `cdecl` (`text, byte`).
const FN_004DEED0: u32 = 0x004d_eed0;
/// `cdecl`, no arguments.
const FN_004DEFB0: u32 = 0x004d_efb0;

/// `ProcessLists` method (`0096bbc0`, `thiscall` on the process lists, one
/// functor object).
const PROCESS_LISTS_VISIT: u32 = 0x0096_bbc0;
/// `PlayerCharacter::ResetPlayerGreetFlag` (Xbox PDB), `thiscall`.
const RESET_PLAYER_GREET_FLAG: u32 = 0x0095_3ce0;
/// `TESTopic::GetTopic` (Xbox PDB), `cdecl` (`a, b`).
const GET_TOPIC: u32 = 0x0061_a2d0;
/// `thiscall`: the first node of the list the object at `[0x011c3f2c]`
/// keeps (`this + 0x108`).
const LIST_FIRST_NODE: u32 = 0x0046_12e0;
/// `thiscall` on a list node: the next node, or 0.
const LIST_NEXT_NODE: u32 = 0x0072_6070;
/// `thiscall` on a list node: the address of its data word.
const LIST_NODE_DATA: u32 = 0x0068_15c0;
/// `BaseProcess::GetActorPackageThatIsRunning` in the map (identical code
/// folded): `this + 4`, the array of `005d8930`'s inner loop.
const ARRAY_AT_4: u32 = 0x0071_7e50;
/// `thiscall` (`index`): the address of the element, `*(this + 4) +
/// index * 4`.
const ARRAY_ELEMENT_ADDRESS: u32 = 0x0087_7a30;
/// `thiscall` on an element.
const ELEMENT_FN_0061F280: u32 = 0x0061_f280;
/// `DialogMenu::DebugResetSaidOnceFlags` (Xbox PDB), `cdecl`.
const DEBUG_RESET_SAID_ONCE_FLAGS: u32 = 0x0076_4290;
/// `thiscall` on the player.
const PLAYER_FN_00966D70: u32 = 0x0096_6d70;

/// `cdecl` (`form id`): the form with that id (`tesform.cpp`).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `TESGlobal` value setter (`float` argument, `thiscall`).
const TES_GLOBAL_SET_VALUE: u32 = 0x0046_dce0;
/// `Interface::CloseConsole` (Xbox PDB).
const CLOSE_CONSOLE: u32 = 0x0070_3da0;
/// `StartMenu::ChooseMainMenu` (Xbox PDB).
const CHOOSE_MAIN_MENU: u32 = 0x007d_0a70;

/// `BSStringT` constructor, `thiscall`.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `BSStringT` destructor, `thiscall`.
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `fallout shared/tesscriptfunctions.cpp` (another part of the unit):
/// `cdecl`, the eight words, a text and a string object.
const SCRIPT_BODY_005B4960: u32 = 0x005b_4960;
/// `Script::InitActionList` (Xbox PDB), `cdecl` (`reference, list`).
const INIT_ACTION_LIST: u32 = 0x005a_c190;
/// `Script::SetActionFlag` (Xbox PDB), `cdecl` (`reference, list, flag`).
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `thiscall` on a reference (`action reference`).
const SET_ACTION_REF: u32 = 0x0057_2e10;
/// `thiscall` (`address of a reference word`) on the list at `0x011cacb8`.
const LIST_ADD_REFERENCE: u32 = 0x005a_e3d0;

/// `TESContainer::TESContainer` (Xbox PDB), `thiscall`.
const CONTAINER_CONSTRUCT: u32 = 0x0048_1610;
/// `thiscall` on the container being built (`item, count, 0`).
const CONTAINER_ADD_ITEM: u32 = 0x0048_18e0;
/// `thiscall` (`float`), `tescontainer.cpp`.
const CONTAINER_FN_00482090: u32 = 0x0048_2090;
/// `thiscall` (`reference, flag`), `tescontainer.cpp`.
const CONTAINER_FN_004821A0: u32 = 0x0048_21a0;
/// `thiscall` on the container being built: its destructor.
const CONTAINER_DESTRUCT: u32 = 0x0048_1680;
/// `thiscall` on `form + 0x30` (`level, count, container, 0`).
const FORM_ADD_LEVELED_ITEMS: u32 = 0x0048_7f70;
/// `thiscall`: `this + 0x18`, the first node of a list.
const LIST_AT_18: u32 = 0x0050_0940;
/// `thiscall` on a list node: true when the node is empty.
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `cdecl` (`reference, a, b, c, d, float`): returns a form or 0
/// (`tesscriptfunctions.cpp`, another part of the unit).
const SCRIPT_BODY_005C4B30: u32 = 0x005c_4b30;

// ---- Globals and constants ---------------------------------------------------

/// The `PlayerCharacter` singleton pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// The process lists singleton (the `this` of [`PROCESS_LISTS_VISIT`]).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The object `005d8710` is called on (an immediate address in the code).
const OBJECT_AT_11F2250: u32 = 0x011f_2250;
/// Pointer to the object whose list [`LIST_FIRST_NODE`] walks.
const LIST_OWNER: u32 = 0x011c_3f2c;
/// Pointer to the `TESGlobal` `ExitGame` sets.
const EXIT_GAME_GLOBAL: u32 = 0x011c_3f3c;
/// The list `005d8e20` adds the reference to (an immediate address).
const REFERENCE_LIST: u32 = 0x011c_acb8;
/// The form `ExitGame` looks up (a perk).
const EXIT_GAME_FORM_ID: u32 = 0x000e_d568;
/// RTTI type descriptor of `TESForm` (`.?AVTESForm@@`).
const RTTI_TES_FORM: u32 = 0x0118_3028;
/// RTTI type descriptor of `BGSPerk` (`.?AVBGSPerk@@`).
const RTTI_BGS_PERK: u32 = 0x0118_61dc;
/// Virtual slot `0x4a0` of the player: tests a perk (`perk, 0`), `AL`.
const PLAYER_HAS_PERK_SLOT: u32 = 0x4a0;
/// Vtable of the functor built by [`fn_005d8af0`] (slot 0 is `005d8a90`).
const FUNCTOR_VTABLE: u32 = 0x0103_cf54;
/// Vtable [`fn_005d8b10`] installs (the base class, a pure virtual).
const FUNCTOR_BASE_VTABLE: u32 = 0x0103_cf5c;
/// The table `005d8550` indexes (`0x10` bytes per entry, 10 entries).
const TABLE_AT_1203258: u32 = 0x0120_3258;
/// Byte at `+0x268` of the TLS block: commands echo to the console when set.
const TLS_ECHO: u32 = 0x268;
/// Dword at `+0x290` of the TLS block: the depth of nested script runs.
const TLS_SCRIPT_DEPTH: u32 = 0x290;
/// The extra data type `005d8a90` removes.
const EXTRA_TYPE_73: u32 = 0x73;

/// Virtual slot `0x100` of a reference: a test returning `AL`.
const REFERENCE_TEST_SLOT: u32 = 0x100;
/// Virtual slot `0x310` of the process (`0`).
const PROCESS_SLOT_310: u32 = 0x310;
/// Virtual slot `0xc4` of a reference (`1`).
const REFERENCE_SLOT_C4: u32 = 0xc4;
/// Virtual slot `0xe4` of a form: a test returning `AL`.
const FORM_TEST_SLOT: u32 = 0xe4;
/// Virtual slot `0x130` of the running script: its name.
const SCRIPT_NAME_SLOT: u32 = 0x130;

// ---- String literals (addresses in the exe's data) ---------------------------

/// `"Script error: quest %s (%08X) does not exist."`
const MSG_QUEST_MISSING: u32 = 0x0103_ce84;
/// `"Script error: objective %d does not exist in quest %s (%08X)"`
const MSG_OBJECTIVE_MISSING: u32 = 0x0103_ceb4;
/// `"GetObjectiveCompleted >> %0.2f"`
const MSG_GET_OBJECTIVE_COMPLETED: u32 = 0x0103_6458;
/// `"GetObjectiveDisplayed >> %0.2f"`
const MSG_GET_OBJECTIVE_DISPLAYED: u32 = 0x0103_6478;
/// `"enable"`
const TEXT_ENABLE: u32 = 0x0103_cf04;
/// `"on"`
const TEXT_ON: u32 = 0x0101_22c8;
/// `"disable"`
const TEXT_DISABLE: u32 = 0x0103_cefc;
/// `"off"`
const TEXT_OFF: u32 = 0x0103_94c8;
/// `"tune"`
const TEXT_TUNE: u32 = 0x0103_cef4;
/// `"GetBroadcastState >> %0.2f"`
const MSG_GET_BROADCAST_STATE: u32 = 0x0103_cf0c;
/// `"VATS Lighting Off"`
const MSG_VATS_LIGHTING_OFF: u32 = 0x0103_cf28;
/// `"VATS Lighting On"`
const MSG_VATS_LIGHTING_ON: u32 = 0x0103_cf3c;
/// `"SCRIPTS: AddItem in script '%s' failed to generate an item."`
const MSG_ADD_ITEM_FAILED: u32 = 0x0103_9358;

// ---- Callees, globals and texts of the functions from 005d9160 --------------

/// `Interface::EmergencyCloseAllMenusAndBreakStuff` (Xbox PDB), `cdecl`.
const EMERGENCY_CLOSE_ALL_MENUS: u32 = 0x0070_37e0;
/// `MenuConsole::Instance` (Xbox PDB), `cdecl` (`create` byte): the console
/// menu.
const MENU_CONSOLE_INSTANCE: u32 = 0x0071_b160;
/// `thiscall` on the console menu (`text` or 0): copies the text (size
/// `0x104`) into the buffer at `this + 0x810`, or empties that buffer for 0.
const CONSOLE_SET_OUTPUT_FILE: u32 = 0x0071_dd50;
/// `Interface::CreateRaceSexMenu` (Xbox PDB), `cdecl` (`0`).
const CREATE_RACE_SEX_MENU: u32 = 0x0070_5870;
/// `Interface::QueueMenuCreate` (Xbox PDB), `cdecl` (`menu, reference, 0, 0,
/// flag, 0`).
const QUEUE_MENU_CREATE: u32 = 0x0070_9470;
/// `TutorialMenu::Create` (Xbox PDB), `cdecl` (`value, 0`).
const TUTORIAL_MENU_CREATE: u32 = 0x007e_8890;
/// `cdecl`, no arguments: always 1 (`tesscriptfunctions.cpp`).
const ALWAYS_TRUE: u32 = 0x005d_4a40;

/// `Script::GetFactionRelationConditionFunction` (Xbox PDB), condition
/// function shape (`thisObj, argument, 0, double* result`).
const GET_FACTION_RELATION_CONDITION: u32 = 0x005a_46d0;
/// `Script::IsLastPlayedIdleConditionFunction` (Xbox PDB), same shape.
const IS_LAST_PLAYED_IDLE_CONDITION: u32 = 0x005a_4780;
/// `Script::GetPlayerTeammateConditionFunction` (Xbox PDB), same shape.
const GET_PLAYER_TEAMMATE_CONDITION: u32 = 0x005a_4820;
/// `Script::GetPlayerTeammateCountConditionFunction` (Xbox PDB), same shape.
const GET_PLAYER_TEAMMATE_COUNT_CONDITION: u32 = 0x005a_48a0;
/// `Script::GetActorCrimePlayerEnemyConditionFunction` (Xbox PDB), same
/// shape.
const GET_ACTOR_CRIME_PLAYER_ENEMY_CONDITION: u32 = 0x005a_4930;
/// `Script::GetActorFactionPlayerEnemyConditionFunction` (Xbox PDB), same
/// shape.
const GET_ACTOR_FACTION_PLAYER_ENEMY_CONDITION: u32 = 0x005a_49f0;
/// `Script::GetMinorCrimeCountConditionFunction` (Xbox PDB), same shape.
const GET_MINOR_CRIME_COUNT_CONDITION: u32 = 0x005a_4aa0;
/// `Script::GetMajorCrimeCountConditionFunction` (Xbox PDB), same shape.
const GET_MAJOR_CRIME_COUNT_CONDITION: u32 = 0x005a_4b60;
/// A condition function of `tesconditionfunctions.cpp`, same shape.
const CONDITION_FN_005A4C20: u32 = 0x005a_4c20;
/// `Script::GetDestructionStageConditionFunction` (Xbox PDB), same shape.
const GET_DESTRUCTION_STAGE_CONDITION: u32 = 0x005a_4c60;
/// `Script::GetThreatRatioConditionFunction` (Xbox PDB), same shape.
const GET_THREAT_RATIO_CONDITION: u32 = 0x005a_4cd0;
/// `Script::GetIsAlignmentConditionFunction` (Xbox PDB), same shape.
const GET_IS_ALIGNMENT_CONDITION: u32 = 0x005a_4dd0;
/// `Script::GetIsUsedItemEquipTypeConditionFunction` (Xbox PDB), same shape.
const GET_IS_USED_ITEM_EQUIP_TYPE_CONDITION: u32 = 0x005a_4ea0;
/// `Script::GetAggroRadiusViolatedConditionFunction` (Xbox PDB), same shape.
const GET_AGGRO_RADIUS_VIOLATED_CONDITION: u32 = 0x005a_4f30;

/// `Actor::SetPlayerTeammate` (Xbox PDB), `thiscall` (`flag`).
const ACTOR_SET_PLAYER_TEAMMATE: u32 = 0x008b_ca90;
/// `thiscall` on an actor: the byte at `this + 0x18d` (`GetForceNextUpdate`
/// of `MiddleHighProcess` in the map).
const ACTOR_FORCE_NEXT_UPDATE: u32 = 0x0056_6950;
/// `thiscall` on the parsed value of `005d9500` (`flag` byte): calls
/// `005fc970(this, 0x10, flag)` (`tesactorbasedata.cpp`).
const FLAG_SETTER_0047EB90: u32 = 0x0047_eb90;
/// `thiscall` on an actor (`actor.cpp`), the target of `005d9550`.
const ACTOR_FN_008BCB40: u32 = 0x008b_cb40;

/// `cdecl` (`value`): true when `0x20 <= value < 0x2e` (`tesaiform.cpp`).
const VALUE_IN_RANGE_20_TO_2D: u32 = 0x0047_f060;
/// `Actor::GetClass` (Xbox PDB), `thiscall` on the player.
const ACTOR_GET_CLASS: u32 = 0x0088_4350;
/// `TESClass::SetTagSkill` (Xbox PDB), `thiscall` (`index, skill`).
const CLASS_SET_TAG_SKILL: u32 = 0x005f_6e80;
/// `thiscall` on a class (`skill`): true when the value equals one of the four
/// words at `this + 0x44` to `this + 0x50` (`tesconditionfunctions.cpp`).
const CLASS_IS_TAG_SKILL: u32 = 0x005a_5f40;
/// `thiscall` on the player: the word at `this + 0x638`, the grabbed
/// reference.
const PLAYER_GRABBED_REF: u32 = 0x0089_f4e0;
/// `PlayerCharacter::SetActiveQuest` (Xbox PDB), `thiscall` (`quest`).
const PLAYER_SET_ACTIVE_QUEST: u32 = 0x0095_29d0;

/// Operator `new` (`size`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// Operator `delete` (`block`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `Character::Character` (Xbox PDB), `thiscall` on the new block.
const CHARACTER_CONSTRUCT: u32 = 0x008d_1d30;
/// `Creature::Creature` (Xbox PDB), `thiscall` on the new block.
const CREATURE_CONSTRUCT: u32 = 0x008d_43a0;
/// `TESObjectREFR::SetEncounterZone` (Xbox PDB), `thiscall` (`zone`).
const REFERENCE_SET_ENCOUNTER_ZONE: u32 = 0x0056_7dd0;
/// `thiscall` on an extra data list (`count`): sets the count extra
/// (type `0x1e`), `extradatalist.cpp`.
const EXTRA_LIST_SET_COUNT: u32 = 0x0042_1540;
/// `thiscall` on a reference: `*(this + 0x40)`, its parent cell (the same
/// body as [`PLAYER_GET_PARENT_CELL`]).
const REFERENCE_PARENT_CELL: u32 = 0x008d_6f30;
/// `TESObjectREFR::SetObjectReference` (Xbox PDB), `thiscall` (`base form`).
const REFERENCE_SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
/// `thiscall` on `form + 0x30` (`tesactorbasedata.cpp`): a test returning
/// `AL`.
const ACTOR_BASE_DATA_TEST: u32 = 0x0047_cdb0;
/// `thiscall` on `form + 0x30` (`reference`), `tesactorbasedata.cpp`.
const ACTOR_BASE_DATA_APPLY: u32 = 0x0047_ce10;
/// `thiscall` on a cell: bit 0 of the byte at `this + 0x24`.
const CELL_FLAG_BIT_0: u32 = 0x0042_5fd0;
/// `TESObjectREFR::GetWorldSpace` (Xbox PDB), `thiscall`.
const REFERENCE_GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// `thiscall` on a reference: `this + 0x24`, the address of a field
/// (`extradataobjects.cpp`).
const REFERENCE_FIELD_24_ADDRESS: u32 = 0x0043_0830;
/// `thiscall`: the word at `this + 0x20` (named `BGSSaveFormBuffer::GetForm`
/// in the map; the body is shared by folding).
const WORD_AT_20: u32 = 0x007a_f430;
/// `thiscall` on the data handler at [`LIST_OWNER`] (`tesdatahandler.cpp`):
/// eight words (`word, word, word, cell, world space, new reference, 0, 0`).
const DATA_HANDLER_FN_004698A0: u32 = 0x0046_98a0;
/// `thiscall` (`flag`) on the new reference, `tesdatahandler.cpp`.
const DATA_HANDLER_FN_0046A010: u32 = 0x0046_a010;

/// `thiscall` on the object at [`GAME_MODE_OBJECT`] (`modelloader.cpp`): its
/// `this + 4` address.
const FN_0043D4D0: u32 = 0x0043_d4d0;
/// The object [`FN_0043D4D0`] is called on.
const GAME_MODE_OBJECT: u32 = 0x011c_3ea4;
/// The task queue interface getter (loads global `011df1a8`); no argument.
const TASK_QUEUE_INTERFACE: u32 = 0x0045_37b0;
/// `cdecl` (`queue, message, three words`), `fallout/misc`: sends a message
/// to the task queue.
const TASK_QUEUE_SEND: u32 = 0x0087_a8b0;
/// The message `005d9b20` sends instead of running `005d9bc0` directly.
const TASK_MESSAGE_11DF: u32 = 0x11df;
/// `TaskQueueInterface::QueueWeaponFire` (Xbox PDB), `thiscall` (`weapon,
/// reference`).
const QUEUE_WEAPON_FIRE: u32 = 0x0087_aa40;
/// `cdecl`, no arguments: `AL` (`fallout/ai`).
const FN_008C7AA0: u32 = 0x008c_7aa0;
/// `thiscall` on a weapon form (`reference`), `tesobjectweap.cpp`.
const WEAPON_FN_00523150: u32 = 0x0052_3150;

/// `Actor::AttackAlarm` (Xbox PDB), `thiscall` (`player, 0, 1`).
const ACTOR_ATTACK_ALARM: u32 = 0x008c_0460;
/// `thiscall` on the process lists (`fallout/ai/processlists.cpp`): the
/// actor reference for an id (`id, flag, 1`).
const PROCESS_LISTS_FIND: u32 = 0x0097_0b30;
/// `ProcessLists::GetActorRefInHigh` (Xbox PDB), `thiscall` (`reference,
/// 0`).
const PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH: u32 = 0x0097_0a20;
/// `TESObjectREFR::GetOwner` (Xbox PDB), `thiscall`.
const REFERENCE_GET_OWNER: u32 = 0x0056_7790;
/// Virtual slot `0x21c` of a reference: a test returning `AL`.
const REFERENCE_SLOT_21C: u32 = 0x21c;
/// Virtual slot `0x33c` of the process, 13 words (`MiddleHighProcess::
/// EnterCombat` at this offset in the Xbox PDB).
const PROCESS_SLOT_33C: u32 = 0x33c;

/// Virtual slot `0xf8` of a form (`IsActorBase` in the Xbox PDB).
const FORM_IS_ACTOR_BASE_SLOT: u32 = 0xf8;
/// Virtual slot `0x48` of a form (`AddChange` in the Xbox PDB): a flag word.
const FORM_SLOT_48: u32 = 0x48;
/// Virtual slot `0x1d0` of a reference: returns a word, tested for non-zero.
const REFERENCE_SLOT_1D0: u32 = 0x1d0;
/// Virtual slot `0x1f4` of a reference: returns a word.
const REFERENCE_SLOT_1F4: u32 = 0x1f4;
/// Virtual slot `0x228` of a new actor (`cell`).
const ACTOR_SLOT_228: u32 = 0x228;

/// `thiscall` on an object (`float`), `modelloader.cpp`: the target of
/// `005da1e0`.
const FN_00444020: u32 = 0x0044_4020;
/// `thiscall` on a form, `fallout shared`: the target of `005da180`.
const FN_0060D720: u32 = 0x0060_d720;
/// `cdecl` (`reference`), `bgsdestructibleobjectform.cpp`: the target of
/// `005da260`.
const FN_00477D10: u32 = 0x0047_7d10;

/// `__eh_vector_constructor_iterator__` (`array, size, count, constructor,
/// destructor`), the callee removes its arguments (`RET 0x14`).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// `__eh_vector_destructor_iterator__` (`array, size, count, destructor`),
/// the callee removes its arguments (`RET 0x10`).
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// Constructor of one 0x20-byte face-gen coordinate block (`modelloader.cpp`).
const COORD_BLOCK_CONSTRUCT: u32 = 0x0044_9610;
/// Destructor of one 0x20-byte face-gen coordinate block.
const COORD_BLOCK_DESTRUCT: u32 = 0x0044_9680;
/// Constructor of the 0x14-byte list `005d9bc0` fills
/// (`tesscriptfunctions.cpp`, another part of the unit).
const COORD_LIST_CONSTRUCT: u32 = 0x005e_0560;
/// Destructor of that list.
const COORD_LIST_DESTRUCT: u32 = 0x005e_0590;
/// `TESActorBase::GetSex` (Xbox PDB), `thiscall`.
const ACTOR_BASE_GET_SEX: u32 = 0x005f_0cc0;
/// `thiscall` on an actor base (`bipedanim.cpp`): a value computed from
/// `this + 0x10c` (the race).
const ACTOR_BASE_RACE: u32 = 0x004a_c110;
/// `cdecl` (`race, sex, list`), `tesnpc.cpp`: fills the list with the NPCs of
/// that race and sex.
const COLLECT_MATCHING_NPCS: u32 = 0x0060_3280;
/// `thiscall` on an NPC: its face-gen coordinate set, the word at `+0x1b4`
/// when set, else `this + 0x134`.
const NPC_COORDS: u32 = 0x0060_1800;
/// `cdecl` (`first, second, destination, 0`), `bsfacegenmanager.cpp`.
const FACEGEN_FN_00652E00: u32 = 0x0065_2e00;
/// `cdecl` (`coords, 0, 0`), `bsfacegenmanager.cpp`: returns a float in `ST0`.
const FACEGEN_FN_00652440: u32 = 0x0065_2440;
/// `BSFaceGenManager::InitFaceGenCoord` (Xbox PDB), `cdecl` (`coords`).
const INIT_FACEGEN_COORD: u32 = 0x0065_21f0;
/// `cdecl` (`float, coords, 1`), `bsfacegenmanager.cpp`.
const FACEGEN_FN_00653000: u32 = 0x0065_3000;
/// `cdecl` (`coords, coords, coords, 1, float`), `bsfacegenmanager.cpp`.
const FACEGEN_FN_00652AF0: u32 = 0x0065_2af0;
/// `cdecl` (`coords, 0, 0, float`), `bsfacegenmanager.cpp`.
const FACEGEN_FN_00652470: u32 = 0x0065_2470;
/// `BSFaceGenManager::CopyFaceGenCoord` (Xbox PDB), `cdecl` (`source,
/// destination, 0, 1`).
const COPY_FACEGEN_COORD: u32 = 0x0065_29a0;
/// `Character::Reset3D` (Xbox PDB), `thiscall`.
const CHARACTER_RESET_3D: u32 = 0x008d_3fa0;
/// RTTI type descriptor of `TESBoundObject` (`.?AVTESBoundObject@@`).
const RTTI_TES_BOUND_OBJECT: u32 = 0x0118_3108;
/// RTTI type descriptor of `TESNPC` (`.?AVTESNPC@@`).
const RTTI_TES_NPC: u32 = 0x0118_3a1c;
/// The `double` 100.0 `005d9bc0` divides the percentage by.
const HUNDRED: u32 = 0x0101_7a40;

/// `"SetConsoleOutputFilename >> OUTPUT_DISABLED"`
const MSG_CONSOLE_OUTPUT_DISABLED: u32 = 0x0103_cf84;
/// `"SetConsoleOutputFilename >> '%s'"`
const MSG_CONSOLE_OUTPUT_FILE: u32 = 0x0103_cf60;
/// `"IsPlayerTagSkill >> %.0f"`
const MSG_IS_PLAYER_TAG_SKILL: u32 = 0x0103_6498;
/// `"GetPlayerGrabbedRef >> (%08x)"`
const MSG_GET_PLAYER_GRABBED_REF: u32 = 0x0103_cfb0;
/// `"SCRIPTS: FireWeapon in script '%s' called with non-weapon parameter."`
const MSG_FIRE_WEAPON_NON_WEAPON: u32 = 0x0103_cfd0;

// ---- Callees, slots and globals of the commands from 005da690 -----------------

/// Virtual slot `0x218` of a reference: a test returning `AL` (the commands
/// `005da690` and `005da810` need it true on both references).
const REFERENCE_TEST_SLOT_218: u32 = 0x218;
/// Virtual slot `0x208` of a reference (`0`).
const REFERENCE_SLOT_208: u32 = 0x208;
/// Virtual slot `0x3c8` of a reference (`item, float, 1`): lowers the health
/// of an equipped item by the float.
const REFERENCE_LOWER_ITEM_HEALTH_SLOT: u32 = 0x3c8;
/// Virtual slot `0x31c` of a reference (`flag`).
const REFERENCE_SLOT_31C: u32 = 0x31c;
/// Virtual slot `0x148` of a process: the item change (equipped weapon) the
/// health commands work on, or 0.
const PROCESS_EQUIPPED_ITEM_SLOT: u32 = 0x148;
/// Virtual slot `0xfc` of a process (`reference, x, y, z`).
const PROCESS_SLOT_FC: u32 = 0xfc;

/// `thiscall` on the base form of an actor, two words (`node, reference or
/// 0`), `tesnpc.cpp`: selects the list node `node` of the list at
/// `base + 0x10c`.
const NPC_SELECT_LIST_NODE: u32 = 0x0060_b240;
/// `BSFaceGenManager::ClearBodyTexturesFromPalette` (Xbox PDB), `cdecl`
/// (`base form`).
const CLEAR_BODY_TEXTURES_FROM_PALETTE: u32 = 0x0065_70e0;

/// `Script::GetConcussedConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, value, 0, double* result`): its `AL`.
const GET_CONCUSSED_CONDITION: u32 = 0x005a_5020;
/// `Script::GetRadiationLevelConditionFunction` (Xbox PDB), same shape.
const GET_RADIATION_LEVEL_CONDITION: u32 = 0x005a_50f0;
/// `Script::GetMapMarkerVisibleConditionFunction` (Xbox PDB), same shape.
const GET_MAP_MARKER_VISIBLE_CONDITION: u32 = 0x005a_51e0;
/// `Script::GetWeaponHealthPercConditionFunction` (Xbox PDB), same shape.
const GET_WEAPON_HEALTH_PERC_CONDITION: u32 = 0x005a_5270;
/// `Script::IsGreetingPlayerConditionFunction` (Xbox PDB), same shape.
const IS_GREETING_PLAYER_CONDITION: u32 = 0x005a_5330;
/// `Script::GetIgnoreCrimeConditionFunction` (Xbox PDB), same shape.
const GET_IGNORE_CRIME_CONDITION: u32 = 0x005a_59a0;
/// `Script::IsCombatTargetConditionFunction` (Xbox PDB), same shape.
const IS_COMBAT_TARGET_CONDITION: u32 = 0x005a_53e0;
/// `Script::GetVATSRightAreaFreeConditionFunction` (Xbox PDB), same shape.
const GET_VATS_RIGHT_AREA_FREE_CONDITION: u32 = 0x005a_5460;
/// `Script::GetVATSLeftAreaFreeConditionFunction` (Xbox PDB), same shape.
const GET_VATS_LEFT_AREA_FREE_CONDITION: u32 = 0x005a_54f0;
/// `Script::GetVATSBackAreaFreeConditionFunction` (Xbox PDB), same shape.
const GET_VATS_BACK_AREA_FREE_CONDITION: u32 = 0x005a_5580;
/// `Script::GetVATSFrontAreaFreeConditionFunction` (Xbox PDB), same shape.
const GET_VATS_FRONT_AREA_FREE_CONDITION: u32 = 0x005a_5610;
/// `Script::GetVATSRightTargetVisibleConditionFunction` (Xbox PDB), same
/// shape.
const GET_VATS_RIGHT_TARGET_VISIBLE_CONDITION: u32 = 0x005a_5690;
/// `Script::GetVATSLeftTargetVisibleConditionFunction` (Xbox PDB), same
/// shape.
const GET_VATS_LEFT_TARGET_VISIBLE_CONDITION: u32 = 0x005a_5720;
/// `Script::GetVATSBackTargetVisibleConditionFunction` (Xbox PDB), same
/// shape.
const GET_VATS_BACK_TARGET_VISIBLE_CONDITION: u32 = 0x005a_57b0;

/// `thiscall` on the player (`flag`), `playercharacter.cpp`: the target of
/// `005da980`.
const PLAYER_FN_00969820: u32 = 0x0096_9820;
/// `thiscall` on the player (`flag`), `playercharacter.cpp`: the target of
/// `005da9e0`.
const PLAYER_FN_009697C0: u32 = 0x0096_97c0;

/// `thiscall` on a form (`tes.cpp`): `AL` is whether bit `0x1000000` of its
/// flags (`this + 8`) is set.
const FORM_HAS_FLAG_1000000: u32 = 0x0045_2370;
/// `BGSDestructibleObjectForm::GetDestructionForm` (Xbox PDB), `cdecl`
/// (`base form`): the destructible data of the form, or 0.
const GET_DESTRUCTION_FORM: u32 = 0x0047_5400;
/// `thiscall` on the destructible data (`tesobjectrefr.cpp`): `AL`.
const DESTRUCTIBLE_FN_00576100: u32 = 0x0057_6100;
/// `thiscall` on a form (`flag`, one byte), `tesform.cpp`.
const FORM_FN_004846E0: u32 = 0x0048_46e0;

/// `ItemChange::GetItemHealth` (Xbox PDB), `thiscall` on an item change
/// (`flag`): the health in `ST0`.
const ITEM_GET_ITEM_HEALTH: u32 = 0x004b_cdb0;
/// `ItemChange::SetItemHealth` (Xbox PDB), `thiscall` on an item change
/// (`health float, container changes, extra data list, 1`).
const ITEM_SET_ITEM_HEALTH: u32 = 0x004b_d030;
/// `ItemChange::HasModEffectActive_ov2` (Xbox PDB), `thiscall` (`effect`),
/// `AL`.
const ITEM_HAS_MOD_EFFECT_ACTIVE: u32 = 0x004b_da70;
/// `thiscall` on an item change: its form (`this + 8`).
const ITEM_GET_FORM: u32 = 0x0044_ddc0;
/// `TESObjectWEAP::GetFormHealth` (Xbox PDB), `thiscall` on the form
/// (`has mod effect`): its maximum health.
const WEAPON_GET_FORM_HEALTH: u32 = 0x004b_cf00;
/// `TESHealthForm::GetFormHealth` (Xbox PDB), `cdecl` (`form`): its maximum
/// health.
const HEALTH_FORM_GET_FORM_HEALTH: u32 = 0x0048_73d0;
/// `cdecl` (`a, b`, floats): `b` when `b <= a` (or unordered), else `a`.
const FLOAT_MIN: u32 = 0x0040_ebd0;

/// `TES::GetWorldSpace` (Xbox PDB), `thiscall` on the `TES` singleton.
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `TESWorldSpace::GetTerrainManager` (Xbox PDB), `thiscall`.
const WORLD_SPACE_GET_TERRAIN_MANAGER: u32 = 0x0058_6170;
/// `thiscall` on the terrain manager (`distant terrain system`).
const TERRAIN_MANAGER_FN_006FD060: u32 = 0x006f_d060;
/// The `TES` singleton pointer.
const TES_SINGLETON: u32 = 0x011d_ea10;
/// The `Main` singleton pointer.
const MAIN_SINGLETON: u32 = 0x011d_ea0c;

/// `thiscall` on the `Main` object (`main.cpp`): the word at `0x011f35cc`,
/// the controls object [`CONTROLS_RUMBLE`] runs on.
const MAIN_GET_CONTROLS: u32 = 0x0087_7720;
/// `Controls::Rumble` (Xbox PDB), `thiscall` (`left float, right float,
/// duration, 0, 0, 0, 1`).
const CONTROLS_RUMBLE: u32 = 0x00a2_55b0;
/// `thiscall` on a form: its sound path (`this + 0x30`).
const FORM_SOUND_PATH: u32 = 0x0051_1840;
/// `BSAudio::Precache_ov3` (Xbox PDB), `thiscall` on the audio object
/// (`path, 0x20000121, form`).
const AUDIO_PRECACHE: u32 = 0x00ad_8100;
/// The value `005db1b0` precaches its form's sound with.
const PRECACHE_FLAGS: u32 = 0x2000_0121;
/// The word `005db260` stores and `005db280` clears.
const WORD_AT_11DCFA8: u32 = 0x011d_cfa8;
/// The byte `005db6e0` stores.
const BYTE_AT_11F5AF1: u32 = 0x011f_5af1;

/// `Actor::FadeSkins` (Xbox PDB), `thiscall` on an actor (`node, float`).
const ACTOR_FADE_SKINS: u32 = 0x008b_d630;
/// `thiscall` on an actor (`modelloader.cpp`): the node `005db290` hands to
/// [`ACTOR_FADE_SKINS`].
const ACTOR_FN_0043FCD0: u32 = 0x0043_fcd0;
/// `thiscall` on the object at `actor + 0xac` (`bhkragdollcontroller.obj`;
/// three flags).
const RAGDOLL_FN_00C79D60: u32 = 0x00c7_9d60;
/// The float `005db290` passes to [`ACTOR_FADE_SKINS`] when any flag is set
/// (0.5 in the exe).
const FADE_SKINS_STRENGTH: u32 = 0x0101_6248;
/// `1000.0` (a `double`): [`script_set_rumble_function`] scales the rumble
/// duration by it.
const THOUSAND: u32 = 0x0101_7b70;

// ---- Small helpers -------------------------------------------------------------

/// `Script::ParseParameters` with the given output addresses after the seven
/// fixed words: its `AL`.
fn parse(e: &mut Engine, a: ScriptArgs, outs: &[u32]) -> bool {
    let mut words = args![
        a.param_info,
        a.script_data,
        a.opcode_offset,
        a.this_obj,
        a.containing_obj,
        a.script_obj,
        a.event_list
    ];
    words.extend_from_slice(outs);
    e.call(PARSE_PARAMETERS, &words).bool()
}

/// [`parse`] with `N` word-sized locals (the stack slots the game passes by
/// address) initialised to `init`. `None` when the parameters do not parse,
/// otherwise the values left in the locals.
fn parse_params<const N: usize>(e: &mut Engine, a: ScriptArgs, init: [u32; N]) -> Option<[u32; N]> {
    let block = e.mem.alloc(4 * N as u32);
    let mut outs = [0u32; N];
    for (i, value) in init.iter().enumerate() {
        outs[i] = block + 4 * i as u32;
        e.mem.set_u32(outs[i], *value);
    }
    let ok = parse(e, a, &outs);
    let mut values = init;
    for (i, value) in values.iter_mut().enumerate() {
        *value = e.mem.u32(outs[i]);
    }
    e.mem.free(block);
    ok.then_some(values)
}

fn console_print(e: &mut Engine, words: &[u32]) {
    e.call(CONSOLE_PRINT, words);
}

/// Whether the TLS echo flag (commands print their result) is set.
fn echo_enabled(e: &mut Engine) -> bool {
    let tls = e.tls();
    e.mem.u8(tls + TLS_ECHO) != 0
}

/// Stores 1.0 or 0.0 into the result double of a function command, then
/// echoes `"<name> >> %0.2f"` when the TLS echo flag is set.
fn store_flag_result(e: &mut Engine, result: Ptr, format: u32, value: bool) {
    e.mem.set_f64(result.addr(), if value { 1.0 } else { 0.0 });
    if echo_enabled(e) {
        let shown = e.mem.f64(result.addr());
        console_print(e, &args![format, shown]);
    }
}

/// Calls the condition function `function` with the command's `thisObj`,
/// `argument`, 0 and the result double: its `AL`.
fn call_condition(e: &mut Engine, function: u32, a: ScriptArgs, argument: u32) -> bool {
    e.call(function, &args![a.this_obj, argument, 0u32, a.result])
        .bool()
}

/// The player singleton (`[0x011dea3c]`).
fn player(e: &mut Engine) -> u32 {
    e.global::<u32>(PLAYER)
}

/// [`parse_params`] for a command that ignores the parse result: the values
/// left in the locals, whether or not the parameters parsed.
fn parse_values<const N: usize>(e: &mut Engine, a: ScriptArgs, init: [u32; N]) -> [u32; N] {
    let block = e.mem.alloc(4 * N as u32);
    let mut outs = [0u32; N];
    for (i, value) in init.iter().enumerate() {
        outs[i] = block + 4 * i as u32;
        e.mem.set_u32(outs[i], *value);
    }
    parse(e, a, &outs);
    let mut values = init;
    for (i, value) in values.iter_mut().enumerate() {
        *value = e.mem.u32(outs[i]);
    }
    e.mem.free(block);
    values
}

// ---- Translated functions --------------------------------------------------------

// Translated from 005d7d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A quest objective command (parameters: quest, objective index, flag).
/// With a missing objective it reports through the logging stub. Otherwise,
/// unless the quest has bit 1 of its flags, a zero flag calls the objective
/// setter (`005ec5d0`) with 0 and a non-zero flag calls it with 1 when bit 0
/// of the objective's word is not yet set.
///
/// A null quest takes the first error branch, which asks the null quest for
/// its form id and name (virtual slot `0x138`), exactly as the game does.
pub fn fn_005d7d30(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([quest, objective, flag]) = parse_params(e, a, [0, 0, 0]) else {
        return false;
    };
    if quest == 0 {
        let form_id = e.call(GET_FORM_ID, &args![quest]).u32();
        let name = e.vcall(quest, QUEST_NAME_SLOT, &args![]).u32();
        e.call(LOG_STUB, &args![MSG_QUEST_MISSING, name, form_id]);
    } else {
        let found = e.call(QUEST_FIND_OBJECTIVE, &args![quest, objective]).u32();
        if found == 0 {
            let form_id = e.call(GET_FORM_ID, &args![quest]).u32();
            let name = e.vcall(quest, QUEST_NAME_SLOT, &args![]).u32();
            e.call(
                LOG_STUB,
                &args![MSG_OBJECTIVE_MISSING, objective, name, form_id],
            );
        } else if !e.call(QUEST_FLAG_2, &args![quest]).bool() {
            if flag == 0 {
                e.call(OBJECTIVE_SET, &args![found, 0u32]);
            } else if !e.call(OBJECTIVE_BIT_0, &args![found]).bool() {
                e.call(OBJECTIVE_SET, &args![found, 1u32]);
            }
        }
    }
    true
}

/// The two objective getters: parse (quest, objective index, a third value
/// that is not used), find the objective and store its test as 1.0 / 0.0.
fn objective_getter(e: &mut Engine, a: ScriptArgs, test: u32, format: u32) -> bool {
    let Some([quest, objective, _unused]) = parse_params(e, a, [0, 0, 0]) else {
        return false;
    };
    if quest != 0 {
        let found = e.call(QUEST_FIND_OBJECTIVE, &args![quest, objective]).u32();
        if found != 0 {
            let value = e.call(test, &args![found]).bool();
            store_flag_result(e, a.result, format, value);
        }
    }
    true
}

// Translated from 005d7e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetObjectiveCompleted` body: with a quest and an existing objective
/// the result is 1.0 when `1 < *(objective + 0x20)`, else 0.0 (echoed as
/// `"GetObjectiveCompleted >> %0.2f"`). Always succeeds once the parameters
/// parse.
pub fn fn_005d7e30(e: &mut Engine, a: ScriptArgs) -> bool {
    objective_getter(e, a, OBJECTIVE_ABOVE_ONE, MSG_GET_OBJECTIVE_COMPLETED)
}

// Translated from 005d7ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetObjectiveDisplayed` body: like [`fn_005d7e30`] with the test
/// `(*(objective + 0x20) & 1) != 0`.
pub fn fn_005d7ef0(e: &mut Engine, a: ScriptArgs) -> bool {
    objective_getter(e, a, OBJECTIVE_BIT_0, MSG_GET_OBJECTIVE_DISPLAYED)
}

// Translated from 005d7fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PipboyRadio` (Xbox PDB): parses a text and an integer. A text
/// that starts with `'1'`, or equals (ignoring case) `"enable"` or `"on"`,
/// enables the Pip-Boy radio and calls `00832240(value, 1)`; one that starts
/// with `'0'`, or equals `"disable"` or `"off"`, disables it; `"tune"` only
/// calls `00832240(value, 1)`. Succeeds once the parameters parse.
pub fn script_pipboy_radio(e: &mut Engine, a: ScriptArgs) -> bool {
    let text = e.mem.alloc(0x200);
    let value = e.mem.alloc(4);
    e.mem.set_u32(value, 0);
    let ok = parse(e, a, &[text, value]);
    if ok {
        let value = e.mem.u32(value);
        let first = e.mem.i8(text);
        let same = |e: &mut Engine, word: u32| {
            e.call(STRING_COMPARE_NO_CASE, &args![text, word]).i32() == 0
        };
        if first == b'1' as i8 || same(e, TEXT_ENABLE) || same(e, TEXT_ON) {
            e.call(PIPBOY_RADIO_ENABLE, &args![1u32]);
            e.call(RADIO_FN_00832240, &args![value, 1u32]);
        } else if first == b'0' as i8 || same(e, TEXT_DISABLE) || same(e, TEXT_OFF) {
            e.call(PIPBOY_RADIO_ENABLE, &args![0u32]);
        } else if same(e, TEXT_TUNE) {
            e.call(RADIO_FN_00832240, &args![value, 1u32]);
        }
    }
    e.mem.free(text);
    e.mem.free(value);
    ok
}

// Translated from 005d8100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetNPCRadio` (Xbox PDB): parses a mode and a value. Fails
/// (returns false) unless `thisObj` exists, its virtual test (slot `0x100`)
/// holds and the value is non-zero; then mode 1 enables the NPC radio of
/// `thisObj` with the value, mode 0 disables it, and the command succeeds.
pub fn script_set_npc_radio(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([mode, value]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if a.this_obj.is_null()
        || !e
            .vcall(a.this_obj.addr(), REFERENCE_TEST_SLOT, &args![])
            .bool()
        || value == 0
    {
        return false;
    }
    if mode == 1 {
        e.call(ENABLE_NPC_RADIO, &args![a.this_obj, value]);
    } else if mode == 0 {
        e.call(DISABLE_NPC_RADIO, &args![a.this_obj]);
    }
    true
}

// Translated from 005d81b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetBroadcastState` (Xbox PDB): the result is 1.0 when
/// `00835b90(thisObj)` holds, else 0.0 (echoed as `"GetBroadcastState >>
/// %0.2f"`). Always succeeds.
pub fn script_get_broadcast_state(e: &mut Engine, a: ScriptArgs) -> bool {
    let state = e.call(RADIO_BROADCAST_STATE, &args![a.this_obj]).bool();
    store_flag_result(e, a.result, MSG_GET_BROADCAST_STATE, state);
    true
}

// Translated from 005d8220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Radio command: parses an integer and calls `008359e0(thisObj, value > 0)`
/// (signed). Returns the parse result.
pub fn fn_005d8220(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    e.call(
        RADIO_FN_008359E0,
        &args![a.this_obj, ((value as i32) > 0) as u32],
    );
    true
}

// Translated from 005d8280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Radio command: `FalloutRadio::Update(1)`. Always succeeds.
pub fn fn_005d8280(e: &mut Engine) -> bool {
    e.call(RADIO_UPDATE, &args![1u32]);
    true
}

// Translated from 005d82a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::StartRadioConversation` (Xbox PDB): parses an integer and calls
/// `FalloutRadio::StartRadioConversation(thisObj, value)`. Returns the parse
/// result.
pub fn script_start_radio_conversation(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    e.call(START_RADIO_CONVERSATION, &args![a.this_obj, value]);
    true
}

// Translated from 005d8300 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command with no script parameters: `005a43b0(thisObj, 0, 0, result)`;
/// the `AL` of the callee is the command's.
pub fn fn_005d8300(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, CONDITION_FN_005A43B0, a, 0)
}

// Translated from 005d8320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Audio command: parses an integer. A non-zero value, when the byte at
/// `+6` of the audio singleton is clear, switches the audio to
/// multi-threaded (`SetMultiThreaded(1)`); a zero value, when it is set,
/// switches it off. Succeeds once the parameter parses.
pub fn fn_005d8320(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([enable]) = parse_params(e, a, [0]) else {
        return false;
    };
    if enable != 0 {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        if !e.call(AUDIO_BYTE_6, &args![audio]).bool() {
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            e.call(AUDIO_SET_MULTI_THREADED, &args![audio, 1u32]);
            return true;
        }
    }
    if enable == 0 {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        if e.call(AUDIO_BYTE_6, &args![audio]).bool() {
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            e.call(AUDIO_SET_MULTI_THREADED, &args![audio, 0u32]);
        }
    }
    true
}

// Translated from 005d83b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reference command with no script parameters: `0056a9b0(thisObj, 0)`.
/// Always succeeds.
pub fn fn_005d83b0(e: &mut Engine, a: ScriptArgs) -> bool {
    e.call(REFERENCE_FN_0056A9B0, &args![a.this_obj, 0u32]);
    true
}

// Translated from 005d83d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reference command: parses an integer and calls `0056a9b0(thisObj,
/// value)`. Returns the parse result.
pub fn fn_005d83d0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    e.call(REFERENCE_FN_0056A9B0, &args![a.this_obj, value]);
    true
}

// Translated from 005d8420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cell command: parses three floats (a point), a text (up to 0x200 bytes,
/// zeroed first) and an integer. When the player has a parent cell, builds
/// the point on the stack and calls `005578e0` on the cell with the point,
/// the text (0 when it is empty) and the table entry [`fn_005d8550`] gives
/// for the integer. Returns the parse result.
pub fn fn_005d8420(e: &mut Engine, a: ScriptArgs) -> bool {
    let text = e.mem.alloc(0x200);
    e.mem.set_u8(text, 0);
    e.call(MEMSET, &args![text + 1, 0u32, 0x1ffu32]);
    let words = e.mem.alloc(16);
    e.mem.set_u32(words + 12, 0);
    let ok = parse(e, a, &[words, words + 4, words + 8, text, words + 12]);
    if ok {
        let (x, y, z) = (e.mem.u32(words), e.mem.u32(words + 4), e.mem.u32(words + 8));
        let index = e.mem.u32(words + 12);
        let player_object = player(e);
        if e.call(PLAYER_GET_PARENT_CELL, &args![player_object]).u32() != 0 {
            e.with_stack(12, |e, point| {
                e.call(POINT3_CONSTRUCT, &args![point, x, y, z]);
                let empty = e.call(STRLEN, &args![text]).u32() == 0;
                let entry = fn_005d8550(e, index);
                let player_object = player(e);
                let cell = e.call(PLAYER_GET_PARENT_CELL, &args![player_object]).u32();
                let shown = if empty { 0 } else { text };
                e.call(CELL_FN_005578E0, &args![cell, point, shown, entry]);
            });
        }
    }
    e.mem.free(text);
    e.mem.free(words);
    ok
}

// Translated from 005d8550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of entry `index % 10` of the 16-byte-entry table at
/// `0x01203258`.
pub fn fn_005d8550(_e: &mut Engine, index: u32) -> u32 {
    TABLE_AT_1203258 + (index % 10) * 0x10
}

// Translated from 005d8570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cell command with no script parameters: when the player has a parent
/// cell, calls `005578f0` on it. Always succeeds.
pub fn fn_005d8570(e: &mut Engine) -> bool {
    let player_object = player(e);
    if e.call(PLAYER_GET_PARENT_CELL, &args![player_object]).u32() != 0 {
        let player_object = player(e);
        let cell = e.call(PLAYER_GET_PARENT_CELL, &args![player_object]).u32();
        e.call(CELL_FN_005578F0, &args![cell]);
    }
    true
}

// Translated from 005d85a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsObjectTypeFunction` (Xbox PDB): parses one byte-sized
/// argument and returns `GetIsObjectTypeConditionFunction(thisObj, type, 0,
/// result)`.
pub fn script_get_is_object_type_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([object_type]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, GET_IS_OBJECT_TYPE_CONDITION, a, object_type & 0xff)
}

// Translated from 005d8600 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command with no script parameters:
/// `GetDialogueEmotionConditionFunction(thisObj, 0, 0, result)`; its `AL` is
/// the command's.
pub fn fn_005d8600(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_DIALOGUE_EMOTION_CONDITION, a, 0)
}

// Translated from 005d8620 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command with no script parameters:
/// `GetDialogueEmotionValueConditionFunction(thisObj, 0, 0, result)`; its
/// `AL` is the command's.
pub fn fn_005d8620(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_DIALOGUE_EMOTION_VALUE_CONDITION, a, 0)
}

// Translated from 005d8640 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command with no script parameters:
/// `IsWaterObjectConditionFunction(thisObj, 0, 0, result)`; its `AL` is the
/// command's.
pub fn fn_005d8640(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_WATER_OBJECT_CONDITION, a, 0)
}

// Translated from 005d8660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleVATSLight` (Xbox PDB): when [`fn_005d8710`] on the object
/// at `0x011f2250` yields a non-zero value, flips a setting. The current
/// state is the `AL` of `00456610` on the object `004e6540` derives. State
/// set: prints `"VATS Lighting On"`, calls `00450f90(0)` on the derived
/// object, then `00b5d270` on the entry `00450b80(0)` returns, with the
/// value of `005d8710`. State clear: prints `"VATS Lighting Off"` and calls
/// `00450f90(1)`. Always succeeds.
pub fn script_toggle_vats_light(e: &mut Engine) -> bool {
    if fn_005d8710(e, OBJECT_AT_11F2250) == 0 {
        return true;
    }
    let first = fn_005d8710(e, OBJECT_AT_11F2250);
    let derived = e.call(DEREF_AT_F8, &args![first]).u32();
    if e.call(FN_00456610, &args![derived]).bool() {
        console_print(e, &args![MSG_VATS_LIGHTING_ON]);
        let value = fn_005d8710(e, OBJECT_AT_11F2250);
        let derived = e.call(DEREF_AT_F8, &args![value]).u32();
        e.call(FN_00450F90, &args![derived, 0u32]);
        let value = fn_005d8710(e, OBJECT_AT_11F2250);
        let entry = e.call(FN_00450B80, &args![0u32]).u32();
        e.call(FN_00B5D270, &args![entry, value]);
    } else {
        console_print(e, &args![MSG_VATS_LIGHTING_OFF]);
        let value = fn_005d8710(e, OBJECT_AT_11F2250);
        let derived = e.call(DEREF_AT_F8, &args![value]).u32();
        e.call(FN_00450F90, &args![derived, 1u32]);
    }
    true
}

// Translated from 005d8710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall`: what `00559450` returns for the address `this + 0x34` (the
/// word stored there).
pub fn fn_005d8710(e: &mut Engine, this: u32) -> u32 {
    e.call(DEREF, &args![this + 0x34]).u32()
}

// Translated from 005d8730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer and calls `004e0330(value, 0.0f, 0)`. Returns the parse
/// result.
pub fn fn_005d8730(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    e.call(FN_004E0330, &args![value, 0.0f32, 0u32]);
    true
}

// Translated from 005d8790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `004e0870()`. Always succeeds.
pub fn fn_005d8790(e: &mut Engine) -> bool {
    e.call(FN_004E0870, &args![]);
    true
}

// Translated from 005d87a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer and three floats and calls `004de8e0(integer, first
/// float, 0, second float, third float)`. Returns the parse result.
pub fn fn_005d87a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([integer, first, second, third]) = parse_params(e, a, [0, 0, 0, 0]) else {
        return false;
    };
    e.call(FN_004DE8E0, &args![integer, first, 0u32, second, third]);
    true
}

// Translated from 005d8830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a float and a byte flag and calls `004def00(float, flag != 0)`.
/// Returns the parse result.
pub fn fn_005d8830(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value, flag]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    e.call(FN_004DEF00, &args![value, ((flag & 0xff) != 0) as u32]);
    true
}

// Translated from 005d88a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a text (0x100 bytes) and a byte and calls `004deed0(text, byte)`.
/// Returns the parse result.
pub fn fn_005d88a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let text = e.mem.alloc(0x100);
    let byte = e.mem.alloc(4);
    e.mem.set_u32(byte, 0);
    let ok = parse(e, a, &[text, byte]);
    if ok {
        let byte_value = e.mem.u32(byte) & 0xff;
        e.call(FN_004DEED0, &args![text, byte_value]);
    }
    e.mem.free(text);
    e.mem.free(byte);
    ok
}

// Translated from 005d8920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `004defb0()`. Always succeeds.
pub fn fn_005d8920(e: &mut Engine) -> bool {
    e.call(FN_004DEFB0, &args![]);
    true
}

// Translated from 005d8930 (decompiled, FalloutNV.exe 1.4.0.525)
/// A debug command with no script parameters that resets the dialogue state:
/// hands a functor (built by [`fn_005d8af0`], whose slot 0 is
/// [`fn_005d8a90`]) to the process lists, resets the player's greet flag,
/// then walks the list at `[0x011c3f2c]`: for every non-null topic entry it
/// walks the topic's own list (`topic + 0x2c`), and for every non-null item
/// of that list calls `0061f280` on each non-null element of the item's
/// array (`00717e50`/`00877a30`). Ends with
/// `DialogMenu::DebugResetSaidOnceFlags`. Always succeeds.
///
/// The game also looks up two topics (`GetTopic(6, 0xc)` and `GetTopic(1,
/// 0)`) and sets two local flags when an entry equals them; the flags are
/// never read, so only the two lookups are kept.
pub fn fn_005d8930(e: &mut Engine) -> bool {
    e.with_stack(8, |e, functor| {
        fn_005d8af0(e, functor);
        e.call(PROCESS_LISTS_VISIT, &args![PROCESS_LISTS, functor]);
    });
    let player_object = player(e);
    e.call(RESET_PLAYER_GREET_FLAG, &args![player_object]);
    e.call(GET_TOPIC, &args![6u32, 0xcu32]);
    e.call(GET_TOPIC, &args![1u32, 0u32]);
    let owner = e.global::<u32>(LIST_OWNER);
    let mut node = e.call(LIST_FIRST_NODE, &args![owner]).u32();
    while node != 0 {
        let data = e.call(LIST_NODE_DATA, &args![node]).u32();
        let topic = e.mem.u32(data);
        if topic != 0 {
            let mut inner = fn_005d8a70(e, topic);
            while inner != 0 {
                let data = e.call(LIST_NODE_DATA, &args![inner]).u32();
                let item = e.mem.u32(data);
                if item != 0 {
                    let array = e.call(ARRAY_AT_4, &args![item]).u32();
                    let count = e.call(GET_FORM_ID, &args![array]).u32();
                    for index in 0..count {
                        let slot = e.call(ARRAY_ELEMENT_ADDRESS, &args![array, index]).u32();
                        let element = e.mem.u32(slot);
                        if element != 0 {
                            e.call(ELEMENT_FN_0061F280, &args![element]);
                        }
                    }
                }
                inner = e.call(LIST_NEXT_NODE, &args![inner]).u32();
            }
        }
        node = e.call(LIST_NEXT_NODE, &args![node]).u32();
    }
    e.call(DEBUG_RESET_SAID_ONCE_FLAGS, &args![]);
    true
}

// Translated from 005d8a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall`: `this + 0x2c`, the list embedded in a topic.
pub fn fn_005d8a70(_e: &mut Engine, this: u32) -> u32 {
    this + 0x2c
}

// Translated from 005d8a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot 0 of the functor's vtable (`thiscall`, one argument, `RET 4`; the
/// functor itself is never read): when the actor's virtual test (slot
/// `0x100`) holds and it has a process, calls the process's slot `0x310`
/// with 0; then removes extra data type `0x73` from the actor's extra data
/// list.
pub fn fn_005d8a90(e: &mut Engine, _unused_this: Ptr, actor: Ptr) {
    if e.vcall(actor.addr(), REFERENCE_TEST_SLOT, &args![]).bool() {
        let process = e.call(GET_PROCESS, &args![actor]).u32();
        if process != 0 {
            e.vcall(process, PROCESS_SLOT_310, &args![0u32]);
        }
    }
    let list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
    e.call(REMOVE_EXTRA, &args![list, EXTRA_TYPE_73]);
}

// Translated from 005d8af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 8-byte functor: the base constructor
/// ([`fn_005d8b10`]), then the vtable at `0x0103cf54`. Returns `this`.
pub fn fn_005d8af0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005d8b10(e, this);
    e.mem.set_u32(this.addr(), FUNCTOR_VTABLE);
    this
}

// Translated from 005d8b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of the functor: vtable `0x0103cf5c` (a pure virtual
/// base), word at `+4` cleared. Returns `this`.
pub fn fn_005d8b10(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), FUNCTOR_BASE_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 005d8b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00966d70` on the player. Always succeeds.
pub fn fn_005d8b40(e: &mut Engine) -> bool {
    let player_object = player(e);
    e.call(PLAYER_FN_00966D70, &args![player_object]);
    true
}

// Translated from 005d8b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ExitGame` (Xbox PDB): looks up form `0x000ed568`, casts it from
/// `TESForm` to `BGSPerk`, asks the player (virtual slot `0x4a0`) about that
/// perk, stores 2.0 (perk held) or 1.0 in the `TESGlobal` at `[0x011c3f3c]`,
/// closes the console and goes to the main menu. Always succeeds.
pub fn script_exit_game(e: &mut Engine) -> bool {
    let form = e.call(LOOKUP_FORM, &args![EXIT_GAME_FORM_ID]).u32();
    let perk = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, RTTI_TES_FORM, RTTI_BGS_PERK, 0u32],
        )
        .u32();
    let player_object = player(e);
    let held = e
        .vcall(player_object, PLAYER_HAS_PERK_SLOT, &args![perk, 0u32])
        .bool();
    let global = e.global::<u32>(EXIT_GAME_GLOBAL);
    let value = if held { 2.0f32 } else { 1.0f32 };
    e.call(TES_GLOBAL_SET_VALUE, &args![global, value]);
    e.call(CLOSE_CONSOLE, &args![]);
    e.call(CHOOSE_MAIN_MENU, &args![]);
    true
}

// Translated from 005d8bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a text (0x200 bytes), builds an 8-byte `BSStringT` and calls
/// `005b4960(the eight words, text, string)`; its `AL` is the command's. The
/// string is destroyed afterwards. The exception-unwinding frame is not
/// translated.
pub fn fn_005d8bf0(e: &mut Engine, a: ScriptArgs) -> bool {
    let text = e.mem.alloc(0x200);
    if !parse(e, a, &[text]) {
        e.mem.free(text);
        return false;
    }
    let result = e.with_stack(8, |e, string| {
        e.call(STRING_CONSTRUCT, &args![string]);
        let result = e.call(SCRIPT_BODY_005B4960, &args![a, text, string]).bool();
        e.call(STRING_DESTRUCT, &args![string]);
        result
    });
    e.mem.free(text);
    result
}

// Translated from 005d8cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::TriggerEnterFunction` (Xbox PDB): parses a reference. With a
/// `thisObj`: initialises its action list (`InitActionList(thisObj,
/// extra data list)`), sets action flags `0x20000000` and `0x10000000` for
/// the parsed reference, sets it as the action reference of `thisObj`
/// (`00572e10`) and, while the TLS script depth (`+0x290`) is below 5, runs
/// `thisObj`'s script with the depth raised by one. Returns the parse
/// result.
pub fn script_trigger_enter_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([target]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        let list = e.call(EXTRA_DATA_LIST, &args![a.this_obj]).u32();
        e.call(INIT_ACTION_LIST, &args![a.this_obj, list]);
        let list = e.call(EXTRA_DATA_LIST, &args![a.this_obj]).u32();
        e.call(SET_ACTION_FLAG, &args![target, list, 0x2000_0000u32]);
        let list = e.call(EXTRA_DATA_LIST, &args![a.this_obj]).u32();
        e.call(SET_ACTION_FLAG, &args![target, list, 0x1000_0000u32]);
        e.call(SET_ACTION_REF, &args![a.this_obj, target]);
        let tls = e.tls();
        if e.mem.i32(tls + TLS_SCRIPT_DEPTH) < 5 {
            let depth = e.mem.i32(tls + TLS_SCRIPT_DEPTH);
            e.mem.set_i32(tls + TLS_SCRIPT_DEPTH, depth + 1);
            e.call(RUN_SCRIPT, &args![a.this_obj]);
            let depth = e.mem.i32(tls + TLS_SCRIPT_DEPTH);
            e.mem.set_i32(tls + TLS_SCRIPT_DEPTH, depth - 1);
        }
    }
    true
}

// Translated from 005d8e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reference command with no script parameters: for a `thisObj` that is
/// not flagged by `00440d80` and not the player, calls its virtual slot
/// `0xc4` with 1 and, when `TESForm::GetFile(0)` gives no file, adds it (by
/// address of a stack word) to the list at `0x011cacb8` (`005ae3d0`).
/// Always succeeds.
pub fn fn_005d8e20(e: &mut Engine, a: ScriptArgs) -> bool {
    let object = a.this_obj;
    if !object.is_null()
        && !e.call(FORM_IS_DELETED_FLAG, &args![object]).bool()
        && object.addr() != player(e)
    {
        e.vcall(object.addr(), REFERENCE_SLOT_C4, &args![1u32]);
        if e.call(GET_FILE, &args![object, 0u32]).u32() == 0 {
            e.with_stack(4, |e, cell| {
                e.mem.set_u32(cell.addr(), object.addr());
                e.call(LIST_ADD_REFERENCE, &args![REFERENCE_LIST, cell]);
            });
        }
    }
    true
}

// Translated from 005d8e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AddItem`-style body (its failure message is `"SCRIPTS: AddItem in
/// script '%s' failed to generate an item."`): parses an item form, a count,
/// a float and an integer. With a `thisObj`, builds a `TESContainer` (12
/// bytes) on the stack and fills it from the item, by form type: type `0x34`
/// adds the leveled items of `item + 0x30` for the reference's calculated
/// level; type `0x55` adds every entry of its list that passes the virtual
/// test `0xe4` (needs a non-zero count); any other item that passes the test
/// is added directly. When nothing applies the logging stub gets the message
/// with the script's name. The container then gets the float
/// (`00482090`) and is applied to `thisObj` (`004821a0(thisObj, integer ==
/// 1)`), then destroyed. Returns the parse result. The exception-unwinding
/// frame is not translated.
pub fn fn_005d8e80(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([item, count, quality, flag]) = parse_params(e, a, [0, 0, 0, 0]) else {
        return false;
    };
    if a.this_obj.is_null() {
        return true;
    }
    let level = e.call(GET_CALC_LEVEL, &args![a.this_obj, 0u32]).u32();
    e.with_stack(12, |e, container| {
        e.call(CONTAINER_CONSTRUCT, &args![container]);
        let mut leveled = 0;
        let mut list_form = 0;
        let mut single = 0;
        let form_type = e.call(FORM_TYPE, &args![item]).u32();
        if form_type == 0x34 {
            leveled = item;
        } else if form_type == 0x55 {
            list_form = item;
        } else if e.vcall(item, FORM_TEST_SLOT, &args![]).bool() {
            single = item;
        }
        if leveled != 0 {
            e.call(
                FORM_ADD_LEVELED_ITEMS,
                &args![
                    leveled + 0x30,
                    level & 0xffff,
                    count & 0xffff,
                    container,
                    0u32
                ],
            );
        } else if list_form != 0 && count != 0 {
            let mut node = e.call(LIST_AT_18, &args![list_form]).u32();
            while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
                let data = e.call(LIST_NODE_DATA, &args![node]).u32();
                let entry = e.mem.u32(data);
                node = e.call(LIST_NEXT_NODE, &args![node]).u32();
                if e.vcall(entry, FORM_TEST_SLOT, &args![]).bool() {
                    e.call(CONTAINER_ADD_ITEM, &args![container, entry, count, 0u32]);
                }
            }
        } else if single != 0 && count != 0 {
            e.call(CONTAINER_ADD_ITEM, &args![container, single, count, 0u32]);
        } else {
            let name = e
                .vcall(a.script_obj.addr(), SCRIPT_NAME_SLOT, &args![])
                .u32();
            e.call(LOG_STUB, &args![MSG_ADD_ITEM_FAILED, name]);
        }
        e.call(CONTAINER_FN_00482090, &args![container, quality]);
        e.call(
            CONTAINER_FN_004821A0,
            &args![container, a.this_obj, (flag == 1) as u32],
        );
        e.call(CONTAINER_DESTRUCT, &args![container]);
    });
    true
}

// Translated from 005d90a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function command: the result double starts at 0.0; parses a form, a
/// float (default 1.0), an integer (default 1) and two more integers, then
/// calls `005c4b30(thisObj, form, integer, integer2, integer3, float)`. When
/// that returns a form, its form id is stored into the result double
/// (`PutNumericIDInDouble`). Returns the parse result.
pub fn fn_005d90a0(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let one = 1.0f32.to_bits();
    let Some([form, scale, first, second, third]) = parse_params(e, a, [0, one, 1, 0, 0]) else {
        return false;
    };
    let made = e
        .call(
            SCRIPT_BODY_005C4B30,
            &args![a.this_obj, form, first, second, third, scale],
        )
        .u32();
    if made != 0 {
        let id = e.call(GET_FORM_ID, &args![made]).u32();
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), id);
            e.call(PUT_NUMERIC_ID_IN_DOUBLE, &args![cell, a.result]);
        });
    }
    true
}

// Translated from 005d9160 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command that closes every menu through
/// `Interface::EmergencyCloseAllMenusAndBreakStuff` (Xbox PDB) and succeeds.
/// None of the eight words is read.
pub fn fn_005d9160(e: &mut Engine, _unused_args: ScriptArgs) -> bool {
    e.call(EMERGENCY_CLOSE_ALL_MENUS, &args![]);
    true
}

// Translated from 005d9170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetConsoleOutputFileFunction` (Xbox PDB): parses a text (a
/// 264-byte local). A text that is empty or starts with `'0'` clears the
/// console's output file name (echoed as `"SetConsoleOutputFilename >>
/// OUTPUT_DISABLED"` before it is cleared); any other text becomes the file
/// name (echoed as `"... >> '%s'"` afterwards). Returns the parse result.
pub fn script_set_console_output_file_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let text = e.mem.alloc(0x108);
    let ok = parse(e, a, &[text]);
    if ok {
        let first = e.mem.i8(text);
        if first == 0 || first == b'0' as i8 {
            if echo_enabled(e) {
                console_print(e, &args![MSG_CONSOLE_OUTPUT_DISABLED]);
            }
            let console = e.call(MENU_CONSOLE_INSTANCE, &args![1u32]).u32();
            e.call(CONSOLE_SET_OUTPUT_FILE, &args![console, 0u32]);
        } else {
            let console = e.call(MENU_CONSOLE_INSTANCE, &args![1u32]).u32();
            e.call(CONSOLE_SET_OUTPUT_FILE, &args![console, text]);
            if echo_enabled(e) {
                console_print(e, &args![MSG_CONSOLE_OUTPUT_FILE, text]);
            }
        }
    }
    e.mem.free(text);
    ok
}

/// The commands that parse one word and, with a `thisObj` and a non-zero
/// word, return a condition function's answer for it (otherwise they
/// succeed): `005d9270` and `005d92e0`.
fn parsed_condition_with_reference(e: &mut Engine, a: ScriptArgs, function: u32) -> bool {
    let Some([argument]) = parse_params(e, a, [0]) else {
        return false;
    };
    if a.this_obj.is_null() || argument == 0 {
        return true;
    }
    call_condition(e, function, a, argument)
}

// Translated from 005d9270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetFactionRelationFunction` (Xbox PDB): parses a form; without a
/// `thisObj` or with a null form the command succeeds, otherwise it returns
/// the `AL` of `GetFactionRelationConditionFunction(thisObj, form, 0,
/// result)`.
pub fn script_get_faction_relation_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parsed_condition_with_reference(e, a, GET_FACTION_RELATION_CONDITION)
}

// Translated from 005d92e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsLastPlayedIdleFunction` (Xbox PDB): like
/// [`script_get_faction_relation_function`] with
/// `IsLastPlayedIdleConditionFunction`.
pub fn script_is_last_played_idle_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parsed_condition_with_reference(e, a, IS_LAST_PLAYED_IDLE_CONDITION)
}

// Translated from 005d9350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `Interface::CreateRaceSexMenu(0)` (Xbox PDB) and succeeds. None of
/// the eight words is read.
pub fn fn_005d9350(e: &mut Engine, _unused_args: ScriptArgs) -> bool {
    e.call(CREATE_RACE_SEX_MENU, &args![0u32]);
    true
}

// Translated from 005d9370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetPlayerTeammateFunction` (Xbox PDB): parses an integer. When
/// `thisObj` exists and its virtual test (slot `0x100`) holds, calls
/// `Actor::SetPlayerTeammate(thisObj, integer != 0)`. Succeeds once the
/// parameters parse.
pub fn script_set_player_teammate_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null()
        && e.vcall(a.this_obj.addr(), REFERENCE_TEST_SLOT, &args![])
            .bool()
    {
        e.call(
            ACTOR_SET_PLAYER_TEAMMATE,
            &args![a.this_obj, (flag != 0) as u32],
        );
    }
    true
}

/// A command that is a condition function with no argument: returns the
/// `AL` of `function(thisObj, 0, 0, result)`.
fn condition_without_argument(e: &mut Engine, a: ScriptArgs, function: u32) -> bool {
    call_condition(e, function, a, 0)
}

// Translated from 005d93f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `GetPlayerTeammateConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005d93f0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, GET_PLAYER_TEAMMATE_CONDITION)
}

// Translated from 005d9410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `GetPlayerTeammateCountConditionFunction(thisObj, 0,
/// 0, result)`.
pub fn fn_005d9410(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, GET_PLAYER_TEAMMATE_COUNT_CONDITION)
}

// Translated from 005d9430 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a `thisObj` that passes the virtual test (slot `0x100`): parses an
/// integer (the parse result is ignored; the local keeps its 0 when the
/// parse fails) and, when the byte `ACTOR_FORCE_NEXT_UPDATE` reads is set or
/// the integer is non-zero, queues the menu creation
/// `QueueMenuCreate(1, thisObj, 0, 0, 3, 0)`. Always succeeds.
pub fn fn_005d9430(e: &mut Engine, a: ScriptArgs) -> bool {
    if !a.this_obj.is_null()
        && e.vcall(a.this_obj.addr(), REFERENCE_TEST_SLOT, &args![])
            .bool()
    {
        let [flag] = parse_values(e, a, [0]);
        if e.call(ACTOR_FORCE_NEXT_UPDATE, &args![a.this_obj]).bool() || flag != 0 {
            e.call(
                QUEUE_MENU_CREATE,
                &args![1u32, a.this_obj, 0u32, 0u32, 3u32, 0u32],
            );
        }
    }
    true
}

// Translated from 005d94c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `GetMinorCrimeCountConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005d94c0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, GET_MINOR_CRIME_COUNT_CONDITION)
}

// Translated from 005d94e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `GetMajorCrimeCountConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005d94e0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, GET_MAJOR_CRIME_COUNT_CONDITION)
}

// Translated from 005d9500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a value and calls `0047eb90(value, 0)` on it (a `thiscall` whose
/// `this` is the parsed word). Returns the parse result.
pub fn fn_005d9500(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    e.call(FLAG_SETTER_0047EB90, &args![value, 0u32]);
    true
}

// Translated from 005d9550 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a `thisObj` that passes the virtual test (slot `0x100`), calls
/// `008bcb40(thisObj)`. Always succeeds; none of the other words is read.
pub fn fn_005d9550(e: &mut Engine, a: ScriptArgs) -> bool {
    if !a.this_obj.is_null()
        && e.vcall(a.this_obj.addr(), REFERENCE_TEST_SLOT, &args![])
            .bool()
    {
        e.call(ACTOR_FN_008BCB40, &args![a.this_obj]);
    }
    true
}

// Translated from 005d9590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `GetActorCrimePlayerEnemyConditionFunction(thisObj,
/// 0, 0, result)`.
pub fn fn_005d9590(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, GET_ACTOR_CRIME_PLAYER_ENEMY_CONDITION)
}

// Translated from 005d95b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `GetActorFactionPlayerEnemyConditionFunction(thisObj,
/// 0, 0, result)`.
pub fn fn_005d95b0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, GET_ACTOR_FACTION_PLAYER_ENEMY_CONDITION)
}

// Translated from 005d95d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetPlayerTagSkillFunction` (Xbox PDB): parses a skill value
/// (default -1) and an index (default 0). When `0047f060(skill)` holds
/// (`0x20 <= skill < 0x2e`) and `0 <= index < 4` (`005d9660`, signed), sets
/// the tag skill `index` of the player's class to the skill
/// (`TESClass::SetTagSkill`). Succeeds once the parameters parse.
pub fn script_set_player_tag_skill_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([skill, index]) = parse_params(e, a, [u32::MAX, 0]) else {
        return false;
    };
    if e.call(VALUE_IN_RANGE_20_TO_2D, &args![skill]).bool()
        && (index as i32) >= 0
        && (index as i32) < fn_005d9660(e) as i32
    {
        let player = player(e);
        let class = e.call(ACTOR_GET_CLASS, &args![player]).u32();
        e.call(CLASS_SET_TAG_SKILL, &args![class, index, skill]);
    }
    true
}

// Translated from 005d9660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 4 (the number of tag skills; `cdecl`, no arguments).
pub fn fn_005d9660(_e: &mut Engine) -> u32 {
    4
}

// Translated from 005d9670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPlayerTagSkillFunction` (Xbox PDB): parses a skill value
/// (default -1). The result double is 0.0, or 1.0 when `0047f060(skill)`
/// holds and the player's class (`Actor::GetClass`) answers true to
/// `005a5f40(skill)`; echoed as `"IsPlayerTagSkill >> %.0f"`. Succeeds once
/// the parameters parse (a failed parse leaves the result untouched).
pub fn script_is_player_tag_skill_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([skill]) = parse_params(e, a, [u32::MAX]) else {
        return false;
    };
    e.mem.set_f64(a.result.addr(), 0.0);
    if e.call(VALUE_IN_RANGE_20_TO_2D, &args![skill]).bool() {
        let player = player(e);
        let class = e.call(ACTOR_GET_CLASS, &args![player]).u32();
        if e.call(CLASS_IS_TAG_SKILL, &args![class, skill]).bool() {
            e.mem.set_f64(a.result.addr(), 1.0);
        }
    }
    if echo_enabled(e) {
        let shown = e.mem.f64(a.result.addr());
        console_print(e, &args![MSG_IS_PLAYER_TAG_SKILL, shown]);
    }
    true
}

// Translated from 005d9730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPlayerGrabbedRefFunction` (Xbox PDB): the result double is
/// 0.0, or the numeric form id of the player's grabbed reference
/// (`PutNumericIDInDouble`) when there is one; echoed as
/// `"GetPlayerGrabbedRef >> (%08x)"` with the form id (0 without one).
/// Always succeeds.
pub fn script_get_player_grabbed_ref_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let mut id = 0;
    let player_object = player(e);
    if e.call(PLAYER_GRABBED_REF, &args![player_object]).u32() != 0 {
        let player_object = player(e);
        let grabbed = e.call(PLAYER_GRABBED_REF, &args![player_object]).u32();
        id = e.call(GET_FORM_ID, &args![grabbed]).u32();
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), id);
            e.call(PUT_NUMERIC_ID_IN_DOUBLE, &args![cell, a.result]);
        });
    }
    if echo_enabled(e) {
        console_print(e, &args![MSG_GET_PLAYER_GRABBED_REF, id]);
    }
    true
}

// Translated from 005d97b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a value (default 0) and returns the `AL` of
/// `005a4c20(thisObj, value, 0, result)`, a condition function.
pub fn fn_005d97b0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, CONDITION_FN_005A4C20, a, value)
}

// Translated from 005d9810 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function command that makes a new actor from an actor base form
/// (parameters: base form, count default 4, encounter zone). The result
/// double starts at 0.0. With a base form that passes the virtual test `0xf8`
/// and a `thisObj`, a form of type `0x2a` is built as a `Character`
/// (`0x1c8` bytes) and one of type `0x2b` as a `Creature` (`0x1c0` bytes);
/// any other type makes nothing. The new reference gets the encounter zone
/// (when given), the count in its extra data list, the flags `0x40000000`
/// (virtual slot `0x48`), the parent cell of `thisObj` (slot `0x228`) and the
/// base form (`SetObjectReference`); the actor base data of `form + 0x30` is
/// applied when its test holds. It is then placed through the data handler
/// (`004698a0`) using `thisObj`'s parent cell (dropped when bit 0 of its flag
/// byte is clear), world space and virtual slot `0x1f4`, and its numeric form
/// id goes to the result double. Returns the parse result. The
/// exception-unwinding frame is not translated.
pub fn fn_005d9810(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([form, count, zone]) = parse_params(e, a, [0, 4, 0]) else {
        return false;
    };
    if form == 0 || !e.vcall(form, FORM_IS_ACTOR_BASE_SLOT, &args![]).bool() || a.this_obj.is_null()
    {
        return true;
    }
    let this_obj = a.this_obj.addr();
    let form_type = e.call(FORM_TYPE, &args![form]).u32();
    let (size, construct) = match form_type {
        0x2a => (0x1c8u32, CHARACTER_CONSTRUCT),
        0x2b => (0x1c0u32, CREATURE_CONSTRUCT),
        _ => return true,
    };
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    let made = if block == 0 {
        0
    } else {
        e.call(construct, &args![block]).u32()
    };
    if made == 0 {
        return true;
    }
    if zone != 0 {
        e.call(REFERENCE_SET_ENCOUNTER_ZONE, &args![made, zone]);
    }
    let list = e.call(EXTRA_DATA_LIST, &args![made]).u32();
    e.call(EXTRA_LIST_SET_COUNT, &args![list, count]);
    e.vcall(made, FORM_SLOT_48, &args![0x4000_0000u32]);
    let parent_cell = e.call(REFERENCE_PARENT_CELL, &args![this_obj]).u32();
    e.vcall(made, ACTOR_SLOT_228, &args![parent_cell]);
    e.call(REFERENCE_SET_OBJECT_REFERENCE, &args![made, form]);
    if e.call(ACTOR_BASE_DATA_TEST, &args![form + 0x30]).bool() {
        e.call(ACTOR_BASE_DATA_APPLY, &args![form + 0x30, made]);
    }
    let mut cell = e.call(REFERENCE_PARENT_CELL, &args![this_obj]).u32();
    if cell != 0 && !e.call(CELL_FLAG_BIT_0, &args![cell]).bool() {
        cell = 0;
    }
    let world_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![this_obj]).u32();
    let field = e.call(REFERENCE_FIELD_24_ADDRESS, &args![this_obj]).u32();
    let word = e.vcall(this_obj, REFERENCE_SLOT_1F4, &args![]).u32();
    let saved = e.call(WORD_AT_20, &args![made]).u32();
    let data_handler = e.global::<u32>(LIST_OWNER);
    e.call(
        DATA_HANDLER_FN_004698A0,
        &args![
            data_handler,
            saved,
            word,
            field,
            cell,
            world_space,
            made,
            0u32,
            0u32
        ],
    );
    e.call(DATA_HANDLER_FN_0046A010, &args![made, 0u32]);
    let id = e.call(GET_FORM_ID, &args![made]).u32();
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), id);
        e.call(PUT_NUMERIC_ID_IN_DOUBLE, &args![cell, a.result]);
    });
    true
}

// Translated from 005d9aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `GetDestructionStageConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005d9aa0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, GET_DESTRUCTION_STAGE_CONDITION)
}

// Translated from 005d9ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ForceActiveQuestFunction` (Xbox PDB): parses a quest and, when
/// it is not null, makes it the player's active quest
/// (`PlayerCharacter::SetActiveQuest`). Returns the parse result.
pub fn script_force_active_quest_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([quest]) = parse_params(e, a, [0]) else {
        return false;
    };
    if quest != 0 {
        let player = player(e);
        e.call(PLAYER_SET_ACTIVE_QUEST, &args![player, quest]);
    }
    true
}

// Translated from 005d9b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses two integers. When the signed word that `0043d4d0` returns the
/// address of for the object at `011c3ea4` is above 1, sends the task queue
/// the message `0x11df` with `thisObj` and the two integers; otherwise runs
/// [`fn_005d9bc0`] with them directly. Returns the parse result.
pub fn fn_005d9b20(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, second]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    let mode = e.call(FN_0043D4D0, &args![GAME_MODE_OBJECT]).u32();
    if (e.mem.u32(mode) as i32) > 1 {
        let queue = e.call(TASK_QUEUE_INTERFACE, &args![]).u32();
        e.call(
            TASK_QUEUE_SEND,
            &args![queue, TASK_MESSAGE_11DF, a.this_obj, first, second],
        );
    } else {
        fn_005d9bc0(e, a.this_obj, Ptr::new(first), second as i32);
    }
    true
}

/// `__RTDynamicCast` of the object `object + 0x20` holds a pointer to (the
/// `TESBoundObject` of a form) to `TESNPC`.
fn cast_to_npc(e: &mut Engine, object: Ptr) -> Ptr {
    let bound = e.call(WORD_AT_20, &args![object]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![bound, 0u32, RTTI_TES_BOUND_OBJECT, RTTI_TES_NPC, 0u32],
    )
    .ptr()
}

// Translated from 005d9bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Blends the face-gen coordinates of the NPC behind `source` into those of
/// the NPC behind `actor` by `percent / 100` (`cdecl`). Nothing happens when
/// either is null. The steps, in the game's order: both are cast to `TESNPC`
/// through the word at `+0x20`; two coordinate arrays (4 blocks of `0x20`
/// bytes) and a list are built on the stack; the list is filled with the NPCs
/// of the target's race and sex (`00603280`) and the first one is taken; an
/// array already set on the target NPC (`+0x1b4`) is destroyed and cleared;
/// the coordinate sets of the target, of the source and of the chosen NPC
/// feed the face-gen routines (`00652e00`, `00652440`, `00653000`,
/// `00652af0`, `00652470`); a fresh counted array of 4 blocks (`0x84` bytes,
/// count 4 first) receives the result (`00652e00`, `CopyFaceGenCoord`) and
/// is stored on the target NPC; the NPC's virtual slot `0x48` gets `0x800`
/// and the actor, when its slot `0x1d0` answers non-zero, has its 3D reset.
/// The list and the arrays are then destroyed. The exception-unwinding frame
/// is not translated.
pub fn fn_005d9bc0(e: &mut Engine, actor: Ptr, source: Ptr, percent: i32) {
    if actor.is_null() || source.is_null() {
        return;
    }
    let fraction = (percent as f64 / e.global::<f64>(HUNDRED)) as f32;
    let source_npc = cast_to_npc(e, source);
    let actor_npc = cast_to_npc(e, actor);
    let blocks_a = e.mem.alloc(0x80);
    let blocks_b = e.mem.alloc(0x80);
    let list = e.mem.alloc(0x14);
    for blocks in [blocks_a, blocks_b] {
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![
                blocks,
                0x20u32,
                4u32,
                COORD_BLOCK_CONSTRUCT,
                COORD_BLOCK_DESTRUCT
            ],
        );
    }
    e.call(COORD_LIST_CONSTRUCT, &args![list]);
    let sex = e.call(ACTOR_BASE_GET_SEX, &args![source_npc]).u32();
    let race = e.call(ACTOR_BASE_RACE, &args![source_npc]).u32();
    e.call(COLLECT_MATCHING_NPCS, &args![race, sex, list]);
    let mut chosen = 0;
    let first = e.call(ARRAY_ELEMENT_ADDRESS, &args![list, 0u32]).u32();
    if e.mem.u32(first) != 0 {
        let first = e.call(ARRAY_ELEMENT_ADDRESS, &args![list, 0u32]).u32();
        chosen = e.mem.u32(first);
    }
    let old = fn_005d9f90(e, actor_npc);
    if old != 0 {
        fn_005d9ff0(e, Ptr::new(old), 3);
        fn_005d9f70(e, actor_npc, 0);
    }
    let actor_coords = e.call(NPC_COORDS, &args![actor_npc]).u32();
    let source_coords = e.call(NPC_COORDS, &args![source_npc]).u32();
    let chosen_coords = e.call(NPC_COORDS, &args![chosen]).u32();
    e.call(
        FACEGEN_FN_00652E00,
        &args![chosen_coords, source_coords, blocks_b, 0u32],
    );
    let scale = e
        .call(FACEGEN_FN_00652440, &args![actor_coords, 0u32, 0u32])
        .f32();
    e.call(INIT_FACEGEN_COORD, &args![blocks_a]);
    e.call(FACEGEN_FN_00653000, &args![fraction, blocks_b, 1u32]);
    e.call(
        FACEGEN_FN_00652AF0,
        &args![actor_coords, blocks_b, blocks_a, 1u32, 0.0f32],
    );
    e.call(FACEGEN_FN_00652470, &args![blocks_a, 0u32, 0u32, scale]);
    let raw = e.call(OPERATOR_NEW, &args![0x84u32]).u32();
    let copy = if raw == 0 {
        0
    } else {
        e.mem.set_u32(raw, 4);
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![
                raw + 4,
                0x20u32,
                4u32,
                COORD_BLOCK_CONSTRUCT,
                COORD_BLOCK_DESTRUCT
            ],
        );
        raw + 4
    };
    e.call(INIT_FACEGEN_COORD, &args![copy]);
    let race_data = e.call(ACTOR_BASE_RACE, &args![actor_npc]).u32();
    let destination = fn_005d9fb0(e, Ptr::new(race_data), 0);
    e.call(
        FACEGEN_FN_00652E00,
        &args![destination, blocks_a, copy, 0u32],
    );
    e.call(COPY_FACEGEN_COORD, &args![actor_coords, copy, 0u32, 1u32]);
    fn_005d9f70(e, actor_npc, copy);
    e.vcall(actor_npc.addr(), FORM_SLOT_48, &args![0x800u32]);
    if e.vcall(actor.addr(), REFERENCE_SLOT_1D0, &args![]).u32() != 0 {
        e.call(CHARACTER_RESET_3D, &args![actor]);
    }
    e.call(COORD_LIST_DESTRUCT, &args![list]);
    for blocks in [blocks_b, blocks_a] {
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &args![blocks, 0x20u32, 4u32, COORD_BLOCK_DESTRUCT],
        );
    }
    e.mem.free(list);
    e.mem.free(blocks_b);
    e.mem.free(blocks_a);
}

// Translated from 005d9f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `this + 0x1b4` of an NPC: the pointer to
/// its own counted array of face-gen coordinate blocks (0 for none).
pub fn fn_005d9f70(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x1b4, value);
}

// Translated from 005d9f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x1b4` of an NPC (see [`fn_005d9f70`]).
pub fn fn_005d9f90(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x1b4)
}

// Translated from 005d9fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of one of two sub-objects of `this`: `this + 0x478` when
/// `flag` is 0, else `this + 0x3f8`.
pub fn fn_005d9fb0(_e: &mut Engine, this: Ptr, flag: u32) -> u32 {
    if flag == 0 {
        this.addr() + 0x478
    } else {
        this.addr() + 0x3f8
    }
}

// Translated from 005d9ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `logic_error::`vector deleting destructor'` (the library name Ghidra gives
/// it), here the one for an array of `0x20`-byte coordinate blocks: with bit 1
/// of `flags`, `this` is the first element of a counted array (the count
/// sits at `this - 4`): every element is destroyed
/// (`__eh_vector_destructor_iterator__`), the block `this - 4` is freed when
/// bit 0 is set, and `this - 4` is returned. Without bit 1 the single object
/// is destroyed (`00449680`), freed when bit 0 is set, and `this` is
/// returned.
pub fn fn_005d9ff0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 2 != 0 {
        let count = e.mem.u32(this.addr() - 4);
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &args![this, 0x20u32, count, COORD_BLOCK_DESTRUCT],
        );
        if flags & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![this.addr() - 4]);
        }
        Ptr::new(this.addr() - 4)
    } else {
        e.call(COORD_BLOCK_DESTRUCT, &args![this]);
        if flags & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![this]);
        }
        this
    }
}

// Translated from 005da060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsUsedItemEquipTypeFunction` (Xbox PDB): parses an integer
/// (default -1) and returns the `AL` of
/// `GetIsUsedItemEquipTypeConditionFunction(thisObj, value, 0, result)`.
pub fn script_get_is_used_item_equip_type_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [u32::MAX]) else {
        return false;
    };
    call_condition(e, GET_IS_USED_ITEM_EQUIP_TYPE_CONDITION, a, value)
}

// Translated from 005da0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetThreatRatioFunction` (Xbox PDB): parses a reference (default
/// 0) and returns the `AL` of `GetThreatRatioConditionFunction(thisObj,
/// reference, 0, result)`.
pub fn script_get_threat_ratio_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([reference]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, GET_THREAT_RATIO_CONDITION, a, reference)
}

// Translated from 005da120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsAlignmentFunction` (Xbox PDB): parses an integer (default
/// 1) and returns the `AL` of `GetIsAlignmentConditionFunction(thisObj,
/// value, 0, result)`.
pub fn script_get_is_alignment_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [1]) else {
        return false;
    };
    call_condition(e, GET_IS_ALIGNMENT_CONDITION, a, value)
}

// Translated from 005da180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a form and, when it is not null, calls `0060d720` on it. Returns
/// the parse result.
pub fn fn_005da180(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([form]) = parse_params(e, a, [0]) else {
        return false;
    };
    if form != 0 {
        e.call(FN_0060D720, &args![form]);
    }
    true
}

// Translated from 005da1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an object (default 0) and a float (default 0.0). With an object,
/// calls `00444020(object, float)` and its virtual slot `0x48` with 4.
/// Returns the parse result.
pub fn fn_005da1e0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([object, amount]) = parse_params(e, a, [0, 0.0f32.to_bits()]) else {
        return false;
    };
    if object != 0 {
        e.call(FN_00444020, &args![object, amount]);
        e.vcall(object, FORM_SLOT_48, &args![4u32]);
    }
    true
}

// Translated from 005da260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00477d10(thisObj)` and succeeds.
pub fn fn_005da260(e: &mut Engine, a: ScriptArgs) -> bool {
    e.call(FN_00477D10, &args![a.this_obj]);
    true
}

// Translated from 005da280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `GetAggroRadiusViolatedConditionFunction(thisObj, 0,
/// 0, result)`.
pub fn fn_005da280(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, GET_AGGRO_RADIUS_VIOLATED_CONDITION)
}

/// The tail the alarm paths of `005da2a0` share: `Actor::AttackAlarm` on
/// `alarmed`, then the 13-word virtual call (slot `0x33c`) of the process of
/// `process_owner` with `target` and the player.
fn raise_alarm(e: &mut Engine, alarmed: u32, process_owner: u32, target: u32) {
    let player = player(e);
    e.call(ACTOR_ATTACK_ALARM, &args![alarmed, player, 0u32, 1u32]);
    let process = e.call(GET_PROCESS, &args![process_owner]).u32();
    e.vcall(
        process,
        PROCESS_SLOT_33C,
        &args![target, player, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
}

// Translated from 005da2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command run on an actor (`thisObj`, virtual test `0x100`; the parse
/// result is ignored): parses an actor and an id. Without either, `thisObj`
/// raises the alarm against the player. Otherwise an actor that is not the
/// player and has a process raises it; failing that, the id is looked up in
/// the process lists: a found reference that fails the virtual test `0x21c`
/// raises it, one that passes it hands it to the owner's actor (found by the
/// owner's form type 8 through the id lookup, else `GetActorRefInHigh`).
/// Always succeeds.
pub fn fn_005da2a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let [target, id] = parse_values(e, a, [0, 0]);
    let this_obj = a.this_obj.addr();
    if !e.vcall(this_obj, REFERENCE_TEST_SLOT, &args![]).bool() {
        return true;
    }
    let player = player(e);
    if target == 0 && id == 0 {
        raise_alarm(e, this_obj, this_obj, this_obj);
        return true;
    }
    if target != 0 && target != player && e.call(GET_PROCESS, &args![target]).u32() != 0 {
        raise_alarm(e, target, target, target);
        return true;
    }
    if id != 0 {
        let found = e
            .call(PROCESS_LISTS_FIND, &args![PROCESS_LISTS, id, 1u32, 1u32])
            .u32();
        if found != 0 {
            if !e.vcall(found, REFERENCE_SLOT_21C, &args![]).bool() {
                raise_alarm(e, found, found, found);
            } else if e.call(REFERENCE_GET_OWNER, &args![found]).u32() != 0 {
                let owner = e.call(REFERENCE_GET_OWNER, &args![found]).u32();
                let owner_actor = if e.call(FORM_TYPE, &args![owner]).u32() == 8 {
                    e.call(PROCESS_LISTS_FIND, &args![PROCESS_LISTS, owner, 0u32, 1u32])
                        .u32()
                } else {
                    e.call(
                        PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH,
                        &args![PROCESS_LISTS, owner, 0u32],
                    )
                    .u32()
                };
                if owner_actor != 0 {
                    raise_alarm(e, owner_actor, found, found);
                }
            }
        }
    }
    true
}

// Translated from 005da540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Closes the console (`Interface::CloseConsole`) and queues the creation of
/// menu 9 for the player: `QueueMenuCreate(9, player, 0, 0, 1, 0)`. Always
/// succeeds; none of the eight words is read.
pub fn fn_005da540(e: &mut Engine, _unused_args: ScriptArgs) -> bool {
    e.call(CLOSE_CONSOLE, &args![]);
    let player = player(e);
    e.call(
        QUEUE_MENU_CREATE,
        &args![9u32, player, 0u32, 0u32, 1u32, 0u32],
    );
    true
}

// Translated from 005da570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::FireWeaponFunction` (Xbox PDB): parses a weapon form. With a
/// `thisObj`: a form of type `0x28` is fired, through the task queue
/// (`QueueWeaponFire(weapon, thisObj)`) when `008c7aa0` holds, else directly
/// (`00523150(weapon, thisObj)`); anything else logs the message `"SCRIPTS:
/// FireWeapon in script '%s' called with non-weapon parameter."` with the
/// script's name (virtual slot `0x130`). Succeeds once the parameters parse.
pub fn script_fire_weapon_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([weapon]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        if weapon != 0 && e.call(FORM_TYPE, &args![weapon]).u32() == 0x28 {
            if e.call(FN_008C7AA0, &args![]).bool() {
                let queue = e.call(TASK_QUEUE_INTERFACE, &args![]).u32();
                e.call(QUEUE_WEAPON_FIRE, &args![queue, weapon, a.this_obj]);
            } else {
                e.call(WEAPON_FN_00523150, &args![weapon, a.this_obj]);
            }
        } else {
            let name = e
                .vcall(a.script_obj.addr(), SCRIPT_NAME_SLOT, &args![])
                .u32();
            e.call(LOG_STUB, &args![MSG_FIRE_WEAPON_NON_WEAPON, name]);
        }
    }
    true
}

// Translated from 005da630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowTutorialMenu` (Xbox PDB): when `005d4a40` holds (it always
/// returns 1), parses an integer (the parse result is ignored; the local
/// keeps 0 when it fails) and creates the tutorial menu for it
/// (`TutorialMenu::Create(value, 0)`). Always succeeds.
pub fn script_show_tutorial_menu(e: &mut Engine, a: ScriptArgs) -> bool {
    if e.call(ALWAYS_TRUE, &args![]).bool() {
        let [value] = parse_values(e, a, [0]);
        e.call(TUTORIAL_MENU_CREATE, &args![value, 0u32]);
    }
    true
}

// Translated from 005da690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a signed integer (default 0) and moves a selection along the chain
/// of an actor's base form. With a non-zero count and a `thisObj` whose
/// virtual test `0x218` holds, the chain starts at the node `00726070` gives
/// for `base + 0x10c` (`base` being the word `007af430` gives for `thisObj`):
/// a negative count follows [`fn_005da7d0`] links (adding one per step), a
/// positive count follows [`fn_005da7f0`] links (subtracting one per step),
/// and a missing link ends the walk. When the walk ended elsewhere than the
/// start, `0060b240(base; node, thisObj or 0)` selects that node (the
/// reference is passed only if the virtual test `0x100` holds) and
/// `Character::Reset3D(thisObj)` runs. Succeeds once the parameters parse.
pub fn fn_005da690(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([count]) = parse_params(e, a, [0]) else {
        return false;
    };
    let mut remaining = count as i32;
    let this_obj = a.this_obj.addr();
    if remaining != 0
        && this_obj != 0
        && e.vcall(this_obj, REFERENCE_TEST_SLOT_218, &args![]).bool()
    {
        let base = e.call(WORD_AT_20, &args![this_obj]).u32();
        let start = e.call(LIST_NEXT_NODE, &args![base + 0x10c]).u32();
        let mut node = start;
        while remaining != 0 {
            if remaining < 0 {
                let link = fn_005da7d0(e, Ptr::new(node));
                if link == 0 {
                    remaining = 0;
                } else {
                    node = link;
                    remaining += 1;
                }
            } else {
                let link = fn_005da7f0(e, Ptr::new(node));
                if link == 0 {
                    remaining = 0;
                } else {
                    node = link;
                    remaining -= 1;
                }
            }
        }
        if node != start {
            let reference = if e.vcall(this_obj, REFERENCE_TEST_SLOT, &args![]).bool() {
                this_obj
            } else {
                0
            };
            e.call(NPC_SELECT_LIST_NODE, &args![base, node, reference]);
            e.call(CHARACTER_RESET_3D, &args![this_obj]);
        }
    }
    true
}

// Translated from 005da7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x508` of a chain node (the link [`fn_005da690`]
/// follows for a negative count).
pub fn fn_005da7d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x508)
}

// Translated from 005da7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x504` of a chain node (the link [`fn_005da690`]
/// follows for a positive count).
pub fn fn_005da7f0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x504)
}

// Translated from 005da810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a reference (default 0) and copies its selection position onto
/// `thisObj`'s. Both references need a non-zero value and a true virtual
/// test `0x218` (tested in that order), and the chain starts of their base
/// forms (`00726070(base + 0x10c)`) must differ. Then the number of links
/// `thisObj`'s start has through [`fn_005da7d0`] is counted, the source's
/// start is walked to the end of the same links and back by that many
/// [`fn_005da7f0`] links (fewer if the chain ends), the face-gen body
/// textures of `thisObj`'s base are cleared
/// (`BSFaceGenManager::ClearBodyTexturesFromPalette`), the node reached is
/// selected for `thisObj`'s base (`0060b240(base; node, 0)`) and
/// `Character::Reset3D(thisObj)` runs. Succeeds once the parameters parse.
pub fn fn_005da810(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([source]) = parse_params(e, a, [0]) else {
        return false;
    };
    let this_obj = a.this_obj.addr();
    if source != 0
        && e.vcall(source, REFERENCE_TEST_SLOT_218, &args![]).bool()
        && this_obj != 0
        && e.vcall(this_obj, REFERENCE_TEST_SLOT_218, &args![]).bool()
    {
        let target_base = e.call(WORD_AT_20, &args![this_obj]).u32();
        let source_base = e.call(WORD_AT_20, &args![source]).u32();
        let target_start = e.call(LIST_NEXT_NODE, &args![target_base + 0x10c]).u32();
        let source_start = e.call(LIST_NEXT_NODE, &args![source_base + 0x10c]).u32();
        if source_start != target_start {
            let mut steps = 0u32;
            let mut target_node = target_start;
            loop {
                let link = fn_005da7d0(e, Ptr::new(target_node));
                if link == 0 {
                    break;
                }
                target_node = link;
                steps += 1;
            }
            let mut source_node = source_start;
            loop {
                let link = fn_005da7d0(e, Ptr::new(source_node));
                if link == 0 {
                    break;
                }
                source_node = link;
            }
            loop {
                let link = fn_005da7f0(e, Ptr::new(source_node));
                if link == 0 || steps == 0 {
                    break;
                }
                source_node = link;
                steps -= 1;
            }
            e.call(CLEAR_BODY_TEXTURES_FROM_PALETTE, &args![target_base]);
            e.call(NPC_SELECT_LIST_NODE, &args![target_base, source_node, 0u32]);
            e.call(CHARACTER_RESET_3D, &args![this_obj]);
        }
    }
    true
}

// Translated from 005da980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer (default 0) and calls `00969820(player, integer != 0)`
/// (`playercharacter.cpp`). Returns the parse result.
pub fn fn_005da980(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag]) = parse_params(e, a, [0]) else {
        return false;
    };
    let player = player(e);
    e.call(PLAYER_FN_00969820, &args![player, (flag != 0) as u32]);
    true
}

// Translated from 005da9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_005da980`] with `009697c0`.
pub fn fn_005da9e0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag]) = parse_params(e, a, [0]) else {
        return false;
    };
    let player = player(e);
    e.call(PLAYER_FN_009697C0, &args![player, (flag != 0) as u32]);
    true
}

/// The commands that parse one integer (default 0) and return a condition
/// function's `AL` for `(thisObj, integer, 0, result)`; `false` when the
/// parameters do not parse.
fn parsed_condition(e: &mut Engine, a: ScriptArgs, function: u32) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, function, a, value)
}

// Translated from 005daa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetConcussedFunction` (Xbox PDB): parses an integer (default 0)
/// and returns the `AL` of `GetConcussedConditionFunction(thisObj, value, 0,
/// result)`.
pub fn script_get_concussed_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parsed_condition(e, a, GET_CONCUSSED_CONDITION)
}

// Translated from 005daaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `GetRadiationLevelConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005daaa0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, GET_RADIATION_LEVEL_CONDITION)
}

// Translated from 005daac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `GetMapMarkerVisibleConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005daac0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, GET_MAP_MARKER_VISIBLE_CONDITION)
}

// Translated from 005daae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer (default 0). When `thisObj` has bit `0x1000000` of its
/// flags (`00452370`), looks up the destructible data of its base form
/// (`GetDestructionForm(007af430(thisObj))`); the byte `00576100` gives for
/// it (0 without data) is compared with `integer != 0`, and
/// `004846e0(thisObj, 0)` is called when they are equal, `004846e0(thisObj,
/// 1)` when they differ. Succeeds once the parameters parse.
pub fn fn_005daae0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag]) = parse_params(e, a, [0]) else {
        return false;
    };
    let this_obj = a.this_obj.addr();
    if e.call(FORM_HAS_FLAG_1000000, &args![this_obj]).bool() {
        let wanted = (flag != 0) as u8;
        let mut current = 0u8;
        let base = e.call(WORD_AT_20, &args![this_obj]).u32();
        let destruction = e.call(GET_DESTRUCTION_FORM, &args![base]).u32();
        if destruction != 0 {
            current = e.call(DESTRUCTIBLE_FN_00576100, &args![destruction]).u8();
        }
        e.call(
            FORM_FN_004846E0,
            &args![this_obj, (current != wanted) as u32],
        );
    }
    true
}

// Translated from 005dab90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a reference and an integer (both default 0). With a reference,
/// [`fn_005dac10`] sets or clears its bit 0 of the byte at `+0x1e`
/// (`integer == 0` sets it) and its virtual slot `0x48` is called with 2.
/// Succeeds once the parameters parse.
pub fn fn_005dab90(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([reference, flag]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if reference != 0 {
        fn_005dac10(e, Ptr::new(reference), (flag == 0) as u8);
        e.vcall(reference, FORM_SLOT_48, &args![2u32]);
    }
    true
}

// Translated from 005dac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets bit 0 of the byte at `this + 0x1e` when `flag` is non-zero, else
/// clears it.
pub fn fn_005dac10(e: &mut Engine, this: Ptr, flag: u8) {
    let address = this.addr() + 0x1e;
    let byte = e.mem.u8(address);
    e.mem
        .set_u8(address, if flag != 0 { byte | 1 } else { byte & !1 });
}

// Translated from 005dac60 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a `thisObj`, calls its virtual slot `0x208` with 0. Always succeeds;
/// only `thisObj` is read of the eight words.
pub fn fn_005dac60(e: &mut Engine, a: ScriptArgs) -> bool {
    if !a.this_obj.is_null() {
        e.vcall(a.this_obj.addr(), REFERENCE_SLOT_208, &args![0u32]);
    }
    true
}

// Translated from 005dac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Without a `thisObj` it succeeds at once, without parsing. Otherwise it
/// parses a reference, an integer (both default 0) and an integer (default
/// 3). When the reference passes the virtual test `0x100` and has a process
/// (`GetSavedAcquireObject`), `thisObj`'s virtual slot `0x1f4` is called with
/// `(second integer, third integer, thisObj)`; it returns a pointer to three
/// words, which the process's virtual slot `0xfc` receives after the
/// reference: `(reference, word 0, word 1, word 2)`. Succeeds once the
/// parameters parse.
pub fn fn_005dac80(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return true;
    }
    let Some([reference, first, second]) = parse_params(e, a, [0, 0, 3]) else {
        return false;
    };
    if reference != 0
        && e.vcall(reference, REFERENCE_TEST_SLOT, &args![]).bool()
        && e.call(GET_PROCESS, &args![reference]).u32() != 0
    {
        let process = e.call(GET_PROCESS, &args![reference]).u32();
        let point = e
            .vcall(a.this_obj.addr(), REFERENCE_SLOT_1F4, &args![])
            .u32();
        let words = [e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8)];
        e.vcall(
            process,
            PROCESS_SLOT_FC,
            // 005dad1d: the three words pushed before slot 0x1f4 (no stack words)
            // are the last three of slot 0xfc (RET 0x1c).
            &args![reference, words[0], words[1], words[2], first, second, a.this_obj],
        );
    }
    true
}

// Translated from 005dad70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `player + 0xdf2` to 1 ([`fn_005dadd0`]), then, when the
/// `TES` singleton has a world space with a terrain manager, calls
/// `006fd060` on the terrain manager. Always succeeds; none of the eight
/// words is read.
pub fn fn_005dad70(e: &mut Engine, _unused_args: ScriptArgs) -> bool {
    let player = player(e);
    fn_005dadd0(e, Ptr::new(player), 1);
    let tes = e.global::<u32>(TES_SINGLETON);
    if e.call(TES_GET_WORLD_SPACE, &args![tes]).u32() != 0 {
        let world = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
        if e.call(WORLD_SPACE_GET_TERRAIN_MANAGER, &args![world]).u32() != 0 {
            let world = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
            let terrain = e.call(WORLD_SPACE_GET_TERRAIN_MANAGER, &args![world]).u32();
            e.call(TERRAIN_MANAGER_FN_006FD060, &args![terrain]);
        }
    }
    true
}

// Translated from 005dadd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at `this + 0xdf2` (of the player).
pub fn fn_005dadd0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xdf2, value);
}

// Translated from 005dadf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `GetWeaponHealthPercConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005dadf0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, GET_WEAPON_HEALTH_PERC_CONDITION)
}

/// The item change (the equipped weapon) of `thisObj` that the two health
/// commands work on: `thisObj` must exist and pass the virtual test `0x100`,
/// have a process (`GetSavedAcquireObject`, asked twice) whose virtual slot
/// `0x148` returns a non-zero item change.
fn equipped_item(e: &mut Engine, this_obj: u32) -> Option<u32> {
    if this_obj == 0
        || !e.vcall(this_obj, REFERENCE_TEST_SLOT, &args![]).bool()
        || e.call(GET_PROCESS, &args![this_obj]).u32() == 0
    {
        return None;
    }
    let process = e.call(GET_PROCESS, &args![this_obj]).u32();
    let item = e.vcall(process, PROCESS_EQUIPPED_ITEM_SLOT, &args![]).u32();
    (item != 0).then_some(item)
}

/// Brings the health of `item` to `target`. Compared with its current
/// health (`GetItemHealth(0)`): a difference that is not positive is taken
/// off through `thisObj`'s virtual slot `0x3c8` (`item, -difference, 1`).
/// A positive one raises the health with `SetItemHealth(item, health,
/// container changes of thisObj's extra data list, first extra data list of
/// the item, 1)`, where `health` is `target`, or the smaller of `target` and
/// `ceiling` (`0040ebd0`, which gives its second argument when that is not
/// greater than the first).
fn change_item_health(e: &mut Engine, this_obj: u32, item: u32, target: f32, ceiling: Option<f32>) {
    let current = e.call(ITEM_GET_ITEM_HEALTH, &args![item, 0u32]).f32();
    let difference = (target as f64 - current as f64) as f32;
    if difference > 0.0 {
        let list = e.call(DEREF, &args![item]).u32();
        let data = e.call(LIST_NODE_DATA, &args![list]).u32();
        let first_extra_list = e.mem.u32(data);
        let extra_list = e.call(EXTRA_DATA_LIST, &args![this_obj]).u32();
        let changes = e.call(GET_CONTAINER_CHANGES, &args![extra_list]).u32();
        let health = match ceiling {
            Some(ceiling) => e.call(FLOAT_MIN, &args![target, ceiling]).f32(),
            None => target,
        };
        e.call(
            ITEM_SET_ITEM_HEALTH,
            &args![item, health, changes, first_extra_list, 1u32],
        );
    } else {
        e.vcall(
            this_obj,
            REFERENCE_LOWER_ITEM_HEALTH_SLOT,
            &args![item, -difference, 1u32],
        );
    }
}

// Translated from 005dae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a float (default 1.0), a percentage. For the equipped item
/// ([`equipped_item`]) the target health is the item form's maximum health
/// (`TESObjectWEAP::GetFormHealth(form; has mod effect 10)`) times the
/// percentage divided by 100; [`change_item_health`] brings the item to it,
/// a raise being limited to the maximum minus 1. Succeeds once the
/// parameters parse.
pub fn fn_005dae10(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([percent]) = parse_params(e, a, [1.0f32.to_bits()]) else {
        return false;
    };
    let this_obj = a.this_obj.addr();
    if let Some(item) = equipped_item(e, this_obj) {
        let hundred = e.global::<f64>(HUNDRED);
        let fraction = (f32::from_bits(percent) as f64 / hundred) as f32;
        let mod_active = e.call(ITEM_HAS_MOD_EFFECT_ACTIVE, &args![item, 10u32]).u8();
        let form = e.call(ITEM_GET_FORM, &args![item]).u32();
        let maximum = e
            .call(WEAPON_GET_FORM_HEALTH, &args![form, mod_active as u32])
            .i32() as f32;
        let target = (maximum as f64 * fraction as f64) as f32;
        let ceiling = (maximum as f64 - 1.0) as f32;
        change_item_health(e, this_obj, item, target, Some(ceiling));
    }
    true
}

// Translated from 005daf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a float (default 1.0), a percentage change. For the equipped item
/// ([`equipped_item`]) the new fraction is (`GetItemHealth(1)` + percentage)
/// / 100, at least 0; the target health is the maximum health of the item's
/// form (`TESHealthForm::GetFormHealth`, unsigned) times the fraction, and
/// [`change_item_health`] brings the item to it (no limit on a raise).
/// Succeeds once the parameters parse.
pub fn fn_005daf90(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([percent]) = parse_params(e, a, [1.0f32.to_bits()]) else {
        return false;
    };
    let this_obj = a.this_obj.addr();
    if let Some(item) = equipped_item(e, this_obj) {
        let current = e.call(ITEM_GET_ITEM_HEALTH, &args![item, 1u32]).f32();
        let total = (current as f64 + f32::from_bits(percent) as f64) as f32;
        let hundred = e.global::<f64>(HUNDRED);
        let mut fraction = (total as f64 / hundred) as f32;
        if fraction < 0.0 {
            fraction = 0.0;
        }
        let form = e.call(ITEM_GET_FORM, &args![item]).u32();
        let maximum = e.call(HEALTH_FORM_GET_FORM_HEALTH, &args![form]).u32() as f32;
        let target = (maximum as f64 * fraction as f64) as f32;
        change_item_health(e, this_obj, item, target, None);
    }
    true
}

// Translated from 005db110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetRumbleFunction` (Xbox PDB): parses three floats (left, right
/// and a duration in seconds; default 0.0). With a controls object
/// (`00877720(Main)`) it calls `Controls::Rumble(left, right, trunc(seconds
/// * 1000.0), 0, 0, 0, 1)`. Succeeds once the parameters parse.
pub fn script_set_rumble_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([left, right, seconds]) = parse_params(e, a, [0.0f32.to_bits(); 3]) else {
        return false;
    };
    let main = e.global::<u32>(MAIN_SINGLETON);
    let controls = e.call(MAIN_GET_CONTROLS, &args![main]).u32();
    if controls != 0 {
        let thousand = e.global::<f64>(THOUSAND);
        let duration = e
            .call(FTOL, &args![f32::from_bits(seconds) as f64 * thousand])
            .u32();
        e.call(
            CONTROLS_RUMBLE,
            &args![controls, left, right, duration, 0u32, 0u32, 0u32, 1u32],
        );
    }
    true
}

// Translated from 005db1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a form (default 0). When the parameters parse and the form is
/// not null, precaches its sound three times
/// (`BSAudio::Precache_ov3(audio; 00511840(form), 0x20000121, form)`) and
/// stores the form in the word at `011dcfa8` ([`fn_005db260`]). Always
/// succeeds, whether the parameters parse or not.
pub fn fn_005db1b0(e: &mut Engine, a: ScriptArgs) -> bool {
    if let Some([form]) = parse_params(e, a, [0]) {
        if form != 0 {
            for _ in 0..3 {
                let path = e.call(FORM_SOUND_PATH, &args![form]).u32();
                let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                e.call(AUDIO_PRECACHE, &args![audio, path, PRECACHE_FLAGS, form]);
            }
            fn_005db260(e, form);
        }
    }
    true
}

// Translated from 005db260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `011dcfa8`.
pub fn fn_005db260(e: &mut Engine, value: u32) {
    e.mem.set_u32(WORD_AT_11DCFA8, value);
}

// Translated from 005db270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `011dcfa8` ([`fn_005db280`]) and succeeds; none of
/// the eight words is read.
pub fn fn_005db270(e: &mut Engine, _unused_args: ScriptArgs) -> bool {
    fn_005db280(e);
    true
}

// Translated from 005db280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `011dcfa8`.
pub fn fn_005db280(e: &mut Engine) {
    e.mem.set_u32(WORD_AT_11DCFA8, 0);
}

// Translated from 005db290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::DrawSkeleton` (Xbox PDB): parses three integers (default 0). A
/// `thisObj` that casts to an actor (`TESObjectREFR` to `Actor`) with a
/// non-zero word at `+0xac` has its skins faded
/// (`Actor::FadeSkins(node from 0043fcd0, strength)`, strength 0.5 (the float
/// at `01016248`) when any integer is non-zero, else 1.0), then
/// `00c79d60(word at +0xac; first != 0, second != 0, third != 0)` is called.
/// Succeeds once the parameters parse.
pub fn script_draw_skeleton(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, second, third]) = parse_params(e, a, [0, 0, 0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        let actor = e
            .call(
                DYNAMIC_CAST,
                &args![a.this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
            )
            .u32();
        let flags = [first != 0, second != 0, third != 0];
        if actor != 0 && e.mem.u32(actor + 0xac) != 0 {
            let node = e.call(ACTOR_FN_0043FCD0, &args![actor]).u32();
            let strength = if flags.iter().any(|flag| *flag) {
                e.global::<f32>(FADE_SKINS_STRENGTH)
            } else {
                1.0f32
            };
            e.call(ACTOR_FADE_SKINS, &args![actor, node, strength]);
            let ragdoll = e.mem.u32(actor + 0xac);
            e.call(
                RAGDOLL_FN_00C79D60,
                &args![ragdoll, flags[0] as u32, flags[1] as u32, flags[2] as u32],
            );
        }
    }
    true
}

// Translated from 005db3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `IsGreetingPlayerConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005db3b0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, IS_GREETING_PLAYER_CONDITION)
}

// Translated from 005db3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of `GetIgnoreCrimeConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005db3d0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_argument(e, a, GET_IGNORE_CRIME_CONDITION)
}

// Translated from 005db3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a signed integer (default 0). When `thisObj` exists and passes
/// the virtual test `0x100`, calls its virtual slot `0x31c` with 1 when the
/// integer is greater than 0, else with 0. Succeeds once the parameters
/// parse.
pub fn fn_005db3f0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([amount]) = parse_params(e, a, [0]) else {
        return false;
    };
    let this_obj = a.this_obj.addr();
    if this_obj != 0 && e.vcall(this_obj, REFERENCE_TEST_SLOT, &args![]).bool() {
        e.vcall(
            this_obj,
            REFERENCE_SLOT_31C,
            &args![(amount as i32 > 0) as u32],
        );
    }
    true
}

// Translated from 005db490 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the Pipboy exists (`Interface::GetPipboy`), sets its byte at
/// `+0x16c` to 1 ([`fn_005db4c0`]). Always succeeds; none of the eight
/// words is read.
pub fn fn_005db490(e: &mut Engine, _unused_args: ScriptArgs) -> bool {
    let pipboy = e.call(INTERFACE_GET_PIPBOY, &args![]).u32();
    if pipboy != 0 {
        fn_005db4c0(e, Ptr::new(pipboy), 1);
    }
    true
}

// Translated from 005db4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at `this + 0x16c` (of the Pipboy).
pub fn fn_005db4c0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x16c, value);
}

// Translated from 005db4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsCombatTargetFunction` (Xbox PDB): parses an integer (default
/// 0) and returns the `AL` of `IsCombatTargetConditionFunction(thisObj,
/// value, 0, result)`.
pub fn script_is_combat_target_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parsed_condition(e, a, IS_COMBAT_TARGET_CONDITION)
}

// Translated from 005db540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSRightAreaFreeFunction` (Xbox PDB): like
/// [`script_is_combat_target_function`] with
/// `GetVATSRightAreaFreeConditionFunction`.
pub fn script_get_vats_right_area_free_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parsed_condition(e, a, GET_VATS_RIGHT_AREA_FREE_CONDITION)
}

// Translated from 005db5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSLeftAreaFreeFunction` (Xbox PDB): like
/// [`script_is_combat_target_function`] with
/// `GetVATSLeftAreaFreeConditionFunction`.
pub fn script_get_vats_left_area_free_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parsed_condition(e, a, GET_VATS_LEFT_AREA_FREE_CONDITION)
}

// Translated from 005db600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSBackAreaFreeFunction` (Xbox PDB): like
/// [`script_is_combat_target_function`] with
/// `GetVATSBackAreaFreeConditionFunction`.
pub fn script_get_vats_back_area_free_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parsed_condition(e, a, GET_VATS_BACK_AREA_FREE_CONDITION)
}

// Translated from 005db660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSFrontAreaFreeFunction` (Xbox PDB): like
/// [`script_is_combat_target_function`] with
/// `GetVATSFrontAreaFreeConditionFunction`.
pub fn script_get_vats_front_area_free_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parsed_condition(e, a, GET_VATS_FRONT_AREA_FREE_CONDITION)
}

// Translated from 005db6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores 1 in the byte at `011f5af1` ([`fn_005db6e0`]), closes the console
/// (`Interface::CloseConsole`) and succeeds; none of the eight words is
/// read.
pub fn fn_005db6c0(e: &mut Engine, _unused_args: ScriptArgs) -> bool {
    fn_005db6e0(e, 1);
    e.call(CLOSE_CONSOLE, &args![]);
    true
}

// Translated from 005db6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at `011f5af1`.
pub fn fn_005db6e0(e: &mut Engine, value: u8) {
    e.mem.set_u8(BYTE_AT_11F5AF1, value);
}

// Translated from 005db6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSRightTargetVisibleFunction` (Xbox PDB): like
/// [`script_is_combat_target_function`] with
/// `GetVATSRightTargetVisibleConditionFunction`.
pub fn script_get_vats_right_target_visible_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parsed_condition(e, a, GET_VATS_RIGHT_TARGET_VISIBLE_CONDITION)
}

// Translated from 005db750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSLeftTargetVisibleFunction` (Xbox PDB): like
/// [`script_is_combat_target_function`] with
/// `GetVATSLeftTargetVisibleConditionFunction`.
pub fn script_get_vats_left_target_visible_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parsed_condition(e, a, GET_VATS_LEFT_TARGET_VISIBLE_CONDITION)
}

// Translated from 005db7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSBackTargetVisibleFunction` (Xbox PDB): like
/// [`script_is_combat_target_function`] with
/// `GetVATSBackTargetVisibleConditionFunction`.
pub fn script_get_vats_back_target_visible_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parsed_condition(e, a, GET_VATS_BACK_TARGET_VISIBLE_CONDITION)
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005d7d30, fn_005d7d30(ScriptArgs) -> bool),
        entry!(0x005d7e30, fn_005d7e30(ScriptArgs) -> bool),
        entry!(0x005d7ef0, fn_005d7ef0(ScriptArgs) -> bool),
        entry!(0x005d7fb0, script_pipboy_radio(ScriptArgs) -> bool),
        entry!(0x005d8100, script_set_npc_radio(ScriptArgs) -> bool),
        entry!(0x005d81b0, script_get_broadcast_state(ScriptArgs) -> bool),
        entry!(0x005d8220, fn_005d8220(ScriptArgs) -> bool),
        entry!(0x005d8280, fn_005d8280() -> bool),
        entry!(0x005d82a0, script_start_radio_conversation(ScriptArgs) -> bool),
        entry!(0x005d8300, fn_005d8300(ScriptArgs) -> bool),
        entry!(0x005d8320, fn_005d8320(ScriptArgs) -> bool),
        entry!(0x005d83b0, fn_005d83b0(ScriptArgs) -> bool),
        entry!(0x005d83d0, fn_005d83d0(ScriptArgs) -> bool),
        entry!(0x005d8420, fn_005d8420(ScriptArgs) -> bool),
        entry!(0x005d8550, fn_005d8550(u32) -> u32),
        entry!(0x005d8570, fn_005d8570() -> bool),
        entry!(0x005d85a0, script_get_is_object_type_function(ScriptArgs) -> bool),
        entry!(0x005d8600, fn_005d8600(ScriptArgs) -> bool),
        entry!(0x005d8620, fn_005d8620(ScriptArgs) -> bool),
        entry!(0x005d8640, fn_005d8640(ScriptArgs) -> bool),
        entry!(0x005d8660, script_toggle_vats_light() -> bool),
        entry!(0x005d8710, fn_005d8710(u32) -> u32),
        entry!(0x005d8730, fn_005d8730(ScriptArgs) -> bool),
        entry!(0x005d8790, fn_005d8790() -> bool),
        entry!(0x005d87a0, fn_005d87a0(ScriptArgs) -> bool),
        entry!(0x005d8830, fn_005d8830(ScriptArgs) -> bool),
        entry!(0x005d88a0, fn_005d88a0(ScriptArgs) -> bool),
        entry!(0x005d8920, fn_005d8920() -> bool),
        entry!(0x005d8930, fn_005d8930() -> bool),
        entry!(0x005d8a70, fn_005d8a70(u32) -> u32),
        entry!(0x005d8a90, fn_005d8a90(Ptr, Ptr)),
        entry!(0x005d8af0, fn_005d8af0(Ptr) -> Ptr),
        entry!(0x005d8b10, fn_005d8b10(Ptr) -> Ptr),
        entry!(0x005d8b40, fn_005d8b40() -> bool),
        entry!(0x005d8b60, script_exit_game() -> bool),
        entry!(0x005d8bf0, fn_005d8bf0(ScriptArgs) -> bool),
        entry!(0x005d8cf0, script_trigger_enter_function(ScriptArgs) -> bool),
        entry!(0x005d8e20, fn_005d8e20(ScriptArgs) -> bool),
        entry!(0x005d8e80, fn_005d8e80(ScriptArgs) -> bool),
        entry!(0x005d90a0, fn_005d90a0(ScriptArgs) -> bool),
        entry!(0x005d9160, fn_005d9160(ScriptArgs) -> bool),
        entry!(0x005d9170, script_set_console_output_file_function(ScriptArgs) -> bool),
        entry!(0x005d9270, script_get_faction_relation_function(ScriptArgs) -> bool),
        entry!(0x005d92e0, script_is_last_played_idle_function(ScriptArgs) -> bool),
        entry!(0x005d9350, fn_005d9350(ScriptArgs) -> bool),
        entry!(0x005d9370, script_set_player_teammate_function(ScriptArgs) -> bool),
        entry!(0x005d93f0, fn_005d93f0(ScriptArgs) -> bool),
        entry!(0x005d9410, fn_005d9410(ScriptArgs) -> bool),
        entry!(0x005d9430, fn_005d9430(ScriptArgs) -> bool),
        entry!(0x005d94c0, fn_005d94c0(ScriptArgs) -> bool),
        entry!(0x005d94e0, fn_005d94e0(ScriptArgs) -> bool),
        entry!(0x005d9500, fn_005d9500(ScriptArgs) -> bool),
        entry!(0x005d9550, fn_005d9550(ScriptArgs) -> bool),
        entry!(0x005d9590, fn_005d9590(ScriptArgs) -> bool),
        entry!(0x005d95b0, fn_005d95b0(ScriptArgs) -> bool),
        entry!(0x005d95d0, script_set_player_tag_skill_function(ScriptArgs) -> bool),
        entry!(0x005d9660, fn_005d9660() -> u32),
        entry!(0x005d9670, script_is_player_tag_skill_function(ScriptArgs) -> bool),
        entry!(0x005d9730, script_get_player_grabbed_ref_function(ScriptArgs) -> bool),
        entry!(0x005d97b0, fn_005d97b0(ScriptArgs) -> bool),
        entry!(0x005d9810, fn_005d9810(ScriptArgs) -> bool),
        entry!(0x005d9aa0, fn_005d9aa0(ScriptArgs) -> bool),
        entry!(0x005d9ac0, script_force_active_quest_function(ScriptArgs) -> bool),
        entry!(0x005d9b20, fn_005d9b20(ScriptArgs) -> bool),
        entry!(0x005d9bc0, fn_005d9bc0(Ptr, Ptr, i32)),
        entry!(0x005d9f70, fn_005d9f70(Ptr, u32)),
        entry!(0x005d9f90, fn_005d9f90(Ptr) -> u32),
        entry!(0x005d9fb0, fn_005d9fb0(Ptr, u32) -> u32),
        entry!(0x005d9ff0, fn_005d9ff0(Ptr, u32) -> Ptr),
        entry!(0x005da060, script_get_is_used_item_equip_type_function(ScriptArgs) -> bool),
        entry!(0x005da0c0, script_get_threat_ratio_function(ScriptArgs) -> bool),
        entry!(0x005da120, script_get_is_alignment_function(ScriptArgs) -> bool),
        entry!(0x005da180, fn_005da180(ScriptArgs) -> bool),
        entry!(0x005da1e0, fn_005da1e0(ScriptArgs) -> bool),
        entry!(0x005da260, fn_005da260(ScriptArgs) -> bool),
        entry!(0x005da280, fn_005da280(ScriptArgs) -> bool),
        entry!(0x005da2a0, fn_005da2a0(ScriptArgs) -> bool),
        entry!(0x005da540, fn_005da540(ScriptArgs) -> bool),
        entry!(0x005da570, script_fire_weapon_function(ScriptArgs) -> bool),
        entry!(0x005da630, script_show_tutorial_menu(ScriptArgs) -> bool),
        entry!(0x005da690, fn_005da690(ScriptArgs) -> bool),
        entry!(0x005da7d0, fn_005da7d0(Ptr) -> u32),
        entry!(0x005da7f0, fn_005da7f0(Ptr) -> u32),
        entry!(0x005da810, fn_005da810(ScriptArgs) -> bool),
        entry!(0x005da980, fn_005da980(ScriptArgs) -> bool),
        entry!(0x005da9e0, fn_005da9e0(ScriptArgs) -> bool),
        entry!(0x005daa40, script_get_concussed_function(ScriptArgs) -> bool),
        entry!(0x005daaa0, fn_005daaa0(ScriptArgs) -> bool),
        entry!(0x005daac0, fn_005daac0(ScriptArgs) -> bool),
        entry!(0x005daae0, fn_005daae0(ScriptArgs) -> bool),
        entry!(0x005dab90, fn_005dab90(ScriptArgs) -> bool),
        entry!(0x005dac10, fn_005dac10(Ptr, u8)),
        entry!(0x005dac60, fn_005dac60(ScriptArgs) -> bool),
        entry!(0x005dac80, fn_005dac80(ScriptArgs) -> bool),
        entry!(0x005dad70, fn_005dad70(ScriptArgs) -> bool),
        entry!(0x005dadd0, fn_005dadd0(Ptr, u8)),
        entry!(0x005dadf0, fn_005dadf0(ScriptArgs) -> bool),
        entry!(0x005dae10, fn_005dae10(ScriptArgs) -> bool),
        entry!(0x005daf90, fn_005daf90(ScriptArgs) -> bool),
        entry!(0x005db110, script_set_rumble_function(ScriptArgs) -> bool),
        entry!(0x005db1b0, fn_005db1b0(ScriptArgs) -> bool),
        entry!(0x005db260, fn_005db260(u32)),
        entry!(0x005db270, fn_005db270(ScriptArgs) -> bool),
        entry!(0x005db280, fn_005db280()),
        entry!(0x005db290, script_draw_skeleton(ScriptArgs) -> bool),
        entry!(0x005db3b0, fn_005db3b0(ScriptArgs) -> bool),
        entry!(0x005db3d0, fn_005db3d0(ScriptArgs) -> bool),
        entry!(0x005db3f0, fn_005db3f0(ScriptArgs) -> bool),
        entry!(0x005db490, fn_005db490(ScriptArgs) -> bool),
        entry!(0x005db4c0, fn_005db4c0(Ptr, u8)),
        entry!(0x005db4e0, script_is_combat_target_function(ScriptArgs) -> bool),
        entry!(0x005db540, script_get_vats_right_area_free_function(ScriptArgs) -> bool),
        entry!(0x005db5a0, script_get_vats_left_area_free_function(ScriptArgs) -> bool),
        entry!(0x005db600, script_get_vats_back_area_free_function(ScriptArgs) -> bool),
        entry!(0x005db660, script_get_vats_front_area_free_function(ScriptArgs) -> bool),
        entry!(0x005db6c0, fn_005db6c0(ScriptArgs) -> bool),
        entry!(0x005db6e0, fn_005db6e0(u8)),
        entry!(0x005db6f0, script_get_vats_right_target_visible_function(ScriptArgs) -> bool),
        entry!(0x005db750, script_get_vats_left_target_visible_function(ScriptArgs) -> bool),
        entry!(0x005db7b0, script_get_vats_back_target_visible_function(ScriptArgs) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    // Fake virtual functions the tests put into vtables.
    const V_NAME: u32 = 0x0900_0001;
    const V_TRUE: u32 = 0x0900_0002;
    const V_FALSE: u32 = 0x0900_0003;
    const V_PERK_HELD: u32 = 0x0900_0004;
    const V_PERK_NOT_HELD: u32 = 0x0900_0005;
    const V_PROCESS: u32 = 0x0900_0006;

    /// An engine with the pages of the globals these commands touch mapped
    /// and the accessors every command uses replaced by doubles that behave
    /// like the exe's code (see the constants' documentation).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.register(CONSOLE_PRINT, |_, _| Ret::default());
        e.register(LOG_STUB, |_, _| Ret::default());
        e.register(GET_FORM_ID, |e, a| e.mem.u32(a[0] + 0x0c).into_ret());
        e.register(V_NAME, |_, _| 0xbbbb.into_ret());
        e.register(V_TRUE, |_, _| true.into_ret());
        e.register(V_FALSE, |_, _| false.into_ret());
        e.register(V_PERK_HELD, |_, _| true.into_ret());
        e.register(V_PERK_NOT_HELD, |_, _| false.into_ret());
        e.register(V_PROCESS, |_, _| Ret::default());
        e
    }

    /// A zeroed object of 0x800 bytes with a vtable that has the given
    /// `(byte offset, function)` slots.
    fn object_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x800);
        for (offset, function) in slots {
            e.mem.set_u32(vtable + offset, *function);
        }
        let object = e.mem.alloc(0x800);
        e.mem.set_u32(object, vtable);
        object
    }

    fn object(e: &mut Engine) -> u32 {
        object_with(e, &[])
    }

    /// The standard eight words with `this_obj` set and a result double.
    fn command(e: &mut Engine, this_obj: u32) -> ScriptArgs {
        let result = e.mem.alloc(8);
        ScriptArgs {
            param_info: 1,
            script_data: 2,
            this_obj: Ptr::new(this_obj),
            containing_obj: Ptr::NULL,
            script_obj: Ptr::new(5),
            event_list: 6,
            result: Ptr::new(result),
            opcode_offset: 8,
        }
    }

    /// `ParseParameters` double: returns `ok` and stores `outs` through its
    /// output pointers (after the seven fixed words).
    fn parse_gives(e: &mut Engine, ok: bool, outs: &[u32]) {
        let outs = outs.to_vec();
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            for (i, value) in outs.iter().enumerate() {
                e.mem.set_u32(a[7 + i], *value);
            }
            ok.into_ret()
        });
    }

    /// `ParseParameters` double for a command whose first output is a text
    /// and whose second is a word.
    fn parse_gives_text(e: &mut Engine, text: &'static str, word: u32) {
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            e.mem.set_cstr(a[7], text.as_bytes());
            e.mem.set_u32(a[8], word);
            true.into_ret()
        });
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The argument words of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn set_echo(e: &mut Engine, on: bool) {
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_ECHO, on as u8);
    }

    fn set_player(e: &mut Engine) -> u32 {
        let existing: u32 = e.global(PLAYER);
        if existing != 0 {
            return existing;
        }
        let player_object = object(e);
        e.set_global(PLAYER, player_object);
        player_object
    }

    /// `ParseParameters` gets: info, data, opcode offset, thisObj,
    /// containing, script, event list, then the address of the local.
    fn assert_parsed(e: &Engine, this_obj: u32) {
        assert_eq!(
            calls(e, PARSE_PARAMETERS)[0][..7],
            [1, 2, 8, this_obj, 0, 5, 6]
        );
    }

    fn run(e: &mut Engine, address: u32, a: ScriptArgs) -> bool {
        e.call(address, &args![a]).bool()
    }

    // ---- 005d7d30 and the objective getters ----

    #[test]
    fn fn_005d7d30_sets_the_objective_flag_or_reports_a_missing_objective() {
        let mut e = engine();
        let quest = object_with(&mut e, &[(QUEST_NAME_SLOT, V_NAME)]);
        e.mem.set_u32(quest + 0x0c, 0x1234);
        let a = command(&mut e, 0);
        e.register(OBJECTIVE_SET, |_, _| Ret::default());
        e.register(QUEST_FLAG_2, |_, _| false.into_ret());
        e.register(OBJECTIVE_BIT_0, |_, _| false.into_ret());

        // Missing objective: the message gets the index, name and id.
        e.register(QUEST_FIND_OBJECTIVE, |_, _| 0u32.into_ret());
        parse_gives(&mut e, true, &[quest, 3, 1]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_7d30, a));
        assert_parsed(&e, 0);
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_OBJECTIVE_MISSING, 3, 0xbbbb, 0x1234]]
        );
        assert_eq!(calls(&e, QUEST_FIND_OBJECTIVE), vec![vec![quest, 3]]);
        assert!(calls(&e, OBJECTIVE_SET).is_empty());

        // Found, flag 1, bit 0 clear: the setter gets 1.
        e.register(QUEST_FIND_OBJECTIVE, |_, _| 0x7000u32.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_7d30, a));
        assert_eq!(calls(&e, OBJECTIVE_SET), vec![vec![0x7000, 1]]);
        assert!(calls(&e, LOG_STUB).is_empty());

        // Flag 1 with bit 0 already set: nothing.
        e.register(OBJECTIVE_BIT_0, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_7d30, a));
        assert!(calls(&e, OBJECTIVE_SET).is_empty());

        // Flag 0: the setter gets 0 whatever bit 0 says.
        parse_gives(&mut e, true, &[quest, 3, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_7d30, a));
        assert_eq!(calls(&e, OBJECTIVE_SET), vec![vec![0x7000, 0]]);

        // The quest flag set: nothing at all.
        e.register(QUEST_FLAG_2, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_7d30, a));
        assert!(calls(&e, OBJECTIVE_SET).is_empty());

        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_7d30, a));
        assert!(calls(&e, QUEST_FIND_OBJECTIVE).is_empty());
    }

    #[test]
    fn fn_005d7d30_asks_a_null_quest_for_its_name_like_the_game() {
        let mut e = engine();
        // The game reads the vtable at address 0; the page is mapped here.
        e.map(0, 0x1000);
        let vtable = e.mem.alloc(0x800);
        e.mem.set_u32(vtable + QUEST_NAME_SLOT, V_NAME);
        e.mem.set_u32(0, vtable);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0, 3, 1]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_7d30, a));
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_QUEST_MISSING, 0xbbbb, 0]]
        );
        assert!(calls(&e, QUEST_FIND_OBJECTIVE).is_empty());
    }

    /// The test of an objective getter: `address` is the command, `test` the
    /// objective test it calls.
    fn check_objective_getter(address: u32, test: u32, format: u32) {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.mem.set_f64(a.result.addr(), 7.0);
        e.register(QUEST_FIND_OBJECTIVE, |_, _| 0x7000u32.into_ret());
        e.register(test, |_, _| true.into_ret());

        // No quest: the result stays as it was.
        parse_gives(&mut e, true, &[0, 1, 0]);
        start_log(&mut e);
        assert!(run(&mut e, address, a));
        assert_eq!(e.mem.f64(a.result.addr()), 7.0);
        assert!(calls(&e, QUEST_FIND_OBJECTIVE).is_empty());

        // Objective missing: the result stays as it was.
        parse_gives(&mut e, true, &[0x5000, 2, 0]);
        e.register(QUEST_FIND_OBJECTIVE, |_, _| 0u32.into_ret());
        assert!(run(&mut e, address, a));
        assert_eq!(e.mem.f64(a.result.addr()), 7.0);

        // Found: the test decides, and the echo prints the result.
        e.register(QUEST_FIND_OBJECTIVE, |_, _| 0x7000u32.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, address, a));
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        assert_eq!(calls(&e, QUEST_FIND_OBJECTIVE), vec![vec![0x5000, 2]]);
        assert_eq!(calls(&e, test), vec![vec![0x7000]]);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        e.register(test, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, address, a));
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![args![format, 0.0f64]]);

        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, address, a));
    }

    #[test]
    fn fn_005d7e30_reports_whether_the_objective_is_completed() {
        check_objective_getter(
            0x005d_7e30,
            OBJECTIVE_ABOVE_ONE,
            MSG_GET_OBJECTIVE_COMPLETED,
        );
    }

    #[test]
    fn fn_005d7ef0_reports_whether_the_objective_is_displayed() {
        check_objective_getter(0x005d_7ef0, OBJECTIVE_BIT_0, MSG_GET_OBJECTIVE_DISPLAYED);
    }

    // ---- Radio commands ----

    #[test]
    fn pipboy_radio_reads_the_word_and_enables_disables_or_tunes() {
        let mut e = engine();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0103_9000, 0x1000);
        e.map(0x0103_c000, 0x1000);
        e.mem.set_cstr(TEXT_ENABLE, b"enable");
        e.mem.set_cstr(TEXT_ON, b"on");
        e.mem.set_cstr(TEXT_DISABLE, b"disable");
        e.mem.set_cstr(TEXT_OFF, b"off");
        e.mem.set_cstr(TEXT_TUNE, b"tune");
        e.register(STRING_COMPARE_NO_CASE, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            u32::from(!x.eq_ignore_ascii_case(&y)).into_ret()
        });
        e.register(PIPBOY_RADIO_ENABLE, |_, _| Ret::default());
        e.register(RADIO_FN_00832240, |_, _| Ret::default());
        let a = command(&mut e, 0);

        // (text, expected enable calls, expected tune calls)
        let enable = vec![vec![1]];
        let disable = vec![vec![0]];
        let tune = vec![vec![9, 1]];
        type Calls = Vec<Vec<u32>>;
        let cases: [(&'static str, Calls, Calls); 9] = [
            ("1", enable.clone(), tune.clone()),
            ("Enable", enable.clone(), tune.clone()),
            ("ON", enable, tune.clone()),
            ("0", disable.clone(), vec![]),
            ("DISABLE", disable.clone(), vec![]),
            ("Off", disable, vec![]),
            ("tune", vec![], tune),
            ("unknown", vec![], vec![]),
            ("", vec![], vec![]),
        ];
        for (text, enable, tune) in cases {
            parse_gives_text(&mut e, text, 9);
            start_log(&mut e);
            assert!(run(&mut e, 0x005d_7fb0, a), "{text:?}");
            assert_eq!(calls(&e, PIPBOY_RADIO_ENABLE), enable, "{text:?}");
            assert_eq!(calls(&e, RADIO_FN_00832240), tune, "{text:?}");
        }

        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_7fb0, a));
        assert!(calls(&e, PIPBOY_RADIO_ENABLE).is_empty());
    }

    #[test]
    fn set_npc_radio_enables_or_disables_the_radio_of_an_actor_with_a_value() {
        let mut e = engine();
        let actor = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_TRUE)]);
        let a = command(&mut e, actor);
        e.register(ENABLE_NPC_RADIO, |_, _| Ret::default());
        e.register(DISABLE_NPC_RADIO, |_, _| Ret::default());

        parse_gives(&mut e, true, &[1, 0x77]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8100, a));
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, ENABLE_NPC_RADIO), vec![vec![actor, 0x77]]);
        assert!(calls(&e, DISABLE_NPC_RADIO).is_empty());

        parse_gives(&mut e, true, &[0, 0x77]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8100, a));
        assert!(calls(&e, ENABLE_NPC_RADIO).is_empty());
        assert_eq!(calls(&e, DISABLE_NPC_RADIO), vec![vec![actor]]);

        // Another mode: succeeds without touching the radio.
        parse_gives(&mut e, true, &[2, 0x77]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8100, a));
        assert!(calls(&e, ENABLE_NPC_RADIO).is_empty());
        assert!(calls(&e, DISABLE_NPC_RADIO).is_empty());

        // A zero value, a failing parse, no reference or a failing test.
        parse_gives(&mut e, true, &[1, 0]);
        assert!(!run(&mut e, 0x005d_8100, a));
        parse_gives(&mut e, false, &[1, 0x77]);
        assert!(!run(&mut e, 0x005d_8100, a));
        parse_gives(&mut e, true, &[1, 0x77]);
        let none = command(&mut e, 0);
        assert!(!run(&mut e, 0x005d_8100, none));
        let refusing = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_FALSE)]);
        let b = command(&mut e, refusing);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_8100, b));
        assert!(calls(&e, ENABLE_NPC_RADIO).is_empty());
    }

    #[test]
    fn get_broadcast_state_stores_the_radio_state_in_the_result() {
        let mut e = engine();
        let a = command(&mut e, 0x4444);
        e.register(RADIO_BROADCAST_STATE, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_81b0, a));
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        assert_eq!(calls(&e, RADIO_BROADCAST_STATE), vec![vec![0x4444]]);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());

        e.register(RADIO_BROADCAST_STATE, |_, _| false.into_ret());
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_81b0, a));
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![args![MSG_GET_BROADCAST_STATE, 0.0f64]]
        );
    }

    #[test]
    fn fn_005d8220_passes_whether_the_value_is_positive() {
        let mut e = engine();
        let a = command(&mut e, 0x4444);
        e.register(RADIO_FN_008359E0, |_, _| Ret::default());
        for (value, flag) in [(5u32, 1u32), (0, 0), (-3i32 as u32, 0)] {
            parse_gives(&mut e, true, &[value]);
            start_log(&mut e);
            assert!(run(&mut e, 0x005d_8220, a));
            assert_parsed(&e, 0x4444);
            assert_eq!(calls(&e, RADIO_FN_008359E0), vec![vec![0x4444, flag]]);
        }
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_8220, a));
        assert!(calls(&e, RADIO_FN_008359E0).is_empty());
    }

    #[test]
    fn fn_005d8280_updates_the_radio() {
        let mut e = engine();
        e.register(RADIO_UPDATE, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_8280, &args![]).bool());
        assert_eq!(calls(&e, RADIO_UPDATE), vec![vec![1]]);
    }

    #[test]
    fn start_radio_conversation_passes_the_parsed_value() {
        let mut e = engine();
        let a = command(&mut e, 0x4444);
        e.register(START_RADIO_CONVERSATION, |_, _| Ret::default());
        parse_gives(&mut e, true, &[0x66]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_82a0, a));
        assert_eq!(
            calls(&e, START_RADIO_CONVERSATION),
            vec![vec![0x4444, 0x66]]
        );
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_82a0, a));
        assert!(calls(&e, START_RADIO_CONVERSATION).is_empty());
    }

    /// The test of a "call a condition function with (thisObj, 0, 0,
    /// result)" command.
    fn check_condition_only(address: u32, condition: u32) {
        let mut e = engine();
        let a = command(&mut e, 0x4444);
        e.register(condition, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(!run(&mut e, address, a));
        assert_eq!(
            calls(&e, condition),
            vec![vec![0x4444, 0, 0, a.result.addr()]]
        );
        e.register(condition, |_, _| true.into_ret());
        assert!(run(&mut e, address, a));
    }

    #[test]
    fn fn_005d8300_returns_the_condition_function_s_result() {
        check_condition_only(0x005d_8300, CONDITION_FN_005A43B0);
    }

    #[test]
    fn fn_005d8600_returns_the_condition_function_s_result() {
        check_condition_only(0x005d_8600, GET_DIALOGUE_EMOTION_CONDITION);
    }

    #[test]
    fn fn_005d8620_returns_the_condition_function_s_result() {
        check_condition_only(0x005d_8620, GET_DIALOGUE_EMOTION_VALUE_CONDITION);
    }

    #[test]
    fn fn_005d8640_returns_the_condition_function_s_result() {
        check_condition_only(0x005d_8640, IS_WATER_OBJECT_CONDITION);
    }

    #[test]
    fn fn_005d8320_switches_the_audio_thread_mode_only_when_it_changes() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.register(AUDIO_INSTANCE, |_, _| 0xa0d10u32.into_ret());
        e.register(AUDIO_SET_MULTI_THREADED, |_, _| Ret::default());
        // (value parsed, current byte, expected SetMultiThreaded calls)
        let cases = [
            (1u32, false, vec![vec![0xa0d10, 1]]),
            (1, true, vec![]),
            (0, true, vec![vec![0xa0d10, 0]]),
            (0, false, vec![]),
        ];
        for (value, current, expected) in cases {
            parse_gives(&mut e, true, &[value]);
            e.register_double(AUDIO_BYTE_6, move |_, _| current.into_ret());
            start_log(&mut e);
            assert!(run(&mut e, 0x005d_8320, a), "{value} {current}");
            assert_eq!(calls(&e, AUDIO_SET_MULTI_THREADED), expected);
        }
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_8320, a));
        assert!(calls(&e, AUDIO_INSTANCE).is_empty());
    }

    #[test]
    fn fn_005d83b0_forwards_this_obj_and_zero() {
        let mut e = engine();
        let a = command(&mut e, 0x4444);
        e.register(REFERENCE_FN_0056A9B0, |_, _| Ret::default());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_83b0, a));
        assert_eq!(calls(&e, REFERENCE_FN_0056A9B0), vec![vec![0x4444, 0]]);
    }

    #[test]
    fn fn_005d83d0_forwards_this_obj_and_the_parsed_value() {
        let mut e = engine();
        let a = command(&mut e, 0x4444);
        e.register(REFERENCE_FN_0056A9B0, |_, _| Ret::default());
        parse_gives(&mut e, true, &[0x21]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_83d0, a));
        assert_eq!(calls(&e, REFERENCE_FN_0056A9B0), vec![vec![0x4444, 0x21]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_83d0, a));
        assert!(calls(&e, REFERENCE_FN_0056A9B0).is_empty());
    }

    // ---- Cell commands ----

    #[test]
    fn fn_005d8420_builds_the_point_and_hands_it_to_the_cell() {
        let mut e = engine();
        let player_object = set_player(&mut e);
        let a = command(&mut e, 0);
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(POINT3_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(CELL_FN_005578E0, |_, _| Ret::default());
        e.register(PLAYER_GET_PARENT_CELL, |_, _| 0xce11u32.into_ret());
        let (x, y, z) = (1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits());

        // With a text: the text address is passed on; 13 % 10 = entry 3.
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            // The text buffer was zeroed before the parse.
            assert_eq!(e.mem.u32(a[10]), 0);
            e.mem.set_u32(a[7], x);
            e.mem.set_u32(a[8], y);
            e.mem.set_u32(a[9], z);
            e.mem.set_cstr(a[10], b"Marker");
            e.mem.set_u32(a[11], 13);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8420, a));
        let memset = calls(&e, MEMSET)[0].clone();
        assert_eq!(memset[1..], [0, 0x1ff]);
        let point = calls(&e, POINT3_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, POINT3_CONSTRUCT), vec![vec![point, x, y, z]]);
        assert_eq!(calls(&e, PLAYER_GET_PARENT_CELL)[0], vec![player_object]);
        let cell_call = calls(&e, CELL_FN_005578E0)[0].clone();
        assert_eq!(cell_call[0], 0xce11);
        assert_eq!(cell_call[1], point);
        assert_eq!(cell_call[2], memset[0] - 1);
        assert_eq!(cell_call[3], 0x0120_3288);

        // An empty text passes 0; entry 4.
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            e.mem.set_u32(a[11], 4);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8420, a));
        let cell_call = calls(&e, CELL_FN_005578E0)[0].clone();
        assert_eq!(cell_call[2..], [0, 0x0120_3298]);

        // No parent cell: nothing is built.
        e.register(PLAYER_GET_PARENT_CELL, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8420, a));
        assert!(calls(&e, POINT3_CONSTRUCT).is_empty());

        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_8420, a));
    }

    #[test]
    fn fn_005d8550_indexes_the_table_modulo_ten() {
        let mut e = engine();
        assert_eq!(e.call(0x005d_8550, &args![0u32]).u32(), 0x0120_3258);
        assert_eq!(e.call(0x005d_8550, &args![13u32]).u32(), 0x0120_3288);
        assert_eq!(e.call(0x005d_8550, &args![9u32]).u32(), 0x0120_32e8);
        assert_eq!(e.call(0x005d_8550, &args![10u32]).u32(), 0x0120_3258);
        // 0xffffffff % 10 = 5 (unsigned).
        assert_eq!(
            e.call(0x005d_8550, &args![0xffff_ffffu32]).u32(),
            0x0120_3258 + 5 * 0x10
        );
    }

    #[test]
    fn fn_005d8570_calls_the_cell_only_when_the_player_has_one() {
        let mut e = engine();
        let player_object = set_player(&mut e);
        e.register(CELL_FN_005578F0, |_, _| Ret::default());
        e.register(PLAYER_GET_PARENT_CELL, |_, _| 0xce11u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_8570, &args![]).bool());
        assert_eq!(calls(&e, CELL_FN_005578F0), vec![vec![0xce11]]);
        assert_eq!(calls(&e, PLAYER_GET_PARENT_CELL)[0], vec![player_object]);
        e.register(PLAYER_GET_PARENT_CELL, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_8570, &args![]).bool());
        assert!(calls(&e, CELL_FN_005578F0).is_empty());
    }

    #[test]
    fn get_is_object_type_passes_the_parsed_byte_to_the_condition_function() {
        let mut e = engine();
        let a = command(&mut e, 0x4444);
        e.register(GET_IS_OBJECT_TYPE_CONDITION, |_, _| true.into_ret());
        // Only the low byte of the local counts.
        parse_gives(&mut e, true, &[0x1234_5642]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_85a0, a));
        assert_parsed(&e, 0x4444);
        assert_eq!(
            calls(&e, GET_IS_OBJECT_TYPE_CONDITION),
            vec![vec![0x4444, 0x42, 0, a.result.addr()]]
        );
        e.register(GET_IS_OBJECT_TYPE_CONDITION, |_, _| false.into_ret());
        assert!(!run(&mut e, 0x005d_85a0, a));
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_85a0, a));
        assert!(calls(&e, GET_IS_OBJECT_TYPE_CONDITION).is_empty());
    }

    // ---- VATS light and the small forwarders ----

    #[test]
    fn toggle_vats_light_flips_the_state_through_the_derived_object() {
        let mut e = engine();
        e.register(DEREF, |_, a| (a[0] + 0x1000).into_ret());
        e.register(DEREF_AT_F8, |_, a| (a[0] + 0x10).into_ret());
        e.register(FN_00456610, |_, _| true.into_ret());
        e.register(FN_00450F90, |_, _| Ret::default());
        e.register(FN_00450B80, |_, _| 0xe0e0u32.into_ret());
        e.register(FN_00B5D270, |_, _| Ret::default());

        // State set: "On", the flag is cleared, the entry gets the value.
        start_log(&mut e);
        assert!(e.call(0x005d_8660, &args![]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_VATS_LIGHTING_ON]]);
        let value = OBJECT_AT_11F2250 + 0x34 + 0x1000;
        assert_eq!(calls(&e, DEREF)[0], vec![OBJECT_AT_11F2250 + 0x34]);
        assert_eq!(calls(&e, FN_00450F90), vec![vec![value + 0x10, 0]]);
        assert_eq!(calls(&e, FN_00450B80), vec![vec![0]]);
        assert_eq!(calls(&e, FN_00B5D270), vec![vec![0xe0e0, value]]);

        // State clear: "Off", the flag is set.
        e.register(FN_00456610, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_8660, &args![]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_VATS_LIGHTING_OFF]]);
        assert_eq!(calls(&e, FN_00450F90), vec![vec![value + 0x10, 1]]);
        assert!(calls(&e, FN_00B5D270).is_empty());

        // Nothing to toggle.
        e.register(DEREF, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_8660, &args![]).bool());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        assert!(calls(&e, FN_00450F90).is_empty());
    }

    #[test]
    fn fn_005d8710_reads_the_word_at_offset_0x34() {
        let mut e = engine();
        e.register(DEREF, |e, a| e.mem.u32(a[0]).into_ret());
        let holder = object(&mut e);
        e.mem.set_u32(holder + 0x34, 0x1357);
        assert_eq!(e.call(0x005d_8710, &args![holder]).u32(), 0x1357);
    }

    #[test]
    fn fn_005d8730_passes_the_value_zero_float_and_zero() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.register(FN_004E0330, |_, _| Ret::default());
        parse_gives(&mut e, true, &[0x44]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8730, a));
        assert_eq!(calls(&e, FN_004E0330), vec![vec![0x44, 0, 0]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_8730, a));
        assert!(calls(&e, FN_004E0330).is_empty());
    }

    #[test]
    fn fn_005d8790_and_fn_005d8920_call_their_callee() {
        let mut e = engine();
        e.register(FN_004E0870, |_, _| Ret::default());
        e.register(FN_004DEFB0, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_8790, &args![]).bool());
        assert!(e.call(0x005d_8920, &args![]).bool());
        assert_eq!(calls(&e, FN_004E0870).len(), 1);
        assert_eq!(calls(&e, FN_004DEFB0).len(), 1);
    }

    #[test]
    fn fn_005d87a0_reorders_the_parsed_words_for_the_callee() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.register(FN_004DE8E0, |_, _| Ret::default());
        parse_gives(&mut e, true, &[0xa, 0xb, 0xc, 0xd]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_87a0, a));
        assert_eq!(calls(&e, FN_004DE8E0), vec![vec![0xa, 0xb, 0, 0xc, 0xd]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_87a0, a));
        assert!(calls(&e, FN_004DE8E0).is_empty());
    }

    #[test]
    fn fn_005d8830_passes_the_float_and_whether_the_byte_is_set() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.register(FN_004DEF00, |_, _| Ret::default());
        let value = 2.5f32.to_bits();
        for (byte, flag) in [(0u32, 0u32), (1, 1), (0x100, 0), (0x80, 1)] {
            parse_gives(&mut e, true, &[value, byte]);
            start_log(&mut e);
            assert!(run(&mut e, 0x005d_8830, a));
            assert_eq!(calls(&e, FN_004DEF00), vec![vec![value, flag]], "{byte}");
        }
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_8830, a));
        assert!(calls(&e, FN_004DEF00).is_empty());
    }

    #[test]
    fn fn_005d88a0_passes_the_text_and_the_byte() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.register_double(FN_004DEED0, |e, a| {
            assert_eq!(e.mem.cstr(a[0]), b"abc");
            Ret::default()
        });
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"abc");
            e.mem.set_u32(a[8], 0x177);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_88a0, a));
        // Only the low byte is passed.
        assert_eq!(calls(&e, FN_004DEED0)[0][1], 0x77);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_88a0, a));
        assert!(calls(&e, FN_004DEED0).is_empty());
    }

    // ---- The dialogue reset and its functor ----

    #[test]
    fn fn_005d8930_resets_every_element_of_every_item_of_every_topic() {
        let mut e = engine();
        let player_object = set_player(&mut e);
        // Lists are chains of nodes `[data][next]`; the owner's first word
        // is the head, a topic's embedded list starts at `topic + 0x2c`.
        fn node(e: &mut Engine, data: u32, next: u32) -> u32 {
            let n = e.mem.alloc(8);
            e.mem.set_u32(n, data);
            e.mem.set_u32(n + 4, next);
            n
        }
        e.register(LIST_FIRST_NODE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_NODE_DATA, |_, a| a[0].into_ret());
        // An item's first word is its array; the array has its count at
        // +0x0c (the form id accessor) and the elements from +0x20.
        e.register(ARRAY_AT_4, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ARRAY_ELEMENT_ADDRESS, |_, a| {
            (a[0] + 0x20 + 4 * a[1]).into_ret()
        });
        for addr in [
            RESET_PLAYER_GREET_FLAG,
            GET_TOPIC,
            ELEMENT_FN_0061F280,
            DEBUG_RESET_SAID_ONCE_FLAGS,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let functor_seen = Rc::new(RefCell::new(vec![]));
        let seen = functor_seen.clone();
        e.register_double(PROCESS_LISTS_VISIT, move |e, a| {
            seen.borrow_mut()
                .push((a[0], e.mem.u32(a[1]), e.mem.u32(a[1] + 4)));
            Ret::default()
        });

        let make_array = |e: &mut Engine, elements: &[u32]| {
            let array = e.mem.alloc(0x40);
            e.mem.set_u32(array + 0x0c, elements.len() as u32);
            for (i, element) in elements.iter().enumerate() {
                e.mem.set_u32(array + 0x20 + 4 * i as u32, *element);
            }
            array
        };
        let make_item = |e: &mut Engine, array: u32| {
            let item = e.mem.alloc(8);
            e.mem.set_u32(item, array);
            item
        };
        let array1 = make_array(&mut e, &[0x101, 0, 0x102]);
        let array2 = make_array(&mut e, &[0x103]);
        let item1 = make_item(&mut e, array1);
        let item2 = make_item(&mut e, array2);
        // Topic 1: items `item1`, a null entry, `item2`. Topic 2: empty.
        let topic1 = e.mem.alloc(0x40);
        let last = node(&mut e, item2, 0);
        let null_entry = node(&mut e, 0, last);
        e.mem.set_u32(topic1 + 0x2c, item1);
        e.mem.set_u32(topic1 + 0x30, null_entry);
        let topic2 = e.mem.alloc(0x40);
        let n3 = node(&mut e, topic2, 0);
        let n2 = node(&mut e, 0, n3);
        let n1 = node(&mut e, topic1, n2);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner, n1);
        e.set_global(LIST_OWNER, owner);

        start_log(&mut e);
        assert!(e.call(0x005d_8930, &args![]).bool());
        // The functor handed to the process lists is the constructed one.
        assert_eq!(
            *functor_seen.borrow(),
            vec![(PROCESS_LISTS, FUNCTOR_VTABLE, 0)]
        );
        assert_eq!(
            calls(&e, RESET_PLAYER_GREET_FLAG),
            vec![vec![player_object]]
        );
        assert_eq!(calls(&e, GET_TOPIC), vec![vec![6, 0xc], vec![1, 0]]);
        assert_eq!(
            calls(&e, ELEMENT_FN_0061F280),
            vec![vec![0x101], vec![0x102], vec![0x103]]
        );
        let log = e.call_log.as_ref().unwrap();
        assert_eq!(log.last().unwrap().0, DEBUG_RESET_SAID_ONCE_FLAGS);
    }

    #[test]
    fn fn_005d8a70_is_the_embedded_list_of_a_topic() {
        let mut e = engine();
        assert_eq!(e.call(0x005d_8a70, &args![0x1000u32]).u32(), 0x102c);
    }

    #[test]
    fn fn_005d8a90_removes_extra_data_and_notifies_the_process_of_a_tested_actor() {
        let mut e = engine();
        let process = object_with(&mut e, &[(PROCESS_SLOT_310, V_PROCESS)]);
        let actor = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_TRUE)]);
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(EXTRA_DATA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(REMOVE_EXTRA, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x005d_8a90, &args![0x9999u32, actor]);
        assert_eq!(calls(&e, V_PROCESS), vec![vec![process, 0]]);
        assert_eq!(calls(&e, REMOVE_EXTRA), vec![vec![actor + 0x44, 0x73]]);

        // No process: only the extra data is removed.
        e.register(GET_PROCESS, |_, _| 0u32.into_ret());
        start_log(&mut e);
        e.call(0x005d_8a90, &args![0x9999u32, actor]);
        assert!(calls(&e, V_PROCESS).is_empty());
        assert_eq!(calls(&e, REMOVE_EXTRA).len(), 1);

        // The virtual test fails: the process is not asked at all.
        let other = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_FALSE)]);
        start_log(&mut e);
        e.call(0x005d_8a90, &args![0x9999u32, other]);
        assert!(calls(&e, GET_PROCESS).is_empty());
        assert_eq!(calls(&e, REMOVE_EXTRA), vec![vec![other + 0x44, 0x73]]);
    }

    #[test]
    fn the_functor_constructors_set_the_vtables_and_clear_the_word() {
        let mut e = engine();
        let functor = e.mem.alloc(8);
        e.mem.set_u32(functor + 4, 0xdead);
        assert_eq!(e.call(0x005d_8b10, &args![functor]).u32(), functor);
        assert_eq!(e.mem.u32(functor), FUNCTOR_BASE_VTABLE);
        assert_eq!(e.mem.u32(functor + 4), 0);
        e.mem.set_u32(functor + 4, 0xdead);
        assert_eq!(e.call(0x005d_8af0, &args![functor]).u32(), functor);
        assert_eq!(e.mem.u32(functor), FUNCTOR_VTABLE);
        assert_eq!(e.mem.u32(functor + 4), 0);
    }

    #[test]
    fn fn_005d8b40_calls_the_player() {
        let mut e = engine();
        let player_object = set_player(&mut e);
        e.register(PLAYER_FN_00966D70, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_8b40, &args![]).bool());
        assert_eq!(calls(&e, PLAYER_FN_00966D70), vec![vec![player_object]]);
    }

    #[test]
    fn exit_game_stores_two_when_the_player_has_the_perk_and_one_otherwise() {
        let mut e = engine();
        let player_object = object_with(&mut e, &[(PLAYER_HAS_PERK_SLOT, V_PERK_HELD)]);
        e.set_global(PLAYER, player_object);
        e.set_global(EXIT_GAME_GLOBAL, 0x5050u32);
        e.register(LOOKUP_FORM, |_, _| 0x77u32.into_ret());
        e.register(DYNAMIC_CAST, |_, _| 0x88u32.into_ret());
        e.register(TES_GLOBAL_SET_VALUE, |_, _| Ret::default());
        e.register(CLOSE_CONSOLE, |_, _| Ret::default());
        e.register(CHOOSE_MAIN_MENU, |_, _| Ret::default());

        start_log(&mut e);
        assert!(e.call(0x005d_8b60, &args![]).bool());
        assert_eq!(calls(&e, LOOKUP_FORM), vec![vec![0x000e_d568]]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x77, 0, 0x0118_3028, 0x0118_61dc, 0]]
        );
        assert_eq!(calls(&e, V_PERK_HELD), vec![vec![player_object, 0x88, 0]]);
        assert_eq!(
            calls(&e, TES_GLOBAL_SET_VALUE),
            vec![vec![0x5050, 2.0f32.to_bits()]]
        );
        let log = e.call_log.as_ref().unwrap();
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(order[order.len() - 2..], [CLOSE_CONSOLE, CHOOSE_MAIN_MENU]);

        let player_object = object_with(&mut e, &[(PLAYER_HAS_PERK_SLOT, V_PERK_NOT_HELD)]);
        e.set_global(PLAYER, player_object);
        start_log(&mut e);
        assert!(e.call(0x005d_8b60, &args![]).bool());
        assert_eq!(
            calls(&e, TES_GLOBAL_SET_VALUE),
            vec![vec![0x5050, 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn fn_005d8bf0_hands_the_words_the_text_and_a_string_to_the_script_body() {
        let mut e = engine();
        let a = command(&mut e, 0x4444);
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_DESTRUCT, |_, _| Ret::default());
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"hello");
            true.into_ret()
        });
        e.register_double(SCRIPT_BODY_005B4960, |e, a| {
            assert_eq!(e.mem.cstr(a[8]), b"hello");
            true.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8bf0, a));
        let body = calls(&e, SCRIPT_BODY_005B4960)[0].clone();
        assert_eq!(body[..8], args![a][..]);
        let string = body[9];
        assert_eq!(calls(&e, STRING_CONSTRUCT), vec![vec![string]]);
        assert_eq!(calls(&e, STRING_DESTRUCT), vec![vec![string]]);

        // The body's result is the command's.
        e.register(SCRIPT_BODY_005B4960, |_, _| false.into_ret());
        assert!(!run(&mut e, 0x005d_8bf0, a));
        assert_eq!(calls(&e, STRING_DESTRUCT).len(), 2);

        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_8bf0, a));
        assert!(calls(&e, STRING_CONSTRUCT).is_empty());
    }

    #[test]
    fn trigger_enter_sets_the_action_and_runs_the_script_below_depth_five() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(EXTRA_DATA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(INIT_ACTION_LIST, |_, _| Ret::default());
        e.register(SET_ACTION_FLAG, |_, _| Ret::default());
        e.register(SET_ACTION_REF, |_, _| Ret::default());
        let depth_seen = Rc::new(Cell::new(-1));
        let seen = depth_seen.clone();
        e.register_double(RUN_SCRIPT, move |e, _| {
            let tls = e.tls();
            seen.set(e.mem.i32(tls + TLS_SCRIPT_DEPTH));
            Ret::default()
        });
        let tls = e.tls();
        e.mem.set_i32(tls + TLS_SCRIPT_DEPTH, 4);
        parse_gives(&mut e, true, &[0x6666]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8cf0, a));
        let list = this_obj + 0x44;
        assert_eq!(calls(&e, INIT_ACTION_LIST), vec![vec![this_obj, list]]);
        assert_eq!(
            calls(&e, SET_ACTION_FLAG),
            vec![
                vec![0x6666, list, 0x2000_0000],
                vec![0x6666, list, 0x1000_0000]
            ]
        );
        assert_eq!(calls(&e, SET_ACTION_REF), vec![vec![this_obj, 0x6666]]);
        assert_eq!(calls(&e, RUN_SCRIPT), vec![vec![this_obj]]);
        // The depth was raised while the script ran and restored after.
        assert_eq!(depth_seen.get(), 5);
        assert_eq!(e.mem.i32(tls + TLS_SCRIPT_DEPTH), 4);

        // At depth 5 the script is not run (the rest still is).
        e.mem.set_i32(tls + TLS_SCRIPT_DEPTH, 5);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8cf0, a));
        assert!(calls(&e, RUN_SCRIPT).is_empty());
        assert_eq!(calls(&e, SET_ACTION_REF).len(), 1);
        assert_eq!(e.mem.i32(tls + TLS_SCRIPT_DEPTH), 5);

        // No thisObj: only the parse. A failing parse returns false.
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8cf0, none));
        assert!(calls(&e, INIT_ACTION_LIST).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_8cf0, a));
    }

    #[test]
    fn fn_005d8e20_registers_an_ordinary_reference_without_a_file() {
        let mut e = engine();
        let player_object = set_player(&mut e);
        let target = object_with(&mut e, &[(REFERENCE_SLOT_C4, V_TRUE)]);
        e.register(FORM_IS_DELETED_FLAG, |_, _| false.into_ret());
        e.register(GET_FILE, |_, _| 0u32.into_ret());
        e.register_double(LIST_ADD_REFERENCE, move |e, a| {
            // The reference is passed by the address of a word.
            assert_eq!(e.mem.u32(a[1]), target);
            Ret::default()
        });
        let a = command(&mut e, target);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8e20, a));
        assert_eq!(calls(&e, V_TRUE), vec![vec![target, 1]]);
        assert_eq!(calls(&e, GET_FILE), vec![vec![target, 0]]);
        assert_eq!(calls(&e, LIST_ADD_REFERENCE).len(), 1);
        assert_eq!(calls(&e, LIST_ADD_REFERENCE)[0][0], REFERENCE_LIST);

        // A reference with a file is not added.
        e.register(GET_FILE, |_, _| 0xf11eu32.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8e20, a));
        assert!(calls(&e, LIST_ADD_REFERENCE).is_empty());

        // The player, a flagged form and no reference are left alone.
        let for_player = command(&mut e, player_object);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8e20, for_player));
        assert!(calls(&e, GET_FILE).is_empty());
        e.register(FORM_IS_DELETED_FLAG, |_, _| true.into_ret());
        assert!(run(&mut e, 0x005d_8e20, a));
        assert!(calls(&e, GET_FILE).is_empty());
        let none = command(&mut e, 0);
        assert!(run(&mut e, 0x005d_8e20, none));
        assert!(calls(&e, GET_FILE).is_empty());
    }

    // ---- The AddItem-style body ----

    /// An engine for `005d8e80` with the container callees stubbed.
    fn engine_for_add_item() -> Engine {
        let mut e = engine();
        for addr in [
            CONTAINER_CONSTRUCT,
            CONTAINER_ADD_ITEM,
            CONTAINER_FN_00482090,
            CONTAINER_FN_004821A0,
            CONTAINER_DESTRUCT,
            FORM_ADD_LEVELED_ITEMS,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(GET_CALC_LEVEL, |_, _| 0x0001_0007u32.into_ret());
        e.register(LIST_NODE_DATA, |_, a| a[0].into_ret());
        e.register(LIST_NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e
    }

    fn add_item_command(e: &mut Engine, this_obj: u32) -> ScriptArgs {
        let script = object_with(e, &[(SCRIPT_NAME_SLOT, V_NAME)]);
        let mut a = command(e, this_obj);
        a.script_obj = Ptr::new(script);
        a
    }

    #[test]
    fn fn_005d8e80_adds_leveled_items_by_level_and_count() {
        let mut e = engine_for_add_item();
        let a = add_item_command(&mut e, 0x4444);
        let item = object(&mut e);
        e.register(FORM_TYPE, |_, _| 0x34u32.into_ret());
        let quality = 0.5f32.to_bits();
        parse_gives(&mut e, true, &[item, 0x0002_0005, quality, 1]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8e80, a));
        assert_eq!(
            calls(&e, PARSE_PARAMETERS)[0][..7],
            [1, 2, 8, 0x4444, 0, a.script_obj.addr(), 6]
        );
        let container = calls(&e, CONTAINER_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, GET_CALC_LEVEL), vec![vec![0x4444, 0]]);
        assert_eq!(
            calls(&e, FORM_ADD_LEVELED_ITEMS),
            vec![vec![item + 0x30, 7, 5, container, 0]]
        );
        assert_eq!(
            calls(&e, CONTAINER_FN_00482090),
            vec![vec![container, quality]]
        );
        assert_eq!(
            calls(&e, CONTAINER_FN_004821A0),
            vec![vec![container, 0x4444, 1]]
        );
        assert_eq!(calls(&e, CONTAINER_DESTRUCT), vec![vec![container]]);
        assert!(calls(&e, LOG_STUB).is_empty());

        // The integer decides the flag: anything but 1 passes 0.
        parse_gives(&mut e, true, &[item, 5, quality, 2]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8e80, a));
        assert_eq!(calls(&e, CONTAINER_FN_004821A0)[0][2], 0);
    }

    #[test]
    fn fn_005d8e80_adds_the_entries_of_a_list_that_pass_the_test() {
        let mut e = engine_for_add_item();
        let a = add_item_command(&mut e, 0x4444);
        let list_form = object(&mut e);
        let passing = object_with(&mut e, &[(FORM_TEST_SLOT, V_TRUE)]);
        let failing = object_with(&mut e, &[(FORM_TEST_SLOT, V_FALSE)]);
        // Nodes `[data][next]`: passing, failing, then an empty node that
        // ends the walk.
        let n3 = e.mem.alloc(8);
        let n2 = e.mem.alloc(8);
        e.mem.set_u32(n2, failing);
        e.mem.set_u32(n2 + 4, n3);
        let n1 = e.mem.alloc(8);
        e.mem.set_u32(n1, passing);
        e.mem.set_u32(n1 + 4, n2);
        // The first node of the item's list: `item + 0x18` in the game.
        e.register_double(LIST_AT_18, move |_, _| n1.into_ret());
        e.register(FORM_TYPE, |_, _| 0x55u32.into_ret());
        parse_gives(&mut e, true, &[list_form, 3, 0, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8e80, a));
        let container = calls(&e, CONTAINER_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, LIST_AT_18), vec![vec![list_form]]);
        assert_eq!(
            calls(&e, CONTAINER_ADD_ITEM),
            vec![vec![container, passing, 3, 0]]
        );
        assert!(calls(&e, LOG_STUB).is_empty());

        // Without a count the list is not used and the failure is logged.
        parse_gives(&mut e, true, &[list_form, 0, 0, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8e80, a));
        assert!(calls(&e, CONTAINER_ADD_ITEM).is_empty());
        assert_eq!(calls(&e, LOG_STUB), vec![vec![MSG_ADD_ITEM_FAILED, 0xbbbb]]);
        // The container is still applied and destroyed.
        assert_eq!(calls(&e, CONTAINER_FN_004821A0).len(), 1);
        assert_eq!(calls(&e, CONTAINER_DESTRUCT).len(), 1);
    }

    #[test]
    fn fn_005d8e80_adds_a_single_item_that_passes_the_test() {
        let mut e = engine_for_add_item();
        let a = add_item_command(&mut e, 0x4444);
        e.register(FORM_TYPE, |_, _| 0x28u32.into_ret());
        let item = object_with(&mut e, &[(FORM_TEST_SLOT, V_TRUE)]);
        parse_gives(&mut e, true, &[item, 4, 0, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8e80, a));
        let container = calls(&e, CONTAINER_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, CONTAINER_ADD_ITEM),
            vec![vec![container, item, 4, 0]]
        );
        assert!(calls(&e, LOG_STUB).is_empty());

        // A zero count: logged. An item that fails the test: logged.
        parse_gives(&mut e, true, &[item, 0, 0, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8e80, a));
        assert!(calls(&e, CONTAINER_ADD_ITEM).is_empty());
        assert_eq!(calls(&e, LOG_STUB).len(), 1);
        let refusing = object_with(&mut e, &[(FORM_TEST_SLOT, V_FALSE)]);
        parse_gives(&mut e, true, &[refusing, 4, 0, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8e80, a));
        assert!(calls(&e, CONTAINER_ADD_ITEM).is_empty());
        assert_eq!(calls(&e, LOG_STUB), vec![vec![MSG_ADD_ITEM_FAILED, 0xbbbb]]);
    }

    #[test]
    fn fn_005d8e80_needs_a_reference_and_parsed_parameters() {
        let mut e = engine_for_add_item();
        let none = add_item_command(&mut e, 0);
        parse_gives(&mut e, true, &[1, 1, 0, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_8e80, none));
        assert!(calls(&e, CONTAINER_CONSTRUCT).is_empty());
        assert!(calls(&e, GET_CALC_LEVEL).is_empty());
        let a = add_item_command(&mut e, 0x4444);
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_8e80, a));
    }

    #[test]
    fn fn_005d90a0_stores_the_id_of_the_form_the_script_body_makes() {
        let mut e = engine();
        let a = command(&mut e, 0x4444);
        e.mem.set_f64(a.result.addr(), 9.0);
        let made = object(&mut e);
        e.mem.set_u32(made + 0x0c, 0x0abc);
        e.register(PUT_NUMERIC_ID_IN_DOUBLE, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_f64(a[1], id as f64);
            Ret::default()
        });
        e.register_double(SCRIPT_BODY_005C4B30, move |_, _| made.into_ret());
        let scale = 2.0f32.to_bits();
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            // The documented defaults are in the locals before the parse.
            assert_eq!(e.mem.u32(a[7]), 0);
            assert_eq!(e.mem.u32(a[8]), 1.0f32.to_bits());
            assert_eq!(e.mem.u32(a[9]), 1);
            assert_eq!(e.mem.u32(a[10]), 0);
            assert_eq!(e.mem.u32(a[11]), 0);
            e.mem.set_u32(a[7], 0x11);
            e.mem.set_u32(a[8], scale);
            e.mem.set_u32(a[9], 5);
            e.mem.set_u32(a[10], 6);
            e.mem.set_u32(a[11], 7);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_90a0, a));
        assert_eq!(
            calls(&e, SCRIPT_BODY_005C4B30),
            vec![vec![0x4444, 0x11, 5, 6, 7, scale]]
        );
        assert_eq!(e.mem.f64(a.result.addr()), 0x0abc as f64);

        // Nothing made: the result stays 0.0.
        e.register(SCRIPT_BODY_005C4B30, |_, _| 0u32.into_ret());
        e.mem.set_f64(a.result.addr(), 9.0);
        assert!(run(&mut e, 0x005d_90a0, a));
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);

        // A failing parse still clears the result.
        e.mem.set_f64(a.result.addr(), 9.0);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_90a0, a));
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert!(calls(&e, SCRIPT_BODY_005C4B30).is_empty());
    }

    // ---- The functions from 005d9160 ----

    const V_RECORD: u32 = 0x0900_0010;
    const V_WORD: u32 = 0x0900_0011;

    fn engine_p7b() -> Engine {
        let mut e = engine();
        e.map(0x0101_7000, 0x1000);
        e.set_global(HUNDRED, 100.0f64);
        e.register(V_RECORD, |_, _| Ret::default());
        e.register(V_WORD, |_, _| 0x5151u32.into_ret());
        e
    }

    /// The two argument words of a `double` pushed on the stack.
    fn f64_words(value: f64) -> [u32; 2] {
        let bits = value.to_bits();
        [bits as u32, (bits >> 32) as u32]
    }

    /// A command that is a condition function with no argument: the callee
    /// gets `(thisObj, 0, 0, result)` and its `AL` is the answer.
    fn assert_plain_condition(address: u32, callee: u32) {
        let mut e = engine_p7b();
        let a = command(&mut e, 0x4444);
        e.register(callee, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, address, a));
        assert_eq!(calls(&e, callee), vec![vec![0x4444, 0, 0, a.result.addr()]]);
        e.register(callee, |_, _| false.into_ret());
        assert!(!run(&mut e, address, a));
    }

    /// A command that parses one word (initialised to `default`) and hands it
    /// to a condition function, whatever `thisObj` is.
    fn assert_parsed_condition(address: u32, callee: u32, default: u32) {
        let mut e = engine_p7b();
        let a = command(&mut e, 0x4444);
        e.register_double(PARSE_PARAMETERS, move |e, args| {
            assert_eq!(e.mem.u32(args[7]), default);
            e.mem.set_u32(args[7], 0x77);
            true.into_ret()
        });
        e.register(callee, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, address, a));
        assert_parsed(&e, 0x4444);
        assert_eq!(
            calls(&e, callee),
            vec![vec![0x4444, 0x77, 0, a.result.addr()]]
        );
        e.register(callee, |_, _| false.into_ret());
        assert!(!run(&mut e, address, a));

        // Parameters that do not parse: nothing is called.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, address, a));
        assert!(calls(&e, callee).is_empty());
    }

    /// `005d9270` and `005d92e0`: a condition function for one parsed word,
    /// skipped (the command succeeds) without a `thisObj` or with a zero word.
    fn assert_parsed_condition_with_reference(address: u32, callee: u32) {
        let mut e = engine_p7b();
        let a = command(&mut e, 0x4444);
        e.register(callee, |_, _| false.into_ret());
        parse_gives(&mut e, true, &[0x77]);
        start_log(&mut e);
        // The condition's answer is the command's answer.
        assert!(!run(&mut e, address, a));
        assert_parsed(&e, 0x4444);
        assert_eq!(
            calls(&e, callee),
            vec![vec![0x4444, 0x77, 0, a.result.addr()]]
        );
        e.register(callee, |_, _| true.into_ret());
        assert!(run(&mut e, address, a));

        // No thisObj, or a zero word: success without the condition.
        e.register(callee, |_, _| false.into_ret());
        let without = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, address, without));
        parse_gives(&mut e, true, &[0]);
        assert!(run(&mut e, address, a));
        assert!(calls(&e, callee).is_empty());

        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, address, a));
    }

    #[test]
    fn fn_005d9160_closes_all_menus() {
        let mut e = engine_p7b();
        let a = command(&mut e, 0);
        e.register(EMERGENCY_CLOSE_ALL_MENUS, |_, _| Ret::default());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9160, a));
        assert_eq!(
            calls(&e, EMERGENCY_CLOSE_ALL_MENUS),
            vec![Vec::<u32>::new()]
        );
    }

    #[test]
    fn script_set_console_output_file_function_sets_or_clears_the_file_name() {
        let mut e = engine_p7b();
        let a = command(&mut e, 0);
        e.register(MENU_CONSOLE_INSTANCE, |_, _| 0xc0de_u32.into_ret());
        let names = Rc::new(RefCell::new(Vec::<(u32, Option<Vec<u8>>)>::new()));
        let sink = names.clone();
        e.register_double(CONSOLE_SET_OUTPUT_FILE, move |e, args| {
            let text = (args[1] != 0).then(|| e.mem.cstr(args[1]));
            sink.borrow_mut().push((args[0], text));
            Ret::default()
        });
        let parse_text = |e: &mut Engine, text: &'static str| {
            e.register_double(PARSE_PARAMETERS, move |e, args| {
                e.mem.set_cstr(args[7], text.as_bytes());
                true.into_ret()
            });
        };

        // A name: the console gets it, then the echo shows it.
        parse_text(&mut e, "out.txt");
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9170, a));
        assert_parsed(&e, 0);
        assert_eq!(calls(&e, MENU_CONSOLE_INSTANCE), vec![vec![1]]);
        assert_eq!(
            names.borrow().as_slice(),
            &[(0xc0de, Some(b"out.txt".to_vec()))]
        );
        let printed = calls(&e, CONSOLE_PRINT);
        assert_eq!(printed.len(), 1);
        assert_eq!(printed[0][0], MSG_CONSOLE_OUTPUT_FILE);

        // "0" and the empty text disable it (the echo comes first).
        for text in ["0", ""] {
            names.borrow_mut().clear();
            parse_text(&mut e, text);
            start_log(&mut e);
            assert!(run(&mut e, 0x005d_9170, a));
            assert_eq!(names.borrow().as_slice(), &[(0xc0de, None)]);
            assert_eq!(
                calls(&e, CONSOLE_PRINT),
                vec![vec![MSG_CONSOLE_OUTPUT_DISABLED]]
            );
        }

        // Without the echo flag nothing is printed.
        set_echo(&mut e, false);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9170, a));
        assert!(calls(&e, CONSOLE_PRINT).is_empty());

        // Parameters that do not parse: the console is left alone.
        parse_gives(&mut e, false, &[]);
        names.borrow_mut().clear();
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_9170, a));
        assert!(names.borrow().is_empty());
        assert!(calls(&e, MENU_CONSOLE_INSTANCE).is_empty());
    }

    #[test]
    fn script_get_faction_relation_function_asks_the_condition_for_a_form() {
        assert_parsed_condition_with_reference(0x005d_9270, GET_FACTION_RELATION_CONDITION);
    }

    #[test]
    fn script_is_last_played_idle_function_asks_the_condition_for_a_form() {
        assert_parsed_condition_with_reference(0x005d_92e0, IS_LAST_PLAYED_IDLE_CONDITION);
    }

    #[test]
    fn fn_005d9350_creates_the_race_sex_menu() {
        let mut e = engine_p7b();
        let a = command(&mut e, 0);
        e.register(CREATE_RACE_SEX_MENU, |_, _| Ret::default());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9350, a));
        assert_eq!(calls(&e, CREATE_RACE_SEX_MENU), vec![vec![0]]);
    }

    #[test]
    fn script_set_player_teammate_function_sets_the_flag_on_actors() {
        let mut e = engine_p7b();
        e.register(ACTOR_SET_PLAYER_TEAMMATE, |_, _| Ret::default());
        let actor = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_TRUE)]);
        let a = command(&mut e, actor);

        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9370, a));
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, ACTOR_SET_PLAYER_TEAMMATE), vec![vec![actor, 1]]);

        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9370, a));
        assert_eq!(calls(&e, ACTOR_SET_PLAYER_TEAMMATE), vec![vec![actor, 0]]);

        // Not an actor, or no thisObj: still a success, nothing set.
        let thing = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_FALSE)]);
        let not_actor = command(&mut e, thing);
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9370, not_actor));
        assert!(run(&mut e, 0x005d_9370, none));
        assert!(calls(&e, ACTOR_SET_PLAYER_TEAMMATE).is_empty());

        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_9370, a));
    }

    #[test]
    fn fn_005d93f0_asks_the_player_teammate_condition() {
        assert_plain_condition(0x005d_93f0, GET_PLAYER_TEAMMATE_CONDITION);
    }

    #[test]
    fn fn_005d9410_asks_the_player_teammate_count_condition() {
        assert_plain_condition(0x005d_9410, GET_PLAYER_TEAMMATE_COUNT_CONDITION);
    }

    #[test]
    fn fn_005d9430_queues_the_menu_when_forced_or_asked() {
        let mut e = engine_p7b();
        e.register(QUEUE_MENU_CREATE, |_, _| Ret::default());
        let actor = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_TRUE)]);
        let a = command(&mut e, actor);
        let force = Rc::new(Cell::new(false));
        let forced = force.clone();
        e.register_double(ACTOR_FORCE_NEXT_UPDATE, move |_, _| forced.get().into_ret());

        // Neither forced nor asked.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9430, a));
        assert_parsed(&e, actor);
        assert!(calls(&e, QUEUE_MENU_CREATE).is_empty());

        // Asked: even when the parse reports failure the local is used.
        parse_gives(&mut e, false, &[7]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9430, a));
        assert_eq!(
            calls(&e, QUEUE_MENU_CREATE),
            vec![vec![1, actor, 0, 0, 3, 0]]
        );

        // Forced.
        force.set(true);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9430, a));
        assert_eq!(calls(&e, QUEUE_MENU_CREATE).len(), 1);

        // Not an actor / no thisObj: the parse is not even done.
        let thing = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_FALSE)]);
        let not_actor = command(&mut e, thing);
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9430, not_actor));
        assert!(run(&mut e, 0x005d_9430, none));
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        assert!(calls(&e, QUEUE_MENU_CREATE).is_empty());
    }

    #[test]
    fn fn_005d94c0_asks_the_minor_crime_count_condition() {
        assert_plain_condition(0x005d_94c0, GET_MINOR_CRIME_COUNT_CONDITION);
    }

    #[test]
    fn fn_005d94e0_asks_the_major_crime_count_condition() {
        assert_plain_condition(0x005d_94e0, GET_MAJOR_CRIME_COUNT_CONDITION);
    }

    #[test]
    fn fn_005d9500_passes_the_parsed_value_on_with_zero() {
        let mut e = engine_p7b();
        e.register(FLAG_SETTER_0047EB90, |_, _| Ret::default());
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0x55]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9500, a));
        assert_parsed(&e, 0);
        assert_eq!(calls(&e, FLAG_SETTER_0047EB90), vec![vec![0x55, 0]]);

        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_9500, a));
        assert!(calls(&e, FLAG_SETTER_0047EB90).is_empty());
    }

    #[test]
    fn fn_005d9550_calls_the_actor_function_on_actors_only() {
        let mut e = engine_p7b();
        e.register(ACTOR_FN_008BCB40, |_, _| Ret::default());
        let actor = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_TRUE)]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9550, a));
        assert_eq!(calls(&e, ACTOR_FN_008BCB40), vec![vec![actor]]);

        let thing = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_FALSE)]);
        let not_actor = command(&mut e, thing);
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9550, not_actor));
        assert!(run(&mut e, 0x005d_9550, none));
        assert!(calls(&e, ACTOR_FN_008BCB40).is_empty());
    }

    #[test]
    fn fn_005d9590_asks_the_actor_crime_player_enemy_condition() {
        assert_plain_condition(0x005d_9590, GET_ACTOR_CRIME_PLAYER_ENEMY_CONDITION);
    }

    #[test]
    fn fn_005d95b0_asks_the_actor_faction_player_enemy_condition() {
        assert_plain_condition(0x005d_95b0, GET_ACTOR_FACTION_PLAYER_ENEMY_CONDITION);
    }

    #[test]
    fn script_set_player_tag_skill_function_sets_a_tag_skill_in_range() {
        let mut e = engine_p7b();
        let player = set_player(&mut e);
        e.register(VALUE_IN_RANGE_20_TO_2D, |_, args| {
            (0x20..0x2e).contains(&args[0]).into_ret()
        });
        e.register(ACTOR_GET_CLASS, |_, _| 0xc1a5_u32.into_ret());
        e.register(CLASS_SET_TAG_SKILL, |_, _| Ret::default());
        let a = command(&mut e, 0);
        // The documented defaults are in the locals before the parse.
        let parsed = Rc::new(RefCell::new((0x25u32, 2u32)));
        let values = parsed.clone();
        e.register_double(PARSE_PARAMETERS, move |e, args| {
            assert_eq!(e.mem.u32(args[7]), u32::MAX);
            assert_eq!(e.mem.u32(args[8]), 0);
            e.mem.set_u32(args[7], values.borrow().0);
            e.mem.set_u32(args[8], values.borrow().1);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_95d0, a));
        assert_eq!(calls(&e, ACTOR_GET_CLASS), vec![vec![player]]);
        assert_eq!(calls(&e, CLASS_SET_TAG_SKILL), vec![vec![0xc1a5, 2, 0x25]]);

        // An index of 4 or more, a negative one, or a skill out of range.
        for (skill, index) in [(0x25, 4), (0x25, u32::MAX), (0x2e, 1), (0x1f, 1)] {
            *parsed.borrow_mut() = (skill, index);
            start_log(&mut e);
            assert!(run(&mut e, 0x005d_95d0, a));
            assert!(calls(&e, CLASS_SET_TAG_SKILL).is_empty(), "{skill} {index}");
        }
        *parsed.borrow_mut() = (0x20, 3);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_95d0, a));
        assert_eq!(calls(&e, CLASS_SET_TAG_SKILL), vec![vec![0xc1a5, 3, 0x20]]);

        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_95d0, a));
        assert!(calls(&e, CLASS_SET_TAG_SKILL).is_empty());
    }

    #[test]
    fn fn_005d9660_is_four() {
        let mut e = engine_p7b();
        assert_eq!(e.call(0x005d_9660, &args![]).u32(), 4);
    }

    #[test]
    fn script_is_player_tag_skill_function_reports_a_tag_skill() {
        let mut e = engine_p7b();
        let player = set_player(&mut e);
        e.register(VALUE_IN_RANGE_20_TO_2D, |_, args| {
            (0x20..0x2e).contains(&args[0]).into_ret()
        });
        e.register(ACTOR_GET_CLASS, |_, _| 0xc1a5_u32.into_ret());
        e.register(CLASS_IS_TAG_SKILL, |_, args| (args[1] == 0x22).into_ret());
        let a = command(&mut e, 0);
        let skill = Rc::new(Cell::new(0x22u32));
        let wanted = skill.clone();
        e.register_double(PARSE_PARAMETERS, move |e, args| {
            assert_eq!(e.mem.u32(args[7]), u32::MAX);
            e.mem.set_u32(args[7], wanted.get());
            true.into_ret()
        });
        set_echo(&mut e, true);

        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9670, a));
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        assert_eq!(calls(&e, ACTOR_GET_CLASS), vec![vec![player]]);
        assert_eq!(calls(&e, CLASS_IS_TAG_SKILL), vec![vec![0xc1a5, 0x22]]);
        let [low, high] = f64_words(1.0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_IS_PLAYER_TAG_SKILL, low, high]]
        );

        // A skill the class does not tag.
        skill.set(0x23);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9670, a));
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);

        // Out of range: the class is not even asked; the echo shows 0.
        skill.set(0x40);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9670, a));
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert!(calls(&e, ACTOR_GET_CLASS).is_empty());
        let [low, high] = f64_words(0.0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_IS_PLAYER_TAG_SKILL, low, high]]
        );

        // No echo flag, and a failed parse leaves the result alone.
        set_echo(&mut e, false);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9670, a));
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        e.mem.set_f64(a.result.addr(), 9.0);
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_9670, a));
        assert_eq!(e.mem.f64(a.result.addr()), 9.0);
    }

    #[test]
    fn script_get_player_grabbed_ref_function_stores_the_grabbed_form_id() {
        let mut e = engine_p7b();
        let player = set_player(&mut e);
        let grabbed = object(&mut e);
        e.mem.set_u32(grabbed + 0x0c, 0x1234);
        let held = Rc::new(Cell::new(grabbed));
        let current = held.clone();
        e.register_double(PLAYER_GRABBED_REF, move |_, _| current.get().into_ret());
        e.register(PUT_NUMERIC_ID_IN_DOUBLE, |e, args| {
            let id = e.mem.u32(args[0]);
            e.mem.set_f64(args[1], id as f64);
            Ret::default()
        });
        let a = command(&mut e, 0);
        set_echo(&mut e, true);
        e.mem.set_f64(a.result.addr(), 9.0);

        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9730, a));
        assert_eq!(e.mem.f64(a.result.addr()), 0x1234 as f64);
        assert_eq!(
            calls(&e, PLAYER_GRABBED_REF),
            vec![vec![player], vec![player]]
        );
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_PLAYER_GRABBED_REF, 0x1234]]
        );

        // Nothing grabbed: 0.0, asked once, the echo shows 0.
        held.set(0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9730, a));
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert_eq!(calls(&e, PLAYER_GRABBED_REF).len(), 1);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_PLAYER_GRABBED_REF, 0]]
        );
        set_echo(&mut e, false);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9730, a));
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn fn_005d97b0_asks_a_condition_for_a_parsed_value() {
        assert_parsed_condition(0x005d_97b0, CONDITION_FN_005A4C20, 0);
    }

    /// The doubles and objects `005d9810` needs.
    struct Spawn {
        this_obj: u32,
        made: u32,
        form: u32,
        handler: u32,
        form_type: Rc<Cell<u32>>,
        base_data_test: Rc<Cell<bool>>,
        cell_flag: Rc<Cell<bool>>,
    }

    fn spawn_engine() -> (Engine, Spawn) {
        let mut e = engine_p7b();
        let this_obj = object_with(&mut e, &[(REFERENCE_SLOT_1F4, V_WORD)]);
        let made = object_with(
            &mut e,
            &[(FORM_SLOT_48, V_RECORD), (ACTOR_SLOT_228, V_RECORD)],
        );
        e.mem.set_u32(made + 0x0c, 0x0abc);
        let form = object_with(&mut e, &[(FORM_IS_ACTOR_BASE_SLOT, V_TRUE)]);
        let handler = 0x000d_a7a0;
        e.set_global(LIST_OWNER, handler);
        let form_type = Rc::new(Cell::new(0x2au32));
        let base_data_test = Rc::new(Cell::new(true));
        let cell_flag = Rc::new(Cell::new(true));
        let current = form_type.clone();
        e.register_double(FORM_TYPE, move |_, _| current.get().into_ret());
        e.register(OPERATOR_NEW, |_, _| 0xb10c_u32.into_ret());
        e.register_double(CHARACTER_CONSTRUCT, move |_, _| made.into_ret());
        e.register_double(CREATURE_CONSTRUCT, move |_, _| made.into_ret());
        e.register(REFERENCE_SET_ENCOUNTER_ZONE, |_, _| Ret::default());
        e.register(EXTRA_DATA_LIST, |_, _| 0x1157_u32.into_ret());
        e.register(EXTRA_LIST_SET_COUNT, |_, _| Ret::default());
        e.register(REFERENCE_PARENT_CELL, |_, _| 0xce11_u32.into_ret());
        e.register(REFERENCE_SET_OBJECT_REFERENCE, |_, _| Ret::default());
        let test = base_data_test.clone();
        e.register_double(ACTOR_BASE_DATA_TEST, move |_, _| test.get().into_ret());
        e.register(ACTOR_BASE_DATA_APPLY, |_, _| Ret::default());
        let flag = cell_flag.clone();
        e.register_double(CELL_FLAG_BIT_0, move |_, _| flag.get().into_ret());
        e.register(REFERENCE_GET_WORLD_SPACE, |_, _| 0x1717_u32.into_ret());
        e.register(REFERENCE_FIELD_24_ADDRESS, |_, _| 0x2424_u32.into_ret());
        e.register(WORD_AT_20, |_, _| 0x2020_u32.into_ret());
        e.register(DATA_HANDLER_FN_004698A0, |_, _| Ret::default());
        e.register(DATA_HANDLER_FN_0046A010, |_, _| Ret::default());
        e.register(PUT_NUMERIC_ID_IN_DOUBLE, |e, args| {
            let id = e.mem.u32(args[0]);
            e.mem.set_f64(args[1], id as f64);
            Ret::default()
        });
        (
            e,
            Spawn {
                this_obj,
                made,
                form,
                handler,
                form_type,
                base_data_test,
                cell_flag,
            },
        )
    }

    #[test]
    fn fn_005d9810_builds_a_character_and_places_it() {
        let (mut e, s) = spawn_engine();
        let a = command(&mut e, s.this_obj);
        e.mem.set_f64(a.result.addr(), 9.0);
        // The documented defaults are in the locals before the parse.
        let form = s.form;
        e.register_double(PARSE_PARAMETERS, move |e, args| {
            assert_eq!(e.mem.u32(args[7]), 0);
            assert_eq!(e.mem.u32(args[8]), 4);
            assert_eq!(e.mem.u32(args[9]), 0);
            e.mem.set_u32(args[7], form);
            e.mem.set_u32(args[8], 7);
            e.mem.set_u32(args[9], 0x20ae);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9810, a));
        assert_parsed(&e, s.this_obj);
        let made = s.made;
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x1c8]]);
        assert_eq!(calls(&e, CHARACTER_CONSTRUCT), vec![vec![0xb10c]]);
        assert!(calls(&e, CREATURE_CONSTRUCT).is_empty());
        assert_eq!(
            calls(&e, REFERENCE_SET_ENCOUNTER_ZONE),
            vec![vec![made, 0x20ae]]
        );
        assert_eq!(calls(&e, EXTRA_DATA_LIST), vec![vec![made]]);
        assert_eq!(calls(&e, EXTRA_LIST_SET_COUNT), vec![vec![0x1157, 7]]);
        assert_eq!(
            calls(&e, V_RECORD),
            vec![vec![made, 0x4000_0000], vec![made, 0xce11]]
        );
        assert_eq!(
            calls(&e, REFERENCE_SET_OBJECT_REFERENCE),
            vec![vec![made, form]]
        );
        assert_eq!(calls(&e, ACTOR_BASE_DATA_TEST), vec![vec![form + 0x30]]);
        assert_eq!(
            calls(&e, ACTOR_BASE_DATA_APPLY),
            vec![vec![form + 0x30, made]]
        );
        assert_eq!(calls(&e, CELL_FLAG_BIT_0), vec![vec![0xce11]]);
        assert_eq!(
            calls(&e, DATA_HANDLER_FN_004698A0),
            vec![vec![
                s.handler, 0x2020, 0x5151, 0x2424, 0xce11, 0x1717, made, 0, 0
            ]]
        );
        assert_eq!(calls(&e, DATA_HANDLER_FN_0046A010), vec![vec![made, 0]]);
        assert_eq!(e.mem.f64(a.result.addr()), 0x0abc as f64);
    }

    #[test]
    fn fn_005d9810_builds_a_creature_and_adapts_to_the_flags() {
        let (mut e, s) = spawn_engine();
        let a = command(&mut e, s.this_obj);
        parse_gives(&mut e, true, &[s.form, 3, 0]);
        s.form_type.set(0x2b);
        s.base_data_test.set(false);
        s.cell_flag.set(false);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9810, a));
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x1c0]]);
        assert_eq!(calls(&e, CREATURE_CONSTRUCT), vec![vec![0xb10c]]);
        assert!(calls(&e, CHARACTER_CONSTRUCT).is_empty());
        // No zone given, no base data applied, the cell dropped.
        assert!(calls(&e, REFERENCE_SET_ENCOUNTER_ZONE).is_empty());
        assert!(calls(&e, ACTOR_BASE_DATA_APPLY).is_empty());
        assert_eq!(
            calls(&e, DATA_HANDLER_FN_004698A0)[0][4],
            0,
            "the cell word"
        );
        // The cell passed to slot 0x228 is the one before the flag test.
        assert_eq!(calls(&e, V_RECORD)[1], vec![s.made, 0xce11]);
    }

    #[test]
    fn fn_005d9810_makes_nothing_for_other_forms_or_without_a_reference() {
        let (mut e, s) = spawn_engine();
        let a = command(&mut e, s.this_obj);
        e.mem.set_f64(a.result.addr(), 9.0);

        // A type that is neither character nor creature.
        parse_gives(&mut e, true, &[s.form, 3, 0]);
        s.form_type.set(0x28);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9810, a));
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);

        // A form that is not an actor base, a null form, no thisObj.
        s.form_type.set(0x2a);
        let thing = object_with(&mut e, &[(FORM_IS_ACTOR_BASE_SLOT, V_FALSE)]);
        parse_gives(&mut e, true, &[thing, 3, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9810, a));
        parse_gives(&mut e, true, &[0, 3, 0]);
        assert!(run(&mut e, 0x005d_9810, a));
        parse_gives(&mut e, true, &[s.form, 3, 0]);
        let none = command(&mut e, 0);
        assert!(run(&mut e, 0x005d_9810, none));
        assert!(calls(&e, OPERATOR_NEW).is_empty());

        // The allocation failing: nothing more happens.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert!(run(&mut e, 0x005d_9810, a));
        assert!(calls(&e, EXTRA_DATA_LIST).is_empty());

        // Parameters that do not parse still clear the result.
        e.mem.set_f64(a.result.addr(), 9.0);
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_9810, a));
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
    }

    #[test]
    fn fn_005d9aa0_asks_the_destruction_stage_condition() {
        assert_plain_condition(0x005d_9aa0, GET_DESTRUCTION_STAGE_CONDITION);
    }

    #[test]
    fn script_force_active_quest_function_sets_the_players_active_quest() {
        let mut e = engine_p7b();
        let player = set_player(&mut e);
        e.register(PLAYER_SET_ACTIVE_QUEST, |_, _| Ret::default());
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0x9999]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9ac0, a));
        assert_parsed(&e, 0);
        assert_eq!(
            calls(&e, PLAYER_SET_ACTIVE_QUEST),
            vec![vec![player, 0x9999]]
        );

        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9ac0, a));
        assert!(calls(&e, PLAYER_SET_ACTIVE_QUEST).is_empty());

        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_9ac0, a));
    }

    /// The doubles `005d9bc0` needs: an actor and a source NPC (form words
    /// `0xaa00` and `0x5500`), the NPC objects they cast to, a list whose
    /// first element is `chosen`, and recorders for everything else.
    struct Blend {
        actor: u32,
        source: u32,
        actor_npc: u32,
        source_npc: u32,
        chosen: u32,
        slot: u32,
        raw: u32,
    }

    fn blend_engine() -> (Engine, Blend) {
        let mut e = engine_p7b();
        let actor = object_with(&mut e, &[(REFERENCE_SLOT_1D0, V_WORD)]);
        let source = object(&mut e);
        let actor_npc = object_with(&mut e, &[(FORM_SLOT_48, V_RECORD)]);
        let source_npc = object(&mut e);
        let chosen = object(&mut e);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, chosen);
        let raw = e.mem.alloc(0x84);
        e.register_double(WORD_AT_20, move |_, args| {
            if args[0] == actor {
                0xaa00u32.into_ret()
            } else {
                assert_eq!(args[0], source);
                0x5500u32.into_ret()
            }
        });
        e.register_double(DYNAMIC_CAST, move |_, args| {
            assert_eq!(args[1..], [0, RTTI_TES_BOUND_OBJECT, RTTI_TES_NPC, 0]);
            if args[0] == 0xaa00 {
                actor_npc.into_ret()
            } else {
                source_npc.into_ret()
            }
        });
        for function in [
            VECTOR_CONSTRUCTOR_ITERATOR,
            VECTOR_DESTRUCTOR_ITERATOR,
            COORD_LIST_CONSTRUCT,
            COORD_LIST_DESTRUCT,
            COLLECT_MATCHING_NPCS,
            FACEGEN_FN_00652E00,
            INIT_FACEGEN_COORD,
            FACEGEN_FN_00653000,
            FACEGEN_FN_00652AF0,
            FACEGEN_FN_00652470,
            COPY_FACEGEN_COORD,
            CHARACTER_RESET_3D,
            OPERATOR_DELETE,
            COORD_BLOCK_DESTRUCT,
        ] {
            e.register(function, |_, _| Ret::default());
        }
        e.register(ACTOR_BASE_GET_SEX, |_, _| 1u32.into_ret());
        e.register(ACTOR_BASE_RACE, |_, _| 0x5000u32.into_ret());
        e.register_double(ARRAY_ELEMENT_ADDRESS, move |_, _| slot.into_ret());
        e.register_double(NPC_COORDS, move |_, args| {
            if args[0] == actor_npc {
                0xc0a0u32.into_ret()
            } else if args[0] == source_npc {
                0xc050u32.into_ret()
            } else {
                0xc0c0u32.into_ret()
            }
        });
        e.register(FACEGEN_FN_00652440, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.register_double(OPERATOR_NEW, move |_, _| raw.into_ret());
        (
            e,
            Blend {
                actor,
                source,
                actor_npc,
                source_npc,
                chosen,
                slot,
                raw,
            },
        )
    }

    #[test]
    fn fn_005d9bc0_blends_the_face_of_the_chosen_npc_into_the_actor() {
        let (mut e, b) = blend_engine();
        start_log(&mut e);
        e.call(0x005d_9bc0, &args![b.actor, b.source, 50i32]);
        let constructed = calls(&e, VECTOR_CONSTRUCTOR_ITERATOR);
        assert_eq!(constructed.len(), 3);
        let (blocks_a, blocks_b) = (constructed[0][0], constructed[1][0]);
        let copy = b.raw + 4;
        assert_eq!(
            constructed[0][1..],
            [0x20, 4, COORD_BLOCK_CONSTRUCT, COORD_BLOCK_DESTRUCT]
        );
        assert_eq!(
            constructed[2],
            vec![copy, 0x20, 4, COORD_BLOCK_CONSTRUCT, COORD_BLOCK_DESTRUCT]
        );
        assert_eq!(e.mem.u32(b.raw), 4, "the count before the first block");
        let list = calls(&e, COORD_LIST_CONSTRUCT)[0][0];

        // The NPCs of the source's race and sex are collected.
        assert_eq!(calls(&e, ACTOR_BASE_GET_SEX), vec![vec![b.source_npc]]);
        assert_eq!(
            calls(&e, ACTOR_BASE_RACE),
            vec![vec![b.source_npc], vec![b.actor_npc]]
        );
        assert_eq!(
            calls(&e, COLLECT_MATCHING_NPCS),
            vec![vec![0x5000, 1, list]]
        );
        assert_eq!(
            calls(&e, NPC_COORDS),
            vec![vec![b.actor_npc], vec![b.source_npc], vec![b.chosen]]
        );
        assert_eq!(
            calls(&e, FACEGEN_FN_00652E00),
            vec![
                vec![0xc0c0, 0xc050, blocks_b, 0],
                vec![0x5000 + 0x478, blocks_a, copy, 0]
            ]
        );
        assert_eq!(calls(&e, FACEGEN_FN_00652440), vec![vec![0xc0a0, 0, 0]]);
        assert_eq!(
            calls(&e, INIT_FACEGEN_COORD),
            vec![vec![blocks_a], vec![copy]]
        );
        assert_eq!(
            calls(&e, FACEGEN_FN_00653000),
            vec![vec![0.5f32.to_bits(), blocks_b, 1]]
        );
        assert_eq!(
            calls(&e, FACEGEN_FN_00652AF0),
            vec![vec![0xc0a0, blocks_b, blocks_a, 1, 0.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, FACEGEN_FN_00652470),
            vec![vec![blocks_a, 0, 0, 1.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, COPY_FACEGEN_COORD),
            vec![vec![0xc0a0, copy, 0, 1]]
        );

        // The copy is the actor NPC's own array; its slot 0x48 gets 0x800.
        assert_eq!(e.mem.u32(b.actor_npc + 0x1b4), copy);
        assert_eq!(calls(&e, V_RECORD), vec![vec![b.actor_npc, 0x800]]);
        assert_eq!(calls(&e, CHARACTER_RESET_3D), vec![vec![b.actor]]);

        // Everything built on the stack is destroyed.
        assert_eq!(calls(&e, COORD_LIST_DESTRUCT), vec![vec![list]]);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![
                vec![blocks_b, 0x20, 4, COORD_BLOCK_DESTRUCT],
                vec![blocks_a, 0x20, 4, COORD_BLOCK_DESTRUCT]
            ]
        );
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn fn_005d9bc0_replaces_an_existing_array_and_handles_an_empty_list() {
        let (mut e, b) = blend_engine();
        // The actor NPC already has an array (count 4 before it).
        let old_block = e.mem.alloc(0x84);
        e.mem.set_u32(old_block, 4);
        e.mem.set_u32(b.actor_npc + 0x1b4, old_block + 4);
        // Nothing collected: the first slot is empty and the actor does
        // not answer slot 0x1d0.
        e.mem.set_u32(b.slot, 0);
        let still = object_with(&mut e, &[(REFERENCE_SLOT_1D0, V_FALSE)]);
        e.register_double(WORD_AT_20, move |_, args| {
            if args[0] == still {
                0xaa00u32.into_ret()
            } else {
                0x5500u32.into_ret()
            }
        });
        start_log(&mut e);
        e.call(0x005d_9bc0, &args![still, b.source, -25i32]);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCTOR_ITERATOR)[0],
            vec![old_block + 4, 0x20, 4, COORD_BLOCK_DESTRUCT]
        );
        assert_eq!(e.mem.u32(b.actor_npc + 0x1b4), b.raw + 4);
        // The chosen NPC is null: its coordinates are asked for the null.
        assert_eq!(calls(&e, NPC_COORDS)[2], vec![0]);
        assert_eq!(calls(&e, FACEGEN_FN_00653000)[0][0], (-0.25f32).to_bits());
        assert!(calls(&e, CHARACTER_RESET_3D).is_empty());
    }

    #[test]
    fn fn_005d9bc0_needs_both_objects() {
        let (mut e, b) = blend_engine();
        start_log(&mut e);
        e.call(0x005d_9bc0, &args![0u32, b.source, 50i32]);
        e.call(0x005d_9bc0, &args![b.actor, 0u32, 50i32]);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .all(|(a, _)| *a == 0x005d_9bc0));
    }

    #[test]
    fn fn_005d9b20_runs_the_blend_or_sends_it_to_the_task_queue() {
        let (mut e, b) = blend_engine();
        let this_obj = b.actor;
        let a = command(&mut e, this_obj);
        let mode = e.mem.alloc(4);
        e.register_double(FN_0043D4D0, move |_, _| mode.into_ret());
        e.register(TASK_QUEUE_INTERFACE, |_, _| 0x9900_u32.into_ret());
        e.register(TASK_QUEUE_SEND, |_, _| Ret::default());
        parse_gives(&mut e, true, &[b.source, 50]);

        // Single player (the word is not above 1): the blend runs here.
        e.mem.set_u32(mode, 1);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9b20, a));
        assert_parsed(&e, this_obj);
        assert_eq!(calls(&e, FN_0043D4D0), vec![vec![GAME_MODE_OBJECT]]);
        assert_eq!(calls(&e, ACTOR_BASE_GET_SEX).len(), 1);
        assert!(calls(&e, TASK_QUEUE_SEND).is_empty());

        // Above 1: the task queue gets the message and the three words.
        e.mem.set_u32(mode, 2);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9b20, a));
        assert_eq!(
            calls(&e, TASK_QUEUE_SEND),
            vec![vec![0x9900, 0x11df, this_obj, b.source, 50]]
        );
        assert!(calls(&e, ACTOR_BASE_GET_SEX).is_empty());

        // A negative word is not above 1.
        e.mem.set_u32(mode, u32::MAX);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_9b20, a));
        assert!(calls(&e, TASK_QUEUE_SEND).is_empty());

        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_9b20, a));
    }

    #[test]
    fn fn_005d9f70_and_005d9f90_set_and_get_the_coordinate_array_word() {
        let mut e = engine_p7b();
        let npc = object(&mut e);
        e.call(0x005d_9f70, &args![npc, 0xabcdu32]);
        assert_eq!(e.mem.u32(npc + 0x1b4), 0xabcd);
        assert_eq!(e.call(0x005d_9f90, &args![npc]).u32(), 0xabcd);
        e.call(0x005d_9f70, &args![npc, 0u32]);
        assert_eq!(e.call(0x005d_9f90, &args![npc]).u32(), 0);
    }

    #[test]
    fn fn_005d9fb0_picks_one_of_two_sub_objects() {
        let mut e = engine_p7b();
        assert_eq!(e.call(0x005d_9fb0, &args![0x1000u32, 0u32]).u32(), 0x1478);
        assert_eq!(e.call(0x005d_9fb0, &args![0x1000u32, 1u32]).u32(), 0x13f8);
        assert_eq!(e.call(0x005d_9fb0, &args![0x1000u32, 7u32]).u32(), 0x13f8);
    }

    #[test]
    fn fn_005d9ff0_destroys_an_array_or_a_single_object() {
        let mut e = engine_p7b();
        e.register(VECTOR_DESTRUCTOR_ITERATOR, |_, _| Ret::default());
        e.register(COORD_BLOCK_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let block = e.mem.alloc(0x84);
        e.mem.set_u32(block, 4);
        let first = block + 4;

        // Bit 1: an array; bit 0: also free it (the block before the first).
        start_log(&mut e);
        assert_eq!(e.call(0x005d_9ff0, &args![first, 3u32]).u32(), block);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![first, 0x20, 4, COORD_BLOCK_DESTRUCT]]
        );
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![block]]);
        start_log(&mut e);
        assert_eq!(e.call(0x005d_9ff0, &args![first, 2u32]).u32(), block);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());

        // Without bit 1: one object.
        start_log(&mut e);
        assert_eq!(e.call(0x005d_9ff0, &args![first, 1u32]).u32(), first);
        assert_eq!(calls(&e, COORD_BLOCK_DESTRUCT), vec![vec![first]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![first]]);
        assert!(calls(&e, VECTOR_DESTRUCTOR_ITERATOR).is_empty());
        start_log(&mut e);
        assert_eq!(e.call(0x005d_9ff0, &args![first, 0u32]).u32(), first);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn script_get_is_used_item_equip_type_function_asks_the_condition() {
        assert_parsed_condition(0x005d_a060, GET_IS_USED_ITEM_EQUIP_TYPE_CONDITION, u32::MAX);
    }

    #[test]
    fn script_get_threat_ratio_function_asks_the_condition() {
        assert_parsed_condition(0x005d_a0c0, GET_THREAT_RATIO_CONDITION, 0);
    }

    #[test]
    fn script_get_is_alignment_function_asks_the_condition() {
        assert_parsed_condition(0x005d_a120, GET_IS_ALIGNMENT_CONDITION, 1);
    }

    #[test]
    fn fn_005da180_calls_the_function_for_a_parsed_form() {
        let mut e = engine_p7b();
        e.register(FN_0060D720, |_, _| Ret::default());
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0x3333]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a180, a));
        assert_parsed(&e, 0);
        assert_eq!(calls(&e, FN_0060D720), vec![vec![0x3333]]);

        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a180, a));
        assert!(calls(&e, FN_0060D720).is_empty());

        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_a180, a));
    }

    #[test]
    fn fn_005da1e0_applies_a_float_and_a_flag_to_an_object() {
        let mut e = engine_p7b();
        e.register(FN_00444020, |_, _| Ret::default());
        let object = object_with(&mut e, &[(FORM_SLOT_48, V_RECORD)]);
        let a = command(&mut e, 0);
        let amount = 2.5f32.to_bits();
        e.register_double(PARSE_PARAMETERS, move |e, args| {
            // The float local starts at 0.0.
            assert_eq!(e.mem.u32(args[7]), 0);
            assert_eq!(e.mem.u32(args[8]), 0);
            e.mem.set_u32(args[7], object);
            e.mem.set_u32(args[8], amount);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a1e0, a));
        assert_parsed(&e, 0);
        assert_eq!(calls(&e, FN_00444020), vec![vec![object, amount]]);
        assert_eq!(calls(&e, V_RECORD), vec![vec![object, 4]]);

        // No object: nothing is called.
        parse_gives(&mut e, true, &[0, amount]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a1e0, a));
        assert!(calls(&e, FN_00444020).is_empty());

        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_a1e0, a));
    }

    #[test]
    fn fn_005da260_passes_this_obj_on() {
        let mut e = engine_p7b();
        e.register(FN_00477D10, |_, _| Ret::default());
        let a = command(&mut e, 0x4444);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a260, a));
        assert_eq!(calls(&e, FN_00477D10), vec![vec![0x4444]]);
    }

    #[test]
    fn fn_005da280_asks_the_aggro_radius_violated_condition() {
        assert_plain_condition(0x005d_a280, GET_AGGRO_RADIUS_VIOLATED_CONDITION);
    }

    /// The doubles `005da2a0` needs: the process of every actor (a process
    /// object whose slot `0x33c` is a recorder), and the process lists.
    struct Alarm {
        player: u32,
        process: u32,
        found: u32,
    }

    fn alarm_engine() -> (Engine, Alarm) {
        let mut e = engine_p7b();
        let player = set_player(&mut e);
        let process = object_with(&mut e, &[(PROCESS_SLOT_33C, V_RECORD)]);
        let found = object_with(&mut e, &[(REFERENCE_SLOT_21C, V_FALSE)]);
        e.register(ACTOR_ATTACK_ALARM, |_, _| Ret::default());
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register_double(PROCESS_LISTS_FIND, move |_, _| found.into_ret());
        (
            e,
            Alarm {
                player,
                process,
                found,
            },
        )
    }

    /// The expected 13 words of the process call for `target`.
    fn combat_words(process: u32, target: u32, player: u32) -> Vec<u32> {
        vec![process, target, player, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]
    }

    #[test]
    fn fn_005da2a0_raises_the_alarm_for_the_actor_itself() {
        let (mut e, s) = alarm_engine();
        let actor = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_TRUE)]);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[0, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a2a0, a));
        assert_parsed(&e, actor);
        assert_eq!(
            calls(&e, ACTOR_ATTACK_ALARM),
            vec![vec![actor, s.player, 0, 1]]
        );
        assert_eq!(calls(&e, GET_PROCESS), vec![vec![actor]]);
        assert_eq!(
            calls(&e, V_RECORD),
            vec![combat_words(s.process, actor, s.player)]
        );

        // The parse result is ignored: the locals keep their zeros.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a2a0, a));
        assert_eq!(calls(&e, ACTOR_ATTACK_ALARM).len(), 1);

        // Not an actor: nothing at all.
        let thing = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_FALSE)]);
        let not_actor = command(&mut e, thing);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a2a0, not_actor));
        assert!(calls(&e, ACTOR_ATTACK_ALARM).is_empty());
    }

    #[test]
    fn fn_005da2a0_raises_the_alarm_for_another_actor_with_a_process() {
        let (mut e, s) = alarm_engine();
        let actor = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_TRUE)]);
        let a = command(&mut e, actor);
        let other = 0x7777u32;
        parse_gives(&mut e, true, &[other, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a2a0, a));
        assert_eq!(
            calls(&e, ACTOR_ATTACK_ALARM),
            vec![vec![other, s.player, 0, 1]]
        );
        assert_eq!(calls(&e, GET_PROCESS), vec![vec![other], vec![other]]);
        assert_eq!(
            calls(&e, V_RECORD),
            vec![combat_words(s.process, other, s.player)]
        );
        assert!(calls(&e, PROCESS_LISTS_FIND).is_empty());

        // The player, or an actor without a process, falls through to the
        // id (here zero): nothing happens.
        parse_gives(&mut e, true, &[s.player, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a2a0, a));
        assert!(calls(&e, ACTOR_ATTACK_ALARM).is_empty());
        e.register(GET_PROCESS, |_, _| 0u32.into_ret());
        parse_gives(&mut e, true, &[other, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a2a0, a));
        assert!(calls(&e, ACTOR_ATTACK_ALARM).is_empty());
    }

    #[test]
    fn fn_005da2a0_looks_the_id_up_in_the_process_lists() {
        let (mut e, s) = alarm_engine();
        let actor = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_TRUE)]);
        let a = command(&mut e, actor);
        e.register(GET_PROCESS, |_, _| 0u32.into_ret());
        parse_gives(&mut e, true, &[0, 0x4321]);

        // A found reference that fails the test (slot 0x21c) raises it.
        e.register_double(GET_PROCESS, {
            let process = s.process;
            move |_, _| process.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a2a0, a));
        assert_eq!(
            calls(&e, PROCESS_LISTS_FIND),
            vec![vec![PROCESS_LISTS, 0x4321, 1, 1]]
        );
        assert_eq!(
            calls(&e, ACTOR_ATTACK_ALARM),
            vec![vec![s.found, s.player, 0, 1]]
        );
        assert_eq!(
            calls(&e, V_RECORD),
            vec![combat_words(s.process, s.found, s.player)]
        );

        // Nothing found.
        e.register(PROCESS_LISTS_FIND, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a2a0, a));
        assert!(calls(&e, ACTOR_ATTACK_ALARM).is_empty());
    }

    #[test]
    fn fn_005da2a0_hands_a_found_reference_to_its_owners_actor() {
        let (mut e, s) = alarm_engine();
        let actor = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_TRUE)]);
        let a = command(&mut e, actor);
        // The found reference passes the test (slot 0x21c).
        let found = object_with(&mut e, &[(REFERENCE_SLOT_21C, V_TRUE)]);
        let owner = Rc::new(Cell::new(0u32));
        let current = owner.clone();
        e.register_double(PROCESS_LISTS_FIND, move |_, args| {
            if args[1] == 0x4321 {
                found.into_ret()
            } else {
                // The owner of form type 8 is looked up by its id.
                0xac70u32.into_ret()
            }
        });
        e.register_double(REFERENCE_GET_OWNER, move |_, _| current.get().into_ret());
        let owner_type = Rc::new(Cell::new(8u32));
        let kind = owner_type.clone();
        e.register_double(FORM_TYPE, move |_, _| kind.get().into_ret());
        e.register(PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH, |_, _| {
            0xac71_u32.into_ret()
        });
        parse_gives(&mut e, true, &[0, 0x4321]);

        // No owner: nothing.
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a2a0, a));
        assert!(calls(&e, ACTOR_ATTACK_ALARM).is_empty());

        // An owner of form type 8: its actor comes from the id lookup.
        owner.set(0x0123);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a2a0, a));
        assert_eq!(
            calls(&e, PROCESS_LISTS_FIND)[1],
            vec![PROCESS_LISTS, 0x0123, 0, 1]
        );
        assert_eq!(
            calls(&e, ACTOR_ATTACK_ALARM),
            vec![vec![0xac70, s.player, 0, 1]]
        );
        // The process call is made for the found reference.
        assert_eq!(calls(&e, GET_PROCESS), vec![vec![found]]);
        assert_eq!(
            calls(&e, V_RECORD),
            vec![combat_words(s.process, found, s.player)]
        );

        // Any other form type: the actor comes from GetActorRefInHigh.
        owner_type.set(0x2a);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a2a0, a));
        assert_eq!(
            calls(&e, PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH),
            vec![vec![PROCESS_LISTS, 0x0123, 0]]
        );
        assert_eq!(
            calls(&e, ACTOR_ATTACK_ALARM),
            vec![vec![0xac71, s.player, 0, 1]]
        );

        // No actor for the owner: nothing.
        e.register(PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a2a0, a));
        assert!(calls(&e, ACTOR_ATTACK_ALARM).is_empty());
    }

    #[test]
    fn fn_005da540_closes_the_console_and_queues_menu_9() {
        let mut e = engine_p7b();
        let player = set_player(&mut e);
        e.register(CLOSE_CONSOLE, |_, _| Ret::default());
        e.register(QUEUE_MENU_CREATE, |_, _| Ret::default());
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a540, a));
        let log = e.call_log.clone().unwrap();
        let order: Vec<u32> = log.iter().map(|(addr, _)| *addr).collect();
        assert_eq!(order, vec![0x005d_a540, CLOSE_CONSOLE, QUEUE_MENU_CREATE]);
        assert_eq!(
            calls(&e, QUEUE_MENU_CREATE),
            vec![vec![9, player, 0, 0, 1, 0]]
        );
    }

    #[test]
    fn script_fire_weapon_function_fires_weapons_only() {
        let mut e = engine_p7b();
        let script = object_with(&mut e, &[(SCRIPT_NAME_SLOT, V_NAME)]);
        let mut a = command(&mut e, 0x4444);
        a.script_obj = Ptr::new(script);
        let queued = Rc::new(Cell::new(true));
        let use_queue = queued.clone();
        e.register_double(FN_008C7AA0, move |_, _| use_queue.get().into_ret());
        e.register(TASK_QUEUE_INTERFACE, |_, _| 0x9900_u32.into_ret());
        e.register(QUEUE_WEAPON_FIRE, |_, _| Ret::default());
        e.register(WEAPON_FN_00523150, |_, _| Ret::default());
        e.register(FORM_TYPE, |_, _| 0x28u32.into_ret());
        parse_gives(&mut e, true, &[0x5050]);

        // Through the task queue.
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a570, a));
        assert_eq!(
            calls(&e, PARSE_PARAMETERS)[0][..7],
            [1, 2, 8, 0x4444, 0, script, 6]
        );
        assert_eq!(
            calls(&e, QUEUE_WEAPON_FIRE),
            vec![vec![0x9900, 0x5050, 0x4444]]
        );
        assert!(calls(&e, WEAPON_FN_00523150).is_empty());

        // Directly.
        queued.set(false);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a570, a));
        assert_eq!(calls(&e, WEAPON_FN_00523150), vec![vec![0x5050, 0x4444]]);
        assert!(calls(&e, QUEUE_WEAPON_FIRE).is_empty());

        // A form of another type, or none: the message names the script.
        e.register(FORM_TYPE, |_, _| 0x29u32.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a570, a));
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_FIRE_WEAPON_NON_WEAPON, 0xbbbb]]
        );
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a570, a));
        assert_eq!(calls(&e, LOG_STUB).len(), 1);
        assert!(calls(&e, FORM_TYPE).is_empty());

        // No thisObj: nothing; unparsed parameters: failure.
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a570, none));
        assert!(calls(&e, LOG_STUB).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_a570, a));
    }

    #[test]
    fn script_show_tutorial_menu_creates_the_menu_for_the_parsed_value() {
        let mut e = engine_p7b();
        e.register(TUTORIAL_MENU_CREATE, |_, _| Ret::default());
        let allowed = Rc::new(Cell::new(true));
        let gate = allowed.clone();
        e.register_double(ALWAYS_TRUE, move |_, _| gate.get().into_ret());
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a630, a));
        assert_parsed(&e, 0);
        assert_eq!(calls(&e, TUTORIAL_MENU_CREATE), vec![vec![5, 0]]);

        // The parse result is ignored: what the local holds is used.
        parse_gives(&mut e, false, &[7]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a630, a));
        assert_eq!(calls(&e, TUTORIAL_MENU_CREATE), vec![vec![7, 0]]);

        // The gate false: no parse, no menu.
        allowed.set(false);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a630, a));
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        assert!(calls(&e, TUTORIAL_MENU_CREATE).is_empty());
    }

    // ---- The functions from 005da690 ----

    const V_POINT: u32 = 0x0900_0020;
    const V_ITEM: u32 = 0x0900_0021;

    /// `engine_p7b` with the pages of the globals the last commands touch.
    fn engine_p7c() -> Engine {
        let mut e = engine_p7b();
        e.map(0x011d_c000, 0x1000);
        e.map(0x011f_5000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.set_global(THOUSAND, 1000.0f64);
        e
    }

    /// A reference with the given virtual slots whose word at `+0x20` is a
    /// base form holding a chain of three nodes: the `+0x508` links lead
    /// from node 0 to 1 to 2 (then 0), the `+0x504` links back. The base
    /// form's start (`+0x110`, the word `00726070` returns for `+0x10c`) is
    /// node 0. Returns `(reference, base form, nodes)`.
    fn chain_reference(e: &mut Engine, slots: &[(u32, u32)]) -> (u32, u32, [u32; 3]) {
        let reference = object_with(e, slots);
        let base = object(e);
        let nodes = [object(e), object(e), object(e)];
        e.mem.set_u32(reference + 0x20, base);
        e.mem.set_u32(base + 0x110, nodes[0]);
        e.mem.set_u32(nodes[0] + 0x508, nodes[1]);
        e.mem.set_u32(nodes[1] + 0x508, nodes[2]);
        e.mem.set_u32(nodes[2] + 0x504, nodes[1]);
        e.mem.set_u32(nodes[1] + 0x504, nodes[0]);
        e.register(WORD_AT_20, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(LIST_NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NPC_SELECT_LIST_NODE, |_, _| Ret::default());
        e.register(CLEAR_BODY_TEXTURES_FROM_PALETTE, |_, _| Ret::default());
        e.register(CHARACTER_RESET_3D, |_, _| Ret::default());
        (reference, base, nodes)
    }

    #[test]
    fn fn_005da690_walks_the_chain_and_selects_the_node_reached() {
        let mut e = engine_p7c();
        let slots = [
            (REFERENCE_TEST_SLOT_218, V_TRUE),
            (REFERENCE_TEST_SLOT, V_TRUE),
        ];
        let (this_obj, base, nodes) = chain_reference(&mut e, &slots);
        let a = command(&mut e, this_obj);

        // One step down the negative links (count -1 adds one per step).
        parse_gives(&mut e, true, &[(-1i32) as u32]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a690, a));
        assert_parsed(&e, this_obj);
        assert_eq!(
            calls(&e, NPC_SELECT_LIST_NODE),
            vec![vec![base, nodes[1], this_obj]]
        );
        assert_eq!(calls(&e, CHARACTER_RESET_3D), vec![vec![this_obj]]);

        // The walk stops at the end of the chain.
        parse_gives(&mut e, true, &[(-9i32) as u32]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a690, a));
        assert_eq!(
            calls(&e, NPC_SELECT_LIST_NODE),
            vec![vec![base, nodes[2], this_obj]]
        );

        // Positive counts follow the other links.
        e.mem.set_u32(base + 0x110, nodes[2]);
        parse_gives(&mut e, true, &[1]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a690, a));
        assert_eq!(
            calls(&e, NPC_SELECT_LIST_NODE),
            vec![vec![base, nodes[1], this_obj]]
        );

        // No link to follow: the selection does not change.
        e.mem.set_u32(base + 0x110, nodes[0]);
        parse_gives(&mut e, true, &[3]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a690, a));
        assert!(calls(&e, NPC_SELECT_LIST_NODE).is_empty());
        assert!(calls(&e, CHARACTER_RESET_3D).is_empty());

        // A count of zero does nothing at all.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a690, a));
        assert!(calls(&e, WORD_AT_20).is_empty());

        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_a690, a));
    }

    #[test]
    fn fn_005da690_passes_the_reference_only_when_the_test_0x100_holds() {
        let mut e = engine_p7c();
        let slots = [
            (REFERENCE_TEST_SLOT_218, V_TRUE),
            (REFERENCE_TEST_SLOT, V_FALSE),
        ];
        let (this_obj, base, nodes) = chain_reference(&mut e, &slots);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[(-2i32) as u32]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a690, a));
        assert_eq!(
            calls(&e, NPC_SELECT_LIST_NODE),
            vec![vec![base, nodes[2], 0]]
        );
        assert_eq!(calls(&e, CHARACTER_RESET_3D), vec![vec![this_obj]]);
    }

    #[test]
    fn fn_005da690_needs_a_reference_that_passes_the_test_0x218() {
        let mut e = engine_p7c();
        let slots = [
            (REFERENCE_TEST_SLOT_218, V_FALSE),
            (REFERENCE_TEST_SLOT, V_TRUE),
        ];
        let (this_obj, _, _) = chain_reference(&mut e, &slots);
        parse_gives(&mut e, true, &[(-2i32) as u32]);
        let a = command(&mut e, this_obj);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a690, a));
        assert!(calls(&e, WORD_AT_20).is_empty());

        let without = command(&mut e, 0);
        assert!(run(&mut e, 0x005d_a690, without));
        assert!(calls(&e, WORD_AT_20).is_empty());
    }

    #[test]
    fn fn_005da7d0_reads_the_word_at_offset_0x508() {
        let mut e = engine_p7c();
        let node = object(&mut e);
        e.mem.set_u32(node + 0x508, 0x4242);
        assert_eq!(e.call(0x005d_a7d0, &args![node]).u32(), 0x4242);
    }

    #[test]
    fn fn_005da7f0_reads_the_word_at_offset_0x504() {
        let mut e = engine_p7c();
        let node = object(&mut e);
        e.mem.set_u32(node + 0x504, 0x2424);
        assert_eq!(e.call(0x005d_a7f0, &args![node]).u32(), 0x2424);
    }

    #[test]
    fn fn_005da810_copies_the_selection_position_of_the_source() {
        let mut e = engine_p7c();
        let slots = [(REFERENCE_TEST_SLOT_218, V_TRUE)];
        let (this_obj, base, nodes) = chain_reference(&mut e, &slots);
        let (source, source_base, source_nodes) = chain_reference(&mut e, &slots);
        // The source starts at its second node.
        e.mem.set_u32(source_base + 0x110, source_nodes[1]);
        let a = command(&mut e, this_obj);

        // thisObj's start has two links down, the source ends two links
        // above the end of its chain: its first node.
        parse_gives(&mut e, true, &[source]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a810, a));
        assert_parsed(&e, this_obj);
        assert_eq!(
            calls(&e, CLEAR_BODY_TEXTURES_FROM_PALETTE),
            vec![vec![base]]
        );
        assert_eq!(
            calls(&e, NPC_SELECT_LIST_NODE),
            vec![vec![base, source_nodes[0], 0]]
        );
        assert_eq!(calls(&e, CHARACTER_RESET_3D), vec![vec![this_obj]]);

        // A source chain that ends early stops the walk back there.
        e.mem.set_u32(source_nodes[1] + 0x504, 0);
        e.mem.set_u32(source_base + 0x110, source_nodes[2]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a810, a));
        assert_eq!(
            calls(&e, NPC_SELECT_LIST_NODE),
            vec![vec![base, source_nodes[1], 0]]
        );

        // The same start: nothing to do.
        e.mem.set_u32(source_base + 0x110, nodes[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a810, a));
        assert!(calls(&e, NPC_SELECT_LIST_NODE).is_empty());
        assert!(calls(&e, CHARACTER_RESET_3D).is_empty());

        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_a810, a));
    }

    #[test]
    fn fn_005da810_needs_both_references_to_pass_the_test_0x218() {
        let mut e = engine_p7c();
        let (this_obj, _, _) = chain_reference(&mut e, &[(REFERENCE_TEST_SLOT_218, V_TRUE)]);
        let (source, _, _) = chain_reference(&mut e, &[(REFERENCE_TEST_SLOT_218, V_FALSE)]);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[source]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a810, a));
        assert!(calls(&e, WORD_AT_20).is_empty());

        // The source passes, thisObj fails.
        let (bad, _, _) = chain_reference(&mut e, &[(REFERENCE_TEST_SLOT_218, V_FALSE)]);
        let (good, _, _) = chain_reference(&mut e, &[(REFERENCE_TEST_SLOT_218, V_TRUE)]);
        let a = command(&mut e, bad);
        parse_gives(&mut e, true, &[good]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_a810, a));
        assert!(calls(&e, WORD_AT_20).is_empty());

        // No source, or no thisObj.
        parse_gives(&mut e, true, &[0]);
        assert!(run(&mut e, 0x005d_a810, a));
        let none = command(&mut e, 0);
        parse_gives(&mut e, true, &[good]);
        assert!(run(&mut e, 0x005d_a810, none));
        assert!(calls(&e, WORD_AT_20).is_empty());
    }

    /// The commands `005da980` and `005da9e0`: an integer handed to a method
    /// of the player as a flag.
    fn assert_player_flag_command(address: u32, callee: u32) {
        let mut e = engine_p7c();
        let player = set_player(&mut e);
        e.register(callee, |_, _| Ret::default());
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[7]);
        start_log(&mut e);
        assert!(run(&mut e, address, a));
        assert_parsed(&e, 0);
        assert_eq!(calls(&e, callee), vec![vec![player, 1]]);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, address, a));
        assert_eq!(calls(&e, callee), vec![vec![player, 0]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, address, a));
        assert!(calls(&e, callee).is_empty());
    }

    #[test]
    fn fn_005da980_hands_the_flag_to_the_player() {
        assert_player_flag_command(0x005d_a980, PLAYER_FN_00969820);
    }

    #[test]
    fn fn_005da9e0_hands_the_flag_to_the_player() {
        assert_player_flag_command(0x005d_a9e0, PLAYER_FN_009697C0);
    }

    #[test]
    fn script_get_concussed_function_returns_the_condition_for_the_parsed_value() {
        assert_parsed_condition(0x005d_aa40, GET_CONCUSSED_CONDITION, 0);
    }

    #[test]
    fn fn_005daaa0_returns_the_condition_function_s_result() {
        assert_plain_condition(0x005d_aaa0, GET_RADIATION_LEVEL_CONDITION);
    }

    #[test]
    fn fn_005daac0_returns_the_condition_function_s_result() {
        assert_plain_condition(0x005d_aac0, GET_MAP_MARKER_VISIBLE_CONDITION);
    }

    /// Doubles for `005daae0`: whether the form has the flag, the
    /// destructible data (0 for none) and the byte `00576100` gives.
    fn destructible_engine(flag: bool, destruction: u32, byte: u32) -> (Engine, u32) {
        let mut e = engine_p7c();
        let this_obj = object(&mut e);
        e.register_double(FORM_HAS_FLAG_1000000, move |_, _| flag.into_ret());
        e.register(WORD_AT_20, |_, _| 0x6000u32.into_ret());
        e.register_double(GET_DESTRUCTION_FORM, move |_, _| destruction.into_ret());
        e.register_double(DESTRUCTIBLE_FN_00576100, move |_, _| byte.into_ret());
        e.register(FORM_FN_004846E0, |_, _| Ret::default());
        (e, this_obj)
    }

    #[test]
    fn fn_005daae0_sets_the_flag_when_the_destructible_byte_differs() {
        // Equal: 0; different: 1.
        let (mut e, this_obj) = destructible_engine(true, 0x7000, 1);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_aae0, a));
        assert_parsed(&e, this_obj);
        assert_eq!(calls(&e, GET_DESTRUCTION_FORM), vec![vec![0x6000]]);
        assert_eq!(calls(&e, DESTRUCTIBLE_FN_00576100), vec![vec![0x7000]]);
        assert_eq!(calls(&e, FORM_FN_004846E0), vec![vec![this_obj, 0]]);

        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_aae0, a));
        assert_eq!(calls(&e, FORM_FN_004846E0), vec![vec![this_obj, 1]]);

        // The bytes are compared as bytes: 2 is not 1.
        let (mut e, this_obj) = destructible_engine(true, 0x7000, 2);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_aae0, a));
        assert_eq!(calls(&e, FORM_FN_004846E0), vec![vec![this_obj, 1]]);
    }

    #[test]
    fn fn_005daae0_treats_a_form_without_destructible_data_as_zero() {
        let (mut e, this_obj) = destructible_engine(true, 0, 1);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_aae0, a));
        assert!(calls(&e, DESTRUCTIBLE_FN_00576100).is_empty());
        assert_eq!(calls(&e, FORM_FN_004846E0), vec![vec![this_obj, 0]]);
    }

    #[test]
    fn fn_005daae0_does_nothing_without_the_form_flag_and_fails_unparsed() {
        let (mut e, this_obj) = destructible_engine(false, 0x7000, 1);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[1]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_aae0, a));
        assert!(calls(&e, FORM_FN_004846E0).is_empty());
        assert!(calls(&e, GET_DESTRUCTION_FORM).is_empty());
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_aae0, a));
        assert!(calls(&e, FORM_HAS_FLAG_1000000).is_empty());
    }

    #[test]
    fn fn_005dab90_sets_or_clears_bit_0_of_the_byte_at_0x1e() {
        let mut e = engine_p7c();
        let reference = object_with(&mut e, &[(FORM_SLOT_48, V_RECORD)]);
        let a = command(&mut e, 0);
        e.register_double(PARSE_PARAMETERS, move |e, args| {
            assert_eq!(e.mem.u32(args[7]), 0);
            assert_eq!(e.mem.u32(args[8]), 0);
            e.mem.set_u32(args[7], reference);
            e.mem.set_u32(args[8], 0);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ab90, a));
        assert_parsed(&e, 0);
        // An integer of 0 sets the bit.
        assert_eq!(e.mem.u8(reference + 0x1e) & 1, 1);
        assert_eq!(calls(&e, V_RECORD), vec![vec![reference, 2]]);

        parse_gives(&mut e, true, &[reference, 3]);
        assert!(run(&mut e, 0x005d_ab90, a));
        assert_eq!(e.mem.u8(reference + 0x1e) & 1, 0);

        // No reference: nothing happens; parameters that do not parse fail.
        start_log(&mut e);
        parse_gives(&mut e, true, &[0, 0]);
        assert!(run(&mut e, 0x005d_ab90, a));
        assert!(calls(&e, V_RECORD).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_ab90, a));
    }

    #[test]
    fn fn_005dac10_sets_or_clears_bit_0_and_keeps_the_others() {
        let mut e = engine_p7c();
        let object = object(&mut e);
        e.mem.set_u8(object + 0x1e, 0xa4);
        e.call(0x005d_ac10, &args![object, 1u32]);
        assert_eq!(e.mem.u8(object + 0x1e), 0xa5);
        e.call(0x005d_ac10, &args![object, 0u32]);
        assert_eq!(e.mem.u8(object + 0x1e), 0xa4);
        e.call(0x005d_ac10, &args![object, 0u32]);
        assert_eq!(e.mem.u8(object + 0x1e), 0xa4);
    }

    #[test]
    fn fn_005dac60_calls_slot_0x208_of_this_obj_with_zero() {
        let mut e = engine_p7c();
        let this_obj = object_with(&mut e, &[(REFERENCE_SLOT_208, V_RECORD)]);
        let a = command(&mut e, this_obj);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ac60, a));
        assert_eq!(calls(&e, V_RECORD), vec![vec![this_obj, 0]]);
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ac60, none));
        assert!(calls(&e, V_RECORD).is_empty());
    }

    #[test]
    fn fn_005dac80_tells_the_process_of_the_reference_a_point_of_this_obj() {
        let mut e = engine_p7c();
        let point = e.mem.alloc(12);
        for (i, word) in [0x11u32, 0x22, 0x33].iter().enumerate() {
            e.mem.set_u32(point + 4 * i as u32, *word);
        }
        e.register_double(V_POINT, move |_, _| point.into_ret());
        let this_obj = object_with(&mut e, &[(REFERENCE_SLOT_1F4, V_POINT)]);
        let process = object_with(&mut e, &[(PROCESS_SLOT_FC, V_RECORD)]);
        let target = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_TRUE)]);
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        let a = command(&mut e, this_obj);
        e.register_double(PARSE_PARAMETERS, move |e, args| {
            // The locals start as 0, 0 and 3.
            assert_eq!(
                [e.mem.u32(args[7]), e.mem.u32(args[8]), e.mem.u32(args[9])],
                [0, 0, 3]
            );
            e.mem.set_u32(args[7], target);
            e.mem.set_u32(args[8], 7);
            e.mem.set_u32(args[9], 9);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ac80, a));
        assert_parsed(&e, this_obj);
        assert_eq!(calls(&e, V_POINT), vec![vec![this_obj]]);
        assert_eq!(
            calls(&e, V_RECORD),
            vec![vec![process, target, 0x11, 0x22, 0x33, 7, 9, this_obj]]
        );
        assert_eq!(calls(&e, GET_PROCESS).len(), 2);
    }

    #[test]
    fn fn_005dac80_skips_what_cannot_be_done() {
        let mut e = engine_p7c();
        e.register(V_POINT, |_, _| 0u32.into_ret());
        let this_obj = object_with(&mut e, &[(REFERENCE_SLOT_1F4, V_POINT)]);
        let process = object_with(&mut e, &[(PROCESS_SLOT_FC, V_RECORD)]);
        let target = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_TRUE)]);
        let refusing = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_FALSE)]);
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        let a = command(&mut e, this_obj);

        // The reference fails the virtual test.
        parse_gives(&mut e, true, &[refusing, 0, 3]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ac80, a));
        assert!(calls(&e, V_POINT).is_empty());

        // No reference.
        parse_gives(&mut e, true, &[0, 0, 3]);
        assert!(run(&mut e, 0x005d_ac80, a));
        assert!(calls(&e, V_POINT).is_empty());

        // No process.
        e.register(GET_PROCESS, |_, _| 0u32.into_ret());
        parse_gives(&mut e, true, &[target, 0, 3]);
        assert!(run(&mut e, 0x005d_ac80, a));
        assert!(calls(&e, V_POINT).is_empty());

        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_ac80, a));

        // No thisObj: the parameters are not even parsed.
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ac80, none));
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
    }

    /// An engine for `005dad70`: the player has room for its byte at
    /// `+0xdf2`, the world space and its terrain manager are the given
    /// words.
    fn terrain_engine(world: u32, terrain: u32) -> (Engine, u32) {
        let mut e = engine_p7c();
        let player = e.mem.alloc(0x1000);
        e.set_global(PLAYER, player);
        e.set_global(TES_SINGLETON, 0x7001u32);
        e.register_double(TES_GET_WORLD_SPACE, move |_, _| world.into_ret());
        e.register_double(WORLD_SPACE_GET_TERRAIN_MANAGER, move |_, _| {
            terrain.into_ret()
        });
        e.register(TERRAIN_MANAGER_FN_006FD060, |_, _| Ret::default());
        (e, player)
    }

    #[test]
    fn fn_005dad70_flags_the_player_and_refreshes_the_terrain() {
        let (mut e, player) = terrain_engine(0x7100, 0x7200);
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ad70, a));
        assert_eq!(e.mem.u8(player + 0xdf2), 1);
        assert_eq!(calls(&e, TES_GET_WORLD_SPACE), vec![vec![0x7001]; 3]);
        assert_eq!(
            calls(&e, WORLD_SPACE_GET_TERRAIN_MANAGER),
            vec![vec![0x7100]; 2]
        );
        assert_eq!(calls(&e, TERRAIN_MANAGER_FN_006FD060), vec![vec![0x7200]]);
    }

    #[test]
    fn fn_005dad70_only_flags_the_player_without_a_world_or_terrain_manager() {
        let (mut e, player) = terrain_engine(0, 0x7200);
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ad70, a));
        assert_eq!(e.mem.u8(player + 0xdf2), 1);
        assert!(calls(&e, WORLD_SPACE_GET_TERRAIN_MANAGER).is_empty());

        let (mut e, player) = terrain_engine(0x7100, 0);
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ad70, a));
        assert_eq!(e.mem.u8(player + 0xdf2), 1);
        assert_eq!(calls(&e, WORLD_SPACE_GET_TERRAIN_MANAGER).len(), 1);
        assert!(calls(&e, TERRAIN_MANAGER_FN_006FD060).is_empty());
    }

    #[test]
    fn fn_005dadd0_stores_the_byte_at_offset_0xdf2() {
        let mut e = engine_p7c();
        let player = e.mem.alloc(0x1000);
        e.call(0x005d_add0, &args![player, 1u32]);
        assert_eq!(e.mem.u8(player + 0xdf2), 1);
        e.call(0x005d_add0, &args![player, 0u32]);
        assert_eq!(e.mem.u8(player + 0xdf2), 0);
    }

    #[test]
    fn fn_005dadf0_returns_the_condition_function_s_result() {
        assert_plain_condition(0x005d_adf0, GET_WEAPON_HEALTH_PERC_CONDITION);
    }

    const ITEM_FORM: u32 = 0x4000;
    const CHANGES: u32 = 0xcc00;
    const FIRST_EXTRA_LIST: u32 = 0xe1e1;

    /// An engine for the two health commands: `thisObj` passes the test
    /// `0x100`, its process hands out an item change whose health is
    /// `current_without_mods` (flag 0) and `current_with_mods` (flag 1).
    /// The form's maximum health is 200. Returns the engine, `thisObj` and
    /// the item.
    fn health_engine(current_flag_0: f32, current_flag_1: f32) -> (Engine, u32, u32) {
        let mut e = engine_p7c();
        let this_obj = object_with(
            &mut e,
            &[
                (REFERENCE_TEST_SLOT, V_TRUE),
                (REFERENCE_LOWER_ITEM_HEALTH_SLOT, V_RECORD),
            ],
        );
        let process = object_with(&mut e, &[(PROCESS_EQUIPPED_ITEM_SLOT, V_ITEM)]);
        let item = object(&mut e);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, FIRST_EXTRA_LIST);
        e.mem.set_u32(item, node);
        e.register_double(V_ITEM, move |_, _| item.into_ret());
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(ITEM_HAS_MOD_EFFECT_ACTIVE, |_, _| 1u32.into_ret());
        e.register(ITEM_GET_FORM, |_, _| ITEM_FORM.into_ret());
        e.register(WEAPON_GET_FORM_HEALTH, |_, _| 200u32.into_ret());
        e.register(HEALTH_FORM_GET_FORM_HEALTH, |_, _| 200u32.into_ret());
        e.register_double(ITEM_GET_ITEM_HEALTH, move |_, a| {
            if a[1] == 0 {
                current_flag_0
            } else {
                current_flag_1
            }
            .into_ret()
        });
        e.register(DEREF, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_NODE_DATA, |_, a| a[0].into_ret());
        e.register(EXTRA_DATA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(GET_CONTAINER_CHANGES, |_, _| CHANGES.into_ret());
        e.register(FLOAT_MIN, |_, a| {
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            if second <= first { second } else { first }.into_ret()
        });
        e.register(ITEM_SET_ITEM_HEALTH, |_, _| Ret::default());
        (e, this_obj, item)
    }

    /// `ParseParameters` double for the health commands: the local starts as
    /// 1.0 and becomes `percent`.
    fn parse_percent(e: &mut Engine, percent: f32) {
        e.register_double(PARSE_PARAMETERS, move |e, args| {
            assert_eq!(e.mem.u32(args[7]), 1.0f32.to_bits());
            e.mem.set_u32(args[7], percent.to_bits());
            true.into_ret()
        });
    }

    #[test]
    fn fn_005dae10_raises_the_item_health_to_a_percentage_of_the_maximum() {
        let (mut e, this_obj, item) = health_engine(40.0, 0.0);
        let a = command(&mut e, this_obj);
        parse_percent(&mut e, 50.0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ae10, a));
        assert_parsed(&e, this_obj);
        assert_eq!(calls(&e, ITEM_HAS_MOD_EFFECT_ACTIVE), vec![vec![item, 10]]);
        assert_eq!(calls(&e, WEAPON_GET_FORM_HEALTH), vec![vec![ITEM_FORM, 1]]);
        assert_eq!(calls(&e, ITEM_GET_ITEM_HEALTH), vec![vec![item, 0]]);
        assert_eq!(calls(&e, EXTRA_DATA_LIST), vec![vec![this_obj]]);
        assert_eq!(
            calls(&e, ITEM_SET_ITEM_HEALTH),
            vec![vec![item, 100.0f32.to_bits(), CHANGES, FIRST_EXTRA_LIST, 1]]
        );
        assert!(calls(&e, V_RECORD).is_empty());
    }

    #[test]
    fn fn_005dae10_limits_a_raise_to_the_maximum_minus_one() {
        let (mut e, this_obj, item) = health_engine(40.0, 0.0);
        let a = command(&mut e, this_obj);
        parse_percent(&mut e, 150.0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ae10, a));
        assert_eq!(
            calls(&e, FLOAT_MIN),
            vec![vec![300.0f32.to_bits(), 199.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, ITEM_SET_ITEM_HEALTH),
            vec![vec![item, 199.0f32.to_bits(), CHANGES, FIRST_EXTRA_LIST, 1]]
        );
    }

    #[test]
    fn fn_005dae10_lowers_the_item_health_through_the_reference() {
        let (mut e, this_obj, item) = health_engine(150.0, 0.0);
        let a = command(&mut e, this_obj);
        parse_percent(&mut e, 50.0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ae10, a));
        assert_eq!(
            calls(&e, V_RECORD),
            vec![vec![this_obj, item, 50.0f32.to_bits(), 1]]
        );
        assert!(calls(&e, ITEM_SET_ITEM_HEALTH).is_empty());

        // The health already at the target: nothing to raise, a lowering by
        // the negated zero (`FCHS` of +0.0 gives -0.0).
        let (mut e, this_obj, item) = health_engine(100.0, 0.0);
        let a = command(&mut e, this_obj);
        parse_percent(&mut e, 50.0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_ae10, a));
        assert_eq!(
            calls(&e, V_RECORD),
            vec![vec![this_obj, item, (-0.0f32).to_bits(), 1]]
        );
    }

    /// The health commands need a reference passing the test `0x100`, a
    /// process and an equipped item; they fail when the parameters do not
    /// parse.
    fn assert_health_command_guards(address: u32) {
        let (mut e, this_obj, _) = health_engine(40.0, 0.0);
        let a = command(&mut e, this_obj);

        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, address, a));
        assert!(calls(&e, GET_PROCESS).is_empty());

        // No thisObj.
        parse_percent(&mut e, 50.0);
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, address, none));
        assert!(calls(&e, GET_PROCESS).is_empty());

        // The test 0x100 fails.
        let refusing = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_FALSE)]);
        let a_refusing = command(&mut e, refusing);
        start_log(&mut e);
        assert!(run(&mut e, address, a_refusing));
        assert!(calls(&e, GET_PROCESS).is_empty());

        // No process.
        e.register(GET_PROCESS, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, address, a));
        assert_eq!(calls(&e, GET_PROCESS).len(), 1);
        assert!(calls(&e, ITEM_GET_ITEM_HEALTH).is_empty());

        // A process without an equipped item.
        let process = object_with(&mut e, &[(PROCESS_EQUIPPED_ITEM_SLOT, V_ITEM)]);
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(V_ITEM, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, address, a));
        assert_eq!(calls(&e, GET_PROCESS).len(), 2);
        assert!(calls(&e, ITEM_GET_ITEM_HEALTH).is_empty());
    }

    #[test]
    fn fn_005dae10_needs_a_reference_a_process_and_an_equipped_item() {
        assert_health_command_guards(0x005d_ae10);
    }

    #[test]
    fn fn_005daf90_raises_the_item_health_by_a_percentage() {
        // Current health with mods 50, plus 25, is 75 percent of 200.
        let (mut e, this_obj, item) = health_engine(100.0, 50.0);
        let a = command(&mut e, this_obj);
        parse_percent(&mut e, 25.0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_af90, a));
        assert_parsed(&e, this_obj);
        assert_eq!(
            calls(&e, ITEM_GET_ITEM_HEALTH),
            vec![vec![item, 1], vec![item, 0]]
        );
        assert_eq!(
            calls(&e, HEALTH_FORM_GET_FORM_HEALTH),
            vec![vec![ITEM_FORM]]
        );
        assert!(calls(&e, FLOAT_MIN).is_empty());
        assert_eq!(
            calls(&e, ITEM_SET_ITEM_HEALTH),
            vec![vec![item, 150.0f32.to_bits(), CHANGES, FIRST_EXTRA_LIST, 1]]
        );
    }

    #[test]
    fn fn_005daf90_lowers_the_item_health_and_never_goes_below_zero() {
        // 10 - 50 is negative: the fraction is 0, the target 0, and the
        // current health 100 is taken off through the reference.
        let (mut e, this_obj, item) = health_engine(100.0, 10.0);
        let a = command(&mut e, this_obj);
        parse_percent(&mut e, -50.0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_af90, a));
        assert_eq!(
            calls(&e, V_RECORD),
            vec![vec![this_obj, item, 100.0f32.to_bits(), 1]]
        );
        assert!(calls(&e, ITEM_SET_ITEM_HEALTH).is_empty());
    }

    #[test]
    fn fn_005daf90_needs_a_reference_a_process_and_an_equipped_item() {
        assert_health_command_guards(0x005d_af90);
    }

    #[test]
    fn script_set_rumble_function_rumbles_the_controls_with_a_duration_in_milliseconds() {
        let mut e = engine_p7c();
        e.set_global(MAIN_SINGLETON, 0x7002u32);
        e.register(MAIN_GET_CONTROLS, |_, a| {
            assert_eq!(a[0], 0x7002);
            0x7003u32.into_ret()
        });
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            (value as i32 as u32).into_ret()
        });
        e.register(CONTROLS_RUMBLE, |_, _| Ret::default());
        let a = command(&mut e, 0);
        e.register_double(PARSE_PARAMETERS, |e, args| {
            // The three floats start as 0.0.
            for i in 0..3 {
                assert_eq!(e.mem.u32(args[7 + i]), 0);
            }
            e.mem.set_u32(args[7], 0.25f32.to_bits());
            e.mem.set_u32(args[8], 0.5f32.to_bits());
            e.mem.set_u32(args[9], 1.5f32.to_bits());
            true.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_b110, a));
        assert_parsed(&e, 0);
        assert_eq!(
            calls(&e, CONTROLS_RUMBLE),
            vec![vec![
                0x7003,
                0.25f32.to_bits(),
                0.5f32.to_bits(),
                1500,
                0,
                0,
                0,
                1
            ]]
        );

        // No controls object: no rumble.
        e.register(MAIN_GET_CONTROLS, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_b110, a));
        assert!(calls(&e, CONTROLS_RUMBLE).is_empty());

        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!run(&mut e, 0x005d_b110, a));
        assert!(calls(&e, MAIN_GET_CONTROLS).is_empty());
    }

    #[test]
    fn fn_005db1b0_precaches_the_sound_of_the_form_three_times_and_remembers_it() {
        let mut e = engine_p7c();
        e.register(FORM_SOUND_PATH, |_, _| 0x5a5au32.into_ret());
        e.register(AUDIO_INSTANCE, |_, _| 0x7004u32.into_ret());
        e.register(AUDIO_PRECACHE, |_, _| Ret::default());
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0x9090]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_b1b0, a));
        assert_parsed(&e, 0);
        assert_eq!(calls(&e, FORM_SOUND_PATH), vec![vec![0x9090]; 3]);
        assert_eq!(
            calls(&e, AUDIO_PRECACHE),
            vec![vec![0x7004, 0x5a5a, 0x2000_0121, 0x9090]; 3]
        );
        assert_eq!(e.mem.u32(WORD_AT_11DCFA8), 0x9090);

        // A null form, or parameters that do not parse: still a success.
        e.mem.set_u32(WORD_AT_11DCFA8, 0);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_b1b0, a));
        parse_gives(&mut e, false, &[]);
        assert!(run(&mut e, 0x005d_b1b0, a));
        assert!(calls(&e, AUDIO_PRECACHE).is_empty());
        assert_eq!(e.mem.u32(WORD_AT_11DCFA8), 0);
    }

    #[test]
    fn fn_005db260_stores_the_word() {
        let mut e = engine_p7c();
        e.call(0x005d_b260, &args![0x1234u32]);
        assert_eq!(e.mem.u32(WORD_AT_11DCFA8), 0x1234);
    }

    #[test]
    fn fn_005db270_clears_the_word_and_succeeds() {
        let mut e = engine_p7c();
        e.mem.set_u32(WORD_AT_11DCFA8, 0x1234);
        let a = command(&mut e, 0);
        assert!(run(&mut e, 0x005d_b270, a));
        assert_eq!(e.mem.u32(WORD_AT_11DCFA8), 0);
    }

    #[test]
    fn fn_005db280_clears_the_word() {
        let mut e = engine_p7c();
        e.mem.set_u32(WORD_AT_11DCFA8, 0x1234);
        e.call(0x005d_b280, &args![]);
        assert_eq!(e.mem.u32(WORD_AT_11DCFA8), 0);
    }

    /// An engine for `005db290`: the cast gives `actor` (an object with the
    /// word `ragdoll` at `+0xac`).
    fn skeleton_engine(cast_to_actor: bool, ragdoll: u32) -> (Engine, u32) {
        let mut e = engine_p7c();
        e.set_global(FADE_SKINS_STRENGTH, 0.5f32);
        let actor = object(&mut e);
        e.mem.set_u32(actor + 0xac, ragdoll);
        e.register_double(DYNAMIC_CAST, move |_, _| {
            if cast_to_actor { actor } else { 0 }.into_ret()
        });
        e.register(ACTOR_FN_0043FCD0, |_, _| 0x7777u32.into_ret());
        e.register(ACTOR_FADE_SKINS, |_, _| Ret::default());
        e.register(RAGDOLL_FN_00C79D60, |_, _| Ret::default());
        (e, actor)
    }

    #[test]
    fn script_draw_skeleton_fades_the_skins_and_updates_the_ragdoll() {
        let (mut e, actor) = skeleton_engine(true, 0x8800);
        let a = command(&mut e, 0x4444);
        e.register_double(PARSE_PARAMETERS, |e, args| {
            for i in 0..3 {
                assert_eq!(e.mem.u32(args[7 + i]), 0);
            }
            e.mem.set_u32(args[8], 5);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_b290, a));
        assert_parsed(&e, 0x4444);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x4444, 0, 0x0118_41cc, 0x0118_46d4, 0]]
        );
        // One flag set: the strength is the exe's 0.5.
        assert_eq!(
            calls(&e, ACTOR_FADE_SKINS),
            vec![vec![actor, 0x7777, 0.5f32.to_bits()]]
        );
        assert_eq!(calls(&e, RAGDOLL_FN_00C79D60), vec![vec![0x8800, 0, 1, 0]]);

        // No flag set: full strength.
        parse_gives(&mut e, true, &[0, 0, 0]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_b290, a));
        assert_eq!(
            calls(&e, ACTOR_FADE_SKINS),
            vec![vec![actor, 0x7777, 1.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, RAGDOLL_FN_00C79D60), vec![vec![0x8800, 0, 0, 0]]);

        // Every flag, with other non-zero values.
        parse_gives(&mut e, true, &[9, 8, 7]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_b290, a));
        assert_eq!(calls(&e, RAGDOLL_FN_00C79D60), vec![vec![0x8800, 1, 1, 1]]);
    }

    #[test]
    fn script_draw_skeleton_skips_what_is_not_an_actor_with_a_ragdoll() {
        // The cast fails.
        let (mut e, _) = skeleton_engine(false, 0x8800);
        let a = command(&mut e, 0x4444);
        parse_gives(&mut e, true, &[1, 1, 1]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_b290, a));
        assert!(calls(&e, ACTOR_FADE_SKINS).is_empty());

        // The actor has no word at +0xac.
        let (mut e, _) = skeleton_engine(true, 0);
        let a = command(&mut e, 0x4444);
        parse_gives(&mut e, true, &[1, 1, 1]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_b290, a));
        assert!(calls(&e, ACTOR_FADE_SKINS).is_empty());
        assert!(calls(&e, RAGDOLL_FN_00C79D60).is_empty());

        // No thisObj: no cast; parameters that do not parse: failure.
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_b290, none));
        assert!(calls(&e, DYNAMIC_CAST).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_b290, a));
    }

    #[test]
    fn fn_005db3b0_returns_the_condition_function_s_result() {
        assert_plain_condition(0x005d_b3b0, IS_GREETING_PLAYER_CONDITION);
    }

    #[test]
    fn fn_005db3d0_returns_the_condition_function_s_result() {
        assert_plain_condition(0x005d_b3d0, GET_IGNORE_CRIME_CONDITION);
    }

    #[test]
    fn fn_005db3f0_calls_slot_0x31c_with_whether_the_integer_is_positive() {
        let mut e = engine_p7c();
        let this_obj = object_with(
            &mut e,
            &[
                (REFERENCE_TEST_SLOT, V_TRUE),
                (REFERENCE_SLOT_31C, V_RECORD),
            ],
        );
        let a = command(&mut e, this_obj);
        for (value, expected) in [(5i32, 1u32), (1, 1), (0, 0), (-3, 0)] {
            parse_gives(&mut e, true, &[value as u32]);
            start_log(&mut e);
            assert!(run(&mut e, 0x005d_b3f0, a));
            assert_parsed(&e, this_obj);
            assert_eq!(calls(&e, V_RECORD), vec![vec![this_obj, expected]]);
        }

        // The test 0x100 fails, or there is no thisObj.
        let refusing = object_with(&mut e, &[(REFERENCE_TEST_SLOT, V_FALSE)]);
        let a_refusing = command(&mut e, refusing);
        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_b3f0, a_refusing));
        let none = command(&mut e, 0);
        assert!(run(&mut e, 0x005d_b3f0, none));
        assert!(calls(&e, V_RECORD).is_empty());

        parse_gives(&mut e, false, &[]);
        assert!(!run(&mut e, 0x005d_b3f0, a));
    }

    #[test]
    fn fn_005db490_flags_the_pipboy_when_there_is_one() {
        let mut e = engine_p7c();
        let pipboy = e.mem.alloc(0x1000);
        e.register_double(INTERFACE_GET_PIPBOY, move |_, _| pipboy.into_ret());
        let a = command(&mut e, 0);
        assert!(run(&mut e, 0x005d_b490, a));
        assert_eq!(e.mem.u8(pipboy + 0x16c), 1);

        e.mem.set_u8(pipboy + 0x16c, 0);
        e.register(INTERFACE_GET_PIPBOY, |_, _| 0u32.into_ret());
        assert!(run(&mut e, 0x005d_b490, a));
        assert_eq!(e.mem.u8(pipboy + 0x16c), 0);
    }

    #[test]
    fn fn_005db4c0_stores_the_byte_at_offset_0x16c() {
        let mut e = engine_p7c();
        let pipboy = e.mem.alloc(0x1000);
        e.call(0x005d_b4c0, &args![pipboy, 1u32]);
        assert_eq!(e.mem.u8(pipboy + 0x16c), 1);
        e.call(0x005d_b4c0, &args![pipboy, 0u32]);
        assert_eq!(e.mem.u8(pipboy + 0x16c), 0);
    }

    #[test]
    fn script_is_combat_target_function_returns_the_condition_for_the_parsed_value() {
        assert_parsed_condition(0x005d_b4e0, IS_COMBAT_TARGET_CONDITION, 0);
    }

    #[test]
    fn script_get_vats_right_area_free_function_returns_the_condition_for_the_parsed_value() {
        assert_parsed_condition(0x005d_b540, GET_VATS_RIGHT_AREA_FREE_CONDITION, 0);
    }

    #[test]
    fn script_get_vats_left_area_free_function_returns_the_condition_for_the_parsed_value() {
        assert_parsed_condition(0x005d_b5a0, GET_VATS_LEFT_AREA_FREE_CONDITION, 0);
    }

    #[test]
    fn script_get_vats_back_area_free_function_returns_the_condition_for_the_parsed_value() {
        assert_parsed_condition(0x005d_b600, GET_VATS_BACK_AREA_FREE_CONDITION, 0);
    }

    #[test]
    fn script_get_vats_front_area_free_function_returns_the_condition_for_the_parsed_value() {
        assert_parsed_condition(0x005d_b660, GET_VATS_FRONT_AREA_FREE_CONDITION, 0);
    }

    #[test]
    fn fn_005db6c0_sets_the_byte_and_closes_the_console() {
        let mut e = engine_p7c();
        e.register(CLOSE_CONSOLE, |_, _| Ret::default());
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(run(&mut e, 0x005d_b6c0, a));
        assert_eq!(e.mem.u8(BYTE_AT_11F5AF1), 1);
        assert_eq!(calls(&e, CLOSE_CONSOLE), vec![Vec::<u32>::new()]);
    }

    #[test]
    fn fn_005db6e0_stores_the_byte() {
        let mut e = engine_p7c();
        e.call(0x005d_b6e0, &args![7u32]);
        assert_eq!(e.mem.u8(BYTE_AT_11F5AF1), 7);
    }

    #[test]
    fn script_get_vats_right_target_visible_function_returns_the_condition_for_the_parsed_value() {
        assert_parsed_condition(0x005d_b6f0, GET_VATS_RIGHT_TARGET_VISIBLE_CONDITION, 0);
    }

    #[test]
    fn script_get_vats_left_target_visible_function_returns_the_condition_for_the_parsed_value() {
        assert_parsed_condition(0x005d_b750, GET_VATS_LEFT_TARGET_VISIBLE_CONDITION, 0);
    }

    #[test]
    fn script_get_vats_back_target_visible_function_returns_the_condition_for_the_parsed_value() {
        assert_parsed_condition(0x005d_b7b0, GET_VATS_BACK_TARGET_VISIBLE_CONDITION, 0);
    }
}

//! `fallout shared/tesscriptfunctions.cpp` (Xbox PDB source unit), part 5: its functions from `005cd990` up to
//! (not including) `005d21e0` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesscriptfunctions`]; anything public there may be used here.
//!
//! Progress: all 120 queue entries of the range (`005cd990` to `005d21e0`)
//! are translated; the range is complete.
//!
//! The bodies follow the conventions of the main file: `cdecl`, the eight
//! stack words as [`ScriptArgs`], `AL` as the result. The members of
//! `PlayerCharacter`, `Actor` and the factions are read at the PC offsets
//! (the PC build differs from the Xbox PDB's layout), with a comment, instead
//! of through a `layout!`.

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
/// RTTI type descriptor of `TESObjectREFR` (`.?AVTESObjectREFR@@`).
const RTTI_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// RTTI type descriptor of `Actor` (`.?AVActor@@`).
const RTTI_ACTOR: u32 = 0x0118_46d4;

/// `Script::GetDetectionLevelConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, actor, 0, result`); the other condition functions below take
/// the same four words.
const GET_DETECTION_LEVEL_CONDITION: u32 = 0x005a_1ee0;
/// `Script::IsSwimmingConditionFunction` (Xbox PDB).
const IS_SWIMMING_CONDITION: u32 = 0x005a_1fa0;
/// `Script::GetAmountStolenSoldConditionFunction` (Xbox PDB).
const GET_AMOUNT_STOLEN_SOLD_CONDITION: u32 = 0x005a_2050;
/// `Script::GetPCExpelledConditionFunction` (Xbox PDB).
const GET_PC_EXPELLED_CONDITION: u32 = 0x005a_20d0;
/// `Script::GetPCFactionMurderConditionFunction` (Xbox PDB).
const GET_PC_FACTION_MURDER_CONDITION: u32 = 0x005a_2150;
/// `Script::GetPlayerEnemyofFactionConditionFunction` (Xbox PDB).
const GET_PLAYER_ENEMY_OF_FACTION_CONDITION: u32 = 0x005a_21f0;
/// `Script::GetPCFactionAttackConditionFunction` (Xbox PDB).
const GET_PC_FACTION_ATTACK_CONDITION: u32 = 0x005a_2290;
/// `Script::GetDestroyedConditionFunction` (Xbox PDB).
const GET_DESTROYED_CONDITION: u32 = 0x005a_2330;
/// `Script::HasMagicEffectConditionFunction` (Xbox PDB).
const HAS_MAGIC_EFFECT_CONDITION: u32 = 0x005a_2390;
/// `Script::IsSpellTargetConditionFunction` (Xbox PDB).
const IS_SPELL_TARGET_CONDITION: u32 = 0x005a_2440;
/// `Script::GetSpellUsageNumberConditionFunction` (Xbox PDB).
const GET_SPELL_USAGE_NUMBER_CONDITION: u32 = 0x005a_24f0;
/// `Script::GetVATSModeConditionFunction` (Xbox PDB).
const GET_VATS_MODE_CONDITION: u32 = 0x005a_2590;
/// `Script::GetVATSTargetHeightConditionFunction` (Xbox PDB).
const GET_VATS_TARGET_HEIGHT_CONDITION: u32 = 0x005a_25f0;

/// `Script::PutNumericIDInDouble` (Xbox PDB), `cdecl` (`address of the id,
/// double* result`): stores the 4-byte id into the double.
const PUT_NUMERIC_ID_IN_DOUBLE: u32 = 0x005a_cc70;
/// `thiscall`: `*(this + 0x0c)`, the form id of a form.
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// `thiscall` on a reference: the name of the reference (the full name of
/// its base form).
const GET_REFERENCE_NAME: u32 = 0x0055_d520;
/// `TESObjectREFR::GetActionRef` (Xbox PDB).
const GET_ACTION_REF: u32 = 0x0057_2e30;
/// `TESObjectREFR::GetRefPersists` (Xbox PDB).
const GET_REF_PERSISTS: u32 = 0x0056_53d0;
/// `*(this + 0x20)`: the base form of a reference (the map calls it
/// `BGSSaveFormBuffer::GetForm`).
const GET_BASE_FORM: u32 = 0x007a_f430;
/// Form type byte (`this + 4`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `TESContainer::ContainerCanHoldType` (Xbox PDB), `cdecl` (`form type`).
const CONTAINER_CAN_HOLD_TYPE: u32 = 0x0048_1f30;
/// `thiscall` on a reference: a reference read from its extra data list
/// through `0041da10` (what `GetParentRef` returns).
const GET_PARENT_REF: u32 = 0x0056_a9f0;
/// `thiscall` on a reference: a reference read from its extra data list
/// through `0041e410` (what `GetLinkedRef` returns).
const GET_LINKED_REF: u32 = 0x0056_9b80;
/// `this + 0x44`: the extra data list of a reference.
const EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::GetPackageExtra` (Xbox PDB): the package of the list.
const GET_PACKAGE_EXTRA: u32 = 0x0041_cb10;
/// `Actor::GetPackageSetAsPcurrent` (Xbox PDB).
const GET_PACKAGE_SET_AS_CURRENT: u32 = 0x0088_1510;
/// `Actor::GetCurrentPackageTarget` (Xbox PDB).
const GET_CURRENT_PACKAGE_TARGET: u32 = 0x0088_1650;
/// Package type: the sign-extended byte at `package + 0x20`.
const GET_PACKAGE_TYPE: u32 = 0x0041_ca90;
/// `thiscall` on a package (`flag`): sets or clears bit `0x10000` of its
/// flags word (`this + 0x1c`) and notifies the data handler.
const SET_PACKAGE_FLAG: u32 = 0x0067_4f30;
/// `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB): `*(this + 0x68)`,
/// the actor's process.
const GET_PROCESS: u32 = 0x008d_8520;
/// `PlayerCharacter` method (`00962720`, `thiscall`, `actor`): whether the
/// actor is among the player's teammates.
const PLAYER_HAS_TEAMMATE: u32 = 0x0096_2720;
/// `PlayerCharacter` method (`00962620`, `thiscall`): the number of
/// teammates.
const PLAYER_TEAMMATE_COUNT: u32 = 0x0096_2620;
/// `thiscall` on a game setting (`011cdad0`): the address of its integer
/// value.
const GET_SETTING_INTEGER: u32 = 0x0043_d4d0;
/// `*(this + 4)` of a string global: its text.
const BS_STRING_TEXT: u32 = 0x0040_3df0;
/// Interface message with icon (`cdecl`: `text, 0, 0, 0, float, 0`).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// `ProcessLists` method (`00973460`, `thiscall` on the process lists
/// singleton, `actor`): a byte, non-zero when the actor detects the player.
const PROCESS_LISTS_IS_ACTOR_DETECTED: u32 = 0x0097_3460;
/// `thiscall` on a faction (`mask, flag`): sets or clears the bits of the
/// mask in the faction's flags word (`this + 0x34`) and notifies it.
const FACTION_SET_FLAG_BITS: u32 = 0x005f_c970;
/// The faction flag `005ce040` sets or clears.
const FACTION_FLAG_EXPELLED: u32 = 0x08;
/// `thiscall` on the player: the player's parent cell, `*(this + 0x40)`.
const GET_PARENT_CELL: u32 = 0x008d_6f30;
/// `TESObjectCELL::GetOwner` (Xbox PDB).
const CELL_GET_OWNER: u32 = 0x0054_6a40;
/// `thiscall` on a faction (`flag`): sets or clears the faction flag `0x40`
/// (it calls `005fc970` with that mask).
const FACTION_SET_MURDER_FLAG: u32 = 0x0047_ebb0;
/// Same shape as [`FACTION_SET_MURDER_FLAG`] (`0047eb90`).
const FACTION_SET_ENEMY_FLAG: u32 = 0x0047_eb90;
/// Same shape as [`FACTION_SET_MURDER_FLAG`] (`0047ebd0`).
const FACTION_SET_ATTACK_FLAG: u32 = 0x0047_ebd0;
/// `thiscall` on a form (`flag`): sets or clears form flag `0x800000`
/// (`this + 8`) and notifies the form.
const FORM_SET_FLAG_800000: u32 = 0x0048_4650;
/// `thiscall` on the player (`value, flag`): stores a `dword` at `+0x654` and
/// a byte at `+0x658` (`005c1a00`, another part of the unit).
const PLAYER_SET_FIELD_654: u32 = 0x005c_1a00;
/// `thiscall` on an actor (`flag`): stores the force-run byte at `+0x124`
/// (`005bf800`, another part of the unit).
const ACTOR_SET_FORCE_RUN: u32 = 0x005b_f800;
/// `thiscall` on an actor: the byte at `+0x124` (the force-run flag).
const ACTOR_GET_FORCE_RUN: u32 = 0x008d_8220;
/// `thiscall` on an actor: the byte at `+0x125` (the force-sneak flag).
const ACTOR_GET_FORCE_SNEAK: u32 = 0x005c_e8f0;
/// `thiscall` on the player: the player's level (`u16`).
const PLAYER_GET_LEVEL: u32 = 0x0087_f9f0;
/// `thiscall` on a reference: its base form, `*(this + 0x20)`.
const GET_BASE_FORM_OF_REFERENCE: u32 = 0x0041_81e0;
/// `thiscall` on the actor base data at `form + 0x30` (`level`): stores the
/// level word at `+0x0c` and notifies.
const ACTOR_BASE_DATA_SET_LEVEL: u32 = 0x0047_dfe0;
/// `Interface::CreateLevelUpMenu` (Xbox PDB).
const CREATE_LEVEL_UP_MENU: u32 = 0x0070_6270;

// ---- Globals and constants ---------------------------------------------------

/// The `PlayerCharacter` singleton pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// The process lists singleton (the `this` of
/// [`PROCESS_LISTS_IS_ACTOR_DETECTED`]).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The follower-limit game setting.
const FOLLOWER_LIMIT_SETTING: u32 = 0x011c_dad0;
/// The string global whose text is the "too many followers" message.
const TOO_MANY_FOLLOWERS_MESSAGE: u32 = 0x011d_3d54;
/// `float` the message is shown with.
const MESSAGE_DURATION: u32 = 0x0101_62c0;
/// Byte at `+0x268` of the TLS block: commands echo to the console when set.
const TLS_ECHO: u32 = 0x268;

/// Virtual slot `0x130` of a package: its name.
const PACKAGE_NAME_SLOT: u32 = 0x130;
/// Virtual slot `0x284` of the actor's process: sets the package stage.
const PROCESS_SET_STAGE_SLOT: u32 = 0x284;
/// Virtual slot `0x128` of the actor's process: the actor's target.
const PROCESS_GET_TARGET_SLOT: u32 = 0x128;
/// Virtual slot `0x42c` of an actor: its combat target.
const ACTOR_GET_COMBAT_TARGET_SLOT: u32 = 0x42c;

// ---- String literals (addresses in the exe's data) ---------------------------

/// `" %s is not detected"`
const MSG_NOT_DETECTED: u32 = 0x0103_b6b8;
/// `" %s is detected"`
const MSG_DETECTED: u32 = 0x0103_b6cc;
/// `"PACKAGES: Package %s is not  %s current package"`
const MSG_PACKAGE_NOT_CURRENT: u32 = 0x0103_b6dc;
/// `"PACKAGES: Package %s is not a follow or an escort. "`
const MSG_NOT_FOLLOW_OR_ESCORT: u32 = 0x0103_b70c;
/// `"GetActionRef >> (%08x)"`
const MSG_GET_ACTION_REF: u32 = 0x0103_b740;
/// `"GetSelf >> (%08x)"`
const MSG_GET_SELF: u32 = 0x0103_b758;
/// `"GetCombatTarget >> (%08x)"`
const MSG_GET_COMBAT_TARGET: u32 = 0x0103_b76c;
/// `"GetPackageTarget >> (%08x)"`
const MSG_GET_PACKAGE_TARGET: u32 = 0x0103_b788;
/// `"GetContainer >>(%08x)"`
const MSG_GET_CONTAINER: u32 = 0x0103_b7a4;
/// `"GetParentRef >> (%08x)"`
const MSG_GET_PARENT_REF: u32 = 0x0103_b7bc;
/// `"GetLinkedRef >> (%08x)"`
const MSG_GET_LINKED_REF: u32 = 0x0103_b7d4;
/// `"GetForceRun >> %0.2f"`
const MSG_GET_FORCE_RUN: u32 = 0x0103_b7ec;
/// `"SetForceRun >> %0.2f"`
const MSG_SET_FORCE_RUN: u32 = 0x0103_b804;
/// `"GetForceSneak >> %0.2f"`
const MSG_GET_FORCE_SNEAK: u32 = 0x0103_b81c;
/// `"SetForceSneak >> %0.2f"`
const MSG_SET_FORCE_SNEAK: u32 = 0x0103_b834;

// ---- Callees of the second batch (by exe address) -----------------------------

/// `cdecl` (`thisObj, 0, 0, result`): the condition behind `GetDefaultOpen`.
/// False without a reference; otherwise the result double is 1.0 when the
/// bits of mask 8 are set in the reference's flags word
/// ([`REFERENCE_TEST_ACTION_BITS`]), else 0.0, and true.
const GET_DEFAULT_OPEN_CONDITION: u32 = 0x005a_27e0;
/// `thiscall` on a reference (`mask`): whether any bit of `mask` is set in
/// the flags word kept in the extra data list (`0041b3a0`).
const REFERENCE_TEST_ACTION_BITS: u32 = 0x0057_2d30;
/// `thiscall` on a reference (`mask`): sets the bits of `mask` in that flags
/// word (`0041b440`); for mask 4 it first removes (`00561d90` true) or adds
/// the change `0x800000` of the form (virtual slots `0x4c` / `0x48`).
const REFERENCE_SET_ACTION_BITS: u32 = 0x0057_2d50;
/// `TESObjectREFR::ClearAction` (Xbox PDB), `thiscall` (`mask`): clears the
/// bits of `mask` in that flags word.
const REFERENCE_CLEAR_ACTION_BITS: u32 = 0x0057_2db0;
/// The action mask the default-open commands work on.
const ACTION_BIT_DEFAULT_OPEN: u32 = 8;
/// `ExtraDataList::RemoveSavedAnimation` (Xbox PDB), `thiscall`.
const REMOVE_SAVED_ANIMATION: u32 = 0x0042_2aa0;
/// `ExtraDataList::RemoveLastFinishedSequence` (Xbox PDB), `thiscall`.
const REMOVE_LAST_FINISHED_SEQUENCE: u32 = 0x0042_2920;
/// `thiscall` on a 4-byte change-flags value (`flags`): stores the flags
/// (the map calls it `BGSChangeFlags::IsForcedChange`).
const CHANGE_FLAGS_CONSTRUCT: u32 = 0x008c_71b0;
/// The change flag `SetDefaultOpen` asks about and sets.
const CHANGE_FLAG_OPEN_STATE: u32 = 0x0040_0000;
/// The `BGSSaveLoadGame` singleton pointer.
const SAVE_LOAD_GAME: u32 = 0x011d_df38;
/// `BGSSaveLoadGame::GetChange` (Xbox PDB), `thiscall` (`reference, change
/// flags by value`, `RET 8`): the map's name; its body asks the object at
/// `*this` about the form id of the reference and the flags.
const SAVE_LOAD_GET_CHANGE: u32 = 0x0084_a6d0;
/// `BGSOpenCloseForm::SetOpenState` (Xbox PDB), `cdecl` (`reference, state,
/// flag`).
const SET_OPEN_STATE: u32 = 0x0047_aec0;
/// `BGSOpenCloseForm::GetOpenState` (Xbox PDB), `cdecl` (`reference`).
const GET_OPEN_STATE: u32 = 0x0047_b250;
/// `TESObjectREFR::Activate` (Xbox PDB), `thiscall` (four words).
const REFERENCE_ACTIVATE: u32 = 0x0057_3170;
/// `TESObjectREFR::SetLastFinishedSequence` (Xbox PDB), `thiscall` (`name`).
const SET_LAST_FINISHED_SEQUENCE: u32 = 0x0057_8a30;
/// `Interface::CreateRaceSexMenu` (Xbox PDB), `cdecl` (`mode`).
const CREATE_RACE_SEX_MENU: u32 = 0x0070_5870;
/// `cdecl`, takes the eight command words and ignores them: closes the
/// console, queues menu 9 for the player and returns true.
const CLOSE_CONSOLE_AND_QUEUE_MENU: u32 = 0x005d_a540;
/// `Interface::QueueMenuCreate` (Xbox PDB), `cdecl` (`menu id, reference,
/// 0, 0, 1, 0`).
const QUEUE_MENU_CREATE: u32 = 0x0070_9470;
/// `thiscall` on a talking activator (`*(this + 0x90)`).
const TALKING_ACTIVATOR_GET_SPEAKER: u32 = 0x0051_6bf0;
/// `thiscall` on a reference: the byte at `+0x81`.
const REFERENCE_GET_BYTE_81: u32 = 0x0057_4900;
/// `thiscall` on a reference: `*(this + 0x6c)`.
const REFERENCE_GET_FIELD_6C: u32 = 0x005e_3fa0;
/// `cdecl` (`container, amount`): opens the barter menu on the container
/// when the interface is in the right state.
const BARTER_MENU_CREATE: u32 = 0x0070_4f80;
/// RTTI type descriptor of `TESBoundObject` (`.?AVTESBoundObject@@`).
const RTTI_TES_BOUND_OBJECT: u32 = 0x0118_3108;
/// RTTI type descriptor of `BGSTalkingActivator`.
const RTTI_BGS_TALKING_ACTIVATOR: u32 = 0x0118_9e04;
/// RTTI type descriptor of `TESCaravanCard`.
const RTTI_TES_CARAVAN_CARD: u32 = 0x0118_c5c4;
/// RTTI type descriptor of `TESPackage`.
const RTTI_TES_PACKAGE: u32 = 0x0118_46a0;
/// RTTI type descriptor of `FleePackage`.
const RTTI_FLEE_PACKAGE: u32 = 0x0118_c6f4;
/// RTTI type descriptor of `TESNPC`.
const RTTI_TES_NPC: u32 = 0x0118_3a1c;

/// Virtual slot `0xfc` of a reference (`IsMobileObject`, Xbox PDB name).
const IS_MOBILE_OBJECT_SLOT: u32 = 0xfc;
/// Virtual slot `0x100` of a reference (`IsActor`, Xbox PDB name).
const IS_ACTOR_SLOT: u32 = 0x100;
/// Virtual slot `0x48` of a form: `AddChange(flags)` (Xbox PDB name).
const ADD_CHANGE_SLOT: u32 = 0x48;
/// Virtual slot `0x4c` of a form: `RemoveChange(flags)` (Xbox PDB name).
const REMOVE_CHANGE_SLOT: u32 = 0x4c;
/// Virtual slot `0x1d0` of a reference: an object (the 3D root of the
/// reference), null when it has none.
const REFERENCE_GET_3D_SLOT: u32 = 0x1d0;

/// `thiscall` on the player: `this + 0x764`, the head of a list of regions.
const PLAYER_REGION_LIST: u32 = 0x004f_2770;
/// `thiscall`: returns `this` (the list node's item cell lies at `node + 4`).
const LIST_ITEM_CELL: u32 = 0x0068_15c0;
/// `thiscall` on the item cell: `*(this + 4)`, the next node's cell.
const LIST_NEXT_CELL: u32 = 0x0072_6070;

/// `SPECIALBookMenu::Create` (Xbox PDB), `cdecl`, no arguments (the
/// variant without a book).
const SPECIAL_BOOK_MENU_CREATE: u32 = 0x007c_5fb0;
/// `SPECIALBookMenu::Create_ov2` (Xbox PDB), `cdecl` (`book`).
const SPECIAL_BOOK_MENU_CREATE_OV2: u32 = 0x007c_5fc0;
/// `SlotMachineMenu::Create` (Xbox PDB), `cdecl` (five words).
const SLOT_MACHINE_MENU_CREATE: u32 = 0x007c_0a40;
/// `BlackJackMenu::Create` (Xbox PDB), `cdecl` (five words).
const BLACK_JACK_MENU_CREATE: u32 = 0x0073_3630;
/// `RouletteMenu::Create` (Xbox PDB), `cdecl` (five words).
const ROULETTE_MENU_CREATE: u32 = 0x007b_be20;
/// `CaravanMenu::Create` (Xbox PDB), `cdecl` (`container, a, b, float, 0`).
const CARAVAN_MENU_CREATE: u32 = 0x0074_1060;
/// `LoveTesterMenu::Create` (Xbox PDB), `cdecl`, no arguments.
const LOVE_TESTER_MENU_CREATE: u32 = 0x0079_15e0;
/// `LoveTesterMenu::Create_ov2` (Xbox PDB), `cdecl` (`one word`).
const LOVE_TESTER_MENU_CREATE_OV2: u32 = 0x0079_15f0;
/// `TraitSelectMenu::Create` (Xbox PDB), `cdecl`, no arguments.
const TRAIT_SELECT_MENU_CREATE: u32 = 0x007e_7b60;
/// `TraitMenu::Create` (Xbox PDB), `cdecl`, no arguments.
const TRAIT_MENU_CREATE: u32 = 0x007e_6990;
/// `thiscall` on the player (`card`): adds a caravan card to the player's
/// cards and returns true.
const PLAYER_ADD_CARD: u32 = 0x0096_9bc0;

/// `operator new` (`cdecl`, size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `thiscall` on a reference (`type`): the extra data of that type, read
/// from the reference's extra data list (null when it has none).
const REFERENCE_GET_EXTRA_DATA: u32 = 0x0052_7080;
/// Constructor of `ExtraSecuritronFace` (`thiscall`, 0x1c bytes).
const SECURITRON_FACE_CONSTRUCT: u32 = 0x0042_ccc0;
/// The extra data type of `ExtraSecuritronFace`.
const EXTRA_TYPE_SECURITRON_FACE: u32 = 0x8f;
/// `BaseExtraList::AddExtra` (Xbox PDB), `thiscall` on the list (`extra`).
const ADD_EXTRA: u32 = 0x0040_ff60;
/// `ExtraSecuritronFace::SetPersonality` (Xbox PDB), `thiscall` (`text`).
const SECURITRON_FACE_SET_PERSONALITY: u32 = 0x0043_8260;
/// `ExtraSecuritronFace::SetMood` (Xbox PDB), `thiscall` (`text`).
const SECURITRON_FACE_SET_MOOD: u32 = 0x0043_8280;
/// `ExtraSecuritronFace::ApplyFace` (Xbox PDB), `thiscall` (`3D root`).
const SECURITRON_FACE_APPLY_FACE: u32 = 0x0043_7f90;
/// Change flags set on the reference after a securitron face was applied.
const CHANGE_FLAG_SECURITRON_FACE_A: u32 = 0x1000_0000;
/// Second change flag set after a securitron face was applied.
const CHANGE_FLAG_SECURITRON_FACE_B: u32 = 0xa406_1840;

/// `thiscall` on a string (`BSStringT`, 8 bytes): constructor.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `thiscall` on a string: destructor.
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `thiscall` on a string (`text`): assigns a C string (`004037f0(text, 0)`).
const STRING_ASSIGN: u32 = 0x0043_8390;
/// `thiscall` on a string: its length (the `u16` at `+4`, or the length of
/// the text when that is `0xffff`).
const STRING_LENGTH: u32 = 0x0040_48e0;
/// `thiscall`: `*this`, the text of a string or the object of a smart
/// pointer.
const POINTER_GET: u32 = 0x0055_9450;
/// `cdecl` varargs (`string, format, ...`): formats into a string.
const STRING_FORMAT: u32 = 0x0040_6f60;
/// `cdecl` (`buffer, size, format, ...`): `vsprintf_s`.
const FORMAT_INTO_BUFFER: u32 = 0x0040_6d00;
/// `SwapPlatformLanguageTexturePath` (Xbox PDB), `cdecl` (`path, result,
/// size`).
const SWAP_PLATFORM_LANGUAGE_TEXTURE_PATH: u32 = 0x004b_7240;
/// `thiscall` on a `NiFixedString` handle (`text`): interns the text (a
/// null text gives the pooled empty handle).
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
/// `thiscall` on a `NiFixedString` handle: releases it.
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// Virtual slot `0x9c` of the 3D root: finds a node by name (`NiFixedString`
/// handle by address).
const FIND_NODE_BY_NAME_SLOT: u32 = 0x9c;
/// `NiPointer` constructor (`thiscall`, `pointer`): stores it and takes a
/// reference.
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiPointer` destructor (`thiscall`): drops the reference.
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `TES::CreateTextureImage` (Xbox PDB), `thiscall` on the TES singleton
/// (`path, NiPointer out, 0, 0`).
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
/// `NiAVObject::GetProperty` (Xbox PDB), `thiscall` (`property type`).
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// `cdecl` (`RTTI, object`): the object if it is of that type or derived
/// from it, else null (null for a null object).
const NI_DYNAMIC_CAST: u32 = 0x0065_3270;
/// RTTI of `NiShadeProperty` (`NiShadeProperty::GetRTTI`, Xbox PDB).
const RTTI_NI_SHADE_PROPERTY: u32 = 0x011f_5ae0;
/// RTTI of the property class whose texture slot `00438230` sets (it is
/// referenced from the `BSShader` code).
const RTTI_TEXTURE_SET_PROPERTY: u32 = 0x011f_a05c;
/// `thiscall`: `*(this + 0x1c)`, `BSShaderProperty::iShaderPropertyType`
/// (Xbox PDB; the map names this body `PathingLocation::GetWorldspace`).
const GET_SHADER_PROPERTY_TYPE: u32 = 0x0044_1110;
/// `cdecl`, no arguments: returns 3, the property type asked for.
const PROPERTY_TYPE_SHADE: u32 = 0x0043_8220;
/// Virtual slot `0xf0` of the shade property: takes `(0, 0, texture)`.
const SHADE_PROPERTY_SET_TEXTURE_SLOT: u32 = 0xf0;
/// `thiscall` on the second property (`texture`): stores the texture at
/// `+0x60` (a smart pointer assignment) and clears the dword at `+0x38`.
const PROPERTY_SET_TEXTURE: u32 = 0x0043_8230;
/// The property type `SwapTexture` asks the 3D root for.
const PROPERTY_TYPE_ASKED: u32 = 3;
/// Pointer to the `HUDMainMenu` singleton.
const HUD_MAIN_MENU: u32 = 0x011d_96c0;
/// `HUDMainMenu::bForceObjectiveReminders` (Xbox PDB, `+0x205`; the PC code
/// uses the same offset).
const HUD_FORCE_OBJECTIVE_REMINDERS: u32 = 0x205;

/// `thiscall` on an actor: whether its life state is 3 (`*(this + 0x108)`).
const ACTOR_IS_LIFE_STATE_3: u32 = 0x0043_7bd0;
/// `thiscall` on an actor (`flag`): sets the life state to 3 when the flag
/// is set, back to 0 when it is clear and the state is 3.
const ACTOR_SET_LIFE_STATE_3: u32 = 0x008a_ce10;
/// `thiscall` on an actor (`flag`): the same for life state 5.
const ACTOR_SET_LIFE_STATE_5: u32 = 0x008a_ce50;
/// `Actor::EndMovement` (Xbox PDB), `thiscall`.
const ACTOR_END_MOVEMENT: u32 = 0x0087_faa0;
/// `Actor::SetGhost` (Xbox PDB), `thiscall` (`flag`).
const ACTOR_SET_GHOST: u32 = 0x008a_cf10;
/// `Actor::QueueEquipObject` (Xbox PDB), `thiscall` (`item, 1, 0, 1, worn
/// flag, 1`).
const ACTOR_QUEUE_EQUIP_OBJECT: u32 = 0x0088_c650;
/// `Actor::QueueUnEquipObject` (Xbox PDB), `thiscall` (same six words).
const ACTOR_QUEUE_UNEQUIP_OBJECT: u32 = 0x0088_c790;
/// `ExtraDataList::GetContainerChanges` (Xbox PDB), `thiscall` on the list.
const GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
/// `InventoryChanges::GetInventoryItem` (Xbox PDB), `thiscall` on the
/// changes (`item, 0`): a heap object describing the item's entry.
const GET_INVENTORY_ITEM: u32 = 0x004d_0650;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), `cdecl`
/// (`reference`).
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::WearingObject` (Xbox PDB), `thiscall` on the changes
/// (`item, 0`).
const WEARING_OBJECT: u32 = 0x004b_fda0;
/// `ExtraDataList::GetWorn` (Xbox PDB), `thiscall` on an extra data list
/// (`0`).
const EXTRA_LIST_GET_WORN: u32 = 0x0041_8ab0;
/// `ExtraDataList::SetCanNotWear` (Xbox PDB), `thiscall` (`flag`).
const EXTRA_LIST_SET_CAN_NOT_WEAR: u32 = 0x0041_ab70;
/// Scalar deleting destructor of the inventory entry (`thiscall`, `1`).
const INVENTORY_ENTRY_DELETE: u32 = 0x0044_59e0;
/// Constructor of an `ExtraDataList` (`thiscall`, 0x20 bytes).
const EXTRA_LIST_CONSTRUCT: u32 = 0x0041_0360;
/// Constructor of the two-word list head (`thiscall`, 8 bytes).
const LIST_HEAD_CONSTRUCT: u32 = 0x0096_a2d0;
/// `thiscall` on a list head (`item cell address`): appends the item.
const LIST_APPEND: u32 = 0x005a_e3d0;
/// `TESFullName::GetFullName` (Xbox PDB), `cdecl` (`form`): the name text.
const GET_FULL_NAME: u32 = 0x0048_2720;
/// `PlayerCharacter::UpdatePlayer3d` (Xbox PDB), `thiscall`.
const PLAYER_UPDATE_3D: u32 = 0x0095_0030;
/// `cdecl`, no arguments: refreshes the open inventory-like menu.
const REFRESH_MENUS: u32 = 0x0070_4af0;
/// Virtual slot `0x580` of the player's process: takes `(1, 0, 0)`.
const PROCESS_REFRESH_SLOT: u32 = 0x580;
/// A string global's text: the text shown after an equipped item.
const EQUIPPED_TEXT_GLOBAL: u32 = 0x011d_2a0c;
/// A string global's text: the text shown after an unequipped item.
const UNEQUIPPED_TEXT_GLOBAL: u32 = 0x011d_424c;

/// `thiscall` on a `TESNPC` (`value`): stores it at `+0x130` and sets change
/// flags `0x400` (virtual slot `0x48`).
const NPC_SET_FIELD_130: u32 = 0x0060_1c70;
/// `TESNPC::InitValues` (Xbox PDB), `thiscall` (`flag`).
const NPC_INIT_VALUES: u32 = 0x0060_3be0;
/// Virtual slot `0x434` of an actor, takes `(0)`.
const ACTOR_SLOT_434: u32 = 0x434;
/// Virtual slot `0x614` of the actor's process, takes `(0x800)`.
const PROCESS_SLOT_614: u32 = 0x614;
/// Virtual slot `0x71c` of the actor's process, takes `(0)`.
const PROCESS_SLOT_71C: u32 = 0x71c;
/// Virtual slot `0x338` of the actor's process, takes a `float`.
const PROCESS_SLOT_338: u32 = 0x338;
/// A `float` global the commands pass to process slots.
const FLOAT_ARGUMENT: u32 = 0x0101_2054;

/// Virtual slots of the actor's process that [`fn_005cfdf0`] reads.
const PROCESS_SLOT_4D0: u32 = 0x4d0;
const PROCESS_SLOT_4D4: u32 = 0x4d4;
const PROCESS_SLOT_4C8: u32 = 0x4c8;
/// Virtual slot `0x374` of the parsed actor: takes the four words
/// `(thisObj, 4c8, 4d4, 4d0)`.
const ACTOR_SLOT_374: u32 = 0x374;
/// Virtual slot `0x378` of the parsed actor: takes `(thisObj)`.
const ACTOR_SLOT_378: u32 = 0x378;
/// Virtual slot `0x428` of an actor: its current package target (null when
/// none).
const ACTOR_SLOT_428: u32 = 0x428;
/// Virtual slot `0x27c` of the actor's process: its package.
const PROCESS_SLOT_27C: u32 = 0x27c;
/// Virtual slot `0x410` of an actor: takes eight words.
const ACTOR_SLOT_410: u32 = 0x410;
/// `thiscall`: `*(this + 0xc0)`.
const GET_FIELD_C0: u32 = 0x0040_30b0;
/// `thiscall` (two words), does nothing (`RET 8`).
const NOOP_TWO_WORDS: u32 = 0x008d_0600;
/// `thiscall`: `*(this + 0x2c)`, the package location of a package.
const PACKAGE_GET_LOCATION: u32 = 0x0055_b980;
/// `PackageLocation::SetLocReference` (Xbox PDB), `thiscall` (`reference`).
const PACKAGE_LOCATION_SET_REFERENCE: u32 = 0x0067_f3c0;
/// `PackageLocation::SetLocCell` (Xbox PDB), `thiscall` (`cell`).
const PACKAGE_LOCATION_SET_CELL: u32 = 0x0067_f410;
/// `thiscall` on a package (`flag`): stores the byte at `+0x80`.
const PACKAGE_SET_BYTE_80: u32 = 0x005d_0b80;

/// `cdecl` (`thisObj, 0, 0, result`), in `tesconditionfunctions.cpp`: stores
/// 0.0 in the result and returns true.
const UNKNOWN_CONDITION_005A2A50: u32 = 0x005a_2a50;
/// `Script::GetSandmanConditionFunction` (Xbox PDB).
const GET_SANDMAN_CONDITION: u32 = 0x005a_2810;
/// `Script::GetCannibalConditionFunction` (Xbox PDB).
const GET_CANNIBAL_CONDITION: u32 = 0x005a_2890;
/// `Script::HasBeenEatenConditionFunction` (Xbox PDB).
const HAS_BEEN_EATEN_CONDITION: u32 = 0x005a_29d0;
/// `Script::GetGhostConditionFunction` (Xbox PDB).
const GET_GHOST_CONDITION: u32 = 0x005a_2a60;
/// `Script::GetUnconsciousConditionFunction` (Xbox PDB).
const GET_UNCONSCIOUS_CONDITION: u32 = 0x0059_fee0;
/// `Script::GetRestrainedConditionFunction` (Xbox PDB).
const GET_RESTRAINED_CONDITION: u32 = 0x0059_ff90;

// ---- String literals of the second batch ---------------------------------------

/// `"GetDefaultOpen >> %0.2f"`
const MSG_GET_DEFAULT_OPEN: u32 = 0x0103_b84c;
/// `"SetDefaultOpen >> %0.2f"`
const MSG_SET_DEFAULT_OPEN: u32 = 0x0103_b864;
/// `"Invalid EditorFormID used in script ShowSlotMachineMenu ..."`
const MSG_SLOT_MACHINE_FORM: u32 = 0x0103_b880;
/// `"Invalid Parameters used in script ShowSlotMachineMenu "`
const MSG_SLOT_MACHINE_PARAMETERS: u32 = 0x0103_b8d8;
/// `"Invalid EditorFormID used in script ShowBlackJackMenu ..."`
const MSG_BLACK_JACK_FORM: u32 = 0x0103_b910;
/// `"Invalid Parameters used in script ShowBlackJackMenu "`
const MSG_BLACK_JACK_PARAMETERS: u32 = 0x0103_b968;
/// `"Invalid EditorFormID used in script ShowBlackJackMenu ..."` (the
/// roulette command carries the blackjack text).
const MSG_ROULETTE_FORM: u32 = 0x0103_b9a0;
/// `"Invalid Parameters used in script ShowRouletteMenu"`
const MSG_ROULETTE_PARAMETERS: u32 = 0x0103_b9f8;
/// `"Invalid Parameters used in script ShowCaravanMenu "`
const MSG_CARAVAN_PARAMETERS: u32 = 0x0103_ba2c;
/// `"%s is not a valid CaravanCard object"`
const MSG_NOT_CARAVAN_CARD: u32 = 0x0103_ba60;
/// `"Invalid Parameter used in script AddCardToPlayer"`
const MSG_ADD_CARD_PARAMETER: u32 = 0x0103_ba88;
/// `"IsPlayerInRegion >> %0.f"`
const MSG_IS_PLAYER_IN_REGION: u32 = 0x0103_babc;
/// `"Invalid Parameter used in script IsPlayerInRegion"`
const MSG_IS_PLAYER_IN_REGION_PARAMETER: u32 = 0x0103_bad8;
/// `"Textures\%s.dds"`
const TEXTURE_PATH_FORMAT: u32 = 0x0103_bb0c;
/// `"SetGhost >> %d"`
const MSG_SET_GHOST: u32 = 0x0103_bb1c;
/// `"SCRIPTS: EquipItem in script '%s' failed to generate an item."`
const MSG_EQUIP_ITEM_FAILED: u32 = 0x0103_bb2c;
/// `"%s %s."`
const ITEM_MESSAGE_FORMAT: u32 = 0x0103_bb6c;
/// `"Interface\Icons\Message Icons\glow_message_vaultboy_neutral.dds"`
const MESSAGE_ICON: u32 = 0x0102_08e0;
/// `"Open"`
const SEQUENCE_OPEN: u32 = 0x0101_1f30;
/// `"Close"`
const SEQUENCE_CLOSE: u32 = 0x0101_abac;

/// The `TES` singleton pointer.
const TES_SINGLETON: u32 = 0x011d_ea10;

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

/// `__RTDynamicCast` of `object` from `TESObjectREFR` to `Actor`.
fn actor_of(e: &mut Engine, object: u32) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![object, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
    )
    .u32()
}

/// Calls a condition function with the command's `thisObj`, `argument`, 0 and
/// the result double: its `AL`.
fn call_condition(e: &mut Engine, function: u32, a: ScriptArgs, argument: u32) -> bool {
    e.call(function, &args![a.this_obj, argument, 0u32, a.result])
        .bool()
}

/// Stores the form id of `form` into the result double through
/// `Script::PutNumericIDInDouble` and returns the id (the game keeps it in a
/// local for the echo).
fn store_form_id(e: &mut Engine, form: u32, result: Ptr) -> u32 {
    let id = e.call(GET_FORM_ID, &args![form]).u32();
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), id);
        e.call(PUT_NUMERIC_ID_IN_DOUBLE, &args![cell, result]);
    });
    id
}

/// The echo of the reference-returning commands: `"<name> >> (%08x)"`.
fn echo_id(e: &mut Engine, format: u32, id: u32) {
    if echo_enabled(e) {
        console_print(e, &args![format, id]);
    }
}

/// Echo of the commands that return a flag in the result double.
fn echo_result(e: &mut Engine, format: u32, result: Ptr) {
    if echo_enabled(e) {
        let value = e.mem.f64(result.addr());
        console_print(e, &args![format, value]);
    }
}

/// The check both package commands start with: the package must be the
/// actor's current package, or the one in its extra data. Otherwise the
/// logging stub gets the message and this returns false.
fn package_is_current(e: &mut Engine, actor: u32, package: u32) -> bool {
    if e.call(GET_PACKAGE_SET_AS_CURRENT, &args![actor]).u32() == package {
        return true;
    }
    let extra_data = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
    if e.call(GET_PACKAGE_EXTRA, &args![extra_data]).u32() == package {
        return true;
    }
    let actor_name = e.call(GET_REFERENCE_NAME, &args![actor]).u32();
    let package_name = e.vcall(package, PACKAGE_NAME_SLOT, &args![]).u32();
    e.call(
        LOG_STUB,
        &args![MSG_PACKAGE_NOT_CURRENT, package_name, actor_name],
    );
    false
}

/// The setter commands of a faction flag: parse a faction and a flag, call
/// `setter(faction, flag != 0)`.
fn set_faction_flag(e: &mut Engine, a: ScriptArgs, setter: u32) -> bool {
    let Some([faction, flag]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    e.call(setter, &args![faction, (flag != 0) as u32]);
    true
}

// ---- Translated functions --------------------------------------------------------

// Translated from 005cd990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDetectionLevelFunction` (Xbox PDB): parses one actor argument
/// and returns `GetDetectionLevelConditionFunction(thisObj, actor, 0,
/// result)`.
pub fn script_get_detection_level_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([actor]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, GET_DETECTION_LEVEL_CONDITION, a, actor)
}

// Translated from 005cd9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `IsSwimming` body: false without a `thisObj`, otherwise
/// `IsSwimmingConditionFunction(thisObj, 0, 0, result)`.
pub fn fn_005cd9f0(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return false;
    }
    call_condition(e, IS_SWIMMING_CONDITION, a, 0)
}

// Translated from 005cda20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsActorDetected` (Xbox PDB): when `thisObj` is an actor, the
/// result is 1.0 if the process lists say that it detects the player, else
/// 0.0; with the TLS echo flag set the console says whether the actor is
/// detected. Always succeeds.
pub fn script_is_actor_detected(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = actor_of(e, a.this_obj.addr());
    if actor != 0 {
        let detected = e
            .call(
                PROCESS_LISTS_IS_ACTOR_DETECTED,
                &args![PROCESS_LISTS, actor],
            )
            .u8();
        e.mem.set_f64(a.result.addr(), detected as f64);
        if echo_enabled(e) {
            let format = if e.mem.f64(a.result.addr()) == 0.0 {
                MSG_NOT_DETECTED
            } else {
                MSG_DETECTED
            };
            let name = e.call(GET_REFERENCE_NAME, &args![actor]).u32();
            console_print(e, &args![format, name]);
        }
    }
    true
}

// Translated from 005cdad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A package command (argument: a package): when `thisObj` is an actor and
/// the package is its current one, a package of type 2 gets its flag set and
/// the actor's process is told stage 2 (virtual slot `0x284`); a package of
/// type 7 or 1 only gets the flag set; any other type is reported through
/// the logging stub.
pub fn fn_005cdad0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([package]) = parse_params(e, a, [0]) else {
        return false;
    };
    let actor = actor_of(e, a.this_obj.addr());
    if actor != 0 && package != 0 {
        if !package_is_current(e, actor, package) {
            return true;
        }
        if e.call(GET_PACKAGE_TYPE, &args![package]).i32() == 2 {
            e.call(SET_PACKAGE_FLAG, &args![package, 1u32]);
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            e.vcall(process, PROCESS_SET_STAGE_SLOT, &args![2u32]);
        } else if e.call(GET_PACKAGE_TYPE, &args![package]).i32() == 7
            || e.call(GET_PACKAGE_TYPE, &args![package]).i32() == 1
        {
            e.call(SET_PACKAGE_FLAG, &args![package, 1u32]);
        } else {
            let package_name = e.vcall(package, PACKAGE_NAME_SLOT, &args![]).u32();
            e.call(LOG_STUB, &args![MSG_NOT_FOLLOW_OR_ESCORT, package_name]);
        }
    }
    true
}

// Translated from 005cdc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The counterpart of [`fn_005cdad0`] (argument: a package): after the same
/// check, a package of type 2 gets its flag cleared and the actor's process
/// is told stage 3; a package of type 7 or 1 has its flag cleared, unless the
/// actor's target (process virtual slot `0x128`, cast to an actor) is the
/// player, the player does not have the actor as a teammate yet and has more
/// teammates than the setting at `011cdad0` allows: then the "too many
/// followers" message is shown and the flag stays. Any other type is
/// reported through the logging stub.
pub fn fn_005cdc10(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([package]) = parse_params(e, a, [0]) else {
        return false;
    };
    let actor = actor_of(e, a.this_obj.addr());
    if actor != 0 && package != 0 {
        if !package_is_current(e, actor, package) {
            return true;
        }
        if e.call(GET_PACKAGE_TYPE, &args![package]).i32() == 2 {
            e.call(SET_PACKAGE_FLAG, &args![package, 0u32]);
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            e.vcall(process, PROCESS_SET_STAGE_SLOT, &args![3u32]);
        } else if e.call(GET_PACKAGE_TYPE, &args![package]).i32() == 7
            || e.call(GET_PACKAGE_TYPE, &args![package]).i32() == 1
        {
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            let target = e.vcall(process, PROCESS_GET_TARGET_SLOT, &args![]).u32();
            let target = actor_of(e, target);
            let player = e.global::<u32>(PLAYER);
            if target == player && !e.call(PLAYER_HAS_TEAMMATE, &args![player, actor]).bool() {
                let teammates = e.call(PLAYER_TEAMMATE_COUNT, &args![player]).i32();
                let limit_address = e
                    .call(GET_SETTING_INTEGER, &args![FOLLOWER_LIMIT_SETTING])
                    .u32();
                if teammates > e.mem.i32(limit_address) {
                    let text = e
                        .call(BS_STRING_TEXT, &args![TOO_MANY_FOLLOWERS_MESSAGE])
                        .u32();
                    let duration: f32 = e.global(MESSAGE_DURATION);
                    e.call(SHOW_MESSAGE, &args![text, 0u32, 0u32, 0u32, duration, 0u32]);
                    return true;
                }
            }
            e.call(SET_PACKAGE_FLAG, &args![package, 0u32]);
        } else {
            let package_name = e.vcall(package, PACKAGE_NAME_SLOT, &args![]).u32();
            e.call(LOG_STUB, &args![MSG_NOT_FOLLOW_OR_ESCORT, package_name]);
        }
    }
    true
}

// Translated from 005cde00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the player's `dword` at `+0x654` ([`fn_005cde20`]) into the result
/// double as an integer. Always succeeds.
pub fn fn_005cde00(e: &mut Engine, a: ScriptArgs) -> bool {
    let player = e.global::<u32>(PLAYER);
    let value = fn_005cde20(e, Ptr::new(player));
    e.mem.set_f64(a.result.addr(), value as f64);
    true
}

// Translated from 005cde20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter`'s `dword` at `+0x654` (PC offset).
pub fn fn_005cde20(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i32(this.addr() + 0x654)
}

// Translated from 005cde40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer and stores it, with the flag 1, in the player's members
/// at `+0x654` / `+0x658` (`005c1a00`).
pub fn fn_005cde40(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    let player = e.global::<u32>(PLAYER);
    e.call(PLAYER_SET_FIELD_654, &args![player, value, 1u32]);
    true
}

// Translated from 005cdea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetAmountStolenSold` body: calls
/// `GetAmountStolenSoldConditionFunction(thisObj, 0, 0, result)`. Always
/// succeeds.
pub fn fn_005cdea0(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_AMOUNT_STOLEN_SOLD_CONDITION, a, 0);
    true
}

// Translated from 005cdec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer and adds it to the player's `dword` at `+0x6dc`
/// ([`fn_005cdf20`]).
pub fn fn_005cdec0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([amount]) = parse_params(e, a, [0]) else {
        return false;
    };
    let player = e.global::<u32>(PLAYER);
    fn_005cdf20(e, Ptr::new(player), amount as i32);
    true
}

// Translated from 005cdf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `amount` to the `dword` at `this + 0x6dc` of the player (PC offset;
/// the Xbox PDB has `iAmountStolenSold` at `+0x6ec`).
pub fn fn_005cdf20(e: &mut Engine, this: Ptr, amount: i32) {
    let address = this.addr() + 0x6dc;
    let sum = e.mem.i32(address).wrapping_add(amount);
    e.mem.set_u32(address, sum as u32);
}

// Translated from 005cdf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPCExpelledFunction` (Xbox PDB): parses one faction and calls
/// `GetPCExpelledConditionFunction(thisObj, faction, 0, result)`. Succeeds
/// whenever the arguments parse.
pub fn script_get_pc_expelled_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([faction]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, GET_PC_EXPELLED_CONDITION, a, faction);
    true
}

// Translated from 005cdfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetPCExpelledFunction` (Xbox PDB): parses a faction and a flag
/// and sets or clears the faction's expelled flag ([`fn_005ce040`]). When
/// setting it, the owner of the player's parent cell is looked up
/// (`TESObjectCELL::GetOwner`, result unused).
pub fn script_set_pc_expelled_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([faction, flag]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if flag != 0 {
        fn_005ce040(e, Ptr::new(faction), 1);
        let player = e.global::<u32>(PLAYER);
        let cell = e.call(GET_PARENT_CELL, &args![player]).u32();
        if cell != 0 {
            e.call(CELL_GET_OWNER, &args![cell]);
        }
    } else {
        fn_005ce040(e, Ptr::new(faction), 0);
    }
    true
}

// Translated from 005ce040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears the faction flag `0x08` (the expelled flag).
pub fn fn_005ce040(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(
        FACTION_SET_FLAG_BITS,
        &args![this, FACTION_FLAG_EXPELLED, flag as u32],
    );
}

// Translated from 005ce060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPCFactionMurderFunction` (Xbox PDB): parses one faction and
/// calls `GetPCFactionMurderConditionFunction(thisObj, faction, 0, result)`.
pub fn script_get_pc_faction_murder_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([faction]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, GET_PC_FACTION_MURDER_CONDITION, a, faction);
    true
}

// Translated from 005ce0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a faction and a flag and sets or clears the faction flag `0x40`
/// (`0047ebb0`).
pub fn fn_005ce0c0(e: &mut Engine, a: ScriptArgs) -> bool {
    set_faction_flag(e, a, FACTION_SET_MURDER_FLAG)
}

// Translated from 005ce130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPlayerEnemyofFactionFunction` (Xbox PDB): parses one faction
/// and calls `GetPlayerEnemyofFactionConditionFunction(thisObj, faction, 0,
/// result)`.
pub fn script_get_player_enemyof_faction_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([faction]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, GET_PLAYER_ENEMY_OF_FACTION_CONDITION, a, faction);
    true
}

// Translated from 005ce190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a faction and a flag and sets or clears a faction flag
/// (`0047eb90`).
pub fn fn_005ce190(e: &mut Engine, a: ScriptArgs) -> bool {
    set_faction_flag(e, a, FACTION_SET_ENEMY_FLAG)
}

// Translated from 005ce200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPCFactionAttackFunction` (Xbox PDB): parses one faction and
/// calls `GetPCFactionAttackConditionFunction(thisObj, faction, 0, result)`.
pub fn script_get_pc_faction_attack_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([faction]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, GET_PC_FACTION_ATTACK_CONDITION, a, faction);
    true
}

// Translated from 005ce260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a faction and a flag and sets or clears a faction flag
/// (`0047ebd0`).
pub fn fn_005ce260(e: &mut Engine, a: ScriptArgs) -> bool {
    set_faction_flag(e, a, FACTION_SET_ATTACK_FLAG)
}

// Translated from 005ce2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetDestroyed` body: calls `GetDestroyedConditionFunction(thisObj, 0,
/// 0, result)`. Always succeeds.
pub fn fn_005ce2d0(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_DESTROYED_CONDITION, a, 0);
    true
}

// Translated from 005ce2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a flag; with a `thisObj`, sets or clears form flag `0x800000` on it
/// (`00484650`).
pub fn fn_005ce2f0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        e.call(FORM_SET_FLAG_800000, &args![a.this_obj, (flag != 0) as u32]);
    }
    true
}

// Translated from 005ce360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetActionRefFunction` (Xbox PDB): the result is the form id of
/// `thisObj`'s action reference (0 when it has none), echoed to the console
/// when the TLS flag is set. Always succeeds.
pub fn script_get_action_ref_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let mut id = 0;
    if !a.this_obj.is_null() && e.call(GET_ACTION_REF, &args![a.this_obj]).u32() != 0 {
        let action_ref = e.call(GET_ACTION_REF, &args![a.this_obj]).u32();
        id = store_form_id(e, action_ref, a.result);
    }
    echo_id(e, MSG_GET_ACTION_REF, id);
    true
}

// Translated from 005ce3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetSelfFunction` (Xbox PDB): the result is the form id of
/// `thisObj`, except for a reference that does not persist and whose base
/// form's type can be held by a container (then it stays 0). Echoed when the
/// TLS flag is set. Always succeeds.
pub fn script_get_self_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let mut id = 0;
    if !a.this_obj.is_null() {
        let mut report = true;
        if !e.call(GET_REF_PERSISTS, &args![a.this_obj]).bool() {
            let base = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
            let form_type = e.call(FORM_TYPE, &args![base]).u8();
            report = !e
                .call(CONTAINER_CAN_HOLD_TYPE, &args![form_type as u32])
                .bool();
        }
        if report {
            id = store_form_id(e, a.this_obj.addr(), a.result);
        }
    }
    echo_id(e, MSG_GET_SELF, id);
    true
}

// Translated from 005ce480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetCombatTargetFunction` (Xbox PDB): when `thisObj` is an actor
/// with a combat target (virtual slot `0x42c`), the result is the target's
/// form id. Echoed when the TLS flag is set. Always succeeds.
pub fn script_get_combat_target_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let actor = actor_of(e, a.this_obj.addr());
    let mut id = 0;
    if actor != 0 {
        let target = e.vcall(actor, ACTOR_GET_COMBAT_TARGET_SLOT, &args![]).u32();
        if target != 0 {
            id = store_form_id(e, target, a.result);
        }
    }
    echo_id(e, MSG_GET_COMBAT_TARGET, id);
    true
}

// Translated from 005ce520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPackageTargetFunction` (Xbox PDB): when `thisObj` is an actor
/// with a current package target, the result is the target's form id. Echoed
/// when the TLS flag is set. Always succeeds.
pub fn script_get_package_target_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let actor = actor_of(e, a.this_obj.addr());
    let mut id = 0;
    if actor != 0 {
        let target = e.call(GET_CURRENT_PACKAGE_TARGET, &args![actor]).u32();
        if target != 0 {
            id = store_form_id(e, target, a.result);
        }
    }
    echo_id(e, MSG_GET_PACKAGE_TARGET, id);
    true
}

// Translated from 005ce5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetContainerFunction` (Xbox PDB): with both `thisObj` and the
/// containing object, the result is the containing object's form id. Echoed
/// when the TLS flag is set. Always succeeds.
pub fn script_get_container_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let mut id = 0;
    if !a.this_obj.is_null() && !a.containing_obj.is_null() {
        id = store_form_id(e, a.containing_obj.addr(), a.result);
    }
    echo_id(e, MSG_GET_CONTAINER, id);
    true
}

// Translated from 005ce630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetParentRefFunction` (Xbox PDB): the result is the form id of
/// the reference `0056a9f0` finds for `thisObj` (0 when none). Echoed when
/// the TLS flag is set. Always succeeds.
pub fn script_get_parent_ref_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let mut id = 0;
    if !a.this_obj.is_null() && e.call(GET_PARENT_REF, &args![a.this_obj]).u32() != 0 {
        let parent = e.call(GET_PARENT_REF, &args![a.this_obj]).u32();
        id = store_form_id(e, parent, a.result);
    }
    echo_id(e, MSG_GET_PARENT_REF, id);
    true
}

// Translated from 005ce6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetLinkedRefFunction` (Xbox PDB): the result is the form id of
/// the reference `00569b80` finds for `thisObj` (0 when none). Echoed when
/// the TLS flag is set. Always succeeds.
pub fn script_get_linked_ref_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let mut id = 0;
    if !a.this_obj.is_null() && e.call(GET_LINKED_REF, &args![a.this_obj]).u32() != 0 {
        let linked = e.call(GET_LINKED_REF, &args![a.this_obj]).u32();
        id = store_form_id(e, linked, a.result);
    }
    echo_id(e, MSG_GET_LINKED_REF, id);
    true
}

// Translated from 005ce730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetForceRun` (Xbox PDB): the result is 1.0 when `thisObj` is an
/// actor whose force-run byte (`+0x124`) is set, else 0.0. Echoed when the
/// TLS flag is set. Always succeeds.
pub fn script_get_force_run(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let actor = actor_of(e, a.this_obj.addr());
    if actor != 0 && e.call(ACTOR_GET_FORCE_RUN, &args![actor]).bool() {
        e.mem.set_f64(a.result.addr(), 1.0);
    }
    echo_result(e, MSG_GET_FORCE_RUN, a.result);
    true
}

// Translated from 005ce7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetForceRun` (Xbox PDB): when `thisObj` is an actor, the result is
/// set to 1.0, one flag is parsed and stored as the actor's force-run byte
/// (`005bf800`); echoed when the TLS flag is set (the parsed integer is
/// passed as it is to the `%0.2f` format, as the game does). Fails when the
/// flag does not parse; succeeds without an actor.
pub fn script_set_force_run(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = actor_of(e, a.this_obj.addr());
    if actor != 0 {
        e.mem.set_f64(a.result.addr(), 1.0);
        let Some([flag]) = parse_params(e, a, [0]) else {
            return false;
        };
        e.call(ACTOR_SET_FORCE_RUN, &args![actor, (flag != 0) as u32]);
        if echo_enabled(e) {
            console_print(e, &args![MSG_SET_FORCE_RUN, flag]);
        }
    }
    true
}

// Translated from 005ce870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetForceSneak` (Xbox PDB): the result is 1.0 when `thisObj` is an
/// actor whose force-sneak byte (`+0x125`) is set, else 0.0. Echoed when the
/// TLS flag is set. Always succeeds.
pub fn script_get_force_sneak(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let actor = actor_of(e, a.this_obj.addr());
    if actor != 0 && e.call(ACTOR_GET_FORCE_SNEAK, &args![actor]).bool() {
        e.mem.set_f64(a.result.addr(), 1.0);
    }
    echo_result(e, MSG_GET_FORCE_SNEAK, a.result);
    true
}

// Translated from 005ce910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetForceSneak` (Xbox PDB): like [`script_set_force_run`] for the
/// force-sneak byte (`+0x125`, [`fn_005ce9d0`]).
pub fn script_set_force_sneak(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = actor_of(e, a.this_obj.addr());
    if actor != 0 {
        e.mem.set_f64(a.result.addr(), 1.0);
        let Some([flag]) = parse_params(e, a, [0]) else {
            return false;
        };
        fn_005ce9d0(e, Ptr::new(actor), (flag != 0) as u8);
        if echo_enabled(e) {
            console_print(e, &args![MSG_SET_FORCE_SNEAK, flag]);
        }
    }
    true
}

// Translated from 005ce9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the force-sneak byte at `this + 0x125` of an actor (PC offset).
pub fn fn_005ce9d0(e: &mut Engine, this: Ptr, flag: u8) {
    e.mem.set_u8(this.addr() + 0x125, flag);
}

// Translated from 005ce9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Raises the player's base level by one and opens the level-up menu
/// (`Interface::CreateLevelUpMenu`); nothing without a player. The new level
/// is written to the base data at `base form + 0x30` (`0047dfe0`).
pub fn fn_005ce9f0(e: &mut Engine) -> bool {
    let player = e.global::<u32>(PLAYER);
    if player != 0 {
        let level = e.call(PLAYER_GET_LEVEL, &args![player]).u16() as u32;
        let base = e.call(GET_BASE_FORM_OF_REFERENCE, &args![player]).u32();
        e.call(ACTOR_BASE_DATA_SET_LEVEL, &args![base + 0x30, level + 1]);
        e.call(CREATE_LEVEL_UP_MENU, &args![]);
    }
    true
}

// Translated from 005cea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::HasMagicEffectFunction` (Xbox PDB): parses one magic effect and
/// returns `HasMagicEffectConditionFunction(thisObj, effect, 0, result)`.
pub fn script_has_magic_effect_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([effect]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, HAS_MAGIC_EFFECT_CONDITION, a, effect)
}

// Translated from 005cea90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsSpellTargetFunction` (Xbox PDB): parses one spell and returns
/// `IsSpellTargetConditionFunction(thisObj, spell, 0, result)`.
pub fn script_is_spell_target_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([spell]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, IS_SPELL_TARGET_CONDITION, a, spell)
}

// Translated from 005ceaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetSpellUsageNumberFunction` (Xbox PDB): parses one spell and
/// returns `GetSpellUsageNumberConditionFunction(thisObj, spell, 0, result)`.
pub fn script_get_spell_usage_number_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([spell]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, GET_SPELL_USAGE_NUMBER_CONDITION, a, spell)
}

// Translated from 005ceb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetVATSMode` body: returns `GetVATSModeConditionFunction(thisObj, 0,
/// 0, result)`.
pub fn fn_005ceb50(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_VATS_MODE_CONDITION, a, 0)
}

// Translated from 005ceb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetVATSTargetHeight` body: returns
/// `GetVATSTargetHeightConditionFunction(thisObj, 0, 0, result)`.
pub fn fn_005ceb70(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_VATS_TARGET_HEIGHT_CONDITION, a, 0)
}

// ---- Helpers of the second batch -------------------------------------------------

/// Virtual slot `0x130` of a form: its name.
const FORM_NAME_SLOT: u32 = 0x130;

/// The lookup `005ceeb0` and `005cf250` share: the object a barter or caravan
/// menu opens on, found from the command's reference. A reference whose
/// virtual `IsMobileObject` is false takes the `+0x90` member of the
/// `BGSTalkingActivator` its base form casts to (zero when it is none); a
/// mobile object that is an actor (`IsActor`) is the answer itself; any other
/// mobile object whose byte at `+0x81` is set takes the `+0x90` member of the
/// base form of its member at `+0x6c`. Zero when none applies.
fn menu_target_of(e: &mut Engine, reference: u32) -> u32 {
    let mut mobile = 0;
    let mut target = 0;
    if e.vcall(reference, IS_MOBILE_OBJECT_SLOT, &args![]).bool() {
        mobile = reference;
    } else {
        let base = e.call(GET_BASE_FORM, &args![reference]).u32();
        let activator = e
            .call(
                DYNAMIC_CAST,
                &args![
                    base,
                    0u32,
                    RTTI_TES_BOUND_OBJECT,
                    RTTI_BGS_TALKING_ACTIVATOR,
                    0u32
                ],
            )
            .u32();
        target = if activator != 0 {
            e.call(TALKING_ACTIVATOR_GET_SPEAKER, &args![activator])
                .u32()
        } else {
            0
        };
    }
    if mobile != 0 {
        if e.vcall(mobile, IS_ACTOR_SLOT, &args![]).bool() {
            target = mobile;
        } else if e.call(REFERENCE_GET_BYTE_81, &args![mobile]).bool() {
            let member = e.call(REFERENCE_GET_FIELD_6C, &args![mobile]).u32();
            let base = e.call(GET_BASE_FORM, &args![member]).u32();
            target = e.call(TALKING_ACTIVATOR_GET_SPEAKER, &args![base]).u32();
        }
    }
    target
}

/// The body of the gambling-table commands: with a player, parses four words
/// (the game leaves the first local uninitialised; 0 here), then calls
/// `create(form, b, c, d or 0, 0)` when the first one is not zero, else
/// reports `invalid_form`; unparsable parameters report
/// `invalid_parameters`. Always succeeds.
fn show_table_menu(
    e: &mut Engine,
    a: ScriptArgs,
    create: u32,
    invalid_parameters: u32,
    invalid_form: u32,
    pass_fourth: bool,
) -> bool {
    if e.global::<u32>(PLAYER) != 0 {
        match parse_params(e, a, [0, 0, 0, 0]) {
            None => {
                e.call(LOG_STUB, &args![invalid_parameters]);
            }
            Some([form, second, third, fourth]) => {
                if form != 0 {
                    let fourth = if pass_fourth { fourth } else { 0 };
                    e.call(create, &args![form, second, third, fourth, 0u32]);
                } else {
                    e.call(LOG_STUB, &args![invalid_form]);
                }
            }
        }
    }
    true
}

/// `005cfcd0` and `005cfd20`: queue the menu `menu` (`QueueMenuCreate(menu,
/// reference or 0, 0, 0, 1, 0)`); the reference goes along when the type of
/// its base form is `0x17`.
fn queue_menu_for_container(e: &mut Engine, a: ScriptArgs, menu: u32) -> bool {
    let mut reference = 0;
    if !a.this_obj.is_null() {
        let base = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() == 0x17 {
            reference = a.this_obj.addr();
        }
    }
    e.call(
        QUEUE_MENU_CREATE,
        &args![menu, reference, 0u32, 0u32, 1u32, 0u32],
    );
    true
}

/// The message `EquipItem` and the unequip command show for the player: the
/// full name of the item followed by the text of a string global
/// (`"%s %s."`), with the vault boy icon; then the player's 3D is updated,
/// its process told `(1, 0, 0)` and the open menus refreshed.
fn show_item_message(e: &mut Engine, item: u32, text_global: u32) {
    e.with_stack(8, |e, text| {
        e.call(STRING_CONSTRUCT, &args![text]);
        let suffix = e.call(BS_STRING_TEXT, &args![text_global]).u32();
        let name = e.call(GET_FULL_NAME, &args![item]).u32();
        e.call(
            STRING_FORMAT,
            &args![text, ITEM_MESSAGE_FORMAT, name, suffix],
        );
        let shown = e.call(POINTER_GET, &args![text]).u32();
        let duration: f32 = e.global(MESSAGE_DURATION);
        e.call(
            SHOW_MESSAGE,
            &args![shown, 0u32, MESSAGE_ICON, 0u32, duration, 0u32],
        );
        let player = e.global::<u32>(PLAYER);
        e.call(PLAYER_UPDATE_3D, &args![player]);
        let process = e.call(GET_PROCESS, &args![player]).u32();
        e.vcall(process, PROCESS_REFRESH_SLOT, &args![1u32, 0u32, 0u32]);
        e.call(REFRESH_MENUS, &args![]);
        e.call(STRING_DESTRUCT, &args![text]);
    });
}

/// Allocates an `ExtraDataList` (`operator new` of 0x20 bytes and its
/// constructor; null when the allocation fails) and applies
/// `SetCanNotWear(worn_flag)` to it. Returns the list.
fn new_can_not_wear_list(e: &mut Engine, worn_flag: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
    let list = if block != 0 {
        e.call(EXTRA_LIST_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(EXTRA_LIST_SET_CAN_NOT_WEAR, &args![list, worn_flag]);
    list
}

/// Appends `list` (through a stack cell holding it) to the list head the
/// inventory entry's first member leads to (`POINTER_GET(entry)`).
fn append_to_entry(e: &mut Engine, entry: u32, list: u32) {
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), list);
        let head = e.call(POINTER_GET, &args![entry]).u32();
        e.call(LIST_APPEND, &args![head, cell]);
    });
}

// ---- Translated functions, second batch ------------------------------------------

// Translated from 005ceb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDefaultOpenFunction` (Xbox PDB): the condition function
/// (`thisObj, 0, 0, result`) decides; when it fails the command fails, else
/// the console echoes `"GetDefaultOpen >> %0.2f"` with the result.
pub fn script_get_default_open_function(e: &mut Engine, a: ScriptArgs) -> bool {
    if !call_condition(e, GET_DEFAULT_OPEN_CONDITION, a, 0) {
        return false;
    }
    echo_result(e, MSG_GET_DEFAULT_OPEN, a.result);
    true
}

// Translated from 005cebf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetDefaultOpenFunction` (Xbox PDB): parses one integer. For a
/// reference, sets (non-zero) or clears (zero) the action bit 8, drops its
/// saved animation and last finished sequence; when the bit changed, the
/// change `0x400000` is removed (`RemoveChange`, virtual slot `0x4c`) when
/// `BGSSaveLoadGame::GetChange` answers true for the reference, else added
/// (`AddChange`, slot `0x48`); finally `SetOpenState(reference, new bit, 1)`. The console
/// echoes `"SetDefaultOpen >> %0.2f"` with the integer word (the game passes
/// an integer for a `%f`; it is passed as the word here).
pub fn script_set_default_open_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([open]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        let reference = a.this_obj.addr();
        let before = e
            .call(
                REFERENCE_TEST_ACTION_BITS,
                &args![reference, ACTION_BIT_DEFAULT_OPEN],
            )
            .u8();
        if open != 0 {
            e.call(
                REFERENCE_SET_ACTION_BITS,
                &args![reference, ACTION_BIT_DEFAULT_OPEN],
            );
        } else {
            e.call(
                REFERENCE_CLEAR_ACTION_BITS,
                &args![reference, ACTION_BIT_DEFAULT_OPEN],
            );
        }
        let after = e
            .call(
                REFERENCE_TEST_ACTION_BITS,
                &args![reference, ACTION_BIT_DEFAULT_OPEN],
            )
            .u8();
        let list = e.call(EXTRA_DATA_LIST, &args![reference]).u32();
        e.call(REMOVE_SAVED_ANIMATION, &args![list]);
        let list = e.call(EXTRA_DATA_LIST, &args![reference]).u32();
        e.call(REMOVE_LAST_FINISHED_SEQUENCE, &args![list]);
        if before != after {
            let flags = e.with_stack(4, |e, cell| {
                e.call(CHANGE_FLAGS_CONSTRUCT, &args![cell, CHANGE_FLAG_OPEN_STATE]);
                e.mem.u32(cell.addr())
            });
            let save_load = e.global::<u32>(SAVE_LOAD_GAME);
            let known = e
                .call(SAVE_LOAD_GET_CHANGE, &args![save_load, reference, flags])
                .bool();
            let slot = if known {
                REMOVE_CHANGE_SLOT
            } else {
                ADD_CHANGE_SLOT
            };
            e.vcall(reference, slot, &args![CHANGE_FLAG_OPEN_STATE]);
        }
        e.call(SET_OPEN_STATE, &args![reference, after as u32, 1u32]);
    }
    if echo_enabled(e) {
        console_print(e, &args![MSG_SET_DEFAULT_OPEN, open]);
    }
    true
}

// Translated from 005ced30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetOpenStateFunction` (Xbox PDB): parses one integer. For a
/// reference: non-zero opens (activates the reference when its open state is
/// above 2) and records the finished sequence `"Open"`; zero closes
/// (activates when the state is below 3) and records `"Close"`; either way
/// the saved animation is removed.
pub fn script_set_open_state_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([open]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        let reference = a.this_obj.addr();
        let state = e.call(GET_OPEN_STATE, &args![reference]).i32();
        let (activate, sequence) = if open != 0 {
            (state > 2, SEQUENCE_OPEN)
        } else {
            (state < 3, SEQUENCE_CLOSE)
        };
        if activate {
            e.call(
                REFERENCE_ACTIVATE,
                &args![reference, 0u32, 0u32, 0u32, 1u32],
            );
        }
        e.call(SET_LAST_FINISHED_SEQUENCE, &args![reference, sequence]);
        let list = e.call(EXTRA_DATA_LIST, &args![reference]).u32();
        e.call(REMOVE_SAVED_ANIMATION, &args![list]);
    }
    true
}

// Translated from 005cedf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a player, opens the race/sex menu in mode 0 when the TLS echo flag is
/// set and in mode 1 when it is clear. Always succeeds.
pub fn fn_005cedf0(e: &mut Engine) -> bool {
    if e.global::<u32>(PLAYER) != 0 {
        let mode = if echo_enabled(e) { 0u32 } else { 1u32 };
        e.call(CREATE_RACE_SEX_MENU, &args![mode]);
    }
    true
}

// Translated from 005cee30 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a player, opens the race/sex menu in mode 2. Always succeeds.
pub fn fn_005cee30(e: &mut Engine) -> bool {
    if e.global::<u32>(PLAYER) != 0 {
        e.call(CREATE_RACE_SEX_MENU, &args![2u32]);
    }
    true
}

// Translated from 005cee50 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a player, opens the race/sex menu in mode 3. Always succeeds.
pub fn fn_005cee50(e: &mut Engine) -> bool {
    if e.global::<u32>(PLAYER) != 0 {
        e.call(CREATE_RACE_SEX_MENU, &args![3u32]);
    }
    true
}

// Translated from 005cee70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Without a player: true. Otherwise the result of `005da540`, which gets all
/// eight command words (it closes the console and queues menu 9 for the
/// player).
pub fn fn_005cee70(e: &mut Engine, a: ScriptArgs) -> bool {
    if e.global::<u32>(PLAYER) == 0 {
        return true;
    }
    e.call(CLOSE_CONSOLE_AND_QUEUE_MENU, &args![a]).bool()
}

// Translated from 005ceeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer (the result of the parse is not looked at: the local
/// keeps 0 when it fails), and for a reference that is not the player finds
/// its menu target ([`menu_target_of`]) and opens the barter menu on it with
/// the integer (`00704f80`). Always succeeds.
pub fn fn_005ceeb0(e: &mut Engine, a: ScriptArgs) -> bool {
    let amount = e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), 0);
        parse(e, a, &[cell.addr()]);
        e.mem.u32(cell.addr())
    });
    let reference = a.this_obj.addr();
    if reference != 0 && reference != e.global::<u32>(PLAYER) {
        let target = menu_target_of(e, reference);
        if target != 0 {
            e.call(BARTER_MENU_CREATE, &args![target, amount]);
        }
    }
    true
}

// Translated from 005cefe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowSPECIALBookMenu` (Xbox PDB): with a player, parses one word
/// and opens `SPECIALBookMenu::Create_ov2(word)`, or `Create()` when the
/// parameters do not parse. Always succeeds.
pub fn script_show_special_book_menu(e: &mut Engine, a: ScriptArgs) -> bool {
    if e.global::<u32>(PLAYER) != 0 {
        match parse_params(e, a, [0]) {
            None => {
                e.call(SPECIAL_BOOK_MENU_CREATE, &args![]);
            }
            Some([book]) => {
                e.call(SPECIAL_BOOK_MENU_CREATE_OV2, &args![book]);
            }
        }
    }
    true
}

// Translated from 005cf040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowSlotMachineMenu` (Xbox PDB): see [`show_table_menu`]; the menu
/// gets the four parsed words and 0.
pub fn script_show_slot_machine_menu(e: &mut Engine, a: ScriptArgs) -> bool {
    show_table_menu(
        e,
        a,
        SLOT_MACHINE_MENU_CREATE,
        MSG_SLOT_MACHINE_PARAMETERS,
        MSG_SLOT_MACHINE_FORM,
        true,
    )
}

// Translated from 005cf0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowBlackJackMenu` (Xbox PDB): see [`show_table_menu`]; the menu
/// gets three parsed words and two zeros (the fourth parsed word is not
/// passed on).
pub fn script_show_black_jack_menu(e: &mut Engine, a: ScriptArgs) -> bool {
    show_table_menu(
        e,
        a,
        BLACK_JACK_MENU_CREATE,
        MSG_BLACK_JACK_PARAMETERS,
        MSG_BLACK_JACK_FORM,
        false,
    )
}

// Translated from 005cf1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowRouletteMenu` (Xbox PDB): see [`show_table_menu`]; the menu
/// gets the four parsed words and 0.
pub fn script_show_roulette_menu(e: &mut Engine, a: ScriptArgs) -> bool {
    show_table_menu(
        e,
        a,
        ROULETTE_MENU_CREATE,
        MSG_ROULETTE_PARAMETERS,
        MSG_ROULETTE_FORM,
        true,
    )
}

// Translated from 005cf250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowCaravanMenu` (Xbox PDB): with a player, parses three words
/// (the third is a `float`, 0.0 by default); unparsable parameters are
/// reported. For a reference that is not the player and a non-zero first
/// word, opens `CaravanMenu::Create(target, first, second, float, 0)` on the
/// menu target of the reference ([`menu_target_of`]; zero when none). Always
/// succeeds.
pub fn script_show_caravan_menu(e: &mut Engine, a: ScriptArgs) -> bool {
    if e.global::<u32>(PLAYER) == 0 {
        return true;
    }
    let Some([first, second, third]) = parse_params(e, a, [0, 0, 0]) else {
        e.call(LOG_STUB, &args![MSG_CARAVAN_PARAMETERS]);
        return true;
    };
    let reference = a.this_obj.addr();
    if reference != 0 && reference != e.global::<u32>(PLAYER) && first != 0 {
        let target = menu_target_of(e, reference);
        e.call(
            CARAVAN_MENU_CREATE,
            &args![target, first, second, third, 0u32],
        );
    }
    true
}

// Translated from 005cf3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::AddCardToPlayer` (Xbox PDB): takes no parameter words (the parse
/// is only checked, and failing is reported). For a reference with a base
/// form, the base form is cast to `TESCaravanCard`; a card is added to the
/// player (`00969bc0`), anything else is reported with the reference's name
/// (virtual slot `0x130`). Always succeeds.
pub fn script_add_card_to_player(e: &mut Engine, a: ScriptArgs) -> bool {
    if !parse(e, a, &[]) {
        e.call(LOG_STUB, &args![MSG_ADD_CARD_PARAMETER]);
    }
    if !a.this_obj.is_null() {
        let reference = a.this_obj.addr();
        if e.call(GET_BASE_FORM, &args![reference]).u32() != 0 {
            let base = e.call(GET_BASE_FORM, &args![reference]).u32();
            let card = e
                .call(
                    DYNAMIC_CAST,
                    &args![
                        base,
                        0u32,
                        RTTI_TES_BOUND_OBJECT,
                        RTTI_TES_CARAVAN_CARD,
                        0u32
                    ],
                )
                .u32();
            if card != 0 {
                let player = e.global::<u32>(PLAYER);
                e.call(PLAYER_ADD_CARD, &args![player, card]);
            } else {
                let name = e.vcall(reference, FORM_NAME_SLOT, &args![]).u32();
                e.call(LOG_STUB, &args![MSG_NOT_CARAVAN_CARD, name]);
            }
        }
    }
    true
}

// Translated from 005cf490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPlayerInRegion` (Xbox PDB): parses one region. The result
/// double is 0.0, then 1.0 when the form id of the region is the form id of
/// one of the player's regions (the list at `player + 0x764`, walked until a
/// node without item; nodes are chained through the cell at `+4` of the next
/// node, minus 4). Echoes `"IsPlayerInRegion >> %0.f"`; when the parse fails
/// the failure is reported and the echo shows the result as it was. Always
/// succeeds.
pub fn script_is_player_in_region(e: &mut Engine, a: ScriptArgs) -> bool {
    let region = e.with_stack(4, |e, cell| {
        // The game leaves this local uninitialised.
        e.mem.set_u32(cell.addr(), 0);
        let ok = parse(e, a, &[cell.addr()]);
        ok.then(|| e.mem.u32(cell.addr()))
    });
    let Some(region) = region else {
        e.call(LOG_STUB, &args![MSG_IS_PLAYER_IN_REGION_PARAMETER]);
        echo_result(e, MSG_IS_PLAYER_IN_REGION, a.result);
        return true;
    };
    e.mem.set_f64(a.result.addr(), 0.0);
    if region != 0 {
        let player = e.global::<u32>(PLAYER);
        let mut node = e.call(PLAYER_REGION_LIST, &args![player]).u32();
        while node != 0 {
            let cell = e.call(LIST_ITEM_CELL, &args![node + 4]).u32();
            if e.mem.u32(cell) == 0 {
                break;
            }
            let cell = e.call(LIST_ITEM_CELL, &args![node + 4]).u32();
            let item = e.mem.u32(cell);
            let item_id = e.call(GET_FORM_ID, &args![item]).u32();
            let region_id = e.call(GET_FORM_ID, &args![region]).u32();
            if item_id == region_id {
                e.mem.set_f64(a.result.addr(), 1.0);
                echo_result(e, MSG_IS_PLAYER_IN_REGION, a.result);
                return true;
            }
            let next = e.call(LIST_NEXT_CELL, &args![node + 4]).u32();
            node = if next != 0 { next - 4 } else { 0 };
        }
    }
    echo_result(e, MSG_IS_PLAYER_IN_REGION, a.result);
    true
}

// Translated from 005cf5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowLoveTesterMenu` (Xbox PDB): with a player, parses one word
/// and opens `LoveTesterMenu::Create_ov2(word)`, or `Create()` when the
/// parameters do not parse. Always succeeds.
pub fn script_show_love_tester_menu(e: &mut Engine, a: ScriptArgs) -> bool {
    if e.global::<u32>(PLAYER) != 0 {
        match parse_params(e, a, [0]) {
            None => {
                e.call(LOVE_TESTER_MENU_CREATE, &args![]);
            }
            Some([word]) => {
                e.call(LOVE_TESTER_MENU_CREATE_OV2, &args![word]);
            }
        }
    }
    true
}

// Translated from 005cf640 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a player, opens `TraitSelectMenu::Create`. Always succeeds.
pub fn fn_005cf640(e: &mut Engine) -> bool {
    if e.global::<u32>(PLAYER) != 0 {
        e.call(TRAIT_SELECT_MENU_CREATE, &args![]);
    }
    true
}

// Translated from 005cf660 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a player, opens `TraitMenu::Create`. Always succeeds.
pub fn fn_005cf660(e: &mut Engine) -> bool {
    if e.global::<u32>(PLAYER) != 0 {
        e.call(TRAIT_MENU_CREATE, &args![]);
    }
    true
}

/// The stack frame of `005cf680`, as offsets from its start: the reference
/// (4 bytes), the personality text (0x204 bytes), a zeroed word and the mood
/// text (0x204 bytes).
const FACE_FRAME_SIZE: u32 = 0x410;
const FACE_PERSONALITY: u32 = 0x04;
const FACE_ZERO_WORD: u32 = 0x208;
const FACE_MOOD: u32 = 0x20c;

// Translated from 005cf680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a reference and two texts. For a reference whose virtual slot
/// `0x1d0` gives a 3D object, finds the reference's `ExtraSecuritronFace`
/// (extra data type `0x8f`; a new one, 0x1c bytes, is added when it has
/// none), sets its personality and mood from the texts, applies the face to
/// the 3D object and adds the changes `0x10000000` and `0xa4061840` to the
/// reference (`AddChange`, virtual slot `0x48`). The compiler's exception frame and stack cookie are not
/// translated.
pub fn fn_005cf680(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(FACE_FRAME_SIZE, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0);
        e.mem.set_u32(base + FACE_ZERO_WORD, 0);
        if !parse(e, a, &[base, base + FACE_PERSONALITY, base + FACE_MOOD]) {
            return false;
        }
        let reference = e.mem.u32(base);
        if reference == 0 {
            return true;
        }
        let root = e.vcall(reference, REFERENCE_GET_3D_SLOT, &args![]).u32();
        if root == 0 {
            return true;
        }
        let mut extra = e
            .call(
                REFERENCE_GET_EXTRA_DATA,
                &args![reference, EXTRA_TYPE_SECURITRON_FACE],
            )
            .u32();
        if extra == 0 {
            let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
            extra = if block != 0 {
                e.call(SECURITRON_FACE_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            let list = e.call(EXTRA_DATA_LIST, &args![reference]).u32();
            e.call(ADD_EXTRA, &args![list, extra]);
        }
        if extra != 0 {
            e.call(
                SECURITRON_FACE_SET_PERSONALITY,
                &args![extra, base + FACE_PERSONALITY],
            );
            e.call(SECURITRON_FACE_SET_MOOD, &args![extra, base + FACE_MOOD]);
            e.call(SECURITRON_FACE_APPLY_FACE, &args![extra, root]);
            e.vcall(
                reference,
                ADD_CHANGE_SLOT,
                &args![CHANGE_FLAG_SECURITRON_FACE_A],
            );
            e.vcall(
                reference,
                ADD_CHANGE_SLOT,
                &args![CHANGE_FLAG_SECURITRON_FACE_B],
            );
        }
        true
    })
}

/// The stack frame of `005cf860` as offsets from its start: the reference
/// (4), the two texts (0x200 bytes each), the two `BSStringT` (8 bytes each),
/// the `NiFixedString` handle (4), the texture smart pointer (4) and the two
/// path buffers (0x104 bytes each).
const SWAP_FRAME_SIZE: u32 = 0x630;
const SWAP_NODE_NAME: u32 = 0x004;
const SWAP_TEXTURE_NAME: u32 = 0x204;
const SWAP_NODE_STRING: u32 = 0x404;
const SWAP_TEXTURE_STRING: u32 = 0x40c;
const SWAP_HANDLE: u32 = 0x414;
const SWAP_TEXTURE: u32 = 0x418;
const SWAP_FORMATTED_PATH: u32 = 0x41c;
const SWAP_LANGUAGE_PATH: u32 = 0x520;
/// Size of the path buffers.
const PATH_SIZE: u32 = 0x104;

/// The reference's 3D property of type 3 cast to `NiShadeProperty` (null when
/// there is none or it is of another class).
fn shade_property_of(e: &mut Engine, root: u32) -> u32 {
    let property = e
        .call(GET_PROPERTY, &args![root, PROPERTY_TYPE_ASKED])
        .u32();
    e.call(NI_DYNAMIC_CAST, &args![RTTI_NI_SHADE_PROPERTY, property])
        .u32()
}

// Translated from 005cf860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SwapTextureOnRef` (Xbox PDB): parses a reference and two texts (a
/// node name and a texture name) and fails when a text is empty. For a
/// reference with a 3D object (virtual slot `0x1d0`) that has a node of the
/// first name (slot `0x9c`): loads the image `Textures\<second name>.dds`
/// (through `SwapPlatformLanguageTexturePath` and `TES::CreateTextureImage`),
/// and when the node's shade property (type 3) is a shader property whose
/// type `iShaderPropertyType` is from 8 to 12, hands the texture to its
/// virtual slot `0xf0` as `(0, 0, texture)`; then hands the texture to the
/// second property class (type `0x00438220()`, RTTI `011fa05c`) through
/// `00438230`. Succeeds when the parameters parse and neither text is empty.
/// The compiler's exception frame and stack cookie are not translated.
pub fn script_swap_texture_on_ref(e: &mut Engine, a: ScriptArgs) -> bool {
    let frame = e.mem.alloc(SWAP_FRAME_SIZE);
    let result = swap_texture_body(e, a, frame);
    e.mem.free(frame);
    result
}

fn swap_texture_body(e: &mut Engine, a: ScriptArgs, frame: u32) -> bool {
    let node_string = frame + SWAP_NODE_STRING;
    let texture_string = frame + SWAP_TEXTURE_STRING;
    e.mem.set_u32(frame, 0);
    e.call(STRING_CONSTRUCT, &args![node_string]);
    e.call(STRING_CONSTRUCT, &args![texture_string]);
    let succeeded = swap_texture_with_strings(e, a, frame);
    e.call(STRING_DESTRUCT, &args![texture_string]);
    e.call(STRING_DESTRUCT, &args![node_string]);
    succeeded
}

fn swap_texture_with_strings(e: &mut Engine, a: ScriptArgs, frame: u32) -> bool {
    let node_string = frame + SWAP_NODE_STRING;
    let texture_string = frame + SWAP_TEXTURE_STRING;
    let node_name = frame + SWAP_NODE_NAME;
    let texture_name = frame + SWAP_TEXTURE_NAME;
    if !parse(e, a, &[frame, node_name, texture_name]) {
        return false;
    }
    e.call(STRING_ASSIGN, &args![node_string, node_name]);
    e.call(STRING_ASSIGN, &args![texture_string, texture_name]);
    if e.call(STRING_LENGTH, &args![node_string]).u32() == 0
        || e.call(STRING_LENGTH, &args![texture_string]).u32() == 0
    {
        return false;
    }
    let reference = e.mem.u32(frame);
    if reference == 0 {
        return true;
    }
    let root = e.vcall(reference, REFERENCE_GET_3D_SLOT, &args![]).u32();
    if root == 0 {
        return true;
    }
    let handle = frame + SWAP_HANDLE;
    e.call(FIXED_STRING_CONSTRUCT, &args![handle, node_name]);
    let node = e.vcall(root, FIND_NODE_BY_NAME_SLOT, &args![handle]).u32();
    e.call(FIXED_STRING_DESTRUCT, &args![handle]);
    if node == 0 {
        return true;
    }
    let texture = frame + SWAP_TEXTURE;
    e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
    let name = e.call(POINTER_GET, &args![texture_string]).u32();
    let formatted = frame + SWAP_FORMATTED_PATH;
    e.call(
        FORMAT_INTO_BUFFER,
        &args![formatted, PATH_SIZE, TEXTURE_PATH_FORMAT, name],
    );
    let language_path = frame + SWAP_LANGUAGE_PATH;
    e.call(
        SWAP_PLATFORM_LANGUAGE_TEXTURE_PATH,
        &args![formatted, language_path, PATH_SIZE],
    );
    let tes = e.global::<u32>(TES_SINGLETON);
    e.call(
        CREATE_TEXTURE_IMAGE,
        &args![tes, language_path, texture, 0u32, 0u32],
    );
    // The shader type must lie in 8..=12 (signed compares); each test asks
    // the 3D object for its property again, as the game does.
    let mut in_range = false;
    let shade = shade_property_of(e, node);
    if shade != 0 {
        let shade = shade_property_of(e, node);
        if e.call(GET_SHADER_PROPERTY_TYPE, &args![shade]).i32() >= 8 {
            let shade = shade_property_of(e, node);
            in_range = e.call(GET_SHADER_PROPERTY_TYPE, &args![shade]).i32() <= 12;
        }
    }
    let shader = if in_range {
        shade_property_of(e, node)
    } else {
        0
    };
    if shader != 0 {
        let image = e.call(POINTER_GET, &args![texture]).u32();
        e.vcall(
            shader,
            SHADE_PROPERTY_SET_TEXTURE_SLOT,
            &args![0u32, 0u32, image],
        );
    }
    let property_type = e.call(PROPERTY_TYPE_SHADE, &args![]).u32();
    let property = e.call(GET_PROPERTY, &args![node, property_type]).u32();
    let textured = e
        .call(NI_DYNAMIC_CAST, &args![RTTI_TEXTURE_SET_PROPERTY, property])
        .u32();
    if textured != 0 {
        let image = e.call(POINTER_GET, &args![texture]).u32();
        e.call(PROPERTY_SET_TEXTURE, &args![textured, image]);
    }
    e.call(NI_POINTER_DESTRUCT, &args![texture]);
    true
}

// Translated from 005cfc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a player: parses one integer and stores `integer > 0` (signed) with
/// [`fn_005cfcb0`]. Always succeeds.
pub fn fn_005cfc50(e: &mut Engine, a: ScriptArgs) -> bool {
    if e.global::<u32>(PLAYER) != 0 {
        if let Some([value]) = parse_params(e, a, [0]) {
            fn_005cfcb0(e, (value as i32 > 0) as u8);
        }
    }
    true
}

// Translated from 005cfcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `+0x205` of the `HUDMainMenu` singleton
/// (`bForceObjectiveReminders`, Xbox PDB), when the menu exists.
pub fn fn_005cfcb0(e: &mut Engine, value: u8) {
    let menu = e.global::<u32>(HUD_MAIN_MENU);
    if menu != 0 {
        e.mem.set_u8(menu + HUD_FORCE_OBJECTIVE_REMINDERS, value);
    }
}

// Translated from 005cfcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues menu 2 (see [`queue_menu_for_container`]). Always succeeds.
pub fn fn_005cfcd0(e: &mut Engine, a: ScriptArgs) -> bool {
    queue_menu_for_container(e, a, 2)
}

// Translated from 005cfd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues menu 3 (see [`queue_menu_for_container`]). Always succeeds.
pub fn fn_005cfd20(e: &mut Engine, a: ScriptArgs) -> bool {
    queue_menu_for_container(e, a, 3)
}

// Translated from 005cfd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `005a2a50(thisObj, 0, 0, result)` (a condition function of
/// `tesconditionfunctions.cpp`).
pub fn fn_005cfd70(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, UNKNOWN_CONDITION_005A2A50, a, 0)
}

// Translated from 005cfd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetSandman` body: returns `GetSandmanConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005cfd90(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_SANDMAN_CONDITION, a, 0)
}

// Translated from 005cfdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetCannibal` body: returns `GetCannibalConditionFunction(thisObj, 0,
/// 0, result)`.
pub fn fn_005cfdb0(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_CANNIBAL_CONDITION, a, 0)
}

// Translated from 005cfdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `HasBeenEaten` body: returns `HasBeenEatenConditionFunction(thisObj,
/// 0, 0, result)`.
pub fn fn_005cfdd0(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, HAS_BEEN_EATEN_CONDITION, a, 0)
}

// Translated from 005cfdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one actor. When `thisObj` and that actor are both actors
/// (`IsActor`), reads three values from `thisObj`'s process (virtual slots
/// `0x4d0`, `0x4d4`, `0x4c8`, in that order) and hands them with `thisObj` to
/// virtual slot `0x374` of the parsed actor as `(thisObj, v4c8, v4d4,
/// v4d0)`. Fails only when the parameters do not parse.
pub fn fn_005cfdf0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([other]) = parse_params(e, a, [0]) else {
        return false;
    };
    let actor = a.this_obj.addr();
    if e.vcall(actor, IS_ACTOR_SLOT, &args![]).bool()
        && other != 0
        && e.vcall(other, IS_ACTOR_SLOT, &args![]).bool()
    {
        let first = e.call(GET_PROCESS, &args![actor]).u32();
        let second = e.call(GET_PROCESS, &args![actor]).u32();
        let third = e.call(GET_PROCESS, &args![actor]).u32();
        let v4d0 = e.vcall(first, PROCESS_SLOT_4D0, &args![]).u32();
        let v4d4 = e.vcall(second, PROCESS_SLOT_4D4, &args![]).u32();
        let v4c8 = e.vcall(third, PROCESS_SLOT_4C8, &args![]).u32();
        e.vcall(other, ACTOR_SLOT_374, &args![actor, v4c8, v4d4, v4d0]);
    }
    true
}

// Translated from 005cfef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one actor. When `thisObj` and that actor are both actors
/// (`IsActor`), calls virtual slot `0x378` of the parsed actor with
/// `thisObj`. Fails only when the parameters do not parse.
pub fn fn_005cfef0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([other]) = parse_params(e, a, [0]) else {
        return false;
    };
    let actor = a.this_obj.addr();
    if e.vcall(actor, IS_ACTOR_SLOT, &args![]).bool()
        && other != 0
        && e.vcall(other, IS_ACTOR_SLOT, &args![]).bool()
    {
        e.vcall(other, ACTOR_SLOT_378, &args![actor]);
    }
    true
}

// Translated from 005cff90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetGhost` body: returns `GetGhostConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005cff90(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_GHOST_CONDITION, a, 0)
}

// Translated from 005cffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetGhostFunction` (Xbox PDB): parses one integer; when `thisObj`
/// casts to an actor, `Actor::SetGhost(integer > 0)` (signed). The console
/// echoes `"SetGhost >> %d"` with the integer.
pub fn script_set_ghost_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    let actor = actor_of(e, a.this_obj.addr());
    if actor != 0 {
        e.call(ACTOR_SET_GHOST, &args![actor, (value as i32 > 0) as u32]);
    }
    if echo_enabled(e) {
        console_print(e, &args![MSG_SET_GHOST, value]);
    }
    true
}

// Translated from 005d0060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::EquipItemFunction` (Xbox PDB): parses an item, a count and a
/// flag. With a reference: the "worn" flag is `count > 0` (signed); without
/// an item the script is reported (the name is virtual slot `0x130` of the
/// running script). For a reference that casts to an actor: when the actor's
/// inventory entry for the item has an extra data list that is worn
/// (`GetWorn(0)`), that list gets `SetCanNotWear(worn flag)` and nothing is
/// queued; otherwise `Actor::QueueEquipObject(item, 1, 0, 1, worn flag, 1)`
/// is queued and, for the player with a zero third flag, the equip message
/// ([`show_item_message`]) is shown. The inventory entry the lookup returned
/// is destroyed. The compiler's exception frame and stack cookie are not
/// translated.
pub fn script_equip_item_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([item, count, silent]) = parse_params(e, a, [0, 0, 0]) else {
        return false;
    };
    let reference = a.this_obj.addr();
    if reference == 0 {
        return true;
    }
    let actor = actor_of(e, reference);
    let worn_flag = (count as i32 > 0) as u32;
    if item == 0 {
        let name = e.vcall(a.script_obj.addr(), FORM_NAME_SLOT, &args![]).u32();
        e.call(LOG_STUB, &args![MSG_EQUIP_ITEM_FAILED, name]);
        return true;
    }
    if actor == 0 {
        return true;
    }
    let list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
    let changes = e.call(GET_CONTAINER_CHANGES, &args![list]).u32();
    let mut not_worn = true;
    if changes != 0 {
        let entry = e
            .call(GET_INVENTORY_ITEM, &args![changes, item, 0u32])
            .u32();
        if entry != 0 {
            let mut node = e.call(POINTER_GET, &args![entry]).u32();
            while node != 0 {
                let cell = e.call(LIST_ITEM_CELL, &args![node]).u32();
                if e.mem.u32(cell) == 0 {
                    break;
                }
                let cell = e.call(LIST_ITEM_CELL, &args![node]).u32();
                let extra_list = e.mem.u32(cell);
                if extra_list != 0 && e.call(EXTRA_LIST_GET_WORN, &args![extra_list, 0u32]).bool() {
                    e.call(EXTRA_LIST_SET_CAN_NOT_WEAR, &args![extra_list, worn_flag]);
                    not_worn = false;
                    break;
                }
                node = e.call(LIST_NEXT_CELL, &args![node]).u32();
            }
            e.call(INVENTORY_ENTRY_DELETE, &args![entry, 1u32]);
        }
    }
    if not_worn {
        e.call(
            ACTOR_QUEUE_EQUIP_OBJECT,
            &args![actor, item, 1u32, 0u32, 1u32, worn_flag, 1u32],
        );
        if actor == e.global::<u32>(PLAYER) && silent == 0 {
            show_item_message(e, item, EQUIPPED_TEXT_GLOBAL);
        }
    }
    true
}

// Translated from 005d0300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The counterpart of `EquipItem` (parses an item, a count and a flag; the
/// worn flag is `count > 0`). With an item and an actor: the actor's
/// inventory changes (`GetInventoryChanges(thisObj)`) are asked which entry
/// of the item is worn (`WearingObject(item, 0)`); that entry gets
/// `SetCanNotWear(0)`, `Actor::QueueUnEquipObject(item, 1, 0, 1, worn flag,
/// 1)` is queued and, for the player with a zero third flag, the message is
/// shown ([`show_item_message`], with the other text global). Without an
/// item the inventory entry `GetInventoryItem(0, 0)` is found, its first
/// extra data list (or a new one, in a new list head when the entry had
/// none) gets `SetCanNotWear(worn flag)`, and the entry is destroyed. The
/// compiler's exception frame and stack cookie are not translated.
pub fn fn_005d0300(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([item, count, silent]) = parse_params(e, a, [0, 0, 0]) else {
        return false;
    };
    let reference = a.this_obj.addr();
    if reference == 0 {
        return true;
    }
    let actor = actor_of(e, reference);
    let worn_flag = (count as i32 > 0) as u32;
    if item != 0 {
        if actor == 0 {
            return true;
        }
        let changes = e.call(GET_INVENTORY_CHANGES, &args![reference]).u32();
        if changes == 0 {
            return true;
        }
        let worn = e.call(WEARING_OBJECT, &args![changes, item, 0u32]).u32();
        if worn == 0 {
            return true;
        }
        e.call(EXTRA_LIST_SET_CAN_NOT_WEAR, &args![worn, 0u32]);
        e.call(
            ACTOR_QUEUE_UNEQUIP_OBJECT,
            &args![actor, item, 1u32, 0u32, 1u32, worn_flag, 1u32],
        );
        if actor == e.global::<u32>(PLAYER) && silent == 0 {
            show_item_message(e, item, UNEQUIPPED_TEXT_GLOBAL);
        }
        return true;
    }
    let changes = e.call(GET_INVENTORY_CHANGES, &args![reference]).u32();
    if changes == 0 {
        return true;
    }
    let entry = e
        .call(GET_INVENTORY_ITEM, &args![changes, item, 0u32])
        .u32();
    if entry == 0 {
        return true;
    }
    if e.call(POINTER_GET, &args![entry]).u32() != 0 {
        let head = e.call(POINTER_GET, &args![entry]).u32();
        let first = e.call(LIST_ITEM_CELL, &args![head]).u32();
        if e.mem.u32(first) != 0 {
            let head = e.call(POINTER_GET, &args![entry]).u32();
            let cell = e.call(LIST_ITEM_CELL, &args![head]).u32();
            let existing = e.mem.u32(cell);
            e.call(EXTRA_LIST_SET_CAN_NOT_WEAR, &args![existing, worn_flag]);
        } else {
            let list = new_can_not_wear_list(e, worn_flag);
            append_to_entry(e, entry, list);
        }
    } else {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let head = if block != 0 {
            e.call(LIST_HEAD_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(entry, head);
        let list = new_can_not_wear_list(e, worn_flag);
        append_to_entry(e, entry, list);
    }
    e.call(INVENTORY_ENTRY_DELETE, &args![entry, 1u32]);
    true
}

// Translated from 005d06a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one word. For an actor reference and a non-zero word: the base form
/// is cast to `TESNPC`; the word goes to the member at `+0x130` of the NPC
/// (which records change flags `0x400`) and `TESNPC::InitValues(0)` runs.
/// Fails only when the parameters do not parse.
pub fn fn_005d06a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        let actor = actor_of(e, a.this_obj.addr());
        if actor != 0 && value != 0 {
            let base = e.call(GET_BASE_FORM, &args![actor]).u32();
            let npc = e
                .call(
                    DYNAMIC_CAST,
                    &args![base, 0u32, RTTI_TES_BOUND_OBJECT, RTTI_TES_NPC, 0u32],
                )
                .u32();
            if npc != 0 {
                e.call(NPC_SET_FIELD_130, &args![npc, value]);
                e.call(NPC_INIT_VALUES, &args![npc, 0u32]);
            }
        }
    }
    true
}

/// The two virtual calls on the actor's process that both branches of
/// `005d0760` make when the actor's process exists: slot `0x614` with
/// `0x800` and slot `0x71c` with 0 (the process is fetched again for each).
fn process_clear_state(e: &mut Engine, actor: u32) {
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    e.vcall(process, PROCESS_SLOT_614, &args![0x800u32]);
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    e.vcall(process, PROCESS_SLOT_71C, &args![0u32]);
}

// Translated from 005d0760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one flag. For an actor reference: with a non-zero flag, virtual
/// slot `0x434(0)` of the actor, then (unless its life state is 3) the
/// process calls of [`process_clear_state`] when it has a process, then life
/// state 3 is set (`008ace10(1)`), the movement ended and the process told
/// the `float` at `01012054` (slot `0x338`). With a zero flag: the process
/// calls when the life state is 3 and the actor has a process, then
/// `008ace10(0)`. Fails only when the parameters do not parse.
pub fn fn_005d0760(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag]) = parse_params(e, a, [0]) else {
        return false;
    };
    if a.this_obj.is_null() {
        return true;
    }
    let actor = actor_of(e, a.this_obj.addr());
    if actor == 0 {
        return true;
    }
    if flag != 0 {
        e.vcall(actor, ACTOR_SLOT_434, &args![0u32]);
        if !e.call(ACTOR_IS_LIFE_STATE_3, &args![actor]).bool()
            && e.call(GET_PROCESS, &args![actor]).u32() != 0
        {
            process_clear_state(e, actor);
        }
        e.call(ACTOR_SET_LIFE_STATE_3, &args![actor, 1u32]);
        e.call(ACTOR_END_MOVEMENT, &args![actor]);
        if e.call(GET_PROCESS, &args![actor]).u32() != 0 {
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            let value: f32 = e.global(FLOAT_ARGUMENT);
            e.vcall(process, PROCESS_SLOT_338, &args![value]);
        }
    } else {
        if e.call(ACTOR_IS_LIFE_STATE_3, &args![actor]).bool()
            && e.call(GET_PROCESS, &args![actor]).u32() != 0
        {
            process_clear_state(e, actor);
        }
        e.call(ACTOR_SET_LIFE_STATE_3, &args![actor, 0u32]);
    }
    true
}

// Translated from 005d0900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetUnconscious` body: returns `GetUnconsciousConditionFunction(thisObj,
/// 0, 0, result)`.
pub fn fn_005d0900(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_UNCONSCIOUS_CONDITION, a, 0)
}

// Translated from 005d0920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one flag. For an actor reference: a non-zero flag calls
/// `008ace50(1)` (life state 5) and ends the movement, zero calls
/// `008ace50(0)`. Fails only when the parameters do not parse.
pub fn fn_005d0920(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        let actor = actor_of(e, a.this_obj.addr());
        if actor != 0 {
            if flag != 0 {
                e.call(ACTOR_SET_LIFE_STATE_5, &args![actor, 1u32]);
                e.call(ACTOR_END_MOVEMENT, &args![actor]);
            } else {
                e.call(ACTOR_SET_LIFE_STATE_5, &args![actor, 0u32]);
            }
        }
    }
    true
}

// Translated from 005d09c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetRestrained` body: returns `GetRestrainedConditionFunction(thisObj,
/// 0, 0, result)`.
pub fn fn_005d09c0(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_RESTRAINED_CONDITION, a, 0)
}

// Translated from 005d09e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a cell and a reference (both words, zero by default). Does nothing
/// to the player. For another actor reference that has a process: when the
/// actor has a package target (virtual slot `0x428`), the process's package
/// (slot `0x27c`) is cast from `TESPackage` to `FleePackage`; for such a
/// package the location is set to the reference (or, without one, the cell)
/// and the byte at `+0x80` of the package is cleared. Without a package
/// target the actor's slot `0x410` is called with `(0, 0, 0, 1, cell,
/// reference, f, f)` where `f` is the `float` at `01012054`. Fails only when
/// the parameters do not parse.
pub fn fn_005d09e0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([cell, reference]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if a.this_obj.addr() == e.global::<u32>(PLAYER) {
        return true;
    }
    if a.this_obj.is_null() {
        return true;
    }
    let actor = actor_of(e, a.this_obj.addr());
    if actor == 0 || e.call(GET_PROCESS, &args![actor]).u32() == 0 {
        return true;
    }
    let package_target = e.vcall(actor, ACTOR_SLOT_428, &args![]).u32();
    if package_target != 0 {
        let value = e.call(GET_FIELD_C0, &args![package_target]).u32();
        e.call(NOOP_TWO_WORDS, &args![package_target, value, 0u32]);
        let process = e.call(GET_PROCESS, &args![actor]).u32();
        let package = e.vcall(process, PROCESS_SLOT_27C, &args![]).u32();
        let flee = e
            .call(
                DYNAMIC_CAST,
                &args![package, 0u32, RTTI_TES_PACKAGE, RTTI_FLEE_PACKAGE, 0u32],
            )
            .u32();
        if flee != 0 {
            let location = e.call(PACKAGE_GET_LOCATION, &args![flee]).u32();
            if reference != 0 {
                e.call(PACKAGE_LOCATION_SET_REFERENCE, &args![location, reference]);
                e.call(PACKAGE_SET_BYTE_80, &args![flee, 0u32]);
            } else if cell != 0 {
                e.call(PACKAGE_LOCATION_SET_CELL, &args![location, cell]);
                e.call(PACKAGE_SET_BYTE_80, &args![flee, 0u32]);
            }
        }
    } else {
        let first: f32 = e.global(FLOAT_ARGUMENT);
        let second: f32 = e.global(FLOAT_ARGUMENT);
        e.vcall(
            actor,
            ACTOR_SLOT_410,
            &args![0u32, 0u32, 0u32, 1u32, cell, reference, second, first],
        );
    }
    true
}

// ---- Callees and constants of the third batch (by exe address) ---------------

/// `Script::GetOffersServiceConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, 0, 0, result`); the condition functions below take the same
/// four words.
const GET_OFFERS_SERVICE_CONDITION: u32 = 0x005a_2af0;
/// `Script::GetBarterGoldConditionFunction` (Xbox PDB).
const GET_BARTER_GOLD_CONDITION: u32 = 0x005a_2ba0;
/// `Script::IsXboxConditionFunction` (Xbox PDB).
const IS_XBOX_CONDITION: u32 = 0x005a_37b0;
/// `Script::IsPS3ConditionFunction` (Xbox PDB).
const IS_PS3_CONDITION: u32 = 0x005a_3820;
/// `Script::IsPCConditionFunction` (Xbox PDB).
const IS_PC_CONDITION: u32 = 0x005a_3870;
/// Condition function without a name in the map (`tesconditionfunctions.cpp`).
const CONDITION_005A2C30: u32 = 0x005a_2c30;
/// `Script::IsTimePassingConditionFunction` (Xbox PDB).
const IS_TIME_PASSING_CONDITION: u32 = 0x005a_2c70;
/// `Script::GetArmorRatingUpperConditionFunction` (Xbox PDB).
const GET_ARMOR_RATING_UPPER_CONDITION: u32 = 0x005a_2ce0;
/// `Script::GetCellOwnershipConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, first, second, result`).
const GET_CELL_OWNERSHIP_CONDITION: u32 = 0x005a_2ed0;
/// Condition function without a name in the map (`tesconditionfunctions.cpp`).
const CONDITION_005A2F60: u32 = 0x005a_2f60;
/// `Script::IsRunningConditionFunction` (Xbox PDB).
const IS_RUNNING_CONDITION: u32 = 0x005a_3010;
/// `Script::GetFriendHitConditionFunction` (Xbox PDB).
const GET_FRIEND_HIT_CONDITION: u32 = 0x005a_30c0;
/// `Script::IsInCombatConditionFunction` (Xbox PDB).
const IS_IN_COMBAT_CONDITION: u32 = 0x005a_3180;

/// `Actor::GetBarterGoldBase` (Xbox PDB), `thiscall`.
const ACTOR_GET_BARTER_GOLD_BASE: u32 = 0x0088_4320;
/// `thiscall` on actor base data: the `u16` at `+0x0a` (the base barter
/// gold, `0047e240` stores it).
const ACTOR_BASE_DATA_GET_BARTER_GOLD: u32 = 0x0047_d3f0;
/// `thiscall` on actor base data (`value`): stores the `u16` at `+0x0a` and
/// notifies.
const ACTOR_BASE_DATA_SET_BARTER_GOLD: u32 = 0x0047_e240;
/// `thiscall` on actor base data (`value`): stores the `u16` at `+0x0e` and
/// notifies.
const ACTOR_BASE_DATA_SET_FIELD_0E: u32 = 0x0047_de40;
/// `thiscall` on actor base data (`value`): stores the `u16` at `+0x10` and
/// notifies.
const ACTOR_BASE_DATA_SET_FIELD_10: u32 = 0x0047_de70;
/// `TESActorBaseData::SetFlagBit` (Xbox PDB), `thiscall` (`mask, set,
/// notify`).
const ACTOR_BASE_DATA_SET_FLAG_BIT: u32 = 0x0047_dd50;
/// The base data flag `005d1070` sets or clears.
const ACTOR_BASE_FLAG_0X80: u32 = 0x80;
/// `thiscall` on the player: `this + 0x878`, its character progression.
const PLAYER_CHARACTER_PROGRESSION: u32 = 0x0046_4e10;
/// `CharacterProgression::ForceLevel` (Xbox PDB), `thiscall` (`level`).
const CHARACTER_PROGRESSION_FORCE_LEVEL: u32 = 0x008d_5300;
/// `TESObjectREFR::HasContainer` (Xbox PDB), `thiscall`: the container of
/// the reference's base form (null without).
const REFERENCE_HAS_CONTAINER: u32 = 0x0055_d310;
/// `InventoryChanges::RemoveAllObjectsWorn` (Xbox PDB), `thiscall` (`0, 0`).
const INVENTORY_REMOVE_ALL_OBJECTS_WORN: u32 = 0x004b_fe50;
/// `thiscall` on the inventory changes, no arguments (`inventorychanges.cpp`).
const INVENTORY_CHANGES_FN_004D17A0: u32 = 0x004d_17a0;
/// `thiscall` on the inventory changes, no arguments (`inventorychanges.cpp`).
const INVENTORY_CHANGES_FN_004D1440: u32 = 0x004d_1440;
/// `thiscall` on a `TESNPC` (`actor, 1, 1, 0, 1`, `tesnpc.cpp`).
const NPC_INIT_DEFAULT_WORN: u32 = 0x0060_47c0;
/// `TESCreature::InitDefaultWorn` (Xbox PDB), `thiscall` (`actor, 1, 1, 1`).
const CREATURE_INIT_DEFAULT_WORN: u32 = 0x005f_9e00;
/// RTTI type descriptor of `TESActorBase` (`.?AVTESActorBase@@`).
const RTTI_TES_ACTOR_BASE: u32 = 0x0118_46e8;
/// RTTI type descriptor of `TESCreature` (`.?AVTESCreature@@`).
const RTTI_TES_CREATURE: u32 = 0x0118_3a00;
/// Virtual slot `0x360` of an actor: a byte, set for the player's actor.
const ACTOR_SLOT_360: u32 = 0x360;
/// Virtual slot `0x218` of an actor: a byte.
const ACTOR_SLOT_218: u32 = 0x218;
/// Virtual slot `0x148` of the actor's process: the weapon item entry
/// (null without).
const PROCESS_SLOT_148: u32 = 0x148;
/// Virtual slot `0x3c8` of an actor: takes `(item, float, 1)`.
const ACTOR_SLOT_3C8: u32 = 0x3c8;
/// Virtual slot `0x538` of the actor's process: takes a `float`.
const PROCESS_SLOT_538: u32 = 0x538;
/// `ItemChange::GetItemHealth` (Xbox PDB), `thiscall` (`0`): the health in
/// `ST0`.
const ITEM_CHANGE_GET_ITEM_HEALTH: u32 = 0x004b_cdb0;
/// `MobileObject::GetCharController` (Xbox PDB), `thiscall`.
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `PlayerCharacter::SetActorBaseValue` (Xbox PDB), `thiscall` (`attribute,
/// value`).
const PLAYER_SET_ACTOR_BASE_VALUE: u32 = 0x0093_a7c0;
/// The object embedded at `+0xa4` of the player: its virtual slot 0 takes an
/// attribute number and returns the attribute's value.
const PLAYER_ATTRIBUTE_OBJECT: u32 = 0xa4;

/// The player's flag byte at `+0x66d` (`005d14d0` sets or clears bit 0,
/// `005d1530` bit 1).
const PLAYER_FLAGS_66D: u32 = 0x66d;
/// The player's byte at `+0x66e` (`005d1590`).
const PLAYER_BYTE_66E: u32 = 0x66e;
/// The `float` at `+0x544` of a character controller (`005d10f0`).
const CHAR_CONTROLLER_FLOAT_544: u32 = 0x544;

/// `BSAudio::QInstance` (Xbox PDB), `cdecl`, no arguments: the audio
/// singleton.
const BS_AUDIO_Q_INSTANCE: u32 = 0x0045_3a70;
/// `thiscall` on the audio singleton: the byte at `+6`.
const AUDIO_GET_FLAG: u32 = 0x005b_b4d0;
/// `BSAudio::SetMultiThreaded` (Xbox PDB), `thiscall` (`flag`).
const AUDIO_SET_MULTI_THREADED: u32 = 0x00ad_7230;
/// `BSAudio::MuteType` (Xbox PDB), `thiscall` (`type, a, b`).
const AUDIO_MUTE_TYPE: u32 = 0x00ad_8480;
/// `thiscall` on the audio singleton (`type, a`), `bsaudio.cpp`.
const AUDIO_FN_00AD84B0: u32 = 0x00ad_84b0;
/// Pointer to the I/O manager (`iomanager.obj`).
const IO_MANAGER: u32 = 0x0120_2d98;
/// `thiscall` on the I/O manager: the call that precedes a movie.
const IO_MANAGER_BEFORE_MOVIE: u32 = 0x00c3_e310;
/// `thiscall` on the I/O manager: the call that follows a movie.
const IO_MANAGER_AFTER_MOVIE: u32 = 0x00c3_e340;
/// Pointer to the movie player.
const MOVIE_PLAYER: u32 = 0x0126_fac4;
/// `MoviePlayer::Play` (Xbox PDB), `thiscall` (file name and eight more
/// words).
const MOVIE_PLAYER_PLAY: u32 = 0x00ec_2320;
/// Size of the `char` buffer `005d15d0` has the file name parsed into.
const MOVIE_NAME_BUFFER_SIZE: u32 = 0x200;
/// The second word `005d15d0` gives the audio mute call.
const MOVIE_MUTE_WORD: u32 = 6000;

/// `thiscall` on a string (a name): its text, or the pooled empty string
/// when it has none (the map names this body
/// `MapMarkerData::GetLocationName`).
const STRING_TEXT_OR_EMPTY: u32 = 0x0040_8da0;
/// `thiscall` on a string (`text`): assigns the text (the map names this
/// body `TESSoundFile::SetSoundFile`).
const STRING_SET_TEXT: u32 = 0x0048_9100;
/// `thiscall` on a cell, one argument (`tesobjectcell.cpp`).
const CELL_FN_00546BF0: u32 = 0x0054_6bf0;
/// Offset of the string `005d1890` and `005d1920` read from their parsed
/// form.
const FORM_STRING_OFFSET: u32 = 0x18;
/// Offset of the string `005d1920` sets in the base form of an actor
/// reference.
const ACTOR_BASE_STRING_OFFSET: u32 = 0xd0;
/// Offset of the string `005d1920` sets in the base form of another
/// reference.
const OTHER_BASE_STRING_OFFSET: u32 = 0x30;
/// Form type byte of the base forms `005d1920` handles (not named here).
const FORM_TYPE_0X16: u32 = 0x16;
/// The change flags `005d1920` raises on the base form of an actor.
const CHANGE_FLAGS_ACTOR_BASE: u32 = 0x20;
/// The change flags `005d1890` and `005d1920` raise on the other form.
const CHANGE_FLAGS_STRING: u32 = 4;

/// `ProcessLists::AddTempEffect` (Xbox PDB), `thiscall` on the process
/// lists (`effect`).
const PROCESS_LISTS_ADD_TEMP_EFFECT: u32 = 0x0097_3fd0;
/// `ProcessLists::FinishMagicShaderHitEffect` (Xbox PDB), `thiscall` on the
/// process lists (`reference, shader`).
const PROCESS_LISTS_FINISH_MAGIC_SHADER: u32 = 0x0097_4a50;
/// `TES::IsCellLoaded` (Xbox PDB), `thiscall` on the `TES` singleton
/// (`cell, 0`).
const TES_IS_CELL_LOADED: u32 = 0x0045_11e0;
/// `cdecl` (`size`): allocates (`NiSystem`).
const ALLOCATE: u32 = 0x00aa_13e0;
/// `thiscall` on a 4-byte stack object (`context, 1, file, line`): a memory
/// context scope guard (`effectitem.cpp`).
const SCOPE_GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
/// `thiscall`: the scope guard's destructor.
const SCOPE_GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// The memory context number of the scope guard of `005d1b80`.
const SCOPE_GUARD_CONTEXT: u32 = 0x32;
/// The source file name the scope guard of `005d1b80` is given
/// (`...\TESScriptFunctions.cpp`).
const SCRIPT_FUNCTIONS_FILE: u32 = 0x0103_bcb8;
/// The line number the scope guard of `005d1b80` is given.
const SCRIPT_FUNCTIONS_LINE: u32 = 0x409c;
/// Constructor of the magic shader hit effect (`magicshaderhiteffect.cpp`,
/// `thiscall`: `reference, shader, float`).
const MAGIC_SHADER_HIT_EFFECT_CONSTRUCT: u32 = 0x0081_f580;
/// Size of the magic shader hit effect.
const MAGIC_SHADER_HIT_EFFECT_SIZE: u32 = 0x6c;
/// `MagicModelHitEffect::MagicModelHitEffect_ov2` (Xbox PDB), `thiscall`
/// (`reference, model path, float`).
const MAGIC_MODEL_HIT_EFFECT_CONSTRUCT: u32 = 0x0081_c720;
/// Size of the magic model hit effect.
const MAGIC_MODEL_HIT_EFFECT_SIZE: u32 = 0x3c;
/// Virtual slot `0xc4` of a hit effect: a byte, true when the effect
/// initialised.
const HIT_EFFECT_INITIALISE_SLOT: u32 = 0xc4;
/// Virtual slot 0 of a hit effect: the scalar deleting destructor (`1`).
const HIT_EFFECT_DESTRUCTOR_SLOT: u32 = 0;
/// `thiscall` on a reference: `this + 0x24`, the address of its rotation.
const REFERENCE_GET_ROTATION: u32 = 0x0043_0830;
/// Virtual slot `0x1f4` of a reference: the address of its position.
const REFERENCE_GET_POSITION_SLOT: u32 = 0x1f4;
/// Constructor of a three-`float` vector (`thiscall`, `x, y, z`).
const VECTOR_CONSTRUCT: u32 = 0x0041_6870;
/// `thiscall` on the model string of an effect setting (at `+0x18`): its
/// length (`0048cee0`).
const STRING_LENGTH_AT_4: u32 = 0x0048_cee0;
/// Offset of the model string in the magic effect setting.
const EFFECT_MODEL_STRING_OFFSET: u32 = 0x18;
/// Virtual slot `0x14` of the model string object: the model path.
const MODEL_PATH_SLOT: u32 = 0x14;
/// `cdecl` (`cell, 1.0, model path, direction (3 words), position (3
/// words), 1.0, 7, 0`): creates a particle temp effect and returns it
/// (`bstempeffectparticle.cpp`).
const CREATE_PARTICLE_EFFECT: u32 = 0x0068_90b0;
/// The last-but-one word `005d1dc0` gives [`CREATE_PARTICLE_EFFECT`].
const PARTICLE_EFFECT_FLAGS: u32 = 7;
/// `thiscall` on that effect (`name`).
const PARTICLE_EFFECT_SET_NAME: u32 = 0x0068_9920;
/// The name `005d1dc0` gives the particle effect (`"SpecialIdle_HitEffect"`).
const SPECIAL_IDLE_HIT_EFFECT_NAME: u32 = 0x0103_bdb8;
/// `cdecl`, one `float` in, one out (`fn_005b9e80`, in the unit's main
/// file).
const FLOAT_FUNCTION_A: u32 = 0x005b_9e80;
/// `cdecl`, one `float` in, one out (`fn_005c53d0`).
const FLOAT_FUNCTION_B: u32 = 0x005c_53d0;

// ---- String literals of the third batch ------------------------------------------

/// `"Actor Barter Gold is  %d "`
const MSG_ACTOR_BARTER_GOLD: u32 = 0x0103_bb74;
/// `" Actor's base barter gold is  %d "`
const MSG_BASE_BARTER_GOLD: u32 = 0x0103_bb90;
/// `"%s has no weapon equipped."`
const MSG_NO_WEAPON: u32 = 0x0103_bbb4;
/// `"%.2f/100.0"`
const MSG_WEAPON_HEALTH: u32 = 0x0103_bbd0;
/// `"Package duration is set to  %.02f "`
const MSG_PACKAGE_DURATION: u32 = 0x0103_bbdc;
/// `"Shader effect initialization failed for reference"`
const MSG_SHADER_FAILED_REFERENCE: u32 = 0x0103_bc00;
/// `"Shader effect initialization failed for %s"`
const MSG_SHADER_FAILED_NAMED: u32 = 0x0103_bc34;
/// `"Shader effect has been applied to reference"`
const MSG_SHADER_APPLIED_REFERENCE: u32 = 0x0103_bc60;
/// `"Shader effect has been applied to %s"`
const MSG_SHADER_APPLIED_NAMED: u32 = 0x0103_bc8c;
/// `"Visual effect initialization failed for reference"`
const MSG_VISUAL_FAILED_REFERENCE: u32 = 0x0103_bd04;
/// `"Visual effect initialization failed for %s"`
const MSG_VISUAL_FAILED_NAMED: u32 = 0x0103_bd38;
/// `"Visual effect has been applied to reference"`
const MSG_VISUAL_APPLIED_REFERENCE: u32 = 0x0103_bd64;
/// `"Visual effect has been applied to %s"`
const MSG_VISUAL_APPLIED_NAMED: u32 = 0x0103_bd90;
/// `"Shader effect has been removed from reference"`
const MSG_SHADER_REMOVED_REFERENCE: u32 = 0x0103_bdd0;
/// `"Shader effect has been removed from %s"`
const MSG_SHADER_REMOVED_NAMED: u32 = 0x0103_be00;

// ---- Translated functions, third batch -----------------------------------------------

/// The echo of the effect commands: when the console echo is on, the
/// message with the name of the reference if it has one (the game asks for
/// the name twice), else the message without a name.
fn echo_reference_name(e: &mut Engine, reference: u32, named: u32, unnamed: u32) {
    if !echo_enabled(e) {
        return;
    }
    if e.call(GET_REFERENCE_NAME, &args![reference]).u32() != 0 {
        let name = e.call(GET_REFERENCE_NAME, &args![reference]).u32();
        console_print(e, &args![named, name]);
    } else {
        console_print(e, &args![unnamed]);
    }
}

// Translated from 005d0b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` on a package (`flag`): stores the byte at `+0x80` (the name of
/// the field is not confirmed).
pub fn fn_005d0b80(e: &mut Engine, this: Ptr, flag: u8) {
    e.mem.set_u8(this.addr() + 0x80, flag);
}

// Translated from 005d0ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses two integers (zero by default). Does nothing to the player. For
/// another actor reference that has a process and a package target (virtual
/// slot `0x428`), calls `008d0600` on the target with the first integer and
/// the second as a `float`; that body does nothing in this build. Fails only
/// when the parameters do not parse.
pub fn fn_005d0ba0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, second]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if a.this_obj.addr() == e.global::<u32>(PLAYER) {
        return true;
    }
    if !a.this_obj.is_null() {
        let actor = actor_of(e, a.this_obj.addr());
        if actor != 0 && e.call(GET_PROCESS, &args![actor]).u32() != 0 {
            let target = e.vcall(actor, ACTOR_SLOT_428, &args![]).u32();
            if target != 0 {
                e.call(NOOP_TWO_WORDS, &args![target, first, second as i32 as f32]);
            }
        }
    }
    true
}

// Translated from 005d0c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetOffersService` body: returns
/// `GetOffersServiceConditionFunction(thisObj, 0, 0, result)`.
pub fn fn_005d0c70(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_OFFERS_SERVICE_CONDITION, a, 0)
}

// Translated from 005d0c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ModBarterGoldFunction` (Xbox PDB): when the reference is an
/// actor, parses one integer (and fails when it does not parse; the value is
/// not used). With the console echo on, prints the actor's base barter gold
/// (`Actor::GetBarterGoldBase`, asked even when `thisObj` is no actor).
pub fn script_mod_barter_gold_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = actor_of(e, a.this_obj.addr());
    if actor != 0 && parse_params(e, a, [0]).is_none() {
        return false;
    }
    if echo_enabled(e) {
        let gold = e.call(ACTOR_GET_BARTER_GOLD_BASE, &args![actor]).u32();
        console_print(e, &args![MSG_ACTOR_BARTER_GOLD, gold]);
    }
    true
}

// Translated from 005d0d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetBarterGoldFunction` (Xbox PDB): for an actor reference whose
/// base form is a `TESNPC`, parses one integer and stores its low 16 bits as
/// the barter gold of the NPC's base data (`npc + 0x30`). For any actor
/// reference, with the console echo on, prints the base data's barter gold
/// (the game reads it from `npc + 0x30` even when the base form is no NPC).
/// Fails only when the parameters do not parse.
pub fn script_set_barter_gold_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = actor_of(e, a.this_obj.addr());
    if actor == 0 {
        return true;
    }
    let base = e.call(GET_BASE_FORM, &args![actor]).u32();
    let npc = e
        .call(
            DYNAMIC_CAST,
            &args![base, 0u32, RTTI_TES_BOUND_OBJECT, RTTI_TES_NPC, 0u32],
        )
        .u32();
    if npc != 0 {
        let Some([gold]) = parse_params(e, a, [0]) else {
            return false;
        };
        e.call(
            ACTOR_BASE_DATA_SET_BARTER_GOLD,
            &args![npc + 0x30, gold as u16],
        );
    }
    if echo_enabled(e) {
        let gold = e
            .call(ACTOR_BASE_DATA_GET_BARTER_GOLD, &args![npc + 0x30])
            .u16();
        console_print(e, &args![MSG_BASE_BARTER_GOLD, gold as u32]);
    }
    true
}

// Translated from 005d0e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetBarterGold` body: returns
/// `GetBarterGoldConditionFunction(thisObj, 0, 0, result)`.
pub fn fn_005d0e10(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_BARTER_GOLD_CONDITION, a, 0)
}

// Translated from 005d0e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a level, a flag and two more levels (default `-1`) for an actor
/// reference. For the actor whose virtual slot `0x360` answers true, the
/// player's character progression is forced to the level
/// (`CharacterProgression::ForceLevel`). For any other actor the level (and
/// the two other levels when given, each followed by setting base flag
/// `0x80`; the flag itself too when the second integer is non-zero) is
/// stored in the base data (`base form + 0x30`), the worn inventory of an
/// actor with a container is cleared and refilled with the default worn
/// items of its `TESNPC` or `TESCreature` base, and an actor whose slot
/// `0x218` answers true has the `TESNPC` values initialised. Fails only when
/// the parameters do not parse; a reference that is no actor succeeds
/// without parsing.
pub fn fn_005d0e30(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = actor_of(e, a.this_obj.addr());
    if actor == 0 {
        return true;
    }
    let base = e.call(GET_BASE_FORM_OF_REFERENCE, &args![actor]).u32();
    let Some([level, set_flag, level_b, level_c]) = parse_params(e, a, [0, 0, u32::MAX, u32::MAX])
    else {
        return false;
    };
    if e.vcall(actor, ACTOR_SLOT_360, &args![]).bool() {
        let player = e.global::<u32>(PLAYER);
        let progression = e.call(PLAYER_CHARACTER_PROGRESSION, &args![player]).u32();
        e.call(
            CHARACTER_PROGRESSION_FORCE_LEVEL,
            &args![progression, level],
        );
        return true;
    }
    let base_data = base + 0x30;
    e.call(ACTOR_BASE_DATA_SET_LEVEL, &args![base_data, level as u16]);
    if set_flag != 0 {
        fn_005d1070(e, Ptr::new(base_data), 1);
    }
    if level_b != u32::MAX {
        e.call(
            ACTOR_BASE_DATA_SET_FIELD_0E,
            &args![base_data, level_b as u16],
        );
        fn_005d1070(e, Ptr::new(base_data), 1);
    }
    if level_c != u32::MAX {
        e.call(
            ACTOR_BASE_DATA_SET_FIELD_10,
            &args![base_data, level_c as u16],
        );
        fn_005d1070(e, Ptr::new(base_data), 1);
    }
    let has_container = e.call(REFERENCE_HAS_CONTAINER, &args![actor]).u32();
    let changes = e.call(GET_INVENTORY_CHANGES, &args![actor]).u32();
    if has_container != 0 && changes != 0 {
        e.call(
            INVENTORY_REMOVE_ALL_OBJECTS_WORN,
            &args![changes, 0u32, 0u32],
        );
        e.call(INVENTORY_CHANGES_FN_004D17A0, &args![changes]);
        e.call(INVENTORY_CHANGES_FN_004D1440, &args![changes]);
        let npc = e
            .call(
                DYNAMIC_CAST,
                &args![base, 0u32, RTTI_TES_ACTOR_BASE, RTTI_TES_NPC, 0u32],
            )
            .u32();
        let creature = e
            .call(
                DYNAMIC_CAST,
                &args![base, 0u32, RTTI_TES_ACTOR_BASE, RTTI_TES_CREATURE, 0u32],
            )
            .u32();
        if npc != 0 {
            e.call(
                NPC_INIT_DEFAULT_WORN,
                &args![npc, actor, 1u32, 1u32, 0u32, 1u32],
            );
        } else if creature != 0 {
            e.call(
                CREATURE_INIT_DEFAULT_WORN,
                &args![creature, actor, 1u32, 1u32, 1u32],
            );
        }
    }
    if e.vcall(actor, ACTOR_SLOT_218, &args![]).bool() {
        let base = e.call(GET_BASE_FORM, &args![actor]).u32();
        let npc = e
            .call(
                DYNAMIC_CAST,
                &args![base, 0u32, RTTI_TES_BOUND_OBJECT, RTTI_TES_NPC, 0u32],
            )
            .u32();
        if npc != 0 {
            e.call(NPC_INIT_VALUES, &args![npc, 0u32]);
        }
    }
    true
}

// Translated from 005d1070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` on actor base data (`flag`): `TESActorBaseData::SetFlagBit(0x80,
/// flag, 1)` (Xbox PDB name of the callee).
pub fn fn_005d1070(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(
        ACTOR_BASE_DATA_SET_FLAG_BIT,
        &args![this, ACTOR_BASE_FLAG_0X80, flag, 1u32],
    );
}

// Translated from 005d10a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ResetFallDamageTimerFunction` (Xbox PDB): for an actor
/// reference whose character controller exists, reads the `float` at
/// `+0x544` of the controller (`005d10f0`) and drops it; nothing is reset in
/// this build. Always succeeds.
pub fn script_reset_fall_damage_timer_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = actor_of(e, a.this_obj.addr());
    if actor != 0 && e.call(GET_CHAR_CONTROLLER, &args![actor]).u32() != 0 {
        let controller = e.call(GET_CHAR_CONTROLLER, &args![actor]).u32();
        // The game stores the value in a local it never reads.
        let _timer = fn_005d10f0(e, Ptr::new(controller));
    }
    true
}

// Translated from 005d10f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fastcall` on a character controller: the `float` at `+0x544`, returned
/// in `ST0`.
pub fn fn_005d10f0(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + CHAR_CONTROLLER_FLOAT_544)
}

// Translated from 005d1110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `IsXbox` body: returns `IsXboxConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005d1110(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_XBOX_CONDITION, a, 0)
}

// Translated from 005d1130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `IsPS3` body: returns `IsPS3ConditionFunction(thisObj, 0, 0,
/// result)`.
pub fn fn_005d1130(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_PS3_CONDITION, a, 0)
}

// Translated from 005d1150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `IsPC` body: returns `IsPCConditionFunction(thisObj, 0, 0, result)`.
pub fn fn_005d1150(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_PC_CONDITION, a, 0)
}

// Translated from 005d1170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the unnamed condition function `005a2c30(thisObj, 0, 0,
/// result)`.
pub fn fn_005d1170(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, CONDITION_005A2C30, a, 0)
}

// Translated from 005d1190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `IsTimePassing` body: returns `IsTimePassingConditionFunction(thisObj,
/// 0, 0, result)`.
pub fn fn_005d1190(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_TIME_PASSING_CONDITION, a, 0)
}

// Translated from 005d11b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ModPCAttributeFunction` (Xbox PDB): parses an attribute number
/// and an amount (both zero by default). For attributes 5 to 11
/// ([`fn_005d1260`]) the player's attribute value (virtual slot 0 of the
/// object at `player + 0xa4`) plus the amount is stored with
/// `PlayerCharacter::SetActorBaseValue`. Fails only when the parameters do
/// not parse.
pub fn script_mod_pc_attribute_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([attribute, amount]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if fn_005d1260(e, attribute as i32) {
        let player = e.global::<u32>(PLAYER);
        let current = e
            .vcall(player + PLAYER_ATTRIBUTE_OBJECT, 0, &args![attribute])
            .i32();
        let value = current.wrapping_add(amount as i32);
        e.call(
            PLAYER_SET_ACTOR_BASE_VALUE,
            &args![player, attribute, value],
        );
    }
    true
}

// Translated from 005d1260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the attribute number lies in 5 to 11 (signed comparison).
pub fn fn_005d1260(_e: &mut Engine, attribute: i32) -> bool {
    (5..0xc).contains(&attribute)
}

// Translated from 005d1290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a `float` (zero by default). Fails when the parameters do not
/// parse or `thisObj` is no actor. For an actor with a weapon item entry
/// (slot `0x148` of its process) the actor's virtual slot `0x3c8` is called
/// with `(item, float, 1)`.
pub fn fn_005d1290(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([bits]) = parse_params(e, a, [0]) else {
        return false;
    };
    let actor = actor_of(e, a.this_obj.addr());
    if actor == 0 {
        return false;
    }
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    let item = e.vcall(process, PROCESS_SLOT_148, &args![]).u32();
    if item != 0 {
        e.vcall(
            actor,
            ACTOR_SLOT_3C8,
            &args![item, f32::from_bits(bits), 1u32],
        );
    }
    true
}

// Translated from 005d1340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetWeaponHealthFunction` (Xbox PDB): fails when `thisObj` is no
/// actor. Prints (always, without the echo flag) the health of the actor's
/// weapon item entry as `"%.2f/100.0"` (a `double`), or that the actor has no
/// weapon equipped.
pub fn script_get_weapon_health_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = actor_of(e, a.this_obj.addr());
    if actor == 0 {
        return false;
    }
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    let item = e.vcall(process, PROCESS_SLOT_148, &args![]).u32();
    if item != 0 {
        let health = e
            .call(ITEM_CHANGE_GET_ITEM_HEALTH, &args![item, 0u32])
            .f64();
        console_print(e, &args![MSG_WEAPON_HEALTH, health]);
    } else {
        let name = e.call(GET_REFERENCE_NAME, &args![actor]).u32();
        console_print(e, &args![MSG_NO_WEAPON, name]);
    }
    true
}

// Translated from 005d13e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses three integers (defaults 0, 1, 0) and sets the player's flags
/// with [`fn_005d14d0`], [`fn_005d1590`] and [`fn_005d1530`]. With the first
/// integer zero: flag 0 cleared, byte `+0x66e` set from the second (0 or 1),
/// and flag 1 set when the third is 1 (left alone otherwise). With the first
/// non-zero: flag 0 set, the byte set to 1, flag 1 set when the third is 1
/// and cleared otherwise. Fails only when the parameters do not parse.
pub fn fn_005d13e0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, second, third]) = parse_params(e, a, [0, 1, 0]) else {
        return false;
    };
    let player = Ptr::new(e.global::<u32>(PLAYER));
    if first == 0 {
        fn_005d14d0(e, player, 0);
        fn_005d1590(e, player, (second != 0) as u8);
        if third == 1 {
            fn_005d1530(e, player, 1);
        }
    } else {
        fn_005d14d0(e, player, 1);
        fn_005d1590(e, player, 1);
        fn_005d1530(e, player, (third == 1) as u8);
    }
    true
}

// Translated from 005d14d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` on the player (`flag`): sets (non-zero) or clears bit 0 of the
/// byte at `+0x66d`.
pub fn fn_005d14d0(e: &mut Engine, this: Ptr, flag: u8) {
    let address = this.addr() + PLAYER_FLAGS_66D;
    let flags = e.mem.u8(address);
    e.mem
        .set_u8(address, if flag != 0 { flags | 1 } else { flags & !1 });
}

// Translated from 005d1530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` on the player (`flag`): sets (non-zero) or clears bit 1 of the
/// byte at `+0x66d`.
pub fn fn_005d1530(e: &mut Engine, this: Ptr, flag: u8) {
    let address = this.addr() + PLAYER_FLAGS_66D;
    let flags = e.mem.u8(address);
    e.mem
        .set_u8(address, if flag != 0 { flags | 2 } else { flags & !2 });
}

// Translated from 005d1590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` on the player (`value`): stores the byte at `+0x66e`.
pub fn fn_005d1590(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + PLAYER_BYTE_66E, value);
}

// Translated from 005d15b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetArmorRatingUpper` body: returns
/// `GetArmorRatingUpperConditionFunction(thisObj, 0, 0, result)`.
pub fn fn_005d15b0(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_ARMOR_RATING_UPPER_CONDITION, a, 0)
}

// Translated from 005d15d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Plays a movie: parses a file name (a 512-byte buffer) and four integers
/// `A`, `B`, `C`, `D` in this order (defaults 0, 1, 1, 1; the game names none
/// of them). Fails only when the parameters do not parse. Otherwise: with
/// `B` non-zero the audio singleton is muted (`005d1720(0, 6000)`); with `B`
/// zero and the audio flag (`005bb4d0`, read before parsing) clear, its multi-threading is turned
/// on. The I/O manager is told before, `MoviePlayer::Play(name, A, 0, 0, 0,
/// D, 0, B, C)` runs, the I/O manager is told after, and the audio is put
/// back (`005d1740(0)`, or multi-threading off in the same case as above).
/// The stack cookie check is not translated.
pub fn fn_005d15d0(e: &mut Engine, a: ScriptArgs) -> bool {
    let audio = e.call(BS_AUDIO_Q_INSTANCE, &args![]).u32();
    let audio_flag = e.call(AUDIO_GET_FLAG, &args![audio]).bool();
    e.with_stack(MOVIE_NAME_BUFFER_SIZE + 16, |e, buffer| {
        let locals = buffer.addr() + MOVIE_NAME_BUFFER_SIZE;
        // Parameters A, B, C, D with their defaults.
        for (i, value) in [0u32, 1, 1, 1].iter().enumerate() {
            e.mem.set_u32(locals + 4 * i as u32, *value);
        }
        let outs = [buffer.addr(), locals, locals + 4, locals + 8, locals + 12];
        if !parse(e, a, &outs) {
            return false;
        }
        let first = e.mem.u32(locals);
        let second = e.mem.u32(locals + 4);
        let third = e.mem.u32(locals + 8);
        let fourth = e.mem.u32(locals + 12);
        if second != 0 {
            let audio = e.call(BS_AUDIO_Q_INSTANCE, &args![]).u32();
            fn_005d1720(e, Ptr::new(audio), 0, MOVIE_MUTE_WORD);
        } else if !audio_flag {
            let audio = e.call(BS_AUDIO_Q_INSTANCE, &args![]).u32();
            e.call(AUDIO_SET_MULTI_THREADED, &args![audio, 1u32]);
        }
        let io_manager = e.global::<u32>(IO_MANAGER);
        e.call(IO_MANAGER_BEFORE_MOVIE, &args![io_manager]);
        let player = e.global::<u32>(MOVIE_PLAYER);
        e.call(
            MOVIE_PLAYER_PLAY,
            &args![
                player,
                buffer,
                (first != 0) as u32,
                0u32,
                0u32,
                0u32,
                (fourth != 0) as u32,
                0u32,
                (second != 0) as u32,
                (third != 0) as u32
            ],
        );
        let io_manager = e.global::<u32>(IO_MANAGER);
        e.call(IO_MANAGER_AFTER_MOVIE, &args![io_manager]);
        if second != 0 {
            let audio = e.call(BS_AUDIO_Q_INSTANCE, &args![]).u32();
            fn_005d1740(e, Ptr::new(audio), 0);
        } else if !audio_flag {
            let audio = e.call(BS_AUDIO_Q_INSTANCE, &args![]).u32();
            e.call(AUDIO_SET_MULTI_THREADED, &args![audio, 0u32]);
        }
        true
    })
}

// Translated from 005d1720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` on the audio singleton (`a, b`): `BSAudio::MuteType(-1, a, b)`
/// (Xbox PDB name of the callee).
pub fn fn_005d1720(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(AUDIO_MUTE_TYPE, &args![this, u32::MAX, first, second]);
}

// Translated from 005d1740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` on the audio singleton (`a`): `00ad84b0(-1, a)`.
pub fn fn_005d1740(e: &mut Engine, this: Ptr, value: u32) {
    e.call(AUDIO_FN_00AD84B0, &args![this, u32::MAX, value]);
}

// Translated from 005d1760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores 0.0 in the result, parses two words (zero by default) and, when
/// they parse, returns true after calling
/// `GetCellOwnershipConditionFunction(thisObj, first, second, result)`; the
/// second word defaults to the player's base form (`007af430`). Fails only
/// when the parameters do not parse.
pub fn fn_005d1760(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([first, mut second]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if second == 0 {
        let player = e.global::<u32>(PLAYER);
        second = e.call(GET_BASE_FORM, &args![player]).u32();
    }
    e.call(
        GET_CELL_OWNERSHIP_CONDITION,
        &args![a.this_obj, first, second, a.result],
    );
    true
}

// Translated from 005d17f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses two words (zero by default; the second defaults to the player's
/// base form, `007af430`). When the first is non-zero, calls `00546bf0` on it
/// (a cell) with the second. Fails only when the parameters do not parse.
pub fn fn_005d17f0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, mut second]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if second == 0 {
        let player = e.global::<u32>(PLAYER);
        second = e.call(GET_BASE_FORM, &args![player]).u32();
    }
    if first != 0 {
        e.call(CELL_FN_00546BF0, &args![first, second]);
    }
    true
}

// Translated from 005d1870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the unnamed condition function `005a2a50(thisObj, 0, 0, result)`
/// and succeeds.
pub fn fn_005d1870(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, UNKNOWN_CONDITION_005A2A50, a, 0);
    true
}

// Translated from 005d1890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses two forms (zero by default). When both are given, the string at
/// `+0x18` of the second is copied to the string at `+0x18` of the first
/// (`00408da0`, `00489100`) and the first is told change flags 4 (virtual
/// slot `0x48`). Fails only when the parameters do not parse.
pub fn fn_005d1890(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([target, source]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if target != 0 && source != 0 {
        let text = e
            .call(STRING_TEXT_OR_EMPTY, &args![source + FORM_STRING_OFFSET])
            .u32();
        e.call(STRING_SET_TEXT, &args![target + FORM_STRING_OFFSET, text]);
        e.vcall(target, ADD_CHANGE_SLOT, &args![CHANGE_FLAGS_STRING]);
    }
    true
}

// Translated from 005d1920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Succeeds without doing anything when `thisObj` is null. Parses one form
/// (zero by default); fails when it does not parse. For a non-zero form, the
/// string at `+0x18` of that form is copied to a string of the base form of
/// `thisObj`: for an actor reference (virtual slot `0x100`) to the string at
/// `+0xd0` of the base form, which is told change flags `0x20`; otherwise,
/// when the base form's type byte is `0x16`, to the string at `+0x30`, told
/// change flags 4.
pub fn fn_005d1920(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return true;
    }
    let this_obj = a.this_obj.addr();
    let Some([form]) = parse_params(e, a, [0]) else {
        return false;
    };
    if form == 0 {
        return true;
    }
    if e.vcall(this_obj, IS_ACTOR_SLOT, &args![]).bool() {
        let text = e
            .call(STRING_TEXT_OR_EMPTY, &args![form + FORM_STRING_OFFSET])
            .u32();
        let base = e.call(GET_BASE_FORM_OF_REFERENCE, &args![this_obj]).u32();
        e.call(
            STRING_SET_TEXT,
            &args![base + ACTOR_BASE_STRING_OFFSET, text],
        );
        let base = e.call(GET_BASE_FORM_OF_REFERENCE, &args![this_obj]).u32();
        e.vcall(base, ADD_CHANGE_SLOT, &args![CHANGE_FLAGS_ACTOR_BASE]);
    } else {
        let base = e.call(GET_BASE_FORM, &args![this_obj]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() == FORM_TYPE_0X16 {
            let base = e.call(GET_BASE_FORM, &args![this_obj]).u32();
            let text = e
                .call(STRING_TEXT_OR_EMPTY, &args![form + FORM_STRING_OFFSET])
                .u32();
            e.call(
                STRING_SET_TEXT,
                &args![base + OTHER_BASE_STRING_OFFSET, text],
            );
            e.vcall(base, ADD_CHANGE_SLOT, &args![CHANGE_FLAGS_STRING]);
        }
    }
    true
}

// Translated from 005d1a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the unnamed condition function `005a2f60(thisObj, 0, 0, result)`
/// and succeeds.
pub fn fn_005d1a30(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, CONDITION_005A2F60, a, 0);
    true
}

// Translated from 005d1a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `IsRunningConditionFunction(thisObj, 0, 0, result)` and succeeds.
pub fn fn_005d1a50(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_RUNNING_CONDITION, a, 0);
    true
}

// Translated from 005d1a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `GetFriendHitConditionFunction(thisObj, 0, 0, result)` and
/// succeeds.
pub fn fn_005d1a70(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_FRIEND_HIT_CONDITION, a, 0);
    true
}

// Translated from 005d1a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `IsInCombatConditionFunction(thisObj, 0, 0, result)` and succeeds.
pub fn fn_005d1a90(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_IN_COMBAT_CONDITION, a, 0);
    true
}

// Translated from 005d1ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetPackageDurationFunction` (Xbox PDB): parses a `float` (zero by
/// default; fails when it does not parse). For an actor reference with a
/// process, calls the process's virtual slot `0x538` with the float. With
/// the console echo on, prints `"Package duration is set to  %.02f "`.
pub fn script_set_package_duration_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = actor_of(e, a.this_obj.addr());
    let Some([bits]) = parse_params(e, a, [0]) else {
        return false;
    };
    if actor != 0 {
        let value = f32::from_bits(bits);
        if e.call(GET_PROCESS, &args![actor]).u32() != 0 {
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            e.vcall(process, PROCESS_SLOT_538, &args![value]);
        }
        if echo_enabled(e) {
            console_print(e, &args![MSG_PACKAGE_DURATION, value as f64]);
        }
    }
    true
}

// Translated from 005d1b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PlayMagicShader` (Xbox PDB): parses a shader form and a `float`
/// (default: the `float` at `01012054`, -1.0). The reference defaults to the
/// player. Succeeds without doing anything when the reference's parent cell
/// is not loaded or the reference has no 3D (virtual slot `0x1d0`).
/// Otherwise, inside a memory-context scope guard, a magic shader hit effect
/// (`0x6c` bytes) is built for the reference; when it initialises (virtual
/// slot `0xc4`) it is added to the process lists' temporary effects,
/// else it is destroyed (slot 0 with 1). The console echo says which, with
/// the reference's name when it has one. Fails only when the parameters do
/// not parse. The exception-unwinding frame is not translated.
pub fn script_play_magic_shader(e: &mut Engine, a: ScriptArgs) -> bool {
    let default_float = e.global::<u32>(FLOAT_ARGUMENT);
    let Some([shader, float_bits]) = parse_params(e, a, [0, default_float]) else {
        return false;
    };
    let mut reference = a.this_obj.addr();
    if reference == 0 {
        reference = e.global::<u32>(PLAYER);
    }
    let cell = e.call(GET_PARENT_CELL, &args![reference]).u32();
    let tes = e.global::<u32>(TES_SINGLETON);
    if !e.call(TES_IS_CELL_LOADED, &args![tes, cell, 0u32]).bool() {
        return true;
    }
    if e.vcall(reference, REFERENCE_GET_3D_SLOT, &args![]).u32() == 0 {
        return true;
    }
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![
                guard,
                SCOPE_GUARD_CONTEXT,
                1u32,
                SCRIPT_FUNCTIONS_FILE,
                SCRIPT_FUNCTIONS_LINE
            ],
        );
        let block = e.call(ALLOCATE, &args![MAGIC_SHADER_HIT_EFFECT_SIZE]).u32();
        let effect = if block != 0 {
            e.call(
                MAGIC_SHADER_HIT_EFFECT_CONSTRUCT,
                &args![block, reference, shader, f32::from_bits(float_bits)],
            )
            .u32()
        } else {
            0
        };
        if e.vcall(effect, HIT_EFFECT_INITIALISE_SLOT, &args![]).bool() {
            let lists = PROCESS_LISTS;
            e.call(PROCESS_LISTS_ADD_TEMP_EFFECT, &args![lists, effect]);
            echo_reference_name(
                e,
                reference,
                MSG_SHADER_APPLIED_NAMED,
                MSG_SHADER_APPLIED_REFERENCE,
            );
        } else {
            if effect != 0 {
                e.vcall(effect, HIT_EFFECT_DESTRUCTOR_SLOT, &args![1u32]);
            }
            echo_reference_name(
                e,
                reference,
                MSG_SHADER_FAILED_NAMED,
                MSG_SHADER_FAILED_REFERENCE,
            );
        }
        e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
    });
    true
}

// Translated from 005d1dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PlayMagicEffect` (Xbox PDB): parses a magic effect setting and a
/// `float` (default: the `float` at `01012054`). The reference defaults to
/// the player. Succeeds without doing anything when the reference's parent
/// cell is not loaded. For a reference without 3D (virtual slot `0x1d0`) a
/// particle temp effect is created at the reference's position
/// (`006890b0`; direction `(-f(-z), g(-z), 0)` from its rotation's z with
/// `005b9e80` and `005c53d0`) from the model path of the setting (`+0x18`,
/// virtual slot `0x14`) and named `"SpecialIdle_HitEffect"`. For a reference
/// with 3D and a setting with a model path, a magic model hit effect (`0x3c`
/// bytes) is built; when it initialises (slot `0xc4`) it is added to the
/// process lists' temporary effects, else destroyed. The console echo says
/// what happened. Fails only when the parameters do not parse. The
/// exception-unwinding frame is not translated.
pub fn script_play_magic_effect(e: &mut Engine, a: ScriptArgs) -> bool {
    let default_float = e.global::<u32>(FLOAT_ARGUMENT);
    let Some([setting, float_bits]) = parse_params(e, a, [0, default_float]) else {
        return false;
    };
    let mut reference = a.this_obj.addr();
    if reference == 0 {
        reference = e.global::<u32>(PLAYER);
    }
    let cell = e.call(GET_PARENT_CELL, &args![reference]).u32();
    let tes = e.global::<u32>(TES_SINGLETON);
    if !e.call(TES_IS_CELL_LOADED, &args![tes, cell, 0u32]).bool() {
        return true;
    }
    if e.vcall(reference, REFERENCE_GET_3D_SLOT, &args![]).u32() == 0 {
        // No 3D: a particle effect at the reference's position.
        let rotation = e.call(REFERENCE_GET_ROTATION, &args![reference]).u32();
        let rotation_z = e.mem.f32(rotation + 8);
        let position = e
            .vcall(reference, REFERENCE_GET_POSITION_SLOT, &args![])
            .u32();
        let position_words = [
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        ];
        let first = e.call(FLOAT_FUNCTION_A, &args![-rotation_z]).f32();
        let second = e.call(FLOAT_FUNCTION_B, &args![-rotation_z]).f32();
        let direction_words = e.with_stack(12, |e, vector| {
            e.call(VECTOR_CONSTRUCT, &args![vector, -second, first, 0.0f32]);
            [
                e.mem.u32(vector.addr()),
                e.mem.u32(vector.addr() + 4),
                e.mem.u32(vector.addr() + 8),
            ]
        });
        let model = e
            .vcall(
                setting + EFFECT_MODEL_STRING_OFFSET,
                MODEL_PATH_SLOT,
                &args![],
            )
            .u32();
        let cell = e.call(GET_PARENT_CELL, &args![reference]).u32();
        let effect = e
            .call(
                CREATE_PARTICLE_EFFECT,
                &args![
                    cell,
                    1.0f32,
                    model,
                    direction_words[0],
                    direction_words[1],
                    direction_words[2],
                    position_words[0],
                    position_words[1],
                    position_words[2],
                    1.0f32,
                    PARTICLE_EFFECT_FLAGS,
                    0u32
                ],
            )
            .u32();
        e.call(
            PARTICLE_EFFECT_SET_NAME,
            &args![effect, SPECIAL_IDLE_HIT_EFFECT_NAME],
        );
        echo_reference_name(
            e,
            reference,
            MSG_VISUAL_APPLIED_NAMED,
            MSG_VISUAL_APPLIED_REFERENCE,
        );
        return true;
    }
    let mut effect = 0;
    if setting != 0
        && e.call(
            STRING_LENGTH_AT_4,
            &args![setting + EFFECT_MODEL_STRING_OFFSET],
        )
        .u32()
            != 0
    {
        let block = e.call(ALLOCATE, &args![MAGIC_MODEL_HIT_EFFECT_SIZE]).u32();
        if block != 0 {
            let model = e
                .vcall(
                    setting + EFFECT_MODEL_STRING_OFFSET,
                    MODEL_PATH_SLOT,
                    &args![],
                )
                .u32();
            effect = e
                .call(
                    MAGIC_MODEL_HIT_EFFECT_CONSTRUCT,
                    &args![block, reference, model, f32::from_bits(float_bits)],
                )
                .u32();
        }
    }
    if effect != 0 && e.vcall(effect, HIT_EFFECT_INITIALISE_SLOT, &args![]).bool() {
        e.call(PROCESS_LISTS_ADD_TEMP_EFFECT, &args![PROCESS_LISTS, effect]);
        echo_reference_name(
            e,
            reference,
            MSG_VISUAL_APPLIED_NAMED,
            MSG_VISUAL_APPLIED_REFERENCE,
        );
    } else {
        if effect != 0 {
            e.vcall(effect, HIT_EFFECT_DESTRUCTOR_SLOT, &args![1u32]);
        }
        echo_reference_name(
            e,
            reference,
            MSG_VISUAL_FAILED_NAMED,
            MSG_VISUAL_FAILED_REFERENCE,
        );
    }
    true
}

// Translated from 005d2130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::StopMagicShader` (Xbox PDB): parses a shader form and a `float`
/// (default: the `float` at `01012054`; unused); fails when they do not
/// parse. The reference defaults to the player.
/// `ProcessLists::FinishMagicShaderHitEffect(reference, shader)` runs and
/// the removal is printed (without checking the echo flag), with the
/// reference's name when it has one.
pub fn script_stop_magic_shader(e: &mut Engine, a: ScriptArgs) -> bool {
    let default_float = e.global::<u32>(FLOAT_ARGUMENT);
    let Some([shader, _float_bits]) = parse_params(e, a, [0, default_float]) else {
        return false;
    };
    let mut reference = a.this_obj.addr();
    if reference == 0 {
        reference = e.global::<u32>(PLAYER);
    }
    e.call(
        PROCESS_LISTS_FINISH_MAGIC_SHADER,
        &args![PROCESS_LISTS, reference, shader],
    );
    if e.call(GET_REFERENCE_NAME, &args![reference]).u32() != 0 {
        let name = e.call(GET_REFERENCE_NAME, &args![reference]).u32();
        console_print(e, &args![MSG_SHADER_REMOVED_NAMED, name]);
    } else {
        console_print(e, &args![MSG_SHADER_REMOVED_REFERENCE]);
    }
    true
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005cd990, script_get_detection_level_function(ScriptArgs) -> bool),
        entry!(0x005cd9f0, fn_005cd9f0(ScriptArgs) -> bool),
        entry!(0x005cda20, script_is_actor_detected(ScriptArgs) -> bool),
        entry!(0x005cdad0, fn_005cdad0(ScriptArgs) -> bool),
        entry!(0x005cdc10, fn_005cdc10(ScriptArgs) -> bool),
        entry!(0x005cde00, fn_005cde00(ScriptArgs) -> bool),
        entry!(0x005cde20, fn_005cde20(Ptr) -> i32),
        entry!(0x005cde40, fn_005cde40(ScriptArgs) -> bool),
        entry!(0x005cdea0, fn_005cdea0(ScriptArgs) -> bool),
        entry!(0x005cdec0, fn_005cdec0(ScriptArgs) -> bool),
        entry!(0x005cdf20, fn_005cdf20(Ptr, i32)),
        entry!(0x005cdf50, script_get_pc_expelled_function(ScriptArgs) -> bool),
        entry!(0x005cdfb0, script_set_pc_expelled_function(ScriptArgs) -> bool),
        entry!(0x005ce040, fn_005ce040(Ptr, u8)),
        entry!(0x005ce060, script_get_pc_faction_murder_function(ScriptArgs) -> bool),
        entry!(0x005ce0c0, fn_005ce0c0(ScriptArgs) -> bool),
        entry!(0x005ce130, script_get_player_enemyof_faction_function(ScriptArgs) -> bool),
        entry!(0x005ce190, fn_005ce190(ScriptArgs) -> bool),
        entry!(0x005ce200, script_get_pc_faction_attack_function(ScriptArgs) -> bool),
        entry!(0x005ce260, fn_005ce260(ScriptArgs) -> bool),
        entry!(0x005ce2d0, fn_005ce2d0(ScriptArgs) -> bool),
        entry!(0x005ce2f0, fn_005ce2f0(ScriptArgs) -> bool),
        entry!(0x005ce360, script_get_action_ref_function(ScriptArgs) -> bool),
        entry!(0x005ce3e0, script_get_self_function(ScriptArgs) -> bool),
        entry!(0x005ce480, script_get_combat_target_function(ScriptArgs) -> bool),
        entry!(0x005ce520, script_get_package_target_function(ScriptArgs) -> bool),
        entry!(0x005ce5c0, script_get_container_function(ScriptArgs) -> bool),
        entry!(0x005ce630, script_get_parent_ref_function(ScriptArgs) -> bool),
        entry!(0x005ce6b0, script_get_linked_ref_function(ScriptArgs) -> bool),
        entry!(0x005ce730, script_get_force_run(ScriptArgs) -> bool),
        entry!(0x005ce7b0, script_set_force_run(ScriptArgs) -> bool),
        entry!(0x005ce870, script_get_force_sneak(ScriptArgs) -> bool),
        entry!(0x005ce910, script_set_force_sneak(ScriptArgs) -> bool),
        entry!(0x005ce9d0, fn_005ce9d0(Ptr, u8)),
        entry!(0x005ce9f0, fn_005ce9f0() -> bool),
        entry!(0x005cea30, script_has_magic_effect_function(ScriptArgs) -> bool),
        entry!(0x005cea90, script_is_spell_target_function(ScriptArgs) -> bool),
        entry!(0x005ceaf0, script_get_spell_usage_number_function(ScriptArgs) -> bool),
        entry!(0x005ceb50, fn_005ceb50(ScriptArgs) -> bool),
        entry!(0x005ceb70, fn_005ceb70(ScriptArgs) -> bool),
        entry!(0x005ceb90, script_get_default_open_function(ScriptArgs) -> bool),
        entry!(0x005cebf0, script_set_default_open_function(ScriptArgs) -> bool),
        entry!(0x005ced30, script_set_open_state_function(ScriptArgs) -> bool),
        entry!(0x005cedf0, fn_005cedf0() -> bool),
        entry!(0x005cee30, fn_005cee30() -> bool),
        entry!(0x005cee50, fn_005cee50() -> bool),
        entry!(0x005cee70, fn_005cee70(ScriptArgs) -> bool),
        entry!(0x005ceeb0, fn_005ceeb0(ScriptArgs) -> bool),
        entry!(0x005cefe0, script_show_special_book_menu(ScriptArgs) -> bool),
        entry!(0x005cf040, script_show_slot_machine_menu(ScriptArgs) -> bool),
        entry!(0x005cf0f0, script_show_black_jack_menu(ScriptArgs) -> bool),
        entry!(0x005cf1a0, script_show_roulette_menu(ScriptArgs) -> bool),
        entry!(0x005cf250, script_show_caravan_menu(ScriptArgs) -> bool),
        entry!(0x005cf3d0, script_add_card_to_player(ScriptArgs) -> bool),
        entry!(0x005cf490, script_is_player_in_region(ScriptArgs) -> bool),
        entry!(0x005cf5e0, script_show_love_tester_menu(ScriptArgs) -> bool),
        entry!(0x005cf640, fn_005cf640() -> bool),
        entry!(0x005cf660, fn_005cf660() -> bool),
        entry!(0x005cf680, fn_005cf680(ScriptArgs) -> bool),
        entry!(0x005cf860, script_swap_texture_on_ref(ScriptArgs) -> bool),
        entry!(0x005cfc50, fn_005cfc50(ScriptArgs) -> bool),
        entry!(0x005cfcb0, fn_005cfcb0(u8)),
        entry!(0x005cfcd0, fn_005cfcd0(ScriptArgs) -> bool),
        entry!(0x005cfd20, fn_005cfd20(ScriptArgs) -> bool),
        entry!(0x005cfd70, fn_005cfd70(ScriptArgs) -> bool),
        entry!(0x005cfd90, fn_005cfd90(ScriptArgs) -> bool),
        entry!(0x005cfdb0, fn_005cfdb0(ScriptArgs) -> bool),
        entry!(0x005cfdd0, fn_005cfdd0(ScriptArgs) -> bool),
        entry!(0x005cfdf0, fn_005cfdf0(ScriptArgs) -> bool),
        entry!(0x005cfef0, fn_005cfef0(ScriptArgs) -> bool),
        entry!(0x005cff90, fn_005cff90(ScriptArgs) -> bool),
        entry!(0x005cffb0, script_set_ghost_function(ScriptArgs) -> bool),
        entry!(0x005d0060, script_equip_item_function(ScriptArgs) -> bool),
        entry!(0x005d0300, fn_005d0300(ScriptArgs) -> bool),
        entry!(0x005d06a0, fn_005d06a0(ScriptArgs) -> bool),
        entry!(0x005d0760, fn_005d0760(ScriptArgs) -> bool),
        entry!(0x005d0900, fn_005d0900(ScriptArgs) -> bool),
        entry!(0x005d0920, fn_005d0920(ScriptArgs) -> bool),
        entry!(0x005d09c0, fn_005d09c0(ScriptArgs) -> bool),
        entry!(0x005d09e0, fn_005d09e0(ScriptArgs) -> bool),
        entry!(0x005d0b80, fn_005d0b80(Ptr, u8)),
        entry!(0x005d0ba0, fn_005d0ba0(ScriptArgs) -> bool),
        entry!(0x005d0c70, fn_005d0c70(ScriptArgs) -> bool),
        entry!(0x005d0c90, script_mod_barter_gold_function(ScriptArgs) -> bool),
        entry!(0x005d0d30, script_set_barter_gold_function(ScriptArgs) -> bool),
        entry!(0x005d0e10, fn_005d0e10(ScriptArgs) -> bool),
        entry!(0x005d0e30, fn_005d0e30(ScriptArgs) -> bool),
        entry!(0x005d1070, fn_005d1070(Ptr, u8)),
        entry!(0x005d10a0, script_reset_fall_damage_timer_function(ScriptArgs) -> bool),
        entry!(0x005d10f0, fn_005d10f0(Ptr) -> f32),
        entry!(0x005d1110, fn_005d1110(ScriptArgs) -> bool),
        entry!(0x005d1130, fn_005d1130(ScriptArgs) -> bool),
        entry!(0x005d1150, fn_005d1150(ScriptArgs) -> bool),
        entry!(0x005d1170, fn_005d1170(ScriptArgs) -> bool),
        entry!(0x005d1190, fn_005d1190(ScriptArgs) -> bool),
        entry!(0x005d11b0, script_mod_pc_attribute_function(ScriptArgs) -> bool),
        entry!(0x005d1260, fn_005d1260(i32) -> bool),
        entry!(0x005d1290, fn_005d1290(ScriptArgs) -> bool),
        entry!(0x005d1340, script_get_weapon_health_function(ScriptArgs) -> bool),
        entry!(0x005d13e0, fn_005d13e0(ScriptArgs) -> bool),
        entry!(0x005d14d0, fn_005d14d0(Ptr, u8)),
        entry!(0x005d1530, fn_005d1530(Ptr, u8)),
        entry!(0x005d1590, fn_005d1590(Ptr, u8)),
        entry!(0x005d15b0, fn_005d15b0(ScriptArgs) -> bool),
        entry!(0x005d15d0, fn_005d15d0(ScriptArgs) -> bool),
        entry!(0x005d1720, fn_005d1720(Ptr, u32, u32)),
        entry!(0x005d1740, fn_005d1740(Ptr, u32)),
        entry!(0x005d1760, fn_005d1760(ScriptArgs) -> bool),
        entry!(0x005d17f0, fn_005d17f0(ScriptArgs) -> bool),
        entry!(0x005d1870, fn_005d1870(ScriptArgs) -> bool),
        entry!(0x005d1890, fn_005d1890(ScriptArgs) -> bool),
        entry!(0x005d1920, fn_005d1920(ScriptArgs) -> bool),
        entry!(0x005d1a30, fn_005d1a30(ScriptArgs) -> bool),
        entry!(0x005d1a50, fn_005d1a50(ScriptArgs) -> bool),
        entry!(0x005d1a70, fn_005d1a70(ScriptArgs) -> bool),
        entry!(0x005d1a90, fn_005d1a90(ScriptArgs) -> bool),
        entry!(0x005d1ab0, script_set_package_duration_function(ScriptArgs) -> bool),
        entry!(0x005d1b80, script_play_magic_shader(ScriptArgs) -> bool),
        entry!(0x005d1dc0, script_play_magic_effect(ScriptArgs) -> bool),
        entry!(0x005d2130, script_stop_magic_shader(ScriptArgs) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fake virtual functions the tests put into vtables.
    const V_NAME: u32 = 0x0900_0001;
    const V_STAGE: u32 = 0x0900_0002;
    const V_TARGET: u32 = 0x0900_0003;
    const V_COMBAT_TARGET: u32 = 0x0900_0004;

    /// An engine with the pages of the globals these commands touch mapped
    /// and the accessors every command uses replaced by doubles that behave
    /// like the exe's code (see the constants' documentation).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.register(CONSOLE_PRINT, |_, _| Ret::default());
        // `__RTDynamicCast`: every object in these tests is an actor.
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(GET_FORM_ID, |e, a| e.mem.u32(a[0] + 0x0c).into_ret());
        e.register(PUT_NUMERIC_ID_IN_DOUBLE, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_f64(a[1], id as f64);
            Ret::default()
        });
        e.register(EXTRA_DATA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(GET_REFERENCE_NAME, |_, _| 0xaaaa.into_ret());
        e.register(V_NAME, |_, _| 0xbbbb.into_ret());
        e.register(V_STAGE, |_, _| Ret::default());
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
        let player = object(e);
        e.set_global(PLAYER, player);
        player
    }

    /// `ParseParameters` gets: info, data, opcode offset, thisObj,
    /// containing, script, event list, then the address of the local.
    fn assert_parsed(e: &Engine, this_obj: u32) {
        assert_eq!(
            calls(e, PARSE_PARAMETERS)[0][..7],
            [1, 2, 8, this_obj, 0, 5, 6]
        );
    }

    /// The test of a "parse one value, hand it to a condition function"
    /// command: the condition function gets `(thisObj, value, 0, result)`.
    fn check_parse_then_condition(command_address: u32, condition: u32, returns_condition: bool) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[0x77]);
        e.register(condition, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(
            calls(&e, condition),
            vec![vec![this_obj, 0x77, 0, a.result.addr()]]
        );
        if returns_condition {
            // The condition function's AL is the command's.
            e.register(condition, |_, _| false.into_ret());
            assert!(!e.call(command_address, &args![a]).bool());
        }
        // Parameters that do not parse: false, nothing called.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(command_address, &args![a]).bool());
        assert!(calls(&e, condition).is_empty());
    }

    /// The test of a "no arguments, call a condition function" command.
    fn check_condition_only(command_address: u32, condition: u32, returns_condition: bool) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(condition, |_, _| false.into_ret());
        start_log(&mut e);
        let result = e.call(command_address, &args![a]).bool();
        assert_eq!(result, !returns_condition);
        assert_eq!(
            calls(&e, condition),
            vec![vec![this_obj, 0, 0, a.result.addr()]]
        );
        e.register(condition, |_, _| true.into_ret());
        assert!(e.call(command_address, &args![a]).bool());
    }

    #[test]
    fn get_detection_level_passes_the_parsed_actor_to_the_condition_function() {
        check_parse_then_condition(0x005c_d990, GET_DETECTION_LEVEL_CONDITION, true);
    }

    #[test]
    fn fn_005cd9f0_needs_a_reference_and_returns_the_condition_function_s_result() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.register(IS_SWIMMING_CONDITION, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(!e.call(0x005c_d9f0, &args![a]).bool());
        assert!(calls(&e, IS_SWIMMING_CONDITION).is_empty());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        assert!(e.call(0x005c_d9f0, &args![a]).bool());
        assert_eq!(
            calls(&e, IS_SWIMMING_CONDITION),
            vec![vec![this_obj, 0, 0, a.result.addr()]]
        );
        e.register(IS_SWIMMING_CONDITION, |_, _| false.into_ret());
        assert!(!e.call(0x005c_d9f0, &args![a]).bool());
    }

    #[test]
    fn is_actor_detected_reports_the_process_lists_answer() {
        let mut e = engine();
        let actor = object(&mut e);
        e.register(PROCESS_LISTS_IS_ACTOR_DETECTED, |_, _| 1u32.into_ret());
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_da20, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        assert_eq!(
            calls(&e, PROCESS_LISTS_IS_ACTOR_DETECTED),
            vec![vec![PROCESS_LISTS, actor]]
        );
        // No echo flag: nothing printed.
        assert!(calls(&e, CONSOLE_PRINT).is_empty());

        // Echo: the name and the verdict are printed.
        set_echo(&mut e, true);
        assert!(e.call(0x005c_da20, &args![a]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_DETECTED, 0xaaaa]]);
        e.register(PROCESS_LISTS_IS_ACTOR_DETECTED, |_, _| 0u32.into_ret());
        assert!(e.call(0x005c_da20, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT).last().unwrap(),
            &vec![MSG_NOT_DETECTED, 0xaaaa]
        );

        // Not an actor (the cast gives null): nothing happens, the command
        // still succeeds.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005c_da20, &args![a]).bool());
        assert!(calls(&e, PROCESS_LISTS_IS_ACTOR_DETECTED).is_empty());
    }

    /// An actor with a package of the given type, the stubs for the package
    /// commands, and the current package either the package or another one.
    /// Returns `(actor, package, process)`.
    fn package_scene(e: &mut Engine, package_type: i8, current: bool) -> (u32, u32, u32) {
        let package = object_with(e, &[(PACKAGE_NAME_SLOT, V_NAME)]);
        e.mem.set_u8(package + 0x20, package_type as u8);
        let process = object_with(e, &[(PROCESS_SET_STAGE_SLOT, V_STAGE)]);
        let actor = object(e);
        e.mem.set_u32(actor + 0x68, process);
        let current_package = if current { package } else { 0x1234 };
        e.register_double(GET_PACKAGE_SET_AS_CURRENT, move |_, _| {
            current_package.into_ret()
        });
        e.register(GET_PACKAGE_EXTRA, |_, _| Ret::default());
        e.register(GET_PACKAGE_TYPE, |e, a| {
            (e.mem.u8(a[0] + 0x20) as i8 as i32 as u32).into_ret()
        });
        e.register(GET_PROCESS, |e, a| e.mem.u32(a[0] + 0x68).into_ret());
        e.register(SET_PACKAGE_FLAG, |_, _| Ret::default());
        (actor, package, process)
    }

    #[test]
    fn fn_005cdad0_sets_the_package_flag_by_package_type() {
        // Type 2: flag set, the process told stage 2.
        let mut e = engine();
        let (actor, package, process) = package_scene(&mut e, 2, true);
        parse_gives(&mut e, true, &[package]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_dad0, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, SET_PACKAGE_FLAG), vec![vec![package, 1]]);
        assert_eq!(calls(&e, V_STAGE), vec![vec![process, 2]]);

        // Types 7 and 1: only the flag.
        for kind in [7, 1] {
            let (actor, package, _) = package_scene(&mut e, kind, true);
            parse_gives(&mut e, true, &[package]);
            let a = command(&mut e, actor);
            start_log(&mut e);
            assert!(e.call(0x005c_dad0, &args![a]).bool());
            assert_eq!(calls(&e, SET_PACKAGE_FLAG), vec![vec![package, 1]]);
            assert!(calls(&e, V_STAGE).is_empty());
        }

        // Another type: reported, nothing set.
        let (actor, package, _) = package_scene(&mut e, 5, true);
        parse_gives(&mut e, true, &[package]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_dad0, &args![a]).bool());
        assert!(calls(&e, SET_PACKAGE_FLAG).is_empty());
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_NOT_FOLLOW_OR_ESCORT, 0xbbbb]]
        );

        // Not the actor's current package, nor the one in its extra data:
        // reported with both names, nothing set.
        let (actor, package, _) = package_scene(&mut e, 2, false);
        parse_gives(&mut e, true, &[package]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_dad0, &args![a]).bool());
        assert!(calls(&e, SET_PACKAGE_FLAG).is_empty());
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_PACKAGE_NOT_CURRENT, 0xbbbb, 0xaaaa]]
        );

        // The package in the actor's extra data counts as current.
        let (actor, package, _) = package_scene(&mut e, 2, false);
        e.register_double(GET_PACKAGE_EXTRA, move |_, _| package.into_ret());
        parse_gives(&mut e, true, &[package]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_dad0, &args![a]).bool());
        assert_eq!(calls(&e, SET_PACKAGE_FLAG), vec![vec![package, 1]]);

        // No package, or parameters that do not parse.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005c_dad0, &args![a]).bool());
        assert!(calls(&e, SET_PACKAGE_FLAG).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_dad0, &args![a]).bool());
    }

    /// The doubles `fn_005cdc10` needs for its follower check; the actor's
    /// target is `target`.
    fn follower_scene(e: &mut Engine, target: u32, teammate: bool, count: i32, limit: i32) -> u32 {
        let player = set_player(e);
        e.register_double(V_TARGET, move |_, _| target.into_ret());
        e.register_double(PLAYER_HAS_TEAMMATE, move |_, _| teammate.into_ret());
        e.register_double(PLAYER_TEAMMATE_COUNT, move |_, _| (count as u32).into_ret());
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, limit as u32);
        e.register_double(GET_SETTING_INTEGER, move |_, _| cell.into_ret());
        e.register(BS_STRING_TEXT, |_, _| 0x7777.into_ret());
        e.register(SHOW_MESSAGE, |_, _| Ret::default());
        e.set_global(MESSAGE_DURATION, 2.5f32);
        player
    }

    #[test]
    fn fn_005cdc10_clears_the_package_flag_unless_the_player_has_too_many_followers() {
        // Type 2: flag cleared, the process told stage 3.
        let mut e = engine();
        let (actor, package, process) = package_scene(&mut e, 2, true);
        parse_gives(&mut e, true, &[package]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_dc10, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, SET_PACKAGE_FLAG), vec![vec![package, 0]]);
        assert_eq!(calls(&e, V_STAGE), vec![vec![process, 3]]);

        // Type 7 with another target: the flag is cleared.
        let (actor, package, process) = package_scene(&mut e, 7, true);
        let vtable = e.mem.alloc(0x800);
        e.mem.set_u32(vtable + PROCESS_GET_TARGET_SLOT, V_TARGET);
        e.mem.set_u32(process, vtable);
        let player = follower_scene(&mut e, 0x5555, false, 9, 3);
        parse_gives(&mut e, true, &[package]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_dc10, &args![a]).bool());
        assert_eq!(calls(&e, SET_PACKAGE_FLAG), vec![vec![package, 0]]);
        assert!(calls(&e, SHOW_MESSAGE).is_empty());

        // The target is the player, who has no room for another follower:
        // the message is shown, the flag stays.
        let player_target = player;
        follower_scene(&mut e, player_target, false, 4, 3);
        start_log(&mut e);
        assert!(e.call(0x005c_dc10, &args![a]).bool());
        assert!(calls(&e, SET_PACKAGE_FLAG).is_empty());
        assert_eq!(
            calls(&e, PLAYER_HAS_TEAMMATE),
            vec![vec![player_target, actor]]
        );
        assert_eq!(
            calls(&e, SHOW_MESSAGE),
            vec![vec![0x7777, 0, 0, 0, 2.5f32.to_bits(), 0]]
        );
        assert_eq!(
            calls(&e, GET_SETTING_INTEGER),
            vec![vec![FOLLOWER_LIMIT_SETTING]]
        );

        // The count is not above the limit: cleared.
        follower_scene(&mut e, player_target, false, 3, 3);
        start_log(&mut e);
        assert!(e.call(0x005c_dc10, &args![a]).bool());
        assert_eq!(calls(&e, SET_PACKAGE_FLAG), vec![vec![package, 0]]);
        assert!(calls(&e, SHOW_MESSAGE).is_empty());

        // The actor is already a teammate: cleared, no count asked.
        follower_scene(&mut e, player_target, true, 9, 3);
        start_log(&mut e);
        assert!(e.call(0x005c_dc10, &args![a]).bool());
        assert_eq!(calls(&e, SET_PACKAGE_FLAG), vec![vec![package, 0]]);
        assert!(calls(&e, PLAYER_TEAMMATE_COUNT).is_empty());

        // Another type is reported; a package that is not current stops.
        let (actor, package, _) = package_scene(&mut e, 4, true);
        parse_gives(&mut e, true, &[package]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_dc10, &args![a]).bool());
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_NOT_FOLLOW_OR_ESCORT, 0xbbbb]]
        );
        let (actor, package, _) = package_scene(&mut e, 2, false);
        parse_gives(&mut e, true, &[package]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_dc10, &args![a]).bool());
        assert!(calls(&e, SET_PACKAGE_FLAG).is_empty());
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_PACKAGE_NOT_CURRENT, 0xbbbb, 0xaaaa]]
        );
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_dc10, &args![a]).bool());
    }

    #[test]
    fn fn_005cde00_stores_the_players_member_as_an_integer() {
        let mut e = engine();
        let player = set_player(&mut e);
        e.mem.set_u32(player + 0x654, (-5i32) as u32);
        let a = command(&mut e, 0);
        assert!(e.call(0x005c_de00, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), -5.0);
    }

    #[test]
    fn fn_005cde20_reads_the_member_at_0x654() {
        let mut e = engine();
        let player = object(&mut e);
        e.mem.set_u32(player + 0x654, 41);
        assert_eq!(e.call(0x005c_de20, &args![player]).i32(), 41);
    }

    #[test]
    fn fn_005cde40_stores_the_parsed_value_in_the_player() {
        let mut e = engine();
        let player = set_player(&mut e);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[7]);
        e.register(PLAYER_SET_FIELD_654, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005c_de40, &args![a]).bool());
        assert_eq!(calls(&e, PLAYER_SET_FIELD_654), vec![vec![player, 7, 1]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_de40, &args![a]).bool());
        assert!(calls(&e, PLAYER_SET_FIELD_654).is_empty());
    }

    #[test]
    fn fn_005cdea0_calls_the_condition_function_and_succeeds() {
        check_condition_only(0x005c_dea0, GET_AMOUNT_STOLEN_SOLD_CONDITION, false);
    }

    #[test]
    fn fn_005cdec0_adds_the_parsed_amount_to_the_players_member() {
        let mut e = engine();
        let player = set_player(&mut e);
        e.mem.set_u32(player + 0x6dc, 10);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[5]);
        assert!(e.call(0x005c_dec0, &args![a]).bool());
        assert_eq!(e.mem.i32(player + 0x6dc), 15);
        parse_gives(&mut e, true, &[(-20i32) as u32]);
        assert!(e.call(0x005c_dec0, &args![a]).bool());
        assert_eq!(e.mem.i32(player + 0x6dc), -5);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_dec0, &args![a]).bool());
        assert_eq!(e.mem.i32(player + 0x6dc), -5);
    }

    #[test]
    fn fn_005cdf20_adds_to_the_member_at_0x6dc() {
        let mut e = engine();
        let player = object(&mut e);
        e.call(0x005c_df20, &args![player, 3i32]);
        e.call(0x005c_df20, &args![player, -1i32]);
        assert_eq!(e.mem.i32(player + 0x6dc), 2);
    }

    #[test]
    fn get_pc_expelled_passes_the_parsed_faction_to_the_condition_function() {
        check_parse_then_condition(0x005c_df50, GET_PC_EXPELLED_CONDITION, false);
    }

    #[test]
    fn set_pc_expelled_sets_the_flag_and_looks_up_the_cell_owner() {
        let mut e = engine();
        let player = set_player(&mut e);
        let a = command(&mut e, 0);
        e.register(FACTION_SET_FLAG_BITS, |_, _| Ret::default());
        e.register(GET_PARENT_CELL, |_, _| 0x3000.into_ret());
        e.register(CELL_GET_OWNER, |_, _| Ret::default());
        // Flag set: the expelled bit set, the owner of the cell asked.
        parse_gives(&mut e, true, &[0x66, 1]);
        start_log(&mut e);
        assert!(e.call(0x005c_dfb0, &args![a]).bool());
        assert_eq!(calls(&e, FACTION_SET_FLAG_BITS), vec![vec![0x66, 8, 1]]);
        assert_eq!(calls(&e, GET_PARENT_CELL), vec![vec![player]]);
        assert_eq!(calls(&e, CELL_GET_OWNER), vec![vec![0x3000]]);
        // No parent cell: no owner lookup.
        e.register(GET_PARENT_CELL, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005c_dfb0, &args![a]).bool());
        assert!(calls(&e, CELL_GET_OWNER).is_empty());
        // Flag clear: the bit cleared, no cell lookup.
        parse_gives(&mut e, true, &[0x66, 0]);
        start_log(&mut e);
        assert!(e.call(0x005c_dfb0, &args![a]).bool());
        assert_eq!(calls(&e, FACTION_SET_FLAG_BITS), vec![vec![0x66, 8, 0]]);
        assert!(calls(&e, GET_PARENT_CELL).is_empty());
        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_dfb0, &args![a]).bool());
        assert!(calls(&e, FACTION_SET_FLAG_BITS).is_empty());
    }

    #[test]
    fn fn_005ce040_sets_the_expelled_bit_of_the_faction() {
        let mut e = engine();
        e.register(FACTION_SET_FLAG_BITS, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x005c_e040, &args![0x66u32, 1u8]);
        e.call(0x005c_e040, &args![0x66u32, 0u8]);
        assert_eq!(
            calls(&e, FACTION_SET_FLAG_BITS),
            vec![vec![0x66, 8, 1], vec![0x66, 8, 0]]
        );
    }

    #[test]
    fn get_pc_faction_murder_passes_the_parsed_faction_to_the_condition_function() {
        check_parse_then_condition(0x005c_e060, GET_PC_FACTION_MURDER_CONDITION, false);
    }

    /// The test of a faction flag setter command: a faction and a flag are
    /// parsed and the setter gets `(faction, flag != 0)`.
    fn check_faction_flag_setter(command_address: u32, setter: u32) {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.register(setter, |_, _| Ret::default());
        parse_gives(&mut e, true, &[0x66, 9]);
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_parsed(&e, 0);
        assert_eq!(calls(&e, setter), vec![vec![0x66, 1]]);
        parse_gives(&mut e, true, &[0x66, 0]);
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_eq!(calls(&e, setter), vec![vec![0x66, 0]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(command_address, &args![a]).bool());
        assert!(calls(&e, setter).is_empty());
    }

    #[test]
    fn fn_005ce0c0_sets_a_faction_flag() {
        check_faction_flag_setter(0x005c_e0c0, FACTION_SET_MURDER_FLAG);
    }

    #[test]
    fn get_player_enemyof_faction_passes_the_parsed_faction_to_the_condition_function() {
        check_parse_then_condition(0x005c_e130, GET_PLAYER_ENEMY_OF_FACTION_CONDITION, false);
    }

    #[test]
    fn fn_005ce190_sets_a_faction_flag() {
        check_faction_flag_setter(0x005c_e190, FACTION_SET_ENEMY_FLAG);
    }

    #[test]
    fn get_pc_faction_attack_passes_the_parsed_faction_to_the_condition_function() {
        check_parse_then_condition(0x005c_e200, GET_PC_FACTION_ATTACK_CONDITION, false);
    }

    #[test]
    fn fn_005ce260_sets_a_faction_flag() {
        check_faction_flag_setter(0x005c_e260, FACTION_SET_ATTACK_FLAG);
    }

    #[test]
    fn fn_005ce2d0_calls_the_condition_function_and_succeeds() {
        check_condition_only(0x005c_e2d0, GET_DESTROYED_CONDITION, false);
    }

    #[test]
    fn fn_005ce2f0_sets_form_flag_0x800000_on_the_reference() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(FORM_SET_FLAG_800000, |_, _| Ret::default());
        parse_gives(&mut e, true, &[3]);
        start_log(&mut e);
        assert!(e.call(0x005c_e2f0, &args![a]).bool());
        assert_eq!(calls(&e, FORM_SET_FLAG_800000), vec![vec![this_obj, 1]]);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005c_e2f0, &args![a]).bool());
        assert_eq!(calls(&e, FORM_SET_FLAG_800000), vec![vec![this_obj, 0]]);
        // Without a reference: succeeds, nothing set.
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_e2f0, &args![none]).bool());
        assert!(calls(&e, FORM_SET_FLAG_800000).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_e2f0, &args![a]).bool());
    }

    /// A form with the given id at `+0x0c`.
    fn form_with_id(e: &mut Engine, id: u32) -> u32 {
        let form = object(e);
        e.mem.set_u32(form + 0x0c, id);
        form
    }

    #[test]
    fn get_action_ref_returns_the_id_of_the_action_reference() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        // No action reference: 0.
        e.register(GET_ACTION_REF, |_, _| Ret::default());
        assert!(e.call(0x005c_e360, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        // With one: its id, echoed when the flag is set.
        let action = form_with_id(&mut e, 0x1234);
        e.register_double(GET_ACTION_REF, move |_, _| action.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005c_e360, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0x1234 as f64);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005c_e360, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_ACTION_REF, 0x1234]]
        );
        // No reference: result 0, echo of 0, success.
        let none = command(&mut e, 0);
        e.mem.set_f64(none.result.addr(), 5.0);
        start_log(&mut e);
        assert!(e.call(0x005c_e360, &args![none]).bool());
        assert_eq!(e.mem.f64(none.result.addr()), 0.0);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_GET_ACTION_REF, 0]]);
    }

    #[test]
    fn get_self_returns_the_id_unless_a_non_persistent_container_item() {
        let mut e = engine();
        let base = object(&mut e);
        let this_obj = form_with_id(&mut e, 0x4321);
        e.mem.set_u32(this_obj + 0x20, base);
        e.mem.set_u8(base + 4, 0x28);
        e.register(GET_REF_PERSISTS, |_, _| true.into_ret());
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(FORM_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
        e.register(CONTAINER_CAN_HOLD_TYPE, |_, a| (a[0] == 0x28).into_ret());
        let a = command(&mut e, this_obj);
        // Persistent: always reported.
        assert!(e.call(0x005c_e3e0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0x4321 as f64);
        // Not persistent and a container item type: stays 0.
        e.register(GET_REF_PERSISTS, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005c_e3e0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert_eq!(calls(&e, CONTAINER_CAN_HOLD_TYPE), vec![vec![0x28]]);
        // Not persistent, not a container item type: reported and echoed.
        e.mem.set_u8(base + 4, 0x10);
        set_echo(&mut e, true);
        assert!(e.call(0x005c_e3e0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0x4321 as f64);
        assert_eq!(
            calls(&e, CONSOLE_PRINT).last().unwrap(),
            &vec![MSG_GET_SELF, 0x4321]
        );
        // No reference.
        let none = command(&mut e, 0);
        assert!(e.call(0x005c_e3e0, &args![none]).bool());
        assert_eq!(e.mem.f64(none.result.addr()), 0.0);
    }

    #[test]
    fn get_combat_target_returns_the_id_of_the_actors_target() {
        let mut e = engine();
        let target = form_with_id(&mut e, 0x99);
        let actor = object_with(&mut e, &[(ACTOR_GET_COMBAT_TARGET_SLOT, V_COMBAT_TARGET)]);
        e.register_double(V_COMBAT_TARGET, move |_, _| target.into_ret());
        let a = command(&mut e, actor);
        start_log(&mut e);
        set_echo(&mut e, true);
        assert!(e.call(0x005c_e480, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0x99 as f64);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_COMBAT_TARGET, 0x99]]
        );
        // No target: 0.
        e.register(V_COMBAT_TARGET, |_, _| Ret::default());
        assert!(e.call(0x005c_e480, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        // Not an actor: 0, success.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        e.mem.set_f64(a.result.addr(), 3.0);
        assert!(e.call(0x005c_e480, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
    }

    #[test]
    fn get_package_target_returns_the_id_of_the_current_package_target() {
        let mut e = engine();
        let target = form_with_id(&mut e, 0x55);
        let actor = object(&mut e);
        e.register_double(GET_CURRENT_PACKAGE_TARGET, move |_, _| target.into_ret());
        let a = command(&mut e, actor);
        start_log(&mut e);
        set_echo(&mut e, true);
        assert!(e.call(0x005c_e520, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0x55 as f64);
        assert_eq!(calls(&e, GET_CURRENT_PACKAGE_TARGET), vec![vec![actor]]);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_PACKAGE_TARGET, 0x55]]
        );
        e.register(GET_CURRENT_PACKAGE_TARGET, |_, _| Ret::default());
        assert!(e.call(0x005c_e520, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x005c_e520, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
    }

    #[test]
    fn get_container_returns_the_id_of_the_containing_object() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let container = form_with_id(&mut e, 0x77);
        let a = ScriptArgs {
            containing_obj: Ptr::new(container),
            ..command(&mut e, this_obj)
        };
        start_log(&mut e);
        set_echo(&mut e, true);
        assert!(e.call(0x005c_e5c0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0x77 as f64);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_CONTAINER, 0x77]]
        );
        // Without a containing object, or without a reference: 0.
        let a = command(&mut e, this_obj);
        e.mem.set_f64(a.result.addr(), 7.0);
        assert!(e.call(0x005c_e5c0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        let a = ScriptArgs {
            containing_obj: Ptr::new(container),
            ..command(&mut e, 0)
        };
        assert!(e.call(0x005c_e5c0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
    }

    #[test]
    fn get_parent_ref_returns_the_id_of_the_parent_reference() {
        let mut e = engine();
        let parent = form_with_id(&mut e, 0x31);
        let this_obj = object(&mut e);
        e.register_double(GET_PARENT_REF, move |_, _| parent.into_ret());
        let a = command(&mut e, this_obj);
        start_log(&mut e);
        set_echo(&mut e, true);
        assert!(e.call(0x005c_e630, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0x31 as f64);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_PARENT_REF, 0x31]]
        );
        // The lookup is made twice, as the game does.
        assert_eq!(calls(&e, GET_PARENT_REF).len(), 2);
        e.register(GET_PARENT_REF, |_, _| Ret::default());
        assert!(e.call(0x005c_e630, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        let none = command(&mut e, 0);
        assert!(e.call(0x005c_e630, &args![none]).bool());
        assert_eq!(e.mem.f64(none.result.addr()), 0.0);
    }

    #[test]
    fn get_linked_ref_returns_the_id_of_the_linked_reference() {
        let mut e = engine();
        let linked = form_with_id(&mut e, 0x41);
        let this_obj = object(&mut e);
        e.register_double(GET_LINKED_REF, move |_, _| linked.into_ret());
        let a = command(&mut e, this_obj);
        start_log(&mut e);
        set_echo(&mut e, true);
        assert!(e.call(0x005c_e6b0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0x41 as f64);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_LINKED_REF, 0x41]]
        );
        e.register(GET_LINKED_REF, |_, _| Ret::default());
        assert!(e.call(0x005c_e6b0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        let none = command(&mut e, 0);
        assert!(e.call(0x005c_e6b0, &args![none]).bool());
        assert_eq!(e.mem.f64(none.result.addr()), 0.0);
    }

    #[test]
    fn get_force_run_reports_the_actors_flag() {
        let mut e = engine();
        let actor = object(&mut e);
        e.register(ACTOR_GET_FORCE_RUN, |_, _| true.into_ret());
        let a = command(&mut e, actor);
        start_log(&mut e);
        set_echo(&mut e, true);
        assert!(e.call(0x005c_e730, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        // The echo passes the double as two words.
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_FORCE_RUN, 0, 0x3ff0_0000]]
        );
        e.register(ACTOR_GET_FORCE_RUN, |_, _| false.into_ret());
        assert!(e.call(0x005c_e730, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        // Not an actor: 0.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        e.mem.set_f64(a.result.addr(), 4.0);
        set_echo(&mut e, false);
        assert!(e.call(0x005c_e730, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
    }

    #[test]
    fn set_force_run_stores_the_parsed_flag() {
        let mut e = engine();
        let actor = object(&mut e);
        e.register(ACTOR_SET_FORCE_RUN, |_, _| Ret::default());
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        set_echo(&mut e, true);
        assert!(e.call(0x005c_e7b0, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        assert_eq!(calls(&e, ACTOR_SET_FORCE_RUN), vec![vec![actor, 1]]);
        // The raw integer goes to the echo.
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_SET_FORCE_RUN, 5]]);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        set_echo(&mut e, false);
        assert!(e.call(0x005c_e7b0, &args![a]).bool());
        assert_eq!(calls(&e, ACTOR_SET_FORCE_RUN), vec![vec![actor, 0]]);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // Parameters that do not parse: false, nothing stored.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_e7b0, &args![a]).bool());
        assert!(calls(&e, ACTOR_SET_FORCE_RUN).is_empty());
        // Not an actor: success without parsing.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005c_e7b0, &args![a]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
    }

    #[test]
    fn get_force_sneak_reports_the_actors_flag() {
        let mut e = engine();
        let actor = object(&mut e);
        e.register(ACTOR_GET_FORCE_SNEAK, |_, _| true.into_ret());
        let a = command(&mut e, actor);
        start_log(&mut e);
        set_echo(&mut e, true);
        assert!(e.call(0x005c_e870, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_FORCE_SNEAK, 0, 0x3ff0_0000]]
        );
        e.register(ACTOR_GET_FORCE_SNEAK, |_, _| false.into_ret());
        assert!(e.call(0x005c_e870, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        e.mem.set_f64(a.result.addr(), 4.0);
        assert!(e.call(0x005c_e870, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
    }

    #[test]
    fn set_force_sneak_stores_the_parsed_flag_in_the_actor() {
        let mut e = engine();
        let actor = object(&mut e);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        set_echo(&mut e, true);
        assert!(e.call(0x005c_e910, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        assert_eq!(e.mem.u8(actor + 0x125), 1);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_SET_FORCE_SNEAK, 5]]);
        parse_gives(&mut e, true, &[0]);
        assert!(e.call(0x005c_e910, &args![a]).bool());
        assert_eq!(e.mem.u8(actor + 0x125), 0);
        parse_gives(&mut e, false, &[]);
        e.mem.set_u8(actor + 0x125, 1);
        assert!(!e.call(0x005c_e910, &args![a]).bool());
        assert_eq!(e.mem.u8(actor + 0x125), 1);
        // Not an actor: success without parsing.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005c_e910, &args![a]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
    }

    #[test]
    fn fn_005ce9d0_stores_the_byte_at_0x125() {
        let mut e = engine();
        let actor = object(&mut e);
        e.call(0x005c_e9d0, &args![actor, 1u8]);
        assert_eq!(e.mem.u8(actor + 0x125), 1);
        e.call(0x005c_e9d0, &args![actor, 0u8]);
        assert_eq!(e.mem.u8(actor + 0x125), 0);
    }

    #[test]
    fn fn_005ce9f0_raises_the_players_level_and_opens_the_level_up_menu() {
        let mut e = engine();
        // Without a player: nothing, but success.
        e.register(PLAYER_GET_LEVEL, |_, _| 0xffff_0004u32.into_ret());
        e.register(GET_BASE_FORM_OF_REFERENCE, |_, _| 0x6000.into_ret());
        e.register(ACTOR_BASE_DATA_SET_LEVEL, |_, _| Ret::default());
        e.register(CREATE_LEVEL_UP_MENU, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005c_e9f0, &args![]).bool());
        assert!(calls(&e, CREATE_LEVEL_UP_MENU).is_empty());
        // With one: the 16-bit level plus one goes to the base data.
        let player = set_player(&mut e);
        assert!(e.call(0x005c_e9f0, &args![]).bool());
        assert_eq!(calls(&e, PLAYER_GET_LEVEL), vec![vec![player]]);
        assert_eq!(calls(&e, ACTOR_BASE_DATA_SET_LEVEL), vec![vec![0x6030, 5]]);
        assert_eq!(calls(&e, CREATE_LEVEL_UP_MENU).len(), 1);
    }

    #[test]
    fn has_magic_effect_passes_the_parsed_effect_to_the_condition_function() {
        check_parse_then_condition(0x005c_ea30, HAS_MAGIC_EFFECT_CONDITION, true);
    }

    #[test]
    fn is_spell_target_passes_the_parsed_spell_to_the_condition_function() {
        check_parse_then_condition(0x005c_ea90, IS_SPELL_TARGET_CONDITION, true);
    }

    #[test]
    fn get_spell_usage_number_passes_the_parsed_spell_to_the_condition_function() {
        check_parse_then_condition(0x005c_eaf0, GET_SPELL_USAGE_NUMBER_CONDITION, true);
    }

    #[test]
    fn fn_005ceb50_returns_the_condition_function_s_result() {
        check_condition_only(0x005c_eb50, GET_VATS_MODE_CONDITION, true);
    }

    #[test]
    fn fn_005ceb70_returns_the_condition_function_s_result() {
        check_condition_only(0x005c_eb70, GET_VATS_TARGET_HEIGHT_CONDITION, true);
    }

    // ---- Second batch ------------------------------------------------------------

    // Fake virtual functions of the second batch.
    const V_FALSE: u32 = 0x0900_0020;
    const V_TRUE: u32 = 0x0900_0021;
    const V_GET_3D: u32 = 0x0900_0022;
    const V_RECORD_A: u32 = 0x0900_0023;
    const V_RECORD_B: u32 = 0x0900_0024;
    const V_FIND_NODE: u32 = 0x0900_0025;
    const V_SET_TEXTURE: u32 = 0x0900_0026;
    const V_PROCESS: u32 = 0x0900_0027;
    const V_PROCESS_B: u32 = 0x0900_0028;
    const V_PROCESS_C: u32 = 0x0900_0029;
    const V_PACKAGE_TARGET: u32 = 0x0900_002a;
    const V_PROCESS_PACKAGE: u32 = 0x0900_002b;
    const V_ACTOR_RECORD: u32 = 0x0900_002c;

    /// [`engine`] with the pages of the globals of the second batch mapped and
    /// the fake virtual functions registered.
    fn engine2() -> Engine {
        let mut e = engine();
        e.map(0x011d_d000, 0x1000);
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.register(V_FALSE, |_, _| false.into_ret());
        e.register(V_TRUE, |_, _| true.into_ret());
        e.register(V_RECORD_A, |_, _| Ret::default());
        e.register(V_RECORD_B, |_, _| Ret::default());
        e.register(V_ACTOR_RECORD, |_, _| Ret::default());
        e.register(V_PROCESS, |_, _| Ret::default());
        e
    }

    /// The two words of a `double` argument.
    fn double_words(value: f64) -> Vec<u32> {
        let bits = value.to_bits();
        vec![bits as u32, (bits >> 32) as u32]
    }

    /// A `ScriptArgs` for a command with the standard words and a running
    /// script object `script`.
    fn command_with_script(e: &mut Engine, this_obj: u32, script: u32) -> ScriptArgs {
        let mut a = command(e, this_obj);
        a.script_obj = Ptr::new(script);
        a
    }

    #[test]
    fn get_default_open_echoes_the_result_when_the_condition_function_succeeds() {
        let mut e = engine2();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(GET_DEFAULT_OPEN_CONDITION, |e, a| {
            e.mem.set_f64(a[3], 1.0);
            true.into_ret()
        });
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005c_eb90, &args![a]).bool());
        assert_eq!(
            calls(&e, GET_DEFAULT_OPEN_CONDITION),
            vec![vec![this_obj, 0, 0, a.result.addr()]]
        );
        let mut expected = vec![MSG_GET_DEFAULT_OPEN];
        expected.extend(double_words(1.0));
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![expected]);
        // No echo flag: nothing printed.
        set_echo(&mut e, false);
        start_log(&mut e);
        assert!(e.call(0x005c_eb90, &args![a]).bool());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // The condition function fails: the command fails, silently.
        e.register(GET_DEFAULT_OPEN_CONDITION, |_, _| false.into_ret());
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(!e.call(0x005c_eb90, &args![a]).bool());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    /// Doubles for the reference accessors of the default-open commands: the
    /// action bits live in the byte at `reference + 0x10`.
    fn action_bits_doubles(e: &mut Engine) {
        e.register(REFERENCE_TEST_ACTION_BITS, |e, a| {
            ((e.mem.u8(a[0] + 0x10) as u32 & a[1]) != 0).into_ret()
        });
        e.register(REFERENCE_SET_ACTION_BITS, |e, a| {
            let bits = e.mem.u8(a[0] + 0x10) | a[1] as u8;
            e.mem.set_u8(a[0] + 0x10, bits);
            Ret::default()
        });
        e.register(REFERENCE_CLEAR_ACTION_BITS, |e, a| {
            let bits = e.mem.u8(a[0] + 0x10) & !(a[1] as u8);
            e.mem.set_u8(a[0] + 0x10, bits);
            Ret::default()
        });
        e.register(REMOVE_SAVED_ANIMATION, |_, _| Ret::default());
        e.register(REMOVE_LAST_FINISHED_SEQUENCE, |_, _| Ret::default());
        e.register(CHANGE_FLAGS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(SET_OPEN_STATE, |_, _| Ret::default());
    }

    #[test]
    fn set_default_open_sets_the_bit_records_the_change_and_the_open_state() {
        let mut e = engine2();
        action_bits_doubles(&mut e);
        e.register(SAVE_LOAD_GET_CHANGE, |_, _| false.into_ret());
        e.set_global(SAVE_LOAD_GAME, 0x5555u32);
        let reference = object_with(&mut e, &[(0x48, V_RECORD_A), (0x4c, V_RECORD_B)]);
        let a = command(&mut e, reference);
        parse_gives(&mut e, true, &[1]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005c_ebf0, &args![a]).bool());
        assert_parsed(&e, reference);
        // The bit was clear and is set: change flags through slot 0x48.
        assert_eq!(e.mem.u8(reference + 0x10), 8);
        assert_eq!(
            calls(&e, SAVE_LOAD_GET_CHANGE),
            vec![vec![0x5555, reference, 0x0040_0000]]
        );
        assert_eq!(calls(&e, V_RECORD_A), vec![vec![reference, 0x0040_0000]]);
        assert!(calls(&e, V_RECORD_B).is_empty());
        assert_eq!(calls(&e, SET_OPEN_STATE), vec![vec![reference, 1, 1]]);
        assert_eq!(
            calls(&e, REMOVE_SAVED_ANIMATION),
            vec![vec![reference + 0x44]]
        );
        assert_eq!(
            calls(&e, REMOVE_LAST_FINISHED_SEQUENCE),
            vec![vec![reference + 0x44]]
        );
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_SET_DEFAULT_OPEN, 1]]
        );
        // The bit does not change: no change flags, the open state is set anyway.
        start_log(&mut e);
        assert!(e.call(0x005c_ebf0, &args![a]).bool());
        assert!(calls(&e, SAVE_LOAD_GET_CHANGE).is_empty());
        assert_eq!(calls(&e, SET_OPEN_STATE), vec![vec![reference, 1, 1]]);
        // Clearing the bit with a change already known: slot 0x4c.
        parse_gives(&mut e, true, &[0]);
        e.register(SAVE_LOAD_GET_CHANGE, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005c_ebf0, &args![a]).bool());
        assert_eq!(e.mem.u8(reference + 0x10), 0);
        assert_eq!(calls(&e, V_RECORD_B), vec![vec![reference, 0x0040_0000]]);
        assert!(calls(&e, V_RECORD_A).is_empty());
        assert_eq!(calls(&e, SET_OPEN_STATE), vec![vec![reference, 0, 1]]);
        // No reference: only the echo.
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_ebf0, &args![none]).bool());
        assert!(calls(&e, SET_OPEN_STATE).is_empty());
        assert_eq!(calls(&e, CONSOLE_PRINT).len(), 1);
        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_ebf0, &args![a]).bool());
        assert!(calls(&e, SET_OPEN_STATE).is_empty());
    }

    #[test]
    fn set_open_state_activates_only_when_the_state_differs() {
        let mut e = engine2();
        e.register(GET_OPEN_STATE, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(REFERENCE_ACTIVATE, |_, _| Ret::default());
        e.register(SET_LAST_FINISHED_SEQUENCE, |_, _| Ret::default());
        e.register(REMOVE_SAVED_ANIMATION, |_, _| Ret::default());
        let reference = object(&mut e);
        let a = command(&mut e, reference);
        // (word, state, activates, sequence)
        for (word, state, activates, sequence) in [
            (1u32, 3u32, true, SEQUENCE_OPEN),
            (1, 2, false, SEQUENCE_OPEN),
            (0, 2, true, SEQUENCE_CLOSE),
            (0, 3, false, SEQUENCE_CLOSE),
        ] {
            parse_gives(&mut e, true, &[word]);
            e.mem.set_u32(reference + 0x10, state);
            start_log(&mut e);
            assert!(e.call(0x005c_ed30, &args![a]).bool());
            assert_parsed(&e, reference);
            let activations = calls(&e, REFERENCE_ACTIVATE);
            if activates {
                assert_eq!(activations, vec![vec![reference, 0, 0, 0, 1]]);
            } else {
                assert!(activations.is_empty());
            }
            assert_eq!(
                calls(&e, SET_LAST_FINISHED_SEQUENCE),
                vec![vec![reference, sequence]]
            );
            assert_eq!(
                calls(&e, REMOVE_SAVED_ANIMATION),
                vec![vec![reference + 0x44]]
            );
        }
        // No reference: succeeds without touching anything.
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_ed30, &args![none]).bool());
        assert!(calls(&e, GET_OPEN_STATE).is_empty());
        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_ed30, &args![a]).bool());
    }

    /// The test of the race/sex menu commands: nothing without a player, else
    /// `CreateRaceSexMenu` gets the modes in order.
    fn check_race_sex_menu(address: u32, echo: &[bool], modes: &[u32]) {
        let mut e = engine2();
        e.register(CREATE_RACE_SEX_MENU, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(address, &args![]).bool());
        assert!(calls(&e, CREATE_RACE_SEX_MENU).is_empty());
        set_player(&mut e);
        for on in echo {
            set_echo(&mut e, *on);
            assert!(e.call(address, &args![]).bool());
        }
        let expected: Vec<Vec<u32>> = modes.iter().map(|mode| vec![*mode]).collect();
        assert_eq!(calls(&e, CREATE_RACE_SEX_MENU), expected);
    }

    #[test]
    fn fn_005cedf0_opens_the_race_sex_menu_in_mode_0_or_1() {
        check_race_sex_menu(0x005c_edf0, &[true, false], &[0, 1]);
    }

    #[test]
    fn fn_005cee30_opens_the_race_sex_menu_in_mode_2() {
        check_race_sex_menu(0x005c_ee30, &[false], &[2]);
    }

    #[test]
    fn fn_005cee50_opens_the_race_sex_menu_in_mode_3() {
        check_race_sex_menu(0x005c_ee50, &[false], &[3]);
    }

    #[test]
    fn fn_005cee70_hands_the_eight_words_on_when_there_is_a_player() {
        let mut e = engine2();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(CLOSE_CONSOLE_AND_QUEUE_MENU, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005c_ee70, &args![a]).bool());
        assert!(calls(&e, CLOSE_CONSOLE_AND_QUEUE_MENU).is_empty());
        set_player(&mut e);
        // The callee's AL is the result.
        assert!(!e.call(0x005c_ee70, &args![a]).bool());
        e.register(CLOSE_CONSOLE_AND_QUEUE_MENU, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005c_ee70, &args![a]).bool());
        assert_eq!(
            calls(&e, CLOSE_CONSOLE_AND_QUEUE_MENU),
            vec![vec![
                a.param_info,
                a.script_data,
                this_obj,
                0,
                5,
                6,
                a.result.addr(),
                8
            ]]
        );
    }

    /// Doubles for the lookups of [`menu_target_of`]: the base form of a
    /// reference is the word at `+0x20`, the speaker of an activator the word
    /// at `+0x90`, the member read by `005e3fa0` the word at `+0x6c`.
    fn menu_target_doubles(e: &mut Engine) {
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(TALKING_ACTIVATOR_GET_SPEAKER, |e, a| {
            e.mem.u32(a[0] + 0x90).into_ret()
        });
        e.register(REFERENCE_GET_FIELD_6C, |e, a| {
            e.mem.u32(a[0] + 0x6c).into_ret()
        });
        e.register(REFERENCE_GET_BYTE_81, |e, a| {
            e.mem.u8(a[0] + 0x81).into_ret()
        });
        // `__RTDynamicCast`: an object is a talking activator when it has the
        // word 0xac71 at +0x94.
        e.register(DYNAMIC_CAST, |e, a| {
            if a[0] != 0 && e.mem.u32(a[0] + 0x94) == 0xac71 {
                a[0].into_ret()
            } else {
                Ret::default()
            }
        });
    }

    /// The reference kinds [`menu_target_of`] distinguishes: `(mobile object,
    /// actor)` virtual answers.
    fn menu_reference(e: &mut Engine, mobile: bool, actor: bool) -> u32 {
        object_with(
            e,
            &[
                (0xfc, if mobile { V_TRUE } else { V_FALSE }),
                (0x100, if actor { V_TRUE } else { V_FALSE }),
            ],
        )
    }

    #[test]
    fn fn_005ceeb0_opens_the_barter_menu_on_the_menu_target() {
        let mut e = engine2();
        menu_target_doubles(&mut e);
        e.register(BARTER_MENU_CREATE, |_, _| Ret::default());
        let player = set_player(&mut e);
        parse_gives(&mut e, true, &[7]);
        // A reference whose base form is a talking activator: its speaker.
        let activator = object(&mut e);
        e.mem.set_u32(activator + 0x94, 0xac71);
        e.mem.set_u32(activator + 0x90, 0x3333);
        let object_ref = menu_reference(&mut e, false, false);
        e.mem.set_u32(object_ref + 0x20, activator);
        let a = command(&mut e, object_ref);
        start_log(&mut e);
        assert!(e.call(0x005c_eeb0, &args![a]).bool());
        assert_eq!(calls(&e, BARTER_MENU_CREATE), vec![vec![0x3333, 7]]);
        // A base form that is not an activator: nothing.
        let other = object(&mut e);
        e.mem.set_u32(object_ref + 0x20, other);
        start_log(&mut e);
        assert!(e.call(0x005c_eeb0, &args![a]).bool());
        assert!(calls(&e, BARTER_MENU_CREATE).is_empty());
        // An actor is its own target.
        let actor = menu_reference(&mut e, true, true);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_eeb0, &args![a]).bool());
        assert_eq!(calls(&e, BARTER_MENU_CREATE), vec![vec![actor, 7]]);
        // A mobile object that is no actor: through the member at +0x6c when
        // the byte at +0x81 is set.
        let creature = menu_reference(&mut e, true, false);
        let member = object(&mut e);
        let member_base = object(&mut e);
        e.mem.set_u32(creature + 0x6c, member);
        e.mem.set_u32(member + 0x20, member_base);
        e.mem.set_u32(member_base + 0x90, 0x4444);
        let a = command(&mut e, creature);
        start_log(&mut e);
        assert!(e.call(0x005c_eeb0, &args![a]).bool());
        assert!(calls(&e, BARTER_MENU_CREATE).is_empty());
        e.mem.set_u8(creature + 0x81, 1);
        assert!(e.call(0x005c_eeb0, &args![a]).bool());
        assert_eq!(calls(&e, BARTER_MENU_CREATE), vec![vec![0x4444, 7]]);
        // The player and no reference: nothing, and the parse is still made.
        for this_obj in [player, 0] {
            let a = command(&mut e, this_obj);
            start_log(&mut e);
            assert!(e.call(0x005c_eeb0, &args![a]).bool());
            assert!(calls(&e, BARTER_MENU_CREATE).is_empty());
            assert_eq!(calls(&e, PARSE_PARAMETERS).len(), 1);
        }
        // Parameters that do not parse: the amount stays 0 and the command goes on.
        parse_gives(&mut e, false, &[]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_eeb0, &args![a]).bool());
        assert_eq!(calls(&e, BARTER_MENU_CREATE), vec![vec![actor, 0]]);
    }

    #[test]
    fn show_special_book_menu_picks_the_variant_by_the_parse() {
        let mut e = engine2();
        e.register(SPECIAL_BOOK_MENU_CREATE, |_, _| Ret::default());
        e.register(SPECIAL_BOOK_MENU_CREATE_OV2, |_, _| Ret::default());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[9]);
        start_log(&mut e);
        // Without a player nothing is parsed.
        assert!(e.call(0x005c_efe0, &args![a]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        set_player(&mut e);
        assert!(e.call(0x005c_efe0, &args![a]).bool());
        assert_eq!(calls(&e, SPECIAL_BOOK_MENU_CREATE_OV2), vec![vec![9]]);
        assert!(calls(&e, SPECIAL_BOOK_MENU_CREATE).is_empty());
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(e.call(0x005c_efe0, &args![a]).bool());
        assert_eq!(calls(&e, SPECIAL_BOOK_MENU_CREATE), vec![vec![]]);
        assert!(calls(&e, SPECIAL_BOOK_MENU_CREATE_OV2).is_empty());
    }

    /// The test of the three gambling-table commands: `fourth` is the word
    /// the menu should get in place of the fourth parsed word.
    fn check_table_menu(
        command_address: u32,
        create: u32,
        invalid_parameters: u32,
        invalid_form: u32,
        passes_fourth: bool,
    ) {
        let mut e = engine2();
        e.register(create, |_, _| Ret::default());
        e.register(LOG_STUB, |_, _| Ret::default());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[0x10, 0x20, 0x30, 0x40]);
        // No player: no parse, no menu, no report.
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        set_player(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        let fourth = if passes_fourth { 0x40 } else { 0 };
        assert_eq!(calls(&e, create), vec![vec![0x10, 0x20, 0x30, fourth, 0]]);
        // A zero form is reported.
        parse_gives(&mut e, true, &[0, 0x20, 0x30, 0x40]);
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert!(calls(&e, create).is_empty());
        assert_eq!(calls(&e, LOG_STUB), vec![vec![invalid_form]]);
        // Parameters that do not parse are reported.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert!(calls(&e, create).is_empty());
        assert_eq!(calls(&e, LOG_STUB), vec![vec![invalid_parameters]]);
    }

    #[test]
    fn show_slot_machine_menu_passes_four_words() {
        check_table_menu(
            0x005c_f040,
            SLOT_MACHINE_MENU_CREATE,
            MSG_SLOT_MACHINE_PARAMETERS,
            MSG_SLOT_MACHINE_FORM,
            true,
        );
    }

    #[test]
    fn show_black_jack_menu_passes_three_words() {
        check_table_menu(
            0x005c_f0f0,
            BLACK_JACK_MENU_CREATE,
            MSG_BLACK_JACK_PARAMETERS,
            MSG_BLACK_JACK_FORM,
            false,
        );
    }

    #[test]
    fn show_roulette_menu_passes_four_words() {
        check_table_menu(
            0x005c_f1a0,
            ROULETTE_MENU_CREATE,
            MSG_ROULETTE_PARAMETERS,
            MSG_ROULETTE_FORM,
            true,
        );
    }

    #[test]
    fn show_caravan_menu_opens_on_the_menu_target() {
        let mut e = engine2();
        menu_target_doubles(&mut e);
        e.register(CARAVAN_MENU_CREATE, |_, _| Ret::default());
        e.register(LOG_STUB, |_, _| Ret::default());
        let player = set_player(&mut e);
        let actor = menu_reference(&mut e, true, true);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[0x10, 0x20, 0x3f80_0000]);
        start_log(&mut e);
        assert!(e.call(0x005c_f250, &args![a]).bool());
        assert_eq!(
            calls(&e, CARAVAN_MENU_CREATE),
            vec![vec![actor, 0x10, 0x20, 0x3f80_0000, 0]]
        );
        // A zero first word, the player and no reference: no menu.
        parse_gives(&mut e, true, &[0, 0x20, 0]);
        start_log(&mut e);
        assert!(e.call(0x005c_f250, &args![a]).bool());
        parse_gives(&mut e, true, &[0x10, 0x20, 0]);
        for this_obj in [player, 0] {
            let a = command(&mut e, this_obj);
            assert!(e.call(0x005c_f250, &args![a]).bool());
        }
        assert!(calls(&e, CARAVAN_MENU_CREATE).is_empty());
        // Parameters that do not parse are reported, and the command succeeds.
        parse_gives(&mut e, false, &[]);
        assert!(e.call(0x005c_f250, &args![a]).bool());
        assert_eq!(calls(&e, LOG_STUB), vec![vec![MSG_CARAVAN_PARAMETERS]]);
        // Without a player nothing is parsed.
        e.set_global(PLAYER, 0u32);
        start_log(&mut e);
        assert!(e.call(0x005c_f250, &args![a]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
    }

    #[test]
    fn add_card_to_player_adds_a_caravan_card() {
        let mut e = engine2();
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(PLAYER_ADD_CARD, |_, _| true.into_ret());
        e.register(LOG_STUB, |_, _| Ret::default());
        e.register(V_NAME, |_, _| 0xbbbb.into_ret());
        // `__RTDynamicCast`: only the object 0xca2d is a card.
        e.register(DYNAMIC_CAST, |_, a| {
            if a[0] == 0xca2d {
                a[0].into_ret()
            } else {
                Ret::default()
            }
        });
        let player = set_player(&mut e);
        let reference = object_with(&mut e, &[(0x130, V_NAME)]);
        let a = command(&mut e, reference);
        parse_gives(&mut e, true, &[]);
        // A reference whose base form is a card.
        e.mem.set_u32(reference + 0x20, 0xca2d);
        start_log(&mut e);
        assert!(e.call(0x005c_f3d0, &args![a]).bool());
        assert_eq!(calls(&e, PLAYER_ADD_CARD), vec![vec![player, 0xca2d]]);
        assert!(calls(&e, LOG_STUB).is_empty());
        assert_eq!(calls(&e, PARSE_PARAMETERS)[0].len(), 7);
        // A base form that is no card: reported with the name.
        e.mem.set_u32(reference + 0x20, 0x1234);
        start_log(&mut e);
        assert!(e.call(0x005c_f3d0, &args![a]).bool());
        assert!(calls(&e, PLAYER_ADD_CARD).is_empty());
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_NOT_CARAVAN_CARD, 0xbbbb]]
        );
        // No base form, or no reference: nothing; a failed parse is reported
        // and the command goes on.
        e.mem.set_u32(reference + 0x20, 0);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(e.call(0x005c_f3d0, &args![a]).bool());
        assert_eq!(calls(&e, LOG_STUB), vec![vec![MSG_ADD_CARD_PARAMETER]]);
        assert!(calls(&e, PLAYER_ADD_CARD).is_empty());
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_f3d0, &args![none]).bool());
        assert_eq!(calls(&e, LOG_STUB).len(), 1);
    }

    #[test]
    fn is_player_in_region_walks_the_players_regions() {
        let mut e = engine2();
        e.register(PLAYER_REGION_LIST, |_, a| (a[0] + 0x764).into_ret());
        e.register(LIST_ITEM_CELL, |_, a| a[0].into_ret());
        e.register(LIST_NEXT_CELL, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LOG_STUB, |_, _| Ret::default());
        let player = set_player(&mut e);
        // Two nodes: the list head sits at player + 0x764 + 4 (item, next),
        // where `next` points at the cell (+4) of the second node.
        let region_a = object(&mut e);
        let region_b = object(&mut e);
        let wanted = object(&mut e);
        e.mem.set_u32(region_a + 0x0c, 0x100);
        e.mem.set_u32(region_b + 0x0c, 0x200);
        e.mem.set_u32(wanted + 0x0c, 0x200);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second + 4, region_b);
        e.mem.set_u32(second + 8, 0);
        e.mem.set_u32(player + 0x764 + 4, region_a);
        e.mem.set_u32(player + 0x764 + 8, second + 4);
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[wanted]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005c_f490, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        let mut expected = vec![MSG_IS_PLAYER_IN_REGION];
        expected.extend(double_words(1.0));
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![expected]);
        // A region the player is not in: 0.0.
        let stranger = object(&mut e);
        e.mem.set_u32(stranger + 0x0c, 0x300);
        parse_gives(&mut e, true, &[stranger]);
        e.mem.set_f64(a.result.addr(), 5.0);
        start_log(&mut e);
        assert!(e.call(0x005c_f490, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        let mut expected = vec![MSG_IS_PLAYER_IN_REGION];
        expected.extend(double_words(0.0));
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![expected]);
        // No region (zero): 0.0 without a walk.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005c_f490, &args![a]).bool());
        assert!(calls(&e, PLAYER_REGION_LIST).is_empty());
        // Parameters that do not parse: reported, the result stays as it was.
        parse_gives(&mut e, false, &[]);
        e.mem.set_f64(a.result.addr(), 5.0);
        start_log(&mut e);
        assert!(e.call(0x005c_f490, &args![a]).bool());
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_IS_PLAYER_IN_REGION_PARAMETER]]
        );
        assert_eq!(e.mem.f64(a.result.addr()), 5.0);
        let mut expected = vec![MSG_IS_PLAYER_IN_REGION];
        expected.extend(double_words(5.0));
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![expected]);
    }

    #[test]
    fn show_love_tester_menu_picks_the_variant_by_the_parse() {
        let mut e = engine2();
        e.register(LOVE_TESTER_MENU_CREATE, |_, _| Ret::default());
        e.register(LOVE_TESTER_MENU_CREATE_OV2, |_, _| Ret::default());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[0x42]);
        start_log(&mut e);
        assert!(e.call(0x005c_f5e0, &args![a]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        set_player(&mut e);
        assert!(e.call(0x005c_f5e0, &args![a]).bool());
        assert_eq!(calls(&e, LOVE_TESTER_MENU_CREATE_OV2), vec![vec![0x42]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(e.call(0x005c_f5e0, &args![a]).bool());
        assert_eq!(calls(&e, LOVE_TESTER_MENU_CREATE), vec![vec![]]);
    }

    /// The test of the trait menu commands: nothing without a player.
    fn check_trait_menu(address: u32, create: u32) {
        let mut e = engine2();
        e.register(create, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(address, &args![]).bool());
        assert!(calls(&e, create).is_empty());
        set_player(&mut e);
        assert!(e.call(address, &args![]).bool());
        assert_eq!(calls(&e, create), vec![vec![]]);
    }

    #[test]
    fn fn_005cf640_opens_the_trait_select_menu() {
        check_trait_menu(0x005c_f640, TRAIT_SELECT_MENU_CREATE);
    }

    #[test]
    fn fn_005cf660_opens_the_trait_menu() {
        check_trait_menu(0x005c_f660, TRAIT_MENU_CREATE);
    }

    /// A `ParseParameters` double that gives `reference` as the first output
    /// and the texts as C strings in the next ones.
    fn parse_gives_texts(e: &mut Engine, reference: u32, texts: &[&str]) {
        let texts: Vec<String> = texts.iter().map(|text| text.to_string()).collect();
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            e.mem.set_u32(a[7], reference);
            for (i, text) in texts.iter().enumerate() {
                for (j, byte) in text.bytes().chain([0]).enumerate() {
                    e.mem.set_u8(a[8 + i] + j as u32, byte);
                }
            }
            true.into_ret()
        });
    }

    #[test]
    fn fn_005cf680_applies_the_securitron_face() {
        let mut e = engine2();
        e.register(V_GET_3D, |_, _| 0x7000u32.into_ret());
        let reference = object_with(&mut e, &[(0x1d0, V_GET_3D), (0x48, V_RECORD_A)]);
        e.register(REFERENCE_GET_EXTRA_DATA, |_, _| 0x6100u32.into_ret());
        for address in [
            SECURITRON_FACE_SET_PERSONALITY,
            SECURITRON_FACE_SET_MOOD,
            SECURITRON_FACE_APPLY_FACE,
            ADD_EXTRA,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(OPERATOR_NEW, |e, _| e.mem.alloc(0x1c).into_ret());
        e.register(SECURITRON_FACE_CONSTRUCT, |_, a| a[0].into_ret());
        let a = command(&mut e, 0);
        parse_gives_texts(&mut e, reference, &["Cheerful", "Happy"]);
        start_log(&mut e);
        assert!(e.call(0x005c_f680, &args![a]).bool());
        let parse_words = calls(&e, PARSE_PARAMETERS).remove(0);
        let (personality, mood) = (parse_words[8], parse_words[9]);
        // The face of the reference is reused.
        assert!(calls(&e, ADD_EXTRA).is_empty());
        assert_eq!(
            calls(&e, REFERENCE_GET_EXTRA_DATA),
            vec![vec![reference, 0x8f]]
        );
        assert_eq!(
            calls(&e, SECURITRON_FACE_SET_PERSONALITY),
            vec![vec![0x6100, personality]]
        );
        assert_eq!(
            calls(&e, SECURITRON_FACE_SET_MOOD),
            vec![vec![0x6100, mood]]
        );
        assert_eq!(
            calls(&e, SECURITRON_FACE_APPLY_FACE),
            vec![vec![0x6100, 0x7000]]
        );
        // Both changes go to slot 0x48 (AddChange), one after the other.
        assert_eq!(
            calls(&e, V_RECORD_A),
            vec![vec![reference, 0x1000_0000], vec![reference, 0xa406_1840]]
        );
        // A reference without a face gets a new one added to its extra data.
        e.register(REFERENCE_GET_EXTRA_DATA, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005c_f680, &args![a]).bool());
        let added = calls(&e, SECURITRON_FACE_CONSTRUCT);
        assert_eq!(added.len(), 1);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x1c]]);
        assert_eq!(
            calls(&e, ADD_EXTRA),
            vec![vec![reference + 0x44, added[0][0]]]
        );
        assert_eq!(
            calls(&e, SECURITRON_FACE_APPLY_FACE),
            vec![vec![added[0][0], 0x7000]]
        );
        // A reference without a 3D object: nothing.
        e.register(V_GET_3D, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005c_f680, &args![a]).bool());
        assert!(calls(&e, REFERENCE_GET_EXTRA_DATA).is_empty());
        // No reference: nothing; parameters that do not parse: failure.
        parse_gives_texts(&mut e, 0, &["", ""]);
        assert!(e.call(0x005c_f680, &args![a]).bool());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_f680, &args![a]).bool());
    }

    /// A node of the swap-texture test: a shade property (type `shader_type`
    /// or none) and a texture property.
    struct SwapSetup {
        reference: u32,
        shader: u32,
        textured: u32,
    }

    fn swap_texture_setup(
        e: &mut Engine,
        shader_type: i32,
        with_shader: bool,
        with_textured: bool,
    ) -> SwapSetup {
        let node = object(e);
        e.register_double(V_FIND_NODE, move |_, _| node.into_ret());
        let root = object_with(e, &[(0x9c, V_FIND_NODE)]);
        e.register_double(V_GET_3D, move |_, _| root.into_ret());
        let reference = object_with(e, &[(0x1d0, V_GET_3D)]);
        let shader = object_with(e, &[(0xf0, V_SET_TEXTURE)]);
        e.register(V_SET_TEXTURE, |_, _| Ret::default());
        let textured = object(e);
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_DESTRUCT, |_, _| Ret::default());
        e.register(STRING_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(STRING_LENGTH, |e, a| {
            let text = e.mem.u32(a[0]);
            (e.mem.u8(text) as u32).into_ret()
        });
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(FIXED_STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(FIXED_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(FORMAT_INTO_BUFFER, |_, _| Ret::default());
        e.register(SWAP_PLATFORM_LANGUAGE_TEXTURE_PATH, |_, _| Ret::default());
        e.register(CREATE_TEXTURE_IMAGE, |e, a| {
            // The loaded image goes to the smart pointer.
            e.mem.set_u32(a[2], 0x7777);
            Ret::default()
        });
        e.register_double(GET_PROPERTY, move |_, a| (a[0] + 0x30).into_ret());
        e.register(PROPERTY_TYPE_SHADE, |_, _| 3u32.into_ret());
        e.register_double(NI_DYNAMIC_CAST, move |_, a| {
            let wanted = if a[0] == RTTI_NI_SHADE_PROPERTY {
                with_shader.then_some(shader)
            } else {
                with_textured.then_some(textured)
            };
            wanted.unwrap_or(0).into_ret()
        });
        e.register_double(GET_SHADER_PROPERTY_TYPE, move |_, _| {
            (shader_type as u32).into_ret()
        });
        e.register(PROPERTY_SET_TEXTURE, |_, _| Ret::default());
        SwapSetup {
            reference,
            shader,
            textured,
        }
    }

    #[test]
    fn swap_texture_on_ref_loads_the_texture_into_the_properties() {
        let mut e = engine2();
        let setup = swap_texture_setup(&mut e, 10, true, true);
        let a = command(&mut e, 0);
        parse_gives_texts(&mut e, setup.reference, &["Node01", "face"]);
        start_log(&mut e);
        assert!(e.call(0x005c_f860, &args![a]).bool());
        let parse_words = calls(&e, PARSE_PARAMETERS).remove(0);
        assert_eq!(parse_words.len(), 10);
        let (node_name, texture_name) = (parse_words[8], parse_words[9]);
        // The texture path is formatted from the second text.
        let formatted = calls(&e, FORMAT_INTO_BUFFER);
        assert_eq!(formatted.len(), 1);
        assert_eq!(
            &formatted[0][1..],
            [0x104, TEXTURE_PATH_FORMAT, texture_name]
        );
        assert_eq!(
            calls(&e, SWAP_PLATFORM_LANGUAGE_TEXTURE_PATH)[0][0],
            formatted[0][0]
        );
        assert_eq!(calls(&e, FIXED_STRING_CONSTRUCT)[0][1], node_name);
        // `TES::CreateTextureImage` on the TES singleton, then the texture
        // goes to both properties.
        assert_eq!(calls(&e, CREATE_TEXTURE_IMAGE).len(), 1);
        assert_eq!(
            calls(&e, CREATE_TEXTURE_IMAGE)[0][0],
            e.global::<u32>(0x011d_ea10)
        );
        assert_eq!(
            calls(&e, V_SET_TEXTURE),
            vec![vec![setup.shader, 0, 0, 0x7777]]
        );
        assert_eq!(
            calls(&e, PROPERTY_SET_TEXTURE),
            vec![vec![setup.textured, 0x7777]]
        );
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 1);
        // Both strings are built and destroyed.
        assert_eq!(calls(&e, STRING_CONSTRUCT).len(), 2);
        assert_eq!(calls(&e, STRING_DESTRUCT).len(), 2);
        // The shader type must be from 8 to 12.
        for (shader_type, passes) in [(7, false), (8, true), (12, true), (13, false), (-1, false)] {
            let mut e = engine2();
            let setup = swap_texture_setup(&mut e, shader_type, true, false);
            parse_gives_texts(&mut e, setup.reference, &["Node01", "face"]);
            start_log(&mut e);
            assert!(e.call(0x005c_f860, &args![a]).bool());
            assert_eq!(
                !calls(&e, V_SET_TEXTURE).is_empty(),
                passes,
                "type {shader_type}"
            );
            assert!(calls(&e, PROPERTY_SET_TEXTURE).is_empty());
        }
        // No shade property of that class: no call to the shader.
        let mut e = engine2();
        let setup = swap_texture_setup(&mut e, 10, false, true);
        parse_gives_texts(&mut e, setup.reference, &["Node01", "face"]);
        start_log(&mut e);
        assert!(e.call(0x005c_f860, &args![a]).bool());
        assert!(calls(&e, V_SET_TEXTURE).is_empty());
        assert_eq!(calls(&e, PROPERTY_SET_TEXTURE).len(), 1);
    }

    #[test]
    fn swap_texture_on_ref_needs_both_texts_and_a_node() {
        let a = ScriptArgs {
            param_info: 1,
            script_data: 2,
            this_obj: Ptr::NULL,
            containing_obj: Ptr::NULL,
            script_obj: Ptr::new(5),
            event_list: 6,
            result: Ptr::NULL,
            opcode_offset: 8,
        };
        // An empty text fails the command, strings still destroyed.
        for texts in [["", "face"], ["Node01", ""]] {
            let mut e = engine2();
            let setup = swap_texture_setup(&mut e, 10, true, true);
            parse_gives_texts(&mut e, setup.reference, &texts);
            start_log(&mut e);
            assert!(!e.call(0x005c_f860, &args![a]).bool());
            assert_eq!(calls(&e, STRING_DESTRUCT).len(), 2);
            assert!(calls(&e, CREATE_TEXTURE_IMAGE).is_empty());
        }
        // Parameters that do not parse: failure, strings destroyed.
        let mut e = engine2();
        swap_texture_setup(&mut e, 10, true, true);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_f860, &args![a]).bool());
        assert_eq!(calls(&e, STRING_DESTRUCT).len(), 2);
        // A node that is not found: success without loading anything.
        let mut e = engine2();
        let setup = swap_texture_setup(&mut e, 10, true, true);
        e.register(V_FIND_NODE, |_, _| Ret::default());
        parse_gives_texts(&mut e, setup.reference, &["Node01", "face"]);
        start_log(&mut e);
        assert!(e.call(0x005c_f860, &args![a]).bool());
        assert_eq!(calls(&e, FIXED_STRING_DESTRUCT).len(), 1);
        assert!(calls(&e, CREATE_TEXTURE_IMAGE).is_empty());
        // No reference and a reference without 3D: success, nothing loaded.
        parse_gives_texts(&mut e, 0, &["Node01", "face"]);
        assert!(e.call(0x005c_f860, &args![a]).bool());
        parse_gives_texts(&mut e, setup.reference, &["Node01", "face"]);
        e.register(V_GET_3D, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005c_f860, &args![a]).bool());
        assert!(calls(&e, FIXED_STRING_CONSTRUCT).is_empty());
    }

    #[test]
    fn fn_005cfcb0_stores_the_hud_flag_when_the_menu_exists() {
        let mut e = engine2();
        let menu = e.mem.alloc(0x300);
        e.call(0x005c_fcb0, &args![1u8]);
        assert_eq!(e.mem.u8(menu + 0x205), 0);
        e.set_global(HUD_MAIN_MENU, menu);
        e.call(0x005c_fcb0, &args![1u8]);
        assert_eq!(e.mem.u8(menu + 0x205), 1);
        e.call(0x005c_fcb0, &args![0u8]);
        assert_eq!(e.mem.u8(menu + 0x205), 0);
    }

    #[test]
    fn fn_005cfc50_stores_whether_the_integer_is_positive() {
        let mut e = engine2();
        let menu = e.mem.alloc(0x300);
        e.set_global(HUD_MAIN_MENU, menu);
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        // Only with a player.
        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        assert!(e.call(0x005c_fc50, &args![a]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        set_player(&mut e);
        assert!(e.call(0x005c_fc50, &args![a]).bool());
        assert_eq!(e.mem.u8(menu + 0x205), 1);
        // Signed `> 0`.
        for word in [0u32, 0xffff_ffff] {
            parse_gives(&mut e, true, &[word]);
            assert!(e.call(0x005c_fc50, &args![a]).bool());
            assert_eq!(e.mem.u8(menu + 0x205), 0);
            e.mem.set_u8(menu + 0x205, 1);
        }
        // Parameters that do not parse: nothing stored, still success.
        parse_gives(&mut e, false, &[]);
        assert!(e.call(0x005c_fc50, &args![a]).bool());
        assert_eq!(e.mem.u8(menu + 0x205), 1);
    }

    /// The test of `005cfcd0` and `005cfd20`: they queue `menu` with the
    /// reference when the base form's type is 0x17.
    fn check_queue_menu(command_address: u32, menu: u32) {
        let mut e = engine2();
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(FORM_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
        e.register(QUEUE_MENU_CREATE, |_, _| Ret::default());
        let base = object(&mut e);
        e.mem.set_u8(base + 4, 0x17);
        let reference = object(&mut e);
        e.mem.set_u32(reference + 0x20, base);
        let a = command(&mut e, reference);
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_eq!(
            calls(&e, QUEUE_MENU_CREATE),
            vec![vec![menu, reference, 0, 0, 1, 0]]
        );
        // Another form type, and no reference: the menu without a reference.
        e.mem.set_u8(base + 4, 0x18);
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        let none = command(&mut e, 0);
        assert!(e.call(command_address, &args![none]).bool());
        assert_eq!(
            calls(&e, QUEUE_MENU_CREATE),
            vec![vec![menu, 0, 0, 0, 1, 0]; 2]
        );
    }

    #[test]
    fn fn_005cfcd0_queues_menu_2() {
        check_queue_menu(0x005c_fcd0, 2);
    }

    #[test]
    fn fn_005cfd20_queues_menu_3() {
        check_queue_menu(0x005c_fd20, 3);
    }

    #[test]
    fn fn_005cfd70_returns_the_condition_function_s_result() {
        check_condition_only(0x005c_fd70, UNKNOWN_CONDITION_005A2A50, true);
    }

    #[test]
    fn fn_005cfd90_returns_the_condition_function_s_result() {
        check_condition_only(0x005c_fd90, GET_SANDMAN_CONDITION, true);
    }

    #[test]
    fn fn_005cfdb0_returns_the_condition_function_s_result() {
        check_condition_only(0x005c_fdb0, GET_CANNIBAL_CONDITION, true);
    }

    #[test]
    fn fn_005cfdd0_returns_the_condition_function_s_result() {
        check_condition_only(0x005c_fdd0, HAS_BEEN_EATEN_CONDITION, true);
    }

    #[test]
    fn fn_005cff90_returns_the_condition_function_s_result() {
        check_condition_only(0x005c_ff90, GET_GHOST_CONDITION, true);
    }

    #[test]
    fn fn_005d0900_returns_the_condition_function_s_result() {
        check_condition_only(0x005d_0900, GET_UNCONSCIOUS_CONDITION, true);
    }

    #[test]
    fn fn_005d09c0_returns_the_condition_function_s_result() {
        check_condition_only(0x005d_09c0, GET_RESTRAINED_CONDITION, true);
    }

    #[test]
    fn fn_005cfdf0_hands_the_process_values_to_the_other_actor() {
        let mut e = engine2();
        e.register(V_PROCESS, |_, _| 0x41u32.into_ret());
        e.register(V_PROCESS_B, |_, _| 0x42u32.into_ret());
        e.register(V_PROCESS_C, |_, _| 0x43u32.into_ret());
        let process = object_with(
            &mut e,
            &[
                (0x4d0, V_PROCESS),
                (0x4d4, V_PROCESS_B),
                (0x4c8, V_PROCESS_C),
            ],
        );
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        let actor = object_with(&mut e, &[(0x100, V_TRUE)]);
        let other = object_with(&mut e, &[(0x100, V_TRUE), (0x374, V_ACTOR_RECORD)]);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[other]);
        start_log(&mut e);
        assert!(e.call(0x005c_fdf0, &args![a]).bool());
        assert_parsed(&e, actor);
        // The values are read in the order 4d0, 4d4, 4c8 and given in the
        // order 4c8, 4d4, 4d0.
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .filter(|address| [V_PROCESS, V_PROCESS_B, V_PROCESS_C].contains(address))
            .collect();
        assert_eq!(order, vec![V_PROCESS, V_PROCESS_B, V_PROCESS_C]);
        assert_eq!(
            calls(&e, V_ACTOR_RECORD),
            vec![vec![other, actor, 0x43, 0x42, 0x41]]
        );
        // The other one is no actor, or there is none: nothing.
        let not_actor = object_with(&mut e, &[(0x100, V_FALSE), (0x374, V_ACTOR_RECORD)]);
        for other in [not_actor, 0] {
            parse_gives(&mut e, true, &[other]);
            start_log(&mut e);
            assert!(e.call(0x005c_fdf0, &args![a]).bool());
            assert!(calls(&e, V_ACTOR_RECORD).is_empty());
        }
        // `thisObj` is no actor.
        let not_actor_this = object_with(&mut e, &[(0x100, V_FALSE)]);
        let b = command(&mut e, not_actor_this);
        parse_gives(&mut e, true, &[other]);
        assert!(e.call(0x005c_fdf0, &args![b]).bool());
        assert!(calls(&e, V_ACTOR_RECORD).is_empty());
        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_fdf0, &args![a]).bool());
    }

    #[test]
    fn fn_005cfef0_calls_slot_0x378_of_the_other_actor() {
        let mut e = engine2();
        let actor = object_with(&mut e, &[(0x100, V_TRUE)]);
        let other = object_with(&mut e, &[(0x100, V_TRUE), (0x378, V_ACTOR_RECORD)]);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[other]);
        start_log(&mut e);
        assert!(e.call(0x005c_fef0, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, V_ACTOR_RECORD), vec![vec![other, actor]]);
        let not_actor = object_with(&mut e, &[(0x100, V_FALSE), (0x378, V_ACTOR_RECORD)]);
        for other in [not_actor, 0] {
            parse_gives(&mut e, true, &[other]);
            start_log(&mut e);
            assert!(e.call(0x005c_fef0, &args![a]).bool());
            assert!(calls(&e, V_ACTOR_RECORD).is_empty());
        }
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_fef0, &args![a]).bool());
    }

    #[test]
    fn set_ghost_passes_whether_the_integer_is_positive() {
        let mut e = engine2();
        e.register(ACTOR_SET_GHOST, |_, _| Ret::default());
        let actor = object(&mut e);
        let a = command(&mut e, actor);
        set_echo(&mut e, true);
        for (word, flag) in [(3u32, 1u32), (0, 0), (0xffff_fffe, 0)] {
            parse_gives(&mut e, true, &[word]);
            start_log(&mut e);
            assert!(e.call(0x005c_ffb0, &args![a]).bool());
            assert_parsed(&e, actor);
            assert_eq!(calls(&e, ACTOR_SET_GHOST), vec![vec![actor, flag]]);
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_SET_GHOST, word]]);
        }
        // Not an actor: no call, the echo stays; echo off: silent.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        parse_gives(&mut e, true, &[1]);
        set_echo(&mut e, false);
        start_log(&mut e);
        assert!(e.call(0x005c_ffb0, &args![a]).bool());
        assert!(calls(&e, ACTOR_SET_GHOST).is_empty());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_ffb0, &args![a]).bool());
    }

    /// The doubles the equip and unequip commands share: the extra data list
    /// of an actor is `actor + 0x44`, list nodes are `(item, next)` pairs, the
    /// message machinery records its calls.
    fn inventory_doubles(e: &mut Engine) {
        e.register(GET_CONTAINER_CHANGES, |_, a| (a[0] + 0x1000).into_ret());
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_ITEM_CELL, |_, a| a[0].into_ret());
        e.register(LIST_NEXT_CELL, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EXTRA_LIST_GET_WORN, |e, a| {
            (e.mem.u8(a[0] + 0x10) != 0).into_ret()
        });
        e.register(EXTRA_LIST_SET_CAN_NOT_WEAR, |_, _| Ret::default());
        e.register(INVENTORY_ENTRY_DELETE, |_, _| Ret::default());
        e.register(ACTOR_QUEUE_EQUIP_OBJECT, |_, _| Ret::default());
        e.register(ACTOR_QUEUE_UNEQUIP_OBJECT, |_, _| Ret::default());
        e.register(LOG_STUB, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_DESTRUCT, |_, _| Ret::default());
        e.register(STRING_FORMAT, |_, _| Ret::default());
        e.register(BS_STRING_TEXT, |_, a| (a[0] + 0x10).into_ret());
        e.register(GET_FULL_NAME, |_, a| (a[0] + 0x20).into_ret());
        e.register(SHOW_MESSAGE, |_, _| Ret::default());
        e.register(PLAYER_UPDATE_3D, |_, _| Ret::default());
        e.register(REFRESH_MENUS, |_, _| Ret::default());
        e.register(V_PROCESS, |_, _| Ret::default());
    }

    /// An inventory entry whose single node holds `extra_list`.
    fn entry_with_list(e: &mut Engine, extra_list: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, extra_list);
        e.mem.set_u32(node + 4, 0);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, node);
        entry
    }

    /// The names of the calls of the message flow, in order.
    fn message_flow(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .filter(|address| {
                [
                    STRING_CONSTRUCT,
                    BS_STRING_TEXT,
                    GET_FULL_NAME,
                    STRING_FORMAT,
                    SHOW_MESSAGE,
                    PLAYER_UPDATE_3D,
                    V_PROCESS,
                    REFRESH_MENUS,
                    STRING_DESTRUCT,
                ]
                .contains(address)
            })
            .collect()
    }

    #[test]
    fn equip_item_queues_the_item_and_tells_the_player() {
        let mut e = engine2();
        inventory_doubles(&mut e);
        let player = set_player(&mut e);
        let process = object_with(&mut e, &[(0x580, V_PROCESS)]);
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        let item = object(&mut e);
        // No entry for the item in the inventory changes.
        e.register(GET_INVENTORY_ITEM, |_, _| Ret::default());
        let a = command_with_script(&mut e, player, 0x5000);
        parse_gives(&mut e, true, &[item, 1, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_0060, &args![a]).bool());
        assert_eq!(
            calls(&e, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![player, item, 1, 0, 1, 1, 1]]
        );
        assert_eq!(calls(&e, GET_CONTAINER_CHANGES), vec![vec![player + 0x44]]);
        assert_eq!(
            calls(&e, GET_INVENTORY_ITEM),
            vec![vec![player + 0x44 + 0x1000, item, 0]]
        );
        // The message: the text global's text, the item's full name.
        assert_eq!(calls(&e, BS_STRING_TEXT), vec![vec![EQUIPPED_TEXT_GLOBAL]]);
        let format = calls(&e, STRING_FORMAT);
        assert_eq!(format.len(), 1);
        assert_eq!(
            &format[0][1..],
            [
                ITEM_MESSAGE_FORMAT,
                item + 0x20,
                EQUIPPED_TEXT_GLOBAL + 0x10
            ]
        );
        assert_eq!(calls(&e, SHOW_MESSAGE)[0][1..4], [0, MESSAGE_ICON, 0]);
        assert_eq!(calls(&e, V_PROCESS), vec![vec![process, 1, 0, 0]]);
        assert_eq!(
            message_flow(&e),
            vec![
                STRING_CONSTRUCT,
                BS_STRING_TEXT,
                GET_FULL_NAME,
                STRING_FORMAT,
                SHOW_MESSAGE,
                PLAYER_UPDATE_3D,
                V_PROCESS,
                REFRESH_MENUS,
                STRING_DESTRUCT
            ]
        );
        // A count of zero or less clears the worn flag; the third flag keeps
        // the message away.
        parse_gives(&mut e, true, &[item, 0xffff_ffff, 1]);
        start_log(&mut e);
        assert!(e.call(0x005d_0060, &args![a]).bool());
        assert_eq!(
            calls(&e, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![player, item, 1, 0, 1, 0, 1]]
        );
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        // Another actor: queued, no message.
        let npc = object(&mut e);
        let b = command_with_script(&mut e, npc, 0x5000);
        parse_gives(&mut e, true, &[item, 1, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_0060, &args![b]).bool());
        assert_eq!(calls(&e, ACTOR_QUEUE_EQUIP_OBJECT).len(), 1);
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        // No inventory changes: also queued.
        e.register(GET_CONTAINER_CHANGES, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_0060, &args![b]).bool());
        assert_eq!(calls(&e, ACTOR_QUEUE_EQUIP_OBJECT).len(), 1);
        assert!(calls(&e, GET_INVENTORY_ITEM).is_empty());
    }

    #[test]
    fn equip_item_leaves_a_worn_item_alone() {
        let mut e = engine2();
        inventory_doubles(&mut e);
        let player = set_player(&mut e);
        let process = object_with(&mut e, &[(0x580, V_PROCESS)]);
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        let item = object(&mut e);
        let worn_list = object(&mut e);
        e.mem.set_u8(worn_list + 0x10, 1);
        let entry = entry_with_list(&mut e, worn_list);
        e.register_double(GET_INVENTORY_ITEM, move |_, _| entry.into_ret());
        let a = command_with_script(&mut e, player, 0x5000);
        parse_gives(&mut e, true, &[item, 5, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_0060, &args![a]).bool());
        assert_eq!(
            calls(&e, EXTRA_LIST_SET_CAN_NOT_WEAR),
            vec![vec![worn_list, 1]]
        );
        assert!(calls(&e, ACTOR_QUEUE_EQUIP_OBJECT).is_empty());
        assert_eq!(calls(&e, INVENTORY_ENTRY_DELETE), vec![vec![entry, 1]]);
        // A list that is not worn: the entry is walked to its end, then the
        // item is queued.
        let plain_list = object(&mut e);
        let entry = entry_with_list(&mut e, plain_list);
        e.register_double(GET_INVENTORY_ITEM, move |_, _| entry.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_0060, &args![a]).bool());
        assert!(calls(&e, EXTRA_LIST_SET_CAN_NOT_WEAR).is_empty());
        assert_eq!(calls(&e, ACTOR_QUEUE_EQUIP_OBJECT).len(), 1);
        assert_eq!(calls(&e, INVENTORY_ENTRY_DELETE), vec![vec![entry, 1]]);
    }

    #[test]
    fn equip_item_reports_a_missing_item_and_bad_parameters() {
        let mut e = engine2();
        inventory_doubles(&mut e);
        let player = set_player(&mut e);
        e.register(V_NAME, |_, _| 0xbbbb.into_ret());
        let script = object_with(&mut e, &[(0x130, V_NAME)]);
        let a = command_with_script(&mut e, player, script);
        parse_gives(&mut e, true, &[0, 1, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_0060, &args![a]).bool());
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_EQUIP_ITEM_FAILED, 0xbbbb]]
        );
        assert!(calls(&e, ACTOR_QUEUE_EQUIP_OBJECT).is_empty());
        // No reference: nothing at all.
        let none = command_with_script(&mut e, 0, script);
        start_log(&mut e);
        assert!(e.call(0x005d_0060, &args![none]).bool());
        assert!(calls(&e, LOG_STUB).is_empty());
        // The reference is no actor: nothing is queued.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        parse_gives(&mut e, true, &[player, 1, 0]);
        assert!(e.call(0x005d_0060, &args![a]).bool());
        assert!(calls(&e, ACTOR_QUEUE_EQUIP_OBJECT).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_0060, &args![a]).bool());
    }

    #[test]
    fn fn_005d0300_unequips_the_worn_item() {
        let mut e = engine2();
        inventory_doubles(&mut e);
        let player = set_player(&mut e);
        let process = object_with(&mut e, &[(0x580, V_PROCESS)]);
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(GET_INVENTORY_CHANGES, |_, a| (a[0] + 0x2000).into_ret());
        e.register(WEARING_OBJECT, |_, _| 0x6600u32.into_ret());
        let item = object(&mut e);
        let a = command(&mut e, player);
        parse_gives(&mut e, true, &[item, 1, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_0300, &args![a]).bool());
        assert_eq!(calls(&e, GET_INVENTORY_CHANGES), vec![vec![player]]);
        assert_eq!(
            calls(&e, WEARING_OBJECT),
            vec![vec![player + 0x2000, item, 0]]
        );
        assert_eq!(
            calls(&e, EXTRA_LIST_SET_CAN_NOT_WEAR),
            vec![vec![0x6600, 0]]
        );
        assert_eq!(
            calls(&e, ACTOR_QUEUE_UNEQUIP_OBJECT),
            vec![vec![player, item, 1, 0, 1, 1, 1]]
        );
        assert_eq!(
            calls(&e, BS_STRING_TEXT),
            vec![vec![UNEQUIPPED_TEXT_GLOBAL]]
        );
        assert_eq!(calls(&e, SHOW_MESSAGE).len(), 1);
        // The third flag keeps the message away; a count of zero clears the
        // worn flag.
        parse_gives(&mut e, true, &[item, 0, 1]);
        start_log(&mut e);
        assert!(e.call(0x005d_0300, &args![a]).bool());
        assert_eq!(
            calls(&e, ACTOR_QUEUE_UNEQUIP_OBJECT),
            vec![vec![player, item, 1, 0, 1, 0, 1]]
        );
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        // Another actor: no message. Nothing worn: nothing is queued.
        let npc = object(&mut e);
        let b = command(&mut e, npc);
        parse_gives(&mut e, true, &[item, 1, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_0300, &args![b]).bool());
        assert_eq!(calls(&e, ACTOR_QUEUE_UNEQUIP_OBJECT).len(), 1);
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        e.register(WEARING_OBJECT, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_0300, &args![b]).bool());
        assert!(calls(&e, ACTOR_QUEUE_UNEQUIP_OBJECT).is_empty());
        // No inventory changes, no actor: nothing either.
        e.register(GET_INVENTORY_CHANGES, |_, _| Ret::default());
        assert!(e.call(0x005d_0300, &args![b]).bool());
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x005d_0300, &args![b]).bool());
        // Parameters that do not parse; no reference.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_0300, &args![b]).bool());
        parse_gives(&mut e, true, &[item, 1, 0]);
        let none = command(&mut e, 0);
        assert!(e.call(0x005d_0300, &args![none]).bool());
    }

    #[test]
    fn fn_005d0300_without_an_item_marks_the_first_extra_list() {
        let mut e = engine2();
        inventory_doubles(&mut e);
        e.register(GET_INVENTORY_CHANGES, |_, a| (a[0] + 0x2000).into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(EXTRA_LIST_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(LIST_HEAD_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(LIST_APPEND, |_, _| Ret::default());
        let reference = object(&mut e);
        let a = command(&mut e, reference);
        parse_gives(&mut e, true, &[0, 1, 0]);
        // An entry whose first node has an extra list: it is marked.
        let existing = object(&mut e);
        let entry = entry_with_list(&mut e, existing);
        e.register_double(GET_INVENTORY_ITEM, move |_, _| entry.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_0300, &args![a]).bool());
        assert_eq!(
            calls(&e, GET_INVENTORY_ITEM),
            vec![vec![reference + 0x2000, 0, 0]]
        );
        assert_eq!(
            calls(&e, EXTRA_LIST_SET_CAN_NOT_WEAR),
            vec![vec![existing, 1]]
        );
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        assert_eq!(calls(&e, INVENTORY_ENTRY_DELETE), vec![vec![entry, 1]]);
        // A head with no item in its first node: a new list is appended.
        let empty_entry = entry_with_list(&mut e, 0);
        e.register_double(GET_INVENTORY_ITEM, move |_, _| empty_entry.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_0300, &args![a]).bool());
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x20]]);
        let marked = calls(&e, EXTRA_LIST_SET_CAN_NOT_WEAR);
        assert_eq!(marked.len(), 1);
        assert_eq!(marked[0][1], 1);
        let appended = calls(&e, LIST_APPEND);
        assert_eq!(appended.len(), 1);
        assert_eq!(appended[0][0], e.mem.u32(empty_entry));
        assert_eq!(e.mem.u32(appended[0][1]), marked[0][0]);
        // An entry with no list at all: a list head is made first.
        let bare = e.mem.alloc(8);
        e.mem.set_u32(bare, 0);
        e.register_double(GET_INVENTORY_ITEM, move |_, _| bare.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_0300, &args![a]).bool());
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![8], vec![0x20]]);
        let head = e.mem.u32(bare);
        assert_ne!(head, 0);
        assert_eq!(calls(&e, LIST_APPEND)[0][0], head);
        assert_eq!(calls(&e, INVENTORY_ENTRY_DELETE), vec![vec![bare, 1]]);
        // A failed allocation gives a null list that is marked all the same.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.mem.set_u32(bare, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_0300, &args![a]).bool());
        assert!(calls(&e, EXTRA_LIST_CONSTRUCT).is_empty());
        // No entry: nothing.
        e.register(GET_INVENTORY_ITEM, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_0300, &args![a]).bool());
        assert!(calls(&e, INVENTORY_ENTRY_DELETE).is_empty());
    }

    #[test]
    fn fn_005d06a0_initialises_the_npc_values() {
        let mut e = engine2();
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(NPC_SET_FIELD_130, |_, _| Ret::default());
        e.register(NPC_INIT_VALUES, |_, _| Ret::default());
        let actor = object(&mut e);
        e.mem.set_u32(actor + 0x20, 0x9999);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[0x77]);
        start_log(&mut e);
        assert!(e.call(0x005d_06a0, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, DYNAMIC_CAST).len(), 2);
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[1],
            vec![0x9999, 0, RTTI_TES_BOUND_OBJECT, RTTI_TES_NPC, 0]
        );
        assert_eq!(calls(&e, NPC_SET_FIELD_130), vec![vec![0x9999, 0x77]]);
        assert_eq!(calls(&e, NPC_INIT_VALUES), vec![vec![0x9999, 0]]);
        // A zero word: the base form is not even looked at.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_06a0, &args![a]).bool());
        assert!(calls(&e, NPC_SET_FIELD_130).is_empty());
        assert!(calls(&e, GET_BASE_FORM).is_empty());
        // A base form that is no NPC.
        parse_gives(&mut e, true, &[0x77]);
        e.register(DYNAMIC_CAST, |_, a| {
            if a[0] == 0x9999 {
                Ret::default()
            } else {
                a[0].into_ret()
            }
        });
        start_log(&mut e);
        assert!(e.call(0x005d_06a0, &args![a]).bool());
        assert!(calls(&e, NPC_SET_FIELD_130).is_empty());
        // No reference, parameters that do not parse.
        let none = command(&mut e, 0);
        assert!(e.call(0x005d_06a0, &args![none]).bool());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_06a0, &args![a]).bool());
    }

    /// Doubles for the life-state commands: the process of an actor is the
    /// object `process`, whose slots record their calls.
    fn life_state_doubles(e: &mut Engine) -> u32 {
        e.register(ACTOR_SET_LIFE_STATE_3, |_, _| Ret::default());
        e.register(ACTOR_SET_LIFE_STATE_5, |_, _| Ret::default());
        e.register(ACTOR_END_MOVEMENT, |_, _| Ret::default());
        e.register(ACTOR_IS_LIFE_STATE_3, |e, a| {
            (e.mem.u8(a[0] + 0x108) == 3).into_ret()
        });
        for slot in [V_RECORD_A, V_RECORD_B, V_PROCESS_B, V_PROCESS_C] {
            e.register(slot, |_, _| Ret::default());
        }
        let process = object_with(
            e,
            &[
                (0x614, V_RECORD_A),
                (0x71c, V_RECORD_B),
                (0x338, V_PROCESS_B),
            ],
        );
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(V_ACTOR_RECORD, |_, _| Ret::default());
        process
    }

    #[test]
    fn fn_005d0760_sets_and_clears_life_state_3() {
        let mut e = engine2();
        let process = life_state_doubles(&mut e);
        e.set_global(FLOAT_ARGUMENT, 2.5f32);
        let actor = object_with(&mut e, &[(0x434, V_ACTOR_RECORD)]);
        let a = command(&mut e, actor);
        // A non-zero flag on an actor that is not in life state 3.
        parse_gives(&mut e, true, &[1]);
        start_log(&mut e);
        assert!(e.call(0x005d_0760, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, V_ACTOR_RECORD), vec![vec![actor, 0]]);
        assert_eq!(calls(&e, V_RECORD_A), vec![vec![process, 0x800]]);
        assert_eq!(calls(&e, V_RECORD_B), vec![vec![process, 0]]);
        assert_eq!(calls(&e, ACTOR_SET_LIFE_STATE_3), vec![vec![actor, 1]]);
        assert_eq!(calls(&e, ACTOR_END_MOVEMENT), vec![vec![actor]]);
        assert_eq!(
            calls(&e, V_PROCESS_B),
            vec![vec![process, 2.5f32.to_bits()]]
        );
        // Already in life state 3: no process calls before the state is set.
        e.mem.set_u8(actor + 0x108, 3);
        start_log(&mut e);
        assert!(e.call(0x005d_0760, &args![a]).bool());
        assert!(calls(&e, V_RECORD_A).is_empty());
        assert_eq!(calls(&e, ACTOR_SET_LIFE_STATE_3), vec![vec![actor, 1]]);
        // A zero flag: the process calls only when the state is 3.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_0760, &args![a]).bool());
        assert_eq!(calls(&e, V_RECORD_A), vec![vec![process, 0x800]]);
        assert_eq!(calls(&e, V_RECORD_B), vec![vec![process, 0]]);
        assert_eq!(calls(&e, ACTOR_SET_LIFE_STATE_3), vec![vec![actor, 0]]);
        assert!(calls(&e, ACTOR_END_MOVEMENT).is_empty());
        e.mem.set_u8(actor + 0x108, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_0760, &args![a]).bool());
        assert!(calls(&e, V_RECORD_A).is_empty());
        assert_eq!(calls(&e, ACTOR_SET_LIFE_STATE_3), vec![vec![actor, 0]]);
        // An actor without a process: only the state changes.
        e.register(GET_PROCESS, |_, _| Ret::default());
        parse_gives(&mut e, true, &[1]);
        start_log(&mut e);
        assert!(e.call(0x005d_0760, &args![a]).bool());
        assert!(calls(&e, V_PROCESS_B).is_empty());
        assert_eq!(calls(&e, ACTOR_SET_LIFE_STATE_3), vec![vec![actor, 1]]);
        // Not an actor, no reference, parameters that do not parse.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_0760, &args![a]).bool());
        assert!(calls(&e, ACTOR_SET_LIFE_STATE_3).is_empty());
        let none = command(&mut e, 0);
        assert!(e.call(0x005d_0760, &args![none]).bool());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_0760, &args![a]).bool());
    }

    #[test]
    fn fn_005d0920_sets_and_clears_life_state_5() {
        let mut e = engine2();
        life_state_doubles(&mut e);
        let actor = object(&mut e);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[1]);
        start_log(&mut e);
        assert!(e.call(0x005d_0920, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, ACTOR_SET_LIFE_STATE_5), vec![vec![actor, 1]]);
        assert_eq!(calls(&e, ACTOR_END_MOVEMENT), vec![vec![actor]]);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_0920, &args![a]).bool());
        assert_eq!(calls(&e, ACTOR_SET_LIFE_STATE_5), vec![vec![actor, 0]]);
        assert!(calls(&e, ACTOR_END_MOVEMENT).is_empty());
        // Not an actor, no reference, parameters that do not parse.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_0920, &args![a]).bool());
        assert!(calls(&e, ACTOR_SET_LIFE_STATE_5).is_empty());
        let none = command(&mut e, 0);
        assert!(e.call(0x005d_0920, &args![none]).bool());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_0920, &args![a]).bool());
    }

    #[test]
    fn fn_005d09e0_moves_the_flee_package_location() {
        let mut e = engine2();
        e.set_global(FLOAT_ARGUMENT, 2.5f32);
        e.register(V_PACKAGE_TARGET, |_, _| 0x4400u32.into_ret());
        e.register(V_PROCESS_PACKAGE, |_, _| 0x5500u32.into_ret());
        e.register(GET_FIELD_C0, |_, _| 0x99u32.into_ret());
        e.register(NOOP_TWO_WORDS, |_, _| Ret::default());
        e.register(PACKAGE_GET_LOCATION, |_, a| (a[0] + 0x2c).into_ret());
        e.register(PACKAGE_LOCATION_SET_REFERENCE, |_, _| Ret::default());
        e.register(PACKAGE_LOCATION_SET_CELL, |_, _| Ret::default());
        e.register(PACKAGE_SET_BYTE_80, |_, _| Ret::default());
        let process = object_with(&mut e, &[(0x27c, V_PROCESS_PACKAGE)]);
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        let actor = object_with(
            &mut e,
            &[(0x428, V_PACKAGE_TARGET), (0x410, V_ACTOR_RECORD)],
        );
        let a = command(&mut e, actor);
        // The flee package gets the reference as its location.
        parse_gives(&mut e, true, &[0x1111, 0x2222]);
        start_log(&mut e);
        assert!(e.call(0x005d_09e0, &args![a]).bool());
        assert_eq!(calls(&e, PARSE_PARAMETERS)[0].len(), 9);
        assert_eq!(calls(&e, GET_FIELD_C0), vec![vec![0x4400]]);
        assert_eq!(calls(&e, NOOP_TWO_WORDS), vec![vec![0x4400, 0x99, 0]]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[1],
            vec![0x5500, 0, RTTI_TES_PACKAGE, RTTI_FLEE_PACKAGE, 0]
        );
        assert_eq!(
            calls(&e, PACKAGE_LOCATION_SET_REFERENCE),
            vec![vec![0x5500 + 0x2c, 0x2222]]
        );
        assert!(calls(&e, PACKAGE_LOCATION_SET_CELL).is_empty());
        assert_eq!(calls(&e, PACKAGE_SET_BYTE_80), vec![vec![0x5500, 0]]);
        // Without a reference the cell is used.
        parse_gives(&mut e, true, &[0x1111, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_09e0, &args![a]).bool());
        assert_eq!(
            calls(&e, PACKAGE_LOCATION_SET_CELL),
            vec![vec![0x5500 + 0x2c, 0x1111]]
        );
        assert_eq!(calls(&e, PACKAGE_SET_BYTE_80).len(), 1);
        // Neither: the location is looked up and left alone.
        parse_gives(&mut e, true, &[0, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_09e0, &args![a]).bool());
        assert!(calls(&e, PACKAGE_SET_BYTE_80).is_empty());
        // A package that is no flee package.
        parse_gives(&mut e, true, &[0x1111, 0x2222]);
        e.register(DYNAMIC_CAST, |_, a| {
            if a[3] == RTTI_FLEE_PACKAGE {
                Ret::default()
            } else {
                a[0].into_ret()
            }
        });
        start_log(&mut e);
        assert!(e.call(0x005d_09e0, &args![a]).bool());
        assert!(calls(&e, PACKAGE_GET_LOCATION).is_empty());
        // No package target: slot 0x410 gets eight words with the two floats.
        let idle = object_with(&mut e, &[(0x428, V_FALSE), (0x410, V_ACTOR_RECORD)]);
        let b = command(&mut e, idle);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_09e0, &args![b]).bool());
        assert_eq!(
            calls(&e, V_ACTOR_RECORD),
            vec![vec![
                idle,
                0,
                0,
                0,
                1,
                0x1111,
                0x2222,
                2.5f32.to_bits(),
                2.5f32.to_bits()
            ]]
        );
        // The player, no reference, an actor without process, no actor, bad
        // parameters.
        let player = set_player(&mut e);
        let c = command(&mut e, player);
        start_log(&mut e);
        assert!(e.call(0x005d_09e0, &args![c]).bool());
        assert!(calls(&e, DYNAMIC_CAST).is_empty());
        let none = command(&mut e, 0);
        assert!(e.call(0x005d_09e0, &args![none]).bool());
        e.register(GET_PROCESS, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_09e0, &args![a]).bool());
        assert!(calls(&e, V_ACTOR_RECORD).is_empty());
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x005d_09e0, &args![a]).bool());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_09e0, &args![a]).bool());
    }

    // ---- Third batch -----------------------------------------------------------------

    // Fake virtual functions of the third batch.
    const V_NONZERO: u32 = 0x0900_0030;
    const V_ITEM: u32 = 0x0900_0031;
    const V_ATTRIBUTE: u32 = 0x0900_0032;
    const V_HIT_INITIALISED: u32 = 0x0900_0033;
    const V_HIT_FAILED: u32 = 0x0900_0034;
    const V_HIT_DESTROY: u32 = 0x0900_0035;
    const V_MODEL_PATH: u32 = 0x0900_0036;
    const V_POSITION: u32 = 0x0900_0037;

    /// [`engine2`] with the pages of the movie globals mapped.
    fn engine3() -> Engine {
        let mut e = engine2();
        e.map(0x0126_f000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.register(V_NONZERO, |_, _| 1u32.into_ret());
        e.register(V_HIT_INITIALISED, |_, _| true.into_ret());
        e.register(V_HIT_FAILED, |_, _| false.into_ret());
        e.register(V_HIT_DESTROY, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_005d0b80_stores_the_byte_at_0x80() {
        let mut e = engine3();
        let package = object(&mut e);
        e.call(0x005d_0b80, &args![package, 7u8]);
        assert_eq!(e.mem.u8(package + 0x80), 7);
        e.call(0x005d_0b80, &args![package, 0u8]);
        assert_eq!(e.mem.u8(package + 0x80), 0);
    }

    #[test]
    fn fn_005d0ba0_calls_the_package_target_with_the_two_integers() {
        let mut e = engine3();
        e.register(V_PACKAGE_TARGET, |_, _| 0x4400u32.into_ret());
        e.register(NOOP_TWO_WORDS, |_, _| Ret::default());
        e.register(GET_PROCESS, |_, _| 0x5500u32.into_ret());
        let actor = object_with(&mut e, &[(0x428, V_PACKAGE_TARGET)]);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[7, 0xffff_fffe]);
        start_log(&mut e);
        assert!(e.call(0x005d_0ba0, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(
            calls(&e, NOOP_TWO_WORDS),
            vec![vec![0x4400, 7, (-2.0f32).to_bits()]]
        );
        // No package target.
        e.register(V_PACKAGE_TARGET, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_0ba0, &args![a]).bool());
        assert!(calls(&e, NOOP_TWO_WORDS).is_empty());
        // No process.
        e.register(V_PACKAGE_TARGET, |_, _| 0x4400u32.into_ret());
        e.register(GET_PROCESS, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_0ba0, &args![a]).bool());
        assert!(calls(&e, V_PACKAGE_TARGET).is_empty());
        // Not an actor, no reference.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x005d_0ba0, &args![a]).bool());
        let none = command(&mut e, 0);
        assert!(e.call(0x005d_0ba0, &args![none]).bool());
        // The player: nothing, not even the cast.
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        let player = set_player(&mut e);
        let b = command(&mut e, player);
        start_log(&mut e);
        assert!(e.call(0x005d_0ba0, &args![b]).bool());
        assert!(calls(&e, DYNAMIC_CAST).is_empty());
        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_0ba0, &args![a]).bool());
    }

    #[test]
    fn fn_005d0c70_returns_the_offers_service_condition() {
        check_condition_only(0x005d_0c70, GET_OFFERS_SERVICE_CONDITION, true);
    }

    #[test]
    fn mod_barter_gold_parses_for_actors_and_echoes_the_base_gold() {
        let mut e = engine3();
        e.register(ACTOR_GET_BARTER_GOLD_BASE, |_, _| 150u32.into_ret());
        let actor = object(&mut e);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        assert!(e.call(0x005d_0c90, &args![a]).bool());
        assert_parsed(&e, actor);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005d_0c90, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_ACTOR_BARTER_GOLD, 150]]
        );
        assert_eq!(calls(&e, ACTOR_GET_BARTER_GOLD_BASE), vec![vec![actor]]);
        // Parameters that do not parse: false, nothing printed.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_0c90, &args![a]).bool());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // Not an actor: nothing is parsed, the echo still asks (with null).
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_0c90, &args![a]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        assert_eq!(calls(&e, ACTOR_GET_BARTER_GOLD_BASE), vec![vec![0]]);
    }

    #[test]
    fn set_barter_gold_stores_the_low_16_bits_in_the_npc_base_data() {
        let mut e = engine3();
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(ACTOR_BASE_DATA_SET_BARTER_GOLD, |_, _| Ret::default());
        e.register(ACTOR_BASE_DATA_GET_BARTER_GOLD, |_, _| 0x1234u32.into_ret());
        let actor = object(&mut e);
        e.mem.set_u32(actor + 0x20, 0x9999);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[0x1_2345]);
        start_log(&mut e);
        assert!(e.call(0x005d_0d30, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[1],
            vec![0x9999, 0, RTTI_TES_BOUND_OBJECT, RTTI_TES_NPC, 0]
        );
        assert_eq!(
            calls(&e, ACTOR_BASE_DATA_SET_BARTER_GOLD),
            vec![vec![0x9999 + 0x30, 0x2345]]
        );
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005d_0d30, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_BASE_BARTER_GOLD, 0x1234]]
        );
        assert_eq!(
            calls(&e, ACTOR_BASE_DATA_GET_BARTER_GOLD),
            vec![vec![0x9999 + 0x30]]
        );
        // A base form that is no NPC: nothing is parsed or stored; the echo
        // still reads the base data of the (null) NPC.
        e.register(DYNAMIC_CAST, |_, a| {
            if a[0] == 0x9999 {
                Ret::default()
            } else {
                a[0].into_ret()
            }
        });
        start_log(&mut e);
        assert!(e.call(0x005d_0d30, &args![a]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        assert!(calls(&e, ACTOR_BASE_DATA_SET_BARTER_GOLD).is_empty());
        assert_eq!(calls(&e, ACTOR_BASE_DATA_GET_BARTER_GOLD), vec![vec![0x30]]);
        // Not an actor: nothing at all. Bad parameters: false.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_0d30, &args![a]).bool());
        assert!(calls(&e, GET_BASE_FORM).is_empty());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_0d30, &args![a]).bool());
    }

    #[test]
    fn fn_005d0e10_returns_the_barter_gold_condition() {
        check_condition_only(0x005d_0e10, GET_BARTER_GOLD_CONDITION, true);
    }

    /// Doubles for `005d0e30`: the actor's base form is `0x7000`, the
    /// inventory changes `0x6000`, the actor has a container and slot `0x218`
    /// answers true; slot `0x360` answers `player_actor`. Returns the actor.
    fn set_level_scene(e: &mut Engine, player_actor: bool) -> u32 {
        e.register(GET_BASE_FORM_OF_REFERENCE, |_, _| 0x7000u32.into_ret());
        e.register(GET_BASE_FORM, |_, _| 0x7000u32.into_ret());
        e.register(REFERENCE_HAS_CONTAINER, |_, _| 1u32.into_ret());
        e.register(GET_INVENTORY_CHANGES, |_, _| 0x6000u32.into_ret());
        for function in [
            ACTOR_BASE_DATA_SET_LEVEL,
            ACTOR_BASE_DATA_SET_FIELD_0E,
            ACTOR_BASE_DATA_SET_FIELD_10,
            ACTOR_BASE_DATA_SET_FLAG_BIT,
            INVENTORY_REMOVE_ALL_OBJECTS_WORN,
            INVENTORY_CHANGES_FN_004D17A0,
            INVENTORY_CHANGES_FN_004D1440,
            NPC_INIT_DEFAULT_WORN,
            CREATURE_INIT_DEFAULT_WORN,
            CHARACTER_PROGRESSION_FORCE_LEVEL,
            NPC_INIT_VALUES,
        ] {
            e.register(function, |_, _| Ret::default());
        }
        e.register(PLAYER_CHARACTER_PROGRESSION, |_, _| 0x8888u32.into_ret());
        set_player(e);
        let slot_360 = if player_actor { V_TRUE } else { V_FALSE };
        object_with(e, &[(0x360, slot_360), (0x218, V_TRUE)])
    }

    #[test]
    fn fn_005d0e30_stores_the_levels_and_refills_the_worn_items() {
        let mut e = engine3();
        let actor = set_level_scene(&mut e, false);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[10, 1, 3, 20]);
        start_log(&mut e);
        assert!(e.call(0x005d_0e30, &args![a]).bool());
        assert_eq!(calls(&e, PARSE_PARAMETERS)[0].len(), 11);
        assert_eq!(calls(&e, ACTOR_BASE_DATA_SET_LEVEL), vec![vec![0x7030, 10]]);
        assert_eq!(
            calls(&e, ACTOR_BASE_DATA_SET_FIELD_0E),
            vec![vec![0x7030, 3]]
        );
        assert_eq!(
            calls(&e, ACTOR_BASE_DATA_SET_FIELD_10),
            vec![vec![0x7030, 20]]
        );
        assert_eq!(
            calls(&e, ACTOR_BASE_DATA_SET_FLAG_BIT),
            vec![vec![0x7030, 0x80, 1, 1]; 3]
        );
        assert_eq!(
            calls(&e, INVENTORY_REMOVE_ALL_OBJECTS_WORN),
            vec![vec![0x6000, 0, 0]]
        );
        assert_eq!(calls(&e, INVENTORY_CHANGES_FN_004D17A0), vec![vec![0x6000]]);
        assert_eq!(calls(&e, INVENTORY_CHANGES_FN_004D1440), vec![vec![0x6000]]);
        assert_eq!(
            calls(&e, NPC_INIT_DEFAULT_WORN),
            vec![vec![0x7000, actor, 1, 1, 0, 1]]
        );
        assert!(calls(&e, CREATURE_INIT_DEFAULT_WORN).is_empty());
        assert_eq!(calls(&e, NPC_INIT_VALUES), vec![vec![0x7000, 0]]);
        assert!(calls(&e, CHARACTER_PROGRESSION_FORCE_LEVEL).is_empty());

        // Defaults: only the level is stored, and no flag.
        parse_gives(&mut e, true, &[10, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_0e30, &args![a]).bool());
        assert_eq!(calls(&e, ACTOR_BASE_DATA_SET_LEVEL).len(), 1);
        assert!(calls(&e, ACTOR_BASE_DATA_SET_FIELD_0E).is_empty());
        assert!(calls(&e, ACTOR_BASE_DATA_SET_FIELD_10).is_empty());
        assert!(calls(&e, ACTOR_BASE_DATA_SET_FLAG_BIT).is_empty());

        // A creature base form.
        e.register(DYNAMIC_CAST, |_, a| {
            if a[3] == RTTI_TES_NPC && a[2] == RTTI_TES_ACTOR_BASE {
                Ret::default()
            } else {
                a[0].into_ret()
            }
        });
        start_log(&mut e);
        assert!(e.call(0x005d_0e30, &args![a]).bool());
        assert!(calls(&e, NPC_INIT_DEFAULT_WORN).is_empty());
        assert_eq!(
            calls(&e, CREATURE_INIT_DEFAULT_WORN),
            vec![vec![0x7000, actor, 1, 1, 1]]
        );
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[1],
            vec![0x7000, 0, RTTI_TES_ACTOR_BASE, RTTI_TES_NPC, 0]
        );
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[2],
            vec![0x7000, 0, RTTI_TES_ACTOR_BASE, RTTI_TES_CREATURE, 0]
        );
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());

        // No container, or no inventory changes: the worn items stay.
        e.register(REFERENCE_HAS_CONTAINER, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_0e30, &args![a]).bool());
        assert!(calls(&e, INVENTORY_REMOVE_ALL_OBJECTS_WORN).is_empty());
        e.register(REFERENCE_HAS_CONTAINER, |_, _| 1u32.into_ret());
        e.register(GET_INVENTORY_CHANGES, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_0e30, &args![a]).bool());
        assert!(calls(&e, INVENTORY_REMOVE_ALL_OBJECTS_WORN).is_empty());

        // Not an actor: succeeds without parsing. Bad parameters: false.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_0e30, &args![a]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_0e30, &args![a]).bool());
    }

    #[test]
    fn fn_005d0e30_forces_the_players_level() {
        let mut e = engine3();
        let actor = set_level_scene(&mut e, true);
        let player = set_player(&mut e);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[0x1_0010]);
        start_log(&mut e);
        assert!(e.call(0x005d_0e30, &args![a]).bool());
        assert_eq!(calls(&e, PLAYER_CHARACTER_PROGRESSION), vec![vec![player]]);
        assert_eq!(
            calls(&e, CHARACTER_PROGRESSION_FORCE_LEVEL),
            vec![vec![0x8888, 0x1_0010]]
        );
        assert!(calls(&e, ACTOR_BASE_DATA_SET_LEVEL).is_empty());
        assert!(calls(&e, NPC_INIT_VALUES).is_empty());
    }

    #[test]
    fn fn_005d1070_sets_base_flag_0x80() {
        let mut e = engine3();
        e.register(ACTOR_BASE_DATA_SET_FLAG_BIT, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x005d_1070, &args![0x7030u32, 1u8]);
        assert_eq!(
            calls(&e, ACTOR_BASE_DATA_SET_FLAG_BIT),
            vec![vec![0x7030, 0x80, 1, 1]]
        );
    }

    #[test]
    fn reset_fall_damage_timer_reads_the_controller_of_actors() {
        let mut e = engine3();
        let controller = object(&mut e);
        e.mem.set_f32(controller + 0x544, 3.5);
        e.register_double(GET_CHAR_CONTROLLER, move |_, _| controller.into_ret());
        let actor = object(&mut e);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005d_10a0, &args![a]).bool());
        assert_eq!(
            calls(&e, GET_CHAR_CONTROLLER),
            vec![vec![actor], vec![actor]]
        );
        // No controller: asked once.
        e.register(GET_CHAR_CONTROLLER, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_10a0, &args![a]).bool());
        assert_eq!(calls(&e, GET_CHAR_CONTROLLER).len(), 1);
        // Not an actor.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_10a0, &args![a]).bool());
        assert!(calls(&e, GET_CHAR_CONTROLLER).is_empty());
    }

    #[test]
    fn fn_005d10f0_returns_the_float_at_0x544() {
        let mut e = engine3();
        let controller = object(&mut e);
        e.mem.set_f32(controller + 0x544, 3.5);
        assert_eq!(e.call(0x005d_10f0, &args![controller]).f32(), 3.5);
    }

    #[test]
    fn fn_005d1110_returns_the_is_xbox_condition() {
        check_condition_only(0x005d_1110, IS_XBOX_CONDITION, true);
    }

    #[test]
    fn fn_005d1130_returns_the_is_ps3_condition() {
        check_condition_only(0x005d_1130, IS_PS3_CONDITION, true);
    }

    #[test]
    fn fn_005d1150_returns_the_is_pc_condition() {
        check_condition_only(0x005d_1150, IS_PC_CONDITION, true);
    }

    #[test]
    fn fn_005d1170_returns_the_unnamed_condition() {
        check_condition_only(0x005d_1170, CONDITION_005A2C30, true);
    }

    #[test]
    fn fn_005d1190_returns_the_is_time_passing_condition() {
        check_condition_only(0x005d_1190, IS_TIME_PASSING_CONDITION, true);
    }

    #[test]
    fn mod_pc_attribute_adds_to_the_players_attributes_5_to_11() {
        let mut e = engine3();
        let player = set_player(&mut e);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table, V_ATTRIBUTE);
        e.mem.set_u32(player + 0xa4, table);
        e.register(V_ATTRIBUTE, |_, _| 40u32.into_ret());
        e.register(PLAYER_SET_ACTOR_BASE_VALUE, |_, _| Ret::default());
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[5, 0xffff_fffd]);
        start_log(&mut e);
        assert!(e.call(0x005d_11b0, &args![a]).bool());
        assert_parsed(&e, 0);
        assert_eq!(calls(&e, V_ATTRIBUTE), vec![vec![player + 0xa4, 5]]);
        assert_eq!(
            calls(&e, PLAYER_SET_ACTOR_BASE_VALUE),
            vec![vec![player, 5, 37]]
        );
        // Out of range: nothing.
        parse_gives(&mut e, true, &[4, 1]);
        start_log(&mut e);
        assert!(e.call(0x005d_11b0, &args![a]).bool());
        parse_gives(&mut e, true, &[12, 1]);
        assert!(e.call(0x005d_11b0, &args![a]).bool());
        assert!(calls(&e, PLAYER_SET_ACTOR_BASE_VALUE).is_empty());
        // Bad parameters.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_11b0, &args![a]).bool());
    }

    #[test]
    fn fn_005d1260_accepts_attributes_5_to_11() {
        let mut e = engine3();
        for (attribute, expected) in [
            (-1i32, false),
            (4, false),
            (5, true),
            (11, true),
            (12, false),
        ] {
            assert_eq!(
                e.call(0x005d_1260, &args![attribute]).bool(),
                expected,
                "attribute {attribute}"
            );
        }
    }

    #[test]
    fn fn_005d1290_hands_the_weapon_and_the_float_to_the_actor() {
        let mut e = engine3();
        e.register(V_ITEM, |_, _| 0x3300u32.into_ret());
        let process = object_with(&mut e, &[(0x148, V_ITEM)]);
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(V_ACTOR_RECORD, |_, _| Ret::default());
        let actor = object_with(&mut e, &[(0x3c8, V_ACTOR_RECORD)]);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[2.5f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_1290, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(
            calls(&e, V_ACTOR_RECORD),
            vec![vec![actor, 0x3300, 2.5f32.to_bits(), 1]]
        );
        // No weapon.
        e.register(V_ITEM, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_1290, &args![a]).bool());
        assert!(calls(&e, V_ACTOR_RECORD).is_empty());
        // Not an actor: false. Bad parameters: false.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        assert!(!e.call(0x005d_1290, &args![a]).bool());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_1290, &args![a]).bool());
    }

    #[test]
    fn get_weapon_health_prints_the_health_or_the_missing_weapon() {
        let mut e = engine3();
        e.register(V_ITEM, |_, _| 0x3300u32.into_ret());
        let process = object_with(&mut e, &[(0x148, V_ITEM)]);
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(ITEM_CHANGE_GET_ITEM_HEALTH, |_, _| 87.5f64.into_ret());
        let actor = object(&mut e);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005d_1340, &args![a]).bool());
        assert_eq!(
            calls(&e, ITEM_CHANGE_GET_ITEM_HEALTH),
            vec![vec![0x3300, 0]]
        );
        let mut expected = vec![MSG_WEAPON_HEALTH];
        expected.extend(double_words(87.5));
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![expected]);
        // No weapon: the name of the actor.
        e.register(V_ITEM, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_1340, &args![a]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_NO_WEAPON, 0xaaaa]]);
        // Not an actor: false, nothing printed.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(!e.call(0x005d_1340, &args![a]).bool());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn fn_005d13e0_sets_the_players_flags() {
        let mut e = engine3();
        let player = set_player(&mut e);
        let a = command(&mut e, 0);
        let flags = |e: &Engine| (e.mem.u8(player + 0x66d), e.mem.u8(player + 0x66e));
        // Defaults (0, 1, 0): flag 0 cleared, byte 1, flag 1 left alone.
        e.mem.set_u8(player + 0x66d, 0b11);
        parse_gives(&mut e, true, &[]);
        start_log(&mut e);
        assert!(e.call(0x005d_13e0, &args![a]).bool());
        assert_parsed(&e, 0);
        assert_eq!(flags(&e), (0b10, 1));
        // (0, 0, 1): byte 0, flag 1 set.
        e.mem.set_u8(player + 0x66d, 0b01);
        parse_gives(&mut e, true, &[0, 0, 1]);
        assert!(e.call(0x005d_13e0, &args![a]).bool());
        assert_eq!(flags(&e), (0b10, 0));
        // (1, 0, 1): flag 0 set, byte 1, flag 1 set.
        e.mem.set_u8(player + 0x66d, 0);
        e.mem.set_u8(player + 0x66e, 0);
        parse_gives(&mut e, true, &[1, 0, 1]);
        assert!(e.call(0x005d_13e0, &args![a]).bool());
        assert_eq!(flags(&e), (0b11, 1));
        // (1, 1, 0): flag 1 cleared.
        parse_gives(&mut e, true, &[1, 1, 0]);
        assert!(e.call(0x005d_13e0, &args![a]).bool());
        assert_eq!(flags(&e), (0b01, 1));
        // Bad parameters.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_13e0, &args![a]).bool());
    }

    #[test]
    fn fn_005d14d0_and_fn_005d1530_set_and_clear_their_bits() {
        let mut e = engine3();
        let player = object(&mut e);
        e.mem.set_u8(player + 0x66d, 0b1000_0100);
        e.call(0x005d_14d0, &args![player, 1u8]);
        assert_eq!(e.mem.u8(player + 0x66d), 0b1000_0101);
        e.call(0x005d_1530, &args![player, 5u8]);
        assert_eq!(e.mem.u8(player + 0x66d), 0b1000_0111);
        e.call(0x005d_14d0, &args![player, 0u8]);
        assert_eq!(e.mem.u8(player + 0x66d), 0b1000_0110);
        e.call(0x005d_1530, &args![player, 0u8]);
        assert_eq!(e.mem.u8(player + 0x66d), 0b1000_0100);
    }

    #[test]
    fn fn_005d1590_stores_the_byte_at_0x66e() {
        let mut e = engine3();
        let player = object(&mut e);
        e.call(0x005d_1590, &args![player, 1u8]);
        assert_eq!(e.mem.u8(player + 0x66e), 1);
    }

    #[test]
    fn fn_005d15b0_returns_the_armor_rating_upper_condition() {
        check_condition_only(0x005d_15b0, GET_ARMOR_RATING_UPPER_CONDITION, true);
    }

    /// Doubles for the movie command; the audio singleton is `0xa000`, the
    /// I/O manager `0x7777` and the movie player `0x6666`.
    fn movie_scene(e: &mut Engine, audio_flag: bool) {
        e.register(BS_AUDIO_Q_INSTANCE, |_, _| 0xa000u32.into_ret());
        e.register_double(AUDIO_GET_FLAG, move |_, _| audio_flag.into_ret());
        for function in [
            AUDIO_SET_MULTI_THREADED,
            AUDIO_MUTE_TYPE,
            AUDIO_FN_00AD84B0,
            IO_MANAGER_BEFORE_MOVIE,
            IO_MANAGER_AFTER_MOVIE,
            MOVIE_PLAYER_PLAY,
        ] {
            e.register(function, |_, _| Ret::default());
        }
        e.set_global(IO_MANAGER, 0x7777u32);
        e.set_global(MOVIE_PLAYER, 0x6666u32);
    }

    /// The addresses of the logged calls, in order.
    fn call_order(e: &Engine, only: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .filter(|address| only.contains(address))
            .collect()
    }

    #[test]
    fn fn_005d15d0_plays_the_movie_between_the_audio_and_io_calls() {
        let mut e = engine3();
        movie_scene(&mut e, false);
        let a = command(&mut e, 0);
        // Buffer text, then the four flags: (1, 1, 0, 1).
        parse_gives(&mut e, true, &[0x4142_4344, 1, 1, 0, 1]);
        start_log(&mut e);
        assert!(e.call(0x005d_15d0, &args![a]).bool());
        let parse = &calls(&e, PARSE_PARAMETERS)[0];
        assert_eq!(parse[..7], [1, 2, 8, 0, 0, 5, 6]);
        let buffer = parse[7];
        assert_eq!(
            calls(&e, MOVIE_PLAYER_PLAY),
            vec![vec![0x6666, buffer, 1, 0, 0, 0, 1, 0, 1, 0]]
        );
        assert_eq!(
            calls(&e, AUDIO_MUTE_TYPE),
            vec![vec![0xa000, 0xffff_ffff, 0, 6000]]
        );
        assert_eq!(
            calls(&e, AUDIO_FN_00AD84B0),
            vec![vec![0xa000, 0xffff_ffff, 0]]
        );
        assert!(calls(&e, AUDIO_SET_MULTI_THREADED).is_empty());
        assert_eq!(calls(&e, IO_MANAGER_BEFORE_MOVIE), vec![vec![0x7777]]);
        assert_eq!(calls(&e, IO_MANAGER_AFTER_MOVIE), vec![vec![0x7777]]);
        assert_eq!(
            call_order(
                &e,
                &[
                    AUDIO_MUTE_TYPE,
                    IO_MANAGER_BEFORE_MOVIE,
                    MOVIE_PLAYER_PLAY,
                    IO_MANAGER_AFTER_MOVIE,
                    AUDIO_FN_00AD84B0
                ]
            ),
            vec![
                AUDIO_MUTE_TYPE,
                IO_MANAGER_BEFORE_MOVIE,
                MOVIE_PLAYER_PLAY,
                IO_MANAGER_AFTER_MOVIE,
                AUDIO_FN_00AD84B0
            ]
        );
        // The buffer is freed after the call; the name was parsed into it.
        // Second flag zero and the audio flag clear: multi-threading on and
        // off around the movie.
        parse_gives(&mut e, true, &[0, 0, 0, 1, 1]);
        start_log(&mut e);
        assert!(e.call(0x005d_15d0, &args![a]).bool());
        assert_eq!(
            calls(&e, AUDIO_SET_MULTI_THREADED),
            vec![vec![0xa000, 1], vec![0xa000, 0]]
        );
        assert!(calls(&e, AUDIO_MUTE_TYPE).is_empty());
        assert!(calls(&e, AUDIO_FN_00AD84B0).is_empty());
        let play = &calls(&e, MOVIE_PLAYER_PLAY)[0];
        assert_eq!(play[2..], [0, 0, 0, 0, 1, 0, 0, 1]);
        // With the audio flag set nothing happens to the audio.
        movie_scene(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_15d0, &args![a]).bool());
        assert!(calls(&e, AUDIO_SET_MULTI_THREADED).is_empty());
        assert_eq!(calls(&e, MOVIE_PLAYER_PLAY).len(), 1);
        // Bad parameters: false, no movie.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_15d0, &args![a]).bool());
        assert!(calls(&e, MOVIE_PLAYER_PLAY).is_empty());
    }

    #[test]
    fn fn_005d1720_and_fn_005d1740_pass_minus_one_to_the_audio_calls() {
        let mut e = engine3();
        e.register(AUDIO_MUTE_TYPE, |_, _| Ret::default());
        e.register(AUDIO_FN_00AD84B0, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x005d_1720, &args![0xa000u32, 3u32, 4u32]);
        e.call(0x005d_1740, &args![0xa000u32, 5u32]);
        assert_eq!(
            calls(&e, AUDIO_MUTE_TYPE),
            vec![vec![0xa000, 0xffff_ffff, 3, 4]]
        );
        assert_eq!(
            calls(&e, AUDIO_FN_00AD84B0),
            vec![vec![0xa000, 0xffff_ffff, 5]]
        );
    }

    #[test]
    fn fn_005d1760_asks_the_cell_ownership_condition() {
        let mut e = engine3();
        e.register(GET_CELL_OWNERSHIP_CONDITION, |_, _| true.into_ret());
        e.register(GET_BASE_FORM, |_, _| 0x4242u32.into_ret());
        set_player(&mut e);
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.mem.set_f64(a.result.addr(), 9.0);
        parse_gives(&mut e, true, &[0x1111, 0x2222]);
        start_log(&mut e);
        assert!(e.call(0x005d_1760, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert_eq!(
            calls(&e, GET_CELL_OWNERSHIP_CONDITION),
            vec![vec![this_obj, 0x1111, 0x2222, a.result.addr()]]
        );
        // The second word defaults to the player's base form.
        parse_gives(&mut e, true, &[0x1111]);
        start_log(&mut e);
        assert!(e.call(0x005d_1760, &args![a]).bool());
        assert_eq!(
            calls(&e, GET_CELL_OWNERSHIP_CONDITION),
            vec![vec![this_obj, 0x1111, 0x4242, a.result.addr()]]
        );
        // Bad parameters: the result is still cleared.
        e.mem.set_f64(a.result.addr(), 9.0);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_1760, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert!(calls(&e, GET_CELL_OWNERSHIP_CONDITION).is_empty());
    }

    #[test]
    fn fn_005d17f0_calls_the_cell_with_the_second_word() {
        let mut e = engine3();
        e.register(CELL_FN_00546BF0, |_, _| Ret::default());
        e.register(GET_BASE_FORM, |_, _| 0x4242u32.into_ret());
        set_player(&mut e);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0x1111, 0x2222]);
        start_log(&mut e);
        assert!(e.call(0x005d_17f0, &args![a]).bool());
        assert_parsed(&e, 0);
        assert_eq!(calls(&e, CELL_FN_00546BF0), vec![vec![0x1111, 0x2222]]);
        parse_gives(&mut e, true, &[0x1111]);
        start_log(&mut e);
        assert!(e.call(0x005d_17f0, &args![a]).bool());
        assert_eq!(calls(&e, CELL_FN_00546BF0), vec![vec![0x1111, 0x4242]]);
        // No first word: nothing is called.
        parse_gives(&mut e, true, &[0, 0x2222]);
        start_log(&mut e);
        assert!(e.call(0x005d_17f0, &args![a]).bool());
        assert!(calls(&e, CELL_FN_00546BF0).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_17f0, &args![a]).bool());
    }

    #[test]
    fn fn_005d1870_succeeds_after_the_unnamed_condition() {
        check_condition_only(0x005d_1870, UNKNOWN_CONDITION_005A2A50, false);
    }

    #[test]
    fn fn_005d1890_copies_the_string_between_forms() {
        let mut e = engine3();
        e.register(STRING_TEXT_OR_EMPTY, |_, _| 0x5050u32.into_ret());
        e.register(STRING_SET_TEXT, |_, _| Ret::default());
        e.register(V_RECORD_A, |_, _| Ret::default());
        let target = object_with(&mut e, &[(0x48, V_RECORD_A)]);
        let source = object(&mut e);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[target, source]);
        start_log(&mut e);
        assert!(e.call(0x005d_1890, &args![a]).bool());
        assert_parsed(&e, 0);
        assert_eq!(calls(&e, STRING_TEXT_OR_EMPTY), vec![vec![source + 0x18]]);
        assert_eq!(
            calls(&e, STRING_SET_TEXT),
            vec![vec![target + 0x18, 0x5050]]
        );
        assert_eq!(calls(&e, V_RECORD_A), vec![vec![target, 4]]);
        // One of the forms missing: nothing.
        parse_gives(&mut e, true, &[target, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_1890, &args![a]).bool());
        parse_gives(&mut e, true, &[0, source]);
        assert!(e.call(0x005d_1890, &args![a]).bool());
        assert!(calls(&e, STRING_SET_TEXT).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_1890, &args![a]).bool());
    }

    #[test]
    fn fn_005d1920_copies_the_string_into_the_base_form() {
        let mut e = engine3();
        e.register(STRING_TEXT_OR_EMPTY, |_, _| 0x5050u32.into_ret());
        e.register(STRING_SET_TEXT, |_, _| Ret::default());
        e.register(V_RECORD_A, |_, _| Ret::default());
        let base = object_with(&mut e, &[(0x48, V_RECORD_A)]);
        e.register_double(GET_BASE_FORM_OF_REFERENCE, move |_, _| base.into_ret());
        e.register_double(GET_BASE_FORM, move |_, _| base.into_ret());
        e.register(FORM_TYPE, |_, _| 0x16u32.into_ret());
        let form = object(&mut e);
        // An actor reference: the string at +0xd0, change flags 0x20.
        let actor = object_with(&mut e, &[(0x100, V_TRUE)]);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[form]);
        start_log(&mut e);
        assert!(e.call(0x005d_1920, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, STRING_TEXT_OR_EMPTY), vec![vec![form + 0x18]]);
        assert_eq!(calls(&e, STRING_SET_TEXT), vec![vec![base + 0xd0, 0x5050]]);
        assert_eq!(calls(&e, V_RECORD_A), vec![vec![base, 0x20]]);
        // Another reference whose base form has type 0x16: +0x30, flags 4.
        let other = object_with(&mut e, &[(0x100, V_FALSE)]);
        let b = command(&mut e, other);
        start_log(&mut e);
        assert!(e.call(0x005d_1920, &args![b]).bool());
        assert_eq!(calls(&e, FORM_TYPE), vec![vec![base]]);
        assert_eq!(calls(&e, STRING_SET_TEXT), vec![vec![base + 0x30, 0x5050]]);
        assert_eq!(calls(&e, V_RECORD_A), vec![vec![base, 4]]);
        // Another type: nothing.
        e.register(FORM_TYPE, |_, _| 0x17u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_1920, &args![b]).bool());
        assert!(calls(&e, STRING_SET_TEXT).is_empty());
        // No form given, no reference, bad parameters.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_1920, &args![a]).bool());
        assert!(calls(&e, V_TRUE).is_empty());
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_1920, &args![none]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_1920, &args![a]).bool());
    }

    #[test]
    fn fn_005d1a30_succeeds_after_its_condition() {
        check_condition_only(0x005d_1a30, CONDITION_005A2F60, false);
    }

    #[test]
    fn fn_005d1a50_succeeds_after_is_running() {
        check_condition_only(0x005d_1a50, IS_RUNNING_CONDITION, false);
    }

    #[test]
    fn fn_005d1a70_succeeds_after_get_friend_hit() {
        check_condition_only(0x005d_1a70, GET_FRIEND_HIT_CONDITION, false);
    }

    #[test]
    fn fn_005d1a90_succeeds_after_is_in_combat() {
        check_condition_only(0x005d_1a90, IS_IN_COMBAT_CONDITION, false);
    }

    #[test]
    fn set_package_duration_sets_it_on_the_process_and_echoes() {
        let mut e = engine3();
        e.register(V_RECORD_A, |_, _| Ret::default());
        let process = object_with(&mut e, &[(0x538, V_RECORD_A)]);
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        let actor = object(&mut e);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[1.5f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_1ab0, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, V_RECORD_A), vec![vec![process, 1.5f32.to_bits()]]);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005d_1ab0, &args![a]).bool());
        let mut expected = vec![MSG_PACKAGE_DURATION];
        expected.extend(double_words(1.5));
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![expected]);
        // No process: the echo still prints.
        e.register(GET_PROCESS, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_1ab0, &args![a]).bool());
        assert!(calls(&e, V_RECORD_A).is_empty());
        assert_eq!(calls(&e, CONSOLE_PRINT).len(), 1);
        // Not an actor: parsed, nothing else. Bad parameters: false.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_1ab0, &args![a]).bool());
        assert_eq!(calls(&e, PARSE_PARAMETERS).len(), 1);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_1ab0, &args![a]).bool());
    }

    /// Doubles shared by the two magic effect commands: the cell of a
    /// reference is `0xcc00` and is loaded when `loaded`; the `TES` singleton
    /// is `0x7100`; the process lists take temporary effects. Returns the
    /// player.
    fn magic_scene(e: &mut Engine, loaded: bool) -> u32 {
        e.register(GET_PARENT_CELL, |_, _| 0xcc00u32.into_ret());
        e.set_global(TES_SINGLETON, 0x7100u32);
        e.set_global(FLOAT_ARGUMENT, 2.5f32);
        e.register_double(TES_IS_CELL_LOADED, move |_, _| loaded.into_ret());
        e.register(PROCESS_LISTS_ADD_TEMP_EFFECT, |_, _| Ret::default());
        e.register(ALLOCATE, |_, a| (0x9000 + a[0]).into_ret());
        set_player(e)
    }

    /// A hit effect object: its virtual slot `0xc4` is `initialise`, slot 0
    /// the destructor double.
    fn hit_effect(e: &mut Engine, initialise: u32) -> u32 {
        object_with(e, &[(0xc4, initialise), (0, V_HIT_DESTROY)])
    }

    #[test]
    fn play_magic_shader_adds_the_effect_when_it_initialises() {
        let mut e = engine3();
        let player = magic_scene(&mut e, true);
        e.register(SCOPE_GUARD_CONSTRUCT, |_, _| Ret::default());
        e.register(SCOPE_GUARD_DESTRUCT, |_, _| Ret::default());
        let effect = hit_effect(&mut e, V_HIT_INITIALISED);
        e.register_double(MAGIC_SHADER_HIT_EFFECT_CONSTRUCT, move |_, _| {
            effect.into_ret()
        });
        let reference = object_with(&mut e, &[(0x1d0, V_NONZERO)]);
        let a = command(&mut e, reference);
        parse_gives(&mut e, true, &[0x3131]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_1b80, &args![a]).bool());
        assert_parsed(&e, reference);
        assert_eq!(calls(&e, TES_IS_CELL_LOADED), vec![vec![0x7100, 0xcc00, 0]]);
        let guard = &calls(&e, SCOPE_GUARD_CONSTRUCT)[0];
        assert_eq!(guard[1..], [0x32, 1, 0x0103_bcb8, 0x409c]);
        assert_eq!(calls(&e, SCOPE_GUARD_DESTRUCT), vec![vec![guard[0]]]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x6c]]);
        // The float defaults to the global.
        assert_eq!(
            calls(&e, MAGIC_SHADER_HIT_EFFECT_CONSTRUCT),
            vec![vec![0x9000 + 0x6c, reference, 0x3131, 2.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, PROCESS_LISTS_ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, effect]]
        );
        assert!(calls(&e, V_HIT_DESTROY).is_empty());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_SHADER_APPLIED_NAMED, 0xaaaa]]
        );
        // A given float; the reference defaults to the player.
        parse_gives(&mut e, true, &[0x3131, 1.0f32.to_bits()]);
        let b = command(&mut e, 0);
        e.mem.set_u32(player, e.mem.u32(reference));
        start_log(&mut e);
        assert!(e.call(0x005d_1b80, &args![b]).bool());
        assert_eq!(calls(&e, GET_PARENT_CELL), vec![vec![player]]);
        assert_eq!(
            calls(&e, MAGIC_SHADER_HIT_EFFECT_CONSTRUCT),
            vec![vec![0x9000 + 0x6c, player, 0x3131, 1.0f32.to_bits()]]
        );
        // Bad parameters.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_1b80, &args![a]).bool());
    }

    #[test]
    fn play_magic_shader_destroys_an_effect_that_fails_to_initialise() {
        let mut e = engine3();
        magic_scene(&mut e, true);
        e.register(SCOPE_GUARD_CONSTRUCT, |_, _| Ret::default());
        e.register(SCOPE_GUARD_DESTRUCT, |_, _| Ret::default());
        let effect = hit_effect(&mut e, V_HIT_FAILED);
        e.register_double(MAGIC_SHADER_HIT_EFFECT_CONSTRUCT, move |_, _| {
            effect.into_ret()
        });
        let reference = object_with(&mut e, &[(0x1d0, V_NONZERO)]);
        let a = command(&mut e, reference);
        parse_gives(&mut e, true, &[0x3131]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_1b80, &args![a]).bool());
        assert_eq!(calls(&e, V_HIT_DESTROY), vec![vec![effect, 1]]);
        assert!(calls(&e, PROCESS_LISTS_ADD_TEMP_EFFECT).is_empty());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_SHADER_FAILED_NAMED, 0xaaaa]]
        );
        assert_eq!(calls(&e, SCOPE_GUARD_DESTRUCT).len(), 1);
        // A reference without a name.
        e.register(GET_REFERENCE_NAME, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_1b80, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_SHADER_FAILED_REFERENCE]]
        );
    }

    #[test]
    fn play_magic_shader_does_nothing_without_a_loaded_cell_or_3d() {
        let mut e = engine3();
        magic_scene(&mut e, false);
        e.register(SCOPE_GUARD_CONSTRUCT, |_, _| Ret::default());
        let reference = object_with(&mut e, &[(0x1d0, V_NONZERO)]);
        let a = command(&mut e, reference);
        parse_gives(&mut e, true, &[0x3131]);
        start_log(&mut e);
        assert!(e.call(0x005d_1b80, &args![a]).bool());
        assert!(calls(&e, SCOPE_GUARD_CONSTRUCT).is_empty());
        e.register_double(TES_IS_CELL_LOADED, |_, _| true.into_ret());
        let no_3d = object_with(&mut e, &[(0x1d0, V_FALSE)]);
        let b = command(&mut e, no_3d);
        assert!(e.call(0x005d_1b80, &args![b]).bool());
        assert!(calls(&e, SCOPE_GUARD_CONSTRUCT).is_empty());
    }

    #[test]
    fn play_magic_effect_creates_a_particle_effect_for_a_reference_without_3d() {
        let mut e = engine3();
        magic_scene(&mut e, true);
        // Rotation (0, 0, 0.5), position (1, 2, 3).
        let rotation = e.mem.alloc(12);
        e.mem.set_f32(rotation + 8, 0.5);
        let position = e.mem.alloc(12);
        for (i, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *value);
        }
        e.register_double(V_POSITION, move |_, _| position.into_ret());
        e.register_double(REFERENCE_GET_ROTATION, move |_, _| rotation.into_ret());
        // The two float helpers: distinguishable results.
        e.register(FLOAT_FUNCTION_A, |_, a| {
            (f32::from_bits(a[0]) * 2.0).into_ret()
        });
        e.register(FLOAT_FUNCTION_B, |_, a| {
            (f32::from_bits(a[0]) + 10.0).into_ret()
        });
        e.register(VECTOR_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret::default()
        });
        e.register(V_MODEL_PATH, |_, _| 0x7a7au32.into_ret());
        e.register(CREATE_PARTICLE_EFFECT, |_, _| 0xeff0u32.into_ret());
        e.register(PARTICLE_EFFECT_SET_NAME, |_, _| Ret::default());
        let reference = object_with(&mut e, &[(0x1d0, V_FALSE), (0x1f4, V_POSITION)]);
        let setting = e.mem.alloc(0x40);
        let model_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(model_vtable + 0x14, V_MODEL_PATH);
        e.mem.set_u32(setting + 0x18, model_vtable);
        let a = command(&mut e, reference);
        parse_gives(&mut e, true, &[setting]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_1dc0, &args![a]).bool());
        assert_parsed(&e, reference);
        assert_eq!(calls(&e, REFERENCE_GET_ROTATION), vec![vec![reference]]);
        assert_eq!(calls(&e, FLOAT_FUNCTION_A), vec![vec![(-0.5f32).to_bits()]]);
        assert_eq!(calls(&e, FLOAT_FUNCTION_B), vec![vec![(-0.5f32).to_bits()]]);
        // The direction is (-B(-z), A(-z), 0) = (-9.5, -1, 0).
        assert_eq!(
            calls(&e, CREATE_PARTICLE_EFFECT),
            vec![vec![
                0xcc00,
                1.0f32.to_bits(),
                0x7a7a,
                (-9.5f32).to_bits(),
                (-1.0f32).to_bits(),
                0,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                1.0f32.to_bits(),
                7,
                0
            ]]
        );
        assert_eq!(
            calls(&e, PARTICLE_EFFECT_SET_NAME),
            vec![vec![0xeff0, 0x0103_bdb8]]
        );
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_VISUAL_APPLIED_NAMED, 0xaaaa]]
        );
        // The cell is not loaded: nothing.
        e.register_double(TES_IS_CELL_LOADED, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_1dc0, &args![a]).bool());
        assert!(calls(&e, CREATE_PARTICLE_EFFECT).is_empty());
        // Bad parameters.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_1dc0, &args![a]).bool());
    }

    /// Doubles of the model hit effect: the setting's model path has the
    /// given length; the effect object initialises or not.
    fn model_scene(e: &mut Engine, length: u32, initialise: u32) -> (u32, u32, u32) {
        magic_scene(e, true);
        e.register_double(STRING_LENGTH_AT_4, move |_, _| length.into_ret());
        e.register(V_MODEL_PATH, |_, _| 0x7a7au32.into_ret());
        let effect = hit_effect(e, initialise);
        e.register_double(MAGIC_MODEL_HIT_EFFECT_CONSTRUCT, move |_, _| {
            effect.into_ret()
        });
        let setting = e.mem.alloc(0x40);
        let model_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(model_vtable + 0x14, V_MODEL_PATH);
        e.mem.set_u32(setting + 0x18, model_vtable);
        let reference = object_with(e, &[(0x1d0, V_NONZERO)]);
        (setting, effect, reference)
    }

    #[test]
    fn play_magic_effect_adds_a_model_hit_effect_that_initialises() {
        let mut e = engine3();
        let (setting, effect, reference) = model_scene(&mut e, 5, V_HIT_INITIALISED);
        let a = command(&mut e, reference);
        parse_gives(&mut e, true, &[setting, 4.0f32.to_bits()]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_1dc0, &args![a]).bool());
        assert_eq!(calls(&e, STRING_LENGTH_AT_4), vec![vec![setting + 0x18]]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x3c]]);
        assert_eq!(
            calls(&e, MAGIC_MODEL_HIT_EFFECT_CONSTRUCT),
            vec![vec![0x9000 + 0x3c, reference, 0x7a7a, 4.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, PROCESS_LISTS_ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, effect]]
        );
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_VISUAL_APPLIED_NAMED, 0xaaaa]]
        );
        // The float defaults to the global.
        parse_gives(&mut e, true, &[setting]);
        start_log(&mut e);
        assert!(e.call(0x005d_1dc0, &args![a]).bool());
        assert_eq!(
            calls(&e, MAGIC_MODEL_HIT_EFFECT_CONSTRUCT),
            vec![vec![0x9000 + 0x3c, reference, 0x7a7a, 2.5f32.to_bits()]]
        );
        // No name for the reference: the unnamed message.
        e.register(GET_REFERENCE_NAME, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_1dc0, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_VISUAL_APPLIED_REFERENCE]]
        );
    }

    #[test]
    fn play_magic_effect_destroys_a_model_hit_effect_that_fails() {
        let mut e = engine3();
        let (setting, effect, reference) = model_scene(&mut e, 5, V_HIT_FAILED);
        let a = command(&mut e, reference);
        parse_gives(&mut e, true, &[setting]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_1dc0, &args![a]).bool());
        assert_eq!(calls(&e, V_HIT_DESTROY), vec![vec![effect, 1]]);
        assert!(calls(&e, PROCESS_LISTS_ADD_TEMP_EFFECT).is_empty());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_VISUAL_FAILED_NAMED, 0xaaaa]]
        );
        e.register(GET_REFERENCE_NAME, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_1dc0, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_VISUAL_FAILED_REFERENCE]]
        );
        // A model path of length zero, or no setting: no effect is built and
        // the failure is printed.
        e.register(GET_REFERENCE_NAME, |_, _| 0xaaaau32.into_ret());
        e.register_double(STRING_LENGTH_AT_4, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_1dc0, &args![a]).bool());
        assert!(calls(&e, ALLOCATE).is_empty());
        assert!(calls(&e, V_HIT_DESTROY).is_empty());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_VISUAL_FAILED_NAMED, 0xaaaa]]
        );
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_1dc0, &args![a]).bool());
        assert!(calls(&e, STRING_LENGTH_AT_4).is_empty());
        assert_eq!(calls(&e, CONSOLE_PRINT).len(), 1);
        // The cell is not loaded: nothing.
        e.register_double(TES_IS_CELL_LOADED, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_1dc0, &args![a]).bool());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn stop_magic_shader_removes_the_shader_and_prints_without_the_echo_flag() {
        let mut e = engine3();
        let player = magic_scene(&mut e, true);
        e.register(PROCESS_LISTS_FINISH_MAGIC_SHADER, |_, _| Ret::default());
        let reference = object(&mut e);
        let a = command(&mut e, reference);
        parse_gives(&mut e, true, &[0x3131]);
        start_log(&mut e);
        assert!(e.call(0x005d_2130, &args![a]).bool());
        assert_parsed(&e, reference);
        assert_eq!(
            calls(&e, PROCESS_LISTS_FINISH_MAGIC_SHADER),
            vec![vec![PROCESS_LISTS, reference, 0x3131]]
        );
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_SHADER_REMOVED_NAMED, 0xaaaa]]
        );
        // No name; the reference defaults to the player.
        e.register(GET_REFERENCE_NAME, |_, _| Ret::default());
        let b = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_2130, &args![b]).bool());
        assert_eq!(
            calls(&e, PROCESS_LISTS_FINISH_MAGIC_SHADER),
            vec![vec![PROCESS_LISTS, player, 0x3131]]
        );
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_SHADER_REMOVED_REFERENCE]]
        );
        // Bad parameters.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_2130, &args![a]).bool());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }
}

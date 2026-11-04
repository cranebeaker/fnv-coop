//! `fallout shared/tesscriptfunctions.cpp` (Xbox PDB source unit), part 8: its functions from `005db810` up to
//! (not including) `005e0310` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesscriptfunctions`]; anything public there may be used here.
//!
//! Progress: all 120 queue entries of the range (`005db810` to `005e0310`)
//! are translated; the range is finished.
//!
//! The command bodies follow the conventions of the main file: `cdecl`, the
//! eight stack words as [`ScriptArgs`], `AL` as the result. Members of
//! classes this unit does not own are read at the PC offsets with a comment.
//! The compiler's exception-unwinding frames are not translated.

#[allow(unused_imports)]
use super::tesscriptfunctions::*;
#[allow(unused_imports)]
use crate::prelude::*;

// ---- Callees outside this part (by exe address) ----------------------------

/// `Script::ParseParameters` (Xbox PDB), `cdecl`, varargs.
const PARSE_PARAMETERS: u32 = 0x005a_ccb0;
/// The console print: format address first (`cdecl`).
const CONSOLE_PRINT: u32 = 0x0070_3c00;
/// The logging stub of the unit (`005b5e40`): takes a format and its
/// arguments, does nothing and returns 0 in this build.
const LOG_STUB: u32 = 0x005b_5e40;
/// `_memset` (`dest, value, count`).
const MEMSET: u32 = 0x00ec_61c0;
/// Zero-fills (`array, 0, bytes`); a wrapper of `_memset`.
const ZERO_FILL: u32 = 0x0040_3d30;
/// `sprintf_s` (`dest, size, format, ...`).
const SPRINTF_S: u32 = 0x0040_6d00;
/// Case-insensitive string comparison (`a, b`), 0 when equal; a wrapper of
/// `_stricmp`.
const STRICMP: u32 = 0x0040_4dc0;
/// Form type (`thiscall`, the byte at `this + 4`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `*(this + 0x20)`: the base form of a reference (the engine map calls it
/// `BGSSaveFormBuffer::GetForm`).
const GET_BASE_FORM: u32 = 0x007a_f430;
/// `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB): `*(this + 0x68)`.
const GET_PROCESS: u32 = 0x008d_8520;
/// `PlayerCharacter::CheckForQuestTargetUpdate` (Xbox PDB), `thiscall` on the
/// player (`reference`).
const CHECK_QUEST_TARGET_UPDATE: u32 = 0x0095_2c30;
/// `thiscall` on a door's base form (`flag`): sets or clears the minimal-use
/// state (`tesobjectdoor.cpp`).
const DOOR_SET_MINIMAL_USE: u32 = 0x0051_8030;

/// `Script::GetVATSFrontTargetVisibleConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, value, 0, result`).
const GET_VATS_FRONT_TARGET_VISIBLE_CONDITION: u32 = 0x005a_5840;
/// `Script::IsInCritStageConditionFunction` (Xbox PDB), same arguments.
const IS_IN_CRIT_STAGE_CONDITION: u32 = 0x005a_2910;
/// `Script::IsGoreDisabledConditionFunction` (Xbox PDB), same arguments.
const IS_GORE_DISABLED_CONDITION: u32 = 0x005a_5a50;
/// `Script::GetActorsInHighConditionFunction` (Xbox PDB), same arguments.
const GET_ACTORS_IN_HIGH_CONDITION: u32 = 0x005a_5ab0;

/// `thiscall` on an actor (`value`): stores `value` at `+0x10c` and updates
/// the actor (`008a1a70`).
const ACTOR_SET_FIELD_10C: u32 = 0x008a_1a40;

/// Float functions with one `float` argument and the result in `ST0`; their
/// meaning is not named in the engine map. The angle commands combine them
/// with the degree/radian constants.
const FLOAT_FN_004E44B0: u32 = 0x004e_44b0;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_004E4470: u32 = 0x004e_4470;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_004B5510: u32 = 0x004b_5510;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_0057DD70: u32 = 0x0057_dd70;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_004F6E40: u32 = 0x004f_6e40;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_004B1460: u32 = 0x004b_1460;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_004019B0: u32 = 0x0040_19b0;
/// See [`FLOAT_FN_004E44B0`].
const FLOAT_FN_00408840: u32 = 0x0040_8840;
/// The CRT `double` function behind [`fn_005dc460`] (the library match is
/// ambiguous between `cos`, `log`, `sin` and `tan`).
const CRT_DOUBLE_FN_00ECA920: u32 = 0x00ec_a920;

/// `Tile::GetMenuByClass` (Xbox PDB), `cdecl` (`class id`).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Tile::GetMenu` (Xbox PDB), `thiscall`.
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `ComputersMenu::DisplayPreviousTerminal` (Xbox PDB), `thiscall`.
const DISPLAY_PREVIOUS_TERMINAL: u32 = 0x0075_8a80;
/// `FalloutRadio::PipboyRadioEnable` (Xbox PDB), `cdecl` (`flag`).
const PIPBOY_RADIO_ENABLE: u32 = 0x0083_24e0;

/// `this + 0x44` (`thiscall`): the stage list of a quest.
const QUEST_STAGE_LIST: u32 = 0x005d_43c0;
/// Node access of the singly linked lists (`BSSimpleList`: item at +0, next
/// node at +4): returns `this`, the address of the node's item.
const LIST_ITEM_PTR: u32 = 0x0068_15c0;
/// `*(this + 4)`: the next node.
const LIST_NEXT: u32 = 0x0072_6070;
/// `this + 4` (`BaseProcess::GetActorPackageThatIsRunning` in the engine
/// map): the item list embedded in a quest stage.
const PLUS_4: u32 = 0x0071_7e50;
/// `thiscall` on a quest stage: the byte at `this + 0`.
const STAGE_INDEX: u32 = 0x0093_73f0;
/// `BSStringT` constructor (an 8-byte object).
const BS_STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `BSStringT` destructor.
const BS_STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `BSStringT<char>::operator=` (`string, text`).
const BS_STRING_ASSIGN: u32 = 0x0043_8390;
/// `BSStringT<char>::operator+=` (`string, text`).
const BS_STRING_APPEND: u32 = 0x0040_4820;
/// `BSStringT` copy constructor (`thiscall` on the new object, `source`).
const BS_STRING_COPY_CONSTRUCT: u32 = 0x0040_47f0;
/// `Interface::PrintLine` (Xbox PDB), `cdecl`: takes the 8-byte `BSStringT` by
/// value (two words).
const PRINT_LINE: u32 = 0x0070_3c80;

/// `TESNPC::InitHead` (Xbox PDB), `thiscall` (`&first, &second`): two output
/// words.
const TES_NPC_INIT_HEAD: u32 = 0x0060_7370;
/// `thiscall` on the base form (`reference, data, first, second`): the
/// head-update step of the face generation command (`00607420`).
const TES_NPC_UPDATE_HEAD: u32 = 0x0060_7420;
/// `thiscall` on the base form: its race, `*(this + 0x110)`.
const TES_NPC_GET_RACE: u32 = 0x004a_c110;
/// `TESRace::GetFaceGenData` (Xbox PDB), `thiscall` on the race (`npc,
/// face gen data, 0, 0`).
const TES_RACE_GET_FACE_GEN_DATA: u32 = 0x0061_41f0;
/// Constructor of a 16-byte object (`thiscall`: `float, byte, byte`).
const FLOAT_AND_FLAGS_CONSTRUCT: u32 = 0x0043_d410;
/// `thiscall` on the node object (`&object`).
const NODE_APPLY: u32 = 0x00a5_9c60;
/// `*(this + 0x18)` (`thiscall`).
const FIELD_18: u32 = 0x0096_11e0;
/// `RecurseAndRemoveObjectsFromPalette` (Xbox PDB), `cdecl` (`object,
/// skeleton`).
const RECURSE_REMOVE_FROM_PALETTE: u32 = 0x00a6_e8e0;
/// `*(this + 0xd8)` as a pointer (`thiscall`, `NiPointer` read).
const POINTER_AT_D8: u32 = 0x0049_6940;
/// `*(this + 0x78)` as a pointer (`thiscall`, `NiPointer` read).
const POINTER_AT_78: u32 = 0x0053_7bd0;
/// `NiPointer` assignment of a raw pointer to a slot (`slot, pointer`);
/// returns the slot.
const NI_POINTER_SET: u32 = 0x0066_b0d0;
/// `NiPointer::operator=` (`to, from`).
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// Stores its byte argument at `011d59e0` (`cdecl`, `flag`): the
/// `ModifyFaceGen` mode byte.
const SET_FACE_GEN_MODE_BYTE: u32 = 0x005d_d860;
/// The name tables of the face generation commands (`cdecl`, `index`): the
/// text of expression, modifier and phoneme number `index`.
const EXPRESSION_NAME: u32 = 0x005d_d830;
/// See [`EXPRESSION_NAME`].
const MODIFIER_NAME: u32 = 0x005d_d840;
/// See [`EXPRESSION_NAME`].
const PHONEME_NAME: u32 = 0x005d_d850;

/// `eh_vector_constructor_iterator` (`array, size, count, constructor,
/// destructor`).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// `eh_vector_destructor_iterator` (`array, size, count, destructor`).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// Element constructor of the 0x20-byte elements of the face generation
/// data.
const ELEMENT_CONSTRUCT: u32 = 0x0044_9610;
/// Element destructor of the same elements.
const ELEMENT_DESTRUCT: u32 = 0x0044_9680;
/// `thiscall` (`0, 1`): constructors of the four 16-byte arrays at `+0x94`,
/// `+0xa4`, `+0xb4` and `+0xc4` of the face generation data.
const ARRAY_94_CONSTRUCT: u32 = 0x005e_0370;
/// See [`ARRAY_94_CONSTRUCT`].
const ARRAY_A4_CONSTRUCT: u32 = 0x005e_0430;
/// See [`ARRAY_94_CONSTRUCT`].
const ARRAY_B4_CONSTRUCT: u32 = 0x005e_0490;
/// See [`ARRAY_94_CONSTRUCT`].
const ARRAY_C4_CONSTRUCT: u32 = 0x005e_0310;
/// `thiscall`: resets one of the arrays (called on `+0x94`, `+0xa4`, `+0xb4`).
const ARRAY_RESET: u32 = 0x005e_03d0;
/// `thiscall` destructor of the array at `+0x94` (what [`fn_005dd7b0`]
/// calls).
const ARRAY_94_DESTRUCT: u32 = 0x005e_0340;
/// `thiscall` destructor of the array at `+0xa4` (what [`fn_005dd7d0`]
/// calls).
const ARRAY_A4_DESTRUCT: u32 = 0x005e_03a0;
/// Destructor wrapper of the array at `+0xb4` (`005dd7f0`, [`fn_005dd7f0`]).
const ARRAY_B4_DESTRUCT_WRAPPER: u32 = 0x005d_d7f0;
/// Destructor wrapper of the array at `+0xc4` (`005dd810`, [`fn_005dd810`]).
const ARRAY_C4_DESTRUCT_WRAPPER: u32 = 0x005d_d810;
/// Initialises an empty list head (`thiscall`, a two-word object).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// Destructor of the list head.
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `thiscall` on the list at `+0xe4` of the face generation data, run
/// before the list destructor in [`fn_005dd6f0`].
const LIST_PRE_DESTRUCT: u32 = 0x0047_0470;

/// The no-op "constructor" of the 16-byte vector temporaries
/// (`thiscall`, returns `this`; the same code as [`LIST_ITEM_PTR`]).
const VECTOR_CONSTRUCT_NOOP: u32 = 0x0068_15c0;
/// `thiscall` on a vector (`output`): stores its first lane in all four lanes
/// of the output.
const VECTOR_SPLAT_X: u32 = 0x0056_1240;
/// `cdecl` (`output, vector`): stores the first three lanes of a vector as
/// three separate floats.
const VECTOR_STORE_THREE: u32 = 0x0045_8620;

// ---- Globals and constants ---------------------------------------------------

/// The `PlayerCharacter` singleton pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// Byte at `+0x268` of the TLS block: commands echo to the console when set.
const TLS_ECHO: u32 = 0x268;
/// Counter incremented by [`fn_005dbe10`] (16 bits, never 0).
const COUNTER: u32 = 0x011b_0808;
/// The depth bias toggle of [`script_toggle_depth_bias_function`].
const DEPTH_BIAS_FLAG: u32 = 0x011a_d80f;
/// The value stored by `ModifyFaceGen coord 1 <value>`.
const FACE_GEN_COORD_VALUE: u32 = 0x011d_59f0;
/// Four `float` ones (`1.0f` in every lane).
const VECTOR_ONES: u32 = 0x010e_c7f0;
/// `double` -1.0, the lower bound of the angle commands.
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
/// `float` -1.0.
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `double` 1.0.
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `double` 57.2957763671875, radians to degrees.
const RADIANS_TO_DEGREES: u32 = 0x0102_f248;
/// `double` 0.017453292 (as the exe stores it), degrees to radians.
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `float` 2.7182817 (e), the default base of the logarithm command.
const EULER_NUMBER: u32 = 0x0103_d018;
/// `double` 100.0.
const HUNDRED: u32 = 0x0101_7a40;

/// Virtual slot `0x100` of a reference: whether it is an actor.
const VSLOT_IS_ACTOR: u32 = 0x100;
/// Virtual slot `0x130`: the name (of a script).
const VSLOT_NAME: u32 = 0x130;
/// Virtual slot `0x1b8` of a reference (`0`): the object whose face
/// generation data the `ModifyFaceGen` command changes.
const VSLOT_FACE_GEN_TARGET: u32 = 0x1b8;
/// Virtual slot `0x1b0` of a reference (`0`).
const VSLOT_OBJECT_1B0: u32 = 0x1b0;
/// Virtual slot `0x1ac` of a reference (`0`).
const VSLOT_OBJECT_1AC: u32 = 0x1ac;
/// Virtual slot `0x218` of a reference: a flag.
const VSLOT_FLAG_218: u32 = 0x218;
/// Virtual slot `0x1d0` of a reference: its node object (null when none).
const VSLOT_NODE: u32 = 0x1d0;
/// Virtual slot `0x1e4` of a reference: its animation object.
const VSLOT_ANIMATION: u32 = 0x1e4;
/// Virtual slot `0x1e8` of a reference: the data `TESNPC::BuildObjectArray`
/// takes as `base`.
const VSLOT_BASE_DATA: u32 = 0x1e8;
/// Virtual slot `0x114` of the face generation target (`index, float`):
/// sets one expression.
const VSLOT_SET_EXPRESSION: u32 = 0x114;
/// Virtual slot `0xa4` of the face generation target (`index`): a `float`
/// in `ST0`, the current phoneme value.
const VSLOT_GET_PHONEME: u32 = 0xa4;
/// Virtual slot `0xec` of the face generation target (`array, float`):
/// sets all phonemes.
const VSLOT_SET_PHONEMES: u32 = 0xec;
/// Virtual slot `0xf0` of the face generation target (`array, float`):
/// sets all modifiers.
const VSLOT_SET_MODIFIERS: u32 = 0xf0;
/// Virtual slot `0xb4` of the face generation target (`1.0, 1, 1, 1, 1,
/// 0`): the `reset` request.
const VSLOT_RESET: u32 = 0xb4;
/// Virtual slot `0x79c` of a process (`0`).
const VSLOT_PROCESS_79C: u32 = 0x79c;
/// Virtual slot `0x794` of a process (`0`).
const VSLOT_PROCESS_794: u32 = 0x794;
/// Virtual slot `0x7a4` of a process (`0`).
const VSLOT_PROCESS_7A4: u32 = 0x7a4;
/// Virtual slot `0x58` of a process.
const VSLOT_PROCESS_58: u32 = 0x58;
/// Virtual slot `0xe8` of the node object (`object`).
const VSLOT_NODE_E8: u32 = 0xe8;
/// Virtual slot `0xc` of the node object: its data.
const VSLOT_NODE_DATA: u32 = 0xc;

// ---- String literals (addresses in the exe's data) ---------------------------

/// `"Depth Bias is now %s!"`
const MSG_DEPTH_BIAS: u32 = 0x0103_d01c;
/// `"OFF"`
const TEXT_OFF: u32 = 0x0103_d034;
/// `"ON"`
const TEXT_ON: u32 = 0x0103_d038;
/// `"SCRIPTS: SetMinimalUse must be called on a door reference."`
const MSG_SET_MINIMAL_USE: u32 = 0x0103_d03c;
/// `")"`
const TEXT_CLOSE_PAREN: u32 = 0x0103_d078;
/// `"Item %d: %d"`
const FORMAT_ITEM: u32 = 0x0103_d07c;
/// `" ("`
const TEXT_OPEN_PAREN: u32 = 0x0103_d088;
/// `"Stage %d: %d "`
const FORMAT_STAGE: u32 = 0x0103_d08c;
/// `", "`
const TEXT_SEPARATOR: u32 = 0x0101_2630;
/// `"reset"`
const KEYWORD_RESET: u32 = 0x0103_d09c;
/// `"coord"`
const KEYWORD_COORD: u32 = 0x0103_d0a4;
/// `"modifier"`
const KEYWORD_MODIFIER: u32 = 0x0103_d0ac;
/// `"phoneme"`
const KEYWORD_PHONEME: u32 = 0x0103_d0b8;
/// `"%d. %s, "`
const FORMAT_NAME_MORE: u32 = 0x0103_d0c0;
/// `"%d. %s"`
const FORMAT_NAME_LAST: u32 = 0x0103_d0cc;
/// `"expression"`
const KEYWORD_EXPRESSION: u32 = 0x0103_d0d4;

// ---- Callees, globals and strings of the second 40 functions -----------------

/// Destructor of the array at `+0xb4` of the face generation data
/// (`thiscall`; what [`fn_005dd7f0`] calls).
const ARRAY_B4_DESTRUCT: u32 = 0x005e_0460;
/// Destructor of the array at `+0xc4` of the face generation data
/// (`thiscall`; what [`fn_005dd810`] calls).
const ARRAY_C4_DESTRUCT: u32 = 0x005e_02e0;

/// Pointer table of the expression names (read by [`fn_005dd830`]).
const EXPRESSION_NAME_TABLE: u32 = 0x0119_b41c;
/// Pointer table of the modifier names (read by [`fn_005dd840`]).
const MODIFIER_NAME_TABLE: u32 = 0x0119_b458;
/// Pointer table of the phoneme names (read by [`fn_005dd850`]).
const PHONEME_NAME_TABLE: u32 = 0x0119_b4a0;
/// The mode byte of the face generation commands (stored by
/// [`fn_005dd860`]).
const FACE_GEN_MODE_BYTE: u32 = 0x011d_59e0;

/// Creates on first use and returns the object kept at `011f3634` (`cdecl`,
/// no argument): the record of the known code strings. The commands push
/// their text buffer before the call, but it belongs to the next call.
const CODE_RECORDS: u32 = 0x00a2_5920;
/// `thiscall` on the object of [`CODE_RECORDS`] (`text`): finds the bit
/// number of the text among four known code strings and tests that bit in
/// the record; `AL` is the result.
const CODE_IS_REDEEMED: u32 = 0x00a2_6020;
/// `thiscall` on the object of [`CODE_RECORDS`] (`text`): sets the text's bit
/// in the record; `AL` is whether the text is one of the known strings.
const CODE_SET_REDEEMED: u32 = 0x00a2_6090;
/// `thiscall` on the object of [`CODE_RECORDS`] (`text`): clears the text's
/// bit in the record.
const CODE_CLEAR_REDEEMED: u32 = 0x00a2_6120;
/// The byte at `this + 0x1b1` of an actor (`thiscall`), in
/// `tesconditionfunctions.cpp`.
const ACTOR_BYTE_1B1: u32 = 0x005a_5bb0;
/// `ExtraDataList::SetWeaponModSlot` (Xbox PDB), `thiscall` (`mask`).
const SET_WEAPON_MOD_SLOT: u32 = 0x0042_e380;
/// `*this` (`thiscall`): reads the pointer a `NiPointer` holds.
const READ_POINTER: u32 = 0x0055_9450;
/// Virtual slot `0x148` of a process: its result points to a pointer that
/// leads to the extra data list of the weapon.
const VSLOT_PROCESS_148: u32 = 0x148;

/// `thiscall` (`index`) on the first parsed object: for index 1 the `float`
/// at `+0x44`, otherwise the one at `+0x48`, in `ST0`.
const OBJECT_FLOAT_GETTER_006155C0: u32 = 0x0061_55c0;
/// `thiscall` (`index`): the same selection with another getter (in `ST0`);
/// it calls `00616a20` for index 1 and `00616a40` otherwise.
const OBJECT_FLOAT_GETTER_00616A60: u32 = 0x0061_6a60;
/// `thiscall` (`index`, 0 to 2): a signed byte in `AL`.
const OBJECT_BYTE_GETTER_00616A90: u32 = 0x0061_6a90;
/// `thiscall` (`index`, `value`), index 0 or 1.
const OBJECT_SET_INT_00615C90: u32 = 0x0061_5c90;
/// `thiscall` (`index`, `float`), index 0 or 1.
const OBJECT_FLOAT_SETTER_00615730: u32 = 0x0061_5730;
/// `thiscall` (`index`, `value`), index 0 or 1.
const OBJECT_SET_INT_00615FA0: u32 = 0x0061_5fa0;
/// `thiscall` (`index`, `float`), index 0 or 1.
const OBJECT_FLOAT_SETTER_00615A00: u32 = 0x0061_5a00;
/// `thiscall` (`index`, `float`), index 0 or 1: stores the `float` at `+0x44`
/// (index 1) or `+0x48` and calls virtual slot `0x48` (`2`).
const OBJECT_FLOAT_SETTER_00615580: u32 = 0x0061_5580;

/// The object at `011e0e80`; the array embedded at its `+4` holds the
/// references the loops of [`fn_005ddf90`] and [`fn_005de110`] visit.
const REFERENCE_LIST_OWNER: u32 = 0x011e_0e80;
/// `thiscall` (`index`) on the array: the element `index`.
const LIST_GET: u32 = 0x0096_8670;
/// `thiscall` (`0`) on the array: its element count, `*(this + 0x20)`.
const LIST_COUNT: u32 = 0x005b_e5c0;
/// `thiscall` on a reference: whether bit `0x200000` of `*(this + 8)` is set.
const FLAG_200000: u32 = 0x0057_6d30;
/// `*(this + 0xfc)` (`thiscall`).
const FIELD_FC: u32 = 0x0087_4480;
/// `TESObjectREFR::GetOwner` (Xbox PDB), `thiscall`.
const GET_OWNER: u32 = 0x0056_7790;
/// The base form of a reference (`thiscall`; calls `007af430`).
const REFERENCE_BASE_FORM: u32 = 0x0041_81e0;
/// `thiscall` (`mask`) on a base form: whether a bit of its flag word is
/// set ([`fn_005de080`] asks for `0x400`).
const FORM_FLAG_TEST: u32 = 0x004f_d420;
/// Virtual slot `0x224` of a reference.
const VSLOT_REFERENCE_224: u32 = 0x224;
/// Virtual slot `0x1a4` of a reference (no argument): compared with the
/// owner.
const VSLOT_REFERENCE_1A4: u32 = 0x1a4;
/// Virtual slot `0x318` of a reference (no argument).
const VSLOT_REFERENCE_318: u32 = 0x318;
/// Virtual slot `0x320` of a reference (`player, 1`).
const VSLOT_REFERENCE_320: u32 = 0x320;
/// The bit [`fn_005ddf90`] sets with [`fn_005de0a0`].
const FLAG_SET_BY_LOOP: u32 = 0x0010_0000;
/// The form flag [`fn_005ddf90`] tests with [`fn_005de080`].
const FORM_FLAG_400: u32 = 0x400;

/// A `u16` global stored by [`fn_005de240`].
const COMMAND_VALUE_011AE75C: u32 = 0x011a_e75c;
/// A `u16` global stored by [`fn_005de2c0`].
const COMMAND_VALUE_011AFA9C: u32 = 0x011a_fa9c;
/// `BSTaskThread::sPauseSleepValue` (named by the command's message), set
/// by [`fn_005de410`].
const TASK_THREAD_PAUSE_SLEEP: u32 = 0x011a_f708;
/// `BSTaskThread::sUpdateSleepValue`, set by [`fn_005de490`].
const TASK_THREAD_UPDATE_SLEEP: u32 = 0x011a_f710;
/// The pointer whose object gets the byte at `+0x254` set by
/// [`fn_005de380`].
const OBJECT_POINTER_011D96C0: u32 = 0x011d_96c0;
/// `PlayerCharacter` field at `+0x604`: the last exterior door activated.
const PLAYER_LAST_EXT_DOOR: u32 = 0x604;
/// `TESObjectREFR::GetWorldSpace` (Xbox PDB), `thiscall`.
const GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// `Interface::SetRootMapSpace` (Xbox PDB), `cdecl` (`worldspace`).
const SET_ROOT_MAP_SPACE: u32 = 0x0070_9d20;

/// The pointer at `011dea10`: an object whose `+0x6c` is the head node of a
/// list ([`script_is_image_space_active_function`]).
const LIST_OWNER_011DEA10: u32 = 0x011d_ea10;
/// `this + 0x6c` (`thiscall`): the head node of that list.
const LIST_HEAD_AT_6C: u32 = 0x0043_b5d0;
/// `NiPointer` copy constructor (`thiscall`, `source`): copies the pointer
/// stored at `source` and takes a reference.
const NI_POINTER_COPY_CONSTRUCT: u32 = 0x0055_9a40;
/// `NiPointer` destructor (`thiscall`): releases the reference.
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// Virtual slot `0x94` of the object read from a list entry.
const VSLOT_ENTRY_94: u32 = 0x94;
/// `*(this + 0x1c)` (`thiscall`; the engine map names it
/// `PathingLocation::GetWorldspace`).
const FIELD_1C: u32 = 0x0044_1110;

/// `thiscall` (`second parsed object`) on the first one, in [`fn_005de600`].
const PAIR_ACTION_00590010: u32 = 0x0059_0010;
/// `thiscall` on `container + 0x30` (`container, count, flags, item,
/// extra`): the shared step of the two leveled-list commands.
const ADD_ITEM_00487A00: u32 = 0x0048_7a00;
/// `operator new` (`size`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `ContainerItemExtra::ContainerItemExtra` (Xbox PDB), `thiscall`: returns
/// `this`.
const CONTAINER_ITEM_EXTRA_CONSTRUCT: u32 = 0x0040_e690;
/// `thiscall` (`float`) on a `ContainerItemExtra`.
const CONTAINER_ITEM_EXTRA_SET_FLOAT: u32 = 0x0040_eb90;
/// The `float` 100.0 that is the default of the leveled-list percentage.
const DEFAULT_PERCENT: u32 = 0x0101_6410;
/// `thiscall` (`replacement`) on the water type to be remapped.
const WATER_REMAP_00580680: u32 = 0x0058_0680;
/// The form type of a water type.
const FORM_TYPE_WATER: u32 = 0x4e;

/// `"No"`
const TEXT_NO: u32 = 0x0103_abc4;
/// `"Yes"`
const TEXT_YES: u32 = 0x0103_abc8;
/// `"IsCodeRedeemed [%s] = %s"`
const FORMAT_IS_CODE_REDEEMED: u32 = 0x0103_d0e0;
/// `"BSTaskThread::sPauseSleepValue set to %d ms"`
const MSG_PAUSE_SLEEP: u32 = 0x0103_d0fc;
/// `"BSTaskThread::sUpdateSleepValue set to %d ms"`
const MSG_UPDATE_SLEEP: u32 = 0x0103_d128;
/// `"ImageSpace is not active"`
const MSG_IMAGE_SPACE_NOT_ACTIVE: u32 = 0x0103_d158;
/// `"ImageSpace is active "`
const MSG_IMAGE_SPACE_ACTIVE: u32 = 0x0103_d174;
/// `"Invalid water type for water to remap to."`
const MSG_WATER_REMAP_TO_INVALID: u32 = 0x0103_d18c;
/// `"Invalid water type for water to be remapped."`
const MSG_WATER_TO_REMAP_INVALID: u32 = 0x0103_d1b8;

// ---- Callees and data of 005de900 and on ---------------------------------------

/// `Script::IsHardcoreConditionalFunction` (Xbox PDB), `cdecl` (`thisObj,
/// value, 0, result`).
const IS_HARDCORE_CONDITION: u32 = 0x005a_5f90;
/// `Script::IsAlwaysHardcoreConditionalFunction` (Xbox PDB), same arguments.
const IS_ALWAYS_HARDCORE_CONDITION: u32 = 0x005a_5ff0;
/// `PlayerCharacter::SetHardcore` (Xbox PDB), `thiscall` on the player
/// (`value, 0`).
const PLAYER_SET_HARDCORE: u32 = 0x0096_9e90;
/// `PlayerCharacter` byte at `+0xe38` (PC offset): cleared by
/// [`fn_005de9b0`] when hardcore is turned off.
const PLAYER_BYTE_E38: u32 = 0xe38;
/// The sub-object at `+0xa4` of the player (it has a vtable of its own).
const PLAYER_SUBOBJECT_A4: u32 = 0xa4;
/// Virtual slot `0xc` of that sub-object (`index`): a `float` in `ST0`.
const VSLOT_SUBOBJECT_VALUE: u32 = 0xc;
/// Virtual slot `0x3a4` of the player (`index, float, 0`).
const VSLOT_PLAYER_3A4: u32 = 0x3a4;
/// The index `ResetXP` passes to both (`0x18`).
const RESET_XP_INDEX: u32 = 0x18;
/// `Script::PutNumericIDInDouble` (Xbox PDB), `cdecl` (`&id, double*`).
const PUT_NUMERIC_ID_IN_DOUBLE: u32 = 0x005a_cc70;
/// `thiscall` on an actor: a pointer to a record whose `+4` is a form, or
/// null (`008a51a0`; it reads the actor's process, `008d8520`, and calls
/// that process's virtual slot `0x77c`).
const ACTOR_RECORD: u32 = 0x008a_51a0;
/// `*(this + 0xc)` (`thiscall`): the form id of a form.
const FORM_ID: u32 = 0x0084_e3a0;
/// Virtual slot `0xfc` of a reference (no arguments): a flag.
const VSLOT_FLAG_FC: u32 = 0xfc;
/// RTTI type descriptor of `TESBoundObject` (`.?AVTESBoundObject@@`).
const RTTI_TES_BOUND_OBJECT: u32 = 0x0118_3108;
/// RTTI type descriptor of `BGSTalkingActivator`
/// (`.?AVBGSTalkingActivator@@`).
const RTTI_BGS_TALKING_ACTIVATOR: u32 = 0x0118_9e04;
/// `*(this + 0x90)` (`thiscall`; the engine map files it under
/// `tesobjectcont.cpp`).
const FIELD_90: u32 = 0x0051_6bf0;
/// The byte at `this + 0x81` (`thiscall`).
const BYTE_AT_81: u32 = 0x0057_4900;
/// `*(this + 0x6c)` (`thiscall`).
const FIELD_6C: u32 = 0x005e_3fa0;
/// `cdecl` (`container, value`): the interface call the command ends with;
/// it checks the interface state and then creates `RecipeMenu`.
const SHOW_MENU_FOR_00704FC0: u32 = 0x0070_4fc0;
/// `*(this + 0x610)` (`thiscall` on the player): the head node of a list
/// of entries (`+0` form id, `+4` value, `+8` a `u16`).
const PLAYER_LIST_HEAD: u32 = 0x005a_6260;
/// `this + 0x1e8` (`thiscall` on a form): a block whose `+0x28` is a limit.
const FORM_BLOCK_1E8: u32 = 0x0050_4e60;
/// The offset of the limit in that block.
const BLOCK_LIMIT: u32 = 0x28;
/// `double` 0.25: the step of the four levels.
const QUARTER: u32 = 0x0102_90b0;
/// `cdecl` (`object`): the shared step of [`fn_005dee30`].
const OBJECT_ACTION_005F64F0: u32 = 0x005f_64f0;
/// `(this + 0x70) & 1` (`thiscall`).
const FLAG_BIT_1: u32 = 0x005a_60d0;
/// `(this + 0x70) & 2` (`thiscall`).
const FLAG_BIT_2: u32 = 0x005a_6150;
/// The flags word at `+0x70` of the object the `005def..` functions work on.
const OBJECT_FLAGS: u32 = 0x70;
/// `thiscall` on the same object, no arguments: calls its virtual slot
/// `0x48` with `2` (`005f5800`).
const OBJECT_FLAGS_CHANGED: u32 = 0x005f_5800;
/// `BSSystemUtility::QInstance` (Xbox PDB name of a singleton getter), no
/// arguments.
const SYSTEM_UTILITY_INSTANCE: u32 = 0x00af_2640;
/// `BSDLCSystemUtility::QInstance` (Xbox PDB), what [`fn_005dfa10`] calls.
const DLC_SYSTEM_UTILITY_INSTANCE: u32 = 0x00af_26c0;
/// `this + 0xc` (`thiscall`): the head node of the list of DLC packages.
const DLC_LIST_HEAD: u32 = 0x0048_d150;
/// Counts the nodes of a list whose item is not null (`thiscall` on the
/// head node; the engine map calls it `VATS::GetCount`).
const LIST_COUNT_NON_NULL: u32 = 0x005a_e380;
/// `thiscall` on a DLC package: the text that follows the name in the
/// `"Examining"` line (it returns the address of a constant).
const DLC_PACKAGE_TAG: u32 = 0x00b0_08e0;
/// `*this` (`thiscall` on a DLC package): its wide-character name.
const DLC_PACKAGE_NAME: u32 = 0x00b0_08d0;
/// `_wcstombs_s` (`&converted, dest, size, source, max`).
const WCSTOMBS_S: u32 = 0x00ec_ad86;
/// `Interface::IsInGameLoadingMenuOpen` (Xbox PDB), no arguments.
const IS_IN_GAME_LOADING_MENU_OPEN: u32 = 0x0070_5ea0;
/// `thiscall` on the object at `011dea10` (`flag, 0, 1`).
const SET_FLAG_00457D70: u32 = 0x0045_7d70;
/// `thiscall` on an actor (`flag`) through the cast to `Actor`: sets whether
/// the actor is essential.
const ACTOR_SET_ESSENTIAL: u32 = 0x008c_1ac0;
/// `thiscall` on an actor: whether it is essential.
const ACTOR_IS_ESSENTIAL: u32 = 0x008c_1b30;
/// `thiscall` on an actor (`flag`): sets whether its subtitles are always
/// shown.
const ACTOR_SET_ALWAYS_SHOW_SUBTITLES: u32 = 0x008c_1b50;
/// `cdecl` (`value, 1`) of [`fn_005dfe30`].
const SHARED_ACTION_005F6110: u32 = 0x005f_6110;
/// The pointer at `011c54c0`: a hash table (`+4` size, `+8` slots).
const HASH_TABLE_011C54C0: u32 = 0x011c_54c0;
/// `thiscall` on that table: the first node.
const HASH_TABLE_FIRST: u32 = 0x004b_9ba0;
/// `thiscall` on that table (`&node, &key, &value`): reads the node's key and
/// value and moves `node` to the next one.
const HASH_TABLE_NEXT: u32 = 0x006b_7f20;
/// Virtual slot `0xf0` of a value of that table (no arguments): a flag.
const VSLOT_FLAG_F0: u32 = 0xf0;
/// `*(this + 8)` (`thiscall`).
const FIELD_8: u32 = 0x0044_ddc0;
/// The bit of `FIELD_8` the persistent reference count looks for.
const PERSISTENT_BIT: u32 = 0x400;
/// The byte that [`fn_005e0080`] stores.
const STORED_FLAG_011DD437: u32 = 0x011d_d437;
/// `thiscall` on the player: opens the sleep/wait choice (`00969fa0`).
const PLAYER_SLEEP_WAIT: u32 = 0x0096_9fa0;
/// `Interface::CreateSleepMenu` (Xbox PDB), `cdecl` (`flag`).
const CREATE_SLEEP_MENU: u32 = 0x0070_54f0;
/// `thiscall` on a container reference (`0, 1`): the inventory count.
const INVENTORY_COUNT: u32 = 0x0057_5590;
/// The object at `011de29c`: [`fn_005e0160`] reads the flag it holds.
const OBJECT_011DE29C: u32 = 0x011d_e29c;
/// `thiscall` on that object: the address of its flag byte.
const FLAG_ADDRESS: u32 = 0x0040_8d60;
/// The pointer at `011de134`: an object whose byte at `+0x11` is set.
const POINTER_011DE134: u32 = 0x011d_e134;
/// `thiscall` on that object: sets the byte at `+0x11`.
const SET_BYTE_11: u32 = 0x0085_1d70;
/// `thiscall` (`text`): appends or sets the text of a message object
/// (`00c33170`); returns `this`.
const MESSAGE_SET_TEXT: u32 = 0x00c3_3170;
/// `thiscall` (`arg, flag`): the step of the two setting functions
/// ([`fn_005e01b0`], [`fn_005e0200`]).
const SETTING_STEP: u32 = 0x005e_0830;
/// Constructor of the one-word scope guard of [`fn_005e0200`] (`thiscall`:
/// `9, 1, file, line`).
const GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
/// Destructor of that guard (`thiscall`).
const GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// Virtual slot `0x20` (no arguments), then `0x1c`, of the setting object.
const VSLOT_SETTING_20: u32 = 0x20;
/// See [`VSLOT_SETTING_20`].
const VSLOT_SETTING_1C: u32 = 0x1c;
/// Virtual slot `0x24` of the setting object ([`fn_005e0200`]).
const VSLOT_SETTING_24: u32 = 0x24;
/// `thiscall` (`arg, &out`): looks `arg` up and stores the match in `out`.
const LOOKUP_005E0860: u32 = 0x005e_0860;
/// `cdecl` (`pointer`): destroys what the pointer points at (the engine map:
/// `NiPointer<BSTempEffect>::_vector_deleting_destructor_`; it calls the
/// vector deleting destructor with 3 when the pointer is not null).
const POINTER_DESTROY: u32 = 0x004b_0220;
/// The vtable [`fn_005e02e0`] stores (the code is a library match for
/// `~basic_streambuf`, which the body is not).
const VTABLE_0103D3E8: u32 = 0x0103_d3e8;
/// The file name passed to the scope guard in [`fn_005e0200`]
/// (`"d:\_fallout3\platforms\common\code\bsmain\Setting.h"`).
const SETTING_H_FILE: u32 = 0x0101_608c;

/// `"Attempting to set hardcore on an invalid player!"`
const MSG_SET_HARDCORE_INVALID: u32 = 0x0103_d1e8;
/// `"Attempting to reset xp on an invalid player!"`
const MSG_RESET_XP_INVALID: u32 = 0x0103_d21c;
/// `"There are %u DLC packages available"`
const MSG_DLC_COUNT: u32 = 0x0103_d250;
/// `"     Examining '%S' [%s]"`
const MSG_EXAMINING_DLC: u32 = 0x0103_d274;
/// `"Checking for DLC package '%s'"`
const MSG_CHECKING_DLC: u32 = 0x0103_d290;
/// `"IsLoadingMenuEnabled >> %0.f"`
const MSG_LOADING_MENU_ENABLED: u32 = 0x0103_d2b0;
/// `"IsActorRefEssential >> Invalid actor reference "`
const MSG_ESSENTIAL_INVALID: u32 = 0x0103_d2d0;
/// `"IsActorRefEssential >> %0.f"`
const MSG_ESSENTIAL_VALUE: u32 = 0x0103_d300;
/// `"SetActorRefEssential >> Invalid actor reference "`
const MSG_SET_ESSENTIAL_INVALID: u32 = 0x0103_d31c;
/// `"IsNoHumanGoreExe >> %0.f"`
const MSG_NO_HUMAN_GORE: u32 = 0x0103_d350;
/// `"PersistentRefCount >> %u"`
const MSG_PERSISTENT_REF_COUNT: u32 = 0x0103_d36c;
/// `"AlwaysShowActorSubtitles >> Invalid actor reference "`
const MSG_SUBTITLES_INVALID: u32 = 0x0103_d388;
/// `"GetContainerInventoryCount >> %i"`
const MSG_CONTAINER_COUNT: u32 = 0x0103_d3c0;

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
    let (ok, values) = parse_locals(e, a, init);
    ok.then_some(values)
}

/// [`parse_params`] for the commands that ignore the `AL` of
/// `ParseParameters`: that `AL`, and the values left in the locals whatever
/// it was.
fn parse_locals<const N: usize>(e: &mut Engine, a: ScriptArgs, init: [u32; N]) -> (bool, [u32; N]) {
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
    (ok, values)
}

/// Whether the TLS echo flag (commands print their result) is set.
fn echo_enabled(e: &mut Engine) -> bool {
    let tls = e.tls();
    e.mem.u8(tls + TLS_ECHO) != 0
}

/// The condition-function call of the commands that hand over their
/// `thisObj` and the result double: `condition(thisObj, argument, 0,
/// result)`, its `AL`.
fn call_condition(e: &mut Engine, condition: u32, a: ScriptArgs, argument: u32) -> bool {
    e.call(condition, &args![a.this_obj, argument, 0u32, a.result])
        .bool()
}

/// Prints a copy of the string: the game builds a copy on the stack and
/// passes the two words of the object to `Interface::PrintLine`.
fn print_line(e: &mut Engine, string: u32) {
    let (first, second) = e.with_stack(8, |e, copy| {
        e.call(BS_STRING_COPY_CONSTRUCT, &args![copy, string]);
        (e.mem.u32(copy.addr()), e.mem.u32(copy.addr() + 4))
    });
    e.call(PRINT_LINE, &args![first, second]);
}

/// The clamp the angle commands apply to their `float` before the inverse
/// function: `FCOMP` against the `double` -1.0 (only an ordered "less"
/// replaces the value with the `float` -1.0), then against the `double` 1.0
/// (only an ordered "greater" replaces it with 1.0).
fn clamp_to_unit(e: &mut Engine, mut value: f32) -> f32 {
    if (value as f64) < e.global::<f64>(MINUS_ONE_DOUBLE) {
        value = e.global::<f32>(MINUS_ONE_FLOAT);
    }
    if (value as f64) > e.global::<f64>(ONE_DOUBLE) {
        value = 1.0;
    }
    value
}

/// Degrees to radians the way the angle commands do it: the product in
/// `double`, rounded to `float`.
fn to_radians(e: &mut Engine, degrees: f32) -> f32 {
    (degrees as f64 * e.global::<f64>(DEGREES_TO_RADIANS)) as f32
}

// ---- Translated functions --------------------------------------------------------

// Translated from 005db810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSFrontTargetVisibleFunction` (Xbox PDB): parses one value
/// and returns `GetVATSFrontTargetVisibleConditionFunction(thisObj, value,
/// 0, result)`.
pub fn script_get_vats_front_target_visible_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, GET_VATS_FRONT_TARGET_VISIBLE_CONDITION, a, value)
}

// Translated from 005dbe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Increments the 16-bit counter at `011b0808`, skipping 0 (it restarts at
/// 10), and returns it.
pub fn fn_005dbe10(e: &mut Engine) -> u32 {
    let next = e.global::<u32>(COUNTER).wrapping_add(1) & 0xffff;
    e.set_global(COUNTER, next);
    if next == 0 {
        e.set_global(COUNTER, 10u32);
    }
    e.global(COUNTER)
}

// Translated from 005dbe40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any bit of `mask` is set in the flags word at `this + 0x10`.
pub fn fn_005dbe40(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr() + 0x10) & mask != 0
}

// Translated from 005dbe60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Interpolates the vectors of `this` ([`fn_005dbec0`]) into a temporary
/// vector and stores its first three lanes through `00458620` into `output`.
pub fn fn_005dbe60(e: &mut Engine, this: Ptr, output: Ptr) {
    e.with_stack(16, |e, result| {
        e.call(VECTOR_CONSTRUCT_NOOP, &args![result]);
        fn_005dbec0(e, this, result);
        e.call(VECTOR_STORE_THREE, &args![output, result]);
    });
}

// Translated from 005dbec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Linear interpolation between the vector at `this` and the vector at `this
/// + 0x10` with the weight `float` at `this + 0x40` ([`fn_005dbf70`]); the
/// result goes to `output`.
pub fn fn_005dbec0(e: &mut Engine, this: Ptr, output: Ptr) {
    e.with_stack(16, |e, weight| {
        let t = e.mem.f32(this.addr() + 0x40);
        fn_005dbf20(e, weight, t);
        fn_005dbf70(e, output, this, Ptr::new(this.addr() + 0x10), weight);
    });
}

// Translated from 005dbf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the vector `(x, 0, 0, 0)` in `this` and returns `this`.
pub fn fn_005dbf20(e: &mut Engine, this: Ptr, x: f32) -> Ptr {
    e.mem.set_f32(this.addr(), x);
    for lane in 1..4 {
        e.mem.set_f32(this.addr() + 4 * lane, 0.0);
    }
    this
}

// Translated from 005dbf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `output = (1 - t) * a + t * b` lane by lane (single precision, as the
/// `SUBPS`/`MULPS`/`ADDPS` code computes), where `t` is the first lane of
/// `weight` replicated into all four lanes by `00561240`.
pub fn fn_005dbf70(e: &mut Engine, output: Ptr, a: Ptr, b: Ptr, weight: Ptr) {
    e.with_stack(16, |e, t| {
        e.call(VECTOR_SPLAT_X, &args![weight, t]);
        for lane in 0..4u32 {
            let ones: f32 = e.global(VECTOR_ONES + 4 * lane);
            let t_lane = e.mem.f32(t.addr() + 4 * lane);
            let b_lane = e.mem.f32(b.addr() + 4 * lane);
            let a_lane = e.mem.f32(a.addr() + 4 * lane);
            let scaled_b = t_lane * b_lane;
            let scaled_a = (ones - t_lane) * a_lane;
            e.mem.set_f32(output.addr() + 4 * lane, scaled_a + scaled_b);
        }
    });
}

// Translated from 005dc010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer; when `thisObj` is an actor (virtual slot `0x100`),
/// stores it at its `+0x10c` through `008a1a40`. Succeeds when the parameter
/// parsed.
pub fn fn_005dc010(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() && e.vcall(a.this_obj.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        e.call(ACTOR_SET_FIELD_10C, &args![a.this_obj, value]);
    }
    true
}

// Translated from 005dc090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsInCritStageFunction` (Xbox PDB): parses one value and returns
/// `IsInCritStageConditionFunction(thisObj, value, 0, result)`.
pub fn script_is_in_crit_stage_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, IS_IN_CRIT_STAGE_CONDITION, a, value)
}

// Translated from 005dc0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angle command: parses a `float` and an integer flag. With the flag
/// clear the result is `004e44b0(value * pi / 180)`; with the flag set it is
/// `004b5510(clamp(value, -1, 1)) * 180 / pi`. Fails when the parameters do
/// not parse.
pub fn fn_005dc0f0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value_bits, flag]) = parse_params(e, a, [0.0f32.to_bits(), 0]) else {
        return false;
    };
    let value = f32::from_bits(value_bits);
    if flag != 0 {
        let value = clamp_to_unit(e, value);
        let result = e.call(FLOAT_FN_004B5510, &args![value]).f64();
        let degrees = result * e.global::<f64>(RADIANS_TO_DEGREES);
        e.mem.set_f64(a.result.addr(), degrees);
    } else {
        let radians = to_radians(e, value);
        let result = e.call(FLOAT_FN_004E44B0, &args![radians]).f64();
        e.mem.set_f64(a.result.addr(), result);
    }
    true
}

// Translated from 005dc1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same angle command with the functions `004e4470` (flag clear) and
/// [`fn_005dc270`] (flag set, after the same clamp).
pub fn fn_005dc1b0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value_bits, flag]) = parse_params(e, a, [0.0f32.to_bits(), 0]) else {
        return false;
    };
    let value = f32::from_bits(value_bits);
    if flag != 0 {
        let value = clamp_to_unit(e, value);
        let result = fn_005dc270(e, value) as f64;
        let degrees = result * e.global::<f64>(RADIANS_TO_DEGREES);
        e.mem.set_f64(a.result.addr(), degrees);
    } else {
        let radians = to_radians(e, value);
        let result = e.call(FLOAT_FN_004E4470, &args![radians]).f64();
        e.mem.set_f64(a.result.addr(), result);
    }
    true
}

// Translated from 005dc270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its `float` to `004f6e40` and returns that function's `ST0`.
pub fn fn_005dc270(e: &mut Engine, x: f32) -> f32 {
    e.call(FLOAT_FN_004F6E40, &args![x]).f32()
}

// Translated from 005dc290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The angle command without clamping: with the flag clear,
/// `0057dd70(value * pi / 180)`; with the flag set, `005dc330(value) * 180 /
/// pi`.
pub fn fn_005dc290(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value_bits, flag]) = parse_params(e, a, [0.0f32.to_bits(), 0]) else {
        return false;
    };
    let value = f32::from_bits(value_bits);
    if flag != 0 {
        let result = fn_005dc330(e, value) as f64;
        let degrees = result * e.global::<f64>(RADIANS_TO_DEGREES);
        e.mem.set_f64(a.result.addr(), degrees);
    } else {
        let radians = to_radians(e, value);
        let result = e.call(FLOAT_FN_0057DD70, &args![radians]).f64();
        e.mem.set_f64(a.result.addr(), result);
    }
    true
}

// Translated from 005dc330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its `float` to `004b1460` and returns that function's `ST0`.
pub fn fn_005dc330(e: &mut Engine, x: f32) -> f32 {
    e.call(FLOAT_FN_004B1460, &args![x]).f32()
}

// Translated from 005dc350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one `float` and stores `004019b0(value)` in the result double.
pub fn fn_005dc350(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value_bits]) = parse_params(e, a, [0.0f32.to_bits()]) else {
        return false;
    };
    let result = e
        .call(FLOAT_FN_004019B0, &args![f32::from_bits(value_bits)])
        .f64();
    e.mem.set_f64(a.result.addr(), result);
    true
}

// Translated from 005dc3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A logarithm command: parses a `float` and a base (default `e`, the float
/// at `0103d018`); the result is 0 when the base is 1 and
/// `log(value) / log(base)` (through [`fn_005dc440`]) otherwise.
pub fn fn_005dc3b0(e: &mut Engine, a: ScriptArgs) -> bool {
    let euler = e.global::<f32>(EULER_NUMBER);
    let Some([value_bits, base_bits]) = parse_params(e, a, [0.0f32.to_bits(), euler.to_bits()])
    else {
        return false;
    };
    let value = f32::from_bits(value_bits);
    let base = f32::from_bits(base_bits);
    // `FCOMP`: an unordered comparison (NaN) takes the division branch.
    if base as f64 == e.global::<f64>(ONE_DOUBLE) {
        e.mem.set_f64(a.result.addr(), 0.0);
    } else {
        let numerator = fn_005dc440(e, value) as f64;
        let denominator = fn_005dc440(e, base) as f64;
        e.mem.set_f64(a.result.addr(), numerator / denominator);
    }
    true
}

// Translated from 005dc440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its `float` to [`fn_005dc460`].
pub fn fn_005dc440(e: &mut Engine, x: f32) -> f32 {
    fn_005dc460(e, x)
}

// Translated from 005dc460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The single-precision wrapper of the CRT `double` function at `00eca920`
/// (the library match calls it `_logf`): the argument widened to `double`,
/// the result rounded to `float`.
pub fn fn_005dc460(e: &mut Engine, x: f32) -> f32 {
    let wide = e.call(CRT_DOUBLE_FN_00ECA920, &args![x as f64]).f64();
    wide as f32
}

// Translated from 005dc480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one `float` and stores `00408840(value)` in the result double.
pub fn fn_005dc480(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value_bits]) = parse_params(e, a, [0.0f32.to_bits()]) else {
        return false;
    };
    let result = e
        .call(FLOAT_FN_00408840, &args![f32::from_bits(value_bits)])
        .f64();
    e.mem.set_f64(a.result.addr(), result);
    true
}

// Translated from 005dc4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ForceTerminalBackFunction` (Xbox PDB): finds the menu of tile
/// class `0x421`, takes its menu object and, if there is one, shows the
/// previous terminal. Always succeeds.
pub fn script_force_terminal_back_function(e: &mut Engine, _a: ScriptArgs) -> bool {
    let tile = e.call(TILE_GET_MENU_BY_CLASS, &args![0x421u32]).u32();
    let menu = if tile != 0 {
        e.call(TILE_GET_MENU, &args![tile]).u32()
    } else {
        0
    };
    if menu != 0 {
        e.call(DISPLAY_PREVIOUS_TERMINAL, &args![menu]);
    }
    true
}

// Translated from 005dc530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleDepthBiasFunction` (Xbox PDB): flips the byte at
/// `011ad80f` and prints `"Depth Bias is now ON!"` or `"... OFF!"`. Always
/// succeeds.
pub fn script_toggle_depth_bias_function(e: &mut Engine, _a: ScriptArgs) -> bool {
    let enabled = e.global::<u8>(DEPTH_BIAS_FLAG) == 0;
    e.set_global(DEPTH_BIAS_FLAG, enabled as u8);
    let text = if enabled { TEXT_ON } else { TEXT_OFF };
    e.call(CONSOLE_PRINT, &args![MSG_DEPTH_BIAS, text]);
    true
}

// Translated from 005dc580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `FalloutRadio::PipboyRadioEnable(0)`. Always succeeds.
pub fn fn_005dc580(e: &mut Engine, _a: ScriptArgs) -> bool {
    e.call(PIPBOY_RADIO_ENABLE, &args![0u32]);
    true
}

// Translated from 005dc5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `IsGoreDisabled` body: returns
/// `IsGoreDisabledConditionFunction(thisObj, 0, 0, result)`.
pub fn fn_005dc5a0(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_GORE_DISABLED_CONDITION, a, 0)
}

// Translated from 005dc5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetActorsInHigh` body: returns
/// `GetActorsInHighConditionFunction(thisObj, 0, 0, result)`.
pub fn fn_005dc5c0(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, GET_ACTORS_IN_HIGH_CONDITION, a, 0)
}

// Translated from 005dc5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetMinimalUseFunction` (Xbox PDB): parses a flag. When `thisObj`
/// has a base form of type `0x1c` (the door), the door's minimal-use state is
/// set to `flag != 0` and the player checks its quest targets; otherwise the
/// logging stub gets the message that the command must be called on a door
/// reference (with the script's name, virtual slot `0x130`). Succeeds when the
/// parameter parsed.
pub fn script_set_minimal_use_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() && e.call(GET_BASE_FORM, &args![a.this_obj]).u32() != 0 {
        let base_form = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
        if e.call(FORM_TYPE, &args![base_form]).u32() == 0x1c {
            let door = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
            e.call(DOOR_SET_MINIMAL_USE, &args![door, (flag != 0) as u32]);
            let player = e.global::<u32>(PLAYER);
            e.call(CHECK_QUEST_TARGET_UPDATE, &args![player, player]);
            return true;
        }
    }
    let name = e.vcall(a.script_obj.addr(), VSLOT_NAME, &args![]).u32();
    e.call(LOG_STUB, &args![MSG_SET_MINIMAL_USE, name]);
    true
}

// Translated from 005dc6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer and stores `value != 0` as the byte at the player's
/// `+0x7c7` ([`fn_005dc710`]).
pub fn fn_005dc6a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    let player = e.global::<u32>(PLAYER);
    fn_005dc710(e, Ptr::new(player), (value != 0) as u8);
    true
}

// Translated from 005dc710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `this + 0x7c7` (`PlayerCharacter`, PC offset).
pub fn fn_005dc710(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x7c7, value);
}

// Translated from 005dc730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowQuestStagesFunction` (Xbox PDB): parses a quest. With the TLS
/// echo flag set, prints one line per stage of the quest's stage list
/// (`quest + 0x44`): `"Stage <index>: <flag> "`, followed, when the stage's
/// item list (`stage + 4`) has a second node, by `" (Item <n>: <flag>, Item
/// ...)"` (an item list of one node prints no items). Succeeds when the
/// parameter parsed.
pub fn script_show_quest_stages_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([quest]) = parse_params(e, a, [0]) else {
        return false;
    };
    let string = e.mem.alloc(8);
    let buffer = e.mem.alloc(0x20);
    e.call(BS_STRING_CONSTRUCT, &args![string]);
    if echo_enabled(e) {
        let mut node = e.call(QUEST_STAGE_LIST, &args![quest]).u32();
        while node != 0 {
            let item_cell = e.call(LIST_ITEM_PTR, &args![node]).u32();
            if e.mem.u32(item_cell) == 0 {
                break;
            }
            let stage_cell = e.call(LIST_ITEM_PTR, &args![node]).u32();
            let stage = e.mem.u32(stage_cell);
            let stage_flag = fn_005dc960(e, Ptr::new(stage)) != 0;
            let index = e.call(STAGE_INDEX, &args![stage]).u8();
            e.call(
                SPRINTF_S,
                &args![
                    buffer,
                    0x1fu32,
                    FORMAT_STAGE,
                    index as u32,
                    stage_flag as u32
                ],
            );
            e.call(BS_STRING_ASSIGN, &args![string, buffer]);
            if e.call(PLUS_4, &args![stage]).u32() != 0 {
                let items = e.call(PLUS_4, &args![stage]).u32();
                if e.call(LIST_NEXT, &args![items]).u32() != 0 {
                    e.call(BS_STRING_APPEND, &args![string, TEXT_OPEN_PAREN]);
                    let mut item_node = e.call(PLUS_4, &args![stage]).u32();
                    while item_node != 0 {
                        let cell = e.call(LIST_ITEM_PTR, &args![item_node]).u32();
                        if e.mem.u32(cell) == 0 {
                            break;
                        }
                        let cell = e.call(LIST_ITEM_PTR, &args![item_node]).u32();
                        let item = e.mem.u32(cell);
                        let has_process = e.call(GET_PROCESS, &args![item]).u32() != 0;
                        let value = fn_005dc980(e, Ptr::new(item));
                        e.call(
                            SPRINTF_S,
                            &args![
                                buffer,
                                0x1fu32,
                                FORMAT_ITEM,
                                value as u32,
                                has_process as u32
                            ],
                        );
                        e.call(BS_STRING_APPEND, &args![string, buffer]);
                        if e.call(LIST_NEXT, &args![item_node]).u32() != 0 {
                            e.call(BS_STRING_APPEND, &args![string, TEXT_SEPARATOR]);
                        }
                        item_node = e.call(LIST_NEXT, &args![item_node]).u32();
                    }
                    e.call(BS_STRING_APPEND, &args![string, TEXT_CLOSE_PAREN]);
                }
            }
            print_line(e, string);
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
    }
    e.call(BS_STRING_DESTRUCT, &args![string]);
    e.mem.free(buffer);
    e.mem.free(string);
    true
}

// Translated from 005dc960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 1` of a quest stage (read as a flag by
/// [`script_show_quest_stages_function`]).
pub fn fn_005dc960(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 1)
}

// Translated from 005dc980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 100` of a quest stage item.
pub fn fn_005dc980(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 100)
}

/// The command's list of names: `"%d. %s, "` for every name but the last
/// (`"%d. %s"`), appended to a string that is then printed.
fn print_name_list(e: &mut Engine, name_function: u32, count: i32) {
    let string = e.mem.alloc(8);
    let buffer = e.mem.alloc(0x200);
    e.call(BS_STRING_CONSTRUCT, &args![string]);
    for index in 0..count {
        let name = e.call(name_function, &args![index]).u32();
        let format = if index == count - 1 {
            FORMAT_NAME_LAST
        } else {
            FORMAT_NAME_MORE
        };
        e.call(SPRINTF_S, &args![buffer, 0x200u32, format, index, name]);
        e.call(BS_STRING_APPEND, &args![string, buffer]);
    }
    print_line(e, string);
    e.call(BS_STRING_DESTRUCT, &args![string]);
    e.mem.free(buffer);
    e.mem.free(string);
}

/// The weight the commands store for a parsed percentage: `value / 100`
/// rounded to `float`.
fn percent(e: &mut Engine, value: i32) -> f32 {
    (value as f64 / e.global::<f64>(HUNDRED)) as f32
}

// Translated from 005dc9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ModifyFaceGen` (Xbox PDB): parses a keyword (a 0x200-byte
/// string), an index (default -1) and a value (default 0). An empty keyword
/// prints `"expression"`. The object it works on is virtual slot `0x1b8` of
/// `thisObj` (called with 0); without one nothing happens. The keywords
/// (compared without case):
///
/// - `expression`: without an index prints the 15 expression names;
///   otherwise virtual slot `0x114` of the object (`index, value / 100`).
/// - `phoneme`: without an index prints the 16 names; otherwise builds the
///   16 current phoneme values (virtual slot `0xa4`), puts `value / 100` at
///   the index and passes the array to slot `0xec` with 1.0.
/// - `modifier`: without an index prints the 17 names; otherwise stores 0
///   in the face generation mode byte (`005dd860`), builds the 17 modifier
///   values ([`fn_005dd4a0`]) with the same replacement and passes them to
///   slot `0xf0` with 1.0.
/// - `coord`: index -1 attaches the reference to the object of slot `0x1b0`
///   (`+0xe8`, `+0xec` = 0 and the bytes cleared by [`fn_005dd520`] and
///   [`fn_005dd540`]); index 1 does the same with `+0xec` = 1, the bytes set
///   and the value stored at `011d59f0`; index 0 rebuilds the head of an
///   actor `thisObj` (slot `0x218` true) from its race: it removes the
///   head objects (slots `0x1b0` and `0x1ac`) from the palette, clears the
///   head of the base form ([`tes_npc_clear_head`]), initialises it again and
///   regenerates it from a fresh face generation data
///   ([`fn_005dd590`], [`fn_005dd6f0`]).
/// - `reset`: virtual slot `0xb4` of the object (`1.0, 1, 1, 1, 1, 0`), then
///   the mode byte is set to 1.
///
/// Succeeds when the parameters parsed.
pub fn script_modify_face_gen(e: &mut Engine, a: ScriptArgs) -> bool {
    let keyword = e.mem.alloc(0x200);
    e.mem.set_u8(keyword, 0);
    e.call(MEMSET, &args![keyword + 1, 0u32, 0x1ffu32]);
    let numbers = e.mem.alloc(8);
    e.mem.set_i32(numbers, -1);
    e.mem.set_i32(numbers + 4, 0);
    let parsed = parse(e, a, &[keyword, numbers, numbers + 4]);
    let index = e.mem.i32(numbers);
    let value = e.mem.i32(numbers + 4);
    if parsed {
        modify_face_gen(e, a, keyword, index, value);
    }
    e.mem.free(numbers);
    e.mem.free(keyword);
    parsed
}

/// The body of [`script_modify_face_gen`] after the parameters parsed.
fn modify_face_gen(e: &mut Engine, a: ScriptArgs, keyword: u32, index: i32, value: i32) {
    if e.mem.i8(keyword) == 0 {
        e.call(CONSOLE_PRINT, &args![KEYWORD_EXPRESSION]);
        return;
    }
    let target = if a.this_obj.is_null() {
        0
    } else {
        e.vcall(a.this_obj.addr(), VSLOT_FACE_GEN_TARGET, &args![0u32])
            .u32()
    };
    if target == 0 {
        return;
    }
    let is_keyword = |e: &mut Engine, text: u32| e.call(STRICMP, &args![keyword, text]).i32() == 0;
    if is_keyword(e, KEYWORD_EXPRESSION) {
        if index == -1 {
            print_name_list(e, EXPRESSION_NAME, 15);
        } else {
            let weight = percent(e, value);
            e.vcall(target, VSLOT_SET_EXPRESSION, &args![index, weight]);
        }
    } else if is_keyword(e, KEYWORD_PHONEME) {
        if index == -1 {
            print_name_list(e, PHONEME_NAME, 16);
        } else {
            let values = e.mem.alloc(0x40);
            e.call(ZERO_FILL, &args![values, 0u32, 0x40u32]);
            for i in 0..16 {
                let element = if i == index {
                    percent(e, value)
                } else {
                    e.vcall(target, VSLOT_GET_PHONEME, &args![i]).f32()
                };
                e.mem.set_f32(values + 4 * i as u32, element);
            }
            e.vcall(target, VSLOT_SET_PHONEMES, &args![values, 1.0f32]);
            e.mem.free(values);
        }
    } else if is_keyword(e, KEYWORD_MODIFIER) {
        if index == -1 {
            print_name_list(e, MODIFIER_NAME, 17);
        } else {
            e.call(SET_FACE_GEN_MODE_BYTE, &args![0u32]);
            let values = e.mem.alloc(0x44);
            e.call(ZERO_FILL, &args![values, 0u32, 0x44u32]);
            for i in 0..17 {
                let element = if i == index {
                    percent(e, value)
                } else {
                    fn_005dd4a0(e, Ptr::new(target), i)
                };
                e.mem.set_f32(values + 4 * i as u32, element);
            }
            e.vcall(target, VSLOT_SET_MODIFIERS, &args![values, 1.0f32]);
            e.mem.free(values);
        }
    } else if is_keyword(e, KEYWORD_COORD) {
        match index {
            -1 => attach_face_gen_object(e, a.this_obj.addr(), false),
            0 => rebuild_head(e, a.this_obj.addr()),
            1 => {
                attach_face_gen_object(e, a.this_obj.addr(), true);
                e.set_global(FACE_GEN_COORD_VALUE, value);
            }
            _ => {}
        }
    } else if is_keyword(e, KEYWORD_RESET) {
        e.vcall(
            target,
            VSLOT_RESET,
            &args![1.0f32, 1u32, 1u32, 1u32, 1u32, 0u32],
        );
        e.call(SET_FACE_GEN_MODE_BYTE, &args![1u32]);
    }
}

/// The `coord` cases -1 and 1: the object of virtual slot `0x1b0` of
/// `reference` is told about it (`+0xe8`) and gets its flag byte (`+0xec`) and
/// the two bytes at `+0xd4` / `+0xe3` set to `on`.
fn attach_face_gen_object(e: &mut Engine, reference: u32, on: bool) {
    let object = e.vcall(reference, VSLOT_OBJECT_1B0, &args![0u32]).u32();
    e.mem.set_u32(object + 0xe8, reference);
    e.mem.set_u8(object + 0xec, on as u8);
    fn_005dd520(e, Ptr::new(object), on as u8);
    fn_005dd540(e, Ptr::new(object), on as u8);
}

/// The `coord 0` case: rebuilds the head of an actor.
fn rebuild_head(e: &mut Engine, reference: u32) {
    if reference == 0 || !e.vcall(reference, VSLOT_FLAG_218, &args![]).bool() {
        return;
    }
    let base_form = e.call(GET_BASE_FORM, &args![reference]).u32();
    let node = e.vcall(reference, VSLOT_NODE, &args![]).u32();
    let node_data = if node != 0 {
        e.vcall(node, VSLOT_NODE_DATA, &args![]).u32()
    } else {
        0
    };
    if node_data != 0 {
        let mut skeleton = 0;
        if e.vcall(reference, VSLOT_ANIMATION, &args![]).u32() != 0 {
            let animation = e.vcall(reference, VSLOT_ANIMATION, &args![]).u32();
            if e.call(POINTER_AT_D8, &args![animation]).u32() != 0 {
                let animation = e.vcall(reference, VSLOT_ANIMATION, &args![]).u32();
                let inner = e.call(POINTER_AT_D8, &args![animation]).u32();
                skeleton = e.call(POINTER_AT_78, &args![inner]).u32();
            }
        }
        let object = e.vcall(reference, VSLOT_OBJECT_1B0, &args![0u32]).u32();
        if object != 0 && e.call(FIELD_18, &args![object]).u32() != 0 {
            e.call(RECURSE_REMOVE_FROM_PALETTE, &args![object, skeleton]);
            let process = e.call(GET_PROCESS, &args![reference]).u32();
            e.vcall(process, VSLOT_PROCESS_79C, &args![0u32]);
            let owner = e.call(FIELD_18, &args![object]).u32();
            e.vcall(owner, VSLOT_NODE_E8, &args![object]);
        }
        let object = e.vcall(reference, VSLOT_OBJECT_1AC, &args![0u32]).u32();
        if object != 0 && e.call(FIELD_18, &args![object]).u32() != 0 {
            e.call(RECURSE_REMOVE_FROM_PALETTE, &args![object, skeleton]);
            let process = e.call(GET_PROCESS, &args![reference]).u32();
            e.vcall(process, VSLOT_PROCESS_794, &args![0u32]);
            let process = e.call(GET_PROCESS, &args![reference]).u32();
            e.vcall(process, VSLOT_PROCESS_7A4, &args![0u32]);
            let owner = e.call(FIELD_18, &args![object]).u32();
            e.vcall(owner, VSLOT_NODE_E8, &args![object]);
        }
    }
    let process = e.call(GET_PROCESS, &args![reference]).u32();
    e.vcall(process, VSLOT_PROCESS_58, &args![]);
    tes_npc_clear_head(e, Ptr::new(base_form));
    e.with_stack(8, |e, outputs| {
        e.call(
            TES_NPC_INIT_HEAD,
            &args![base_form, outputs, outputs.addr() + 4],
        );
        let first = e.mem.u32(outputs.addr());
        let second = e.mem.u32(outputs.addr() + 4);
        let data = e.vcall(reference, VSLOT_BASE_DATA, &args![]).u32();
        e.call(
            TES_NPC_UPDATE_HEAD,
            &args![base_form, reference, data, first, second],
        );
    });
    e.with_stack(0xf0, |e, face_gen| {
        fn_005dd590(e, face_gen);
        let race = e.call(TES_NPC_GET_RACE, &args![base_form]).u32();
        e.call(
            TES_RACE_GET_FACE_GEN_DATA,
            &args![race, base_form, face_gen, 0u32, 0u32],
        );
        e.with_stack(16, |e, scratch| {
            e.call(
                FLOAT_AND_FLAGS_CONSTRUCT,
                &args![scratch, 0.0f32, 0u32, 0u32],
            );
            e.call(NODE_APPLY, &args![node, scratch]);
        });
        fn_005dd6f0(e, face_gen);
    });
}

// Translated from 005dd4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The modifier value number `index` of the face generation target: `0.0` for
/// an index of 17 or more (signed comparison), else the value of the float
/// array at `this + 0xa8` ([`fn_005dd4e0`]).
pub fn fn_005dd4a0(e: &mut Engine, this: Ptr, index: i32) -> f32 {
    if index < 0x11 {
        fn_005dd4e0(e, Ptr::new(this.addr() + 0xa8), index as u32)
    } else {
        0.0
    }
}

// Translated from 005dd4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Element `index` of a `float` array (pointer at `this + 0xc`, count at
/// `this + 0x10`), or `0.0` when the index is out of range (unsigned).
pub fn fn_005dd4e0(e: &mut Engine, this: Ptr, index: u32) -> f32 {
    if index < e.mem.u32(this.addr() + 0x10) {
        let data = e.mem.u32(this.addr() + 0xc);
        e.mem.f32(data.wrapping_add(index.wrapping_mul(4)))
    } else {
        0.0
    }
}

// Translated from 005dd520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `this + 0xd4`.
pub fn fn_005dd520(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xd4, value);
}

// Translated from 005dd540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `this + 0xe3`.
pub fn fn_005dd540(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xe3, value);
}

// Translated from 005dd560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::ClearHead` (Xbox PDB): empties the `NiPointer` at `this + 0x1c4`
/// and assigns it to the one at `this + 0x1c8`.
pub fn tes_npc_clear_head(e: &mut Engine, this: Ptr) {
    let cleared = e
        .call(NI_POINTER_SET, &args![this.addr() + 0x1c4, 0u32])
        .u32();
    e.call(NI_POINTER_ASSIGN, &args![this.addr() + 0x1c8, cleared]);
}

// Translated from 005dd590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the face generation data (0xec bytes): four 0x20-byte
/// elements, four 16-byte arrays (`+0x94`, `+0xa4`, `+0xb4`, `+0xc4`), a
/// list head at `+0xe4`; the fields `+0x80..+0x90` are cleared, the arrays
/// reset, the bytes at `+0xd4` / `+0xd5` cleared and `+0xe0` set to -1.
/// Returns `this`.
pub fn fn_005dd590(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(
        VECTOR_CONSTRUCT,
        &args![base, 0x20u32, 4u32, ELEMENT_CONSTRUCT, ELEMENT_DESTRUCT],
    );
    e.call(ARRAY_94_CONSTRUCT, &args![base + 0x94, 0u32, 1u32]);
    e.call(ARRAY_A4_CONSTRUCT, &args![base + 0xa4, 0u32, 1u32]);
    e.call(ARRAY_B4_CONSTRUCT, &args![base + 0xb4, 0u32, 1u32]);
    e.call(ARRAY_C4_CONSTRUCT, &args![base + 0xc4, 0u32, 1u32]);
    e.call(LIST_CONSTRUCT, &args![base + 0xe4]);
    e.mem.set_u32(base + 0x80, 0);
    e.mem.set_u32(base + 0x84, 0);
    e.mem.set_f32(base + 0x88, 0.0);
    e.mem.set_u32(base + 0x8c, 0);
    e.mem.set_u32(base + 0x90, 0);
    e.call(ARRAY_RESET, &args![base + 0x94]);
    e.call(ARRAY_RESET, &args![base + 0xa4]);
    e.call(ARRAY_RESET, &args![base + 0xb4]);
    e.mem.set_u8(base + 0xd4, 0);
    e.mem.set_u8(base + 0xd5, 0);
    e.mem.set_u32(base + 0xe0, 0xffff_ffff);
    this
}

// Translated from 005dd6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the face generation data: the list at `+0xe4`, the arrays at
/// `+0xc4`, `+0xb4`, `+0xa4` and `+0x94`, then the four 0x20-byte elements.
pub fn fn_005dd6f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(LIST_PRE_DESTRUCT, &args![base + 0xe4]);
    e.call(LIST_DESTRUCT, &args![base + 0xe4]);
    e.call(ARRAY_C4_DESTRUCT_WRAPPER, &args![base + 0xc4]);
    e.call(ARRAY_B4_DESTRUCT_WRAPPER, &args![base + 0xb4]);
    fn_005dd7d0(e, Ptr::new(base + 0xa4));
    fn_005dd7b0(e, Ptr::new(base + 0x94));
    e.call(
        VECTOR_DESTRUCT,
        &args![base, 0x20u32, 4u32, ELEMENT_DESTRUCT],
    );
}

// Translated from 005dd7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the array at `+0x94` of the face generation data: calls
/// `005e0340`.
pub fn fn_005dd7b0(e: &mut Engine, this: Ptr) {
    e.call(ARRAY_94_DESTRUCT, &args![this]);
}

// Translated from 005dd7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the array at `+0xa4` of the face generation data: calls
/// `005e03a0`.
pub fn fn_005dd7d0(e: &mut Engine, this: Ptr) {
    e.call(ARRAY_A4_DESTRUCT, &args![this]);
}

// Translated from 005dd7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor wrapper of the array at `+0xb4` of the face generation data:
/// calls `005e0460`.
pub fn fn_005dd7f0(e: &mut Engine, this: Ptr) {
    e.call(ARRAY_B4_DESTRUCT, &args![this]);
}

// Translated from 005dd810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor wrapper of the array at `+0xc4` of the face generation data:
/// calls `005e02e0`.
pub fn fn_005dd810(e: &mut Engine, this: Ptr) {
    e.call(ARRAY_C4_DESTRUCT, &args![this]);
}

// Translated from 005dd830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The text of expression number `index`: the pointer at that index of the
/// table at `0119b41c` (no range check).
pub fn fn_005dd830(e: &mut Engine, index: i32) -> u32 {
    e.mem
        .u32(EXPRESSION_NAME_TABLE.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 005dd840 (decompiled, FalloutNV.exe 1.4.0.525)
/// The text of modifier number `index`: the pointer at that index of the
/// table at `0119b458` (no range check).
pub fn fn_005dd840(e: &mut Engine, index: i32) -> u32 {
    e.mem
        .u32(MODIFIER_NAME_TABLE.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 005dd850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The text of phoneme number `index`: the pointer at that index of the
/// table at `0119b4a0` (no range check).
pub fn fn_005dd850(e: &mut Engine, index: i32) -> u32 {
    e.mem
        .u32(PHONEME_NAME_TABLE.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 005dd860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `011d59e0`, the mode byte of the face generation
/// commands.
pub fn fn_005dd860(e: &mut Engine, value: u8) {
    e.set_global(FACE_GEN_MODE_BYTE, value);
}

/// Parses the one string parameter (a 256-byte buffer) of the code
/// commands. Returns the buffer when the parameters parsed; the caller frees
/// it.
fn parse_text(e: &mut Engine, a: ScriptArgs) -> Option<u32> {
    let text = e.mem.alloc(0x100);
    if parse(e, a, &[text]) {
        Some(text)
    } else {
        e.mem.free(text);
        None
    }
}

// Translated from 005dd870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsCodeRedeemedFunction` (Xbox PDB): parses a text and asks the
/// code record whether it is redeemed. Prints `"IsCodeRedeemed [<text>] =
/// Yes"` (or `No`), stores 1.0 or 0.0 as the result and returns the answer
/// byte itself (0 when the parameters do not parse).
pub fn script_is_code_redeemed_function(e: &mut Engine, a: ScriptArgs) -> u8 {
    let Some(text) = parse_text(e, a) else {
        return 0;
    };
    let records = e.call(CODE_RECORDS, &args![]).u32();
    let redeemed = e.call(CODE_IS_REDEEMED, &args![records, text]).u8();
    let answer = if redeemed != 0 { TEXT_YES } else { TEXT_NO };
    let line = e.mem.alloc(200);
    e.call(
        SPRINTF_S,
        &args![line, 200u32, FORMAT_IS_CODE_REDEEMED, text, answer],
    );
    e.call(CONSOLE_PRINT, &args![line]);
    e.mem
        .set_f64(a.result.addr(), if redeemed != 0 { 1.0 } else { 0.0 });
    e.mem.free(line);
    e.mem.free(text);
    redeemed
}

// Translated from 005dd960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a text and sets its bit in the code record; the result is 1.0 when
/// that call answered true, else 0.0, and the answer byte is returned (0 when
/// the parameters do not parse).
pub fn fn_005dd960(e: &mut Engine, a: ScriptArgs) -> u8 {
    let Some(text) = parse_text(e, a) else {
        return 0;
    };
    let records = e.call(CODE_RECORDS, &args![]).u32();
    let known = e.call(CODE_SET_REDEEMED, &args![records, text]).u8();
    e.mem
        .set_f64(a.result.addr(), if known != 0 { 1.0 } else { 0.0 });
    e.mem.free(text);
    known
}

// Translated from 005dda00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a text and clears its bit in the code record. Succeeds when the
/// parameters parsed.
pub fn fn_005dda00(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some(text) = parse_text(e, a) else {
        return false;
    };
    let records = e.call(CODE_RECORDS, &args![]).u32();
    e.call(CODE_CLEAR_REDEEMED, &args![records, text]);
    e.mem.free(text);
    true
}

// Translated from 005dda70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses no value. When `thisObj` is an actor (virtual slot `0x100`) the
/// result is the byte at `+0x1b1` of it, as a `double`. Succeeds when the
/// parameters parsed.
pub fn fn_005dda70(e: &mut Engine, a: ScriptArgs) -> bool {
    if !parse(e, a, &[]) {
        return false;
    }
    if e.vcall(a.this_obj.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        let byte = e.call(ACTOR_BYTE_1B1, &args![a.this_obj]).u8();
        e.mem.set_f64(a.result.addr(), byte as f64);
    }
    true
}

// Translated from 005ddae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses two integers (the first is not used) and, when `thisObj` is an
/// actor and the second, a slot number, is 1 to 3, sets bit `slot - 1` as the
/// weapon mod slot of the extra data list that its process leads to
/// (`ExtraDataList::SetWeaponModSlot`). Succeeds when the parameters parsed.
pub fn fn_005ddae0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([_first, slot]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    let slot = slot as i32;
    if e.vcall(a.this_obj.addr(), VSLOT_IS_ACTOR, &args![]).bool() && slot > 0 && slot < 4 {
        let process = e.call(GET_PROCESS, &args![a.this_obj]).u32();
        let holder = e.vcall(process, VSLOT_PROCESS_148, &args![]).u32();
        let pointer = e.call(READ_POINTER, &args![holder]).u32();
        let item = e.call(LIST_ITEM_PTR, &args![pointer]).u32();
        let extra_data_list = e.mem.u32(item);
        let mask = 1u32 << ((slot as u32).wrapping_sub(1) & 0x1f);
        e.call(SET_WEAPON_MOD_SLOT, &args![extra_data_list, mask]);
    }
    true
}

/// The commands of `fn_005ddbb0` and `fn_005ddc20`: parse an object and an
/// index; an index of 0 or 1 gives `getter(object, index)` as a `double`
/// result, anything else makes the command fail.
fn get_float_by_index(e: &mut Engine, a: ScriptArgs, getter: u32) -> bool {
    let Some([object, index]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if !(0..=1).contains(&(index as i32)) {
        return false;
    }
    let value = e.call(getter, &args![object, index]).f64();
    e.mem.set_f64(a.result.addr(), value);
    true
}

// Translated from 005ddbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an object and an index (0 or 1) and stores
/// `006155c0(object, index)` as the result.
pub fn fn_005ddbb0(e: &mut Engine, a: ScriptArgs) -> bool {
    get_float_by_index(e, a, OBJECT_FLOAT_GETTER_006155C0)
}

// Translated from 005ddc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an object and an index (0 or 1) and stores
/// `00616a60(object, index)` as the result.
pub fn fn_005ddc20(e: &mut Engine, a: ScriptArgs) -> bool {
    get_float_by_index(e, a, OBJECT_FLOAT_GETTER_00616A60)
}

// Translated from 005ddc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an object and an index (0 to 2) and stores the signed byte
/// `00616a90(object, index)` returns as the result. Fails for another index.
pub fn fn_005ddc90(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([object, index]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if !(0..=2).contains(&(index as i32)) {
        return false;
    }
    let value = e
        .call(OBJECT_BYTE_GETTER_00616A90, &args![object, index & 0xff])
        .u8() as i8;
    e.mem.set_f64(a.result.addr(), value as f64);
    true
}

/// The commands of `005ddd10` to `005ddf10`: parse an object, an index and a
/// value word (an integer or the bits of a `float`); an index of 0 or 1 calls
/// `setter(object, index, value)`, anything else makes the command fail.
fn set_by_index(e: &mut Engine, a: ScriptArgs, setter: u32) -> bool {
    let Some([object, index, value]) = parse_params(e, a, [0, 0, 0]) else {
        return false;
    };
    if !(0..=1).contains(&(index as i32)) {
        return false;
    }
    e.call(setter, &args![object, index, value]);
    true
}

// Translated from 005ddd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an object, an index (0 or 1) and an integer and calls
/// `00615c90(object, index, value)`.
pub fn fn_005ddd10(e: &mut Engine, a: ScriptArgs) -> bool {
    set_by_index(e, a, OBJECT_SET_INT_00615C90)
}

// Translated from 005ddd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an object, an index (0 or 1) and a `float` and calls
/// `00615730(object, index, value)`.
pub fn fn_005ddd90(e: &mut Engine, a: ScriptArgs) -> bool {
    set_by_index(e, a, OBJECT_FLOAT_SETTER_00615730)
}

// Translated from 005dde10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an object, an index (0 or 1) and an integer and calls
/// `00615fa0(object, index, value)`.
pub fn fn_005dde10(e: &mut Engine, a: ScriptArgs) -> bool {
    set_by_index(e, a, OBJECT_SET_INT_00615FA0)
}

// Translated from 005dde90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an object, an index (0 or 1) and a `float` and calls
/// `00615a00(object, index, value)`.
pub fn fn_005dde90(e: &mut Engine, a: ScriptArgs) -> bool {
    set_by_index(e, a, OBJECT_FLOAT_SETTER_00615A00)
}

// Translated from 005ddf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an object, an index (0 or 1) and a `float` and calls
/// `00615580(object, index, value)`.
pub fn fn_005ddf10(e: &mut Engine, a: ScriptArgs) -> bool {
    set_by_index(e, a, OBJECT_FLOAT_SETTER_00615580)
}

// Translated from 005ddf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command without parameters. With a `thisObj`, visits the references of
/// the array at `011e0e84` (from the index stored at the array's `+0x10`, up
/// to its count): a reference is skipped when its flag word has bit
/// `0x200000` set or virtual slot `0x224` is false, or when it is neither
/// `thisObj`'s own `+0xfc` value nor owned by what `thisObj`'s virtual slot
/// `0x1a4` returns. For the others whose base form passes the test of
/// [`fn_005de080`], bit `0x100000` of the word at `+0xc8` is set
/// ([`fn_005de0a0`]). Always succeeds. (A null element would crash the game
/// at its virtual call, as here.)
pub fn fn_005ddf90(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return true;
    }
    let list = e.call(PLUS_4, &args![REFERENCE_LIST_OWNER]).u32();
    let mut index = fn_005de0f0(e, Ptr::new(list), 0);
    while index < e.call(LIST_COUNT, &args![list, 0u32]).u32() {
        let reference = e.call(LIST_GET, &args![list, index]).u32();
        visit_reference(e, a.this_obj.addr(), reference);
        index = index.wrapping_add(1);
    }
    true
}

/// The body of the loop of [`fn_005ddf90`] for one reference.
fn visit_reference(e: &mut Engine, this_obj: u32, reference: u32) {
    if reference != 0 && e.call(FLAG_200000, &args![reference]).bool() {
        return;
    }
    if !e.vcall(reference, VSLOT_REFERENCE_224, &args![]).bool() {
        return;
    }
    if e.call(FIELD_FC, &args![reference]).u32() != this_obj {
        let owner = e.call(GET_OWNER, &args![reference]).u32();
        let wanted = e.vcall(this_obj, VSLOT_REFERENCE_1A4, &args![]).u32();
        if owner != wanted {
            return;
        }
    }
    let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    if fn_005de080(e, Ptr::new(base_form)) {
        fn_005de0a0(e, Ptr::new(reference), FLAG_SET_BY_LOOP, 1);
    }
}

// Translated from 005de080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x400` of the flag word of the form is set (`004fd420`).
pub fn fn_005de080(e: &mut Engine, this: Ptr) -> bool {
    e.call(FORM_FLAG_TEST, &args![this, FORM_FLAG_400]).bool()
}

// Translated from 005de0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` not 0) or clears the bits of `mask` in the word at
/// `this + 0xc8`.
pub fn fn_005de0a0(e: &mut Engine, this: Ptr, mask: u32, set: u8) {
    let flags = e.mem.u32(this.addr() + 0xc8);
    let flags = if set != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    e.mem.set_u32(this.addr() + 0xc8, flags);
}

// Translated from 005de0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x10 + index * 4`.
pub fn fn_005de0f0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.mem.u32(
        this.addr()
            .wrapping_add(0x10)
            .wrapping_add(index.wrapping_mul(4)),
    )
}

// Translated from 005de110 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command without parameters: visits every reference of the array at
/// `011e0e84` (from index 0 to its count); each non-null one whose virtual
/// slot `0x224` and then slot `0x318` are true gets virtual slot `0x320`
/// called with the player and 1. Always succeeds.
pub fn fn_005de110(e: &mut Engine, _a: ScriptArgs) -> bool {
    let mut index = 0u32;
    loop {
        let list = e.call(PLUS_4, &args![REFERENCE_LIST_OWNER]).u32();
        if index >= e.call(LIST_COUNT, &args![list, 0u32]).u32() {
            break;
        }
        let list = e.call(PLUS_4, &args![REFERENCE_LIST_OWNER]).u32();
        let reference = e.call(LIST_GET, &args![list, index]).u32();
        if reference != 0
            && e.vcall(reference, VSLOT_REFERENCE_224, &args![]).bool()
            && e.vcall(reference, VSLOT_REFERENCE_318, &args![]).bool()
        {
            let player = e.global::<u32>(PLAYER);
            e.vcall(reference, VSLOT_REFERENCE_320, &args![player, 1u32]);
        }
        index = index.wrapping_add(1);
    }
    true
}

// Translated from 005de1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer; a value of 0 to 65535 is stored by [`fn_005de240`] and
/// the command succeeds, any other value fails.
pub fn fn_005de1d0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !(0..=0xffff).contains(&(value as i32)) {
        return false;
    }
    fn_005de240(e, value as u16);
    true
}

// Translated from 005de240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the `u16` at `011ae75c`.
pub fn fn_005de240(e: &mut Engine, value: u16) {
    e.set_global(COMMAND_VALUE_011AE75C, value);
}

// Translated from 005de250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer; a value of 0 to 65535 is stored by [`fn_005de2c0`] and
/// the command succeeds, any other value fails.
pub fn fn_005de250(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !(0..=0xffff).contains(&(value as i32)) {
        return false;
    }
    fn_005de2c0(e, value as u16);
    true
}

// Translated from 005de2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the `u16` at `011afa9c`.
pub fn fn_005de2c0(e: &mut Engine, value: u16) {
    e.set_global(COMMAND_VALUE_011AFA9C, value);
}

// Translated from 005de2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetLastExtDoorActivated` (Xbox PDB): parses a reference and
/// stores it at the player's `+0x604`. A non-null reference with a world
/// space makes that world space the root map space. Succeeds when the
/// parameter parsed.
pub fn script_set_last_ext_door_activated(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([reference]) = parse_params(e, a, [0]) else {
        return false;
    };
    let player = e.global::<u32>(PLAYER);
    e.mem.set_u32(player + PLAYER_LAST_EXT_DOOR, reference);
    if reference != 0 && e.call(GET_WORLD_SPACE, &args![reference]).u32() != 0 {
        let world_space = e.call(GET_WORLD_SPACE, &args![reference]).u32();
        e.call(SET_ROOT_MAP_SPACE, &args![world_space]);
    }
    true
}

// Translated from 005de350 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command without parameters: when the pointer at `011d96c0` is set, calls
/// [`fn_005de380`] with it. Always succeeds.
pub fn fn_005de350(e: &mut Engine, _a: ScriptArgs) -> bool {
    let object = e.global::<u32>(OBJECT_POINTER_011D96C0);
    if object != 0 {
        fn_005de380(e, Ptr::new(object));
    }
    true
}

// Translated from 005de380 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the pointer at `011d96c0` is set, sets the byte at `+0x254` of its
/// object to 1 (the word the caller passes as `this` is not used).
pub fn fn_005de380(e: &mut Engine, _unused_this: Ptr) {
    let object = e.global::<u32>(OBJECT_POINTER_011D96C0);
    if object != 0 {
        e.mem.set_u8(object + 0x254, 1);
    }
}

// Translated from 005de3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetTaskThreadSleepFunction` (Xbox PDB): parses an integer, stores
/// it with [`fn_005de410`] and prints `"BSTaskThread::sPauseSleepValue set to
/// <value> ms"`. Succeeds when the parameter parsed.
pub fn script_set_task_thread_sleep_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    fn_005de410(e, value);
    e.call(CONSOLE_PRINT, &args![MSG_PAUSE_SLEEP, value]);
    true
}

// Translated from 005de410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word at `011af708` (`BSTaskThread::sPauseSleepValue`).
pub fn fn_005de410(e: &mut Engine, value: u32) {
    e.set_global(TASK_THREAD_PAUSE_SLEEP, value);
}

// Translated from 005de420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetTaskThreadUpdateSleepFunction` (Xbox PDB): parses an integer,
/// stores it with [`fn_005de490`] and prints `"BSTaskThread::sUpdateSleepValue
/// set to <value> ms"`. Succeeds when the parameter parsed.
pub fn script_set_task_thread_update_sleep_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    fn_005de490(e, value);
    e.call(CONSOLE_PRINT, &args![MSG_UPDATE_SLEEP, value]);
    true
}

// Translated from 005de490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word at `011af710` (`BSTaskThread::sUpdateSleepValue`).
pub fn fn_005de490(e: &mut Engine, value: u32) {
    e.set_global(TASK_THREAD_UPDATE_SLEEP, value);
}

// Translated from 005de4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsImageSpaceActiveFunction` (Xbox PDB): sets the result to 0.0,
/// parses an integer (and a `float` that is not used). Walks the list headed
/// at `+0x6c` of the object that `011dea10` points to; for every entry it
/// takes a counted copy of the pointer in it, and when that object's virtual
/// slot `0x94` gives something whose `+0x1c` field equals the integer, the
/// result becomes 1.0 and the walk ends. Prints `"ImageSpace is active "` or
/// `"ImageSpace is not active"` (by the result). Succeeds when the parameters
/// parsed. (The structured exception frame is not translated.)
pub fn script_is_image_space_active_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([wanted, _unused_float]) = parse_params(e, a, [0, 1.0f32.to_bits()]) else {
        return false;
    };
    let owner = e.global::<u32>(LIST_OWNER_011DEA10);
    let mut node = e.call(LIST_HEAD_AT_6C, &args![owner]).u32();
    while node != 0 {
        let item = e.call(LIST_ITEM_PTR, &args![node]).u32();
        let found = e.with_stack(4, |e, copy| {
            e.call(NI_POINTER_COPY_CONSTRUCT, &args![copy, item]);
            let object = e.call(READ_POINTER, &args![copy]).u32();
            let mut found = false;
            if object != 0 {
                let entry = e.vcall(object, VSLOT_ENTRY_94, &args![]).u32();
                found = entry != 0 && e.call(FIELD_1C, &args![entry]).u32() == wanted;
            }
            if found {
                e.mem.set_f64(a.result.addr(), 1.0);
            } else {
                node = e.call(LIST_NEXT, &args![node]).u32();
            }
            e.call(NI_POINTER_DESTRUCT, &args![copy]);
            found
        });
        if found {
            break;
        }
    }
    let text = if e.mem.f64(a.result.addr()) == 0.0 {
        MSG_IMAGE_SPACE_NOT_ACTIVE
    } else {
        MSG_IMAGE_SPACE_ACTIVE
    };
    e.call(CONSOLE_PRINT, &args![text]);
    true
}

// Translated from 005de600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the result to 0.0 and parses two references; when both are given,
/// calls `00590010` on the first with the second. Succeeds when the
/// parameters parsed.
pub fn fn_005de600(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([first, second]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if second != 0 && first != 0 {
        e.call(PAIR_ACTION_00590010, &args![first, second]);
    }
    true
}

// Translated from 005de670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the result to 0.0 and parses a container, an item and a number; when
/// the container and the item are given, calls `00487a00` on the container's
/// `+0x30` with the number (low 16 bits), 1 and no extra data.
/// Succeeds when the parameters parsed.
pub fn fn_005de670(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([container, item, number]) = parse_params(e, a, [0, 0, 0]) else {
        return false;
    };
    if container != 0 && item != 0 {
        e.call(
            ADD_ITEM_00487A00,
            &args![
                container + 0x30,
                container,
                number & 0xffff,
                1u32,
                item,
                0u32
            ],
        );
    }
    true
}

// Translated from 005de700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::AddItemToLeveledListFunction` (Xbox PDB): sets the result to 0.0
/// and parses a list, an item, two numbers and a percentage (default
/// 100.0). When the list, the item and the second number are given, builds a
/// `ContainerItemExtra` holding the percentage divided by 100 (rounded to
/// `float`) and calls `00487a00` on the list's `+0x30` with the numbers
/// (low 16 bits each). Succeeds when the parameters parsed. (The
/// exception frame around the allocation is not translated.)
pub fn script_add_item_to_leveled_list_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let percent = e.global::<u32>(DEFAULT_PERCENT);
    let Some([list, item, first_number, second_number, percent]) =
        parse_params(e, a, [0, 0, 0, 0, percent])
    else {
        return false;
    };
    if list != 0 && item != 0 && second_number != 0 {
        let weight = (f32::from_bits(percent) as f64 / e.global::<f64>(HUNDRED)) as f32;
        let memory = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        let extra = if memory != 0 {
            e.call(CONTAINER_ITEM_EXTRA_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        e.call(CONTAINER_ITEM_EXTRA_SET_FLOAT, &args![extra, weight]);
        e.call(
            ADD_ITEM_00487A00,
            &args![
                list + 0x30,
                list,
                first_number & 0xffff,
                second_number & 0xffff,
                item,
                extra
            ],
        );
    }
    true
}

// Translated from 005de840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::RemapWaterTypeFunction` (Xbox PDB): sets the result to 0.0 and
/// parses the water type to be remapped and the one to remap to. A missing
/// one, or one whose form type is not `0x4e`, is reported through the logging
/// stub and dropped; when both remain, `00580680` is called on the first with
/// the second. Succeeds when the parameters parsed.
pub fn script_remap_water_type_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([mut to_remap, mut remap_to]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if to_remap == 0 || e.call(FORM_TYPE, &args![to_remap]).u32() != FORM_TYPE_WATER {
        e.call(LOG_STUB, &args![MSG_WATER_TO_REMAP_INVALID]);
        to_remap = 0;
    }
    if remap_to == 0 || e.call(FORM_TYPE, &args![remap_to]).u32() != FORM_TYPE_WATER {
        e.call(LOG_STUB, &args![MSG_WATER_REMAP_TO_INVALID]);
        remap_to = 0;
    }
    if to_remap != 0 && remap_to != 0 {
        e.call(WATER_REMAP_00580680, &args![to_remap, remap_to]);
    }
    true
}

// Translated from 005de900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `IsHardcoreConditionalFunction(thisObj, 0, 0, result)` (its `AL`
/// is the command's).
pub fn fn_005de900(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_HARDCORE_CONDITION, a, 0)
}

// Translated from 005de920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetHardcoreFunction` (Xbox PDB): parses an integer. When the
/// player exists and the value is 0 or 1, calls `PlayerCharacter::SetHardcore`
/// with it and, when it is not 1, clears the player's byte at `+0xe38`
/// ([`fn_005de9b0`]) and succeeds; otherwise reports through the logging stub
/// and fails. Fails too when the parameter did not parse.
pub fn script_set_hardcore_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    let value = value as i32;
    let player = e.global::<u32>(PLAYER);
    if player == 0 || !(0..=1).contains(&value) {
        e.call(LOG_STUB, &args![MSG_SET_HARDCORE_INVALID]);
        return false;
    }
    e.call(PLAYER_SET_HARDCORE, &args![player, value, 0u32]);
    if value != 1 {
        fn_005de9b0(e, Ptr::new(player));
    }
    true
}

// Translated from 005de9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the byte at `this + 0xe38` (`PlayerCharacter`, PC offset).
pub fn fn_005de9b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + PLAYER_BYTE_E38, 0);
}

// Translated from 005de9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the player's byte at `+0xe38` when the player exists. Always
/// succeeds.
pub fn fn_005de9d0(e: &mut Engine, _a: ScriptArgs) -> bool {
    let player = e.global::<u32>(PLAYER);
    if player != 0 {
        fn_005de9b0(e, Ptr::new(player));
    }
    true
}

// Translated from 005de9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ResetXPFunction` (Xbox PDB): parses nothing it uses (the result of
/// `ParseParameters` is ignored). Without a player it reports through the
/// logging stub and fails. Otherwise it reads the value `0x18` from the
/// sub-object at the player's `+0xa4` (virtual slot `0xc`, a `float`) and
/// calls the player's virtual slot `0x3a4` with `0x18`, the negated value
/// (rounded to `float`) and 0: the value is taken back to zero. Succeeds.
pub fn script_reset_xp_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parse(e, a, &[]);
    let player = e.global::<u32>(PLAYER);
    if player == 0 {
        e.call(LOG_STUB, &args![MSG_RESET_XP_INVALID]);
        return false;
    }
    let value = e
        .vcall(
            player + PLAYER_SUBOBJECT_A4,
            VSLOT_SUBOBJECT_VALUE,
            &args![RESET_XP_INDEX],
        )
        .f32();
    e.vcall(
        player,
        VSLOT_PLAYER_3A4,
        &args![RESET_XP_INDEX, -value, 0u32],
    );
    true
}

// Translated from 005dea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `IsAlwaysHardcoreConditionalFunction(thisObj, 0, 0, result)` (its
/// `AL` is the command's).
pub fn fn_005dea80(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_ALWAYS_HARDCORE_CONDITION, a, 0)
}

// Translated from 005deaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the result to 0.0. When `thisObj` is an actor (virtual slot `0x100`)
/// whose record (`008a51a0`) has a form at `+4`, puts that form's id into the
/// result double (`PutNumericIDInDouble`). Always succeeds.
pub fn fn_005deaa0(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    if !a.this_obj.is_null() && e.vcall(a.this_obj.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        let record = e.call(ACTOR_RECORD, &args![a.this_obj]).u32();
        if record != 0 {
            let form = e.mem.u32(record + 4);
            if form != 0 {
                let id = e.call(FORM_ID, &args![form]).u32();
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), id);
                    e.call(PUT_NUMERIC_ID_IN_DOUBLE, &args![slot, a.result]);
                });
            }
        }
    }
    true
}

// Translated from 005deb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer (the result of `ParseParameters` is ignored). With a
/// `thisObj`, picks the reference to act on: when virtual slot `0xfc` is set
/// it is a candidate itself; otherwise the base form is cast from
/// `TESBoundObject` to `BGSTalkingActivator` and, when that works, its field
/// at `+0x90` is the reference. A candidate that is an actor (slot `0x100`)
/// is the reference; one with the byte `+0x81` set gives the field at `+0x90`
/// of the base form of the object at its `+0x6c`. When a reference was found,
/// calls `00704fc0(reference, value)`. Always succeeds.
pub fn fn_005deb10(e: &mut Engine, a: ScriptArgs) -> bool {
    let (_, [value]) = parse_locals(e, a, [0]);
    if a.this_obj.is_null() {
        return true;
    }
    let this_obj = a.this_obj.addr();
    let mut candidate = 0u32;
    let mut reference = 0u32;
    if e.vcall(this_obj, VSLOT_FLAG_FC, &args![]).bool() {
        candidate = this_obj;
    } else {
        let base = e.call(GET_BASE_FORM, &args![this_obj]).u32();
        let cast = e
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
        reference = if cast != 0 {
            e.call(FIELD_90, &args![cast]).u32()
        } else {
            0
        };
    }
    if candidate != 0 {
        if e.vcall(candidate, VSLOT_IS_ACTOR, &args![]).bool() {
            reference = candidate;
        } else if e.call(BYTE_AT_81, &args![candidate]).bool() {
            let inner = e.call(FIELD_6C, &args![candidate]).u32();
            let base = e.call(GET_BASE_FORM, &args![inner]).u32();
            reference = e.call(FIELD_90, &args![base]).u32();
        }
    }
    if reference != 0 {
        e.call(SHOW_MENU_FOR_00704FC0, &args![reference, value]);
    }
    true
}

/// The loop of [`fn_005dec30`] and [`fn_005ded40`]: walks the player's list
/// (`+0x610`, ended by a node whose item cell is null) and calls
/// `visit(node)` for every entry whose first word is the id of `target`.
fn for_each_player_entry(e: &mut Engine, target: u32, mut visit: impl FnMut(&mut Engine, u32)) {
    let player = e.global::<u32>(PLAYER);
    let mut node = e.call(PLAYER_LIST_HEAD, &args![player]).u32();
    while node != 0 {
        let cell = e.call(LIST_ITEM_PTR, &args![node]).u32();
        if e.mem.u32(cell) == 0 {
            break;
        }
        let cell = e.call(LIST_ITEM_PTR, &args![node]).u32();
        let entry = e.mem.u32(cell);
        let form_id = e.call(FORM_ID, &args![target]).u32();
        if e.mem.u32(entry) == form_id {
            visit(e, node);
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
}

// Translated from 005dec30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a form (the result of `ParseParameters` is ignored; the local is
/// uninitialised in the game, 0 here). Fails when it is null. Otherwise
/// walks the player's list (`+0x610`, ended by a node whose item cell is
/// null) and for the entry whose first word is the form's id sets the result
/// to the number of quarters (1 to 4) of the limit (`+0x28` of the block at
/// the form's `+0x1e8`) that the entry's value (`+4`) reaches, or 0.0 when it
/// reaches none. Succeeds.
pub fn fn_005dec30(e: &mut Engine, a: ScriptArgs) -> bool {
    let (_, [target]) = parse_locals(e, a, [0]);
    if target == 0 {
        return false;
    }
    for_each_player_entry(e, target, |e, node| {
        let block = e.call(FORM_BLOCK_1E8, &args![target]).u32();
        let limit = e.mem.i32(block + BLOCK_LIMIT);
        let cell = e.call(LIST_ITEM_PTR, &args![node]).u32();
        let value = e.mem.i32(e.mem.u32(cell) + 4);
        e.mem.set_f64(a.result.addr(), 0.0);
        for level in 0..4 {
            let quarter = e.global::<f64>(QUARTER);
            let threshold = limit as f64 * ((level + 1) as f64 * quarter);
            if threshold <= value as f64 {
                e.mem.set_f64(a.result.addr(), (level + 1) as f64);
            }
        }
    });
    true
}

// Translated from 005ded40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a form and an integer (the result of `ParseParameters` is ignored;
/// the form's local is uninitialised in the game, 0 here). Fails when the form
/// is null. Otherwise walks the player's list as [`fn_005dec30`] does and, for
/// the entry whose first word is the form's id, when the integer is 0 to 4
/// (signed), stores it as the entry's `u16` at `+8` and the value
/// `integer * 0.25 * limit` (truncated; the limit is `+0x28` of the block at
/// the form's `+0x1e8`) as the entry's value at `+4`. Succeeds.
pub fn fn_005ded40(e: &mut Engine, a: ScriptArgs) -> bool {
    let (_, [target, level]) = parse_locals(e, a, [0, 0]);
    if target == 0 {
        return false;
    }
    let level = level as i32;
    for_each_player_entry(e, target, |e, node| {
        if !(0..=4).contains(&level) {
            return;
        }
        let block = e.call(FORM_BLOCK_1E8, &args![target]).u32();
        let limit = e.mem.i32(block + BLOCK_LIMIT);
        let cell = e.call(LIST_ITEM_PTR, &args![node]).u32();
        let entry = e.mem.u32(cell);
        e.mem.set_u16(entry + 8, level as u16);
        let quarter = e.global::<f64>(QUARTER);
        let scaled = (level as f64 * quarter) * limit as f64;
        let value = e.call(FTOL, &args![scaled]).u32();
        let cell = e.call(LIST_ITEM_PTR, &args![node]).u32();
        let entry = e.mem.u32(cell);
        e.mem.set_u32(entry + 4, value);
    });
    true
}

// Translated from 005dee30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one object; when it is not null calls `005f64f0(object)` and
/// succeeds, otherwise fails.
pub fn fn_005dee30(e: &mut Engine, a: ScriptArgs) -> bool {
    let (_, [object]) = parse_locals(e, a, [0]);
    if object == 0 {
        return false;
    }
    e.call(OBJECT_ACTION_005F64F0, &args![object]);
    true
}

// Translated from 005dee90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one object; when it is not null sets the result to its flag bit 1
/// (`(flags & 1)` as a number) and succeeds, otherwise fails (the result is
/// left alone).
pub fn fn_005dee90(e: &mut Engine, a: ScriptArgs) -> bool {
    let (_, [object]) = parse_locals(e, a, [0]);
    if object == 0 {
        return false;
    }
    let bit = e.call(FLAG_BIT_1, &args![object]).i32();
    e.mem.set_f64(a.result.addr(), bit as f64);
    true
}

// Translated from 005deef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one object; when it is not null sets the result to 1.0 when its
/// flag bit 2 or its flag bit 4 ([`fn_005def70`]) is set and to 0.0
/// otherwise, and succeeds; fails otherwise.
pub fn fn_005deef0(e: &mut Engine, a: ScriptArgs) -> bool {
    let (_, [object]) = parse_locals(e, a, [0]);
    if object == 0 {
        return false;
    }
    let set =
        e.call(FLAG_BIT_2, &args![object]).u32() != 0 || fn_005def70(e, Ptr::new(object)) != 0;
    e.mem.set_f64(a.result.addr(), set as i32 as f64);
    true
}

// Translated from 005def70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this->flags & 4`, the word at `+0x70`.
pub fn fn_005def70(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + OBJECT_FLAGS) & 4
}

// Translated from 005def90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one object; when it is not null calls [`fn_005defe0`] on it with 1
/// and succeeds, otherwise fails.
pub fn fn_005def90(e: &mut Engine, a: ScriptArgs) -> bool {
    let (_, [object]) = parse_locals(e, a, [0]);
    if object == 0 {
        return false;
    }
    fn_005defe0(e, Ptr::new(object), 1);
    true
}

// Translated from 005defe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` not 0) or clears bit 8 of the word at `this + 0x70`, then
/// calls `005f5800` on `this` (it calls the object's virtual slot `0x48` with
/// 2).
pub fn fn_005defe0(e: &mut Engine, this: Ptr, flag: u8) {
    let flags = e.mem.u32(this.addr() + OBJECT_FLAGS);
    let flags = if flag == 0 { flags & !8 } else { flags | 8 };
    e.mem.set_u32(this.addr() + OBJECT_FLAGS, flags);
    e.call(OBJECT_FLAGS_CHANGED, &args![this]);
}

// Translated from 005df510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three floats at `source` to element `index` of the array of
/// 12-byte elements at `this`.
pub fn fn_005df510(e: &mut Engine, this: Ptr, index: i32, source: Ptr) {
    let element = this.addr().wrapping_add((index as u32).wrapping_mul(12));
    for lane in 0..3 {
        let value = e.mem.f32(source.addr() + 4 * lane);
        e.mem.set_f32(element + 4 * lane, value);
    }
}

// Translated from 005df560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three floats of element `index` of the array of 12-byte
/// elements at `this` to the three output addresses.
pub fn fn_005df560(e: &mut Engine, this: Ptr, index: i32, x: Ptr, y: Ptr, z: Ptr) {
    let element = this.addr().wrapping_add((index as u32).wrapping_mul(12));
    for (lane, out) in [x, y, z].into_iter().enumerate() {
        let value = e.mem.f32(element + 4 * lane as u32);
        e.mem.set_f32(out.addr(), value);
    }
}

// Translated from 005df5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0xde0`.
pub fn fn_005df5b0(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0xde0))
}

// Translated from 005df930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDLCCountFunction` (Xbox PDB): parses nothing (one null output
/// word). When that fails, sets the result to 0.0 and fails. Otherwise takes
/// the DLC system utility's list of packages (`+0xc`), sets the result to
/// the number of its nodes with an item (0.0 for no list), prints `"There are
/// <n> DLC packages available"` (the result truncated to an integer) and
/// succeeds. The security-cookie check is not translated.
pub fn script_get_dlc_count_function(e: &mut Engine, a: ScriptArgs) -> bool {
    if !parse(e, a, &[0]) {
        e.mem.set_f64(a.result.addr(), 0.0);
        return false;
    }
    let utility = e.call(SYSTEM_UTILITY_INSTANCE, &args![]).u32();
    let dlc = fn_005dfa10(e, Ptr::new(utility));
    let list = e.call(DLC_LIST_HEAD, &args![dlc]).u32();
    let count = if list != 0 {
        e.call(LIST_COUNT_NON_NULL, &args![list]).u32() as f64
    } else {
        0.0
    };
    e.mem.set_f64(a.result.addr(), count);
    // `FISTP` with truncation: the low word is what `%u` prints.
    let printed = (e.mem.f64(a.result.addr()) as i64) as u32;
    e.with_stack(0x40, |e, text| {
        e.call(SPRINTF_S, &args![text, 0x40u32, MSG_DLC_COUNT, printed]);
        e.call(CONSOLE_PRINT, &args![text]);
    });
    true
}

// Translated from 005dfa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `BSDLCSystemUtility::QInstance` with `this` in `ECX` and returns its
/// result.
pub fn fn_005dfa10(e: &mut Engine, this: Ptr) -> u32 {
    e.call(DLC_SYSTEM_UTILITY_INSTANCE, &args![this]).u32()
}

// Translated from 005dfa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsDLCInstalledFunction` (Xbox PDB): parses a package name (up to
/// 512 bytes). When that fails, sets the result to 0.0 and fails. Otherwise
/// formats `"Checking for DLC package '<name>'"` into a 200-byte buffer, then
/// walks the DLC list (`+0xc` of the DLC system utility): for every node with
/// an item it formats `"     Examining '<wide name>' [<tag>]"` and converts the
/// item's wide name into the same buffer; when that equals the parsed name
/// (case-insensitively) the result becomes 1.0 and the command succeeds
/// at once. Without a match the result is 0.0 and it succeeds. (The
/// security-cookie check is not translated; the formatted lines are never
/// printed.)
pub fn script_is_dlc_installed_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(0x200, |e, name| {
        if !parse(e, a, &[name.addr()]) {
            e.mem.set_f64(a.result.addr(), 0.0);
            return false;
        }
        let utility = e.call(SYSTEM_UTILITY_INSTANCE, &args![]).u32();
        let dlc = fn_005dfa10(e, Ptr::new(utility));
        let mut node = e.call(DLC_LIST_HEAD, &args![dlc]).u32();
        e.with_stack(200, |e, text| {
            e.call(SPRINTF_S, &args![text, 200u32, MSG_CHECKING_DLC, name]);
            while node != 0 {
                let cell = e.call(LIST_ITEM_PTR, &args![node]).u32();
                let package = e.mem.u32(cell);
                if package != 0 {
                    let tag = e.call(DLC_PACKAGE_TAG, &args![package]).u32();
                    let wide_name = e.call(DLC_PACKAGE_NAME, &args![package]).u32();
                    e.call(
                        SPRINTF_S,
                        &args![text, 200u32, MSG_EXAMINING_DLC, wide_name, tag],
                    );
                    e.with_stack(4, |e, converted| {
                        e.mem.set_u32(converted.addr(), 0);
                        let source = e.call(DLC_PACKAGE_NAME, &args![package]).u32();
                        e.call(WCSTOMBS_S, &args![converted, text, 200u32, source, 200u32]);
                    });
                    if e.call(STRICMP, &args![text, name]).u32() == 0 {
                        e.mem.set_f64(a.result.addr(), 1.0);
                        return true;
                    }
                }
                node = e.call(LIST_NEXT, &args![node]).u32();
            }
            e.mem.set_f64(a.result.addr(), 0.0);
            true
        })
    })
}

// Translated from 005dfba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer. When that fails, sets the result to 0.0 and fails.
/// Otherwise calls `00457d70(1, 0, 1)` (value above 0, signed) or
/// `00457d70(0, 0, 1)` on the object at `011dea10` and succeeds.
pub fn fn_005dfba0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        e.mem.set_f64(a.result.addr(), 0.0);
        return false;
    };
    let owner = e.global::<u32>(LIST_OWNER_011DEA10);
    let flag = (value as i32 > 0) as u32;
    e.call(SET_FLAG_00457D70, &args![owner, flag, 0u32, 1u32]);
    true
}

// Translated from 005dfc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsLoadingMenuEnabled` (Xbox PDB): sets the result to 1.0 when
/// `Interface::IsInGameLoadingMenuOpen` says so, else 0.0, and prints
/// `"IsLoadingMenuEnabled >> <result>"` when the TLS echo flag is set. Always
/// succeeds.
pub fn script_is_loading_menu_enabled(e: &mut Engine, a: ScriptArgs) -> bool {
    let open = e.call(IS_IN_GAME_LOADING_MENU_OPEN, &args![]).bool();
    e.mem.set_f64(a.result.addr(), open as i32 as f64);
    if echo_enabled(e) {
        let value = e.mem.f64(a.result.addr());
        e.call(CONSOLE_PRINT, &args![MSG_LOADING_MENU_ENABLED, value]);
    }
    true
}

// Translated from 005dfc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsActorRefEssential` (Xbox PDB): sets the result to 0.0. When
/// `thisObj` (not checked for null) is an actor (virtual slot `0x100`), casts
/// it from `TESObjectREFR` to `Actor`, sets the result to whether that actor
/// is essential (`008c1b30`) and, with the TLS echo flag, prints
/// `"IsActorRefEssential >> <result>"`; otherwise, with the echo flag, prints
/// `"IsActorRefEssential >> Invalid actor reference "`. Always succeeds.
pub fn script_is_actor_ref_essential(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    if e.vcall(a.this_obj.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        let actor = e
            .call(
                DYNAMIC_CAST,
                &args![a.this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
            )
            .u32();
        let essential = e.call(ACTOR_IS_ESSENTIAL, &args![actor]).bool();
        e.mem.set_f64(a.result.addr(), essential as i32 as f64);
        if echo_enabled(e) {
            let value = e.mem.f64(a.result.addr());
            e.call(CONSOLE_PRINT, &args![MSG_ESSENTIAL_VALUE, value]);
        }
    } else if echo_enabled(e) {
        e.call(CONSOLE_PRINT, &args![MSG_ESSENTIAL_INVALID]);
    }
    true
}

// Translated from 005dfd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetActorRefEssential` (Xbox PDB): parses an integer; fails when
/// that fails. When `thisObj` (not checked for null) is an actor (virtual
/// slot `0x100`), casts it from `TESObjectREFR` to `Actor` and calls
/// `008c1ac0(actor, value > 0)`; otherwise, with the TLS echo flag, prints
/// `"SetActorRefEssential >> Invalid actor reference "`. Succeeds.
pub fn script_set_actor_ref_essential(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    if e.vcall(a.this_obj.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        let actor = e
            .call(
                DYNAMIC_CAST,
                &args![a.this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
            )
            .u32();
        let essential = (value as i32 > 0) as u32;
        e.call(ACTOR_SET_ESSENTIAL, &args![actor, essential]);
    } else if echo_enabled(e) {
        e.call(CONSOLE_PRINT, &args![MSG_SET_ESSENTIAL_INVALID]);
    }
    true
}

// Translated from 005dfde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsNoHumanGoreExe` (Xbox PDB): sets the result to 0.0 and, with
/// the TLS echo flag, prints `"IsNoHumanGoreExe >> 0"`. Always succeeds.
pub fn script_is_no_human_gore_exe(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    if echo_enabled(e) {
        let value = e.mem.f64(a.result.addr());
        e.call(CONSOLE_PRINT, &args![MSG_NO_HUMAN_GORE, value]);
    }
    true
}

// Translated from 005dfe30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer; when that works calls `005f6110(value, 1)` and
/// succeeds, otherwise fails.
pub fn fn_005dfe30(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    e.call(SHARED_ACTION_005F6110, &args![value, 1u32]);
    true
}

// Translated from 005dfe90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PersistentRefCountFunction` (Xbox PDB): without the table at
/// `011c54c0` it does nothing (the result is left alone). Otherwise it walks
/// the table (first node, then `006b7f20` for key and value) and counts the
/// values that are not null, whose virtual slot `0xf0` is set and whose word
/// at `+8` has bit `0x400`. The result is the count; with the TLS echo flag
/// it prints `"PersistentRefCount >> <count>"`. Always succeeds.
pub fn script_persistent_ref_count_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let table = e.global::<u32>(HASH_TABLE_011C54C0);
    if table == 0 {
        return true;
    }
    let mut count = 0u32;
    e.with_stack(12, |e, locals| {
        let (node, key, value) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        let first = e.call(HASH_TABLE_FIRST, &args![table]).u32();
        e.mem.set_u32(node, first);
        e.mem.set_f64(a.result.addr(), 0.0);
        while e.mem.u32(node) != 0 {
            e.call(HASH_TABLE_NEXT, &args![table, node, key, value]);
            let item = e.mem.u32(value);
            if item != 0
                && e.vcall(item, VSLOT_FLAG_F0, &args![]).bool()
                && e.call(FIELD_8, &args![item]).u32() & PERSISTENT_BIT != 0
            {
                count += 1;
            }
        }
    });
    e.mem.set_f64(a.result.addr(), count as f64);
    if echo_enabled(e) {
        e.call(CONSOLE_PRINT, &args![MSG_PERSISTENT_REF_COUNT, count]);
    }
    true
}

// Translated from 005dff70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::AlwaysShowActorSubtitles` (Xbox PDB): parses an integer; fails
/// when that fails. When `thisObj` (not checked for null) is an actor
/// (virtual slot `0x100`), casts it from `TESObjectREFR` to `Actor` and calls
/// `008c1b50(actor, value > 0)`; otherwise, with the TLS echo flag, prints
/// `"AlwaysShowActorSubtitles >> Invalid actor reference "`. Succeeds.
pub fn script_always_show_actor_subtitles(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    if e.vcall(a.this_obj.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        let actor = e
            .call(
                DYNAMIC_CAST,
                &args![a.this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
            )
            .u32();
        let flag = (value as i32 > 0) as u32;
        e.call(ACTOR_SET_ALWAYS_SHOW_SUBTITLES, &args![actor, flag]);
    } else if echo_enabled(e) {
        e.call(CONSOLE_PRINT, &args![MSG_SUBTITLES_INVALID]);
    }
    true
}

// Translated from 005e0020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer; fails when that fails. Stores 1 ([`fn_005e0080`]) when
/// the integer is 1, 0 for any other. Succeeds.
pub fn fn_005e0020(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    fn_005e0080(e, (value == 1) as u8);
    true
}

// Translated from 005e0080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `011dd437`.
pub fn fn_005e0080(e: &mut Engine, value: u8) {
    e.set_global(STORED_FLAG_011DD437, value);
}

// Translated from 005e0090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowSleepWaitMenu` (Xbox PDB): parses two integers (the first
/// stays uninitialised in the game when it is not given, 0 here); fails when
/// that fails. When the second is 1 calls `00969fa0` on the player; otherwise
/// `Interface::CreateSleepMenu(first == 1)`. Succeeds.
pub fn script_show_sleep_wait_menu(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, second]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if second == 1 {
        let player = e.global::<u32>(PLAYER);
        e.call(PLAYER_SLEEP_WAIT, &args![player]);
    } else {
        e.call(CREATE_SLEEP_MENU, &args![(first == 1) as u32]);
    }
    true
}

// Translated from 005e0100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetContainerInventoryCount` (Xbox PDB): with a `thisObj`, sets
/// the result to `00575590(thisObj, 0, 1)` and, with the TLS echo flag, prints
/// `"GetContainerInventoryCount >> <count>"` (the result truncated). Without
/// one it does nothing. Always succeeds.
pub fn script_get_container_inventory_count(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return true;
    }
    let count = e
        .call(INVENTORY_COUNT, &args![a.this_obj, 0u32, 1u32])
        .i32();
    e.mem.set_f64(a.result.addr(), count as f64);
    if echo_enabled(e) {
        let value = e.mem.f64(a.result.addr());
        let truncated = e.call(FTOL, &args![value]).u32();
        e.call(CONSOLE_PRINT, &args![MSG_CONTAINER_COUNT, truncated]);
    }
    true
}

// Translated from 005e0160 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the flag byte of the object at `011de29c` (`00408d60` gives its
/// address) is set, calls `00851d70` on the object `011de134` points to (it
/// sets the byte at its `+0x11`). Always succeeds.
pub fn fn_005e0160(e: &mut Engine, _a: ScriptArgs) -> bool {
    let flag_address = e.call(FLAG_ADDRESS, &args![OBJECT_011DE29C]).u32();
    if e.mem.u8(flag_address) != 0 {
        let target = e.global::<u32>(POINTER_011DE134);
        e.call(SET_BYTE_11, &args![target]);
    }
    true
}

// Translated from 005e0190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00c33170(this, text)` and returns `this`.
pub fn fn_005e0190(e: &mut Engine, this: Ptr, text: u32) -> Ptr {
    e.call(MESSAGE_SET_TEXT, &args![this, text]);
    this
}

// Translated from 005e01b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `005e0830(this, arg, 1)`; when that succeeds calls the object's
/// virtual slot `0x20` (its `AL` becomes the result) and then slot `0x1c`.
/// Returns the result.
pub fn fn_005e01b0(e: &mut Engine, this: Ptr, arg: u32) -> bool {
    let mut result = e.call(SETTING_STEP, &args![this, arg, 1u32]).bool();
    if result {
        result = e.vcall(this.addr(), VSLOT_SETTING_20, &args![]).bool();
        e.vcall(this.addr(), VSLOT_SETTING_1C, &args![]);
    }
    result
}

// Translated from 005e0200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_005e01b0`] with a scope guard (`Setting.h`, line `0x19d`) around
/// the work: `005e0830(this, arg, 0)`; when that succeeds calls virtual slot
/// `0x24` and then `0x1c` (the result of `005e0830` is returned, not the
/// slot's). (The exception-unwinding frame is not translated.)
pub fn fn_005e0200(e: &mut Engine, this: Ptr, arg: u32) -> bool {
    e.with_stack(4, |e, guard| {
        e.call(
            GUARD_CONSTRUCT,
            &args![guard, 9u32, 1u32, SETTING_H_FILE, 0x19du32],
        );
        let result = e.call(SETTING_STEP, &args![this, arg, 0u32]).bool();
        if result {
            e.vcall(this.addr(), VSLOT_SETTING_24, &args![]);
            e.vcall(this.addr(), VSLOT_SETTING_1C, &args![]);
        }
        e.call(GUARD_DESTRUCT, &args![guard]);
        result
    })
}

// Translated from 005e02b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `005e0860(this, arg, &found)` with `found` starting at 0 and returns
/// `found`.
pub fn fn_005e02b0(e: &mut Engine, this: Ptr, arg: u32) -> u32 {
    e.with_stack(4, |e, found| {
        e.call(LOOKUP_005E0860, &args![this, arg, found]);
        e.mem.u32(found.addr())
    })
}

// Translated from 005e02e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0103d3e8` in `this` and destroys the pointer at
/// `this + 4` (`004b0220`). (The code is a library match for
/// `~basic_streambuf`; the body is not that.)
pub fn fn_005e02e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_0103D3E8);
    let pointer = e.mem.u32(this.addr() + 4);
    e.call(POINTER_DESTROY, &args![pointer]);
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005db810, script_get_vats_front_target_visible_function(ScriptArgs) -> bool),
        entry!(0x005dbe10, fn_005dbe10() -> u32),
        entry!(0x005dbe40, fn_005dbe40(Ptr, u32) -> bool),
        entry!(0x005dbe60, fn_005dbe60(Ptr, Ptr)),
        entry!(0x005dbec0, fn_005dbec0(Ptr, Ptr)),
        entry!(0x005dbf20, fn_005dbf20(Ptr, f32) -> Ptr),
        entry!(0x005dbf70, fn_005dbf70(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x005dc010, fn_005dc010(ScriptArgs) -> bool),
        entry!(0x005dc090, script_is_in_crit_stage_function(ScriptArgs) -> bool),
        entry!(0x005dc0f0, fn_005dc0f0(ScriptArgs) -> bool),
        entry!(0x005dc1b0, fn_005dc1b0(ScriptArgs) -> bool),
        entry!(0x005dc270, fn_005dc270(f32) -> f32),
        entry!(0x005dc290, fn_005dc290(ScriptArgs) -> bool),
        entry!(0x005dc330, fn_005dc330(f32) -> f32),
        entry!(0x005dc350, fn_005dc350(ScriptArgs) -> bool),
        entry!(0x005dc3b0, fn_005dc3b0(ScriptArgs) -> bool),
        entry!(0x005dc440, fn_005dc440(f32) -> f32),
        entry!(0x005dc460, fn_005dc460(f32) -> f32),
        entry!(0x005dc480, fn_005dc480(ScriptArgs) -> bool),
        entry!(0x005dc4e0, script_force_terminal_back_function(ScriptArgs) -> bool),
        entry!(0x005dc530, script_toggle_depth_bias_function(ScriptArgs) -> bool),
        entry!(0x005dc580, fn_005dc580(ScriptArgs) -> bool),
        entry!(0x005dc5a0, fn_005dc5a0(ScriptArgs) -> bool),
        entry!(0x005dc5c0, fn_005dc5c0(ScriptArgs) -> bool),
        entry!(0x005dc5e0, script_set_minimal_use_function(ScriptArgs) -> bool),
        entry!(0x005dc6a0, fn_005dc6a0(ScriptArgs) -> bool),
        entry!(0x005dc710, fn_005dc710(Ptr, u8)),
        entry!(0x005dc730, script_show_quest_stages_function(ScriptArgs) -> bool),
        entry!(0x005dc960, fn_005dc960(Ptr) -> u8),
        entry!(0x005dc980, fn_005dc980(Ptr) -> u8),
        entry!(0x005dc9a0, script_modify_face_gen(ScriptArgs) -> bool),
        entry!(0x005dd4a0, fn_005dd4a0(Ptr, i32) -> f32),
        entry!(0x005dd4e0, fn_005dd4e0(Ptr, u32) -> f32),
        entry!(0x005dd520, fn_005dd520(Ptr, u8)),
        entry!(0x005dd540, fn_005dd540(Ptr, u8)),
        entry!(0x005dd560, tes_npc_clear_head(Ptr)),
        entry!(0x005dd590, fn_005dd590(Ptr) -> Ptr),
        entry!(0x005dd6f0, fn_005dd6f0(Ptr)),
        entry!(0x005dd7b0, fn_005dd7b0(Ptr)),
        entry!(0x005dd7d0, fn_005dd7d0(Ptr)),
        entry!(0x005dd7f0, fn_005dd7f0(Ptr)),
        entry!(0x005dd810, fn_005dd810(Ptr)),
        entry!(0x005dd830, fn_005dd830(i32) -> u32),
        entry!(0x005dd840, fn_005dd840(i32) -> u32),
        entry!(0x005dd850, fn_005dd850(i32) -> u32),
        entry!(0x005dd860, fn_005dd860(u8)),
        entry!(0x005dd870, script_is_code_redeemed_function(ScriptArgs) -> u8),
        entry!(0x005dd960, fn_005dd960(ScriptArgs) -> u8),
        entry!(0x005dda00, fn_005dda00(ScriptArgs) -> bool),
        entry!(0x005dda70, fn_005dda70(ScriptArgs) -> bool),
        entry!(0x005ddae0, fn_005ddae0(ScriptArgs) -> bool),
        entry!(0x005ddbb0, fn_005ddbb0(ScriptArgs) -> bool),
        entry!(0x005ddc20, fn_005ddc20(ScriptArgs) -> bool),
        entry!(0x005ddc90, fn_005ddc90(ScriptArgs) -> bool),
        entry!(0x005ddd10, fn_005ddd10(ScriptArgs) -> bool),
        entry!(0x005ddd90, fn_005ddd90(ScriptArgs) -> bool),
        entry!(0x005dde10, fn_005dde10(ScriptArgs) -> bool),
        entry!(0x005dde90, fn_005dde90(ScriptArgs) -> bool),
        entry!(0x005ddf10, fn_005ddf10(ScriptArgs) -> bool),
        entry!(0x005ddf90, fn_005ddf90(ScriptArgs) -> bool),
        entry!(0x005de080, fn_005de080(Ptr) -> bool),
        entry!(0x005de0a0, fn_005de0a0(Ptr, u32, u8)),
        entry!(0x005de0f0, fn_005de0f0(Ptr, u32) -> u32),
        entry!(0x005de110, fn_005de110(ScriptArgs) -> bool),
        entry!(0x005de1d0, fn_005de1d0(ScriptArgs) -> bool),
        entry!(0x005de240, fn_005de240(u16)),
        entry!(0x005de250, fn_005de250(ScriptArgs) -> bool),
        entry!(0x005de2c0, fn_005de2c0(u16)),
        entry!(0x005de2d0, script_set_last_ext_door_activated(ScriptArgs) -> bool),
        entry!(0x005de350, fn_005de350(ScriptArgs) -> bool),
        entry!(0x005de380, fn_005de380(Ptr)),
        entry!(0x005de3a0, script_set_task_thread_sleep_function(ScriptArgs) -> bool),
        entry!(0x005de410, fn_005de410(u32)),
        entry!(0x005de420, script_set_task_thread_update_sleep_function(ScriptArgs) -> bool),
        entry!(0x005de490, fn_005de490(u32)),
        entry!(0x005de4a0, script_is_image_space_active_function(ScriptArgs) -> bool),
        entry!(0x005de600, fn_005de600(ScriptArgs) -> bool),
        entry!(0x005de670, fn_005de670(ScriptArgs) -> bool),
        entry!(0x005de700, script_add_item_to_leveled_list_function(ScriptArgs) -> bool),
        entry!(0x005de840, script_remap_water_type_function(ScriptArgs) -> bool),
        entry!(0x005de900, fn_005de900(ScriptArgs) -> bool),
        entry!(0x005de920, script_set_hardcore_function(ScriptArgs) -> bool),
        entry!(0x005de9b0, fn_005de9b0(Ptr)),
        entry!(0x005de9d0, fn_005de9d0(ScriptArgs) -> bool),
        entry!(0x005de9f0, script_reset_xp_function(ScriptArgs) -> bool),
        entry!(0x005dea80, fn_005dea80(ScriptArgs) -> bool),
        entry!(0x005deaa0, fn_005deaa0(ScriptArgs) -> bool),
        entry!(0x005deb10, fn_005deb10(ScriptArgs) -> bool),
        entry!(0x005dec30, fn_005dec30(ScriptArgs) -> bool),
        entry!(0x005ded40, fn_005ded40(ScriptArgs) -> bool),
        entry!(0x005dee30, fn_005dee30(ScriptArgs) -> bool),
        entry!(0x005dee90, fn_005dee90(ScriptArgs) -> bool),
        entry!(0x005deef0, fn_005deef0(ScriptArgs) -> bool),
        entry!(0x005def70, fn_005def70(Ptr) -> u32),
        entry!(0x005def90, fn_005def90(ScriptArgs) -> bool),
        entry!(0x005defe0, fn_005defe0(Ptr, u8)),
        entry!(0x005df510, fn_005df510(Ptr, i32, Ptr)),
        entry!(0x005df560, fn_005df560(Ptr, i32, Ptr, Ptr, Ptr)),
        entry!(0x005df5b0, fn_005df5b0(Ptr) -> Ptr),
        entry!(0x005df930, script_get_dlc_count_function(ScriptArgs) -> bool),
        entry!(0x005dfa10, fn_005dfa10(Ptr) -> u32),
        entry!(0x005dfa20, script_is_dlc_installed_function(ScriptArgs) -> bool),
        entry!(0x005dfba0, fn_005dfba0(ScriptArgs) -> bool),
        entry!(0x005dfc10, script_is_loading_menu_enabled(ScriptArgs) -> bool),
        entry!(0x005dfc70, script_is_actor_ref_essential(ScriptArgs) -> bool),
        entry!(0x005dfd30, script_set_actor_ref_essential(ScriptArgs) -> bool),
        entry!(0x005dfde0, script_is_no_human_gore_exe(ScriptArgs) -> bool),
        entry!(0x005dfe30, fn_005dfe30(ScriptArgs) -> bool),
        entry!(0x005dfe90, script_persistent_ref_count_function(ScriptArgs) -> bool),
        entry!(0x005dff70, script_always_show_actor_subtitles(ScriptArgs) -> bool),
        entry!(0x005e0020, fn_005e0020(ScriptArgs) -> bool),
        entry!(0x005e0080, fn_005e0080(u8)),
        entry!(0x005e0090, script_show_sleep_wait_menu(ScriptArgs) -> bool),
        entry!(0x005e0100, script_get_container_inventory_count(ScriptArgs) -> bool),
        entry!(0x005e0160, fn_005e0160(ScriptArgs) -> bool),
        entry!(0x005e0190, fn_005e0190(Ptr, u32) -> Ptr),
        entry!(0x005e01b0, fn_005e01b0(Ptr, u32) -> bool),
        entry!(0x005e0200, fn_005e0200(Ptr, u32) -> bool),
        entry!(0x005e02b0, fn_005e02b0(Ptr, u32) -> u32),
        entry!(0x005e02e0, fn_005e02e0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fake virtual functions the tests put into vtables.
    const V_NAME: u32 = 0x0900_0130;
    const V_IS_ACTOR: u32 = 0x0900_0100;

    /// An engine with the pages of the globals these commands read mapped and
    /// set to the exe's values.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000u32,
            0x0101_7000,
            0x0101_a000,
            0x0102_3000,
            0x0102_f000,
            0x0103_d000,
            0x010e_c000,
            0x011a_d000,
            0x011b_0000,
            0x011d_5000,
            0x011d_e000,
            0x0119_b000,
            0x011a_e000,
            0x011a_f000,
            0x011d_9000,
            0x0101_6000,
            0x0102_9000,
            0x011c_5000,
            0x011d_d000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(MINUS_ONE_DOUBLE, -1.0f64);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(RADIANS_TO_DEGREES, 57.2957763671875f64);
        e.set_global(DEGREES_TO_RADIANS, f64::from_bits(0x3f91_df46_a000_0000));
        e.set_global(EULER_NUMBER, 2.7182817f32);
        e.set_global(HUNDRED, 100.0f64);
        e.set_global(DEFAULT_PERCENT, 100.0f32);
        for lane in 0..4 {
            e.set_global(VECTOR_ONES + 4 * lane, 1.0f32);
        }
        e.register(CONSOLE_PRINT, |_, _| Ret::default());
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

    /// The addresses of all logged calls, in order.
    fn call_order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    fn set_echo(e: &mut Engine, on: bool) {
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_ECHO, on as u8);
    }

    fn set_player(e: &mut Engine) -> u32 {
        let player = object(e);
        e.set_global(PLAYER, player);
        player
    }

    fn f(bits: u32) -> f32 {
        f32::from_bits(bits)
    }

    /// The test of a "parse one value, hand it to a condition function"
    /// command: the condition function gets `(thisObj, value, 0, result)`.
    fn check_parse_then_condition(command_address: u32, condition: u32) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[0x77]);
        e.register(condition, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_eq!(
            calls(&e, PARSE_PARAMETERS)[0][..7],
            [1, 2, 8, this_obj, 0, 5, 6]
        );
        assert_eq!(
            calls(&e, condition),
            vec![vec![this_obj, 0x77, 0, a.result.addr()]]
        );
        // The condition function's AL is the command's.
        e.register(condition, |_, _| false.into_ret());
        assert!(!e.call(command_address, &args![a]).bool());
        // Parameters that do not parse: false, nothing called.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(command_address, &args![a]).bool());
        assert!(calls(&e, condition).is_empty());
    }

    /// The test of a "no arguments, return a condition function" command.
    fn check_condition_only(command_address: u32, condition: u32) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(condition, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_eq!(
            calls(&e, condition),
            vec![vec![this_obj, 0, 0, a.result.addr()]]
        );
        e.register(condition, |_, _| false.into_ret());
        assert!(!e.call(command_address, &args![a]).bool());
    }

    #[test]
    fn get_vats_front_target_visible_passes_the_parsed_value_to_the_condition_function() {
        check_parse_then_condition(0x005d_b810, GET_VATS_FRONT_TARGET_VISIBLE_CONDITION);
    }

    #[test]
    fn fn_005dbe10_counts_up_and_skips_zero() {
        let mut e = engine();
        e.set_global(COUNTER, 5u32);
        assert_eq!(e.call(0x005d_be10, &args![]).u32(), 6);
        assert_eq!(e.global::<u32>(COUNTER), 6);
        // The 16-bit counter wraps to 0, which is replaced by 10.
        e.set_global(COUNTER, 0xffffu32);
        assert_eq!(e.call(0x005d_be10, &args![]).u32(), 10);
        assert_eq!(e.global::<u32>(COUNTER), 10);
        // Bits above the low 16 are dropped.
        e.set_global(COUNTER, 0x1_2345u32);
        assert_eq!(e.call(0x005d_be10, &args![]).u32(), 0x2346);
    }

    #[test]
    fn fn_005dbe40_tests_the_flags_word_against_the_mask() {
        let mut e = engine();
        let this = object(&mut e);
        e.mem.set_u32(this + 0x10, 0b1010);
        assert!(e.call(0x005d_be40, &args![this, 0b0010u32]).bool());
        assert!(!e.call(0x005d_be40, &args![this, 0b0101u32]).bool());
        assert!(!e.call(0x005d_be40, &args![this, 0u32]).bool());
    }

    /// Doubles for the vector helpers: `00561240` replicates lane 0.
    fn vector_engine() -> Engine {
        let mut e = engine();
        e.register(VECTOR_SPLAT_X, |e, a| {
            let x = e.mem.f32(a[0]);
            for lane in 0..4 {
                e.mem.set_f32(a[1] + 4 * lane, x);
            }
            Ret::default()
        });
        e.register(VECTOR_CONSTRUCT_NOOP, |_, a| a[0].into_ret());
        e
    }

    fn put_vector(e: &mut Engine, at: u32, lanes: [f32; 4]) {
        for (i, lane) in lanes.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *lane);
        }
    }

    fn get_vector(e: &Engine, at: u32) -> [f32; 4] {
        [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
    }

    #[test]
    fn fn_005dbf20_builds_a_vector_with_only_the_first_lane() {
        let mut e = engine();
        let vector = e.mem.alloc(16);
        put_vector(&mut e, vector, [9.0; 4]);
        let returned = e.call(0x005d_bf20, &args![vector, 2.5f32]).u32();
        assert_eq!(returned, vector);
        assert_eq!(get_vector(&e, vector), [2.5, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn fn_005dbf70_interpolates_lane_by_lane() {
        let mut e = vector_engine();
        let (output, a, b, weight) = (
            e.mem.alloc(16),
            e.mem.alloc(16),
            e.mem.alloc(16),
            e.mem.alloc(16),
        );
        put_vector(&mut e, a, [1.0, 2.0, 3.0, 4.0]);
        put_vector(&mut e, b, [10.0, 20.0, 30.0, 40.0]);
        put_vector(&mut e, weight, [0.25, 9.0, 9.0, 9.0]);
        start_log(&mut e);
        e.call(0x005d_bf70, &args![output, a, b, weight]);
        // `t * b + (1 - t) * a`.
        assert_eq!(get_vector(&e, output), [3.25, 6.5, 9.75, 13.0]);
        // The first lane of the weight vector is replicated by `00561240`.
        assert_eq!(calls(&e, VECTOR_SPLAT_X).len(), 1);
        assert_eq!(calls(&e, VECTOR_SPLAT_X)[0][0], weight);
    }

    #[test]
    fn fn_005dbec0_interpolates_the_two_vectors_with_the_weight_at_0x40() {
        let mut e = vector_engine();
        let this = e.mem.alloc(0x50);
        let output = e.mem.alloc(16);
        put_vector(&mut e, this, [0.0, 4.0, 8.0, 12.0]);
        put_vector(&mut e, this + 0x10, [4.0, 8.0, 12.0, 16.0]);
        e.mem.set_f32(this + 0x40, 0.5);
        e.call(0x005d_bec0, &args![this, output]);
        assert_eq!(get_vector(&e, output), [2.0, 6.0, 10.0, 14.0]);
        // A weight of 0 gives the first vector, 1 the second.
        e.mem.set_f32(this + 0x40, 0.0);
        e.call(0x005d_bec0, &args![this, output]);
        assert_eq!(get_vector(&e, output), [0.0, 4.0, 8.0, 12.0]);
        e.mem.set_f32(this + 0x40, 1.0);
        e.call(0x005d_bec0, &args![this, output]);
        assert_eq!(get_vector(&e, output), [4.0, 8.0, 12.0, 16.0]);
    }

    #[test]
    fn fn_005dbe60_interpolates_into_a_temporary_and_stores_it_through_00458620() {
        let mut e = vector_engine();
        // `00458620 (output, vector)` copies three lanes.
        e.register(VECTOR_STORE_THREE, |e, a| {
            for lane in 0..3 {
                let value = e.mem.f32(a[1] + 4 * lane);
                e.mem.set_f32(a[0] + 4 * lane, value);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x50);
        let output = e.mem.alloc(16);
        put_vector(&mut e, this, [0.0, 4.0, 8.0, 12.0]);
        put_vector(&mut e, this + 0x10, [4.0, 8.0, 12.0, 16.0]);
        e.mem.set_f32(this + 0x40, 0.5);
        put_vector(&mut e, output, [7.0; 4]);
        start_log(&mut e);
        e.call(0x005d_be60, &args![this, output]);
        // Three lanes are stored, the fourth is untouched.
        assert_eq!(get_vector(&e, output), [2.0, 6.0, 10.0, 7.0]);
        let order = call_order(&e);
        assert_eq!(order[1], VECTOR_CONSTRUCT_NOOP);
        assert_eq!(*order.last().unwrap(), VECTOR_STORE_THREE);
        assert_eq!(calls(&e, VECTOR_STORE_THREE)[0][0], output);
    }

    #[test]
    fn fn_005dc010_stores_the_value_on_actors_only() {
        let mut e = engine();
        let actor = object_with(&mut e, &[(0x100, V_IS_ACTOR)]);
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        e.register(ACTOR_SET_FIELD_10C, |_, _| Ret::default());
        parse_gives(&mut e, true, &[7]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005d_c010, &args![a]).bool());
        assert_eq!(calls(&e, ACTOR_SET_FIELD_10C), vec![vec![actor, 7]]);
        // Not an actor: the value is dropped, the command still succeeds.
        e.register(V_IS_ACTOR, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_c010, &args![a]).bool());
        assert!(calls(&e, ACTOR_SET_FIELD_10C).is_empty());
        // No reference.
        let none = command(&mut e, 0);
        assert!(e.call(0x005d_c010, &args![none]).bool());
        assert!(calls(&e, ACTOR_SET_FIELD_10C).is_empty());
        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_c010, &args![a]).bool());
    }

    #[test]
    fn is_in_crit_stage_passes_the_parsed_value_to_the_condition_function() {
        check_parse_then_condition(0x005d_c090, IS_IN_CRIT_STAGE_CONDITION);
    }

    /// Parse double for the angle commands: a `float` and an integer flag.
    fn parse_angle(e: &mut Engine, value: f32, flag: u32) {
        parse_gives(e, true, &[value.to_bits(), flag]);
    }

    fn result_double(e: &Engine, a: ScriptArgs) -> f64 {
        e.mem.f64(a.result.addr())
    }

    fn radians_of(degrees: f32) -> f32 {
        (degrees as f64 * f64::from_bits(0x3f91_df46_a000_0000)) as f32
    }

    #[test]
    fn fn_005dc0f0_converts_between_degrees_and_the_function_values() {
        let mut e = engine();
        // The two float functions: `+ 1` and `* 0.5`.
        e.register(FLOAT_FN_004E44B0, |_, a| (f(a[0]) + 1.0).into_ret());
        e.register(FLOAT_FN_004B5510, |_, a| (f(a[0]) * 0.5).into_ret());
        let a = command(&mut e, 0);
        // Flag clear: degrees to radians (rounded to float), then 004e44b0.
        parse_angle(&mut e, 90.0, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_c0f0, &args![a]).bool());
        let radians = radians_of(90.0);
        assert_eq!(calls(&e, FLOAT_FN_004E44B0), vec![vec![radians.to_bits()]]);
        assert_eq!(result_double(&e, a), (radians + 1.0) as f64);
        // Flag set: clamped to [-1, 1], then 004b5510, times 180 / pi.
        for (value, clamped) in [(2.0f32, 1.0f32), (-5.0, -1.0), (0.25, 0.25)] {
            parse_angle(&mut e, value, 1);
            start_log(&mut e);
            assert!(e.call(0x005d_c0f0, &args![a]).bool());
            assert_eq!(calls(&e, FLOAT_FN_004B5510), vec![vec![clamped.to_bits()]]);
            assert_eq!(
                result_double(&e, a),
                (clamped * 0.5) as f64 * 57.2957763671875
            );
        }
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_c0f0, &args![a]).bool());
    }

    #[test]
    fn fn_005dc1b0_converts_between_degrees_and_the_function_values() {
        let mut e = engine();
        e.register(FLOAT_FN_004E4470, |_, a| (f(a[0]) + 1.0).into_ret());
        e.register(FLOAT_FN_004F6E40, |_, a| (f(a[0]) * 0.5).into_ret());
        let a = command(&mut e, 0);
        parse_angle(&mut e, 45.0, 0);
        assert!(e.call(0x005d_c1b0, &args![a]).bool());
        assert_eq!(result_double(&e, a), (radians_of(45.0) + 1.0) as f64);
        for (value, clamped) in [(1.5f32, 1.0f32), (-1.5, -1.0), (-0.5, -0.5)] {
            parse_angle(&mut e, value, 1);
            start_log(&mut e);
            assert!(e.call(0x005d_c1b0, &args![a]).bool());
            assert_eq!(calls(&e, FLOAT_FN_004F6E40), vec![vec![clamped.to_bits()]]);
            assert_eq!(
                result_double(&e, a),
                (clamped * 0.5) as f64 * 57.2957763671875
            );
        }
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_c1b0, &args![a]).bool());
    }

    #[test]
    fn fn_005dc270_forwards_to_004f6e40() {
        let mut e = engine();
        e.register(FLOAT_FN_004F6E40, |_, a| (f(a[0]) + 0.5).into_ret());
        start_log(&mut e);
        assert_eq!(e.call(0x005d_c270, &args![2.0f32]).f32(), 2.5);
        assert_eq!(calls(&e, FLOAT_FN_004F6E40), vec![vec![2.0f32.to_bits()]]);
    }

    #[test]
    fn fn_005dc290_does_not_clamp() {
        let mut e = engine();
        e.register(FLOAT_FN_0057DD70, |_, a| (f(a[0]) + 1.0).into_ret());
        e.register(FLOAT_FN_004B1460, |_, a| (f(a[0]) * 0.5).into_ret());
        let a = command(&mut e, 0);
        parse_angle(&mut e, 30.0, 0);
        assert!(e.call(0x005d_c290, &args![a]).bool());
        assert_eq!(result_double(&e, a), (radians_of(30.0) + 1.0) as f64);
        // Flag set: the value 5.0 reaches 004b1460 unchanged.
        parse_angle(&mut e, 5.0, 1);
        start_log(&mut e);
        assert!(e.call(0x005d_c290, &args![a]).bool());
        assert_eq!(calls(&e, FLOAT_FN_004B1460), vec![vec![5.0f32.to_bits()]]);
        assert_eq!(result_double(&e, a), 2.5 * 57.2957763671875);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_c290, &args![a]).bool());
    }

    #[test]
    fn fn_005dc330_forwards_to_004b1460() {
        let mut e = engine();
        e.register(FLOAT_FN_004B1460, |_, a| (f(a[0]) + 0.5).into_ret());
        assert_eq!(e.call(0x005d_c330, &args![2.0f32]).f32(), 2.5);
    }

    /// The test of the "parse a float, store the result of a float function"
    /// commands.
    fn check_float_command(command_address: u32, function: u32) {
        let mut e = engine();
        e.register(function, |_, a| (f(a[0]) * 3.0).into_ret());
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[1.5f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_eq!(calls(&e, function), vec![vec![1.5f32.to_bits()]]);
        assert_eq!(result_double(&e, a), 4.5);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(command_address, &args![a]).bool());
        assert!(calls(&e, function).is_empty());
    }

    #[test]
    fn fn_005dc350_stores_the_result_of_004019b0() {
        check_float_command(0x005d_c350, FLOAT_FN_004019B0);
    }

    #[test]
    fn fn_005dc480_stores_the_result_of_00408840() {
        check_float_command(0x005d_c480, FLOAT_FN_00408840);
    }

    /// `00eca920` as the natural logarithm.
    fn ln_engine() -> Engine {
        let mut e = engine();
        e.register(CRT_DOUBLE_FN_00ECA920, |_, a| {
            let x = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            Ret {
                st0: x.ln(),
                ..Ret::default()
            }
        });
        e
    }

    #[test]
    fn fn_005dc460_widens_the_argument_and_rounds_the_result_to_float() {
        let mut e = ln_engine();
        start_log(&mut e);
        let result = e.call(0x005d_c460, &args![8.0f32]).f32();
        assert_eq!(result, 8.0f64.ln() as f32);
        // The CRT function got the argument as a double (two words).
        assert_eq!(
            calls(&e, CRT_DOUBLE_FN_00ECA920),
            vec![vec![
                8.0f64.to_bits() as u32,
                (8.0f64.to_bits() >> 32) as u32
            ]]
        );
    }

    #[test]
    fn fn_005dc440_forwards_to_005dc460() {
        let mut e = ln_engine();
        assert_eq!(
            e.call(0x005d_c440, &args![2.0f32]).f32(),
            2.0f64.ln() as f32
        );
    }

    #[test]
    fn fn_005dc3b0_divides_the_logarithms_of_the_value_and_the_base() {
        let mut e = ln_engine();
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[8.0f32.to_bits(), 2.0f32.to_bits()]);
        assert!(e.call(0x005d_c3b0, &args![a]).bool());
        let expected = (8.0f64.ln() as f32) as f64 / (2.0f64.ln() as f32) as f64;
        assert_eq!(result_double(&e, a), expected);
        // A base of 1 gives 0 without taking logarithms.
        parse_gives(&mut e, true, &[8.0f32.to_bits(), 1.0f32.to_bits()]);
        e.mem.set_f64(a.result.addr(), 5.0);
        start_log(&mut e);
        assert!(e.call(0x005d_c3b0, &args![a]).bool());
        assert_eq!(result_double(&e, a), 0.0);
        assert!(calls(&e, CRT_DOUBLE_FN_00ECA920).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_c3b0, &args![a]).bool());
    }

    #[test]
    fn fn_005dc3b0_starts_with_value_0_and_base_e() {
        let mut e = ln_engine();
        let a = command(&mut e, 0);
        // The locals the command passes to `ParseParameters` start as 0.0
        // and e.
        e.register_double(PARSE_PARAMETERS, |e, a| {
            assert_eq!(e.mem.u32(a[7]), 0.0f32.to_bits());
            assert_eq!(e.mem.u32(a[8]), 2.7182817f32.to_bits());
            true.into_ret()
        });
        assert!(e.call(0x005d_c3b0, &args![a]).bool());
        // log(0) / log(e): minus infinity.
        assert_eq!(result_double(&e, a), f64::NEG_INFINITY);
    }

    #[test]
    fn force_terminal_back_shows_the_previous_terminal_only_with_a_menu() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| 0.into_ret());
        e.register(TILE_GET_MENU, |_, _| 0x7700.into_ret());
        e.register(DISPLAY_PREVIOUS_TERMINAL, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_c4e0, &args![a]).bool());
        assert_eq!(calls(&e, TILE_GET_MENU_BY_CLASS), vec![vec![0x421]]);
        assert!(calls(&e, TILE_GET_MENU).is_empty());
        assert!(calls(&e, DISPLAY_PREVIOUS_TERMINAL).is_empty());
        // A tile whose menu is null.
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| 0x6600.into_ret());
        e.register(TILE_GET_MENU, |_, _| 0.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_c4e0, &args![a]).bool());
        assert_eq!(calls(&e, TILE_GET_MENU), vec![vec![0x6600]]);
        assert!(calls(&e, DISPLAY_PREVIOUS_TERMINAL).is_empty());
        // Both present.
        e.register(TILE_GET_MENU, |_, _| 0x7700.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_c4e0, &args![a]).bool());
        assert_eq!(calls(&e, DISPLAY_PREVIOUS_TERMINAL), vec![vec![0x7700]]);
    }

    #[test]
    fn toggle_depth_bias_flips_the_flag_and_prints_it() {
        let mut e = engine();
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_c530, &args![a]).bool());
        assert_eq!(e.global::<u8>(DEPTH_BIAS_FLAG), 1);
        assert!(e.call(0x005d_c530, &args![a]).bool());
        assert_eq!(e.global::<u8>(DEPTH_BIAS_FLAG), 0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![
                vec![MSG_DEPTH_BIAS, TEXT_ON],
                vec![MSG_DEPTH_BIAS, TEXT_OFF]
            ]
        );
        // Any non-zero byte counts as on and is cleared.
        e.set_global(DEPTH_BIAS_FLAG, 7u8);
        assert!(e.call(0x005d_c530, &args![a]).bool());
        assert_eq!(e.global::<u8>(DEPTH_BIAS_FLAG), 0);
    }

    #[test]
    fn fn_005dc580_disables_the_pipboy_radio() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.register(PIPBOY_RADIO_ENABLE, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_c580, &args![a]).bool());
        assert_eq!(calls(&e, PIPBOY_RADIO_ENABLE), vec![vec![0]]);
    }

    #[test]
    fn fn_005dc5a0_returns_the_gore_condition_function_result() {
        check_condition_only(0x005d_c5a0, IS_GORE_DISABLED_CONDITION);
    }

    #[test]
    fn fn_005dc5c0_returns_the_actors_in_high_condition_function_result() {
        check_condition_only(0x005d_c5c0, GET_ACTORS_IN_HIGH_CONDITION);
    }

    /// A reference whose base form (at `+0x20`) has the given type byte.
    fn reference_with_base(e: &mut Engine, form_type: u8) -> (u32, u32) {
        let base = object(e);
        e.mem.set_u8(base + 4, form_type);
        let reference = object(e);
        e.mem.set_u32(reference + 0x20, base);
        (reference, base)
    }

    fn minimal_use_engine() -> Engine {
        let mut e = engine();
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(FORM_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
        e.register(DOOR_SET_MINIMAL_USE, |_, _| Ret::default());
        e.register(CHECK_QUEST_TARGET_UPDATE, |_, _| Ret::default());
        e.register(LOG_STUB, |_, _| Ret::default());
        e.register(V_NAME, |_, _| 0x4e41.into_ret());
        e
    }

    #[test]
    fn set_minimal_use_sets_the_door_state_and_updates_quest_targets() {
        let mut e = minimal_use_engine();
        let player = set_player(&mut e);
        let (reference, base) = reference_with_base(&mut e, 0x1c);
        let script = object_with(&mut e, &[(0x130, V_NAME)]);
        let mut a = command(&mut e, reference);
        a.script_obj = Ptr::new(script);
        for (parsed, state) in [(1u32, 1u32), (0, 0), (5, 1)] {
            parse_gives(&mut e, true, &[parsed]);
            start_log(&mut e);
            assert!(e.call(0x005d_c5e0, &args![a]).bool());
            assert_eq!(calls(&e, DOOR_SET_MINIMAL_USE), vec![vec![base, state]]);
            assert_eq!(
                calls(&e, CHECK_QUEST_TARGET_UPDATE),
                vec![vec![player, player]]
            );
            assert!(calls(&e, LOG_STUB).is_empty());
        }
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_c5e0, &args![a]).bool());
        assert!(calls(&e, DOOR_SET_MINIMAL_USE).is_empty());
    }

    #[test]
    fn set_minimal_use_logs_when_the_reference_is_not_a_door() {
        let mut e = minimal_use_engine();
        set_player(&mut e);
        let script = object_with(&mut e, &[(0x130, V_NAME)]);
        parse_gives(&mut e, true, &[1]);
        // Another form type, no base form, no reference.
        let (reference, _) = reference_with_base(&mut e, 0x1b);
        let no_base = object(&mut e);
        for this_obj in [reference, no_base, 0] {
            let mut a = command(&mut e, this_obj);
            a.script_obj = Ptr::new(script);
            start_log(&mut e);
            assert!(e.call(0x005d_c5e0, &args![a]).bool());
            assert_eq!(calls(&e, LOG_STUB), vec![vec![MSG_SET_MINIMAL_USE, 0x4e41]]);
            assert!(calls(&e, DOOR_SET_MINIMAL_USE).is_empty());
            assert!(calls(&e, CHECK_QUEST_TARGET_UPDATE).is_empty());
        }
    }

    #[test]
    fn fn_005dc6a0_stores_the_flag_on_the_player() {
        let mut e = engine();
        let player = set_player(&mut e);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[3]);
        assert!(e.call(0x005d_c6a0, &args![a]).bool());
        assert_eq!(e.mem.u8(player + 0x7c7), 1);
        parse_gives(&mut e, true, &[0]);
        assert!(e.call(0x005d_c6a0, &args![a]).bool());
        assert_eq!(e.mem.u8(player + 0x7c7), 0);
        e.mem.set_u8(player + 0x7c7, 9);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_c6a0, &args![a]).bool());
        assert_eq!(e.mem.u8(player + 0x7c7), 9);
    }

    #[test]
    fn fn_005dc710_stores_the_byte_at_0x7c7() {
        let mut e = engine();
        let this = object(&mut e);
        e.call(0x005d_c710, &args![this, 0x105u32]);
        assert_eq!(e.mem.u8(this + 0x7c7), 5);
        assert_eq!(e.mem.u8(this + 0x7c8), 0);
    }

    #[test]
    fn fn_005dc960_and_fn_005dc980_read_single_bytes() {
        let mut e = engine();
        let this = object(&mut e);
        e.mem.set_u8(this + 1, 0x11);
        e.mem.set_u8(this + 100, 0x64);
        assert_eq!(e.call(0x005d_c960, &args![this]).u8(), 0x11);
        assert_eq!(e.call(0x005d_c980, &args![this]).u8(), 0x64);
    }

    // ---- ShowQuestStages ----------------------------------------------------

    /// Doubles for the list accessors and string helpers of the quest stage
    /// printout; the string operations are only logged.
    fn stages_engine() -> Engine {
        let mut e = engine();
        e.register(QUEST_STAGE_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(LIST_ITEM_PTR, |_, a| a[0].into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(PLUS_4, |_, a| (a[0] + 4).into_ret());
        e.register(STAGE_INDEX, |e, a| (e.mem.u8(a[0]) as u32).into_ret());
        e.register(GET_PROCESS, |e, a| e.mem.u32(a[0] + 0x68).into_ret());
        e.register(SPRINTF_S, |_, _| Ret::default());
        e.register(BS_STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(BS_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(BS_STRING_ASSIGN, |_, _| Ret::default());
        e.register(BS_STRING_APPEND, |_, _| Ret::default());
        e.register(BS_STRING_COPY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xc0c0);
            e.mem.set_u32(a[0] + 4, 0x0505);
            a[0].into_ret()
        });
        e.register(PRINT_LINE, |_, _| Ret::default());
        e
    }

    /// A stage with the given index, flag and `(value, has process)` items.
    /// The embedded list head sits at `stage + 4` (item at +4, next at +8).
    fn make_stage(e: &mut Engine, index: u8, flag: u8, items: &[(u8, bool)]) -> u32 {
        let stage = e.mem.alloc(0x40);
        e.mem.set_u8(stage, index);
        e.mem.set_u8(stage + 1, flag);
        let mut previous_next_cell = stage + 8;
        for (position, (value, has_process)) in items.iter().enumerate() {
            let item = e.mem.alloc(0x80);
            e.mem.set_u8(item + 100, *value);
            e.mem
                .set_u32(item + 0x68, if *has_process { 0x9999 } else { 0 });
            if position == 0 {
                e.mem.set_u32(stage + 4, item);
            } else {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, item);
                e.mem.set_u32(previous_next_cell, node);
                previous_next_cell = node + 4;
            }
        }
        stage
    }

    /// A quest whose stage list (at `+0x44`) holds the given stages.
    fn make_quest(e: &mut Engine, stages: &[u32]) -> u32 {
        let quest = e.mem.alloc(0x80);
        let mut previous_next_cell = quest + 0x48;
        for (position, stage) in stages.iter().enumerate() {
            if position == 0 {
                e.mem.set_u32(quest + 0x44, *stage);
            } else {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, *stage);
                e.mem.set_u32(previous_next_cell, node);
                previous_next_cell = node + 4;
            }
        }
        quest
    }

    /// The logged string operations as `(function, words after the first)`;
    /// the print gets both of its words.
    fn string_operations(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| {
                [SPRINTF_S, BS_STRING_ASSIGN, BS_STRING_APPEND, PRINT_LINE].contains(a)
            })
            .map(|(a, words)| {
                let keep = if *a == PRINT_LINE {
                    words.clone()
                } else {
                    words[1..].to_vec()
                };
                (*a, keep)
            })
            .collect()
    }

    #[test]
    fn show_quest_stages_prints_one_line_per_stage_with_its_items() {
        let mut e = stages_engine();
        set_echo(&mut e, true);
        let stage_one = make_stage(&mut e, 10, 1, &[(7, true), (8, false)]);
        let stage_two = make_stage(&mut e, 20, 0, &[]);
        let quest = make_quest(&mut e, &[stage_one, stage_two]);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[quest]);
        start_log(&mut e);
        assert!(e.call(0x005d_c730, &args![a]).bool());
        let buffer = calls(&e, SPRINTF_S)[0][0];
        assert_eq!(
            string_operations(&e),
            vec![
                (SPRINTF_S, vec![0x1f, FORMAT_STAGE, 10, 1]),
                (BS_STRING_ASSIGN, vec![buffer]),
                (BS_STRING_APPEND, vec![TEXT_OPEN_PAREN]),
                (SPRINTF_S, vec![0x1f, FORMAT_ITEM, 7, 1]),
                (BS_STRING_APPEND, vec![buffer]),
                (BS_STRING_APPEND, vec![TEXT_SEPARATOR]),
                (SPRINTF_S, vec![0x1f, FORMAT_ITEM, 8, 0]),
                (BS_STRING_APPEND, vec![buffer]),
                (BS_STRING_APPEND, vec![TEXT_CLOSE_PAREN]),
                (PRINT_LINE, vec![0xc0c0, 0x0505]),
                (SPRINTF_S, vec![0x1f, FORMAT_STAGE, 20, 0]),
                (BS_STRING_ASSIGN, vec![buffer]),
                (PRINT_LINE, vec![0xc0c0, 0x0505]),
            ]
        );
        // The string is constructed once and destroyed once.
        assert_eq!(calls(&e, BS_STRING_CONSTRUCT).len(), 1);
        assert_eq!(
            calls(&e, BS_STRING_CONSTRUCT),
            calls(&e, BS_STRING_DESTRUCT)
        );
    }

    #[test]
    fn show_quest_stages_lists_items_only_when_the_item_list_has_a_second_node() {
        let mut e = stages_engine();
        set_echo(&mut e, true);
        // One item: the list head has no next node, so no items are printed.
        let stage = make_stage(&mut e, 5, 0, &[(7, true)]);
        let quest = make_quest(&mut e, &[stage]);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[quest]);
        start_log(&mut e);
        assert!(e.call(0x005d_c730, &args![a]).bool());
        assert!(calls(&e, BS_STRING_APPEND).is_empty());
        assert_eq!(calls(&e, PRINT_LINE).len(), 1);
    }

    #[test]
    fn show_quest_stages_prints_nothing_without_the_echo_flag() {
        let mut e = stages_engine();
        set_echo(&mut e, false);
        let stage = make_stage(&mut e, 10, 1, &[]);
        let quest = make_quest(&mut e, &[stage]);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[quest]);
        start_log(&mut e);
        assert!(e.call(0x005d_c730, &args![a]).bool());
        assert!(calls(&e, PRINT_LINE).is_empty());
        assert!(calls(&e, SPRINTF_S).is_empty());
        // The string is still built and destroyed.
        assert_eq!(calls(&e, BS_STRING_CONSTRUCT).len(), 1);
        assert_eq!(calls(&e, BS_STRING_DESTRUCT).len(), 1);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_c730, &args![a]).bool());
        assert!(calls(&e, BS_STRING_CONSTRUCT).is_empty());
    }

    #[test]
    fn show_quest_stages_stops_at_a_node_without_a_stage() {
        let mut e = stages_engine();
        set_echo(&mut e, true);
        let stage = make_stage(&mut e, 10, 1, &[]);
        // The head holds a stage, the second node holds null.
        let quest = make_quest(&mut e, &[stage, 0]);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[quest]);
        start_log(&mut e);
        assert!(e.call(0x005d_c730, &args![a]).bool());
        assert_eq!(calls(&e, PRINT_LINE).len(), 1);
        // An empty list (head item null) prints nothing.
        let empty = make_quest(&mut e, &[]);
        parse_gives(&mut e, true, &[empty]);
        start_log(&mut e);
        assert!(e.call(0x005d_c730, &args![a]).bool());
        assert!(calls(&e, PRINT_LINE).is_empty());
    }

    // ---- ModifyFaceGen ----------------------------------------------------------

    const V_TARGET: u32 = 0x0900_01b8;
    const V_OBJECT_1B0: u32 = 0x0900_01b0;
    const V_OBJECT_1AC: u32 = 0x0900_01ac;
    const V_FLAG_218: u32 = 0x0900_0218;
    const V_NODE: u32 = 0x0900_01d0;
    const V_ANIMATION: u32 = 0x0900_01e4;
    const V_BASE_DATA: u32 = 0x0900_01e8;
    const V_SET_EXPRESSION: u32 = 0x0900_0114;
    const V_GET_PHONEME: u32 = 0x0900_00a4;
    const V_SET_PHONEMES: u32 = 0x0900_00ec;
    const V_SET_MODIFIERS: u32 = 0x0900_00f0;
    const V_RESET: u32 = 0x0900_00b4;
    const V_NODE_DATA: u32 = 0x0900_000c;
    const V_NODE_E8: u32 = 0x0900_00e8;
    const V_PROCESS_79C: u32 = 0x0900_079c;
    const V_PROCESS_794: u32 = 0x0900_0794;
    const V_PROCESS_7A4: u32 = 0x0900_07a4;
    const V_PROCESS_58: u32 = 0x0900_0058;

    /// Where the fake reference keeps what its virtual functions return.
    const REF_FLAG_218: u32 = 0x300;
    const REF_NODE: u32 = 0x304;
    const REF_ANIMATION: u32 = 0x308;
    const REF_TARGET: u32 = 0x30c;
    const REF_OBJECT_1B0: u32 = 0x310;
    const REF_OBJECT_1AC: u32 = 0x314;
    /// Where the fake target copies the arrays it is given (phonemes,
    /// modifiers).
    const TARGET_PHONEMES: u32 = 0x400;
    const TARGET_MODIFIERS: u32 = 0x500;

    fn put_keywords(e: &mut Engine) {
        e.mem.set_cstr(KEYWORD_EXPRESSION, b"expression");
        e.mem.set_cstr(KEYWORD_PHONEME, b"phoneme");
        e.mem.set_cstr(KEYWORD_MODIFIER, b"modifier");
        e.mem.set_cstr(KEYWORD_COORD, b"coord");
        e.mem.set_cstr(KEYWORD_RESET, b"reset");
    }

    /// `ParseParameters` double for `ModifyFaceGen`: the keyword, the index
    /// and the value.
    fn parse_face_gen(e: &mut Engine, ok: bool, keyword: &str, index: i32, value: i32) {
        let keyword = keyword.as_bytes().to_vec();
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            e.mem.set_cstr(a[7], &keyword);
            e.mem.set_i32(a[8], index);
            e.mem.set_i32(a[9], value);
            ok.into_ret()
        });
    }

    /// The reference, the face generation target and the holder objects
    /// `ModifyFaceGen` works on.
    struct FaceGen {
        a: ScriptArgs,
        reference: u32,
        target: u32,
        holder_1b0: u32,
        holder_1ac: u32,
    }

    fn face_gen_setup() -> (Engine, FaceGen) {
        let mut e = engine();
        put_keywords(&mut e);
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.register(ZERO_FILL, |_, _| Ret::default());
        e.register(STRICMP, |e, a| {
            let first = e.mem.cstr(a[0]).to_ascii_lowercase();
            let second = e.mem.cstr(a[1]).to_ascii_lowercase();
            (first.cmp(&second) as i32 as u32).into_ret()
        });
        e.register(SPRINTF_S, |_, _| Ret::default());
        e.register(BS_STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(BS_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(BS_STRING_APPEND, |_, _| Ret::default());
        e.register(BS_STRING_COPY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xc0c0);
            e.mem.set_u32(a[0] + 4, 0x0505);
            a[0].into_ret()
        });
        e.register(PRINT_LINE, |_, _| Ret::default());
        e.register(EXPRESSION_NAME, |_, a| (0x5000 + a[0]).into_ret());
        e.register(PHONEME_NAME, |_, a| (0x6000 + a[0]).into_ret());
        e.register(MODIFIER_NAME, |_, a| (0x7000 + a[0]).into_ret());
        e.register(SET_FACE_GEN_MODE_BYTE, |_, _| Ret::default());
        // Reference.
        e.register(V_TARGET, |e, a| e.mem.u32(a[0] + REF_TARGET).into_ret());
        e.register(V_OBJECT_1B0, |e, a| {
            e.mem.u32(a[0] + REF_OBJECT_1B0).into_ret()
        });
        e.register(V_OBJECT_1AC, |e, a| {
            e.mem.u32(a[0] + REF_OBJECT_1AC).into_ret()
        });
        e.register(V_FLAG_218, |e, a| e.mem.u32(a[0] + REF_FLAG_218).into_ret());
        e.register(V_NODE, |e, a| e.mem.u32(a[0] + REF_NODE).into_ret());
        e.register(V_ANIMATION, |e, a| {
            e.mem.u32(a[0] + REF_ANIMATION).into_ret()
        });
        e.register(V_BASE_DATA, |_, _| 0xda7a.into_ret());
        // Target.
        e.register(V_SET_EXPRESSION, |_, _| Ret::default());
        e.register(V_GET_PHONEME, |_, a| (a[1] as f32 / 10.0).into_ret());
        e.register(V_SET_PHONEMES, |e, a| {
            for i in 0..16 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + TARGET_PHONEMES + 4 * i, word);
            }
            Ret::default()
        });
        e.register(V_SET_MODIFIERS, |e, a| {
            for i in 0..17 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + TARGET_MODIFIERS + 4 * i, word);
            }
            Ret::default()
        });
        e.register(V_RESET, |_, _| Ret::default());
        let target = object_with(
            &mut e,
            &[
                (0x114, V_SET_EXPRESSION),
                (0xa4, V_GET_PHONEME),
                (0xec, V_SET_PHONEMES),
                (0xf0, V_SET_MODIFIERS),
                (0xb4, V_RESET),
            ],
        );
        // The modifier values: 17 floats `0.5 * index` behind `target + 0xa8`.
        let values = e.mem.alloc(17 * 4);
        for i in 0..17 {
            e.mem.set_f32(values + 4 * i, 0.5 * i as f32);
        }
        e.mem.set_u32(target + 0xa8 + 0xc, values);
        e.mem.set_u32(target + 0xa8 + 0x10, 17);
        let holder_1b0 = object(&mut e);
        let holder_1ac = object(&mut e);
        let reference = object_with(
            &mut e,
            &[
                (0x1b8, V_TARGET),
                (0x1b0, V_OBJECT_1B0),
                (0x1ac, V_OBJECT_1AC),
                (0x218, V_FLAG_218),
                (0x1d0, V_NODE),
                (0x1e4, V_ANIMATION),
                (0x1e8, V_BASE_DATA),
            ],
        );
        e.mem.set_u32(reference + REF_TARGET, target);
        e.mem.set_u32(reference + REF_OBJECT_1B0, holder_1b0);
        e.mem.set_u32(reference + REF_OBJECT_1AC, holder_1ac);
        let a = command(&mut e, reference);
        (
            e,
            FaceGen {
                a,
                reference,
                target,
                holder_1b0,
                holder_1ac,
            },
        )
    }

    fn run_face_gen(e: &mut Engine, g: &FaceGen) -> bool {
        start_log(e);
        e.call(0x005d_c9a0, &args![g.a]).bool()
    }

    #[test]
    fn modify_face_gen_fails_when_the_parameters_do_not_parse() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, false, "expression", 1, 2);
        assert!(!run_face_gen(&mut e, &g));
        assert!(calls(&e, V_SET_EXPRESSION).is_empty());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // The keyword buffer got a leading NUL and `memset` cleared the rest.
        assert_eq!(calls(&e, MEMSET)[0][1..], [0, 0x1ff]);
        // The parameters: the buffer, then the index and the value cells.
        let parse = &calls(&e, PARSE_PARAMETERS)[0];
        assert_eq!(parse.len(), 10);
        assert_eq!(parse[7] + 1, calls(&e, MEMSET)[0][0]);
    }

    #[test]
    fn modify_face_gen_without_a_keyword_prints_expression() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "", -1, 0);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![KEYWORD_EXPRESSION]]);
        assert!(calls(&e, V_TARGET).is_empty());
    }

    #[test]
    fn modify_face_gen_needs_a_target_object() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "expression", 3, 50);
        // No reference.
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_c9a0, &args![none]).bool());
        assert!(calls(&e, V_TARGET).is_empty());
        // The reference has no target.
        e.mem.set_u32(g.reference + REF_TARGET, 0);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(calls(&e, V_TARGET), vec![vec![g.reference, 0]]);
        assert!(calls(&e, V_SET_EXPRESSION).is_empty());
    }

    /// The names of a printed list: `(format, index, name)` of every
    /// `sprintf_s` call and the number of `PrintLine` calls.
    fn printed_names(e: &Engine) -> (Vec<(u32, u32, u32)>, usize) {
        (
            calls(e, SPRINTF_S)
                .iter()
                .map(|w| {
                    assert_eq!(w[1], 0x200);
                    (w[2], w[3], w[4])
                })
                .collect(),
            calls(e, PRINT_LINE).len(),
        )
    }

    fn check_name_list(keyword: &str, count: u32, name_base: u32) {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, keyword, -1, 0);
        assert!(run_face_gen(&mut e, &g));
        let (names, printed) = printed_names(&e);
        assert_eq!(names.len(), count as usize);
        for (i, (format, index, name)) in names.iter().enumerate() {
            let last = i as u32 == count - 1;
            assert_eq!(
                *format,
                if last {
                    FORMAT_NAME_LAST
                } else {
                    FORMAT_NAME_MORE
                }
            );
            assert_eq!(*index, i as u32);
            assert_eq!(*name, name_base + i as u32);
        }
        assert_eq!(printed, 1);
        // One append per name, then the string is printed and destroyed.
        assert_eq!(calls(&e, BS_STRING_APPEND).len(), count as usize);
        assert_eq!(
            calls(&e, BS_STRING_CONSTRUCT),
            calls(&e, BS_STRING_DESTRUCT)
        );
        // No virtual function of the target was called.
        assert!(calls(&e, V_SET_EXPRESSION).is_empty());
        assert!(calls(&e, V_SET_PHONEMES).is_empty());
        assert!(calls(&e, V_SET_MODIFIERS).is_empty());
    }

    #[test]
    fn modify_face_gen_lists_the_expressions() {
        check_name_list("expression", 15, 0x5000);
    }

    #[test]
    fn modify_face_gen_lists_the_phonemes() {
        check_name_list("phoneme", 16, 0x6000);
    }

    #[test]
    fn modify_face_gen_lists_the_modifiers() {
        check_name_list("MODIFIER", 17, 0x7000);
    }

    #[test]
    fn modify_face_gen_sets_one_expression() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "Expression", 3, 50);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(
            calls(&e, V_SET_EXPRESSION),
            vec![vec![g.target, 3, 0.5f32.to_bits()]]
        );
        assert!(calls(&e, PRINT_LINE).is_empty());
    }

    fn phoneme_array(e: &Engine, g: &FaceGen) -> Vec<f32> {
        (0..16)
            .map(|i| e.mem.f32(g.target + TARGET_PHONEMES + 4 * i))
            .collect()
    }

    #[test]
    fn modify_face_gen_replaces_one_phoneme_and_keeps_the_others() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "phoneme", 2, 25);
        assert!(run_face_gen(&mut e, &g));
        let mut expected: Vec<f32> = (0..16).map(|i| i as f32 / 10.0).collect();
        expected[2] = 0.25;
        assert_eq!(phoneme_array(&e, &g), expected);
        // The array is zeroed first, the current values are asked for every
        // other phoneme and the array goes to slot 0xec with 1.0.
        assert_eq!(calls(&e, ZERO_FILL)[0][1..], [0, 0x40]);
        assert_eq!(calls(&e, V_GET_PHONEME).len(), 15);
        let set = &calls(&e, V_SET_PHONEMES)[0];
        assert_eq!((set[0], set[2]), (g.target, 1.0f32.to_bits()));
        assert_eq!(set[1], calls(&e, ZERO_FILL)[0][0]);
        // The mode byte is not touched.
        assert!(calls(&e, SET_FACE_GEN_MODE_BYTE).is_empty());
    }

    #[test]
    fn modify_face_gen_ignores_an_out_of_range_index() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "phoneme", 40, 25);
        assert!(run_face_gen(&mut e, &g));
        let expected: Vec<f32> = (0..16).map(|i| i as f32 / 10.0).collect();
        assert_eq!(phoneme_array(&e, &g), expected);
        assert_eq!(calls(&e, V_GET_PHONEME).len(), 16);
    }

    #[test]
    fn modify_face_gen_replaces_one_modifier_after_clearing_the_mode_byte() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "modifier", 5, 200);
        assert!(run_face_gen(&mut e, &g));
        let mut expected: Vec<f32> = (0..17).map(|i| 0.5 * i as f32).collect();
        expected[5] = 2.0;
        let modifiers: Vec<f32> = (0..17)
            .map(|i| e.mem.f32(g.target + TARGET_MODIFIERS + 4 * i))
            .collect();
        assert_eq!(modifiers, expected);
        // The mode byte is cleared before the array is built.
        let order = call_order(&e);
        let mode = order
            .iter()
            .position(|a| *a == SET_FACE_GEN_MODE_BYTE)
            .unwrap();
        let zero = order.iter().position(|a| *a == ZERO_FILL).unwrap();
        assert!(mode < zero);
        assert_eq!(calls(&e, SET_FACE_GEN_MODE_BYTE), vec![vec![0]]);
        assert_eq!(calls(&e, ZERO_FILL)[0][1..], [0, 0x44]);
        let set = &calls(&e, V_SET_MODIFIERS)[0];
        assert_eq!((set[0], set[2]), (g.target, 1.0f32.to_bits()));
    }

    fn fill_holder(e: &mut Engine, holder: u32) {
        e.mem.set_u8(holder + 0xec, 0xaa);
        e.mem.set_u8(holder + 0xd4, 0xaa);
        e.mem.set_u8(holder + 0xe3, 0xaa);
    }

    #[test]
    fn modify_face_gen_coord_minus_one_attaches_the_object_and_clears_the_bytes() {
        let (mut e, g) = face_gen_setup();
        fill_holder(&mut e, g.holder_1b0);
        parse_face_gen(&mut e, true, "coord", -1, 77);
        e.set_global(FACE_GEN_COORD_VALUE, 5u32);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(calls(&e, V_OBJECT_1B0), vec![vec![g.reference, 0]]);
        assert_eq!(e.mem.u32(g.holder_1b0 + 0xe8), g.reference);
        assert_eq!(e.mem.u8(g.holder_1b0 + 0xec), 0);
        assert_eq!(e.mem.u8(g.holder_1b0 + 0xd4), 0);
        assert_eq!(e.mem.u8(g.holder_1b0 + 0xe3), 0);
        assert_eq!(e.global::<u32>(FACE_GEN_COORD_VALUE), 5);
    }

    #[test]
    fn modify_face_gen_coord_one_sets_the_bytes_and_stores_the_value() {
        let (mut e, g) = face_gen_setup();
        fill_holder(&mut e, g.holder_1b0);
        parse_face_gen(&mut e, true, "coord", 1, 77);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(e.mem.u32(g.holder_1b0 + 0xe8), g.reference);
        assert_eq!(e.mem.u8(g.holder_1b0 + 0xec), 1);
        assert_eq!(e.mem.u8(g.holder_1b0 + 0xd4), 1);
        assert_eq!(e.mem.u8(g.holder_1b0 + 0xe3), 1);
        assert_eq!(e.global::<u32>(FACE_GEN_COORD_VALUE), 77);
        // Other indices do nothing.
        parse_face_gen(&mut e, true, "coord", 2, 99);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(e.global::<u32>(FACE_GEN_COORD_VALUE), 77);
        assert!(calls(&e, V_OBJECT_1B0).is_empty());
    }

    /// Everything the head rebuild (`coord 0`) calls, with a recording
    /// double each; the reference is an actor with a node and an animation.
    fn rebuild_setup() -> (Engine, FaceGen, u32, u32) {
        let (mut e, g) = face_gen_setup();
        let base_form = object(&mut e);
        e.mem.set_u32(g.reference + 0x20, base_form);
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.mem.set_u32(g.reference + REF_FLAG_218, 1);
        let node = object_with(&mut e, &[(0xc, V_NODE_DATA)]);
        e.register(V_NODE_DATA, |_, _| 0x1111.into_ret());
        e.mem.set_u32(g.reference + REF_NODE, node);
        e.mem.set_u32(g.reference + REF_ANIMATION, 0xa11);
        e.register(POINTER_AT_D8, |_, _| 0xd8d8.into_ret());
        e.register(POINTER_AT_78, |_, _| 0x7878.into_ret());
        // The holders answer `+0x18` with an owner object.
        let owner = object_with(&mut e, &[(0xe8, V_NODE_E8)]);
        e.register(V_NODE_E8, |_, _| Ret::default());
        e.register_double(FIELD_18, move |_, _| owner.into_ret());
        e.register(RECURSE_REMOVE_FROM_PALETTE, |_, _| Ret::default());
        let process = object_with(
            &mut e,
            &[
                (0x79c, V_PROCESS_79C),
                (0x794, V_PROCESS_794),
                (0x7a4, V_PROCESS_7A4),
                (0x58, V_PROCESS_58),
            ],
        );
        e.register(V_PROCESS_79C, |_, _| Ret::default());
        e.register(V_PROCESS_794, |_, _| Ret::default());
        e.register(V_PROCESS_7A4, |_, _| Ret::default());
        e.register(V_PROCESS_58, |_, _| Ret::default());
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(NI_POINTER_SET, |_, a| a[0].into_ret());
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        e.register(TES_NPC_INIT_HEAD, |e, a| {
            e.mem.set_u32(a[1], 11);
            e.mem.set_u32(a[2], 22);
            Ret::default()
        });
        e.register(TES_NPC_UPDATE_HEAD, |_, _| Ret::default());
        e.register(TES_NPC_GET_RACE, |_, _| 0x4ace.into_ret());
        e.register(TES_RACE_GET_FACE_GEN_DATA, |_, _| Ret::default());
        e.register(FLOAT_AND_FLAGS_CONSTRUCT, |_, _| Ret::default());
        e.register(NODE_APPLY, |_, _| Ret::default());
        face_gen_data_doubles(&mut e);
        (e, g, base_form, node)
    }

    /// Doubles for what the face generation data's constructor and destructor
    /// call.
    fn face_gen_data_doubles(e: &mut Engine) {
        for address in [
            VECTOR_CONSTRUCT,
            VECTOR_DESTRUCT,
            ARRAY_94_CONSTRUCT,
            ARRAY_A4_CONSTRUCT,
            ARRAY_B4_CONSTRUCT,
            ARRAY_C4_CONSTRUCT,
            LIST_CONSTRUCT,
            LIST_DESTRUCT,
            LIST_PRE_DESTRUCT,
            ARRAY_RESET,
            ARRAY_94_DESTRUCT,
            ARRAY_A4_DESTRUCT,
            ARRAY_B4_DESTRUCT_WRAPPER,
            ARRAY_C4_DESTRUCT_WRAPPER,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn modify_face_gen_coord_zero_rebuilds_the_head() {
        let (mut e, g, base_form, node) = rebuild_setup();
        // The animation's pointer chain: `00496940` then `00537bd0`.
        parse_face_gen(&mut e, true, "coord", 0, 0);
        assert!(run_face_gen(&mut e, &g));
        // The palette is cleaned of both head objects with the skeleton
        // `00537bd0` found.
        assert_eq!(
            calls(&e, RECURSE_REMOVE_FROM_PALETTE),
            vec![vec![g.holder_1b0, 0x7878], vec![g.holder_1ac, 0x7878],]
        );
        assert_eq!(calls(&e, POINTER_AT_78), vec![vec![0xd8d8]]);
        assert_eq!(calls(&e, V_OBJECT_1B0), vec![vec![g.reference, 0]]);
        assert_eq!(calls(&e, V_OBJECT_1AC), vec![vec![g.reference, 0]]);
        assert_eq!(calls(&e, V_PROCESS_79C).len(), 1);
        assert_eq!(calls(&e, V_PROCESS_794).len(), 1);
        assert_eq!(calls(&e, V_PROCESS_7A4).len(), 1);
        assert_eq!(calls(&e, V_PROCESS_58).len(), 1);
        assert_eq!(calls(&e, V_NODE_E8).len(), 2);
        // The head: cleared, initialised with two outputs and updated with
        // the data of slot 0x1e8 and the two values `InitHead` produced.
        assert_eq!(calls(&e, NI_POINTER_SET), vec![vec![base_form + 0x1c4, 0]]);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN),
            vec![vec![base_form + 0x1c8, base_form + 0x1c4]]
        );
        let init = &calls(&e, TES_NPC_INIT_HEAD)[0];
        assert_eq!(init[0], base_form);
        assert_eq!(init[2], init[1] + 4);
        assert_eq!(
            calls(&e, TES_NPC_UPDATE_HEAD),
            vec![vec![base_form, g.reference, 0xda7a, 11, 22]]
        );
        // The face generation data is built, filled by the race and torn
        // down; the node is told about the 16-byte scratch object.
        let construct = &calls(&e, ARRAY_94_CONSTRUCT)[0];
        let face_gen = construct[0] - 0x94;
        assert_eq!(
            calls(&e, TES_RACE_GET_FACE_GEN_DATA),
            vec![vec![0x4ace, base_form, face_gen, 0, 0]]
        );
        assert_eq!(calls(&e, TES_NPC_GET_RACE), vec![vec![base_form]]);
        let scratch = &calls(&e, FLOAT_AND_FLAGS_CONSTRUCT)[0];
        assert_eq!(scratch[1..], [0.0f32.to_bits(), 0, 0]);
        assert_eq!(calls(&e, NODE_APPLY), vec![vec![node, scratch[0]]]);
        assert_eq!(calls(&e, ARRAY_94_DESTRUCT), vec![vec![face_gen + 0x94]]);
        // Order: constructed, filled, applied, destroyed.
        let order = call_order(&e);
        let position = |address: u32| order.iter().position(|a| *a == address).unwrap();
        assert!(position(ARRAY_94_CONSTRUCT) < position(TES_RACE_GET_FACE_GEN_DATA));
        assert!(position(TES_RACE_GET_FACE_GEN_DATA) < position(NODE_APPLY));
        assert!(position(NODE_APPLY) < position(ARRAY_94_DESTRUCT));
        assert!(position(TES_NPC_UPDATE_HEAD) < position(ARRAY_94_CONSTRUCT));
    }

    #[test]
    fn modify_face_gen_coord_zero_skips_the_palette_without_a_node_and_needs_the_flag() {
        let (mut e, g, base_form, _) = rebuild_setup();
        parse_face_gen(&mut e, true, "coord", 0, 0);
        // No node: the palette is left alone but the head is rebuilt.
        e.mem.set_u32(g.reference + REF_NODE, 0);
        assert!(run_face_gen(&mut e, &g));
        assert!(calls(&e, RECURSE_REMOVE_FROM_PALETTE).is_empty());
        assert!(calls(&e, V_ANIMATION).is_empty());
        assert_eq!(calls(&e, V_PROCESS_58).len(), 1);
        assert_eq!(calls(&e, TES_NPC_UPDATE_HEAD)[0][0], base_form);
        assert_eq!(calls(&e, NODE_APPLY)[0][0], 0);
        // Without the flag of slot 0x218 nothing happens.
        e.mem.set_u32(g.reference + REF_FLAG_218, 0);
        assert!(run_face_gen(&mut e, &g));
        assert!(calls(&e, V_NODE).is_empty());
        assert!(calls(&e, TES_NPC_INIT_HEAD).is_empty());
    }

    #[test]
    fn modify_face_gen_reset_asks_the_target_and_sets_the_mode_byte() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "reset", -1, 0);
        assert!(run_face_gen(&mut e, &g));
        assert_eq!(
            calls(&e, V_RESET),
            vec![vec![g.target, 1.0f32.to_bits(), 1, 1, 1, 1, 0]]
        );
        assert_eq!(calls(&e, SET_FACE_GEN_MODE_BYTE), vec![vec![1]]);
    }

    #[test]
    fn modify_face_gen_ignores_unknown_keywords() {
        let (mut e, g) = face_gen_setup();
        parse_face_gen(&mut e, true, "nonsense", 1, 1);
        assert!(run_face_gen(&mut e, &g));
        for slot in [V_SET_EXPRESSION, V_SET_PHONEMES, V_SET_MODIFIERS, V_RESET] {
            assert!(calls(&e, slot).is_empty());
        }
        assert!(calls(&e, SET_FACE_GEN_MODE_BYTE).is_empty());
        assert!(calls(&e, PRINT_LINE).is_empty());
    }

    // ---- The face generation data ------------------------------------------------

    #[test]
    fn fn_005dd4e0_reads_a_float_array_with_an_unsigned_range_check() {
        let mut e = engine();
        let this = object(&mut e);
        let data = e.mem.alloc(16);
        for i in 0..3 {
            e.mem.set_f32(data + 4 * i, 1.5 + i as f32);
        }
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, 3);
        assert_eq!(e.call(0x005d_d4e0, &args![this, 0u32]).f32(), 1.5);
        assert_eq!(e.call(0x005d_d4e0, &args![this, 2u32]).f32(), 3.5);
        assert_eq!(e.call(0x005d_d4e0, &args![this, 3u32]).f32(), 0.0);
        // A negative index is a huge unsigned one.
        assert_eq!(e.call(0x005d_d4e0, &args![this, -1i32]).f32(), 0.0);
    }

    #[test]
    fn fn_005dd4a0_reads_the_array_at_0xa8_for_indices_below_17() {
        let mut e = engine();
        let this = object(&mut e);
        let data = e.mem.alloc(80);
        for i in 0..20 {
            e.mem.set_f32(data + 4 * i, 10.0 + i as f32);
        }
        e.mem.set_u32(this + 0xa8 + 0xc, data);
        e.mem.set_u32(this + 0xa8 + 0x10, 20);
        assert_eq!(e.call(0x005d_d4a0, &args![this, 0i32]).f32(), 10.0);
        assert_eq!(e.call(0x005d_d4a0, &args![this, 16i32]).f32(), 26.0);
        // 17 and above are zero even though the array has more.
        assert_eq!(e.call(0x005d_d4a0, &args![this, 17i32]).f32(), 0.0);
        // The signed comparison lets a negative index through to the
        // unsigned range check.
        assert_eq!(e.call(0x005d_d4a0, &args![this, -1i32]).f32(), 0.0);
    }

    #[test]
    fn fn_005dd520_and_fn_005dd540_store_single_bytes() {
        let mut e = engine();
        let this = object(&mut e);
        e.call(0x005d_d520, &args![this, 1u32]);
        e.call(0x005d_d540, &args![this, 2u32]);
        assert_eq!(e.mem.u8(this + 0xd4), 1);
        assert_eq!(e.mem.u8(this + 0xe3), 2);
        assert_eq!(e.mem.u8(this + 0xd5), 0);
    }

    #[test]
    fn tes_npc_clear_head_empties_one_pointer_and_assigns_it_to_the_other() {
        let mut e = engine();
        e.register(NI_POINTER_SET, |_, a| a[0].into_ret());
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x005d_d560, &args![0x1000u32]);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1..],
            [
                (NI_POINTER_SET, vec![0x11c4, 0]),
                (NI_POINTER_ASSIGN, vec![0x11c8, 0x11c4]),
            ]
        );
    }

    #[test]
    fn fn_005dd590_constructs_the_face_generation_data() {
        let mut e = engine();
        face_gen_data_doubles(&mut e);
        let this = e.mem.alloc(0x100);
        for offset in (0..0x100).step_by(4) {
            e.mem.set_u32(this + offset, 0xaaaa_aaaa);
        }
        start_log(&mut e);
        let returned = e.call(0x005d_d590, &args![this]).u32();
        assert_eq!(returned, this);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1..],
            [
                (
                    VECTOR_CONSTRUCT,
                    vec![this, 0x20, 4, ELEMENT_CONSTRUCT, ELEMENT_DESTRUCT]
                ),
                (ARRAY_94_CONSTRUCT, vec![this + 0x94, 0, 1]),
                (ARRAY_A4_CONSTRUCT, vec![this + 0xa4, 0, 1]),
                (ARRAY_B4_CONSTRUCT, vec![this + 0xb4, 0, 1]),
                (ARRAY_C4_CONSTRUCT, vec![this + 0xc4, 0, 1]),
                (LIST_CONSTRUCT, vec![this + 0xe4]),
                (ARRAY_RESET, vec![this + 0x94]),
                (ARRAY_RESET, vec![this + 0xa4]),
                (ARRAY_RESET, vec![this + 0xb4]),
            ]
        );
        for offset in [0x80, 0x84, 0x88, 0x8c, 0x90] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0xd4), 0);
        assert_eq!(e.mem.u8(this + 0xd5), 0);
        assert_eq!(e.mem.u32(this + 0xe0), 0xffff_ffff);
        // Fields the constructor does not touch keep their bytes.
        assert_eq!(e.mem.u32(this + 0x94), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(this + 0xd8), 0xaaaa_aaaa);
    }

    #[test]
    fn fn_005dd6f0_destroys_the_parts_in_reverse_order() {
        let mut e = engine();
        face_gen_data_doubles(&mut e);
        let this = 0x2000u32;
        start_log(&mut e);
        e.call(0x005d_d6f0, &args![this]);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1..],
            [
                (LIST_PRE_DESTRUCT, vec![this + 0xe4]),
                (LIST_DESTRUCT, vec![this + 0xe4]),
                (ARRAY_C4_DESTRUCT_WRAPPER, vec![this + 0xc4]),
                (ARRAY_B4_DESTRUCT_WRAPPER, vec![this + 0xb4]),
                (ARRAY_A4_DESTRUCT, vec![this + 0xa4]),
                (ARRAY_94_DESTRUCT, vec![this + 0x94]),
                (VECTOR_DESTRUCT, vec![this, 0x20, 4, ELEMENT_DESTRUCT]),
            ]
        );
    }

    #[test]
    fn fn_005dd7b0_and_fn_005dd7d0_call_the_array_destructors() {
        let mut e = engine();
        face_gen_data_doubles(&mut e);
        start_log(&mut e);
        e.call(0x005d_d7b0, &args![0x3000u32]);
        e.call(0x005d_d7d0, &args![0x4000u32]);
        assert_eq!(calls(&e, ARRAY_94_DESTRUCT), vec![vec![0x3000]]);
        assert_eq!(calls(&e, ARRAY_A4_DESTRUCT), vec![vec![0x4000]]);
    }

    // ---- 005dd7f0 onwards ---------------------------------------------------

    /// Registers a double that answers the same word for every call.
    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    /// Registers a double that does nothing.
    fn nothing(e: &mut Engine, address: u32) {
        e.register_double(address, |_, _| Ret::default());
    }

    /// A reference-like object: zeroed, with the given vtable slots.
    fn parsed_one(e: &mut Engine, this_obj: u32, value: u32) -> ScriptArgs {
        let a = command(e, this_obj);
        parse_gives(e, true, &[value]);
        a
    }

    #[test]
    fn fn_005dd7f0_calls_the_destructor_of_the_array_at_b4() {
        let mut e = engine();
        nothing(&mut e, ARRAY_B4_DESTRUCT);
        start_log(&mut e);
        e.call(0x005d_d7f0, &args![0x3000u32]);
        assert_eq!(calls(&e, ARRAY_B4_DESTRUCT), vec![vec![0x3000]]);
    }

    #[test]
    fn fn_005dd810_calls_the_destructor_of_the_array_at_c4() {
        let mut e = engine();
        nothing(&mut e, ARRAY_C4_DESTRUCT);
        start_log(&mut e);
        e.call(0x005d_d810, &args![0x4000u32]);
        assert_eq!(calls(&e, ARRAY_C4_DESTRUCT), vec![vec![0x4000]]);
    }

    /// The name functions read the pointer at `table + 4 * index`.
    fn check_name_function(address: u32, table: u32) {
        let mut e = engine();
        e.set_global(table, 0x0103_d000u32);
        e.set_global(table + 12, 0xabcdu32);
        assert_eq!(e.call(address, &args![0i32]).u32(), 0x0103_d000);
        assert_eq!(e.call(address, &args![3i32]).u32(), 0xabcd);
    }

    #[test]
    fn fn_005dd830_reads_the_expression_name_table() {
        check_name_function(0x005d_d830, EXPRESSION_NAME_TABLE);
    }

    #[test]
    fn fn_005dd840_reads_the_modifier_name_table() {
        check_name_function(0x005d_d840, MODIFIER_NAME_TABLE);
    }

    #[test]
    fn fn_005dd850_reads_the_phoneme_name_table() {
        check_name_function(0x005d_d850, PHONEME_NAME_TABLE);
    }

    #[test]
    fn fn_005dd860_stores_the_mode_byte() {
        let mut e = engine();
        e.call(0x005d_d860, &args![1u32]);
        assert_eq!(e.global::<u8>(FACE_GEN_MODE_BYTE), 1);
        e.call(0x005d_d860, &args![0u32]);
        assert_eq!(e.global::<u8>(FACE_GEN_MODE_BYTE), 0);
    }

    /// Doubles of the code record: the record object is `0x5000`.
    fn code_record_doubles(e: &mut Engine, method: u32, answer_byte: u32) {
        answer(e, CODE_RECORDS, 0x5000);
        answer(e, method, answer_byte);
        nothing(e, SPRINTF_S);
    }

    #[test]
    fn is_code_redeemed_prints_yes_and_stores_one() {
        let mut e = engine();
        let a = parsed_one(&mut e, 0, 0);
        code_record_doubles(&mut e, CODE_IS_REDEEMED, 1);
        start_log(&mut e);
        assert_eq!(e.call(0x005d_d870, &args![a]).u8(), 1);
        let parse_call = calls(&e, PARSE_PARAMETERS)[0].clone();
        let text = parse_call[7];
        assert_eq!(calls(&e, CODE_RECORDS), vec![Vec::<u32>::new()]);
        assert_eq!(calls(&e, CODE_IS_REDEEMED), vec![vec![0x5000, text]]);
        let sprintf = calls(&e, SPRINTF_S)[0].clone();
        assert_eq!(sprintf[1..], [200, FORMAT_IS_CODE_REDEEMED, text, TEXT_YES]);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![sprintf[0]]]);
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
    }

    #[test]
    fn is_code_redeemed_prints_no_and_stores_zero() {
        let mut e = engine();
        let a = parsed_one(&mut e, 0, 0);
        e.mem.set_f64(a.result.addr(), 9.0);
        code_record_doubles(&mut e, CODE_IS_REDEEMED, 0);
        start_log(&mut e);
        assert_eq!(e.call(0x005d_d870, &args![a]).u8(), 0);
        let sprintf = calls(&e, SPRINTF_S)[0].clone();
        assert_eq!(sprintf[4], TEXT_NO);
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
    }

    #[test]
    fn is_code_redeemed_returns_the_answer_byte_itself() {
        let mut e = engine();
        let a = parsed_one(&mut e, 0, 0);
        code_record_doubles(&mut e, CODE_IS_REDEEMED, 0x1_0002);
        assert_eq!(e.call(0x005d_d870, &args![a]).u8(), 2);
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
    }

    #[test]
    fn is_code_redeemed_does_nothing_when_the_parameters_do_not_parse() {
        let mut e = engine();
        let a = command(&mut e, 0);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert_eq!(e.call(0x005d_d870, &args![a]).u8(), 0);
        assert_eq!(call_order(&e), vec![0x005d_d870, PARSE_PARAMETERS]);
    }

    #[test]
    fn fn_005dd960_sets_the_code_and_stores_the_answer() {
        let mut e = engine();
        let a = parsed_one(&mut e, 0, 0);
        code_record_doubles(&mut e, CODE_SET_REDEEMED, 1);
        start_log(&mut e);
        assert_eq!(e.call(0x005d_d960, &args![a]).u8(), 1);
        let text = calls(&e, PARSE_PARAMETERS)[0][7];
        assert_eq!(calls(&e, CODE_SET_REDEEMED), vec![vec![0x5000, text]]);
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        code_record_doubles(&mut e, CODE_SET_REDEEMED, 0);
        assert_eq!(e.call(0x005d_d960, &args![a]).u8(), 0);
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert_eq!(e.call(0x005d_d960, &args![a]).u8(), 0);
        assert!(calls(&e, CODE_RECORDS).is_empty());
    }

    #[test]
    fn fn_005dda00_clears_the_code() {
        let mut e = engine();
        let a = parsed_one(&mut e, 0, 0);
        code_record_doubles(&mut e, CODE_CLEAR_REDEEMED, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_da00, &args![a]).bool());
        let text = calls(&e, PARSE_PARAMETERS)[0][7];
        assert_eq!(calls(&e, CODE_CLEAR_REDEEMED), vec![vec![0x5000, text]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_da00, &args![a]).bool());
        assert!(calls(&e, CODE_RECORDS).is_empty());
    }

    #[test]
    fn fn_005dda70_stores_the_actor_byte() {
        let mut e = engine();
        let this_obj = object_with(&mut e, &[(0x100, V_IS_ACTOR)]);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[]);
        answer(&mut e, V_IS_ACTOR, 1);
        answer(&mut e, ACTOR_BYTE_1B1, 0x1_07);
        start_log(&mut e);
        assert!(e.call(0x005d_da70, &args![a]).bool());
        assert_eq!(calls(&e, PARSE_PARAMETERS)[0].len(), 7);
        assert_eq!(calls(&e, ACTOR_BYTE_1B1), vec![vec![this_obj]]);
        assert_eq!(e.mem.f64(a.result.addr()), 7.0);
    }

    #[test]
    fn fn_005dda70_leaves_the_result_for_a_non_actor_and_fails_without_parameters() {
        let mut e = engine();
        let this_obj = object_with(&mut e, &[(0x100, V_IS_ACTOR)]);
        let a = command(&mut e, this_obj);
        e.mem.set_f64(a.result.addr(), 3.0);
        parse_gives(&mut e, true, &[]);
        answer(&mut e, V_IS_ACTOR, 0);
        nothing(&mut e, ACTOR_BYTE_1B1);
        start_log(&mut e);
        assert!(e.call(0x005d_da70, &args![a]).bool());
        assert!(calls(&e, ACTOR_BYTE_1B1).is_empty());
        assert_eq!(e.mem.f64(a.result.addr()), 3.0);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_da70, &args![a]).bool());
        assert!(calls(&e, V_IS_ACTOR).is_empty());
    }

    /// The doubles of the weapon mod slot command; returns `(this_obj,
    /// item)`: the extra data list sits at the first word of `item`.
    fn weapon_mod_doubles(e: &mut Engine, actor: bool) -> u32 {
        const V_PROCESS_148: u32 = 0x0900_0148;
        let process = object_with(e, &[(0x148, V_PROCESS_148)]);
        let this_obj = object_with(e, &[(0x100, V_IS_ACTOR)]);
        let item = e.mem.alloc(8);
        e.mem.set_u32(item, 0xe1e1);
        answer(e, V_IS_ACTOR, actor as u32);
        answer(e, GET_PROCESS, process);
        answer(e, V_PROCESS_148, 0x5555);
        answer(e, READ_POINTER, 0x6666);
        answer(e, LIST_ITEM_PTR, item);
        nothing(e, SET_WEAPON_MOD_SLOT);
        this_obj
    }

    #[test]
    fn fn_005ddae0_sets_the_weapon_mod_slot_bit() {
        for (slot, mask) in [(1u32, 1u32), (2, 2), (3, 4)] {
            let mut e = engine();
            let this_obj = weapon_mod_doubles(&mut e, true);
            let a = command(&mut e, this_obj);
            parse_gives(&mut e, true, &[7, slot]);
            start_log(&mut e);
            assert!(e.call(0x005d_dae0, &args![a]).bool());
            assert_eq!(calls(&e, GET_PROCESS), vec![vec![this_obj]]);
            assert_eq!(calls(&e, READ_POINTER), vec![vec![0x5555]]);
            assert_eq!(calls(&e, LIST_ITEM_PTR), vec![vec![0x6666]]);
            assert_eq!(calls(&e, SET_WEAPON_MOD_SLOT), vec![vec![0xe1e1, mask]]);
        }
    }

    #[test]
    fn fn_005ddae0_ignores_other_slots_and_non_actors() {
        for (actor, slot) in [(true, 0u32), (true, 4), (true, 0xffff_ffff), (false, 2)] {
            let mut e = engine();
            let this_obj = weapon_mod_doubles(&mut e, actor);
            let a = command(&mut e, this_obj);
            parse_gives(&mut e, true, &[7, slot]);
            start_log(&mut e);
            assert!(e.call(0x005d_dae0, &args![a]).bool());
            assert!(calls(&e, SET_WEAPON_MOD_SLOT).is_empty());
        }
        let mut e = engine();
        let this_obj = weapon_mod_doubles(&mut e, true);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_dae0, &args![a]).bool());
        assert!(calls(&e, V_IS_ACTOR).is_empty());
    }

    /// The test of the getters by index (`005ddbb0`, `005ddc20`).
    fn check_float_getter(address: u32, getter: u32) {
        let mut e = engine();
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0x1234, 1]);
        e.register_double(getter, |_, _| 2.5f64.into_ret());
        start_log(&mut e);
        assert!(e.call(address, &args![a]).bool());
        assert_eq!(calls(&e, getter), vec![vec![0x1234, 1]]);
        assert_eq!(e.mem.f64(a.result.addr()), 2.5);
        // Index 0 is allowed as well.
        parse_gives(&mut e, true, &[0x1234, 0]);
        e.mem.set_f64(a.result.addr(), 0.0);
        assert!(e.call(address, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 2.5);
        // Other indexes fail without calling the getter or touching the result.
        for bad in [2u32, 0xffff_ffff] {
            parse_gives(&mut e, true, &[0x1234, bad]);
            e.mem.set_f64(a.result.addr(), 0.0);
            start_log(&mut e);
            assert!(!e.call(address, &args![a]).bool());
            assert!(calls(&e, getter).is_empty());
            assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        }
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(address, &args![a]).bool());
    }

    #[test]
    fn fn_005ddbb0_stores_the_float_getter_result() {
        check_float_getter(0x005d_dbb0, OBJECT_FLOAT_GETTER_006155C0);
    }

    #[test]
    fn fn_005ddc20_stores_the_other_float_getter_result() {
        check_float_getter(0x005d_dc20, OBJECT_FLOAT_GETTER_00616A60);
    }

    #[test]
    fn fn_005ddc90_stores_the_signed_byte() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.register_double(OBJECT_BYTE_GETTER_00616A90, |_, _| 0x1_00fe_u32.into_ret());
        for index in 0..=2u32 {
            parse_gives(&mut e, true, &[0x1234, index]);
            e.mem.set_f64(a.result.addr(), 0.0);
            start_log(&mut e);
            assert!(e.call(0x005d_dc90, &args![a]).bool());
            assert_eq!(
                calls(&e, OBJECT_BYTE_GETTER_00616A90),
                vec![vec![0x1234, index]]
            );
            assert_eq!(e.mem.f64(a.result.addr()), -2.0);
        }
        e.register_double(OBJECT_BYTE_GETTER_00616A90, |_, _| 5u32.into_ret());
        parse_gives(&mut e, true, &[0x1234, 1]);
        assert!(e.call(0x005d_dc90, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 5.0);
        for bad in [3u32, 0xffff_ffff] {
            parse_gives(&mut e, true, &[0x1234, bad]);
            start_log(&mut e);
            assert!(!e.call(0x005d_dc90, &args![a]).bool());
            assert!(calls(&e, OBJECT_BYTE_GETTER_00616A90).is_empty());
        }
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_dc90, &args![a]).bool());
    }

    /// The test of the setters by index: `(object, index, value)` is passed
    /// on for an index of 0 or 1.
    fn check_setter(address: u32, setter: u32) {
        let mut e = engine();
        let a = command(&mut e, 0);
        nothing(&mut e, setter);
        for index in 0..=1u32 {
            parse_gives(&mut e, true, &[0x1234, index, 0x4048_0000]);
            start_log(&mut e);
            assert!(e.call(address, &args![a]).bool());
            assert_eq!(calls(&e, setter), vec![vec![0x1234, index, 0x4048_0000]]);
        }
        for bad in [2u32, 0xffff_ffff] {
            parse_gives(&mut e, true, &[0x1234, bad, 5]);
            start_log(&mut e);
            assert!(!e.call(address, &args![a]).bool());
            assert!(calls(&e, setter).is_empty());
        }
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(address, &args![a]).bool());
        assert!(calls(&e, setter).is_empty());
    }

    #[test]
    fn fn_005ddd10_sets_an_integer_by_index() {
        check_setter(0x005d_dd10, OBJECT_SET_INT_00615C90);
    }

    #[test]
    fn fn_005ddd90_sets_a_float_by_index() {
        check_setter(0x005d_dd90, OBJECT_FLOAT_SETTER_00615730);
    }

    #[test]
    fn fn_005dde10_sets_an_integer_by_index() {
        check_setter(0x005d_de10, OBJECT_SET_INT_00615FA0);
    }

    #[test]
    fn fn_005dde90_sets_a_float_by_index() {
        check_setter(0x005d_de90, OBJECT_FLOAT_SETTER_00615A00);
    }

    #[test]
    fn fn_005ddf10_sets_a_float_by_index() {
        check_setter(0x005d_df10, OBJECT_FLOAT_SETTER_00615580);
    }

    /// The array of references used by the two loops: the elements are
    /// `refs`; the array object sits at `0x7000` with its first index (the
    /// word at `+0x10`) `start`.
    fn reference_list(e: &mut Engine, start: u32, refs: Vec<u32>) {
        let list = e.mem.alloc(0x40);
        e.mem.set_u32(list + 0x10, start);
        let count = refs.len() as u32;
        answer(e, PLUS_4, list);
        answer(e, LIST_COUNT, count);
        e.register_double(LIST_GET, move |_, a| refs[a[1] as usize].into_ret());
    }

    /// References for [`fn_005ddf90`]: the field at `+0x10` is the answer of
    /// virtual slot `0x224`, the word at `+0x14` the `+0xfc` field, the word
    /// at `+0x18` the owner, the word at `+8` the flags, the word at `+0x30`
    /// the form flags of its own base form (the reference itself).
    fn loop_doubles(e: &mut Engine) {
        e.register_double(V_REFERENCE_224, |e, a| {
            (e.mem.u32(a[0] + 0x10) != 0).into_ret()
        });
        e.register_double(FLAG_200000, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x20_0000 != 0).into_ret()
        });
        e.register_double(FIELD_FC, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register_double(GET_OWNER, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register_double(REFERENCE_BASE_FORM, |_, a| a[0].into_ret());
        e.register_double(FORM_FLAG_TEST, |e, a| {
            (e.mem.u32(a[0] + 0x30) & a[1] != 0).into_ret()
        });
        e.register_double(V_REFERENCE_1A4, |_, _| 0xaaaa_u32.into_ret());
    }

    const V_REFERENCE_224: u32 = 0x0900_0224;
    const V_REFERENCE_1A4: u32 = 0x0900_01a4;
    const V_REFERENCE_318: u32 = 0x0900_0318;
    const V_REFERENCE_320: u32 = 0x0900_0320;

    fn reference(e: &mut Engine) -> u32 {
        object_with(
            e,
            &[
                (0x224, V_REFERENCE_224),
                (0x318, V_REFERENCE_318),
                (0x320, V_REFERENCE_320),
            ],
        )
    }

    #[test]
    fn fn_005ddf90_flags_the_matching_references() {
        let mut e = engine();
        let this_obj = object_with(&mut e, &[(0x1a4, V_REFERENCE_1A4)]);
        let a = command(&mut e, this_obj);
        loop_doubles(&mut e);
        // Index 0 is before the start index and never visited.
        let before = reference(&mut e);
        let skipped_by_flag = reference(&mut e);
        let skipped_by_virtual = reference(&mut e);
        let own = reference(&mut e);
        let owned = reference(&mut e);
        let foreign = reference(&mut e);
        let no_flag_on_form = reference(&mut e);
        for r in [
            before,
            skipped_by_flag,
            skipped_by_virtual,
            own,
            owned,
            foreign,
        ] {
            e.mem.set_u32(r + 0x10, 1);
            e.mem.set_u32(r + 0x30, 0x400);
        }
        e.mem.set_u32(no_flag_on_form + 0x10, 1);
        e.mem.set_u32(no_flag_on_form + 0x14, this_obj);
        e.mem.set_u32(skipped_by_flag + 8, 0x20_0000);
        e.mem.set_u32(skipped_by_virtual + 0x10, 0);
        e.mem.set_u32(own + 0x14, this_obj);
        e.mem.set_u32(owned + 0x14, 0x1234);
        e.mem.set_u32(owned + 0x18, 0xaaaa);
        e.mem.set_u32(foreign + 0x14, 0x1234);
        e.mem.set_u32(foreign + 0x18, 0xbbbb);
        reference_list(
            &mut e,
            1,
            vec![
                before,
                skipped_by_flag,
                skipped_by_virtual,
                own,
                owned,
                foreign,
                no_flag_on_form,
            ],
        );
        e.mem.set_u32(own + 0xc8, 0x8000_0001);
        assert!(e.call(0x005d_df90, &args![a]).bool());
        assert_eq!(e.mem.u32(before + 0xc8), 0);
        assert_eq!(e.mem.u32(skipped_by_flag + 0xc8), 0);
        assert_eq!(e.mem.u32(skipped_by_virtual + 0xc8), 0);
        assert_eq!(e.mem.u32(own + 0xc8), 0x8010_0001);
        assert_eq!(e.mem.u32(owned + 0xc8), 0x10_0000);
        assert_eq!(e.mem.u32(foreign + 0xc8), 0);
        assert_eq!(e.mem.u32(no_flag_on_form + 0xc8), 0);
    }

    #[test]
    fn fn_005ddf90_without_this_obj_does_nothing() {
        let mut e = engine();
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_df90, &args![a]).bool());
        assert_eq!(call_order(&e), vec![0x005d_df90]);
    }

    #[test]
    fn fn_005de080_tests_form_flag_400() {
        let mut e = engine();
        answer(&mut e, FORM_FLAG_TEST, 1);
        start_log(&mut e);
        assert!(e.call(0x005d_e080, &args![0x1000u32]).bool());
        assert_eq!(calls(&e, FORM_FLAG_TEST), vec![vec![0x1000, 0x400]]);
        answer(&mut e, FORM_FLAG_TEST, 0);
        assert!(!e.call(0x005d_e080, &args![0x1000u32]).bool());
    }

    #[test]
    fn fn_005de0a0_sets_or_clears_flag_bits() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0xc8, 0x0000_00f0);
        e.call(0x005d_e0a0, &args![this, 0x0f00_0000u32, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc8), 0x0f00_00f0);
        e.call(0x005d_e0a0, &args![this, 0x0100_0010u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0xc8), 0x0e00_00e0);
        // Any non-zero byte sets.
        e.call(0x005d_e0a0, &args![this, 1u32, 0x100u32]);
        assert_eq!(e.mem.u32(this + 0xc8), 0x0e00_00e0);
        e.call(0x005d_e0a0, &args![this, 1u32, 2u32]);
        assert_eq!(e.mem.u32(this + 0xc8), 0x0e00_00e1);
    }

    #[test]
    fn fn_005de0f0_reads_the_indexed_word() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x10, 11);
        e.mem.set_u32(this + 0x18, 33);
        assert_eq!(e.call(0x005d_e0f0, &args![this, 0u32]).u32(), 11);
        assert_eq!(e.call(0x005d_e0f0, &args![this, 2u32]).u32(), 33);
    }

    #[test]
    fn fn_005de110_calls_slot_320_on_the_matching_references() {
        let mut e = engine();
        let player = set_player(&mut e);
        let a = command(&mut e, 0);
        let both = reference(&mut e);
        let only_first = reference(&mut e);
        let neither = reference(&mut e);
        e.register_double(V_REFERENCE_224, |e, a| {
            (e.mem.u32(a[0] + 0x10) != 0).into_ret()
        });
        e.register_double(V_REFERENCE_318, |e, a| {
            (e.mem.u32(a[0] + 0x14) != 0).into_ret()
        });
        nothing(&mut e, V_REFERENCE_320);
        e.mem.set_u32(both + 0x10, 1);
        e.mem.set_u32(both + 0x14, 1);
        e.mem.set_u32(only_first + 0x10, 1);
        e.mem.set_u32(neither + 0x14, 1);
        // The start index of the array is not used by this command; null
        // elements are skipped.
        reference_list(&mut e, 3, vec![only_first, 0, both, neither]);
        start_log(&mut e);
        assert!(e.call(0x005d_e110, &args![a]).bool());
        assert_eq!(
            calls(&e, V_REFERENCE_320),
            vec![vec![both, player, 1]],
            "only the reference that passes both tests"
        );
        assert_eq!(calls(&e, V_REFERENCE_224).len(), 3);
        assert_eq!(calls(&e, V_REFERENCE_318).len(), 2);
    }

    /// The test of the two commands that store a `u16` (value 0 to 65535).
    fn check_u16_command(address: u32, global: u32) {
        let mut e = engine();
        let a = command(&mut e, 0);
        for (value, stored) in [(0u32, 0u16), (0x1234, 0x1234), (0xffff, 0xffff)] {
            e.set_global(global, 0x7777u16);
            parse_gives(&mut e, true, &[value]);
            assert!(e.call(address, &args![a]).bool());
            assert_eq!(e.global::<u16>(global), stored);
        }
        for bad in [0x1_0000u32, 0xffff_ffff] {
            e.set_global(global, 0x7777u16);
            parse_gives(&mut e, true, &[bad]);
            assert!(!e.call(address, &args![a]).bool());
            assert_eq!(e.global::<u16>(global), 0x7777);
        }
        parse_gives(&mut e, false, &[5]);
        assert!(!e.call(address, &args![a]).bool());
        assert_eq!(e.global::<u16>(global), 0x7777);
    }

    #[test]
    fn fn_005de1d0_stores_a_u16_through_the_setter() {
        check_u16_command(0x005d_e1d0, COMMAND_VALUE_011AE75C);
    }

    #[test]
    fn fn_005de240_stores_a_u16() {
        let mut e = engine();
        e.call(0x005d_e240, &args![0xbeefu32]);
        assert_eq!(e.global::<u16>(COMMAND_VALUE_011AE75C), 0xbeef);
        // Only the low 16 bits are stored.
        e.call(0x005d_e240, &args![0x1_0001u32]);
        assert_eq!(e.global::<u16>(COMMAND_VALUE_011AE75C), 1);
    }

    #[test]
    fn fn_005de250_stores_a_u16_through_the_other_setter() {
        check_u16_command(0x005d_e250, COMMAND_VALUE_011AFA9C);
    }

    #[test]
    fn fn_005de2c0_stores_a_u16() {
        let mut e = engine();
        e.call(0x005d_e2c0, &args![0xbeefu32]);
        assert_eq!(e.global::<u16>(COMMAND_VALUE_011AFA9C), 0xbeef);
    }

    #[test]
    fn set_last_ext_door_activated_stores_the_reference_and_the_world_space() {
        let mut e = engine();
        let player = set_player(&mut e);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0x4444]);
        answer(&mut e, GET_WORLD_SPACE, 0x9999);
        nothing(&mut e, SET_ROOT_MAP_SPACE);
        start_log(&mut e);
        assert!(e.call(0x005d_e2d0, &args![a]).bool());
        assert_eq!(e.mem.u32(player + PLAYER_LAST_EXT_DOOR), 0x4444);
        assert_eq!(calls(&e, GET_WORLD_SPACE), vec![vec![0x4444], vec![0x4444]]);
        assert_eq!(calls(&e, SET_ROOT_MAP_SPACE), vec![vec![0x9999]]);
    }

    #[test]
    fn set_last_ext_door_activated_without_a_world_space_or_reference() {
        let mut e = engine();
        let player = set_player(&mut e);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0x4444]);
        answer(&mut e, GET_WORLD_SPACE, 0);
        nothing(&mut e, SET_ROOT_MAP_SPACE);
        start_log(&mut e);
        assert!(e.call(0x005d_e2d0, &args![a]).bool());
        assert_eq!(e.mem.u32(player + PLAYER_LAST_EXT_DOOR), 0x4444);
        assert!(calls(&e, SET_ROOT_MAP_SPACE).is_empty());
        // A null reference is stored and nothing is asked.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_e2d0, &args![a]).bool());
        assert_eq!(e.mem.u32(player + PLAYER_LAST_EXT_DOOR), 0);
        assert!(calls(&e, GET_WORLD_SPACE).is_empty());
        // Parameters that do not parse leave the player alone.
        e.mem.set_u32(player + PLAYER_LAST_EXT_DOOR, 0x55);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_e2d0, &args![a]).bool());
        assert_eq!(e.mem.u32(player + PLAYER_LAST_EXT_DOOR), 0x55);
    }

    #[test]
    fn fn_005de350_and_fn_005de380_set_the_flag_byte_of_the_object() {
        let mut e = engine();
        let a = command(&mut e, 0);
        let object = e.mem.alloc(0x300);
        // Without the object nothing happens, but the command succeeds.
        assert!(e.call(0x005d_e350, &args![a]).bool());
        e.call(0x005d_e380, &args![0u32]);
        e.set_global(OBJECT_POINTER_011D96C0, object);
        assert!(e.call(0x005d_e350, &args![a]).bool());
        assert_eq!(e.mem.u8(object + 0x254), 1);
        e.mem.set_u8(object + 0x254, 0);
        // The `this` word is not used by the function itself.
        e.call(0x005d_e380, &args![0x1234u32]);
        assert_eq!(e.mem.u8(object + 0x254), 1);
    }

    #[test]
    fn set_task_thread_sleep_stores_and_prints() {
        let mut e = engine();
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[250]);
        start_log(&mut e);
        assert!(e.call(0x005d_e3a0, &args![a]).bool());
        assert_eq!(e.global::<u32>(TASK_THREAD_PAUSE_SLEEP), 250);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_PAUSE_SLEEP, 250]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_e3a0, &args![a]).bool());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        assert_eq!(e.global::<u32>(TASK_THREAD_PAUSE_SLEEP), 250);
    }

    #[test]
    fn fn_005de410_stores_the_pause_sleep_value() {
        let mut e = engine();
        e.call(0x005d_e410, &args![0xdead_beefu32]);
        assert_eq!(e.global::<u32>(TASK_THREAD_PAUSE_SLEEP), 0xdead_beef);
    }

    #[test]
    fn set_task_thread_update_sleep_stores_and_prints() {
        let mut e = engine();
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[30]);
        start_log(&mut e);
        assert!(e.call(0x005d_e420, &args![a]).bool());
        assert_eq!(e.global::<u32>(TASK_THREAD_UPDATE_SLEEP), 30);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_UPDATE_SLEEP, 30]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_e420, &args![a]).bool());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn fn_005de490_stores_the_update_sleep_value() {
        let mut e = engine();
        e.call(0x005d_e490, &args![0x1234_5678u32]);
        assert_eq!(e.global::<u32>(TASK_THREAD_UPDATE_SLEEP), 0x1234_5678);
    }

    const V_ENTRY_94: u32 = 0x0900_0094;

    /// A list of `entries`: each node is `[item, next]`; an item points to a
    /// holder of the object, an object has the world space of its entry in
    /// the field at `+0x1c` of the thing its slot `0x94` returns. `None` is a
    /// holder of a null object; `Some(None)` an object without entry.
    fn image_space_list(e: &mut Engine, worlds: &[Option<Option<u32>>]) -> Vec<u32> {
        let owner = e.mem.alloc(0x100);
        e.set_global(LIST_OWNER_011DEA10, owner);
        let mut nodes = vec![];
        for world in worlds {
            let node = e.mem.alloc(8);
            let object = match world {
                None => 0,
                Some(world) => {
                    let object = object_with(e, &[(0x94, V_ENTRY_94)]);
                    if let Some(world) = world {
                        let entry = e.mem.alloc(0x40);
                        e.mem.set_u32(entry + 0x1c, *world);
                        e.mem.set_u32(object + 0x10, entry);
                    }
                    object
                }
            };
            e.mem.set_u32(node, object);
            nodes.push(node);
        }
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0] + 4, pair[1]);
        }
        let head = nodes.first().copied().unwrap_or(0);
        e.register_double(LIST_HEAD_AT_6C, move |_, _| head.into_ret());
        e.register_double(LIST_ITEM_PTR, |_, a| a[0].into_ret());
        e.register_double(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register_double(NI_POINTER_COPY_CONSTRUCT, |e, a| {
            let source = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], source);
            a[0].into_ret()
        });
        e.register_double(READ_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        e.register_double(V_ENTRY_94, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register_double(FIELD_1C, |e, a| e.mem.u32(a[0] + 0x1c).into_ret());
        nothing(e, NI_POINTER_DESTRUCT);
        nodes
    }

    #[test]
    fn is_image_space_active_finds_the_world_space() {
        let mut e = engine();
        let a = command(&mut e, 0);
        image_space_list(&mut e, &[Some(Some(5)), Some(Some(9)), Some(Some(9))]);
        parse_gives(&mut e, true, &[9]);
        e.mem.set_f64(a.result.addr(), 4.0);
        start_log(&mut e);
        assert!(e.call(0x005d_e4a0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        // The walk ended at the second node.
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 2);
        assert_eq!(calls(&e, LIST_NEXT).len(), 1);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_IMAGE_SPACE_ACTIVE]]);
        // The two outputs: the world space (0) and a 1.0 float.
        assert_eq!(calls(&e, PARSE_PARAMETERS)[0].len(), 9);
    }

    #[test]
    fn is_image_space_active_reports_inactive_when_nothing_matches() {
        let mut e = engine();
        let a = command(&mut e, 0);
        image_space_list(&mut e, &[Some(Some(5)), None, Some(None), Some(Some(6))]);
        parse_gives(&mut e, true, &[9]);
        start_log(&mut e);
        assert!(e.call(0x005d_e4a0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 4);
        assert_eq!(calls(&e, NI_POINTER_COPY_CONSTRUCT).len(), 4);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_IMAGE_SPACE_NOT_ACTIVE]]
        );
    }

    #[test]
    fn is_image_space_active_with_an_empty_list_or_bad_parameters() {
        let mut e = engine();
        let a = command(&mut e, 0);
        image_space_list(&mut e, &[]);
        parse_gives(&mut e, true, &[9]);
        start_log(&mut e);
        assert!(e.call(0x005d_e4a0, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_IMAGE_SPACE_NOT_ACTIVE]]
        );
        // The result is cleared before the parameters are parsed.
        e.mem.set_f64(a.result.addr(), 4.0);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_e4a0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn fn_005de600_calls_the_pair_action_when_both_are_given() {
        let mut e = engine();
        let a = command(&mut e, 0);
        nothing(&mut e, PAIR_ACTION_00590010);
        e.mem.set_f64(a.result.addr(), 5.0);
        parse_gives(&mut e, true, &[0x10, 0x20]);
        start_log(&mut e);
        assert!(e.call(0x005d_e600, &args![a]).bool());
        assert_eq!(calls(&e, PAIR_ACTION_00590010), vec![vec![0x10, 0x20]]);
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        for pair in [[0u32, 0x20], [0x10, 0]] {
            parse_gives(&mut e, true, &pair);
            start_log(&mut e);
            assert!(e.call(0x005d_e600, &args![a]).bool());
            assert!(calls(&e, PAIR_ACTION_00590010).is_empty());
        }
        e.mem.set_f64(a.result.addr(), 5.0);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_e600, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
    }

    #[test]
    fn fn_005de670_adds_the_item_to_the_container() {
        let mut e = engine();
        let a = command(&mut e, 0);
        nothing(&mut e, ADD_ITEM_00487A00);
        parse_gives(&mut e, true, &[0x1000, 0x20, 0x1_0005]);
        start_log(&mut e);
        assert!(e.call(0x005d_e670, &args![a]).bool());
        assert_eq!(
            calls(&e, ADD_ITEM_00487A00),
            vec![vec![0x1030, 0x1000, 5, 1, 0x20, 0]]
        );
        for outs in [[0u32, 0x20, 1], [0x1000, 0, 1]] {
            parse_gives(&mut e, true, &outs);
            start_log(&mut e);
            assert!(e.call(0x005d_e670, &args![a]).bool());
            assert!(calls(&e, ADD_ITEM_00487A00).is_empty());
        }
        // A count of 0 is passed on.
        parse_gives(&mut e, true, &[0x1000, 0x20, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_e670, &args![a]).bool());
        assert_eq!(calls(&e, ADD_ITEM_00487A00).len(), 1);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_e670, &args![a]).bool());
    }

    fn leveled_list_doubles(e: &mut Engine) {
        answer(e, OPERATOR_NEW, 0x9000);
        e.register_double(CONTAINER_ITEM_EXTRA_CONSTRUCT, |_, a| a[0].into_ret());
        nothing(e, CONTAINER_ITEM_EXTRA_SET_FLOAT);
        nothing(e, ADD_ITEM_00487A00);
    }

    #[test]
    fn add_item_to_leveled_list_builds_the_extra_data_with_the_percentage() {
        let mut e = engine();
        let a = command(&mut e, 0);
        leveled_list_doubles(&mut e);
        e.mem.set_f64(a.result.addr(), 5.0);
        parse_gives(
            &mut e,
            true,
            &[0x1000, 0x20, 0x1_0003, 0x2_0004, 50.0f32.to_bits()],
        );
        start_log(&mut e);
        assert!(e.call(0x005d_e700, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![12]]);
        assert_eq!(
            calls(&e, CONTAINER_ITEM_EXTRA_CONSTRUCT),
            vec![vec![0x9000]]
        );
        assert_eq!(
            calls(&e, CONTAINER_ITEM_EXTRA_SET_FLOAT),
            vec![vec![0x9000, 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, ADD_ITEM_00487A00),
            vec![vec![0x1030, 0x1000, 3, 4, 0x20, 0x9000]]
        );
        // The parsed outputs: four words and the percentage, 100.0 by default.
        assert_eq!(calls(&e, PARSE_PARAMETERS)[0].len(), 12);
    }

    #[test]
    fn add_item_to_leveled_list_defaults_to_one_hundred_percent() {
        let mut e = engine();
        let a = command(&mut e, 0);
        leveled_list_doubles(&mut e);
        e.set_global(DEFAULT_PERCENT, 100.0f32);
        parse_gives(&mut e, true, &[0x1000, 0x20, 1, 2]);
        start_log(&mut e);
        assert!(e.call(0x005d_e700, &args![a]).bool());
        assert_eq!(
            calls(&e, CONTAINER_ITEM_EXTRA_SET_FLOAT),
            vec![vec![0x9000, 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn add_item_to_leveled_list_needs_the_list_item_and_second_number() {
        let mut e = engine();
        let a = command(&mut e, 0);
        leveled_list_doubles(&mut e);
        for outs in [[0u32, 0x20, 1, 2], [0x1000, 0, 1, 2], [0x1000, 0x20, 1, 0]] {
            parse_gives(&mut e, true, &outs);
            start_log(&mut e);
            assert!(e.call(0x005d_e700, &args![a]).bool());
            assert!(calls(&e, OPERATOR_NEW).is_empty());
            assert!(calls(&e, ADD_ITEM_00487A00).is_empty());
        }
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_e700, &args![a]).bool());
    }

    #[test]
    fn add_item_to_leveled_list_passes_a_null_extra_when_the_allocation_fails() {
        let mut e = engine();
        let a = command(&mut e, 0);
        leveled_list_doubles(&mut e);
        answer(&mut e, OPERATOR_NEW, 0);
        parse_gives(&mut e, true, &[0x1000, 0x20, 1, 2]);
        start_log(&mut e);
        assert!(e.call(0x005d_e700, &args![a]).bool());
        assert!(calls(&e, CONTAINER_ITEM_EXTRA_CONSTRUCT).is_empty());
        assert_eq!(
            calls(&e, ADD_ITEM_00487A00),
            vec![vec![0x1030, 0x1000, 1, 2, 0x20, 0]]
        );
    }

    /// A form with the given type byte at `+4`.
    fn form_of_type(e: &mut Engine, form_type: u8) -> u32 {
        let form = e.mem.alloc(0x20);
        e.mem.set_u8(form + 4, form_type);
        form
    }

    fn water_doubles(e: &mut Engine) {
        e.register_double(FORM_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
        nothing(e, LOG_STUB);
        nothing(e, WATER_REMAP_00580680);
    }

    #[test]
    fn remap_water_type_calls_the_remap_for_two_water_types() {
        let mut e = engine();
        let a = command(&mut e, 0);
        water_doubles(&mut e);
        let from = form_of_type(&mut e, 0x4e);
        let to = form_of_type(&mut e, 0x4e);
        e.mem.set_f64(a.result.addr(), 3.0);
        parse_gives(&mut e, true, &[from, to]);
        start_log(&mut e);
        assert!(e.call(0x005d_e840, &args![a]).bool());
        assert_eq!(calls(&e, WATER_REMAP_00580680), vec![vec![from, to]]);
        assert!(calls(&e, LOG_STUB).is_empty());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
    }

    #[test]
    fn remap_water_type_reports_each_invalid_type() {
        let mut e = engine();
        let a = command(&mut e, 0);
        water_doubles(&mut e);
        let water = form_of_type(&mut e, 0x4e);
        let other = form_of_type(&mut e, 0x10);
        parse_gives(&mut e, true, &[other, water]);
        start_log(&mut e);
        assert!(e.call(0x005d_e840, &args![a]).bool());
        assert_eq!(calls(&e, LOG_STUB), vec![vec![MSG_WATER_TO_REMAP_INVALID]]);
        assert!(calls(&e, WATER_REMAP_00580680).is_empty());
        parse_gives(&mut e, true, &[water, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_e840, &args![a]).bool());
        assert_eq!(calls(&e, LOG_STUB), vec![vec![MSG_WATER_REMAP_TO_INVALID]]);
        assert!(calls(&e, WATER_REMAP_00580680).is_empty());
        parse_gives(&mut e, true, &[0, other]);
        start_log(&mut e);
        assert!(e.call(0x005d_e840, &args![a]).bool());
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![
                vec![MSG_WATER_TO_REMAP_INVALID],
                vec![MSG_WATER_REMAP_TO_INVALID]
            ]
        );
    }

    #[test]
    fn remap_water_type_fails_when_the_parameters_do_not_parse() {
        let mut e = engine();
        let a = command(&mut e, 0);
        water_doubles(&mut e);
        e.mem.set_f64(a.result.addr(), 3.0);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_e840, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert!(calls(&e, LOG_STUB).is_empty());
    }

    // ---- 005de900 onwards ---------------------------------------------------

    const V_FLAG_FC: u32 = 0x0900_00fc;
    const V_FLAG_F0: u32 = 0x0900_00f0;
    const V_SUBOBJECT_VALUE: u32 = 0x0900_000c;
    const V_PLAYER_3A4: u32 = 0x0900_03a4;
    const V_SETTING_20: u32 = 0x0900_0020;
    const V_SETTING_1C: u32 = 0x0900_001c;
    const V_SETTING_24: u32 = 0x0900_0024;

    /// A result with only `ST0` set.
    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// A node of a singly linked list: `item` at `+0`, `next` at `+4`.
    fn list_node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    /// Doubles for the list accessors: the item cell of a node is the node,
    /// the next node is at `+4`.
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_ITEM_PTR, |_, a| a[0].into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
    }

    fn bool_double(e: &mut Engine, address: u32, value: bool) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    /// A player object big enough for the fields past `+0xe38`.
    fn big_player(e: &mut Engine) -> u32 {
        let player = e.mem.alloc(0x1000);
        e.set_global(PLAYER, player);
        player
    }

    #[test]
    fn fn_005de900_returns_the_hardcore_condition() {
        check_condition_only(0x005d_e900, IS_HARDCORE_CONDITION);
    }

    #[test]
    fn fn_005dea80_returns_the_always_hardcore_condition() {
        check_condition_only(0x005d_ea80, IS_ALWAYS_HARDCORE_CONDITION);
    }

    #[test]
    fn set_hardcore_sets_the_mode_and_clears_the_byte_unless_it_is_1() {
        let mut e = engine();
        let player = big_player(&mut e);
        let a = command(&mut e, 0);
        nothing(&mut e, PLAYER_SET_HARDCORE);
        nothing(&mut e, LOG_STUB);
        e.mem.set_u8(player + 0xe38, 7);
        parse_gives(&mut e, true, &[1]);
        start_log(&mut e);
        assert!(e.call(0x005d_e920, &args![a]).bool());
        assert_eq!(calls(&e, PLAYER_SET_HARDCORE), vec![vec![player, 1, 0]]);
        assert_eq!(e.mem.u8(player + 0xe38), 7);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_e920, &args![a]).bool());
        assert_eq!(calls(&e, PLAYER_SET_HARDCORE), vec![vec![player, 0, 0]]);
        assert_eq!(e.mem.u8(player + 0xe38), 0);
        assert!(calls(&e, LOG_STUB).is_empty());
    }

    #[test]
    fn set_hardcore_reports_a_value_out_of_range_or_a_missing_player() {
        let mut e = engine();
        let player = big_player(&mut e);
        let a = command(&mut e, 0);
        nothing(&mut e, PLAYER_SET_HARDCORE);
        nothing(&mut e, LOG_STUB);
        for value in [2u32, 0xffff_ffff] {
            parse_gives(&mut e, true, &[value]);
            start_log(&mut e);
            assert!(!e.call(0x005d_e920, &args![a]).bool());
            assert_eq!(calls(&e, LOG_STUB), vec![vec![MSG_SET_HARDCORE_INVALID]]);
            assert!(calls(&e, PLAYER_SET_HARDCORE).is_empty());
        }
        e.set_global(PLAYER, 0u32);
        parse_gives(&mut e, true, &[1]);
        start_log(&mut e);
        assert!(!e.call(0x005d_e920, &args![a]).bool());
        assert_eq!(calls(&e, LOG_STUB).len(), 1);
        assert!(calls(&e, PLAYER_SET_HARDCORE).is_empty());
        // A parameter that does not parse: failure without a report.
        e.set_global(PLAYER, player);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_e920, &args![a]).bool());
        assert!(calls(&e, LOG_STUB).is_empty());
    }

    #[test]
    fn fn_005de9b0_clears_the_byte_at_0xe38() {
        let mut e = engine();
        let object = e.mem.alloc(0x1000);
        e.mem.set_u8(object + 0xe38, 5);
        e.call(0x005d_e9b0, &args![object]);
        assert_eq!(e.mem.u8(object + 0xe38), 0);
    }

    #[test]
    fn fn_005de9d0_clears_the_players_byte_when_there_is_a_player() {
        let mut e = engine();
        let a = command(&mut e, 0);
        assert!(e.call(0x005d_e9d0, &args![a]).bool());
        let player = big_player(&mut e);
        e.mem.set_u8(player + 0xe38, 5);
        assert!(e.call(0x005d_e9d0, &args![a]).bool());
        assert_eq!(e.mem.u8(player + 0xe38), 0);
    }

    #[test]
    fn reset_xp_takes_the_value_back_to_zero_through_the_player() {
        let mut e = engine();
        let player = object_with(&mut e, &[(0x3a4, V_PLAYER_3A4)]);
        e.set_global(PLAYER, player);
        let sub_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(sub_vtable + 0xc, V_SUBOBJECT_VALUE);
        e.mem.set_u32(player + 0xa4, sub_vtable);
        e.register(V_SUBOBJECT_VALUE, |_, _| st0(12.5));
        nothing(&mut e, V_PLAYER_3A4);
        nothing(&mut e, LOG_STUB);
        let a = command(&mut e, 0);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(e.call(0x005d_e9f0, &args![a]).bool());
        // The result of ParseParameters does not matter; no outputs.
        assert_eq!(calls(&e, PARSE_PARAMETERS)[0].len(), 7);
        assert_eq!(
            calls(&e, V_SUBOBJECT_VALUE),
            vec![vec![player + 0xa4, 0x18]]
        );
        assert_eq!(
            calls(&e, V_PLAYER_3A4),
            vec![vec![player, 0x18, (-12.5f32).to_bits(), 0]]
        );
    }

    #[test]
    fn reset_xp_reports_a_missing_player() {
        let mut e = engine();
        let a = command(&mut e, 0);
        nothing(&mut e, LOG_STUB);
        parse_gives(&mut e, true, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_e9f0, &args![a]).bool());
        assert_eq!(calls(&e, LOG_STUB), vec![vec![MSG_RESET_XP_INVALID]]);
        assert_eq!(call_order(&e).len(), 3);
    }

    #[test]
    fn fn_005deaa0_puts_the_form_id_of_the_actors_record_in_the_result() {
        let mut e = engine();
        let actor = object_with(&mut e, &[(0x100, V_IS_ACTOR)]);
        let a = command(&mut e, actor);
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        let form = e.mem.alloc(0x20);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record + 4, form);
        e.register_double(ACTOR_RECORD, move |_, _| record.into_ret());
        e.register(FORM_ID, |_, _| 0xabu32.into_ret());
        // The double stores the id it is given through the slot pointer.
        e.register(PUT_NUMERIC_ID_IN_DOUBLE, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_f64(a[1], id as f64);
            Ret::default()
        });
        e.mem.set_f64(a.result.addr(), 3.0);
        start_log(&mut e);
        assert!(e.call(0x005d_eaa0, &args![a]).bool());
        assert_eq!(calls(&e, ACTOR_RECORD), vec![vec![actor]]);
        assert_eq!(calls(&e, FORM_ID), vec![vec![form]]);
        assert_eq!(e.mem.f64(a.result.addr()), 171.0);
    }

    #[test]
    fn fn_005deaa0_leaves_zero_without_an_actor_record_or_form() {
        let mut e = engine();
        let actor = object_with(&mut e, &[(0x100, V_IS_ACTOR)]);
        let a = command(&mut e, actor);
        let record = e.mem.alloc(8);
        e.register_double(ACTOR_RECORD, move |_, _| record.into_ret());
        nothing(&mut e, FORM_ID);
        nothing(&mut e, PUT_NUMERIC_ID_IN_DOUBLE);
        // Not an actor.
        e.register(V_IS_ACTOR, |_, _| false.into_ret());
        e.mem.set_f64(a.result.addr(), 3.0);
        start_log(&mut e);
        assert!(e.call(0x005d_eaa0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert!(calls(&e, ACTOR_RECORD).is_empty());
        // An actor whose record has no form.
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_eaa0, &args![a]).bool());
        assert_eq!(calls(&e, ACTOR_RECORD).len(), 1);
        assert!(calls(&e, FORM_ID).is_empty());
        // No object at all.
        let none = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_eaa0, &args![none]).bool());
        assert!(calls(&e, ACTOR_RECORD).is_empty());
    }

    /// The doubles of the reference lookup of `005deb10`.
    fn reference_doubles(e: &mut Engine) {
        e.register(GET_BASE_FORM, |_, a| (a[0] + 0x1000).into_ret());
        e.register(FIELD_6C, |_, a| (a[0] + 0x2000).into_ret());
        e.register(FIELD_90, |_, a| (a[0] + 0x10).into_ret());
        nothing(e, SHOW_MENU_FOR_00704FC0);
    }

    #[test]
    fn fn_005deb10_uses_the_talking_activator_of_the_base_form() {
        let mut e = engine();
        reference_doubles(&mut e);
        let this_obj = object_with(&mut e, &[(0xfc, V_FLAG_FC), (0x100, V_IS_ACTOR)]);
        let a = command(&mut e, this_obj);
        e.register(V_FLAG_FC, |_, _| false.into_ret());
        e.register(DYNAMIC_CAST, |_, _| 0x7000u32.into_ret());
        parse_gives(&mut e, false, &[5]);
        start_log(&mut e);
        assert!(e.call(0x005d_eb10, &args![a]).bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![
                this_obj + 0x1000,
                0,
                RTTI_TES_BOUND_OBJECT,
                RTTI_BGS_TALKING_ACTIVATOR,
                0
            ]]
        );
        // The value is used whatever ParseParameters returned.
        assert_eq!(calls(&e, SHOW_MENU_FOR_00704FC0), vec![vec![0x7010, 5]]);
        // The cast failing: nothing is shown.
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_eb10, &args![a]).bool());
        assert!(calls(&e, FIELD_90).is_empty());
        assert!(calls(&e, SHOW_MENU_FOR_00704FC0).is_empty());
    }

    #[test]
    fn fn_005deb10_uses_an_actor_candidate_itself() {
        let mut e = engine();
        reference_doubles(&mut e);
        let this_obj = object_with(&mut e, &[(0xfc, V_FLAG_FC), (0x100, V_IS_ACTOR)]);
        let a = command(&mut e, this_obj);
        e.register(V_FLAG_FC, |_, _| true.into_ret());
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        parse_gives(&mut e, true, &[9]);
        start_log(&mut e);
        assert!(e.call(0x005d_eb10, &args![a]).bool());
        assert_eq!(calls(&e, SHOW_MENU_FOR_00704FC0), vec![vec![this_obj, 9]]);
        assert!(calls(&e, DYNAMIC_CAST).is_empty());
    }

    #[test]
    fn fn_005deb10_follows_the_base_form_of_a_candidate_with_the_flag_byte() {
        let mut e = engine();
        reference_doubles(&mut e);
        let this_obj = object_with(&mut e, &[(0xfc, V_FLAG_FC), (0x100, V_IS_ACTOR)]);
        let a = command(&mut e, this_obj);
        e.register(V_FLAG_FC, |_, _| true.into_ret());
        e.register(V_IS_ACTOR, |_, _| false.into_ret());
        parse_gives(&mut e, true, &[9]);
        bool_double(&mut e, BYTE_AT_81, true);
        start_log(&mut e);
        assert!(e.call(0x005d_eb10, &args![a]).bool());
        // *(candidate + 0x6c), its base form, then the field at +0x90.
        let inner = this_obj + 0x2000;
        let base = inner + 0x1000;
        assert_eq!(
            calls(&e, SHOW_MENU_FOR_00704FC0),
            vec![vec![base + 0x10, 9]]
        );
        // Without the flag byte nothing is shown.
        bool_double(&mut e, BYTE_AT_81, false);
        start_log(&mut e);
        assert!(e.call(0x005d_eb10, &args![a]).bool());
        assert!(calls(&e, SHOW_MENU_FOR_00704FC0).is_empty());
    }

    #[test]
    fn fn_005deb10_does_nothing_without_this_obj() {
        let mut e = engine();
        reference_doubles(&mut e);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[9]);
        start_log(&mut e);
        assert!(e.call(0x005d_eb10, &args![a]).bool());
        assert_eq!(call_order(&e), vec![0x005d_eb10, PARSE_PARAMETERS]);
    }

    /// A player with a list of entries `(form id, value)` and a form with the
    /// given id and limit; returns the form and the entries.
    fn quarter_world(
        e: &mut Engine,
        form_id: u32,
        limit: i32,
        entries: &[(u32, i32)],
    ) -> (u32, Vec<u32>) {
        set_player(e);
        e.set_global(QUARTER, 0.25f64);
        list_doubles(e);
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(FORM_BLOCK_1E8, |_, a| (a[0] + 0x1e8).into_ret());
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            (value as i32 as u32).into_ret()
        });
        let form = e.mem.alloc(0x400);
        e.mem.set_u32(form + 0xc, form_id);
        e.mem.set_i32(form + 0x1e8 + 0x28, limit);
        let mut next = 0;
        let mut records = vec![];
        for (id, value) in entries.iter().rev() {
            let record = e.mem.alloc(12);
            e.mem.set_u32(record, *id);
            e.mem.set_i32(record + 4, *value);
            // The id 0xffff_ffff stands for a node whose item is null.
            let item = if *id == 0xffff_ffff { 0 } else { record };
            next = list_node(e, item, next);
            records.insert(0, record);
        }
        e.register_double(PLAYER_LIST_HEAD, move |_, _| next.into_ret());
        (form, records)
    }

    #[test]
    fn fn_005dec30_counts_the_quarters_of_the_limit_reached() {
        for (value, expected) in [(10, 0.0), (25, 1.0), (50, 2.0), (99, 3.0), (100, 4.0)] {
            let mut e = engine();
            let (form, _) = quarter_world(&mut e, 0x55, 100, &[(1, 90), (0x55, value)]);
            let a = command(&mut e, 0);
            parse_gives(&mut e, true, &[form]);
            e.mem.set_f64(a.result.addr(), 9.0);
            assert!(e.call(0x005d_ec30, &args![a]).bool());
            assert_eq!(e.mem.f64(a.result.addr()), expected, "value {value}");
        }
    }

    #[test]
    fn fn_005dec30_leaves_the_result_without_a_matching_entry() {
        let mut e = engine();
        let (form, _) = quarter_world(&mut e, 0x55, 100, &[(1, 90)]);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[form]);
        e.mem.set_f64(a.result.addr(), 9.0);
        assert!(e.call(0x005d_ec30, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 9.0);
        // The walk ends at an entry cell that is null.
        let (form, _) = quarter_world(&mut e, 0x55, 100, &[(1, 90), (0xffff_ffff, 0), (0x55, 100)]);
        parse_gives(&mut e, true, &[form]);
        assert!(e.call(0x005d_ec30, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 9.0);
    }

    #[test]
    fn fn_005dec30_fails_without_a_form() {
        let mut e = engine();
        quarter_world(&mut e, 0x55, 100, &[]);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0]);
        assert!(!e.call(0x005d_ec30, &args![a]).bool());
    }

    #[test]
    fn fn_005ded40_stores_the_level_and_the_scaled_value() {
        let mut e = engine();
        let (form, records) = quarter_world(&mut e, 0x55, 100, &[(1, 90), (0x55, 5)]);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[form, 3]);
        assert!(e.call(0x005d_ed40, &args![a]).bool());
        assert_eq!(e.mem.u16(records[1] + 8), 3);
        assert_eq!(e.mem.i32(records[1] + 4), 75);
        // The other entry is untouched.
        assert_eq!(e.mem.i32(records[0] + 4), 90);
        assert_eq!(e.mem.u16(records[0] + 8), 0);
    }

    #[test]
    fn fn_005ded40_ignores_a_level_out_of_range() {
        for level in [5u32, 0xffff_ffff] {
            let mut e = engine();
            let (form, records) = quarter_world(&mut e, 0x55, 100, &[(0x55, 5)]);
            let a = command(&mut e, 0);
            parse_gives(&mut e, true, &[form, level]);
            assert!(e.call(0x005d_ed40, &args![a]).bool());
            assert_eq!(e.mem.i32(records[0] + 4), 5);
        }
        let mut e = engine();
        let (form, records) = quarter_world(&mut e, 0x55, 100, &[(0x55, 5)]);
        let a = command(&mut e, 0);
        // Level 0 is allowed: value 0.
        parse_gives(&mut e, true, &[form, 0]);
        assert!(e.call(0x005d_ed40, &args![a]).bool());
        assert_eq!(e.mem.i32(records[0] + 4), 0);
        // No form: failure.
        parse_gives(&mut e, true, &[0, 2]);
        assert!(!e.call(0x005d_ed40, &args![a]).bool());
    }

    #[test]
    fn fn_005dee30_calls_the_action_with_the_parsed_object() {
        let mut e = engine();
        nothing(&mut e, OBJECT_ACTION_005F64F0);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0x44]);
        start_log(&mut e);
        assert!(e.call(0x005d_ee30, &args![a]).bool());
        assert_eq!(calls(&e, OBJECT_ACTION_005F64F0), vec![vec![0x44]]);
        for (ok, object) in [(true, 0u32), (false, 0)] {
            parse_gives(&mut e, ok, &[object]);
            start_log(&mut e);
            assert!(!e.call(0x005d_ee30, &args![a]).bool());
            assert!(calls(&e, OBJECT_ACTION_005F64F0).is_empty());
        }
    }

    #[test]
    fn fn_005dee90_stores_the_first_flag_bit() {
        let mut e = engine();
        e.register(FLAG_BIT_1, |_, a| (a[0] & 1).into_ret());
        let a = command(&mut e, 0);
        e.mem.set_f64(a.result.addr(), 5.0);
        parse_gives(&mut e, true, &[0x41]);
        assert!(e.call(0x005d_ee90, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        parse_gives(&mut e, true, &[0x40]);
        assert!(e.call(0x005d_ee90, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        // No object: failure, result untouched.
        e.mem.set_f64(a.result.addr(), 5.0);
        parse_gives(&mut e, true, &[0]);
        assert!(!e.call(0x005d_ee90, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 5.0);
    }

    #[test]
    fn fn_005deef0_is_set_by_bit_2_or_bit_4() {
        let mut e = engine();
        e.register(FLAG_BIT_2, |e, a| (e.mem.u32(a[0] + 0x70) & 2).into_ret());
        let a = command(&mut e, 0);
        let object = e.mem.alloc(0x100);
        parse_gives(&mut e, true, &[object]);
        for (flags, expected) in [(0u32, 0.0), (2, 1.0), (4, 1.0), (8, 0.0), (6, 1.0)] {
            e.mem.set_u32(object + 0x70, flags);
            e.mem.set_f64(a.result.addr(), 5.0);
            assert!(e.call(0x005d_eef0, &args![a]).bool());
            assert_eq!(e.mem.f64(a.result.addr()), expected, "flags {flags}");
        }
        parse_gives(&mut e, true, &[0]);
        assert!(!e.call(0x005d_eef0, &args![a]).bool());
    }

    #[test]
    fn fn_005def70_masks_bit_4_of_the_flags() {
        let mut e = engine();
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0x70, 0xff);
        assert_eq!(e.call(0x005d_ef70, &args![object]).u32(), 4);
        e.mem.set_u32(object + 0x70, 0xfb);
        assert_eq!(e.call(0x005d_ef70, &args![object]).u32(), 0);
    }

    #[test]
    fn fn_005def90_sets_bit_8_of_the_parsed_object() {
        let mut e = engine();
        nothing(&mut e, OBJECT_FLAGS_CHANGED);
        let a = command(&mut e, 0);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0x70, 1);
        parse_gives(&mut e, true, &[object]);
        start_log(&mut e);
        assert!(e.call(0x005d_ef90, &args![a]).bool());
        assert_eq!(e.mem.u32(object + 0x70), 9);
        assert_eq!(calls(&e, OBJECT_FLAGS_CHANGED), vec![vec![object]]);
        parse_gives(&mut e, true, &[0]);
        assert!(!e.call(0x005d_ef90, &args![a]).bool());
    }

    #[test]
    fn fn_005defe0_sets_or_clears_bit_8_and_notifies() {
        let mut e = engine();
        nothing(&mut e, OBJECT_FLAGS_CHANGED);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0x70, 0x1);
        start_log(&mut e);
        e.call(0x005d_efe0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object + 0x70), 0x9);
        e.call(0x005d_efe0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object + 0x70), 0x1);
        assert_eq!(calls(&e, OBJECT_FLAGS_CHANGED), vec![vec![object]; 2]);
    }

    #[test]
    fn fn_005df510_and_fn_005df560_copy_the_triple_of_an_element() {
        let mut e = engine();
        let array = e.mem.alloc(0x100);
        let source = e.mem.alloc(12);
        for (lane, value) in [1.5f32, -2.0, 3.25].into_iter().enumerate() {
            e.mem.set_f32(source + 4 * lane as u32, value);
        }
        e.call(0x005d_f510, &args![array, 2i32, source]);
        assert_eq!(e.mem.f32(array + 24), 1.5);
        assert_eq!(e.mem.f32(array + 28), -2.0);
        assert_eq!(e.mem.f32(array + 32), 3.25);
        assert_eq!(e.mem.f32(array), 0.0);
        let out = e.mem.alloc(12);
        e.call(0x005d_f560, &args![array, 2i32, out, out + 4, out + 8]);
        assert_eq!(
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
            [1.5, -2.0, 3.25]
        );
    }

    #[test]
    fn fn_005df5b0_adds_0xde0() {
        let mut e = engine();
        assert_eq!(e.call(0x005d_f5b0, &args![0x1000u32]).u32(), 0x1de0);
    }

    #[test]
    fn fn_005dfa10_passes_this_to_the_dlc_system_utility() {
        let mut e = engine();
        e.register(DLC_SYSTEM_UTILITY_INSTANCE, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x005d_fa10, &args![0x500u32]).u32(), 0x501);
    }

    /// The doubles of the DLC singleton chain.
    fn dlc_doubles(e: &mut Engine, list: u32) {
        e.register(SYSTEM_UTILITY_INSTANCE, |_, _| 0x500u32.into_ret());
        e.register(DLC_SYSTEM_UTILITY_INSTANCE, |_, a| {
            (a[0] + 0x100).into_ret()
        });
        e.register_double(DLC_LIST_HEAD, move |_, a| {
            assert_eq!(a, [0x600]);
            list.into_ret()
        });
    }

    #[test]
    fn get_dlc_count_prints_the_number_of_packages() {
        let mut e = engine();
        dlc_doubles(&mut e, 0x7000);
        e.register(LIST_COUNT_NON_NULL, |_, a| {
            assert_eq!(a, [0x7000]);
            3u32.into_ret()
        });
        nothing(&mut e, SPRINTF_S);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[]);
        start_log(&mut e);
        assert!(e.call(0x005d_f930, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 3.0);
        assert_eq!(calls(&e, PARSE_PARAMETERS)[0].len(), 8);
        let sprintf = calls(&e, SPRINTF_S);
        assert_eq!(sprintf.len(), 1);
        assert_eq!(sprintf[0][1..], [0x40, MSG_DLC_COUNT, 3]);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![sprintf[0][0]]]);
    }

    #[test]
    fn get_dlc_count_is_zero_without_a_list_and_fails_when_the_parse_fails() {
        let mut e = engine();
        dlc_doubles(&mut e, 0);
        nothing(&mut e, SPRINTF_S);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[]);
        e.mem.set_f64(a.result.addr(), 4.0);
        start_log(&mut e);
        assert!(e.call(0x005d_f930, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert_eq!(calls(&e, SPRINTF_S)[0][1..], [0x40, MSG_DLC_COUNT, 0]);
        parse_gives(&mut e, false, &[]);
        e.mem.set_f64(a.result.addr(), 4.0);
        start_log(&mut e);
        assert!(!e.call(0x005d_f930, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert!(calls(&e, SPRINTF_S).is_empty());
        assert!(calls(&e, SYSTEM_UTILITY_INSTANCE).is_empty());
    }

    /// Doubles of `IsDLCInstalled`: the parser writes `name`; the packages are
    /// objects whose first word is the address of their (narrow, here) name.
    fn dlc_installed_world(e: &mut Engine, packages: &[Option<&str>]) -> u32 {
        list_doubles(e);
        e.register(DLC_PACKAGE_TAG, |_, _| 0xaaaau32.into_ret());
        e.register(DLC_PACKAGE_NAME, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(WCSTOMBS_S, |e, a| {
            let text = e.mem.cstr(a[3]);
            e.mem.write(a[1], &text);
            e.mem.set_u8(a[1] + text.len() as u32, 0);
            Ret::default()
        });
        e.register(STRICMP, |e, a| {
            let (first, second) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            (!first.eq_ignore_ascii_case(&second) as u32).into_ret()
        });
        nothing(e, SPRINTF_S);
        e.register(PARSE_PARAMETERS, |e, a| {
            e.mem.write(a[7], b"NVDLC03\0");
            true.into_ret()
        });
        let mut next = 0;
        for package in packages.iter().rev() {
            let item = match package {
                Some(name) => {
                    let text = e.mem.alloc(32);
                    e.mem.write(text, name.as_bytes());
                    let object = e.mem.alloc(8);
                    e.mem.set_u32(object, text);
                    object
                }
                None => 0,
            };
            next = list_node(e, item, next);
        }
        next
    }

    #[test]
    fn is_dlc_installed_finds_the_package_ignoring_case() {
        let mut e = engine();
        let list = dlc_installed_world(&mut e, &[Some("nvdlc01"), None, Some("nvdlc03")]);
        dlc_doubles(&mut e, list);
        let a = command(&mut e, 0);
        e.mem.set_f64(a.result.addr(), 5.0);
        start_log(&mut e);
        assert!(e.call(0x005d_fa20, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        let sprintf = calls(&e, SPRINTF_S);
        // The "Checking" line, then one "Examining" line per package with an
        // item (the null one is skipped).
        assert_eq!(sprintf.len(), 3);
        assert_eq!(sprintf[0][1..3], [200, MSG_CHECKING_DLC]);
        assert_eq!(sprintf[1][1..3], [200, MSG_EXAMINING_DLC]);
        assert_eq!(sprintf[1][4], 0xaaaa);
        assert_eq!(calls(&e, STRICMP).len(), 2);
        assert_eq!(calls(&e, WCSTOMBS_S).len(), 2);
        let words = &calls(&e, WCSTOMBS_S)[0];
        assert_eq!([words[2], words[4]], [200, 200]);
    }

    #[test]
    fn is_dlc_installed_is_zero_without_a_match_and_on_an_empty_list() {
        let mut e = engine();
        let list = dlc_installed_world(&mut e, &[Some("nvdlc01"), Some("nvdlc02")]);
        dlc_doubles(&mut e, list);
        let a = command(&mut e, 0);
        e.mem.set_f64(a.result.addr(), 5.0);
        assert!(e.call(0x005d_fa20, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        dlc_doubles(&mut e, 0);
        e.mem.set_f64(a.result.addr(), 5.0);
        start_log(&mut e);
        assert!(e.call(0x005d_fa20, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert_eq!(calls(&e, SPRINTF_S).len(), 1);
    }

    #[test]
    fn is_dlc_installed_fails_when_the_name_does_not_parse() {
        let mut e = engine();
        dlc_doubles(&mut e, 0);
        parse_gives(&mut e, false, &[]);
        let a = command(&mut e, 0);
        e.mem.set_f64(a.result.addr(), 5.0);
        start_log(&mut e);
        assert!(!e.call(0x005d_fa20, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert!(calls(&e, SYSTEM_UTILITY_INSTANCE).is_empty());
    }

    #[test]
    fn fn_005dfba0_passes_whether_the_value_is_above_zero() {
        let mut e = engine();
        nothing(&mut e, SET_FLAG_00457D70);
        e.set_global(LIST_OWNER_011DEA10, 0x7000u32);
        let a = command(&mut e, 0);
        for (value, flag) in [(1u32, 1u32), (5, 1), (0, 0), (0xffff_ffff, 0)] {
            parse_gives(&mut e, true, &[value]);
            start_log(&mut e);
            assert!(e.call(0x005d_fba0, &args![a]).bool());
            assert_eq!(calls(&e, SET_FLAG_00457D70), vec![vec![0x7000, flag, 0, 1]]);
        }
        parse_gives(&mut e, false, &[]);
        e.mem.set_f64(a.result.addr(), 3.0);
        start_log(&mut e);
        assert!(!e.call(0x005d_fba0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert!(calls(&e, SET_FLAG_00457D70).is_empty());
    }

    #[test]
    fn is_loading_menu_enabled_reports_whether_the_menu_is_open() {
        let mut e = engine();
        let a = command(&mut e, 0);
        // Only the low byte counts.
        e.register(IS_IN_GAME_LOADING_MENU_OPEN, |_, _| 0x100u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_fc10, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        e.register(IS_IN_GAME_LOADING_MENU_OPEN, |_, _| 0x1u32.into_ret());
        assert!(e.call(0x005d_fc10, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_fc10, &args![a]).bool());
        let one = 1.0f64.to_bits();
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![
                MSG_LOADING_MENU_ENABLED,
                one as u32,
                (one >> 32) as u32
            ]]
        );
    }

    #[test]
    fn is_actor_ref_essential_reports_the_actors_flag() {
        let mut e = engine();
        let this_obj = object_with(&mut e, &[(0x100, V_IS_ACTOR)]);
        let a = command(&mut e, this_obj);
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        e.register(DYNAMIC_CAST, |_, _| 0xace0u32.into_ret());
        bool_double(&mut e, ACTOR_IS_ESSENTIAL, true);
        e.mem.set_f64(a.result.addr(), 7.0);
        start_log(&mut e);
        assert!(e.call(0x005d_fc70, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 1.0);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![this_obj, 0, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0]]
        );
        assert_eq!(calls(&e, ACTOR_IS_ESSENTIAL), vec![vec![0xace0]]);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        bool_double(&mut e, ACTOR_IS_ESSENTIAL, false);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_fc70, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_ESSENTIAL_VALUE, 0, 0]]
        );
    }

    #[test]
    fn is_actor_ref_essential_reports_a_reference_that_is_not_an_actor() {
        let mut e = engine();
        let this_obj = object_with(&mut e, &[(0x100, V_IS_ACTOR)]);
        let a = command(&mut e, this_obj);
        e.register(V_IS_ACTOR, |_, _| false.into_ret());
        e.mem.set_f64(a.result.addr(), 7.0);
        start_log(&mut e);
        assert!(e.call(0x005d_fc70, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_fc70, &args![a]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_ESSENTIAL_INVALID]]);
    }

    /// The test shared by the two commands that parse a flag and hand it to
    /// an actor method after the cast: `(command, method, invalid message)`.
    fn check_actor_flag_command(command_address: u32, method: u32, message: u32) {
        let mut e = engine();
        let this_obj = object_with(&mut e, &[(0x100, V_IS_ACTOR)]);
        let a = command(&mut e, this_obj);
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        e.register(DYNAMIC_CAST, |_, _| 0xace0u32.into_ret());
        nothing(&mut e, method);
        for (value, flag) in [(1u32, 1u32), (0, 0), (0xffff_ffff, 0)] {
            parse_gives(&mut e, true, &[value]);
            start_log(&mut e);
            assert!(e.call(command_address, &args![a]).bool());
            assert_eq!(
                calls(&e, DYNAMIC_CAST),
                vec![vec![this_obj, 0, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0]]
            );
            assert_eq!(calls(&e, method), vec![vec![0xace0, flag]]);
        }
        // Not an actor: only a message, and only with the echo flag.
        e.register(V_IS_ACTOR, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert!(calls(&e, method).is_empty());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![message]]);
        // A parameter that does not parse.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(command_address, &args![a]).bool());
        assert!(calls(&e, V_IS_ACTOR).is_empty());
    }

    #[test]
    fn set_actor_ref_essential_sets_the_flag_of_the_cast_actor() {
        check_actor_flag_command(0x005d_fd30, ACTOR_SET_ESSENTIAL, MSG_SET_ESSENTIAL_INVALID);
    }

    #[test]
    fn always_show_actor_subtitles_sets_the_flag_of_the_cast_actor() {
        check_actor_flag_command(
            0x005d_ff70,
            ACTOR_SET_ALWAYS_SHOW_SUBTITLES,
            MSG_SUBTITLES_INVALID,
        );
    }

    #[test]
    fn is_no_human_gore_exe_is_always_zero() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.mem.set_f64(a.result.addr(), 5.0);
        start_log(&mut e);
        assert!(e.call(0x005d_fde0, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_fde0, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_NO_HUMAN_GORE, 0, 0]]
        );
    }

    #[test]
    fn fn_005dfe30_passes_the_value_and_1() {
        let mut e = engine();
        nothing(&mut e, SHARED_ACTION_005F6110);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0x33]);
        start_log(&mut e);
        assert!(e.call(0x005d_fe30, &args![a]).bool());
        assert_eq!(calls(&e, SHARED_ACTION_005F6110), vec![vec![0x33, 1]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005d_fe30, &args![a]).bool());
        assert!(calls(&e, SHARED_ACTION_005F6110).is_empty());
    }

    #[test]
    fn persistent_ref_count_counts_the_persistent_values() {
        let mut e = engine();
        let table = e.mem.alloc(0x20);
        e.set_global(HASH_TABLE_011C54C0, table);
        // (virtual flag, word at +8) per value; a null value is skipped.
        let make = |e: &mut Engine, flag: bool, word: u32| {
            let object = object_with(e, &[(0xf0, V_FLAG_F0)]);
            e.mem.set_u8(object + 0x10, flag as u8);
            e.mem.set_u32(object + 8, word);
            object
        };
        let values = [
            make(&mut e, true, 0x400),
            make(&mut e, false, 0x400),
            make(&mut e, true, 0x3ff),
            0,
            make(&mut e, true, 0x4ff),
        ];
        e.register(V_FLAG_F0, |e, a| (e.mem.u8(a[0] + 0x10) != 0).into_ret());
        e.register(FIELD_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(HASH_TABLE_FIRST, |_, _| 1u32.into_ret());
        let mut index = 0;
        e.register_double(HASH_TABLE_NEXT, move |e, a| {
            // `a`: table, &node, &key, &value.
            e.mem.set_u32(a[3], values[index]);
            index += 1;
            e.mem
                .set_u32(a[1], if index < values.len() { 1 } else { 0 });
            Ret::default()
        });
        let a = command(&mut e, 0);
        e.mem.set_f64(a.result.addr(), 9.0);
        start_log(&mut e);
        assert!(e.call(0x005d_fe90, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 2.0);
        assert_eq!(calls(&e, HASH_TABLE_NEXT).len(), 5);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn persistent_ref_count_prints_the_count_and_needs_the_table() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.mem.set_f64(a.result.addr(), 9.0);
        set_echo(&mut e, true);
        // No table: nothing happens and the result is left alone.
        start_log(&mut e);
        assert!(e.call(0x005d_fe90, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 9.0);
        assert_eq!(call_order(&e).len(), 1);
        // An empty table: 0, printed.
        let table = e.mem.alloc(0x20);
        e.set_global(HASH_TABLE_011C54C0, table);
        e.register(HASH_TABLE_FIRST, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_fe90, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 0.0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_PERSISTENT_REF_COUNT, 0]]
        );
    }

    #[test]
    fn fn_005e0020_stores_whether_the_value_is_1() {
        let mut e = engine();
        let a = command(&mut e, 0);
        for (value, stored) in [(1u32, 1u8), (2, 0), (0, 0)] {
            e.set_global(STORED_FLAG_011DD437, 9u8);
            parse_gives(&mut e, true, &[value]);
            assert!(e.call(0x005e_0020, &args![a]).bool());
            assert_eq!(e.global::<u8>(STORED_FLAG_011DD437), stored);
        }
        e.set_global(STORED_FLAG_011DD437, 9u8);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005e_0020, &args![a]).bool());
        assert_eq!(e.global::<u8>(STORED_FLAG_011DD437), 9);
    }

    #[test]
    fn fn_005e0080_stores_the_byte() {
        let mut e = engine();
        e.call(0x005e_0080, &args![0x7fu32]);
        assert_eq!(e.global::<u8>(STORED_FLAG_011DD437), 0x7f);
    }

    #[test]
    fn show_sleep_wait_menu_opens_one_of_the_two_menus() {
        let mut e = engine();
        let player = set_player(&mut e);
        nothing(&mut e, PLAYER_SLEEP_WAIT);
        nothing(&mut e, CREATE_SLEEP_MENU);
        let a = command(&mut e, 0);
        // Second value 1: the player's menu.
        parse_gives(&mut e, true, &[0, 1]);
        start_log(&mut e);
        assert!(e.call(0x005e_0090, &args![a]).bool());
        assert_eq!(calls(&e, PLAYER_SLEEP_WAIT), vec![vec![player]]);
        assert!(calls(&e, CREATE_SLEEP_MENU).is_empty());
        // Otherwise the interface's, with `first == 1`.
        for (first, second, flag) in [(1u32, 0u32, 1u32), (0, 0, 0), (1, 2, 1)] {
            parse_gives(&mut e, true, &[first, second]);
            start_log(&mut e);
            assert!(e.call(0x005e_0090, &args![a]).bool());
            assert_eq!(calls(&e, CREATE_SLEEP_MENU), vec![vec![flag]]);
            assert!(calls(&e, PLAYER_SLEEP_WAIT).is_empty());
        }
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005e_0090, &args![a]).bool());
        assert_eq!(call_order(&e).len(), 2);
    }

    #[test]
    fn get_container_inventory_count_stores_and_prints_the_count() {
        let mut e = engine();
        let container = object(&mut e);
        let a = command(&mut e, container);
        e.register(INVENTORY_COUNT, |_, _| 7u32.into_ret());
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            (value as i32 as u32).into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005e_0100, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 7.0);
        assert_eq!(calls(&e, INVENTORY_COUNT), vec![vec![container, 0, 1]]);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005e_0100, &args![a]).bool());
        let seven = 7.0f64.to_bits();
        assert_eq!(
            calls(&e, FTOL),
            vec![vec![seven as u32, (seven >> 32) as u32]]
        );
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_CONTAINER_COUNT, 7]]);
        // A negative count is printed as an integer.
        e.register(INVENTORY_COUNT, |_, _| (-3i32 as u32).into_ret());
        start_log(&mut e);
        assert!(e.call(0x005e_0100, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), -3.0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_CONTAINER_COUNT, -3i32 as u32]]
        );
    }

    #[test]
    fn get_container_inventory_count_needs_a_this_obj() {
        let mut e = engine();
        let a = command(&mut e, 0);
        e.mem.set_f64(a.result.addr(), 4.0);
        start_log(&mut e);
        assert!(e.call(0x005e_0100, &args![a]).bool());
        assert_eq!(e.mem.f64(a.result.addr()), 4.0);
        assert_eq!(call_order(&e).len(), 1);
    }

    #[test]
    fn fn_005e0160_marks_the_object_when_the_flag_is_set() {
        let mut e = engine();
        let flag = e.mem.alloc(4);
        e.register_double(FLAG_ADDRESS, move |_, _| flag.into_ret());
        nothing(&mut e, SET_BYTE_11);
        e.set_global(POINTER_011DE134, 0x9000u32);
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005e_0160, &args![a]).bool());
        assert_eq!(calls(&e, FLAG_ADDRESS), vec![vec![OBJECT_011DE29C]]);
        assert!(calls(&e, SET_BYTE_11).is_empty());
        e.mem.set_u8(flag, 1);
        start_log(&mut e);
        assert!(e.call(0x005e_0160, &args![a]).bool());
        assert_eq!(calls(&e, SET_BYTE_11), vec![vec![0x9000]]);
    }

    #[test]
    fn fn_005e0190_sets_the_text_and_returns_this() {
        let mut e = engine();
        nothing(&mut e, MESSAGE_SET_TEXT);
        start_log(&mut e);
        assert_eq!(
            e.call(0x005e_0190, &args![0x1000u32, 0x2000u32]).u32(),
            0x1000
        );
        assert_eq!(calls(&e, MESSAGE_SET_TEXT), vec![vec![0x1000, 0x2000]]);
    }

    fn setting_object(e: &mut Engine) -> u32 {
        object_with(
            e,
            &[
                (0x1c, V_SETTING_1C),
                (0x20, V_SETTING_20),
                (0x24, V_SETTING_24),
            ],
        )
    }

    #[test]
    fn fn_005e01b0_returns_the_result_of_slot_0x20_after_a_successful_step() {
        let mut e = engine();
        let setting = setting_object(&mut e);
        nothing(&mut e, V_SETTING_1C);
        bool_double(&mut e, SETTING_STEP, true);
        bool_double(&mut e, V_SETTING_20, false);
        start_log(&mut e);
        assert!(!e.call(0x005e_01b0, &args![setting, 0x44u32]).bool());
        assert_eq!(
            call_order(&e)[1..],
            [SETTING_STEP, V_SETTING_20, V_SETTING_1C]
        );
        assert_eq!(calls(&e, SETTING_STEP), vec![vec![setting, 0x44, 1]]);
        bool_double(&mut e, V_SETTING_20, true);
        assert!(e.call(0x005e_01b0, &args![setting, 0x44u32]).bool());
    }

    #[test]
    fn fn_005e01b0_does_nothing_more_when_the_step_fails() {
        let mut e = engine();
        let setting = setting_object(&mut e);
        bool_double(&mut e, SETTING_STEP, false);
        start_log(&mut e);
        assert!(!e.call(0x005e_01b0, &args![setting, 0x44u32]).bool());
        assert_eq!(call_order(&e).len(), 2);
    }

    #[test]
    fn fn_005e0200_runs_the_step_inside_the_guard() {
        let mut e = engine();
        let setting = setting_object(&mut e);
        nothing(&mut e, V_SETTING_1C);
        nothing(&mut e, GUARD_CONSTRUCT);
        nothing(&mut e, GUARD_DESTRUCT);
        bool_double(&mut e, SETTING_STEP, true);
        // The result is the step's, not the slot's.
        bool_double(&mut e, V_SETTING_24, false);
        start_log(&mut e);
        assert!(e.call(0x005e_0200, &args![setting, 0x44u32]).bool());
        assert_eq!(
            call_order(&e)[1..],
            [
                GUARD_CONSTRUCT,
                SETTING_STEP,
                V_SETTING_24,
                V_SETTING_1C,
                GUARD_DESTRUCT
            ]
        );
        let guard = calls(&e, GUARD_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, GUARD_CONSTRUCT),
            vec![vec![guard, 9, 1, SETTING_H_FILE, 0x19d]]
        );
        assert_eq!(calls(&e, GUARD_DESTRUCT), vec![vec![guard]]);
        assert_eq!(calls(&e, SETTING_STEP), vec![vec![setting, 0x44, 0]]);
    }

    #[test]
    fn fn_005e0200_leaves_the_guard_when_the_step_fails() {
        let mut e = engine();
        let setting = setting_object(&mut e);
        nothing(&mut e, GUARD_CONSTRUCT);
        nothing(&mut e, GUARD_DESTRUCT);
        bool_double(&mut e, SETTING_STEP, false);
        start_log(&mut e);
        assert!(!e.call(0x005e_0200, &args![setting, 0x44u32]).bool());
        assert_eq!(
            call_order(&e)[1..],
            [GUARD_CONSTRUCT, SETTING_STEP, GUARD_DESTRUCT]
        );
    }

    #[test]
    fn fn_005e02b0_returns_what_the_lookup_stores() {
        let mut e = engine();
        e.register(LOOKUP_005E0860, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            if a[1] == 0x77 {
                e.mem.set_u32(a[2], 0x4242);
            }
            Ret::default()
        });
        assert_eq!(
            e.call(0x005e_02b0, &args![0x1000u32, 0x77u32]).u32(),
            0x4242
        );
        assert_eq!(e.call(0x005e_02b0, &args![0x1000u32, 0x78u32]).u32(), 0);
    }

    #[test]
    fn fn_005e02e0_stores_the_vtable_and_destroys_the_pointer() {
        let mut e = engine();
        nothing(&mut e, POINTER_DESTROY);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, 0x5000);
        start_log(&mut e);
        e.call(0x005e_02e0, &args![object]);
        assert_eq!(e.mem.u32(object), 0x0103_d3e8);
        assert_eq!(calls(&e, POINTER_DESTROY), vec![vec![0x5000]]);
    }
}

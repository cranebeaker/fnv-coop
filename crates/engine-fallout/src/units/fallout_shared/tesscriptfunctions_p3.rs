//! `fallout shared/tesscriptfunctions.cpp` (Xbox PDB source unit), part 3: its functions from `005c4240` up to
//! (not including) `005c8450` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesscriptfunctions`]; anything public there may be used here.
//!
//! Progress: all 120 queue entries of this range (`005c4240` to `005c83f0`)
//! are translated; the range is complete.
//!
//! Notes on the exe's code that the translations rely on:
//! - Every body is `cdecl` with the eight stack words of [`ScriptArgs`]. The
//!   word the decompiler calls the sixth parameter (`[ebp+0x1c]`) is the
//!   script event list; the fifth (`[ebp+0x18]`) is the running script.
//! - Many one-line accessors are called by address and named after what they
//!   do (the linker folded identical code, so the engine map often names
//!   another class's method).
//! - Some callees (`005c42d0`'s neighbours, the `NiPoint3` operators, the
//!   helpers that return `this + offset`) are plain `ret` functions: when a
//!   call site pushes stack words before such a call, those words belong to
//!   the *next* call, and the translation passes them there.
//! - The stack-protector cookie checks and the exception-unwinding frames
//!   are not translated. `005b5e40` is the logging stub of the release
//!   build (see the main file); it is called with the arguments the game
//!   pushes.

#[allow(unused_imports)]
use super::tesscriptfunctions::*;
#[allow(unused_imports)]
use crate::prelude::*;

// ---- Callees outside the unit (by exe address) -----------------------------

/// `Script::ParseParameters` (Xbox PDB), `cdecl`, varargs.
const PARSE_PARAMETERS: u32 = 0x005a_ccb0;
/// The console print: format address, then the arguments (`cdecl`); a
/// `double` argument takes two words.
const CONSOLE_PRINT: u32 = 0x0070_3c00;
/// The logging stub of the release build (`005b5e40`): discards its
/// arguments and returns 0.
const LOG_STUB: u32 = 0x005b_5e40;
/// `Script::GetMenuModeConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, parameter, 0, result`).
const GET_MENU_MODE_CONDITION: u32 = 0x0059_c380;
/// `Script::GetDisabledConditionFunction` (Xbox PDB), `cdecl`.
const GET_DISABLED_CONDITION: u32 = 0x0059_cf50;
/// `Script::GetLockedConditionFunction` (Xbox PDB), `cdecl`.
const GET_LOCKED_CONDITION: u32 = 0x0059_d010;
/// `Script::GetLockLevelConditionFunction` (Xbox PDB), `cdecl`.
const GET_LOCK_LEVEL_CONDITION: u32 = 0x0059_d100;
/// `Script::GetIsLockBrokenConditionFunction` (Xbox PDB), `cdecl`.
const GET_IS_LOCK_BROKEN_CONDITION: u32 = 0x0059_d1d0;
/// `Script::GetDiseaseConditionFunction` (Xbox PDB), `cdecl`.
const GET_DISEASE_CONDITION: u32 = 0x0059_d250;
/// `Script::GetVampireConditionFunction` (Xbox PDB), `cdecl`.
const GET_VAMPIRE_CONDITION: u32 = 0x0059_d2a0;
/// `Script::GetIsVoiceTypeConditionFunction` (Xbox PDB), `cdecl`.
const GET_IS_VOICE_TYPE_CONDITION: u32 = 0x0059_f450;
/// `Interface::IsInMenuMode` (Xbox PDB).
const IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// `Script::PutNumericIDInDouble` (Xbox PDB), `cdecl` (`address of the
/// id, double* result`).
const PUT_NUMERIC_ID_IN_DOUBLE: u32 = 0x005a_cc70;
/// `Script::AddPendingDisabledReference` (Xbox PDB), `cdecl`
/// (`reference, bool`).
const ADD_PENDING_DISABLED_REFERENCE: u32 = 0x005a_a500;
/// `Script::AddPendingEnabledReference` (Xbox PDB), `cdecl` (`reference`).
const ADD_PENDING_ENABLED_REFERENCE: u32 = 0x005a_a580;
/// `Script::RemoveDelayedScriptActionReference` (Xbox PDB), `cdecl`
/// (`reference`).
const REMOVE_DELAYED_SCRIPT_ACTION_REFERENCE: u32 = 0x005a_a5d0;
/// `__RTDynamicCast` (`object, 0, source type, target type, 0`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor of `TESObjectREFR` (`.?AVTESObjectREFR@@`).
const RTTI_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// RTTI type descriptor of `Actor` (`.?AVActor@@`).
const RTTI_ACTOR: u32 = 0x0118_46d4;

// Small accessors of forms and references.
/// `*(this + 0x20)`: the base form of a reference (the map calls it
/// `BGSSaveFormBuffer::GetForm`).
const GET_BASE_FORM: u32 = 0x007a_f430;
/// Form type byte (`this + 4`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `*(this + 0xc)`: the form id.
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// Tests form flag `0x800` (`this + 8`).
const HAS_FORM_FLAG_0X800: u32 = 0x0044_0da0;
/// Tests form flag `0x4000` (`this + 8`).
const HAS_FORM_FLAG_0X4000: u32 = 0x0040_77c0;
/// `this + 0x18`.
const PLUS_0X18: u32 = 0x0050_0940;
/// `thiscall` on a reference (`flag`): sets or clears form flag
/// `0x0800_0000`.
const SET_FORM_FLAG_0X8000000: u32 = 0x0056_c780;
/// `thiscall` on a reference: the enable state parent (non-zero when the
/// reference follows another's enable state).
const GET_ENABLE_STATE_PARENT: u32 = 0x0056_a9f0;
/// `thiscall` on a reference: its editor name through virtual slot `0x130`
/// unless the guard flag is set (then 0).
const GET_NAME_GUARDED: u32 = 0x0047_4cb0;
/// `thiscall` on a reference: the name text of its base form.
const GET_NAME_TEXT: u32 = 0x0055_d520;
/// `thiscall` on a reference: whether the flags of its extra data `0x92`
/// have bit 8 set.
const HAS_EXTRA_FLAG_0X8: u32 = 0x0056_5090;
/// `thiscall` on a reference (`bool`): sets or clears bit 8 of the flags of
/// its extra data `0x92` (creating the extra data when setting).
const SET_EXTRA_FLAG_0X8: u32 = 0x0056_50d0;
/// `thiscall` on the process object (`+0xc4` of it): the interface mode.
const GET_PROCESS_MODE: u32 = 0x0070_58c0;
/// `thiscall` on the process object (`mode`): stores the mode at `+0xc4`.
const SET_PROCESS_MODE: u32 = 0x0070_5b10;
/// `TES::GetLODMult` (Xbox PDB): the mode for a base form, `cdecl`
/// (`form`).
const GET_MODE_FOR_FORM: u32 = 0x0045_c6b0;
/// `this + 4` (`BaseProcess::GetActorPackageThatIsRunning` in the engine
/// map): used on the process lists singleton and on the temporary container.
const PLUS_4: u32 = 0x0071_7e50;
/// `thiscall` on `process lists + 4` (`list`): the number of actors in that
/// list (`*(this + 0x20 + 4 * list)`); the call site pushes the argument
/// before the `this + 4` call.
const PROCESS_LISTS_COUNT: u32 = 0x005b_e5c0;
/// `thiscall` on `process lists + 4` (`index`): the actor at the index.
const PROCESS_LISTS_ACTOR: u32 = 0x0096_8670;
/// `Actor::FadeIn` (Xbox PDB), `thiscall` (no stack arguments).
const FADE_IN: u32 = 0x008b_cd20;

// Placing references.
/// `*(this + 0x24)`: the rotation (three floats) of a reference.
const GET_ROTATION: u32 = 0x0043_0830;
/// `NiPoint3::NiPoint3(x, y, z)`, `thiscall`, returns `this`.
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// `*(this + 0x34)`: the world rotation (nine floats) of a scene node.
const GET_WORLD_ROTATE: u32 = 0x006a_9540;
/// `*(this + 0x58)`: the world translation (three floats) of a scene node.
const GET_WORLD_TRANSLATE: u32 = 0x0043_c490;
/// `thiscall` on a 3x3 matrix (`row, column`): the float at
/// `this + 12 * row + 4 * column`.
const MATRIX_ELEMENT: u32 = 0x004b_55c0;
/// `thiscall` on a `NiPoint3` (`factor`): multiplies it in place.
const POINT3_SCALE: u32 = 0x0043_9180;
/// `thiscall` on a `NiPoint3` (`other`): subtracts in place.
const POINT3_SUBTRACT_ASSIGN: u32 = 0x0045_78c0;
/// `thiscall` on a `NiPoint3` (`other`): adds in place.
const POINT3_ADD_ASSIGN: u32 = 0x0063_c8a0;
/// `thiscall` on a `NiPoint3` (`out, factor`): `this * factor` into `out`;
/// returns `out`.
const POINT3_SCALED: u32 = 0x0045_bb20;
/// `thiscall` on a `NiPoint3` (`out, other`): `this + other` into `out`;
/// returns `out`.
const POINT3_SUM: u32 = 0x0043_9e90;
/// `thiscall` on a `NiPoint3` (`out, other`): `this - other` into `out`;
/// returns `out`.
const POINT3_DIFFERENCE: u32 = 0x0043_9ef0;
/// `thiscall` on a `NiPoint3`: divides by its length (or zeroes it).
const POINT3_UNITIZE: u32 = 0x004a_0c10;
/// The default `NiPoint3` constructor passed to the vector iterator (it
/// does nothing and returns `this`).
const POINT3_DEFAULT_CONSTRUCT: u32 = 0x0068_15c0;
/// `_vector_constructor_iterator_` (`array, size, count, constructor`).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `cdecl` (`angle`): trigonometric wrapper (the x component of the ring
/// offset).
const RING_X: u32 = 0x005b_9e80;
/// `TESObjectREFR::GetWorldSpace` (Xbox PDB): a plain `ret` function whose
/// call sites push the three trailing zeros of [`CREATE_REFERENCE`].
const GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// `thiscall` on a reference: its parent cell (`this + 0x40`).
const GET_PARENT_CELL: u32 = 0x008d_6f30;
/// `thiscall` on the data handler (`form, position*, rotation*, cell,
/// world space, 0, 0, 0`): creates a reference of the form; returns it.
const CREATE_REFERENCE: u32 = 0x0046_98a0;
/// `TESHealthForm::GetFormAsHealthForm` (Xbox PDB), `cdecl` (`form`).
const GET_FORM_AS_HEALTH_FORM: u32 = 0x0048_72e0;
/// `thiscall` on the reference (`float`): sets its health.
const SET_HEALTH: u32 = 0x0056_8bd0;
/// `TESObjectREFR::GetCalcLevel` (Xbox PDB), `thiscall` (`0`).
const GET_CALC_LEVEL: u32 = 0x0056_7e10;
/// `TESBoundObject::GetBoundSize` (Xbox PDB), `thiscall`: a `float` in
/// `ST0`.
const GET_BOUND_SIZE: u32 = 0x0050_ebf0;
/// `TESContainer::TESContainer` (Xbox PDB): a 0x10-byte object.
const TES_CONTAINER_CONSTRUCT: u32 = 0x0048_1610;
/// Destructor of the container object.
const TES_CONTAINER_DESTRUCT: u32 = 0x0048_1680;
/// `thiscall` on `leveled list + 0x30` (`level, count, container, 0`).
const ROLL_LEVELED_LIST: u32 = 0x0048_7f70;
/// `thiscall` on a list node: `true` when it has neither an item nor a next
/// node.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `thiscall` on a list node: the address of the node's item.
const LIST_ITEM_PTR: u32 = 0x0068_15c0;
/// `thiscall` on a list node: the next node.
const LIST_NEXT: u32 = 0x0072_6070;
/// `thiscall` on the `TES` object (`position*`): whether the position is in
/// the loaded area.
const IS_POSITION_LOADED: u32 = 0x0045_1110;
/// `thiscall` on a pick (the constructor, `0xbc` bytes).
const PICK_CONSTRUCT: u32 = 0x004a_3c20;
/// `thiscall` on a `UInt32` (`value`): stores it.
const STORE_VALUE: u32 = 0x008c_71b0;
/// `thiscall` on a mask (`bits`): replaces its low 7 bits.
const MASK_SET_LOW_BITS: u32 = 0x004a_39f0;
/// `thiscall` on a mask (`bits`): replaces its high 16 bits.
const MASK_SET_HIGH_BITS: u32 = 0x0059_ce80;
/// `thiscall` on a pick (`mask`).
const PICK_SET_MASK: u32 = 0x004a_3f70;
/// `thiscall` on a pick (`position*`): the ray start.
const PICK_SET_START: u32 = 0x004a_3da0;
/// `thiscall` on a pick (`position*`): the ray end.
const PICK_SET_END: u32 = 0x004a_3eb0;
/// `TES::Pick` (Xbox PDB), `thiscall` on the `TES` object (`pick*`).
const TES_PICK: u32 = 0x0045_8420;
/// `thiscall` on the byte's address: whether it is non-zero.
const BYTE_IS_SET: u32 = 0x0056_09f0;
/// `thiscall` on the byte's address (`value`): stores the byte.
const STORE_BYTE: u32 = 0x0062_2570;
/// `cdecl` (`angle`): trigonometric wrapper (the y component of the ring
/// offset).
const RING_Y: u32 = 0x004e_44b0;
/// Returns the object whose `ST0`-valued method [`RANDOM_ANGLE_FROM`]
/// yields the starting angle of the ring.
const RANDOM_OBJECT: u32 = 0x0047_6c00;
/// `thiscall` on [`RANDOM_OBJECT`]'s result: a `float` in `ST0`.
const RANDOM_ANGLE_FROM: u32 = 0x004d_ff60;

// Save games, files, strings.
/// `BGSSaveLoadManager::SaveGame` (Xbox PDB), `thiscall` (`name, -1,
/// bool`).
const SAVE_GAME: u32 = 0x0085_03b0;
/// `thiscall` on the manager (`name, -1, bool, 1`).
const SAVE_GAME_WITH_FLAG: u32 = 0x0085_0760;
/// `BGSSaveLoadManager::CopySaveGames` (Xbox PDB), `thiscall` (`bool`).
const COPY_SAVE_GAMES: u32 = 0x0085_2080;
/// `thiscall` on the manager (no arguments).
const MANAGER_ACTION_A: u32 = 0x0085_1d30;
/// `thiscall` on the manager (no arguments).
const MANAGER_ACTION_B: u32 = 0x0085_1d50;
/// `DetailedActorPathHandler::IsDetailedPathHandler` (Xbox PDB), `thiscall`
/// (`-1`); compiled to `AL = 1`.
const IS_DETAILED_PATH_HANDLER: u32 = 0x0040_1290;
/// `thiscall` on a setting object: the address of its value byte.
const GET_SETTING_VALUE_ADDRESS: u32 = 0x0040_8d60;
/// `strcpy` (`dest, source`), `cdecl`.
const STRING_COPY: u32 = 0x0040_46f0;
/// `_stricmp` (`a, b`), `cdecl`; 0 when equal.
const STRING_COMPARE_NO_CASE: u32 = 0x0040_4dc0;
/// `_strcmp` (`a, b`), `cdecl`; 0 when equal.
const STRING_COMPARE: u32 = 0x00ec_6da0;
/// `_strlen`.
const STRLEN: u32 = 0x00ec_6130;
/// `strcpy_s` (`dest, size, source`).
const STRCPY_S: u32 = 0x0040_6d30;
/// `sprintf_s` (`dest, size, format, ...`).
const SPRINTF_S: u32 = 0x0040_6d00;
/// `strstr`-like search (`haystack, needle`): non-zero when found.
const FIND_SUBSTRING: u32 = 0x0048_12f0;
/// `BSStringT` constructor (an 8-byte object).
const BS_STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `BSStringT` destructor.
const BS_STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `BSStringT` format (`string, format, ...`).
const BS_STRING_FORMAT: u32 = 0x0040_6f60;
/// `NiPointer::operator T*` and `BSStringT::c_str`: `*this`.
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `cdecl` (`value`): returns it (compiled identity).
const IDENTITY: u32 = 0x0046_4f30;

// Scene, camera and node toggles.
/// `thiscall` on the `TES` object (`node index, state byte, 0`).
const APPLY_CELL_NODE_STATE: u32 = 0x0045_6d00;
/// `cdecl` (`node index`): whether bit `index` of the global `011ca08c` is
/// set.
const GET_CELL_NODE_FLAG: u32 = 0x0054_68f0;
/// `DetailedActorPathHandler::GetCurrentNodeIndex` (Xbox PDB): `this + ...`,
/// a plain `ret` function; its call sites push the arguments of
/// [`REFRESH_PATH_NODES`] first.
const GET_CURRENT_NODE_INDEX: u32 = 0x0070_ec90;
/// `thiscall` on the result of [`GET_CURRENT_NODE_INDEX`] (`displayed, 0,
/// 0`).
const REFRESH_PATH_NODES: u32 = 0x004e_6370;
/// `cdecl`: whether all trees are culled.
const GET_TREES_CULLED: u32 = 0x0054_ee20;
/// `cdecl` (`bool`): sets whether all trees are culled.
const SET_TREES_CULLED: u32 = 0x0054_ee60;
/// `cdecl` (`a, b`): the smaller of the two floats; a `float` in `ST0`.
const MINIMUM: u32 = 0x0040_ebd0;
/// `thiscall`-less read of the scene graph pointer (global `011deb7c`).
const GET_SCENE_GRAPH: u32 = 0x0045_c670;
/// `BSSceneGraph::SetCameraFOV` (Xbox PDB), `thiscall` (`fov, 0, 0, 0`).
const SET_CAMERA_FOV: u32 = 0x00c5_2020;
/// `cdecl` (`fov`): updates the shader manager's field of view.
const SET_SHADER_FOV: u32 = 0x00b5_4000;
/// `thiscall` (`fov`): stores a field of view in a water singleton.
const SET_WATER_FOV: u32 = 0x004e_d780;
/// Water singleton (x field of view), `this` of [`SET_WATER_FOV`].
const WATER_FOV_X: u32 = 0x0120_315c;
/// Water singleton (y field of view), `this` of [`SET_WATER_FOV`].
const WATER_FOV_Y: u32 = 0x0120_3168;
/// `cdecl`: walks the objects of the shader manager and calls virtual slot
/// `0x120` on them; its output goes through the callback global.
const SHADER_MANAGER_DUMP: u32 = 0x00b5_57d0;
/// The console-printing callback `005c5a20` installs (`text`).
const PRINT_CALLBACK: u32 = 0x005c_5a00;

// Dumps.
/// `MessageHandler::IncDisableWarningCount` (Xbox PDB), `cdecl` (`bool`).
const INC_DISABLE_WARNING_COUNT: u32 = 0x0043_b2b0;
/// `BSFile::BSFile` (Xbox PDB), `thiscall` (`path, 1, 0x4000, 0`).
const BS_FILE_CONSTRUCT: u32 = 0x00b0_0260;
/// `BSFile::Open` (Xbox PDB), `thiscall` (`0, 0`).
const BS_FILE_OPEN: u32 = 0x00af_f300;
/// `BSFile::Close` (Xbox PDB).
const BS_FILE_CLOSE: u32 = 0x00af_fd10;
/// `BSFile::~BSFile` (Xbox PDB).
const BS_FILE_DESTRUCT: u32 = 0x00af_f240;
/// `thiscall` on a `BSFile` (`buffer, size`): writes bytes.
const BS_FILE_WRITE: u32 = 0x00af_f950;
/// `ModelLoader::OutputModelMapContents` (Xbox PDB), `thiscall`
/// (`file, 0`).
const MODEL_LOADER_OUTPUT_MODEL_MAP: u32 = 0x0044_3270;
/// `BSFaceGenManager::GetModelCache` (Xbox PDB).
const GET_MODEL_CACHE: u32 = 0x0065_2110;
/// `BSFaceGenModelMap::OutputModelMapContents` (Xbox PDB), `thiscall`
/// (`file`).
const FACE_GEN_OUTPUT_MODEL_MAP: u32 = 0x0064_fe80;
/// `eh_vector_constructor_iterator` (`array, size, count, constructor,
/// destructor`).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// `eh_vector_destructor_iterator` (`array, size, count, destructor`).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// `NiPointer` constructor passed to the vector iterator.
const NI_POINTER_CONSTRUCT: u32 = 0x0066_94e0;
/// `NiPointer` destructor passed to the vector iterator.
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// Zero-fills (`array, 0, bytes`).
const ZERO_FILL: u32 = 0x0040_3d30;
/// First texture of the global list.
const FIRST_TEXTURE: u32 = 0x0045_6510;
/// `this + 0x2c`: the next texture.
const NEXT_TEXTURE: u32 = 0x0055_b980;
/// `this + 0x24`: the texture's Direct3D texture.
const GET_D3D_TEXTURE: u32 = 0x0059_bb30;
/// `this + 0x24` of the `TES` object.
const TES_GRID_X: u32 = 0x0059_bb30;
/// `this + 0x28` of the `TES` object.
const TES_GRID_Y: u32 = 0x0045_cd60;
/// `NiPointer::operator=` (`to`, `from`).
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// Assigns a raw pointer to an `NiPointer` (`slot, pointer`).
const NI_POINTER_SET: u32 = 0x0066_b0d0;
/// `cdecl` (`class, texture`): whether the texture is of the class.
const TEXTURE_IS_SOURCE: u32 = 0x0043_b300;
/// `thiscall`: the C string of a name handle.
const NAME_TEXT: u32 = 0x0043_b1b0;
/// `this + 8`.
const FN_00413F40: u32 = 0x0041_3f40;
/// `cdecl` (`text, size`): fixes the path in place.
const FIX_PATH: u32 = 0x004a_fb00;
/// `this + 0x14`.
const FN_007D6BB0: u32 = 0x007d_6bb0;
/// `cdecl` (`surface`): the Direct3D format id.
const D3D_FORMAT_ID: u32 = 0x00e7_bd20;
/// `NiXenonRenderer::GetD3DFormatString` (Xbox PDB), `cdecl`
/// (`format id`).
const GET_D3D_FORMAT_STRING: u32 = 0x00e7_a9a0;
/// `thiscall` (`texture`): 3 for the textures flagged "L".
const TEXTURE_KIND: u32 = 0x0043_c430;
/// `TESWorldSpace` / interior accessor: `this + 0x34` of the `TES` object
/// (`ActorMover::GetPreferredMoveMode` in the engine map).
const GET_INTERIOR_CELL: u32 = 0x005f_36f0;
/// `TES::GetWorldSpace` (Xbox PDB): `this + 0x88` of the `TES` object.
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `thiscall` on the `ActorBase`-side object (`this + 0x10c`'s next).
const GET_RACE: u32 = 0x004a_c110;
/// `TESActorBase::GetSex` (Xbox PDB): -1 when not an NPC, otherwise
/// whether the base is female.
const GET_SEX: u32 = 0x005f_0cc0;
/// `thiscall` on an actor: its sex (through the base form).
const ACTOR_GET_SEX: u32 = 0x0087_f4c0;
/// `TESActorBaseData::SetFlagBit` (Xbox PDB), `thiscall` (`bit, set,
/// notify`).
const SET_FLAG_BIT: u32 = 0x0047_dd50;
/// Constructor of the 0x10-byte array the actor search fills.
const ARRAY_CONSTRUCT: u32 = 0x005e_0560;
/// Destructor of that array.
const ARRAY_DESTRUCT: u32 = 0x005e_0590;
/// `thiscall` (`race, sex, array`): collects the NPCs of the race and sex.
const FIND_NPCS_OF_RACE_AND_SEX: u32 = 0x0060_3280;
/// `BSSimpleArray::operator[]`-like (`index`): the address of the element.
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// `thiscall` on an NPC base (`template NPC`): copies the look of the
/// template.
const COPY_LOOK_FROM: u32 = 0x0060_3790;
/// `Character::Reset3D` (Xbox PDB), `thiscall`.
const RESET_3D: u32 = 0x008d_3fa0;

// Condition functions of `fallout shared/tesconditionfunctions.cpp` that the
// command bodies of this range call (all `cdecl`: `thisObj, parameter 1,
// parameter 2, result`; the result is their `AL`).
/// `Script::GetClothingValueConditionFunction` (Xbox PDB).
const GET_CLOTHING_VALUE_CONDITION: u32 = 0x0059_d320;
/// `Script::SameFactionConditionFunction` (Xbox PDB).
const SAME_FACTION_CONDITION: u32 = 0x0059_d400;
/// `Script::SameRaceConditionFunction` (Xbox PDB).
const SAME_RACE_CONDITION: u32 = 0x0059_d540;
/// `Script::SameSexConditionFunction` (Xbox PDB).
const SAME_SEX_CONDITION: u32 = 0x0059_d610;
/// `Script::GetDetectedConditionFunction` (Xbox PDB).
const GET_DETECTED_CONDITION: u32 = 0x0059_d6e0;
/// `Script::GetDeadConditionFunction` (Xbox PDB).
const GET_DEAD_CONDITION: u32 = 0x0059_d840;
/// `Script::GetItemCountConditionFunction` (Xbox PDB).
const GET_ITEM_COUNT_CONDITION: u32 = 0x0059_d8e0;
/// `Script::GetEquippedConditionFunction` (Xbox PDB).
const GET_EQUIPPED_CONDITION: u32 = 0x0059_da90;
/// `Script::GetGoldConditionFunction` (Xbox PDB).
const GET_GOLD_CONDITION: u32 = 0x0059_dbe0;
/// `Script::GetSleepingConditionFunction` (Xbox PDB).
const GET_SLEEPING_CONDITION: u32 = 0x0059_dc90;
/// `Script::GetSittingConditionFunction` (Xbox PDB).
const GET_SITTING_CONDITION: u32 = 0x0059_dd90;
/// `Script::GetFurnitureMarkerIDConditionFunction` (Xbox PDB).
const GET_FURNITURE_MARKER_ID_CONDITION: u32 = 0x0059_de90;
/// `Script::IsCurrentFurnitureRefConditionFunction` (Xbox PDB).
const IS_CURRENT_FURNITURE_REF_CONDITION: u32 = 0x0059_df40;
/// `Script::IsCurrentFurnitureObjConditionFunction` (Xbox PDB).
const IS_CURRENT_FURNITURE_OBJ_CONDITION: u32 = 0x0059_dfe0;
/// `Script::GetTalkedToPCConditionFunction` (Xbox PDB).
const GET_TALKED_TO_PC_CONDITION: u32 = 0x0059_e0f0;
/// `Script::GetQuestRunningConditionFunction` (Xbox PDB).
const GET_QUEST_RUNNING_CONDITION: u32 = 0x0059_e320;
/// `Script::GetQuestCompletedConditionFunction` (Xbox PDB).
const GET_QUEST_COMPLETED_CONDITION: u32 = 0x0059_e390;
/// `Script::GetStageConditionFunction` (Xbox PDB).
const GET_STAGE_CONDITION: u32 = 0x0059_e420;
/// `Script::GetStageDoneConditionFunction` (Xbox PDB).
const GET_STAGE_DONE_CONDITION: u32 = 0x0059_e490;
/// `Script::GetFactionRankDifferenceConditionFunction` (Xbox PDB).
const GET_FACTION_RANK_DIFFERENCE_CONDITION: u32 = 0x0059_e510;
/// `Script::GetAlarmedConditionFunction` (Xbox PDB).
const GET_ALARMED_CONDITION: u32 = 0x0059_e650;
/// `Script::GetIsPleasantConditionFunction` (Xbox PDB).
const GET_IS_PLEASANT_CONDITION: u32 = 0x0059_e700;
/// `Script::GetIsCloudyConditionFunction` (Xbox PDB).
const GET_IS_CLOUDY_CONDITION: u32 = 0x0059_e7f0;
/// `Script::GetIsRainingConditionFunction` (Xbox PDB).
const GET_IS_RAINING_CONDITION: u32 = 0x0059_e8e0;
/// `Script::GetIsSnowingConditionFunction` (Xbox PDB).
const GET_IS_SNOWING_CONDITION: u32 = 0x0059_ea10;
/// `Script::GetWindSpeedConditionFunction` (Xbox PDB).
const GET_WIND_SPEED_CONDITION: u32 = 0x005a_1880;
/// `Script::GetWeatherPercentConditionFunction` (Xbox PDB).
const GET_WEATHER_PERCENT_CONDITION: u32 = 0x0059_eb60;
/// `Script::GetIsCurrentWeatherConditionFunction` (Xbox PDB).
const GET_IS_CURRENT_WEATHER_CONDITION: u32 = 0x0059_ebb0;

// Actors, quests and NPC bases.
/// `Actor::RestoreActorValue` (Xbox PDB), `thiscall` (`actor value code,
/// float amount`).
const RESTORE_ACTOR_VALUE: u32 = 0x0088_b740;
/// `thiscall` on a quest (`stage` byte): the engine map names it
/// `TESQuest::SetStageDone`; the `SetStage` command tests its `AL`.
const QUEST_SET_STAGE: u32 = 0x0060_d510;
/// `thiscall` on a quest (`bool`): sets or clears bit 0 of a flags byte
/// (found through `005a8080`) and notifies through virtual slot `0x48`
/// (`2`).
const QUEST_SET_FLAG_0X1: u32 = 0x0060_c9c0;
/// `thiscall` on a quest (`bool`): sets or clears bit 1 of the byte at
/// `+0x3c`, notifies through virtual slot `0x48` (`2`) and runs follow-up
/// work.
const QUEST_SET_FLAG_0X2: u32 = 0x0060_ca30;
/// `thiscall` on an NPC base: `*(this + 0x130)` (the engine map calls it
/// `MiddleHighProcess::GetFireNode`; the linker folded identical code).
const NPC_GET_FIELD_0X130: u32 = 0x0050_2430;
/// `thiscall` on an NPC base (`value`): stores `*(this + 0x130)` and marks
/// the change through virtual slot `0x48` (`0x400`).
const NPC_SET_FIELD_0X130: u32 = 0x0060_1c70;
/// `thiscall` on the `TESActorBaseData` part (`NPC + 0x30`): a 16-bit
/// level value (the code that stores it uses `sLevel` at `+0xc` of the
/// Xbox `ACTOR_BASE_DATA` layout).
const ACTOR_BASE_DATA_GET_LEVEL: u32 = 0x0047_ded0;
/// `thiscall` on the `TESActorBaseData` part (`NPC + 0x30`) (`u16`): stores
/// the 16-bit value at `+0xc` and marks the change (`2`).
const ACTOR_BASE_DATA_SET_LEVEL: u32 = 0x0047_dfe0;
/// `TESNPC::InitValues` (Xbox PDB), `thiscall` (`0`).
const TES_NPC_INIT_VALUES: u32 = 0x0060_3be0;
/// RTTI type descriptor of `TESBoundObject` (`.?AVTESBoundObject@@`).
const RTTI_TES_BOUND_OBJECT: u32 = 0x0118_3108;
/// RTTI type descriptor of `TESNPC` (`.?AVTESNPC@@`).
const RTTI_TES_NPC: u32 = 0x0118_3a1c;
/// The float `999.0` that `005c6b60` restores the limb values by.
const LIMB_RESTORE_AMOUNT: u32 = 0x0103_b2d0;

// Condition functions of 005c7490..005c83f0 (all `cdecl`: `thisObj,
// first, second, result`; their `AL` is the command's result).
/// `Script::GetAttackedConditionFunction` (Xbox PDB).
const GET_ATTACKED_CONDITION: u32 = 0x0059_ec30;
/// `Script::GetIsCreatureConditionFunction` (Xbox PDB).
const GET_IS_CREATURE_CONDITION: u32 = 0x0059_ecc0;
/// `Script::GetShouldAttackConditionFunction` (Xbox PDB).
const GET_SHOULD_ATTACK_CONDITION: u32 = 0x0059_ed30;
/// `Script::GetInCellConditionFunction` (Xbox PDB).
const GET_IN_CELL_CONDITION: u32 = 0x0059_ef60;
/// `Script::GetInWorldspaceConditionFunction` (Xbox PDB).
const GET_IN_WORLDSPACE_CONDITION: u32 = 0x0059_f0c0;
/// `Script::GetInSameCellConditionFunction` (Xbox PDB).
const GET_IN_SAME_CELL_CONDITION: u32 = 0x0059_ee80;
/// `Script::GetIsClassConditionFunction` (Xbox PDB).
const GET_IS_CLASS_CONDITION: u32 = 0x0059_f180;
/// `Script::GetIsRaceConditionFunction` (Xbox PDB).
const GET_IS_RACE_CONDITION: u32 = 0x0059_f240;
/// `Script::GetIsCreatureTypeConditionFunction` (Xbox PDB).
const GET_IS_CREATURE_TYPE_CONDITION: u32 = 0x0059_f300;
/// `Script::GetIsSexConditionFunction` (Xbox PDB).
const GET_IS_SEX_CONDITION: u32 = 0x0059_f3c0;
/// `Script::GetIsPlayableRaceConditionFunction` (Xbox PDB).
const GET_IS_PLAYABLE_RACE_CONDITION: u32 = 0x0059_f540;
/// `Script::GetInFactionConditionFunction` (Xbox PDB).
const GET_IN_FACTION_CONDITION: u32 = 0x0059_f630;
/// `Script::GetIsIDConditionFunction` (Xbox PDB).
const GET_IS_ID_CONDITION: u32 = 0x0059_f7b0;
/// `Script::IsInListConditionFunction` (Xbox PDB).
const IS_IN_LIST_CONDITION: u32 = 0x0059_f890;
/// `Script::GetIsChildConditionFunction` (Xbox PDB).
const GET_IS_CHILD_CONDITION: u32 = 0x0059_f940;
/// `Script::GetIsUsedItemConditionFunction` (Xbox PDB).
const GET_IS_USED_ITEM_CONDITION: u32 = 0x0059_f9b0;
/// `Script::GetIsUsedItemTypeConditionFunction` (Xbox PDB).
const GET_IS_USED_ITEM_TYPE_CONDITION: u32 = 0x0059_fa90;
/// `Script::GetUsedItemLevelConditionFunction` (Xbox PDB).
const GET_USED_ITEM_LEVEL_CONDITION: u32 = 0x0059_fb00;
/// `Script::GetUsedItemActivateConditionFunction` (Xbox PDB).
const GET_USED_ITEM_ACTIVATE_CONDITION: u32 = 0x0059_fb50;
/// `Script::GetIsRefConditionFunction` (Xbox PDB).
const GET_IS_REF_CONDITION: u32 = 0x0059_fbb0;
/// `Script::GetFactionRankConditionFunction` (Xbox PDB).
const GET_FACTION_RANK_CONDITION: u32 = 0x0059_fc50;
/// `Script::GetGlobalValueConditionFunction` (Xbox PDB).
const GET_GLOBAL_VALUE_CONDITION: u32 = 0x0059_fd30;
/// `Script::GetXPForNextLevelConditionFunction` (Xbox PDB).
const GET_XP_FOR_NEXT_LEVEL_CONDITION: u32 = 0x005a_58c0;

// Cells, the player, actors, factions and extra data.
/// `thiscall` on a cell (`bool`): sets or clears bit `0x20` of the byte at
/// `this + 0x24` and notifies through virtual slot `0x48` (`2`).
const CELL_SET_FLAG_0X20: u32 = 0x0054_4340;
/// `thiscall` on the player (`byte`): stores the byte at `this + 0x75c`
/// (`bChargen` of the Xbox `PlayerCharacter`, which sits at `+0x76c` there;
/// 0x10 lower on PC, as for every class derived from `TESForm`).
const PLAYER_SET_CHARGEN: u32 = 0x0095_0010;
/// `thiscall` on an actor (`faction, rank`, the rank a byte): sets the
/// actor's rank in a faction; the faction is added to the actor, changed or
/// (rank `-1`) removed.
const ACTOR_SET_FACTION_RANK: u32 = 0x008b_8580;
/// `thiscall` on an actor (`faction, delta, flag`, the delta and the flag
/// bytes): when the actor has the faction at a rank of at least 0, sets the
/// rank to the old rank plus the (signed) delta, not below 0, through
/// [`ACTOR_SET_FACTION_RANK`].
const ACTOR_ADD_TO_FACTION_RANK: u32 = 0x008b_86e0;
/// `thiscall` on a faction (`other faction`): the reaction of this faction
/// to the other, an `int`.
const FACTION_GET_REACTION: u32 = 0x005f_d3b0;
/// `thiscall` on a faction (`other faction, value`): stores a reaction and
/// notifies through virtual slot `0x48` (`4`).
const FACTION_SET_REACTION: u32 = 0x005f_d3d0;
/// `thiscall` on a faction (`other faction, value`): the same shape as
/// [`FACTION_SET_REACTION`], through another helper (`0048c0b0`).
const FACTION_MODIFY_REACTION: u32 = 0x005f_d400;
/// `thiscall` on a faction (`other faction, code`): the same shape as
/// [`FACTION_SET_REACTION`], through another helper (`0048c220`).
const FACTION_SET_RELATION_CODE: u32 = 0x005f_d430;
/// `thiscall` on a reference: `this + 0x44`, its embedded `ExtraDataList`.
const REFERENCE_EXTRA_LIST: u32 = 0x005d_43c0;
/// `thiscall` on an `ExtraDataList`:
/// `BaseExtraList::GetExtraData(0x5e)` (Xbox PDB), the extra data
/// object or 0.
const GET_EXTRA_DATA_0X5E: u32 = 0x0042_e800;
/// `thiscall` on an `ExtraDataList`: adds the extra data `0x5e` object when
/// the list has none.
const ADD_EXTRA_DATA_0X5E: u32 = 0x0042_e760;
/// `thiscall` on a reference: its base form (the engine map calls it
/// `BGSSaveFormBuffer::GetForm`).
const REFERENCE_BASE_FORM: u32 = 0x0041_81e0;
/// `thiscall` on the extra data `0x5e` object (`base form`): updates the
/// object's lists (it walks the list at `this + 0xc` and, when a second
/// object is given, another list); not examined further here.
const EXTRA_DATA_0X5E_UPDATE: u32 = 0x0043_70f0;
/// `thiscall` on a string holder (the engine map calls it
/// `MapMarkerData::GetLocationName`): the string's text, or the empty string
/// when it has none.
const STRING_OR_EMPTY: u32 = 0x0040_8da0;
/// `_ftol2_sse`: the `double` in `ST0` (a leading `f64` argument) truncated
/// into `EAX`.
const FTOL: u32 = 0x00ec_62c0;
/// `"GetChargenValue >> %i"`
const MSG_GET_CHARGEN_VALUE: u32 = 0x0103_b2d4;
/// `"%.20s reaction to %.20s is %.1f"`
const MSG_FACTION_REACTION: u32 = 0x0103_b2ec;

// ---- Globals ----------------------------------------------------------------

/// The `PlayerCharacter` singleton pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// Pointer to the `TES` object.
const TES: u32 = 0x011d_ea10;
/// Pointer to the data handler singleton.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// Pointer to the save/load manager.
const SAVE_LOAD_MANAGER: u32 = 0x011d_e134;
/// Pointer to the model loader (non-null once created).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The process lists singleton.
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// Byte read by [`fn_005c42d0`].
const FLAG_011DEA2D: u32 = 0x011d_ea2d;
/// Setting object of [`fn_005c5560`].
const SETTING_A: u32 = 0x011d_e29c;
/// Setting object of [`fn_005c5590`].
const SETTING_B: u32 = 0x011d_e2b8;
/// The callback global [`fn_005c5a50`] and [`fn_005c5a60`] access.
const CALLBACK_GLOBAL: u32 = 0x011f_91f4;
/// Texture class object passed to [`TEXTURE_IS_SOURCE`].
const TEXTURE_CLASS: u32 = 0x011f_444c;
/// Table of the form type names (12-byte entries, the name pointer first).
const FORM_TYPE_NAMES: u32 = 0x0118_7004;
/// Table of the two sex names (pointers).
const SEX_NAMES: u32 = 0x0119_9e8c;
/// Byte at `+0x268` of the TLS block: commands echo to the console when set.
const TLS_ECHO: u32 = 0x268;
/// Size of a `BSFile` object (`BSFile::BSFile`).
const BS_FILE_SIZE: u32 = 0x160;
/// Number of slots in the sorted texture table.
const TEXTURE_SLOTS: u32 = 0x800;
/// `float` the field of view is limited to.
const FOV_LIMIT: u32 = 0x0103_b1a4;
/// `float` the field of view defaults to when it is 0.
const FOV_DEFAULT: u32 = 0x0102_f0f8;
/// `double` 0.0 the field of view is compared with.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `double` 1048576.0.
const MEGABYTE: u32 = 0x0101_ece0;
/// `double` that is `float(pi / 4)`: the angle between ring points.
const RING_ANGLE_STEP: u32 = 0x0103_b100;
/// `float` 100.0: the radius of the ring.
const RING_RADIUS: u32 = 0x0101_6410;
/// `double` 64.0: the height added to the pick's start and end.
const PICK_HEIGHT: u32 = 0x0102_40c0;

// ---- String literals (addresses in the exe's data) ---------------------------

/// `"Get Is Form Type >> %0.2f"`
const MSG_GET_IS_FORM_TYPE: u32 = 0x0103_b0e4;
/// `""`: the default string of the copy-save-games command.
const EMPTY_STRING: u32 = 0x0101_1584;
/// `"ms"`
const TEXT_MS: u32 = 0x0103_b108;
/// `"Invalid node index. (0-6)."`
const MSG_INVALID_NODE_INDEX: u32 = 0x0103_b10c;
/// `"Cell %s nodes now %s."`
const MSG_CELL_NODES: u32 = 0x0103_b128;
/// `"CULLED"`
const TEXT_CULLED: u32 = 0x0103_b140;
/// `"DISPLAYED"`
const TEXT_DISPLAYED: u32 = 0x0103_b148;
/// `"All trees are now %s."`
const MSG_ALL_TREES: u32 = 0x0103_b180;
/// `"NOT CULLED"`
const TEXT_NOT_CULLED: u32 = 0x0103_b198;
/// `"Actor"`
const NODE_ACTOR: u32 = 0x0103_b178;
/// `"Marker"`
const NODE_MARKER: u32 = 0x0103_b170;
/// `"Land"`
const NODE_LAND: u32 = 0x0103_b168;
/// `"Static"`
const NODE_STATIC: u32 = 0x0102_9520;
/// `"Dynamic"`
const NODE_DYNAMIC: u32 = 0x0103_b160;
/// `"MultiBound"`
const NODE_MULTI_BOUND: u32 = 0x0103_b154;
/// `"Water"`
const NODE_WATER: u32 = 0x0103_31a8;
/// Enable: `"SCRIPTS: Enable is being called on reference %08X %s in script
/// %08X %s even though it has an enable state parent. ..."`
const MSG_ENABLE_IN_SCRIPT: u32 = 0x0103_aef0;
/// Enable: the same "... in a results script ..." message.
const MSG_ENABLE_IN_RESULTS_SCRIPT: u32 = 0x0103_ae48;
/// Disable: the "... in script %08X %s ..." message.
const MSG_DISABLE_IN_SCRIPT: u32 = 0x0103_b040;
/// Disable: the "... in a results script ..." message.
const MSG_DISABLE_IN_RESULTS_SCRIPT: u32 = 0x0103_af98;
/// `"\"%s\" (%08x) is now %s"`
const FORMAT_SEX_CHANGED: u32 = 0x0103_b2b8;
/// `"MODELS: <<< DUMPMODELMAP results"`
const MSG_MODEL_MAP_BEGIN: u32 = 0x0103_b294;
/// `"MODELS: >>> DUMPMODELMAP results"`
const MSG_MODEL_MAP_END: u32 = 0x0103_b270;
/// `"ModelDump%s.txt"`
const FORMAT_MODEL_DUMP: u32 = 0x0103_9690;
/// `"TexDump"`
const TEXT_TEX_DUMP: u32 = 0x0103_b268;
/// `"%s-INT-%s.txt"`
const FORMAT_TEX_DUMP_INTERIOR: u32 = 0x0103_b258;
/// `"%s-%s.%i.%i.txt"`
const FORMAT_TEX_DUMP_EXTERIOR: u32 = 0x0103_b248;
/// `"%s-UNKNOWN.txt"`
const FORMAT_TEX_DUMP_UNKNOWN: u32 = 0x0103_b238;
/// `"D:\TexDump.txt"`
const DEFAULT_TEX_DUMP_PATH: u32 = 0x0103_b228;
/// `"TEXTURES: <<< DUMPTEXTUREPALETTE results"`
const MSG_TEXTURE_PALETTE_BEGIN: u32 = 0x0103_b1fc;
/// `"TEXTURES: Total Textures = %d @ %.2f Mb"`
const MSG_TOTAL_TEXTURES: u32 = 0x0103_b1d4;
/// `"TEXTURES: >>> DUMPTEXTUREPALETTE results"`
const MSG_TEXTURE_PALETTE_END: u32 = 0x0103_b1a8;
/// `"UNNAMED SOURCE TEXTURE"`
const TEXT_UNNAMED_SOURCE_TEXTURE: u32 = 0x0103_9664;
/// `"RENDERED_TEXTURE (%s)"`
const FORMAT_RENDERED_TEXTURE: u32 = 0x0103_964c;
/// `"RENDERED_TEXTURE (none)"`
const TEXT_RENDERED_TEXTURE_NONE: u32 = 0x0103_9634;
/// `"TEXTURES: %d:\t%s\t%s\t%dx%d\t%d\t%s\trefcount %d\t%s\t%s\r\n"`
const FORMAT_TEXTURE_LINE: u32 = 0x0103_952c;
/// `"L"`
const TEXT_L: u32 = 0x0103_9564;
/// `"H"`
const TEXT_H: u32 = 0x0103_9560;
// The path fragments the textures are grouped by.
/// `"\armor\"`
const PATH_ARMOR: u32 = 0x0103_9624;
/// `"\creatures\"`
const PATH_CREATURES: u32 = 0x0103_9618;
/// `"\characters\"`
const PATH_CHARACTERS: u32 = 0x0103_9608;
/// `"\pipboy3000\"`
const PATH_PIPBOY: u32 = 0x0103_95f8;
/// `"\weapons\"`
const PATH_WEAPONS: u32 = 0x0103_95ec;
/// `"\decals\"`
const PATH_DECALS: u32 = 0x0103_95e0;
/// `"\projectiles\"`
const PATH_PROJECTILES: u32 = 0x0103_95d0;
/// `"\fonts\"`
const PATH_FONTS: u32 = 0x0103_95c0;
/// `"\interface\"`
const PATH_INTERFACE: u32 = 0x0103_95b4;
/// `"\effects\"`
const PATH_EFFECTS: u32 = 0x0103_95a0;
/// `"\gore\"`
const PATH_GORE: u32 = 0x0103_9598;
/// `"\sky\"`
const PATH_SKY: u32 = 0x0103_9588;
/// `"\lod\"`
const PATH_LOD: u32 = 0x0103_957c;
/// `"\water\"`
const PATH_WATER: u32 = 0x0103_9570;
/// `"SYSTEM"`
const CATEGORY_SYSTEM: u32 = 0x0103_962c;
/// `"ACTOR"`
const CATEGORY_ACTOR: u32 = 0x0103_95c8;
/// `"MENUS"`
const CATEGORY_MENUS: u32 = 0x0103_95ac;
/// `"EFFECTS"`
const CATEGORY_EFFECTS: u32 = 0x0103_9590;
/// `"SKY"`
const CATEGORY_SKY: u32 = 0x0103_9584;
/// `"LOD"`
const CATEGORY_LOD: u32 = 0x0103_9578;
/// `"WATER"`
const CATEGORY_WATER: u32 = 0x0102_3014;
/// `"MISCREF"`
const CATEGORY_MISCREF: u32 = 0x0103_9568;

// ---- Small helpers -------------------------------------------------------------

/// `Script::ParseParameters` (`005accb0`) with the given output addresses
/// after the seven fixed words: its `AL`.
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
fn parse_into<const N: usize>(e: &mut Engine, a: ScriptArgs, init: [u32; N]) -> Option<[u32; N]> {
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

/// Writes the `double` a function-style command returns.
fn set_result(e: &mut Engine, a: ScriptArgs, value: f64) {
    e.mem.set_f64(a.result.addr(), value);
}

/// Copies three dwords (a `NiPoint3`).
fn copy_point3(e: &mut Engine, to: u32, from: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

fn ni_pointer_get(e: &mut Engine, slot: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}

fn echo_enabled(e: &mut Engine) -> bool {
    let tls = e.tls();
    e.mem.u8(tls + TLS_ECHO) != 0
}

/// The warning Enable and Disable log when the reference has an enable
/// state parent: the message "in script" when the running script has a form
/// id, otherwise "in a results script". Arguments of the log call: the
/// message, the reference's id and name, the script's id and name.
fn log_enable_state_parent(e: &mut Engine, a: ScriptArgs, in_script: u32, in_results: u32) {
    let named_script =
        !a.script_obj.is_null() && e.call(GET_FORM_ID, &args![a.script_obj]).u32() != 0;
    let this_name = if e.call(GET_NAME_GUARDED, &args![a.this_obj]).u32() != 0 {
        e.vcall(a.this_obj.addr(), 0x130, &args![]).u32()
    } else {
        e.call(GET_NAME_TEXT, &args![a.this_obj]).u32()
    };
    let script_name = e.vcall(a.script_obj.addr(), 0x130, &args![]).u32();
    let script_id = e.call(GET_FORM_ID, &args![a.script_obj]).u32();
    let this_id = e.call(GET_FORM_ID, &args![a.this_obj]).u32();
    let message = if named_script { in_script } else { in_results };
    e.call(
        LOG_STUB,
        &args![message, this_id, this_name, script_id, script_name],
    );
}

// ---- Translated functions --------------------------------------------------------

// Translated from 005c4240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::MenuModeFunction` (Xbox PDB): parses one argument and returns
/// `Script::GetMenuModeConditionFunction(0, argument, 0, result)`.
pub fn script_menu_mode_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([menu]) = parse_into(e, a, [0]) else {
        return false;
    };
    e.call(GET_MENU_MODE_CONDITION, &args![0u32, menu, 0u32, a.result])
        .bool()
}

// Translated from 005c42a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function-style command: 0.0 when the interface is in menu mode or the
/// byte `011dea2d` is set, otherwise 1.0.
pub fn fn_005c42a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let value = if e.call(IS_IN_MENU_MODE, &args![]).bool() || fn_005c42d0(e) != 0 {
        0.0
    } else {
        1.0
    };
    set_result(e, a, value);
    true
}

// Translated from 005c42d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte global `011dea2d`.
pub fn fn_005c42d0(e: &mut Engine) -> u8 {
    e.global::<u8>(FLAG_011DEA2D)
}

// Translated from 005c42e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function-style command on the event list: 1.0 when the event list has a
/// block at `+0x10` whose first byte is non-zero, otherwise 0.0.
pub fn fn_005c42e0(e: &mut Engine, a: ScriptArgs) -> bool {
    set_result(e, a, 0.0);
    if a.event_list != 0 {
        let block = e.mem.u32(a.event_list + 0x10);
        if block != 0 && e.mem.u8(block) != 0 {
            set_result(e, a, 1.0);
        }
    }
    true
}

// Translated from 005c4320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_005c42e0`] for the second byte of the block.
pub fn fn_005c4320(e: &mut Engine, a: ScriptArgs) -> bool {
    set_result(e, a, 0.0);
    if a.event_list != 0 {
        let block = e.mem.u32(a.event_list + 0x10);
        if block != 0 && e.mem.u8(block + 1) != 0 {
            set_result(e, a, 1.0);
        }
    }
    true
}

// Translated from 005c4360 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function-style command on the running script: 1.0 when the byte at
/// `+0x11` of the block at `script + 0x18` is non-zero, otherwise 0.0.
pub fn fn_005c4360(e: &mut Engine, a: ScriptArgs) -> bool {
    set_result(e, a, 0.0);
    if !a.script_obj.is_null() {
        let block = e.call(PLUS_0X18, &args![a.script_obj]).u32();
        if block != 0 {
            let block = e.call(PLUS_0X18, &args![a.script_obj]).u32();
            if e.mem.u8(block + 0x11) != 0 {
                set_result(e, a, 1.0);
            }
        }
    }
    true
}

// Translated from 005c43a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function-style command on the event list: the `float` at `+4` of the
/// block at `+0x10`, otherwise 0.0.
pub fn fn_005c43a0(e: &mut Engine, a: ScriptArgs) -> bool {
    set_result(e, a, 0.0);
    if a.event_list != 0 {
        let block = e.mem.u32(a.event_list + 0x10);
        if block != 0 {
            let value = e.mem.f32(block + 4);
            set_result(e, a, value as f64);
        }
    }
    true
}

// Translated from 005c43d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::EnableFunction` (Xbox PDB): one optional argument (default 1;
/// when 0 the reference's form flag `0x0800_0000` is set through
/// `0056c780`). A reference with an enable state parent only logs a warning
/// ([`log_enable_state_parent`]). Otherwise the pending delayed actions of
/// the reference are removed, it is queued as pending-enabled when it has
/// form flag `0x800`, a process in interface mode 9 gets the mode of its
/// base form, and bit 8 of its extra data `0x92` is cleared when set. The
/// event list's bit 0 (disabled) is cleared in every case that gets past the
/// parameters.
pub fn script_enable_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag]) = parse_into(e, a, [1]) else {
        return false;
    };
    if flag == 0 {
        e.call(SET_FORM_FLAG_0X8000000, &args![a.this_obj, 1u32]);
    }
    if !a.this_obj.is_null() {
        if e.call(GET_ENABLE_STATE_PARENT, &args![a.this_obj]).u32() != 0 {
            log_enable_state_parent(e, a, MSG_ENABLE_IN_SCRIPT, MSG_ENABLE_IN_RESULTS_SCRIPT);
            return true;
        }
        e.call(REMOVE_DELAYED_SCRIPT_ACTION_REFERENCE, &args![a.this_obj]);
        if e.call(HAS_FORM_FLAG_0X800, &args![a.this_obj]).bool() {
            e.call(ADD_PENDING_ENABLED_REFERENCE, &args![a.this_obj]);
        }
        let process = e.vcall(a.this_obj.addr(), 0x1d0, &args![]).u32();
        let process_object = if process != 0 {
            e.vcall(process, 0x10, &args![]).u32()
        } else {
            0
        };
        if process_object != 0 && e.call(GET_PROCESS_MODE, &args![process_object]).u32() == 9 {
            let base = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
            let mode = e.call(GET_MODE_FOR_FORM, &args![base]).u32();
            e.call(SET_PROCESS_MODE, &args![process_object, mode]);
        }
        if e.call(HAS_EXTRA_FLAG_0X8, &args![a.this_obj]).u8() == 1 {
            e.call(SET_EXTRA_FLAG_0X8, &args![a.this_obj, 0u32]);
        }
    }
    let flags = e.mem.u8(a.event_list + 4);
    e.mem.set_u8(a.event_list + 4, flags & 0xfe);
    true
}

// Translated from 005c45e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::DisableFunction` (Xbox PDB): one optional argument (default 0;
/// the "fade" flag). A reference that has form flag `0x800` or `0x4000` is
/// left alone. One with an enable state parent only logs a warning
/// ([`log_enable_state_parent`]). Otherwise, when the argument is positive,
/// its process gets interface mode 9 and bit 8 of its extra data `0x92` is
/// set when clear; it is then queued as pending-disabled (with the
/// argument as a flag). The event list's bit 0 (disabled) is set in every
/// case that gets past the parameters, except the warning.
pub fn script_disable_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([fade]) = parse_into(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null()
        && !e.call(HAS_FORM_FLAG_0X800, &args![a.this_obj]).bool()
        && !e.call(HAS_FORM_FLAG_0X4000, &args![a.this_obj]).bool()
    {
        if e.call(GET_ENABLE_STATE_PARENT, &args![a.this_obj]).u32() != 0 {
            log_enable_state_parent(e, a, MSG_DISABLE_IN_SCRIPT, MSG_DISABLE_IN_RESULTS_SCRIPT);
            return true;
        }
        if fade as i32 > 0 {
            let process = e.vcall(a.this_obj.addr(), 0x1d0, &args![]).u32();
            let process_object = if process != 0 {
                e.vcall(process, 0x10, &args![]).u32()
            } else {
                0
            };
            if process_object != 0 {
                e.call(SET_PROCESS_MODE, &args![process_object, 9u32]);
            }
            if !e.call(HAS_EXTRA_FLAG_0X8, &args![a.this_obj]).bool() {
                e.call(SET_EXTRA_FLAG_0X8, &args![a.this_obj, 1u32]);
            }
        }
        e.call(
            ADD_PENDING_DISABLED_REFERENCE,
            &args![a.this_obj, fade != 0],
        );
    }
    let flags = e.mem.u8(a.event_list + 4);
    e.mem.set_u8(a.event_list + 4, flags | 1);
    true
}

// Translated from 005c47e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Disables every actor of the high process list (list 3) that is not the
/// running reference (as an `Actor`) and does not have form flag `0x800`:
/// each is queued as pending-disabled with the fade flag set, and the event
/// list's bit 0 is set. Only actors that answer true to virtual slot
/// `0x100` are considered.
pub fn fn_005c47e0(e: &mut Engine, a: ScriptArgs) -> bool {
    let this_actor = e
        .call(
            DYNAMIC_CAST,
            &args![a.this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
        )
        .u32();
    let mut index = 0u32;
    loop {
        // The call sites push the list number (3) before the `this + 4`
        // call; it is the argument of the count function.
        let lists = e.call(PLUS_4, &args![PROCESS_LISTS]).u32();
        let count = e.call(PROCESS_LISTS_COUNT, &args![lists, 3u32]).u32();
        if index >= count {
            break;
        }
        let lists = e.call(PLUS_4, &args![PROCESS_LISTS]).u32();
        let actor = e.call(PROCESS_LISTS_ACTOR, &args![lists, index]).u32();
        if actor != 0
            && e.vcall(actor, 0x100, &args![]).bool()
            && actor != this_actor
            && !e.call(HAS_FORM_FLAG_0X800, &args![actor]).bool()
        {
            e.call(ADD_PENDING_DISABLED_REFERENCE, &args![actor, 1u32]);
            let flags = e.mem.u8(a.event_list + 4);
            e.mem.set_u8(a.event_list + 4, flags | 1);
        }
        index += 1;
    }
    true
}

// Translated from 005c48c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetDisabledConditionFunction(thisObj,
/// event list, 0, result)`; returns its `AL`.
pub fn fn_005c48c0(e: &mut Engine, a: ScriptArgs) -> bool {
    e.call(
        GET_DISABLED_CONDITION,
        &args![a.this_obj, a.event_list, 0u32, a.result],
    )
    .bool()
}

// Translated from 005c48e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetLockedConditionFunction(thisObj, event
/// list, 0, result)`; returns its `AL`.
pub fn fn_005c48e0(e: &mut Engine, a: ScriptArgs) -> bool {
    e.call(
        GET_LOCKED_CONDITION,
        &args![a.this_obj, a.event_list, 0u32, a.result],
    )
    .bool()
}

// Translated from 005c4900 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetLockLevelConditionFunction(thisObj,
/// event list, 0, result)`; returns its `AL`.
pub fn fn_005c4900(e: &mut Engine, a: ScriptArgs) -> bool {
    e.call(
        GET_LOCK_LEVEL_CONDITION,
        &args![a.this_obj, a.event_list, 0u32, a.result],
    )
    .bool()
}

// Translated from 005c4920 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetIsLockBrokenConditionFunction(thisObj, 0,
/// 0, result)`; returns its `AL`.
pub fn fn_005c4920(e: &mut Engine, a: ScriptArgs) -> bool {
    e.call(
        GET_IS_LOCK_BROKEN_CONDITION,
        &args![a.this_obj, 0u32, 0u32, a.result],
    )
    .bool()
}

// Translated from 005c4940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsFormTypeFunction` (Xbox PDB): parses one string argument
/// (a form type name); the result is 1.0 when it is the name of the type of
/// the reference's base form (`strcmp` of the table entry `011870 04 + 12 *
/// type`). With the console echo on, the result is printed. The game's
/// 0x204-byte buffer is uninitialised stack when the argument does not
/// parse (zeroed here, and not used then).
pub fn script_get_is_form_type_function(e: &mut Engine, a: ScriptArgs) -> bool {
    set_result(e, a, 0.0);
    let buffer = e.mem.alloc(0x204);
    if !parse(e, a, &[buffer]) {
        e.mem.free(buffer);
        return false;
    }
    let base = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
    let form_type = e.call(FORM_TYPE, &args![base]).u32();
    let name = e.mem.u32(FORM_TYPE_NAMES + form_type * 12);
    if e.call(STRING_COMPARE, &args![name, buffer]).u32() == 0 {
        set_result(e, a, 1.0);
    }
    if echo_enabled(e) {
        let value = e.mem.f64(a.result.addr());
        console_print(e, &args![MSG_GET_IS_FORM_TYPE, value]);
    }
    e.mem.free(buffer);
    true
}

// Translated from 005c4a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsVoiceTypeFunction` (Xbox PDB): parses one argument and
/// returns `Script::GetIsVoiceTypeConditionFunction(thisObj, argument, 0,
/// result)`.
pub fn script_get_is_voice_type_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([voice_type]) = parse_into(e, a, [0]) else {
        return false;
    };
    e.call(
        GET_IS_VOICE_TYPE_CONDITION,
        &args![a.this_obj, voice_type, 0u32, a.result],
    )
    .bool()
}

// Translated from 005c4a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The PlaceAtMe command body: arguments are a form, a count (default 1), a
/// distance and a direction. The result starts at 0.0; the reference
/// [`fn_005c4b30`] placed (health scale 1.0) has its form id stored in the
/// result through `Script::PutNumericIDInDouble`.
pub fn fn_005c4a70(e: &mut Engine, a: ScriptArgs) -> bool {
    set_result(e, a, 0.0);
    let Some([form, count, distance, direction]) = parse_into(e, a, [0, 1, 0, 0]) else {
        return false;
    };
    let reference = fn_005c4b30(
        e,
        a.this_obj,
        Ptr::new(form),
        count as i32,
        distance as i32,
        direction as i32,
        1.0,
    );
    if !reference.is_null() {
        let id = e.call(GET_FORM_ID, &args![reference]).u32();
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), id);
            e.call(PUT_NUMERIC_ID_IN_DOUBLE, &args![cell, a.result]);
        });
    }
    true
}

// Offsets of the locals in the scratch block of `fn_005c4b30`.
/// The reference's rotation (3 floats).
const PLACE_ROTATION: u32 = 0x000;
/// The position the form is created at (3 floats).
const PLACE_POSITION: u32 = 0x010;
/// The offset by distance and direction (3 floats).
const PLACE_OFFSET: u32 = 0x020;
/// The scene node's world rotation (9 floats).
const PLACE_MATRIX: u32 = 0x030;
/// The nine positions: the centre and the ring around it (9 x 3 floats).
const PLACE_RING: u32 = 0x060;
/// Temporaries of the ring (3 floats each).
const PLACE_RING_SCALED: u32 = 0x0d0;
const PLACE_RING_DIRECTION: u32 = 0x0e0;
const PLACE_RING_SUM: u32 = 0x0f0;
/// The temporary container of the leveled-list branch (0x10 bytes).
const PLACE_CONTAINER: u32 = 0x100;
/// The pick (0xbc bytes) and its mask.
const PLACE_PICK: u32 = 0x110;
const PLACE_PICK_MASK: u32 = 0x1d0;
/// Pick start and end (3 floats each), the "has a hit" byte.
const PLACE_PICK_FROM: u32 = 0x1e0;
const PLACE_PICK_TO: u32 = 0x1f0;
const PLACE_PICK_HIT: u32 = 0x200;
/// Temporaries of the pick branch (3 floats each).
const PLACE_PICK_NORMAL: u32 = 0x210;
const PLACE_PICK_NORMAL_SCALED: u32 = 0x220;
const PLACE_PICK_RESULT: u32 = 0x230;
/// Size of the scratch block.
const PLACE_FRAME_SIZE: u32 = 0x240;

// Translated from 005c4b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The PlaceAtMe worker, `cdecl` (`reference, form, count, distance,
/// direction, health scale`): creates `count` references of `form` around
/// `reference` and returns the last one (0 when none).
///
/// - `distance` and `direction` move the centre: from the reference's 3D
///   node the position is moved `distance` along column 0 (directions 2 and
///   3) or column 1 (the others) of its world rotation, subtracted for
///   directions 1 and 2, added otherwise. That centre is the position of
///   the leveled-list branch and the centre of the ring; the first
///   reference of the plain branch is, as the game does, created at the
///   reference's own position.
/// - A ring of nine positions is computed: the centre and eight points at a
///   radius of 100 starting at the angle of `005c5420`, 45 degrees apart.
/// - Forms of type `0x2c` and `0x2d` (leveled lists) are rolled into a
///   temporary container at the reference's level (`count` truncated to 16
///   bits), and every item of it is created, as many times as its count, at
///   the centre.
/// - Any other form that passes virtual slot `0xe4` is created `count`
///   times, the first at the reference's position and the following at the
///   ring positions (in order, wrapping after nine) when they are inside the
///   loaded area; a ray from 64 above the reference to 64 above the ring
///   position moves the position back along the ray by the form's bound size
///   when it hits something, and the height is the reference's. A form with
///   a health form gets its health set to its virtual slot `0x10` value
///   times the health scale. When a creation fails the function returns 0.
/// - Actors created get their virtual slot `0x41c` (0.0) and `Actor::FadeIn`.
pub fn fn_005c4b30(
    e: &mut Engine,
    this_obj: Ptr,
    form: Ptr,
    count: i32,
    distance: i32,
    direction: i32,
    health_scale: f32,
) -> Ptr {
    if this_obj.is_null() || form.is_null() {
        return Ptr::NULL;
    }
    let this_addr = this_obj.addr();
    let form_addr = form.addr();
    let placed = e.with_stack(PLACE_FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let rotation = frame + PLACE_ROTATION;
        let position = frame + PLACE_POSITION;
        let offset = frame + PLACE_OFFSET;
        let matrix = frame + PLACE_MATRIX;
        let ring = frame + PLACE_RING;
        let mut placed = 0u32;

        let source = e.call(GET_ROTATION, &args![this_addr]).u32();
        copy_point3(e, rotation, source);
        let source = e.vcall(this_addr, 0x1f4, &args![]).u32();
        copy_point3(e, position, source);

        if distance != 0 {
            let node = e.vcall(this_addr, 0x1d0, &args![]).u32();
            if node != 0 {
                e.call(POINT3_CONSTRUCT, &args![offset, 0f32, 0f32, 0f32]);
                let world_rotate = e.call(GET_WORLD_ROTATE, &args![node]).u32();
                for i in 0..9 {
                    let word = e.mem.u32(world_rotate + 4 * i);
                    e.mem.set_u32(matrix + 4 * i, word);
                }
                let world_translate = e.call(GET_WORLD_TRANSLATE, &args![node]).u32();
                copy_point3(e, position, world_translate);
                let column = if (2..=3).contains(&direction) {
                    0u32
                } else {
                    1u32
                };
                for axis in 0..3u32 {
                    let value = e.call(MATRIX_ELEMENT, &args![matrix, axis, column]).f32();
                    e.mem.set_f32(offset + 4 * axis, value);
                }
                e.call(POINT3_SCALE, &args![offset, distance as f32]);
                if (1..=2).contains(&direction) {
                    e.call(POINT3_SUBTRACT_ASSIGN, &args![position, offset]);
                } else {
                    e.call(POINT3_ADD_ASSIGN, &args![position, offset]);
                }
            }
        }

        // The nine positions: the centre, then eight around it.
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![ring, 12u32, 9u32, POINT3_DEFAULT_CONSTRUCT],
        );
        let mut slot = 0u32;
        copy_point3(e, ring + 12 * slot, position);
        slot += 1;
        let step = e.global::<f64>(RING_ANGLE_STEP);
        let start = fn_005c5420(e);
        let mut angle = (start as f64 * step) as f32;
        for _ in 0..8 {
            let radius = e.global::<f32>(RING_RADIUS);
            let y = fn_005c53d0(e, angle);
            let x = e.call(RING_X, &args![angle]).f32();
            let direction_vector = frame + PLACE_RING_DIRECTION;
            let scaled = frame + PLACE_RING_SCALED;
            e.call(POINT3_CONSTRUCT, &args![direction_vector, x, y, 0f32]);
            let scaled = e
                .call(POINT3_SCALED, &args![direction_vector, scaled, radius])
                .u32();
            let sum = e
                .call(POINT3_SUM, &args![position, frame + PLACE_RING_SUM, scaled])
                .u32();
            copy_point3(e, ring + 12 * slot, sum);
            slot += 1;
            angle = (angle as f64 + step) as f32;
        }

        let form_type = e.call(FORM_TYPE, &args![form_addr]).u32();
        if form_type == 0x2c || form_type == 0x2d {
            let level = e.call(GET_CALC_LEVEL, &args![this_addr, 0u32]).u32();
            let container = frame + PLACE_CONTAINER;
            e.call(TES_CONTAINER_CONSTRUCT, &args![container]);
            // Both form types roll the list at `form + 0x30`.
            e.call(
                ROLL_LEVELED_LIST,
                &args![
                    form_addr + 0x30,
                    level & 0xffff,
                    (count as u32) & 0xffff,
                    container,
                    0u32
                ],
            );
            let mut node = e.call(PLUS_4, &args![container]).u32();
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let item_slot = e.call(LIST_ITEM_PTR, &args![node]).u32();
                let entry = e.mem.u32(item_slot);
                if entry != 0 && e.mem.u32(entry + 4) != 0 {
                    while e.mem.i32(entry) > 0 {
                        let world_space = e.call(GET_WORLD_SPACE, &args![this_addr]).u32();
                        let cell = e.call(GET_PARENT_CELL, &args![this_addr]).u32();
                        let item_form = e.mem.u32(entry + 4);
                        let data_handler = e.global::<u32>(DATA_HANDLER);
                        placed = e
                            .call(
                                CREATE_REFERENCE,
                                &args![
                                    data_handler,
                                    item_form,
                                    position,
                                    rotation,
                                    cell,
                                    world_space,
                                    0u32,
                                    0u32,
                                    0u32
                                ],
                            )
                            .u32();
                        if placed != 0 && e.vcall(placed, 0x100, &args![]).bool() {
                            e.vcall(placed, 0x41c, &args![0f32]);
                            e.call(FADE_IN, &args![placed]);
                        }
                        let remaining = e.mem.i32(entry) - 1;
                        e.mem.set_i32(entry, remaining);
                    }
                }
                node = e.call(LIST_NEXT, &args![node]).u32();
            }
            e.call(TES_CONTAINER_DESTRUCT, &args![container]);
            return placed;
        }

        if !e.vcall(form_addr, 0xe4, &args![]).bool() {
            return placed;
        }
        let health_form = e.call(GET_FORM_AS_HEALTH_FORM, &args![form_addr]).u32();
        let tes = e.global::<u32>(TES);
        let own_position = e.vcall(this_addr, 0x1f4, &args![]).u32();
        let far = !e.call(IS_POSITION_LOADED, &args![tes, own_position]).bool();
        let mut ring_index = 0u32;
        for _ in 0..count {
            let usable = ring_index != 0
                && !far
                && e.call(IS_POSITION_LOADED, &args![tes, ring + 12 * ring_index])
                    .bool();
            if usable {
                let pick = frame + PLACE_PICK;
                let mask = frame + PLACE_PICK_MASK;
                let from = frame + PLACE_PICK_FROM;
                let to = frame + PLACE_PICK_TO;
                let hit_byte = frame + PLACE_PICK_HIT;
                e.call(PICK_CONSTRUCT, &args![pick]);
                e.call(STORE_VALUE, &args![mask, 0u32]);
                e.call(MASK_SET_LOW_BITS, &args![mask, 0x26u32]);
                e.call(MASK_SET_HIGH_BITS, &args![mask, 3u32]);
                let mask_value = e.mem.u32(mask);
                e.call(PICK_SET_MASK, &args![pick, mask_value]);
                let source = e.vcall(this_addr, 0x1f4, &args![]).u32();
                copy_point3(e, from, source);
                let height = e.global::<f64>(PICK_HEIGHT);
                let from_z = (e.mem.f32(from + 8) as f64 + height) as f32;
                e.mem.set_f32(from + 8, from_z);
                copy_point3(e, to, ring + 12 * ring_index);
                let to_z = (e.mem.f32(to + 8) as f64 + height) as f32;
                e.mem.set_f32(to + 8, to_z);
                e.call(PICK_SET_START, &args![pick, from]);
                e.call(PICK_SET_END, &args![pick, to]);
                e.call(TES_PICK, &args![tes, pick]);
                let flag = fn_005c53f0(e, Ptr::new(pick + 0x30), hit_byte);
                if e.call(BYTE_IS_SET, &args![flag]).bool() {
                    let normal = frame + PLACE_PICK_NORMAL;
                    e.call(POINT3_DIFFERENCE, &args![to, normal, from]);
                    e.call(POINT3_UNITIZE, &args![normal]);
                    let bound_size = e.call(GET_BOUND_SIZE, &args![form_addr]).f32();
                    let scaled = e
                        .call(
                            POINT3_SCALED,
                            &args![normal, frame + PLACE_PICK_NORMAL_SCALED, bound_size],
                        )
                        .u32();
                    let result = e
                        .call(
                            POINT3_DIFFERENCE,
                            &args![to, frame + PLACE_PICK_RESULT, scaled],
                        )
                        .u32();
                    copy_point3(e, position, result);
                } else {
                    copy_point3(e, position, to);
                }
                let source = e.vcall(this_addr, 0x1f4, &args![]).u32();
                let z = e.mem.f32(source + 8);
                e.mem.set_f32(position + 8, z);
            } else {
                let source = e.vcall(this_addr, 0x1f4, &args![]).u32();
                copy_point3(e, position, source);
            }
            let world_space = e.call(GET_WORLD_SPACE, &args![this_addr]).u32();
            let cell = e.call(GET_PARENT_CELL, &args![this_addr]).u32();
            let data_handler = e.global::<u32>(DATA_HANDLER);
            placed = e
                .call(
                    CREATE_REFERENCE,
                    &args![
                        data_handler,
                        form_addr,
                        position,
                        rotation,
                        cell,
                        world_space,
                        0u32,
                        0u32,
                        0u32
                    ],
                )
                .u32();
            if placed == 0 {
                return 0;
            }
            if e.vcall(placed, 0x100, &args![]).bool() {
                e.vcall(placed, 0x41c, &args![0f32]);
                e.call(FADE_IN, &args![placed]);
            }
            if health_form != 0 {
                let health = e.vcall(health_form, 0x10, &args![]).u32();
                let value = health as f32 * health_scale;
                e.call(SET_HEALTH, &args![placed, value]);
            }
            ring_index += 1;
            if ring_index == 9 {
                ring_index = 0;
            }
        }
        placed
    });
    Ptr::new(placed)
}

// Translated from 005c53d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` (`angle`): the trigonometric wrapper `005c53d0` -> `004e44b0`;
/// returns its `float` result.
pub fn fn_005c53d0(e: &mut Engine, angle: f32) -> f32 {
    e.call(RING_Y, &args![angle]).f32()
}

// Translated from 005c53f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` on a pick result (`byte address`): stores whether `this +
/// 0x50` is non-zero into the byte and returns the byte's address.
pub fn fn_005c53f0(e: &mut Engine, this: Ptr, byte_address: u32) -> u32 {
    let has_hit = e.mem.u32(this.addr() + 0x50) != 0;
    e.call(STORE_BYTE, &args![byte_address, has_hit]);
    byte_address
}

// Translated from 005c5420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The starting angle of the placement ring: `004dff60` on the object
/// `00476c00` returns, a `float`.
pub fn fn_005c5420(e: &mut Engine) -> f32 {
    let object = e.call(RANDOM_OBJECT, &args![]).u32();
    e.call(RANDOM_ANGLE_FROM, &args![object]).f32()
}

// Translated from 005c5440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SaveGame` (Xbox PDB): arguments are a name (string) and a flag;
/// `BGSSaveLoadManager::SaveGame(name, -1, flag != 0)`.
pub fn script_save_game(e: &mut Engine, a: ScriptArgs) -> bool {
    let name = e.mem.alloc(0x204);
    let flag = e.mem.alloc(4);
    let ok = parse(e, a, &[name, flag]);
    let flag_value = e.mem.u32(flag);
    e.mem.free(flag);
    if ok {
        let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
        e.call(
            SAVE_GAME,
            &args![manager, name, 0xffff_ffffu32, flag_value != 0],
        );
    }
    e.mem.free(name);
    ok
}

// Translated from 005c54d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`script_save_game`] through `00850760(name, -1, flag != 0, 1)`.
pub fn fn_005c54d0(e: &mut Engine, a: ScriptArgs) -> bool {
    let name = e.mem.alloc(0x204);
    let flag = e.mem.alloc(4);
    let ok = parse(e, a, &[name, flag]);
    let flag_value = e.mem.u32(flag);
    e.mem.free(flag);
    if ok {
        let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
        e.call(
            SAVE_GAME_WITH_FLAG,
            &args![manager, name, 0xffff_ffffu32, flag_value != 0, 1u32],
        );
    }
    e.mem.free(name);
    ok
}

// Translated from 005c5560 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the value byte of the setting `011de29c` is set, calls `00851d30` on
/// the save/load manager.
pub fn fn_005c5560(e: &mut Engine) -> bool {
    let value = e.call(GET_SETTING_VALUE_ADDRESS, &args![SETTING_A]).u32();
    if e.mem.u8(value) != 0 {
        let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
        e.call(MANAGER_ACTION_A, &args![manager]);
    }
    true
}

// Translated from 005c5590 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the value byte of the setting `011de2b8` is set and the manager
/// answers `IsDetailedPathHandler(-1)`, calls `00851d50` on the save/load
/// manager.
pub fn fn_005c5590(e: &mut Engine) -> bool {
    let value = e.call(GET_SETTING_VALUE_ADDRESS, &args![SETTING_B]).u32();
    if e.mem.u8(value) != 0 {
        let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
        if e.call(IS_DETAILED_PATH_HANDLER, &args![manager, 0xffff_ffffu32])
            .bool()
        {
            e.call(MANAGER_ACTION_B, &args![manager]);
        }
    }
    true
}

// Translated from 005c55d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::CopySaveGames` (Xbox PDB): one string argument (default `""`).
/// With a save/load manager, `CopySaveGames(manager, argument == "ms")`
/// (case-insensitive).
pub fn script_copy_save_games(e: &mut Engine, a: ScriptArgs) -> bool {
    let argument = e.mem.alloc(0x204);
    e.call(STRING_COPY, &args![argument, EMPTY_STRING]);
    let ok = parse(e, a, &[argument]);
    if ok {
        let manager = e.global::<u32>(SAVE_LOAD_MANAGER);
        if manager != 0 {
            let different = e
                .call(STRING_COMPARE_NO_CASE, &args![argument, TEXT_MS])
                .u32();
            e.call(COPY_SAVE_GAMES, &args![manager, different == 0]);
        }
    }
    e.mem.free(argument);
    ok
}

// Translated from 005c5680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleCellNode` (Xbox PDB): one argument, the node index
/// (0 Actor, 1 Marker, 2 Land, 3 Static, 4 Dynamic, 5 MultiBound, 6 Water;
/// default -1 does nothing). An index above 6 prints "Invalid node index".
/// Otherwise the node kind's flag is read (`005468f0`) and passed as the
/// state byte to `00456d00` on the `TES` object, for Water the path handler
/// is refreshed (`004e6370`), and "CULLED" (state 0) or "DISPLAYED" is
/// printed with the node name.
pub fn script_toggle_cell_node(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([index]) = parse_into(e, a, [0xffff_ffff]) else {
        return false;
    };
    if index as i32 != -1 {
        let name = match index {
            0 => NODE_ACTOR,
            1 => NODE_MARKER,
            2 => NODE_LAND,
            3 => NODE_STATIC,
            4 => NODE_DYNAMIC,
            5 => NODE_MULTI_BOUND,
            6 => NODE_WATER,
            _ => 0,
        };
        if name == 0 {
            console_print(e, &args![MSG_INVALID_NODE_INDEX]);
        } else {
            let displayed = e.call(GET_CELL_NODE_FLAG, &args![index]).u8();
            let tes = e.global::<u32>(TES);
            e.call(APPLY_CELL_NODE_STATE, &args![tes, index, displayed, 0u32]);
            if index == 6 && e.call(GET_CURRENT_NODE_INDEX, &args![tes]).u32() != 0 {
                // The call site pushes the three arguments of `004e6370`
                // before this plain `ret` accessor.
                let handler = e.call(GET_CURRENT_NODE_INDEX, &args![tes]).u32();
                e.call(REFRESH_PATH_NODES, &args![handler, displayed, 0u32, 0u32]);
            }
            let state = if displayed == 0 {
                TEXT_CULLED
            } else {
                TEXT_DISPLAYED
            };
            console_print(e, &args![MSG_CELL_NODES, name, state]);
        }
    }
    true
}

// Translated from 005c57f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleTrees` (Xbox PDB): flips whether all trees are culled and
/// prints the new state ("CULLED" or "NOT CULLED").
pub fn script_toggle_trees(e: &mut Engine) -> bool {
    let culled = e.call(GET_TREES_CULLED, &args![]).bool();
    e.call(SET_TREES_CULLED, &args![!culled]);
    let text = if e.call(GET_TREES_CULLED, &args![]).bool() {
        TEXT_NOT_CULLED
    } else {
        TEXT_CULLED
    };
    console_print(e, &args![MSG_ALL_TREES, text]);
    true
}

// Translated from 005c5840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetCameraFOV` (Xbox PDB): two float arguments (the world and the
/// first-person field of view), each limited to 160 (`0040ebd0`), 0 replaced
/// by 75, and a non-positive value `v` turned into `1 / -v`. The world value
/// goes to `BSSceneGraph::SetCameraFOV`, the shader manager, the player
/// (`+0x670`) and the water singleton `0120315c`; the second to the player
/// (`+0x674`) and the water singleton `01203168`.
pub fn script_set_camera_fov(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([world_bits, first_person_bits]) = parse_into(e, a, [0f32.to_bits(), 0f32.to_bits()])
    else {
        return false;
    };
    let limit = e.global::<f32>(FOV_LIMIT);
    let zero = e.global::<f64>(ZERO_DOUBLE);
    let default_fov = e.global::<f32>(FOV_DEFAULT);
    let mut fov = [
        f32::from_bits(world_bits),
        f32::from_bits(first_person_bits),
    ];
    for value in fov.iter_mut() {
        *value = e.call(MINIMUM, &args![*value, limit]).f32();
        if *value as f64 == zero {
            *value = default_fov;
        }
    }
    let converted = fov.map(|value| {
        let value = value as f64;
        (if value > zero { value } else { 1.0 / -value }) as f32
    });
    let [world, first_person] = converted;
    let scene_graph = e.call(GET_SCENE_GRAPH, &args![]).u32();
    e.call(SET_CAMERA_FOV, &args![scene_graph, world, 0u32, 0u32, 0u32]);
    e.call(SET_SHADER_FOV, &args![world]);
    let player = e.global::<u32>(PLAYER);
    fn_005c59c0(e, Ptr::new(player), world);
    fn_005c59e0(e, Ptr::new(player), first_person);
    e.call(SET_WATER_FOV, &args![WATER_FOV_X, world]);
    e.call(SET_WATER_FOV, &args![WATER_FOV_Y, first_person]);
    true
}

// Translated from 005c59c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` (`float`): stores it at `this + 0x670`.
pub fn fn_005c59c0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x670, x87_quiet(value));
}

// Translated from 005c59e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` (`float`): stores it at `this + 0x674`.
pub fn fn_005c59e0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x674, x87_quiet(value));
}

// Translated from 005c5a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback `005c5a20` installs: prints the text on the console (the
/// text passes through the identity function `00464f30`).
pub fn fn_005c5a00(e: &mut Engine, text: u32) {
    let text = e.call(IDENTITY, &args![text, 0u32]).u32();
    console_print(e, &args![text]);
}

// Translated from 005c5a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00b557d0` (the shader manager dump) with the console print
/// [`fn_005c5a00`] installed as the callback in the global `011f91f4`, then
/// restores the previous callback.
pub fn fn_005c5a20(e: &mut Engine) -> bool {
    let previous = fn_005c5a50(e);
    fn_005c5a60(e, PRINT_CALLBACK);
    e.call(SHADER_MANAGER_DUMP, &args![]);
    fn_005c5a60(e, previous);
    true
}

// Translated from 005c5a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the callback global `011f91f4`.
pub fn fn_005c5a50(e: &mut Engine) -> u32 {
    e.global::<u32>(CALLBACK_GLOBAL)
}

// Translated from 005c5a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the callback global `011f91f4`.
pub fn fn_005c5a60(e: &mut Engine, value: u32) {
    e.set_global(CALLBACK_GLOBAL, value);
}

// Translated from 005c5a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::DumpTexturePalette` (Xbox PDB): writes the list of live textures
/// to a file.
///
/// The one optional string argument is the file name. When it is empty (or
/// the command ran without parameters) the name is `TexDump-INT-<mover
/// name>.txt` when the `TES` object has an interior (`005f36f0`),
/// `TexDump-<world space name>.<x>.<y>.txt` for an exterior, and
/// `TexDump-UNKNOWN.txt` otherwise; if still empty it is `D:\TexDump.txt`.
/// Every live texture is inserted into a table of 0x800 `NiPointer`s ordered
/// by size (the size function is the stub `005b5e40`, so every size is 0),
/// then one tab-separated line is written per table entry (index, category
/// by path, description, width x height, size in KB, format, reference count
/// minus one, "L" or "H", and the owner's name).
pub fn script_dump_texture_palette(e: &mut Engine, a: ScriptArgs) -> bool {
    let name = e.mem.alloc(0x200);
    e.mem.set_u8(name, 0);
    if a.param_info != 0 && !parse(e, a, &[name]) {
        e.mem.free(name);
        return false;
    }
    if e.mem.i8(name) == 0 {
        let tes = e.global::<u32>(TES);
        let interior = e.call(GET_INTERIOR_CELL, &args![tes]).u32();
        if interior != 0 {
            let interior_name = e.vcall(interior, 0x130, &args![]).u32();
            e.call(
                SPRINTF_S,
                &args![
                    name,
                    0x200u32,
                    FORMAT_TEX_DUMP_INTERIOR,
                    TEXT_TEX_DUMP,
                    interior_name
                ],
            );
        } else {
            let world_space = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
            if world_space != 0 {
                let grid_y = e.call(TES_GRID_Y, &args![tes]).u32();
                let grid_x = e.call(TES_GRID_X, &args![tes]).u32();
                let world_name = e.vcall(world_space, 0x130, &args![]).u32();
                e.call(
                    SPRINTF_S,
                    &args![
                        name,
                        0x200u32,
                        FORMAT_TEX_DUMP_EXTERIOR,
                        TEXT_TEX_DUMP,
                        world_name,
                        grid_x,
                        grid_y
                    ],
                );
            } else {
                e.call(
                    SPRINTF_S,
                    &args![name, 0x200u32, FORMAT_TEX_DUMP_UNKNOWN, TEXT_TEX_DUMP],
                );
            }
        }
    }
    if e.mem.i8(name) == 0 {
        e.call(STRING_COPY, &args![name, DEFAULT_TEX_DUMP_PATH]);
    }
    e.call(INC_DISABLE_WARNING_COUNT, &args![1u32]);
    e.call(LOG_STUB, &args![MSG_TEXTURE_PALETTE_BEGIN]);

    let description = e.mem.alloc(0x104);
    let line = e.mem.alloc(0x200);
    let file = e.mem.alloc(BS_FILE_SIZE);
    let table = e.mem.alloc(4 * TEXTURE_SLOTS);
    e.call(BS_FILE_CONSTRUCT, &args![file, name, 1u32, 0x4000u32, 0u32]);
    e.call(BS_FILE_OPEN, &args![file, 0u32, 0u32]);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            table,
            4u32,
            TEXTURE_SLOTS,
            NI_POINTER_CONSTRUCT,
            NI_POINTER_DESTRUCT
        ],
    );
    e.call(ZERO_FILL, &args![table, 0u32, 4 * TEXTURE_SLOTS]);

    // Insert every texture before the first table entry that is empty or
    // smaller than it, shifting the entries above up by one.
    let mut total_size = 0u32;
    let mut texture_count = 0i32;
    let mut texture = e.call(FIRST_TEXTURE, &args![]).u32();
    while texture != 0 {
        texture_count += 1;
        if e.call(GET_D3D_TEXTURE, &args![texture]).u32() != 0 {
            let size = e.call(LOG_STUB, &args![texture]).u32();
            total_size = total_size.wrapping_add(size);
            let mut insert_at = -1i32;
            let mut i = 0i32;
            while i < TEXTURE_SLOTS as i32 && insert_at == -1 {
                let slot = table + 4 * i as u32;
                if ni_pointer_get(e, slot) == 0 {
                    insert_at = i;
                } else {
                    let occupant = ni_pointer_get(e, slot);
                    if e.call(LOG_STUB, &args![occupant]).u32() < size {
                        insert_at = i;
                    }
                }
                i += 1;
            }
            if insert_at != -1 {
                let mut j = TEXTURE_SLOTS as i32 - 1;
                while j > insert_at {
                    let to = table + 4 * j as u32;
                    e.call(NI_POINTER_ASSIGN, &args![to, to - 4]);
                    j -= 1;
                }
                e.call(
                    NI_POINTER_SET,
                    &args![table + 4 * insert_at as u32, texture],
                );
            }
        }
        texture = e.call(NEXT_TEXTURE, &args![texture]).u32();
    }
    let megabytes = total_size as f64 / e.global::<f64>(MEGABYTE);
    e.call(
        LOG_STUB,
        &args![MSG_TOTAL_TEXTURES, texture_count, megabytes],
    );

    for i in 0..TEXTURE_SLOTS {
        let slot = table + 4 * i;
        if ni_pointer_get(e, slot) == 0 {
            continue;
        }
        let mut rendered = false;
        let candidate = ni_pointer_get(e, slot);
        if e.call(TEXTURE_IS_SOURCE, &args![TEXTURE_CLASS, candidate])
            .bool()
        {
            let texture = ni_pointer_get(e, slot);
            let source = e.vcall(texture, 0x9c, &args![]).u32();
            let mut text = e.call(NAME_TEXT, &args![source]).u32();
            if text == 0 || e.mem.i8(text) == 0 {
                let texture = ni_pointer_get(e, slot);
                let source = e.call(FN_00413F40, &args![texture]).u32();
                text = e.call(NAME_TEXT, &args![source]).u32();
            }
            if text == 0 || e.mem.i8(text) == 0 {
                e.call(
                    STRCPY_S,
                    &args![description, 0x104u32, TEXT_UNNAMED_SOURCE_TEXTURE],
                );
            } else {
                e.call(STRCPY_S, &args![description, 0x104u32, text]);
            }
        } else {
            rendered = true;
            let texture = ni_pointer_get(e, slot);
            let source = e.call(FN_00413F40, &args![texture]).u32();
            let text = e.call(NAME_TEXT, &args![source]).u32();
            if text != 0 && e.mem.i8(text) != 0 {
                e.call(
                    SPRINTF_S,
                    &args![description, 0x104u32, FORMAT_RENDERED_TEXTURE, text],
                );
            } else {
                e.call(
                    SPRINTF_S,
                    &args![description, 0x104u32, TEXT_RENDERED_TEXTURE_NONE],
                );
            }
        }
        e.call(FIX_PATH, &args![description, 0x104u32]);
        let mut category = CATEGORY_SYSTEM;
        if !rendered {
            let contains = |e: &mut Engine, needle: u32| {
                e.call(FIND_SUBSTRING, &args![description, needle]).u32() != 0
            };
            category = if [
                PATH_ARMOR,
                PATH_CREATURES,
                PATH_CHARACTERS,
                PATH_PIPBOY,
                PATH_WEAPONS,
                PATH_DECALS,
                PATH_PROJECTILES,
            ]
            .into_iter()
            .any(|needle| contains(e, needle))
            {
                CATEGORY_ACTOR
            } else if contains(e, PATH_FONTS) || contains(e, PATH_INTERFACE) {
                CATEGORY_MENUS
            } else if contains(e, PATH_EFFECTS) || contains(e, PATH_GORE) {
                CATEGORY_EFFECTS
            } else if contains(e, PATH_SKY) {
                CATEGORY_SKY
            } else if contains(e, PATH_LOD) {
                CATEGORY_LOD
            } else if contains(e, PATH_WATER) {
                CATEGORY_WATER
            } else {
                CATEGORY_MISCREF
            };
        }
        let texture = ni_pointer_get(e, slot);
        let size = e.call(LOG_STUB, &args![texture]).u32();
        let texture = ni_pointer_get(e, slot);
        let d3d_texture = e.call(GET_D3D_TEXTURE, &args![texture]).u32();
        let surface = e.call(FN_007D6BB0, &args![d3d_texture]).u32();
        let format_id = e.call(D3D_FORMAT_ID, &args![surface]).u32();
        let format_name = e.call(GET_D3D_FORMAT_STRING, &args![format_id]).u32();
        let texture = ni_pointer_get(e, slot);
        let owner = e.vcall(texture, 8, &args![]).u32();
        let owner_name = ni_pointer_get(e, owner);
        let texture = ni_pointer_get(e, slot);
        let flag_text = if e.call(TEXTURE_KIND, &args![texture]).u32() == 3 {
            TEXT_L
        } else {
            TEXT_H
        };
        let for_height = ni_pointer_get(e, slot);
        let for_width = ni_pointer_get(e, slot);
        let texture = ni_pointer_get(e, slot);
        let references = e.call(LIST_NEXT, &args![texture]).u32().wrapping_sub(1);
        let height = e.vcall(for_height, 0x98, &args![]).u32();
        let width = e.vcall(for_width, 0x94, &args![]).u32();
        e.call(
            SPRINTF_S,
            &args![
                line,
                0x200u32,
                FORMAT_TEXTURE_LINE,
                i + 1,
                category,
                description,
                width,
                height,
                size >> 10,
                format_name,
                references,
                flag_text,
                owner_name
            ],
        );
        e.call(LOG_STUB, &args![line]);
        let length = e.call(STRLEN, &args![line]).u32();
        e.call(BS_FILE_WRITE, &args![file, line, length + 1]);
    }

    e.call(BS_FILE_CLOSE, &args![file]);
    e.call(LOG_STUB, &args![MSG_TEXTURE_PALETTE_END]);
    e.call(INC_DISABLE_WARNING_COUNT, &args![0u32]);
    e.call(
        VECTOR_DESTRUCT,
        &args![table, 4u32, TEXTURE_SLOTS, NI_POINTER_DESTRUCT],
    );
    e.call(BS_FILE_DESTRUCT, &args![file]);
    for block in [name, description, line, file, table] {
        e.mem.free(block);
    }
    true
}

// Translated from 005c6430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::DumpModelMap` (Xbox PDB): writes the model maps of the model
/// loader and of the face-gen manager to `ModelDump<argument>.txt`. The
/// optional string argument is not parsed when the command runs without
/// parameters (`param_info` 0).
pub fn script_dump_model_map(e: &mut Engine, a: ScriptArgs) -> bool {
    let argument = e.mem.alloc(0x200);
    e.mem.set_u8(argument, 0);
    if a.param_info != 0 && !parse(e, a, &[argument]) {
        e.mem.free(argument);
        return false;
    }
    e.call(INC_DISABLE_WARNING_COUNT, &args![1u32]);
    e.call(LOG_STUB, &args![MSG_MODEL_MAP_BEGIN]);
    let path = e.mem.alloc(0x104);
    e.call(
        SPRINTF_S,
        &args![path, 0x104u32, FORMAT_MODEL_DUMP, argument],
    );
    let file = e.mem.alloc(BS_FILE_SIZE);
    e.call(BS_FILE_CONSTRUCT, &args![file, path, 1u32, 0x4000u32, 0u32]);
    e.call(BS_FILE_OPEN, &args![file, 0u32, 0u32]);
    let model_loader = e.global::<u32>(MODEL_LOADER);
    if model_loader != 0 {
        e.call(
            MODEL_LOADER_OUTPUT_MODEL_MAP,
            &args![model_loader, file, 0u32],
        );
    }
    if e.call(GET_MODEL_CACHE, &args![]).u32() != 0 {
        // The game pushes the file before the second call; it is the
        // argument of the output function.
        let cache = e.call(GET_MODEL_CACHE, &args![]).u32();
        e.call(FACE_GEN_OUTPUT_MODEL_MAP, &args![cache, file]);
    }
    e.call(BS_FILE_CLOSE, &args![file]);
    e.call(LOG_STUB, &args![MSG_MODEL_MAP_END]);
    e.call(INC_DISABLE_WARNING_COUNT, &args![0u32]);
    e.call(BS_FILE_DESTRUCT, &args![file]);
    for block in [argument, path, file] {
        e.mem.free(block);
    }
    true
}

// Translated from 005c65a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sex-change command body: arguments are the wanted sex (default -1,
/// toggle; 1 female, anything else male) and a flag (default 0) that also
/// copies the look of the first NPC of the same race and (new) sex.
///
/// It runs on the reference when that is an actor (virtual slot `0x218`),
/// otherwise on the player. Nothing happens when the actor already has the
/// wanted sex (`0087f4c0`). Otherwise the female bit (bit 1) of the base
/// form's `TESActorBaseData` (`+0x30`) is set or cleared
/// (`TESActorBaseData::SetFlagBit`); for a reference that answers true to
/// virtual slot `0x360`, with the flag, the NPCs of the base's race and sex
/// are collected (`00603280`) and the look of the first is copied
/// (`00603790`); then `Character::Reset3D`. With the console echo on, it
/// prints `"<name>" (<id>) is now <sex>`.
pub fn fn_005c65a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([wanted, copy_look]) = parse_into(e, a, [0xffff_ffff, 0]) else {
        return false;
    };
    let mut actor = e.global::<u32>(PLAYER);
    if !a.this_obj.is_null() && e.vcall(a.this_obj.addr(), 0x218, &args![]).bool() {
        actor = a.this_obj.addr();
    }
    if actor == 0 {
        return true;
    }
    if e.call(ACTOR_GET_SEX, &args![actor]).u32() == wanted {
        return true;
    }
    let base = e.call(GET_BASE_FORM, &args![actor]).u32();
    let process = e.vcall(actor, 0x1d0, &args![]).u32();
    if process != 0 {
        // The result is stored in a local the game never reads.
        e.vcall(process, 0xc, &args![]);
    }
    let mut female = 0u32;
    if wanted as i32 == -1 {
        if e.call(ACTOR_GET_SEX, &args![actor]).u32() == 0 {
            female = 1;
        }
    } else if wanted == 1 {
        female = 1;
    }
    e.call(SET_FLAG_BIT, &args![base + 0x30, 1u32, female == 1, 1u32]);
    if e.vcall(actor, 0x360, &args![]).bool() && copy_look != 0 {
        e.with_stack(0x10, |e, found| {
            let found = found.addr();
            e.call(ARRAY_CONSTRUCT, &args![found]);
            let sex = e.call(GET_SEX, &args![base]).u32();
            let race = e.call(GET_RACE, &args![base]).u32();
            e.call(FIND_NPCS_OF_RACE_AND_SEX, &args![race, sex, found]);
            let first_slot = e.call(ARRAY_ELEMENT, &args![found, 0u32]).u32();
            let first = e.mem.u32(first_slot);
            e.call(COPY_LOOK_FROM, &args![base, first]);
            e.call(ARRAY_DESTRUCT, &args![found]);
        });
    }
    e.call(RESET_3D, &args![actor]);
    if echo_enabled(e) {
        e.with_stack(8, |e, text| {
            e.call(BS_STRING_CONSTRUCT, &args![text]);
            let sex_name = e.global::<u32>(SEX_NAMES + 4 * female);
            let id = e.call(GET_FORM_ID, &args![actor]).u32();
            let name = e.call(GET_NAME_TEXT, &args![actor]).u32();
            e.call(
                BS_STRING_FORMAT,
                &args![text, FORMAT_SEX_CHANGED, name, id, sex_name],
            );
            let message = ni_pointer_get(e, text.addr());
            console_print(e, &args![message]);
            e.call(BS_STRING_DESTRUCT, &args![text]);
        });
    }
    true
}

// Translated from 005c67e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetDiseaseConditionFunction(thisObj, 0, 0,
/// result)`; returns its `AL`.
pub fn fn_005c67e0(e: &mut Engine, a: ScriptArgs) -> bool {
    e.call(
        GET_DISEASE_CONDITION,
        &args![a.this_obj, 0u32, 0u32, a.result],
    )
    .bool()
}

// Translated from 005c6800 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetVampireConditionFunction(thisObj, 0, 0,
/// result)`; returns its `AL`.
pub fn fn_005c6800(e: &mut Engine, a: ScriptArgs) -> bool {
    e.call(
        GET_VAMPIRE_CONDITION,
        &args![a.this_obj, 0u32, 0u32, a.result],
    )
    .bool()
}

// ---- Condition command wrappers ---------------------------------------------

/// A condition command with no parsed argument: the condition function gets
/// `(thisObj, 0, 0, result)`; returns its `AL`.
fn condition_without_arguments(e: &mut Engine, a: ScriptArgs, condition: u32) -> bool {
    e.call(condition, &args![a.this_obj, 0u32, 0u32, a.result])
        .bool()
}

/// A condition command with one parsed argument (a word local initialised
/// to 0): false when the parameters do not parse, otherwise the condition
/// function gets `(thisObj, argument, 0, result)` and its `AL` is returned.
fn condition_with_one_argument(e: &mut Engine, a: ScriptArgs, condition: u32) -> bool {
    let Some([argument]) = parse_into(e, a, [0]) else {
        return false;
    };
    e.call(condition, &args![a.this_obj, argument, 0u32, a.result])
        .bool()
}

/// A condition command run with the player as its argument: the condition
/// function gets `(thisObj, player, 0, result)`; returns its `AL`.
fn condition_with_player(e: &mut Engine, a: ScriptArgs, condition: u32) -> bool {
    let player = e.global::<u32>(PLAYER);
    e.call(condition, &args![a.this_obj, player, 0u32, a.result])
        .bool()
}

// Translated from 005c6820 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetClothingValueConditionFunction(thisObj,
/// 0, 0, result)`; returns its `AL`.
pub fn fn_005c6820(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_CLOTHING_VALUE_CONDITION)
}

// Translated from 005c6840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SameFaction` (Xbox PDB): parses one argument and returns
/// `SameFactionConditionFunction(thisObj, argument, 0, result)`.
pub fn script_same_faction(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, SAME_FACTION_CONDITION)
}

// Translated from 005c68a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SameRace` (Xbox PDB): parses one argument and returns
/// `SameRaceConditionFunction(thisObj, argument, 0, result)`.
pub fn script_same_race(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, SAME_RACE_CONDITION)
}

// Translated from 005c6900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SameSex` (Xbox PDB): parses one argument and returns
/// `SameSexConditionFunction(thisObj, argument, 0, result)`.
pub fn script_same_sex(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, SAME_SEX_CONDITION)
}

// Translated from 005c6960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player variant of `SameFaction`: `SameFactionConditionFunction(
/// thisObj, player, 0, result)` with the player singleton as the argument;
/// returns its `AL`.
pub fn fn_005c6960(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_player(e, a, SAME_FACTION_CONDITION)
}

// Translated from 005c6990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player variant of `SameRace`: `SameRaceConditionFunction(thisObj,
/// player, 0, result)`; returns its `AL`.
pub fn fn_005c6990(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_player(e, a, SAME_RACE_CONDITION)
}

// Translated from 005c69c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player variant of `SameSex`: `SameSexConditionFunction(thisObj,
/// player, 0, result)`; returns its `AL`.
pub fn fn_005c69c0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_player(e, a, SAME_SEX_CONDITION)
}

// Translated from 005c69f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDetected` (Xbox PDB): parses one argument and returns
/// `GetDetectedConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_detected(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_DETECTED_CONDITION)
}

// Translated from 005c6a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetDeadConditionFunction(thisObj, 0, 0,
/// result)`; returns its `AL`.
pub fn fn_005c6a50(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_DEAD_CONDITION)
}

// Translated from 005c6a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetItemCount` (Xbox PDB): parses one argument and returns
/// `GetItemCountConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_item_count(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_ITEM_COUNT_CONDITION)
}

// Translated from 005c6ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command taking one signed number: stores 1 through [`fn_005c6b40`] on
/// the player when the number is greater than 0, 0 otherwise; returns true
/// (false when the parameters do not parse).
pub fn fn_005c6ad0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([number]) = parse_into(e, a, [0]) else {
        return false;
    };
    let player = Ptr::new(e.global::<u32>(PLAYER));
    fn_005c6b40(e, player, (number as i32 > 0) as u8);
    true
}

// Translated from 005c6b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at `this + 0x240` (a setter used on the player; the Xbox
/// layout of `PlayerCharacter` has a different field there, so the field is
/// not named).
pub fn fn_005c6b40(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x240, value);
}

// Translated from 005c6b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command on an actor reference: when the reference is given and answers
/// true to virtual slot `0x100`, restores actor value `0x10` by the
/// difference of two values its object at `+0xa4` returns (slot `0x20` minus
/// slot `0x0c`, both asked for `0x10`), then actor values `0x1d, 0x1e, 0x1b,
/// 0x1c, 0x19, 0x1a, 0x1f` by 999.0 each (`Actor::RestoreActorValue`).
/// Always returns true.
pub fn fn_005c6b60(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() || !e.vcall(a.this_obj.addr(), 0x100, &args![]).bool() {
        return true;
    }
    let actor = a.this_obj;
    let owner = actor.addr() + 0xa4;
    let first = e.vcall(owner, 0xc, &args![0x10u32]).f32();
    let second = e.vcall(owner, 0x20, &args![0x10u32]).f32();
    // x87: the difference is computed in extended precision and stored as a
    // `float`.
    let difference = (second as f64 - first as f64) as f32;
    e.call(RESTORE_ACTOR_VALUE, &args![actor, 0x10u32, difference]);
    for actor_value in [0x1du32, 0x1e, 0x1b, 0x1c, 0x19, 0x1a, 0x1f] {
        let amount = e.global::<f32>(LIMB_RESTORE_AMOUNT);
        e.call(RESTORE_ACTOR_VALUE, &args![actor, actor_value, amount]);
    }
    true
}

// Translated from 005c6c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command taking a reference: when the command's reference answers true
/// to virtual slot `0x100` and the argument is given, both base forms are
/// cast from `TESBoundObject` to `TESNPC`; when both succeed, the argument
/// NPC's field `+0x130` ([`NPC_GET_FIELD_0X130`]) and its actor base data
/// level are copied to the command's NPC, whose `TESNPC::InitValues(0)` then
/// runs. Returns true (false when the parameters do not parse).
pub fn fn_005c6c80(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([argument]) = parse_into(e, a, [0]) else {
        return false;
    };
    if a.this_obj.is_null() || !e.vcall(a.this_obj.addr(), 0x100, &args![]).bool() || argument == 0
    {
        return true;
    }
    let this_base = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
    let this_npc = e
        .call(
            DYNAMIC_CAST,
            &args![this_base, 0u32, RTTI_TES_BOUND_OBJECT, RTTI_TES_NPC, 0u32],
        )
        .u32();
    let argument_base = e.call(GET_BASE_FORM, &args![argument]).u32();
    let argument_npc = e
        .call(
            DYNAMIC_CAST,
            &args![
                argument_base,
                0u32,
                RTTI_TES_BOUND_OBJECT,
                RTTI_TES_NPC,
                0u32
            ],
        )
        .u32();
    if this_npc != 0 && argument_npc != 0 {
        let field = e.call(NPC_GET_FIELD_0X130, &args![argument_npc]).u32();
        e.call(NPC_SET_FIELD_0X130, &args![this_npc, field]);
        let level = e
            .call(ACTOR_BASE_DATA_GET_LEVEL, &args![argument_npc + 0x30])
            .u16();
        e.call(
            ACTOR_BASE_DATA_SET_LEVEL,
            &args![this_npc + 0x30, level as u32],
        );
        e.call(TES_NPC_INIT_VALUES, &args![this_npc, 0u32]);
    }
    true
}

// Translated from 005c6d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetEquippedFunction` (Xbox PDB): parses one argument and returns
/// `GetEquippedConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_equipped_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_EQUIPPED_CONDITION)
}

// Translated from 005c6df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetGoldConditionFunction(thisObj, 0, 0,
/// result)`; returns its `AL`.
pub fn fn_005c6df0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_GOLD_CONDITION)
}

// Translated from 005c6e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetSleepingConditionFunction(thisObj, 0, 0,
/// result)`; returns its `AL`.
pub fn fn_005c6e10(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_SLEEPING_CONDITION)
}

// Translated from 005c6e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetSittingConditionFunction(thisObj, 0, 0,
/// result)`; returns its `AL`.
pub fn fn_005c6e30(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_SITTING_CONDITION)
}

// Translated from 005c6e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetFurnitureMarkerIDConditionFunction(
/// thisObj, 0, 0, result)`; returns its `AL`.
pub fn fn_005c6e50(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_FURNITURE_MARKER_ID_CONDITION)
}

// Translated from 005c6e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsCurrentFurnitureRef` (Xbox PDB): parses one argument and
/// returns `IsCurrentFurnitureRefConditionFunction(thisObj, argument, 0,
/// result)`.
pub fn script_is_current_furniture_ref(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, IS_CURRENT_FURNITURE_REF_CONDITION)
}

// Translated from 005c6ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsCurrentFurnitureObj` (Xbox PDB): parses one argument and
/// returns `IsCurrentFurnitureObjConditionFunction(thisObj, argument, 0,
/// result)`.
pub fn script_is_current_furniture_obj(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, IS_CURRENT_FURNITURE_OBJ_CONDITION)
}

// Translated from 005c6f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetTalkedToPCConditionFunction(thisObj, 0,
/// 0, result)`; returns its `AL`.
pub fn fn_005c6f30(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_TALKED_TO_PC_CONDITION)
}

// Translated from 005c6f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetTalkedToPCParam` (Xbox PDB): parses one argument and returns
/// `GetTalkedToPCConditionFunction(0, argument, 0, result)`; the reference
/// is passed as 0, not `thisObj`.
pub fn script_get_talked_to_pc_param(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([argument]) = parse_into(e, a, [0]) else {
        return false;
    };
    e.call(
        GET_TALKED_TO_PC_CONDITION,
        &args![0u32, argument, 0u32, a.result],
    )
    .bool()
}

// Translated from 005c6fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetQuestRunning` (Xbox PDB): parses one argument and returns
/// `GetQuestRunningConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_quest_running(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_QUEST_RUNNING_CONDITION)
}

// Translated from 005c7010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetQuestCompleted` (Xbox PDB): parses one argument and returns
/// `GetQuestCompletedConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_quest_completed(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_QUEST_COMPLETED_CONDITION)
}

// Translated from 005c7070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetStage` (Xbox PDB): parses one argument and returns
/// `GetStageConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_stage(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_STAGE_CONDITION)
}

// Translated from 005c70d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetStageDone` (Xbox PDB): parses two arguments (both locals
/// start at 0) and returns `GetStageDoneConditionFunction(thisObj, first,
/// second, result)`.
pub fn script_get_stage_done(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, second]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    e.call(
        GET_STAGE_DONE_CONDITION,
        &args![a.this_obj, first, second, a.result],
    )
    .bool()
}

// Translated from 005c7140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetStageFunction` (Xbox PDB): the result starts at 0.0; parses a
/// quest and a stage (only the low byte of the second local is used). When
/// the quest is given and [`QUEST_SET_STAGE`] answers true, the result is
/// 1.0. Returns true (false when the parameters do not parse).
pub fn script_set_stage_function(e: &mut Engine, a: ScriptArgs) -> bool {
    set_result(e, a, 0.0);
    let Some([quest, stage]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    if quest != 0 && e.call(QUEST_SET_STAGE, &args![quest, stage & 0xff]).bool() {
        set_result(e, a, 1.0);
    }
    true
}

// Translated from 005c71c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command taking a quest: calls [`QUEST_SET_FLAG_0X1`] on it with true
/// when it is given. Returns true (false when the parameters do not parse).
pub fn fn_005c71c0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([quest]) = parse_into(e, a, [0]) else {
        return false;
    };
    if quest != 0 {
        e.call(QUEST_SET_FLAG_0X1, &args![quest, 1u32]);
    }
    true
}

// Translated from 005c7220 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command taking a quest: calls [`QUEST_SET_FLAG_0X1`] on it with false
/// when it is given. Returns true (false when the parameters do not parse).
pub fn fn_005c7220(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([quest]) = parse_into(e, a, [0]) else {
        return false;
    };
    if quest != 0 {
        e.call(QUEST_SET_FLAG_0X1, &args![quest, 0u32]);
    }
    true
}

// Translated from 005c7280 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command taking a quest: calls [`QUEST_SET_FLAG_0X2`] on it with true
/// when it is given. Returns true (false when the parameters do not parse).
pub fn fn_005c7280(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([quest]) = parse_into(e, a, [0]) else {
        return false;
    };
    if quest != 0 {
        e.call(QUEST_SET_FLAG_0X2, &args![quest, 1u32]);
    }
    true
}

// Translated from 005c72e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetFactionRankDifference` (Xbox PDB): parses two arguments (in
/// output order) and returns `GetFactionRankDifferenceConditionFunction(
/// thisObj, first, second, result)`.
pub fn script_get_faction_rank_difference(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, second]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    e.call(
        GET_FACTION_RANK_DIFFERENCE_CONDITION,
        &args![a.this_obj, first, second, a.result],
    )
    .bool()
}

// Translated from 005c7350 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetAlarmedConditionFunction(thisObj, 0, 0,
/// result)`; returns its `AL`.
pub fn fn_005c7350(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_ALARMED_CONDITION)
}

// Translated from 005c7370 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetIsPleasantConditionFunction(thisObj, 0,
/// 0, result)`; returns its `AL`.
pub fn fn_005c7370(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_IS_PLEASANT_CONDITION)
}

// Translated from 005c7390 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetIsCloudyConditionFunction(thisObj, 0, 0,
/// result)`; returns its `AL`.
pub fn fn_005c7390(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_IS_CLOUDY_CONDITION)
}

// Translated from 005c73b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetIsRainingConditionFunction(thisObj, 0,
/// 0, result)`; returns its `AL`.
pub fn fn_005c73b0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_IS_RAINING_CONDITION)
}

// Translated from 005c73d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetIsSnowingConditionFunction(thisObj, 0,
/// 0, result)`; returns its `AL`.
pub fn fn_005c73d0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_IS_SNOWING_CONDITION)
}

// Translated from 005c73f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetWindSpeedConditionFunction(thisObj, 0,
/// 0, result)`; returns its `AL`.
pub fn fn_005c73f0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_WIND_SPEED_CONDITION)
}

// Translated from 005c7410 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetWeatherPercentConditionFunction(
/// thisObj, 0, 0, result)`; returns its `AL`.
pub fn fn_005c7410(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_WEATHER_PERCENT_CONDITION)
}

// Translated from 005c7430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsCurrentWeatherFunction` (Xbox PDB): parses one argument and
/// returns `GetIsCurrentWeatherConditionFunction(thisObj, argument, 0,
/// result)`.
pub fn script_get_is_current_weather_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_IS_CURRENT_WEATHER_CONDITION)
}

// ---- 005c7490 .. 005c83f0 ---------------------------------------------------

/// A condition command with two parsed arguments (word locals initialised
/// to 0, in output order): false when the parameters do not parse,
/// otherwise the condition function gets `(thisObj, first, second, result)`
/// and its `AL` is returned.
fn condition_with_two_arguments(e: &mut Engine, a: ScriptArgs, condition: u32) -> bool {
    let Some([first, second]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    e.call(condition, &args![a.this_obj, first, second, a.result])
        .bool()
}

/// A condition command with one parsed argument that runs on the player
/// instead of the command's reference: the condition function gets
/// `(player, argument, 0, result)`.
fn condition_on_player_with_one_argument(e: &mut Engine, a: ScriptArgs, condition: u32) -> bool {
    let Some([argument]) = parse_into(e, a, [0]) else {
        return false;
    };
    let player = e.global::<u32>(PLAYER);
    e.call(condition, &args![player, argument, 0u32, a.result])
        .bool()
}

/// The `RTTI` cast of the command's reference to `Actor` (`__RTDynamicCast`
/// from `TESObjectREFR`): the actor or 0.
fn cast_to_actor(e: &mut Engine, reference: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![reference, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
    )
    .u32()
}

// Translated from 005c7490 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetAttackedConditionFunction(thisObj, 0, 0,
/// result)`; returns its `AL`.
pub fn fn_005c7490(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_ATTACKED_CONDITION)
}

// Translated from 005c74b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetIsCreatureConditionFunction(thisObj, 0,
/// 0, result)`; returns its `AL`.
pub fn fn_005c74b0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_IS_CREATURE_CONDITION)
}

// Translated from 005c74d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetShouldAttack` (Xbox PDB): parses one argument and returns
/// `GetShouldAttackConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_should_attack(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_SHOULD_ATTACK_CONDITION)
}

// Translated from 005c7530 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command parsing one argument: `Script::GetInCellConditionFunction(
/// thisObj, argument, 0, result)`; returns its `AL`.
pub fn fn_005c7530(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_IN_CELL_CONDITION)
}

// Translated from 005c7590 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command parsing two arguments: `Script::GetInCellConditionFunction(
/// thisObj, first, second, result)`; returns its `AL` (false when the
/// parameters do not parse).
pub fn fn_005c7590(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_two_arguments(e, a, GET_IN_CELL_CONDITION)
}

// Translated from 005c7600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetInWorldspace` (Xbox PDB): parses one argument and returns
/// `GetInWorldspaceConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_in_worldspace(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_IN_WORLDSPACE_CONDITION)
}

// Translated from 005c7660 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command taking a cell and a signed number: when the cell is given,
/// sets (number greater than 0) or clears bit `0x20` of its flags byte
/// through [`CELL_SET_FLAG_0X20`]. Returns true (false when the parameters
/// do not parse).
pub fn fn_005c7660(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([cell, number]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    if cell != 0 {
        e.call(CELL_SET_FLAG_0X20, &args![cell, (number as i32 > 0) as u32]);
    }
    true
}

// Translated from 005c76d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetInSameCell` (Xbox PDB): parses one argument and returns
/// `GetInSameCellConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_in_same_cell(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_IN_SAME_CELL_CONDITION)
}

// Translated from 005c7730 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command parsing one argument: `Script::GetIsClassConditionFunction(
/// thisObj, argument, 0, result)`; returns its `AL`.
pub fn fn_005c7730(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_IS_CLASS_CONDITION)
}

// Translated from 005c7790 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command taking one number: stores 1 (number non-zero) or 0 as the
/// player's chargen byte through [`PLAYER_SET_CHARGEN`]. Returns true
/// (false when the parameters do not parse).
pub fn fn_005c7790(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag]) = parse_into(e, a, [0]) else {
        return false;
    };
    let player = e.global::<u32>(PLAYER);
    e.call(PLAYER_SET_CHARGEN, &args![player, (flag != 0) as u32]);
    true
}

// Translated from 005c7800 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function-style command: the result is 0.0, or, when the player exists,
/// his chargen byte ([`fn_005c7870`]). With the console echo on, the result
/// is printed truncated to an `int`. Always returns true.
pub fn fn_005c7800(e: &mut Engine, a: ScriptArgs) -> bool {
    set_result(e, a, 0.0);
    let player = e.global::<u32>(PLAYER);
    if player != 0 {
        let chargen = fn_005c7870(e, Ptr::new(player));
        set_result(e, a, chargen as f64);
        if echo_enabled(e) {
            let value = e.mem.f64(a.result.addr());
            let truncated = e.call(FTOL, &args![value]).u32();
            console_print(e, &args![MSG_GET_CHARGEN_VALUE, truncated]);
        }
    }
    true
}

// Translated from 005c7870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player's chargen byte (`bChargen` of the Xbox `PlayerCharacter`):
/// `*(this + 0x75c)` (`fastcall`).
pub fn fn_005c7870(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x75c)
}

// Translated from 005c7890 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command parsing one argument: `Script::GetIsRaceConditionFunction(
/// thisObj, argument, 0, result)`; returns its `AL`.
pub fn fn_005c7890(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_IS_RACE_CONDITION)
}

// Translated from 005c78f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsCreatureType` (Xbox PDB): parses one argument and returns
/// `GetIsCreatureTypeConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_is_creature_type(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_IS_CREATURE_TYPE_CONDITION)
}

// Translated from 005c7950 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command parsing one argument: `Script::GetIsSexConditionFunction(
/// thisObj, argument, 0, result)`; returns its `AL`. (The game does not
/// initialise the local; it starts at 0 here.)
pub fn fn_005c7950(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_IS_SEX_CONDITION)
}

// Translated from 005c79b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetIsPlayableRaceConditionFunction(
/// thisObj, 0, 0, result)`; returns its `AL`.
pub fn fn_005c79b0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_IS_PLAYABLE_RACE_CONDITION)
}

// Translated from 005c79d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command parsing one argument: `Script::GetInFactionConditionFunction(
/// thisObj, argument, 0, result)`; returns its `AL`.
pub fn fn_005c79d0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_IN_FACTION_CONDITION)
}

// Translated from 005c7a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player variant of `GetIsClass`: parses one argument and returns
/// `GetIsClassConditionFunction(player, argument, 0, result)`.
pub fn fn_005c7a30(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_on_player_with_one_argument(e, a, GET_IS_CLASS_CONDITION)
}

// Translated from 005c7a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player variant of `GetIsRace`: parses one argument and returns
/// `GetIsRaceConditionFunction(player, argument, 0, result)`.
pub fn fn_005c7a90(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_on_player_with_one_argument(e, a, GET_IS_RACE_CONDITION)
}

// Translated from 005c7af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player variant of `GetIsSex`: parses one argument and returns
/// `GetIsSexConditionFunction(player, argument, 0, result)`. (The game does
/// not initialise the local; it starts at 0 here.)
pub fn fn_005c7af0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_on_player_with_one_argument(e, a, GET_IS_SEX_CONDITION)
}

// Translated from 005c7b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player variant of `GetInFaction`: parses one argument and returns
/// `GetInFactionConditionFunction(player, argument, 0, result)`.
pub fn fn_005c7b50(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_on_player_with_one_argument(e, a, GET_IN_FACTION_CONDITION)
}

// Translated from 005c7bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsID` (Xbox PDB): parses one argument and returns
/// `GetIsIDConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_is_id(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_IS_ID_CONDITION)
}

// Translated from 005c7c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsInListFunction` (Xbox PDB): parses one argument and returns
/// `IsInListConditionFunction(thisObj, argument, 0, result)`.
pub fn script_is_in_list_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, IS_IN_LIST_CONDITION)
}

// Translated from 005c7c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetIsChildConditionFunction(thisObj, 0, 0,
/// result)`; returns its `AL`.
pub fn fn_005c7c70(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_IS_CHILD_CONDITION)
}

// Translated from 005c7ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsUsedItem` (Xbox PDB): parses one argument and returns
/// `GetIsUsedItemConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_is_used_item(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_IS_USED_ITEM_CONDITION)
}

// Translated from 005c7d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsUsedItemType` (Xbox PDB): parses one argument that the
/// parser stores as a single byte (the game's local is an uninitialised
/// byte; it starts at 0 here) and returns
/// `GetIsUsedItemTypeConditionFunction(thisObj, byte, 0, result)`.
pub fn script_get_is_used_item_type(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([kind]) = parse_into(e, a, [0]) else {
        return false;
    };
    e.call(
        GET_IS_USED_ITEM_TYPE_CONDITION,
        &args![a.this_obj, kind & 0xff, 0u32, a.result],
    )
    .bool()
}

// Translated from 005c7d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetUsedItemLevelConditionFunction(thisObj,
/// 0, 0, result)`; returns its `AL`.
pub fn fn_005c7d60(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_USED_ITEM_LEVEL_CONDITION)
}

// Translated from 005c7d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetUsedItemActivateConditionFunction(
/// thisObj, 0, 0, result)`; returns its `AL`.
pub fn fn_005c7d80(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_USED_ITEM_ACTIVATE_CONDITION)
}

// Translated from 005c7da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsRef` (Xbox PDB): parses one argument and returns
/// `GetIsRefConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_is_ref(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_IS_REF_CONDITION)
}

// Translated from 005c7e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetFactionRank` (Xbox PDB): parses one argument and returns
/// `GetFactionRankConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_faction_rank(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_FACTION_RANK_CONDITION)
}

// Translated from 005c7e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command taking a faction and a rank (the second parsed as a byte):
/// the result is 0.0. When the command's reference casts to `Actor`, its
/// base form accessor runs ([`REFERENCE_BASE_FORM`], result unused) and,
/// when the faction is given, [`ACTOR_SET_FACTION_RANK`] sets the rank.
/// Returns true (false when the parameters do not parse; the result is then
/// not touched).
pub fn fn_005c7e60(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([faction, rank]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    set_result(e, a, 0.0);
    let actor = cast_to_actor(e, a.this_obj);
    if actor == 0 {
        return true;
    }
    e.call(REFERENCE_BASE_FORM, &args![actor]);
    if faction != 0 {
        e.call(ACTOR_SET_FACTION_RANK, &args![actor, faction, rank & 0xff]);
    }
    true
}

// Translated from 005c7f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command taking a faction and a rank change (the second parsed as a
/// byte): the result is 0.0. When the command's reference casts to `Actor`,
/// its base form accessor runs ([`REFERENCE_BASE_FORM`], result unused) and,
/// when the faction is given, [`ACTOR_ADD_TO_FACTION_RANK`] runs with the
/// change and whether the actor is the player. Returns true (false when the
/// parameters do not parse; the result is then not touched).
pub fn fn_005c7f10(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([faction, change]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    set_result(e, a, 0.0);
    let actor = cast_to_actor(e, a.this_obj);
    if actor == 0 {
        return true;
    }
    e.call(REFERENCE_BASE_FORM, &args![actor]);
    if faction != 0 {
        let is_player = actor == e.global::<u32>(PLAYER);
        e.call(
            ACTOR_ADD_TO_FACTION_RANK,
            &args![actor, faction, change & 0xff, is_player as u32],
        );
    }
    true
}

// Translated from 005c7fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetFactionReaction` (Xbox PDB): the result is 0.0, or, when
/// both factions are given, the first faction's reaction to the second
/// ([`FACTION_GET_REACTION`]). With the console echo on, prints "<first
/// name> reaction to <second name> is <value>". Returns true (false when
/// the parameters do not parse; the result is then 0.0).
pub fn script_get_faction_reaction(e: &mut Engine, a: ScriptArgs) -> bool {
    set_result(e, a, 0.0);
    let Some([first, second]) = parse_into(e, a, [0, 0]) else {
        return false;
    };
    if first != 0 && second != 0 {
        let reaction = e.call(FACTION_GET_REACTION, &args![first, second]).u32() as i32;
        set_result(e, a, reaction as f64);
    }
    if echo_enabled(e) {
        // The game asks for the second name first (it is pushed first).
        let second_name = e.call(STRING_OR_EMPTY, &args![second + 0x18]).u32();
        let first_name = e.call(STRING_OR_EMPTY, &args![first + 0x18]).u32();
        let value = e.mem.f64(a.result.addr());
        console_print(
            e,
            &args![MSG_FACTION_REACTION, first_name, second_name, value],
        );
    }
    true
}

// Translated from 005c80a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command taking a faction, another faction and a value: the result is
/// 0.0, and when both factions are given [`FACTION_SET_REACTION`] runs on
/// the first with the second and the value. Returns true (false when the
/// parameters do not parse; the result is then not touched).
pub fn fn_005c80a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, second, value]) = parse_into(e, a, [0, 0, 0]) else {
        return false;
    };
    set_result(e, a, 0.0);
    if second != 0 && first != 0 {
        e.call(FACTION_SET_REACTION, &args![first, second, value]);
    }
    true
}

// Translated from 005c8120 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command taking two factions and two flags: the result is 0.0, and when
/// both factions are given each is told about the other through
/// [`FACTION_SET_RELATION_CODE`], with code 3 when its flag is non-zero and
/// 2 when it is 0 (the first flag for the first faction). Returns true
/// (false when the parameters do not parse; the result is then not
/// touched).
pub fn fn_005c8120(e: &mut Engine, a: ScriptArgs) -> bool {
    set_relation_codes(e, a, 2, 3)
}

// Translated from 005c81f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_005c8120`], with code 0 when the flag is non-zero and 1 when
/// it is 0.
pub fn fn_005c81f0(e: &mut Engine, a: ScriptArgs) -> bool {
    set_relation_codes(e, a, 1, 0)
}

/// The body of `005c8120` and `005c81f0`: `if_clear` and `if_set` are the
/// relation codes for a flag that is 0 and one that is not.
fn set_relation_codes(e: &mut Engine, a: ScriptArgs, if_clear: u32, if_set: u32) -> bool {
    let Some([first, second, first_flag, second_flag]) = parse_into(e, a, [0, 0, 0, 0]) else {
        return false;
    };
    set_result(e, a, 0.0);
    if second != 0 && first != 0 {
        let code = if first_flag == 0 { if_clear } else { if_set };
        e.call(FACTION_SET_RELATION_CODE, &args![first, second, code]);
        let code = if second_flag == 0 { if_clear } else { if_set };
        e.call(FACTION_SET_RELATION_CODE, &args![second, first, code]);
    }
    true
}

// Translated from 005c82c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command taking a faction, another faction and a value: the result is
/// 0.0, and when both factions are given [`FACTION_MODIFY_REACTION`] runs
/// on the first with the second and the value. Returns true (false when
/// the parameters do not parse; the result is then not touched).
pub fn fn_005c82c0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, second, value]) = parse_into(e, a, [0, 0, 0]) else {
        return false;
    };
    set_result(e, a, 0.0);
    if second != 0 && first != 0 {
        e.call(FACTION_MODIFY_REACTION, &args![first, second, value]);
    }
    true
}

// Translated from 005c8340 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command on an actor-like reference: when the reference is given and
/// answers true to virtual slot `0x100`, marks it changed through slot
/// `0x48` (`0x8000_0000`), makes sure its extra data `0x5e` exists (adding
/// it when `GetExtraData(0x5e)` of its embedded `ExtraDataList` finds none)
/// and updates that object from the reference's base form
/// ([`EXTRA_DATA_0X5E_UPDATE`]). Always returns true.
pub fn fn_005c8340(e: &mut Engine, a: ScriptArgs) -> bool {
    let reference = a.this_obj;
    if reference.is_null() || !e.vcall(reference.addr(), 0x100, &args![]).bool() {
        return true;
    }
    e.vcall(reference.addr(), 0x48, &args![0x8000_0000u32]);
    let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
    let mut extra = e.call(GET_EXTRA_DATA_0X5E, &args![list]).u32();
    if extra == 0 {
        let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
        e.call(ADD_EXTRA_DATA_0X5E, &args![list]);
        let list = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
        extra = e.call(GET_EXTRA_DATA_0X5E, &args![list]).u32();
    }
    let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    e.call(EXTRA_DATA_0X5E_UPDATE, &args![extra, base_form]);
    true
}

// Translated from 005c83d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition command: `Script::GetXPForNextLevelConditionFunction(thisObj,
/// 0, 0, result)`; returns its `AL`.
pub fn fn_005c83d0(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_without_arguments(e, a, GET_XP_FOR_NEXT_LEVEL_CONDITION)
}

// Translated from 005c83f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetGlobalValue` (Xbox PDB): parses one argument and returns
/// `GetGlobalValueConditionFunction(thisObj, argument, 0, result)`.
pub fn script_get_global_value(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_one_argument(e, a, GET_GLOBAL_VALUE_CONDITION)
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005c4240, script_menu_mode_function(ScriptArgs) -> bool),
        entry!(0x005c42a0, fn_005c42a0(ScriptArgs) -> bool),
        entry!(0x005c42d0, fn_005c42d0() -> u8),
        entry!(0x005c42e0, fn_005c42e0(ScriptArgs) -> bool),
        entry!(0x005c4320, fn_005c4320(ScriptArgs) -> bool),
        entry!(0x005c4360, fn_005c4360(ScriptArgs) -> bool),
        entry!(0x005c43a0, fn_005c43a0(ScriptArgs) -> bool),
        entry!(0x005c43d0, script_enable_function(ScriptArgs) -> bool),
        entry!(0x005c45e0, script_disable_function(ScriptArgs) -> bool),
        entry!(0x005c47e0, fn_005c47e0(ScriptArgs) -> bool),
        entry!(0x005c48c0, fn_005c48c0(ScriptArgs) -> bool),
        entry!(0x005c48e0, fn_005c48e0(ScriptArgs) -> bool),
        entry!(0x005c4900, fn_005c4900(ScriptArgs) -> bool),
        entry!(0x005c4920, fn_005c4920(ScriptArgs) -> bool),
        entry!(
            0x005c4940,
            script_get_is_form_type_function(ScriptArgs) -> bool
        ),
        entry!(
            0x005c4a10,
            script_get_is_voice_type_function(ScriptArgs) -> bool
        ),
        entry!(0x005c4a70, fn_005c4a70(ScriptArgs) -> bool),
        entry!(
            0x005c4b30,
            fn_005c4b30(Ptr, Ptr, i32, i32, i32, f32) -> Ptr
        ),
        entry!(0x005c53d0, fn_005c53d0(f32) -> f32),
        entry!(0x005c53f0, fn_005c53f0(Ptr, u32) -> u32),
        entry!(0x005c5420, fn_005c5420() -> f32),
        entry!(0x005c5440, script_save_game(ScriptArgs) -> bool),
        entry!(0x005c54d0, fn_005c54d0(ScriptArgs) -> bool),
        entry!(0x005c5560, fn_005c5560() -> bool),
        entry!(0x005c5590, fn_005c5590() -> bool),
        entry!(0x005c55d0, script_copy_save_games(ScriptArgs) -> bool),
        entry!(0x005c5680, script_toggle_cell_node(ScriptArgs) -> bool),
        entry!(0x005c57f0, script_toggle_trees() -> bool),
        entry!(0x005c5840, script_set_camera_fov(ScriptArgs) -> bool),
        entry!(0x005c59c0, fn_005c59c0(Ptr, f32)),
        entry!(0x005c59e0, fn_005c59e0(Ptr, f32)),
        entry!(0x005c5a00, fn_005c5a00(u32)),
        entry!(0x005c5a20, fn_005c5a20() -> bool),
        entry!(0x005c5a50, fn_005c5a50() -> u32),
        entry!(0x005c5a60, fn_005c5a60(u32)),
        entry!(
            0x005c5a70,
            script_dump_texture_palette(ScriptArgs) -> bool
        ),
        entry!(0x005c6430, script_dump_model_map(ScriptArgs) -> bool),
        entry!(0x005c65a0, fn_005c65a0(ScriptArgs) -> bool),
        entry!(0x005c67e0, fn_005c67e0(ScriptArgs) -> bool),
        entry!(0x005c6800, fn_005c6800(ScriptArgs) -> bool),
        entry!(0x005c6820, fn_005c6820(ScriptArgs) -> bool),
        entry!(0x005c6840, script_same_faction(ScriptArgs) -> bool),
        entry!(0x005c68a0, script_same_race(ScriptArgs) -> bool),
        entry!(0x005c6900, script_same_sex(ScriptArgs) -> bool),
        entry!(0x005c6960, fn_005c6960(ScriptArgs) -> bool),
        entry!(0x005c6990, fn_005c6990(ScriptArgs) -> bool),
        entry!(0x005c69c0, fn_005c69c0(ScriptArgs) -> bool),
        entry!(0x005c69f0, script_get_detected(ScriptArgs) -> bool),
        entry!(0x005c6a50, fn_005c6a50(ScriptArgs) -> bool),
        entry!(0x005c6a70, script_get_item_count(ScriptArgs) -> bool),
        entry!(0x005c6ad0, fn_005c6ad0(ScriptArgs) -> bool),
        entry!(0x005c6b40, fn_005c6b40(Ptr, u8)),
        entry!(0x005c6b60, fn_005c6b60(ScriptArgs) -> bool),
        entry!(0x005c6c80, fn_005c6c80(ScriptArgs) -> bool),
        entry!(
            0x005c6d90,
            script_get_equipped_function(ScriptArgs) -> bool
        ),
        entry!(0x005c6df0, fn_005c6df0(ScriptArgs) -> bool),
        entry!(0x005c6e10, fn_005c6e10(ScriptArgs) -> bool),
        entry!(0x005c6e30, fn_005c6e30(ScriptArgs) -> bool),
        entry!(0x005c6e50, fn_005c6e50(ScriptArgs) -> bool),
        entry!(
            0x005c6e70,
            script_is_current_furniture_ref(ScriptArgs) -> bool
        ),
        entry!(
            0x005c6ed0,
            script_is_current_furniture_obj(ScriptArgs) -> bool
        ),
        entry!(0x005c6f30, fn_005c6f30(ScriptArgs) -> bool),
        entry!(
            0x005c6f50,
            script_get_talked_to_pc_param(ScriptArgs) -> bool
        ),
        entry!(0x005c6fb0, script_get_quest_running(ScriptArgs) -> bool),
        entry!(
            0x005c7010,
            script_get_quest_completed(ScriptArgs) -> bool
        ),
        entry!(0x005c7070, script_get_stage(ScriptArgs) -> bool),
        entry!(0x005c70d0, script_get_stage_done(ScriptArgs) -> bool),
        entry!(
            0x005c7140,
            script_set_stage_function(ScriptArgs) -> bool
        ),
        entry!(0x005c71c0, fn_005c71c0(ScriptArgs) -> bool),
        entry!(0x005c7220, fn_005c7220(ScriptArgs) -> bool),
        entry!(0x005c7280, fn_005c7280(ScriptArgs) -> bool),
        entry!(
            0x005c72e0,
            script_get_faction_rank_difference(ScriptArgs) -> bool
        ),
        entry!(0x005c7350, fn_005c7350(ScriptArgs) -> bool),
        entry!(0x005c7370, fn_005c7370(ScriptArgs) -> bool),
        entry!(0x005c7390, fn_005c7390(ScriptArgs) -> bool),
        entry!(0x005c73b0, fn_005c73b0(ScriptArgs) -> bool),
        entry!(0x005c73d0, fn_005c73d0(ScriptArgs) -> bool),
        entry!(0x005c73f0, fn_005c73f0(ScriptArgs) -> bool),
        entry!(0x005c7410, fn_005c7410(ScriptArgs) -> bool),
        entry!(
            0x005c7430,
            script_get_is_current_weather_function(ScriptArgs) -> bool
        ),
        entry!(0x005c7490, fn_005c7490(ScriptArgs) -> bool),
        entry!(0x005c74b0, fn_005c74b0(ScriptArgs) -> bool),
        entry!(0x005c74d0, script_get_should_attack(ScriptArgs) -> bool),
        entry!(0x005c7530, fn_005c7530(ScriptArgs) -> bool),
        entry!(0x005c7590, fn_005c7590(ScriptArgs) -> bool),
        entry!(0x005c7600, script_get_in_worldspace(ScriptArgs) -> bool),
        entry!(0x005c7660, fn_005c7660(ScriptArgs) -> bool),
        entry!(0x005c76d0, script_get_in_same_cell(ScriptArgs) -> bool),
        entry!(0x005c7730, fn_005c7730(ScriptArgs) -> bool),
        entry!(0x005c7790, fn_005c7790(ScriptArgs) -> bool),
        entry!(0x005c7800, fn_005c7800(ScriptArgs) -> bool),
        entry!(0x005c7870, fn_005c7870(Ptr) -> u8),
        entry!(0x005c7890, fn_005c7890(ScriptArgs) -> bool),
        entry!(0x005c78f0, script_get_is_creature_type(ScriptArgs) -> bool),
        entry!(0x005c7950, fn_005c7950(ScriptArgs) -> bool),
        entry!(0x005c79b0, fn_005c79b0(ScriptArgs) -> bool),
        entry!(0x005c79d0, fn_005c79d0(ScriptArgs) -> bool),
        entry!(0x005c7a30, fn_005c7a30(ScriptArgs) -> bool),
        entry!(0x005c7a90, fn_005c7a90(ScriptArgs) -> bool),
        entry!(0x005c7af0, fn_005c7af0(ScriptArgs) -> bool),
        entry!(0x005c7b50, fn_005c7b50(ScriptArgs) -> bool),
        entry!(0x005c7bb0, script_get_is_id(ScriptArgs) -> bool),
        entry!(0x005c7c10, script_is_in_list_function(ScriptArgs) -> bool),
        entry!(0x005c7c70, fn_005c7c70(ScriptArgs) -> bool),
        entry!(0x005c7ca0, script_get_is_used_item(ScriptArgs) -> bool),
        entry!(0x005c7d00, script_get_is_used_item_type(ScriptArgs) -> bool),
        entry!(0x005c7d60, fn_005c7d60(ScriptArgs) -> bool),
        entry!(0x005c7d80, fn_005c7d80(ScriptArgs) -> bool),
        entry!(0x005c7da0, script_get_is_ref(ScriptArgs) -> bool),
        entry!(0x005c7e00, script_get_faction_rank(ScriptArgs) -> bool),
        entry!(0x005c7e60, fn_005c7e60(ScriptArgs) -> bool),
        entry!(0x005c7f10, fn_005c7f10(ScriptArgs) -> bool),
        entry!(0x005c7fd0, script_get_faction_reaction(ScriptArgs) -> bool),
        entry!(0x005c80a0, fn_005c80a0(ScriptArgs) -> bool),
        entry!(0x005c8120, fn_005c8120(ScriptArgs) -> bool),
        entry!(0x005c81f0, fn_005c81f0(ScriptArgs) -> bool),
        entry!(0x005c82c0, fn_005c82c0(ScriptArgs) -> bool),
        entry!(0x005c8340, fn_005c8340(ScriptArgs) -> bool),
        entry!(0x005c83d0, fn_005c83d0(ScriptArgs) -> bool),
        entry!(0x005c83f0, script_get_global_value(ScriptArgs) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fake virtual functions the tests put into vtables.
    const V_TRUE: u32 = 0x0900_0001;
    const V_FALSE: u32 = 0x0900_0002;
    const V_RECORD: u32 = 0x0900_0004;
    /// Returns `this + 0x30`: the position of a test reference.
    const V_POSITION: u32 = 0x0900_0005;
    /// Returns the dword at `this + 0x100`: the 3D node of a test reference.
    const V_NODE: u32 = 0x0900_0006;
    /// `V_KIND + k` returns `k`.
    const V_KIND: u32 = 0x0900_0010;

    /// An engine with the pages of the globals these commands touch mapped
    /// and the small accessors the commands use replaced by doubles that
    /// behave like the exe's code (see the constants' documentation).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_e000,
            0x0102_4000,
            0x0102_f000,
            0x0103_9000,
            0x0103_b000,
            0x0118_7000,
            0x0119_9000,
            0x011c_3000,
            0x011d_e000,
            0x011f_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(V_TRUE, |_, _| true.into_ret());
        e.register(V_FALSE, |_, _| false.into_ret());
        e.register(V_RECORD, |_, _| Ret::default());
        e.register(V_POSITION, |_, a| (a[0] + 0x30).into_ret());
        e.register(V_NODE, |e, a| e.mem.u32(a[0] + 0x100).into_ret());
        for k in 0..16u32 {
            e.register_double(V_KIND + k, move |_, _| k.into_ret());
        }
        e.register(LOG_STUB, |_, _| 0u32.into_ret());
        e.register(CONSOLE_PRINT, |_, _| Ret::default());
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GET_FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(FORM_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
        e.register(HAS_FORM_FLAG_0X800, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x800 != 0).into_ret()
        });
        e.register(HAS_FORM_FLAG_0X4000, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x4000 != 0).into_ret()
        });
        e.register(PLUS_4, |_, a| (a[0] + 4).into_ret());
        e.register(PLUS_0X18, |_, a| (a[0] + 0x18).into_ret());
        e
    }

    fn ptr(addr: u32) -> Ptr {
        Ptr::new(addr)
    }

    /// An object whose vtable (in the heap) has the given `(byte offset,
    /// function)` slots.
    fn object_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x600);
        for (offset, function) in slots {
            e.mem.set_u32(vtable + offset, *function);
        }
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The standard eight words with `this_obj` set and a result cell.
    fn script(e: &mut Engine, this_obj: u32) -> ScriptArgs {
        let result = e.mem.alloc(8);
        ScriptArgs {
            param_info: 1,
            script_data: 2,
            this_obj: ptr(this_obj),
            containing_obj: Ptr::NULL,
            script_obj: ptr(5),
            event_list: 6,
            result: ptr(result),
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

    /// Registers doubles that just accept the call.
    fn accept(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn result_of(e: &Engine, a: ScriptArgs) -> f64 {
        e.mem.f64(a.result.addr())
    }

    /// An event list with a block at `+0x10` holding `bytes`, and its flag
    /// byte at `+4` set to `flags`.
    fn event_list(e: &mut Engine, bytes: &[u8], flags: u8) -> u32 {
        let list = e.mem.alloc(0x20);
        let block = e.mem.alloc(0x10);
        for (i, byte) in bytes.iter().enumerate() {
            e.mem.set_u8(block + i as u32, *byte);
        }
        e.mem.set_u32(list + 0x10, block);
        e.mem.set_u8(list + 4, flags);
        list
    }

    // ---- 005c4240 .. 005c4360 -------------------------------------------------

    #[test]
    fn menu_mode_passes_the_parsed_argument_to_the_condition_function() {
        let mut e = engine();
        parse_gives(&mut e, true, &[7]);
        e.register(GET_MENU_MODE_CONDITION, |_, _| true.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(e.call(0x005c_4240, &args![a]).bool());
        // ParseParameters gets: info, data, opcode offset, thisObj, containing,
        // script, event list, then the address of the local.
        assert_eq!(
            calls(&e, PARSE_PARAMETERS)[0][..7],
            [1, 2, 8, 0x40, 0, 5, 6]
        );
        // The first argument is 0, not the reference.
        assert_eq!(
            calls(&e, GET_MENU_MODE_CONDITION),
            vec![vec![0, 7, 0, a.result.addr()]]
        );
        // Parameters that do not parse: false, nothing called.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_4240, &args![a]).bool());
        assert!(calls(&e, GET_MENU_MODE_CONDITION).is_empty());
    }

    #[test]
    fn menu_flag_command_is_one_unless_in_menu_mode_or_the_byte_is_set() {
        let mut e = engine();
        let a = script(&mut e, 0);
        for (in_menu, byte, expected) in [(false, 0u8, 1.0), (true, 0, 0.0), (false, 1, 0.0)] {
            e.register_double(IS_IN_MENU_MODE, move |_, _| in_menu.into_ret());
            e.set_global(FLAG_011DEA2D, byte);
            assert!(e.call(0x005c_42a0, &args![a]).bool());
            assert_eq!(result_of(&e, a), expected);
        }
    }

    #[test]
    fn flag_getter_returns_the_global_byte() {
        let mut e = engine();
        e.set_global(FLAG_011DEA2D, 0x5au8);
        assert_eq!(e.call(0x005c_42d0, &args![]).u8(), 0x5a);
    }

    #[test]
    fn event_list_byte_commands_read_the_block_behind_the_list() {
        let mut e = engine();
        for (bytes, first, second) in [
            (&[1u8, 0][..], 1.0, 0.0),
            (&[0, 7][..], 0.0, 1.0),
            (&[0, 0][..], 0.0, 0.0),
        ] {
            let mut a = script(&mut e, 0);
            a.event_list = event_list(&mut e, bytes, 0);
            assert!(e.call(0x005c_42e0, &args![a]).bool());
            assert_eq!(result_of(&e, a), first);
            assert!(e.call(0x005c_4320, &args![a]).bool());
            assert_eq!(result_of(&e, a), second);
        }
        // No event list, or no block: 0.0.
        let mut a = script(&mut e, 0);
        e.mem.set_f64(a.result.addr(), 9.0);
        a.event_list = 0;
        assert!(e.call(0x005c_42e0, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
        let list = e.mem.alloc(0x20);
        a.event_list = list;
        e.mem.set_f64(a.result.addr(), 9.0);
        assert!(e.call(0x005c_4320, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
    }

    #[test]
    fn script_flag_command_reads_the_byte_behind_the_script_block() {
        let mut e = engine();
        let mut a = script(&mut e, 0);
        let script_obj = e.mem.alloc(0x40);
        a.script_obj = ptr(script_obj);
        assert!(e.call(0x005c_4360, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
        e.mem.set_u8(script_obj + 0x18 + 0x11, 1);
        assert!(e.call(0x005c_4360, &args![a]).bool());
        assert_eq!(result_of(&e, a), 1.0);
        // Without a script nothing is read.
        a.script_obj = Ptr::NULL;
        assert!(e.call(0x005c_4360, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
    }

    #[test]
    fn float_command_returns_the_float_in_the_block() {
        let mut e = engine();
        let mut a = script(&mut e, 0);
        a.event_list = event_list(&mut e, &[0; 8], 0);
        let block = e.mem.u32(a.event_list + 0x10);
        e.mem.set_f32(block + 4, 2.5);
        assert!(e.call(0x005c_43a0, &args![a]).bool());
        assert_eq!(result_of(&e, a), 2.5);
        a.event_list = 0;
        assert!(e.call(0x005c_43a0, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
    }

    // ---- Enable and Disable -----------------------------------------------------

    /// Doubles every Enable/Disable test needs.
    fn enable_engine() -> Engine {
        let mut e = engine();
        accept(
            &mut e,
            &[
                SET_FORM_FLAG_0X8000000,
                REMOVE_DELAYED_SCRIPT_ACTION_REFERENCE,
                ADD_PENDING_ENABLED_REFERENCE,
                ADD_PENDING_DISABLED_REFERENCE,
                SET_PROCESS_MODE,
                SET_EXTRA_FLAG_0X8,
            ],
        );
        e.register(GET_ENABLE_STATE_PARENT, |_, _| 0u32.into_ret());
        e.register(GET_NAME_GUARDED, |_, _| 0u32.into_ret());
        e.register(GET_NAME_TEXT, |_, _| 0x7777u32.into_ret());
        e.register(GET_PROCESS_MODE, |e, a| e.mem.u32(a[0] + 0xc4).into_ret());
        e.register(GET_MODE_FOR_FORM, |_, a| (a[0] + 1).into_ret());
        e.register(HAS_EXTRA_FLAG_0X8, |_, _| false.into_ret());
        e
    }

    /// A reference whose virtual slot `0x1d0` gives a process whose slot
    /// `0x10` gives `process_object`, and whose slot `0x130` gives 0x4444.
    fn reference_with_process(e: &mut Engine, process_object: u32) -> u32 {
        let process = object_with(e, &[(0x10, V_KIND + 5)]);
        e.register_double(V_KIND + 5, move |_, _| process_object.into_ret());
        let reference = object_with(e, &[(0x1d0, V_NODE), (0x130, V_KIND + 6)]);
        e.mem.set_u32(reference + 0x100, process);
        e.mem.set_u32(reference + 0x20, 0xba5e);
        e.mem.set_u32(reference + 0xc, 0x1234);
        reference
    }

    #[test]
    fn enable_reports_parameters_that_do_not_parse() {
        let mut e = enable_engine();
        parse_gives(&mut e, false, &[]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(!e.call(0x005c_43d0, &args![a]).bool());
        // Only the parse was attempted.
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert!(calls(&e, SET_FORM_FLAG_0X8000000).is_empty());
    }

    #[test]
    fn enable_with_argument_zero_sets_the_flag_even_without_a_reference() {
        let mut e = enable_engine();
        parse_gives(&mut e, true, &[0]);
        let mut a = script(&mut e, 0);
        a.event_list = event_list(&mut e, &[], 0xff);
        start_log(&mut e);
        assert!(e.call(0x005c_43d0, &args![a]).bool());
        assert_eq!(calls(&e, SET_FORM_FLAG_0X8000000), vec![vec![0, 1]]);
        // The disabled bit of the event list is cleared.
        assert_eq!(e.mem.u8(a.event_list + 4), 0xfe);
        assert!(calls(&e, GET_ENABLE_STATE_PARENT).is_empty());
    }

    #[test]
    fn enable_does_the_work_of_an_ordinary_reference() {
        let mut e = enable_engine();
        parse_gives(&mut e, true, &[1]);
        let process_object = e.mem.alloc(0x100);
        e.mem.set_u32(process_object + 0xc4, 9);
        let reference = reference_with_process(&mut e, process_object);
        e.mem.set_u32(reference + 8, 0x800);
        e.register(HAS_EXTRA_FLAG_0X8, |_, _| true.into_ret());
        let mut a = script(&mut e, reference);
        a.event_list = event_list(&mut e, &[], 0x03);
        start_log(&mut e);
        assert!(e.call(0x005c_43d0, &args![a]).bool());
        assert!(calls(&e, SET_FORM_FLAG_0X8000000).is_empty());
        assert_eq!(
            calls(&e, REMOVE_DELAYED_SCRIPT_ACTION_REFERENCE),
            vec![vec![reference]]
        );
        // Form flag 0x800: queued as pending-enabled.
        assert_eq!(
            calls(&e, ADD_PENDING_ENABLED_REFERENCE),
            vec![vec![reference]]
        );
        // A process in mode 9 gets the mode of the base form (id + 1 here).
        assert_eq!(
            calls(&e, SET_PROCESS_MODE),
            vec![vec![process_object, 0xba5f]]
        );
        // The persistent state is switched back.
        assert_eq!(calls(&e, SET_EXTRA_FLAG_0X8), vec![vec![reference, 0]]);
        assert_eq!(e.mem.u8(a.event_list + 4), 0x02);

        // Without the flag and with the process in another mode: only the
        // delayed actions are removed.
        e.mem.set_u32(reference + 8, 0);
        e.mem.set_u32(process_object + 0xc4, 3);
        e.register(HAS_EXTRA_FLAG_0X8, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005c_43d0, &args![a]).bool());
        assert!(calls(&e, ADD_PENDING_ENABLED_REFERENCE).is_empty());
        assert!(calls(&e, SET_PROCESS_MODE).is_empty());
        assert!(calls(&e, SET_EXTRA_FLAG_0X8).is_empty());
    }

    #[test]
    fn enable_with_an_enable_state_parent_only_logs_a_warning() {
        let mut e = enable_engine();
        parse_gives(&mut e, true, &[1]);
        e.register(GET_ENABLE_STATE_PARENT, |_, _| 1u32.into_ret());
        let reference = reference_with_process(&mut e, 0);
        let script_obj = object_with(&mut e, &[(0x130, V_KIND + 7)]);
        let mut a = script(&mut e, reference);
        a.script_obj = ptr(script_obj);
        a.event_list = event_list(&mut e, &[], 0x01);
        // A script without a form id: the "results script" message.
        start_log(&mut e);
        assert!(e.call(0x005c_43d0, &args![a]).bool());
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_ENABLE_IN_RESULTS_SCRIPT, 0x1234, 0x7777, 0, 7]]
        );
        // The early return leaves the event list alone.
        assert_eq!(e.mem.u8(a.event_list + 4), 0x01);
        assert!(calls(&e, REMOVE_DELAYED_SCRIPT_ACTION_REFERENCE).is_empty());
        // A script with a form id: the "in script" message, and the name
        // comes from the guarded accessor's slot when it is set.
        e.mem.set_u32(script_obj + 0xc, 0x99);
        e.register(GET_NAME_GUARDED, |_, _| 1u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005c_43d0, &args![a]).bool());
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_ENABLE_IN_SCRIPT, 0x1234, 6, 0x99, 7]]
        );
        // The unguarded name comes from 0055d520.
        e.register(GET_NAME_GUARDED, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005c_43d0, &args![a]).bool());
        assert_eq!(calls(&e, LOG_STUB)[0][2], 0x7777);
    }

    #[test]
    fn disable_marks_the_reference_pending_disabled() {
        let mut e = enable_engine();
        parse_gives(&mut e, true, &[1]);
        let process_object = e.mem.alloc(0x100);
        let reference = reference_with_process(&mut e, process_object);
        let mut a = script(&mut e, reference);
        a.event_list = event_list(&mut e, &[], 0x00);
        start_log(&mut e);
        assert!(e.call(0x005c_45e0, &args![a]).bool());
        // A positive argument: the process gets mode 9 and the persistent
        // state is set.
        assert_eq!(calls(&e, SET_PROCESS_MODE), vec![vec![process_object, 9]]);
        assert_eq!(calls(&e, SET_EXTRA_FLAG_0X8), vec![vec![reference, 1]]);
        assert_eq!(
            calls(&e, ADD_PENDING_DISABLED_REFERENCE),
            vec![vec![reference, 1]]
        );
        assert_eq!(e.mem.u8(a.event_list + 4), 0x01);

        // Argument 0: only queued, with the flag false.
        parse_gives(&mut e, true, &[0]);
        e.mem.set_u8(a.event_list + 4, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_45e0, &args![a]).bool());
        assert!(calls(&e, SET_PROCESS_MODE).is_empty());
        assert!(calls(&e, SET_EXTRA_FLAG_0X8).is_empty());
        assert_eq!(
            calls(&e, ADD_PENDING_DISABLED_REFERENCE),
            vec![vec![reference, 0]]
        );

        // Already persistent-disabled: the state is not set again.
        parse_gives(&mut e, true, &[2]);
        e.register(HAS_EXTRA_FLAG_0X8, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005c_45e0, &args![a]).bool());
        assert!(calls(&e, SET_EXTRA_FLAG_0X8).is_empty());
    }

    #[test]
    fn disable_leaves_flagged_references_alone_but_still_sets_the_bit() {
        let mut e = enable_engine();
        parse_gives(&mut e, true, &[1]);
        let reference = reference_with_process(&mut e, 0);
        let mut a = script(&mut e, reference);
        a.event_list = event_list(&mut e, &[], 0x00);
        for flag in [0x800u32, 0x4000] {
            e.mem.set_u32(reference + 8, flag);
            e.mem.set_u8(a.event_list + 4, 0);
            start_log(&mut e);
            assert!(e.call(0x005c_45e0, &args![a]).bool());
            assert!(calls(&e, ADD_PENDING_DISABLED_REFERENCE).is_empty());
            assert!(calls(&e, GET_ENABLE_STATE_PARENT).is_empty());
            assert_eq!(e.mem.u8(a.event_list + 4), 0x01);
        }
        // No reference: the bit is set as well.
        let mut none = a;
        none.this_obj = Ptr::NULL;
        e.mem.set_u8(a.event_list + 4, 0);
        assert!(e.call(0x005c_45e0, &args![none]).bool());
        assert_eq!(e.mem.u8(a.event_list + 4), 0x01);
        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_45e0, &args![a]).bool());
    }

    #[test]
    fn disable_with_an_enable_state_parent_only_logs_a_warning() {
        let mut e = enable_engine();
        parse_gives(&mut e, true, &[1]);
        e.register(GET_ENABLE_STATE_PARENT, |_, _| 1u32.into_ret());
        let reference = reference_with_process(&mut e, 0);
        let script_obj = object_with(&mut e, &[(0x130, V_KIND + 7)]);
        e.mem.set_u32(script_obj + 0xc, 0x99);
        let mut a = script(&mut e, reference);
        a.script_obj = ptr(script_obj);
        a.event_list = event_list(&mut e, &[], 0x00);
        start_log(&mut e);
        assert!(e.call(0x005c_45e0, &args![a]).bool());
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_DISABLE_IN_SCRIPT, 0x1234, 0x7777, 0x99, 7]]
        );
        assert!(calls(&e, ADD_PENDING_DISABLED_REFERENCE).is_empty());
        assert_eq!(e.mem.u8(a.event_list + 4), 0x00);
        // A script without a form id.
        e.mem.set_u32(script_obj + 0xc, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_45e0, &args![a]).bool());
        assert_eq!(calls(&e, LOG_STUB)[0][0], MSG_DISABLE_IN_RESULTS_SCRIPT);
    }

    #[test]
    fn disable_all_actors_skips_the_reference_flagged_ones_and_non_actors() {
        let mut e = engine();
        accept(&mut e, &[ADD_PENDING_DISABLED_REFERENCE]);
        // Four entries in list 3: an actor, the running reference itself,
        // an actor with form flag 0x800 and a non-actor.
        let actor = object_with(&mut e, &[(0x100, V_TRUE)]);
        let this_actor = object_with(&mut e, &[(0x100, V_TRUE)]);
        let flagged = object_with(&mut e, &[(0x100, V_TRUE)]);
        e.mem.set_u32(flagged + 8, 0x800);
        let other = object_with(&mut e, &[(0x100, V_FALSE)]);
        let entries = [actor, this_actor, flagged, other, 0];
        let table = e.mem.alloc(0x40);
        for (i, entry) in entries.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *entry);
        }
        e.register(PROCESS_LISTS_COUNT, |_, a| {
            assert_eq!(a[1], 3);
            5u32.into_ret()
        });
        e.register_double(PROCESS_LISTS_ACTOR, move |e, a| {
            e.mem.u32(table + 4 * a[1]).into_ret()
        });
        e.register_double(DYNAMIC_CAST, move |_, a| {
            assert_eq!(a[1..], [0, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0]);
            this_actor.into_ret()
        });
        let mut a = script(&mut e, 0x40);
        a.event_list = event_list(&mut e, &[], 0x00);
        start_log(&mut e);
        assert!(e.call(0x005c_47e0, &args![a]).bool());
        assert_eq!(
            calls(&e, ADD_PENDING_DISABLED_REFERENCE),
            vec![vec![actor, 1]]
        );
        assert_eq!(e.mem.u8(a.event_list + 4), 0x01);
        // The process lists singleton is the `this` of the list accessors.
        assert_eq!(calls(&e, PLUS_4)[0], vec![PROCESS_LISTS]);

        // An empty list does nothing and leaves the bit alone.
        e.register(PROCESS_LISTS_COUNT, |_, _| 0u32.into_ret());
        e.mem.set_u8(a.event_list + 4, 0);
        assert!(e.call(0x005c_47e0, &args![a]).bool());
        assert_eq!(e.mem.u8(a.event_list + 4), 0);
    }

    // ---- Condition wrappers -----------------------------------------------------

    /// A wrapper command that forwards to a condition function with `want`
    /// as the argument words and returns its `AL`.
    fn check_condition_wrapper(entry: u32, callee: u32, second: Option<u32>) {
        let mut e = engine();
        e.register(callee, |_, _| true.into_ret());
        let mut a = script(&mut e, 0x40);
        a.event_list = 0x66;
        start_log(&mut e);
        assert!(e.call(entry, &args![a]).bool());
        assert_eq!(
            calls(&e, callee),
            vec![vec![0x40, second.unwrap_or(0), 0, a.result.addr()]]
        );
        e.register(callee, |_, _| false.into_ret());
        assert!(!e.call(entry, &args![a]).bool());
    }

    #[test]
    fn disabled_condition_command_forwards_the_event_list() {
        check_condition_wrapper(0x005c_48c0, GET_DISABLED_CONDITION, Some(0x66));
    }

    #[test]
    fn locked_condition_command_forwards_the_event_list() {
        check_condition_wrapper(0x005c_48e0, GET_LOCKED_CONDITION, Some(0x66));
    }

    #[test]
    fn lock_level_condition_command_forwards_the_event_list() {
        check_condition_wrapper(0x005c_4900, GET_LOCK_LEVEL_CONDITION, Some(0x66));
    }

    #[test]
    fn lock_broken_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_4920, GET_IS_LOCK_BROKEN_CONDITION, None);
    }

    #[test]
    fn disease_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_67e0, GET_DISEASE_CONDITION, None);
    }

    #[test]
    fn vampire_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_6800, GET_VAMPIRE_CONDITION, None);
    }

    #[test]
    fn form_type_command_compares_the_type_name_with_the_argument() {
        let mut e = engine();
        e.register(STRING_COMPARE, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            (x.cmp(&y) as i32 as u32).into_ret()
        });
        // The argument is parsed into the buffer (the only output).
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"ARMO");
            true.into_ret()
        });
        // Form type 0x28 has the name "ARMO" in the table (12-byte entries).
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"ARMO");
        e.mem.set_u32(FORM_TYPE_NAMES + 0x28 * 12, name);
        let base = e.mem.alloc(0x20);
        e.mem.set_u8(base + 4, 0x28);
        let reference = e.mem.alloc(0x40);
        e.mem.set_u32(reference + 0x20, base);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(e.call(0x005c_4940, &args![a]).bool());
        assert_eq!(result_of(&e, a), 1.0);
        // The console echo is off: nothing printed.
        assert!(calls(&e, CONSOLE_PRINT).is_empty());

        // Another type: 0.0, and with the echo on the value is printed.
        e.mem.set_u8(base + 4, 0x29);
        let other = e.mem.alloc(8);
        e.mem.set_cstr(other, b"WEAP");
        e.mem.set_u32(FORM_TYPE_NAMES + 0x29 * 12, other);
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_ECHO, 1);
        start_log(&mut e);
        assert!(e.call(0x005c_4940, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_IS_FORM_TYPE, 0, 0]]
        );

        // Parameters that do not parse: false, result 0.0.
        e.register(PARSE_PARAMETERS, |_, _| false.into_ret());
        e.mem.set_f64(a.result.addr(), 5.0);
        assert!(!e.call(0x005c_4940, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
    }

    #[test]
    fn voice_type_command_passes_the_parsed_argument_to_the_condition() {
        let mut e = engine();
        parse_gives(&mut e, true, &[0x55]);
        e.register(GET_IS_VOICE_TYPE_CONDITION, |_, _| true.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(e.call(0x005c_4a10, &args![a]).bool());
        assert_eq!(
            calls(&e, GET_IS_VOICE_TYPE_CONDITION),
            vec![vec![0x40, 0x55, 0, a.result.addr()]]
        );
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_4a10, &args![a]).bool());
        assert!(calls(&e, GET_IS_VOICE_TYPE_CONDITION).is_empty());
    }

    // ---- PlaceAtMe ----------------------------------------------------------------

    fn point3(e: &Engine, p: u32) -> [f32; 3] {
        [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]
    }

    fn put_point3(e: &mut Engine, p: u32, v: [f32; 3]) {
        for (i, value) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *value);
        }
    }

    fn assert_near(actual: [f32; 3], expected: [f32; 3]) {
        for i in 0..3 {
            assert!(
                (actual[i] - expected[i]).abs() < 1e-3,
                "{actual:?} != {expected:?}"
            );
        }
    }

    /// The positions (and forms) the data handler was asked to create
    /// references at.
    type Created = std::rc::Rc<std::cell::RefCell<Vec<(u32, [f32; 3], [f32; 3])>>>;

    /// Doubles for everything `fn_005c4b30` calls: the `NiPoint3` operators
    /// do their arithmetic, the trigonometric wrappers are cosine (x) and
    /// sine (y), the ring starts at angle 0, every position is in the loaded
    /// area and the data handler creates actors (virtual slots `0x100` and
    /// `0x41c`). Returns the engine and the list of created references.
    fn place_engine() -> (Engine, Created) {
        let mut e = engine();
        let created: Created = Default::default();
        e.set_global(RING_ANGLE_STEP, 0.785_398_185_253_143_3f64);
        e.set_global(RING_RADIUS, 100.0f32);
        e.set_global(PICK_HEIGHT, 64.0f64);
        e.set_global(TES, 0x7e5u32);
        e.set_global(DATA_HANDLER, 0xda7au32);
        e.register(GET_ROTATION, |_, a| (a[0] + 0x24).into_ret());
        e.register(POINT3_CONSTRUCT, |e, a| {
            put_point3(
                e,
                a[0],
                [
                    f32::from_bits(a[1]),
                    f32::from_bits(a[2]),
                    f32::from_bits(a[3]),
                ],
            );
            a[0].into_ret()
        });
        e.register(GET_WORLD_ROTATE, |_, a| (a[0] + 0x34).into_ret());
        e.register(GET_WORLD_TRANSLATE, |_, a| (a[0] + 0x58).into_ret());
        e.register(MATRIX_ELEMENT, |e, a| {
            e.mem.f32(a[0] + a[1] * 12 + a[2] * 4).into_ret()
        });
        e.register(POINT3_SCALE, |e, a| {
            let v = point3(e, a[0]);
            let f = f32::from_bits(a[1]);
            put_point3(e, a[0], v.map(|x| x * f));
            a[0].into_ret()
        });
        e.register(POINT3_SUBTRACT_ASSIGN, |e, a| {
            let (v, w) = (point3(e, a[0]), point3(e, a[1]));
            put_point3(e, a[0], [v[0] - w[0], v[1] - w[1], v[2] - w[2]]);
            a[0].into_ret()
        });
        e.register(POINT3_ADD_ASSIGN, |e, a| {
            let (v, w) = (point3(e, a[0]), point3(e, a[1]));
            put_point3(e, a[0], [v[0] + w[0], v[1] + w[1], v[2] + w[2]]);
            a[0].into_ret()
        });
        e.register(POINT3_SCALED, |e, a| {
            let v = point3(e, a[0]);
            let f = f32::from_bits(a[2]);
            put_point3(e, a[1], v.map(|x| x * f));
            a[1].into_ret()
        });
        e.register(POINT3_SUM, |e, a| {
            let (v, w) = (point3(e, a[0]), point3(e, a[2]));
            put_point3(e, a[1], [v[0] + w[0], v[1] + w[1], v[2] + w[2]]);
            a[1].into_ret()
        });
        e.register(POINT3_DIFFERENCE, |e, a| {
            let (v, w) = (point3(e, a[0]), point3(e, a[2]));
            put_point3(e, a[1], [v[0] - w[0], v[1] - w[1], v[2] - w[2]]);
            a[1].into_ret()
        });
        e.register(POINT3_UNITIZE, |e, a| {
            let v = point3(e, a[0]);
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            put_point3(
                e,
                a[0],
                v.map(|x| if length > 0.0 { x / length } else { 0.0 }),
            );
            Ret::default()
        });
        accept(
            &mut e,
            &[POINT3_DEFAULT_CONSTRUCT, VECTOR_CONSTRUCTOR_ITERATOR],
        );
        e.register(RING_X, |_, a| f32::from_bits(a[0]).cos().into_ret());
        e.register(RING_Y, |_, a| f32::from_bits(a[0]).sin().into_ret());
        e.register(RANDOM_OBJECT, |_, _| 1u32.into_ret());
        e.register(RANDOM_ANGLE_FROM, |_, _| 0.0f32.into_ret());
        e.register(GET_WORLD_SPACE, |_, _| 0x5ace_u32.into_ret());
        e.register(GET_PARENT_CELL, |_, _| 0xce11u32.into_ret());
        e.register(IS_POSITION_LOADED, |_, _| true.into_ret());
        accept(
            &mut e,
            &[FADE_IN, SET_HEALTH, PICK_CONSTRUCT, PICK_SET_MASK],
        );
        accept(&mut e, &[PICK_SET_START, PICK_SET_END]);
        e.register(STORE_VALUE, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(MASK_SET_LOW_BITS, |e, a| {
            let mask = (e.mem.u32(a[0]) & !0x7f) | (a[1] & 0x7f);
            e.mem.set_u32(a[0], mask);
            Ret::default()
        });
        e.register(MASK_SET_HIGH_BITS, |e, a| {
            let mask = (e.mem.u32(a[0]) & 0xffff) | (a[1] << 16);
            e.mem.set_u32(a[0], mask);
            Ret::default()
        });
        e.register(STORE_BYTE, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            Ret::default()
        });
        e.register(BYTE_IS_SET, |e, a| (e.mem.u8(a[0]) != 0).into_ret());
        accept(&mut e, &[TES_PICK]);
        e.register(GET_BOUND_SIZE, |_, _| 10.0f32.into_ret());
        e.register(GET_FORM_AS_HEALTH_FORM, |_, _| 0u32.into_ret());
        let log = created.clone();
        e.register_double(CREATE_REFERENCE, move |e, a| {
            assert_eq!(a[0], 0xda7a, "the data handler is `this`");
            assert_eq!(a[4..], [0xce11, 0x5ace, 0, 0, 0]);
            let position = point3(e, a[2]);
            let rotation = point3(e, a[3]);
            log.borrow_mut().push((a[1], position, rotation));
            let index = log.borrow().len() as u32;
            let reference = object_with(e, &[(0x100, V_TRUE), (0x41c, V_RECORD)]);
            e.mem.set_u32(reference + 0xc, 0x00ff_0000 + index);
            reference.into_ret()
        });
        (e, created)
    }

    /// A reference at `position` with `rotation`, whose 3D node (when
    /// `node` is non-zero) is at `node + ...`.
    fn placing_reference(e: &mut Engine, position: [f32; 3], node: u32) -> u32 {
        let reference = object_with(e, &[(0x1f4, V_POSITION), (0x1d0, V_NODE)]);
        put_point3(e, reference + 0x30, position);
        put_point3(e, reference + 0x24, [0.1, 0.2, 0.3]);
        e.mem.set_u32(reference + 0x100, node);
        reference
    }

    /// A form of `kind` that passes virtual slot `0xe4`.
    fn placed_form(e: &mut Engine, kind: u8) -> u32 {
        let form = object_with(e, &[(0xe4, V_TRUE)]);
        e.mem.set_u8(form + 4, kind);
        form
    }

    #[test]
    fn place_at_me_needs_a_reference_and_a_form() {
        let (mut e, created) = place_engine();
        let reference = placing_reference(&mut e, [0.0; 3], 0);
        let form = placed_form(&mut e, 0x28);
        start_log(&mut e);
        for (r, f) in [(0, form), (reference, 0)] {
            let placed = e
                .call(
                    0x005c_4b30,
                    &args![ptr(r), ptr(f), 1i32, 0i32, 0i32, 1.0f32],
                )
                .u32();
            assert_eq!(placed, 0);
        }
        assert!(created.borrow().is_empty());
        assert!(e.call_log.as_ref().unwrap().len() == 2);
    }

    #[test]
    fn place_at_me_puts_the_first_at_the_reference_and_the_rest_on_the_ring() {
        let (mut e, created) = place_engine();
        let reference = placing_reference(&mut e, [10.0, 20.0, 30.0], 0);
        let form = placed_form(&mut e, 0x28);
        let placed = e
            .call(
                0x005c_4b30,
                &args![ptr(reference), ptr(form), 3i32, 0i32, 0i32, 1.0f32],
            )
            .u32();
        let created = created.borrow();
        assert_eq!(created.len(), 3);
        // The last reference is the result.
        assert_eq!(e.mem.u32(placed + 0xc), 0x00ff_0003);
        // The rotation of the reference is passed along.
        assert!(created
            .iter()
            .all(|c| c.0 == form && c.2 == [0.1, 0.2, 0.3]));
        // The first is at the reference itself.
        assert_eq!(created[0].1, [10.0, 20.0, 30.0]);
        // Ring points: angle 0 then pi/4 at a radius of 100, at the height
        // of the reference.
        assert_near(created[1].1, [110.0, 20.0, 30.0]);
        let step = std::f32::consts::FRAC_PI_4;
        assert_near(
            created[2].1,
            [10.0 + 100.0 * step.cos(), 20.0 + 100.0 * step.sin(), 30.0],
        );
        // Actors are faded in: slot 0x41c (0.0) and `Actor::FadeIn`.
        assert_eq!(e.mem.u32(placed + 0xc), 0x00ff_0003);
    }

    #[test]
    fn place_at_me_wraps_around_the_ring_after_nine_positions() {
        let (mut e, created) = place_engine();
        let reference = placing_reference(&mut e, [0.0, 0.0, 0.0], 0);
        let form = placed_form(&mut e, 0x28);
        e.call(
            0x005c_4b30,
            &args![ptr(reference), ptr(form), 11i32, 0i32, 0i32, 1.0f32],
        );
        let created = created.borrow();
        assert_eq!(created.len(), 11);
        // Position 9 (index 9) is the reference's own again, 10 is ring point 1.
        assert_eq!(created[9].1, [0.0, 0.0, 0.0]);
        assert_near(created[10].1, created[1].1);
    }

    #[test]
    fn place_at_me_without_a_loaded_ring_position_uses_the_reference_position() {
        let (mut e, created) = place_engine();
        e.register(IS_POSITION_LOADED, |_, _| false.into_ret());
        let reference = placing_reference(&mut e, [1.0, 2.0, 3.0], 0);
        let form = placed_form(&mut e, 0x28);
        e.call(
            0x005c_4b30,
            &args![ptr(reference), ptr(form), 2i32, 0i32, 0i32, 1.0f32],
        );
        let created = created.borrow();
        assert_eq!(created[0].1, [1.0, 2.0, 3.0]);
        assert_eq!(created[1].1, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn place_at_me_casts_a_ray_and_backs_off_when_it_hits() {
        let (mut e, created) = place_engine();
        // The pick reports a hit: dword `+0x50` of the pick result.
        e.register(TES_PICK, |e, a| {
            e.mem.set_u32(a[1] + 0x30 + 0x50, 1);
            Ret::default()
        });
        // The ray start and end are recorded when the function sets them.
        let points: std::rc::Rc<std::cell::RefCell<Vec<[f32; 3]>>> = Default::default();
        for addr in [PICK_SET_START, PICK_SET_END] {
            let log = points.clone();
            e.register_double(addr, move |e, a| {
                log.borrow_mut().push(point3(e, a[1]));
                Ret::default()
            });
        }
        let reference = placing_reference(&mut e, [10.0, 20.0, 30.0], 0);
        let form = placed_form(&mut e, 0x28);
        start_log(&mut e);
        e.call(
            0x005c_4b30,
            &args![ptr(reference), ptr(form), 2i32, 0i32, 0i32, 1.0f32],
        );
        // The ray runs from 64 above the reference to 64 above ring point 1.
        assert_eq!(
            *points.borrow(),
            vec![[10.0, 20.0, 94.0], [110.0, 20.0, 94.0]]
        );
        // The pick mask: 0x26 in the low bits, 3 in the high 16.
        assert_eq!(calls(&e, PICK_SET_MASK)[0][1], 0x0003_0026);
        let created = created.borrow();
        assert_eq!(created[0].1, [10.0, 20.0, 30.0]);
        // The end (110, 20, 94) backed off by the bound size (10) along the
        // ray direction (1, 0, 0), at the reference's height.
        assert_near(created[1].1, [100.0, 20.0, 30.0]);
    }

    #[test]
    fn place_at_me_gives_the_health_form_a_scaled_health() {
        let (mut e, created) = place_engine();
        let reference = placing_reference(&mut e, [0.0; 3], 0);
        let form = placed_form(&mut e, 0x28);
        let health_form = object_with(&mut e, &[(0x10, V_KIND + 10)]);
        e.register_double(GET_FORM_AS_HEALTH_FORM, move |_, a| {
            assert_eq!(a[0], form);
            health_form.into_ret()
        });
        start_log(&mut e);
        let placed = e
            .call(
                0x005c_4b30,
                &args![ptr(reference), ptr(form), 1i32, 0i32, 0i32, 0.5f32],
            )
            .u32();
        assert_eq!(created.borrow().len(), 1);
        // Slot 0x10 gives 10: 10 * 0.5.
        assert_eq!(calls(&e, SET_HEALTH), vec![vec![placed, 5.0f32.to_bits()]]);
        assert_eq!(calls(&e, FADE_IN), vec![vec![placed]]);
        assert_eq!(calls(&e, V_RECORD), vec![vec![placed, 0]]);
    }

    #[test]
    fn place_at_me_stops_when_a_reference_cannot_be_created() {
        let (mut e, created) = place_engine();
        let log = created.clone();
        e.register_double(CREATE_REFERENCE, move |e, a| {
            log.borrow_mut().push((a[1], [0.0; 3], [0.0; 3]));
            if log.borrow().len() == 2 {
                0u32.into_ret()
            } else {
                object_with(e, &[(0x100, V_FALSE)]).into_ret()
            }
        });
        let reference = placing_reference(&mut e, [0.0; 3], 0);
        let form = placed_form(&mut e, 0x28);
        // The second creation fails: the function returns 0 at once.
        let placed = e
            .call(
                0x005c_4b30,
                &args![ptr(reference), ptr(form), 3i32, 0i32, 0i32, 1.0f32],
            )
            .u32();
        assert_eq!(placed, 0);
        assert_eq!(created.borrow().len(), 2);
    }

    #[test]
    fn place_at_me_ignores_forms_that_fail_virtual_slot_0xe4() {
        let (mut e, created) = place_engine();
        let reference = placing_reference(&mut e, [0.0; 3], 0);
        let form = object_with(&mut e, &[(0xe4, V_FALSE)]);
        e.mem.set_u8(form + 4, 0x28);
        let placed = e
            .call(
                0x005c_4b30,
                &args![ptr(reference), ptr(form), 2i32, 0i32, 0i32, 1.0f32],
            )
            .u32();
        assert_eq!(placed, 0);
        assert!(created.borrow().is_empty());
    }

    /// A leveled-list form (type 0x2c or 0x2d) whose roll fills the
    /// container with the items `(count, form)`.
    fn install_leveled_roll(e: &mut Engine, items: &[(i32, u32)]) {
        let items = items.to_vec();
        e.register(GET_CALC_LEVEL, |_, _| 0x0001_0005u32.into_ret());
        e.register(TES_CONTAINER_CONSTRUCT, |_, _| Ret::default());
        e.register(TES_CONTAINER_DESTRUCT, |_, _| Ret::default());
        e.register_double(ROLL_LEVELED_LIST, move |e, a| {
            // The container's list head is at +4 (item at +0, next at +4).
            let mut node = a[3] + 4;
            for (i, (count, form)) in items.iter().enumerate() {
                let entry = e.mem.alloc(8);
                e.mem.set_i32(entry, *count);
                e.mem.set_u32(entry + 4, *form);
                e.mem.set_u32(node, entry);
                if i + 1 < items.len() {
                    let next = e.mem.alloc(8);
                    e.mem.set_u32(node + 4, next);
                    node = next;
                }
            }
            Ret::default()
        });
        e.register(LIST_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_ITEM_PTR, |_, a| a[0].into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
    }

    #[test]
    fn place_at_me_rolls_a_leveled_list_and_creates_each_item_at_the_centre() {
        for kind in [0x2c, 0x2d] {
            let (mut e, created) = place_engine();
            install_leveled_roll(&mut e, &[(2, 0xf1), (1, 0xf2)]);
            let reference = placing_reference(&mut e, [10.0, 20.0, 30.0], 0);
            let list = placed_form(&mut e, kind);
            start_log(&mut e);
            let placed = e
                .call(
                    0x005c_4b30,
                    &args![
                        ptr(reference),
                        ptr(list),
                        0x0002_0004i32,
                        0i32,
                        0i32,
                        1.0f32
                    ],
                )
                .u32();
            // Level 5 (low 16 bits of the calc level) and count 4 (low 16
            // bits of the count), rolled at `form + 0x30`.
            let roll = &calls(&e, ROLL_LEVELED_LIST)[0];
            assert_eq!(roll[0], list + 0x30);
            assert_eq!(roll[1..3], [5, 4]);
            assert_eq!(roll[4], 0);
            let created = created.borrow();
            let forms: Vec<u32> = created.iter().map(|c| c.0).collect();
            assert_eq!(forms, vec![0xf1, 0xf1, 0xf2]);
            // All at the centre (no distance).
            assert!(created.iter().all(|c| c.1 == [10.0, 20.0, 30.0]));
            assert_eq!(e.mem.u32(placed + 0xc), 0x00ff_0003);
            assert_eq!(calls(&e, TES_CONTAINER_DESTRUCT).len(), 1);
            // The leveled-list branch never asks slot 0xe4.
            assert!(calls(&e, V_TRUE).iter().all(|w| w[0] != list));
        }
    }

    #[test]
    fn place_at_me_offsets_the_centre_along_the_nodes_rotation() {
        // (direction, expected centre) for a node at (1, 2, 3) whose world
        // rotation columns are x = (1,0,0) (column 0) and y = (0,1,0)
        // (column 1), and a distance of 10.
        for (direction, expected) in [
            (0, [1.0, 12.0, 3.0]),
            (1, [1.0, -8.0, 3.0]),
            (2, [-9.0, 2.0, 3.0]),
            (3, [11.0, 2.0, 3.0]),
            (7, [1.0, 12.0, 3.0]),
        ] {
            let (mut e, created) = place_engine();
            install_leveled_roll(&mut e, &[(1, 0xf1)]);
            let node = e.mem.alloc(0x100);
            // Rows of the matrix at +0x34: m[0] = (1, 0, 0), m[1] = (0, 1, 0)
            // so that column 0 is (1, 0, 0) and column 1 is (0, 1, 0)... the
            // matrix is stored so that element (row, column) is
            // `12 * row + 4 * column`.
            for (row, column, value) in [(0, 0, 1.0f32), (1, 1, 1.0)] {
                e.mem.set_f32(node + 0x34 + 12 * row + 4 * column, value);
            }
            put_point3(&mut e, node + 0x58, [1.0, 2.0, 3.0]);
            let reference = placing_reference(&mut e, [100.0, 100.0, 100.0], node);
            let list = placed_form(&mut e, 0x2c);
            e.call(
                0x005c_4b30,
                &args![ptr(reference), ptr(list), 1i32, 10i32, direction, 1.0f32],
            );
            let created = created.borrow();
            assert_near(created[0].1, expected);
        }
    }

    #[test]
    fn place_at_me_command_stores_the_id_of_the_placed_reference() {
        let (mut e, created) = place_engine();
        e.register(PUT_NUMERIC_ID_IN_DOUBLE, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_f64(a[1], id as f64 + 0.5);
            Ret::default()
        });
        let reference = placing_reference(&mut e, [1.0, 2.0, 3.0], 0);
        let form = placed_form(&mut e, 0x28);
        // The arguments: form, count (default 1), distance, direction.
        parse_gives(&mut e, true, &[form, 2, 0, 0]);
        let a = script(&mut e, reference);
        e.mem.set_f64(a.result.addr(), 9.0);
        start_log(&mut e);
        assert!(e.call(0x005c_4a70, &args![a]).bool());
        assert_eq!(created.borrow().len(), 2);
        // The form id of the last reference placed (health scale 1.0).
        assert_eq!(result_of(&e, a), 0x00ff_0002 as f64 + 0.5);
        // The locals start as form 0, count 1, distance 0, direction 0.
        let parse = &calls(&e, PARSE_PARAMETERS)[0];
        assert_eq!(parse.len(), 7 + 4);

        // Nothing placed: the result stays 0.0 and no id is stored.
        parse_gives(&mut e, true, &[0, 1, 0, 0]);
        e.mem.set_f64(a.result.addr(), 9.0);
        assert!(e.call(0x005c_4a70, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);

        // Parameters that do not parse: false, result 0.0.
        parse_gives(&mut e, false, &[]);
        e.mem.set_f64(a.result.addr(), 9.0);
        assert!(!e.call(0x005c_4a70, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
    }

    #[test]
    fn place_at_me_command_defaults_to_one_reference() {
        let (mut e, created) = place_engine();
        e.register(PUT_NUMERIC_ID_IN_DOUBLE, |_, _| Ret::default());
        let reference = placing_reference(&mut e, [1.0, 2.0, 3.0], 0);
        let form = placed_form(&mut e, 0x28);
        // The parse double leaves the defaults and only gives the form.
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            e.mem.set_u32(a[7], form);
            true.into_ret()
        });
        let a = script(&mut e, reference);
        assert!(e.call(0x005c_4a70, &args![a]).bool());
        assert_eq!(created.borrow().len(), 1);
    }

    // ---- Small helpers of the placement ----------------------------------------

    #[test]
    fn ring_y_wrapper_returns_the_float_of_the_callee() {
        let mut e = engine();
        e.register(RING_Y, |_, a| (f32::from_bits(a[0]) * 2.0).into_ret());
        start_log(&mut e);
        assert_eq!(e.call(0x005c_53d0, &args![1.5f32]).f32(), 3.0);
        assert_eq!(calls(&e, RING_Y), vec![vec![1.5f32.to_bits()]]);
    }

    #[test]
    fn pick_result_helper_stores_whether_the_result_hit() {
        let mut e = engine();
        e.register(STORE_BYTE, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            Ret::default()
        });
        let pick_result = e.mem.alloc(0x100);
        let byte = e.mem.alloc(4);
        assert_eq!(
            e.call(0x005c_53f0, &args![ptr(pick_result), byte]).u32(),
            byte
        );
        assert_eq!(e.mem.u8(byte), 0);
        e.mem.set_u32(pick_result + 0x50, 0x1000_0000);
        assert_eq!(
            e.call(0x005c_53f0, &args![ptr(pick_result), byte]).u32(),
            byte
        );
        assert_eq!(e.mem.u8(byte), 1);
    }

    #[test]
    fn ring_start_angle_comes_from_the_random_object() {
        let mut e = engine();
        e.register(RANDOM_OBJECT, |_, _| 9u32.into_ret());
        e.register_double(RANDOM_ANGLE_FROM, |_, a| {
            assert_eq!(a[0], 9);
            2.5f32.into_ret()
        });
        assert_eq!(e.call(0x005c_5420, &args![]).f32(), 2.5);
    }

    // ---- Save games ---------------------------------------------------------------

    /// A `ParseParameters` double that gives the name "quick" and a flag.
    fn parse_save_name(e: &mut Engine, flag: u32) {
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            e.mem.set_cstr(a[7], b"quick");
            e.mem.set_u32(a[8], flag);
            true.into_ret()
        });
    }

    /// The argument words and the name string of each save call.
    type SavedCalls = std::rc::Rc<std::cell::RefCell<Vec<(Vec<u32>, Vec<u8>)>>>;

    #[test]
    fn save_game_saves_under_the_given_name() {
        let mut e = engine();
        e.set_global(SAVE_LOAD_MANAGER, 0x3a9eu32);
        parse_save_name(&mut e, 1);
        let saved: SavedCalls = Default::default();
        let log = saved.clone();
        e.register_double(SAVE_GAME, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.cstr(a[1])));
            Ret::default()
        });
        let a = script(&mut e, 0);
        assert!(e.call(0x005c_5440, &args![a]).bool());
        let saved_calls = saved.borrow();
        assert_eq!(saved_calls[0].1, b"quick");
        assert_eq!(saved_calls[0].0[0], 0x3a9e);
        assert_eq!(saved_calls[0].0[2..], [0xffff_ffff, 1]);
        drop(saved_calls);
        // Flag 0: false; parameters that do not parse: nothing saved.
        parse_save_name(&mut e, 0);
        assert!(e.call(0x005c_5440, &args![a]).bool());
        assert_eq!(saved.borrow()[1].0[3], 0);
        e.register(PARSE_PARAMETERS, |_, _| false.into_ret());
        assert!(!e.call(0x005c_5440, &args![a]).bool());
        assert_eq!(saved.borrow().len(), 2);
    }

    #[test]
    fn save_game_variant_passes_an_extra_one() {
        let mut e = engine();
        e.set_global(SAVE_LOAD_MANAGER, 0x3a9eu32);
        parse_save_name(&mut e, 5);
        accept(&mut e, &[SAVE_GAME_WITH_FLAG]);
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_54d0, &args![a]).bool());
        let call = &calls(&e, SAVE_GAME_WITH_FLAG)[0];
        assert_eq!(call[0], 0x3a9e);
        assert_eq!(call[2..], [0xffff_ffff, 1, 1]);
        e.register(PARSE_PARAMETERS, |_, _| false.into_ret());
        assert!(!e.call(0x005c_54d0, &args![a]).bool());
        assert_eq!(calls(&e, SAVE_GAME_WITH_FLAG).len(), 1);
    }

    /// A setting whose value byte is `value`.
    fn setting_with_value(e: &mut Engine, value: u8) {
        let byte = e.mem.alloc(8);
        e.mem.set_u8(byte, value);
        e.register_double(GET_SETTING_VALUE_ADDRESS, move |_, _| byte.into_ret());
    }

    #[test]
    fn manager_action_runs_when_the_setting_is_set() {
        let mut e = engine();
        e.set_global(SAVE_LOAD_MANAGER, 0x3a9eu32);
        accept(&mut e, &[MANAGER_ACTION_A]);
        setting_with_value(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_5560, &args![]).bool());
        assert!(calls(&e, MANAGER_ACTION_A).is_empty());
        assert_eq!(calls(&e, GET_SETTING_VALUE_ADDRESS), vec![vec![SETTING_A]]);
        setting_with_value(&mut e, 1);
        assert!(e.call(0x005c_5560, &args![]).bool());
        assert_eq!(calls(&e, MANAGER_ACTION_A), vec![vec![0x3a9e]]);
    }

    #[test]
    fn second_manager_action_also_needs_the_path_handler() {
        let mut e = engine();
        e.set_global(SAVE_LOAD_MANAGER, 0x3a9eu32);
        accept(&mut e, &[MANAGER_ACTION_B]);
        e.register(IS_DETAILED_PATH_HANDLER, |_, _| true.into_ret());
        setting_with_value(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_5590, &args![]).bool());
        assert!(calls(&e, MANAGER_ACTION_B).is_empty());
        assert!(calls(&e, IS_DETAILED_PATH_HANDLER).is_empty());
        assert_eq!(calls(&e, GET_SETTING_VALUE_ADDRESS), vec![vec![SETTING_B]]);
        setting_with_value(&mut e, 1);
        assert!(e.call(0x005c_5590, &args![]).bool());
        assert_eq!(
            calls(&e, IS_DETAILED_PATH_HANDLER),
            vec![vec![0x3a9e, 0xffff_ffff]]
        );
        assert_eq!(calls(&e, MANAGER_ACTION_B), vec![vec![0x3a9e]]);
        e.register(IS_DETAILED_PATH_HANDLER, |_, _| false.into_ret());
        assert!(e.call(0x005c_5590, &args![]).bool());
        assert_eq!(calls(&e, MANAGER_ACTION_B).len(), 1);
    }

    #[test]
    fn copy_save_games_compares_the_argument_with_ms() {
        let mut e = engine();
        e.register(STRING_COPY, |e, a| {
            let text = e.mem.cstr(a[1]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRING_COMPARE_NO_CASE, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            let (x, y) = (x.to_ascii_lowercase(), y.to_ascii_lowercase());
            (x.cmp(&y) as i32 as u32).into_ret()
        });
        accept(&mut e, &[COPY_SAVE_GAMES]);
        e.set_global(SAVE_LOAD_MANAGER, 0x3a9eu32);
        e.mem.set_cstr(TEXT_MS, b"ms");
        let a = script(&mut e, 0);
        for (argument, expected) in [(&b"MS"[..], 1u32), (b"pc", 0), (b"", 0)] {
            e.register_double(PARSE_PARAMETERS, move |e, a| {
                e.mem.set_cstr(a[7], argument);
                true.into_ret()
            });
            start_log(&mut e);
            assert!(e.call(0x005c_55d0, &args![a]).bool());
            assert_eq!(calls(&e, COPY_SAVE_GAMES), vec![vec![0x3a9e, expected]]);
            // The buffer starts as the default (empty) string.
            assert_eq!(calls(&e, STRING_COPY)[0][1], EMPTY_STRING);
        }
        // No manager: nothing copied.
        e.set_global(SAVE_LOAD_MANAGER, 0u32);
        start_log(&mut e);
        assert!(e.call(0x005c_55d0, &args![a]).bool());
        assert!(calls(&e, COPY_SAVE_GAMES).is_empty());
        // Parameters that do not parse.
        e.register(PARSE_PARAMETERS, |_, _| false.into_ret());
        assert!(!e.call(0x005c_55d0, &args![a]).bool());
    }

    // ---- Node, tree and camera commands ---------------------------------------------

    #[test]
    fn toggle_cell_node_prints_the_new_state_of_the_named_node() {
        let mut e = engine();
        e.set_global(TES, 0x7e5u32);
        e.register(GET_CELL_NODE_FLAG, |_, a| (a[0] % 2 == 1).into_ret());
        accept(&mut e, &[APPLY_CELL_NODE_STATE, REFRESH_PATH_NODES]);
        e.register(GET_CURRENT_NODE_INDEX, |_, _| 0u32.into_ret());
        let a = script(&mut e, 0);
        for (index, name, state) in [
            (0u32, NODE_ACTOR, TEXT_CULLED),
            (1, NODE_MARKER, TEXT_DISPLAYED),
            (2, NODE_LAND, TEXT_CULLED),
            (3, NODE_STATIC, TEXT_DISPLAYED),
            (4, NODE_DYNAMIC, TEXT_CULLED),
            (5, NODE_MULTI_BOUND, TEXT_DISPLAYED),
            (6, NODE_WATER, TEXT_CULLED),
        ] {
            parse_gives(&mut e, true, &[index]);
            start_log(&mut e);
            assert!(e.call(0x005c_5680, &args![a]).bool());
            let shown = u32::from(index % 2 == 1);
            assert_eq!(
                calls(&e, APPLY_CELL_NODE_STATE),
                vec![vec![0x7e5, index, shown, 0]]
            );
            assert_eq!(
                calls(&e, CONSOLE_PRINT),
                vec![vec![MSG_CELL_NODES, name, state]]
            );
            assert!(calls(&e, REFRESH_PATH_NODES).is_empty());
        }
    }

    #[test]
    fn toggle_cell_node_refreshes_the_path_handler_for_water() {
        let mut e = engine();
        e.set_global(TES, 0x7e5u32);
        e.register(GET_CELL_NODE_FLAG, |_, _| true.into_ret());
        accept(&mut e, &[APPLY_CELL_NODE_STATE, REFRESH_PATH_NODES]);
        e.register(GET_CURRENT_NODE_INDEX, |_, _| 0x4a4au32.into_ret());
        parse_gives(&mut e, true, &[6]);
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_5680, &args![a]).bool());
        assert_eq!(calls(&e, REFRESH_PATH_NODES), vec![vec![0x4a4a, 1, 0, 0]]);
    }

    #[test]
    fn toggle_cell_node_rejects_other_indices_and_ignores_the_default() {
        let mut e = engine();
        e.set_global(TES, 0x7e5u32);
        accept(&mut e, &[APPLY_CELL_NODE_STATE]);
        let a = script(&mut e, 0);
        for index in [7u32, 100, 0x8000_0000] {
            parse_gives(&mut e, true, &[index]);
            start_log(&mut e);
            assert!(e.call(0x005c_5680, &args![a]).bool());
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_INVALID_NODE_INDEX]]);
            assert!(calls(&e, APPLY_CELL_NODE_STATE).is_empty());
        }
        // The default (-1) does nothing; unparsed parameters return false.
        parse_gives(&mut e, true, &[0xffff_ffff]);
        start_log(&mut e);
        assert!(e.call(0x005c_5680, &args![a]).bool());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_5680, &args![a]).bool());
    }

    #[test]
    fn toggle_trees_flips_the_culling_and_prints_the_new_state() {
        let mut e = engine();
        let culled = std::rc::Rc::new(std::cell::Cell::new(false));
        let state = culled.clone();
        e.register_double(GET_TREES_CULLED, move |_, _| state.get().into_ret());
        let state = culled.clone();
        e.register_double(SET_TREES_CULLED, move |_, a| {
            state.set(a[0] != 0);
            Ret::default()
        });
        start_log(&mut e);
        assert!(e.call(0x005c_57f0, &args![]).bool());
        assert!(culled.get());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_ALL_TREES, TEXT_NOT_CULLED]]
        );
        assert!(e.call(0x005c_57f0, &args![]).bool());
        assert!(!culled.get());
        assert_eq!(
            calls(&e, CONSOLE_PRINT)[1],
            vec![MSG_ALL_TREES, TEXT_CULLED]
        );
        assert_eq!(calls(&e, SET_TREES_CULLED), vec![vec![1], vec![0]]);
    }

    /// Doubles of the camera field-of-view command.
    fn camera_engine() -> Engine {
        let mut e = engine();
        e.set_global(FOV_LIMIT, 160.0f32);
        e.set_global(FOV_DEFAULT, 75.0f32);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.register(MINIMUM, |_, a| {
            f32::from_bits(a[0]).min(f32::from_bits(a[1])).into_ret()
        });
        e.register(GET_SCENE_GRAPH, |_, _| 0x5c3eu32.into_ret());
        accept(&mut e, &[SET_CAMERA_FOV, SET_SHADER_FOV, SET_WATER_FOV]);
        e
    }

    #[test]
    fn set_camera_fov_limits_and_defaults_the_values() {
        let mut e = camera_engine();
        let player = e.mem.alloc(0x800);
        e.set_global(PLAYER, player);
        // 200 is limited to 160; 0 becomes the default 75.
        parse_gives(&mut e, true, &[200.0f32.to_bits(), 0.0f32.to_bits()]);
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_5840, &args![a]).bool());
        assert_eq!(
            calls(&e, SET_CAMERA_FOV),
            vec![vec![0x5c3e, 160.0f32.to_bits(), 0, 0, 0]]
        );
        assert_eq!(calls(&e, SET_SHADER_FOV), vec![vec![160.0f32.to_bits()]]);
        assert_eq!(e.mem.f32(player + 0x670), 160.0);
        assert_eq!(e.mem.f32(player + 0x674), 75.0);
        assert_eq!(
            calls(&e, SET_WATER_FOV),
            vec![
                vec![WATER_FOV_X, 160.0f32.to_bits()],
                vec![WATER_FOV_Y, 75.0f32.to_bits()]
            ]
        );
    }

    #[test]
    fn set_camera_fov_inverts_negative_values() {
        let mut e = camera_engine();
        let player = e.mem.alloc(0x800);
        e.set_global(PLAYER, player);
        parse_gives(&mut e, true, &[(-2.0f32).to_bits(), (-4.0f32).to_bits()]);
        let a = script(&mut e, 0);
        assert!(e.call(0x005c_5840, &args![a]).bool());
        assert_eq!(e.mem.f32(player + 0x670), 0.5);
        assert_eq!(e.mem.f32(player + 0x674), 0.25);
        // Parameters that do not parse: false and nothing changes.
        e.mem.set_f32(player + 0x670, 1.0);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_5840, &args![a]).bool());
        assert_eq!(e.mem.f32(player + 0x670), 1.0);
    }

    #[test]
    fn player_fov_setters_store_the_floats() {
        let mut e = engine();
        let player = e.mem.alloc(0x800);
        e.call(0x005c_59c0, &args![ptr(player), 1.25f32]);
        e.call(0x005c_59e0, &args![ptr(player), 2.5f32]);
        assert_eq!(e.mem.f32(player + 0x670), 1.25);
        assert_eq!(e.mem.f32(player + 0x674), 2.5);
    }

    #[test]
    fn console_callback_prints_the_text_it_is_given() {
        let mut e = engine();
        e.register(IDENTITY, |_, a| a[0].into_ret());
        start_log(&mut e);
        e.call(0x005c_5a00, &args![0x1234u32]);
        assert_eq!(calls(&e, IDENTITY), vec![vec![0x1234, 0]]);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![0x1234]]);
    }

    #[test]
    fn callback_global_accessors_read_and_write_the_global() {
        let mut e = engine();
        e.call(0x005c_5a60, &args![0xcafeu32]);
        assert_eq!(e.global::<u32>(CALLBACK_GLOBAL), 0xcafe);
        assert_eq!(e.call(0x005c_5a50, &args![]).u32(), 0xcafe);
    }

    #[test]
    fn shader_dump_runs_with_the_console_callback_installed() {
        let mut e = engine();
        e.set_global(CALLBACK_GLOBAL, 0x77u32);
        let seen: std::rc::Rc<std::cell::Cell<u32>> = Default::default();
        let during = seen.clone();
        e.register_double(SHADER_MANAGER_DUMP, move |e, _| {
            during.set(e.global::<u32>(CALLBACK_GLOBAL));
            Ret::default()
        });
        assert!(e.call(0x005c_5a20, &args![]).bool());
        assert_eq!(seen.get(), PRINT_CALLBACK);
        // The previous callback is restored.
        assert_eq!(e.global::<u32>(CALLBACK_GLOBAL), 0x77);
    }

    // ---- Texture palette and model map ------------------------------------------------

    /// Writes the C string `text` at `addr` (a page the engine mapped).
    fn put_str(e: &mut Engine, addr: u32, text: &str) {
        e.mem.set_cstr(addr, text.as_bytes());
    }

    /// Doubles for the string and file functions the dump commands call: the
    /// C library ones behave like the real ones, the formatting ones only
    /// record their arguments (the names stay empty).
    fn dump_engine() -> Engine {
        let mut e = engine();
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRING_COPY, |e, a| {
            let text = e.mem.cstr(a[1]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(SPRINTF_S, |_, _| Ret::default());
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(FIND_SUBSTRING, |e, a| {
            let haystack = e.mem.cstr(a[0]);
            let needle = e.mem.cstr(a[1]);
            let found = !needle.is_empty()
                && haystack
                    .windows(needle.len())
                    .any(|window| window == needle.as_slice());
            (found as u32).into_ret()
        });
        accept(
            &mut e,
            &[
                INC_DISABLE_WARNING_COUNT,
                BS_FILE_CONSTRUCT,
                BS_FILE_OPEN,
                BS_FILE_CLOSE,
                BS_FILE_DESTRUCT,
                BS_FILE_WRITE,
                MODEL_LOADER_OUTPUT_MODEL_MAP,
                FACE_GEN_OUTPUT_MODEL_MAP,
                VECTOR_CONSTRUCT,
                VECTOR_DESTRUCT,
                ZERO_FILL,
                FIX_PATH,
            ],
        );
        e.register(GET_MODEL_CACHE, |_, _| 0u32.into_ret());
        // The reference count of a texture is its dword at +4.
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(FIRST_TEXTURE, |_, _| 0u32.into_ret());
        e.register(GET_INTERIOR_CELL, |_, _| 0u32.into_ret());
        e.register(TES_GET_WORLD_SPACE, |_, _| 0u32.into_ret());
        e.set_global(TES, 0x7e5u32);
        e.set_global(MEGABYTE, 1048576.0f64);
        e
    }

    #[test]
    fn texture_palette_uses_the_name_it_is_given() {
        let mut e = dump_engine();
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"mine.txt");
            true.into_ret()
        });
        let paths: std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>> = Default::default();
        let log = paths.clone();
        e.register_double(BS_FILE_CONSTRUCT, move |e, a| {
            log.borrow_mut().push(e.mem.cstr(a[1]));
            Ret::default()
        });
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_5a70, &args![a]).bool());
        assert_eq!(paths.borrow()[0], b"mine.txt");
        // No name is built.
        assert!(calls(&e, SPRINTF_S).is_empty());
        // The warnings are disabled around the dump and the table of 0x800
        // NiPointers is built and destroyed.
        assert_eq!(calls(&e, INC_DISABLE_WARNING_COUNT), vec![vec![1], vec![0]]);
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![
                vec![MSG_TEXTURE_PALETTE_BEGIN],
                vec![MSG_TOTAL_TEXTURES, 0, 0, 0],
                vec![MSG_TEXTURE_PALETTE_END]
            ]
        );
        assert_eq!(calls(&e, VECTOR_CONSTRUCT)[0][1..4], [4, 0x800, 0x6694e0]);
        assert_eq!(calls(&e, VECTOR_DESTRUCT)[0][1..], [4, 0x800, 0x45cec0]);
        assert_eq!(calls(&e, BS_FILE_CLOSE).len(), 1);
        assert_eq!(calls(&e, BS_FILE_DESTRUCT).len(), 1);
        // Parameters that do not parse: false before anything else happens.
        e.register(PARSE_PARAMETERS, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(!e.call(0x005c_5a70, &args![a]).bool());
        assert!(calls(&e, BS_FILE_CONSTRUCT).is_empty());
    }

    #[test]
    fn texture_palette_without_parameters_names_the_file_after_the_place() {
        // The command ran without parameters (`param_info` 0): no parsing.
        let mut e = dump_engine();
        let mut a = script(&mut e, 0);
        a.param_info = 0;
        // An interior: `TexDump-INT-<name>.txt`.
        let interior = object_with(&mut e, &[(0x130, V_KIND + 9)]);
        e.register_double(GET_INTERIOR_CELL, move |_, _| interior.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005c_5a70, &args![a]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        let naming = &calls(&e, SPRINTF_S)[0];
        assert_eq!(
            naming[1..],
            [0x200, FORMAT_TEX_DUMP_INTERIOR, TEXT_TEX_DUMP, 9]
        );
        // The (recording-only) formatter left the name empty: the default.
        assert_eq!(calls(&e, STRING_COPY)[0][1], DEFAULT_TEX_DUMP_PATH);

        // An exterior: the world space name and the grid coordinates.
        e.register(GET_INTERIOR_CELL, |_, _| 0u32.into_ret());
        let world = object_with(&mut e, &[(0x130, V_KIND + 8)]);
        e.register_double(TES_GET_WORLD_SPACE, move |_, _| world.into_ret());
        e.register(TES_GRID_X, |_, _| 3u32.into_ret());
        e.register(TES_GRID_Y, |_, _| 4u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005c_5a70, &args![a]).bool());
        let naming = &calls(&e, SPRINTF_S)[0];
        assert_eq!(
            naming[1..],
            [0x200, FORMAT_TEX_DUMP_EXTERIOR, TEXT_TEX_DUMP, 8, 3, 4]
        );

        // Neither: `TexDump-UNKNOWN.txt`.
        e.register(TES_GET_WORLD_SPACE, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005c_5a70, &args![a]).bool());
        let naming = &calls(&e, SPRINTF_S)[0];
        assert_eq!(naming[1..], [0x200, FORMAT_TEX_DUMP_UNKNOWN, TEXT_TEX_DUMP]);
    }

    #[test]
    fn texture_palette_lists_the_textures_with_their_category() {
        let mut e = dump_engine();
        for (addr, text) in [
            (PATH_ARMOR, "\\armor\\"),
            (PATH_CREATURES, "\\creatures\\"),
            (PATH_CHARACTERS, "\\characters\\"),
            (PATH_PIPBOY, "\\pipboy3000\\"),
            (PATH_WEAPONS, "\\weapons\\"),
            (PATH_DECALS, "\\decals\\"),
            (PATH_PROJECTILES, "\\projectiles\\"),
            (PATH_FONTS, "\\fonts\\"),
            (PATH_INTERFACE, "\\interface\\"),
            (PATH_EFFECTS, "\\effects\\"),
            (PATH_GORE, "\\gore\\"),
            (PATH_SKY, "\\sky\\"),
            (PATH_LOD, "\\lod\\"),
            (PATH_WATER, "\\water\\"),
        ] {
            put_str(&mut e, addr, text);
        }
        // Three textures: a source texture under \armor\, a source texture
        // without a name, and a rendered one. Layout of the test objects:
        // +0x24 Direct3D texture, +0x2c next texture, +4 reference count,
        // +0x30 "is a source texture", +0x34 name text, +0x38 "L" kind.
        let vtable = e.mem.alloc(0x100);
        let owner_name = e.mem.alloc(8);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, owner_name);
        e.register_double(V_KIND + 1, move |_, _| owner.into_ret());
        e.register(V_KIND + 2, |_, _| 128u32.into_ret());
        e.register(V_KIND + 3, |_, _| 64u32.into_ret());
        e.register(V_KIND + 4, |_, a| a[0].into_ret());
        e.mem.set_u32(vtable + 0x08, V_KIND + 1);
        e.mem.set_u32(vtable + 0x94, V_KIND + 2);
        e.mem.set_u32(vtable + 0x98, V_KIND + 3);
        e.mem.set_u32(vtable + 0x9c, V_KIND + 4);
        let armor_name = e.mem.alloc(0x40);
        put_str(&mut e, armor_name, "textures\\armor\\vault.dds");
        let empty = e.mem.alloc(8);
        let mut textures = vec![];
        for (is_source, name, kind) in [(1, armor_name, 3), (1, 0, 1), (0, empty, 1)] {
            let t = e.mem.alloc(0x60);
            e.mem.set_u32(t, vtable);
            e.mem.set_u32(t + 4, 3);
            e.mem.set_u32(t + 0x24, 0xd3d0 + textures.len() as u32);
            e.mem.set_u32(t + 0x30, is_source);
            e.mem.set_u32(t + 0x34, name);
            e.mem.set_u32(t + 0x38, kind);
            textures.push(t);
        }
        for i in 0..2 {
            e.mem.set_u32(textures[i] + 0x2c, textures[i + 1]);
        }
        let first = textures[0];
        e.register_double(FIRST_TEXTURE, move |_, _| first.into_ret());
        e.register(NEXT_TEXTURE, |e, a| e.mem.u32(a[0] + 0x2c).into_ret());
        e.register(GET_D3D_TEXTURE, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(NI_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(TEXTURE_IS_SOURCE, |e, a| {
            assert_eq!(a[0], TEXTURE_CLASS);
            (e.mem.u32(a[1] + 0x30) != 0).into_ret()
        });
        // A source texture's name comes from slot 0x9c (a handle that is the
        // texture itself here); a nameless one falls back to `+8`'s handle.
        e.register(NAME_TEXT, |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        e.register(FN_00413F40, |_, a| a[0].into_ret());
        e.register(FN_007D6BB0, |_, a| a[0].into_ret());
        e.register(D3D_FORMAT_ID, |_, a| (a[0] + 1).into_ret());
        e.register(GET_D3D_FORMAT_STRING, |_, a| (a[0] + 2).into_ret());
        e.register(TEXTURE_KIND, |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        let mut a = script(&mut e, 0);
        a.param_info = 0;
        start_log(&mut e);
        assert!(e.call(0x005c_5a70, &args![a]).bool());

        // The textures were inserted in order (all sizes are 0).
        let table = calls(&e, NI_POINTER_SET);
        assert_eq!(table.iter().map(|w| w[1]).collect::<Vec<_>>(), textures);
        assert_eq!(table[1][0], table[0][0] + 4);
        // The summary: the number of textures and their size in megabytes.
        let summary = calls(&e, LOG_STUB)
            .into_iter()
            .find(|w| w[0] == MSG_TOTAL_TEXTURES)
            .unwrap();
        assert_eq!(summary[1..], [3, 0, 0]);
        // Descriptions: the armor path and the unnamed one (the first copy is
        // the default file name).
        let describing: Vec<u32> = calls(&e, STRCPY_S).iter().map(|w| w[2]).collect();
        assert_eq!(describing, vec![armor_name, TEXT_UNNAMED_SOURCE_TEXTURE]);
        let rendered = calls(&e, SPRINTF_S)
            .into_iter()
            .filter(|w| w[2] == TEXT_RENDERED_TEXTURE_NONE)
            .count();
        assert_eq!(rendered, 1);
        // One line per texture: index, category, description, width, height,
        // size in KB, format, reference count minus one, "L"/"H", owner name.
        let lines: Vec<Vec<u32>> = calls(&e, SPRINTF_S)
            .into_iter()
            .filter(|w| w[2] == FORMAT_TEXTURE_LINE)
            .collect();
        assert_eq!(lines.len(), 3);
        let description = lines[0][5];
        assert_eq!(
            lines[0][3..],
            [
                1,
                CATEGORY_ACTOR,
                description,
                128,
                64,
                0,
                0xd3d0 + 3,
                2,
                TEXT_L,
                owner_name
            ]
        );
        // No path fragment matches the unnamed texture: MISCREF.
        assert_eq!(lines[1][3..5], [2, CATEGORY_MISCREF]);
        assert_eq!(lines[1][11], TEXT_H);
        // The rendered one is the "SYSTEM" category, whatever its text says.
        assert_eq!(lines[2][3..5], [3, CATEGORY_SYSTEM]);
        assert_eq!(calls(&e, BS_FILE_WRITE).len(), 3);
    }

    #[test]
    fn texture_palette_categories_follow_the_path() {
        // Each fragment, in the game's order, selects its category.
        for (path, category) in [
            ("a\\creatures\\x", CATEGORY_ACTOR),
            ("a\\projectiles\\x", CATEGORY_ACTOR),
            ("a\\interface\\x", CATEGORY_MENUS),
            ("a\\gore\\x", CATEGORY_EFFECTS),
            ("a\\sky\\x", CATEGORY_SKY),
            ("a\\lod\\x", CATEGORY_LOD),
            ("a\\water\\x", CATEGORY_WATER),
            ("a\\other\\x", CATEGORY_MISCREF),
        ] {
            let mut e = dump_engine();
            for (addr, text) in [
                (PATH_ARMOR, "\\armor\\"),
                (PATH_CREATURES, "\\creatures\\"),
                (PATH_CHARACTERS, "\\characters\\"),
                (PATH_PIPBOY, "\\pipboy3000\\"),
                (PATH_WEAPONS, "\\weapons\\"),
                (PATH_DECALS, "\\decals\\"),
                (PATH_PROJECTILES, "\\projectiles\\"),
                (PATH_FONTS, "\\fonts\\"),
                (PATH_INTERFACE, "\\interface\\"),
                (PATH_EFFECTS, "\\effects\\"),
                (PATH_GORE, "\\gore\\"),
                (PATH_SKY, "\\sky\\"),
                (PATH_LOD, "\\lod\\"),
                (PATH_WATER, "\\water\\"),
            ] {
                put_str(&mut e, addr, text);
            }
            let name = e.mem.alloc(0x40);
            put_str(&mut e, name, path);
            let vtable = e.mem.alloc(0x100);
            let owner = e.mem.alloc(8);
            e.register_double(V_KIND + 1, move |_, _| owner.into_ret());
            e.mem.set_u32(vtable + 0x08, V_KIND + 1);
            e.register(V_KIND + 4, |_, a| a[0].into_ret());
            e.mem.set_u32(vtable + 0x9c, V_KIND + 4);
            e.mem.set_u32(vtable + 0x94, V_KIND);
            e.mem.set_u32(vtable + 0x98, V_KIND);
            let texture = e.mem.alloc(0x60);
            e.mem.set_u32(texture, vtable);
            e.mem.set_u32(texture + 0x34, name);
            e.register_double(FIRST_TEXTURE, move |_, _| texture.into_ret());
            e.register(NEXT_TEXTURE, |_, _| 0u32.into_ret());
            e.register(GET_D3D_TEXTURE, |_, _| 1u32.into_ret());
            e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
            e.register(NI_POINTER_SET, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
            e.register(TEXTURE_IS_SOURCE, |_, _| true.into_ret());
            e.register(NAME_TEXT, |e, a| e.mem.u32(a[0] + 0x34).into_ret());
            accept(&mut e, &[FN_00413F40, FN_007D6BB0, D3D_FORMAT_ID]);
            e.register(GET_D3D_FORMAT_STRING, |_, _| 0u32.into_ret());
            e.register(TEXTURE_KIND, |_, _| 0u32.into_ret());
            let mut a = script(&mut e, 0);
            a.param_info = 0;
            start_log(&mut e);
            assert!(e.call(0x005c_5a70, &args![a]).bool());
            let line = calls(&e, SPRINTF_S)
                .into_iter()
                .find(|w| w[2] == FORMAT_TEXTURE_LINE)
                .unwrap();
            assert_eq!(line[4], category, "{path}");
        }
    }

    #[test]
    fn model_map_dump_writes_the_loaders_and_the_face_gen_maps() {
        let mut e = dump_engine();
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"Run1");
            true.into_ret()
        });
        e.set_global(MODEL_LOADER, 0x6000u32);
        e.register(GET_MODEL_CACHE, |_, _| 0x7000u32.into_ret());
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_6430, &args![a]).bool());
        // The path is formatted from the argument.
        let naming = &calls(&e, SPRINTF_S)[0];
        assert_eq!(naming[1..3], [0x104, FORMAT_MODEL_DUMP]);
        let file = calls(&e, BS_FILE_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, BS_FILE_CONSTRUCT)[0][1], naming[0]);
        assert_eq!(calls(&e, BS_FILE_CONSTRUCT)[0][2..], [1, 0x4000, 0]);
        assert_eq!(
            calls(&e, MODEL_LOADER_OUTPUT_MODEL_MAP),
            vec![vec![0x6000, file, 0]]
        );
        assert_eq!(
            calls(&e, FACE_GEN_OUTPUT_MODEL_MAP),
            vec![vec![0x7000, file]]
        );
        assert_eq!(calls(&e, INC_DISABLE_WARNING_COUNT), vec![vec![1], vec![0]]);
        assert_eq!(
            calls(&e, LOG_STUB),
            vec![vec![MSG_MODEL_MAP_BEGIN], vec![MSG_MODEL_MAP_END]]
        );
        assert_eq!(calls(&e, BS_FILE_CLOSE), vec![vec![file]]);
        assert_eq!(calls(&e, BS_FILE_DESTRUCT), vec![vec![file]]);
        // The game asks the face-gen manager twice.
        assert_eq!(calls(&e, GET_MODEL_CACHE).len(), 2);

        // Without a model loader or a model cache only the file is made; the
        // command without parameters does not parse.
        e.set_global(MODEL_LOADER, 0u32);
        e.register(GET_MODEL_CACHE, |_, _| 0u32.into_ret());
        let mut b = a;
        b.param_info = 0;
        start_log(&mut e);
        assert!(e.call(0x005c_6430, &args![b]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        assert!(calls(&e, MODEL_LOADER_OUTPUT_MODEL_MAP).is_empty());
        assert!(calls(&e, FACE_GEN_OUTPUT_MODEL_MAP).is_empty());
        assert_eq!(calls(&e, BS_FILE_CLOSE).len(), 1);

        // Parameters that do not parse: false and no file.
        e.register(PARSE_PARAMETERS, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(!e.call(0x005c_6430, &args![a]).bool());
        assert!(calls(&e, BS_FILE_CONSTRUCT).is_empty());
    }

    // ---- The sex change command -----------------------------------------------------------

    /// Doubles of the sex change command.
    fn sex_engine() -> Engine {
        let mut e = engine();
        accept(
            &mut e,
            &[
                SET_FLAG_BIT,
                RESET_3D,
                ARRAY_CONSTRUCT,
                ARRAY_DESTRUCT,
                FIND_NPCS_OF_RACE_AND_SEX,
                COPY_LOOK_FROM,
                BS_STRING_CONSTRUCT,
                BS_STRING_DESTRUCT,
            ],
        );
        e.register(ACTOR_GET_SEX, |e, a| e.mem.u32(a[0] + 0x150).into_ret());
        e.register(GET_SEX, |_, _| 1u32.into_ret());
        e.register(GET_RACE, |_, _| 0x6aceu32.into_ret());
        e.register(ARRAY_ELEMENT, |e, a| {
            let cell = e.mem.alloc(4);
            e.mem.set_u32(cell, 0x0a9c);
            let _ = a;
            cell.into_ret()
        });
        e.register(GET_NAME_TEXT, |_, _| 0x7777u32.into_ret());
        e.set_global(SEX_NAMES, 0x1000u32);
        e.set_global(SEX_NAMES + 4, 0x2000u32);
        e
    }

    /// An actor (sex stored at `+0x150`), male or female, with a base form
    /// at `+0x20`; virtual slots `0x218` and `0x360` answer true, `0x1d0`
    /// gives a process whose slot `0xc` is recorded.
    fn sex_actor(e: &mut Engine, sex: u32) -> (u32, u32) {
        let process = object_with(e, &[(0xc, V_RECORD)]);
        let actor = object_with(
            e,
            &[
                (0x218, V_TRUE),
                (0x360, V_TRUE),
                (0x1d0, V_NODE),
                (0x100, V_TRUE),
            ],
        );
        let base = e.mem.alloc(0x100);
        e.mem.set_u32(actor + 0x100, process);
        e.mem.set_u32(actor + 0x20, base);
        e.mem.set_u32(actor + 0xc, 0xa7);
        e.mem.set_u32(actor + 0x150, sex);
        (actor, base)
    }

    #[test]
    fn sex_change_toggles_by_default() {
        let mut e = sex_engine();
        let (actor, base) = sex_actor(&mut e, 0);
        parse_gives(&mut e, true, &[0xffff_ffff, 0]);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_65a0, &args![a]).bool());
        // A male (0) becomes female: bit 1 of the base data at +0x30 is set.
        assert_eq!(calls(&e, SET_FLAG_BIT), vec![vec![base + 0x30, 1, 1, 1]]);
        assert_eq!(calls(&e, RESET_3D), vec![vec![actor]]);
        // The process was asked for (and the result dropped).
        assert_eq!(calls(&e, V_RECORD).len(), 1);
        // No look is copied without the flag.
        assert!(calls(&e, FIND_NPCS_OF_RACE_AND_SEX).is_empty());
        // The echo is off.
        assert!(calls(&e, CONSOLE_PRINT).is_empty());

        // A female (1) becomes male.
        e.mem.set_u32(actor + 0x150, 1);
        start_log(&mut e);
        assert!(e.call(0x005c_65a0, &args![a]).bool());
        assert_eq!(calls(&e, SET_FLAG_BIT), vec![vec![base + 0x30, 1, 0, 1]]);
    }

    #[test]
    fn sex_change_to_a_given_sex_does_nothing_when_it_is_already_that_sex() {
        let mut e = sex_engine();
        let (actor, base) = sex_actor(&mut e, 1);
        let a = script(&mut e, actor);
        // Wanted 1 (female) on a female: nothing.
        parse_gives(&mut e, true, &[1, 0]);
        start_log(&mut e);
        assert!(e.call(0x005c_65a0, &args![a]).bool());
        assert!(calls(&e, SET_FLAG_BIT).is_empty());
        assert!(calls(&e, RESET_3D).is_empty());
        // Wanted 0 (male) on a female: the bit is cleared.
        parse_gives(&mut e, true, &[0, 0]);
        start_log(&mut e);
        assert!(e.call(0x005c_65a0, &args![a]).bool());
        assert_eq!(calls(&e, SET_FLAG_BIT), vec![vec![base + 0x30, 1, 0, 1]]);
        // Wanted 1 on a male: set.
        e.mem.set_u32(actor + 0x150, 0);
        parse_gives(&mut e, true, &[1, 0]);
        start_log(&mut e);
        assert!(e.call(0x005c_65a0, &args![a]).bool());
        assert_eq!(calls(&e, SET_FLAG_BIT), vec![vec![base + 0x30, 1, 1, 1]]);
        // A reference that is not an actor (-1 from the sex accessor) and the
        // default wanted value (-1): already "that sex".
        e.mem.set_u32(actor + 0x150, 0xffff_ffff);
        parse_gives(&mut e, true, &[0xffff_ffff, 0]);
        start_log(&mut e);
        assert!(e.call(0x005c_65a0, &args![a]).bool());
        assert!(calls(&e, SET_FLAG_BIT).is_empty());
    }

    #[test]
    fn sex_change_copies_the_look_of_the_first_npc_of_the_new_sex() {
        let mut e = sex_engine();
        let (actor, base) = sex_actor(&mut e, 0);
        parse_gives(&mut e, true, &[1, 1]);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_65a0, &args![a]).bool());
        // The search gets the race, the sex of the base and the array.
        let search = &calls(&e, FIND_NPCS_OF_RACE_AND_SEX)[0];
        assert_eq!(search[..2], [0x6ace, 1]);
        let array = search[2];
        assert_eq!(calls(&e, ARRAY_CONSTRUCT), vec![vec![array]]);
        assert_eq!(calls(&e, ARRAY_ELEMENT), vec![vec![array, 0]]);
        assert_eq!(calls(&e, COPY_LOOK_FROM), vec![vec![base, 0x0a9c]]);
        assert_eq!(calls(&e, ARRAY_DESTRUCT), vec![vec![array]]);
        // Slot 0x360 answering false skips the copy.
        let (actor, _) = sex_actor(&mut e, 0);
        let vtable = e.mem.u32(actor);
        e.mem.set_u32(vtable + 0x360, V_FALSE);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_65a0, &args![a]).bool());
        assert!(calls(&e, FIND_NPCS_OF_RACE_AND_SEX).is_empty());
    }

    #[test]
    fn sex_change_prints_the_new_sex_when_the_console_echo_is_on() {
        let mut e = sex_engine();
        let (actor, _) = sex_actor(&mut e, 0);
        parse_gives(&mut e, true, &[0xffff_ffff, 0]);
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_ECHO, 1);
        let message = e.mem.alloc(0x40);
        e.register_double(BS_STRING_FORMAT, move |e, a| {
            e.mem.set_u32(a[0], message);
            Ret::default()
        });
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_65a0, &args![a]).bool());
        // The format gets the name, the form id and the sex's name (index 1
        // of the table: female).
        let format = &calls(&e, BS_STRING_FORMAT)[0];
        assert_eq!(format[1..], [FORMAT_SEX_CHANGED, 0x7777, 0xa7, 0x2000]);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![message]]);
        assert_eq!(calls(&e, BS_STRING_DESTRUCT), vec![vec![format[0]]]);
    }

    #[test]
    fn sex_change_falls_back_to_the_player_and_does_nothing_without_one() {
        let mut e = sex_engine();
        let (player, base) = sex_actor(&mut e, 0);
        e.set_global(PLAYER, player);
        parse_gives(&mut e, true, &[0xffff_ffff, 0]);
        // A reference that is not an actor (slot 0x218 false): the player.
        let other = object_with(&mut e, &[(0x218, V_FALSE)]);
        let a = script(&mut e, other);
        start_log(&mut e);
        assert!(e.call(0x005c_65a0, &args![a]).bool());
        assert_eq!(calls(&e, SET_FLAG_BIT)[0][0], base + 0x30);
        // No reference at all: the player as well.
        let a = script(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005c_65a0, &args![a]).bool());
        assert_eq!(calls(&e, SET_FLAG_BIT).len(), 1);
        // No player: nothing happens.
        e.set_global(PLAYER, 0u32);
        start_log(&mut e);
        assert!(e.call(0x005c_65a0, &args![a]).bool());
        assert!(calls(&e, SET_FLAG_BIT).is_empty());
        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_65a0, &args![a]).bool());
    }

    // ---- 005c6820 .. 005c7430 ---------------------------------------------------

    /// A command that parses one argument (the double stores `0x55` in the
    /// local) and forwards it to `callee` as `(thisObj, argument, 0,
    /// result)`.
    fn check_one_argument_condition(entry: u32, callee: u32) {
        let mut e = engine();
        parse_gives(&mut e, true, &[0x55]);
        e.register(callee, |_, _| true.into_ret());
        let mut a = script(&mut e, 0x40);
        a.event_list = 0x66;
        start_log(&mut e);
        assert!(e.call(entry, &args![a]).bool());
        assert_eq!(
            calls(&e, PARSE_PARAMETERS)[0][..7],
            [1, 2, 8, 0x40, 0, 5, 0x66]
        );
        assert_eq!(
            calls(&e, callee),
            vec![vec![0x40, 0x55, 0, a.result.addr()]]
        );
        // The callee's `AL` is the result.
        e.register(callee, |_, _| false.into_ret());
        assert!(!e.call(entry, &args![a]).bool());
        // A parse that leaves the local alone: the local starts at 0.
        parse_gives(&mut e, true, &[]);
        e.register(callee, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(entry, &args![a]).bool());
        assert_eq!(calls(&e, callee)[0][1], 0);
        // Parameters that do not parse: false, nothing called.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(entry, &args![a]).bool());
        assert!(calls(&e, callee).is_empty());
    }

    /// A command run with the player as the argument.
    fn check_player_condition(entry: u32, callee: u32) {
        let mut e = engine();
        e.set_global(PLAYER, 0x77u32);
        e.register(callee, |_, _| true.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(e.call(entry, &args![a]).bool());
        assert_eq!(
            calls(&e, callee),
            vec![vec![0x40, 0x77, 0, a.result.addr()]]
        );
        e.register(callee, |_, _| false.into_ret());
        assert!(!e.call(entry, &args![a]).bool());
    }

    #[test]
    fn clothing_value_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_6820, GET_CLOTHING_VALUE_CONDITION, None);
    }

    #[test]
    fn same_faction_command_parses_one_argument() {
        check_one_argument_condition(0x005c_6840, SAME_FACTION_CONDITION);
    }

    #[test]
    fn same_race_command_parses_one_argument() {
        check_one_argument_condition(0x005c_68a0, SAME_RACE_CONDITION);
    }

    #[test]
    fn same_sex_command_parses_one_argument() {
        check_one_argument_condition(0x005c_6900, SAME_SEX_CONDITION);
    }

    #[test]
    fn same_faction_with_the_player_passes_the_player() {
        check_player_condition(0x005c_6960, SAME_FACTION_CONDITION);
    }

    #[test]
    fn same_race_with_the_player_passes_the_player() {
        check_player_condition(0x005c_6990, SAME_RACE_CONDITION);
    }

    #[test]
    fn same_sex_with_the_player_passes_the_player() {
        check_player_condition(0x005c_69c0, SAME_SEX_CONDITION);
    }

    #[test]
    fn detected_command_parses_one_argument() {
        check_one_argument_condition(0x005c_69f0, GET_DETECTED_CONDITION);
    }

    #[test]
    fn dead_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_6a50, GET_DEAD_CONDITION, None);
    }

    #[test]
    fn item_count_command_parses_one_argument() {
        check_one_argument_condition(0x005c_6a70, GET_ITEM_COUNT_CONDITION);
    }

    #[test]
    fn player_byte_command_stores_whether_the_number_is_positive() {
        let mut e = engine();
        let player = e.mem.alloc(0x300);
        e.set_global(PLAYER, player);
        let a = script(&mut e, 0x40);
        for (number, expected) in [
            (5u32, 1u8),
            (1, 1),
            (0, 0),
            (0xffff_ffff, 0),
            (0x8000_0000, 0),
        ] {
            e.mem.set_u8(player + 0x240, 0xaa);
            parse_gives(&mut e, true, &[number]);
            assert!(e.call(0x005c_6ad0, &args![a]).bool());
            assert_eq!(e.mem.u8(player + 0x240), expected, "number {number:#x}");
        }
        // Parameters that do not parse: false, the byte is not touched.
        e.mem.set_u8(player + 0x240, 0xaa);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_6ad0, &args![a]).bool());
        assert_eq!(e.mem.u8(player + 0x240), 0xaa);
    }

    #[test]
    fn player_byte_setter_stores_one_byte() {
        let mut e = engine();
        let object = e.mem.alloc(0x300);
        e.mem.set_u32(object + 0x240, 0x1122_3344);
        e.call(0x005c_6b40, &args![object, 0x1u32]);
        assert_eq!(e.mem.u32(object + 0x240), 0x1122_3301);
        e.call(0x005c_6b40, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object + 0x240), 0x1122_3300);
    }

    /// Slot doubles of the actor-value owner of `fn_005c6b60`: slot `0x0c`
    /// answers 30.25, slot `0x20` answers 100.5; both are asked for `0x10`.
    const V_OWNER_FIRST: u32 = 0x0900_0020;
    const V_OWNER_SECOND: u32 = 0x0900_0021;

    fn restore_engine(actor_answer: u32) -> (Engine, u32) {
        let mut e = engine();
        let actor = object_with(&mut e, &[(0x100, actor_answer)]);
        let owner = actor + 0xa4;
        // The slots are called on the object embedded at +0xa4, with 0x10.
        e.register_double(V_OWNER_FIRST, move |_, a| {
            assert_eq!(a[..2], [owner, 0x10]);
            30.25f32.into_ret()
        });
        e.register_double(V_OWNER_SECOND, move |_, a| {
            assert_eq!(a[..2], [owner, 0x10]);
            100.5f32.into_ret()
        });
        accept(&mut e, &[RESTORE_ACTOR_VALUE]);
        e.set_global(LIMB_RESTORE_AMOUNT, 999.0f32);
        let owner_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(owner_vtable + 0xc, V_OWNER_FIRST);
        e.mem.set_u32(owner_vtable + 0x20, V_OWNER_SECOND);
        e.mem.set_u32(owner, owner_vtable);
        (e, actor)
    }

    #[test]
    fn restore_command_restores_health_and_the_limb_values() {
        let (mut e, actor) = restore_engine(V_TRUE);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_6b60, &args![a]).bool());
        let log = calls(&e, RESTORE_ACTOR_VALUE);
        assert_eq!(log.len(), 8);
        // Health first: the owner's slot 0x20 value minus its slot 0x0c value.
        assert_eq!(log[0], vec![actor, 0x10, (100.5f32 - 30.25).to_bits()]);
        for (call, code) in log[1..]
            .iter()
            .zip([0x1d, 0x1e, 0x1b, 0x1c, 0x19, 0x1a, 0x1f])
        {
            assert_eq!(*call, vec![actor, code, 999.0f32.to_bits()]);
        }
    }

    #[test]
    fn restore_command_does_nothing_for_a_non_actor_or_no_reference() {
        let (mut e, actor) = restore_engine(V_FALSE);
        let a = script(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005c_6b60, &args![a]).bool());
        assert!(calls(&e, RESTORE_ACTOR_VALUE).is_empty());
        let a = script(&mut e, 0);
        assert!(e.call(0x005c_6b60, &args![a]).bool());
        assert!(calls(&e, RESTORE_ACTOR_VALUE).is_empty());
    }

    /// The engine of the NPC-copy command: the reference `0x40` is an actor
    /// whose base form is `0x5000`, the argument `0x60` has the base form
    /// `0x6000`; the casts give the NPCs `0x5100` and `0x6100`.
    fn copy_engine() -> (Engine, ScriptArgs) {
        let mut e = engine();
        let this_obj = object_with(&mut e, &[(0x100, V_TRUE)]);
        let argument = e.mem.alloc(0x40);
        e.mem.set_u32(this_obj + 0x20, 0x5000);
        e.mem.set_u32(argument + 0x20, 0x6000);
        e.register_double(DYNAMIC_CAST, |_, a| {
            assert_eq!(a[1..], [0, RTTI_TES_BOUND_OBJECT, RTTI_TES_NPC, 0]);
            match a[0] {
                0x5000 => 0x5100u32.into_ret(),
                0x6000 => 0x6100u32.into_ret(),
                _ => 0u32.into_ret(),
            }
        });
        e.register(NPC_GET_FIELD_0X130, |_, _| 0xf1e1_d000u32.into_ret());
        e.register(ACTOR_BASE_DATA_GET_LEVEL, |_, _| 0xdead_1234u32.into_ret());
        accept(
            &mut e,
            &[
                NPC_SET_FIELD_0X130,
                ACTOR_BASE_DATA_SET_LEVEL,
                TES_NPC_INIT_VALUES,
            ],
        );
        parse_gives(&mut e, true, &[argument]);
        let a = script(&mut e, this_obj);
        (e, a)
    }

    #[test]
    fn copy_npc_command_copies_the_field_and_the_level_then_initializes() {
        let (mut e, a) = copy_engine();
        start_log(&mut e);
        assert!(e.call(0x005c_6c80, &args![a]).bool());
        assert_eq!(calls(&e, NPC_GET_FIELD_0X130), vec![vec![0x6100]]);
        assert_eq!(
            calls(&e, NPC_SET_FIELD_0X130),
            vec![vec![0x5100, 0xf1e1_d000]]
        );
        assert_eq!(calls(&e, ACTOR_BASE_DATA_GET_LEVEL), vec![vec![0x6130]]);
        // Only the low 16 bits of the level go on.
        assert_eq!(
            calls(&e, ACTOR_BASE_DATA_SET_LEVEL),
            vec![vec![0x5130, 0x1234]]
        );
        assert_eq!(calls(&e, TES_NPC_INIT_VALUES), vec![vec![0x5100, 0]]);
    }

    #[test]
    fn copy_npc_command_needs_both_casts_an_actor_and_an_argument() {
        // The command's reference is not an actor.
        let (mut e, a) = copy_engine();
        let vtable = e.mem.u32(a.this_obj.addr());
        e.mem.set_u32(vtable + 0x100, V_FALSE);
        start_log(&mut e);
        assert!(e.call(0x005c_6c80, &args![a]).bool());
        assert!(calls(&e, GET_BASE_FORM).is_empty());
        // No reference.
        let mut none = a;
        none.this_obj = Ptr::NULL;
        assert!(e.call(0x005c_6c80, &args![none]).bool());
        assert!(calls(&e, GET_BASE_FORM).is_empty());
        // No argument.
        let (mut e, a) = copy_engine();
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005c_6c80, &args![a]).bool());
        assert!(calls(&e, GET_BASE_FORM).is_empty());
        // A base form that is not an NPC (the cast gives 0).
        let (mut e, a) = copy_engine();
        e.mem.set_u32(a.this_obj.addr() + 0x20, 0x7000);
        start_log(&mut e);
        assert!(e.call(0x005c_6c80, &args![a]).bool());
        assert_eq!(calls(&e, DYNAMIC_CAST).len(), 2);
        assert!(calls(&e, NPC_GET_FIELD_0X130).is_empty());
        assert!(calls(&e, TES_NPC_INIT_VALUES).is_empty());
        // Parameters that do not parse.
        let (mut e, a) = copy_engine();
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_6c80, &args![a]).bool());
        assert!(calls(&e, GET_BASE_FORM).is_empty());
    }

    #[test]
    fn equipped_command_parses_one_argument() {
        check_one_argument_condition(0x005c_6d90, GET_EQUIPPED_CONDITION);
    }

    #[test]
    fn gold_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_6df0, GET_GOLD_CONDITION, None);
    }

    #[test]
    fn sleeping_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_6e10, GET_SLEEPING_CONDITION, None);
    }

    #[test]
    fn sitting_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_6e30, GET_SITTING_CONDITION, None);
    }

    #[test]
    fn furniture_marker_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_6e50, GET_FURNITURE_MARKER_ID_CONDITION, None);
    }

    #[test]
    fn current_furniture_ref_command_parses_one_argument() {
        check_one_argument_condition(0x005c_6e70, IS_CURRENT_FURNITURE_REF_CONDITION);
    }

    #[test]
    fn current_furniture_obj_command_parses_one_argument() {
        check_one_argument_condition(0x005c_6ed0, IS_CURRENT_FURNITURE_OBJ_CONDITION);
    }

    #[test]
    fn talked_to_pc_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_6f30, GET_TALKED_TO_PC_CONDITION, None);
    }

    #[test]
    fn talked_to_pc_param_command_passes_no_reference_and_the_argument() {
        let mut e = engine();
        parse_gives(&mut e, true, &[0x55]);
        e.register(GET_TALKED_TO_PC_CONDITION, |_, _| true.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(e.call(0x005c_6f50, &args![a]).bool());
        assert_eq!(
            calls(&e, GET_TALKED_TO_PC_CONDITION),
            vec![vec![0, 0x55, 0, a.result.addr()]]
        );
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_6f50, &args![a]).bool());
        assert!(calls(&e, GET_TALKED_TO_PC_CONDITION).is_empty());
    }

    #[test]
    fn quest_running_command_parses_one_argument() {
        check_one_argument_condition(0x005c_6fb0, GET_QUEST_RUNNING_CONDITION);
    }

    #[test]
    fn quest_completed_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7010, GET_QUEST_COMPLETED_CONDITION);
    }

    #[test]
    fn stage_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7070, GET_STAGE_CONDITION);
    }

    /// A command parsing two arguments (`0x33` and `0x44` in the two locals).
    fn check_two_argument_condition(entry: u32, callee: u32) {
        let mut e = engine();
        parse_gives(&mut e, true, &[0x33, 0x44]);
        e.register(callee, |_, _| true.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(e.call(entry, &args![a]).bool());
        assert_eq!(
            calls(&e, callee),
            vec![vec![0x40, 0x33, 0x44, a.result.addr()]]
        );
        e.register(callee, |_, _| false.into_ret());
        assert!(!e.call(entry, &args![a]).bool());
        // Both locals start at 0.
        parse_gives(&mut e, true, &[]);
        e.register(callee, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(entry, &args![a]).bool());
        assert_eq!(calls(&e, callee)[0][1..3], [0, 0]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(entry, &args![a]).bool());
        assert!(calls(&e, callee).is_empty());
    }

    #[test]
    fn stage_done_command_parses_two_arguments() {
        check_two_argument_condition(0x005c_70d0, GET_STAGE_DONE_CONDITION);
    }

    #[test]
    fn faction_rank_difference_command_parses_two_arguments() {
        check_two_argument_condition(0x005c_72e0, GET_FACTION_RANK_DIFFERENCE_CONDITION);
    }

    #[test]
    fn set_stage_command_sets_the_result_when_the_quest_accepts_the_stage() {
        let mut e = engine();
        parse_gives(&mut e, true, &[0x4000, 0x1234_5678]);
        e.register(QUEST_SET_STAGE, |_, _| true.into_ret());
        let a = script(&mut e, 0x40);
        e.mem.set_f64(a.result.addr(), 9.0);
        start_log(&mut e);
        assert!(e.call(0x005c_7140, &args![a]).bool());
        // Only the low byte of the stage goes on.
        assert_eq!(calls(&e, QUEST_SET_STAGE), vec![vec![0x4000, 0x78]]);
        assert_eq!(result_of(&e, a), 1.0);
        // The quest refuses: the result stays 0.0.
        e.register(QUEST_SET_STAGE, |_, _| false.into_ret());
        assert!(e.call(0x005c_7140, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
        // No quest: nothing called.
        parse_gives(&mut e, true, &[0, 5]);
        e.mem.set_f64(a.result.addr(), 9.0);
        start_log(&mut e);
        assert!(e.call(0x005c_7140, &args![a]).bool());
        assert!(calls(&e, QUEST_SET_STAGE).is_empty());
        assert_eq!(result_of(&e, a), 0.0);
        // Parameters that do not parse: false, the result is still 0.0.
        parse_gives(&mut e, false, &[]);
        e.mem.set_f64(a.result.addr(), 9.0);
        assert!(!e.call(0x005c_7140, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
    }

    /// A command taking a quest and calling `callee(quest, flag)` on it.
    fn check_quest_flag_command(entry: u32, callee: u32, flag: u32) {
        let mut e = engine();
        parse_gives(&mut e, true, &[0x4000]);
        accept(&mut e, &[callee]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(e.call(entry, &args![a]).bool());
        assert_eq!(calls(&e, callee), vec![vec![0x4000, flag]]);
        // No quest: true, nothing called.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(entry, &args![a]).bool());
        assert!(calls(&e, callee).is_empty());
        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(entry, &args![a]).bool());
        assert!(calls(&e, callee).is_empty());
    }

    #[test]
    fn quest_command_sets_flag_one() {
        check_quest_flag_command(0x005c_71c0, QUEST_SET_FLAG_0X1, 1);
    }

    #[test]
    fn quest_command_clears_flag_one() {
        check_quest_flag_command(0x005c_7220, QUEST_SET_FLAG_0X1, 0);
    }

    #[test]
    fn quest_command_sets_flag_two() {
        check_quest_flag_command(0x005c_7280, QUEST_SET_FLAG_0X2, 1);
    }

    #[test]
    fn alarmed_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_7350, GET_ALARMED_CONDITION, None);
    }

    #[test]
    fn pleasant_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_7370, GET_IS_PLEASANT_CONDITION, None);
    }

    #[test]
    fn cloudy_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_7390, GET_IS_CLOUDY_CONDITION, None);
    }

    #[test]
    fn raining_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_73b0, GET_IS_RAINING_CONDITION, None);
    }

    #[test]
    fn snowing_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_73d0, GET_IS_SNOWING_CONDITION, None);
    }

    #[test]
    fn wind_speed_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_73f0, GET_WIND_SPEED_CONDITION, None);
    }

    #[test]
    fn weather_percent_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_7410, GET_WEATHER_PERCENT_CONDITION, None);
    }

    #[test]
    fn current_weather_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7430, GET_IS_CURRENT_WEATHER_CONDITION);
    }

    // ---- 005c7490 .. 005c83f0 ---------------------------------------------------

    /// A command run on the player (`0x77`) with one parsed argument (the
    /// double stores `0x55` in the local).
    fn check_player_one_argument_condition(entry: u32, callee: u32) {
        let mut e = engine();
        e.set_global(PLAYER, 0x77u32);
        parse_gives(&mut e, true, &[0x55]);
        e.register(callee, |_, _| true.into_ret());
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(e.call(entry, &args![a]).bool());
        assert_eq!(
            calls(&e, callee),
            vec![vec![0x77, 0x55, 0, a.result.addr()]]
        );
        e.register(callee, |_, _| false.into_ret());
        assert!(!e.call(entry, &args![a]).bool());
        // A parse that leaves the local alone: the local starts at 0.
        parse_gives(&mut e, true, &[]);
        e.register(callee, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(entry, &args![a]).bool());
        assert_eq!(calls(&e, callee)[0][1], 0);
        // Parameters that do not parse: false, nothing called.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(entry, &args![a]).bool());
        assert!(calls(&e, callee).is_empty());
    }

    #[test]
    fn attacked_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_7490, GET_ATTACKED_CONDITION, None);
    }

    #[test]
    fn is_creature_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_74b0, GET_IS_CREATURE_CONDITION, None);
    }

    #[test]
    fn should_attack_command_parses_one_argument() {
        check_one_argument_condition(0x005c_74d0, GET_SHOULD_ATTACK_CONDITION);
    }

    #[test]
    fn in_cell_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7530, GET_IN_CELL_CONDITION);
    }

    #[test]
    fn in_cell_command_with_two_arguments_passes_both() {
        check_two_argument_condition(0x005c_7590, GET_IN_CELL_CONDITION);
    }

    #[test]
    fn in_worldspace_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7600, GET_IN_WORLDSPACE_CONDITION);
    }

    #[test]
    fn cell_flag_command_sets_the_flag_when_the_number_is_positive() {
        let mut e = engine();
        accept(&mut e, &[CELL_SET_FLAG_0X20]);
        let a = script(&mut e, 0x40);
        for (number, expected) in [
            (5u32, 1u32),
            (1, 1),
            (0, 0),
            (0xffff_ffff, 0),
            (0x8000_0000, 0),
        ] {
            parse_gives(&mut e, true, &[0x3000, number]);
            start_log(&mut e);
            assert!(e.call(0x005c_7660, &args![a]).bool());
            assert_eq!(
                calls(&e, CELL_SET_FLAG_0X20),
                vec![vec![0x3000, expected]],
                "number {number:#x}"
            );
        }
        // No cell: true, nothing called.
        parse_gives(&mut e, true, &[0, 5]);
        start_log(&mut e);
        assert!(e.call(0x005c_7660, &args![a]).bool());
        assert!(calls(&e, CELL_SET_FLAG_0X20).is_empty());
        // Parameters that do not parse: false, nothing called.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_7660, &args![a]).bool());
        assert!(calls(&e, CELL_SET_FLAG_0X20).is_empty());
    }

    #[test]
    fn in_same_cell_command_parses_one_argument() {
        check_one_argument_condition(0x005c_76d0, GET_IN_SAME_CELL_CONDITION);
    }

    #[test]
    fn is_class_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7730, GET_IS_CLASS_CONDITION);
    }

    #[test]
    fn chargen_command_stores_whether_the_number_is_non_zero() {
        let mut e = engine();
        e.set_global(PLAYER, 0x77u32);
        accept(&mut e, &[PLAYER_SET_CHARGEN]);
        let a = script(&mut e, 0x40);
        for (number, expected) in [(0u32, 0u32), (1, 1), (0xffff_ffff, 1), (0x100, 1)] {
            parse_gives(&mut e, true, &[number]);
            start_log(&mut e);
            assert!(e.call(0x005c_7790, &args![a]).bool());
            assert_eq!(
                calls(&e, PLAYER_SET_CHARGEN),
                vec![vec![0x77, expected]],
                "number {number:#x}"
            );
        }
        // Parameters that do not parse: false, nothing called.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_7790, &args![a]).bool());
        assert!(calls(&e, PLAYER_SET_CHARGEN).is_empty());
    }

    #[test]
    fn chargen_getter_reads_the_byte_behind_the_player() {
        let mut e = engine();
        let player = e.mem.alloc(0x800);
        e.mem.set_u32(player + 0x75c, 0xaabb_cc01);
        assert_eq!(e.call(0x005c_7870, &args![player]).u32() & 0xff, 1);
        e.mem.set_u8(player + 0x75c, 0);
        assert_eq!(e.call(0x005c_7870, &args![player]).u32() & 0xff, 0);
    }

    #[test]
    fn chargen_value_command_returns_the_byte_and_prints_it_with_the_echo() {
        let mut e = engine();
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32 as u32).into_ret());
        let player = e.mem.alloc(0x800);
        e.mem.set_u8(player + 0x75c, 1);
        e.set_global(PLAYER, player);
        let a = script(&mut e, 0x40);
        e.mem.set_f64(a.result.addr(), 9.0);
        start_log(&mut e);
        assert!(e.call(0x005c_7800, &args![a]).bool());
        assert_eq!(result_of(&e, a), 1.0);
        // The echo is off: nothing printed.
        assert!(calls(&e, CONSOLE_PRINT).is_empty());

        e.mem.set_u8(player + 0x75c, 0);
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_ECHO, 1);
        start_log(&mut e);
        assert!(e.call(0x005c_7800, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_CHARGEN_VALUE, 0]]
        );

        e.mem.set_u8(player + 0x75c, 1);
        start_log(&mut e);
        assert!(e.call(0x005c_7800, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GET_CHARGEN_VALUE, 1]]
        );

        // No player: the result is 0.0 and nothing is printed.
        e.set_global(PLAYER, 0u32);
        e.mem.set_f64(a.result.addr(), 9.0);
        start_log(&mut e);
        assert!(e.call(0x005c_7800, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn is_race_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7890, GET_IS_RACE_CONDITION);
    }

    #[test]
    fn is_creature_type_command_parses_one_argument() {
        check_one_argument_condition(0x005c_78f0, GET_IS_CREATURE_TYPE_CONDITION);
    }

    #[test]
    fn is_sex_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7950, GET_IS_SEX_CONDITION);
    }

    #[test]
    fn is_playable_race_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_79b0, GET_IS_PLAYABLE_RACE_CONDITION, None);
    }

    #[test]
    fn in_faction_command_parses_one_argument() {
        check_one_argument_condition(0x005c_79d0, GET_IN_FACTION_CONDITION);
    }

    #[test]
    fn is_class_command_on_the_player_passes_the_player() {
        check_player_one_argument_condition(0x005c_7a30, GET_IS_CLASS_CONDITION);
    }

    #[test]
    fn is_race_command_on_the_player_passes_the_player() {
        check_player_one_argument_condition(0x005c_7a90, GET_IS_RACE_CONDITION);
    }

    #[test]
    fn is_sex_command_on_the_player_passes_the_player() {
        check_player_one_argument_condition(0x005c_7af0, GET_IS_SEX_CONDITION);
    }

    #[test]
    fn in_faction_command_on_the_player_passes_the_player() {
        check_player_one_argument_condition(0x005c_7b50, GET_IN_FACTION_CONDITION);
    }

    #[test]
    fn is_id_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7bb0, GET_IS_ID_CONDITION);
    }

    #[test]
    fn is_in_list_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7c10, IS_IN_LIST_CONDITION);
    }

    #[test]
    fn is_child_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_7c70, GET_IS_CHILD_CONDITION, None);
    }

    #[test]
    fn is_used_item_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7ca0, GET_IS_USED_ITEM_CONDITION);
    }

    #[test]
    fn is_used_item_type_command_passes_only_the_byte_the_parser_stored() {
        let mut e = engine();
        parse_gives(&mut e, true, &[0x1122_3344]);
        e.register(GET_IS_USED_ITEM_TYPE_CONDITION, |_, _| true.into_ret());
        let mut a = script(&mut e, 0x40);
        a.event_list = 0x66;
        start_log(&mut e);
        assert!(e.call(0x005c_7d00, &args![a]).bool());
        assert_eq!(
            calls(&e, GET_IS_USED_ITEM_TYPE_CONDITION),
            vec![vec![0x40, 0x44, 0, a.result.addr()]]
        );
        assert_eq!(
            calls(&e, PARSE_PARAMETERS)[0][..7],
            [1, 2, 8, 0x40, 0, 5, 0x66]
        );
        // The callee's `AL` is the result.
        e.register(GET_IS_USED_ITEM_TYPE_CONDITION, |_, _| false.into_ret());
        assert!(!e.call(0x005c_7d00, &args![a]).bool());
        // Parameters that do not parse: false, nothing called.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_7d00, &args![a]).bool());
        assert!(calls(&e, GET_IS_USED_ITEM_TYPE_CONDITION).is_empty());
    }

    #[test]
    fn used_item_level_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_7d60, GET_USED_ITEM_LEVEL_CONDITION, None);
    }

    #[test]
    fn used_item_activate_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_7d80, GET_USED_ITEM_ACTIVATE_CONDITION, None);
    }

    #[test]
    fn is_ref_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7da0, GET_IS_REF_CONDITION);
    }

    #[test]
    fn faction_rank_command_parses_one_argument() {
        check_one_argument_condition(0x005c_7e00, GET_FACTION_RANK_CONDITION);
    }

    /// An engine for the actor commands: `__RTDynamicCast` answers `actor`
    /// (0 for "not an actor"), the other callees just accept the call.
    fn actor_engine(actor: u32) -> Engine {
        let mut e = engine();
        e.register_double(DYNAMIC_CAST, move |_, _| actor.into_ret());
        accept(&mut e, &[REFERENCE_BASE_FORM, ACTOR_SET_FACTION_RANK]);
        accept(&mut e, &[ACTOR_ADD_TO_FACTION_RANK]);
        e
    }

    #[test]
    fn set_faction_rank_command_sets_the_rank_on_an_actor() {
        let mut e = actor_engine(0x7000);
        parse_gives(&mut e, true, &[0x4000, 0x1234_5603]);
        let a = script(&mut e, 0x40);
        e.mem.set_f64(a.result.addr(), 5.0);
        start_log(&mut e);
        assert!(e.call(0x005c_7e60, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x40, 0, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0]]
        );
        assert_eq!(calls(&e, REFERENCE_BASE_FORM), vec![vec![0x7000]]);
        // The rank is the low byte of the second local.
        assert_eq!(
            calls(&e, ACTOR_SET_FACTION_RANK),
            vec![vec![0x7000, 0x4000, 3]]
        );

        // No faction: the base form accessor runs, nothing else.
        parse_gives(&mut e, true, &[0, 3]);
        start_log(&mut e);
        assert!(e.call(0x005c_7e60, &args![a]).bool());
        assert_eq!(calls(&e, REFERENCE_BASE_FORM).len(), 1);
        assert!(calls(&e, ACTOR_SET_FACTION_RANK).is_empty());

        // Parameters that do not parse: false, the result is not touched.
        e.mem.set_f64(a.result.addr(), 5.0);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_7e60, &args![a]).bool());
        assert_eq!(result_of(&e, a), 5.0);
        assert!(calls(&e, DYNAMIC_CAST).is_empty());
    }

    #[test]
    fn set_faction_rank_command_does_nothing_on_a_non_actor() {
        let mut e = actor_engine(0);
        parse_gives(&mut e, true, &[0x4000, 3]);
        let a = script(&mut e, 0x40);
        e.mem.set_f64(a.result.addr(), 5.0);
        start_log(&mut e);
        assert!(e.call(0x005c_7e60, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
        assert!(calls(&e, REFERENCE_BASE_FORM).is_empty());
        assert!(calls(&e, ACTOR_SET_FACTION_RANK).is_empty());
    }

    #[test]
    fn modify_faction_rank_command_tells_whether_the_actor_is_the_player() {
        let mut e = actor_engine(0x7000);
        parse_gives(&mut e, true, &[0x4000, 0x1234_56ff]);
        e.set_global(PLAYER, 0x7000u32);
        let a = script(&mut e, 0x40);
        e.mem.set_f64(a.result.addr(), 5.0);
        start_log(&mut e);
        assert!(e.call(0x005c_7f10, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
        assert_eq!(calls(&e, REFERENCE_BASE_FORM), vec![vec![0x7000]]);
        assert_eq!(
            calls(&e, ACTOR_ADD_TO_FACTION_RANK),
            vec![vec![0x7000, 0x4000, 0xff, 1]]
        );

        // Another actor than the player.
        e.set_global(PLAYER, 0x8000u32);
        start_log(&mut e);
        assert!(e.call(0x005c_7f10, &args![a]).bool());
        assert_eq!(
            calls(&e, ACTOR_ADD_TO_FACTION_RANK),
            vec![vec![0x7000, 0x4000, 0xff, 0]]
        );

        // No faction: nothing changes.
        parse_gives(&mut e, true, &[0, 1]);
        start_log(&mut e);
        assert!(e.call(0x005c_7f10, &args![a]).bool());
        assert!(calls(&e, ACTOR_ADD_TO_FACTION_RANK).is_empty());

        // Parameters that do not parse: false, the result is not touched.
        e.mem.set_f64(a.result.addr(), 5.0);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005c_7f10, &args![a]).bool());
        assert_eq!(result_of(&e, a), 5.0);
    }

    #[test]
    fn modify_faction_rank_command_does_nothing_on_a_non_actor() {
        let mut e = actor_engine(0);
        parse_gives(&mut e, true, &[0x4000, 1]);
        let a = script(&mut e, 0x40);
        start_log(&mut e);
        assert!(e.call(0x005c_7f10, &args![a]).bool());
        assert!(calls(&e, REFERENCE_BASE_FORM).is_empty());
        assert!(calls(&e, ACTOR_ADD_TO_FACTION_RANK).is_empty());
    }

    #[test]
    fn faction_reaction_command_returns_the_reaction_and_prints_it_with_the_echo() {
        let mut e = engine();
        e.register(FACTION_GET_REACTION, |_, _| (-3i32 as u32).into_ret());
        e.register(STRING_OR_EMPTY, |_, a| (a[0] + 0x1000).into_ret());
        let a = script(&mut e, 0x40);
        parse_gives(&mut e, true, &[0x4000, 0x5000]);
        e.mem.set_f64(a.result.addr(), 9.0);
        start_log(&mut e);
        assert!(e.call(0x005c_7fd0, &args![a]).bool());
        assert_eq!(result_of(&e, a), -3.0);
        assert_eq!(calls(&e, FACTION_GET_REACTION), vec![vec![0x4000, 0x5000]]);
        // The echo is off: nothing printed.
        assert!(calls(&e, CONSOLE_PRINT).is_empty());

        // With the echo on the names of the two factions (`+ 0x18`) and
        // the value are printed; the second name is asked for first.
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_ECHO, 1);
        start_log(&mut e);
        assert!(e.call(0x005c_7fd0, &args![a]).bool());
        assert_eq!(calls(&e, STRING_OR_EMPTY), vec![vec![0x5018], vec![0x4018]]);
        let value = (-3.0f64).to_bits();
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![
                MSG_FACTION_REACTION,
                0x5018,
                0x6018,
                value as u32,
                (value >> 32) as u32
            ]]
        );

        // A missing faction: the result stays 0.0 (the echo still prints).
        parse_gives(&mut e, true, &[0x4000, 0]);
        e.mem.set_f64(a.result.addr(), 9.0);
        start_log(&mut e);
        assert!(e.call(0x005c_7fd0, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
        assert!(calls(&e, FACTION_GET_REACTION).is_empty());
        assert_eq!(calls(&e, CONSOLE_PRINT).len(), 1);

        // Parameters that do not parse: false, the result is 0.0.
        e.mem.set_f64(a.result.addr(), 9.0);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005c_7fd0, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    /// A command parsing two factions and a value and calling
    /// `callee(first, second, value)`.
    fn check_faction_value_command(entry: u32, callee: u32) {
        let mut e = engine();
        accept(&mut e, &[callee]);
        parse_gives(&mut e, true, &[0x4000, 0x5000, 7]);
        let a = script(&mut e, 0x40);
        e.mem.set_f64(a.result.addr(), 5.0);
        start_log(&mut e);
        assert!(e.call(entry, &args![a]).bool());
        assert_eq!(result_of(&e, a), 0.0);
        assert_eq!(calls(&e, callee), vec![vec![0x4000, 0x5000, 7]]);
        // Either faction missing: nothing is called.
        for outs in [[0, 0x5000, 7], [0x4000, 0, 7]] {
            parse_gives(&mut e, true, &outs);
            start_log(&mut e);
            assert!(e.call(entry, &args![a]).bool());
            assert!(calls(&e, callee).is_empty());
        }
        // Parameters that do not parse: false, the result is not touched.
        e.mem.set_f64(a.result.addr(), 5.0);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(entry, &args![a]).bool());
        assert_eq!(result_of(&e, a), 5.0);
    }

    #[test]
    fn set_reaction_command_passes_both_factions_and_the_value() {
        check_faction_value_command(0x005c_80a0, FACTION_SET_REACTION);
    }

    #[test]
    fn modify_reaction_command_passes_both_factions_and_the_value() {
        check_faction_value_command(0x005c_82c0, FACTION_MODIFY_REACTION);
    }

    /// A command parsing two factions and two flags; `if_clear` and `if_set`
    /// are the codes for a clear and a set flag.
    fn check_relation_codes(entry: u32, if_clear: u32, if_set: u32) {
        let mut e = engine();
        accept(&mut e, &[FACTION_SET_RELATION_CODE]);
        let a = script(&mut e, 0x40);
        for (first_flag, second_flag) in [(0u32, 0u32), (0, 9), (9, 0), (9, 9)] {
            parse_gives(&mut e, true, &[0x4000, 0x5000, first_flag, second_flag]);
            e.mem.set_f64(a.result.addr(), 5.0);
            start_log(&mut e);
            assert!(e.call(entry, &args![a]).bool());
            assert_eq!(result_of(&e, a), 0.0);
            let code = |flag: u32| if flag == 0 { if_clear } else { if_set };
            assert_eq!(
                calls(&e, FACTION_SET_RELATION_CODE),
                vec![
                    vec![0x4000, 0x5000, code(first_flag)],
                    vec![0x5000, 0x4000, code(second_flag)]
                ]
            );
        }
        // Either faction missing: nothing is called.
        for outs in [[0, 0x5000, 1, 1], [0x4000, 0, 1, 1]] {
            parse_gives(&mut e, true, &outs);
            start_log(&mut e);
            assert!(e.call(entry, &args![a]).bool());
            assert!(calls(&e, FACTION_SET_RELATION_CODE).is_empty());
        }
        // Parameters that do not parse: false, the result is not touched.
        e.mem.set_f64(a.result.addr(), 5.0);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(entry, &args![a]).bool());
        assert_eq!(result_of(&e, a), 5.0);
    }

    #[test]
    fn relation_command_uses_codes_two_and_three() {
        check_relation_codes(0x005c_8120, 2, 3);
    }

    #[test]
    fn relation_command_uses_codes_one_and_zero() {
        check_relation_codes(0x005c_81f0, 1, 0);
    }

    /// The reference of `005c8340`: its virtual slot `0x100` answers
    /// `answer`, slot `0x48` records the call. The embedded extra data list
    /// is `reference + 0x44`; the extra data `0x5e` object is `0xe000`, found
    /// at once when `extra_present` and otherwise only after
    /// [`ADD_EXTRA_DATA_0X5E`] ran.
    fn extra_data_engine(answer: u32, extra_present: bool) -> (Engine, u32) {
        let mut e = engine();
        let reference = object_with(&mut e, &[(0x100, answer), (0x48, V_RECORD)]);
        e.register(REFERENCE_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(REFERENCE_BASE_FORM, |_, _| 0x9000u32.into_ret());
        accept(&mut e, &[EXTRA_DATA_0X5E_UPDATE]);
        let present = std::rc::Rc::new(std::cell::Cell::new(extra_present));
        let for_add = present.clone();
        e.register_double(ADD_EXTRA_DATA_0X5E, move |_, _| {
            for_add.set(true);
            Ret::default()
        });
        e.register_double(GET_EXTRA_DATA_0X5E, move |_, _| {
            (if present.get() { 0xe000u32 } else { 0 }).into_ret()
        });
        (e, reference)
    }

    #[test]
    fn update_extra_data_command_uses_the_existing_extra_data() {
        let (mut e, reference) = extra_data_engine(V_TRUE, true);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(e.call(0x005c_8340, &args![a]).bool());
        assert_eq!(calls(&e, V_RECORD), vec![vec![reference, 0x8000_0000]]);
        assert!(calls(&e, ADD_EXTRA_DATA_0X5E).is_empty());
        assert_eq!(calls(&e, GET_EXTRA_DATA_0X5E), vec![vec![reference + 0x44]]);
        assert_eq!(
            calls(&e, EXTRA_DATA_0X5E_UPDATE),
            vec![vec![0xe000, 0x9000]]
        );
    }

    #[test]
    fn update_extra_data_command_adds_the_extra_data_when_missing() {
        let (mut e, reference) = extra_data_engine(V_TRUE, false);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(e.call(0x005c_8340, &args![a]).bool());
        assert_eq!(calls(&e, ADD_EXTRA_DATA_0X5E), vec![vec![reference + 0x44]]);
        assert_eq!(calls(&e, GET_EXTRA_DATA_0X5E).len(), 2);
        assert_eq!(
            calls(&e, EXTRA_DATA_0X5E_UPDATE),
            vec![vec![0xe000, 0x9000]]
        );
    }

    #[test]
    fn update_extra_data_command_needs_a_reference_that_answers_true() {
        let (mut e, reference) = extra_data_engine(V_FALSE, true);
        let a = script(&mut e, reference);
        start_log(&mut e);
        assert!(e.call(0x005c_8340, &args![a]).bool());
        assert!(calls(&e, V_RECORD).is_empty());
        assert!(calls(&e, EXTRA_DATA_0X5E_UPDATE).is_empty());
        // No reference at all.
        let a = script(&mut e, 0);
        assert!(e.call(0x005c_8340, &args![a]).bool());
        assert!(calls(&e, EXTRA_DATA_0X5E_UPDATE).is_empty());
    }

    #[test]
    fn xp_for_next_level_condition_command_passes_zero() {
        check_condition_wrapper(0x005c_83d0, GET_XP_FOR_NEXT_LEVEL_CONDITION, None);
    }

    #[test]
    fn global_value_command_parses_one_argument() {
        check_one_argument_condition(0x005c_83f0, GET_GLOBAL_VALUE_CONDITION);
    }

    #[test]
    fn every_function_of_the_part_is_registered_once() {
        let mut addresses: Vec<u32> = funcs().iter().map(|f| f.0).collect();
        assert_eq!(addresses.len(), 120);
        addresses.sort_unstable();
        addresses.dedup();
        assert_eq!(addresses.len(), 120);
        assert!(addresses
            .iter()
            .all(|a| (0x005c_4240..0x005c_8450).contains(a)));
    }
}

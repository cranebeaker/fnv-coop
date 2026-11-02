//! `fallout shared/tesscriptfunctions.cpp` (Xbox PDB source unit), part 6: its functions from `005d21e0` up to
//! (not including) `005d7d30` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesscriptfunctions`]; anything public there may be used here.
//!
//! Progress: the whole range (`005d21e0` up to `005d7d30`) is translated: 120
//! queue entries, the last 40 being `005d5420` to `005d7c20`.
//!
//! The bodies follow the conventions of the main file: `cdecl`, the eight
//! stack words as [`ScriptArgs`], `AL` as the result. The members of the
//! forms, the water shader property and the actors are read at the PC
//! offsets (the PC build differs from the Xbox PDB's layout), with a comment,
//! instead of through a `layout!`. The compiler's exception-unwinding frames
//! (`005d22d0`) and the stack cookie checks (`005d2a80`, `005d5780`, `005d5850`,
//! `005d67f0`) are not translated.

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
/// `__RTDynamicCast` (`object, 0, source type, target type, 0`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor of `TESObjectREFR` (`.?AVTESObjectREFR@@`).
const RTTI_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// RTTI type descriptor of `Actor` (`.?AVActor@@`).
const RTTI_ACTOR: u32 = 0x0118_46d4;
/// RTTI type descriptor of `TESForm` (`.?AVTESForm@@`).
const RTTI_TES_FORM: u32 = 0x0118_3028;
/// RTTI type descriptor of `TESImageSpaceModifier`.
const RTTI_TES_IMAGE_SPACE_MODIFIER: u32 = 0x0118_62b8;
/// RTTI type descriptor of `TESObject` (`.?AVTESObject@@`).
const RTTI_TES_OBJECT: u32 = 0x0118_3128;
/// RTTI type descriptor of `TESValueForm` (`.?AVTESValueForm@@`).
const RTTI_TES_VALUE_FORM: u32 = 0x0118_6b6c;

/// `thiscall` on a reference: the name of the reference (the full name of
/// its base form).
const GET_REFERENCE_NAME: u32 = 0x0055_d520;
/// `thiscall` on a reference: its base form, `*(this + 0x20)`.
const GET_BASE_FORM_OF_REFERENCE: u32 = 0x0041_81e0;
/// `*(this + 0x20)`: the base form of a reference (the map calls it
/// `BGSSaveFormBuffer::GetForm`).
const GET_BASE_FORM: u32 = 0x007a_f430;
/// `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB): `*(this + 0x68)`,
/// the actor's process.
const GET_PROCESS: u32 = 0x008d_8520;
/// `Actor::UpdateAlpha` (Xbox PDB), `thiscall`.
const ACTOR_UPDATE_ALPHA: u32 = 0x008c_4640;
/// `thiscall` on a component of a form (`form + 0x18`): the length of the
/// string at `this + 4` (`004048e0`: the cached length, or the string's own).
const MODEL_PATH_LENGTH: u32 = 0x0048_cee0;
/// `ProcessLists` method (`00974b80`, `thiscall` on the process lists
/// singleton, `reference, model path`): the call `StopMagicEffect` makes to
/// remove the visual effect from the reference (it walks a list of the
/// process lists).
const PROCESS_LISTS_REMOVE_VISUAL_EFFECT: u32 = 0x0097_4b80;
/// `thiscall` on a stack slot (`value`): stores the value in the slot and,
/// when it is not null, calls `0040f6e0` on it (`00633c90`).
const MODEL_KEY_CONSTRUCT: u32 = 0x0063_3c90;
/// `thiscall` on that slot (`0040c110`): when the slot holds a non-null value,
/// calls `0040c130` on it.
const MODEL_KEY_DESTRUCT: u32 = 0x0040_c110;
/// `ModelLoader::FindModel` (Xbox PDB), `thiscall` on the model loader
/// (`path, key slot`): `AL`.
const MODEL_LOADER_FIND_MODEL: u32 = 0x0044_72a0;
/// `ModelLoader::QueueModel` (Xbox PDB), `thiscall` on the model loader
/// (`path, 5, 0, 0, 1, 0, 0`).
const MODEL_LOADER_QUEUE_MODEL: u32 = 0x0044_4040;
/// Method of the `tesobjectcell.cpp` unit (`00546b10`, `thiscall`, `-1, 0`).
const CELL_UPDATE: u32 = 0x0054_6b10;

/// `float` minimum (`cdecl`, `a, b`), result in `ST0`: `b` when `b <= a` (or
/// the two are unordered), otherwise `a`.
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// `float` maximum (`cdecl`, `a, b`), result in `ST0`: `a` when `b < a`,
/// otherwise `b`.
const FLOAT_MAX: u32 = 0x0040_4010;
/// `cdecl`, no arguments, result in `ST0`: the float at the address
/// `00403e20` gives for the global object `011c36d8`.
const SETTING_FLOAT_A: u32 = 0x0040_df50;
/// `cdecl`, no arguments, result in `ST0`: the float at the address
/// `00403e20` gives for the global object `011c3664`.
const SETTING_FLOAT_B: u32 = 0x0040_df70;
/// `cdecl` (`p, q, r, s, v`), result in `ST0`:
/// `((v - r) / (s - r)) * (q - p) + p`.
const LERP: u32 = 0x004b_3ab0;
/// `_ftol2` (`00ec62c0`): the `ST0` argument (a leading `f64`) truncated to
/// an integer.
const FTOL2: u32 = 0x00ec_62c0;
/// `__stricmp` through `00404dc0` (`cdecl`, `a, b`): 0 when the strings are
/// equal ignoring case.
const STRICMP: u32 = 0x0040_4dc0;
/// `sprintf_s` (`cdecl`, `buffer, size, format, ...`).
const SPRINTF_S: u32 = 0x0040_6d00;
/// `Error` (Xbox PDB, `cdecl`, `message, 0`).
const ERROR_REPORT: u32 = 0x0040_fbe0;
/// Import slot of `QueryPerformanceFrequency`.
const QUERY_PERFORMANCE_FREQUENCY: u32 = 0x00fd_f0a4;
/// Import slot of `QueryPerformanceCounter`.
const QUERY_PERFORMANCE_COUNTER: u32 = 0x00fd_f0a0;

/// `Script::IsInInteriorConditionFunction` (Xbox PDB), `cdecl` (`thisObj,
/// argument, 0, result`); the other condition functions below take the same
/// four words.
const IS_IN_INTERIOR_CONDITION: u32 = 0x005a_3270;
/// `Script::GetPCMiscStatConditionFunction` (Xbox PDB).
const GET_PC_MISC_STAT_CONDITION: u32 = 0x005a_3310;
/// `Script::IsActorEvilConditionFunction` (Xbox PDB).
const IS_ACTOR_EVIL_CONDITION: u32 = 0x005a_3390;
/// `Script::IsActorVictimConditionFunction` (Xbox PDB).
const IS_ACTOR_VICTIM_CONDITION: u32 = 0x005a_3460;
/// Condition function `005a3570`.
const CONDITION_005A3570: u32 = 0x005a_3570;
/// Condition function `005a35f0` (`GetNoRumors`).
const GET_NO_RUMORS_CONDITION: u32 = 0x005a_35f0;
/// Condition function `005a3650` (`GetWhichService`).
const GET_WHICH_SERVICE_CONDITION: u32 = 0x005a_3650;
/// Condition function `005a3670` (`IsActorRidingHorse`).
const IS_ACTOR_RIDING_HORSE_CONDITION: u32 = 0x005a_3670;
/// Condition function `005a2a50` (the player's last ridden horse).
const IS_PLAYERS_LAST_RIDDEN_HORSE_CONDITION: u32 = 0x005a_2a50;
/// `Script::IsInDangerousWaterConditionFunction` (Xbox PDB).
const IS_IN_DANGEROUS_WATER_CONDITION: u32 = 0x005a_36b0;
/// Condition function `005a3760` (`GetIgnoreFriendlyHits`).
const GET_IGNORE_FRIENDLY_HITS_CONDITION: u32 = 0x005a_3760;

/// `MiscStatManager::ModVal` (Xbox PDB), `cdecl` (`stat id, amount`).
const MISC_STAT_MOD_VAL: u32 = 0x004d_5e10;
/// `ImageSpaceModifierInstanceForm::Trigger` (Xbox PDB), `cdecl` (`modifier,
/// strength, 0`).
const IMAGE_SPACE_MODIFIER_TRIGGER: u32 = 0x0052_99a0;
/// `cdecl` (`form type index`): the default form of that index.
const GET_DEFAULT_FORM: u32 = 0x0048_39c0;
/// `thiscall` on an actor base data block (`flag mask`): whether any bit of
/// the mask is set in the flags word `this + 4`.
const FLAGS_TEST: u32 = 0x0046_1580;
/// `ExtraDataList` method (`004216b0`, `thiscall`, no arguments).
const EXTRA_DATA_LIST_CLEAR: u32 = 0x0042_16b0;
/// `ExtraDataList` method (`00421600`, `thiscall`, `flag`).
const EXTRA_DATA_LIST_SET: u32 = 0x0042_1600;
/// `ExtraDataList::RemoveSavedAnimation` (Xbox PDB), `thiscall`.
const EXTRA_DATA_LIST_REMOVE_SAVED_ANIMATION: u32 = 0x0042_2aa0;
/// `ExtraDataList::RemoveSavedHavokData` (Xbox PDB), `thiscall`.
const EXTRA_DATA_LIST_REMOVE_SAVED_HAVOK_DATA: u32 = 0x0042_2c20;
/// `thiscall` on a reference, no arguments (`0055f970`): the first call
/// `Reset3DState` makes.
const REFERENCE_UNLOAD_3D: u32 = 0x0055_f970;
/// `BSAwardsSystemUtility::QInstance` (Xbox PDB), `cdecl`: the singleton.
const AWARDS_QUERY_INSTANCE: u32 = 0x00af_22d0;
/// `BSAwardsSystemUtility::Unlock` (Xbox PDB), `thiscall` (`achievement id`).
const AWARDS_UNLOCK: u32 = 0x00af_2420;
/// `thiscall` on a game setting (`011c3ea4`): the address of its integer
/// value.
const GET_SETTING_INTEGER: u32 = 0x0043_d4d0;
/// `cdecl`, no arguments: the global at `011df1a8`.
const GET_QUEUE_OWNER: u32 = 0x0045_37b0;
/// `cdecl` (`owner, 0x1156, reference, double`; `0087a8b0`).
const SEND_REFERENCE_EVENT: u32 = 0x0087_a8b0;
/// `cdecl` (`object, block, callback`), `00c68ec0`: when `00c43490(object)`
/// gives something whose word at `+0xc` has bit 2 set, passes the three
/// words on to `00c68900` and returns 1 in `AL`.
const FOR_EACH_ENTITY: u32 = 0x00c6_8ec0;
/// `fastcall` on an entity (`006fa820`): `*this`, the key the lookup
/// `00653270` takes.
const ENTITY_KEY: u32 = 0x006f_a820;
/// `cdecl` (`table, key`; `00653270`): 0 for a null key, otherwise `table`
/// when `006532c0(key)` is true and 0 if not.
const LOOKUP_BY_KEY: u32 = 0x0065_3270;
/// `fastcall` (`004ae750`): 0 for a null `this`, otherwise `00620b80(this)`
/// (the map names it `bhkCharacterProxy::operatorP`, which the body does not
/// confirm).
const GET_PROXY: u32 = 0x004a_e750;
/// `thiscall` on what `004ae750` returns (`float`; `00ca88c0`): calls slot
/// `0xc` of `*(this + 0xe0)` with the `float`.
const PROXY_SET_VALUE: u32 = 0x00ca_88c0;
/// `thiscall` (`00ca86c0`): `*(this + 0x1f8)` when the byte at `+0xe8` is 4
/// or 5, else 0.
const PROXY_GET_OBJECT: u32 = 0x00ca_86c0;
/// `thiscall` on a form (`flag`; `00484990`): sets (non-zero) or clears the
/// bit `0x100000` of the flags word at `this + 8`.
const FORM_SET_FLAG: u32 = 0x0048_4990;
/// `TESValueForm` method (`0048e960`, `thiscall`, `value`): sets the value.
const VALUE_FORM_SET_VALUE: u32 = 0x0048_e960;
/// `cdecl` (`flag`; `00730690`, `bartermenu.cpp` range): with 0 sets the byte
/// at `011d8fc0`; otherwise refreshes through `0072dc30(0)` when the word at
/// `011d8fa4` is set and clears the byte.
const REFRESH_MENU: u32 = 0x0073_0690;
/// `Script::SetProcessScripts` (Xbox PDB), `cdecl` (`flag`).
const SET_PROCESS_SCRIPTS: u32 = 0x005a_c730;
/// `Script::GetProcessScripts` (Xbox PDB), `cdecl`: the flag.
const GET_PROCESS_SCRIPTS: u32 = 0x005a_c740;
/// `fastcall` on the data handler (`00455600`): `this + 0x118`, the list the
/// quest commands walk.
const DATA_HANDLER_QUEST_LIST: u32 = 0x0045_5600;
/// `fastcall` on a list node (`008256d0`): whether the node is empty (its
/// item and its next pointer are both null).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `fastcall` on a list node (`006815c0`): the address of the item slot, the
/// node itself.
const NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
/// `fastcall` on a list node (`00726070`): the next node, `*(this + 4)`.
const NODE_NEXT: u32 = 0x0072_6070;
/// `thiscall` on a quest (`flag`; `0060c9c0`): sets (non-zero) or clears bit 0
/// of a byte of the quest (found through `005a8080`) and calls its virtual
/// slot `0x48` with 2.
const QUEST_SET_ENABLED: u32 = 0x0060_c9c0;
/// `thiscall` (`0060f4a0`, `object, flag`), called on each element of the list
/// at `quest + 0x44` with the quest and 1.
const QUEST_STAGE_SET_DONE: u32 = 0x0060_f4a0;
/// `thiscall` (`005ec5d0`, `state`), called on each element of the list at
/// `quest + 0x4c` with 1 and then 3 (it stores the state at `this + 0x20`).
const QUEST_TARGET_SET_STATE: u32 = 0x005e_c5d0;
/// `TES::GetCurrentCell` (Xbox PDB), `thiscall` on the TES singleton.
const TES_GET_CURRENT_CELL: u32 = 0x0045_7070;
/// `TES::RunCellTest` (Xbox PDB), `thiscall` on the TES singleton (one
/// word).
const TES_RUN_CELL_TEST: u32 = 0x0045_9ae0;
/// `thiscall` on a cell (`00451cb0`): its name.
const CELL_GET_NAME: u32 = 0x0045_1cb0;
/// `thiscall` on the TES singleton (`0070ec90`): `*(this + 0x64)`, the water
/// system.
const TES_GET_WATER_SYSTEM: u32 = 0x0070_ec90;
/// `thiscall` on the water system (`004e8030`, `reference`): finds an object of
/// the reference's scene graph (through its slot `0x1d0`) or returns null.
const WATER_SYSTEM_GET_OBJECT: u32 = 0x004e_8030;
/// `NiAVObject::GetProperty` (Xbox PDB), `thiscall` (`property type`).
const NI_AV_OBJECT_GET_PROPERTY: u32 = 0x00a5_9d30;
/// `thiscall` on what slot `0x1d0` of a reference returns (`0043b4a0`, `0`):
/// gives the object `GetProperty(3)` is asked for.
const GET_SHAPE: u32 = 0x0043_b4a0;
/// `TESWaterSystem::EnableWaterSystem` (Xbox PDB), `thiscall`.
const WATER_SYSTEM_ENABLE: u32 = 0x004e_65d0;
/// `thiscall` on the water system (`004e6620`, `1, 0`); the call the "off" branch
/// of `ToggleWaterSystem` makes.
const WATER_SYSTEM_DISABLE: u32 = 0x004e_6620;
/// `thiscall` on the water system (`004e6370`, `1, 1, 1`); the call the `lod`
/// setting makes.
const WATER_SYSTEM_TOGGLE_LOD: u32 = 0x004e_6370;
/// `BGSAutoWater::GeneratePlaceableWaterForCell` (Xbox PDB), `cdecl`
/// (`cell`).
const GENERATE_PLACEABLE_WATER_FOR_CELL: u32 = 0x0049_c860;
/// `thiscall` (`004de2d0`, `value`): sets the value byte of a global object.
const GLOBAL_VALUE_SET: u32 = 0x004d_e2d0;
/// `thiscall` (`this`): `this ? this + 4 : address of a static zero byte`, the
/// address of the value byte of a global object.
const GLOBAL_VALUE_ADDRESS: u32 = 0x0040_8d60;
/// `thiscall` on a four-float colour (`red, green, blue, alpha`): fills the
/// colour and returns it.
const COLOUR_CONSTRUCT: u32 = 0x0041_4430;

// ---- Globals and constants ---------------------------------------------------

/// The `PlayerCharacter` singleton pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// The TES singleton pointer (`this` of [`TES_GET_CURRENT_CELL`]).
const TES_SINGLETON: u32 = 0x011d_ea10;
/// The process lists singleton (the `this` of
/// [`PROCESS_LISTS_REMOVE_VISUAL_EFFECT`]).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The model loader pointer.
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The data handler pointer (the quest list is at `+0x118`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The game setting [`GET_SETTING_INTEGER`] is called on.
const EVENT_SETTING: u32 = 0x011c_3ea4;
/// `float` `-1.0`.
const FLOAT_MINUS_ONE: u32 = 0x0101_2054;
/// `double` `0.0`.
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// `double` `1.0`.
const DOUBLE_ONE: u32 = 0x0101_2070;
/// `double` `100.0`.
const DOUBLE_HUNDRED: u32 = 0x0101_7a40;
/// `double` `20.0`.
const DOUBLE_TWENTY: u32 = 0x0102_fc70;
/// `double` `255.0`.
const DOUBLE_255: u32 = 0x0101_e568;
/// `double` `1e-6`.
const DOUBLE_MICRO: u32 = 0x0101_e3d0;
/// `float` the refraction power is limited to (`10.0`).
const REFRACTION_LIMIT: u32 = 0x0101_7b78;
/// `float` the refraction power of the player is set to in the first branch
/// of `005d3f10` (`0.05`).
const PLAYER_REFRACTION: u32 = 0x0107_a214;
/// Byte at `+0x268` of the TLS block: commands echo to the console when set.
const TLS_ECHO: u32 = 0x268;
/// Byte: the water system flag `005d2a40` reads.
const WATER_SYSTEM_FLAG: u32 = 0x0118_9624;
/// Byte set by `005d2a50` and by `ModifyWaterShader` to say that the global
/// water settings changed.
const WATER_SETTINGS_CHANGED: u32 = 0x011c_7a65;
/// Byte set by `005d2a50`.
const WATER_FLAG_011C7A64: u32 = 0x011c_7a64;
/// Byte: set when the awards system is disabled.
const AWARDS_DISABLED: u32 = 0x011d_8ce4;
/// Byte: the image space modifier getter returns null when it is clear.
const IMAGE_SPACE_ENABLED: u32 = 0x0118_abb1;
/// The cached modifier `005d2860` finds.
const CACHED_GET_HIT_MODIFIER: u32 = 0x011c_9700;
/// Index of the default form `005d2860` looks up.
const GET_HIT_DEFAULT_FORM_INDEX: u32 = 0x162;
/// The table [`LOOKUP_BY_KEY`] searches.
const ENTITY_TABLE: u32 = 0x0126_81c0;
/// Callback `005d3c20` hands to [`FOR_EACH_ENTITY`].
const ENTITY_CALLBACK: u32 = 0x005d_3c70;
/// Event code `005d3b90` sends.
const EVENT_CODE_1156: u32 = 0x1156;

/// Virtual slot `0x14` of a model component (`form + 0x18`): the model path.
const MODEL_PATH_SLOT: u32 = 0x14;
/// Virtual slot `0x100` of a reference: whether it is an actor (`AL`).
const REFR_IS_ACTOR_SLOT: u32 = 0x100;
/// Virtual slot `0x48` (`flags`): marks the object changed.
const MARK_CHANGED_SLOT: u32 = 0x48;
/// Virtual slot `0x1d0` of a reference, no arguments: the object whose
/// shader properties the commands change.
const REFR_GET_TARGET_SLOT: u32 = 0x1d0;
/// Virtual slot `0x5b0` of the actor's process (`float`): sets the alpha.
const PROCESS_SET_ALPHA_SLOT: u32 = 0x5b0;
/// Virtual slot `0x5b8` of the actor's process (`float`): sets the
/// refraction.
const PROCESS_SET_REFRACTION_SLOT: u32 = 0x5b8;
/// Virtual slot `0x384` of an `Actor` (`enable, power`).
const ACTOR_SET_REFRACTION_SLOT: u32 = 0x384;
/// Virtual slot `0x41c` of an `Actor` (`float`).
const ACTOR_SLOT_41C: u32 = 0x41c;
/// Virtual slot `0xc` of a character proxy's object (`float`).
const PROXY_OBJECT_SLOT_C: u32 = 0xc;

// ---- String literals (addresses in the exe's data) ---------------------------

/// `"Visual effect has been removed from reference"`
const MSG_EFFECT_REMOVED_UNNAMED: u32 = 0x0103_be28;
/// `"Visual effect has been removed from %s"`
const MSG_EFFECT_REMOVED: u32 = 0x0103_be58;
/// `"GetNoRumors >> %0.2f"`
const MSG_GET_NO_RUMORS: u32 = 0x0103_be80;
/// `"GetWhichSerivce >> %0.2f"`
const MSG_GET_WHICH_SERVICE: u32 = 0x0103_be98;
/// `"Is actor riding horse >> %0.2f"`
const MSG_IS_ACTOR_RIDING_HORSE: u32 = 0x0103_beb4;
/// `"Actor players last ridden horse >> %0.2f"`
const MSG_PLAYERS_LAST_RIDDEN_HORSE: u32 = 0x0103_bed4;
/// `"Actor is in lava >> %0.2f"`
const MSG_IN_LAVA: u32 = 0x0103_bf00;
/// `"Water System On"`
const MSG_WATER_SYSTEM_ON: u32 = 0x0103_bf1c;
/// `"Water System Off"`
const MSG_WATER_SYSTEM_OFF: u32 = 0x0103_bf2c;
/// `"GeneratePlaceableWaterForCell() for cell %s: Ticks: %I64d, Microseconds: %f"`
const MSG_GENERATE_WATER_TIMING: u32 = 0x0103_bf40;
/// `"autowater"`
const NAME_AUTOWATER: u32 = 0x0103_bf8c;
/// `"Toggle LOD Water"`
const MSG_TOGGLE_LOD_WATER: u32 = 0x0103_bf98;
/// `"lod"`
const NAME_LOD: u32 = 0x0103_bfac;
/// `"set fog amount to %d"`
const MSG_FOG_AMOUNT: u32 = 0x0103_bfb0;
/// `"fog"`
const NAME_FOG: u32 = 0x0103_bfc8;
/// `"Toggle Global Water Depth : ON"`
const MSG_GLOBAL_DEPTH_ON: u32 = 0x0103_bfcc;
/// `"Toggle Global Water Depth : OFF"`
const MSG_GLOBAL_DEPTH_OFF: u32 = 0x0103_bfec;
/// `"Toggle Depth : ON"`
const MSG_DEPTH_ON: u32 = 0x0103_c00c;
/// `"Toggle Depth : OFF"`
const MSG_DEPTH_OFF: u32 = 0x0103_c020;
/// `"depth"`
const NAME_DEPTH: u32 = 0x0103_c034;
/// `"Toggle Global Water Refractions : ON"`
const MSG_GLOBAL_REFRACTIONS_ON: u32 = 0x0103_c03c;
/// `"Toggle Global Water Refractions : OFF"`
const MSG_GLOBAL_REFRACTIONS_OFF: u32 = 0x0103_c064;
/// `"Toggle Refractions : ON"`
const MSG_REFRACTIONS_ON: u32 = 0x0103_c08c;
/// `"Toggle Refractions : OFF"`
const MSG_REFRACTIONS_OFF: u32 = 0x0103_c0a4;
/// `"refract"`
const NAME_REFRACT: u32 = 0x0103_c0c0;
/// `"Toggle Global Water Reflections : ON"`
const MSG_GLOBAL_REFLECTIONS_ON: u32 = 0x0103_c0c8;
/// `"Toggle Global Water Reflections : OFF"`
const MSG_GLOBAL_REFLECTIONS_OFF: u32 = 0x0103_c0f0;
/// `"Toggle Reflections : ON"`
const MSG_REFLECTIONS_ON: u32 = 0x0103_c118;
/// `"Toggle Reflections : OFF"`
const MSG_REFLECTIONS_OFF: u32 = 0x0103_c130;
/// `"reflect"`
const NAME_REFLECT: u32 = 0x0103_c14c;
/// `"rainsize"`
const NAME_RAIN_SIZE: u32 = 0x0103_c154;
/// `"rainfalloff"`
const NAME_RAIN_FALLOFF: u32 = 0x0103_c160;
/// `"rainvelocity"`
const NAME_RAIN_VELOCITY: u32 = 0x0103_c16c;
/// `"rainforce"`
const NAME_RAIN_FORCE: u32 = 0x0103_c17c;
/// `"displacedampener"`
const NAME_DISPLACE_DAMPENER: u32 = 0x0103_c188;
/// `"displacefalloff"`
const NAME_DISPLACE_FALLOFF: u32 = 0x0103_c19c;
/// `"displacevelocity"`
const NAME_DISPLACE_VELOCITY: u32 = 0x0103_c1ac;
/// `"displaceforce"`
const NAME_DISPLACE_FORCE: u32 = 0x0103_c1c0;
/// `"usage : mws noise (0.0 - 100.0)"`
const MSG_USAGE_NOISE: u32 = 0x0103_c1d0;
/// `"set noise to %f"`
const MSG_NOISE: u32 = 0x0103_c1f0;
/// `"set noise scale to %f"`
const MSG_NOISE_SCALE: u32 = 0x0103_c200;
/// `"noise"`
const NAME_NOISE: u32 = 0x0103_c218;
/// `"blend"`
const NAME_BLEND: u32 = 0x0103_c220;
/// `"usage : mws opacity (0.0 - 100.0)"`
const MSG_USAGE_OPACITY: u32 = 0x0103_c228;
/// `"set water opacity to %d"`
const MSG_OPACITY: u32 = 0x0103_c24c;
/// `"opacity"`
const NAME_OPACITY: u32 = 0x0103_c264;
/// `"usage : mws fresnel (0.0 - 1.0)"`
const MSG_USAGE_FRESNEL: u32 = 0x0103_c26c;
/// `"set fresnel term to %f"`
const MSG_FRESNEL: u32 = 0x0103_c28c;
/// `"fresnel"`
const NAME_FRESNEL: u32 = 0x0103_c2a4;
/// `"set water distortion amount to %f"`
const MSG_DISTORTION: u32 = 0x0103_c2ac;
/// `"distort"`
const NAME_DISTORT: u32 = 0x0103_c2d0;
/// `"usage : mws reflectamt (0.0 - 1.0)"`
const MSG_USAGE_REFLECT_AMOUNT: u32 = 0x0103_c2d8;
/// `"set water reflectivity amount to %f"`
const MSG_REFLECT_AMOUNT: u32 = 0x0103_c2fc;
/// `"reflectamt"`
const NAME_REFLECT_AMOUNT: u32 = 0x0103_c320;
/// `"frequency"`
const NAME_FREQUENCY: u32 = 0x0103_c32c;
/// `"amplitude"`
const NAME_AMPLITUDE: u32 = 0x0103_c338;
/// `"direction"`
const NAME_DIRECTION: u32 = 0x0103_c350;
/// `"velocity"`
const NAME_VELOCITY: u32 = 0x0103_c35c;
/// `"reflectamt, fresnel, opacity, speed, noise, reflect, refract, lod"`
const MSG_MWS_USAGE: u32 = 0x0103_c368;
/// `"off"`
const NAME_OFF: u32 = 0x0103_94c8;
/// `"%s"`
const FORMAT_STRING: u32 = 0x0101_9f08;
/// `"Actor counts friendly hits"`
const TEXT_COUNTS_FRIENDLY_HITS: u32 = 0x0103_c3ac;
/// `"Actor ignores friendly hits"`
const TEXT_IGNORES_FRIENDLY_HITS: u32 = 0x0103_c3c8;
/// `"%s has been set to a VALUE of %i"`
const MSG_SET_ITEM_VALUE: u32 = 0x0103_c3e4;
/// `"%s refraction has been set to %f"`
const MSG_REFRACTION_SET: u32 = 0x0103_9b40;
/// `"All Quests Enabled."`
const MSG_ALL_QUESTS_ENABLED: u32 = 0x0103_c408;
/// `"All Quest Stages Completed."`
const MSG_ALL_STAGES_COMPLETED: u32 = 0x0103_c41c;

// ---- The functions from `005d43c0` on: callees, globals and strings ---------

/// `ProcessLists::FlushNonPersistentActors` (Xbox PDB), `thiscall` on
/// [`PROCESS_LISTS`] (`RET 4`: the parsed value).
const PROCESS_LISTS_FLUSH_NON_PERSISTENT_ACTORS: u32 = 0x0097_6030;
/// `cdecl`, no arguments (`007043d0`): `AL`, whether the fog of war is on.
const FOG_OF_WAR_IS_ENABLED: u32 = 0x0070_43d0;
/// `cdecl` (`flag`; `00704420`): switches the fog of war.
const FOG_OF_WAR_SET: u32 = 0x0070_4420;
/// `cdecl` (`1`; `0079ffb0`), a function of the map menu unit
/// (`mapmenu.cpp` in the engine map).
const MAP_MENU_FUNCTION_0079FFB0: u32 = 0x0079_ffb0;
/// `cdecl` (`float`; `004dc2d0`, `renderer.cpp` in the engine map): when the
/// value is positive and differs from the float at `0118945c`, stores it
/// there and in `011ad83c` and sets the byte `011c6fb8`.
const RENDERER_SET_VALUE: u32 = 0x004d_c2d0;
/// `ImageSpaceEffectHDR::ClearAdaptedLight` (Xbox PDB), `cdecl`, no arguments.
const IMAGE_SPACE_HDR_CLEAR_ADAPTED_LIGHT: u32 = 0x00ba_d4a0;
/// Method of the `actor.cpp` range (`008c0ec0`, `thiscall`, `RET 0xC`:
/// `actor, form, -1`); its `this` is the parsed value.
const ACTOR_UNIT_METHOD_008C0EC0: u32 = 0x008c_0ec0;
/// `thiscall` on a form (`00401170`): the byte at `this + 4`, the form type.
const FORM_TYPE_BYTE: u32 = 0x0040_1170;
/// The form type byte `005d4cc0` looks for.
const FORM_TYPE_CHECKED: u32 = 0x16;
/// `thiscall` on a form of that type (`004ff0e0`, `bgstalkingactivator.cpp`
/// in the engine map, `RET 4`: the parsed value).
const FORM_TYPE_CHECKED_METHOD: u32 = 0x004f_f0e0;
/// `thiscall` on a name component of a form (`this = form + 0x18` or
/// `form + 0x48`; the engine map names it `MapMarkerData::GetLocationName`):
/// the text of the name.
const GET_NAME_TEXT: u32 = 0x0040_8da0;
/// `Script::IsEssentialConditionFunction` (Xbox PDB), `cdecl` (`thisObj, 0, 0,
/// result`).
const IS_ESSENTIAL_CONDITION: u32 = 0x005a_3980;
/// Condition function `005a39d0` (`IsActor`), same four words.
const IS_ACTOR_CONDITION: u32 = 0x005a_39d0;
/// Condition function `005a3a00` (`IsPlayerMovingIntoNewSpace`).
const IS_PLAYER_MOVING_INTO_NEW_SPACE_CONDITION: u32 = 0x005a_3a00;
/// Condition function `005a3a30` (`GetTimeDead`).
const GET_TIME_DEAD_CONDITION: u32 = 0x005a_3a30;
/// Condition function `005a3af0`.
const CONDITION_005A3AF0: u32 = 0x005a_3af0;
/// `Script::IsPlayerActionActiveConditionFunction` (Xbox PDB).
const IS_PLAYER_ACTION_ACTIVE_CONDITION: u32 = 0x005a_3b20;
/// Condition function `005a3b90`.
const CONDITION_005A3B90: u32 = 0x005a_3b90;
/// Condition function `005a3bf0`.
const CONDITION_005A3BF0: u32 = 0x005a_3bf0;
/// `Script::HasPerkConditionFunction` (Xbox PDB), `cdecl` (`thisObj, perk,
/// rank, result`).
const HAS_PERK_CONDITION: u32 = 0x005a_4630;
/// Method of the `actor.cpp` range (`008b8e20`, `thiscall`, `RET 8`: the
/// parsed value and the second parsed value).
const ACTOR_UNIT_METHOD_008B8E20: u32 = 0x008b_8e20;
/// `cdecl` (`reference`; `00704690`, `interface.cpp` in the engine map).
const INTERFACE_FUNCTION_00704690: u32 = 0x0070_4690;
/// `PlayerCharacter::AddNote` (Xbox PDB), `thiscall` on the player
/// (`RET 8`: `note, echo flag`).
const PLAYER_ADD_NOTE: u32 = 0x0096_6a70;
/// `thiscall` on the player (`00966b80`, `RET 4`: `note`).
const PLAYER_REMOVE_NOTE: u32 = 0x0096_6b80;
/// `PlayerCharacter::RewardKarma` (Xbox PDB), `thiscall` on the player
/// (`RET 4`: the amount).
const PLAYER_REWARD_KARMA: u32 = 0x0094_fd30;
/// `cdecl`, no arguments (`005ba8d0`): the debug text table, created on first
/// use (the unit's main file translates it).
const DEBUG_TEXT_TABLE_GETTER: u32 = 0x005b_a8d0;
/// `thiscall` on the debug text table (`00802a00`, `RET 8`: `page name
/// buffer, reference`): the PC build's body does nothing but return 0.
const DEBUG_TEXT_TABLE_SEND_PAGE: u32 = 0x0080_2a00;
/// `memset` (`cdecl`: `destination, value, size`).
const MEMSET: u32 = 0x00ec_61c0;

/// Pointer to the object whose field `+0x128` `005d45c0` clears and for
/// which `005d45e0` calls [`MAP_MENU_FUNCTION_0079FFB0`] (null until
/// created).
const MAP_MENU: u32 = 0x011d_a368;
/// Byte: set by `005d4580` while it runs.
const MAP_MENU_BUSY_FLAG: u32 = 0x0120_04fc;
/// `int`: the maximum anisotropy level `005d4660` sets.
const MAX_ANISOTROPY: u32 = 0x011a_9608;
/// Byte: `005d47c0` clears the adapted light when it is set.
const HDR_ACTIVE_FLAG: u32 = 0x011f_941e;
/// Bit mask of the shadow categories `005d4870` toggles (bit `n` for the
/// category `n`).
const SHADOW_CATEGORY_MASK: u32 = 0x011a_d82c;
/// Byte: `005d49c0` stores whether its parsed value is non-zero.
const SWITCH_FLAG_005D49C0: u32 = 0x0126_82f8;
/// Byte: `005d4a30` stores its argument.
const SAVED_FLAG_005D4A30: u32 = 0x011d_ea2a;

/// Virtual slot `0x10` of a reference (`1`): the call `DeleteReference`
/// makes on its reference.
const REFR_DELETE_SLOT: u32 = 0x10;
/// Virtual slot `0x144` of a reference (`float, 1`).
const REFR_SLOT_144: u32 = 0x144;
/// Virtual slot `0x4a0` of an actor (`perk, flag`): `AL`, the rank of the
/// perk.
const ACTOR_GET_PERK_RANK_SLOT: u32 = 0x4a0;
/// Virtual slot `0x498` of an actor (`perk, rank, flag`): sets the rank of
/// the perk.
const ACTOR_SET_PERK_RANK_SLOT: u32 = 0x498;
/// Virtual slot `0x49c` of an actor (`perk, flag`).
const ACTOR_PERK_SLOT_49C: u32 = 0x49c;
/// Virtual slot `0x488` of the player (`value`).
const PLAYER_SLOT_488: u32 = 0x488;
/// The event code `005d4b90` sends when the setting is above 1.
const EVENT_CODE_116C: u32 = 0x116c;
/// Offset of the byte `005d4ee0` returns (a perk's highest rank).
const PERK_MAX_RANK_OFFSET: u32 = 0x3a;

/// `"Deleted all non persistent actors in high process."`
const MSG_FLUSHED_ACTORS: u32 = 0x0103_c438;
/// `"No reference to flush"`
const MSG_NO_REFERENCE_TO_FLUSH: u32 = 0x0103_c46c;
/// `"Deleting reference %s"`
const MSG_DELETING_REFERENCE: u32 = 0x0103_c484;
/// `"Fog of war - %s."`
const MSG_FOG_OF_WAR: u32 = 0x0103_c49c;
/// `"ENABLED"`
const TEXT_ENABLED: u32 = 0x0103_c4b0;
/// `"DISABLED"`
const TEXT_DISABLED: u32 = 0x0103_c4b8;
/// `"Maxaniso set to %d"`
const MSG_MAX_ANISO: u32 = 0x0103_c4c4;
/// `"Shadows %s: %s"`
const MSG_SHADOWS: u32 = 0x0103_c4d8;
/// `"Shadow object type must be [0,6]"`
const MSG_SHADOW_TYPE_RANGE: u32 = 0x0103_c518;
/// The names of the shadow categories 0 to 6 (`"Undetermined"`,
/// `"Architecture"`, `"Furniture"`, `"Actors"`, `"Items"`, `"Misc"`,
/// `"Other"`).
const SHADOW_CATEGORY_NAMES: [u32; 7] = [
    0x0103_c508,
    0x0103_c4f8,
    0x0102_94a4,
    0x0101_ec1c,
    0x0102_95c4,
    0x0103_c4f0,
    0x0103_c4e8,
];
/// `"Off"`
const TEXT_OFF: u32 = 0x0103_99b8;
/// `"On"`
const TEXT_ON: u32 = 0x0103_99bc;
/// `"IsEssential >> %0.2f"`
const MSG_IS_ESSENTIAL: u32 = 0x0103_c53c;
/// `"IsActor >> %0.2f"`
const MSG_IS_ACTOR: u32 = 0x0103_c554;
/// `"Player is moving to new area >> %0.2f"`
const MSG_PLAYER_MOVING_TO_NEW_AREA: u32 = 0x0103_c568;
/// `"time dead >> %0.2f"`
const MSG_TIME_DEAD: u32 = 0x0103_c590;
/// `"Added Perk %s to %s with rank %i"`
const MSG_ADDED_PERK: u32 = 0x0103_c5a4;
/// `"You must supply a debug page name if debug text is not showing"`
const MSG_DEBUG_PAGE_NAME_REQUIRED: u32 = 0x0103_c5c8;
/// `"Removed Note %s from Player "`
const MSG_REMOVED_NOTE: u32 = 0x0103_c608;

// ---- The functions from 005d5420 on: callees, globals and strings ---------

/// Method of the `actor.cpp` range (`008b8f20`, `thiscall`, `RET 4`: the
/// parsed value).
const ACTOR_UNIT_METHOD_008B8F20: u32 = 0x008b_8f20;
/// `BSTimer::SetGlobalTimeMultiplier` (Xbox PDB), `thiscall` on [`TIMER`]
/// (`RET 8`: the multiplier and a flag).
const TIMER_SET_GLOBAL_TIME_MULTIPLIER: u32 = 0x00aa_4db0;
/// The timer object [`TIMER_SET_GLOBAL_TIME_MULTIPLIER`] is called on.
const TIMER: u32 = 0x011f_6394;
/// `BSAudioManager::QInstance` (Xbox PDB), `cdecl`, no arguments: the audio
/// manager singleton.
const AUDIO_MANAGER_QUERY_INSTANCE: u32 = 0x00ad_9060;
/// Condition function `005a3c30` (`GetHitLocation`; the engine map gives it
/// no name): `thisObj, 0, 0, result`.
const GET_HIT_LOCATION_CONDITION: u32 = 0x005a_3c30;
/// Condition function `005a3c90` (`GetLastHitCritical`).
const GET_LAST_HIT_CRITICAL_CONDITION: u32 = 0x005a_3c90;
/// Condition function `005a3d00` (`IsPC1stPerson`).
const IS_PC_1ST_PERSON_CONDITION: u32 = 0x005a_3d00;
/// Virtual slot `0x428` of an actor, no arguments: what the combat
/// commands use as the actor's combat controller (`0097fb80` is in
/// `combatcontroller.cpp`, and `ForceCombatGroupStrategy` reports "is not in
/// combat" when the slot gives null).
const ACTOR_GET_COMBAT_CONTROLLER_SLOT: u32 = 0x428;
/// `thiscall` on the combat controller (`0097fb80`, `combatcontroller.cpp`
/// in the engine map), no arguments.
const COMBAT_CONTROLLER_FUNCTION_0097FB80: u32 = 0x0097_fb80;
/// `cdecl` (`name buffer, value`; `0097abd0`, `combataction.cpp` in the engine
/// map): `AL`, whether the combat action named by the text exists (and was
/// given the cost).
const COMBAT_ACTION_SET_COST: u32 = 0x0097_abd0;
/// `strcpy` (`cdecl`, `destination, source`; `004046f0`).
const STRING_COPY: u32 = 0x0040_46f0;
/// `_strlen` (`cdecl`).
const STRING_LENGTH: u32 = 0x00ec_6130;
/// The empty string of the exe's data (a `"\0"`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `thiscall` on a global float object (`00403e20`): the address of its value
/// (`this + 4`, or a static zero for a null `this`).
const GLOBAL_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// `thiscall` on a global float object (`004ed780`, `teswater.cpp` in the
/// engine map; `RET 4`): sets its float.
const GLOBAL_FLOAT_SET: u32 = 0x004e_d780;
/// `thiscall` (`00474cb0`): 0 when it is entered again; otherwise the length of
/// the text that the object's virtual slot `0x130` (its name) gives.
const NAME_TEXT_LENGTH: u32 = 0x0047_4cb0;
/// Virtual slot `0x130` of a reference or form, no arguments: its name.
const NAME_SLOT: u32 = 0x130;
/// `TESFullName::GetFullName` (Xbox PDB), `cdecl` (`form`): the text of the
/// form's name (the empty string when the form has none).
const GET_FULL_NAME: u32 = 0x0048_2720;
/// `fastcall` on the combat controller (`004fb070`): `this + 0x80`, the combat
/// group.
const COMBAT_GROUP_OF_CONTROLLER: u32 = 0x004f_b070;
/// `CPropertySection::GetCount` as the engine map folds it (`00990890`,
/// `thiscall` on the combat group): its member count.
const COMBAT_GROUP_GET_COUNT: u32 = 0x0099_0890;
/// `thiscall` on the combat group (`009862e0`, `combatgroup.cpp`; `RET 4`):
/// sets the strategy (null clears it).
const COMBAT_GROUP_SET_STRATEGY: u32 = 0x0098_62e0;
/// `CombatGroupStrategy::GetCombatGroupStrategyByName` (Xbox PDB), `cdecl`
/// (`name`): the strategy or null.
const COMBAT_GROUP_STRATEGY_BY_NAME: u32 = 0x0099_0250;
/// `fastcall` on a strategy (`0098fdd0`): its name (an entry of the table that
/// starts with `COMBAT_GROUP_STRATEGY_FALLBACK`).
const COMBAT_GROUP_STRATEGY_NAME: u32 = 0x0098_fdd0;
/// The game's log (`005b5e40`, `cdecl`, printf-like): in this build it does
/// nothing and returns 0.
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// `CombatFormulas::GetKnockbackSpeed` (Xbox PDB), `cdecl` (`a pointer inside
/// the actor, integer`), result in `ST0`.
const GET_KNOCKBACK_SPEED: u32 = 0x0064_6580;
/// `fastcall` on an actor's process (`0045cd60`): `this + 0x28`.
const PROCESS_FIELD_28: u32 = 0x0045_cd60;
/// Virtual slot `0x148` of an actor's process, no arguments: a list whose
/// word at `+8` ([`FIELD_AT_8`]) is its count.
const PROCESS_GET_LIST_SLOT: u32 = 0x148;
/// `fastcall` (`0044ddc0`): the word at `this + 8` (the count of a list, the
/// form of an item entry).
const FIELD_AT_8: u32 = 0x0044_ddc0;
/// `cdecl` (`3, 1, form, count, 0xffff, 2, 0`, ADD ESP,0x1c; `005f5950`, 1520 bytes, no
/// name in the engine map).
const FUNCTION_005F5950: u32 = 0x005f_5950;
/// Virtual slot `0x1f4` of a reference (no stack word): gives the address of three
/// floats (a vector).
const REFR_GET_VECTOR_SLOT: u32 = 0x1f4;
/// Virtual slot `0x418` of an actor's process (`reference, x, y, z, speed`; RET 0x14).
const PROCESS_SLOT_418: u32 = 0x418;
/// `ExtraDataList::RemoveOwnership` (Xbox PDB), `thiscall`.
const EXTRA_DATA_LIST_REMOVE_OWNERSHIP: u32 = 0x0041_aed0;
/// Virtual slot `0x344` of an actor (`form, 0`): an integer value of the form.
const ACTOR_GET_VALUE_SLOT: u32 = 0x344;
/// Virtual slot `0x460` of an actor (`form, float`): changes that value.
const ACTOR_MOD_VALUE_SLOT: u32 = 0x460;
/// Virtual slot `0x138` of a quest, no arguments: its name.
const QUEST_NAME_SLOT: u32 = 0x138;
/// The byte of an actor that `005d6a60` sets from its parameter and echoes as
/// "processing" (PC build offset).
const ACTOR_PROCESSING_FLAG: u32 = 0xbc;

/// `thiscall` on a reference (`00575590`, `RET 8`: `0, 1`): the number of
/// inventory entries (an `i32`, -1 and below 1 when there is no container).
const GET_INVENTORY_COUNT: u32 = 0x0057_5590;
/// `thiscall` on a reference (`005754a0`, `RET 8`: `index, 0`): the inventory
/// entry or null.
const GET_INVENTORY_ENTRY: u32 = 0x0057_54a0;
/// `fastcall` (`0084e3a0`): the word at `this + 0xc` (the form id).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// `thiscall` on an actor (`008bc9d0`, `RET 8`: `form, 1`): `AL`, whether the
/// actor can equip the form.
const ACTOR_CAN_EQUIP: u32 = 0x008b_c9d0;
/// `ItemChange::GetWorn` (Xbox PDB), `thiscall` (`RET 4`: `0`): `AL`.
const ITEM_CHANGE_GET_WORN: u32 = 0x004b_ddd0;
/// `ItemChange::GetItemHealth` (Xbox PDB), `thiscall` (`RET 4`: `percent`),
/// result in `ST0`.
const ITEM_CHANGE_GET_ITEM_HEALTH: u32 = 0x004b_cdb0;
/// `BGSBipedModelList::GetFormAsBipedModelList` (Xbox PDB), `cdecl` (`form`).
const GET_FORM_AS_BIPED_MODEL_LIST: u32 = 0x0047_5020;
/// `fastcall` (`00500940`): `this + 0x18`.
const FIELD_ADDRESS_18: u32 = 0x0050_0940;
/// `thiscall` on an inventory entry (`004459e0`, `RET 4`: `1`): releases it.
const INVENTORY_ENTRY_RELEASE: u32 = 0x0044_59e0;
/// The form type byte `ShowInventory` prints with the can-equip text (the
/// form type names are not confirmed by the exe).
const FORM_TYPE_WITH_EQUIP_TEXT: u32 = 0x28;
/// The form type byte `ShowInventory` prints with the health but without the
/// can-equip text.
const FORM_TYPE_WITH_HEALTH: u32 = 0x18;
/// `cdecl` (`007062a0`, `interface.cpp` in the engine map; `mode, value,
/// flag, flag`): opens the menu the decompiler calls `CharGenMenu::Create`.
const OPEN_CHARGEN_MENU: u32 = 0x0070_62a0;
/// `cdecl` (`0, size, flag`; `00705830`, `interface.cpp`).
const INTERFACE_FUNCTION_00705830: u32 = 0x0070_5830;
/// `Script::GetKillerConditionFunction` (Xbox PDB), `cdecl` (`thisObj,
/// argument, 0, result`).
const GET_KILLER_CONDITION: u32 = 0x005a_3f00;
/// Condition function `005a3fa0` (`GetKillerObject`).
const GET_KILLER_OBJECT_CONDITION: u32 = 0x005a_3fa0;
/// Condition function `005a4090`, two arguments.
const CONDITION_005A4090: u32 = 0x005a_4090;
/// Condition function `005a40d0` (`IsMoving`).
const IS_MOVING_CONDITION: u32 = 0x005a_40d0;
/// Condition function `005a4160` (`IsTurning`).
const IS_TURNING_CONDITION: u32 = 0x005a_4160;
/// Condition function `005a41c0` (`GetAnimAction`).
const GET_ANIM_ACTION_CONDITION: u32 = 0x005a_41c0;
/// Condition function `005a4210`, one argument.
const CONDITION_005A4210: u32 = 0x005a_4210;
/// Condition function `005a4240`.
const CONDITION_005A4240: u32 = 0x005a_4240;
/// Condition function `005a42b0`.
const CONDITION_005A42B0: u32 = 0x005a_42b0;
/// Bit mask of the decal debug displays `ToggleDecalDebug` switches (bit `n`
/// for the option `n`).
const DECAL_DEBUG_FLAGS: u32 = 0x011c_57f4;
/// Byte set with bit `0x20` of [`DECAL_DEBUG_FLAGS`] (the wireframe decals).
const DECAL_WIREFRAME_BYTE: u32 = 0x011f_9442;
/// Pointer to the object whose decal rendering flag `ToggleDecalRendering`
/// switches.
const DECAL_OWNER: u32 = 0x011d_ea0c;
/// `thiscall` on [`DECAL_OWNER`] (`004e2190`): `this + 0x88`, passed on to
/// `00559450`; the answer is the object the next two functions use.
const DECAL_OWNER_OBJECT: u32 = 0x004e_2190;
/// `thiscall` on that object (`0086f840`): `AL`, the byte at `+0x16c`.
const DECAL_RENDERING_GET: u32 = 0x0086_f840;
/// `thiscall` on that object (`005db4c0`, `RET 4`: `flag`): stores the byte
/// at `+0x16c`.
const DECAL_RENDERING_SET: u32 = 0x005d_b4c0;
/// Byte stored by `005d7930`.
const SWITCH_FLAG_0118ABB0: u32 = 0x0118_abb0;
/// Virtual slot `0x434` of an actor (`0`).
const ACTOR_SLOT_434: u32 = 0x434;
/// `thiscall` on an actor (`008bbf40`, 756 bytes, `actor.cpp`): nine
/// arguments (see `005d7a30`).
const ACTOR_UNIT_METHOD_008BBF40: u32 = 0x008b_bf40;
/// Virtual slot `0x19c` of a reference, no arguments.
const REFR_SLOT_19C: u32 = 0x19c;
/// `thiscall` (`008255a0`, `fallout/magic` range), no arguments.
const MAGIC_FUNCTION_008255A0: u32 = 0x0082_55a0;
/// `thiscall` on a quest (`0060c8e0`, `RET 4`: `objective index`): the
/// objective of the quest's second list or null.
const QUEST_FIND_OBJECTIVE: u32 = 0x0060_c8e0;
/// `fastcall` on a quest (`0059e400`): `AL`, bit 1 of the quest's flag byte.
const QUEST_FLAG_2: u32 = 0x0059_e400;
/// `fastcall` on an objective (`005a5dc0`): whether its state (`+0x20`) is
/// above 1.
const OBJECTIVE_ABOVE_ONE: u32 = 0x005a_5dc0;
/// `fastcall` on an objective (`005a5e70`): bit 0 of its state (`+0x20`).
const OBJECTIVE_BIT_0: u32 = 0x005a_5e70;

/// `"Global Time Multiplier >> '%0.2f'"`
const MSG_GLOBAL_TIME_MULTIPLIER: u32 = 0x0103_c628;
/// `"GetHitLocation >> %0.2f"`
const MSG_GET_HIT_LOCATION: u32 = 0x0103_c64c;
/// `"GetLastHitCritical >> %0.2f"`
const MSG_GET_LAST_HIT_CRITICAL: u32 = 0x0103_c664;
/// `"IsPC1stPerson >> %0.2f"`
const MSG_IS_PC_1ST_PERSON: u32 = 0x0103_c680;
/// `"%s not found"`
const MSG_NOT_FOUND: u32 = 0x0103_c698;
/// `"%s >> %0.2f"`
const MSG_NAME_VALUE: u32 = 0x0103_c6a8;
/// `"Unknown option '%s'"`
const MSG_UNKNOWN_OPTION: u32 = 0x0103_c6b4;
/// `"All Combat debugging turned %s"`
const MSG_ALL_COMBAT_DEBUGGING: u32 = 0x0103_cad4;
/// `"Combat Debug Text color changed to %s"`
const MSG_COMBAT_TEXT_COLOUR: u32 = 0x0103_c7d8;
/// `"Dark"`
const TEXT_DARK: u32 = 0x0103_c800;
/// `"Light"`
const TEXT_LIGHT: u32 = 0x0101_1f58;
/// `"Combat Debug Text size changed to %.2f"`
const MSG_COMBAT_TEXT_SIZE: u32 = 0x0103_c7a0;
/// `"%s is not an actor"`
const MSG_NOT_AN_ACTOR: u32 = 0x0103_cb68;
/// `"%s is not in combat"`
const MSG_NOT_IN_COMBAT: u32 = 0x0103_5d3c;
/// `"The combat group has only one member"`
const MSG_ONE_MEMBER: u32 = 0x0103_cb40;
/// `"Group strategy cleared"`
const MSG_STRATEGY_CLEARED: u32 = 0x0103_cb28;
/// `"Group strategy set to: %s"`
const MSG_STRATEGY_SET: u32 = 0x0103_cb0c;
/// `"Unknown strategy '%s'"`
const MSG_UNKNOWN_STRATEGY: u32 = 0x0103_caf4;
/// `"%s processing is  %s"`
const MSG_PROCESSING_IS: u32 = 0x0103_9f58;
/// `"SCRIPTS: PushActorAway in script '%s' is attempting to push a non-actor
/// reference."`
const MSG_PUSH_NON_ACTOR: u32 = 0x0103_cb80;
/// `"%s (%08X) has %i items:"`
const MSG_HAS_ITEMS: u32 = 0x0103_cc6c;
/// `" [CANNOT EQUIP]"`
const TEXT_CANNOT_EQUIP: u32 = 0x0103_cc5c;
/// `" - Worn"`
const TEXT_WORN: u32 = 0x0103_cc54;
/// `"%d - %s (%08X) (%.0f,%0.2f%%)%s%s"`
const MSG_ITEM_WITH_EQUIP: u32 = 0x0103_cc30;
/// `"%d - %s (%08X) (%.0f,%0.2f%%)%s"`
const MSG_ITEM_WITH_HEALTH: u32 = 0x0103_cc10;
/// `"%d - %s (%08X)%s"`
const MSG_ITEM_PLAIN: u32 = 0x0103_cbfc;
/// `"  === %s (%08X) ==="`
const MSG_BIPED_LIST_HEADER: u32 = 0x0103_cbe8;
/// `"  ---> %s (%08X)"`
const MSG_BIPED_LIST_ENTRY: u32 = 0x0103_cbd4;
/// `"IsMoving >> %0.0f"`
const MSG_IS_MOVING: u32 = 0x0103_ce44;
/// `"IsTurning >> %0.0f"`
const MSG_IS_TURNING: u32 = 0x0103_ce58;
/// `"GetAnimAction >> %0.0f"`
const MSG_GET_ANIM_ACTION: u32 = 0x0103_ce6c;
/// `"Decal Rendering On"`
const MSG_DECAL_RENDERING_ON: u32 = 0x0103_ce1c;
/// `"Decal Rendering Off"`
const MSG_DECAL_RENDERING_OFF: u32 = 0x0103_ce30;
/// `"Script error: quest %s (%08X) does not exist."`
const MSG_QUEST_MISSING: u32 = 0x0103_ce84;
/// `"Script error: objective %d does not exist in quest %s (%08X)"`
const MSG_OBJECTIVE_MISSING: u32 = 0x0103_ceb4;

/// Offsets in the water shader property (`NiAVObject::GetProperty(3)` of the
/// water object), PC build. Each is described by what `ModifyWaterShader`
/// does with it.
mod water {
    /// Byte: cleared by every setting, set by `off`.
    pub const MANUAL_FLAG: u32 = 0x7f;
    /// Byte toggled by `reflect` (and set by `refract`, see there).
    pub const REFLECT_FLAG: u32 = 0x80;
    /// Byte toggled by `refract`.
    pub const REFRACT_FLAG: u32 = 0x81;
    /// Byte toggled by `depth`.
    pub const DEPTH_FLAG: u32 = 0x63;
    /// `float`: `reflectamt`.
    pub const REFLECT_AMOUNT: u32 = 0xbc;
    /// `float`: `opacity` (divided by 100).
    pub const OPACITY: u32 = 0xc0;
    /// `float`: `distort`.
    pub const DISTORTION: u32 = 0xc4;
    /// `float`: `fresnel`.
    pub const FRESNEL: u32 = 0x118;
    /// `float`: `noise`.
    pub const NOISE_SCALE: u32 = 0x11c;
    /// `float`: `fog` (divided by 100).
    pub const FOG: u32 = 0x120;
}

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

/// The player pointer.
fn player(e: &Engine) -> Ptr {
    Ptr::new(e.global::<u32>(PLAYER))
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

/// The commands that call a condition function with no argument, echo the
/// result double with `format` when the TLS flag is set and return the
/// condition function's `AL`.
fn condition_with_echo(e: &mut Engine, a: ScriptArgs, function: u32, format: u32) -> bool {
    let verdict = call_condition(e, function, a, 0);
    if echo_enabled(e) {
        let value = e.mem.f64(a.result.addr());
        console_print(e, &args![format, value]);
    }
    verdict
}

/// `_ftol2` of a `float`.
fn float_to_int(e: &mut Engine, value: f32) -> u32 {
    e.call(FTOL2, &args![f64::from(value)]).u32()
}

/// `0040ebd0(0040404010(...))`: `max(a, b)` through [`FLOAT_MAX`] and then
/// [`FLOAT_MIN`] with `upper`, i.e. `FLOAT_MIN(upper, FLOAT_MAX(0.0, value))`.
fn limit_to(e: &mut Engine, value: f32, upper: f32) -> f32 {
    let floor = e.call(FLOAT_MAX, &args![0.0f32, value]).f32();
    e.call(FLOAT_MIN, &args![upper, floor]).f32()
}

// ---- Translated functions --------------------------------------------------------

// Translated from 005d21e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::StopMagicEffect` (Xbox PDB): parses a form and a `float` (which
/// starts at -1.0 and is not used). When the string at `form + 0x18 + 4` (the
/// component at `form + 0x18`) is not empty, the process lists are told to remove the
/// visual effect of its model path (virtual slot `0x14`) from `thisObj` (the
/// player when none is given). With the TLS echo flag set, the console says
/// that the visual effect has been removed from the reference's name, or from
/// "reference" when it has none.
pub fn script_stop_magic_effect(e: &mut Engine, a: ScriptArgs) -> bool {
    let minus_one = e.global::<u32>(FLOAT_MINUS_ONE);
    let Some([form, _unused_scale]) = parse_params(e, a, [0, minus_one]) else {
        return false;
    };
    let mut this_obj = a.this_obj;
    if this_obj.is_null() {
        this_obj = player(e);
    }
    if form != 0 {
        // The model component of the form, `TESForm + 0x18`.
        let model = form + 0x18;
        if e.call(MODEL_PATH_LENGTH, &args![model]).u32() > 0 {
            let path = e.vcall(model, MODEL_PATH_SLOT, &args![]).u32();
            e.call(
                PROCESS_LISTS_REMOVE_VISUAL_EFFECT,
                &args![PROCESS_LISTS, this_obj, path],
            );
        }
    }
    if echo_enabled(e) {
        if e.call(GET_REFERENCE_NAME, &args![this_obj]).u32() == 0 {
            console_print(e, &args![MSG_EFFECT_REMOVED_UNNAMED]);
        } else {
            let name = e.call(GET_REFERENCE_NAME, &args![this_obj]).u32();
            console_print(e, &args![MSG_EFFECT_REMOVED, name]);
        }
    }
    true
}

// Translated from 005d22d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PreloadMagicEffect` (Xbox PDB): parses a form; when the string at `form + 0x18 + 4`
/// (the component at `form + 0x18`) is not empty and `ModelLoader::FindModel` does
/// not already know the model path (virtual slot `0x14`), queues the model
/// (`QueueModel(path, 5, 0, 0, 1, 0, 0)`). The key slot `FindModel` is given
/// lives on the stack for the call.
pub fn script_preload_magic_effect(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([form]) = parse_params(e, a, [0]) else {
        return false;
    };
    if form == 0 {
        return true;
    }
    let model = form + 0x18;
    if e.call(MODEL_PATH_LENGTH, &args![model]).u32() == 0 {
        return true;
    }
    e.with_stack(4, |e, key| {
        e.call(MODEL_KEY_CONSTRUCT, &args![key, 0u32]);
        let loader = e.global::<u32>(MODEL_LOADER);
        let path = e.vcall(model, MODEL_PATH_SLOT, &args![]).u32();
        if !e
            .call(MODEL_LOADER_FIND_MODEL, &args![loader, path, key])
            .bool()
        {
            let path = e.vcall(model, MODEL_PATH_SLOT, &args![]).u32();
            e.call(
                MODEL_LOADER_QUEUE_MODEL,
                &args![loader, path, 5u32, 0u32, 0u32, 1u32, 0u32, 0u32],
            );
        }
        e.call(MODEL_KEY_DESTRUCT, &args![key]);
    });
    true
}

// Translated from 005d23d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An alpha command (it ends with `Actor::UpdateAlpha`): parses a `float`,
/// takes `thisObj` (the player when none) and, when it is an actor (virtual slot `0x100`) with a process,
/// limits the value to 0..1 (`FLOAT_MIN(1.0, FLOAT_MAX(0.0, value))`), hands
/// it to slot `0x5b0` of the process and calls `Actor::UpdateAlpha`. Always
/// succeeds once the parameters parse.
pub fn fn_005d23d0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0.0f32.to_bits()]) else {
        return false;
    };
    let mut this_obj = a.this_obj;
    if this_obj.is_null() {
        this_obj = player(e);
    }
    if !e
        .vcall(this_obj.addr(), REFR_IS_ACTOR_SLOT, &args![])
        .bool()
        || e.call(GET_PROCESS, &args![this_obj]).u32() == 0
    {
        return true;
    }
    let value = limit_to(e, f32::from_bits(value), 1.0);
    let process = e.call(GET_PROCESS, &args![this_obj]).u32();
    e.vcall(process, PROCESS_SET_ALPHA_SLOT, &args![value]);
    e.call(ACTOR_UPDATE_ALPHA, &args![this_obj]);
    true
}

// Translated from 005d24a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an object; when it is not null calls its method `00546b10` with
/// `-1, 0`.
pub fn fn_005d24a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([object]) = parse_params(e, a, [0]) else {
        return false;
    };
    if object != 0 {
        e.call(CELL_UPDATE, &args![object, 0xffff_ffffu32, 0u32]);
    }
    true
}

// Translated from 005d2500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `IsInInteriorConditionFunction(thisObj, 0, 0, result)` and always
/// succeeds (its `AL` is 1, not the condition function's).
pub fn fn_005d2500(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_IN_INTERIOR_CONDITION, a, 0);
    true
}

// Translated from 005d2520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ModPCMiscStat` (Xbox PDB): parses a stat id and an amount and
/// calls `MiscStatManager::ModVal(stat, amount)`.
pub fn script_mod_pc_misc_stat(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([stat, amount]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    e.call(MISC_STAT_MOD_VAL, &args![stat, amount]);
    true
}

// Translated from 005d2590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPCMiscStat` (Xbox PDB): parses a stat id and calls
/// `GetPCMiscStatConditionFunction(thisObj, stat, 0, result)`.
pub fn script_get_pc_misc_stat(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([stat]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, GET_PC_MISC_STAT_CONDITION, a, stat);
    true
}

// Translated from 005d25f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `IsActorEvil` body: returns `IsActorEvilConditionFunction(thisObj, 0,
/// 0, result)`.
pub fn fn_005d25f0(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_ACTOR_EVIL_CONDITION, a, 0)
}

// Translated from 005d2610 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `IsActorAVictim` body: returns `IsActorVictimConditionFunction(thisObj,
/// 0, 0, result)`.
pub fn fn_005d2610(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, IS_ACTOR_VICTIM_CONDITION, a, 0)
}

// Translated from 005d2630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the condition function `005a3570(thisObj, 0, 0, result)`.
pub fn fn_005d2630(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, CONDITION_005A3570, a, 0)
}

// Translated from 005d2650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetNoRumorsFunction` (Xbox PDB): the condition function
/// `005a35f0(thisObj, 0, 0, result)`, echoed as "GetNoRumors >> value".
pub fn script_get_no_rumors_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_echo(e, a, GET_NO_RUMORS_CONDITION, MSG_GET_NO_RUMORS)
}

// Translated from 005d26b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a flag; when `thisObj` is an actor, compares the flag with the
/// `0x2000` flag of the actor's base data (`005d2780` on `base form +
/// 0x30`): when they agree the list at `actor + 0x44` is cleared
/// (`004216b0`), otherwise it is set to the flag (`00421600`). Then slot
/// `0x48` of the actor is called with `0x80000000`.
pub fn fn_005d26b0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([flag_value]) = parse_params(e, a, [0]) else {
        return false;
    };
    let actor = actor_of(e, a.this_obj.addr());
    if actor != 0 {
        let flag = (flag_value != 0) as u32;
        let base_form = e.call(GET_BASE_FORM_OF_REFERENCE, &args![actor]).u32();
        // The actor base data inside the base form, `+0x30` on PC.
        let current = fn_005d2780(e, Ptr::new(base_form + 0x30)) as u32;
        if flag == current {
            let list = fn_005d43c0(e, actor);
            e.call(EXTRA_DATA_LIST_CLEAR, &args![list]);
        } else {
            let list = fn_005d43c0(e, actor);
            e.call(EXTRA_DATA_LIST_SET, &args![list, flag]);
        }
        e.vcall(actor, MARK_CHANGED_SLOT, &args![0x8000_0000u32]);
    }
    true
}

// Translated from 005d2780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the flag `0x2000` is set in the flags word at `this + 4`
/// (`00461580(this, 0x2000)`; the base data block of an actor).
pub fn fn_005d2780(e: &mut Engine, this: Ptr) -> u8 {
    e.call(FLAGS_TEST, &args![this, 0x2000u32]).u8()
}

// Translated from 005d27a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetWhichServiceFunction` (Xbox PDB): the condition function
/// `005a3650(thisObj, 0, 0, result)`, echoed as "GetWhichSerivce >> value".
pub fn script_get_which_service_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_echo(e, a, GET_WHICH_SERVICE_CONDITION, MSG_GET_WHICH_SERVICE)
}

// Translated from 005d2800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::TriggerHitShaderFunction` (Xbox PDB): parses a `float` (default
/// 1.0) and triggers the image space modifier [`tes_image_space_modifier_get_get_hit`]
/// returns with that strength (`Trigger(modifier, strength, 0)`).
pub fn script_trigger_hit_shader_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([strength]) = parse_params(e, a, [1.0f32.to_bits()]) else {
        return false;
    };
    let modifier = tes_image_space_modifier_get_get_hit(e);
    e.call(
        IMAGE_SPACE_MODIFIER_TRIGGER,
        &args![modifier, f32::from_bits(strength), 0u32],
    );
    true
}

// Translated from 005d2860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifier::GetGetHit` (Xbox PDB): null while the byte at
/// `0118abb1` is clear; otherwise the modifier cached at `011c9700`, looked
/// up the first time as the default form with index `0x162` cast from
/// `TESForm` to `TESImageSpaceModifier`.
pub fn tes_image_space_modifier_get_get_hit(e: &mut Engine) -> u32 {
    if e.global::<u8>(IMAGE_SPACE_ENABLED) == 0 {
        return 0;
    }
    if e.global::<u32>(CACHED_GET_HIT_MODIFIER) == 0 {
        let form = e
            .call(GET_DEFAULT_FORM, &args![GET_HIT_DEFAULT_FORM_INDEX])
            .u32();
        let modifier = e
            .call(
                DYNAMIC_CAST,
                &args![
                    form,
                    0u32,
                    RTTI_TES_FORM,
                    RTTI_TES_IMAGE_SPACE_MODIFIER,
                    0u32
                ],
            )
            .u32();
        e.set_global(CACHED_GET_HIT_MODIFIER, modifier);
    }
    e.global::<u32>(CACHED_GET_HIT_MODIFIER)
}

// Translated from 005d28b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsActorRidingHorseFunction` (Xbox PDB): the condition function
/// `005a3670(thisObj, 0, 0, result)`, echoed as "Is actor riding horse >>
/// value".
pub fn script_is_actor_riding_horse_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_echo(
        e,
        a,
        IS_ACTOR_RIDING_HORSE_CONDITION,
        MSG_IS_ACTOR_RIDING_HORSE,
    )
}

// Translated from 005d2910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPlayersLastRiddenHorseFunction` (Xbox PDB): the condition
/// function `005a2a50(thisObj, 0, 0, result)`, echoed as "Actor players last
/// ridden horse >> value".
pub fn script_is_players_last_ridden_horse_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_echo(
        e,
        a,
        IS_PLAYERS_LAST_RIDDEN_HORSE_CONDITION,
        MSG_PLAYERS_LAST_RIDDEN_HORSE,
    )
}

// Translated from 005d2970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsInDangerousWaterFunction` (Xbox PDB):
/// `IsInDangerousWaterConditionFunction(thisObj, 0, 0, result)`, echoed as
/// "Actor is in lava >> value".
pub fn script_is_in_dangerous_water_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_with_echo(e, a, IS_IN_DANGEROUS_WATER_CONDITION, MSG_IN_LAVA)
}

// Translated from 005d29d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleWaterSystem` (Xbox PDB): when the flag [`fn_005d2a40`]
/// reads is clear prints "Water System On" and calls
/// `TESWaterSystem::EnableWaterSystem` on the water system; otherwise prints
/// "Water System Off" and calls `004e6620(1, 0)` on it. The result is the
/// `AL` of that last call. (The first `0070ec90` call, whose result is not
/// used, is made as the code does.)
pub fn script_toggle_water_system(e: &mut Engine, _args: ScriptArgs) -> bool {
    let tes = e.global::<u32>(TES_SINGLETON);
    e.call(TES_GET_WATER_SYSTEM, &args![tes]);
    if fn_005d2a40(e) != 0 {
        console_print(e, &args![MSG_WATER_SYSTEM_OFF]);
        let tes = e.global::<u32>(TES_SINGLETON);
        let water = e.call(TES_GET_WATER_SYSTEM, &args![tes]).u32();
        e.call(WATER_SYSTEM_DISABLE, &args![water, 1u32, 0u32])
            .bool()
    } else {
        console_print(e, &args![MSG_WATER_SYSTEM_ON]);
        let tes = e.global::<u32>(TES_SINGLETON);
        let water = e.call(TES_GET_WATER_SYSTEM, &args![tes]).u32();
        e.call(WATER_SYSTEM_ENABLE, &args![water]).bool()
    }
}

// Translated from 005d2a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `01189624` (the water system flag the water commands test).
pub fn fn_005d2a40(e: &mut Engine) -> u8 {
    e.global::<u8>(WATER_SYSTEM_FLAG)
}

// Translated from 005d2a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0070ec90` on the TES singleton (the result is not used) and, when
/// [`fn_005d2a40`] is set, sets the byte at `011c7a64` to 1. Always
/// succeeds.
pub fn fn_005d2a50(e: &mut Engine, _args: ScriptArgs) -> bool {
    let tes = e.global::<u32>(TES_SINGLETON);
    e.call(TES_GET_WATER_SYSTEM, &args![tes]);
    if fn_005d2a40(e) != 0 {
        e.set_global(WATER_FLAG_011C7A64, 1u8);
    }
    true
}

/// Whether `0 <= value <= upper` (`upper` read as a `double` from the exe),
/// the range test of the `ModifyWaterShader` settings: false for NaN too.
fn within(e: &Engine, value: f32, upper_address: u32) -> bool {
    let value = f64::from(value);
    value >= e.global::<f64>(DOUBLE_ZERO) && value <= e.global::<f64>(upper_address)
}

/// Whether the parsed setting name is `name` (`__stricmp` is 0).
fn name_is(e: &mut Engine, buffer: u32, name: u32) -> bool {
    e.call(STRICMP, &args![buffer, name]).i32() == 0
}

/// A range-checked setting of the global water values: stores the value at
/// `global` and raises the "changed" byte when it is within `0..upper`.
fn set_global_water_value(e: &mut Engine, value: f32, upper_address: u32, global: u32) {
    if within(e, value, upper_address) {
        e.set_global(global, value);
        e.set_global(WATER_SETTINGS_CHANGED, 1u8);
    }
}

/// A toggle of a flag byte of the water shader property (`property_flag`
/// offset) or, without a property, of the value byte of the global object at
/// `global_object`, with the four messages: property off/on, global off/on.
/// `set_offset` is the byte the property branch writes when the flag was
/// clear (the same as `property_flag` except for `refract`).
fn toggle_water_flag(
    e: &mut Engine,
    property: u32,
    property_flag: u32,
    set_offset: u32,
    global_object: u32,
    messages: [u32; 4],
) {
    let [property_off, property_on, global_off, global_on] = messages;
    if property != 0 {
        if e.mem.u8(property + property_flag) != 0 {
            e.mem.set_u8(property + property_flag, 0);
            console_print(e, &args![property_off]);
        } else {
            e.mem.set_u8(property + set_offset, 1);
            console_print(e, &args![property_on]);
        }
    } else {
        let value_address = e.call(GLOBAL_VALUE_ADDRESS, &args![global_object]).u32();
        if e.mem.u8(value_address) != 0 {
            e.call(GLOBAL_VALUE_SET, &args![global_object, 0u32]);
            console_print(e, &args![global_off]);
        } else {
            e.call(GLOBAL_VALUE_SET, &args![global_object, 1u32]);
            console_print(e, &args![global_on]);
        }
    }
}

/// The global water objects whose value byte `reflect`, `refract` and
/// `depth` toggle.
const GLOBAL_REFLECT_OBJECT: u32 = 0x011c_7b6c;
const GLOBAL_REFRACT_OBJECT: u32 = 0x011c_7c60;
const GLOBAL_DEPTH_OBJECT: u32 = 0x011c_7bbc;

// Translated from 005d2a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ModifyWaterShader` (Xbox PDB), the `mws` command: parses a
/// setting name (a text buffer) and a `float`; when they do not parse, prints
/// the list of settings and fails. The settings apply to the water shader
/// property of `thisObj` (found through the water system and
/// `NiAVObject::GetProperty(3)`) or, without a reference, to the global water
/// values. The name is compared in this order (`__stricmp`, first match
/// wins): `velocity`, `direction`, `amplitude`, `frequency` (accepted, do
/// nothing), `reflectamt`, `distort`, `fresnel`, `opacity`, `blend` (does
/// nothing), `noise`, `off`, `displaceforce`, `displacevelocity`,
/// `displacefalloff`, `displacedampener`, `rainforce`, `rainvelocity`,
/// `rainfalloff`, `rainsize`, `reflect`, `refract`, `depth`, `fog`, `lod`,
/// `autowater`. Range-checked values (0..1, 0..100, 0..20) that are outside
/// their range are ignored; `reflectamt`, `fresnel`, `opacity` and `noise`
/// print a usage message. Setting the property clears its byte `+0x7f` (the
/// global values raise the byte `011c7a65`). Succeeds whenever the
/// parameters parse.
pub fn script_modify_water_shader(e: &mut Engine, a: ScriptArgs) -> bool {
    // The text buffer (0x208 bytes) and the float local of the stack frame.
    e.with_stack(0x20c, |e, block| {
        let buffer = block.addr();
        let value_cell = buffer + 0x208;
        e.mem.set_f32(value_cell, 0.0);
        if !parse(e, a, &[buffer, value_cell]) {
            console_print(e, &args![MSG_MWS_USAGE]);
            return false;
        }
        modify_water_shader(e, a, buffer, e.mem.f32(value_cell));
        true
    })
}

/// The body of [`script_modify_water_shader`] after the parameters parsed.
fn modify_water_shader(e: &mut Engine, a: ScriptArgs, buffer: u32, value: f32) {
    let mut property = 0;
    if !a.this_obj.is_null() {
        let tes = e.global::<u32>(TES_SINGLETON);
        let water_system = e.call(TES_GET_WATER_SYSTEM, &args![tes]).u32();
        let object = e
            .call(WATER_SYSTEM_GET_OBJECT, &args![water_system, a.this_obj])
            .u32();
        property = e
            .call(NI_AV_OBJECT_GET_PROPERTY, &args![object, 3u32])
            .u32();
    }
    // `velocity`, `direction`, `amplitude` and `frequency` are accepted and
    // do nothing (`direction` compares the value with 360.0 and drops the
    // answer).
    for ignored in [
        NAME_VELOCITY,
        NAME_DIRECTION,
        NAME_AMPLITUDE,
        NAME_FREQUENCY,
    ] {
        if name_is(e, buffer, ignored) {
            return;
        }
    }
    if name_is(e, buffer, NAME_REFLECT_AMOUNT) {
        if within(e, value, DOUBLE_ONE) {
            if property != 0 {
                e.mem.set_f32(property + water::REFLECT_AMOUNT, value);
                console_print(e, &args![MSG_REFLECT_AMOUNT, f64::from(value)]);
                e.mem.set_u8(property + water::MANUAL_FLAG, 0);
            } else {
                e.set_global(0x011f_f3ec, value);
                console_print(e, &args![MSG_REFLECT_AMOUNT, f64::from(value)]);
                e.set_global(WATER_SETTINGS_CHANGED, 1u8);
            }
        } else {
            console_print(e, &args![MSG_USAGE_REFLECT_AMOUNT, f64::from(value)]);
        }
        return;
    }
    if name_is(e, buffer, NAME_DISTORT) {
        if property != 0 {
            e.mem.set_f32(property + water::DISTORTION, value);
            console_print(e, &args![MSG_DISTORTION, f64::from(value)]);
            e.mem.set_u8(property + water::MANUAL_FLAG, 0);
        } else {
            e.set_global(0x011f_f3f4, value);
            console_print(e, &args![MSG_DISTORTION, f64::from(value)]);
            e.set_global(WATER_SETTINGS_CHANGED, 1u8);
        }
        return;
    }
    if name_is(e, buffer, NAME_FRESNEL) {
        if within(e, value, DOUBLE_ONE) {
            if property != 0 {
                e.mem.set_f32(property + water::FRESNEL, value);
                console_print(e, &args![MSG_FRESNEL, f64::from(value)]);
                e.mem.set_u8(property + water::MANUAL_FLAG, 0);
            } else {
                e.set_global(0x011f_f10c, value);
                console_print(e, &args![MSG_FRESNEL, f64::from(value)]);
                e.set_global(WATER_SETTINGS_CHANGED, 1u8);
            }
        } else {
            console_print(e, &args![MSG_USAGE_FRESNEL, f64::from(value)]);
        }
        return;
    }
    if name_is(e, buffer, NAME_OPACITY) {
        if within(e, value, DOUBLE_HUNDRED) {
            let scaled = (f64::from(value) / e.global::<f64>(DOUBLE_HUNDRED)) as f32;
            if property != 0 {
                e.mem.set_f32(property + water::OPACITY, scaled);
                let percent = float_to_int(e, value);
                console_print(e, &args![MSG_OPACITY, percent]);
                e.mem.set_u8(property + water::MANUAL_FLAG, 0);
            } else {
                e.set_global(0x011f_f3f0, scaled);
                let percent = float_to_int(e, value);
                console_print(e, &args![MSG_OPACITY, percent]);
                e.set_global(WATER_SETTINGS_CHANGED, 1u8);
            }
        } else {
            console_print(e, &args![MSG_USAGE_OPACITY, f64::from(value)]);
        }
        return;
    }
    if name_is(e, buffer, NAME_BLEND) {
        return;
    }
    if name_is(e, buffer, NAME_NOISE) {
        if within(e, value, DOUBLE_HUNDRED) {
            if property != 0 {
                e.mem.set_f32(property + water::NOISE_SCALE, value);
                console_print(e, &args![MSG_NOISE_SCALE, f64::from(value)]);
                e.mem.set_u8(property + water::MANUAL_FLAG, 0);
            } else {
                e.set_global(0x011f_fe48, value);
                console_print(e, &args![MSG_NOISE, f64::from(value)]);
                e.set_global(WATER_SETTINGS_CHANGED, 1u8);
            }
        } else {
            console_print(e, &args![MSG_USAGE_NOISE, f64::from(value)]);
        }
        return;
    }
    if name_is(e, buffer, NAME_OFF) {
        if property != 0 {
            e.mem.set_u8(property + water::MANUAL_FLAG, 1);
        } else {
            e.set_global(WATER_SETTINGS_CHANGED, 0u8);
        }
        return;
    }
    // Settings that only store the value in a global when it is within
    // `0..upper`.
    let ranged_globals = [
        (NAME_DISPLACE_FORCE, DOUBLE_ONE, 0x0120_0014),
        (NAME_DISPLACE_VELOCITY, DOUBLE_ONE, 0x0120_0018),
        (NAME_DISPLACE_FALLOFF, DOUBLE_ONE, 0x0120_001c),
        (NAME_DISPLACE_DAMPENER, DOUBLE_TWENTY, 0x011f_fff0),
        (NAME_RAIN_FORCE, DOUBLE_ONE, 0x0120_0004),
        (NAME_RAIN_VELOCITY, DOUBLE_ONE, 0x0120_0008),
        (NAME_RAIN_FALLOFF, DOUBLE_ONE, 0x0120_000c),
        (NAME_RAIN_SIZE, DOUBLE_ONE, 0x0120_0010),
    ];
    for (name, upper, global) in ranged_globals {
        if name_is(e, buffer, name) {
            set_global_water_value(e, value, upper, global);
            return;
        }
    }
    if name_is(e, buffer, NAME_REFLECT) {
        toggle_water_flag(
            e,
            property,
            water::REFLECT_FLAG,
            water::REFLECT_FLAG,
            GLOBAL_REFLECT_OBJECT,
            [
                MSG_REFLECTIONS_OFF,
                MSG_REFLECTIONS_ON,
                MSG_GLOBAL_REFLECTIONS_OFF,
                MSG_GLOBAL_REFLECTIONS_ON,
            ],
        );
        return;
    }
    if name_is(e, buffer, NAME_REFRACT) {
        // The code tests the byte `+0x81` but, when it is clear, sets
        // `+0x80` (the byte `reflect` uses).
        toggle_water_flag(
            e,
            property,
            water::REFRACT_FLAG,
            water::REFLECT_FLAG,
            GLOBAL_REFRACT_OBJECT,
            [
                MSG_REFRACTIONS_OFF,
                MSG_REFRACTIONS_ON,
                MSG_GLOBAL_REFRACTIONS_OFF,
                MSG_GLOBAL_REFRACTIONS_ON,
            ],
        );
        return;
    }
    if name_is(e, buffer, NAME_DEPTH) {
        toggle_water_flag(
            e,
            property,
            water::DEPTH_FLAG,
            water::DEPTH_FLAG,
            GLOBAL_DEPTH_OBJECT,
            [
                MSG_DEPTH_OFF,
                MSG_DEPTH_ON,
                MSG_GLOBAL_DEPTH_OFF,
                MSG_GLOBAL_DEPTH_ON,
            ],
        );
        return;
    }
    if name_is(e, buffer, NAME_FOG) {
        if within(e, value, DOUBLE_HUNDRED) {
            let scaled = (f64::from(value) / e.global::<f64>(DOUBLE_HUNDRED)) as f32;
            if property != 0 {
                e.mem.set_f32(property + water::FOG, scaled);
                let percent = float_to_int(e, value);
                console_print(e, &args![MSG_FOG_AMOUNT, percent]);
                e.mem.set_u8(property + water::MANUAL_FLAG, 0);
            } else {
                e.set_global(0x011f_f414, scaled);
                let percent = float_to_int(e, value);
                console_print(e, &args![MSG_FOG_AMOUNT, percent]);
                e.set_global(WATER_SETTINGS_CHANGED, 1u8);
            }
        }
        return;
    }
    if name_is(e, buffer, NAME_LOD) {
        let tes = e.global::<u32>(TES_SINGLETON);
        if e.call(TES_GET_WATER_SYSTEM, &args![tes]).u32() != 0 {
            let tes = e.global::<u32>(TES_SINGLETON);
            let water_system = e.call(TES_GET_WATER_SYSTEM, &args![tes]).u32();
            e.call(
                WATER_SYSTEM_TOGGLE_LOD,
                &args![water_system, 1u32, 1u32, 1u32],
            );
        }
        console_print(e, &args![MSG_TOGGLE_LOD_WATER]);
        return;
    }
    if name_is(e, buffer, NAME_AUTOWATER) {
        time_water_generation(e);
    }
}

/// The `autowater` setting: generates the placeable water of the current
/// cell, times it with the performance counter and reports the ticks and
/// microseconds through `Error`; then switches the water system on when the
/// flag [`fn_005d2a40`] reads is clear. (The code also stores 1000.0 and 0
/// into locals first; both are overwritten before they are read.)
fn time_water_generation(e: &mut Engine) {
    // Frequency, start and end (8 bytes each), then the 256-byte message.
    e.with_stack(0x118, |e, block| {
        let (frequency_cell, start_cell, end_cell, text) = (
            block.addr(),
            block.addr() + 8,
            block.addr() + 16,
            block.addr() + 24,
        );
        e.call(QUERY_PERFORMANCE_FREQUENCY, &args![frequency_cell]);
        let frequency = e.mem.u64(frequency_cell) as i64 as f64;
        let ticks_per_microsecond = frequency * e.global::<f64>(DOUBLE_MICRO);
        e.call(QUERY_PERFORMANCE_COUNTER, &args![start_cell]);
        let tes = e.global::<u32>(TES_SINGLETON);
        let cell = e.call(TES_GET_CURRENT_CELL, &args![tes]).u32();
        e.call(GENERATE_PLACEABLE_WATER_FOR_CELL, &args![cell]);
        e.call(QUERY_PERFORMANCE_COUNTER, &args![end_cell]);
        let ticks = e.mem.u64(end_cell).wrapping_sub(e.mem.u64(start_cell));
        let microseconds = ticks as i64 as f64 / ticks_per_microsecond;
        let cell = e.call(TES_GET_CURRENT_CELL, &args![tes]).u32();
        let name = e.call(CELL_GET_NAME, &args![cell]).u32();
        e.call(
            SPRINTF_S,
            &args![
                text,
                0xffu32,
                MSG_GENERATE_WATER_TIMING,
                name,
                ticks,
                microseconds
            ],
        );
        e.call(ERROR_REPORT, &args![text, 0u32]);
    });
    if fn_005d2a40(e) == 0 {
        let tes = e.global::<u32>(TES_SINGLETON);
        let water_system = e.call(TES_GET_WATER_SYSTEM, &args![tes]).u32();
        e.call(WATER_SYSTEM_ENABLE, &args![water_system]);
    }
}

/// The three colour commands: parse red, green and blue (0..255), find the
/// water shader property of `thisObj` (slot `0x1d0`, then `0043b4a0(.., 0)`
/// and `NiAVObject::GetProperty(3)`), and when all three are within 0..255
/// store the colour (each as `value / 255.0`, alpha 1.0) at
/// `property + property_offset` or, without a reference, at the four words
/// at `global`.
fn set_water_colour(e: &mut Engine, a: ScriptArgs, property_offset: u32, global: u32) -> bool {
    let Some([red, green, blue]) = parse_params(e, a, [0, 0, 0]) else {
        return false;
    };
    let mut property = 0;
    if !a.this_obj.is_null() {
        let target = e
            .vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
            .u32();
        let shape = e.call(GET_SHAPE, &args![target, 0u32]).u32();
        property = e.call(NI_AV_OBJECT_GET_PROPERTY, &args![shape, 3u32]).u32();
    }
    let in_range = |value: u32| (0..=0xff).contains(&(value as i32));
    if in_range(red) && in_range(green) && in_range(blue) {
        let scale = e.global::<f64>(DOUBLE_255);
        let channel = |value: u32| (f64::from(value as i32) / scale) as f32;
        let (red, green, blue) = (channel(red), channel(green), channel(blue));
        e.with_stack(16, |e, colour| {
            let result = e
                .call(COLOUR_CONSTRUCT, &args![colour, red, green, blue, 1.0f32])
                .u32();
            let destination = if property != 0 {
                property + property_offset
            } else {
                global
            };
            for word in 0..4 {
                let value = e.mem.u32(result + 4 * word);
                e.mem.set_u32(destination + 4 * word, value);
            }
        });
    }
    true
}

// Translated from 005d35d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A water colour command: sets the colour at `+0x88` of the water shader
/// property of `thisObj`, or the four words at `011ff3b8` without a
/// reference ([`set_water_colour`]).
pub fn fn_005d35d0(e: &mut Engine, a: ScriptArgs) -> bool {
    set_water_colour(e, a, 0x88, 0x011f_f3b8)
}

// Translated from 005d3780 (decompiled, FalloutNV.exe 1.4.0.525)
/// A water colour command like [`fn_005d35d0`]: the colour at `+0x98` of the
/// property, or the four words at `011ff3c8`.
pub fn fn_005d3780(e: &mut Engine, a: ScriptArgs) -> bool {
    set_water_colour(e, a, 0x98, 0x011f_f3c8)
}

// Translated from 005d3930 (decompiled, FalloutNV.exe 1.4.0.525)
/// A water colour command like [`fn_005d35d0`]: the colour at `+0xa8` of the
/// property, or the four words at `011ff3d8`.
pub fn fn_005d3930(e: &mut Engine, a: ScriptArgs) -> bool {
    set_water_colour(e, a, 0xa8, 0x011f_f3d8)
}

// Translated from 005d3ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::Reset3DStateFunction` (Xbox PDB): for a `thisObj`, unloads its 3D
/// (`0055f970`) and removes the saved animation and the saved Havok data from
/// its extra data list. Always succeeds.
pub fn script_reset_3d_state_function(e: &mut Engine, a: ScriptArgs) -> bool {
    if !a.this_obj.is_null() {
        e.call(REFERENCE_UNLOAD_3D, &args![a.this_obj]);
        let list = fn_005d43c0(e, a.this_obj.addr());
        e.call(EXTRA_DATA_LIST_REMOVE_SAVED_ANIMATION, &args![list]);
        let list = fn_005d43c0(e, a.this_obj.addr());
        e.call(EXTRA_DATA_LIST_REMOVE_SAVED_HAVOK_DATA, &args![list]);
    }
    true
}

// Translated from 005d3b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::AddAchievement` (Xbox PDB): parses an achievement id and, unless
/// the byte at `011d8ce4` is set, unlocks it through
/// `BSAwardsSystemUtility::QInstance()->Unlock(id)`.
pub fn script_add_achievement(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([achievement]) = parse_params(e, a, [0]) else {
        return false;
    };
    if e.global::<u8>(AWARDS_DISABLED) == 0 {
        let awards = e.call(AWARDS_QUERY_INSTANCE, &args![]).u32();
        e.call(AWARDS_UNLOCK, &args![awards, achievement]);
    }
    true
}

// Translated from 005d3b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a `float`; fails without a `thisObj`. When the integer game setting
/// `011c3ea4` is at most 1 the value goes to [`fn_005d3c20`]; otherwise the
/// call `0087a8b0(owner, 0x1156, thisObj, value as a double)` is made, `owner`
/// being what `004537b0` returns.
pub fn fn_005d3b90(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    if a.this_obj.is_null() {
        return false;
    }
    let value = f32::from_bits(value);
    let setting = e.call(GET_SETTING_INTEGER, &args![EVENT_SETTING]).u32();
    if e.mem.i32(setting) > 1 {
        let owner = e.call(GET_QUEUE_OWNER, &args![]).u32();
        e.call(
            SEND_REFERENCE_EVENT,
            &args![owner, EVENT_CODE_1156, a.this_obj, f64::from(value)],
        );
    } else {
        fn_005d3c20(e, a.this_obj, value);
    }
    true
}

// Translated from 005d3c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds a 16-byte block on the stack (a zero word, a byte 1, the word
/// `0x12`, the `float`) and calls `00c68ec0(object, block, callback)` with
/// `object` the result of slot `0x1d0` of `reference` and the callback
/// [`fn_005d3c70`].
pub fn fn_005d3c20(e: &mut Engine, reference: Ptr, value: f32) {
    e.with_stack(16, |e, block| {
        e.mem.set_u32(block.addr(), 0);
        // Only the low byte of this word is written by the code.
        e.mem.set_u8(block.addr() + 4, 1);
        e.mem.set_u32(block.addr() + 8, 0x12);
        e.mem.set_f32(block.addr() + 12, value);
        let object = e
            .vcall(reference.addr(), REFR_GET_TARGET_SLOT, &args![])
            .u32();
        e.call(FOR_EACH_ENTITY, &args![object, block, ENTITY_CALLBACK]);
    });
}

// Translated from 005d3c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback of [`fn_005d3c20`] (`cdecl`, `entity, block`): finds the
/// object registered for the entity's key (`006fa820`, then `00653270` in
/// the table at `012681c0`); when there is one, hands it the `float` at
/// `block + 0xc` ([`fn_005d3ce0`]) and passes the same `float` to slot `0xc`
/// of the object [`fn_005d3d10`] returns, if any.
pub fn fn_005d3c70(e: &mut Engine, entity: u32, block: Ptr) {
    let key = e.call(ENTITY_KEY, &args![entity]).u32();
    let registered = e.call(LOOKUP_BY_KEY, &args![ENTITY_TABLE, key]).u32();
    if registered != 0 {
        let value = e.mem.f32(block.addr() + 0xc);
        fn_005d3ce0(e, registered, value);
        let object = fn_005d3d10(e, registered);
        if object != 0 {
            let value = e.mem.f32(block.addr() + 0xc);
            e.vcall(object, PROXY_OBJECT_SLOT_C, &args![value]);
        }
    }
}

// Translated from 005d3ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` (`float`): when `004ae750(this)` gives a character proxy, calls
/// `00ca88c0(proxy, value)` on it.
pub fn fn_005d3ce0(e: &mut Engine, this: u32, value: f32) {
    let proxy = e.call(GET_PROXY, &args![this]).u32();
    if proxy != 0 {
        e.call(PROXY_SET_VALUE, &args![proxy, value]);
    }
}

// Translated from 005d3d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall`: `00ca86c0(proxy)` for the character proxy `004ae750(this)`
/// gives, or 0 when there is none.
pub fn fn_005d3d10(e: &mut Engine, this: u32) -> u32 {
    let proxy = e.call(GET_PROXY, &args![this]).u32();
    if proxy != 0 {
        e.call(PROXY_GET_OBJECT, &args![proxy]).u32()
    } else {
        0
    }
}

// Translated from 005d3d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer (default -1); unless it is -1 sets or clears the
/// form flag `0x100000` of `thisObj` (`00484990(thisObj, value != 0)`) and calls its virtual slot
/// `0x48` with 1. Always succeeds once the parameters parse (`thisObj` is not
/// checked).
pub fn fn_005d3d50(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0xffff_ffff]) else {
        return false;
    };
    if value as i32 != -1 {
        e.call(FORM_SET_FLAG, &args![a.this_obj, (value != 0) as u32]);
        e.vcall(a.this_obj.addr(), MARK_CHANGED_SLOT, &args![1u32]);
    }
    true
}

// Translated from 005d3dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIgnoreFriendlyHitsFunction` (Xbox PDB): the condition function
/// `005a3760(thisObj, 0, 0, result)`; with the TLS echo flag set the console
/// says "Actor counts friendly hits" when the result is 0.0 and "Actor
/// ignores friendly hits" otherwise. Returns the condition function's `AL`.
pub fn script_get_ignore_friendly_hits_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let verdict = call_condition(e, GET_IGNORE_FRIENDLY_HITS_CONDITION, a, 0);
    if echo_enabled(e) {
        let text = if e.mem.f64(a.result.addr()) == 0.0 {
            TEXT_COUNTS_FRIENDLY_HITS
        } else {
            TEXT_IGNORES_FRIENDLY_HITS
        };
        console_print(e, &args![FORMAT_STRING, text]);
    }
    verdict
}

// Translated from 005d3e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetItemValueFunction` (Xbox PDB): parses an integer; when
/// `thisObj`'s base form casts from `TESObject` to `TESValueForm`, sets its
/// value (`0048e960`), echoes "<name> has been set to a VALUE of <value>"
/// and calls `00730690(0)`. Always succeeds once the parameters parse.
pub fn script_set_item_value_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        let base_form = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
        if base_form != 0 {
            let value_form = e
                .call(
                    DYNAMIC_CAST,
                    &args![base_form, 0u32, RTTI_TES_OBJECT, RTTI_TES_VALUE_FORM, 0u32],
                )
                .u32();
            if value_form != 0 {
                e.call(VALUE_FORM_SET_VALUE, &args![value_form, value]);
                if echo_enabled(e) {
                    let name = e.call(GET_REFERENCE_NAME, &args![a.this_obj]).u32();
                    console_print(e, &args![MSG_SET_ITEM_VALUE, name, value]);
                }
                e.call(REFRESH_MENU, &args![0u32]);
            }
        }
    }
    true
}

// Translated from 005d3f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the refraction of an actor: parses a `float`, takes `thisObj` (the
/// player when none), limits the value to `0..` the `float` at `01017b78`
/// (10.0) and, for an actor with a process, hands it to slot `0x5b8` of the
/// process. When `fn_005b9b00` allows, the actor's refraction (slot `0x384`,
/// `enable, power`) is set: the code first builds two locals that are never
/// written again (0.0), so the player branch (needs the first local above 0)
/// cannot run; a positive second local (`FLOAT_MIN(1.0, FLOAT_MAX(0.0,
/// 0.0))`, which two correct limit functions make 0.0) would blend the two
/// game settings `0040df50`/`0040df70` through `004b3ab0`; a positive value
/// sets `(1, value)`; otherwise `(0, 0.0)` and `Actor::UpdateAlpha`. With the
/// TLS echo flag set the console says "<name> refraction has been set to
/// <value>". Always succeeds once the parameters parse.
pub fn fn_005d3f10(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0.0f32.to_bits()]) else {
        return false;
    };
    let mut this_obj = a.this_obj;
    if this_obj.is_null() {
        this_obj = player(e);
    }
    let limit = e.global::<f32>(REFRACTION_LIMIT);
    let smaller = e
        .call(FLOAT_MIN, &args![f32::from_bits(value), limit])
        .f32();
    let value = e.call(FLOAT_MAX, &args![smaller, 0.0f32]).f32();
    let actor = actor_of(e, this_obj.addr());
    if actor == 0 || e.call(GET_PROCESS, &args![actor]).u32() == 0 {
        return true;
    }
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    e.vcall(process, PROCESS_SET_REFRACTION_SLOT, &args![value]);
    if fn_005b9b00(e) {
        // The two locals of the code; neither is written again.
        let first_local = 0.0f32;
        let second_local = limit_to(e, 0.0, 1.0);
        let zero = e.global::<f64>(DOUBLE_ZERO);
        if f64::from(first_local) > zero && actor == player(e).addr() {
            e.vcall(actor, ACTOR_SLOT_41C, &args![1.0f32]);
            let power = e.global::<f32>(PLAYER_REFRACTION);
            e.vcall(actor, ACTOR_SET_REFRACTION_SLOT, &args![1u32, power]);
        } else if f64::from(second_local) > zero {
            e.vcall(actor, ACTOR_SLOT_41C, &args![1.0f32]);
            let hundred = e.global::<f64>(DOUBLE_HUNDRED);
            let blend = (1.0 - f64::from(second_local) / hundred) as f32;
            let first = e.call(SETTING_FLOAT_A, &args![]).f32();
            let second = e.call(SETTING_FLOAT_B, &args![]).f32();
            let power = e
                .call(LERP, &args![second, first, 0.0f32, 1.0f32, blend])
                .f32();
            e.vcall(actor, ACTOR_SET_REFRACTION_SLOT, &args![1u32, power]);
        } else if f64::from(value) > zero {
            e.vcall(actor, ACTOR_SET_REFRACTION_SLOT, &args![1u32, value]);
        } else {
            e.vcall(actor, ACTOR_SET_REFRACTION_SLOT, &args![0u32, 0.0f32]);
            e.call(ACTOR_UPDATE_ALPHA, &args![actor]);
        }
    }
    if echo_enabled(e) {
        let name = e.call(GET_REFERENCE_NAME, &args![this_obj]).u32();
        console_print(e, &args![MSG_REFRACTION_SET, name, f64::from(value)]);
    }
    true
}

// Translated from 005d4190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::RunCellTest` (Xbox PDB): parses an integer and calls
/// `TES::RunCellTest` on the TES singleton with it.
pub fn script_run_cell_test(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([argument]) = parse_params(e, a, [0]) else {
        return false;
    };
    let tes = e.global::<u32>(TES_SINGLETON);
    e.call(TES_RUN_CELL_TEST, &args![tes, argument]);
    true
}

// Translated from 005d41f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::StartAllQuests` (Xbox PDB): walks the data handler's quest list
/// (the nodes, up to the first empty node) and calls `0060c9c0(quest, 1)` on
/// every quest; with the TLS echo flag set prints "All Quests Enabled." with
/// the result double. Always succeeds.
pub fn script_start_all_quests(e: &mut Engine, a: ScriptArgs) -> bool {
    let handler = e.global::<u32>(DATA_HANDLER);
    let mut node = e.call(DATA_HANDLER_QUEST_LIST, &args![handler]).u32();
    while node != 0 {
        if e.call(NODE_IS_EMPTY, &args![node]).bool() {
            break;
        }
        let slot = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let quest = e.mem.u32(slot);
        e.call(QUEST_SET_ENABLED, &args![quest, 1u32]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if echo_enabled(e) {
        let value = e.mem.f64(a.result.addr());
        console_print(e, &args![MSG_ALL_QUESTS_ENABLED, value]);
    }
    true
}

// Translated from 005d4280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::CompleteAllQuestStages` (Xbox PDB): with script processing
/// switched off for the duration (`GetProcessScripts`, `SetProcessScripts(0)`
/// ... `SetProcessScripts(old)`), walks the data handler's quest list; for
/// every quest walks the list at `quest + 0x44` calling `0060f4a0(stage,
/// quest, 1)` and the list at `quest + 0x4c` calling `005ec5d0(target, 1)`
/// and `005ec5d0(target, 3)`. With the TLS echo flag set prints "All Quest
/// Stages Completed." with the result double. Always succeeds.
pub fn script_complete_all_quest_stages(e: &mut Engine, a: ScriptArgs) -> bool {
    let processing = e.call(GET_PROCESS_SCRIPTS, &args![]).u8();
    e.call(SET_PROCESS_SCRIPTS, &args![0u32]);
    let handler = e.global::<u32>(DATA_HANDLER);
    let mut quest_node = e.call(DATA_HANDLER_QUEST_LIST, &args![handler]).u32();
    while quest_node != 0 {
        if e.call(NODE_IS_EMPTY, &args![quest_node]).bool() {
            break;
        }
        let slot = e.call(NODE_ITEM_ADDRESS, &args![quest_node]).u32();
        let quest = e.mem.u32(slot);
        let mut stage_node = fn_005d43c0(e, quest);
        while stage_node != 0 {
            if e.call(NODE_IS_EMPTY, &args![stage_node]).bool() {
                break;
            }
            let slot = e.call(NODE_ITEM_ADDRESS, &args![stage_node]).u32();
            let stage = e.mem.u32(slot);
            e.call(QUEST_STAGE_SET_DONE, &args![stage, quest, 1u32]);
            stage_node = e.call(NODE_NEXT, &args![stage_node]).u32();
        }
        let mut target_node = fn_005d43e0(e, quest);
        while target_node != 0 {
            if e.call(NODE_IS_EMPTY, &args![target_node]).bool() {
                break;
            }
            let slot = e.call(NODE_ITEM_ADDRESS, &args![target_node]).u32();
            let target = e.mem.u32(slot);
            e.call(QUEST_TARGET_SET_STATE, &args![target, 1u32]);
            e.call(QUEST_TARGET_SET_STATE, &args![target, 3u32]);
            target_node = e.call(NODE_NEXT, &args![target_node]).u32();
        }
        quest_node = e.call(NODE_NEXT, &args![quest_node]).u32();
    }
    e.call(SET_PROCESS_SCRIPTS, &args![processing as u32]);
    if echo_enabled(e) {
        let value = e.mem.f64(a.result.addr());
        console_print(e, &args![MSG_ALL_STAGES_COMPLETED, value]);
    }
    true
}

// ---- The functions from 005d43c0 on --------------------------------------------

/// The commands that call a condition function with no argument, echo the
/// result double with `format` when the TLS flag is set and always return
/// true (unlike [`condition_with_echo`], which returns the condition
/// function's `AL`).
fn condition_echo_then_true(e: &mut Engine, a: ScriptArgs, function: u32, format: u32) -> bool {
    condition_with_echo(e, a, function, format);
    true
}

// Translated from 005d43c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fastcall` (`this`): `this + 0x44`. On a reference this is the address of
/// its embedded extra data list; on a quest, the node of its first list (the
/// stages).
pub fn fn_005d43c0(_e: &mut Engine, this: u32) -> u32 {
    this.wrapping_add(0x44)
}

// Translated from 005d43e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fastcall` (`this`): `this + 0x4c`. On a quest this is the node of its
/// second list (the targets).
pub fn fn_005d43e0(_e: &mut Engine, this: u32) -> u32 {
    this.wrapping_add(0x4c)
}

// Translated from 005d4400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::FlushPersistantActors` (Xbox PDB): parses an integer and calls
/// `ProcessLists::FlushNonPersistentActors` on the process lists with it;
/// with the TLS echo flag set prints that all non persistent actors in the
/// high process were deleted. Fails when the parameters do not parse.
pub fn script_flush_persistant_actors(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([argument]) = parse_params(e, a, [0]) else {
        return false;
    };
    e.call(
        PROCESS_LISTS_FLUSH_NON_PERSISTENT_ACTORS,
        &args![PROCESS_LISTS, argument],
    );
    if echo_enabled(e) {
        console_print(e, &args![MSG_FLUSHED_ACTORS]);
    }
    true
}

// Translated from 005d4480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::DeleteReference` (Xbox PDB): with the TLS echo flag set prints
/// "No reference to flush" or "Deleting reference <name>". With a `thisObj`
/// calls its virtual slot `0x10` with 1 and returns false; without one
/// returns true.
pub fn script_delete_reference(e: &mut Engine, a: ScriptArgs) -> bool {
    let reference = a.this_obj;
    if echo_enabled(e) {
        if reference.is_null() {
            console_print(e, &args![MSG_NO_REFERENCE_TO_FLUSH]);
        } else {
            let name = e.call(GET_REFERENCE_NAME, &args![reference]).u32();
            console_print(e, &args![MSG_DELETING_REFERENCE, name]);
        }
    }
    if reference.is_null() {
        return true;
    }
    e.vcall(reference.addr(), REFR_DELETE_SLOT, &args![1u32]);
    false
}

// Translated from 005d4510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleFogOfWar` (Xbox PDB): reads whether the fog of war is on
/// (`007043d0`) and sets the opposite (`00704420`); with the TLS echo flag
/// set prints "Fog of war - ENABLED." or "DISABLED." according to a second
/// read. Always succeeds.
pub fn script_toggle_fog_of_war(e: &mut Engine, _args: ScriptArgs) -> bool {
    let enabled = e.call(FOG_OF_WAR_IS_ENABLED, &args![]).bool();
    e.call(FOG_OF_WAR_SET, &args![!enabled]);
    if echo_enabled(e) {
        let enabled = e.call(FOG_OF_WAR_IS_ENABLED, &args![]).bool();
        let text = if enabled { TEXT_ENABLED } else { TEXT_DISABLED };
        console_print(e, &args![MSG_FOG_OF_WAR, text]);
    }
    true
}

// Translated from 005d4580 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command without parameters: sets the byte `012004fc`, calls
/// [`fn_005d45c0`], calls [`fn_005d45e0`] 25 times, clears the byte and
/// succeeds.
pub fn fn_005d4580(e: &mut Engine, _args: ScriptArgs) -> bool {
    e.mem.set_u8(MAP_MENU_BUSY_FLAG, 1);
    fn_005d45c0(e);
    for _ in 0..0x19 {
        fn_005d45e0(e);
    }
    e.mem.set_u8(MAP_MENU_BUSY_FLAG, 0);
    true
}

// Translated from 005d45c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object at `011da368` exists, clears its word at `+0x128`.
pub fn fn_005d45c0(e: &mut Engine) {
    let object = e.global::<u32>(MAP_MENU);
    if object != 0 {
        e.mem.set_u32(object + 0x128, 0);
    }
}

// Translated from 005d45e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object at `011da368` exists, calls `0079ffb0(1)`.
pub fn fn_005d45e0(e: &mut Engine) {
    if e.global::<u32>(MAP_MENU) != 0 {
        e.call(MAP_MENU_FUNCTION_0079FFB0, &args![1u32]);
    }
}

// Translated from 005d4600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a `float` (default 0.0) and hands it to `004dc2d0`. Fails when the
/// parameters do not parse.
pub fn fn_005d4600(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    e.call(RENDERER_SET_VALUE, &args![value]);
    true
}

// Translated from 005d4660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names it `Script::SetDepthOfField`, but the body sets the
/// maximum anisotropy: parses a `float`, truncates it (`_ftol2`) and maps it
/// to a level (below 2: 1, below 4: 2, below 8: 4, below 12: 8, below 16: 12,
/// otherwise 16). With the TLS echo flag set prints "Maxaniso set to %d".
/// The level goes to the integer at `011a9608`.
pub fn fn_005d4660(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    let requested = float_to_int(e, f32::from_bits(value)) as i32;
    let level: u32 = if requested < 2 {
        1
    } else if requested < 4 {
        2
    } else if requested < 8 {
        4
    } else if requested < 12 {
        8
    } else if requested < 16 {
        12
    } else {
        16
    };
    if echo_enabled(e) {
        console_print(e, &args![MSG_MAX_ANISO, level]);
    }
    e.mem.set_u32(MAX_ANISOTROPY, level);
    true
}

// Translated from 005d4740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a `float` (default 0.0) and does nothing with it: the code
/// compares it with 0 and, when it is positive, converts it to an integer
/// shifted left by 20, but only into locals that are never read. Fails when
/// the parameters do not parse.
pub fn fn_005d4740(e: &mut Engine, a: ScriptArgs) -> bool {
    parse_params(e, a, [0]).is_some()
}

// Translated from 005d47c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command without parameters: when the byte `011f941e` is set calls
/// `ImageSpaceEffectHDR::ClearAdaptedLight`. Always succeeds.
pub fn fn_005d47c0(e: &mut Engine, _args: ScriptArgs) -> bool {
    if e.mem.u8(HDR_ACTIVE_FLAG) != 0 {
        e.call(IMAGE_SPACE_HDR_CLEAR_ADAPTED_LIGHT, &args![]);
    }
    true
}

// Translated from 005d47e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one value; when it is not zero and `thisObj` is an actor calls
/// `008c0ec0` on the parsed value with the actor, the actor's base form and
/// -1. Succeeds whenever the parameters parse.
pub fn fn_005d47e0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([target]) = parse_params(e, a, [0]) else {
        return false;
    };
    let actor = actor_of(e, a.this_obj.addr());
    if target != 0 && actor != 0 {
        let form = e.call(GET_BASE_FORM, &args![actor]).u32();
        e.call(
            ACTOR_UNIT_METHOD_008C0EC0,
            &args![target, actor, form, 0xffff_ffffu32],
        );
    }
    true
}

// Translated from 005d4870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleCastShadows` (Xbox PDB): parses a shadow object type; outside
/// 0 to 6 prints (with the TLS echo flag) that it must be in [0,6]. Otherwise
/// flips that bit of the mask at `011ad82c` and, with the echo flag set, prints
/// "Shadows <category>: On/Off". Succeeds whenever the parameters parse.
pub fn script_toggle_cast_shadows(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([category]) = parse_params(e, a, [0]) else {
        return false;
    };
    let category = category as i32;
    if !(0..=6).contains(&category) {
        if echo_enabled(e) {
            console_print(e, &args![MSG_SHADOW_TYPE_RANGE]);
        }
        return true;
    }
    let bit = 1u32 << category;
    let mask = e.mem.u32(SHADOW_CATEGORY_MASK);
    let toggled = if mask & bit != 0 {
        mask & !bit
    } else {
        mask | bit
    };
    e.mem.set_u32(SHADOW_CATEGORY_MASK, toggled);
    if echo_enabled(e) {
        let name = SHADOW_CATEGORY_NAMES[category as usize];
        let state = if e.mem.u32(SHADOW_CATEGORY_MASK) & bit != 0 {
            TEXT_ON
        } else {
            TEXT_OFF
        };
        console_print(e, &args![MSG_SHADOWS, name, state]);
    }
    true
}

// Translated from 005d49c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer; [`fn_005d4a30`] gets whether it is non-zero and so does
/// the byte `012682f8`. Fails when the parameters do not parse.
pub fn fn_005d49c0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    let flag = (value != 0) as u8;
    fn_005d4a30(e, flag);
    e.mem.set_u8(SWITCH_FLAG_005D49C0, flag);
    true
}

// Translated from 005d4a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` (`byte`): stores the byte at `011dea2a`.
pub fn fn_005d4a30(e: &mut Engine, value: u8) {
    e.mem.set_u8(SAVED_FLAG_005D4A30, value);
}

// Translated from 005d4a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 1 in `AL`.
pub fn fn_005d4a40(_e: &mut Engine) -> u8 {
    1
}

// Translated from 005d4a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsEssentialFunction` (Xbox PDB): calls the `IsEssential` condition
/// function on `thisObj`, echoes the result double with the TLS echo flag
/// ("IsEssential >> %0.2f") and always succeeds.
pub fn script_is_essential_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_echo_then_true(e, a, IS_ESSENTIAL_CONDITION, MSG_IS_ESSENTIAL)
}

// Translated from 005d4aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsActorFunction` (Xbox PDB): like [`script_is_essential_function`]
/// with the `IsActor` condition function ("IsActor >> %0.2f").
pub fn script_is_actor_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_echo_then_true(e, a, IS_ACTOR_CONDITION, MSG_IS_ACTOR)
}

// Translated from 005d4af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPlayerMovingIntoNewSpaceFunction` (Xbox PDB): like
/// [`script_is_essential_function`] with its condition function ("Player is
/// moving to new area >> %0.2f").
pub fn script_is_player_moving_into_new_space_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_echo_then_true(
        e,
        a,
        IS_PLAYER_MOVING_INTO_NEW_SPACE_CONDITION,
        MSG_PLAYER_MOVING_TO_NEW_AREA,
    )
}

// Translated from 005d4b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetTimeDeadFunction` (Xbox PDB): like [`script_is_essential_function`]
/// with its condition function ("time dead >> %0.2f").
pub fn script_get_time_dead_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_echo_then_true(e, a, GET_TIME_DEAD_CONDITION, MSG_TIME_DEAD)
}

// Translated from 005d4b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a `float`; fails without a `thisObj`. When the integer game setting
/// `011c3ea4` is at most 1 the value goes to [`fn_005d4c20`]; otherwise the
/// call `0087a8b0(owner, 0x116c, thisObj, value as a double)` is made, `owner`
/// being what `004537b0` returns (the same shape as `005d3b90`, with another
/// event code).
pub fn fn_005d4b90(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    if a.this_obj.is_null() {
        return false;
    }
    let value = f32::from_bits(value);
    let setting = e.call(GET_SETTING_INTEGER, &args![EVENT_SETTING]).u32();
    if e.mem.i32(setting) > 1 {
        let owner = e.call(GET_QUEUE_OWNER, &args![]).u32();
        e.call(
            SEND_REFERENCE_EVENT,
            &args![owner, EVENT_CODE_116C, a.this_obj, f64::from(value)],
        );
    } else {
        fn_005d4c20(e, a.this_obj, value);
    }
    true
}

// Translated from 005d4c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` (`reference, float`): calls virtual slot `0x144` of the reference
/// with the `float` and 1.
pub fn fn_005d4c20(e: &mut Engine, reference: Ptr, value: f32) {
    e.vcall(reference.addr(), REFR_SLOT_144, &args![value, 1u32]);
}

// Translated from 005d4c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command without parameters that returns the `AL` of condition function
/// `005a3af0` called on `thisObj`.
pub fn fn_005d4c40(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, CONDITION_005A3AF0, a, 0)
}

// Translated from 005d4c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPlayerActionActiveFunction` (Xbox PDB): parses an integer and
/// returns the `AL` of the `IsPlayerActionActive` condition function with it.
pub fn script_is_player_action_active_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([action]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, IS_PLAYER_ACTION_ACTIVE_CONDITION, a, action)
}

// Translated from 005d4cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one value (whether it parsed is not checked); with a `thisObj`
/// whose base form has the form type byte `0x16`, calls `004ff0e0` on that
/// base form with the value. Always succeeds.
pub fn fn_005d4cc0(e: &mut Engine, a: ScriptArgs) -> bool {
    let value = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        parse(e, a, &[slot.addr()]);
        e.mem.u32(slot.addr())
    });
    if !a.this_obj.is_null() {
        let form = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
        if e.call(FORM_TYPE_BYTE, &args![form]).u8() as u32 == FORM_TYPE_CHECKED {
            let form = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
            e.call(FORM_TYPE_CHECKED_METHOD, &args![form, value]);
        }
    }
    true
}

// Translated from 005d4d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer and returns the `AL` of condition function `005a3b90`
/// called with it.
pub fn fn_005d4d30(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([argument]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, CONDITION_005A3B90, a, argument)
}

// Translated from 005d4d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::AddPerkFunction` (Xbox PDB): parses a perk and a flag. When the
/// perk is not null and `thisObj` is an actor (virtual slot `0x100`): reads
/// the actor's rank of the perk (slot `0x4a0`); when it is below the perk's
/// highest rank ([`fn_005d4ee0`]) sets the rank plus one (slot `0x498`), with
/// the TLS echo flag prints "Added Perk %s to %s with rank %i" and stores the
/// new rank in the result double; otherwise stores the current rank.
/// Succeeds whenever the parameters parse.
pub fn script_add_perk_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([perk, flag]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    let flag = (flag != 0) as u32;
    let actor = a.this_obj.addr();
    if perk != 0 && e.vcall(actor, REFR_IS_ACTOR_SLOT, &args![]).bool() {
        let rank = e
            .vcall(actor, ACTOR_GET_PERK_RANK_SLOT, &args![perk, flag])
            .u8() as u32;
        let highest = fn_005d4ee0(e, perk) as u32;
        if highest > rank {
            e.vcall(
                actor,
                ACTOR_SET_PERK_RANK_SLOT,
                &args![perk, rank + 1, flag],
            );
            if echo_enabled(e) {
                let reference_name = e.call(GET_REFERENCE_NAME, &args![actor]).u32();
                // The perk's name component, `perk + 0x18`.
                let perk_name = e.call(GET_NAME_TEXT, &args![perk + 0x18]).u32();
                console_print(
                    e,
                    &args![MSG_ADDED_PERK, perk_name, reference_name, rank + 1],
                );
            }
            e.mem.set_f64(a.result.addr(), f64::from(rank + 1));
        } else {
            e.mem.set_f64(a.result.addr(), f64::from(rank));
        }
    }
    true
}

// Translated from 005d4ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fastcall` (`this`): the byte at `this + 0x3a` (the perk's highest rank).
pub fn fn_005d4ee0(e: &mut Engine, this: u32) -> u8 {
    e.mem.u8(this + PERK_MAX_RANK_OFFSET)
}

// Translated from 005d4f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a perk and a flag; when the perk is not null and `thisObj` is an
/// actor (virtual slot `0x100`) calls the actor's slot `0x49c` with them.
/// Succeeds whenever the parameters parse.
pub fn fn_005d4f00(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([perk, flag]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    let flag = (flag != 0) as u32;
    let actor = a.this_obj.addr();
    if perk != 0 && e.vcall(actor, REFR_IS_ACTOR_SLOT, &args![]).bool() {
        e.vcall(actor, ACTOR_PERK_SLOT_49C, &args![perk, flag]);
    }
    true
}

// Translated from 005d4fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::HasPerkFunction` (Xbox PDB): parses a perk and a rank and returns
/// the `AL` of the `HasPerk` condition function with them.
pub fn script_has_perk_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([perk, rank]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    e.call(HAS_PERK_CONDITION, &args![a.this_obj, perk, rank, a.result])
        .bool()
}

// Translated from 005d5020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SendSherlockDebugPage` (Xbox PDB): clears a 0x800-byte buffer and
/// parses a string into it. When debug text is not visible and the string is
/// empty prints that a page name is needed. Otherwise the page name is the
/// buffer, or, when debug text is visible and the string is empty, the slot
/// [`fn_005d5120`] gives for the debug text table. With the player standing in
/// for a missing `thisObj`, calls `00802a00` on the table with the page name
/// and the reference. Always succeeds (even when the parameters do not
/// parse).
pub fn script_send_sherlock_debug_page(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(0x800, |e, buffer| {
        let buffer = buffer.addr();
        e.mem.set_u32(buffer, 0);
        e.call(MEMSET, &args![buffer + 4, 0u32, 0x7fcu32]);
        if parse(e, a, &[buffer]) {
            let visible = e.call(INTERFACE_IS_DEBUG_TEXT_VISIBLE, &args![]).bool();
            let has_name = e.mem.u32(buffer) != 0;
            if !visible && !has_name {
                console_print(e, &args![MSG_DEBUG_PAGE_NAME_REQUIRED]);
            } else {
                let visible = e.call(INTERFACE_IS_DEBUG_TEXT_VISIBLE, &args![]).bool();
                let page_name = if visible && !has_name {
                    let table = e.call(DEBUG_TEXT_TABLE_GETTER, &args![]).u32();
                    fn_005d5120(e, table)
                } else {
                    buffer
                };
                let reference = if a.this_obj.is_null() {
                    e.global::<u32>(PLAYER)
                } else {
                    a.this_obj.addr()
                };
                let table = e.call(DEBUG_TEXT_TABLE_GETTER, &args![]).u32();
                e.call(
                    DEBUG_TEXT_TABLE_SEND_PAGE,
                    &args![table, page_name, reference],
                );
            }
        }
    });
    true
}

// Translated from 005d5120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fastcall` (`this`): `this + 0x20 + *(this + 0x18) * 0x44`, the address of
/// the entry after the table's last (entries of 0x44 bytes from `this + 0x20`,
/// the count at `this + 0x18`).
pub fn fn_005d5120(e: &mut Engine, this: u32) -> u32 {
    let count = e.mem.u32(this + 0x18);
    this.wrapping_add(count.wrapping_mul(0x44))
        .wrapping_add(0x20)
}

// Translated from 005d5140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer and calls virtual slot `0x488` of the player with it.
/// Fails when the parameters do not parse.
pub fn fn_005d5140(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    let player = player(e);
    e.vcall(player.addr(), PLAYER_SLOT_488, &args![value]);
    true
}

// Translated from 005d51a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::RewardKarmaFunction` (Xbox PDB): parses an integer and calls
/// `PlayerCharacter::RewardKarma` on the player with it. Fails when the
/// parameters do not parse.
pub fn script_reward_karma_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([amount]) = parse_params(e, a, [0]) else {
        return false;
    };
    let player = player(e);
    e.call(PLAYER_REWARD_KARMA, &args![player, amount]);
    true
}

// Translated from 005d5200 (decompiled, FalloutNV.exe 1.4.0.525)
/// A command without parameters: when `thisObj` is an actor (virtual slot
/// `0x100`) calls `00704690(thisObj)`. Always succeeds.
pub fn fn_005d5200(e: &mut Engine, a: ScriptArgs) -> bool {
    if !a.this_obj.is_null()
        && e.vcall(a.this_obj.addr(), REFR_IS_ACTOR_SLOT, &args![])
            .bool()
    {
        e.call(INTERFACE_FUNCTION_00704690, &args![a.this_obj]);
    }
    true
}

// Translated from 005d5230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::AddNoteFunction` (Xbox PDB): parses a note; when it is not null
/// calls `PlayerCharacter::AddNote` on the player with the note and the TLS
/// echo flag byte. Fails when the parameters do not parse.
pub fn script_add_note_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([note]) = parse_params(e, a, [0]) else {
        return false;
    };
    if note != 0 {
        let tls = e.tls();
        let echo = e.mem.u8(tls + TLS_ECHO) as u32;
        let player = player(e);
        e.call(PLAYER_ADD_NOTE, &args![player, note, echo]);
    }
    true
}

// Translated from 005d52a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::RemoveNoteFunction` (Xbox PDB): parses a note; when it is not null
/// calls `00966b80` on the player with it. With the TLS echo flag set prints
/// "Removed Note %s from Player " with the text of the name component at
/// `note + 0x48` (read even for a null note, as the code does). Fails when
/// the parameters do not parse.
pub fn script_remove_note_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([note]) = parse_params(e, a, [0]) else {
        return false;
    };
    if note != 0 {
        let player = player(e);
        e.call(PLAYER_REMOVE_NOTE, &args![player, note]);
    }
    if echo_enabled(e) {
        let name = e.call(GET_NAME_TEXT, &args![note.wrapping_add(0x48)]).u32();
        console_print(e, &args![MSG_REMOVED_NOTE, name]);
    }
    true
}

// Translated from 005d5330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer and returns the `AL` of condition function `005a3bf0`
/// called with it.
pub fn fn_005d5330(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([argument]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, CONDITION_005A3BF0, a, argument)
}

// Translated from 005d5390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a value and a second value (default -1); when the first is not zero
/// and `thisObj` is an actor (virtual slot `0x100`) calls `008b8e20` on
/// `thisObj` with the two. Succeeds whenever the parameters parse.
pub fn fn_005d5390(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, second]) = parse_params(e, a, [0, 0xffff_ffff]) else {
        return false;
    };
    if first != 0
        && e.vcall(a.this_obj.addr(), REFR_IS_ACTOR_SLOT, &args![])
            .bool()
    {
        e.call(
            ACTOR_UNIT_METHOD_008B8E20,
            &args![a.this_obj, first, second],
        );
    }
    true
}

// ---- The functions from 005d5420 on ------------------------------------------

/// The name the commands print for a reference: its virtual slot `0x130` when
/// [`NAME_TEXT_LENGTH`] finds a name, otherwise the reference name
/// [`GET_REFERENCE_NAME`] gives.
fn reference_name(e: &mut Engine, object: u32) -> u32 {
    if e.call(NAME_TEXT_LENGTH, &args![object]).u32() != 0 {
        e.vcall(object, NAME_SLOT, &args![]).u32()
    } else {
        e.call(GET_REFERENCE_NAME, &args![object]).u32()
    }
}

/// The name the commands print for a form: its virtual slot `0x130` when
/// [`NAME_TEXT_LENGTH`] finds a name, otherwise [`GET_FULL_NAME`].
fn form_name(e: &mut Engine, form: u32) -> u32 {
    if e.call(NAME_TEXT_LENGTH, &args![form]).u32() != 0 {
        e.vcall(form, NAME_SLOT, &args![]).u32()
    } else {
        e.call(GET_FULL_NAME, &args![form]).u32()
    }
}

// Translated from 005d5420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one value; when it is not zero and `thisObj` is an actor, calls the
/// `actor.cpp` method `008b8f20` on the actor with it. Fails when the
/// parameters do not parse.
pub fn fn_005d5420(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    if value != 0
        && e.vcall(a.this_obj.addr(), REFR_IS_ACTOR_SLOT, &args![])
            .bool()
    {
        e.call(ACTOR_UNIT_METHOD_008B8F20, &args![a.this_obj, value]);
    }
    true
}

// Translated from 005d54a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a form and an integer. When the form is not zero and `thisObj` is
/// an actor, the actor's virtual slot `0x344` (`form, 0`) gives an integer
/// (the current value), and slot `0x460` is called with the form and the
/// difference `integer - current` as a `float`. Fails when the parameters do
/// not parse.
pub fn fn_005d54a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = if e
        .vcall(a.this_obj.addr(), REFR_IS_ACTOR_SLOT, &args![])
        .bool()
    {
        a.this_obj.addr()
    } else {
        0
    };
    let Some([form, wanted]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if form != 0 && actor != 0 {
        let current = e
            .vcall(actor, ACTOR_GET_VALUE_SLOT, &args![form, 0u32])
            .i32();
        let difference = (wanted as i32).wrapping_sub(current);
        e.vcall(actor, ACTOR_MOD_VALUE_SLOT, &args![form, difference as f32]);
    }
    true
}

// Translated from 005d5560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetGlobalTimeMultiplierFunction` (Xbox PDB): parses a `float`
/// (default 1.0) and an integer. Calls `BSTimer::SetGlobalTimeMultiplier` on
/// the timer with the multiplier and 1, then stores the flag `integer != 0` in
/// the audio manager singleton ([`fn_005d5610`]); with the TLS echo flag set
/// prints "Global Time Multiplier >> 'x'". Fails when the parameters do not
/// parse.
pub fn script_set_global_time_multiplier_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([multiplier, flag]) = parse_params(e, a, [1.0f32.to_bits(), 0]) else {
        return false;
    };
    let multiplier = f32::from_bits(multiplier);
    e.call(
        TIMER_SET_GLOBAL_TIME_MULTIPLIER,
        &args![TIMER, multiplier, 1u32],
    );
    // The game pushes the flag before the singleton getter, which takes no
    // argument: the setter below pops it.
    let audio_manager = e.call(AUDIO_MANAGER_QUERY_INSTANCE, &args![]).u32();
    fn_005d5610(e, audio_manager, (flag != 0) as u8);
    if echo_enabled(e) {
        console_print(e, &args![MSG_GLOBAL_TIME_MULTIPLIER, f64::from(multiplier)]);
    }
    true
}

// Translated from 005d5610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall` (`this, flag`): stores the byte at `this + 0x184` (the audio
/// manager's field that `SetGlobalTimeMultiplier` sets).
pub fn fn_005d5610(e: &mut Engine, this: u32, value: u8) {
    e.mem.set_u8(this.wrapping_add(0x184), value);
}

// Translated from 005d5630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetHitLocationFunction` (Xbox PDB): the condition function
/// `005a3c30(thisObj, 0, 0, result)`, echoed as "GetHitLocation >> value";
/// always true.
pub fn script_get_hit_location_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_echo_then_true(e, a, GET_HIT_LOCATION_CONDITION, MSG_GET_HIT_LOCATION)
}

// Translated from 005d5680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetLastHitCriticalFunction` (Xbox PDB): like
/// [`script_get_hit_location_function`] with the condition function
/// `005a3c90` ("GetLastHitCritical >> value").
pub fn script_get_last_hit_critical_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_echo_then_true(
        e,
        a,
        GET_LAST_HIT_CRITICAL_CONDITION,
        MSG_GET_LAST_HIT_CRITICAL,
    )
}

// Translated from 005d56d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPC1stPersonFunction` (Xbox PDB): like
/// [`script_get_hit_location_function`] with the condition function
/// `005a3d00` ("IsPC1stPerson >> value").
pub fn script_is_pc_1st_person_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_echo_then_true(e, a, IS_PC_1ST_PERSON_CONDITION, MSG_IS_PC_1ST_PERSON)
}

// Translated from 005d5720 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `thisObj` is an actor whose virtual slot `0x428` (the combat
/// controller) gives something, calls `0097fb80` (`combatcontroller.cpp`) on
/// it. Always true.
pub fn fn_005d5720(e: &mut Engine, a: ScriptArgs) -> bool {
    if e.vcall(a.this_obj.addr(), REFR_IS_ACTOR_SLOT, &args![])
        .bool()
    {
        let controller = e
            .vcall(
                a.this_obj.addr(),
                ACTOR_GET_COMBAT_CONTROLLER_SLOT,
                &args![],
            )
            .u32();
        if controller != 0 {
            e.call(COMBAT_CONTROLLER_FUNCTION_0097FB80, &args![controller]);
        }
    }
    true
}

// Translated from 005d5780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetCombatActionCostFunction` (Xbox PDB): parses a text (a buffer
/// of 0x204 bytes) and a `float` (default 0.0) and passes them to `0097abd0`.
/// With the TLS echo flag set prints "name >> value" when it answers true and
/// "name not found" when it does not. Succeeds whenever the parameters parse.
pub fn script_set_combat_action_cost_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(0x208, |e, block| {
        let buffer = block.addr();
        let value_cell = buffer + 0x204;
        e.mem.set_f32(value_cell, 0.0);
        if !parse(e, a, &[buffer, value_cell]) {
            return false;
        }
        let value = e.mem.f32(value_cell);
        let found = e.call(COMBAT_ACTION_SET_COST, &args![buffer, value]).bool();
        if echo_enabled(e) {
            if found {
                console_print(e, &args![MSG_NAME_VALUE, buffer, f64::from(value)]);
            } else {
                console_print(e, &args![MSG_NOT_FOUND, buffer]);
            }
        }
        true
    })
}

/// What a name of `ToggleCombatDebug` does.
enum CombatOption {
    /// Switches the byte of the global object `global` and prints `message`
    /// with its new state ("Off" or "On").
    Toggle { global: u32, message: u32 },
    /// Switches the byte of `011f1c30` and prints whether the debug text is
    /// now "Dark" or "Light".
    Colour,
    /// Lowers the float of `011f1c70` by the double at `0101ffa0` (0.1),
    /// back to 1.0 when it is at most 0.05, and prints it.
    Size,
}

/// The options of `ToggleCombatDebug` in the order the game compares them
/// with the text (case-insensitively): the names (the strings are in the exe's
/// data) and the action. The comment names the text at the address.
const COMBAT_OPTIONS: [(&[u32], CombatOption); 19] = [
    // "cover"
    (
        &[0x0103_cacc],
        CombatOption::Toggle {
            global: 0x011f_1680,
            message: 0x0103_ca9c,
        },
    ),
    // "covertext"
    (
        &[0x0103_ca90],
        CombatOption::Toggle {
            global: 0x011f_16c0,
            message: 0x0103_ca60,
        },
    ),
    // "cover2", "coversearch"
    (
        &[0x0103_ca58, 0x0103_ca4c],
        CombatOption::Toggle {
            global: 0x011f_1674,
            message: 0x0103_ca24,
        },
    ),
    // "cover2text", "coversearchtext"
    (
        &[0x0103_ca18, 0x0103_ca08],
        CombatOption::Toggle {
            global: 0x011f_16e0,
            message: 0x0103_c9dc,
        },
    ),
    // "cover3", "coversearch2"
    (
        &[0x0103_c9d4, 0x0103_c9c4],
        CombatOption::Toggle {
            global: 0x011f_168c,
            message: 0x0103_c990,
        },
    ),
    // "threats"
    (
        &[0x0103_c988],
        CombatOption::Toggle {
            global: 0x011f_1ba0,
            message: 0x0103_c964,
        },
    ),
    // "groups"
    (
        &[0x0103_c95c],
        CombatOption::Toggle {
            global: 0x011f_1878,
            message: 0x0103_c938,
        },
    ),
    // "groups2"
    (
        &[0x0103_c930],
        CombatOption::Toggle {
            global: 0x011f_1858,
            message: 0x0103_c90c,
        },
    ),
    // "targets"
    (
        &[0x0103_c904],
        CombatOption::Toggle {
            global: 0x011f_1b24,
            message: 0x0103_c8e0,
        },
    ),
    // "search"
    (
        &[0x0103_c8d8],
        CombatOption::Toggle {
            global: 0x011f_1814,
            message: 0x0103_c8b4,
        },
    ),
    // "guard"
    (
        &[0x0103_c8ac],
        CombatOption::Toggle {
            global: 0x011f_15b0,
            message: 0x0103_c884,
        },
    ),
    // "cluster", "clusters"
    (
        &[0x0103_c87c, 0x0103_c870],
        CombatOption::Toggle {
            global: 0x011f_1824,
            message: 0x0103_c844,
        },
    ),
    // "text"
    (
        &[0x0103_c83c],
        CombatOption::Toggle {
            global: 0x011f_15a4,
            message: 0x0103_c810,
        },
    ),
    // "color"
    (&[0x0103_c808], CombatOption::Colour),
    // "size"
    (&[0x0103_c7d0], CombatOption::Size),
    // "unreach", "unreachable"
    (
        &[0x0103_c798, 0x0103_c78c],
        CombatOption::Toggle {
            global: 0x011f_15d0,
            message: 0x0103_c758,
        },
    ),
    // "events"
    (
        &[0x0103_c750],
        CombatOption::Toggle {
            global: 0x011f_197c,
            message: 0x0103_c724,
        },
    ),
    // "plos"
    (
        &[0x0103_c71c],
        CombatOption::Toggle {
            global: 0x011f_1c4c,
            message: 0x0103_c6f0,
        },
    ),
    // "area"
    (
        &[0x0103_c6e8],
        CombatOption::Toggle {
            global: 0x011f_15ec,
            message: 0x0103_c6c8,
        },
    ),
];

/// The globals `ToggleCombatDebug` reads, in the order it reads them, to find
/// out whether any display is on (the reading stops at the first one that is).
const COMBAT_DEBUG_READ_ORDER: [u32; 17] = [
    0x011f_1680,
    0x011f_1ba0,
    0x011f_1878,
    0x011f_1858,
    0x011f_1b24,
    0x011f_1674,
    0x011f_1814,
    0x011f_168c,
    0x011f_16e0,
    0x011f_16c0,
    0x011f_15a4,
    0x011f_15b0,
    0x011f_1824,
    0x011f_15d0,
    0x011f_197c,
    0x011f_1c4c,
    0x011f_15ec,
];

/// The same globals in the order `ToggleCombatDebug` writes them when it
/// switches every display (the same set; `168c` and `1814` are swapped).
const COMBAT_DEBUG_WRITE_ORDER: [u32; 17] = [
    0x011f_1680,
    0x011f_1ba0,
    0x011f_1878,
    0x011f_1858,
    0x011f_1b24,
    0x011f_1674,
    0x011f_168c,
    0x011f_1814,
    0x011f_16e0,
    0x011f_16c0,
    0x011f_15a4,
    0x011f_15b0,
    0x011f_1824,
    0x011f_15d0,
    0x011f_197c,
    0x011f_1c4c,
    0x011f_15ec,
];

/// The global object whose byte `color` switches.
const COMBAT_TEXT_COLOUR_GLOBAL: u32 = 0x011f_1c30;
/// The global float object `size` changes.
const COMBAT_TEXT_SIZE_GLOBAL: u32 = 0x011f_1c70;
/// `double` `0.1` (the step of `size`).
const COMBAT_TEXT_SIZE_STEP: u32 = 0x0101_ffa0;
/// `double` `0.05` (the smallest size).
const COMBAT_TEXT_SIZE_MINIMUM: u32 = 0x0103_c7c8;

/// The value byte of the global object `global` ([`GLOBAL_VALUE_ADDRESS`]).
fn global_flag(e: &mut Engine, global: u32) -> bool {
    let address = e.call(GLOBAL_VALUE_ADDRESS, &args![global]).u32();
    e.mem.u8(address) != 0
}

/// "On" when the global's byte is set, otherwise "Off".
fn global_flag_text(e: &mut Engine, global: u32) -> u32 {
    if global_flag(e, global) {
        TEXT_ON
    } else {
        TEXT_OFF
    }
}

/// Reads the flag of `global`, writes its opposite through
/// [`GLOBAL_VALUE_SET`] and, with the echo flag set, prints `message` with the
/// new state.
fn toggle_combat_flag(e: &mut Engine, global: u32, message: u32) {
    let flag = global_flag(e, global);
    e.call(GLOBAL_VALUE_SET, &args![global, !flag as u32]);
    if echo_enabled(e) {
        let state = global_flag_text(e, global);
        console_print(e, &args![message, state]);
    }
}

// Translated from 005d5850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleCombatDebugFunction` (Xbox PDB): parses a text (a buffer of
/// 0x204 bytes, empty at first). With no text, switches every combat debug
/// display: when any is on they all go off, otherwise they all go on
/// ("All Combat debugging turned Off/On" with the echo flag set). With a name
/// of [`COMBAT_OPTIONS`] switches that display (`color` and `size` change the
/// debug text: dark or light, and 0.1 smaller down to 1.0 again); another name
/// is reported as an unknown option when the echo flag is set. Fails when the
/// parameters do not parse.
pub fn script_toggle_combat_debug_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(0x204, |e, block| {
        let buffer = block.addr();
        e.call(STRING_COPY, &args![buffer, EMPTY_STRING]);
        if !parse(e, a, &[buffer]) {
            return false;
        }
        if e.call(STRING_LENGTH, &args![buffer]).u32() == 0 {
            let mut any_on = false;
            for global in COMBAT_DEBUG_READ_ORDER {
                if global_flag(e, global) {
                    any_on = true;
                    break;
                }
            }
            for global in COMBAT_DEBUG_WRITE_ORDER {
                e.call(GLOBAL_VALUE_SET, &args![global, !any_on as u32]);
            }
            if echo_enabled(e) {
                let state = if any_on { TEXT_OFF } else { TEXT_ON };
                console_print(e, &args![MSG_ALL_COMBAT_DEBUGGING, state]);
            }
            return true;
        }
        for (names, option) in &COMBAT_OPTIONS {
            let mut matched = false;
            for name in *names {
                if e.call(STRICMP, &args![buffer, *name]).u32() == 0 {
                    matched = true;
                    break;
                }
            }
            if !matched {
                continue;
            }
            match option {
                CombatOption::Toggle { global, message } => {
                    toggle_combat_flag(e, *global, *message);
                }
                CombatOption::Colour => {
                    let flag = global_flag(e, COMBAT_TEXT_COLOUR_GLOBAL);
                    e.call(
                        GLOBAL_VALUE_SET,
                        &args![COMBAT_TEXT_COLOUR_GLOBAL, !flag as u32],
                    );
                    if echo_enabled(e) {
                        let text = if global_flag(e, COMBAT_TEXT_COLOUR_GLOBAL) {
                            TEXT_DARK
                        } else {
                            TEXT_LIGHT
                        };
                        console_print(e, &args![MSG_COMBAT_TEXT_COLOUR, text]);
                    }
                }
                CombatOption::Size => {
                    let value_address = e
                        .call(GLOBAL_FLOAT_ADDRESS, &args![COMBAT_TEXT_SIZE_GLOBAL])
                        .u32();
                    let step = e.global::<f64>(COMBAT_TEXT_SIZE_STEP);
                    let smaller = (f64::from(e.mem.f32(value_address)) - step) as f32;
                    e.call(GLOBAL_FLOAT_SET, &args![COMBAT_TEXT_SIZE_GLOBAL, smaller]);
                    let value_address = e
                        .call(GLOBAL_FLOAT_ADDRESS, &args![COMBAT_TEXT_SIZE_GLOBAL])
                        .u32();
                    let minimum = e.global::<f64>(COMBAT_TEXT_SIZE_MINIMUM);
                    if f64::from(e.mem.f32(value_address)) <= minimum {
                        e.call(GLOBAL_FLOAT_SET, &args![COMBAT_TEXT_SIZE_GLOBAL, 1.0f32]);
                    }
                    if echo_enabled(e) {
                        let value_address = e
                            .call(GLOBAL_FLOAT_ADDRESS, &args![COMBAT_TEXT_SIZE_GLOBAL])
                            .u32();
                        let value = e.mem.f32(value_address);
                        console_print(e, &args![MSG_COMBAT_TEXT_SIZE, f64::from(value)]);
                    }
                }
            }
            return true;
        }
        if echo_enabled(e) {
            console_print(e, &args![MSG_UNKNOWN_OPTION, buffer]);
        }
        true
    })
}

// Translated from 005d67f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ForceCombatGroupStrategyFunction` (Xbox PDB): parses a text (a
/// buffer of 0x204 bytes, empty at first). A `thisObj` that is not an actor
/// is reported ("x is not an actor"), as is an actor whose combat controller
/// (virtual slot `0x428`) is null ("x is not in combat"); a combat group of
/// fewer than two members is reported ("The combat group has only one
/// member"). Otherwise an empty text clears the group's strategy (`009862e0`
/// with null) and a known strategy name sets it; an unknown name is reported
/// with the echo flag. Fails only when the parameters do not parse.
pub fn script_force_combat_group_strategy_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(0x204, |e, block| {
        let buffer = block.addr();
        e.call(STRING_COPY, &args![buffer, EMPTY_STRING]);
        if !parse(e, a, &[buffer]) {
            return false;
        }
        let this_obj = a.this_obj.addr();
        if !e.vcall(this_obj, REFR_IS_ACTOR_SLOT, &args![]).bool() {
            let name = reference_name(e, this_obj);
            console_print(e, &args![MSG_NOT_AN_ACTOR, name]);
            return true;
        }
        let controller = e
            .vcall(this_obj, ACTOR_GET_COMBAT_CONTROLLER_SLOT, &args![])
            .u32();
        if controller == 0 {
            let name = reference_name(e, this_obj);
            console_print(e, &args![MSG_NOT_IN_COMBAT, name]);
            return true;
        }
        let group = e.call(COMBAT_GROUP_OF_CONTROLLER, &args![controller]).u32();
        if e.call(COMBAT_GROUP_GET_COUNT, &args![group]).u32() < 2 {
            console_print(e, &args![MSG_ONE_MEMBER]);
            return true;
        }
        if e.call(STRING_LENGTH, &args![buffer]).u32() == 0 {
            e.call(COMBAT_GROUP_SET_STRATEGY, &args![group, 0u32]);
            if echo_enabled(e) {
                console_print(e, &args![MSG_STRATEGY_CLEARED]);
            }
            return true;
        }
        let strategy = e.call(COMBAT_GROUP_STRATEGY_BY_NAME, &args![buffer]).u32();
        if strategy != 0 {
            e.call(COMBAT_GROUP_SET_STRATEGY, &args![group, strategy]);
            if echo_enabled(e) {
                let name = e.call(COMBAT_GROUP_STRATEGY_NAME, &args![strategy]).u32();
                console_print(e, &args![MSG_STRATEGY_SET, name]);
            }
        } else if echo_enabled(e) {
            console_print(e, &args![MSG_UNKNOWN_STRATEGY, buffer]);
        }
        true
    })
}

// Translated from 005d6a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing unless `thisObj` is an actor. Parses one value (fails when it
/// does not parse), casts `thisObj` from `TESObjectREFR` to `Actor` and, when
/// that works, stores `value != 0` in the byte at `actor + 0xbc`; with the
/// echo flag set prints "name processing is On/Off". Otherwise true.
pub fn fn_005d6a60(e: &mut Engine, a: ScriptArgs) -> bool {
    if !e
        .vcall(a.this_obj.addr(), REFR_IS_ACTOR_SLOT, &args![])
        .bool()
    {
        return true;
    }
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    let actor = actor_of(e, a.this_obj.addr());
    if actor != 0 {
        e.mem
            .set_u8(actor + ACTOR_PROCESSING_FLAG, (value != 0) as u8);
        if echo_enabled(e) {
            let state = if e.mem.u8(actor + ACTOR_PROCESSING_FLAG) != 0 {
                TEXT_ON
            } else {
                TEXT_OFF
            };
            let name = e.call(GET_REFERENCE_NAME, &args![actor]).u32();
            console_print(e, &args![MSG_PROCESSING_IS, name, state]);
        }
    }
    true
}

// Translated from 005d6b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PushActorAwayFunction` (Xbox PDB): parses an actor reference and
/// an integer. A reference that is not an actor is logged through
/// [`LOG_MESSAGE`] (with the name of the script object) when the command has
/// a script object. For an actor whose process exists and whose
/// [`PROCESS_FIELD_28`] is zero: the knockback speed
/// (`GetKnockbackSpeed(actor + 0xa4, integer)`) is computed; when `thisObj` is
/// the player the player's inventory list count and `005f5950(3, 1, form)` are
/// evaluated first (the game also pushes four words for the form getter
/// that it does not read); then the actor's process is told (virtual slot
/// `0x418`) to push the actor with the vector `thisObj`'s slot `0x1f4` gives for
/// the speed. Fails when the parameters do not parse.
pub fn script_push_actor_away_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([actor, strength]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    if !e.vcall(actor, REFR_IS_ACTOR_SLOT, &args![]).bool() {
        if !a.script_obj.is_null() {
            let script_name = e.vcall(a.script_obj.addr(), NAME_SLOT, &args![]).u32();
            e.call(LOG_MESSAGE, &args![MSG_PUSH_NON_ACTOR, script_name]);
        }
        return true;
    }
    let inside_actor = if actor == 0 { 0 } else { actor + 0xa4 };
    let speed = e
        .call(GET_KNOCKBACK_SPEED, &args![inside_actor, strength])
        .f32();
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    if process != 0 && e.call(PROCESS_FIELD_28, &args![process]).u32() == 0 {
        let player_pointer = e.global::<u32>(PLAYER);
        if a.this_obj.addr() == player_pointer {
            let player_process = e.call(GET_PROCESS, &args![player_pointer]).u32();
            let list = e
                .vcall(player_process, PROCESS_GET_LIST_SLOT, &args![])
                .u32();
            let count = if list != 0 {
                e.call(FIELD_AT_8, &args![list]).u32()
            } else {
                0
            };
            // 005d6c73..005d6c8d: the count, 0xffff, 2 and 0 are pushed before the getter
            // (`007af430`, `this` only); they are the last four words of the cdecl
            // `005f5950` (ADD ESP,0x1c).
            let form = e.call(GET_BASE_FORM, &args![actor]).u32();
            e.call(
                FUNCTION_005F5950,
                &args![3u32, 1u32, form, count, 0xffffu32, 2u32, 0u32],
            );
        }
        // 005d6caa: the reference's +0x1f4 takes no stack word (plain RET in every
        // reference class); the speed pushed before it is the last word of the
        // process' +0x418 (RET 0x14).
        let vector = e
            .vcall(a.this_obj.addr(), REFR_GET_VECTOR_SLOT, &args![])
            .u32();
        let (x, y, z) = (
            e.mem.u32(vector),
            e.mem.u32(vector + 4),
            e.mem.u32(vector + 8),
        );
        e.vcall(process, PROCESS_SLOT_418, &args![actor, x, y, z, speed]);
    }
    true
}

// Translated from 005d6d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a `thisObj`: removes the ownership from the extra data list at
/// `thisObj + 0x44` and calls the object's virtual slot `0x48` with `0x40`
/// (mark changed). Always true.
pub fn fn_005d6d10(e: &mut Engine, a: ScriptArgs) -> bool {
    if !a.this_obj.is_null() {
        let list = fn_005d43c0(e, a.this_obj.addr());
        e.call(EXTRA_DATA_LIST_REMOVE_OWNERSHIP, &args![list]);
        e.vcall(a.this_obj.addr(), MARK_CHANGED_SLOT, &args![0x40u32]);
    }
    true
}

// Translated from 005d6d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowInventoryFunction` (Xbox PDB): lists the inventory of
/// `thisObj` on the console. Does nothing without a `thisObj` or when the
/// inventory count (`00575590(0, 1)`) is not above zero. Prints "name (id) has
/// n items:", then for each entry that has a form its count, name, id and, by
/// form type, the health, the worn text and (for actors) a "can not equip"
/// text, and for a form with a biped model list the list's name and members.
/// Each entry is released. Always true.
pub fn script_show_inventory_function(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return true;
    }
    let this_obj = a.this_obj.addr();
    let actor = if e.vcall(this_obj, REFR_IS_ACTOR_SLOT, &args![]).bool() {
        this_obj
    } else {
        0
    };
    let count = e
        .call(GET_INVENTORY_COUNT, &args![this_obj, 0u32, 1u32])
        .i32();
    if count <= 0 {
        return true;
    }
    let name = reference_name(e, this_obj);
    let id = e.call(GET_FORM_ID, &args![this_obj]).u32();
    console_print(e, &args![MSG_HAS_ITEMS, name, id, count]);
    for index in 0..count as u32 {
        let item = e
            .call(GET_INVENTORY_ENTRY, &args![this_obj, index, 0u32])
            .u32();
        if item != 0 && e.call(FIELD_AT_8, &args![item]).u32() != 0 {
            let form = e.call(FIELD_AT_8, &args![item]).u32();
            let form_type = e.call(FORM_TYPE_BYTE, &args![form]).u32();
            if form_type == FORM_TYPE_WITH_EQUIP_TEXT {
                let cannot_equip =
                    if actor != 0 && !e.call(ACTOR_CAN_EQUIP, &args![actor, form, 1u32]).bool() {
                        TEXT_CANNOT_EQUIP
                    } else {
                        EMPTY_STRING
                    };
                let worn = worn_text(e, item);
                let name = form_name(e, form);
                let percent = e
                    .call(ITEM_CHANGE_GET_ITEM_HEALTH, &args![item, 1u32])
                    .f32();
                let health = e
                    .call(ITEM_CHANGE_GET_ITEM_HEALTH, &args![item, 0u32])
                    .f32();
                let id = e.call(GET_FORM_ID, &args![form]).u32();
                let number = e.call(NODE_NEXT, &args![item]).u32();
                console_print(
                    e,
                    &args![
                        MSG_ITEM_WITH_EQUIP,
                        number,
                        name,
                        id,
                        f64::from(health),
                        f64::from(percent),
                        worn,
                        cannot_equip
                    ],
                );
            } else if form_type == FORM_TYPE_WITH_HEALTH {
                let worn = worn_text(e, item);
                let name = form_name(e, form);
                let percent = e
                    .call(ITEM_CHANGE_GET_ITEM_HEALTH, &args![item, 1u32])
                    .f32();
                let health = e
                    .call(ITEM_CHANGE_GET_ITEM_HEALTH, &args![item, 0u32])
                    .f32();
                let id = e.call(GET_FORM_ID, &args![form]).u32();
                let number = e.call(NODE_NEXT, &args![item]).u32();
                console_print(
                    e,
                    &args![
                        MSG_ITEM_WITH_HEALTH,
                        number,
                        name,
                        id,
                        f64::from(health),
                        f64::from(percent),
                        worn
                    ],
                );
            } else {
                let worn = worn_text(e, item);
                let name = form_name(e, form);
                let id = e.call(GET_FORM_ID, &args![form]).u32();
                let number = e.call(NODE_NEXT, &args![item]).u32();
                console_print(e, &args![MSG_ITEM_PLAIN, number, name, id, worn]);
            }
            show_biped_model_list(e, form);
        }
        if item != 0 {
            e.call(INVENTORY_ENTRY_RELEASE, &args![item, 1u32]);
        }
    }
    true
}

/// " - Worn" when [`ITEM_CHANGE_GET_WORN`] says the entry is worn, otherwise
/// the empty string.
fn worn_text(e: &mut Engine, item: u32) -> u32 {
    if e.call(ITEM_CHANGE_GET_WORN, &args![item, 0u32]).bool() {
        TEXT_WORN
    } else {
        EMPTY_STRING
    }
}

/// The part of `ShowInventory` for a form with a biped model list: the
/// header with the list's form and one line per member.
fn show_biped_model_list(e: &mut Engine, form: u32) {
    let biped_list = e.call(GET_FORM_AS_BIPED_MODEL_LIST, &args![form]).u32();
    if biped_list == 0 {
        return;
    }
    let owner = e.call(NODE_NEXT, &args![biped_list]).u32();
    if owner == 0 {
        return;
    }
    let mut node = e.call(FIELD_ADDRESS_18, &args![owner]).u32();
    let name = form_name(e, owner);
    let id = e.call(GET_FORM_ID, &args![owner]).u32();
    console_print(e, &args![MSG_BIPED_LIST_HEADER, name, id]);
    while node != 0 {
        let slot = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let member = e.mem.u32(slot);
        let name = form_name(e, member);
        let id = e.call(GET_FORM_ID, &args![member]).u32();
        console_print(e, &args![MSG_BIPED_LIST_ENTRY, name, id]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// The commands that parse up to three integers and open the menu
/// [`OPEN_CHARGEN_MENU`] (`005d7150`, `005d71f0`): when the first is above
/// zero the menu gets mode 0 and that value, otherwise when the third is above
/// zero mode 1 and the third; `flag` is the third word of the call.
fn open_chargen_with_two_modes(e: &mut Engine, a: ScriptArgs, flag: u32) -> bool {
    let Some([first, _second, third]) = parse_params(e, a, [0, 0, 0]) else {
        return false;
    };
    if (first as i32) > 0 {
        e.call(OPEN_CHARGEN_MENU, &args![0u32, first, flag, 1u32]);
    } else if (third as i32) > 0 {
        e.call(OPEN_CHARGEN_MENU, &args![1u32, third, flag, 1u32]);
    }
    true
}

// Translated from 005d7150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses three integers (the second is not used). The first above zero opens
/// the menu `007062a0(0, first, 0, 1)`; otherwise the third above zero opens
/// `007062a0(1, third, 0, 1)`. Fails when the parameters do not parse.
pub fn fn_005d7150(e: &mut Engine, a: ScriptArgs) -> bool {
    open_chargen_with_two_modes(e, a, 0)
}

// Translated from 005d71f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_005d7150`], with 1 for the third word of the menu call.
pub fn fn_005d71f0(e: &mut Engine, a: ScriptArgs) -> bool {
    open_chargen_with_two_modes(e, a, 1)
}

// Translated from 005d7290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer and opens the menu `007062a0(0, value, 0, 1)`. Fails
/// when the parameters do not parse.
pub fn fn_005d7290(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    e.call(OPEN_CHARGEN_MENU, &args![0u32, value, 0u32, 1u32]);
    true
}

// Translated from 005d72f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_005d7290`] with `007062a0(0, value, 1, 1)`.
pub fn fn_005d72f0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    e.call(OPEN_CHARGEN_MENU, &args![0u32, value, 1u32, 1u32]);
    true
}

// Translated from 005d7350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses two integers (the second starts at 1) and opens the menu
/// `007062a0(1, first, 0, second == 1)`. Fails when the parameters do not
/// parse.
pub fn fn_005d7350(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value, mode]) = parse_params(e, a, [0, 1]) else {
        return false;
    };
    e.call(
        OPEN_CHARGEN_MENU,
        &args![1u32, value, 0u32, (mode == 1) as u32],
    );
    true
}

// Translated from 005d73d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer and opens the menu `007062a0(1, value, 1, 1)`. Fails
/// when the parameters do not parse.
pub fn fn_005d73d0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [0]) else {
        return false;
    };
    e.call(OPEN_CHARGEN_MENU, &args![1u32, value, 1u32, 1u32]);
    true
}

/// The commands that parse one integer (default 3; whether it parsed is not
/// checked) and call `00705830(0, value - 1, flag)`.
fn interface_function_with_default(e: &mut Engine, a: ScriptArgs, flag: u32) -> bool {
    let value = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 3);
        parse(e, a, &[slot.addr()]);
        e.mem.u32(slot.addr())
    });
    e.call(
        INTERFACE_FUNCTION_00705830,
        &args![0u32, value.wrapping_sub(1), flag],
    );
    true
}

// Translated from 005d7430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer (default 3; the result of the parse is not checked) and
/// calls `00705830(0, value - 1, 0)`. Always true.
pub fn fn_005d7430(e: &mut Engine, a: ScriptArgs) -> bool {
    interface_function_with_default(e, a, 0)
}

// Translated from 005d7480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_005d7430`] with `00705830(0, value - 1, 1)`.
pub fn fn_005d7480(e: &mut Engine, a: ScriptArgs) -> bool {
    interface_function_with_default(e, a, 1)
}

// Translated from 005d74d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `Error` (`0040fbe0`, which does nothing in this build) with no
/// arguments and returns true.
pub fn fn_005d74d0(e: &mut Engine, _args: ScriptArgs) -> bool {
    e.call(ERROR_REPORT, &args![]);
    true
}

/// The commands that parse one integer and return the `AL` of a condition
/// function called with `thisObj`, the integer, 0 and the result; false when
/// the parameters do not parse.
fn parse_then_condition(e: &mut Engine, a: ScriptArgs, function: u32) -> bool {
    let Some([argument]) = parse_params(e, a, [0]) else {
        return false;
    };
    call_condition(e, function, a, argument)
}

// Translated from 005d74e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetKillerFunction` (Xbox PDB): parses one value and returns the
/// `AL` of `Script::GetKillerConditionFunction(thisObj, value, 0, result)`;
/// false when the parameters do not parse.
pub fn script_get_killer_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parse_then_condition(e, a, GET_KILLER_CONDITION)
}

// Translated from 005d7540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetKillerObjectFunction` (Xbox PDB): like
/// [`script_get_killer_function`] with the condition function `005a3fa0`.
pub fn script_get_killer_object_function(e: &mut Engine, a: ScriptArgs) -> bool {
    parse_then_condition(e, a, GET_KILLER_OBJECT_CONDITION)
}

// Translated from 005d75a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses two values and returns the `AL` of the condition function
/// `005a4090(thisObj, first, second, result)`; false when the parameters do
/// not parse.
pub fn fn_005d75a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, second]) = parse_params(e, a, [0, 0]) else {
        return false;
    };
    e.call(
        CONDITION_005A4090,
        &args![a.this_obj, first, second, a.result],
    )
    .bool()
}

/// Switches the bit `bit` of [`DECAL_DEBUG_FLAGS`] and prints `enabled` when
/// it was clear, `disabled` when it was set.
fn toggle_decal_debug_bit(e: &mut Engine, bit: u32, enabled: u32, disabled: u32) {
    let flags = e.global::<u32>(DECAL_DEBUG_FLAGS);
    if flags & bit != 0 {
        e.set_global(DECAL_DEBUG_FLAGS, flags & !bit);
        console_print(e, &args![disabled]);
    } else {
        e.set_global(DECAL_DEBUG_FLAGS, flags | bit);
        console_print(e, &args![enabled]);
    }
}

// Translated from 005d7610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleDecalDebug` (Xbox PDB): parses an option number (starting
/// at -1). Options 0 to 5 switch the bits 1, 2, 4, 8, 0x10 and 0x20 of the
/// decal debug flags and print "... : ENABLED" or "... : DISABLED"
/// (wireframe decal clipping planes, decal clipping planes, occlusion query
/// camera direction, decal transform, failed decals, wireframe decals); option
/// 5 also sets or clears the byte `011f9442`. Any other number does nothing.
/// Fails when the parameters do not parse.
pub fn script_toggle_decal_debug(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([option]) = parse_params(e, a, [u32::MAX]) else {
        return false;
    };
    match option {
        0 => toggle_decal_debug_bit(e, 0x1, 0x0103_cdc4, 0x0103_cdf0),
        1 => toggle_decal_debug_bit(e, 0x2, 0x0103_cd80, 0x0103_cda0),
        2 => toggle_decal_debug_bit(e, 0x4, 0x0103_cd28, 0x0103_cd54),
        3 => toggle_decal_debug_bit(e, 0x8, 0x0103_ccf0, 0x0103_cd0c),
        4 => toggle_decal_debug_bit(e, 0x10, 0x0103_ccbc, 0x0103_ccd4),
        5 => {
            let flags = e.global::<u32>(DECAL_DEBUG_FLAGS);
            if flags & 0x20 != 0 {
                e.set_global(DECAL_DEBUG_FLAGS, flags & !0x20);
                e.set_global(DECAL_WIREFRAME_BYTE, 0u8);
                console_print(e, &args![0x0103_cca0u32]);
            } else {
                e.set_global(DECAL_DEBUG_FLAGS, flags | 0x20);
                e.set_global(DECAL_WIREFRAME_BYTE, 1u8);
                console_print(e, &args![0x0103_cc84u32]);
            }
        }
        _ => {}
    }
    true
}

// Translated from 005d7850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleDecalRendering` (Xbox PDB): reads the decal rendering flag
/// of the object `004e2190` finds on the owner `011dea0c`; when it is set
/// prints "Decal Rendering Off" and clears it, otherwise prints "Decal
/// Rendering On" and sets it (`005db4c0`, the getter being called again for the
/// object each time). Always true.
pub fn script_toggle_decal_rendering(e: &mut Engine, _args: ScriptArgs) -> bool {
    let owner = e.global::<u32>(DECAL_OWNER);
    let object = e.call(DECAL_OWNER_OBJECT, &args![owner]).u32();
    let on = e.call(DECAL_RENDERING_GET, &args![object]).bool();
    let (message, value) = if on {
        (MSG_DECAL_RENDERING_OFF, 0u32)
    } else {
        (MSG_DECAL_RENDERING_ON, 1u32)
    };
    console_print(e, &args![message]);
    let owner = e.global::<u32>(DECAL_OWNER);
    let object = e.call(DECAL_OWNER_OBJECT, &args![owner]).u32();
    e.call(DECAL_RENDERING_SET, &args![object, value]);
    true
}

// Translated from 005d78c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer (starting at -1); 0 and 1 are stored by
/// [`fn_005d7930`], any other value does nothing. Fails when the parameters do
/// not parse.
pub fn fn_005d78c0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = parse_params(e, a, [u32::MAX]) else {
        return false;
    };
    if value == 0 {
        fn_005d7930(e, 0);
    }
    if value == 1 {
        fn_005d7930(e, 1);
    }
    true
}

// Translated from 005d7930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` (`value`): stores the byte at `0118abb0`.
pub fn fn_005d7930(e: &mut Engine, value: u8) {
    e.set_global(SWITCH_FLAG_0118ABB0, value);
}

// Translated from 005d7940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsMovingFunction` (Xbox PDB): the condition function
/// `005a40d0(thisObj, 0, 0, result)`, echoed as "IsMoving >> value"; always
/// true.
pub fn script_get_is_moving_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_echo_then_true(e, a, IS_MOVING_CONDITION, MSG_IS_MOVING)
}

// Translated from 005d7990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsTurningFunction` (Xbox PDB): like
/// [`script_get_is_moving_function`] with the condition function
/// `005a4160` ("IsTurning >> value").
pub fn script_get_is_turning_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_echo_then_true(e, a, IS_TURNING_CONDITION, MSG_IS_TURNING)
}

// Translated from 005d79e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetAnimActionFunction` (Xbox PDB): like
/// [`script_get_is_moving_function`] with the condition function
/// `005a41c0` ("GetAnimAction >> value").
pub fn script_get_anim_action_function(e: &mut Engine, a: ScriptArgs) -> bool {
    condition_echo_then_true(e, a, GET_ANIM_ACTION_CONDITION, MSG_GET_ANIM_ACTION)
}

// Translated from 005d7a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses nine values (the fifth and sixth start at 1, the others at 0). When
/// `thisObj` is an actor, calls its virtual slot `0x434` with 0 and then the
/// `actor.cpp` method `008bbf40` on it with the first four values, the ninth
/// value and the fifth to eighth values as flags (`!= 0`). Fails when the
/// parameters do not parse.
pub fn fn_005d7a30(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([v0, v1, v2, v3, v4, v5, v6, v7, v8]) =
        parse_params(e, a, [0, 0, 0, 0, 1, 1, 0, 0, 0])
    else {
        return false;
    };
    let this_obj = a.this_obj.addr();
    if e.vcall(this_obj, REFR_IS_ACTOR_SLOT, &args![]).bool() {
        e.vcall(this_obj, ACTOR_SLOT_434, &args![0u32]);
        e.call(
            ACTOR_UNIT_METHOD_008BBF40,
            &args![
                this_obj,
                v0,
                v1,
                v2,
                v3,
                v8,
                (v4 != 0) as u32,
                (v5 != 0) as u32,
                (v6 != 0) as u32,
                (v7 != 0) as u32
            ],
        );
    }
    true
}

// Translated from 005d7b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls virtual slot `0x19c` of `thisObj` (no null check) and, when it gives
/// something, `008255a0` on it. Always true.
pub fn fn_005d7b50(e: &mut Engine, a: ScriptArgs) -> bool {
    let target = e.vcall(a.this_obj.addr(), REFR_SLOT_19C, &args![]).u32();
    if target != 0 {
        e.call(MAGIC_FUNCTION_008255A0, &args![target]);
    }
    true
}

// Translated from 005d7b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one value and returns the `AL` of the condition function
/// `005a4210(thisObj, value, 0, result)`; false when the parameters do not
/// parse.
pub fn fn_005d7b80(e: &mut Engine, a: ScriptArgs) -> bool {
    parse_then_condition(e, a, CONDITION_005A4210)
}

// Translated from 005d7be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of the condition function `005a4240(thisObj, 0, 0,
/// result)`.
pub fn fn_005d7be0(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, CONDITION_005A4240, a, 0)
}

// Translated from 005d7c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `AL` of the condition function `005a42b0(thisObj, 0, 0,
/// result)`.
pub fn fn_005d7c00(e: &mut Engine, a: ScriptArgs) -> bool {
    call_condition(e, CONDITION_005A42B0, a, 0)
}

// Translated from 005d7c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a quest, an objective index and a flag. Without a quest, logs
/// "Script error: quest ... does not exist." through [`LOG_MESSAGE`] (the
/// game reads the quest's id and name without a null check); with a quest that
/// has no such objective, logs "Script error: objective ... does not exist".
/// When the quest's flag byte has bit 2 clear: a zero flag sets the
/// objective's state to 0 (`005ec5d0`), otherwise an objective whose state is
/// not above 1 gets state 2 or 3 (3 when its bit 0 is set). Always true when
/// the parameters parse.
pub fn fn_005d7c20(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([quest, index, flag]) = parse_params(e, a, [0, 0, 0]) else {
        return false;
    };
    if quest == 0 {
        let id = e.call(GET_FORM_ID, &args![quest]).u32();
        let name = e.vcall(quest, QUEST_NAME_SLOT, &args![]).u32();
        e.call(LOG_MESSAGE, &args![MSG_QUEST_MISSING, name, id]);
        return true;
    }
    let objective = e.call(QUEST_FIND_OBJECTIVE, &args![quest, index]).u32();
    if objective == 0 {
        let id = e.call(GET_FORM_ID, &args![quest]).u32();
        let name = e.vcall(quest, QUEST_NAME_SLOT, &args![]).u32();
        e.call(LOG_MESSAGE, &args![MSG_OBJECTIVE_MISSING, index, name, id]);
        return true;
    }
    if !e.call(QUEST_FLAG_2, &args![quest]).bool() {
        if flag == 0 {
            e.call(QUEST_TARGET_SET_STATE, &args![objective, 0u32]);
        } else if !e.call(OBJECTIVE_ABOVE_ONE, &args![objective]).bool() {
            let odd = e.call(OBJECTIVE_BIT_0, &args![objective]).bool();
            e.call(QUEST_TARGET_SET_STATE, &args![objective, odd as u32 + 2]);
        }
    }
    true
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005d21e0, script_stop_magic_effect(ScriptArgs) -> bool),
        entry!(0x005d22d0, script_preload_magic_effect(ScriptArgs) -> bool),
        entry!(0x005d23d0, fn_005d23d0(ScriptArgs) -> bool),
        entry!(0x005d24a0, fn_005d24a0(ScriptArgs) -> bool),
        entry!(0x005d2500, fn_005d2500(ScriptArgs) -> bool),
        entry!(0x005d2520, script_mod_pc_misc_stat(ScriptArgs) -> bool),
        entry!(0x005d2590, script_get_pc_misc_stat(ScriptArgs) -> bool),
        entry!(0x005d25f0, fn_005d25f0(ScriptArgs) -> bool),
        entry!(0x005d2610, fn_005d2610(ScriptArgs) -> bool),
        entry!(0x005d2630, fn_005d2630(ScriptArgs) -> bool),
        entry!(0x005d2650, script_get_no_rumors_function(ScriptArgs) -> bool),
        entry!(0x005d26b0, fn_005d26b0(ScriptArgs) -> bool),
        entry!(0x005d2780, fn_005d2780(Ptr) -> u8),
        entry!(0x005d27a0, script_get_which_service_function(ScriptArgs) -> bool),
        entry!(0x005d2800, script_trigger_hit_shader_function(ScriptArgs) -> bool),
        entry!(0x005d2860, tes_image_space_modifier_get_get_hit() -> u32),
        entry!(0x005d28b0, script_is_actor_riding_horse_function(ScriptArgs) -> bool),
        entry!(0x005d2910, script_is_players_last_ridden_horse_function(ScriptArgs) -> bool),
        entry!(0x005d2970, script_is_in_dangerous_water_function(ScriptArgs) -> bool),
        entry!(0x005d29d0, script_toggle_water_system(ScriptArgs) -> bool),
        entry!(0x005d2a40, fn_005d2a40() -> u8),
        entry!(0x005d2a50, fn_005d2a50(ScriptArgs) -> bool),
        entry!(0x005d2a80, script_modify_water_shader(ScriptArgs) -> bool),
        entry!(0x005d35d0, fn_005d35d0(ScriptArgs) -> bool),
        entry!(0x005d3780, fn_005d3780(ScriptArgs) -> bool),
        entry!(0x005d3930, fn_005d3930(ScriptArgs) -> bool),
        entry!(0x005d3ae0, script_reset_3d_state_function(ScriptArgs) -> bool),
        entry!(0x005d3b20, script_add_achievement(ScriptArgs) -> bool),
        entry!(0x005d3b90, fn_005d3b90(ScriptArgs) -> bool),
        entry!(0x005d3c20, fn_005d3c20(Ptr, f32)),
        entry!(0x005d3c70, fn_005d3c70(u32, Ptr)),
        entry!(0x005d3ce0, fn_005d3ce0(u32, f32)),
        entry!(0x005d3d10, fn_005d3d10(u32) -> u32),
        entry!(0x005d3d50, fn_005d3d50(ScriptArgs) -> bool),
        entry!(0x005d3dc0, script_get_ignore_friendly_hits_function(ScriptArgs) -> bool),
        entry!(0x005d3e30, script_set_item_value_function(ScriptArgs) -> bool),
        entry!(0x005d3f10, fn_005d3f10(ScriptArgs) -> bool),
        entry!(0x005d4190, script_run_cell_test(ScriptArgs) -> bool),
        entry!(0x005d41f0, script_start_all_quests(ScriptArgs) -> bool),
        entry!(0x005d4280, script_complete_all_quest_stages(ScriptArgs) -> bool),
        entry!(0x005d43c0, fn_005d43c0(u32) -> u32),
        entry!(0x005d43e0, fn_005d43e0(u32) -> u32),
        entry!(0x005d4400, script_flush_persistant_actors(ScriptArgs) -> bool),
        entry!(0x005d4480, script_delete_reference(ScriptArgs) -> bool),
        entry!(0x005d4510, script_toggle_fog_of_war(ScriptArgs) -> bool),
        entry!(0x005d4580, fn_005d4580(ScriptArgs) -> bool),
        entry!(0x005d45c0, fn_005d45c0()),
        entry!(0x005d45e0, fn_005d45e0()),
        entry!(0x005d4600, fn_005d4600(ScriptArgs) -> bool),
        entry!(0x005d4660, fn_005d4660(ScriptArgs) -> bool),
        entry!(0x005d4740, fn_005d4740(ScriptArgs) -> bool),
        entry!(0x005d47c0, fn_005d47c0(ScriptArgs) -> bool),
        entry!(0x005d47e0, fn_005d47e0(ScriptArgs) -> bool),
        entry!(0x005d4870, script_toggle_cast_shadows(ScriptArgs) -> bool),
        entry!(0x005d49c0, fn_005d49c0(ScriptArgs) -> bool),
        entry!(0x005d4a30, fn_005d4a30(u8)),
        entry!(0x005d4a40, fn_005d4a40() -> u8),
        entry!(0x005d4a50, script_is_essential_function(ScriptArgs) -> bool),
        entry!(0x005d4aa0, script_is_actor_function(ScriptArgs) -> bool),
        entry!(0x005d4af0, script_is_player_moving_into_new_space_function(ScriptArgs) -> bool),
        entry!(0x005d4b40, script_get_time_dead_function(ScriptArgs) -> bool),
        entry!(0x005d4b90, fn_005d4b90(ScriptArgs) -> bool),
        entry!(0x005d4c20, fn_005d4c20(Ptr, f32)),
        entry!(0x005d4c40, fn_005d4c40(ScriptArgs) -> bool),
        entry!(0x005d4c60, script_is_player_action_active_function(ScriptArgs) -> bool),
        entry!(0x005d4cc0, fn_005d4cc0(ScriptArgs) -> bool),
        entry!(0x005d4d30, fn_005d4d30(ScriptArgs) -> bool),
        entry!(0x005d4d90, script_add_perk_function(ScriptArgs) -> bool),
        entry!(0x005d4ee0, fn_005d4ee0(u32) -> u8),
        entry!(0x005d4f00, fn_005d4f00(ScriptArgs) -> bool),
        entry!(0x005d4fa0, script_has_perk_function(ScriptArgs) -> bool),
        entry!(0x005d5020, script_send_sherlock_debug_page(ScriptArgs) -> bool),
        entry!(0x005d5120, fn_005d5120(u32) -> u32),
        entry!(0x005d5140, fn_005d5140(ScriptArgs) -> bool),
        entry!(0x005d51a0, script_reward_karma_function(ScriptArgs) -> bool),
        entry!(0x005d5200, fn_005d5200(ScriptArgs) -> bool),
        entry!(0x005d5230, script_add_note_function(ScriptArgs) -> bool),
        entry!(0x005d52a0, script_remove_note_function(ScriptArgs) -> bool),
        entry!(0x005d5330, fn_005d5330(ScriptArgs) -> bool),
        entry!(0x005d5390, fn_005d5390(ScriptArgs) -> bool),
        entry!(0x005d5420, fn_005d5420(ScriptArgs) -> bool),
        entry!(0x005d54a0, fn_005d54a0(ScriptArgs) -> bool),
        entry!(0x005d5560, script_set_global_time_multiplier_function(ScriptArgs) -> bool),
        entry!(0x005d5610, fn_005d5610(u32, u8)),
        entry!(0x005d5630, script_get_hit_location_function(ScriptArgs) -> bool),
        entry!(0x005d5680, script_get_last_hit_critical_function(ScriptArgs) -> bool),
        entry!(0x005d56d0, script_is_pc_1st_person_function(ScriptArgs) -> bool),
        entry!(0x005d5720, fn_005d5720(ScriptArgs) -> bool),
        entry!(0x005d5780, script_set_combat_action_cost_function(ScriptArgs) -> bool),
        entry!(0x005d5850, script_toggle_combat_debug_function(ScriptArgs) -> bool),
        entry!(0x005d67f0, script_force_combat_group_strategy_function(ScriptArgs) -> bool),
        entry!(0x005d6a60, fn_005d6a60(ScriptArgs) -> bool),
        entry!(0x005d6b60, script_push_actor_away_function(ScriptArgs) -> bool),
        entry!(0x005d6d10, fn_005d6d10(ScriptArgs) -> bool),
        entry!(0x005d6d40, script_show_inventory_function(ScriptArgs) -> bool),
        entry!(0x005d7150, fn_005d7150(ScriptArgs) -> bool),
        entry!(0x005d71f0, fn_005d71f0(ScriptArgs) -> bool),
        entry!(0x005d7290, fn_005d7290(ScriptArgs) -> bool),
        entry!(0x005d72f0, fn_005d72f0(ScriptArgs) -> bool),
        entry!(0x005d7350, fn_005d7350(ScriptArgs) -> bool),
        entry!(0x005d73d0, fn_005d73d0(ScriptArgs) -> bool),
        entry!(0x005d7430, fn_005d7430(ScriptArgs) -> bool),
        entry!(0x005d7480, fn_005d7480(ScriptArgs) -> bool),
        entry!(0x005d74d0, fn_005d74d0(ScriptArgs) -> bool),
        entry!(0x005d74e0, script_get_killer_function(ScriptArgs) -> bool),
        entry!(0x005d7540, script_get_killer_object_function(ScriptArgs) -> bool),
        entry!(0x005d75a0, fn_005d75a0(ScriptArgs) -> bool),
        entry!(0x005d7610, script_toggle_decal_debug(ScriptArgs) -> bool),
        entry!(0x005d7850, script_toggle_decal_rendering(ScriptArgs) -> bool),
        entry!(0x005d78c0, fn_005d78c0(ScriptArgs) -> bool),
        entry!(0x005d7930, fn_005d7930(u8)),
        entry!(0x005d7940, script_get_is_moving_function(ScriptArgs) -> bool),
        entry!(0x005d7990, script_get_is_turning_function(ScriptArgs) -> bool),
        entry!(0x005d79e0, script_get_anim_action_function(ScriptArgs) -> bool),
        entry!(0x005d7a30, fn_005d7a30(ScriptArgs) -> bool),
        entry!(0x005d7b50, fn_005d7b50(ScriptArgs) -> bool),
        entry!(0x005d7b80, fn_005d7b80(ScriptArgs) -> bool),
        entry!(0x005d7be0, fn_005d7be0(ScriptArgs) -> bool),
        entry!(0x005d7c00, fn_005d7c00(ScriptArgs) -> bool),
        entry!(0x005d7c20, fn_005d7c20(ScriptArgs) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fake virtual functions the tests put into vtables.
    const V_PATH: u32 = 0x0900_0001;
    const V_IS_ACTOR: u32 = 0x0900_0002;
    const V_SLOT: u32 = 0x0900_0003;
    const V_TARGET: u32 = 0x0900_0004;
    const V_PROXY_SLOT: u32 = 0x0900_0005;

    /// An engine with the pages of the constants, strings and globals these
    /// commands touch mapped, the constants the exe holds written, and the
    /// accessors every command uses replaced by doubles that behave like the
    /// exe's code (see the constants' documentation).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x9_0000);
        e.map(0x0118_0000, 0x9_0000);
        e.set_global(FLOAT_MINUS_ONE, -1.0f32);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.set_global(DOUBLE_HUNDRED, 100.0f64);
        e.set_global(DOUBLE_TWENTY, 20.0f64);
        e.set_global(DOUBLE_255, 255.0f64);
        e.set_global(DOUBLE_MICRO, 1e-6f64);
        e.set_global(REFRACTION_LIMIT, 10.0f32);
        e.set_global(PLAYER_REFRACTION, 0.05f32);
        e.register(CONSOLE_PRINT, |_, _| Ret::default());
        // `__RTDynamicCast`: every object in these tests casts to itself.
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(GET_REFERENCE_NAME, |_, _| 0xaaaa.into_ret());
        e.register(FLOAT_MIN, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            (if y <= x { y } else { x }).into_ret()
        });
        e.register(FLOAT_MAX, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            (if y < x { x } else { y }).into_ret()
        });
        e.register(FTOL2, |_, a| {
            (f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32).into_ret()
        });
        e.register(TES_GET_WATER_SYSTEM, |_, a| (a[0] + 0x64).into_ret());
        for v in [V_PATH, V_IS_ACTOR, V_SLOT, V_TARGET, V_PROXY_SLOT] {
            e.register(v, |_, _| Ret::default());
        }
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
    /// containing, script, event list, then the addresses of the locals.
    fn assert_parsed(e: &Engine, this_obj: u32) {
        assert_eq!(
            calls(e, PARSE_PARAMETERS)[0][..7],
            [1, 2, 8, this_obj, 0, 5, 6]
        );
    }

    /// The two words of a `double`.
    fn f64_words(value: f64) -> [u32; 2] {
        let bits = value.to_bits();
        [bits as u32, (bits >> 32) as u32]
    }

    /// Runs a command that must not parse: false and `callee` not called.
    fn assert_fails_when_unparsed(command_address: u32, callee: u32) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, false, &[]);
        e.register(callee, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(!e.call(command_address, &args![a]).bool());
        assert!(calls(&e, callee).is_empty());
    }

    // ---- 005d21e0 and 005d22d0 -------------------------------------------

    /// A form whose model component (at `+0x18`) has a vtable with the path
    /// function, and `MODEL_PATH_LENGTH` answering `length`.
    fn form_with_model(e: &mut Engine, length: u32) -> u32 {
        let form = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + MODEL_PATH_SLOT, V_PATH);
        e.mem.set_u32(form + 0x18, vtable);
        e.register(V_PATH, |_, _| 0xbeef.into_ret());
        e.register_double(MODEL_PATH_LENGTH, move |_, _| length.into_ret());
        form
    }

    #[test]
    fn stop_magic_effect_removes_the_visual_effect_of_the_model() {
        let mut e = engine();
        let player = set_player(&mut e);
        let form = form_with_model(&mut e, 5);
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            // The float starts at -1.0.
            assert_eq!(e.mem.u32(a[8]), (-1.0f32).to_bits());
            e.mem.set_u32(a[7], form);
            true.into_ret()
        });
        e.register(PROCESS_LISTS_REMOVE_VISUAL_EFFECT, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_21e0, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(
            calls(&e, PROCESS_LISTS_REMOVE_VISUAL_EFFECT),
            vec![vec![PROCESS_LISTS, this_obj, 0xbeef]]
        );
        assert_eq!(calls(&e, MODEL_PATH_LENGTH), vec![vec![form + 0x18]]);
        // No echo flag: nothing printed.
        assert!(calls(&e, CONSOLE_PRINT).is_empty());

        // Without a reference the player is used.
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_21e0, &args![a]).bool());
        assert_eq!(
            calls(&e, PROCESS_LISTS_REMOVE_VISUAL_EFFECT),
            vec![vec![PROCESS_LISTS, player, 0xbeef]]
        );

        // A model without a path, or no form: nothing removed.
        let empty = form_with_model(&mut e, 0);
        parse_gives(&mut e, true, &[empty]);
        start_log(&mut e);
        assert!(e.call(0x005d_21e0, &args![a]).bool());
        parse_gives(&mut e, true, &[0]);
        assert!(e.call(0x005d_21e0, &args![a]).bool());
        assert!(calls(&e, PROCESS_LISTS_REMOVE_VISUAL_EFFECT).is_empty());
    }

    #[test]
    fn stop_magic_effect_echoes_the_reference_name() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[0]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_21e0, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_EFFECT_REMOVED, 0xaaaa]]
        );
        // A reference without a name.
        e.register(GET_REFERENCE_NAME, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_21e0, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_EFFECT_REMOVED_UNNAMED]]
        );
        // Parameters that do not parse: false.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_21e0, &args![a]).bool());
    }

    #[test]
    fn preload_magic_effect_queues_a_model_the_loader_does_not_know() {
        let mut e = engine();
        e.set_global(MODEL_LOADER, 0x7777u32);
        let form = form_with_model(&mut e, 5);
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[form]);
        e.register(MODEL_KEY_CONSTRUCT, |_, _| Ret::default());
        e.register(MODEL_KEY_DESTRUCT, |_, _| Ret::default());
        e.register(MODEL_LOADER_QUEUE_MODEL, |_, _| Ret::default());
        e.register(MODEL_LOADER_FIND_MODEL, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_22d0, &args![a]).bool());
        let key = calls(&e, MODEL_KEY_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, MODEL_KEY_CONSTRUCT), vec![vec![key, 0]]);
        assert_eq!(
            calls(&e, MODEL_LOADER_FIND_MODEL),
            vec![vec![0x7777, 0xbeef, key]]
        );
        assert_eq!(
            calls(&e, MODEL_LOADER_QUEUE_MODEL),
            vec![vec![0x7777, 0xbeef, 5, 0, 0, 1, 0, 0]]
        );
        assert_eq!(calls(&e, MODEL_KEY_DESTRUCT), vec![vec![key]]);

        // A known model is not queued again.
        e.register(MODEL_LOADER_FIND_MODEL, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_22d0, &args![a]).bool());
        assert!(calls(&e, MODEL_LOADER_QUEUE_MODEL).is_empty());
        assert_eq!(calls(&e, MODEL_KEY_DESTRUCT).len(), 1);

        // No form, or a model without a path: nothing at all.
        let empty = form_with_model(&mut e, 0);
        for form in [0, empty] {
            parse_gives(&mut e, true, &[form]);
            start_log(&mut e);
            assert!(e.call(0x005d_22d0, &args![a]).bool());
            assert!(calls(&e, MODEL_KEY_CONSTRUCT).is_empty());
        }
        // Parameters that do not parse: false.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_22d0, &args![a]).bool());
    }

    // ---- 005d23d0, 005d24a0, 005d2500 ------------------------------------

    #[test]
    fn fn_005d23d0_sets_the_clamped_alpha_of_an_actor() {
        let mut e = engine();
        let process = object_with(&mut e, &[(PROCESS_SET_ALPHA_SLOT, V_SLOT)]);
        let actor = object_with(&mut e, &[(REFR_IS_ACTOR_SLOT, V_IS_ACTOR)]);
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(ACTOR_UPDATE_ALPHA, |_, _| Ret::default());
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[1.5f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_23d0, &args![a]).bool());
        assert_parsed(&e, actor);
        // 1.5 is limited to 1.0.
        assert_eq!(calls(&e, V_SLOT), vec![vec![process, 1.0f32.to_bits()]]);
        assert_eq!(calls(&e, ACTOR_UPDATE_ALPHA), vec![vec![actor]]);
        parse_gives(&mut e, true, &[(-0.5f32).to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_23d0, &args![a]).bool());
        assert_eq!(calls(&e, V_SLOT), vec![vec![process, 0.0f32.to_bits()]]);
        parse_gives(&mut e, true, &[0.25f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_23d0, &args![a]).bool());
        assert_eq!(calls(&e, V_SLOT), vec![vec![process, 0.25f32.to_bits()]]);

        // Not an actor: succeeds without touching the process.
        e.register(V_IS_ACTOR, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_23d0, &args![a]).bool());
        assert!(calls(&e, V_SLOT).is_empty());
        // An actor without a process.
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        e.register(GET_PROCESS, |_, _| Ret::default());
        assert!(e.call(0x005d_23d0, &args![a]).bool());
        assert!(calls(&e, V_SLOT).is_empty());
        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_23d0, &args![a]).bool());
    }

    #[test]
    fn fn_005d23d0_uses_the_player_without_a_reference() {
        let mut e = engine();
        let player = set_player(&mut e);
        let process = object_with(&mut e, &[(PROCESS_SET_ALPHA_SLOT, V_SLOT)]);
        e.mem
            .set_u32(e.mem.u32(player) + REFR_IS_ACTOR_SLOT, V_IS_ACTOR);
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(ACTOR_UPDATE_ALPHA, |_, _| Ret::default());
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0.5f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_23d0, &args![a]).bool());
        assert_eq!(calls(&e, ACTOR_UPDATE_ALPHA), vec![vec![player]]);
    }

    #[test]
    fn fn_005d24a0_calls_the_method_for_a_parsed_object() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(CELL_UPDATE, |_, _| Ret::default());
        parse_gives(&mut e, true, &[0x1234]);
        start_log(&mut e);
        assert!(e.call(0x005d_24a0, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(calls(&e, CELL_UPDATE), vec![vec![0x1234, 0xffff_ffff, 0]]);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_24a0, &args![a]).bool());
        assert!(calls(&e, CELL_UPDATE).is_empty());
        assert_fails_when_unparsed(0x005d_24a0, CELL_UPDATE);
    }

    #[test]
    fn fn_005d2500_always_succeeds() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(IS_IN_INTERIOR_CONDITION, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_2500, &args![a]).bool());
        assert_eq!(
            calls(&e, IS_IN_INTERIOR_CONDITION),
            vec![vec![this_obj, 0, 0, a.result.addr()]]
        );
    }

    // ---- 005d2520 .. 005d2650 --------------------------------------------

    #[test]
    fn mod_pc_misc_stat_passes_the_parsed_stat_and_amount() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(MISC_STAT_MOD_VAL, |_, _| Ret::default());
        parse_gives(&mut e, true, &[7, 0xffff_fffe]);
        start_log(&mut e);
        assert!(e.call(0x005d_2520, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(calls(&e, MISC_STAT_MOD_VAL), vec![vec![7, 0xffff_fffe]]);
        assert_fails_when_unparsed(0x005d_2520, MISC_STAT_MOD_VAL);
    }

    #[test]
    fn get_pc_misc_stat_hands_the_stat_to_the_condition_function() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        // The condition function's `AL` is not the command's.
        e.register(GET_PC_MISC_STAT_CONDITION, |_, _| false.into_ret());
        parse_gives(&mut e, true, &[9]);
        start_log(&mut e);
        assert!(e.call(0x005d_2590, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(
            calls(&e, GET_PC_MISC_STAT_CONDITION),
            vec![vec![this_obj, 9, 0, a.result.addr()]]
        );
        assert_fails_when_unparsed(0x005d_2590, GET_PC_MISC_STAT_CONDITION);
    }

    /// The test of a "no arguments, return the condition function's `AL`"
    /// command.
    fn check_condition_only(command_address: u32, condition: u32) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(condition, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(!e.call(command_address, &args![a]).bool());
        assert_eq!(
            calls(&e, condition),
            vec![vec![this_obj, 0, 0, a.result.addr()]]
        );
        e.register(condition, |_, _| true.into_ret());
        assert!(e.call(command_address, &args![a]).bool());
    }

    #[test]
    fn the_plain_condition_commands_return_the_condition_functions_al() {
        check_condition_only(0x005d_25f0, IS_ACTOR_EVIL_CONDITION);
        check_condition_only(0x005d_2610, IS_ACTOR_VICTIM_CONDITION);
        check_condition_only(0x005d_2630, CONDITION_005A3570);
    }

    /// The test of a "condition function, then echo the result double"
    /// command.
    fn check_condition_with_echo(command_address: u32, condition: u32, format: u32) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(condition, |e, a| {
            e.mem.set_f64(a[3], 2.5);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_eq!(
            calls(&e, condition),
            vec![vec![this_obj, 0, 0, a.result.addr()]]
        );
        // No echo flag: nothing printed.
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(command_address, &args![a]).bool());
        let [low, high] = f64_words(2.5);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![format, low, high]]);
        // The condition function's `AL` is returned.
        e.register(condition, |_, _| false.into_ret());
        assert!(!e.call(command_address, &args![a]).bool());
    }

    #[test]
    fn the_echoing_condition_commands_print_the_result() {
        check_condition_with_echo(0x005d_2650, GET_NO_RUMORS_CONDITION, MSG_GET_NO_RUMORS);
        check_condition_with_echo(
            0x005d_27a0,
            GET_WHICH_SERVICE_CONDITION,
            MSG_GET_WHICH_SERVICE,
        );
        check_condition_with_echo(
            0x005d_28b0,
            IS_ACTOR_RIDING_HORSE_CONDITION,
            MSG_IS_ACTOR_RIDING_HORSE,
        );
        check_condition_with_echo(
            0x005d_2910,
            IS_PLAYERS_LAST_RIDDEN_HORSE_CONDITION,
            MSG_PLAYERS_LAST_RIDDEN_HORSE,
        );
        check_condition_with_echo(0x005d_2970, IS_IN_DANGEROUS_WATER_CONDITION, MSG_IN_LAVA);
    }

    // ---- 005d26b0, 005d2780 ----------------------------------------------

    #[test]
    fn fn_005d2780_tests_the_flag_0x2000() {
        let mut e = engine();
        e.register(FLAGS_TEST, |_, a| {
            assert_eq!(a, [0x4000, 0x2000]);
            1u32.into_ret()
        });
        start_log(&mut e);
        assert_eq!(e.call(0x005d_2780, &args![0x4000u32]).u8(), 1);
        assert_eq!(calls(&e, FLAGS_TEST), vec![vec![0x4000, 0x2000]]);
    }

    /// The scene of `005d26b0`: an actor whose base form has the flag
    /// `flag_set` in its actor base data (`+0x30`).
    fn flag_scene(e: &mut Engine, flag_set: bool) -> (u32, u32) {
        let actor = object_with(e, &[(MARK_CHANGED_SLOT, V_SLOT)]);
        let base_form = e.mem.alloc(0x100);
        e.register_double(GET_BASE_FORM_OF_REFERENCE, move |_, _| base_form.into_ret());
        e.register_double(FLAGS_TEST, move |_, a| {
            assert_eq!(a, [base_form + 0x30, 0x2000]);
            (flag_set as u32).into_ret()
        });
        e.register(EXTRA_DATA_LIST_CLEAR, |_, _| Ret::default());
        e.register(EXTRA_DATA_LIST_SET, |_, _| Ret::default());
        (actor, base_form)
    }

    #[test]
    fn fn_005d26b0_clears_or_sets_the_list_depending_on_the_flag() {
        // The command's flag agrees with the base data: the list is cleared.
        let mut e = engine();
        let (actor, _) = flag_scene(&mut e, true);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        assert!(e.call(0x005d_26b0, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, EXTRA_DATA_LIST_CLEAR), vec![vec![actor + 0x44]]);
        assert!(calls(&e, EXTRA_DATA_LIST_SET).is_empty());
        assert_eq!(calls(&e, V_SLOT), vec![vec![actor, 0x8000_0000]]);

        // They differ: the list is set to the flag.
        let mut e = engine();
        let (actor, _) = flag_scene(&mut e, false);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        assert!(e.call(0x005d_26b0, &args![a]).bool());
        assert_eq!(calls(&e, EXTRA_DATA_LIST_SET), vec![vec![actor + 0x44, 1]]);
        assert!(calls(&e, EXTRA_DATA_LIST_CLEAR).is_empty());
        assert_eq!(calls(&e, V_SLOT), vec![vec![actor, 0x8000_0000]]);

        // A zero argument against a set flag differs too and sets 0.
        parse_gives(&mut e, true, &[0]);
        e.register(FLAGS_TEST, |_, _| 1u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_26b0, &args![a]).bool());
        assert_eq!(calls(&e, EXTRA_DATA_LIST_SET), vec![vec![actor + 0x44, 0]]);

        // Not an actor: nothing happens but the command succeeds.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_26b0, &args![a]).bool());
        assert!(calls(&e, V_SLOT).is_empty());
        assert_fails_when_unparsed(0x005d_26b0, EXTRA_DATA_LIST_SET);
    }

    // ---- 005d2800, 005d2860 ----------------------------------------------

    #[test]
    fn get_get_hit_looks_the_modifier_up_once() {
        let mut e = engine();
        // Disabled: null, nothing looked up.
        e.register(GET_DEFAULT_FORM, |_, _| 0x5000u32.into_ret());
        start_log(&mut e);
        assert_eq!(e.call(0x005d_2860, &args![]).u32(), 0);
        assert!(calls(&e, GET_DEFAULT_FORM).is_empty());

        e.set_global(IMAGE_SPACE_ENABLED, 1u8);
        e.register(DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x005d_2860, &args![]).u32(), 0x5001);
        assert_eq!(calls(&e, GET_DEFAULT_FORM), vec![vec![0x162]]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![
                0x5000,
                0,
                RTTI_TES_FORM,
                RTTI_TES_IMAGE_SPACE_MODIFIER,
                0
            ]]
        );
        assert_eq!(e.global::<u32>(CACHED_GET_HIT_MODIFIER), 0x5001);
        // The cached modifier is returned without another lookup.
        assert_eq!(e.call(0x005d_2860, &args![]).u32(), 0x5001);
        assert_eq!(calls(&e, GET_DEFAULT_FORM).len(), 1);
    }

    #[test]
    fn trigger_hit_shader_triggers_the_get_hit_modifier() {
        let mut e = engine();
        e.set_global(IMAGE_SPACE_ENABLED, 1u8);
        e.set_global(CACHED_GET_HIT_MODIFIER, 0x5001u32);
        e.register(IMAGE_SPACE_MODIFIER_TRIGGER, |_, _| Ret::default());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[0.5f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_2800, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(
            calls(&e, IMAGE_SPACE_MODIFIER_TRIGGER),
            vec![vec![0x5001, 0.5f32.to_bits(), 0]]
        );
        // The default strength is 1.0.
        e.register_double(PARSE_PARAMETERS, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_2800, &args![a]).bool());
        assert_eq!(
            calls(&e, IMAGE_SPACE_MODIFIER_TRIGGER),
            vec![vec![0x5001, 1.0f32.to_bits(), 0]]
        );
        assert_fails_when_unparsed(0x005d_2800, IMAGE_SPACE_MODIFIER_TRIGGER);
    }

    // ---- 005d29d0, 005d2a40, 005d2a50 ------------------------------------

    #[test]
    fn toggle_water_system_switches_by_the_flag() {
        let mut e = engine();
        e.set_global(TES_SINGLETON, 0x1000u32);
        e.register(WATER_SYSTEM_ENABLE, |_, _| true.into_ret());
        e.register(WATER_SYSTEM_DISABLE, |_, _| false.into_ret());
        let a = command(&mut e, 0);
        // Flag clear: "On", enable, the enable call's `AL`.
        start_log(&mut e);
        assert!(e.call(0x005d_29d0, &args![a]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_WATER_SYSTEM_ON]]);
        assert_eq!(calls(&e, WATER_SYSTEM_ENABLE), vec![vec![0x1064]]);
        assert!(calls(&e, WATER_SYSTEM_DISABLE).is_empty());
        // Flag set: "Off", `004e6620(water, 1, 0)`.
        e.set_global(WATER_SYSTEM_FLAG, 1u8);
        start_log(&mut e);
        assert!(!e.call(0x005d_29d0, &args![a]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_WATER_SYSTEM_OFF]]);
        assert_eq!(calls(&e, WATER_SYSTEM_DISABLE), vec![vec![0x1064, 1, 0]]);
        assert!(calls(&e, WATER_SYSTEM_ENABLE).is_empty());
    }

    #[test]
    fn the_water_flag_accessors() {
        let mut e = engine();
        e.set_global(TES_SINGLETON, 0x1000u32);
        assert_eq!(e.call(0x005d_2a40, &args![]).u8(), 0);
        let a = command(&mut e, 0);
        // Flag clear: the other byte stays clear.
        assert!(e.call(0x005d_2a50, &args![a]).bool());
        assert_eq!(e.global::<u8>(WATER_FLAG_011C7A64), 0);
        e.set_global(WATER_SYSTEM_FLAG, 1u8);
        assert_eq!(e.call(0x005d_2a40, &args![]).u8(), 1);
        start_log(&mut e);
        assert!(e.call(0x005d_2a50, &args![a]).bool());
        assert_eq!(e.global::<u8>(WATER_FLAG_011C7A64), 1);
        assert_eq!(calls(&e, TES_GET_WATER_SYSTEM), vec![vec![0x1000]]);
    }

    // ---- 005d2a80 ---------------------------------------------------------

    /// The names `STRICMP` is asked about, as text.
    const SETTING_NAMES: [(u32, &str); 25] = [
        (NAME_VELOCITY, "velocity"),
        (NAME_DIRECTION, "direction"),
        (NAME_AMPLITUDE, "amplitude"),
        (NAME_FREQUENCY, "frequency"),
        (NAME_REFLECT_AMOUNT, "reflectamt"),
        (NAME_DISTORT, "distort"),
        (NAME_FRESNEL, "fresnel"),
        (NAME_OPACITY, "opacity"),
        (NAME_BLEND, "blend"),
        (NAME_NOISE, "noise"),
        (NAME_OFF, "off"),
        (NAME_DISPLACE_FORCE, "displaceforce"),
        (NAME_DISPLACE_VELOCITY, "displacevelocity"),
        (NAME_DISPLACE_FALLOFF, "displacefalloff"),
        (NAME_DISPLACE_DAMPENER, "displacedampener"),
        (NAME_RAIN_FORCE, "rainforce"),
        (NAME_RAIN_VELOCITY, "rainvelocity"),
        (NAME_RAIN_FALLOFF, "rainfalloff"),
        (NAME_RAIN_SIZE, "rainsize"),
        (NAME_REFLECT, "reflect"),
        (NAME_REFRACT, "refract"),
        (NAME_DEPTH, "depth"),
        (NAME_FOG, "fog"),
        (NAME_LOD, "lod"),
        (NAME_AUTOWATER, "autowater"),
    ];

    /// An engine for `ModifyWaterShader`: `ParseParameters` writes `name` and
    /// `value`, `__stricmp` compares with the text of the exe's strings, and
    /// the water system objects of a reference are reachable. Returns the
    /// engine and the water shader property of the command's reference.
    fn water_scene(name: &str, value: f32) -> (Engine, ScriptArgs, u32) {
        let mut e = engine();
        e.set_global(TES_SINGLETON, 0x1000u32);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        let name = name.to_string();
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            e.mem.set_cstr(a[7], name.as_bytes());
            e.mem.set_f32(a[8], value);
            true.into_ret()
        });
        e.register(STRICMP, |e, a| {
            let text = SETTING_NAMES
                .iter()
                .find(|(address, _)| *address == a[1])
                .map(|(_, text)| *text)
                .expect("an unknown setting name");
            let given = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            (if given.eq_ignore_ascii_case(text) {
                0u32
            } else {
                1u32
            })
            .into_ret()
        });
        // The water object of a reference and its property of type 3.
        let property = e.mem.alloc(0x200);
        e.register(WATER_SYSTEM_GET_OBJECT, |_, a| (a[1] + 0x100).into_ret());
        e.register_double(NI_AV_OBJECT_GET_PROPERTY, move |_, a| {
            assert_eq!(a[1], 3);
            property.into_ret()
        });
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        (e, a, property)
    }

    /// Runs `005d2a80` on a scene with `this_obj` set or cleared.
    fn run_water(e: &mut Engine, a: ScriptArgs, with_reference: bool) -> bool {
        let a = if with_reference {
            a
        } else {
            ScriptArgs {
                this_obj: Ptr::NULL,
                ..a
            }
        };
        start_log(e);
        e.call(0x005d_2a80, &args![a]).bool()
    }

    #[test]
    fn modify_water_shader_prints_the_usage_when_the_parameters_do_not_parse() {
        let (mut e, a, _) = water_scene("noise", 5.0);
        parse_gives(&mut e, false, &[]);
        assert!(!run_water(&mut e, a, true));
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_MWS_USAGE]]);
        assert!(calls(&e, STRICMP).is_empty());
    }

    #[test]
    fn modify_water_shader_finds_the_property_of_the_reference() {
        let (mut e, a, property) = water_scene("velocity", 5.0);
        assert!(run_water(&mut e, a, true));
        assert_eq!(calls(&e, TES_GET_WATER_SYSTEM), vec![vec![0x1000]]);
        assert_eq!(
            calls(&e, WATER_SYSTEM_GET_OBJECT),
            vec![vec![0x1064, a.this_obj.addr()]]
        );
        assert_eq!(
            calls(&e, NI_AV_OBJECT_GET_PROPERTY),
            vec![vec![a.this_obj.addr() + 0x100, 3]]
        );
        // The first name is accepted and nothing else is compared or done.
        assert_eq!(calls(&e, STRICMP).len(), 1);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        assert_eq!(e.mem.u8(property + water::MANUAL_FLAG), 0);
        // Without a reference no property is looked up.
        assert!(run_water(&mut e, a, false));
        assert!(calls(&e, WATER_SYSTEM_GET_OBJECT).is_empty());
    }

    #[test]
    fn modify_water_shader_ignores_the_names_that_do_nothing() {
        for (name, compares) in [
            ("direction", 2),
            ("Amplitude", 3),
            ("FREQUENCY", 4),
            ("blend", 9),
        ] {
            let (mut e, a, _) = water_scene(name, 5.0);
            assert!(run_water(&mut e, a, false));
            assert_eq!(calls(&e, STRICMP).len(), compares, "{name}");
            assert!(calls(&e, CONSOLE_PRINT).is_empty());
        }
        // An unknown name goes through every comparison and does nothing.
        let (mut e, a, property) = water_scene("nonsense", 5.0);
        assert!(run_water(&mut e, a, true));
        assert_eq!(calls(&e, STRICMP).len(), SETTING_NAMES.len());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        assert_eq!(e.mem.u8(property + water::MANUAL_FLAG), 0);
    }

    /// A setting that stores a value in the property (`offset`) or the global
    /// (`global`), prints `format` with the value and marks the change.
    fn check_float_setting(
        name: &str,
        value: f32,
        stored: f32,
        offset: u32,
        global: u32,
        formats: [u32; 2],
    ) {
        let [property_format, global_format] = formats;
        // With a reference: the property field, the message, the manual
        // byte cleared.
        let (mut e, a, property) = water_scene(name, value);
        e.mem.set_u8(property + water::MANUAL_FLAG, 1);
        assert!(run_water(&mut e, a, true), "{name}");
        assert_eq!(e.mem.f32(property + offset), stored, "{name}");
        assert_eq!(e.mem.u8(property + water::MANUAL_FLAG), 0, "{name}");
        let [low, high] = f64_words(f64::from(value));
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![property_format, low, high]],
            "{name}"
        );
        assert_eq!(e.global::<u8>(WATER_SETTINGS_CHANGED), 0);
        // Without: the global, the message, the changed byte raised.
        let (mut e, a, _) = water_scene(name, value);
        assert!(run_water(&mut e, a, false), "{name}");
        assert_eq!(e.global::<f32>(global), stored, "{name}");
        assert_eq!(e.global::<u8>(WATER_SETTINGS_CHANGED), 1, "{name}");
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![global_format, low, high]],
            "{name}"
        );
    }

    #[test]
    fn modify_water_shader_sets_the_float_settings() {
        check_float_setting(
            "reflectamt",
            0.75,
            0.75,
            water::REFLECT_AMOUNT,
            0x011f_f3ec,
            [MSG_REFLECT_AMOUNT; 2],
        );
        check_float_setting(
            "distort",
            3.5,
            3.5,
            water::DISTORTION,
            0x011f_f3f4,
            [MSG_DISTORTION; 2],
        );
        check_float_setting(
            "FRESNEL",
            0.5,
            0.5,
            water::FRESNEL,
            0x011f_f10c,
            [MSG_FRESNEL; 2],
        );
        check_float_setting(
            "noise",
            50.0,
            50.0,
            water::NOISE_SCALE,
            0x011f_fe48,
            // The property branch prints another message.
            [MSG_NOISE_SCALE, MSG_NOISE],
        );
    }

    #[test]
    fn modify_water_shader_prints_usage_for_values_out_of_range() {
        for (name, value, usage, upper) in [
            ("reflectamt", 1.5f32, MSG_USAGE_REFLECT_AMOUNT, 1.0f32),
            ("reflectamt", -0.5, MSG_USAGE_REFLECT_AMOUNT, 1.0),
            ("fresnel", 2.0, MSG_USAGE_FRESNEL, 1.0),
            ("opacity", 101.0, MSG_USAGE_OPACITY, 100.0),
            ("noise", -1.0, MSG_USAGE_NOISE, 100.0),
            ("noise", f32::NAN, MSG_USAGE_NOISE, 100.0),
        ] {
            let (mut e, a, property) = water_scene(name, value);
            e.mem.set_u8(property + water::MANUAL_FLAG, 1);
            assert!(run_water(&mut e, a, true));
            let [low, high] = f64_words(f64::from(value));
            let printed = calls(&e, CONSOLE_PRINT);
            assert_eq!(printed.len(), 1, "{name} {value}");
            assert_eq!(printed[0][0], usage);
            if !value.is_nan() {
                assert_eq!(printed[0][1..], [low, high]);
            }
            // Nothing was stored.
            assert_eq!(e.mem.u8(property + water::MANUAL_FLAG), 1);
            assert_eq!(e.global::<u8>(WATER_SETTINGS_CHANGED), 0);
            assert!(upper > 0.0);
        }
        // The upper bounds themselves are accepted.
        let (mut e, a, property) = water_scene("fresnel", 1.0);
        assert!(run_water(&mut e, a, true));
        assert_eq!(e.mem.f32(property + water::FRESNEL), 1.0);
        let (mut e, a, property) = water_scene("fresnel", 0.0);
        assert!(run_water(&mut e, a, true));
        assert_eq!(e.mem.f32(property + water::FRESNEL), 0.0);
    }

    #[test]
    fn modify_water_shader_scales_opacity_and_fog_by_a_hundred() {
        for (name, offset, global, format) in [
            ("opacity", water::OPACITY, 0x011f_f3f0, MSG_OPACITY),
            ("fog", water::FOG, 0x011f_f414, MSG_FOG_AMOUNT),
        ] {
            let (mut e, a, property) = water_scene(name, 25.0);
            e.mem.set_u8(property + water::MANUAL_FLAG, 1);
            assert!(run_water(&mut e, a, true));
            assert_eq!(e.mem.f32(property + offset), 0.25);
            assert_eq!(e.mem.u8(property + water::MANUAL_FLAG), 0);
            // The message gets the truncated integer.
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![format, 25]]);
            assert_eq!(calls(&e, FTOL2), vec![f64_words(25.0).to_vec()]);
            let (mut e, a, _) = water_scene(name, 25.0);
            assert!(run_water(&mut e, a, false));
            assert_eq!(e.global::<f32>(global), 0.25);
            assert_eq!(e.global::<u8>(WATER_SETTINGS_CHANGED), 1);
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![format, 25]]);
        }
        // Fog out of range: ignored without a message.
        let (mut e, a, property) = water_scene("fog", 150.0);
        assert!(run_water(&mut e, a, true));
        assert_eq!(e.mem.f32(property + water::FOG), 0.0);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn modify_water_shader_off_marks_the_property_or_clears_the_global_byte() {
        let (mut e, a, property) = water_scene("off", 0.0);
        assert!(run_water(&mut e, a, true));
        assert_eq!(e.mem.u8(property + water::MANUAL_FLAG), 1);
        let (mut e, a, _) = water_scene("off", 0.0);
        e.set_global(WATER_SETTINGS_CHANGED, 1u8);
        assert!(run_water(&mut e, a, false));
        assert_eq!(e.global::<u8>(WATER_SETTINGS_CHANGED), 0);
    }

    #[test]
    fn modify_water_shader_stores_the_ranged_global_values() {
        for (name, upper, global) in [
            ("displaceforce", 1.0f32, 0x0120_0014),
            ("displacevelocity", 1.0, 0x0120_0018),
            ("displacefalloff", 1.0, 0x0120_001c),
            ("displacedampener", 20.0, 0x011f_fff0),
            ("rainforce", 1.0, 0x0120_0004),
            ("rainvelocity", 1.0, 0x0120_0008),
            ("rainfalloff", 1.0, 0x0120_000c),
            ("rainsize", 1.0, 0x0120_0010),
        ] {
            // Within range (also with a reference: the property is not used).
            let (mut e, a, property) = water_scene(name, upper / 2.0);
            assert!(run_water(&mut e, a, true), "{name}");
            assert_eq!(e.global::<f32>(global), upper / 2.0, "{name}");
            assert_eq!(e.global::<u8>(WATER_SETTINGS_CHANGED), 1, "{name}");
            assert_eq!(e.mem.u8(property + water::MANUAL_FLAG), 0);
            // The upper bound is accepted, anything above is not.
            let (mut e, a, _) = water_scene(name, upper);
            assert!(run_water(&mut e, a, false), "{name}");
            assert_eq!(e.global::<f32>(global), upper, "{name}");
            let (mut e, a, _) = water_scene(name, upper + 0.5);
            assert!(run_water(&mut e, a, false), "{name}");
            assert_eq!(e.global::<f32>(global), 0.0, "{name}");
            assert_eq!(e.global::<u8>(WATER_SETTINGS_CHANGED), 0, "{name}");
            // Negative values are ignored.
            let (mut e, a, _) = water_scene(name, -0.5);
            assert!(run_water(&mut e, a, false), "{name}");
            assert_eq!(e.global::<f32>(global), 0.0, "{name}");
            assert!(calls(&e, CONSOLE_PRINT).is_empty());
        }
    }

    /// The global value objects `reflect`, `refract` and `depth` toggle: a
    /// byte that the `GLOBAL_VALUE_ADDRESS` double finds at `object + 4`.
    fn global_toggle_scene(name: &str, object: u32) -> (Engine, ScriptArgs, u32) {
        let (mut e, a, property) = water_scene(name, 0.0);
        e.register(GLOBAL_VALUE_ADDRESS, |_, a| (a[0] + 4).into_ret());
        e.register(GLOBAL_VALUE_SET, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret::default()
        });
        e.set_global(object + 4, 0u8);
        (e, a, property)
    }

    #[test]
    fn modify_water_shader_toggles_reflect_refract_and_depth_on_the_property() {
        // (name, tested byte, byte written when the tested one is clear,
        // off message, on message)
        for (name, tested, written, off, on) in [
            (
                "reflect",
                water::REFLECT_FLAG,
                water::REFLECT_FLAG,
                MSG_REFLECTIONS_OFF,
                MSG_REFLECTIONS_ON,
            ),
            // `refract` tests +0x81 but sets +0x80.
            (
                "refract",
                water::REFRACT_FLAG,
                water::REFLECT_FLAG,
                MSG_REFRACTIONS_OFF,
                MSG_REFRACTIONS_ON,
            ),
            (
                "depth",
                water::DEPTH_FLAG,
                water::DEPTH_FLAG,
                MSG_DEPTH_OFF,
                MSG_DEPTH_ON,
            ),
        ] {
            let (mut e, a, property) = water_scene(name, 0.0);
            assert!(run_water(&mut e, a, true), "{name}");
            assert_eq!(e.mem.u8(property + written), 1, "{name}");
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![on]], "{name}");
            e.mem.set_u8(property + tested, 1);
            assert!(run_water(&mut e, a, true), "{name}");
            assert_eq!(e.mem.u8(property + tested), 0, "{name}");
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![off]], "{name}");
            assert_eq!(e.global::<u8>(WATER_SETTINGS_CHANGED), 0);
        }
        // The `refract` quirk: the tested byte stays clear.
        let (mut e, a, property) = water_scene("refract", 0.0);
        assert!(run_water(&mut e, a, true));
        assert_eq!(e.mem.u8(property + water::REFRACT_FLAG), 0);
    }

    #[test]
    fn modify_water_shader_toggles_the_global_objects_without_a_reference() {
        for (name, object, off, on) in [
            (
                "reflect",
                GLOBAL_REFLECT_OBJECT,
                MSG_GLOBAL_REFLECTIONS_OFF,
                MSG_GLOBAL_REFLECTIONS_ON,
            ),
            (
                "refract",
                GLOBAL_REFRACT_OBJECT,
                MSG_GLOBAL_REFRACTIONS_OFF,
                MSG_GLOBAL_REFRACTIONS_ON,
            ),
            (
                "depth",
                GLOBAL_DEPTH_OBJECT,
                MSG_GLOBAL_DEPTH_OFF,
                MSG_GLOBAL_DEPTH_ON,
            ),
        ] {
            let (mut e, a, _) = global_toggle_scene(name, object);
            assert!(run_water(&mut e, a, false), "{name}");
            assert_eq!(e.global::<u8>(object + 4), 1, "{name}");
            assert_eq!(calls(&e, GLOBAL_VALUE_SET), vec![vec![object, 1]]);
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![on]], "{name}");
            assert!(run_water(&mut e, a, false), "{name}");
            assert_eq!(e.global::<u8>(object + 4), 0, "{name}");
            assert_eq!(calls(&e, GLOBAL_VALUE_SET), vec![vec![object, 0]]);
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![off]], "{name}");
        }
    }

    #[test]
    fn modify_water_shader_toggles_the_lod_water() {
        let (mut e, a, _) = water_scene("lod", 0.0);
        e.register(WATER_SYSTEM_TOGGLE_LOD, |_, _| Ret::default());
        assert!(run_water(&mut e, a, false));
        assert_eq!(
            calls(&e, WATER_SYSTEM_TOGGLE_LOD),
            vec![vec![0x1064, 1, 1, 1]]
        );
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_TOGGLE_LOD_WATER]]);
        // Without a water system only the message.
        e.register(TES_GET_WATER_SYSTEM, |_, _| Ret::default());
        assert!(run_water(&mut e, a, false));
        assert!(calls(&e, WATER_SYSTEM_TOGGLE_LOD).is_empty());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_TOGGLE_LOD_WATER]]);
    }

    #[test]
    fn modify_water_shader_times_the_water_generation() {
        let (mut e, a, _) = water_scene("autowater", 0.0);
        // 2 MHz clock; the counter reads 1000 then 5000 (a 64-bit value with
        // a high word).
        e.register(QUERY_PERFORMANCE_FREQUENCY, |e, a| {
            e.mem.set_u64(a[0], 2_000_000);
            1u32.into_ret()
        });
        let readings = std::cell::Cell::new(0);
        e.register_double(QUERY_PERFORMANCE_COUNTER, move |e, a| {
            let value = [0x1_0000_0000u64 - 500, 0x1_0000_0000 + 4_500][readings.get()];
            readings.set(readings.get() + 1);
            e.mem.set_u64(a[0], value);
            1u32.into_ret()
        });
        e.register(TES_GET_CURRENT_CELL, |_, _| 0x2000u32.into_ret());
        e.register(GENERATE_PLACEABLE_WATER_FOR_CELL, |_, _| Ret::default());
        e.register(CELL_GET_NAME, |_, _| 0x3000u32.into_ret());
        e.register(SPRINTF_S, |_, _| Ret::default());
        e.register(ERROR_REPORT, |_, _| Ret::default());
        e.register(WATER_SYSTEM_ENABLE, |_, _| Ret::default());
        assert!(run_water(&mut e, a, false));
        assert_eq!(
            calls(&e, GENERATE_PLACEABLE_WATER_FOR_CELL),
            vec![vec![0x2000]]
        );
        // 5000 ticks at 2 ticks per microsecond.
        let sprintf = &calls(&e, SPRINTF_S)[0];
        let [low, high] = f64_words(2500.0);
        assert_eq!(
            sprintf[1..],
            [0xff, MSG_GENERATE_WATER_TIMING, 0x3000, 5000, 0, low, high]
        );
        let text = sprintf[0];
        assert_eq!(calls(&e, ERROR_REPORT), vec![vec![text, 0]]);
        // The water system flag is clear: the water system is enabled.
        assert_eq!(calls(&e, WATER_SYSTEM_ENABLE), vec![vec![0x1064]]);
        // With the flag set it is left alone.
        e.set_global(WATER_SYSTEM_FLAG, 1u8);
        let readings = std::cell::Cell::new(0);
        e.register_double(QUERY_PERFORMANCE_COUNTER, move |e, a| {
            e.mem.set_u64(a[0], 10 * readings.get());
            readings.set(readings.get() + 1);
            1u32.into_ret()
        });
        assert!(run_water(&mut e, a, false));
        assert!(calls(&e, WATER_SYSTEM_ENABLE).is_empty());
    }

    // ---- 005d35d0, 005d3780, 005d3930 ------------------------------------

    /// The colour commands: `(command address, property offset, global)`.
    const COLOUR_COMMANDS: [(u32, u32, u32); 3] = [
        (0x005d_35d0, 0x88, 0x011f_f3b8),
        (0x005d_3780, 0x98, 0x011f_f3c8),
        (0x005d_3930, 0xa8, 0x011f_f3d8),
    ];

    /// An engine for the colour commands: a reference whose slot `0x1d0`
    /// gives the target, `0043b4a0` the shape and `GetProperty` the
    /// property; `00414430` builds `(r, g, b, a)` in its block.
    fn colour_scene(parsed: [u32; 3]) -> (Engine, ScriptArgs, u32) {
        let mut e = engine();
        let reference = object_with(&mut e, &[(REFR_GET_TARGET_SLOT, V_TARGET)]);
        let property = e.mem.alloc(0x200);
        e.register(V_TARGET, |_, _| 0x4000u32.into_ret());
        e.register(GET_SHAPE, |_, a| {
            assert_eq!(a, [0x4000, 0]);
            0x4100u32.into_ret()
        });
        e.register_double(NI_AV_OBJECT_GET_PROPERTY, move |_, a| {
            assert_eq!(a, [0x4100, 3]);
            property.into_ret()
        });
        e.register(COLOUR_CONSTRUCT, |e, a| {
            for word in 0..4 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            a[0].into_ret()
        });
        parse_gives(&mut e, true, &parsed);
        let a = command(&mut e, reference);
        (e, a, property)
    }

    #[test]
    fn the_water_colour_commands_store_the_colour() {
        for (address, offset, global) in COLOUR_COMMANDS {
            // With a reference: the four floats in the property.
            let (mut e, a, property) = colour_scene([255, 0, 51]);
            start_log(&mut e);
            assert!(e.call(address, &args![a]).bool());
            assert_parsed(&e, a.this_obj.addr());
            assert_eq!(calls(&e, V_TARGET), vec![vec![a.this_obj.addr()]]);
            assert_eq!(
                e.mem.bytes(property + offset, 16),
                [1.0f32, 0.0, 51.0 / 255.0, 1.0]
                    .iter()
                    .flat_map(|v| v.to_bits().to_le_bytes())
                    .collect::<Vec<u8>>()
            );
            // Without one: the global (nothing is looked up).
            let a = ScriptArgs {
                this_obj: Ptr::NULL,
                ..a
            };
            start_log(&mut e);
            assert!(e.call(address, &args![a]).bool());
            assert!(calls(&e, GET_SHAPE).is_empty());
            assert_eq!(e.global::<f32>(global), 1.0);
            assert_eq!(e.global::<f32>(global + 4), 0.0);
            assert_eq!(e.global::<f32>(global + 8), 51.0f32 / 255.0);
            assert_eq!(e.global::<f32>(global + 12), 1.0);
        }
    }

    #[test]
    fn the_water_colour_commands_ignore_channels_out_of_range() {
        for (address, offset, global) in COLOUR_COMMANDS {
            for parsed in [[256, 0, 0], [0, 0xffff_ffff, 0], [0, 0, 300]] {
                let (mut e, a, property) = colour_scene(parsed);
                assert!(e.call(address, &args![a]).bool());
                assert_eq!(e.mem.bytes(property + offset, 16), vec![0u8; 16]);
                let a = ScriptArgs {
                    this_obj: Ptr::NULL,
                    ..a
                };
                assert!(e.call(address, &args![a]).bool());
                assert_eq!(e.global::<u32>(global), 0);
            }
            // Parameters that do not parse.
            let (mut e, a, _) = colour_scene([1, 2, 3]);
            parse_gives(&mut e, false, &[]);
            assert!(!e.call(address, &args![a]).bool());
        }
    }

    // ---- 005d3ae0, 005d3b20 ----------------------------------------------

    #[test]
    fn reset_3d_state_clears_the_saved_state_of_a_reference() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(REFERENCE_UNLOAD_3D, |_, _| Ret::default());
        e.register(EXTRA_DATA_LIST_REMOVE_SAVED_ANIMATION, |_, _| {
            Ret::default()
        });
        e.register(EXTRA_DATA_LIST_REMOVE_SAVED_HAVOK_DATA, |_, _| {
            Ret::default()
        });
        start_log(&mut e);
        assert!(e.call(0x005d_3ae0, &args![a]).bool());
        assert_eq!(calls(&e, REFERENCE_UNLOAD_3D), vec![vec![this_obj]]);
        assert_eq!(
            calls(&e, EXTRA_DATA_LIST_REMOVE_SAVED_ANIMATION),
            vec![vec![this_obj + 0x44]]
        );
        assert_eq!(
            calls(&e, EXTRA_DATA_LIST_REMOVE_SAVED_HAVOK_DATA),
            vec![vec![this_obj + 0x44]]
        );
        // No reference: still succeeds, nothing happens.
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_3ae0, &args![a]).bool());
        assert!(calls(&e, REFERENCE_UNLOAD_3D).is_empty());
    }

    #[test]
    fn add_achievement_unlocks_unless_disabled() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(AWARDS_QUERY_INSTANCE, |_, _| 0x6000u32.into_ret());
        e.register(AWARDS_UNLOCK, |_, _| Ret::default());
        parse_gives(&mut e, true, &[12]);
        start_log(&mut e);
        assert!(e.call(0x005d_3b20, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(calls(&e, AWARDS_UNLOCK), vec![vec![0x6000, 12]]);
        e.set_global(AWARDS_DISABLED, 1u8);
        start_log(&mut e);
        assert!(e.call(0x005d_3b20, &args![a]).bool());
        assert!(calls(&e, AWARDS_UNLOCK).is_empty());
        assert_fails_when_unparsed(0x005d_3b20, AWARDS_UNLOCK);
    }

    // ---- 005d3b90 .. 005d3d10 --------------------------------------------

    #[test]
    fn fn_005d3b90_picks_the_path_by_the_setting() {
        let mut e = engine();
        let reference = object_with(&mut e, &[(REFR_GET_TARGET_SLOT, V_TARGET)]);
        let a = command(&mut e, reference);
        e.register(V_TARGET, |_, _| 0x4000u32.into_ret());
        e.register(FOR_EACH_ENTITY, |_, _| Ret::default());
        e.register(SEND_REFERENCE_EVENT, |_, _| Ret::default());
        e.register(GET_QUEUE_OWNER, |_, _| 0x8000u32.into_ret());
        let setting = e.mem.alloc(4);
        e.register_double(GET_SETTING_INTEGER, move |_, a| {
            assert_eq!(a, [EVENT_SETTING]);
            setting.into_ret()
        });
        parse_gives(&mut e, true, &[2.5f32.to_bits()]);

        // Setting at most 1: the block path.
        e.mem.set_u32(setting, 1);
        start_log(&mut e);
        assert!(e.call(0x005d_3b90, &args![a]).bool());
        assert_parsed(&e, reference);
        assert_eq!(calls(&e, FOR_EACH_ENTITY).len(), 1);
        assert!(calls(&e, SEND_REFERENCE_EVENT).is_empty());

        // Above 1 (signed): the event, with the float as a double.
        e.mem.set_u32(setting, 2);
        start_log(&mut e);
        assert!(e.call(0x005d_3b90, &args![a]).bool());
        let [low, high] = f64_words(2.5);
        assert_eq!(
            calls(&e, SEND_REFERENCE_EVENT),
            vec![vec![0x8000, 0x1156, reference, low, high]]
        );
        assert!(calls(&e, FOR_EACH_ENTITY).is_empty());
        e.mem.set_i32(setting, -5);
        start_log(&mut e);
        assert!(e.call(0x005d_3b90, &args![a]).bool());
        assert_eq!(calls(&e, FOR_EACH_ENTITY).len(), 1);

        // No reference: false without anything else.
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(!e.call(0x005d_3b90, &args![a]).bool());
        assert!(calls(&e, GET_SETTING_INTEGER).is_empty());
        assert_fails_when_unparsed(0x005d_3b90, GET_SETTING_INTEGER);
    }

    #[test]
    fn fn_005d3c20_builds_the_block_and_hands_it_to_the_walker() {
        let mut e = engine();
        let reference = object_with(&mut e, &[(REFR_GET_TARGET_SLOT, V_TARGET)]);
        e.register(V_TARGET, |_, _| 0x4000u32.into_ret());
        e.register(FOR_EACH_ENTITY, |e, a| {
            // The block as the callback will see it.
            assert_eq!(e.mem.u32(a[1]), 0);
            assert_eq!(e.mem.u8(a[1] + 4), 1);
            assert_eq!(e.mem.u32(a[1] + 8), 0x12);
            assert_eq!(e.mem.f32(a[1] + 12), 0.75);
            Ret::default()
        });
        start_log(&mut e);
        e.call(0x005d_3c20, &args![reference, 0.75f32]);
        let walker = calls(&e, FOR_EACH_ENTITY);
        assert_eq!(walker.len(), 1);
        assert_eq!(walker[0][0], 0x4000);
        assert_eq!(walker[0][2], 0x005d_3c70);
    }

    #[test]
    fn fn_005d3c70_passes_the_float_on_to_the_registered_object() {
        let mut e = engine();
        e.register(ENTITY_KEY, |_, a| (a[0] + 1).into_ret());
        e.register(LOOKUP_BY_KEY, |_, a| {
            assert_eq!(a[0], ENTITY_TABLE);
            (if a[1] == 0x1001 { 0x5000u32 } else { 0 }).into_ret()
        });
        let proxy_target = object_with(&mut e, &[(PROXY_OBJECT_SLOT_C, V_PROXY_SLOT)]);
        e.register(GET_PROXY, |_, a| (a[0] + 0x10).into_ret());
        e.register(PROXY_SET_VALUE, |_, _| Ret::default());
        e.register_double(PROXY_GET_OBJECT, move |_, _| proxy_target.into_ret());
        let block = e.mem.alloc(16);
        e.mem.set_f32(block + 12, 0.5);
        start_log(&mut e);
        e.call(0x005d_3c70, &args![0x1000u32, block]);
        assert_eq!(
            calls(&e, PROXY_SET_VALUE),
            vec![vec![0x5010, 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, V_PROXY_SLOT),
            vec![vec![proxy_target, 0.5f32.to_bits()]]
        );
        // An entity without a registered object: nothing.
        start_log(&mut e);
        e.call(0x005d_3c70, &args![0x2000u32, block]);
        assert!(calls(&e, PROXY_SET_VALUE).is_empty());
        // A registered object whose second object is null: only the first
        // call.
        e.register(PROXY_GET_OBJECT, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x005d_3c70, &args![0x1000u32, block]);
        assert_eq!(calls(&e, PROXY_SET_VALUE).len(), 1);
        assert!(calls(&e, V_PROXY_SLOT).is_empty());
    }

    #[test]
    fn fn_005d3ce0_and_fn_005d3d10_go_through_the_proxy_when_there_is_one() {
        let mut e = engine();
        e.register(GET_PROXY, |_, a| {
            (if a[0] == 0x10 { 0x500 } else { 0 }).into_ret()
        });
        e.register(PROXY_SET_VALUE, |_, _| Ret::default());
        e.register(PROXY_GET_OBJECT, |_, a| (a[0] + 1).into_ret());
        start_log(&mut e);
        e.call(0x005d_3ce0, &args![0x10u32, 3.0f32]);
        assert_eq!(
            calls(&e, PROXY_SET_VALUE),
            vec![vec![0x500, 3.0f32.to_bits()]]
        );
        assert_eq!(e.call(0x005d_3d10, &args![0x10u32]).u32(), 0x501);
        start_log(&mut e);
        e.call(0x005d_3ce0, &args![0x20u32, 3.0f32]);
        assert!(calls(&e, PROXY_SET_VALUE).is_empty());
        assert_eq!(e.call(0x005d_3d10, &args![0x20u32]).u32(), 0);
        assert!(calls(&e, PROXY_GET_OBJECT).is_empty());
    }

    // ---- 005d3d50 .. 005d3e30 --------------------------------------------

    #[test]
    fn fn_005d3d50_sets_the_form_flag_unless_the_argument_is_minus_one() {
        let mut e = engine();
        let this_obj = object_with(&mut e, &[(MARK_CHANGED_SLOT, V_SLOT)]);
        let a = command(&mut e, this_obj);
        e.register(FORM_SET_FLAG, |_, _| Ret::default());
        parse_gives(&mut e, true, &[3]);
        start_log(&mut e);
        assert!(e.call(0x005d_3d50, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this_obj, 1]]);
        assert_eq!(calls(&e, V_SLOT), vec![vec![this_obj, 1]]);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_3d50, &args![a]).bool());
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this_obj, 0]]);
        // -1 (the default): nothing.
        parse_gives(&mut e, true, &[0xffff_ffff]);
        start_log(&mut e);
        assert!(e.call(0x005d_3d50, &args![a]).bool());
        assert!(calls(&e, FORM_SET_FLAG).is_empty());
        assert!(calls(&e, V_SLOT).is_empty());
        e.register_double(PARSE_PARAMETERS, |_, _| true.into_ret());
        assert!(e.call(0x005d_3d50, &args![a]).bool());
        assert!(calls(&e, FORM_SET_FLAG).is_empty());
        assert_fails_when_unparsed(0x005d_3d50, FORM_SET_FLAG);
    }

    #[test]
    fn get_ignore_friendly_hits_echoes_the_wording() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(GET_IGNORE_FRIENDLY_HITS_CONDITION, |e, a| {
            let value = e.mem.u32(0x011d_f000) as f64;
            e.mem.set_f64(a[3], value);
            (value == 0.0).into_ret()
        });
        e.map(0x011d_f000, 0x1000);
        start_log(&mut e);
        assert!(e.call(0x005d_3dc0, &args![a]).bool());
        assert_eq!(
            calls(&e, GET_IGNORE_FRIENDLY_HITS_CONDITION),
            vec![vec![this_obj, 0, 0, a.result.addr()]]
        );
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005d_3dc0, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_STRING, TEXT_COUNTS_FRIENDLY_HITS]]
        );
        e.mem.set_u32(0x011d_f000, 1);
        start_log(&mut e);
        // The condition function's `AL` is returned.
        assert!(!e.call(0x005d_3dc0, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![FORMAT_STRING, TEXT_IGNORES_FRIENDLY_HITS]]
        );
    }

    #[test]
    fn set_item_value_sets_the_value_of_a_value_form() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(GET_BASE_FORM, |_, _| 0x9000u32.into_ret());
        e.register(VALUE_FORM_SET_VALUE, |_, _| Ret::default());
        e.register(REFRESH_MENU, |_, _| Ret::default());
        parse_gives(&mut e, true, &[77]);
        start_log(&mut e);
        assert!(e.call(0x005d_3e30, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x9000, 0, RTTI_TES_OBJECT, RTTI_TES_VALUE_FORM, 0]]
        );
        assert_eq!(calls(&e, VALUE_FORM_SET_VALUE), vec![vec![0x9000, 77]]);
        assert_eq!(calls(&e, REFRESH_MENU), vec![vec![0]]);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // With the echo flag the name and the value are printed.
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_3e30, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_SET_ITEM_VALUE, 0xaaaa, 77]]
        );

        // The base form is not a value form.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_3e30, &args![a]).bool());
        assert!(calls(&e, VALUE_FORM_SET_VALUE).is_empty());
        assert!(calls(&e, REFRESH_MENU).is_empty());
        // No base form, no reference.
        e.register(GET_BASE_FORM, |_, _| Ret::default());
        assert!(e.call(0x005d_3e30, &args![a]).bool());
        let a = command(&mut e, 0);
        assert!(e.call(0x005d_3e30, &args![a]).bool());
        assert_fails_when_unparsed(0x005d_3e30, VALUE_FORM_SET_VALUE);
    }

    // ---- 005d3f10 ---------------------------------------------------------

    /// A scene for `005d3f10`: an actor with a process, the refraction gate
    /// open, and doubles for the slots and helpers.
    fn refraction_scene(gate_open: bool) -> (Engine, ScriptArgs, u32, u32) {
        let mut e = engine();
        let process = object_with(&mut e, &[(PROCESS_SET_REFRACTION_SLOT, V_PROXY_SLOT)]);
        let actor = object_with(
            &mut e,
            &[
                (ACTOR_SET_REFRACTION_SLOT, V_SLOT),
                (ACTOR_SLOT_41C, V_TARGET),
            ],
        );
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(ACTOR_UPDATE_ALPHA, |_, _| Ret::default());
        e.register(FN_004DC0A0, |e, _| e.mem.u8(0x011f_9180).into_ret());
        e.register(FN_004DC060, |e, _| e.mem.u32(0x011f_91c0).into_ret());
        e.mem.set_u8(0x011f_9180, gate_open as u8);
        e.mem.set_u32(0x011f_91c0, 2);
        let a = command(&mut e, actor);
        (e, a, actor, process)
    }

    /// The addresses of the gate functions of `fn_005b9b00`.
    const FN_004DC0A0: u32 = 0x004d_c0a0;
    const FN_004DC060: u32 = 0x004d_c060;

    #[test]
    fn fn_005d3f10_sets_the_refraction_power() {
        let (mut e, a, actor, process) = refraction_scene(true);
        parse_gives(&mut e, true, &[3.0f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_3f10, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(
            calls(&e, V_PROXY_SLOT),
            vec![vec![process, 3.0f32.to_bits()]]
        );
        // A positive value: enable and the power.
        assert_eq!(calls(&e, V_SLOT), vec![vec![actor, 1, 3.0f32.to_bits()]]);
        assert!(calls(&e, ACTOR_UPDATE_ALPHA).is_empty());
        // The power is limited to 10.0.
        parse_gives(&mut e, true, &[20.0f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_3f10, &args![a]).bool());
        assert_eq!(
            calls(&e, V_PROXY_SLOT),
            vec![vec![process, 10.0f32.to_bits()]]
        );
        // Zero and below: disabled, and the alpha is updated.
        for value in [0.0f32, -2.0] {
            parse_gives(&mut e, true, &[value.to_bits()]);
            start_log(&mut e);
            assert!(e.call(0x005d_3f10, &args![a]).bool());
            assert_eq!(calls(&e, V_SLOT), vec![vec![actor, 0, 0]]);
            assert_eq!(calls(&e, ACTOR_UPDATE_ALPHA), vec![vec![actor]]);
        }
        // No echo flag: nothing printed.
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        parse_gives(&mut e, true, &[2.0f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_3f10, &args![a]).bool());
        let [low, high] = f64_words(2.0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_REFRACTION_SET, 0xaaaa, low, high]]
        );
    }

    #[test]
    fn fn_005d3f10_skips_the_actor_slots_when_the_gate_is_closed() {
        let (mut e, a, _, process) = refraction_scene(false);
        parse_gives(&mut e, true, &[3.0f32.to_bits()]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_3f10, &args![a]).bool());
        // The process still gets the power, the actor nothing; the echo
        // still appears.
        assert_eq!(
            calls(&e, V_PROXY_SLOT),
            vec![vec![process, 3.0f32.to_bits()]]
        );
        assert!(calls(&e, V_SLOT).is_empty());
        assert_eq!(calls(&e, CONSOLE_PRINT).len(), 1);
    }

    #[test]
    fn fn_005d3f10_needs_an_actor_with_a_process() {
        let (mut e, a, _, _) = refraction_scene(true);
        parse_gives(&mut e, true, &[3.0f32.to_bits()]);
        set_echo(&mut e, true);
        // No process: succeeds without the echo.
        e.register(GET_PROCESS, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_3f10, &args![a]).bool());
        assert!(calls(&e, V_PROXY_SLOT).is_empty());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // Not an actor.
        start_log(&mut e);
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x005d_3f10, &args![a]).bool());
        assert!(calls(&e, GET_PROCESS).is_empty());
        // The player is used without a reference.
        let player = set_player(&mut e);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(GET_PROCESS, |_, _| 0u32.into_ret());
        let a = ScriptArgs {
            this_obj: Ptr::NULL,
            ..a
        };
        start_log(&mut e);
        assert!(e.call(0x005d_3f10, &args![a]).bool());
        assert_eq!(calls(&e, GET_PROCESS), vec![vec![player]]);
        // Parameters that do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_3f10, &args![a]).bool());
    }

    #[test]
    fn fn_005d3f10_blends_the_settings_when_the_ramp_is_positive() {
        // Doubles that make the second local positive (the real limit
        // functions never do).
        let (mut e, a, actor, _) = refraction_scene(true);
        e.register(FLOAT_MAX, |_, a| {
            if f32::from_bits(a[0]) == 0.0 && f32::from_bits(a[1]) == 0.0 {
                50.0f32.into_ret()
            } else {
                let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
                (if y < x { x } else { y }).into_ret()
            }
        });
        // The upper limit never applies: the ramp is 50.0.
        e.register(FLOAT_MIN, |_, a| f32::from_bits(a[1]).into_ret());
        e.register(SETTING_FLOAT_A, |_, _| 4.0f32.into_ret());
        e.register(SETTING_FLOAT_B, |_, _| 2.0f32.into_ret());
        e.register(LERP, |_, _| 3.0f32.into_ret());
        parse_gives(&mut e, true, &[1.0f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_3f10, &args![a]).bool());
        assert_eq!(calls(&e, V_TARGET), vec![vec![actor, 1.0f32.to_bits()]]);
        assert_eq!(
            calls(&e, LERP),
            vec![vec![
                2.0f32.to_bits(),
                4.0f32.to_bits(),
                0.0f32.to_bits(),
                1.0f32.to_bits(),
                0.5f32.to_bits()
            ]]
        );
        assert_eq!(calls(&e, V_SLOT), vec![vec![actor, 1, 3.0f32.to_bits()]]);
    }

    // ---- 005d4190 .. 005d4280 --------------------------------------------

    #[test]
    fn run_cell_test_calls_the_tes_method() {
        let mut e = engine();
        e.set_global(TES_SINGLETON, 0x1000u32);
        e.register(TES_RUN_CELL_TEST, |_, _| Ret::default());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[4]);
        start_log(&mut e);
        assert!(e.call(0x005d_4190, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(calls(&e, TES_RUN_CELL_TEST), vec![vec![0x1000, 4]]);
        assert_fails_when_unparsed(0x005d_4190, TES_RUN_CELL_TEST);
    }

    /// A list of nodes `(node, item)` as the quest list functions walk it: the
    /// doubles for the node accessors read the table. `next` is the node
    /// after; a node address of 0 ends the walk; `empty` is the node that
    /// ends the walk early.
    struct Nodes {
        items: Vec<(u32, u32, u32)>,
        empty: Vec<u32>,
    }

    fn node_doubles(e: &mut Engine, nodes: Vec<(u32, u32, u32)>, empty: Vec<u32>) {
        let table = std::rc::Rc::new(Nodes {
            items: nodes,
            empty,
        });
        let t = table.clone();
        e.register_double(NODE_IS_EMPTY, move |_, a| {
            t.empty.contains(&a[0]).into_ret()
        });
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        let t = table.clone();
        e.register_double(NODE_NEXT, move |_, a| {
            t.items
                .iter()
                .find(|(node, _, _)| *node == a[0])
                .map(|(_, _, next)| *next)
                .unwrap_or(0)
                .into_ret()
        });
        // The item of a node is the word at the node's address; the tests
        // write it.
        for (node, item, _) in &table.items {
            e.map(*node & !0xfff, 0x1000);
            e.mem.set_u32(*node, *item);
        }
    }

    #[test]
    fn start_all_quests_enables_every_quest_up_to_an_empty_node() {
        let mut e = engine();
        e.set_global(DATA_HANDLER, 0x3000u32);
        e.register(DATA_HANDLER_QUEST_LIST, |_, a| (a[0] + 0x118).into_ret());
        e.register(QUEST_SET_ENABLED, |_, _| Ret::default());
        node_doubles(
            &mut e,
            vec![
                (0x0a00_0100, 0xa1, 0x0a00_0200),
                (0x0a00_0200, 0xa2, 0x0a00_0300),
                (0x0a00_0300, 0xa3, 0x0a00_0400),
                (0x0a00_0400, 0xa4, 0),
            ],
            vec![0x0a00_0300],
        );
        e.register(DATA_HANDLER_QUEST_LIST, |_, _| 0x0a00_0100u32.into_ret());
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_41f0, &args![a]).bool());
        assert_eq!(
            calls(&e, QUEST_SET_ENABLED),
            vec![vec![0xa1, 1], vec![0xa2, 1]]
        );
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        e.mem.set_f64(a.result.addr(), 1.5);
        start_log(&mut e);
        assert!(e.call(0x005d_41f0, &args![a]).bool());
        let [low, high] = f64_words(1.5);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_ALL_QUESTS_ENABLED, low, high]]
        );
        // An empty list: nothing.
        e.register(DATA_HANDLER_QUEST_LIST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_41f0, &args![a]).bool());
        assert!(calls(&e, QUEST_SET_ENABLED).is_empty());
    }

    #[test]
    fn complete_all_quest_stages_walks_the_stages_and_the_targets() {
        let mut e = engine();
        e.set_global(DATA_HANDLER, 0x3000u32);
        e.register(GET_PROCESS_SCRIPTS, |_, _| 1u32.into_ret());
        e.register(SET_PROCESS_SCRIPTS, |_, _| Ret::default());
        e.register(QUEST_STAGE_SET_DONE, |_, _| Ret::default());
        e.register(QUEST_TARGET_SET_STATE, |_, _| Ret::default());
        // Two quests; the first has two stages and one target, the second
        // none. The lists are embedded in the quest: nodes at `quest + 0x44`
        // (stages) and `quest + 0x4c` (targets).
        let (quest_a, quest_b) = (0x0b00_0000u32, 0x0b01_0000u32);
        e.map(quest_a, 0x1000);
        e.map(quest_b, 0x1000);
        node_doubles(
            &mut e,
            vec![
                (0x0a00_0100, quest_a, 0x0a00_0200),
                (0x0a00_0200, quest_b, 0),
                (0x0b00_0044, 0xe1, 0x0c00_0200),
                (0x0c00_0200, 0xe2, 0),
                (0x0b00_004c, 0xf1, 0),
            ],
            vec![0x0b01_0044, 0x0b01_004c],
        );
        e.register(DATA_HANDLER_QUEST_LIST, |_, _| 0x0a00_0100u32.into_ret());
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_4280, &args![a]).bool());
        assert_eq!(
            calls(&e, QUEST_STAGE_SET_DONE),
            vec![vec![0xe1, quest_a, 1], vec![0xe2, quest_a, 1]]
        );
        assert_eq!(
            calls(&e, QUEST_TARGET_SET_STATE),
            vec![vec![0xf1, 1], vec![0xf1, 3]]
        );
        // Script processing is switched off and restored to what it was.
        assert_eq!(calls(&e, SET_PROCESS_SCRIPTS), vec![vec![0], vec![1]]);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        e.mem.set_f64(a.result.addr(), 2.5);
        start_log(&mut e);
        assert!(e.call(0x005d_4280, &args![a]).bool());
        let [low, high] = f64_words(2.5);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_ALL_STAGES_COMPLETED, low, high]]
        );
    }

    #[test]
    fn complete_all_quest_stages_stops_at_an_empty_node() {
        let mut e = engine();
        e.set_global(DATA_HANDLER, 0x3000u32);
        e.register(GET_PROCESS_SCRIPTS, |_, _| 0u32.into_ret());
        e.register(SET_PROCESS_SCRIPTS, |_, _| Ret::default());
        e.register(QUEST_STAGE_SET_DONE, |_, _| Ret::default());
        e.register(QUEST_TARGET_SET_STATE, |_, _| Ret::default());
        let quest = 0x0b00_0000u32;
        e.map(quest, 0x1000);
        node_doubles(
            &mut e,
            vec![
                (0x0a00_0100, quest, 0),
                (0x0b00_0044, 0xe1, 0x0c00_0200),
                (0x0c00_0200, 0xe2, 0),
                (0x0b00_004c, 0xf1, 0x0d00_0200),
                (0x0d00_0200, 0xf2, 0),
            ],
            vec![0x0c00_0200, 0x0d00_0200],
        );
        e.register(DATA_HANDLER_QUEST_LIST, |_, _| 0x0a00_0100u32.into_ret());
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_4280, &args![a]).bool());
        assert_eq!(calls(&e, QUEST_STAGE_SET_DONE).len(), 1);
        assert_eq!(calls(&e, QUEST_TARGET_SET_STATE).len(), 2);
        assert_eq!(calls(&e, SET_PROCESS_SCRIPTS), vec![vec![0], vec![0]]);
    }

    // ---- 005d43c0 .. 005d4a40 --------------------------------------------

    /// The same pages `engine()` maps plus the one of `012682f8`.
    fn engine_with_high_globals() -> Engine {
        let mut e = engine();
        e.map(0x0126_0000, 0x1_0000);
        e
    }

    #[test]
    fn the_list_accessors_add_their_offsets() {
        let mut e = engine();
        assert_eq!(e.call(0x005d_43c0, &args![0x1000u32]).u32(), 0x1044);
        assert_eq!(e.call(0x005d_43e0, &args![0x1000u32]).u32(), 0x104c);
    }

    #[test]
    fn flush_persistant_actors_flushes_the_process_lists() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(PROCESS_LISTS_FLUSH_NON_PERSISTENT_ACTORS, |_, _| {
            Ret::default()
        });
        parse_gives(&mut e, true, &[7]);
        start_log(&mut e);
        assert!(e.call(0x005d_4400, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(
            calls(&e, PROCESS_LISTS_FLUSH_NON_PERSISTENT_ACTORS),
            vec![vec![PROCESS_LISTS, 7]]
        );
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005d_4400, &args![a]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_FLUSHED_ACTORS]]);
        assert_fails_when_unparsed(0x005d_4400, PROCESS_LISTS_FLUSH_NON_PERSISTENT_ACTORS);
    }

    #[test]
    fn delete_reference_calls_the_virtual_delete_and_reports_false() {
        let mut e = engine();
        let reference = object_with(&mut e, &[(REFR_DELETE_SLOT, V_SLOT)]);
        let a = command(&mut e, reference);
        start_log(&mut e);
        assert!(!e.call(0x005d_4480, &args![a]).bool());
        assert_eq!(calls(&e, V_SLOT), vec![vec![reference, 1]]);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        assert!(!e.call(0x005d_4480, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_DELETING_REFERENCE, 0xaaaa]]
        );
    }

    #[test]
    fn delete_reference_without_a_reference_succeeds_and_says_so() {
        let mut e = engine();
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_4480, &args![a]).bool());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005d_4480, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_NO_REFERENCE_TO_FLUSH]]
        );
    }

    #[test]
    fn toggle_fog_of_war_sets_the_opposite_and_reports_the_new_state() {
        use std::{cell::Cell, rc::Rc};
        for (before, text) in [(false, TEXT_ENABLED), (true, TEXT_DISABLED)] {
            let mut e = engine();
            let a = command(&mut e, 0);
            let state = Rc::new(Cell::new(before));
            let read = state.clone();
            e.register_double(FOG_OF_WAR_IS_ENABLED, move |_, _| read.get().into_ret());
            let write = state.clone();
            e.register_double(FOG_OF_WAR_SET, move |_, a| {
                write.set(a[0] != 0);
                Ret::default()
            });
            start_log(&mut e);
            assert!(e.call(0x005d_4510, &args![a]).bool());
            assert_eq!(calls(&e, FOG_OF_WAR_SET), vec![vec![(!before) as u32]]);
            assert!(calls(&e, CONSOLE_PRINT).is_empty());
            set_echo(&mut e, true);
            state.set(before);
            start_log(&mut e);
            assert!(e.call(0x005d_4510, &args![a]).bool());
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_FOG_OF_WAR, text]]);
        }
    }

    #[test]
    fn fn_005d45c0_clears_the_word_of_the_object_when_there_is_one() {
        let mut e = engine();
        e.call(0x005d_45c0, &args![]);
        let menu = object(&mut e);
        e.mem.set_u32(menu + 0x128, 5);
        e.set_global(MAP_MENU, menu);
        e.call(0x005d_45c0, &args![]);
        assert_eq!(e.mem.u32(menu + 0x128), 0);
    }

    #[test]
    fn fn_005d45e0_calls_the_menu_function_when_there_is_an_object() {
        let mut e = engine();
        e.register(MAP_MENU_FUNCTION_0079FFB0, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x005d_45e0, &args![]);
        assert!(calls(&e, MAP_MENU_FUNCTION_0079FFB0).is_empty());
        let menu = object(&mut e);
        e.set_global(MAP_MENU, menu);
        e.call(0x005d_45e0, &args![]);
        assert_eq!(calls(&e, MAP_MENU_FUNCTION_0079FFB0), vec![vec![1]]);
    }

    #[test]
    fn fn_005d4580_runs_the_menu_function_25_times_with_the_byte_set() {
        let mut e = engine();
        let menu = object(&mut e);
        e.mem.set_u32(menu + 0x128, 9);
        e.set_global(MAP_MENU, menu);
        e.register(MAP_MENU_FUNCTION_0079FFB0, |e, _| {
            // The byte is set while the loop runs and the word is cleared.
            assert_eq!(e.mem.u8(MAP_MENU_BUSY_FLAG), 1);
            let menu: u32 = e.global(MAP_MENU);
            assert_eq!(e.mem.u32(menu + 0x128), 0);
            Ret::default()
        });
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_4580, &args![a]).bool());
        assert_eq!(calls(&e, MAP_MENU_FUNCTION_0079FFB0).len(), 25);
        assert_eq!(e.mem.u8(MAP_MENU_BUSY_FLAG), 0);
        // Without the object nothing is called but the command succeeds.
        e.set_global(MAP_MENU, 0u32);
        start_log(&mut e);
        assert!(e.call(0x005d_4580, &args![a]).bool());
        assert!(calls(&e, MAP_MENU_FUNCTION_0079FFB0).is_empty());
    }

    #[test]
    fn fn_005d4600_hands_the_float_to_the_renderer() {
        let mut e = engine();
        e.register(RENDERER_SET_VALUE, |_, _| Ret::default());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[2.5f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_4600, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(calls(&e, RENDERER_SET_VALUE), vec![vec![2.5f32.to_bits()]]);
        assert_fails_when_unparsed(0x005d_4600, RENDERER_SET_VALUE);
    }

    #[test]
    fn fn_005d4660_maps_the_value_to_an_anisotropy_level() {
        let cases = [
            (-5.0f32, 1u32),
            (0.5, 1),
            (1.9, 1),
            (2.0, 2),
            (3.9, 2),
            (4.0, 4),
            (7.0, 4),
            (8.0, 8),
            (11.9, 8),
            (12.0, 12),
            (15.0, 12),
            (16.0, 16),
            (100.0, 16),
        ];
        for (value, level) in cases {
            let mut e = engine();
            let a = command(&mut e, 0);
            parse_gives(&mut e, true, &[value.to_bits()]);
            start_log(&mut e);
            assert!(e.call(0x005d_4660, &args![a]).bool());
            assert_eq!(e.mem.u32(MAX_ANISOTROPY), level, "value {value}");
            assert!(calls(&e, CONSOLE_PRINT).is_empty());
            set_echo(&mut e, true);
            assert!(e.call(0x005d_4660, &args![a]).bool());
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_MAX_ANISO, level]]);
        }
        // Not parsed: false and the setting stays.
        let mut e = engine();
        e.mem.set_u32(MAX_ANISOTROPY, 3);
        let a = command(&mut e, 0);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_4660, &args![a]).bool());
        assert_eq!(e.mem.u32(MAX_ANISOTROPY), 3);
    }

    #[test]
    fn fn_005d4740_only_parses() {
        let mut e = engine();
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[3.0f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005d_4740, &args![a]).bool());
        assert_parsed(&e, 0);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_4740, &args![a]).bool());
    }

    #[test]
    fn fn_005d47c0_clears_the_adapted_light_only_when_the_byte_is_set() {
        let mut e = engine();
        e.register(IMAGE_SPACE_HDR_CLEAR_ADAPTED_LIGHT, |_, _| Ret::default());
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_47c0, &args![a]).bool());
        assert!(calls(&e, IMAGE_SPACE_HDR_CLEAR_ADAPTED_LIGHT).is_empty());
        e.mem.set_u8(HDR_ACTIVE_FLAG, 1);
        assert!(e.call(0x005d_47c0, &args![a]).bool());
        assert_eq!(calls(&e, IMAGE_SPACE_HDR_CLEAR_ADAPTED_LIGHT).len(), 1);
    }

    #[test]
    fn fn_005d47e0_calls_the_method_on_the_parsed_value_for_an_actor() {
        let mut e = engine();
        let actor = object(&mut e);
        let a = command(&mut e, actor);
        e.register(GET_BASE_FORM, |_, _| 0xf0f0u32.into_ret());
        e.register(ACTOR_UNIT_METHOD_008C0EC0, |_, _| Ret::default());
        parse_gives(&mut e, true, &[0x7777]);
        start_log(&mut e);
        assert!(e.call(0x005d_47e0, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(
            calls(&e, ACTOR_UNIT_METHOD_008C0EC0),
            vec![vec![0x7777, actor, 0xf0f0, 0xffff_ffff]]
        );
        // A zero value, or no actor: nothing, still true.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_47e0, &args![a]).bool());
        parse_gives(&mut e, true, &[0x7777]);
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x005d_47e0, &args![a]).bool());
        assert!(calls(&e, ACTOR_UNIT_METHOD_008C0EC0).is_empty());
        assert_fails_when_unparsed(0x005d_47e0, ACTOR_UNIT_METHOD_008C0EC0);
    }

    #[test]
    fn toggle_cast_shadows_flips_the_bit_and_reports_the_state() {
        let mut e = engine();
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[3]);
        start_log(&mut e);
        assert!(e.call(0x005d_4870, &args![a]).bool());
        assert_eq!(e.mem.u32(SHADOW_CATEGORY_MASK), 0b1000);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        // The second call clears the bit again and says "Off".
        assert!(e.call(0x005d_4870, &args![a]).bool());
        assert_eq!(e.mem.u32(SHADOW_CATEGORY_MASK), 0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_SHADOWS, SHADOW_CATEGORY_NAMES[3], TEXT_OFF]]
        );
        // And once more sets it, with "On"; the other bits stay.
        e.mem.set_u32(SHADOW_CATEGORY_MASK, 0b10_0001);
        start_log(&mut e);
        assert!(e.call(0x005d_4870, &args![a]).bool());
        assert_eq!(e.mem.u32(SHADOW_CATEGORY_MASK), 0b10_1001);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_SHADOWS, SHADOW_CATEGORY_NAMES[3], TEXT_ON]]
        );
        assert_fails_when_unparsed(0x005d_4870, CONSOLE_PRINT);
    }

    #[test]
    fn toggle_cast_shadows_rejects_categories_outside_0_to_6() {
        for category in [7u32, 0xffff_ffff, 100] {
            let mut e = engine();
            e.mem.set_u32(SHADOW_CATEGORY_MASK, 0b101);
            let a = command(&mut e, 0);
            parse_gives(&mut e, true, &[category]);
            start_log(&mut e);
            assert!(e.call(0x005d_4870, &args![a]).bool());
            assert_eq!(e.mem.u32(SHADOW_CATEGORY_MASK), 0b101);
            assert!(calls(&e, CONSOLE_PRINT).is_empty());
            set_echo(&mut e, true);
            assert!(e.call(0x005d_4870, &args![a]).bool());
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_SHADOW_TYPE_RANGE]]);
        }
        // The edges 0 and 6 are accepted.
        for (category, mask) in [(0u32, 1u32), (6, 0x40)] {
            let mut e = engine();
            let a = command(&mut e, 0);
            parse_gives(&mut e, true, &[category]);
            assert!(e.call(0x005d_4870, &args![a]).bool());
            assert_eq!(e.mem.u32(SHADOW_CATEGORY_MASK), mask);
        }
    }

    #[test]
    fn fn_005d49c0_stores_whether_the_value_is_non_zero() {
        let mut e = engine_with_high_globals();
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[5]);
        assert!(e.call(0x005d_49c0, &args![a]).bool());
        assert_eq!(e.mem.u8(SAVED_FLAG_005D4A30), 1);
        assert_eq!(e.mem.u8(SWITCH_FLAG_005D49C0), 1);
        parse_gives(&mut e, true, &[0]);
        assert!(e.call(0x005d_49c0, &args![a]).bool());
        assert_eq!(e.mem.u8(SAVED_FLAG_005D4A30), 0);
        assert_eq!(e.mem.u8(SWITCH_FLAG_005D49C0), 0);
        parse_gives(&mut e, true, &[1]);
        assert!(e.call(0x005d_49c0, &args![a]).bool());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_49c0, &args![a]).bool());
        // Not parsed: the bytes stay.
        assert_eq!(e.mem.u8(SWITCH_FLAG_005D49C0), 1);
    }

    #[test]
    fn fn_005d4a30_stores_its_byte_and_fn_005d4a40_returns_one() {
        let mut e = engine();
        e.call(0x005d_4a30, &args![0x7fu8]);
        assert_eq!(e.mem.u8(SAVED_FLAG_005D4A30), 0x7f);
        assert_eq!(e.call(0x005d_4a40, &args![]).u8(), 1);
    }

    // ---- 005d4a50 .. 005d4cc0 --------------------------------------------

    /// The test of a "condition function, echo the result double, always
    /// true" command.
    fn check_condition_echo_always_true(command_address: u32, condition: u32, format: u32) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(condition, |e, a| {
            e.mem.set_f64(a[3], 1.5);
            false.into_ret()
        });
        start_log(&mut e);
        // The condition function's `AL` is ignored.
        assert!(e.call(command_address, &args![a]).bool());
        assert_eq!(
            calls(&e, condition),
            vec![vec![this_obj, 0, 0, a.result.addr()]]
        );
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(command_address, &args![a]).bool());
        let [low, high] = f64_words(1.5);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![format, low, high]]);
    }

    #[test]
    fn the_always_true_condition_commands_echo_their_result() {
        check_condition_echo_always_true(0x005d_4a50, IS_ESSENTIAL_CONDITION, MSG_IS_ESSENTIAL);
        check_condition_echo_always_true(0x005d_4aa0, IS_ACTOR_CONDITION, MSG_IS_ACTOR);
        check_condition_echo_always_true(
            0x005d_4af0,
            IS_PLAYER_MOVING_INTO_NEW_SPACE_CONDITION,
            MSG_PLAYER_MOVING_TO_NEW_AREA,
        );
        check_condition_echo_always_true(0x005d_4b40, GET_TIME_DEAD_CONDITION, MSG_TIME_DEAD);
    }

    #[test]
    fn fn_005d4b90_picks_the_path_by_the_setting() {
        let mut e = engine();
        let reference = object_with(&mut e, &[(REFR_SLOT_144, V_SLOT)]);
        let a = command(&mut e, reference);
        e.register(SEND_REFERENCE_EVENT, |_, _| Ret::default());
        e.register(GET_QUEUE_OWNER, |_, _| 0x8000u32.into_ret());
        let setting = e.mem.alloc(4);
        e.register_double(GET_SETTING_INTEGER, move |_, a| {
            assert_eq!(a, [EVENT_SETTING]);
            setting.into_ret()
        });
        parse_gives(&mut e, true, &[2.5f32.to_bits()]);

        // Setting at most 1: the virtual call with the float and 1.
        e.mem.set_u32(setting, 1);
        start_log(&mut e);
        assert!(e.call(0x005d_4b90, &args![a]).bool());
        assert_parsed(&e, reference);
        assert_eq!(
            calls(&e, V_SLOT),
            vec![vec![reference, 2.5f32.to_bits(), 1]]
        );
        assert!(calls(&e, SEND_REFERENCE_EVENT).is_empty());

        // Above 1: the event, with the float as a double.
        e.mem.set_u32(setting, 2);
        start_log(&mut e);
        assert!(e.call(0x005d_4b90, &args![a]).bool());
        let [low, high] = f64_words(2.5);
        assert_eq!(
            calls(&e, SEND_REFERENCE_EVENT),
            vec![vec![0x8000, 0x116c, reference, low, high]]
        );
        assert!(calls(&e, V_SLOT).is_empty());

        // No reference: false without anything else.
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(!e.call(0x005d_4b90, &args![a]).bool());
        assert!(calls(&e, GET_SETTING_INTEGER).is_empty());
        assert_fails_when_unparsed(0x005d_4b90, GET_SETTING_INTEGER);
    }

    #[test]
    fn fn_005d4c20_calls_slot_144_with_the_float_and_one() {
        let mut e = engine();
        let reference = object_with(&mut e, &[(REFR_SLOT_144, V_SLOT)]);
        start_log(&mut e);
        e.call(0x005d_4c20, &args![reference, 1.25f32]);
        assert_eq!(
            calls(&e, V_SLOT),
            vec![vec![reference, 1.25f32.to_bits(), 1]]
        );
    }

    #[test]
    fn fn_005d4c40_returns_the_condition_functions_al() {
        check_condition_only(0x005d_4c40, CONDITION_005A3AF0);
    }

    /// The test of a "parse one integer, give it to a condition function,
    /// return its `AL`" command.
    fn check_condition_with_argument(command_address: u32, condition: u32) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(condition, |_, _| false.into_ret());
        parse_gives(&mut e, true, &[9]);
        start_log(&mut e);
        assert!(!e.call(command_address, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(
            calls(&e, condition),
            vec![vec![this_obj, 9, 0, a.result.addr()]]
        );
        e.register(condition, |_, _| true.into_ret());
        assert!(e.call(command_address, &args![a]).bool());
        assert_fails_when_unparsed(command_address, condition);
    }

    #[test]
    fn the_commands_with_one_integer_return_the_condition_functions_al() {
        check_condition_with_argument(0x005d_4c60, IS_PLAYER_ACTION_ACTIVE_CONDITION);
        check_condition_with_argument(0x005d_4d30, CONDITION_005A3B90);
        check_condition_with_argument(0x005d_5330, CONDITION_005A3BF0);
    }

    #[test]
    fn fn_005d4cc0_calls_the_method_for_a_base_form_of_the_checked_type() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(GET_BASE_FORM, |_, _| 0xf0f0u32.into_ret());
        e.register(FORM_TYPE_CHECKED_METHOD, |_, _| Ret::default());
        e.register_double(FORM_TYPE_BYTE, |_, a| {
            assert_eq!(a, [0xf0f0]);
            // The code reads a byte: the other bytes of the word are junk.
            0xaaaa_aa16u32.into_ret()
        });
        parse_gives(&mut e, true, &[0x55]);
        start_log(&mut e);
        assert!(e.call(0x005d_4cc0, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(
            calls(&e, FORM_TYPE_CHECKED_METHOD),
            vec![vec![0xf0f0, 0x55]]
        );
        // Another type: nothing.
        e.register(FORM_TYPE_BYTE, |_, _| 0x15u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_4cc0, &args![a]).bool());
        assert!(calls(&e, FORM_TYPE_CHECKED_METHOD).is_empty());
        // The parse result is not checked: the command still succeeds, and the
        // value is whatever the parse left.
        e.register(FORM_TYPE_BYTE, |_, _| 0x16u32.into_ret());
        parse_gives(&mut e, false, &[0x66]);
        start_log(&mut e);
        assert!(e.call(0x005d_4cc0, &args![a]).bool());
        assert_eq!(
            calls(&e, FORM_TYPE_CHECKED_METHOD),
            vec![vec![0xf0f0, 0x66]]
        );
        // No reference: nothing.
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_4cc0, &args![a]).bool());
        assert!(calls(&e, GET_BASE_FORM).is_empty());
    }

    // ---- 005d4d90 .. 005d5390 --------------------------------------------

    const V_RANK: u32 = 0x0900_0006;
    const V_SET_RANK: u32 = 0x0900_0007;
    const V_SLOT_49C: u32 = 0x0900_0008;

    /// An actor whose slots for the perk commands are the fake functions.
    fn perk_actor(e: &mut Engine, is_actor: bool, rank: u8) -> u32 {
        e.register_double(V_IS_ACTOR, move |_, _| is_actor.into_ret());
        e.register_double(V_RANK, move |_, _| (rank as u32).into_ret());
        e.register(V_SET_RANK, |_, _| Ret::default());
        e.register(V_SLOT_49C, |_, _| Ret::default());
        object_with(
            e,
            &[
                (REFR_IS_ACTOR_SLOT, V_IS_ACTOR),
                (ACTOR_GET_PERK_RANK_SLOT, V_RANK),
                (ACTOR_SET_PERK_RANK_SLOT, V_SET_RANK),
                (ACTOR_PERK_SLOT_49C, V_SLOT_49C),
            ],
        )
    }

    /// A perk object with the given highest rank at `+0x3a`.
    fn perk_with_max_rank(e: &mut Engine, max_rank: u8) -> u32 {
        let perk = e.mem.alloc(0x100);
        e.mem.set_u8(perk + 0x3a, max_rank);
        perk
    }

    #[test]
    fn add_perk_raises_the_rank_below_the_maximum() {
        let mut e = engine();
        let actor = perk_actor(&mut e, true, 1);
        let perk = perk_with_max_rank(&mut e, 3);
        let a = command(&mut e, actor);
        e.register(GET_NAME_TEXT, |_, _| 0xbbbbu32.into_ret());
        parse_gives(&mut e, true, &[perk, 4]);
        start_log(&mut e);
        assert!(e.call(0x005d_4d90, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, V_RANK), vec![vec![actor, perk, 1]]);
        assert_eq!(calls(&e, V_SET_RANK), vec![vec![actor, perk, 2, 1]]);
        assert_eq!(e.mem.f64(a.result.addr()), 2.0);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // With the echo flag: the perk name component is `perk + 0x18`.
        set_echo(&mut e, true);
        assert!(e.call(0x005d_4d90, &args![a]).bool());
        assert_eq!(calls(&e, GET_NAME_TEXT), vec![vec![perk + 0x18]]);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_ADDED_PERK, 0xbbbb, 0xaaaa, 2]]
        );
    }

    #[test]
    fn add_perk_keeps_a_rank_that_reached_the_maximum() {
        let mut e = engine();
        let actor = perk_actor(&mut e, true, 3);
        let perk = perk_with_max_rank(&mut e, 3);
        let a = command(&mut e, actor);
        e.mem.set_f64(a.result.addr(), -1.0);
        parse_gives(&mut e, true, &[perk, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_4d90, &args![a]).bool());
        assert!(calls(&e, V_SET_RANK).is_empty());
        assert_eq!(e.mem.f64(a.result.addr()), 3.0);
        assert_eq!(calls(&e, V_RANK), vec![vec![actor, perk, 0]]);
    }

    #[test]
    fn add_perk_does_nothing_without_a_perk_or_an_actor() {
        let mut e = engine();
        let actor = perk_actor(&mut e, false, 1);
        let perk = perk_with_max_rank(&mut e, 3);
        let a = command(&mut e, actor);
        e.mem.set_f64(a.result.addr(), -1.0);
        parse_gives(&mut e, true, &[perk, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_4d90, &args![a]).bool());
        assert!(calls(&e, V_RANK).is_empty());
        assert_eq!(e.mem.f64(a.result.addr()), -1.0);
        let actor = perk_actor(&mut e, true, 1);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[0, 0]);
        assert!(e.call(0x005d_4d90, &args![a]).bool());
        assert!(calls(&e, V_RANK).is_empty());
        assert_fails_when_unparsed(0x005d_4d90, V_RANK);
    }

    #[test]
    fn fn_005d4ee0_reads_the_byte_at_0x3a() {
        let mut e = engine();
        let perk = perk_with_max_rank(&mut e, 7);
        assert_eq!(e.call(0x005d_4ee0, &args![perk]).u8(), 7);
    }

    #[test]
    fn fn_005d4f00_calls_slot_49c_for_a_perk_on_an_actor() {
        let mut e = engine();
        let actor = perk_actor(&mut e, true, 0);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[0x4444, 9]);
        start_log(&mut e);
        assert!(e.call(0x005d_4f00, &args![a]).bool());
        assert_parsed(&e, actor);
        // The flag is reduced to 0 or 1.
        assert_eq!(calls(&e, V_SLOT_49C), vec![vec![actor, 0x4444, 1]]);
        parse_gives(&mut e, true, &[0, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_4f00, &args![a]).bool());
        assert!(calls(&e, V_SLOT_49C).is_empty());
        let actor = perk_actor(&mut e, false, 0);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[0x4444, 0]);
        assert!(e.call(0x005d_4f00, &args![a]).bool());
        assert!(calls(&e, V_SLOT_49C).is_empty());
        assert_fails_when_unparsed(0x005d_4f00, V_SLOT_49C);
    }

    #[test]
    fn has_perk_returns_the_condition_functions_al() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(HAS_PERK_CONDITION, |_, _| true.into_ret());
        parse_gives(&mut e, true, &[0x4444, 2]);
        start_log(&mut e);
        assert!(e.call(0x005d_4fa0, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(
            calls(&e, HAS_PERK_CONDITION),
            vec![vec![this_obj, 0x4444, 2, a.result.addr()]]
        );
        e.register(HAS_PERK_CONDITION, |_, _| false.into_ret());
        assert!(!e.call(0x005d_4fa0, &args![a]).bool());
        assert_fails_when_unparsed(0x005d_4fa0, HAS_PERK_CONDITION);
    }

    /// The doubles of the debug page command: memset, the visibility, the
    /// table (count 3 at `+0x18`) and the page call. Returns the table.
    fn debug_page_scene(e: &mut Engine, visible: bool) -> u32 {
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(table + 0x18, 3);
        e.register(MEMSET, |_, _| Ret::default());
        e.register_double(INTERFACE_IS_DEBUG_TEXT_VISIBLE, move |_, _| {
            visible.into_ret()
        });
        e.register_double(DEBUG_TEXT_TABLE_GETTER, move |_, _| table.into_ret());
        e.register(DEBUG_TEXT_TABLE_SEND_PAGE, |_, _| Ret::default());
        table
    }

    #[test]
    fn sherlock_page_uses_the_parsed_name_as_the_page() {
        let mut e = engine();
        let table = debug_page_scene(&mut e, false);
        let reference = object(&mut e);
        let a = command(&mut e, reference);
        // The parse writes a non-empty name into the buffer.
        parse_gives(&mut e, true, &[0x6e61_6d65]);
        start_log(&mut e);
        assert!(e.call(0x005d_5020, &args![a]).bool());
        assert_parsed(&e, reference);
        let buffer = calls(&e, PARSE_PARAMETERS)[0][7];
        // The buffer is cleared first: memset after the first word.
        assert_eq!(calls(&e, MEMSET), vec![vec![buffer + 4, 0, 0x7fc]]);
        assert_eq!(
            calls(&e, DEBUG_TEXT_TABLE_SEND_PAGE),
            vec![vec![table, buffer, reference]]
        );
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn sherlock_page_asks_for_a_name_when_debug_text_is_hidden() {
        let mut e = engine();
        debug_page_scene(&mut e, false);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_5020, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_DEBUG_PAGE_NAME_REQUIRED]]
        );
        assert!(calls(&e, DEBUG_TEXT_TABLE_SEND_PAGE).is_empty());
    }

    #[test]
    fn sherlock_page_uses_the_table_slot_when_visible_without_a_name() {
        let mut e = engine();
        let table = debug_page_scene(&mut e, true);
        let player = set_player(&mut e);
        // No `thisObj`: the player stands in.
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_5020, &args![a]).bool());
        assert_eq!(
            calls(&e, DEBUG_TEXT_TABLE_SEND_PAGE),
            vec![vec![table, table + 0x20 + 3 * 0x44, player]]
        );
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
    }

    #[test]
    fn sherlock_page_succeeds_even_when_the_parameters_do_not_parse() {
        let mut e = engine();
        debug_page_scene(&mut e, true);
        let a = command(&mut e, 0);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(e.call(0x005d_5020, &args![a]).bool());
        assert!(calls(&e, DEBUG_TEXT_TABLE_SEND_PAGE).is_empty());
        assert!(calls(&e, INTERFACE_IS_DEBUG_TEXT_VISIBLE).is_empty());
    }

    #[test]
    fn fn_005d5120_gives_the_entry_after_the_last() {
        let mut e = engine();
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(table + 0x18, 0);
        assert_eq!(e.call(0x005d_5120, &args![table]).u32(), table + 0x20);
        e.mem.set_u32(table + 0x18, 5);
        assert_eq!(
            e.call(0x005d_5120, &args![table]).u32(),
            table + 0x20 + 5 * 0x44
        );
    }

    #[test]
    fn fn_005d5140_calls_slot_488_of_the_player() {
        let mut e = engine();
        let player = object_with(&mut e, &[(PLAYER_SLOT_488, V_SLOT)]);
        e.set_global(PLAYER, player);
        let a = command(&mut e, 0);
        parse_gives(&mut e, true, &[12]);
        start_log(&mut e);
        assert!(e.call(0x005d_5140, &args![a]).bool());
        assert_eq!(calls(&e, V_SLOT), vec![vec![player, 12]]);
        assert_fails_when_unparsed(0x005d_5140, V_SLOT);
    }

    #[test]
    fn reward_karma_rewards_the_player() {
        let mut e = engine();
        let player = set_player(&mut e);
        let a = command(&mut e, 0);
        e.register(PLAYER_REWARD_KARMA, |_, _| Ret::default());
        parse_gives(&mut e, true, &[0xffff_fff6]);
        start_log(&mut e);
        assert!(e.call(0x005d_51a0, &args![a]).bool());
        assert_eq!(
            calls(&e, PLAYER_REWARD_KARMA),
            vec![vec![player, 0xffff_fff6]]
        );
        assert_fails_when_unparsed(0x005d_51a0, PLAYER_REWARD_KARMA);
    }

    #[test]
    fn fn_005d5200_calls_the_function_for_an_actor_only() {
        let mut e = engine();
        e.register(INTERFACE_FUNCTION_00704690, |_, _| Ret::default());
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        let actor = object_with(&mut e, &[(REFR_IS_ACTOR_SLOT, V_IS_ACTOR)]);
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005d_5200, &args![a]).bool());
        assert_eq!(calls(&e, INTERFACE_FUNCTION_00704690), vec![vec![actor]]);
        e.register(V_IS_ACTOR, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_5200, &args![a]).bool());
        assert!(calls(&e, INTERFACE_FUNCTION_00704690).is_empty());
        let a = command(&mut e, 0);
        assert!(e.call(0x005d_5200, &args![a]).bool());
        assert!(calls(&e, INTERFACE_FUNCTION_00704690).is_empty());
    }

    #[test]
    fn add_note_passes_the_echo_byte() {
        let mut e = engine();
        let player = set_player(&mut e);
        let a = command(&mut e, 0);
        e.register(PLAYER_ADD_NOTE, |_, _| Ret::default());
        parse_gives(&mut e, true, &[0x3300]);
        start_log(&mut e);
        assert!(e.call(0x005d_5230, &args![a]).bool());
        assert_eq!(calls(&e, PLAYER_ADD_NOTE), vec![vec![player, 0x3300, 0]]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_5230, &args![a]).bool());
        assert_eq!(calls(&e, PLAYER_ADD_NOTE), vec![vec![player, 0x3300, 1]]);
        // A null note: nothing.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_5230, &args![a]).bool());
        assert!(calls(&e, PLAYER_ADD_NOTE).is_empty());
        assert_fails_when_unparsed(0x005d_5230, PLAYER_ADD_NOTE);
    }

    #[test]
    fn remove_note_removes_it_and_reports_the_name() {
        let mut e = engine();
        let player = set_player(&mut e);
        let a = command(&mut e, 0);
        e.register(PLAYER_REMOVE_NOTE, |_, _| Ret::default());
        e.register(GET_NAME_TEXT, |_, _| 0xccccu32.into_ret());
        parse_gives(&mut e, true, &[0x3300]);
        start_log(&mut e);
        assert!(e.call(0x005d_52a0, &args![a]).bool());
        assert_parsed(&e, 0);
        assert_eq!(calls(&e, PLAYER_REMOVE_NOTE), vec![vec![player, 0x3300]]);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005d_52a0, &args![a]).bool());
        assert_eq!(calls(&e, GET_NAME_TEXT), vec![vec![0x3300 + 0x48]]);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_REMOVED_NOTE, 0xcccc]]
        );
        // A null note is not removed, but the message is still printed.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005d_52a0, &args![a]).bool());
        assert!(calls(&e, PLAYER_REMOVE_NOTE).is_empty());
        assert_eq!(calls(&e, GET_NAME_TEXT), vec![vec![0x48]]);
        assert_fails_when_unparsed(0x005d_52a0, PLAYER_REMOVE_NOTE);
    }

    #[test]
    fn fn_005d5390_calls_the_method_for_an_actor() {
        let mut e = engine();
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        let actor = object_with(&mut e, &[(REFR_IS_ACTOR_SLOT, V_IS_ACTOR)]);
        let a = command(&mut e, actor);
        e.register(ACTOR_UNIT_METHOD_008B8E20, |_, _| Ret::default());
        // The second value defaults to -1: the parse double leaves it alone.
        e.register_double(PARSE_PARAMETERS, |e, a| {
            assert_eq!(e.mem.u32(a[7]), 0);
            assert_eq!(e.mem.u32(a[8]), 0xffff_ffff);
            e.mem.set_u32(a[7], 0x5151);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005d_5390, &args![a]).bool());
        assert_eq!(
            calls(&e, ACTOR_UNIT_METHOD_008B8E20),
            vec![vec![actor, 0x5151, 0xffff_ffff]]
        );
        // A zero first value, or not an actor: nothing.
        parse_gives(&mut e, true, &[0, 5]);
        start_log(&mut e);
        assert!(e.call(0x005d_5390, &args![a]).bool());
        parse_gives(&mut e, true, &[0x5151, 5]);
        e.register(V_IS_ACTOR, |_, _| false.into_ret());
        assert!(e.call(0x005d_5390, &args![a]).bool());
        assert!(calls(&e, ACTOR_UNIT_METHOD_008B8E20).is_empty());
        assert_fails_when_unparsed(0x005d_5390, ACTOR_UNIT_METHOD_008B8E20);
    }

    // ---- 005d5420 and later ------------------------------------------------

    const V_NOT_ACTOR: u32 = 0x0900_0020;
    const V_GET_VALUE: u32 = 0x0900_0021;
    const V_MOD_VALUE: u32 = 0x0900_0022;
    const V_CONTROLLER: u32 = 0x0900_0023;
    const V_NAME_SLOT: u32 = 0x0900_0024;
    const V_VECTOR: u32 = 0x0900_0025;
    const V_PROCESS_PUSH: u32 = 0x0900_0026;
    const V_PROCESS_LIST: u32 = 0x0900_0027;
    const V_SLOT_434: u32 = 0x0900_0028;
    const V_SLOT_19C: u32 = 0x0900_0029;
    const V_QUEST_NAME: u32 = 0x0900_002a;
    const V_MARK_CHANGED: u32 = 0x0900_002b;

    /// An actor (its slot `0x100` answers true) with the given extra slots.
    fn actor_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        e.register(V_IS_ACTOR, |_, _| true.into_ret());
        let mut all = vec![(REFR_IS_ACTOR_SLOT, V_IS_ACTOR)];
        all.extend_from_slice(slots);
        object_with(e, &all)
    }

    /// A reference that is not an actor, with the given extra slots.
    fn non_actor_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        e.register(V_NOT_ACTOR, |_, _| false.into_ret());
        let mut all = vec![(REFR_IS_ACTOR_SLOT, V_NOT_ACTOR)];
        all.extend_from_slice(slots);
        object_with(e, &all)
    }

    /// The words of a `float` as the logged call arguments.
    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// Makes the name function `00474cb0` answer `length` and slot `0x130`
    /// answer the text address `text`.
    fn name_functions(e: &mut Engine, length: u32, text: u32) {
        e.register_double(NAME_TEXT_LENGTH, move |_, _| length.into_ret());
        e.register_double(V_NAME_SLOT, move |_, _| text.into_ret());
        e.register(GET_FULL_NAME, |_, _| 0xf0f0u32.into_ret());
    }

    #[test]
    fn fn_005d5420_calls_the_method_for_an_actor_with_a_value() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[]);
        let a = command(&mut e, actor);
        e.register(ACTOR_UNIT_METHOD_008B8F20, |_, _| Ret::default());
        parse_gives(&mut e, true, &[0x4242]);
        start_log(&mut e);
        assert!(e.call(0x005d_5420, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(
            calls(&e, ACTOR_UNIT_METHOD_008B8F20),
            vec![vec![actor, 0x4242]]
        );
        // A zero value: nothing, and the actor test is not even made.
        parse_gives(&mut e, true, &[0]);
        e.register(V_IS_ACTOR, |_, _| panic!("not asked"));
        start_log(&mut e);
        assert!(e.call(0x005d_5420, &args![a]).bool());
        assert!(calls(&e, ACTOR_UNIT_METHOD_008B8F20).is_empty());
        // Not an actor.
        let plain = non_actor_with(&mut e, &[]);
        let a = command(&mut e, plain);
        parse_gives(&mut e, true, &[0x4242]);
        start_log(&mut e);
        assert!(e.call(0x005d_5420, &args![a]).bool());
        assert!(calls(&e, ACTOR_UNIT_METHOD_008B8F20).is_empty());
        assert_fails_when_unparsed(0x005d_5420, ACTOR_UNIT_METHOD_008B8F20);
    }

    #[test]
    fn fn_005d54a0_changes_the_value_by_the_difference() {
        let mut e = engine();
        e.register(V_GET_VALUE, |_, _| 10u32.into_ret());
        e.register(V_MOD_VALUE, |_, _| Ret::default());
        let actor = actor_with(
            &mut e,
            &[
                (ACTOR_GET_VALUE_SLOT, V_GET_VALUE),
                (ACTOR_MOD_VALUE_SLOT, V_MOD_VALUE),
            ],
        );
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[0x77, 25]);
        start_log(&mut e);
        assert!(e.call(0x005d_54a0, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(calls(&e, V_GET_VALUE), vec![vec![actor, 0x77, 0]]);
        assert_eq!(calls(&e, V_MOD_VALUE), vec![vec![actor, 0x77, bits(15.0)]]);
        // A smaller wanted value gives a negative difference.
        parse_gives(&mut e, true, &[0x77, 4]);
        start_log(&mut e);
        assert!(e.call(0x005d_54a0, &args![a]).bool());
        assert_eq!(calls(&e, V_MOD_VALUE), vec![vec![actor, 0x77, bits(-6.0)]]);
        // No form: nothing.
        parse_gives(&mut e, true, &[0, 4]);
        start_log(&mut e);
        assert!(e.call(0x005d_54a0, &args![a]).bool());
        assert!(calls(&e, V_MOD_VALUE).is_empty());
        // Not an actor: nothing.
        let plain = non_actor_with(&mut e, &[(ACTOR_MOD_VALUE_SLOT, V_MOD_VALUE)]);
        let a = command(&mut e, plain);
        parse_gives(&mut e, true, &[0x77, 4]);
        start_log(&mut e);
        assert!(e.call(0x005d_54a0, &args![a]).bool());
        assert!(calls(&e, V_MOD_VALUE).is_empty());
        // The parameters do not parse.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_54a0, &args![a]).bool());
    }

    #[test]
    fn set_global_time_multiplier_sets_the_timer_and_the_audio_flag() {
        let mut e = engine();
        let audio = e.mem.alloc(0x200);
        e.register(TIMER_SET_GLOBAL_TIME_MULTIPLIER, |_, _| Ret::default());
        e.register_double(AUDIO_MANAGER_QUERY_INSTANCE, move |_, _| audio.into_ret());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register_double(PARSE_PARAMETERS, |e, a| {
            // The multiplier starts at 1.0 and the flag at 0.
            assert_eq!(e.mem.u32(a[7]), bits(1.0));
            assert_eq!(e.mem.u32(a[8]), 0);
            e.mem.set_u32(a[7], bits(2.5));
            e.mem.set_u32(a[8], 7);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005d_5560, &args![a]).bool());
        assert_eq!(
            calls(&e, TIMER_SET_GLOBAL_TIME_MULTIPLIER),
            vec![vec![TIMER, bits(2.5), 1]]
        );
        assert_eq!(e.mem.u8(audio + 0x184), 1);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // A zero flag clears the byte; with the echo flag the value is printed.
        parse_gives(&mut e, true, &[bits(0.5), 0]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_5560, &args![a]).bool());
        assert_eq!(e.mem.u8(audio + 0x184), 0);
        let [low, high] = f64_words(0.5);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_GLOBAL_TIME_MULTIPLIER, low, high]]
        );
        assert_fails_when_unparsed(0x005d_5560, TIMER_SET_GLOBAL_TIME_MULTIPLIER);
    }

    #[test]
    fn fn_005d5610_stores_the_flag_byte() {
        let mut e = engine();
        let object = e.mem.alloc(0x200);
        e.call(0x005d_5610, &args![object, 3u32]);
        assert_eq!(e.mem.u8(object + 0x184), 3);
        e.call(0x005d_5610, &args![object, 0u32]);
        assert_eq!(e.mem.u8(object + 0x184), 0);
    }

    /// The echo commands of `005d5630`, `005d5680`, `005d56d0`, `005d7940`,
    /// `005d7990` and `005d79e0`: the condition function gets `thisObj, 0, 0,
    /// result`, the command prints `format` with the result double when the
    /// echo flag is set, and is true whatever the condition function says.
    fn check_echo_command(command_address: u32, condition: u32, format: u32) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.mem.set_f64(a.result.addr(), 3.5);
        e.register(condition, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        assert_eq!(
            calls(&e, condition),
            vec![vec![this_obj, 0, 0, a.result.addr()]]
        );
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        let [low, high] = f64_words(3.5);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![format, low, high]]);
    }

    #[test]
    fn get_hit_location_echoes_the_condition_result() {
        check_echo_command(
            0x005d_5630,
            GET_HIT_LOCATION_CONDITION,
            MSG_GET_HIT_LOCATION,
        );
    }

    #[test]
    fn get_last_hit_critical_echoes_the_condition_result() {
        check_echo_command(
            0x005d_5680,
            GET_LAST_HIT_CRITICAL_CONDITION,
            MSG_GET_LAST_HIT_CRITICAL,
        );
    }

    #[test]
    fn is_pc_1st_person_echoes_the_condition_result() {
        check_echo_command(
            0x005d_56d0,
            IS_PC_1ST_PERSON_CONDITION,
            MSG_IS_PC_1ST_PERSON,
        );
    }

    #[test]
    fn fn_005d5720_calls_the_combat_controller_of_an_actor() {
        let mut e = engine();
        e.register(V_CONTROLLER, |_, _| 0xc0c0u32.into_ret());
        let actor = actor_with(&mut e, &[(ACTOR_GET_COMBAT_CONTROLLER_SLOT, V_CONTROLLER)]);
        let a = command(&mut e, actor);
        e.register(COMBAT_CONTROLLER_FUNCTION_0097FB80, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_5720, &args![a]).bool());
        assert_eq!(
            calls(&e, COMBAT_CONTROLLER_FUNCTION_0097FB80),
            vec![vec![0xc0c0]]
        );
        // No controller.
        e.register(V_CONTROLLER, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_5720, &args![a]).bool());
        assert!(calls(&e, COMBAT_CONTROLLER_FUNCTION_0097FB80).is_empty());
        // Not an actor: the controller slot is not asked.
        let plain = non_actor_with(&mut e, &[]);
        let a = command(&mut e, plain);
        start_log(&mut e);
        assert!(e.call(0x005d_5720, &args![a]).bool());
        assert!(calls(&e, COMBAT_CONTROLLER_FUNCTION_0097FB80).is_empty());
    }

    #[test]
    fn set_combat_action_cost_reports_the_action() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register_double(PARSE_PARAMETERS, |e, a| {
            // The float starts at 0.0.
            assert_eq!(e.mem.u32(a[8]), 0);
            e.mem.set_cstr(a[7], b"flee");
            e.mem.set_f32(a[8], 1.5);
            true.into_ret()
        });
        e.register_double(COMBAT_ACTION_SET_COST, |e, a| {
            assert_eq!(e.mem.cstr(a[0]), b"flee");
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005d_5780, &args![a]).bool());
        let buffer = calls(&e, COMBAT_ACTION_SET_COST)[0][0];
        assert_eq!(
            calls(&e, COMBAT_ACTION_SET_COST),
            vec![vec![buffer, bits(1.5)]]
        );
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // Found, with the echo flag.
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_5780, &args![a]).bool());
        let [low, high] = f64_words(1.5);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_NAME_VALUE, buffer, low, high]]
        );
        // Not found.
        e.register(COMBAT_ACTION_SET_COST, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_5780, &args![a]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_NOT_FOUND, buffer]]);
        // The parameters do not parse: false and no echo.
        assert_fails_when_unparsed(0x005d_5780, COMBAT_ACTION_SET_COST);
    }

    // ---- ToggleCombatDebug (005d5850) --------------------------------------

    /// The global objects of the combat debug displays (value byte at
    /// `+4`), the text functions and the string compare standing in for the
    /// game's, and the names of the options written into the string data.
    fn combat_debug_engine() -> Engine {
        let mut e = engine();
        e.map(0x011f_1000, 0x1_0000);
        e.register(GLOBAL_VALUE_ADDRESS, |_, a| (a[0] + 4).into_ret());
        e.register(GLOBAL_VALUE_SET, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret::default()
        });
        e.register(STRING_COPY, |e, a| {
            let text = e.mem.cstr(a[1]);
            e.mem.set_cstr(a[0], &text);
            a[0].into_ret()
        });
        e.register(STRING_LENGTH, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(STRICMP, |e, a| {
            let first = e.mem.cstr(a[0]).to_ascii_lowercase();
            let second = e.mem.cstr(a[1]).to_ascii_lowercase();
            (first.cmp(&second) as i32 as u32).into_ret()
        });
        for (address, name) in COMBAT_OPTION_NAMES {
            e.mem.set_cstr(address, name.as_bytes());
        }
        e.mem.set_cstr(TEXT_ON, b"On");
        e.mem.set_cstr(TEXT_OFF, b"Off");
        e
    }

    /// Every name `ToggleCombatDebug` knows, its address in the string data.
    const COMBAT_OPTION_NAMES: [(u32, &str); 24] = [
        (0x0103_cacc, "cover"),
        (0x0103_ca90, "covertext"),
        (0x0103_ca58, "cover2"),
        (0x0103_ca4c, "coversearch"),
        (0x0103_ca18, "cover2text"),
        (0x0103_ca08, "coversearchtext"),
        (0x0103_c9d4, "cover3"),
        (0x0103_c9c4, "coversearch2"),
        (0x0103_c988, "threats"),
        (0x0103_c95c, "groups"),
        (0x0103_c930, "groups2"),
        (0x0103_c904, "targets"),
        (0x0103_c8d8, "search"),
        (0x0103_c8ac, "guard"),
        (0x0103_c87c, "cluster"),
        (0x0103_c870, "clusters"),
        (0x0103_c83c, "text"),
        (0x0103_c808, "color"),
        (0x0103_c7d0, "size"),
        (0x0103_c798, "unreach"),
        (0x0103_c78c, "unreachable"),
        (0x0103_c750, "events"),
        (0x0103_c71c, "plos"),
        (0x0103_c6e8, "area"),
    ];

    /// Runs `005d5850` with the given text (the parse puts it in the buffer) and
    /// returns the command's result.
    fn run_combat_debug(e: &mut Engine, text: &str) -> bool {
        let this_obj = object(e);
        let a = command(e, this_obj);
        let text = text.as_bytes().to_vec();
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            // The buffer is empty before the parse.
            assert_eq!(e.mem.cstr(a[7]), b"");
            e.mem.set_cstr(a[7], &text);
            true.into_ret()
        });
        start_log(e);
        e.call(0x005d_5850, &args![a]).bool()
    }

    #[test]
    fn toggle_combat_debug_without_a_name_switches_all_displays() {
        let mut e = combat_debug_engine();
        // None on: all go on, in the order the game writes them.
        assert!(run_combat_debug(&mut e, ""));
        let writes = calls(&e, GLOBAL_VALUE_SET);
        assert_eq!(writes.len(), 17);
        let order: Vec<u32> = writes.iter().map(|w| w[0]).collect();
        assert_eq!(order, COMBAT_DEBUG_WRITE_ORDER.to_vec());
        assert!(writes.iter().all(|w| w[1] == 1));
        assert!(COMBAT_DEBUG_WRITE_ORDER
            .iter()
            .all(|g| e.mem.u8(g + 4) == 1));
        // The reading looked at every display (none was on).
        assert_eq!(calls(&e, GLOBAL_VALUE_ADDRESS).len(), 17);
        // One on (the sixth one asked): all go off, and the reading stops
        // there.
        e.mem.set_u8(COMBAT_DEBUG_READ_ORDER[5] + 4, 1);
        for global in COMBAT_DEBUG_READ_ORDER {
            if global != COMBAT_DEBUG_READ_ORDER[5] {
                e.mem.set_u8(global + 4, 0);
            }
        }
        set_echo(&mut e, true);
        assert!(run_combat_debug(&mut e, ""));
        assert_eq!(calls(&e, GLOBAL_VALUE_ADDRESS).len(), 6);
        assert!(calls(&e, GLOBAL_VALUE_SET).iter().all(|w| w[1] == 0));
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_ALL_COMBAT_DEBUGGING, TEXT_OFF]]
        );
        // And on again with the echo text "On".
        assert!(run_combat_debug(&mut e, ""));
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_ALL_COMBAT_DEBUGGING, TEXT_ON]]
        );
        // The parameters do not parse.
        parse_gives(&mut e, false, &[]);
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        assert!(!e.call(0x005d_5850, &args![a]).bool());
    }

    #[test]
    fn toggle_combat_debug_switches_the_display_of_a_name() {
        // (name, display, message), as the disassembly of 005d5850 gives them.
        let expected: [(&str, u32, u32); 23] = [
            ("cover", 0x011f_1680, 0x0103_ca9c),
            ("covertext", 0x011f_16c0, 0x0103_ca60),
            ("cover2", 0x011f_1674, 0x0103_ca24),
            ("COVERSEARCH", 0x011f_1674, 0x0103_ca24),
            ("cover2text", 0x011f_16e0, 0x0103_c9dc),
            ("coversearchtext", 0x011f_16e0, 0x0103_c9dc),
            ("cover3", 0x011f_168c, 0x0103_c990),
            ("coversearch2", 0x011f_168c, 0x0103_c990),
            ("threats", 0x011f_1ba0, 0x0103_c964),
            ("groups", 0x011f_1878, 0x0103_c938),
            ("groups2", 0x011f_1858, 0x0103_c90c),
            ("targets", 0x011f_1b24, 0x0103_c8e0),
            ("search", 0x011f_1814, 0x0103_c8b4),
            ("guard", 0x011f_15b0, 0x0103_c884),
            ("cluster", 0x011f_1824, 0x0103_c844),
            ("clusters", 0x011f_1824, 0x0103_c844),
            ("text", 0x011f_15a4, 0x0103_c810),
            ("unreach", 0x011f_15d0, 0x0103_c758),
            ("unreachable", 0x011f_15d0, 0x0103_c758),
            ("events", 0x011f_197c, 0x0103_c724),
            ("plos", 0x011f_1c4c, 0x0103_c6f0),
            ("area", 0x011f_15ec, 0x0103_c6c8),
            ("Area", 0x011f_15ec, 0x0103_c6c8),
        ];
        for (name, global, message) in expected {
            let mut e = combat_debug_engine();
            assert!(run_combat_debug(&mut e, name), "{name}");
            // Off becomes on, and only that display was written.
            assert_eq!(calls(&e, GLOBAL_VALUE_SET), vec![vec![global, 1]], "{name}");
            assert_eq!(e.mem.u8(global + 4), 1, "{name}");
            assert!(calls(&e, CONSOLE_PRINT).is_empty());
            // Switched back, with the echo flag.
            set_echo(&mut e, true);
            assert!(run_combat_debug(&mut e, name));
            assert_eq!(e.mem.u8(global + 4), 0, "{name}");
            assert_eq!(
                calls(&e, CONSOLE_PRINT),
                vec![vec![message, TEXT_OFF]],
                "{name}"
            );
            assert!(run_combat_debug(&mut e, name));
            assert_eq!(
                calls(&e, CONSOLE_PRINT),
                vec![vec![message, TEXT_ON]],
                "{name}"
            );
        }
    }

    #[test]
    fn toggle_combat_debug_changes_the_text_colour() {
        let mut e = combat_debug_engine();
        assert!(run_combat_debug(&mut e, "color"));
        assert_eq!(
            calls(&e, GLOBAL_VALUE_SET),
            vec![vec![COMBAT_TEXT_COLOUR_GLOBAL, 1]]
        );
        set_echo(&mut e, true);
        assert!(run_combat_debug(&mut e, "color"));
        assert_eq!(e.mem.u8(COMBAT_TEXT_COLOUR_GLOBAL + 4), 0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_COMBAT_TEXT_COLOUR, TEXT_LIGHT]]
        );
        assert!(run_combat_debug(&mut e, "color"));
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_COMBAT_TEXT_COLOUR, TEXT_DARK]]
        );
    }

    #[test]
    fn toggle_combat_debug_lowers_the_text_size() {
        let mut e = combat_debug_engine();
        e.set_global(COMBAT_TEXT_SIZE_STEP, f64::from(0.1f32));
        e.set_global(COMBAT_TEXT_SIZE_MINIMUM, f64::from(0.05f32));
        e.register(GLOBAL_FLOAT_ADDRESS, |_, a| (a[0] + 4).into_ret());
        e.register(GLOBAL_FLOAT_SET, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let size = COMBAT_TEXT_SIZE_GLOBAL + 4;
        e.mem.set_f32(size, 0.5);
        assert!(run_combat_debug(&mut e, "size"));
        let lowered = (f64::from(0.5f32) - f64::from(0.1f32)) as f32;
        assert_eq!(e.mem.f32(size), lowered);
        assert_eq!(
            calls(&e, GLOBAL_FLOAT_SET),
            vec![vec![COMBAT_TEXT_SIZE_GLOBAL, bits(lowered)]]
        );
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // At 0.1 the next step gives 0.0, which is at most 0.05: back to 1.0.
        e.mem.set_f32(size, 0.1);
        set_echo(&mut e, true);
        assert!(run_combat_debug(&mut e, "size"));
        assert_eq!(e.mem.f32(size), 1.0);
        assert_eq!(calls(&e, GLOBAL_FLOAT_SET).len(), 2);
        let [low, high] = f64_words(1.0);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_COMBAT_TEXT_SIZE, low, high]]
        );
        // 0.15 - 0.1 is just above the minimum as a float (0.050000004 against
        // 0.05): kept. 0.14 - 0.1 is below it: reset.
        e.mem.set_f32(size, 0.15);
        assert!(run_combat_debug(&mut e, "size"));
        assert_eq!(
            e.mem.f32(size),
            (f64::from(0.15f32) - f64::from(0.1f32)) as f32
        );
        assert_eq!(calls(&e, GLOBAL_FLOAT_SET).len(), 1);
        e.mem.set_f32(size, 0.14);
        assert!(run_combat_debug(&mut e, "size"));
        assert_eq!(e.mem.f32(size), 1.0);
    }

    #[test]
    fn toggle_combat_debug_reports_unknown_names() {
        let mut e = combat_debug_engine();
        assert!(run_combat_debug(&mut e, "never"));
        assert!(calls(&e, GLOBAL_VALUE_SET).is_empty());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        assert!(run_combat_debug(&mut e, "never"));
        let buffer = calls(&e, CONSOLE_PRINT)[0][1];
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_UNKNOWN_OPTION, buffer]]
        );
        assert_eq!(e.mem.cstr(buffer), b"never");
    }

    // ---- ForceCombatGroupStrategy (005d67f0) -------------------------------

    /// An engine for `005d67f0`: the text functions, the name functions and
    /// the combat group accessors.
    fn strategy_engine(text: &'static [u8]) -> Engine {
        let mut e = combat_debug_engine();
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            e.mem.set_cstr(a[7], text);
            true.into_ret()
        });
        name_functions(&mut e, 1, 0xabcd);
        e.register(COMBAT_GROUP_OF_CONTROLLER, |_, a| (a[0] + 0x80).into_ret());
        e.register(COMBAT_GROUP_GET_COUNT, |_, _| 3u32.into_ret());
        e.register(COMBAT_GROUP_SET_STRATEGY, |_, _| Ret::default());
        e.register(COMBAT_GROUP_STRATEGY_BY_NAME, |_, _| 0x5757u32.into_ret());
        e.register(COMBAT_GROUP_STRATEGY_NAME, |_, _| 0x7575u32.into_ret());
        e
    }

    #[test]
    fn force_combat_group_strategy_reports_a_non_actor() {
        let mut e = strategy_engine(b"flank");
        let plain = non_actor_with(&mut e, &[(NAME_SLOT, V_NAME_SLOT)]);
        let a = command(&mut e, plain);
        start_log(&mut e);
        assert!(e.call(0x005d_67f0, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_NOT_AN_ACTOR, 0xabcd]]
        );
        // Without a name from slot 0x130 the reference name is used.
        name_functions(&mut e, 0, 0xabcd);
        start_log(&mut e);
        assert!(e.call(0x005d_67f0, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_NOT_AN_ACTOR, 0xaaaa]]
        );
        assert!(calls(&e, COMBAT_GROUP_SET_STRATEGY).is_empty());
        // The parse failure is false.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_67f0, &args![a]).bool());
    }

    #[test]
    fn force_combat_group_strategy_reports_an_actor_not_in_combat() {
        let mut e = strategy_engine(b"flank");
        e.register(V_CONTROLLER, |_, _| Ret::default());
        let actor = actor_with(
            &mut e,
            &[
                (NAME_SLOT, V_NAME_SLOT),
                (ACTOR_GET_COMBAT_CONTROLLER_SLOT, V_CONTROLLER),
            ],
        );
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005d_67f0, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_NOT_IN_COMBAT, 0xabcd]]
        );
    }

    #[test]
    fn force_combat_group_strategy_sets_clears_and_reports() {
        let controller = 0x7000_0000u32;
        let mut e = strategy_engine(b"flank");
        e.register_double(V_CONTROLLER, move |_, _| controller.into_ret());
        let actor = actor_with(&mut e, &[(ACTOR_GET_COMBAT_CONTROLLER_SLOT, V_CONTROLLER)]);
        let a = command(&mut e, actor);
        let group = controller + 0x80;
        // A group of one member.
        e.register(COMBAT_GROUP_GET_COUNT, |_, _| 1u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_67f0, &args![a]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_ONE_MEMBER]]);
        assert!(calls(&e, COMBAT_GROUP_SET_STRATEGY).is_empty());
        e.register(COMBAT_GROUP_GET_COUNT, |_, _| 2u32.into_ret());
        // A known name sets the strategy; the echo flag names it.
        start_log(&mut e);
        assert!(e.call(0x005d_67f0, &args![a]).bool());
        let buffer = calls(&e, COMBAT_GROUP_STRATEGY_BY_NAME)[0][0];
        assert_eq!(
            calls(&e, COMBAT_GROUP_SET_STRATEGY),
            vec![vec![group, 0x5757]]
        );
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_67f0, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_STRATEGY_SET, 0x7575]]
        );
        // An unknown name.
        e.register(COMBAT_GROUP_STRATEGY_BY_NAME, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_67f0, &args![a]).bool());
        assert!(calls(&e, COMBAT_GROUP_SET_STRATEGY).is_empty());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_UNKNOWN_STRATEGY, buffer]]
        );
        // An empty text clears the strategy.
        e.register_double(PARSE_PARAMETERS, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_67f0, &args![a]).bool());
        assert_eq!(calls(&e, COMBAT_GROUP_SET_STRATEGY), vec![vec![group, 0]]);
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_STRATEGY_CLEARED]]);
    }

    // ---- 005d6a60 and PushActorAway (005d6b60) -----------------------------

    #[test]
    fn fn_005d6a60_sets_the_processing_byte_of_an_actor() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[]);
        let a = command(&mut e, actor);
        parse_gives(&mut e, true, &[5]);
        start_log(&mut e);
        assert!(e.call(0x005d_6a60, &args![a]).bool());
        assert_parsed(&e, actor);
        assert_eq!(e.mem.u8(actor + 0xbc), 1);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // Zero clears the byte; the echo flag prints the name and state.
        parse_gives(&mut e, true, &[0]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005d_6a60, &args![a]).bool());
        assert_eq!(e.mem.u8(actor + 0xbc), 0);
        assert_eq!(calls(&e, GET_REFERENCE_NAME), vec![vec![actor]]);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_PROCESSING_IS, 0xaaaa, TEXT_OFF]]
        );
        parse_gives(&mut e, true, &[1]);
        start_log(&mut e);
        assert!(e.call(0x005d_6a60, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_PROCESSING_IS, 0xaaaa, TEXT_ON]]
        );
        // The cast fails: nothing is stored.
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        e.mem.set_u8(actor + 0xbc, 9);
        assert!(e.call(0x005d_6a60, &args![a]).bool());
        assert_eq!(e.mem.u8(actor + 0xbc), 9);
        // Not an actor: not even parsed. An actor with unparsable parameters
        // fails.
        let plain = non_actor_with(&mut e, &[]);
        let a = command(&mut e, plain);
        start_log(&mut e);
        assert!(e.call(0x005d_6a60, &args![a]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        let a = command(&mut e, actor);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_6a60, &args![a]).bool());
    }

    /// An actor `reference` for `005d6b60` with its process `process`
    /// (`GET_PROCESS` answers it) and the callees stubbed.
    fn push_actor_scene() -> (Engine, u32, u32, u32) {
        let mut e = engine();
        e.register(V_PROCESS_PUSH, |_, _| Ret::default());
        let process = object_with(&mut e, &[(PROCESS_SLOT_418, V_PROCESS_PUSH)]);
        let vector = e.mem.alloc(16);
        e.mem.set_u32(vector, bits(1.0));
        e.mem.set_u32(vector + 4, bits(2.0));
        e.mem.set_u32(vector + 8, bits(3.0));
        e.register_double(V_VECTOR, move |_, _| vector.into_ret());
        let actor = actor_with(&mut e, &[(REFR_GET_VECTOR_SLOT, V_VECTOR)]);
        e.register_double(GET_PROCESS, move |_, a| {
            if a[0] == actor {
                process.into_ret()
            } else {
                Ret::default()
            }
        });
        e.register(GET_KNOCKBACK_SPEED, |_, _| 6.5f32.into_ret());
        e.register(PROCESS_FIELD_28, |_, _| Ret::default());
        parse_gives(&mut e, true, &[actor, 3]);
        (e, actor, process, vector)
    }

    #[test]
    fn push_actor_away_pushes_the_actor_with_the_knockback_vector() {
        let (mut e, actor, process, _) = push_actor_scene();
        let this_obj = non_actor_with(&mut e, &[(REFR_GET_VECTOR_SLOT, V_VECTOR)]);
        let a = command(&mut e, this_obj);
        start_log(&mut e);
        assert!(e.call(0x005d_6b60, &args![a]).bool());
        assert_parsed(&e, this_obj);
        assert_eq!(calls(&e, GET_KNOCKBACK_SPEED), vec![vec![actor + 0xa4, 3]]);
        assert_eq!(calls(&e, V_VECTOR), vec![vec![this_obj]]);
        assert_eq!(
            calls(&e, V_PROCESS_PUSH),
            vec![vec![
                process,
                actor,
                bits(1.0),
                bits(2.0),
                bits(3.0),
                bits(6.5)
            ]]
        );
        // Not the player: nothing else.
        assert!(calls(&e, FUNCTION_005F5950).is_empty());
        // A busy process, or none: no push.
        e.register(PROCESS_FIELD_28, |_, _| 1u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_6b60, &args![a]).bool());
        assert!(calls(&e, V_PROCESS_PUSH).is_empty());
        e.register(PROCESS_FIELD_28, |_, _| Ret::default());
        e.register(GET_PROCESS, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_6b60, &args![a]).bool());
        assert!(calls(&e, V_PROCESS_PUSH).is_empty());
        assert!(calls(&e, PROCESS_FIELD_28).is_empty());
        assert_fails_when_unparsed(0x005d_6b60, GET_KNOCKBACK_SPEED);
    }

    #[test]
    fn push_actor_away_by_the_player_evaluates_the_form_first() {
        let (mut e, actor, process, _) = push_actor_scene();
        let player = actor_with(&mut e, &[(REFR_GET_VECTOR_SLOT, V_VECTOR)]);
        e.set_global(PLAYER, player);
        let list = e.mem.alloc(16);
        e.register_double(V_PROCESS_LIST, move |_, _| list.into_ret());
        let player_process = object_with(&mut e, &[(PROCESS_GET_LIST_SLOT, V_PROCESS_LIST)]);
        e.register_double(GET_PROCESS, move |_, a| {
            if a[0] == actor {
                process.into_ret()
            } else {
                player_process.into_ret()
            }
        });
        e.register(FIELD_AT_8, |_, _| 4u32.into_ret());
        e.register(GET_BASE_FORM, |_, _| 0xf00du32.into_ret());
        e.register(FUNCTION_005F5950, |_, _| Ret::default());
        let a = command(&mut e, player);
        start_log(&mut e);
        assert!(e.call(0x005d_6b60, &args![a]).bool());
        assert_eq!(calls(&e, FIELD_AT_8), vec![vec![list]]);
        assert_eq!(calls(&e, GET_BASE_FORM), vec![vec![actor]]);
        assert_eq!(
            calls(&e, FUNCTION_005F5950),
            vec![vec![3, 1, 0xf00d, 4, 0xffff, 2, 0]]
        );
        assert_eq!(calls(&e, V_PROCESS_PUSH).len(), 1);
        // An empty list is not counted.
        e.register(V_PROCESS_LIST, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_6b60, &args![a]).bool());
        assert!(calls(&e, FIELD_AT_8).is_empty());
        assert_eq!(calls(&e, FUNCTION_005F5950)[0][3], 0);
    }

    #[test]
    fn push_actor_away_logs_a_non_actor_target() {
        let mut e = engine();
        let target = non_actor_with(&mut e, &[]);
        let script_obj = object_with(&mut e, &[(NAME_SLOT, V_NAME_SLOT)]);
        e.register(V_NAME_SLOT, |_, _| 0x5c5cu32.into_ret());
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        let mut a = command(&mut e, 0);
        a.script_obj = Ptr::new(script_obj);
        parse_gives(&mut e, true, &[target, 1]);
        start_log(&mut e);
        assert!(e.call(0x005d_6b60, &args![a]).bool());
        assert_eq!(
            calls(&e, LOG_MESSAGE),
            vec![vec![MSG_PUSH_NON_ACTOR, 0x5c5c]]
        );
        // Without a script object nothing is logged.
        a.script_obj = Ptr::NULL;
        start_log(&mut e);
        assert!(e.call(0x005d_6b60, &args![a]).bool());
        assert!(calls(&e, LOG_MESSAGE).is_empty());
    }

    #[test]
    fn fn_005d6d10_removes_the_ownership() {
        let mut e = engine();
        e.register(EXTRA_DATA_LIST_REMOVE_OWNERSHIP, |_, _| Ret::default());
        e.register(V_MARK_CHANGED, |_, _| Ret::default());
        let object = object_with(&mut e, &[(MARK_CHANGED_SLOT, V_MARK_CHANGED)]);
        let a = command(&mut e, object);
        start_log(&mut e);
        assert!(e.call(0x005d_6d10, &args![a]).bool());
        assert_eq!(
            calls(&e, EXTRA_DATA_LIST_REMOVE_OWNERSHIP),
            vec![vec![object + 0x44]]
        );
        assert_eq!(calls(&e, V_MARK_CHANGED), vec![vec![object, 0x40]]);
        // Without a thisObj nothing happens.
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_6d10, &args![a]).bool());
        assert!(calls(&e, V_MARK_CHANGED).is_empty());
    }

    // ---- ShowInventory (005d6d40) ------------------------------------------

    #[test]
    fn show_inventory_prints_nothing_for_an_empty_inventory() {
        let mut e = engine();
        let actor = actor_with(&mut e, &[]);
        e.register(GET_INVENTORY_COUNT, |_, _| 0u32.into_ret());
        let a = command(&mut e, actor);
        start_log(&mut e);
        assert!(e.call(0x005d_6d40, &args![a]).bool());
        assert_eq!(calls(&e, GET_INVENTORY_COUNT), vec![vec![actor, 0, 1]]);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // No container: -1 is not above zero either.
        e.register(GET_INVENTORY_COUNT, |_, _| u32::MAX.into_ret());
        assert!(e.call(0x005d_6d40, &args![a]).bool());
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        // Without a thisObj nothing is even asked.
        let a = command(&mut e, 0);
        start_log(&mut e);
        assert!(e.call(0x005d_6d40, &args![a]).bool());
        assert!(calls(&e, GET_INVENTORY_COUNT).is_empty());
    }

    #[test]
    fn show_inventory_lists_the_entries_by_form_type() {
        let mut e = engine();
        name_functions(&mut e, 1, 0xabcd);
        let actor = actor_with(&mut e, &[(NAME_SLOT, V_NAME_SLOT)]);
        let a = command(&mut e, actor);
        // Three entries: a type 0x28 form, a type 0x18 form and another one.
        let forms: Vec<u32> = (0..3)
            .map(|_| object_with(&mut e, &[(NAME_SLOT, V_NAME_SLOT)]))
            .collect();
        let items: Vec<u32> = (0..3).map(|_| e.mem.alloc(16)).collect();
        for (item, form) in items.iter().zip(&forms) {
            e.mem.set_u32(item + 8, *form);
        }
        e.mem.set_u32(items[0] + 4, 2);
        e.mem.set_u32(items[1] + 4, 1);
        e.mem.set_u32(items[2] + 4, 30);
        e.register(GET_INVENTORY_COUNT, |_, _| 4u32.into_ret());
        let entries = items.clone();
        e.register_double(GET_INVENTORY_ENTRY, move |_, a| {
            // Entry 3 does not exist.
            entries.get(a[1] as usize).copied().unwrap_or(0).into_ret()
        });
        e.register(FIELD_AT_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(GET_FORM_ID, |_, a| (a[0] + 0x1000).into_ret());
        let form_types = forms.clone();
        e.register_double(FORM_TYPE_BYTE, move |_, a| {
            let which = form_types.iter().position(|f| *f == a[0]).unwrap();
            [0x28u32, 0x18, 0x31][which].into_ret()
        });
        e.register(ACTOR_CAN_EQUIP, |_, _| false.into_ret());
        e.register_double(ITEM_CHANGE_GET_WORN, |_, a| (a[0] % 16 == 0).into_ret());
        e.register(ITEM_CHANGE_GET_ITEM_HEALTH, |_, a| {
            (if a[1] == 0 { 120.0f32 } else { 75.5f32 }).into_ret()
        });
        e.register(GET_FORM_AS_BIPED_MODEL_LIST, |_, _| Ret::default());
        e.register(INVENTORY_ENTRY_RELEASE, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_6d40, &args![a]).bool());
        let worn = |item: u32| {
            if item % 16 == 0 {
                TEXT_WORN
            } else {
                EMPTY_STRING
            }
        };
        let [h0_low, h0_high] = f64_words(120.0);
        let [h1_low, h1_high] = f64_words(75.5);
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![
                vec![MSG_HAS_ITEMS, 0xabcd, actor + 0x1000, 4],
                vec![
                    MSG_ITEM_WITH_EQUIP,
                    2,
                    0xabcd,
                    forms[0] + 0x1000,
                    h0_low,
                    h0_high,
                    h1_low,
                    h1_high,
                    worn(items[0]),
                    TEXT_CANNOT_EQUIP
                ],
                vec![
                    MSG_ITEM_WITH_HEALTH,
                    1,
                    0xabcd,
                    forms[1] + 0x1000,
                    h0_low,
                    h0_high,
                    h1_low,
                    h1_high,
                    worn(items[1])
                ],
                vec![
                    MSG_ITEM_PLAIN,
                    30,
                    0xabcd,
                    forms[2] + 0x1000,
                    worn(items[2])
                ],
            ]
        );
        // The can-equip test is made for the actor with the form and 1; only
        // the entries that exist are released.
        assert_eq!(calls(&e, ACTOR_CAN_EQUIP), vec![vec![actor, forms[0], 1]]);
        assert_eq!(
            calls(&e, INVENTORY_ENTRY_RELEASE),
            items.iter().map(|i| vec![*i, 1]).collect::<Vec<_>>()
        );
        assert_eq!(
            calls(&e, GET_INVENTORY_ENTRY).len(),
            4,
            "one query per index"
        );
        // An actor that can equip the item gets no extra text; a plain
        // reference is never asked.
        e.register(ACTOR_CAN_EQUIP, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_6d40, &args![a]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT)[1][9], EMPTY_STRING);
        let plain = non_actor_with(&mut e, &[(NAME_SLOT, V_NAME_SLOT)]);
        let a = command(&mut e, plain);
        start_log(&mut e);
        assert!(e.call(0x005d_6d40, &args![a]).bool());
        assert!(calls(&e, ACTOR_CAN_EQUIP).is_empty());
        assert_eq!(calls(&e, CONSOLE_PRINT)[1][9], EMPTY_STRING);
    }

    #[test]
    fn show_inventory_names_forms_without_a_name_slot_text_and_lists_biped_models() {
        let mut e = engine();
        // `00474cb0` finds no name: the reference name and the full name are used.
        name_functions(&mut e, 0, 0xabcd);
        let reference = object_with(&mut e, &[(REFR_IS_ACTOR_SLOT, V_NOT_ACTOR)]);
        e.register(V_NOT_ACTOR, |_, _| false.into_ret());
        let a = command(&mut e, reference);
        let form = e.mem.alloc(16);
        let item = e.mem.alloc(16);
        e.mem.set_u32(item + 8, form);
        e.mem.set_u32(item + 4, 1);
        e.register(GET_INVENTORY_COUNT, |_, _| 1u32.into_ret());
        e.register_double(GET_INVENTORY_ENTRY, move |_, _| item.into_ret());
        e.register(FIELD_AT_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(GET_FORM_ID, |_, a| (a[0] + 0x1000).into_ret());
        e.register(FORM_TYPE_BYTE, |_, _| 0x31u32.into_ret());
        e.register(ITEM_CHANGE_GET_WORN, |_, _| false.into_ret());
        e.register(INVENTORY_ENTRY_RELEASE, |_, _| Ret::default());
        // The form has a biped model list whose owner (the word at +4) holds an
        // embedded list of two members starting at owner + 0x18.
        let list = e.mem.alloc(16);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(list + 4, owner);
        let (first, second) = (e.mem.alloc(16), e.mem.alloc(16));
        let node_b = e.mem.alloc(16);
        e.mem.set_u32(owner + 0x18, first);
        e.mem.set_u32(owner + 0x1c, node_b);
        e.mem.set_u32(node_b, second);
        e.register(FIELD_ADDRESS_18, |_, a| (a[0] + 0x18).into_ret());
        e.register_double(GET_FORM_AS_BIPED_MODEL_LIST, move |_, _| list.into_ret());
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        start_log(&mut e);
        assert!(e.call(0x005d_6d40, &args![a]).bool());
        let prints = calls(&e, CONSOLE_PRINT);
        assert_eq!(
            prints[0],
            vec![MSG_HAS_ITEMS, 0xaaaa, reference + 0x1000, 1]
        );
        assert_eq!(
            prints[1],
            vec![MSG_ITEM_PLAIN, 1, 0xf0f0, form + 0x1000, EMPTY_STRING]
        );
        assert_eq!(
            prints[2],
            vec![MSG_BIPED_LIST_HEADER, 0xf0f0, owner + 0x1000]
        );
        assert_eq!(
            prints[3],
            vec![MSG_BIPED_LIST_ENTRY, 0xf0f0, first + 0x1000]
        );
        assert_eq!(
            prints[4],
            vec![MSG_BIPED_LIST_ENTRY, 0xf0f0, second + 0x1000]
        );
        assert_eq!(prints.len(), 5);
    }

    // ---- The menu commands (005d7150 to 005d73d0) ---------------------------

    #[test]
    fn fn_005d7150_opens_the_menu_for_the_first_or_the_third_value() {
        let mut e = engine();
        e.register(OPEN_CHARGEN_MENU, |_, _| Ret::default());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register_double(PARSE_PARAMETERS, |e, a| {
            for slot in &a[7..10] {
                assert_eq!(e.mem.u32(*slot), 0);
            }
            e.mem.set_u32(a[7], 4);
            e.mem.set_u32(a[9], 9);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005d_7150, &args![a]).bool());
        assert_eq!(calls(&e, OPEN_CHARGEN_MENU), vec![vec![0, 4, 0, 1]]);
        // The first value is not positive: the third is used.
        parse_gives(&mut e, true, &[0, 7, 9]);
        start_log(&mut e);
        assert!(e.call(0x005d_7150, &args![a]).bool());
        assert_eq!(calls(&e, OPEN_CHARGEN_MENU), vec![vec![1, 9, 0, 1]]);
        // A negative first value counts as not positive.
        parse_gives(&mut e, true, &[u32::MAX, 7, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_7150, &args![a]).bool());
        assert!(calls(&e, OPEN_CHARGEN_MENU).is_empty());
        assert_fails_when_unparsed(0x005d_7150, OPEN_CHARGEN_MENU);
    }

    #[test]
    fn fn_005d71f0_opens_the_menu_with_the_second_flag() {
        let mut e = engine();
        e.register(OPEN_CHARGEN_MENU, |_, _| Ret::default());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[4, 0, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_71f0, &args![a]).bool());
        assert_eq!(calls(&e, OPEN_CHARGEN_MENU), vec![vec![0, 4, 1, 1]]);
        parse_gives(&mut e, true, &[0, 0, 6]);
        start_log(&mut e);
        assert!(e.call(0x005d_71f0, &args![a]).bool());
        assert_eq!(calls(&e, OPEN_CHARGEN_MENU), vec![vec![1, 6, 1, 1]]);
        assert_fails_when_unparsed(0x005d_71f0, OPEN_CHARGEN_MENU);
    }

    #[test]
    fn fn_005d7290_and_fn_005d72f0_open_the_menu_with_mode_zero() {
        for (address, flag) in [(0x005d_7290u32, 0u32), (0x005d_72f0, 1)] {
            let mut e = engine();
            e.register(OPEN_CHARGEN_MENU, |_, _| Ret::default());
            let this_obj = object(&mut e);
            let a = command(&mut e, this_obj);
            parse_gives(&mut e, true, &[12]);
            start_log(&mut e);
            assert!(e.call(address, &args![a]).bool());
            assert_eq!(calls(&e, OPEN_CHARGEN_MENU), vec![vec![0, 12, flag, 1]]);
            assert_fails_when_unparsed(address, OPEN_CHARGEN_MENU);
        }
    }

    #[test]
    fn fn_005d7350_opens_the_menu_with_mode_one() {
        let mut e = engine();
        e.register(OPEN_CHARGEN_MENU, |_, _| Ret::default());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register_double(PARSE_PARAMETERS, |e, a| {
            // The second value starts at 1.
            assert_eq!(e.mem.u32(a[8]), 1);
            e.mem.set_u32(a[7], 8);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005d_7350, &args![a]).bool());
        assert_eq!(calls(&e, OPEN_CHARGEN_MENU), vec![vec![1, 8, 0, 1]]);
        parse_gives(&mut e, true, &[8, 2]);
        start_log(&mut e);
        assert!(e.call(0x005d_7350, &args![a]).bool());
        assert_eq!(calls(&e, OPEN_CHARGEN_MENU), vec![vec![1, 8, 0, 0]]);
        assert_fails_when_unparsed(0x005d_7350, OPEN_CHARGEN_MENU);
    }

    #[test]
    fn fn_005d73d0_opens_the_menu_with_mode_one_and_the_flag() {
        let mut e = engine();
        e.register(OPEN_CHARGEN_MENU, |_, _| Ret::default());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[3]);
        start_log(&mut e);
        assert!(e.call(0x005d_73d0, &args![a]).bool());
        assert_eq!(calls(&e, OPEN_CHARGEN_MENU), vec![vec![1, 3, 1, 1]]);
        assert_fails_when_unparsed(0x005d_73d0, OPEN_CHARGEN_MENU);
    }

    #[test]
    fn fn_005d7430_and_fn_005d7480_call_the_interface_function() {
        for (address, flag) in [(0x005d_7430u32, 0u32), (0x005d_7480, 1)] {
            let mut e = engine();
            e.register(INTERFACE_FUNCTION_00705830, |_, _| Ret::default());
            let this_obj = object(&mut e);
            let a = command(&mut e, this_obj);
            // The default is 3 and a failed parse is not checked.
            e.register_double(PARSE_PARAMETERS, |e, a| {
                assert_eq!(e.mem.u32(a[7]), 3);
                false.into_ret()
            });
            start_log(&mut e);
            assert!(e.call(address, &args![a]).bool());
            assert_eq!(
                calls(&e, INTERFACE_FUNCTION_00705830),
                vec![vec![0, 2, flag]]
            );
            parse_gives(&mut e, true, &[10]);
            start_log(&mut e);
            assert!(e.call(address, &args![a]).bool());
            assert_eq!(
                calls(&e, INTERFACE_FUNCTION_00705830),
                vec![vec![0, 9, flag]]
            );
        }
    }

    #[test]
    fn fn_005d74d0_calls_error_without_arguments() {
        let mut e = engine();
        e.register(ERROR_REPORT, |_, _| Ret::default());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        start_log(&mut e);
        assert!(e.call(0x005d_74d0, &args![a]).bool());
        assert_eq!(calls(&e, ERROR_REPORT), vec![Vec::<u32>::new()]);
    }

    // ---- The condition commands with one or two values -----------------------

    /// Checks a command that parses `parsed` values and returns the condition
    /// function's `AL` with `thisObj`, the values, (0 for a single value) and
    /// the result double.
    fn check_parse_then_condition(command_address: u32, condition: u32, parsed: &[u32]) {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register(condition, |_, _| true.into_ret());
        parse_gives(&mut e, true, parsed);
        start_log(&mut e);
        assert!(e.call(command_address, &args![a]).bool());
        let expected = if parsed.len() == 1 {
            vec![this_obj, parsed[0], 0, a.result.addr()]
        } else {
            vec![this_obj, parsed[0], parsed[1], a.result.addr()]
        };
        assert_eq!(calls(&e, condition), vec![expected]);
        // The condition function's answer is the command's answer.
        e.register(condition, |_, _| false.into_ret());
        assert!(!e.call(command_address, &args![a]).bool());
        assert_fails_when_unparsed(command_address, condition);
    }

    #[test]
    fn get_killer_function_returns_the_condition_answer() {
        check_parse_then_condition(0x005d_74e0, GET_KILLER_CONDITION, &[0x44]);
    }

    #[test]
    fn get_killer_object_function_returns_the_condition_answer() {
        check_parse_then_condition(0x005d_7540, GET_KILLER_OBJECT_CONDITION, &[0x45]);
    }

    #[test]
    fn fn_005d75a0_passes_two_values_to_the_condition() {
        check_parse_then_condition(0x005d_75a0, CONDITION_005A4090, &[0x46, 0x47]);
    }

    #[test]
    fn fn_005d7b80_returns_the_condition_answer() {
        check_parse_then_condition(0x005d_7b80, CONDITION_005A4210, &[0x48]);
    }

    /// A command without parameters that returns the condition function's
    /// `AL`.
    fn check_plain_condition(command_address: u32, condition: u32) {
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
    fn fn_005d7be0_and_fn_005d7c00_return_the_condition_answer() {
        check_plain_condition(0x005d_7be0, CONDITION_005A4240);
        check_plain_condition(0x005d_7c00, CONDITION_005A42B0);
    }

    // ---- The decal commands ---------------------------------------------------

    #[test]
    fn toggle_decal_debug_switches_the_bit_of_each_option() {
        // (option, bit, "enabled" message, "disabled" message)
        let options: [(u32, u32, u32, u32); 6] = [
            (0, 0x01, 0x0103_cdc4, 0x0103_cdf0),
            (1, 0x02, 0x0103_cd80, 0x0103_cda0),
            (2, 0x04, 0x0103_cd28, 0x0103_cd54),
            (3, 0x08, 0x0103_ccf0, 0x0103_cd0c),
            (4, 0x10, 0x0103_ccbc, 0x0103_ccd4),
            (5, 0x20, 0x0103_cc84, 0x0103_cca0),
        ];
        for (option, bit, enabled, disabled) in options {
            let mut e = engine();
            let this_obj = object(&mut e);
            let a = command(&mut e, this_obj);
            e.set_global(DECAL_DEBUG_FLAGS, 0x100u32);
            parse_gives(&mut e, true, &[option]);
            start_log(&mut e);
            assert!(e.call(0x005d_7610, &args![a]).bool());
            assert_eq!(e.global::<u32>(DECAL_DEBUG_FLAGS), 0x100 | bit);
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![enabled]]);
            let wireframe = (bit == 0x20) as u8;
            assert_eq!(e.global::<u8>(DECAL_WIREFRAME_BYTE), wireframe);
            // The second call switches it off again.
            start_log(&mut e);
            assert!(e.call(0x005d_7610, &args![a]).bool());
            assert_eq!(e.global::<u32>(DECAL_DEBUG_FLAGS), 0x100);
            assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![disabled]]);
            assert_eq!(e.global::<u8>(DECAL_WIREFRAME_BYTE), 0);
        }
    }

    #[test]
    fn toggle_decal_debug_ignores_other_options() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.register_double(PARSE_PARAMETERS, |e, a| {
            // The option starts at -1.
            assert_eq!(e.mem.u32(a[7]), u32::MAX);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005d_7610, &args![a]).bool());
        for option in [6u32, 100] {
            parse_gives(&mut e, true, &[option]);
            assert!(e.call(0x005d_7610, &args![a]).bool());
        }
        assert_eq!(e.global::<u32>(DECAL_DEBUG_FLAGS), 0);
        assert!(calls(&e, CONSOLE_PRINT).is_empty());
        assert_fails_when_unparsed(0x005d_7610, CONSOLE_PRINT);
    }

    #[test]
    fn toggle_decal_rendering_flips_the_flag() {
        let mut e = engine();
        let owner = e.mem.alloc(0x200);
        let object = e.mem.alloc(0x200);
        e.set_global(DECAL_OWNER, owner);
        e.register_double(DECAL_OWNER_OBJECT, move |_, _| object.into_ret());
        e.register(DECAL_RENDERING_GET, |e, a| {
            e.mem.u8(a[0] + 0x16c).into_ret()
        });
        e.register(DECAL_RENDERING_SET, |e, a| {
            e.mem.set_u8(a[0] + 0x16c, a[1] as u8);
            Ret::default()
        });
        let this_obj = object_with(&mut e, &[]);
        let a = command(&mut e, this_obj);
        start_log(&mut e);
        assert!(e.call(0x005d_7850, &args![a]).bool());
        assert_eq!(calls(&e, CONSOLE_PRINT), vec![vec![MSG_DECAL_RENDERING_ON]]);
        assert_eq!(e.mem.u8(object + 0x16c), 1);
        assert_eq!(calls(&e, DECAL_RENDERING_SET), vec![vec![object, 1]]);
        assert_eq!(
            calls(&e, DECAL_OWNER_OBJECT),
            vec![vec![owner], vec![owner]]
        );
        start_log(&mut e);
        assert!(e.call(0x005d_7850, &args![a]).bool());
        assert_eq!(
            calls(&e, CONSOLE_PRINT),
            vec![vec![MSG_DECAL_RENDERING_OFF]]
        );
        assert_eq!(e.mem.u8(object + 0x16c), 0);
    }

    #[test]
    fn fn_005d78c0_stores_zero_and_one() {
        let mut e = engine();
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        e.set_global(SWITCH_FLAG_0118ABB0, 7u8);
        e.register_double(PARSE_PARAMETERS, |e, a| {
            assert_eq!(e.mem.u32(a[7]), u32::MAX);
            true.into_ret()
        });
        // The default (-1) changes nothing.
        assert!(e.call(0x005d_78c0, &args![a]).bool());
        assert_eq!(e.global::<u8>(SWITCH_FLAG_0118ABB0), 7);
        parse_gives(&mut e, true, &[1]);
        assert!(e.call(0x005d_78c0, &args![a]).bool());
        assert_eq!(e.global::<u8>(SWITCH_FLAG_0118ABB0), 1);
        parse_gives(&mut e, true, &[0]);
        assert!(e.call(0x005d_78c0, &args![a]).bool());
        assert_eq!(e.global::<u8>(SWITCH_FLAG_0118ABB0), 0);
        e.set_global(SWITCH_FLAG_0118ABB0, 7u8);
        parse_gives(&mut e, true, &[2]);
        assert!(e.call(0x005d_78c0, &args![a]).bool());
        assert_eq!(e.global::<u8>(SWITCH_FLAG_0118ABB0), 7);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005d_78c0, &args![a]).bool());
    }

    #[test]
    fn fn_005d7930_stores_the_byte() {
        let mut e = engine();
        e.call(0x005d_7930, &args![1u32]);
        assert_eq!(e.global::<u8>(SWITCH_FLAG_0118ABB0), 1);
        e.call(0x005d_7930, &args![0u32]);
        assert_eq!(e.global::<u8>(SWITCH_FLAG_0118ABB0), 0);
    }

    #[test]
    fn get_is_moving_echoes_the_condition_result() {
        check_echo_command(0x005d_7940, IS_MOVING_CONDITION, MSG_IS_MOVING);
    }

    #[test]
    fn get_is_turning_echoes_the_condition_result() {
        check_echo_command(0x005d_7990, IS_TURNING_CONDITION, MSG_IS_TURNING);
    }

    #[test]
    fn get_anim_action_echoes_the_condition_result() {
        check_echo_command(0x005d_79e0, GET_ANIM_ACTION_CONDITION, MSG_GET_ANIM_ACTION);
    }

    // ---- 005d7a30, 005d7b50 and 005d7c20 ------------------------------------

    #[test]
    fn fn_005d7a30_calls_the_actor_method_with_the_nine_values() {
        let mut e = engine();
        e.register(V_SLOT_434, |_, _| Ret::default());
        let actor = actor_with(&mut e, &[(ACTOR_SLOT_434, V_SLOT_434)]);
        let a = command(&mut e, actor);
        e.register(ACTOR_UNIT_METHOD_008BBF40, |_, _| Ret::default());
        e.register_double(PARSE_PARAMETERS, |e, a| {
            let initial: Vec<u32> = (7..16).map(|i| e.mem.u32(a[i])).collect();
            assert_eq!(initial, vec![0, 0, 0, 0, 1, 1, 0, 0, 0]);
            for (i, value) in [10u32, 11, 12, 13, 0, 5, 0, 8, 19].iter().enumerate() {
                e.mem.set_u32(a[7 + i], *value);
            }
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005d_7a30, &args![a]).bool());
        assert_eq!(calls(&e, V_SLOT_434), vec![vec![actor, 0]]);
        assert_eq!(
            calls(&e, ACTOR_UNIT_METHOD_008BBF40),
            vec![vec![actor, 10, 11, 12, 13, 19, 0, 1, 0, 1]]
        );
        // Not an actor: nothing.
        let plain = non_actor_with(&mut e, &[]);
        let a = command(&mut e, plain);
        start_log(&mut e);
        assert!(e.call(0x005d_7a30, &args![a]).bool());
        assert!(calls(&e, ACTOR_UNIT_METHOD_008BBF40).is_empty());
        assert_fails_when_unparsed(0x005d_7a30, ACTOR_UNIT_METHOD_008BBF40);
    }

    #[test]
    fn fn_005d7b50_calls_the_function_on_what_the_slot_gives() {
        let mut e = engine();
        e.register(V_SLOT_19C, |_, _| 0x1919u32.into_ret());
        e.register(MAGIC_FUNCTION_008255A0, |_, _| Ret::default());
        let object = object_with(&mut e, &[(REFR_SLOT_19C, V_SLOT_19C)]);
        let a = command(&mut e, object);
        start_log(&mut e);
        assert!(e.call(0x005d_7b50, &args![a]).bool());
        assert_eq!(calls(&e, MAGIC_FUNCTION_008255A0), vec![vec![0x1919]]);
        e.register(V_SLOT_19C, |_, _| Ret::default());
        start_log(&mut e);
        assert!(e.call(0x005d_7b50, &args![a]).bool());
        assert!(calls(&e, MAGIC_FUNCTION_008255A0).is_empty());
    }

    /// A quest `quest` (an object with a name slot) and the objective `objective`
    /// for `005d7c20`, with the callees stubbed; the quest's flag bit 2 is
    /// `flag_set`, the objective's state predicates `above_one` and `odd`.
    fn quest_scene(flag_set: bool, above_one: bool, odd: bool) -> (Engine, u32, u32, ScriptArgs) {
        let mut e = engine();
        e.register(V_QUEST_NAME, |_, _| 0x9999u32.into_ret());
        let quest = object_with(&mut e, &[(QUEST_NAME_SLOT, V_QUEST_NAME)]);
        let objective = e.mem.alloc(0x40);
        e.register_double(QUEST_FIND_OBJECTIVE, move |_, a| {
            (if a[1] == 3 { objective } else { 0 }).into_ret()
        });
        e.register_double(QUEST_FLAG_2, move |_, _| flag_set.into_ret());
        e.register_double(OBJECTIVE_ABOVE_ONE, move |_, _| above_one.into_ret());
        e.register_double(OBJECTIVE_BIT_0, move |_, _| odd.into_ret());
        e.register(QUEST_TARGET_SET_STATE, |_, _| Ret::default());
        e.register(GET_FORM_ID, |_, a| (a[0] + 0x1000).into_ret());
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        let this_obj = object(&mut e);
        let a = command(&mut e, this_obj);
        parse_gives(&mut e, true, &[quest, 3, 1]);
        (e, quest, objective, a)
    }

    #[test]
    fn fn_005d7c20_sets_the_state_of_the_objective() {
        // A zero flag sets state 0.
        let (mut e, quest, objective, a) = quest_scene(false, false, false);
        parse_gives(&mut e, true, &[quest, 3, 0]);
        start_log(&mut e);
        assert!(e.call(0x005d_7c20, &args![a]).bool());
        assert_eq!(calls(&e, QUEST_FIND_OBJECTIVE), vec![vec![quest, 3]]);
        assert_eq!(calls(&e, QUEST_TARGET_SET_STATE), vec![vec![objective, 0]]);
        // A flag with an objective that is not above one: state 2, or 3 when
        // its bit 0 is set.
        start_log(&mut e);
        parse_gives(&mut e, true, &[quest, 3, 1]);
        assert!(e.call(0x005d_7c20, &args![a]).bool());
        assert_eq!(calls(&e, QUEST_TARGET_SET_STATE), vec![vec![objective, 2]]);
        let (mut e, quest, objective, a) = quest_scene(false, false, true);
        start_log(&mut e);
        assert!(e.call(0x005d_7c20, &args![a]).bool());
        assert_eq!(calls(&e, QUEST_TARGET_SET_STATE), vec![vec![objective, 3]]);
        let _ = quest;
        // An objective above one is left alone, as is a quest with the flag.
        let (mut e, _, _, a) = quest_scene(false, true, true);
        start_log(&mut e);
        assert!(e.call(0x005d_7c20, &args![a]).bool());
        assert!(calls(&e, QUEST_TARGET_SET_STATE).is_empty());
        let (mut e, _, _, a) = quest_scene(true, false, false);
        start_log(&mut e);
        assert!(e.call(0x005d_7c20, &args![a]).bool());
        assert!(calls(&e, QUEST_TARGET_SET_STATE).is_empty());
        assert!(calls(&e, LOG_MESSAGE).is_empty());
        assert_fails_when_unparsed(0x005d_7c20, QUEST_TARGET_SET_STATE);
    }

    #[test]
    fn fn_005d7c20_logs_a_missing_objective() {
        let (mut e, quest, _, a) = quest_scene(false, false, false);
        parse_gives(&mut e, true, &[quest, 8, 1]);
        start_log(&mut e);
        assert!(e.call(0x005d_7c20, &args![a]).bool());
        assert_eq!(
            calls(&e, LOG_MESSAGE),
            vec![vec![MSG_OBJECTIVE_MISSING, 8, 0x9999, quest + 0x1000]]
        );
        assert!(calls(&e, QUEST_TARGET_SET_STATE).is_empty());
        assert_eq!(calls(&e, GET_FORM_ID), vec![vec![quest]]);
    }

    #[test]
    fn fn_005d7c20_logs_a_missing_quest() {
        // The game reads the id and the name slot of the null quest without a
        // check: page 0 holds a vtable for the test.
        let (mut e, _, _, a) = quest_scene(false, false, false);
        e.map(0, 0x1000);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + QUEST_NAME_SLOT, V_QUEST_NAME);
        e.mem.set_u32(0, vtable);
        parse_gives(&mut e, true, &[0, 3, 1]);
        start_log(&mut e);
        assert!(e.call(0x005d_7c20, &args![a]).bool());
        assert_eq!(
            calls(&e, LOG_MESSAGE),
            vec![vec![MSG_QUEST_MISSING, 0x9999, 0x1000]]
        );
        assert!(calls(&e, QUEST_FIND_OBJECTIVE).is_empty());
    }
}

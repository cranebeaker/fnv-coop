//! `fallout shared/tesconditionfunctions.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is the script condition functions: one `bool` function per
//! condition command (`GetDistance`, `GetPos`, `SameFaction`, ...), all with
//! the same shape `bool f(TESObjectREFR* ref, void* param1, void* param2,
//! double* result)` (cdecl). The function writes the condition's value to
//! `*result` and returns `true`; the command table in `.data` points at them.
//! Most start by writing `0.0` to `*result`, and all but a few finish with a
//! debug line through `00703c00` when the byte at `TLS + 0x268` is set.
//!
//! Progress: this file holds the first 160 functions of the unit in address
//! order: `0059bfa0` to `0059dbe0` (the first session; it left `0059c380`
//! out because `crates/world` already describes it), `0059c380` plus
//! `0059dc90` to `0059f540` (the second session), `0059f610` to
//! `005a0f10` (the third) and `005a0f90` to `005a29d0` (the fourth). The
//! next session continues with the first function after `005a29d0`
//! (`005a2a50`, outside this file's address range: the range ends there);
//! keep new shared helpers and constants in the block below, above the
//! functions.
//!
//! x87 note: the game computes in extended precision and stores `double`
//! results. The translations compute in `f64`, which can differ from the
//! x87 only in the last bit of a result rounded twice.

#[allow(unused_imports)]
use crate::prelude::*;

/// Offset in the TLS block of the byte that turns on the condition
/// functions' debug lines (`*(*(FS:[0x2c] + _tls_index * 4) + 0x268)`).
pub(crate) const TLS_TRACE_FLAG: u32 = 0x268;
/// The debug print (`printf`-like, varargs, `interface.cpp`).
pub(crate) const DEBUG_PRINT: u32 = 0x0070_3c00;
/// `PlayerCharacter*` global.
pub(crate) const PLAYER: u32 = 0x011d_ea3c;
/// `TES*` global (`TES::Pick`'s `this`).
pub(crate) const TES: u32 = 0x011d_ea10;
/// The `Calendar` singleton.
pub(crate) const CALENDAR: u32 = 0x011d_e7b8;
/// The save/load global data object `GetSecondsPassed` falls back on.
pub(crate) const SECONDS_PASSED_SOURCE: u32 = 0x011f_6394;

/// `_ftol2_sse` (`00ec62c0`): truncates the value in ST0 (passed as a leading
/// `f64` argument) to an integer in EAX.
pub(crate) const FTOL: u32 = 0x00ec_62c0;
/// `operator delete` (`00401030`).
pub(crate) const OPERATOR_DELETE: u32 = 0x0040_1030;

/// Value of virtual `IsActor` (vtable `+0x100`, Xbox PDB name).
pub(crate) const VSLOT_IS_ACTOR: u32 = 0x100;
/// Virtual `IsBoundObject` (vtable `+0xe4`, Xbox PDB name).
pub(crate) const VSLOT_IS_BOUND_OBJECT: u32 = 0xe4;
/// Virtual `IsMobileObject` (vtable `+0xfc`, Xbox PDB name).
pub(crate) const VSLOT_IS_MOBILE_OBJECT: u32 = 0xfc;
/// Virtual on `TESObjectREFR` returning a pointer to its position (3 floats).
pub(crate) const VSLOT_GET_POSITION: u32 = 0x1f4;

/// Reads `[this + 0x20]` (`TESObjectREFR`'s base form; the engine map names
/// this body `BGSSaveFormBuffer::GetForm` because the linker folded the
/// identical code).
pub(crate) const REF_GET_BASE_FORM: u32 = 0x007a_f430;
/// Same body as [`REF_GET_BASE_FORM`], in `extradatalist.cpp`: the actor's
/// base form.
pub(crate) const ACTOR_GET_BASE_FORM: u32 = 0x0041_81e0;
/// `TESForm::cFormType` (byte at `+0x04`) of the form in ECX.
pub(crate) const FORM_GET_TYPE: u32 = 0x0040_1170;
/// True when the `TESForm` in ECX has flag `0x800` (`iFormFlags` at `+0x08`).
pub(crate) const FORM_HAS_FLAG_0800: u32 = 0x0044_0da0;
/// `TESObjectREFR::HasContainer` (Xbox PDB): non-zero when the reference's
/// base form holds items.
pub(crate) const REF_HAS_CONTAINER: u32 = 0x0055_d310;
/// Form type of an `NPC_` (`TESNPC`).
pub(crate) const FORM_TYPE_NPC: u32 = 0x2a;
/// Form type of a `FLST` (`BGSListForm`).
pub(crate) const FORM_TYPE_FORM_LIST: u32 = 0x55;
/// Form type of a `TERM` (`BGSTerminal`).
pub(crate) const FORM_TYPE_TERMINAL: u32 = 0x17;

/// `BGSListForm`'s list: returns `this + 0x18`.
pub(crate) const FORM_LIST_GET_LIST: u32 = 0x0050_0940;
/// `BSSimpleList` node helpers (`this` is the node): true for an empty
/// list's head node (item and next both null) ...
pub(crate) const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// ... the address of the node's item field (returns `this`) ...
pub(crate) const LIST_NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
/// ... and the next node (`[this + 4]`).
pub(crate) const LIST_NODE_NEXT: u32 = 0x0072_6070;

/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), cdecl, one reference
/// argument.
pub(crate) const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::GetObjectCount` (Xbox PDB name), `this` the inventory
/// changes, one item form argument.
pub(crate) const INVENTORY_GET_OBJECT_COUNT: u32 = 0x004c_8f30;
/// CRT function the item counts pass through (cdecl, one `int`).
pub(crate) const ITEM_COUNT_FILTER: u32 = 0x00ec_7d40;

/// True when the `Actor`'s middle-high process exists (`this` the actor;
/// the engine map names it `MiddleHighProcess::GetSavedAcquireObject`
/// because of identical code folding).
pub(crate) const ACTOR_GET_PROCESS: u32 = 0x008d_8520;
/// `ActorValue::GetActorValueScriptName` (Xbox PDB), cdecl.
pub(crate) const ACTOR_VALUE_SCRIPT_NAME: u32 = 0x0066_eac0;
/// `TESObjectREFR::GetScale` (Xbox PDB): the reference's scale with its base.
pub(crate) const REF_GET_SCALE: u32 = 0x0056_7400;
/// `[this + 0x3c]` as a `float` (the reference's own scale).
pub(crate) const REF_GET_OWN_SCALE: u32 = 0x0059_8040;
/// `TESObjectREFR::GetDistanceFromReference` (Xbox PDB), `this` and
/// (other, flag, flag).
pub(crate) const REF_GET_DISTANCE_FROM_REFERENCE: u32 = 0x0057_23b0;
/// `TESObjectREFR::GetLock` (Xbox PDB): the reference's `REFR_LOCK`, or null.
pub(crate) const REF_GET_LOCK: u32 = 0x0056_9160;
/// `Actor::LineOfSight` (Xbox PDB): `this` and (0, target, 1, 0, 0), byte
/// result.
pub(crate) const ACTOR_LINE_OF_SIGHT: u32 = 0x0088_b880;
/// The name of a reference (`tesobjectrefr.cpp`), used by `GetLOS`'s debug line.
pub(crate) const REF_GET_NAME: u32 = 0x0055_d520;

/// `57.295776` (`double`): degrees per radian, `GetAngle`'s multiplier.
pub(crate) const DEGREES_PER_RADIAN: u32 = 0x0102_f248;

/// `1.0` (`double`), the constant `GetSleeping` and `GetSitting` add.
pub(crate) const DOUBLE_ONE: u32 = 0x0101_2070;
/// The `float` constants of the rain / snow tests (`0059e950`, `0059ea80`):
/// the second argument of the first and of the second weather lookup.
pub(crate) const WEATHER_RAIN_FROM: u32 = 0x0102_31e0;
pub(crate) const WEATHER_RAIN_TO: u32 = 0x0101_7d00;

/// `Interface::IsInMenuMode` (Xbox PDB), cdecl, byte result.
pub(crate) const INTERFACE_IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// `Interface::IsTopMenuID` (Xbox PDB), cdecl, one menu id.
pub(crate) const INTERFACE_IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `Interface::IsMenuIDVisible` (Xbox PDB), cdecl, (menu id, 0).
pub(crate) const INTERFACE_IS_MENU_ID_VISIBLE: u32 = 0x0070_2680;
/// Byte global: when set, `GetMenuMode` asks for the top menu only.
pub(crate) const MENU_MODE_TOP_ONLY: u32 = 0x011c_ab24;

/// Virtual on `Actor` (vtable `+0x214`): the sit / sleep state, 0 for none.
pub(crate) const VSLOT_SIT_SLEEP_STATE: u32 = 0x214;
/// Virtuals of the actor's middle-high process: the furniture marker id
/// (`+0x4c4`) and the furniture reference the actor uses (`+0x4c8`).
pub(crate) const VSLOT_PROCESS_FURNITURE_MARKER_ID: u32 = 0x4c4;
pub(crate) const VSLOT_PROCESS_FURNITURE_REFERENCE: u32 = 0x4c8;
/// Virtual on `TESObjectREFR` behind `GetTalkedToPC` (bool).
pub(crate) const VSLOT_TALKED_TO_PC: u32 = 0x98;
/// Virtual on `Actor` behind `GetAttacked` (bool).
pub(crate) const VSLOT_ATTACKED: u32 = 0x470;
/// Virtual on `Actor` that `GetShouldAttack` calls with (target, 0) before
/// its checks; the result is not used.
pub(crate) const VSLOT_ACTOR_PREPARE_ATTACK_CHECK: u32 = 0x344;
/// Virtual on the target of `GetShouldAttack`; non-zero lets the combat
/// manager answer first.
pub(crate) const VSLOT_TARGET_COMBAT_CHECK: u32 = 0x428;
/// Virtual on `TESForm` returning a name string (`GetInCell` compares them).
pub(crate) const VSLOT_FORM_GET_NAME: u32 = 0x130;
/// Virtual `+0x68` of the component at `+0x30` of an `NPC_` / `CREA` base
/// form: its voice type.
pub(crate) const VSLOT_BASE_COMPONENT_VOICE_TYPE: u32 = 0x68;

/// Form types the functions test (`TESForm::cFormType`).
pub(crate) const FORM_TYPE_CLASS: u32 = 7;
pub(crate) const FORM_TYPE_FACTION: u32 = 8;
pub(crate) const FORM_TYPE_RACE: u32 = 0xc;
/// The form type whose voice type comes from `0x009185e0` (`+0x94`).
pub(crate) const FORM_TYPE_VOICE_SOURCE: u32 = 0x16;
pub(crate) const FORM_TYPE_FURNITURE: u32 = 0x27;
pub(crate) const FORM_TYPE_CREATURE: u32 = 0x2b;
pub(crate) const FORM_TYPE_WEATHER: u32 = 0x35;
pub(crate) const FORM_TYPE_CELL: u32 = 0x39;
pub(crate) const FORM_TYPE_WORLDSPACE: u32 = 0x41;
pub(crate) const FORM_TYPE_QUEST: u32 = 0x47;
/// The range of placed reference types (`0x3a` to `0x40`), and the one more
/// type (`0x69`) the reference tests accept.
pub(crate) const FORM_TYPE_REFERENCE_FIRST: u32 = 0x3a;
pub(crate) const FORM_TYPE_REFERENCE_LAST: u32 = 0x40;
pub(crate) const FORM_TYPE_REFERENCE_EXTRA: u32 = 0x69;
/// The two types `GetShouldAttack` accepts for its target.
pub(crate) const FORM_TYPE_ACTOR_FIRST: u32 = 0x3b;
pub(crate) const FORM_TYPE_ACTOR_LAST: u32 = 0x3c;

/// `[this + 0x40]` of a reference: its parent cell.
pub(crate) const REF_GET_PARENT_CELL: u32 = 0x008d_6f30;
/// `TESObjectREFR::GetWorldSpace` (Xbox PDB).
pub(crate) const REF_GET_WORLDSPACE: u32 = 0x0057_5d70;
/// `this + 0x44`: the reference's `ExtraDataList` (`tesscriptfunctions.cpp`).
pub(crate) const REF_GET_EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::GetReferencePointer` (Xbox PDB).
pub(crate) const EXTRA_DATA_LIST_GET_REFERENCE_POINTER: u32 = 0x0041_c8d0;
/// `[this + 0x0c]` of a reference: the id `GetInventoryItem` takes.
pub(crate) const REF_GET_ITEM_ID: u32 = 0x0084_e3a0;
/// `TESObjectREFR::GetInventoryItem` (Xbox PDB): `this` and (base form,
/// item id).
pub(crate) const REF_GET_INVENTORY_ITEM: u32 = 0x0057_6260;
/// `TESObjectREFR::GetScriptVariables` (Xbox PDB).
pub(crate) const REF_GET_SCRIPT_VARIABLES: u32 = 0x0056_73e0;
/// `ItemChange::GetScriptLocals` (Xbox PDB).
pub(crate) const ITEM_CHANGE_GET_SCRIPT_LOCALS: u32 = 0x004b_dea0;
/// Destructor with a delete flag: `this` the `ItemChange`, flag 1.
pub(crate) const ITEM_CHANGE_DESTROY: u32 = 0x0044_59e0;
/// `ScriptLocals::GetVariable` (Xbox PDB): `this` and (variable, 0), the
/// value in ST0.
pub(crate) const SCRIPT_LOCALS_GET_VARIABLE: u32 = 0x005a_9140;
/// True when the singly linked list starting at `this` holds the item whose
/// address is the argument (`0x005f65d0`).
pub(crate) const LIST_CONTAINS: u32 = 0x005f_65d0;

/// `Sky::GetInstance` (Xbox PDB).
pub(crate) const SKY_GET_INSTANCE: u32 = 0x0046_dd00;
/// `Sky::pCurrentWeather` (`+0x10`); the engine map names this body
/// `BaseProcess::GetCurrentProcedureIndex` because of folded code.
pub(crate) const SKY_GET_CURRENT_WEATHER: u32 = 0x0044_edb0;
/// `Sky::pLastWeather` (`+0x14`).
pub(crate) const SKY_GET_LAST_WEATHER: u32 = 0x0082_5c00;
/// `Sky::fCurrentWeatherPct` (`+0xf4`), returned as a `float` in ST0.
pub(crate) const SKY_GET_WEATHER_PERCENT: u32 = 0x0064_47d0;
/// `TESWeather`: true when flag `4` of the byte at `+0xeb` is set.
pub(crate) const WEATHER_HAS_FLAG_PRECIPITATION: u32 = 0x004e_d270;
/// `TESWeather`: interpolates a per-weather byte (`this`, index, from, to)
/// and returns the `float` in ST0.
pub(crate) const WEATHER_INTERPOLATE: u32 = 0x004e_d230;

/// `Actor::GetShouldAttackActor` (Xbox PDB): `this` the actor and (target, 0,
/// out struct, 0), byte result.
pub(crate) const ACTOR_GET_SHOULD_ATTACK_ACTOR: u32 = 0x008b_06d0;
/// Byte at `+0x104` of the target (`animation.cpp`).
pub(crate) const TARGET_GET_FLAG: u32 = 0x0049_3bb0;
/// The attack test of the combat manager: `this` the manager read from
/// [`COMBAT_MANAGER`] and (actor, target), byte result.
pub(crate) const COMBAT_MANAGER_CHECK: u32 = 0x0099_2640;
pub(crate) const COMBAT_MANAGER: u32 = 0x011f_1958;
/// `TESActorBaseData::GetFactionRank` (Xbox PDB) on `base + 0x30`:
/// (faction, is the player), -1 when not a member.
pub(crate) const ACTOR_BASE_DATA_GET_FACTION_RANK: u32 = 0x0047_d680;
/// `fallout/ai/actor.cpp`: the entry of a string table (`0x0119bcb0`) that
/// `GetAlarmed` compares with "Alarm", or null.
pub(crate) const ACTOR_GET_PROCEDURE_NAME: u32 = 0x0088_b7f0;
/// `_stricmp` (cdecl, two strings).
pub(crate) const STRING_COMPARE_NO_CASE: u32 = 0x0040_4dc0;
/// `_strnicmp` (cdecl, two strings and a count).
pub(crate) const STRING_COMPARE_N_NO_CASE: u32 = 0x00ec_7ec0;
/// Number of characters of a cell's name to compare (`this` the cell).
pub(crate) const CELL_GET_NAME_LENGTH: u32 = 0x0047_4cb0;
/// `TESQuest` accessors: running test, current stage, stage-done test (with
/// a byte argument).
pub(crate) const QUEST_IS_RUNNING: u32 = 0x0045_5620;
pub(crate) const QUEST_GET_CURRENT_STAGE: u32 = 0x0060_d700;
pub(crate) const QUEST_IS_STAGE_DONE: u32 = 0x0060_d600;
/// `this + 0x3c`: the address of a quest's flag byte.
pub(crate) const QUEST_GET_FLAGS_ADDRESS: u32 = 0x005a_8080;

/// True when bit 1 of a `RACE`'s flags (`this + 0x70`) is set; the next
/// function of this unit (`0059f610`), called by address.
pub(crate) const RACE_IS_PLAYABLE: u32 = 0x0059_f610;
/// `NPC_` accessors: the race (`bipedanim.cpp`), the class (the engine map
/// names this body `MiddleHighProcess::GetFireNode`, folded) and the sex
/// (`TESActorBase::GetSex`, Xbox PDB).
pub(crate) const NPC_GET_RACE: u32 = 0x004a_c110;
pub(crate) const NPC_GET_CLASS: u32 = 0x0050_2430;
pub(crate) const NPC_GET_SEX: u32 = 0x005f_0cc0;
/// `[this + 0x94]` (`middlehighprocess.cpp`): the voice type of a
/// [`FORM_TYPE_VOICE_SOURCE`] form.
pub(crate) const FORM_GET_VOICE_TYPE: u32 = 0x0091_85e0;

/// `GetInCell`'s cache: the last reference, cell and result (`float`).
pub(crate) const IN_CELL_CACHE_REFERENCE: u32 = 0x011c_aaf4;
pub(crate) const IN_CELL_CACHE_CELL: u32 = 0x011c_aaf8;
pub(crate) const IN_CELL_CACHE_RESULT: u32 = 0x011c_aafc;
/// `GetIsVoiceType`'s cache: the last reference and its voice type.
pub(crate) const VOICE_TYPE_CACHE_VOICE: u32 = 0x011c_ab14;
pub(crate) const VOICE_TYPE_CACHE_REFERENCE: u32 = 0x011c_ab18;
/// `GetIsPlayableRace`'s cache: the last reference and its result (`float`).
pub(crate) const PLAYABLE_RACE_CACHE_REFERENCE: u32 = 0x011c_ab1c;
pub(crate) const PLAYABLE_RACE_CACHE_RESULT: u32 = 0x011c_ab20;
/// `GetInFaction`'s cache: the last faction, reference and result (`float`).
pub(crate) const IN_FACTION_CACHE_FACTION: u32 = 0x011c_ab00;
pub(crate) const IN_FACTION_CACHE_REFERENCE: u32 = 0x011c_ab04;
pub(crate) const IN_FACTION_CACHE_RESULT: u32 = 0x011c_ab08;
/// `GetIsID`'s cache: the last reference's original base form and the
/// reference.
pub(crate) const IS_ID_CACHE_BASE: u32 = 0x011c_ab0c;
pub(crate) const IS_ID_CACHE_REFERENCE: u32 = 0x011c_ab10;
/// `GetDisposition`'s cache: the last actor, target and result (`float`).
pub(crate) const DISPOSITION_CACHE_ACTOR: u32 = 0x011c_aae8;
pub(crate) const DISPOSITION_CACHE_TARGET: u32 = 0x011c_aaec;
pub(crate) const DISPOSITION_CACHE_RESULT: u32 = 0x011c_aaf0;

/// `Actor::IsInFaction` (Xbox PDB): `this` the actor and the faction (may be
/// null), byte result.
pub(crate) const ACTOR_IS_IN_FACTION: u32 = 0x008b_8e90;
/// `Actor::GetFactionRank` (Xbox PDB): `this` the actor and (faction, is the
/// player), an `int` rank.
pub(crate) const ACTOR_GET_FACTION_RANK: u32 = 0x008b_8290;
/// `ExtraDataList::GetLevCreaOriginalBase` (Xbox PDB): `this` the extra data
/// list; the original base form of a leveled creature, or null.
pub(crate) const EXTRA_DATA_LIST_GET_LEV_CREA_ORIGINAL_BASE: u32 = 0x0042_16f0;
/// Virtual (vtable `+0xe8`) on the form parameter of `GetIsUsedItem`: a bool
/// that lets the function compare the form with the used item.
pub(crate) const VSLOT_FORM_COMPARES_USED_ITEM: u32 = 0xe8;
/// Virtual (vtable `+0x1a0`) on a reference, called with 0 (bool): the test
/// behind `IsChild`.
pub(crate) const VSLOT_REF_IS_CHILD: u32 = 0x1a0;
/// Virtual (vtable `+0x344`) on an `Actor`, called with (target, 0): the
/// actor's disposition toward the target (`int`). It is the same slot as
/// [`VSLOT_ACTOR_PREPARE_ATTACK_CHECK`].
pub(crate) const VSLOT_ACTOR_DISPOSITION_TOWARD: u32 = 0x344;
/// Virtual (vtable `+0x2bc`) on a `MobileObject`, called with 0: its facing
/// (a `float` in ST0).
pub(crate) const VSLOT_MOBILE_Z_ANGLE: u32 = 0x2bc;
/// Virtual (vtable `+0x40c`) on the object `GetKnockedState` casts the
/// process to: the knocked state number.
pub(crate) const VSLOT_KNOCKED_STATE: u32 = 0x40c;
/// Virtual (vtable `+0x148`) on the middle-high process: the data of the
/// equipped weapon (null when nothing is out); its form is at `+0x08`.
pub(crate) const VSLOT_PROCESS_WEAPON_DATA: u32 = 0x148;
/// Virtual (vtable `+0x1e4`) on a reference: its animation object.
pub(crate) const VSLOT_REF_GET_ANIMATION: u32 = 0x1e4;

/// `TESIdleManager::GetUsedItem` (Xbox PDB): the global at `0x011cb6a4`.
pub(crate) const IDLE_MANAGER_GET_USED_ITEM: u32 = 0x0060_08f0;
/// The `int` global at `0x01199c8c` (`tesidlemanager.cpp`): the used item's
/// level.
pub(crate) const IDLE_MANAGER_GET_USED_ITEM_LEVEL: u32 = 0x0060_0910;
/// The byte global at `0x011cb6a8` (`tesidlemanager.cpp`): the used item is
/// being activated.
pub(crate) const IDLE_MANAGER_GET_USED_ITEM_ACTIVATE: u32 = 0x0060_0930;
/// `[this + 0x24]` as a `float` (the global form in ECX; the engine map
/// names this body `BSMultiBoundCapsule::QMultiBoundRadius` because of
/// folded code), the value in ST0.
pub(crate) const GLOBAL_GET_VALUE: u32 = 0x0052_6ac0;
/// Byte tests on the actor in ECX (`extradataobjects.cpp`): unconscious, and
/// restrained.
pub(crate) const ACTOR_IS_UNCONSCIOUS: u32 = 0x0043_7bd0;
pub(crate) const ACTOR_IS_RESTRAINED: u32 = 0x0043_7bf0;
/// The random generator singleton (`bgsdestructibleobjectform.cpp`) and
/// `BSRandom::UnsignedInt` (Xbox PDB): `this` the generator and a maximum.
pub(crate) const RANDOM_GET_INSTANCE: u32 = 0x0047_6c00;
pub(crate) const RANDOM_UNSIGNED_INT: u32 = 0x00aa_5230;
/// An actor's level (`actor.cpp`), a `u16` in AX.
pub(crate) const ACTOR_GET_LEVEL: u32 = 0x0087_f9f0;
/// Dead count of a form (`tes.cpp`): `this` the `TES*` global and the form,
/// a `short` in AX.
pub(crate) const TES_GET_DEAD_COUNT: u32 = 0x0045_9000;
/// `Actor::GetAlert` (Xbox PDB): a byte.
pub(crate) const ACTOR_GET_ALERT: u32 = 0x008a_5e80;
/// `Actor::IsWeaponDrawn` (Xbox PDB): a byte.
pub(crate) const ACTOR_IS_WEAPON_DRAWN: u32 = 0x008a_16d0;
/// Byte test of the actor in ECX behind `IsWaiting` (`actor.cpp`).
pub(crate) const ACTOR_IS_WAITING: u32 = 0x008a_6210;
/// `this - other` into an out vector (`NiPoint3`): `this`, the out vector
/// and the other vector.
pub(crate) const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
/// The Z angle of the vector at the argument (cdecl, `float` in ST0).
pub(crate) const GET_Z_ANGLE_FROM_VECTOR: u32 = 0x004b_13c0;
/// Byte test of the animation in ECX (`animation.cpp`): the special idle is
/// done playing.
pub(crate) const ANIMATION_SPECIAL_IDLE_DONE_PLAYING: u32 = 0x0049_85f0;
/// `[this + 0x08]` of the weapon data returned by [`VSLOT_PROCESS_WEAPON_DATA`]:
/// the weapon form.
pub(crate) const WEAPON_DATA_GET_FORM: u32 = 0x0044_ddc0;
/// The global at `0x011ca278`: the form `IsWeaponInList` falls back on when
/// no weapon is equipped.
pub(crate) const FALLBACK_WEAPON: u32 = 0x011c_a278;
/// `[this + 0xf4]` of a weapon form, a sign-extended byte: its index into
/// the table of animation types at [`WEAPON_ANIM_TYPE_TABLE`].
pub(crate) const WEAPON_GET_ANIM_TYPE_INDEX: u32 = 0x0044_6390;
pub(crate) const WEAPON_ANIM_TYPE_TABLE: u32 = 0x0118_a838;
/// `[this + 0x15c]` of a weapon form: the value `IsWeaponSkillType` compares
/// (the engine map names this body `MiddleHighProcess::GetLastBoundWeapon`
/// because of folded code).
pub(crate) const WEAPON_GET_SKILL: u32 = 0x008d_85e0;
/// `MobileObject::GetCurrentPackage` (Xbox PDB): virtual `+0x27c` of the
/// object at `+0x68`, or 0.
pub(crate) const MOBILE_GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
/// `[this + 0x20]` of a package, a sign-extended byte: its type number.
pub(crate) const PACKAGE_GET_TYPE: u32 = 0x0041_ca90;
/// The table of seven control names (`char*`) that `GetPlayerControlsDisabled`
/// lists, and the byte of the `PlayerCharacter` that holds the flags.
pub(crate) const CONTROL_NAMES: u32 = 0x0118_c608;
pub(crate) const PLAYER_DISABLED_CONTROLS_OFFSET: u32 = 0x680;
/// `__RTDynamicCast` (cdecl): (object, vfdelta, source type, target type,
/// is reference).
pub(crate) const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The two type descriptors `GetKnockedState` casts between.
pub(crate) const TYPE_SOURCE_OF_KNOCKED_STATE_CAST: u32 = 0x0118_a6b8;
pub(crate) const TYPE_TARGET_OF_KNOCKED_STATE_CAST: u32 = 0x0118_c64c;
/// Node lookups of a reference (`tesobjectrefr.cpp`, `animation.cpp`):
/// `this` and (key, name or flag), returning the node.
pub(crate) const REF_FIND_NODE: u32 = 0x0057_1530;
pub(crate) const REF_FIND_NODE_BY_NAME: u32 = 0x0049_0310;
/// The node key of a reference (`modelloader.cpp`); the node lookups take
/// it with [`fn_005a07f0`].
pub(crate) const REF_GET_NODE_KEY: u32 = 0x0043_fcd0;
/// Tests of a node, cdecl (node, 0), byte result: `bhkNiCollisionObject::
/// GetFaceUp` (Xbox PDB) and its left-up counterpart.
pub(crate) const COLLISION_GET_FACE_UP: u32 = 0x00c6_b7b0;
pub(crate) const COLLISION_GET_LEFT_UP: u32 = 0x00c6_b860;
/// "Bip01 Spine0", the node `IsFacingUp` falls back on.
pub(crate) const SPINE_NODE_NAME: u32 = 0x0103_5604;
/// "UNKNOWN", `GetIsUsedItem`'s name for a missing form.
pub(crate) const UNKNOWN_NAME: u32 = 0x0101_5890;

/// Form type of a `GLOB` (`TESGlobal`) and of a `WEAP` (`TESObjectWEAP`).
pub(crate) const FORM_TYPE_GLOBAL: u32 = 6;
pub(crate) const FORM_TYPE_WEAPON: u32 = 0x28;

/// `GetCurrentAIPackage`'s value for each package type 0 to 31: the address
/// of a `double` in the exe's data, or 0 for the types that are `0.0` (type
/// 0), `1.0` (type 1) and the default (type 17, and any above 31).
pub(crate) const PACKAGE_VALUE_ADDRESS: [u32; 32] = [
    0,
    0,
    0x0101_1590,
    0x0102_1928,
    0x0101_db80,
    0x0103_5708,
    0x0103_5700,
    0x0101_5a38,
    0x0102_40b8,
    0x0103_56f8,
    0x0103_56f0,
    0x0103_56e8,
    0x0103_56a8,
    0x0103_56a0,
    0x0103_56b0,
    0x0103_56b8,
    0x0103_56c0,
    0,
    0x0102_0998,
    0x0102_fc70,
    0x0101_5a40,
    0x0102_40b0,
    0x0102_0758,
    0x0103_5718,
    0x0102_e438,
    0x0103_5710,
    0x0103_56e0,
    0x0103_56d8,
    0x0103_4208,
    0x0103_56d0,
    0x0103_56c8,
    0x0102_f070,
];
/// `-1.0` (`double`): the default of the package switch and the start value
/// of `GetFactionRank` and `GetGlobalValue`.
pub(crate) const DOUBLE_MINUS_ONE: u32 = 0x0101_a6b0;
/// `-pi`, `pi` and `2 * pi` as `double`s (`GetHeadingAngle`).
pub(crate) const DOUBLE_MINUS_PI: u32 = 0x0101_ff58;
pub(crate) const DOUBLE_PI: u32 = 0x0101_ff40;
pub(crate) const DOUBLE_TWO_PI: u32 = 0x0101_ff48;

// Callees and data of the functions from `005a0f90` on.

/// The object at `0x011e0e80`, `this` of the `processlists.cpp` function
/// [`PROCESS_LISTS_CRIME_TEST`].
pub(crate) const PROCESS_LISTS: u32 = 0x011e_0e80;
/// `processlists.cpp`, `this` the object at [`PROCESS_LISTS`], stack
/// arguments (actor, placed reference, actor, crime number or -1, 0, -1),
/// byte result (the test behind `GetCrime`).
pub(crate) const PROCESS_LISTS_CRIME_TEST: u32 = 0x0097_2600;
/// `TESClass::IsGuard` (Xbox PDB): byte, `this` the class.
pub(crate) const CLASS_IS_GUARD: u32 = 0x005f_6e60;
/// `Actor::IsTalking` (Xbox PDB): byte.
pub(crate) const ACTOR_IS_TALKING: u32 = 0x008a_67f0;
/// `TESObjectREFR::GetMobileObjectExtra` (Xbox PDB): the reference as a
/// mobile object, or null.
pub(crate) const REF_GET_MOBILE_OBJECT_EXTRA: u32 = 0x0057_8970;
/// `MobileObject::IsTalking` (Xbox PDB): byte.
pub(crate) const MOBILE_OBJECT_IS_TALKING: u32 = 0x0092_f160;

/// `[this + 0x68]` (`highprocess.cpp` body, the engine map names it
/// `MiddleHighProcess::GetSavedAcquireObject`): on the `TES` global it is
/// `TES::pSky` (Xbox PDB `+0x68`).
pub(crate) const TES_GET_SKY: u32 = 0x008d_8520;
/// `[this + 0xcc]` of the sky, a `float` in ST0 (`tes.cpp`).
pub(crate) const SKY_GET_WIND_SPEED: u32 = 0x0045_36e0;
/// Virtual (vtable `+0x448`) on an actor: byte behind `IsTrespassing`.
pub(crate) const VSLOT_ACTOR_IS_TRESPASSING: u32 = 0x448;
/// `tesobjectcell.cpp`: `this` the cell and (actor), stack argument popped
/// by the callee, byte result (the test behind `IsInMyOwnedCell`).
pub(crate) const CELL_IS_OWNED_BY: u32 = 0x0054_6ca0;
/// `Actor::IsContinuingPackageforPC` (Xbox PDB): byte.
pub(crate) const ACTOR_IS_CONTINUING_PACKAGE_FOR_PC: u32 = 0x008a_69d0;

/// Virtual (vtable `+0x1d0`) on a reference: its loaded node, or null
/// (`0059c4c0` tests it for non-null).
pub(crate) const VSLOT_REF_GET_NODE: u32 = 0x1d0;
/// `modelloader.cpp`: returns the global at `0x01202ddc`, the name (a
/// string handle passed by pointer) `CanHaveFlames` looks up.
pub(crate) const FLAMES_EXTRA_DATA_NAME: u32 = 0x0044_8a80;
/// `NiObjectNET::GetExtraData` (Xbox PDB): `this` the node and (name).
pub(crate) const NODE_GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// Type descriptor `CanHaveFlames` casts the extra data with (the first
/// argument of [`EXTRA_DATA_CAST`]).
pub(crate) const FLAMES_EXTRA_DATA_TYPE: u32 = 0x0120_2de0;
/// `bsfacegenmanager.cpp`: cdecl (type descriptor, object); null when the
/// object is null, else the object converted by the type.
pub(crate) const EXTRA_DATA_CAST: u32 = 0x0065_3270;
/// `modelloader.cpp`: byte, `this` the converted extra data (asks
/// `0x00448a60` with `0x10`).
pub(crate) const FLAMES_EXTRA_DATA_TEST: u32 = 0x0044_8a40;
/// Virtual (vtable `+0x0c`) on the node: its value goes to
/// [`REF_HAS_ADDON_NODES`].
pub(crate) const VSLOT_NODE_FLAMES_ARGUMENT: u32 = 0xc;
/// `TESObjectREFR::HasAddonNodes` (Xbox PDB): cdecl, one argument, byte.
pub(crate) const REF_HAS_ADDON_NODES: u32 = 0x0057_8400;
/// `BGSOpenCloseForm::GetOpenState` (Xbox PDB): cdecl (reference), an `int`.
pub(crate) const OPEN_CLOSE_GET_OPEN_STATE: u32 = 0x0047_b250;

/// Form type of the packages `GetIsCurrentPackage` accepts (the form it
/// compares with the actor's current package).
pub(crate) const FORM_TYPE_PACKAGE: u32 = 0x49;
/// `Actor::GetPackageSetAsPcurrent` (Xbox PDB).
pub(crate) const ACTOR_GET_PACKAGE_SET_AS_CURRENT: u32 = 0x0088_1510;
/// `TESPackage::GetIsCreated` (Xbox PDB): byte.
pub(crate) const PACKAGE_GET_IS_CREATED: u32 = 0x0067_4d40;
/// `ExtraDataList::GetPackageExtra` (Xbox PDB): `this` the extra data list.
pub(crate) const EXTRA_DATA_LIST_GET_PACKAGE_EXTRA: u32 = 0x0041_cb10;

/// Calendar day of the week (`calendar.cpp` region), `this` the
/// [`CALENDAR`] singleton, an `int`.
pub(crate) const CALENDAR_GET_DAY_OF_WEEK: u32 = 0x0086_7ef0;
/// Byte of the player (`playercharacter.cpp`) behind `IsPlayerAMurderer`.
pub(crate) const PLAYER_IS_A_MURDERER: u32 = 0x0095_3f60;
/// `actor.cpp`: `this` the actor and (target), an `int` (the detection
/// level; popped by the callee).
pub(crate) const ACTOR_GET_DETECTION_LEVEL: u32 = 0x008a_81e0;

/// `tesactorbasedata.cpp`: byte test of the faction in ECX (bit `0x08` of
/// its flags at `+0x34`).
pub(crate) const FACTION_TEST_EXPELLED: u32 = 0x0047_d720;
/// Byte test of a reference (`+0x08` has bit `0x00800000`; `GetDestroyed`).
pub(crate) const REF_TEST_DESTROYED: u32 = 0x0047_7ba0;
/// Virtual (vtable `+0x19c`) on an actor: its `MagicTarget`, the `this` of
/// the magic tests below (no stack argument).
pub(crate) const VSLOT_ACTOR_GET_MAGIC_TARGET: u32 = 0x19c;
/// `MagicTarget` tests: `this` the magic target and (effect), byte result
/// (behind `HasMagicEffect`) ...
pub(crate) const MAGIC_TARGET_HAS_EFFECT: u32 = 0x0082_2d80;
/// ... `MagicTarget::IsSpellTarget` (Xbox PDB), (spell, 1), byte ...
pub(crate) const MAGIC_TARGET_IS_SPELL_TARGET: u32 = 0x0082_2b90;
/// ... and `MagicTarget::GetUsageMonitorEffectMagnitude` (Xbox PDB),
/// (effect), a `float` in ST0.
pub(crate) const MAGIC_TARGET_GET_USAGE_MONITOR_MAGNITUDE: u32 = 0x0082_2eb0;
/// The offset of the `MagicTarget` inside an actor that `GetSpellUsageNumber`
/// uses directly.
pub(crate) const ACTOR_MAGIC_TARGET_OFFSET: u32 = 0x94;

/// The object at `0x011f2250` (`this` of `VATS::GetCurrentAction`).
pub(crate) const VATS: u32 = 0x011f_2250;
/// `[this + 0x08]` (same body as [`WEAPON_DATA_GET_FORM`]): the VATS mode.
pub(crate) const VATS_GET_MODE: u32 = 0x0044_ddc0;
/// The mode value `GetVATSTargetHeight` requires.
pub(crate) const VATS_MODE_TARGETING: u32 = 4;
/// `VATS::GetCurrentAction` (Xbox PDB): the action, or null.
pub(crate) const VATS_GET_CURRENT_ACTION: u32 = 0x009c_71c0;
/// `ActionPoints::IsAttackAction` (Xbox PDB): cdecl (action type), byte.
pub(crate) const ACTION_IS_ATTACK: u32 = 0x0066_dde0;
/// Virtual (vtable `+0x180`) on an actor's base form: its body part data,
/// or null.
pub(crate) const VSLOT_BASE_FORM_BODY_PART_DATA: u32 = 0x180;
/// `BGSBodyPartData::GetBodyPart_ov2` (Xbox PDB): `this` the data and the
/// part number, the part or null.
pub(crate) const BODY_PART_DATA_GET_BODY_PART: u32 = 0x005e_5130;
/// `modelloader.cpp`: the node name of a body part (`this` the part).
pub(crate) const BODY_PART_GET_NODE_NAME: u32 = 0x0043_b230;
/// cdecl (node, name): the named node below the node (`0x00c4b310` with 1).
pub(crate) const NODE_FIND_BY_NAME: u32 = 0x004a_ae30;
/// `this + 0x8c` (`bipedanim.cpp`): the address of the node's world
/// position (3 floats).
pub(crate) const NODE_GET_WORLD_POSITION: u32 = 0x0045_bb80;
/// `NiAVObject::GetWorldBound` (Xbox PDB): `[this + 0x20]`, or the default
/// bound at `0x011f4288` when null.
pub(crate) const NODE_GET_WORLD_BOUND: u32 = 0x0043_d450;
/// Returns `this` (same body as [`LIST_NODE_ITEM_ADDRESS`]); called on a
/// local vector and on the bound, whose first field is its center.
pub(crate) const RETURN_THIS: u32 = 0x0068_15c0;
/// `tesobjectrefr.cpp`: `this` the reference and a number; asks the
/// reference's extra data list (`0x0041b3a0`), byte result.
pub(crate) const REF_EXTRA_DATA_LIST_TEST: u32 = 0x0057_2d30;

/// Virtuals of the player: `GetSandman` and `GetCannibal` (byte results).
pub(crate) const VSLOT_PLAYER_SANDMAN: u32 = 0x36c;
pub(crate) const VSLOT_PLAYER_CANNIBAL: u32 = 0x364;
/// `[this + 0x10c]` of an actor (`highprocess.cpp` body, the engine map
/// names it `MiddleHighProcess::GetLastIdlePlayed`): the number
/// `IsInCritStage` compares with its stage.
pub(crate) const ACTOR_GET_CRIT_STAGE: u32 = 0x005a_29b0;
/// The table of stage names (`char*`) indexed by stage number.
pub(crate) const CRIT_STAGE_NAMES: u32 = 0x0119_bbb0;
/// `actor.cpp`: byte test behind `HasBeenEaten`.
pub(crate) const ACTOR_HAS_BEEN_EATEN: u32 = 0x008a_8170;

/// `[this + 0x18]` of a package: the index into [`PROCEDURE_TABLES`].
pub(crate) const PACKAGE_GET_PROCEDURE_GROUP: u32 = 0x0096_11e0;
/// Table of pointers (indexed by the group) to tables of `int`s (indexed by
/// the process's procedure index) holding the procedure number.
pub(crate) const PROCEDURE_TABLES: u32 = 0x011a_3ff0;
/// Virtuals of the actor's middle-high process: the procedure index
/// (`+0x280`) and the object `fn_005a1210` and `fn_005a1770` read (`+0x274`).
pub(crate) const VSLOT_PROCESS_PROCEDURE_INDEX: u32 = 0x280;
pub(crate) const VSLOT_PROCESS_PROCEDURE_OBJECT: u32 = 0x274;
/// Package type of the combat packages the fleeing test applies to, and
/// `CombatController::IsFleeing` (Xbox PDB name of the byte test run on the
/// package).
pub(crate) const PACKAGE_TYPE_COMBAT: u32 = 0x12;
pub(crate) const COMBAT_CONTROLLER_IS_FLEEING: u32 = 0x0098_1990;
/// The procedure number a fleeing combat package maps to.
pub(crate) const PROCEDURE_FLEEING: u32 = 10;
/// `[this + 0x84]` of the package of type `0x17`: the number
/// `fn_005a1770` returns.
pub(crate) const PACKAGE_GET_NUMBER: u32 = 0x004f_d3c0;
/// `[this + 0x0c]` of the object of `+0x274` (same body as
/// [`REF_GET_ITEM_ID`]).
pub(crate) const PROCEDURE_OBJECT_GET_ID: u32 = 0x0084_e3a0;
/// `[this + 0x08]` of the object of `+0x274` (same body as [`VATS_GET_MODE`]).
pub(crate) const PROCEDURE_OBJECT_GET_VALUE: u32 = 0x0044_ddc0;
/// The two package types `fn_005a1770` distinguishes.
pub(crate) const PACKAGE_TYPE_WITH_PROCESS_OBJECT: u32 = 0xe;
pub(crate) const PACKAGE_TYPE_WITH_NUMBER: u32 = 0x17;
/// `fn_005a1210`'s table: for each procedure number 0 to `0x36`, the address
/// of the `double` it yields in the exe's data, or 0 for the numbers handled
/// in code (0 gives `0.0`, 2 gives `1.0`, `0x2a` is special, and the rest
/// give `-1.0`).
pub(crate) const PROCEDURE_VALUE_ADDRESS: [u32; 0x37] = [
    0,
    0x0101_5a40,
    0,
    0x0101_1590,
    0x0102_40b0,
    0x0102_0758,
    0x0103_5718,
    0x0103_5710,
    0x0103_5700,
    0x0103_5708,
    0x0102_40b8,
    0x0103_56f0,
    0x0101_db80,
    0x0102_1928,
    0x0103_56e8,
    0x0101_5a38,
    0x0102_0998,
    0x0102_fc70,
    0x0102_e438,
    0,
    0x0103_4208,
    0,
    0x0103_56e0,
    0x0103_5840,
    0x0103_5838,
    0x0103_56d8,
    0,
    0x0103_5830,
    0x0103_56d0,
    0x0103_56c8,
    0x0101_db88,
    0x0103_5828,
    0x0102_f070,
    0x0103_56c0,
    0x0103_56b8,
    0x0103_56b0,
    0x0103_56a8,
    0x0103_56a0,
    0x0103_5820,
    0x0103_5818,
    0x0103_5810,
    0x0103_57a8,
    0,
    0x0103_57b0,
    0x0103_5808,
    0,
    0,
    0x0103_57f0,
    0,
    0x0103_57f8,
    0x0103_5800,
    0x0103_57a0,
    0x0101_e2c0,
    0,
    0x0103_56f8,
];
/// Procedure number with the special case, the `double` it starts from, and
/// the two `float` tables it reads (indexed by `+0x04` of the object, and by
/// `value + 1` for object type 3).
pub(crate) const PROCEDURE_SPECIAL: u32 = 0x2a;
pub(crate) const PROCEDURE_SPECIAL_DEFAULT: u32 = 0x0103_57e8;
pub(crate) const PROCEDURE_SPECIAL_TABLE_BY_TYPE: u32 = 0x0103_57b8;
pub(crate) const PROCEDURE_SPECIAL_TABLE_BY_VALUE: u32 = 0x0103_57c8;

/// The byte at `TLS + 0x268`: the condition functions print a debug line
/// for their result when it is set.
pub(crate) fn trace_enabled(e: &mut Engine) -> bool {
    let tls = e.tls();
    e.mem.u8(tls + TLS_TRACE_FLAG) != 0
}

/// The debug line `format(*result)`.
pub(crate) fn trace_result(e: &mut Engine, format: u32, result: Ptr) {
    let value = e.mem.f64(result.addr());
    e.call(DEBUG_PRINT, &args![format, value]);
}

/// The debug line `format(label, *result)`.
pub(crate) fn trace_labeled_result(e: &mut Engine, format: u32, label: u32, result: Ptr) {
    let value = e.mem.f64(result.addr());
    e.call(DEBUG_PRINT, &args![format, label, value]);
}

pub(crate) fn set_result(e: &mut Engine, result: Ptr, value: f64) {
    e.mem.set_f64(result.addr(), value);
}

/// `ref` when it is non-null and virtual `IsActor` says so, else null (the
/// game's `DYNAMIC_CAST<Actor*>`-style test every actor condition opens
/// with).
pub(crate) fn actor_of(e: &mut Engine, reference: Ptr) -> Ptr {
    if !reference.is_null() && e.vcall(reference.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        reference
    } else {
        Ptr::NULL
    }
}

/// The reference's base form when its type is `NPC_`, else null (the test
/// `SameRace` and `SameSex` run on both references).
pub(crate) fn npc_base_form_of(e: &mut Engine, reference: Ptr) -> Ptr {
    if reference.is_null() {
        return Ptr::NULL;
    }
    let base = e.call(REF_GET_BASE_FORM, &args![reference]).ptr::<()>();
    if e.call(FORM_GET_TYPE, &args![base]).u32() == FORM_TYPE_NPC {
        e.call(REF_GET_BASE_FORM, &args![reference]).ptr()
    } else {
        Ptr::NULL
    }
}

/// `form` when it is non-null and has the given `TESForm::cFormType`, else
/// null (the `if (p && p->type == T) typed = p` test the form parameters go
/// through).
pub(crate) fn form_of_type(e: &mut Engine, form: Ptr, form_type: u32) -> Ptr {
    if !form.is_null() && e.call(FORM_GET_TYPE, &args![form]).u32() == form_type {
        form
    } else {
        Ptr::NULL
    }
}

/// `form` when it is non-null and one of the placed reference types
/// (`0x3a` to `0x40`, or `0x69`), else null.
pub(crate) fn placed_reference_of(e: &mut Engine, form: Ptr) -> Ptr {
    if form.is_null() {
        return Ptr::NULL;
    }
    let form_type = e.call(FORM_GET_TYPE, &args![form]).u32();
    if (FORM_TYPE_REFERENCE_FIRST..=FORM_TYPE_REFERENCE_LAST).contains(&form_type)
        || form_type == FORM_TYPE_REFERENCE_EXTRA
    {
        form
    } else {
        Ptr::NULL
    }
}

/// The reference's base form when its type is `form_type`, else null (the
/// test [`npc_base_form_of`] runs for `NPC_`).
pub(crate) fn base_form_of_type(e: &mut Engine, reference: Ptr, form_type: u32) -> Ptr {
    if reference.is_null() {
        return Ptr::NULL;
    }
    let base = e.call(REF_GET_BASE_FORM, &args![reference]).ptr::<()>();
    if e.call(FORM_GET_TYPE, &args![base]).u32() == form_type {
        e.call(REF_GET_BASE_FORM, &args![reference]).ptr()
    } else {
        Ptr::NULL
    }
}

// Translated from 0059bfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDistanceConditionFunction` (Xbox PDB): the distance between
/// the two references. Writes nothing when either is null.
pub fn script_get_distance_condition_function(
    e: &mut Engine,
    reference: Ptr,
    other: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    if !reference.is_null() && !other.is_null() {
        let distance = e
            .call(
                REF_GET_DISTANCE_FROM_REFERENCE,
                &args![reference, other, 1u32, 0u32],
            )
            .f64();
        set_result(e, result, distance);
    }
    if trace_enabled(e) {
        // "GetDistance >> %0.2f"
        trace_result(e, 0x0103_4d0c, result);
    }
    true
}

// Translated from 0059c010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetInZoneConditionFunction` (Xbox PDB): 1.0 when the reference's
/// cell (or, failing that, its worldspace's cell) is the given one.
pub fn script_get_in_zone_condition_function(
    e: &mut Engine,
    reference: Ptr,
    zone: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() && !zone.is_null() {
        let mut found = Ptr::NULL;
        let parent_cell = e.call(0x008d_6f30, &args![reference]).ptr::<()>();
        if !parent_cell.is_null() {
            found = e.call(0x0054_6c20, &args![parent_cell]).ptr();
        }
        if found.is_null() {
            // TESObjectREFR::GetWorldSpace (Xbox PDB)
            let worldspace = e.call(0x0057_5d70, &args![reference]).ptr::<()>();
            if !worldspace.is_null() {
                found = e.call(0x0045_8400, &args![worldspace]).ptr();
            }
        }
        if found == zone {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "GetInZone >> %0.2f"
        trace_result(e, 0x0103_4d24, result);
    }
    true
}

// Translated from 0059c0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPosConditionFunction` (Xbox PDB): one coordinate of the
/// reference's position, selected by the axis letter `'X'` (0x58), `'Y'` or
/// `'Z'`. Any other letter leaves `*result` as it was.
pub fn script_get_pos_condition_function(
    e: &mut Engine,
    reference: Ptr,
    axis: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    if !reference.is_null() {
        let position = e
            .vcall(reference.addr(), VSLOT_GET_POSITION, &args![])
            .u32();
        let coordinates = [
            e.mem.f32(position),
            e.mem.f32(position + 4),
            e.mem.f32(position + 8),
        ];
        match axis {
            0x58 => set_result(e, result, coordinates[0] as f64),
            0x59 => set_result(e, result, coordinates[1] as f64),
            0x5a => set_result(e, result, coordinates[2] as f64),
            _ => {}
        }
        if trace_enabled(e) {
            // "GetPos: %c >> %0.2f"
            trace_labeled_result(e, 0x0103_4d38, axis, result);
        }
    }
    true
}

// Translated from 0059c170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetAngleConditionFunction` (Xbox PDB): one component of the
/// reference's rotation (`00430830` returns the three radians), in degrees.
pub fn script_get_angle_condition_function(
    e: &mut Engine,
    reference: Ptr,
    axis: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    if !reference.is_null() {
        let rotation = e.call(0x0043_0830, &args![reference]).u32();
        let angles = [
            e.mem.f32(rotation),
            e.mem.f32(rotation + 4),
            e.mem.f32(rotation + 8),
        ];
        let degrees_per_radian: f64 = e.global(DEGREES_PER_RADIAN);
        match axis {
            0x58 => set_result(e, result, angles[0] as f64 * degrees_per_radian),
            0x59 => set_result(e, result, angles[1] as f64 * degrees_per_radian),
            0x5a => set_result(e, result, angles[2] as f64 * degrees_per_radian),
            _ => {}
        }
        if trace_enabled(e) {
            // "GetAngle: %c >> %0.2f"
            trace_labeled_result(e, 0x0103_4d4c, axis, result);
        }
    }
    true
}

// Translated from 0059c230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetStartingPosConditionFunction` (Xbox PDB): one coordinate of
/// the position the reference was placed at (virtual `+0x170` fills a
/// 3-float out buffer).
pub fn script_get_starting_pos_condition_function(
    e: &mut Engine,
    reference: Ptr,
    axis: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    if !reference.is_null() {
        let coordinates = e.with_stack(12, |e, buffer| {
            e.vcall(reference.addr(), 0x170, &args![buffer]);
            [
                e.mem.f32(buffer.addr()),
                e.mem.f32(buffer.addr() + 4),
                e.mem.f32(buffer.addr() + 8),
            ]
        });
        match axis {
            0x58 => set_result(e, result, coordinates[0] as f64),
            0x59 => set_result(e, result, coordinates[1] as f64),
            0x5a => set_result(e, result, coordinates[2] as f64),
            _ => {}
        }
        if trace_enabled(e) {
            // "GetStartingPos: %c >> %0.2f"
            trace_labeled_result(e, 0x0103_4d64, axis, result);
        }
    }
    true
}

// Translated from 0059c2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetStartingAngleConditionFunction` (Xbox PDB): one component of
/// the rotation the reference was placed with (virtual `+0x16c` fills a
/// 3-float out buffer), in degrees.
pub fn script_get_starting_angle_condition_function(
    e: &mut Engine,
    reference: Ptr,
    axis: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    if !reference.is_null() {
        let angles = e.with_stack(12, |e, buffer| {
            e.vcall(reference.addr(), 0x16c, &args![buffer]);
            [
                e.mem.f32(buffer.addr()),
                e.mem.f32(buffer.addr() + 4),
                e.mem.f32(buffer.addr() + 8),
            ]
        });
        let degrees_per_radian: f64 = e.global(DEGREES_PER_RADIAN);
        match axis {
            0x58 => set_result(e, result, angles[0] as f64 * degrees_per_radian),
            0x59 => set_result(e, result, angles[1] as f64 * degrees_per_radian),
            0x5a => set_result(e, result, angles[2] as f64 * degrees_per_radian),
            _ => {}
        }
        if trace_enabled(e) {
            // "GetStartingAngle: %c >> %0.2f"
            trace_labeled_result(e, 0x0103_4d80, axis, result);
        }
    }
    true
}

// Translated from 0059c430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetSecondsPassedConditionFunction` (Xbox PDB): when the second
/// argument (`source`) is set, the byte at `+0x28` of it (`00500940` returns
/// `source + 0x18`, the byte is at `+0x10` of that) is non-zero and the
/// float `00598040` reads from it (`[source + 0x3c]`) is positive, that
/// float; otherwise the value `0084d030` returns for the global data object
/// `011f6394`. The first argument is unused.
pub fn script_get_seconds_passed_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    source: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    let mut from_source = false;
    if !source.is_null() {
        let record = e.call(FORM_LIST_GET_LIST, &args![source]).u32();
        // Byte at +0x10 of the object `00500940` returns (`source + 0x18`).
        if e.mem.u8(record + 0x10) != 0 {
            let value = e.call(REF_GET_OWN_SCALE, &args![source]).f64();
            let zero: f64 = e.global(0x0101_2060);
            if value > zero {
                let value = e.call(REF_GET_OWN_SCALE, &args![source]).f64();
                set_result(e, result, value);
                from_source = true;
            }
        }
    }
    if !from_source {
        let value = e.call(0x0084_d030, &args![SECONDS_PASSED_SOURCE]).f64();
        set_result(e, result, value);
    }
    if trace_enabled(e) {
        // "GetSecondsPassed >> %0.2f"
        trace_result(e, 0x0103_4db8, result);
    }
    true
}

// Translated from 0059c4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without debug line: 1.0 when virtual `+0x1d0` of the
/// reference returns non-null, else 0.0.
pub fn fn_0059c4c0(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() && e.vcall(reference.addr(), 0x1d0, &args![]).u32() != 0 {
        set_result(e, result, 1.0);
    }
    true
}

// Translated from 0059c4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetActorValue` condition function: the actor's current value of
/// actor value `actor_value`, read through the `ActorValueOwner` embedded at
/// `+0xa4` of the actor (virtual `+0xc`), or of its base form's owner at
/// `+0x100` when the actor has form flag `0x800`. Writes nothing for a
/// non-actor.
pub fn fn_0059c4f0(
    e: &mut Engine,
    reference: Ptr,
    actor_value: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let value = if !e.call(FORM_HAS_FLAG_0800, &args![actor]).bool() {
            // ActorValueOwner at Actor+0xa4
            e.vcall(actor.addr() + 0xa4, 0xc, &args![actor_value]).f64()
        } else {
            let base = e.call(ACTOR_GET_BASE_FORM, &args![actor]).u32();
            // ActorValueOwner at TESActorBase+0x100
            e.vcall(base + 0x100, 0xc, &args![actor_value]).f64()
        };
        set_result(e, result, value);
        if trace_enabled(e) {
            let name = e.call(ACTOR_VALUE_SCRIPT_NAME, &args![actor_value]).u32();
            // "GetActorValue: %s >> %0.2f"
            trace_labeled_result(e, 0x0103_4dd4, name, result);
        }
    }
    true
}

// Translated from 0059c5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetBaseActorValueConditionFunction` (Xbox PDB): the actor's base
/// value of an actor value (the integer returned by the `ActorValueOwner` at
/// `+0xa4`, virtual `+0x0`). Writes nothing for a non-actor.
pub fn script_get_base_actor_value_condition_function(
    e: &mut Engine,
    reference: Ptr,
    actor_value: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let value = e.vcall(actor.addr() + 0xa4, 0, &args![actor_value]).i32();
        set_result(e, result, value as f64);
        if trace_enabled(e) {
            let name = e.call(ACTOR_VALUE_SCRIPT_NAME, &args![actor_value]).u32();
            // "GetBaseActorValue: %s >> %0.2f"
            trace_labeled_result(e, 0x0103_4df0, name, result);
        }
    }
    true
}

// Translated from 0059c680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetPermanentActorValue` condition function: the same as
/// [`fn_0059c4f0`] through virtual `+0x20` of the `ActorValueOwner` (and
/// with the same `GetActorValue` debug line).
pub fn fn_0059c680(
    e: &mut Engine,
    reference: Ptr,
    actor_value: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let value = if !e.call(FORM_HAS_FLAG_0800, &args![actor]).bool() {
            e.vcall(actor.addr() + 0xa4, 0x20, &args![actor_value])
                .f64()
        } else {
            let base = e.call(ACTOR_GET_BASE_FORM, &args![actor]).u32();
            e.vcall(base + 0x100, 0x20, &args![actor_value]).f64()
        };
        set_result(e, result, value);
        if trace_enabled(e) {
            let name = e.call(ACTOR_VALUE_SCRIPT_NAME, &args![actor_value]).u32();
            // "GetActorValue: %s >> %0.2f"
            trace_labeled_result(e, 0x0103_4dd4, name, result);
        }
    }
    true
}

// Translated from 0059c760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetFatiguePercentageConditionFunction` (Xbox PDB): the actor's
/// fatigue percentage (`Actor::GetFatiguePercentage`). Writes nothing for a
/// non-actor.
pub fn script_get_fatigue_percentage_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let value = e.call(0x0089_3530, &args![actor]).f64();
        set_result(e, result, value);
        if trace_enabled(e) {
            // "GetFatiguePercentage >> %0.2f"
            trace_result(e, 0x0103_4e10, result);
        }
    }
    true
}

// Translated from 0059c7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetHealthPercentageConditionFunction` (Xbox PDB): the actor's
/// health percentage (`Actor::GetHealthPercentage`). Writes nothing for a
/// non-actor.
pub fn script_get_health_percentage_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let value = e.call(0x0089_3590, &args![actor]).f64();
        set_result(e, result, value);
        if trace_enabled(e) {
            // "GetHealthPercentage >> %0.2f"
            trace_result(e, 0x0103_4e30, result);
        }
    }
    true
}

// Translated from 0059c860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetWalkSpeedConditionFunction` (Xbox PDB): the actor's walk
/// speed (`Actor::GetWalkSpeed`). Writes nothing for a non-actor.
pub fn script_get_walk_speed_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let value = e.call(0x0088_4dc0, &args![actor]).f64();
        set_result(e, result, value);
        if trace_enabled(e) {
            // "GetWalkSpeed >> %0.2f"
            trace_result(e, 0x0103_4e50, result);
        }
    }
    true
}

// Translated from 0059c8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetCurrentTimeConditionFunction` (Xbox PDB): the game hour
/// (`Calendar::GetHour` on the calendar singleton). Uses no parameter.
pub fn script_get_current_time_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let hour = e.call(0x0086_7da0, &args![CALENDAR]).f64();
    set_result(e, result, hour);
    if trace_enabled(e) {
        // "GetCurrentTime >> %0.2f"
        trace_result(e, 0x0103_4e68, result);
    }
    true
}

// Translated from 0059c930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetScaleConditionFunction` (Xbox PDB): the reference's own scale
/// (`[ref + 0x3c]`); the debug line also prints `TESObjectREFR::GetScale`.
pub fn script_get_scale_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    if !reference.is_null() {
        let scale = e.call(REF_GET_OWN_SCALE, &args![reference]).f64();
        set_result(e, result, scale);
        if trace_enabled(e) {
            let with_base = e.call(REF_GET_SCALE, &args![reference]).f64();
            let shown = e.mem.f64(result.addr());
            // "GetScale >> %0.2f (with base %0.2f)"
            e.call(DEBUG_PRINT, &args![0x0103_4e80u32, shown, with_base]);
        }
    }
    true
}

/// The player case of `GetLOS` (the body of `if (actor == PlayerCharacter)`
/// in `0059c990`): the player sees `target` when the target is in the
/// player's view (an animation-data test on the target's bound, or on a
/// bound built from its position), and then a ray from the player's eye is
/// not blocked at one of three heights, or `Actor::LineOfSight` holds.
///
/// `frame` is the game's stack frame: the original keeps these locals at
/// fixed `ebp` offsets, and this keeps them there so that the blocks are as
/// far apart as the game lays them out.
pub(crate) fn get_los_player_case(e: &mut Engine, player: Ptr, target: Ptr, result: Ptr) {
    let mut visible = false;
    let tes_singleton = e.call(0x0045_c670, &args![]).u32();
    let tes_cell = e.call(0x0055_8310, &args![tes_singleton]).u32();
    let tes_singleton = e.call(0x0045_c670, &args![]).u32();
    // BSFaceGenNiNode::GetAnimationData (Xbox PDB) on that object.
    let animation_data = e.call(0x0066_29f0, &args![tes_singleton]).u32();

    let bound_owner = if e.vcall(target.addr(), 0x1d0, &args![]).u32() != 0 {
        let owner = e.vcall(target.addr(), 0x1d0, &args![]).u32();
        e.vcall(owner, 0xc, &args![]).u32()
    } else {
        0
    };
    if bound_owner != 0
        && e.call(0x004b_5fc0, &args![bound_owner, animation_data])
            .bool()
    {
        visible = true;
    } else if e.call(0x0044_4ed0, &args![target]).bool() {
        // A 16-byte bound (centre and radius) built on the stack.
        e.with_stack(0x10, |e, bound| {
            e.call(0x0062_40d0, &args![bound]);
            let position = e.vcall(target.addr(), VSLOT_GET_POSITION, &args![]).u32();
            e.call(0x0098_ddd0, &args![bound, position]);
            e.call(0x0063_f790, &args![bound, 1.0f32]);
            if e.call(0x004b_5ff0, &args![bound, animation_data]).bool() {
                visible = true;
            }
        });
    }
    if !visible {
        return;
    }

    let mut found = false;
    e.with_stack(0x200, |e, frame| {
        let ebp = frame.addr() + 0x200;
        let ray_origin = ebp - 0x50;
        let pick_data = ebp - 0x100;
        let filter_target = ebp - 0x104;
        let collector = ebp - 0x190;
        let ray_end = ebp - 0x19c;
        let ray_end_z = ebp - 0x194;
        let filter_bits = ebp - 0x1a0;
        let filter_info = ebp - 0x1a4;
        let bound_max_buffer = ebp - 0x1b0;
        let bound_min_buffer = ebp - 0x1bc;

        let origin = e.call(0x0043_c490, &args![tes_cell]).u32();
        for i in 0..3 {
            let word = e.mem.u32(origin + 4 * i);
            e.mem.set_u32(ray_origin + 4 * i, word);
        }
        // The pick data (a 0xb0-byte object, constructed by 004a3c20).
        e.call(0x004a_3c20, &args![pick_data]);
        e.mem.set_u32(filter_target, 0);
        if e.vcall(target.addr(), VSLOT_IS_MOBILE_OBJECT, &args![])
            .bool()
        {
            e.mem.set_u32(filter_target, target.addr());
        }
        let filter = e.mem.u32(filter_target);
        // The ray-hit collector built for collision layer 0x25 and the filter.
        e.call(0x0062_a190, &args![collector, 0x25u32, filter]);
        fn_0059ceb0(e, Ptr::new(pick_data), Ptr::new(collector));

        let position = e.vcall(target.addr(), VSLOT_GET_POSITION, &args![]).u32();
        for i in 0..3 {
            let word = e.mem.u32(position + 4 * i);
            e.mem.set_u32(ray_end + 4 * i, word);
        }
        e.call(0x004a_3da0, &args![pick_data, ray_origin]);
        e.call(0x0093_1ed0, &args![player, filter_bits]);
        e.call(0x008c_71b0, &args![filter_info, 0u32]);
        e.call(0x004a_39f0, &args![filter_info, 0x25u32]);
        let high_bits = e.call(0x004a_3a20, &args![filter_bits]).u32();
        fn_0059ce80(e, Ptr::new(filter_info), high_bits);
        let info = e.mem.u32(filter_info);
        e.call(0x004a_3f70, &args![pick_data, info]);

        let bound_max = e
            .vcall(target.addr(), 0x1dc, &args![bound_max_buffer])
            .u32();
        let bound_min = e
            .vcall(target.addr(), 0x1d8, &args![bound_min_buffer])
            .u32();
        let height = (e.mem.f32(bound_max + 8) as f64 - e.mem.f32(bound_min + 8) as f64) as f32;
        let eye_z = e.mem.f32(ray_end_z);
        let fractions = [0x0101_de30u32, 0x0101_1588, 0x0102_90b0];
        let mut heights = [0f32; 3];
        for (slot, fraction) in heights.iter_mut().zip(fractions) {
            let fraction: f64 = e.global(fraction);
            *slot = (height as f64 * fraction + eye_z as f64) as f32;
        }

        let tes: u32 = e.global(TES);
        for height in heights {
            e.mem.set_f32(ray_end_z, height);
            e.call(0x004a_3eb0, &args![pick_data, ray_end]);
            let picked = e.call(0x0045_8420, &args![tes, pick_data]).u32();
            let picked_reference = if picked != 0 {
                e.call(0x0056_f930, &args![picked]).u32()
            } else {
                0
            };
            if picked == 0 || picked_reference == target.addr() {
                found = true;
                break;
            }
        }

        if found {
            set_result(e, result, 1.0);
        } else {
            let seen = e
                .call(
                    ACTOR_LINE_OF_SIGHT,
                    &args![player, 0u32, target, 1u32, 0u32, 0u32],
                )
                .u8();
            set_result(e, result, seen as f64);
        }
        fn_0059cee0(e, Ptr::new(collector));
    });
}

// Translated from 0059c990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetLOSConditionFunction` (Xbox PDB): 1.0 when the reference (an
/// actor) has line of sight to the target. For the player this is first a
/// view test and ray casts ([`get_los_player_case`]); for any other actor
/// it is `Actor::LineOfSight`. Writes 0.0 otherwise.
///
/// The compiler's exception-unwinding frame and the stack cookie are not
/// translated.
pub fn script_get_los_condition_function(
    e: &mut Engine,
    reference: Ptr,
    target: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if reference.is_null() {
        return true;
    }
    let viewer = actor_of(e, reference);
    if viewer.is_null() || target.is_null() {
        return true;
    }
    let player: u32 = e.global(PLAYER);
    if viewer.addr() == player {
        get_los_player_case(e, viewer, target, result);
    } else {
        let seen = e
            .call(
                ACTOR_LINE_OF_SIGHT,
                &args![viewer, 0u32, target, 1u32, 0u32, 0u32],
            )
            .u8();
        set_result(e, result, seen as f64);
    }
    if trace_enabled(e) {
        let sees = e.mem.f64(result.addr()) != 0.0;
        let target_name = e.call(REF_GET_NAME, &args![target]).u32();
        let viewer_name = e.call(REF_GET_NAME, &args![viewer]).u32();
        // "%s sees %s" / "%s can't see %s"
        let format = if sees { 0x0103_4eb4u32 } else { 0x0103_4ea4 };
        e.call(DEBUG_PRINT, &args![format, viewer_name, target_name]);
    }
    true
}

// Translated from 0059ce80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the upper 16 bits of the word at `this` (the pick filter's
/// collision-group field), keeping the lower 16.
pub fn fn_0059ce80(e: &mut Engine, this: Ptr, high: u32) {
    let low = e.mem.u32(this.addr()) & 0xffff;
    e.mem.set_u32(this.addr(), low);
    e.mem.set_u32(this.addr(), (high << 16) | low);
}

// Translated from 0059ceb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the pick data (`this`) its ray-hit collector: stores the collector
/// at `+0xa4` and clears `+0xa8`.
pub fn fn_0059ceb0(e: &mut Engine, this: Ptr, collector: Ptr) {
    e.mem.set_u32(this.addr() + 0xa4, collector.addr());
    e.mem.set_u32(this.addr() + 0xa8, 0);
}

// Translated from 0059cee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the ray-hit collector `GetLOS` builds (calls `0059cf00`).
pub fn fn_0059cee0(e: &mut Engine, this: Ptr) {
    fn_0059cf00(e, this);
}

// Translated from 0059cf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The collector's destructor body: puts back the vtable of its own class
/// (`01034ec4`, the table with `addRayHit`) and runs the base destructor
/// (`004a3ae0`).
pub fn fn_0059cf00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0103_4ec4);
    e.call(0x004a_3ae0, &args![this]);
}

// Translated from 0059cf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The collector's scalar deleting destructor: destroys it, and frees the
/// memory when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0059cf20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0059cf00(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0059cf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDisabledConditionFunction` (Xbox PDB): with a reference, 1.0
/// when it is disabled (form flag `0x800` and not pending enable, or pending
/// disable). Without one, bit 0 of the byte at `+0x04` of the parameter
/// record.
pub fn script_get_disabled_condition_function(
    e: &mut Engine,
    reference: Ptr,
    record: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    if !reference.is_null() {
        set_result(e, result, 0.0);
        let disabled = (e.call(FORM_HAS_FLAG_0800, &args![reference]).bool()
            && !e.call(0x005a_a680, &args![reference]).bool())
            || e.call(0x005a_a630, &args![reference]).bool();
        if disabled {
            set_result(e, result, 1.0);
        }
    } else if !record.is_null() {
        // Byte at +0x04 of the record (where `TESForm::cFormType` sits).
        let bit = e.mem.u8(record.addr() + 4) & 1 != 0;
        set_result(e, result, bit as u8 as f64);
    }
    if trace_enabled(e) {
        // "GetDisabled >> %0.f"
        trace_result(e, 0x0103_4ecc, result);
    }
    true
}

// Translated from 0059d010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetLockedConditionFunction` (Xbox PDB): 2.0 for a broken lock
/// (or a terminal the player has fully hacked into), 1.0 for a locked lock
/// (or a terminal that is not unlocked), else 0.0.
pub fn script_get_locked_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        let lock = e.call(REF_GET_LOCK, &args![reference]).ptr::<()>();
        if !lock.is_null() {
            // REFR_LOCK::IsBroken (Xbox PDB)
            if e.call(0x0043_0ae0, &args![lock]).bool() {
                let two: f64 = e.global(0x0101_1590);
                set_result(e, result, two);
            } else if e.call(0x0050_21a0, &args![lock]).bool() {
                set_result(e, result, 1.0);
            }
        } else {
            let base = e.call(REF_GET_BASE_FORM, &args![reference]).ptr::<()>();
            if e.call(FORM_GET_TYPE, &args![base]).u32() == FORM_TYPE_TERMINAL {
                // PlayerCharacter::GetTerminalAccess (Xbox PDB)
                let player: u32 = e.global(PLAYER);
                let access = e.call(0x0096_6c60, &args![player, reference]).u32();
                if access == 2 {
                    let two: f64 = e.global(0x0101_1590);
                    set_result(e, result, two);
                } else {
                    let terminal = e.call(REF_GET_BASE_FORM, &args![reference]).u32();
                    // BGSTerminal::IsUnlocked (Xbox PDB)
                    if !e.call(0x0050_1ae0, &args![terminal, reference]).bool() {
                        set_result(e, result, 1.0);
                    }
                }
            }
        }
    }
    if trace_enabled(e) {
        // "GetLocked >> %0.f"
        trace_result(e, 0x0103_4ee0, result);
    }
    true
}

// Translated from 0059d100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetLockLevelConditionFunction` (Xbox PDB): the lock's level
/// (`REFR_LOCK::GetLevel`), or for a terminal -1.0 when unlocked and its hack
/// difficulty's lock level otherwise. 0.0 for anything else.
pub fn script_get_lock_level_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        let lock = e.call(REF_GET_LOCK, &args![reference]).ptr::<()>();
        if !lock.is_null() {
            // REFR_LOCK::GetLevel (Xbox PDB), with the reference
            let level = e.call(0x0043_0a10, &args![lock, reference]).i32();
            set_result(e, result, level as f64);
        } else {
            let base = e.call(REF_GET_BASE_FORM, &args![reference]).ptr::<()>();
            if e.call(FORM_GET_TYPE, &args![base]).u32() == FORM_TYPE_TERMINAL {
                let terminal = e.call(REF_GET_BASE_FORM, &args![reference]).u32();
                // BGSTerminal::IsUnlocked (Xbox PDB)
                if e.call(0x0050_1ae0, &args![terminal, reference]).bool() {
                    let minus_one: f64 = e.global(0x0101_a6b0);
                    set_result(e, result, minus_one);
                } else {
                    // BGSTerminal::GetHackDifficultyLockLevel (Xbox PDB)
                    let level = e.call(0x0050_11a0, &args![terminal, reference]).i32();
                    set_result(e, result, level as f64);
                }
            }
        }
    }
    if trace_enabled(e) {
        // "GetLockLevel >> %0.f"
        trace_result(e, 0x0103_4ef4, result);
    }
    true
}

// Translated from 0059d1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsLockBrokenConditionFunction` (Xbox PDB): 1.0 when the
/// reference has a broken lock, else 0.0.
pub fn script_get_is_lock_broken_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        let lock = e.call(REF_GET_LOCK, &args![reference]).ptr::<()>();
        if !lock.is_null() {
            // REFR_LOCK::IsBroken (Xbox PDB)
            let broken = e.call(0x0043_0ae0, &args![lock]).u8();
            set_result(e, result, broken as f64);
        }
    }
    if trace_enabled(e) {
        // "GetIsLockBroken >> %0.f"
        trace_result(e, 0x0103_4f0c, result);
    }
    true
}

// Translated from 0059d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDiseaseConditionFunction` (Xbox PDB): not implemented in the
/// game, always 0.0 (its debug line says `UNIMPLEMENTED`).
pub fn script_get_disease_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if trace_enabled(e) {
        // "UNIMPLEMENTED: GetDisease >> %0.2f"
        trace_result(e, 0x0103_4f24, result);
    }
    true
}

// Translated from 0059d2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVampireConditionFunction` (Xbox PDB): 1.0 when the actor is a
/// vampire (`0047c850`), else 0.0.
pub fn script_get_vampire_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(0x0047_c850, &args![actor]).bool() {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetVampire >> %0.2f"
        trace_result(e, 0x0103_4f48, result);
    }
    true
}

// Translated from 0059d320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetClothingValueConditionFunction` (Xbox PDB): the character's
/// clothing value (`Character::GetClothingValue`) when the reference's base
/// form is an `NPC_`; leaves `*result` as it was otherwise.
pub fn script_get_clothing_value_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let mut character = Ptr::NULL;
    if !reference.is_null() {
        let base = e.call(REF_GET_BASE_FORM, &args![reference]).ptr::<()>();
        if e.call(FORM_GET_TYPE, &args![base]).u32() == FORM_TYPE_NPC {
            character = reference;
        }
    }
    if !character.is_null() {
        let value = e.call(0x008d_3110, &args![character]).f64();
        set_result(e, result, value);
    }
    if trace_enabled(e) {
        // "GetClothingValue >> %0.2f"
        trace_result(e, 0x0103_4f5c, result);
    }
    true
}

// Translated from 0059d3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SameFaction` against the player: calls
/// [`script_same_faction_condition_function`] with the player as the second
/// reference.
pub fn fn_0059d3a0(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let player = Ptr::new(e.global::<u32>(PLAYER));
    script_same_faction_condition_function(e, reference, player, 0, result)
}

// Translated from 0059d3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SameRace` against the player (wrapper around
/// [`script_same_race_condition_function`]).
pub fn fn_0059d3c0(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let player = Ptr::new(e.global::<u32>(PLAYER));
    script_same_race_condition_function(e, reference, player, 0, result)
}

// Translated from 0059d3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SameSex` against the player (wrapper around
/// [`script_same_sex_condition_function`]).
pub fn fn_0059d3e0(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let player = Ptr::new(e.global::<u32>(PLAYER));
    script_same_sex_condition_function(e, reference, player, 0, result)
}

// Translated from 0059d400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SameFactionConditionFunction` (Xbox PDB): 1.0 when some faction
/// of the first actor's base data has a rank (not -1) in the second
/// actor's base data (`TESActorBaseData::GetFactionRank`, which takes
/// whether the second reference is the player). Returns without a debug
/// line when either reference is not usable.
pub fn script_same_faction_condition_function(
    e: &mut Engine,
    reference: Ptr,
    other: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if actor.is_null() || other.is_null() {
        return true;
    }
    let actor_base = e.call(ACTOR_GET_BASE_FORM, &args![actor]).u32();
    let other_base = e.call(ACTOR_GET_BASE_FORM, &args![other]).u32();
    if actor_base != 0 && other_base != 0 {
        // The faction list of the actor's base data (+0x30).
        let mut node = e.call(0x005d_8a70, &args![actor_base + 0x30]).ptr::<()>();
        while !node.is_null()
            && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool()
            && e.mem.f64(result.addr()) != 1.0
        {
            let item_address = e.call(LIST_NODE_ITEM_ADDRESS, &args![node]).u32();
            // The node's item is a faction rank record whose first field is
            // the faction (read without a null check, as the game does).
            let faction_rank = e.mem.u32(item_address);
            let faction = e.mem.u32(faction_rank);
            if faction != 0 {
                let is_player = other.addr() == e.global::<u32>(PLAYER);
                let rank = e
                    .call(0x0047_d680, &args![other_base + 0x30, faction, is_player])
                    .i32();
                if rank != -1 {
                    set_result(e, result, 1.0);
                }
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).ptr();
        }
    }
    if trace_enabled(e) {
        // "SameFaction >> %0.2f"
        trace_result(e, 0x0103_4f78, result);
    }
    true
}

// Translated from 0059d540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SameRaceConditionFunction` (Xbox PDB): 1.0 when both references
/// are `NPC_`s of the same race (`004ac110`).
pub fn script_same_race_condition_function(
    e: &mut Engine,
    reference: Ptr,
    other: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let first = npc_base_form_of(e, reference);
    let second = npc_base_form_of(e, other);
    if !first.is_null() && !second.is_null() {
        let first_race = e.call(0x004a_c110, &args![first]).u32();
        let second_race = e.call(0x004a_c110, &args![second]).u32();
        if first_race == second_race {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "SameRace >> %0.2f"
        trace_result(e, 0x0103_4f90, result);
    }
    true
}

// Translated from 0059d610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SameSexConditionFunction` (Xbox PDB): 1.0 when both references
/// are `NPC_`s of the same sex (`TESActorBase::GetSex`, `005f0cc0`).
pub fn script_same_sex_condition_function(
    e: &mut Engine,
    reference: Ptr,
    other: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let first = npc_base_form_of(e, reference);
    let second = npc_base_form_of(e, other);
    if !first.is_null() && !second.is_null() {
        let first_sex = e.call(0x005f_0cc0, &args![first]).i32();
        let second_sex = e.call(0x005f_0cc0, &args![second]).i32();
        if first_sex == second_sex {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "SameSex >> %0.2f"
        trace_result(e, 0x0103_4fa4, result);
    }
    true
}

// Translated from 0059d6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDetectedConditionFunction` (Xbox PDB): 1.0 when the first
/// actor detects the second (`Actor::GetDetectionLevelAgainstActor` above
/// 0). The debug line also prints the second actor's light level (virtual
/// `+0x734` of its process, truncated).
pub fn script_get_detected_condition_function(
    e: &mut Engine,
    reference: Ptr,
    target: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let mut detection_level = 0i32;
    let mut light_level = -1i32;
    let detector = actor_of(e, reference);
    let subject = actor_of(e, target);
    if !detector.is_null()
        && !subject.is_null()
        && !e
            .call(ACTOR_GET_PROCESS, &args![detector])
            .ptr::<()>()
            .is_null()
        && !e
            .call(ACTOR_GET_PROCESS, &args![subject])
            .ptr::<()>()
            .is_null()
    {
        let player: u32 = e.global(PLAYER);
        // Two out bytes on the stack: the detection call's flag at +0 and
        // the player's in-combat out flag at +2.
        detection_level = e.with_stack(4, |e, out| {
            let in_combat = if subject.addr() == player {
                e.call(0x0095_3c50, &args![player, out.byte_add(2)]).u8()
            } else {
                e.call(0x0049_3bb0, &args![subject]).u8()
            };
            e.call(
                0x008a_0d10,
                &args![detector, 0u32, subject, out, 0u32, in_combat, 0u32, 0u32],
            )
            .i32()
        });
        if detection_level > 0 {
            set_result(e, result, 1.0);
        }
        let process = e.call(ACTOR_GET_PROCESS, &args![subject]).u32();
        let light = e.vcall(process, 0x734, &args![]).f64();
        light_level = e.call(FTOL, &args![light]).i32();
    }
    if trace_enabled(e) {
        // "GetDetected >> %i and light %i"
        e.call(
            DEBUG_PRINT,
            &args![0x0103_4fb8u32, detection_level, light_level],
        );
    }
    true
}

// Translated from 0059d840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDeadConditionFunction` (Xbox PDB): 1.0 when the actor is
/// dead (virtual `+0x22c` with 1).
pub fn script_get_dead_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.vcall(actor.addr(), 0x22c, &args![1u32]).bool() {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetDead >> %0.2f"
        trace_result(e, 0x0103_4fd8, result);
    }
    true
}

// Translated from 0059d8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetItemCountConditionFunction` (Xbox PDB): how many of an item
/// the container reference holds (`InventoryChanges::GetObjectCount`); for a
/// form list the sum over its items. A reference without a container leaves
/// `*result` at 0.0 and prints "Calling Reference is not a Container Object".
pub fn script_get_item_count_condition_function(
    e: &mut Engine,
    reference: Ptr,
    form: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if e.call(REF_HAS_CONTAINER, &args![reference]).u32() == 0 {
        if trace_enabled(e) {
            // "Calling Reference is not a Container Object ..."
            e.call(DEBUG_PRINT, &args![0x0103_5004u32]);
        }
        return true;
    }
    let mut item = Ptr::NULL;
    if !form.is_null() && e.vcall(form.addr(), VSLOT_IS_BOUND_OBJECT, &args![]).bool() {
        item = form;
    }
    let inventory = if reference.is_null() {
        Ptr::NULL
    } else {
        e.call(GET_INVENTORY_CHANGES, &args![reference]).ptr::<()>()
    };
    if !inventory.is_null() {
        if !item.is_null() {
            let count = e
                .call(INVENTORY_GET_OBJECT_COUNT, &args![inventory, item])
                .u32();
            let count = e.call(ITEM_COUNT_FILTER, &args![count]).i32();
            set_result(e, result, count as f64);
        } else if !form.is_null()
            && e.call(FORM_GET_TYPE, &args![form]).u32() == FORM_TYPE_FORM_LIST
        {
            let mut node = e.call(FORM_LIST_GET_LIST, &args![form]).ptr::<()>();
            while !node.is_null() && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
                let item_address = e.call(LIST_NODE_ITEM_ADDRESS, &args![node]).u32();
                let entry = e.mem.u32(item_address);
                node = e.call(LIST_NODE_NEXT, &args![node]).ptr();
                if e.vcall(entry, VSLOT_IS_BOUND_OBJECT, &args![]).bool() {
                    let count = e
                        .call(INVENTORY_GET_OBJECT_COUNT, &args![inventory, entry])
                        .u32();
                    let count = e.call(ITEM_COUNT_FILTER, &args![count]).i32();
                    let total = count as f64 + e.mem.f64(result.addr());
                    set_result(e, result, total);
                }
            }
        }
    }
    if trace_enabled(e) {
        // "GetItemCount >> %0.2f"
        trace_result(e, 0x0103_4fec, result);
    }
    true
}

// Translated from 0059da90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetEquippedConditionFunction` (Xbox PDB): 1.0 when the actor
/// wears or wields the item, or any item of the form list
/// (`InventoryChanges::WearingObject`).
pub fn script_get_equipped_condition_function(
    e: &mut Engine,
    reference: Ptr,
    form: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    let mut item = form;
    if !actor.is_null() && !item.is_null() {
        let mut node: Ptr = Ptr::NULL;
        if e.call(FORM_GET_TYPE, &args![item]).u32() == FORM_TYPE_FORM_LIST {
            node = e.call(FORM_LIST_GET_LIST, &args![item]).ptr();
            let item_address = e.call(LIST_NODE_ITEM_ADDRESS, &args![node]).u32();
            item = Ptr::new(e.mem.u32(item_address));
        }
        while !item.is_null() {
            if e.vcall(item.addr(), VSLOT_IS_BOUND_OBJECT, &args![]).bool() {
                let inventory = e.call(GET_INVENTORY_CHANGES, &args![actor]).ptr::<()>();
                // InventoryChanges::WearingObject (Xbox PDB)
                if !inventory.is_null()
                    && e.call(0x004b_fda0, &args![inventory, item, 0u32]).u32() != 0
                {
                    set_result(e, result, 1.0);
                    break;
                }
            }
            if !node.is_null() {
                node = e.call(LIST_NODE_NEXT, &args![node]).ptr();
            }
            item = if node.is_null() {
                Ptr::NULL
            } else {
                let item_address = e.call(LIST_NODE_ITEM_ADDRESS, &args![node]).u32();
                Ptr::new(e.mem.u32(item_address))
            };
        }
    }
    if trace_enabled(e) {
        // "GetEquipped >> %0.2f"
        trace_result(e, 0x0103_5030, result);
    }
    true
}

// Translated from 0059dbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetGoldConditionFunction` (Xbox PDB): how many caps (the form
/// `004839c0(0xf)` returns) the container reference holds.
pub fn script_get_gold_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let gold = e.call(0x0048_39c0, &args![0xfu32]).ptr::<()>();
    if !reference.is_null()
        && !gold.is_null()
        && e.call(REF_HAS_CONTAINER, &args![reference]).u32() != 0
    {
        let inventory = e.call(GET_INVENTORY_CHANGES, &args![reference]).ptr::<()>();
        if !inventory.is_null() {
            let count = e
                .call(INVENTORY_GET_OBJECT_COUNT, &args![inventory, gold])
                .i32();
            set_result(e, result, count as f64);
        }
    }
    if trace_enabled(e) {
        // "GetGold >> %0.2f"
        trace_result(e, 0x0103_5048, result);
    }
    true
}

// Translated from 0059c380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetMenuModeConditionFunction` (Xbox PDB): with menu id 0, whether
/// the game is in menu mode; otherwise whether that menu is visible (or the
/// top menu when the byte at `011cab24` is set).
pub fn script_get_menu_mode_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    menu_id: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if menu_id == 0 {
        let in_menu = e.call(INTERFACE_IS_IN_MENU_MODE, &args![]).u8();
        set_result(e, result, in_menu as f64);
    } else if e.mem.u8(MENU_MODE_TOP_ONLY) != 0 {
        if e.call(INTERFACE_IS_TOP_MENU_ID, &args![menu_id]).bool() {
            set_result(e, result, 1.0);
        }
    } else if e
        .call(INTERFACE_IS_MENU_ID_VISIBLE, &args![menu_id, 0u32])
        .bool()
    {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "MenuMode %d >> %0.2f"
        trace_labeled_result(e, 0x0103_4da0, menu_id, result);
    }
    true
}

/// Adds `1.0` (the `double` at [`DOUBLE_ONE`]) to `*result` `times` times,
/// one x87 add and store each, as the fall-through `switch` of `GetSleeping`
/// and `GetSitting` does.
pub(crate) fn add_ones_to_result(e: &mut Engine, result: Ptr, times: u32) {
    for _ in 0..times {
        let one = e.global::<f64>(DOUBLE_ONE);
        let sum = e.mem.f64(result.addr()) + one;
        set_result(e, result, sum);
    }
}

// Translated from 0059dc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetSleepingConditionFunction` (Xbox PDB): the actor's sleep state
/// (virtual `+0x214`) as 0 to 4: states 6 / 7 and 8 / 9 / 10 map to
/// 1 / 2 / 2 / 3 / 4.
pub fn script_get_sleeping_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let state = e.vcall(actor.addr(), VSLOT_SIT_SLEEP_STATE, &args![]).u32();
        let additions = match state {
            10 => 4,
            9 => 3,
            7 | 8 => 2,
            6 => 1,
            _ => 0,
        };
        add_ones_to_result(e, result, additions);
    }
    if trace_enabled(e) {
        // "GetSleeping >> %0.2f"
        trace_result(e, 0x0103_505c, result);
    }
    true
}

// Translated from 0059dd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetSittingConditionFunction` (Xbox PDB): the actor's sit state
/// (virtual `+0x214`) as 0 to 4: states 1 / 2 / 3 / 4 / 5 map to
/// 1 / 2 / 2 / 3 / 4.
pub fn script_get_sitting_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let state = e.vcall(actor.addr(), VSLOT_SIT_SLEEP_STATE, &args![]).u32();
        let additions = match state {
            5 => 4,
            4 => 3,
            2 | 3 => 2,
            1 => 1,
            _ => 0,
        };
        add_ones_to_result(e, result, additions);
    }
    if trace_enabled(e) {
        // "GetSitting >> %0.2f"
        trace_result(e, 0x0103_5074, result);
    }
    true
}

// Translated from 0059de90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetFurnitureMarkerIDConditionFunction` (Xbox PDB): the marker id
/// (virtual `+0x4c4` of the actor's process) of the furniture the actor uses.
/// The result is left alone for a non-actor.
pub fn script_get_furniture_marker_id_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).ptr::<()>();
        let marker_id = e
            .vcall(process.addr(), VSLOT_PROCESS_FURNITURE_MARKER_ID, &args![])
            .u32();
        set_result(e, result, marker_id as f64);
    }
    if trace_enabled(e) {
        // "GetFurnitureMarkerID >> %0.2f"
        trace_result(e, 0x0103_5088, result);
    }
    true
}

// Translated from 0059df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsCurrentFurnitureRefConditionFunction` (Xbox PDB): 1.0 when the
/// furniture reference the actor uses (virtual `+0x4c8` of its process) is
/// the given reference.
pub fn script_is_current_furniture_ref_condition_function(
    e: &mut Engine,
    reference: Ptr,
    furniture: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).ptr::<()>();
        let current = e
            .vcall(process.addr(), VSLOT_PROCESS_FURNITURE_REFERENCE, &args![])
            .ptr::<()>();
        if current == furniture {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "IsCurrentFurnitureRef>> %0.2f"
        trace_result(e, 0x0103_50a8, result);
    }
    true
}

// Translated from 0059dfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsCurrentFurnitureObjConditionFunction` (Xbox PDB): 1.0 when the
/// base form of the furniture the actor uses is the given `FURN`, or is in
/// the given form list.
pub fn script_is_current_furniture_obj_condition_function(
    e: &mut Engine,
    reference: Ptr,
    form: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && !form.is_null() {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).ptr::<()>();
        let furniture = e
            .vcall(process.addr(), VSLOT_PROCESS_FURNITURE_REFERENCE, &args![])
            .ptr::<()>();
        if !furniture.is_null() {
            if e.call(FORM_GET_TYPE, &args![form]).u32() == FORM_TYPE_FURNITURE {
                let base = e.call(REF_GET_BASE_FORM, &args![furniture]).ptr::<()>();
                if base == form {
                    set_result(e, result, 1.0);
                }
            } else if e.call(FORM_GET_TYPE, &args![form]).u32() == FORM_TYPE_FORM_LIST {
                let base = e.call(REF_GET_BASE_FORM, &args![furniture]).u32();
                let listed = e.with_stack(4, |e, local| {
                    e.mem.set_u32(local.addr(), base);
                    let list = e.call(FORM_LIST_GET_LIST, &args![form]).ptr::<()>();
                    e.call(LIST_CONTAINS, &args![list, local]).bool()
                });
                if listed {
                    set_result(e, result, 1.0);
                }
            }
        }
    }
    if trace_enabled(e) {
        // "IsCurrentFurnitureObj>> %0.2f"
        trace_result(e, 0x0103_50c8, result);
    }
    true
}

// Translated from 0059e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetTalkedToPCConditionFunction` (Xbox PDB): 1.0 when virtual
/// `+0x98` of the actor says it has talked to the player. A null reference
/// falls back on the first parameter as the actor candidate (which is then
/// not tested for being an actor).
pub fn script_get_talked_to_pc_condition_function(
    e: &mut Engine,
    reference: Ptr,
    fallback: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let subject = if reference.is_null() {
        fallback
    } else {
        actor_of(e, reference)
    };
    if !subject.is_null() && e.vcall(subject.addr(), VSLOT_TALKED_TO_PC, &args![]).bool() {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetTalkedToPC >> %0.2f"
        trace_result(e, 0x0103_50e8, result);
    }
    true
}

// Translated from 0059e1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GetVariable` condition (no Xbox name; no debug line): the value of the
/// script variable `variable` in the script locals of the given reference
/// (placed reference types: its own, or those of the matching inventory item
/// when it is a carried item) or quest (`QUST`, `+0x5c`).
pub fn fn_0059e1a0(e: &mut Engine, _reference: Ptr, form: Ptr, variable: u32, result: Ptr) -> bool {
    set_result(e, result, 0.0);
    if form.is_null() {
        return true;
    }
    let mut locals: Ptr = Ptr::NULL;
    let form_type = e.call(FORM_GET_TYPE, &args![form]).u32();
    if (FORM_TYPE_REFERENCE_FIRST..=FORM_TYPE_REFERENCE_LAST).contains(&form_type)
        || form_type == FORM_TYPE_REFERENCE_EXTRA
    {
        let extra_list = e.call(REF_GET_EXTRA_DATA_LIST, &args![form]).ptr::<()>();
        let carried_by = e
            .call(EXTRA_DATA_LIST_GET_REFERENCE_POINTER, &args![extra_list])
            .ptr::<()>();
        if carried_by.is_null() {
            locals = e.call(REF_GET_SCRIPT_VARIABLES, &args![form]).ptr();
        } else {
            let extra_list = e.call(REF_GET_EXTRA_DATA_LIST, &args![form]).ptr::<()>();
            let owner = e
                .call(EXTRA_DATA_LIST_GET_REFERENCE_POINTER, &args![extra_list])
                .ptr::<()>();
            let item_id = e.call(REF_GET_ITEM_ID, &args![form]).u32();
            let base = e.call(REF_GET_BASE_FORM, &args![form]).u32();
            let item = e
                .call(REF_GET_INVENTORY_ITEM, &args![owner, base, item_id])
                .ptr::<()>();
            if !item.is_null() {
                locals = e.call(ITEM_CHANGE_GET_SCRIPT_LOCALS, &args![item]).ptr();
            }
            if !item.is_null() {
                e.call(ITEM_CHANGE_DESTROY, &args![item, 1u32]);
            }
        }
    } else if form_type == FORM_TYPE_QUEST {
        locals = Ptr::new(fn_0059e300(e, form));
    }
    if !locals.is_null() && e.mem.u32(locals.addr()) != 0 {
        let value = e
            .call(SCRIPT_LOCALS_GET_VARIABLE, &args![locals, variable, 0u32])
            .f64();
        set_result(e, result, value);
    }
    true
}

// Translated from 0059e300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `[this + 0x5c]` of a quest: its script locals pointer.
pub fn fn_0059e300(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x5c)
}

// Translated from 0059e320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetQuestRunningConditionFunction` (Xbox PDB): 1.0 when the quest
/// is running (`00455620`).
pub fn script_get_quest_running_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    quest: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !quest.is_null() && e.call(QUEST_IS_RUNNING, &args![quest]).bool() {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetQuestRunning >> %0.2f"
        trace_result(e, 0x0103_5100, result);
    }
    true
}

// Translated from 0059e390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetQuestCompletedConditionFunction` (Xbox PDB): 1.0 when the
/// quest's flag byte has bit 2 set ([`fn_0059e400`]).
pub fn script_get_quest_completed_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    quest: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !quest.is_null() && fn_0059e400(e, quest) {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetQuestCompleted >> %0.2f"
        trace_result(e, 0x0103_511c, result);
    }
    true
}

// Translated from 0059e400 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 2 of the quest's flag byte (at `this + 0x3c`) is set.
pub fn fn_0059e400(e: &mut Engine, this: Ptr) -> bool {
    let flags = e.call(QUEST_GET_FLAGS_ADDRESS, &args![this]).u32();
    e.mem.u8(flags) & 2 != 0
}

// Translated from 0059e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetStageConditionFunction` (Xbox PDB): the quest's current stage.
pub fn script_get_stage_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    quest: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !quest.is_null() {
        let stage = e.call(QUEST_GET_CURRENT_STAGE, &args![quest]).u32();
        set_result(e, result, stage as f64);
    }
    if trace_enabled(e) {
        // "GetStage >> %0.2f"
        trace_result(e, 0x0103_5138, result);
    }
    true
}

// Translated from 0059e490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetStageDoneConditionFunction` (Xbox PDB): 1.0 when the quest
/// has completed the stage (low byte of the second parameter).
pub fn script_get_stage_done_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    quest: Ptr,
    stage: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !quest.is_null()
        && e.call(QUEST_IS_STAGE_DONE, &args![quest, stage & 0xff])
            .bool()
    {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetStageDone >> %0.2f"
        trace_result(e, 0x0103_514c, result);
    }
    true
}

// Translated from 0059e510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetFactionRankDifferenceConditionFunction` (Xbox PDB): the
/// faction rank of the actor minus that of the other reference, when both are
/// members. Returns without a debug line when the actor or the other
/// reference is missing.
pub fn script_get_faction_rank_difference_condition_function(
    e: &mut Engine,
    reference: Ptr,
    faction: Ptr,
    other: Ptr,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if actor.is_null() || other.is_null() {
        return true;
    }
    let actor_base = e.call(ACTOR_GET_BASE_FORM, &args![actor]).ptr::<()>();
    let other_base = e.call(ACTOR_GET_BASE_FORM, &args![other]).ptr::<()>();
    let faction = form_of_type(e, faction, FORM_TYPE_FACTION);
    if !actor_base.is_null() && !other_base.is_null() && !faction.is_null() {
        let player = e.global::<u32>(PLAYER);
        let actor_is_player = (actor.addr() == player) as u32;
        let actor_rank = e
            .call(
                ACTOR_BASE_DATA_GET_FACTION_RANK,
                &args![actor_base.addr() + 0x30, faction, actor_is_player],
            )
            .i32();
        let other_is_player = (other.addr() == player) as u32;
        let other_rank = e
            .call(
                ACTOR_BASE_DATA_GET_FACTION_RANK,
                &args![other_base.addr() + 0x30, faction, other_is_player],
            )
            .i32();
        if actor_rank != -1 && other_rank != -1 {
            set_result(e, result, actor_rank.wrapping_sub(other_rank) as f64);
        }
    }
    if trace_enabled(e) {
        // "GetFactionRankDifference >> %0.2f"
        trace_result(e, 0x0103_5164, result);
    }
    true
}

// Translated from 0059e650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetAlarmedConditionFunction` (Xbox PDB): 1.0 when the actor's
/// current procedure name (`0088b7f0`) is "Alarm" (case-insensitive).
pub fn script_get_alarmed_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(ACTOR_GET_PROCEDURE_NAME, &args![actor]).u32() != 0 {
        let name = e.call(ACTOR_GET_PROCEDURE_NAME, &args![actor]).u32();
        // "Alarm"
        if e.call(STRING_COMPARE_NO_CASE, &args![name, 0x0103_519cu32])
            .i32()
            == 0
        {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "GetAlarmed >> %0.2f"
        trace_result(e, 0x0103_5188, result);
    }
    true
}

/// The shared body of `GetIsPleasant` and `GetIsCloudy`: the current
/// weather's share of the sky when it has the flag, plus the share of the
/// weather it is changing from (one minus the share) when that one has the
/// flag.
pub(crate) fn sky_weather_flag_share(
    e: &mut Engine,
    result: Ptr,
    has_flag: fn(&mut Engine, Ptr) -> bool,
    format: u32,
) {
    let sky = e.call(SKY_GET_INSTANCE, &args![]).ptr::<()>();
    let current = e.call(SKY_GET_CURRENT_WEATHER, &args![sky]).ptr::<()>();
    let current_share = if !current.is_null() && has_flag(e, current) {
        e.call(SKY_GET_WEATHER_PERCENT, &args![sky]).f32() as f64
    } else {
        0.0
    };
    set_result(e, result, current_share);
    let last = e.call(SKY_GET_LAST_WEATHER, &args![sky]).ptr::<()>();
    let last_share = if !last.is_null() && has_flag(e, last) {
        1.0 - e.call(SKY_GET_WEATHER_PERCENT, &args![sky]).f32() as f64
    } else {
        0.0
    };
    let total = e.mem.f64(result.addr()) + last_share;
    set_result(e, result, total);
    if trace_enabled(e) {
        trace_result(e, format, result);
    }
}

// Translated from 0059e700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsPleasantConditionFunction` (Xbox PDB): how much of the sky
/// is pleasant weather ([`fn_0059e7d0`]).
pub fn script_get_is_pleasant_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    // "GetIsPleasant >> %0.2f"
    sky_weather_flag_share(e, result, fn_0059e7d0, 0x0103_51a4);
    true
}

// Translated from 0059e7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when flag 1 of the `TESWeather` byte at `+0xeb` is set (pleasant).
pub fn fn_0059e7d0(e: &mut Engine, weather: Ptr) -> bool {
    e.mem.u8(weather.addr() + 0xeb) & 1 != 0
}

// Translated from 0059e7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsCloudyConditionFunction` (Xbox PDB): how much of the sky is
/// cloudy weather ([`fn_0059e8c0`]).
pub fn script_get_is_cloudy_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    // "GetIsCloudy >> %0.2f"
    sky_weather_flag_share(e, result, fn_0059e8c0, 0x0103_51bc);
    true
}

// Translated from 0059e8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when flag 2 of the `TESWeather` byte at `+0xeb` is set (cloudy).
pub fn fn_0059e8c0(e: &mut Engine, weather: Ptr) -> bool {
    e.mem.u8(weather.addr() + 0xeb) & 2 != 0
}

// Translated from 0059e8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsRainingConditionFunction` (Xbox PDB): 1.0 when
/// [`fn_0059e950`] says it rains.
pub fn script_get_is_raining_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let sky = e.call(SKY_GET_INSTANCE, &args![]).ptr::<()>();
    let raining = fn_0059e950(e, sky);
    set_result(e, result, if raining { 1.0 } else { 0.0 });
    if trace_enabled(e) {
        // "GetIsRaining >> %0.2f"
        trace_result(e, 0x0103_51d4, result);
    }
    true
}

/// The test behind `GetIsRaining` and `GetIsSnowing`: the weather being
/// shown (`has_flag` on `Sky::pCurrentWeather`) has passed the threshold at
/// which that weather's precipitation starts (`WEATHER_INTERPOLATE` of its
/// entry 6 below the current weather percentage), or the weather being left
/// (`Sky::pLastWeather`) has not yet fallen below the one of its entry 7.
pub(crate) fn sky_precipitation_active(
    e: &mut Engine,
    sky: Ptr,
    has_flag: fn(&mut Engine, Ptr) -> bool,
) -> bool {
    // Sky::pCurrentWeather (Xbox PDB) +0x10, pLastWeather +0x14,
    // fCurrentWeatherPct +0xf4.
    let current = Ptr::new(e.mem.u32(sky.addr() + 0x10));
    if !current.is_null() && has_flag(e, current) {
        let from = e.global::<f32>(WEATHER_RAIN_FROM);
        let limit = e
            .call(WEATHER_INTERPOLATE, &args![current, 6u32, from, 0.0f32])
            .f32();
        let percent = e.mem.f32(sky.addr() + 0xf4);
        if percent as f64 > limit as f64 {
            return true;
        }
    }
    let last = Ptr::new(e.mem.u32(sky.addr() + 0x14));
    if !last.is_null() && has_flag(e, last) {
        let to = e.global::<f32>(WEATHER_RAIN_TO);
        let limit = e
            .call(WEATHER_INTERPOLATE, &args![last, 7u32, 1.0f32, to])
            .f32();
        let percent = e.mem.f32(sky.addr() + 0xf4);
        if (percent as f64) < limit as f64 {
            return true;
        }
    }
    false
}

// Translated from 0059e950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The rain test of a `Sky` (`this`): [`sky_precipitation_active`] with the
/// precipitation flag (`004ed270`) as the weather test.
pub fn fn_0059e950(e: &mut Engine, sky: Ptr) -> bool {
    sky_precipitation_active(e, sky, |e, weather| {
        e.call(WEATHER_HAS_FLAG_PRECIPITATION, &args![weather])
            .bool()
    })
}

// Translated from 0059ea10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsSnowingConditionFunction` (Xbox PDB): 1.0 when
/// [`fn_0059ea80`] says it snows.
pub fn script_get_is_snowing_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let sky = e.call(SKY_GET_INSTANCE, &args![]).ptr::<()>();
    let snowing = fn_0059ea80(e, sky);
    set_result(e, result, if snowing { 1.0 } else { 0.0 });
    if trace_enabled(e) {
        // "GetIsSnowing >> %0.2f"
        trace_result(e, 0x0103_51ec, result);
    }
    true
}

// Translated from 0059ea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The snow test of a `Sky` (`this`): [`sky_precipitation_active`] with
/// [`fn_0059eb40`] (flag 8 of the weather) as the weather test.
pub fn fn_0059ea80(e: &mut Engine, sky: Ptr) -> bool {
    sky_precipitation_active(e, sky, fn_0059eb40)
}

// Translated from 0059eb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when flag 8 of the `TESWeather` byte at `+0xeb` is set (snow).
pub fn fn_0059eb40(e: &mut Engine, weather: Ptr) -> bool {
    e.mem.u8(weather.addr() + 0xeb) & 8 != 0
}

// Translated from 0059eb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetWeatherPercentConditionFunction` (Xbox PDB): the sky's
/// current weather percentage.
pub fn script_get_weather_percent_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let sky = e.call(SKY_GET_INSTANCE, &args![]).ptr::<()>();
    let percent = e.call(SKY_GET_WEATHER_PERCENT, &args![sky]).f32();
    set_result(e, result, percent as f64);
    if trace_enabled(e) {
        // "GetCurrentWeatherPercent >> %0.2f"
        trace_result(e, 0x0103_5204, result);
    }
    true
}

// Translated from 0059ebb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsCurrentWeatherConditionFunction` (Xbox PDB): 1.0 when the
/// given `WTHR` is the sky's current weather (a null weather matches when
/// the sky has none).
pub fn script_get_is_current_weather_condition_function(
    e: &mut Engine,
    _reference: Ptr,
    weather: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let weather = form_of_type(e, weather, FORM_TYPE_WEATHER);
    let sky = e.call(SKY_GET_INSTANCE, &args![]).ptr::<()>();
    let current = e.call(SKY_GET_CURRENT_WEATHER, &args![sky]).ptr::<()>();
    if weather == current {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetIsCurrentWeather >> %0.2f"
        trace_result(e, 0x0103_5228, result);
    }
    true
}

// Translated from 0059ec30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetAttackedConditionFunction` (Xbox PDB): the actor's virtual
/// `+0x470` (a byte). Unlike its neighbours it does not zero the result
/// first: a non-actor leaves `*result` as it was.
pub fn script_get_attacked_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let attacked = e.vcall(actor.addr(), VSLOT_ATTACKED, &args![]).u8();
        set_result(e, result, attacked as f64);
    }
    if trace_enabled(e) {
        // "GetAttacked >> %0.2f"
        trace_result(e, 0x0103_5248, result);
    }
    true
}

// Translated from 0059ecc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsCreatureConditionFunction` (Xbox PDB): 1.0 when the
/// reference's base form is a `CREA`.
pub fn script_get_is_creature_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        let base = e.call(REF_GET_BASE_FORM, &args![reference]).ptr::<()>();
        if e.call(FORM_GET_TYPE, &args![base]).u32() == FORM_TYPE_CREATURE {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "GetIsCreature >> %0.2f"
        trace_result(e, 0x0103_5260, result);
    }
    true
}

// Translated from 0059ed30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetShouldAttackConditionFunction` (Xbox PDB): 100 when the actor
/// should attack the target (an `ACHR` / `ACRE`), else 0. The result is
/// written only when both are valid and the combat manager has not already
/// answered yes, in which case the function returns without a debug line.
pub fn script_get_should_attack_condition_function(
    e: &mut Engine,
    reference: Ptr,
    target: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    let mut verdict = 0u32;
    let mut attack_target = Ptr::NULL;
    if !target.is_null() {
        let target_type = e.call(FORM_GET_TYPE, &args![target]).u32();
        if (FORM_TYPE_ACTOR_FIRST..=FORM_TYPE_ACTOR_LAST).contains(&target_type) {
            attack_target = target;
        }
    }
    let actor = actor_of(e, reference);
    if !actor.is_null() && !attack_target.is_null() {
        let flagged = e.call(TARGET_GET_FLAG, &args![attack_target]).bool();
        e.vcall(
            actor.addr(),
            VSLOT_ACTOR_PREPARE_ATTACK_CHECK,
            &args![attack_target, 0u32],
        );
        if flagged
            && e.vcall(attack_target.addr(), VSLOT_TARGET_COMBAT_CHECK, &args![])
                .u32()
                != 0
        {
            let manager = e.global::<u32>(COMBAT_MANAGER);
            if e.call(COMBAT_MANAGER_CHECK, &args![manager, actor, attack_target])
                .bool()
            {
                return true;
            }
        }
        // The out structure on the game's stack: 16 bytes, zeroed.
        let should_attack = e.with_stack(16, |e, out| {
            e.call(
                ACTOR_GET_SHOULD_ATTACK_ACTOR,
                &args![actor, attack_target, 0u32, out, 0u32],
            )
            .bool()
        });
        if should_attack {
            verdict = 100;
        }
        set_result(e, result, verdict as f64);
    }
    if trace_enabled(e) {
        // "GetShouldAttack >> %i"
        e.call(DEBUG_PRINT, &args![0x0103_5278u32, verdict]);
    }
    true
}

// Translated from 0059ee80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetInSameCellConditionFunction` (Xbox PDB): 1.0 when the
/// reference and the other (placed) reference are in the same parent cell.
pub fn script_get_in_same_cell_condition_function(
    e: &mut Engine,
    reference: Ptr,
    other: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let other = placed_reference_of(e, other);
    let reference_cell = if reference.is_null() {
        Ptr::NULL
    } else {
        e.call(REF_GET_PARENT_CELL, &args![reference]).ptr::<()>()
    };
    let other_cell = if other.is_null() {
        Ptr::NULL
    } else {
        e.call(REF_GET_PARENT_CELL, &args![other]).ptr::<()>()
    };
    if !reference_cell.is_null() && !other_cell.is_null() && reference_cell == other_cell {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetInSameCell >> %0.2f"
        trace_result(e, 0x0103_5290, result);
    }
    true
}

// Translated from 0059ef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetInCellConditionFunction` (Xbox PDB): 1.0 when the (second
/// parameter or calling) reference's parent cell has a name starting with the
/// given cell's. The last (reference, cell, answer) is cached in globals.
pub fn script_get_in_cell_condition_function(
    e: &mut Engine,
    reference: Ptr,
    cell: Ptr,
    other: Ptr,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let cell = form_of_type(e, cell, FORM_TYPE_CELL);
    let mut subject = placed_reference_of(e, other);
    if subject.is_null() {
        subject = reference;
    }
    if !subject.is_null() {
        if subject.addr() == e.global::<u32>(IN_CELL_CACHE_REFERENCE)
            && e.global::<u32>(IN_CELL_CACHE_CELL) == cell.addr()
        {
            let cached = e.global::<f32>(IN_CELL_CACHE_RESULT);
            set_result(e, result, cached as f64);
        } else {
            let parent_cell = e.call(REF_GET_PARENT_CELL, &args![subject]).ptr::<()>();
            if !parent_cell.is_null() && !cell.is_null() {
                let length = e.call(CELL_GET_NAME_LENGTH, &args![cell]).u32();
                let cell_name = e.vcall(cell.addr(), VSLOT_FORM_GET_NAME, &args![]).u32();
                let parent_name = e
                    .vcall(parent_cell.addr(), VSLOT_FORM_GET_NAME, &args![])
                    .u32();
                let same = e
                    .call(
                        STRING_COMPARE_N_NO_CASE,
                        &args![parent_name, cell_name, length],
                    )
                    .i32();
                if same == 0 {
                    set_result(e, result, 1.0);
                }
            }
            e.set_global::<u32>(IN_CELL_CACHE_REFERENCE, subject.addr());
            e.set_global::<u32>(IN_CELL_CACHE_CELL, cell.addr());
            let answer = e.mem.f64(result.addr());
            e.set_global::<f32>(IN_CELL_CACHE_RESULT, answer as f32);
        }
    }
    if trace_enabled(e) {
        // "GetInCell >> %0.2f"
        trace_result(e, 0x0103_52a8, result);
    }
    true
}

// Translated from 0059f0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetInWorldspaceConditionFunction` (Xbox PDB): 1.0 when the
/// (second parameter or calling) reference is in the given worldspace.
pub fn script_get_in_worldspace_condition_function(
    e: &mut Engine,
    reference: Ptr,
    worldspace: Ptr,
    other: Ptr,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let worldspace = form_of_type(e, worldspace, FORM_TYPE_WORLDSPACE);
    let mut subject = placed_reference_of(e, other);
    if subject.is_null() {
        subject = reference;
    }
    if !worldspace.is_null()
        && e.call(REF_GET_WORLDSPACE, &args![subject]).ptr::<()>() == worldspace
    {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetInWorldspace >> %0.2f"
        trace_result(e, 0x0103_52bc, result);
    }
    true
}

// Translated from 0059f180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsClassConditionFunction` (Xbox PDB): 1.0 when the `NPC_`'s
/// class is the given `CLAS`.
pub fn script_get_is_class_condition_function(
    e: &mut Engine,
    reference: Ptr,
    class: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let npc = npc_base_form_of(e, reference);
    let class = form_of_type(e, class, FORM_TYPE_CLASS);
    if !npc.is_null() && !class.is_null() && e.call(NPC_GET_CLASS, &args![npc]).ptr::<()>() == class
    {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetIsClass >> %0.2f"
        trace_result(e, 0x0103_52d8, result);
    }
    true
}

// Translated from 0059f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsRaceConditionFunction` (Xbox PDB): 1.0 when the `NPC_`'s
/// race is the given `RACE`.
pub fn script_get_is_race_condition_function(
    e: &mut Engine,
    reference: Ptr,
    race: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let npc = npc_base_form_of(e, reference);
    let race = form_of_type(e, race, FORM_TYPE_RACE);
    if !npc.is_null() && !race.is_null() && e.call(NPC_GET_RACE, &args![npc]).ptr::<()>() == race {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetIsRace >> %0.2f"
        trace_result(e, 0x0103_52ec, result);
    }
    true
}

// Translated from 0059f300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsCreatureTypeConditionFunction` (Xbox PDB): 1.0 when the
/// `CREA`'s type byte ([`fn_0059f3a0`], sign-extended) is the given number.
pub fn script_get_is_creature_type_condition_function(
    e: &mut Engine,
    reference: Ptr,
    creature_type: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let creature = base_form_of_type(e, reference, FORM_TYPE_CREATURE);
    if !creature.is_null() {
        let kind = fn_0059f3a0(e, creature) as i8 as i32 as u32;
        if kind == creature_type {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "GetIsCreatureType >> %0.2f"
        trace_result(e, 0x0103_5300, result);
    }
    true
}

// Translated from 0059f3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The type byte of a creature form (`this + 0x12c`).
pub fn fn_0059f3a0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x12c)
}

// Translated from 0059f3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsSexConditionFunction` (Xbox PDB): 1.0 when the `NPC_`'s sex
/// is the given number.
pub fn script_get_is_sex_condition_function(
    e: &mut Engine,
    reference: Ptr,
    sex: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let npc = npc_base_form_of(e, reference);
    if !npc.is_null() && e.call(NPC_GET_SEX, &args![npc]).u32() == sex {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetIsSex >> %0.2f"
        trace_result(e, 0x0103_531c, result);
    }
    true
}

// Translated from 0059f450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsVoiceTypeConditionFunction` (Xbox PDB): 1.0 when the voice
/// type of the reference's base form is the given one. The last (reference,
/// voice type) is cached in globals.
pub fn script_get_is_voice_type_condition_function(
    e: &mut Engine,
    reference: Ptr,
    voice_type: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        let voice = if reference.addr() == e.global::<u32>(VOICE_TYPE_CACHE_REFERENCE) {
            e.global::<u32>(VOICE_TYPE_CACHE_VOICE)
        } else {
            let mut found = 0u32;
            let base = e.call(REF_GET_BASE_FORM, &args![reference]).ptr::<()>();
            let base_type = e.call(FORM_GET_TYPE, &args![base]).i32();
            if base_type == FORM_TYPE_VOICE_SOURCE as i32 {
                let base = e.call(REF_GET_BASE_FORM, &args![reference]).ptr::<()>();
                found = e.call(FORM_GET_VOICE_TYPE, &args![base]).u32();
            } else if base_type > 0x29 && base_type <= FORM_TYPE_CREATURE as i32 {
                // The component at +0x30 of the NPC_ / CREA base form.
                let base = e.call(REF_GET_BASE_FORM, &args![reference]).ptr::<()>();
                found = e
                    .vcall(
                        base.addr() + 0x30,
                        VSLOT_BASE_COMPONENT_VOICE_TYPE,
                        &args![],
                    )
                    .u32();
            }
            e.set_global::<u32>(VOICE_TYPE_CACHE_REFERENCE, reference.addr());
            e.set_global::<u32>(VOICE_TYPE_CACHE_VOICE, found);
            found
        };
        if voice == voice_type {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "GetIsVoiceType >> %0.2f"
        trace_result(e, 0x0103_5330, result);
    }
    true
}

// Translated from 0059f540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsPlayableRaceConditionFunction` (Xbox PDB): 1.0 when the
/// `NPC_`'s race has its playable flag (`0059f610`). The last (reference,
/// answer) is cached in globals and answers first (the debug line is still
/// printed).
pub fn script_get_is_playable_race_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    if e.global::<u32>(PLAYABLE_RACE_CACHE_REFERENCE) == reference.addr() {
        let cached = e.global::<f32>(PLAYABLE_RACE_CACHE_RESULT);
        set_result(e, result, cached as f64);
    } else {
        set_result(e, result, 0.0);
        let npc = npc_base_form_of(e, reference);
        if !npc.is_null() {
            let race = e.call(NPC_GET_RACE, &args![npc]).ptr::<()>();
            if !race.is_null() && e.call(RACE_IS_PLAYABLE, &args![race]).bool() {
                set_result(e, result, 1.0);
            }
        }
        e.set_global::<u32>(PLAYABLE_RACE_CACHE_REFERENCE, reference.addr());
        let answer = e.mem.f64(result.addr());
        e.set_global::<f32>(PLAYABLE_RACE_CACHE_RESULT, answer as f32);
    }
    if trace_enabled(e) {
        // "GetIsPlayableRace >> %0.2f"
        trace_result(e, 0x0103_5348, result);
    }
    true
}

/// The reference as an `Actor` (virtual `IsMobileObject`, vtable `+0xfc`),
/// else null.
pub(crate) fn mobile_object_of(e: &mut Engine, reference: Ptr) -> Ptr {
    if !reference.is_null()
        && e.vcall(reference.addr(), VSLOT_IS_MOBILE_OBJECT, &args![])
            .bool()
    {
        reference
    } else {
        Ptr::NULL
    }
}

/// The form of the weapon the process has out: the weapon data
/// ([`VSLOT_PROCESS_WEAPON_DATA`]) form when that is a `WEAP`, else null
/// (the part `IsWeaponInList` and `GetWeaponAnimType` share).
pub(crate) fn equipped_weapon_of(e: &mut Engine, process: Ptr) -> Ptr {
    let data = e
        .vcall(process.addr(), VSLOT_PROCESS_WEAPON_DATA, &args![])
        .ptr::<()>();
    let mut weapon = Ptr::NULL;
    if !data.is_null() && e.call(WEAPON_DATA_GET_FORM, &args![data]).u32() != 0 {
        let form = e.call(WEAPON_DATA_GET_FORM, &args![data]).ptr::<()>();
        if e.call(FORM_GET_TYPE, &args![form]).u32() == FORM_TYPE_WEAPON {
            weapon = e.call(WEAPON_DATA_GET_FORM, &args![data]).ptr();
        }
    }
    weapon
}

// Translated from 0059f610 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 1 of the `RACE`'s flags (`this + 0x70`) is set: the race
/// is playable.
pub fn fn_0059f610(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x70) & 1 != 0
}

// Translated from 0059f630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetInFactionConditionFunction` (Xbox PDB): 1.0 when the actor is
/// in the faction. The last (reference, faction, answer) is cached in
/// globals and answers first. A reference that is not an actor returns
/// without a debug line (and without caching).
pub fn script_get_in_faction_condition_function(
    e: &mut Engine,
    reference: Ptr,
    faction_form: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let faction = form_of_type(e, faction_form, FORM_TYPE_FACTION);
    if reference.addr() == e.global::<u32>(IN_FACTION_CACHE_REFERENCE)
        && faction.addr() == e.global::<u32>(IN_FACTION_CACHE_FACTION)
    {
        let cached = e.global::<f32>(IN_FACTION_CACHE_RESULT);
        set_result(e, result, cached as f64);
    } else {
        let actor = actor_of(e, reference);
        if actor.is_null() {
            return true;
        }
        if e.call(ACTOR_IS_IN_FACTION, &args![actor, faction]).bool() {
            set_result(e, result, 1.0);
        }
        e.set_global::<u32>(IN_FACTION_CACHE_REFERENCE, reference.addr());
        e.set_global::<u32>(IN_FACTION_CACHE_FACTION, faction.addr());
        let answer = e.mem.f64(result.addr());
        e.set_global::<f32>(IN_FACTION_CACHE_RESULT, answer as f32);
    }
    if trace_enabled(e) {
        // "GetInFaction >> %0.2f"
        trace_result(e, 0x0103_5364, result);
    }
    true
}

// Translated from 0059f730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetIsClass` with the player as the reference (the reference argument
/// is ignored).
pub fn fn_0059f730(e: &mut Engine, _reference: u32, class: Ptr, param2: u32, result: Ptr) -> bool {
    let player = Ptr::new(e.global::<u32>(PLAYER));
    script_get_is_class_condition_function(e, player, class, param2, result)
}

// Translated from 0059f750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetIsRace` with the player as the reference (the reference argument
/// is ignored).
pub fn fn_0059f750(e: &mut Engine, _reference: u32, race: Ptr, param2: u32, result: Ptr) -> bool {
    let player = Ptr::new(e.global::<u32>(PLAYER));
    script_get_is_race_condition_function(e, player, race, param2, result)
}

// Translated from 0059f770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetIsSex` with the player as the reference (the reference argument
/// is ignored).
pub fn fn_0059f770(e: &mut Engine, _reference: u32, sex: u32, param2: u32, result: Ptr) -> bool {
    let player = Ptr::new(e.global::<u32>(PLAYER));
    script_get_is_sex_condition_function(e, player, sex, param2, result)
}

// Translated from 0059f790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetInFaction` with the player as the reference (the reference argument
/// is ignored).
pub fn fn_0059f790(
    e: &mut Engine,
    _reference: u32,
    faction: Ptr,
    param2: u32,
    result: Ptr,
) -> bool {
    let player = Ptr::new(e.global::<u32>(PLAYER));
    script_get_in_faction_condition_function(e, player, faction, param2, result)
}

// Translated from 0059f7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsIDConditionFunction` (Xbox PDB): 1.0 when the reference's
/// original base form (the leveled creature's original base, else its base
/// form) is the given bound object. The last (reference, base) is cached in
/// globals.
pub fn script_get_is_id_condition_function(
    e: &mut Engine,
    reference: Ptr,
    form: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let mut bound = Ptr::NULL;
    if !form.is_null() && e.vcall(form.addr(), VSLOT_IS_BOUND_OBJECT, &args![]).bool() {
        bound = form;
    }
    if !reference.is_null() && !bound.is_null() {
        let base = if reference.addr() == e.global::<u32>(IS_ID_CACHE_REFERENCE) {
            e.global::<u32>(IS_ID_CACHE_BASE)
        } else {
            let extra_list = e
                .call(REF_GET_EXTRA_DATA_LIST, &args![reference])
                .ptr::<()>();
            let mut base = e
                .call(
                    EXTRA_DATA_LIST_GET_LEV_CREA_ORIGINAL_BASE,
                    &args![extra_list],
                )
                .u32();
            if base == 0 {
                base = e.call(REF_GET_BASE_FORM, &args![reference]).u32();
            }
            e.set_global::<u32>(IS_ID_CACHE_REFERENCE, reference.addr());
            e.set_global::<u32>(IS_ID_CACHE_BASE, base);
            base
        };
        if base == bound.addr() {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "GetIsID >> %0.2f"
        trace_result(e, 0x0103_537c, result);
    }
    true
}

// Translated from 0059f890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsInListConditionFunction` (Xbox PDB): 1.0 when the reference's
/// original base form (as in `GetIsID`) is in the form list.
pub fn script_is_in_list_condition_function(
    e: &mut Engine,
    reference: Ptr,
    list_form: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() && !list_form.is_null() {
        let extra_list = e
            .call(REF_GET_EXTRA_DATA_LIST, &args![reference])
            .ptr::<()>();
        let mut base = e
            .call(
                EXTRA_DATA_LIST_GET_LEV_CREA_ORIGINAL_BASE,
                &args![extra_list],
            )
            .u32();
        if base == 0 {
            base = e.call(REF_GET_BASE_FORM, &args![reference]).u32();
        }
        if base != 0 {
            let listed = e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), base);
                let list = e.call(FORM_LIST_GET_LIST, &args![list_form]).ptr::<()>();
                e.call(LIST_CONTAINS, &args![list, local]).bool()
            });
            if listed {
                set_result(e, result, 1.0);
            }
        }
    }
    if trace_enabled(e) {
        // "IsInList >> %0.2f"
        trace_result(e, 0x0103_5390, result);
    }
    true
}

// Translated from 0059f940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsChildConditionFunction` (Xbox PDB): 1.0 when the
/// reference's virtual at `+0x1a0` (called with 0) says so.
pub fn script_get_is_child_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null()
        && e.vcall(reference.addr(), VSLOT_REF_IS_CHILD, &args![0u32])
            .bool()
    {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "IsChild >> %0.2f"
        trace_result(e, 0x0103_53a4, result);
    }
    true
}

// Translated from 0059f9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsUsedItemConditionFunction` (Xbox PDB): 1.0 when the form
/// is the idle manager's used item, or a form list that holds it. The debug
/// line names the form ("UNKNOWN" for none).
pub fn script_get_is_used_item_condition_function(
    e: &mut Engine,
    _reference: u32,
    form: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !form.is_null() {
        let mut matched = false;
        if e.vcall(form.addr(), VSLOT_FORM_COMPARES_USED_ITEM, &args![])
            .bool()
            && e.call(IDLE_MANAGER_GET_USED_ITEM, &args![]).u32() == form.addr()
        {
            set_result(e, result, 1.0);
            matched = true;
        }
        if !matched && e.call(FORM_GET_TYPE, &args![form]).u32() == FORM_TYPE_FORM_LIST {
            let used = e.call(IDLE_MANAGER_GET_USED_ITEM, &args![]).u32();
            let listed = e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), used);
                let list = e.call(FORM_LIST_GET_LIST, &args![form]).ptr::<()>();
                e.call(LIST_CONTAINS, &args![list, local]).bool()
            });
            if listed {
                set_result(e, result, 1.0);
            }
        }
    }
    if trace_enabled(e) {
        let name = if form.is_null() {
            UNKNOWN_NAME
        } else {
            e.vcall(form.addr(), VSLOT_FORM_GET_NAME, &args![]).u32()
        };
        // "GetIsUsedItem '%s' >> %0.2f"
        trace_labeled_result(e, 0x0103_53b8, name, result);
    }
    true
}

// Translated from 0059fa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsUsedItemTypeConditionFunction` (Xbox PDB): 1.0 when the
/// idle manager has a used item whose form type is the given number.
pub fn script_get_is_used_item_type_condition_function(
    e: &mut Engine,
    _reference: u32,
    form_type: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if form_type != 0 && e.call(IDLE_MANAGER_GET_USED_ITEM, &args![]).u32() != 0 {
        let used = e.call(IDLE_MANAGER_GET_USED_ITEM, &args![]).ptr::<()>();
        if e.call(FORM_GET_TYPE, &args![used]).u32() == form_type {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "GetIsUsedItemType >> %0.2f"
        trace_result(e, 0x0103_53d4, result);
    }
    true
}

// Translated from 0059fb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetUsedItemLevelConditionFunction` (Xbox PDB): the used item's
/// level (an `int`).
pub fn script_get_used_item_level_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let level = e.call(IDLE_MANAGER_GET_USED_ITEM_LEVEL, &args![]).i32();
    set_result(e, result, level as f64);
    if trace_enabled(e) {
        // "GetIsUsedItemLevel >> %0.2f"
        trace_result(e, 0x0103_53f0, result);
    }
    true
}

// Translated from 0059fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetUsedItemActivateConditionFunction` (Xbox PDB): 1.0 when the
/// used item is being activated.
pub fn script_get_used_item_activate_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if e.call(IDLE_MANAGER_GET_USED_ITEM_ACTIVATE, &args![]).bool() {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetIsUsedItemActivate >> %0.2f"
        trace_result(e, 0x0103_540c, result);
    }
    true
}

// Translated from 0059fbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsRefConditionFunction` (Xbox PDB): 1.0 when the parameter
/// is a placed reference (types `0x3a` to `0x40`, `0x69`) and is the
/// reference itself.
pub fn script_get_is_ref_condition_function(
    e: &mut Engine,
    reference: Ptr,
    other: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let placed = placed_reference_of(e, other);
    if !reference.is_null() && !placed.is_null() && reference == placed {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetIsRef >> %0.2f"
        trace_result(e, 0x0103_542c, result);
    }
    true
}

// Translated from 0059fc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetFactionRankConditionFunction` (Xbox PDB): the actor's rank in
/// the faction (-1.0 when it is not one, or the parameter is no faction).
/// A reference that is not an actor returns without a debug line.
pub fn script_get_faction_rank_condition_function(
    e: &mut Engine,
    reference: Ptr,
    faction_form: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    let minus_one = e.global::<f64>(DOUBLE_MINUS_ONE);
    set_result(e, result, minus_one);
    let actor = actor_of(e, reference);
    if actor.is_null() {
        return true;
    }
    // The actor's base form is fetched and not used.
    e.call(ACTOR_GET_BASE_FORM, &args![actor]);
    let faction = form_of_type(e, faction_form, FORM_TYPE_FACTION);
    if !faction.is_null() {
        let is_player = actor.addr() == e.global::<u32>(PLAYER);
        let rank = e
            .call(
                ACTOR_GET_FACTION_RANK,
                &args![actor, faction, is_player as u32],
            )
            .i32();
        set_result(e, result, rank as f64);
    }
    if trace_enabled(e) {
        // "GetFactionRank >> %0.2f"
        trace_result(e, 0x0103_5440, result);
    }
    true
}

// Translated from 0059fd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetGlobalValueConditionFunction` (Xbox PDB): the value of the
/// `GLOB` form (-1.0 when the parameter is not one).
pub fn script_get_global_value_condition_function(
    e: &mut Engine,
    _reference: u32,
    global_form: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    let minus_one = e.global::<f64>(DOUBLE_MINUS_ONE);
    set_result(e, result, minus_one);
    let global = form_of_type(e, global_form, FORM_TYPE_GLOBAL);
    if !global.is_null() {
        let value = e.call(GLOBAL_GET_VALUE, &args![global]).f64();
        set_result(e, result, value);
    }
    if trace_enabled(e) {
        // "GetGlobalValue >> %0.2f"
        trace_result(e, 0x0103_5458, result);
    }
    true
}

// Translated from 0059fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDispositionConditionFunction` (Xbox PDB): the first actor's
/// disposition toward the second (virtual `+0x344`). The last (actor,
/// target, answer) is cached in globals. Nothing is printed unless both are
/// actors.
pub fn script_get_disposition_condition_function(
    e: &mut Engine,
    reference: Ptr,
    target_reference: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    let target = actor_of(e, target_reference);
    if !actor.is_null() && !target.is_null() {
        if actor.addr() == e.global::<u32>(DISPOSITION_CACHE_ACTOR)
            && target.addr() == e.global::<u32>(DISPOSITION_CACHE_TARGET)
        {
            let cached = e.global::<f32>(DISPOSITION_CACHE_RESULT);
            set_result(e, result, cached as f64);
        } else {
            let disposition = e
                .vcall(
                    actor.addr(),
                    VSLOT_ACTOR_DISPOSITION_TOWARD,
                    &args![target, 0u32],
                )
                .i32();
            set_result(e, result, disposition as f64);
            e.set_global::<u32>(DISPOSITION_CACHE_TARGET, target.addr());
            e.set_global::<u32>(DISPOSITION_CACHE_ACTOR, actor.addr());
            let answer = e.mem.f64(result.addr());
            e.set_global::<f32>(DISPOSITION_CACHE_RESULT, answer as f32);
        }
        if trace_enabled(e) {
            let value = e.mem.f64(result.addr());
            let target_name = e.call(REF_GET_NAME, &args![target]).u32();
            let actor_name = e.call(REF_GET_NAME, &args![actor]).u32();
            // "%.20s disposition to %.20s is %.1f"
            e.call(
                DEBUG_PRINT,
                &args![0x0103_5470u32, actor_name, target_name, value],
            );
        }
    }
    true
}

// Translated from 0059fee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetUnconsciousConditionFunction` (Xbox PDB): 1.0 when the actor
/// is unconscious. The debug line says which, and only exists for actors.
pub fn script_get_unconscious_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        if e.call(ACTOR_IS_UNCONSCIOUS, &args![actor]).bool() {
            set_result(e, result, 1.0);
        }
        if trace_enabled(e) {
            let name = e.call(REF_GET_NAME, &args![actor]).u32();
            let format = if e.mem.f64(result.addr()) == 0.0 {
                0x0103_5494u32 // "%s is not unconscious"
            } else {
                0x0103_54ac // "%s is unconscious"
            };
            e.call(DEBUG_PRINT, &args![format, name]);
        }
    }
    true
}

// Translated from 0059ff90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetRestrainedConditionFunction` (Xbox PDB): 1.0 when the actor
/// is restrained. The debug line says which, and only exists for actors.
pub fn script_get_restrained_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        if e.call(ACTOR_IS_RESTRAINED, &args![actor]).bool() {
            set_result(e, result, 1.0);
        }
        if trace_enabled(e) {
            let name = e.call(REF_GET_NAME, &args![actor]).u32();
            let format = if e.mem.f64(result.addr()) == 0.0 {
                0x0103_54c0u32 // "%s is not restrained"
            } else {
                0x0103_54d8 // "%s is restrained"
            };
            e.call(DEBUG_PRINT, &args![format, name]);
        }
    }
    true
}

// Translated from 005a0040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetRandomPercentConditionFunction` (Xbox PDB): a random number
/// from 0 up to 100 (`BSRandom::UnsignedInt` with 100, unsigned).
pub fn script_get_random_percent_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let value = fn_005a00a0(e, 100);
    set_result(e, result, value as f64);
    if trace_enabled(e) {
        // "GetRandomPercent >> %0.2f"
        trace_result(e, 0x0103_54ec, result);
    }
    true
}

// Translated from 005a00a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random number up to `maximum` from the random generator singleton
/// (`BSRandom::UnsignedInt`, Xbox PDB).
pub fn fn_005a00a0(e: &mut Engine, maximum: u32) -> u32 {
    let generator = e.call(RANDOM_GET_INSTANCE, &args![]).u32();
    e.call(RANDOM_UNSIGNED_INT, &args![generator, maximum])
        .u32()
}

// Translated from 005a00c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetLevelConditionFunction` (Xbox PDB): the actor's level (a
/// `u16`).
pub fn script_get_level_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let level = e.call(ACTOR_GET_LEVEL, &args![actor]).u32() & 0xffff;
        set_result(e, result, level as f64);
    }
    if trace_enabled(e) {
        // "GetLevel >> %0.2f"
        trace_result(e, 0x0103_5508, result);
    }
    true
}

// Translated from 005a0150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetArmorRatingConditionFunction` (Xbox PDB): the actor's armor
/// rating, the `float` returned by virtual `+0x0c` of the object at actor
/// `+0xa4` (called with 0x12). The debug line only exists for actors.
pub fn script_get_armor_rating_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        // The sub-object at +0xa4 has its own vtable pointer.
        let rating = e.vcall(actor.addr() + 0xa4, 0xc, &args![0x12u32]).f64();
        set_result(e, result, rating);
        if trace_enabled(e) {
            // "Armor Rating: %0.2f"
            trace_result(e, 0x0103_551c, result);
        }
    }
    true
}

// Translated from 005a01f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDeadCountConditionFunction` (Xbox PDB): how many of the form
/// are dead (a `short`, from the `TES` global). The debug line only exists
/// when the parameter is not null.
pub fn script_get_dead_count_condition_function(
    e: &mut Engine,
    _reference: u32,
    form: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !form.is_null() {
        let tes = e.global::<u32>(TES);
        let count = e.call(TES_GET_DEAD_COUNT, &args![tes, form]).u32() as u16 as i16;
        set_result(e, result, count as f64);
        if trace_enabled(e) {
            // "Dead Count: %0.2f"
            trace_result(e, 0x0103_5530, result);
        }
    }
    true
}

// Translated from 005a0260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetAlertConditionFunction` (Xbox PDB): the actor's alert byte.
/// The debug line only exists for actors.
pub fn script_get_alert_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let alert = e.call(ACTOR_GET_ALERT, &args![actor]).u32() & 0xff;
        set_result(e, result, alert as f64);
        if trace_enabled(e) {
            // "GetIsAlerted: %0.2f"
            trace_result(e, 0x0103_5544, result);
        }
    }
    true
}

// Translated from 005a02f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPlayerControlsDisabledConditionFunction` (Xbox PDB): 1.0 when
/// the player has any of the control flags in the parameter disabled. The
/// debug lines list each of the seven controls as enabled or disabled.
pub fn script_get_player_controls_disabled_condition_function(
    e: &mut Engine,
    _reference: u32,
    mask: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let player = Ptr::new(e.global::<u32>(PLAYER));
    if fn_005a03f0(e, player, mask) {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetPlayerControlsDisabled: %0.2f"
        trace_result(e, 0x0103_5580, result);
        let mut flags = fn_005a03d0(e, player) as i8;
        for index in 0..7u32 {
            let name = e.global::<u32>(CONTROL_NAMES + 4 * index);
            if flags & 1 != 0 {
                // "     %s - Disabled"
                e.call(DEBUG_PRINT, &args![0x0103_556cu32, name]);
            } else {
                // "     %s - Enabled"
                e.call(DEBUG_PRINT, &args![0x0103_5558u32, name]);
            }
            flags >>= 1;
        }
    }
    true
}

// Translated from 005a03d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte of the player's disabled-controls flags (`this + 0x680`).
pub fn fn_005a03d0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + PLAYER_DISABLED_CONTROLS_OFFSET)
}

// Translated from 005a03f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the player's disabled-controls byte (`this + 0x680`) has any
/// bit of `mask` (a full `u32`, as the code ands it).
pub fn fn_005a03f0(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u8(this.addr() + PLAYER_DISABLED_CONTROLS_OFFSET) as u32 & mask != 0
}

// Translated from 005a0410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetHeadingAngleConditionFunction` (Xbox PDB): the angle in
/// degrees (-180 to 180) from the reference's facing to the direction of
/// the target reference. Needs a mobile object and a target.
pub fn script_get_heading_angle_condition_function(
    e: &mut Engine,
    reference: Ptr,
    target: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let mobile = mobile_object_of(e, reference);
    if !mobile.is_null() && !target.is_null() {
        let direction_angle = e.with_stack(12, |e, offset| {
            let own_position = e.vcall(mobile.addr(), VSLOT_GET_POSITION, &args![]).u32();
            let target_position = e.vcall(target.addr(), VSLOT_GET_POSITION, &args![]).u32();
            e.call(
                POINT3_SUBTRACT,
                &args![target_position, offset, own_position],
            );
            e.call(GET_Z_ANGLE_FROM_VECTOR, &args![offset]).f64() as f32
        });
        let facing = e
            .vcall(mobile.addr(), VSLOT_MOBILE_Z_ANGLE, &args![0u32])
            .f64();
        let mut angle = direction_angle as f64 - facing;
        let minus_pi = e.global::<f64>(DOUBLE_MINUS_PI);
        let pi = e.global::<f64>(DOUBLE_PI);
        let two_pi = e.global::<f64>(DOUBLE_TWO_PI);
        while angle < minus_pi {
            angle += two_pi;
        }
        while pi < angle {
            angle -= two_pi;
        }
        let degrees = e.global::<f64>(DEGREES_PER_RADIAN);
        set_result(e, result, angle * degrees);
    }
    if trace_enabled(e) {
        // "Heading Angle: %0.2f"
        trace_result(e, 0x0103_55a4, result);
    }
    true
}

// Translated from 005a0550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsWeaponOutConditionFunction` (Xbox PDB): 1.0 when the actor has
/// its weapon drawn.
pub fn script_is_weapon_out_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(ACTOR_IS_WEAPON_DRAWN, &args![actor]).bool() {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "Is Weapon Out >> %0.2f"
        trace_result(e, 0x0103_55bc, result);
    }
    true
}

// Translated from 005a05e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsWeaponInListConditionFunction` (Xbox PDB): 1.0 when the
/// actor's equipped weapon (else the fallback weapon global) is in the form
/// list. Needs an actor with a process and a list.
pub fn script_is_weapon_in_list_condition_function(
    e: &mut Engine,
    reference: Ptr,
    list_form: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null()
        && e.call(ACTOR_GET_PROCESS, &args![actor]).u32() != 0
        && !list_form.is_null()
    {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).ptr::<()>();
        let mut weapon = equipped_weapon_of(e, process);
        if weapon.is_null() {
            weapon = Ptr::new(e.global::<u32>(FALLBACK_WEAPON));
        }
        if !weapon.is_null() {
            let listed = e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), weapon.addr());
                let list = e.call(FORM_LIST_GET_LIST, &args![list_form]).ptr::<()>();
                e.call(LIST_CONTAINS, &args![list, local]).bool()
            });
            if listed {
                set_result(e, result, 1.0);
            }
        }
    }
    if trace_enabled(e) {
        // "IsWeaponInList >> %0.2f"
        trace_result(e, 0x0103_55d4, result);
    }
    true
}

// Translated from 005a0710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsFacingUpConditionFunction` (Xbox PDB): 1.0 when the actor's
/// node (looked up by the node key, else "Bip01 Spine0") is facing up, or
/// when no node is found.
pub fn script_is_facing_up_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let key = e.call(REF_GET_NODE_KEY, &args![actor]).u32();
        let suffix = fn_005a07f0(e);
        let mut node = e
            .call(REF_FIND_NODE, &args![actor, key, suffix])
            .ptr::<()>();
        if node.is_null() {
            node = e
                .call(REF_FIND_NODE_BY_NAME, &args![actor, key, SPINE_NODE_NAME])
                .ptr();
        }
        if node.is_null() || e.call(COLLISION_GET_FACE_UP, &args![node, 0u32]).bool() {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "Is Facing Up >> %0.2f"
        trace_result(e, 0x0103_55ec, result);
    }
    true
}

// Translated from 005a07f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `int` global at `0x011c61b4` (the second argument of the node
/// lookups of `IsFacingUp` and `IsLeftUp`).
pub fn fn_005a07f0(e: &mut Engine) -> u32 {
    e.global::<u32>(0x011c_61b4)
}

// Translated from 005a0800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsLeftUpConditionFunction` (Xbox PDB): 1.0 when the actor's node
/// (looked up by the node key) is left up, or when no node is found.
pub fn script_is_left_up_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let key = e.call(REF_GET_NODE_KEY, &args![actor]).u32();
        let suffix = fn_005a07f0(e);
        let node = e
            .call(REF_FIND_NODE, &args![actor, key, suffix])
            .ptr::<()>();
        if node.is_null() || e.call(COLLISION_GET_LEFT_UP, &args![node, 0u32]).bool() {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "Is Left Up >> %0.2f"
        trace_result(e, 0x0103_5614, result);
    }
    true
}

// Translated from 005a08c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetKnockedStateConditionFunction` (Xbox PDB): the actor's
/// process cast to the class with the knocked state; states 3 and 4 give
/// 1.0, 0, 5 and 6 give 0.0, and 1, 2 (or above 6) leave the 0.0 as is.
pub fn script_get_knocked_state_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        let object = e
            .call(
                RT_DYNAMIC_CAST,
                &args![
                    process,
                    0u32,
                    TYPE_SOURCE_OF_KNOCKED_STATE_CAST,
                    TYPE_TARGET_OF_KNOCKED_STATE_CAST,
                    0u32
                ],
            )
            .ptr::<()>();
        if !object.is_null() {
            let state = e.vcall(object.addr(), VSLOT_KNOCKED_STATE, &args![]).u32();
            match state {
                0 | 5 | 6 => set_result(e, result, 0.0),
                3 | 4 => set_result(e, result, 1.0),
                _ => {}
            }
        }
    }
    if trace_enabled(e) {
        // "Get Knocked State >> %0.2f"
        trace_result(e, 0x0103_5628, result);
    }
    true
}

// Translated from 005a09b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetWeaponAnimTypeConditionFunction` (Xbox PDB): the animation
/// type of the actor's equipped weapon (from the table in the exe's data),
/// or 1.0 when the actor has a process but no weapon.
pub fn script_get_weapon_anim_type_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(ACTOR_GET_PROCESS, &args![actor]).u32() != 0 {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).ptr::<()>();
        let weapon = equipped_weapon_of(e, process);
        if weapon.is_null() {
            set_result(e, result, 1.0);
        } else {
            let index = e.call(WEAPON_GET_ANIM_TYPE_INDEX, &args![weapon]).i32();
            let kind = e
                .mem
                .i32(WEAPON_ANIM_TYPE_TABLE.wrapping_add((index as u32).wrapping_mul(4)));
            set_result(e, result, kind as f64);
        }
    }
    if trace_enabled(e) {
        // "Get Weapon Anim >> %0.2f"
        trace_result(e, 0x0103_5644, result);
    }
    true
}

// Translated from 005a0ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsWeaponSkillTypeConditionFunction` (Xbox PDB): 1.0 when the
/// reference's base weapon has the given skill (a reference whose base is
/// no weapon matches skill `0x2d`). Returns false (no debug line) when the
/// base is no weapon and the skill is not `0x2d`.
pub fn script_is_weapon_skill_type_condition_function(
    e: &mut Engine,
    reference: Ptr,
    skill: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let mut weapon: Ptr = Ptr::NULL;
    let base = e.call(REF_GET_BASE_FORM, &args![reference]).ptr::<()>();
    if e.call(FORM_GET_TYPE, &args![base]).u32() == FORM_TYPE_WEAPON {
        weapon = e.call(REF_GET_BASE_FORM, &args![reference]).ptr();
    }
    if weapon.is_null() {
        if skill == 0x2d {
            set_result(e, result, 1.0);
        } else {
            return false;
        }
    } else if e.call(WEAPON_GET_SKILL, &args![weapon]).u32() == skill {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "Is Weapon Skill Type >> %0.2f"
        trace_result(e, 0x0103_5660, result);
    }
    true
}

// Translated from 005a0b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetCurrentAIPackageConditionFunction` (Xbox PDB): a number for
/// the type of the actor's current package (a switch of `double` constants
/// in the exe's data: 0.0, 1.0, 2.0, 3.0, ... up to 37.0; -1.0 for a type
/// without a case). Stays 0.0 without a process or package.
pub fn script_get_current_ai_package_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null()
        && e.call(ACTOR_GET_PROCESS, &args![actor]).u32() != 0
        && e.call(MOBILE_GET_CURRENT_PACKAGE, &args![actor]).u32() != 0
    {
        let package = e
            .call(MOBILE_GET_CURRENT_PACKAGE, &args![actor])
            .ptr::<()>();
        let kind = e.call(PACKAGE_GET_TYPE, &args![package]).u32();
        let value = match kind {
            0 => 0.0,
            1 => 1.0,
            2..=31 if PACKAGE_VALUE_ADDRESS[kind as usize] != 0 => {
                e.global::<f64>(PACKAGE_VALUE_ADDRESS[kind as usize])
            }
            _ => e.global::<f64>(DOUBLE_MINUS_ONE),
        };
        set_result(e, result, value);
    }
    if trace_enabled(e) {
        // "Current Process >> %0.2f"
        trace_result(e, 0x0103_5680, result);
    }
    true
}

// Translated from 005a0e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsWaitingConditionFunction` (Xbox PDB): 1.0 when the actor is
/// waiting.
pub fn script_is_waiting_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(ACTOR_IS_WAITING, &args![actor]).bool() {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "Is Waiting >> %0.2f"
        trace_result(e, 0x0103_5720, result);
    }
    true
}

// Translated from 005a0f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsIdlePlayingConditionFunction` (Xbox PDB): 1.0 when the
/// reference has an animation (virtual `+0x1e4`) whose special idle is not
/// done playing.
pub fn script_is_idle_playing_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        let animation = e
            .vcall(reference.addr(), VSLOT_REF_GET_ANIMATION, &args![])
            .ptr::<()>();
        if !animation.is_null()
            && !e
                .call(ANIMATION_SPECIAL_IDLE_DONE_PLAYING, &args![animation])
                .bool()
        {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "Is Idle Playing >> %0.2f"
        trace_result(e, 0x0103_5734, result);
    }
    true
}

// Translated from 005a0f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetCrimeConditionFunction` (Xbox PDB): 1.0 when the
/// `processlists.cpp` test answers yes for the actor (the reference) and the
/// placed reference (the form parameter); the third parameter is a crime
/// number, used when it is 0 to 4 and replaced by -1 otherwise.
pub fn script_get_crime_condition_function(
    e: &mut Engine,
    reference: Ptr,
    form: Ptr,
    crime: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let target = placed_reference_of(e, form);
    let actor = actor_of(e, reference);
    if !actor.is_null() && !target.is_null() {
        let crime_number = if (0..5).contains(&(crime as i32)) {
            crime
        } else {
            0xffff_ffff
        };
        if e.call(
            PROCESS_LISTS_CRIME_TEST,
            &args![
                PROCESS_LISTS,
                actor,
                target,
                actor,
                crime_number,
                0u32,
                0xffff_ffffu32
            ],
        )
        .bool()
        {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "GetCrime >> %0.2f"
        trace_result(e, 0x0103_5750, result);
    }
    true
}

// Translated from 005a10b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsGuardConditionFunction` (Xbox PDB): 1.0 when the reference is
/// an `NPC_` whose class says it is a guard.
pub fn script_is_guard_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        let npc = npc_base_form_of(e, reference);
        if !npc.is_null() && e.call(NPC_GET_CLASS, &args![npc]).u32() != 0 {
            let class = e.call(NPC_GET_CLASS, &args![npc]).ptr::<()>();
            if e.call(CLASS_IS_GUARD, &args![class]).bool() {
                set_result(e, result, 1.0);
            }
        }
    }
    if trace_enabled(e) {
        // "Is Guard >> %0.2f"
        trace_result(e, 0x0103_5764, result);
    }
    true
}

// Translated from 005a1150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsTalkingConditionFunction` (Xbox PDB): 1.0 when the actor is
/// talking; a reference that is not an actor is asked as a mobile object
/// (null when it has none).
pub fn script_is_talking_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if actor.is_null() {
        let mobile = e
            .call(REF_GET_MOBILE_OBJECT_EXTRA, &args![reference])
            .ptr::<()>();
        if !mobile.is_null() && e.call(MOBILE_OBJECT_IS_TALKING, &args![mobile]).bool() {
            set_result(e, result, 1.0);
        }
    } else if e.call(ACTOR_IS_TALKING, &args![actor]).bool() {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "Talking >> %0.2f"
        trace_result(e, 0x0103_5778, result);
    }
    true
}

// Translated from 005a1210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name, debug line "Procedure": a
/// number for the actor's current procedure. The procedure number comes from
/// a table indexed by the current package's group and the process's
/// procedure index (10 for a fleeing combat package), and a switch maps it to
/// a `double` in the exe's data (-1.0 for a number without a case). Stays
/// 0.0 without a process or package.
pub fn fn_005a1210(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null()
        && e.call(ACTOR_GET_PROCESS, &args![actor]).u32() != 0
        && e.call(MOBILE_GET_CURRENT_PACKAGE, &args![actor]).u32() != 0
    {
        let package = e
            .call(MOBILE_GET_CURRENT_PACKAGE, &args![actor])
            .ptr::<()>();
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).ptr::<()>();
        let group = e.call(PACKAGE_GET_PROCEDURE_GROUP, &args![package]).u32();
        let index = e
            .vcall(process.addr(), VSLOT_PROCESS_PROCEDURE_INDEX, &args![])
            .u32();
        let table = e
            .mem
            .u32(PROCEDURE_TABLES.wrapping_add(group.wrapping_mul(4)));
        let mut procedure = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        if e.call(PACKAGE_GET_TYPE, &args![package]).u32() == PACKAGE_TYPE_COMBAT
            && e.call(COMBAT_CONTROLLER_IS_FLEEING, &args![package]).bool()
        {
            procedure = PROCEDURE_FLEEING;
        }
        match procedure {
            0 => set_result(e, result, 0.0),
            2 => set_result(e, result, 1.0),
            PROCEDURE_SPECIAL => {
                let object = e
                    .vcall(process.addr(), VSLOT_PROCESS_PROCEDURE_OBJECT, &args![])
                    .ptr::<()>();
                let start = e.global::<f64>(PROCEDURE_SPECIAL_DEFAULT);
                set_result(e, result, start);
                if !object.is_null() {
                    let kind = e.mem.u32(object.addr() + 4);
                    if kind == 3 {
                        let value = e.call(PROCEDURE_OBJECT_GET_VALUE, &args![object]).u32();
                        let slot = value.wrapping_add(1);
                        if slot < 7 {
                            let entry =
                                e.global::<f32>(PROCEDURE_SPECIAL_TABLE_BY_VALUE + slot * 4);
                            set_result(e, result, entry as f64);
                        }
                    } else if (kind as i32) >= 0 && kind < 4 {
                        let entry = e.global::<f32>(PROCEDURE_SPECIAL_TABLE_BY_TYPE + kind * 4);
                        set_result(e, result, entry as f64);
                    }
                }
            }
            1..=0x36 if PROCEDURE_VALUE_ADDRESS[procedure as usize] != 0 => {
                let value = e.global::<f64>(PROCEDURE_VALUE_ADDRESS[procedure as usize]);
                set_result(e, result, value);
            }
            _ => {
                let value = e.global::<f64>(DOUBLE_MINUS_ONE);
                set_result(e, result, value);
            }
        }
    }
    if trace_enabled(e) {
        // "Procedure >> %0.2f"
        trace_result(e, 0x0103_578c, result);
    }
    true
}

// Translated from 005a1770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name, debug line "Procedure": -1.0
/// unless the actor's current package has type `0x0e` (the `+0x0c` of the
/// object at virtual `+0x274` of its process) or type `0x17` (the package's
/// own number at `+0x84`).
pub fn fn_005a1770(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let start = e.global::<f64>(DOUBLE_MINUS_ONE);
    set_result(e, result, start);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let package = e
            .call(MOBILE_GET_CURRENT_PACKAGE, &args![actor])
            .ptr::<()>();
        if !package.is_null() {
            let kind = e.call(PACKAGE_GET_TYPE, &args![package]).u32();
            if kind == PACKAGE_TYPE_WITH_PROCESS_OBJECT {
                let process = e.call(ACTOR_GET_PROCESS, &args![actor]).ptr::<()>();
                if !process.is_null() {
                    let object = e
                        .vcall(process.addr(), VSLOT_PROCESS_PROCEDURE_OBJECT, &args![])
                        .ptr::<()>();
                    if !object.is_null() {
                        let id = e.call(PROCEDURE_OBJECT_GET_ID, &args![object]).i32();
                        set_result(e, result, id as f64);
                    }
                }
            } else if kind == PACKAGE_TYPE_WITH_NUMBER {
                let number = e.call(PACKAGE_GET_NUMBER, &args![package]).i32();
                set_result(e, result, number as f64);
            }
        }
    }
    if trace_enabled(e) {
        // "Procedure >> %0.2f"
        trace_result(e, 0x0103_578c, result);
    }
    true
}

// Translated from 005a1880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetWindSpeedConditionFunction` (Xbox PDB): the wind speed of the
/// sky. Returns false (no debug line) when the `TES` global or its sky is
/// null; the result is then 0.0.
pub fn script_get_wind_speed_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let tes = e.global::<u32>(TES);
    if tes == 0 || e.call(TES_GET_SKY, &args![tes]).u32() == 0 {
        return false;
    }
    let sky = e.call(TES_GET_SKY, &args![tes]).ptr::<()>();
    let speed = e.call(SKY_GET_WIND_SPEED, &args![sky]).f64();
    set_result(e, result, speed);
    if trace_enabled(e) {
        // "GetWindSpeed >> %0.2f"
        trace_result(e, 0x0103_5848, result);
    }
    true
}

// Translated from 005a1900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsTrespassingConditionFunction` (Xbox PDB): 1.0 when the actor's
/// trespassing flag (virtual `+0x448`) is set.
pub fn script_is_trespassing_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null()
        && e.vcall(actor.addr(), VSLOT_ACTOR_IS_TRESPASSING, &args![])
            .bool()
    {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "Is Trespassing Value %0.2f"
        trace_result(e, 0x0103_5860, result);
    }
    true
}

// Translated from 005a19a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsInMyOwnedCellConditionFunction` (Xbox PDB): 1.0 when the
/// actor's parent cell is owned by the actor (the `tesobjectcell.cpp` test).
pub fn script_is_in_my_owned_cell_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(REF_GET_PARENT_CELL, &args![actor]).u32() != 0 {
        let cell = e.call(REF_GET_PARENT_CELL, &args![actor]).ptr::<()>();
        if e.call(CELL_IS_OWNED_BY, &args![cell, actor]).bool() {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "Is in owned cell value %0.2f"
        trace_result(e, 0x0103_587c, result);
    }
    true
}

// Translated from 005a1a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsContinuingPackagePCNearConditionFunction` (Xbox PDB): the byte
/// `Actor::IsContinuingPackageforPC` (Xbox PDB) returns, as the result.
pub fn script_is_continuing_package_pc_near_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let near = e
            .call(ACTOR_IS_CONTINUING_PACKAGE_FOR_PC, &args![actor])
            .u8();
        set_result(e, result, near as f64);
    }
    if trace_enabled(e) {
        // "is continuing package near PC value %0.2f"
        trace_result(e, 0x0103_589c, result);
    }
    true
}

// Translated from 005a1ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::CanHaveFlamesConditionFunction` (Xbox PDB): looks up the extra
/// data of the reference's node by the flames name, converts it and asks the
/// `modelloader.cpp` test; 1.0 or 0.0 when the node has the extra data.
pub fn script_can_have_flames_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        let name = e.call(FLAMES_EXTRA_DATA_NAME, &args![]).u32();
        let node = e
            .vcall(reference.addr(), VSLOT_REF_GET_NODE, &args![])
            .ptr::<()>();
        let extra = e.call(NODE_GET_EXTRA_DATA, &args![node, name]).ptr::<()>();
        if !extra.is_null() {
            let converted = e
                .call(EXTRA_DATA_CAST, &args![FLAMES_EXTRA_DATA_TYPE, extra])
                .ptr::<()>();
            let can = e.call(FLAMES_EXTRA_DATA_TEST, &args![converted]).bool();
            set_result(e, result, if can { 1.0 } else { 0.0 });
        }
    }
    if trace_enabled(e) {
        // "CanHaveFlames >> %0.2f"
        trace_result(e, 0x0103_58c8, result);
    }
    true
}

// Translated from 005a1b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::HasFlamesConditionFunction` (Xbox PDB): 1.0 when
/// `TESObjectREFR::HasAddonNodes` (Xbox PDB) says yes for the value of
/// virtual `+0x0c` of the reference's node, else 0.0; stays 0.0 without a
/// node.
pub fn script_has_flames_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null()
        && e.vcall(reference.addr(), VSLOT_REF_GET_NODE, &args![])
            .u32()
            != 0
    {
        let node = e
            .vcall(reference.addr(), VSLOT_REF_GET_NODE, &args![])
            .ptr::<()>();
        if !node.is_null() {
            let argument = e
                .vcall(node.addr(), VSLOT_NODE_FLAMES_ARGUMENT, &args![])
                .u32();
            let has = e.call(REF_HAS_ADDON_NODES, &args![argument]).bool();
            set_result(e, result, if has { 1.0 } else { 0.0 });
        }
    }
    if trace_enabled(e) {
        // "HasFlames >> %0.2f"
        trace_result(e, 0x0103_58e0, result);
    }
    true
}

// Translated from 005a1c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetOpenStateConditionFunction` (Xbox PDB): the open state
/// `BGSOpenCloseForm::GetOpenState` (Xbox PDB) gives for the reference.
pub fn script_get_open_state_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        let state = e.call(OPEN_CLOSE_GET_OPEN_STATE, &args![reference]).i32();
        set_result(e, result, state as f64);
    }
    if trace_enabled(e) {
        // "GetOpenState >> %0.2f"
        trace_result(e, 0x0103_58f4, result);
    }
    true
}

// Translated from 005a1c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsCurrentPackageConditionFunction` (Xbox PDB): 1.0 when the
/// package form parameter is the package the actor was set to run; a package
/// created at run time is compared by the one in the reference's extra data.
pub fn script_get_is_current_package_condition_function(
    e: &mut Engine,
    reference: Ptr,
    package: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    let wanted = form_of_type(e, package, FORM_TYPE_PACKAGE);
    if !actor.is_null() && !wanted.is_null() {
        let mut current = e
            .call(ACTOR_GET_PACKAGE_SET_AS_CURRENT, &args![actor])
            .ptr::<()>();
        if !current.is_null() && e.call(PACKAGE_GET_IS_CREATED, &args![current]).bool() {
            let list = e.call(REF_GET_EXTRA_DATA_LIST, &args![actor]).ptr::<()>();
            if e.call(EXTRA_DATA_LIST_GET_PACKAGE_EXTRA, &args![list])
                .u32()
                != 0
            {
                let list = e.call(REF_GET_EXTRA_DATA_LIST, &args![actor]).ptr::<()>();
                current = e
                    .call(EXTRA_DATA_LIST_GET_PACKAGE_EXTRA, &args![list])
                    .ptr::<()>();
            }
        }
        if current == wanted {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "GetIsCurrentPackage >> %0.2f"
        trace_result(e, 0x0103_590c, result);
    }
    true
}

// Translated from 005a1d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDayOfWeekConditionFunction` (Xbox PDB): the calendar's day of
/// the week.
pub fn script_get_day_of_week_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let day = e.call(CALENDAR_GET_DAY_OF_WEEK, &args![CALENDAR]).i32();
    set_result(e, result, day as f64);
    if trace_enabled(e) {
        // "GetDayOfWeek >> %0.2f"
        trace_result(e, 0x0103_592c, result);
    }
    true
}

// Translated from 005a1de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPCSleepingConditionFunction` (Xbox PDB): 1.0 when the byte at
/// `+0x658` of the player is set.
pub fn script_is_pc_sleeping_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let player = Ptr::new(e.global::<u32>(PLAYER));
    let sleeping = fn_005a1e50(e, player) != 0;
    set_result(e, result, if sleeping { 1.0 } else { 0.0 });
    if trace_enabled(e) {
        // "IsPlayerSleeping >> %0.2f"
        trace_result(e, 0x0103_5944, result);
    }
    true
}

// Translated from 005a1e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x658` of the player (`this`), which `IsPCSleeping` tests.
pub fn fn_005a1e50(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x658)
}

// Translated from 005a1e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPCAMurdererConditionFunction` (Xbox PDB): 1.0 when the
/// player's murderer test (`playercharacter.cpp`) says yes.
pub fn script_is_pca_murderer_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let player = e.global::<u32>(PLAYER);
    let murderer = e.call(PLAYER_IS_A_MURDERER, &args![player]).bool();
    set_result(e, result, if murderer { 1.0 } else { 0.0 });
    if trace_enabled(e) {
        // "IsPlayerAMurderer >> %0.2f"
        trace_result(e, 0x0103_5960, result);
    }
    true
}

// Translated from 005a1ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDetectionLevelConditionFunction` (Xbox PDB): the level at
/// which the actor (the reference) detects the target (the parameter). Does
/// nothing for a null reference, and its debug line only appears when the
/// level was computed.
pub fn script_get_detection_level_condition_function(
    e: &mut Engine,
    reference: Ptr,
    target: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        set_result(e, result, 0.0);
        let actor = actor_of(e, reference);
        if !actor.is_null() && !target.is_null() {
            let level = e
                .call(ACTOR_GET_DETECTION_LEVEL, &args![actor, target])
                .i32();
            set_result(e, result, level as f64);
            if trace_enabled(e) {
                let value = e.mem.f64(result.addr());
                let target_name = e.call(REF_GET_NAME, &args![target]).u32();
                let actor_name = e.call(REF_GET_NAME, &args![actor]).u32();
                // "%s detects %s at level %.02f"
                e.call(
                    DEBUG_PRINT,
                    &args![0x0103_597cu32, actor_name, target_name, value],
                );
            }
        }
    }
    true
}

// Translated from 005a1fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsSwimmingConditionFunction` (Xbox PDB): 1.0 when the actor's
/// swimming byte (`+0x14d`) is set.
pub fn script_is_swimming_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && fn_005a2030(e, actor) != 0 {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "Is Swimming >> %0.2f"
        trace_result(e, 0x0103_599c, result);
    }
    true
}

// Translated from 005a2030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x14d` of the actor (`this`), which `IsSwimming` tests.
pub fn fn_005a2030(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x14d)
}

// Translated from 005a2050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetAmountStolenSoldConditionFunction` (Xbox PDB): the `int` at
/// `+0x6dc` of the player.
pub fn script_get_amount_stolen_sold_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let player = Ptr::new(e.global::<u32>(PLAYER));
    let amount = fn_005a20b0(e, player) as i32;
    set_result(e, result, amount as f64);
    if trace_enabled(e) {
        // "Amount stolen the player sold >> %0.2f"
        trace_result(e, 0x0103_59b4, result);
    }
    true
}

// Translated from 005a20b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `int` at `+0x6dc` of the player (`this`), which `GetAmountStolenSold`
/// reads.
pub fn fn_005a20b0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x6dc)
}

// Translated from 005a20d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPCExpelledConditionFunction` (Xbox PDB): 1.0 when the faction
/// (the form parameter) has the player expelled; the debug line is one of
/// two fixed sentences instead of the value.
pub fn script_get_pc_expelled_condition_function(
    e: &mut Engine,
    _reference: u32,
    faction: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !faction.is_null() && e.call(FACTION_TEST_EXPELLED, &args![faction]).bool() {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        if e.mem.f64(result.addr()) == 0.0 {
            // "PC is not expelled."
            e.call(DEBUG_PRINT, &args![0x0103_59dcu32]);
        } else {
            // "PC is expelled."
            e.call(DEBUG_PRINT, &args![0x0103_59f0u32]);
        }
    }
    true
}

// Translated from 005a2150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPCFactionMurderConditionFunction` (Xbox PDB): 1.0 when the
/// faction (the form parameter) has the "player murdered a member" flag
/// (`0x40` of `+0x34`); the debug line is one of two fixed sentences.
pub fn script_get_pc_faction_murder_condition_function(
    e: &mut Engine,
    _reference: u32,
    faction: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !faction.is_null() && fn_005a21d0(e, faction) {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        if e.mem.f64(result.addr()) == 0.0 {
            // "PC did not Murder a faction member."
            e.call(DEBUG_PRINT, &args![0x0103_5a00u32]);
        } else {
            // "PC murdered a faction member."
            e.call(DEBUG_PRINT, &args![0x0103_5a24u32]);
        }
    }
    true
}

// Translated from 005a21d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit `0x40` of the faction's flags (`this + 0x34`) is set.
pub fn fn_005a21d0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x34) & 0x40 != 0
}

// Translated from 005a21f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPlayerEnemyofFactionConditionFunction` (Xbox PDB): 1.0 when
/// the faction (the form parameter) has the "player is an enemy" flag (`0x10`
/// of `+0x34`); the debug line is one of two fixed sentences.
pub fn script_get_player_enemyof_faction_condition_function(
    e: &mut Engine,
    _reference: u32,
    faction: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !faction.is_null() && fn_005a2270(e, faction) {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        if e.mem.f64(result.addr()) == 0.0 {
            // "PC is not an enemy of this faction."
            e.call(DEBUG_PRINT, &args![0x0103_5a44u32]);
        } else {
            // "PC Is an Enemy of the faction."
            e.call(DEBUG_PRINT, &args![0x0103_5a68u32]);
        }
    }
    true
}

// Translated from 005a2270 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit `0x10` of the faction's flags (`this + 0x34`) is set.
pub fn fn_005a2270(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x34) & 0x10 != 0
}

// Translated from 005a2290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPCFactionAttackConditionFunction` (Xbox PDB): 1.0 when the
/// faction (the form parameter) has the "player attacked a member" flag
/// (`0x20` of `+0x34`); the debug line is one of two fixed sentences.
pub fn script_get_pc_faction_attack_condition_function(
    e: &mut Engine,
    _reference: u32,
    faction: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !faction.is_null() && fn_005a2310(e, faction) {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        if e.mem.f64(result.addr()) == 0.0 {
            // "PC did not attack a faction member."
            e.call(DEBUG_PRINT, &args![0x0103_5a88u32]);
        } else {
            // "PC attacked a faction member."
            e.call(DEBUG_PRINT, &args![0x0103_5aacu32]);
        }
    }
    true
}

// Translated from 005a2310 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit `0x20` of the faction's flags (`this + 0x34`) is set.
pub fn fn_005a2310(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x34) & 0x20 != 0
}

// Translated from 005a2330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDestroyedConditionFunction` (Xbox PDB): 1.0 when the
/// reference's form flags have bit `0x00800000` set.
pub fn script_get_destroyed_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() && e.call(REF_TEST_DESTROYED, &args![reference]).bool() {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetDestroyed >> %.0f"
        trace_result(e, 0x0103_5acc, result);
    }
    true
}

// Translated from 005a2390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::HasMagicEffectConditionFunction` (Xbox PDB): 1.0 when the
/// actor's magic target has the effect (the form parameter).
pub fn script_has_magic_effect_condition_function(
    e: &mut Engine,
    reference: Ptr,
    effect: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && !effect.is_null() {
        let magic_target = e
            .vcall(actor.addr(), VSLOT_ACTOR_GET_MAGIC_TARGET, &args![])
            .ptr::<()>();
        if e.call(MAGIC_TARGET_HAS_EFFECT, &args![magic_target, effect])
            .bool()
        {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "Has Magic Effect >> %0.2f"
        trace_result(e, 0x0103_5ae4, result);
    }
    true
}

// Translated from 005a2440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsSpellTargetConditionFunction` (Xbox PDB): 1.0 when
/// `MagicTarget::IsSpellTarget` (Xbox PDB) says the actor's magic target is
/// a target of the spell (the form parameter).
pub fn script_is_spell_target_condition_function(
    e: &mut Engine,
    reference: Ptr,
    spell: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && !spell.is_null() {
        let magic_target = e
            .vcall(actor.addr(), VSLOT_ACTOR_GET_MAGIC_TARGET, &args![])
            .ptr::<()>();
        if e.call(
            MAGIC_TARGET_IS_SPELL_TARGET,
            &args![magic_target, spell, 1u32],
        )
        .bool()
        {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "Is Spell Target >> %0.2f"
        trace_result(e, 0x0103_5b00, result);
    }
    true
}

// Translated from 005a24f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetSpellUsageNumberConditionFunction` (Xbox PDB): the usage
/// monitor magnitude of the effect (the form parameter) in the magic target
/// at `actor + 0x94`.
pub fn script_get_spell_usage_number_condition_function(
    e: &mut Engine,
    reference: Ptr,
    effect: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && !effect.is_null() {
        let magnitude = e
            .call(
                MAGIC_TARGET_GET_USAGE_MONITOR_MAGNITUDE,
                &args![actor.addr() + ACTOR_MAGIC_TARGET_OFFSET, effect],
            )
            .f64();
        set_result(e, result, magnitude);
    }
    if trace_enabled(e) {
        // "GetSpellUsageNum >> %0.2f"
        trace_result(e, 0x0103_5b1c, result);
    }
    true
}

// Translated from 005a2590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSModeConditionFunction` (Xbox PDB): the VATS object's mode
/// (`+0x08`).
pub fn script_get_vats_mode_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let mode = e.call(VATS_GET_MODE, &args![VATS]).i32();
    set_result(e, result, mode as f64);
    if trace_enabled(e) {
        // "VATS Mode >> %0.2f"
        trace_result(e, 0x0103_5b38, result);
    }
    true
}

// Translated from 005a25f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSTargetHeightConditionFunction` (Xbox PDB): while VATS is
/// in mode 4 with an attack action, the height of the action's target above
/// the player: the Z of the targeted body part's node when the target is an
/// actor and the part exists, else the Z of the target's node bound center,
/// else the Z of its position, minus the Z of the player's position.
pub fn script_get_vats_target_height_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if e.call(VATS_GET_MODE, &args![VATS]).u32() == VATS_MODE_TARGETING {
        let action = e.call(VATS_GET_CURRENT_ACTION, &args![VATS]).ptr::<()>();
        if !action.is_null() {
            let action_type = e.mem.u32(action.addr());
            if e.call(ACTION_IS_ATTACK, &args![action_type]).bool() {
                let target = Ptr::<()>::new(e.mem.u32(action.addr() + 0xc));
                let body_part = e.mem.u32(action.addr() + 0x10);
                if !target.is_null() {
                    let node = e
                        .vcall(target.addr(), VSLOT_REF_GET_NODE, &args![])
                        .ptr::<()>();
                    e.with_stack(12, |e, vector| {
                        e.call(RETURN_THIS, &args![vector]);
                    });
                    let mut target_z = None;
                    if body_part != 0xffff_ffff
                        && e.vcall(target.addr(), VSLOT_IS_ACTOR, &args![]).bool()
                    {
                        let base = e.call(ACTOR_GET_BASE_FORM, &args![target]).ptr::<()>();
                        let body_part_data = e
                            .vcall(base.addr(), VSLOT_BASE_FORM_BODY_PART_DATA, &args![])
                            .ptr::<()>();
                        if !body_part_data.is_null() {
                            let part = e
                                .call(
                                    BODY_PART_DATA_GET_BODY_PART,
                                    &args![body_part_data, body_part],
                                )
                                .ptr::<()>();
                            if !part.is_null() {
                                let name = e.call(BODY_PART_GET_NODE_NAME, &args![part]).u32();
                                let target_node = e
                                    .vcall(target.addr(), VSLOT_REF_GET_NODE, &args![])
                                    .ptr::<()>();
                                let part_node = e
                                    .call(NODE_FIND_BY_NAME, &args![target_node, name])
                                    .ptr::<()>();
                                if !part_node.is_null() {
                                    let position = e
                                        .call(NODE_GET_WORLD_POSITION, &args![part_node])
                                        .ptr::<()>();
                                    target_z = Some(e.mem.f32(position.addr() + 8));
                                }
                            }
                        }
                    }
                    let target_z = match target_z {
                        Some(z) => z,
                        None if !node.is_null() => {
                            let bound = e.call(NODE_GET_WORLD_BOUND, &args![node]).ptr::<()>();
                            let center = e.call(RETURN_THIS, &args![bound]).ptr::<()>();
                            e.mem.f32(center.addr() + 8)
                        }
                        None => {
                            let position = e
                                .vcall(target.addr(), VSLOT_GET_POSITION, &args![])
                                .ptr::<()>();
                            e.mem.f32(position.addr() + 8)
                        }
                    };
                    let player = e.global::<u32>(PLAYER);
                    let player_position = e.vcall(player, VSLOT_GET_POSITION, &args![]).ptr::<()>();
                    let player_z = e.mem.f32(player_position.addr() + 8);
                    set_result(e, result, target_z as f64 - player_z as f64);
                }
            }
        }
    }
    if trace_enabled(e) {
        // "VATS Target Height >> %0.2f"
        trace_result(e, 0x0103_5b4c, result);
    }
    true
}

// Translated from 005a27e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name and debug line: returns false
/// (leaving the result alone) for a null reference; otherwise 1.0 when the
/// reference's extra data list test with 8 says yes, else 0.0.
pub fn fn_005a27e0(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    if reference.is_null() {
        return false;
    }
    set_result(e, result, 0.0);
    if e.call(REF_EXTRA_DATA_LIST_TEST, &args![reference, 8u32])
        .bool()
    {
        set_result(e, result, 1.0);
    }
    true
}

// Translated from 005a2810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetSandmanConditionFunction` (Xbox PDB): for the player only,
/// the byte of the player's virtual `+0x36c`; 0.0 for any other reference.
pub fn script_get_sandman_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let player = e.global::<u32>(PLAYER);
    if reference.addr() == player {
        let sandman = e.vcall(player, VSLOT_PLAYER_SANDMAN, &args![]).u8();
        set_result(e, result, sandman as f64);
    }
    if trace_enabled(e) {
        // "Player is in Sandman action >> %0.2f"
        trace_result(e, 0x0103_5b68, result);
    }
    true
}

// Translated from 005a2890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetCannibalConditionFunction` (Xbox PDB): for the player only,
/// the byte of the player's virtual `+0x364`; 0.0 for any other reference.
pub fn script_get_cannibal_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let player = e.global::<u32>(PLAYER);
    if reference.addr() == player {
        let cannibal = e.vcall(player, VSLOT_PLAYER_CANNIBAL, &args![]).u8();
        set_result(e, result, cannibal as f64);
    }
    if trace_enabled(e) {
        // "Player is in Cannibal action >> %0.2f"
        trace_result(e, 0x0103_5b90, result);
    }
    true
}

// Translated from 005a2910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsInCritStageConditionFunction` (Xbox PDB): 1.0 when the actor's
/// crit stage number equals the parameter. The reference is not checked for
/// null (the virtual `IsActor` is called on it directly). The debug line says
/// "IS" or "IS NOT" and names the stage from a table.
pub fn script_is_in_crit_stage_condition_function(
    e: &mut Engine,
    reference: Ptr,
    stage: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if e.vcall(reference.addr(), VSLOT_IS_ACTOR, &args![]).bool()
        && e.call(ACTOR_GET_CRIT_STAGE, &args![reference]).u32() == stage
    {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        let verdict: u32 = if e.mem.f64(result.addr()) == 1.0 {
            // "IS"
            0x0103_5bd8
        } else {
            // "IS NOT"
            0x0103_5bd0
        };
        let name = e
            .mem
            .u32(CRIT_STAGE_NAMES.wrapping_add(stage.wrapping_mul(4)));
        // "Actor %s in stage %s."
        e.call(DEBUG_PRINT, &args![0x0103_5bb8u32, verdict, name]);
    }
    true
}

// Translated from 005a29d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::HasBeenEatenConditionFunction` (Xbox PDB): 1.0 when the
/// `actor.cpp` test says the actor has been eaten. The reference is not
/// checked for null (the virtual `IsActor` is called on it directly).
pub fn script_has_been_eaten_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if e.vcall(reference.addr(), VSLOT_IS_ACTOR, &args![]).bool()
        && e.call(ACTOR_HAS_BEEN_EATEN, &args![reference]).bool()
    {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "Actor has been eaten >> %0.2f"
        trace_result(e, 0x0103_5bdc, result);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059bfa0, script_get_distance_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059c010, script_get_in_zone_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059c0c0, script_get_pos_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c170, script_get_angle_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c230, script_get_starting_pos_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c2d0, script_get_starting_angle_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c380, script_get_menu_mode_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c430, script_get_seconds_passed_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059c4c0, fn_0059c4c0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c4f0, fn_0059c4f0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c5d0, script_get_base_actor_value_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c680, fn_0059c680(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c760, script_get_fatigue_percentage_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c7e0, script_get_health_percentage_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c860, script_get_walk_speed_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c8e0, script_get_current_time_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c930, script_get_scale_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059c990, script_get_los_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059ce80, fn_0059ce80(Ptr, u32)),
        entry!(0x0059ceb0, fn_0059ceb0(Ptr, Ptr)),
        entry!(0x0059cee0, fn_0059cee0(Ptr)),
        entry!(0x0059cf00, fn_0059cf00(Ptr)),
        entry!(0x0059cf20, fn_0059cf20(Ptr, u32) -> Ptr),
        entry!(0x0059cf50, script_get_disabled_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059d010, script_get_locked_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059d100, script_get_lock_level_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059d1d0, script_get_is_lock_broken_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059d250, script_get_disease_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059d2a0, script_get_vampire_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059d320, script_get_clothing_value_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059d3a0, fn_0059d3a0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059d3c0, fn_0059d3c0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059d3e0, fn_0059d3e0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059d400, script_same_faction_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059d540, script_same_race_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059d610, script_same_sex_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059d6e0, script_get_detected_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059d840, script_get_dead_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059d8e0, script_get_item_count_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059da90, script_get_equipped_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059dbe0, script_get_gold_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059dc90, script_get_sleeping_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059dd90, script_get_sitting_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059de90, script_get_furniture_marker_id_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059df40, script_is_current_furniture_ref_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059dfe0, script_is_current_furniture_obj_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059e0f0, script_get_talked_to_pc_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059e1a0, fn_0059e1a0(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059e300, fn_0059e300(Ptr) -> u32),
        entry!(0x0059e320, script_get_quest_running_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059e390, script_get_quest_completed_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059e400, fn_0059e400(Ptr) -> bool),
        entry!(0x0059e420, script_get_stage_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059e490, script_get_stage_done_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059e510, script_get_faction_rank_difference_condition_function(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x0059e650, script_get_alarmed_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059e700, script_get_is_pleasant_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059e7d0, fn_0059e7d0(Ptr) -> bool),
        entry!(0x0059e7f0, script_get_is_cloudy_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059e8c0, fn_0059e8c0(Ptr) -> bool),
        entry!(0x0059e8e0, script_get_is_raining_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059e950, fn_0059e950(Ptr) -> bool),
        entry!(0x0059ea10, script_get_is_snowing_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059ea80, fn_0059ea80(Ptr) -> bool),
        entry!(0x0059eb40, fn_0059eb40(Ptr) -> bool),
        entry!(0x0059eb60, script_get_weather_percent_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059ebb0, script_get_is_current_weather_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059ec30, script_get_attacked_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059ecc0, script_get_is_creature_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059ed30, script_get_should_attack_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059ee80, script_get_in_same_cell_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059ef60, script_get_in_cell_condition_function(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x0059f0c0, script_get_in_worldspace_condition_function(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x0059f180, script_get_is_class_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059f240, script_get_is_race_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059f300, script_get_is_creature_type_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059f3a0, fn_0059f3a0(Ptr) -> u8),
        entry!(0x0059f3c0, script_get_is_sex_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059f450, script_get_is_voice_type_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059f540, script_get_is_playable_race_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059f610, fn_0059f610(Ptr) -> bool),
        entry!(0x0059f630, script_get_in_faction_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059f730, fn_0059f730(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x0059f750, fn_0059f750(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x0059f770, fn_0059f770(u32, u32, u32, Ptr) -> bool),
        entry!(0x0059f790, fn_0059f790(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x0059f7b0, script_get_is_id_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059f890, script_is_in_list_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059f940, script_get_is_child_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059f9b0, script_get_is_used_item_condition_function(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x0059fa90, script_get_is_used_item_type_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x0059fb00, script_get_used_item_level_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x0059fb50, script_get_used_item_activate_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x0059fbb0, script_get_is_ref_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059fc50, script_get_faction_rank_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059fd30, script_get_global_value_condition_function(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x0059fdb0, script_get_disposition_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x0059fee0, script_get_unconscious_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0059ff90, script_get_restrained_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a0040, script_get_random_percent_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a00a0, fn_005a00a0(u32) -> u32),
        entry!(0x005a00c0, script_get_level_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a0150, script_get_armor_rating_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a01f0, script_get_dead_count_condition_function(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x005a0260, script_get_alert_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a02f0, script_get_player_controls_disabled_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a03d0, fn_005a03d0(Ptr) -> u8),
        entry!(0x005a03f0, fn_005a03f0(Ptr, u32) -> bool),
        entry!(0x005a0410, script_get_heading_angle_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x005a0550, script_is_weapon_out_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a05e0, script_is_weapon_in_list_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x005a0710, script_is_facing_up_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a07f0, fn_005a07f0() -> u32),
        entry!(0x005a0800, script_is_left_up_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a08c0, script_get_knocked_state_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a09b0, script_get_weapon_anim_type_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a0ab0, script_is_weapon_skill_type_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a0b60, script_get_current_ai_package_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a0e80, script_is_waiting_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a0f10, script_is_idle_playing_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a0f90, script_get_crime_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x005a10b0, script_is_guard_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a1150, script_is_talking_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a1210, fn_005a1210(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a1770, fn_005a1770(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a1880, script_get_wind_speed_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a1900, script_is_trespassing_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a19a0, script_is_in_my_owned_cell_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a1a40, script_is_continuing_package_pc_near_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a1ad0, script_can_have_flames_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a1b80, script_has_flames_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a1c30, script_get_open_state_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a1c90, script_get_is_current_package_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x005a1d80, script_get_day_of_week_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a1de0, script_is_pc_sleeping_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a1e50, fn_005a1e50(Ptr) -> u8),
        entry!(0x005a1e70, script_is_pca_murderer_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a1ee0, script_get_detection_level_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x005a1fa0, script_is_swimming_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a2030, fn_005a2030(Ptr) -> u8),
        entry!(0x005a2050, script_get_amount_stolen_sold_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a20b0, fn_005a20b0(Ptr) -> u32),
        entry!(0x005a20d0, script_get_pc_expelled_condition_function(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x005a2150, script_get_pc_faction_murder_condition_function(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x005a21d0, fn_005a21d0(Ptr) -> bool),
        entry!(0x005a21f0, script_get_player_enemyof_faction_condition_function(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x005a2270, fn_005a2270(Ptr) -> bool),
        entry!(0x005a2290, script_get_pc_faction_attack_condition_function(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x005a2310, fn_005a2310(Ptr) -> bool),
        entry!(0x005a2330, script_get_destroyed_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a2390, script_has_magic_effect_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x005a2440, script_is_spell_target_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x005a24f0, script_get_spell_usage_number_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x005a2590, script_get_vats_mode_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a25f0, script_get_vats_target_height_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a27e0, fn_005a27e0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a2810, script_get_sandman_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a2890, script_get_cannibal_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a2910, script_is_in_crit_stage_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a29d0, script_has_been_eaten_condition_function(Ptr, u32, u32, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What `*result` holds before a call; a function that leaves the result
    /// alone leaves this.
    const SENTINEL: f64 = -7.5;
    /// Virtual `IsActor` answering yes / no.
    const IS_ACTOR_YES: u32 = 0x0f00_0001;
    const IS_ACTOR_NO: u32 = 0x0f00_0002;

    /// An engine with the constants the functions read from the exe's data,
    /// the debug print and the two `IsActor` answers.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_a000,
            0x0101_d000,
            0x0102_9000,
            0x0102_f000,
            0x0102_3000,
            0x0101_7000,
            0x011c_a000,
            0x011d_e000,
            0x011f_1000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(0x0101_1588, 0.5f64);
        e.set_global(0x0101_1590, 2.0f64);
        e.set_global(0x0101_2060, 0.0f64);
        e.set_global(0x0101_a6b0, -1.0f64);
        e.set_global(0x0101_de30, 0.75f64);
        e.set_global(0x0102_90b0, 0.25f64);
        e.set_global(DEGREES_PER_RADIAN, 57.29577951308232f64);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.set_global(WEATHER_RAIN_FROM, 0.5f32);
        e.set_global(WEATHER_RAIN_TO, 0.125f32);
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        e.register(IS_ACTOR_YES, |_, _| true.into_ret());
        e.register(IS_ACTOR_NO, |_, _| false.into_ret());
        // BSSimpleList node helpers.
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(FORM_LIST_GET_LIST, |_, a| (a[0] + 0x18).into_ret());
        e
    }

    /// Calls the condition function at `addr` and returns what it returned
    /// and what it left in `*result`.
    fn run(e: &mut Engine, addr: u32, reference: u32, param1: u32, param2: u32) -> (bool, f64) {
        let result = e.mem.alloc(8);
        e.mem.set_f64(result, SENTINEL);
        let returned = e
            .call(addr, &args![reference, param1, param2, result])
            .bool();
        (returned, e.mem.f64(result))
    }

    /// A function that returns `eax`.
    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    /// A function that returns `value` in ST0.
    fn stub_st0(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    /// An object whose vtable has the given (offset, function) slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x800);
        for &(offset, function) in slots {
            e.mem.set_u32(vtable + offset, function);
        }
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        object
    }

    fn actor(e: &mut Engine) -> u32 {
        object(e, &[(VSLOT_IS_ACTOR, IS_ACTOR_YES)])
    }

    fn non_actor(e: &mut Engine) -> u32 {
        object(e, &[(VSLOT_IS_ACTOR, IS_ACTOR_NO)])
    }

    /// Three floats in memory.
    fn floats(e: &mut Engine, values: [f32; 3]) -> u32 {
        let block = e.mem.alloc(12);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        block
    }

    /// Turns the debug lines on and starts recording calls.
    fn trace_on(e: &mut Engine) {
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_TRACE_FLAG, 1);
        e.call_log = Some(vec![]);
    }

    type Log = Vec<(u32, Vec<u32>)>;

    fn take_log(e: &mut Engine) -> Log {
        e.call_log.take().unwrap_or_default()
    }

    /// The argument words of every call to `addr`.
    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn words(value: f64) -> [u32; 2] {
        let bits = value.to_bits();
        [bits as u32, (bits >> 32) as u32]
    }

    #[test]
    fn distance_comes_from_the_reference_distance() {
        let mut e = engine();
        stub_st0(&mut e, REF_GET_DISTANCE_FROM_REFERENCE, 12.5);
        trace_on(&mut e);
        let (returned, value) = run(&mut e, 0x0059_bfa0, 0x2000, 0x3000, 0);
        assert!(returned);
        assert_eq!(value, 12.5);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, REF_GET_DISTANCE_FROM_REFERENCE),
            vec![vec![0x2000, 0x3000, 1, 0]]
        );
        let w = words(12.5);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4d0c, w[0], w[1]]]
        );
    }

    #[test]
    fn distance_leaves_the_result_when_a_reference_is_null() {
        let mut e = engine();
        assert_eq!(run(&mut e, 0x0059_bfa0, 0x2000, 0, 0), (true, SENTINEL));
        assert_eq!(run(&mut e, 0x0059_bfa0, 0, 0x2000, 0), (true, SENTINEL));
    }

    #[test]
    fn in_zone_matches_the_parent_cell_then_the_worldspace_cell() {
        let mut e = engine();
        // The reference has a parent (00 8d6f30) whose cell (00546c20) is 0x500.
        stub(&mut e, 0x008d_6f30, 0x9000);
        stub(&mut e, 0x0054_6c20, 0x500);
        assert_eq!(run(&mut e, 0x0059_c010, 0x2000, 0x500, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_c010, 0x2000, 0x600, 0), (true, 0.0));

        // No parent cell: the worldspace's cell (0045 8400) is used.
        stub(&mut e, 0x008d_6f30, 0);
        stub(&mut e, 0x0057_5d70, 0x9100);
        stub(&mut e, 0x0045_8400, 0x700);
        assert_eq!(run(&mut e, 0x0059_c010, 0x2000, 0x700, 0), (true, 1.0));
        // Neither: the cell is null and never equals a non-null zone.
        stub(&mut e, 0x0057_5d70, 0);
        assert_eq!(run(&mut e, 0x0059_c010, 0x2000, 0x700, 0), (true, 0.0));
        // A null zone: nothing is computed, but the result is 0.
        assert_eq!(run(&mut e, 0x0059_c010, 0x2000, 0, 0), (true, 0.0));
    }

    #[test]
    fn pos_picks_the_axis() {
        let mut e = engine();
        let position = floats(&mut e, [1.5, 2.5, 3.5]);
        let reference = object(&mut e, &[(VSLOT_GET_POSITION, 0x0f00_0010)]);
        stub(&mut e, 0x0f00_0010, position);
        assert_eq!(run(&mut e, 0x0059_c0c0, reference, 0x58, 0), (true, 1.5));
        assert_eq!(run(&mut e, 0x0059_c0c0, reference, 0x59, 0), (true, 2.5));
        assert_eq!(run(&mut e, 0x0059_c0c0, reference, 0x5a, 0), (true, 3.5));
        // Any other letter leaves the result alone.
        assert_eq!(
            run(&mut e, 0x0059_c0c0, reference, 0x41, 0),
            (true, SENTINEL)
        );
        // A null reference does nothing at all.
        assert_eq!(run(&mut e, 0x0059_c0c0, 0, 0x58, 0), (true, SENTINEL));

        trace_on(&mut e);
        run(&mut e, 0x0059_c0c0, reference, 0x59, 0);
        let log = take_log(&mut e);
        let w = words(2.5);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4d38, 0x59, w[0], w[1]]]
        );
    }

    #[test]
    fn angle_is_in_degrees() {
        let mut e = engine();
        let angles = floats(&mut e, [1.0, 0.5, 0.0]);
        stub(&mut e, 0x0043_0830, angles);
        let (_, x) = run(&mut e, 0x0059_c170, 0x2000, 0x58, 0);
        assert_eq!(x, 1.0f32 as f64 * 57.29577951308232);
        let (_, y) = run(&mut e, 0x0059_c170, 0x2000, 0x59, 0);
        assert_eq!(y, 0.5f32 as f64 * 57.29577951308232);
        assert_eq!(run(&mut e, 0x0059_c170, 0x2000, 0x5a, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_c170, 0x2000, 0x41, 0), (true, SENTINEL));
        assert_eq!(run(&mut e, 0x0059_c170, 0, 0x58, 0), (true, SENTINEL));
    }

    #[test]
    fn starting_pos_reads_the_out_buffer_the_virtual_fills() {
        let mut e = engine();
        e.register(0x0f00_0020, |e, a| {
            for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        let reference = object(&mut e, &[(0x170, 0x0f00_0020)]);
        assert_eq!(run(&mut e, 0x0059_c230, reference, 0x58, 0), (true, 10.0));
        assert_eq!(run(&mut e, 0x0059_c230, reference, 0x59, 0), (true, 20.0));
        assert_eq!(run(&mut e, 0x0059_c230, reference, 0x5a, 0), (true, 30.0));
        assert_eq!(
            run(&mut e, 0x0059_c230, reference, 0x41, 0),
            (true, SENTINEL)
        );
        assert_eq!(run(&mut e, 0x0059_c230, 0, 0x58, 0), (true, SENTINEL));
    }

    #[test]
    fn starting_angle_is_in_degrees() {
        let mut e = engine();
        e.register(0x0f00_0021, |e, a| {
            for (i, v) in [0.0f32, 2.0, 0.25].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        let reference = object(&mut e, &[(0x16c, 0x0f00_0021)]);
        assert_eq!(run(&mut e, 0x0059_c2d0, reference, 0x58, 0), (true, 0.0));
        let (_, y) = run(&mut e, 0x0059_c2d0, reference, 0x59, 0);
        assert_eq!(y, 2.0f32 as f64 * 57.29577951308232);
        let (_, z) = run(&mut e, 0x0059_c2d0, reference, 0x5a, 0);
        assert_eq!(z, 0.25f32 as f64 * 57.29577951308232);
        assert_eq!(run(&mut e, 0x0059_c2d0, 0, 0x58, 0), (true, SENTINEL));
    }

    #[test]
    fn seconds_passed_prefers_the_source_when_flagged_and_positive() {
        let mut e = engine();
        // The source's record (source + 0x18) with the flag byte at +0x10.
        let source = e.mem.alloc(0x40);
        e.mem.set_u8(source + 0x18 + 0x10, 1);
        stub_st0(&mut e, REF_GET_OWN_SCALE, 0.25);
        stub_st0(&mut e, 0x0084_d030, 9.0);
        assert_eq!(run(&mut e, 0x0059_c430, 0, source, 0), (true, 0.25));

        // Not positive: the global data's value.
        stub_st0(&mut e, REF_GET_OWN_SCALE, 0.0);
        assert_eq!(run(&mut e, 0x0059_c430, 0, source, 0), (true, 9.0));
        // Flag clear: also the global data's value.
        stub_st0(&mut e, REF_GET_OWN_SCALE, 3.0);
        e.mem.set_u8(source + 0x18 + 0x10, 0);
        assert_eq!(run(&mut e, 0x0059_c430, 0, source, 0), (true, 9.0));
        // No source.
        assert_eq!(run(&mut e, 0x0059_c430, 0, 0, 0), (true, 9.0));

        trace_on(&mut e);
        run(&mut e, 0x0059_c430, 0, 0, 0);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x0084_d030),
            vec![vec![SECONDS_PASSED_SOURCE]]
        );
        let w = words(9.0);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4db8, w[0], w[1]]]
        );
    }

    #[test]
    fn unnamed_0059c4c0_is_one_when_the_virtual_answers() {
        let mut e = engine();
        stub(&mut e, 0x0f00_0030, 0x1234);
        stub(&mut e, 0x0f00_0031, 0);
        let yes = object(&mut e, &[(0x1d0, 0x0f00_0030)]);
        let no = object(&mut e, &[(0x1d0, 0x0f00_0031)]);
        assert_eq!(run(&mut e, 0x0059_c4c0, yes, 0, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_c4c0, no, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_c4c0, 0, 0, 0), (true, 0.0));
    }

    /// An actor whose embedded `ActorValueOwner` (at +0xa4) has `slots`, and
    /// whose base form (returned by `0041 81e0`) has another one at +0x100.
    fn actor_with_owners(
        e: &mut Engine,
        actor_slots: &[(u32, u32)],
        base_slots: &[(u32, u32)],
    ) -> u32 {
        let actor = actor(e);
        let owner_table = e.mem.alloc(0x100);
        for &(offset, function) in actor_slots {
            e.mem.set_u32(owner_table + offset, function);
        }
        e.mem.set_u32(actor + 0xa4, owner_table);
        let base = e.mem.alloc(0x200);
        let base_table = e.mem.alloc(0x100);
        for &(offset, function) in base_slots {
            e.mem.set_u32(base_table + offset, function);
        }
        e.mem.set_u32(base + 0x100, base_table);
        e.register_double(ACTOR_GET_BASE_FORM, move |_, _| Ret {
            eax: base,
            ..Ret::default()
        });
        actor
    }

    #[test]
    fn actor_value_reads_the_actors_owner_or_the_base_forms() {
        let mut e = engine();
        // Owner virtual +0xc returns 42.5 (actor) / 7.5 (base form); the
        // argument is the actor value index.
        e.register(0x0f00_0040, |e, a| {
            // `this` is the embedded owner (actor + 0xa4), the argument the
            // actor value index.
            assert_eq!(a[1], 8);
            assert!(e.mem.u32(a[0]) != 0);
            Ret {
                st0: 42.5,
                ..Ret::default()
            }
        });
        e.register(0x0f00_0041, |_, _| Ret {
            st0: 7.5,
            ..Ret::default()
        });
        let actor = actor_with_owners(&mut e, &[(0xc, 0x0f00_0040)], &[(0xc, 0x0f00_0041)]);
        stub(&mut e, FORM_HAS_FLAG_0800, 0);
        assert_eq!(run(&mut e, 0x0059_c4f0, actor, 8, 0), (true, 42.5));
        // With form flag 0x800 the base form's owner answers.
        stub(&mut e, FORM_HAS_FLAG_0800, 1);
        assert_eq!(run(&mut e, 0x0059_c4f0, actor, 8, 0), (true, 7.5));
        // A non-actor and a null reference write nothing.
        let other = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_c4f0, other, 8, 0), (true, SENTINEL));
        assert_eq!(run(&mut e, 0x0059_c4f0, 0, 8, 0), (true, SENTINEL));

        // The debug line names the actor value.
        stub(&mut e, ACTOR_VALUE_SCRIPT_NAME, 0x7777);
        trace_on(&mut e);
        run(&mut e, 0x0059_c4f0, actor, 8, 0);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, ACTOR_VALUE_SCRIPT_NAME), vec![vec![8]]);
        let w = words(7.5);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4dd4, 0x7777, w[0], w[1]]]
        );
    }

    #[test]
    fn base_actor_value_is_an_integer_from_slot_zero() {
        let mut e = engine();
        stub(&mut e, 0x0f00_0042, (-12i32) as u32);
        let actor = actor_with_owners(&mut e, &[(0, 0x0f00_0042)], &[]);
        assert_eq!(run(&mut e, 0x0059_c5d0, actor, 3, 0), (true, -12.0));
        let other = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_c5d0, other, 3, 0), (true, SENTINEL));
        stub(&mut e, ACTOR_VALUE_SCRIPT_NAME, 0x7778);
        trace_on(&mut e);
        run(&mut e, 0x0059_c5d0, actor, 3, 0);
        let log = take_log(&mut e);
        let w = words(-12.0);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4df0, 0x7778, w[0], w[1]]]
        );
    }

    #[test]
    fn permanent_actor_value_uses_slot_0x20() {
        let mut e = engine();
        e.register(0x0f00_0043, |_, _| Ret {
            st0: 55.0,
            ..Ret::default()
        });
        e.register(0x0f00_0044, |_, _| Ret {
            st0: 66.0,
            ..Ret::default()
        });
        let actor = actor_with_owners(&mut e, &[(0x20, 0x0f00_0043)], &[(0x20, 0x0f00_0044)]);
        stub(&mut e, FORM_HAS_FLAG_0800, 0);
        assert_eq!(run(&mut e, 0x0059_c680, actor, 1, 0), (true, 55.0));
        stub(&mut e, FORM_HAS_FLAG_0800, 1);
        assert_eq!(run(&mut e, 0x0059_c680, actor, 1, 0), (true, 66.0));
        assert_eq!(run(&mut e, 0x0059_c680, 0, 1, 0), (true, SENTINEL));
    }

    #[test]
    fn actor_percentages_and_walk_speed() {
        let mut e = engine();
        let actor = actor(&mut e);
        let other = non_actor(&mut e);
        for (condition, callee, value) in [
            (0x0059_c760u32, 0x0089_3530u32, 0.75),
            (0x0059_c7e0, 0x0089_3590, 0.5),
            (0x0059_c860, 0x0088_4dc0, 135.0),
        ] {
            stub_st0(&mut e, callee, value);
            assert_eq!(run(&mut e, condition, actor, 0, 0), (true, value));
            assert_eq!(run(&mut e, condition, other, 0, 0), (true, SENTINEL));
            assert_eq!(run(&mut e, condition, 0, 0, 0), (true, SENTINEL));
        }
        trace_on(&mut e);
        run(&mut e, 0x0059_c7e0, actor, 0, 0);
        let log = take_log(&mut e);
        let w = words(0.5);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4e30, w[0], w[1]]]
        );
    }

    #[test]
    fn current_time_is_the_calendar_hour() {
        let mut e = engine();
        stub_st0(&mut e, 0x0086_7da0, 13.5);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_c8e0, 0, 0, 0), (true, 13.5));
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0086_7da0), vec![vec![CALENDAR]]);
    }

    #[test]
    fn scale_prints_the_base_scale_too() {
        let mut e = engine();
        stub_st0(&mut e, REF_GET_OWN_SCALE, 1.25);
        stub_st0(&mut e, REF_GET_SCALE, 2.5);
        assert_eq!(run(&mut e, 0x0059_c930, 0x2000, 0, 0), (true, 1.25));
        assert_eq!(run(&mut e, 0x0059_c930, 0, 0, 0), (true, SENTINEL));
        trace_on(&mut e);
        run(&mut e, 0x0059_c930, 0x2000, 0, 0);
        let log = take_log(&mut e);
        let result = words(1.25);
        let base = words(2.5);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4e80, result[0], result[1], base[0], base[1]]]
        );
    }

    /// Everything `GetLOS` calls for the player, with simple answers. The
    /// target's virtuals: `0x1d0` returns an owner whose `0xc` returns
    /// `0x6000`, `0x1f4` the position (1, 2, 10), `0x1dc` / `0x1d8` bounds
    /// with z 190 and 100, `0xfc` yes.
    fn los_engine() -> (Engine, u32, u32) {
        let mut e = engine();
        let player = actor(&mut e);
        e.set_global(PLAYER, player);
        e.set_global(TES, 0x00ee_0000u32);
        let owner = object(&mut e, &[(0xc, 0x0f00_0051)]);
        stub(&mut e, 0x0f00_0050, owner);
        stub(&mut e, 0x0f00_0051, 0x6000);
        let position = floats(&mut e, [1.0, 2.0, 10.0]);
        stub(&mut e, 0x0f00_0052, position);
        let max_bound = floats(&mut e, [0.0, 0.0, 190.0]);
        let min_bound = floats(&mut e, [0.0, 0.0, 100.0]);
        stub(&mut e, 0x0f00_0053, max_bound);
        stub(&mut e, 0x0f00_0054, min_bound);
        stub(&mut e, 0x0f00_0055, 1);
        let target = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (0x1d0, 0x0f00_0050),
                (VSLOT_GET_POSITION, 0x0f00_0052),
                (0x1dc, 0x0f00_0053),
                (0x1d8, 0x0f00_0054),
                (VSLOT_IS_MOBILE_OBJECT, 0x0f00_0055),
            ],
        );
        stub(&mut e, 0x0045_c670, 0x7000);
        stub(&mut e, 0x0055_8310, 0x7100);
        stub(&mut e, 0x0066_29f0, 0x7200);
        stub(&mut e, 0x004b_5fc0, 1);
        stub(&mut e, 0x0044_4ed0, 0);
        let origin = floats(&mut e, [5.0, 6.0, 7.0]);
        stub(&mut e, 0x0043_c490, origin);
        for addr in [
            0x0062_40d0u32,
            0x0098_ddd0,
            0x0063_f790,
            0x004a_3c20,
            0x0062_a190,
            0x004a_3da0,
            0x0093_1ed0,
            0x004a_3f70,
            0x004a_3eb0,
            0x004a_3ae0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        stub(&mut e, 0x004b_5ff0, 0);
        e.register(0x008c_71b0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x004a_39f0, |e, a| {
            let word = e.mem.u32(a[0]) & !0x7f | (a[1] & 0x7f);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        stub(&mut e, 0x004a_3a20, 0x1234);
        stub(&mut e, 0x0088_b880, 0);
        stub(&mut e, 0x0056_f930, 0);
        stub(&mut e, 0x0055_d520, 0x8000);
        (e, player, target)
    }

    #[test]
    fn los_player_sees_the_target_when_the_first_ray_is_unblocked() {
        let (mut e, player, target) = los_engine();
        // Records the ray end the pick was made with, and finds nothing.
        e.register_double(0x004a_3eb0, |e, a| {
            let z = e.mem.f32(a[1] + 8);
            e.mem.set_f32(0x0101_1ff0, z);
            Ret::default()
        });
        stub(&mut e, 0x0045_8420, 0);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_c990, player, target, 0), (true, 1.0));
        let log = take_log(&mut e);
        // Height 90 * 0.75 + the target position's z (10).
        assert_eq!(e.mem.f32(0x0101_1ff0), 77.5);
        assert_eq!(calls_to(&log, 0x0045_8420).len(), 1);
        assert_eq!(calls_to(&log, ACTOR_LINE_OF_SIGHT).len(), 0);
        // The filter: layer 0x25 and the target (it is a mobile object).
        assert_eq!(&calls_to(&log, 0x0062_a190)[0][1..], &[0x25, target]);
        // The collector is destroyed again.
        assert_eq!(calls_to(&log, 0x004a_3ae0).len(), 1);
        // "sees" line: names of the player and the target.
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4eb4, 0x8000, 0x8000]]
        );
    }

    #[test]
    fn los_player_tries_three_heights_then_falls_back_to_line_of_sight() {
        let (mut e, player, target) = los_engine();
        e.register(0x004a_3eb0, |e, a| {
            // Keep each ray end's z for the check below.
            let n = e.mem.u32(0x0101_1ff0);
            e.mem.set_u32(0x0101_1ff0, n + 1);
            let z = e.mem.f32(a[1] + 8);
            e.mem.set_f32(0x0101_1ff4 + 4 * n, z);
            Ret::default()
        });
        // Something else is always picked (a reference that is not the target).
        stub(&mut e, 0x0045_8420, 0x9999);
        stub(&mut e, 0x0056_f930, 0x4444);
        stub(&mut e, 0x0088_b880, 1);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_c990, player, target, 0), (true, 1.0));
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(0x0101_1ff0), 3);
        assert_eq!(e.mem.f32(0x0101_1ff4), 77.5);
        assert_eq!(e.mem.f32(0x0101_1ff8), 55.0);
        assert_eq!(e.mem.f32(0x0101_1ffc), 32.5);
        assert_eq!(
            calls_to(&log, ACTOR_LINE_OF_SIGHT),
            vec![vec![player, 0, target, 1, 0, 0]]
        );

        // The line of sight fails too: the player cannot see the target.
        stub(&mut e, 0x0088_b880, 0);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_c990, player, target, 0), (true, 0.0));
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4ea4, 0x8000, 0x8000]]
        );
    }

    #[test]
    fn los_player_sees_when_the_second_pick_is_the_target() {
        let (mut e, player, target) = los_engine();
        e.register_double(0x0045_8420, |_, _| Ret {
            eax: 0x9999,
            ..Ret::default()
        });
        // The first pick resolves to another reference, the second to the
        // target (compared by address).
        let target_address = target;
        let mut calls = 0;
        e.register_double(0x0056_f930, move |_, _| {
            calls += 1;
            Ret {
                eax: if calls == 2 { target_address } else { 0x4444 },
                ..Ret::default()
            }
        });
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_c990, player, target, 0), (true, 1.0));
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0045_8420).len(), 2);
        assert_eq!(calls_to(&log, ACTOR_LINE_OF_SIGHT).len(), 0);
    }

    #[test]
    fn los_player_cannot_see_a_target_outside_the_view() {
        let (mut e, player, target) = los_engine();
        stub(&mut e, 0x004b_5fc0, 0);
        stub(&mut e, 0x0044_4ed0, 0);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_c990, player, target, 0), (true, 0.0));
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0045_8420).len(), 0);
        assert_eq!(calls_to(&log, ACTOR_LINE_OF_SIGHT).len(), 0);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4ea4, 0x8000, 0x8000]]
        );
    }

    #[test]
    fn los_player_falls_back_on_a_bound_built_from_the_position() {
        let (mut e, player, target) = los_engine();
        // No bound owner test passes, but the target can be tested by a bound.
        stub(&mut e, 0x004b_5fc0, 0);
        stub(&mut e, 0x0044_4ed0, 1);
        stub(&mut e, 0x004b_5ff0, 1);
        stub(&mut e, 0x0045_8420, 0);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_c990, player, target, 0), (true, 1.0));
        let log = take_log(&mut e);
        // The bound gets the position and the radius 1.0.
        let bound_position = calls_to(&log, 0x0098_ddd0);
        assert_eq!(bound_position.len(), 1);
        assert_eq!(calls_to(&log, 0x0063_f790)[0][1], 1.0f32.to_bits());
    }

    #[test]
    fn los_other_actors_use_actor_line_of_sight() {
        let (mut e, _, target) = los_engine();
        let viewer = actor(&mut e);
        stub(&mut e, 0x0088_b880, 1);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_c990, viewer, target, 0), (true, 1.0));
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, ACTOR_LINE_OF_SIGHT),
            vec![vec![viewer, 0, target, 1, 0, 0]]
        );
        assert_eq!(calls_to(&log, 0x0045_8420).len(), 0);
        stub(&mut e, 0x0088_b880, 0);
        assert_eq!(run(&mut e, 0x0059_c990, viewer, target, 0), (true, 0.0));
        // A viewer that is not an actor, or no target: 0.
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_c990, plain, target, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_c990, viewer, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_c990, 0, target, 0), (true, 0.0));
    }

    #[test]
    fn collision_group_sets_the_upper_half_word() {
        let mut e = engine();
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 0x1234_5678);
        e.call(0x0059_ce80, &args![word, 0xabcdu32]);
        assert_eq!(e.mem.u32(word), 0xabcd_5678);
        // Only 16 bits of the argument fit.
        e.call(0x0059_ce80, &args![word, 0x1_0002u32]);
        assert_eq!(e.mem.u32(word), 0x0002_5678);
    }

    #[test]
    fn pick_data_gets_its_collector() {
        let mut e = engine();
        let pick = e.mem.alloc(0xb0);
        e.mem.set_u32(pick + 0xa8, 0xffff);
        e.call(0x0059_ceb0, &args![pick, 0x4321u32]);
        assert_eq!(e.mem.u32(pick + 0xa4), 0x4321);
        assert_eq!(e.mem.u32(pick + 0xa8), 0);
    }

    #[test]
    fn collector_destructors() {
        let mut e = engine();
        let collector = e.mem.alloc(0x90);
        e.register(0x004a_3ae0, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0059_cee0, &args![collector]);
        assert_eq!(e.mem.u32(collector), 0x0103_4ec4);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x004a_3ae0), vec![vec![collector]]);

        // The deleting form frees the memory only when bit 0 is set.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0059_cf20, &args![collector, 0u32]).u32(),
            collector
        );
        assert_eq!(
            e.call(0x0059_cf20, &args![collector, 3u32]).u32(),
            collector
        );
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![collector]]);
    }

    #[test]
    fn disabled_of_a_reference_follows_the_pending_flags() {
        let mut e = engine();
        // flag 0x800 set, not pending enable: disabled.
        stub(&mut e, FORM_HAS_FLAG_0800, 1);
        stub(&mut e, 0x005a_a680, 0);
        stub(&mut e, 0x005a_a630, 0);
        assert_eq!(run(&mut e, 0x0059_cf50, 0x2000, 0, 0), (true, 1.0));
        // flag set but pending enable: not disabled unless also pending disable.
        stub(&mut e, 0x005a_a680, 1);
        assert_eq!(run(&mut e, 0x0059_cf50, 0x2000, 0, 0), (true, 0.0));
        stub(&mut e, 0x005a_a630, 1);
        assert_eq!(run(&mut e, 0x0059_cf50, 0x2000, 0, 0), (true, 1.0));
        // flag clear: only pending disable counts.
        stub(&mut e, FORM_HAS_FLAG_0800, 0);
        assert_eq!(run(&mut e, 0x0059_cf50, 0x2000, 0, 0), (true, 1.0));
        stub(&mut e, 0x005a_a630, 0);
        assert_eq!(run(&mut e, 0x0059_cf50, 0x2000, 0, 0), (true, 0.0));
    }

    #[test]
    fn disabled_without_a_reference_reads_bit_zero_of_the_record() {
        let mut e = engine();
        let record = e.mem.alloc(0x10);
        e.mem.set_u8(record + 4, 0x03);
        assert_eq!(run(&mut e, 0x0059_cf50, 0, record, 0), (true, 1.0));
        e.mem.set_u8(record + 4, 0x02);
        assert_eq!(run(&mut e, 0x0059_cf50, 0, record, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_cf50, 0, 0, 0), (true, SENTINEL));
    }

    #[test]
    fn locked_reports_broken_and_locked_locks_and_terminals() {
        let mut e = engine();
        // A lock: broken is 2, otherwise locked (005021a0) is 1, else 0.
        stub(&mut e, REF_GET_LOCK, 0x5000);
        stub(&mut e, 0x0043_0ae0, 1);
        assert_eq!(run(&mut e, 0x0059_d010, 0x2000, 0, 0), (true, 2.0));
        stub(&mut e, 0x0043_0ae0, 0);
        stub(&mut e, 0x0050_21a0, 1);
        assert_eq!(run(&mut e, 0x0059_d010, 0x2000, 0, 0), (true, 1.0));
        stub(&mut e, 0x0050_21a0, 0);
        assert_eq!(run(&mut e, 0x0059_d010, 0x2000, 0, 0), (true, 0.0));

        // No lock: a terminal depends on the player's access and whether it
        // is unlocked; other forms give 0.
        stub(&mut e, REF_GET_LOCK, 0);
        stub(&mut e, REF_GET_BASE_FORM, 0x6000);
        stub(&mut e, FORM_GET_TYPE, FORM_TYPE_TERMINAL);
        let player = actor(&mut e);
        e.set_global(PLAYER, player);
        stub(&mut e, 0x0096_6c60, 2);
        assert_eq!(run(&mut e, 0x0059_d010, 0x2000, 0, 0), (true, 2.0));
        stub(&mut e, 0x0096_6c60, 0);
        stub(&mut e, 0x0050_1ae0, 0);
        assert_eq!(run(&mut e, 0x0059_d010, 0x2000, 0, 0), (true, 1.0));
        stub(&mut e, 0x0050_1ae0, 1);
        assert_eq!(run(&mut e, 0x0059_d010, 0x2000, 0, 0), (true, 0.0));
        stub(&mut e, FORM_GET_TYPE, 0x28);
        assert_eq!(run(&mut e, 0x0059_d010, 0x2000, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_d010, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn lock_level_of_locks_and_terminals() {
        let mut e = engine();
        stub(&mut e, REF_GET_LOCK, 0x5000);
        stub(&mut e, 0x0043_0a10, 75);
        assert_eq!(run(&mut e, 0x0059_d100, 0x2000, 0, 0), (true, 75.0));

        stub(&mut e, REF_GET_LOCK, 0);
        stub(&mut e, REF_GET_BASE_FORM, 0x6000);
        stub(&mut e, FORM_GET_TYPE, FORM_TYPE_TERMINAL);
        stub(&mut e, 0x0050_1ae0, 1);
        assert_eq!(run(&mut e, 0x0059_d100, 0x2000, 0, 0), (true, -1.0));
        stub(&mut e, 0x0050_1ae0, 0);
        stub(&mut e, 0x0050_11a0, 50);
        assert_eq!(run(&mut e, 0x0059_d100, 0x2000, 0, 0), (true, 50.0));
        stub(&mut e, FORM_GET_TYPE, 0x28);
        assert_eq!(run(&mut e, 0x0059_d100, 0x2000, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_d100, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn is_lock_broken() {
        let mut e = engine();
        stub(&mut e, REF_GET_LOCK, 0x5000);
        stub(&mut e, 0x0043_0ae0, 1);
        assert_eq!(run(&mut e, 0x0059_d1d0, 0x2000, 0, 0), (true, 1.0));
        stub(&mut e, 0x0043_0ae0, 0);
        assert_eq!(run(&mut e, 0x0059_d1d0, 0x2000, 0, 0), (true, 0.0));
        stub(&mut e, REF_GET_LOCK, 0);
        assert_eq!(run(&mut e, 0x0059_d1d0, 0x2000, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_d1d0, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn disease_is_always_zero() {
        let mut e = engine();
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_d250, 0x2000, 0, 0), (true, 0.0));
        let log = take_log(&mut e);
        let w = words(0.0);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4f24, w[0], w[1]]]
        );
    }

    #[test]
    fn vampire_asks_the_actor() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, 0x0047_c850, 1);
        assert_eq!(run(&mut e, 0x0059_d2a0, actor, 0, 0), (true, 1.0));
        stub(&mut e, 0x0047_c850, 0);
        assert_eq!(run(&mut e, 0x0059_d2a0, actor, 0, 0), (true, 0.0));
        let other = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_d2a0, other, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_d2a0, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn clothing_value_needs_an_npc() {
        let mut e = engine();
        stub(&mut e, REF_GET_BASE_FORM, 0x6000);
        stub(&mut e, FORM_GET_TYPE, FORM_TYPE_NPC);
        stub_st0(&mut e, 0x008d_3110, 123.0);
        assert_eq!(run(&mut e, 0x0059_d320, 0x2000, 0, 0), (true, 123.0));
        // Not an NPC: the result is not touched.
        stub(&mut e, FORM_GET_TYPE, 0x28);
        assert_eq!(run(&mut e, 0x0059_d320, 0x2000, 0, 0), (true, SENTINEL));
        assert_eq!(run(&mut e, 0x0059_d320, 0, 0, 0), (true, SENTINEL));
    }

    /// An NPC reference whose base form is `base`; `FORM_GET_TYPE` answers
    /// NPC for every form.
    fn npc_reference(e: &mut Engine, base: u32) -> u32 {
        let reference = e.mem.alloc(0x40);
        e.mem.set_u32(reference + 0x20, base);
        reference
    }

    fn install_base_form_stubs(e: &mut Engine) {
        e.register(REF_GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        stub(e, FORM_GET_TYPE, FORM_TYPE_NPC);
    }

    #[test]
    fn same_race_compares_the_races_of_two_npcs() {
        let mut e = engine();
        install_base_form_stubs(&mut e);
        e.register(0x004a_c110, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        let first_base = e.mem.alloc(0x40);
        let second_base = e.mem.alloc(0x40);
        e.mem.set_u32(first_base + 0x10, 5);
        e.mem.set_u32(second_base + 0x10, 5);
        let first = npc_reference(&mut e, first_base);
        let second = npc_reference(&mut e, second_base);
        assert_eq!(run(&mut e, 0x0059_d540, first, second, 0), (true, 1.0));
        e.mem.set_u32(second_base + 0x10, 6);
        assert_eq!(run(&mut e, 0x0059_d540, first, second, 0), (true, 0.0));
        // A null reference (or one that is not an NPC) gives 0.
        assert_eq!(run(&mut e, 0x0059_d540, first, 0, 0), (true, 0.0));
        stub(&mut e, FORM_GET_TYPE, 0x28);
        assert_eq!(run(&mut e, 0x0059_d540, first, second, 0), (true, 0.0));

        // Against the player (0059d3c0).
        stub(&mut e, FORM_GET_TYPE, FORM_TYPE_NPC);
        e.set_global(PLAYER, first);
        e.mem.set_u32(second_base + 0x10, 5);
        assert_eq!(run(&mut e, 0x0059_d3c0, second, 0, 0), (true, 1.0));
        e.mem.set_u32(second_base + 0x10, 9);
        assert_eq!(run(&mut e, 0x0059_d3c0, second, 0, 0), (true, 0.0));
    }

    #[test]
    fn same_sex_compares_the_sexes_of_two_npcs() {
        let mut e = engine();
        install_base_form_stubs(&mut e);
        e.register(0x005f_0cc0, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        let first_base = e.mem.alloc(0x40);
        let second_base = e.mem.alloc(0x40);
        e.mem.set_u32(first_base + 0x10, 1);
        e.mem.set_u32(second_base + 0x10, 1);
        let first = npc_reference(&mut e, first_base);
        let second = npc_reference(&mut e, second_base);
        assert_eq!(run(&mut e, 0x0059_d610, first, second, 0), (true, 1.0));
        e.mem.set_u32(second_base + 0x10, 0);
        assert_eq!(run(&mut e, 0x0059_d610, first, second, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_d610, 0, second, 0), (true, 0.0));

        // Against the player (0059d3e0).
        e.set_global(PLAYER, first);
        assert_eq!(run(&mut e, 0x0059_d3e0, second, 0, 0), (true, 0.0));
        e.mem.set_u32(second_base + 0x10, 1);
        assert_eq!(run(&mut e, 0x0059_d3e0, second, 0, 0), (true, 1.0));
    }

    /// A faction list of `factions` (each wrapped in a rank record whose
    /// first field is the faction) at `base + 0x5c`, the place `005d8a70`
    /// (`this + 0x2c`) of `base + 0x30` returns.
    fn faction_base(e: &mut Engine, factions: &[u32]) -> u32 {
        let base = e.mem.alloc(0x80);
        let mut node = base + 0x5c;
        for (i, &faction) in factions.iter().enumerate() {
            let rank_record = e.mem.alloc(8);
            e.mem.set_u32(rank_record, faction);
            e.mem.set_u32(node, rank_record);
            if i + 1 < factions.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        base
    }

    #[test]
    fn same_faction_looks_for_a_shared_rank() {
        let mut e = engine();
        e.register(0x005d_8a70, |_, a| (a[0] + 0x2c).into_ret());
        e.register(ACTOR_GET_BASE_FORM, |e, a| {
            e.mem.u32(a[0] + 0x20).into_ret()
        });
        let first_base = faction_base(&mut e, &[0xa1, 0xa2, 0xa3]);
        let second_base = faction_base(&mut e, &[0xb1]);
        let first = {
            let reference = actor(&mut e);
            e.mem.set_u32(reference + 0x20, first_base);
            reference
        };
        let second = {
            let reference = actor(&mut e);
            e.mem.set_u32(reference + 0x20, second_base);
            reference
        };
        // Rank of faction 0xa3 in the second actor's data is 3, others -1.
        e.register(0x0047_d680, |_, a| {
            (if a[1] == 0xa3 { 3i32 } else { -1 }).into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_d400, first, second, 0), (true, 1.0));
        e.register(0x0047_d680, |_, _| (-1i32).into_ret());
        assert_eq!(run(&mut e, 0x0059_d400, first, second, 0), (true, 0.0));

        // The "is the player" flag is passed on.
        e.set_global(PLAYER, second);
        e.register_double(0x0047_d680, |_, _| (-1i32).into_ret());
        e.call_log = Some(vec![]);
        run(&mut e, 0x0059_d400, first, second, 0);
        let log = take_log(&mut e);
        let calls = calls_to(&log, 0x0047_d680);
        assert_eq!(calls.len(), 3);
        assert!(calls
            .iter()
            .all(|w| w[0] == second_base + 0x30 && w[2] == 1));

        // Not an actor, or no second reference: 0, no debug line needed.
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_d400, plain, second, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_d400, first, 0, 0), (true, 0.0));

        // The wrapper (0059d3a0) compares with the player.
        e.register_double(0x0047_d680, |_, a| {
            (if a[1] == 0xa2 { 1i32 } else { -1 }).into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_d3a0, first, 0, 0), (true, 1.0));
    }

    #[test]
    fn detected_needs_both_actors_and_both_processes() {
        let mut e = engine();
        let detector = actor(&mut e);
        let subject = actor(&mut e);
        e.set_global(PLAYER, 0x00ee_1111u32);
        e.register(ACTOR_GET_PROCESS, |e, a| {
            // The "process" of an actor is the object at +0x30; its vtable
            // holds the light level virtual at +0x734.
            e.mem.u32(a[0] + 0x30).into_ret()
        });
        let process = object(&mut e, &[(0x734, 0x0f00_0070)]);
        e.mem.set_u32(detector + 0x30, 0x8888);
        e.mem.set_u32(subject + 0x30, process);
        e.register(0x0f00_0070, |_, _| Ret {
            st0: 37.9,
            ..Ret::default()
        });
        stub(&mut e, 0x0049_3bb0, 1);
        stub(&mut e, 0x008a_0d10, 25);
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        assert_eq!(run(&mut e, 0x0059_d6e0, detector, subject, 0), (true, 1.0));

        // The call: this = detector, (0, subject, &flag, 0, in_combat, 0, 0).
        trace_on(&mut e);
        run(&mut e, 0x0059_d6e0, detector, subject, 0);
        let log = take_log(&mut e);
        let detection = calls_to(&log, 0x008a_0d10);
        assert_eq!(detection.len(), 1);
        assert_eq!(detection[0][0], detector);
        assert_eq!(detection[0][2], subject);
        assert_eq!(detection[0][5], 1);
        // "GetDetected >> %i and light %i": level 25, light 37.
        assert_eq!(calls_to(&log, DEBUG_PRINT), vec![vec![0x0103_4fb8, 25, 37]]);

        // Level 0: not detected, still no error.
        stub(&mut e, 0x008a_0d10, 0);
        assert_eq!(run(&mut e, 0x0059_d6e0, detector, subject, 0), (true, 0.0));

        // The subject being the player uses the combat query with an out flag.
        e.set_global(PLAYER, subject);
        stub(&mut e, 0x0095_3c50, 0);
        e.call_log = Some(vec![]);
        run(&mut e, 0x0059_d6e0, detector, subject, 0);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0095_3c50).len(), 1);
        assert_eq!(calls_to(&log, 0x0049_3bb0).len(), 0);

        // A missing process: nothing is asked, level 0 and light -1 stay.
        e.mem.set_u32(detector + 0x30, 0);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_d6e0, detector, subject, 0), (true, 0.0));
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4fb8, 0, (-1i32) as u32]]
        );
    }

    #[test]
    fn dead_asks_the_actor_virtual_with_one() {
        let mut e = engine();
        e.register(0x0f00_0080, |_, a| (a[1] == 1).into_ret());
        let dead = object(
            &mut e,
            &[(VSLOT_IS_ACTOR, IS_ACTOR_YES), (0x22c, 0x0f00_0080)],
        );
        assert_eq!(run(&mut e, 0x0059_d840, dead, 0, 0), (true, 1.0));
        stub(&mut e, 0x0f00_0081, 0);
        let alive = object(
            &mut e,
            &[(VSLOT_IS_ACTOR, IS_ACTOR_YES), (0x22c, 0x0f00_0081)],
        );
        assert_eq!(run(&mut e, 0x0059_d840, alive, 0, 0), (true, 0.0));
        let other = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_d840, other, 0, 0), (true, 0.0));
    }

    /// A bound object (virtual `IsBoundObject` yes).
    fn bound_object(e: &mut Engine) -> u32 {
        stub(e, 0x0f00_0090, 1);
        object(e, &[(VSLOT_IS_BOUND_OBJECT, 0x0f00_0090)])
    }

    #[test]
    fn item_count_of_one_item_and_of_a_form_list() {
        let mut e = engine();
        stub(&mut e, REF_HAS_CONTAINER, 0x1111);
        stub(&mut e, GET_INVENTORY_CHANGES, 0x3000);
        // Counts by item address: the first item has 4, the second 6.
        let first = bound_object(&mut e);
        let second = bound_object(&mut e);
        e.register_double(INVENTORY_GET_OBJECT_COUNT, move |_, a| {
            assert_eq!(a[0], 0x3000);
            (if a[1] == first { 4u32 } else { 6 }).into_ret()
        });
        // The CRT function passes the count through.
        e.register(ITEM_COUNT_FILTER, |_, a| a[0].into_ret());
        assert_eq!(run(&mut e, 0x0059_d8e0, 0x2000, first, 0), (true, 4.0));

        // A form list (type 0x55; itself not a bound object) of both items:
        // 4 + 6. Its list head node is embedded at +0x18.
        let list_object = object(&mut e, &[(VSLOT_IS_BOUND_OBJECT, IS_ACTOR_NO)]);
        let second_node = e.mem.alloc(8);
        e.mem.set_u32(list_object + 0x18, first);
        e.mem.set_u32(list_object + 0x1c, second_node);
        e.mem.set_u32(second_node, second);
        stub(&mut e, FORM_GET_TYPE, FORM_TYPE_FORM_LIST);
        assert_eq!(
            run(&mut e, 0x0059_d8e0, 0x2000, list_object, 0),
            (true, 10.0)
        );

        // Not a form list and not a bound object: nothing is counted.
        stub(&mut e, FORM_GET_TYPE, 0x28);
        assert_eq!(
            run(&mut e, 0x0059_d8e0, 0x2000, list_object, 0),
            (true, 0.0)
        );
        assert_eq!(run(&mut e, 0x0059_d8e0, 0x2000, 0, 0), (true, 0.0));

        // No inventory changes: 0.
        stub(&mut e, GET_INVENTORY_CHANGES, 0);
        assert_eq!(run(&mut e, 0x0059_d8e0, 0x2000, first, 0), (true, 0.0));
    }

    #[test]
    fn item_count_without_a_container_prints_the_complaint() {
        let mut e = engine();
        stub(&mut e, REF_HAS_CONTAINER, 0);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_d8e0, 0x2000, 0x5000, 0), (true, 0.0));
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, DEBUG_PRINT), vec![vec![0x0103_5004]]);
        assert_eq!(calls_to(&log, GET_INVENTORY_CHANGES).len(), 0);
    }

    #[test]
    fn equipped_checks_one_item_or_a_whole_list() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, GET_INVENTORY_CHANGES, 0x3000);
        let worn = bound_object(&mut e);
        let other_item = bound_object(&mut e);
        // Single items: the form type is not a form list.
        stub(&mut e, FORM_GET_TYPE, 0x28);
        e.register_double(0x004b_fda0, move |_, a| {
            assert_eq!(a[0], 0x3000);
            assert_eq!(a[2], 0);
            (a[1] == worn).into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_da90, actor, worn, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_da90, actor, other_item, 0), (true, 0.0));

        // A list of [other_item, worn]: found at the second entry.
        let list_object = object(&mut e, &[(VSLOT_IS_BOUND_OBJECT, IS_ACTOR_NO)]);
        let second_node = e.mem.alloc(8);
        e.mem.set_u32(list_object + 0x18, other_item);
        e.mem.set_u32(list_object + 0x1c, second_node);
        e.mem.set_u32(second_node, worn);
        stub(&mut e, FORM_GET_TYPE, FORM_TYPE_FORM_LIST);
        assert_eq!(run(&mut e, 0x0059_da90, actor, list_object, 0), (true, 1.0));
        // Without the worn item in it, nothing is equipped.
        e.mem.set_u32(second_node, other_item);
        assert_eq!(run(&mut e, 0x0059_da90, actor, list_object, 0), (true, 0.0));

        // A non-actor, or no form: 0.
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_da90, plain, worn, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_da90, actor, 0, 0), (true, 0.0));
    }

    #[test]
    fn gold_counts_the_caps_form_in_the_inventory() {
        let mut e = engine();
        stub(&mut e, 0x0048_39c0, 0xca95);
        stub(&mut e, REF_HAS_CONTAINER, 1);
        stub(&mut e, GET_INVENTORY_CHANGES, 0x3000);
        e.register_double(INVENTORY_GET_OBJECT_COUNT, |_, a| {
            assert_eq!((a[0], a[1]), (0x3000, 0xca95));
            250u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, 0x0059_dbe0, 0x2000, 0, 0), (true, 250.0));
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0048_39c0), vec![vec![0xf]]);

        // No container, no inventory, no caps form or no reference: 0.
        stub(&mut e, REF_HAS_CONTAINER, 0);
        assert_eq!(run(&mut e, 0x0059_dbe0, 0x2000, 0, 0), (true, 0.0));
        stub(&mut e, REF_HAS_CONTAINER, 1);
        stub(&mut e, GET_INVENTORY_CHANGES, 0);
        assert_eq!(run(&mut e, 0x0059_dbe0, 0x2000, 0, 0), (true, 0.0));
        stub(&mut e, 0x0048_39c0, 0);
        assert_eq!(run(&mut e, 0x0059_dbe0, 0x2000, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_dbe0, 0, 0, 0), (true, 0.0));
    }

    // ---- second session: 0059c380, 0059dc90 to 0059f540 ----

    /// A virtual-slot function answering `value` in `eax`.
    fn slot_function(e: &mut Engine, addr: u32, value: u32) -> u32 {
        e.register_double(addr, move |_, _| value.into_ret());
        addr
    }

    /// `TESForm::cFormType` (`00401170`) reading the byte at `+4`.
    fn use_form_types(e: &mut Engine) {
        e.register(FORM_GET_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
    }

    /// A form-like object whose type byte (`+4`) is `form_type`.
    fn typed_object(e: &mut Engine, form_type: u8) -> u32 {
        let object = e.mem.alloc(0x200);
        e.mem.set_u8(object + 4, form_type);
        object
    }

    /// An actor object (`IsActor` yes) with extra vtable slots.
    fn actor_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let mut all = vec![(VSLOT_IS_ACTOR, IS_ACTOR_YES)];
        all.extend_from_slice(slots);
        object(e, &all)
    }

    /// Whether the debug line was printed with the format at `format`.
    fn printed(log: &Log, format: u32) -> bool {
        calls_to(log, DEBUG_PRINT).iter().any(|w| w[0] == format)
    }

    #[test]
    fn menu_mode_asks_the_menu_functions() {
        let mut e = engine();
        stub(&mut e, INTERFACE_IS_IN_MENU_MODE, 1);
        assert_eq!(run(&mut e, 0x0059_c380, 0, 0, 0), (true, 1.0));
        stub(&mut e, INTERFACE_IS_IN_MENU_MODE, 0);
        assert_eq!(run(&mut e, 0x0059_c380, 0, 0, 0), (true, 0.0));

        // A menu id: visibility (with a 0 second argument) ...
        e.register_double(INTERFACE_IS_MENU_ID_VISIBLE, |_, a| {
            assert_eq!(a, [0x3ec, 0]);
            true.into_ret()
        });
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_c380, 0, 0x3ec, 0), (true, 1.0));
        let log = take_log(&mut e);
        let w = words(1.0);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_4da0, 0x3ec, w[0], w[1]]]
        );
        // ... or, with the byte at 011cab24 set, the top menu only.
        e.mem.set_u8(MENU_MODE_TOP_ONLY, 1);
        stub(&mut e, INTERFACE_IS_TOP_MENU_ID, 0);
        assert_eq!(run(&mut e, 0x0059_c380, 0, 0x3ec, 0), (true, 0.0));
        e.register_double(INTERFACE_IS_TOP_MENU_ID, |_, a| {
            assert_eq!(a, [0x3ec]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_c380, 0, 0x3ec, 0), (true, 1.0));
    }

    #[test]
    fn sleeping_and_sitting_count_up_by_state() {
        let mut e = engine();
        // Sleeping: states 6..=10 give 1, 2, 2, 3, 4; sitting: 1..=5 the same.
        for (state, sleeping, sitting) in [
            (0u32, 0.0, 0.0),
            (1, 0.0, 1.0),
            (2, 0.0, 2.0),
            (3, 0.0, 2.0),
            (4, 0.0, 3.0),
            (5, 0.0, 4.0),
            (6, 1.0, 0.0),
            (7, 2.0, 0.0),
            (8, 2.0, 0.0),
            (9, 3.0, 0.0),
            (10, 4.0, 0.0),
            (11, 0.0, 0.0),
        ] {
            let state_function = slot_function(&mut e, 0x0f00_0100 + state, state);
            let actor = actor_with(&mut e, &[(VSLOT_SIT_SLEEP_STATE, state_function)]);
            assert_eq!(
                run(&mut e, 0x0059_dc90, actor, 0, 0),
                (true, sleeping),
                "sleeping state {state}"
            );
            assert_eq!(
                run(&mut e, 0x0059_dd90, actor, 0, 0),
                (true, sitting),
                "sitting state {state}"
            );
        }
        // A non-actor and no reference: 0.
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_dc90, plain, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_dd90, 0, 0, 0), (true, 0.0));
        // The debug lines.
        let six = slot_function(&mut e, 0x0f00_0200, 6);
        let actor = actor_with(&mut e, &[(VSLOT_SIT_SLEEP_STATE, six)]);
        trace_on(&mut e);
        run(&mut e, 0x0059_dc90, actor, 0, 0);
        run(&mut e, 0x0059_dd90, actor, 0, 0);
        let log = take_log(&mut e);
        let one = words(1.0);
        let zero = words(0.0);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![
                vec![0x0103_505c, one[0], one[1]],
                vec![0x0103_5074, zero[0], zero[1]]
            ]
        );
    }

    #[test]
    fn furniture_marker_id_is_unsigned() {
        let mut e = engine();
        let marker = slot_function(&mut e, 0x0f00_0300, 0xffff_fff0);
        let process = object(&mut e, &[(VSLOT_PROCESS_FURNITURE_MARKER_ID, marker)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        let actor = actor(&mut e);
        assert_eq!(
            run(&mut e, 0x0059_de90, actor, 0, 0),
            (true, 4_294_967_280.0)
        );
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_de90, plain, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_de90, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn current_furniture_reference_is_compared() {
        let mut e = engine();
        let furniture = slot_function(&mut e, 0x0f00_0310, 0x7000);
        let process = object(&mut e, &[(VSLOT_PROCESS_FURNITURE_REFERENCE, furniture)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        let actor = actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_df40, actor, 0x7000, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_df40, actor, 0x7001, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_df40, plain, 0x7000, 0), (true, 0.0));
    }

    #[test]
    fn current_furniture_object_matches_a_furn_or_a_list() {
        let mut e = engine();
        use_form_types(&mut e);
        let furniture = typed_object(&mut e, 0x27);
        let list = typed_object(&mut e, 0x55);
        let in_use = slot_function(&mut e, 0x0f00_0320, 0x7000);
        let process = object(&mut e, &[(VSLOT_PROCESS_FURNITURE_REFERENCE, in_use)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        let actor = actor(&mut e);
        // GetForm of the furniture reference.
        stub(&mut e, REF_GET_BASE_FORM, furniture);
        assert_eq!(run(&mut e, 0x0059_dfe0, actor, furniture, 0), (true, 1.0));
        let other_furniture = typed_object(&mut e, 0x27);
        assert_eq!(
            run(&mut e, 0x0059_dfe0, actor, other_furniture, 0),
            (true, 0.0)
        );

        // A form list: the base form is looked up in the list through the
        // list-contains function, handed the address of a local that holds it.
        e.register_double(LIST_CONTAINS, move |e, a| {
            assert_eq!(a[0], list + 0x18);
            (e.mem.u32(a[1]) == furniture).into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_dfe0, actor, list, 0), (true, 1.0));
        stub(&mut e, REF_GET_BASE_FORM, other_furniture);
        assert_eq!(run(&mut e, 0x0059_dfe0, actor, list, 0), (true, 0.0));

        // Any other type, no furniture in use, no form or a non-actor: 0.
        let weapon = typed_object(&mut e, 0x28);
        assert_eq!(run(&mut e, 0x0059_dfe0, actor, weapon, 0), (true, 0.0));
        let nothing = slot_function(&mut e, 0x0f00_0321, 0);
        let idle_process = object(&mut e, &[(VSLOT_PROCESS_FURNITURE_REFERENCE, nothing)]);
        stub(&mut e, ACTOR_GET_PROCESS, idle_process);
        assert_eq!(run(&mut e, 0x0059_dfe0, actor, furniture, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_dfe0, actor, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_dfe0, plain, furniture, 0), (true, 0.0));
    }

    #[test]
    fn talked_to_pc_asks_the_actor_or_the_fallback() {
        let mut e = engine();
        let yes = slot_function(&mut e, 0x0f00_0330, 1);
        let no = slot_function(&mut e, 0x0f00_0331, 0);
        let talked = actor_with(&mut e, &[(VSLOT_TALKED_TO_PC, yes)]);
        let silent = actor_with(&mut e, &[(VSLOT_TALKED_TO_PC, no)]);
        assert_eq!(run(&mut e, 0x0059_e0f0, talked, 0, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_e0f0, silent, 0, 0), (true, 0.0));
        // A reference that is not an actor is dropped, whatever the fallback.
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_e0f0, plain, talked, 0), (true, 0.0));
        // No reference at all: the first parameter is the actor (untested).
        assert_eq!(run(&mut e, 0x0059_e0f0, 0, talked, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_e0f0, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn get_variable_reads_the_locals_of_a_reference_an_item_or_a_quest() {
        let mut e = engine();
        use_form_types(&mut e);
        let locals = e.mem.alloc(8);
        e.mem.set_u32(locals, 0x1234);
        e.register_double(SCRIPT_LOCALS_GET_VARIABLE, move |_, a| {
            assert_eq!((a[0], a[1], a[2]), (locals, 7, 0));
            Ret {
                st0: 3.5,
                ..Ret::default()
            }
        });
        // A placed reference that is not carried: its own script variables.
        let reference = typed_object(&mut e, 0x3b);
        stub(&mut e, REF_GET_EXTRA_DATA_LIST, 0x6000);
        stub(&mut e, EXTRA_DATA_LIST_GET_REFERENCE_POINTER, 0);
        stub(&mut e, REF_GET_SCRIPT_VARIABLES, locals);
        assert_eq!(run(&mut e, 0x0059_e1a0, 0, reference, 7), (true, 3.5));

        // Locals that are not set up (first word 0): 0.0.
        e.mem.set_u32(locals, 0);
        assert_eq!(run(&mut e, 0x0059_e1a0, 0, reference, 7), (true, 0.0));
        e.mem.set_u32(locals, 0x1234);

        // A carried item: the owner's inventory item, its locals, then the
        // item is destroyed (flag 1).
        let owner = 0x9100;
        stub(&mut e, EXTRA_DATA_LIST_GET_REFERENCE_POINTER, owner);
        stub(&mut e, REF_GET_ITEM_ID, 0x55);
        stub(&mut e, REF_GET_BASE_FORM, 0xba5e);
        stub(&mut e, REF_GET_INVENTORY_ITEM, 0x9200);
        stub(&mut e, ITEM_CHANGE_GET_SCRIPT_LOCALS, locals);
        stub(&mut e, ITEM_CHANGE_DESTROY, 0);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, 0x0059_e1a0, 0, reference, 7), (true, 3.5));
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, REF_GET_INVENTORY_ITEM),
            vec![vec![owner, 0xba5e, 0x55]]
        );
        assert_eq!(calls_to(&log, ITEM_CHANGE_DESTROY), vec![vec![0x9200, 1]]);
        assert!(calls_to(&log, REF_GET_SCRIPT_VARIABLES).is_empty());
        // No such inventory item: nothing to read or destroy.
        stub(&mut e, REF_GET_INVENTORY_ITEM, 0);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, 0x0059_e1a0, 0, reference, 7), (true, 0.0));
        let log = take_log(&mut e);
        assert!(calls_to(&log, ITEM_CHANGE_DESTROY).is_empty());

        // A quest: the pointer at +0x5c.
        let quest = typed_object(&mut e, 0x47);
        e.mem.set_u32(quest + 0x5c, locals);
        assert_eq!(run(&mut e, 0x0059_e1a0, 0, quest, 7), (true, 3.5));
        assert_eq!(fn_0059e300(&mut e, Ptr::new(quest)), locals);
        // Another form type, or none: 0.
        let weapon = typed_object(&mut e, 0x28);
        assert_eq!(run(&mut e, 0x0059_e1a0, 0, weapon, 7), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_e1a0, 0, 0, 7), (true, 0.0));
    }

    #[test]
    fn quest_conditions_ask_the_quest() {
        let mut e = engine();
        trace_on(&mut e);
        // Running.
        stub(&mut e, QUEST_IS_RUNNING, 1);
        assert_eq!(run(&mut e, 0x0059_e320, 0, 0x4000, 0), (true, 1.0));
        stub(&mut e, QUEST_IS_RUNNING, 0);
        assert_eq!(run(&mut e, 0x0059_e320, 0, 0x4000, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_e320, 0, 0, 0), (true, 0.0));
        // Completed: bit 2 of the flag byte.
        let flags = e.mem.alloc(4);
        stub(&mut e, QUEST_GET_FLAGS_ADDRESS, flags);
        e.mem.set_u8(flags, 0x05);
        assert_eq!(run(&mut e, 0x0059_e390, 0, 0x4000, 0), (true, 0.0));
        e.mem.set_u8(flags, 0x06);
        assert_eq!(run(&mut e, 0x0059_e390, 0, 0x4000, 0), (true, 1.0));
        assert!(fn_0059e400(&mut e, Ptr::new(0x4000)));
        assert_eq!(run(&mut e, 0x0059_e390, 0, 0, 0), (true, 0.0));
        // Current stage.
        e.register_double(QUEST_GET_CURRENT_STAGE, |_, a| {
            assert_eq!(a, [0x4000]);
            40u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_e420, 0, 0x4000, 0), (true, 40.0));
        assert_eq!(run(&mut e, 0x0059_e420, 0, 0, 0), (true, 0.0));
        // Stage done: the stage is the low byte of the second parameter.
        e.register_double(QUEST_IS_STAGE_DONE, |_, a| {
            assert_eq!(a, [0x4000, 0x78]);
            true.into_ret()
        });
        assert_eq!(
            run(&mut e, 0x0059_e490, 0, 0x4000, 0x1234_5678),
            (true, 1.0)
        );
        assert_eq!(run(&mut e, 0x0059_e490, 0, 0, 0x78), (true, 0.0));
        let log = take_log(&mut e);
        assert!(printed(&log, 0x0103_5100));
        assert!(printed(&log, 0x0103_511c));
        assert!(printed(&log, 0x0103_5138));
        assert!(printed(&log, 0x0103_514c));
    }

    #[test]
    fn faction_rank_difference_needs_two_members() {
        let mut e = engine();
        use_form_types(&mut e);
        let faction = typed_object(&mut e, 8);
        let actor = actor(&mut e);
        let other = typed_object(&mut e, 0x3b);
        e.set_global::<u32>(PLAYER, other);
        e.register(ACTOR_GET_BASE_FORM, |_, a| (a[0] + 0x1000).into_ret());
        // Rank 5 for the actor, 3 for the other; (faction, is the player).
        e.register_double(ACTOR_BASE_DATA_GET_FACTION_RANK, move |_, a| {
            assert_eq!(a[1], faction);
            if a[0] == actor + 0x1000 + 0x30 {
                assert_eq!(a[2], 0);
                5u32.into_ret()
            } else {
                assert_eq!(a[0], other + 0x1000 + 0x30);
                assert_eq!(a[2], 1);
                3u32.into_ret()
            }
        });
        assert_eq!(run(&mut e, 0x0059_e510, actor, faction, other), (true, 2.0));
        // Not a member of the faction: nothing written.
        e.register_double(ACTOR_BASE_DATA_GET_FACTION_RANK, move |_, a| {
            if a[2] == 1 {
                u32::MAX.into_ret()
            } else {
                5u32.into_ret()
            }
        });
        assert_eq!(run(&mut e, 0x0059_e510, actor, faction, other), (true, 0.0));
        // Not a faction.
        let class = typed_object(&mut e, 7);
        assert_eq!(run(&mut e, 0x0059_e510, actor, class, other), (true, 0.0));

        // No actor, or no other reference: back without a debug line.
        trace_on(&mut e);
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_e510, plain, faction, other), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_e510, actor, faction, 0), (true, 0.0));
        let log = take_log(&mut e);
        assert!(!printed(&log, 0x0103_5164));
        // The valid case prints.
        trace_on(&mut e);
        run(&mut e, 0x0059_e510, actor, faction, other);
        let log = take_log(&mut e);
        assert!(printed(&log, 0x0103_5164));
    }

    #[test]
    fn alarmed_compares_the_procedure_name_with_alarm() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, ACTOR_GET_PROCEDURE_NAME, 0x9000);
        e.register_double(STRING_COMPARE_NO_CASE, |_, a| {
            assert_eq!(a, [0x9000, 0x0103_519c]);
            0u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_e650, actor, 0, 0), (true, 1.0));
        stub(&mut e, STRING_COMPARE_NO_CASE, 1);
        assert_eq!(run(&mut e, 0x0059_e650, actor, 0, 0), (true, 0.0));
        // No name, a non-actor or no reference: 0.
        stub(&mut e, STRING_COMPARE_NO_CASE, 0);
        stub(&mut e, ACTOR_GET_PROCEDURE_NAME, 0);
        assert_eq!(run(&mut e, 0x0059_e650, actor, 0, 0), (true, 0.0));
        stub(&mut e, ACTOR_GET_PROCEDURE_NAME, 0x9000);
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_e650, plain, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_e650, 0, 0, 0), (true, 0.0));
    }

    /// A weather object: the flag byte at `+0xeb`.
    fn weather(e: &mut Engine, flags: u8) -> u32 {
        let weather = e.mem.alloc(0x100);
        e.mem.set_u8(weather + 0xeb, flags);
        weather
    }

    /// A sky object at the address `Sky::GetInstance` returns.
    fn sky(e: &mut Engine, current: u32, last: u32, percent: f32) -> u32 {
        let sky = e.mem.alloc(0x140);
        e.mem.set_u32(sky + 0x10, current);
        e.mem.set_u32(sky + 0x14, last);
        e.mem.set_f32(sky + 0xf4, percent);
        stub(e, SKY_GET_INSTANCE, sky);
        e.register(SKY_GET_CURRENT_WEATHER, |e, a| {
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.register(SKY_GET_LAST_WEATHER, |e, a| {
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(SKY_GET_WEATHER_PERCENT, |e, a| Ret {
            st0: e.mem.f32(a[0] + 0xf4) as f64,
            ..Ret::default()
        });
        sky
    }

    #[test]
    fn pleasant_and_cloudy_add_the_shares_of_both_weathers() {
        let mut e = engine();
        // Percentage 0.25: the current weather counts 0.25, the last 0.75.
        for (current, last, pleasant, cloudy) in [
            (1u8, 0u8, 0.25, 0.0),
            (0, 1, 0.75, 0.0),
            (1, 1, 1.0, 0.0),
            (2, 2, 0.0, 1.0),
            (3, 2, 0.25, 1.0),
            (0, 0, 0.0, 0.0),
        ] {
            let current_weather = weather(&mut e, current);
            let last_weather = weather(&mut e, last);
            sky(&mut e, current_weather, last_weather, 0.25);
            assert_eq!(
                run(&mut e, 0x0059_e700, 0, 0, 0),
                (true, pleasant),
                "pleasant {current} {last}"
            );
            assert_eq!(
                run(&mut e, 0x0059_e7f0, 0, 0, 0),
                (true, cloudy),
                "cloudy {current} {last}"
            );
        }
        // Without weathers: 0.
        sky(&mut e, 0, 0, 0.25);
        assert_eq!(run(&mut e, 0x0059_e700, 0, 0, 0), (true, 0.0));
        // The flag tests.
        let both = weather(&mut e, 3);
        assert!(fn_0059e7d0(&mut e, Ptr::new(both)));
        assert!(fn_0059e8c0(&mut e, Ptr::new(both)));
        assert!(!fn_0059eb40(&mut e, Ptr::new(both)));
        trace_on(&mut e);
        run(&mut e, 0x0059_e700, 0, 0, 0);
        run(&mut e, 0x0059_e7f0, 0, 0, 0);
        let log = take_log(&mut e);
        assert!(printed(&log, 0x0103_51a4));
        assert!(printed(&log, 0x0103_51bc));
    }

    #[test]
    fn raining_and_snowing_test_the_weather_percentage() {
        let mut e = engine();
        // Interpolated limits: 0.75 for entry 6 (current), 0.25 for entry 7
        // (last); the weather argument tells which.
        let calls = std::rc::Rc::new(std::cell::RefCell::new(Vec::<Vec<u32>>::new()));
        let recorded = calls.clone();
        e.register_double(WEATHER_INTERPOLATE, move |_, a| {
            recorded.borrow_mut().push(a.to_vec());
            Ret {
                st0: if a[1] == 6 { 0.75 } else { 0.25 },
                ..Ret::default()
            }
        });
        stub(&mut e, WEATHER_HAS_FLAG_PRECIPITATION, 1);
        let current = weather(&mut e, 0);
        let last = weather(&mut e, 0);
        // Percentage 0.875: above the current limit (0.75): raining.
        let sky_address = sky(&mut e, current, last, 0.875);
        assert!(fn_0059e950(&mut e, Ptr::new(sky_address)));
        assert_eq!(
            calls.borrow()[0],
            vec![current, 6, 0.5f32.to_bits(), 0.0f32.to_bits()]
        );
        assert_eq!(run(&mut e, 0x0059_e8e0, 0, 0, 0), (true, 1.0));
        // Percentage 0.125: not above the current limit but below the last
        // weather's (0.25): still raining.
        let sky_address = sky(&mut e, current, last, 0.125);
        calls.borrow_mut().clear();
        assert!(fn_0059e950(&mut e, Ptr::new(sky_address)));
        assert_eq!(
            calls.borrow()[1],
            vec![last, 7, 1.0f32.to_bits(), 0.125f32.to_bits()]
        );
        // Percentage 0.5: neither.
        let sky_address = sky(&mut e, current, last, 0.5);
        assert!(!fn_0059e950(&mut e, Ptr::new(sky_address)));
        assert_eq!(run(&mut e, 0x0059_e8e0, 0, 0, 0), (true, 0.0));
        // No precipitation flag: no rain whatever the percentage.
        stub(&mut e, WEATHER_HAS_FLAG_PRECIPITATION, 0);
        let sky_address = sky(&mut e, current, last, 0.875);
        assert!(!fn_0059e950(&mut e, Ptr::new(sky_address)));
        // No weathers.
        stub(&mut e, WEATHER_HAS_FLAG_PRECIPITATION, 1);
        sky(&mut e, 0, 0, 0.875);
        assert_eq!(run(&mut e, 0x0059_e8e0, 0, 0, 0), (true, 0.0));

        // Snow uses flag 8 of the weather byte instead.
        let snowy = weather(&mut e, 8);
        let dry = weather(&mut e, 0);
        let sky_address = sky(&mut e, snowy, dry, 0.875);
        assert!(fn_0059ea80(&mut e, Ptr::new(sky_address)));
        assert_eq!(run(&mut e, 0x0059_ea10, 0, 0, 0), (true, 1.0));
        sky(&mut e, dry, snowy, 0.125);
        assert_eq!(run(&mut e, 0x0059_ea10, 0, 0, 0), (true, 1.0));
        sky(&mut e, dry, snowy, 0.5);
        assert_eq!(run(&mut e, 0x0059_ea10, 0, 0, 0), (true, 0.0));
        sky(&mut e, dry, dry, 0.125);
        assert_eq!(run(&mut e, 0x0059_ea10, 0, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x0059_e8e0, 0, 0, 0);
        run(&mut e, 0x0059_ea10, 0, 0, 0);
        let log = take_log(&mut e);
        assert!(printed(&log, 0x0103_51d4));
        assert!(printed(&log, 0x0103_51ec));
    }

    #[test]
    fn weather_percent_and_current_weather() {
        let mut e = engine();
        use_form_types(&mut e);
        let current = typed_object(&mut e, 0x35);
        sky(&mut e, current, 0, 0.375);
        assert_eq!(run(&mut e, 0x0059_eb60, 0, 0, 0), (true, 0.375));
        // The given WTHR is the current one.
        assert_eq!(run(&mut e, 0x0059_ebb0, 0, current, 0), (true, 1.0));
        let other = typed_object(&mut e, 0x35);
        assert_eq!(run(&mut e, 0x0059_ebb0, 0, other, 0), (true, 0.0));
        // A form of another type counts as no weather, which matches only a
        // sky without one.
        let class = typed_object(&mut e, 7);
        assert_eq!(run(&mut e, 0x0059_ebb0, 0, class, 0), (true, 0.0));
        sky(&mut e, 0, 0, 0.0);
        assert_eq!(run(&mut e, 0x0059_ebb0, 0, class, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_ebb0, 0, 0, 0), (true, 1.0));
    }

    #[test]
    fn attacked_reads_the_actor_byte_and_keeps_the_result_for_others() {
        let mut e = engine();
        let yes = slot_function(&mut e, 0x0f00_0340, 0x0101);
        let no = slot_function(&mut e, 0x0f00_0341, 0x0100);
        let attacked = actor_with(&mut e, &[(VSLOT_ATTACKED, yes)]);
        let calm = actor_with(&mut e, &[(VSLOT_ATTACKED, no)]);
        // Only the low byte counts.
        assert_eq!(run(&mut e, 0x0059_ec30, attacked, 0, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_ec30, calm, 0, 0), (true, 0.0));
        // The result is not zeroed first.
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_ec30, plain, 0, 0), (true, SENTINEL));
        assert_eq!(run(&mut e, 0x0059_ec30, 0, 0, 0), (true, SENTINEL));
    }

    #[test]
    fn is_creature_tests_the_base_form_type() {
        let mut e = engine();
        use_form_types(&mut e);
        let creature = typed_object(&mut e, 0x2b);
        let npc = typed_object(&mut e, 0x2a);
        stub(&mut e, REF_GET_BASE_FORM, creature);
        assert_eq!(run(&mut e, 0x0059_ecc0, 0x2000, 0, 0), (true, 1.0));
        stub(&mut e, REF_GET_BASE_FORM, npc);
        assert_eq!(run(&mut e, 0x0059_ecc0, 0x2000, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_ecc0, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn should_attack_goes_through_the_combat_manager_then_the_actor() {
        let mut e = engine();
        use_form_types(&mut e);
        let prepare = slot_function(&mut e, 0x0f00_0350, 0);
        let actor = actor_with(&mut e, &[(VSLOT_ACTOR_PREPARE_ATTACK_CHECK, prepare)]);
        let combat_yes = slot_function(&mut e, 0x0f00_0351, 1);
        let target = object(&mut e, &[(VSLOT_TARGET_COMBAT_CHECK, combat_yes)]);
        e.mem.set_u8(target + 4, 0x3b);
        e.set_global::<u32>(COMBAT_MANAGER, 0x7770);
        e.register_double(ACTOR_GET_SHOULD_ATTACK_ACTOR, move |e, a| {
            assert_eq!((a[0], a[1], a[2], a[4]), (actor, target, 0, 0));
            // The out structure starts zeroed.
            assert!((0..4).all(|i| e.mem.u32(a[3] + 4 * i) == 0));
            true.into_ret()
        });
        stub(&mut e, TARGET_GET_FLAG, 0);
        stub(&mut e, COMBAT_MANAGER_CHECK, 0);
        // The flag is clear: the actor decides.
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_ed30, actor, target, 0), (true, 100.0));
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, DEBUG_PRINT), vec![vec![0x0103_5278, 100]]);
        assert_eq!(calls_to(&log, ACTOR_GET_SHOULD_ATTACK_ACTOR).len(), 1);

        // The flag is set and the combat manager says yes: back at once, with
        // neither a result nor a debug line.
        stub(&mut e, TARGET_GET_FLAG, 1);
        e.register_double(COMBAT_MANAGER_CHECK, move |_, a| {
            assert_eq!(a, [0x7770, actor, target]);
            true.into_ret()
        });
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_ed30, actor, target, 0), (true, SENTINEL));
        let log = take_log(&mut e);
        assert!(calls_to(&log, DEBUG_PRINT).is_empty());
        assert!(calls_to(&log, ACTOR_GET_SHOULD_ATTACK_ACTOR).is_empty());

        // The combat manager says no: the actor decides again; a "no" gives 0.
        stub(&mut e, COMBAT_MANAGER_CHECK, 0);
        assert_eq!(run(&mut e, 0x0059_ed30, actor, target, 0), (true, 100.0));
        stub(&mut e, ACTOR_GET_SHOULD_ATTACK_ACTOR, 0);
        assert_eq!(run(&mut e, 0x0059_ed30, actor, target, 0), (true, 0.0));

        // A target of another type, or a non-actor: the result is left and the
        // debug line shows 0.
        let wrong_target = typed_object(&mut e, 0x3a);
        trace_on(&mut e);
        assert_eq!(
            run(&mut e, 0x0059_ed30, actor, wrong_target, 0),
            (true, SENTINEL)
        );
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x0059_ed30, plain, target, 0), (true, SENTINEL));
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_5278, 0], vec![0x0103_5278, 0]]
        );
    }

    #[test]
    fn in_same_cell_compares_the_parent_cells() {
        let mut e = engine();
        use_form_types(&mut e);
        e.register(REF_GET_PARENT_CELL, |e, a| {
            e.mem.u32(a[0] + 0x40).into_ret()
        });
        let reference = typed_object(&mut e, 0x3b);
        let same_cell = typed_object(&mut e, 0x3b);
        let other_cell = typed_object(&mut e, 0x3a);
        let not_a_reference = typed_object(&mut e, 0x28);
        let extra_type = typed_object(&mut e, 0x69);
        for object in [reference, same_cell, extra_type, not_a_reference] {
            e.mem.set_u32(object + 0x40, 0xce11);
        }
        e.mem.set_u32(other_cell + 0x40, 0xce12);
        assert_eq!(
            run(&mut e, 0x0059_ee80, reference, same_cell, 0),
            (true, 1.0)
        );
        assert_eq!(
            run(&mut e, 0x0059_ee80, reference, extra_type, 0),
            (true, 1.0)
        );
        assert_eq!(
            run(&mut e, 0x0059_ee80, reference, other_cell, 0),
            (true, 0.0)
        );
        // The other form must be a reference type, and neither side may be null
        // or in no cell.
        assert_eq!(
            run(&mut e, 0x0059_ee80, reference, not_a_reference, 0),
            (true, 0.0)
        );
        assert_eq!(run(&mut e, 0x0059_ee80, reference, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_ee80, 0, same_cell, 0), (true, 0.0));
        let no_cell = typed_object(&mut e, 0x3b);
        assert_eq!(run(&mut e, 0x0059_ee80, no_cell, no_cell, 0), (true, 0.0));
    }

    fn run_in_cell(e: &mut Engine, reference: u32, cell: u32, other: u32) -> (bool, f64) {
        run(e, 0x0059_ef60, reference, cell, other)
    }

    #[test]
    fn in_cell_compares_name_prefixes_and_caches_the_answer() {
        let mut e = engine();
        use_form_types(&mut e);
        e.register(REF_GET_PARENT_CELL, |e, a| {
            e.mem.u32(a[0] + 0x40).into_ret()
        });
        let cell_name = slot_function(&mut e, 0x0f00_0360, 0xa000);
        let parent_name = slot_function(&mut e, 0x0f00_0361, 0xa100);
        let cell = object(&mut e, &[(VSLOT_FORM_GET_NAME, cell_name)]);
        e.mem.set_u8(cell + 4, 0x39);
        let parent = object(&mut e, &[(VSLOT_FORM_GET_NAME, parent_name)]);
        let reference = typed_object(&mut e, 0x3b);
        e.mem.set_u32(reference + 0x40, parent);
        stub(&mut e, CELL_GET_NAME_LENGTH, 6);
        let compares = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let counted = compares.clone();
        e.register_double(STRING_COMPARE_N_NO_CASE, move |_, a| {
            assert_eq!(a, [0xa100, 0xa000, 6]);
            counted.set(counted.get() + 1);
            0u32.into_ret()
        });
        // The reference itself (second parameter null): equal prefixes.
        assert_eq!(run_in_cell(&mut e, reference, cell, 0), (true, 1.0));
        assert_eq!(compares.get(), 1);
        assert_eq!(e.global::<u32>(IN_CELL_CACHE_REFERENCE), reference);
        assert_eq!(e.global::<u32>(IN_CELL_CACHE_CELL), cell);
        assert_eq!(e.global::<f32>(IN_CELL_CACHE_RESULT), 1.0);
        // The same pair again: the cached answer, no comparison.
        stub(&mut e, STRING_COMPARE_N_NO_CASE, 1);
        assert_eq!(run_in_cell(&mut e, reference, cell, 0), (true, 1.0));
        // Another pair, now with different prefixes; the second parameter
        // supplies the reference.
        let other_reference = typed_object(&mut e, 0x3c);
        e.mem.set_u32(other_reference + 0x40, parent);
        assert_eq!(run_in_cell(&mut e, 0, cell, other_reference), (true, 0.0));
        assert_eq!(e.global::<u32>(IN_CELL_CACHE_REFERENCE), other_reference);
        assert_eq!(e.global::<f32>(IN_CELL_CACHE_RESULT), 0.0);
        // A form that is not a cell, a reference without a parent cell, and no
        // reference at all: 0, and the last has no cache effect.
        stub(&mut e, STRING_COMPARE_N_NO_CASE, 0);
        let not_a_cell = typed_object(&mut e, 7);
        assert_eq!(run_in_cell(&mut e, reference, not_a_cell, 0), (true, 0.0));
        let orphan = typed_object(&mut e, 0x3b);
        assert_eq!(run_in_cell(&mut e, orphan, cell, 0), (true, 0.0));
        let cached_before = e.global::<u32>(IN_CELL_CACHE_REFERENCE);
        assert_eq!(run_in_cell(&mut e, 0, cell, 0), (true, 0.0));
        assert_eq!(e.global::<u32>(IN_CELL_CACHE_REFERENCE), cached_before);
    }

    #[test]
    fn in_worldspace_compares_the_reference_worldspace() {
        let mut e = engine();
        use_form_types(&mut e);
        let worldspace = typed_object(&mut e, 0x41);
        let other_worldspace = typed_object(&mut e, 0x41);
        e.register_double(REF_GET_WORLDSPACE, move |_, a| {
            if a[0] == 0x2000 {
                worldspace.into_ret()
            } else {
                other_worldspace.into_ret()
            }
        });
        // The calling reference, or the placed reference of the third argument.
        assert_eq!(run(&mut e, 0x0059_f0c0, 0x2000, worldspace, 0), (true, 1.0));
        assert_eq!(
            run(&mut e, 0x0059_f0c0, 0x2000, other_worldspace, 0),
            (true, 0.0)
        );
        let placed = typed_object(&mut e, 0x3b);
        assert_eq!(
            run(&mut e, 0x0059_f0c0, 0x2000, worldspace, placed),
            (true, 0.0)
        );
        assert_eq!(
            run(&mut e, 0x0059_f0c0, 0x2000, other_worldspace, placed),
            (true, 1.0)
        );
        // Not a worldspace: 0.
        let class = typed_object(&mut e, 7);
        assert_eq!(run(&mut e, 0x0059_f0c0, 0x2000, class, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f0c0, 0x2000, 0, 0), (true, 0.0));
    }

    #[test]
    fn is_class_and_is_race_compare_the_npc_forms() {
        let mut e = engine();
        use_form_types(&mut e);
        let npc = typed_object(&mut e, 0x2a);
        let creature = typed_object(&mut e, 0x2b);
        let class = typed_object(&mut e, 7);
        let other_class = typed_object(&mut e, 7);
        let race = typed_object(&mut e, 0xc);
        let other_race = typed_object(&mut e, 0xc);
        stub(&mut e, REF_GET_BASE_FORM, npc);
        stub(&mut e, NPC_GET_CLASS, class);
        stub(&mut e, NPC_GET_RACE, race);
        assert_eq!(run(&mut e, 0x0059_f180, 0x2000, class, 0), (true, 1.0));
        assert_eq!(
            run(&mut e, 0x0059_f180, 0x2000, other_class, 0),
            (true, 0.0)
        );
        // A form of the wrong type, or none.
        assert_eq!(run(&mut e, 0x0059_f180, 0x2000, race, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f180, 0x2000, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f240, 0x2000, race, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_f240, 0x2000, other_race, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f240, 0x2000, class, 0), (true, 0.0));
        // The base form is not an NPC_, or there is no reference.
        stub(&mut e, REF_GET_BASE_FORM, creature);
        assert_eq!(run(&mut e, 0x0059_f180, 0x2000, class, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f240, 0x2000, race, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f240, 0, race, 0), (true, 0.0));
    }

    #[test]
    fn creature_type_is_a_sign_extended_byte() {
        let mut e = engine();
        use_form_types(&mut e);
        let creature = typed_object(&mut e, 0x2b);
        e.mem.set_u8(creature + 0x12c, 0xff);
        stub(&mut e, REF_GET_BASE_FORM, creature);
        assert_eq!(fn_0059f3a0(&mut e, Ptr::new(creature)), 0xff);
        // The byte 0xff reads as -1 and equals the parameter -1 (0xffff_ffff).
        assert_eq!(
            run(&mut e, 0x0059_f300, 0x2000, 0xffff_ffff, 0),
            (true, 1.0)
        );
        assert_eq!(run(&mut e, 0x0059_f300, 0x2000, 0xff, 0), (true, 0.0));
        e.mem.set_u8(creature + 0x12c, 4);
        assert_eq!(run(&mut e, 0x0059_f300, 0x2000, 4, 0), (true, 1.0));
        // Not a creature, or no reference.
        let npc = typed_object(&mut e, 0x2a);
        stub(&mut e, REF_GET_BASE_FORM, npc);
        assert_eq!(run(&mut e, 0x0059_f300, 0x2000, 4, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f300, 0, 4, 0), (true, 0.0));
    }

    #[test]
    fn is_sex_compares_the_npc_sex() {
        let mut e = engine();
        use_form_types(&mut e);
        let npc = typed_object(&mut e, 0x2a);
        stub(&mut e, REF_GET_BASE_FORM, npc);
        stub(&mut e, NPC_GET_SEX, 1);
        assert_eq!(run(&mut e, 0x0059_f3c0, 0x2000, 1, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_f3c0, 0x2000, 0, 0), (true, 0.0));
        let creature = typed_object(&mut e, 0x2b);
        stub(&mut e, REF_GET_BASE_FORM, creature);
        assert_eq!(run(&mut e, 0x0059_f3c0, 0x2000, 1, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f3c0, 0, 1, 0), (true, 0.0));
    }

    #[test]
    fn voice_type_comes_from_the_base_form_and_is_cached() {
        let mut e = engine();
        use_form_types(&mut e);
        // An NPC_ base form with a component at +0x30 whose vtable slot 0x68
        // gives the voice type.
        let voice = slot_function(&mut e, 0x0f00_0370, 0x77);
        let component_vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(component_vtable + VSLOT_BASE_COMPONENT_VOICE_TYPE, voice);
        let npc = typed_object(&mut e, 0x2a);
        e.mem.set_u32(npc + 0x30, component_vtable);
        stub(&mut e, REF_GET_BASE_FORM, npc);
        assert_eq!(run(&mut e, 0x0059_f450, 0x2000, 0x77, 0), (true, 1.0));
        assert_eq!(e.global::<u32>(VOICE_TYPE_CACHE_REFERENCE), 0x2000);
        assert_eq!(e.global::<u32>(VOICE_TYPE_CACHE_VOICE), 0x77);
        assert_eq!(run(&mut e, 0x0059_f450, 0x2000, 0x78, 0), (true, 0.0));
        // The cache answers for the same reference even if the form changed.
        let other_voice = typed_object(&mut e, 0x2a);
        let other_vtable = e.mem.alloc(0x100);
        let other_function = slot_function(&mut e, 0x0f00_0371, 0x99);
        e.mem.set_u32(
            other_vtable + VSLOT_BASE_COMPONENT_VOICE_TYPE,
            other_function,
        );
        e.mem.set_u32(other_voice + 0x30, other_vtable);
        stub(&mut e, REF_GET_BASE_FORM, other_voice);
        assert_eq!(run(&mut e, 0x0059_f450, 0x2000, 0x77, 0), (true, 1.0));
        // A different reference is looked up again.
        assert_eq!(run(&mut e, 0x0059_f450, 0x2004, 0x99, 0), (true, 1.0));
        // The form type whose voice type is at +0x94.
        let source = typed_object(&mut e, 0x16);
        stub(&mut e, REF_GET_BASE_FORM, source);
        stub(&mut e, FORM_GET_VOICE_TYPE, 0x55);
        assert_eq!(run(&mut e, 0x0059_f450, 0x2008, 0x55, 0), (true, 1.0));
        // Another type: voice type 0, which is cached too.
        let weapon = typed_object(&mut e, 0x28);
        stub(&mut e, REF_GET_BASE_FORM, weapon);
        assert_eq!(run(&mut e, 0x0059_f450, 0x200c, 0, 0), (true, 1.0));
        assert_eq!(e.global::<u32>(VOICE_TYPE_CACHE_REFERENCE), 0x200c);
        // No reference: nothing happens (the cache stays).
        assert_eq!(run(&mut e, 0x0059_f450, 0, 0, 0), (true, 0.0));
        assert_eq!(e.global::<u32>(VOICE_TYPE_CACHE_REFERENCE), 0x200c);
    }

    #[test]
    fn playable_race_tests_the_race_flag_and_caches_the_answer() {
        let mut e = engine();
        use_form_types(&mut e);
        let npc = typed_object(&mut e, 0x2a);
        let race = typed_object(&mut e, 0xc);
        stub(&mut e, REF_GET_BASE_FORM, npc);
        stub(&mut e, NPC_GET_RACE, race);
        e.register_double(RACE_IS_PLAYABLE, move |_, a| {
            assert_eq!(a, [race]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_f540, 0x2000, 0, 0), (true, 1.0));
        assert_eq!(e.global::<u32>(PLAYABLE_RACE_CACHE_REFERENCE), 0x2000);
        assert_eq!(e.global::<f32>(PLAYABLE_RACE_CACHE_RESULT), 1.0);
        // The cache answers for the same reference.
        stub(&mut e, RACE_IS_PLAYABLE, 0);
        assert_eq!(run(&mut e, 0x0059_f540, 0x2000, 0, 0), (true, 1.0));
        // A new reference whose race is not playable.
        assert_eq!(run(&mut e, 0x0059_f540, 0x2004, 0, 0), (true, 0.0));
        assert_eq!(e.global::<f32>(PLAYABLE_RACE_CACHE_RESULT), 0.0);
        // No race, or not an NPC_: 0.
        stub(&mut e, RACE_IS_PLAYABLE, 1);
        stub(&mut e, NPC_GET_RACE, 0);
        assert_eq!(run(&mut e, 0x0059_f540, 0x2008, 0, 0), (true, 0.0));
        let creature = typed_object(&mut e, 0x2b);
        stub(&mut e, REF_GET_BASE_FORM, creature);
        stub(&mut e, NPC_GET_RACE, race);
        assert_eq!(run(&mut e, 0x0059_f540, 0x200c, 0, 0), (true, 0.0));
        // The debug line is printed on the cached path too.
        trace_on(&mut e);
        run(&mut e, 0x0059_f540, 0x200c, 0, 0);
        let log = take_log(&mut e);
        assert!(printed(&log, 0x0103_5348));
    }

    // ----- the third session: 0059f610 to 005a0f10 -----

    fn put_u32(e: &mut Engine, addr: u32, value: u32) {
        e.map(addr & !0xfff, 0x2000);
        e.mem.set_u32(addr, value);
    }

    fn put_f64(e: &mut Engine, addr: u32, value: f64) {
        e.map(addr & !0xfff, 0x2000);
        e.mem.set_f64(addr, value);
    }

    /// A form-like object (type byte at `+4`) with vtable slots.
    fn typed_with(e: &mut Engine, form_type: u8, slots: &[(u32, u32)]) -> u32 {
        let form = object(e, slots);
        e.mem.set_u8(form + 4, form_type);
        form
    }

    #[test]
    fn race_playable_is_bit_one_of_the_flags() {
        let mut e = engine();
        let race = e.mem.alloc(0x100);
        e.mem.set_u32(race + 0x70, 3);
        assert!(e.call(0x0059_f610, &args![race]).bool());
        e.mem.set_u32(race + 0x70, 0xffff_fffe);
        assert!(!e.call(0x0059_f610, &args![race]).bool());
    }

    #[test]
    fn in_faction_asks_the_actor_and_caches_the_answer() {
        let mut e = engine();
        use_form_types(&mut e);
        let actor = actor(&mut e);
        let faction = typed_object(&mut e, 8);
        let other = typed_object(&mut e, 8);
        e.register_double(ACTOR_IS_IN_FACTION, move |_, a| {
            assert_eq!(a[0], actor);
            (a[1] == faction).into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_f630, actor, faction, 0), (true, 1.0));
        assert_eq!(e.global::<u32>(IN_FACTION_CACHE_REFERENCE), actor);
        assert_eq!(e.global::<u32>(IN_FACTION_CACHE_FACTION), faction);
        assert_eq!(e.global::<f32>(IN_FACTION_CACHE_RESULT), 1.0);
        // The same (reference, faction) is answered from the cache.
        e.register_double(ACTOR_IS_IN_FACTION, |_, _| panic!("the cache answers"));
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_f630, actor, faction, 0), (true, 1.0));
        let log = take_log(&mut e);
        assert!(printed(&log, 0x0103_5364));
        // Another faction asks again.
        e.register_double(ACTOR_IS_IN_FACTION, move |_, a| {
            (a[1] == faction).into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_f630, actor, other, 0), (true, 0.0));
        assert_eq!(e.global::<u32>(IN_FACTION_CACHE_FACTION), other);
        // A form that is no faction asks with a null faction.
        let weapon = typed_object(&mut e, 0x28);
        e.register_double(ACTOR_IS_IN_FACTION, move |_, a| {
            assert_eq!(a[1], 0);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_f630, actor, weapon, 0), (true, 1.0));
        // A reference that is no actor: 0, no debug line, no cache.
        let plain = non_actor(&mut e);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_f630, plain, faction, 0), (true, 0.0));
        assert!(!printed(&take_log(&mut e), 0x0103_5364));
        assert_eq!(e.global::<u32>(IN_FACTION_CACHE_REFERENCE), actor);
    }

    #[test]
    fn the_player_versions_use_the_player_as_the_reference() {
        let mut e = engine();
        use_form_types(&mut e);
        let player = actor(&mut e);
        put_u32(&mut e, PLAYER, player);
        let npc = typed_object(&mut e, 0x2a);
        e.register_double(REF_GET_BASE_FORM, move |_, a| {
            assert_eq!(a[0], player);
            npc.into_ret()
        });
        // GetPCIsClass, GetPCIsRace, GetPCIsSex: the reference argument
        // (0x9999) is ignored.
        let class = typed_object(&mut e, 7);
        stub(&mut e, NPC_GET_CLASS, class);
        assert_eq!(run(&mut e, 0x0059_f730, 0x9999, class, 0), (true, 1.0));
        let race = typed_object(&mut e, 0xc);
        stub(&mut e, NPC_GET_RACE, race);
        assert_eq!(run(&mut e, 0x0059_f750, 0x9999, race, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_f750, 0x9999, class, 0), (true, 0.0));
        stub(&mut e, NPC_GET_SEX, 1);
        assert_eq!(run(&mut e, 0x0059_f770, 0x9999, 1, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_f770, 0x9999, 0, 0), (true, 0.0));
        // GetPCInFaction.
        let faction = typed_object(&mut e, 8);
        e.register_double(ACTOR_IS_IN_FACTION, move |_, a| {
            assert_eq!(a[0], player);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_f790, 0x9999, faction, 0), (true, 1.0));
        assert_eq!(e.global::<u32>(IN_FACTION_CACHE_REFERENCE), player);
    }

    #[test]
    fn is_id_compares_the_original_base_form_and_caches_it() {
        let mut e = engine();
        let form = bound_object(&mut e);
        stub(&mut e, REF_GET_EXTRA_DATA_LIST, 0x5000);
        stub(&mut e, EXTRA_DATA_LIST_GET_LEV_CREA_ORIGINAL_BASE, 0);
        stub(&mut e, REF_GET_BASE_FORM, form);
        // No leveled-creature base: the base form is compared.
        assert_eq!(run(&mut e, 0x0059_f7b0, 0x2000, form, 0), (true, 1.0));
        assert_eq!(e.global::<u32>(IS_ID_CACHE_REFERENCE), 0x2000);
        assert_eq!(e.global::<u32>(IS_ID_CACHE_BASE), form);
        // The same reference is answered from the cache.
        stub(&mut e, REF_GET_BASE_FORM, 0x7777);
        assert_eq!(run(&mut e, 0x0059_f7b0, 0x2000, form, 0), (true, 1.0));
        // A leveled creature's original base wins over the base form.
        let original = bound_object(&mut e);
        stub(&mut e, EXTRA_DATA_LIST_GET_LEV_CREA_ORIGINAL_BASE, original);
        assert_eq!(run(&mut e, 0x0059_f7b0, 0x2004, original, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_f7b0, 0x2008, form, 0), (true, 0.0));
        // A form that is no bound object, or no reference: 0.
        let plain = object(&mut e, &[(VSLOT_IS_BOUND_OBJECT, 0x0f10_0001)]);
        stub(&mut e, 0x0f10_0001, 0);
        assert_eq!(run(&mut e, 0x0059_f7b0, 0x200c, plain, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f7b0, 0, form, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f7b0, 0x2010, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x0059_f7b0, 0x2010, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_537c));
    }

    #[test]
    fn is_in_list_looks_up_the_original_base_in_the_list() {
        let mut e = engine();
        let list = e.mem.alloc(0x200);
        stub(&mut e, REF_GET_EXTRA_DATA_LIST, 0x5000);
        stub(&mut e, EXTRA_DATA_LIST_GET_LEV_CREA_ORIGINAL_BASE, 0);
        stub(&mut e, REF_GET_BASE_FORM, 0x7000);
        e.register_double(LIST_CONTAINS, move |e, a| {
            assert_eq!(a[0], list + 0x18);
            (e.mem.u32(a[1]) == 0x7000).into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_f890, 0x2000, list, 0), (true, 1.0));
        stub(&mut e, EXTRA_DATA_LIST_GET_LEV_CREA_ORIGINAL_BASE, 0x7100);
        assert_eq!(run(&mut e, 0x0059_f890, 0x2000, list, 0), (true, 0.0));
        // Nothing to look up, no list or no reference: the list is not asked.
        e.register_double(LIST_CONTAINS, |_, _| panic!("not asked"));
        stub(&mut e, EXTRA_DATA_LIST_GET_LEV_CREA_ORIGINAL_BASE, 0);
        stub(&mut e, REF_GET_BASE_FORM, 0);
        assert_eq!(run(&mut e, 0x0059_f890, 0x2000, list, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f890, 0x2000, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f890, 0, list, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x0059_f890, 0, list, 0);
        assert!(printed(&take_log(&mut e), 0x0103_5390));
    }

    #[test]
    fn is_child_asks_the_reference_with_zero() {
        let mut e = engine();
        e.register(0x0f10_0010, |_, a| (a[1] == 0).into_ret());
        let child = object(&mut e, &[(VSLOT_REF_IS_CHILD, 0x0f10_0010)]);
        assert_eq!(run(&mut e, 0x0059_f940, child, 0, 0), (true, 1.0));
        stub(&mut e, 0x0f10_0011, 0);
        let adult = object(&mut e, &[(VSLOT_REF_IS_CHILD, 0x0f10_0011)]);
        assert_eq!(run(&mut e, 0x0059_f940, adult, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_f940, 0, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x0059_f940, 0, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_53a4));
    }

    #[test]
    fn is_used_item_compares_with_the_used_item_or_its_list() {
        let mut e = engine();
        use_form_types(&mut e);
        stub(&mut e, 0x0f10_0020, 1);
        stub(&mut e, 0x0f10_0021, 0x4444);
        let slots = [
            (VSLOT_FORM_COMPARES_USED_ITEM, 0x0f10_0020),
            (VSLOT_FORM_GET_NAME, 0x0f10_0021),
        ];
        let item = typed_with(&mut e, 0x28, &slots);
        let other = typed_with(&mut e, 0x28, &slots);
        stub(&mut e, IDLE_MANAGER_GET_USED_ITEM, item);
        assert_eq!(run(&mut e, 0x0059_f9b0, 0, item, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_f9b0, 0, other, 0), (true, 0.0));
        // A form list holds the used item or not.
        let list = typed_with(&mut e, 0x55, &slots);
        e.register_double(LIST_CONTAINS, move |e, a| {
            assert_eq!(a[0], list + 0x18);
            (e.mem.u32(a[1]) == item).into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_f9b0, 0, list, 0), (true, 1.0));
        stub(&mut e, IDLE_MANAGER_GET_USED_ITEM, other);
        assert_eq!(run(&mut e, 0x0059_f9b0, 0, list, 0), (true, 0.0));
        // The debug line names the form, or UNKNOWN.
        trace_on(&mut e);
        run(&mut e, 0x0059_f9b0, 0, item, 0);
        run(&mut e, 0x0059_f9b0, 0, 0, 0);
        let log = take_log(&mut e);
        let zero = words(0.0);
        let first = words(0.0);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![
                vec![0x0103_53b8, 0x4444, first[0], first[1]],
                vec![0x0103_53b8, UNKNOWN_NAME, zero[0], zero[1]],
            ]
        );
    }

    #[test]
    fn used_item_type_compares_the_form_type_of_the_used_item() {
        let mut e = engine();
        use_form_types(&mut e);
        let item = typed_object(&mut e, 0x28);
        stub(&mut e, IDLE_MANAGER_GET_USED_ITEM, item);
        assert_eq!(run(&mut e, 0x0059_fa90, 0, 0x28, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x0059_fa90, 0, 0x29, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_fa90, 0, 0, 0), (true, 0.0));
        stub(&mut e, IDLE_MANAGER_GET_USED_ITEM, 0);
        assert_eq!(run(&mut e, 0x0059_fa90, 0, 0x28, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x0059_fa90, 0, 0x28, 0);
        assert!(printed(&take_log(&mut e), 0x0103_53d4));
    }

    #[test]
    fn used_item_level_is_a_signed_int() {
        let mut e = engine();
        stub(&mut e, IDLE_MANAGER_GET_USED_ITEM_LEVEL, (-3i32) as u32);
        assert_eq!(run(&mut e, 0x0059_fb00, 0, 0, 0), (true, -3.0));
        trace_on(&mut e);
        run(&mut e, 0x0059_fb00, 0, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_53f0));
    }

    #[test]
    fn used_item_activate_is_the_idle_managers_flag() {
        let mut e = engine();
        stub(&mut e, IDLE_MANAGER_GET_USED_ITEM_ACTIVATE, 1);
        assert_eq!(run(&mut e, 0x0059_fb50, 0, 0, 0), (true, 1.0));
        stub(&mut e, IDLE_MANAGER_GET_USED_ITEM_ACTIVATE, 0);
        assert_eq!(run(&mut e, 0x0059_fb50, 0, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x0059_fb50, 0, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_540c));
    }

    #[test]
    fn is_ref_accepts_placed_references_only() {
        let mut e = engine();
        use_form_types(&mut e);
        for (form_type, expected) in [
            (0x39u8, 0.0),
            (0x3a, 1.0),
            (0x3b, 1.0),
            (0x40, 1.0),
            (0x41, 0.0),
            (0x69, 1.0),
        ] {
            let form = typed_object(&mut e, form_type);
            assert_eq!(
                run(&mut e, 0x0059_fbb0, form, form, 0),
                (true, expected),
                "type {form_type:#x}"
            );
        }
        let placed = typed_object(&mut e, 0x3b);
        let reference = typed_object(&mut e, 0x3b);
        assert_eq!(run(&mut e, 0x0059_fbb0, reference, placed, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_fbb0, 0, placed, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_fbb0, reference, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x0059_fbb0, reference, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_542c));
    }

    #[test]
    fn faction_rank_is_minus_one_without_a_faction() {
        let mut e = engine();
        use_form_types(&mut e);
        let actor = actor(&mut e);
        let faction = typed_object(&mut e, 8);
        stub(&mut e, ACTOR_GET_BASE_FORM, 0x6000);
        e.register_double(ACTOR_GET_FACTION_RANK, move |_, a| {
            assert_eq!(a, [actor, faction, 0]);
            3u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_fc50, actor, faction, 0), (true, 3.0));
        // The player is told apart.
        put_u32(&mut e, PLAYER, actor);
        e.register_double(ACTOR_GET_FACTION_RANK, move |_, a| {
            assert_eq!(a, [actor, faction, 1]);
            ((-1i32) as u32).into_ret()
        });
        assert_eq!(run(&mut e, 0x0059_fc50, actor, faction, 0), (true, -1.0));
        // Not a faction, or no form: the rank is not asked, -1.0 stays.
        e.register_double(ACTOR_GET_FACTION_RANK, |_, _| panic!("not asked"));
        let weapon = typed_object(&mut e, 0x28);
        assert_eq!(run(&mut e, 0x0059_fc50, actor, weapon, 0), (true, -1.0));
        assert_eq!(run(&mut e, 0x0059_fc50, actor, 0, 0), (true, -1.0));
        // Not an actor: -1.0 and no debug line.
        let plain = non_actor(&mut e);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_fc50, plain, faction, 0), (true, -1.0));
        assert!(!printed(&take_log(&mut e), 0x0103_5440));
        trace_on(&mut e);
        run(&mut e, 0x0059_fc50, actor, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_5440));
    }

    #[test]
    fn global_value_reads_a_global_form() {
        let mut e = engine();
        use_form_types(&mut e);
        let global = typed_object(&mut e, 6);
        e.register_double(GLOBAL_GET_VALUE, move |_, a| {
            assert_eq!(a, [global]);
            Ret {
                st0: 12.5,
                ..Ret::default()
            }
        });
        assert_eq!(run(&mut e, 0x0059_fd30, 0, global, 0), (true, 12.5));
        let other = typed_object(&mut e, 8);
        assert_eq!(run(&mut e, 0x0059_fd30, 0, other, 0), (true, -1.0));
        assert_eq!(run(&mut e, 0x0059_fd30, 0, 0, 0), (true, -1.0));
        trace_on(&mut e);
        run(&mut e, 0x0059_fd30, 0, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_5458));
    }

    #[test]
    fn disposition_asks_the_actor_and_caches_the_answer() {
        let mut e = engine();
        let target = actor(&mut e);
        e.register_double(0x0f10_0030, move |_, a| {
            assert_eq!(a[1], target);
            assert_eq!(a[2], 0);
            25u32.into_ret()
        });
        let actor = actor_with(&mut e, &[(VSLOT_ACTOR_DISPOSITION_TOWARD, 0x0f10_0030)]);
        assert_eq!(run(&mut e, 0x0059_fdb0, actor, target, 0), (true, 25.0));
        assert_eq!(e.global::<u32>(DISPOSITION_CACHE_ACTOR), actor);
        assert_eq!(e.global::<u32>(DISPOSITION_CACHE_TARGET), target);
        assert_eq!(e.global::<f32>(DISPOSITION_CACHE_RESULT), 25.0);
        // The cache answers (the virtual is not called again).
        e.register_double(0x0f10_0030, |_, _| panic!("the cache answers"));
        assert_eq!(run(&mut e, 0x0059_fdb0, actor, target, 0), (true, 25.0));
        // The debug line names the target first, then the actor.
        e.register(REF_GET_NAME, |_, a| (0x100 + a[0]).into_ret());
        trace_on(&mut e);
        run(&mut e, 0x0059_fdb0, actor, target, 0);
        let log = take_log(&mut e);
        let names: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == REF_GET_NAME)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(names, vec![target, actor]);
        let value = words(25.0);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![
                0x0103_5470,
                0x100 + actor,
                0x100 + target,
                value[0],
                value[1]
            ]]
        );
        // Both must be actors; otherwise 0 and no line.
        let plain = non_actor(&mut e);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_fdb0, plain, target, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_fdb0, actor, plain, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x0059_fdb0, actor, 0, 0), (true, 0.0));
        assert!(!printed(&take_log(&mut e), 0x0103_5470));
    }

    #[test]
    fn unconscious_prints_which_it_is() {
        let mut e = engine();
        let actor = actor(&mut e);
        e.register(REF_GET_NAME, |_, _| 0x1234u32.into_ret());
        stub(&mut e, ACTOR_IS_UNCONSCIOUS, 1);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_fee0, actor, 0, 0), (true, 1.0));
        assert_eq!(
            calls_to(&take_log(&mut e), DEBUG_PRINT),
            vec![vec![0x0103_54ac, 0x1234]]
        );
        stub(&mut e, ACTOR_IS_UNCONSCIOUS, 0);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_fee0, actor, 0, 0), (true, 0.0));
        assert_eq!(
            calls_to(&take_log(&mut e), DEBUG_PRINT),
            vec![vec![0x0103_5494, 0x1234]]
        );
        // Not an actor: 0, no line.
        let plain = non_actor(&mut e);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_fee0, plain, 0, 0), (true, 0.0));
        assert!(calls_to(&take_log(&mut e), DEBUG_PRINT).is_empty());
    }

    #[test]
    fn restrained_prints_which_it_is() {
        let mut e = engine();
        let actor = actor(&mut e);
        e.register(REF_GET_NAME, |_, _| 0x1234u32.into_ret());
        stub(&mut e, ACTOR_IS_RESTRAINED, 1);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_ff90, actor, 0, 0), (true, 1.0));
        assert_eq!(
            calls_to(&take_log(&mut e), DEBUG_PRINT),
            vec![vec![0x0103_54d8, 0x1234]]
        );
        stub(&mut e, ACTOR_IS_RESTRAINED, 0);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_ff90, actor, 0, 0), (true, 0.0));
        assert_eq!(
            calls_to(&take_log(&mut e), DEBUG_PRINT),
            vec![vec![0x0103_54c0, 0x1234]]
        );
        let plain = non_actor(&mut e);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x0059_ff90, plain, 0, 0), (true, 0.0));
        assert!(calls_to(&take_log(&mut e), DEBUG_PRINT).is_empty());
    }

    #[test]
    fn random_percent_rolls_up_to_100_unsigned() {
        let mut e = engine();
        stub(&mut e, RANDOM_GET_INSTANCE, 0x6000);
        e.register_double(RANDOM_UNSIGNED_INT, |_, a| {
            assert_eq!(a, [0x6000, 100]);
            0x8000_0000u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_0040, 0, 0, 0), (true, 2147483648.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_0040, 0, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_54ec));
    }

    #[test]
    fn random_roll_goes_through_the_generator_singleton() {
        let mut e = engine();
        stub(&mut e, RANDOM_GET_INSTANCE, 0x6000);
        e.register_double(RANDOM_UNSIGNED_INT, |_, a| {
            assert_eq!(a[0], 0x6000);
            (a[1] + 1).into_ret()
        });
        assert_eq!(e.call(0x005a_00a0, &args![33u32]).u32(), 34);
    }

    #[test]
    fn level_is_a_u16() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, ACTOR_GET_LEVEL, 0x1234_0007);
        assert_eq!(run(&mut e, 0x005a_00c0, actor, 0, 0), (true, 7.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_00c0, plain, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_00c0, 0, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_5508));
    }

    #[test]
    fn armor_rating_asks_the_subobject_at_0xa4() {
        let mut e = engine();
        let actor = actor(&mut e);
        let subobject_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(subobject_vtable + 0xc, 0x0f10_0040);
        e.mem.set_u32(actor + 0xa4, subobject_vtable);
        e.register_double(0x0f10_0040, move |_, a| {
            assert_eq!(a, [actor + 0xa4, 0x12]);
            Ret {
                st0: 42.5,
                ..Ret::default()
            }
        });
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x005a_0150, actor, 0, 0), (true, 42.5));
        assert!(printed(&take_log(&mut e), 0x0103_551c));
        // Not an actor: 0, no line.
        let plain = non_actor(&mut e);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x005a_0150, plain, 0, 0), (true, 0.0));
        assert!(!printed(&take_log(&mut e), 0x0103_551c));
    }

    #[test]
    fn dead_count_asks_the_tes_global_and_sign_extends() {
        let mut e = engine();
        put_u32(&mut e, TES, 0x7000);
        e.register_double(TES_GET_DEAD_COUNT, |_, a| {
            assert_eq!(a, [0x7000, 0x3000]);
            0x1234_fffe_u32.into_ret()
        });
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x005a_01f0, 0, 0x3000, 0), (true, -2.0));
        assert!(printed(&take_log(&mut e), 0x0103_5530));
        // No form: 0, no line.
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x005a_01f0, 0, 0, 0), (true, 0.0));
        assert!(!printed(&take_log(&mut e), 0x0103_5530));
    }

    #[test]
    fn alert_is_a_byte() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, ACTOR_GET_ALERT, 0x1234_5603);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x005a_0260, actor, 0, 0), (true, 3.0));
        assert!(printed(&take_log(&mut e), 0x0103_5544));
        let plain = non_actor(&mut e);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x005a_0260, plain, 0, 0), (true, 0.0));
        assert!(!printed(&take_log(&mut e), 0x0103_5544));
    }

    #[test]
    fn player_controls_disabled_tests_the_mask_and_lists_the_controls() {
        let mut e = engine();
        let player = e.mem.alloc(0x700);
        e.mem.set_u8(player + 0x680, 0b0000_0101);
        put_u32(&mut e, PLAYER, player);
        for i in 0..7 {
            put_u32(&mut e, CONTROL_NAMES + 4 * i, 0x5000 + i);
        }
        assert_eq!(run(&mut e, 0x005a_02f0, 0, 4, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_02f0, 0, 2, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_02f0, 0, 2, 0);
        let log = take_log(&mut e);
        let prints = calls_to(&log, DEBUG_PRINT);
        let zero = words(0.0);
        assert_eq!(prints[0], vec![0x0103_5580, zero[0], zero[1]]);
        let disabled = 0x0103_556c;
        let enabled = 0x0103_5558;
        let formats: Vec<u32> = prints[1..].iter().map(|w| w[0]).collect();
        assert_eq!(
            formats,
            vec![disabled, enabled, disabled, enabled, enabled, enabled, enabled]
        );
        let names: Vec<u32> = prints[1..].iter().map(|w| w[1]).collect();
        assert_eq!(names, (0..7).map(|i| 0x5000 + i).collect::<Vec<_>>());
    }

    #[test]
    fn disabled_controls_byte_and_mask_test() {
        let mut e = engine();
        let player = e.mem.alloc(0x700);
        e.mem.set_u8(player + 0x680, 0x80);
        assert_eq!(e.call(0x005a_03d0, &args![player]).u32() & 0xff, 0x80);
        assert!(e.call(0x005a_03f0, &args![player, 0x80u32]).bool());
        // The mask is a full word: bits above the byte never match.
        assert!(!e.call(0x005a_03f0, &args![player, 0x100u32]).bool());
        assert!(!e.call(0x005a_03f0, &args![player, 0x01u32]).bool());
    }

    #[test]
    fn heading_angle_is_wrapped_into_plus_minus_pi_and_given_in_degrees() {
        let mut e = engine();
        let pi = std::f32::consts::PI as f64;
        put_f64(&mut e, DOUBLE_MINUS_PI, -pi);
        put_f64(&mut e, DOUBLE_PI, pi);
        put_f64(&mut e, DOUBLE_TWO_PI, 2.0 * pi);
        let degrees = e.global::<f64>(DEGREES_PER_RADIAN);
        let own = floats(&mut e, [1.0, 2.0, 3.0]);
        let target = floats(&mut e, [4.0, 6.0, 8.0]);
        stub(&mut e, 0x0f10_0050, own);
        stub(&mut e, 0x0f10_0051, target);
        let facing = std::rc::Rc::new(std::cell::Cell::new(-0.5f64));
        let facing_in = facing.clone();
        e.register_double(0x0f10_0052, move |_, a| {
            assert_eq!(a[1], 0);
            Ret {
                st0: facing_in.get(),
                ..Ret::default()
            }
        });
        stub(&mut e, 0x0f10_0053, 1);
        let mobile = object(
            &mut e,
            &[
                (VSLOT_IS_MOBILE_OBJECT, 0x0f10_0053),
                (VSLOT_GET_POSITION, 0x0f10_0050),
                (VSLOT_MOBILE_Z_ANGLE, 0x0f10_0052),
            ],
        );
        let target_ref = object(&mut e, &[(VSLOT_GET_POSITION, 0x0f10_0051)]);
        // target - own goes into the out vector, whose Z angle is 2.5.
        e.register_double(POINT3_SUBTRACT, move |e, a| {
            assert_eq!((a[0], a[2]), (target, own));
            for i in 0..3 {
                let d = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, d);
            }
            Ret::default()
        });
        e.register_double(GET_Z_ANGLE_FROM_VECTOR, |e, a| {
            assert_eq!(
                [e.mem.f32(a[0]), e.mem.f32(a[0] + 4), e.mem.f32(a[0] + 8)],
                [3.0, 4.0, 5.0]
            );
            Ret {
                st0: 2.5,
                ..Ret::default()
            }
        });
        // In range: 2.5 - (-0.5) = 3.0.
        assert_eq!(
            run(&mut e, 0x005a_0410, mobile, target_ref, 0),
            (true, 3.0 * degrees)
        );
        // Above pi: wrapped down by 2 pi.
        facing.set(-1.5);
        assert_eq!(
            run(&mut e, 0x005a_0410, mobile, target_ref, 0),
            (true, (4.0 - 2.0 * pi) * degrees)
        );
        // Below -pi: wrapped up.
        facing.set(6.0);
        assert_eq!(
            run(&mut e, 0x005a_0410, mobile, target_ref, 0),
            (true, (-3.5 + 2.0 * pi) * degrees)
        );
        // No target, or no mobile object: 0.
        assert_eq!(run(&mut e, 0x005a_0410, mobile, 0, 0), (true, 0.0));
        stub(&mut e, 0x0f10_0054, 0);
        let plain = object(&mut e, &[(VSLOT_IS_MOBILE_OBJECT, 0x0f10_0054)]);
        assert_eq!(run(&mut e, 0x005a_0410, plain, target_ref, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_0410, plain, target_ref, 0);
        assert!(printed(&take_log(&mut e), 0x0103_55a4));
    }

    #[test]
    fn weapon_out_asks_the_actor() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, ACTOR_IS_WEAPON_DRAWN, 1);
        assert_eq!(run(&mut e, 0x005a_0550, actor, 0, 0), (true, 1.0));
        stub(&mut e, ACTOR_IS_WEAPON_DRAWN, 0);
        assert_eq!(run(&mut e, 0x005a_0550, actor, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_0550, 0, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_55bc));
    }

    /// A middle-high process whose weapon data (virtual `+0x148`) is
    /// `data`; the data's form is read at `+8`.
    fn process_with_weapon_data(e: &mut Engine, data: u32) -> u32 {
        stub(e, 0x0f10_0060, data);
        e.register(WEAPON_DATA_GET_FORM, |e, a| e.mem.u32(a[0] + 8).into_ret());
        object(e, &[(VSLOT_PROCESS_WEAPON_DATA, 0x0f10_0060)])
    }

    #[test]
    fn weapon_in_list_uses_the_equipped_weapon_or_the_fallback() {
        let mut e = engine();
        use_form_types(&mut e);
        let actor = actor(&mut e);
        let list = e.mem.alloc(0x200);
        let weapon = typed_object(&mut e, 0x28);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data + 8, weapon);
        let process = process_with_weapon_data(&mut e, data);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        put_u32(&mut e, FALLBACK_WEAPON, 0x7777);
        e.register_double(LIST_CONTAINS, move |e, a| {
            assert_eq!(a[0], list + 0x18);
            (e.mem.u32(a[1]) == weapon).into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_05e0, actor, list, 0), (true, 1.0));
        // The data's form is no weapon: the fallback weapon is asked.
        let armor = typed_object(&mut e, 0x18);
        e.mem.set_u32(data + 8, armor);
        e.register_double(LIST_CONTAINS, move |e, a| {
            (e.mem.u32(a[1]) == 0x7777).into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_05e0, actor, list, 0), (true, 1.0));
        // Neither a weapon nor a fallback: the list is not asked.
        put_u32(&mut e, FALLBACK_WEAPON, 0);
        e.register_double(LIST_CONTAINS, |_, _| panic!("not asked"));
        assert_eq!(run(&mut e, 0x005a_05e0, actor, list, 0), (true, 0.0));
        // No process, no list, no actor: 0.
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_05e0, actor, list, 0), (true, 0.0));
        stub(&mut e, ACTOR_GET_PROCESS, process);
        assert_eq!(run(&mut e, 0x005a_05e0, actor, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_05e0, plain, list, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_05e0, plain, list, 0);
        assert!(printed(&take_log(&mut e), 0x0103_55d4));
    }

    #[test]
    fn facing_up_looks_for_the_node_then_the_spine() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, REF_GET_NODE_KEY, 0x1111);
        put_u32(&mut e, 0x011c_61b4, 0x2222);
        e.register_double(REF_FIND_NODE, move |_, a| {
            assert_eq!(a, [actor, 0x1111, 0x2222]);
            0x5000u32.into_ret()
        });
        e.register_double(COLLISION_GET_FACE_UP, |_, a| {
            assert_eq!(a, [0x5000, 0]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_0710, actor, 0, 0), (true, 1.0));
        stub(&mut e, COLLISION_GET_FACE_UP, 0);
        assert_eq!(run(&mut e, 0x005a_0710, actor, 0, 0), (true, 0.0));
        // No node: the spine node by name, and with none at all it is up.
        stub(&mut e, REF_FIND_NODE, 0);
        e.register_double(REF_FIND_NODE_BY_NAME, move |_, a| {
            assert_eq!(a, [actor, 0x1111, SPINE_NODE_NAME]);
            0x5100u32.into_ret()
        });
        e.register_double(COLLISION_GET_FACE_UP, |_, a| {
            assert_eq!(a[0], 0x5100);
            false.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_0710, actor, 0, 0), (true, 0.0));
        stub(&mut e, REF_FIND_NODE_BY_NAME, 0);
        assert_eq!(run(&mut e, 0x005a_0710, actor, 0, 0), (true, 1.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_0710, plain, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_0710, plain, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_55ec));
    }

    #[test]
    fn node_key_suffix_is_a_global() {
        let mut e = engine();
        put_u32(&mut e, 0x011c_61b4, 0xabcd);
        assert_eq!(e.call(0x005a_07f0, &args![]).u32(), 0xabcd);
    }

    #[test]
    fn left_up_looks_for_the_node() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, REF_GET_NODE_KEY, 0x1111);
        put_u32(&mut e, 0x011c_61b4, 0x2222);
        e.register_double(REF_FIND_NODE, move |_, a| {
            assert_eq!(a, [actor, 0x1111, 0x2222]);
            0x5000u32.into_ret()
        });
        e.register_double(COLLISION_GET_LEFT_UP, |_, a| {
            assert_eq!(a, [0x5000, 0]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_0800, actor, 0, 0), (true, 1.0));
        stub(&mut e, COLLISION_GET_LEFT_UP, 0);
        assert_eq!(run(&mut e, 0x005a_0800, actor, 0, 0), (true, 0.0));
        // No node at all counts as left up.
        stub(&mut e, REF_FIND_NODE, 0);
        assert_eq!(run(&mut e, 0x005a_0800, actor, 0, 0), (true, 1.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_0800, plain, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_0800, plain, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_5614));
    }

    #[test]
    fn knocked_state_maps_states_three_and_four_to_one() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, ACTOR_GET_PROCESS, 0x6600);
        let state = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let state_in = state.clone();
        e.register_double(0x0f10_0070, move |_, _| state_in.get().into_ret());
        let knocked = object(&mut e, &[(VSLOT_KNOCKED_STATE, 0x0f10_0070)]);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            assert_eq!(
                a,
                [
                    0x6600,
                    0,
                    TYPE_SOURCE_OF_KNOCKED_STATE_CAST,
                    TYPE_TARGET_OF_KNOCKED_STATE_CAST,
                    0
                ]
            );
            knocked.into_ret()
        });
        for (number, expected) in [
            (0u32, 0.0),
            (1, 0.0),
            (2, 0.0),
            (3, 1.0),
            (4, 1.0),
            (5, 0.0),
            (6, 0.0),
            (7, 0.0),
        ] {
            state.set(number);
            assert_eq!(
                run(&mut e, 0x005a_08c0, actor, 0, 0),
                (true, expected),
                "state {number}"
            );
        }
        // The cast fails: 0.
        stub(&mut e, RT_DYNAMIC_CAST, 0);
        assert_eq!(run(&mut e, 0x005a_08c0, actor, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_08c0, plain, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_08c0, plain, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_5628));
    }

    #[test]
    fn weapon_anim_type_reads_the_table_by_the_weapons_index() {
        let mut e = engine();
        use_form_types(&mut e);
        let actor = actor(&mut e);
        let weapon = typed_object(&mut e, 0x28);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data + 8, weapon);
        let process = process_with_weapon_data(&mut e, data);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        e.map(WEAPON_ANIM_TYPE_TABLE & !0xfff, 0x1000);
        e.mem.set_i32(WEAPON_ANIM_TYPE_TABLE + 8, 17);
        stub(&mut e, WEAPON_GET_ANIM_TYPE_INDEX, 2);
        assert_eq!(run(&mut e, 0x005a_09b0, actor, 0, 0), (true, 17.0));
        // A process but no weapon: 1.0.
        e.mem.set_u32(data + 8, 0);
        assert_eq!(run(&mut e, 0x005a_09b0, actor, 0, 0), (true, 1.0));
        // No process or no actor: 0.
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_09b0, actor, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_09b0, plain, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_09b0, plain, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_5644));
    }

    #[test]
    fn weapon_skill_type_compares_the_weapons_skill() {
        let mut e = engine();
        use_form_types(&mut e);
        let weapon = typed_object(&mut e, 0x28);
        stub(&mut e, REF_GET_BASE_FORM, weapon);
        stub(&mut e, WEAPON_GET_SKILL, 0x20);
        assert_eq!(run(&mut e, 0x005a_0ab0, 0x2000, 0x20, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_0ab0, 0x2000, 0x21, 0), (true, 0.0));
        // A base that is no weapon matches skill 0x2d only, and otherwise the
        // function returns false without a debug line.
        let armor = typed_object(&mut e, 0x18);
        stub(&mut e, REF_GET_BASE_FORM, armor);
        assert_eq!(run(&mut e, 0x005a_0ab0, 0x2000, 0x2d, 0), (true, 1.0));
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x005a_0ab0, 0x2000, 0x20, 0), (false, 0.0));
        assert!(!printed(&take_log(&mut e), 0x0103_5660));
        trace_on(&mut e);
        run(&mut e, 0x005a_0ab0, 0x2000, 0x2d, 0);
        assert!(printed(&take_log(&mut e), 0x0103_5660));
    }

    #[test]
    fn current_ai_package_maps_the_package_type_to_a_number() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, ACTOR_GET_PROCESS, 0x6600);
        stub(&mut e, MOBILE_GET_CURRENT_PACKAGE, 0x5500);
        let kind = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let kind_in = kind.clone();
        e.register_double(PACKAGE_GET_TYPE, move |_, a| {
            assert_eq!(a, [0x5500]);
            kind_in.get().into_ret()
        });
        // The value of each package type (the doubles in the exe's data).
        let values: [(u32, f64); 29] = [
            (2, 2.0),
            (3, 3.0),
            (4, 4.0),
            (5, 13.0),
            (6, 14.0),
            (7, 15.0),
            (8, 16.0),
            (9, 17.0),
            (10, 18.0),
            (11, 19.0),
            (12, 36.0),
            (13, 37.0),
            (14, 35.0),
            (15, 34.0),
            (16, 33.0),
            (18, 5.0),
            (19, 20.0),
            (20, 7.0),
            (21, 8.0),
            (22, 10.0),
            (23, 11.0),
            (24, 9.0),
            (25, 12.0),
            (26, 21.0),
            (27, 24.0),
            (28, 6.0),
            (29, 28.0),
            (30, 29.0),
            (31, 32.0),
        ];
        for (number, value) in values {
            put_f64(&mut e, PACKAGE_VALUE_ADDRESS[number as usize], value);
        }
        let expected = |number: u32| match number {
            0 => 0.0,
            1 => 1.0,
            _ => values
                .iter()
                .find(|(n, _)| *n == number)
                .map_or(-1.0, |(_, v)| *v),
        };
        for number in (0..=40).chain([100, 0xffff_ffff]) {
            kind.set(number);
            assert_eq!(
                run(&mut e, 0x005a_0b60, actor, 0, 0),
                (true, expected(number)),
                "type {number}"
            );
        }
        // No package, no process, no actor: 0.
        stub(&mut e, MOBILE_GET_CURRENT_PACKAGE, 0);
        assert_eq!(run(&mut e, 0x005a_0b60, actor, 0, 0), (true, 0.0));
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_0b60, actor, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_0b60, plain, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_0b60, plain, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_5680));
    }

    #[test]
    fn waiting_asks_the_actor() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, ACTOR_IS_WAITING, 1);
        assert_eq!(run(&mut e, 0x005a_0e80, actor, 0, 0), (true, 1.0));
        stub(&mut e, ACTOR_IS_WAITING, 0);
        assert_eq!(run(&mut e, 0x005a_0e80, actor, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_0e80, plain, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_0e80, 0, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_5720));
    }

    #[test]
    fn idle_playing_is_a_special_idle_not_done() {
        let mut e = engine();
        stub(&mut e, 0x0f10_0080, 0x5000);
        let reference = object(&mut e, &[(VSLOT_REF_GET_ANIMATION, 0x0f10_0080)]);
        e.register_double(ANIMATION_SPECIAL_IDLE_DONE_PLAYING, |_, a| {
            assert_eq!(a, [0x5000]);
            false.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_0f10, reference, 0, 0), (true, 1.0));
        stub(&mut e, ANIMATION_SPECIAL_IDLE_DONE_PLAYING, 1);
        assert_eq!(run(&mut e, 0x005a_0f10, reference, 0, 0), (true, 0.0));
        // No animation, or no reference: 0.
        stub(&mut e, 0x0f10_0080, 0);
        assert_eq!(run(&mut e, 0x005a_0f10, reference, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_0f10, 0, 0, 0), (true, 0.0));
        trace_on(&mut e);
        run(&mut e, 0x005a_0f10, 0, 0, 0);
        assert!(printed(&take_log(&mut e), 0x0103_5734));
    }

    // ---- functions from 005a0f90 on ----

    /// Runs the condition function with the debug lines on and tells whether
    /// the line with the given format was printed.
    fn traces(e: &mut Engine, addr: u32, reference: u32, param1: u32, format: u32) -> bool {
        trace_on(e);
        run(e, addr, reference, param1, 0);
        printed(&take_log(e), format)
    }

    #[test]
    fn crime_asks_the_process_lists_with_a_clamped_crime_number() {
        let mut e = engine();
        use_form_types(&mut e);
        let actor = actor(&mut e);
        let placed = typed_object(&mut e, 0x3c);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(PROCESS_LISTS_CRIME_TEST, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            true.into_ret()
        });
        for (crime, passed) in [
            (0, 0),
            (3, 3),
            (4, 4),
            (5, u32::MAX),
            (u32::MAX, u32::MAX),
            (0x7fff_ffff, u32::MAX),
        ] {
            seen.borrow_mut().clear();
            assert_eq!(
                run(&mut e, 0x005a_0f90, actor, placed, crime),
                (true, 1.0),
                "crime {crime:#x}"
            );
            assert_eq!(
                *seen.borrow(),
                vec![vec![
                    PROCESS_LISTS,
                    actor,
                    placed,
                    actor,
                    passed,
                    0,
                    u32::MAX
                ]]
            );
        }
        stub(&mut e, PROCESS_LISTS_CRIME_TEST, 0);
        assert_eq!(run(&mut e, 0x005a_0f90, actor, placed, 1), (true, 0.0));
        // A form that is not a placed reference, a non-actor, or no form.
        stub(&mut e, PROCESS_LISTS_CRIME_TEST, 1);
        let wrong = typed_object(&mut e, 0x20);
        assert_eq!(run(&mut e, 0x005a_0f90, actor, wrong, 1), (true, 0.0));
        let extra_type = typed_object(&mut e, 0x69);
        assert_eq!(run(&mut e, 0x005a_0f90, actor, extra_type, 1), (true, 1.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_0f90, plain, placed, 1), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_0f90, actor, 0, 1), (true, 0.0));
        assert!(traces(&mut e, 0x005a_0f90, 0, 0, 0x0103_5750));
    }

    #[test]
    fn guard_asks_the_class_of_an_npc() {
        let mut e = engine();
        use_form_types(&mut e);
        let npc = typed_object(&mut e, 0x2a);
        stub(&mut e, REF_GET_BASE_FORM, npc);
        stub(&mut e, NPC_GET_CLASS, 0x7000);
        e.register_double(CLASS_IS_GUARD, |_, a| {
            assert_eq!(a, [0x7000]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_10b0, 0x2000, 0, 0), (true, 1.0));
        stub(&mut e, CLASS_IS_GUARD, 0);
        assert_eq!(run(&mut e, 0x005a_10b0, 0x2000, 0, 0), (true, 0.0));
        // No class, a base form that is not an NPC_, no reference.
        stub(&mut e, CLASS_IS_GUARD, 1);
        stub(&mut e, NPC_GET_CLASS, 0);
        assert_eq!(run(&mut e, 0x005a_10b0, 0x2000, 0, 0), (true, 0.0));
        stub(&mut e, NPC_GET_CLASS, 0x7000);
        let creature = typed_object(&mut e, 0x2b);
        stub(&mut e, REF_GET_BASE_FORM, creature);
        assert_eq!(run(&mut e, 0x005a_10b0, 0x2000, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_10b0, 0, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_10b0, 0, 0, 0x0103_5764));
    }

    #[test]
    fn talking_asks_the_actor_or_the_mobile_object() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, ACTOR_IS_TALKING, 1);
        assert_eq!(run(&mut e, 0x005a_1150, actor, 0, 0), (true, 1.0));
        stub(&mut e, ACTOR_IS_TALKING, 0);
        assert_eq!(run(&mut e, 0x005a_1150, actor, 0, 0), (true, 0.0));
        // Not an actor: the reference as a mobile object.
        let plain = non_actor(&mut e);
        e.register_double(REF_GET_MOBILE_OBJECT_EXTRA, move |_, a| {
            assert_eq!(a, [plain]);
            0x6000u32.into_ret()
        });
        e.register_double(MOBILE_OBJECT_IS_TALKING, |_, a| {
            assert_eq!(a, [0x6000]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_1150, plain, 0, 0), (true, 1.0));
        stub(&mut e, MOBILE_OBJECT_IS_TALKING, 0);
        assert_eq!(run(&mut e, 0x005a_1150, plain, 0, 0), (true, 0.0));
        stub(&mut e, REF_GET_MOBILE_OBJECT_EXTRA, 0);
        assert_eq!(run(&mut e, 0x005a_1150, plain, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_1150, plain, 0, 0x0103_5778));
    }

    /// An engine for the two procedure functions: the actor, its process
    /// (virtual `+0x280` answers `index`, virtual `+0x274` the object), its
    /// package, and the procedure tables mapping (group 2, `index`) to
    /// `procedure`.
    fn procedure_setup(e: &mut Engine, index: u32, procedure: u32, process_object: u32) -> u32 {
        for page in [0x011a_3000u32, 0x0119_b000] {
            e.map(page, 0x1000);
        }
        e.register_double(0x0f20_0001, move |_, _| index.into_ret());
        e.register_double(0x0f20_0002, move |_, _| process_object.into_ret());
        let process = object(
            e,
            &[
                (VSLOT_PROCESS_PROCEDURE_INDEX, 0x0f20_0001),
                (VSLOT_PROCESS_PROCEDURE_OBJECT, 0x0f20_0002),
            ],
        );
        stub(e, ACTOR_GET_PROCESS, process);
        stub(e, MOBILE_GET_CURRENT_PACKAGE, 0x5000);
        stub(e, PACKAGE_GET_PROCEDURE_GROUP, 2);
        stub(e, PACKAGE_GET_TYPE, 0);
        stub(e, COMBAT_CONTROLLER_IS_FLEEING, 0);
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(table + index * 4, procedure);
        e.mem.set_u32(PROCEDURE_TABLES + 8, table);
        actor(e)
    }

    #[test]
    fn procedure_maps_the_number_to_a_double_of_the_exe() {
        let mut e = engine();
        // Distinct values at every address the switch can read.
        for (number, address) in PROCEDURE_VALUE_ADDRESS.iter().enumerate() {
            if *address != 0 {
                e.map(*address & !0xfff, 0x1000);
                e.set_global(*address, 1000.0 + number as f64);
            }
        }
        e.map(PROCEDURE_SPECIAL_DEFAULT & !0xfff, 0x1000);
        e.set_global(PROCEDURE_SPECIAL_DEFAULT, 45.0f64);
        let actor = procedure_setup(&mut e, 3, 0, 0);
        for number in (0..=0x36u32).chain([0x37, 0x100, u32::MAX]) {
            let table = e.mem.u32(PROCEDURE_TABLES + 8);
            e.mem.set_u32(table + 12, number);
            let expected = match number {
                0 => 0.0,
                2 => 1.0,
                PROCEDURE_SPECIAL => 45.0,
                n if n <= 0x36 && PROCEDURE_VALUE_ADDRESS[n as usize] != 0 => 1000.0 + n as f64,
                _ => -1.0,
            };
            assert_eq!(
                run(&mut e, 0x005a_1210, actor, 0, 0),
                (true, expected),
                "procedure {number:#x}"
            );
        }
    }

    #[test]
    fn procedure_special_case_reads_the_float_tables() {
        let mut e = engine();
        for address in [
            PROCEDURE_SPECIAL_DEFAULT,
            PROCEDURE_SPECIAL_TABLE_BY_TYPE,
            PROCEDURE_SPECIAL_TABLE_BY_VALUE,
        ] {
            e.map(address & !0xfff, 0x1000);
        }
        e.set_global(PROCEDURE_SPECIAL_DEFAULT, 45.0f64);
        for i in 0..4u32 {
            e.set_global(PROCEDURE_SPECIAL_TABLE_BY_TYPE + 4 * i, 10.5f32 + i as f32);
        }
        for i in 0..7u32 {
            e.set_global(PROCEDURE_SPECIAL_TABLE_BY_VALUE + 4 * i, 20.5f32 + i as f32);
        }
        // The object of virtual +0x274: type at +4, value at +8.
        let object = e.mem.alloc(0x20);
        stub(&mut e, PROCEDURE_OBJECT_GET_VALUE, 0);
        e.register_double(PROCEDURE_OBJECT_GET_VALUE, |e, a| {
            e.mem.u32(a[0] + 8).into_ret()
        });
        let actor = procedure_setup(&mut e, 1, PROCEDURE_SPECIAL, object);
        let cases: [(u32, u32, f64); 8] = [
            (1, 0, 11.5),
            (0, 0, 10.5),
            (2, 0, 12.5),
            (4, 0, 45.0),
            (u32::MAX, 0, 45.0),
            (3, u32::MAX, 20.5),
            (3, 5, 26.5),
            (3, 6, 45.0),
        ];
        for (kind, value, expected) in cases {
            e.mem.set_u32(object + 4, kind);
            e.mem.set_u32(object + 8, value);
            assert_eq!(
                run(&mut e, 0x005a_1210, actor, 0, 0),
                (true, expected),
                "kind {kind:#x} value {value:#x}"
            );
        }
        // No object: the default stays.
        let actor = procedure_setup(&mut e, 1, PROCEDURE_SPECIAL, 0);
        assert_eq!(run(&mut e, 0x005a_1210, actor, 0, 0), (true, 45.0));
    }

    #[test]
    fn procedure_of_a_fleeing_combat_package_is_ten() {
        let mut e = engine();
        e.map(PROCEDURE_VALUE_ADDRESS[10] & !0xfff, 0x1000);
        e.set_global(PROCEDURE_VALUE_ADDRESS[10], 77.0f64);
        let actor = procedure_setup(&mut e, 0, 2, 0);
        assert_eq!(run(&mut e, 0x005a_1210, actor, 0, 0), (true, 1.0));
        stub(&mut e, PACKAGE_GET_TYPE, PACKAGE_TYPE_COMBAT);
        assert_eq!(run(&mut e, 0x005a_1210, actor, 0, 0), (true, 1.0));
        stub(&mut e, COMBAT_CONTROLLER_IS_FLEEING, 1);
        assert_eq!(run(&mut e, 0x005a_1210, actor, 0, 0), (true, 77.0));
        // Fleeing only counts for combat packages.
        stub(&mut e, PACKAGE_GET_TYPE, 5);
        assert_eq!(run(&mut e, 0x005a_1210, actor, 0, 0), (true, 1.0));
        // No package, no process, not an actor.
        stub(&mut e, MOBILE_GET_CURRENT_PACKAGE, 0);
        assert_eq!(run(&mut e, 0x005a_1210, actor, 0, 0), (true, 0.0));
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_1210, actor, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_1210, plain, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_1210, plain, 0, 0x0103_578c));
    }

    #[test]
    fn procedure_number_comes_from_the_package_or_the_process_object() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, MOBILE_GET_CURRENT_PACKAGE, 0x5000);
        stub(&mut e, PACKAGE_GET_TYPE, PACKAGE_TYPE_WITH_NUMBER);
        e.register_double(PACKAGE_GET_NUMBER, |_, a| {
            assert_eq!(a, [0x5000]);
            (-6i32 as u32).into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_1770, actor, 0, 0), (true, -6.0));
        // Type 0x0e: the id of the process object.
        stub(&mut e, 0x0f20_0002, 0x9100);
        let process = object(&mut e, &[(VSLOT_PROCESS_PROCEDURE_OBJECT, 0x0f20_0002)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        stub(&mut e, PROCEDURE_OBJECT_GET_ID, 0x1234);
        stub(&mut e, PACKAGE_GET_TYPE, PACKAGE_TYPE_WITH_PROCESS_OBJECT);
        assert_eq!(run(&mut e, 0x005a_1770, actor, 0, 0), (true, 4660.0));
        stub(&mut e, 0x0f20_0002, 0);
        assert_eq!(run(&mut e, 0x005a_1770, actor, 0, 0), (true, -1.0));
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_1770, actor, 0, 0), (true, -1.0));
        // Another type, no package, not an actor: -1.
        stub(&mut e, PACKAGE_GET_TYPE, 3);
        assert_eq!(run(&mut e, 0x005a_1770, actor, 0, 0), (true, -1.0));
        stub(&mut e, MOBILE_GET_CURRENT_PACKAGE, 0);
        assert_eq!(run(&mut e, 0x005a_1770, actor, 0, 0), (true, -1.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_1770, plain, 0, 0), (true, -1.0));
        assert!(traces(&mut e, 0x005a_1770, plain, 0, 0x0103_578c));
    }

    #[test]
    fn wind_speed_needs_the_tes_and_its_sky() {
        let mut e = engine();
        e.set_global(TES, 0x9000u32);
        e.register_double(TES_GET_SKY, |_, a| {
            assert_eq!(a, [0x9000]);
            0x9100u32.into_ret()
        });
        e.register_double(SKY_GET_WIND_SPEED, |_, a| {
            assert_eq!(a, [0x9100]);
            Ret {
                st0: 3.25,
                ..Ret::default()
            }
        });
        assert_eq!(run(&mut e, 0x005a_1880, 0, 0, 0), (true, 3.25));
        assert!(traces(&mut e, 0x005a_1880, 0, 0, 0x0103_5848));
        // No sky, or no TES: false, result 0, no debug line.
        stub(&mut e, TES_GET_SKY, 0);
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x005a_1880, 0, 0, 0), (false, 0.0));
        assert!(!printed(&take_log(&mut e), 0x0103_5848));
        e.set_global(TES, 0u32);
        assert_eq!(run(&mut e, 0x005a_1880, 0, 0, 0), (false, 0.0));
    }

    #[test]
    fn trespassing_asks_the_actor() {
        let mut e = engine();
        stub(&mut e, 0x0f20_0010, 1);
        let trespassing = actor_with(&mut e, &[(VSLOT_ACTOR_IS_TRESPASSING, 0x0f20_0010)]);
        assert_eq!(run(&mut e, 0x005a_1900, trespassing, 0, 0), (true, 1.0));
        stub(&mut e, 0x0f20_0010, 0);
        assert_eq!(run(&mut e, 0x005a_1900, trespassing, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_1900, plain, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_1900, 0, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_1900, 0, 0, 0x0103_5860));
    }

    #[test]
    fn in_my_owned_cell_asks_the_parent_cell() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, REF_GET_PARENT_CELL, 0x8000);
        e.register_double(CELL_IS_OWNED_BY, move |_, a| {
            assert_eq!(a, [0x8000, actor]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_19a0, actor, 0, 0), (true, 1.0));
        stub(&mut e, CELL_IS_OWNED_BY, 0);
        assert_eq!(run(&mut e, 0x005a_19a0, actor, 0, 0), (true, 0.0));
        // No parent cell: never asked.
        stub(&mut e, CELL_IS_OWNED_BY, 1);
        stub(&mut e, REF_GET_PARENT_CELL, 0);
        assert_eq!(run(&mut e, 0x005a_19a0, actor, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_19a0, 0, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_19a0, 0, 0, 0x0103_587c));
    }

    #[test]
    fn continuing_package_near_pc_is_the_byte() {
        let mut e = engine();
        let actor = actor(&mut e);
        stub(&mut e, ACTOR_IS_CONTINUING_PACKAGE_FOR_PC, 1);
        assert_eq!(run(&mut e, 0x005a_1a40, actor, 0, 0), (true, 1.0));
        // Only the low byte counts.
        stub(&mut e, ACTOR_IS_CONTINUING_PACKAGE_FOR_PC, 0x1200);
        assert_eq!(run(&mut e, 0x005a_1a40, actor, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_1a40, plain, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_1a40, 0, 0, 0x0103_589c));
    }

    #[test]
    fn can_have_flames_looks_up_the_extra_data_of_the_node() {
        let mut e = engine();
        stub(&mut e, FLAMES_EXTRA_DATA_NAME, 0x1234);
        stub(&mut e, 0x0f20_0020, 0x5000);
        let reference = object(&mut e, &[(VSLOT_REF_GET_NODE, 0x0f20_0020)]);
        e.register_double(NODE_GET_EXTRA_DATA, |_, a| {
            assert_eq!(a, [0x5000, 0x1234]);
            0x5100u32.into_ret()
        });
        e.register_double(EXTRA_DATA_CAST, |_, a| {
            assert_eq!(a, [FLAMES_EXTRA_DATA_TYPE, 0x5100]);
            0x5200u32.into_ret()
        });
        e.register_double(FLAMES_EXTRA_DATA_TEST, |_, a| {
            assert_eq!(a, [0x5200]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_1ad0, reference, 0, 0), (true, 1.0));
        stub(&mut e, FLAMES_EXTRA_DATA_TEST, 0);
        assert_eq!(run(&mut e, 0x005a_1ad0, reference, 0, 0), (true, 0.0));
        // No extra data: the test is not run and the result stays 0.
        stub(&mut e, NODE_GET_EXTRA_DATA, 0);
        stub(&mut e, FLAMES_EXTRA_DATA_TEST, 1);
        assert_eq!(run(&mut e, 0x005a_1ad0, reference, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_1ad0, 0, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_1ad0, 0, 0, 0x0103_58c8));
    }

    #[test]
    fn has_flames_asks_for_the_addon_nodes_of_the_node() {
        let mut e = engine();
        stub(&mut e, 0x0f20_0031, 0x77);
        let node = object(&mut e, &[(VSLOT_NODE_FLAMES_ARGUMENT, 0x0f20_0031)]);
        stub(&mut e, 0x0f20_0030, node);
        let reference = object(&mut e, &[(VSLOT_REF_GET_NODE, 0x0f20_0030)]);
        e.register_double(REF_HAS_ADDON_NODES, |_, a| {
            assert_eq!(a, [0x77]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_1b80, reference, 0, 0), (true, 1.0));
        stub(&mut e, REF_HAS_ADDON_NODES, 0);
        assert_eq!(run(&mut e, 0x005a_1b80, reference, 0, 0), (true, 0.0));
        // No node, no reference: 0 and the test is not run.
        stub(&mut e, REF_HAS_ADDON_NODES, 1);
        stub(&mut e, 0x0f20_0030, 0);
        assert_eq!(run(&mut e, 0x005a_1b80, reference, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_1b80, 0, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_1b80, 0, 0, 0x0103_58e0));
    }

    #[test]
    fn open_state_is_a_signed_integer() {
        let mut e = engine();
        e.register_double(OPEN_CLOSE_GET_OPEN_STATE, |_, a| {
            assert_eq!(a, [0x2000]);
            2u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_1c30, 0x2000, 0, 0), (true, 2.0));
        stub(&mut e, OPEN_CLOSE_GET_OPEN_STATE, -1i32 as u32);
        assert_eq!(run(&mut e, 0x005a_1c30, 0x2000, 0, 0), (true, -1.0));
        assert_eq!(run(&mut e, 0x005a_1c30, 0, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_1c30, 0, 0, 0x0103_58f4));
    }

    #[test]
    fn current_package_compares_with_the_package_form() {
        let mut e = engine();
        use_form_types(&mut e);
        let actor = actor(&mut e);
        let package = typed_object(&mut e, 0x49);
        let other = typed_object(&mut e, 0x49);
        stub(&mut e, ACTOR_GET_PACKAGE_SET_AS_CURRENT, package);
        stub(&mut e, PACKAGE_GET_IS_CREATED, 0);
        assert_eq!(run(&mut e, 0x005a_1c90, actor, package, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_1c90, actor, other, 0), (true, 0.0));
        // A package created at run time: the one in the extra data is used.
        stub(&mut e, PACKAGE_GET_IS_CREATED, 1);
        stub(&mut e, REF_GET_EXTRA_DATA_LIST, 0x7000);
        e.register_double(EXTRA_DATA_LIST_GET_PACKAGE_EXTRA, move |_, a| {
            assert_eq!(a, [0x7000]);
            other.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_1c90, actor, other, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_1c90, actor, package, 0), (true, 0.0));
        // Extra data without a package keeps the original one.
        stub(&mut e, EXTRA_DATA_LIST_GET_PACKAGE_EXTRA, 0);
        assert_eq!(run(&mut e, 0x005a_1c90, actor, package, 0), (true, 1.0));
        // No current package never matches; wrong form type, non-actor.
        stub(&mut e, ACTOR_GET_PACKAGE_SET_AS_CURRENT, 0);
        assert_eq!(run(&mut e, 0x005a_1c90, actor, package, 0), (true, 0.0));
        let wrong = typed_object(&mut e, 0x47);
        stub(&mut e, ACTOR_GET_PACKAGE_SET_AS_CURRENT, wrong);
        assert_eq!(run(&mut e, 0x005a_1c90, actor, wrong, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_1c90, plain, package, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_1c90, plain, 0, 0x0103_590c));
    }

    #[test]
    fn day_of_week_asks_the_calendar() {
        let mut e = engine();
        e.register_double(CALENDAR_GET_DAY_OF_WEEK, |_, a| {
            assert_eq!(a, [CALENDAR]);
            4u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_1d80, 0, 0, 0), (true, 4.0));
        assert!(traces(&mut e, 0x005a_1d80, 0, 0, 0x0103_592c));
    }

    /// A zeroed player-sized object set as the player global.
    fn player_object(e: &mut Engine) -> u32 {
        let player = e.mem.alloc(0x800);
        e.set_global(PLAYER, player);
        player
    }

    #[test]
    fn pc_sleeping_reads_the_player_byte() {
        let mut e = engine();
        let player = player_object(&mut e);
        assert_eq!(run(&mut e, 0x005a_1de0, 0, 0, 0), (true, 0.0));
        e.mem.set_u8(player + 0x658, 1);
        assert_eq!(run(&mut e, 0x005a_1de0, 0, 0, 0), (true, 1.0));
        assert!(traces(&mut e, 0x005a_1de0, 0, 0, 0x0103_5944));
    }

    #[test]
    fn player_sleeping_byte_is_at_0x658() {
        let mut e = engine();
        let player = player_object(&mut e);
        e.mem.set_u8(player + 0x658, 0x5a);
        assert_eq!(e.call(0x005a_1e50, &args![player]).u32() & 0xff, 0x5a);
        e.mem.set_u8(player + 0x658, 0);
        assert_eq!(e.call(0x005a_1e50, &args![player]).u32() & 0xff, 0);
    }

    #[test]
    fn pc_a_murderer_asks_the_player() {
        let mut e = engine();
        let player = player_object(&mut e);
        e.register_double(PLAYER_IS_A_MURDERER, move |_, a| {
            assert_eq!(a, [player]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_1e70, 0, 0, 0), (true, 1.0));
        stub(&mut e, PLAYER_IS_A_MURDERER, 0);
        assert_eq!(run(&mut e, 0x005a_1e70, 0, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_1e70, 0, 0, 0x0103_5960));
    }

    #[test]
    fn detection_level_is_an_integer_and_names_both_references() {
        let mut e = engine();
        let actor = actor(&mut e);
        e.register_double(ACTOR_GET_DETECTION_LEVEL, move |_, a| {
            assert_eq!(a, [actor, 0x3000]);
            (-30i32 as u32).into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_1ee0, actor, 0x3000, 0), (true, -30.0));
        // The debug line: format, actor name, target name, level.
        e.register_double(REF_GET_NAME, |_, a| (0x8000 + a[0]).into_ret());
        trace_on(&mut e);
        run(&mut e, 0x005a_1ee0, actor, 0x3000, 0);
        let log = take_log(&mut e);
        let w = words(-30.0);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![
                0x0103_597c,
                0x8000 + actor,
                0x8000 + 0x3000,
                w[0],
                w[1]
            ]]
        );
        // A non-actor, no target or no reference: 0 and no debug line.
        let plain = non_actor(&mut e);
        for (reference, target) in [(plain, 0x3000), (actor, 0), (0, 0x3000)] {
            trace_on(&mut e);
            assert_eq!(run(&mut e, 0x005a_1ee0, reference, target, 0), (true, 0.0));
            assert!(!printed(&take_log(&mut e), 0x0103_597c));
        }
    }

    #[test]
    fn swimming_reads_the_actor_byte() {
        let mut e = engine();
        let swimmer = actor(&mut e);
        e.mem.set_u8(swimmer + 0x14d, 1);
        assert_eq!(run(&mut e, 0x005a_1fa0, swimmer, 0, 0), (true, 1.0));
        e.mem.set_u8(swimmer + 0x14d, 0);
        assert_eq!(run(&mut e, 0x005a_1fa0, swimmer, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        e.mem.set_u8(plain + 0x14d, 1);
        assert_eq!(run(&mut e, 0x005a_1fa0, plain, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_1fa0, 0, 0, 0x0103_599c));
    }

    #[test]
    fn swimming_byte_is_at_0x14d() {
        let mut e = engine();
        let block = e.mem.alloc(0x200);
        e.mem.set_u8(block + 0x14d, 0x33);
        assert_eq!(e.call(0x005a_2030, &args![block]).u32() & 0xff, 0x33);
    }

    #[test]
    fn amount_stolen_sold_reads_the_player_integer() {
        let mut e = engine();
        let player = player_object(&mut e);
        e.mem.set_u32(player + 0x6dc, 250);
        assert_eq!(run(&mut e, 0x005a_2050, 0, 0, 0), (true, 250.0));
        e.mem.set_u32(player + 0x6dc, -5i32 as u32);
        assert_eq!(run(&mut e, 0x005a_2050, 0, 0, 0), (true, -5.0));
        assert!(traces(&mut e, 0x005a_2050, 0, 0, 0x0103_59b4));
    }

    #[test]
    fn amount_stolen_field_is_at_0x6dc() {
        let mut e = engine();
        let block = e.mem.alloc(0x800);
        e.mem.set_u32(block + 0x6dc, 0xdead);
        assert_eq!(e.call(0x005a_20b0, &args![block]).u32(), 0xdead);
    }

    /// A faction-like object whose flags (`+0x34`) are `flags`.
    fn faction_with_flags(e: &mut Engine, flags: u32) -> u32 {
        let faction = e.mem.alloc(0x80);
        e.mem.set_u32(faction + 0x34, flags);
        faction
    }

    /// The text of the debug line (the only argument) a faction condition
    /// printed, as a format address.
    fn faction_line(e: &mut Engine, addr: u32, faction: u32) -> u32 {
        trace_on(e);
        run(e, addr, 0, faction, 0);
        let log = take_log(e);
        calls_to(&log, DEBUG_PRINT)[0][0]
    }

    #[test]
    fn pc_expelled_tests_the_faction() {
        let mut e = engine();
        e.register_double(FACTION_TEST_EXPELLED, |_, a| {
            assert_eq!(a, [0x4000]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_20d0, 0, 0x4000, 0), (true, 1.0));
        assert_eq!(faction_line(&mut e, 0x005a_20d0, 0x4000), 0x0103_59f0);
        stub(&mut e, FACTION_TEST_EXPELLED, 0);
        assert_eq!(run(&mut e, 0x005a_20d0, 0, 0x4000, 0), (true, 0.0));
        assert_eq!(faction_line(&mut e, 0x005a_20d0, 0x4000), 0x0103_59dc);
        // No faction: 0 and the test is never run.
        stub(&mut e, FACTION_TEST_EXPELLED, 1);
        assert_eq!(run(&mut e, 0x005a_20d0, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn pc_faction_murder_tests_flag_0x40() {
        let mut e = engine();
        let murder = faction_with_flags(&mut e, 0x40);
        let other = faction_with_flags(&mut e, 0xffff_ffbf);
        assert_eq!(run(&mut e, 0x005a_2150, 0, murder, 0), (true, 1.0));
        assert_eq!(faction_line(&mut e, 0x005a_2150, murder), 0x0103_5a24);
        assert_eq!(run(&mut e, 0x005a_2150, 0, other, 0), (true, 0.0));
        assert_eq!(faction_line(&mut e, 0x005a_2150, other), 0x0103_5a00);
        assert_eq!(run(&mut e, 0x005a_2150, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn faction_flag_0x40_helper() {
        let mut e = engine();
        let yes = faction_with_flags(&mut e, 0x41);
        let no = faction_with_flags(&mut e, 0xffff_ffbf);
        assert_eq!(e.call(0x005a_21d0, &args![yes]).u32() & 0xff, 1);
        assert_eq!(e.call(0x005a_21d0, &args![no]).u32() & 0xff, 0);
    }

    #[test]
    fn player_enemy_of_faction_tests_flag_0x10() {
        let mut e = engine();
        let enemy = faction_with_flags(&mut e, 0x10);
        let other = faction_with_flags(&mut e, 0xffff_ffef);
        assert_eq!(run(&mut e, 0x005a_21f0, 0, enemy, 0), (true, 1.0));
        assert_eq!(faction_line(&mut e, 0x005a_21f0, enemy), 0x0103_5a68);
        assert_eq!(run(&mut e, 0x005a_21f0, 0, other, 0), (true, 0.0));
        assert_eq!(faction_line(&mut e, 0x005a_21f0, other), 0x0103_5a44);
        assert_eq!(run(&mut e, 0x005a_21f0, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn faction_flag_0x10_helper() {
        let mut e = engine();
        let yes = faction_with_flags(&mut e, 0x11);
        let no = faction_with_flags(&mut e, 0xffff_ffef);
        assert_eq!(e.call(0x005a_2270, &args![yes]).u32() & 0xff, 1);
        assert_eq!(e.call(0x005a_2270, &args![no]).u32() & 0xff, 0);
    }

    #[test]
    fn pc_faction_attack_tests_flag_0x20() {
        let mut e = engine();
        let attacked = faction_with_flags(&mut e, 0x20);
        let other = faction_with_flags(&mut e, 0xffff_ffdf);
        assert_eq!(run(&mut e, 0x005a_2290, 0, attacked, 0), (true, 1.0));
        assert_eq!(faction_line(&mut e, 0x005a_2290, attacked), 0x0103_5aac);
        assert_eq!(run(&mut e, 0x005a_2290, 0, other, 0), (true, 0.0));
        assert_eq!(faction_line(&mut e, 0x005a_2290, other), 0x0103_5a88);
        assert_eq!(run(&mut e, 0x005a_2290, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn faction_flag_0x20_helper() {
        let mut e = engine();
        let yes = faction_with_flags(&mut e, 0x21);
        let no = faction_with_flags(&mut e, 0xffff_ffdf);
        assert_eq!(e.call(0x005a_2310, &args![yes]).u32() & 0xff, 1);
        assert_eq!(e.call(0x005a_2310, &args![no]).u32() & 0xff, 0);
    }

    #[test]
    fn destroyed_asks_the_reference() {
        let mut e = engine();
        e.register_double(REF_TEST_DESTROYED, |_, a| {
            assert_eq!(a, [0x2000]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_2330, 0x2000, 0, 0), (true, 1.0));
        stub(&mut e, REF_TEST_DESTROYED, 0);
        assert_eq!(run(&mut e, 0x005a_2330, 0x2000, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_2330, 0, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_2330, 0, 0, 0x0103_5acc));
    }

    /// An actor whose virtual `+0x19c` returns `magic_target`.
    fn actor_with_magic_target(e: &mut Engine, magic_target: u32) -> u32 {
        stub(e, 0x0f20_0040, magic_target);
        actor_with(e, &[(VSLOT_ACTOR_GET_MAGIC_TARGET, 0x0f20_0040)])
    }

    #[test]
    fn has_magic_effect_asks_the_magic_target() {
        let mut e = engine();
        let actor = actor_with_magic_target(&mut e, 0x6000);
        e.register_double(MAGIC_TARGET_HAS_EFFECT, |_, a| {
            assert_eq!(a, [0x6000, 0x4000]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_2390, actor, 0x4000, 0), (true, 1.0));
        stub(&mut e, MAGIC_TARGET_HAS_EFFECT, 0);
        assert_eq!(run(&mut e, 0x005a_2390, actor, 0x4000, 0), (true, 0.0));
        // No effect form, not an actor.
        stub(&mut e, MAGIC_TARGET_HAS_EFFECT, 1);
        assert_eq!(run(&mut e, 0x005a_2390, actor, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_2390, plain, 0x4000, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_2390, 0, 0, 0x0103_5ae4));
    }

    #[test]
    fn is_spell_target_passes_a_one() {
        let mut e = engine();
        let actor = actor_with_magic_target(&mut e, 0x6000);
        e.register_double(MAGIC_TARGET_IS_SPELL_TARGET, |_, a| {
            assert_eq!(a, [0x6000, 0x4000, 1]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_2440, actor, 0x4000, 0), (true, 1.0));
        stub(&mut e, MAGIC_TARGET_IS_SPELL_TARGET, 0);
        assert_eq!(run(&mut e, 0x005a_2440, actor, 0x4000, 0), (true, 0.0));
        stub(&mut e, MAGIC_TARGET_IS_SPELL_TARGET, 1);
        assert_eq!(run(&mut e, 0x005a_2440, actor, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_2440, plain, 0x4000, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_2440, 0, 0, 0x0103_5b00));
    }

    #[test]
    fn spell_usage_number_uses_the_magic_target_inside_the_actor() {
        let mut e = engine();
        let actor = actor(&mut e);
        e.register_double(MAGIC_TARGET_GET_USAGE_MONITOR_MAGNITUDE, move |_, a| {
            assert_eq!(a, [actor + 0x94, 0x4000]);
            Ret {
                st0: 2.75,
                ..Ret::default()
            }
        });
        assert_eq!(run(&mut e, 0x005a_24f0, actor, 0x4000, 0), (true, 2.75));
        // No effect, not an actor: 0.
        assert_eq!(run(&mut e, 0x005a_24f0, actor, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_24f0, plain, 0x4000, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_24f0, 0, 0, 0x0103_5b1c));
    }

    #[test]
    fn vats_mode_is_the_vats_field() {
        let mut e = engine();
        e.register_double(VATS_GET_MODE, |_, a| {
            assert_eq!(a, [VATS]);
            3u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_2590, 0, 0, 0), (true, 3.0));
        assert!(traces(&mut e, 0x005a_2590, 0, 0, 0x0103_5b38));
    }

    /// Everything `GetVATSTargetHeight` needs: mode 4, an attack action on
    /// `target` with the body part number `body_part`, the player at height
    /// `player_z`. Returns (action, player).
    fn vats_setup(e: &mut Engine, target: u32, body_part: u32, player_z: f32) -> (u32, u32) {
        e.register_double(VATS_GET_MODE, |_, _| 4u32.into_ret());
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(action, 7);
        e.mem.set_u32(action + 0xc, target);
        e.mem.set_u32(action + 0x10, body_part);
        stub(e, VATS_GET_CURRENT_ACTION, action);
        e.register_double(ACTION_IS_ATTACK, |_, a| (a[0] == 7).into_ret());
        e.register_double(RETURN_THIS, |_, a| a[0].into_ret());
        let position = floats(e, [1.0, 2.0, player_z]);
        stub(e, 0x0f20_0050, position);
        let player = e.mem.alloc(0x800);
        let vtable = e.mem.alloc(0x800);
        e.mem.set_u32(vtable + VSLOT_GET_POSITION, 0x0f20_0050);
        e.mem.set_u32(player, vtable);
        e.set_global(PLAYER, player);
        (action, player)
    }

    #[test]
    fn vats_target_height_uses_the_position_without_a_node() {
        let mut e = engine();
        let position = floats(&mut e, [5.0, 6.0, 10.5]);
        stub(&mut e, 0x0f20_0060, position);
        stub(&mut e, 0x0f20_0061, 0);
        let target = object(
            &mut e,
            &[
                (VSLOT_GET_POSITION, 0x0f20_0060),
                (VSLOT_REF_GET_NODE, 0x0f20_0061),
            ],
        );
        vats_setup(&mut e, target, 0xffff_ffff, 4.0);
        assert_eq!(run(&mut e, 0x005a_25f0, 0, 0, 0), (true, 6.5));
        assert!(traces(&mut e, 0x005a_25f0, 0, 0, 0x0103_5b4c));
    }

    #[test]
    fn vats_target_height_uses_the_bound_center_with_a_node() {
        let mut e = engine();
        let center = floats(&mut e, [0.0, 0.0, 21.25]);
        stub(&mut e, 0x0f20_0062, 0x5000);
        let target = object(&mut e, &[(VSLOT_REF_GET_NODE, 0x0f20_0062)]);
        vats_setup(&mut e, target, 0xffff_ffff, 1.25);
        e.register_double(NODE_GET_WORLD_BOUND, |_, a| {
            assert_eq!(a, [0x5000]);
            0x5400u32.into_ret()
        });
        // The bound's first field is the center; RETURN_THIS gives its address.
        e.register_double(RETURN_THIS, move |_, a| {
            if a[0] == 0x5400 {
                center.into_ret()
            } else {
                a[0].into_ret()
            }
        });
        assert_eq!(run(&mut e, 0x005a_25f0, 0, 0, 0), (true, 20.0));
    }

    #[test]
    fn vats_target_height_uses_the_node_of_the_body_part() {
        let mut e = engine();
        let part_position = floats(&mut e, [0.0, 0.0, 30.0]);
        stub(&mut e, 0x0f20_0063, 0x5000);
        stub(&mut e, 0x0f20_0064, 0x5500);
        stub(&mut e, 0x0f20_0065, 0);
        let target = actor_with(
            &mut e,
            &[
                (VSLOT_REF_GET_NODE, 0x0f20_0063),
                (VSLOT_GET_POSITION, 0x0f20_0065),
            ],
        );
        stub(&mut e, 0x0f20_0066, 0x5600);
        let base = object(&mut e, &[(VSLOT_BASE_FORM_BODY_PART_DATA, 0x0f20_0066)]);
        vats_setup(&mut e, target, 9, 10.0);
        e.register_double(ACTOR_GET_BASE_FORM, move |_, a| {
            assert_eq!(a, [target]);
            base.into_ret()
        });
        e.register_double(BODY_PART_DATA_GET_BODY_PART, |_, a| {
            assert_eq!(a, [0x5600, 9]);
            0x5700u32.into_ret()
        });
        e.register_double(BODY_PART_GET_NODE_NAME, |_, a| {
            assert_eq!(a, [0x5700]);
            0x5800u32.into_ret()
        });
        e.register_double(NODE_FIND_BY_NAME, |_, a| {
            assert_eq!(a, [0x5000, 0x5800]);
            0x5900u32.into_ret()
        });
        e.register_double(NODE_GET_WORLD_POSITION, move |_, a| {
            assert_eq!(a, [0x5900]);
            part_position.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_25f0, 0, 0, 0), (true, 20.0));
        // A part without a node falls back to the center of the target node bound.
        stub(&mut e, NODE_FIND_BY_NAME, 0);
        stub(&mut e, NODE_GET_WORLD_BOUND, 0x5a00);
        let center = floats(&mut e, [0.0, 0.0, 50.0]);
        let zero = floats(&mut e, [0.0, 0.0, 0.0]);
        stub(&mut e, 0x0f20_0065, zero);
        e.register_double(RETURN_THIS, move |_, a| {
            if a[0] == 0x5a00 {
                center.into_ret()
            } else {
                a[0].into_ret()
            }
        });
        assert_eq!(run(&mut e, 0x005a_25f0, 0, 0, 0), (true, 40.0));
        // No such part: same fallback.
        stub(&mut e, BODY_PART_DATA_GET_BODY_PART, 0);
        assert_eq!(run(&mut e, 0x005a_25f0, 0, 0, 0), (true, 40.0));
    }

    #[test]
    fn vats_target_height_needs_mode_4_and_an_attack_action() {
        let mut e = engine();
        let position = floats(&mut e, [0.0, 0.0, 10.0]);
        stub(&mut e, 0x0f20_0067, position);
        stub(&mut e, 0x0f20_0068, 0);
        let target = object(
            &mut e,
            &[
                (VSLOT_GET_POSITION, 0x0f20_0067),
                (VSLOT_REF_GET_NODE, 0x0f20_0068),
            ],
        );
        vats_setup(&mut e, target, 0xffff_ffff, 0.0);
        assert_eq!(run(&mut e, 0x005a_25f0, 0, 0, 0), (true, 10.0));
        e.register_double(ACTION_IS_ATTACK, |_, _| false.into_ret());
        assert_eq!(run(&mut e, 0x005a_25f0, 0, 0, 0), (true, 0.0));
        e.register_double(ACTION_IS_ATTACK, |_, _| true.into_ret());
        stub(&mut e, VATS_GET_CURRENT_ACTION, 0);
        assert_eq!(run(&mut e, 0x005a_25f0, 0, 0, 0), (true, 0.0));
        e.register_double(VATS_GET_MODE, |_, _| 3u32.into_ret());
        assert_eq!(run(&mut e, 0x005a_25f0, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn vats_target_height_without_a_target_is_zero() {
        let mut e = engine();
        vats_setup(&mut e, 0, 0xffff_ffff, 0.0);
        assert_eq!(run(&mut e, 0x005a_25f0, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn unnamed_extra_data_test_with_eight() {
        let mut e = engine();
        e.register_double(REF_EXTRA_DATA_LIST_TEST, |_, a| {
            assert_eq!(a, [0x2000, 8]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_27e0, 0x2000, 0, 0), (true, 1.0));
        stub(&mut e, REF_EXTRA_DATA_LIST_TEST, 0);
        assert_eq!(run(&mut e, 0x005a_27e0, 0x2000, 0, 0), (true, 0.0));
        // A null reference: false and the result is not written.
        trace_on(&mut e);
        assert_eq!(run(&mut e, 0x005a_27e0, 0, 0, 0), (false, SENTINEL));
        assert!(calls_to(&take_log(&mut e), DEBUG_PRINT).is_empty());
    }

    #[test]
    fn sandman_and_cannibal_only_answer_for_the_player() {
        let mut e = engine();
        stub(&mut e, 0x0f20_0070, 1);
        stub(&mut e, 0x0f20_0071, 0x0101);
        let player = object(
            &mut e,
            &[
                (VSLOT_PLAYER_SANDMAN, 0x0f20_0070),
                (VSLOT_PLAYER_CANNIBAL, 0x0f20_0071),
            ],
        );
        e.set_global(PLAYER, player);
        let other = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_2810, player, 0, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_2810, other, 0, 0), (true, 0.0));
        // Only the low byte of the answer counts.
        assert_eq!(run(&mut e, 0x005a_2890, player, 0, 0), (true, 1.0));
        stub(&mut e, 0x0f20_0071, 0x0100);
        assert_eq!(run(&mut e, 0x005a_2890, player, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_2890, other, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_2810, other, 0, 0x0103_5b68));
        assert!(traces(&mut e, 0x005a_2890, other, 0, 0x0103_5b90));
    }

    #[test]
    fn crit_stage_compares_and_names_the_stage() {
        let mut e = engine();
        let actor = actor(&mut e);
        e.map(CRIT_STAGE_NAMES & !0xfff, 0x1000);
        e.mem.set_u32(CRIT_STAGE_NAMES + 3 * 4, 0x0f30_0003);
        stub(&mut e, ACTOR_GET_CRIT_STAGE, 3);
        assert_eq!(run(&mut e, 0x005a_2910, actor, 3, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_2910, actor, 2, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_2910, plain, 3, 0), (true, 0.0));
        // "Actor %s in stage %s.": IS / IS NOT and the name from the table.
        trace_on(&mut e);
        run(&mut e, 0x005a_2910, actor, 3, 0);
        run(&mut e, 0x005a_2910, actor, 2, 0);
        let log = take_log(&mut e);
        let lines = calls_to(&log, DEBUG_PRINT);
        assert_eq!(lines[0], vec![0x0103_5bb8, 0x0103_5bd8, 0x0f30_0003]);
        assert_eq!(lines[1][..2], [0x0103_5bb8, 0x0103_5bd0]);
    }

    #[test]
    fn has_been_eaten_asks_the_actor() {
        let mut e = engine();
        let actor = actor(&mut e);
        e.register_double(ACTOR_HAS_BEEN_EATEN, move |_, a| {
            assert_eq!(a, [actor]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_29d0, actor, 0, 0), (true, 1.0));
        stub(&mut e, ACTOR_HAS_BEEN_EATEN, 0);
        assert_eq!(run(&mut e, 0x005a_29d0, actor, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        stub(&mut e, ACTOR_HAS_BEEN_EATEN, 1);
        assert_eq!(run(&mut e, 0x005a_29d0, plain, 0, 0), (true, 0.0));
        assert!(traces(&mut e, 0x005a_29d0, plain, 0, 0x0103_5bdc));
    }
}
